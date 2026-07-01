#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
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
        var_guard182: f64,
        var_guard199: f64,
        var_guard326: f64,
        var_guard330: f64,
        var_guard333: f64,
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
        var_v2: f64,
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
        var_guard334_slot: &mut f64,
        var_guard335_slot: &mut f64,
        var_guard336_slot: &mut f64,
        var_guard337_slot: &mut f64,
        var_guard338_slot: &mut f64,
        var_guard339_slot: &mut f64,
        var_guard340_slot: &mut f64,
        var_guard341_slot: &mut f64,
        var_guard342_slot: &mut f64,
        var_guard343_slot: &mut f64,
        var_guard344_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
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
        let mut var_guard334: f64 = *var_guard334_slot;
        let mut var_guard335: f64 = *var_guard335_slot;
        let mut var_guard336: f64 = *var_guard336_slot;
        let mut var_guard337: f64 = *var_guard337_slot;
        let mut var_guard338: f64 = *var_guard338_slot;
        let mut var_guard339: f64 = *var_guard339_slot;
        let mut var_guard340: f64 = *var_guard340_slot;
        let mut var_guard341: f64 = *var_guard341_slot;
        let mut var_guard342: f64 = *var_guard342_slot;
        let mut var_guard343: f64 = *var_guard343_slot;
        let mut var_guard344: f64 = *var_guard344_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
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

        let (assign19280_e19148, assign19280_e19148_d_n5, assign19280_e19148_d_n6, assign19280_e19148_d_n7, assign19280_e19148_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) && (var_guard333 == 0.0)) {
        let assign19280_e19115: f64 = (-230.25850929940458);
        let assign19280_e19117: f64 = (-var_ysq);
        let assign19280_e19119: f64 = (assign19280_e19117 + var_mtat);
        let assign19280_e19120: f64 = (assign19280_e19115 - assign19280_e19119);
        let assign19280_e19124: f64 = (-230.25850929940458);
        let assign19280_e19126: f64 = (-var_ysq);
        let assign19280_e19128: f64 = (assign19280_e19126 + var_mtat);
        let assign19280_e19129: f64 = (assign19280_e19124 - assign19280_e19128);
        let assign19280_e19132: f64 = (-230.25850929940458);
        let assign19280_e19134: f64 = (-var_ysq);
        let assign19280_e19136: f64 = (assign19280_e19134 + var_mtat);
        let assign19280_e19137: f64 = (assign19280_e19132 - assign19280_e19136);
        let assign19280_e19139: f64 = (assign19280_e19137 * 0.3333333333333333);
        let assign19280_e19140: f64 = (1.0 + assign19280_e19139);
        let assign19280_e19141: f64 = (assign19280_e19129 * assign19280_e19140);
        let assign19280_e19142: f64 = (0.5 * assign19280_e19141);
        let assign19280_e19143: f64 = (1.0 + assign19280_e19142);
        let assign19280_e19144: f64 = (assign19280_e19120 * assign19280_e19143);
        let assign19280_e19145: f64 = (1.0 + assign19280_e19144);
        let assign19280_e19146: f64 = (1e-100 / assign19280_e19145);
        (assign19280_e19146, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign19280_e19143) + (assign19280_e19120 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign19280_e19140) + (assign19280_e19129 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign19280_e19145 * assign19280_e19145))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign19280_e19143) + (assign19280_e19120 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign19280_e19140) + (assign19280_e19129 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign19280_e19145 * assign19280_e19145))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign19280_e19143) + (assign19280_e19120 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign19280_e19140) + (assign19280_e19129 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign19280_e19145 * assign19280_e19145))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign19280_e19143) + (assign19280_e19120 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign19280_e19140) + (assign19280_e19129 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign19280_e19145 * assign19280_e19145))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19280_e19148;
        var_tmp_dn5 = assign19280_e19148_d_n5;
        var_tmp_dn6 = assign19280_e19148_d_n6;
        var_tmp_dn7 = assign19280_e19148_d_n7;
        var_tmp_dn8 = assign19280_e19148_d_n8;

        let (assign19290_e19178, assign19290_e19178_d_n5, assign19290_e19178_d_n6, assign19290_e19178_d_n7, assign19290_e19178_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) {
        let assign19290_e19160: f64 = (0.29214664 * var_terfc);
        let assign19290_e19164: f64 = (var_terfc * var_terfc);
        let assign19290_e19165: f64 = (var_berfc * assign19290_e19164);
        let assign19290_e19166: f64 = (assign19290_e19160 + assign19290_e19165);
        let assign19290_e19170: f64 = (var_terfc * var_terfc);
        let assign19290_e19172: f64 = (assign19290_e19170 * var_terfc);
        let assign19290_e19173: f64 = (var_cerfc * assign19290_e19172);
        let assign19290_e19174: f64 = (assign19290_e19166 + assign19290_e19173);
        let assign19290_e19176: f64 = (assign19290_e19174 * var_tmp);
        (assign19290_e19176, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign19290_e19170 * var_terfc_dn5)))) * var_tmp) + (assign19290_e19174 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign19290_e19170 * var_terfc_dn6)))) * var_tmp) + (assign19290_e19174 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign19290_e19170 * var_terfc_dn7)))) * var_tmp) + (assign19290_e19174 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign19290_e19170 * var_terfc_dn8)))) * var_tmp) + (assign19290_e19174 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign19290_e19178;
        var_erfcpos_dn5 = assign19290_e19178_d_n5;
        var_erfcpos_dn6 = assign19290_e19178_d_n6;
        var_erfcpos_dn7 = assign19290_e19178_d_n7;
        var_erfcpos_dn8 = assign19290_e19178_d_n8;

        let assign19300_e19181: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard334 = assign19300_e19181;

        let (assign19310_e19195, assign19310_e19195_d_n5, assign19310_e19195_d_n6, assign19310_e19195_d_n7, assign19310_e19195_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) && (var_guard334 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign19310_e19195;
        var_erfctimesexpmtat_dn5 = assign19310_e19195_d_n5;
        var_erfctimesexpmtat_dn6 = assign19310_e19195_d_n6;
        var_erfctimesexpmtat_dn7 = assign19310_e19195_d_n7;
        var_erfctimesexpmtat_dn8 = assign19310_e19195_d_n8;

        let assign19320_e19198: f64 = (-230.25850929940458);
        let assign19320_e19199: f64 = if var_mtat > assign19320_e19198 { 1.0 } else { 0.0 };
        var_guard335 = assign19320_e19199;

        let (assign19330_e19217, assign19330_e19217_d_n5, assign19330_e19217_d_n6, assign19330_e19217_d_n7, assign19330_e19217_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) && (var_guard334 == 0.0)) && (var_guard335 != 0.0)) {
        let assign19330_e19215: f64 = (var_mtat).exp();
        (assign19330_e19215, (assign19330_e19215 * var_mtat_dn5), (assign19330_e19215 * var_mtat_dn6), (assign19330_e19215 * var_mtat_dn7), (assign19330_e19215 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19330_e19217;
        var_tmp_dn5 = assign19330_e19217_d_n5;
        var_tmp_dn6 = assign19330_e19217_d_n6;
        var_tmp_dn7 = assign19330_e19217_d_n7;
        var_tmp_dn8 = assign19330_e19217_d_n8;

        let (assign19340_e19260, assign19340_e19260_d_n5, assign19340_e19260_d_n6, assign19340_e19260_d_n7, assign19340_e19260_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) && (var_guard334 == 0.0)) && (var_guard335 == 0.0)) {
        let assign19340_e19236: f64 = (-230.25850929940458);
        let assign19340_e19238: f64 = (assign19340_e19236 - var_mtat);
        let assign19340_e19242: f64 = (-230.25850929940458);
        let assign19340_e19244: f64 = (assign19340_e19242 - var_mtat);
        let assign19340_e19247: f64 = (-230.25850929940458);
        let assign19340_e19249: f64 = (assign19340_e19247 - var_mtat);
        let assign19340_e19251: f64 = (assign19340_e19249 * 0.3333333333333333);
        let assign19340_e19252: f64 = (1.0 + assign19340_e19251);
        let assign19340_e19253: f64 = (assign19340_e19244 * assign19340_e19252);
        let assign19340_e19254: f64 = (0.5 * assign19340_e19253);
        let assign19340_e19255: f64 = (1.0 + assign19340_e19254);
        let assign19340_e19256: f64 = (assign19340_e19238 * assign19340_e19255);
        let assign19340_e19257: f64 = (1.0 + assign19340_e19256);
        let assign19340_e19258: f64 = (1e-100 / assign19340_e19257);
        (assign19340_e19258, (-((1e-100 * (((-var_mtat_dn5) * assign19340_e19255) + (assign19340_e19238 * (0.5 * (((-var_mtat_dn5) * assign19340_e19252) + (assign19340_e19244 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign19340_e19257 * assign19340_e19257))), (-((1e-100 * (((-var_mtat_dn6) * assign19340_e19255) + (assign19340_e19238 * (0.5 * (((-var_mtat_dn6) * assign19340_e19252) + (assign19340_e19244 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign19340_e19257 * assign19340_e19257))), (-((1e-100 * (((-var_mtat_dn7) * assign19340_e19255) + (assign19340_e19238 * (0.5 * (((-var_mtat_dn7) * assign19340_e19252) + (assign19340_e19244 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign19340_e19257 * assign19340_e19257))), (-((1e-100 * (((-var_mtat_dn8) * assign19340_e19255) + (assign19340_e19238 * (0.5 * (((-var_mtat_dn8) * assign19340_e19252) + (assign19340_e19244 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign19340_e19257 * assign19340_e19257))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19340_e19260;
        var_tmp_dn5 = assign19340_e19260_d_n5;
        var_tmp_dn6 = assign19340_e19260_d_n6;
        var_tmp_dn7 = assign19340_e19260_d_n7;
        var_tmp_dn8 = assign19340_e19260_d_n8;

        let (assign19350_e19279, assign19350_e19279_d_n5, assign19350_e19279_d_n6, assign19350_e19279_d_n7, assign19350_e19279_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) && (var_guard334 == 0.0)) {
        let assign19350_e19275: f64 = (2.0 * var_tmp);
        let assign19350_e19277: f64 = (assign19350_e19275 - var_erfcpos);
        (assign19350_e19277, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign19350_e19279;
        var_erfctimesexpmtat_dn5 = assign19350_e19279_d_n5;
        var_erfctimesexpmtat_dn6 = assign19350_e19279_d_n6;
        var_erfctimesexpmtat_dn7 = assign19350_e19279_d_n7;
        var_erfctimesexpmtat_dn8 = assign19350_e19279_d_n8;

        let (assign19360_e19299, assign19360_e19299_d_n5, assign19360_e19299_d_n6, assign19360_e19299_d_n7, assign19360_e19299_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) {
        let assign19360_e19291: f64 = (1.772453850905516 * 0.5);
        let assign19360_e19294: f64 = (var_atatsti * var_erfctimesexpmtat);
        let assign19360_e19296: f64 = (assign19360_e19294 / var_ktat);
        let assign19360_e19297: f64 = (assign19360_e19291 * assign19360_e19296);
        (assign19360_e19297, (assign19360_e19291 * ((((var_atatsti * var_erfctimesexpmtat_dn5) * var_ktat) - (assign19360_e19294 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign19360_e19291 * ((((var_atatsti * var_erfctimesexpmtat_dn6) * var_ktat) - (assign19360_e19294 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign19360_e19291 * ((((var_atatsti * var_erfctimesexpmtat_dn7) * var_ktat) - (assign19360_e19294 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign19360_e19291 * ((((var_atatsti * var_erfctimesexpmtat_dn8) * var_ktat) - (assign19360_e19294 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign19360_e19299;
        var_gammamax_dn5 = assign19360_e19299_d_n5;
        var_gammamax_dn6 = assign19360_e19299_d_n6;
        var_gammamax_dn7 = assign19360_e19299_d_n7;
        var_gammamax_dn8 = assign19360_e19299_d_n8;

        let (assign19370_e19317, assign19370_e19317_d_n5, assign19370_e19317_d_n6, assign19370_e19317_d_n7, assign19370_e19317_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard330 == 0.0)) {
        let assign19370_e19312: f64 = (var_asrh * var_gammamax);
        let assign19370_e19314: f64 = (assign19370_e19312 * var_wtat);
        let assign19370_e19315: f64 = (p.p846 * assign19370_e19314);
        (assign19370_e19315, (p.p846 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign19370_e19312 * var_wtat_dn5))), (p.p846 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign19370_e19312 * var_wtat_dn6))), (p.p846 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign19370_e19312 * var_wtat_dn7))), (p.p846 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign19370_e19312 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign19370_e19317;
        var_itat_dn5 = assign19370_e19317_d_n5;
        var_itat_dn6 = assign19370_e19317_d_n6;
        var_itat_dn7 = assign19370_e19317_d_n7;
        var_itat_dn8 = assign19370_e19317_d_n8;

        let assign19380_e19320: f64 = if p.p852 == 0.0 { 1.0 } else { 0.0 };
        var_guard336 = assign19380_e19320;

        let (assign19390_e19331, assign19390_e19331_d_n5, assign19390_e19331_d_n6, assign19390_e19331_d_n7, assign19390_e19331_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard336 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign19390_e19331;
        var_ibbt_dn5 = assign19390_e19331_d_n5;
        var_ibbt_dn6 = assign19390_e19331_d_n6;
        var_ibbt_dn7 = assign19390_e19331_d_n7;
        var_ibbt_dn8 = assign19390_e19331_d_n8;

        let assign19400_e19334: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        var_guard337 = assign19400_e19334;

        let (assign19410_e19353, assign19410_e19353_d_n5, assign19410_e19353_d_n6, assign19410_e19353_d_n7, assign19410_e19353_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard336 == 0.0)) && (var_guard337 != 0.0)) {
        let assign19410_e19348: f64 = (p.p829 - var_vbbt);
        let assign19410_e19350: f64 = (assign19410_e19348 * var_vbirstiinv);
        let assign19410_e19351: f64 = (assign19410_e19350).sqrt();
        (assign19410_e19351, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19410_e19353;
        var_tmp_dn5 = assign19410_e19353_d_n5;
        var_tmp_dn6 = assign19410_e19353_d_n6;
        var_tmp_dn7 = assign19410_e19353_d_n7;
        var_tmp_dn8 = assign19410_e19353_d_n8;

        let (assign19420_e19374, assign19420_e19374_d_n5, assign19420_e19374_d_n6, assign19420_e19374_d_n7, assign19420_e19374_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard336 == 0.0)) && (var_guard337 == 0.0)) {
        let assign19420_e19368: f64 = (p.p829 - var_vbbt);
        let assign19420_e19370: f64 = (assign19420_e19368 * var_vbirstiinv);
        let assign19420_e19372: f64 = (assign19420_e19370).powf(p.p832);
        (assign19420_e19372, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19420_e19374;
        var_tmp_dn5 = assign19420_e19374_d_n5;
        var_tmp_dn6 = assign19420_e19374_d_n6;
        var_tmp_dn7 = assign19420_e19374_d_n7;
        var_tmp_dn8 = assign19420_e19374_d_n8;

        let (assign19430_e19394, assign19430_e19394_d_n5, assign19430_e19394_d_n6, assign19430_e19394_d_n7, assign19430_e19394_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard336 == 0.0)) {
        let assign19430_e19387: f64 = (p.p829 - var_vbbt);
        let assign19430_e19389: f64 = (assign19430_e19387 * var_wdepnulrinvsti);
        let assign19430_e19391: f64 = (assign19430_e19389 / var_tmp);
        let assign19430_e19392: f64 = (var_one_over_one_minus_psti * assign19430_e19391);
        (assign19430_e19392, (var_one_over_one_minus_psti * (-((assign19430_e19389 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign19430_e19389 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign19430_e19389 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign19430_e19389 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign19430_e19394;
        var_fmaxr_dn5 = assign19430_e19394_d_n5;
        var_fmaxr_dn6 = assign19430_e19394_d_n6;
        var_fmaxr_dn7 = assign19430_e19394_d_n7;
        var_fmaxr_dn8 = assign19430_e19394_d_n8;

        let assign19440_e19396: f64 = (-var_fbbtsti);
        let assign19440_e19398: f64 = (assign19440_e19396 / var_fmaxr);
        let assign19440_e19399: f64 = (assign19440_e19398).abs();
        let assign19440_e19401: f64 = if assign19440_e19399 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard338 = assign19440_e19401;

        let (assign19450_e19419, assign19450_e19419_d_n5, assign19450_e19419_d_n6, assign19450_e19419_d_n7, assign19450_e19419_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard336 == 0.0)) && (var_guard338 != 0.0)) {
        let assign19450_e19414: f64 = (-var_fbbtsti);
        let assign19450_e19416: f64 = (assign19450_e19414 / var_fmaxr);
        let assign19450_e19417: f64 = (assign19450_e19416).exp();
        (assign19450_e19417, (assign19450_e19417 * (-((assign19450_e19414 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign19450_e19417 * (-((assign19450_e19414 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign19450_e19417 * (-((assign19450_e19414 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign19450_e19417 * (-((assign19450_e19414 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19450_e19419;
        var_tmp_dn5 = assign19450_e19419_d_n5;
        var_tmp_dn6 = assign19450_e19419_d_n6;
        var_tmp_dn7 = assign19450_e19419_d_n7;
        var_tmp_dn8 = assign19450_e19419_d_n8;

        let assign19460_e19421: f64 = (-var_fbbtsti);
        let assign19460_e19423: f64 = (assign19460_e19421 / var_fmaxr);
        let assign19460_e19425: f64 = if assign19460_e19423 < 0.0 { 1.0 } else { 0.0 };
        var_guard339 = assign19460_e19425;

        let (assign19470_e19476, assign19470_e19476_d_n5, assign19470_e19476_d_n6, assign19470_e19476_d_n7, assign19470_e19476_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard336 == 0.0)) && (var_guard338 == 0.0)) && (var_guard339 != 0.0)) {
        let assign19470_e19443: f64 = (-230.25850929940458);
        let assign19470_e19445: f64 = (-var_fbbtsti);
        let assign19470_e19447: f64 = (assign19470_e19445 / var_fmaxr);
        let assign19470_e19448: f64 = (assign19470_e19443 - assign19470_e19447);
        let assign19470_e19452: f64 = (-230.25850929940458);
        let assign19470_e19454: f64 = (-var_fbbtsti);
        let assign19470_e19456: f64 = (assign19470_e19454 / var_fmaxr);
        let assign19470_e19457: f64 = (assign19470_e19452 - assign19470_e19456);
        let assign19470_e19460: f64 = (-230.25850929940458);
        let assign19470_e19462: f64 = (-var_fbbtsti);
        let assign19470_e19464: f64 = (assign19470_e19462 / var_fmaxr);
        let assign19470_e19465: f64 = (assign19470_e19460 - assign19470_e19464);
        let assign19470_e19467: f64 = (assign19470_e19465 * 0.3333333333333333);
        let assign19470_e19468: f64 = (1.0 + assign19470_e19467);
        let assign19470_e19469: f64 = (assign19470_e19457 * assign19470_e19468);
        let assign19470_e19470: f64 = (0.5 * assign19470_e19469);
        let assign19470_e19471: f64 = (1.0 + assign19470_e19470);
        let assign19470_e19472: f64 = (assign19470_e19448 * assign19470_e19471);
        let assign19470_e19473: f64 = (1.0 + assign19470_e19472);
        let assign19470_e19474: f64 = (1e-100 / assign19470_e19473);
        (assign19470_e19474, (-((1e-100 * (((-(-((assign19470_e19445 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign19470_e19471) + (assign19470_e19448 * (0.5 * (((-(-((assign19470_e19454 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign19470_e19468) + (assign19470_e19457 * ((-(-((assign19470_e19462 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign19470_e19473 * assign19470_e19473))), (-((1e-100 * (((-(-((assign19470_e19445 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign19470_e19471) + (assign19470_e19448 * (0.5 * (((-(-((assign19470_e19454 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign19470_e19468) + (assign19470_e19457 * ((-(-((assign19470_e19462 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign19470_e19473 * assign19470_e19473))), (-((1e-100 * (((-(-((assign19470_e19445 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign19470_e19471) + (assign19470_e19448 * (0.5 * (((-(-((assign19470_e19454 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign19470_e19468) + (assign19470_e19457 * ((-(-((assign19470_e19462 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign19470_e19473 * assign19470_e19473))), (-((1e-100 * (((-(-((assign19470_e19445 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign19470_e19471) + (assign19470_e19448 * (0.5 * (((-(-((assign19470_e19454 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign19470_e19468) + (assign19470_e19457 * ((-(-((assign19470_e19462 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign19470_e19473 * assign19470_e19473))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19470_e19476;
        var_tmp_dn5 = assign19470_e19476_d_n5;
        var_tmp_dn6 = assign19470_e19476_d_n6;
        var_tmp_dn7 = assign19470_e19476_d_n7;
        var_tmp_dn8 = assign19470_e19476_d_n8;

        let (assign19480_e19525, assign19480_e19525_d_n5, assign19480_e19525_d_n6, assign19480_e19525_d_n7, assign19480_e19525_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard336 == 0.0)) && (var_guard338 == 0.0)) && (var_guard339 == 0.0)) {
        let assign19480_e19495: f64 = (-var_fbbtsti);
        let assign19480_e19497: f64 = (assign19480_e19495 / var_fmaxr);
        let assign19480_e19499: f64 = (assign19480_e19497 - 230.25850929940458);
        let assign19480_e19503: f64 = (-var_fbbtsti);
        let assign19480_e19505: f64 = (assign19480_e19503 / var_fmaxr);
        let assign19480_e19507: f64 = (assign19480_e19505 - 230.25850929940458);
        let assign19480_e19510: f64 = (-var_fbbtsti);
        let assign19480_e19512: f64 = (assign19480_e19510 / var_fmaxr);
        let assign19480_e19514: f64 = (assign19480_e19512 - 230.25850929940458);
        let assign19480_e19516: f64 = (assign19480_e19514 * 0.3333333333333333);
        let assign19480_e19517: f64 = (1.0 + assign19480_e19516);
        let assign19480_e19518: f64 = (assign19480_e19507 * assign19480_e19517);
        let assign19480_e19519: f64 = (0.5 * assign19480_e19518);
        let assign19480_e19520: f64 = (1.0 + assign19480_e19519);
        let assign19480_e19521: f64 = (assign19480_e19499 * assign19480_e19520);
        let assign19480_e19522: f64 = (1.0 + assign19480_e19521);
        let assign19480_e19523: f64 = (1e100 * assign19480_e19522);
        (assign19480_e19523, (1e100 * (((-((assign19480_e19495 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign19480_e19520) + (assign19480_e19499 * (0.5 * (((-((assign19480_e19503 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign19480_e19517) + (assign19480_e19507 * ((-((assign19480_e19510 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign19480_e19495 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign19480_e19520) + (assign19480_e19499 * (0.5 * (((-((assign19480_e19503 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign19480_e19517) + (assign19480_e19507 * ((-((assign19480_e19510 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign19480_e19495 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign19480_e19520) + (assign19480_e19499 * (0.5 * (((-((assign19480_e19503 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign19480_e19517) + (assign19480_e19507 * ((-((assign19480_e19510 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign19480_e19495 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign19480_e19520) + (assign19480_e19499 * (0.5 * (((-((assign19480_e19503 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign19480_e19517) + (assign19480_e19507 * ((-((assign19480_e19510 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19480_e19525;
        var_tmp_dn5 = assign19480_e19525_d_n5;
        var_tmp_dn6 = assign19480_e19525_d_n6;
        var_tmp_dn7 = assign19480_e19525_d_n7;
        var_tmp_dn8 = assign19480_e19525_d_n8;

        let (assign19490_e19545, assign19490_e19545_d_n5, assign19490_e19545_d_n6, assign19490_e19545_d_n7, assign19490_e19545_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard336 == 0.0)) {
        let assign19490_e19538: f64 = (var_v2 * var_fmaxr);
        let assign19490_e19540: f64 = (assign19490_e19538 * var_fmaxr);
        let assign19490_e19542: f64 = (assign19490_e19540 * var_tmp);
        let assign19490_e19543: f64 = (p.p852 * assign19490_e19542);
        (assign19490_e19543, (p.p852 * (((((var_v2 * var_fmaxr_dn5) * var_fmaxr) + (assign19490_e19538 * var_fmaxr_dn5)) * var_tmp) + (assign19490_e19540 * var_tmp_dn5))), (p.p852 * (((((var_v2 * var_fmaxr_dn6) * var_fmaxr) + (assign19490_e19538 * var_fmaxr_dn6)) * var_tmp) + (assign19490_e19540 * var_tmp_dn6))), (p.p852 * (((((var_v2 * var_fmaxr_dn7) * var_fmaxr) + (assign19490_e19538 * var_fmaxr_dn7)) * var_tmp) + (assign19490_e19540 * var_tmp_dn7))), (p.p852 * (((((var_v2 * var_fmaxr_dn8) * var_fmaxr) + (assign19490_e19538 * var_fmaxr_dn8)) * var_tmp) + (assign19490_e19540 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign19490_e19545;
        var_ibbt_dn5 = assign19490_e19545_d_n5;
        var_ibbt_dn6 = assign19490_e19545_d_n6;
        var_ibbt_dn7 = assign19490_e19545_d_n7;
        var_ibbt_dn8 = assign19490_e19545_d_n8;

        let assign19500_e19548: f64 = if p.p861 > 1000.0 { 1.0 } else { 0.0 };
        var_guard340 = assign19500_e19548;

        let (assign19510_e19559, assign19510_e19559_d_n5, assign19510_e19559_d_n6, assign19510_e19559_d_n7, assign19510_e19559_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard340 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign19510_e19559;
        var_fbreakdown_dn5 = assign19510_e19559_d_n5;
        var_fbreakdown_dn6 = assign19510_e19559_d_n6;
        var_fbreakdown_dn7 = assign19510_e19559_d_n7;
        var_fbreakdown_dn8 = assign19510_e19559_d_n8;

        let assign19520_e19562: f64 = (-var_alphaav);
        let assign19520_e19564: f64 = (assign19520_e19562 * p.p861);
        let assign19520_e19565: f64 = if var_vav > assign19520_e19564 { 1.0 } else { 0.0 };
        var_guard341 = assign19520_e19565;

        let assign19530_e19568: f64 = if p.p864 == 4.0 { 1.0 } else { 0.0 };
        var_guard342 = assign19530_e19568;

        let (assign19540_e19598, assign19540_e19598_d_n5, assign19540_e19598_d_n6, assign19540_e19598_d_n7, assign19540_e19598_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard340 == 0.0)) && (var_guard341 != 0.0)) && (var_guard342 != 0.0)) {
        let assign19540_e19584: f64 = (var_vav * var_vbrinvsti);
        let assign19540_e19587: f64 = (var_vav * var_vbrinvsti);
        let assign19540_e19588: f64 = (assign19540_e19584 * assign19540_e19587);
        let assign19540_e19591: f64 = (var_vav * var_vbrinvsti);
        let assign19540_e19592: f64 = (assign19540_e19588 * assign19540_e19591);
        let assign19540_e19595: f64 = (var_vav * var_vbrinvsti);
        let assign19540_e19596: f64 = (assign19540_e19592 * assign19540_e19595);
        (assign19540_e19596, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19540_e19598;
        var_tmp_dn5 = assign19540_e19598_d_n5;
        var_tmp_dn6 = assign19540_e19598_d_n6;
        var_tmp_dn7 = assign19540_e19598_d_n7;
        var_tmp_dn8 = assign19540_e19598_d_n8;

        let (assign19550_e19620, assign19550_e19620_d_n5, assign19550_e19620_d_n6, assign19550_e19620_d_n7, assign19550_e19620_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard340 == 0.0)) && (var_guard341 != 0.0)) && (var_guard342 == 0.0)) {
        let assign19550_e19615: f64 = (var_vav * var_vbrinvsti);
        let assign19550_e19616: f64 = (assign19550_e19615).abs();
        let assign19550_e19618: f64 = (assign19550_e19616).powf(p.p864);
        (assign19550_e19618, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19550_e19620;
        var_tmp_dn5 = assign19550_e19620_d_n5;
        var_tmp_dn6 = assign19550_e19620_d_n6;
        var_tmp_dn7 = assign19550_e19620_d_n7;
        var_tmp_dn8 = assign19550_e19620_d_n8;

        let (assign19560_e19638, assign19560_e19638_d_n5, assign19560_e19638_d_n6, assign19560_e19638_d_n7, assign19560_e19638_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard340 == 0.0)) && (var_guard341 != 0.0)) {
        let assign19560_e19635: f64 = (1.0 - var_tmp);
        let assign19560_e19636: f64 = (1.0 / assign19560_e19635);
        (assign19560_e19636, (-((-var_tmp_dn5) / (assign19560_e19635 * assign19560_e19635))), (-((-var_tmp_dn6) / (assign19560_e19635 * assign19560_e19635))), (-((-var_tmp_dn7) / (assign19560_e19635 * assign19560_e19635))), (-((-var_tmp_dn8) / (assign19560_e19635 * assign19560_e19635))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign19560_e19638;
        var_fbreakdown_dn5 = assign19560_e19638_d_n5;
        var_fbreakdown_dn6 = assign19560_e19638_d_n6;
        var_fbreakdown_dn7 = assign19560_e19638_d_n7;
        var_fbreakdown_dn8 = assign19560_e19638_d_n8;

        let (assign19570_e19661, assign19570_e19661_d_n5, assign19570_e19661_d_n6, assign19570_e19661_d_n7, assign19570_e19661_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) && (var_guard340 == 0.0)) && (var_guard341 == 0.0)) {
        let assign19570_e19655: f64 = (var_alphaav * p.p861);
        let assign19570_e19656: f64 = (var_vav + assign19570_e19655);
        let assign19570_e19658: f64 = (assign19570_e19656 * var_slopesti);
        let assign19570_e19659: f64 = (var_fstopsti + assign19570_e19658);
        (assign19570_e19659, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign19570_e19661;
        var_fbreakdown_dn5 = assign19570_e19661_d_n5;
        var_fbreakdown_dn6 = assign19570_e19661_d_n6;
        var_fbreakdown_dn7 = assign19570_e19661_d_n7;
        var_fbreakdown_dn8 = assign19570_e19661_d_n8;

        let (assign19580_e19680, assign19580_e19680_d_n5, assign19580_e19680_d_n6, assign19580_e19680_d_n7, assign19580_e19680_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard326 == 0.0)) {
        let assign19580_e19671: f64 = (var_id__blk219 + var_isrh);
        let assign19580_e19673: f64 = (assign19580_e19671 + var_itat);
        let assign19580_e19675: f64 = (assign19580_e19673 + var_ibbt);
        let assign19580_e19676: f64 = (p.p29 * assign19580_e19675);
        let assign19580_e19678: f64 = (assign19580_e19676 * var_fbreakdown);
        (assign19580_e19678, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign19580_e19676 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign19580_e19676 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign19580_e19676 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign19580_e19676 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign19580_e19680;
        var_ijunsti_dn5 = assign19580_e19680_d_n5;
        var_ijunsti_dn6 = assign19580_e19680_d_n6;
        var_ijunsti_dn7 = assign19580_e19680_d_n7;
        var_ijunsti_dn8 = assign19580_e19680_d_n8;

        let assign19590_e19683: f64 = if var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard343 = assign19590_e19683;

        let (assign19600_e19691, assign19600_e19691_d_n5, assign19600_e19691_d_n6, assign19600_e19691_d_n7, assign19600_e19691_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign19600_e19691;
        var_ijungat_dn5 = assign19600_e19691_d_n5;
        var_ijungat_dn6 = assign19600_e19691_d_n6;
        var_ijungat_dn7 = assign19600_e19691_d_n7;
        var_ijungat_dn8 = assign19600_e19691_d_n8;

        let (assign19610_e19702,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) {
        let assign19610_e19700: f64 = (var_idsatgat * var_idmult);
        (assign19610_e19700,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign19610_e19702;

        let assign19620_e19709: f64 = if ((p.p842 == 0.0) && (p.p847 == 0.0)) { 1.0 } else { 0.0 };
        var_guard344 = assign19620_e19709;

        let (assign19630_e19720, assign19630_e19720_d_n5, assign19630_e19720_d_n6, assign19630_e19720_d_n7, assign19630_e19720_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard344 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign19630_e19720;
        var_isrh_dn5 = assign19630_e19720_d_n5;
        var_isrh_dn6 = assign19630_e19720_d_n6;
        var_isrh_dn7 = assign19630_e19720_d_n7;
        var_isrh_dn8 = assign19630_e19720_d_n8;

        let (assign19640_e19734,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard344 == 0.0)) {
        let assign19640_e19732: f64 = (var_vbigat - var_vjsrh);
        (assign19640_e19732,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign19640_e19734;

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
        *var_guard334_slot = var_guard334;
        *var_guard335_slot = var_guard335;
        *var_guard336_slot = var_guard336;
        *var_guard337_slot = var_guard337;
        *var_guard338_slot = var_guard338;
        *var_guard339_slot = var_guard339;
        *var_guard340_slot = var_guard340;
        *var_guard341_slot = var_guard341;
        *var_guard342_slot = var_guard342;
        *var_guard343_slot = var_guard343;
        *var_guard344_slot = var_guard344;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_id__blk219_slot = var_id__blk219;
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

    pub(super) fn stamp_transient_block_33(
        p: &Parameters,
        var_atatgat: f64,
        var_berfc: f64,
        var_btatpartgat: f64,
        var_cerfc: f64,
        var_ftdgat: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard343: f64,
        var_guard344: f64,
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
        var_guard345_slot: &mut f64,
        var_guard346_slot: &mut f64,
        var_guard347_slot: &mut f64,
        var_guard348_slot: &mut f64,
        var_guard349_slot: &mut f64,
        var_guard350_slot: &mut f64,
        var_guard351_slot: &mut f64,
        var_guard352_slot: &mut f64,
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
        let mut var_guard345: f64 = *var_guard345_slot;
        let mut var_guard346: f64 = *var_guard346_slot;
        let mut var_guard347: f64 = *var_guard347_slot;
        let mut var_guard348: f64 = *var_guard348_slot;
        let mut var_guard349: f64 = *var_guard349_slot;
        let mut var_guard350: f64 = *var_guard350_slot;
        let mut var_guard351: f64 = *var_guard351_slot;
        let mut var_guard352: f64 = *var_guard352_slot;
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

        let (assign19650_e19753,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard344 == 0.0)) {
        let assign19650_e19748: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign19650_e19749: f64 = (1.0 - assign19650_e19748);
        let assign19650_e19750: f64 = (assign19650_e19749).sqrt();
        let assign19650_e19751: f64 = (1.0 - assign19650_e19750);
        (assign19650_e19751,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign19650_e19753;

        let assign19660_e19756: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        var_guard345 = assign19660_e19756;

        let (assign19670_e19770,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard344 == 0.0)) && (var_guard345 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign19670_e19770;

        let (assign19680_e19802,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard344 == 0.0)) && (var_guard345 == 0.0)) {
        let assign19680_e19785: f64 = (var_wsrhstep * var_wsrhstep);
        let assign19680_e19787: f64 = (var_wsrhstep).ln();
        let assign19680_e19788: f64 = (assign19680_e19785 * assign19680_e19787);
        let assign19680_e19791: f64 = (1.0 - var_wsrhstep);
        let assign19680_e19792: f64 = (assign19680_e19788 / assign19680_e19791);
        let assign19680_e19794: f64 = (assign19680_e19792 + var_wsrhstep);
        let assign19680_e19798: f64 = (2.0 * p.p833);
        let assign19680_e19799: f64 = (1.0 - assign19680_e19798);
        let assign19680_e19800: f64 = (assign19680_e19794 * assign19680_e19799);
        (assign19680_e19800,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign19680_e19802;

        let (assign19690_e19816,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard344 == 0.0)) {
        let assign19690_e19814: f64 = (var_wsrhstep + var_dwsrh);
        (assign19690_e19814,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign19690_e19816;

        let assign19700_e19819: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        var_guard346 = assign19700_e19819;

        let (assign19710_e19836, assign19710_e19836_d_n5, assign19710_e19836_d_n6, assign19710_e19836_d_n7, assign19710_e19836_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard344 == 0.0)) && (var_guard346 != 0.0)) {
        let assign19710_e19833: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign19710_e19834: f64 = (assign19710_e19833).sqrt();
        (assign19710_e19834, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19710_e19836;
        var_tmp_dn5 = assign19710_e19836_d_n5;
        var_tmp_dn6 = assign19710_e19836_d_n6;
        var_tmp_dn7 = assign19710_e19836_d_n7;
        var_tmp_dn8 = assign19710_e19836_d_n8;

        let (assign19720_e19855, assign19720_e19855_d_n5, assign19720_e19855_d_n6, assign19720_e19855_d_n7, assign19720_e19855_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard344 == 0.0)) && (var_guard346 == 0.0)) {
        let assign19720_e19851: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign19720_e19853: f64 = (assign19720_e19851).powf(p.p833);
        (assign19720_e19853, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19720_e19855;
        var_tmp_dn5 = assign19720_e19855_d_n5;
        var_tmp_dn6 = assign19720_e19855_d_n6;
        var_tmp_dn7 = assign19720_e19855_d_n7;
        var_tmp_dn8 = assign19720_e19855_d_n8;

        let (assign19730_e19869, assign19730_e19869_d_n5, assign19730_e19869_d_n6, assign19730_e19869_d_n7, assign19730_e19869_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard344 == 0.0)) {
        let assign19730_e19867: f64 = (var_wdepnulrgat * var_tmp);
        (assign19730_e19867, (var_wdepnulrgat * var_tmp_dn5), (var_wdepnulrgat * var_tmp_dn6), (var_wdepnulrgat * var_tmp_dn7), (var_wdepnulrgat * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign19730_e19869;
        var_wdep_dn5 = assign19730_e19869_d_n5;
        var_wdep_dn6 = assign19730_e19869_d_n6;
        var_wdep_dn7 = assign19730_e19869_d_n7;
        var_wdep_dn8 = assign19730_e19869_d_n8;

        let (assign19740_e19887, assign19740_e19887_d_n5, assign19740_e19887_d_n6, assign19740_e19887_d_n7, assign19740_e19887_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard344 == 0.0)) {
        let assign19740_e19882: f64 = (var_zinv - 1.0);
        let assign19740_e19884: f64 = (assign19740_e19882 * var_wdep);
        let assign19740_e19885: f64 = (var_ftdgat * assign19740_e19884);
        (assign19740_e19885, (var_ftdgat * (assign19740_e19882 * var_wdep_dn5)), (var_ftdgat * (assign19740_e19882 * var_wdep_dn6)), (var_ftdgat * (assign19740_e19882 * var_wdep_dn7)), (var_ftdgat * (assign19740_e19882 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign19740_e19887;
        var_asrh_dn5 = assign19740_e19887_d_n5;
        var_asrh_dn6 = assign19740_e19887_d_n6;
        var_asrh_dn7 = assign19740_e19887_d_n7;
        var_asrh_dn8 = assign19740_e19887_d_n8;

        let (assign19750_e19903, assign19750_e19903_d_n5, assign19750_e19903_d_n6, assign19750_e19903_d_n7, assign19750_e19903_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard344 == 0.0)) {
        let assign19750_e19900: f64 = (var_asrh * var_wsrh);
        let assign19750_e19901: f64 = (p.p842 * assign19750_e19900);
        (assign19750_e19901, (p.p842 * (var_asrh_dn5 * var_wsrh)), (p.p842 * (var_asrh_dn6 * var_wsrh)), (p.p842 * (var_asrh_dn7 * var_wsrh)), (p.p842 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign19750_e19903;
        var_isrh_dn5 = assign19750_e19903_d_n5;
        var_isrh_dn6 = assign19750_e19903_d_n6;
        var_isrh_dn7 = assign19750_e19903_d_n7;
        var_isrh_dn8 = assign19750_e19903_d_n8;

        let assign19760_e19906: f64 = if p.p847 == 0.0 { 1.0 } else { 0.0 };
        var_guard347 = assign19760_e19906;

        let (assign19770_e19917, assign19770_e19917_d_n5, assign19770_e19917_d_n6, assign19770_e19917_d_n7, assign19770_e19917_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign19770_e19917;
        var_itat_dn5 = assign19770_e19917_d_n5;
        var_itat_dn6 = assign19770_e19917_d_n6;
        var_itat_dn7 = assign19770_e19917_d_n7;
        var_itat_dn8 = assign19770_e19917_d_n8;

        let (assign19780_e19935, assign19780_e19935_d_n5, assign19780_e19935_d_n6, assign19780_e19935_d_n7, assign19780_e19935_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) {
        let assign19780_e19930: f64 = (var_wdep * var_one_minus_pgat);
        let assign19780_e19932: f64 = (assign19780_e19930 / var_vbi_minus_vjsrh);
        let assign19780_e19933: f64 = (var_btatpartgat * assign19780_e19932);
        (assign19780_e19933, (var_btatpartgat * ((var_wdep_dn5 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn6 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn7 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn8 * var_one_minus_pgat) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign19780_e19935;
        var_btat_dn5 = assign19780_e19935_d_n5;
        var_btat_dn6 = assign19780_e19935_d_n6;
        var_btat_dn7 = assign19780_e19935_d_n7;
        var_btat_dn8 = assign19780_e19935_d_n8;

        let (assign19790_e19951, assign19790_e19951_d_n5, assign19790_e19951_d_n6, assign19790_e19951_d_n7, assign19790_e19951_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) {
        let assign19790_e19947: f64 = (0.666666666666667 * var_atatgat);
        let assign19790_e19949: f64 = (assign19790_e19947 / var_btat);
        (assign19790_e19949, (-((assign19790_e19947 * var_btat_dn5) / (var_btat * var_btat))), (-((assign19790_e19947 * var_btat_dn6) / (var_btat * var_btat))), (-((assign19790_e19947 * var_btat_dn7) / (var_btat * var_btat))), (-((assign19790_e19947 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign19790_e19951;
        var_twoatatoverthreebtat_dn5 = assign19790_e19951_d_n5;
        var_twoatatoverthreebtat_dn6 = assign19790_e19951_d_n6;
        var_twoatatoverthreebtat_dn7 = assign19790_e19951_d_n7;
        var_twoatatoverthreebtat_dn8 = assign19790_e19951_d_n8;

        let (assign19800_e19965, assign19800_e19965_d_n5, assign19800_e19965_d_n6, assign19800_e19965_d_n7, assign19800_e19965_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) {
        let assign19800_e19963: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign19800_e19963, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign19800_e19965;
        var_umaxbeforelimiting_dn5 = assign19800_e19965_d_n5;
        var_umaxbeforelimiting_dn6 = assign19800_e19965_d_n6;
        var_umaxbeforelimiting_dn7 = assign19800_e19965_d_n7;
        var_umaxbeforelimiting_dn8 = assign19800_e19965_d_n8;

        let (assign19810_e19986, assign19810_e19986_d_n5, assign19810_e19986_d_n6, assign19810_e19986_d_n7, assign19810_e19986_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) {
        let assign19810_e19977: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign19810_e19980: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign19810_e19982: f64 = (assign19810_e19980 + 1.0);
        let assign19810_e19983: f64 = (assign19810_e19977 / assign19810_e19982);
        let assign19810_e19984: f64 = (assign19810_e19983).sqrt();
        (assign19810_e19984, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign19810_e19982) - (assign19810_e19977 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign19810_e19982 * assign19810_e19982)) / (2.0 * assign19810_e19984)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign19810_e19982) - (assign19810_e19977 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign19810_e19982 * assign19810_e19982)) / (2.0 * assign19810_e19984)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign19810_e19982) - (assign19810_e19977 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign19810_e19982 * assign19810_e19982)) / (2.0 * assign19810_e19984)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign19810_e19982) - (assign19810_e19977 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign19810_e19982 * assign19810_e19982)) / (2.0 * assign19810_e19984)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign19810_e19986;
        var_umax_dn5 = assign19810_e19986_d_n5;
        var_umax_dn6 = assign19810_e19986_d_n6;
        var_umax_dn7 = assign19810_e19986_d_n7;
        var_umax_dn8 = assign19810_e19986_d_n8;

        let (assign19820_e19999, assign19820_e19999_d_n5, assign19820_e19999_d_n6, assign19820_e19999_d_n7, assign19820_e19999_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) {
        let assign19820_e19997: f64 = (var_umax).sqrt();
        (assign19820_e19997, (var_umax_dn5 / (2.0 * assign19820_e19997)), (var_umax_dn6 / (2.0 * assign19820_e19997)), (var_umax_dn7 / (2.0 * assign19820_e19997)), (var_umax_dn8 / (2.0 * assign19820_e19997)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign19820_e19999;
        var_sqrtumax_dn5 = assign19820_e19999_d_n5;
        var_sqrtumax_dn6 = assign19820_e19999_d_n6;
        var_sqrtumax_dn7 = assign19820_e19999_d_n7;
        var_sqrtumax_dn8 = assign19820_e19999_d_n8;

        let (assign19830_e20013, assign19830_e20013_d_n5, assign19830_e20013_d_n6, assign19830_e20013_d_n7, assign19830_e20013_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) {
        let assign19830_e20011: f64 = (var_umax * var_sqrtumax);
        (assign19830_e20011, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign19830_e20013;
        var_umaxpoweronepointfive_dn5 = assign19830_e20013_d_n5;
        var_umaxpoweronepointfive_dn6 = assign19830_e20013_d_n6;
        var_umaxpoweronepointfive_dn7 = assign19830_e20013_d_n7;
        var_umaxpoweronepointfive_dn8 = assign19830_e20013_d_n8;

        let assign19840_e20015: f64 = (-p.p833);
        let assign19840_e20017: f64 = (assign19840_e20015 * var_one_over_one_minus_pgat);
        let assign19840_e20019: f64 = (-1.0);
        let assign19840_e20020: f64 = if assign19840_e20017 == assign19840_e20019 { 1.0 } else { 0.0 };
        var_guard348 = assign19840_e20020;

        let (assign19850_e20040, assign19850_e20040_d_n5, assign19850_e20040_d_n6, assign19850_e20040_d_n7, assign19850_e20040_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) && (var_guard348 != 0.0)) {
        let assign19850_e20036: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign19850_e20037: f64 = (1.0 + assign19850_e20036);
        let assign19850_e20038: f64 = (1.0 / assign19850_e20037);
        (assign19850_e20038, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign19850_e20037 * assign19850_e20037))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign19850_e20037 * assign19850_e20037))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign19850_e20037 * assign19850_e20037))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign19850_e20037 * assign19850_e20037))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign19850_e20040;
        var_wgamma_dn5 = assign19850_e20040_d_n5;
        var_wgamma_dn6 = assign19850_e20040_d_n6;
        var_wgamma_dn7 = assign19850_e20040_d_n7;
        var_wgamma_dn8 = assign19850_e20040_d_n8;

        let (assign19860_e20064, assign19860_e20064_d_n5, assign19860_e20064_d_n6, assign19860_e20064_d_n7, assign19860_e20064_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) && (var_guard348 == 0.0)) {
        let assign19860_e20056: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign19860_e20057: f64 = (1.0 + assign19860_e20056);
        let assign19860_e20059: f64 = (-p.p833);
        let assign19860_e20061: f64 = (assign19860_e20059 * var_one_over_one_minus_pgat);
        let assign19860_e20062: f64 = (assign19860_e20057).powf(assign19860_e20061);
        (assign19860_e20062, if 0.0 == 0.0 && ((assign19860_e20061) as f64).is_finite() && ((assign19860_e20061) as f64).fract() == 0.0 { if assign19860_e20061 == 0.0 { 0.0 } else { (assign19860_e20061 * ((assign19860_e20057).powf(assign19860_e20061 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign19860_e20062 * (assign19860_e20061 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign19860_e20057))) }, if 0.0 == 0.0 && ((assign19860_e20061) as f64).is_finite() && ((assign19860_e20061) as f64).fract() == 0.0 { if assign19860_e20061 == 0.0 { 0.0 } else { (assign19860_e20061 * ((assign19860_e20057).powf(assign19860_e20061 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign19860_e20062 * (assign19860_e20061 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign19860_e20057))) }, if 0.0 == 0.0 && ((assign19860_e20061) as f64).is_finite() && ((assign19860_e20061) as f64).fract() == 0.0 { if assign19860_e20061 == 0.0 { 0.0 } else { (assign19860_e20061 * ((assign19860_e20057).powf(assign19860_e20061 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign19860_e20062 * (assign19860_e20061 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign19860_e20057))) }, if 0.0 == 0.0 && ((assign19860_e20061) as f64).is_finite() && ((assign19860_e20061) as f64).fract() == 0.0 { if assign19860_e20061 == 0.0 { 0.0 } else { (assign19860_e20061 * ((assign19860_e20057).powf(assign19860_e20061 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign19860_e20062 * (assign19860_e20061 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign19860_e20057))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign19860_e20064;
        var_wgamma_dn5 = assign19860_e20064_d_n5;
        var_wgamma_dn6 = assign19860_e20064_d_n6;
        var_wgamma_dn7 = assign19860_e20064_d_n7;
        var_wgamma_dn8 = assign19860_e20064_d_n8;

        let (assign19870_e20082, assign19870_e20082_d_n5, assign19870_e20082_d_n6, assign19870_e20082_d_n7, assign19870_e20082_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) {
        let assign19870_e20076: f64 = (var_wsrh * var_wgamma);
        let assign19870_e20079: f64 = (var_wsrh + var_wgamma);
        let assign19870_e20080: f64 = (assign19870_e20076 / assign19870_e20079);
        (assign19870_e20080, ((((var_wsrh * var_wgamma_dn5) * assign19870_e20079) - (assign19870_e20076 * var_wgamma_dn5)) / (assign19870_e20079 * assign19870_e20079)), ((((var_wsrh * var_wgamma_dn6) * assign19870_e20079) - (assign19870_e20076 * var_wgamma_dn6)) / (assign19870_e20079 * assign19870_e20079)), ((((var_wsrh * var_wgamma_dn7) * assign19870_e20079) - (assign19870_e20076 * var_wgamma_dn7)) / (assign19870_e20079 * assign19870_e20079)), ((((var_wsrh * var_wgamma_dn8) * assign19870_e20079) - (assign19870_e20076 * var_wgamma_dn8)) / (assign19870_e20079 * assign19870_e20079)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign19870_e20082;
        var_wtat_dn5 = assign19870_e20082_d_n5;
        var_wtat_dn6 = assign19870_e20082_d_n6;
        var_wtat_dn7 = assign19870_e20082_d_n7;
        var_wtat_dn8 = assign19870_e20082_d_n8;

        let (assign19880_e20099, assign19880_e20099_d_n5, assign19880_e20099_d_n6, assign19880_e20099_d_n7, assign19880_e20099_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) {
        let assign19880_e20095: f64 = (var_btat / var_sqrtumax);
        let assign19880_e20096: f64 = (0.375 * assign19880_e20095);
        let assign19880_e20097: f64 = (assign19880_e20096).sqrt();
        (assign19880_e20097, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign19880_e20097)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign19880_e20097)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign19880_e20097)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign19880_e20097)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign19880_e20099;
        var_ktat_dn5 = assign19880_e20099_d_n5;
        var_ktat_dn6 = assign19880_e20099_d_n6;
        var_ktat_dn7 = assign19880_e20099_d_n7;
        var_ktat_dn8 = assign19880_e20099_d_n8;

        let (assign19890_e20117, assign19890_e20117_d_n5, assign19890_e20117_d_n6, assign19890_e20117_d_n7, assign19890_e20117_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) {
        let assign19890_e20112: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign19890_e20113: f64 = (2.0 * assign19890_e20112);
        let assign19890_e20115: f64 = (assign19890_e20113 - var_umax);
        (assign19890_e20115, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign19890_e20117;
        var_ltat_dn5 = assign19890_e20117_d_n5;
        var_ltat_dn6 = assign19890_e20117_d_n6;
        var_ltat_dn7 = assign19890_e20117_d_n7;
        var_ltat_dn8 = assign19890_e20117_d_n8;

        let (assign19900_e20143, assign19900_e20143_d_n5, assign19900_e20143_d_n6, assign19900_e20143_d_n7, assign19900_e20143_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) {
        let assign19900_e20129: f64 = (var_atatgat * var_twoatatoverthreebtat);
        let assign19900_e20131: f64 = (assign19900_e20129 * var_sqrtumax);
        let assign19900_e20134: f64 = (var_atatgat * var_umax);
        let assign19900_e20135: f64 = (assign19900_e20131 - assign19900_e20134);
        let assign19900_e20139: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign19900_e20140: f64 = (0.5 * assign19900_e20139);
        let assign19900_e20141: f64 = (assign19900_e20135 + assign19900_e20140);
        (assign19900_e20141, (((((var_atatgat * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign19900_e20129 * var_sqrtumax_dn5)) - (var_atatgat * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatgat * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign19900_e20129 * var_sqrtumax_dn6)) - (var_atatgat * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign19900_e20129 * var_sqrtumax_dn7)) - (var_atatgat * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign19900_e20129 * var_sqrtumax_dn8)) - (var_atatgat * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign19900_e20143;
        var_mtat_dn5 = assign19900_e20143_d_n5;
        var_mtat_dn6 = assign19900_e20143_d_n6;
        var_mtat_dn7 = assign19900_e20143_d_n7;
        var_mtat_dn8 = assign19900_e20143_d_n8;

        let (assign19910_e20159, assign19910_e20159_d_n5, assign19910_e20159_d_n6, assign19910_e20159_d_n7, assign19910_e20159_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) {
        let assign19910_e20155: f64 = (var_ltat - 1.0);
        let assign19910_e20157: f64 = (assign19910_e20155 * var_ktat);
        (assign19910_e20157, ((var_ltat_dn5 * var_ktat) + (assign19910_e20155 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign19910_e20155 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign19910_e20155 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign19910_e20155 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign19910_e20159;
        var_xerfc_dn5 = assign19910_e20159_d_n5;
        var_xerfc_dn6 = assign19910_e20159_d_n6;
        var_xerfc_dn7 = assign19910_e20159_d_n7;
        var_xerfc_dn8 = assign19910_e20159_d_n8;

        let (assign19920_e20173, assign19920_e20173_d_n5, assign19920_e20173_d_n6, assign19920_e20173_d_n7, assign19920_e20173_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) {
        let assign19920_e20171: f64 = (var_xerfc * var_xerfc);
        (assign19920_e20171, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign19920_e20173;
        var_ysq_dn5 = assign19920_e20173_d_n5;
        var_ysq_dn6 = assign19920_e20173_d_n6;
        var_ysq_dn7 = assign19920_e20173_d_n7;
        var_ysq_dn8 = assign19920_e20173_d_n8;

        let assign19930_e20176: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard349 = assign19930_e20176;

        let (assign19940_e20196, assign19940_e20196_d_n5, assign19940_e20196_d_n6, assign19940_e20196_d_n7, assign19940_e20196_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) && (var_guard349 != 0.0)) {
        let assign19940_e20192: f64 = (var_perfc * var_xerfc);
        let assign19940_e20193: f64 = (1.0 + assign19940_e20192);
        let assign19940_e20194: f64 = (1.0 / assign19940_e20193);
        (assign19940_e20194, (-((var_perfc * var_xerfc_dn5) / (assign19940_e20193 * assign19940_e20193))), (-((var_perfc * var_xerfc_dn6) / (assign19940_e20193 * assign19940_e20193))), (-((var_perfc * var_xerfc_dn7) / (assign19940_e20193 * assign19940_e20193))), (-((var_perfc * var_xerfc_dn8) / (assign19940_e20193 * assign19940_e20193))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign19940_e20196;
        var_terfc_dn5 = assign19940_e20196_d_n5;
        var_terfc_dn6 = assign19940_e20196_d_n6;
        var_terfc_dn7 = assign19940_e20196_d_n7;
        var_terfc_dn8 = assign19940_e20196_d_n8;

        let (assign19950_e20217, assign19950_e20217_d_n5, assign19950_e20217_d_n6, assign19950_e20217_d_n7, assign19950_e20217_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) && (var_guard349 == 0.0)) {
        let assign19950_e20213: f64 = (var_perfc * var_xerfc);
        let assign19950_e20214: f64 = (1.0 - assign19950_e20213);
        let assign19950_e20215: f64 = (1.0 / assign19950_e20214);
        (assign19950_e20215, (-((-(var_perfc * var_xerfc_dn5)) / (assign19950_e20214 * assign19950_e20214))), (-((-(var_perfc * var_xerfc_dn6)) / (assign19950_e20214 * assign19950_e20214))), (-((-(var_perfc * var_xerfc_dn7)) / (assign19950_e20214 * assign19950_e20214))), (-((-(var_perfc * var_xerfc_dn8)) / (assign19950_e20214 * assign19950_e20214))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign19950_e20217;
        var_terfc_dn5 = assign19950_e20217_d_n5;
        var_terfc_dn6 = assign19950_e20217_d_n6;
        var_terfc_dn7 = assign19950_e20217_d_n7;
        var_terfc_dn8 = assign19950_e20217_d_n8;

        let assign19960_e20219: f64 = (-var_ysq);
        let assign19960_e20221: f64 = (assign19960_e20219 + var_mtat);
        let assign19960_e20223: f64 = (-230.25850929940458);
        let assign19960_e20224: f64 = if assign19960_e20221 > assign19960_e20223 { 1.0 } else { 0.0 };
        var_guard350 = assign19960_e20224;

        let (assign19970_e20242, assign19970_e20242_d_n5, assign19970_e20242_d_n6, assign19970_e20242_d_n7, assign19970_e20242_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) && (var_guard350 != 0.0)) {
        let assign19970_e20237: f64 = (-var_ysq);
        let assign19970_e20239: f64 = (assign19970_e20237 + var_mtat);
        let assign19970_e20240: f64 = (assign19970_e20239).exp();
        (assign19970_e20240, (assign19970_e20240 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign19970_e20240 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign19970_e20240 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign19970_e20240 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19970_e20242;
        var_tmp_dn5 = assign19970_e20242_d_n5;
        var_tmp_dn6 = assign19970_e20242_d_n6;
        var_tmp_dn7 = assign19970_e20242_d_n7;
        var_tmp_dn8 = assign19970_e20242_d_n8;

        let (assign19980_e20291, assign19980_e20291_d_n5, assign19980_e20291_d_n6, assign19980_e20291_d_n7, assign19980_e20291_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) && (var_guard350 == 0.0)) {
        let assign19980_e20258: f64 = (-230.25850929940458);
        let assign19980_e20260: f64 = (-var_ysq);
        let assign19980_e20262: f64 = (assign19980_e20260 + var_mtat);
        let assign19980_e20263: f64 = (assign19980_e20258 - assign19980_e20262);
        let assign19980_e20267: f64 = (-230.25850929940458);
        let assign19980_e20269: f64 = (-var_ysq);
        let assign19980_e20271: f64 = (assign19980_e20269 + var_mtat);
        let assign19980_e20272: f64 = (assign19980_e20267 - assign19980_e20271);
        let assign19980_e20275: f64 = (-230.25850929940458);
        let assign19980_e20277: f64 = (-var_ysq);
        let assign19980_e20279: f64 = (assign19980_e20277 + var_mtat);
        let assign19980_e20280: f64 = (assign19980_e20275 - assign19980_e20279);
        let assign19980_e20282: f64 = (assign19980_e20280 * 0.3333333333333333);
        let assign19980_e20283: f64 = (1.0 + assign19980_e20282);
        let assign19980_e20284: f64 = (assign19980_e20272 * assign19980_e20283);
        let assign19980_e20285: f64 = (0.5 * assign19980_e20284);
        let assign19980_e20286: f64 = (1.0 + assign19980_e20285);
        let assign19980_e20287: f64 = (assign19980_e20263 * assign19980_e20286);
        let assign19980_e20288: f64 = (1.0 + assign19980_e20287);
        let assign19980_e20289: f64 = (1e-100 / assign19980_e20288);
        (assign19980_e20289, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign19980_e20286) + (assign19980_e20263 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign19980_e20283) + (assign19980_e20272 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign19980_e20288 * assign19980_e20288))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign19980_e20286) + (assign19980_e20263 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign19980_e20283) + (assign19980_e20272 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign19980_e20288 * assign19980_e20288))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign19980_e20286) + (assign19980_e20263 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign19980_e20283) + (assign19980_e20272 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign19980_e20288 * assign19980_e20288))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign19980_e20286) + (assign19980_e20263 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign19980_e20283) + (assign19980_e20272 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign19980_e20288 * assign19980_e20288))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign19980_e20291;
        var_tmp_dn5 = assign19980_e20291_d_n5;
        var_tmp_dn6 = assign19980_e20291_d_n6;
        var_tmp_dn7 = assign19980_e20291_d_n7;
        var_tmp_dn8 = assign19980_e20291_d_n8;

        let (assign19990_e20321, assign19990_e20321_d_n5, assign19990_e20321_d_n6, assign19990_e20321_d_n7, assign19990_e20321_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) {
        let assign19990_e20303: f64 = (0.29214664 * var_terfc);
        let assign19990_e20307: f64 = (var_terfc * var_terfc);
        let assign19990_e20308: f64 = (var_berfc * assign19990_e20307);
        let assign19990_e20309: f64 = (assign19990_e20303 + assign19990_e20308);
        let assign19990_e20313: f64 = (var_terfc * var_terfc);
        let assign19990_e20315: f64 = (assign19990_e20313 * var_terfc);
        let assign19990_e20316: f64 = (var_cerfc * assign19990_e20315);
        let assign19990_e20317: f64 = (assign19990_e20309 + assign19990_e20316);
        let assign19990_e20319: f64 = (assign19990_e20317 * var_tmp);
        (assign19990_e20319, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign19990_e20313 * var_terfc_dn5)))) * var_tmp) + (assign19990_e20317 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign19990_e20313 * var_terfc_dn6)))) * var_tmp) + (assign19990_e20317 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign19990_e20313 * var_terfc_dn7)))) * var_tmp) + (assign19990_e20317 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign19990_e20313 * var_terfc_dn8)))) * var_tmp) + (assign19990_e20317 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign19990_e20321;
        var_erfcpos_dn5 = assign19990_e20321_d_n5;
        var_erfcpos_dn6 = assign19990_e20321_d_n6;
        var_erfcpos_dn7 = assign19990_e20321_d_n7;
        var_erfcpos_dn8 = assign19990_e20321_d_n8;

        let assign20000_e20324: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard351 = assign20000_e20324;

        let (assign20010_e20338, assign20010_e20338_d_n5, assign20010_e20338_d_n6, assign20010_e20338_d_n7, assign20010_e20338_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) && (var_guard351 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign20010_e20338;
        var_erfctimesexpmtat_dn5 = assign20010_e20338_d_n5;
        var_erfctimesexpmtat_dn6 = assign20010_e20338_d_n6;
        var_erfctimesexpmtat_dn7 = assign20010_e20338_d_n7;
        var_erfctimesexpmtat_dn8 = assign20010_e20338_d_n8;

        let assign20020_e20341: f64 = (-230.25850929940458);
        let assign20020_e20342: f64 = if var_mtat > assign20020_e20341 { 1.0 } else { 0.0 };
        var_guard352 = assign20020_e20342;

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
        *var_guard345_slot = var_guard345;
        *var_guard346_slot = var_guard346;
        *var_guard347_slot = var_guard347;
        *var_guard348_slot = var_guard348;
        *var_guard349_slot = var_guard349;
        *var_guard350_slot = var_guard350;
        *var_guard351_slot = var_guard351;
        *var_guard352_slot = var_guard352;
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

    pub(super) fn stamp_transient_block_34(
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
        var_guard182: f64,
        var_guard199: f64,
        var_guard343: f64,
        var_guard347: f64,
        var_guard351: f64,
        var_guard352: f64,
        var_id__blk219: f64,
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
        var_v2: f64,
        var_v3: f64,
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
        var_guard353_slot: &mut f64,
        var_guard354_slot: &mut f64,
        var_guard355_slot: &mut f64,
        var_guard356_slot: &mut f64,
        var_guard357_slot: &mut f64,
        var_guard358_slot: &mut f64,
        var_guard359_slot: &mut f64,
        var_guard360_slot: &mut f64,
        var_guard361_slot: &mut f64,
        var_guard362_slot: &mut f64,
        var_guard363_slot: &mut f64,
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
        let mut var_guard353: f64 = *var_guard353_slot;
        let mut var_guard354: f64 = *var_guard354_slot;
        let mut var_guard355: f64 = *var_guard355_slot;
        let mut var_guard356: f64 = *var_guard356_slot;
        let mut var_guard357: f64 = *var_guard357_slot;
        let mut var_guard358: f64 = *var_guard358_slot;
        let mut var_guard359: f64 = *var_guard359_slot;
        let mut var_guard360: f64 = *var_guard360_slot;
        let mut var_guard361: f64 = *var_guard361_slot;
        let mut var_guard362: f64 = *var_guard362_slot;
        let mut var_guard363: f64 = *var_guard363_slot;
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

        let (assign20030_e20360, assign20030_e20360_d_n5, assign20030_e20360_d_n6, assign20030_e20360_d_n7, assign20030_e20360_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) && (var_guard351 == 0.0)) && (var_guard352 != 0.0)) {
        let assign20030_e20358: f64 = (var_mtat).exp();
        (assign20030_e20358, (assign20030_e20358 * var_mtat_dn5), (assign20030_e20358 * var_mtat_dn6), (assign20030_e20358 * var_mtat_dn7), (assign20030_e20358 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20030_e20360;
        var_tmp_dn5 = assign20030_e20360_d_n5;
        var_tmp_dn6 = assign20030_e20360_d_n6;
        var_tmp_dn7 = assign20030_e20360_d_n7;
        var_tmp_dn8 = assign20030_e20360_d_n8;

        let (assign20040_e20403, assign20040_e20403_d_n5, assign20040_e20403_d_n6, assign20040_e20403_d_n7, assign20040_e20403_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) && (var_guard351 == 0.0)) && (var_guard352 == 0.0)) {
        let assign20040_e20379: f64 = (-230.25850929940458);
        let assign20040_e20381: f64 = (assign20040_e20379 - var_mtat);
        let assign20040_e20385: f64 = (-230.25850929940458);
        let assign20040_e20387: f64 = (assign20040_e20385 - var_mtat);
        let assign20040_e20390: f64 = (-230.25850929940458);
        let assign20040_e20392: f64 = (assign20040_e20390 - var_mtat);
        let assign20040_e20394: f64 = (assign20040_e20392 * 0.3333333333333333);
        let assign20040_e20395: f64 = (1.0 + assign20040_e20394);
        let assign20040_e20396: f64 = (assign20040_e20387 * assign20040_e20395);
        let assign20040_e20397: f64 = (0.5 * assign20040_e20396);
        let assign20040_e20398: f64 = (1.0 + assign20040_e20397);
        let assign20040_e20399: f64 = (assign20040_e20381 * assign20040_e20398);
        let assign20040_e20400: f64 = (1.0 + assign20040_e20399);
        let assign20040_e20401: f64 = (1e-100 / assign20040_e20400);
        (assign20040_e20401, (-((1e-100 * (((-var_mtat_dn5) * assign20040_e20398) + (assign20040_e20381 * (0.5 * (((-var_mtat_dn5) * assign20040_e20395) + (assign20040_e20387 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign20040_e20400 * assign20040_e20400))), (-((1e-100 * (((-var_mtat_dn6) * assign20040_e20398) + (assign20040_e20381 * (0.5 * (((-var_mtat_dn6) * assign20040_e20395) + (assign20040_e20387 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign20040_e20400 * assign20040_e20400))), (-((1e-100 * (((-var_mtat_dn7) * assign20040_e20398) + (assign20040_e20381 * (0.5 * (((-var_mtat_dn7) * assign20040_e20395) + (assign20040_e20387 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign20040_e20400 * assign20040_e20400))), (-((1e-100 * (((-var_mtat_dn8) * assign20040_e20398) + (assign20040_e20381 * (0.5 * (((-var_mtat_dn8) * assign20040_e20395) + (assign20040_e20387 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign20040_e20400 * assign20040_e20400))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20040_e20403;
        var_tmp_dn5 = assign20040_e20403_d_n5;
        var_tmp_dn6 = assign20040_e20403_d_n6;
        var_tmp_dn7 = assign20040_e20403_d_n7;
        var_tmp_dn8 = assign20040_e20403_d_n8;

        let (assign20050_e20422, assign20050_e20422_d_n5, assign20050_e20422_d_n6, assign20050_e20422_d_n7, assign20050_e20422_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) && (var_guard351 == 0.0)) {
        let assign20050_e20418: f64 = (2.0 * var_tmp);
        let assign20050_e20420: f64 = (assign20050_e20418 - var_erfcpos);
        (assign20050_e20420, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign20050_e20422;
        var_erfctimesexpmtat_dn5 = assign20050_e20422_d_n5;
        var_erfctimesexpmtat_dn6 = assign20050_e20422_d_n6;
        var_erfctimesexpmtat_dn7 = assign20050_e20422_d_n7;
        var_erfctimesexpmtat_dn8 = assign20050_e20422_d_n8;

        let (assign20060_e20442, assign20060_e20442_d_n5, assign20060_e20442_d_n6, assign20060_e20442_d_n7, assign20060_e20442_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) {
        let assign20060_e20434: f64 = (1.772453850905516 * 0.5);
        let assign20060_e20437: f64 = (var_atatgat * var_erfctimesexpmtat);
        let assign20060_e20439: f64 = (assign20060_e20437 / var_ktat);
        let assign20060_e20440: f64 = (assign20060_e20434 * assign20060_e20439);
        (assign20060_e20440, (assign20060_e20434 * ((((var_atatgat * var_erfctimesexpmtat_dn5) * var_ktat) - (assign20060_e20437 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign20060_e20434 * ((((var_atatgat * var_erfctimesexpmtat_dn6) * var_ktat) - (assign20060_e20437 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign20060_e20434 * ((((var_atatgat * var_erfctimesexpmtat_dn7) * var_ktat) - (assign20060_e20437 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign20060_e20434 * ((((var_atatgat * var_erfctimesexpmtat_dn8) * var_ktat) - (assign20060_e20437 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign20060_e20442;
        var_gammamax_dn5 = assign20060_e20442_d_n5;
        var_gammamax_dn6 = assign20060_e20442_d_n6;
        var_gammamax_dn7 = assign20060_e20442_d_n7;
        var_gammamax_dn8 = assign20060_e20442_d_n8;

        let (assign20070_e20460, assign20070_e20460_d_n5, assign20070_e20460_d_n6, assign20070_e20460_d_n7, assign20070_e20460_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard347 == 0.0)) {
        let assign20070_e20455: f64 = (var_asrh * var_gammamax);
        let assign20070_e20457: f64 = (assign20070_e20455 * var_wtat);
        let assign20070_e20458: f64 = (p.p847 * assign20070_e20457);
        (assign20070_e20458, (p.p847 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign20070_e20455 * var_wtat_dn5))), (p.p847 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign20070_e20455 * var_wtat_dn6))), (p.p847 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign20070_e20455 * var_wtat_dn7))), (p.p847 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign20070_e20455 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign20070_e20460;
        var_itat_dn5 = assign20070_e20460_d_n5;
        var_itat_dn6 = assign20070_e20460_d_n6;
        var_itat_dn7 = assign20070_e20460_d_n7;
        var_itat_dn8 = assign20070_e20460_d_n8;

        let assign20080_e20463: f64 = if p.p853 == 0.0 { 1.0 } else { 0.0 };
        var_guard353 = assign20080_e20463;

        let (assign20090_e20474, assign20090_e20474_d_n5, assign20090_e20474_d_n6, assign20090_e20474_d_n7, assign20090_e20474_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard353 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign20090_e20474;
        var_ibbt_dn5 = assign20090_e20474_d_n5;
        var_ibbt_dn6 = assign20090_e20474_d_n6;
        var_ibbt_dn7 = assign20090_e20474_d_n7;
        var_ibbt_dn8 = assign20090_e20474_d_n8;

        let assign20100_e20477: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        var_guard354 = assign20100_e20477;

        let (assign20110_e20496, assign20110_e20496_d_n5, assign20110_e20496_d_n6, assign20110_e20496_d_n7, assign20110_e20496_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard353 == 0.0)) && (var_guard354 != 0.0)) {
        let assign20110_e20491: f64 = (p.p830 - var_vbbt);
        let assign20110_e20493: f64 = (assign20110_e20491 * var_vbirgatinv);
        let assign20110_e20494: f64 = (assign20110_e20493).sqrt();
        (assign20110_e20494, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20110_e20496;
        var_tmp_dn5 = assign20110_e20496_d_n5;
        var_tmp_dn6 = assign20110_e20496_d_n6;
        var_tmp_dn7 = assign20110_e20496_d_n7;
        var_tmp_dn8 = assign20110_e20496_d_n8;

        let (assign20120_e20517, assign20120_e20517_d_n5, assign20120_e20517_d_n6, assign20120_e20517_d_n7, assign20120_e20517_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard353 == 0.0)) && (var_guard354 == 0.0)) {
        let assign20120_e20511: f64 = (p.p830 - var_vbbt);
        let assign20120_e20513: f64 = (assign20120_e20511 * var_vbirgatinv);
        let assign20120_e20515: f64 = (assign20120_e20513).powf(p.p833);
        (assign20120_e20515, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20120_e20517;
        var_tmp_dn5 = assign20120_e20517_d_n5;
        var_tmp_dn6 = assign20120_e20517_d_n6;
        var_tmp_dn7 = assign20120_e20517_d_n7;
        var_tmp_dn8 = assign20120_e20517_d_n8;

        let (assign20130_e20537, assign20130_e20537_d_n5, assign20130_e20537_d_n6, assign20130_e20537_d_n7, assign20130_e20537_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard353 == 0.0)) {
        let assign20130_e20530: f64 = (p.p830 - var_vbbt);
        let assign20130_e20532: f64 = (assign20130_e20530 * var_wdepnulrinvgat);
        let assign20130_e20534: f64 = (assign20130_e20532 / var_tmp);
        let assign20130_e20535: f64 = (var_one_over_one_minus_pgat * assign20130_e20534);
        (assign20130_e20535, (var_one_over_one_minus_pgat * (-((assign20130_e20532 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign20130_e20532 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign20130_e20532 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign20130_e20532 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign20130_e20537;
        var_fmaxr_dn5 = assign20130_e20537_d_n5;
        var_fmaxr_dn6 = assign20130_e20537_d_n6;
        var_fmaxr_dn7 = assign20130_e20537_d_n7;
        var_fmaxr_dn8 = assign20130_e20537_d_n8;

        let assign20140_e20539: f64 = (-var_fbbtgat);
        let assign20140_e20541: f64 = (assign20140_e20539 / var_fmaxr);
        let assign20140_e20542: f64 = (assign20140_e20541).abs();
        let assign20140_e20544: f64 = if assign20140_e20542 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard355 = assign20140_e20544;

        let (assign20150_e20562, assign20150_e20562_d_n5, assign20150_e20562_d_n6, assign20150_e20562_d_n7, assign20150_e20562_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard353 == 0.0)) && (var_guard355 != 0.0)) {
        let assign20150_e20557: f64 = (-var_fbbtgat);
        let assign20150_e20559: f64 = (assign20150_e20557 / var_fmaxr);
        let assign20150_e20560: f64 = (assign20150_e20559).exp();
        (assign20150_e20560, (assign20150_e20560 * ((((-var_fbbtgat_dn5) * var_fmaxr) - (assign20150_e20557 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))), (assign20150_e20560 * ((((-var_fbbtgat_dn6) * var_fmaxr) - (assign20150_e20557 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign20150_e20560 * ((((-var_fbbtgat_dn7) * var_fmaxr) - (assign20150_e20557 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign20150_e20560 * ((((-var_fbbtgat_dn8) * var_fmaxr) - (assign20150_e20557 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20150_e20562;
        var_tmp_dn5 = assign20150_e20562_d_n5;
        var_tmp_dn6 = assign20150_e20562_d_n6;
        var_tmp_dn7 = assign20150_e20562_d_n7;
        var_tmp_dn8 = assign20150_e20562_d_n8;

        let assign20160_e20564: f64 = (-var_fbbtgat);
        let assign20160_e20566: f64 = (assign20160_e20564 / var_fmaxr);
        let assign20160_e20568: f64 = if assign20160_e20566 < 0.0 { 1.0 } else { 0.0 };
        var_guard356 = assign20160_e20568;

        let (assign20170_e20619, assign20170_e20619_d_n5, assign20170_e20619_d_n6, assign20170_e20619_d_n7, assign20170_e20619_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard353 == 0.0)) && (var_guard355 == 0.0)) && (var_guard356 != 0.0)) {
        let assign20170_e20586: f64 = (-230.25850929940458);
        let assign20170_e20588: f64 = (-var_fbbtgat);
        let assign20170_e20590: f64 = (assign20170_e20588 / var_fmaxr);
        let assign20170_e20591: f64 = (assign20170_e20586 - assign20170_e20590);
        let assign20170_e20595: f64 = (-230.25850929940458);
        let assign20170_e20597: f64 = (-var_fbbtgat);
        let assign20170_e20599: f64 = (assign20170_e20597 / var_fmaxr);
        let assign20170_e20600: f64 = (assign20170_e20595 - assign20170_e20599);
        let assign20170_e20603: f64 = (-230.25850929940458);
        let assign20170_e20605: f64 = (-var_fbbtgat);
        let assign20170_e20607: f64 = (assign20170_e20605 / var_fmaxr);
        let assign20170_e20608: f64 = (assign20170_e20603 - assign20170_e20607);
        let assign20170_e20610: f64 = (assign20170_e20608 * 0.3333333333333333);
        let assign20170_e20611: f64 = (1.0 + assign20170_e20610);
        let assign20170_e20612: f64 = (assign20170_e20600 * assign20170_e20611);
        let assign20170_e20613: f64 = (0.5 * assign20170_e20612);
        let assign20170_e20614: f64 = (1.0 + assign20170_e20613);
        let assign20170_e20615: f64 = (assign20170_e20591 * assign20170_e20614);
        let assign20170_e20616: f64 = (1.0 + assign20170_e20615);
        let assign20170_e20617: f64 = (1e-100 / assign20170_e20616);
        (assign20170_e20617, (-((1e-100 * (((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign20170_e20588 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign20170_e20614) + (assign20170_e20591 * (0.5 * (((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign20170_e20597 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign20170_e20611) + (assign20170_e20600 * ((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign20170_e20605 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign20170_e20616 * assign20170_e20616))), (-((1e-100 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign20170_e20588 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign20170_e20614) + (assign20170_e20591 * (0.5 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign20170_e20597 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign20170_e20611) + (assign20170_e20600 * ((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign20170_e20605 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign20170_e20616 * assign20170_e20616))), (-((1e-100 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign20170_e20588 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign20170_e20614) + (assign20170_e20591 * (0.5 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign20170_e20597 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign20170_e20611) + (assign20170_e20600 * ((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign20170_e20605 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign20170_e20616 * assign20170_e20616))), (-((1e-100 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign20170_e20588 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign20170_e20614) + (assign20170_e20591 * (0.5 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign20170_e20597 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign20170_e20611) + (assign20170_e20600 * ((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign20170_e20605 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign20170_e20616 * assign20170_e20616))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20170_e20619;
        var_tmp_dn5 = assign20170_e20619_d_n5;
        var_tmp_dn6 = assign20170_e20619_d_n6;
        var_tmp_dn7 = assign20170_e20619_d_n7;
        var_tmp_dn8 = assign20170_e20619_d_n8;

        let (assign20180_e20668, assign20180_e20668_d_n5, assign20180_e20668_d_n6, assign20180_e20668_d_n7, assign20180_e20668_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard353 == 0.0)) && (var_guard355 == 0.0)) && (var_guard356 == 0.0)) {
        let assign20180_e20638: f64 = (-var_fbbtgat);
        let assign20180_e20640: f64 = (assign20180_e20638 / var_fmaxr);
        let assign20180_e20642: f64 = (assign20180_e20640 - 230.25850929940458);
        let assign20180_e20646: f64 = (-var_fbbtgat);
        let assign20180_e20648: f64 = (assign20180_e20646 / var_fmaxr);
        let assign20180_e20650: f64 = (assign20180_e20648 - 230.25850929940458);
        let assign20180_e20653: f64 = (-var_fbbtgat);
        let assign20180_e20655: f64 = (assign20180_e20653 / var_fmaxr);
        let assign20180_e20657: f64 = (assign20180_e20655 - 230.25850929940458);
        let assign20180_e20659: f64 = (assign20180_e20657 * 0.3333333333333333);
        let assign20180_e20660: f64 = (1.0 + assign20180_e20659);
        let assign20180_e20661: f64 = (assign20180_e20650 * assign20180_e20660);
        let assign20180_e20662: f64 = (0.5 * assign20180_e20661);
        let assign20180_e20663: f64 = (1.0 + assign20180_e20662);
        let assign20180_e20664: f64 = (assign20180_e20642 * assign20180_e20663);
        let assign20180_e20665: f64 = (1.0 + assign20180_e20664);
        let assign20180_e20666: f64 = (1e100 * assign20180_e20665);
        (assign20180_e20666, (1e100 * ((((((-var_fbbtgat_dn5) * var_fmaxr) - (assign20180_e20638 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign20180_e20663) + (assign20180_e20642 * (0.5 * ((((((-var_fbbtgat_dn5) * var_fmaxr) - (assign20180_e20646 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign20180_e20660) + (assign20180_e20650 * (((((-var_fbbtgat_dn5) * var_fmaxr) - (assign20180_e20653 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign20180_e20638 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign20180_e20663) + (assign20180_e20642 * (0.5 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign20180_e20646 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign20180_e20660) + (assign20180_e20650 * (((((-var_fbbtgat_dn6) * var_fmaxr) - (assign20180_e20653 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign20180_e20638 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign20180_e20663) + (assign20180_e20642 * (0.5 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign20180_e20646 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign20180_e20660) + (assign20180_e20650 * (((((-var_fbbtgat_dn7) * var_fmaxr) - (assign20180_e20653 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign20180_e20638 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign20180_e20663) + (assign20180_e20642 * (0.5 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign20180_e20646 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign20180_e20660) + (assign20180_e20650 * (((((-var_fbbtgat_dn8) * var_fmaxr) - (assign20180_e20653 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20180_e20668;
        var_tmp_dn5 = assign20180_e20668_d_n5;
        var_tmp_dn6 = assign20180_e20668_d_n6;
        var_tmp_dn7 = assign20180_e20668_d_n7;
        var_tmp_dn8 = assign20180_e20668_d_n8;

        let (assign20190_e20688, assign20190_e20688_d_n5, assign20190_e20688_d_n6, assign20190_e20688_d_n7, assign20190_e20688_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard353 == 0.0)) {
        let assign20190_e20681: f64 = (var_v2 * var_fmaxr);
        let assign20190_e20683: f64 = (assign20190_e20681 * var_fmaxr);
        let assign20190_e20685: f64 = (assign20190_e20683 * var_tmp);
        let assign20190_e20686: f64 = (p.p853 * assign20190_e20685);
        (assign20190_e20686, (p.p853 * (((((var_v2 * var_fmaxr_dn5) * var_fmaxr) + (assign20190_e20681 * var_fmaxr_dn5)) * var_tmp) + (assign20190_e20683 * var_tmp_dn5))), (p.p853 * (((((var_v2 * var_fmaxr_dn6) * var_fmaxr) + (assign20190_e20681 * var_fmaxr_dn6)) * var_tmp) + (assign20190_e20683 * var_tmp_dn6))), (p.p853 * (((((var_v2 * var_fmaxr_dn7) * var_fmaxr) + (assign20190_e20681 * var_fmaxr_dn7)) * var_tmp) + (assign20190_e20683 * var_tmp_dn7))), (p.p853 * (((((var_v2 * var_fmaxr_dn8) * var_fmaxr) + (assign20190_e20681 * var_fmaxr_dn8)) * var_tmp) + (assign20190_e20683 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign20190_e20688;
        var_ibbt_dn5 = assign20190_e20688_d_n5;
        var_ibbt_dn6 = assign20190_e20688_d_n6;
        var_ibbt_dn7 = assign20190_e20688_d_n7;
        var_ibbt_dn8 = assign20190_e20688_d_n8;

        let assign20200_e20691: f64 = if p.p862 > 1000.0 { 1.0 } else { 0.0 };
        var_guard357 = assign20200_e20691;

        let (assign20210_e20702, assign20210_e20702_d_n5, assign20210_e20702_d_n6, assign20210_e20702_d_n7, assign20210_e20702_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard357 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign20210_e20702;
        var_fbreakdown_dn5 = assign20210_e20702_d_n5;
        var_fbreakdown_dn6 = assign20210_e20702_d_n6;
        var_fbreakdown_dn7 = assign20210_e20702_d_n7;
        var_fbreakdown_dn8 = assign20210_e20702_d_n8;

        let assign20220_e20705: f64 = (-var_alphaav);
        let assign20220_e20707: f64 = (assign20220_e20705 * p.p862);
        let assign20220_e20708: f64 = if var_vav > assign20220_e20707 { 1.0 } else { 0.0 };
        var_guard358 = assign20220_e20708;

        let assign20230_e20711: f64 = if p.p865 == 4.0 { 1.0 } else { 0.0 };
        var_guard359 = assign20230_e20711;

        let (assign20240_e20741, assign20240_e20741_d_n5, assign20240_e20741_d_n6, assign20240_e20741_d_n7, assign20240_e20741_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard357 == 0.0)) && (var_guard358 != 0.0)) && (var_guard359 != 0.0)) {
        let assign20240_e20727: f64 = (var_vav * var_vbrinvgat);
        let assign20240_e20730: f64 = (var_vav * var_vbrinvgat);
        let assign20240_e20731: f64 = (assign20240_e20727 * assign20240_e20730);
        let assign20240_e20734: f64 = (var_vav * var_vbrinvgat);
        let assign20240_e20735: f64 = (assign20240_e20731 * assign20240_e20734);
        let assign20240_e20738: f64 = (var_vav * var_vbrinvgat);
        let assign20240_e20739: f64 = (assign20240_e20735 * assign20240_e20738);
        (assign20240_e20739, (((((((var_vav * var_vbrinvgat_dn5) * assign20240_e20730) + (assign20240_e20727 * (var_vav * var_vbrinvgat_dn5))) * assign20240_e20734) + (assign20240_e20731 * (var_vav * var_vbrinvgat_dn5))) * assign20240_e20738) + (assign20240_e20735 * (var_vav * var_vbrinvgat_dn5))), (((((((var_vav * var_vbrinvgat_dn6) * assign20240_e20730) + (assign20240_e20727 * (var_vav * var_vbrinvgat_dn6))) * assign20240_e20734) + (assign20240_e20731 * (var_vav * var_vbrinvgat_dn6))) * assign20240_e20738) + (assign20240_e20735 * (var_vav * var_vbrinvgat_dn6))), (((((((var_vav * var_vbrinvgat_dn7) * assign20240_e20730) + (assign20240_e20727 * (var_vav * var_vbrinvgat_dn7))) * assign20240_e20734) + (assign20240_e20731 * (var_vav * var_vbrinvgat_dn7))) * assign20240_e20738) + (assign20240_e20735 * (var_vav * var_vbrinvgat_dn7))), (((((((var_vav * var_vbrinvgat_dn8) * assign20240_e20730) + (assign20240_e20727 * (var_vav * var_vbrinvgat_dn8))) * assign20240_e20734) + (assign20240_e20731 * (var_vav * var_vbrinvgat_dn8))) * assign20240_e20738) + (assign20240_e20735 * (var_vav * var_vbrinvgat_dn8))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20240_e20741;
        var_tmp_dn5 = assign20240_e20741_d_n5;
        var_tmp_dn6 = assign20240_e20741_d_n6;
        var_tmp_dn7 = assign20240_e20741_d_n7;
        var_tmp_dn8 = assign20240_e20741_d_n8;

        let (assign20250_e20763, assign20250_e20763_d_n5, assign20250_e20763_d_n6, assign20250_e20763_d_n7, assign20250_e20763_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard357 == 0.0)) && (var_guard358 != 0.0)) && (var_guard359 == 0.0)) {
        let assign20250_e20758: f64 = (var_vav * var_vbrinvgat);
        let assign20250_e20759: f64 = (assign20250_e20758).abs();
        let assign20250_e20761: f64 = (assign20250_e20759).powf(p.p865);
        (assign20250_e20761, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign20250_e20759).powf(p.p865 - 1.0) * if assign20250_e20758 >= 0.0 { (var_vav * var_vbrinvgat_dn5) } else { (-(var_vav * var_vbrinvgat_dn5)) })) } } else { (assign20250_e20761 * (p.p865 * (if assign20250_e20758 >= 0.0 { (var_vav * var_vbrinvgat_dn5) } else { (-(var_vav * var_vbrinvgat_dn5)) } / assign20250_e20759))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign20250_e20759).powf(p.p865 - 1.0) * if assign20250_e20758 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) })) } } else { (assign20250_e20761 * (p.p865 * (if assign20250_e20758 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) } / assign20250_e20759))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign20250_e20759).powf(p.p865 - 1.0) * if assign20250_e20758 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) })) } } else { (assign20250_e20761 * (p.p865 * (if assign20250_e20758 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) } / assign20250_e20759))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign20250_e20759).powf(p.p865 - 1.0) * if assign20250_e20758 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) })) } } else { (assign20250_e20761 * (p.p865 * (if assign20250_e20758 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) } / assign20250_e20759))) },)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20250_e20763;
        var_tmp_dn5 = assign20250_e20763_d_n5;
        var_tmp_dn6 = assign20250_e20763_d_n6;
        var_tmp_dn7 = assign20250_e20763_d_n7;
        var_tmp_dn8 = assign20250_e20763_d_n8;

        let (assign20260_e20781, assign20260_e20781_d_n5, assign20260_e20781_d_n6, assign20260_e20781_d_n7, assign20260_e20781_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard357 == 0.0)) && (var_guard358 != 0.0)) {
        let assign20260_e20778: f64 = (1.0 - var_tmp);
        let assign20260_e20779: f64 = (1.0 / assign20260_e20778);
        (assign20260_e20779, (-((-var_tmp_dn5) / (assign20260_e20778 * assign20260_e20778))), (-((-var_tmp_dn6) / (assign20260_e20778 * assign20260_e20778))), (-((-var_tmp_dn7) / (assign20260_e20778 * assign20260_e20778))), (-((-var_tmp_dn8) / (assign20260_e20778 * assign20260_e20778))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign20260_e20781;
        var_fbreakdown_dn5 = assign20260_e20781_d_n5;
        var_fbreakdown_dn6 = assign20260_e20781_d_n6;
        var_fbreakdown_dn7 = assign20260_e20781_d_n7;
        var_fbreakdown_dn8 = assign20260_e20781_d_n8;

        let (assign20270_e20804, assign20270_e20804_d_n5, assign20270_e20804_d_n6, assign20270_e20804_d_n7, assign20270_e20804_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) && (var_guard357 == 0.0)) && (var_guard358 == 0.0)) {
        let assign20270_e20798: f64 = (var_alphaav * p.p862);
        let assign20270_e20799: f64 = (var_vav + assign20270_e20798);
        let assign20270_e20801: f64 = (assign20270_e20799 * var_slopegat);
        let assign20270_e20802: f64 = (var_fstopgat + assign20270_e20801);
        (assign20270_e20802, (assign20270_e20799 * var_slopegat_dn5), (assign20270_e20799 * var_slopegat_dn6), (assign20270_e20799 * var_slopegat_dn7), (assign20270_e20799 * var_slopegat_dn8),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign20270_e20804;
        var_fbreakdown_dn5 = assign20270_e20804_d_n5;
        var_fbreakdown_dn6 = assign20270_e20804_d_n6;
        var_fbreakdown_dn7 = assign20270_e20804_d_n7;
        var_fbreakdown_dn8 = assign20270_e20804_d_n8;

        let (assign20280_e20823, assign20280_e20823_d_n5, assign20280_e20823_d_n6, assign20280_e20823_d_n7, assign20280_e20823_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard343 == 0.0)) {
        let assign20280_e20814: f64 = (var_id__blk219 + var_isrh);
        let assign20280_e20816: f64 = (assign20280_e20814 + var_itat);
        let assign20280_e20818: f64 = (assign20280_e20816 + var_ibbt);
        let assign20280_e20819: f64 = (p.p29 * assign20280_e20818);
        let assign20280_e20821: f64 = (assign20280_e20819 * var_fbreakdown);
        (assign20280_e20821, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign20280_e20819 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign20280_e20819 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign20280_e20819 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign20280_e20819 * var_fbreakdown_dn8)),)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign20280_e20823;
        var_ijungat_dn5 = assign20280_e20823_d_n5;
        var_ijungat_dn6 = assign20280_e20823_d_n6;
        var_ijungat_dn7 = assign20280_e20823_d_n7;
        var_ijungat_dn8 = assign20280_e20823_d_n8;

        let (assign20290_e20839, assign20290_e20839_d_n5, assign20290_e20839_d_n6, assign20290_e20839_d_n7, assign20290_e20839_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign20290_e20829: f64 = (var_absource_i * var_ijunbot);
        let assign20290_e20832: f64 = (var_lssource_i * var_ijunsti);
        let assign20290_e20833: f64 = (assign20290_e20829 + assign20290_e20832);
        let assign20290_e20836: f64 = (var_lgsource_i * var_ijungat);
        let assign20290_e20837: f64 = (assign20290_e20833 + assign20290_e20836);
        (assign20290_e20837, (((var_absource_i * var_ijunbot_dn5) + (var_lssource_i * var_ijunsti_dn5)) + (var_lgsource_i * var_ijungat_dn5)), (((var_absource_i * var_ijunbot_dn6) + (var_lssource_i * var_ijunsti_dn6)) + (var_lgsource_i * var_ijungat_dn6)), (((var_absource_i * var_ijunbot_dn7) + (var_lssource_i * var_ijunsti_dn7)) + (var_lgsource_i * var_ijungat_dn7)), (((var_absource_i * var_ijunbot_dn8) + (var_lssource_i * var_ijunsti_dn8)) + (var_lgsource_i * var_ijungat_dn8)),)
    } else {
        (var_i2, var_i2_dn5, var_i2_dn6, var_i2_dn7, var_i2_dn8,)
    }
};
        var_i2 = assign20290_e20839;
        var_i2_dn5 = assign20290_e20839_d_n5;
        var_i2_dn6 = assign20290_e20839_d_n6;
        var_i2_dn7 = assign20290_e20839_d_n7;
        var_i2_dn8 = assign20290_e20839_d_n8;

        let (assign20300_e20845,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign20300_e20845;

        let (assign20310_e20851,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign20310_e20851;

        let assign20320_e20863: f64 = if (!(((var_absource_i == 0.0) && (var_lssource_i == 0.0)) && (var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard360 = assign20320_e20863;

        let assign20400_e20949: f64 = if var_v3 < var_vmax_s { 1.0 } else { 0.0 };
        var_guard361 = assign20400_e20949;

        let assign20410_e20951: f64 = (-0.5);
        let assign20410_e20954: f64 = (var_v3 * var_phitdinv);
        let assign20410_e20955: f64 = (assign20410_e20951 * assign20410_e20954);
        let assign20410_e20956: f64 = (assign20410_e20955).abs();
        let assign20410_e20958: f64 = if assign20410_e20956 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard362 = assign20410_e20958;

        let (assign20420_e20976,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard360 != 0.0)) && (var_guard361 != 0.0)) && (var_guard362 != 0.0)) {
        let assign20420_e20969: f64 = (-0.5);
        let assign20420_e20972: f64 = (var_v3 * var_phitdinv);
        let assign20420_e20973: f64 = (assign20420_e20969 * assign20420_e20972);
        let assign20420_e20974: f64 = (assign20420_e20973).exp();
        (assign20420_e20974,)
    } else {
        (var_z,)
    }
};
        var_z = assign20420_e20976;

        let assign20430_e20978: f64 = (-0.5);
        let assign20430_e20981: f64 = (var_v3 * var_phitdinv);
        let assign20430_e20982: f64 = (assign20430_e20978 * assign20430_e20981);
        let assign20430_e20984: f64 = if assign20430_e20982 < 0.0 { 1.0 } else { 0.0 };
        var_guard363 = assign20430_e20984;

        let (assign20440_e21039,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard360 != 0.0)) && (var_guard361 != 0.0)) && (var_guard362 == 0.0)) && (var_guard363 != 0.0)) {
        let assign20440_e21000: f64 = (-230.25850929940458);
        let assign20440_e21002: f64 = (-0.5);
        let assign20440_e21005: f64 = (var_v3 * var_phitdinv);
        let assign20440_e21006: f64 = (assign20440_e21002 * assign20440_e21005);
        let assign20440_e21007: f64 = (assign20440_e21000 - assign20440_e21006);
        let assign20440_e21011: f64 = (-230.25850929940458);
        let assign20440_e21013: f64 = (-0.5);
        let assign20440_e21016: f64 = (var_v3 * var_phitdinv);
        let assign20440_e21017: f64 = (assign20440_e21013 * assign20440_e21016);
        let assign20440_e21018: f64 = (assign20440_e21011 - assign20440_e21017);
        let assign20440_e21021: f64 = (-230.25850929940458);
        let assign20440_e21023: f64 = (-0.5);
        let assign20440_e21026: f64 = (var_v3 * var_phitdinv);
        let assign20440_e21027: f64 = (assign20440_e21023 * assign20440_e21026);
        let assign20440_e21028: f64 = (assign20440_e21021 - assign20440_e21027);
        let assign20440_e21030: f64 = (assign20440_e21028 * 0.3333333333333333);
        let assign20440_e21031: f64 = (1.0 + assign20440_e21030);
        let assign20440_e21032: f64 = (assign20440_e21018 * assign20440_e21031);
        let assign20440_e21033: f64 = (0.5 * assign20440_e21032);
        let assign20440_e21034: f64 = (1.0 + assign20440_e21033);
        let assign20440_e21035: f64 = (assign20440_e21007 * assign20440_e21034);
        let assign20440_e21036: f64 = (1.0 + assign20440_e21035);
        let assign20440_e21037: f64 = (1e-100 / assign20440_e21036);
        (assign20440_e21037,)
    } else {
        (var_z,)
    }
};
        var_z = assign20440_e21039;

        let (assign20450_e21092,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard360 != 0.0)) && (var_guard361 != 0.0)) && (var_guard362 == 0.0)) && (var_guard363 == 0.0)) {
        let assign20450_e21056: f64 = (-0.5);
        let assign20450_e21059: f64 = (var_v3 * var_phitdinv);
        let assign20450_e21060: f64 = (assign20450_e21056 * assign20450_e21059);
        let assign20450_e21062: f64 = (assign20450_e21060 - 230.25850929940458);
        let assign20450_e21066: f64 = (-0.5);
        let assign20450_e21069: f64 = (var_v3 * var_phitdinv);
        let assign20450_e21070: f64 = (assign20450_e21066 * assign20450_e21069);
        let assign20450_e21072: f64 = (assign20450_e21070 - 230.25850929940458);
        let assign20450_e21075: f64 = (-0.5);
        let assign20450_e21078: f64 = (var_v3 * var_phitdinv);
        let assign20450_e21079: f64 = (assign20450_e21075 * assign20450_e21078);
        let assign20450_e21081: f64 = (assign20450_e21079 - 230.25850929940458);
        let assign20450_e21083: f64 = (assign20450_e21081 * 0.3333333333333333);
        let assign20450_e21084: f64 = (1.0 + assign20450_e21083);
        let assign20450_e21085: f64 = (assign20450_e21072 * assign20450_e21084);
        let assign20450_e21086: f64 = (0.5 * assign20450_e21085);
        let assign20450_e21087: f64 = (1.0 + assign20450_e21086);
        let assign20450_e21088: f64 = (assign20450_e21062 * assign20450_e21087);
        let assign20450_e21089: f64 = (1.0 + assign20450_e21088);
        let assign20450_e21090: f64 = (1e100 * assign20450_e21089);
        (assign20450_e21090,)
    } else {
        (var_z,)
    }
};
        var_z = assign20450_e21092;

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
        *var_guard353_slot = var_guard353;
        *var_guard354_slot = var_guard354;
        *var_guard355_slot = var_guard355;
        *var_guard356_slot = var_guard356;
        *var_guard357_slot = var_guard357;
        *var_guard358_slot = var_guard358;
        *var_guard359_slot = var_guard359;
        *var_guard360_slot = var_guard360;
        *var_guard361_slot = var_guard361;
        *var_guard362_slot = var_guard362;
        *var_guard363_slot = var_guard363;
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

    pub(super) fn stamp_transient_block_35(
        p: &Parameters,
        var_absource_i: f64,
        var_atatbot: f64,
        var_btatpartbot: f64,
        var_exp_vmax_over_phitd_s: f64,
        var_ftdbot: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard360: f64,
        var_guard361: f64,
        var_idsatbot: f64,
        var_one_minus_pbot: f64,
        var_one_over_one_minus_pbot: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitr: f64,
        var_v3: f64,
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
        var_guard364_slot: &mut f64,
        var_guard365_slot: &mut f64,
        var_guard366_slot: &mut f64,
        var_guard367_slot: &mut f64,
        var_guard368_slot: &mut f64,
        var_guard369_slot: &mut f64,
        var_guard370_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
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
        let mut var_guard364: f64 = *var_guard364_slot;
        let mut var_guard365: f64 = *var_guard365_slot;
        let mut var_guard366: f64 = *var_guard366_slot;
        let mut var_guard367: f64 = *var_guard367_slot;
        let mut var_guard368: f64 = *var_guard368_slot;
        let mut var_guard369: f64 = *var_guard369_slot;
        let mut var_guard370: f64 = *var_guard370_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
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

        let (assign20460_e21104,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard360 != 0.0)) && (var_guard361 != 0.0)) {
        let assign20460_e21102: f64 = (1.0 / var_z);
        (assign20460_e21102,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign20460_e21104;

        let (assign20470_e21116,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard360 != 0.0)) && (var_guard361 != 0.0)) {
        let assign20470_e21114: f64 = (var_zinv * var_zinv);
        (assign20470_e21114,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign20470_e21116;

        let (assign20480_e21135,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard360 != 0.0)) && (var_guard361 == 0.0)) {
        let assign20480_e21128: f64 = (var_v3 - var_vmax_s);
        let assign20480_e21130: f64 = (assign20480_e21128 * var_phitdinv);
        let assign20480_e21131: f64 = (1.0 + assign20480_e21130);
        let assign20480_e21133: f64 = (assign20480_e21131 * var_exp_vmax_over_phitd_s);
        (assign20480_e21133,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign20480_e21135;

        let (assign20490_e21147,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard360 != 0.0)) && (var_guard361 == 0.0)) {
        let assign20490_e21145: f64 = (var_idmult).sqrt();
        (assign20490_e21145,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign20490_e21147;

        let (assign20500_e21160,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard360 != 0.0)) && (var_guard361 == 0.0)) {
        let assign20500_e21158: f64 = (1.0 / var_zinv);
        (assign20500_e21158,)
    } else {
        (var_z,)
    }
};
        var_z = assign20500_e21160;

        let (assign20510_e21170,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard360 != 0.0)) {
        let assign20510_e21168: f64 = (var_idmult - 1.0);
        (assign20510_e21168,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign20510_e21170;

        let assign20520_e21173: f64 = if var_v3 > 0.0 { 1.0 } else { 0.0 };
        var_guard364 = assign20520_e21173;

        let (assign20530_e21199,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard360 != 0.0)) && (var_guard364 != 0.0)) {
        let assign20530_e21185: f64 = (2.0 + var_z);
        let assign20530_e21188: f64 = (var_z + 1.0);
        let assign20530_e21191: f64 = (var_z + 3.0);
        let assign20530_e21192: f64 = (assign20530_e21188 * assign20530_e21191);
        let assign20530_e21193: f64 = (assign20530_e21192).sqrt();
        let assign20530_e21194: f64 = (assign20530_e21185 + assign20530_e21193);
        let assign20530_e21195: f64 = (assign20530_e21194).ln();
        let assign20530_e21196: f64 = (var_phitd * assign20530_e21195);
        let assign20530_e21197: f64 = (2.0 * assign20530_e21196);
        (assign20530_e21197,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign20530_e21199;

        let (assign20540_e21233,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard360 != 0.0)) && (var_guard364 == 0.0)) {
        let assign20540_e21209: f64 = (-var_v3);
        let assign20540_e21214: f64 = (2.0 * var_zinv);
        let assign20540_e21216: f64 = (assign20540_e21214 + 1.0);
        let assign20540_e21219: f64 = (1.0 + var_zinv);
        let assign20540_e21223: f64 = (3.0 * var_zinv);
        let assign20540_e21224: f64 = (1.0 + assign20540_e21223);
        let assign20540_e21225: f64 = (assign20540_e21219 * assign20540_e21224);
        let assign20540_e21226: f64 = (assign20540_e21225).sqrt();
        let assign20540_e21227: f64 = (assign20540_e21216 + assign20540_e21226);
        let assign20540_e21228: f64 = (assign20540_e21227).ln();
        let assign20540_e21229: f64 = (var_phitd * assign20540_e21228);
        let assign20540_e21230: f64 = (2.0 * assign20540_e21229);
        let assign20540_e21231: f64 = (assign20540_e21209 + assign20540_e21230);
        (assign20540_e21231,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign20540_e21233;

        let (assign20550_e21243,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard360 != 0.0)) {
        let assign20550_e21241: f64 = (var_vbimin_s - var_two_psistar);
        (assign20550_e21241,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign20550_e21243;

        let (assign20560_e21270,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard360 != 0.0)) {
        let assign20560_e21252: f64 = (var_v3 + var_vjlim);
        let assign20560_e21255: f64 = (var_v3 - var_vjlim);
        let assign20560_e21258: f64 = (var_v3 - var_vjlim);
        let assign20560_e21259: f64 = (assign20560_e21255 * assign20560_e21258);
        let assign20560_e21262: f64 = (4.0 * var_phitd);
        let assign20560_e21264: f64 = (assign20560_e21262 * var_phitd);
        let assign20560_e21265: f64 = (assign20560_e21259 + assign20560_e21264);
        let assign20560_e21266: f64 = (assign20560_e21265).sqrt();
        let assign20560_e21267: f64 = (assign20560_e21252 - assign20560_e21266);
        let assign20560_e21268: f64 = (0.5 * assign20560_e21267);
        (assign20560_e21268,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign20560_e21270;

        let (assign20570_e21297,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard360 != 0.0)) {
        let assign20570_e21279: f64 = (var_v3 + var_vbbtlim_s);
        let assign20570_e21282: f64 = (var_v3 - var_vbbtlim_s);
        let assign20570_e21285: f64 = (var_v3 - var_vbbtlim_s);
        let assign20570_e21286: f64 = (assign20570_e21282 * assign20570_e21285);
        let assign20570_e21289: f64 = (4.0 * var_phitr);
        let assign20570_e21291: f64 = (assign20570_e21289 * var_phitr);
        let assign20570_e21292: f64 = (assign20570_e21286 + assign20570_e21291);
        let assign20570_e21293: f64 = (assign20570_e21292).sqrt();
        let assign20570_e21294: f64 = (assign20570_e21279 - assign20570_e21293);
        let assign20570_e21295: f64 = (0.5 * assign20570_e21294);
        (assign20570_e21295,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign20570_e21297;

        let (assign20580_e21324,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard360 != 0.0)) {
        let assign20580_e21306: f64 = var_v3;
        let assign20580_e21309: f64 = var_v3;
        let assign20580_e21312: f64 = var_v3;
        let assign20580_e21313: f64 = (assign20580_e21309 * assign20580_e21312);
        let assign20580_e21316: f64 = (4.0 * 1e-6);
        let assign20580_e21318: f64 = (assign20580_e21316 * 1e-6);
        let assign20580_e21319: f64 = (assign20580_e21313 + assign20580_e21318);
        let assign20580_e21320: f64 = (assign20580_e21319).sqrt();
        let assign20580_e21321: f64 = (assign20580_e21306 - assign20580_e21320);
        let assign20580_e21322: f64 = (0.5 * assign20580_e21321);
        (assign20580_e21322,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign20580_e21324;

        let assign20590_e21327: f64 = if var_absource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard365 = assign20590_e21327;

        let (assign20600_e21335, assign20600_e21335_d_n5, assign20600_e21335_d_n6, assign20600_e21335_d_n7, assign20600_e21335_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign20600_e21335;
        var_ijunbot_dn5 = assign20600_e21335_d_n5;
        var_ijunbot_dn6 = assign20600_e21335_d_n6;
        var_ijunbot_dn7 = assign20600_e21335_d_n7;
        var_ijunbot_dn8 = assign20600_e21335_d_n8;

        let (assign20610_e21346,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) {
        let assign20610_e21344: f64 = (var_idsatbot * var_idmult);
        (assign20610_e21344,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign20610_e21346;

        let assign20620_e21353: f64 = if ((p.p840 == 0.0) && (p.p845 == 0.0)) { 1.0 } else { 0.0 };
        var_guard366 = assign20620_e21353;

        let (assign20630_e21364, assign20630_e21364_d_n5, assign20630_e21364_d_n6, assign20630_e21364_d_n7, assign20630_e21364_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard366 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign20630_e21364;
        var_isrh_dn5 = assign20630_e21364_d_n5;
        var_isrh_dn6 = assign20630_e21364_d_n6;
        var_isrh_dn7 = assign20630_e21364_d_n7;
        var_isrh_dn8 = assign20630_e21364_d_n8;

        let (assign20640_e21378,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard366 == 0.0)) {
        let assign20640_e21376: f64 = (var_vbibot - var_vjsrh);
        (assign20640_e21376,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign20640_e21378;

        let (assign20650_e21397,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard366 == 0.0)) {
        let assign20650_e21392: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign20650_e21393: f64 = (1.0 - assign20650_e21392);
        let assign20650_e21394: f64 = (assign20650_e21393).sqrt();
        let assign20650_e21395: f64 = (1.0 - assign20650_e21394);
        (assign20650_e21395,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign20650_e21397;

        let assign20660_e21400: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        var_guard367 = assign20660_e21400;

        let (assign20670_e21414,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard366 == 0.0)) && (var_guard367 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign20670_e21414;

        let (assign20680_e21446,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard366 == 0.0)) && (var_guard367 == 0.0)) {
        let assign20680_e21429: f64 = (var_wsrhstep * var_wsrhstep);
        let assign20680_e21431: f64 = (var_wsrhstep).ln();
        let assign20680_e21432: f64 = (assign20680_e21429 * assign20680_e21431);
        let assign20680_e21435: f64 = (1.0 - var_wsrhstep);
        let assign20680_e21436: f64 = (assign20680_e21432 / assign20680_e21435);
        let assign20680_e21438: f64 = (assign20680_e21436 + var_wsrhstep);
        let assign20680_e21442: f64 = (2.0 * p.p831);
        let assign20680_e21443: f64 = (1.0 - assign20680_e21442);
        let assign20680_e21444: f64 = (assign20680_e21438 * assign20680_e21443);
        (assign20680_e21444,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign20680_e21446;

        let (assign20690_e21460,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard366 == 0.0)) {
        let assign20690_e21458: f64 = (var_wsrhstep + var_dwsrh);
        (assign20690_e21458,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign20690_e21460;

        let assign20700_e21463: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        var_guard368 = assign20700_e21463;

        let (assign20710_e21480, assign20710_e21480_d_n5, assign20710_e21480_d_n6, assign20710_e21480_d_n7, assign20710_e21480_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard366 == 0.0)) && (var_guard368 != 0.0)) {
        let assign20710_e21477: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign20710_e21478: f64 = (assign20710_e21477).sqrt();
        (assign20710_e21478, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20710_e21480;
        var_tmp_dn5 = assign20710_e21480_d_n5;
        var_tmp_dn6 = assign20710_e21480_d_n6;
        var_tmp_dn7 = assign20710_e21480_d_n7;
        var_tmp_dn8 = assign20710_e21480_d_n8;

        let (assign20720_e21499, assign20720_e21499_d_n5, assign20720_e21499_d_n6, assign20720_e21499_d_n7, assign20720_e21499_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard366 == 0.0)) && (var_guard368 == 0.0)) {
        let assign20720_e21495: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign20720_e21497: f64 = (assign20720_e21495).powf(p.p831);
        (assign20720_e21497, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20720_e21499;
        var_tmp_dn5 = assign20720_e21499_d_n5;
        var_tmp_dn6 = assign20720_e21499_d_n6;
        var_tmp_dn7 = assign20720_e21499_d_n7;
        var_tmp_dn8 = assign20720_e21499_d_n8;

        let (assign20730_e21513, assign20730_e21513_d_n5, assign20730_e21513_d_n6, assign20730_e21513_d_n7, assign20730_e21513_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard366 == 0.0)) {
        let assign20730_e21511: f64 = (var_wdepnulrbot * var_tmp);
        (assign20730_e21511, (var_wdepnulrbot * var_tmp_dn5), (var_wdepnulrbot * var_tmp_dn6), (var_wdepnulrbot * var_tmp_dn7), (var_wdepnulrbot * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign20730_e21513;
        var_wdep_dn5 = assign20730_e21513_d_n5;
        var_wdep_dn6 = assign20730_e21513_d_n6;
        var_wdep_dn7 = assign20730_e21513_d_n7;
        var_wdep_dn8 = assign20730_e21513_d_n8;

        let (assign20740_e21531, assign20740_e21531_d_n5, assign20740_e21531_d_n6, assign20740_e21531_d_n7, assign20740_e21531_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard366 == 0.0)) {
        let assign20740_e21526: f64 = (var_zinv - 1.0);
        let assign20740_e21528: f64 = (assign20740_e21526 * var_wdep);
        let assign20740_e21529: f64 = (var_ftdbot * assign20740_e21528);
        (assign20740_e21529, (var_ftdbot * (assign20740_e21526 * var_wdep_dn5)), (var_ftdbot * (assign20740_e21526 * var_wdep_dn6)), (var_ftdbot * (assign20740_e21526 * var_wdep_dn7)), (var_ftdbot * (assign20740_e21526 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign20740_e21531;
        var_asrh_dn5 = assign20740_e21531_d_n5;
        var_asrh_dn6 = assign20740_e21531_d_n6;
        var_asrh_dn7 = assign20740_e21531_d_n7;
        var_asrh_dn8 = assign20740_e21531_d_n8;

        let (assign20750_e21547, assign20750_e21547_d_n5, assign20750_e21547_d_n6, assign20750_e21547_d_n7, assign20750_e21547_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard366 == 0.0)) {
        let assign20750_e21544: f64 = (var_asrh * var_wsrh);
        let assign20750_e21545: f64 = (p.p840 * assign20750_e21544);
        (assign20750_e21545, (p.p840 * (var_asrh_dn5 * var_wsrh)), (p.p840 * (var_asrh_dn6 * var_wsrh)), (p.p840 * (var_asrh_dn7 * var_wsrh)), (p.p840 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign20750_e21547;
        var_isrh_dn5 = assign20750_e21547_d_n5;
        var_isrh_dn6 = assign20750_e21547_d_n6;
        var_isrh_dn7 = assign20750_e21547_d_n7;
        var_isrh_dn8 = assign20750_e21547_d_n8;

        let assign20760_e21550: f64 = if p.p845 == 0.0 { 1.0 } else { 0.0 };
        var_guard369 = assign20760_e21550;

        let (assign20770_e21561, assign20770_e21561_d_n5, assign20770_e21561_d_n6, assign20770_e21561_d_n7, assign20770_e21561_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign20770_e21561;
        var_itat_dn5 = assign20770_e21561_d_n5;
        var_itat_dn6 = assign20770_e21561_d_n6;
        var_itat_dn7 = assign20770_e21561_d_n7;
        var_itat_dn8 = assign20770_e21561_d_n8;

        let (assign20780_e21579, assign20780_e21579_d_n5, assign20780_e21579_d_n6, assign20780_e21579_d_n7, assign20780_e21579_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) {
        let assign20780_e21574: f64 = (var_wdep * var_one_minus_pbot);
        let assign20780_e21576: f64 = (assign20780_e21574 / var_vbi_minus_vjsrh);
        let assign20780_e21577: f64 = (var_btatpartbot * assign20780_e21576);
        (assign20780_e21577, (var_btatpartbot * ((var_wdep_dn5 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn6 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn7 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn8 * var_one_minus_pbot) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign20780_e21579;
        var_btat_dn5 = assign20780_e21579_d_n5;
        var_btat_dn6 = assign20780_e21579_d_n6;
        var_btat_dn7 = assign20780_e21579_d_n7;
        var_btat_dn8 = assign20780_e21579_d_n8;

        let (assign20790_e21595, assign20790_e21595_d_n5, assign20790_e21595_d_n6, assign20790_e21595_d_n7, assign20790_e21595_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) {
        let assign20790_e21591: f64 = (0.666666666666667 * var_atatbot);
        let assign20790_e21593: f64 = (assign20790_e21591 / var_btat);
        (assign20790_e21593, (-((assign20790_e21591 * var_btat_dn5) / (var_btat * var_btat))), (-((assign20790_e21591 * var_btat_dn6) / (var_btat * var_btat))), (-((assign20790_e21591 * var_btat_dn7) / (var_btat * var_btat))), (-((assign20790_e21591 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign20790_e21595;
        var_twoatatoverthreebtat_dn5 = assign20790_e21595_d_n5;
        var_twoatatoverthreebtat_dn6 = assign20790_e21595_d_n6;
        var_twoatatoverthreebtat_dn7 = assign20790_e21595_d_n7;
        var_twoatatoverthreebtat_dn8 = assign20790_e21595_d_n8;

        let (assign20800_e21609, assign20800_e21609_d_n5, assign20800_e21609_d_n6, assign20800_e21609_d_n7, assign20800_e21609_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) {
        let assign20800_e21607: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign20800_e21607, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign20800_e21609;
        var_umaxbeforelimiting_dn5 = assign20800_e21609_d_n5;
        var_umaxbeforelimiting_dn6 = assign20800_e21609_d_n6;
        var_umaxbeforelimiting_dn7 = assign20800_e21609_d_n7;
        var_umaxbeforelimiting_dn8 = assign20800_e21609_d_n8;

        let (assign20810_e21630, assign20810_e21630_d_n5, assign20810_e21630_d_n6, assign20810_e21630_d_n7, assign20810_e21630_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) {
        let assign20810_e21621: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign20810_e21624: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign20810_e21626: f64 = (assign20810_e21624 + 1.0);
        let assign20810_e21627: f64 = (assign20810_e21621 / assign20810_e21626);
        let assign20810_e21628: f64 = (assign20810_e21627).sqrt();
        (assign20810_e21628, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign20810_e21626) - (assign20810_e21621 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign20810_e21626 * assign20810_e21626)) / (2.0 * assign20810_e21628)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign20810_e21626) - (assign20810_e21621 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign20810_e21626 * assign20810_e21626)) / (2.0 * assign20810_e21628)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign20810_e21626) - (assign20810_e21621 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign20810_e21626 * assign20810_e21626)) / (2.0 * assign20810_e21628)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign20810_e21626) - (assign20810_e21621 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign20810_e21626 * assign20810_e21626)) / (2.0 * assign20810_e21628)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign20810_e21630;
        var_umax_dn5 = assign20810_e21630_d_n5;
        var_umax_dn6 = assign20810_e21630_d_n6;
        var_umax_dn7 = assign20810_e21630_d_n7;
        var_umax_dn8 = assign20810_e21630_d_n8;

        let (assign20820_e21643, assign20820_e21643_d_n5, assign20820_e21643_d_n6, assign20820_e21643_d_n7, assign20820_e21643_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) {
        let assign20820_e21641: f64 = (var_umax).sqrt();
        (assign20820_e21641, (var_umax_dn5 / (2.0 * assign20820_e21641)), (var_umax_dn6 / (2.0 * assign20820_e21641)), (var_umax_dn7 / (2.0 * assign20820_e21641)), (var_umax_dn8 / (2.0 * assign20820_e21641)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign20820_e21643;
        var_sqrtumax_dn5 = assign20820_e21643_d_n5;
        var_sqrtumax_dn6 = assign20820_e21643_d_n6;
        var_sqrtumax_dn7 = assign20820_e21643_d_n7;
        var_sqrtumax_dn8 = assign20820_e21643_d_n8;

        let (assign20830_e21657, assign20830_e21657_d_n5, assign20830_e21657_d_n6, assign20830_e21657_d_n7, assign20830_e21657_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) {
        let assign20830_e21655: f64 = (var_umax * var_sqrtumax);
        (assign20830_e21655, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign20830_e21657;
        var_umaxpoweronepointfive_dn5 = assign20830_e21657_d_n5;
        var_umaxpoweronepointfive_dn6 = assign20830_e21657_d_n6;
        var_umaxpoweronepointfive_dn7 = assign20830_e21657_d_n7;
        var_umaxpoweronepointfive_dn8 = assign20830_e21657_d_n8;

        let assign20840_e21659: f64 = (-p.p831);
        let assign20840_e21661: f64 = (assign20840_e21659 * var_one_over_one_minus_pbot);
        let assign20840_e21663: f64 = (-1.0);
        let assign20840_e21664: f64 = if assign20840_e21661 == assign20840_e21663 { 1.0 } else { 0.0 };
        var_guard370 = assign20840_e21664;

        let (assign20850_e21684, assign20850_e21684_d_n5, assign20850_e21684_d_n6, assign20850_e21684_d_n7, assign20850_e21684_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) && (var_guard370 != 0.0)) {
        let assign20850_e21680: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign20850_e21681: f64 = (1.0 + assign20850_e21680);
        let assign20850_e21682: f64 = (1.0 / assign20850_e21681);
        (assign20850_e21682, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign20850_e21681 * assign20850_e21681))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign20850_e21681 * assign20850_e21681))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign20850_e21681 * assign20850_e21681))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign20850_e21681 * assign20850_e21681))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign20850_e21684;
        var_wgamma_dn5 = assign20850_e21684_d_n5;
        var_wgamma_dn6 = assign20850_e21684_d_n6;
        var_wgamma_dn7 = assign20850_e21684_d_n7;
        var_wgamma_dn8 = assign20850_e21684_d_n8;

        let (assign20860_e21708, assign20860_e21708_d_n5, assign20860_e21708_d_n6, assign20860_e21708_d_n7, assign20860_e21708_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) && (var_guard370 == 0.0)) {
        let assign20860_e21700: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign20860_e21701: f64 = (1.0 + assign20860_e21700);
        let assign20860_e21703: f64 = (-p.p831);
        let assign20860_e21705: f64 = (assign20860_e21703 * var_one_over_one_minus_pbot);
        let assign20860_e21706: f64 = (assign20860_e21701).powf(assign20860_e21705);
        (assign20860_e21706, if 0.0 == 0.0 && ((assign20860_e21705) as f64).is_finite() && ((assign20860_e21705) as f64).fract() == 0.0 { if assign20860_e21705 == 0.0 { 0.0 } else { (assign20860_e21705 * ((assign20860_e21701).powf(assign20860_e21705 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign20860_e21706 * (assign20860_e21705 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign20860_e21701))) }, if 0.0 == 0.0 && ((assign20860_e21705) as f64).is_finite() && ((assign20860_e21705) as f64).fract() == 0.0 { if assign20860_e21705 == 0.0 { 0.0 } else { (assign20860_e21705 * ((assign20860_e21701).powf(assign20860_e21705 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign20860_e21706 * (assign20860_e21705 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign20860_e21701))) }, if 0.0 == 0.0 && ((assign20860_e21705) as f64).is_finite() && ((assign20860_e21705) as f64).fract() == 0.0 { if assign20860_e21705 == 0.0 { 0.0 } else { (assign20860_e21705 * ((assign20860_e21701).powf(assign20860_e21705 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign20860_e21706 * (assign20860_e21705 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign20860_e21701))) }, if 0.0 == 0.0 && ((assign20860_e21705) as f64).is_finite() && ((assign20860_e21705) as f64).fract() == 0.0 { if assign20860_e21705 == 0.0 { 0.0 } else { (assign20860_e21705 * ((assign20860_e21701).powf(assign20860_e21705 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign20860_e21706 * (assign20860_e21705 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign20860_e21701))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign20860_e21708;
        var_wgamma_dn5 = assign20860_e21708_d_n5;
        var_wgamma_dn6 = assign20860_e21708_d_n6;
        var_wgamma_dn7 = assign20860_e21708_d_n7;
        var_wgamma_dn8 = assign20860_e21708_d_n8;

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
        *var_guard364_slot = var_guard364;
        *var_guard365_slot = var_guard365;
        *var_guard366_slot = var_guard366;
        *var_guard367_slot = var_guard367;
        *var_guard368_slot = var_guard368;
        *var_guard369_slot = var_guard369;
        *var_guard370_slot = var_guard370;
        *var_id__blk219_slot = var_id__blk219;
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

    pub(super) fn stamp_transient_block_36(
        p: &Parameters,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatbot: f64,
        var_berfc: f64,
        var_btat: f64,
        var_btat_dn5: f64,
        var_btat_dn6: f64,
        var_btat_dn7: f64,
        var_btat_dn8: f64,
        var_cerfc: f64,
        var_fbbtbot: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard365: f64,
        var_guard369: f64,
        var_one_over_one_minus_pbot: f64,
        var_perfc: f64,
        var_sqrtumax: f64,
        var_sqrtumax_dn5: f64,
        var_sqrtumax_dn6: f64,
        var_sqrtumax_dn7: f64,
        var_sqrtumax_dn8: f64,
        var_twoatatoverthreebtat: f64,
        var_twoatatoverthreebtat_dn5: f64,
        var_twoatatoverthreebtat_dn6: f64,
        var_twoatatoverthreebtat_dn7: f64,
        var_twoatatoverthreebtat_dn8: f64,
        var_umax: f64,
        var_umax_dn5: f64,
        var_umax_dn6: f64,
        var_umax_dn7: f64,
        var_umax_dn8: f64,
        var_umaxpoweronepointfive: f64,
        var_umaxpoweronepointfive_dn5: f64,
        var_umaxpoweronepointfive_dn6: f64,
        var_umaxpoweronepointfive_dn7: f64,
        var_umaxpoweronepointfive_dn8: f64,
        var_v3: f64,
        var_vbbt: f64,
        var_vbirbotinv: f64,
        var_wdepnulrinvbot: f64,
        var_wgamma: f64,
        var_wgamma_dn5: f64,
        var_wgamma_dn6: f64,
        var_wgamma_dn7: f64,
        var_wgamma_dn8: f64,
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
        var_guard371_slot: &mut f64,
        var_guard372_slot: &mut f64,
        var_guard373_slot: &mut f64,
        var_guard374_slot: &mut f64,
        var_guard375_slot: &mut f64,
        var_guard376_slot: &mut f64,
        var_guard377_slot: &mut f64,
        var_guard378_slot: &mut f64,
        var_guard379_slot: &mut f64,
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
        let mut var_guard371: f64 = *var_guard371_slot;
        let mut var_guard372: f64 = *var_guard372_slot;
        let mut var_guard373: f64 = *var_guard373_slot;
        let mut var_guard374: f64 = *var_guard374_slot;
        let mut var_guard375: f64 = *var_guard375_slot;
        let mut var_guard376: f64 = *var_guard376_slot;
        let mut var_guard377: f64 = *var_guard377_slot;
        let mut var_guard378: f64 = *var_guard378_slot;
        let mut var_guard379: f64 = *var_guard379_slot;
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

        let (assign20870_e21726, assign20870_e21726_d_n5, assign20870_e21726_d_n6, assign20870_e21726_d_n7, assign20870_e21726_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) {
        let assign20870_e21720: f64 = (var_wsrh * var_wgamma);
        let assign20870_e21723: f64 = (var_wsrh + var_wgamma);
        let assign20870_e21724: f64 = (assign20870_e21720 / assign20870_e21723);
        (assign20870_e21724, ((((var_wsrh * var_wgamma_dn5) * assign20870_e21723) - (assign20870_e21720 * var_wgamma_dn5)) / (assign20870_e21723 * assign20870_e21723)), ((((var_wsrh * var_wgamma_dn6) * assign20870_e21723) - (assign20870_e21720 * var_wgamma_dn6)) / (assign20870_e21723 * assign20870_e21723)), ((((var_wsrh * var_wgamma_dn7) * assign20870_e21723) - (assign20870_e21720 * var_wgamma_dn7)) / (assign20870_e21723 * assign20870_e21723)), ((((var_wsrh * var_wgamma_dn8) * assign20870_e21723) - (assign20870_e21720 * var_wgamma_dn8)) / (assign20870_e21723 * assign20870_e21723)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign20870_e21726;
        var_wtat_dn5 = assign20870_e21726_d_n5;
        var_wtat_dn6 = assign20870_e21726_d_n6;
        var_wtat_dn7 = assign20870_e21726_d_n7;
        var_wtat_dn8 = assign20870_e21726_d_n8;

        let (assign20880_e21743, assign20880_e21743_d_n5, assign20880_e21743_d_n6, assign20880_e21743_d_n7, assign20880_e21743_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) {
        let assign20880_e21739: f64 = (var_btat / var_sqrtumax);
        let assign20880_e21740: f64 = (0.375 * assign20880_e21739);
        let assign20880_e21741: f64 = (assign20880_e21740).sqrt();
        (assign20880_e21741, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign20880_e21741)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign20880_e21741)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign20880_e21741)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign20880_e21741)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign20880_e21743;
        var_ktat_dn5 = assign20880_e21743_d_n5;
        var_ktat_dn6 = assign20880_e21743_d_n6;
        var_ktat_dn7 = assign20880_e21743_d_n7;
        var_ktat_dn8 = assign20880_e21743_d_n8;

        let (assign20890_e21761, assign20890_e21761_d_n5, assign20890_e21761_d_n6, assign20890_e21761_d_n7, assign20890_e21761_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) {
        let assign20890_e21756: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign20890_e21757: f64 = (2.0 * assign20890_e21756);
        let assign20890_e21759: f64 = (assign20890_e21757 - var_umax);
        (assign20890_e21759, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign20890_e21761;
        var_ltat_dn5 = assign20890_e21761_d_n5;
        var_ltat_dn6 = assign20890_e21761_d_n6;
        var_ltat_dn7 = assign20890_e21761_d_n7;
        var_ltat_dn8 = assign20890_e21761_d_n8;

        let (assign20900_e21787, assign20900_e21787_d_n5, assign20900_e21787_d_n6, assign20900_e21787_d_n7, assign20900_e21787_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) {
        let assign20900_e21773: f64 = (var_atatbot * var_twoatatoverthreebtat);
        let assign20900_e21775: f64 = (assign20900_e21773 * var_sqrtumax);
        let assign20900_e21778: f64 = (var_atatbot * var_umax);
        let assign20900_e21779: f64 = (assign20900_e21775 - assign20900_e21778);
        let assign20900_e21783: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign20900_e21784: f64 = (0.5 * assign20900_e21783);
        let assign20900_e21785: f64 = (assign20900_e21779 + assign20900_e21784);
        (assign20900_e21785, (((((var_atatbot * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign20900_e21773 * var_sqrtumax_dn5)) - (var_atatbot * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatbot * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign20900_e21773 * var_sqrtumax_dn6)) - (var_atatbot * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign20900_e21773 * var_sqrtumax_dn7)) - (var_atatbot * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign20900_e21773 * var_sqrtumax_dn8)) - (var_atatbot * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign20900_e21787;
        var_mtat_dn5 = assign20900_e21787_d_n5;
        var_mtat_dn6 = assign20900_e21787_d_n6;
        var_mtat_dn7 = assign20900_e21787_d_n7;
        var_mtat_dn8 = assign20900_e21787_d_n8;

        let (assign20910_e21803, assign20910_e21803_d_n5, assign20910_e21803_d_n6, assign20910_e21803_d_n7, assign20910_e21803_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) {
        let assign20910_e21799: f64 = (var_ltat - 1.0);
        let assign20910_e21801: f64 = (assign20910_e21799 * var_ktat);
        (assign20910_e21801, ((var_ltat_dn5 * var_ktat) + (assign20910_e21799 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign20910_e21799 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign20910_e21799 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign20910_e21799 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign20910_e21803;
        var_xerfc_dn5 = assign20910_e21803_d_n5;
        var_xerfc_dn6 = assign20910_e21803_d_n6;
        var_xerfc_dn7 = assign20910_e21803_d_n7;
        var_xerfc_dn8 = assign20910_e21803_d_n8;

        let (assign20920_e21817, assign20920_e21817_d_n5, assign20920_e21817_d_n6, assign20920_e21817_d_n7, assign20920_e21817_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) {
        let assign20920_e21815: f64 = (var_xerfc * var_xerfc);
        (assign20920_e21815, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign20920_e21817;
        var_ysq_dn5 = assign20920_e21817_d_n5;
        var_ysq_dn6 = assign20920_e21817_d_n6;
        var_ysq_dn7 = assign20920_e21817_d_n7;
        var_ysq_dn8 = assign20920_e21817_d_n8;

        let assign20930_e21820: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard371 = assign20930_e21820;

        let (assign20940_e21840, assign20940_e21840_d_n5, assign20940_e21840_d_n6, assign20940_e21840_d_n7, assign20940_e21840_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) && (var_guard371 != 0.0)) {
        let assign20940_e21836: f64 = (var_perfc * var_xerfc);
        let assign20940_e21837: f64 = (1.0 + assign20940_e21836);
        let assign20940_e21838: f64 = (1.0 / assign20940_e21837);
        (assign20940_e21838, (-((var_perfc * var_xerfc_dn5) / (assign20940_e21837 * assign20940_e21837))), (-((var_perfc * var_xerfc_dn6) / (assign20940_e21837 * assign20940_e21837))), (-((var_perfc * var_xerfc_dn7) / (assign20940_e21837 * assign20940_e21837))), (-((var_perfc * var_xerfc_dn8) / (assign20940_e21837 * assign20940_e21837))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign20940_e21840;
        var_terfc_dn5 = assign20940_e21840_d_n5;
        var_terfc_dn6 = assign20940_e21840_d_n6;
        var_terfc_dn7 = assign20940_e21840_d_n7;
        var_terfc_dn8 = assign20940_e21840_d_n8;

        let (assign20950_e21861, assign20950_e21861_d_n5, assign20950_e21861_d_n6, assign20950_e21861_d_n7, assign20950_e21861_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) && (var_guard371 == 0.0)) {
        let assign20950_e21857: f64 = (var_perfc * var_xerfc);
        let assign20950_e21858: f64 = (1.0 - assign20950_e21857);
        let assign20950_e21859: f64 = (1.0 / assign20950_e21858);
        (assign20950_e21859, (-((-(var_perfc * var_xerfc_dn5)) / (assign20950_e21858 * assign20950_e21858))), (-((-(var_perfc * var_xerfc_dn6)) / (assign20950_e21858 * assign20950_e21858))), (-((-(var_perfc * var_xerfc_dn7)) / (assign20950_e21858 * assign20950_e21858))), (-((-(var_perfc * var_xerfc_dn8)) / (assign20950_e21858 * assign20950_e21858))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign20950_e21861;
        var_terfc_dn5 = assign20950_e21861_d_n5;
        var_terfc_dn6 = assign20950_e21861_d_n6;
        var_terfc_dn7 = assign20950_e21861_d_n7;
        var_terfc_dn8 = assign20950_e21861_d_n8;

        let assign20960_e21863: f64 = (-var_ysq);
        let assign20960_e21865: f64 = (assign20960_e21863 + var_mtat);
        let assign20960_e21867: f64 = (-230.25850929940458);
        let assign20960_e21868: f64 = if assign20960_e21865 > assign20960_e21867 { 1.0 } else { 0.0 };
        var_guard372 = assign20960_e21868;

        let (assign20970_e21886, assign20970_e21886_d_n5, assign20970_e21886_d_n6, assign20970_e21886_d_n7, assign20970_e21886_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) && (var_guard372 != 0.0)) {
        let assign20970_e21881: f64 = (-var_ysq);
        let assign20970_e21883: f64 = (assign20970_e21881 + var_mtat);
        let assign20970_e21884: f64 = (assign20970_e21883).exp();
        (assign20970_e21884, (assign20970_e21884 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign20970_e21884 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign20970_e21884 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign20970_e21884 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20970_e21886;
        var_tmp_dn5 = assign20970_e21886_d_n5;
        var_tmp_dn6 = assign20970_e21886_d_n6;
        var_tmp_dn7 = assign20970_e21886_d_n7;
        var_tmp_dn8 = assign20970_e21886_d_n8;

        let (assign20980_e21935, assign20980_e21935_d_n5, assign20980_e21935_d_n6, assign20980_e21935_d_n7, assign20980_e21935_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) && (var_guard372 == 0.0)) {
        let assign20980_e21902: f64 = (-230.25850929940458);
        let assign20980_e21904: f64 = (-var_ysq);
        let assign20980_e21906: f64 = (assign20980_e21904 + var_mtat);
        let assign20980_e21907: f64 = (assign20980_e21902 - assign20980_e21906);
        let assign20980_e21911: f64 = (-230.25850929940458);
        let assign20980_e21913: f64 = (-var_ysq);
        let assign20980_e21915: f64 = (assign20980_e21913 + var_mtat);
        let assign20980_e21916: f64 = (assign20980_e21911 - assign20980_e21915);
        let assign20980_e21919: f64 = (-230.25850929940458);
        let assign20980_e21921: f64 = (-var_ysq);
        let assign20980_e21923: f64 = (assign20980_e21921 + var_mtat);
        let assign20980_e21924: f64 = (assign20980_e21919 - assign20980_e21923);
        let assign20980_e21926: f64 = (assign20980_e21924 * 0.3333333333333333);
        let assign20980_e21927: f64 = (1.0 + assign20980_e21926);
        let assign20980_e21928: f64 = (assign20980_e21916 * assign20980_e21927);
        let assign20980_e21929: f64 = (0.5 * assign20980_e21928);
        let assign20980_e21930: f64 = (1.0 + assign20980_e21929);
        let assign20980_e21931: f64 = (assign20980_e21907 * assign20980_e21930);
        let assign20980_e21932: f64 = (1.0 + assign20980_e21931);
        let assign20980_e21933: f64 = (1e-100 / assign20980_e21932);
        (assign20980_e21933, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign20980_e21930) + (assign20980_e21907 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign20980_e21927) + (assign20980_e21916 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign20980_e21932 * assign20980_e21932))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign20980_e21930) + (assign20980_e21907 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign20980_e21927) + (assign20980_e21916 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign20980_e21932 * assign20980_e21932))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign20980_e21930) + (assign20980_e21907 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign20980_e21927) + (assign20980_e21916 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign20980_e21932 * assign20980_e21932))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign20980_e21930) + (assign20980_e21907 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign20980_e21927) + (assign20980_e21916 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign20980_e21932 * assign20980_e21932))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign20980_e21935;
        var_tmp_dn5 = assign20980_e21935_d_n5;
        var_tmp_dn6 = assign20980_e21935_d_n6;
        var_tmp_dn7 = assign20980_e21935_d_n7;
        var_tmp_dn8 = assign20980_e21935_d_n8;

        let (assign20990_e21965, assign20990_e21965_d_n5, assign20990_e21965_d_n6, assign20990_e21965_d_n7, assign20990_e21965_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) {
        let assign20990_e21947: f64 = (0.29214664 * var_terfc);
        let assign20990_e21951: f64 = (var_terfc * var_terfc);
        let assign20990_e21952: f64 = (var_berfc * assign20990_e21951);
        let assign20990_e21953: f64 = (assign20990_e21947 + assign20990_e21952);
        let assign20990_e21957: f64 = (var_terfc * var_terfc);
        let assign20990_e21959: f64 = (assign20990_e21957 * var_terfc);
        let assign20990_e21960: f64 = (var_cerfc * assign20990_e21959);
        let assign20990_e21961: f64 = (assign20990_e21953 + assign20990_e21960);
        let assign20990_e21963: f64 = (assign20990_e21961 * var_tmp);
        (assign20990_e21963, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign20990_e21957 * var_terfc_dn5)))) * var_tmp) + (assign20990_e21961 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign20990_e21957 * var_terfc_dn6)))) * var_tmp) + (assign20990_e21961 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign20990_e21957 * var_terfc_dn7)))) * var_tmp) + (assign20990_e21961 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign20990_e21957 * var_terfc_dn8)))) * var_tmp) + (assign20990_e21961 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign20990_e21965;
        var_erfcpos_dn5 = assign20990_e21965_d_n5;
        var_erfcpos_dn6 = assign20990_e21965_d_n6;
        var_erfcpos_dn7 = assign20990_e21965_d_n7;
        var_erfcpos_dn8 = assign20990_e21965_d_n8;

        let assign21000_e21968: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard373 = assign21000_e21968;

        let (assign21010_e21982, assign21010_e21982_d_n5, assign21010_e21982_d_n6, assign21010_e21982_d_n7, assign21010_e21982_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) && (var_guard373 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign21010_e21982;
        var_erfctimesexpmtat_dn5 = assign21010_e21982_d_n5;
        var_erfctimesexpmtat_dn6 = assign21010_e21982_d_n6;
        var_erfctimesexpmtat_dn7 = assign21010_e21982_d_n7;
        var_erfctimesexpmtat_dn8 = assign21010_e21982_d_n8;

        let assign21020_e21985: f64 = (-230.25850929940458);
        let assign21020_e21986: f64 = if var_mtat > assign21020_e21985 { 1.0 } else { 0.0 };
        var_guard374 = assign21020_e21986;

        let (assign21030_e22004, assign21030_e22004_d_n5, assign21030_e22004_d_n6, assign21030_e22004_d_n7, assign21030_e22004_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) && (var_guard373 == 0.0)) && (var_guard374 != 0.0)) {
        let assign21030_e22002: f64 = (var_mtat).exp();
        (assign21030_e22002, (assign21030_e22002 * var_mtat_dn5), (assign21030_e22002 * var_mtat_dn6), (assign21030_e22002 * var_mtat_dn7), (assign21030_e22002 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21030_e22004;
        var_tmp_dn5 = assign21030_e22004_d_n5;
        var_tmp_dn6 = assign21030_e22004_d_n6;
        var_tmp_dn7 = assign21030_e22004_d_n7;
        var_tmp_dn8 = assign21030_e22004_d_n8;

        let (assign21040_e22047, assign21040_e22047_d_n5, assign21040_e22047_d_n6, assign21040_e22047_d_n7, assign21040_e22047_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) && (var_guard373 == 0.0)) && (var_guard374 == 0.0)) {
        let assign21040_e22023: f64 = (-230.25850929940458);
        let assign21040_e22025: f64 = (assign21040_e22023 - var_mtat);
        let assign21040_e22029: f64 = (-230.25850929940458);
        let assign21040_e22031: f64 = (assign21040_e22029 - var_mtat);
        let assign21040_e22034: f64 = (-230.25850929940458);
        let assign21040_e22036: f64 = (assign21040_e22034 - var_mtat);
        let assign21040_e22038: f64 = (assign21040_e22036 * 0.3333333333333333);
        let assign21040_e22039: f64 = (1.0 + assign21040_e22038);
        let assign21040_e22040: f64 = (assign21040_e22031 * assign21040_e22039);
        let assign21040_e22041: f64 = (0.5 * assign21040_e22040);
        let assign21040_e22042: f64 = (1.0 + assign21040_e22041);
        let assign21040_e22043: f64 = (assign21040_e22025 * assign21040_e22042);
        let assign21040_e22044: f64 = (1.0 + assign21040_e22043);
        let assign21040_e22045: f64 = (1e-100 / assign21040_e22044);
        (assign21040_e22045, (-((1e-100 * (((-var_mtat_dn5) * assign21040_e22042) + (assign21040_e22025 * (0.5 * (((-var_mtat_dn5) * assign21040_e22039) + (assign21040_e22031 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign21040_e22044 * assign21040_e22044))), (-((1e-100 * (((-var_mtat_dn6) * assign21040_e22042) + (assign21040_e22025 * (0.5 * (((-var_mtat_dn6) * assign21040_e22039) + (assign21040_e22031 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign21040_e22044 * assign21040_e22044))), (-((1e-100 * (((-var_mtat_dn7) * assign21040_e22042) + (assign21040_e22025 * (0.5 * (((-var_mtat_dn7) * assign21040_e22039) + (assign21040_e22031 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign21040_e22044 * assign21040_e22044))), (-((1e-100 * (((-var_mtat_dn8) * assign21040_e22042) + (assign21040_e22025 * (0.5 * (((-var_mtat_dn8) * assign21040_e22039) + (assign21040_e22031 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign21040_e22044 * assign21040_e22044))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21040_e22047;
        var_tmp_dn5 = assign21040_e22047_d_n5;
        var_tmp_dn6 = assign21040_e22047_d_n6;
        var_tmp_dn7 = assign21040_e22047_d_n7;
        var_tmp_dn8 = assign21040_e22047_d_n8;

        let (assign21050_e22066, assign21050_e22066_d_n5, assign21050_e22066_d_n6, assign21050_e22066_d_n7, assign21050_e22066_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) && (var_guard373 == 0.0)) {
        let assign21050_e22062: f64 = (2.0 * var_tmp);
        let assign21050_e22064: f64 = (assign21050_e22062 - var_erfcpos);
        (assign21050_e22064, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign21050_e22066;
        var_erfctimesexpmtat_dn5 = assign21050_e22066_d_n5;
        var_erfctimesexpmtat_dn6 = assign21050_e22066_d_n6;
        var_erfctimesexpmtat_dn7 = assign21050_e22066_d_n7;
        var_erfctimesexpmtat_dn8 = assign21050_e22066_d_n8;

        let (assign21060_e22086, assign21060_e22086_d_n5, assign21060_e22086_d_n6, assign21060_e22086_d_n7, assign21060_e22086_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) {
        let assign21060_e22078: f64 = (1.772453850905516 * 0.5);
        let assign21060_e22081: f64 = (var_atatbot * var_erfctimesexpmtat);
        let assign21060_e22083: f64 = (assign21060_e22081 / var_ktat);
        let assign21060_e22084: f64 = (assign21060_e22078 * assign21060_e22083);
        (assign21060_e22084, (assign21060_e22078 * ((((var_atatbot * var_erfctimesexpmtat_dn5) * var_ktat) - (assign21060_e22081 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign21060_e22078 * ((((var_atatbot * var_erfctimesexpmtat_dn6) * var_ktat) - (assign21060_e22081 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign21060_e22078 * ((((var_atatbot * var_erfctimesexpmtat_dn7) * var_ktat) - (assign21060_e22081 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign21060_e22078 * ((((var_atatbot * var_erfctimesexpmtat_dn8) * var_ktat) - (assign21060_e22081 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign21060_e22086;
        var_gammamax_dn5 = assign21060_e22086_d_n5;
        var_gammamax_dn6 = assign21060_e22086_d_n6;
        var_gammamax_dn7 = assign21060_e22086_d_n7;
        var_gammamax_dn8 = assign21060_e22086_d_n8;

        let (assign21070_e22104, assign21070_e22104_d_n5, assign21070_e22104_d_n6, assign21070_e22104_d_n7, assign21070_e22104_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard369 == 0.0)) {
        let assign21070_e22099: f64 = (var_asrh * var_gammamax);
        let assign21070_e22101: f64 = (assign21070_e22099 * var_wtat);
        let assign21070_e22102: f64 = (p.p845 * assign21070_e22101);
        (assign21070_e22102, (p.p845 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign21070_e22099 * var_wtat_dn5))), (p.p845 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign21070_e22099 * var_wtat_dn6))), (p.p845 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign21070_e22099 * var_wtat_dn7))), (p.p845 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign21070_e22099 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign21070_e22104;
        var_itat_dn5 = assign21070_e22104_d_n5;
        var_itat_dn6 = assign21070_e22104_d_n6;
        var_itat_dn7 = assign21070_e22104_d_n7;
        var_itat_dn8 = assign21070_e22104_d_n8;

        let assign21080_e22107: f64 = if p.p851 == 0.0 { 1.0 } else { 0.0 };
        var_guard375 = assign21080_e22107;

        let (assign21090_e22118, assign21090_e22118_d_n5, assign21090_e22118_d_n6, assign21090_e22118_d_n7, assign21090_e22118_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard375 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign21090_e22118;
        var_ibbt_dn5 = assign21090_e22118_d_n5;
        var_ibbt_dn6 = assign21090_e22118_d_n6;
        var_ibbt_dn7 = assign21090_e22118_d_n7;
        var_ibbt_dn8 = assign21090_e22118_d_n8;

        let assign21100_e22121: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        var_guard376 = assign21100_e22121;

        let (assign21110_e22140, assign21110_e22140_d_n5, assign21110_e22140_d_n6, assign21110_e22140_d_n7, assign21110_e22140_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard375 == 0.0)) && (var_guard376 != 0.0)) {
        let assign21110_e22135: f64 = (p.p828 - var_vbbt);
        let assign21110_e22137: f64 = (assign21110_e22135 * var_vbirbotinv);
        let assign21110_e22138: f64 = (assign21110_e22137).sqrt();
        (assign21110_e22138, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21110_e22140;
        var_tmp_dn5 = assign21110_e22140_d_n5;
        var_tmp_dn6 = assign21110_e22140_d_n6;
        var_tmp_dn7 = assign21110_e22140_d_n7;
        var_tmp_dn8 = assign21110_e22140_d_n8;

        let (assign21120_e22161, assign21120_e22161_d_n5, assign21120_e22161_d_n6, assign21120_e22161_d_n7, assign21120_e22161_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard375 == 0.0)) && (var_guard376 == 0.0)) {
        let assign21120_e22155: f64 = (p.p828 - var_vbbt);
        let assign21120_e22157: f64 = (assign21120_e22155 * var_vbirbotinv);
        let assign21120_e22159: f64 = (assign21120_e22157).powf(p.p831);
        (assign21120_e22159, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21120_e22161;
        var_tmp_dn5 = assign21120_e22161_d_n5;
        var_tmp_dn6 = assign21120_e22161_d_n6;
        var_tmp_dn7 = assign21120_e22161_d_n7;
        var_tmp_dn8 = assign21120_e22161_d_n8;

        let (assign21130_e22181, assign21130_e22181_d_n5, assign21130_e22181_d_n6, assign21130_e22181_d_n7, assign21130_e22181_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard375 == 0.0)) {
        let assign21130_e22174: f64 = (p.p828 - var_vbbt);
        let assign21130_e22176: f64 = (assign21130_e22174 * var_wdepnulrinvbot);
        let assign21130_e22178: f64 = (assign21130_e22176 / var_tmp);
        let assign21130_e22179: f64 = (var_one_over_one_minus_pbot * assign21130_e22178);
        (assign21130_e22179, (var_one_over_one_minus_pbot * (-((assign21130_e22176 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign21130_e22176 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign21130_e22176 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign21130_e22176 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign21130_e22181;
        var_fmaxr_dn5 = assign21130_e22181_d_n5;
        var_fmaxr_dn6 = assign21130_e22181_d_n6;
        var_fmaxr_dn7 = assign21130_e22181_d_n7;
        var_fmaxr_dn8 = assign21130_e22181_d_n8;

        let assign21140_e22183: f64 = (-var_fbbtbot);
        let assign21140_e22185: f64 = (assign21140_e22183 / var_fmaxr);
        let assign21140_e22186: f64 = (assign21140_e22185).abs();
        let assign21140_e22188: f64 = if assign21140_e22186 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard377 = assign21140_e22188;

        let (assign21150_e22206, assign21150_e22206_d_n5, assign21150_e22206_d_n6, assign21150_e22206_d_n7, assign21150_e22206_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard375 == 0.0)) && (var_guard377 != 0.0)) {
        let assign21150_e22201: f64 = (-var_fbbtbot);
        let assign21150_e22203: f64 = (assign21150_e22201 / var_fmaxr);
        let assign21150_e22204: f64 = (assign21150_e22203).exp();
        (assign21150_e22204, (assign21150_e22204 * (-((assign21150_e22201 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign21150_e22204 * (-((assign21150_e22201 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign21150_e22204 * (-((assign21150_e22201 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign21150_e22204 * (-((assign21150_e22201 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21150_e22206;
        var_tmp_dn5 = assign21150_e22206_d_n5;
        var_tmp_dn6 = assign21150_e22206_d_n6;
        var_tmp_dn7 = assign21150_e22206_d_n7;
        var_tmp_dn8 = assign21150_e22206_d_n8;

        let assign21160_e22208: f64 = (-var_fbbtbot);
        let assign21160_e22210: f64 = (assign21160_e22208 / var_fmaxr);
        let assign21160_e22212: f64 = if assign21160_e22210 < 0.0 { 1.0 } else { 0.0 };
        var_guard378 = assign21160_e22212;

        let (assign21170_e22263, assign21170_e22263_d_n5, assign21170_e22263_d_n6, assign21170_e22263_d_n7, assign21170_e22263_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard375 == 0.0)) && (var_guard377 == 0.0)) && (var_guard378 != 0.0)) {
        let assign21170_e22230: f64 = (-230.25850929940458);
        let assign21170_e22232: f64 = (-var_fbbtbot);
        let assign21170_e22234: f64 = (assign21170_e22232 / var_fmaxr);
        let assign21170_e22235: f64 = (assign21170_e22230 - assign21170_e22234);
        let assign21170_e22239: f64 = (-230.25850929940458);
        let assign21170_e22241: f64 = (-var_fbbtbot);
        let assign21170_e22243: f64 = (assign21170_e22241 / var_fmaxr);
        let assign21170_e22244: f64 = (assign21170_e22239 - assign21170_e22243);
        let assign21170_e22247: f64 = (-230.25850929940458);
        let assign21170_e22249: f64 = (-var_fbbtbot);
        let assign21170_e22251: f64 = (assign21170_e22249 / var_fmaxr);
        let assign21170_e22252: f64 = (assign21170_e22247 - assign21170_e22251);
        let assign21170_e22254: f64 = (assign21170_e22252 * 0.3333333333333333);
        let assign21170_e22255: f64 = (1.0 + assign21170_e22254);
        let assign21170_e22256: f64 = (assign21170_e22244 * assign21170_e22255);
        let assign21170_e22257: f64 = (0.5 * assign21170_e22256);
        let assign21170_e22258: f64 = (1.0 + assign21170_e22257);
        let assign21170_e22259: f64 = (assign21170_e22235 * assign21170_e22258);
        let assign21170_e22260: f64 = (1.0 + assign21170_e22259);
        let assign21170_e22261: f64 = (1e-100 / assign21170_e22260);
        (assign21170_e22261, (-((1e-100 * (((-(-((assign21170_e22232 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign21170_e22258) + (assign21170_e22235 * (0.5 * (((-(-((assign21170_e22241 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign21170_e22255) + (assign21170_e22244 * ((-(-((assign21170_e22249 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign21170_e22260 * assign21170_e22260))), (-((1e-100 * (((-(-((assign21170_e22232 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign21170_e22258) + (assign21170_e22235 * (0.5 * (((-(-((assign21170_e22241 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign21170_e22255) + (assign21170_e22244 * ((-(-((assign21170_e22249 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign21170_e22260 * assign21170_e22260))), (-((1e-100 * (((-(-((assign21170_e22232 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign21170_e22258) + (assign21170_e22235 * (0.5 * (((-(-((assign21170_e22241 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign21170_e22255) + (assign21170_e22244 * ((-(-((assign21170_e22249 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign21170_e22260 * assign21170_e22260))), (-((1e-100 * (((-(-((assign21170_e22232 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign21170_e22258) + (assign21170_e22235 * (0.5 * (((-(-((assign21170_e22241 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign21170_e22255) + (assign21170_e22244 * ((-(-((assign21170_e22249 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign21170_e22260 * assign21170_e22260))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21170_e22263;
        var_tmp_dn5 = assign21170_e22263_d_n5;
        var_tmp_dn6 = assign21170_e22263_d_n6;
        var_tmp_dn7 = assign21170_e22263_d_n7;
        var_tmp_dn8 = assign21170_e22263_d_n8;

        let (assign21180_e22312, assign21180_e22312_d_n5, assign21180_e22312_d_n6, assign21180_e22312_d_n7, assign21180_e22312_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard375 == 0.0)) && (var_guard377 == 0.0)) && (var_guard378 == 0.0)) {
        let assign21180_e22282: f64 = (-var_fbbtbot);
        let assign21180_e22284: f64 = (assign21180_e22282 / var_fmaxr);
        let assign21180_e22286: f64 = (assign21180_e22284 - 230.25850929940458);
        let assign21180_e22290: f64 = (-var_fbbtbot);
        let assign21180_e22292: f64 = (assign21180_e22290 / var_fmaxr);
        let assign21180_e22294: f64 = (assign21180_e22292 - 230.25850929940458);
        let assign21180_e22297: f64 = (-var_fbbtbot);
        let assign21180_e22299: f64 = (assign21180_e22297 / var_fmaxr);
        let assign21180_e22301: f64 = (assign21180_e22299 - 230.25850929940458);
        let assign21180_e22303: f64 = (assign21180_e22301 * 0.3333333333333333);
        let assign21180_e22304: f64 = (1.0 + assign21180_e22303);
        let assign21180_e22305: f64 = (assign21180_e22294 * assign21180_e22304);
        let assign21180_e22306: f64 = (0.5 * assign21180_e22305);
        let assign21180_e22307: f64 = (1.0 + assign21180_e22306);
        let assign21180_e22308: f64 = (assign21180_e22286 * assign21180_e22307);
        let assign21180_e22309: f64 = (1.0 + assign21180_e22308);
        let assign21180_e22310: f64 = (1e100 * assign21180_e22309);
        (assign21180_e22310, (1e100 * (((-((assign21180_e22282 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign21180_e22307) + (assign21180_e22286 * (0.5 * (((-((assign21180_e22290 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign21180_e22304) + (assign21180_e22294 * ((-((assign21180_e22297 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign21180_e22282 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign21180_e22307) + (assign21180_e22286 * (0.5 * (((-((assign21180_e22290 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign21180_e22304) + (assign21180_e22294 * ((-((assign21180_e22297 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign21180_e22282 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign21180_e22307) + (assign21180_e22286 * (0.5 * (((-((assign21180_e22290 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign21180_e22304) + (assign21180_e22294 * ((-((assign21180_e22297 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign21180_e22282 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign21180_e22307) + (assign21180_e22286 * (0.5 * (((-((assign21180_e22290 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign21180_e22304) + (assign21180_e22294 * ((-((assign21180_e22297 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21180_e22312;
        var_tmp_dn5 = assign21180_e22312_d_n5;
        var_tmp_dn6 = assign21180_e22312_d_n6;
        var_tmp_dn7 = assign21180_e22312_d_n7;
        var_tmp_dn8 = assign21180_e22312_d_n8;

        let (assign21190_e22332, assign21190_e22332_d_n5, assign21190_e22332_d_n6, assign21190_e22332_d_n7, assign21190_e22332_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard375 == 0.0)) {
        let assign21190_e22325: f64 = (var_v3 * var_fmaxr);
        let assign21190_e22327: f64 = (assign21190_e22325 * var_fmaxr);
        let assign21190_e22329: f64 = (assign21190_e22327 * var_tmp);
        let assign21190_e22330: f64 = (p.p851 * assign21190_e22329);
        (assign21190_e22330, (p.p851 * (((((var_v3 * var_fmaxr_dn5) * var_fmaxr) + (assign21190_e22325 * var_fmaxr_dn5)) * var_tmp) + (assign21190_e22327 * var_tmp_dn5))), (p.p851 * (((((var_v3 * var_fmaxr_dn6) * var_fmaxr) + (assign21190_e22325 * var_fmaxr_dn6)) * var_tmp) + (assign21190_e22327 * var_tmp_dn6))), (p.p851 * (((((var_v3 * var_fmaxr_dn7) * var_fmaxr) + (assign21190_e22325 * var_fmaxr_dn7)) * var_tmp) + (assign21190_e22327 * var_tmp_dn7))), (p.p851 * (((((var_v3 * var_fmaxr_dn8) * var_fmaxr) + (assign21190_e22325 * var_fmaxr_dn8)) * var_tmp) + (assign21190_e22327 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign21190_e22332;
        var_ibbt_dn5 = assign21190_e22332_d_n5;
        var_ibbt_dn6 = assign21190_e22332_d_n6;
        var_ibbt_dn7 = assign21190_e22332_d_n7;
        var_ibbt_dn8 = assign21190_e22332_d_n8;

        let assign21200_e22335: f64 = if p.p860 > 1000.0 { 1.0 } else { 0.0 };
        var_guard379 = assign21200_e22335;

        let (assign21210_e22346, assign21210_e22346_d_n5, assign21210_e22346_d_n6, assign21210_e22346_d_n7, assign21210_e22346_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard379 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign21210_e22346;
        var_fbreakdown_dn5 = assign21210_e22346_d_n5;
        var_fbreakdown_dn6 = assign21210_e22346_d_n6;
        var_fbreakdown_dn7 = assign21210_e22346_d_n7;
        var_fbreakdown_dn8 = assign21210_e22346_d_n8;

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
        *var_guard371_slot = var_guard371;
        *var_guard372_slot = var_guard372;
        *var_guard373_slot = var_guard373;
        *var_guard374_slot = var_guard374;
        *var_guard375_slot = var_guard375;
        *var_guard376_slot = var_guard376;
        *var_guard377_slot = var_guard377;
        *var_guard378_slot = var_guard378;
        *var_guard379_slot = var_guard379;
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

    pub(super) fn stamp_transient_block_37(
        p: &Parameters,
        var_alphaav: f64,
        var_atatsti: f64,
        var_btatpartsti: f64,
        var_fstopbot: f64,
        var_ftdsti: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard365: f64,
        var_guard379: f64,
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
        var_slopebot: f64,
        var_two_psistar: f64,
        var_vav: f64,
        var_vbirstiinv: f64,
        var_vbisti: f64,
        var_vbrinvbot: f64,
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
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_guard380_slot: &mut f64,
        var_guard381_slot: &mut f64,
        var_guard382_slot: &mut f64,
        var_guard383_slot: &mut f64,
        var_guard384_slot: &mut f64,
        var_guard385_slot: &mut f64,
        var_guard386_slot: &mut f64,
        var_guard387_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
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
        let mut var_guard380: f64 = *var_guard380_slot;
        let mut var_guard381: f64 = *var_guard381_slot;
        let mut var_guard382: f64 = *var_guard382_slot;
        let mut var_guard383: f64 = *var_guard383_slot;
        let mut var_guard384: f64 = *var_guard384_slot;
        let mut var_guard385: f64 = *var_guard385_slot;
        let mut var_guard386: f64 = *var_guard386_slot;
        let mut var_guard387: f64 = *var_guard387_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
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

        let assign21220_e22349: f64 = (-var_alphaav);
        let assign21220_e22351: f64 = (assign21220_e22349 * p.p860);
        let assign21220_e22352: f64 = if var_vav > assign21220_e22351 { 1.0 } else { 0.0 };
        var_guard380 = assign21220_e22352;

        let assign21230_e22355: f64 = if p.p863 == 4.0 { 1.0 } else { 0.0 };
        var_guard381 = assign21230_e22355;

        let (assign21240_e22385, assign21240_e22385_d_n5, assign21240_e22385_d_n6, assign21240_e22385_d_n7, assign21240_e22385_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard379 == 0.0)) && (var_guard380 != 0.0)) && (var_guard381 != 0.0)) {
        let assign21240_e22371: f64 = (var_vav * var_vbrinvbot);
        let assign21240_e22374: f64 = (var_vav * var_vbrinvbot);
        let assign21240_e22375: f64 = (assign21240_e22371 * assign21240_e22374);
        let assign21240_e22378: f64 = (var_vav * var_vbrinvbot);
        let assign21240_e22379: f64 = (assign21240_e22375 * assign21240_e22378);
        let assign21240_e22382: f64 = (var_vav * var_vbrinvbot);
        let assign21240_e22383: f64 = (assign21240_e22379 * assign21240_e22382);
        (assign21240_e22383, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21240_e22385;
        var_tmp_dn5 = assign21240_e22385_d_n5;
        var_tmp_dn6 = assign21240_e22385_d_n6;
        var_tmp_dn7 = assign21240_e22385_d_n7;
        var_tmp_dn8 = assign21240_e22385_d_n8;

        let (assign21250_e22407, assign21250_e22407_d_n5, assign21250_e22407_d_n6, assign21250_e22407_d_n7, assign21250_e22407_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard379 == 0.0)) && (var_guard380 != 0.0)) && (var_guard381 == 0.0)) {
        let assign21250_e22402: f64 = (var_vav * var_vbrinvbot);
        let assign21250_e22403: f64 = (assign21250_e22402).abs();
        let assign21250_e22405: f64 = (assign21250_e22403).powf(p.p863);
        (assign21250_e22405, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21250_e22407;
        var_tmp_dn5 = assign21250_e22407_d_n5;
        var_tmp_dn6 = assign21250_e22407_d_n6;
        var_tmp_dn7 = assign21250_e22407_d_n7;
        var_tmp_dn8 = assign21250_e22407_d_n8;

        let (assign21260_e22425, assign21260_e22425_d_n5, assign21260_e22425_d_n6, assign21260_e22425_d_n7, assign21260_e22425_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard379 == 0.0)) && (var_guard380 != 0.0)) {
        let assign21260_e22422: f64 = (1.0 - var_tmp);
        let assign21260_e22423: f64 = (1.0 / assign21260_e22422);
        (assign21260_e22423, (-((-var_tmp_dn5) / (assign21260_e22422 * assign21260_e22422))), (-((-var_tmp_dn6) / (assign21260_e22422 * assign21260_e22422))), (-((-var_tmp_dn7) / (assign21260_e22422 * assign21260_e22422))), (-((-var_tmp_dn8) / (assign21260_e22422 * assign21260_e22422))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign21260_e22425;
        var_fbreakdown_dn5 = assign21260_e22425_d_n5;
        var_fbreakdown_dn6 = assign21260_e22425_d_n6;
        var_fbreakdown_dn7 = assign21260_e22425_d_n7;
        var_fbreakdown_dn8 = assign21260_e22425_d_n8;

        let (assign21270_e22448, assign21270_e22448_d_n5, assign21270_e22448_d_n6, assign21270_e22448_d_n7, assign21270_e22448_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) && (var_guard379 == 0.0)) && (var_guard380 == 0.0)) {
        let assign21270_e22442: f64 = (var_alphaav * p.p860);
        let assign21270_e22443: f64 = (var_vav + assign21270_e22442);
        let assign21270_e22445: f64 = (assign21270_e22443 * var_slopebot);
        let assign21270_e22446: f64 = (var_fstopbot + assign21270_e22445);
        (assign21270_e22446, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign21270_e22448;
        var_fbreakdown_dn5 = assign21270_e22448_d_n5;
        var_fbreakdown_dn6 = assign21270_e22448_d_n6;
        var_fbreakdown_dn7 = assign21270_e22448_d_n7;
        var_fbreakdown_dn8 = assign21270_e22448_d_n8;

        let (assign21280_e22467, assign21280_e22467_d_n5, assign21280_e22467_d_n6, assign21280_e22467_d_n7, assign21280_e22467_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard365 == 0.0)) {
        let assign21280_e22458: f64 = (var_id__blk219 + var_isrh);
        let assign21280_e22460: f64 = (assign21280_e22458 + var_itat);
        let assign21280_e22462: f64 = (assign21280_e22460 + var_ibbt);
        let assign21280_e22463: f64 = (p.p29 * assign21280_e22462);
        let assign21280_e22465: f64 = (assign21280_e22463 * var_fbreakdown);
        (assign21280_e22465, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign21280_e22463 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign21280_e22463 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign21280_e22463 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign21280_e22463 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign21280_e22467;
        var_ijunbot_dn5 = assign21280_e22467_d_n5;
        var_ijunbot_dn6 = assign21280_e22467_d_n6;
        var_ijunbot_dn7 = assign21280_e22467_d_n7;
        var_ijunbot_dn8 = assign21280_e22467_d_n8;

        let assign21290_e22470: f64 = if var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard382 = assign21290_e22470;

        let (assign21300_e22478, assign21300_e22478_d_n5, assign21300_e22478_d_n6, assign21300_e22478_d_n7, assign21300_e22478_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign21300_e22478;
        var_ijunsti_dn5 = assign21300_e22478_d_n5;
        var_ijunsti_dn6 = assign21300_e22478_d_n6;
        var_ijunsti_dn7 = assign21300_e22478_d_n7;
        var_ijunsti_dn8 = assign21300_e22478_d_n8;

        let (assign21310_e22489,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) {
        let assign21310_e22487: f64 = (var_idsatsti * var_idmult);
        (assign21310_e22487,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign21310_e22489;

        let assign21320_e22496: f64 = if ((p.p841 == 0.0) && (p.p846 == 0.0)) { 1.0 } else { 0.0 };
        var_guard383 = assign21320_e22496;

        let (assign21330_e22507, assign21330_e22507_d_n5, assign21330_e22507_d_n6, assign21330_e22507_d_n7, assign21330_e22507_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard383 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign21330_e22507;
        var_isrh_dn5 = assign21330_e22507_d_n5;
        var_isrh_dn6 = assign21330_e22507_d_n6;
        var_isrh_dn7 = assign21330_e22507_d_n7;
        var_isrh_dn8 = assign21330_e22507_d_n8;

        let (assign21340_e22521,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard383 == 0.0)) {
        let assign21340_e22519: f64 = (var_vbisti - var_vjsrh);
        (assign21340_e22519,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign21340_e22521;

        let (assign21350_e22540,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard383 == 0.0)) {
        let assign21350_e22535: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign21350_e22536: f64 = (1.0 - assign21350_e22535);
        let assign21350_e22537: f64 = (assign21350_e22536).sqrt();
        let assign21350_e22538: f64 = (1.0 - assign21350_e22537);
        (assign21350_e22538,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign21350_e22540;

        let assign21360_e22543: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        var_guard384 = assign21360_e22543;

        let (assign21370_e22557,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard383 == 0.0)) && (var_guard384 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign21370_e22557;

        let (assign21380_e22589,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard383 == 0.0)) && (var_guard384 == 0.0)) {
        let assign21380_e22572: f64 = (var_wsrhstep * var_wsrhstep);
        let assign21380_e22574: f64 = (var_wsrhstep).ln();
        let assign21380_e22575: f64 = (assign21380_e22572 * assign21380_e22574);
        let assign21380_e22578: f64 = (1.0 - var_wsrhstep);
        let assign21380_e22579: f64 = (assign21380_e22575 / assign21380_e22578);
        let assign21380_e22581: f64 = (assign21380_e22579 + var_wsrhstep);
        let assign21380_e22585: f64 = (2.0 * p.p832);
        let assign21380_e22586: f64 = (1.0 - assign21380_e22585);
        let assign21380_e22587: f64 = (assign21380_e22581 * assign21380_e22586);
        (assign21380_e22587,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign21380_e22589;

        let (assign21390_e22603,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard383 == 0.0)) {
        let assign21390_e22601: f64 = (var_wsrhstep + var_dwsrh);
        (assign21390_e22601,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign21390_e22603;

        let assign21400_e22606: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        var_guard385 = assign21400_e22606;

        let (assign21410_e22623, assign21410_e22623_d_n5, assign21410_e22623_d_n6, assign21410_e22623_d_n7, assign21410_e22623_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard383 == 0.0)) && (var_guard385 != 0.0)) {
        let assign21410_e22620: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign21410_e22621: f64 = (assign21410_e22620).sqrt();
        (assign21410_e22621, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21410_e22623;
        var_tmp_dn5 = assign21410_e22623_d_n5;
        var_tmp_dn6 = assign21410_e22623_d_n6;
        var_tmp_dn7 = assign21410_e22623_d_n7;
        var_tmp_dn8 = assign21410_e22623_d_n8;

        let (assign21420_e22642, assign21420_e22642_d_n5, assign21420_e22642_d_n6, assign21420_e22642_d_n7, assign21420_e22642_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard383 == 0.0)) && (var_guard385 == 0.0)) {
        let assign21420_e22638: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign21420_e22640: f64 = (assign21420_e22638).powf(p.p832);
        (assign21420_e22640, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21420_e22642;
        var_tmp_dn5 = assign21420_e22642_d_n5;
        var_tmp_dn6 = assign21420_e22642_d_n6;
        var_tmp_dn7 = assign21420_e22642_d_n7;
        var_tmp_dn8 = assign21420_e22642_d_n8;

        let (assign21430_e22656, assign21430_e22656_d_n5, assign21430_e22656_d_n6, assign21430_e22656_d_n7, assign21430_e22656_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard383 == 0.0)) {
        let assign21430_e22654: f64 = (var_wdepnulrsti * var_tmp);
        (assign21430_e22654, (var_wdepnulrsti * var_tmp_dn5), (var_wdepnulrsti * var_tmp_dn6), (var_wdepnulrsti * var_tmp_dn7), (var_wdepnulrsti * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign21430_e22656;
        var_wdep_dn5 = assign21430_e22656_d_n5;
        var_wdep_dn6 = assign21430_e22656_d_n6;
        var_wdep_dn7 = assign21430_e22656_d_n7;
        var_wdep_dn8 = assign21430_e22656_d_n8;

        let (assign21440_e22674, assign21440_e22674_d_n5, assign21440_e22674_d_n6, assign21440_e22674_d_n7, assign21440_e22674_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard383 == 0.0)) {
        let assign21440_e22669: f64 = (var_zinv - 1.0);
        let assign21440_e22671: f64 = (assign21440_e22669 * var_wdep);
        let assign21440_e22672: f64 = (var_ftdsti * assign21440_e22671);
        (assign21440_e22672, (var_ftdsti * (assign21440_e22669 * var_wdep_dn5)), (var_ftdsti * (assign21440_e22669 * var_wdep_dn6)), (var_ftdsti * (assign21440_e22669 * var_wdep_dn7)), (var_ftdsti * (assign21440_e22669 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign21440_e22674;
        var_asrh_dn5 = assign21440_e22674_d_n5;
        var_asrh_dn6 = assign21440_e22674_d_n6;
        var_asrh_dn7 = assign21440_e22674_d_n7;
        var_asrh_dn8 = assign21440_e22674_d_n8;

        let (assign21450_e22690, assign21450_e22690_d_n5, assign21450_e22690_d_n6, assign21450_e22690_d_n7, assign21450_e22690_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard383 == 0.0)) {
        let assign21450_e22687: f64 = (var_asrh * var_wsrh);
        let assign21450_e22688: f64 = (p.p841 * assign21450_e22687);
        (assign21450_e22688, (p.p841 * (var_asrh_dn5 * var_wsrh)), (p.p841 * (var_asrh_dn6 * var_wsrh)), (p.p841 * (var_asrh_dn7 * var_wsrh)), (p.p841 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign21450_e22690;
        var_isrh_dn5 = assign21450_e22690_d_n5;
        var_isrh_dn6 = assign21450_e22690_d_n6;
        var_isrh_dn7 = assign21450_e22690_d_n7;
        var_isrh_dn8 = assign21450_e22690_d_n8;

        let assign21460_e22693: f64 = if p.p846 == 0.0 { 1.0 } else { 0.0 };
        var_guard386 = assign21460_e22693;

        let (assign21470_e22704, assign21470_e22704_d_n5, assign21470_e22704_d_n6, assign21470_e22704_d_n7, assign21470_e22704_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign21470_e22704;
        var_itat_dn5 = assign21470_e22704_d_n5;
        var_itat_dn6 = assign21470_e22704_d_n6;
        var_itat_dn7 = assign21470_e22704_d_n7;
        var_itat_dn8 = assign21470_e22704_d_n8;

        let (assign21480_e22722, assign21480_e22722_d_n5, assign21480_e22722_d_n6, assign21480_e22722_d_n7, assign21480_e22722_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) {
        let assign21480_e22717: f64 = (var_wdep * var_one_minus_psti);
        let assign21480_e22719: f64 = (assign21480_e22717 / var_vbi_minus_vjsrh);
        let assign21480_e22720: f64 = (var_btatpartsti * assign21480_e22719);
        (assign21480_e22720, (var_btatpartsti * ((var_wdep_dn5 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn6 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn7 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn8 * var_one_minus_psti) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign21480_e22722;
        var_btat_dn5 = assign21480_e22722_d_n5;
        var_btat_dn6 = assign21480_e22722_d_n6;
        var_btat_dn7 = assign21480_e22722_d_n7;
        var_btat_dn8 = assign21480_e22722_d_n8;

        let (assign21490_e22738, assign21490_e22738_d_n5, assign21490_e22738_d_n6, assign21490_e22738_d_n7, assign21490_e22738_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) {
        let assign21490_e22734: f64 = (0.666666666666667 * var_atatsti);
        let assign21490_e22736: f64 = (assign21490_e22734 / var_btat);
        (assign21490_e22736, (-((assign21490_e22734 * var_btat_dn5) / (var_btat * var_btat))), (-((assign21490_e22734 * var_btat_dn6) / (var_btat * var_btat))), (-((assign21490_e22734 * var_btat_dn7) / (var_btat * var_btat))), (-((assign21490_e22734 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign21490_e22738;
        var_twoatatoverthreebtat_dn5 = assign21490_e22738_d_n5;
        var_twoatatoverthreebtat_dn6 = assign21490_e22738_d_n6;
        var_twoatatoverthreebtat_dn7 = assign21490_e22738_d_n7;
        var_twoatatoverthreebtat_dn8 = assign21490_e22738_d_n8;

        let (assign21500_e22752, assign21500_e22752_d_n5, assign21500_e22752_d_n6, assign21500_e22752_d_n7, assign21500_e22752_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) {
        let assign21500_e22750: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign21500_e22750, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign21500_e22752;
        var_umaxbeforelimiting_dn5 = assign21500_e22752_d_n5;
        var_umaxbeforelimiting_dn6 = assign21500_e22752_d_n6;
        var_umaxbeforelimiting_dn7 = assign21500_e22752_d_n7;
        var_umaxbeforelimiting_dn8 = assign21500_e22752_d_n8;

        let (assign21510_e22773, assign21510_e22773_d_n5, assign21510_e22773_d_n6, assign21510_e22773_d_n7, assign21510_e22773_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) {
        let assign21510_e22764: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign21510_e22767: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign21510_e22769: f64 = (assign21510_e22767 + 1.0);
        let assign21510_e22770: f64 = (assign21510_e22764 / assign21510_e22769);
        let assign21510_e22771: f64 = (assign21510_e22770).sqrt();
        (assign21510_e22771, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign21510_e22769) - (assign21510_e22764 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign21510_e22769 * assign21510_e22769)) / (2.0 * assign21510_e22771)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign21510_e22769) - (assign21510_e22764 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign21510_e22769 * assign21510_e22769)) / (2.0 * assign21510_e22771)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign21510_e22769) - (assign21510_e22764 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign21510_e22769 * assign21510_e22769)) / (2.0 * assign21510_e22771)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign21510_e22769) - (assign21510_e22764 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign21510_e22769 * assign21510_e22769)) / (2.0 * assign21510_e22771)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign21510_e22773;
        var_umax_dn5 = assign21510_e22773_d_n5;
        var_umax_dn6 = assign21510_e22773_d_n6;
        var_umax_dn7 = assign21510_e22773_d_n7;
        var_umax_dn8 = assign21510_e22773_d_n8;

        let (assign21520_e22786, assign21520_e22786_d_n5, assign21520_e22786_d_n6, assign21520_e22786_d_n7, assign21520_e22786_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) {
        let assign21520_e22784: f64 = (var_umax).sqrt();
        (assign21520_e22784, (var_umax_dn5 / (2.0 * assign21520_e22784)), (var_umax_dn6 / (2.0 * assign21520_e22784)), (var_umax_dn7 / (2.0 * assign21520_e22784)), (var_umax_dn8 / (2.0 * assign21520_e22784)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign21520_e22786;
        var_sqrtumax_dn5 = assign21520_e22786_d_n5;
        var_sqrtumax_dn6 = assign21520_e22786_d_n6;
        var_sqrtumax_dn7 = assign21520_e22786_d_n7;
        var_sqrtumax_dn8 = assign21520_e22786_d_n8;

        let (assign21530_e22800, assign21530_e22800_d_n5, assign21530_e22800_d_n6, assign21530_e22800_d_n7, assign21530_e22800_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) {
        let assign21530_e22798: f64 = (var_umax * var_sqrtumax);
        (assign21530_e22798, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign21530_e22800;
        var_umaxpoweronepointfive_dn5 = assign21530_e22800_d_n5;
        var_umaxpoweronepointfive_dn6 = assign21530_e22800_d_n6;
        var_umaxpoweronepointfive_dn7 = assign21530_e22800_d_n7;
        var_umaxpoweronepointfive_dn8 = assign21530_e22800_d_n8;

        let assign21540_e22802: f64 = (-p.p832);
        let assign21540_e22804: f64 = (assign21540_e22802 * var_one_over_one_minus_psti);
        let assign21540_e22806: f64 = (-1.0);
        let assign21540_e22807: f64 = if assign21540_e22804 == assign21540_e22806 { 1.0 } else { 0.0 };
        var_guard387 = assign21540_e22807;

        let (assign21550_e22827, assign21550_e22827_d_n5, assign21550_e22827_d_n6, assign21550_e22827_d_n7, assign21550_e22827_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) && (var_guard387 != 0.0)) {
        let assign21550_e22823: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign21550_e22824: f64 = (1.0 + assign21550_e22823);
        let assign21550_e22825: f64 = (1.0 / assign21550_e22824);
        (assign21550_e22825, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign21550_e22824 * assign21550_e22824))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign21550_e22824 * assign21550_e22824))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign21550_e22824 * assign21550_e22824))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign21550_e22824 * assign21550_e22824))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign21550_e22827;
        var_wgamma_dn5 = assign21550_e22827_d_n5;
        var_wgamma_dn6 = assign21550_e22827_d_n6;
        var_wgamma_dn7 = assign21550_e22827_d_n7;
        var_wgamma_dn8 = assign21550_e22827_d_n8;

        let (assign21560_e22851, assign21560_e22851_d_n5, assign21560_e22851_d_n6, assign21560_e22851_d_n7, assign21560_e22851_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) && (var_guard387 == 0.0)) {
        let assign21560_e22843: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign21560_e22844: f64 = (1.0 + assign21560_e22843);
        let assign21560_e22846: f64 = (-p.p832);
        let assign21560_e22848: f64 = (assign21560_e22846 * var_one_over_one_minus_psti);
        let assign21560_e22849: f64 = (assign21560_e22844).powf(assign21560_e22848);
        (assign21560_e22849, if 0.0 == 0.0 && ((assign21560_e22848) as f64).is_finite() && ((assign21560_e22848) as f64).fract() == 0.0 { if assign21560_e22848 == 0.0 { 0.0 } else { (assign21560_e22848 * ((assign21560_e22844).powf(assign21560_e22848 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign21560_e22849 * (assign21560_e22848 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign21560_e22844))) }, if 0.0 == 0.0 && ((assign21560_e22848) as f64).is_finite() && ((assign21560_e22848) as f64).fract() == 0.0 { if assign21560_e22848 == 0.0 { 0.0 } else { (assign21560_e22848 * ((assign21560_e22844).powf(assign21560_e22848 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign21560_e22849 * (assign21560_e22848 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign21560_e22844))) }, if 0.0 == 0.0 && ((assign21560_e22848) as f64).is_finite() && ((assign21560_e22848) as f64).fract() == 0.0 { if assign21560_e22848 == 0.0 { 0.0 } else { (assign21560_e22848 * ((assign21560_e22844).powf(assign21560_e22848 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign21560_e22849 * (assign21560_e22848 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign21560_e22844))) }, if 0.0 == 0.0 && ((assign21560_e22848) as f64).is_finite() && ((assign21560_e22848) as f64).fract() == 0.0 { if assign21560_e22848 == 0.0 { 0.0 } else { (assign21560_e22848 * ((assign21560_e22844).powf(assign21560_e22848 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign21560_e22849 * (assign21560_e22848 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign21560_e22844))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign21560_e22851;
        var_wgamma_dn5 = assign21560_e22851_d_n5;
        var_wgamma_dn6 = assign21560_e22851_d_n6;
        var_wgamma_dn7 = assign21560_e22851_d_n7;
        var_wgamma_dn8 = assign21560_e22851_d_n8;

        let (assign21570_e22869, assign21570_e22869_d_n5, assign21570_e22869_d_n6, assign21570_e22869_d_n7, assign21570_e22869_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) {
        let assign21570_e22863: f64 = (var_wsrh * var_wgamma);
        let assign21570_e22866: f64 = (var_wsrh + var_wgamma);
        let assign21570_e22867: f64 = (assign21570_e22863 / assign21570_e22866);
        (assign21570_e22867, ((((var_wsrh * var_wgamma_dn5) * assign21570_e22866) - (assign21570_e22863 * var_wgamma_dn5)) / (assign21570_e22866 * assign21570_e22866)), ((((var_wsrh * var_wgamma_dn6) * assign21570_e22866) - (assign21570_e22863 * var_wgamma_dn6)) / (assign21570_e22866 * assign21570_e22866)), ((((var_wsrh * var_wgamma_dn7) * assign21570_e22866) - (assign21570_e22863 * var_wgamma_dn7)) / (assign21570_e22866 * assign21570_e22866)), ((((var_wsrh * var_wgamma_dn8) * assign21570_e22866) - (assign21570_e22863 * var_wgamma_dn8)) / (assign21570_e22866 * assign21570_e22866)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign21570_e22869;
        var_wtat_dn5 = assign21570_e22869_d_n5;
        var_wtat_dn6 = assign21570_e22869_d_n6;
        var_wtat_dn7 = assign21570_e22869_d_n7;
        var_wtat_dn8 = assign21570_e22869_d_n8;

        let (assign21580_e22886, assign21580_e22886_d_n5, assign21580_e22886_d_n6, assign21580_e22886_d_n7, assign21580_e22886_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) {
        let assign21580_e22882: f64 = (var_btat / var_sqrtumax);
        let assign21580_e22883: f64 = (0.375 * assign21580_e22882);
        let assign21580_e22884: f64 = (assign21580_e22883).sqrt();
        (assign21580_e22884, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign21580_e22884)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign21580_e22884)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign21580_e22884)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign21580_e22884)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign21580_e22886;
        var_ktat_dn5 = assign21580_e22886_d_n5;
        var_ktat_dn6 = assign21580_e22886_d_n6;
        var_ktat_dn7 = assign21580_e22886_d_n7;
        var_ktat_dn8 = assign21580_e22886_d_n8;

        let (assign21590_e22904, assign21590_e22904_d_n5, assign21590_e22904_d_n6, assign21590_e22904_d_n7, assign21590_e22904_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) {
        let assign21590_e22899: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign21590_e22900: f64 = (2.0 * assign21590_e22899);
        let assign21590_e22902: f64 = (assign21590_e22900 - var_umax);
        (assign21590_e22902, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign21590_e22904;
        var_ltat_dn5 = assign21590_e22904_d_n5;
        var_ltat_dn6 = assign21590_e22904_d_n6;
        var_ltat_dn7 = assign21590_e22904_d_n7;
        var_ltat_dn8 = assign21590_e22904_d_n8;

        let (assign21600_e22930, assign21600_e22930_d_n5, assign21600_e22930_d_n6, assign21600_e22930_d_n7, assign21600_e22930_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) {
        let assign21600_e22916: f64 = (var_atatsti * var_twoatatoverthreebtat);
        let assign21600_e22918: f64 = (assign21600_e22916 * var_sqrtumax);
        let assign21600_e22921: f64 = (var_atatsti * var_umax);
        let assign21600_e22922: f64 = (assign21600_e22918 - assign21600_e22921);
        let assign21600_e22926: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign21600_e22927: f64 = (0.5 * assign21600_e22926);
        let assign21600_e22928: f64 = (assign21600_e22922 + assign21600_e22927);
        (assign21600_e22928, (((((var_atatsti * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign21600_e22916 * var_sqrtumax_dn5)) - (var_atatsti * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatsti * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign21600_e22916 * var_sqrtumax_dn6)) - (var_atatsti * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign21600_e22916 * var_sqrtumax_dn7)) - (var_atatsti * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign21600_e22916 * var_sqrtumax_dn8)) - (var_atatsti * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign21600_e22930;
        var_mtat_dn5 = assign21600_e22930_d_n5;
        var_mtat_dn6 = assign21600_e22930_d_n6;
        var_mtat_dn7 = assign21600_e22930_d_n7;
        var_mtat_dn8 = assign21600_e22930_d_n8;

        let (assign21610_e22946, assign21610_e22946_d_n5, assign21610_e22946_d_n6, assign21610_e22946_d_n7, assign21610_e22946_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) {
        let assign21610_e22942: f64 = (var_ltat - 1.0);
        let assign21610_e22944: f64 = (assign21610_e22942 * var_ktat);
        (assign21610_e22944, ((var_ltat_dn5 * var_ktat) + (assign21610_e22942 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign21610_e22942 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign21610_e22942 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign21610_e22942 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign21610_e22946;
        var_xerfc_dn5 = assign21610_e22946_d_n5;
        var_xerfc_dn6 = assign21610_e22946_d_n6;
        var_xerfc_dn7 = assign21610_e22946_d_n7;
        var_xerfc_dn8 = assign21610_e22946_d_n8;

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
        *var_guard380_slot = var_guard380;
        *var_guard381_slot = var_guard381;
        *var_guard382_slot = var_guard382;
        *var_guard383_slot = var_guard383;
        *var_guard384_slot = var_guard384;
        *var_guard385_slot = var_guard385;
        *var_guard386_slot = var_guard386;
        *var_guard387_slot = var_guard387;
        *var_id__blk219_slot = var_id__blk219;
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
        var_berfc: f64,
        var_cerfc: f64,
        var_fbbtsti: f64,
        var_fstopsti: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard382: f64,
        var_guard386: f64,
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
        var_one_over_one_minus_psti: f64,
        var_perfc: f64,
        var_slopesti: f64,
        var_v3: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirstiinv: f64,
        var_vbrinvsti: f64,
        var_wdepnulrinvsti: f64,
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
        var_guard388_slot: &mut f64,
        var_guard389_slot: &mut f64,
        var_guard390_slot: &mut f64,
        var_guard391_slot: &mut f64,
        var_guard392_slot: &mut f64,
        var_guard393_slot: &mut f64,
        var_guard394_slot: &mut f64,
        var_guard395_slot: &mut f64,
        var_guard396_slot: &mut f64,
        var_guard397_slot: &mut f64,
        var_guard398_slot: &mut f64,
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
        let mut var_guard388: f64 = *var_guard388_slot;
        let mut var_guard389: f64 = *var_guard389_slot;
        let mut var_guard390: f64 = *var_guard390_slot;
        let mut var_guard391: f64 = *var_guard391_slot;
        let mut var_guard392: f64 = *var_guard392_slot;
        let mut var_guard393: f64 = *var_guard393_slot;
        let mut var_guard394: f64 = *var_guard394_slot;
        let mut var_guard395: f64 = *var_guard395_slot;
        let mut var_guard396: f64 = *var_guard396_slot;
        let mut var_guard397: f64 = *var_guard397_slot;
        let mut var_guard398: f64 = *var_guard398_slot;
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

        let (assign21620_e22960, assign21620_e22960_d_n5, assign21620_e22960_d_n6, assign21620_e22960_d_n7, assign21620_e22960_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) {
        let assign21620_e22958: f64 = (var_xerfc * var_xerfc);
        (assign21620_e22958, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign21620_e22960;
        var_ysq_dn5 = assign21620_e22960_d_n5;
        var_ysq_dn6 = assign21620_e22960_d_n6;
        var_ysq_dn7 = assign21620_e22960_d_n7;
        var_ysq_dn8 = assign21620_e22960_d_n8;

        let assign21630_e22963: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard388 = assign21630_e22963;

        let (assign21640_e22983, assign21640_e22983_d_n5, assign21640_e22983_d_n6, assign21640_e22983_d_n7, assign21640_e22983_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) && (var_guard388 != 0.0)) {
        let assign21640_e22979: f64 = (var_perfc * var_xerfc);
        let assign21640_e22980: f64 = (1.0 + assign21640_e22979);
        let assign21640_e22981: f64 = (1.0 / assign21640_e22980);
        (assign21640_e22981, (-((var_perfc * var_xerfc_dn5) / (assign21640_e22980 * assign21640_e22980))), (-((var_perfc * var_xerfc_dn6) / (assign21640_e22980 * assign21640_e22980))), (-((var_perfc * var_xerfc_dn7) / (assign21640_e22980 * assign21640_e22980))), (-((var_perfc * var_xerfc_dn8) / (assign21640_e22980 * assign21640_e22980))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign21640_e22983;
        var_terfc_dn5 = assign21640_e22983_d_n5;
        var_terfc_dn6 = assign21640_e22983_d_n6;
        var_terfc_dn7 = assign21640_e22983_d_n7;
        var_terfc_dn8 = assign21640_e22983_d_n8;

        let (assign21650_e23004, assign21650_e23004_d_n5, assign21650_e23004_d_n6, assign21650_e23004_d_n7, assign21650_e23004_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) && (var_guard388 == 0.0)) {
        let assign21650_e23000: f64 = (var_perfc * var_xerfc);
        let assign21650_e23001: f64 = (1.0 - assign21650_e23000);
        let assign21650_e23002: f64 = (1.0 / assign21650_e23001);
        (assign21650_e23002, (-((-(var_perfc * var_xerfc_dn5)) / (assign21650_e23001 * assign21650_e23001))), (-((-(var_perfc * var_xerfc_dn6)) / (assign21650_e23001 * assign21650_e23001))), (-((-(var_perfc * var_xerfc_dn7)) / (assign21650_e23001 * assign21650_e23001))), (-((-(var_perfc * var_xerfc_dn8)) / (assign21650_e23001 * assign21650_e23001))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign21650_e23004;
        var_terfc_dn5 = assign21650_e23004_d_n5;
        var_terfc_dn6 = assign21650_e23004_d_n6;
        var_terfc_dn7 = assign21650_e23004_d_n7;
        var_terfc_dn8 = assign21650_e23004_d_n8;

        let assign21660_e23006: f64 = (-var_ysq);
        let assign21660_e23008: f64 = (assign21660_e23006 + var_mtat);
        let assign21660_e23010: f64 = (-230.25850929940458);
        let assign21660_e23011: f64 = if assign21660_e23008 > assign21660_e23010 { 1.0 } else { 0.0 };
        var_guard389 = assign21660_e23011;

        let (assign21670_e23029, assign21670_e23029_d_n5, assign21670_e23029_d_n6, assign21670_e23029_d_n7, assign21670_e23029_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) && (var_guard389 != 0.0)) {
        let assign21670_e23024: f64 = (-var_ysq);
        let assign21670_e23026: f64 = (assign21670_e23024 + var_mtat);
        let assign21670_e23027: f64 = (assign21670_e23026).exp();
        (assign21670_e23027, (assign21670_e23027 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign21670_e23027 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign21670_e23027 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign21670_e23027 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21670_e23029;
        var_tmp_dn5 = assign21670_e23029_d_n5;
        var_tmp_dn6 = assign21670_e23029_d_n6;
        var_tmp_dn7 = assign21670_e23029_d_n7;
        var_tmp_dn8 = assign21670_e23029_d_n8;

        let (assign21680_e23078, assign21680_e23078_d_n5, assign21680_e23078_d_n6, assign21680_e23078_d_n7, assign21680_e23078_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) && (var_guard389 == 0.0)) {
        let assign21680_e23045: f64 = (-230.25850929940458);
        let assign21680_e23047: f64 = (-var_ysq);
        let assign21680_e23049: f64 = (assign21680_e23047 + var_mtat);
        let assign21680_e23050: f64 = (assign21680_e23045 - assign21680_e23049);
        let assign21680_e23054: f64 = (-230.25850929940458);
        let assign21680_e23056: f64 = (-var_ysq);
        let assign21680_e23058: f64 = (assign21680_e23056 + var_mtat);
        let assign21680_e23059: f64 = (assign21680_e23054 - assign21680_e23058);
        let assign21680_e23062: f64 = (-230.25850929940458);
        let assign21680_e23064: f64 = (-var_ysq);
        let assign21680_e23066: f64 = (assign21680_e23064 + var_mtat);
        let assign21680_e23067: f64 = (assign21680_e23062 - assign21680_e23066);
        let assign21680_e23069: f64 = (assign21680_e23067 * 0.3333333333333333);
        let assign21680_e23070: f64 = (1.0 + assign21680_e23069);
        let assign21680_e23071: f64 = (assign21680_e23059 * assign21680_e23070);
        let assign21680_e23072: f64 = (0.5 * assign21680_e23071);
        let assign21680_e23073: f64 = (1.0 + assign21680_e23072);
        let assign21680_e23074: f64 = (assign21680_e23050 * assign21680_e23073);
        let assign21680_e23075: f64 = (1.0 + assign21680_e23074);
        let assign21680_e23076: f64 = (1e-100 / assign21680_e23075);
        (assign21680_e23076, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign21680_e23073) + (assign21680_e23050 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign21680_e23070) + (assign21680_e23059 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign21680_e23075 * assign21680_e23075))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign21680_e23073) + (assign21680_e23050 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign21680_e23070) + (assign21680_e23059 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign21680_e23075 * assign21680_e23075))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign21680_e23073) + (assign21680_e23050 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign21680_e23070) + (assign21680_e23059 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign21680_e23075 * assign21680_e23075))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign21680_e23073) + (assign21680_e23050 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign21680_e23070) + (assign21680_e23059 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign21680_e23075 * assign21680_e23075))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21680_e23078;
        var_tmp_dn5 = assign21680_e23078_d_n5;
        var_tmp_dn6 = assign21680_e23078_d_n6;
        var_tmp_dn7 = assign21680_e23078_d_n7;
        var_tmp_dn8 = assign21680_e23078_d_n8;

        let (assign21690_e23108, assign21690_e23108_d_n5, assign21690_e23108_d_n6, assign21690_e23108_d_n7, assign21690_e23108_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) {
        let assign21690_e23090: f64 = (0.29214664 * var_terfc);
        let assign21690_e23094: f64 = (var_terfc * var_terfc);
        let assign21690_e23095: f64 = (var_berfc * assign21690_e23094);
        let assign21690_e23096: f64 = (assign21690_e23090 + assign21690_e23095);
        let assign21690_e23100: f64 = (var_terfc * var_terfc);
        let assign21690_e23102: f64 = (assign21690_e23100 * var_terfc);
        let assign21690_e23103: f64 = (var_cerfc * assign21690_e23102);
        let assign21690_e23104: f64 = (assign21690_e23096 + assign21690_e23103);
        let assign21690_e23106: f64 = (assign21690_e23104 * var_tmp);
        (assign21690_e23106, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign21690_e23100 * var_terfc_dn5)))) * var_tmp) + (assign21690_e23104 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign21690_e23100 * var_terfc_dn6)))) * var_tmp) + (assign21690_e23104 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign21690_e23100 * var_terfc_dn7)))) * var_tmp) + (assign21690_e23104 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign21690_e23100 * var_terfc_dn8)))) * var_tmp) + (assign21690_e23104 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign21690_e23108;
        var_erfcpos_dn5 = assign21690_e23108_d_n5;
        var_erfcpos_dn6 = assign21690_e23108_d_n6;
        var_erfcpos_dn7 = assign21690_e23108_d_n7;
        var_erfcpos_dn8 = assign21690_e23108_d_n8;

        let assign21700_e23111: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard390 = assign21700_e23111;

        let (assign21710_e23125, assign21710_e23125_d_n5, assign21710_e23125_d_n6, assign21710_e23125_d_n7, assign21710_e23125_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) && (var_guard390 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign21710_e23125;
        var_erfctimesexpmtat_dn5 = assign21710_e23125_d_n5;
        var_erfctimesexpmtat_dn6 = assign21710_e23125_d_n6;
        var_erfctimesexpmtat_dn7 = assign21710_e23125_d_n7;
        var_erfctimesexpmtat_dn8 = assign21710_e23125_d_n8;

        let assign21720_e23128: f64 = (-230.25850929940458);
        let assign21720_e23129: f64 = if var_mtat > assign21720_e23128 { 1.0 } else { 0.0 };
        var_guard391 = assign21720_e23129;

        let (assign21730_e23147, assign21730_e23147_d_n5, assign21730_e23147_d_n6, assign21730_e23147_d_n7, assign21730_e23147_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) && (var_guard390 == 0.0)) && (var_guard391 != 0.0)) {
        let assign21730_e23145: f64 = (var_mtat).exp();
        (assign21730_e23145, (assign21730_e23145 * var_mtat_dn5), (assign21730_e23145 * var_mtat_dn6), (assign21730_e23145 * var_mtat_dn7), (assign21730_e23145 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21730_e23147;
        var_tmp_dn5 = assign21730_e23147_d_n5;
        var_tmp_dn6 = assign21730_e23147_d_n6;
        var_tmp_dn7 = assign21730_e23147_d_n7;
        var_tmp_dn8 = assign21730_e23147_d_n8;

        let (assign21740_e23190, assign21740_e23190_d_n5, assign21740_e23190_d_n6, assign21740_e23190_d_n7, assign21740_e23190_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) && (var_guard390 == 0.0)) && (var_guard391 == 0.0)) {
        let assign21740_e23166: f64 = (-230.25850929940458);
        let assign21740_e23168: f64 = (assign21740_e23166 - var_mtat);
        let assign21740_e23172: f64 = (-230.25850929940458);
        let assign21740_e23174: f64 = (assign21740_e23172 - var_mtat);
        let assign21740_e23177: f64 = (-230.25850929940458);
        let assign21740_e23179: f64 = (assign21740_e23177 - var_mtat);
        let assign21740_e23181: f64 = (assign21740_e23179 * 0.3333333333333333);
        let assign21740_e23182: f64 = (1.0 + assign21740_e23181);
        let assign21740_e23183: f64 = (assign21740_e23174 * assign21740_e23182);
        let assign21740_e23184: f64 = (0.5 * assign21740_e23183);
        let assign21740_e23185: f64 = (1.0 + assign21740_e23184);
        let assign21740_e23186: f64 = (assign21740_e23168 * assign21740_e23185);
        let assign21740_e23187: f64 = (1.0 + assign21740_e23186);
        let assign21740_e23188: f64 = (1e-100 / assign21740_e23187);
        (assign21740_e23188, (-((1e-100 * (((-var_mtat_dn5) * assign21740_e23185) + (assign21740_e23168 * (0.5 * (((-var_mtat_dn5) * assign21740_e23182) + (assign21740_e23174 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign21740_e23187 * assign21740_e23187))), (-((1e-100 * (((-var_mtat_dn6) * assign21740_e23185) + (assign21740_e23168 * (0.5 * (((-var_mtat_dn6) * assign21740_e23182) + (assign21740_e23174 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign21740_e23187 * assign21740_e23187))), (-((1e-100 * (((-var_mtat_dn7) * assign21740_e23185) + (assign21740_e23168 * (0.5 * (((-var_mtat_dn7) * assign21740_e23182) + (assign21740_e23174 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign21740_e23187 * assign21740_e23187))), (-((1e-100 * (((-var_mtat_dn8) * assign21740_e23185) + (assign21740_e23168 * (0.5 * (((-var_mtat_dn8) * assign21740_e23182) + (assign21740_e23174 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign21740_e23187 * assign21740_e23187))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21740_e23190;
        var_tmp_dn5 = assign21740_e23190_d_n5;
        var_tmp_dn6 = assign21740_e23190_d_n6;
        var_tmp_dn7 = assign21740_e23190_d_n7;
        var_tmp_dn8 = assign21740_e23190_d_n8;

        let (assign21750_e23209, assign21750_e23209_d_n5, assign21750_e23209_d_n6, assign21750_e23209_d_n7, assign21750_e23209_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) && (var_guard390 == 0.0)) {
        let assign21750_e23205: f64 = (2.0 * var_tmp);
        let assign21750_e23207: f64 = (assign21750_e23205 - var_erfcpos);
        (assign21750_e23207, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign21750_e23209;
        var_erfctimesexpmtat_dn5 = assign21750_e23209_d_n5;
        var_erfctimesexpmtat_dn6 = assign21750_e23209_d_n6;
        var_erfctimesexpmtat_dn7 = assign21750_e23209_d_n7;
        var_erfctimesexpmtat_dn8 = assign21750_e23209_d_n8;

        let (assign21760_e23229, assign21760_e23229_d_n5, assign21760_e23229_d_n6, assign21760_e23229_d_n7, assign21760_e23229_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) {
        let assign21760_e23221: f64 = (1.772453850905516 * 0.5);
        let assign21760_e23224: f64 = (var_atatsti * var_erfctimesexpmtat);
        let assign21760_e23226: f64 = (assign21760_e23224 / var_ktat);
        let assign21760_e23227: f64 = (assign21760_e23221 * assign21760_e23226);
        (assign21760_e23227, (assign21760_e23221 * ((((var_atatsti * var_erfctimesexpmtat_dn5) * var_ktat) - (assign21760_e23224 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign21760_e23221 * ((((var_atatsti * var_erfctimesexpmtat_dn6) * var_ktat) - (assign21760_e23224 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign21760_e23221 * ((((var_atatsti * var_erfctimesexpmtat_dn7) * var_ktat) - (assign21760_e23224 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign21760_e23221 * ((((var_atatsti * var_erfctimesexpmtat_dn8) * var_ktat) - (assign21760_e23224 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign21760_e23229;
        var_gammamax_dn5 = assign21760_e23229_d_n5;
        var_gammamax_dn6 = assign21760_e23229_d_n6;
        var_gammamax_dn7 = assign21760_e23229_d_n7;
        var_gammamax_dn8 = assign21760_e23229_d_n8;

        let (assign21770_e23247, assign21770_e23247_d_n5, assign21770_e23247_d_n6, assign21770_e23247_d_n7, assign21770_e23247_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard386 == 0.0)) {
        let assign21770_e23242: f64 = (var_asrh * var_gammamax);
        let assign21770_e23244: f64 = (assign21770_e23242 * var_wtat);
        let assign21770_e23245: f64 = (p.p846 * assign21770_e23244);
        (assign21770_e23245, (p.p846 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign21770_e23242 * var_wtat_dn5))), (p.p846 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign21770_e23242 * var_wtat_dn6))), (p.p846 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign21770_e23242 * var_wtat_dn7))), (p.p846 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign21770_e23242 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign21770_e23247;
        var_itat_dn5 = assign21770_e23247_d_n5;
        var_itat_dn6 = assign21770_e23247_d_n6;
        var_itat_dn7 = assign21770_e23247_d_n7;
        var_itat_dn8 = assign21770_e23247_d_n8;

        let assign21780_e23250: f64 = if p.p852 == 0.0 { 1.0 } else { 0.0 };
        var_guard392 = assign21780_e23250;

        let (assign21790_e23261, assign21790_e23261_d_n5, assign21790_e23261_d_n6, assign21790_e23261_d_n7, assign21790_e23261_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard392 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign21790_e23261;
        var_ibbt_dn5 = assign21790_e23261_d_n5;
        var_ibbt_dn6 = assign21790_e23261_d_n6;
        var_ibbt_dn7 = assign21790_e23261_d_n7;
        var_ibbt_dn8 = assign21790_e23261_d_n8;

        let assign21800_e23264: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        var_guard393 = assign21800_e23264;

        let (assign21810_e23283, assign21810_e23283_d_n5, assign21810_e23283_d_n6, assign21810_e23283_d_n7, assign21810_e23283_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard392 == 0.0)) && (var_guard393 != 0.0)) {
        let assign21810_e23278: f64 = (p.p829 - var_vbbt);
        let assign21810_e23280: f64 = (assign21810_e23278 * var_vbirstiinv);
        let assign21810_e23281: f64 = (assign21810_e23280).sqrt();
        (assign21810_e23281, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21810_e23283;
        var_tmp_dn5 = assign21810_e23283_d_n5;
        var_tmp_dn6 = assign21810_e23283_d_n6;
        var_tmp_dn7 = assign21810_e23283_d_n7;
        var_tmp_dn8 = assign21810_e23283_d_n8;

        let (assign21820_e23304, assign21820_e23304_d_n5, assign21820_e23304_d_n6, assign21820_e23304_d_n7, assign21820_e23304_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard392 == 0.0)) && (var_guard393 == 0.0)) {
        let assign21820_e23298: f64 = (p.p829 - var_vbbt);
        let assign21820_e23300: f64 = (assign21820_e23298 * var_vbirstiinv);
        let assign21820_e23302: f64 = (assign21820_e23300).powf(p.p832);
        (assign21820_e23302, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21820_e23304;
        var_tmp_dn5 = assign21820_e23304_d_n5;
        var_tmp_dn6 = assign21820_e23304_d_n6;
        var_tmp_dn7 = assign21820_e23304_d_n7;
        var_tmp_dn8 = assign21820_e23304_d_n8;

        let (assign21830_e23324, assign21830_e23324_d_n5, assign21830_e23324_d_n6, assign21830_e23324_d_n7, assign21830_e23324_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard392 == 0.0)) {
        let assign21830_e23317: f64 = (p.p829 - var_vbbt);
        let assign21830_e23319: f64 = (assign21830_e23317 * var_wdepnulrinvsti);
        let assign21830_e23321: f64 = (assign21830_e23319 / var_tmp);
        let assign21830_e23322: f64 = (var_one_over_one_minus_psti * assign21830_e23321);
        (assign21830_e23322, (var_one_over_one_minus_psti * (-((assign21830_e23319 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign21830_e23319 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign21830_e23319 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign21830_e23319 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign21830_e23324;
        var_fmaxr_dn5 = assign21830_e23324_d_n5;
        var_fmaxr_dn6 = assign21830_e23324_d_n6;
        var_fmaxr_dn7 = assign21830_e23324_d_n7;
        var_fmaxr_dn8 = assign21830_e23324_d_n8;

        let assign21840_e23326: f64 = (-var_fbbtsti);
        let assign21840_e23328: f64 = (assign21840_e23326 / var_fmaxr);
        let assign21840_e23329: f64 = (assign21840_e23328).abs();
        let assign21840_e23331: f64 = if assign21840_e23329 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard394 = assign21840_e23331;

        let (assign21850_e23349, assign21850_e23349_d_n5, assign21850_e23349_d_n6, assign21850_e23349_d_n7, assign21850_e23349_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard392 == 0.0)) && (var_guard394 != 0.0)) {
        let assign21850_e23344: f64 = (-var_fbbtsti);
        let assign21850_e23346: f64 = (assign21850_e23344 / var_fmaxr);
        let assign21850_e23347: f64 = (assign21850_e23346).exp();
        (assign21850_e23347, (assign21850_e23347 * (-((assign21850_e23344 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign21850_e23347 * (-((assign21850_e23344 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign21850_e23347 * (-((assign21850_e23344 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign21850_e23347 * (-((assign21850_e23344 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21850_e23349;
        var_tmp_dn5 = assign21850_e23349_d_n5;
        var_tmp_dn6 = assign21850_e23349_d_n6;
        var_tmp_dn7 = assign21850_e23349_d_n7;
        var_tmp_dn8 = assign21850_e23349_d_n8;

        let assign21860_e23351: f64 = (-var_fbbtsti);
        let assign21860_e23353: f64 = (assign21860_e23351 / var_fmaxr);
        let assign21860_e23355: f64 = if assign21860_e23353 < 0.0 { 1.0 } else { 0.0 };
        var_guard395 = assign21860_e23355;

        let (assign21870_e23406, assign21870_e23406_d_n5, assign21870_e23406_d_n6, assign21870_e23406_d_n7, assign21870_e23406_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard392 == 0.0)) && (var_guard394 == 0.0)) && (var_guard395 != 0.0)) {
        let assign21870_e23373: f64 = (-230.25850929940458);
        let assign21870_e23375: f64 = (-var_fbbtsti);
        let assign21870_e23377: f64 = (assign21870_e23375 / var_fmaxr);
        let assign21870_e23378: f64 = (assign21870_e23373 - assign21870_e23377);
        let assign21870_e23382: f64 = (-230.25850929940458);
        let assign21870_e23384: f64 = (-var_fbbtsti);
        let assign21870_e23386: f64 = (assign21870_e23384 / var_fmaxr);
        let assign21870_e23387: f64 = (assign21870_e23382 - assign21870_e23386);
        let assign21870_e23390: f64 = (-230.25850929940458);
        let assign21870_e23392: f64 = (-var_fbbtsti);
        let assign21870_e23394: f64 = (assign21870_e23392 / var_fmaxr);
        let assign21870_e23395: f64 = (assign21870_e23390 - assign21870_e23394);
        let assign21870_e23397: f64 = (assign21870_e23395 * 0.3333333333333333);
        let assign21870_e23398: f64 = (1.0 + assign21870_e23397);
        let assign21870_e23399: f64 = (assign21870_e23387 * assign21870_e23398);
        let assign21870_e23400: f64 = (0.5 * assign21870_e23399);
        let assign21870_e23401: f64 = (1.0 + assign21870_e23400);
        let assign21870_e23402: f64 = (assign21870_e23378 * assign21870_e23401);
        let assign21870_e23403: f64 = (1.0 + assign21870_e23402);
        let assign21870_e23404: f64 = (1e-100 / assign21870_e23403);
        (assign21870_e23404, (-((1e-100 * (((-(-((assign21870_e23375 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign21870_e23401) + (assign21870_e23378 * (0.5 * (((-(-((assign21870_e23384 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign21870_e23398) + (assign21870_e23387 * ((-(-((assign21870_e23392 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign21870_e23403 * assign21870_e23403))), (-((1e-100 * (((-(-((assign21870_e23375 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign21870_e23401) + (assign21870_e23378 * (0.5 * (((-(-((assign21870_e23384 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign21870_e23398) + (assign21870_e23387 * ((-(-((assign21870_e23392 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign21870_e23403 * assign21870_e23403))), (-((1e-100 * (((-(-((assign21870_e23375 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign21870_e23401) + (assign21870_e23378 * (0.5 * (((-(-((assign21870_e23384 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign21870_e23398) + (assign21870_e23387 * ((-(-((assign21870_e23392 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign21870_e23403 * assign21870_e23403))), (-((1e-100 * (((-(-((assign21870_e23375 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign21870_e23401) + (assign21870_e23378 * (0.5 * (((-(-((assign21870_e23384 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign21870_e23398) + (assign21870_e23387 * ((-(-((assign21870_e23392 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign21870_e23403 * assign21870_e23403))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21870_e23406;
        var_tmp_dn5 = assign21870_e23406_d_n5;
        var_tmp_dn6 = assign21870_e23406_d_n6;
        var_tmp_dn7 = assign21870_e23406_d_n7;
        var_tmp_dn8 = assign21870_e23406_d_n8;

        let (assign21880_e23455, assign21880_e23455_d_n5, assign21880_e23455_d_n6, assign21880_e23455_d_n7, assign21880_e23455_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard392 == 0.0)) && (var_guard394 == 0.0)) && (var_guard395 == 0.0)) {
        let assign21880_e23425: f64 = (-var_fbbtsti);
        let assign21880_e23427: f64 = (assign21880_e23425 / var_fmaxr);
        let assign21880_e23429: f64 = (assign21880_e23427 - 230.25850929940458);
        let assign21880_e23433: f64 = (-var_fbbtsti);
        let assign21880_e23435: f64 = (assign21880_e23433 / var_fmaxr);
        let assign21880_e23437: f64 = (assign21880_e23435 - 230.25850929940458);
        let assign21880_e23440: f64 = (-var_fbbtsti);
        let assign21880_e23442: f64 = (assign21880_e23440 / var_fmaxr);
        let assign21880_e23444: f64 = (assign21880_e23442 - 230.25850929940458);
        let assign21880_e23446: f64 = (assign21880_e23444 * 0.3333333333333333);
        let assign21880_e23447: f64 = (1.0 + assign21880_e23446);
        let assign21880_e23448: f64 = (assign21880_e23437 * assign21880_e23447);
        let assign21880_e23449: f64 = (0.5 * assign21880_e23448);
        let assign21880_e23450: f64 = (1.0 + assign21880_e23449);
        let assign21880_e23451: f64 = (assign21880_e23429 * assign21880_e23450);
        let assign21880_e23452: f64 = (1.0 + assign21880_e23451);
        let assign21880_e23453: f64 = (1e100 * assign21880_e23452);
        (assign21880_e23453, (1e100 * (((-((assign21880_e23425 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign21880_e23450) + (assign21880_e23429 * (0.5 * (((-((assign21880_e23433 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign21880_e23447) + (assign21880_e23437 * ((-((assign21880_e23440 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign21880_e23425 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign21880_e23450) + (assign21880_e23429 * (0.5 * (((-((assign21880_e23433 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign21880_e23447) + (assign21880_e23437 * ((-((assign21880_e23440 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign21880_e23425 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign21880_e23450) + (assign21880_e23429 * (0.5 * (((-((assign21880_e23433 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign21880_e23447) + (assign21880_e23437 * ((-((assign21880_e23440 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign21880_e23425 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign21880_e23450) + (assign21880_e23429 * (0.5 * (((-((assign21880_e23433 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign21880_e23447) + (assign21880_e23437 * ((-((assign21880_e23440 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21880_e23455;
        var_tmp_dn5 = assign21880_e23455_d_n5;
        var_tmp_dn6 = assign21880_e23455_d_n6;
        var_tmp_dn7 = assign21880_e23455_d_n7;
        var_tmp_dn8 = assign21880_e23455_d_n8;

        let (assign21890_e23475, assign21890_e23475_d_n5, assign21890_e23475_d_n6, assign21890_e23475_d_n7, assign21890_e23475_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard392 == 0.0)) {
        let assign21890_e23468: f64 = (var_v3 * var_fmaxr);
        let assign21890_e23470: f64 = (assign21890_e23468 * var_fmaxr);
        let assign21890_e23472: f64 = (assign21890_e23470 * var_tmp);
        let assign21890_e23473: f64 = (p.p852 * assign21890_e23472);
        (assign21890_e23473, (p.p852 * (((((var_v3 * var_fmaxr_dn5) * var_fmaxr) + (assign21890_e23468 * var_fmaxr_dn5)) * var_tmp) + (assign21890_e23470 * var_tmp_dn5))), (p.p852 * (((((var_v3 * var_fmaxr_dn6) * var_fmaxr) + (assign21890_e23468 * var_fmaxr_dn6)) * var_tmp) + (assign21890_e23470 * var_tmp_dn6))), (p.p852 * (((((var_v3 * var_fmaxr_dn7) * var_fmaxr) + (assign21890_e23468 * var_fmaxr_dn7)) * var_tmp) + (assign21890_e23470 * var_tmp_dn7))), (p.p852 * (((((var_v3 * var_fmaxr_dn8) * var_fmaxr) + (assign21890_e23468 * var_fmaxr_dn8)) * var_tmp) + (assign21890_e23470 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign21890_e23475;
        var_ibbt_dn5 = assign21890_e23475_d_n5;
        var_ibbt_dn6 = assign21890_e23475_d_n6;
        var_ibbt_dn7 = assign21890_e23475_d_n7;
        var_ibbt_dn8 = assign21890_e23475_d_n8;

        let assign21900_e23478: f64 = if p.p861 > 1000.0 { 1.0 } else { 0.0 };
        var_guard396 = assign21900_e23478;

        let (assign21910_e23489, assign21910_e23489_d_n5, assign21910_e23489_d_n6, assign21910_e23489_d_n7, assign21910_e23489_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard396 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign21910_e23489;
        var_fbreakdown_dn5 = assign21910_e23489_d_n5;
        var_fbreakdown_dn6 = assign21910_e23489_d_n6;
        var_fbreakdown_dn7 = assign21910_e23489_d_n7;
        var_fbreakdown_dn8 = assign21910_e23489_d_n8;

        let assign21920_e23492: f64 = (-var_alphaav);
        let assign21920_e23494: f64 = (assign21920_e23492 * p.p861);
        let assign21920_e23495: f64 = if var_vav > assign21920_e23494 { 1.0 } else { 0.0 };
        var_guard397 = assign21920_e23495;

        let assign21930_e23498: f64 = if p.p864 == 4.0 { 1.0 } else { 0.0 };
        var_guard398 = assign21930_e23498;

        let (assign21940_e23528, assign21940_e23528_d_n5, assign21940_e23528_d_n6, assign21940_e23528_d_n7, assign21940_e23528_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard396 == 0.0)) && (var_guard397 != 0.0)) && (var_guard398 != 0.0)) {
        let assign21940_e23514: f64 = (var_vav * var_vbrinvsti);
        let assign21940_e23517: f64 = (var_vav * var_vbrinvsti);
        let assign21940_e23518: f64 = (assign21940_e23514 * assign21940_e23517);
        let assign21940_e23521: f64 = (var_vav * var_vbrinvsti);
        let assign21940_e23522: f64 = (assign21940_e23518 * assign21940_e23521);
        let assign21940_e23525: f64 = (var_vav * var_vbrinvsti);
        let assign21940_e23526: f64 = (assign21940_e23522 * assign21940_e23525);
        (assign21940_e23526, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21940_e23528;
        var_tmp_dn5 = assign21940_e23528_d_n5;
        var_tmp_dn6 = assign21940_e23528_d_n6;
        var_tmp_dn7 = assign21940_e23528_d_n7;
        var_tmp_dn8 = assign21940_e23528_d_n8;

        let (assign21950_e23550, assign21950_e23550_d_n5, assign21950_e23550_d_n6, assign21950_e23550_d_n7, assign21950_e23550_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard396 == 0.0)) && (var_guard397 != 0.0)) && (var_guard398 == 0.0)) {
        let assign21950_e23545: f64 = (var_vav * var_vbrinvsti);
        let assign21950_e23546: f64 = (assign21950_e23545).abs();
        let assign21950_e23548: f64 = (assign21950_e23546).powf(p.p864);
        (assign21950_e23548, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign21950_e23550;
        var_tmp_dn5 = assign21950_e23550_d_n5;
        var_tmp_dn6 = assign21950_e23550_d_n6;
        var_tmp_dn7 = assign21950_e23550_d_n7;
        var_tmp_dn8 = assign21950_e23550_d_n8;

        let (assign21960_e23568, assign21960_e23568_d_n5, assign21960_e23568_d_n6, assign21960_e23568_d_n7, assign21960_e23568_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard396 == 0.0)) && (var_guard397 != 0.0)) {
        let assign21960_e23565: f64 = (1.0 - var_tmp);
        let assign21960_e23566: f64 = (1.0 / assign21960_e23565);
        (assign21960_e23566, (-((-var_tmp_dn5) / (assign21960_e23565 * assign21960_e23565))), (-((-var_tmp_dn6) / (assign21960_e23565 * assign21960_e23565))), (-((-var_tmp_dn7) / (assign21960_e23565 * assign21960_e23565))), (-((-var_tmp_dn8) / (assign21960_e23565 * assign21960_e23565))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign21960_e23568;
        var_fbreakdown_dn5 = assign21960_e23568_d_n5;
        var_fbreakdown_dn6 = assign21960_e23568_d_n6;
        var_fbreakdown_dn7 = assign21960_e23568_d_n7;
        var_fbreakdown_dn8 = assign21960_e23568_d_n8;

        let (assign21970_e23591, assign21970_e23591_d_n5, assign21970_e23591_d_n6, assign21970_e23591_d_n7, assign21970_e23591_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) && (var_guard396 == 0.0)) && (var_guard397 == 0.0)) {
        let assign21970_e23585: f64 = (var_alphaav * p.p861);
        let assign21970_e23586: f64 = (var_vav + assign21970_e23585);
        let assign21970_e23588: f64 = (assign21970_e23586 * var_slopesti);
        let assign21970_e23589: f64 = (var_fstopsti + assign21970_e23588);
        (assign21970_e23589, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign21970_e23591;
        var_fbreakdown_dn5 = assign21970_e23591_d_n5;
        var_fbreakdown_dn6 = assign21970_e23591_d_n6;
        var_fbreakdown_dn7 = assign21970_e23591_d_n7;
        var_fbreakdown_dn8 = assign21970_e23591_d_n8;

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
        *var_guard388_slot = var_guard388;
        *var_guard389_slot = var_guard389;
        *var_guard390_slot = var_guard390;
        *var_guard391_slot = var_guard391;
        *var_guard392_slot = var_guard392;
        *var_guard393_slot = var_guard393;
        *var_guard394_slot = var_guard394;
        *var_guard395_slot = var_guard395;
        *var_guard396_slot = var_guard396;
        *var_guard397_slot = var_guard397;
        *var_guard398_slot = var_guard398;
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

    pub(super) fn stamp_transient_block_39(
        p: &Parameters,
        var_atatgat: f64,
        var_btatpartgat: f64,
        var_fbreakdown: f64,
        var_fbreakdown_dn5: f64,
        var_fbreakdown_dn6: f64,
        var_fbreakdown_dn7: f64,
        var_fbreakdown_dn8: f64,
        var_ftdgat: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard382: f64,
        var_ibbt: f64,
        var_ibbt_dn5: f64,
        var_ibbt_dn6: f64,
        var_ibbt_dn7: f64,
        var_ibbt_dn8: f64,
        var_idmult: f64,
        var_idsatgat: f64,
        var_lgsource_i: f64,
        var_one_minus_pgat: f64,
        var_one_over_one_minus_pgat: f64,
        var_perfc: f64,
        var_two_psistar: f64,
        var_vbigat: f64,
        var_vbirgatinv: f64,
        var_vjsrh: f64,
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
        var_guard399_slot: &mut f64,
        var_guard400_slot: &mut f64,
        var_guard401_slot: &mut f64,
        var_guard402_slot: &mut f64,
        var_guard403_slot: &mut f64,
        var_guard404_slot: &mut f64,
        var_guard405_slot: &mut f64,
        var_guard406_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
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
        let mut var_guard399: f64 = *var_guard399_slot;
        let mut var_guard400: f64 = *var_guard400_slot;
        let mut var_guard401: f64 = *var_guard401_slot;
        let mut var_guard402: f64 = *var_guard402_slot;
        let mut var_guard403: f64 = *var_guard403_slot;
        let mut var_guard404: f64 = *var_guard404_slot;
        let mut var_guard405: f64 = *var_guard405_slot;
        let mut var_guard406: f64 = *var_guard406_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
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

        let (assign21980_e23610, assign21980_e23610_d_n5, assign21980_e23610_d_n6, assign21980_e23610_d_n7, assign21980_e23610_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard382 == 0.0)) {
        let assign21980_e23601: f64 = (var_id__blk219 + var_isrh);
        let assign21980_e23603: f64 = (assign21980_e23601 + var_itat);
        let assign21980_e23605: f64 = (assign21980_e23603 + var_ibbt);
        let assign21980_e23606: f64 = (p.p29 * assign21980_e23605);
        let assign21980_e23608: f64 = (assign21980_e23606 * var_fbreakdown);
        (assign21980_e23608, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign21980_e23606 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign21980_e23606 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign21980_e23606 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign21980_e23606 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign21980_e23610;
        var_ijunsti_dn5 = assign21980_e23610_d_n5;
        var_ijunsti_dn6 = assign21980_e23610_d_n6;
        var_ijunsti_dn7 = assign21980_e23610_d_n7;
        var_ijunsti_dn8 = assign21980_e23610_d_n8;

        let assign21990_e23613: f64 = if var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard399 = assign21990_e23613;

        let (assign22000_e23621, assign22000_e23621_d_n5, assign22000_e23621_d_n6, assign22000_e23621_d_n7, assign22000_e23621_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign22000_e23621;
        var_ijungat_dn5 = assign22000_e23621_d_n5;
        var_ijungat_dn6 = assign22000_e23621_d_n6;
        var_ijungat_dn7 = assign22000_e23621_d_n7;
        var_ijungat_dn8 = assign22000_e23621_d_n8;

        let (assign22010_e23632,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) {
        let assign22010_e23630: f64 = (var_idsatgat * var_idmult);
        (assign22010_e23630,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign22010_e23632;

        let assign22020_e23639: f64 = if ((p.p842 == 0.0) && (p.p847 == 0.0)) { 1.0 } else { 0.0 };
        var_guard400 = assign22020_e23639;

        let (assign22030_e23650, assign22030_e23650_d_n5, assign22030_e23650_d_n6, assign22030_e23650_d_n7, assign22030_e23650_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard400 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign22030_e23650;
        var_isrh_dn5 = assign22030_e23650_d_n5;
        var_isrh_dn6 = assign22030_e23650_d_n6;
        var_isrh_dn7 = assign22030_e23650_d_n7;
        var_isrh_dn8 = assign22030_e23650_d_n8;

        let (assign22040_e23664,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard400 == 0.0)) {
        let assign22040_e23662: f64 = (var_vbigat - var_vjsrh);
        (assign22040_e23662,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign22040_e23664;

        let (assign22050_e23683,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard400 == 0.0)) {
        let assign22050_e23678: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign22050_e23679: f64 = (1.0 - assign22050_e23678);
        let assign22050_e23680: f64 = (assign22050_e23679).sqrt();
        let assign22050_e23681: f64 = (1.0 - assign22050_e23680);
        (assign22050_e23681,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign22050_e23683;

        let assign22060_e23686: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        var_guard401 = assign22060_e23686;

        let (assign22070_e23700,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard400 == 0.0)) && (var_guard401 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign22070_e23700;

        let (assign22080_e23732,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard400 == 0.0)) && (var_guard401 == 0.0)) {
        let assign22080_e23715: f64 = (var_wsrhstep * var_wsrhstep);
        let assign22080_e23717: f64 = (var_wsrhstep).ln();
        let assign22080_e23718: f64 = (assign22080_e23715 * assign22080_e23717);
        let assign22080_e23721: f64 = (1.0 - var_wsrhstep);
        let assign22080_e23722: f64 = (assign22080_e23718 / assign22080_e23721);
        let assign22080_e23724: f64 = (assign22080_e23722 + var_wsrhstep);
        let assign22080_e23728: f64 = (2.0 * p.p833);
        let assign22080_e23729: f64 = (1.0 - assign22080_e23728);
        let assign22080_e23730: f64 = (assign22080_e23724 * assign22080_e23729);
        (assign22080_e23730,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign22080_e23732;

        let (assign22090_e23746,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard400 == 0.0)) {
        let assign22090_e23744: f64 = (var_wsrhstep + var_dwsrh);
        (assign22090_e23744,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign22090_e23746;

        let assign22100_e23749: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        var_guard402 = assign22100_e23749;

        let (assign22110_e23766, assign22110_e23766_d_n5, assign22110_e23766_d_n6, assign22110_e23766_d_n7, assign22110_e23766_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard400 == 0.0)) && (var_guard402 != 0.0)) {
        let assign22110_e23763: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign22110_e23764: f64 = (assign22110_e23763).sqrt();
        (assign22110_e23764, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22110_e23766;
        var_tmp_dn5 = assign22110_e23766_d_n5;
        var_tmp_dn6 = assign22110_e23766_d_n6;
        var_tmp_dn7 = assign22110_e23766_d_n7;
        var_tmp_dn8 = assign22110_e23766_d_n8;

        let (assign22120_e23785, assign22120_e23785_d_n5, assign22120_e23785_d_n6, assign22120_e23785_d_n7, assign22120_e23785_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard400 == 0.0)) && (var_guard402 == 0.0)) {
        let assign22120_e23781: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign22120_e23783: f64 = (assign22120_e23781).powf(p.p833);
        (assign22120_e23783, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22120_e23785;
        var_tmp_dn5 = assign22120_e23785_d_n5;
        var_tmp_dn6 = assign22120_e23785_d_n6;
        var_tmp_dn7 = assign22120_e23785_d_n7;
        var_tmp_dn8 = assign22120_e23785_d_n8;

        let (assign22130_e23799, assign22130_e23799_d_n5, assign22130_e23799_d_n6, assign22130_e23799_d_n7, assign22130_e23799_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard400 == 0.0)) {
        let assign22130_e23797: f64 = (var_wdepnulrgat * var_tmp);
        (assign22130_e23797, (var_wdepnulrgat * var_tmp_dn5), (var_wdepnulrgat * var_tmp_dn6), (var_wdepnulrgat * var_tmp_dn7), (var_wdepnulrgat * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign22130_e23799;
        var_wdep_dn5 = assign22130_e23799_d_n5;
        var_wdep_dn6 = assign22130_e23799_d_n6;
        var_wdep_dn7 = assign22130_e23799_d_n7;
        var_wdep_dn8 = assign22130_e23799_d_n8;

        let (assign22140_e23817, assign22140_e23817_d_n5, assign22140_e23817_d_n6, assign22140_e23817_d_n7, assign22140_e23817_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard400 == 0.0)) {
        let assign22140_e23812: f64 = (var_zinv - 1.0);
        let assign22140_e23814: f64 = (assign22140_e23812 * var_wdep);
        let assign22140_e23815: f64 = (var_ftdgat * assign22140_e23814);
        (assign22140_e23815, (var_ftdgat * (assign22140_e23812 * var_wdep_dn5)), (var_ftdgat * (assign22140_e23812 * var_wdep_dn6)), (var_ftdgat * (assign22140_e23812 * var_wdep_dn7)), (var_ftdgat * (assign22140_e23812 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign22140_e23817;
        var_asrh_dn5 = assign22140_e23817_d_n5;
        var_asrh_dn6 = assign22140_e23817_d_n6;
        var_asrh_dn7 = assign22140_e23817_d_n7;
        var_asrh_dn8 = assign22140_e23817_d_n8;

        let (assign22150_e23833, assign22150_e23833_d_n5, assign22150_e23833_d_n6, assign22150_e23833_d_n7, assign22150_e23833_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard400 == 0.0)) {
        let assign22150_e23830: f64 = (var_asrh * var_wsrh);
        let assign22150_e23831: f64 = (p.p842 * assign22150_e23830);
        (assign22150_e23831, (p.p842 * (var_asrh_dn5 * var_wsrh)), (p.p842 * (var_asrh_dn6 * var_wsrh)), (p.p842 * (var_asrh_dn7 * var_wsrh)), (p.p842 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign22150_e23833;
        var_isrh_dn5 = assign22150_e23833_d_n5;
        var_isrh_dn6 = assign22150_e23833_d_n6;
        var_isrh_dn7 = assign22150_e23833_d_n7;
        var_isrh_dn8 = assign22150_e23833_d_n8;

        let assign22160_e23836: f64 = if p.p847 == 0.0 { 1.0 } else { 0.0 };
        var_guard403 = assign22160_e23836;

        let (assign22170_e23847, assign22170_e23847_d_n5, assign22170_e23847_d_n6, assign22170_e23847_d_n7, assign22170_e23847_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign22170_e23847;
        var_itat_dn5 = assign22170_e23847_d_n5;
        var_itat_dn6 = assign22170_e23847_d_n6;
        var_itat_dn7 = assign22170_e23847_d_n7;
        var_itat_dn8 = assign22170_e23847_d_n8;

        let (assign22180_e23865, assign22180_e23865_d_n5, assign22180_e23865_d_n6, assign22180_e23865_d_n7, assign22180_e23865_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) {
        let assign22180_e23860: f64 = (var_wdep * var_one_minus_pgat);
        let assign22180_e23862: f64 = (assign22180_e23860 / var_vbi_minus_vjsrh);
        let assign22180_e23863: f64 = (var_btatpartgat * assign22180_e23862);
        (assign22180_e23863, (var_btatpartgat * ((var_wdep_dn5 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn6 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn7 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn8 * var_one_minus_pgat) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign22180_e23865;
        var_btat_dn5 = assign22180_e23865_d_n5;
        var_btat_dn6 = assign22180_e23865_d_n6;
        var_btat_dn7 = assign22180_e23865_d_n7;
        var_btat_dn8 = assign22180_e23865_d_n8;

        let (assign22190_e23881, assign22190_e23881_d_n5, assign22190_e23881_d_n6, assign22190_e23881_d_n7, assign22190_e23881_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) {
        let assign22190_e23877: f64 = (0.666666666666667 * var_atatgat);
        let assign22190_e23879: f64 = (assign22190_e23877 / var_btat);
        (assign22190_e23879, (-((assign22190_e23877 * var_btat_dn5) / (var_btat * var_btat))), (-((assign22190_e23877 * var_btat_dn6) / (var_btat * var_btat))), (-((assign22190_e23877 * var_btat_dn7) / (var_btat * var_btat))), (-((assign22190_e23877 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign22190_e23881;
        var_twoatatoverthreebtat_dn5 = assign22190_e23881_d_n5;
        var_twoatatoverthreebtat_dn6 = assign22190_e23881_d_n6;
        var_twoatatoverthreebtat_dn7 = assign22190_e23881_d_n7;
        var_twoatatoverthreebtat_dn8 = assign22190_e23881_d_n8;

        let (assign22200_e23895, assign22200_e23895_d_n5, assign22200_e23895_d_n6, assign22200_e23895_d_n7, assign22200_e23895_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) {
        let assign22200_e23893: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign22200_e23893, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign22200_e23895;
        var_umaxbeforelimiting_dn5 = assign22200_e23895_d_n5;
        var_umaxbeforelimiting_dn6 = assign22200_e23895_d_n6;
        var_umaxbeforelimiting_dn7 = assign22200_e23895_d_n7;
        var_umaxbeforelimiting_dn8 = assign22200_e23895_d_n8;

        let (assign22210_e23916, assign22210_e23916_d_n5, assign22210_e23916_d_n6, assign22210_e23916_d_n7, assign22210_e23916_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) {
        let assign22210_e23907: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign22210_e23910: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign22210_e23912: f64 = (assign22210_e23910 + 1.0);
        let assign22210_e23913: f64 = (assign22210_e23907 / assign22210_e23912);
        let assign22210_e23914: f64 = (assign22210_e23913).sqrt();
        (assign22210_e23914, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign22210_e23912) - (assign22210_e23907 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign22210_e23912 * assign22210_e23912)) / (2.0 * assign22210_e23914)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign22210_e23912) - (assign22210_e23907 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign22210_e23912 * assign22210_e23912)) / (2.0 * assign22210_e23914)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign22210_e23912) - (assign22210_e23907 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign22210_e23912 * assign22210_e23912)) / (2.0 * assign22210_e23914)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign22210_e23912) - (assign22210_e23907 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign22210_e23912 * assign22210_e23912)) / (2.0 * assign22210_e23914)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign22210_e23916;
        var_umax_dn5 = assign22210_e23916_d_n5;
        var_umax_dn6 = assign22210_e23916_d_n6;
        var_umax_dn7 = assign22210_e23916_d_n7;
        var_umax_dn8 = assign22210_e23916_d_n8;

        let (assign22220_e23929, assign22220_e23929_d_n5, assign22220_e23929_d_n6, assign22220_e23929_d_n7, assign22220_e23929_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) {
        let assign22220_e23927: f64 = (var_umax).sqrt();
        (assign22220_e23927, (var_umax_dn5 / (2.0 * assign22220_e23927)), (var_umax_dn6 / (2.0 * assign22220_e23927)), (var_umax_dn7 / (2.0 * assign22220_e23927)), (var_umax_dn8 / (2.0 * assign22220_e23927)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign22220_e23929;
        var_sqrtumax_dn5 = assign22220_e23929_d_n5;
        var_sqrtumax_dn6 = assign22220_e23929_d_n6;
        var_sqrtumax_dn7 = assign22220_e23929_d_n7;
        var_sqrtumax_dn8 = assign22220_e23929_d_n8;

        let (assign22230_e23943, assign22230_e23943_d_n5, assign22230_e23943_d_n6, assign22230_e23943_d_n7, assign22230_e23943_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) {
        let assign22230_e23941: f64 = (var_umax * var_sqrtumax);
        (assign22230_e23941, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign22230_e23943;
        var_umaxpoweronepointfive_dn5 = assign22230_e23943_d_n5;
        var_umaxpoweronepointfive_dn6 = assign22230_e23943_d_n6;
        var_umaxpoweronepointfive_dn7 = assign22230_e23943_d_n7;
        var_umaxpoweronepointfive_dn8 = assign22230_e23943_d_n8;

        let assign22240_e23945: f64 = (-p.p833);
        let assign22240_e23947: f64 = (assign22240_e23945 * var_one_over_one_minus_pgat);
        let assign22240_e23949: f64 = (-1.0);
        let assign22240_e23950: f64 = if assign22240_e23947 == assign22240_e23949 { 1.0 } else { 0.0 };
        var_guard404 = assign22240_e23950;

        let (assign22250_e23970, assign22250_e23970_d_n5, assign22250_e23970_d_n6, assign22250_e23970_d_n7, assign22250_e23970_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) && (var_guard404 != 0.0)) {
        let assign22250_e23966: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign22250_e23967: f64 = (1.0 + assign22250_e23966);
        let assign22250_e23968: f64 = (1.0 / assign22250_e23967);
        (assign22250_e23968, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign22250_e23967 * assign22250_e23967))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign22250_e23967 * assign22250_e23967))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign22250_e23967 * assign22250_e23967))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign22250_e23967 * assign22250_e23967))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign22250_e23970;
        var_wgamma_dn5 = assign22250_e23970_d_n5;
        var_wgamma_dn6 = assign22250_e23970_d_n6;
        var_wgamma_dn7 = assign22250_e23970_d_n7;
        var_wgamma_dn8 = assign22250_e23970_d_n8;

        let (assign22260_e23994, assign22260_e23994_d_n5, assign22260_e23994_d_n6, assign22260_e23994_d_n7, assign22260_e23994_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) && (var_guard404 == 0.0)) {
        let assign22260_e23986: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign22260_e23987: f64 = (1.0 + assign22260_e23986);
        let assign22260_e23989: f64 = (-p.p833);
        let assign22260_e23991: f64 = (assign22260_e23989 * var_one_over_one_minus_pgat);
        let assign22260_e23992: f64 = (assign22260_e23987).powf(assign22260_e23991);
        (assign22260_e23992, if 0.0 == 0.0 && ((assign22260_e23991) as f64).is_finite() && ((assign22260_e23991) as f64).fract() == 0.0 { if assign22260_e23991 == 0.0 { 0.0 } else { (assign22260_e23991 * ((assign22260_e23987).powf(assign22260_e23991 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign22260_e23992 * (assign22260_e23991 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign22260_e23987))) }, if 0.0 == 0.0 && ((assign22260_e23991) as f64).is_finite() && ((assign22260_e23991) as f64).fract() == 0.0 { if assign22260_e23991 == 0.0 { 0.0 } else { (assign22260_e23991 * ((assign22260_e23987).powf(assign22260_e23991 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign22260_e23992 * (assign22260_e23991 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign22260_e23987))) }, if 0.0 == 0.0 && ((assign22260_e23991) as f64).is_finite() && ((assign22260_e23991) as f64).fract() == 0.0 { if assign22260_e23991 == 0.0 { 0.0 } else { (assign22260_e23991 * ((assign22260_e23987).powf(assign22260_e23991 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign22260_e23992 * (assign22260_e23991 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign22260_e23987))) }, if 0.0 == 0.0 && ((assign22260_e23991) as f64).is_finite() && ((assign22260_e23991) as f64).fract() == 0.0 { if assign22260_e23991 == 0.0 { 0.0 } else { (assign22260_e23991 * ((assign22260_e23987).powf(assign22260_e23991 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign22260_e23992 * (assign22260_e23991 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign22260_e23987))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign22260_e23994;
        var_wgamma_dn5 = assign22260_e23994_d_n5;
        var_wgamma_dn6 = assign22260_e23994_d_n6;
        var_wgamma_dn7 = assign22260_e23994_d_n7;
        var_wgamma_dn8 = assign22260_e23994_d_n8;

        let (assign22270_e24012, assign22270_e24012_d_n5, assign22270_e24012_d_n6, assign22270_e24012_d_n7, assign22270_e24012_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) {
        let assign22270_e24006: f64 = (var_wsrh * var_wgamma);
        let assign22270_e24009: f64 = (var_wsrh + var_wgamma);
        let assign22270_e24010: f64 = (assign22270_e24006 / assign22270_e24009);
        (assign22270_e24010, ((((var_wsrh * var_wgamma_dn5) * assign22270_e24009) - (assign22270_e24006 * var_wgamma_dn5)) / (assign22270_e24009 * assign22270_e24009)), ((((var_wsrh * var_wgamma_dn6) * assign22270_e24009) - (assign22270_e24006 * var_wgamma_dn6)) / (assign22270_e24009 * assign22270_e24009)), ((((var_wsrh * var_wgamma_dn7) * assign22270_e24009) - (assign22270_e24006 * var_wgamma_dn7)) / (assign22270_e24009 * assign22270_e24009)), ((((var_wsrh * var_wgamma_dn8) * assign22270_e24009) - (assign22270_e24006 * var_wgamma_dn8)) / (assign22270_e24009 * assign22270_e24009)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign22270_e24012;
        var_wtat_dn5 = assign22270_e24012_d_n5;
        var_wtat_dn6 = assign22270_e24012_d_n6;
        var_wtat_dn7 = assign22270_e24012_d_n7;
        var_wtat_dn8 = assign22270_e24012_d_n8;

        let (assign22280_e24029, assign22280_e24029_d_n5, assign22280_e24029_d_n6, assign22280_e24029_d_n7, assign22280_e24029_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) {
        let assign22280_e24025: f64 = (var_btat / var_sqrtumax);
        let assign22280_e24026: f64 = (0.375 * assign22280_e24025);
        let assign22280_e24027: f64 = (assign22280_e24026).sqrt();
        (assign22280_e24027, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign22280_e24027)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign22280_e24027)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign22280_e24027)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign22280_e24027)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign22280_e24029;
        var_ktat_dn5 = assign22280_e24029_d_n5;
        var_ktat_dn6 = assign22280_e24029_d_n6;
        var_ktat_dn7 = assign22280_e24029_d_n7;
        var_ktat_dn8 = assign22280_e24029_d_n8;

        let (assign22290_e24047, assign22290_e24047_d_n5, assign22290_e24047_d_n6, assign22290_e24047_d_n7, assign22290_e24047_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) {
        let assign22290_e24042: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign22290_e24043: f64 = (2.0 * assign22290_e24042);
        let assign22290_e24045: f64 = (assign22290_e24043 - var_umax);
        (assign22290_e24045, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign22290_e24047;
        var_ltat_dn5 = assign22290_e24047_d_n5;
        var_ltat_dn6 = assign22290_e24047_d_n6;
        var_ltat_dn7 = assign22290_e24047_d_n7;
        var_ltat_dn8 = assign22290_e24047_d_n8;

        let (assign22300_e24073, assign22300_e24073_d_n5, assign22300_e24073_d_n6, assign22300_e24073_d_n7, assign22300_e24073_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) {
        let assign22300_e24059: f64 = (var_atatgat * var_twoatatoverthreebtat);
        let assign22300_e24061: f64 = (assign22300_e24059 * var_sqrtumax);
        let assign22300_e24064: f64 = (var_atatgat * var_umax);
        let assign22300_e24065: f64 = (assign22300_e24061 - assign22300_e24064);
        let assign22300_e24069: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign22300_e24070: f64 = (0.5 * assign22300_e24069);
        let assign22300_e24071: f64 = (assign22300_e24065 + assign22300_e24070);
        (assign22300_e24071, (((((var_atatgat * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign22300_e24059 * var_sqrtumax_dn5)) - (var_atatgat * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatgat * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign22300_e24059 * var_sqrtumax_dn6)) - (var_atatgat * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign22300_e24059 * var_sqrtumax_dn7)) - (var_atatgat * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign22300_e24059 * var_sqrtumax_dn8)) - (var_atatgat * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign22300_e24073;
        var_mtat_dn5 = assign22300_e24073_d_n5;
        var_mtat_dn6 = assign22300_e24073_d_n6;
        var_mtat_dn7 = assign22300_e24073_d_n7;
        var_mtat_dn8 = assign22300_e24073_d_n8;

        let (assign22310_e24089, assign22310_e24089_d_n5, assign22310_e24089_d_n6, assign22310_e24089_d_n7, assign22310_e24089_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) {
        let assign22310_e24085: f64 = (var_ltat - 1.0);
        let assign22310_e24087: f64 = (assign22310_e24085 * var_ktat);
        (assign22310_e24087, ((var_ltat_dn5 * var_ktat) + (assign22310_e24085 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign22310_e24085 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign22310_e24085 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign22310_e24085 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign22310_e24089;
        var_xerfc_dn5 = assign22310_e24089_d_n5;
        var_xerfc_dn6 = assign22310_e24089_d_n6;
        var_xerfc_dn7 = assign22310_e24089_d_n7;
        var_xerfc_dn8 = assign22310_e24089_d_n8;

        let (assign22320_e24103, assign22320_e24103_d_n5, assign22320_e24103_d_n6, assign22320_e24103_d_n7, assign22320_e24103_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) {
        let assign22320_e24101: f64 = (var_xerfc * var_xerfc);
        (assign22320_e24101, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign22320_e24103;
        var_ysq_dn5 = assign22320_e24103_d_n5;
        var_ysq_dn6 = assign22320_e24103_d_n6;
        var_ysq_dn7 = assign22320_e24103_d_n7;
        var_ysq_dn8 = assign22320_e24103_d_n8;

        let assign22330_e24106: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard405 = assign22330_e24106;

        let (assign22340_e24126, assign22340_e24126_d_n5, assign22340_e24126_d_n6, assign22340_e24126_d_n7, assign22340_e24126_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) && (var_guard405 != 0.0)) {
        let assign22340_e24122: f64 = (var_perfc * var_xerfc);
        let assign22340_e24123: f64 = (1.0 + assign22340_e24122);
        let assign22340_e24124: f64 = (1.0 / assign22340_e24123);
        (assign22340_e24124, (-((var_perfc * var_xerfc_dn5) / (assign22340_e24123 * assign22340_e24123))), (-((var_perfc * var_xerfc_dn6) / (assign22340_e24123 * assign22340_e24123))), (-((var_perfc * var_xerfc_dn7) / (assign22340_e24123 * assign22340_e24123))), (-((var_perfc * var_xerfc_dn8) / (assign22340_e24123 * assign22340_e24123))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign22340_e24126;
        var_terfc_dn5 = assign22340_e24126_d_n5;
        var_terfc_dn6 = assign22340_e24126_d_n6;
        var_terfc_dn7 = assign22340_e24126_d_n7;
        var_terfc_dn8 = assign22340_e24126_d_n8;

        let (assign22350_e24147, assign22350_e24147_d_n5, assign22350_e24147_d_n6, assign22350_e24147_d_n7, assign22350_e24147_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) && (var_guard405 == 0.0)) {
        let assign22350_e24143: f64 = (var_perfc * var_xerfc);
        let assign22350_e24144: f64 = (1.0 - assign22350_e24143);
        let assign22350_e24145: f64 = (1.0 / assign22350_e24144);
        (assign22350_e24145, (-((-(var_perfc * var_xerfc_dn5)) / (assign22350_e24144 * assign22350_e24144))), (-((-(var_perfc * var_xerfc_dn6)) / (assign22350_e24144 * assign22350_e24144))), (-((-(var_perfc * var_xerfc_dn7)) / (assign22350_e24144 * assign22350_e24144))), (-((-(var_perfc * var_xerfc_dn8)) / (assign22350_e24144 * assign22350_e24144))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign22350_e24147;
        var_terfc_dn5 = assign22350_e24147_d_n5;
        var_terfc_dn6 = assign22350_e24147_d_n6;
        var_terfc_dn7 = assign22350_e24147_d_n7;
        var_terfc_dn8 = assign22350_e24147_d_n8;

        let assign22360_e24149: f64 = (-var_ysq);
        let assign22360_e24151: f64 = (assign22360_e24149 + var_mtat);
        let assign22360_e24153: f64 = (-230.25850929940458);
        let assign22360_e24154: f64 = if assign22360_e24151 > assign22360_e24153 { 1.0 } else { 0.0 };
        var_guard406 = assign22360_e24154;

        let (assign22370_e24172, assign22370_e24172_d_n5, assign22370_e24172_d_n6, assign22370_e24172_d_n7, assign22370_e24172_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) && (var_guard406 != 0.0)) {
        let assign22370_e24167: f64 = (-var_ysq);
        let assign22370_e24169: f64 = (assign22370_e24167 + var_mtat);
        let assign22370_e24170: f64 = (assign22370_e24169).exp();
        (assign22370_e24170, (assign22370_e24170 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign22370_e24170 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign22370_e24170 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign22370_e24170 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22370_e24172;
        var_tmp_dn5 = assign22370_e24172_d_n5;
        var_tmp_dn6 = assign22370_e24172_d_n6;
        var_tmp_dn7 = assign22370_e24172_d_n7;
        var_tmp_dn8 = assign22370_e24172_d_n8;

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
        *var_guard399_slot = var_guard399;
        *var_guard400_slot = var_guard400;
        *var_guard401_slot = var_guard401;
        *var_guard402_slot = var_guard402;
        *var_guard403_slot = var_guard403;
        *var_guard404_slot = var_guard404;
        *var_guard405_slot = var_guard405;
        *var_guard406_slot = var_guard406;
        *var_id__blk219_slot = var_id__blk219;
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

    pub(super) fn stamp_transient_block_40(
        p: &Parameters,
        var_absource_i: f64,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatgat: f64,
        var_berfc: f64,
        var_cerfc: f64,
        var_fbbtgat: f64,
        var_fbbtgat_dn5: f64,
        var_fbbtgat_dn6: f64,
        var_fbbtgat_dn7: f64,
        var_fbbtgat_dn8: f64,
        var_fstopgat: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard399: f64,
        var_guard403: f64,
        var_guard406: f64,
        var_id__blk219: f64,
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
        var_terfc: f64,
        var_terfc_dn5: f64,
        var_terfc_dn6: f64,
        var_terfc_dn7: f64,
        var_terfc_dn8: f64,
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
        var_guard407_slot: &mut f64,
        var_guard408_slot: &mut f64,
        var_guard409_slot: &mut f64,
        var_guard410_slot: &mut f64,
        var_guard411_slot: &mut f64,
        var_guard412_slot: &mut f64,
        var_guard413_slot: &mut f64,
        var_guard414_slot: &mut f64,
        var_guard415_slot: &mut f64,
        var_guard416_slot: &mut f64,
        var_guard417_slot: &mut f64,
        var_guard418_slot: &mut f64,
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
        let mut var_guard407: f64 = *var_guard407_slot;
        let mut var_guard408: f64 = *var_guard408_slot;
        let mut var_guard409: f64 = *var_guard409_slot;
        let mut var_guard410: f64 = *var_guard410_slot;
        let mut var_guard411: f64 = *var_guard411_slot;
        let mut var_guard412: f64 = *var_guard412_slot;
        let mut var_guard413: f64 = *var_guard413_slot;
        let mut var_guard414: f64 = *var_guard414_slot;
        let mut var_guard415: f64 = *var_guard415_slot;
        let mut var_guard416: f64 = *var_guard416_slot;
        let mut var_guard417: f64 = *var_guard417_slot;
        let mut var_guard418: f64 = *var_guard418_slot;
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

        let (assign22380_e24221, assign22380_e24221_d_n5, assign22380_e24221_d_n6, assign22380_e24221_d_n7, assign22380_e24221_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) && (var_guard406 == 0.0)) {
        let assign22380_e24188: f64 = (-230.25850929940458);
        let assign22380_e24190: f64 = (-var_ysq);
        let assign22380_e24192: f64 = (assign22380_e24190 + var_mtat);
        let assign22380_e24193: f64 = (assign22380_e24188 - assign22380_e24192);
        let assign22380_e24197: f64 = (-230.25850929940458);
        let assign22380_e24199: f64 = (-var_ysq);
        let assign22380_e24201: f64 = (assign22380_e24199 + var_mtat);
        let assign22380_e24202: f64 = (assign22380_e24197 - assign22380_e24201);
        let assign22380_e24205: f64 = (-230.25850929940458);
        let assign22380_e24207: f64 = (-var_ysq);
        let assign22380_e24209: f64 = (assign22380_e24207 + var_mtat);
        let assign22380_e24210: f64 = (assign22380_e24205 - assign22380_e24209);
        let assign22380_e24212: f64 = (assign22380_e24210 * 0.3333333333333333);
        let assign22380_e24213: f64 = (1.0 + assign22380_e24212);
        let assign22380_e24214: f64 = (assign22380_e24202 * assign22380_e24213);
        let assign22380_e24215: f64 = (0.5 * assign22380_e24214);
        let assign22380_e24216: f64 = (1.0 + assign22380_e24215);
        let assign22380_e24217: f64 = (assign22380_e24193 * assign22380_e24216);
        let assign22380_e24218: f64 = (1.0 + assign22380_e24217);
        let assign22380_e24219: f64 = (1e-100 / assign22380_e24218);
        (assign22380_e24219, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign22380_e24216) + (assign22380_e24193 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign22380_e24213) + (assign22380_e24202 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign22380_e24218 * assign22380_e24218))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign22380_e24216) + (assign22380_e24193 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign22380_e24213) + (assign22380_e24202 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign22380_e24218 * assign22380_e24218))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign22380_e24216) + (assign22380_e24193 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign22380_e24213) + (assign22380_e24202 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign22380_e24218 * assign22380_e24218))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign22380_e24216) + (assign22380_e24193 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign22380_e24213) + (assign22380_e24202 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign22380_e24218 * assign22380_e24218))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22380_e24221;
        var_tmp_dn5 = assign22380_e24221_d_n5;
        var_tmp_dn6 = assign22380_e24221_d_n6;
        var_tmp_dn7 = assign22380_e24221_d_n7;
        var_tmp_dn8 = assign22380_e24221_d_n8;

        let (assign22390_e24251, assign22390_e24251_d_n5, assign22390_e24251_d_n6, assign22390_e24251_d_n7, assign22390_e24251_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) {
        let assign22390_e24233: f64 = (0.29214664 * var_terfc);
        let assign22390_e24237: f64 = (var_terfc * var_terfc);
        let assign22390_e24238: f64 = (var_berfc * assign22390_e24237);
        let assign22390_e24239: f64 = (assign22390_e24233 + assign22390_e24238);
        let assign22390_e24243: f64 = (var_terfc * var_terfc);
        let assign22390_e24245: f64 = (assign22390_e24243 * var_terfc);
        let assign22390_e24246: f64 = (var_cerfc * assign22390_e24245);
        let assign22390_e24247: f64 = (assign22390_e24239 + assign22390_e24246);
        let assign22390_e24249: f64 = (assign22390_e24247 * var_tmp);
        (assign22390_e24249, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign22390_e24243 * var_terfc_dn5)))) * var_tmp) + (assign22390_e24247 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign22390_e24243 * var_terfc_dn6)))) * var_tmp) + (assign22390_e24247 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign22390_e24243 * var_terfc_dn7)))) * var_tmp) + (assign22390_e24247 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign22390_e24243 * var_terfc_dn8)))) * var_tmp) + (assign22390_e24247 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign22390_e24251;
        var_erfcpos_dn5 = assign22390_e24251_d_n5;
        var_erfcpos_dn6 = assign22390_e24251_d_n6;
        var_erfcpos_dn7 = assign22390_e24251_d_n7;
        var_erfcpos_dn8 = assign22390_e24251_d_n8;

        let assign22400_e24254: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard407 = assign22400_e24254;

        let (assign22410_e24268, assign22410_e24268_d_n5, assign22410_e24268_d_n6, assign22410_e24268_d_n7, assign22410_e24268_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) && (var_guard407 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign22410_e24268;
        var_erfctimesexpmtat_dn5 = assign22410_e24268_d_n5;
        var_erfctimesexpmtat_dn6 = assign22410_e24268_d_n6;
        var_erfctimesexpmtat_dn7 = assign22410_e24268_d_n7;
        var_erfctimesexpmtat_dn8 = assign22410_e24268_d_n8;

        let assign22420_e24271: f64 = (-230.25850929940458);
        let assign22420_e24272: f64 = if var_mtat > assign22420_e24271 { 1.0 } else { 0.0 };
        var_guard408 = assign22420_e24272;

        let (assign22430_e24290, assign22430_e24290_d_n5, assign22430_e24290_d_n6, assign22430_e24290_d_n7, assign22430_e24290_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) && (var_guard407 == 0.0)) && (var_guard408 != 0.0)) {
        let assign22430_e24288: f64 = (var_mtat).exp();
        (assign22430_e24288, (assign22430_e24288 * var_mtat_dn5), (assign22430_e24288 * var_mtat_dn6), (assign22430_e24288 * var_mtat_dn7), (assign22430_e24288 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22430_e24290;
        var_tmp_dn5 = assign22430_e24290_d_n5;
        var_tmp_dn6 = assign22430_e24290_d_n6;
        var_tmp_dn7 = assign22430_e24290_d_n7;
        var_tmp_dn8 = assign22430_e24290_d_n8;

        let (assign22440_e24333, assign22440_e24333_d_n5, assign22440_e24333_d_n6, assign22440_e24333_d_n7, assign22440_e24333_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) && (var_guard407 == 0.0)) && (var_guard408 == 0.0)) {
        let assign22440_e24309: f64 = (-230.25850929940458);
        let assign22440_e24311: f64 = (assign22440_e24309 - var_mtat);
        let assign22440_e24315: f64 = (-230.25850929940458);
        let assign22440_e24317: f64 = (assign22440_e24315 - var_mtat);
        let assign22440_e24320: f64 = (-230.25850929940458);
        let assign22440_e24322: f64 = (assign22440_e24320 - var_mtat);
        let assign22440_e24324: f64 = (assign22440_e24322 * 0.3333333333333333);
        let assign22440_e24325: f64 = (1.0 + assign22440_e24324);
        let assign22440_e24326: f64 = (assign22440_e24317 * assign22440_e24325);
        let assign22440_e24327: f64 = (0.5 * assign22440_e24326);
        let assign22440_e24328: f64 = (1.0 + assign22440_e24327);
        let assign22440_e24329: f64 = (assign22440_e24311 * assign22440_e24328);
        let assign22440_e24330: f64 = (1.0 + assign22440_e24329);
        let assign22440_e24331: f64 = (1e-100 / assign22440_e24330);
        (assign22440_e24331, (-((1e-100 * (((-var_mtat_dn5) * assign22440_e24328) + (assign22440_e24311 * (0.5 * (((-var_mtat_dn5) * assign22440_e24325) + (assign22440_e24317 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign22440_e24330 * assign22440_e24330))), (-((1e-100 * (((-var_mtat_dn6) * assign22440_e24328) + (assign22440_e24311 * (0.5 * (((-var_mtat_dn6) * assign22440_e24325) + (assign22440_e24317 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign22440_e24330 * assign22440_e24330))), (-((1e-100 * (((-var_mtat_dn7) * assign22440_e24328) + (assign22440_e24311 * (0.5 * (((-var_mtat_dn7) * assign22440_e24325) + (assign22440_e24317 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign22440_e24330 * assign22440_e24330))), (-((1e-100 * (((-var_mtat_dn8) * assign22440_e24328) + (assign22440_e24311 * (0.5 * (((-var_mtat_dn8) * assign22440_e24325) + (assign22440_e24317 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign22440_e24330 * assign22440_e24330))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22440_e24333;
        var_tmp_dn5 = assign22440_e24333_d_n5;
        var_tmp_dn6 = assign22440_e24333_d_n6;
        var_tmp_dn7 = assign22440_e24333_d_n7;
        var_tmp_dn8 = assign22440_e24333_d_n8;

        let (assign22450_e24352, assign22450_e24352_d_n5, assign22450_e24352_d_n6, assign22450_e24352_d_n7, assign22450_e24352_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) && (var_guard407 == 0.0)) {
        let assign22450_e24348: f64 = (2.0 * var_tmp);
        let assign22450_e24350: f64 = (assign22450_e24348 - var_erfcpos);
        (assign22450_e24350, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign22450_e24352;
        var_erfctimesexpmtat_dn5 = assign22450_e24352_d_n5;
        var_erfctimesexpmtat_dn6 = assign22450_e24352_d_n6;
        var_erfctimesexpmtat_dn7 = assign22450_e24352_d_n7;
        var_erfctimesexpmtat_dn8 = assign22450_e24352_d_n8;

        let (assign22460_e24372, assign22460_e24372_d_n5, assign22460_e24372_d_n6, assign22460_e24372_d_n7, assign22460_e24372_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) {
        let assign22460_e24364: f64 = (1.772453850905516 * 0.5);
        let assign22460_e24367: f64 = (var_atatgat * var_erfctimesexpmtat);
        let assign22460_e24369: f64 = (assign22460_e24367 / var_ktat);
        let assign22460_e24370: f64 = (assign22460_e24364 * assign22460_e24369);
        (assign22460_e24370, (assign22460_e24364 * ((((var_atatgat * var_erfctimesexpmtat_dn5) * var_ktat) - (assign22460_e24367 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign22460_e24364 * ((((var_atatgat * var_erfctimesexpmtat_dn6) * var_ktat) - (assign22460_e24367 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign22460_e24364 * ((((var_atatgat * var_erfctimesexpmtat_dn7) * var_ktat) - (assign22460_e24367 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign22460_e24364 * ((((var_atatgat * var_erfctimesexpmtat_dn8) * var_ktat) - (assign22460_e24367 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign22460_e24372;
        var_gammamax_dn5 = assign22460_e24372_d_n5;
        var_gammamax_dn6 = assign22460_e24372_d_n6;
        var_gammamax_dn7 = assign22460_e24372_d_n7;
        var_gammamax_dn8 = assign22460_e24372_d_n8;

        let (assign22470_e24390, assign22470_e24390_d_n5, assign22470_e24390_d_n6, assign22470_e24390_d_n7, assign22470_e24390_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard403 == 0.0)) {
        let assign22470_e24385: f64 = (var_asrh * var_gammamax);
        let assign22470_e24387: f64 = (assign22470_e24385 * var_wtat);
        let assign22470_e24388: f64 = (p.p847 * assign22470_e24387);
        (assign22470_e24388, (p.p847 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign22470_e24385 * var_wtat_dn5))), (p.p847 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign22470_e24385 * var_wtat_dn6))), (p.p847 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign22470_e24385 * var_wtat_dn7))), (p.p847 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign22470_e24385 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign22470_e24390;
        var_itat_dn5 = assign22470_e24390_d_n5;
        var_itat_dn6 = assign22470_e24390_d_n6;
        var_itat_dn7 = assign22470_e24390_d_n7;
        var_itat_dn8 = assign22470_e24390_d_n8;

        let assign22480_e24393: f64 = if p.p853 == 0.0 { 1.0 } else { 0.0 };
        var_guard409 = assign22480_e24393;

        let (assign22490_e24404, assign22490_e24404_d_n5, assign22490_e24404_d_n6, assign22490_e24404_d_n7, assign22490_e24404_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard409 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign22490_e24404;
        var_ibbt_dn5 = assign22490_e24404_d_n5;
        var_ibbt_dn6 = assign22490_e24404_d_n6;
        var_ibbt_dn7 = assign22490_e24404_d_n7;
        var_ibbt_dn8 = assign22490_e24404_d_n8;

        let assign22500_e24407: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        var_guard410 = assign22500_e24407;

        let (assign22510_e24426, assign22510_e24426_d_n5, assign22510_e24426_d_n6, assign22510_e24426_d_n7, assign22510_e24426_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard409 == 0.0)) && (var_guard410 != 0.0)) {
        let assign22510_e24421: f64 = (p.p830 - var_vbbt);
        let assign22510_e24423: f64 = (assign22510_e24421 * var_vbirgatinv);
        let assign22510_e24424: f64 = (assign22510_e24423).sqrt();
        (assign22510_e24424, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22510_e24426;
        var_tmp_dn5 = assign22510_e24426_d_n5;
        var_tmp_dn6 = assign22510_e24426_d_n6;
        var_tmp_dn7 = assign22510_e24426_d_n7;
        var_tmp_dn8 = assign22510_e24426_d_n8;

        let (assign22520_e24447, assign22520_e24447_d_n5, assign22520_e24447_d_n6, assign22520_e24447_d_n7, assign22520_e24447_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard409 == 0.0)) && (var_guard410 == 0.0)) {
        let assign22520_e24441: f64 = (p.p830 - var_vbbt);
        let assign22520_e24443: f64 = (assign22520_e24441 * var_vbirgatinv);
        let assign22520_e24445: f64 = (assign22520_e24443).powf(p.p833);
        (assign22520_e24445, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22520_e24447;
        var_tmp_dn5 = assign22520_e24447_d_n5;
        var_tmp_dn6 = assign22520_e24447_d_n6;
        var_tmp_dn7 = assign22520_e24447_d_n7;
        var_tmp_dn8 = assign22520_e24447_d_n8;

        let (assign22530_e24467, assign22530_e24467_d_n5, assign22530_e24467_d_n6, assign22530_e24467_d_n7, assign22530_e24467_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard409 == 0.0)) {
        let assign22530_e24460: f64 = (p.p830 - var_vbbt);
        let assign22530_e24462: f64 = (assign22530_e24460 * var_wdepnulrinvgat);
        let assign22530_e24464: f64 = (assign22530_e24462 / var_tmp);
        let assign22530_e24465: f64 = (var_one_over_one_minus_pgat * assign22530_e24464);
        (assign22530_e24465, (var_one_over_one_minus_pgat * (-((assign22530_e24462 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign22530_e24462 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign22530_e24462 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign22530_e24462 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign22530_e24467;
        var_fmaxr_dn5 = assign22530_e24467_d_n5;
        var_fmaxr_dn6 = assign22530_e24467_d_n6;
        var_fmaxr_dn7 = assign22530_e24467_d_n7;
        var_fmaxr_dn8 = assign22530_e24467_d_n8;

        let assign22540_e24469: f64 = (-var_fbbtgat);
        let assign22540_e24471: f64 = (assign22540_e24469 / var_fmaxr);
        let assign22540_e24472: f64 = (assign22540_e24471).abs();
        let assign22540_e24474: f64 = if assign22540_e24472 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard411 = assign22540_e24474;

        let (assign22550_e24492, assign22550_e24492_d_n5, assign22550_e24492_d_n6, assign22550_e24492_d_n7, assign22550_e24492_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard409 == 0.0)) && (var_guard411 != 0.0)) {
        let assign22550_e24487: f64 = (-var_fbbtgat);
        let assign22550_e24489: f64 = (assign22550_e24487 / var_fmaxr);
        let assign22550_e24490: f64 = (assign22550_e24489).exp();
        (assign22550_e24490, (assign22550_e24490 * ((((-var_fbbtgat_dn5) * var_fmaxr) - (assign22550_e24487 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))), (assign22550_e24490 * ((((-var_fbbtgat_dn6) * var_fmaxr) - (assign22550_e24487 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign22550_e24490 * ((((-var_fbbtgat_dn7) * var_fmaxr) - (assign22550_e24487 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign22550_e24490 * ((((-var_fbbtgat_dn8) * var_fmaxr) - (assign22550_e24487 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22550_e24492;
        var_tmp_dn5 = assign22550_e24492_d_n5;
        var_tmp_dn6 = assign22550_e24492_d_n6;
        var_tmp_dn7 = assign22550_e24492_d_n7;
        var_tmp_dn8 = assign22550_e24492_d_n8;

        let assign22560_e24494: f64 = (-var_fbbtgat);
        let assign22560_e24496: f64 = (assign22560_e24494 / var_fmaxr);
        let assign22560_e24498: f64 = if assign22560_e24496 < 0.0 { 1.0 } else { 0.0 };
        var_guard412 = assign22560_e24498;

        let (assign22570_e24549, assign22570_e24549_d_n5, assign22570_e24549_d_n6, assign22570_e24549_d_n7, assign22570_e24549_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard409 == 0.0)) && (var_guard411 == 0.0)) && (var_guard412 != 0.0)) {
        let assign22570_e24516: f64 = (-230.25850929940458);
        let assign22570_e24518: f64 = (-var_fbbtgat);
        let assign22570_e24520: f64 = (assign22570_e24518 / var_fmaxr);
        let assign22570_e24521: f64 = (assign22570_e24516 - assign22570_e24520);
        let assign22570_e24525: f64 = (-230.25850929940458);
        let assign22570_e24527: f64 = (-var_fbbtgat);
        let assign22570_e24529: f64 = (assign22570_e24527 / var_fmaxr);
        let assign22570_e24530: f64 = (assign22570_e24525 - assign22570_e24529);
        let assign22570_e24533: f64 = (-230.25850929940458);
        let assign22570_e24535: f64 = (-var_fbbtgat);
        let assign22570_e24537: f64 = (assign22570_e24535 / var_fmaxr);
        let assign22570_e24538: f64 = (assign22570_e24533 - assign22570_e24537);
        let assign22570_e24540: f64 = (assign22570_e24538 * 0.3333333333333333);
        let assign22570_e24541: f64 = (1.0 + assign22570_e24540);
        let assign22570_e24542: f64 = (assign22570_e24530 * assign22570_e24541);
        let assign22570_e24543: f64 = (0.5 * assign22570_e24542);
        let assign22570_e24544: f64 = (1.0 + assign22570_e24543);
        let assign22570_e24545: f64 = (assign22570_e24521 * assign22570_e24544);
        let assign22570_e24546: f64 = (1.0 + assign22570_e24545);
        let assign22570_e24547: f64 = (1e-100 / assign22570_e24546);
        (assign22570_e24547, (-((1e-100 * (((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign22570_e24518 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign22570_e24544) + (assign22570_e24521 * (0.5 * (((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign22570_e24527 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign22570_e24541) + (assign22570_e24530 * ((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign22570_e24535 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign22570_e24546 * assign22570_e24546))), (-((1e-100 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign22570_e24518 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign22570_e24544) + (assign22570_e24521 * (0.5 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign22570_e24527 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign22570_e24541) + (assign22570_e24530 * ((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign22570_e24535 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign22570_e24546 * assign22570_e24546))), (-((1e-100 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign22570_e24518 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign22570_e24544) + (assign22570_e24521 * (0.5 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign22570_e24527 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign22570_e24541) + (assign22570_e24530 * ((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign22570_e24535 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign22570_e24546 * assign22570_e24546))), (-((1e-100 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign22570_e24518 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign22570_e24544) + (assign22570_e24521 * (0.5 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign22570_e24527 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign22570_e24541) + (assign22570_e24530 * ((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign22570_e24535 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign22570_e24546 * assign22570_e24546))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22570_e24549;
        var_tmp_dn5 = assign22570_e24549_d_n5;
        var_tmp_dn6 = assign22570_e24549_d_n6;
        var_tmp_dn7 = assign22570_e24549_d_n7;
        var_tmp_dn8 = assign22570_e24549_d_n8;

        let (assign22580_e24598, assign22580_e24598_d_n5, assign22580_e24598_d_n6, assign22580_e24598_d_n7, assign22580_e24598_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard409 == 0.0)) && (var_guard411 == 0.0)) && (var_guard412 == 0.0)) {
        let assign22580_e24568: f64 = (-var_fbbtgat);
        let assign22580_e24570: f64 = (assign22580_e24568 / var_fmaxr);
        let assign22580_e24572: f64 = (assign22580_e24570 - 230.25850929940458);
        let assign22580_e24576: f64 = (-var_fbbtgat);
        let assign22580_e24578: f64 = (assign22580_e24576 / var_fmaxr);
        let assign22580_e24580: f64 = (assign22580_e24578 - 230.25850929940458);
        let assign22580_e24583: f64 = (-var_fbbtgat);
        let assign22580_e24585: f64 = (assign22580_e24583 / var_fmaxr);
        let assign22580_e24587: f64 = (assign22580_e24585 - 230.25850929940458);
        let assign22580_e24589: f64 = (assign22580_e24587 * 0.3333333333333333);
        let assign22580_e24590: f64 = (1.0 + assign22580_e24589);
        let assign22580_e24591: f64 = (assign22580_e24580 * assign22580_e24590);
        let assign22580_e24592: f64 = (0.5 * assign22580_e24591);
        let assign22580_e24593: f64 = (1.0 + assign22580_e24592);
        let assign22580_e24594: f64 = (assign22580_e24572 * assign22580_e24593);
        let assign22580_e24595: f64 = (1.0 + assign22580_e24594);
        let assign22580_e24596: f64 = (1e100 * assign22580_e24595);
        (assign22580_e24596, (1e100 * ((((((-var_fbbtgat_dn5) * var_fmaxr) - (assign22580_e24568 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign22580_e24593) + (assign22580_e24572 * (0.5 * ((((((-var_fbbtgat_dn5) * var_fmaxr) - (assign22580_e24576 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign22580_e24590) + (assign22580_e24580 * (((((-var_fbbtgat_dn5) * var_fmaxr) - (assign22580_e24583 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign22580_e24568 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign22580_e24593) + (assign22580_e24572 * (0.5 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign22580_e24576 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign22580_e24590) + (assign22580_e24580 * (((((-var_fbbtgat_dn6) * var_fmaxr) - (assign22580_e24583 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign22580_e24568 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign22580_e24593) + (assign22580_e24572 * (0.5 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign22580_e24576 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign22580_e24590) + (assign22580_e24580 * (((((-var_fbbtgat_dn7) * var_fmaxr) - (assign22580_e24583 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign22580_e24568 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign22580_e24593) + (assign22580_e24572 * (0.5 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign22580_e24576 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign22580_e24590) + (assign22580_e24580 * (((((-var_fbbtgat_dn8) * var_fmaxr) - (assign22580_e24583 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22580_e24598;
        var_tmp_dn5 = assign22580_e24598_d_n5;
        var_tmp_dn6 = assign22580_e24598_d_n6;
        var_tmp_dn7 = assign22580_e24598_d_n7;
        var_tmp_dn8 = assign22580_e24598_d_n8;

        let (assign22590_e24618, assign22590_e24618_d_n5, assign22590_e24618_d_n6, assign22590_e24618_d_n7, assign22590_e24618_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard409 == 0.0)) {
        let assign22590_e24611: f64 = (var_v3 * var_fmaxr);
        let assign22590_e24613: f64 = (assign22590_e24611 * var_fmaxr);
        let assign22590_e24615: f64 = (assign22590_e24613 * var_tmp);
        let assign22590_e24616: f64 = (p.p853 * assign22590_e24615);
        (assign22590_e24616, (p.p853 * (((((var_v3 * var_fmaxr_dn5) * var_fmaxr) + (assign22590_e24611 * var_fmaxr_dn5)) * var_tmp) + (assign22590_e24613 * var_tmp_dn5))), (p.p853 * (((((var_v3 * var_fmaxr_dn6) * var_fmaxr) + (assign22590_e24611 * var_fmaxr_dn6)) * var_tmp) + (assign22590_e24613 * var_tmp_dn6))), (p.p853 * (((((var_v3 * var_fmaxr_dn7) * var_fmaxr) + (assign22590_e24611 * var_fmaxr_dn7)) * var_tmp) + (assign22590_e24613 * var_tmp_dn7))), (p.p853 * (((((var_v3 * var_fmaxr_dn8) * var_fmaxr) + (assign22590_e24611 * var_fmaxr_dn8)) * var_tmp) + (assign22590_e24613 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign22590_e24618;
        var_ibbt_dn5 = assign22590_e24618_d_n5;
        var_ibbt_dn6 = assign22590_e24618_d_n6;
        var_ibbt_dn7 = assign22590_e24618_d_n7;
        var_ibbt_dn8 = assign22590_e24618_d_n8;

        let assign22600_e24621: f64 = if p.p862 > 1000.0 { 1.0 } else { 0.0 };
        var_guard413 = assign22600_e24621;

        let (assign22610_e24632, assign22610_e24632_d_n5, assign22610_e24632_d_n6, assign22610_e24632_d_n7, assign22610_e24632_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard413 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign22610_e24632;
        var_fbreakdown_dn5 = assign22610_e24632_d_n5;
        var_fbreakdown_dn6 = assign22610_e24632_d_n6;
        var_fbreakdown_dn7 = assign22610_e24632_d_n7;
        var_fbreakdown_dn8 = assign22610_e24632_d_n8;

        let assign22620_e24635: f64 = (-var_alphaav);
        let assign22620_e24637: f64 = (assign22620_e24635 * p.p862);
        let assign22620_e24638: f64 = if var_vav > assign22620_e24637 { 1.0 } else { 0.0 };
        var_guard414 = assign22620_e24638;

        let assign22630_e24641: f64 = if p.p865 == 4.0 { 1.0 } else { 0.0 };
        var_guard415 = assign22630_e24641;

        let (assign22640_e24671, assign22640_e24671_d_n5, assign22640_e24671_d_n6, assign22640_e24671_d_n7, assign22640_e24671_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard413 == 0.0)) && (var_guard414 != 0.0)) && (var_guard415 != 0.0)) {
        let assign22640_e24657: f64 = (var_vav * var_vbrinvgat);
        let assign22640_e24660: f64 = (var_vav * var_vbrinvgat);
        let assign22640_e24661: f64 = (assign22640_e24657 * assign22640_e24660);
        let assign22640_e24664: f64 = (var_vav * var_vbrinvgat);
        let assign22640_e24665: f64 = (assign22640_e24661 * assign22640_e24664);
        let assign22640_e24668: f64 = (var_vav * var_vbrinvgat);
        let assign22640_e24669: f64 = (assign22640_e24665 * assign22640_e24668);
        (assign22640_e24669, (((((((var_vav * var_vbrinvgat_dn5) * assign22640_e24660) + (assign22640_e24657 * (var_vav * var_vbrinvgat_dn5))) * assign22640_e24664) + (assign22640_e24661 * (var_vav * var_vbrinvgat_dn5))) * assign22640_e24668) + (assign22640_e24665 * (var_vav * var_vbrinvgat_dn5))), (((((((var_vav * var_vbrinvgat_dn6) * assign22640_e24660) + (assign22640_e24657 * (var_vav * var_vbrinvgat_dn6))) * assign22640_e24664) + (assign22640_e24661 * (var_vav * var_vbrinvgat_dn6))) * assign22640_e24668) + (assign22640_e24665 * (var_vav * var_vbrinvgat_dn6))), (((((((var_vav * var_vbrinvgat_dn7) * assign22640_e24660) + (assign22640_e24657 * (var_vav * var_vbrinvgat_dn7))) * assign22640_e24664) + (assign22640_e24661 * (var_vav * var_vbrinvgat_dn7))) * assign22640_e24668) + (assign22640_e24665 * (var_vav * var_vbrinvgat_dn7))), (((((((var_vav * var_vbrinvgat_dn8) * assign22640_e24660) + (assign22640_e24657 * (var_vav * var_vbrinvgat_dn8))) * assign22640_e24664) + (assign22640_e24661 * (var_vav * var_vbrinvgat_dn8))) * assign22640_e24668) + (assign22640_e24665 * (var_vav * var_vbrinvgat_dn8))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22640_e24671;
        var_tmp_dn5 = assign22640_e24671_d_n5;
        var_tmp_dn6 = assign22640_e24671_d_n6;
        var_tmp_dn7 = assign22640_e24671_d_n7;
        var_tmp_dn8 = assign22640_e24671_d_n8;

        let (assign22650_e24693, assign22650_e24693_d_n5, assign22650_e24693_d_n6, assign22650_e24693_d_n7, assign22650_e24693_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard413 == 0.0)) && (var_guard414 != 0.0)) && (var_guard415 == 0.0)) {
        let assign22650_e24688: f64 = (var_vav * var_vbrinvgat);
        let assign22650_e24689: f64 = (assign22650_e24688).abs();
        let assign22650_e24691: f64 = (assign22650_e24689).powf(p.p865);
        (assign22650_e24691, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign22650_e24689).powf(p.p865 - 1.0) * if assign22650_e24688 >= 0.0 { (var_vav * var_vbrinvgat_dn5) } else { (-(var_vav * var_vbrinvgat_dn5)) })) } } else { (assign22650_e24691 * (p.p865 * (if assign22650_e24688 >= 0.0 { (var_vav * var_vbrinvgat_dn5) } else { (-(var_vav * var_vbrinvgat_dn5)) } / assign22650_e24689))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign22650_e24689).powf(p.p865 - 1.0) * if assign22650_e24688 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) })) } } else { (assign22650_e24691 * (p.p865 * (if assign22650_e24688 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) } / assign22650_e24689))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign22650_e24689).powf(p.p865 - 1.0) * if assign22650_e24688 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) })) } } else { (assign22650_e24691 * (p.p865 * (if assign22650_e24688 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) } / assign22650_e24689))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign22650_e24689).powf(p.p865 - 1.0) * if assign22650_e24688 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) })) } } else { (assign22650_e24691 * (p.p865 * (if assign22650_e24688 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) } / assign22650_e24689))) },)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign22650_e24693;
        var_tmp_dn5 = assign22650_e24693_d_n5;
        var_tmp_dn6 = assign22650_e24693_d_n6;
        var_tmp_dn7 = assign22650_e24693_d_n7;
        var_tmp_dn8 = assign22650_e24693_d_n8;

        let (assign22660_e24711, assign22660_e24711_d_n5, assign22660_e24711_d_n6, assign22660_e24711_d_n7, assign22660_e24711_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard413 == 0.0)) && (var_guard414 != 0.0)) {
        let assign22660_e24708: f64 = (1.0 - var_tmp);
        let assign22660_e24709: f64 = (1.0 / assign22660_e24708);
        (assign22660_e24709, (-((-var_tmp_dn5) / (assign22660_e24708 * assign22660_e24708))), (-((-var_tmp_dn6) / (assign22660_e24708 * assign22660_e24708))), (-((-var_tmp_dn7) / (assign22660_e24708 * assign22660_e24708))), (-((-var_tmp_dn8) / (assign22660_e24708 * assign22660_e24708))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign22660_e24711;
        var_fbreakdown_dn5 = assign22660_e24711_d_n5;
        var_fbreakdown_dn6 = assign22660_e24711_d_n6;
        var_fbreakdown_dn7 = assign22660_e24711_d_n7;
        var_fbreakdown_dn8 = assign22660_e24711_d_n8;

        let (assign22670_e24734, assign22670_e24734_d_n5, assign22670_e24734_d_n6, assign22670_e24734_d_n7, assign22670_e24734_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) && (var_guard413 == 0.0)) && (var_guard414 == 0.0)) {
        let assign22670_e24728: f64 = (var_alphaav * p.p862);
        let assign22670_e24729: f64 = (var_vav + assign22670_e24728);
        let assign22670_e24731: f64 = (assign22670_e24729 * var_slopegat);
        let assign22670_e24732: f64 = (var_fstopgat + assign22670_e24731);
        (assign22670_e24732, (assign22670_e24729 * var_slopegat_dn5), (assign22670_e24729 * var_slopegat_dn6), (assign22670_e24729 * var_slopegat_dn7), (assign22670_e24729 * var_slopegat_dn8),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign22670_e24734;
        var_fbreakdown_dn5 = assign22670_e24734_d_n5;
        var_fbreakdown_dn6 = assign22670_e24734_d_n6;
        var_fbreakdown_dn7 = assign22670_e24734_d_n7;
        var_fbreakdown_dn8 = assign22670_e24734_d_n8;

        let (assign22680_e24753, assign22680_e24753_d_n5, assign22680_e24753_d_n6, assign22680_e24753_d_n7, assign22680_e24753_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard399 == 0.0)) {
        let assign22680_e24744: f64 = (var_id__blk219 + var_isrh);
        let assign22680_e24746: f64 = (assign22680_e24744 + var_itat);
        let assign22680_e24748: f64 = (assign22680_e24746 + var_ibbt);
        let assign22680_e24749: f64 = (p.p29 * assign22680_e24748);
        let assign22680_e24751: f64 = (assign22680_e24749 * var_fbreakdown);
        (assign22680_e24751, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign22680_e24749 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign22680_e24749 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign22680_e24749 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign22680_e24749 * var_fbreakdown_dn8)),)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign22680_e24753;
        var_ijungat_dn5 = assign22680_e24753_d_n5;
        var_ijungat_dn6 = assign22680_e24753_d_n6;
        var_ijungat_dn7 = assign22680_e24753_d_n7;
        var_ijungat_dn8 = assign22680_e24753_d_n8;

        let (assign22690_e24769, assign22690_e24769_d_n5, assign22690_e24769_d_n6, assign22690_e24769_d_n7, assign22690_e24769_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign22690_e24759: f64 = (var_absource_i * var_ijunbot);
        let assign22690_e24762: f64 = (var_lssource_i * var_ijunsti);
        let assign22690_e24763: f64 = (assign22690_e24759 + assign22690_e24762);
        let assign22690_e24766: f64 = (var_lgsource_i * var_ijungat);
        let assign22690_e24767: f64 = (assign22690_e24763 + assign22690_e24766);
        (assign22690_e24767, (((var_absource_i * var_ijunbot_dn5) + (var_lssource_i * var_ijunsti_dn5)) + (var_lgsource_i * var_ijungat_dn5)), (((var_absource_i * var_ijunbot_dn6) + (var_lssource_i * var_ijunsti_dn6)) + (var_lgsource_i * var_ijungat_dn6)), (((var_absource_i * var_ijunbot_dn7) + (var_lssource_i * var_ijunsti_dn7)) + (var_lgsource_i * var_ijungat_dn7)), (((var_absource_i * var_ijunbot_dn8) + (var_lssource_i * var_ijunsti_dn8)) + (var_lgsource_i * var_ijungat_dn8)),)
    } else {
        (var_i3, var_i3_dn5, var_i3_dn6, var_i3_dn7, var_i3_dn8,)
    }
};
        var_i3 = assign22690_e24769;
        var_i3_dn5 = assign22690_e24769_d_n5;
        var_i3_dn6 = assign22690_e24769_d_n6;
        var_i3_dn7 = assign22690_e24769_d_n7;
        var_i3_dn8 = assign22690_e24769_d_n8;

        let (assign22700_e24775,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign22700_e24775;

        let (assign22710_e24781,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign22710_e24781;

        let assign22720_e24793: f64 = if (!(((var_absource_i == 0.0) && (var_lssource_i == 0.0)) && (var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard416 = assign22720_e24793;

        let assign22800_e24879: f64 = if var_v4 < var_vmax_s { 1.0 } else { 0.0 };
        var_guard417 = assign22800_e24879;

        let assign22810_e24881: f64 = (-0.5);
        let assign22810_e24884: f64 = (var_v4 * var_phitdinv);
        let assign22810_e24885: f64 = (assign22810_e24881 * assign22810_e24884);
        let assign22810_e24886: f64 = (assign22810_e24885).abs();
        let assign22810_e24888: f64 = if assign22810_e24886 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard418 = assign22810_e24888;

        let (assign22820_e24906,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard416 != 0.0)) && (var_guard417 != 0.0)) && (var_guard418 != 0.0)) {
        let assign22820_e24899: f64 = (-0.5);
        let assign22820_e24902: f64 = (var_v4 * var_phitdinv);
        let assign22820_e24903: f64 = (assign22820_e24899 * assign22820_e24902);
        let assign22820_e24904: f64 = (assign22820_e24903).exp();
        (assign22820_e24904,)
    } else {
        (var_z,)
    }
};
        var_z = assign22820_e24906;

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
        *var_guard407_slot = var_guard407;
        *var_guard408_slot = var_guard408;
        *var_guard409_slot = var_guard409;
        *var_guard410_slot = var_guard410;
        *var_guard411_slot = var_guard411;
        *var_guard412_slot = var_guard412;
        *var_guard413_slot = var_guard413;
        *var_guard414_slot = var_guard414;
        *var_guard415_slot = var_guard415;
        *var_guard416_slot = var_guard416;
        *var_guard417_slot = var_guard417;
        *var_guard418_slot = var_guard418;
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

    pub(super) fn stamp_transient_block_41(
        p: &Parameters,
        var_absource_i: f64,
        var_atatbot: f64,
        var_btatpartbot: f64,
        var_exp_vmax_over_phitd_s: f64,
        var_ftdbot: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard416: f64,
        var_guard417: f64,
        var_guard418: f64,
        var_idsatbot: f64,
        var_one_minus_pbot: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitr: f64,
        var_v4: f64,
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
        var_guard419_slot: &mut f64,
        var_guard420_slot: &mut f64,
        var_guard421_slot: &mut f64,
        var_guard422_slot: &mut f64,
        var_guard423_slot: &mut f64,
        var_guard424_slot: &mut f64,
        var_guard425_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
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
        let mut var_guard419: f64 = *var_guard419_slot;
        let mut var_guard420: f64 = *var_guard420_slot;
        let mut var_guard421: f64 = *var_guard421_slot;
        let mut var_guard422: f64 = *var_guard422_slot;
        let mut var_guard423: f64 = *var_guard423_slot;
        let mut var_guard424: f64 = *var_guard424_slot;
        let mut var_guard425: f64 = *var_guard425_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
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
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_z: f64 = *var_z_slot;
        let mut var_zinv: f64 = *var_zinv_slot;

        let assign22830_e24908: f64 = (-0.5);
        let assign22830_e24911: f64 = (var_v4 * var_phitdinv);
        let assign22830_e24912: f64 = (assign22830_e24908 * assign22830_e24911);
        let assign22830_e24914: f64 = if assign22830_e24912 < 0.0 { 1.0 } else { 0.0 };
        var_guard419 = assign22830_e24914;

        let (assign22840_e24969,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard416 != 0.0)) && (var_guard417 != 0.0)) && (var_guard418 == 0.0)) && (var_guard419 != 0.0)) {
        let assign22840_e24930: f64 = (-230.25850929940458);
        let assign22840_e24932: f64 = (-0.5);
        let assign22840_e24935: f64 = (var_v4 * var_phitdinv);
        let assign22840_e24936: f64 = (assign22840_e24932 * assign22840_e24935);
        let assign22840_e24937: f64 = (assign22840_e24930 - assign22840_e24936);
        let assign22840_e24941: f64 = (-230.25850929940458);
        let assign22840_e24943: f64 = (-0.5);
        let assign22840_e24946: f64 = (var_v4 * var_phitdinv);
        let assign22840_e24947: f64 = (assign22840_e24943 * assign22840_e24946);
        let assign22840_e24948: f64 = (assign22840_e24941 - assign22840_e24947);
        let assign22840_e24951: f64 = (-230.25850929940458);
        let assign22840_e24953: f64 = (-0.5);
        let assign22840_e24956: f64 = (var_v4 * var_phitdinv);
        let assign22840_e24957: f64 = (assign22840_e24953 * assign22840_e24956);
        let assign22840_e24958: f64 = (assign22840_e24951 - assign22840_e24957);
        let assign22840_e24960: f64 = (assign22840_e24958 * 0.3333333333333333);
        let assign22840_e24961: f64 = (1.0 + assign22840_e24960);
        let assign22840_e24962: f64 = (assign22840_e24948 * assign22840_e24961);
        let assign22840_e24963: f64 = (0.5 * assign22840_e24962);
        let assign22840_e24964: f64 = (1.0 + assign22840_e24963);
        let assign22840_e24965: f64 = (assign22840_e24937 * assign22840_e24964);
        let assign22840_e24966: f64 = (1.0 + assign22840_e24965);
        let assign22840_e24967: f64 = (1e-100 / assign22840_e24966);
        (assign22840_e24967,)
    } else {
        (var_z,)
    }
};
        var_z = assign22840_e24969;

        let (assign22850_e25022,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard416 != 0.0)) && (var_guard417 != 0.0)) && (var_guard418 == 0.0)) && (var_guard419 == 0.0)) {
        let assign22850_e24986: f64 = (-0.5);
        let assign22850_e24989: f64 = (var_v4 * var_phitdinv);
        let assign22850_e24990: f64 = (assign22850_e24986 * assign22850_e24989);
        let assign22850_e24992: f64 = (assign22850_e24990 - 230.25850929940458);
        let assign22850_e24996: f64 = (-0.5);
        let assign22850_e24999: f64 = (var_v4 * var_phitdinv);
        let assign22850_e25000: f64 = (assign22850_e24996 * assign22850_e24999);
        let assign22850_e25002: f64 = (assign22850_e25000 - 230.25850929940458);
        let assign22850_e25005: f64 = (-0.5);
        let assign22850_e25008: f64 = (var_v4 * var_phitdinv);
        let assign22850_e25009: f64 = (assign22850_e25005 * assign22850_e25008);
        let assign22850_e25011: f64 = (assign22850_e25009 - 230.25850929940458);
        let assign22850_e25013: f64 = (assign22850_e25011 * 0.3333333333333333);
        let assign22850_e25014: f64 = (1.0 + assign22850_e25013);
        let assign22850_e25015: f64 = (assign22850_e25002 * assign22850_e25014);
        let assign22850_e25016: f64 = (0.5 * assign22850_e25015);
        let assign22850_e25017: f64 = (1.0 + assign22850_e25016);
        let assign22850_e25018: f64 = (assign22850_e24992 * assign22850_e25017);
        let assign22850_e25019: f64 = (1.0 + assign22850_e25018);
        let assign22850_e25020: f64 = (1e100 * assign22850_e25019);
        (assign22850_e25020,)
    } else {
        (var_z,)
    }
};
        var_z = assign22850_e25022;

        let (assign22860_e25034,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard416 != 0.0)) && (var_guard417 != 0.0)) {
        let assign22860_e25032: f64 = (1.0 / var_z);
        (assign22860_e25032,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign22860_e25034;

        let (assign22870_e25046,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard416 != 0.0)) && (var_guard417 != 0.0)) {
        let assign22870_e25044: f64 = (var_zinv * var_zinv);
        (assign22870_e25044,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign22870_e25046;

        let (assign22880_e25065,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard416 != 0.0)) && (var_guard417 == 0.0)) {
        let assign22880_e25058: f64 = (var_v4 - var_vmax_s);
        let assign22880_e25060: f64 = (assign22880_e25058 * var_phitdinv);
        let assign22880_e25061: f64 = (1.0 + assign22880_e25060);
        let assign22880_e25063: f64 = (assign22880_e25061 * var_exp_vmax_over_phitd_s);
        (assign22880_e25063,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign22880_e25065;

        let (assign22890_e25077,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard416 != 0.0)) && (var_guard417 == 0.0)) {
        let assign22890_e25075: f64 = (var_idmult).sqrt();
        (assign22890_e25075,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign22890_e25077;

        let (assign22900_e25090,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard416 != 0.0)) && (var_guard417 == 0.0)) {
        let assign22900_e25088: f64 = (1.0 / var_zinv);
        (assign22900_e25088,)
    } else {
        (var_z,)
    }
};
        var_z = assign22900_e25090;

        let (assign22910_e25100,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard416 != 0.0)) {
        let assign22910_e25098: f64 = (var_idmult - 1.0);
        (assign22910_e25098,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign22910_e25100;

        let assign22920_e25103: f64 = if var_v4 > 0.0 { 1.0 } else { 0.0 };
        var_guard420 = assign22920_e25103;

        let (assign22930_e25129,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard416 != 0.0)) && (var_guard420 != 0.0)) {
        let assign22930_e25115: f64 = (2.0 + var_z);
        let assign22930_e25118: f64 = (var_z + 1.0);
        let assign22930_e25121: f64 = (var_z + 3.0);
        let assign22930_e25122: f64 = (assign22930_e25118 * assign22930_e25121);
        let assign22930_e25123: f64 = (assign22930_e25122).sqrt();
        let assign22930_e25124: f64 = (assign22930_e25115 + assign22930_e25123);
        let assign22930_e25125: f64 = (assign22930_e25124).ln();
        let assign22930_e25126: f64 = (var_phitd * assign22930_e25125);
        let assign22930_e25127: f64 = (2.0 * assign22930_e25126);
        (assign22930_e25127,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign22930_e25129;

        let (assign22940_e25163,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard416 != 0.0)) && (var_guard420 == 0.0)) {
        let assign22940_e25139: f64 = (-var_v4);
        let assign22940_e25144: f64 = (2.0 * var_zinv);
        let assign22940_e25146: f64 = (assign22940_e25144 + 1.0);
        let assign22940_e25149: f64 = (1.0 + var_zinv);
        let assign22940_e25153: f64 = (3.0 * var_zinv);
        let assign22940_e25154: f64 = (1.0 + assign22940_e25153);
        let assign22940_e25155: f64 = (assign22940_e25149 * assign22940_e25154);
        let assign22940_e25156: f64 = (assign22940_e25155).sqrt();
        let assign22940_e25157: f64 = (assign22940_e25146 + assign22940_e25156);
        let assign22940_e25158: f64 = (assign22940_e25157).ln();
        let assign22940_e25159: f64 = (var_phitd * assign22940_e25158);
        let assign22940_e25160: f64 = (2.0 * assign22940_e25159);
        let assign22940_e25161: f64 = (assign22940_e25139 + assign22940_e25160);
        (assign22940_e25161,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign22940_e25163;

        let (assign22950_e25173,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard416 != 0.0)) {
        let assign22950_e25171: f64 = (var_vbimin_s - var_two_psistar);
        (assign22950_e25171,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign22950_e25173;

        let (assign22960_e25200,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard416 != 0.0)) {
        let assign22960_e25182: f64 = (var_v4 + var_vjlim);
        let assign22960_e25185: f64 = (var_v4 - var_vjlim);
        let assign22960_e25188: f64 = (var_v4 - var_vjlim);
        let assign22960_e25189: f64 = (assign22960_e25185 * assign22960_e25188);
        let assign22960_e25192: f64 = (4.0 * var_phitd);
        let assign22960_e25194: f64 = (assign22960_e25192 * var_phitd);
        let assign22960_e25195: f64 = (assign22960_e25189 + assign22960_e25194);
        let assign22960_e25196: f64 = (assign22960_e25195).sqrt();
        let assign22960_e25197: f64 = (assign22960_e25182 - assign22960_e25196);
        let assign22960_e25198: f64 = (0.5 * assign22960_e25197);
        (assign22960_e25198,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign22960_e25200;

        let (assign22970_e25227,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard416 != 0.0)) {
        let assign22970_e25209: f64 = (var_v4 + var_vbbtlim_s);
        let assign22970_e25212: f64 = (var_v4 - var_vbbtlim_s);
        let assign22970_e25215: f64 = (var_v4 - var_vbbtlim_s);
        let assign22970_e25216: f64 = (assign22970_e25212 * assign22970_e25215);
        let assign22970_e25219: f64 = (4.0 * var_phitr);
        let assign22970_e25221: f64 = (assign22970_e25219 * var_phitr);
        let assign22970_e25222: f64 = (assign22970_e25216 + assign22970_e25221);
        let assign22970_e25223: f64 = (assign22970_e25222).sqrt();
        let assign22970_e25224: f64 = (assign22970_e25209 - assign22970_e25223);
        let assign22970_e25225: f64 = (0.5 * assign22970_e25224);
        (assign22970_e25225,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign22970_e25227;

        let (assign22980_e25254,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard416 != 0.0)) {
        let assign22980_e25236: f64 = var_v4;
        let assign22980_e25239: f64 = var_v4;
        let assign22980_e25242: f64 = var_v4;
        let assign22980_e25243: f64 = (assign22980_e25239 * assign22980_e25242);
        let assign22980_e25246: f64 = (4.0 * 1e-6);
        let assign22980_e25248: f64 = (assign22980_e25246 * 1e-6);
        let assign22980_e25249: f64 = (assign22980_e25243 + assign22980_e25248);
        let assign22980_e25250: f64 = (assign22980_e25249).sqrt();
        let assign22980_e25251: f64 = (assign22980_e25236 - assign22980_e25250);
        let assign22980_e25252: f64 = (0.5 * assign22980_e25251);
        (assign22980_e25252,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign22980_e25254;

        let assign22990_e25257: f64 = if var_absource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard421 = assign22990_e25257;

        let (assign23000_e25265, assign23000_e25265_d_n5, assign23000_e25265_d_n6, assign23000_e25265_d_n7, assign23000_e25265_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign23000_e25265;
        var_ijunbot_dn5 = assign23000_e25265_d_n5;
        var_ijunbot_dn6 = assign23000_e25265_d_n6;
        var_ijunbot_dn7 = assign23000_e25265_d_n7;
        var_ijunbot_dn8 = assign23000_e25265_d_n8;

        let (assign23010_e25276,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) {
        let assign23010_e25274: f64 = (var_idsatbot * var_idmult);
        (assign23010_e25274,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign23010_e25276;

        let assign23020_e25283: f64 = if ((p.p840 == 0.0) && (p.p845 == 0.0)) { 1.0 } else { 0.0 };
        var_guard422 = assign23020_e25283;

        let (assign23030_e25294, assign23030_e25294_d_n5, assign23030_e25294_d_n6, assign23030_e25294_d_n7, assign23030_e25294_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard422 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign23030_e25294;
        var_isrh_dn5 = assign23030_e25294_d_n5;
        var_isrh_dn6 = assign23030_e25294_d_n6;
        var_isrh_dn7 = assign23030_e25294_d_n7;
        var_isrh_dn8 = assign23030_e25294_d_n8;

        let (assign23040_e25308,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard422 == 0.0)) {
        let assign23040_e25306: f64 = (var_vbibot - var_vjsrh);
        (assign23040_e25306,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign23040_e25308;

        let (assign23050_e25327,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard422 == 0.0)) {
        let assign23050_e25322: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign23050_e25323: f64 = (1.0 - assign23050_e25322);
        let assign23050_e25324: f64 = (assign23050_e25323).sqrt();
        let assign23050_e25325: f64 = (1.0 - assign23050_e25324);
        (assign23050_e25325,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign23050_e25327;

        let assign23060_e25330: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        var_guard423 = assign23060_e25330;

        let (assign23070_e25344,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard422 == 0.0)) && (var_guard423 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign23070_e25344;

        let (assign23080_e25376,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard422 == 0.0)) && (var_guard423 == 0.0)) {
        let assign23080_e25359: f64 = (var_wsrhstep * var_wsrhstep);
        let assign23080_e25361: f64 = (var_wsrhstep).ln();
        let assign23080_e25362: f64 = (assign23080_e25359 * assign23080_e25361);
        let assign23080_e25365: f64 = (1.0 - var_wsrhstep);
        let assign23080_e25366: f64 = (assign23080_e25362 / assign23080_e25365);
        let assign23080_e25368: f64 = (assign23080_e25366 + var_wsrhstep);
        let assign23080_e25372: f64 = (2.0 * p.p831);
        let assign23080_e25373: f64 = (1.0 - assign23080_e25372);
        let assign23080_e25374: f64 = (assign23080_e25368 * assign23080_e25373);
        (assign23080_e25374,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign23080_e25376;

        let (assign23090_e25390,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard422 == 0.0)) {
        let assign23090_e25388: f64 = (var_wsrhstep + var_dwsrh);
        (assign23090_e25388,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign23090_e25390;

        let assign23100_e25393: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        var_guard424 = assign23100_e25393;

        let (assign23110_e25410, assign23110_e25410_d_n5, assign23110_e25410_d_n6, assign23110_e25410_d_n7, assign23110_e25410_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard422 == 0.0)) && (var_guard424 != 0.0)) {
        let assign23110_e25407: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign23110_e25408: f64 = (assign23110_e25407).sqrt();
        (assign23110_e25408, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23110_e25410;
        var_tmp_dn5 = assign23110_e25410_d_n5;
        var_tmp_dn6 = assign23110_e25410_d_n6;
        var_tmp_dn7 = assign23110_e25410_d_n7;
        var_tmp_dn8 = assign23110_e25410_d_n8;

        let (assign23120_e25429, assign23120_e25429_d_n5, assign23120_e25429_d_n6, assign23120_e25429_d_n7, assign23120_e25429_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard422 == 0.0)) && (var_guard424 == 0.0)) {
        let assign23120_e25425: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign23120_e25427: f64 = (assign23120_e25425).powf(p.p831);
        (assign23120_e25427, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23120_e25429;
        var_tmp_dn5 = assign23120_e25429_d_n5;
        var_tmp_dn6 = assign23120_e25429_d_n6;
        var_tmp_dn7 = assign23120_e25429_d_n7;
        var_tmp_dn8 = assign23120_e25429_d_n8;

        let (assign23130_e25443, assign23130_e25443_d_n5, assign23130_e25443_d_n6, assign23130_e25443_d_n7, assign23130_e25443_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard422 == 0.0)) {
        let assign23130_e25441: f64 = (var_wdepnulrbot * var_tmp);
        (assign23130_e25441, (var_wdepnulrbot * var_tmp_dn5), (var_wdepnulrbot * var_tmp_dn6), (var_wdepnulrbot * var_tmp_dn7), (var_wdepnulrbot * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign23130_e25443;
        var_wdep_dn5 = assign23130_e25443_d_n5;
        var_wdep_dn6 = assign23130_e25443_d_n6;
        var_wdep_dn7 = assign23130_e25443_d_n7;
        var_wdep_dn8 = assign23130_e25443_d_n8;

        let (assign23140_e25461, assign23140_e25461_d_n5, assign23140_e25461_d_n6, assign23140_e25461_d_n7, assign23140_e25461_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard422 == 0.0)) {
        let assign23140_e25456: f64 = (var_zinv - 1.0);
        let assign23140_e25458: f64 = (assign23140_e25456 * var_wdep);
        let assign23140_e25459: f64 = (var_ftdbot * assign23140_e25458);
        (assign23140_e25459, (var_ftdbot * (assign23140_e25456 * var_wdep_dn5)), (var_ftdbot * (assign23140_e25456 * var_wdep_dn6)), (var_ftdbot * (assign23140_e25456 * var_wdep_dn7)), (var_ftdbot * (assign23140_e25456 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign23140_e25461;
        var_asrh_dn5 = assign23140_e25461_d_n5;
        var_asrh_dn6 = assign23140_e25461_d_n6;
        var_asrh_dn7 = assign23140_e25461_d_n7;
        var_asrh_dn8 = assign23140_e25461_d_n8;

        let (assign23150_e25477, assign23150_e25477_d_n5, assign23150_e25477_d_n6, assign23150_e25477_d_n7, assign23150_e25477_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard422 == 0.0)) {
        let assign23150_e25474: f64 = (var_asrh * var_wsrh);
        let assign23150_e25475: f64 = (p.p840 * assign23150_e25474);
        (assign23150_e25475, (p.p840 * (var_asrh_dn5 * var_wsrh)), (p.p840 * (var_asrh_dn6 * var_wsrh)), (p.p840 * (var_asrh_dn7 * var_wsrh)), (p.p840 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign23150_e25477;
        var_isrh_dn5 = assign23150_e25477_d_n5;
        var_isrh_dn6 = assign23150_e25477_d_n6;
        var_isrh_dn7 = assign23150_e25477_d_n7;
        var_isrh_dn8 = assign23150_e25477_d_n8;

        let assign23160_e25480: f64 = if p.p845 == 0.0 { 1.0 } else { 0.0 };
        var_guard425 = assign23160_e25480;

        let (assign23170_e25491, assign23170_e25491_d_n5, assign23170_e25491_d_n6, assign23170_e25491_d_n7, assign23170_e25491_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign23170_e25491;
        var_itat_dn5 = assign23170_e25491_d_n5;
        var_itat_dn6 = assign23170_e25491_d_n6;
        var_itat_dn7 = assign23170_e25491_d_n7;
        var_itat_dn8 = assign23170_e25491_d_n8;

        let (assign23180_e25509, assign23180_e25509_d_n5, assign23180_e25509_d_n6, assign23180_e25509_d_n7, assign23180_e25509_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) {
        let assign23180_e25504: f64 = (var_wdep * var_one_minus_pbot);
        let assign23180_e25506: f64 = (assign23180_e25504 / var_vbi_minus_vjsrh);
        let assign23180_e25507: f64 = (var_btatpartbot * assign23180_e25506);
        (assign23180_e25507, (var_btatpartbot * ((var_wdep_dn5 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn6 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn7 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn8 * var_one_minus_pbot) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign23180_e25509;
        var_btat_dn5 = assign23180_e25509_d_n5;
        var_btat_dn6 = assign23180_e25509_d_n6;
        var_btat_dn7 = assign23180_e25509_d_n7;
        var_btat_dn8 = assign23180_e25509_d_n8;

        let (assign23190_e25525, assign23190_e25525_d_n5, assign23190_e25525_d_n6, assign23190_e25525_d_n7, assign23190_e25525_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) {
        let assign23190_e25521: f64 = (0.666666666666667 * var_atatbot);
        let assign23190_e25523: f64 = (assign23190_e25521 / var_btat);
        (assign23190_e25523, (-((assign23190_e25521 * var_btat_dn5) / (var_btat * var_btat))), (-((assign23190_e25521 * var_btat_dn6) / (var_btat * var_btat))), (-((assign23190_e25521 * var_btat_dn7) / (var_btat * var_btat))), (-((assign23190_e25521 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign23190_e25525;
        var_twoatatoverthreebtat_dn5 = assign23190_e25525_d_n5;
        var_twoatatoverthreebtat_dn6 = assign23190_e25525_d_n6;
        var_twoatatoverthreebtat_dn7 = assign23190_e25525_d_n7;
        var_twoatatoverthreebtat_dn8 = assign23190_e25525_d_n8;

        let (assign23200_e25539, assign23200_e25539_d_n5, assign23200_e25539_d_n6, assign23200_e25539_d_n7, assign23200_e25539_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) {
        let assign23200_e25537: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign23200_e25537, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign23200_e25539;
        var_umaxbeforelimiting_dn5 = assign23200_e25539_d_n5;
        var_umaxbeforelimiting_dn6 = assign23200_e25539_d_n6;
        var_umaxbeforelimiting_dn7 = assign23200_e25539_d_n7;
        var_umaxbeforelimiting_dn8 = assign23200_e25539_d_n8;

        let (assign23210_e25560, assign23210_e25560_d_n5, assign23210_e25560_d_n6, assign23210_e25560_d_n7, assign23210_e25560_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) {
        let assign23210_e25551: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign23210_e25554: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign23210_e25556: f64 = (assign23210_e25554 + 1.0);
        let assign23210_e25557: f64 = (assign23210_e25551 / assign23210_e25556);
        let assign23210_e25558: f64 = (assign23210_e25557).sqrt();
        (assign23210_e25558, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign23210_e25556) - (assign23210_e25551 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign23210_e25556 * assign23210_e25556)) / (2.0 * assign23210_e25558)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign23210_e25556) - (assign23210_e25551 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign23210_e25556 * assign23210_e25556)) / (2.0 * assign23210_e25558)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign23210_e25556) - (assign23210_e25551 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign23210_e25556 * assign23210_e25556)) / (2.0 * assign23210_e25558)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign23210_e25556) - (assign23210_e25551 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign23210_e25556 * assign23210_e25556)) / (2.0 * assign23210_e25558)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign23210_e25560;
        var_umax_dn5 = assign23210_e25560_d_n5;
        var_umax_dn6 = assign23210_e25560_d_n6;
        var_umax_dn7 = assign23210_e25560_d_n7;
        var_umax_dn8 = assign23210_e25560_d_n8;

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
        *var_guard419_slot = var_guard419;
        *var_guard420_slot = var_guard420;
        *var_guard421_slot = var_guard421;
        *var_guard422_slot = var_guard422;
        *var_guard423_slot = var_guard423;
        *var_guard424_slot = var_guard424;
        *var_guard425_slot = var_guard425;
        *var_id__blk219_slot = var_id__blk219;
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
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_z_slot = var_z;
        *var_zinv_slot = var_zinv;
    }

    pub(super) fn stamp_transient_block_42(
        p: &Parameters,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatbot: f64,
        var_berfc: f64,
        var_btat: f64,
        var_btat_dn5: f64,
        var_btat_dn6: f64,
        var_btat_dn7: f64,
        var_btat_dn8: f64,
        var_cerfc: f64,
        var_fbbtbot: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard421: f64,
        var_guard425: f64,
        var_one_over_one_minus_pbot: f64,
        var_perfc: f64,
        var_twoatatoverthreebtat: f64,
        var_twoatatoverthreebtat_dn5: f64,
        var_twoatatoverthreebtat_dn6: f64,
        var_twoatatoverthreebtat_dn7: f64,
        var_twoatatoverthreebtat_dn8: f64,
        var_umax: f64,
        var_umax_dn5: f64,
        var_umax_dn6: f64,
        var_umax_dn7: f64,
        var_umax_dn8: f64,
        var_vbbt: f64,
        var_vbirbotinv: f64,
        var_wdepnulrinvbot: f64,
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
        var_guard426_slot: &mut f64,
        var_guard427_slot: &mut f64,
        var_guard428_slot: &mut f64,
        var_guard429_slot: &mut f64,
        var_guard430_slot: &mut f64,
        var_guard431_slot: &mut f64,
        var_guard432_slot: &mut f64,
        var_guard433_slot: &mut f64,
        var_guard434_slot: &mut f64,
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
        let mut var_guard426: f64 = *var_guard426_slot;
        let mut var_guard427: f64 = *var_guard427_slot;
        let mut var_guard428: f64 = *var_guard428_slot;
        let mut var_guard429: f64 = *var_guard429_slot;
        let mut var_guard430: f64 = *var_guard430_slot;
        let mut var_guard431: f64 = *var_guard431_slot;
        let mut var_guard432: f64 = *var_guard432_slot;
        let mut var_guard433: f64 = *var_guard433_slot;
        let mut var_guard434: f64 = *var_guard434_slot;
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

        let (assign23220_e25573, assign23220_e25573_d_n5, assign23220_e25573_d_n6, assign23220_e25573_d_n7, assign23220_e25573_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) {
        let assign23220_e25571: f64 = (var_umax).sqrt();
        (assign23220_e25571, (var_umax_dn5 / (2.0 * assign23220_e25571)), (var_umax_dn6 / (2.0 * assign23220_e25571)), (var_umax_dn7 / (2.0 * assign23220_e25571)), (var_umax_dn8 / (2.0 * assign23220_e25571)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign23220_e25573;
        var_sqrtumax_dn5 = assign23220_e25573_d_n5;
        var_sqrtumax_dn6 = assign23220_e25573_d_n6;
        var_sqrtumax_dn7 = assign23220_e25573_d_n7;
        var_sqrtumax_dn8 = assign23220_e25573_d_n8;

        let (assign23230_e25587, assign23230_e25587_d_n5, assign23230_e25587_d_n6, assign23230_e25587_d_n7, assign23230_e25587_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) {
        let assign23230_e25585: f64 = (var_umax * var_sqrtumax);
        (assign23230_e25585, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign23230_e25587;
        var_umaxpoweronepointfive_dn5 = assign23230_e25587_d_n5;
        var_umaxpoweronepointfive_dn6 = assign23230_e25587_d_n6;
        var_umaxpoweronepointfive_dn7 = assign23230_e25587_d_n7;
        var_umaxpoweronepointfive_dn8 = assign23230_e25587_d_n8;

        let assign23240_e25589: f64 = (-p.p831);
        let assign23240_e25591: f64 = (assign23240_e25589 * var_one_over_one_minus_pbot);
        let assign23240_e25593: f64 = (-1.0);
        let assign23240_e25594: f64 = if assign23240_e25591 == assign23240_e25593 { 1.0 } else { 0.0 };
        var_guard426 = assign23240_e25594;

        let (assign23250_e25614, assign23250_e25614_d_n5, assign23250_e25614_d_n6, assign23250_e25614_d_n7, assign23250_e25614_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) && (var_guard426 != 0.0)) {
        let assign23250_e25610: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign23250_e25611: f64 = (1.0 + assign23250_e25610);
        let assign23250_e25612: f64 = (1.0 / assign23250_e25611);
        (assign23250_e25612, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign23250_e25611 * assign23250_e25611))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign23250_e25611 * assign23250_e25611))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign23250_e25611 * assign23250_e25611))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign23250_e25611 * assign23250_e25611))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign23250_e25614;
        var_wgamma_dn5 = assign23250_e25614_d_n5;
        var_wgamma_dn6 = assign23250_e25614_d_n6;
        var_wgamma_dn7 = assign23250_e25614_d_n7;
        var_wgamma_dn8 = assign23250_e25614_d_n8;

        let (assign23260_e25638, assign23260_e25638_d_n5, assign23260_e25638_d_n6, assign23260_e25638_d_n7, assign23260_e25638_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) && (var_guard426 == 0.0)) {
        let assign23260_e25630: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign23260_e25631: f64 = (1.0 + assign23260_e25630);
        let assign23260_e25633: f64 = (-p.p831);
        let assign23260_e25635: f64 = (assign23260_e25633 * var_one_over_one_minus_pbot);
        let assign23260_e25636: f64 = (assign23260_e25631).powf(assign23260_e25635);
        (assign23260_e25636, if 0.0 == 0.0 && ((assign23260_e25635) as f64).is_finite() && ((assign23260_e25635) as f64).fract() == 0.0 { if assign23260_e25635 == 0.0 { 0.0 } else { (assign23260_e25635 * ((assign23260_e25631).powf(assign23260_e25635 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign23260_e25636 * (assign23260_e25635 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign23260_e25631))) }, if 0.0 == 0.0 && ((assign23260_e25635) as f64).is_finite() && ((assign23260_e25635) as f64).fract() == 0.0 { if assign23260_e25635 == 0.0 { 0.0 } else { (assign23260_e25635 * ((assign23260_e25631).powf(assign23260_e25635 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign23260_e25636 * (assign23260_e25635 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign23260_e25631))) }, if 0.0 == 0.0 && ((assign23260_e25635) as f64).is_finite() && ((assign23260_e25635) as f64).fract() == 0.0 { if assign23260_e25635 == 0.0 { 0.0 } else { (assign23260_e25635 * ((assign23260_e25631).powf(assign23260_e25635 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign23260_e25636 * (assign23260_e25635 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign23260_e25631))) }, if 0.0 == 0.0 && ((assign23260_e25635) as f64).is_finite() && ((assign23260_e25635) as f64).fract() == 0.0 { if assign23260_e25635 == 0.0 { 0.0 } else { (assign23260_e25635 * ((assign23260_e25631).powf(assign23260_e25635 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign23260_e25636 * (assign23260_e25635 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign23260_e25631))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign23260_e25638;
        var_wgamma_dn5 = assign23260_e25638_d_n5;
        var_wgamma_dn6 = assign23260_e25638_d_n6;
        var_wgamma_dn7 = assign23260_e25638_d_n7;
        var_wgamma_dn8 = assign23260_e25638_d_n8;

        let (assign23270_e25656, assign23270_e25656_d_n5, assign23270_e25656_d_n6, assign23270_e25656_d_n7, assign23270_e25656_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) {
        let assign23270_e25650: f64 = (var_wsrh * var_wgamma);
        let assign23270_e25653: f64 = (var_wsrh + var_wgamma);
        let assign23270_e25654: f64 = (assign23270_e25650 / assign23270_e25653);
        (assign23270_e25654, ((((var_wsrh * var_wgamma_dn5) * assign23270_e25653) - (assign23270_e25650 * var_wgamma_dn5)) / (assign23270_e25653 * assign23270_e25653)), ((((var_wsrh * var_wgamma_dn6) * assign23270_e25653) - (assign23270_e25650 * var_wgamma_dn6)) / (assign23270_e25653 * assign23270_e25653)), ((((var_wsrh * var_wgamma_dn7) * assign23270_e25653) - (assign23270_e25650 * var_wgamma_dn7)) / (assign23270_e25653 * assign23270_e25653)), ((((var_wsrh * var_wgamma_dn8) * assign23270_e25653) - (assign23270_e25650 * var_wgamma_dn8)) / (assign23270_e25653 * assign23270_e25653)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign23270_e25656;
        var_wtat_dn5 = assign23270_e25656_d_n5;
        var_wtat_dn6 = assign23270_e25656_d_n6;
        var_wtat_dn7 = assign23270_e25656_d_n7;
        var_wtat_dn8 = assign23270_e25656_d_n8;

        let (assign23280_e25673, assign23280_e25673_d_n5, assign23280_e25673_d_n6, assign23280_e25673_d_n7, assign23280_e25673_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) {
        let assign23280_e25669: f64 = (var_btat / var_sqrtumax);
        let assign23280_e25670: f64 = (0.375 * assign23280_e25669);
        let assign23280_e25671: f64 = (assign23280_e25670).sqrt();
        (assign23280_e25671, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign23280_e25671)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign23280_e25671)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign23280_e25671)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign23280_e25671)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign23280_e25673;
        var_ktat_dn5 = assign23280_e25673_d_n5;
        var_ktat_dn6 = assign23280_e25673_d_n6;
        var_ktat_dn7 = assign23280_e25673_d_n7;
        var_ktat_dn8 = assign23280_e25673_d_n8;

        let (assign23290_e25691, assign23290_e25691_d_n5, assign23290_e25691_d_n6, assign23290_e25691_d_n7, assign23290_e25691_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) {
        let assign23290_e25686: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign23290_e25687: f64 = (2.0 * assign23290_e25686);
        let assign23290_e25689: f64 = (assign23290_e25687 - var_umax);
        (assign23290_e25689, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign23290_e25691;
        var_ltat_dn5 = assign23290_e25691_d_n5;
        var_ltat_dn6 = assign23290_e25691_d_n6;
        var_ltat_dn7 = assign23290_e25691_d_n7;
        var_ltat_dn8 = assign23290_e25691_d_n8;

        let (assign23300_e25717, assign23300_e25717_d_n5, assign23300_e25717_d_n6, assign23300_e25717_d_n7, assign23300_e25717_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) {
        let assign23300_e25703: f64 = (var_atatbot * var_twoatatoverthreebtat);
        let assign23300_e25705: f64 = (assign23300_e25703 * var_sqrtumax);
        let assign23300_e25708: f64 = (var_atatbot * var_umax);
        let assign23300_e25709: f64 = (assign23300_e25705 - assign23300_e25708);
        let assign23300_e25713: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign23300_e25714: f64 = (0.5 * assign23300_e25713);
        let assign23300_e25715: f64 = (assign23300_e25709 + assign23300_e25714);
        (assign23300_e25715, (((((var_atatbot * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign23300_e25703 * var_sqrtumax_dn5)) - (var_atatbot * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatbot * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign23300_e25703 * var_sqrtumax_dn6)) - (var_atatbot * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign23300_e25703 * var_sqrtumax_dn7)) - (var_atatbot * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign23300_e25703 * var_sqrtumax_dn8)) - (var_atatbot * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign23300_e25717;
        var_mtat_dn5 = assign23300_e25717_d_n5;
        var_mtat_dn6 = assign23300_e25717_d_n6;
        var_mtat_dn7 = assign23300_e25717_d_n7;
        var_mtat_dn8 = assign23300_e25717_d_n8;

        let (assign23310_e25733, assign23310_e25733_d_n5, assign23310_e25733_d_n6, assign23310_e25733_d_n7, assign23310_e25733_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) {
        let assign23310_e25729: f64 = (var_ltat - 1.0);
        let assign23310_e25731: f64 = (assign23310_e25729 * var_ktat);
        (assign23310_e25731, ((var_ltat_dn5 * var_ktat) + (assign23310_e25729 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign23310_e25729 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign23310_e25729 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign23310_e25729 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign23310_e25733;
        var_xerfc_dn5 = assign23310_e25733_d_n5;
        var_xerfc_dn6 = assign23310_e25733_d_n6;
        var_xerfc_dn7 = assign23310_e25733_d_n7;
        var_xerfc_dn8 = assign23310_e25733_d_n8;

        let (assign23320_e25747, assign23320_e25747_d_n5, assign23320_e25747_d_n6, assign23320_e25747_d_n7, assign23320_e25747_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) {
        let assign23320_e25745: f64 = (var_xerfc * var_xerfc);
        (assign23320_e25745, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign23320_e25747;
        var_ysq_dn5 = assign23320_e25747_d_n5;
        var_ysq_dn6 = assign23320_e25747_d_n6;
        var_ysq_dn7 = assign23320_e25747_d_n7;
        var_ysq_dn8 = assign23320_e25747_d_n8;

        let assign23330_e25750: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard427 = assign23330_e25750;

        let (assign23340_e25770, assign23340_e25770_d_n5, assign23340_e25770_d_n6, assign23340_e25770_d_n7, assign23340_e25770_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) && (var_guard427 != 0.0)) {
        let assign23340_e25766: f64 = (var_perfc * var_xerfc);
        let assign23340_e25767: f64 = (1.0 + assign23340_e25766);
        let assign23340_e25768: f64 = (1.0 / assign23340_e25767);
        (assign23340_e25768, (-((var_perfc * var_xerfc_dn5) / (assign23340_e25767 * assign23340_e25767))), (-((var_perfc * var_xerfc_dn6) / (assign23340_e25767 * assign23340_e25767))), (-((var_perfc * var_xerfc_dn7) / (assign23340_e25767 * assign23340_e25767))), (-((var_perfc * var_xerfc_dn8) / (assign23340_e25767 * assign23340_e25767))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign23340_e25770;
        var_terfc_dn5 = assign23340_e25770_d_n5;
        var_terfc_dn6 = assign23340_e25770_d_n6;
        var_terfc_dn7 = assign23340_e25770_d_n7;
        var_terfc_dn8 = assign23340_e25770_d_n8;

        let (assign23350_e25791, assign23350_e25791_d_n5, assign23350_e25791_d_n6, assign23350_e25791_d_n7, assign23350_e25791_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) && (var_guard427 == 0.0)) {
        let assign23350_e25787: f64 = (var_perfc * var_xerfc);
        let assign23350_e25788: f64 = (1.0 - assign23350_e25787);
        let assign23350_e25789: f64 = (1.0 / assign23350_e25788);
        (assign23350_e25789, (-((-(var_perfc * var_xerfc_dn5)) / (assign23350_e25788 * assign23350_e25788))), (-((-(var_perfc * var_xerfc_dn6)) / (assign23350_e25788 * assign23350_e25788))), (-((-(var_perfc * var_xerfc_dn7)) / (assign23350_e25788 * assign23350_e25788))), (-((-(var_perfc * var_xerfc_dn8)) / (assign23350_e25788 * assign23350_e25788))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign23350_e25791;
        var_terfc_dn5 = assign23350_e25791_d_n5;
        var_terfc_dn6 = assign23350_e25791_d_n6;
        var_terfc_dn7 = assign23350_e25791_d_n7;
        var_terfc_dn8 = assign23350_e25791_d_n8;

        let assign23360_e25793: f64 = (-var_ysq);
        let assign23360_e25795: f64 = (assign23360_e25793 + var_mtat);
        let assign23360_e25797: f64 = (-230.25850929940458);
        let assign23360_e25798: f64 = if assign23360_e25795 > assign23360_e25797 { 1.0 } else { 0.0 };
        var_guard428 = assign23360_e25798;

        let (assign23370_e25816, assign23370_e25816_d_n5, assign23370_e25816_d_n6, assign23370_e25816_d_n7, assign23370_e25816_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) && (var_guard428 != 0.0)) {
        let assign23370_e25811: f64 = (-var_ysq);
        let assign23370_e25813: f64 = (assign23370_e25811 + var_mtat);
        let assign23370_e25814: f64 = (assign23370_e25813).exp();
        (assign23370_e25814, (assign23370_e25814 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign23370_e25814 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign23370_e25814 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign23370_e25814 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23370_e25816;
        var_tmp_dn5 = assign23370_e25816_d_n5;
        var_tmp_dn6 = assign23370_e25816_d_n6;
        var_tmp_dn7 = assign23370_e25816_d_n7;
        var_tmp_dn8 = assign23370_e25816_d_n8;

        let (assign23380_e25865, assign23380_e25865_d_n5, assign23380_e25865_d_n6, assign23380_e25865_d_n7, assign23380_e25865_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) && (var_guard428 == 0.0)) {
        let assign23380_e25832: f64 = (-230.25850929940458);
        let assign23380_e25834: f64 = (-var_ysq);
        let assign23380_e25836: f64 = (assign23380_e25834 + var_mtat);
        let assign23380_e25837: f64 = (assign23380_e25832 - assign23380_e25836);
        let assign23380_e25841: f64 = (-230.25850929940458);
        let assign23380_e25843: f64 = (-var_ysq);
        let assign23380_e25845: f64 = (assign23380_e25843 + var_mtat);
        let assign23380_e25846: f64 = (assign23380_e25841 - assign23380_e25845);
        let assign23380_e25849: f64 = (-230.25850929940458);
        let assign23380_e25851: f64 = (-var_ysq);
        let assign23380_e25853: f64 = (assign23380_e25851 + var_mtat);
        let assign23380_e25854: f64 = (assign23380_e25849 - assign23380_e25853);
        let assign23380_e25856: f64 = (assign23380_e25854 * 0.3333333333333333);
        let assign23380_e25857: f64 = (1.0 + assign23380_e25856);
        let assign23380_e25858: f64 = (assign23380_e25846 * assign23380_e25857);
        let assign23380_e25859: f64 = (0.5 * assign23380_e25858);
        let assign23380_e25860: f64 = (1.0 + assign23380_e25859);
        let assign23380_e25861: f64 = (assign23380_e25837 * assign23380_e25860);
        let assign23380_e25862: f64 = (1.0 + assign23380_e25861);
        let assign23380_e25863: f64 = (1e-100 / assign23380_e25862);
        (assign23380_e25863, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign23380_e25860) + (assign23380_e25837 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign23380_e25857) + (assign23380_e25846 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign23380_e25862 * assign23380_e25862))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign23380_e25860) + (assign23380_e25837 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign23380_e25857) + (assign23380_e25846 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign23380_e25862 * assign23380_e25862))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign23380_e25860) + (assign23380_e25837 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign23380_e25857) + (assign23380_e25846 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign23380_e25862 * assign23380_e25862))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign23380_e25860) + (assign23380_e25837 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign23380_e25857) + (assign23380_e25846 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign23380_e25862 * assign23380_e25862))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23380_e25865;
        var_tmp_dn5 = assign23380_e25865_d_n5;
        var_tmp_dn6 = assign23380_e25865_d_n6;
        var_tmp_dn7 = assign23380_e25865_d_n7;
        var_tmp_dn8 = assign23380_e25865_d_n8;

        let (assign23390_e25895, assign23390_e25895_d_n5, assign23390_e25895_d_n6, assign23390_e25895_d_n7, assign23390_e25895_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) {
        let assign23390_e25877: f64 = (0.29214664 * var_terfc);
        let assign23390_e25881: f64 = (var_terfc * var_terfc);
        let assign23390_e25882: f64 = (var_berfc * assign23390_e25881);
        let assign23390_e25883: f64 = (assign23390_e25877 + assign23390_e25882);
        let assign23390_e25887: f64 = (var_terfc * var_terfc);
        let assign23390_e25889: f64 = (assign23390_e25887 * var_terfc);
        let assign23390_e25890: f64 = (var_cerfc * assign23390_e25889);
        let assign23390_e25891: f64 = (assign23390_e25883 + assign23390_e25890);
        let assign23390_e25893: f64 = (assign23390_e25891 * var_tmp);
        (assign23390_e25893, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign23390_e25887 * var_terfc_dn5)))) * var_tmp) + (assign23390_e25891 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign23390_e25887 * var_terfc_dn6)))) * var_tmp) + (assign23390_e25891 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign23390_e25887 * var_terfc_dn7)))) * var_tmp) + (assign23390_e25891 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign23390_e25887 * var_terfc_dn8)))) * var_tmp) + (assign23390_e25891 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign23390_e25895;
        var_erfcpos_dn5 = assign23390_e25895_d_n5;
        var_erfcpos_dn6 = assign23390_e25895_d_n6;
        var_erfcpos_dn7 = assign23390_e25895_d_n7;
        var_erfcpos_dn8 = assign23390_e25895_d_n8;

        let assign23400_e25898: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard429 = assign23400_e25898;

        let (assign23410_e25912, assign23410_e25912_d_n5, assign23410_e25912_d_n6, assign23410_e25912_d_n7, assign23410_e25912_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) && (var_guard429 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign23410_e25912;
        var_erfctimesexpmtat_dn5 = assign23410_e25912_d_n5;
        var_erfctimesexpmtat_dn6 = assign23410_e25912_d_n6;
        var_erfctimesexpmtat_dn7 = assign23410_e25912_d_n7;
        var_erfctimesexpmtat_dn8 = assign23410_e25912_d_n8;

        let assign23420_e25915: f64 = (-230.25850929940458);
        let assign23420_e25916: f64 = if var_mtat > assign23420_e25915 { 1.0 } else { 0.0 };
        var_guard430 = assign23420_e25916;

        let (assign23430_e25934, assign23430_e25934_d_n5, assign23430_e25934_d_n6, assign23430_e25934_d_n7, assign23430_e25934_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) && (var_guard429 == 0.0)) && (var_guard430 != 0.0)) {
        let assign23430_e25932: f64 = (var_mtat).exp();
        (assign23430_e25932, (assign23430_e25932 * var_mtat_dn5), (assign23430_e25932 * var_mtat_dn6), (assign23430_e25932 * var_mtat_dn7), (assign23430_e25932 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23430_e25934;
        var_tmp_dn5 = assign23430_e25934_d_n5;
        var_tmp_dn6 = assign23430_e25934_d_n6;
        var_tmp_dn7 = assign23430_e25934_d_n7;
        var_tmp_dn8 = assign23430_e25934_d_n8;

        let (assign23440_e25977, assign23440_e25977_d_n5, assign23440_e25977_d_n6, assign23440_e25977_d_n7, assign23440_e25977_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) && (var_guard429 == 0.0)) && (var_guard430 == 0.0)) {
        let assign23440_e25953: f64 = (-230.25850929940458);
        let assign23440_e25955: f64 = (assign23440_e25953 - var_mtat);
        let assign23440_e25959: f64 = (-230.25850929940458);
        let assign23440_e25961: f64 = (assign23440_e25959 - var_mtat);
        let assign23440_e25964: f64 = (-230.25850929940458);
        let assign23440_e25966: f64 = (assign23440_e25964 - var_mtat);
        let assign23440_e25968: f64 = (assign23440_e25966 * 0.3333333333333333);
        let assign23440_e25969: f64 = (1.0 + assign23440_e25968);
        let assign23440_e25970: f64 = (assign23440_e25961 * assign23440_e25969);
        let assign23440_e25971: f64 = (0.5 * assign23440_e25970);
        let assign23440_e25972: f64 = (1.0 + assign23440_e25971);
        let assign23440_e25973: f64 = (assign23440_e25955 * assign23440_e25972);
        let assign23440_e25974: f64 = (1.0 + assign23440_e25973);
        let assign23440_e25975: f64 = (1e-100 / assign23440_e25974);
        (assign23440_e25975, (-((1e-100 * (((-var_mtat_dn5) * assign23440_e25972) + (assign23440_e25955 * (0.5 * (((-var_mtat_dn5) * assign23440_e25969) + (assign23440_e25961 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign23440_e25974 * assign23440_e25974))), (-((1e-100 * (((-var_mtat_dn6) * assign23440_e25972) + (assign23440_e25955 * (0.5 * (((-var_mtat_dn6) * assign23440_e25969) + (assign23440_e25961 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign23440_e25974 * assign23440_e25974))), (-((1e-100 * (((-var_mtat_dn7) * assign23440_e25972) + (assign23440_e25955 * (0.5 * (((-var_mtat_dn7) * assign23440_e25969) + (assign23440_e25961 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign23440_e25974 * assign23440_e25974))), (-((1e-100 * (((-var_mtat_dn8) * assign23440_e25972) + (assign23440_e25955 * (0.5 * (((-var_mtat_dn8) * assign23440_e25969) + (assign23440_e25961 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign23440_e25974 * assign23440_e25974))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23440_e25977;
        var_tmp_dn5 = assign23440_e25977_d_n5;
        var_tmp_dn6 = assign23440_e25977_d_n6;
        var_tmp_dn7 = assign23440_e25977_d_n7;
        var_tmp_dn8 = assign23440_e25977_d_n8;

        let (assign23450_e25996, assign23450_e25996_d_n5, assign23450_e25996_d_n6, assign23450_e25996_d_n7, assign23450_e25996_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) && (var_guard429 == 0.0)) {
        let assign23450_e25992: f64 = (2.0 * var_tmp);
        let assign23450_e25994: f64 = (assign23450_e25992 - var_erfcpos);
        (assign23450_e25994, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign23450_e25996;
        var_erfctimesexpmtat_dn5 = assign23450_e25996_d_n5;
        var_erfctimesexpmtat_dn6 = assign23450_e25996_d_n6;
        var_erfctimesexpmtat_dn7 = assign23450_e25996_d_n7;
        var_erfctimesexpmtat_dn8 = assign23450_e25996_d_n8;

        let (assign23460_e26016, assign23460_e26016_d_n5, assign23460_e26016_d_n6, assign23460_e26016_d_n7, assign23460_e26016_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) {
        let assign23460_e26008: f64 = (1.772453850905516 * 0.5);
        let assign23460_e26011: f64 = (var_atatbot * var_erfctimesexpmtat);
        let assign23460_e26013: f64 = (assign23460_e26011 / var_ktat);
        let assign23460_e26014: f64 = (assign23460_e26008 * assign23460_e26013);
        (assign23460_e26014, (assign23460_e26008 * ((((var_atatbot * var_erfctimesexpmtat_dn5) * var_ktat) - (assign23460_e26011 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign23460_e26008 * ((((var_atatbot * var_erfctimesexpmtat_dn6) * var_ktat) - (assign23460_e26011 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign23460_e26008 * ((((var_atatbot * var_erfctimesexpmtat_dn7) * var_ktat) - (assign23460_e26011 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign23460_e26008 * ((((var_atatbot * var_erfctimesexpmtat_dn8) * var_ktat) - (assign23460_e26011 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign23460_e26016;
        var_gammamax_dn5 = assign23460_e26016_d_n5;
        var_gammamax_dn6 = assign23460_e26016_d_n6;
        var_gammamax_dn7 = assign23460_e26016_d_n7;
        var_gammamax_dn8 = assign23460_e26016_d_n8;

        let (assign23470_e26034, assign23470_e26034_d_n5, assign23470_e26034_d_n6, assign23470_e26034_d_n7, assign23470_e26034_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard425 == 0.0)) {
        let assign23470_e26029: f64 = (var_asrh * var_gammamax);
        let assign23470_e26031: f64 = (assign23470_e26029 * var_wtat);
        let assign23470_e26032: f64 = (p.p845 * assign23470_e26031);
        (assign23470_e26032, (p.p845 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign23470_e26029 * var_wtat_dn5))), (p.p845 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign23470_e26029 * var_wtat_dn6))), (p.p845 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign23470_e26029 * var_wtat_dn7))), (p.p845 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign23470_e26029 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign23470_e26034;
        var_itat_dn5 = assign23470_e26034_d_n5;
        var_itat_dn6 = assign23470_e26034_d_n6;
        var_itat_dn7 = assign23470_e26034_d_n7;
        var_itat_dn8 = assign23470_e26034_d_n8;

        let assign23480_e26037: f64 = if p.p851 == 0.0 { 1.0 } else { 0.0 };
        var_guard431 = assign23480_e26037;

        let (assign23490_e26048, assign23490_e26048_d_n5, assign23490_e26048_d_n6, assign23490_e26048_d_n7, assign23490_e26048_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard431 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign23490_e26048;
        var_ibbt_dn5 = assign23490_e26048_d_n5;
        var_ibbt_dn6 = assign23490_e26048_d_n6;
        var_ibbt_dn7 = assign23490_e26048_d_n7;
        var_ibbt_dn8 = assign23490_e26048_d_n8;

        let assign23500_e26051: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        var_guard432 = assign23500_e26051;

        let (assign23510_e26070, assign23510_e26070_d_n5, assign23510_e26070_d_n6, assign23510_e26070_d_n7, assign23510_e26070_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard431 == 0.0)) && (var_guard432 != 0.0)) {
        let assign23510_e26065: f64 = (p.p828 - var_vbbt);
        let assign23510_e26067: f64 = (assign23510_e26065 * var_vbirbotinv);
        let assign23510_e26068: f64 = (assign23510_e26067).sqrt();
        (assign23510_e26068, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23510_e26070;
        var_tmp_dn5 = assign23510_e26070_d_n5;
        var_tmp_dn6 = assign23510_e26070_d_n6;
        var_tmp_dn7 = assign23510_e26070_d_n7;
        var_tmp_dn8 = assign23510_e26070_d_n8;

        let (assign23520_e26091, assign23520_e26091_d_n5, assign23520_e26091_d_n6, assign23520_e26091_d_n7, assign23520_e26091_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard431 == 0.0)) && (var_guard432 == 0.0)) {
        let assign23520_e26085: f64 = (p.p828 - var_vbbt);
        let assign23520_e26087: f64 = (assign23520_e26085 * var_vbirbotinv);
        let assign23520_e26089: f64 = (assign23520_e26087).powf(p.p831);
        (assign23520_e26089, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23520_e26091;
        var_tmp_dn5 = assign23520_e26091_d_n5;
        var_tmp_dn6 = assign23520_e26091_d_n6;
        var_tmp_dn7 = assign23520_e26091_d_n7;
        var_tmp_dn8 = assign23520_e26091_d_n8;

        let (assign23530_e26111, assign23530_e26111_d_n5, assign23530_e26111_d_n6, assign23530_e26111_d_n7, assign23530_e26111_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard431 == 0.0)) {
        let assign23530_e26104: f64 = (p.p828 - var_vbbt);
        let assign23530_e26106: f64 = (assign23530_e26104 * var_wdepnulrinvbot);
        let assign23530_e26108: f64 = (assign23530_e26106 / var_tmp);
        let assign23530_e26109: f64 = (var_one_over_one_minus_pbot * assign23530_e26108);
        (assign23530_e26109, (var_one_over_one_minus_pbot * (-((assign23530_e26106 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign23530_e26106 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign23530_e26106 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign23530_e26106 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign23530_e26111;
        var_fmaxr_dn5 = assign23530_e26111_d_n5;
        var_fmaxr_dn6 = assign23530_e26111_d_n6;
        var_fmaxr_dn7 = assign23530_e26111_d_n7;
        var_fmaxr_dn8 = assign23530_e26111_d_n8;

        let assign23540_e26113: f64 = (-var_fbbtbot);
        let assign23540_e26115: f64 = (assign23540_e26113 / var_fmaxr);
        let assign23540_e26116: f64 = (assign23540_e26115).abs();
        let assign23540_e26118: f64 = if assign23540_e26116 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard433 = assign23540_e26118;

        let (assign23550_e26136, assign23550_e26136_d_n5, assign23550_e26136_d_n6, assign23550_e26136_d_n7, assign23550_e26136_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard431 == 0.0)) && (var_guard433 != 0.0)) {
        let assign23550_e26131: f64 = (-var_fbbtbot);
        let assign23550_e26133: f64 = (assign23550_e26131 / var_fmaxr);
        let assign23550_e26134: f64 = (assign23550_e26133).exp();
        (assign23550_e26134, (assign23550_e26134 * (-((assign23550_e26131 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign23550_e26134 * (-((assign23550_e26131 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign23550_e26134 * (-((assign23550_e26131 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign23550_e26134 * (-((assign23550_e26131 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23550_e26136;
        var_tmp_dn5 = assign23550_e26136_d_n5;
        var_tmp_dn6 = assign23550_e26136_d_n6;
        var_tmp_dn7 = assign23550_e26136_d_n7;
        var_tmp_dn8 = assign23550_e26136_d_n8;

        let assign23560_e26138: f64 = (-var_fbbtbot);
        let assign23560_e26140: f64 = (assign23560_e26138 / var_fmaxr);
        let assign23560_e26142: f64 = if assign23560_e26140 < 0.0 { 1.0 } else { 0.0 };
        var_guard434 = assign23560_e26142;

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
        *var_guard426_slot = var_guard426;
        *var_guard427_slot = var_guard427;
        *var_guard428_slot = var_guard428;
        *var_guard429_slot = var_guard429;
        *var_guard430_slot = var_guard430;
        *var_guard431_slot = var_guard431;
        *var_guard432_slot = var_guard432;
        *var_guard433_slot = var_guard433;
        *var_guard434_slot = var_guard434;
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

    pub(super) fn stamp_transient_block_43(
        p: &Parameters,
        var_alphaav: f64,
        var_atatsti: f64,
        var_btatpartsti: f64,
        var_fbbtbot: f64,
        var_fmaxr: f64,
        var_fmaxr_dn5: f64,
        var_fmaxr_dn6: f64,
        var_fmaxr_dn7: f64,
        var_fmaxr_dn8: f64,
        var_fstopbot: f64,
        var_ftdsti: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard421: f64,
        var_guard431: f64,
        var_guard433: f64,
        var_guard434: f64,
        var_idmult: f64,
        var_idsatsti: f64,
        var_lssource_i: f64,
        var_one_minus_psti: f64,
        var_one_over_one_minus_psti: f64,
        var_slopebot: f64,
        var_two_psistar: f64,
        var_v4: f64,
        var_vav: f64,
        var_vbirstiinv: f64,
        var_vbisti: f64,
        var_vbrinvbot: f64,
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
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_guard435_slot: &mut f64,
        var_guard436_slot: &mut f64,
        var_guard437_slot: &mut f64,
        var_guard438_slot: &mut f64,
        var_guard439_slot: &mut f64,
        var_guard440_slot: &mut f64,
        var_guard441_slot: &mut f64,
        var_guard442_slot: &mut f64,
        var_guard443_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn5_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
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
        let mut var_guard435: f64 = *var_guard435_slot;
        let mut var_guard436: f64 = *var_guard436_slot;
        let mut var_guard437: f64 = *var_guard437_slot;
        let mut var_guard438: f64 = *var_guard438_slot;
        let mut var_guard439: f64 = *var_guard439_slot;
        let mut var_guard440: f64 = *var_guard440_slot;
        let mut var_guard441: f64 = *var_guard441_slot;
        let mut var_guard442: f64 = *var_guard442_slot;
        let mut var_guard443: f64 = *var_guard443_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn5: f64 = *var_ibbt_dn5_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
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

        let (assign23570_e26193, assign23570_e26193_d_n5, assign23570_e26193_d_n6, assign23570_e26193_d_n7, assign23570_e26193_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard431 == 0.0)) && (var_guard433 == 0.0)) && (var_guard434 != 0.0)) {
        let assign23570_e26160: f64 = (-230.25850929940458);
        let assign23570_e26162: f64 = (-var_fbbtbot);
        let assign23570_e26164: f64 = (assign23570_e26162 / var_fmaxr);
        let assign23570_e26165: f64 = (assign23570_e26160 - assign23570_e26164);
        let assign23570_e26169: f64 = (-230.25850929940458);
        let assign23570_e26171: f64 = (-var_fbbtbot);
        let assign23570_e26173: f64 = (assign23570_e26171 / var_fmaxr);
        let assign23570_e26174: f64 = (assign23570_e26169 - assign23570_e26173);
        let assign23570_e26177: f64 = (-230.25850929940458);
        let assign23570_e26179: f64 = (-var_fbbtbot);
        let assign23570_e26181: f64 = (assign23570_e26179 / var_fmaxr);
        let assign23570_e26182: f64 = (assign23570_e26177 - assign23570_e26181);
        let assign23570_e26184: f64 = (assign23570_e26182 * 0.3333333333333333);
        let assign23570_e26185: f64 = (1.0 + assign23570_e26184);
        let assign23570_e26186: f64 = (assign23570_e26174 * assign23570_e26185);
        let assign23570_e26187: f64 = (0.5 * assign23570_e26186);
        let assign23570_e26188: f64 = (1.0 + assign23570_e26187);
        let assign23570_e26189: f64 = (assign23570_e26165 * assign23570_e26188);
        let assign23570_e26190: f64 = (1.0 + assign23570_e26189);
        let assign23570_e26191: f64 = (1e-100 / assign23570_e26190);
        (assign23570_e26191, (-((1e-100 * (((-(-((assign23570_e26162 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign23570_e26188) + (assign23570_e26165 * (0.5 * (((-(-((assign23570_e26171 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign23570_e26185) + (assign23570_e26174 * ((-(-((assign23570_e26179 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign23570_e26190 * assign23570_e26190))), (-((1e-100 * (((-(-((assign23570_e26162 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign23570_e26188) + (assign23570_e26165 * (0.5 * (((-(-((assign23570_e26171 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign23570_e26185) + (assign23570_e26174 * ((-(-((assign23570_e26179 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign23570_e26190 * assign23570_e26190))), (-((1e-100 * (((-(-((assign23570_e26162 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign23570_e26188) + (assign23570_e26165 * (0.5 * (((-(-((assign23570_e26171 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign23570_e26185) + (assign23570_e26174 * ((-(-((assign23570_e26179 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign23570_e26190 * assign23570_e26190))), (-((1e-100 * (((-(-((assign23570_e26162 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign23570_e26188) + (assign23570_e26165 * (0.5 * (((-(-((assign23570_e26171 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign23570_e26185) + (assign23570_e26174 * ((-(-((assign23570_e26179 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign23570_e26190 * assign23570_e26190))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23570_e26193;
        var_tmp_dn5 = assign23570_e26193_d_n5;
        var_tmp_dn6 = assign23570_e26193_d_n6;
        var_tmp_dn7 = assign23570_e26193_d_n7;
        var_tmp_dn8 = assign23570_e26193_d_n8;

        let (assign23580_e26242, assign23580_e26242_d_n5, assign23580_e26242_d_n6, assign23580_e26242_d_n7, assign23580_e26242_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard431 == 0.0)) && (var_guard433 == 0.0)) && (var_guard434 == 0.0)) {
        let assign23580_e26212: f64 = (-var_fbbtbot);
        let assign23580_e26214: f64 = (assign23580_e26212 / var_fmaxr);
        let assign23580_e26216: f64 = (assign23580_e26214 - 230.25850929940458);
        let assign23580_e26220: f64 = (-var_fbbtbot);
        let assign23580_e26222: f64 = (assign23580_e26220 / var_fmaxr);
        let assign23580_e26224: f64 = (assign23580_e26222 - 230.25850929940458);
        let assign23580_e26227: f64 = (-var_fbbtbot);
        let assign23580_e26229: f64 = (assign23580_e26227 / var_fmaxr);
        let assign23580_e26231: f64 = (assign23580_e26229 - 230.25850929940458);
        let assign23580_e26233: f64 = (assign23580_e26231 * 0.3333333333333333);
        let assign23580_e26234: f64 = (1.0 + assign23580_e26233);
        let assign23580_e26235: f64 = (assign23580_e26224 * assign23580_e26234);
        let assign23580_e26236: f64 = (0.5 * assign23580_e26235);
        let assign23580_e26237: f64 = (1.0 + assign23580_e26236);
        let assign23580_e26238: f64 = (assign23580_e26216 * assign23580_e26237);
        let assign23580_e26239: f64 = (1.0 + assign23580_e26238);
        let assign23580_e26240: f64 = (1e100 * assign23580_e26239);
        (assign23580_e26240, (1e100 * (((-((assign23580_e26212 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign23580_e26237) + (assign23580_e26216 * (0.5 * (((-((assign23580_e26220 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign23580_e26234) + (assign23580_e26224 * ((-((assign23580_e26227 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign23580_e26212 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign23580_e26237) + (assign23580_e26216 * (0.5 * (((-((assign23580_e26220 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign23580_e26234) + (assign23580_e26224 * ((-((assign23580_e26227 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign23580_e26212 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign23580_e26237) + (assign23580_e26216 * (0.5 * (((-((assign23580_e26220 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign23580_e26234) + (assign23580_e26224 * ((-((assign23580_e26227 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign23580_e26212 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign23580_e26237) + (assign23580_e26216 * (0.5 * (((-((assign23580_e26220 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign23580_e26234) + (assign23580_e26224 * ((-((assign23580_e26227 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23580_e26242;
        var_tmp_dn5 = assign23580_e26242_d_n5;
        var_tmp_dn6 = assign23580_e26242_d_n6;
        var_tmp_dn7 = assign23580_e26242_d_n7;
        var_tmp_dn8 = assign23580_e26242_d_n8;

        let (assign23590_e26262, assign23590_e26262_d_n5, assign23590_e26262_d_n6, assign23590_e26262_d_n7, assign23590_e26262_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard431 == 0.0)) {
        let assign23590_e26255: f64 = (var_v4 * var_fmaxr);
        let assign23590_e26257: f64 = (assign23590_e26255 * var_fmaxr);
        let assign23590_e26259: f64 = (assign23590_e26257 * var_tmp);
        let assign23590_e26260: f64 = (p.p851 * assign23590_e26259);
        (assign23590_e26260, (p.p851 * (((((var_v4 * var_fmaxr_dn5) * var_fmaxr) + (assign23590_e26255 * var_fmaxr_dn5)) * var_tmp) + (assign23590_e26257 * var_tmp_dn5))), (p.p851 * (((((var_v4 * var_fmaxr_dn6) * var_fmaxr) + (assign23590_e26255 * var_fmaxr_dn6)) * var_tmp) + (assign23590_e26257 * var_tmp_dn6))), (p.p851 * (((((var_v4 * var_fmaxr_dn7) * var_fmaxr) + (assign23590_e26255 * var_fmaxr_dn7)) * var_tmp) + (assign23590_e26257 * var_tmp_dn7))), (p.p851 * (((((var_v4 * var_fmaxr_dn8) * var_fmaxr) + (assign23590_e26255 * var_fmaxr_dn8)) * var_tmp) + (assign23590_e26257 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign23590_e26262;
        var_ibbt_dn5 = assign23590_e26262_d_n5;
        var_ibbt_dn6 = assign23590_e26262_d_n6;
        var_ibbt_dn7 = assign23590_e26262_d_n7;
        var_ibbt_dn8 = assign23590_e26262_d_n8;

        let assign23600_e26265: f64 = if p.p860 > 1000.0 { 1.0 } else { 0.0 };
        var_guard435 = assign23600_e26265;

        let (assign23610_e26276, assign23610_e26276_d_n5, assign23610_e26276_d_n6, assign23610_e26276_d_n7, assign23610_e26276_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard435 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign23610_e26276;
        var_fbreakdown_dn5 = assign23610_e26276_d_n5;
        var_fbreakdown_dn6 = assign23610_e26276_d_n6;
        var_fbreakdown_dn7 = assign23610_e26276_d_n7;
        var_fbreakdown_dn8 = assign23610_e26276_d_n8;

        let assign23620_e26279: f64 = (-var_alphaav);
        let assign23620_e26281: f64 = (assign23620_e26279 * p.p860);
        let assign23620_e26282: f64 = if var_vav > assign23620_e26281 { 1.0 } else { 0.0 };
        var_guard436 = assign23620_e26282;

        let assign23630_e26285: f64 = if p.p863 == 4.0 { 1.0 } else { 0.0 };
        var_guard437 = assign23630_e26285;

        let (assign23640_e26315, assign23640_e26315_d_n5, assign23640_e26315_d_n6, assign23640_e26315_d_n7, assign23640_e26315_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard435 == 0.0)) && (var_guard436 != 0.0)) && (var_guard437 != 0.0)) {
        let assign23640_e26301: f64 = (var_vav * var_vbrinvbot);
        let assign23640_e26304: f64 = (var_vav * var_vbrinvbot);
        let assign23640_e26305: f64 = (assign23640_e26301 * assign23640_e26304);
        let assign23640_e26308: f64 = (var_vav * var_vbrinvbot);
        let assign23640_e26309: f64 = (assign23640_e26305 * assign23640_e26308);
        let assign23640_e26312: f64 = (var_vav * var_vbrinvbot);
        let assign23640_e26313: f64 = (assign23640_e26309 * assign23640_e26312);
        (assign23640_e26313, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23640_e26315;
        var_tmp_dn5 = assign23640_e26315_d_n5;
        var_tmp_dn6 = assign23640_e26315_d_n6;
        var_tmp_dn7 = assign23640_e26315_d_n7;
        var_tmp_dn8 = assign23640_e26315_d_n8;

        let (assign23650_e26337, assign23650_e26337_d_n5, assign23650_e26337_d_n6, assign23650_e26337_d_n7, assign23650_e26337_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard435 == 0.0)) && (var_guard436 != 0.0)) && (var_guard437 == 0.0)) {
        let assign23650_e26332: f64 = (var_vav * var_vbrinvbot);
        let assign23650_e26333: f64 = (assign23650_e26332).abs();
        let assign23650_e26335: f64 = (assign23650_e26333).powf(p.p863);
        (assign23650_e26335, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23650_e26337;
        var_tmp_dn5 = assign23650_e26337_d_n5;
        var_tmp_dn6 = assign23650_e26337_d_n6;
        var_tmp_dn7 = assign23650_e26337_d_n7;
        var_tmp_dn8 = assign23650_e26337_d_n8;

        let (assign23660_e26355, assign23660_e26355_d_n5, assign23660_e26355_d_n6, assign23660_e26355_d_n7, assign23660_e26355_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard435 == 0.0)) && (var_guard436 != 0.0)) {
        let assign23660_e26352: f64 = (1.0 - var_tmp);
        let assign23660_e26353: f64 = (1.0 / assign23660_e26352);
        (assign23660_e26353, (-((-var_tmp_dn5) / (assign23660_e26352 * assign23660_e26352))), (-((-var_tmp_dn6) / (assign23660_e26352 * assign23660_e26352))), (-((-var_tmp_dn7) / (assign23660_e26352 * assign23660_e26352))), (-((-var_tmp_dn8) / (assign23660_e26352 * assign23660_e26352))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign23660_e26355;
        var_fbreakdown_dn5 = assign23660_e26355_d_n5;
        var_fbreakdown_dn6 = assign23660_e26355_d_n6;
        var_fbreakdown_dn7 = assign23660_e26355_d_n7;
        var_fbreakdown_dn8 = assign23660_e26355_d_n8;

        let (assign23670_e26378, assign23670_e26378_d_n5, assign23670_e26378_d_n6, assign23670_e26378_d_n7, assign23670_e26378_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) && (var_guard435 == 0.0)) && (var_guard436 == 0.0)) {
        let assign23670_e26372: f64 = (var_alphaav * p.p860);
        let assign23670_e26373: f64 = (var_vav + assign23670_e26372);
        let assign23670_e26375: f64 = (assign23670_e26373 * var_slopebot);
        let assign23670_e26376: f64 = (var_fstopbot + assign23670_e26375);
        (assign23670_e26376, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign23670_e26378;
        var_fbreakdown_dn5 = assign23670_e26378_d_n5;
        var_fbreakdown_dn6 = assign23670_e26378_d_n6;
        var_fbreakdown_dn7 = assign23670_e26378_d_n7;
        var_fbreakdown_dn8 = assign23670_e26378_d_n8;

        let (assign23680_e26397, assign23680_e26397_d_n5, assign23680_e26397_d_n6, assign23680_e26397_d_n7, assign23680_e26397_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard421 == 0.0)) {
        let assign23680_e26388: f64 = (var_id__blk219 + var_isrh);
        let assign23680_e26390: f64 = (assign23680_e26388 + var_itat);
        let assign23680_e26392: f64 = (assign23680_e26390 + var_ibbt);
        let assign23680_e26393: f64 = (p.p29 * assign23680_e26392);
        let assign23680_e26395: f64 = (assign23680_e26393 * var_fbreakdown);
        (assign23680_e26395, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign23680_e26393 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign23680_e26393 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign23680_e26393 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign23680_e26393 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign23680_e26397;
        var_ijunbot_dn5 = assign23680_e26397_d_n5;
        var_ijunbot_dn6 = assign23680_e26397_d_n6;
        var_ijunbot_dn7 = assign23680_e26397_d_n7;
        var_ijunbot_dn8 = assign23680_e26397_d_n8;

        let assign23690_e26400: f64 = if var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard438 = assign23690_e26400;

        let (assign23700_e26408, assign23700_e26408_d_n5, assign23700_e26408_d_n6, assign23700_e26408_d_n7, assign23700_e26408_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign23700_e26408;
        var_ijunsti_dn5 = assign23700_e26408_d_n5;
        var_ijunsti_dn6 = assign23700_e26408_d_n6;
        var_ijunsti_dn7 = assign23700_e26408_d_n7;
        var_ijunsti_dn8 = assign23700_e26408_d_n8;

        let (assign23710_e26419,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) {
        let assign23710_e26417: f64 = (var_idsatsti * var_idmult);
        (assign23710_e26417,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign23710_e26419;

        let assign23720_e26426: f64 = if ((p.p841 == 0.0) && (p.p846 == 0.0)) { 1.0 } else { 0.0 };
        var_guard439 = assign23720_e26426;

        let (assign23730_e26437, assign23730_e26437_d_n5, assign23730_e26437_d_n6, assign23730_e26437_d_n7, assign23730_e26437_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign23730_e26437;
        var_isrh_dn5 = assign23730_e26437_d_n5;
        var_isrh_dn6 = assign23730_e26437_d_n6;
        var_isrh_dn7 = assign23730_e26437_d_n7;
        var_isrh_dn8 = assign23730_e26437_d_n8;

        let (assign23740_e26451,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard439 == 0.0)) {
        let assign23740_e26449: f64 = (var_vbisti - var_vjsrh);
        (assign23740_e26449,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign23740_e26451;

        let (assign23750_e26470,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard439 == 0.0)) {
        let assign23750_e26465: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign23750_e26466: f64 = (1.0 - assign23750_e26465);
        let assign23750_e26467: f64 = (assign23750_e26466).sqrt();
        let assign23750_e26468: f64 = (1.0 - assign23750_e26467);
        (assign23750_e26468,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign23750_e26470;

        let assign23760_e26473: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        var_guard440 = assign23760_e26473;

        let (assign23770_e26487,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard439 == 0.0)) && (var_guard440 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign23770_e26487;

        let (assign23780_e26519,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard439 == 0.0)) && (var_guard440 == 0.0)) {
        let assign23780_e26502: f64 = (var_wsrhstep * var_wsrhstep);
        let assign23780_e26504: f64 = (var_wsrhstep).ln();
        let assign23780_e26505: f64 = (assign23780_e26502 * assign23780_e26504);
        let assign23780_e26508: f64 = (1.0 - var_wsrhstep);
        let assign23780_e26509: f64 = (assign23780_e26505 / assign23780_e26508);
        let assign23780_e26511: f64 = (assign23780_e26509 + var_wsrhstep);
        let assign23780_e26515: f64 = (2.0 * p.p832);
        let assign23780_e26516: f64 = (1.0 - assign23780_e26515);
        let assign23780_e26517: f64 = (assign23780_e26511 * assign23780_e26516);
        (assign23780_e26517,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign23780_e26519;

        let (assign23790_e26533,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard439 == 0.0)) {
        let assign23790_e26531: f64 = (var_wsrhstep + var_dwsrh);
        (assign23790_e26531,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign23790_e26533;

        let assign23800_e26536: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        var_guard441 = assign23800_e26536;

        let (assign23810_e26553, assign23810_e26553_d_n5, assign23810_e26553_d_n6, assign23810_e26553_d_n7, assign23810_e26553_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard439 == 0.0)) && (var_guard441 != 0.0)) {
        let assign23810_e26550: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign23810_e26551: f64 = (assign23810_e26550).sqrt();
        (assign23810_e26551, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23810_e26553;
        var_tmp_dn5 = assign23810_e26553_d_n5;
        var_tmp_dn6 = assign23810_e26553_d_n6;
        var_tmp_dn7 = assign23810_e26553_d_n7;
        var_tmp_dn8 = assign23810_e26553_d_n8;

        let (assign23820_e26572, assign23820_e26572_d_n5, assign23820_e26572_d_n6, assign23820_e26572_d_n7, assign23820_e26572_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard439 == 0.0)) && (var_guard441 == 0.0)) {
        let assign23820_e26568: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign23820_e26570: f64 = (assign23820_e26568).powf(p.p832);
        (assign23820_e26570, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign23820_e26572;
        var_tmp_dn5 = assign23820_e26572_d_n5;
        var_tmp_dn6 = assign23820_e26572_d_n6;
        var_tmp_dn7 = assign23820_e26572_d_n7;
        var_tmp_dn8 = assign23820_e26572_d_n8;

        let (assign23830_e26586, assign23830_e26586_d_n5, assign23830_e26586_d_n6, assign23830_e26586_d_n7, assign23830_e26586_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard439 == 0.0)) {
        let assign23830_e26584: f64 = (var_wdepnulrsti * var_tmp);
        (assign23830_e26584, (var_wdepnulrsti * var_tmp_dn5), (var_wdepnulrsti * var_tmp_dn6), (var_wdepnulrsti * var_tmp_dn7), (var_wdepnulrsti * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign23830_e26586;
        var_wdep_dn5 = assign23830_e26586_d_n5;
        var_wdep_dn6 = assign23830_e26586_d_n6;
        var_wdep_dn7 = assign23830_e26586_d_n7;
        var_wdep_dn8 = assign23830_e26586_d_n8;

        let (assign23840_e26604, assign23840_e26604_d_n5, assign23840_e26604_d_n6, assign23840_e26604_d_n7, assign23840_e26604_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard439 == 0.0)) {
        let assign23840_e26599: f64 = (var_zinv - 1.0);
        let assign23840_e26601: f64 = (assign23840_e26599 * var_wdep);
        let assign23840_e26602: f64 = (var_ftdsti * assign23840_e26601);
        (assign23840_e26602, (var_ftdsti * (assign23840_e26599 * var_wdep_dn5)), (var_ftdsti * (assign23840_e26599 * var_wdep_dn6)), (var_ftdsti * (assign23840_e26599 * var_wdep_dn7)), (var_ftdsti * (assign23840_e26599 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign23840_e26604;
        var_asrh_dn5 = assign23840_e26604_d_n5;
        var_asrh_dn6 = assign23840_e26604_d_n6;
        var_asrh_dn7 = assign23840_e26604_d_n7;
        var_asrh_dn8 = assign23840_e26604_d_n8;

        let (assign23850_e26620, assign23850_e26620_d_n5, assign23850_e26620_d_n6, assign23850_e26620_d_n7, assign23850_e26620_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard439 == 0.0)) {
        let assign23850_e26617: f64 = (var_asrh * var_wsrh);
        let assign23850_e26618: f64 = (p.p841 * assign23850_e26617);
        (assign23850_e26618, (p.p841 * (var_asrh_dn5 * var_wsrh)), (p.p841 * (var_asrh_dn6 * var_wsrh)), (p.p841 * (var_asrh_dn7 * var_wsrh)), (p.p841 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign23850_e26620;
        var_isrh_dn5 = assign23850_e26620_d_n5;
        var_isrh_dn6 = assign23850_e26620_d_n6;
        var_isrh_dn7 = assign23850_e26620_d_n7;
        var_isrh_dn8 = assign23850_e26620_d_n8;

        let assign23860_e26623: f64 = if p.p846 == 0.0 { 1.0 } else { 0.0 };
        var_guard442 = assign23860_e26623;

        let (assign23870_e26634, assign23870_e26634_d_n5, assign23870_e26634_d_n6, assign23870_e26634_d_n7, assign23870_e26634_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign23870_e26634;
        var_itat_dn5 = assign23870_e26634_d_n5;
        var_itat_dn6 = assign23870_e26634_d_n6;
        var_itat_dn7 = assign23870_e26634_d_n7;
        var_itat_dn8 = assign23870_e26634_d_n8;

        let (assign23880_e26652, assign23880_e26652_d_n5, assign23880_e26652_d_n6, assign23880_e26652_d_n7, assign23880_e26652_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) {
        let assign23880_e26647: f64 = (var_wdep * var_one_minus_psti);
        let assign23880_e26649: f64 = (assign23880_e26647 / var_vbi_minus_vjsrh);
        let assign23880_e26650: f64 = (var_btatpartsti * assign23880_e26649);
        (assign23880_e26650, (var_btatpartsti * ((var_wdep_dn5 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn6 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn7 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn8 * var_one_minus_psti) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign23880_e26652;
        var_btat_dn5 = assign23880_e26652_d_n5;
        var_btat_dn6 = assign23880_e26652_d_n6;
        var_btat_dn7 = assign23880_e26652_d_n7;
        var_btat_dn8 = assign23880_e26652_d_n8;

        let (assign23890_e26668, assign23890_e26668_d_n5, assign23890_e26668_d_n6, assign23890_e26668_d_n7, assign23890_e26668_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) {
        let assign23890_e26664: f64 = (0.666666666666667 * var_atatsti);
        let assign23890_e26666: f64 = (assign23890_e26664 / var_btat);
        (assign23890_e26666, (-((assign23890_e26664 * var_btat_dn5) / (var_btat * var_btat))), (-((assign23890_e26664 * var_btat_dn6) / (var_btat * var_btat))), (-((assign23890_e26664 * var_btat_dn7) / (var_btat * var_btat))), (-((assign23890_e26664 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign23890_e26668;
        var_twoatatoverthreebtat_dn5 = assign23890_e26668_d_n5;
        var_twoatatoverthreebtat_dn6 = assign23890_e26668_d_n6;
        var_twoatatoverthreebtat_dn7 = assign23890_e26668_d_n7;
        var_twoatatoverthreebtat_dn8 = assign23890_e26668_d_n8;

        let (assign23900_e26682, assign23900_e26682_d_n5, assign23900_e26682_d_n6, assign23900_e26682_d_n7, assign23900_e26682_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) {
        let assign23900_e26680: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign23900_e26680, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign23900_e26682;
        var_umaxbeforelimiting_dn5 = assign23900_e26682_d_n5;
        var_umaxbeforelimiting_dn6 = assign23900_e26682_d_n6;
        var_umaxbeforelimiting_dn7 = assign23900_e26682_d_n7;
        var_umaxbeforelimiting_dn8 = assign23900_e26682_d_n8;

        let (assign23910_e26703, assign23910_e26703_d_n5, assign23910_e26703_d_n6, assign23910_e26703_d_n7, assign23910_e26703_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) {
        let assign23910_e26694: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign23910_e26697: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign23910_e26699: f64 = (assign23910_e26697 + 1.0);
        let assign23910_e26700: f64 = (assign23910_e26694 / assign23910_e26699);
        let assign23910_e26701: f64 = (assign23910_e26700).sqrt();
        (assign23910_e26701, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign23910_e26699) - (assign23910_e26694 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign23910_e26699 * assign23910_e26699)) / (2.0 * assign23910_e26701)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign23910_e26699) - (assign23910_e26694 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign23910_e26699 * assign23910_e26699)) / (2.0 * assign23910_e26701)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign23910_e26699) - (assign23910_e26694 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign23910_e26699 * assign23910_e26699)) / (2.0 * assign23910_e26701)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign23910_e26699) - (assign23910_e26694 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign23910_e26699 * assign23910_e26699)) / (2.0 * assign23910_e26701)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign23910_e26703;
        var_umax_dn5 = assign23910_e26703_d_n5;
        var_umax_dn6 = assign23910_e26703_d_n6;
        var_umax_dn7 = assign23910_e26703_d_n7;
        var_umax_dn8 = assign23910_e26703_d_n8;

        let (assign23920_e26716, assign23920_e26716_d_n5, assign23920_e26716_d_n6, assign23920_e26716_d_n7, assign23920_e26716_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) {
        let assign23920_e26714: f64 = (var_umax).sqrt();
        (assign23920_e26714, (var_umax_dn5 / (2.0 * assign23920_e26714)), (var_umax_dn6 / (2.0 * assign23920_e26714)), (var_umax_dn7 / (2.0 * assign23920_e26714)), (var_umax_dn8 / (2.0 * assign23920_e26714)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign23920_e26716;
        var_sqrtumax_dn5 = assign23920_e26716_d_n5;
        var_sqrtumax_dn6 = assign23920_e26716_d_n6;
        var_sqrtumax_dn7 = assign23920_e26716_d_n7;
        var_sqrtumax_dn8 = assign23920_e26716_d_n8;

        let (assign23930_e26730, assign23930_e26730_d_n5, assign23930_e26730_d_n6, assign23930_e26730_d_n7, assign23930_e26730_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) {
        let assign23930_e26728: f64 = (var_umax * var_sqrtumax);
        (assign23930_e26728, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign23930_e26730;
        var_umaxpoweronepointfive_dn5 = assign23930_e26730_d_n5;
        var_umaxpoweronepointfive_dn6 = assign23930_e26730_d_n6;
        var_umaxpoweronepointfive_dn7 = assign23930_e26730_d_n7;
        var_umaxpoweronepointfive_dn8 = assign23930_e26730_d_n8;

        let assign23940_e26732: f64 = (-p.p832);
        let assign23940_e26734: f64 = (assign23940_e26732 * var_one_over_one_minus_psti);
        let assign23940_e26736: f64 = (-1.0);
        let assign23940_e26737: f64 = if assign23940_e26734 == assign23940_e26736 { 1.0 } else { 0.0 };
        var_guard443 = assign23940_e26737;

        let (assign23950_e26757, assign23950_e26757_d_n5, assign23950_e26757_d_n6, assign23950_e26757_d_n7, assign23950_e26757_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) && (var_guard443 != 0.0)) {
        let assign23950_e26753: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign23950_e26754: f64 = (1.0 + assign23950_e26753);
        let assign23950_e26755: f64 = (1.0 / assign23950_e26754);
        (assign23950_e26755, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign23950_e26754 * assign23950_e26754))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign23950_e26754 * assign23950_e26754))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign23950_e26754 * assign23950_e26754))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign23950_e26754 * assign23950_e26754))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign23950_e26757;
        var_wgamma_dn5 = assign23950_e26757_d_n5;
        var_wgamma_dn6 = assign23950_e26757_d_n6;
        var_wgamma_dn7 = assign23950_e26757_d_n7;
        var_wgamma_dn8 = assign23950_e26757_d_n8;

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
        *var_guard435_slot = var_guard435;
        *var_guard436_slot = var_guard436;
        *var_guard437_slot = var_guard437;
        *var_guard438_slot = var_guard438;
        *var_guard439_slot = var_guard439;
        *var_guard440_slot = var_guard440;
        *var_guard441_slot = var_guard441;
        *var_guard442_slot = var_guard442;
        *var_guard443_slot = var_guard443;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn5_slot = var_ibbt_dn5;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_id__blk219_slot = var_id__blk219;
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
    }

    pub(super) fn stamp_transient_block_44(
        p: &Parameters,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatsti: f64,
        var_berfc: f64,
        var_btat: f64,
        var_btat_dn5: f64,
        var_btat_dn6: f64,
        var_btat_dn7: f64,
        var_btat_dn8: f64,
        var_cerfc: f64,
        var_fbbtsti: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard438: f64,
        var_guard442: f64,
        var_guard443: f64,
        var_one_over_one_minus_psti: f64,
        var_perfc: f64,
        var_sqrtumax: f64,
        var_sqrtumax_dn5: f64,
        var_sqrtumax_dn6: f64,
        var_sqrtumax_dn7: f64,
        var_sqrtumax_dn8: f64,
        var_twoatatoverthreebtat: f64,
        var_twoatatoverthreebtat_dn5: f64,
        var_twoatatoverthreebtat_dn6: f64,
        var_twoatatoverthreebtat_dn7: f64,
        var_twoatatoverthreebtat_dn8: f64,
        var_umax: f64,
        var_umax_dn5: f64,
        var_umax_dn6: f64,
        var_umax_dn7: f64,
        var_umax_dn8: f64,
        var_umaxpoweronepointfive: f64,
        var_umaxpoweronepointfive_dn5: f64,
        var_umaxpoweronepointfive_dn6: f64,
        var_umaxpoweronepointfive_dn7: f64,
        var_umaxpoweronepointfive_dn8: f64,
        var_v4: f64,
        var_vbbt: f64,
        var_vbirstiinv: f64,
        var_wdepnulrinvsti: f64,
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
        var_guard444_slot: &mut f64,
        var_guard445_slot: &mut f64,
        var_guard446_slot: &mut f64,
        var_guard447_slot: &mut f64,
        var_guard448_slot: &mut f64,
        var_guard449_slot: &mut f64,
        var_guard450_slot: &mut f64,
        var_guard451_slot: &mut f64,
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
        let mut var_guard444: f64 = *var_guard444_slot;
        let mut var_guard445: f64 = *var_guard445_slot;
        let mut var_guard446: f64 = *var_guard446_slot;
        let mut var_guard447: f64 = *var_guard447_slot;
        let mut var_guard448: f64 = *var_guard448_slot;
        let mut var_guard449: f64 = *var_guard449_slot;
        let mut var_guard450: f64 = *var_guard450_slot;
        let mut var_guard451: f64 = *var_guard451_slot;
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

        let (assign23960_e26781, assign23960_e26781_d_n5, assign23960_e26781_d_n6, assign23960_e26781_d_n7, assign23960_e26781_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) && (var_guard443 == 0.0)) {
        let assign23960_e26773: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign23960_e26774: f64 = (1.0 + assign23960_e26773);
        let assign23960_e26776: f64 = (-p.p832);
        let assign23960_e26778: f64 = (assign23960_e26776 * var_one_over_one_minus_psti);
        let assign23960_e26779: f64 = (assign23960_e26774).powf(assign23960_e26778);
        (assign23960_e26779, if 0.0 == 0.0 && ((assign23960_e26778) as f64).is_finite() && ((assign23960_e26778) as f64).fract() == 0.0 { if assign23960_e26778 == 0.0 { 0.0 } else { (assign23960_e26778 * ((assign23960_e26774).powf(assign23960_e26778 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign23960_e26779 * (assign23960_e26778 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign23960_e26774))) }, if 0.0 == 0.0 && ((assign23960_e26778) as f64).is_finite() && ((assign23960_e26778) as f64).fract() == 0.0 { if assign23960_e26778 == 0.0 { 0.0 } else { (assign23960_e26778 * ((assign23960_e26774).powf(assign23960_e26778 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign23960_e26779 * (assign23960_e26778 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign23960_e26774))) }, if 0.0 == 0.0 && ((assign23960_e26778) as f64).is_finite() && ((assign23960_e26778) as f64).fract() == 0.0 { if assign23960_e26778 == 0.0 { 0.0 } else { (assign23960_e26778 * ((assign23960_e26774).powf(assign23960_e26778 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign23960_e26779 * (assign23960_e26778 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign23960_e26774))) }, if 0.0 == 0.0 && ((assign23960_e26778) as f64).is_finite() && ((assign23960_e26778) as f64).fract() == 0.0 { if assign23960_e26778 == 0.0 { 0.0 } else { (assign23960_e26778 * ((assign23960_e26774).powf(assign23960_e26778 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign23960_e26779 * (assign23960_e26778 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign23960_e26774))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign23960_e26781;
        var_wgamma_dn5 = assign23960_e26781_d_n5;
        var_wgamma_dn6 = assign23960_e26781_d_n6;
        var_wgamma_dn7 = assign23960_e26781_d_n7;
        var_wgamma_dn8 = assign23960_e26781_d_n8;

        let (assign23970_e26799, assign23970_e26799_d_n5, assign23970_e26799_d_n6, assign23970_e26799_d_n7, assign23970_e26799_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) {
        let assign23970_e26793: f64 = (var_wsrh * var_wgamma);
        let assign23970_e26796: f64 = (var_wsrh + var_wgamma);
        let assign23970_e26797: f64 = (assign23970_e26793 / assign23970_e26796);
        (assign23970_e26797, ((((var_wsrh * var_wgamma_dn5) * assign23970_e26796) - (assign23970_e26793 * var_wgamma_dn5)) / (assign23970_e26796 * assign23970_e26796)), ((((var_wsrh * var_wgamma_dn6) * assign23970_e26796) - (assign23970_e26793 * var_wgamma_dn6)) / (assign23970_e26796 * assign23970_e26796)), ((((var_wsrh * var_wgamma_dn7) * assign23970_e26796) - (assign23970_e26793 * var_wgamma_dn7)) / (assign23970_e26796 * assign23970_e26796)), ((((var_wsrh * var_wgamma_dn8) * assign23970_e26796) - (assign23970_e26793 * var_wgamma_dn8)) / (assign23970_e26796 * assign23970_e26796)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign23970_e26799;
        var_wtat_dn5 = assign23970_e26799_d_n5;
        var_wtat_dn6 = assign23970_e26799_d_n6;
        var_wtat_dn7 = assign23970_e26799_d_n7;
        var_wtat_dn8 = assign23970_e26799_d_n8;

        let (assign23980_e26816, assign23980_e26816_d_n5, assign23980_e26816_d_n6, assign23980_e26816_d_n7, assign23980_e26816_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) {
        let assign23980_e26812: f64 = (var_btat / var_sqrtumax);
        let assign23980_e26813: f64 = (0.375 * assign23980_e26812);
        let assign23980_e26814: f64 = (assign23980_e26813).sqrt();
        (assign23980_e26814, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign23980_e26814)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign23980_e26814)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign23980_e26814)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign23980_e26814)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign23980_e26816;
        var_ktat_dn5 = assign23980_e26816_d_n5;
        var_ktat_dn6 = assign23980_e26816_d_n6;
        var_ktat_dn7 = assign23980_e26816_d_n7;
        var_ktat_dn8 = assign23980_e26816_d_n8;

        let (assign23990_e26834, assign23990_e26834_d_n5, assign23990_e26834_d_n6, assign23990_e26834_d_n7, assign23990_e26834_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) {
        let assign23990_e26829: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign23990_e26830: f64 = (2.0 * assign23990_e26829);
        let assign23990_e26832: f64 = (assign23990_e26830 - var_umax);
        (assign23990_e26832, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign23990_e26834;
        var_ltat_dn5 = assign23990_e26834_d_n5;
        var_ltat_dn6 = assign23990_e26834_d_n6;
        var_ltat_dn7 = assign23990_e26834_d_n7;
        var_ltat_dn8 = assign23990_e26834_d_n8;

        let (assign24000_e26860, assign24000_e26860_d_n5, assign24000_e26860_d_n6, assign24000_e26860_d_n7, assign24000_e26860_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) {
        let assign24000_e26846: f64 = (var_atatsti * var_twoatatoverthreebtat);
        let assign24000_e26848: f64 = (assign24000_e26846 * var_sqrtumax);
        let assign24000_e26851: f64 = (var_atatsti * var_umax);
        let assign24000_e26852: f64 = (assign24000_e26848 - assign24000_e26851);
        let assign24000_e26856: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign24000_e26857: f64 = (0.5 * assign24000_e26856);
        let assign24000_e26858: f64 = (assign24000_e26852 + assign24000_e26857);
        (assign24000_e26858, (((((var_atatsti * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign24000_e26846 * var_sqrtumax_dn5)) - (var_atatsti * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatsti * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign24000_e26846 * var_sqrtumax_dn6)) - (var_atatsti * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign24000_e26846 * var_sqrtumax_dn7)) - (var_atatsti * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign24000_e26846 * var_sqrtumax_dn8)) - (var_atatsti * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign24000_e26860;
        var_mtat_dn5 = assign24000_e26860_d_n5;
        var_mtat_dn6 = assign24000_e26860_d_n6;
        var_mtat_dn7 = assign24000_e26860_d_n7;
        var_mtat_dn8 = assign24000_e26860_d_n8;

        let (assign24010_e26876, assign24010_e26876_d_n5, assign24010_e26876_d_n6, assign24010_e26876_d_n7, assign24010_e26876_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) {
        let assign24010_e26872: f64 = (var_ltat - 1.0);
        let assign24010_e26874: f64 = (assign24010_e26872 * var_ktat);
        (assign24010_e26874, ((var_ltat_dn5 * var_ktat) + (assign24010_e26872 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign24010_e26872 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign24010_e26872 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign24010_e26872 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign24010_e26876;
        var_xerfc_dn5 = assign24010_e26876_d_n5;
        var_xerfc_dn6 = assign24010_e26876_d_n6;
        var_xerfc_dn7 = assign24010_e26876_d_n7;
        var_xerfc_dn8 = assign24010_e26876_d_n8;

        let (assign24020_e26890, assign24020_e26890_d_n5, assign24020_e26890_d_n6, assign24020_e26890_d_n7, assign24020_e26890_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) {
        let assign24020_e26888: f64 = (var_xerfc * var_xerfc);
        (assign24020_e26888, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign24020_e26890;
        var_ysq_dn5 = assign24020_e26890_d_n5;
        var_ysq_dn6 = assign24020_e26890_d_n6;
        var_ysq_dn7 = assign24020_e26890_d_n7;
        var_ysq_dn8 = assign24020_e26890_d_n8;

        let assign24030_e26893: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard444 = assign24030_e26893;

        let (assign24040_e26913, assign24040_e26913_d_n5, assign24040_e26913_d_n6, assign24040_e26913_d_n7, assign24040_e26913_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) && (var_guard444 != 0.0)) {
        let assign24040_e26909: f64 = (var_perfc * var_xerfc);
        let assign24040_e26910: f64 = (1.0 + assign24040_e26909);
        let assign24040_e26911: f64 = (1.0 / assign24040_e26910);
        (assign24040_e26911, (-((var_perfc * var_xerfc_dn5) / (assign24040_e26910 * assign24040_e26910))), (-((var_perfc * var_xerfc_dn6) / (assign24040_e26910 * assign24040_e26910))), (-((var_perfc * var_xerfc_dn7) / (assign24040_e26910 * assign24040_e26910))), (-((var_perfc * var_xerfc_dn8) / (assign24040_e26910 * assign24040_e26910))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign24040_e26913;
        var_terfc_dn5 = assign24040_e26913_d_n5;
        var_terfc_dn6 = assign24040_e26913_d_n6;
        var_terfc_dn7 = assign24040_e26913_d_n7;
        var_terfc_dn8 = assign24040_e26913_d_n8;

        let (assign24050_e26934, assign24050_e26934_d_n5, assign24050_e26934_d_n6, assign24050_e26934_d_n7, assign24050_e26934_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) && (var_guard444 == 0.0)) {
        let assign24050_e26930: f64 = (var_perfc * var_xerfc);
        let assign24050_e26931: f64 = (1.0 - assign24050_e26930);
        let assign24050_e26932: f64 = (1.0 / assign24050_e26931);
        (assign24050_e26932, (-((-(var_perfc * var_xerfc_dn5)) / (assign24050_e26931 * assign24050_e26931))), (-((-(var_perfc * var_xerfc_dn6)) / (assign24050_e26931 * assign24050_e26931))), (-((-(var_perfc * var_xerfc_dn7)) / (assign24050_e26931 * assign24050_e26931))), (-((-(var_perfc * var_xerfc_dn8)) / (assign24050_e26931 * assign24050_e26931))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign24050_e26934;
        var_terfc_dn5 = assign24050_e26934_d_n5;
        var_terfc_dn6 = assign24050_e26934_d_n6;
        var_terfc_dn7 = assign24050_e26934_d_n7;
        var_terfc_dn8 = assign24050_e26934_d_n8;

        let assign24060_e26936: f64 = (-var_ysq);
        let assign24060_e26938: f64 = (assign24060_e26936 + var_mtat);
        let assign24060_e26940: f64 = (-230.25850929940458);
        let assign24060_e26941: f64 = if assign24060_e26938 > assign24060_e26940 { 1.0 } else { 0.0 };
        var_guard445 = assign24060_e26941;

        let (assign24070_e26959, assign24070_e26959_d_n5, assign24070_e26959_d_n6, assign24070_e26959_d_n7, assign24070_e26959_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) && (var_guard445 != 0.0)) {
        let assign24070_e26954: f64 = (-var_ysq);
        let assign24070_e26956: f64 = (assign24070_e26954 + var_mtat);
        let assign24070_e26957: f64 = (assign24070_e26956).exp();
        (assign24070_e26957, (assign24070_e26957 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign24070_e26957 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign24070_e26957 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign24070_e26957 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24070_e26959;
        var_tmp_dn5 = assign24070_e26959_d_n5;
        var_tmp_dn6 = assign24070_e26959_d_n6;
        var_tmp_dn7 = assign24070_e26959_d_n7;
        var_tmp_dn8 = assign24070_e26959_d_n8;

        let (assign24080_e27008, assign24080_e27008_d_n5, assign24080_e27008_d_n6, assign24080_e27008_d_n7, assign24080_e27008_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) && (var_guard445 == 0.0)) {
        let assign24080_e26975: f64 = (-230.25850929940458);
        let assign24080_e26977: f64 = (-var_ysq);
        let assign24080_e26979: f64 = (assign24080_e26977 + var_mtat);
        let assign24080_e26980: f64 = (assign24080_e26975 - assign24080_e26979);
        let assign24080_e26984: f64 = (-230.25850929940458);
        let assign24080_e26986: f64 = (-var_ysq);
        let assign24080_e26988: f64 = (assign24080_e26986 + var_mtat);
        let assign24080_e26989: f64 = (assign24080_e26984 - assign24080_e26988);
        let assign24080_e26992: f64 = (-230.25850929940458);
        let assign24080_e26994: f64 = (-var_ysq);
        let assign24080_e26996: f64 = (assign24080_e26994 + var_mtat);
        let assign24080_e26997: f64 = (assign24080_e26992 - assign24080_e26996);
        let assign24080_e26999: f64 = (assign24080_e26997 * 0.3333333333333333);
        let assign24080_e27000: f64 = (1.0 + assign24080_e26999);
        let assign24080_e27001: f64 = (assign24080_e26989 * assign24080_e27000);
        let assign24080_e27002: f64 = (0.5 * assign24080_e27001);
        let assign24080_e27003: f64 = (1.0 + assign24080_e27002);
        let assign24080_e27004: f64 = (assign24080_e26980 * assign24080_e27003);
        let assign24080_e27005: f64 = (1.0 + assign24080_e27004);
        let assign24080_e27006: f64 = (1e-100 / assign24080_e27005);
        (assign24080_e27006, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign24080_e27003) + (assign24080_e26980 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign24080_e27000) + (assign24080_e26989 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign24080_e27005 * assign24080_e27005))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign24080_e27003) + (assign24080_e26980 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign24080_e27000) + (assign24080_e26989 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign24080_e27005 * assign24080_e27005))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign24080_e27003) + (assign24080_e26980 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign24080_e27000) + (assign24080_e26989 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign24080_e27005 * assign24080_e27005))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign24080_e27003) + (assign24080_e26980 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign24080_e27000) + (assign24080_e26989 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign24080_e27005 * assign24080_e27005))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24080_e27008;
        var_tmp_dn5 = assign24080_e27008_d_n5;
        var_tmp_dn6 = assign24080_e27008_d_n6;
        var_tmp_dn7 = assign24080_e27008_d_n7;
        var_tmp_dn8 = assign24080_e27008_d_n8;

        let (assign24090_e27038, assign24090_e27038_d_n5, assign24090_e27038_d_n6, assign24090_e27038_d_n7, assign24090_e27038_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) {
        let assign24090_e27020: f64 = (0.29214664 * var_terfc);
        let assign24090_e27024: f64 = (var_terfc * var_terfc);
        let assign24090_e27025: f64 = (var_berfc * assign24090_e27024);
        let assign24090_e27026: f64 = (assign24090_e27020 + assign24090_e27025);
        let assign24090_e27030: f64 = (var_terfc * var_terfc);
        let assign24090_e27032: f64 = (assign24090_e27030 * var_terfc);
        let assign24090_e27033: f64 = (var_cerfc * assign24090_e27032);
        let assign24090_e27034: f64 = (assign24090_e27026 + assign24090_e27033);
        let assign24090_e27036: f64 = (assign24090_e27034 * var_tmp);
        (assign24090_e27036, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign24090_e27030 * var_terfc_dn5)))) * var_tmp) + (assign24090_e27034 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign24090_e27030 * var_terfc_dn6)))) * var_tmp) + (assign24090_e27034 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign24090_e27030 * var_terfc_dn7)))) * var_tmp) + (assign24090_e27034 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign24090_e27030 * var_terfc_dn8)))) * var_tmp) + (assign24090_e27034 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign24090_e27038;
        var_erfcpos_dn5 = assign24090_e27038_d_n5;
        var_erfcpos_dn6 = assign24090_e27038_d_n6;
        var_erfcpos_dn7 = assign24090_e27038_d_n7;
        var_erfcpos_dn8 = assign24090_e27038_d_n8;

        let assign24100_e27041: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard446 = assign24100_e27041;

        let (assign24110_e27055, assign24110_e27055_d_n5, assign24110_e27055_d_n6, assign24110_e27055_d_n7, assign24110_e27055_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) && (var_guard446 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign24110_e27055;
        var_erfctimesexpmtat_dn5 = assign24110_e27055_d_n5;
        var_erfctimesexpmtat_dn6 = assign24110_e27055_d_n6;
        var_erfctimesexpmtat_dn7 = assign24110_e27055_d_n7;
        var_erfctimesexpmtat_dn8 = assign24110_e27055_d_n8;

        let assign24120_e27058: f64 = (-230.25850929940458);
        let assign24120_e27059: f64 = if var_mtat > assign24120_e27058 { 1.0 } else { 0.0 };
        var_guard447 = assign24120_e27059;

        let (assign24130_e27077, assign24130_e27077_d_n5, assign24130_e27077_d_n6, assign24130_e27077_d_n7, assign24130_e27077_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) && (var_guard446 == 0.0)) && (var_guard447 != 0.0)) {
        let assign24130_e27075: f64 = (var_mtat).exp();
        (assign24130_e27075, (assign24130_e27075 * var_mtat_dn5), (assign24130_e27075 * var_mtat_dn6), (assign24130_e27075 * var_mtat_dn7), (assign24130_e27075 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24130_e27077;
        var_tmp_dn5 = assign24130_e27077_d_n5;
        var_tmp_dn6 = assign24130_e27077_d_n6;
        var_tmp_dn7 = assign24130_e27077_d_n7;
        var_tmp_dn8 = assign24130_e27077_d_n8;

        let (assign24140_e27120, assign24140_e27120_d_n5, assign24140_e27120_d_n6, assign24140_e27120_d_n7, assign24140_e27120_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) && (var_guard446 == 0.0)) && (var_guard447 == 0.0)) {
        let assign24140_e27096: f64 = (-230.25850929940458);
        let assign24140_e27098: f64 = (assign24140_e27096 - var_mtat);
        let assign24140_e27102: f64 = (-230.25850929940458);
        let assign24140_e27104: f64 = (assign24140_e27102 - var_mtat);
        let assign24140_e27107: f64 = (-230.25850929940458);
        let assign24140_e27109: f64 = (assign24140_e27107 - var_mtat);
        let assign24140_e27111: f64 = (assign24140_e27109 * 0.3333333333333333);
        let assign24140_e27112: f64 = (1.0 + assign24140_e27111);
        let assign24140_e27113: f64 = (assign24140_e27104 * assign24140_e27112);
        let assign24140_e27114: f64 = (0.5 * assign24140_e27113);
        let assign24140_e27115: f64 = (1.0 + assign24140_e27114);
        let assign24140_e27116: f64 = (assign24140_e27098 * assign24140_e27115);
        let assign24140_e27117: f64 = (1.0 + assign24140_e27116);
        let assign24140_e27118: f64 = (1e-100 / assign24140_e27117);
        (assign24140_e27118, (-((1e-100 * (((-var_mtat_dn5) * assign24140_e27115) + (assign24140_e27098 * (0.5 * (((-var_mtat_dn5) * assign24140_e27112) + (assign24140_e27104 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign24140_e27117 * assign24140_e27117))), (-((1e-100 * (((-var_mtat_dn6) * assign24140_e27115) + (assign24140_e27098 * (0.5 * (((-var_mtat_dn6) * assign24140_e27112) + (assign24140_e27104 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign24140_e27117 * assign24140_e27117))), (-((1e-100 * (((-var_mtat_dn7) * assign24140_e27115) + (assign24140_e27098 * (0.5 * (((-var_mtat_dn7) * assign24140_e27112) + (assign24140_e27104 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign24140_e27117 * assign24140_e27117))), (-((1e-100 * (((-var_mtat_dn8) * assign24140_e27115) + (assign24140_e27098 * (0.5 * (((-var_mtat_dn8) * assign24140_e27112) + (assign24140_e27104 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign24140_e27117 * assign24140_e27117))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24140_e27120;
        var_tmp_dn5 = assign24140_e27120_d_n5;
        var_tmp_dn6 = assign24140_e27120_d_n6;
        var_tmp_dn7 = assign24140_e27120_d_n7;
        var_tmp_dn8 = assign24140_e27120_d_n8;

        let (assign24150_e27139, assign24150_e27139_d_n5, assign24150_e27139_d_n6, assign24150_e27139_d_n7, assign24150_e27139_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) && (var_guard446 == 0.0)) {
        let assign24150_e27135: f64 = (2.0 * var_tmp);
        let assign24150_e27137: f64 = (assign24150_e27135 - var_erfcpos);
        (assign24150_e27137, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign24150_e27139;
        var_erfctimesexpmtat_dn5 = assign24150_e27139_d_n5;
        var_erfctimesexpmtat_dn6 = assign24150_e27139_d_n6;
        var_erfctimesexpmtat_dn7 = assign24150_e27139_d_n7;
        var_erfctimesexpmtat_dn8 = assign24150_e27139_d_n8;

        let (assign24160_e27159, assign24160_e27159_d_n5, assign24160_e27159_d_n6, assign24160_e27159_d_n7, assign24160_e27159_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) {
        let assign24160_e27151: f64 = (1.772453850905516 * 0.5);
        let assign24160_e27154: f64 = (var_atatsti * var_erfctimesexpmtat);
        let assign24160_e27156: f64 = (assign24160_e27154 / var_ktat);
        let assign24160_e27157: f64 = (assign24160_e27151 * assign24160_e27156);
        (assign24160_e27157, (assign24160_e27151 * ((((var_atatsti * var_erfctimesexpmtat_dn5) * var_ktat) - (assign24160_e27154 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign24160_e27151 * ((((var_atatsti * var_erfctimesexpmtat_dn6) * var_ktat) - (assign24160_e27154 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign24160_e27151 * ((((var_atatsti * var_erfctimesexpmtat_dn7) * var_ktat) - (assign24160_e27154 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign24160_e27151 * ((((var_atatsti * var_erfctimesexpmtat_dn8) * var_ktat) - (assign24160_e27154 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign24160_e27159;
        var_gammamax_dn5 = assign24160_e27159_d_n5;
        var_gammamax_dn6 = assign24160_e27159_d_n6;
        var_gammamax_dn7 = assign24160_e27159_d_n7;
        var_gammamax_dn8 = assign24160_e27159_d_n8;

        let (assign24170_e27177, assign24170_e27177_d_n5, assign24170_e27177_d_n6, assign24170_e27177_d_n7, assign24170_e27177_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard442 == 0.0)) {
        let assign24170_e27172: f64 = (var_asrh * var_gammamax);
        let assign24170_e27174: f64 = (assign24170_e27172 * var_wtat);
        let assign24170_e27175: f64 = (p.p846 * assign24170_e27174);
        (assign24170_e27175, (p.p846 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign24170_e27172 * var_wtat_dn5))), (p.p846 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign24170_e27172 * var_wtat_dn6))), (p.p846 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign24170_e27172 * var_wtat_dn7))), (p.p846 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign24170_e27172 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign24170_e27177;
        var_itat_dn5 = assign24170_e27177_d_n5;
        var_itat_dn6 = assign24170_e27177_d_n6;
        var_itat_dn7 = assign24170_e27177_d_n7;
        var_itat_dn8 = assign24170_e27177_d_n8;

        let assign24180_e27180: f64 = if p.p852 == 0.0 { 1.0 } else { 0.0 };
        var_guard448 = assign24180_e27180;

        let (assign24190_e27191, assign24190_e27191_d_n5, assign24190_e27191_d_n6, assign24190_e27191_d_n7, assign24190_e27191_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard448 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign24190_e27191;
        var_ibbt_dn5 = assign24190_e27191_d_n5;
        var_ibbt_dn6 = assign24190_e27191_d_n6;
        var_ibbt_dn7 = assign24190_e27191_d_n7;
        var_ibbt_dn8 = assign24190_e27191_d_n8;

        let assign24200_e27194: f64 = if p.p832 == 0.5 { 1.0 } else { 0.0 };
        var_guard449 = assign24200_e27194;

        let (assign24210_e27213, assign24210_e27213_d_n5, assign24210_e27213_d_n6, assign24210_e27213_d_n7, assign24210_e27213_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard448 == 0.0)) && (var_guard449 != 0.0)) {
        let assign24210_e27208: f64 = (p.p829 - var_vbbt);
        let assign24210_e27210: f64 = (assign24210_e27208 * var_vbirstiinv);
        let assign24210_e27211: f64 = (assign24210_e27210).sqrt();
        (assign24210_e27211, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24210_e27213;
        var_tmp_dn5 = assign24210_e27213_d_n5;
        var_tmp_dn6 = assign24210_e27213_d_n6;
        var_tmp_dn7 = assign24210_e27213_d_n7;
        var_tmp_dn8 = assign24210_e27213_d_n8;

        let (assign24220_e27234, assign24220_e27234_d_n5, assign24220_e27234_d_n6, assign24220_e27234_d_n7, assign24220_e27234_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard448 == 0.0)) && (var_guard449 == 0.0)) {
        let assign24220_e27228: f64 = (p.p829 - var_vbbt);
        let assign24220_e27230: f64 = (assign24220_e27228 * var_vbirstiinv);
        let assign24220_e27232: f64 = (assign24220_e27230).powf(p.p832);
        (assign24220_e27232, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24220_e27234;
        var_tmp_dn5 = assign24220_e27234_d_n5;
        var_tmp_dn6 = assign24220_e27234_d_n6;
        var_tmp_dn7 = assign24220_e27234_d_n7;
        var_tmp_dn8 = assign24220_e27234_d_n8;

        let (assign24230_e27254, assign24230_e27254_d_n5, assign24230_e27254_d_n6, assign24230_e27254_d_n7, assign24230_e27254_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard448 == 0.0)) {
        let assign24230_e27247: f64 = (p.p829 - var_vbbt);
        let assign24230_e27249: f64 = (assign24230_e27247 * var_wdepnulrinvsti);
        let assign24230_e27251: f64 = (assign24230_e27249 / var_tmp);
        let assign24230_e27252: f64 = (var_one_over_one_minus_psti * assign24230_e27251);
        (assign24230_e27252, (var_one_over_one_minus_psti * (-((assign24230_e27249 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign24230_e27249 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign24230_e27249 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign24230_e27249 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign24230_e27254;
        var_fmaxr_dn5 = assign24230_e27254_d_n5;
        var_fmaxr_dn6 = assign24230_e27254_d_n6;
        var_fmaxr_dn7 = assign24230_e27254_d_n7;
        var_fmaxr_dn8 = assign24230_e27254_d_n8;

        let assign24240_e27256: f64 = (-var_fbbtsti);
        let assign24240_e27258: f64 = (assign24240_e27256 / var_fmaxr);
        let assign24240_e27259: f64 = (assign24240_e27258).abs();
        let assign24240_e27261: f64 = if assign24240_e27259 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard450 = assign24240_e27261;

        let (assign24250_e27279, assign24250_e27279_d_n5, assign24250_e27279_d_n6, assign24250_e27279_d_n7, assign24250_e27279_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard448 == 0.0)) && (var_guard450 != 0.0)) {
        let assign24250_e27274: f64 = (-var_fbbtsti);
        let assign24250_e27276: f64 = (assign24250_e27274 / var_fmaxr);
        let assign24250_e27277: f64 = (assign24250_e27276).exp();
        (assign24250_e27277, (assign24250_e27277 * (-((assign24250_e27274 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))), (assign24250_e27277 * (-((assign24250_e27274 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign24250_e27277 * (-((assign24250_e27274 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign24250_e27277 * (-((assign24250_e27274 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24250_e27279;
        var_tmp_dn5 = assign24250_e27279_d_n5;
        var_tmp_dn6 = assign24250_e27279_d_n6;
        var_tmp_dn7 = assign24250_e27279_d_n7;
        var_tmp_dn8 = assign24250_e27279_d_n8;

        let assign24260_e27281: f64 = (-var_fbbtsti);
        let assign24260_e27283: f64 = (assign24260_e27281 / var_fmaxr);
        let assign24260_e27285: f64 = if assign24260_e27283 < 0.0 { 1.0 } else { 0.0 };
        var_guard451 = assign24260_e27285;

        let (assign24270_e27336, assign24270_e27336_d_n5, assign24270_e27336_d_n6, assign24270_e27336_d_n7, assign24270_e27336_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard448 == 0.0)) && (var_guard450 == 0.0)) && (var_guard451 != 0.0)) {
        let assign24270_e27303: f64 = (-230.25850929940458);
        let assign24270_e27305: f64 = (-var_fbbtsti);
        let assign24270_e27307: f64 = (assign24270_e27305 / var_fmaxr);
        let assign24270_e27308: f64 = (assign24270_e27303 - assign24270_e27307);
        let assign24270_e27312: f64 = (-230.25850929940458);
        let assign24270_e27314: f64 = (-var_fbbtsti);
        let assign24270_e27316: f64 = (assign24270_e27314 / var_fmaxr);
        let assign24270_e27317: f64 = (assign24270_e27312 - assign24270_e27316);
        let assign24270_e27320: f64 = (-230.25850929940458);
        let assign24270_e27322: f64 = (-var_fbbtsti);
        let assign24270_e27324: f64 = (assign24270_e27322 / var_fmaxr);
        let assign24270_e27325: f64 = (assign24270_e27320 - assign24270_e27324);
        let assign24270_e27327: f64 = (assign24270_e27325 * 0.3333333333333333);
        let assign24270_e27328: f64 = (1.0 + assign24270_e27327);
        let assign24270_e27329: f64 = (assign24270_e27317 * assign24270_e27328);
        let assign24270_e27330: f64 = (0.5 * assign24270_e27329);
        let assign24270_e27331: f64 = (1.0 + assign24270_e27330);
        let assign24270_e27332: f64 = (assign24270_e27308 * assign24270_e27331);
        let assign24270_e27333: f64 = (1.0 + assign24270_e27332);
        let assign24270_e27334: f64 = (1e-100 / assign24270_e27333);
        (assign24270_e27334, (-((1e-100 * (((-(-((assign24270_e27305 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign24270_e27331) + (assign24270_e27308 * (0.5 * (((-(-((assign24270_e27314 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * assign24270_e27328) + (assign24270_e27317 * ((-(-((assign24270_e27322 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign24270_e27333 * assign24270_e27333))), (-((1e-100 * (((-(-((assign24270_e27305 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign24270_e27331) + (assign24270_e27308 * (0.5 * (((-(-((assign24270_e27314 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign24270_e27328) + (assign24270_e27317 * ((-(-((assign24270_e27322 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign24270_e27333 * assign24270_e27333))), (-((1e-100 * (((-(-((assign24270_e27305 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign24270_e27331) + (assign24270_e27308 * (0.5 * (((-(-((assign24270_e27314 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign24270_e27328) + (assign24270_e27317 * ((-(-((assign24270_e27322 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign24270_e27333 * assign24270_e27333))), (-((1e-100 * (((-(-((assign24270_e27305 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign24270_e27331) + (assign24270_e27308 * (0.5 * (((-(-((assign24270_e27314 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign24270_e27328) + (assign24270_e27317 * ((-(-((assign24270_e27322 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign24270_e27333 * assign24270_e27333))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24270_e27336;
        var_tmp_dn5 = assign24270_e27336_d_n5;
        var_tmp_dn6 = assign24270_e27336_d_n6;
        var_tmp_dn7 = assign24270_e27336_d_n7;
        var_tmp_dn8 = assign24270_e27336_d_n8;

        let (assign24280_e27385, assign24280_e27385_d_n5, assign24280_e27385_d_n6, assign24280_e27385_d_n7, assign24280_e27385_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard448 == 0.0)) && (var_guard450 == 0.0)) && (var_guard451 == 0.0)) {
        let assign24280_e27355: f64 = (-var_fbbtsti);
        let assign24280_e27357: f64 = (assign24280_e27355 / var_fmaxr);
        let assign24280_e27359: f64 = (assign24280_e27357 - 230.25850929940458);
        let assign24280_e27363: f64 = (-var_fbbtsti);
        let assign24280_e27365: f64 = (assign24280_e27363 / var_fmaxr);
        let assign24280_e27367: f64 = (assign24280_e27365 - 230.25850929940458);
        let assign24280_e27370: f64 = (-var_fbbtsti);
        let assign24280_e27372: f64 = (assign24280_e27370 / var_fmaxr);
        let assign24280_e27374: f64 = (assign24280_e27372 - 230.25850929940458);
        let assign24280_e27376: f64 = (assign24280_e27374 * 0.3333333333333333);
        let assign24280_e27377: f64 = (1.0 + assign24280_e27376);
        let assign24280_e27378: f64 = (assign24280_e27367 * assign24280_e27377);
        let assign24280_e27379: f64 = (0.5 * assign24280_e27378);
        let assign24280_e27380: f64 = (1.0 + assign24280_e27379);
        let assign24280_e27381: f64 = (assign24280_e27359 * assign24280_e27380);
        let assign24280_e27382: f64 = (1.0 + assign24280_e27381);
        let assign24280_e27383: f64 = (1e100 * assign24280_e27382);
        (assign24280_e27383, (1e100 * (((-((assign24280_e27355 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign24280_e27380) + (assign24280_e27359 * (0.5 * (((-((assign24280_e27363 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * assign24280_e27377) + (assign24280_e27367 * ((-((assign24280_e27370 * var_fmaxr_dn5) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign24280_e27355 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign24280_e27380) + (assign24280_e27359 * (0.5 * (((-((assign24280_e27363 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign24280_e27377) + (assign24280_e27367 * ((-((assign24280_e27370 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign24280_e27355 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign24280_e27380) + (assign24280_e27359 * (0.5 * (((-((assign24280_e27363 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign24280_e27377) + (assign24280_e27367 * ((-((assign24280_e27370 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign24280_e27355 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign24280_e27380) + (assign24280_e27359 * (0.5 * (((-((assign24280_e27363 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign24280_e27377) + (assign24280_e27367 * ((-((assign24280_e27370 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24280_e27385;
        var_tmp_dn5 = assign24280_e27385_d_n5;
        var_tmp_dn6 = assign24280_e27385_d_n6;
        var_tmp_dn7 = assign24280_e27385_d_n7;
        var_tmp_dn8 = assign24280_e27385_d_n8;

        let (assign24290_e27405, assign24290_e27405_d_n5, assign24290_e27405_d_n6, assign24290_e27405_d_n7, assign24290_e27405_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard448 == 0.0)) {
        let assign24290_e27398: f64 = (var_v4 * var_fmaxr);
        let assign24290_e27400: f64 = (assign24290_e27398 * var_fmaxr);
        let assign24290_e27402: f64 = (assign24290_e27400 * var_tmp);
        let assign24290_e27403: f64 = (p.p852 * assign24290_e27402);
        (assign24290_e27403, (p.p852 * (((((var_v4 * var_fmaxr_dn5) * var_fmaxr) + (assign24290_e27398 * var_fmaxr_dn5)) * var_tmp) + (assign24290_e27400 * var_tmp_dn5))), (p.p852 * (((((var_v4 * var_fmaxr_dn6) * var_fmaxr) + (assign24290_e27398 * var_fmaxr_dn6)) * var_tmp) + (assign24290_e27400 * var_tmp_dn6))), (p.p852 * (((((var_v4 * var_fmaxr_dn7) * var_fmaxr) + (assign24290_e27398 * var_fmaxr_dn7)) * var_tmp) + (assign24290_e27400 * var_tmp_dn7))), (p.p852 * (((((var_v4 * var_fmaxr_dn8) * var_fmaxr) + (assign24290_e27398 * var_fmaxr_dn8)) * var_tmp) + (assign24290_e27400 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign24290_e27405;
        var_ibbt_dn5 = assign24290_e27405_d_n5;
        var_ibbt_dn6 = assign24290_e27405_d_n6;
        var_ibbt_dn7 = assign24290_e27405_d_n7;
        var_ibbt_dn8 = assign24290_e27405_d_n8;

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
        *var_guard444_slot = var_guard444;
        *var_guard445_slot = var_guard445;
        *var_guard446_slot = var_guard446;
        *var_guard447_slot = var_guard447;
        *var_guard448_slot = var_guard448;
        *var_guard449_slot = var_guard449;
        *var_guard450_slot = var_guard450;
        *var_guard451_slot = var_guard451;
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

    pub(super) fn stamp_transient_block_45(
        p: &Parameters,
        var_alphaav: f64,
        var_atatgat: f64,
        var_btatpartgat: f64,
        var_fstopsti: f64,
        var_ftdgat: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard438: f64,
        var_ibbt: f64,
        var_ibbt_dn5: f64,
        var_ibbt_dn6: f64,
        var_ibbt_dn7: f64,
        var_ibbt_dn8: f64,
        var_idmult: f64,
        var_idsatgat: f64,
        var_lgsource_i: f64,
        var_one_minus_pgat: f64,
        var_one_over_one_minus_pgat: f64,
        var_slopesti: f64,
        var_two_psistar: f64,
        var_vav: f64,
        var_vbigat: f64,
        var_vbirgatinv: f64,
        var_vbrinvsti: f64,
        var_vjsrh: f64,
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
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_guard452_slot: &mut f64,
        var_guard453_slot: &mut f64,
        var_guard454_slot: &mut f64,
        var_guard455_slot: &mut f64,
        var_guard456_slot: &mut f64,
        var_guard457_slot: &mut f64,
        var_guard458_slot: &mut f64,
        var_guard459_slot: &mut f64,
        var_guard460_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
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
        let mut var_guard452: f64 = *var_guard452_slot;
        let mut var_guard453: f64 = *var_guard453_slot;
        let mut var_guard454: f64 = *var_guard454_slot;
        let mut var_guard455: f64 = *var_guard455_slot;
        let mut var_guard456: f64 = *var_guard456_slot;
        let mut var_guard457: f64 = *var_guard457_slot;
        let mut var_guard458: f64 = *var_guard458_slot;
        let mut var_guard459: f64 = *var_guard459_slot;
        let mut var_guard460: f64 = *var_guard460_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
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

        let assign24300_e27408: f64 = if p.p861 > 1000.0 { 1.0 } else { 0.0 };
        var_guard452 = assign24300_e27408;

        let (assign24310_e27419, assign24310_e27419_d_n5, assign24310_e27419_d_n6, assign24310_e27419_d_n7, assign24310_e27419_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard452 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign24310_e27419;
        var_fbreakdown_dn5 = assign24310_e27419_d_n5;
        var_fbreakdown_dn6 = assign24310_e27419_d_n6;
        var_fbreakdown_dn7 = assign24310_e27419_d_n7;
        var_fbreakdown_dn8 = assign24310_e27419_d_n8;

        let assign24320_e27422: f64 = (-var_alphaav);
        let assign24320_e27424: f64 = (assign24320_e27422 * p.p861);
        let assign24320_e27425: f64 = if var_vav > assign24320_e27424 { 1.0 } else { 0.0 };
        var_guard453 = assign24320_e27425;

        let assign24330_e27428: f64 = if p.p864 == 4.0 { 1.0 } else { 0.0 };
        var_guard454 = assign24330_e27428;

        let (assign24340_e27458, assign24340_e27458_d_n5, assign24340_e27458_d_n6, assign24340_e27458_d_n7, assign24340_e27458_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard452 == 0.0)) && (var_guard453 != 0.0)) && (var_guard454 != 0.0)) {
        let assign24340_e27444: f64 = (var_vav * var_vbrinvsti);
        let assign24340_e27447: f64 = (var_vav * var_vbrinvsti);
        let assign24340_e27448: f64 = (assign24340_e27444 * assign24340_e27447);
        let assign24340_e27451: f64 = (var_vav * var_vbrinvsti);
        let assign24340_e27452: f64 = (assign24340_e27448 * assign24340_e27451);
        let assign24340_e27455: f64 = (var_vav * var_vbrinvsti);
        let assign24340_e27456: f64 = (assign24340_e27452 * assign24340_e27455);
        (assign24340_e27456, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24340_e27458;
        var_tmp_dn5 = assign24340_e27458_d_n5;
        var_tmp_dn6 = assign24340_e27458_d_n6;
        var_tmp_dn7 = assign24340_e27458_d_n7;
        var_tmp_dn8 = assign24340_e27458_d_n8;

        let (assign24350_e27480, assign24350_e27480_d_n5, assign24350_e27480_d_n6, assign24350_e27480_d_n7, assign24350_e27480_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard452 == 0.0)) && (var_guard453 != 0.0)) && (var_guard454 == 0.0)) {
        let assign24350_e27475: f64 = (var_vav * var_vbrinvsti);
        let assign24350_e27476: f64 = (assign24350_e27475).abs();
        let assign24350_e27478: f64 = (assign24350_e27476).powf(p.p864);
        (assign24350_e27478, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24350_e27480;
        var_tmp_dn5 = assign24350_e27480_d_n5;
        var_tmp_dn6 = assign24350_e27480_d_n6;
        var_tmp_dn7 = assign24350_e27480_d_n7;
        var_tmp_dn8 = assign24350_e27480_d_n8;

        let (assign24360_e27498, assign24360_e27498_d_n5, assign24360_e27498_d_n6, assign24360_e27498_d_n7, assign24360_e27498_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard452 == 0.0)) && (var_guard453 != 0.0)) {
        let assign24360_e27495: f64 = (1.0 - var_tmp);
        let assign24360_e27496: f64 = (1.0 / assign24360_e27495);
        (assign24360_e27496, (-((-var_tmp_dn5) / (assign24360_e27495 * assign24360_e27495))), (-((-var_tmp_dn6) / (assign24360_e27495 * assign24360_e27495))), (-((-var_tmp_dn7) / (assign24360_e27495 * assign24360_e27495))), (-((-var_tmp_dn8) / (assign24360_e27495 * assign24360_e27495))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign24360_e27498;
        var_fbreakdown_dn5 = assign24360_e27498_d_n5;
        var_fbreakdown_dn6 = assign24360_e27498_d_n6;
        var_fbreakdown_dn7 = assign24360_e27498_d_n7;
        var_fbreakdown_dn8 = assign24360_e27498_d_n8;

        let (assign24370_e27521, assign24370_e27521_d_n5, assign24370_e27521_d_n6, assign24370_e27521_d_n7, assign24370_e27521_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) && (var_guard452 == 0.0)) && (var_guard453 == 0.0)) {
        let assign24370_e27515: f64 = (var_alphaav * p.p861);
        let assign24370_e27516: f64 = (var_vav + assign24370_e27515);
        let assign24370_e27518: f64 = (assign24370_e27516 * var_slopesti);
        let assign24370_e27519: f64 = (var_fstopsti + assign24370_e27518);
        (assign24370_e27519, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign24370_e27521;
        var_fbreakdown_dn5 = assign24370_e27521_d_n5;
        var_fbreakdown_dn6 = assign24370_e27521_d_n6;
        var_fbreakdown_dn7 = assign24370_e27521_d_n7;
        var_fbreakdown_dn8 = assign24370_e27521_d_n8;

        let (assign24380_e27540, assign24380_e27540_d_n5, assign24380_e27540_d_n6, assign24380_e27540_d_n7, assign24380_e27540_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard438 == 0.0)) {
        let assign24380_e27531: f64 = (var_id__blk219 + var_isrh);
        let assign24380_e27533: f64 = (assign24380_e27531 + var_itat);
        let assign24380_e27535: f64 = (assign24380_e27533 + var_ibbt);
        let assign24380_e27536: f64 = (p.p29 * assign24380_e27535);
        let assign24380_e27538: f64 = (assign24380_e27536 * var_fbreakdown);
        (assign24380_e27538, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign24380_e27536 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign24380_e27536 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign24380_e27536 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign24380_e27536 * var_fbreakdown_dn8)),)
    } else {
        (var_ijunsti, var_ijunsti_dn5, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8,)
    }
};
        var_ijunsti = assign24380_e27540;
        var_ijunsti_dn5 = assign24380_e27540_d_n5;
        var_ijunsti_dn6 = assign24380_e27540_d_n6;
        var_ijunsti_dn7 = assign24380_e27540_d_n7;
        var_ijunsti_dn8 = assign24380_e27540_d_n8;

        let assign24390_e27543: f64 = if var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard455 = assign24390_e27543;

        let (assign24400_e27551, assign24400_e27551_d_n5, assign24400_e27551_d_n6, assign24400_e27551_d_n7, assign24400_e27551_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign24400_e27551;
        var_ijungat_dn5 = assign24400_e27551_d_n5;
        var_ijungat_dn6 = assign24400_e27551_d_n6;
        var_ijungat_dn7 = assign24400_e27551_d_n7;
        var_ijungat_dn8 = assign24400_e27551_d_n8;

        let (assign24410_e27562,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) {
        let assign24410_e27560: f64 = (var_idsatgat * var_idmult);
        (assign24410_e27560,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign24410_e27562;

        let assign24420_e27569: f64 = if ((p.p842 == 0.0) && (p.p847 == 0.0)) { 1.0 } else { 0.0 };
        var_guard456 = assign24420_e27569;

        let (assign24430_e27580, assign24430_e27580_d_n5, assign24430_e27580_d_n6, assign24430_e27580_d_n7, assign24430_e27580_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard456 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign24430_e27580;
        var_isrh_dn5 = assign24430_e27580_d_n5;
        var_isrh_dn6 = assign24430_e27580_d_n6;
        var_isrh_dn7 = assign24430_e27580_d_n7;
        var_isrh_dn8 = assign24430_e27580_d_n8;

        let (assign24440_e27594,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard456 == 0.0)) {
        let assign24440_e27592: f64 = (var_vbigat - var_vjsrh);
        (assign24440_e27592,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign24440_e27594;

        let (assign24450_e27613,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard456 == 0.0)) {
        let assign24450_e27608: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign24450_e27609: f64 = (1.0 - assign24450_e27608);
        let assign24450_e27610: f64 = (assign24450_e27609).sqrt();
        let assign24450_e27611: f64 = (1.0 - assign24450_e27610);
        (assign24450_e27611,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign24450_e27613;

        let assign24460_e27616: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        var_guard457 = assign24460_e27616;

        let (assign24470_e27630,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard456 == 0.0)) && (var_guard457 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign24470_e27630;

        let (assign24480_e27662,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard456 == 0.0)) && (var_guard457 == 0.0)) {
        let assign24480_e27645: f64 = (var_wsrhstep * var_wsrhstep);
        let assign24480_e27647: f64 = (var_wsrhstep).ln();
        let assign24480_e27648: f64 = (assign24480_e27645 * assign24480_e27647);
        let assign24480_e27651: f64 = (1.0 - var_wsrhstep);
        let assign24480_e27652: f64 = (assign24480_e27648 / assign24480_e27651);
        let assign24480_e27654: f64 = (assign24480_e27652 + var_wsrhstep);
        let assign24480_e27658: f64 = (2.0 * p.p833);
        let assign24480_e27659: f64 = (1.0 - assign24480_e27658);
        let assign24480_e27660: f64 = (assign24480_e27654 * assign24480_e27659);
        (assign24480_e27660,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign24480_e27662;

        let (assign24490_e27676,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard456 == 0.0)) {
        let assign24490_e27674: f64 = (var_wsrhstep + var_dwsrh);
        (assign24490_e27674,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign24490_e27676;

        let assign24500_e27679: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        var_guard458 = assign24500_e27679;

        let (assign24510_e27696, assign24510_e27696_d_n5, assign24510_e27696_d_n6, assign24510_e27696_d_n7, assign24510_e27696_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard456 == 0.0)) && (var_guard458 != 0.0)) {
        let assign24510_e27693: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign24510_e27694: f64 = (assign24510_e27693).sqrt();
        (assign24510_e27694, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24510_e27696;
        var_tmp_dn5 = assign24510_e27696_d_n5;
        var_tmp_dn6 = assign24510_e27696_d_n6;
        var_tmp_dn7 = assign24510_e27696_d_n7;
        var_tmp_dn8 = assign24510_e27696_d_n8;

        let (assign24520_e27715, assign24520_e27715_d_n5, assign24520_e27715_d_n6, assign24520_e27715_d_n7, assign24520_e27715_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard456 == 0.0)) && (var_guard458 == 0.0)) {
        let assign24520_e27711: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign24520_e27713: f64 = (assign24520_e27711).powf(p.p833);
        (assign24520_e27713, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24520_e27715;
        var_tmp_dn5 = assign24520_e27715_d_n5;
        var_tmp_dn6 = assign24520_e27715_d_n6;
        var_tmp_dn7 = assign24520_e27715_d_n7;
        var_tmp_dn8 = assign24520_e27715_d_n8;

        let (assign24530_e27729, assign24530_e27729_d_n5, assign24530_e27729_d_n6, assign24530_e27729_d_n7, assign24530_e27729_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard456 == 0.0)) {
        let assign24530_e27727: f64 = (var_wdepnulrgat * var_tmp);
        (assign24530_e27727, (var_wdepnulrgat * var_tmp_dn5), (var_wdepnulrgat * var_tmp_dn6), (var_wdepnulrgat * var_tmp_dn7), (var_wdepnulrgat * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign24530_e27729;
        var_wdep_dn5 = assign24530_e27729_d_n5;
        var_wdep_dn6 = assign24530_e27729_d_n6;
        var_wdep_dn7 = assign24530_e27729_d_n7;
        var_wdep_dn8 = assign24530_e27729_d_n8;

        let (assign24540_e27747, assign24540_e27747_d_n5, assign24540_e27747_d_n6, assign24540_e27747_d_n7, assign24540_e27747_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard456 == 0.0)) {
        let assign24540_e27742: f64 = (var_zinv - 1.0);
        let assign24540_e27744: f64 = (assign24540_e27742 * var_wdep);
        let assign24540_e27745: f64 = (var_ftdgat * assign24540_e27744);
        (assign24540_e27745, (var_ftdgat * (assign24540_e27742 * var_wdep_dn5)), (var_ftdgat * (assign24540_e27742 * var_wdep_dn6)), (var_ftdgat * (assign24540_e27742 * var_wdep_dn7)), (var_ftdgat * (assign24540_e27742 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign24540_e27747;
        var_asrh_dn5 = assign24540_e27747_d_n5;
        var_asrh_dn6 = assign24540_e27747_d_n6;
        var_asrh_dn7 = assign24540_e27747_d_n7;
        var_asrh_dn8 = assign24540_e27747_d_n8;

        let (assign24550_e27763, assign24550_e27763_d_n5, assign24550_e27763_d_n6, assign24550_e27763_d_n7, assign24550_e27763_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard456 == 0.0)) {
        let assign24550_e27760: f64 = (var_asrh * var_wsrh);
        let assign24550_e27761: f64 = (p.p842 * assign24550_e27760);
        (assign24550_e27761, (p.p842 * (var_asrh_dn5 * var_wsrh)), (p.p842 * (var_asrh_dn6 * var_wsrh)), (p.p842 * (var_asrh_dn7 * var_wsrh)), (p.p842 * (var_asrh_dn8 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign24550_e27763;
        var_isrh_dn5 = assign24550_e27763_d_n5;
        var_isrh_dn6 = assign24550_e27763_d_n6;
        var_isrh_dn7 = assign24550_e27763_d_n7;
        var_isrh_dn8 = assign24550_e27763_d_n8;

        let assign24560_e27766: f64 = if p.p847 == 0.0 { 1.0 } else { 0.0 };
        var_guard459 = assign24560_e27766;

        let (assign24570_e27777, assign24570_e27777_d_n5, assign24570_e27777_d_n6, assign24570_e27777_d_n7, assign24570_e27777_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign24570_e27777;
        var_itat_dn5 = assign24570_e27777_d_n5;
        var_itat_dn6 = assign24570_e27777_d_n6;
        var_itat_dn7 = assign24570_e27777_d_n7;
        var_itat_dn8 = assign24570_e27777_d_n8;

        let (assign24580_e27795, assign24580_e27795_d_n5, assign24580_e27795_d_n6, assign24580_e27795_d_n7, assign24580_e27795_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) {
        let assign24580_e27790: f64 = (var_wdep * var_one_minus_pgat);
        let assign24580_e27792: f64 = (assign24580_e27790 / var_vbi_minus_vjsrh);
        let assign24580_e27793: f64 = (var_btatpartgat * assign24580_e27792);
        (assign24580_e27793, (var_btatpartgat * ((var_wdep_dn5 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn6 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn7 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn8 * var_one_minus_pgat) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn5, var_btat_dn6, var_btat_dn7, var_btat_dn8,)
    }
};
        var_btat = assign24580_e27795;
        var_btat_dn5 = assign24580_e27795_d_n5;
        var_btat_dn6 = assign24580_e27795_d_n6;
        var_btat_dn7 = assign24580_e27795_d_n7;
        var_btat_dn8 = assign24580_e27795_d_n8;

        let (assign24590_e27811, assign24590_e27811_d_n5, assign24590_e27811_d_n6, assign24590_e27811_d_n7, assign24590_e27811_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) {
        let assign24590_e27807: f64 = (0.666666666666667 * var_atatgat);
        let assign24590_e27809: f64 = (assign24590_e27807 / var_btat);
        (assign24590_e27809, (-((assign24590_e27807 * var_btat_dn5) / (var_btat * var_btat))), (-((assign24590_e27807 * var_btat_dn6) / (var_btat * var_btat))), (-((assign24590_e27807 * var_btat_dn7) / (var_btat * var_btat))), (-((assign24590_e27807 * var_btat_dn8) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn5, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8,)
    }
};
        var_twoatatoverthreebtat = assign24590_e27811;
        var_twoatatoverthreebtat_dn5 = assign24590_e27811_d_n5;
        var_twoatatoverthreebtat_dn6 = assign24590_e27811_d_n6;
        var_twoatatoverthreebtat_dn7 = assign24590_e27811_d_n7;
        var_twoatatoverthreebtat_dn8 = assign24590_e27811_d_n8;

        let (assign24600_e27825, assign24600_e27825_d_n5, assign24600_e27825_d_n6, assign24600_e27825_d_n7, assign24600_e27825_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) {
        let assign24600_e27823: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign24600_e27823, ((var_twoatatoverthreebtat_dn5 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn5)), ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn5, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8,)
    }
};
        var_umaxbeforelimiting = assign24600_e27825;
        var_umaxbeforelimiting_dn5 = assign24600_e27825_d_n5;
        var_umaxbeforelimiting_dn6 = assign24600_e27825_d_n6;
        var_umaxbeforelimiting_dn7 = assign24600_e27825_d_n7;
        var_umaxbeforelimiting_dn8 = assign24600_e27825_d_n8;

        let (assign24610_e27846, assign24610_e27846_d_n5, assign24610_e27846_d_n6, assign24610_e27846_d_n7, assign24610_e27846_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) {
        let assign24610_e27837: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign24610_e27840: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign24610_e27842: f64 = (assign24610_e27840 + 1.0);
        let assign24610_e27843: f64 = (assign24610_e27837 / assign24610_e27842);
        let assign24610_e27844: f64 = (assign24610_e27843).sqrt();
        (assign24610_e27844, ((((((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)) * assign24610_e27842) - (assign24610_e27837 * ((var_umaxbeforelimiting_dn5 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn5)))) / (assign24610_e27842 * assign24610_e27842)) / (2.0 * assign24610_e27844)), ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign24610_e27842) - (assign24610_e27837 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign24610_e27842 * assign24610_e27842)) / (2.0 * assign24610_e27844)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign24610_e27842) - (assign24610_e27837 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign24610_e27842 * assign24610_e27842)) / (2.0 * assign24610_e27844)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign24610_e27842) - (assign24610_e27837 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign24610_e27842 * assign24610_e27842)) / (2.0 * assign24610_e27844)),)
    } else {
        (var_umax, var_umax_dn5, var_umax_dn6, var_umax_dn7, var_umax_dn8,)
    }
};
        var_umax = assign24610_e27846;
        var_umax_dn5 = assign24610_e27846_d_n5;
        var_umax_dn6 = assign24610_e27846_d_n6;
        var_umax_dn7 = assign24610_e27846_d_n7;
        var_umax_dn8 = assign24610_e27846_d_n8;

        let (assign24620_e27859, assign24620_e27859_d_n5, assign24620_e27859_d_n6, assign24620_e27859_d_n7, assign24620_e27859_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) {
        let assign24620_e27857: f64 = (var_umax).sqrt();
        (assign24620_e27857, (var_umax_dn5 / (2.0 * assign24620_e27857)), (var_umax_dn6 / (2.0 * assign24620_e27857)), (var_umax_dn7 / (2.0 * assign24620_e27857)), (var_umax_dn8 / (2.0 * assign24620_e27857)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn5, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8,)
    }
};
        var_sqrtumax = assign24620_e27859;
        var_sqrtumax_dn5 = assign24620_e27859_d_n5;
        var_sqrtumax_dn6 = assign24620_e27859_d_n6;
        var_sqrtumax_dn7 = assign24620_e27859_d_n7;
        var_sqrtumax_dn8 = assign24620_e27859_d_n8;

        let (assign24630_e27873, assign24630_e27873_d_n5, assign24630_e27873_d_n6, assign24630_e27873_d_n7, assign24630_e27873_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) {
        let assign24630_e27871: f64 = (var_umax * var_sqrtumax);
        (assign24630_e27871, ((var_umax_dn5 * var_sqrtumax) + (var_umax * var_sqrtumax_dn5)), ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn5, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8,)
    }
};
        var_umaxpoweronepointfive = assign24630_e27873;
        var_umaxpoweronepointfive_dn5 = assign24630_e27873_d_n5;
        var_umaxpoweronepointfive_dn6 = assign24630_e27873_d_n6;
        var_umaxpoweronepointfive_dn7 = assign24630_e27873_d_n7;
        var_umaxpoweronepointfive_dn8 = assign24630_e27873_d_n8;

        let assign24640_e27875: f64 = (-p.p833);
        let assign24640_e27877: f64 = (assign24640_e27875 * var_one_over_one_minus_pgat);
        let assign24640_e27879: f64 = (-1.0);
        let assign24640_e27880: f64 = if assign24640_e27877 == assign24640_e27879 { 1.0 } else { 0.0 };
        var_guard460 = assign24640_e27880;

        let (assign24650_e27900, assign24650_e27900_d_n5, assign24650_e27900_d_n6, assign24650_e27900_d_n7, assign24650_e27900_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) && (var_guard460 != 0.0)) {
        let assign24650_e27896: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign24650_e27897: f64 = (1.0 + assign24650_e27896);
        let assign24650_e27898: f64 = (1.0 / assign24650_e27897);
        (assign24650_e27898, (-(((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / (assign24650_e27897 * assign24650_e27897))), (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign24650_e27897 * assign24650_e27897))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign24650_e27897 * assign24650_e27897))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign24650_e27897 * assign24650_e27897))),)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign24650_e27900;
        var_wgamma_dn5 = assign24650_e27900_d_n5;
        var_wgamma_dn6 = assign24650_e27900_d_n6;
        var_wgamma_dn7 = assign24650_e27900_d_n7;
        var_wgamma_dn8 = assign24650_e27900_d_n8;

        let (assign24660_e27924, assign24660_e27924_d_n5, assign24660_e27924_d_n6, assign24660_e27924_d_n7, assign24660_e27924_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) && (var_guard460 == 0.0)) {
        let assign24660_e27916: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign24660_e27917: f64 = (1.0 + assign24660_e27916);
        let assign24660_e27919: f64 = (-p.p833);
        let assign24660_e27921: f64 = (assign24660_e27919 * var_one_over_one_minus_pgat);
        let assign24660_e27922: f64 = (assign24660_e27917).powf(assign24660_e27921);
        (assign24660_e27922, if 0.0 == 0.0 && ((assign24660_e27921) as f64).is_finite() && ((assign24660_e27921) as f64).fract() == 0.0 { if assign24660_e27921 == 0.0 { 0.0 } else { (assign24660_e27921 * ((assign24660_e27917).powf(assign24660_e27921 - 1.0) * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))) } } else { (assign24660_e27922 * (assign24660_e27921 * (((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)) / assign24660_e27917))) }, if 0.0 == 0.0 && ((assign24660_e27921) as f64).is_finite() && ((assign24660_e27921) as f64).fract() == 0.0 { if assign24660_e27921 == 0.0 { 0.0 } else { (assign24660_e27921 * ((assign24660_e27917).powf(assign24660_e27921 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign24660_e27922 * (assign24660_e27921 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign24660_e27917))) }, if 0.0 == 0.0 && ((assign24660_e27921) as f64).is_finite() && ((assign24660_e27921) as f64).fract() == 0.0 { if assign24660_e27921 == 0.0 { 0.0 } else { (assign24660_e27921 * ((assign24660_e27917).powf(assign24660_e27921 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign24660_e27922 * (assign24660_e27921 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign24660_e27917))) }, if 0.0 == 0.0 && ((assign24660_e27921) as f64).is_finite() && ((assign24660_e27921) as f64).fract() == 0.0 { if assign24660_e27921 == 0.0 { 0.0 } else { (assign24660_e27921 * ((assign24660_e27917).powf(assign24660_e27921 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign24660_e27922 * (assign24660_e27921 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign24660_e27917))) },)
    } else {
        (var_wgamma, var_wgamma_dn5, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8,)
    }
};
        var_wgamma = assign24660_e27924;
        var_wgamma_dn5 = assign24660_e27924_d_n5;
        var_wgamma_dn6 = assign24660_e27924_d_n6;
        var_wgamma_dn7 = assign24660_e27924_d_n7;
        var_wgamma_dn8 = assign24660_e27924_d_n8;

        let (assign24670_e27942, assign24670_e27942_d_n5, assign24670_e27942_d_n6, assign24670_e27942_d_n7, assign24670_e27942_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) {
        let assign24670_e27936: f64 = (var_wsrh * var_wgamma);
        let assign24670_e27939: f64 = (var_wsrh + var_wgamma);
        let assign24670_e27940: f64 = (assign24670_e27936 / assign24670_e27939);
        (assign24670_e27940, ((((var_wsrh * var_wgamma_dn5) * assign24670_e27939) - (assign24670_e27936 * var_wgamma_dn5)) / (assign24670_e27939 * assign24670_e27939)), ((((var_wsrh * var_wgamma_dn6) * assign24670_e27939) - (assign24670_e27936 * var_wgamma_dn6)) / (assign24670_e27939 * assign24670_e27939)), ((((var_wsrh * var_wgamma_dn7) * assign24670_e27939) - (assign24670_e27936 * var_wgamma_dn7)) / (assign24670_e27939 * assign24670_e27939)), ((((var_wsrh * var_wgamma_dn8) * assign24670_e27939) - (assign24670_e27936 * var_wgamma_dn8)) / (assign24670_e27939 * assign24670_e27939)),)
    } else {
        (var_wtat, var_wtat_dn5, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8,)
    }
};
        var_wtat = assign24670_e27942;
        var_wtat_dn5 = assign24670_e27942_d_n5;
        var_wtat_dn6 = assign24670_e27942_d_n6;
        var_wtat_dn7 = assign24670_e27942_d_n7;
        var_wtat_dn8 = assign24670_e27942_d_n8;

        let (assign24680_e27959, assign24680_e27959_d_n5, assign24680_e27959_d_n6, assign24680_e27959_d_n7, assign24680_e27959_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) {
        let assign24680_e27955: f64 = (var_btat / var_sqrtumax);
        let assign24680_e27956: f64 = (0.375 * assign24680_e27955);
        let assign24680_e27957: f64 = (assign24680_e27956).sqrt();
        (assign24680_e27957, ((0.375 * (((var_btat_dn5 * var_sqrtumax) - (var_btat * var_sqrtumax_dn5)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign24680_e27957)), ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign24680_e27957)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign24680_e27957)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign24680_e27957)),)
    } else {
        (var_ktat, var_ktat_dn5, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8,)
    }
};
        var_ktat = assign24680_e27959;
        var_ktat_dn5 = assign24680_e27959_d_n5;
        var_ktat_dn6 = assign24680_e27959_d_n6;
        var_ktat_dn7 = assign24680_e27959_d_n7;
        var_ktat_dn8 = assign24680_e27959_d_n8;

        let (assign24690_e27977, assign24690_e27977_d_n5, assign24690_e27977_d_n6, assign24690_e27977_d_n7, assign24690_e27977_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) {
        let assign24690_e27972: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign24690_e27973: f64 = (2.0 * assign24690_e27972);
        let assign24690_e27975: f64 = (assign24690_e27973 - var_umax);
        (assign24690_e27975, ((2.0 * ((var_twoatatoverthreebtat_dn5 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn5))) - var_umax_dn5), ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8),)
    } else {
        (var_ltat, var_ltat_dn5, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8,)
    }
};
        var_ltat = assign24690_e27977;
        var_ltat_dn5 = assign24690_e27977_d_n5;
        var_ltat_dn6 = assign24690_e27977_d_n6;
        var_ltat_dn7 = assign24690_e27977_d_n7;
        var_ltat_dn8 = assign24690_e27977_d_n8;

        let (assign24700_e28003, assign24700_e28003_d_n5, assign24700_e28003_d_n6, assign24700_e28003_d_n7, assign24700_e28003_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) {
        let assign24700_e27989: f64 = (var_atatgat * var_twoatatoverthreebtat);
        let assign24700_e27991: f64 = (assign24700_e27989 * var_sqrtumax);
        let assign24700_e27994: f64 = (var_atatgat * var_umax);
        let assign24700_e27995: f64 = (assign24700_e27991 - assign24700_e27994);
        let assign24700_e27999: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign24700_e28000: f64 = (0.5 * assign24700_e27999);
        let assign24700_e28001: f64 = (assign24700_e27995 + assign24700_e28000);
        (assign24700_e28001, (((((var_atatgat * var_twoatatoverthreebtat_dn5) * var_sqrtumax) + (assign24700_e27989 * var_sqrtumax_dn5)) - (var_atatgat * var_umax_dn5)) + (0.5 * ((var_btat_dn5 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn5)))), (((((var_atatgat * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign24700_e27989 * var_sqrtumax_dn6)) - (var_atatgat * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign24700_e27989 * var_sqrtumax_dn7)) - (var_atatgat * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign24700_e27989 * var_sqrtumax_dn8)) - (var_atatgat * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))),)
    } else {
        (var_mtat, var_mtat_dn5, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8,)
    }
};
        var_mtat = assign24700_e28003;
        var_mtat_dn5 = assign24700_e28003_d_n5;
        var_mtat_dn6 = assign24700_e28003_d_n6;
        var_mtat_dn7 = assign24700_e28003_d_n7;
        var_mtat_dn8 = assign24700_e28003_d_n8;

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
        *var_guard452_slot = var_guard452;
        *var_guard453_slot = var_guard453;
        *var_guard454_slot = var_guard454;
        *var_guard455_slot = var_guard455;
        *var_guard456_slot = var_guard456;
        *var_guard457_slot = var_guard457;
        *var_guard458_slot = var_guard458;
        *var_guard459_slot = var_guard459;
        *var_guard460_slot = var_guard460;
        *var_id__blk219_slot = var_id__blk219;
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
    }

    pub(super) fn stamp_transient_block_46(
        p: &Parameters,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn5: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_atatgat: f64,
        var_berfc: f64,
        var_cerfc: f64,
        var_fbbtgat: f64,
        var_fbbtgat_dn5: f64,
        var_fbbtgat_dn6: f64,
        var_fbbtgat_dn7: f64,
        var_fbbtgat_dn8: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard455: f64,
        var_guard459: f64,
        var_ktat: f64,
        var_ktat_dn5: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_ltat: f64,
        var_ltat_dn5: f64,
        var_ltat_dn6: f64,
        var_ltat_dn7: f64,
        var_ltat_dn8: f64,
        var_mtat: f64,
        var_mtat_dn5: f64,
        var_mtat_dn6: f64,
        var_mtat_dn7: f64,
        var_mtat_dn8: f64,
        var_one_over_one_minus_pgat: f64,
        var_perfc: f64,
        var_v4: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirgatinv: f64,
        var_vbrinvgat: f64,
        var_vbrinvgat_dn5: f64,
        var_vbrinvgat_dn6: f64,
        var_vbrinvgat_dn7: f64,
        var_vbrinvgat_dn8: f64,
        var_wdepnulrinvgat: f64,
        var_wtat: f64,
        var_wtat_dn5: f64,
        var_wtat_dn6: f64,
        var_wtat_dn7: f64,
        var_wtat_dn8: f64,
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
        var_guard461_slot: &mut f64,
        var_guard462_slot: &mut f64,
        var_guard463_slot: &mut f64,
        var_guard464_slot: &mut f64,
        var_guard465_slot: &mut f64,
        var_guard466_slot: &mut f64,
        var_guard467_slot: &mut f64,
        var_guard468_slot: &mut f64,
        var_guard469_slot: &mut f64,
        var_guard470_slot: &mut f64,
        var_guard471_slot: &mut f64,
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
        let mut var_guard461: f64 = *var_guard461_slot;
        let mut var_guard462: f64 = *var_guard462_slot;
        let mut var_guard463: f64 = *var_guard463_slot;
        let mut var_guard464: f64 = *var_guard464_slot;
        let mut var_guard465: f64 = *var_guard465_slot;
        let mut var_guard466: f64 = *var_guard466_slot;
        let mut var_guard467: f64 = *var_guard467_slot;
        let mut var_guard468: f64 = *var_guard468_slot;
        let mut var_guard469: f64 = *var_guard469_slot;
        let mut var_guard470: f64 = *var_guard470_slot;
        let mut var_guard471: f64 = *var_guard471_slot;
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

        let (assign24710_e28019, assign24710_e28019_d_n5, assign24710_e28019_d_n6, assign24710_e28019_d_n7, assign24710_e28019_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) {
        let assign24710_e28015: f64 = (var_ltat - 1.0);
        let assign24710_e28017: f64 = (assign24710_e28015 * var_ktat);
        (assign24710_e28017, ((var_ltat_dn5 * var_ktat) + (assign24710_e28015 * var_ktat_dn5)), ((var_ltat_dn6 * var_ktat) + (assign24710_e28015 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign24710_e28015 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign24710_e28015 * var_ktat_dn8)),)
    } else {
        (var_xerfc, var_xerfc_dn5, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8,)
    }
};
        var_xerfc = assign24710_e28019;
        var_xerfc_dn5 = assign24710_e28019_d_n5;
        var_xerfc_dn6 = assign24710_e28019_d_n6;
        var_xerfc_dn7 = assign24710_e28019_d_n7;
        var_xerfc_dn8 = assign24710_e28019_d_n8;

        let (assign24720_e28033, assign24720_e28033_d_n5, assign24720_e28033_d_n6, assign24720_e28033_d_n7, assign24720_e28033_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) {
        let assign24720_e28031: f64 = (var_xerfc * var_xerfc);
        (assign24720_e28031, ((var_xerfc_dn5 * var_xerfc) + (var_xerfc * var_xerfc_dn5)), ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)),)
    } else {
        (var_ysq, var_ysq_dn5, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8,)
    }
};
        var_ysq = assign24720_e28033;
        var_ysq_dn5 = assign24720_e28033_d_n5;
        var_ysq_dn6 = assign24720_e28033_d_n6;
        var_ysq_dn7 = assign24720_e28033_d_n7;
        var_ysq_dn8 = assign24720_e28033_d_n8;

        let assign24730_e28036: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard461 = assign24730_e28036;

        let (assign24740_e28056, assign24740_e28056_d_n5, assign24740_e28056_d_n6, assign24740_e28056_d_n7, assign24740_e28056_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) && (var_guard461 != 0.0)) {
        let assign24740_e28052: f64 = (var_perfc * var_xerfc);
        let assign24740_e28053: f64 = (1.0 + assign24740_e28052);
        let assign24740_e28054: f64 = (1.0 / assign24740_e28053);
        (assign24740_e28054, (-((var_perfc * var_xerfc_dn5) / (assign24740_e28053 * assign24740_e28053))), (-((var_perfc * var_xerfc_dn6) / (assign24740_e28053 * assign24740_e28053))), (-((var_perfc * var_xerfc_dn7) / (assign24740_e28053 * assign24740_e28053))), (-((var_perfc * var_xerfc_dn8) / (assign24740_e28053 * assign24740_e28053))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign24740_e28056;
        var_terfc_dn5 = assign24740_e28056_d_n5;
        var_terfc_dn6 = assign24740_e28056_d_n6;
        var_terfc_dn7 = assign24740_e28056_d_n7;
        var_terfc_dn8 = assign24740_e28056_d_n8;

        let (assign24750_e28077, assign24750_e28077_d_n5, assign24750_e28077_d_n6, assign24750_e28077_d_n7, assign24750_e28077_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) && (var_guard461 == 0.0)) {
        let assign24750_e28073: f64 = (var_perfc * var_xerfc);
        let assign24750_e28074: f64 = (1.0 - assign24750_e28073);
        let assign24750_e28075: f64 = (1.0 / assign24750_e28074);
        (assign24750_e28075, (-((-(var_perfc * var_xerfc_dn5)) / (assign24750_e28074 * assign24750_e28074))), (-((-(var_perfc * var_xerfc_dn6)) / (assign24750_e28074 * assign24750_e28074))), (-((-(var_perfc * var_xerfc_dn7)) / (assign24750_e28074 * assign24750_e28074))), (-((-(var_perfc * var_xerfc_dn8)) / (assign24750_e28074 * assign24750_e28074))),)
    } else {
        (var_terfc, var_terfc_dn5, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8,)
    }
};
        var_terfc = assign24750_e28077;
        var_terfc_dn5 = assign24750_e28077_d_n5;
        var_terfc_dn6 = assign24750_e28077_d_n6;
        var_terfc_dn7 = assign24750_e28077_d_n7;
        var_terfc_dn8 = assign24750_e28077_d_n8;

        let assign24760_e28079: f64 = (-var_ysq);
        let assign24760_e28081: f64 = (assign24760_e28079 + var_mtat);
        let assign24760_e28083: f64 = (-230.25850929940458);
        let assign24760_e28084: f64 = if assign24760_e28081 > assign24760_e28083 { 1.0 } else { 0.0 };
        var_guard462 = assign24760_e28084;

        let (assign24770_e28102, assign24770_e28102_d_n5, assign24770_e28102_d_n6, assign24770_e28102_d_n7, assign24770_e28102_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) && (var_guard462 != 0.0)) {
        let assign24770_e28097: f64 = (-var_ysq);
        let assign24770_e28099: f64 = (assign24770_e28097 + var_mtat);
        let assign24770_e28100: f64 = (assign24770_e28099).exp();
        (assign24770_e28100, (assign24770_e28100 * ((-var_ysq_dn5) + var_mtat_dn5)), (assign24770_e28100 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign24770_e28100 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign24770_e28100 * ((-var_ysq_dn8) + var_mtat_dn8)),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24770_e28102;
        var_tmp_dn5 = assign24770_e28102_d_n5;
        var_tmp_dn6 = assign24770_e28102_d_n6;
        var_tmp_dn7 = assign24770_e28102_d_n7;
        var_tmp_dn8 = assign24770_e28102_d_n8;

        let (assign24780_e28151, assign24780_e28151_d_n5, assign24780_e28151_d_n6, assign24780_e28151_d_n7, assign24780_e28151_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) && (var_guard462 == 0.0)) {
        let assign24780_e28118: f64 = (-230.25850929940458);
        let assign24780_e28120: f64 = (-var_ysq);
        let assign24780_e28122: f64 = (assign24780_e28120 + var_mtat);
        let assign24780_e28123: f64 = (assign24780_e28118 - assign24780_e28122);
        let assign24780_e28127: f64 = (-230.25850929940458);
        let assign24780_e28129: f64 = (-var_ysq);
        let assign24780_e28131: f64 = (assign24780_e28129 + var_mtat);
        let assign24780_e28132: f64 = (assign24780_e28127 - assign24780_e28131);
        let assign24780_e28135: f64 = (-230.25850929940458);
        let assign24780_e28137: f64 = (-var_ysq);
        let assign24780_e28139: f64 = (assign24780_e28137 + var_mtat);
        let assign24780_e28140: f64 = (assign24780_e28135 - assign24780_e28139);
        let assign24780_e28142: f64 = (assign24780_e28140 * 0.3333333333333333);
        let assign24780_e28143: f64 = (1.0 + assign24780_e28142);
        let assign24780_e28144: f64 = (assign24780_e28132 * assign24780_e28143);
        let assign24780_e28145: f64 = (0.5 * assign24780_e28144);
        let assign24780_e28146: f64 = (1.0 + assign24780_e28145);
        let assign24780_e28147: f64 = (assign24780_e28123 * assign24780_e28146);
        let assign24780_e28148: f64 = (1.0 + assign24780_e28147);
        let assign24780_e28149: f64 = (1e-100 / assign24780_e28148);
        (assign24780_e28149, (-((1e-100 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign24780_e28146) + (assign24780_e28123 * (0.5 * (((-((-var_ysq_dn5) + var_mtat_dn5)) * assign24780_e28143) + (assign24780_e28132 * ((-((-var_ysq_dn5) + var_mtat_dn5)) * 0.3333333333333333))))))) / (assign24780_e28148 * assign24780_e28148))), (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign24780_e28146) + (assign24780_e28123 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign24780_e28143) + (assign24780_e28132 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign24780_e28148 * assign24780_e28148))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign24780_e28146) + (assign24780_e28123 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign24780_e28143) + (assign24780_e28132 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign24780_e28148 * assign24780_e28148))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign24780_e28146) + (assign24780_e28123 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign24780_e28143) + (assign24780_e28132 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign24780_e28148 * assign24780_e28148))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24780_e28151;
        var_tmp_dn5 = assign24780_e28151_d_n5;
        var_tmp_dn6 = assign24780_e28151_d_n6;
        var_tmp_dn7 = assign24780_e28151_d_n7;
        var_tmp_dn8 = assign24780_e28151_d_n8;

        let (assign24790_e28181, assign24790_e28181_d_n5, assign24790_e28181_d_n6, assign24790_e28181_d_n7, assign24790_e28181_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) {
        let assign24790_e28163: f64 = (0.29214664 * var_terfc);
        let assign24790_e28167: f64 = (var_terfc * var_terfc);
        let assign24790_e28168: f64 = (var_berfc * assign24790_e28167);
        let assign24790_e28169: f64 = (assign24790_e28163 + assign24790_e28168);
        let assign24790_e28173: f64 = (var_terfc * var_terfc);
        let assign24790_e28175: f64 = (assign24790_e28173 * var_terfc);
        let assign24790_e28176: f64 = (var_cerfc * assign24790_e28175);
        let assign24790_e28177: f64 = (assign24790_e28169 + assign24790_e28176);
        let assign24790_e28179: f64 = (assign24790_e28177 * var_tmp);
        (assign24790_e28179, (((((0.29214664 * var_terfc_dn5) + (var_berfc * ((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)))) + (var_cerfc * ((((var_terfc_dn5 * var_terfc) + (var_terfc * var_terfc_dn5)) * var_terfc) + (assign24790_e28173 * var_terfc_dn5)))) * var_tmp) + (assign24790_e28177 * var_tmp_dn5)), (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign24790_e28173 * var_terfc_dn6)))) * var_tmp) + (assign24790_e28177 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign24790_e28173 * var_terfc_dn7)))) * var_tmp) + (assign24790_e28177 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign24790_e28173 * var_terfc_dn8)))) * var_tmp) + (assign24790_e28177 * var_tmp_dn8)),)
    } else {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    }
};
        var_erfcpos = assign24790_e28181;
        var_erfcpos_dn5 = assign24790_e28181_d_n5;
        var_erfcpos_dn6 = assign24790_e28181_d_n6;
        var_erfcpos_dn7 = assign24790_e28181_d_n7;
        var_erfcpos_dn8 = assign24790_e28181_d_n8;

        let assign24800_e28184: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard463 = assign24800_e28184;

        let (assign24810_e28198, assign24810_e28198_d_n5, assign24810_e28198_d_n6, assign24810_e28198_d_n7, assign24810_e28198_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) && (var_guard463 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn5, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign24810_e28198;
        var_erfctimesexpmtat_dn5 = assign24810_e28198_d_n5;
        var_erfctimesexpmtat_dn6 = assign24810_e28198_d_n6;
        var_erfctimesexpmtat_dn7 = assign24810_e28198_d_n7;
        var_erfctimesexpmtat_dn8 = assign24810_e28198_d_n8;

        let assign24820_e28201: f64 = (-230.25850929940458);
        let assign24820_e28202: f64 = if var_mtat > assign24820_e28201 { 1.0 } else { 0.0 };
        var_guard464 = assign24820_e28202;

        let (assign24830_e28220, assign24830_e28220_d_n5, assign24830_e28220_d_n6, assign24830_e28220_d_n7, assign24830_e28220_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) && (var_guard463 == 0.0)) && (var_guard464 != 0.0)) {
        let assign24830_e28218: f64 = (var_mtat).exp();
        (assign24830_e28218, (assign24830_e28218 * var_mtat_dn5), (assign24830_e28218 * var_mtat_dn6), (assign24830_e28218 * var_mtat_dn7), (assign24830_e28218 * var_mtat_dn8),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24830_e28220;
        var_tmp_dn5 = assign24830_e28220_d_n5;
        var_tmp_dn6 = assign24830_e28220_d_n6;
        var_tmp_dn7 = assign24830_e28220_d_n7;
        var_tmp_dn8 = assign24830_e28220_d_n8;

        let (assign24840_e28263, assign24840_e28263_d_n5, assign24840_e28263_d_n6, assign24840_e28263_d_n7, assign24840_e28263_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) && (var_guard463 == 0.0)) && (var_guard464 == 0.0)) {
        let assign24840_e28239: f64 = (-230.25850929940458);
        let assign24840_e28241: f64 = (assign24840_e28239 - var_mtat);
        let assign24840_e28245: f64 = (-230.25850929940458);
        let assign24840_e28247: f64 = (assign24840_e28245 - var_mtat);
        let assign24840_e28250: f64 = (-230.25850929940458);
        let assign24840_e28252: f64 = (assign24840_e28250 - var_mtat);
        let assign24840_e28254: f64 = (assign24840_e28252 * 0.3333333333333333);
        let assign24840_e28255: f64 = (1.0 + assign24840_e28254);
        let assign24840_e28256: f64 = (assign24840_e28247 * assign24840_e28255);
        let assign24840_e28257: f64 = (0.5 * assign24840_e28256);
        let assign24840_e28258: f64 = (1.0 + assign24840_e28257);
        let assign24840_e28259: f64 = (assign24840_e28241 * assign24840_e28258);
        let assign24840_e28260: f64 = (1.0 + assign24840_e28259);
        let assign24840_e28261: f64 = (1e-100 / assign24840_e28260);
        (assign24840_e28261, (-((1e-100 * (((-var_mtat_dn5) * assign24840_e28258) + (assign24840_e28241 * (0.5 * (((-var_mtat_dn5) * assign24840_e28255) + (assign24840_e28247 * ((-var_mtat_dn5) * 0.3333333333333333))))))) / (assign24840_e28260 * assign24840_e28260))), (-((1e-100 * (((-var_mtat_dn6) * assign24840_e28258) + (assign24840_e28241 * (0.5 * (((-var_mtat_dn6) * assign24840_e28255) + (assign24840_e28247 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign24840_e28260 * assign24840_e28260))), (-((1e-100 * (((-var_mtat_dn7) * assign24840_e28258) + (assign24840_e28241 * (0.5 * (((-var_mtat_dn7) * assign24840_e28255) + (assign24840_e28247 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign24840_e28260 * assign24840_e28260))), (-((1e-100 * (((-var_mtat_dn8) * assign24840_e28258) + (assign24840_e28241 * (0.5 * (((-var_mtat_dn8) * assign24840_e28255) + (assign24840_e28247 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign24840_e28260 * assign24840_e28260))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24840_e28263;
        var_tmp_dn5 = assign24840_e28263_d_n5;
        var_tmp_dn6 = assign24840_e28263_d_n6;
        var_tmp_dn7 = assign24840_e28263_d_n7;
        var_tmp_dn8 = assign24840_e28263_d_n8;

        let (assign24850_e28282, assign24850_e28282_d_n5, assign24850_e28282_d_n6, assign24850_e28282_d_n7, assign24850_e28282_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) && (var_guard463 == 0.0)) {
        let assign24850_e28278: f64 = (2.0 * var_tmp);
        let assign24850_e28280: f64 = (assign24850_e28278 - var_erfcpos);
        (assign24850_e28280, ((2.0 * var_tmp_dn5) - var_erfcpos_dn5), ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn5, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8,)
    }
};
        var_erfctimesexpmtat = assign24850_e28282;
        var_erfctimesexpmtat_dn5 = assign24850_e28282_d_n5;
        var_erfctimesexpmtat_dn6 = assign24850_e28282_d_n6;
        var_erfctimesexpmtat_dn7 = assign24850_e28282_d_n7;
        var_erfctimesexpmtat_dn8 = assign24850_e28282_d_n8;

        let (assign24860_e28302, assign24860_e28302_d_n5, assign24860_e28302_d_n6, assign24860_e28302_d_n7, assign24860_e28302_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) {
        let assign24860_e28294: f64 = (1.772453850905516 * 0.5);
        let assign24860_e28297: f64 = (var_atatgat * var_erfctimesexpmtat);
        let assign24860_e28299: f64 = (assign24860_e28297 / var_ktat);
        let assign24860_e28300: f64 = (assign24860_e28294 * assign24860_e28299);
        (assign24860_e28300, (assign24860_e28294 * ((((var_atatgat * var_erfctimesexpmtat_dn5) * var_ktat) - (assign24860_e28297 * var_ktat_dn5)) / (var_ktat * var_ktat))), (assign24860_e28294 * ((((var_atatgat * var_erfctimesexpmtat_dn6) * var_ktat) - (assign24860_e28297 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign24860_e28294 * ((((var_atatgat * var_erfctimesexpmtat_dn7) * var_ktat) - (assign24860_e28297 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign24860_e28294 * ((((var_atatgat * var_erfctimesexpmtat_dn8) * var_ktat) - (assign24860_e28297 * var_ktat_dn8)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn5, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8,)
    }
};
        var_gammamax = assign24860_e28302;
        var_gammamax_dn5 = assign24860_e28302_d_n5;
        var_gammamax_dn6 = assign24860_e28302_d_n6;
        var_gammamax_dn7 = assign24860_e28302_d_n7;
        var_gammamax_dn8 = assign24860_e28302_d_n8;

        let (assign24870_e28320, assign24870_e28320_d_n5, assign24870_e28320_d_n6, assign24870_e28320_d_n7, assign24870_e28320_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard459 == 0.0)) {
        let assign24870_e28315: f64 = (var_asrh * var_gammamax);
        let assign24870_e28317: f64 = (assign24870_e28315 * var_wtat);
        let assign24870_e28318: f64 = (p.p847 * assign24870_e28317);
        (assign24870_e28318, (p.p847 * ((((var_asrh_dn5 * var_gammamax) + (var_asrh * var_gammamax_dn5)) * var_wtat) + (assign24870_e28315 * var_wtat_dn5))), (p.p847 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign24870_e28315 * var_wtat_dn6))), (p.p847 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign24870_e28315 * var_wtat_dn7))), (p.p847 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign24870_e28315 * var_wtat_dn8))),)
    } else {
        (var_itat, var_itat_dn5, var_itat_dn6, var_itat_dn7, var_itat_dn8,)
    }
};
        var_itat = assign24870_e28320;
        var_itat_dn5 = assign24870_e28320_d_n5;
        var_itat_dn6 = assign24870_e28320_d_n6;
        var_itat_dn7 = assign24870_e28320_d_n7;
        var_itat_dn8 = assign24870_e28320_d_n8;

        let assign24880_e28323: f64 = if p.p853 == 0.0 { 1.0 } else { 0.0 };
        var_guard465 = assign24880_e28323;

        let (assign24890_e28334, assign24890_e28334_d_n5, assign24890_e28334_d_n6, assign24890_e28334_d_n7, assign24890_e28334_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard465 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign24890_e28334;
        var_ibbt_dn5 = assign24890_e28334_d_n5;
        var_ibbt_dn6 = assign24890_e28334_d_n6;
        var_ibbt_dn7 = assign24890_e28334_d_n7;
        var_ibbt_dn8 = assign24890_e28334_d_n8;

        let assign24900_e28337: f64 = if p.p833 == 0.5 { 1.0 } else { 0.0 };
        var_guard466 = assign24900_e28337;

        let (assign24910_e28356, assign24910_e28356_d_n5, assign24910_e28356_d_n6, assign24910_e28356_d_n7, assign24910_e28356_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard465 == 0.0)) && (var_guard466 != 0.0)) {
        let assign24910_e28351: f64 = (p.p830 - var_vbbt);
        let assign24910_e28353: f64 = (assign24910_e28351 * var_vbirgatinv);
        let assign24910_e28354: f64 = (assign24910_e28353).sqrt();
        (assign24910_e28354, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24910_e28356;
        var_tmp_dn5 = assign24910_e28356_d_n5;
        var_tmp_dn6 = assign24910_e28356_d_n6;
        var_tmp_dn7 = assign24910_e28356_d_n7;
        var_tmp_dn8 = assign24910_e28356_d_n8;

        let (assign24920_e28377, assign24920_e28377_d_n5, assign24920_e28377_d_n6, assign24920_e28377_d_n7, assign24920_e28377_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard465 == 0.0)) && (var_guard466 == 0.0)) {
        let assign24920_e28371: f64 = (p.p830 - var_vbbt);
        let assign24920_e28373: f64 = (assign24920_e28371 * var_vbirgatinv);
        let assign24920_e28375: f64 = (assign24920_e28373).powf(p.p833);
        (assign24920_e28375, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24920_e28377;
        var_tmp_dn5 = assign24920_e28377_d_n5;
        var_tmp_dn6 = assign24920_e28377_d_n6;
        var_tmp_dn7 = assign24920_e28377_d_n7;
        var_tmp_dn8 = assign24920_e28377_d_n8;

        let (assign24930_e28397, assign24930_e28397_d_n5, assign24930_e28397_d_n6, assign24930_e28397_d_n7, assign24930_e28397_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard465 == 0.0)) {
        let assign24930_e28390: f64 = (p.p830 - var_vbbt);
        let assign24930_e28392: f64 = (assign24930_e28390 * var_wdepnulrinvgat);
        let assign24930_e28394: f64 = (assign24930_e28392 / var_tmp);
        let assign24930_e28395: f64 = (var_one_over_one_minus_pgat * assign24930_e28394);
        (assign24930_e28395, (var_one_over_one_minus_pgat * (-((assign24930_e28392 * var_tmp_dn5) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign24930_e28392 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign24930_e28392 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign24930_e28392 * var_tmp_dn8) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn5, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8,)
    }
};
        var_fmaxr = assign24930_e28397;
        var_fmaxr_dn5 = assign24930_e28397_d_n5;
        var_fmaxr_dn6 = assign24930_e28397_d_n6;
        var_fmaxr_dn7 = assign24930_e28397_d_n7;
        var_fmaxr_dn8 = assign24930_e28397_d_n8;

        let assign24940_e28399: f64 = (-var_fbbtgat);
        let assign24940_e28401: f64 = (assign24940_e28399 / var_fmaxr);
        let assign24940_e28402: f64 = (assign24940_e28401).abs();
        let assign24940_e28404: f64 = if assign24940_e28402 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard467 = assign24940_e28404;

        let (assign24950_e28422, assign24950_e28422_d_n5, assign24950_e28422_d_n6, assign24950_e28422_d_n7, assign24950_e28422_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard465 == 0.0)) && (var_guard467 != 0.0)) {
        let assign24950_e28417: f64 = (-var_fbbtgat);
        let assign24950_e28419: f64 = (assign24950_e28417 / var_fmaxr);
        let assign24950_e28420: f64 = (assign24950_e28419).exp();
        (assign24950_e28420, (assign24950_e28420 * ((((-var_fbbtgat_dn5) * var_fmaxr) - (assign24950_e28417 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))), (assign24950_e28420 * ((((-var_fbbtgat_dn6) * var_fmaxr) - (assign24950_e28417 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign24950_e28420 * ((((-var_fbbtgat_dn7) * var_fmaxr) - (assign24950_e28417 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign24950_e28420 * ((((-var_fbbtgat_dn8) * var_fmaxr) - (assign24950_e28417 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24950_e28422;
        var_tmp_dn5 = assign24950_e28422_d_n5;
        var_tmp_dn6 = assign24950_e28422_d_n6;
        var_tmp_dn7 = assign24950_e28422_d_n7;
        var_tmp_dn8 = assign24950_e28422_d_n8;

        let assign24960_e28424: f64 = (-var_fbbtgat);
        let assign24960_e28426: f64 = (assign24960_e28424 / var_fmaxr);
        let assign24960_e28428: f64 = if assign24960_e28426 < 0.0 { 1.0 } else { 0.0 };
        var_guard468 = assign24960_e28428;

        let (assign24970_e28479, assign24970_e28479_d_n5, assign24970_e28479_d_n6, assign24970_e28479_d_n7, assign24970_e28479_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard465 == 0.0)) && (var_guard467 == 0.0)) && (var_guard468 != 0.0)) {
        let assign24970_e28446: f64 = (-230.25850929940458);
        let assign24970_e28448: f64 = (-var_fbbtgat);
        let assign24970_e28450: f64 = (assign24970_e28448 / var_fmaxr);
        let assign24970_e28451: f64 = (assign24970_e28446 - assign24970_e28450);
        let assign24970_e28455: f64 = (-230.25850929940458);
        let assign24970_e28457: f64 = (-var_fbbtgat);
        let assign24970_e28459: f64 = (assign24970_e28457 / var_fmaxr);
        let assign24970_e28460: f64 = (assign24970_e28455 - assign24970_e28459);
        let assign24970_e28463: f64 = (-230.25850929940458);
        let assign24970_e28465: f64 = (-var_fbbtgat);
        let assign24970_e28467: f64 = (assign24970_e28465 / var_fmaxr);
        let assign24970_e28468: f64 = (assign24970_e28463 - assign24970_e28467);
        let assign24970_e28470: f64 = (assign24970_e28468 * 0.3333333333333333);
        let assign24970_e28471: f64 = (1.0 + assign24970_e28470);
        let assign24970_e28472: f64 = (assign24970_e28460 * assign24970_e28471);
        let assign24970_e28473: f64 = (0.5 * assign24970_e28472);
        let assign24970_e28474: f64 = (1.0 + assign24970_e28473);
        let assign24970_e28475: f64 = (assign24970_e28451 * assign24970_e28474);
        let assign24970_e28476: f64 = (1.0 + assign24970_e28475);
        let assign24970_e28477: f64 = (1e-100 / assign24970_e28476);
        (assign24970_e28477, (-((1e-100 * (((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign24970_e28448 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign24970_e28474) + (assign24970_e28451 * (0.5 * (((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign24970_e28457 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * assign24970_e28471) + (assign24970_e28460 * ((-((((-var_fbbtgat_dn5) * var_fmaxr) - (assign24970_e28465 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign24970_e28476 * assign24970_e28476))), (-((1e-100 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign24970_e28448 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign24970_e28474) + (assign24970_e28451 * (0.5 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign24970_e28457 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign24970_e28471) + (assign24970_e28460 * ((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign24970_e28465 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign24970_e28476 * assign24970_e28476))), (-((1e-100 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign24970_e28448 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign24970_e28474) + (assign24970_e28451 * (0.5 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign24970_e28457 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign24970_e28471) + (assign24970_e28460 * ((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign24970_e28465 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign24970_e28476 * assign24970_e28476))), (-((1e-100 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign24970_e28448 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign24970_e28474) + (assign24970_e28451 * (0.5 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign24970_e28457 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign24970_e28471) + (assign24970_e28460 * ((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign24970_e28465 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign24970_e28476 * assign24970_e28476))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24970_e28479;
        var_tmp_dn5 = assign24970_e28479_d_n5;
        var_tmp_dn6 = assign24970_e28479_d_n6;
        var_tmp_dn7 = assign24970_e28479_d_n7;
        var_tmp_dn8 = assign24970_e28479_d_n8;

        let (assign24980_e28528, assign24980_e28528_d_n5, assign24980_e28528_d_n6, assign24980_e28528_d_n7, assign24980_e28528_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard465 == 0.0)) && (var_guard467 == 0.0)) && (var_guard468 == 0.0)) {
        let assign24980_e28498: f64 = (-var_fbbtgat);
        let assign24980_e28500: f64 = (assign24980_e28498 / var_fmaxr);
        let assign24980_e28502: f64 = (assign24980_e28500 - 230.25850929940458);
        let assign24980_e28506: f64 = (-var_fbbtgat);
        let assign24980_e28508: f64 = (assign24980_e28506 / var_fmaxr);
        let assign24980_e28510: f64 = (assign24980_e28508 - 230.25850929940458);
        let assign24980_e28513: f64 = (-var_fbbtgat);
        let assign24980_e28515: f64 = (assign24980_e28513 / var_fmaxr);
        let assign24980_e28517: f64 = (assign24980_e28515 - 230.25850929940458);
        let assign24980_e28519: f64 = (assign24980_e28517 * 0.3333333333333333);
        let assign24980_e28520: f64 = (1.0 + assign24980_e28519);
        let assign24980_e28521: f64 = (assign24980_e28510 * assign24980_e28520);
        let assign24980_e28522: f64 = (0.5 * assign24980_e28521);
        let assign24980_e28523: f64 = (1.0 + assign24980_e28522);
        let assign24980_e28524: f64 = (assign24980_e28502 * assign24980_e28523);
        let assign24980_e28525: f64 = (1.0 + assign24980_e28524);
        let assign24980_e28526: f64 = (1e100 * assign24980_e28525);
        (assign24980_e28526, (1e100 * ((((((-var_fbbtgat_dn5) * var_fmaxr) - (assign24980_e28498 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign24980_e28523) + (assign24980_e28502 * (0.5 * ((((((-var_fbbtgat_dn5) * var_fmaxr) - (assign24980_e28506 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * assign24980_e28520) + (assign24980_e28510 * (((((-var_fbbtgat_dn5) * var_fmaxr) - (assign24980_e28513 * var_fmaxr_dn5)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign24980_e28498 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign24980_e28523) + (assign24980_e28502 * (0.5 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign24980_e28506 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign24980_e28520) + (assign24980_e28510 * (((((-var_fbbtgat_dn6) * var_fmaxr) - (assign24980_e28513 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign24980_e28498 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign24980_e28523) + (assign24980_e28502 * (0.5 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign24980_e28506 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign24980_e28520) + (assign24980_e28510 * (((((-var_fbbtgat_dn7) * var_fmaxr) - (assign24980_e28513 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign24980_e28498 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign24980_e28523) + (assign24980_e28502 * (0.5 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign24980_e28506 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign24980_e28520) + (assign24980_e28510 * (((((-var_fbbtgat_dn8) * var_fmaxr) - (assign24980_e28513 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign24980_e28528;
        var_tmp_dn5 = assign24980_e28528_d_n5;
        var_tmp_dn6 = assign24980_e28528_d_n6;
        var_tmp_dn7 = assign24980_e28528_d_n7;
        var_tmp_dn8 = assign24980_e28528_d_n8;

        let (assign24990_e28548, assign24990_e28548_d_n5, assign24990_e28548_d_n6, assign24990_e28548_d_n7, assign24990_e28548_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard465 == 0.0)) {
        let assign24990_e28541: f64 = (var_v4 * var_fmaxr);
        let assign24990_e28543: f64 = (assign24990_e28541 * var_fmaxr);
        let assign24990_e28545: f64 = (assign24990_e28543 * var_tmp);
        let assign24990_e28546: f64 = (p.p853 * assign24990_e28545);
        (assign24990_e28546, (p.p853 * (((((var_v4 * var_fmaxr_dn5) * var_fmaxr) + (assign24990_e28541 * var_fmaxr_dn5)) * var_tmp) + (assign24990_e28543 * var_tmp_dn5))), (p.p853 * (((((var_v4 * var_fmaxr_dn6) * var_fmaxr) + (assign24990_e28541 * var_fmaxr_dn6)) * var_tmp) + (assign24990_e28543 * var_tmp_dn6))), (p.p853 * (((((var_v4 * var_fmaxr_dn7) * var_fmaxr) + (assign24990_e28541 * var_fmaxr_dn7)) * var_tmp) + (assign24990_e28543 * var_tmp_dn7))), (p.p853 * (((((var_v4 * var_fmaxr_dn8) * var_fmaxr) + (assign24990_e28541 * var_fmaxr_dn8)) * var_tmp) + (assign24990_e28543 * var_tmp_dn8))),)
    } else {
        (var_ibbt, var_ibbt_dn5, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8,)
    }
};
        var_ibbt = assign24990_e28548;
        var_ibbt_dn5 = assign24990_e28548_d_n5;
        var_ibbt_dn6 = assign24990_e28548_d_n6;
        var_ibbt_dn7 = assign24990_e28548_d_n7;
        var_ibbt_dn8 = assign24990_e28548_d_n8;

        let assign25000_e28551: f64 = if p.p862 > 1000.0 { 1.0 } else { 0.0 };
        var_guard469 = assign25000_e28551;

        let (assign25010_e28562, assign25010_e28562_d_n5, assign25010_e28562_d_n6, assign25010_e28562_d_n7, assign25010_e28562_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard469 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign25010_e28562;
        var_fbreakdown_dn5 = assign25010_e28562_d_n5;
        var_fbreakdown_dn6 = assign25010_e28562_d_n6;
        var_fbreakdown_dn7 = assign25010_e28562_d_n7;
        var_fbreakdown_dn8 = assign25010_e28562_d_n8;

        let assign25020_e28565: f64 = (-var_alphaav);
        let assign25020_e28567: f64 = (assign25020_e28565 * p.p862);
        let assign25020_e28568: f64 = if var_vav > assign25020_e28567 { 1.0 } else { 0.0 };
        var_guard470 = assign25020_e28568;

        let assign25030_e28571: f64 = if p.p865 == 4.0 { 1.0 } else { 0.0 };
        var_guard471 = assign25030_e28571;

        let (assign25040_e28601, assign25040_e28601_d_n5, assign25040_e28601_d_n6, assign25040_e28601_d_n7, assign25040_e28601_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard469 == 0.0)) && (var_guard470 != 0.0)) && (var_guard471 != 0.0)) {
        let assign25040_e28587: f64 = (var_vav * var_vbrinvgat);
        let assign25040_e28590: f64 = (var_vav * var_vbrinvgat);
        let assign25040_e28591: f64 = (assign25040_e28587 * assign25040_e28590);
        let assign25040_e28594: f64 = (var_vav * var_vbrinvgat);
        let assign25040_e28595: f64 = (assign25040_e28591 * assign25040_e28594);
        let assign25040_e28598: f64 = (var_vav * var_vbrinvgat);
        let assign25040_e28599: f64 = (assign25040_e28595 * assign25040_e28598);
        (assign25040_e28599, (((((((var_vav * var_vbrinvgat_dn5) * assign25040_e28590) + (assign25040_e28587 * (var_vav * var_vbrinvgat_dn5))) * assign25040_e28594) + (assign25040_e28591 * (var_vav * var_vbrinvgat_dn5))) * assign25040_e28598) + (assign25040_e28595 * (var_vav * var_vbrinvgat_dn5))), (((((((var_vav * var_vbrinvgat_dn6) * assign25040_e28590) + (assign25040_e28587 * (var_vav * var_vbrinvgat_dn6))) * assign25040_e28594) + (assign25040_e28591 * (var_vav * var_vbrinvgat_dn6))) * assign25040_e28598) + (assign25040_e28595 * (var_vav * var_vbrinvgat_dn6))), (((((((var_vav * var_vbrinvgat_dn7) * assign25040_e28590) + (assign25040_e28587 * (var_vav * var_vbrinvgat_dn7))) * assign25040_e28594) + (assign25040_e28591 * (var_vav * var_vbrinvgat_dn7))) * assign25040_e28598) + (assign25040_e28595 * (var_vav * var_vbrinvgat_dn7))), (((((((var_vav * var_vbrinvgat_dn8) * assign25040_e28590) + (assign25040_e28587 * (var_vav * var_vbrinvgat_dn8))) * assign25040_e28594) + (assign25040_e28591 * (var_vav * var_vbrinvgat_dn8))) * assign25040_e28598) + (assign25040_e28595 * (var_vav * var_vbrinvgat_dn8))),)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25040_e28601;
        var_tmp_dn5 = assign25040_e28601_d_n5;
        var_tmp_dn6 = assign25040_e28601_d_n6;
        var_tmp_dn7 = assign25040_e28601_d_n7;
        var_tmp_dn8 = assign25040_e28601_d_n8;

        let (assign25050_e28623, assign25050_e28623_d_n5, assign25050_e28623_d_n6, assign25050_e28623_d_n7, assign25050_e28623_d_n8,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard469 == 0.0)) && (var_guard470 != 0.0)) && (var_guard471 == 0.0)) {
        let assign25050_e28618: f64 = (var_vav * var_vbrinvgat);
        let assign25050_e28619: f64 = (assign25050_e28618).abs();
        let assign25050_e28621: f64 = (assign25050_e28619).powf(p.p865);
        (assign25050_e28621, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign25050_e28619).powf(p.p865 - 1.0) * if assign25050_e28618 >= 0.0 { (var_vav * var_vbrinvgat_dn5) } else { (-(var_vav * var_vbrinvgat_dn5)) })) } } else { (assign25050_e28621 * (p.p865 * (if assign25050_e28618 >= 0.0 { (var_vav * var_vbrinvgat_dn5) } else { (-(var_vav * var_vbrinvgat_dn5)) } / assign25050_e28619))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign25050_e28619).powf(p.p865 - 1.0) * if assign25050_e28618 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) })) } } else { (assign25050_e28621 * (p.p865 * (if assign25050_e28618 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) } / assign25050_e28619))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign25050_e28619).powf(p.p865 - 1.0) * if assign25050_e28618 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) })) } } else { (assign25050_e28621 * (p.p865 * (if assign25050_e28618 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) } / assign25050_e28619))) }, if 0.0 == 0.0 && ((p.p865) as f64).is_finite() && ((p.p865) as f64).fract() == 0.0 { if p.p865 == 0.0 { 0.0 } else { (p.p865 * ((assign25050_e28619).powf(p.p865 - 1.0) * if assign25050_e28618 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) })) } } else { (assign25050_e28621 * (p.p865 * (if assign25050_e28618 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) } / assign25050_e28619))) },)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25050_e28623;
        var_tmp_dn5 = assign25050_e28623_d_n5;
        var_tmp_dn6 = assign25050_e28623_d_n6;
        var_tmp_dn7 = assign25050_e28623_d_n7;
        var_tmp_dn8 = assign25050_e28623_d_n8;

        let (assign25060_e28641, assign25060_e28641_d_n5, assign25060_e28641_d_n6, assign25060_e28641_d_n7, assign25060_e28641_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard469 == 0.0)) && (var_guard470 != 0.0)) {
        let assign25060_e28638: f64 = (1.0 - var_tmp);
        let assign25060_e28639: f64 = (1.0 / assign25060_e28638);
        (assign25060_e28639, (-((-var_tmp_dn5) / (assign25060_e28638 * assign25060_e28638))), (-((-var_tmp_dn6) / (assign25060_e28638 * assign25060_e28638))), (-((-var_tmp_dn7) / (assign25060_e28638 * assign25060_e28638))), (-((-var_tmp_dn8) / (assign25060_e28638 * assign25060_e28638))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign25060_e28641;
        var_fbreakdown_dn5 = assign25060_e28641_d_n5;
        var_fbreakdown_dn6 = assign25060_e28641_d_n6;
        var_fbreakdown_dn7 = assign25060_e28641_d_n7;
        var_fbreakdown_dn8 = assign25060_e28641_d_n8;

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
        *var_guard461_slot = var_guard461;
        *var_guard462_slot = var_guard462;
        *var_guard463_slot = var_guard463;
        *var_guard464_slot = var_guard464;
        *var_guard465_slot = var_guard465;
        *var_guard466_slot = var_guard466;
        *var_guard467_slot = var_guard467;
        *var_guard468_slot = var_guard468;
        *var_guard469_slot = var_guard469;
        *var_guard470_slot = var_guard470;
        *var_guard471_slot = var_guard471;
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

    pub(super) fn stamp_transient_block_47(
        p: &Parameters,
        var_absource_i: f64,
        var_alphaav: f64,
        var_exp_vmax_over_phitd_s: f64,
        var_fstopgat: f64,
        var_ftdbot: f64,
        var_guard182: f64,
        var_guard199: f64,
        var_guard455: f64,
        var_guard469: f64,
        var_guard470: f64,
        var_ibbt: f64,
        var_ibbt_dn5: f64,
        var_ibbt_dn6: f64,
        var_ibbt_dn7: f64,
        var_ibbt_dn8: f64,
        var_idsatbot: f64,
        var_ijunsti: f64,
        var_ijunsti_dn5: f64,
        var_ijunsti_dn6: f64,
        var_ijunsti_dn7: f64,
        var_ijunsti_dn8: f64,
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
        var_dwsrh_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn5_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_guard472_slot: &mut f64,
        var_guard473_slot: &mut f64,
        var_guard474_slot: &mut f64,
        var_guard475_slot: &mut f64,
        var_guard476_slot: &mut f64,
        var_guard477_slot: &mut f64,
        var_guard478_slot: &mut f64,
        var_guard479_slot: &mut f64,
        var_guard480_slot: &mut f64,
        var_i4_slot: &mut f64,
        var_i4_dn5_slot: &mut f64,
        var_i4_dn6_slot: &mut f64,
        var_i4_dn7_slot: &mut f64,
        var_i4_dn8_slot: &mut f64,
        var_id__blk219_slot: &mut f64,
        var_idmult_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn5_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn5_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn5_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn5_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
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
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn5: f64 = *var_fbreakdown_dn5_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_guard472: f64 = *var_guard472_slot;
        let mut var_guard473: f64 = *var_guard473_slot;
        let mut var_guard474: f64 = *var_guard474_slot;
        let mut var_guard475: f64 = *var_guard475_slot;
        let mut var_guard476: f64 = *var_guard476_slot;
        let mut var_guard477: f64 = *var_guard477_slot;
        let mut var_guard478: f64 = *var_guard478_slot;
        let mut var_guard479: f64 = *var_guard479_slot;
        let mut var_guard480: f64 = *var_guard480_slot;
        let mut var_i4: f64 = *var_i4_slot;
        let mut var_i4_dn5: f64 = *var_i4_dn5_slot;
        let mut var_i4_dn6: f64 = *var_i4_dn6_slot;
        let mut var_i4_dn7: f64 = *var_i4_dn7_slot;
        let mut var_i4_dn8: f64 = *var_i4_dn8_slot;
        let mut var_id__blk219: f64 = *var_id__blk219_slot;
        let mut var_idmult: f64 = *var_idmult_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn5: f64 = *var_ijunbot_dn5_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn5: f64 = *var_ijungat_dn5_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn5: f64 = *var_isrh_dn5_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn5: f64 = *var_tmp_dn5_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
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
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_z: f64 = *var_z_slot;
        let mut var_zinv: f64 = *var_zinv_slot;

        let (assign25070_e28664, assign25070_e28664_d_n5, assign25070_e28664_d_n6, assign25070_e28664_d_n7, assign25070_e28664_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) && (var_guard469 == 0.0)) && (var_guard470 == 0.0)) {
        let assign25070_e28658: f64 = (var_alphaav * p.p862);
        let assign25070_e28659: f64 = (var_vav + assign25070_e28658);
        let assign25070_e28661: f64 = (assign25070_e28659 * var_slopegat);
        let assign25070_e28662: f64 = (var_fstopgat + assign25070_e28661);
        (assign25070_e28662, (assign25070_e28659 * var_slopegat_dn5), (assign25070_e28659 * var_slopegat_dn6), (assign25070_e28659 * var_slopegat_dn7), (assign25070_e28659 * var_slopegat_dn8),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn5, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8,)
    }
};
        var_fbreakdown = assign25070_e28664;
        var_fbreakdown_dn5 = assign25070_e28664_d_n5;
        var_fbreakdown_dn6 = assign25070_e28664_d_n6;
        var_fbreakdown_dn7 = assign25070_e28664_d_n7;
        var_fbreakdown_dn8 = assign25070_e28664_d_n8;

        let (assign25080_e28683, assign25080_e28683_d_n5, assign25080_e28683_d_n6, assign25080_e28683_d_n7, assign25080_e28683_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard455 == 0.0)) {
        let assign25080_e28674: f64 = (var_id__blk219 + var_isrh);
        let assign25080_e28676: f64 = (assign25080_e28674 + var_itat);
        let assign25080_e28678: f64 = (assign25080_e28676 + var_ibbt);
        let assign25080_e28679: f64 = (p.p29 * assign25080_e28678);
        let assign25080_e28681: f64 = (assign25080_e28679 * var_fbreakdown);
        (assign25080_e28681, (((p.p29 * ((var_isrh_dn5 + var_itat_dn5) + var_ibbt_dn5)) * var_fbreakdown) + (assign25080_e28679 * var_fbreakdown_dn5)), (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign25080_e28679 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign25080_e28679 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign25080_e28679 * var_fbreakdown_dn8)),)
    } else {
        (var_ijungat, var_ijungat_dn5, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8,)
    }
};
        var_ijungat = assign25080_e28683;
        var_ijungat_dn5 = assign25080_e28683_d_n5;
        var_ijungat_dn6 = assign25080_e28683_d_n6;
        var_ijungat_dn7 = assign25080_e28683_d_n7;
        var_ijungat_dn8 = assign25080_e28683_d_n8;

        let (assign25090_e28699, assign25090_e28699_d_n5, assign25090_e28699_d_n6, assign25090_e28699_d_n7, assign25090_e28699_d_n8,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        let assign25090_e28689: f64 = (var_absource_i * var_ijunbot);
        let assign25090_e28692: f64 = (var_lssource_i * var_ijunsti);
        let assign25090_e28693: f64 = (assign25090_e28689 + assign25090_e28692);
        let assign25090_e28696: f64 = (var_lgsource_i * var_ijungat);
        let assign25090_e28697: f64 = (assign25090_e28693 + assign25090_e28696);
        (assign25090_e28697, (((var_absource_i * var_ijunbot_dn5) + (var_lssource_i * var_ijunsti_dn5)) + (var_lgsource_i * var_ijungat_dn5)), (((var_absource_i * var_ijunbot_dn6) + (var_lssource_i * var_ijunsti_dn6)) + (var_lgsource_i * var_ijungat_dn6)), (((var_absource_i * var_ijunbot_dn7) + (var_lssource_i * var_ijunsti_dn7)) + (var_lgsource_i * var_ijungat_dn7)), (((var_absource_i * var_ijunbot_dn8) + (var_lssource_i * var_ijunsti_dn8)) + (var_lgsource_i * var_ijungat_dn8)),)
    } else {
        (var_i4, var_i4_dn5, var_i4_dn6, var_i4_dn7, var_i4_dn8,)
    }
};
        var_i4 = assign25090_e28699;
        var_i4_dn5 = assign25090_e28699_d_n5;
        var_i4_dn6 = assign25090_e28699_d_n6;
        var_i4_dn7 = assign25090_e28699_d_n7;
        var_i4_dn8 = assign25090_e28699_d_n8;

        let (assign25100_e28705,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign25100_e28705;

        let (assign25110_e28711,) = {
    if ((var_guard182 != 0.0) && (var_guard199 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign25110_e28711;

        let assign25120_e28723: f64 = if (!(((var_absource_i == 0.0) && (var_lssource_i == 0.0)) && (var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard472 = assign25120_e28723;

        let assign25200_e28809: f64 = if var_v5 < var_vmax_s { 1.0 } else { 0.0 };
        var_guard473 = assign25200_e28809;

        let assign25210_e28811: f64 = (-0.5);
        let assign25210_e28814: f64 = (var_v5 * var_phitdinv);
        let assign25210_e28815: f64 = (assign25210_e28811 * assign25210_e28814);
        let assign25210_e28816: f64 = (assign25210_e28815).abs();
        let assign25210_e28818: f64 = if assign25210_e28816 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard474 = assign25210_e28818;

        let (assign25220_e28836,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard472 != 0.0)) && (var_guard473 != 0.0)) && (var_guard474 != 0.0)) {
        let assign25220_e28829: f64 = (-0.5);
        let assign25220_e28832: f64 = (var_v5 * var_phitdinv);
        let assign25220_e28833: f64 = (assign25220_e28829 * assign25220_e28832);
        let assign25220_e28834: f64 = (assign25220_e28833).exp();
        (assign25220_e28834,)
    } else {
        (var_z,)
    }
};
        var_z = assign25220_e28836;

        let assign25230_e28838: f64 = (-0.5);
        let assign25230_e28841: f64 = (var_v5 * var_phitdinv);
        let assign25230_e28842: f64 = (assign25230_e28838 * assign25230_e28841);
        let assign25230_e28844: f64 = if assign25230_e28842 < 0.0 { 1.0 } else { 0.0 };
        var_guard475 = assign25230_e28844;

        let (assign25240_e28899,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard472 != 0.0)) && (var_guard473 != 0.0)) && (var_guard474 == 0.0)) && (var_guard475 != 0.0)) {
        let assign25240_e28860: f64 = (-230.25850929940458);
        let assign25240_e28862: f64 = (-0.5);
        let assign25240_e28865: f64 = (var_v5 * var_phitdinv);
        let assign25240_e28866: f64 = (assign25240_e28862 * assign25240_e28865);
        let assign25240_e28867: f64 = (assign25240_e28860 - assign25240_e28866);
        let assign25240_e28871: f64 = (-230.25850929940458);
        let assign25240_e28873: f64 = (-0.5);
        let assign25240_e28876: f64 = (var_v5 * var_phitdinv);
        let assign25240_e28877: f64 = (assign25240_e28873 * assign25240_e28876);
        let assign25240_e28878: f64 = (assign25240_e28871 - assign25240_e28877);
        let assign25240_e28881: f64 = (-230.25850929940458);
        let assign25240_e28883: f64 = (-0.5);
        let assign25240_e28886: f64 = (var_v5 * var_phitdinv);
        let assign25240_e28887: f64 = (assign25240_e28883 * assign25240_e28886);
        let assign25240_e28888: f64 = (assign25240_e28881 - assign25240_e28887);
        let assign25240_e28890: f64 = (assign25240_e28888 * 0.3333333333333333);
        let assign25240_e28891: f64 = (1.0 + assign25240_e28890);
        let assign25240_e28892: f64 = (assign25240_e28878 * assign25240_e28891);
        let assign25240_e28893: f64 = (0.5 * assign25240_e28892);
        let assign25240_e28894: f64 = (1.0 + assign25240_e28893);
        let assign25240_e28895: f64 = (assign25240_e28867 * assign25240_e28894);
        let assign25240_e28896: f64 = (1.0 + assign25240_e28895);
        let assign25240_e28897: f64 = (1e-100 / assign25240_e28896);
        (assign25240_e28897,)
    } else {
        (var_z,)
    }
};
        var_z = assign25240_e28899;

        let (assign25250_e28952,) = {
    if ((((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard472 != 0.0)) && (var_guard473 != 0.0)) && (var_guard474 == 0.0)) && (var_guard475 == 0.0)) {
        let assign25250_e28916: f64 = (-0.5);
        let assign25250_e28919: f64 = (var_v5 * var_phitdinv);
        let assign25250_e28920: f64 = (assign25250_e28916 * assign25250_e28919);
        let assign25250_e28922: f64 = (assign25250_e28920 - 230.25850929940458);
        let assign25250_e28926: f64 = (-0.5);
        let assign25250_e28929: f64 = (var_v5 * var_phitdinv);
        let assign25250_e28930: f64 = (assign25250_e28926 * assign25250_e28929);
        let assign25250_e28932: f64 = (assign25250_e28930 - 230.25850929940458);
        let assign25250_e28935: f64 = (-0.5);
        let assign25250_e28938: f64 = (var_v5 * var_phitdinv);
        let assign25250_e28939: f64 = (assign25250_e28935 * assign25250_e28938);
        let assign25250_e28941: f64 = (assign25250_e28939 - 230.25850929940458);
        let assign25250_e28943: f64 = (assign25250_e28941 * 0.3333333333333333);
        let assign25250_e28944: f64 = (1.0 + assign25250_e28943);
        let assign25250_e28945: f64 = (assign25250_e28932 * assign25250_e28944);
        let assign25250_e28946: f64 = (0.5 * assign25250_e28945);
        let assign25250_e28947: f64 = (1.0 + assign25250_e28946);
        let assign25250_e28948: f64 = (assign25250_e28922 * assign25250_e28947);
        let assign25250_e28949: f64 = (1.0 + assign25250_e28948);
        let assign25250_e28950: f64 = (1e100 * assign25250_e28949);
        (assign25250_e28950,)
    } else {
        (var_z,)
    }
};
        var_z = assign25250_e28952;

        let (assign25260_e28964,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard472 != 0.0)) && (var_guard473 != 0.0)) {
        let assign25260_e28962: f64 = (1.0 / var_z);
        (assign25260_e28962,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign25260_e28964;

        let (assign25270_e28976,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard472 != 0.0)) && (var_guard473 != 0.0)) {
        let assign25270_e28974: f64 = (var_zinv * var_zinv);
        (assign25270_e28974,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign25270_e28976;

        let (assign25280_e28995,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard472 != 0.0)) && (var_guard473 == 0.0)) {
        let assign25280_e28988: f64 = (var_v5 - var_vmax_s);
        let assign25280_e28990: f64 = (assign25280_e28988 * var_phitdinv);
        let assign25280_e28991: f64 = (1.0 + assign25280_e28990);
        let assign25280_e28993: f64 = (assign25280_e28991 * var_exp_vmax_over_phitd_s);
        (assign25280_e28993,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign25280_e28995;

        let (assign25290_e29007,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard472 != 0.0)) && (var_guard473 == 0.0)) {
        let assign25290_e29005: f64 = (var_idmult).sqrt();
        (assign25290_e29005,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign25290_e29007;

        let (assign25300_e29020,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard472 != 0.0)) && (var_guard473 == 0.0)) {
        let assign25300_e29018: f64 = (1.0 / var_zinv);
        (assign25300_e29018,)
    } else {
        (var_z,)
    }
};
        var_z = assign25300_e29020;

        let (assign25310_e29030,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard472 != 0.0)) {
        let assign25310_e29028: f64 = (var_idmult - 1.0);
        (assign25310_e29028,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign25310_e29030;

        let assign25320_e29033: f64 = if var_v5 > 0.0 { 1.0 } else { 0.0 };
        var_guard476 = assign25320_e29033;

        let (assign25330_e29059,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard472 != 0.0)) && (var_guard476 != 0.0)) {
        let assign25330_e29045: f64 = (2.0 + var_z);
        let assign25330_e29048: f64 = (var_z + 1.0);
        let assign25330_e29051: f64 = (var_z + 3.0);
        let assign25330_e29052: f64 = (assign25330_e29048 * assign25330_e29051);
        let assign25330_e29053: f64 = (assign25330_e29052).sqrt();
        let assign25330_e29054: f64 = (assign25330_e29045 + assign25330_e29053);
        let assign25330_e29055: f64 = (assign25330_e29054).ln();
        let assign25330_e29056: f64 = (var_phitd * assign25330_e29055);
        let assign25330_e29057: f64 = (2.0 * assign25330_e29056);
        (assign25330_e29057,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign25330_e29059;

        let (assign25340_e29093,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard472 != 0.0)) && (var_guard476 == 0.0)) {
        let assign25340_e29069: f64 = (-var_v5);
        let assign25340_e29074: f64 = (2.0 * var_zinv);
        let assign25340_e29076: f64 = (assign25340_e29074 + 1.0);
        let assign25340_e29079: f64 = (1.0 + var_zinv);
        let assign25340_e29083: f64 = (3.0 * var_zinv);
        let assign25340_e29084: f64 = (1.0 + assign25340_e29083);
        let assign25340_e29085: f64 = (assign25340_e29079 * assign25340_e29084);
        let assign25340_e29086: f64 = (assign25340_e29085).sqrt();
        let assign25340_e29087: f64 = (assign25340_e29076 + assign25340_e29086);
        let assign25340_e29088: f64 = (assign25340_e29087).ln();
        let assign25340_e29089: f64 = (var_phitd * assign25340_e29088);
        let assign25340_e29090: f64 = (2.0 * assign25340_e29089);
        let assign25340_e29091: f64 = (assign25340_e29069 + assign25340_e29090);
        (assign25340_e29091,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign25340_e29093;

        let (assign25350_e29103,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard472 != 0.0)) {
        let assign25350_e29101: f64 = (var_vbimin_s - var_two_psistar);
        (assign25350_e29101,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign25350_e29103;

        let (assign25360_e29130,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard472 != 0.0)) {
        let assign25360_e29112: f64 = (var_v5 + var_vjlim);
        let assign25360_e29115: f64 = (var_v5 - var_vjlim);
        let assign25360_e29118: f64 = (var_v5 - var_vjlim);
        let assign25360_e29119: f64 = (assign25360_e29115 * assign25360_e29118);
        let assign25360_e29122: f64 = (4.0 * var_phitd);
        let assign25360_e29124: f64 = (assign25360_e29122 * var_phitd);
        let assign25360_e29125: f64 = (assign25360_e29119 + assign25360_e29124);
        let assign25360_e29126: f64 = (assign25360_e29125).sqrt();
        let assign25360_e29127: f64 = (assign25360_e29112 - assign25360_e29126);
        let assign25360_e29128: f64 = (0.5 * assign25360_e29127);
        (assign25360_e29128,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign25360_e29130;

        let (assign25370_e29157,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard472 != 0.0)) {
        let assign25370_e29139: f64 = (var_v5 + var_vbbtlim_s);
        let assign25370_e29142: f64 = (var_v5 - var_vbbtlim_s);
        let assign25370_e29145: f64 = (var_v5 - var_vbbtlim_s);
        let assign25370_e29146: f64 = (assign25370_e29142 * assign25370_e29145);
        let assign25370_e29149: f64 = (4.0 * var_phitr);
        let assign25370_e29151: f64 = (assign25370_e29149 * var_phitr);
        let assign25370_e29152: f64 = (assign25370_e29146 + assign25370_e29151);
        let assign25370_e29153: f64 = (assign25370_e29152).sqrt();
        let assign25370_e29154: f64 = (assign25370_e29139 - assign25370_e29153);
        let assign25370_e29155: f64 = (0.5 * assign25370_e29154);
        (assign25370_e29155,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign25370_e29157;

        let (assign25380_e29184,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard472 != 0.0)) {
        let assign25380_e29166: f64 = var_v5;
        let assign25380_e29169: f64 = var_v5;
        let assign25380_e29172: f64 = var_v5;
        let assign25380_e29173: f64 = (assign25380_e29169 * assign25380_e29172);
        let assign25380_e29176: f64 = (4.0 * 1e-6);
        let assign25380_e29178: f64 = (assign25380_e29176 * 1e-6);
        let assign25380_e29179: f64 = (assign25380_e29173 + assign25380_e29178);
        let assign25380_e29180: f64 = (assign25380_e29179).sqrt();
        let assign25380_e29181: f64 = (assign25380_e29166 - assign25380_e29180);
        let assign25380_e29182: f64 = (0.5 * assign25380_e29181);
        (assign25380_e29182,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign25380_e29184;

        let assign25390_e29187: f64 = if var_absource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard477 = assign25390_e29187;

        let (assign25400_e29195, assign25400_e29195_d_n5, assign25400_e29195_d_n6, assign25400_e29195_d_n7, assign25400_e29195_d_n8,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn5, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8,)
    }
};
        var_ijunbot = assign25400_e29195;
        var_ijunbot_dn5 = assign25400_e29195_d_n5;
        var_ijunbot_dn6 = assign25400_e29195_d_n6;
        var_ijunbot_dn7 = assign25400_e29195_d_n7;
        var_ijunbot_dn8 = assign25400_e29195_d_n8;

        let (assign25410_e29206,) = {
    if (((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) {
        let assign25410_e29204: f64 = (var_idsatbot * var_idmult);
        (assign25410_e29204,)
    } else {
        (var_id__blk219,)
    }
};
        var_id__blk219 = assign25410_e29206;

        let assign25420_e29213: f64 = if ((p.p840 == 0.0) && (p.p845 == 0.0)) { 1.0 } else { 0.0 };
        var_guard478 = assign25420_e29213;

        let (assign25430_e29224, assign25430_e29224_d_n5, assign25430_e29224_d_n6, assign25430_e29224_d_n7, assign25430_e29224_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard478 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn5, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8,)
    }
};
        var_isrh = assign25430_e29224;
        var_isrh_dn5 = assign25430_e29224_d_n5;
        var_isrh_dn6 = assign25430_e29224_d_n6;
        var_isrh_dn7 = assign25430_e29224_d_n7;
        var_isrh_dn8 = assign25430_e29224_d_n8;

        let (assign25440_e29238,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard478 == 0.0)) {
        let assign25440_e29236: f64 = (var_vbibot - var_vjsrh);
        (assign25440_e29236,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign25440_e29238;

        let (assign25450_e29257,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard478 == 0.0)) {
        let assign25450_e29252: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign25450_e29253: f64 = (1.0 - assign25450_e29252);
        let assign25450_e29254: f64 = (assign25450_e29253).sqrt();
        let assign25450_e29255: f64 = (1.0 - assign25450_e29254);
        (assign25450_e29255,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign25450_e29257;

        let assign25460_e29260: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        var_guard479 = assign25460_e29260;

        let (assign25470_e29274,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard478 == 0.0)) && (var_guard479 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign25470_e29274;

        let (assign25480_e29306,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard478 == 0.0)) && (var_guard479 == 0.0)) {
        let assign25480_e29289: f64 = (var_wsrhstep * var_wsrhstep);
        let assign25480_e29291: f64 = (var_wsrhstep).ln();
        let assign25480_e29292: f64 = (assign25480_e29289 * assign25480_e29291);
        let assign25480_e29295: f64 = (1.0 - var_wsrhstep);
        let assign25480_e29296: f64 = (assign25480_e29292 / assign25480_e29295);
        let assign25480_e29298: f64 = (assign25480_e29296 + var_wsrhstep);
        let assign25480_e29302: f64 = (2.0 * p.p831);
        let assign25480_e29303: f64 = (1.0 - assign25480_e29302);
        let assign25480_e29304: f64 = (assign25480_e29298 * assign25480_e29303);
        (assign25480_e29304,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign25480_e29306;

        let (assign25490_e29320,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard478 == 0.0)) {
        let assign25490_e29318: f64 = (var_wsrhstep + var_dwsrh);
        (assign25490_e29318,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign25490_e29320;

        let assign25500_e29323: f64 = if p.p831 == 0.5 { 1.0 } else { 0.0 };
        var_guard480 = assign25500_e29323;

        let (assign25510_e29340, assign25510_e29340_d_n5, assign25510_e29340_d_n6, assign25510_e29340_d_n7, assign25510_e29340_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard478 == 0.0)) && (var_guard480 != 0.0)) {
        let assign25510_e29337: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign25510_e29338: f64 = (assign25510_e29337).sqrt();
        (assign25510_e29338, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25510_e29340;
        var_tmp_dn5 = assign25510_e29340_d_n5;
        var_tmp_dn6 = assign25510_e29340_d_n6;
        var_tmp_dn7 = assign25510_e29340_d_n7;
        var_tmp_dn8 = assign25510_e29340_d_n8;

        let (assign25520_e29359, assign25520_e29359_d_n5, assign25520_e29359_d_n6, assign25520_e29359_d_n7, assign25520_e29359_d_n8,) = {
    if (((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard478 == 0.0)) && (var_guard480 == 0.0)) {
        let assign25520_e29355: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign25520_e29357: f64 = (assign25520_e29355).powf(p.p831);
        (assign25520_e29357, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn5, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8,)
    }
};
        var_tmp = assign25520_e29359;
        var_tmp_dn5 = assign25520_e29359_d_n5;
        var_tmp_dn6 = assign25520_e29359_d_n6;
        var_tmp_dn7 = assign25520_e29359_d_n7;
        var_tmp_dn8 = assign25520_e29359_d_n8;

        let (assign25530_e29373, assign25530_e29373_d_n5, assign25530_e29373_d_n6, assign25530_e29373_d_n7, assign25530_e29373_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard478 == 0.0)) {
        let assign25530_e29371: f64 = (var_wdepnulrbot * var_tmp);
        (assign25530_e29371, (var_wdepnulrbot * var_tmp_dn5), (var_wdepnulrbot * var_tmp_dn6), (var_wdepnulrbot * var_tmp_dn7), (var_wdepnulrbot * var_tmp_dn8),)
    } else {
        (var_wdep, var_wdep_dn5, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8,)
    }
};
        var_wdep = assign25530_e29373;
        var_wdep_dn5 = assign25530_e29373_d_n5;
        var_wdep_dn6 = assign25530_e29373_d_n6;
        var_wdep_dn7 = assign25530_e29373_d_n7;
        var_wdep_dn8 = assign25530_e29373_d_n8;

        let (assign25540_e29391, assign25540_e29391_d_n5, assign25540_e29391_d_n6, assign25540_e29391_d_n7, assign25540_e29391_d_n8,) = {
    if ((((var_guard182 != 0.0) && (var_guard199 != 0.0)) && (var_guard477 == 0.0)) && (var_guard478 == 0.0)) {
        let assign25540_e29386: f64 = (var_zinv - 1.0);
        let assign25540_e29388: f64 = (assign25540_e29386 * var_wdep);
        let assign25540_e29389: f64 = (var_ftdbot * assign25540_e29388);
        (assign25540_e29389, (var_ftdbot * (assign25540_e29386 * var_wdep_dn5)), (var_ftdbot * (assign25540_e29386 * var_wdep_dn6)), (var_ftdbot * (assign25540_e29386 * var_wdep_dn7)), (var_ftdbot * (assign25540_e29386 * var_wdep_dn8)),)
    } else {
        (var_asrh, var_asrh_dn5, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8,)
    }
};
        var_asrh = assign25540_e29391;
        var_asrh_dn5 = assign25540_e29391_d_n5;
        var_asrh_dn6 = assign25540_e29391_d_n6;
        var_asrh_dn7 = assign25540_e29391_d_n7;
        var_asrh_dn8 = assign25540_e29391_d_n8;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn5_slot = var_asrh_dn5;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_dwsrh_slot = var_dwsrh;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn5_slot = var_fbreakdown_dn5;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_guard472_slot = var_guard472;
        *var_guard473_slot = var_guard473;
        *var_guard474_slot = var_guard474;
        *var_guard475_slot = var_guard475;
        *var_guard476_slot = var_guard476;
        *var_guard477_slot = var_guard477;
        *var_guard478_slot = var_guard478;
        *var_guard479_slot = var_guard479;
        *var_guard480_slot = var_guard480;
        *var_i4_slot = var_i4;
        *var_i4_dn5_slot = var_i4_dn5;
        *var_i4_dn6_slot = var_i4_dn6;
        *var_i4_dn7_slot = var_i4_dn7;
        *var_i4_dn8_slot = var_i4_dn8;
        *var_id__blk219_slot = var_id__blk219;
        *var_idmult_slot = var_idmult;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn5_slot = var_ijunbot_dn5;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn5_slot = var_ijungat_dn5;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn5_slot = var_isrh_dn5;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn5_slot = var_tmp_dn5;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_two_psistar_slot = var_two_psistar;
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
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_z_slot = var_z;
        *var_zinv_slot = var_zinv;
    }
}
