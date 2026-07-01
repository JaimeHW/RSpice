#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_112(
        var_betn1_i: f64,
        var_betn1_i_dn4: f64,
        var_betn1_i_dn6: f64,
        var_betn1_i_dn7: f64,
        var_betn1_i_dn8: f64,
        var_betn1_i_dn9: f64,
        var_betn1_t: f64,
        var_betn1_t_dn4: f64,
        var_betn1_t_dn6: f64,
        var_betn1_t_dn7: f64,
        var_betn1_t_dn8: f64,
        var_betn1_t_dn9: f64,
        var_betn2_t: f64,
        var_betn2_t_dn4: f64,
        var_betn2_t_dn6: f64,
        var_betn2_t_dn7: f64,
        var_betn2_t_dn8: f64,
        var_betn2_t_dn9: f64,
        var_cs_i: f64,
        var_cs_i_dn4: f64,
        var_cs_i_dn6: f64,
        var_cs_i_dn7: f64,
        var_cs_i_dn8: f64,
        var_cs_i_dn9: f64,
        var_csbi_i: f64,
        var_csfi_i: f64,
        var_ecpl1s__blk954: f64,
        var_ecpl1s__blk954_dn4: f64,
        var_ecpl1s__blk954_dn6: f64,
        var_ecpl1s__blk954_dn7: f64,
        var_ecpl1s__blk954_dn8: f64,
        var_ecpl1s__blk954_dn9: f64,
        var_ecpl2s__blk955: f64,
        var_ecpl2s__blk955_dn4: f64,
        var_ecpl2s__blk955_dn6: f64,
        var_ecpl2s__blk955_dn7: f64,
        var_ecpl2s__blk955_dn8: f64,
        var_ecpl2s__blk955_dn9: f64,
        var_eeff1s__blk956: f64,
        var_eeff1s__blk956_dn4: f64,
        var_eeff1s__blk956_dn6: f64,
        var_eeff1s__blk956_dn7: f64,
        var_eeff1s__blk956_dn8: f64,
        var_eeff1s__blk956_dn9: f64,
        var_eeff2s__blk957: f64,
        var_eeff2s__blk957_dn4: f64,
        var_eeff2s__blk957_dn6: f64,
        var_eeff2s__blk957_dn7: f64,
        var_eeff2s__blk957_dn8: f64,
        var_eeff2s__blk957_dn9: f64,
        var_esurf1d__blk1021: f64,
        var_esurf1d__blk1021_dn4: f64,
        var_esurf1d__blk1021_dn6: f64,
        var_esurf1d__blk1021_dn7: f64,
        var_esurf1d__blk1021_dn8: f64,
        var_esurf1d__blk1021_dn9: f64,
        var_esurf1s__blk952: f64,
        var_esurf1s__blk952_dn4: f64,
        var_esurf1s__blk952_dn6: f64,
        var_esurf1s__blk952_dn7: f64,
        var_esurf1s__blk952_dn8: f64,
        var_esurf1s__blk952_dn9: f64,
        var_esurf2d__blk1022: f64,
        var_esurf2d__blk1022_dn4: f64,
        var_esurf2d__blk1022_dn6: f64,
        var_esurf2d__blk1022_dn7: f64,
        var_esurf2d__blk1022_dn8: f64,
        var_esurf2d__blk1022_dn9: f64,
        var_esurf2s__blk953: f64,
        var_esurf2s__blk953_dn4: f64,
        var_esurf2s__blk953_dn6: f64,
        var_esurf2s__blk953_dn7: f64,
        var_esurf2s__blk953_dn8: f64,
        var_esurf2s__blk953_dn9: f64,
        var_eta_mu: f64,
        var_fmue: f64,
        var_fmue_dn4: f64,
        var_fmue_dn6: f64,
        var_fmue_dn7: f64,
        var_fmue_dn8: f64,
        var_fmue_dn9: f64,
        var_frscsi__blk964: f64,
        var_frscsi__blk964_dn4: f64,
        var_frscsi__blk964_dn6: f64,
        var_frscsi__blk964_dn7: f64,
        var_frscsi__blk964_dn8: f64,
        var_frscsi__blk964_dn9: f64,
        var_guard1080: f64,
        var_inv_qi1cs: f64,
        var_inv_qi2cs: f64,
        var_k1q1d__blk1004: f64,
        var_k1q1d__blk1004_dn4: f64,
        var_k1q1d__blk1004_dn6: f64,
        var_k1q1d__blk1004_dn7: f64,
        var_k1q1d__blk1004_dn8: f64,
        var_k1q1d__blk1004_dn9: f64,
        var_k2q2d__blk1005: f64,
        var_k2q2d__blk1005_dn4: f64,
        var_k2q2d__blk1005_dn6: f64,
        var_k2q2d__blk1005_dn7: f64,
        var_k2q2d__blk1005_dn8: f64,
        var_k2q2d__blk1005_dn9: f64,
        var_lnrtn: f64,
        var_lnrtn_dn4: f64,
        var_lnrtn_dn6: f64,
        var_lnrtn_dn7: f64,
        var_lnrtn_dn8: f64,
        var_lnrtn_dn9: f64,
        var_one_m_eta: f64,
        var_qim__blk1016: f64,
        var_qim__blk1016_dn4: f64,
        var_qim__blk1016_dn6: f64,
        var_qim__blk1016_dn7: f64,
        var_qim__blk1016_dn8: f64,
        var_qim__blk1016_dn9: f64,
        var_ratio_pd__blk1020: f64,
        var_ratio_pd__blk1020_dn4: f64,
        var_ratio_pd__blk1020_dn6: f64,
        var_ratio_pd__blk1020_dn7: f64,
        var_ratio_pd__blk1020_dn8: f64,
        var_ratio_pd__blk1020_dn9: f64,
        var_rsg_i: f64,
        var_rsig_i: f64,
        var_stbet_i: f64,
        var_thecs_i: f64,
        var_thecs_i_dn4: f64,
        var_thecs_i_dn6: f64,
        var_thecs_i_dn7: f64,
        var_thecs_i_dn8: f64,
        var_thecs_i_dn9: f64,
        var_themu_i: f64,
        var_themu_i_dn4: f64,
        var_themu_i_dn6: f64,
        var_themu_i_dn7: f64,
        var_themu_i_dn8: f64,
        var_themu_i_dn9: f64,
        var_thersg_i: f64,
        var_xcor_i: f64,
        var_xcor_i_dn4: f64,
        var_xcor_i_dn6: f64,
        var_xcor_i_dn7: f64,
        var_xcor_i_dn8: f64,
        var_xcor_i_dn9: f64,
        var_xcorb_i: f64,
        var_c1__blk1035_slot: &mut f64,
        var_c1__blk1035_dn4_slot: &mut f64,
        var_c1__blk1035_dn6_slot: &mut f64,
        var_c1__blk1035_dn7_slot: &mut f64,
        var_c1__blk1035_dn8_slot: &mut f64,
        var_c1__blk1035_dn9_slot: &mut f64,
        var_c2__blk1036_slot: &mut f64,
        var_c2__blk1036_dn4_slot: &mut f64,
        var_c2__blk1036_dn6_slot: &mut f64,
        var_c2__blk1036_dn7_slot: &mut f64,
        var_c2__blk1036_dn8_slot: &mut f64,
        var_c2__blk1036_dn9_slot: &mut f64,
        var_csum__blk1037_slot: &mut f64,
        var_csum__blk1037_dn4_slot: &mut f64,
        var_csum__blk1037_dn6_slot: &mut f64,
        var_csum__blk1037_dn7_slot: &mut f64,
        var_csum__blk1037_dn8_slot: &mut f64,
        var_csum__blk1037_dn9_slot: &mut f64,
        var_ecpl1__blk1031_slot: &mut f64,
        var_ecpl1__blk1031_dn4_slot: &mut f64,
        var_ecpl1__blk1031_dn6_slot: &mut f64,
        var_ecpl1__blk1031_dn7_slot: &mut f64,
        var_ecpl1__blk1031_dn8_slot: &mut f64,
        var_ecpl1__blk1031_dn9_slot: &mut f64,
        var_ecpl1d__blk1023_slot: &mut f64,
        var_ecpl1d__blk1023_dn4_slot: &mut f64,
        var_ecpl1d__blk1023_dn6_slot: &mut f64,
        var_ecpl1d__blk1023_dn7_slot: &mut f64,
        var_ecpl1d__blk1023_dn8_slot: &mut f64,
        var_ecpl1d__blk1023_dn9_slot: &mut f64,
        var_ecpl2__blk1032_slot: &mut f64,
        var_ecpl2__blk1032_dn4_slot: &mut f64,
        var_ecpl2__blk1032_dn6_slot: &mut f64,
        var_ecpl2__blk1032_dn7_slot: &mut f64,
        var_ecpl2__blk1032_dn8_slot: &mut f64,
        var_ecpl2__blk1032_dn9_slot: &mut f64,
        var_ecpl2d__blk1024_slot: &mut f64,
        var_ecpl2d__blk1024_dn4_slot: &mut f64,
        var_ecpl2d__blk1024_dn6_slot: &mut f64,
        var_ecpl2d__blk1024_dn7_slot: &mut f64,
        var_ecpl2d__blk1024_dn8_slot: &mut f64,
        var_ecpl2d__blk1024_dn9_slot: &mut f64,
        var_eeff1__blk1033_slot: &mut f64,
        var_eeff1__blk1033_dn4_slot: &mut f64,
        var_eeff1__blk1033_dn6_slot: &mut f64,
        var_eeff1__blk1033_dn7_slot: &mut f64,
        var_eeff1__blk1033_dn8_slot: &mut f64,
        var_eeff1__blk1033_dn9_slot: &mut f64,
        var_eeff1d__blk1025_slot: &mut f64,
        var_eeff1d__blk1025_dn4_slot: &mut f64,
        var_eeff1d__blk1025_dn6_slot: &mut f64,
        var_eeff1d__blk1025_dn7_slot: &mut f64,
        var_eeff1d__blk1025_dn8_slot: &mut f64,
        var_eeff1d__blk1025_dn9_slot: &mut f64,
        var_eeff2__blk1034_slot: &mut f64,
        var_eeff2__blk1034_dn4_slot: &mut f64,
        var_eeff2__blk1034_dn6_slot: &mut f64,
        var_eeff2__blk1034_dn7_slot: &mut f64,
        var_eeff2__blk1034_dn8_slot: &mut f64,
        var_eeff2__blk1034_dn9_slot: &mut f64,
        var_eeff2d__blk1026_slot: &mut f64,
        var_eeff2d__blk1026_dn4_slot: &mut f64,
        var_eeff2d__blk1026_dn6_slot: &mut f64,
        var_eeff2d__blk1026_dn7_slot: &mut f64,
        var_eeff2d__blk1026_dn8_slot: &mut f64,
        var_eeff2d__blk1026_dn9_slot: &mut f64,
        var_esurf1__blk1027_slot: &mut f64,
        var_esurf1__blk1027_dn4_slot: &mut f64,
        var_esurf1__blk1027_dn6_slot: &mut f64,
        var_esurf1__blk1027_dn7_slot: &mut f64,
        var_esurf1__blk1027_dn8_slot: &mut f64,
        var_esurf1__blk1027_dn9_slot: &mut f64,
        var_esurf2__blk1028_slot: &mut f64,
        var_esurf2__blk1028_dn4_slot: &mut f64,
        var_esurf2__blk1028_dn6_slot: &mut f64,
        var_esurf2__blk1028_dn7_slot: &mut f64,
        var_esurf2__blk1028_dn8_slot: &mut f64,
        var_esurf2__blk1028_dn9_slot: &mut f64,
        var_fcor__blk1038_slot: &mut f64,
        var_fcor__blk1038_dn4_slot: &mut f64,
        var_fcor__blk1038_dn6_slot: &mut f64,
        var_fcor__blk1038_dn7_slot: &mut f64,
        var_fcor__blk1038_dn8_slot: &mut f64,
        var_fcor__blk1038_dn9_slot: &mut f64,
        var_gcs__blk1039_slot: &mut f64,
        var_gcs__blk1039_dn4_slot: &mut f64,
        var_gcs__blk1039_dn6_slot: &mut f64,
        var_gcs__blk1039_dn7_slot: &mut f64,
        var_gcs__blk1039_dn8_slot: &mut f64,
        var_gcs__blk1039_dn9_slot: &mut f64,
        var_gmob1__blk1041_slot: &mut f64,
        var_gmob1__blk1041_dn4_slot: &mut f64,
        var_gmob1__blk1041_dn6_slot: &mut f64,
        var_gmob1__blk1041_dn7_slot: &mut f64,
        var_gmob1__blk1041_dn8_slot: &mut f64,
        var_gmob1__blk1041_dn9_slot: &mut f64,
        var_grs__blk1040_slot: &mut f64,
        var_grs__blk1040_dn4_slot: &mut f64,
        var_grs__blk1040_dn6_slot: &mut f64,
        var_grs__blk1040_dn7_slot: &mut f64,
        var_grs__blk1040_dn8_slot: &mut f64,
        var_grs__blk1040_dn9_slot: &mut f64,
        var_guard1222_slot: &mut f64,
        var_guard1223_slot: &mut f64,
        var_qi1m__blk1029_slot: &mut f64,
        var_qi1m__blk1029_dn4_slot: &mut f64,
        var_qi1m__blk1029_dn6_slot: &mut f64,
        var_qi1m__blk1029_dn7_slot: &mut f64,
        var_qi1m__blk1029_dn8_slot: &mut f64,
        var_qi1m__blk1029_dn9_slot: &mut f64,
        var_qi2m__blk1030_slot: &mut f64,
        var_qi2m__blk1030_dn4_slot: &mut f64,
        var_qi2m__blk1030_dn6_slot: &mut f64,
        var_qi2m__blk1030_dn7_slot: &mut f64,
        var_qi2m__blk1030_dn8_slot: &mut f64,
        var_qi2m__blk1030_dn9_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp3_slot: &mut f64,
        var_temp3_dn4_slot: &mut f64,
        var_temp3_dn6_slot: &mut f64,
        var_temp3_dn7_slot: &mut f64,
        var_temp3_dn8_slot: &mut f64,
        var_temp3_dn9_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
    ) {
        let mut var_c1__blk1035: f64 = *var_c1__blk1035_slot;
        let mut var_c1__blk1035_dn4: f64 = *var_c1__blk1035_dn4_slot;
        let mut var_c1__blk1035_dn6: f64 = *var_c1__blk1035_dn6_slot;
        let mut var_c1__blk1035_dn7: f64 = *var_c1__blk1035_dn7_slot;
        let mut var_c1__blk1035_dn8: f64 = *var_c1__blk1035_dn8_slot;
        let mut var_c1__blk1035_dn9: f64 = *var_c1__blk1035_dn9_slot;
        let mut var_c2__blk1036: f64 = *var_c2__blk1036_slot;
        let mut var_c2__blk1036_dn4: f64 = *var_c2__blk1036_dn4_slot;
        let mut var_c2__blk1036_dn6: f64 = *var_c2__blk1036_dn6_slot;
        let mut var_c2__blk1036_dn7: f64 = *var_c2__blk1036_dn7_slot;
        let mut var_c2__blk1036_dn8: f64 = *var_c2__blk1036_dn8_slot;
        let mut var_c2__blk1036_dn9: f64 = *var_c2__blk1036_dn9_slot;
        let mut var_csum__blk1037: f64 = *var_csum__blk1037_slot;
        let mut var_csum__blk1037_dn4: f64 = *var_csum__blk1037_dn4_slot;
        let mut var_csum__blk1037_dn6: f64 = *var_csum__blk1037_dn6_slot;
        let mut var_csum__blk1037_dn7: f64 = *var_csum__blk1037_dn7_slot;
        let mut var_csum__blk1037_dn8: f64 = *var_csum__blk1037_dn8_slot;
        let mut var_csum__blk1037_dn9: f64 = *var_csum__blk1037_dn9_slot;
        let mut var_ecpl1__blk1031: f64 = *var_ecpl1__blk1031_slot;
        let mut var_ecpl1__blk1031_dn4: f64 = *var_ecpl1__blk1031_dn4_slot;
        let mut var_ecpl1__blk1031_dn6: f64 = *var_ecpl1__blk1031_dn6_slot;
        let mut var_ecpl1__blk1031_dn7: f64 = *var_ecpl1__blk1031_dn7_slot;
        let mut var_ecpl1__blk1031_dn8: f64 = *var_ecpl1__blk1031_dn8_slot;
        let mut var_ecpl1__blk1031_dn9: f64 = *var_ecpl1__blk1031_dn9_slot;
        let mut var_ecpl1d__blk1023: f64 = *var_ecpl1d__blk1023_slot;
        let mut var_ecpl1d__blk1023_dn4: f64 = *var_ecpl1d__blk1023_dn4_slot;
        let mut var_ecpl1d__blk1023_dn6: f64 = *var_ecpl1d__blk1023_dn6_slot;
        let mut var_ecpl1d__blk1023_dn7: f64 = *var_ecpl1d__blk1023_dn7_slot;
        let mut var_ecpl1d__blk1023_dn8: f64 = *var_ecpl1d__blk1023_dn8_slot;
        let mut var_ecpl1d__blk1023_dn9: f64 = *var_ecpl1d__blk1023_dn9_slot;
        let mut var_ecpl2__blk1032: f64 = *var_ecpl2__blk1032_slot;
        let mut var_ecpl2__blk1032_dn4: f64 = *var_ecpl2__blk1032_dn4_slot;
        let mut var_ecpl2__blk1032_dn6: f64 = *var_ecpl2__blk1032_dn6_slot;
        let mut var_ecpl2__blk1032_dn7: f64 = *var_ecpl2__blk1032_dn7_slot;
        let mut var_ecpl2__blk1032_dn8: f64 = *var_ecpl2__blk1032_dn8_slot;
        let mut var_ecpl2__blk1032_dn9: f64 = *var_ecpl2__blk1032_dn9_slot;
        let mut var_ecpl2d__blk1024: f64 = *var_ecpl2d__blk1024_slot;
        let mut var_ecpl2d__blk1024_dn4: f64 = *var_ecpl2d__blk1024_dn4_slot;
        let mut var_ecpl2d__blk1024_dn6: f64 = *var_ecpl2d__blk1024_dn6_slot;
        let mut var_ecpl2d__blk1024_dn7: f64 = *var_ecpl2d__blk1024_dn7_slot;
        let mut var_ecpl2d__blk1024_dn8: f64 = *var_ecpl2d__blk1024_dn8_slot;
        let mut var_ecpl2d__blk1024_dn9: f64 = *var_ecpl2d__blk1024_dn9_slot;
        let mut var_eeff1__blk1033: f64 = *var_eeff1__blk1033_slot;
        let mut var_eeff1__blk1033_dn4: f64 = *var_eeff1__blk1033_dn4_slot;
        let mut var_eeff1__blk1033_dn6: f64 = *var_eeff1__blk1033_dn6_slot;
        let mut var_eeff1__blk1033_dn7: f64 = *var_eeff1__blk1033_dn7_slot;
        let mut var_eeff1__blk1033_dn8: f64 = *var_eeff1__blk1033_dn8_slot;
        let mut var_eeff1__blk1033_dn9: f64 = *var_eeff1__blk1033_dn9_slot;
        let mut var_eeff1d__blk1025: f64 = *var_eeff1d__blk1025_slot;
        let mut var_eeff1d__blk1025_dn4: f64 = *var_eeff1d__blk1025_dn4_slot;
        let mut var_eeff1d__blk1025_dn6: f64 = *var_eeff1d__blk1025_dn6_slot;
        let mut var_eeff1d__blk1025_dn7: f64 = *var_eeff1d__blk1025_dn7_slot;
        let mut var_eeff1d__blk1025_dn8: f64 = *var_eeff1d__blk1025_dn8_slot;
        let mut var_eeff1d__blk1025_dn9: f64 = *var_eeff1d__blk1025_dn9_slot;
        let mut var_eeff2__blk1034: f64 = *var_eeff2__blk1034_slot;
        let mut var_eeff2__blk1034_dn4: f64 = *var_eeff2__blk1034_dn4_slot;
        let mut var_eeff2__blk1034_dn6: f64 = *var_eeff2__blk1034_dn6_slot;
        let mut var_eeff2__blk1034_dn7: f64 = *var_eeff2__blk1034_dn7_slot;
        let mut var_eeff2__blk1034_dn8: f64 = *var_eeff2__blk1034_dn8_slot;
        let mut var_eeff2__blk1034_dn9: f64 = *var_eeff2__blk1034_dn9_slot;
        let mut var_eeff2d__blk1026: f64 = *var_eeff2d__blk1026_slot;
        let mut var_eeff2d__blk1026_dn4: f64 = *var_eeff2d__blk1026_dn4_slot;
        let mut var_eeff2d__blk1026_dn6: f64 = *var_eeff2d__blk1026_dn6_slot;
        let mut var_eeff2d__blk1026_dn7: f64 = *var_eeff2d__blk1026_dn7_slot;
        let mut var_eeff2d__blk1026_dn8: f64 = *var_eeff2d__blk1026_dn8_slot;
        let mut var_eeff2d__blk1026_dn9: f64 = *var_eeff2d__blk1026_dn9_slot;
        let mut var_esurf1__blk1027: f64 = *var_esurf1__blk1027_slot;
        let mut var_esurf1__blk1027_dn4: f64 = *var_esurf1__blk1027_dn4_slot;
        let mut var_esurf1__blk1027_dn6: f64 = *var_esurf1__blk1027_dn6_slot;
        let mut var_esurf1__blk1027_dn7: f64 = *var_esurf1__blk1027_dn7_slot;
        let mut var_esurf1__blk1027_dn8: f64 = *var_esurf1__blk1027_dn8_slot;
        let mut var_esurf1__blk1027_dn9: f64 = *var_esurf1__blk1027_dn9_slot;
        let mut var_esurf2__blk1028: f64 = *var_esurf2__blk1028_slot;
        let mut var_esurf2__blk1028_dn4: f64 = *var_esurf2__blk1028_dn4_slot;
        let mut var_esurf2__blk1028_dn6: f64 = *var_esurf2__blk1028_dn6_slot;
        let mut var_esurf2__blk1028_dn7: f64 = *var_esurf2__blk1028_dn7_slot;
        let mut var_esurf2__blk1028_dn8: f64 = *var_esurf2__blk1028_dn8_slot;
        let mut var_esurf2__blk1028_dn9: f64 = *var_esurf2__blk1028_dn9_slot;
        let mut var_fcor__blk1038: f64 = *var_fcor__blk1038_slot;
        let mut var_fcor__blk1038_dn4: f64 = *var_fcor__blk1038_dn4_slot;
        let mut var_fcor__blk1038_dn6: f64 = *var_fcor__blk1038_dn6_slot;
        let mut var_fcor__blk1038_dn7: f64 = *var_fcor__blk1038_dn7_slot;
        let mut var_fcor__blk1038_dn8: f64 = *var_fcor__blk1038_dn8_slot;
        let mut var_fcor__blk1038_dn9: f64 = *var_fcor__blk1038_dn9_slot;
        let mut var_gcs__blk1039: f64 = *var_gcs__blk1039_slot;
        let mut var_gcs__blk1039_dn4: f64 = *var_gcs__blk1039_dn4_slot;
        let mut var_gcs__blk1039_dn6: f64 = *var_gcs__blk1039_dn6_slot;
        let mut var_gcs__blk1039_dn7: f64 = *var_gcs__blk1039_dn7_slot;
        let mut var_gcs__blk1039_dn8: f64 = *var_gcs__blk1039_dn8_slot;
        let mut var_gcs__blk1039_dn9: f64 = *var_gcs__blk1039_dn9_slot;
        let mut var_gmob1__blk1041: f64 = *var_gmob1__blk1041_slot;
        let mut var_gmob1__blk1041_dn4: f64 = *var_gmob1__blk1041_dn4_slot;
        let mut var_gmob1__blk1041_dn6: f64 = *var_gmob1__blk1041_dn6_slot;
        let mut var_gmob1__blk1041_dn7: f64 = *var_gmob1__blk1041_dn7_slot;
        let mut var_gmob1__blk1041_dn8: f64 = *var_gmob1__blk1041_dn8_slot;
        let mut var_gmob1__blk1041_dn9: f64 = *var_gmob1__blk1041_dn9_slot;
        let mut var_grs__blk1040: f64 = *var_grs__blk1040_slot;
        let mut var_grs__blk1040_dn4: f64 = *var_grs__blk1040_dn4_slot;
        let mut var_grs__blk1040_dn6: f64 = *var_grs__blk1040_dn6_slot;
        let mut var_grs__blk1040_dn7: f64 = *var_grs__blk1040_dn7_slot;
        let mut var_grs__blk1040_dn8: f64 = *var_grs__blk1040_dn8_slot;
        let mut var_grs__blk1040_dn9: f64 = *var_grs__blk1040_dn9_slot;
        let mut var_guard1222: f64 = *var_guard1222_slot;
        let mut var_guard1223: f64 = *var_guard1223_slot;
        let mut var_qi1m__blk1029: f64 = *var_qi1m__blk1029_slot;
        let mut var_qi1m__blk1029_dn4: f64 = *var_qi1m__blk1029_dn4_slot;
        let mut var_qi1m__blk1029_dn6: f64 = *var_qi1m__blk1029_dn6_slot;
        let mut var_qi1m__blk1029_dn7: f64 = *var_qi1m__blk1029_dn7_slot;
        let mut var_qi1m__blk1029_dn8: f64 = *var_qi1m__blk1029_dn8_slot;
        let mut var_qi1m__blk1029_dn9: f64 = *var_qi1m__blk1029_dn9_slot;
        let mut var_qi2m__blk1030: f64 = *var_qi2m__blk1030_slot;
        let mut var_qi2m__blk1030_dn4: f64 = *var_qi2m__blk1030_dn4_slot;
        let mut var_qi2m__blk1030_dn6: f64 = *var_qi2m__blk1030_dn6_slot;
        let mut var_qi2m__blk1030_dn7: f64 = *var_qi2m__blk1030_dn7_slot;
        let mut var_qi2m__blk1030_dn8: f64 = *var_qi2m__blk1030_dn8_slot;
        let mut var_qi2m__blk1030_dn9: f64 = *var_qi2m__blk1030_dn9_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp3: f64 = *var_temp3_slot;
        let mut var_temp3_dn4: f64 = *var_temp3_dn4_slot;
        let mut var_temp3_dn6: f64 = *var_temp3_dn6_slot;
        let mut var_temp3_dn7: f64 = *var_temp3_dn7_slot;
        let mut var_temp3_dn8: f64 = *var_temp3_dn8_slot;
        let mut var_temp3_dn9: f64 = *var_temp3_dn9_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;

        let (assign40290_e46030, assign40290_e46030_d_n4, assign40290_e46030_d_n6, assign40290_e46030_d_n7, assign40290_e46030_d_n8, assign40290_e46030_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40290_e46028: f64 = (var_esurf2d__blk1022 - var_k2q2d__blk1005);
        (assign40290_e46028, (var_esurf2d__blk1022_dn4 - var_k2q2d__blk1005_dn4), (var_esurf2d__blk1022_dn6 - var_k2q2d__blk1005_dn6), (var_esurf2d__blk1022_dn7 - var_k2q2d__blk1005_dn7), (var_esurf2d__blk1022_dn8 - var_k2q2d__blk1005_dn8), (var_esurf2d__blk1022_dn9 - var_k2q2d__blk1005_dn9),)
    } else {
        (var_ecpl1d__blk1023, var_ecpl1d__blk1023_dn4, var_ecpl1d__blk1023_dn6, var_ecpl1d__blk1023_dn7, var_ecpl1d__blk1023_dn8, var_ecpl1d__blk1023_dn9,)
    }
};
        var_ecpl1d__blk1023 = assign40290_e46030;
        var_ecpl1d__blk1023_dn4 = assign40290_e46030_d_n4;
        var_ecpl1d__blk1023_dn6 = assign40290_e46030_d_n6;
        var_ecpl1d__blk1023_dn7 = assign40290_e46030_d_n7;
        var_ecpl1d__blk1023_dn8 = assign40290_e46030_d_n8;
        var_ecpl1d__blk1023_dn9 = assign40290_e46030_d_n9;

        let (assign40300_e46036, assign40300_e46036_d_n4, assign40300_e46036_d_n6, assign40300_e46036_d_n7, assign40300_e46036_d_n8, assign40300_e46036_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40300_e46034: f64 = (var_esurf1d__blk1021 - var_k1q1d__blk1004);
        (assign40300_e46034, (var_esurf1d__blk1021_dn4 - var_k1q1d__blk1004_dn4), (var_esurf1d__blk1021_dn6 - var_k1q1d__blk1004_dn6), (var_esurf1d__blk1021_dn7 - var_k1q1d__blk1004_dn7), (var_esurf1d__blk1021_dn8 - var_k1q1d__blk1004_dn8), (var_esurf1d__blk1021_dn9 - var_k1q1d__blk1004_dn9),)
    } else {
        (var_ecpl2d__blk1024, var_ecpl2d__blk1024_dn4, var_ecpl2d__blk1024_dn6, var_ecpl2d__blk1024_dn7, var_ecpl2d__blk1024_dn8, var_ecpl2d__blk1024_dn9,)
    }
};
        var_ecpl2d__blk1024 = assign40300_e46036;
        var_ecpl2d__blk1024_dn4 = assign40300_e46036_d_n4;
        var_ecpl2d__blk1024_dn6 = assign40300_e46036_d_n6;
        var_ecpl2d__blk1024_dn7 = assign40300_e46036_d_n7;
        var_ecpl2d__blk1024_dn8 = assign40300_e46036_d_n8;
        var_ecpl2d__blk1024_dn9 = assign40300_e46036_d_n9;

        let (assign40310_e46046, assign40310_e46046_d_n4, assign40310_e46046_d_n6, assign40310_e46046_d_n7, assign40310_e46046_d_n8, assign40310_e46046_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40310_e46040: f64 = (var_eta_mu * var_esurf1d__blk1021);
        let assign40310_e46043: f64 = (var_one_m_eta * var_ecpl1d__blk1023);
        let assign40310_e46044: f64 = (assign40310_e46040 + assign40310_e46043);
        (assign40310_e46044, ((var_eta_mu * var_esurf1d__blk1021_dn4) + (var_one_m_eta * var_ecpl1d__blk1023_dn4)), ((var_eta_mu * var_esurf1d__blk1021_dn6) + (var_one_m_eta * var_ecpl1d__blk1023_dn6)), ((var_eta_mu * var_esurf1d__blk1021_dn7) + (var_one_m_eta * var_ecpl1d__blk1023_dn7)), ((var_eta_mu * var_esurf1d__blk1021_dn8) + (var_one_m_eta * var_ecpl1d__blk1023_dn8)), ((var_eta_mu * var_esurf1d__blk1021_dn9) + (var_one_m_eta * var_ecpl1d__blk1023_dn9)),)
    } else {
        (var_eeff1d__blk1025, var_eeff1d__blk1025_dn4, var_eeff1d__blk1025_dn6, var_eeff1d__blk1025_dn7, var_eeff1d__blk1025_dn8, var_eeff1d__blk1025_dn9,)
    }
};
        var_eeff1d__blk1025 = assign40310_e46046;
        var_eeff1d__blk1025_dn4 = assign40310_e46046_d_n4;
        var_eeff1d__blk1025_dn6 = assign40310_e46046_d_n6;
        var_eeff1d__blk1025_dn7 = assign40310_e46046_d_n7;
        var_eeff1d__blk1025_dn8 = assign40310_e46046_d_n8;
        var_eeff1d__blk1025_dn9 = assign40310_e46046_d_n9;

        let (assign40320_e46056, assign40320_e46056_d_n4, assign40320_e46056_d_n6, assign40320_e46056_d_n7, assign40320_e46056_d_n8, assign40320_e46056_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40320_e46050: f64 = (var_eta_mu * var_esurf2d__blk1022);
        let assign40320_e46053: f64 = (var_one_m_eta * var_ecpl2d__blk1024);
        let assign40320_e46054: f64 = (assign40320_e46050 + assign40320_e46053);
        (assign40320_e46054, ((var_eta_mu * var_esurf2d__blk1022_dn4) + (var_one_m_eta * var_ecpl2d__blk1024_dn4)), ((var_eta_mu * var_esurf2d__blk1022_dn6) + (var_one_m_eta * var_ecpl2d__blk1024_dn6)), ((var_eta_mu * var_esurf2d__blk1022_dn7) + (var_one_m_eta * var_ecpl2d__blk1024_dn7)), ((var_eta_mu * var_esurf2d__blk1022_dn8) + (var_one_m_eta * var_ecpl2d__blk1024_dn8)), ((var_eta_mu * var_esurf2d__blk1022_dn9) + (var_one_m_eta * var_ecpl2d__blk1024_dn9)),)
    } else {
        (var_eeff2d__blk1026, var_eeff2d__blk1026_dn4, var_eeff2d__blk1026_dn6, var_eeff2d__blk1026_dn7, var_eeff2d__blk1026_dn8, var_eeff2d__blk1026_dn9,)
    }
};
        var_eeff2d__blk1026 = assign40320_e46056;
        var_eeff2d__blk1026_dn4 = assign40320_e46056_d_n4;
        var_eeff2d__blk1026_dn6 = assign40320_e46056_d_n6;
        var_eeff2d__blk1026_dn7 = assign40320_e46056_d_n7;
        var_eeff2d__blk1026_dn8 = assign40320_e46056_d_n8;
        var_eeff2d__blk1026_dn9 = assign40320_e46056_d_n9;

        let (assign40330_e46064, assign40330_e46064_d_n4, assign40330_e46064_d_n6, assign40330_e46064_d_n7, assign40330_e46064_d_n8, assign40330_e46064_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40330_e46061: f64 = (var_esurf1s__blk952 + var_esurf1d__blk1021);
        let assign40330_e46062: f64 = (0.5 * assign40330_e46061);
        (assign40330_e46062, (0.5 * (var_esurf1s__blk952_dn4 + var_esurf1d__blk1021_dn4)), (0.5 * (var_esurf1s__blk952_dn6 + var_esurf1d__blk1021_dn6)), (0.5 * (var_esurf1s__blk952_dn7 + var_esurf1d__blk1021_dn7)), (0.5 * (var_esurf1s__blk952_dn8 + var_esurf1d__blk1021_dn8)), (0.5 * (var_esurf1s__blk952_dn9 + var_esurf1d__blk1021_dn9)),)
    } else {
        (var_esurf1__blk1027, var_esurf1__blk1027_dn4, var_esurf1__blk1027_dn6, var_esurf1__blk1027_dn7, var_esurf1__blk1027_dn8, var_esurf1__blk1027_dn9,)
    }
};
        var_esurf1__blk1027 = assign40330_e46064;
        var_esurf1__blk1027_dn4 = assign40330_e46064_d_n4;
        var_esurf1__blk1027_dn6 = assign40330_e46064_d_n6;
        var_esurf1__blk1027_dn7 = assign40330_e46064_d_n7;
        var_esurf1__blk1027_dn8 = assign40330_e46064_d_n8;
        var_esurf1__blk1027_dn9 = assign40330_e46064_d_n9;

        let (assign40340_e46072, assign40340_e46072_d_n4, assign40340_e46072_d_n6, assign40340_e46072_d_n7, assign40340_e46072_d_n8, assign40340_e46072_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40340_e46069: f64 = (var_esurf2s__blk953 + var_esurf2d__blk1022);
        let assign40340_e46070: f64 = (0.5 * assign40340_e46069);
        (assign40340_e46070, (0.5 * (var_esurf2s__blk953_dn4 + var_esurf2d__blk1022_dn4)), (0.5 * (var_esurf2s__blk953_dn6 + var_esurf2d__blk1022_dn6)), (0.5 * (var_esurf2s__blk953_dn7 + var_esurf2d__blk1022_dn7)), (0.5 * (var_esurf2s__blk953_dn8 + var_esurf2d__blk1022_dn8)), (0.5 * (var_esurf2s__blk953_dn9 + var_esurf2d__blk1022_dn9)),)
    } else {
        (var_esurf2__blk1028, var_esurf2__blk1028_dn4, var_esurf2__blk1028_dn6, var_esurf2__blk1028_dn7, var_esurf2__blk1028_dn8, var_esurf2__blk1028_dn9,)
    }
};
        var_esurf2__blk1028 = assign40340_e46072;
        var_esurf2__blk1028_dn4 = assign40340_e46072_d_n4;
        var_esurf2__blk1028_dn6 = assign40340_e46072_d_n6;
        var_esurf2__blk1028_dn7 = assign40340_e46072_d_n7;
        var_esurf2__blk1028_dn8 = assign40340_e46072_d_n8;
        var_esurf2__blk1028_dn9 = assign40340_e46072_d_n9;

        let (assign40350_e46080, assign40350_e46080_d_n4, assign40350_e46080_d_n6, assign40350_e46080_d_n7, assign40350_e46080_d_n8, assign40350_e46080_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40350_e46077: f64 = (var_esurf1__blk1027 + var_esurf2__blk1028);
        let assign40350_e46078: f64 = (1.0 / assign40350_e46077);
        (assign40350_e46078, (-((var_esurf1__blk1027_dn4 + var_esurf2__blk1028_dn4) / (assign40350_e46077 * assign40350_e46077))), (-((var_esurf1__blk1027_dn6 + var_esurf2__blk1028_dn6) / (assign40350_e46077 * assign40350_e46077))), (-((var_esurf1__blk1027_dn7 + var_esurf2__blk1028_dn7) / (assign40350_e46077 * assign40350_e46077))), (-((var_esurf1__blk1027_dn8 + var_esurf2__blk1028_dn8) / (assign40350_e46077 * assign40350_e46077))), (-((var_esurf1__blk1027_dn9 + var_esurf2__blk1028_dn9) / (assign40350_e46077 * assign40350_e46077))),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign40350_e46080;
        var_temp_dn4 = assign40350_e46080_d_n4;
        var_temp_dn6 = assign40350_e46080_d_n6;
        var_temp_dn7 = assign40350_e46080_d_n7;
        var_temp_dn8 = assign40350_e46080_d_n8;
        var_temp_dn9 = assign40350_e46080_d_n9;

        let (assign40360_e46088, assign40360_e46088_d_n4, assign40360_e46088_d_n6, assign40360_e46088_d_n7, assign40360_e46088_d_n8, assign40360_e46088_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40360_e46084: f64 = (var_qim__blk1016 * var_esurf1__blk1027);
        let assign40360_e46086: f64 = (assign40360_e46084 * var_temp);
        (assign40360_e46086, ((((var_qim__blk1016_dn4 * var_esurf1__blk1027) + (var_qim__blk1016 * var_esurf1__blk1027_dn4)) * var_temp) + (assign40360_e46084 * var_temp_dn4)), ((((var_qim__blk1016_dn6 * var_esurf1__blk1027) + (var_qim__blk1016 * var_esurf1__blk1027_dn6)) * var_temp) + (assign40360_e46084 * var_temp_dn6)), ((((var_qim__blk1016_dn7 * var_esurf1__blk1027) + (var_qim__blk1016 * var_esurf1__blk1027_dn7)) * var_temp) + (assign40360_e46084 * var_temp_dn7)), ((((var_qim__blk1016_dn8 * var_esurf1__blk1027) + (var_qim__blk1016 * var_esurf1__blk1027_dn8)) * var_temp) + (assign40360_e46084 * var_temp_dn8)), ((((var_qim__blk1016_dn9 * var_esurf1__blk1027) + (var_qim__blk1016 * var_esurf1__blk1027_dn9)) * var_temp) + (assign40360_e46084 * var_temp_dn9)),)
    } else {
        (var_qi1m__blk1029, var_qi1m__blk1029_dn4, var_qi1m__blk1029_dn6, var_qi1m__blk1029_dn7, var_qi1m__blk1029_dn8, var_qi1m__blk1029_dn9,)
    }
};
        var_qi1m__blk1029 = assign40360_e46088;
        var_qi1m__blk1029_dn4 = assign40360_e46088_d_n4;
        var_qi1m__blk1029_dn6 = assign40360_e46088_d_n6;
        var_qi1m__blk1029_dn7 = assign40360_e46088_d_n7;
        var_qi1m__blk1029_dn8 = assign40360_e46088_d_n8;
        var_qi1m__blk1029_dn9 = assign40360_e46088_d_n9;

        let (assign40370_e46096, assign40370_e46096_d_n4, assign40370_e46096_d_n6, assign40370_e46096_d_n7, assign40370_e46096_d_n8, assign40370_e46096_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40370_e46092: f64 = (var_qim__blk1016 * var_esurf2__blk1028);
        let assign40370_e46094: f64 = (assign40370_e46092 * var_temp);
        (assign40370_e46094, ((((var_qim__blk1016_dn4 * var_esurf2__blk1028) + (var_qim__blk1016 * var_esurf2__blk1028_dn4)) * var_temp) + (assign40370_e46092 * var_temp_dn4)), ((((var_qim__blk1016_dn6 * var_esurf2__blk1028) + (var_qim__blk1016 * var_esurf2__blk1028_dn6)) * var_temp) + (assign40370_e46092 * var_temp_dn6)), ((((var_qim__blk1016_dn7 * var_esurf2__blk1028) + (var_qim__blk1016 * var_esurf2__blk1028_dn7)) * var_temp) + (assign40370_e46092 * var_temp_dn7)), ((((var_qim__blk1016_dn8 * var_esurf2__blk1028) + (var_qim__blk1016 * var_esurf2__blk1028_dn8)) * var_temp) + (assign40370_e46092 * var_temp_dn8)), ((((var_qim__blk1016_dn9 * var_esurf2__blk1028) + (var_qim__blk1016 * var_esurf2__blk1028_dn9)) * var_temp) + (assign40370_e46092 * var_temp_dn9)),)
    } else {
        (var_qi2m__blk1030, var_qi2m__blk1030_dn4, var_qi2m__blk1030_dn6, var_qi2m__blk1030_dn7, var_qi2m__blk1030_dn8, var_qi2m__blk1030_dn9,)
    }
};
        var_qi2m__blk1030 = assign40370_e46096;
        var_qi2m__blk1030_dn4 = assign40370_e46096_d_n4;
        var_qi2m__blk1030_dn6 = assign40370_e46096_d_n6;
        var_qi2m__blk1030_dn7 = assign40370_e46096_d_n7;
        var_qi2m__blk1030_dn8 = assign40370_e46096_d_n8;
        var_qi2m__blk1030_dn9 = assign40370_e46096_d_n9;

        let (assign40380_e46104, assign40380_e46104_d_n4, assign40380_e46104_d_n6, assign40380_e46104_d_n7, assign40380_e46104_d_n8, assign40380_e46104_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40380_e46101: f64 = (var_ecpl1s__blk954 + var_ecpl1d__blk1023);
        let assign40380_e46102: f64 = (0.5 * assign40380_e46101);
        (assign40380_e46102, (0.5 * (var_ecpl1s__blk954_dn4 + var_ecpl1d__blk1023_dn4)), (0.5 * (var_ecpl1s__blk954_dn6 + var_ecpl1d__blk1023_dn6)), (0.5 * (var_ecpl1s__blk954_dn7 + var_ecpl1d__blk1023_dn7)), (0.5 * (var_ecpl1s__blk954_dn8 + var_ecpl1d__blk1023_dn8)), (0.5 * (var_ecpl1s__blk954_dn9 + var_ecpl1d__blk1023_dn9)),)
    } else {
        (var_ecpl1__blk1031, var_ecpl1__blk1031_dn4, var_ecpl1__blk1031_dn6, var_ecpl1__blk1031_dn7, var_ecpl1__blk1031_dn8, var_ecpl1__blk1031_dn9,)
    }
};
        var_ecpl1__blk1031 = assign40380_e46104;
        var_ecpl1__blk1031_dn4 = assign40380_e46104_d_n4;
        var_ecpl1__blk1031_dn6 = assign40380_e46104_d_n6;
        var_ecpl1__blk1031_dn7 = assign40380_e46104_d_n7;
        var_ecpl1__blk1031_dn8 = assign40380_e46104_d_n8;
        var_ecpl1__blk1031_dn9 = assign40380_e46104_d_n9;

        let (assign40390_e46112, assign40390_e46112_d_n4, assign40390_e46112_d_n6, assign40390_e46112_d_n7, assign40390_e46112_d_n8, assign40390_e46112_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40390_e46109: f64 = (var_ecpl2s__blk955 + var_ecpl2d__blk1024);
        let assign40390_e46110: f64 = (0.5 * assign40390_e46109);
        (assign40390_e46110, (0.5 * (var_ecpl2s__blk955_dn4 + var_ecpl2d__blk1024_dn4)), (0.5 * (var_ecpl2s__blk955_dn6 + var_ecpl2d__blk1024_dn6)), (0.5 * (var_ecpl2s__blk955_dn7 + var_ecpl2d__blk1024_dn7)), (0.5 * (var_ecpl2s__blk955_dn8 + var_ecpl2d__blk1024_dn8)), (0.5 * (var_ecpl2s__blk955_dn9 + var_ecpl2d__blk1024_dn9)),)
    } else {
        (var_ecpl2__blk1032, var_ecpl2__blk1032_dn4, var_ecpl2__blk1032_dn6, var_ecpl2__blk1032_dn7, var_ecpl2__blk1032_dn8, var_ecpl2__blk1032_dn9,)
    }
};
        var_ecpl2__blk1032 = assign40390_e46112;
        var_ecpl2__blk1032_dn4 = assign40390_e46112_d_n4;
        var_ecpl2__blk1032_dn6 = assign40390_e46112_d_n6;
        var_ecpl2__blk1032_dn7 = assign40390_e46112_d_n7;
        var_ecpl2__blk1032_dn8 = assign40390_e46112_d_n8;
        var_ecpl2__blk1032_dn9 = assign40390_e46112_d_n9;

        let (assign40400_e46120, assign40400_e46120_d_n4, assign40400_e46120_d_n6, assign40400_e46120_d_n7, assign40400_e46120_d_n8, assign40400_e46120_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40400_e46117: f64 = (var_eeff1s__blk956 + var_eeff1d__blk1025);
        let assign40400_e46118: f64 = (0.5 * assign40400_e46117);
        (assign40400_e46118, (0.5 * (var_eeff1s__blk956_dn4 + var_eeff1d__blk1025_dn4)), (0.5 * (var_eeff1s__blk956_dn6 + var_eeff1d__blk1025_dn6)), (0.5 * (var_eeff1s__blk956_dn7 + var_eeff1d__blk1025_dn7)), (0.5 * (var_eeff1s__blk956_dn8 + var_eeff1d__blk1025_dn8)), (0.5 * (var_eeff1s__blk956_dn9 + var_eeff1d__blk1025_dn9)),)
    } else {
        (var_eeff1__blk1033, var_eeff1__blk1033_dn4, var_eeff1__blk1033_dn6, var_eeff1__blk1033_dn7, var_eeff1__blk1033_dn8, var_eeff1__blk1033_dn9,)
    }
};
        var_eeff1__blk1033 = assign40400_e46120;
        var_eeff1__blk1033_dn4 = assign40400_e46120_d_n4;
        var_eeff1__blk1033_dn6 = assign40400_e46120_d_n6;
        var_eeff1__blk1033_dn7 = assign40400_e46120_d_n7;
        var_eeff1__blk1033_dn8 = assign40400_e46120_d_n8;
        var_eeff1__blk1033_dn9 = assign40400_e46120_d_n9;

        let (assign40410_e46128, assign40410_e46128_d_n4, assign40410_e46128_d_n6, assign40410_e46128_d_n7, assign40410_e46128_d_n8, assign40410_e46128_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40410_e46125: f64 = (var_eeff2s__blk957 + var_eeff2d__blk1026);
        let assign40410_e46126: f64 = (0.5 * assign40410_e46125);
        (assign40410_e46126, (0.5 * (var_eeff2s__blk957_dn4 + var_eeff2d__blk1026_dn4)), (0.5 * (var_eeff2s__blk957_dn6 + var_eeff2d__blk1026_dn6)), (0.5 * (var_eeff2s__blk957_dn7 + var_eeff2d__blk1026_dn7)), (0.5 * (var_eeff2s__blk957_dn8 + var_eeff2d__blk1026_dn8)), (0.5 * (var_eeff2s__blk957_dn9 + var_eeff2d__blk1026_dn9)),)
    } else {
        (var_eeff2__blk1034, var_eeff2__blk1034_dn4, var_eeff2__blk1034_dn6, var_eeff2__blk1034_dn7, var_eeff2__blk1034_dn8, var_eeff2__blk1034_dn9,)
    }
};
        var_eeff2__blk1034 = assign40410_e46128;
        var_eeff2__blk1034_dn4 = assign40410_e46128_d_n4;
        var_eeff2__blk1034_dn6 = assign40410_e46128_d_n6;
        var_eeff2__blk1034_dn7 = assign40410_e46128_d_n7;
        var_eeff2__blk1034_dn8 = assign40410_e46128_d_n8;
        var_eeff2__blk1034_dn9 = assign40410_e46128_d_n9;

        let (assign40420_e46141, assign40420_e46141_d_n4, assign40420_e46141_d_n6, assign40420_e46141_d_n7, assign40420_e46141_d_n8, assign40420_e46141_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40420_e46132: f64 = (var_esurf1__blk1027 * var_betn1_t);
        let assign40420_e46135: f64 = (var_stbet_i * var_lnrtn);
        let assign40420_e46136: f64 = (assign40420_e46135).exp();
        let assign40420_e46137: f64 = (assign40420_e46132 * assign40420_e46136);
        let assign40420_e46139: f64 = (assign40420_e46137 * var_ratio_pd__blk1020);
        (assign40420_e46139, ((((((var_esurf1__blk1027_dn4 * var_betn1_t) + (var_esurf1__blk1027 * var_betn1_t_dn4)) * assign40420_e46136) + (assign40420_e46132 * (assign40420_e46136 * (var_stbet_i * var_lnrtn_dn4)))) * var_ratio_pd__blk1020) + (assign40420_e46137 * var_ratio_pd__blk1020_dn4)), ((((((var_esurf1__blk1027_dn6 * var_betn1_t) + (var_esurf1__blk1027 * var_betn1_t_dn6)) * assign40420_e46136) + (assign40420_e46132 * (assign40420_e46136 * (var_stbet_i * var_lnrtn_dn6)))) * var_ratio_pd__blk1020) + (assign40420_e46137 * var_ratio_pd__blk1020_dn6)), ((((((var_esurf1__blk1027_dn7 * var_betn1_t) + (var_esurf1__blk1027 * var_betn1_t_dn7)) * assign40420_e46136) + (assign40420_e46132 * (assign40420_e46136 * (var_stbet_i * var_lnrtn_dn7)))) * var_ratio_pd__blk1020) + (assign40420_e46137 * var_ratio_pd__blk1020_dn7)), ((((((var_esurf1__blk1027_dn8 * var_betn1_t) + (var_esurf1__blk1027 * var_betn1_t_dn8)) * assign40420_e46136) + (assign40420_e46132 * (assign40420_e46136 * (var_stbet_i * var_lnrtn_dn8)))) * var_ratio_pd__blk1020) + (assign40420_e46137 * var_ratio_pd__blk1020_dn8)), ((((((var_esurf1__blk1027_dn9 * var_betn1_t) + (var_esurf1__blk1027 * var_betn1_t_dn9)) * assign40420_e46136) + (assign40420_e46132 * (assign40420_e46136 * (var_stbet_i * var_lnrtn_dn9)))) * var_ratio_pd__blk1020) + (assign40420_e46137 * var_ratio_pd__blk1020_dn9)),)
    } else {
        (var_c1__blk1035, var_c1__blk1035_dn4, var_c1__blk1035_dn6, var_c1__blk1035_dn7, var_c1__blk1035_dn8, var_c1__blk1035_dn9,)
    }
};
        var_c1__blk1035 = assign40420_e46141;
        var_c1__blk1035_dn4 = assign40420_e46141_d_n4;
        var_c1__blk1035_dn6 = assign40420_e46141_d_n6;
        var_c1__blk1035_dn7 = assign40420_e46141_d_n7;
        var_c1__blk1035_dn8 = assign40420_e46141_d_n8;
        var_c1__blk1035_dn9 = assign40420_e46141_d_n9;

        let (assign40430_e46152, assign40430_e46152_d_n4, assign40430_e46152_d_n6, assign40430_e46152_d_n7, assign40430_e46152_d_n8, assign40430_e46152_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40430_e46145: f64 = (var_esurf2__blk1028 * var_betn2_t);
        let assign40430_e46148: f64 = (var_stbet_i * var_lnrtn);
        let assign40430_e46149: f64 = (assign40430_e46148).exp();
        let assign40430_e46150: f64 = (assign40430_e46145 * assign40430_e46149);
        (assign40430_e46150, ((((var_esurf2__blk1028_dn4 * var_betn2_t) + (var_esurf2__blk1028 * var_betn2_t_dn4)) * assign40430_e46149) + (assign40430_e46145 * (assign40430_e46149 * (var_stbet_i * var_lnrtn_dn4)))), ((((var_esurf2__blk1028_dn6 * var_betn2_t) + (var_esurf2__blk1028 * var_betn2_t_dn6)) * assign40430_e46149) + (assign40430_e46145 * (assign40430_e46149 * (var_stbet_i * var_lnrtn_dn6)))), ((((var_esurf2__blk1028_dn7 * var_betn2_t) + (var_esurf2__blk1028 * var_betn2_t_dn7)) * assign40430_e46149) + (assign40430_e46145 * (assign40430_e46149 * (var_stbet_i * var_lnrtn_dn7)))), ((((var_esurf2__blk1028_dn8 * var_betn2_t) + (var_esurf2__blk1028 * var_betn2_t_dn8)) * assign40430_e46149) + (assign40430_e46145 * (assign40430_e46149 * (var_stbet_i * var_lnrtn_dn8)))), ((((var_esurf2__blk1028_dn9 * var_betn2_t) + (var_esurf2__blk1028 * var_betn2_t_dn9)) * assign40430_e46149) + (assign40430_e46145 * (assign40430_e46149 * (var_stbet_i * var_lnrtn_dn9)))),)
    } else {
        (var_c2__blk1036, var_c2__blk1036_dn4, var_c2__blk1036_dn6, var_c2__blk1036_dn7, var_c2__blk1036_dn8, var_c2__blk1036_dn9,)
    }
};
        var_c2__blk1036 = assign40430_e46152;
        var_c2__blk1036_dn4 = assign40430_e46152_d_n4;
        var_c2__blk1036_dn6 = assign40430_e46152_d_n6;
        var_c2__blk1036_dn7 = assign40430_e46152_d_n7;
        var_c2__blk1036_dn8 = assign40430_e46152_d_n8;
        var_c2__blk1036_dn9 = assign40430_e46152_d_n9;

        let (assign40440_e46158, assign40440_e46158_d_n4, assign40440_e46158_d_n6, assign40440_e46158_d_n7, assign40440_e46158_d_n8, assign40440_e46158_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40440_e46156: f64 = (var_c1__blk1035 + var_c2__blk1036);
        (assign40440_e46156, (var_c1__blk1035_dn4 + var_c2__blk1036_dn4), (var_c1__blk1035_dn6 + var_c2__blk1036_dn6), (var_c1__blk1035_dn7 + var_c2__blk1036_dn7), (var_c1__blk1035_dn8 + var_c2__blk1036_dn8), (var_c1__blk1035_dn9 + var_c2__blk1036_dn9),)
    } else {
        (var_csum__blk1037, var_csum__blk1037_dn4, var_csum__blk1037_dn6, var_csum__blk1037_dn7, var_csum__blk1037_dn8, var_csum__blk1037_dn9,)
    }
};
        var_csum__blk1037 = assign40440_e46158;
        var_csum__blk1037_dn4 = assign40440_e46158_d_n4;
        var_csum__blk1037_dn6 = assign40440_e46158_d_n6;
        var_csum__blk1037_dn7 = assign40440_e46158_d_n7;
        var_csum__blk1037_dn8 = assign40440_e46158_d_n8;
        var_csum__blk1037_dn9 = assign40440_e46158_d_n9;

        let (assign40450_e46168, assign40450_e46168_d_n4, assign40450_e46168_d_n6, assign40450_e46168_d_n7, assign40450_e46168_d_n8, assign40450_e46168_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40450_e46164: f64 = (var_xcorb_i * var_ecpl2__blk1032);
        let assign40450_e46165: f64 = (var_ecpl1__blk1031 + assign40450_e46164);
        let assign40450_e46166: f64 = (var_xcor_i * assign40450_e46165);
        (assign40450_e46166, ((var_xcor_i_dn4 * assign40450_e46165) + (var_xcor_i * (var_ecpl1__blk1031_dn4 + (var_xcorb_i * var_ecpl2__blk1032_dn4)))), ((var_xcor_i_dn6 * assign40450_e46165) + (var_xcor_i * (var_ecpl1__blk1031_dn6 + (var_xcorb_i * var_ecpl2__blk1032_dn6)))), ((var_xcor_i_dn7 * assign40450_e46165) + (var_xcor_i * (var_ecpl1__blk1031_dn7 + (var_xcorb_i * var_ecpl2__blk1032_dn7)))), ((var_xcor_i_dn8 * assign40450_e46165) + (var_xcor_i * (var_ecpl1__blk1031_dn8 + (var_xcorb_i * var_ecpl2__blk1032_dn8)))), ((var_xcor_i_dn9 * assign40450_e46165) + (var_xcor_i * (var_ecpl1__blk1031_dn9 + (var_xcorb_i * var_ecpl2__blk1032_dn9)))),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign40450_e46168;
        var_temp1_dn4 = assign40450_e46168_d_n4;
        var_temp1_dn6 = assign40450_e46168_d_n6;
        var_temp1_dn7 = assign40450_e46168_d_n7;
        var_temp1_dn8 = assign40450_e46168_d_n8;
        var_temp1_dn9 = assign40450_e46168_d_n9;

        let (assign40460_e46193, assign40460_e46193_d_n4, assign40460_e46193_d_n6, assign40460_e46193_d_n7, assign40460_e46193_d_n8, assign40460_e46193_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40460_e46173: f64 = (1.0 + var_temp1);
        let assign40460_e46175: f64 = assign40460_e46173;
        let assign40460_e46178: f64 = (1.0 + var_temp1);
        let assign40460_e46180: f64 = assign40460_e46178;
        let assign40460_e46183: f64 = (1.0 + var_temp1);
        let assign40460_e46185: f64 = assign40460_e46183;
        let assign40460_e46186: f64 = (assign40460_e46180 * assign40460_e46185);
        let assign40460_e46188: f64 = (assign40460_e46186 + 0.01);
        let assign40460_e46189: f64 = (assign40460_e46188).sqrt();
        let assign40460_e46190: f64 = (assign40460_e46175 + assign40460_e46189);
        let assign40460_e46191: f64 = (0.5 * assign40460_e46190);
        (assign40460_e46191, (0.5 * (var_temp1_dn4 + (((var_temp1_dn4 * assign40460_e46185) + (assign40460_e46180 * var_temp1_dn4)) / (2.0 * assign40460_e46189)))), (0.5 * (var_temp1_dn6 + (((var_temp1_dn6 * assign40460_e46185) + (assign40460_e46180 * var_temp1_dn6)) / (2.0 * assign40460_e46189)))), (0.5 * (var_temp1_dn7 + (((var_temp1_dn7 * assign40460_e46185) + (assign40460_e46180 * var_temp1_dn7)) / (2.0 * assign40460_e46189)))), (0.5 * (var_temp1_dn8 + (((var_temp1_dn8 * assign40460_e46185) + (assign40460_e46180 * var_temp1_dn8)) / (2.0 * assign40460_e46189)))), (0.5 * (var_temp1_dn9 + (((var_temp1_dn9 * assign40460_e46185) + (assign40460_e46180 * var_temp1_dn9)) / (2.0 * assign40460_e46189)))),)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign40460_e46193;
        var_temp2_dn4 = assign40460_e46193_d_n4;
        var_temp2_dn6 = assign40460_e46193_d_n6;
        var_temp2_dn7 = assign40460_e46193_d_n7;
        var_temp2_dn8 = assign40460_e46193_d_n8;
        var_temp2_dn9 = assign40460_e46193_d_n9;

        let (assign40470_e46224, assign40470_e46224_d_n4, assign40470_e46224_d_n6, assign40470_e46224_d_n7, assign40470_e46224_d_n8, assign40470_e46224_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40470_e46199: f64 = (0.2 * var_temp1);
        let assign40470_e46200: f64 = (1.0 + assign40470_e46199);
        let assign40470_e46202: f64 = assign40470_e46200;
        let assign40470_e46206: f64 = (0.2 * var_temp1);
        let assign40470_e46207: f64 = (1.0 + assign40470_e46206);
        let assign40470_e46209: f64 = assign40470_e46207;
        let assign40470_e46213: f64 = (0.2 * var_temp1);
        let assign40470_e46214: f64 = (1.0 + assign40470_e46213);
        let assign40470_e46216: f64 = assign40470_e46214;
        let assign40470_e46217: f64 = (assign40470_e46209 * assign40470_e46216);
        let assign40470_e46219: f64 = (assign40470_e46217 + 0.01);
        let assign40470_e46220: f64 = (assign40470_e46219).sqrt();
        let assign40470_e46221: f64 = (assign40470_e46202 + assign40470_e46220);
        let assign40470_e46222: f64 = (0.5 * assign40470_e46221);
        (assign40470_e46222, (0.5 * ((0.2 * var_temp1_dn4) + ((((0.2 * var_temp1_dn4) * assign40470_e46216) + (assign40470_e46209 * (0.2 * var_temp1_dn4))) / (2.0 * assign40470_e46220)))), (0.5 * ((0.2 * var_temp1_dn6) + ((((0.2 * var_temp1_dn6) * assign40470_e46216) + (assign40470_e46209 * (0.2 * var_temp1_dn6))) / (2.0 * assign40470_e46220)))), (0.5 * ((0.2 * var_temp1_dn7) + ((((0.2 * var_temp1_dn7) * assign40470_e46216) + (assign40470_e46209 * (0.2 * var_temp1_dn7))) / (2.0 * assign40470_e46220)))), (0.5 * ((0.2 * var_temp1_dn8) + ((((0.2 * var_temp1_dn8) * assign40470_e46216) + (assign40470_e46209 * (0.2 * var_temp1_dn8))) / (2.0 * assign40470_e46220)))), (0.5 * ((0.2 * var_temp1_dn9) + ((((0.2 * var_temp1_dn9) * assign40470_e46216) + (assign40470_e46209 * (0.2 * var_temp1_dn9))) / (2.0 * assign40470_e46220)))),)
    } else {
        (var_temp3, var_temp3_dn4, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9,)
    }
};
        var_temp3 = assign40470_e46224;
        var_temp3_dn4 = assign40470_e46224_d_n4;
        var_temp3_dn6 = assign40470_e46224_d_n6;
        var_temp3_dn7 = assign40470_e46224_d_n7;
        var_temp3_dn8 = assign40470_e46224_d_n8;
        var_temp3_dn9 = assign40470_e46224_d_n9;

        let (assign40480_e46230, assign40480_e46230_d_n4, assign40480_e46230_d_n6, assign40480_e46230_d_n7, assign40480_e46230_d_n8, assign40480_e46230_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40480_e46228: f64 = (var_temp2 / var_temp3);
        (assign40480_e46228, (((var_temp2_dn4 * var_temp3) - (var_temp2 * var_temp3_dn4)) / (var_temp3 * var_temp3)), (((var_temp2_dn6 * var_temp3) - (var_temp2 * var_temp3_dn6)) / (var_temp3 * var_temp3)), (((var_temp2_dn7 * var_temp3) - (var_temp2 * var_temp3_dn7)) / (var_temp3 * var_temp3)), (((var_temp2_dn8 * var_temp3) - (var_temp2 * var_temp3_dn8)) / (var_temp3 * var_temp3)), (((var_temp2_dn9 * var_temp3) - (var_temp2 * var_temp3_dn9)) / (var_temp3 * var_temp3)),)
    } else {
        (var_fcor__blk1038, var_fcor__blk1038_dn4, var_fcor__blk1038_dn6, var_fcor__blk1038_dn7, var_fcor__blk1038_dn8, var_fcor__blk1038_dn9,)
    }
};
        var_fcor__blk1038 = assign40480_e46230;
        var_fcor__blk1038_dn4 = assign40480_e46230_d_n4;
        var_fcor__blk1038_dn6 = assign40480_e46230_d_n6;
        var_fcor__blk1038_dn7 = assign40480_e46230_d_n7;
        var_fcor__blk1038_dn8 = assign40480_e46230_d_n8;
        var_fcor__blk1038_dn9 = assign40480_e46230_d_n9;

        let (assign40490_e46259, assign40490_e46259_d_n4, assign40490_e46259_d_n6, assign40490_e46259_d_n7, assign40490_e46259_d_n8, assign40490_e46259_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40490_e46236: f64 = (var_csfi_i * var_ecpl1__blk1031);
        let assign40490_e46237: f64 = (1.0 + assign40490_e46236);
        let assign40490_e46240: f64 = (var_csbi_i * var_ecpl2__blk1032);
        let assign40490_e46241: f64 = (assign40490_e46237 + assign40490_e46240);
        let assign40490_e46242: f64 = (var_cs_i * assign40490_e46241);
        let assign40490_e46244: f64 = (-var_thecs_i);
        let assign40490_e46248: f64 = (var_qi1m__blk1029 * var_inv_qi1cs);
        let assign40490_e46249: f64 = (1.0 + assign40490_e46248);
        let assign40490_e46252: f64 = (var_qi2m__blk1030 * var_inv_qi2cs);
        let assign40490_e46253: f64 = (assign40490_e46249 + assign40490_e46252);
        let assign40490_e46254: f64 = (assign40490_e46253).ln();
        let assign40490_e46255: f64 = (assign40490_e46244 * assign40490_e46254);
        let assign40490_e46256: f64 = (assign40490_e46255).exp();
        let assign40490_e46257: f64 = (assign40490_e46242 * assign40490_e46256);
        (assign40490_e46257, ((((var_cs_i_dn4 * assign40490_e46241) + (var_cs_i * ((var_csfi_i * var_ecpl1__blk1031_dn4) + (var_csbi_i * var_ecpl2__blk1032_dn4)))) * assign40490_e46256) + (assign40490_e46242 * (assign40490_e46256 * (((-var_thecs_i_dn4) * assign40490_e46254) + (assign40490_e46244 * (((var_qi1m__blk1029_dn4 * var_inv_qi1cs) + (var_qi2m__blk1030_dn4 * var_inv_qi2cs)) / assign40490_e46253)))))), ((((var_cs_i_dn6 * assign40490_e46241) + (var_cs_i * ((var_csfi_i * var_ecpl1__blk1031_dn6) + (var_csbi_i * var_ecpl2__blk1032_dn6)))) * assign40490_e46256) + (assign40490_e46242 * (assign40490_e46256 * (((-var_thecs_i_dn6) * assign40490_e46254) + (assign40490_e46244 * (((var_qi1m__blk1029_dn6 * var_inv_qi1cs) + (var_qi2m__blk1030_dn6 * var_inv_qi2cs)) / assign40490_e46253)))))), ((((var_cs_i_dn7 * assign40490_e46241) + (var_cs_i * ((var_csfi_i * var_ecpl1__blk1031_dn7) + (var_csbi_i * var_ecpl2__blk1032_dn7)))) * assign40490_e46256) + (assign40490_e46242 * (assign40490_e46256 * (((-var_thecs_i_dn7) * assign40490_e46254) + (assign40490_e46244 * (((var_qi1m__blk1029_dn7 * var_inv_qi1cs) + (var_qi2m__blk1030_dn7 * var_inv_qi2cs)) / assign40490_e46253)))))), ((((var_cs_i_dn8 * assign40490_e46241) + (var_cs_i * ((var_csfi_i * var_ecpl1__blk1031_dn8) + (var_csbi_i * var_ecpl2__blk1032_dn8)))) * assign40490_e46256) + (assign40490_e46242 * (assign40490_e46256 * (((-var_thecs_i_dn8) * assign40490_e46254) + (assign40490_e46244 * (((var_qi1m__blk1029_dn8 * var_inv_qi1cs) + (var_qi2m__blk1030_dn8 * var_inv_qi2cs)) / assign40490_e46253)))))), ((((var_cs_i_dn9 * assign40490_e46241) + (var_cs_i * ((var_csfi_i * var_ecpl1__blk1031_dn9) + (var_csbi_i * var_ecpl2__blk1032_dn9)))) * assign40490_e46256) + (assign40490_e46242 * (assign40490_e46256 * (((-var_thecs_i_dn9) * assign40490_e46254) + (assign40490_e46244 * (((var_qi1m__blk1029_dn9 * var_inv_qi1cs) + (var_qi2m__blk1030_dn9 * var_inv_qi2cs)) / assign40490_e46253)))))),)
    } else {
        (var_gcs__blk1039, var_gcs__blk1039_dn4, var_gcs__blk1039_dn6, var_gcs__blk1039_dn7, var_gcs__blk1039_dn8, var_gcs__blk1039_dn9,)
    }
};
        var_gcs__blk1039 = assign40490_e46259;
        var_gcs__blk1039_dn4 = assign40490_e46259_d_n4;
        var_gcs__blk1039_dn6 = assign40490_e46259_d_n6;
        var_gcs__blk1039_dn7 = assign40490_e46259_d_n7;
        var_gcs__blk1039_dn8 = assign40490_e46259_d_n8;
        var_gcs__blk1039_dn9 = assign40490_e46259_d_n9;

        let assign40500_e46262: f64 = if var_rsg_i == 0.0 { 1.0 } else { 0.0 };
        var_guard1222 = assign40500_e46262;

        let (assign40510_e46268, assign40510_e46268_d_n4, assign40510_e46268_d_n6, assign40510_e46268_d_n7, assign40510_e46268_d_n8, assign40510_e46268_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1222 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp3, var_temp3_dn4, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9,)
    }
};
        var_temp3 = assign40510_e46268;
        var_temp3_dn4 = assign40510_e46268_d_n4;
        var_temp3_dn6 = assign40510_e46268_d_n6;
        var_temp3_dn7 = assign40510_e46268_d_n7;
        var_temp3_dn8 = assign40510_e46268_d_n8;
        var_temp3_dn9 = assign40510_e46268_d_n9;

        let assign40520_e46271: f64 = if var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        var_guard1223 = assign40520_e46271;

        let (assign40530_e46288, assign40530_e46288_d_n4, assign40530_e46288_d_n6, assign40530_e46288_d_n7, assign40530_e46288_d_n8, assign40530_e46288_d_n9,) = {
    if (((var_guard1080 != 0.0) && (var_guard1222 == 0.0)) && (var_guard1223 != 0.0)) {
        let assign40530_e46282: f64 = (var_qim__blk1016 + 1e-12);
        let assign40530_e46283: f64 = (assign40530_e46282).ln();
        let assign40530_e46284: f64 = (var_thersg_i * assign40530_e46283);
        let assign40530_e46285: f64 = (assign40530_e46284).exp();
        let assign40530_e46286: f64 = (var_rsg_i * assign40530_e46285);
        (assign40530_e46286, (var_rsg_i * (assign40530_e46285 * (var_thersg_i * (var_qim__blk1016_dn4 / assign40530_e46282)))), (var_rsg_i * (assign40530_e46285 * (var_thersg_i * (var_qim__blk1016_dn6 / assign40530_e46282)))), (var_rsg_i * (assign40530_e46285 * (var_thersg_i * (var_qim__blk1016_dn7 / assign40530_e46282)))), (var_rsg_i * (assign40530_e46285 * (var_thersg_i * (var_qim__blk1016_dn8 / assign40530_e46282)))), (var_rsg_i * (assign40530_e46285 * (var_thersg_i * (var_qim__blk1016_dn9 / assign40530_e46282)))),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign40530_e46288;
        var_temp1_dn4 = assign40530_e46288_d_n4;
        var_temp1_dn6 = assign40530_e46288_d_n6;
        var_temp1_dn7 = assign40530_e46288_d_n7;
        var_temp1_dn8 = assign40530_e46288_d_n8;
        var_temp1_dn9 = assign40530_e46288_d_n9;

        let (assign40540_e46299, assign40540_e46299_d_n4, assign40540_e46299_d_n6, assign40540_e46299_d_n7, assign40540_e46299_d_n8, assign40540_e46299_d_n9,) = {
    if (((var_guard1080 != 0.0) && (var_guard1222 == 0.0)) && (var_guard1223 != 0.0)) {
        let assign40540_e46297: f64 = (1.0 - var_temp1);
        (assign40540_e46297, (-var_temp1_dn4), (-var_temp1_dn6), (-var_temp1_dn7), (-var_temp1_dn8), (-var_temp1_dn9),)
    } else {
        (var_temp3, var_temp3_dn4, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9,)
    }
};
        var_temp3 = assign40540_e46299;
        var_temp3_dn4 = assign40540_e46299_d_n4;
        var_temp3_dn6 = assign40540_e46299_d_n6;
        var_temp3_dn7 = assign40540_e46299_d_n7;
        var_temp3_dn8 = assign40540_e46299_d_n8;
        var_temp3_dn9 = assign40540_e46299_d_n9;

        let (assign40550_e46317, assign40550_e46317_d_n4, assign40550_e46317_d_n6, assign40550_e46317_d_n7, assign40550_e46317_d_n8, assign40550_e46317_d_n9,) = {
    if (((var_guard1080 != 0.0) && (var_guard1222 == 0.0)) && (var_guard1223 == 0.0)) {
        let assign40550_e46311: f64 = (var_qim__blk1016 + 1e-12);
        let assign40550_e46312: f64 = (assign40550_e46311).ln();
        let assign40550_e46313: f64 = (var_thersg_i * assign40550_e46312);
        let assign40550_e46314: f64 = (assign40550_e46313).exp();
        let assign40550_e46315: f64 = (var_rsg_i * assign40550_e46314);
        (assign40550_e46315, (var_rsg_i * (assign40550_e46314 * (var_thersg_i * (var_qim__blk1016_dn4 / assign40550_e46311)))), (var_rsg_i * (assign40550_e46314 * (var_thersg_i * (var_qim__blk1016_dn6 / assign40550_e46311)))), (var_rsg_i * (assign40550_e46314 * (var_thersg_i * (var_qim__blk1016_dn7 / assign40550_e46311)))), (var_rsg_i * (assign40550_e46314 * (var_thersg_i * (var_qim__blk1016_dn8 / assign40550_e46311)))), (var_rsg_i * (assign40550_e46314 * (var_thersg_i * (var_qim__blk1016_dn9 / assign40550_e46311)))),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign40550_e46317;
        var_temp1_dn4 = assign40550_e46317_d_n4;
        var_temp1_dn6 = assign40550_e46317_d_n6;
        var_temp1_dn7 = assign40550_e46317_d_n7;
        var_temp1_dn8 = assign40550_e46317_d_n8;
        var_temp1_dn9 = assign40550_e46317_d_n9;

        let (assign40560_e46331, assign40560_e46331_d_n4, assign40560_e46331_d_n6, assign40560_e46331_d_n7, assign40560_e46331_d_n8, assign40560_e46331_d_n9,) = {
    if (((var_guard1080 != 0.0) && (var_guard1222 == 0.0)) && (var_guard1223 == 0.0)) {
        let assign40560_e46328: f64 = (1.0 + var_temp1);
        let assign40560_e46329: f64 = (1.0 / assign40560_e46328);
        (assign40560_e46329, (-(var_temp1_dn4 / (assign40560_e46328 * assign40560_e46328))), (-(var_temp1_dn6 / (assign40560_e46328 * assign40560_e46328))), (-(var_temp1_dn7 / (assign40560_e46328 * assign40560_e46328))), (-(var_temp1_dn8 / (assign40560_e46328 * assign40560_e46328))), (-(var_temp1_dn9 / (assign40560_e46328 * assign40560_e46328))),)
    } else {
        (var_temp3, var_temp3_dn4, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9,)
    }
};
        var_temp3 = assign40560_e46331;
        var_temp3_dn4 = assign40560_e46331_d_n4;
        var_temp3_dn6 = assign40560_e46331_d_n6;
        var_temp3_dn7 = assign40560_e46331_d_n7;
        var_temp3_dn8 = assign40560_e46331_d_n8;
        var_temp3_dn9 = assign40560_e46331_d_n9;

        let (assign40570_e46341, assign40570_e46341_d_n4, assign40570_e46341_d_n6, assign40570_e46341_d_n7, assign40570_e46341_d_n8, assign40570_e46341_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40570_e46336: f64 = (var_qim__blk1016 * var_temp3);
        let assign40570_e46338: f64 = (assign40570_e46336 + var_rsig_i);
        let assign40570_e46339: f64 = (var_frscsi__blk964 * assign40570_e46338);
        (assign40570_e46339, ((var_frscsi__blk964_dn4 * assign40570_e46338) + (var_frscsi__blk964 * ((var_qim__blk1016_dn4 * var_temp3) + (var_qim__blk1016 * var_temp3_dn4)))), ((var_frscsi__blk964_dn6 * assign40570_e46338) + (var_frscsi__blk964 * ((var_qim__blk1016_dn6 * var_temp3) + (var_qim__blk1016 * var_temp3_dn6)))), ((var_frscsi__blk964_dn7 * assign40570_e46338) + (var_frscsi__blk964 * ((var_qim__blk1016_dn7 * var_temp3) + (var_qim__blk1016 * var_temp3_dn7)))), ((var_frscsi__blk964_dn8 * assign40570_e46338) + (var_frscsi__blk964 * ((var_qim__blk1016_dn8 * var_temp3) + (var_qim__blk1016 * var_temp3_dn8)))), ((var_frscsi__blk964_dn9 * assign40570_e46338) + (var_frscsi__blk964 * ((var_qim__blk1016_dn9 * var_temp3) + (var_qim__blk1016 * var_temp3_dn9)))),)
    } else {
        (var_grs__blk1040, var_grs__blk1040_dn4, var_grs__blk1040_dn6, var_grs__blk1040_dn7, var_grs__blk1040_dn8, var_grs__blk1040_dn9,)
    }
};
        var_grs__blk1040 = assign40570_e46341;
        var_grs__blk1040_dn4 = assign40570_e46341_d_n4;
        var_grs__blk1040_dn6 = assign40570_e46341_d_n6;
        var_grs__blk1040_dn7 = assign40570_e46341_d_n7;
        var_grs__blk1040_dn8 = assign40570_e46341_d_n8;
        var_grs__blk1040_dn9 = assign40570_e46341_d_n9;

        let (assign40580_e46361, assign40580_e46361_d_n4, assign40580_e46361_d_n6, assign40580_e46361_d_n7, assign40580_e46361_d_n8, assign40580_e46361_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40580_e46347: f64 = (var_fmue * var_eeff1__blk1033);
        let assign40580_e46349: f64 = (assign40580_e46347 + 1e-6);
        let assign40580_e46350: f64 = (assign40580_e46349).ln();
        let assign40580_e46351: f64 = (var_themu_i * assign40580_e46350);
        let assign40580_e46352: f64 = (assign40580_e46351).exp();
        let assign40580_e46353: f64 = (1.0 + assign40580_e46352);
        let assign40580_e46355: f64 = (assign40580_e46353 + var_gcs__blk1039);
        let assign40580_e46358: f64 = (var_betn1_i * var_grs__blk1040);
        let assign40580_e46359: f64 = (assign40580_e46355 + assign40580_e46358);
        (assign40580_e46359, (((assign40580_e46352 * ((var_themu_i_dn4 * assign40580_e46350) + (var_themu_i * (((var_fmue_dn4 * var_eeff1__blk1033) + (var_fmue * var_eeff1__blk1033_dn4)) / assign40580_e46349)))) + var_gcs__blk1039_dn4) + ((var_betn1_i_dn4 * var_grs__blk1040) + (var_betn1_i * var_grs__blk1040_dn4))), (((assign40580_e46352 * ((var_themu_i_dn6 * assign40580_e46350) + (var_themu_i * (((var_fmue_dn6 * var_eeff1__blk1033) + (var_fmue * var_eeff1__blk1033_dn6)) / assign40580_e46349)))) + var_gcs__blk1039_dn6) + ((var_betn1_i_dn6 * var_grs__blk1040) + (var_betn1_i * var_grs__blk1040_dn6))), (((assign40580_e46352 * ((var_themu_i_dn7 * assign40580_e46350) + (var_themu_i * (((var_fmue_dn7 * var_eeff1__blk1033) + (var_fmue * var_eeff1__blk1033_dn7)) / assign40580_e46349)))) + var_gcs__blk1039_dn7) + ((var_betn1_i_dn7 * var_grs__blk1040) + (var_betn1_i * var_grs__blk1040_dn7))), (((assign40580_e46352 * ((var_themu_i_dn8 * assign40580_e46350) + (var_themu_i * (((var_fmue_dn8 * var_eeff1__blk1033) + (var_fmue * var_eeff1__blk1033_dn8)) / assign40580_e46349)))) + var_gcs__blk1039_dn8) + ((var_betn1_i_dn8 * var_grs__blk1040) + (var_betn1_i * var_grs__blk1040_dn8))), (((assign40580_e46352 * ((var_themu_i_dn9 * assign40580_e46350) + (var_themu_i * (((var_fmue_dn9 * var_eeff1__blk1033) + (var_fmue * var_eeff1__blk1033_dn9)) / assign40580_e46349)))) + var_gcs__blk1039_dn9) + ((var_betn1_i_dn9 * var_grs__blk1040) + (var_betn1_i * var_grs__blk1040_dn9))),)
    } else {
        (var_gmob1__blk1041, var_gmob1__blk1041_dn4, var_gmob1__blk1041_dn6, var_gmob1__blk1041_dn7, var_gmob1__blk1041_dn8, var_gmob1__blk1041_dn9,)
    }
};
        var_gmob1__blk1041 = assign40580_e46361;
        var_gmob1__blk1041_dn4 = assign40580_e46361_d_n4;
        var_gmob1__blk1041_dn6 = assign40580_e46361_d_n6;
        var_gmob1__blk1041_dn7 = assign40580_e46361_d_n7;
        var_gmob1__blk1041_dn8 = assign40580_e46361_d_n8;
        var_gmob1__blk1041_dn9 = assign40580_e46361_d_n9;

        *var_c1__blk1035_slot = var_c1__blk1035;
        *var_c1__blk1035_dn4_slot = var_c1__blk1035_dn4;
        *var_c1__blk1035_dn6_slot = var_c1__blk1035_dn6;
        *var_c1__blk1035_dn7_slot = var_c1__blk1035_dn7;
        *var_c1__blk1035_dn8_slot = var_c1__blk1035_dn8;
        *var_c1__blk1035_dn9_slot = var_c1__blk1035_dn9;
        *var_c2__blk1036_slot = var_c2__blk1036;
        *var_c2__blk1036_dn4_slot = var_c2__blk1036_dn4;
        *var_c2__blk1036_dn6_slot = var_c2__blk1036_dn6;
        *var_c2__blk1036_dn7_slot = var_c2__blk1036_dn7;
        *var_c2__blk1036_dn8_slot = var_c2__blk1036_dn8;
        *var_c2__blk1036_dn9_slot = var_c2__blk1036_dn9;
        *var_csum__blk1037_slot = var_csum__blk1037;
        *var_csum__blk1037_dn4_slot = var_csum__blk1037_dn4;
        *var_csum__blk1037_dn6_slot = var_csum__blk1037_dn6;
        *var_csum__blk1037_dn7_slot = var_csum__blk1037_dn7;
        *var_csum__blk1037_dn8_slot = var_csum__blk1037_dn8;
        *var_csum__blk1037_dn9_slot = var_csum__blk1037_dn9;
        *var_ecpl1__blk1031_slot = var_ecpl1__blk1031;
        *var_ecpl1__blk1031_dn4_slot = var_ecpl1__blk1031_dn4;
        *var_ecpl1__blk1031_dn6_slot = var_ecpl1__blk1031_dn6;
        *var_ecpl1__blk1031_dn7_slot = var_ecpl1__blk1031_dn7;
        *var_ecpl1__blk1031_dn8_slot = var_ecpl1__blk1031_dn8;
        *var_ecpl1__blk1031_dn9_slot = var_ecpl1__blk1031_dn9;
        *var_ecpl1d__blk1023_slot = var_ecpl1d__blk1023;
        *var_ecpl1d__blk1023_dn4_slot = var_ecpl1d__blk1023_dn4;
        *var_ecpl1d__blk1023_dn6_slot = var_ecpl1d__blk1023_dn6;
        *var_ecpl1d__blk1023_dn7_slot = var_ecpl1d__blk1023_dn7;
        *var_ecpl1d__blk1023_dn8_slot = var_ecpl1d__blk1023_dn8;
        *var_ecpl1d__blk1023_dn9_slot = var_ecpl1d__blk1023_dn9;
        *var_ecpl2__blk1032_slot = var_ecpl2__blk1032;
        *var_ecpl2__blk1032_dn4_slot = var_ecpl2__blk1032_dn4;
        *var_ecpl2__blk1032_dn6_slot = var_ecpl2__blk1032_dn6;
        *var_ecpl2__blk1032_dn7_slot = var_ecpl2__blk1032_dn7;
        *var_ecpl2__blk1032_dn8_slot = var_ecpl2__blk1032_dn8;
        *var_ecpl2__blk1032_dn9_slot = var_ecpl2__blk1032_dn9;
        *var_ecpl2d__blk1024_slot = var_ecpl2d__blk1024;
        *var_ecpl2d__blk1024_dn4_slot = var_ecpl2d__blk1024_dn4;
        *var_ecpl2d__blk1024_dn6_slot = var_ecpl2d__blk1024_dn6;
        *var_ecpl2d__blk1024_dn7_slot = var_ecpl2d__blk1024_dn7;
        *var_ecpl2d__blk1024_dn8_slot = var_ecpl2d__blk1024_dn8;
        *var_ecpl2d__blk1024_dn9_slot = var_ecpl2d__blk1024_dn9;
        *var_eeff1__blk1033_slot = var_eeff1__blk1033;
        *var_eeff1__blk1033_dn4_slot = var_eeff1__blk1033_dn4;
        *var_eeff1__blk1033_dn6_slot = var_eeff1__blk1033_dn6;
        *var_eeff1__blk1033_dn7_slot = var_eeff1__blk1033_dn7;
        *var_eeff1__blk1033_dn8_slot = var_eeff1__blk1033_dn8;
        *var_eeff1__blk1033_dn9_slot = var_eeff1__blk1033_dn9;
        *var_eeff1d__blk1025_slot = var_eeff1d__blk1025;
        *var_eeff1d__blk1025_dn4_slot = var_eeff1d__blk1025_dn4;
        *var_eeff1d__blk1025_dn6_slot = var_eeff1d__blk1025_dn6;
        *var_eeff1d__blk1025_dn7_slot = var_eeff1d__blk1025_dn7;
        *var_eeff1d__blk1025_dn8_slot = var_eeff1d__blk1025_dn8;
        *var_eeff1d__blk1025_dn9_slot = var_eeff1d__blk1025_dn9;
        *var_eeff2__blk1034_slot = var_eeff2__blk1034;
        *var_eeff2__blk1034_dn4_slot = var_eeff2__blk1034_dn4;
        *var_eeff2__blk1034_dn6_slot = var_eeff2__blk1034_dn6;
        *var_eeff2__blk1034_dn7_slot = var_eeff2__blk1034_dn7;
        *var_eeff2__blk1034_dn8_slot = var_eeff2__blk1034_dn8;
        *var_eeff2__blk1034_dn9_slot = var_eeff2__blk1034_dn9;
        *var_eeff2d__blk1026_slot = var_eeff2d__blk1026;
        *var_eeff2d__blk1026_dn4_slot = var_eeff2d__blk1026_dn4;
        *var_eeff2d__blk1026_dn6_slot = var_eeff2d__blk1026_dn6;
        *var_eeff2d__blk1026_dn7_slot = var_eeff2d__blk1026_dn7;
        *var_eeff2d__blk1026_dn8_slot = var_eeff2d__blk1026_dn8;
        *var_eeff2d__blk1026_dn9_slot = var_eeff2d__blk1026_dn9;
        *var_esurf1__blk1027_slot = var_esurf1__blk1027;
        *var_esurf1__blk1027_dn4_slot = var_esurf1__blk1027_dn4;
        *var_esurf1__blk1027_dn6_slot = var_esurf1__blk1027_dn6;
        *var_esurf1__blk1027_dn7_slot = var_esurf1__blk1027_dn7;
        *var_esurf1__blk1027_dn8_slot = var_esurf1__blk1027_dn8;
        *var_esurf1__blk1027_dn9_slot = var_esurf1__blk1027_dn9;
        *var_esurf2__blk1028_slot = var_esurf2__blk1028;
        *var_esurf2__blk1028_dn4_slot = var_esurf2__blk1028_dn4;
        *var_esurf2__blk1028_dn6_slot = var_esurf2__blk1028_dn6;
        *var_esurf2__blk1028_dn7_slot = var_esurf2__blk1028_dn7;
        *var_esurf2__blk1028_dn8_slot = var_esurf2__blk1028_dn8;
        *var_esurf2__blk1028_dn9_slot = var_esurf2__blk1028_dn9;
        *var_fcor__blk1038_slot = var_fcor__blk1038;
        *var_fcor__blk1038_dn4_slot = var_fcor__blk1038_dn4;
        *var_fcor__blk1038_dn6_slot = var_fcor__blk1038_dn6;
        *var_fcor__blk1038_dn7_slot = var_fcor__blk1038_dn7;
        *var_fcor__blk1038_dn8_slot = var_fcor__blk1038_dn8;
        *var_fcor__blk1038_dn9_slot = var_fcor__blk1038_dn9;
        *var_gcs__blk1039_slot = var_gcs__blk1039;
        *var_gcs__blk1039_dn4_slot = var_gcs__blk1039_dn4;
        *var_gcs__blk1039_dn6_slot = var_gcs__blk1039_dn6;
        *var_gcs__blk1039_dn7_slot = var_gcs__blk1039_dn7;
        *var_gcs__blk1039_dn8_slot = var_gcs__blk1039_dn8;
        *var_gcs__blk1039_dn9_slot = var_gcs__blk1039_dn9;
        *var_gmob1__blk1041_slot = var_gmob1__blk1041;
        *var_gmob1__blk1041_dn4_slot = var_gmob1__blk1041_dn4;
        *var_gmob1__blk1041_dn6_slot = var_gmob1__blk1041_dn6;
        *var_gmob1__blk1041_dn7_slot = var_gmob1__blk1041_dn7;
        *var_gmob1__blk1041_dn8_slot = var_gmob1__blk1041_dn8;
        *var_gmob1__blk1041_dn9_slot = var_gmob1__blk1041_dn9;
        *var_grs__blk1040_slot = var_grs__blk1040;
        *var_grs__blk1040_dn4_slot = var_grs__blk1040_dn4;
        *var_grs__blk1040_dn6_slot = var_grs__blk1040_dn6;
        *var_grs__blk1040_dn7_slot = var_grs__blk1040_dn7;
        *var_grs__blk1040_dn8_slot = var_grs__blk1040_dn8;
        *var_grs__blk1040_dn9_slot = var_grs__blk1040_dn9;
        *var_guard1222_slot = var_guard1222;
        *var_guard1223_slot = var_guard1223;
        *var_qi1m__blk1029_slot = var_qi1m__blk1029;
        *var_qi1m__blk1029_dn4_slot = var_qi1m__blk1029_dn4;
        *var_qi1m__blk1029_dn6_slot = var_qi1m__blk1029_dn6;
        *var_qi1m__blk1029_dn7_slot = var_qi1m__blk1029_dn7;
        *var_qi1m__blk1029_dn8_slot = var_qi1m__blk1029_dn8;
        *var_qi1m__blk1029_dn9_slot = var_qi1m__blk1029_dn9;
        *var_qi2m__blk1030_slot = var_qi2m__blk1030;
        *var_qi2m__blk1030_dn4_slot = var_qi2m__blk1030_dn4;
        *var_qi2m__blk1030_dn6_slot = var_qi2m__blk1030_dn6;
        *var_qi2m__blk1030_dn7_slot = var_qi2m__blk1030_dn7;
        *var_qi2m__blk1030_dn8_slot = var_qi2m__blk1030_dn8;
        *var_qi2m__blk1030_dn9_slot = var_qi2m__blk1030_dn9;
        *var_temp_slot = var_temp;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp3_slot = var_temp3;
        *var_temp3_dn4_slot = var_temp3_dn4;
        *var_temp3_dn6_slot = var_temp3_dn6;
        *var_temp3_dn7_slot = var_temp3_dn7;
        *var_temp3_dn8_slot = var_temp3_dn8;
        *var_temp3_dn9_slot = var_temp3_dn9;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
    }

    pub(super) fn stamp_transient_block_113(
        p: &Parameters,
        var_a1d__blk1011: f64,
        var_a1d__blk1011_dn4: f64,
        var_a1d__blk1011_dn6: f64,
        var_a1d__blk1011_dn7: f64,
        var_a1d__blk1011_dn8: f64,
        var_a1d__blk1011_dn9: f64,
        var_a2d__blk1012: f64,
        var_a2d__blk1012_dn4: f64,
        var_a2d__blk1012_dn6: f64,
        var_a2d__blk1012_dn7: f64,
        var_a2d__blk1012_dn8: f64,
        var_a2d__blk1012_dn9: f64,
        var_alp_loc__blk898: f64,
        var_alpb_i: f64,
        var_betn2_i: f64,
        var_betn2_i_dn4: f64,
        var_betn2_i_dn6: f64,
        var_betn2_i_dn7: f64,
        var_betn2_i_dn8: f64,
        var_betn2_i_dn9: f64,
        var_c1__blk1035: f64,
        var_c1__blk1035_dn4: f64,
        var_c1__blk1035_dn6: f64,
        var_c1__blk1035_dn7: f64,
        var_c1__blk1035_dn8: f64,
        var_c1__blk1035_dn9: f64,
        var_c2__blk1036: f64,
        var_c2__blk1036_dn4: f64,
        var_c2__blk1036_dn6: f64,
        var_c2__blk1036_dn7: f64,
        var_c2__blk1036_dn8: f64,
        var_c2__blk1036_dn9: f64,
        var_csum__blk1037: f64,
        var_csum__blk1037_dn4: f64,
        var_csum__blk1037_dn6: f64,
        var_csum__blk1037_dn7: f64,
        var_csum__blk1037_dn8: f64,
        var_csum__blk1037_dn9: f64,
        var_dxdrift__blk1017: f64,
        var_dxdrift__blk1017_dn4: f64,
        var_dxdrift__blk1017_dn6: f64,
        var_dxdrift__blk1017_dn7: f64,
        var_dxdrift__blk1017_dn8: f64,
        var_dxdrift__blk1017_dn9: f64,
        var_eeff2__blk1034: f64,
        var_eeff2__blk1034_dn4: f64,
        var_eeff2__blk1034_dn6: f64,
        var_eeff2__blk1034_dn7: f64,
        var_eeff2__blk1034_dn8: f64,
        var_eeff2__blk1034_dn9: f64,
        var_esurf1__blk1027: f64,
        var_esurf1__blk1027_dn4: f64,
        var_esurf1__blk1027_dn6: f64,
        var_esurf1__blk1027_dn7: f64,
        var_esurf1__blk1027_dn8: f64,
        var_esurf1__blk1027_dn9: f64,
        var_esurf2__blk1028: f64,
        var_esurf2__blk1028_dn4: f64,
        var_esurf2__blk1028_dn6: f64,
        var_esurf2__blk1028_dn7: f64,
        var_esurf2__blk1028_dn8: f64,
        var_esurf2__blk1028_dn9: f64,
        var_fcor__blk1038: f64,
        var_fcor__blk1038_dn4: f64,
        var_fcor__blk1038_dn6: f64,
        var_fcor__blk1038_dn7: f64,
        var_fcor__blk1038_dn8: f64,
        var_fcor__blk1038_dn9: f64,
        var_fmue: f64,
        var_fmue_dn4: f64,
        var_fmue_dn6: f64,
        var_fmue_dn7: f64,
        var_fmue_dn8: f64,
        var_fmue_dn9: f64,
        var_gcs__blk1039: f64,
        var_gcs__blk1039_dn4: f64,
        var_gcs__blk1039_dn6: f64,
        var_gcs__blk1039_dn7: f64,
        var_gcs__blk1039_dn8: f64,
        var_gcs__blk1039_dn9: f64,
        var_gmob1__blk1041: f64,
        var_gmob1__blk1041_dn4: f64,
        var_gmob1__blk1041_dn6: f64,
        var_gmob1__blk1041_dn7: f64,
        var_gmob1__blk1041_dn8: f64,
        var_gmob1__blk1041_dn9: f64,
        var_grs__blk1040: f64,
        var_grs__blk1040_dn4: f64,
        var_grs__blk1040_dn6: f64,
        var_grs__blk1040_dn7: f64,
        var_grs__blk1040_dn8: f64,
        var_grs__blk1040_dn9: f64,
        var_guard1080: f64,
        var_inv_phit: f64,
        var_inv_phit_dn4: f64,
        var_inv_phit_dn6: f64,
        var_inv_phit_dn7: f64,
        var_inv_phit_dn8: f64,
        var_inv_phit_dn9: f64,
        var_k1__blk932: f64,
        var_k1__blk932_dn4: f64,
        var_k1__blk932_dn6: f64,
        var_k1__blk932_dn7: f64,
        var_k1__blk932_dn8: f64,
        var_k1__blk932_dn9: f64,
        var_k2__blk933: f64,
        var_k2__blk933_dn4: f64,
        var_k2__blk933_dn6: f64,
        var_k2__blk933_dn7: f64,
        var_k2__blk933_dn8: f64,
        var_k2__blk933_dn9: f64,
        var_q1d__blk1001: f64,
        var_q1d__blk1001_dn4: f64,
        var_q1d__blk1001_dn6: f64,
        var_q1d__blk1001_dn7: f64,
        var_q1d__blk1001_dn8: f64,
        var_q1d__blk1001_dn9: f64,
        var_q2d__blk1002: f64,
        var_q2d__blk1002_dn4: f64,
        var_q2d__blk1002_dn6: f64,
        var_q2d__blk1002_dn7: f64,
        var_q2d__blk1002_dn8: f64,
        var_q2d__blk1002_dn9: f64,
        var_qi2m__blk1030: f64,
        var_qi2m__blk1030_dn4: f64,
        var_qi2m__blk1030_dn6: f64,
        var_qi2m__blk1030_dn7: f64,
        var_qi2m__blk1030_dn8: f64,
        var_qi2m__blk1030_dn9: f64,
        var_qid__blk1003: f64,
        var_qim__blk1016: f64,
        var_qim__blk1016_dn4: f64,
        var_qim__blk1016_dn6: f64,
        var_qim__blk1016_dn7: f64,
        var_qim__blk1016_dn8: f64,
        var_qim__blk1016_dn9: f64,
        var_qis__blk938: f64,
        var_qq: f64,
        var_qq_dn4: f64,
        var_qq_dn6: f64,
        var_qq_dn7: f64,
        var_qq_dn8: f64,
        var_qq_dn9: f64,
        var_sat_phit_loc__blk896: f64,
        var_sat_phit_loc__blk896_dn4: f64,
        var_sat_phit_loc__blk896_dn6: f64,
        var_sat_phit_loc__blk896_dn7: f64,
        var_sat_phit_loc__blk896_dn8: f64,
        var_sat_phit_loc__blk896_dn9: f64,
        var_themu_i: f64,
        var_themu_i_dn4: f64,
        var_themu_i_dn6: f64,
        var_themu_i_dn7: f64,
        var_themu_i_dn8: f64,
        var_themu_i_dn9: f64,
        var_thesat1_i: f64,
        var_thesat2_i: f64,
        var_tox1fact__blk913: f64,
        var_tox1fact__blk913_dn4: f64,
        var_tox1fact__blk913_dn6: f64,
        var_tox1fact__blk913_dn7: f64,
        var_tox1fact__blk913_dn8: f64,
        var_tox1fact__blk913_dn9: f64,
        var_tox2fact__blk914: f64,
        var_tox2fact__blk914_dn4: f64,
        var_tox2fact__blk914_dn6: f64,
        var_tox2fact__blk914_dn7: f64,
        var_tox2fact__blk914_dn8: f64,
        var_tox2fact__blk914_dn9: f64,
        var_vp_i: f64,
        var_vpg_i: f64,
        var_xd: f64,
        var_xd_dn4: f64,
        var_xd_dn6: f64,
        var_xd_dn7: f64,
        var_xd_dn8: f64,
        var_xd_dn9: f64,
        var_xdeff__blk1000: f64,
        var_xdeff__blk1000_dn4: f64,
        var_xdeff__blk1000_dn6: f64,
        var_xdeff__blk1000_dn7: f64,
        var_xdeff__blk1000_dn8: f64,
        var_xdeff__blk1000_dn9: f64,
        var_dl_l__blk1047_slot: &mut f64,
        var_dl_l__blk1047_dn4_slot: &mut f64,
        var_dl_l__blk1047_dn6_slot: &mut f64,
        var_dl_l__blk1047_dn7_slot: &mut f64,
        var_dl_l__blk1047_dn8_slot: &mut f64,
        var_dl_l__blk1047_dn9_slot: &mut f64,
        var_dl_l_fact__blk1046_slot: &mut f64,
        var_dl_l_fact__blk1046_dn4_slot: &mut f64,
        var_dl_l_fact__blk1046_dn6_slot: &mut f64,
        var_dl_l_fact__blk1046_dn7_slot: &mut f64,
        var_dl_l_fact__blk1046_dn8_slot: &mut f64,
        var_dl_l_fact__blk1046_dn9_slot: &mut f64,
        var_gdl__blk1048_slot: &mut f64,
        var_gdl__blk1048_dn4_slot: &mut f64,
        var_gdl__blk1048_dn6_slot: &mut f64,
        var_gdl__blk1048_dn7_slot: &mut f64,
        var_gdl__blk1048_dn8_slot: &mut f64,
        var_gdl__blk1048_dn9_slot: &mut f64,
        var_ggamma__blk1049_slot: &mut f64,
        var_ggamma__blk1049_dn4_slot: &mut f64,
        var_ggamma__blk1049_dn6_slot: &mut f64,
        var_ggamma__blk1049_dn7_slot: &mut f64,
        var_ggamma__blk1049_dn8_slot: &mut f64,
        var_ggamma__blk1049_dn9_slot: &mut f64,
        var_gmob2__blk1042_slot: &mut f64,
        var_gmob2__blk1042_dn4_slot: &mut f64,
        var_gmob2__blk1042_dn6_slot: &mut f64,
        var_gmob2__blk1042_dn7_slot: &mut f64,
        var_gmob2__blk1042_dn8_slot: &mut f64,
        var_gmob2__blk1042_dn9_slot: &mut f64,
        var_gmob__blk1043_slot: &mut f64,
        var_gmob__blk1043_dn4_slot: &mut f64,
        var_gmob__blk1043_dn6_slot: &mut f64,
        var_gmob__blk1043_dn7_slot: &mut f64,
        var_gmob__blk1043_dn8_slot: &mut f64,
        var_gmob__blk1043_dn9_slot: &mut f64,
        var_guard1224_slot: &mut f64,
        var_guard1225_slot: &mut f64,
        var_guard1226_slot: &mut f64,
        var_guard1227_slot: &mut f64,
        var_guard1228_slot: &mut f64,
        var_guard1229_slot: &mut f64,
        var_guard1230_slot: &mut f64,
        var_hsat__blk1053_slot: &mut f64,
        var_hsat__blk1053_dn4_slot: &mut f64,
        var_hsat__blk1053_dn6_slot: &mut f64,
        var_hsat__blk1053_dn7_slot: &mut f64,
        var_hsat__blk1053_dn8_slot: &mut f64,
        var_hsat__blk1053_dn9_slot: &mut f64,
        var_inv_qimstar1__blk1044_slot: &mut f64,
        var_inv_qimstar1__blk1044_dn4_slot: &mut f64,
        var_inv_qimstar1__blk1044_dn6_slot: &mut f64,
        var_inv_qimstar1__blk1044_dn7_slot: &mut f64,
        var_inv_qimstar1__blk1044_dn8_slot: &mut f64,
        var_inv_qimstar1__blk1044_dn9_slot: &mut f64,
        var_qmfact1__blk1054_slot: &mut f64,
        var_qmfact1__blk1054_dn4_slot: &mut f64,
        var_qmfact1__blk1054_dn6_slot: &mut f64,
        var_qmfact1__blk1054_dn7_slot: &mut f64,
        var_qmfact1__blk1054_dn8_slot: &mut f64,
        var_qmfact1__blk1054_dn9_slot: &mut f64,
        var_qmfact2__blk1055_slot: &mut f64,
        var_qmfact2__blk1055_dn4_slot: &mut f64,
        var_qmfact2__blk1055_dn6_slot: &mut f64,
        var_qmfact2__blk1055_dn7_slot: &mut f64,
        var_qmfact2__blk1055_dn8_slot: &mut f64,
        var_qmfact2__blk1055_dn9_slot: &mut f64,
        var_r1__blk1045_slot: &mut f64,
        var_r1__blk1045_dn4_slot: &mut f64,
        var_r1__blk1045_dn6_slot: &mut f64,
        var_r1__blk1045_dn7_slot: &mut f64,
        var_r1__blk1045_dn8_slot: &mut f64,
        var_r1__blk1045_dn9_slot: &mut f64,
        var_sat_fact1__blk977_slot: &mut f64,
        var_sat_fact1__blk977_dn4_slot: &mut f64,
        var_sat_fact1__blk977_dn6_slot: &mut f64,
        var_sat_fact1__blk977_dn7_slot: &mut f64,
        var_sat_fact1__blk977_dn8_slot: &mut f64,
        var_sat_fact1__blk977_dn9_slot: &mut f64,
        var_sat_fact2__blk979_slot: &mut f64,
        var_sat_fact2__blk979_dn4_slot: &mut f64,
        var_sat_fact2__blk979_dn6_slot: &mut f64,
        var_sat_fact2__blk979_dn7_slot: &mut f64,
        var_sat_fact2__blk979_dn8_slot: &mut f64,
        var_sat_fact2__blk979_dn9_slot: &mut f64,
        var_sqrt_zsat__blk1050_slot: &mut f64,
        var_sqrt_zsat__blk1050_dn4_slot: &mut f64,
        var_sqrt_zsat__blk1050_dn6_slot: &mut f64,
        var_sqrt_zsat__blk1050_dn7_slot: &mut f64,
        var_sqrt_zsat__blk1050_dn8_slot: &mut f64,
        var_sqrt_zsat__blk1050_dn9_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_vsat_fact__blk1052_slot: &mut f64,
        var_vsat_fact__blk1052_dn4_slot: &mut f64,
        var_vsat_fact__blk1052_dn6_slot: &mut f64,
        var_vsat_fact__blk1052_dn7_slot: &mut f64,
        var_vsat_fact__blk1052_dn8_slot: &mut f64,
        var_vsat_fact__blk1052_dn9_slot: &mut f64,
        var_wsat1__blk976_slot: &mut f64,
        var_wsat1__blk976_dn4_slot: &mut f64,
        var_wsat1__blk976_dn6_slot: &mut f64,
        var_wsat1__blk976_dn7_slot: &mut f64,
        var_wsat1__blk976_dn8_slot: &mut f64,
        var_wsat1__blk976_dn9_slot: &mut f64,
        var_wsat2__blk978_slot: &mut f64,
        var_wsat2__blk978_dn4_slot: &mut f64,
        var_wsat2__blk978_dn6_slot: &mut f64,
        var_wsat2__blk978_dn7_slot: &mut f64,
        var_wsat2__blk978_dn8_slot: &mut f64,
        var_wsat2__blk978_dn9_slot: &mut f64,
        var_zsat__blk1051_slot: &mut f64,
        var_zsat__blk1051_dn4_slot: &mut f64,
        var_zsat__blk1051_dn6_slot: &mut f64,
        var_zsat__blk1051_dn7_slot: &mut f64,
        var_zsat__blk1051_dn8_slot: &mut f64,
        var_zsat__blk1051_dn9_slot: &mut f64,
    ) {
        let mut var_dl_l__blk1047: f64 = *var_dl_l__blk1047_slot;
        let mut var_dl_l__blk1047_dn4: f64 = *var_dl_l__blk1047_dn4_slot;
        let mut var_dl_l__blk1047_dn6: f64 = *var_dl_l__blk1047_dn6_slot;
        let mut var_dl_l__blk1047_dn7: f64 = *var_dl_l__blk1047_dn7_slot;
        let mut var_dl_l__blk1047_dn8: f64 = *var_dl_l__blk1047_dn8_slot;
        let mut var_dl_l__blk1047_dn9: f64 = *var_dl_l__blk1047_dn9_slot;
        let mut var_dl_l_fact__blk1046: f64 = *var_dl_l_fact__blk1046_slot;
        let mut var_dl_l_fact__blk1046_dn4: f64 = *var_dl_l_fact__blk1046_dn4_slot;
        let mut var_dl_l_fact__blk1046_dn6: f64 = *var_dl_l_fact__blk1046_dn6_slot;
        let mut var_dl_l_fact__blk1046_dn7: f64 = *var_dl_l_fact__blk1046_dn7_slot;
        let mut var_dl_l_fact__blk1046_dn8: f64 = *var_dl_l_fact__blk1046_dn8_slot;
        let mut var_dl_l_fact__blk1046_dn9: f64 = *var_dl_l_fact__blk1046_dn9_slot;
        let mut var_gdl__blk1048: f64 = *var_gdl__blk1048_slot;
        let mut var_gdl__blk1048_dn4: f64 = *var_gdl__blk1048_dn4_slot;
        let mut var_gdl__blk1048_dn6: f64 = *var_gdl__blk1048_dn6_slot;
        let mut var_gdl__blk1048_dn7: f64 = *var_gdl__blk1048_dn7_slot;
        let mut var_gdl__blk1048_dn8: f64 = *var_gdl__blk1048_dn8_slot;
        let mut var_gdl__blk1048_dn9: f64 = *var_gdl__blk1048_dn9_slot;
        let mut var_ggamma__blk1049: f64 = *var_ggamma__blk1049_slot;
        let mut var_ggamma__blk1049_dn4: f64 = *var_ggamma__blk1049_dn4_slot;
        let mut var_ggamma__blk1049_dn6: f64 = *var_ggamma__blk1049_dn6_slot;
        let mut var_ggamma__blk1049_dn7: f64 = *var_ggamma__blk1049_dn7_slot;
        let mut var_ggamma__blk1049_dn8: f64 = *var_ggamma__blk1049_dn8_slot;
        let mut var_ggamma__blk1049_dn9: f64 = *var_ggamma__blk1049_dn9_slot;
        let mut var_gmob2__blk1042: f64 = *var_gmob2__blk1042_slot;
        let mut var_gmob2__blk1042_dn4: f64 = *var_gmob2__blk1042_dn4_slot;
        let mut var_gmob2__blk1042_dn6: f64 = *var_gmob2__blk1042_dn6_slot;
        let mut var_gmob2__blk1042_dn7: f64 = *var_gmob2__blk1042_dn7_slot;
        let mut var_gmob2__blk1042_dn8: f64 = *var_gmob2__blk1042_dn8_slot;
        let mut var_gmob2__blk1042_dn9: f64 = *var_gmob2__blk1042_dn9_slot;
        let mut var_gmob__blk1043: f64 = *var_gmob__blk1043_slot;
        let mut var_gmob__blk1043_dn4: f64 = *var_gmob__blk1043_dn4_slot;
        let mut var_gmob__blk1043_dn6: f64 = *var_gmob__blk1043_dn6_slot;
        let mut var_gmob__blk1043_dn7: f64 = *var_gmob__blk1043_dn7_slot;
        let mut var_gmob__blk1043_dn8: f64 = *var_gmob__blk1043_dn8_slot;
        let mut var_gmob__blk1043_dn9: f64 = *var_gmob__blk1043_dn9_slot;
        let mut var_guard1224: f64 = *var_guard1224_slot;
        let mut var_guard1225: f64 = *var_guard1225_slot;
        let mut var_guard1226: f64 = *var_guard1226_slot;
        let mut var_guard1227: f64 = *var_guard1227_slot;
        let mut var_guard1228: f64 = *var_guard1228_slot;
        let mut var_guard1229: f64 = *var_guard1229_slot;
        let mut var_guard1230: f64 = *var_guard1230_slot;
        let mut var_hsat__blk1053: f64 = *var_hsat__blk1053_slot;
        let mut var_hsat__blk1053_dn4: f64 = *var_hsat__blk1053_dn4_slot;
        let mut var_hsat__blk1053_dn6: f64 = *var_hsat__blk1053_dn6_slot;
        let mut var_hsat__blk1053_dn7: f64 = *var_hsat__blk1053_dn7_slot;
        let mut var_hsat__blk1053_dn8: f64 = *var_hsat__blk1053_dn8_slot;
        let mut var_hsat__blk1053_dn9: f64 = *var_hsat__blk1053_dn9_slot;
        let mut var_inv_qimstar1__blk1044: f64 = *var_inv_qimstar1__blk1044_slot;
        let mut var_inv_qimstar1__blk1044_dn4: f64 = *var_inv_qimstar1__blk1044_dn4_slot;
        let mut var_inv_qimstar1__blk1044_dn6: f64 = *var_inv_qimstar1__blk1044_dn6_slot;
        let mut var_inv_qimstar1__blk1044_dn7: f64 = *var_inv_qimstar1__blk1044_dn7_slot;
        let mut var_inv_qimstar1__blk1044_dn8: f64 = *var_inv_qimstar1__blk1044_dn8_slot;
        let mut var_inv_qimstar1__blk1044_dn9: f64 = *var_inv_qimstar1__blk1044_dn9_slot;
        let mut var_qmfact1__blk1054: f64 = *var_qmfact1__blk1054_slot;
        let mut var_qmfact1__blk1054_dn4: f64 = *var_qmfact1__blk1054_dn4_slot;
        let mut var_qmfact1__blk1054_dn6: f64 = *var_qmfact1__blk1054_dn6_slot;
        let mut var_qmfact1__blk1054_dn7: f64 = *var_qmfact1__blk1054_dn7_slot;
        let mut var_qmfact1__blk1054_dn8: f64 = *var_qmfact1__blk1054_dn8_slot;
        let mut var_qmfact1__blk1054_dn9: f64 = *var_qmfact1__blk1054_dn9_slot;
        let mut var_qmfact2__blk1055: f64 = *var_qmfact2__blk1055_slot;
        let mut var_qmfact2__blk1055_dn4: f64 = *var_qmfact2__blk1055_dn4_slot;
        let mut var_qmfact2__blk1055_dn6: f64 = *var_qmfact2__blk1055_dn6_slot;
        let mut var_qmfact2__blk1055_dn7: f64 = *var_qmfact2__blk1055_dn7_slot;
        let mut var_qmfact2__blk1055_dn8: f64 = *var_qmfact2__blk1055_dn8_slot;
        let mut var_qmfact2__blk1055_dn9: f64 = *var_qmfact2__blk1055_dn9_slot;
        let mut var_r1__blk1045: f64 = *var_r1__blk1045_slot;
        let mut var_r1__blk1045_dn4: f64 = *var_r1__blk1045_dn4_slot;
        let mut var_r1__blk1045_dn6: f64 = *var_r1__blk1045_dn6_slot;
        let mut var_r1__blk1045_dn7: f64 = *var_r1__blk1045_dn7_slot;
        let mut var_r1__blk1045_dn8: f64 = *var_r1__blk1045_dn8_slot;
        let mut var_r1__blk1045_dn9: f64 = *var_r1__blk1045_dn9_slot;
        let mut var_sat_fact1__blk977: f64 = *var_sat_fact1__blk977_slot;
        let mut var_sat_fact1__blk977_dn4: f64 = *var_sat_fact1__blk977_dn4_slot;
        let mut var_sat_fact1__blk977_dn6: f64 = *var_sat_fact1__blk977_dn6_slot;
        let mut var_sat_fact1__blk977_dn7: f64 = *var_sat_fact1__blk977_dn7_slot;
        let mut var_sat_fact1__blk977_dn8: f64 = *var_sat_fact1__blk977_dn8_slot;
        let mut var_sat_fact1__blk977_dn9: f64 = *var_sat_fact1__blk977_dn9_slot;
        let mut var_sat_fact2__blk979: f64 = *var_sat_fact2__blk979_slot;
        let mut var_sat_fact2__blk979_dn4: f64 = *var_sat_fact2__blk979_dn4_slot;
        let mut var_sat_fact2__blk979_dn6: f64 = *var_sat_fact2__blk979_dn6_slot;
        let mut var_sat_fact2__blk979_dn7: f64 = *var_sat_fact2__blk979_dn7_slot;
        let mut var_sat_fact2__blk979_dn8: f64 = *var_sat_fact2__blk979_dn8_slot;
        let mut var_sat_fact2__blk979_dn9: f64 = *var_sat_fact2__blk979_dn9_slot;
        let mut var_sqrt_zsat__blk1050: f64 = *var_sqrt_zsat__blk1050_slot;
        let mut var_sqrt_zsat__blk1050_dn4: f64 = *var_sqrt_zsat__blk1050_dn4_slot;
        let mut var_sqrt_zsat__blk1050_dn6: f64 = *var_sqrt_zsat__blk1050_dn6_slot;
        let mut var_sqrt_zsat__blk1050_dn7: f64 = *var_sqrt_zsat__blk1050_dn7_slot;
        let mut var_sqrt_zsat__blk1050_dn8: f64 = *var_sqrt_zsat__blk1050_dn8_slot;
        let mut var_sqrt_zsat__blk1050_dn9: f64 = *var_sqrt_zsat__blk1050_dn9_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_vsat_fact__blk1052: f64 = *var_vsat_fact__blk1052_slot;
        let mut var_vsat_fact__blk1052_dn4: f64 = *var_vsat_fact__blk1052_dn4_slot;
        let mut var_vsat_fact__blk1052_dn6: f64 = *var_vsat_fact__blk1052_dn6_slot;
        let mut var_vsat_fact__blk1052_dn7: f64 = *var_vsat_fact__blk1052_dn7_slot;
        let mut var_vsat_fact__blk1052_dn8: f64 = *var_vsat_fact__blk1052_dn8_slot;
        let mut var_vsat_fact__blk1052_dn9: f64 = *var_vsat_fact__blk1052_dn9_slot;
        let mut var_wsat1__blk976: f64 = *var_wsat1__blk976_slot;
        let mut var_wsat1__blk976_dn4: f64 = *var_wsat1__blk976_dn4_slot;
        let mut var_wsat1__blk976_dn6: f64 = *var_wsat1__blk976_dn6_slot;
        let mut var_wsat1__blk976_dn7: f64 = *var_wsat1__blk976_dn7_slot;
        let mut var_wsat1__blk976_dn8: f64 = *var_wsat1__blk976_dn8_slot;
        let mut var_wsat1__blk976_dn9: f64 = *var_wsat1__blk976_dn9_slot;
        let mut var_wsat2__blk978: f64 = *var_wsat2__blk978_slot;
        let mut var_wsat2__blk978_dn4: f64 = *var_wsat2__blk978_dn4_slot;
        let mut var_wsat2__blk978_dn6: f64 = *var_wsat2__blk978_dn6_slot;
        let mut var_wsat2__blk978_dn7: f64 = *var_wsat2__blk978_dn7_slot;
        let mut var_wsat2__blk978_dn8: f64 = *var_wsat2__blk978_dn8_slot;
        let mut var_wsat2__blk978_dn9: f64 = *var_wsat2__blk978_dn9_slot;
        let mut var_zsat__blk1051: f64 = *var_zsat__blk1051_slot;
        let mut var_zsat__blk1051_dn4: f64 = *var_zsat__blk1051_dn4_slot;
        let mut var_zsat__blk1051_dn6: f64 = *var_zsat__blk1051_dn6_slot;
        let mut var_zsat__blk1051_dn7: f64 = *var_zsat__blk1051_dn7_slot;
        let mut var_zsat__blk1051_dn8: f64 = *var_zsat__blk1051_dn8_slot;
        let mut var_zsat__blk1051_dn9: f64 = *var_zsat__blk1051_dn9_slot;

        let (assign40590_e46381, assign40590_e46381_d_n4, assign40590_e46381_d_n6, assign40590_e46381_d_n7, assign40590_e46381_d_n8, assign40590_e46381_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40590_e46367: f64 = (var_fmue * var_eeff2__blk1034);
        let assign40590_e46369: f64 = (assign40590_e46367 + 1e-6);
        let assign40590_e46370: f64 = (assign40590_e46369).ln();
        let assign40590_e46371: f64 = (var_themu_i * assign40590_e46370);
        let assign40590_e46372: f64 = (assign40590_e46371).exp();
        let assign40590_e46373: f64 = (1.0 + assign40590_e46372);
        let assign40590_e46375: f64 = (assign40590_e46373 + var_gcs__blk1039);
        let assign40590_e46378: f64 = (var_betn2_i * var_grs__blk1040);
        let assign40590_e46379: f64 = (assign40590_e46375 + assign40590_e46378);
        (assign40590_e46379, (((assign40590_e46372 * ((var_themu_i_dn4 * assign40590_e46370) + (var_themu_i * (((var_fmue_dn4 * var_eeff2__blk1034) + (var_fmue * var_eeff2__blk1034_dn4)) / assign40590_e46369)))) + var_gcs__blk1039_dn4) + ((var_betn2_i_dn4 * var_grs__blk1040) + (var_betn2_i * var_grs__blk1040_dn4))), (((assign40590_e46372 * ((var_themu_i_dn6 * assign40590_e46370) + (var_themu_i * (((var_fmue_dn6 * var_eeff2__blk1034) + (var_fmue * var_eeff2__blk1034_dn6)) / assign40590_e46369)))) + var_gcs__blk1039_dn6) + ((var_betn2_i_dn6 * var_grs__blk1040) + (var_betn2_i * var_grs__blk1040_dn6))), (((assign40590_e46372 * ((var_themu_i_dn7 * assign40590_e46370) + (var_themu_i * (((var_fmue_dn7 * var_eeff2__blk1034) + (var_fmue * var_eeff2__blk1034_dn7)) / assign40590_e46369)))) + var_gcs__blk1039_dn7) + ((var_betn2_i_dn7 * var_grs__blk1040) + (var_betn2_i * var_grs__blk1040_dn7))), (((assign40590_e46372 * ((var_themu_i_dn8 * assign40590_e46370) + (var_themu_i * (((var_fmue_dn8 * var_eeff2__blk1034) + (var_fmue * var_eeff2__blk1034_dn8)) / assign40590_e46369)))) + var_gcs__blk1039_dn8) + ((var_betn2_i_dn8 * var_grs__blk1040) + (var_betn2_i * var_grs__blk1040_dn8))), (((assign40590_e46372 * ((var_themu_i_dn9 * assign40590_e46370) + (var_themu_i * (((var_fmue_dn9 * var_eeff2__blk1034) + (var_fmue * var_eeff2__blk1034_dn9)) / assign40590_e46369)))) + var_gcs__blk1039_dn9) + ((var_betn2_i_dn9 * var_grs__blk1040) + (var_betn2_i * var_grs__blk1040_dn9))),)
    } else {
        (var_gmob2__blk1042, var_gmob2__blk1042_dn4, var_gmob2__blk1042_dn6, var_gmob2__blk1042_dn7, var_gmob2__blk1042_dn8, var_gmob2__blk1042_dn9,)
    }
};
        var_gmob2__blk1042 = assign40590_e46381;
        var_gmob2__blk1042_dn4 = assign40590_e46381_d_n4;
        var_gmob2__blk1042_dn6 = assign40590_e46381_d_n6;
        var_gmob2__blk1042_dn7 = assign40590_e46381_d_n7;
        var_gmob2__blk1042_dn8 = assign40590_e46381_d_n8;
        var_gmob2__blk1042_dn9 = assign40590_e46381_d_n9;

        let (assign40600_e46395, assign40600_e46395_d_n4, assign40600_e46395_d_n6, assign40600_e46395_d_n7, assign40600_e46395_d_n8, assign40600_e46395_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40600_e46385: f64 = (var_fcor__blk1038 * var_csum__blk1037);
        let assign40600_e46388: f64 = (var_c1__blk1035 / var_gmob1__blk1041);
        let assign40600_e46391: f64 = (var_c2__blk1036 / var_gmob2__blk1042);
        let assign40600_e46392: f64 = (assign40600_e46388 + assign40600_e46391);
        let assign40600_e46393: f64 = (assign40600_e46385 / assign40600_e46392);
        (assign40600_e46393, (((((var_fcor__blk1038_dn4 * var_csum__blk1037) + (var_fcor__blk1038 * var_csum__blk1037_dn4)) * assign40600_e46392) - (assign40600_e46385 * ((((var_c1__blk1035_dn4 * var_gmob1__blk1041) - (var_c1__blk1035 * var_gmob1__blk1041_dn4)) / (var_gmob1__blk1041 * var_gmob1__blk1041)) + (((var_c2__blk1036_dn4 * var_gmob2__blk1042) - (var_c2__blk1036 * var_gmob2__blk1042_dn4)) / (var_gmob2__blk1042 * var_gmob2__blk1042))))) / (assign40600_e46392 * assign40600_e46392)), (((((var_fcor__blk1038_dn6 * var_csum__blk1037) + (var_fcor__blk1038 * var_csum__blk1037_dn6)) * assign40600_e46392) - (assign40600_e46385 * ((((var_c1__blk1035_dn6 * var_gmob1__blk1041) - (var_c1__blk1035 * var_gmob1__blk1041_dn6)) / (var_gmob1__blk1041 * var_gmob1__blk1041)) + (((var_c2__blk1036_dn6 * var_gmob2__blk1042) - (var_c2__blk1036 * var_gmob2__blk1042_dn6)) / (var_gmob2__blk1042 * var_gmob2__blk1042))))) / (assign40600_e46392 * assign40600_e46392)), (((((var_fcor__blk1038_dn7 * var_csum__blk1037) + (var_fcor__blk1038 * var_csum__blk1037_dn7)) * assign40600_e46392) - (assign40600_e46385 * ((((var_c1__blk1035_dn7 * var_gmob1__blk1041) - (var_c1__blk1035 * var_gmob1__blk1041_dn7)) / (var_gmob1__blk1041 * var_gmob1__blk1041)) + (((var_c2__blk1036_dn7 * var_gmob2__blk1042) - (var_c2__blk1036 * var_gmob2__blk1042_dn7)) / (var_gmob2__blk1042 * var_gmob2__blk1042))))) / (assign40600_e46392 * assign40600_e46392)), (((((var_fcor__blk1038_dn8 * var_csum__blk1037) + (var_fcor__blk1038 * var_csum__blk1037_dn8)) * assign40600_e46392) - (assign40600_e46385 * ((((var_c1__blk1035_dn8 * var_gmob1__blk1041) - (var_c1__blk1035 * var_gmob1__blk1041_dn8)) / (var_gmob1__blk1041 * var_gmob1__blk1041)) + (((var_c2__blk1036_dn8 * var_gmob2__blk1042) - (var_c2__blk1036 * var_gmob2__blk1042_dn8)) / (var_gmob2__blk1042 * var_gmob2__blk1042))))) / (assign40600_e46392 * assign40600_e46392)), (((((var_fcor__blk1038_dn9 * var_csum__blk1037) + (var_fcor__blk1038 * var_csum__blk1037_dn9)) * assign40600_e46392) - (assign40600_e46385 * ((((var_c1__blk1035_dn9 * var_gmob1__blk1041) - (var_c1__blk1035 * var_gmob1__blk1041_dn9)) / (var_gmob1__blk1041 * var_gmob1__blk1041)) + (((var_c2__blk1036_dn9 * var_gmob2__blk1042) - (var_c2__blk1036 * var_gmob2__blk1042_dn9)) / (var_gmob2__blk1042 * var_gmob2__blk1042))))) / (assign40600_e46392 * assign40600_e46392)),)
    } else {
        (var_gmob__blk1043, var_gmob__blk1043_dn4, var_gmob__blk1043_dn6, var_gmob__blk1043_dn7, var_gmob__blk1043_dn8, var_gmob__blk1043_dn9,)
    }
};
        var_gmob__blk1043 = assign40600_e46395;
        var_gmob__blk1043_dn4 = assign40600_e46395_d_n4;
        var_gmob__blk1043_dn6 = assign40600_e46395_d_n6;
        var_gmob__blk1043_dn7 = assign40600_e46395_d_n7;
        var_gmob__blk1043_dn8 = assign40600_e46395_d_n8;
        var_gmob__blk1043_dn9 = assign40600_e46395_d_n9;

        let (assign40610_e46403, assign40610_e46403_d_n4, assign40610_e46403_d_n6, assign40610_e46403_d_n7, assign40610_e46403_d_n8, assign40610_e46403_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40610_e46400: f64 = (4.0 + var_qim__blk1016);
        let assign40610_e46401: f64 = (1.0 / assign40610_e46400);
        (assign40610_e46401, (-(var_qim__blk1016_dn4 / (assign40610_e46400 * assign40610_e46400))), (-(var_qim__blk1016_dn6 / (assign40610_e46400 * assign40610_e46400))), (-(var_qim__blk1016_dn7 / (assign40610_e46400 * assign40610_e46400))), (-(var_qim__blk1016_dn8 / (assign40610_e46400 * assign40610_e46400))), (-(var_qim__blk1016_dn9 / (assign40610_e46400 * assign40610_e46400))),)
    } else {
        (var_inv_qimstar1__blk1044, var_inv_qimstar1__blk1044_dn4, var_inv_qimstar1__blk1044_dn6, var_inv_qimstar1__blk1044_dn7, var_inv_qimstar1__blk1044_dn8, var_inv_qimstar1__blk1044_dn9,)
    }
};
        var_inv_qimstar1__blk1044 = assign40610_e46403;
        var_inv_qimstar1__blk1044_dn4 = assign40610_e46403_d_n4;
        var_inv_qimstar1__blk1044_dn6 = assign40610_e46403_d_n6;
        var_inv_qimstar1__blk1044_dn7 = assign40610_e46403_d_n7;
        var_inv_qimstar1__blk1044_dn8 = assign40610_e46403_d_n8;
        var_inv_qimstar1__blk1044_dn9 = assign40610_e46403_d_n9;

        let assign40620_e46406: f64 = if var_alpb_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1224 = assign40620_e46406;

        let (assign40630_e46418, assign40630_e46418_d_n4, assign40630_e46418_d_n6, assign40630_e46418_d_n7, assign40630_e46418_d_n8, assign40630_e46418_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1224 != 0.0)) {
        let assign40630_e46414: f64 = (var_alpb_i * var_qi2m__blk1030);
        let assign40630_e46415: f64 = (1.0 + assign40630_e46414);
        let assign40630_e46416: f64 = (1.0 / assign40630_e46415);
        (assign40630_e46416, (-((var_alpb_i * var_qi2m__blk1030_dn4) / (assign40630_e46415 * assign40630_e46415))), (-((var_alpb_i * var_qi2m__blk1030_dn6) / (assign40630_e46415 * assign40630_e46415))), (-((var_alpb_i * var_qi2m__blk1030_dn7) / (assign40630_e46415 * assign40630_e46415))), (-((var_alpb_i * var_qi2m__blk1030_dn8) / (assign40630_e46415 * assign40630_e46415))), (-((var_alpb_i * var_qi2m__blk1030_dn9) / (assign40630_e46415 * assign40630_e46415))),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign40630_e46418;
        var_temp_dn4 = assign40630_e46418_d_n4;
        var_temp_dn6 = assign40630_e46418_d_n6;
        var_temp_dn7 = assign40630_e46418_d_n7;
        var_temp_dn8 = assign40630_e46418_d_n8;
        var_temp_dn9 = assign40630_e46418_d_n9;

        let (assign40640_e46429, assign40640_e46429_d_n4, assign40640_e46429_d_n6, assign40640_e46429_d_n7, assign40640_e46429_d_n8, assign40640_e46429_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1224 == 0.0)) {
        let assign40640_e46426: f64 = (var_alpb_i * var_qi2m__blk1030);
        let assign40640_e46427: f64 = (1.0 - assign40640_e46426);
        (assign40640_e46427, (-(var_alpb_i * var_qi2m__blk1030_dn4)), (-(var_alpb_i * var_qi2m__blk1030_dn6)), (-(var_alpb_i * var_qi2m__blk1030_dn7)), (-(var_alpb_i * var_qi2m__blk1030_dn8)), (-(var_alpb_i * var_qi2m__blk1030_dn9)),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign40640_e46429;
        var_temp_dn4 = assign40640_e46429_d_n4;
        var_temp_dn6 = assign40640_e46429_d_n6;
        var_temp_dn7 = assign40640_e46429_d_n7;
        var_temp_dn8 = assign40640_e46429_d_n8;
        var_temp_dn9 = assign40640_e46429_d_n9;

        let (assign40650_e46437, assign40650_e46437_d_n4, assign40650_e46437_d_n6, assign40650_e46437_d_n7, assign40650_e46437_d_n8, assign40650_e46437_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40650_e46433: f64 = (var_qim__blk1016 * var_inv_qimstar1__blk1044);
        let assign40650_e46435: f64 = (assign40650_e46433 * var_temp);
        (assign40650_e46435, ((((var_qim__blk1016_dn4 * var_inv_qimstar1__blk1044) + (var_qim__blk1016 * var_inv_qimstar1__blk1044_dn4)) * var_temp) + (assign40650_e46433 * var_temp_dn4)), ((((var_qim__blk1016_dn6 * var_inv_qimstar1__blk1044) + (var_qim__blk1016 * var_inv_qimstar1__blk1044_dn6)) * var_temp) + (assign40650_e46433 * var_temp_dn6)), ((((var_qim__blk1016_dn7 * var_inv_qimstar1__blk1044) + (var_qim__blk1016 * var_inv_qimstar1__blk1044_dn7)) * var_temp) + (assign40650_e46433 * var_temp_dn7)), ((((var_qim__blk1016_dn8 * var_inv_qimstar1__blk1044) + (var_qim__blk1016 * var_inv_qimstar1__blk1044_dn8)) * var_temp) + (assign40650_e46433 * var_temp_dn8)), ((((var_qim__blk1016_dn9 * var_inv_qimstar1__blk1044) + (var_qim__blk1016 * var_inv_qimstar1__blk1044_dn9)) * var_temp) + (assign40650_e46433 * var_temp_dn9)),)
    } else {
        (var_r1__blk1045, var_r1__blk1045_dn4, var_r1__blk1045_dn6, var_r1__blk1045_dn7, var_r1__blk1045_dn8, var_r1__blk1045_dn9,)
    }
};
        var_r1__blk1045 = assign40650_e46437;
        var_r1__blk1045_dn4 = assign40650_e46437_d_n4;
        var_r1__blk1045_dn6 = assign40650_e46437_d_n6;
        var_r1__blk1045_dn7 = assign40650_e46437_d_n7;
        var_r1__blk1045_dn8 = assign40650_e46437_d_n8;
        var_r1__blk1045_dn9 = assign40650_e46437_d_n9;

        let (assign40660_e46458, assign40660_e46458_d_n4, assign40660_e46458_d_n6, assign40660_e46458_d_n7, assign40660_e46458_d_n8, assign40660_e46458_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40660_e46442: f64 = (var_xd - var_xdeff__blk1000);
        let assign40660_e46445: f64 = (var_vp_i * var_inv_phit);
        let assign40660_e46448: f64 = (var_vpg_i * var_qim__blk1016);
        let assign40660_e46450: f64 = (assign40660_e46448 * var_qim__blk1016);
        let assign40660_e46451: f64 = (assign40660_e46445 + assign40660_e46450);
        let assign40660_e46452: f64 = (assign40660_e46442 / assign40660_e46451);
        let assign40660_e46453: f64 = (1.0 + assign40660_e46452);
        let assign40660_e46454: f64 = (assign40660_e46453).ln();
        let assign40660_e46456: f64 = (assign40660_e46454 * var_r1__blk1045);
        (assign40660_e46456, (((((((var_xd_dn4 - var_xdeff__blk1000_dn4) * assign40660_e46451) - (assign40660_e46442 * ((var_vp_i * var_inv_phit_dn4) + (((var_vpg_i * var_qim__blk1016_dn4) * var_qim__blk1016) + (assign40660_e46448 * var_qim__blk1016_dn4))))) / (assign40660_e46451 * assign40660_e46451)) / assign40660_e46453) * var_r1__blk1045) + (assign40660_e46454 * var_r1__blk1045_dn4)), (((((((var_xd_dn6 - var_xdeff__blk1000_dn6) * assign40660_e46451) - (assign40660_e46442 * ((var_vp_i * var_inv_phit_dn6) + (((var_vpg_i * var_qim__blk1016_dn6) * var_qim__blk1016) + (assign40660_e46448 * var_qim__blk1016_dn6))))) / (assign40660_e46451 * assign40660_e46451)) / assign40660_e46453) * var_r1__blk1045) + (assign40660_e46454 * var_r1__blk1045_dn6)), (((((((var_xd_dn7 - var_xdeff__blk1000_dn7) * assign40660_e46451) - (assign40660_e46442 * ((var_vp_i * var_inv_phit_dn7) + (((var_vpg_i * var_qim__blk1016_dn7) * var_qim__blk1016) + (assign40660_e46448 * var_qim__blk1016_dn7))))) / (assign40660_e46451 * assign40660_e46451)) / assign40660_e46453) * var_r1__blk1045) + (assign40660_e46454 * var_r1__blk1045_dn7)), (((((((var_xd_dn8 - var_xdeff__blk1000_dn8) * assign40660_e46451) - (assign40660_e46442 * ((var_vp_i * var_inv_phit_dn8) + (((var_vpg_i * var_qim__blk1016_dn8) * var_qim__blk1016) + (assign40660_e46448 * var_qim__blk1016_dn8))))) / (assign40660_e46451 * assign40660_e46451)) / assign40660_e46453) * var_r1__blk1045) + (assign40660_e46454 * var_r1__blk1045_dn8)), (((((((var_xd_dn9 - var_xdeff__blk1000_dn9) * assign40660_e46451) - (assign40660_e46442 * ((var_vp_i * var_inv_phit_dn9) + (((var_vpg_i * var_qim__blk1016_dn9) * var_qim__blk1016) + (assign40660_e46448 * var_qim__blk1016_dn9))))) / (assign40660_e46451 * assign40660_e46451)) / assign40660_e46453) * var_r1__blk1045) + (assign40660_e46454 * var_r1__blk1045_dn9)),)
    } else {
        (var_dl_l_fact__blk1046, var_dl_l_fact__blk1046_dn4, var_dl_l_fact__blk1046_dn6, var_dl_l_fact__blk1046_dn7, var_dl_l_fact__blk1046_dn8, var_dl_l_fact__blk1046_dn9,)
    }
};
        var_dl_l_fact__blk1046 = assign40660_e46458;
        var_dl_l_fact__blk1046_dn4 = assign40660_e46458_d_n4;
        var_dl_l_fact__blk1046_dn6 = assign40660_e46458_d_n6;
        var_dl_l_fact__blk1046_dn7 = assign40660_e46458_d_n7;
        var_dl_l_fact__blk1046_dn8 = assign40660_e46458_d_n8;
        var_dl_l_fact__blk1046_dn9 = assign40660_e46458_d_n9;

        let (assign40670_e46464, assign40670_e46464_d_n4, assign40670_e46464_d_n6, assign40670_e46464_d_n7, assign40670_e46464_d_n8, assign40670_e46464_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40670_e46462: f64 = (var_alp_loc__blk898 * var_dl_l_fact__blk1046);
        (assign40670_e46462, (var_alp_loc__blk898 * var_dl_l_fact__blk1046_dn4), (var_alp_loc__blk898 * var_dl_l_fact__blk1046_dn6), (var_alp_loc__blk898 * var_dl_l_fact__blk1046_dn7), (var_alp_loc__blk898 * var_dl_l_fact__blk1046_dn8), (var_alp_loc__blk898 * var_dl_l_fact__blk1046_dn9),)
    } else {
        (var_dl_l__blk1047, var_dl_l__blk1047_dn4, var_dl_l__blk1047_dn6, var_dl_l__blk1047_dn7, var_dl_l__blk1047_dn8, var_dl_l__blk1047_dn9,)
    }
};
        var_dl_l__blk1047 = assign40670_e46464;
        var_dl_l__blk1047_dn4 = assign40670_e46464_d_n4;
        var_dl_l__blk1047_dn6 = assign40670_e46464_d_n6;
        var_dl_l__blk1047_dn7 = assign40670_e46464_d_n7;
        var_dl_l__blk1047_dn8 = assign40670_e46464_d_n8;
        var_dl_l__blk1047_dn9 = assign40670_e46464_d_n9;

        let (assign40680_e46476, assign40680_e46476_d_n4, assign40680_e46476_d_n6, assign40680_e46476_d_n7, assign40680_e46476_d_n8, assign40680_e46476_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40680_e46471: f64 = (1.0 + var_dl_l__blk1047);
        let assign40680_e46472: f64 = (var_dl_l__blk1047 * assign40680_e46471);
        let assign40680_e46473: f64 = (1.0 + assign40680_e46472);
        let assign40680_e46474: f64 = (1.0 / assign40680_e46473);
        (assign40680_e46474, (-(((var_dl_l__blk1047_dn4 * assign40680_e46471) + (var_dl_l__blk1047 * var_dl_l__blk1047_dn4)) / (assign40680_e46473 * assign40680_e46473))), (-(((var_dl_l__blk1047_dn6 * assign40680_e46471) + (var_dl_l__blk1047 * var_dl_l__blk1047_dn6)) / (assign40680_e46473 * assign40680_e46473))), (-(((var_dl_l__blk1047_dn7 * assign40680_e46471) + (var_dl_l__blk1047 * var_dl_l__blk1047_dn7)) / (assign40680_e46473 * assign40680_e46473))), (-(((var_dl_l__blk1047_dn8 * assign40680_e46471) + (var_dl_l__blk1047 * var_dl_l__blk1047_dn8)) / (assign40680_e46473 * assign40680_e46473))), (-(((var_dl_l__blk1047_dn9 * assign40680_e46471) + (var_dl_l__blk1047 * var_dl_l__blk1047_dn9)) / (assign40680_e46473 * assign40680_e46473))),)
    } else {
        (var_gdl__blk1048, var_gdl__blk1048_dn4, var_gdl__blk1048_dn6, var_gdl__blk1048_dn7, var_gdl__blk1048_dn8, var_gdl__blk1048_dn9,)
    }
};
        var_gdl__blk1048 = assign40680_e46476;
        var_gdl__blk1048_dn4 = assign40680_e46476_d_n4;
        var_gdl__blk1048_dn6 = assign40680_e46476_d_n6;
        var_gdl__blk1048_dn7 = assign40680_e46476_d_n7;
        var_gdl__blk1048_dn8 = assign40680_e46476_d_n8;
        var_gdl__blk1048_dn9 = assign40680_e46476_d_n9;

        let (assign40690_e46486, assign40690_e46486_d_n4, assign40690_e46486_d_n6, assign40690_e46486_d_n7, assign40690_e46486_d_n8, assign40690_e46486_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40690_e46480: f64 = (100.0 * var_esurf1__blk1027);
        let assign40690_e46483: f64 = (100.0 + var_esurf1__blk1027);
        let assign40690_e46484: f64 = (assign40690_e46480 / assign40690_e46483);
        (assign40690_e46484, ((((100.0 * var_esurf1__blk1027_dn4) * assign40690_e46483) - (assign40690_e46480 * var_esurf1__blk1027_dn4)) / (assign40690_e46483 * assign40690_e46483)), ((((100.0 * var_esurf1__blk1027_dn6) * assign40690_e46483) - (assign40690_e46480 * var_esurf1__blk1027_dn6)) / (assign40690_e46483 * assign40690_e46483)), ((((100.0 * var_esurf1__blk1027_dn7) * assign40690_e46483) - (assign40690_e46480 * var_esurf1__blk1027_dn7)) / (assign40690_e46483 * assign40690_e46483)), ((((100.0 * var_esurf1__blk1027_dn8) * assign40690_e46483) - (assign40690_e46480 * var_esurf1__blk1027_dn8)) / (assign40690_e46483 * assign40690_e46483)), ((((100.0 * var_esurf1__blk1027_dn9) * assign40690_e46483) - (assign40690_e46480 * var_esurf1__blk1027_dn9)) / (assign40690_e46483 * assign40690_e46483)),)
    } else {
        (var_wsat1__blk976, var_wsat1__blk976_dn4, var_wsat1__blk976_dn6, var_wsat1__blk976_dn7, var_wsat1__blk976_dn8, var_wsat1__blk976_dn9,)
    }
};
        var_wsat1__blk976 = assign40690_e46486;
        var_wsat1__blk976_dn4 = assign40690_e46486_d_n4;
        var_wsat1__blk976_dn6 = assign40690_e46486_d_n6;
        var_wsat1__blk976_dn7 = assign40690_e46486_d_n7;
        var_wsat1__blk976_dn8 = assign40690_e46486_d_n8;
        var_wsat1__blk976_dn9 = assign40690_e46486_d_n9;

        let assign40700_e46489: f64 = if var_thesat1_i < 0.0 { 1.0 } else { 0.0 };
        var_guard1225 = assign40700_e46489;

        let (assign40710_e46501, assign40710_e46501_d_n4, assign40710_e46501_d_n6, assign40710_e46501_d_n7, assign40710_e46501_d_n8, assign40710_e46501_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1225 != 0.0)) {
        let assign40710_e46497: f64 = (var_thesat1_i * var_wsat1__blk976);
        let assign40710_e46498: f64 = (1.0 - assign40710_e46497);
        let assign40710_e46499: f64 = (1.0 / assign40710_e46498);
        (assign40710_e46499, (-((-(var_thesat1_i * var_wsat1__blk976_dn4)) / (assign40710_e46498 * assign40710_e46498))), (-((-(var_thesat1_i * var_wsat1__blk976_dn6)) / (assign40710_e46498 * assign40710_e46498))), (-((-(var_thesat1_i * var_wsat1__blk976_dn7)) / (assign40710_e46498 * assign40710_e46498))), (-((-(var_thesat1_i * var_wsat1__blk976_dn8)) / (assign40710_e46498 * assign40710_e46498))), (-((-(var_thesat1_i * var_wsat1__blk976_dn9)) / (assign40710_e46498 * assign40710_e46498))),)
    } else {
        (var_sat_fact1__blk977, var_sat_fact1__blk977_dn4, var_sat_fact1__blk977_dn6, var_sat_fact1__blk977_dn7, var_sat_fact1__blk977_dn8, var_sat_fact1__blk977_dn9,)
    }
};
        var_sat_fact1__blk977 = assign40710_e46501;
        var_sat_fact1__blk977_dn4 = assign40710_e46501_d_n4;
        var_sat_fact1__blk977_dn6 = assign40710_e46501_d_n6;
        var_sat_fact1__blk977_dn7 = assign40710_e46501_d_n7;
        var_sat_fact1__blk977_dn8 = assign40710_e46501_d_n8;
        var_sat_fact1__blk977_dn9 = assign40710_e46501_d_n9;

        let (assign40720_e46512, assign40720_e46512_d_n4, assign40720_e46512_d_n6, assign40720_e46512_d_n7, assign40720_e46512_d_n8, assign40720_e46512_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1225 == 0.0)) {
        let assign40720_e46509: f64 = (var_thesat1_i * var_wsat1__blk976);
        let assign40720_e46510: f64 = (1.0 + assign40720_e46509);
        (assign40720_e46510, (var_thesat1_i * var_wsat1__blk976_dn4), (var_thesat1_i * var_wsat1__blk976_dn6), (var_thesat1_i * var_wsat1__blk976_dn7), (var_thesat1_i * var_wsat1__blk976_dn8), (var_thesat1_i * var_wsat1__blk976_dn9),)
    } else {
        (var_sat_fact1__blk977, var_sat_fact1__blk977_dn4, var_sat_fact1__blk977_dn6, var_sat_fact1__blk977_dn7, var_sat_fact1__blk977_dn8, var_sat_fact1__blk977_dn9,)
    }
};
        var_sat_fact1__blk977 = assign40720_e46512;
        var_sat_fact1__blk977_dn4 = assign40720_e46512_d_n4;
        var_sat_fact1__blk977_dn6 = assign40720_e46512_d_n6;
        var_sat_fact1__blk977_dn7 = assign40720_e46512_d_n7;
        var_sat_fact1__blk977_dn8 = assign40720_e46512_d_n8;
        var_sat_fact1__blk977_dn9 = assign40720_e46512_d_n9;

        let (assign40730_e46522, assign40730_e46522_d_n4, assign40730_e46522_d_n6, assign40730_e46522_d_n7, assign40730_e46522_d_n8, assign40730_e46522_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40730_e46516: f64 = (100.0 * var_esurf2__blk1028);
        let assign40730_e46519: f64 = (100.0 + var_esurf2__blk1028);
        let assign40730_e46520: f64 = (assign40730_e46516 / assign40730_e46519);
        (assign40730_e46520, ((((100.0 * var_esurf2__blk1028_dn4) * assign40730_e46519) - (assign40730_e46516 * var_esurf2__blk1028_dn4)) / (assign40730_e46519 * assign40730_e46519)), ((((100.0 * var_esurf2__blk1028_dn6) * assign40730_e46519) - (assign40730_e46516 * var_esurf2__blk1028_dn6)) / (assign40730_e46519 * assign40730_e46519)), ((((100.0 * var_esurf2__blk1028_dn7) * assign40730_e46519) - (assign40730_e46516 * var_esurf2__blk1028_dn7)) / (assign40730_e46519 * assign40730_e46519)), ((((100.0 * var_esurf2__blk1028_dn8) * assign40730_e46519) - (assign40730_e46516 * var_esurf2__blk1028_dn8)) / (assign40730_e46519 * assign40730_e46519)), ((((100.0 * var_esurf2__blk1028_dn9) * assign40730_e46519) - (assign40730_e46516 * var_esurf2__blk1028_dn9)) / (assign40730_e46519 * assign40730_e46519)),)
    } else {
        (var_wsat2__blk978, var_wsat2__blk978_dn4, var_wsat2__blk978_dn6, var_wsat2__blk978_dn7, var_wsat2__blk978_dn8, var_wsat2__blk978_dn9,)
    }
};
        var_wsat2__blk978 = assign40730_e46522;
        var_wsat2__blk978_dn4 = assign40730_e46522_d_n4;
        var_wsat2__blk978_dn6 = assign40730_e46522_d_n6;
        var_wsat2__blk978_dn7 = assign40730_e46522_d_n7;
        var_wsat2__blk978_dn8 = assign40730_e46522_d_n8;
        var_wsat2__blk978_dn9 = assign40730_e46522_d_n9;

        let assign40740_e46525: f64 = if var_thesat2_i < 0.0 { 1.0 } else { 0.0 };
        var_guard1226 = assign40740_e46525;

        let (assign40750_e46537, assign40750_e46537_d_n4, assign40750_e46537_d_n6, assign40750_e46537_d_n7, assign40750_e46537_d_n8, assign40750_e46537_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1226 != 0.0)) {
        let assign40750_e46533: f64 = (var_thesat2_i * var_wsat2__blk978);
        let assign40750_e46534: f64 = (1.0 - assign40750_e46533);
        let assign40750_e46535: f64 = (1.0 / assign40750_e46534);
        (assign40750_e46535, (-((-(var_thesat2_i * var_wsat2__blk978_dn4)) / (assign40750_e46534 * assign40750_e46534))), (-((-(var_thesat2_i * var_wsat2__blk978_dn6)) / (assign40750_e46534 * assign40750_e46534))), (-((-(var_thesat2_i * var_wsat2__blk978_dn7)) / (assign40750_e46534 * assign40750_e46534))), (-((-(var_thesat2_i * var_wsat2__blk978_dn8)) / (assign40750_e46534 * assign40750_e46534))), (-((-(var_thesat2_i * var_wsat2__blk978_dn9)) / (assign40750_e46534 * assign40750_e46534))),)
    } else {
        (var_sat_fact2__blk979, var_sat_fact2__blk979_dn4, var_sat_fact2__blk979_dn6, var_sat_fact2__blk979_dn7, var_sat_fact2__blk979_dn8, var_sat_fact2__blk979_dn9,)
    }
};
        var_sat_fact2__blk979 = assign40750_e46537;
        var_sat_fact2__blk979_dn4 = assign40750_e46537_d_n4;
        var_sat_fact2__blk979_dn6 = assign40750_e46537_d_n6;
        var_sat_fact2__blk979_dn7 = assign40750_e46537_d_n7;
        var_sat_fact2__blk979_dn8 = assign40750_e46537_d_n8;
        var_sat_fact2__blk979_dn9 = assign40750_e46537_d_n9;

        let (assign40760_e46548, assign40760_e46548_d_n4, assign40760_e46548_d_n6, assign40760_e46548_d_n7, assign40760_e46548_d_n8, assign40760_e46548_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1226 == 0.0)) {
        let assign40760_e46545: f64 = (var_thesat2_i * var_wsat2__blk978);
        let assign40760_e46546: f64 = (1.0 + assign40760_e46545);
        (assign40760_e46546, (var_thesat2_i * var_wsat2__blk978_dn4), (var_thesat2_i * var_wsat2__blk978_dn6), (var_thesat2_i * var_wsat2__blk978_dn7), (var_thesat2_i * var_wsat2__blk978_dn8), (var_thesat2_i * var_wsat2__blk978_dn9),)
    } else {
        (var_sat_fact2__blk979, var_sat_fact2__blk979_dn4, var_sat_fact2__blk979_dn6, var_sat_fact2__blk979_dn7, var_sat_fact2__blk979_dn8, var_sat_fact2__blk979_dn9,)
    }
};
        var_sat_fact2__blk979 = assign40760_e46548;
        var_sat_fact2__blk979_dn4 = assign40760_e46548_d_n4;
        var_sat_fact2__blk979_dn6 = assign40760_e46548_d_n6;
        var_sat_fact2__blk979_dn7 = assign40760_e46548_d_n7;
        var_sat_fact2__blk979_dn8 = assign40760_e46548_d_n8;
        var_sat_fact2__blk979_dn9 = assign40760_e46548_d_n9;

        let (assign40770_e46560, assign40770_e46560_d_n4, assign40770_e46560_d_n6, assign40770_e46560_d_n7, assign40770_e46560_d_n8, assign40770_e46560_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40770_e46552: f64 = (var_sat_phit_loc__blk896 * var_dxdrift__blk1017);
        let assign40770_e46554: f64 = (assign40770_e46552 * 0.5);
        let assign40770_e46557: f64 = (var_sat_fact1__blk977 + var_sat_fact2__blk979);
        let assign40770_e46558: f64 = (assign40770_e46554 * assign40770_e46557);
        (assign40770_e46558, (((((var_sat_phit_loc__blk896_dn4 * var_dxdrift__blk1017) + (var_sat_phit_loc__blk896 * var_dxdrift__blk1017_dn4)) * 0.5) * assign40770_e46557) + (assign40770_e46554 * (var_sat_fact1__blk977_dn4 + var_sat_fact2__blk979_dn4))), (((((var_sat_phit_loc__blk896_dn6 * var_dxdrift__blk1017) + (var_sat_phit_loc__blk896 * var_dxdrift__blk1017_dn6)) * 0.5) * assign40770_e46557) + (assign40770_e46554 * (var_sat_fact1__blk977_dn6 + var_sat_fact2__blk979_dn6))), (((((var_sat_phit_loc__blk896_dn7 * var_dxdrift__blk1017) + (var_sat_phit_loc__blk896 * var_dxdrift__blk1017_dn7)) * 0.5) * assign40770_e46557) + (assign40770_e46554 * (var_sat_fact1__blk977_dn7 + var_sat_fact2__blk979_dn7))), (((((var_sat_phit_loc__blk896_dn8 * var_dxdrift__blk1017) + (var_sat_phit_loc__blk896 * var_dxdrift__blk1017_dn8)) * 0.5) * assign40770_e46557) + (assign40770_e46554 * (var_sat_fact1__blk977_dn8 + var_sat_fact2__blk979_dn8))), (((((var_sat_phit_loc__blk896_dn9 * var_dxdrift__blk1017) + (var_sat_phit_loc__blk896 * var_dxdrift__blk1017_dn9)) * 0.5) * assign40770_e46557) + (assign40770_e46554 * (var_sat_fact1__blk977_dn9 + var_sat_fact2__blk979_dn9))),)
    } else {
        (var_ggamma__blk1049, var_ggamma__blk1049_dn4, var_ggamma__blk1049_dn6, var_ggamma__blk1049_dn7, var_ggamma__blk1049_dn8, var_ggamma__blk1049_dn9,)
    }
};
        var_ggamma__blk1049 = assign40770_e46560;
        var_ggamma__blk1049_dn4 = assign40770_e46560_d_n4;
        var_ggamma__blk1049_dn6 = assign40770_e46560_d_n6;
        var_ggamma__blk1049_dn7 = assign40770_e46560_d_n7;
        var_ggamma__blk1049_dn8 = assign40770_e46560_d_n8;
        var_ggamma__blk1049_dn9 = assign40770_e46560_d_n9;

        let (assign40780_e46568, assign40780_e46568_d_n4, assign40780_e46568_d_n6, assign40780_e46568_d_n7, assign40780_e46568_d_n8, assign40780_e46568_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40780_e46565: f64 = (var_gmob__blk1043 * var_gdl__blk1048);
        let assign40780_e46566: f64 = (var_ggamma__blk1049 / assign40780_e46565);
        (assign40780_e46566, (((var_ggamma__blk1049_dn4 * assign40780_e46565) - (var_ggamma__blk1049 * ((var_gmob__blk1043_dn4 * var_gdl__blk1048) + (var_gmob__blk1043 * var_gdl__blk1048_dn4)))) / (assign40780_e46565 * assign40780_e46565)), (((var_ggamma__blk1049_dn6 * assign40780_e46565) - (var_ggamma__blk1049 * ((var_gmob__blk1043_dn6 * var_gdl__blk1048) + (var_gmob__blk1043 * var_gdl__blk1048_dn6)))) / (assign40780_e46565 * assign40780_e46565)), (((var_ggamma__blk1049_dn7 * assign40780_e46565) - (var_ggamma__blk1049 * ((var_gmob__blk1043_dn7 * var_gdl__blk1048) + (var_gmob__blk1043 * var_gdl__blk1048_dn7)))) / (assign40780_e46565 * assign40780_e46565)), (((var_ggamma__blk1049_dn8 * assign40780_e46565) - (var_ggamma__blk1049 * ((var_gmob__blk1043_dn8 * var_gdl__blk1048) + (var_gmob__blk1043 * var_gdl__blk1048_dn8)))) / (assign40780_e46565 * assign40780_e46565)), (((var_ggamma__blk1049_dn9 * assign40780_e46565) - (var_ggamma__blk1049 * ((var_gmob__blk1043_dn9 * var_gdl__blk1048) + (var_gmob__blk1043 * var_gdl__blk1048_dn9)))) / (assign40780_e46565 * assign40780_e46565)),)
    } else {
        (var_sqrt_zsat__blk1050, var_sqrt_zsat__blk1050_dn4, var_sqrt_zsat__blk1050_dn6, var_sqrt_zsat__blk1050_dn7, var_sqrt_zsat__blk1050_dn8, var_sqrt_zsat__blk1050_dn9,)
    }
};
        var_sqrt_zsat__blk1050 = assign40780_e46568;
        var_sqrt_zsat__blk1050_dn4 = assign40780_e46568_d_n4;
        var_sqrt_zsat__blk1050_dn6 = assign40780_e46568_d_n6;
        var_sqrt_zsat__blk1050_dn7 = assign40780_e46568_d_n7;
        var_sqrt_zsat__blk1050_dn8 = assign40780_e46568_d_n8;
        var_sqrt_zsat__blk1050_dn9 = assign40780_e46568_d_n9;

        let (assign40790_e46574, assign40790_e46574_d_n4, assign40790_e46574_d_n6, assign40790_e46574_d_n7, assign40790_e46574_d_n8, assign40790_e46574_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40790_e46572: f64 = (var_sqrt_zsat__blk1050 * var_sqrt_zsat__blk1050);
        (assign40790_e46572, ((var_sqrt_zsat__blk1050_dn4 * var_sqrt_zsat__blk1050) + (var_sqrt_zsat__blk1050 * var_sqrt_zsat__blk1050_dn4)), ((var_sqrt_zsat__blk1050_dn6 * var_sqrt_zsat__blk1050) + (var_sqrt_zsat__blk1050 * var_sqrt_zsat__blk1050_dn6)), ((var_sqrt_zsat__blk1050_dn7 * var_sqrt_zsat__blk1050) + (var_sqrt_zsat__blk1050 * var_sqrt_zsat__blk1050_dn7)), ((var_sqrt_zsat__blk1050_dn8 * var_sqrt_zsat__blk1050) + (var_sqrt_zsat__blk1050 * var_sqrt_zsat__blk1050_dn8)), ((var_sqrt_zsat__blk1050_dn9 * var_sqrt_zsat__blk1050) + (var_sqrt_zsat__blk1050 * var_sqrt_zsat__blk1050_dn9)),)
    } else {
        (var_zsat__blk1051, var_zsat__blk1051_dn4, var_zsat__blk1051_dn6, var_zsat__blk1051_dn7, var_zsat__blk1051_dn8, var_zsat__blk1051_dn9,)
    }
};
        var_zsat__blk1051 = assign40790_e46574;
        var_zsat__blk1051_dn4 = assign40790_e46574_d_n4;
        var_zsat__blk1051_dn6 = assign40790_e46574_d_n6;
        var_zsat__blk1051_dn7 = assign40790_e46574_d_n7;
        var_zsat__blk1051_dn8 = assign40790_e46574_d_n8;
        var_zsat__blk1051_dn9 = assign40790_e46574_d_n9;

        let (assign40800_e46581, assign40800_e46581_d_n4, assign40800_e46581_d_n6, assign40800_e46581_d_n7, assign40800_e46581_d_n8, assign40800_e46581_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40800_e46578: f64 = (1.0 + var_zsat__blk1051);
        let assign40800_e46579: f64 = (assign40800_e46578).sqrt();
        (assign40800_e46579, (var_zsat__blk1051_dn4 / (2.0 * assign40800_e46579)), (var_zsat__blk1051_dn6 / (2.0 * assign40800_e46579)), (var_zsat__blk1051_dn7 / (2.0 * assign40800_e46579)), (var_zsat__blk1051_dn8 / (2.0 * assign40800_e46579)), (var_zsat__blk1051_dn9 / (2.0 * assign40800_e46579)),)
    } else {
        (var_vsat_fact__blk1052, var_vsat_fact__blk1052_dn4, var_vsat_fact__blk1052_dn6, var_vsat_fact__blk1052_dn7, var_vsat_fact__blk1052_dn8, var_vsat_fact__blk1052_dn9,)
    }
};
        var_vsat_fact__blk1052 = assign40800_e46581;
        var_vsat_fact__blk1052_dn4 = assign40800_e46581_d_n4;
        var_vsat_fact__blk1052_dn6 = assign40800_e46581_d_n6;
        var_vsat_fact__blk1052_dn7 = assign40800_e46581_d_n7;
        var_vsat_fact__blk1052_dn8 = assign40800_e46581_d_n8;
        var_vsat_fact__blk1052_dn9 = assign40800_e46581_d_n9;

        let (assign40810_e46591, assign40810_e46591_d_n4, assign40810_e46591_d_n6, assign40810_e46591_d_n7, assign40810_e46591_d_n8, assign40810_e46591_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign40810_e46586: f64 = (1.5 * var_zsat__blk1051);
        let assign40810_e46587: f64 = (1.0 + assign40810_e46586);
        let assign40810_e46589: f64 = (assign40810_e46587 / var_vsat_fact__blk1052);
        (assign40810_e46589, ((((1.5 * var_zsat__blk1051_dn4) * var_vsat_fact__blk1052) - (assign40810_e46587 * var_vsat_fact__blk1052_dn4)) / (var_vsat_fact__blk1052 * var_vsat_fact__blk1052)), ((((1.5 * var_zsat__blk1051_dn6) * var_vsat_fact__blk1052) - (assign40810_e46587 * var_vsat_fact__blk1052_dn6)) / (var_vsat_fact__blk1052 * var_vsat_fact__blk1052)), ((((1.5 * var_zsat__blk1051_dn7) * var_vsat_fact__blk1052) - (assign40810_e46587 * var_vsat_fact__blk1052_dn7)) / (var_vsat_fact__blk1052 * var_vsat_fact__blk1052)), ((((1.5 * var_zsat__blk1051_dn8) * var_vsat_fact__blk1052) - (assign40810_e46587 * var_vsat_fact__blk1052_dn8)) / (var_vsat_fact__blk1052 * var_vsat_fact__blk1052)), ((((1.5 * var_zsat__blk1051_dn9) * var_vsat_fact__blk1052) - (assign40810_e46587 * var_vsat_fact__blk1052_dn9)) / (var_vsat_fact__blk1052 * var_vsat_fact__blk1052)),)
    } else {
        (var_hsat__blk1053, var_hsat__blk1053_dn4, var_hsat__blk1053_dn6, var_hsat__blk1053_dn7, var_hsat__blk1053_dn8, var_hsat__blk1053_dn9,)
    }
};
        var_hsat__blk1053 = assign40810_e46591;
        var_hsat__blk1053_dn4 = assign40810_e46591_d_n4;
        var_hsat__blk1053_dn6 = assign40810_e46591_d_n6;
        var_hsat__blk1053_dn7 = assign40810_e46591_d_n7;
        var_hsat__blk1053_dn8 = assign40810_e46591_d_n8;
        var_hsat__blk1053_dn9 = assign40810_e46591_d_n9;

        let assign40820_e46594: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        var_guard1227 = assign40820_e46594;

        let (assign40830_e46613, assign40830_e46613_d_n4, assign40830_e46613_d_n6, assign40830_e46613_d_n7, assign40830_e46613_d_n8, assign40830_e46613_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1227 != 0.0)) {
        let assign40830_e46600: f64 = (0.6 * var_qq);
        let assign40830_e46602: f64 = (-0.1666666666667);
        let assign40830_e46605: f64 = (var_esurf1__blk1027 * var_esurf1__blk1027);
        let assign40830_e46607: f64 = (assign40830_e46605 + 60.0);
        let assign40830_e46608: f64 = (assign40830_e46607).ln();
        let assign40830_e46609: f64 = (assign40830_e46602 * assign40830_e46608);
        let assign40830_e46610: f64 = (assign40830_e46609).exp();
        let assign40830_e46611: f64 = (assign40830_e46600 * assign40830_e46610);
        (assign40830_e46611, (((0.6 * var_qq_dn4) * assign40830_e46610) + (assign40830_e46600 * (assign40830_e46610 * (assign40830_e46602 * (((var_esurf1__blk1027_dn4 * var_esurf1__blk1027) + (var_esurf1__blk1027 * var_esurf1__blk1027_dn4)) / assign40830_e46607))))), (((0.6 * var_qq_dn6) * assign40830_e46610) + (assign40830_e46600 * (assign40830_e46610 * (assign40830_e46602 * (((var_esurf1__blk1027_dn6 * var_esurf1__blk1027) + (var_esurf1__blk1027 * var_esurf1__blk1027_dn6)) / assign40830_e46607))))), (((0.6 * var_qq_dn7) * assign40830_e46610) + (assign40830_e46600 * (assign40830_e46610 * (assign40830_e46602 * (((var_esurf1__blk1027_dn7 * var_esurf1__blk1027) + (var_esurf1__blk1027 * var_esurf1__blk1027_dn7)) / assign40830_e46607))))), (((0.6 * var_qq_dn8) * assign40830_e46610) + (assign40830_e46600 * (assign40830_e46610 * (assign40830_e46602 * (((var_esurf1__blk1027_dn8 * var_esurf1__blk1027) + (var_esurf1__blk1027 * var_esurf1__blk1027_dn8)) / assign40830_e46607))))), (((0.6 * var_qq_dn9) * assign40830_e46610) + (assign40830_e46600 * (assign40830_e46610 * (assign40830_e46602 * (((var_esurf1__blk1027_dn9 * var_esurf1__blk1027) + (var_esurf1__blk1027 * var_esurf1__blk1027_dn9)) / assign40830_e46607))))),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign40830_e46613;
        var_temp1_dn4 = assign40830_e46613_d_n4;
        var_temp1_dn6 = assign40830_e46613_d_n6;
        var_temp1_dn7 = assign40830_e46613_d_n7;
        var_temp1_dn8 = assign40830_e46613_d_n8;
        var_temp1_dn9 = assign40830_e46613_d_n9;

        let (assign40840_e46632, assign40840_e46632_d_n4, assign40840_e46632_d_n6, assign40840_e46632_d_n7, assign40840_e46632_d_n8, assign40840_e46632_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1227 != 0.0)) {
        let assign40840_e46619: f64 = (0.6 * var_qq);
        let assign40840_e46621: f64 = (-0.1666666666667);
        let assign40840_e46624: f64 = (var_esurf2__blk1028 * var_esurf2__blk1028);
        let assign40840_e46626: f64 = (assign40840_e46624 + 60.0);
        let assign40840_e46627: f64 = (assign40840_e46626).ln();
        let assign40840_e46628: f64 = (assign40840_e46621 * assign40840_e46627);
        let assign40840_e46629: f64 = (assign40840_e46628).exp();
        let assign40840_e46630: f64 = (assign40840_e46619 * assign40840_e46629);
        (assign40840_e46630, (((0.6 * var_qq_dn4) * assign40840_e46629) + (assign40840_e46619 * (assign40840_e46629 * (assign40840_e46621 * (((var_esurf2__blk1028_dn4 * var_esurf2__blk1028) + (var_esurf2__blk1028 * var_esurf2__blk1028_dn4)) / assign40840_e46626))))), (((0.6 * var_qq_dn6) * assign40840_e46629) + (assign40840_e46619 * (assign40840_e46629 * (assign40840_e46621 * (((var_esurf2__blk1028_dn6 * var_esurf2__blk1028) + (var_esurf2__blk1028 * var_esurf2__blk1028_dn6)) / assign40840_e46626))))), (((0.6 * var_qq_dn7) * assign40840_e46629) + (assign40840_e46619 * (assign40840_e46629 * (assign40840_e46621 * (((var_esurf2__blk1028_dn7 * var_esurf2__blk1028) + (var_esurf2__blk1028 * var_esurf2__blk1028_dn7)) / assign40840_e46626))))), (((0.6 * var_qq_dn8) * assign40840_e46629) + (assign40840_e46619 * (assign40840_e46629 * (assign40840_e46621 * (((var_esurf2__blk1028_dn8 * var_esurf2__blk1028) + (var_esurf2__blk1028 * var_esurf2__blk1028_dn8)) / assign40840_e46626))))), (((0.6 * var_qq_dn9) * assign40840_e46629) + (assign40840_e46619 * (assign40840_e46629 * (assign40840_e46621 * (((var_esurf2__blk1028_dn9 * var_esurf2__blk1028) + (var_esurf2__blk1028 * var_esurf2__blk1028_dn9)) / assign40840_e46626))))),)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign40840_e46632;
        var_temp2_dn4 = assign40840_e46632_d_n4;
        var_temp2_dn6 = assign40840_e46632_d_n6;
        var_temp2_dn7 = assign40840_e46632_d_n7;
        var_temp2_dn8 = assign40840_e46632_d_n8;
        var_temp2_dn9 = assign40840_e46632_d_n9;

        let (assign40850_e46644, assign40850_e46644_d_n4, assign40850_e46644_d_n6, assign40850_e46644_d_n7, assign40850_e46644_d_n8, assign40850_e46644_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1227 != 0.0)) {
        let assign40850_e46639: f64 = (var_k1__blk932 * var_temp1);
        let assign40850_e46640: f64 = (1.0 + assign40850_e46639);
        let assign40850_e46642: f64 = (assign40850_e46640 / var_tox1fact__blk913);
        (assign40850_e46642, (((((var_k1__blk932_dn4 * var_temp1) + (var_k1__blk932 * var_temp1_dn4)) * var_tox1fact__blk913) - (assign40850_e46640 * var_tox1fact__blk913_dn4)) / (var_tox1fact__blk913 * var_tox1fact__blk913)), (((((var_k1__blk932_dn6 * var_temp1) + (var_k1__blk932 * var_temp1_dn6)) * var_tox1fact__blk913) - (assign40850_e46640 * var_tox1fact__blk913_dn6)) / (var_tox1fact__blk913 * var_tox1fact__blk913)), (((((var_k1__blk932_dn7 * var_temp1) + (var_k1__blk932 * var_temp1_dn7)) * var_tox1fact__blk913) - (assign40850_e46640 * var_tox1fact__blk913_dn7)) / (var_tox1fact__blk913 * var_tox1fact__blk913)), (((((var_k1__blk932_dn8 * var_temp1) + (var_k1__blk932 * var_temp1_dn8)) * var_tox1fact__blk913) - (assign40850_e46640 * var_tox1fact__blk913_dn8)) / (var_tox1fact__blk913 * var_tox1fact__blk913)), (((((var_k1__blk932_dn9 * var_temp1) + (var_k1__blk932 * var_temp1_dn9)) * var_tox1fact__blk913) - (assign40850_e46640 * var_tox1fact__blk913_dn9)) / (var_tox1fact__blk913 * var_tox1fact__blk913)),)
    } else {
        (var_qmfact1__blk1054, var_qmfact1__blk1054_dn4, var_qmfact1__blk1054_dn6, var_qmfact1__blk1054_dn7, var_qmfact1__blk1054_dn8, var_qmfact1__blk1054_dn9,)
    }
};
        var_qmfact1__blk1054 = assign40850_e46644;
        var_qmfact1__blk1054_dn4 = assign40850_e46644_d_n4;
        var_qmfact1__blk1054_dn6 = assign40850_e46644_d_n6;
        var_qmfact1__blk1054_dn7 = assign40850_e46644_d_n7;
        var_qmfact1__blk1054_dn8 = assign40850_e46644_d_n8;
        var_qmfact1__blk1054_dn9 = assign40850_e46644_d_n9;

        let (assign40860_e46656, assign40860_e46656_d_n4, assign40860_e46656_d_n6, assign40860_e46656_d_n7, assign40860_e46656_d_n8, assign40860_e46656_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1227 != 0.0)) {
        let assign40860_e46651: f64 = (var_k2__blk933 * var_temp2);
        let assign40860_e46652: f64 = (1.0 + assign40860_e46651);
        let assign40860_e46654: f64 = (assign40860_e46652 / var_tox2fact__blk914);
        (assign40860_e46654, (((((var_k2__blk933_dn4 * var_temp2) + (var_k2__blk933 * var_temp2_dn4)) * var_tox2fact__blk914) - (assign40860_e46652 * var_tox2fact__blk914_dn4)) / (var_tox2fact__blk914 * var_tox2fact__blk914)), (((((var_k2__blk933_dn6 * var_temp2) + (var_k2__blk933 * var_temp2_dn6)) * var_tox2fact__blk914) - (assign40860_e46652 * var_tox2fact__blk914_dn6)) / (var_tox2fact__blk914 * var_tox2fact__blk914)), (((((var_k2__blk933_dn7 * var_temp2) + (var_k2__blk933 * var_temp2_dn7)) * var_tox2fact__blk914) - (assign40860_e46652 * var_tox2fact__blk914_dn7)) / (var_tox2fact__blk914 * var_tox2fact__blk914)), (((((var_k2__blk933_dn8 * var_temp2) + (var_k2__blk933 * var_temp2_dn8)) * var_tox2fact__blk914) - (assign40860_e46652 * var_tox2fact__blk914_dn8)) / (var_tox2fact__blk914 * var_tox2fact__blk914)), (((((var_k2__blk933_dn9 * var_temp2) + (var_k2__blk933 * var_temp2_dn9)) * var_tox2fact__blk914) - (assign40860_e46652 * var_tox2fact__blk914_dn9)) / (var_tox2fact__blk914 * var_tox2fact__blk914)),)
    } else {
        (var_qmfact2__blk1055, var_qmfact2__blk1055_dn4, var_qmfact2__blk1055_dn6, var_qmfact2__blk1055_dn7, var_qmfact2__blk1055_dn8, var_qmfact2__blk1055_dn9,)
    }
};
        var_qmfact2__blk1055 = assign40860_e46656;
        var_qmfact2__blk1055_dn4 = assign40860_e46656_d_n4;
        var_qmfact2__blk1055_dn6 = assign40860_e46656_d_n6;
        var_qmfact2__blk1055_dn7 = assign40860_e46656_d_n7;
        var_qmfact2__blk1055_dn8 = assign40860_e46656_d_n8;
        var_qmfact2__blk1055_dn9 = assign40860_e46656_d_n9;

        let (assign40870_e46663, assign40870_e46663_d_n4, assign40870_e46663_d_n6, assign40870_e46663_d_n7, assign40870_e46663_d_n8, assign40870_e46663_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1227 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qmfact1__blk1054, var_qmfact1__blk1054_dn4, var_qmfact1__blk1054_dn6, var_qmfact1__blk1054_dn7, var_qmfact1__blk1054_dn8, var_qmfact1__blk1054_dn9,)
    }
};
        var_qmfact1__blk1054 = assign40870_e46663;
        var_qmfact1__blk1054_dn4 = assign40870_e46663_d_n4;
        var_qmfact1__blk1054_dn6 = assign40870_e46663_d_n6;
        var_qmfact1__blk1054_dn7 = assign40870_e46663_d_n7;
        var_qmfact1__blk1054_dn8 = assign40870_e46663_d_n8;
        var_qmfact1__blk1054_dn9 = assign40870_e46663_d_n9;

        let (assign40880_e46670, assign40880_e46670_d_n4, assign40880_e46670_d_n6, assign40880_e46670_d_n7, assign40880_e46670_d_n8, assign40880_e46670_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1227 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qmfact2__blk1055, var_qmfact2__blk1055_dn4, var_qmfact2__blk1055_dn6, var_qmfact2__blk1055_dn7, var_qmfact2__blk1055_dn8, var_qmfact2__blk1055_dn9,)
    }
};
        var_qmfact2__blk1055 = assign40880_e46670;
        var_qmfact2__blk1055_dn4 = assign40880_e46670_d_n4;
        var_qmfact2__blk1055_dn6 = assign40880_e46670_d_n6;
        var_qmfact2__blk1055_dn7 = assign40880_e46670_d_n7;
        var_qmfact2__blk1055_dn8 = assign40880_e46670_d_n8;
        var_qmfact2__blk1055_dn9 = assign40880_e46670_d_n9;

        let assign40890_e46673: f64 = if var_qis__blk938 > 1e-6 { 1.0 } else { 0.0 };
        var_guard1228 = assign40890_e46673;

        let assign40900_e46676: f64 = if var_qid__blk1003 > 1e-6 { 1.0 } else { 0.0 };
        var_guard1229 = assign40900_e46676;

        let assign40910_e46678: f64 = (var_a2d__blk1012).abs();
        let assign40910_e46680: f64 = if assign40910_e46678 < 0.01 { 1.0 } else { 0.0 };
        var_guard1230 = assign40910_e46680;

        let (assign40920_e46702, assign40920_e46702_d_n4, assign40920_e46702_d_n6, assign40920_e46702_d_n7, assign40920_e46702_d_n8, assign40920_e46702_d_n9,) = {
    if ((((var_guard1080 != 0.0) && (var_guard1228 != 0.0)) && (var_guard1229 != 0.0)) && (var_guard1230 != 0.0)) {
        let assign40920_e46690: f64 = (2.0 + var_q1d__blk1001);
        let assign40920_e46693: f64 = (0.5 * var_a1d__blk1011);
        let assign40920_e46694: f64 = (assign40920_e46690 + assign40920_e46693);
        let assign40920_e46697: f64 = (2.0 + var_q2d__blk1002);
        let assign40920_e46699: f64 = (assign40920_e46697 * var_a1d__blk1011);
        let assign40920_e46700: f64 = (assign40920_e46694 / assign40920_e46699);
        (assign40920_e46700, ((((var_q1d__blk1001_dn4 + (0.5 * var_a1d__blk1011_dn4)) * assign40920_e46699) - (assign40920_e46694 * ((var_q2d__blk1002_dn4 * var_a1d__blk1011) + (assign40920_e46697 * var_a1d__blk1011_dn4)))) / (assign40920_e46699 * assign40920_e46699)), ((((var_q1d__blk1001_dn6 + (0.5 * var_a1d__blk1011_dn6)) * assign40920_e46699) - (assign40920_e46694 * ((var_q2d__blk1002_dn6 * var_a1d__blk1011) + (assign40920_e46697 * var_a1d__blk1011_dn6)))) / (assign40920_e46699 * assign40920_e46699)), ((((var_q1d__blk1001_dn7 + (0.5 * var_a1d__blk1011_dn7)) * assign40920_e46699) - (assign40920_e46694 * ((var_q2d__blk1002_dn7 * var_a1d__blk1011) + (assign40920_e46697 * var_a1d__blk1011_dn7)))) / (assign40920_e46699 * assign40920_e46699)), ((((var_q1d__blk1001_dn8 + (0.5 * var_a1d__blk1011_dn8)) * assign40920_e46699) - (assign40920_e46694 * ((var_q2d__blk1002_dn8 * var_a1d__blk1011) + (assign40920_e46697 * var_a1d__blk1011_dn8)))) / (assign40920_e46699 * assign40920_e46699)), ((((var_q1d__blk1001_dn9 + (0.5 * var_a1d__blk1011_dn9)) * assign40920_e46699) - (assign40920_e46694 * ((var_q2d__blk1002_dn9 * var_a1d__blk1011) + (assign40920_e46697 * var_a1d__blk1011_dn9)))) / (assign40920_e46699 * assign40920_e46699)),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign40920_e46702;
        var_temp_dn4 = assign40920_e46702_d_n4;
        var_temp_dn6 = assign40920_e46702_d_n6;
        var_temp_dn7 = assign40920_e46702_d_n7;
        var_temp_dn8 = assign40920_e46702_d_n8;
        var_temp_dn9 = assign40920_e46702_d_n9;

        let (assign40930_e46714, assign40930_e46714_d_n4, assign40930_e46714_d_n6, assign40930_e46714_d_n7, assign40930_e46714_d_n8, assign40930_e46714_d_n9,) = {
    if ((((var_guard1080 != 0.0) && (var_guard1228 != 0.0)) && (var_guard1229 != 0.0)) && (var_guard1230 != 0.0)) {
        let assign40930_e46712: f64 = (var_temp * var_a2d__blk1012);
        (assign40930_e46712, ((var_temp_dn4 * var_a2d__blk1012) + (var_temp * var_a2d__blk1012_dn4)), ((var_temp_dn6 * var_a2d__blk1012) + (var_temp * var_a2d__blk1012_dn6)), ((var_temp_dn7 * var_a2d__blk1012) + (var_temp * var_a2d__blk1012_dn7)), ((var_temp_dn8 * var_a2d__blk1012) + (var_temp * var_a2d__blk1012_dn8)), ((var_temp_dn9 * var_a2d__blk1012) + (var_temp * var_a2d__blk1012_dn9)),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign40930_e46714;
        var_temp1_dn4 = assign40930_e46714_d_n4;
        var_temp1_dn6 = assign40930_e46714_d_n6;
        var_temp1_dn7 = assign40930_e46714_d_n7;
        var_temp1_dn8 = assign40930_e46714_d_n8;
        var_temp1_dn9 = assign40930_e46714_d_n9;

        *var_dl_l__blk1047_slot = var_dl_l__blk1047;
        *var_dl_l__blk1047_dn4_slot = var_dl_l__blk1047_dn4;
        *var_dl_l__blk1047_dn6_slot = var_dl_l__blk1047_dn6;
        *var_dl_l__blk1047_dn7_slot = var_dl_l__blk1047_dn7;
        *var_dl_l__blk1047_dn8_slot = var_dl_l__blk1047_dn8;
        *var_dl_l__blk1047_dn9_slot = var_dl_l__blk1047_dn9;
        *var_dl_l_fact__blk1046_slot = var_dl_l_fact__blk1046;
        *var_dl_l_fact__blk1046_dn4_slot = var_dl_l_fact__blk1046_dn4;
        *var_dl_l_fact__blk1046_dn6_slot = var_dl_l_fact__blk1046_dn6;
        *var_dl_l_fact__blk1046_dn7_slot = var_dl_l_fact__blk1046_dn7;
        *var_dl_l_fact__blk1046_dn8_slot = var_dl_l_fact__blk1046_dn8;
        *var_dl_l_fact__blk1046_dn9_slot = var_dl_l_fact__blk1046_dn9;
        *var_gdl__blk1048_slot = var_gdl__blk1048;
        *var_gdl__blk1048_dn4_slot = var_gdl__blk1048_dn4;
        *var_gdl__blk1048_dn6_slot = var_gdl__blk1048_dn6;
        *var_gdl__blk1048_dn7_slot = var_gdl__blk1048_dn7;
        *var_gdl__blk1048_dn8_slot = var_gdl__blk1048_dn8;
        *var_gdl__blk1048_dn9_slot = var_gdl__blk1048_dn9;
        *var_ggamma__blk1049_slot = var_ggamma__blk1049;
        *var_ggamma__blk1049_dn4_slot = var_ggamma__blk1049_dn4;
        *var_ggamma__blk1049_dn6_slot = var_ggamma__blk1049_dn6;
        *var_ggamma__blk1049_dn7_slot = var_ggamma__blk1049_dn7;
        *var_ggamma__blk1049_dn8_slot = var_ggamma__blk1049_dn8;
        *var_ggamma__blk1049_dn9_slot = var_ggamma__blk1049_dn9;
        *var_gmob2__blk1042_slot = var_gmob2__blk1042;
        *var_gmob2__blk1042_dn4_slot = var_gmob2__blk1042_dn4;
        *var_gmob2__blk1042_dn6_slot = var_gmob2__blk1042_dn6;
        *var_gmob2__blk1042_dn7_slot = var_gmob2__blk1042_dn7;
        *var_gmob2__blk1042_dn8_slot = var_gmob2__blk1042_dn8;
        *var_gmob2__blk1042_dn9_slot = var_gmob2__blk1042_dn9;
        *var_gmob__blk1043_slot = var_gmob__blk1043;
        *var_gmob__blk1043_dn4_slot = var_gmob__blk1043_dn4;
        *var_gmob__blk1043_dn6_slot = var_gmob__blk1043_dn6;
        *var_gmob__blk1043_dn7_slot = var_gmob__blk1043_dn7;
        *var_gmob__blk1043_dn8_slot = var_gmob__blk1043_dn8;
        *var_gmob__blk1043_dn9_slot = var_gmob__blk1043_dn9;
        *var_guard1224_slot = var_guard1224;
        *var_guard1225_slot = var_guard1225;
        *var_guard1226_slot = var_guard1226;
        *var_guard1227_slot = var_guard1227;
        *var_guard1228_slot = var_guard1228;
        *var_guard1229_slot = var_guard1229;
        *var_guard1230_slot = var_guard1230;
        *var_hsat__blk1053_slot = var_hsat__blk1053;
        *var_hsat__blk1053_dn4_slot = var_hsat__blk1053_dn4;
        *var_hsat__blk1053_dn6_slot = var_hsat__blk1053_dn6;
        *var_hsat__blk1053_dn7_slot = var_hsat__blk1053_dn7;
        *var_hsat__blk1053_dn8_slot = var_hsat__blk1053_dn8;
        *var_hsat__blk1053_dn9_slot = var_hsat__blk1053_dn9;
        *var_inv_qimstar1__blk1044_slot = var_inv_qimstar1__blk1044;
        *var_inv_qimstar1__blk1044_dn4_slot = var_inv_qimstar1__blk1044_dn4;
        *var_inv_qimstar1__blk1044_dn6_slot = var_inv_qimstar1__blk1044_dn6;
        *var_inv_qimstar1__blk1044_dn7_slot = var_inv_qimstar1__blk1044_dn7;
        *var_inv_qimstar1__blk1044_dn8_slot = var_inv_qimstar1__blk1044_dn8;
        *var_inv_qimstar1__blk1044_dn9_slot = var_inv_qimstar1__blk1044_dn9;
        *var_qmfact1__blk1054_slot = var_qmfact1__blk1054;
        *var_qmfact1__blk1054_dn4_slot = var_qmfact1__blk1054_dn4;
        *var_qmfact1__blk1054_dn6_slot = var_qmfact1__blk1054_dn6;
        *var_qmfact1__blk1054_dn7_slot = var_qmfact1__blk1054_dn7;
        *var_qmfact1__blk1054_dn8_slot = var_qmfact1__blk1054_dn8;
        *var_qmfact1__blk1054_dn9_slot = var_qmfact1__blk1054_dn9;
        *var_qmfact2__blk1055_slot = var_qmfact2__blk1055;
        *var_qmfact2__blk1055_dn4_slot = var_qmfact2__blk1055_dn4;
        *var_qmfact2__blk1055_dn6_slot = var_qmfact2__blk1055_dn6;
        *var_qmfact2__blk1055_dn7_slot = var_qmfact2__blk1055_dn7;
        *var_qmfact2__blk1055_dn8_slot = var_qmfact2__blk1055_dn8;
        *var_qmfact2__blk1055_dn9_slot = var_qmfact2__blk1055_dn9;
        *var_r1__blk1045_slot = var_r1__blk1045;
        *var_r1__blk1045_dn4_slot = var_r1__blk1045_dn4;
        *var_r1__blk1045_dn6_slot = var_r1__blk1045_dn6;
        *var_r1__blk1045_dn7_slot = var_r1__blk1045_dn7;
        *var_r1__blk1045_dn8_slot = var_r1__blk1045_dn8;
        *var_r1__blk1045_dn9_slot = var_r1__blk1045_dn9;
        *var_sat_fact1__blk977_slot = var_sat_fact1__blk977;
        *var_sat_fact1__blk977_dn4_slot = var_sat_fact1__blk977_dn4;
        *var_sat_fact1__blk977_dn6_slot = var_sat_fact1__blk977_dn6;
        *var_sat_fact1__blk977_dn7_slot = var_sat_fact1__blk977_dn7;
        *var_sat_fact1__blk977_dn8_slot = var_sat_fact1__blk977_dn8;
        *var_sat_fact1__blk977_dn9_slot = var_sat_fact1__blk977_dn9;
        *var_sat_fact2__blk979_slot = var_sat_fact2__blk979;
        *var_sat_fact2__blk979_dn4_slot = var_sat_fact2__blk979_dn4;
        *var_sat_fact2__blk979_dn6_slot = var_sat_fact2__blk979_dn6;
        *var_sat_fact2__blk979_dn7_slot = var_sat_fact2__blk979_dn7;
        *var_sat_fact2__blk979_dn8_slot = var_sat_fact2__blk979_dn8;
        *var_sat_fact2__blk979_dn9_slot = var_sat_fact2__blk979_dn9;
        *var_sqrt_zsat__blk1050_slot = var_sqrt_zsat__blk1050;
        *var_sqrt_zsat__blk1050_dn4_slot = var_sqrt_zsat__blk1050_dn4;
        *var_sqrt_zsat__blk1050_dn6_slot = var_sqrt_zsat__blk1050_dn6;
        *var_sqrt_zsat__blk1050_dn7_slot = var_sqrt_zsat__blk1050_dn7;
        *var_sqrt_zsat__blk1050_dn8_slot = var_sqrt_zsat__blk1050_dn8;
        *var_sqrt_zsat__blk1050_dn9_slot = var_sqrt_zsat__blk1050_dn9;
        *var_temp_slot = var_temp;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_vsat_fact__blk1052_slot = var_vsat_fact__blk1052;
        *var_vsat_fact__blk1052_dn4_slot = var_vsat_fact__blk1052_dn4;
        *var_vsat_fact__blk1052_dn6_slot = var_vsat_fact__blk1052_dn6;
        *var_vsat_fact__blk1052_dn7_slot = var_vsat_fact__blk1052_dn7;
        *var_vsat_fact__blk1052_dn8_slot = var_vsat_fact__blk1052_dn8;
        *var_vsat_fact__blk1052_dn9_slot = var_vsat_fact__blk1052_dn9;
        *var_wsat1__blk976_slot = var_wsat1__blk976;
        *var_wsat1__blk976_dn4_slot = var_wsat1__blk976_dn4;
        *var_wsat1__blk976_dn6_slot = var_wsat1__blk976_dn6;
        *var_wsat1__blk976_dn7_slot = var_wsat1__blk976_dn7;
        *var_wsat1__blk976_dn8_slot = var_wsat1__blk976_dn8;
        *var_wsat1__blk976_dn9_slot = var_wsat1__blk976_dn9;
        *var_wsat2__blk978_slot = var_wsat2__blk978;
        *var_wsat2__blk978_dn4_slot = var_wsat2__blk978_dn4;
        *var_wsat2__blk978_dn6_slot = var_wsat2__blk978_dn6;
        *var_wsat2__blk978_dn7_slot = var_wsat2__blk978_dn7;
        *var_wsat2__blk978_dn8_slot = var_wsat2__blk978_dn8;
        *var_wsat2__blk978_dn9_slot = var_wsat2__blk978_dn9;
        *var_zsat__blk1051_slot = var_zsat__blk1051;
        *var_zsat__blk1051_dn4_slot = var_zsat__blk1051_dn4;
        *var_zsat__blk1051_dn6_slot = var_zsat__blk1051_dn6;
        *var_zsat__blk1051_dn7_slot = var_zsat__blk1051_dn7;
        *var_zsat__blk1051_dn8_slot = var_zsat__blk1051_dn8;
        *var_zsat__blk1051_dn9_slot = var_zsat__blk1051_dn9;
    }

    pub(super) fn stamp_transient_block_114(
        var_a1d__blk1011: f64,
        var_a1d__blk1011_dn4: f64,
        var_a1d__blk1011_dn6: f64,
        var_a1d__blk1011_dn7: f64,
        var_a1d__blk1011_dn8: f64,
        var_a1d__blk1011_dn9: f64,
        var_a1s__blk947: f64,
        var_a1s__blk947_dn4: f64,
        var_a1s__blk947_dn6: f64,
        var_a1s__blk947_dn7: f64,
        var_a1s__blk947_dn8: f64,
        var_a1s__blk947_dn9: f64,
        var_a2d__blk1012: f64,
        var_a2d__blk1012_dn4: f64,
        var_a2d__blk1012_dn6: f64,
        var_a2d__blk1012_dn7: f64,
        var_a2d__blk1012_dn8: f64,
        var_a2d__blk1012_dn9: f64,
        var_a2s__blk948: f64,
        var_a2s__blk948_dn4: f64,
        var_a2s__blk948_dn6: f64,
        var_a2s__blk948_dn7: f64,
        var_a2s__blk948_dn8: f64,
        var_a2s__blk948_dn9: f64,
        var_aexp1d__blk1007: f64,
        var_aexp1d__blk1007_dn4: f64,
        var_aexp1d__blk1007_dn6: f64,
        var_aexp1d__blk1007_dn7: f64,
        var_aexp1d__blk1007_dn8: f64,
        var_aexp1d__blk1007_dn9: f64,
        var_aexp1s__blk943: f64,
        var_aexp1s__blk943_dn4: f64,
        var_aexp1s__blk943_dn6: f64,
        var_aexp1s__blk943_dn7: f64,
        var_aexp1s__blk943_dn8: f64,
        var_aexp1s__blk943_dn9: f64,
        var_aexp2d__blk1008: f64,
        var_aexp2d__blk1008_dn4: f64,
        var_aexp2d__blk1008_dn6: f64,
        var_aexp2d__blk1008_dn7: f64,
        var_aexp2d__blk1008_dn8: f64,
        var_aexp2d__blk1008_dn9: f64,
        var_aexp2s__blk944: f64,
        var_aexp2s__blk944_dn4: f64,
        var_aexp2s__blk944_dn6: f64,
        var_aexp2s__blk944_dn7: f64,
        var_aexp2s__blk944_dn8: f64,
        var_aexp2s__blk944_dn9: f64,
        var_dinf__blk974: f64,
        var_dinf__blk974_dn4: f64,
        var_dinf__blk974_dn6: f64,
        var_dinf__blk974_dn7: f64,
        var_dinf__blk974_dn8: f64,
        var_dinf__blk974_dn9: f64,
        var_dqsqd_dxn_qi__blk1014: f64,
        var_dqsqd_dxn_qi__blk1014_dn4: f64,
        var_dqsqd_dxn_qi__blk1014_dn6: f64,
        var_dqsqd_dxn_qi__blk1014_dn7: f64,
        var_dqsqd_dxn_qi__blk1014_dn8: f64,
        var_dqsqd_dxn_qi__blk1014_dn9: f64,
        var_dqsqs_dxn_qi__blk950: f64,
        var_dqsqs_dxn_qi__blk950_dn4: f64,
        var_dqsqs_dxn_qi__blk950_dn6: f64,
        var_dqsqs_dxn_qi__blk950_dn7: f64,
        var_dqsqs_dxn_qi__blk950_dn8: f64,
        var_dqsqs_dxn_qi__blk950_dn9: f64,
        var_ds__blk981: f64,
        var_ds__blk981_dn4: f64,
        var_ds__blk981_dn6: f64,
        var_ds__blk981_dn7: f64,
        var_ds__blk981_dn8: f64,
        var_ds__blk981_dn9: f64,
        var_dxdrift__blk1017: f64,
        var_dxdrift__blk1017_dn4: f64,
        var_dxdrift__blk1017_dn6: f64,
        var_dxdrift__blk1017_dn7: f64,
        var_dxdrift__blk1017_dn8: f64,
        var_dxdrift__blk1017_dn9: f64,
        var_guard1080: f64,
        var_guard1228: f64,
        var_guard1229: f64,
        var_guard1230: f64,
        var_k2q2d__blk1005: f64,
        var_k2q2d__blk1005_dn4: f64,
        var_k2q2d__blk1005_dn6: f64,
        var_k2q2d__blk1005_dn7: f64,
        var_k2q2d__blk1005_dn8: f64,
        var_k2q2d__blk1005_dn9: f64,
        var_q2d__blk1002: f64,
        var_q2d__blk1002_dn4: f64,
        var_q2d__blk1002_dn6: f64,
        var_q2d__blk1002_dn7: f64,
        var_q2d__blk1002_dn8: f64,
        var_q2d__blk1002_dn9: f64,
        var_qid__blk1003: f64,
        var_qid__blk1003_dn4: f64,
        var_qid__blk1003_dn6: f64,
        var_qid__blk1003_dn7: f64,
        var_qid__blk1003_dn8: f64,
        var_qid__blk1003_dn9: f64,
        var_qim__blk1016: f64,
        var_qim__blk1016_dn4: f64,
        var_qim__blk1016_dn6: f64,
        var_qim__blk1016_dn7: f64,
        var_qim__blk1016_dn8: f64,
        var_qim__blk1016_dn9: f64,
        var_qis__blk938: f64,
        var_qis__blk938_dn4: f64,
        var_qis__blk938_dn6: f64,
        var_qis__blk938_dn7: f64,
        var_qis__blk938_dn8: f64,
        var_qis__blk938_dn9: f64,
        var_qsqd__blk1006: f64,
        var_qsqd__blk1006_dn4: f64,
        var_qsqd__blk1006_dn6: f64,
        var_qsqd__blk1006_dn7: f64,
        var_qsqd__blk1006_dn8: f64,
        var_qsqd__blk1006_dn9: f64,
        var_sumd__blk1013: f64,
        var_sumd__blk1013_dn4: f64,
        var_sumd__blk1013_dn6: f64,
        var_sumd__blk1013_dn7: f64,
        var_sumd__blk1013_dn8: f64,
        var_sumd__blk1013_dn9: f64,
        var_temp: f64,
        var_temp_dn4: f64,
        var_temp_dn6: f64,
        var_temp_dn7: f64,
        var_temp_dn8: f64,
        var_temp_dn9: f64,
        var_dd__blk1057_slot: &mut f64,
        var_dd__blk1057_dn4_slot: &mut f64,
        var_dd__blk1057_dn6_slot: &mut f64,
        var_dd__blk1057_dn7_slot: &mut f64,
        var_dd__blk1057_dn8_slot: &mut f64,
        var_dd__blk1057_dn9_slot: &mut f64,
        var_dqid_dxn_qi__blk1056_slot: &mut f64,
        var_dqid_dxn_qi__blk1056_dn4_slot: &mut f64,
        var_dqid_dxn_qi__blk1056_dn6_slot: &mut f64,
        var_dqid_dxn_qi__blk1056_dn7_slot: &mut f64,
        var_dqid_dxn_qi__blk1056_dn8_slot: &mut f64,
        var_dqid_dxn_qi__blk1056_dn9_slot: &mut f64,
        var_guard1231_slot: &mut f64,
        var_guard1232_slot: &mut f64,
        var_guard1233_slot: &mut f64,
        var_idrift2__blk1062_slot: &mut f64,
        var_idrift2__blk1062_dn4_slot: &mut f64,
        var_idrift2__blk1062_dn6_slot: &mut f64,
        var_idrift2__blk1062_dn7_slot: &mut f64,
        var_idrift2__blk1062_dn8_slot: &mut f64,
        var_idrift2__blk1062_dn9_slot: &mut f64,
        var_inv_k1h1_0__blk1066_slot: &mut f64,
        var_inv_k1h1_0__blk1066_dn4_slot: &mut f64,
        var_inv_k1h1_0__blk1066_dn6_slot: &mut f64,
        var_inv_k1h1_0__blk1066_dn7_slot: &mut f64,
        var_inv_k1h1_0__blk1066_dn8_slot: &mut f64,
        var_inv_k1h1_0__blk1066_dn9_slot: &mut f64,
        var_inv_k2h2_0__blk1069_slot: &mut f64,
        var_inv_k2h2_0__blk1069_dn4_slot: &mut f64,
        var_inv_k2h2_0__blk1069_dn6_slot: &mut f64,
        var_inv_k2h2_0__blk1069_dn7_slot: &mut f64,
        var_inv_k2h2_0__blk1069_dn8_slot: &mut f64,
        var_inv_k2h2_0__blk1069_dn9_slot: &mut f64,
        var_ld__blk1059_slot: &mut f64,
        var_ld__blk1059_dn4_slot: &mut f64,
        var_ld__blk1059_dn6_slot: &mut f64,
        var_ld__blk1059_dn7_slot: &mut f64,
        var_ld__blk1059_dn8_slot: &mut f64,
        var_ld__blk1059_dn9_slot: &mut f64,
        var_ls__blk1058_slot: &mut f64,
        var_ls__blk1058_dn4_slot: &mut f64,
        var_ls__blk1058_dn6_slot: &mut f64,
        var_ls__blk1058_dn7_slot: &mut f64,
        var_ls__blk1058_dn8_slot: &mut f64,
        var_ls__blk1058_dn9_slot: &mut f64,
        var_norm_ids__blk1063_slot: &mut f64,
        var_norm_ids__blk1063_dn4_slot: &mut f64,
        var_norm_ids__blk1063_dn6_slot: &mut f64,
        var_norm_ids__blk1063_dn7_slot: &mut f64,
        var_norm_ids__blk1063_dn8_slot: &mut f64,
        var_norm_ids__blk1063_dn9_slot: &mut f64,
        var_q1d_chap__blk1065_slot: &mut f64,
        var_q1d_chap__blk1065_dn4_slot: &mut f64,
        var_q1d_chap__blk1065_dn6_slot: &mut f64,
        var_q1d_chap__blk1065_dn7_slot: &mut f64,
        var_q1d_chap__blk1065_dn8_slot: &mut f64,
        var_q1d_chap__blk1065_dn9_slot: &mut f64,
        var_q1s_chap__blk1064_slot: &mut f64,
        var_q1s_chap__blk1064_dn4_slot: &mut f64,
        var_q1s_chap__blk1064_dn6_slot: &mut f64,
        var_q1s_chap__blk1064_dn7_slot: &mut f64,
        var_q1s_chap__blk1064_dn8_slot: &mut f64,
        var_q1s_chap__blk1064_dn9_slot: &mut f64,
        var_q2d_chap__blk1068_slot: &mut f64,
        var_q2d_chap__blk1068_dn4_slot: &mut f64,
        var_q2d_chap__blk1068_dn6_slot: &mut f64,
        var_q2d_chap__blk1068_dn7_slot: &mut f64,
        var_q2d_chap__blk1068_dn8_slot: &mut f64,
        var_q2d_chap__blk1068_dn9_slot: &mut f64,
        var_q2s_chap__blk1067_slot: &mut f64,
        var_q2s_chap__blk1067_dn4_slot: &mut f64,
        var_q2s_chap__blk1067_dn6_slot: &mut f64,
        var_q2s_chap__blk1067_dn7_slot: &mut f64,
        var_q2s_chap__blk1067_dn8_slot: &mut f64,
        var_q2s_chap__blk1067_dn9_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp3_slot: &mut f64,
        var_temp3_dn4_slot: &mut f64,
        var_temp3_dn6_slot: &mut f64,
        var_temp3_dn7_slot: &mut f64,
        var_temp3_dn8_slot: &mut f64,
        var_temp3_dn9_slot: &mut f64,
        var_temp4_slot: &mut f64,
        var_temp4_dn4_slot: &mut f64,
        var_temp4_dn6_slot: &mut f64,
        var_temp4_dn7_slot: &mut f64,
        var_temp4_dn8_slot: &mut f64,
        var_temp4_dn9_slot: &mut f64,
        var_ud__blk1061_slot: &mut f64,
        var_ud__blk1061_dn4_slot: &mut f64,
        var_ud__blk1061_dn6_slot: &mut f64,
        var_ud__blk1061_dn7_slot: &mut f64,
        var_ud__blk1061_dn8_slot: &mut f64,
        var_ud__blk1061_dn9_slot: &mut f64,
        var_us__blk1060_slot: &mut f64,
        var_us__blk1060_dn4_slot: &mut f64,
        var_us__blk1060_dn6_slot: &mut f64,
        var_us__blk1060_dn7_slot: &mut f64,
        var_us__blk1060_dn8_slot: &mut f64,
        var_us__blk1060_dn9_slot: &mut f64,
    ) {
        let mut var_dd__blk1057: f64 = *var_dd__blk1057_slot;
        let mut var_dd__blk1057_dn4: f64 = *var_dd__blk1057_dn4_slot;
        let mut var_dd__blk1057_dn6: f64 = *var_dd__blk1057_dn6_slot;
        let mut var_dd__blk1057_dn7: f64 = *var_dd__blk1057_dn7_slot;
        let mut var_dd__blk1057_dn8: f64 = *var_dd__blk1057_dn8_slot;
        let mut var_dd__blk1057_dn9: f64 = *var_dd__blk1057_dn9_slot;
        let mut var_dqid_dxn_qi__blk1056: f64 = *var_dqid_dxn_qi__blk1056_slot;
        let mut var_dqid_dxn_qi__blk1056_dn4: f64 = *var_dqid_dxn_qi__blk1056_dn4_slot;
        let mut var_dqid_dxn_qi__blk1056_dn6: f64 = *var_dqid_dxn_qi__blk1056_dn6_slot;
        let mut var_dqid_dxn_qi__blk1056_dn7: f64 = *var_dqid_dxn_qi__blk1056_dn7_slot;
        let mut var_dqid_dxn_qi__blk1056_dn8: f64 = *var_dqid_dxn_qi__blk1056_dn8_slot;
        let mut var_dqid_dxn_qi__blk1056_dn9: f64 = *var_dqid_dxn_qi__blk1056_dn9_slot;
        let mut var_guard1231: f64 = *var_guard1231_slot;
        let mut var_guard1232: f64 = *var_guard1232_slot;
        let mut var_guard1233: f64 = *var_guard1233_slot;
        let mut var_idrift2__blk1062: f64 = *var_idrift2__blk1062_slot;
        let mut var_idrift2__blk1062_dn4: f64 = *var_idrift2__blk1062_dn4_slot;
        let mut var_idrift2__blk1062_dn6: f64 = *var_idrift2__blk1062_dn6_slot;
        let mut var_idrift2__blk1062_dn7: f64 = *var_idrift2__blk1062_dn7_slot;
        let mut var_idrift2__blk1062_dn8: f64 = *var_idrift2__blk1062_dn8_slot;
        let mut var_idrift2__blk1062_dn9: f64 = *var_idrift2__blk1062_dn9_slot;
        let mut var_inv_k1h1_0__blk1066: f64 = *var_inv_k1h1_0__blk1066_slot;
        let mut var_inv_k1h1_0__blk1066_dn4: f64 = *var_inv_k1h1_0__blk1066_dn4_slot;
        let mut var_inv_k1h1_0__blk1066_dn6: f64 = *var_inv_k1h1_0__blk1066_dn6_slot;
        let mut var_inv_k1h1_0__blk1066_dn7: f64 = *var_inv_k1h1_0__blk1066_dn7_slot;
        let mut var_inv_k1h1_0__blk1066_dn8: f64 = *var_inv_k1h1_0__blk1066_dn8_slot;
        let mut var_inv_k1h1_0__blk1066_dn9: f64 = *var_inv_k1h1_0__blk1066_dn9_slot;
        let mut var_inv_k2h2_0__blk1069: f64 = *var_inv_k2h2_0__blk1069_slot;
        let mut var_inv_k2h2_0__blk1069_dn4: f64 = *var_inv_k2h2_0__blk1069_dn4_slot;
        let mut var_inv_k2h2_0__blk1069_dn6: f64 = *var_inv_k2h2_0__blk1069_dn6_slot;
        let mut var_inv_k2h2_0__blk1069_dn7: f64 = *var_inv_k2h2_0__blk1069_dn7_slot;
        let mut var_inv_k2h2_0__blk1069_dn8: f64 = *var_inv_k2h2_0__blk1069_dn8_slot;
        let mut var_inv_k2h2_0__blk1069_dn9: f64 = *var_inv_k2h2_0__blk1069_dn9_slot;
        let mut var_ld__blk1059: f64 = *var_ld__blk1059_slot;
        let mut var_ld__blk1059_dn4: f64 = *var_ld__blk1059_dn4_slot;
        let mut var_ld__blk1059_dn6: f64 = *var_ld__blk1059_dn6_slot;
        let mut var_ld__blk1059_dn7: f64 = *var_ld__blk1059_dn7_slot;
        let mut var_ld__blk1059_dn8: f64 = *var_ld__blk1059_dn8_slot;
        let mut var_ld__blk1059_dn9: f64 = *var_ld__blk1059_dn9_slot;
        let mut var_ls__blk1058: f64 = *var_ls__blk1058_slot;
        let mut var_ls__blk1058_dn4: f64 = *var_ls__blk1058_dn4_slot;
        let mut var_ls__blk1058_dn6: f64 = *var_ls__blk1058_dn6_slot;
        let mut var_ls__blk1058_dn7: f64 = *var_ls__blk1058_dn7_slot;
        let mut var_ls__blk1058_dn8: f64 = *var_ls__blk1058_dn8_slot;
        let mut var_ls__blk1058_dn9: f64 = *var_ls__blk1058_dn9_slot;
        let mut var_norm_ids__blk1063: f64 = *var_norm_ids__blk1063_slot;
        let mut var_norm_ids__blk1063_dn4: f64 = *var_norm_ids__blk1063_dn4_slot;
        let mut var_norm_ids__blk1063_dn6: f64 = *var_norm_ids__blk1063_dn6_slot;
        let mut var_norm_ids__blk1063_dn7: f64 = *var_norm_ids__blk1063_dn7_slot;
        let mut var_norm_ids__blk1063_dn8: f64 = *var_norm_ids__blk1063_dn8_slot;
        let mut var_norm_ids__blk1063_dn9: f64 = *var_norm_ids__blk1063_dn9_slot;
        let mut var_q1d_chap__blk1065: f64 = *var_q1d_chap__blk1065_slot;
        let mut var_q1d_chap__blk1065_dn4: f64 = *var_q1d_chap__blk1065_dn4_slot;
        let mut var_q1d_chap__blk1065_dn6: f64 = *var_q1d_chap__blk1065_dn6_slot;
        let mut var_q1d_chap__blk1065_dn7: f64 = *var_q1d_chap__blk1065_dn7_slot;
        let mut var_q1d_chap__blk1065_dn8: f64 = *var_q1d_chap__blk1065_dn8_slot;
        let mut var_q1d_chap__blk1065_dn9: f64 = *var_q1d_chap__blk1065_dn9_slot;
        let mut var_q1s_chap__blk1064: f64 = *var_q1s_chap__blk1064_slot;
        let mut var_q1s_chap__blk1064_dn4: f64 = *var_q1s_chap__blk1064_dn4_slot;
        let mut var_q1s_chap__blk1064_dn6: f64 = *var_q1s_chap__blk1064_dn6_slot;
        let mut var_q1s_chap__blk1064_dn7: f64 = *var_q1s_chap__blk1064_dn7_slot;
        let mut var_q1s_chap__blk1064_dn8: f64 = *var_q1s_chap__blk1064_dn8_slot;
        let mut var_q1s_chap__blk1064_dn9: f64 = *var_q1s_chap__blk1064_dn9_slot;
        let mut var_q2d_chap__blk1068: f64 = *var_q2d_chap__blk1068_slot;
        let mut var_q2d_chap__blk1068_dn4: f64 = *var_q2d_chap__blk1068_dn4_slot;
        let mut var_q2d_chap__blk1068_dn6: f64 = *var_q2d_chap__blk1068_dn6_slot;
        let mut var_q2d_chap__blk1068_dn7: f64 = *var_q2d_chap__blk1068_dn7_slot;
        let mut var_q2d_chap__blk1068_dn8: f64 = *var_q2d_chap__blk1068_dn8_slot;
        let mut var_q2d_chap__blk1068_dn9: f64 = *var_q2d_chap__blk1068_dn9_slot;
        let mut var_q2s_chap__blk1067: f64 = *var_q2s_chap__blk1067_slot;
        let mut var_q2s_chap__blk1067_dn4: f64 = *var_q2s_chap__blk1067_dn4_slot;
        let mut var_q2s_chap__blk1067_dn6: f64 = *var_q2s_chap__blk1067_dn6_slot;
        let mut var_q2s_chap__blk1067_dn7: f64 = *var_q2s_chap__blk1067_dn7_slot;
        let mut var_q2s_chap__blk1067_dn8: f64 = *var_q2s_chap__blk1067_dn8_slot;
        let mut var_q2s_chap__blk1067_dn9: f64 = *var_q2s_chap__blk1067_dn9_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp3: f64 = *var_temp3_slot;
        let mut var_temp3_dn4: f64 = *var_temp3_dn4_slot;
        let mut var_temp3_dn6: f64 = *var_temp3_dn6_slot;
        let mut var_temp3_dn7: f64 = *var_temp3_dn7_slot;
        let mut var_temp3_dn8: f64 = *var_temp3_dn8_slot;
        let mut var_temp3_dn9: f64 = *var_temp3_dn9_slot;
        let mut var_temp4: f64 = *var_temp4_slot;
        let mut var_temp4_dn4: f64 = *var_temp4_dn4_slot;
        let mut var_temp4_dn6: f64 = *var_temp4_dn6_slot;
        let mut var_temp4_dn7: f64 = *var_temp4_dn7_slot;
        let mut var_temp4_dn8: f64 = *var_temp4_dn8_slot;
        let mut var_temp4_dn9: f64 = *var_temp4_dn9_slot;
        let mut var_ud__blk1061: f64 = *var_ud__blk1061_slot;
        let mut var_ud__blk1061_dn4: f64 = *var_ud__blk1061_dn4_slot;
        let mut var_ud__blk1061_dn6: f64 = *var_ud__blk1061_dn6_slot;
        let mut var_ud__blk1061_dn7: f64 = *var_ud__blk1061_dn7_slot;
        let mut var_ud__blk1061_dn8: f64 = *var_ud__blk1061_dn8_slot;
        let mut var_ud__blk1061_dn9: f64 = *var_ud__blk1061_dn9_slot;
        let mut var_us__blk1060: f64 = *var_us__blk1060_slot;
        let mut var_us__blk1060_dn4: f64 = *var_us__blk1060_dn4_slot;
        let mut var_us__blk1060_dn6: f64 = *var_us__blk1060_dn6_slot;
        let mut var_us__blk1060_dn7: f64 = *var_us__blk1060_dn7_slot;
        let mut var_us__blk1060_dn8: f64 = *var_us__blk1060_dn8_slot;
        let mut var_us__blk1060_dn9: f64 = *var_us__blk1060_dn9_slot;

        let (assign40940_e46726, assign40940_e46726_d_n4, assign40940_e46726_d_n6, assign40940_e46726_d_n7, assign40940_e46726_d_n8, assign40940_e46726_d_n9,) = {
    if ((((var_guard1080 != 0.0) && (var_guard1228 != 0.0)) && (var_guard1229 != 0.0)) && (var_guard1230 != 0.0)) {
        let assign40940_e46724: f64 = (var_temp1 * var_temp1);
        (assign40940_e46724, ((var_temp1_dn4 * var_temp1) + (var_temp1 * var_temp1_dn4)), ((var_temp1_dn6 * var_temp1) + (var_temp1 * var_temp1_dn6)), ((var_temp1_dn7 * var_temp1) + (var_temp1 * var_temp1_dn7)), ((var_temp1_dn8 * var_temp1) + (var_temp1 * var_temp1_dn8)), ((var_temp1_dn9 * var_temp1) + (var_temp1 * var_temp1_dn9)),)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign40940_e46726;
        var_temp2_dn4 = assign40940_e46726_d_n4;
        var_temp2_dn6 = assign40940_e46726_d_n6;
        var_temp2_dn7 = assign40940_e46726_d_n7;
        var_temp2_dn8 = assign40940_e46726_d_n8;
        var_temp2_dn9 = assign40940_e46726_d_n9;

        let (assign40950_e46740, assign40950_e46740_d_n4, assign40950_e46740_d_n6, assign40950_e46740_d_n7, assign40950_e46740_d_n8, assign40950_e46740_d_n9,) = {
    if ((((var_guard1080 != 0.0) && (var_guard1228 != 0.0)) && (var_guard1229 != 0.0)) && (var_guard1230 != 0.0)) {
        let assign40950_e46736: f64 = (1.0 - var_temp1);
        let assign40950_e46738: f64 = (assign40950_e46736 + var_temp2);
        (assign40950_e46738, ((-var_temp1_dn4) + var_temp2_dn4), ((-var_temp1_dn6) + var_temp2_dn6), ((-var_temp1_dn7) + var_temp2_dn7), ((-var_temp1_dn8) + var_temp2_dn8), ((-var_temp1_dn9) + var_temp2_dn9),)
    } else {
        (var_temp3, var_temp3_dn4, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9,)
    }
};
        var_temp3 = assign40950_e46740;
        var_temp3_dn4 = assign40950_e46740_d_n4;
        var_temp3_dn6 = assign40950_e46740_d_n6;
        var_temp3_dn7 = assign40950_e46740_d_n7;
        var_temp3_dn8 = assign40950_e46740_d_n8;
        var_temp3_dn9 = assign40950_e46740_d_n9;

        let (assign40960_e46754, assign40960_e46754_d_n4, assign40960_e46754_d_n6, assign40960_e46754_d_n7, assign40960_e46754_d_n8, assign40960_e46754_d_n9,) = {
    if ((((var_guard1080 != 0.0) && (var_guard1228 != 0.0)) && (var_guard1229 != 0.0)) && (var_guard1230 != 0.0)) {
        let assign40960_e46751: f64 = (var_temp1 * var_temp2);
        let assign40960_e46752: f64 = (var_temp3 - assign40960_e46751);
        (assign40960_e46752, (var_temp3_dn4 - ((var_temp1_dn4 * var_temp2) + (var_temp1 * var_temp2_dn4))), (var_temp3_dn6 - ((var_temp1_dn6 * var_temp2) + (var_temp1 * var_temp2_dn6))), (var_temp3_dn7 - ((var_temp1_dn7 * var_temp2) + (var_temp1 * var_temp2_dn7))), (var_temp3_dn8 - ((var_temp1_dn8 * var_temp2) + (var_temp1 * var_temp2_dn8))), (var_temp3_dn9 - ((var_temp1_dn9 * var_temp2) + (var_temp1 * var_temp2_dn9))),)
    } else {
        (var_temp4, var_temp4_dn4, var_temp4_dn6, var_temp4_dn7, var_temp4_dn8, var_temp4_dn9,)
    }
};
        var_temp4 = assign40960_e46754;
        var_temp4_dn4 = assign40960_e46754_d_n4;
        var_temp4_dn6 = assign40960_e46754_d_n6;
        var_temp4_dn7 = assign40960_e46754_d_n7;
        var_temp4_dn8 = assign40960_e46754_d_n8;
        var_temp4_dn9 = assign40960_e46754_d_n9;

        let (assign40970_e46780, assign40970_e46780_d_n4, assign40970_e46780_d_n6, assign40970_e46780_d_n7, assign40970_e46780_d_n8, assign40970_e46780_d_n9,) = {
    if ((((var_guard1080 != 0.0) && (var_guard1228 != 0.0)) && (var_guard1229 != 0.0)) && (var_guard1230 != 0.0)) {
        let assign40970_e46765: f64 = (2.0 * var_qsqd__blk1006);
        let assign40970_e46769: f64 = (1.0 / var_a1d__blk1011);
        let assign40970_e46770: f64 = (var_temp - assign40970_e46769);
        let assign40970_e46771: f64 = (assign40970_e46765 * assign40970_e46770);
        let assign40970_e46773: f64 = (assign40970_e46771 * var_temp4);
        let assign40970_e46774: f64 = (var_k2q2d__blk1005 - assign40970_e46773);
        let assign40970_e46777: f64 = (2.0 + var_q2d__blk1002);
        let assign40970_e46778: f64 = (assign40970_e46774 / assign40970_e46777);
        (assign40970_e46778, ((((var_k2q2d__blk1005_dn4 - (((((2.0 * var_qsqd__blk1006_dn4) * assign40970_e46770) + (assign40970_e46765 * (var_temp_dn4 - (-(var_a1d__blk1011_dn4 / (var_a1d__blk1011 * var_a1d__blk1011)))))) * var_temp4) + (assign40970_e46771 * var_temp4_dn4))) * assign40970_e46777) - (assign40970_e46774 * var_q2d__blk1002_dn4)) / (assign40970_e46777 * assign40970_e46777)), ((((var_k2q2d__blk1005_dn6 - (((((2.0 * var_qsqd__blk1006_dn6) * assign40970_e46770) + (assign40970_e46765 * (var_temp_dn6 - (-(var_a1d__blk1011_dn6 / (var_a1d__blk1011 * var_a1d__blk1011)))))) * var_temp4) + (assign40970_e46771 * var_temp4_dn6))) * assign40970_e46777) - (assign40970_e46774 * var_q2d__blk1002_dn6)) / (assign40970_e46777 * assign40970_e46777)), ((((var_k2q2d__blk1005_dn7 - (((((2.0 * var_qsqd__blk1006_dn7) * assign40970_e46770) + (assign40970_e46765 * (var_temp_dn7 - (-(var_a1d__blk1011_dn7 / (var_a1d__blk1011 * var_a1d__blk1011)))))) * var_temp4) + (assign40970_e46771 * var_temp4_dn7))) * assign40970_e46777) - (assign40970_e46774 * var_q2d__blk1002_dn7)) / (assign40970_e46777 * assign40970_e46777)), ((((var_k2q2d__blk1005_dn8 - (((((2.0 * var_qsqd__blk1006_dn8) * assign40970_e46770) + (assign40970_e46765 * (var_temp_dn8 - (-(var_a1d__blk1011_dn8 / (var_a1d__blk1011 * var_a1d__blk1011)))))) * var_temp4) + (assign40970_e46771 * var_temp4_dn8))) * assign40970_e46777) - (assign40970_e46774 * var_q2d__blk1002_dn8)) / (assign40970_e46777 * assign40970_e46777)), ((((var_k2q2d__blk1005_dn9 - (((((2.0 * var_qsqd__blk1006_dn9) * assign40970_e46770) + (assign40970_e46765 * (var_temp_dn9 - (-(var_a1d__blk1011_dn9 / (var_a1d__blk1011 * var_a1d__blk1011)))))) * var_temp4) + (assign40970_e46771 * var_temp4_dn9))) * assign40970_e46777) - (assign40970_e46774 * var_q2d__blk1002_dn9)) / (assign40970_e46777 * assign40970_e46777)),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign40970_e46780;
        var_temp1_dn4 = assign40970_e46780_d_n4;
        var_temp1_dn6 = assign40970_e46780_d_n6;
        var_temp1_dn7 = assign40970_e46780_d_n7;
        var_temp1_dn8 = assign40970_e46780_d_n8;
        var_temp1_dn9 = assign40970_e46780_d_n9;

        let (assign40980_e46800, assign40980_e46800_d_n4, assign40980_e46800_d_n6, assign40980_e46800_d_n7, assign40980_e46800_d_n8, assign40980_e46800_d_n9,) = {
    if ((((var_guard1080 != 0.0) && (var_guard1228 != 0.0)) && (var_guard1229 != 0.0)) && (var_guard1230 != 0.0)) {
        let assign40980_e46790: f64 = (var_dqsqd_dxn_qi__blk1014 * var_qid__blk1003);
        let assign40980_e46792: f64 = (assign40980_e46790 - var_aexp1d__blk1007);
        let assign40980_e46794: f64 = (assign40980_e46792 / var_a1d__blk1011);
        let assign40980_e46796: f64 = (assign40980_e46794 - var_temp1);
        let assign40980_e46798: f64 = (assign40980_e46796 / var_qid__blk1003);
        (assign40980_e46798, ((((((((((var_dqsqd_dxn_qi__blk1014_dn4 * var_qid__blk1003) + (var_dqsqd_dxn_qi__blk1014 * var_qid__blk1003_dn4)) - var_aexp1d__blk1007_dn4) * var_a1d__blk1011) - (assign40980_e46792 * var_a1d__blk1011_dn4)) / (var_a1d__blk1011 * var_a1d__blk1011)) - var_temp1_dn4) * var_qid__blk1003) - (assign40980_e46796 * var_qid__blk1003_dn4)) / (var_qid__blk1003 * var_qid__blk1003)), ((((((((((var_dqsqd_dxn_qi__blk1014_dn6 * var_qid__blk1003) + (var_dqsqd_dxn_qi__blk1014 * var_qid__blk1003_dn6)) - var_aexp1d__blk1007_dn6) * var_a1d__blk1011) - (assign40980_e46792 * var_a1d__blk1011_dn6)) / (var_a1d__blk1011 * var_a1d__blk1011)) - var_temp1_dn6) * var_qid__blk1003) - (assign40980_e46796 * var_qid__blk1003_dn6)) / (var_qid__blk1003 * var_qid__blk1003)), ((((((((((var_dqsqd_dxn_qi__blk1014_dn7 * var_qid__blk1003) + (var_dqsqd_dxn_qi__blk1014 * var_qid__blk1003_dn7)) - var_aexp1d__blk1007_dn7) * var_a1d__blk1011) - (assign40980_e46792 * var_a1d__blk1011_dn7)) / (var_a1d__blk1011 * var_a1d__blk1011)) - var_temp1_dn7) * var_qid__blk1003) - (assign40980_e46796 * var_qid__blk1003_dn7)) / (var_qid__blk1003 * var_qid__blk1003)), ((((((((((var_dqsqd_dxn_qi__blk1014_dn8 * var_qid__blk1003) + (var_dqsqd_dxn_qi__blk1014 * var_qid__blk1003_dn8)) - var_aexp1d__blk1007_dn8) * var_a1d__blk1011) - (assign40980_e46792 * var_a1d__blk1011_dn8)) / (var_a1d__blk1011 * var_a1d__blk1011)) - var_temp1_dn8) * var_qid__blk1003) - (assign40980_e46796 * var_qid__blk1003_dn8)) / (var_qid__blk1003 * var_qid__blk1003)), ((((((((((var_dqsqd_dxn_qi__blk1014_dn9 * var_qid__blk1003) + (var_dqsqd_dxn_qi__blk1014 * var_qid__blk1003_dn9)) - var_aexp1d__blk1007_dn9) * var_a1d__blk1011) - (assign40980_e46792 * var_a1d__blk1011_dn9)) / (var_a1d__blk1011 * var_a1d__blk1011)) - var_temp1_dn9) * var_qid__blk1003) - (assign40980_e46796 * var_qid__blk1003_dn9)) / (var_qid__blk1003 * var_qid__blk1003)),)
    } else {
        (var_dqid_dxn_qi__blk1056, var_dqid_dxn_qi__blk1056_dn4, var_dqid_dxn_qi__blk1056_dn6, var_dqid_dxn_qi__blk1056_dn7, var_dqid_dxn_qi__blk1056_dn8, var_dqid_dxn_qi__blk1056_dn9,)
    }
};
        var_dqid_dxn_qi__blk1056 = assign40980_e46800;
        var_dqid_dxn_qi__blk1056_dn4 = assign40980_e46800_d_n4;
        var_dqid_dxn_qi__blk1056_dn6 = assign40980_e46800_d_n6;
        var_dqid_dxn_qi__blk1056_dn7 = assign40980_e46800_d_n7;
        var_dqid_dxn_qi__blk1056_dn8 = assign40980_e46800_d_n8;
        var_dqid_dxn_qi__blk1056_dn9 = assign40980_e46800_d_n9;

        let (assign40990_e46816, assign40990_e46816_d_n4, assign40990_e46816_d_n6, assign40990_e46816_d_n7, assign40990_e46816_d_n8, assign40990_e46816_d_n9,) = {
    if ((((var_guard1080 != 0.0) && (var_guard1228 != 0.0)) && (var_guard1229 != 0.0)) && (var_guard1230 != 0.0)) {
        let assign40990_e46810: f64 = (var_dqid_dxn_qi__blk1056 * var_qid__blk1003);
        let assign40990_e46813: f64 = (var_dqid_dxn_qi__blk1056 + 1.0);
        let assign40990_e46814: f64 = (assign40990_e46810 / assign40990_e46813);
        (assign40990_e46814, (((((var_dqid_dxn_qi__blk1056_dn4 * var_qid__blk1003) + (var_dqid_dxn_qi__blk1056 * var_qid__blk1003_dn4)) * assign40990_e46813) - (assign40990_e46810 * var_dqid_dxn_qi__blk1056_dn4)) / (assign40990_e46813 * assign40990_e46813)), (((((var_dqid_dxn_qi__blk1056_dn6 * var_qid__blk1003) + (var_dqid_dxn_qi__blk1056 * var_qid__blk1003_dn6)) * assign40990_e46813) - (assign40990_e46810 * var_dqid_dxn_qi__blk1056_dn6)) / (assign40990_e46813 * assign40990_e46813)), (((((var_dqid_dxn_qi__blk1056_dn7 * var_qid__blk1003) + (var_dqid_dxn_qi__blk1056 * var_qid__blk1003_dn7)) * assign40990_e46813) - (assign40990_e46810 * var_dqid_dxn_qi__blk1056_dn7)) / (assign40990_e46813 * assign40990_e46813)), (((((var_dqid_dxn_qi__blk1056_dn8 * var_qid__blk1003) + (var_dqid_dxn_qi__blk1056 * var_qid__blk1003_dn8)) * assign40990_e46813) - (assign40990_e46810 * var_dqid_dxn_qi__blk1056_dn8)) / (assign40990_e46813 * assign40990_e46813)), (((((var_dqid_dxn_qi__blk1056_dn9 * var_qid__blk1003) + (var_dqid_dxn_qi__blk1056 * var_qid__blk1003_dn9)) * assign40990_e46813) - (assign40990_e46810 * var_dqid_dxn_qi__blk1056_dn9)) / (assign40990_e46813 * assign40990_e46813)),)
    } else {
        (var_dd__blk1057, var_dd__blk1057_dn4, var_dd__blk1057_dn6, var_dd__blk1057_dn7, var_dd__blk1057_dn8, var_dd__blk1057_dn9,)
    }
};
        var_dd__blk1057 = assign40990_e46816;
        var_dd__blk1057_dn4 = assign40990_e46816_d_n4;
        var_dd__blk1057_dn6 = assign40990_e46816_d_n6;
        var_dd__blk1057_dn7 = assign40990_e46816_d_n7;
        var_dd__blk1057_dn8 = assign40990_e46816_d_n8;
        var_dd__blk1057_dn9 = assign40990_e46816_d_n9;

        let (assign41000_e46843, assign41000_e46843_d_n4, assign41000_e46843_d_n6, assign41000_e46843_d_n7, assign41000_e46843_d_n8, assign41000_e46843_d_n9,) = {
    if ((((var_guard1080 != 0.0) && (var_guard1228 != 0.0)) && (var_guard1229 != 0.0)) && (var_guard1230 == 0.0)) {
        let assign41000_e46827: f64 = (var_dqsqd_dxn_qi__blk1014 * var_sumd__blk1013);
        let assign41000_e46830: f64 = (var_a1d__blk1011 * var_a2d__blk1012);
        let assign41000_e46831: f64 = (assign41000_e46827 / assign41000_e46830);
        let assign41000_e46834: f64 = (var_aexp1d__blk1007 / var_a1d__blk1011);
        let assign41000_e46837: f64 = (var_aexp2d__blk1008 / var_a2d__blk1012);
        let assign41000_e46838: f64 = (assign41000_e46834 + assign41000_e46837);
        let assign41000_e46840: f64 = (assign41000_e46838 / var_qid__blk1003);
        let assign41000_e46841: f64 = (assign41000_e46831 - assign41000_e46840);
        (assign41000_e46841, ((((((var_dqsqd_dxn_qi__blk1014_dn4 * var_sumd__blk1013) + (var_dqsqd_dxn_qi__blk1014 * var_sumd__blk1013_dn4)) * assign41000_e46830) - (assign41000_e46827 * ((var_a1d__blk1011_dn4 * var_a2d__blk1012) + (var_a1d__blk1011 * var_a2d__blk1012_dn4)))) / (assign41000_e46830 * assign41000_e46830)) - (((((((var_aexp1d__blk1007_dn4 * var_a1d__blk1011) - (var_aexp1d__blk1007 * var_a1d__blk1011_dn4)) / (var_a1d__blk1011 * var_a1d__blk1011)) + (((var_aexp2d__blk1008_dn4 * var_a2d__blk1012) - (var_aexp2d__blk1008 * var_a2d__blk1012_dn4)) / (var_a2d__blk1012 * var_a2d__blk1012))) * var_qid__blk1003) - (assign41000_e46838 * var_qid__blk1003_dn4)) / (var_qid__blk1003 * var_qid__blk1003))), ((((((var_dqsqd_dxn_qi__blk1014_dn6 * var_sumd__blk1013) + (var_dqsqd_dxn_qi__blk1014 * var_sumd__blk1013_dn6)) * assign41000_e46830) - (assign41000_e46827 * ((var_a1d__blk1011_dn6 * var_a2d__blk1012) + (var_a1d__blk1011 * var_a2d__blk1012_dn6)))) / (assign41000_e46830 * assign41000_e46830)) - (((((((var_aexp1d__blk1007_dn6 * var_a1d__blk1011) - (var_aexp1d__blk1007 * var_a1d__blk1011_dn6)) / (var_a1d__blk1011 * var_a1d__blk1011)) + (((var_aexp2d__blk1008_dn6 * var_a2d__blk1012) - (var_aexp2d__blk1008 * var_a2d__blk1012_dn6)) / (var_a2d__blk1012 * var_a2d__blk1012))) * var_qid__blk1003) - (assign41000_e46838 * var_qid__blk1003_dn6)) / (var_qid__blk1003 * var_qid__blk1003))), ((((((var_dqsqd_dxn_qi__blk1014_dn7 * var_sumd__blk1013) + (var_dqsqd_dxn_qi__blk1014 * var_sumd__blk1013_dn7)) * assign41000_e46830) - (assign41000_e46827 * ((var_a1d__blk1011_dn7 * var_a2d__blk1012) + (var_a1d__blk1011 * var_a2d__blk1012_dn7)))) / (assign41000_e46830 * assign41000_e46830)) - (((((((var_aexp1d__blk1007_dn7 * var_a1d__blk1011) - (var_aexp1d__blk1007 * var_a1d__blk1011_dn7)) / (var_a1d__blk1011 * var_a1d__blk1011)) + (((var_aexp2d__blk1008_dn7 * var_a2d__blk1012) - (var_aexp2d__blk1008 * var_a2d__blk1012_dn7)) / (var_a2d__blk1012 * var_a2d__blk1012))) * var_qid__blk1003) - (assign41000_e46838 * var_qid__blk1003_dn7)) / (var_qid__blk1003 * var_qid__blk1003))), ((((((var_dqsqd_dxn_qi__blk1014_dn8 * var_sumd__blk1013) + (var_dqsqd_dxn_qi__blk1014 * var_sumd__blk1013_dn8)) * assign41000_e46830) - (assign41000_e46827 * ((var_a1d__blk1011_dn8 * var_a2d__blk1012) + (var_a1d__blk1011 * var_a2d__blk1012_dn8)))) / (assign41000_e46830 * assign41000_e46830)) - (((((((var_aexp1d__blk1007_dn8 * var_a1d__blk1011) - (var_aexp1d__blk1007 * var_a1d__blk1011_dn8)) / (var_a1d__blk1011 * var_a1d__blk1011)) + (((var_aexp2d__blk1008_dn8 * var_a2d__blk1012) - (var_aexp2d__blk1008 * var_a2d__blk1012_dn8)) / (var_a2d__blk1012 * var_a2d__blk1012))) * var_qid__blk1003) - (assign41000_e46838 * var_qid__blk1003_dn8)) / (var_qid__blk1003 * var_qid__blk1003))), ((((((var_dqsqd_dxn_qi__blk1014_dn9 * var_sumd__blk1013) + (var_dqsqd_dxn_qi__blk1014 * var_sumd__blk1013_dn9)) * assign41000_e46830) - (assign41000_e46827 * ((var_a1d__blk1011_dn9 * var_a2d__blk1012) + (var_a1d__blk1011 * var_a2d__blk1012_dn9)))) / (assign41000_e46830 * assign41000_e46830)) - (((((((var_aexp1d__blk1007_dn9 * var_a1d__blk1011) - (var_aexp1d__blk1007 * var_a1d__blk1011_dn9)) / (var_a1d__blk1011 * var_a1d__blk1011)) + (((var_aexp2d__blk1008_dn9 * var_a2d__blk1012) - (var_aexp2d__blk1008 * var_a2d__blk1012_dn9)) / (var_a2d__blk1012 * var_a2d__blk1012))) * var_qid__blk1003) - (assign41000_e46838 * var_qid__blk1003_dn9)) / (var_qid__blk1003 * var_qid__blk1003))),)
    } else {
        (var_dqid_dxn_qi__blk1056, var_dqid_dxn_qi__blk1056_dn4, var_dqid_dxn_qi__blk1056_dn6, var_dqid_dxn_qi__blk1056_dn7, var_dqid_dxn_qi__blk1056_dn8, var_dqid_dxn_qi__blk1056_dn9,)
    }
};
        var_dqid_dxn_qi__blk1056 = assign41000_e46843;
        var_dqid_dxn_qi__blk1056_dn4 = assign41000_e46843_d_n4;
        var_dqid_dxn_qi__blk1056_dn6 = assign41000_e46843_d_n6;
        var_dqid_dxn_qi__blk1056_dn7 = assign41000_e46843_d_n7;
        var_dqid_dxn_qi__blk1056_dn8 = assign41000_e46843_d_n8;
        var_dqid_dxn_qi__blk1056_dn9 = assign41000_e46843_d_n9;

        let (assign41010_e46860, assign41010_e46860_d_n4, assign41010_e46860_d_n6, assign41010_e46860_d_n7, assign41010_e46860_d_n8, assign41010_e46860_d_n9,) = {
    if ((((var_guard1080 != 0.0) && (var_guard1228 != 0.0)) && (var_guard1229 != 0.0)) && (var_guard1230 == 0.0)) {
        let assign41010_e46854: f64 = (var_dqid_dxn_qi__blk1056 * var_qid__blk1003);
        let assign41010_e46857: f64 = (var_dqid_dxn_qi__blk1056 + 1.0);
        let assign41010_e46858: f64 = (assign41010_e46854 / assign41010_e46857);
        (assign41010_e46858, (((((var_dqid_dxn_qi__blk1056_dn4 * var_qid__blk1003) + (var_dqid_dxn_qi__blk1056 * var_qid__blk1003_dn4)) * assign41010_e46857) - (assign41010_e46854 * var_dqid_dxn_qi__blk1056_dn4)) / (assign41010_e46857 * assign41010_e46857)), (((((var_dqid_dxn_qi__blk1056_dn6 * var_qid__blk1003) + (var_dqid_dxn_qi__blk1056 * var_qid__blk1003_dn6)) * assign41010_e46857) - (assign41010_e46854 * var_dqid_dxn_qi__blk1056_dn6)) / (assign41010_e46857 * assign41010_e46857)), (((((var_dqid_dxn_qi__blk1056_dn7 * var_qid__blk1003) + (var_dqid_dxn_qi__blk1056 * var_qid__blk1003_dn7)) * assign41010_e46857) - (assign41010_e46854 * var_dqid_dxn_qi__blk1056_dn7)) / (assign41010_e46857 * assign41010_e46857)), (((((var_dqid_dxn_qi__blk1056_dn8 * var_qid__blk1003) + (var_dqid_dxn_qi__blk1056 * var_qid__blk1003_dn8)) * assign41010_e46857) - (assign41010_e46854 * var_dqid_dxn_qi__blk1056_dn8)) / (assign41010_e46857 * assign41010_e46857)), (((((var_dqid_dxn_qi__blk1056_dn9 * var_qid__blk1003) + (var_dqid_dxn_qi__blk1056 * var_qid__blk1003_dn9)) * assign41010_e46857) - (assign41010_e46854 * var_dqid_dxn_qi__blk1056_dn9)) / (assign41010_e46857 * assign41010_e46857)),)
    } else {
        (var_dd__blk1057, var_dd__blk1057_dn4, var_dd__blk1057_dn6, var_dd__blk1057_dn7, var_dd__blk1057_dn8, var_dd__blk1057_dn9,)
    }
};
        var_dd__blk1057 = assign41010_e46860;
        var_dd__blk1057_dn4 = assign41010_e46860_d_n4;
        var_dd__blk1057_dn6 = assign41010_e46860_d_n6;
        var_dd__blk1057_dn7 = assign41010_e46860_d_n7;
        var_dd__blk1057_dn8 = assign41010_e46860_d_n8;
        var_dd__blk1057_dn9 = assign41010_e46860_d_n9;

        let (assign41020_e46869, assign41020_e46869_d_n4, assign41020_e46869_d_n6, assign41020_e46869_d_n7, assign41020_e46869_d_n8, assign41020_e46869_d_n9,) = {
    if (((var_guard1080 != 0.0) && (var_guard1228 != 0.0)) && (var_guard1229 == 0.0)) {
        (var_dinf__blk974, var_dinf__blk974_dn4, var_dinf__blk974_dn6, var_dinf__blk974_dn7, var_dinf__blk974_dn8, var_dinf__blk974_dn9,)
    } else {
        (var_dd__blk1057, var_dd__blk1057_dn4, var_dd__blk1057_dn6, var_dd__blk1057_dn7, var_dd__blk1057_dn8, var_dd__blk1057_dn9,)
    }
};
        var_dd__blk1057 = assign41020_e46869;
        var_dd__blk1057_dn4 = assign41020_e46869_d_n4;
        var_dd__blk1057_dn6 = assign41020_e46869_d_n6;
        var_dd__blk1057_dn7 = assign41020_e46869_d_n7;
        var_dd__blk1057_dn8 = assign41020_e46869_d_n8;
        var_dd__blk1057_dn9 = assign41020_e46869_d_n9;

        let (assign41030_e46877, assign41030_e46877_d_n4, assign41030_e46877_d_n6, assign41030_e46877_d_n7, assign41030_e46877_d_n8, assign41030_e46877_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1228 != 0.0)) {
        let assign41030_e46875: f64 = (var_dd__blk1057 - var_ds__blk981);
        (assign41030_e46875, (var_dd__blk1057_dn4 - var_ds__blk981_dn4), (var_dd__blk1057_dn6 - var_ds__blk981_dn6), (var_dd__blk1057_dn7 - var_ds__blk981_dn7), (var_dd__blk1057_dn8 - var_ds__blk981_dn8), (var_dd__blk1057_dn9 - var_ds__blk981_dn9),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign41030_e46877;
        var_temp1_dn4 = assign41030_e46877_d_n4;
        var_temp1_dn6 = assign41030_e46877_d_n6;
        var_temp1_dn7 = assign41030_e46877_d_n7;
        var_temp1_dn8 = assign41030_e46877_d_n8;
        var_temp1_dn9 = assign41030_e46877_d_n9;

        let (assign41040_e46889, assign41040_e46889_d_n4, assign41040_e46889_d_n6, assign41040_e46889_d_n7, assign41040_e46889_d_n8, assign41040_e46889_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1228 != 0.0)) {
        let assign41040_e46884: f64 = (36.0 * var_temp1);
        let assign41040_e46886: f64 = (assign41040_e46884 * var_temp1);
        let assign41040_e46887: f64 = (1.0 + assign41040_e46886);
        (assign41040_e46887, (((36.0 * var_temp1_dn4) * var_temp1) + (assign41040_e46884 * var_temp1_dn4)), (((36.0 * var_temp1_dn6) * var_temp1) + (assign41040_e46884 * var_temp1_dn6)), (((36.0 * var_temp1_dn7) * var_temp1) + (assign41040_e46884 * var_temp1_dn7)), (((36.0 * var_temp1_dn8) * var_temp1) + (assign41040_e46884 * var_temp1_dn8)), (((36.0 * var_temp1_dn9) * var_temp1) + (assign41040_e46884 * var_temp1_dn9)),)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign41040_e46889;
        var_temp2_dn4 = assign41040_e46889_d_n4;
        var_temp2_dn6 = assign41040_e46889_d_n6;
        var_temp2_dn7 = assign41040_e46889_d_n7;
        var_temp2_dn8 = assign41040_e46889_d_n8;
        var_temp2_dn9 = assign41040_e46889_d_n9;

        let assign41050_e46891: f64 = (var_temp1).abs();
        let assign41050_e46893: f64 = if assign41050_e46891 > 0.001 { 1.0 } else { 0.0 };
        var_guard1231 = assign41050_e46893;

        let (assign41060_e46903, assign41060_e46903_d_n4, assign41060_e46903_d_n6, assign41060_e46903_d_n7, assign41060_e46903_d_n8, assign41060_e46903_d_n9,) = {
    if (((var_guard1080 != 0.0) && (var_guard1228 != 0.0)) && (var_guard1231 != 0.0)) {
        let assign41060_e46901: f64 = (var_qid__blk1003 - var_qis__blk938);
        (assign41060_e46901, (var_qid__blk1003_dn4 - var_qis__blk938_dn4), (var_qid__blk1003_dn6 - var_qis__blk938_dn6), (var_qid__blk1003_dn7 - var_qis__blk938_dn7), (var_qid__blk1003_dn8 - var_qis__blk938_dn8), (var_qid__blk1003_dn9 - var_qis__blk938_dn9),)
    } else {
        (var_temp3, var_temp3_dn4, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9,)
    }
};
        var_temp3 = assign41060_e46903;
        var_temp3_dn4 = assign41060_e46903_d_n4;
        var_temp3_dn6 = assign41060_e46903_d_n6;
        var_temp3_dn7 = assign41060_e46903_d_n7;
        var_temp3_dn8 = assign41060_e46903_d_n8;
        var_temp3_dn9 = assign41060_e46903_d_n9;

        let (assign41070_e46915, assign41070_e46915_d_n4, assign41070_e46915_d_n6, assign41070_e46915_d_n7, assign41070_e46915_d_n8, assign41070_e46915_d_n9,) = {
    if (((var_guard1080 != 0.0) && (var_guard1228 != 0.0)) && (var_guard1231 != 0.0)) {
        let assign41070_e46912: f64 = (var_dd__blk1057 * var_dxdrift__blk1017);
        let assign41070_e46913: f64 = (var_temp3 - assign41070_e46912);
        (assign41070_e46913, (var_temp3_dn4 - ((var_dd__blk1057_dn4 * var_dxdrift__blk1017) + (var_dd__blk1057 * var_dxdrift__blk1017_dn4))), (var_temp3_dn6 - ((var_dd__blk1057_dn6 * var_dxdrift__blk1017) + (var_dd__blk1057 * var_dxdrift__blk1017_dn6))), (var_temp3_dn7 - ((var_dd__blk1057_dn7 * var_dxdrift__blk1017) + (var_dd__blk1057 * var_dxdrift__blk1017_dn7))), (var_temp3_dn8 - ((var_dd__blk1057_dn8 * var_dxdrift__blk1017) + (var_dd__blk1057 * var_dxdrift__blk1017_dn8))), (var_temp3_dn9 - ((var_dd__blk1057_dn9 * var_dxdrift__blk1017) + (var_dd__blk1057 * var_dxdrift__blk1017_dn9))),)
    } else {
        (var_ls__blk1058, var_ls__blk1058_dn4, var_ls__blk1058_dn6, var_ls__blk1058_dn7, var_ls__blk1058_dn8, var_ls__blk1058_dn9,)
    }
};
        var_ls__blk1058 = assign41070_e46915;
        var_ls__blk1058_dn4 = assign41070_e46915_d_n4;
        var_ls__blk1058_dn6 = assign41070_e46915_d_n6;
        var_ls__blk1058_dn7 = assign41070_e46915_d_n7;
        var_ls__blk1058_dn8 = assign41070_e46915_d_n8;
        var_ls__blk1058_dn9 = assign41070_e46915_d_n9;

        let (assign41080_e46927, assign41080_e46927_d_n4, assign41080_e46927_d_n6, assign41080_e46927_d_n7, assign41080_e46927_d_n8, assign41080_e46927_d_n9,) = {
    if (((var_guard1080 != 0.0) && (var_guard1228 != 0.0)) && (var_guard1231 != 0.0)) {
        let assign41080_e46924: f64 = (var_ds__blk981 * var_dxdrift__blk1017);
        let assign41080_e46925: f64 = (var_temp3 - assign41080_e46924);
        (assign41080_e46925, (var_temp3_dn4 - ((var_ds__blk981_dn4 * var_dxdrift__blk1017) + (var_ds__blk981 * var_dxdrift__blk1017_dn4))), (var_temp3_dn6 - ((var_ds__blk981_dn6 * var_dxdrift__blk1017) + (var_ds__blk981 * var_dxdrift__blk1017_dn6))), (var_temp3_dn7 - ((var_ds__blk981_dn7 * var_dxdrift__blk1017) + (var_ds__blk981 * var_dxdrift__blk1017_dn7))), (var_temp3_dn8 - ((var_ds__blk981_dn8 * var_dxdrift__blk1017) + (var_ds__blk981 * var_dxdrift__blk1017_dn8))), (var_temp3_dn9 - ((var_ds__blk981_dn9 * var_dxdrift__blk1017) + (var_ds__blk981 * var_dxdrift__blk1017_dn9))),)
    } else {
        (var_ld__blk1059, var_ld__blk1059_dn4, var_ld__blk1059_dn6, var_ld__blk1059_dn7, var_ld__blk1059_dn8, var_ld__blk1059_dn9,)
    }
};
        var_ld__blk1059 = assign41080_e46927;
        var_ld__blk1059_dn4 = assign41080_e46927_d_n4;
        var_ld__blk1059_dn6 = assign41080_e46927_d_n6;
        var_ld__blk1059_dn7 = assign41080_e46927_d_n7;
        var_ld__blk1059_dn8 = assign41080_e46927_d_n8;
        var_ld__blk1059_dn9 = assign41080_e46927_d_n9;

        let (assign41090_e46940, assign41090_e46940_d_n4, assign41090_e46940_d_n6, assign41090_e46940_d_n7, assign41090_e46940_d_n8, assign41090_e46940_d_n9,) = {
    if (((var_guard1080 != 0.0) && (var_guard1228 != 0.0)) && (var_guard1231 != 0.0)) {
        let assign41090_e46935: f64 = (var_ls__blk1058 * var_ls__blk1058);
        let assign41090_e46937: f64 = (assign41090_e46935 + var_temp2);
        let assign41090_e46938: f64 = (assign41090_e46937).sqrt();
        (assign41090_e46938, ((((var_ls__blk1058_dn4 * var_ls__blk1058) + (var_ls__blk1058 * var_ls__blk1058_dn4)) + var_temp2_dn4) / (2.0 * assign41090_e46938)), ((((var_ls__blk1058_dn6 * var_ls__blk1058) + (var_ls__blk1058 * var_ls__blk1058_dn6)) + var_temp2_dn6) / (2.0 * assign41090_e46938)), ((((var_ls__blk1058_dn7 * var_ls__blk1058) + (var_ls__blk1058 * var_ls__blk1058_dn7)) + var_temp2_dn7) / (2.0 * assign41090_e46938)), ((((var_ls__blk1058_dn8 * var_ls__blk1058) + (var_ls__blk1058 * var_ls__blk1058_dn8)) + var_temp2_dn8) / (2.0 * assign41090_e46938)), ((((var_ls__blk1058_dn9 * var_ls__blk1058) + (var_ls__blk1058 * var_ls__blk1058_dn9)) + var_temp2_dn9) / (2.0 * assign41090_e46938)),)
    } else {
        (var_us__blk1060, var_us__blk1060_dn4, var_us__blk1060_dn6, var_us__blk1060_dn7, var_us__blk1060_dn8, var_us__blk1060_dn9,)
    }
};
        var_us__blk1060 = assign41090_e46940;
        var_us__blk1060_dn4 = assign41090_e46940_d_n4;
        var_us__blk1060_dn6 = assign41090_e46940_d_n6;
        var_us__blk1060_dn7 = assign41090_e46940_d_n7;
        var_us__blk1060_dn8 = assign41090_e46940_d_n8;
        var_us__blk1060_dn9 = assign41090_e46940_d_n9;

        let (assign41100_e46953, assign41100_e46953_d_n4, assign41100_e46953_d_n6, assign41100_e46953_d_n7, assign41100_e46953_d_n8, assign41100_e46953_d_n9,) = {
    if (((var_guard1080 != 0.0) && (var_guard1228 != 0.0)) && (var_guard1231 != 0.0)) {
        let assign41100_e46948: f64 = (var_ld__blk1059 * var_ld__blk1059);
        let assign41100_e46950: f64 = (assign41100_e46948 + var_temp2);
        let assign41100_e46951: f64 = (assign41100_e46950).sqrt();
        (assign41100_e46951, ((((var_ld__blk1059_dn4 * var_ld__blk1059) + (var_ld__blk1059 * var_ld__blk1059_dn4)) + var_temp2_dn4) / (2.0 * assign41100_e46951)), ((((var_ld__blk1059_dn6 * var_ld__blk1059) + (var_ld__blk1059 * var_ld__blk1059_dn6)) + var_temp2_dn6) / (2.0 * assign41100_e46951)), ((((var_ld__blk1059_dn7 * var_ld__blk1059) + (var_ld__blk1059 * var_ld__blk1059_dn7)) + var_temp2_dn7) / (2.0 * assign41100_e46951)), ((((var_ld__blk1059_dn8 * var_ld__blk1059) + (var_ld__blk1059 * var_ld__blk1059_dn8)) + var_temp2_dn8) / (2.0 * assign41100_e46951)), ((((var_ld__blk1059_dn9 * var_ld__blk1059) + (var_ld__blk1059 * var_ld__blk1059_dn9)) + var_temp2_dn9) / (2.0 * assign41100_e46951)),)
    } else {
        (var_ud__blk1061, var_ud__blk1061_dn4, var_ud__blk1061_dn6, var_ud__blk1061_dn7, var_ud__blk1061_dn8, var_ud__blk1061_dn9,)
    }
};
        var_ud__blk1061 = assign41100_e46953;
        var_ud__blk1061_dn4 = assign41100_e46953_d_n4;
        var_ud__blk1061_dn6 = assign41100_e46953_d_n6;
        var_ud__blk1061_dn7 = assign41100_e46953_d_n7;
        var_ud__blk1061_dn8 = assign41100_e46953_d_n8;
        var_ud__blk1061_dn9 = assign41100_e46953_d_n9;

        let (assign41110_e46982, assign41110_e46982_d_n4, assign41110_e46982_d_n6, assign41110_e46982_d_n7, assign41110_e46982_d_n8, assign41110_e46982_d_n9,) = {
    if (((var_guard1080 != 0.0) && (var_guard1228 != 0.0)) && (var_guard1231 != 0.0)) {
        let assign41110_e46961: f64 = (0.25 / var_temp1);
        let assign41110_e46964: f64 = (var_ud__blk1061 * var_ls__blk1058);
        let assign41110_e46967: f64 = (var_us__blk1060 * var_ld__blk1059);
        let assign41110_e46968: f64 = (assign41110_e46964 - assign41110_e46967);
        let assign41110_e46972: f64 = (var_ld__blk1059 + var_ud__blk1061);
        let assign41110_e46975: f64 = (var_ls__blk1058 + var_us__blk1060);
        let assign41110_e46976: f64 = (assign41110_e46972 / assign41110_e46975);
        let assign41110_e46977: f64 = (assign41110_e46976).ln();
        let assign41110_e46978: f64 = (var_temp2 * assign41110_e46977);
        let assign41110_e46979: f64 = (assign41110_e46968 + assign41110_e46978);
        let assign41110_e46980: f64 = (assign41110_e46961 * assign41110_e46979);
        (assign41110_e46980, (((-((0.25 * var_temp1_dn4) / (var_temp1 * var_temp1))) * assign41110_e46979) + (assign41110_e46961 * ((((var_ud__blk1061_dn4 * var_ls__blk1058) + (var_ud__blk1061 * var_ls__blk1058_dn4)) - ((var_us__blk1060_dn4 * var_ld__blk1059) + (var_us__blk1060 * var_ld__blk1059_dn4))) + ((var_temp2_dn4 * assign41110_e46977) + (var_temp2 * (((((var_ld__blk1059_dn4 + var_ud__blk1061_dn4) * assign41110_e46975) - (assign41110_e46972 * (var_ls__blk1058_dn4 + var_us__blk1060_dn4))) / (assign41110_e46975 * assign41110_e46975)) / assign41110_e46976)))))), (((-((0.25 * var_temp1_dn6) / (var_temp1 * var_temp1))) * assign41110_e46979) + (assign41110_e46961 * ((((var_ud__blk1061_dn6 * var_ls__blk1058) + (var_ud__blk1061 * var_ls__blk1058_dn6)) - ((var_us__blk1060_dn6 * var_ld__blk1059) + (var_us__blk1060 * var_ld__blk1059_dn6))) + ((var_temp2_dn6 * assign41110_e46977) + (var_temp2 * (((((var_ld__blk1059_dn6 + var_ud__blk1061_dn6) * assign41110_e46975) - (assign41110_e46972 * (var_ls__blk1058_dn6 + var_us__blk1060_dn6))) / (assign41110_e46975 * assign41110_e46975)) / assign41110_e46976)))))), (((-((0.25 * var_temp1_dn7) / (var_temp1 * var_temp1))) * assign41110_e46979) + (assign41110_e46961 * ((((var_ud__blk1061_dn7 * var_ls__blk1058) + (var_ud__blk1061 * var_ls__blk1058_dn7)) - ((var_us__blk1060_dn7 * var_ld__blk1059) + (var_us__blk1060 * var_ld__blk1059_dn7))) + ((var_temp2_dn7 * assign41110_e46977) + (var_temp2 * (((((var_ld__blk1059_dn7 + var_ud__blk1061_dn7) * assign41110_e46975) - (assign41110_e46972 * (var_ls__blk1058_dn7 + var_us__blk1060_dn7))) / (assign41110_e46975 * assign41110_e46975)) / assign41110_e46976)))))), (((-((0.25 * var_temp1_dn8) / (var_temp1 * var_temp1))) * assign41110_e46979) + (assign41110_e46961 * ((((var_ud__blk1061_dn8 * var_ls__blk1058) + (var_ud__blk1061 * var_ls__blk1058_dn8)) - ((var_us__blk1060_dn8 * var_ld__blk1059) + (var_us__blk1060 * var_ld__blk1059_dn8))) + ((var_temp2_dn8 * assign41110_e46977) + (var_temp2 * (((((var_ld__blk1059_dn8 + var_ud__blk1061_dn8) * assign41110_e46975) - (assign41110_e46972 * (var_ls__blk1058_dn8 + var_us__blk1060_dn8))) / (assign41110_e46975 * assign41110_e46975)) / assign41110_e46976)))))), (((-((0.25 * var_temp1_dn9) / (var_temp1 * var_temp1))) * assign41110_e46979) + (assign41110_e46961 * ((((var_ud__blk1061_dn9 * var_ls__blk1058) + (var_ud__blk1061 * var_ls__blk1058_dn9)) - ((var_us__blk1060_dn9 * var_ld__blk1059) + (var_us__blk1060 * var_ld__blk1059_dn9))) + ((var_temp2_dn9 * assign41110_e46977) + (var_temp2 * (((((var_ld__blk1059_dn9 + var_ud__blk1061_dn9) * assign41110_e46975) - (assign41110_e46972 * (var_ls__blk1058_dn9 + var_us__blk1060_dn9))) / (assign41110_e46975 * assign41110_e46975)) / assign41110_e46976)))))),)
    } else {
        (var_idrift2__blk1062, var_idrift2__blk1062_dn4, var_idrift2__blk1062_dn6, var_idrift2__blk1062_dn7, var_idrift2__blk1062_dn8, var_idrift2__blk1062_dn9,)
    }
};
        var_idrift2__blk1062 = assign41110_e46982;
        var_idrift2__blk1062_dn4 = assign41110_e46982_d_n4;
        var_idrift2__blk1062_dn6 = assign41110_e46982_d_n6;
        var_idrift2__blk1062_dn7 = assign41110_e46982_d_n7;
        var_idrift2__blk1062_dn8 = assign41110_e46982_d_n8;
        var_idrift2__blk1062_dn9 = assign41110_e46982_d_n9;

        let (assign41120_e46993, assign41120_e46993_d_n4, assign41120_e46993_d_n6, assign41120_e46993_d_n7, assign41120_e46993_d_n8, assign41120_e46993_d_n9,) = {
    if (((var_guard1080 != 0.0) && (var_guard1228 != 0.0)) && (var_guard1231 == 0.0)) {
        let assign41120_e46991: f64 = (var_dxdrift__blk1017 * var_temp1);
        (assign41120_e46991, ((var_dxdrift__blk1017_dn4 * var_temp1) + (var_dxdrift__blk1017 * var_temp1_dn4)), ((var_dxdrift__blk1017_dn6 * var_temp1) + (var_dxdrift__blk1017 * var_temp1_dn6)), ((var_dxdrift__blk1017_dn7 * var_temp1) + (var_dxdrift__blk1017 * var_temp1_dn7)), ((var_dxdrift__blk1017_dn8 * var_temp1) + (var_dxdrift__blk1017 * var_temp1_dn8)), ((var_dxdrift__blk1017_dn9 * var_temp1) + (var_dxdrift__blk1017 * var_temp1_dn9)),)
    } else {
        (var_temp3, var_temp3_dn4, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9,)
    }
};
        var_temp3 = assign41120_e46993;
        var_temp3_dn4 = assign41120_e46993_d_n4;
        var_temp3_dn6 = assign41120_e46993_d_n6;
        var_temp3_dn7 = assign41120_e46993_d_n7;
        var_temp3_dn8 = assign41120_e46993_d_n8;
        var_temp3_dn9 = assign41120_e46993_d_n9;

        let (assign41130_e47014, assign41130_e47014_d_n4, assign41130_e47014_d_n6, assign41130_e47014_d_n7, assign41130_e47014_d_n8, assign41130_e47014_d_n9,) = {
    if (((var_guard1080 != 0.0) && (var_guard1228 != 0.0)) && (var_guard1231 == 0.0)) {
        let assign41130_e47001: f64 = (-0.25);
        let assign41130_e47003: f64 = (assign41130_e47001 * 0.1666666666667);
        let assign41130_e47005: f64 = (assign41130_e47003 * var_dxdrift__blk1017);
        let assign41130_e47007: f64 = (assign41130_e47005 * var_temp3);
        let assign41130_e47009: f64 = (assign41130_e47007 * var_temp3);
        let assign41130_e47011: f64 = (var_temp2).sqrt();
        let assign41130_e47012: f64 = (assign41130_e47009 / assign41130_e47011);
        (assign41130_e47012, ((((((((assign41130_e47003 * var_dxdrift__blk1017_dn4) * var_temp3) + (assign41130_e47005 * var_temp3_dn4)) * var_temp3) + (assign41130_e47007 * var_temp3_dn4)) * assign41130_e47011) - (assign41130_e47009 * (var_temp2_dn4 / (2.0 * assign41130_e47011)))) / (assign41130_e47011 * assign41130_e47011)), ((((((((assign41130_e47003 * var_dxdrift__blk1017_dn6) * var_temp3) + (assign41130_e47005 * var_temp3_dn6)) * var_temp3) + (assign41130_e47007 * var_temp3_dn6)) * assign41130_e47011) - (assign41130_e47009 * (var_temp2_dn6 / (2.0 * assign41130_e47011)))) / (assign41130_e47011 * assign41130_e47011)), ((((((((assign41130_e47003 * var_dxdrift__blk1017_dn7) * var_temp3) + (assign41130_e47005 * var_temp3_dn7)) * var_temp3) + (assign41130_e47007 * var_temp3_dn7)) * assign41130_e47011) - (assign41130_e47009 * (var_temp2_dn7 / (2.0 * assign41130_e47011)))) / (assign41130_e47011 * assign41130_e47011)), ((((((((assign41130_e47003 * var_dxdrift__blk1017_dn8) * var_temp3) + (assign41130_e47005 * var_temp3_dn8)) * var_temp3) + (assign41130_e47007 * var_temp3_dn8)) * assign41130_e47011) - (assign41130_e47009 * (var_temp2_dn8 / (2.0 * assign41130_e47011)))) / (assign41130_e47011 * assign41130_e47011)), ((((((((assign41130_e47003 * var_dxdrift__blk1017_dn9) * var_temp3) + (assign41130_e47005 * var_temp3_dn9)) * var_temp3) + (assign41130_e47007 * var_temp3_dn9)) * assign41130_e47011) - (assign41130_e47009 * (var_temp2_dn9 / (2.0 * assign41130_e47011)))) / (assign41130_e47011 * assign41130_e47011)),)
    } else {
        (var_idrift2__blk1062, var_idrift2__blk1062_dn4, var_idrift2__blk1062_dn6, var_idrift2__blk1062_dn7, var_idrift2__blk1062_dn8, var_idrift2__blk1062_dn9,)
    }
};
        var_idrift2__blk1062 = assign41130_e47014;
        var_idrift2__blk1062_dn4 = assign41130_e47014_d_n4;
        var_idrift2__blk1062_dn6 = assign41130_e47014_d_n6;
        var_idrift2__blk1062_dn7 = assign41130_e47014_d_n7;
        var_idrift2__blk1062_dn8 = assign41130_e47014_d_n8;
        var_idrift2__blk1062_dn9 = assign41130_e47014_d_n9;

        let (assign41140_e47021, assign41140_e47021_d_n4, assign41140_e47021_d_n6, assign41140_e47021_d_n7, assign41140_e47021_d_n8, assign41140_e47021_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1228 == 0.0)) {
        (var_dinf__blk974, var_dinf__blk974_dn4, var_dinf__blk974_dn6, var_dinf__blk974_dn7, var_dinf__blk974_dn8, var_dinf__blk974_dn9,)
    } else {
        (var_dd__blk1057, var_dd__blk1057_dn4, var_dd__blk1057_dn6, var_dd__blk1057_dn7, var_dd__blk1057_dn8, var_dd__blk1057_dn9,)
    }
};
        var_dd__blk1057 = assign41140_e47021;
        var_dd__blk1057_dn4 = assign41140_e47021_d_n4;
        var_dd__blk1057_dn6 = assign41140_e47021_d_n6;
        var_dd__blk1057_dn7 = assign41140_e47021_d_n7;
        var_dd__blk1057_dn8 = assign41140_e47021_d_n8;
        var_dd__blk1057_dn9 = assign41140_e47021_d_n9;

        let (assign41150_e47028, assign41150_e47028_d_n4, assign41150_e47028_d_n6, assign41150_e47028_d_n7, assign41150_e47028_d_n8, assign41150_e47028_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1228 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_idrift2__blk1062, var_idrift2__blk1062_dn4, var_idrift2__blk1062_dn6, var_idrift2__blk1062_dn7, var_idrift2__blk1062_dn8, var_idrift2__blk1062_dn9,)
    }
};
        var_idrift2__blk1062 = assign41150_e47028;
        var_idrift2__blk1062_dn4 = assign41150_e47028_d_n4;
        var_idrift2__blk1062_dn6 = assign41150_e47028_d_n6;
        var_idrift2__blk1062_dn7 = assign41150_e47028_d_n7;
        var_idrift2__blk1062_dn8 = assign41150_e47028_d_n8;
        var_idrift2__blk1062_dn9 = assign41150_e47028_d_n9;

        let (assign41160_e47040, assign41160_e47040_d_n4, assign41160_e47040_d_n6, assign41160_e47040_d_n7, assign41160_e47040_d_n8, assign41160_e47040_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign41160_e47032: f64 = (var_qim__blk1016 * var_dxdrift__blk1017);
        let assign41160_e47034: f64 = (assign41160_e47032 + var_idrift2__blk1062);
        let assign41160_e47036: f64 = (assign41160_e47034 + var_qis__blk938);
        let assign41160_e47038: f64 = (assign41160_e47036 - var_qid__blk1003);
        (assign41160_e47038, (((((var_qim__blk1016_dn4 * var_dxdrift__blk1017) + (var_qim__blk1016 * var_dxdrift__blk1017_dn4)) + var_idrift2__blk1062_dn4) + var_qis__blk938_dn4) - var_qid__blk1003_dn4), (((((var_qim__blk1016_dn6 * var_dxdrift__blk1017) + (var_qim__blk1016 * var_dxdrift__blk1017_dn6)) + var_idrift2__blk1062_dn6) + var_qis__blk938_dn6) - var_qid__blk1003_dn6), (((((var_qim__blk1016_dn7 * var_dxdrift__blk1017) + (var_qim__blk1016 * var_dxdrift__blk1017_dn7)) + var_idrift2__blk1062_dn7) + var_qis__blk938_dn7) - var_qid__blk1003_dn7), (((((var_qim__blk1016_dn8 * var_dxdrift__blk1017) + (var_qim__blk1016 * var_dxdrift__blk1017_dn8)) + var_idrift2__blk1062_dn8) + var_qis__blk938_dn8) - var_qid__blk1003_dn8), (((((var_qim__blk1016_dn9 * var_dxdrift__blk1017) + (var_qim__blk1016 * var_dxdrift__blk1017_dn9)) + var_idrift2__blk1062_dn9) + var_qis__blk938_dn9) - var_qid__blk1003_dn9),)
    } else {
        (var_norm_ids__blk1063, var_norm_ids__blk1063_dn4, var_norm_ids__blk1063_dn6, var_norm_ids__blk1063_dn7, var_norm_ids__blk1063_dn8, var_norm_ids__blk1063_dn9,)
    }
};
        var_norm_ids__blk1063 = assign41160_e47040;
        var_norm_ids__blk1063_dn4 = assign41160_e47040_d_n4;
        var_norm_ids__blk1063_dn6 = assign41160_e47040_d_n6;
        var_norm_ids__blk1063_dn7 = assign41160_e47040_d_n7;
        var_norm_ids__blk1063_dn8 = assign41160_e47040_d_n8;
        var_norm_ids__blk1063_dn9 = assign41160_e47040_d_n9;

        let assign41170_e47043: f64 = if var_qis__blk938 > 1e-6 { 1.0 } else { 0.0 };
        var_guard1232 = assign41170_e47043;

        let assign41180_e47046: f64 = if var_norm_ids__blk1063 > 1e-30 { 1.0 } else { 0.0 };
        var_guard1233 = assign41180_e47046;

        let (assign41190_e47060, assign41190_e47060_d_n4, assign41190_e47060_d_n6, assign41190_e47060_d_n7, assign41190_e47060_d_n8, assign41190_e47060_d_n9,) = {
    if (((var_guard1080 != 0.0) && (var_guard1232 != 0.0)) && (var_guard1233 != 0.0)) {
        let assign41190_e47055: f64 = (var_aexp1s__blk943 / var_qis__blk938);
        let assign41190_e47057: f64 = (assign41190_e47055 - var_dqsqs_dxn_qi__blk950);
        let assign41190_e47058: f64 = (var_a1s__blk947 / assign41190_e47057);
        (assign41190_e47058, (((var_a1s__blk947_dn4 * assign41190_e47057) - (var_a1s__blk947 * ((((var_aexp1s__blk943_dn4 * var_qis__blk938) - (var_aexp1s__blk943 * var_qis__blk938_dn4)) / (var_qis__blk938 * var_qis__blk938)) - var_dqsqs_dxn_qi__blk950_dn4))) / (assign41190_e47057 * assign41190_e47057)), (((var_a1s__blk947_dn6 * assign41190_e47057) - (var_a1s__blk947 * ((((var_aexp1s__blk943_dn6 * var_qis__blk938) - (var_aexp1s__blk943 * var_qis__blk938_dn6)) / (var_qis__blk938 * var_qis__blk938)) - var_dqsqs_dxn_qi__blk950_dn6))) / (assign41190_e47057 * assign41190_e47057)), (((var_a1s__blk947_dn7 * assign41190_e47057) - (var_a1s__blk947 * ((((var_aexp1s__blk943_dn7 * var_qis__blk938) - (var_aexp1s__blk943 * var_qis__blk938_dn7)) / (var_qis__blk938 * var_qis__blk938)) - var_dqsqs_dxn_qi__blk950_dn7))) / (assign41190_e47057 * assign41190_e47057)), (((var_a1s__blk947_dn8 * assign41190_e47057) - (var_a1s__blk947 * ((((var_aexp1s__blk943_dn8 * var_qis__blk938) - (var_aexp1s__blk943 * var_qis__blk938_dn8)) / (var_qis__blk938 * var_qis__blk938)) - var_dqsqs_dxn_qi__blk950_dn8))) / (assign41190_e47057 * assign41190_e47057)), (((var_a1s__blk947_dn9 * assign41190_e47057) - (var_a1s__blk947 * ((((var_aexp1s__blk943_dn9 * var_qis__blk938) - (var_aexp1s__blk943 * var_qis__blk938_dn9)) / (var_qis__blk938 * var_qis__blk938)) - var_dqsqs_dxn_qi__blk950_dn9))) / (assign41190_e47057 * assign41190_e47057)),)
    } else {
        (var_q1s_chap__blk1064, var_q1s_chap__blk1064_dn4, var_q1s_chap__blk1064_dn6, var_q1s_chap__blk1064_dn7, var_q1s_chap__blk1064_dn8, var_q1s_chap__blk1064_dn9,)
    }
};
        var_q1s_chap__blk1064 = assign41190_e47060;
        var_q1s_chap__blk1064_dn4 = assign41190_e47060_d_n4;
        var_q1s_chap__blk1064_dn6 = assign41190_e47060_d_n6;
        var_q1s_chap__blk1064_dn7 = assign41190_e47060_d_n7;
        var_q1s_chap__blk1064_dn8 = assign41190_e47060_d_n8;
        var_q1s_chap__blk1064_dn9 = assign41190_e47060_d_n9;

        let (assign41200_e47074, assign41200_e47074_d_n4, assign41200_e47074_d_n6, assign41200_e47074_d_n7, assign41200_e47074_d_n8, assign41200_e47074_d_n9,) = {
    if (((var_guard1080 != 0.0) && (var_guard1232 != 0.0)) && (var_guard1233 != 0.0)) {
        let assign41200_e47069: f64 = (var_aexp1d__blk1007 / var_qid__blk1003);
        let assign41200_e47071: f64 = (assign41200_e47069 - var_dqsqd_dxn_qi__blk1014);
        let assign41200_e47072: f64 = (var_a1d__blk1011 / assign41200_e47071);
        (assign41200_e47072, (((var_a1d__blk1011_dn4 * assign41200_e47071) - (var_a1d__blk1011 * ((((var_aexp1d__blk1007_dn4 * var_qid__blk1003) - (var_aexp1d__blk1007 * var_qid__blk1003_dn4)) / (var_qid__blk1003 * var_qid__blk1003)) - var_dqsqd_dxn_qi__blk1014_dn4))) / (assign41200_e47071 * assign41200_e47071)), (((var_a1d__blk1011_dn6 * assign41200_e47071) - (var_a1d__blk1011 * ((((var_aexp1d__blk1007_dn6 * var_qid__blk1003) - (var_aexp1d__blk1007 * var_qid__blk1003_dn6)) / (var_qid__blk1003 * var_qid__blk1003)) - var_dqsqd_dxn_qi__blk1014_dn6))) / (assign41200_e47071 * assign41200_e47071)), (((var_a1d__blk1011_dn7 * assign41200_e47071) - (var_a1d__blk1011 * ((((var_aexp1d__blk1007_dn7 * var_qid__blk1003) - (var_aexp1d__blk1007 * var_qid__blk1003_dn7)) / (var_qid__blk1003 * var_qid__blk1003)) - var_dqsqd_dxn_qi__blk1014_dn7))) / (assign41200_e47071 * assign41200_e47071)), (((var_a1d__blk1011_dn8 * assign41200_e47071) - (var_a1d__blk1011 * ((((var_aexp1d__blk1007_dn8 * var_qid__blk1003) - (var_aexp1d__blk1007 * var_qid__blk1003_dn8)) / (var_qid__blk1003 * var_qid__blk1003)) - var_dqsqd_dxn_qi__blk1014_dn8))) / (assign41200_e47071 * assign41200_e47071)), (((var_a1d__blk1011_dn9 * assign41200_e47071) - (var_a1d__blk1011 * ((((var_aexp1d__blk1007_dn9 * var_qid__blk1003) - (var_aexp1d__blk1007 * var_qid__blk1003_dn9)) / (var_qid__blk1003 * var_qid__blk1003)) - var_dqsqd_dxn_qi__blk1014_dn9))) / (assign41200_e47071 * assign41200_e47071)),)
    } else {
        (var_q1d_chap__blk1065, var_q1d_chap__blk1065_dn4, var_q1d_chap__blk1065_dn6, var_q1d_chap__blk1065_dn7, var_q1d_chap__blk1065_dn8, var_q1d_chap__blk1065_dn9,)
    }
};
        var_q1d_chap__blk1065 = assign41200_e47074;
        var_q1d_chap__blk1065_dn4 = assign41200_e47074_d_n4;
        var_q1d_chap__blk1065_dn6 = assign41200_e47074_d_n6;
        var_q1d_chap__blk1065_dn7 = assign41200_e47074_d_n7;
        var_q1d_chap__blk1065_dn8 = assign41200_e47074_d_n8;
        var_q1d_chap__blk1065_dn9 = assign41200_e47074_d_n9;

        let (assign41210_e47086, assign41210_e47086_d_n4, assign41210_e47086_d_n6, assign41210_e47086_d_n7, assign41210_e47086_d_n8, assign41210_e47086_d_n9,) = {
    if (((var_guard1080 != 0.0) && (var_guard1232 != 0.0)) && (var_guard1233 != 0.0)) {
        let assign41210_e47082: f64 = (var_q1s_chap__blk1064 - var_q1d_chap__blk1065);
        let assign41210_e47084: f64 = (assign41210_e47082 / var_norm_ids__blk1063);
        (assign41210_e47084, ((((var_q1s_chap__blk1064_dn4 - var_q1d_chap__blk1065_dn4) * var_norm_ids__blk1063) - (assign41210_e47082 * var_norm_ids__blk1063_dn4)) / (var_norm_ids__blk1063 * var_norm_ids__blk1063)), ((((var_q1s_chap__blk1064_dn6 - var_q1d_chap__blk1065_dn6) * var_norm_ids__blk1063) - (assign41210_e47082 * var_norm_ids__blk1063_dn6)) / (var_norm_ids__blk1063 * var_norm_ids__blk1063)), ((((var_q1s_chap__blk1064_dn7 - var_q1d_chap__blk1065_dn7) * var_norm_ids__blk1063) - (assign41210_e47082 * var_norm_ids__blk1063_dn7)) / (var_norm_ids__blk1063 * var_norm_ids__blk1063)), ((((var_q1s_chap__blk1064_dn8 - var_q1d_chap__blk1065_dn8) * var_norm_ids__blk1063) - (assign41210_e47082 * var_norm_ids__blk1063_dn8)) / (var_norm_ids__blk1063 * var_norm_ids__blk1063)), ((((var_q1s_chap__blk1064_dn9 - var_q1d_chap__blk1065_dn9) * var_norm_ids__blk1063) - (assign41210_e47082 * var_norm_ids__blk1063_dn9)) / (var_norm_ids__blk1063 * var_norm_ids__blk1063)),)
    } else {
        (var_inv_k1h1_0__blk1066, var_inv_k1h1_0__blk1066_dn4, var_inv_k1h1_0__blk1066_dn6, var_inv_k1h1_0__blk1066_dn7, var_inv_k1h1_0__blk1066_dn8, var_inv_k1h1_0__blk1066_dn9,)
    }
};
        var_inv_k1h1_0__blk1066 = assign41210_e47086;
        var_inv_k1h1_0__blk1066_dn4 = assign41210_e47086_d_n4;
        var_inv_k1h1_0__blk1066_dn6 = assign41210_e47086_d_n6;
        var_inv_k1h1_0__blk1066_dn7 = assign41210_e47086_d_n7;
        var_inv_k1h1_0__blk1066_dn8 = assign41210_e47086_d_n8;
        var_inv_k1h1_0__blk1066_dn9 = assign41210_e47086_d_n9;

        let (assign41220_e47100, assign41220_e47100_d_n4, assign41220_e47100_d_n6, assign41220_e47100_d_n7, assign41220_e47100_d_n8, assign41220_e47100_d_n9,) = {
    if (((var_guard1080 != 0.0) && (var_guard1232 != 0.0)) && (var_guard1233 != 0.0)) {
        let assign41220_e47095: f64 = (var_aexp2s__blk944 / var_qis__blk938);
        let assign41220_e47097: f64 = (assign41220_e47095 - var_dqsqs_dxn_qi__blk950);
        let assign41220_e47098: f64 = (var_a2s__blk948 / assign41220_e47097);
        (assign41220_e47098, (((var_a2s__blk948_dn4 * assign41220_e47097) - (var_a2s__blk948 * ((((var_aexp2s__blk944_dn4 * var_qis__blk938) - (var_aexp2s__blk944 * var_qis__blk938_dn4)) / (var_qis__blk938 * var_qis__blk938)) - var_dqsqs_dxn_qi__blk950_dn4))) / (assign41220_e47097 * assign41220_e47097)), (((var_a2s__blk948_dn6 * assign41220_e47097) - (var_a2s__blk948 * ((((var_aexp2s__blk944_dn6 * var_qis__blk938) - (var_aexp2s__blk944 * var_qis__blk938_dn6)) / (var_qis__blk938 * var_qis__blk938)) - var_dqsqs_dxn_qi__blk950_dn6))) / (assign41220_e47097 * assign41220_e47097)), (((var_a2s__blk948_dn7 * assign41220_e47097) - (var_a2s__blk948 * ((((var_aexp2s__blk944_dn7 * var_qis__blk938) - (var_aexp2s__blk944 * var_qis__blk938_dn7)) / (var_qis__blk938 * var_qis__blk938)) - var_dqsqs_dxn_qi__blk950_dn7))) / (assign41220_e47097 * assign41220_e47097)), (((var_a2s__blk948_dn8 * assign41220_e47097) - (var_a2s__blk948 * ((((var_aexp2s__blk944_dn8 * var_qis__blk938) - (var_aexp2s__blk944 * var_qis__blk938_dn8)) / (var_qis__blk938 * var_qis__blk938)) - var_dqsqs_dxn_qi__blk950_dn8))) / (assign41220_e47097 * assign41220_e47097)), (((var_a2s__blk948_dn9 * assign41220_e47097) - (var_a2s__blk948 * ((((var_aexp2s__blk944_dn9 * var_qis__blk938) - (var_aexp2s__blk944 * var_qis__blk938_dn9)) / (var_qis__blk938 * var_qis__blk938)) - var_dqsqs_dxn_qi__blk950_dn9))) / (assign41220_e47097 * assign41220_e47097)),)
    } else {
        (var_q2s_chap__blk1067, var_q2s_chap__blk1067_dn4, var_q2s_chap__blk1067_dn6, var_q2s_chap__blk1067_dn7, var_q2s_chap__blk1067_dn8, var_q2s_chap__blk1067_dn9,)
    }
};
        var_q2s_chap__blk1067 = assign41220_e47100;
        var_q2s_chap__blk1067_dn4 = assign41220_e47100_d_n4;
        var_q2s_chap__blk1067_dn6 = assign41220_e47100_d_n6;
        var_q2s_chap__blk1067_dn7 = assign41220_e47100_d_n7;
        var_q2s_chap__blk1067_dn8 = assign41220_e47100_d_n8;
        var_q2s_chap__blk1067_dn9 = assign41220_e47100_d_n9;

        let (assign41230_e47114, assign41230_e47114_d_n4, assign41230_e47114_d_n6, assign41230_e47114_d_n7, assign41230_e47114_d_n8, assign41230_e47114_d_n9,) = {
    if (((var_guard1080 != 0.0) && (var_guard1232 != 0.0)) && (var_guard1233 != 0.0)) {
        let assign41230_e47109: f64 = (var_aexp2d__blk1008 / var_qid__blk1003);
        let assign41230_e47111: f64 = (assign41230_e47109 - var_dqsqd_dxn_qi__blk1014);
        let assign41230_e47112: f64 = (var_a2d__blk1012 / assign41230_e47111);
        (assign41230_e47112, (((var_a2d__blk1012_dn4 * assign41230_e47111) - (var_a2d__blk1012 * ((((var_aexp2d__blk1008_dn4 * var_qid__blk1003) - (var_aexp2d__blk1008 * var_qid__blk1003_dn4)) / (var_qid__blk1003 * var_qid__blk1003)) - var_dqsqd_dxn_qi__blk1014_dn4))) / (assign41230_e47111 * assign41230_e47111)), (((var_a2d__blk1012_dn6 * assign41230_e47111) - (var_a2d__blk1012 * ((((var_aexp2d__blk1008_dn6 * var_qid__blk1003) - (var_aexp2d__blk1008 * var_qid__blk1003_dn6)) / (var_qid__blk1003 * var_qid__blk1003)) - var_dqsqd_dxn_qi__blk1014_dn6))) / (assign41230_e47111 * assign41230_e47111)), (((var_a2d__blk1012_dn7 * assign41230_e47111) - (var_a2d__blk1012 * ((((var_aexp2d__blk1008_dn7 * var_qid__blk1003) - (var_aexp2d__blk1008 * var_qid__blk1003_dn7)) / (var_qid__blk1003 * var_qid__blk1003)) - var_dqsqd_dxn_qi__blk1014_dn7))) / (assign41230_e47111 * assign41230_e47111)), (((var_a2d__blk1012_dn8 * assign41230_e47111) - (var_a2d__blk1012 * ((((var_aexp2d__blk1008_dn8 * var_qid__blk1003) - (var_aexp2d__blk1008 * var_qid__blk1003_dn8)) / (var_qid__blk1003 * var_qid__blk1003)) - var_dqsqd_dxn_qi__blk1014_dn8))) / (assign41230_e47111 * assign41230_e47111)), (((var_a2d__blk1012_dn9 * assign41230_e47111) - (var_a2d__blk1012 * ((((var_aexp2d__blk1008_dn9 * var_qid__blk1003) - (var_aexp2d__blk1008 * var_qid__blk1003_dn9)) / (var_qid__blk1003 * var_qid__blk1003)) - var_dqsqd_dxn_qi__blk1014_dn9))) / (assign41230_e47111 * assign41230_e47111)),)
    } else {
        (var_q2d_chap__blk1068, var_q2d_chap__blk1068_dn4, var_q2d_chap__blk1068_dn6, var_q2d_chap__blk1068_dn7, var_q2d_chap__blk1068_dn8, var_q2d_chap__blk1068_dn9,)
    }
};
        var_q2d_chap__blk1068 = assign41230_e47114;
        var_q2d_chap__blk1068_dn4 = assign41230_e47114_d_n4;
        var_q2d_chap__blk1068_dn6 = assign41230_e47114_d_n6;
        var_q2d_chap__blk1068_dn7 = assign41230_e47114_d_n7;
        var_q2d_chap__blk1068_dn8 = assign41230_e47114_d_n8;
        var_q2d_chap__blk1068_dn9 = assign41230_e47114_d_n9;

        let (assign41240_e47126, assign41240_e47126_d_n4, assign41240_e47126_d_n6, assign41240_e47126_d_n7, assign41240_e47126_d_n8, assign41240_e47126_d_n9,) = {
    if (((var_guard1080 != 0.0) && (var_guard1232 != 0.0)) && (var_guard1233 != 0.0)) {
        let assign41240_e47122: f64 = (var_q2s_chap__blk1067 - var_q2d_chap__blk1068);
        let assign41240_e47124: f64 = (assign41240_e47122 / var_norm_ids__blk1063);
        (assign41240_e47124, ((((var_q2s_chap__blk1067_dn4 - var_q2d_chap__blk1068_dn4) * var_norm_ids__blk1063) - (assign41240_e47122 * var_norm_ids__blk1063_dn4)) / (var_norm_ids__blk1063 * var_norm_ids__blk1063)), ((((var_q2s_chap__blk1067_dn6 - var_q2d_chap__blk1068_dn6) * var_norm_ids__blk1063) - (assign41240_e47122 * var_norm_ids__blk1063_dn6)) / (var_norm_ids__blk1063 * var_norm_ids__blk1063)), ((((var_q2s_chap__blk1067_dn7 - var_q2d_chap__blk1068_dn7) * var_norm_ids__blk1063) - (assign41240_e47122 * var_norm_ids__blk1063_dn7)) / (var_norm_ids__blk1063 * var_norm_ids__blk1063)), ((((var_q2s_chap__blk1067_dn8 - var_q2d_chap__blk1068_dn8) * var_norm_ids__blk1063) - (assign41240_e47122 * var_norm_ids__blk1063_dn8)) / (var_norm_ids__blk1063 * var_norm_ids__blk1063)), ((((var_q2s_chap__blk1067_dn9 - var_q2d_chap__blk1068_dn9) * var_norm_ids__blk1063) - (assign41240_e47122 * var_norm_ids__blk1063_dn9)) / (var_norm_ids__blk1063 * var_norm_ids__blk1063)),)
    } else {
        (var_inv_k2h2_0__blk1069, var_inv_k2h2_0__blk1069_dn4, var_inv_k2h2_0__blk1069_dn6, var_inv_k2h2_0__blk1069_dn7, var_inv_k2h2_0__blk1069_dn8, var_inv_k2h2_0__blk1069_dn9,)
    }
};
        var_inv_k2h2_0__blk1069 = assign41240_e47126;
        var_inv_k2h2_0__blk1069_dn4 = assign41240_e47126_d_n4;
        var_inv_k2h2_0__blk1069_dn6 = assign41240_e47126_d_n6;
        var_inv_k2h2_0__blk1069_dn7 = assign41240_e47126_d_n7;
        var_inv_k2h2_0__blk1069_dn8 = assign41240_e47126_d_n8;
        var_inv_k2h2_0__blk1069_dn9 = assign41240_e47126_d_n9;

        let (assign41250_e47135, assign41250_e47135_d_n4, assign41250_e47135_d_n6, assign41250_e47135_d_n7, assign41250_e47135_d_n8, assign41250_e47135_d_n9,) = {
    if (((var_guard1080 != 0.0) && (var_guard1232 != 0.0)) && (var_guard1233 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_inv_k1h1_0__blk1066, var_inv_k1h1_0__blk1066_dn4, var_inv_k1h1_0__blk1066_dn6, var_inv_k1h1_0__blk1066_dn7, var_inv_k1h1_0__blk1066_dn8, var_inv_k1h1_0__blk1066_dn9,)
    }
};
        var_inv_k1h1_0__blk1066 = assign41250_e47135;
        var_inv_k1h1_0__blk1066_dn4 = assign41250_e47135_d_n4;
        var_inv_k1h1_0__blk1066_dn6 = assign41250_e47135_d_n6;
        var_inv_k1h1_0__blk1066_dn7 = assign41250_e47135_d_n7;
        var_inv_k1h1_0__blk1066_dn8 = assign41250_e47135_d_n8;
        var_inv_k1h1_0__blk1066_dn9 = assign41250_e47135_d_n9;

        *var_dd__blk1057_slot = var_dd__blk1057;
        *var_dd__blk1057_dn4_slot = var_dd__blk1057_dn4;
        *var_dd__blk1057_dn6_slot = var_dd__blk1057_dn6;
        *var_dd__blk1057_dn7_slot = var_dd__blk1057_dn7;
        *var_dd__blk1057_dn8_slot = var_dd__blk1057_dn8;
        *var_dd__blk1057_dn9_slot = var_dd__blk1057_dn9;
        *var_dqid_dxn_qi__blk1056_slot = var_dqid_dxn_qi__blk1056;
        *var_dqid_dxn_qi__blk1056_dn4_slot = var_dqid_dxn_qi__blk1056_dn4;
        *var_dqid_dxn_qi__blk1056_dn6_slot = var_dqid_dxn_qi__blk1056_dn6;
        *var_dqid_dxn_qi__blk1056_dn7_slot = var_dqid_dxn_qi__blk1056_dn7;
        *var_dqid_dxn_qi__blk1056_dn8_slot = var_dqid_dxn_qi__blk1056_dn8;
        *var_dqid_dxn_qi__blk1056_dn9_slot = var_dqid_dxn_qi__blk1056_dn9;
        *var_guard1231_slot = var_guard1231;
        *var_guard1232_slot = var_guard1232;
        *var_guard1233_slot = var_guard1233;
        *var_idrift2__blk1062_slot = var_idrift2__blk1062;
        *var_idrift2__blk1062_dn4_slot = var_idrift2__blk1062_dn4;
        *var_idrift2__blk1062_dn6_slot = var_idrift2__blk1062_dn6;
        *var_idrift2__blk1062_dn7_slot = var_idrift2__blk1062_dn7;
        *var_idrift2__blk1062_dn8_slot = var_idrift2__blk1062_dn8;
        *var_idrift2__blk1062_dn9_slot = var_idrift2__blk1062_dn9;
        *var_inv_k1h1_0__blk1066_slot = var_inv_k1h1_0__blk1066;
        *var_inv_k1h1_0__blk1066_dn4_slot = var_inv_k1h1_0__blk1066_dn4;
        *var_inv_k1h1_0__blk1066_dn6_slot = var_inv_k1h1_0__blk1066_dn6;
        *var_inv_k1h1_0__blk1066_dn7_slot = var_inv_k1h1_0__blk1066_dn7;
        *var_inv_k1h1_0__blk1066_dn8_slot = var_inv_k1h1_0__blk1066_dn8;
        *var_inv_k1h1_0__blk1066_dn9_slot = var_inv_k1h1_0__blk1066_dn9;
        *var_inv_k2h2_0__blk1069_slot = var_inv_k2h2_0__blk1069;
        *var_inv_k2h2_0__blk1069_dn4_slot = var_inv_k2h2_0__blk1069_dn4;
        *var_inv_k2h2_0__blk1069_dn6_slot = var_inv_k2h2_0__blk1069_dn6;
        *var_inv_k2h2_0__blk1069_dn7_slot = var_inv_k2h2_0__blk1069_dn7;
        *var_inv_k2h2_0__blk1069_dn8_slot = var_inv_k2h2_0__blk1069_dn8;
        *var_inv_k2h2_0__blk1069_dn9_slot = var_inv_k2h2_0__blk1069_dn9;
        *var_ld__blk1059_slot = var_ld__blk1059;
        *var_ld__blk1059_dn4_slot = var_ld__blk1059_dn4;
        *var_ld__blk1059_dn6_slot = var_ld__blk1059_dn6;
        *var_ld__blk1059_dn7_slot = var_ld__blk1059_dn7;
        *var_ld__blk1059_dn8_slot = var_ld__blk1059_dn8;
        *var_ld__blk1059_dn9_slot = var_ld__blk1059_dn9;
        *var_ls__blk1058_slot = var_ls__blk1058;
        *var_ls__blk1058_dn4_slot = var_ls__blk1058_dn4;
        *var_ls__blk1058_dn6_slot = var_ls__blk1058_dn6;
        *var_ls__blk1058_dn7_slot = var_ls__blk1058_dn7;
        *var_ls__blk1058_dn8_slot = var_ls__blk1058_dn8;
        *var_ls__blk1058_dn9_slot = var_ls__blk1058_dn9;
        *var_norm_ids__blk1063_slot = var_norm_ids__blk1063;
        *var_norm_ids__blk1063_dn4_slot = var_norm_ids__blk1063_dn4;
        *var_norm_ids__blk1063_dn6_slot = var_norm_ids__blk1063_dn6;
        *var_norm_ids__blk1063_dn7_slot = var_norm_ids__blk1063_dn7;
        *var_norm_ids__blk1063_dn8_slot = var_norm_ids__blk1063_dn8;
        *var_norm_ids__blk1063_dn9_slot = var_norm_ids__blk1063_dn9;
        *var_q1d_chap__blk1065_slot = var_q1d_chap__blk1065;
        *var_q1d_chap__blk1065_dn4_slot = var_q1d_chap__blk1065_dn4;
        *var_q1d_chap__blk1065_dn6_slot = var_q1d_chap__blk1065_dn6;
        *var_q1d_chap__blk1065_dn7_slot = var_q1d_chap__blk1065_dn7;
        *var_q1d_chap__blk1065_dn8_slot = var_q1d_chap__blk1065_dn8;
        *var_q1d_chap__blk1065_dn9_slot = var_q1d_chap__blk1065_dn9;
        *var_q1s_chap__blk1064_slot = var_q1s_chap__blk1064;
        *var_q1s_chap__blk1064_dn4_slot = var_q1s_chap__blk1064_dn4;
        *var_q1s_chap__blk1064_dn6_slot = var_q1s_chap__blk1064_dn6;
        *var_q1s_chap__blk1064_dn7_slot = var_q1s_chap__blk1064_dn7;
        *var_q1s_chap__blk1064_dn8_slot = var_q1s_chap__blk1064_dn8;
        *var_q1s_chap__blk1064_dn9_slot = var_q1s_chap__blk1064_dn9;
        *var_q2d_chap__blk1068_slot = var_q2d_chap__blk1068;
        *var_q2d_chap__blk1068_dn4_slot = var_q2d_chap__blk1068_dn4;
        *var_q2d_chap__blk1068_dn6_slot = var_q2d_chap__blk1068_dn6;
        *var_q2d_chap__blk1068_dn7_slot = var_q2d_chap__blk1068_dn7;
        *var_q2d_chap__blk1068_dn8_slot = var_q2d_chap__blk1068_dn8;
        *var_q2d_chap__blk1068_dn9_slot = var_q2d_chap__blk1068_dn9;
        *var_q2s_chap__blk1067_slot = var_q2s_chap__blk1067;
        *var_q2s_chap__blk1067_dn4_slot = var_q2s_chap__blk1067_dn4;
        *var_q2s_chap__blk1067_dn6_slot = var_q2s_chap__blk1067_dn6;
        *var_q2s_chap__blk1067_dn7_slot = var_q2s_chap__blk1067_dn7;
        *var_q2s_chap__blk1067_dn8_slot = var_q2s_chap__blk1067_dn8;
        *var_q2s_chap__blk1067_dn9_slot = var_q2s_chap__blk1067_dn9;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp3_slot = var_temp3;
        *var_temp3_dn4_slot = var_temp3_dn4;
        *var_temp3_dn6_slot = var_temp3_dn6;
        *var_temp3_dn7_slot = var_temp3_dn7;
        *var_temp3_dn8_slot = var_temp3_dn8;
        *var_temp3_dn9_slot = var_temp3_dn9;
        *var_temp4_slot = var_temp4;
        *var_temp4_dn4_slot = var_temp4_dn4;
        *var_temp4_dn6_slot = var_temp4_dn6;
        *var_temp4_dn7_slot = var_temp4_dn7;
        *var_temp4_dn8_slot = var_temp4_dn8;
        *var_temp4_dn9_slot = var_temp4_dn9;
        *var_ud__blk1061_slot = var_ud__blk1061;
        *var_ud__blk1061_dn4_slot = var_ud__blk1061_dn4;
        *var_ud__blk1061_dn6_slot = var_ud__blk1061_dn6;
        *var_ud__blk1061_dn7_slot = var_ud__blk1061_dn7;
        *var_ud__blk1061_dn8_slot = var_ud__blk1061_dn8;
        *var_ud__blk1061_dn9_slot = var_ud__blk1061_dn9;
        *var_us__blk1060_slot = var_us__blk1060;
        *var_us__blk1060_dn4_slot = var_us__blk1060_dn4;
        *var_us__blk1060_dn6_slot = var_us__blk1060_dn6;
        *var_us__blk1060_dn7_slot = var_us__blk1060_dn7;
        *var_us__blk1060_dn8_slot = var_us__blk1060_dn8;
        *var_us__blk1060_dn9_slot = var_us__blk1060_dn9;
    }

    pub(super) fn stamp_transient_block_115(
        var_a0__blk905: f64,
        var_a0__blk905_dn4: f64,
        var_a0__blk905_dn6: f64,
        var_a0__blk905_dn7: f64,
        var_a0__blk905_dn8: f64,
        var_a0__blk905_dn9: f64,
        var_csiprime__blk919: f64,
        var_csiprime__blk919_dn4: f64,
        var_csiprime__blk919_dn6: f64,
        var_csiprime__blk919_dn7: f64,
        var_csiprime__blk919_dn8: f64,
        var_csiprime__blk919_dn9: f64,
        var_diff_min__blk904: f64,
        var_diff_min__blk904_dn4: f64,
        var_diff_min__blk904_dn6: f64,
        var_diff_min__blk904_dn7: f64,
        var_diff_min__blk904_dn8: f64,
        var_diff_min__blk904_dn9: f64,
        var_dleff__blk922: f64,
        var_dleff__blk922_dn4: f64,
        var_dleff__blk922_dn6: f64,
        var_dleff__blk922_dn7: f64,
        var_dleff__blk922_dn8: f64,
        var_dleff__blk922_dn9: f64,
        var_dx_wi_1d__blk918: f64,
        var_dx_wi_1d__blk918_dn4: f64,
        var_dx_wi_1d__blk918_dn6: f64,
        var_dx_wi_1d__blk918_dn7: f64,
        var_dx_wi_1d__blk918_dn8: f64,
        var_dx_wi_1d__blk918_dn9: f64,
        var_dx_wi__blk935: f64,
        var_dx_wi__blk935_dn4: f64,
        var_dx_wi__blk935_dn6: f64,
        var_dx_wi__blk935_dn7: f64,
        var_dx_wi__blk935_dn8: f64,
        var_dx_wi__blk935_dn9: f64,
        var_dxg1_dibl__blk926: f64,
        var_dxg1_dibl__blk926_dn4: f64,
        var_dxg1_dibl__blk926_dn6: f64,
        var_dxg1_dibl__blk926_dn7: f64,
        var_dxg1_dibl__blk926_dn8: f64,
        var_dxg1_dibl__blk926_dn9: f64,
        var_guard1080: f64,
        var_guard1232: f64,
        var_guard1233: f64,
        var_hsat__blk1053: f64,
        var_hsat__blk1053_dn4: f64,
        var_hsat__blk1053_dn6: f64,
        var_hsat__blk1053_dn7: f64,
        var_hsat__blk1053_dn8: f64,
        var_hsat__blk1053_dn9: f64,
        var_inv_dinf__blk975: f64,
        var_inv_dinf__blk975_dn4: f64,
        var_inv_dinf__blk975_dn6: f64,
        var_inv_dinf__blk975_dn7: f64,
        var_inv_dinf__blk975_dn8: f64,
        var_inv_dinf__blk975_dn9: f64,
        var_inv_k1__blk906: f64,
        var_inv_k1__blk906_dn4: f64,
        var_inv_k1__blk906_dn6: f64,
        var_inv_k1__blk906_dn7: f64,
        var_inv_k1__blk906_dn8: f64,
        var_inv_k1__blk906_dn9: f64,
        var_inv_k2__blk907: f64,
        var_inv_k2__blk907_dn4: f64,
        var_inv_k2__blk907_dn6: f64,
        var_inv_k2__blk907_dn7: f64,
        var_inv_k2__blk907_dn8: f64,
        var_inv_k2__blk907_dn9: f64,
        var_k1q1d__blk1004: f64,
        var_k1q1d__blk1004_dn4: f64,
        var_k1q1d__blk1004_dn6: f64,
        var_k1q1d__blk1004_dn7: f64,
        var_k1q1d__blk1004_dn8: f64,
        var_k1q1d__blk1004_dn9: f64,
        var_k1q1s__blk939: f64,
        var_k1q1s__blk939_dn4: f64,
        var_k1q1s__blk939_dn6: f64,
        var_k1q1s__blk939_dn7: f64,
        var_k1q1s__blk939_dn8: f64,
        var_k1q1s__blk939_dn9: f64,
        var_k2q2d__blk1005: f64,
        var_k2q2d__blk1005_dn4: f64,
        var_k2q2d__blk1005_dn6: f64,
        var_k2q2d__blk1005_dn7: f64,
        var_k2q2d__blk1005_dn8: f64,
        var_k2q2d__blk1005_dn9: f64,
        var_k2q2s__blk940: f64,
        var_k2q2s__blk940_dn4: f64,
        var_k2q2s__blk940_dn6: f64,
        var_k2q2s__blk940_dn7: f64,
        var_k2q2s__blk940_dn8: f64,
        var_k2q2s__blk940_dn9: f64,
        var_keq__blk934: f64,
        var_keq__blk934_dn4: f64,
        var_keq__blk934_dn6: f64,
        var_keq__blk934_dn7: f64,
        var_keq__blk934_dn8: f64,
        var_keq__blk934_dn9: f64,
        var_q1chapinf__blk972: f64,
        var_q1chapinf__blk972_dn4: f64,
        var_q1chapinf__blk972_dn6: f64,
        var_q1chapinf__blk972_dn7: f64,
        var_q1chapinf__blk972_dn8: f64,
        var_q1chapinf__blk972_dn9: f64,
        var_q2chapinf__blk973: f64,
        var_q2chapinf__blk973_dn4: f64,
        var_q2chapinf__blk973_dn6: f64,
        var_q2chapinf__blk973_dn7: f64,
        var_q2chapinf__blk973_dn8: f64,
        var_q2chapinf__blk973_dn9: f64,
        var_s1__blk969: f64,
        var_s1__blk969_dn4: f64,
        var_s1__blk969_dn6: f64,
        var_s1__blk969_dn7: f64,
        var_s1__blk969_dn8: f64,
        var_s1__blk969_dn9: f64,
        var_s2__blk970: f64,
        var_s2__blk970_dn4: f64,
        var_s2__blk970_dn6: f64,
        var_s2__blk970_dn7: f64,
        var_s2__blk970_dn8: f64,
        var_s2__blk970_dn9: f64,
        var_sce1__blk924: f64,
        var_sce1__blk924_dn4: f64,
        var_sce1__blk924_dn6: f64,
        var_sce1__blk924_dn7: f64,
        var_sce1__blk924_dn8: f64,
        var_sce1__blk924_dn9: f64,
        var_sce2__blk925: f64,
        var_sce2__blk925_dn4: f64,
        var_sce2__blk925_dn6: f64,
        var_sce2__blk925_dn7: f64,
        var_sce2__blk925_dn8: f64,
        var_sce2__blk925_dn9: f64,
        var_xedge__blk923: f64,
        var_xedge__blk923_dn4: f64,
        var_xedge__blk923_dn6: f64,
        var_xedge__blk923_dn7: f64,
        var_xedge__blk923_dn8: f64,
        var_xedge__blk923_dn9: f64,
        var_xg20shift__blk900: f64,
        var_xg20shift__blk900_dn4: f64,
        var_xg20shift__blk900_dn6: f64,
        var_xg20shift__blk900_dn7: f64,
        var_xg20shift__blk900_dn8: f64,
        var_xg20shift__blk900_dn9: f64,
        var_xg2__blk929: f64,
        var_xg2__blk929_dn4: f64,
        var_xg2__blk929_dn6: f64,
        var_xg2__blk929_dn7: f64,
        var_xg2__blk929_dn8: f64,
        var_xg2__blk929_dn9: f64,
        var_a0_ac_slot: &mut f64,
        var_a0_ac_dn4_slot: &mut f64,
        var_a0_ac_dn6_slot: &mut f64,
        var_a0_ac_dn7_slot: &mut f64,
        var_a0_ac_dn8_slot: &mut f64,
        var_a0_ac_dn9_slot: &mut f64,
        var_csiprime_ac_slot: &mut f64,
        var_csiprime_ac_dn4_slot: &mut f64,
        var_csiprime_ac_dn6_slot: &mut f64,
        var_csiprime_ac_dn7_slot: &mut f64,
        var_csiprime_ac_dn8_slot: &mut f64,
        var_csiprime_ac_dn9_slot: &mut f64,
        var_delta_k1q1__blk1076_slot: &mut f64,
        var_delta_k1q1__blk1076_dn4_slot: &mut f64,
        var_delta_k1q1__blk1076_dn6_slot: &mut f64,
        var_delta_k1q1__blk1076_dn7_slot: &mut f64,
        var_delta_k1q1__blk1076_dn8_slot: &mut f64,
        var_delta_k1q1__blk1076_dn9_slot: &mut f64,
        var_delta_k2q2__blk1077_slot: &mut f64,
        var_delta_k2q2__blk1077_dn4_slot: &mut f64,
        var_delta_k2q2__blk1077_dn6_slot: &mut f64,
        var_delta_k2q2__blk1077_dn7_slot: &mut f64,
        var_delta_k2q2__blk1077_dn8_slot: &mut f64,
        var_delta_k2q2__blk1077_dn9_slot: &mut f64,
        var_diff_min_ac_slot: &mut f64,
        var_diff_min_ac_dn4_slot: &mut f64,
        var_diff_min_ac_dn6_slot: &mut f64,
        var_diff_min_ac_dn7_slot: &mut f64,
        var_diff_min_ac_dn8_slot: &mut f64,
        var_diff_min_ac_dn9_slot: &mut f64,
        var_dleff_ac_slot: &mut f64,
        var_dleff_ac_dn4_slot: &mut f64,
        var_dleff_ac_dn6_slot: &mut f64,
        var_dleff_ac_dn7_slot: &mut f64,
        var_dleff_ac_dn8_slot: &mut f64,
        var_dleff_ac_dn9_slot: &mut f64,
        var_dx_wi_1d_ac_slot: &mut f64,
        var_dx_wi_1d_ac_dn4_slot: &mut f64,
        var_dx_wi_1d_ac_dn6_slot: &mut f64,
        var_dx_wi_1d_ac_dn7_slot: &mut f64,
        var_dx_wi_1d_ac_dn8_slot: &mut f64,
        var_dx_wi_1d_ac_dn9_slot: &mut f64,
        var_dx_wi_ac_slot: &mut f64,
        var_dx_wi_ac_dn4_slot: &mut f64,
        var_dx_wi_ac_dn6_slot: &mut f64,
        var_dx_wi_ac_dn7_slot: &mut f64,
        var_dx_wi_ac_dn8_slot: &mut f64,
        var_dx_wi_ac_dn9_slot: &mut f64,
        var_dxg1_dibl_ac_slot: &mut f64,
        var_dxg1_dibl_ac_dn4_slot: &mut f64,
        var_dxg1_dibl_ac_dn6_slot: &mut f64,
        var_dxg1_dibl_ac_dn7_slot: &mut f64,
        var_dxg1_dibl_ac_dn8_slot: &mut f64,
        var_dxg1_dibl_ac_dn9_slot: &mut f64,
        var_inv_k1_ac_slot: &mut f64,
        var_inv_k1_ac_dn4_slot: &mut f64,
        var_inv_k1_ac_dn6_slot: &mut f64,
        var_inv_k1_ac_dn7_slot: &mut f64,
        var_inv_k1_ac_dn8_slot: &mut f64,
        var_inv_k1_ac_dn9_slot: &mut f64,
        var_inv_k1h1_0__blk1066_slot: &mut f64,
        var_inv_k1h1_0__blk1066_dn4_slot: &mut f64,
        var_inv_k1h1_0__blk1066_dn6_slot: &mut f64,
        var_inv_k1h1_0__blk1066_dn7_slot: &mut f64,
        var_inv_k1h1_0__blk1066_dn8_slot: &mut f64,
        var_inv_k1h1_0__blk1066_dn9_slot: &mut f64,
        var_inv_k1h1__blk1074_slot: &mut f64,
        var_inv_k1h1__blk1074_dn4_slot: &mut f64,
        var_inv_k1h1__blk1074_dn6_slot: &mut f64,
        var_inv_k1h1__blk1074_dn7_slot: &mut f64,
        var_inv_k1h1__blk1074_dn8_slot: &mut f64,
        var_inv_k1h1__blk1074_dn9_slot: &mut f64,
        var_inv_k2_ac_slot: &mut f64,
        var_inv_k2_ac_dn4_slot: &mut f64,
        var_inv_k2_ac_dn6_slot: &mut f64,
        var_inv_k2_ac_dn7_slot: &mut f64,
        var_inv_k2_ac_dn8_slot: &mut f64,
        var_inv_k2_ac_dn9_slot: &mut f64,
        var_inv_k2h2_0__blk1069_slot: &mut f64,
        var_inv_k2h2_0__blk1069_dn4_slot: &mut f64,
        var_inv_k2h2_0__blk1069_dn6_slot: &mut f64,
        var_inv_k2h2_0__blk1069_dn7_slot: &mut f64,
        var_inv_k2h2_0__blk1069_dn8_slot: &mut f64,
        var_inv_k2h2_0__blk1069_dn9_slot: &mut f64,
        var_inv_k2h2__blk1075_slot: &mut f64,
        var_inv_k2h2__blk1075_dn4_slot: &mut f64,
        var_inv_k2h2__blk1075_dn6_slot: &mut f64,
        var_inv_k2h2__blk1075_dn7_slot: &mut f64,
        var_inv_k2h2__blk1075_dn8_slot: &mut f64,
        var_inv_k2h2__blk1075_dn9_slot: &mut f64,
        var_keq_ac_slot: &mut f64,
        var_keq_ac_dn4_slot: &mut f64,
        var_keq_ac_dn6_slot: &mut f64,
        var_keq_ac_dn7_slot: &mut f64,
        var_keq_ac_dn8_slot: &mut f64,
        var_keq_ac_dn9_slot: &mut f64,
        var_ksi1__blk1072_slot: &mut f64,
        var_ksi1__blk1072_dn4_slot: &mut f64,
        var_ksi1__blk1072_dn6_slot: &mut f64,
        var_ksi1__blk1072_dn7_slot: &mut f64,
        var_ksi1__blk1072_dn8_slot: &mut f64,
        var_ksi1__blk1072_dn9_slot: &mut f64,
        var_ksi2__blk1073_slot: &mut f64,
        var_ksi2__blk1073_dn4_slot: &mut f64,
        var_ksi2__blk1073_dn6_slot: &mut f64,
        var_ksi2__blk1073_dn7_slot: &mut f64,
        var_ksi2__blk1073_dn8_slot: &mut f64,
        var_ksi2__blk1073_dn9_slot: &mut f64,
        var_prod1__blk1078_slot: &mut f64,
        var_prod1__blk1078_dn4_slot: &mut f64,
        var_prod1__blk1078_dn6_slot: &mut f64,
        var_prod1__blk1078_dn7_slot: &mut f64,
        var_prod1__blk1078_dn8_slot: &mut f64,
        var_prod1__blk1078_dn9_slot: &mut f64,
        var_prod2__blk1079_slot: &mut f64,
        var_prod2__blk1079_dn4_slot: &mut f64,
        var_prod2__blk1079_dn6_slot: &mut f64,
        var_prod2__blk1079_dn7_slot: &mut f64,
        var_prod2__blk1079_dn8_slot: &mut f64,
        var_prod2__blk1079_dn9_slot: &mut f64,
        var_sce1_ac_slot: &mut f64,
        var_sce1_ac_dn4_slot: &mut f64,
        var_sce1_ac_dn6_slot: &mut f64,
        var_sce1_ac_dn7_slot: &mut f64,
        var_sce1_ac_dn8_slot: &mut f64,
        var_sce1_ac_dn9_slot: &mut f64,
        var_sce2_ac_slot: &mut f64,
        var_sce2_ac_dn4_slot: &mut f64,
        var_sce2_ac_dn6_slot: &mut f64,
        var_sce2_ac_dn7_slot: &mut f64,
        var_sce2_ac_dn8_slot: &mut f64,
        var_sce2_ac_dn9_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp3_slot: &mut f64,
        var_temp3_dn4_slot: &mut f64,
        var_temp3_dn6_slot: &mut f64,
        var_temp3_dn7_slot: &mut f64,
        var_temp3_dn8_slot: &mut f64,
        var_temp3_dn9_slot: &mut f64,
        var_temp4_slot: &mut f64,
        var_temp4_dn4_slot: &mut f64,
        var_temp4_dn6_slot: &mut f64,
        var_temp4_dn7_slot: &mut f64,
        var_temp4_dn8_slot: &mut f64,
        var_temp4_dn9_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_xedge_ac_slot: &mut f64,
        var_xedge_ac_dn4_slot: &mut f64,
        var_xedge_ac_dn6_slot: &mut f64,
        var_xedge_ac_dn7_slot: &mut f64,
        var_xedge_ac_dn8_slot: &mut f64,
        var_xedge_ac_dn9_slot: &mut f64,
        var_xg20shift_ac_slot: &mut f64,
        var_xg20shift_ac_dn4_slot: &mut f64,
        var_xg20shift_ac_dn6_slot: &mut f64,
        var_xg20shift_ac_dn7_slot: &mut f64,
        var_xg20shift_ac_dn8_slot: &mut f64,
        var_xg20shift_ac_dn9_slot: &mut f64,
        var_xg2_ac_slot: &mut f64,
        var_xg2_ac_dn4_slot: &mut f64,
        var_xg2_ac_dn6_slot: &mut f64,
        var_xg2_ac_dn7_slot: &mut f64,
        var_xg2_ac_dn8_slot: &mut f64,
        var_xg2_ac_dn9_slot: &mut f64,
        var_zeta1__blk1070_slot: &mut f64,
        var_zeta1__blk1070_dn4_slot: &mut f64,
        var_zeta1__blk1070_dn6_slot: &mut f64,
        var_zeta1__blk1070_dn7_slot: &mut f64,
        var_zeta1__blk1070_dn8_slot: &mut f64,
        var_zeta1__blk1070_dn9_slot: &mut f64,
        var_zeta2__blk1071_slot: &mut f64,
        var_zeta2__blk1071_dn4_slot: &mut f64,
        var_zeta2__blk1071_dn6_slot: &mut f64,
        var_zeta2__blk1071_dn7_slot: &mut f64,
        var_zeta2__blk1071_dn8_slot: &mut f64,
        var_zeta2__blk1071_dn9_slot: &mut f64,
    ) {
        let mut var_a0_ac: f64 = *var_a0_ac_slot;
        let mut var_a0_ac_dn4: f64 = *var_a0_ac_dn4_slot;
        let mut var_a0_ac_dn6: f64 = *var_a0_ac_dn6_slot;
        let mut var_a0_ac_dn7: f64 = *var_a0_ac_dn7_slot;
        let mut var_a0_ac_dn8: f64 = *var_a0_ac_dn8_slot;
        let mut var_a0_ac_dn9: f64 = *var_a0_ac_dn9_slot;
        let mut var_csiprime_ac: f64 = *var_csiprime_ac_slot;
        let mut var_csiprime_ac_dn4: f64 = *var_csiprime_ac_dn4_slot;
        let mut var_csiprime_ac_dn6: f64 = *var_csiprime_ac_dn6_slot;
        let mut var_csiprime_ac_dn7: f64 = *var_csiprime_ac_dn7_slot;
        let mut var_csiprime_ac_dn8: f64 = *var_csiprime_ac_dn8_slot;
        let mut var_csiprime_ac_dn9: f64 = *var_csiprime_ac_dn9_slot;
        let mut var_delta_k1q1__blk1076: f64 = *var_delta_k1q1__blk1076_slot;
        let mut var_delta_k1q1__blk1076_dn4: f64 = *var_delta_k1q1__blk1076_dn4_slot;
        let mut var_delta_k1q1__blk1076_dn6: f64 = *var_delta_k1q1__blk1076_dn6_slot;
        let mut var_delta_k1q1__blk1076_dn7: f64 = *var_delta_k1q1__blk1076_dn7_slot;
        let mut var_delta_k1q1__blk1076_dn8: f64 = *var_delta_k1q1__blk1076_dn8_slot;
        let mut var_delta_k1q1__blk1076_dn9: f64 = *var_delta_k1q1__blk1076_dn9_slot;
        let mut var_delta_k2q2__blk1077: f64 = *var_delta_k2q2__blk1077_slot;
        let mut var_delta_k2q2__blk1077_dn4: f64 = *var_delta_k2q2__blk1077_dn4_slot;
        let mut var_delta_k2q2__blk1077_dn6: f64 = *var_delta_k2q2__blk1077_dn6_slot;
        let mut var_delta_k2q2__blk1077_dn7: f64 = *var_delta_k2q2__blk1077_dn7_slot;
        let mut var_delta_k2q2__blk1077_dn8: f64 = *var_delta_k2q2__blk1077_dn8_slot;
        let mut var_delta_k2q2__blk1077_dn9: f64 = *var_delta_k2q2__blk1077_dn9_slot;
        let mut var_diff_min_ac: f64 = *var_diff_min_ac_slot;
        let mut var_diff_min_ac_dn4: f64 = *var_diff_min_ac_dn4_slot;
        let mut var_diff_min_ac_dn6: f64 = *var_diff_min_ac_dn6_slot;
        let mut var_diff_min_ac_dn7: f64 = *var_diff_min_ac_dn7_slot;
        let mut var_diff_min_ac_dn8: f64 = *var_diff_min_ac_dn8_slot;
        let mut var_diff_min_ac_dn9: f64 = *var_diff_min_ac_dn9_slot;
        let mut var_dleff_ac: f64 = *var_dleff_ac_slot;
        let mut var_dleff_ac_dn4: f64 = *var_dleff_ac_dn4_slot;
        let mut var_dleff_ac_dn6: f64 = *var_dleff_ac_dn6_slot;
        let mut var_dleff_ac_dn7: f64 = *var_dleff_ac_dn7_slot;
        let mut var_dleff_ac_dn8: f64 = *var_dleff_ac_dn8_slot;
        let mut var_dleff_ac_dn9: f64 = *var_dleff_ac_dn9_slot;
        let mut var_dx_wi_1d_ac: f64 = *var_dx_wi_1d_ac_slot;
        let mut var_dx_wi_1d_ac_dn4: f64 = *var_dx_wi_1d_ac_dn4_slot;
        let mut var_dx_wi_1d_ac_dn6: f64 = *var_dx_wi_1d_ac_dn6_slot;
        let mut var_dx_wi_1d_ac_dn7: f64 = *var_dx_wi_1d_ac_dn7_slot;
        let mut var_dx_wi_1d_ac_dn8: f64 = *var_dx_wi_1d_ac_dn8_slot;
        let mut var_dx_wi_1d_ac_dn9: f64 = *var_dx_wi_1d_ac_dn9_slot;
        let mut var_dx_wi_ac: f64 = *var_dx_wi_ac_slot;
        let mut var_dx_wi_ac_dn4: f64 = *var_dx_wi_ac_dn4_slot;
        let mut var_dx_wi_ac_dn6: f64 = *var_dx_wi_ac_dn6_slot;
        let mut var_dx_wi_ac_dn7: f64 = *var_dx_wi_ac_dn7_slot;
        let mut var_dx_wi_ac_dn8: f64 = *var_dx_wi_ac_dn8_slot;
        let mut var_dx_wi_ac_dn9: f64 = *var_dx_wi_ac_dn9_slot;
        let mut var_dxg1_dibl_ac: f64 = *var_dxg1_dibl_ac_slot;
        let mut var_dxg1_dibl_ac_dn4: f64 = *var_dxg1_dibl_ac_dn4_slot;
        let mut var_dxg1_dibl_ac_dn6: f64 = *var_dxg1_dibl_ac_dn6_slot;
        let mut var_dxg1_dibl_ac_dn7: f64 = *var_dxg1_dibl_ac_dn7_slot;
        let mut var_dxg1_dibl_ac_dn8: f64 = *var_dxg1_dibl_ac_dn8_slot;
        let mut var_dxg1_dibl_ac_dn9: f64 = *var_dxg1_dibl_ac_dn9_slot;
        let mut var_inv_k1_ac: f64 = *var_inv_k1_ac_slot;
        let mut var_inv_k1_ac_dn4: f64 = *var_inv_k1_ac_dn4_slot;
        let mut var_inv_k1_ac_dn6: f64 = *var_inv_k1_ac_dn6_slot;
        let mut var_inv_k1_ac_dn7: f64 = *var_inv_k1_ac_dn7_slot;
        let mut var_inv_k1_ac_dn8: f64 = *var_inv_k1_ac_dn8_slot;
        let mut var_inv_k1_ac_dn9: f64 = *var_inv_k1_ac_dn9_slot;
        let mut var_inv_k1h1_0__blk1066: f64 = *var_inv_k1h1_0__blk1066_slot;
        let mut var_inv_k1h1_0__blk1066_dn4: f64 = *var_inv_k1h1_0__blk1066_dn4_slot;
        let mut var_inv_k1h1_0__blk1066_dn6: f64 = *var_inv_k1h1_0__blk1066_dn6_slot;
        let mut var_inv_k1h1_0__blk1066_dn7: f64 = *var_inv_k1h1_0__blk1066_dn7_slot;
        let mut var_inv_k1h1_0__blk1066_dn8: f64 = *var_inv_k1h1_0__blk1066_dn8_slot;
        let mut var_inv_k1h1_0__blk1066_dn9: f64 = *var_inv_k1h1_0__blk1066_dn9_slot;
        let mut var_inv_k1h1__blk1074: f64 = *var_inv_k1h1__blk1074_slot;
        let mut var_inv_k1h1__blk1074_dn4: f64 = *var_inv_k1h1__blk1074_dn4_slot;
        let mut var_inv_k1h1__blk1074_dn6: f64 = *var_inv_k1h1__blk1074_dn6_slot;
        let mut var_inv_k1h1__blk1074_dn7: f64 = *var_inv_k1h1__blk1074_dn7_slot;
        let mut var_inv_k1h1__blk1074_dn8: f64 = *var_inv_k1h1__blk1074_dn8_slot;
        let mut var_inv_k1h1__blk1074_dn9: f64 = *var_inv_k1h1__blk1074_dn9_slot;
        let mut var_inv_k2_ac: f64 = *var_inv_k2_ac_slot;
        let mut var_inv_k2_ac_dn4: f64 = *var_inv_k2_ac_dn4_slot;
        let mut var_inv_k2_ac_dn6: f64 = *var_inv_k2_ac_dn6_slot;
        let mut var_inv_k2_ac_dn7: f64 = *var_inv_k2_ac_dn7_slot;
        let mut var_inv_k2_ac_dn8: f64 = *var_inv_k2_ac_dn8_slot;
        let mut var_inv_k2_ac_dn9: f64 = *var_inv_k2_ac_dn9_slot;
        let mut var_inv_k2h2_0__blk1069: f64 = *var_inv_k2h2_0__blk1069_slot;
        let mut var_inv_k2h2_0__blk1069_dn4: f64 = *var_inv_k2h2_0__blk1069_dn4_slot;
        let mut var_inv_k2h2_0__blk1069_dn6: f64 = *var_inv_k2h2_0__blk1069_dn6_slot;
        let mut var_inv_k2h2_0__blk1069_dn7: f64 = *var_inv_k2h2_0__blk1069_dn7_slot;
        let mut var_inv_k2h2_0__blk1069_dn8: f64 = *var_inv_k2h2_0__blk1069_dn8_slot;
        let mut var_inv_k2h2_0__blk1069_dn9: f64 = *var_inv_k2h2_0__blk1069_dn9_slot;
        let mut var_inv_k2h2__blk1075: f64 = *var_inv_k2h2__blk1075_slot;
        let mut var_inv_k2h2__blk1075_dn4: f64 = *var_inv_k2h2__blk1075_dn4_slot;
        let mut var_inv_k2h2__blk1075_dn6: f64 = *var_inv_k2h2__blk1075_dn6_slot;
        let mut var_inv_k2h2__blk1075_dn7: f64 = *var_inv_k2h2__blk1075_dn7_slot;
        let mut var_inv_k2h2__blk1075_dn8: f64 = *var_inv_k2h2__blk1075_dn8_slot;
        let mut var_inv_k2h2__blk1075_dn9: f64 = *var_inv_k2h2__blk1075_dn9_slot;
        let mut var_keq_ac: f64 = *var_keq_ac_slot;
        let mut var_keq_ac_dn4: f64 = *var_keq_ac_dn4_slot;
        let mut var_keq_ac_dn6: f64 = *var_keq_ac_dn6_slot;
        let mut var_keq_ac_dn7: f64 = *var_keq_ac_dn7_slot;
        let mut var_keq_ac_dn8: f64 = *var_keq_ac_dn8_slot;
        let mut var_keq_ac_dn9: f64 = *var_keq_ac_dn9_slot;
        let mut var_ksi1__blk1072: f64 = *var_ksi1__blk1072_slot;
        let mut var_ksi1__blk1072_dn4: f64 = *var_ksi1__blk1072_dn4_slot;
        let mut var_ksi1__blk1072_dn6: f64 = *var_ksi1__blk1072_dn6_slot;
        let mut var_ksi1__blk1072_dn7: f64 = *var_ksi1__blk1072_dn7_slot;
        let mut var_ksi1__blk1072_dn8: f64 = *var_ksi1__blk1072_dn8_slot;
        let mut var_ksi1__blk1072_dn9: f64 = *var_ksi1__blk1072_dn9_slot;
        let mut var_ksi2__blk1073: f64 = *var_ksi2__blk1073_slot;
        let mut var_ksi2__blk1073_dn4: f64 = *var_ksi2__blk1073_dn4_slot;
        let mut var_ksi2__blk1073_dn6: f64 = *var_ksi2__blk1073_dn6_slot;
        let mut var_ksi2__blk1073_dn7: f64 = *var_ksi2__blk1073_dn7_slot;
        let mut var_ksi2__blk1073_dn8: f64 = *var_ksi2__blk1073_dn8_slot;
        let mut var_ksi2__blk1073_dn9: f64 = *var_ksi2__blk1073_dn9_slot;
        let mut var_prod1__blk1078: f64 = *var_prod1__blk1078_slot;
        let mut var_prod1__blk1078_dn4: f64 = *var_prod1__blk1078_dn4_slot;
        let mut var_prod1__blk1078_dn6: f64 = *var_prod1__blk1078_dn6_slot;
        let mut var_prod1__blk1078_dn7: f64 = *var_prod1__blk1078_dn7_slot;
        let mut var_prod1__blk1078_dn8: f64 = *var_prod1__blk1078_dn8_slot;
        let mut var_prod1__blk1078_dn9: f64 = *var_prod1__blk1078_dn9_slot;
        let mut var_prod2__blk1079: f64 = *var_prod2__blk1079_slot;
        let mut var_prod2__blk1079_dn4: f64 = *var_prod2__blk1079_dn4_slot;
        let mut var_prod2__blk1079_dn6: f64 = *var_prod2__blk1079_dn6_slot;
        let mut var_prod2__blk1079_dn7: f64 = *var_prod2__blk1079_dn7_slot;
        let mut var_prod2__blk1079_dn8: f64 = *var_prod2__blk1079_dn8_slot;
        let mut var_prod2__blk1079_dn9: f64 = *var_prod2__blk1079_dn9_slot;
        let mut var_sce1_ac: f64 = *var_sce1_ac_slot;
        let mut var_sce1_ac_dn4: f64 = *var_sce1_ac_dn4_slot;
        let mut var_sce1_ac_dn6: f64 = *var_sce1_ac_dn6_slot;
        let mut var_sce1_ac_dn7: f64 = *var_sce1_ac_dn7_slot;
        let mut var_sce1_ac_dn8: f64 = *var_sce1_ac_dn8_slot;
        let mut var_sce1_ac_dn9: f64 = *var_sce1_ac_dn9_slot;
        let mut var_sce2_ac: f64 = *var_sce2_ac_slot;
        let mut var_sce2_ac_dn4: f64 = *var_sce2_ac_dn4_slot;
        let mut var_sce2_ac_dn6: f64 = *var_sce2_ac_dn6_slot;
        let mut var_sce2_ac_dn7: f64 = *var_sce2_ac_dn7_slot;
        let mut var_sce2_ac_dn8: f64 = *var_sce2_ac_dn8_slot;
        let mut var_sce2_ac_dn9: f64 = *var_sce2_ac_dn9_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp3: f64 = *var_temp3_slot;
        let mut var_temp3_dn4: f64 = *var_temp3_dn4_slot;
        let mut var_temp3_dn6: f64 = *var_temp3_dn6_slot;
        let mut var_temp3_dn7: f64 = *var_temp3_dn7_slot;
        let mut var_temp3_dn8: f64 = *var_temp3_dn8_slot;
        let mut var_temp3_dn9: f64 = *var_temp3_dn9_slot;
        let mut var_temp4: f64 = *var_temp4_slot;
        let mut var_temp4_dn4: f64 = *var_temp4_dn4_slot;
        let mut var_temp4_dn6: f64 = *var_temp4_dn6_slot;
        let mut var_temp4_dn7: f64 = *var_temp4_dn7_slot;
        let mut var_temp4_dn8: f64 = *var_temp4_dn8_slot;
        let mut var_temp4_dn9: f64 = *var_temp4_dn9_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_xedge_ac: f64 = *var_xedge_ac_slot;
        let mut var_xedge_ac_dn4: f64 = *var_xedge_ac_dn4_slot;
        let mut var_xedge_ac_dn6: f64 = *var_xedge_ac_dn6_slot;
        let mut var_xedge_ac_dn7: f64 = *var_xedge_ac_dn7_slot;
        let mut var_xedge_ac_dn8: f64 = *var_xedge_ac_dn8_slot;
        let mut var_xedge_ac_dn9: f64 = *var_xedge_ac_dn9_slot;
        let mut var_xg20shift_ac: f64 = *var_xg20shift_ac_slot;
        let mut var_xg20shift_ac_dn4: f64 = *var_xg20shift_ac_dn4_slot;
        let mut var_xg20shift_ac_dn6: f64 = *var_xg20shift_ac_dn6_slot;
        let mut var_xg20shift_ac_dn7: f64 = *var_xg20shift_ac_dn7_slot;
        let mut var_xg20shift_ac_dn8: f64 = *var_xg20shift_ac_dn8_slot;
        let mut var_xg20shift_ac_dn9: f64 = *var_xg20shift_ac_dn9_slot;
        let mut var_xg2_ac: f64 = *var_xg2_ac_slot;
        let mut var_xg2_ac_dn4: f64 = *var_xg2_ac_dn4_slot;
        let mut var_xg2_ac_dn6: f64 = *var_xg2_ac_dn6_slot;
        let mut var_xg2_ac_dn7: f64 = *var_xg2_ac_dn7_slot;
        let mut var_xg2_ac_dn8: f64 = *var_xg2_ac_dn8_slot;
        let mut var_xg2_ac_dn9: f64 = *var_xg2_ac_dn9_slot;
        let mut var_zeta1__blk1070: f64 = *var_zeta1__blk1070_slot;
        let mut var_zeta1__blk1070_dn4: f64 = *var_zeta1__blk1070_dn4_slot;
        let mut var_zeta1__blk1070_dn6: f64 = *var_zeta1__blk1070_dn6_slot;
        let mut var_zeta1__blk1070_dn7: f64 = *var_zeta1__blk1070_dn7_slot;
        let mut var_zeta1__blk1070_dn8: f64 = *var_zeta1__blk1070_dn8_slot;
        let mut var_zeta1__blk1070_dn9: f64 = *var_zeta1__blk1070_dn9_slot;
        let mut var_zeta2__blk1071: f64 = *var_zeta2__blk1071_slot;
        let mut var_zeta2__blk1071_dn4: f64 = *var_zeta2__blk1071_dn4_slot;
        let mut var_zeta2__blk1071_dn6: f64 = *var_zeta2__blk1071_dn6_slot;
        let mut var_zeta2__blk1071_dn7: f64 = *var_zeta2__blk1071_dn7_slot;
        let mut var_zeta2__blk1071_dn8: f64 = *var_zeta2__blk1071_dn8_slot;
        let mut var_zeta2__blk1071_dn9: f64 = *var_zeta2__blk1071_dn9_slot;

        let (assign41260_e47144, assign41260_e47144_d_n4, assign41260_e47144_d_n6, assign41260_e47144_d_n7, assign41260_e47144_d_n8, assign41260_e47144_d_n9,) = {
    if (((var_guard1080 != 0.0) && (var_guard1232 != 0.0)) && (var_guard1233 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_inv_k2h2_0__blk1069, var_inv_k2h2_0__blk1069_dn4, var_inv_k2h2_0__blk1069_dn6, var_inv_k2h2_0__blk1069_dn7, var_inv_k2h2_0__blk1069_dn8, var_inv_k2h2_0__blk1069_dn9,)
    }
};
        var_inv_k2h2_0__blk1069 = assign41260_e47144;
        var_inv_k2h2_0__blk1069_dn4 = assign41260_e47144_d_n4;
        var_inv_k2h2_0__blk1069_dn6 = assign41260_e47144_d_n6;
        var_inv_k2h2_0__blk1069_dn7 = assign41260_e47144_d_n7;
        var_inv_k2h2_0__blk1069_dn8 = assign41260_e47144_d_n8;
        var_inv_k2h2_0__blk1069_dn9 = assign41260_e47144_d_n9;

        let (assign41270_e47160, assign41270_e47160_d_n4, assign41270_e47160_d_n6, assign41270_e47160_d_n7, assign41270_e47160_d_n8, assign41270_e47160_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1232 == 0.0)) {
        let assign41270_e47150: f64 = (-2.0);
        let assign41270_e47152: f64 = (assign41270_e47150 * var_s1__blk969);
        let assign41270_e47155: f64 = (var_inv_k1__blk906 / var_q1chapinf__blk972);
        let assign41270_e47157: f64 = (assign41270_e47155 + var_inv_dinf__blk975);
        let assign41270_e47158: f64 = (assign41270_e47152 * assign41270_e47157);
        (assign41270_e47158, (((assign41270_e47150 * var_s1__blk969_dn4) * assign41270_e47157) + (assign41270_e47152 * ((((var_inv_k1__blk906_dn4 * var_q1chapinf__blk972) - (var_inv_k1__blk906 * var_q1chapinf__blk972_dn4)) / (var_q1chapinf__blk972 * var_q1chapinf__blk972)) + var_inv_dinf__blk975_dn4))), (((assign41270_e47150 * var_s1__blk969_dn6) * assign41270_e47157) + (assign41270_e47152 * ((((var_inv_k1__blk906_dn6 * var_q1chapinf__blk972) - (var_inv_k1__blk906 * var_q1chapinf__blk972_dn6)) / (var_q1chapinf__blk972 * var_q1chapinf__blk972)) + var_inv_dinf__blk975_dn6))), (((assign41270_e47150 * var_s1__blk969_dn7) * assign41270_e47157) + (assign41270_e47152 * ((((var_inv_k1__blk906_dn7 * var_q1chapinf__blk972) - (var_inv_k1__blk906 * var_q1chapinf__blk972_dn7)) / (var_q1chapinf__blk972 * var_q1chapinf__blk972)) + var_inv_dinf__blk975_dn7))), (((assign41270_e47150 * var_s1__blk969_dn8) * assign41270_e47157) + (assign41270_e47152 * ((((var_inv_k1__blk906_dn8 * var_q1chapinf__blk972) - (var_inv_k1__blk906 * var_q1chapinf__blk972_dn8)) / (var_q1chapinf__blk972 * var_q1chapinf__blk972)) + var_inv_dinf__blk975_dn8))), (((assign41270_e47150 * var_s1__blk969_dn9) * assign41270_e47157) + (assign41270_e47152 * ((((var_inv_k1__blk906_dn9 * var_q1chapinf__blk972) - (var_inv_k1__blk906 * var_q1chapinf__blk972_dn9)) / (var_q1chapinf__blk972 * var_q1chapinf__blk972)) + var_inv_dinf__blk975_dn9))),)
    } else {
        (var_zeta1__blk1070, var_zeta1__blk1070_dn4, var_zeta1__blk1070_dn6, var_zeta1__blk1070_dn7, var_zeta1__blk1070_dn8, var_zeta1__blk1070_dn9,)
    }
};
        var_zeta1__blk1070 = assign41270_e47160;
        var_zeta1__blk1070_dn4 = assign41270_e47160_d_n4;
        var_zeta1__blk1070_dn6 = assign41270_e47160_d_n6;
        var_zeta1__blk1070_dn7 = assign41270_e47160_d_n7;
        var_zeta1__blk1070_dn8 = assign41270_e47160_d_n8;
        var_zeta1__blk1070_dn9 = assign41270_e47160_d_n9;

        let (assign41280_e47176, assign41280_e47176_d_n4, assign41280_e47176_d_n6, assign41280_e47176_d_n7, assign41280_e47176_d_n8, assign41280_e47176_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1232 == 0.0)) {
        let assign41280_e47166: f64 = (-2.0);
        let assign41280_e47168: f64 = (assign41280_e47166 * var_s2__blk970);
        let assign41280_e47171: f64 = (var_inv_k2__blk907 / var_q2chapinf__blk973);
        let assign41280_e47173: f64 = (assign41280_e47171 + var_inv_dinf__blk975);
        let assign41280_e47174: f64 = (assign41280_e47168 * assign41280_e47173);
        (assign41280_e47174, (((assign41280_e47166 * var_s2__blk970_dn4) * assign41280_e47173) + (assign41280_e47168 * ((((var_inv_k2__blk907_dn4 * var_q2chapinf__blk973) - (var_inv_k2__blk907 * var_q2chapinf__blk973_dn4)) / (var_q2chapinf__blk973 * var_q2chapinf__blk973)) + var_inv_dinf__blk975_dn4))), (((assign41280_e47166 * var_s2__blk970_dn6) * assign41280_e47173) + (assign41280_e47168 * ((((var_inv_k2__blk907_dn6 * var_q2chapinf__blk973) - (var_inv_k2__blk907 * var_q2chapinf__blk973_dn6)) / (var_q2chapinf__blk973 * var_q2chapinf__blk973)) + var_inv_dinf__blk975_dn6))), (((assign41280_e47166 * var_s2__blk970_dn7) * assign41280_e47173) + (assign41280_e47168 * ((((var_inv_k2__blk907_dn7 * var_q2chapinf__blk973) - (var_inv_k2__blk907 * var_q2chapinf__blk973_dn7)) / (var_q2chapinf__blk973 * var_q2chapinf__blk973)) + var_inv_dinf__blk975_dn7))), (((assign41280_e47166 * var_s2__blk970_dn8) * assign41280_e47173) + (assign41280_e47168 * ((((var_inv_k2__blk907_dn8 * var_q2chapinf__blk973) - (var_inv_k2__blk907 * var_q2chapinf__blk973_dn8)) / (var_q2chapinf__blk973 * var_q2chapinf__blk973)) + var_inv_dinf__blk975_dn8))), (((assign41280_e47166 * var_s2__blk970_dn9) * assign41280_e47173) + (assign41280_e47168 * ((((var_inv_k2__blk907_dn9 * var_q2chapinf__blk973) - (var_inv_k2__blk907 * var_q2chapinf__blk973_dn9)) / (var_q2chapinf__blk973 * var_q2chapinf__blk973)) + var_inv_dinf__blk975_dn9))),)
    } else {
        (var_zeta2__blk1071, var_zeta2__blk1071_dn4, var_zeta2__blk1071_dn6, var_zeta2__blk1071_dn7, var_zeta2__blk1071_dn8, var_zeta2__blk1071_dn9,)
    }
};
        var_zeta2__blk1071 = assign41280_e47176;
        var_zeta2__blk1071_dn4 = assign41280_e47176_d_n4;
        var_zeta2__blk1071_dn6 = assign41280_e47176_d_n6;
        var_zeta2__blk1071_dn7 = assign41280_e47176_d_n7;
        var_zeta2__blk1071_dn8 = assign41280_e47176_d_n8;
        var_zeta2__blk1071_dn9 = assign41280_e47176_d_n9;

        let (assign41290_e47187, assign41290_e47187_d_n4, assign41290_e47187_d_n6, assign41290_e47187_d_n7, assign41290_e47187_d_n8, assign41290_e47187_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1232 == 0.0)) {
        let assign41290_e47183: f64 = (var_zeta2__blk1071 - var_zeta1__blk1070);
        let assign41290_e47185: f64 = (assign41290_e47183 * var_inv_dinf__blk975);
        (assign41290_e47185, (((var_zeta2__blk1071_dn4 - var_zeta1__blk1070_dn4) * var_inv_dinf__blk975) + (assign41290_e47183 * var_inv_dinf__blk975_dn4)), (((var_zeta2__blk1071_dn6 - var_zeta1__blk1070_dn6) * var_inv_dinf__blk975) + (assign41290_e47183 * var_inv_dinf__blk975_dn6)), (((var_zeta2__blk1071_dn7 - var_zeta1__blk1070_dn7) * var_inv_dinf__blk975) + (assign41290_e47183 * var_inv_dinf__blk975_dn7)), (((var_zeta2__blk1071_dn8 - var_zeta1__blk1070_dn8) * var_inv_dinf__blk975) + (assign41290_e47183 * var_inv_dinf__blk975_dn8)), (((var_zeta2__blk1071_dn9 - var_zeta1__blk1070_dn9) * var_inv_dinf__blk975) + (assign41290_e47183 * var_inv_dinf__blk975_dn9)),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign41290_e47187;
        var_temp_dn4 = assign41290_e47187_d_n4;
        var_temp_dn6 = assign41290_e47187_d_n6;
        var_temp_dn7 = assign41290_e47187_d_n7;
        var_temp_dn8 = assign41290_e47187_d_n8;
        var_temp_dn9 = assign41290_e47187_d_n9;

        let (assign41300_e47196, assign41300_e47196_d_n4, assign41300_e47196_d_n6, assign41300_e47196_d_n7, assign41300_e47196_d_n8, assign41300_e47196_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1232 == 0.0)) {
        let assign41300_e47194: f64 = (var_zeta1__blk1070 * var_inv_k1__blk906);
        (assign41300_e47194, ((var_zeta1__blk1070_dn4 * var_inv_k1__blk906) + (var_zeta1__blk1070 * var_inv_k1__blk906_dn4)), ((var_zeta1__blk1070_dn6 * var_inv_k1__blk906) + (var_zeta1__blk1070 * var_inv_k1__blk906_dn6)), ((var_zeta1__blk1070_dn7 * var_inv_k1__blk906) + (var_zeta1__blk1070 * var_inv_k1__blk906_dn7)), ((var_zeta1__blk1070_dn8 * var_inv_k1__blk906) + (var_zeta1__blk1070 * var_inv_k1__blk906_dn8)), ((var_zeta1__blk1070_dn9 * var_inv_k1__blk906) + (var_zeta1__blk1070 * var_inv_k1__blk906_dn9)),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign41300_e47196;
        var_temp1_dn4 = assign41300_e47196_d_n4;
        var_temp1_dn6 = assign41300_e47196_d_n6;
        var_temp1_dn7 = assign41300_e47196_d_n7;
        var_temp1_dn8 = assign41300_e47196_d_n8;
        var_temp1_dn9 = assign41300_e47196_d_n9;

        let (assign41310_e47205, assign41310_e47205_d_n4, assign41310_e47205_d_n6, assign41310_e47205_d_n7, assign41310_e47205_d_n8, assign41310_e47205_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1232 == 0.0)) {
        let assign41310_e47203: f64 = (var_zeta2__blk1071 * var_inv_k2__blk907);
        (assign41310_e47203, ((var_zeta2__blk1071_dn4 * var_inv_k2__blk907) + (var_zeta2__blk1071 * var_inv_k2__blk907_dn4)), ((var_zeta2__blk1071_dn6 * var_inv_k2__blk907) + (var_zeta2__blk1071 * var_inv_k2__blk907_dn6)), ((var_zeta2__blk1071_dn7 * var_inv_k2__blk907) + (var_zeta2__blk1071 * var_inv_k2__blk907_dn7)), ((var_zeta2__blk1071_dn8 * var_inv_k2__blk907) + (var_zeta2__blk1071 * var_inv_k2__blk907_dn8)), ((var_zeta2__blk1071_dn9 * var_inv_k2__blk907) + (var_zeta2__blk1071 * var_inv_k2__blk907_dn9)),)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign41310_e47205;
        var_temp2_dn4 = assign41310_e47205_d_n4;
        var_temp2_dn6 = assign41310_e47205_d_n6;
        var_temp2_dn7 = assign41310_e47205_d_n7;
        var_temp2_dn8 = assign41310_e47205_d_n8;
        var_temp2_dn9 = assign41310_e47205_d_n9;

        let (assign41320_e47214, assign41320_e47214_d_n4, assign41320_e47214_d_n6, assign41320_e47214_d_n7, assign41320_e47214_d_n8, assign41320_e47214_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1232 == 0.0)) {
        let assign41320_e47212: f64 = (var_temp1 + var_temp2);
        (assign41320_e47212, (var_temp1_dn4 + var_temp2_dn4), (var_temp1_dn6 + var_temp2_dn6), (var_temp1_dn7 + var_temp2_dn7), (var_temp1_dn8 + var_temp2_dn8), (var_temp1_dn9 + var_temp2_dn9),)
    } else {
        (var_temp3, var_temp3_dn4, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9,)
    }
};
        var_temp3 = assign41320_e47214;
        var_temp3_dn4 = assign41320_e47214_d_n4;
        var_temp3_dn6 = assign41320_e47214_d_n6;
        var_temp3_dn7 = assign41320_e47214_d_n7;
        var_temp3_dn8 = assign41320_e47214_d_n8;
        var_temp3_dn9 = assign41320_e47214_d_n9;

        let (assign41330_e47231, assign41330_e47231_d_n4, assign41330_e47231_d_n6, assign41330_e47231_d_n7, assign41330_e47231_d_n8, assign41330_e47231_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1232 == 0.0)) {
        let assign41330_e47223: f64 = (var_s1__blk969 * var_inv_k1__blk906);
        let assign41330_e47226: f64 = (var_s2__blk970 * var_inv_k2__blk907);
        let assign41330_e47227: f64 = (assign41330_e47223 + assign41330_e47226);
        let assign41330_e47228: f64 = (2.0 * assign41330_e47227);
        let assign41330_e47229: f64 = (3.0 + assign41330_e47228);
        (assign41330_e47229, (2.0 * (((var_s1__blk969_dn4 * var_inv_k1__blk906) + (var_s1__blk969 * var_inv_k1__blk906_dn4)) + ((var_s2__blk970_dn4 * var_inv_k2__blk907) + (var_s2__blk970 * var_inv_k2__blk907_dn4)))), (2.0 * (((var_s1__blk969_dn6 * var_inv_k1__blk906) + (var_s1__blk969 * var_inv_k1__blk906_dn6)) + ((var_s2__blk970_dn6 * var_inv_k2__blk907) + (var_s2__blk970 * var_inv_k2__blk907_dn6)))), (2.0 * (((var_s1__blk969_dn7 * var_inv_k1__blk906) + (var_s1__blk969 * var_inv_k1__blk906_dn7)) + ((var_s2__blk970_dn7 * var_inv_k2__blk907) + (var_s2__blk970 * var_inv_k2__blk907_dn7)))), (2.0 * (((var_s1__blk969_dn8 * var_inv_k1__blk906) + (var_s1__blk969 * var_inv_k1__blk906_dn8)) + ((var_s2__blk970_dn8 * var_inv_k2__blk907) + (var_s2__blk970 * var_inv_k2__blk907_dn8)))), (2.0 * (((var_s1__blk969_dn9 * var_inv_k1__blk906) + (var_s1__blk969 * var_inv_k1__blk906_dn9)) + ((var_s2__blk970_dn9 * var_inv_k2__blk907) + (var_s2__blk970 * var_inv_k2__blk907_dn9)))),)
    } else {
        (var_temp4, var_temp4_dn4, var_temp4_dn6, var_temp4_dn7, var_temp4_dn8, var_temp4_dn9,)
    }
};
        var_temp4 = assign41330_e47231;
        var_temp4_dn4 = assign41330_e47231_d_n4;
        var_temp4_dn6 = assign41330_e47231_d_n6;
        var_temp4_dn7 = assign41330_e47231_d_n7;
        var_temp4_dn8 = assign41330_e47231_d_n8;
        var_temp4_dn9 = assign41330_e47231_d_n9;

        let (assign41340_e47246, assign41340_e47246_d_n4, assign41340_e47246_d_n6, assign41340_e47246_d_n7, assign41340_e47246_d_n8, assign41340_e47246_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1232 == 0.0)) {
        let assign41340_e47238: f64 = (var_temp2 + var_temp);
        let assign41340_e47241: f64 = (var_temp3 / var_q1chapinf__blk972);
        let assign41340_e47242: f64 = (assign41340_e47238 - assign41340_e47241);
        let assign41340_e47244: f64 = (assign41340_e47242 / var_temp4);
        (assign41340_e47244, (((((var_temp2_dn4 + var_temp_dn4) - (((var_temp3_dn4 * var_q1chapinf__blk972) - (var_temp3 * var_q1chapinf__blk972_dn4)) / (var_q1chapinf__blk972 * var_q1chapinf__blk972))) * var_temp4) - (assign41340_e47242 * var_temp4_dn4)) / (var_temp4 * var_temp4)), (((((var_temp2_dn6 + var_temp_dn6) - (((var_temp3_dn6 * var_q1chapinf__blk972) - (var_temp3 * var_q1chapinf__blk972_dn6)) / (var_q1chapinf__blk972 * var_q1chapinf__blk972))) * var_temp4) - (assign41340_e47242 * var_temp4_dn6)) / (var_temp4 * var_temp4)), (((((var_temp2_dn7 + var_temp_dn7) - (((var_temp3_dn7 * var_q1chapinf__blk972) - (var_temp3 * var_q1chapinf__blk972_dn7)) / (var_q1chapinf__blk972 * var_q1chapinf__blk972))) * var_temp4) - (assign41340_e47242 * var_temp4_dn7)) / (var_temp4 * var_temp4)), (((((var_temp2_dn8 + var_temp_dn8) - (((var_temp3_dn8 * var_q1chapinf__blk972) - (var_temp3 * var_q1chapinf__blk972_dn8)) / (var_q1chapinf__blk972 * var_q1chapinf__blk972))) * var_temp4) - (assign41340_e47242 * var_temp4_dn8)) / (var_temp4 * var_temp4)), (((((var_temp2_dn9 + var_temp_dn9) - (((var_temp3_dn9 * var_q1chapinf__blk972) - (var_temp3 * var_q1chapinf__blk972_dn9)) / (var_q1chapinf__blk972 * var_q1chapinf__blk972))) * var_temp4) - (assign41340_e47242 * var_temp4_dn9)) / (var_temp4 * var_temp4)),)
    } else {
        (var_ksi1__blk1072, var_ksi1__blk1072_dn4, var_ksi1__blk1072_dn6, var_ksi1__blk1072_dn7, var_ksi1__blk1072_dn8, var_ksi1__blk1072_dn9,)
    }
};
        var_ksi1__blk1072 = assign41340_e47246;
        var_ksi1__blk1072_dn4 = assign41340_e47246_d_n4;
        var_ksi1__blk1072_dn6 = assign41340_e47246_d_n6;
        var_ksi1__blk1072_dn7 = assign41340_e47246_d_n7;
        var_ksi1__blk1072_dn8 = assign41340_e47246_d_n8;
        var_ksi1__blk1072_dn9 = assign41340_e47246_d_n9;

        let (assign41350_e47261, assign41350_e47261_d_n4, assign41350_e47261_d_n6, assign41350_e47261_d_n7, assign41350_e47261_d_n8, assign41350_e47261_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1232 == 0.0)) {
        let assign41350_e47253: f64 = (var_temp1 - var_temp);
        let assign41350_e47256: f64 = (var_temp3 / var_q2chapinf__blk973);
        let assign41350_e47257: f64 = (assign41350_e47253 - assign41350_e47256);
        let assign41350_e47259: f64 = (assign41350_e47257 / var_temp4);
        (assign41350_e47259, (((((var_temp1_dn4 - var_temp_dn4) - (((var_temp3_dn4 * var_q2chapinf__blk973) - (var_temp3 * var_q2chapinf__blk973_dn4)) / (var_q2chapinf__blk973 * var_q2chapinf__blk973))) * var_temp4) - (assign41350_e47257 * var_temp4_dn4)) / (var_temp4 * var_temp4)), (((((var_temp1_dn6 - var_temp_dn6) - (((var_temp3_dn6 * var_q2chapinf__blk973) - (var_temp3 * var_q2chapinf__blk973_dn6)) / (var_q2chapinf__blk973 * var_q2chapinf__blk973))) * var_temp4) - (assign41350_e47257 * var_temp4_dn6)) / (var_temp4 * var_temp4)), (((((var_temp1_dn7 - var_temp_dn7) - (((var_temp3_dn7 * var_q2chapinf__blk973) - (var_temp3 * var_q2chapinf__blk973_dn7)) / (var_q2chapinf__blk973 * var_q2chapinf__blk973))) * var_temp4) - (assign41350_e47257 * var_temp4_dn7)) / (var_temp4 * var_temp4)), (((((var_temp1_dn8 - var_temp_dn8) - (((var_temp3_dn8 * var_q2chapinf__blk973) - (var_temp3 * var_q2chapinf__blk973_dn8)) / (var_q2chapinf__blk973 * var_q2chapinf__blk973))) * var_temp4) - (assign41350_e47257 * var_temp4_dn8)) / (var_temp4 * var_temp4)), (((((var_temp1_dn9 - var_temp_dn9) - (((var_temp3_dn9 * var_q2chapinf__blk973) - (var_temp3 * var_q2chapinf__blk973_dn9)) / (var_q2chapinf__blk973 * var_q2chapinf__blk973))) * var_temp4) - (assign41350_e47257 * var_temp4_dn9)) / (var_temp4 * var_temp4)),)
    } else {
        (var_ksi2__blk1073, var_ksi2__blk1073_dn4, var_ksi2__blk1073_dn6, var_ksi2__blk1073_dn7, var_ksi2__blk1073_dn8, var_ksi2__blk1073_dn9,)
    }
};
        var_ksi2__blk1073 = assign41350_e47261;
        var_ksi2__blk1073_dn4 = assign41350_e47261_d_n4;
        var_ksi2__blk1073_dn6 = assign41350_e47261_d_n6;
        var_ksi2__blk1073_dn7 = assign41350_e47261_d_n7;
        var_ksi2__blk1073_dn8 = assign41350_e47261_d_n8;
        var_ksi2__blk1073_dn9 = assign41350_e47261_d_n9;

        let (assign41360_e47275, assign41360_e47275_d_n4, assign41360_e47275_d_n6, assign41360_e47275_d_n7, assign41360_e47275_d_n8, assign41360_e47275_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1232 == 0.0)) {
        let assign41360_e47267: f64 = (-var_q1chapinf__blk972);
        let assign41360_e47270: f64 = (var_ksi1__blk1072 * var_q1chapinf__blk972);
        let assign41360_e47272: f64 = (assign41360_e47270 + var_inv_dinf__blk975);
        let assign41360_e47273: f64 = (assign41360_e47267 * assign41360_e47272);
        (assign41360_e47273, (((-var_q1chapinf__blk972_dn4) * assign41360_e47272) + (assign41360_e47267 * (((var_ksi1__blk1072_dn4 * var_q1chapinf__blk972) + (var_ksi1__blk1072 * var_q1chapinf__blk972_dn4)) + var_inv_dinf__blk975_dn4))), (((-var_q1chapinf__blk972_dn6) * assign41360_e47272) + (assign41360_e47267 * (((var_ksi1__blk1072_dn6 * var_q1chapinf__blk972) + (var_ksi1__blk1072 * var_q1chapinf__blk972_dn6)) + var_inv_dinf__blk975_dn6))), (((-var_q1chapinf__blk972_dn7) * assign41360_e47272) + (assign41360_e47267 * (((var_ksi1__blk1072_dn7 * var_q1chapinf__blk972) + (var_ksi1__blk1072 * var_q1chapinf__blk972_dn7)) + var_inv_dinf__blk975_dn7))), (((-var_q1chapinf__blk972_dn8) * assign41360_e47272) + (assign41360_e47267 * (((var_ksi1__blk1072_dn8 * var_q1chapinf__blk972) + (var_ksi1__blk1072 * var_q1chapinf__blk972_dn8)) + var_inv_dinf__blk975_dn8))), (((-var_q1chapinf__blk972_dn9) * assign41360_e47272) + (assign41360_e47267 * (((var_ksi1__blk1072_dn9 * var_q1chapinf__blk972) + (var_ksi1__blk1072 * var_q1chapinf__blk972_dn9)) + var_inv_dinf__blk975_dn9))),)
    } else {
        (var_inv_k1h1_0__blk1066, var_inv_k1h1_0__blk1066_dn4, var_inv_k1h1_0__blk1066_dn6, var_inv_k1h1_0__blk1066_dn7, var_inv_k1h1_0__blk1066_dn8, var_inv_k1h1_0__blk1066_dn9,)
    }
};
        var_inv_k1h1_0__blk1066 = assign41360_e47275;
        var_inv_k1h1_0__blk1066_dn4 = assign41360_e47275_d_n4;
        var_inv_k1h1_0__blk1066_dn6 = assign41360_e47275_d_n6;
        var_inv_k1h1_0__blk1066_dn7 = assign41360_e47275_d_n7;
        var_inv_k1h1_0__blk1066_dn8 = assign41360_e47275_d_n8;
        var_inv_k1h1_0__blk1066_dn9 = assign41360_e47275_d_n9;

        let (assign41370_e47289, assign41370_e47289_d_n4, assign41370_e47289_d_n6, assign41370_e47289_d_n7, assign41370_e47289_d_n8, assign41370_e47289_d_n9,) = {
    if ((var_guard1080 != 0.0) && (var_guard1232 == 0.0)) {
        let assign41370_e47281: f64 = (-var_q2chapinf__blk973);
        let assign41370_e47284: f64 = (var_ksi2__blk1073 * var_q2chapinf__blk973);
        let assign41370_e47286: f64 = (assign41370_e47284 + var_inv_dinf__blk975);
        let assign41370_e47287: f64 = (assign41370_e47281 * assign41370_e47286);
        (assign41370_e47287, (((-var_q2chapinf__blk973_dn4) * assign41370_e47286) + (assign41370_e47281 * (((var_ksi2__blk1073_dn4 * var_q2chapinf__blk973) + (var_ksi2__blk1073 * var_q2chapinf__blk973_dn4)) + var_inv_dinf__blk975_dn4))), (((-var_q2chapinf__blk973_dn6) * assign41370_e47286) + (assign41370_e47281 * (((var_ksi2__blk1073_dn6 * var_q2chapinf__blk973) + (var_ksi2__blk1073 * var_q2chapinf__blk973_dn6)) + var_inv_dinf__blk975_dn6))), (((-var_q2chapinf__blk973_dn7) * assign41370_e47286) + (assign41370_e47281 * (((var_ksi2__blk1073_dn7 * var_q2chapinf__blk973) + (var_ksi2__blk1073 * var_q2chapinf__blk973_dn7)) + var_inv_dinf__blk975_dn7))), (((-var_q2chapinf__blk973_dn8) * assign41370_e47286) + (assign41370_e47281 * (((var_ksi2__blk1073_dn8 * var_q2chapinf__blk973) + (var_ksi2__blk1073 * var_q2chapinf__blk973_dn8)) + var_inv_dinf__blk975_dn8))), (((-var_q2chapinf__blk973_dn9) * assign41370_e47286) + (assign41370_e47281 * (((var_ksi2__blk1073_dn9 * var_q2chapinf__blk973) + (var_ksi2__blk1073 * var_q2chapinf__blk973_dn9)) + var_inv_dinf__blk975_dn9))),)
    } else {
        (var_inv_k2h2_0__blk1069, var_inv_k2h2_0__blk1069_dn4, var_inv_k2h2_0__blk1069_dn6, var_inv_k2h2_0__blk1069_dn7, var_inv_k2h2_0__blk1069_dn8, var_inv_k2h2_0__blk1069_dn9,)
    }
};
        var_inv_k2h2_0__blk1069 = assign41370_e47289;
        var_inv_k2h2_0__blk1069_dn4 = assign41370_e47289_d_n4;
        var_inv_k2h2_0__blk1069_dn6 = assign41370_e47289_d_n6;
        var_inv_k2h2_0__blk1069_dn7 = assign41370_e47289_d_n7;
        var_inv_k2h2_0__blk1069_dn8 = assign41370_e47289_d_n8;
        var_inv_k2h2_0__blk1069_dn9 = assign41370_e47289_d_n9;

        let (assign41380_e47295, assign41380_e47295_d_n4, assign41380_e47295_d_n6, assign41380_e47295_d_n7, assign41380_e47295_d_n8, assign41380_e47295_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign41380_e47293: f64 = (var_inv_k1h1_0__blk1066 * var_hsat__blk1053);
        (assign41380_e47293, ((var_inv_k1h1_0__blk1066_dn4 * var_hsat__blk1053) + (var_inv_k1h1_0__blk1066 * var_hsat__blk1053_dn4)), ((var_inv_k1h1_0__blk1066_dn6 * var_hsat__blk1053) + (var_inv_k1h1_0__blk1066 * var_hsat__blk1053_dn6)), ((var_inv_k1h1_0__blk1066_dn7 * var_hsat__blk1053) + (var_inv_k1h1_0__blk1066 * var_hsat__blk1053_dn7)), ((var_inv_k1h1_0__blk1066_dn8 * var_hsat__blk1053) + (var_inv_k1h1_0__blk1066 * var_hsat__blk1053_dn8)), ((var_inv_k1h1_0__blk1066_dn9 * var_hsat__blk1053) + (var_inv_k1h1_0__blk1066 * var_hsat__blk1053_dn9)),)
    } else {
        (var_inv_k1h1__blk1074, var_inv_k1h1__blk1074_dn4, var_inv_k1h1__blk1074_dn6, var_inv_k1h1__blk1074_dn7, var_inv_k1h1__blk1074_dn8, var_inv_k1h1__blk1074_dn9,)
    }
};
        var_inv_k1h1__blk1074 = assign41380_e47295;
        var_inv_k1h1__blk1074_dn4 = assign41380_e47295_d_n4;
        var_inv_k1h1__blk1074_dn6 = assign41380_e47295_d_n6;
        var_inv_k1h1__blk1074_dn7 = assign41380_e47295_d_n7;
        var_inv_k1h1__blk1074_dn8 = assign41380_e47295_d_n8;
        var_inv_k1h1__blk1074_dn9 = assign41380_e47295_d_n9;

        let (assign41390_e47301, assign41390_e47301_d_n4, assign41390_e47301_d_n6, assign41390_e47301_d_n7, assign41390_e47301_d_n8, assign41390_e47301_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign41390_e47299: f64 = (var_inv_k2h2_0__blk1069 * var_hsat__blk1053);
        (assign41390_e47299, ((var_inv_k2h2_0__blk1069_dn4 * var_hsat__blk1053) + (var_inv_k2h2_0__blk1069 * var_hsat__blk1053_dn4)), ((var_inv_k2h2_0__blk1069_dn6 * var_hsat__blk1053) + (var_inv_k2h2_0__blk1069 * var_hsat__blk1053_dn6)), ((var_inv_k2h2_0__blk1069_dn7 * var_hsat__blk1053) + (var_inv_k2h2_0__blk1069 * var_hsat__blk1053_dn7)), ((var_inv_k2h2_0__blk1069_dn8 * var_hsat__blk1053) + (var_inv_k2h2_0__blk1069 * var_hsat__blk1053_dn8)), ((var_inv_k2h2_0__blk1069_dn9 * var_hsat__blk1053) + (var_inv_k2h2_0__blk1069 * var_hsat__blk1053_dn9)),)
    } else {
        (var_inv_k2h2__blk1075, var_inv_k2h2__blk1075_dn4, var_inv_k2h2__blk1075_dn6, var_inv_k2h2__blk1075_dn7, var_inv_k2h2__blk1075_dn8, var_inv_k2h2__blk1075_dn9,)
    }
};
        var_inv_k2h2__blk1075 = assign41390_e47301;
        var_inv_k2h2__blk1075_dn4 = assign41390_e47301_d_n4;
        var_inv_k2h2__blk1075_dn6 = assign41390_e47301_d_n6;
        var_inv_k2h2__blk1075_dn7 = assign41390_e47301_d_n7;
        var_inv_k2h2__blk1075_dn8 = assign41390_e47301_d_n8;
        var_inv_k2h2__blk1075_dn9 = assign41390_e47301_d_n9;

        let (assign41400_e47309, assign41400_e47309_d_n4, assign41400_e47309_d_n6, assign41400_e47309_d_n7, assign41400_e47309_d_n8, assign41400_e47309_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign41400_e47306: f64 = (var_k1q1d__blk1004 - var_k1q1s__blk939);
        let assign41400_e47307: f64 = (0.5 * assign41400_e47306);
        (assign41400_e47307, (0.5 * (var_k1q1d__blk1004_dn4 - var_k1q1s__blk939_dn4)), (0.5 * (var_k1q1d__blk1004_dn6 - var_k1q1s__blk939_dn6)), (0.5 * (var_k1q1d__blk1004_dn7 - var_k1q1s__blk939_dn7)), (0.5 * (var_k1q1d__blk1004_dn8 - var_k1q1s__blk939_dn8)), (0.5 * (var_k1q1d__blk1004_dn9 - var_k1q1s__blk939_dn9)),)
    } else {
        (var_delta_k1q1__blk1076, var_delta_k1q1__blk1076_dn4, var_delta_k1q1__blk1076_dn6, var_delta_k1q1__blk1076_dn7, var_delta_k1q1__blk1076_dn8, var_delta_k1q1__blk1076_dn9,)
    }
};
        var_delta_k1q1__blk1076 = assign41400_e47309;
        var_delta_k1q1__blk1076_dn4 = assign41400_e47309_d_n4;
        var_delta_k1q1__blk1076_dn6 = assign41400_e47309_d_n6;
        var_delta_k1q1__blk1076_dn7 = assign41400_e47309_d_n7;
        var_delta_k1q1__blk1076_dn8 = assign41400_e47309_d_n8;
        var_delta_k1q1__blk1076_dn9 = assign41400_e47309_d_n9;

        let (assign41410_e47317, assign41410_e47317_d_n4, assign41410_e47317_d_n6, assign41410_e47317_d_n7, assign41410_e47317_d_n8, assign41410_e47317_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign41410_e47314: f64 = (var_k2q2d__blk1005 - var_k2q2s__blk940);
        let assign41410_e47315: f64 = (0.5 * assign41410_e47314);
        (assign41410_e47315, (0.5 * (var_k2q2d__blk1005_dn4 - var_k2q2s__blk940_dn4)), (0.5 * (var_k2q2d__blk1005_dn6 - var_k2q2s__blk940_dn6)), (0.5 * (var_k2q2d__blk1005_dn7 - var_k2q2s__blk940_dn7)), (0.5 * (var_k2q2d__blk1005_dn8 - var_k2q2s__blk940_dn8)), (0.5 * (var_k2q2d__blk1005_dn9 - var_k2q2s__blk940_dn9)),)
    } else {
        (var_delta_k2q2__blk1077, var_delta_k2q2__blk1077_dn4, var_delta_k2q2__blk1077_dn6, var_delta_k2q2__blk1077_dn7, var_delta_k2q2__blk1077_dn8, var_delta_k2q2__blk1077_dn9,)
    }
};
        var_delta_k2q2__blk1077 = assign41410_e47317;
        var_delta_k2q2__blk1077_dn4 = assign41410_e47317_d_n4;
        var_delta_k2q2__blk1077_dn6 = assign41410_e47317_d_n6;
        var_delta_k2q2__blk1077_dn7 = assign41410_e47317_d_n7;
        var_delta_k2q2__blk1077_dn8 = assign41410_e47317_d_n8;
        var_delta_k2q2__blk1077_dn9 = assign41410_e47317_d_n9;

        let (assign41420_e47323, assign41420_e47323_d_n4, assign41420_e47323_d_n6, assign41420_e47323_d_n7, assign41420_e47323_d_n8, assign41420_e47323_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign41420_e47321: f64 = (var_delta_k1q1__blk1076 * var_inv_k1h1__blk1074);
        (assign41420_e47321, ((var_delta_k1q1__blk1076_dn4 * var_inv_k1h1__blk1074) + (var_delta_k1q1__blk1076 * var_inv_k1h1__blk1074_dn4)), ((var_delta_k1q1__blk1076_dn6 * var_inv_k1h1__blk1074) + (var_delta_k1q1__blk1076 * var_inv_k1h1__blk1074_dn6)), ((var_delta_k1q1__blk1076_dn7 * var_inv_k1h1__blk1074) + (var_delta_k1q1__blk1076 * var_inv_k1h1__blk1074_dn7)), ((var_delta_k1q1__blk1076_dn8 * var_inv_k1h1__blk1074) + (var_delta_k1q1__blk1076 * var_inv_k1h1__blk1074_dn8)), ((var_delta_k1q1__blk1076_dn9 * var_inv_k1h1__blk1074) + (var_delta_k1q1__blk1076 * var_inv_k1h1__blk1074_dn9)),)
    } else {
        (var_prod1__blk1078, var_prod1__blk1078_dn4, var_prod1__blk1078_dn6, var_prod1__blk1078_dn7, var_prod1__blk1078_dn8, var_prod1__blk1078_dn9,)
    }
};
        var_prod1__blk1078 = assign41420_e47323;
        var_prod1__blk1078_dn4 = assign41420_e47323_d_n4;
        var_prod1__blk1078_dn6 = assign41420_e47323_d_n6;
        var_prod1__blk1078_dn7 = assign41420_e47323_d_n7;
        var_prod1__blk1078_dn8 = assign41420_e47323_d_n8;
        var_prod1__blk1078_dn9 = assign41420_e47323_d_n9;

        let (assign41430_e47329, assign41430_e47329_d_n4, assign41430_e47329_d_n6, assign41430_e47329_d_n7, assign41430_e47329_d_n8, assign41430_e47329_d_n9,) = {
    if (var_guard1080 != 0.0) {
        let assign41430_e47327: f64 = (var_delta_k2q2__blk1077 * var_inv_k2h2__blk1075);
        (assign41430_e47327, ((var_delta_k2q2__blk1077_dn4 * var_inv_k2h2__blk1075) + (var_delta_k2q2__blk1077 * var_inv_k2h2__blk1075_dn4)), ((var_delta_k2q2__blk1077_dn6 * var_inv_k2h2__blk1075) + (var_delta_k2q2__blk1077 * var_inv_k2h2__blk1075_dn6)), ((var_delta_k2q2__blk1077_dn7 * var_inv_k2h2__blk1075) + (var_delta_k2q2__blk1077 * var_inv_k2h2__blk1075_dn7)), ((var_delta_k2q2__blk1077_dn8 * var_inv_k2h2__blk1075) + (var_delta_k2q2__blk1077 * var_inv_k2h2__blk1075_dn8)), ((var_delta_k2q2__blk1077_dn9 * var_inv_k2h2__blk1075) + (var_delta_k2q2__blk1077 * var_inv_k2h2__blk1075_dn9)),)
    } else {
        (var_prod2__blk1079, var_prod2__blk1079_dn4, var_prod2__blk1079_dn6, var_prod2__blk1079_dn7, var_prod2__blk1079_dn8, var_prod2__blk1079_dn9,)
    }
};
        var_prod2__blk1079 = assign41430_e47329;
        var_prod2__blk1079_dn4 = assign41430_e47329_d_n4;
        var_prod2__blk1079_dn6 = assign41430_e47329_d_n6;
        var_prod2__blk1079_dn7 = assign41430_e47329_d_n7;
        var_prod2__blk1079_dn8 = assign41430_e47329_d_n8;
        var_prod2__blk1079_dn9 = assign41430_e47329_d_n9;

        let (assign41440_e47333, assign41440_e47333_d_n4, assign41440_e47333_d_n6, assign41440_e47333_d_n7, assign41440_e47333_d_n8, assign41440_e47333_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_xg20shift__blk900, var_xg20shift__blk900_dn4, var_xg20shift__blk900_dn6, var_xg20shift__blk900_dn7, var_xg20shift__blk900_dn8, var_xg20shift__blk900_dn9,)
    } else {
        (var_xg20shift_ac, var_xg20shift_ac_dn4, var_xg20shift_ac_dn6, var_xg20shift_ac_dn7, var_xg20shift_ac_dn8, var_xg20shift_ac_dn9,)
    }
};
        var_xg20shift_ac = assign41440_e47333;
        var_xg20shift_ac_dn4 = assign41440_e47333_d_n4;
        var_xg20shift_ac_dn6 = assign41440_e47333_d_n6;
        var_xg20shift_ac_dn7 = assign41440_e47333_d_n7;
        var_xg20shift_ac_dn8 = assign41440_e47333_d_n8;
        var_xg20shift_ac_dn9 = assign41440_e47333_d_n9;

        let (assign41450_e47337, assign41450_e47337_d_n4, assign41450_e47337_d_n6, assign41450_e47337_d_n7, assign41450_e47337_d_n8, assign41450_e47337_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_diff_min__blk904, var_diff_min__blk904_dn4, var_diff_min__blk904_dn6, var_diff_min__blk904_dn7, var_diff_min__blk904_dn8, var_diff_min__blk904_dn9,)
    } else {
        (var_diff_min_ac, var_diff_min_ac_dn4, var_diff_min_ac_dn6, var_diff_min_ac_dn7, var_diff_min_ac_dn8, var_diff_min_ac_dn9,)
    }
};
        var_diff_min_ac = assign41450_e47337;
        var_diff_min_ac_dn4 = assign41450_e47337_d_n4;
        var_diff_min_ac_dn6 = assign41450_e47337_d_n6;
        var_diff_min_ac_dn7 = assign41450_e47337_d_n7;
        var_diff_min_ac_dn8 = assign41450_e47337_d_n8;
        var_diff_min_ac_dn9 = assign41450_e47337_d_n9;

        let (assign41460_e47341, assign41460_e47341_d_n4, assign41460_e47341_d_n6, assign41460_e47341_d_n7, assign41460_e47341_d_n8, assign41460_e47341_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_a0__blk905, var_a0__blk905_dn4, var_a0__blk905_dn6, var_a0__blk905_dn7, var_a0__blk905_dn8, var_a0__blk905_dn9,)
    } else {
        (var_a0_ac, var_a0_ac_dn4, var_a0_ac_dn6, var_a0_ac_dn7, var_a0_ac_dn8, var_a0_ac_dn9,)
    }
};
        var_a0_ac = assign41460_e47341;
        var_a0_ac_dn4 = assign41460_e47341_d_n4;
        var_a0_ac_dn6 = assign41460_e47341_d_n6;
        var_a0_ac_dn7 = assign41460_e47341_d_n7;
        var_a0_ac_dn8 = assign41460_e47341_d_n8;
        var_a0_ac_dn9 = assign41460_e47341_d_n9;

        let (assign41470_e47345, assign41470_e47345_d_n4, assign41470_e47345_d_n6, assign41470_e47345_d_n7, assign41470_e47345_d_n8, assign41470_e47345_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_inv_k1__blk906, var_inv_k1__blk906_dn4, var_inv_k1__blk906_dn6, var_inv_k1__blk906_dn7, var_inv_k1__blk906_dn8, var_inv_k1__blk906_dn9,)
    } else {
        (var_inv_k1_ac, var_inv_k1_ac_dn4, var_inv_k1_ac_dn6, var_inv_k1_ac_dn7, var_inv_k1_ac_dn8, var_inv_k1_ac_dn9,)
    }
};
        var_inv_k1_ac = assign41470_e47345;
        var_inv_k1_ac_dn4 = assign41470_e47345_d_n4;
        var_inv_k1_ac_dn6 = assign41470_e47345_d_n6;
        var_inv_k1_ac_dn7 = assign41470_e47345_d_n7;
        var_inv_k1_ac_dn8 = assign41470_e47345_d_n8;
        var_inv_k1_ac_dn9 = assign41470_e47345_d_n9;

        let (assign41480_e47349, assign41480_e47349_d_n4, assign41480_e47349_d_n6, assign41480_e47349_d_n7, assign41480_e47349_d_n8, assign41480_e47349_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_inv_k2__blk907, var_inv_k2__blk907_dn4, var_inv_k2__blk907_dn6, var_inv_k2__blk907_dn7, var_inv_k2__blk907_dn8, var_inv_k2__blk907_dn9,)
    } else {
        (var_inv_k2_ac, var_inv_k2_ac_dn4, var_inv_k2_ac_dn6, var_inv_k2_ac_dn7, var_inv_k2_ac_dn8, var_inv_k2_ac_dn9,)
    }
};
        var_inv_k2_ac = assign41480_e47349;
        var_inv_k2_ac_dn4 = assign41480_e47349_d_n4;
        var_inv_k2_ac_dn6 = assign41480_e47349_d_n6;
        var_inv_k2_ac_dn7 = assign41480_e47349_d_n7;
        var_inv_k2_ac_dn8 = assign41480_e47349_d_n8;
        var_inv_k2_ac_dn9 = assign41480_e47349_d_n9;

        let (assign41490_e47353, assign41490_e47353_d_n4, assign41490_e47353_d_n6, assign41490_e47353_d_n7, assign41490_e47353_d_n8, assign41490_e47353_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_keq__blk934, var_keq__blk934_dn4, var_keq__blk934_dn6, var_keq__blk934_dn7, var_keq__blk934_dn8, var_keq__blk934_dn9,)
    } else {
        (var_keq_ac, var_keq_ac_dn4, var_keq_ac_dn6, var_keq_ac_dn7, var_keq_ac_dn8, var_keq_ac_dn9,)
    }
};
        var_keq_ac = assign41490_e47353;
        var_keq_ac_dn4 = assign41490_e47353_d_n4;
        var_keq_ac_dn6 = assign41490_e47353_d_n6;
        var_keq_ac_dn7 = assign41490_e47353_d_n7;
        var_keq_ac_dn8 = assign41490_e47353_d_n8;
        var_keq_ac_dn9 = assign41490_e47353_d_n9;

        let (assign41500_e47357, assign41500_e47357_d_n4, assign41500_e47357_d_n6, assign41500_e47357_d_n7, assign41500_e47357_d_n8, assign41500_e47357_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_dx_wi__blk935, var_dx_wi__blk935_dn4, var_dx_wi__blk935_dn6, var_dx_wi__blk935_dn7, var_dx_wi__blk935_dn8, var_dx_wi__blk935_dn9,)
    } else {
        (var_dx_wi_ac, var_dx_wi_ac_dn4, var_dx_wi_ac_dn6, var_dx_wi_ac_dn7, var_dx_wi_ac_dn8, var_dx_wi_ac_dn9,)
    }
};
        var_dx_wi_ac = assign41500_e47357;
        var_dx_wi_ac_dn4 = assign41500_e47357_d_n4;
        var_dx_wi_ac_dn6 = assign41500_e47357_d_n6;
        var_dx_wi_ac_dn7 = assign41500_e47357_d_n7;
        var_dx_wi_ac_dn8 = assign41500_e47357_d_n8;
        var_dx_wi_ac_dn9 = assign41500_e47357_d_n9;

        let (assign41510_e47361, assign41510_e47361_d_n4, assign41510_e47361_d_n6, assign41510_e47361_d_n7, assign41510_e47361_d_n8, assign41510_e47361_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_csiprime__blk919, var_csiprime__blk919_dn4, var_csiprime__blk919_dn6, var_csiprime__blk919_dn7, var_csiprime__blk919_dn8, var_csiprime__blk919_dn9,)
    } else {
        (var_csiprime_ac, var_csiprime_ac_dn4, var_csiprime_ac_dn6, var_csiprime_ac_dn7, var_csiprime_ac_dn8, var_csiprime_ac_dn9,)
    }
};
        var_csiprime_ac = assign41510_e47361;
        var_csiprime_ac_dn4 = assign41510_e47361_d_n4;
        var_csiprime_ac_dn6 = assign41510_e47361_d_n6;
        var_csiprime_ac_dn7 = assign41510_e47361_d_n7;
        var_csiprime_ac_dn8 = assign41510_e47361_d_n8;
        var_csiprime_ac_dn9 = assign41510_e47361_d_n9;

        let (assign41520_e47365, assign41520_e47365_d_n4, assign41520_e47365_d_n6, assign41520_e47365_d_n7, assign41520_e47365_d_n8, assign41520_e47365_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_dx_wi_1d__blk918, var_dx_wi_1d__blk918_dn4, var_dx_wi_1d__blk918_dn6, var_dx_wi_1d__blk918_dn7, var_dx_wi_1d__blk918_dn8, var_dx_wi_1d__blk918_dn9,)
    } else {
        (var_dx_wi_1d_ac, var_dx_wi_1d_ac_dn4, var_dx_wi_1d_ac_dn6, var_dx_wi_1d_ac_dn7, var_dx_wi_1d_ac_dn8, var_dx_wi_1d_ac_dn9,)
    }
};
        var_dx_wi_1d_ac = assign41520_e47365;
        var_dx_wi_1d_ac_dn4 = assign41520_e47365_d_n4;
        var_dx_wi_1d_ac_dn6 = assign41520_e47365_d_n6;
        var_dx_wi_1d_ac_dn7 = assign41520_e47365_d_n7;
        var_dx_wi_1d_ac_dn8 = assign41520_e47365_d_n8;
        var_dx_wi_1d_ac_dn9 = assign41520_e47365_d_n9;

        let (assign41530_e47369, assign41530_e47369_d_n4, assign41530_e47369_d_n6, assign41530_e47369_d_n7, assign41530_e47369_d_n8, assign41530_e47369_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_dleff__blk922, var_dleff__blk922_dn4, var_dleff__blk922_dn6, var_dleff__blk922_dn7, var_dleff__blk922_dn8, var_dleff__blk922_dn9,)
    } else {
        (var_dleff_ac, var_dleff_ac_dn4, var_dleff_ac_dn6, var_dleff_ac_dn7, var_dleff_ac_dn8, var_dleff_ac_dn9,)
    }
};
        var_dleff_ac = assign41530_e47369;
        var_dleff_ac_dn4 = assign41530_e47369_d_n4;
        var_dleff_ac_dn6 = assign41530_e47369_d_n6;
        var_dleff_ac_dn7 = assign41530_e47369_d_n7;
        var_dleff_ac_dn8 = assign41530_e47369_d_n8;
        var_dleff_ac_dn9 = assign41530_e47369_d_n9;

        let (assign41540_e47373, assign41540_e47373_d_n4, assign41540_e47373_d_n6, assign41540_e47373_d_n7, assign41540_e47373_d_n8, assign41540_e47373_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_xedge__blk923, var_xedge__blk923_dn4, var_xedge__blk923_dn6, var_xedge__blk923_dn7, var_xedge__blk923_dn8, var_xedge__blk923_dn9,)
    } else {
        (var_xedge_ac, var_xedge_ac_dn4, var_xedge_ac_dn6, var_xedge_ac_dn7, var_xedge_ac_dn8, var_xedge_ac_dn9,)
    }
};
        var_xedge_ac = assign41540_e47373;
        var_xedge_ac_dn4 = assign41540_e47373_d_n4;
        var_xedge_ac_dn6 = assign41540_e47373_d_n6;
        var_xedge_ac_dn7 = assign41540_e47373_d_n7;
        var_xedge_ac_dn8 = assign41540_e47373_d_n8;
        var_xedge_ac_dn9 = assign41540_e47373_d_n9;

        let (assign41550_e47377, assign41550_e47377_d_n4, assign41550_e47377_d_n6, assign41550_e47377_d_n7, assign41550_e47377_d_n8, assign41550_e47377_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_sce1__blk924, var_sce1__blk924_dn4, var_sce1__blk924_dn6, var_sce1__blk924_dn7, var_sce1__blk924_dn8, var_sce1__blk924_dn9,)
    } else {
        (var_sce1_ac, var_sce1_ac_dn4, var_sce1_ac_dn6, var_sce1_ac_dn7, var_sce1_ac_dn8, var_sce1_ac_dn9,)
    }
};
        var_sce1_ac = assign41550_e47377;
        var_sce1_ac_dn4 = assign41550_e47377_d_n4;
        var_sce1_ac_dn6 = assign41550_e47377_d_n6;
        var_sce1_ac_dn7 = assign41550_e47377_d_n7;
        var_sce1_ac_dn8 = assign41550_e47377_d_n8;
        var_sce1_ac_dn9 = assign41550_e47377_d_n9;

        let (assign41560_e47381, assign41560_e47381_d_n4, assign41560_e47381_d_n6, assign41560_e47381_d_n7, assign41560_e47381_d_n8, assign41560_e47381_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_sce2__blk925, var_sce2__blk925_dn4, var_sce2__blk925_dn6, var_sce2__blk925_dn7, var_sce2__blk925_dn8, var_sce2__blk925_dn9,)
    } else {
        (var_sce2_ac, var_sce2_ac_dn4, var_sce2_ac_dn6, var_sce2_ac_dn7, var_sce2_ac_dn8, var_sce2_ac_dn9,)
    }
};
        var_sce2_ac = assign41560_e47381;
        var_sce2_ac_dn4 = assign41560_e47381_d_n4;
        var_sce2_ac_dn6 = assign41560_e47381_d_n6;
        var_sce2_ac_dn7 = assign41560_e47381_d_n7;
        var_sce2_ac_dn8 = assign41560_e47381_d_n8;
        var_sce2_ac_dn9 = assign41560_e47381_d_n9;

        let (assign41570_e47385, assign41570_e47385_d_n4, assign41570_e47385_d_n6, assign41570_e47385_d_n7, assign41570_e47385_d_n8, assign41570_e47385_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_dxg1_dibl__blk926, var_dxg1_dibl__blk926_dn4, var_dxg1_dibl__blk926_dn6, var_dxg1_dibl__blk926_dn7, var_dxg1_dibl__blk926_dn8, var_dxg1_dibl__blk926_dn9,)
    } else {
        (var_dxg1_dibl_ac, var_dxg1_dibl_ac_dn4, var_dxg1_dibl_ac_dn6, var_dxg1_dibl_ac_dn7, var_dxg1_dibl_ac_dn8, var_dxg1_dibl_ac_dn9,)
    }
};
        var_dxg1_dibl_ac = assign41570_e47385;
        var_dxg1_dibl_ac_dn4 = assign41570_e47385_d_n4;
        var_dxg1_dibl_ac_dn6 = assign41570_e47385_d_n6;
        var_dxg1_dibl_ac_dn7 = assign41570_e47385_d_n7;
        var_dxg1_dibl_ac_dn8 = assign41570_e47385_d_n8;
        var_dxg1_dibl_ac_dn9 = assign41570_e47385_d_n9;

        let (assign41580_e47389, assign41580_e47389_d_n4, assign41580_e47389_d_n6, assign41580_e47389_d_n7, assign41580_e47389_d_n8, assign41580_e47389_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_xg2__blk929, var_xg2__blk929_dn4, var_xg2__blk929_dn6, var_xg2__blk929_dn7, var_xg2__blk929_dn8, var_xg2__blk929_dn9,)
    } else {
        (var_xg2_ac, var_xg2_ac_dn4, var_xg2_ac_dn6, var_xg2_ac_dn7, var_xg2_ac_dn8, var_xg2_ac_dn9,)
    }
};
        var_xg2_ac = assign41580_e47389;
        var_xg2_ac_dn4 = assign41580_e47389_d_n4;
        var_xg2_ac_dn6 = assign41580_e47389_d_n6;
        var_xg2_ac_dn7 = assign41580_e47389_d_n7;
        var_xg2_ac_dn8 = assign41580_e47389_d_n8;
        var_xg2_ac_dn9 = assign41580_e47389_d_n9;

        *var_a0_ac_slot = var_a0_ac;
        *var_a0_ac_dn4_slot = var_a0_ac_dn4;
        *var_a0_ac_dn6_slot = var_a0_ac_dn6;
        *var_a0_ac_dn7_slot = var_a0_ac_dn7;
        *var_a0_ac_dn8_slot = var_a0_ac_dn8;
        *var_a0_ac_dn9_slot = var_a0_ac_dn9;
        *var_csiprime_ac_slot = var_csiprime_ac;
        *var_csiprime_ac_dn4_slot = var_csiprime_ac_dn4;
        *var_csiprime_ac_dn6_slot = var_csiprime_ac_dn6;
        *var_csiprime_ac_dn7_slot = var_csiprime_ac_dn7;
        *var_csiprime_ac_dn8_slot = var_csiprime_ac_dn8;
        *var_csiprime_ac_dn9_slot = var_csiprime_ac_dn9;
        *var_delta_k1q1__blk1076_slot = var_delta_k1q1__blk1076;
        *var_delta_k1q1__blk1076_dn4_slot = var_delta_k1q1__blk1076_dn4;
        *var_delta_k1q1__blk1076_dn6_slot = var_delta_k1q1__blk1076_dn6;
        *var_delta_k1q1__blk1076_dn7_slot = var_delta_k1q1__blk1076_dn7;
        *var_delta_k1q1__blk1076_dn8_slot = var_delta_k1q1__blk1076_dn8;
        *var_delta_k1q1__blk1076_dn9_slot = var_delta_k1q1__blk1076_dn9;
        *var_delta_k2q2__blk1077_slot = var_delta_k2q2__blk1077;
        *var_delta_k2q2__blk1077_dn4_slot = var_delta_k2q2__blk1077_dn4;
        *var_delta_k2q2__blk1077_dn6_slot = var_delta_k2q2__blk1077_dn6;
        *var_delta_k2q2__blk1077_dn7_slot = var_delta_k2q2__blk1077_dn7;
        *var_delta_k2q2__blk1077_dn8_slot = var_delta_k2q2__blk1077_dn8;
        *var_delta_k2q2__blk1077_dn9_slot = var_delta_k2q2__blk1077_dn9;
        *var_diff_min_ac_slot = var_diff_min_ac;
        *var_diff_min_ac_dn4_slot = var_diff_min_ac_dn4;
        *var_diff_min_ac_dn6_slot = var_diff_min_ac_dn6;
        *var_diff_min_ac_dn7_slot = var_diff_min_ac_dn7;
        *var_diff_min_ac_dn8_slot = var_diff_min_ac_dn8;
        *var_diff_min_ac_dn9_slot = var_diff_min_ac_dn9;
        *var_dleff_ac_slot = var_dleff_ac;
        *var_dleff_ac_dn4_slot = var_dleff_ac_dn4;
        *var_dleff_ac_dn6_slot = var_dleff_ac_dn6;
        *var_dleff_ac_dn7_slot = var_dleff_ac_dn7;
        *var_dleff_ac_dn8_slot = var_dleff_ac_dn8;
        *var_dleff_ac_dn9_slot = var_dleff_ac_dn9;
        *var_dx_wi_1d_ac_slot = var_dx_wi_1d_ac;
        *var_dx_wi_1d_ac_dn4_slot = var_dx_wi_1d_ac_dn4;
        *var_dx_wi_1d_ac_dn6_slot = var_dx_wi_1d_ac_dn6;
        *var_dx_wi_1d_ac_dn7_slot = var_dx_wi_1d_ac_dn7;
        *var_dx_wi_1d_ac_dn8_slot = var_dx_wi_1d_ac_dn8;
        *var_dx_wi_1d_ac_dn9_slot = var_dx_wi_1d_ac_dn9;
        *var_dx_wi_ac_slot = var_dx_wi_ac;
        *var_dx_wi_ac_dn4_slot = var_dx_wi_ac_dn4;
        *var_dx_wi_ac_dn6_slot = var_dx_wi_ac_dn6;
        *var_dx_wi_ac_dn7_slot = var_dx_wi_ac_dn7;
        *var_dx_wi_ac_dn8_slot = var_dx_wi_ac_dn8;
        *var_dx_wi_ac_dn9_slot = var_dx_wi_ac_dn9;
        *var_dxg1_dibl_ac_slot = var_dxg1_dibl_ac;
        *var_dxg1_dibl_ac_dn4_slot = var_dxg1_dibl_ac_dn4;
        *var_dxg1_dibl_ac_dn6_slot = var_dxg1_dibl_ac_dn6;
        *var_dxg1_dibl_ac_dn7_slot = var_dxg1_dibl_ac_dn7;
        *var_dxg1_dibl_ac_dn8_slot = var_dxg1_dibl_ac_dn8;
        *var_dxg1_dibl_ac_dn9_slot = var_dxg1_dibl_ac_dn9;
        *var_inv_k1_ac_slot = var_inv_k1_ac;
        *var_inv_k1_ac_dn4_slot = var_inv_k1_ac_dn4;
        *var_inv_k1_ac_dn6_slot = var_inv_k1_ac_dn6;
        *var_inv_k1_ac_dn7_slot = var_inv_k1_ac_dn7;
        *var_inv_k1_ac_dn8_slot = var_inv_k1_ac_dn8;
        *var_inv_k1_ac_dn9_slot = var_inv_k1_ac_dn9;
        *var_inv_k1h1_0__blk1066_slot = var_inv_k1h1_0__blk1066;
        *var_inv_k1h1_0__blk1066_dn4_slot = var_inv_k1h1_0__blk1066_dn4;
        *var_inv_k1h1_0__blk1066_dn6_slot = var_inv_k1h1_0__blk1066_dn6;
        *var_inv_k1h1_0__blk1066_dn7_slot = var_inv_k1h1_0__blk1066_dn7;
        *var_inv_k1h1_0__blk1066_dn8_slot = var_inv_k1h1_0__blk1066_dn8;
        *var_inv_k1h1_0__blk1066_dn9_slot = var_inv_k1h1_0__blk1066_dn9;
        *var_inv_k1h1__blk1074_slot = var_inv_k1h1__blk1074;
        *var_inv_k1h1__blk1074_dn4_slot = var_inv_k1h1__blk1074_dn4;
        *var_inv_k1h1__blk1074_dn6_slot = var_inv_k1h1__blk1074_dn6;
        *var_inv_k1h1__blk1074_dn7_slot = var_inv_k1h1__blk1074_dn7;
        *var_inv_k1h1__blk1074_dn8_slot = var_inv_k1h1__blk1074_dn8;
        *var_inv_k1h1__blk1074_dn9_slot = var_inv_k1h1__blk1074_dn9;
        *var_inv_k2_ac_slot = var_inv_k2_ac;
        *var_inv_k2_ac_dn4_slot = var_inv_k2_ac_dn4;
        *var_inv_k2_ac_dn6_slot = var_inv_k2_ac_dn6;
        *var_inv_k2_ac_dn7_slot = var_inv_k2_ac_dn7;
        *var_inv_k2_ac_dn8_slot = var_inv_k2_ac_dn8;
        *var_inv_k2_ac_dn9_slot = var_inv_k2_ac_dn9;
        *var_inv_k2h2_0__blk1069_slot = var_inv_k2h2_0__blk1069;
        *var_inv_k2h2_0__blk1069_dn4_slot = var_inv_k2h2_0__blk1069_dn4;
        *var_inv_k2h2_0__blk1069_dn6_slot = var_inv_k2h2_0__blk1069_dn6;
        *var_inv_k2h2_0__blk1069_dn7_slot = var_inv_k2h2_0__blk1069_dn7;
        *var_inv_k2h2_0__blk1069_dn8_slot = var_inv_k2h2_0__blk1069_dn8;
        *var_inv_k2h2_0__blk1069_dn9_slot = var_inv_k2h2_0__blk1069_dn9;
        *var_inv_k2h2__blk1075_slot = var_inv_k2h2__blk1075;
        *var_inv_k2h2__blk1075_dn4_slot = var_inv_k2h2__blk1075_dn4;
        *var_inv_k2h2__blk1075_dn6_slot = var_inv_k2h2__blk1075_dn6;
        *var_inv_k2h2__blk1075_dn7_slot = var_inv_k2h2__blk1075_dn7;
        *var_inv_k2h2__blk1075_dn8_slot = var_inv_k2h2__blk1075_dn8;
        *var_inv_k2h2__blk1075_dn9_slot = var_inv_k2h2__blk1075_dn9;
        *var_keq_ac_slot = var_keq_ac;
        *var_keq_ac_dn4_slot = var_keq_ac_dn4;
        *var_keq_ac_dn6_slot = var_keq_ac_dn6;
        *var_keq_ac_dn7_slot = var_keq_ac_dn7;
        *var_keq_ac_dn8_slot = var_keq_ac_dn8;
        *var_keq_ac_dn9_slot = var_keq_ac_dn9;
        *var_ksi1__blk1072_slot = var_ksi1__blk1072;
        *var_ksi1__blk1072_dn4_slot = var_ksi1__blk1072_dn4;
        *var_ksi1__blk1072_dn6_slot = var_ksi1__blk1072_dn6;
        *var_ksi1__blk1072_dn7_slot = var_ksi1__blk1072_dn7;
        *var_ksi1__blk1072_dn8_slot = var_ksi1__blk1072_dn8;
        *var_ksi1__blk1072_dn9_slot = var_ksi1__blk1072_dn9;
        *var_ksi2__blk1073_slot = var_ksi2__blk1073;
        *var_ksi2__blk1073_dn4_slot = var_ksi2__blk1073_dn4;
        *var_ksi2__blk1073_dn6_slot = var_ksi2__blk1073_dn6;
        *var_ksi2__blk1073_dn7_slot = var_ksi2__blk1073_dn7;
        *var_ksi2__blk1073_dn8_slot = var_ksi2__blk1073_dn8;
        *var_ksi2__blk1073_dn9_slot = var_ksi2__blk1073_dn9;
        *var_prod1__blk1078_slot = var_prod1__blk1078;
        *var_prod1__blk1078_dn4_slot = var_prod1__blk1078_dn4;
        *var_prod1__blk1078_dn6_slot = var_prod1__blk1078_dn6;
        *var_prod1__blk1078_dn7_slot = var_prod1__blk1078_dn7;
        *var_prod1__blk1078_dn8_slot = var_prod1__blk1078_dn8;
        *var_prod1__blk1078_dn9_slot = var_prod1__blk1078_dn9;
        *var_prod2__blk1079_slot = var_prod2__blk1079;
        *var_prod2__blk1079_dn4_slot = var_prod2__blk1079_dn4;
        *var_prod2__blk1079_dn6_slot = var_prod2__blk1079_dn6;
        *var_prod2__blk1079_dn7_slot = var_prod2__blk1079_dn7;
        *var_prod2__blk1079_dn8_slot = var_prod2__blk1079_dn8;
        *var_prod2__blk1079_dn9_slot = var_prod2__blk1079_dn9;
        *var_sce1_ac_slot = var_sce1_ac;
        *var_sce1_ac_dn4_slot = var_sce1_ac_dn4;
        *var_sce1_ac_dn6_slot = var_sce1_ac_dn6;
        *var_sce1_ac_dn7_slot = var_sce1_ac_dn7;
        *var_sce1_ac_dn8_slot = var_sce1_ac_dn8;
        *var_sce1_ac_dn9_slot = var_sce1_ac_dn9;
        *var_sce2_ac_slot = var_sce2_ac;
        *var_sce2_ac_dn4_slot = var_sce2_ac_dn4;
        *var_sce2_ac_dn6_slot = var_sce2_ac_dn6;
        *var_sce2_ac_dn7_slot = var_sce2_ac_dn7;
        *var_sce2_ac_dn8_slot = var_sce2_ac_dn8;
        *var_sce2_ac_dn9_slot = var_sce2_ac_dn9;
        *var_temp_slot = var_temp;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp3_slot = var_temp3;
        *var_temp3_dn4_slot = var_temp3_dn4;
        *var_temp3_dn6_slot = var_temp3_dn6;
        *var_temp3_dn7_slot = var_temp3_dn7;
        *var_temp3_dn8_slot = var_temp3_dn8;
        *var_temp3_dn9_slot = var_temp3_dn9;
        *var_temp4_slot = var_temp4;
        *var_temp4_dn4_slot = var_temp4_dn4;
        *var_temp4_dn6_slot = var_temp4_dn6;
        *var_temp4_dn7_slot = var_temp4_dn7;
        *var_temp4_dn8_slot = var_temp4_dn8;
        *var_temp4_dn9_slot = var_temp4_dn9;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_xedge_ac_slot = var_xedge_ac;
        *var_xedge_ac_dn4_slot = var_xedge_ac_dn4;
        *var_xedge_ac_dn6_slot = var_xedge_ac_dn6;
        *var_xedge_ac_dn7_slot = var_xedge_ac_dn7;
        *var_xedge_ac_dn8_slot = var_xedge_ac_dn8;
        *var_xedge_ac_dn9_slot = var_xedge_ac_dn9;
        *var_xg20shift_ac_slot = var_xg20shift_ac;
        *var_xg20shift_ac_dn4_slot = var_xg20shift_ac_dn4;
        *var_xg20shift_ac_dn6_slot = var_xg20shift_ac_dn6;
        *var_xg20shift_ac_dn7_slot = var_xg20shift_ac_dn7;
        *var_xg20shift_ac_dn8_slot = var_xg20shift_ac_dn8;
        *var_xg20shift_ac_dn9_slot = var_xg20shift_ac_dn9;
        *var_xg2_ac_slot = var_xg2_ac;
        *var_xg2_ac_dn4_slot = var_xg2_ac_dn4;
        *var_xg2_ac_dn6_slot = var_xg2_ac_dn6;
        *var_xg2_ac_dn7_slot = var_xg2_ac_dn7;
        *var_xg2_ac_dn8_slot = var_xg2_ac_dn8;
        *var_xg2_ac_dn9_slot = var_xg2_ac_dn9;
        *var_zeta1__blk1070_slot = var_zeta1__blk1070;
        *var_zeta1__blk1070_dn4_slot = var_zeta1__blk1070_dn4;
        *var_zeta1__blk1070_dn6_slot = var_zeta1__blk1070_dn6;
        *var_zeta1__blk1070_dn7_slot = var_zeta1__blk1070_dn7;
        *var_zeta1__blk1070_dn8_slot = var_zeta1__blk1070_dn8;
        *var_zeta1__blk1070_dn9_slot = var_zeta1__blk1070_dn9;
        *var_zeta2__blk1071_slot = var_zeta2__blk1071;
        *var_zeta2__blk1071_dn4_slot = var_zeta2__blk1071_dn4;
        *var_zeta2__blk1071_dn6_slot = var_zeta2__blk1071_dn6;
        *var_zeta2__blk1071_dn7_slot = var_zeta2__blk1071_dn7;
        *var_zeta2__blk1071_dn8_slot = var_zeta2__blk1071_dn8;
        *var_zeta2__blk1071_dn9_slot = var_zeta2__blk1071_dn9;
    }

    pub(super) fn stamp_transient_block_116(
        var_a0_dc: f64,
        var_a0_dc_dn4: f64,
        var_a0_dc_dn6: f64,
        var_a0_dc_dn7: f64,
        var_a0_dc_dn8: f64,
        var_a0_dc_dn9: f64,
        var_csiprime_dc: f64,
        var_csiprime_dc_dn4: f64,
        var_csiprime_dc_dn6: f64,
        var_csiprime_dc_dn7: f64,
        var_csiprime_dc_dn8: f64,
        var_csiprime_dc_dn9: f64,
        var_delta_k1q1__blk1076: f64,
        var_delta_k1q1__blk1076_dn4: f64,
        var_delta_k1q1__blk1076_dn6: f64,
        var_delta_k1q1__blk1076_dn7: f64,
        var_delta_k1q1__blk1076_dn8: f64,
        var_delta_k1q1__blk1076_dn9: f64,
        var_delta_k2q2__blk1077: f64,
        var_delta_k2q2__blk1077_dn4: f64,
        var_delta_k2q2__blk1077_dn6: f64,
        var_delta_k2q2__blk1077_dn7: f64,
        var_delta_k2q2__blk1077_dn8: f64,
        var_delta_k2q2__blk1077_dn9: f64,
        var_diff_min_dc: f64,
        var_diff_min_dc_dn4: f64,
        var_diff_min_dc_dn6: f64,
        var_diff_min_dc_dn7: f64,
        var_diff_min_dc_dn8: f64,
        var_diff_min_dc_dn9: f64,
        var_dleff_dc: f64,
        var_dleff_dc_dn4: f64,
        var_dleff_dc_dn6: f64,
        var_dleff_dc_dn7: f64,
        var_dleff_dc_dn8: f64,
        var_dleff_dc_dn9: f64,
        var_dx_wi_1d_dc: f64,
        var_dx_wi_1d_dc_dn4: f64,
        var_dx_wi_1d_dc_dn6: f64,
        var_dx_wi_1d_dc_dn7: f64,
        var_dx_wi_1d_dc_dn8: f64,
        var_dx_wi_1d_dc_dn9: f64,
        var_dx_wi_dc: f64,
        var_dx_wi_dc_dn4: f64,
        var_dx_wi_dc_dn6: f64,
        var_dx_wi_dc_dn7: f64,
        var_dx_wi_dc_dn8: f64,
        var_dx_wi_dc_dn9: f64,
        var_dxg1_dibl_dc: f64,
        var_dxg1_dibl_dc_dn4: f64,
        var_dxg1_dibl_dc_dn6: f64,
        var_dxg1_dibl_dc_dn7: f64,
        var_dxg1_dibl_dc_dn8: f64,
        var_dxg1_dibl_dc_dn9: f64,
        var_guard1080: f64,
        var_inv_k1_dc: f64,
        var_inv_k1_dc_dn4: f64,
        var_inv_k1_dc_dn6: f64,
        var_inv_k1_dc_dn7: f64,
        var_inv_k1_dc_dn8: f64,
        var_inv_k1_dc_dn9: f64,
        var_inv_k2_dc: f64,
        var_inv_k2_dc_dn4: f64,
        var_inv_k2_dc_dn6: f64,
        var_inv_k2_dc_dn7: f64,
        var_inv_k2_dc_dn8: f64,
        var_inv_k2_dc_dn9: f64,
        var_k1__blk932: f64,
        var_k1__blk932_dn4: f64,
        var_k1__blk932_dn6: f64,
        var_k1__blk932_dn7: f64,
        var_k1__blk932_dn8: f64,
        var_k1__blk932_dn9: f64,
        var_k1q1d__blk1004: f64,
        var_k1q1d__blk1004_dn4: f64,
        var_k1q1d__blk1004_dn6: f64,
        var_k1q1d__blk1004_dn7: f64,
        var_k1q1d__blk1004_dn8: f64,
        var_k1q1d__blk1004_dn9: f64,
        var_k1q1s__blk939: f64,
        var_k1q1s__blk939_dn4: f64,
        var_k1q1s__blk939_dn6: f64,
        var_k1q1s__blk939_dn7: f64,
        var_k1q1s__blk939_dn8: f64,
        var_k1q1s__blk939_dn9: f64,
        var_k2__blk933: f64,
        var_k2__blk933_dn4: f64,
        var_k2__blk933_dn6: f64,
        var_k2__blk933_dn7: f64,
        var_k2__blk933_dn8: f64,
        var_k2__blk933_dn9: f64,
        var_k2q2d__blk1005: f64,
        var_k2q2d__blk1005_dn4: f64,
        var_k2q2d__blk1005_dn6: f64,
        var_k2q2d__blk1005_dn7: f64,
        var_k2q2d__blk1005_dn8: f64,
        var_k2q2d__blk1005_dn9: f64,
        var_k2q2s__blk940: f64,
        var_k2q2s__blk940_dn4: f64,
        var_k2q2s__blk940_dn6: f64,
        var_k2q2s__blk940_dn7: f64,
        var_k2q2s__blk940_dn8: f64,
        var_k2q2s__blk940_dn9: f64,
        var_keq_dc: f64,
        var_keq_dc_dn4: f64,
        var_keq_dc_dn6: f64,
        var_keq_dc_dn7: f64,
        var_keq_dc_dn8: f64,
        var_keq_dc_dn9: f64,
        var_prod1__blk1078: f64,
        var_prod1__blk1078_dn4: f64,
        var_prod1__blk1078_dn6: f64,
        var_prod1__blk1078_dn7: f64,
        var_prod1__blk1078_dn8: f64,
        var_prod1__blk1078_dn9: f64,
        var_prod2__blk1079: f64,
        var_prod2__blk1079_dn4: f64,
        var_prod2__blk1079_dn6: f64,
        var_prod2__blk1079_dn7: f64,
        var_prod2__blk1079_dn8: f64,
        var_prod2__blk1079_dn9: f64,
        var_qi1m__blk1029: f64,
        var_qi1m__blk1029_dn4: f64,
        var_qi1m__blk1029_dn6: f64,
        var_qi1m__blk1029_dn7: f64,
        var_qi1m__blk1029_dn8: f64,
        var_qi1m__blk1029_dn9: f64,
        var_qi2m__blk1030: f64,
        var_qi2m__blk1030_dn4: f64,
        var_qi2m__blk1030_dn6: f64,
        var_qi2m__blk1030_dn7: f64,
        var_qi2m__blk1030_dn8: f64,
        var_qi2m__blk1030_dn9: f64,
        var_qim__blk1016: f64,
        var_qim__blk1016_dn4: f64,
        var_qim__blk1016_dn6: f64,
        var_qim__blk1016_dn7: f64,
        var_qim__blk1016_dn8: f64,
        var_qim__blk1016_dn9: f64,
        var_qmfact1__blk1054: f64,
        var_qmfact1__blk1054_dn4: f64,
        var_qmfact1__blk1054_dn6: f64,
        var_qmfact1__blk1054_dn7: f64,
        var_qmfact1__blk1054_dn8: f64,
        var_qmfact1__blk1054_dn9: f64,
        var_qmfact2__blk1055: f64,
        var_qmfact2__blk1055_dn4: f64,
        var_qmfact2__blk1055_dn6: f64,
        var_qmfact2__blk1055_dn7: f64,
        var_qmfact2__blk1055_dn8: f64,
        var_qmfact2__blk1055_dn9: f64,
        var_ratio_pd__blk1020: f64,
        var_ratio_pd__blk1020_dn4: f64,
        var_ratio_pd__blk1020_dn6: f64,
        var_ratio_pd__blk1020_dn7: f64,
        var_ratio_pd__blk1020_dn8: f64,
        var_ratio_pd__blk1020_dn9: f64,
        var_sce1_dc: f64,
        var_sce1_dc_dn4: f64,
        var_sce1_dc_dn6: f64,
        var_sce1_dc_dn7: f64,
        var_sce1_dc_dn8: f64,
        var_sce1_dc_dn9: f64,
        var_sce2_dc: f64,
        var_sce2_dc_dn4: f64,
        var_sce2_dc_dn6: f64,
        var_sce2_dc_dn7: f64,
        var_sce2_dc_dn8: f64,
        var_sce2_dc_dn9: f64,
        var_xdriftd__blk1015: f64,
        var_xdriftd__blk1015_dn4: f64,
        var_xdriftd__blk1015_dn6: f64,
        var_xdriftd__blk1015_dn7: f64,
        var_xdriftd__blk1015_dn8: f64,
        var_xdriftd__blk1015_dn9: f64,
        var_xdrifts__blk951: f64,
        var_xdrifts__blk951_dn4: f64,
        var_xdrifts__blk951_dn6: f64,
        var_xdrifts__blk951_dn7: f64,
        var_xdrifts__blk951_dn8: f64,
        var_xdrifts__blk951_dn9: f64,
        var_xedge_dc: f64,
        var_xedge_dc_dn4: f64,
        var_xedge_dc_dn6: f64,
        var_xedge_dc_dn7: f64,
        var_xedge_dc_dn8: f64,
        var_xedge_dc_dn9: f64,
        var_xg20shift_dc: f64,
        var_xg20shift_dc_dn4: f64,
        var_xg20shift_dc_dn6: f64,
        var_xg20shift_dc_dn7: f64,
        var_xg20shift_dc_dn8: f64,
        var_xg20shift_dc_dn9: f64,
        var_xg2_dc: f64,
        var_xg2_dc_dn4: f64,
        var_xg2_dc_dn6: f64,
        var_xg2_dc_dn7: f64,
        var_xg2_dc_dn8: f64,
        var_xg2_dc_dn9: f64,
        var_xg2x__blk931: f64,
        var_xg2x__blk931_dn4: f64,
        var_xg2x__blk931_dn6: f64,
        var_xg2x__blk931_dn7: f64,
        var_xg2x__blk931_dn8: f64,
        var_xg2x__blk931_dn9: f64,
        var_xg2x_dc: f64,
        var_xg2x_dc_dn4: f64,
        var_xg2x_dc_dn6: f64,
        var_xg2x_dc_dn7: f64,
        var_xg2x_dc_dn8: f64,
        var_xg2x_dc_dn9: f64,
        var_zsat__blk1051: f64,
        var_zsat__blk1051_dn4: f64,
        var_zsat__blk1051_dn6: f64,
        var_zsat__blk1051_dn7: f64,
        var_zsat__blk1051_dn8: f64,
        var_zsat__blk1051_dn9: f64,
        var_a0_ac_slot: &mut f64,
        var_a0_ac_dn4_slot: &mut f64,
        var_a0_ac_dn6_slot: &mut f64,
        var_a0_ac_dn7_slot: &mut f64,
        var_a0_ac_dn8_slot: &mut f64,
        var_a0_ac_dn9_slot: &mut f64,
        var_csiprime_ac_slot: &mut f64,
        var_csiprime_ac_dn4_slot: &mut f64,
        var_csiprime_ac_dn6_slot: &mut f64,
        var_csiprime_ac_dn7_slot: &mut f64,
        var_csiprime_ac_dn8_slot: &mut f64,
        var_csiprime_ac_dn9_slot: &mut f64,
        var_delta_k1q1_ac_slot: &mut f64,
        var_delta_k1q1_ac_dn4_slot: &mut f64,
        var_delta_k1q1_ac_dn6_slot: &mut f64,
        var_delta_k1q1_ac_dn7_slot: &mut f64,
        var_delta_k1q1_ac_dn8_slot: &mut f64,
        var_delta_k1q1_ac_dn9_slot: &mut f64,
        var_delta_k2q2_ac_slot: &mut f64,
        var_delta_k2q2_ac_dn4_slot: &mut f64,
        var_delta_k2q2_ac_dn6_slot: &mut f64,
        var_delta_k2q2_ac_dn7_slot: &mut f64,
        var_delta_k2q2_ac_dn8_slot: &mut f64,
        var_delta_k2q2_ac_dn9_slot: &mut f64,
        var_diff_min_ac_slot: &mut f64,
        var_diff_min_ac_dn4_slot: &mut f64,
        var_diff_min_ac_dn6_slot: &mut f64,
        var_diff_min_ac_dn7_slot: &mut f64,
        var_diff_min_ac_dn8_slot: &mut f64,
        var_diff_min_ac_dn9_slot: &mut f64,
        var_dleff_ac_slot: &mut f64,
        var_dleff_ac_dn4_slot: &mut f64,
        var_dleff_ac_dn6_slot: &mut f64,
        var_dleff_ac_dn7_slot: &mut f64,
        var_dleff_ac_dn8_slot: &mut f64,
        var_dleff_ac_dn9_slot: &mut f64,
        var_dx_wi_1d_ac_slot: &mut f64,
        var_dx_wi_1d_ac_dn4_slot: &mut f64,
        var_dx_wi_1d_ac_dn6_slot: &mut f64,
        var_dx_wi_1d_ac_dn7_slot: &mut f64,
        var_dx_wi_1d_ac_dn8_slot: &mut f64,
        var_dx_wi_1d_ac_dn9_slot: &mut f64,
        var_dx_wi_ac_slot: &mut f64,
        var_dx_wi_ac_dn4_slot: &mut f64,
        var_dx_wi_ac_dn6_slot: &mut f64,
        var_dx_wi_ac_dn7_slot: &mut f64,
        var_dx_wi_ac_dn8_slot: &mut f64,
        var_dx_wi_ac_dn9_slot: &mut f64,
        var_dxg1_dibl_ac_slot: &mut f64,
        var_dxg1_dibl_ac_dn4_slot: &mut f64,
        var_dxg1_dibl_ac_dn6_slot: &mut f64,
        var_dxg1_dibl_ac_dn7_slot: &mut f64,
        var_dxg1_dibl_ac_dn8_slot: &mut f64,
        var_dxg1_dibl_ac_dn9_slot: &mut f64,
        var_inv_k1_ac_slot: &mut f64,
        var_inv_k1_ac_dn4_slot: &mut f64,
        var_inv_k1_ac_dn6_slot: &mut f64,
        var_inv_k1_ac_dn7_slot: &mut f64,
        var_inv_k1_ac_dn8_slot: &mut f64,
        var_inv_k1_ac_dn9_slot: &mut f64,
        var_inv_k2_ac_slot: &mut f64,
        var_inv_k2_ac_dn4_slot: &mut f64,
        var_inv_k2_ac_dn6_slot: &mut f64,
        var_inv_k2_ac_dn7_slot: &mut f64,
        var_inv_k2_ac_dn8_slot: &mut f64,
        var_inv_k2_ac_dn9_slot: &mut f64,
        var_k1_ac_slot: &mut f64,
        var_k1_ac_dn4_slot: &mut f64,
        var_k1_ac_dn6_slot: &mut f64,
        var_k1_ac_dn7_slot: &mut f64,
        var_k1_ac_dn8_slot: &mut f64,
        var_k1_ac_dn9_slot: &mut f64,
        var_k1q1d_ac_slot: &mut f64,
        var_k1q1d_ac_dn4_slot: &mut f64,
        var_k1q1d_ac_dn6_slot: &mut f64,
        var_k1q1d_ac_dn7_slot: &mut f64,
        var_k1q1d_ac_dn8_slot: &mut f64,
        var_k1q1d_ac_dn9_slot: &mut f64,
        var_k1q1s_ac_slot: &mut f64,
        var_k1q1s_ac_dn4_slot: &mut f64,
        var_k1q1s_ac_dn6_slot: &mut f64,
        var_k1q1s_ac_dn7_slot: &mut f64,
        var_k1q1s_ac_dn8_slot: &mut f64,
        var_k1q1s_ac_dn9_slot: &mut f64,
        var_k2_ac_slot: &mut f64,
        var_k2_ac_dn4_slot: &mut f64,
        var_k2_ac_dn6_slot: &mut f64,
        var_k2_ac_dn7_slot: &mut f64,
        var_k2_ac_dn8_slot: &mut f64,
        var_k2_ac_dn9_slot: &mut f64,
        var_k2q2d_ac_slot: &mut f64,
        var_k2q2d_ac_dn4_slot: &mut f64,
        var_k2q2d_ac_dn6_slot: &mut f64,
        var_k2q2d_ac_dn7_slot: &mut f64,
        var_k2q2d_ac_dn8_slot: &mut f64,
        var_k2q2d_ac_dn9_slot: &mut f64,
        var_k2q2s_ac_slot: &mut f64,
        var_k2q2s_ac_dn4_slot: &mut f64,
        var_k2q2s_ac_dn6_slot: &mut f64,
        var_k2q2s_ac_dn7_slot: &mut f64,
        var_k2q2s_ac_dn8_slot: &mut f64,
        var_k2q2s_ac_dn9_slot: &mut f64,
        var_keq_ac_slot: &mut f64,
        var_keq_ac_dn4_slot: &mut f64,
        var_keq_ac_dn6_slot: &mut f64,
        var_keq_ac_dn7_slot: &mut f64,
        var_keq_ac_dn8_slot: &mut f64,
        var_keq_ac_dn9_slot: &mut f64,
        var_prod1_ac_slot: &mut f64,
        var_prod1_ac_dn4_slot: &mut f64,
        var_prod1_ac_dn6_slot: &mut f64,
        var_prod1_ac_dn7_slot: &mut f64,
        var_prod1_ac_dn8_slot: &mut f64,
        var_prod1_ac_dn9_slot: &mut f64,
        var_prod2_ac_slot: &mut f64,
        var_prod2_ac_dn4_slot: &mut f64,
        var_prod2_ac_dn6_slot: &mut f64,
        var_prod2_ac_dn7_slot: &mut f64,
        var_prod2_ac_dn8_slot: &mut f64,
        var_prod2_ac_dn9_slot: &mut f64,
        var_qi1m_ac_slot: &mut f64,
        var_qi1m_ac_dn4_slot: &mut f64,
        var_qi1m_ac_dn6_slot: &mut f64,
        var_qi1m_ac_dn7_slot: &mut f64,
        var_qi1m_ac_dn8_slot: &mut f64,
        var_qi1m_ac_dn9_slot: &mut f64,
        var_qi2m_ac_slot: &mut f64,
        var_qi2m_ac_dn4_slot: &mut f64,
        var_qi2m_ac_dn6_slot: &mut f64,
        var_qi2m_ac_dn7_slot: &mut f64,
        var_qi2m_ac_dn8_slot: &mut f64,
        var_qi2m_ac_dn9_slot: &mut f64,
        var_qim_ac_slot: &mut f64,
        var_qim_ac_dn4_slot: &mut f64,
        var_qim_ac_dn6_slot: &mut f64,
        var_qim_ac_dn7_slot: &mut f64,
        var_qim_ac_dn8_slot: &mut f64,
        var_qim_ac_dn9_slot: &mut f64,
        var_qmfact1_ac_slot: &mut f64,
        var_qmfact1_ac_dn4_slot: &mut f64,
        var_qmfact1_ac_dn6_slot: &mut f64,
        var_qmfact1_ac_dn7_slot: &mut f64,
        var_qmfact1_ac_dn8_slot: &mut f64,
        var_qmfact1_ac_dn9_slot: &mut f64,
        var_qmfact2_ac_slot: &mut f64,
        var_qmfact2_ac_dn4_slot: &mut f64,
        var_qmfact2_ac_dn6_slot: &mut f64,
        var_qmfact2_ac_dn7_slot: &mut f64,
        var_qmfact2_ac_dn8_slot: &mut f64,
        var_qmfact2_ac_dn9_slot: &mut f64,
        var_ratio_pd_ac_slot: &mut f64,
        var_ratio_pd_ac_dn4_slot: &mut f64,
        var_ratio_pd_ac_dn6_slot: &mut f64,
        var_ratio_pd_ac_dn7_slot: &mut f64,
        var_ratio_pd_ac_dn8_slot: &mut f64,
        var_ratio_pd_ac_dn9_slot: &mut f64,
        var_sce1_ac_slot: &mut f64,
        var_sce1_ac_dn4_slot: &mut f64,
        var_sce1_ac_dn6_slot: &mut f64,
        var_sce1_ac_dn7_slot: &mut f64,
        var_sce1_ac_dn8_slot: &mut f64,
        var_sce1_ac_dn9_slot: &mut f64,
        var_sce2_ac_slot: &mut f64,
        var_sce2_ac_dn4_slot: &mut f64,
        var_sce2_ac_dn6_slot: &mut f64,
        var_sce2_ac_dn7_slot: &mut f64,
        var_sce2_ac_dn8_slot: &mut f64,
        var_sce2_ac_dn9_slot: &mut f64,
        var_xdriftd_ac_slot: &mut f64,
        var_xdriftd_ac_dn4_slot: &mut f64,
        var_xdriftd_ac_dn6_slot: &mut f64,
        var_xdriftd_ac_dn7_slot: &mut f64,
        var_xdriftd_ac_dn8_slot: &mut f64,
        var_xdriftd_ac_dn9_slot: &mut f64,
        var_xdrifts_ac_slot: &mut f64,
        var_xdrifts_ac_dn4_slot: &mut f64,
        var_xdrifts_ac_dn6_slot: &mut f64,
        var_xdrifts_ac_dn7_slot: &mut f64,
        var_xdrifts_ac_dn8_slot: &mut f64,
        var_xdrifts_ac_dn9_slot: &mut f64,
        var_xedge_ac_slot: &mut f64,
        var_xedge_ac_dn4_slot: &mut f64,
        var_xedge_ac_dn6_slot: &mut f64,
        var_xedge_ac_dn7_slot: &mut f64,
        var_xedge_ac_dn8_slot: &mut f64,
        var_xedge_ac_dn9_slot: &mut f64,
        var_xg20shift_ac_slot: &mut f64,
        var_xg20shift_ac_dn4_slot: &mut f64,
        var_xg20shift_ac_dn6_slot: &mut f64,
        var_xg20shift_ac_dn7_slot: &mut f64,
        var_xg20shift_ac_dn8_slot: &mut f64,
        var_xg20shift_ac_dn9_slot: &mut f64,
        var_xg2_ac_slot: &mut f64,
        var_xg2_ac_dn4_slot: &mut f64,
        var_xg2_ac_dn6_slot: &mut f64,
        var_xg2_ac_dn7_slot: &mut f64,
        var_xg2_ac_dn8_slot: &mut f64,
        var_xg2_ac_dn9_slot: &mut f64,
        var_xg2x_ac_slot: &mut f64,
        var_xg2x_ac_dn4_slot: &mut f64,
        var_xg2x_ac_dn6_slot: &mut f64,
        var_xg2x_ac_dn7_slot: &mut f64,
        var_xg2x_ac_dn8_slot: &mut f64,
        var_xg2x_ac_dn9_slot: &mut f64,
        var_zsat_ac_slot: &mut f64,
        var_zsat_ac_dn4_slot: &mut f64,
        var_zsat_ac_dn6_slot: &mut f64,
        var_zsat_ac_dn7_slot: &mut f64,
        var_zsat_ac_dn8_slot: &mut f64,
        var_zsat_ac_dn9_slot: &mut f64,
    ) {
        let mut var_a0_ac: f64 = *var_a0_ac_slot;
        let mut var_a0_ac_dn4: f64 = *var_a0_ac_dn4_slot;
        let mut var_a0_ac_dn6: f64 = *var_a0_ac_dn6_slot;
        let mut var_a0_ac_dn7: f64 = *var_a0_ac_dn7_slot;
        let mut var_a0_ac_dn8: f64 = *var_a0_ac_dn8_slot;
        let mut var_a0_ac_dn9: f64 = *var_a0_ac_dn9_slot;
        let mut var_csiprime_ac: f64 = *var_csiprime_ac_slot;
        let mut var_csiprime_ac_dn4: f64 = *var_csiprime_ac_dn4_slot;
        let mut var_csiprime_ac_dn6: f64 = *var_csiprime_ac_dn6_slot;
        let mut var_csiprime_ac_dn7: f64 = *var_csiprime_ac_dn7_slot;
        let mut var_csiprime_ac_dn8: f64 = *var_csiprime_ac_dn8_slot;
        let mut var_csiprime_ac_dn9: f64 = *var_csiprime_ac_dn9_slot;
        let mut var_delta_k1q1_ac: f64 = *var_delta_k1q1_ac_slot;
        let mut var_delta_k1q1_ac_dn4: f64 = *var_delta_k1q1_ac_dn4_slot;
        let mut var_delta_k1q1_ac_dn6: f64 = *var_delta_k1q1_ac_dn6_slot;
        let mut var_delta_k1q1_ac_dn7: f64 = *var_delta_k1q1_ac_dn7_slot;
        let mut var_delta_k1q1_ac_dn8: f64 = *var_delta_k1q1_ac_dn8_slot;
        let mut var_delta_k1q1_ac_dn9: f64 = *var_delta_k1q1_ac_dn9_slot;
        let mut var_delta_k2q2_ac: f64 = *var_delta_k2q2_ac_slot;
        let mut var_delta_k2q2_ac_dn4: f64 = *var_delta_k2q2_ac_dn4_slot;
        let mut var_delta_k2q2_ac_dn6: f64 = *var_delta_k2q2_ac_dn6_slot;
        let mut var_delta_k2q2_ac_dn7: f64 = *var_delta_k2q2_ac_dn7_slot;
        let mut var_delta_k2q2_ac_dn8: f64 = *var_delta_k2q2_ac_dn8_slot;
        let mut var_delta_k2q2_ac_dn9: f64 = *var_delta_k2q2_ac_dn9_slot;
        let mut var_diff_min_ac: f64 = *var_diff_min_ac_slot;
        let mut var_diff_min_ac_dn4: f64 = *var_diff_min_ac_dn4_slot;
        let mut var_diff_min_ac_dn6: f64 = *var_diff_min_ac_dn6_slot;
        let mut var_diff_min_ac_dn7: f64 = *var_diff_min_ac_dn7_slot;
        let mut var_diff_min_ac_dn8: f64 = *var_diff_min_ac_dn8_slot;
        let mut var_diff_min_ac_dn9: f64 = *var_diff_min_ac_dn9_slot;
        let mut var_dleff_ac: f64 = *var_dleff_ac_slot;
        let mut var_dleff_ac_dn4: f64 = *var_dleff_ac_dn4_slot;
        let mut var_dleff_ac_dn6: f64 = *var_dleff_ac_dn6_slot;
        let mut var_dleff_ac_dn7: f64 = *var_dleff_ac_dn7_slot;
        let mut var_dleff_ac_dn8: f64 = *var_dleff_ac_dn8_slot;
        let mut var_dleff_ac_dn9: f64 = *var_dleff_ac_dn9_slot;
        let mut var_dx_wi_1d_ac: f64 = *var_dx_wi_1d_ac_slot;
        let mut var_dx_wi_1d_ac_dn4: f64 = *var_dx_wi_1d_ac_dn4_slot;
        let mut var_dx_wi_1d_ac_dn6: f64 = *var_dx_wi_1d_ac_dn6_slot;
        let mut var_dx_wi_1d_ac_dn7: f64 = *var_dx_wi_1d_ac_dn7_slot;
        let mut var_dx_wi_1d_ac_dn8: f64 = *var_dx_wi_1d_ac_dn8_slot;
        let mut var_dx_wi_1d_ac_dn9: f64 = *var_dx_wi_1d_ac_dn9_slot;
        let mut var_dx_wi_ac: f64 = *var_dx_wi_ac_slot;
        let mut var_dx_wi_ac_dn4: f64 = *var_dx_wi_ac_dn4_slot;
        let mut var_dx_wi_ac_dn6: f64 = *var_dx_wi_ac_dn6_slot;
        let mut var_dx_wi_ac_dn7: f64 = *var_dx_wi_ac_dn7_slot;
        let mut var_dx_wi_ac_dn8: f64 = *var_dx_wi_ac_dn8_slot;
        let mut var_dx_wi_ac_dn9: f64 = *var_dx_wi_ac_dn9_slot;
        let mut var_dxg1_dibl_ac: f64 = *var_dxg1_dibl_ac_slot;
        let mut var_dxg1_dibl_ac_dn4: f64 = *var_dxg1_dibl_ac_dn4_slot;
        let mut var_dxg1_dibl_ac_dn6: f64 = *var_dxg1_dibl_ac_dn6_slot;
        let mut var_dxg1_dibl_ac_dn7: f64 = *var_dxg1_dibl_ac_dn7_slot;
        let mut var_dxg1_dibl_ac_dn8: f64 = *var_dxg1_dibl_ac_dn8_slot;
        let mut var_dxg1_dibl_ac_dn9: f64 = *var_dxg1_dibl_ac_dn9_slot;
        let mut var_inv_k1_ac: f64 = *var_inv_k1_ac_slot;
        let mut var_inv_k1_ac_dn4: f64 = *var_inv_k1_ac_dn4_slot;
        let mut var_inv_k1_ac_dn6: f64 = *var_inv_k1_ac_dn6_slot;
        let mut var_inv_k1_ac_dn7: f64 = *var_inv_k1_ac_dn7_slot;
        let mut var_inv_k1_ac_dn8: f64 = *var_inv_k1_ac_dn8_slot;
        let mut var_inv_k1_ac_dn9: f64 = *var_inv_k1_ac_dn9_slot;
        let mut var_inv_k2_ac: f64 = *var_inv_k2_ac_slot;
        let mut var_inv_k2_ac_dn4: f64 = *var_inv_k2_ac_dn4_slot;
        let mut var_inv_k2_ac_dn6: f64 = *var_inv_k2_ac_dn6_slot;
        let mut var_inv_k2_ac_dn7: f64 = *var_inv_k2_ac_dn7_slot;
        let mut var_inv_k2_ac_dn8: f64 = *var_inv_k2_ac_dn8_slot;
        let mut var_inv_k2_ac_dn9: f64 = *var_inv_k2_ac_dn9_slot;
        let mut var_k1_ac: f64 = *var_k1_ac_slot;
        let mut var_k1_ac_dn4: f64 = *var_k1_ac_dn4_slot;
        let mut var_k1_ac_dn6: f64 = *var_k1_ac_dn6_slot;
        let mut var_k1_ac_dn7: f64 = *var_k1_ac_dn7_slot;
        let mut var_k1_ac_dn8: f64 = *var_k1_ac_dn8_slot;
        let mut var_k1_ac_dn9: f64 = *var_k1_ac_dn9_slot;
        let mut var_k1q1d_ac: f64 = *var_k1q1d_ac_slot;
        let mut var_k1q1d_ac_dn4: f64 = *var_k1q1d_ac_dn4_slot;
        let mut var_k1q1d_ac_dn6: f64 = *var_k1q1d_ac_dn6_slot;
        let mut var_k1q1d_ac_dn7: f64 = *var_k1q1d_ac_dn7_slot;
        let mut var_k1q1d_ac_dn8: f64 = *var_k1q1d_ac_dn8_slot;
        let mut var_k1q1d_ac_dn9: f64 = *var_k1q1d_ac_dn9_slot;
        let mut var_k1q1s_ac: f64 = *var_k1q1s_ac_slot;
        let mut var_k1q1s_ac_dn4: f64 = *var_k1q1s_ac_dn4_slot;
        let mut var_k1q1s_ac_dn6: f64 = *var_k1q1s_ac_dn6_slot;
        let mut var_k1q1s_ac_dn7: f64 = *var_k1q1s_ac_dn7_slot;
        let mut var_k1q1s_ac_dn8: f64 = *var_k1q1s_ac_dn8_slot;
        let mut var_k1q1s_ac_dn9: f64 = *var_k1q1s_ac_dn9_slot;
        let mut var_k2_ac: f64 = *var_k2_ac_slot;
        let mut var_k2_ac_dn4: f64 = *var_k2_ac_dn4_slot;
        let mut var_k2_ac_dn6: f64 = *var_k2_ac_dn6_slot;
        let mut var_k2_ac_dn7: f64 = *var_k2_ac_dn7_slot;
        let mut var_k2_ac_dn8: f64 = *var_k2_ac_dn8_slot;
        let mut var_k2_ac_dn9: f64 = *var_k2_ac_dn9_slot;
        let mut var_k2q2d_ac: f64 = *var_k2q2d_ac_slot;
        let mut var_k2q2d_ac_dn4: f64 = *var_k2q2d_ac_dn4_slot;
        let mut var_k2q2d_ac_dn6: f64 = *var_k2q2d_ac_dn6_slot;
        let mut var_k2q2d_ac_dn7: f64 = *var_k2q2d_ac_dn7_slot;
        let mut var_k2q2d_ac_dn8: f64 = *var_k2q2d_ac_dn8_slot;
        let mut var_k2q2d_ac_dn9: f64 = *var_k2q2d_ac_dn9_slot;
        let mut var_k2q2s_ac: f64 = *var_k2q2s_ac_slot;
        let mut var_k2q2s_ac_dn4: f64 = *var_k2q2s_ac_dn4_slot;
        let mut var_k2q2s_ac_dn6: f64 = *var_k2q2s_ac_dn6_slot;
        let mut var_k2q2s_ac_dn7: f64 = *var_k2q2s_ac_dn7_slot;
        let mut var_k2q2s_ac_dn8: f64 = *var_k2q2s_ac_dn8_slot;
        let mut var_k2q2s_ac_dn9: f64 = *var_k2q2s_ac_dn9_slot;
        let mut var_keq_ac: f64 = *var_keq_ac_slot;
        let mut var_keq_ac_dn4: f64 = *var_keq_ac_dn4_slot;
        let mut var_keq_ac_dn6: f64 = *var_keq_ac_dn6_slot;
        let mut var_keq_ac_dn7: f64 = *var_keq_ac_dn7_slot;
        let mut var_keq_ac_dn8: f64 = *var_keq_ac_dn8_slot;
        let mut var_keq_ac_dn9: f64 = *var_keq_ac_dn9_slot;
        let mut var_prod1_ac: f64 = *var_prod1_ac_slot;
        let mut var_prod1_ac_dn4: f64 = *var_prod1_ac_dn4_slot;
        let mut var_prod1_ac_dn6: f64 = *var_prod1_ac_dn6_slot;
        let mut var_prod1_ac_dn7: f64 = *var_prod1_ac_dn7_slot;
        let mut var_prod1_ac_dn8: f64 = *var_prod1_ac_dn8_slot;
        let mut var_prod1_ac_dn9: f64 = *var_prod1_ac_dn9_slot;
        let mut var_prod2_ac: f64 = *var_prod2_ac_slot;
        let mut var_prod2_ac_dn4: f64 = *var_prod2_ac_dn4_slot;
        let mut var_prod2_ac_dn6: f64 = *var_prod2_ac_dn6_slot;
        let mut var_prod2_ac_dn7: f64 = *var_prod2_ac_dn7_slot;
        let mut var_prod2_ac_dn8: f64 = *var_prod2_ac_dn8_slot;
        let mut var_prod2_ac_dn9: f64 = *var_prod2_ac_dn9_slot;
        let mut var_qi1m_ac: f64 = *var_qi1m_ac_slot;
        let mut var_qi1m_ac_dn4: f64 = *var_qi1m_ac_dn4_slot;
        let mut var_qi1m_ac_dn6: f64 = *var_qi1m_ac_dn6_slot;
        let mut var_qi1m_ac_dn7: f64 = *var_qi1m_ac_dn7_slot;
        let mut var_qi1m_ac_dn8: f64 = *var_qi1m_ac_dn8_slot;
        let mut var_qi1m_ac_dn9: f64 = *var_qi1m_ac_dn9_slot;
        let mut var_qi2m_ac: f64 = *var_qi2m_ac_slot;
        let mut var_qi2m_ac_dn4: f64 = *var_qi2m_ac_dn4_slot;
        let mut var_qi2m_ac_dn6: f64 = *var_qi2m_ac_dn6_slot;
        let mut var_qi2m_ac_dn7: f64 = *var_qi2m_ac_dn7_slot;
        let mut var_qi2m_ac_dn8: f64 = *var_qi2m_ac_dn8_slot;
        let mut var_qi2m_ac_dn9: f64 = *var_qi2m_ac_dn9_slot;
        let mut var_qim_ac: f64 = *var_qim_ac_slot;
        let mut var_qim_ac_dn4: f64 = *var_qim_ac_dn4_slot;
        let mut var_qim_ac_dn6: f64 = *var_qim_ac_dn6_slot;
        let mut var_qim_ac_dn7: f64 = *var_qim_ac_dn7_slot;
        let mut var_qim_ac_dn8: f64 = *var_qim_ac_dn8_slot;
        let mut var_qim_ac_dn9: f64 = *var_qim_ac_dn9_slot;
        let mut var_qmfact1_ac: f64 = *var_qmfact1_ac_slot;
        let mut var_qmfact1_ac_dn4: f64 = *var_qmfact1_ac_dn4_slot;
        let mut var_qmfact1_ac_dn6: f64 = *var_qmfact1_ac_dn6_slot;
        let mut var_qmfact1_ac_dn7: f64 = *var_qmfact1_ac_dn7_slot;
        let mut var_qmfact1_ac_dn8: f64 = *var_qmfact1_ac_dn8_slot;
        let mut var_qmfact1_ac_dn9: f64 = *var_qmfact1_ac_dn9_slot;
        let mut var_qmfact2_ac: f64 = *var_qmfact2_ac_slot;
        let mut var_qmfact2_ac_dn4: f64 = *var_qmfact2_ac_dn4_slot;
        let mut var_qmfact2_ac_dn6: f64 = *var_qmfact2_ac_dn6_slot;
        let mut var_qmfact2_ac_dn7: f64 = *var_qmfact2_ac_dn7_slot;
        let mut var_qmfact2_ac_dn8: f64 = *var_qmfact2_ac_dn8_slot;
        let mut var_qmfact2_ac_dn9: f64 = *var_qmfact2_ac_dn9_slot;
        let mut var_ratio_pd_ac: f64 = *var_ratio_pd_ac_slot;
        let mut var_ratio_pd_ac_dn4: f64 = *var_ratio_pd_ac_dn4_slot;
        let mut var_ratio_pd_ac_dn6: f64 = *var_ratio_pd_ac_dn6_slot;
        let mut var_ratio_pd_ac_dn7: f64 = *var_ratio_pd_ac_dn7_slot;
        let mut var_ratio_pd_ac_dn8: f64 = *var_ratio_pd_ac_dn8_slot;
        let mut var_ratio_pd_ac_dn9: f64 = *var_ratio_pd_ac_dn9_slot;
        let mut var_sce1_ac: f64 = *var_sce1_ac_slot;
        let mut var_sce1_ac_dn4: f64 = *var_sce1_ac_dn4_slot;
        let mut var_sce1_ac_dn6: f64 = *var_sce1_ac_dn6_slot;
        let mut var_sce1_ac_dn7: f64 = *var_sce1_ac_dn7_slot;
        let mut var_sce1_ac_dn8: f64 = *var_sce1_ac_dn8_slot;
        let mut var_sce1_ac_dn9: f64 = *var_sce1_ac_dn9_slot;
        let mut var_sce2_ac: f64 = *var_sce2_ac_slot;
        let mut var_sce2_ac_dn4: f64 = *var_sce2_ac_dn4_slot;
        let mut var_sce2_ac_dn6: f64 = *var_sce2_ac_dn6_slot;
        let mut var_sce2_ac_dn7: f64 = *var_sce2_ac_dn7_slot;
        let mut var_sce2_ac_dn8: f64 = *var_sce2_ac_dn8_slot;
        let mut var_sce2_ac_dn9: f64 = *var_sce2_ac_dn9_slot;
        let mut var_xdriftd_ac: f64 = *var_xdriftd_ac_slot;
        let mut var_xdriftd_ac_dn4: f64 = *var_xdriftd_ac_dn4_slot;
        let mut var_xdriftd_ac_dn6: f64 = *var_xdriftd_ac_dn6_slot;
        let mut var_xdriftd_ac_dn7: f64 = *var_xdriftd_ac_dn7_slot;
        let mut var_xdriftd_ac_dn8: f64 = *var_xdriftd_ac_dn8_slot;
        let mut var_xdriftd_ac_dn9: f64 = *var_xdriftd_ac_dn9_slot;
        let mut var_xdrifts_ac: f64 = *var_xdrifts_ac_slot;
        let mut var_xdrifts_ac_dn4: f64 = *var_xdrifts_ac_dn4_slot;
        let mut var_xdrifts_ac_dn6: f64 = *var_xdrifts_ac_dn6_slot;
        let mut var_xdrifts_ac_dn7: f64 = *var_xdrifts_ac_dn7_slot;
        let mut var_xdrifts_ac_dn8: f64 = *var_xdrifts_ac_dn8_slot;
        let mut var_xdrifts_ac_dn9: f64 = *var_xdrifts_ac_dn9_slot;
        let mut var_xedge_ac: f64 = *var_xedge_ac_slot;
        let mut var_xedge_ac_dn4: f64 = *var_xedge_ac_dn4_slot;
        let mut var_xedge_ac_dn6: f64 = *var_xedge_ac_dn6_slot;
        let mut var_xedge_ac_dn7: f64 = *var_xedge_ac_dn7_slot;
        let mut var_xedge_ac_dn8: f64 = *var_xedge_ac_dn8_slot;
        let mut var_xedge_ac_dn9: f64 = *var_xedge_ac_dn9_slot;
        let mut var_xg20shift_ac: f64 = *var_xg20shift_ac_slot;
        let mut var_xg20shift_ac_dn4: f64 = *var_xg20shift_ac_dn4_slot;
        let mut var_xg20shift_ac_dn6: f64 = *var_xg20shift_ac_dn6_slot;
        let mut var_xg20shift_ac_dn7: f64 = *var_xg20shift_ac_dn7_slot;
        let mut var_xg20shift_ac_dn8: f64 = *var_xg20shift_ac_dn8_slot;
        let mut var_xg20shift_ac_dn9: f64 = *var_xg20shift_ac_dn9_slot;
        let mut var_xg2_ac: f64 = *var_xg2_ac_slot;
        let mut var_xg2_ac_dn4: f64 = *var_xg2_ac_dn4_slot;
        let mut var_xg2_ac_dn6: f64 = *var_xg2_ac_dn6_slot;
        let mut var_xg2_ac_dn7: f64 = *var_xg2_ac_dn7_slot;
        let mut var_xg2_ac_dn8: f64 = *var_xg2_ac_dn8_slot;
        let mut var_xg2_ac_dn9: f64 = *var_xg2_ac_dn9_slot;
        let mut var_xg2x_ac: f64 = *var_xg2x_ac_slot;
        let mut var_xg2x_ac_dn4: f64 = *var_xg2x_ac_dn4_slot;
        let mut var_xg2x_ac_dn6: f64 = *var_xg2x_ac_dn6_slot;
        let mut var_xg2x_ac_dn7: f64 = *var_xg2x_ac_dn7_slot;
        let mut var_xg2x_ac_dn8: f64 = *var_xg2x_ac_dn8_slot;
        let mut var_xg2x_ac_dn9: f64 = *var_xg2x_ac_dn9_slot;
        let mut var_zsat_ac: f64 = *var_zsat_ac_slot;
        let mut var_zsat_ac_dn4: f64 = *var_zsat_ac_dn4_slot;
        let mut var_zsat_ac_dn6: f64 = *var_zsat_ac_dn6_slot;
        let mut var_zsat_ac_dn7: f64 = *var_zsat_ac_dn7_slot;
        let mut var_zsat_ac_dn8: f64 = *var_zsat_ac_dn8_slot;
        let mut var_zsat_ac_dn9: f64 = *var_zsat_ac_dn9_slot;

        let (assign41590_e47393, assign41590_e47393_d_n4, assign41590_e47393_d_n6, assign41590_e47393_d_n7, assign41590_e47393_d_n8, assign41590_e47393_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_xg2x__blk931, var_xg2x__blk931_dn4, var_xg2x__blk931_dn6, var_xg2x__blk931_dn7, var_xg2x__blk931_dn8, var_xg2x__blk931_dn9,)
    } else {
        (var_xg2x_ac, var_xg2x_ac_dn4, var_xg2x_ac_dn6, var_xg2x_ac_dn7, var_xg2x_ac_dn8, var_xg2x_ac_dn9,)
    }
};
        var_xg2x_ac = assign41590_e47393;
        var_xg2x_ac_dn4 = assign41590_e47393_d_n4;
        var_xg2x_ac_dn6 = assign41590_e47393_d_n6;
        var_xg2x_ac_dn7 = assign41590_e47393_d_n7;
        var_xg2x_ac_dn8 = assign41590_e47393_d_n8;
        var_xg2x_ac_dn9 = assign41590_e47393_d_n9;

        let (assign41600_e47397, assign41600_e47397_d_n4, assign41600_e47397_d_n6, assign41600_e47397_d_n7, assign41600_e47397_d_n8, assign41600_e47397_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_k1__blk932, var_k1__blk932_dn4, var_k1__blk932_dn6, var_k1__blk932_dn7, var_k1__blk932_dn8, var_k1__blk932_dn9,)
    } else {
        (var_k1_ac, var_k1_ac_dn4, var_k1_ac_dn6, var_k1_ac_dn7, var_k1_ac_dn8, var_k1_ac_dn9,)
    }
};
        var_k1_ac = assign41600_e47397;
        var_k1_ac_dn4 = assign41600_e47397_d_n4;
        var_k1_ac_dn6 = assign41600_e47397_d_n6;
        var_k1_ac_dn7 = assign41600_e47397_d_n7;
        var_k1_ac_dn8 = assign41600_e47397_d_n8;
        var_k1_ac_dn9 = assign41600_e47397_d_n9;

        let (assign41610_e47401, assign41610_e47401_d_n4, assign41610_e47401_d_n6, assign41610_e47401_d_n7, assign41610_e47401_d_n8, assign41610_e47401_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_k2__blk933, var_k2__blk933_dn4, var_k2__blk933_dn6, var_k2__blk933_dn7, var_k2__blk933_dn8, var_k2__blk933_dn9,)
    } else {
        (var_k2_ac, var_k2_ac_dn4, var_k2_ac_dn6, var_k2_ac_dn7, var_k2_ac_dn8, var_k2_ac_dn9,)
    }
};
        var_k2_ac = assign41610_e47401;
        var_k2_ac_dn4 = assign41610_e47401_d_n4;
        var_k2_ac_dn6 = assign41610_e47401_d_n6;
        var_k2_ac_dn7 = assign41610_e47401_d_n7;
        var_k2_ac_dn8 = assign41610_e47401_d_n8;
        var_k2_ac_dn9 = assign41610_e47401_d_n9;

        let (assign41620_e47405, assign41620_e47405_d_n4, assign41620_e47405_d_n6, assign41620_e47405_d_n7, assign41620_e47405_d_n8, assign41620_e47405_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_k1q1s__blk939, var_k1q1s__blk939_dn4, var_k1q1s__blk939_dn6, var_k1q1s__blk939_dn7, var_k1q1s__blk939_dn8, var_k1q1s__blk939_dn9,)
    } else {
        (var_k1q1s_ac, var_k1q1s_ac_dn4, var_k1q1s_ac_dn6, var_k1q1s_ac_dn7, var_k1q1s_ac_dn8, var_k1q1s_ac_dn9,)
    }
};
        var_k1q1s_ac = assign41620_e47405;
        var_k1q1s_ac_dn4 = assign41620_e47405_d_n4;
        var_k1q1s_ac_dn6 = assign41620_e47405_d_n6;
        var_k1q1s_ac_dn7 = assign41620_e47405_d_n7;
        var_k1q1s_ac_dn8 = assign41620_e47405_d_n8;
        var_k1q1s_ac_dn9 = assign41620_e47405_d_n9;

        let (assign41630_e47409, assign41630_e47409_d_n4, assign41630_e47409_d_n6, assign41630_e47409_d_n7, assign41630_e47409_d_n8, assign41630_e47409_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_k2q2s__blk940, var_k2q2s__blk940_dn4, var_k2q2s__blk940_dn6, var_k2q2s__blk940_dn7, var_k2q2s__blk940_dn8, var_k2q2s__blk940_dn9,)
    } else {
        (var_k2q2s_ac, var_k2q2s_ac_dn4, var_k2q2s_ac_dn6, var_k2q2s_ac_dn7, var_k2q2s_ac_dn8, var_k2q2s_ac_dn9,)
    }
};
        var_k2q2s_ac = assign41630_e47409;
        var_k2q2s_ac_dn4 = assign41630_e47409_d_n4;
        var_k2q2s_ac_dn6 = assign41630_e47409_d_n6;
        var_k2q2s_ac_dn7 = assign41630_e47409_d_n7;
        var_k2q2s_ac_dn8 = assign41630_e47409_d_n8;
        var_k2q2s_ac_dn9 = assign41630_e47409_d_n9;

        let (assign41640_e47413, assign41640_e47413_d_n4, assign41640_e47413_d_n6, assign41640_e47413_d_n7, assign41640_e47413_d_n8, assign41640_e47413_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_xdrifts__blk951, var_xdrifts__blk951_dn4, var_xdrifts__blk951_dn6, var_xdrifts__blk951_dn7, var_xdrifts__blk951_dn8, var_xdrifts__blk951_dn9,)
    } else {
        (var_xdrifts_ac, var_xdrifts_ac_dn4, var_xdrifts_ac_dn6, var_xdrifts_ac_dn7, var_xdrifts_ac_dn8, var_xdrifts_ac_dn9,)
    }
};
        var_xdrifts_ac = assign41640_e47413;
        var_xdrifts_ac_dn4 = assign41640_e47413_d_n4;
        var_xdrifts_ac_dn6 = assign41640_e47413_d_n6;
        var_xdrifts_ac_dn7 = assign41640_e47413_d_n7;
        var_xdrifts_ac_dn8 = assign41640_e47413_d_n8;
        var_xdrifts_ac_dn9 = assign41640_e47413_d_n9;

        let (assign41650_e47417, assign41650_e47417_d_n4, assign41650_e47417_d_n6, assign41650_e47417_d_n7, assign41650_e47417_d_n8, assign41650_e47417_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_k1q1d__blk1004, var_k1q1d__blk1004_dn4, var_k1q1d__blk1004_dn6, var_k1q1d__blk1004_dn7, var_k1q1d__blk1004_dn8, var_k1q1d__blk1004_dn9,)
    } else {
        (var_k1q1d_ac, var_k1q1d_ac_dn4, var_k1q1d_ac_dn6, var_k1q1d_ac_dn7, var_k1q1d_ac_dn8, var_k1q1d_ac_dn9,)
    }
};
        var_k1q1d_ac = assign41650_e47417;
        var_k1q1d_ac_dn4 = assign41650_e47417_d_n4;
        var_k1q1d_ac_dn6 = assign41650_e47417_d_n6;
        var_k1q1d_ac_dn7 = assign41650_e47417_d_n7;
        var_k1q1d_ac_dn8 = assign41650_e47417_d_n8;
        var_k1q1d_ac_dn9 = assign41650_e47417_d_n9;

        let (assign41660_e47421, assign41660_e47421_d_n4, assign41660_e47421_d_n6, assign41660_e47421_d_n7, assign41660_e47421_d_n8, assign41660_e47421_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_k2q2d__blk1005, var_k2q2d__blk1005_dn4, var_k2q2d__blk1005_dn6, var_k2q2d__blk1005_dn7, var_k2q2d__blk1005_dn8, var_k2q2d__blk1005_dn9,)
    } else {
        (var_k2q2d_ac, var_k2q2d_ac_dn4, var_k2q2d_ac_dn6, var_k2q2d_ac_dn7, var_k2q2d_ac_dn8, var_k2q2d_ac_dn9,)
    }
};
        var_k2q2d_ac = assign41660_e47421;
        var_k2q2d_ac_dn4 = assign41660_e47421_d_n4;
        var_k2q2d_ac_dn6 = assign41660_e47421_d_n6;
        var_k2q2d_ac_dn7 = assign41660_e47421_d_n7;
        var_k2q2d_ac_dn8 = assign41660_e47421_d_n8;
        var_k2q2d_ac_dn9 = assign41660_e47421_d_n9;

        let (assign41670_e47425, assign41670_e47425_d_n4, assign41670_e47425_d_n6, assign41670_e47425_d_n7, assign41670_e47425_d_n8, assign41670_e47425_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_xdriftd__blk1015, var_xdriftd__blk1015_dn4, var_xdriftd__blk1015_dn6, var_xdriftd__blk1015_dn7, var_xdriftd__blk1015_dn8, var_xdriftd__blk1015_dn9,)
    } else {
        (var_xdriftd_ac, var_xdriftd_ac_dn4, var_xdriftd_ac_dn6, var_xdriftd_ac_dn7, var_xdriftd_ac_dn8, var_xdriftd_ac_dn9,)
    }
};
        var_xdriftd_ac = assign41670_e47425;
        var_xdriftd_ac_dn4 = assign41670_e47425_d_n4;
        var_xdriftd_ac_dn6 = assign41670_e47425_d_n6;
        var_xdriftd_ac_dn7 = assign41670_e47425_d_n7;
        var_xdriftd_ac_dn8 = assign41670_e47425_d_n8;
        var_xdriftd_ac_dn9 = assign41670_e47425_d_n9;

        let (assign41680_e47429, assign41680_e47429_d_n4, assign41680_e47429_d_n6, assign41680_e47429_d_n7, assign41680_e47429_d_n8, assign41680_e47429_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_qim__blk1016, var_qim__blk1016_dn4, var_qim__blk1016_dn6, var_qim__blk1016_dn7, var_qim__blk1016_dn8, var_qim__blk1016_dn9,)
    } else {
        (var_qim_ac, var_qim_ac_dn4, var_qim_ac_dn6, var_qim_ac_dn7, var_qim_ac_dn8, var_qim_ac_dn9,)
    }
};
        var_qim_ac = assign41680_e47429;
        var_qim_ac_dn4 = assign41680_e47429_d_n4;
        var_qim_ac_dn6 = assign41680_e47429_d_n6;
        var_qim_ac_dn7 = assign41680_e47429_d_n7;
        var_qim_ac_dn8 = assign41680_e47429_d_n8;
        var_qim_ac_dn9 = assign41680_e47429_d_n9;

        let (assign41690_e47433, assign41690_e47433_d_n4, assign41690_e47433_d_n6, assign41690_e47433_d_n7, assign41690_e47433_d_n8, assign41690_e47433_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_ratio_pd__blk1020, var_ratio_pd__blk1020_dn4, var_ratio_pd__blk1020_dn6, var_ratio_pd__blk1020_dn7, var_ratio_pd__blk1020_dn8, var_ratio_pd__blk1020_dn9,)
    } else {
        (var_ratio_pd_ac, var_ratio_pd_ac_dn4, var_ratio_pd_ac_dn6, var_ratio_pd_ac_dn7, var_ratio_pd_ac_dn8, var_ratio_pd_ac_dn9,)
    }
};
        var_ratio_pd_ac = assign41690_e47433;
        var_ratio_pd_ac_dn4 = assign41690_e47433_d_n4;
        var_ratio_pd_ac_dn6 = assign41690_e47433_d_n6;
        var_ratio_pd_ac_dn7 = assign41690_e47433_d_n7;
        var_ratio_pd_ac_dn8 = assign41690_e47433_d_n8;
        var_ratio_pd_ac_dn9 = assign41690_e47433_d_n9;

        let (assign41700_e47437, assign41700_e47437_d_n4, assign41700_e47437_d_n6, assign41700_e47437_d_n7, assign41700_e47437_d_n8, assign41700_e47437_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_qi1m__blk1029, var_qi1m__blk1029_dn4, var_qi1m__blk1029_dn6, var_qi1m__blk1029_dn7, var_qi1m__blk1029_dn8, var_qi1m__blk1029_dn9,)
    } else {
        (var_qi1m_ac, var_qi1m_ac_dn4, var_qi1m_ac_dn6, var_qi1m_ac_dn7, var_qi1m_ac_dn8, var_qi1m_ac_dn9,)
    }
};
        var_qi1m_ac = assign41700_e47437;
        var_qi1m_ac_dn4 = assign41700_e47437_d_n4;
        var_qi1m_ac_dn6 = assign41700_e47437_d_n6;
        var_qi1m_ac_dn7 = assign41700_e47437_d_n7;
        var_qi1m_ac_dn8 = assign41700_e47437_d_n8;
        var_qi1m_ac_dn9 = assign41700_e47437_d_n9;

        let (assign41710_e47441, assign41710_e47441_d_n4, assign41710_e47441_d_n6, assign41710_e47441_d_n7, assign41710_e47441_d_n8, assign41710_e47441_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_qi2m__blk1030, var_qi2m__blk1030_dn4, var_qi2m__blk1030_dn6, var_qi2m__blk1030_dn7, var_qi2m__blk1030_dn8, var_qi2m__blk1030_dn9,)
    } else {
        (var_qi2m_ac, var_qi2m_ac_dn4, var_qi2m_ac_dn6, var_qi2m_ac_dn7, var_qi2m_ac_dn8, var_qi2m_ac_dn9,)
    }
};
        var_qi2m_ac = assign41710_e47441;
        var_qi2m_ac_dn4 = assign41710_e47441_d_n4;
        var_qi2m_ac_dn6 = assign41710_e47441_d_n6;
        var_qi2m_ac_dn7 = assign41710_e47441_d_n7;
        var_qi2m_ac_dn8 = assign41710_e47441_d_n8;
        var_qi2m_ac_dn9 = assign41710_e47441_d_n9;

        let (assign41720_e47445, assign41720_e47445_d_n4, assign41720_e47445_d_n6, assign41720_e47445_d_n7, assign41720_e47445_d_n8, assign41720_e47445_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_zsat__blk1051, var_zsat__blk1051_dn4, var_zsat__blk1051_dn6, var_zsat__blk1051_dn7, var_zsat__blk1051_dn8, var_zsat__blk1051_dn9,)
    } else {
        (var_zsat_ac, var_zsat_ac_dn4, var_zsat_ac_dn6, var_zsat_ac_dn7, var_zsat_ac_dn8, var_zsat_ac_dn9,)
    }
};
        var_zsat_ac = assign41720_e47445;
        var_zsat_ac_dn4 = assign41720_e47445_d_n4;
        var_zsat_ac_dn6 = assign41720_e47445_d_n6;
        var_zsat_ac_dn7 = assign41720_e47445_d_n7;
        var_zsat_ac_dn8 = assign41720_e47445_d_n8;
        var_zsat_ac_dn9 = assign41720_e47445_d_n9;

        let (assign41730_e47449, assign41730_e47449_d_n4, assign41730_e47449_d_n6, assign41730_e47449_d_n7, assign41730_e47449_d_n8, assign41730_e47449_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_qmfact1__blk1054, var_qmfact1__blk1054_dn4, var_qmfact1__blk1054_dn6, var_qmfact1__blk1054_dn7, var_qmfact1__blk1054_dn8, var_qmfact1__blk1054_dn9,)
    } else {
        (var_qmfact1_ac, var_qmfact1_ac_dn4, var_qmfact1_ac_dn6, var_qmfact1_ac_dn7, var_qmfact1_ac_dn8, var_qmfact1_ac_dn9,)
    }
};
        var_qmfact1_ac = assign41730_e47449;
        var_qmfact1_ac_dn4 = assign41730_e47449_d_n4;
        var_qmfact1_ac_dn6 = assign41730_e47449_d_n6;
        var_qmfact1_ac_dn7 = assign41730_e47449_d_n7;
        var_qmfact1_ac_dn8 = assign41730_e47449_d_n8;
        var_qmfact1_ac_dn9 = assign41730_e47449_d_n9;

        let (assign41740_e47453, assign41740_e47453_d_n4, assign41740_e47453_d_n6, assign41740_e47453_d_n7, assign41740_e47453_d_n8, assign41740_e47453_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_qmfact2__blk1055, var_qmfact2__blk1055_dn4, var_qmfact2__blk1055_dn6, var_qmfact2__blk1055_dn7, var_qmfact2__blk1055_dn8, var_qmfact2__blk1055_dn9,)
    } else {
        (var_qmfact2_ac, var_qmfact2_ac_dn4, var_qmfact2_ac_dn6, var_qmfact2_ac_dn7, var_qmfact2_ac_dn8, var_qmfact2_ac_dn9,)
    }
};
        var_qmfact2_ac = assign41740_e47453;
        var_qmfact2_ac_dn4 = assign41740_e47453_d_n4;
        var_qmfact2_ac_dn6 = assign41740_e47453_d_n6;
        var_qmfact2_ac_dn7 = assign41740_e47453_d_n7;
        var_qmfact2_ac_dn8 = assign41740_e47453_d_n8;
        var_qmfact2_ac_dn9 = assign41740_e47453_d_n9;

        let (assign41750_e47457, assign41750_e47457_d_n4, assign41750_e47457_d_n6, assign41750_e47457_d_n7, assign41750_e47457_d_n8, assign41750_e47457_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_delta_k1q1__blk1076, var_delta_k1q1__blk1076_dn4, var_delta_k1q1__blk1076_dn6, var_delta_k1q1__blk1076_dn7, var_delta_k1q1__blk1076_dn8, var_delta_k1q1__blk1076_dn9,)
    } else {
        (var_delta_k1q1_ac, var_delta_k1q1_ac_dn4, var_delta_k1q1_ac_dn6, var_delta_k1q1_ac_dn7, var_delta_k1q1_ac_dn8, var_delta_k1q1_ac_dn9,)
    }
};
        var_delta_k1q1_ac = assign41750_e47457;
        var_delta_k1q1_ac_dn4 = assign41750_e47457_d_n4;
        var_delta_k1q1_ac_dn6 = assign41750_e47457_d_n6;
        var_delta_k1q1_ac_dn7 = assign41750_e47457_d_n7;
        var_delta_k1q1_ac_dn8 = assign41750_e47457_d_n8;
        var_delta_k1q1_ac_dn9 = assign41750_e47457_d_n9;

        let (assign41760_e47461, assign41760_e47461_d_n4, assign41760_e47461_d_n6, assign41760_e47461_d_n7, assign41760_e47461_d_n8, assign41760_e47461_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_delta_k2q2__blk1077, var_delta_k2q2__blk1077_dn4, var_delta_k2q2__blk1077_dn6, var_delta_k2q2__blk1077_dn7, var_delta_k2q2__blk1077_dn8, var_delta_k2q2__blk1077_dn9,)
    } else {
        (var_delta_k2q2_ac, var_delta_k2q2_ac_dn4, var_delta_k2q2_ac_dn6, var_delta_k2q2_ac_dn7, var_delta_k2q2_ac_dn8, var_delta_k2q2_ac_dn9,)
    }
};
        var_delta_k2q2_ac = assign41760_e47461;
        var_delta_k2q2_ac_dn4 = assign41760_e47461_d_n4;
        var_delta_k2q2_ac_dn6 = assign41760_e47461_d_n6;
        var_delta_k2q2_ac_dn7 = assign41760_e47461_d_n7;
        var_delta_k2q2_ac_dn8 = assign41760_e47461_d_n8;
        var_delta_k2q2_ac_dn9 = assign41760_e47461_d_n9;

        let (assign41770_e47465, assign41770_e47465_d_n4, assign41770_e47465_d_n6, assign41770_e47465_d_n7, assign41770_e47465_d_n8, assign41770_e47465_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_prod1__blk1078, var_prod1__blk1078_dn4, var_prod1__blk1078_dn6, var_prod1__blk1078_dn7, var_prod1__blk1078_dn8, var_prod1__blk1078_dn9,)
    } else {
        (var_prod1_ac, var_prod1_ac_dn4, var_prod1_ac_dn6, var_prod1_ac_dn7, var_prod1_ac_dn8, var_prod1_ac_dn9,)
    }
};
        var_prod1_ac = assign41770_e47465;
        var_prod1_ac_dn4 = assign41770_e47465_d_n4;
        var_prod1_ac_dn6 = assign41770_e47465_d_n6;
        var_prod1_ac_dn7 = assign41770_e47465_d_n7;
        var_prod1_ac_dn8 = assign41770_e47465_d_n8;
        var_prod1_ac_dn9 = assign41770_e47465_d_n9;

        let (assign41780_e47469, assign41780_e47469_d_n4, assign41780_e47469_d_n6, assign41780_e47469_d_n7, assign41780_e47469_d_n8, assign41780_e47469_d_n9,) = {
    if (var_guard1080 != 0.0) {
        (var_prod2__blk1079, var_prod2__blk1079_dn4, var_prod2__blk1079_dn6, var_prod2__blk1079_dn7, var_prod2__blk1079_dn8, var_prod2__blk1079_dn9,)
    } else {
        (var_prod2_ac, var_prod2_ac_dn4, var_prod2_ac_dn6, var_prod2_ac_dn7, var_prod2_ac_dn8, var_prod2_ac_dn9,)
    }
};
        var_prod2_ac = assign41780_e47469;
        var_prod2_ac_dn4 = assign41780_e47469_d_n4;
        var_prod2_ac_dn6 = assign41780_e47469_d_n6;
        var_prod2_ac_dn7 = assign41780_e47469_d_n7;
        var_prod2_ac_dn8 = assign41780_e47469_d_n8;
        var_prod2_ac_dn9 = assign41780_e47469_d_n9;

        let (assign41790_e47474, assign41790_e47474_d_n4, assign41790_e47474_d_n6, assign41790_e47474_d_n7, assign41790_e47474_d_n8, assign41790_e47474_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_xg20shift_dc, var_xg20shift_dc_dn4, var_xg20shift_dc_dn6, var_xg20shift_dc_dn7, var_xg20shift_dc_dn8, var_xg20shift_dc_dn9,)
    } else {
        (var_xg20shift_ac, var_xg20shift_ac_dn4, var_xg20shift_ac_dn6, var_xg20shift_ac_dn7, var_xg20shift_ac_dn8, var_xg20shift_ac_dn9,)
    }
};
        var_xg20shift_ac = assign41790_e47474;
        var_xg20shift_ac_dn4 = assign41790_e47474_d_n4;
        var_xg20shift_ac_dn6 = assign41790_e47474_d_n6;
        var_xg20shift_ac_dn7 = assign41790_e47474_d_n7;
        var_xg20shift_ac_dn8 = assign41790_e47474_d_n8;
        var_xg20shift_ac_dn9 = assign41790_e47474_d_n9;

        let (assign41800_e47479, assign41800_e47479_d_n4, assign41800_e47479_d_n6, assign41800_e47479_d_n7, assign41800_e47479_d_n8, assign41800_e47479_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_diff_min_dc, var_diff_min_dc_dn4, var_diff_min_dc_dn6, var_diff_min_dc_dn7, var_diff_min_dc_dn8, var_diff_min_dc_dn9,)
    } else {
        (var_diff_min_ac, var_diff_min_ac_dn4, var_diff_min_ac_dn6, var_diff_min_ac_dn7, var_diff_min_ac_dn8, var_diff_min_ac_dn9,)
    }
};
        var_diff_min_ac = assign41800_e47479;
        var_diff_min_ac_dn4 = assign41800_e47479_d_n4;
        var_diff_min_ac_dn6 = assign41800_e47479_d_n6;
        var_diff_min_ac_dn7 = assign41800_e47479_d_n7;
        var_diff_min_ac_dn8 = assign41800_e47479_d_n8;
        var_diff_min_ac_dn9 = assign41800_e47479_d_n9;

        let (assign41810_e47484, assign41810_e47484_d_n4, assign41810_e47484_d_n6, assign41810_e47484_d_n7, assign41810_e47484_d_n8, assign41810_e47484_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_a0_dc, var_a0_dc_dn4, var_a0_dc_dn6, var_a0_dc_dn7, var_a0_dc_dn8, var_a0_dc_dn9,)
    } else {
        (var_a0_ac, var_a0_ac_dn4, var_a0_ac_dn6, var_a0_ac_dn7, var_a0_ac_dn8, var_a0_ac_dn9,)
    }
};
        var_a0_ac = assign41810_e47484;
        var_a0_ac_dn4 = assign41810_e47484_d_n4;
        var_a0_ac_dn6 = assign41810_e47484_d_n6;
        var_a0_ac_dn7 = assign41810_e47484_d_n7;
        var_a0_ac_dn8 = assign41810_e47484_d_n8;
        var_a0_ac_dn9 = assign41810_e47484_d_n9;

        let (assign41820_e47489, assign41820_e47489_d_n4, assign41820_e47489_d_n6, assign41820_e47489_d_n7, assign41820_e47489_d_n8, assign41820_e47489_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_inv_k1_dc, var_inv_k1_dc_dn4, var_inv_k1_dc_dn6, var_inv_k1_dc_dn7, var_inv_k1_dc_dn8, var_inv_k1_dc_dn9,)
    } else {
        (var_inv_k1_ac, var_inv_k1_ac_dn4, var_inv_k1_ac_dn6, var_inv_k1_ac_dn7, var_inv_k1_ac_dn8, var_inv_k1_ac_dn9,)
    }
};
        var_inv_k1_ac = assign41820_e47489;
        var_inv_k1_ac_dn4 = assign41820_e47489_d_n4;
        var_inv_k1_ac_dn6 = assign41820_e47489_d_n6;
        var_inv_k1_ac_dn7 = assign41820_e47489_d_n7;
        var_inv_k1_ac_dn8 = assign41820_e47489_d_n8;
        var_inv_k1_ac_dn9 = assign41820_e47489_d_n9;

        let (assign41830_e47494, assign41830_e47494_d_n4, assign41830_e47494_d_n6, assign41830_e47494_d_n7, assign41830_e47494_d_n8, assign41830_e47494_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_inv_k2_dc, var_inv_k2_dc_dn4, var_inv_k2_dc_dn6, var_inv_k2_dc_dn7, var_inv_k2_dc_dn8, var_inv_k2_dc_dn9,)
    } else {
        (var_inv_k2_ac, var_inv_k2_ac_dn4, var_inv_k2_ac_dn6, var_inv_k2_ac_dn7, var_inv_k2_ac_dn8, var_inv_k2_ac_dn9,)
    }
};
        var_inv_k2_ac = assign41830_e47494;
        var_inv_k2_ac_dn4 = assign41830_e47494_d_n4;
        var_inv_k2_ac_dn6 = assign41830_e47494_d_n6;
        var_inv_k2_ac_dn7 = assign41830_e47494_d_n7;
        var_inv_k2_ac_dn8 = assign41830_e47494_d_n8;
        var_inv_k2_ac_dn9 = assign41830_e47494_d_n9;

        let (assign41840_e47499, assign41840_e47499_d_n4, assign41840_e47499_d_n6, assign41840_e47499_d_n7, assign41840_e47499_d_n8, assign41840_e47499_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_keq_dc, var_keq_dc_dn4, var_keq_dc_dn6, var_keq_dc_dn7, var_keq_dc_dn8, var_keq_dc_dn9,)
    } else {
        (var_keq_ac, var_keq_ac_dn4, var_keq_ac_dn6, var_keq_ac_dn7, var_keq_ac_dn8, var_keq_ac_dn9,)
    }
};
        var_keq_ac = assign41840_e47499;
        var_keq_ac_dn4 = assign41840_e47499_d_n4;
        var_keq_ac_dn6 = assign41840_e47499_d_n6;
        var_keq_ac_dn7 = assign41840_e47499_d_n7;
        var_keq_ac_dn8 = assign41840_e47499_d_n8;
        var_keq_ac_dn9 = assign41840_e47499_d_n9;

        let (assign41850_e47504, assign41850_e47504_d_n4, assign41850_e47504_d_n6, assign41850_e47504_d_n7, assign41850_e47504_d_n8, assign41850_e47504_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_dx_wi_dc, var_dx_wi_dc_dn4, var_dx_wi_dc_dn6, var_dx_wi_dc_dn7, var_dx_wi_dc_dn8, var_dx_wi_dc_dn9,)
    } else {
        (var_dx_wi_ac, var_dx_wi_ac_dn4, var_dx_wi_ac_dn6, var_dx_wi_ac_dn7, var_dx_wi_ac_dn8, var_dx_wi_ac_dn9,)
    }
};
        var_dx_wi_ac = assign41850_e47504;
        var_dx_wi_ac_dn4 = assign41850_e47504_d_n4;
        var_dx_wi_ac_dn6 = assign41850_e47504_d_n6;
        var_dx_wi_ac_dn7 = assign41850_e47504_d_n7;
        var_dx_wi_ac_dn8 = assign41850_e47504_d_n8;
        var_dx_wi_ac_dn9 = assign41850_e47504_d_n9;

        let (assign41860_e47509, assign41860_e47509_d_n4, assign41860_e47509_d_n6, assign41860_e47509_d_n7, assign41860_e47509_d_n8, assign41860_e47509_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_csiprime_dc, var_csiprime_dc_dn4, var_csiprime_dc_dn6, var_csiprime_dc_dn7, var_csiprime_dc_dn8, var_csiprime_dc_dn9,)
    } else {
        (var_csiprime_ac, var_csiprime_ac_dn4, var_csiprime_ac_dn6, var_csiprime_ac_dn7, var_csiprime_ac_dn8, var_csiprime_ac_dn9,)
    }
};
        var_csiprime_ac = assign41860_e47509;
        var_csiprime_ac_dn4 = assign41860_e47509_d_n4;
        var_csiprime_ac_dn6 = assign41860_e47509_d_n6;
        var_csiprime_ac_dn7 = assign41860_e47509_d_n7;
        var_csiprime_ac_dn8 = assign41860_e47509_d_n8;
        var_csiprime_ac_dn9 = assign41860_e47509_d_n9;

        let (assign41870_e47514, assign41870_e47514_d_n4, assign41870_e47514_d_n6, assign41870_e47514_d_n7, assign41870_e47514_d_n8, assign41870_e47514_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_dx_wi_1d_dc, var_dx_wi_1d_dc_dn4, var_dx_wi_1d_dc_dn6, var_dx_wi_1d_dc_dn7, var_dx_wi_1d_dc_dn8, var_dx_wi_1d_dc_dn9,)
    } else {
        (var_dx_wi_1d_ac, var_dx_wi_1d_ac_dn4, var_dx_wi_1d_ac_dn6, var_dx_wi_1d_ac_dn7, var_dx_wi_1d_ac_dn8, var_dx_wi_1d_ac_dn9,)
    }
};
        var_dx_wi_1d_ac = assign41870_e47514;
        var_dx_wi_1d_ac_dn4 = assign41870_e47514_d_n4;
        var_dx_wi_1d_ac_dn6 = assign41870_e47514_d_n6;
        var_dx_wi_1d_ac_dn7 = assign41870_e47514_d_n7;
        var_dx_wi_1d_ac_dn8 = assign41870_e47514_d_n8;
        var_dx_wi_1d_ac_dn9 = assign41870_e47514_d_n9;

        let (assign41880_e47519, assign41880_e47519_d_n4, assign41880_e47519_d_n6, assign41880_e47519_d_n7, assign41880_e47519_d_n8, assign41880_e47519_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_dleff_dc, var_dleff_dc_dn4, var_dleff_dc_dn6, var_dleff_dc_dn7, var_dleff_dc_dn8, var_dleff_dc_dn9,)
    } else {
        (var_dleff_ac, var_dleff_ac_dn4, var_dleff_ac_dn6, var_dleff_ac_dn7, var_dleff_ac_dn8, var_dleff_ac_dn9,)
    }
};
        var_dleff_ac = assign41880_e47519;
        var_dleff_ac_dn4 = assign41880_e47519_d_n4;
        var_dleff_ac_dn6 = assign41880_e47519_d_n6;
        var_dleff_ac_dn7 = assign41880_e47519_d_n7;
        var_dleff_ac_dn8 = assign41880_e47519_d_n8;
        var_dleff_ac_dn9 = assign41880_e47519_d_n9;

        let (assign41890_e47524, assign41890_e47524_d_n4, assign41890_e47524_d_n6, assign41890_e47524_d_n7, assign41890_e47524_d_n8, assign41890_e47524_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_xedge_dc, var_xedge_dc_dn4, var_xedge_dc_dn6, var_xedge_dc_dn7, var_xedge_dc_dn8, var_xedge_dc_dn9,)
    } else {
        (var_xedge_ac, var_xedge_ac_dn4, var_xedge_ac_dn6, var_xedge_ac_dn7, var_xedge_ac_dn8, var_xedge_ac_dn9,)
    }
};
        var_xedge_ac = assign41890_e47524;
        var_xedge_ac_dn4 = assign41890_e47524_d_n4;
        var_xedge_ac_dn6 = assign41890_e47524_d_n6;
        var_xedge_ac_dn7 = assign41890_e47524_d_n7;
        var_xedge_ac_dn8 = assign41890_e47524_d_n8;
        var_xedge_ac_dn9 = assign41890_e47524_d_n9;

        let (assign41900_e47529, assign41900_e47529_d_n4, assign41900_e47529_d_n6, assign41900_e47529_d_n7, assign41900_e47529_d_n8, assign41900_e47529_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_sce1_dc, var_sce1_dc_dn4, var_sce1_dc_dn6, var_sce1_dc_dn7, var_sce1_dc_dn8, var_sce1_dc_dn9,)
    } else {
        (var_sce1_ac, var_sce1_ac_dn4, var_sce1_ac_dn6, var_sce1_ac_dn7, var_sce1_ac_dn8, var_sce1_ac_dn9,)
    }
};
        var_sce1_ac = assign41900_e47529;
        var_sce1_ac_dn4 = assign41900_e47529_d_n4;
        var_sce1_ac_dn6 = assign41900_e47529_d_n6;
        var_sce1_ac_dn7 = assign41900_e47529_d_n7;
        var_sce1_ac_dn8 = assign41900_e47529_d_n8;
        var_sce1_ac_dn9 = assign41900_e47529_d_n9;

        let (assign41910_e47534, assign41910_e47534_d_n4, assign41910_e47534_d_n6, assign41910_e47534_d_n7, assign41910_e47534_d_n8, assign41910_e47534_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_sce2_dc, var_sce2_dc_dn4, var_sce2_dc_dn6, var_sce2_dc_dn7, var_sce2_dc_dn8, var_sce2_dc_dn9,)
    } else {
        (var_sce2_ac, var_sce2_ac_dn4, var_sce2_ac_dn6, var_sce2_ac_dn7, var_sce2_ac_dn8, var_sce2_ac_dn9,)
    }
};
        var_sce2_ac = assign41910_e47534;
        var_sce2_ac_dn4 = assign41910_e47534_d_n4;
        var_sce2_ac_dn6 = assign41910_e47534_d_n6;
        var_sce2_ac_dn7 = assign41910_e47534_d_n7;
        var_sce2_ac_dn8 = assign41910_e47534_d_n8;
        var_sce2_ac_dn9 = assign41910_e47534_d_n9;

        let (assign41920_e47539, assign41920_e47539_d_n4, assign41920_e47539_d_n6, assign41920_e47539_d_n7, assign41920_e47539_d_n8, assign41920_e47539_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_dxg1_dibl_dc, var_dxg1_dibl_dc_dn4, var_dxg1_dibl_dc_dn6, var_dxg1_dibl_dc_dn7, var_dxg1_dibl_dc_dn8, var_dxg1_dibl_dc_dn9,)
    } else {
        (var_dxg1_dibl_ac, var_dxg1_dibl_ac_dn4, var_dxg1_dibl_ac_dn6, var_dxg1_dibl_ac_dn7, var_dxg1_dibl_ac_dn8, var_dxg1_dibl_ac_dn9,)
    }
};
        var_dxg1_dibl_ac = assign41920_e47539;
        var_dxg1_dibl_ac_dn4 = assign41920_e47539_d_n4;
        var_dxg1_dibl_ac_dn6 = assign41920_e47539_d_n6;
        var_dxg1_dibl_ac_dn7 = assign41920_e47539_d_n7;
        var_dxg1_dibl_ac_dn8 = assign41920_e47539_d_n8;
        var_dxg1_dibl_ac_dn9 = assign41920_e47539_d_n9;

        let (assign41930_e47544, assign41930_e47544_d_n4, assign41930_e47544_d_n6, assign41930_e47544_d_n7, assign41930_e47544_d_n8, assign41930_e47544_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_xg2_dc, var_xg2_dc_dn4, var_xg2_dc_dn6, var_xg2_dc_dn7, var_xg2_dc_dn8, var_xg2_dc_dn9,)
    } else {
        (var_xg2_ac, var_xg2_ac_dn4, var_xg2_ac_dn6, var_xg2_ac_dn7, var_xg2_ac_dn8, var_xg2_ac_dn9,)
    }
};
        var_xg2_ac = assign41930_e47544;
        var_xg2_ac_dn4 = assign41930_e47544_d_n4;
        var_xg2_ac_dn6 = assign41930_e47544_d_n6;
        var_xg2_ac_dn7 = assign41930_e47544_d_n7;
        var_xg2_ac_dn8 = assign41930_e47544_d_n8;
        var_xg2_ac_dn9 = assign41930_e47544_d_n9;

        let (assign41940_e47549, assign41940_e47549_d_n4, assign41940_e47549_d_n6, assign41940_e47549_d_n7, assign41940_e47549_d_n8, assign41940_e47549_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_xg2x_dc, var_xg2x_dc_dn4, var_xg2x_dc_dn6, var_xg2x_dc_dn7, var_xg2x_dc_dn8, var_xg2x_dc_dn9,)
    } else {
        (var_xg2x_ac, var_xg2x_ac_dn4, var_xg2x_ac_dn6, var_xg2x_ac_dn7, var_xg2x_ac_dn8, var_xg2x_ac_dn9,)
    }
};
        var_xg2x_ac = assign41940_e47549;
        var_xg2x_ac_dn4 = assign41940_e47549_d_n4;
        var_xg2x_ac_dn6 = assign41940_e47549_d_n6;
        var_xg2x_ac_dn7 = assign41940_e47549_d_n7;
        var_xg2x_ac_dn8 = assign41940_e47549_d_n8;
        var_xg2x_ac_dn9 = assign41940_e47549_d_n9;

        *var_a0_ac_slot = var_a0_ac;
        *var_a0_ac_dn4_slot = var_a0_ac_dn4;
        *var_a0_ac_dn6_slot = var_a0_ac_dn6;
        *var_a0_ac_dn7_slot = var_a0_ac_dn7;
        *var_a0_ac_dn8_slot = var_a0_ac_dn8;
        *var_a0_ac_dn9_slot = var_a0_ac_dn9;
        *var_csiprime_ac_slot = var_csiprime_ac;
        *var_csiprime_ac_dn4_slot = var_csiprime_ac_dn4;
        *var_csiprime_ac_dn6_slot = var_csiprime_ac_dn6;
        *var_csiprime_ac_dn7_slot = var_csiprime_ac_dn7;
        *var_csiprime_ac_dn8_slot = var_csiprime_ac_dn8;
        *var_csiprime_ac_dn9_slot = var_csiprime_ac_dn9;
        *var_delta_k1q1_ac_slot = var_delta_k1q1_ac;
        *var_delta_k1q1_ac_dn4_slot = var_delta_k1q1_ac_dn4;
        *var_delta_k1q1_ac_dn6_slot = var_delta_k1q1_ac_dn6;
        *var_delta_k1q1_ac_dn7_slot = var_delta_k1q1_ac_dn7;
        *var_delta_k1q1_ac_dn8_slot = var_delta_k1q1_ac_dn8;
        *var_delta_k1q1_ac_dn9_slot = var_delta_k1q1_ac_dn9;
        *var_delta_k2q2_ac_slot = var_delta_k2q2_ac;
        *var_delta_k2q2_ac_dn4_slot = var_delta_k2q2_ac_dn4;
        *var_delta_k2q2_ac_dn6_slot = var_delta_k2q2_ac_dn6;
        *var_delta_k2q2_ac_dn7_slot = var_delta_k2q2_ac_dn7;
        *var_delta_k2q2_ac_dn8_slot = var_delta_k2q2_ac_dn8;
        *var_delta_k2q2_ac_dn9_slot = var_delta_k2q2_ac_dn9;
        *var_diff_min_ac_slot = var_diff_min_ac;
        *var_diff_min_ac_dn4_slot = var_diff_min_ac_dn4;
        *var_diff_min_ac_dn6_slot = var_diff_min_ac_dn6;
        *var_diff_min_ac_dn7_slot = var_diff_min_ac_dn7;
        *var_diff_min_ac_dn8_slot = var_diff_min_ac_dn8;
        *var_diff_min_ac_dn9_slot = var_diff_min_ac_dn9;
        *var_dleff_ac_slot = var_dleff_ac;
        *var_dleff_ac_dn4_slot = var_dleff_ac_dn4;
        *var_dleff_ac_dn6_slot = var_dleff_ac_dn6;
        *var_dleff_ac_dn7_slot = var_dleff_ac_dn7;
        *var_dleff_ac_dn8_slot = var_dleff_ac_dn8;
        *var_dleff_ac_dn9_slot = var_dleff_ac_dn9;
        *var_dx_wi_1d_ac_slot = var_dx_wi_1d_ac;
        *var_dx_wi_1d_ac_dn4_slot = var_dx_wi_1d_ac_dn4;
        *var_dx_wi_1d_ac_dn6_slot = var_dx_wi_1d_ac_dn6;
        *var_dx_wi_1d_ac_dn7_slot = var_dx_wi_1d_ac_dn7;
        *var_dx_wi_1d_ac_dn8_slot = var_dx_wi_1d_ac_dn8;
        *var_dx_wi_1d_ac_dn9_slot = var_dx_wi_1d_ac_dn9;
        *var_dx_wi_ac_slot = var_dx_wi_ac;
        *var_dx_wi_ac_dn4_slot = var_dx_wi_ac_dn4;
        *var_dx_wi_ac_dn6_slot = var_dx_wi_ac_dn6;
        *var_dx_wi_ac_dn7_slot = var_dx_wi_ac_dn7;
        *var_dx_wi_ac_dn8_slot = var_dx_wi_ac_dn8;
        *var_dx_wi_ac_dn9_slot = var_dx_wi_ac_dn9;
        *var_dxg1_dibl_ac_slot = var_dxg1_dibl_ac;
        *var_dxg1_dibl_ac_dn4_slot = var_dxg1_dibl_ac_dn4;
        *var_dxg1_dibl_ac_dn6_slot = var_dxg1_dibl_ac_dn6;
        *var_dxg1_dibl_ac_dn7_slot = var_dxg1_dibl_ac_dn7;
        *var_dxg1_dibl_ac_dn8_slot = var_dxg1_dibl_ac_dn8;
        *var_dxg1_dibl_ac_dn9_slot = var_dxg1_dibl_ac_dn9;
        *var_inv_k1_ac_slot = var_inv_k1_ac;
        *var_inv_k1_ac_dn4_slot = var_inv_k1_ac_dn4;
        *var_inv_k1_ac_dn6_slot = var_inv_k1_ac_dn6;
        *var_inv_k1_ac_dn7_slot = var_inv_k1_ac_dn7;
        *var_inv_k1_ac_dn8_slot = var_inv_k1_ac_dn8;
        *var_inv_k1_ac_dn9_slot = var_inv_k1_ac_dn9;
        *var_inv_k2_ac_slot = var_inv_k2_ac;
        *var_inv_k2_ac_dn4_slot = var_inv_k2_ac_dn4;
        *var_inv_k2_ac_dn6_slot = var_inv_k2_ac_dn6;
        *var_inv_k2_ac_dn7_slot = var_inv_k2_ac_dn7;
        *var_inv_k2_ac_dn8_slot = var_inv_k2_ac_dn8;
        *var_inv_k2_ac_dn9_slot = var_inv_k2_ac_dn9;
        *var_k1_ac_slot = var_k1_ac;
        *var_k1_ac_dn4_slot = var_k1_ac_dn4;
        *var_k1_ac_dn6_slot = var_k1_ac_dn6;
        *var_k1_ac_dn7_slot = var_k1_ac_dn7;
        *var_k1_ac_dn8_slot = var_k1_ac_dn8;
        *var_k1_ac_dn9_slot = var_k1_ac_dn9;
        *var_k1q1d_ac_slot = var_k1q1d_ac;
        *var_k1q1d_ac_dn4_slot = var_k1q1d_ac_dn4;
        *var_k1q1d_ac_dn6_slot = var_k1q1d_ac_dn6;
        *var_k1q1d_ac_dn7_slot = var_k1q1d_ac_dn7;
        *var_k1q1d_ac_dn8_slot = var_k1q1d_ac_dn8;
        *var_k1q1d_ac_dn9_slot = var_k1q1d_ac_dn9;
        *var_k1q1s_ac_slot = var_k1q1s_ac;
        *var_k1q1s_ac_dn4_slot = var_k1q1s_ac_dn4;
        *var_k1q1s_ac_dn6_slot = var_k1q1s_ac_dn6;
        *var_k1q1s_ac_dn7_slot = var_k1q1s_ac_dn7;
        *var_k1q1s_ac_dn8_slot = var_k1q1s_ac_dn8;
        *var_k1q1s_ac_dn9_slot = var_k1q1s_ac_dn9;
        *var_k2_ac_slot = var_k2_ac;
        *var_k2_ac_dn4_slot = var_k2_ac_dn4;
        *var_k2_ac_dn6_slot = var_k2_ac_dn6;
        *var_k2_ac_dn7_slot = var_k2_ac_dn7;
        *var_k2_ac_dn8_slot = var_k2_ac_dn8;
        *var_k2_ac_dn9_slot = var_k2_ac_dn9;
        *var_k2q2d_ac_slot = var_k2q2d_ac;
        *var_k2q2d_ac_dn4_slot = var_k2q2d_ac_dn4;
        *var_k2q2d_ac_dn6_slot = var_k2q2d_ac_dn6;
        *var_k2q2d_ac_dn7_slot = var_k2q2d_ac_dn7;
        *var_k2q2d_ac_dn8_slot = var_k2q2d_ac_dn8;
        *var_k2q2d_ac_dn9_slot = var_k2q2d_ac_dn9;
        *var_k2q2s_ac_slot = var_k2q2s_ac;
        *var_k2q2s_ac_dn4_slot = var_k2q2s_ac_dn4;
        *var_k2q2s_ac_dn6_slot = var_k2q2s_ac_dn6;
        *var_k2q2s_ac_dn7_slot = var_k2q2s_ac_dn7;
        *var_k2q2s_ac_dn8_slot = var_k2q2s_ac_dn8;
        *var_k2q2s_ac_dn9_slot = var_k2q2s_ac_dn9;
        *var_keq_ac_slot = var_keq_ac;
        *var_keq_ac_dn4_slot = var_keq_ac_dn4;
        *var_keq_ac_dn6_slot = var_keq_ac_dn6;
        *var_keq_ac_dn7_slot = var_keq_ac_dn7;
        *var_keq_ac_dn8_slot = var_keq_ac_dn8;
        *var_keq_ac_dn9_slot = var_keq_ac_dn9;
        *var_prod1_ac_slot = var_prod1_ac;
        *var_prod1_ac_dn4_slot = var_prod1_ac_dn4;
        *var_prod1_ac_dn6_slot = var_prod1_ac_dn6;
        *var_prod1_ac_dn7_slot = var_prod1_ac_dn7;
        *var_prod1_ac_dn8_slot = var_prod1_ac_dn8;
        *var_prod1_ac_dn9_slot = var_prod1_ac_dn9;
        *var_prod2_ac_slot = var_prod2_ac;
        *var_prod2_ac_dn4_slot = var_prod2_ac_dn4;
        *var_prod2_ac_dn6_slot = var_prod2_ac_dn6;
        *var_prod2_ac_dn7_slot = var_prod2_ac_dn7;
        *var_prod2_ac_dn8_slot = var_prod2_ac_dn8;
        *var_prod2_ac_dn9_slot = var_prod2_ac_dn9;
        *var_qi1m_ac_slot = var_qi1m_ac;
        *var_qi1m_ac_dn4_slot = var_qi1m_ac_dn4;
        *var_qi1m_ac_dn6_slot = var_qi1m_ac_dn6;
        *var_qi1m_ac_dn7_slot = var_qi1m_ac_dn7;
        *var_qi1m_ac_dn8_slot = var_qi1m_ac_dn8;
        *var_qi1m_ac_dn9_slot = var_qi1m_ac_dn9;
        *var_qi2m_ac_slot = var_qi2m_ac;
        *var_qi2m_ac_dn4_slot = var_qi2m_ac_dn4;
        *var_qi2m_ac_dn6_slot = var_qi2m_ac_dn6;
        *var_qi2m_ac_dn7_slot = var_qi2m_ac_dn7;
        *var_qi2m_ac_dn8_slot = var_qi2m_ac_dn8;
        *var_qi2m_ac_dn9_slot = var_qi2m_ac_dn9;
        *var_qim_ac_slot = var_qim_ac;
        *var_qim_ac_dn4_slot = var_qim_ac_dn4;
        *var_qim_ac_dn6_slot = var_qim_ac_dn6;
        *var_qim_ac_dn7_slot = var_qim_ac_dn7;
        *var_qim_ac_dn8_slot = var_qim_ac_dn8;
        *var_qim_ac_dn9_slot = var_qim_ac_dn9;
        *var_qmfact1_ac_slot = var_qmfact1_ac;
        *var_qmfact1_ac_dn4_slot = var_qmfact1_ac_dn4;
        *var_qmfact1_ac_dn6_slot = var_qmfact1_ac_dn6;
        *var_qmfact1_ac_dn7_slot = var_qmfact1_ac_dn7;
        *var_qmfact1_ac_dn8_slot = var_qmfact1_ac_dn8;
        *var_qmfact1_ac_dn9_slot = var_qmfact1_ac_dn9;
        *var_qmfact2_ac_slot = var_qmfact2_ac;
        *var_qmfact2_ac_dn4_slot = var_qmfact2_ac_dn4;
        *var_qmfact2_ac_dn6_slot = var_qmfact2_ac_dn6;
        *var_qmfact2_ac_dn7_slot = var_qmfact2_ac_dn7;
        *var_qmfact2_ac_dn8_slot = var_qmfact2_ac_dn8;
        *var_qmfact2_ac_dn9_slot = var_qmfact2_ac_dn9;
        *var_ratio_pd_ac_slot = var_ratio_pd_ac;
        *var_ratio_pd_ac_dn4_slot = var_ratio_pd_ac_dn4;
        *var_ratio_pd_ac_dn6_slot = var_ratio_pd_ac_dn6;
        *var_ratio_pd_ac_dn7_slot = var_ratio_pd_ac_dn7;
        *var_ratio_pd_ac_dn8_slot = var_ratio_pd_ac_dn8;
        *var_ratio_pd_ac_dn9_slot = var_ratio_pd_ac_dn9;
        *var_sce1_ac_slot = var_sce1_ac;
        *var_sce1_ac_dn4_slot = var_sce1_ac_dn4;
        *var_sce1_ac_dn6_slot = var_sce1_ac_dn6;
        *var_sce1_ac_dn7_slot = var_sce1_ac_dn7;
        *var_sce1_ac_dn8_slot = var_sce1_ac_dn8;
        *var_sce1_ac_dn9_slot = var_sce1_ac_dn9;
        *var_sce2_ac_slot = var_sce2_ac;
        *var_sce2_ac_dn4_slot = var_sce2_ac_dn4;
        *var_sce2_ac_dn6_slot = var_sce2_ac_dn6;
        *var_sce2_ac_dn7_slot = var_sce2_ac_dn7;
        *var_sce2_ac_dn8_slot = var_sce2_ac_dn8;
        *var_sce2_ac_dn9_slot = var_sce2_ac_dn9;
        *var_xdriftd_ac_slot = var_xdriftd_ac;
        *var_xdriftd_ac_dn4_slot = var_xdriftd_ac_dn4;
        *var_xdriftd_ac_dn6_slot = var_xdriftd_ac_dn6;
        *var_xdriftd_ac_dn7_slot = var_xdriftd_ac_dn7;
        *var_xdriftd_ac_dn8_slot = var_xdriftd_ac_dn8;
        *var_xdriftd_ac_dn9_slot = var_xdriftd_ac_dn9;
        *var_xdrifts_ac_slot = var_xdrifts_ac;
        *var_xdrifts_ac_dn4_slot = var_xdrifts_ac_dn4;
        *var_xdrifts_ac_dn6_slot = var_xdrifts_ac_dn6;
        *var_xdrifts_ac_dn7_slot = var_xdrifts_ac_dn7;
        *var_xdrifts_ac_dn8_slot = var_xdrifts_ac_dn8;
        *var_xdrifts_ac_dn9_slot = var_xdrifts_ac_dn9;
        *var_xedge_ac_slot = var_xedge_ac;
        *var_xedge_ac_dn4_slot = var_xedge_ac_dn4;
        *var_xedge_ac_dn6_slot = var_xedge_ac_dn6;
        *var_xedge_ac_dn7_slot = var_xedge_ac_dn7;
        *var_xedge_ac_dn8_slot = var_xedge_ac_dn8;
        *var_xedge_ac_dn9_slot = var_xedge_ac_dn9;
        *var_xg20shift_ac_slot = var_xg20shift_ac;
        *var_xg20shift_ac_dn4_slot = var_xg20shift_ac_dn4;
        *var_xg20shift_ac_dn6_slot = var_xg20shift_ac_dn6;
        *var_xg20shift_ac_dn7_slot = var_xg20shift_ac_dn7;
        *var_xg20shift_ac_dn8_slot = var_xg20shift_ac_dn8;
        *var_xg20shift_ac_dn9_slot = var_xg20shift_ac_dn9;
        *var_xg2_ac_slot = var_xg2_ac;
        *var_xg2_ac_dn4_slot = var_xg2_ac_dn4;
        *var_xg2_ac_dn6_slot = var_xg2_ac_dn6;
        *var_xg2_ac_dn7_slot = var_xg2_ac_dn7;
        *var_xg2_ac_dn8_slot = var_xg2_ac_dn8;
        *var_xg2_ac_dn9_slot = var_xg2_ac_dn9;
        *var_xg2x_ac_slot = var_xg2x_ac;
        *var_xg2x_ac_dn4_slot = var_xg2x_ac_dn4;
        *var_xg2x_ac_dn6_slot = var_xg2x_ac_dn6;
        *var_xg2x_ac_dn7_slot = var_xg2x_ac_dn7;
        *var_xg2x_ac_dn8_slot = var_xg2x_ac_dn8;
        *var_xg2x_ac_dn9_slot = var_xg2x_ac_dn9;
        *var_zsat_ac_slot = var_zsat_ac;
        *var_zsat_ac_dn4_slot = var_zsat_ac_dn4;
        *var_zsat_ac_dn6_slot = var_zsat_ac_dn6;
        *var_zsat_ac_dn7_slot = var_zsat_ac_dn7;
        *var_zsat_ac_dn8_slot = var_zsat_ac_dn8;
        *var_zsat_ac_dn9_slot = var_zsat_ac_dn9;
    }

    pub(super) fn stamp_transient_block_117(
        p: &Parameters,
        var_area_phit: f64,
        var_area_phit_dn4: f64,
        var_area_phit_dn6: f64,
        var_area_phit_dn7: f64,
        var_area_phit_dn8: f64,
        var_area_phit_dn9: f64,
        var_csiprime_ac: f64,
        var_csiprime_ac_dn4: f64,
        var_csiprime_ac_dn6: f64,
        var_csiprime_ac_dn7: f64,
        var_csiprime_ac_dn8: f64,
        var_csiprime_ac_dn9: f64,
        var_delta_k1q1_dc: f64,
        var_delta_k1q1_dc_dn4: f64,
        var_delta_k1q1_dc_dn6: f64,
        var_delta_k1q1_dc_dn7: f64,
        var_delta_k1q1_dc_dn8: f64,
        var_delta_k1q1_dc_dn9: f64,
        var_delta_k2q2_dc: f64,
        var_delta_k2q2_dc_dn4: f64,
        var_delta_k2q2_dc_dn6: f64,
        var_delta_k2q2_dc_dn7: f64,
        var_delta_k2q2_dc_dn8: f64,
        var_delta_k2q2_dc_dn9: f64,
        var_dx_wi_1d_ac: f64,
        var_dx_wi_1d_ac_dn4: f64,
        var_dx_wi_1d_ac_dn6: f64,
        var_dx_wi_1d_ac_dn7: f64,
        var_dx_wi_1d_ac_dn8: f64,
        var_dx_wi_1d_ac_dn9: f64,
        var_dx_wi_ac: f64,
        var_dx_wi_ac_dn4: f64,
        var_dx_wi_ac_dn6: f64,
        var_dx_wi_ac_dn7: f64,
        var_dx_wi_ac_dn8: f64,
        var_dx_wi_ac_dn9: f64,
        var_fif_i: f64,
        var_fsceac_i: f64,
        var_guard1080: f64,
        var_k1_dc: f64,
        var_k1_dc_dn4: f64,
        var_k1_dc_dn6: f64,
        var_k1_dc_dn7: f64,
        var_k1_dc_dn8: f64,
        var_k1_dc_dn9: f64,
        var_k1q1d_dc: f64,
        var_k1q1d_dc_dn4: f64,
        var_k1q1d_dc_dn6: f64,
        var_k1q1d_dc_dn7: f64,
        var_k1q1d_dc_dn8: f64,
        var_k1q1d_dc_dn9: f64,
        var_k1q1s_dc: f64,
        var_k1q1s_dc_dn4: f64,
        var_k1q1s_dc_dn6: f64,
        var_k1q1s_dc_dn7: f64,
        var_k1q1s_dc_dn8: f64,
        var_k1q1s_dc_dn9: f64,
        var_k2_dc: f64,
        var_k2_dc_dn4: f64,
        var_k2_dc_dn6: f64,
        var_k2_dc_dn7: f64,
        var_k2_dc_dn8: f64,
        var_k2_dc_dn9: f64,
        var_k2q2d_dc: f64,
        var_k2q2d_dc_dn4: f64,
        var_k2q2d_dc_dn6: f64,
        var_k2q2d_dc_dn7: f64,
        var_k2q2d_dc_dn8: f64,
        var_k2q2d_dc_dn9: f64,
        var_k2q2s_dc: f64,
        var_k2q2s_dc_dn4: f64,
        var_k2q2s_dc_dn6: f64,
        var_k2q2s_dc_dn7: f64,
        var_k2q2s_dc_dn8: f64,
        var_k2q2s_dc_dn9: f64,
        var_prod1_dc: f64,
        var_prod1_dc_dn4: f64,
        var_prod1_dc_dn6: f64,
        var_prod1_dc_dn7: f64,
        var_prod1_dc_dn8: f64,
        var_prod1_dc_dn9: f64,
        var_prod2_dc: f64,
        var_prod2_dc_dn4: f64,
        var_prod2_dc_dn6: f64,
        var_prod2_dc_dn7: f64,
        var_prod2_dc_dn8: f64,
        var_prod2_dc_dn9: f64,
        var_qi1m_dc: f64,
        var_qi1m_dc_dn4: f64,
        var_qi1m_dc_dn6: f64,
        var_qi1m_dc_dn7: f64,
        var_qi1m_dc_dn8: f64,
        var_qi1m_dc_dn9: f64,
        var_qi2m_dc: f64,
        var_qi2m_dc_dn4: f64,
        var_qi2m_dc_dn6: f64,
        var_qi2m_dc_dn7: f64,
        var_qi2m_dc_dn8: f64,
        var_qi2m_dc_dn9: f64,
        var_qim_dc: f64,
        var_qim_dc_dn4: f64,
        var_qim_dc_dn6: f64,
        var_qim_dc_dn7: f64,
        var_qim_dc_dn8: f64,
        var_qim_dc_dn9: f64,
        var_qmfact1_dc: f64,
        var_qmfact1_dc_dn4: f64,
        var_qmfact1_dc_dn6: f64,
        var_qmfact1_dc_dn7: f64,
        var_qmfact1_dc_dn8: f64,
        var_qmfact1_dc_dn9: f64,
        var_qmfact2_dc: f64,
        var_qmfact2_dc_dn4: f64,
        var_qmfact2_dc_dn6: f64,
        var_qmfact2_dc_dn7: f64,
        var_qmfact2_dc_dn8: f64,
        var_qmfact2_dc_dn9: f64,
        var_ratio_pd_dc: f64,
        var_ratio_pd_dc_dn4: f64,
        var_ratio_pd_dc_dn6: f64,
        var_ratio_pd_dc_dn7: f64,
        var_ratio_pd_dc_dn8: f64,
        var_ratio_pd_dc_dn9: f64,
        var_xdriftd_dc: f64,
        var_xdriftd_dc_dn4: f64,
        var_xdriftd_dc_dn6: f64,
        var_xdriftd_dc_dn7: f64,
        var_xdriftd_dc_dn8: f64,
        var_xdriftd_dc_dn9: f64,
        var_xdrifts_dc: f64,
        var_xdrifts_dc_dn4: f64,
        var_xdrifts_dc_dn6: f64,
        var_xdrifts_dc_dn7: f64,
        var_xdrifts_dc_dn8: f64,
        var_xdrifts_dc_dn9: f64,
        var_xth_1d: f64,
        var_xth_1d_dn4: f64,
        var_xth_1d_dn6: f64,
        var_xth_1d_dn7: f64,
        var_xth_1d_dn8: f64,
        var_xth_1d_dn9: f64,
        var_zsat_dc: f64,
        var_zsat_dc_dn4: f64,
        var_zsat_dc_dn6: f64,
        var_zsat_dc_dn7: f64,
        var_zsat_dc_dn8: f64,
        var_zsat_dc_dn9: f64,
        var_delta_k1q1_ac_slot: &mut f64,
        var_delta_k1q1_ac_dn4_slot: &mut f64,
        var_delta_k1q1_ac_dn6_slot: &mut f64,
        var_delta_k1q1_ac_dn7_slot: &mut f64,
        var_delta_k1q1_ac_dn8_slot: &mut f64,
        var_delta_k1q1_ac_dn9_slot: &mut f64,
        var_delta_k2q2_ac_slot: &mut f64,
        var_delta_k2q2_ac_dn4_slot: &mut f64,
        var_delta_k2q2_ac_dn6_slot: &mut f64,
        var_delta_k2q2_ac_dn7_slot: &mut f64,
        var_delta_k2q2_ac_dn8_slot: &mut f64,
        var_delta_k2q2_ac_dn9_slot: &mut f64,
        var_guard1234_slot: &mut f64,
        var_guard1235_slot: &mut f64,
        var_k1_ac_slot: &mut f64,
        var_k1_ac_dn4_slot: &mut f64,
        var_k1_ac_dn6_slot: &mut f64,
        var_k1_ac_dn7_slot: &mut f64,
        var_k1_ac_dn8_slot: &mut f64,
        var_k1_ac_dn9_slot: &mut f64,
        var_k1q1d_ac_slot: &mut f64,
        var_k1q1d_ac_dn4_slot: &mut f64,
        var_k1q1d_ac_dn6_slot: &mut f64,
        var_k1q1d_ac_dn7_slot: &mut f64,
        var_k1q1d_ac_dn8_slot: &mut f64,
        var_k1q1d_ac_dn9_slot: &mut f64,
        var_k1q1deff_slot: &mut f64,
        var_k1q1deff_dn4_slot: &mut f64,
        var_k1q1deff_dn6_slot: &mut f64,
        var_k1q1deff_dn7_slot: &mut f64,
        var_k1q1deff_dn8_slot: &mut f64,
        var_k1q1deff_dn9_slot: &mut f64,
        var_k1q1eff_slot: &mut f64,
        var_k1q1eff_dn4_slot: &mut f64,
        var_k1q1eff_dn6_slot: &mut f64,
        var_k1q1eff_dn7_slot: &mut f64,
        var_k1q1eff_dn8_slot: &mut f64,
        var_k1q1eff_dn9_slot: &mut f64,
        var_k1q1m_slot: &mut f64,
        var_k1q1m_dn4_slot: &mut f64,
        var_k1q1m_dn6_slot: &mut f64,
        var_k1q1m_dn7_slot: &mut f64,
        var_k1q1m_dn8_slot: &mut f64,
        var_k1q1m_dn9_slot: &mut f64,
        var_k1q1s_ac_slot: &mut f64,
        var_k1q1s_ac_dn4_slot: &mut f64,
        var_k1q1s_ac_dn6_slot: &mut f64,
        var_k1q1s_ac_dn7_slot: &mut f64,
        var_k1q1s_ac_dn8_slot: &mut f64,
        var_k1q1s_ac_dn9_slot: &mut f64,
        var_k2_ac_slot: &mut f64,
        var_k2_ac_dn4_slot: &mut f64,
        var_k2_ac_dn6_slot: &mut f64,
        var_k2_ac_dn7_slot: &mut f64,
        var_k2_ac_dn8_slot: &mut f64,
        var_k2_ac_dn9_slot: &mut f64,
        var_k2q2d_ac_slot: &mut f64,
        var_k2q2d_ac_dn4_slot: &mut f64,
        var_k2q2d_ac_dn6_slot: &mut f64,
        var_k2q2d_ac_dn7_slot: &mut f64,
        var_k2q2d_ac_dn8_slot: &mut f64,
        var_k2q2d_ac_dn9_slot: &mut f64,
        var_k2q2deff_slot: &mut f64,
        var_k2q2deff_dn4_slot: &mut f64,
        var_k2q2deff_dn6_slot: &mut f64,
        var_k2q2deff_dn7_slot: &mut f64,
        var_k2q2deff_dn8_slot: &mut f64,
        var_k2q2deff_dn9_slot: &mut f64,
        var_k2q2eff_slot: &mut f64,
        var_k2q2eff_dn4_slot: &mut f64,
        var_k2q2eff_dn6_slot: &mut f64,
        var_k2q2eff_dn7_slot: &mut f64,
        var_k2q2eff_dn8_slot: &mut f64,
        var_k2q2eff_dn9_slot: &mut f64,
        var_k2q2m_slot: &mut f64,
        var_k2q2m_dn4_slot: &mut f64,
        var_k2q2m_dn6_slot: &mut f64,
        var_k2q2m_dn7_slot: &mut f64,
        var_k2q2m_dn8_slot: &mut f64,
        var_k2q2m_dn9_slot: &mut f64,
        var_k2q2s_ac_slot: &mut f64,
        var_k2q2s_ac_dn4_slot: &mut f64,
        var_k2q2s_ac_dn6_slot: &mut f64,
        var_k2q2s_ac_dn7_slot: &mut f64,
        var_k2q2s_ac_dn8_slot: &mut f64,
        var_k2q2s_ac_dn9_slot: &mut f64,
        var_prod1_ac_slot: &mut f64,
        var_prod1_ac_dn4_slot: &mut f64,
        var_prod1_ac_dn6_slot: &mut f64,
        var_prod1_ac_dn7_slot: &mut f64,
        var_prod1_ac_dn8_slot: &mut f64,
        var_prod1_ac_dn9_slot: &mut f64,
        var_prod2_ac_slot: &mut f64,
        var_prod2_ac_dn4_slot: &mut f64,
        var_prod2_ac_dn6_slot: &mut f64,
        var_prod2_ac_dn7_slot: &mut f64,
        var_prod2_ac_dn8_slot: &mut f64,
        var_prod2_ac_dn9_slot: &mut f64,
        var_qb_slot: &mut f64,
        var_qb_dn4_slot: &mut f64,
        var_qb_dn6_slot: &mut f64,
        var_qb_dn7_slot: &mut f64,
        var_qb_dn8_slot: &mut f64,
        var_qb_dn9_slot: &mut f64,
        var_qd_slot: &mut f64,
        var_qd_dn4_slot: &mut f64,
        var_qd_dn6_slot: &mut f64,
        var_qd_dn7_slot: &mut f64,
        var_qd_dn8_slot: &mut f64,
        var_qd_dn9_slot: &mut f64,
        var_qg_slot: &mut f64,
        var_qg_dn4_slot: &mut f64,
        var_qg_dn6_slot: &mut f64,
        var_qg_dn7_slot: &mut f64,
        var_qg_dn8_slot: &mut f64,
        var_qg_dn9_slot: &mut f64,
        var_qi1m_ac_slot: &mut f64,
        var_qi1m_ac_dn4_slot: &mut f64,
        var_qi1m_ac_dn6_slot: &mut f64,
        var_qi1m_ac_dn7_slot: &mut f64,
        var_qi1m_ac_dn8_slot: &mut f64,
        var_qi1m_ac_dn9_slot: &mut f64,
        var_qi2m_ac_slot: &mut f64,
        var_qi2m_ac_dn4_slot: &mut f64,
        var_qi2m_ac_dn6_slot: &mut f64,
        var_qi2m_ac_dn7_slot: &mut f64,
        var_qi2m_ac_dn8_slot: &mut f64,
        var_qi2m_ac_dn9_slot: &mut f64,
        var_qim_ac_slot: &mut f64,
        var_qim_ac_dn4_slot: &mut f64,
        var_qim_ac_dn6_slot: &mut f64,
        var_qim_ac_dn7_slot: &mut f64,
        var_qim_ac_dn8_slot: &mut f64,
        var_qim_ac_dn9_slot: &mut f64,
        var_qmfact1_ac_slot: &mut f64,
        var_qmfact1_ac_dn4_slot: &mut f64,
        var_qmfact1_ac_dn6_slot: &mut f64,
        var_qmfact1_ac_dn7_slot: &mut f64,
        var_qmfact1_ac_dn8_slot: &mut f64,
        var_qmfact1_ac_dn9_slot: &mut f64,
        var_qmfact2_ac_slot: &mut f64,
        var_qmfact2_ac_dn4_slot: &mut f64,
        var_qmfact2_ac_dn6_slot: &mut f64,
        var_qmfact2_ac_dn7_slot: &mut f64,
        var_qmfact2_ac_dn8_slot: &mut f64,
        var_qmfact2_ac_dn9_slot: &mut f64,
        var_ratio_pd_ac_slot: &mut f64,
        var_ratio_pd_ac_dn4_slot: &mut f64,
        var_ratio_pd_ac_dn6_slot: &mut f64,
        var_ratio_pd_ac_dn7_slot: &mut f64,
        var_ratio_pd_ac_dn8_slot: &mut f64,
        var_ratio_pd_ac_dn9_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_xdriftd_ac_slot: &mut f64,
        var_xdriftd_ac_dn4_slot: &mut f64,
        var_xdriftd_ac_dn6_slot: &mut f64,
        var_xdriftd_ac_dn7_slot: &mut f64,
        var_xdriftd_ac_dn8_slot: &mut f64,
        var_xdriftd_ac_dn9_slot: &mut f64,
        var_xdrifts_ac_slot: &mut f64,
        var_xdrifts_ac_dn4_slot: &mut f64,
        var_xdrifts_ac_dn6_slot: &mut f64,
        var_xdrifts_ac_dn7_slot: &mut f64,
        var_xdrifts_ac_dn8_slot: &mut f64,
        var_xdrifts_ac_dn9_slot: &mut f64,
        var_xeffs_slot: &mut f64,
        var_xeffs_dn4_slot: &mut f64,
        var_xeffs_dn6_slot: &mut f64,
        var_xeffs_dn7_slot: &mut f64,
        var_xeffs_dn8_slot: &mut f64,
        var_xeffs_dn9_slot: &mut f64,
        var_zsat_ac_slot: &mut f64,
        var_zsat_ac_dn4_slot: &mut f64,
        var_zsat_ac_dn6_slot: &mut f64,
        var_zsat_ac_dn7_slot: &mut f64,
        var_zsat_ac_dn8_slot: &mut f64,
        var_zsat_ac_dn9_slot: &mut f64,
    ) {
        let mut var_delta_k1q1_ac: f64 = *var_delta_k1q1_ac_slot;
        let mut var_delta_k1q1_ac_dn4: f64 = *var_delta_k1q1_ac_dn4_slot;
        let mut var_delta_k1q1_ac_dn6: f64 = *var_delta_k1q1_ac_dn6_slot;
        let mut var_delta_k1q1_ac_dn7: f64 = *var_delta_k1q1_ac_dn7_slot;
        let mut var_delta_k1q1_ac_dn8: f64 = *var_delta_k1q1_ac_dn8_slot;
        let mut var_delta_k1q1_ac_dn9: f64 = *var_delta_k1q1_ac_dn9_slot;
        let mut var_delta_k2q2_ac: f64 = *var_delta_k2q2_ac_slot;
        let mut var_delta_k2q2_ac_dn4: f64 = *var_delta_k2q2_ac_dn4_slot;
        let mut var_delta_k2q2_ac_dn6: f64 = *var_delta_k2q2_ac_dn6_slot;
        let mut var_delta_k2q2_ac_dn7: f64 = *var_delta_k2q2_ac_dn7_slot;
        let mut var_delta_k2q2_ac_dn8: f64 = *var_delta_k2q2_ac_dn8_slot;
        let mut var_delta_k2q2_ac_dn9: f64 = *var_delta_k2q2_ac_dn9_slot;
        let mut var_guard1234: f64 = *var_guard1234_slot;
        let mut var_guard1235: f64 = *var_guard1235_slot;
        let mut var_k1_ac: f64 = *var_k1_ac_slot;
        let mut var_k1_ac_dn4: f64 = *var_k1_ac_dn4_slot;
        let mut var_k1_ac_dn6: f64 = *var_k1_ac_dn6_slot;
        let mut var_k1_ac_dn7: f64 = *var_k1_ac_dn7_slot;
        let mut var_k1_ac_dn8: f64 = *var_k1_ac_dn8_slot;
        let mut var_k1_ac_dn9: f64 = *var_k1_ac_dn9_slot;
        let mut var_k1q1d_ac: f64 = *var_k1q1d_ac_slot;
        let mut var_k1q1d_ac_dn4: f64 = *var_k1q1d_ac_dn4_slot;
        let mut var_k1q1d_ac_dn6: f64 = *var_k1q1d_ac_dn6_slot;
        let mut var_k1q1d_ac_dn7: f64 = *var_k1q1d_ac_dn7_slot;
        let mut var_k1q1d_ac_dn8: f64 = *var_k1q1d_ac_dn8_slot;
        let mut var_k1q1d_ac_dn9: f64 = *var_k1q1d_ac_dn9_slot;
        let mut var_k1q1deff: f64 = *var_k1q1deff_slot;
        let mut var_k1q1deff_dn4: f64 = *var_k1q1deff_dn4_slot;
        let mut var_k1q1deff_dn6: f64 = *var_k1q1deff_dn6_slot;
        let mut var_k1q1deff_dn7: f64 = *var_k1q1deff_dn7_slot;
        let mut var_k1q1deff_dn8: f64 = *var_k1q1deff_dn8_slot;
        let mut var_k1q1deff_dn9: f64 = *var_k1q1deff_dn9_slot;
        let mut var_k1q1eff: f64 = *var_k1q1eff_slot;
        let mut var_k1q1eff_dn4: f64 = *var_k1q1eff_dn4_slot;
        let mut var_k1q1eff_dn6: f64 = *var_k1q1eff_dn6_slot;
        let mut var_k1q1eff_dn7: f64 = *var_k1q1eff_dn7_slot;
        let mut var_k1q1eff_dn8: f64 = *var_k1q1eff_dn8_slot;
        let mut var_k1q1eff_dn9: f64 = *var_k1q1eff_dn9_slot;
        let mut var_k1q1m: f64 = *var_k1q1m_slot;
        let mut var_k1q1m_dn4: f64 = *var_k1q1m_dn4_slot;
        let mut var_k1q1m_dn6: f64 = *var_k1q1m_dn6_slot;
        let mut var_k1q1m_dn7: f64 = *var_k1q1m_dn7_slot;
        let mut var_k1q1m_dn8: f64 = *var_k1q1m_dn8_slot;
        let mut var_k1q1m_dn9: f64 = *var_k1q1m_dn9_slot;
        let mut var_k1q1s_ac: f64 = *var_k1q1s_ac_slot;
        let mut var_k1q1s_ac_dn4: f64 = *var_k1q1s_ac_dn4_slot;
        let mut var_k1q1s_ac_dn6: f64 = *var_k1q1s_ac_dn6_slot;
        let mut var_k1q1s_ac_dn7: f64 = *var_k1q1s_ac_dn7_slot;
        let mut var_k1q1s_ac_dn8: f64 = *var_k1q1s_ac_dn8_slot;
        let mut var_k1q1s_ac_dn9: f64 = *var_k1q1s_ac_dn9_slot;
        let mut var_k2_ac: f64 = *var_k2_ac_slot;
        let mut var_k2_ac_dn4: f64 = *var_k2_ac_dn4_slot;
        let mut var_k2_ac_dn6: f64 = *var_k2_ac_dn6_slot;
        let mut var_k2_ac_dn7: f64 = *var_k2_ac_dn7_slot;
        let mut var_k2_ac_dn8: f64 = *var_k2_ac_dn8_slot;
        let mut var_k2_ac_dn9: f64 = *var_k2_ac_dn9_slot;
        let mut var_k2q2d_ac: f64 = *var_k2q2d_ac_slot;
        let mut var_k2q2d_ac_dn4: f64 = *var_k2q2d_ac_dn4_slot;
        let mut var_k2q2d_ac_dn6: f64 = *var_k2q2d_ac_dn6_slot;
        let mut var_k2q2d_ac_dn7: f64 = *var_k2q2d_ac_dn7_slot;
        let mut var_k2q2d_ac_dn8: f64 = *var_k2q2d_ac_dn8_slot;
        let mut var_k2q2d_ac_dn9: f64 = *var_k2q2d_ac_dn9_slot;
        let mut var_k2q2deff: f64 = *var_k2q2deff_slot;
        let mut var_k2q2deff_dn4: f64 = *var_k2q2deff_dn4_slot;
        let mut var_k2q2deff_dn6: f64 = *var_k2q2deff_dn6_slot;
        let mut var_k2q2deff_dn7: f64 = *var_k2q2deff_dn7_slot;
        let mut var_k2q2deff_dn8: f64 = *var_k2q2deff_dn8_slot;
        let mut var_k2q2deff_dn9: f64 = *var_k2q2deff_dn9_slot;
        let mut var_k2q2eff: f64 = *var_k2q2eff_slot;
        let mut var_k2q2eff_dn4: f64 = *var_k2q2eff_dn4_slot;
        let mut var_k2q2eff_dn6: f64 = *var_k2q2eff_dn6_slot;
        let mut var_k2q2eff_dn7: f64 = *var_k2q2eff_dn7_slot;
        let mut var_k2q2eff_dn8: f64 = *var_k2q2eff_dn8_slot;
        let mut var_k2q2eff_dn9: f64 = *var_k2q2eff_dn9_slot;
        let mut var_k2q2m: f64 = *var_k2q2m_slot;
        let mut var_k2q2m_dn4: f64 = *var_k2q2m_dn4_slot;
        let mut var_k2q2m_dn6: f64 = *var_k2q2m_dn6_slot;
        let mut var_k2q2m_dn7: f64 = *var_k2q2m_dn7_slot;
        let mut var_k2q2m_dn8: f64 = *var_k2q2m_dn8_slot;
        let mut var_k2q2m_dn9: f64 = *var_k2q2m_dn9_slot;
        let mut var_k2q2s_ac: f64 = *var_k2q2s_ac_slot;
        let mut var_k2q2s_ac_dn4: f64 = *var_k2q2s_ac_dn4_slot;
        let mut var_k2q2s_ac_dn6: f64 = *var_k2q2s_ac_dn6_slot;
        let mut var_k2q2s_ac_dn7: f64 = *var_k2q2s_ac_dn7_slot;
        let mut var_k2q2s_ac_dn8: f64 = *var_k2q2s_ac_dn8_slot;
        let mut var_k2q2s_ac_dn9: f64 = *var_k2q2s_ac_dn9_slot;
        let mut var_prod1_ac: f64 = *var_prod1_ac_slot;
        let mut var_prod1_ac_dn4: f64 = *var_prod1_ac_dn4_slot;
        let mut var_prod1_ac_dn6: f64 = *var_prod1_ac_dn6_slot;
        let mut var_prod1_ac_dn7: f64 = *var_prod1_ac_dn7_slot;
        let mut var_prod1_ac_dn8: f64 = *var_prod1_ac_dn8_slot;
        let mut var_prod1_ac_dn9: f64 = *var_prod1_ac_dn9_slot;
        let mut var_prod2_ac: f64 = *var_prod2_ac_slot;
        let mut var_prod2_ac_dn4: f64 = *var_prod2_ac_dn4_slot;
        let mut var_prod2_ac_dn6: f64 = *var_prod2_ac_dn6_slot;
        let mut var_prod2_ac_dn7: f64 = *var_prod2_ac_dn7_slot;
        let mut var_prod2_ac_dn8: f64 = *var_prod2_ac_dn8_slot;
        let mut var_prod2_ac_dn9: f64 = *var_prod2_ac_dn9_slot;
        let mut var_qb: f64 = *var_qb_slot;
        let mut var_qb_dn4: f64 = *var_qb_dn4_slot;
        let mut var_qb_dn6: f64 = *var_qb_dn6_slot;
        let mut var_qb_dn7: f64 = *var_qb_dn7_slot;
        let mut var_qb_dn8: f64 = *var_qb_dn8_slot;
        let mut var_qb_dn9: f64 = *var_qb_dn9_slot;
        let mut var_qd: f64 = *var_qd_slot;
        let mut var_qd_dn4: f64 = *var_qd_dn4_slot;
        let mut var_qd_dn6: f64 = *var_qd_dn6_slot;
        let mut var_qd_dn7: f64 = *var_qd_dn7_slot;
        let mut var_qd_dn8: f64 = *var_qd_dn8_slot;
        let mut var_qd_dn9: f64 = *var_qd_dn9_slot;
        let mut var_qg: f64 = *var_qg_slot;
        let mut var_qg_dn4: f64 = *var_qg_dn4_slot;
        let mut var_qg_dn6: f64 = *var_qg_dn6_slot;
        let mut var_qg_dn7: f64 = *var_qg_dn7_slot;
        let mut var_qg_dn8: f64 = *var_qg_dn8_slot;
        let mut var_qg_dn9: f64 = *var_qg_dn9_slot;
        let mut var_qi1m_ac: f64 = *var_qi1m_ac_slot;
        let mut var_qi1m_ac_dn4: f64 = *var_qi1m_ac_dn4_slot;
        let mut var_qi1m_ac_dn6: f64 = *var_qi1m_ac_dn6_slot;
        let mut var_qi1m_ac_dn7: f64 = *var_qi1m_ac_dn7_slot;
        let mut var_qi1m_ac_dn8: f64 = *var_qi1m_ac_dn8_slot;
        let mut var_qi1m_ac_dn9: f64 = *var_qi1m_ac_dn9_slot;
        let mut var_qi2m_ac: f64 = *var_qi2m_ac_slot;
        let mut var_qi2m_ac_dn4: f64 = *var_qi2m_ac_dn4_slot;
        let mut var_qi2m_ac_dn6: f64 = *var_qi2m_ac_dn6_slot;
        let mut var_qi2m_ac_dn7: f64 = *var_qi2m_ac_dn7_slot;
        let mut var_qi2m_ac_dn8: f64 = *var_qi2m_ac_dn8_slot;
        let mut var_qi2m_ac_dn9: f64 = *var_qi2m_ac_dn9_slot;
        let mut var_qim_ac: f64 = *var_qim_ac_slot;
        let mut var_qim_ac_dn4: f64 = *var_qim_ac_dn4_slot;
        let mut var_qim_ac_dn6: f64 = *var_qim_ac_dn6_slot;
        let mut var_qim_ac_dn7: f64 = *var_qim_ac_dn7_slot;
        let mut var_qim_ac_dn8: f64 = *var_qim_ac_dn8_slot;
        let mut var_qim_ac_dn9: f64 = *var_qim_ac_dn9_slot;
        let mut var_qmfact1_ac: f64 = *var_qmfact1_ac_slot;
        let mut var_qmfact1_ac_dn4: f64 = *var_qmfact1_ac_dn4_slot;
        let mut var_qmfact1_ac_dn6: f64 = *var_qmfact1_ac_dn6_slot;
        let mut var_qmfact1_ac_dn7: f64 = *var_qmfact1_ac_dn7_slot;
        let mut var_qmfact1_ac_dn8: f64 = *var_qmfact1_ac_dn8_slot;
        let mut var_qmfact1_ac_dn9: f64 = *var_qmfact1_ac_dn9_slot;
        let mut var_qmfact2_ac: f64 = *var_qmfact2_ac_slot;
        let mut var_qmfact2_ac_dn4: f64 = *var_qmfact2_ac_dn4_slot;
        let mut var_qmfact2_ac_dn6: f64 = *var_qmfact2_ac_dn6_slot;
        let mut var_qmfact2_ac_dn7: f64 = *var_qmfact2_ac_dn7_slot;
        let mut var_qmfact2_ac_dn8: f64 = *var_qmfact2_ac_dn8_slot;
        let mut var_qmfact2_ac_dn9: f64 = *var_qmfact2_ac_dn9_slot;
        let mut var_ratio_pd_ac: f64 = *var_ratio_pd_ac_slot;
        let mut var_ratio_pd_ac_dn4: f64 = *var_ratio_pd_ac_dn4_slot;
        let mut var_ratio_pd_ac_dn6: f64 = *var_ratio_pd_ac_dn6_slot;
        let mut var_ratio_pd_ac_dn7: f64 = *var_ratio_pd_ac_dn7_slot;
        let mut var_ratio_pd_ac_dn8: f64 = *var_ratio_pd_ac_dn8_slot;
        let mut var_ratio_pd_ac_dn9: f64 = *var_ratio_pd_ac_dn9_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_xdriftd_ac: f64 = *var_xdriftd_ac_slot;
        let mut var_xdriftd_ac_dn4: f64 = *var_xdriftd_ac_dn4_slot;
        let mut var_xdriftd_ac_dn6: f64 = *var_xdriftd_ac_dn6_slot;
        let mut var_xdriftd_ac_dn7: f64 = *var_xdriftd_ac_dn7_slot;
        let mut var_xdriftd_ac_dn8: f64 = *var_xdriftd_ac_dn8_slot;
        let mut var_xdriftd_ac_dn9: f64 = *var_xdriftd_ac_dn9_slot;
        let mut var_xdrifts_ac: f64 = *var_xdrifts_ac_slot;
        let mut var_xdrifts_ac_dn4: f64 = *var_xdrifts_ac_dn4_slot;
        let mut var_xdrifts_ac_dn6: f64 = *var_xdrifts_ac_dn6_slot;
        let mut var_xdrifts_ac_dn7: f64 = *var_xdrifts_ac_dn7_slot;
        let mut var_xdrifts_ac_dn8: f64 = *var_xdrifts_ac_dn8_slot;
        let mut var_xdrifts_ac_dn9: f64 = *var_xdrifts_ac_dn9_slot;
        let mut var_xeffs: f64 = *var_xeffs_slot;
        let mut var_xeffs_dn4: f64 = *var_xeffs_dn4_slot;
        let mut var_xeffs_dn6: f64 = *var_xeffs_dn6_slot;
        let mut var_xeffs_dn7: f64 = *var_xeffs_dn7_slot;
        let mut var_xeffs_dn8: f64 = *var_xeffs_dn8_slot;
        let mut var_xeffs_dn9: f64 = *var_xeffs_dn9_slot;
        let mut var_zsat_ac: f64 = *var_zsat_ac_slot;
        let mut var_zsat_ac_dn4: f64 = *var_zsat_ac_dn4_slot;
        let mut var_zsat_ac_dn6: f64 = *var_zsat_ac_dn6_slot;
        let mut var_zsat_ac_dn7: f64 = *var_zsat_ac_dn7_slot;
        let mut var_zsat_ac_dn8: f64 = *var_zsat_ac_dn8_slot;
        let mut var_zsat_ac_dn9: f64 = *var_zsat_ac_dn9_slot;

        let (assign41950_e47554, assign41950_e47554_d_n4, assign41950_e47554_d_n6, assign41950_e47554_d_n7, assign41950_e47554_d_n8, assign41950_e47554_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_k1_dc, var_k1_dc_dn4, var_k1_dc_dn6, var_k1_dc_dn7, var_k1_dc_dn8, var_k1_dc_dn9,)
    } else {
        (var_k1_ac, var_k1_ac_dn4, var_k1_ac_dn6, var_k1_ac_dn7, var_k1_ac_dn8, var_k1_ac_dn9,)
    }
};
        var_k1_ac = assign41950_e47554;
        var_k1_ac_dn4 = assign41950_e47554_d_n4;
        var_k1_ac_dn6 = assign41950_e47554_d_n6;
        var_k1_ac_dn7 = assign41950_e47554_d_n7;
        var_k1_ac_dn8 = assign41950_e47554_d_n8;
        var_k1_ac_dn9 = assign41950_e47554_d_n9;

        let (assign41960_e47559, assign41960_e47559_d_n4, assign41960_e47559_d_n6, assign41960_e47559_d_n7, assign41960_e47559_d_n8, assign41960_e47559_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_k2_dc, var_k2_dc_dn4, var_k2_dc_dn6, var_k2_dc_dn7, var_k2_dc_dn8, var_k2_dc_dn9,)
    } else {
        (var_k2_ac, var_k2_ac_dn4, var_k2_ac_dn6, var_k2_ac_dn7, var_k2_ac_dn8, var_k2_ac_dn9,)
    }
};
        var_k2_ac = assign41960_e47559;
        var_k2_ac_dn4 = assign41960_e47559_d_n4;
        var_k2_ac_dn6 = assign41960_e47559_d_n6;
        var_k2_ac_dn7 = assign41960_e47559_d_n7;
        var_k2_ac_dn8 = assign41960_e47559_d_n8;
        var_k2_ac_dn9 = assign41960_e47559_d_n9;

        let (assign41970_e47564, assign41970_e47564_d_n4, assign41970_e47564_d_n6, assign41970_e47564_d_n7, assign41970_e47564_d_n8, assign41970_e47564_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_k1q1s_dc, var_k1q1s_dc_dn4, var_k1q1s_dc_dn6, var_k1q1s_dc_dn7, var_k1q1s_dc_dn8, var_k1q1s_dc_dn9,)
    } else {
        (var_k1q1s_ac, var_k1q1s_ac_dn4, var_k1q1s_ac_dn6, var_k1q1s_ac_dn7, var_k1q1s_ac_dn8, var_k1q1s_ac_dn9,)
    }
};
        var_k1q1s_ac = assign41970_e47564;
        var_k1q1s_ac_dn4 = assign41970_e47564_d_n4;
        var_k1q1s_ac_dn6 = assign41970_e47564_d_n6;
        var_k1q1s_ac_dn7 = assign41970_e47564_d_n7;
        var_k1q1s_ac_dn8 = assign41970_e47564_d_n8;
        var_k1q1s_ac_dn9 = assign41970_e47564_d_n9;

        let (assign41980_e47569, assign41980_e47569_d_n4, assign41980_e47569_d_n6, assign41980_e47569_d_n7, assign41980_e47569_d_n8, assign41980_e47569_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_k2q2s_dc, var_k2q2s_dc_dn4, var_k2q2s_dc_dn6, var_k2q2s_dc_dn7, var_k2q2s_dc_dn8, var_k2q2s_dc_dn9,)
    } else {
        (var_k2q2s_ac, var_k2q2s_ac_dn4, var_k2q2s_ac_dn6, var_k2q2s_ac_dn7, var_k2q2s_ac_dn8, var_k2q2s_ac_dn9,)
    }
};
        var_k2q2s_ac = assign41980_e47569;
        var_k2q2s_ac_dn4 = assign41980_e47569_d_n4;
        var_k2q2s_ac_dn6 = assign41980_e47569_d_n6;
        var_k2q2s_ac_dn7 = assign41980_e47569_d_n7;
        var_k2q2s_ac_dn8 = assign41980_e47569_d_n8;
        var_k2q2s_ac_dn9 = assign41980_e47569_d_n9;

        let (assign41990_e47574, assign41990_e47574_d_n4, assign41990_e47574_d_n6, assign41990_e47574_d_n7, assign41990_e47574_d_n8, assign41990_e47574_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_xdrifts_dc, var_xdrifts_dc_dn4, var_xdrifts_dc_dn6, var_xdrifts_dc_dn7, var_xdrifts_dc_dn8, var_xdrifts_dc_dn9,)
    } else {
        (var_xdrifts_ac, var_xdrifts_ac_dn4, var_xdrifts_ac_dn6, var_xdrifts_ac_dn7, var_xdrifts_ac_dn8, var_xdrifts_ac_dn9,)
    }
};
        var_xdrifts_ac = assign41990_e47574;
        var_xdrifts_ac_dn4 = assign41990_e47574_d_n4;
        var_xdrifts_ac_dn6 = assign41990_e47574_d_n6;
        var_xdrifts_ac_dn7 = assign41990_e47574_d_n7;
        var_xdrifts_ac_dn8 = assign41990_e47574_d_n8;
        var_xdrifts_ac_dn9 = assign41990_e47574_d_n9;

        let (assign42000_e47579, assign42000_e47579_d_n4, assign42000_e47579_d_n6, assign42000_e47579_d_n7, assign42000_e47579_d_n8, assign42000_e47579_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_k1q1d_dc, var_k1q1d_dc_dn4, var_k1q1d_dc_dn6, var_k1q1d_dc_dn7, var_k1q1d_dc_dn8, var_k1q1d_dc_dn9,)
    } else {
        (var_k1q1d_ac, var_k1q1d_ac_dn4, var_k1q1d_ac_dn6, var_k1q1d_ac_dn7, var_k1q1d_ac_dn8, var_k1q1d_ac_dn9,)
    }
};
        var_k1q1d_ac = assign42000_e47579;
        var_k1q1d_ac_dn4 = assign42000_e47579_d_n4;
        var_k1q1d_ac_dn6 = assign42000_e47579_d_n6;
        var_k1q1d_ac_dn7 = assign42000_e47579_d_n7;
        var_k1q1d_ac_dn8 = assign42000_e47579_d_n8;
        var_k1q1d_ac_dn9 = assign42000_e47579_d_n9;

        let (assign42010_e47584, assign42010_e47584_d_n4, assign42010_e47584_d_n6, assign42010_e47584_d_n7, assign42010_e47584_d_n8, assign42010_e47584_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_k2q2d_dc, var_k2q2d_dc_dn4, var_k2q2d_dc_dn6, var_k2q2d_dc_dn7, var_k2q2d_dc_dn8, var_k2q2d_dc_dn9,)
    } else {
        (var_k2q2d_ac, var_k2q2d_ac_dn4, var_k2q2d_ac_dn6, var_k2q2d_ac_dn7, var_k2q2d_ac_dn8, var_k2q2d_ac_dn9,)
    }
};
        var_k2q2d_ac = assign42010_e47584;
        var_k2q2d_ac_dn4 = assign42010_e47584_d_n4;
        var_k2q2d_ac_dn6 = assign42010_e47584_d_n6;
        var_k2q2d_ac_dn7 = assign42010_e47584_d_n7;
        var_k2q2d_ac_dn8 = assign42010_e47584_d_n8;
        var_k2q2d_ac_dn9 = assign42010_e47584_d_n9;

        let (assign42020_e47589, assign42020_e47589_d_n4, assign42020_e47589_d_n6, assign42020_e47589_d_n7, assign42020_e47589_d_n8, assign42020_e47589_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_xdriftd_dc, var_xdriftd_dc_dn4, var_xdriftd_dc_dn6, var_xdriftd_dc_dn7, var_xdriftd_dc_dn8, var_xdriftd_dc_dn9,)
    } else {
        (var_xdriftd_ac, var_xdriftd_ac_dn4, var_xdriftd_ac_dn6, var_xdriftd_ac_dn7, var_xdriftd_ac_dn8, var_xdriftd_ac_dn9,)
    }
};
        var_xdriftd_ac = assign42020_e47589;
        var_xdriftd_ac_dn4 = assign42020_e47589_d_n4;
        var_xdriftd_ac_dn6 = assign42020_e47589_d_n6;
        var_xdriftd_ac_dn7 = assign42020_e47589_d_n7;
        var_xdriftd_ac_dn8 = assign42020_e47589_d_n8;
        var_xdriftd_ac_dn9 = assign42020_e47589_d_n9;

        let (assign42030_e47594, assign42030_e47594_d_n4, assign42030_e47594_d_n6, assign42030_e47594_d_n7, assign42030_e47594_d_n8, assign42030_e47594_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_qim_dc, var_qim_dc_dn4, var_qim_dc_dn6, var_qim_dc_dn7, var_qim_dc_dn8, var_qim_dc_dn9,)
    } else {
        (var_qim_ac, var_qim_ac_dn4, var_qim_ac_dn6, var_qim_ac_dn7, var_qim_ac_dn8, var_qim_ac_dn9,)
    }
};
        var_qim_ac = assign42030_e47594;
        var_qim_ac_dn4 = assign42030_e47594_d_n4;
        var_qim_ac_dn6 = assign42030_e47594_d_n6;
        var_qim_ac_dn7 = assign42030_e47594_d_n7;
        var_qim_ac_dn8 = assign42030_e47594_d_n8;
        var_qim_ac_dn9 = assign42030_e47594_d_n9;

        let (assign42040_e47599, assign42040_e47599_d_n4, assign42040_e47599_d_n6, assign42040_e47599_d_n7, assign42040_e47599_d_n8, assign42040_e47599_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_ratio_pd_dc, var_ratio_pd_dc_dn4, var_ratio_pd_dc_dn6, var_ratio_pd_dc_dn7, var_ratio_pd_dc_dn8, var_ratio_pd_dc_dn9,)
    } else {
        (var_ratio_pd_ac, var_ratio_pd_ac_dn4, var_ratio_pd_ac_dn6, var_ratio_pd_ac_dn7, var_ratio_pd_ac_dn8, var_ratio_pd_ac_dn9,)
    }
};
        var_ratio_pd_ac = assign42040_e47599;
        var_ratio_pd_ac_dn4 = assign42040_e47599_d_n4;
        var_ratio_pd_ac_dn6 = assign42040_e47599_d_n6;
        var_ratio_pd_ac_dn7 = assign42040_e47599_d_n7;
        var_ratio_pd_ac_dn8 = assign42040_e47599_d_n8;
        var_ratio_pd_ac_dn9 = assign42040_e47599_d_n9;

        let (assign42050_e47604, assign42050_e47604_d_n4, assign42050_e47604_d_n6, assign42050_e47604_d_n7, assign42050_e47604_d_n8, assign42050_e47604_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_qi1m_dc, var_qi1m_dc_dn4, var_qi1m_dc_dn6, var_qi1m_dc_dn7, var_qi1m_dc_dn8, var_qi1m_dc_dn9,)
    } else {
        (var_qi1m_ac, var_qi1m_ac_dn4, var_qi1m_ac_dn6, var_qi1m_ac_dn7, var_qi1m_ac_dn8, var_qi1m_ac_dn9,)
    }
};
        var_qi1m_ac = assign42050_e47604;
        var_qi1m_ac_dn4 = assign42050_e47604_d_n4;
        var_qi1m_ac_dn6 = assign42050_e47604_d_n6;
        var_qi1m_ac_dn7 = assign42050_e47604_d_n7;
        var_qi1m_ac_dn8 = assign42050_e47604_d_n8;
        var_qi1m_ac_dn9 = assign42050_e47604_d_n9;

        let (assign42060_e47609, assign42060_e47609_d_n4, assign42060_e47609_d_n6, assign42060_e47609_d_n7, assign42060_e47609_d_n8, assign42060_e47609_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_qi2m_dc, var_qi2m_dc_dn4, var_qi2m_dc_dn6, var_qi2m_dc_dn7, var_qi2m_dc_dn8, var_qi2m_dc_dn9,)
    } else {
        (var_qi2m_ac, var_qi2m_ac_dn4, var_qi2m_ac_dn6, var_qi2m_ac_dn7, var_qi2m_ac_dn8, var_qi2m_ac_dn9,)
    }
};
        var_qi2m_ac = assign42060_e47609;
        var_qi2m_ac_dn4 = assign42060_e47609_d_n4;
        var_qi2m_ac_dn6 = assign42060_e47609_d_n6;
        var_qi2m_ac_dn7 = assign42060_e47609_d_n7;
        var_qi2m_ac_dn8 = assign42060_e47609_d_n8;
        var_qi2m_ac_dn9 = assign42060_e47609_d_n9;

        let (assign42070_e47614, assign42070_e47614_d_n4, assign42070_e47614_d_n6, assign42070_e47614_d_n7, assign42070_e47614_d_n8, assign42070_e47614_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_zsat_dc, var_zsat_dc_dn4, var_zsat_dc_dn6, var_zsat_dc_dn7, var_zsat_dc_dn8, var_zsat_dc_dn9,)
    } else {
        (var_zsat_ac, var_zsat_ac_dn4, var_zsat_ac_dn6, var_zsat_ac_dn7, var_zsat_ac_dn8, var_zsat_ac_dn9,)
    }
};
        var_zsat_ac = assign42070_e47614;
        var_zsat_ac_dn4 = assign42070_e47614_d_n4;
        var_zsat_ac_dn6 = assign42070_e47614_d_n6;
        var_zsat_ac_dn7 = assign42070_e47614_d_n7;
        var_zsat_ac_dn8 = assign42070_e47614_d_n8;
        var_zsat_ac_dn9 = assign42070_e47614_d_n9;

        let (assign42080_e47619, assign42080_e47619_d_n4, assign42080_e47619_d_n6, assign42080_e47619_d_n7, assign42080_e47619_d_n8, assign42080_e47619_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_qmfact1_dc, var_qmfact1_dc_dn4, var_qmfact1_dc_dn6, var_qmfact1_dc_dn7, var_qmfact1_dc_dn8, var_qmfact1_dc_dn9,)
    } else {
        (var_qmfact1_ac, var_qmfact1_ac_dn4, var_qmfact1_ac_dn6, var_qmfact1_ac_dn7, var_qmfact1_ac_dn8, var_qmfact1_ac_dn9,)
    }
};
        var_qmfact1_ac = assign42080_e47619;
        var_qmfact1_ac_dn4 = assign42080_e47619_d_n4;
        var_qmfact1_ac_dn6 = assign42080_e47619_d_n6;
        var_qmfact1_ac_dn7 = assign42080_e47619_d_n7;
        var_qmfact1_ac_dn8 = assign42080_e47619_d_n8;
        var_qmfact1_ac_dn9 = assign42080_e47619_d_n9;

        let (assign42090_e47624, assign42090_e47624_d_n4, assign42090_e47624_d_n6, assign42090_e47624_d_n7, assign42090_e47624_d_n8, assign42090_e47624_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_qmfact2_dc, var_qmfact2_dc_dn4, var_qmfact2_dc_dn6, var_qmfact2_dc_dn7, var_qmfact2_dc_dn8, var_qmfact2_dc_dn9,)
    } else {
        (var_qmfact2_ac, var_qmfact2_ac_dn4, var_qmfact2_ac_dn6, var_qmfact2_ac_dn7, var_qmfact2_ac_dn8, var_qmfact2_ac_dn9,)
    }
};
        var_qmfact2_ac = assign42090_e47624;
        var_qmfact2_ac_dn4 = assign42090_e47624_d_n4;
        var_qmfact2_ac_dn6 = assign42090_e47624_d_n6;
        var_qmfact2_ac_dn7 = assign42090_e47624_d_n7;
        var_qmfact2_ac_dn8 = assign42090_e47624_d_n8;
        var_qmfact2_ac_dn9 = assign42090_e47624_d_n9;

        let (assign42100_e47629, assign42100_e47629_d_n4, assign42100_e47629_d_n6, assign42100_e47629_d_n7, assign42100_e47629_d_n8, assign42100_e47629_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_delta_k1q1_dc, var_delta_k1q1_dc_dn4, var_delta_k1q1_dc_dn6, var_delta_k1q1_dc_dn7, var_delta_k1q1_dc_dn8, var_delta_k1q1_dc_dn9,)
    } else {
        (var_delta_k1q1_ac, var_delta_k1q1_ac_dn4, var_delta_k1q1_ac_dn6, var_delta_k1q1_ac_dn7, var_delta_k1q1_ac_dn8, var_delta_k1q1_ac_dn9,)
    }
};
        var_delta_k1q1_ac = assign42100_e47629;
        var_delta_k1q1_ac_dn4 = assign42100_e47629_d_n4;
        var_delta_k1q1_ac_dn6 = assign42100_e47629_d_n6;
        var_delta_k1q1_ac_dn7 = assign42100_e47629_d_n7;
        var_delta_k1q1_ac_dn8 = assign42100_e47629_d_n8;
        var_delta_k1q1_ac_dn9 = assign42100_e47629_d_n9;

        let (assign42110_e47634, assign42110_e47634_d_n4, assign42110_e47634_d_n6, assign42110_e47634_d_n7, assign42110_e47634_d_n8, assign42110_e47634_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_delta_k2q2_dc, var_delta_k2q2_dc_dn4, var_delta_k2q2_dc_dn6, var_delta_k2q2_dc_dn7, var_delta_k2q2_dc_dn8, var_delta_k2q2_dc_dn9,)
    } else {
        (var_delta_k2q2_ac, var_delta_k2q2_ac_dn4, var_delta_k2q2_ac_dn6, var_delta_k2q2_ac_dn7, var_delta_k2q2_ac_dn8, var_delta_k2q2_ac_dn9,)
    }
};
        var_delta_k2q2_ac = assign42110_e47634;
        var_delta_k2q2_ac_dn4 = assign42110_e47634_d_n4;
        var_delta_k2q2_ac_dn6 = assign42110_e47634_d_n6;
        var_delta_k2q2_ac_dn7 = assign42110_e47634_d_n7;
        var_delta_k2q2_ac_dn8 = assign42110_e47634_d_n8;
        var_delta_k2q2_ac_dn9 = assign42110_e47634_d_n9;

        let (assign42120_e47639, assign42120_e47639_d_n4, assign42120_e47639_d_n6, assign42120_e47639_d_n7, assign42120_e47639_d_n8, assign42120_e47639_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_prod1_dc, var_prod1_dc_dn4, var_prod1_dc_dn6, var_prod1_dc_dn7, var_prod1_dc_dn8, var_prod1_dc_dn9,)
    } else {
        (var_prod1_ac, var_prod1_ac_dn4, var_prod1_ac_dn6, var_prod1_ac_dn7, var_prod1_ac_dn8, var_prod1_ac_dn9,)
    }
};
        var_prod1_ac = assign42120_e47639;
        var_prod1_ac_dn4 = assign42120_e47639_d_n4;
        var_prod1_ac_dn6 = assign42120_e47639_d_n6;
        var_prod1_ac_dn7 = assign42120_e47639_d_n7;
        var_prod1_ac_dn8 = assign42120_e47639_d_n8;
        var_prod1_ac_dn9 = assign42120_e47639_d_n9;

        let (assign42130_e47644, assign42130_e47644_d_n4, assign42130_e47644_d_n6, assign42130_e47644_d_n7, assign42130_e47644_d_n8, assign42130_e47644_d_n9,) = {
    if (var_guard1080 == 0.0) {
        (var_prod2_dc, var_prod2_dc_dn4, var_prod2_dc_dn6, var_prod2_dc_dn7, var_prod2_dc_dn8, var_prod2_dc_dn9,)
    } else {
        (var_prod2_ac, var_prod2_ac_dn4, var_prod2_ac_dn6, var_prod2_ac_dn7, var_prod2_ac_dn8, var_prod2_ac_dn9,)
    }
};
        var_prod2_ac = assign42130_e47644;
        var_prod2_ac_dn4 = assign42130_e47644_d_n4;
        var_prod2_ac_dn6 = assign42130_e47644_d_n6;
        var_prod2_ac_dn7 = assign42130_e47644_d_n7;
        var_prod2_ac_dn8 = assign42130_e47644_d_n8;
        var_prod2_ac_dn9 = assign42130_e47644_d_n9;

        let assign42140_e47648: f64 = (var_dx_wi_1d_ac - var_dx_wi_ac);
        let assign42140_e47649: f64 = (var_fsceac_i * assign42140_e47648);
        let assign42140_e47653: f64 = (0.25 * var_qim_ac);
        let assign42140_e47654: f64 = (1.0 + assign42140_e47653);
        let assign42140_e47655: f64 = (assign42140_e47649 / assign42140_e47654);
        var_temp = assign42140_e47655;
        var_temp_dn4 = ((((var_fsceac_i * (var_dx_wi_1d_ac_dn4 - var_dx_wi_ac_dn4)) * assign42140_e47654) - (assign42140_e47649 * (0.25 * var_qim_ac_dn4))) / (assign42140_e47654 * assign42140_e47654));
        var_temp_dn6 = ((((var_fsceac_i * (var_dx_wi_1d_ac_dn6 - var_dx_wi_ac_dn6)) * assign42140_e47654) - (assign42140_e47649 * (0.25 * var_qim_ac_dn6))) / (assign42140_e47654 * assign42140_e47654));
        var_temp_dn7 = ((((var_fsceac_i * (var_dx_wi_1d_ac_dn7 - var_dx_wi_ac_dn7)) * assign42140_e47654) - (assign42140_e47649 * (0.25 * var_qim_ac_dn7))) / (assign42140_e47654 * assign42140_e47654));
        var_temp_dn8 = ((((var_fsceac_i * (var_dx_wi_1d_ac_dn8 - var_dx_wi_ac_dn8)) * assign42140_e47654) - (assign42140_e47649 * (0.25 * var_qim_ac_dn8))) / (assign42140_e47654 * assign42140_e47654));
        var_temp_dn9 = ((((var_fsceac_i * (var_dx_wi_1d_ac_dn9 - var_dx_wi_ac_dn9)) * assign42140_e47654) - (assign42140_e47649 * (0.25 * var_qim_ac_dn9))) / (assign42140_e47654 * assign42140_e47654));

        let assign42150_e47659: f64 = (var_k1q1s_ac + var_k1q1d_ac);
        let assign42150_e47660: f64 = (0.5 * assign42150_e47659);
        let assign42150_e47662: f64 = (assign42150_e47660 + var_temp);
        var_k1q1m = assign42150_e47662;
        var_k1q1m_dn4 = ((0.5 * (var_k1q1s_ac_dn4 + var_k1q1d_ac_dn4)) + var_temp_dn4);
        var_k1q1m_dn6 = ((0.5 * (var_k1q1s_ac_dn6 + var_k1q1d_ac_dn6)) + var_temp_dn6);
        var_k1q1m_dn7 = ((0.5 * (var_k1q1s_ac_dn7 + var_k1q1d_ac_dn7)) + var_temp_dn7);
        var_k1q1m_dn8 = ((0.5 * (var_k1q1s_ac_dn8 + var_k1q1d_ac_dn8)) + var_temp_dn8);
        var_k1q1m_dn9 = ((0.5 * (var_k1q1s_ac_dn9 + var_k1q1d_ac_dn9)) + var_temp_dn9);

        let assign42160_e47666: f64 = (var_k2q2s_ac + var_k2q2d_ac);
        let assign42160_e47667: f64 = (0.5 * assign42160_e47666);
        let assign42160_e47669: f64 = (assign42160_e47667 - var_temp);
        var_k2q2m = assign42160_e47669;
        var_k2q2m_dn4 = ((0.5 * (var_k2q2s_ac_dn4 + var_k2q2d_ac_dn4)) - var_temp_dn4);
        var_k2q2m_dn6 = ((0.5 * (var_k2q2s_ac_dn6 + var_k2q2d_ac_dn6)) - var_temp_dn6);
        var_k2q2m_dn7 = ((0.5 * (var_k2q2s_ac_dn7 + var_k2q2d_ac_dn7)) - var_temp_dn7);
        var_k2q2m_dn8 = ((0.5 * (var_k2q2s_ac_dn8 + var_k2q2d_ac_dn8)) - var_temp_dn8);
        var_k2q2m_dn9 = ((0.5 * (var_k2q2s_ac_dn9 + var_k2q2d_ac_dn9)) - var_temp_dn9);

        let assign42170_e47672: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        var_guard1234 = assign42170_e47672;

        let (assign42180_e47682, assign42180_e47682_d_n4, assign42180_e47682_d_n6, assign42180_e47682_d_n7, assign42180_e47682_d_n8, assign42180_e47682_d_n9,) = {
    if (var_guard1234 != 0.0) {
        let assign42180_e47677: f64 = (var_qi1m_ac / var_qmfact1_ac);
        let assign42180_e47678: f64 = (var_k1q1m + assign42180_e47677);
        let assign42180_e47680: f64 = (assign42180_e47678 - var_qi1m_ac);
        (assign42180_e47680, ((var_k1q1m_dn4 + (((var_qi1m_ac_dn4 * var_qmfact1_ac) - (var_qi1m_ac * var_qmfact1_ac_dn4)) / (var_qmfact1_ac * var_qmfact1_ac))) - var_qi1m_ac_dn4), ((var_k1q1m_dn6 + (((var_qi1m_ac_dn6 * var_qmfact1_ac) - (var_qi1m_ac * var_qmfact1_ac_dn6)) / (var_qmfact1_ac * var_qmfact1_ac))) - var_qi1m_ac_dn6), ((var_k1q1m_dn7 + (((var_qi1m_ac_dn7 * var_qmfact1_ac) - (var_qi1m_ac * var_qmfact1_ac_dn7)) / (var_qmfact1_ac * var_qmfact1_ac))) - var_qi1m_ac_dn7), ((var_k1q1m_dn8 + (((var_qi1m_ac_dn8 * var_qmfact1_ac) - (var_qi1m_ac * var_qmfact1_ac_dn8)) / (var_qmfact1_ac * var_qmfact1_ac))) - var_qi1m_ac_dn8), ((var_k1q1m_dn9 + (((var_qi1m_ac_dn9 * var_qmfact1_ac) - (var_qi1m_ac * var_qmfact1_ac_dn9)) / (var_qmfact1_ac * var_qmfact1_ac))) - var_qi1m_ac_dn9),)
    } else {
        (var_k1q1eff, var_k1q1eff_dn4, var_k1q1eff_dn6, var_k1q1eff_dn7, var_k1q1eff_dn8, var_k1q1eff_dn9,)
    }
};
        var_k1q1eff = assign42180_e47682;
        var_k1q1eff_dn4 = assign42180_e47682_d_n4;
        var_k1q1eff_dn6 = assign42180_e47682_d_n6;
        var_k1q1eff_dn7 = assign42180_e47682_d_n7;
        var_k1q1eff_dn8 = assign42180_e47682_d_n8;
        var_k1q1eff_dn9 = assign42180_e47682_d_n9;

        let (assign42190_e47692, assign42190_e47692_d_n4, assign42190_e47692_d_n6, assign42190_e47692_d_n7, assign42190_e47692_d_n8, assign42190_e47692_d_n9,) = {
    if (var_guard1234 != 0.0) {
        let assign42190_e47687: f64 = (var_qi2m_ac / var_qmfact2_ac);
        let assign42190_e47688: f64 = (var_k2q2m + assign42190_e47687);
        let assign42190_e47690: f64 = (assign42190_e47688 - var_qi2m_ac);
        (assign42190_e47690, ((var_k2q2m_dn4 + (((var_qi2m_ac_dn4 * var_qmfact2_ac) - (var_qi2m_ac * var_qmfact2_ac_dn4)) / (var_qmfact2_ac * var_qmfact2_ac))) - var_qi2m_ac_dn4), ((var_k2q2m_dn6 + (((var_qi2m_ac_dn6 * var_qmfact2_ac) - (var_qi2m_ac * var_qmfact2_ac_dn6)) / (var_qmfact2_ac * var_qmfact2_ac))) - var_qi2m_ac_dn6), ((var_k2q2m_dn7 + (((var_qi2m_ac_dn7 * var_qmfact2_ac) - (var_qi2m_ac * var_qmfact2_ac_dn7)) / (var_qmfact2_ac * var_qmfact2_ac))) - var_qi2m_ac_dn7), ((var_k2q2m_dn8 + (((var_qi2m_ac_dn8 * var_qmfact2_ac) - (var_qi2m_ac * var_qmfact2_ac_dn8)) / (var_qmfact2_ac * var_qmfact2_ac))) - var_qi2m_ac_dn8), ((var_k2q2m_dn9 + (((var_qi2m_ac_dn9 * var_qmfact2_ac) - (var_qi2m_ac * var_qmfact2_ac_dn9)) / (var_qmfact2_ac * var_qmfact2_ac))) - var_qi2m_ac_dn9),)
    } else {
        (var_k2q2eff, var_k2q2eff_dn4, var_k2q2eff_dn6, var_k2q2eff_dn7, var_k2q2eff_dn8, var_k2q2eff_dn9,)
    }
};
        var_k2q2eff = assign42190_e47692;
        var_k2q2eff_dn4 = assign42190_e47692_d_n4;
        var_k2q2eff_dn6 = assign42190_e47692_d_n6;
        var_k2q2eff_dn7 = assign42190_e47692_d_n7;
        var_k2q2eff_dn8 = assign42190_e47692_d_n8;
        var_k2q2eff_dn9 = assign42190_e47692_d_n9;

        let (assign42200_e47697, assign42200_e47697_d_n4, assign42200_e47697_d_n6, assign42200_e47697_d_n7, assign42200_e47697_d_n8, assign42200_e47697_d_n9,) = {
    if (var_guard1234 == 0.0) {
        (var_k1q1m, var_k1q1m_dn4, var_k1q1m_dn6, var_k1q1m_dn7, var_k1q1m_dn8, var_k1q1m_dn9,)
    } else {
        (var_k1q1eff, var_k1q1eff_dn4, var_k1q1eff_dn6, var_k1q1eff_dn7, var_k1q1eff_dn8, var_k1q1eff_dn9,)
    }
};
        var_k1q1eff = assign42200_e47697;
        var_k1q1eff_dn4 = assign42200_e47697_d_n4;
        var_k1q1eff_dn6 = assign42200_e47697_d_n6;
        var_k1q1eff_dn7 = assign42200_e47697_d_n7;
        var_k1q1eff_dn8 = assign42200_e47697_d_n8;
        var_k1q1eff_dn9 = assign42200_e47697_d_n9;

        let (assign42210_e47702, assign42210_e47702_d_n4, assign42210_e47702_d_n6, assign42210_e47702_d_n7, assign42210_e47702_d_n8, assign42210_e47702_d_n9,) = {
    if (var_guard1234 == 0.0) {
        (var_k2q2m, var_k2q2m_dn4, var_k2q2m_dn6, var_k2q2m_dn7, var_k2q2m_dn8, var_k2q2m_dn9,)
    } else {
        (var_k2q2eff, var_k2q2eff_dn4, var_k2q2eff_dn6, var_k2q2eff_dn7, var_k2q2eff_dn8, var_k2q2eff_dn9,)
    }
};
        var_k2q2eff = assign42210_e47702;
        var_k2q2eff_dn4 = assign42210_e47702_d_n4;
        var_k2q2eff_dn6 = assign42210_e47702_d_n6;
        var_k2q2eff_dn7 = assign42210_e47702_d_n7;
        var_k2q2eff_dn8 = assign42210_e47702_d_n8;
        var_k2q2eff_dn9 = assign42210_e47702_d_n9;

        let assign42220_e47705: f64 = (var_delta_k1q1_ac * var_prod1_ac);
        let assign42220_e47707: f64 = (assign42220_e47705 * 0.3333333333333);
        var_temp1 = assign42220_e47707;
        var_temp1_dn4 = (((var_delta_k1q1_ac_dn4 * var_prod1_ac) + (var_delta_k1q1_ac * var_prod1_ac_dn4)) * 0.3333333333333);
        var_temp1_dn6 = (((var_delta_k1q1_ac_dn6 * var_prod1_ac) + (var_delta_k1q1_ac * var_prod1_ac_dn6)) * 0.3333333333333);
        var_temp1_dn7 = (((var_delta_k1q1_ac_dn7 * var_prod1_ac) + (var_delta_k1q1_ac * var_prod1_ac_dn7)) * 0.3333333333333);
        var_temp1_dn8 = (((var_delta_k1q1_ac_dn8 * var_prod1_ac) + (var_delta_k1q1_ac * var_prod1_ac_dn8)) * 0.3333333333333);
        var_temp1_dn9 = (((var_delta_k1q1_ac_dn9 * var_prod1_ac) + (var_delta_k1q1_ac * var_prod1_ac_dn9)) * 0.3333333333333);

        let assign42230_e47710: f64 = (var_delta_k1q1_ac * 0.1666666666667);
        let assign42230_e47716: f64 = (0.2 * var_prod1_ac);
        let assign42230_e47717: f64 = (1.0 - assign42230_e47716);
        let assign42230_e47718: f64 = (var_prod1_ac * assign42230_e47717);
        let assign42230_e47719: f64 = (1.0 + assign42230_e47718);
        let assign42230_e47720: f64 = (assign42230_e47710 * assign42230_e47719);
        var_temp2 = assign42230_e47720;
        var_temp2_dn4 = (((var_delta_k1q1_ac_dn4 * 0.1666666666667) * assign42230_e47719) + (assign42230_e47710 * ((var_prod1_ac_dn4 * assign42230_e47717) + (var_prod1_ac * (-(0.2 * var_prod1_ac_dn4))))));
        var_temp2_dn6 = (((var_delta_k1q1_ac_dn6 * 0.1666666666667) * assign42230_e47719) + (assign42230_e47710 * ((var_prod1_ac_dn6 * assign42230_e47717) + (var_prod1_ac * (-(0.2 * var_prod1_ac_dn6))))));
        var_temp2_dn7 = (((var_delta_k1q1_ac_dn7 * 0.1666666666667) * assign42230_e47719) + (assign42230_e47710 * ((var_prod1_ac_dn7 * assign42230_e47717) + (var_prod1_ac * (-(0.2 * var_prod1_ac_dn7))))));
        var_temp2_dn8 = (((var_delta_k1q1_ac_dn8 * 0.1666666666667) * assign42230_e47719) + (assign42230_e47710 * ((var_prod1_ac_dn8 * assign42230_e47717) + (var_prod1_ac * (-(0.2 * var_prod1_ac_dn8))))));
        var_temp2_dn9 = (((var_delta_k1q1_ac_dn9 * 0.1666666666667) * assign42230_e47719) + (assign42230_e47710 * ((var_prod1_ac_dn9 * assign42230_e47717) + (var_prod1_ac * (-(0.2 * var_prod1_ac_dn9))))));

        let assign42240_e47723: f64 = (0.5 * var_k1q1eff);
        let assign42240_e47725: f64 = (assign42240_e47723 * var_ratio_pd_ac);
        let assign42240_e47727: f64 = (assign42240_e47725 + var_temp2);
        var_k1q1deff = assign42240_e47727;
        var_k1q1deff_dn4 = ((((0.5 * var_k1q1eff_dn4) * var_ratio_pd_ac) + (assign42240_e47723 * var_ratio_pd_ac_dn4)) + var_temp2_dn4);
        var_k1q1deff_dn6 = ((((0.5 * var_k1q1eff_dn6) * var_ratio_pd_ac) + (assign42240_e47723 * var_ratio_pd_ac_dn6)) + var_temp2_dn6);
        var_k1q1deff_dn7 = ((((0.5 * var_k1q1eff_dn7) * var_ratio_pd_ac) + (assign42240_e47723 * var_ratio_pd_ac_dn7)) + var_temp2_dn7);
        var_k1q1deff_dn8 = ((((0.5 * var_k1q1eff_dn8) * var_ratio_pd_ac) + (assign42240_e47723 * var_ratio_pd_ac_dn8)) + var_temp2_dn8);
        var_k1q1deff_dn9 = ((((0.5 * var_k1q1eff_dn9) * var_ratio_pd_ac) + (assign42240_e47723 * var_ratio_pd_ac_dn9)) + var_temp2_dn9);

        let assign42250_e47730: f64 = (var_k1q1eff * var_ratio_pd_ac);
        let assign42250_e47732: f64 = (assign42250_e47730 + var_temp1);
        var_k1q1eff = assign42250_e47732;
        var_k1q1eff_dn4 = (((var_k1q1eff_dn4 * var_ratio_pd_ac) + (var_k1q1eff * var_ratio_pd_ac_dn4)) + var_temp1_dn4);
        var_k1q1eff_dn6 = (((var_k1q1eff_dn6 * var_ratio_pd_ac) + (var_k1q1eff * var_ratio_pd_ac_dn6)) + var_temp1_dn6);
        var_k1q1eff_dn7 = (((var_k1q1eff_dn7 * var_ratio_pd_ac) + (var_k1q1eff * var_ratio_pd_ac_dn7)) + var_temp1_dn7);
        var_k1q1eff_dn8 = (((var_k1q1eff_dn8 * var_ratio_pd_ac) + (var_k1q1eff * var_ratio_pd_ac_dn8)) + var_temp1_dn8);
        var_k1q1eff_dn9 = (((var_k1q1eff_dn9 * var_ratio_pd_ac) + (var_k1q1eff * var_ratio_pd_ac_dn9)) + var_temp1_dn9);

        let assign42260_e47735: f64 = (var_delta_k2q2_ac * var_prod2_ac);
        let assign42260_e47737: f64 = (assign42260_e47735 * 0.3333333333333);
        var_temp1 = assign42260_e47737;
        var_temp1_dn4 = (((var_delta_k2q2_ac_dn4 * var_prod2_ac) + (var_delta_k2q2_ac * var_prod2_ac_dn4)) * 0.3333333333333);
        var_temp1_dn6 = (((var_delta_k2q2_ac_dn6 * var_prod2_ac) + (var_delta_k2q2_ac * var_prod2_ac_dn6)) * 0.3333333333333);
        var_temp1_dn7 = (((var_delta_k2q2_ac_dn7 * var_prod2_ac) + (var_delta_k2q2_ac * var_prod2_ac_dn7)) * 0.3333333333333);
        var_temp1_dn8 = (((var_delta_k2q2_ac_dn8 * var_prod2_ac) + (var_delta_k2q2_ac * var_prod2_ac_dn8)) * 0.3333333333333);
        var_temp1_dn9 = (((var_delta_k2q2_ac_dn9 * var_prod2_ac) + (var_delta_k2q2_ac * var_prod2_ac_dn9)) * 0.3333333333333);

        let assign42270_e47740: f64 = (var_delta_k2q2_ac * 0.1666666666667);
        let assign42270_e47746: f64 = (0.2 * var_prod2_ac);
        let assign42270_e47747: f64 = (1.0 - assign42270_e47746);
        let assign42270_e47748: f64 = (var_prod2_ac * assign42270_e47747);
        let assign42270_e47749: f64 = (1.0 + assign42270_e47748);
        let assign42270_e47750: f64 = (assign42270_e47740 * assign42270_e47749);
        var_temp2 = assign42270_e47750;
        var_temp2_dn4 = (((var_delta_k2q2_ac_dn4 * 0.1666666666667) * assign42270_e47749) + (assign42270_e47740 * ((var_prod2_ac_dn4 * assign42270_e47747) + (var_prod2_ac * (-(0.2 * var_prod2_ac_dn4))))));
        var_temp2_dn6 = (((var_delta_k2q2_ac_dn6 * 0.1666666666667) * assign42270_e47749) + (assign42270_e47740 * ((var_prod2_ac_dn6 * assign42270_e47747) + (var_prod2_ac * (-(0.2 * var_prod2_ac_dn6))))));
        var_temp2_dn7 = (((var_delta_k2q2_ac_dn7 * 0.1666666666667) * assign42270_e47749) + (assign42270_e47740 * ((var_prod2_ac_dn7 * assign42270_e47747) + (var_prod2_ac * (-(0.2 * var_prod2_ac_dn7))))));
        var_temp2_dn8 = (((var_delta_k2q2_ac_dn8 * 0.1666666666667) * assign42270_e47749) + (assign42270_e47740 * ((var_prod2_ac_dn8 * assign42270_e47747) + (var_prod2_ac * (-(0.2 * var_prod2_ac_dn8))))));
        var_temp2_dn9 = (((var_delta_k2q2_ac_dn9 * 0.1666666666667) * assign42270_e47749) + (assign42270_e47740 * ((var_prod2_ac_dn9 * assign42270_e47747) + (var_prod2_ac * (-(0.2 * var_prod2_ac_dn9))))));

        let assign42280_e47753: f64 = (0.5 * var_k2q2eff);
        let assign42280_e47755: f64 = (assign42280_e47753 + var_temp2);
        var_k2q2deff = assign42280_e47755;
        var_k2q2deff_dn4 = ((0.5 * var_k2q2eff_dn4) + var_temp2_dn4);
        var_k2q2deff_dn6 = ((0.5 * var_k2q2eff_dn6) + var_temp2_dn6);
        var_k2q2deff_dn7 = ((0.5 * var_k2q2eff_dn7) + var_temp2_dn7);
        var_k2q2deff_dn8 = ((0.5 * var_k2q2eff_dn8) + var_temp2_dn8);
        var_k2q2deff_dn9 = ((0.5 * var_k2q2eff_dn9) + var_temp2_dn9);

        let assign42290_e47758: f64 = (var_k2q2eff + var_temp1);
        var_k2q2eff = assign42290_e47758;
        var_k2q2eff_dn4 = (var_k2q2eff_dn4 + var_temp1_dn4);
        var_k2q2eff_dn6 = (var_k2q2eff_dn6 + var_temp1_dn6);
        var_k2q2eff_dn7 = (var_k2q2eff_dn7 + var_temp1_dn7);
        var_k2q2eff_dn8 = (var_k2q2eff_dn8 + var_temp1_dn8);
        var_k2q2eff_dn9 = (var_k2q2eff_dn9 + var_temp1_dn9);

        let assign42300_e47761: f64 = (var_csiprime_ac * var_area_phit);
        var_temp = assign42300_e47761;
        var_temp_dn4 = ((var_csiprime_ac_dn4 * var_area_phit) + (var_csiprime_ac * var_area_phit_dn4));
        var_temp_dn6 = ((var_csiprime_ac_dn6 * var_area_phit) + (var_csiprime_ac * var_area_phit_dn6));
        var_temp_dn7 = ((var_csiprime_ac_dn7 * var_area_phit) + (var_csiprime_ac * var_area_phit_dn7));
        var_temp_dn8 = ((var_csiprime_ac_dn8 * var_area_phit) + (var_csiprime_ac * var_area_phit_dn8));
        var_temp_dn9 = ((var_csiprime_ac_dn9 * var_area_phit) + (var_csiprime_ac * var_area_phit_dn9));

        let assign42310_e47764: f64 = (var_temp * var_k1q1eff);
        var_qg = assign42310_e47764;
        var_qg_dn4 = ((var_temp_dn4 * var_k1q1eff) + (var_temp * var_k1q1eff_dn4));
        var_qg_dn6 = ((var_temp_dn6 * var_k1q1eff) + (var_temp * var_k1q1eff_dn6));
        var_qg_dn7 = ((var_temp_dn7 * var_k1q1eff) + (var_temp * var_k1q1eff_dn7));
        var_qg_dn8 = ((var_temp_dn8 * var_k1q1eff) + (var_temp * var_k1q1eff_dn8));
        var_qg_dn9 = ((var_temp_dn9 * var_k1q1eff) + (var_temp * var_k1q1eff_dn9));

        let assign42320_e47767: f64 = (var_temp * var_k2q2eff);
        var_qb = assign42320_e47767;
        var_qb_dn4 = ((var_temp_dn4 * var_k2q2eff) + (var_temp * var_k2q2eff_dn4));
        var_qb_dn6 = ((var_temp_dn6 * var_k2q2eff) + (var_temp * var_k2q2eff_dn6));
        var_qb_dn7 = ((var_temp_dn7 * var_k2q2eff) + (var_temp * var_k2q2eff_dn7));
        var_qb_dn8 = ((var_temp_dn8 * var_k2q2eff) + (var_temp * var_k2q2eff_dn8));
        var_qb_dn9 = ((var_temp_dn9 * var_k2q2eff) + (var_temp * var_k2q2eff_dn9));

        let assign42330_e47769: f64 = (-var_temp);
        let assign42330_e47772: f64 = (var_k1q1deff + var_k2q2deff);
        let assign42330_e47773: f64 = (assign42330_e47769 * assign42330_e47772);
        var_qd = assign42330_e47773;
        var_qd_dn4 = (((-var_temp_dn4) * assign42330_e47772) + (assign42330_e47769 * (var_k1q1deff_dn4 + var_k2q2deff_dn4)));
        var_qd_dn6 = (((-var_temp_dn6) * assign42330_e47772) + (assign42330_e47769 * (var_k1q1deff_dn6 + var_k2q2deff_dn6)));
        var_qd_dn7 = (((-var_temp_dn7) * assign42330_e47772) + (assign42330_e47769 * (var_k1q1deff_dn7 + var_k2q2deff_dn7)));
        var_qd_dn8 = (((-var_temp_dn8) * assign42330_e47772) + (assign42330_e47769 * (var_k1q1deff_dn8 + var_k2q2deff_dn8)));
        var_qd_dn9 = (((-var_temp_dn9) * assign42330_e47772) + (assign42330_e47769 * (var_k1q1deff_dn9 + var_k2q2deff_dn9)));

        let assign42340_e47776: f64 = if var_fif_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1235 = assign42340_e47776;

        let (assign42350_e47784, assign42350_e47784_d_n4, assign42350_e47784_d_n6, assign42350_e47784_d_n7, assign42350_e47784_d_n8, assign42350_e47784_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42350_e47781: f64 = (2.0 * 0.6931471805599);
        let assign42350_e47782: f64 = (var_xth_1d + assign42350_e47781);
        (assign42350_e47782, var_xth_1d_dn4, var_xth_1d_dn6, var_xth_1d_dn7, var_xth_1d_dn8, var_xth_1d_dn9,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign42350_e47784;
        var_temp_dn4 = assign42350_e47784_d_n4;
        var_temp_dn6 = assign42350_e47784_d_n6;
        var_temp_dn7 = assign42350_e47784_d_n7;
        var_temp_dn8 = assign42350_e47784_d_n8;
        var_temp_dn9 = assign42350_e47784_d_n9;

        let (assign42360_e47790, assign42360_e47790_d_n4, assign42360_e47790_d_n6, assign42360_e47790_d_n7, assign42360_e47790_d_n8, assign42360_e47790_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42360_e47788: f64 = (var_xdrifts_ac + var_temp);
        (assign42360_e47788, (var_xdrifts_ac_dn4 + var_temp_dn4), (var_xdrifts_ac_dn6 + var_temp_dn6), (var_xdrifts_ac_dn7 + var_temp_dn7), (var_xdrifts_ac_dn8 + var_temp_dn8), (var_xdrifts_ac_dn9 + var_temp_dn9),)
    } else {
        (var_xeffs, var_xeffs_dn4, var_xeffs_dn6, var_xeffs_dn7, var_xeffs_dn8, var_xeffs_dn9,)
    }
};
        var_xeffs = assign42360_e47790;
        var_xeffs_dn4 = assign42360_e47790_d_n4;
        var_xeffs_dn6 = assign42360_e47790_d_n6;
        var_xeffs_dn7 = assign42360_e47790_d_n7;
        var_xeffs_dn8 = assign42360_e47790_d_n8;
        var_xeffs_dn9 = assign42360_e47790_d_n9;

        *var_delta_k1q1_ac_slot = var_delta_k1q1_ac;
        *var_delta_k1q1_ac_dn4_slot = var_delta_k1q1_ac_dn4;
        *var_delta_k1q1_ac_dn6_slot = var_delta_k1q1_ac_dn6;
        *var_delta_k1q1_ac_dn7_slot = var_delta_k1q1_ac_dn7;
        *var_delta_k1q1_ac_dn8_slot = var_delta_k1q1_ac_dn8;
        *var_delta_k1q1_ac_dn9_slot = var_delta_k1q1_ac_dn9;
        *var_delta_k2q2_ac_slot = var_delta_k2q2_ac;
        *var_delta_k2q2_ac_dn4_slot = var_delta_k2q2_ac_dn4;
        *var_delta_k2q2_ac_dn6_slot = var_delta_k2q2_ac_dn6;
        *var_delta_k2q2_ac_dn7_slot = var_delta_k2q2_ac_dn7;
        *var_delta_k2q2_ac_dn8_slot = var_delta_k2q2_ac_dn8;
        *var_delta_k2q2_ac_dn9_slot = var_delta_k2q2_ac_dn9;
        *var_guard1234_slot = var_guard1234;
        *var_guard1235_slot = var_guard1235;
        *var_k1_ac_slot = var_k1_ac;
        *var_k1_ac_dn4_slot = var_k1_ac_dn4;
        *var_k1_ac_dn6_slot = var_k1_ac_dn6;
        *var_k1_ac_dn7_slot = var_k1_ac_dn7;
        *var_k1_ac_dn8_slot = var_k1_ac_dn8;
        *var_k1_ac_dn9_slot = var_k1_ac_dn9;
        *var_k1q1d_ac_slot = var_k1q1d_ac;
        *var_k1q1d_ac_dn4_slot = var_k1q1d_ac_dn4;
        *var_k1q1d_ac_dn6_slot = var_k1q1d_ac_dn6;
        *var_k1q1d_ac_dn7_slot = var_k1q1d_ac_dn7;
        *var_k1q1d_ac_dn8_slot = var_k1q1d_ac_dn8;
        *var_k1q1d_ac_dn9_slot = var_k1q1d_ac_dn9;
        *var_k1q1deff_slot = var_k1q1deff;
        *var_k1q1deff_dn4_slot = var_k1q1deff_dn4;
        *var_k1q1deff_dn6_slot = var_k1q1deff_dn6;
        *var_k1q1deff_dn7_slot = var_k1q1deff_dn7;
        *var_k1q1deff_dn8_slot = var_k1q1deff_dn8;
        *var_k1q1deff_dn9_slot = var_k1q1deff_dn9;
        *var_k1q1eff_slot = var_k1q1eff;
        *var_k1q1eff_dn4_slot = var_k1q1eff_dn4;
        *var_k1q1eff_dn6_slot = var_k1q1eff_dn6;
        *var_k1q1eff_dn7_slot = var_k1q1eff_dn7;
        *var_k1q1eff_dn8_slot = var_k1q1eff_dn8;
        *var_k1q1eff_dn9_slot = var_k1q1eff_dn9;
        *var_k1q1m_slot = var_k1q1m;
        *var_k1q1m_dn4_slot = var_k1q1m_dn4;
        *var_k1q1m_dn6_slot = var_k1q1m_dn6;
        *var_k1q1m_dn7_slot = var_k1q1m_dn7;
        *var_k1q1m_dn8_slot = var_k1q1m_dn8;
        *var_k1q1m_dn9_slot = var_k1q1m_dn9;
        *var_k1q1s_ac_slot = var_k1q1s_ac;
        *var_k1q1s_ac_dn4_slot = var_k1q1s_ac_dn4;
        *var_k1q1s_ac_dn6_slot = var_k1q1s_ac_dn6;
        *var_k1q1s_ac_dn7_slot = var_k1q1s_ac_dn7;
        *var_k1q1s_ac_dn8_slot = var_k1q1s_ac_dn8;
        *var_k1q1s_ac_dn9_slot = var_k1q1s_ac_dn9;
        *var_k2_ac_slot = var_k2_ac;
        *var_k2_ac_dn4_slot = var_k2_ac_dn4;
        *var_k2_ac_dn6_slot = var_k2_ac_dn6;
        *var_k2_ac_dn7_slot = var_k2_ac_dn7;
        *var_k2_ac_dn8_slot = var_k2_ac_dn8;
        *var_k2_ac_dn9_slot = var_k2_ac_dn9;
        *var_k2q2d_ac_slot = var_k2q2d_ac;
        *var_k2q2d_ac_dn4_slot = var_k2q2d_ac_dn4;
        *var_k2q2d_ac_dn6_slot = var_k2q2d_ac_dn6;
        *var_k2q2d_ac_dn7_slot = var_k2q2d_ac_dn7;
        *var_k2q2d_ac_dn8_slot = var_k2q2d_ac_dn8;
        *var_k2q2d_ac_dn9_slot = var_k2q2d_ac_dn9;
        *var_k2q2deff_slot = var_k2q2deff;
        *var_k2q2deff_dn4_slot = var_k2q2deff_dn4;
        *var_k2q2deff_dn6_slot = var_k2q2deff_dn6;
        *var_k2q2deff_dn7_slot = var_k2q2deff_dn7;
        *var_k2q2deff_dn8_slot = var_k2q2deff_dn8;
        *var_k2q2deff_dn9_slot = var_k2q2deff_dn9;
        *var_k2q2eff_slot = var_k2q2eff;
        *var_k2q2eff_dn4_slot = var_k2q2eff_dn4;
        *var_k2q2eff_dn6_slot = var_k2q2eff_dn6;
        *var_k2q2eff_dn7_slot = var_k2q2eff_dn7;
        *var_k2q2eff_dn8_slot = var_k2q2eff_dn8;
        *var_k2q2eff_dn9_slot = var_k2q2eff_dn9;
        *var_k2q2m_slot = var_k2q2m;
        *var_k2q2m_dn4_slot = var_k2q2m_dn4;
        *var_k2q2m_dn6_slot = var_k2q2m_dn6;
        *var_k2q2m_dn7_slot = var_k2q2m_dn7;
        *var_k2q2m_dn8_slot = var_k2q2m_dn8;
        *var_k2q2m_dn9_slot = var_k2q2m_dn9;
        *var_k2q2s_ac_slot = var_k2q2s_ac;
        *var_k2q2s_ac_dn4_slot = var_k2q2s_ac_dn4;
        *var_k2q2s_ac_dn6_slot = var_k2q2s_ac_dn6;
        *var_k2q2s_ac_dn7_slot = var_k2q2s_ac_dn7;
        *var_k2q2s_ac_dn8_slot = var_k2q2s_ac_dn8;
        *var_k2q2s_ac_dn9_slot = var_k2q2s_ac_dn9;
        *var_prod1_ac_slot = var_prod1_ac;
        *var_prod1_ac_dn4_slot = var_prod1_ac_dn4;
        *var_prod1_ac_dn6_slot = var_prod1_ac_dn6;
        *var_prod1_ac_dn7_slot = var_prod1_ac_dn7;
        *var_prod1_ac_dn8_slot = var_prod1_ac_dn8;
        *var_prod1_ac_dn9_slot = var_prod1_ac_dn9;
        *var_prod2_ac_slot = var_prod2_ac;
        *var_prod2_ac_dn4_slot = var_prod2_ac_dn4;
        *var_prod2_ac_dn6_slot = var_prod2_ac_dn6;
        *var_prod2_ac_dn7_slot = var_prod2_ac_dn7;
        *var_prod2_ac_dn8_slot = var_prod2_ac_dn8;
        *var_prod2_ac_dn9_slot = var_prod2_ac_dn9;
        *var_qb_slot = var_qb;
        *var_qb_dn4_slot = var_qb_dn4;
        *var_qb_dn6_slot = var_qb_dn6;
        *var_qb_dn7_slot = var_qb_dn7;
        *var_qb_dn8_slot = var_qb_dn8;
        *var_qb_dn9_slot = var_qb_dn9;
        *var_qd_slot = var_qd;
        *var_qd_dn4_slot = var_qd_dn4;
        *var_qd_dn6_slot = var_qd_dn6;
        *var_qd_dn7_slot = var_qd_dn7;
        *var_qd_dn8_slot = var_qd_dn8;
        *var_qd_dn9_slot = var_qd_dn9;
        *var_qg_slot = var_qg;
        *var_qg_dn4_slot = var_qg_dn4;
        *var_qg_dn6_slot = var_qg_dn6;
        *var_qg_dn7_slot = var_qg_dn7;
        *var_qg_dn8_slot = var_qg_dn8;
        *var_qg_dn9_slot = var_qg_dn9;
        *var_qi1m_ac_slot = var_qi1m_ac;
        *var_qi1m_ac_dn4_slot = var_qi1m_ac_dn4;
        *var_qi1m_ac_dn6_slot = var_qi1m_ac_dn6;
        *var_qi1m_ac_dn7_slot = var_qi1m_ac_dn7;
        *var_qi1m_ac_dn8_slot = var_qi1m_ac_dn8;
        *var_qi1m_ac_dn9_slot = var_qi1m_ac_dn9;
        *var_qi2m_ac_slot = var_qi2m_ac;
        *var_qi2m_ac_dn4_slot = var_qi2m_ac_dn4;
        *var_qi2m_ac_dn6_slot = var_qi2m_ac_dn6;
        *var_qi2m_ac_dn7_slot = var_qi2m_ac_dn7;
        *var_qi2m_ac_dn8_slot = var_qi2m_ac_dn8;
        *var_qi2m_ac_dn9_slot = var_qi2m_ac_dn9;
        *var_qim_ac_slot = var_qim_ac;
        *var_qim_ac_dn4_slot = var_qim_ac_dn4;
        *var_qim_ac_dn6_slot = var_qim_ac_dn6;
        *var_qim_ac_dn7_slot = var_qim_ac_dn7;
        *var_qim_ac_dn8_slot = var_qim_ac_dn8;
        *var_qim_ac_dn9_slot = var_qim_ac_dn9;
        *var_qmfact1_ac_slot = var_qmfact1_ac;
        *var_qmfact1_ac_dn4_slot = var_qmfact1_ac_dn4;
        *var_qmfact1_ac_dn6_slot = var_qmfact1_ac_dn6;
        *var_qmfact1_ac_dn7_slot = var_qmfact1_ac_dn7;
        *var_qmfact1_ac_dn8_slot = var_qmfact1_ac_dn8;
        *var_qmfact1_ac_dn9_slot = var_qmfact1_ac_dn9;
        *var_qmfact2_ac_slot = var_qmfact2_ac;
        *var_qmfact2_ac_dn4_slot = var_qmfact2_ac_dn4;
        *var_qmfact2_ac_dn6_slot = var_qmfact2_ac_dn6;
        *var_qmfact2_ac_dn7_slot = var_qmfact2_ac_dn7;
        *var_qmfact2_ac_dn8_slot = var_qmfact2_ac_dn8;
        *var_qmfact2_ac_dn9_slot = var_qmfact2_ac_dn9;
        *var_ratio_pd_ac_slot = var_ratio_pd_ac;
        *var_ratio_pd_ac_dn4_slot = var_ratio_pd_ac_dn4;
        *var_ratio_pd_ac_dn6_slot = var_ratio_pd_ac_dn6;
        *var_ratio_pd_ac_dn7_slot = var_ratio_pd_ac_dn7;
        *var_ratio_pd_ac_dn8_slot = var_ratio_pd_ac_dn8;
        *var_ratio_pd_ac_dn9_slot = var_ratio_pd_ac_dn9;
        *var_temp_slot = var_temp;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_xdriftd_ac_slot = var_xdriftd_ac;
        *var_xdriftd_ac_dn4_slot = var_xdriftd_ac_dn4;
        *var_xdriftd_ac_dn6_slot = var_xdriftd_ac_dn6;
        *var_xdriftd_ac_dn7_slot = var_xdriftd_ac_dn7;
        *var_xdriftd_ac_dn8_slot = var_xdriftd_ac_dn8;
        *var_xdriftd_ac_dn9_slot = var_xdriftd_ac_dn9;
        *var_xdrifts_ac_slot = var_xdrifts_ac;
        *var_xdrifts_ac_dn4_slot = var_xdrifts_ac_dn4;
        *var_xdrifts_ac_dn6_slot = var_xdrifts_ac_dn6;
        *var_xdrifts_ac_dn7_slot = var_xdrifts_ac_dn7;
        *var_xdrifts_ac_dn8_slot = var_xdrifts_ac_dn8;
        *var_xdrifts_ac_dn9_slot = var_xdrifts_ac_dn9;
        *var_xeffs_slot = var_xeffs;
        *var_xeffs_dn4_slot = var_xeffs_dn4;
        *var_xeffs_dn6_slot = var_xeffs_dn6;
        *var_xeffs_dn7_slot = var_xeffs_dn7;
        *var_xeffs_dn8_slot = var_xeffs_dn8;
        *var_xeffs_dn9_slot = var_xeffs_dn9;
        *var_zsat_ac_slot = var_zsat_ac;
        *var_zsat_ac_dn4_slot = var_zsat_ac_dn4;
        *var_zsat_ac_dn6_slot = var_zsat_ac_dn6;
        *var_zsat_ac_dn7_slot = var_zsat_ac_dn7;
        *var_zsat_ac_dn8_slot = var_zsat_ac_dn8;
        *var_zsat_ac_dn9_slot = var_zsat_ac_dn9;
    }

    pub(super) fn stamp_transient_block_118(
        var_csiprime_ac: f64,
        var_csiprime_ac_dn4: f64,
        var_csiprime_ac_dn6: f64,
        var_csiprime_ac_dn7: f64,
        var_csiprime_ac_dn8: f64,
        var_csiprime_ac_dn9: f64,
        var_fif_phit: f64,
        var_fif_phit_dn4: f64,
        var_fif_phit_dn6: f64,
        var_fif_phit_dn7: f64,
        var_fif_phit_dn8: f64,
        var_fif_phit_dn9: f64,
        var_guard1235: f64,
        var_inner_sd: f64,
        var_inner_sd_dn4: f64,
        var_inner_sd_dn6: f64,
        var_inner_sd_dn7: f64,
        var_inner_sd_dn8: f64,
        var_inner_sd_dn9: f64,
        var_inv_k1_ac: f64,
        var_inv_k1_ac_dn4: f64,
        var_inv_k1_ac_dn6: f64,
        var_inv_k1_ac_dn7: f64,
        var_inv_k1_ac_dn8: f64,
        var_inv_k1_ac_dn9: f64,
        var_inv_k2_ac: f64,
        var_inv_k2_ac_dn4: f64,
        var_inv_k2_ac_dn6: f64,
        var_inv_k2_ac_dn7: f64,
        var_inv_k2_ac_dn8: f64,
        var_inv_k2_ac_dn9: f64,
        var_k1_ac: f64,
        var_k1_ac_dn4: f64,
        var_k1_ac_dn6: f64,
        var_k1_ac_dn7: f64,
        var_k1_ac_dn8: f64,
        var_k1_ac_dn9: f64,
        var_k2_ac: f64,
        var_k2_ac_dn4: f64,
        var_k2_ac_dn6: f64,
        var_k2_ac_dn7: f64,
        var_k2_ac_dn8: f64,
        var_k2_ac_dn9: f64,
        var_keq_ac: f64,
        var_keq_ac_dn4: f64,
        var_keq_ac_dn6: f64,
        var_keq_ac_dn7: f64,
        var_keq_ac_dn8: f64,
        var_keq_ac_dn9: f64,
        var_lambda2d: f64,
        var_sce1_ac: f64,
        var_sce1_ac_dn4: f64,
        var_sce1_ac_dn6: f64,
        var_sce1_ac_dn7: f64,
        var_sce1_ac_dn8: f64,
        var_sce1_ac_dn9: f64,
        var_sce2_ac: f64,
        var_sce2_ac_dn4: f64,
        var_sce2_ac_dn6: f64,
        var_sce2_ac_dn7: f64,
        var_sce2_ac_dn8: f64,
        var_sce2_ac_dn9: f64,
        var_xd: f64,
        var_xd_dn4: f64,
        var_xd_dn6: f64,
        var_xd_dn7: f64,
        var_xd_dn8: f64,
        var_xd_dn9: f64,
        var_xdriftd_ac: f64,
        var_xdriftd_ac_dn4: f64,
        var_xdriftd_ac_dn6: f64,
        var_xdriftd_ac_dn7: f64,
        var_xdriftd_ac_dn8: f64,
        var_xdriftd_ac_dn9: f64,
        var_xeffs: f64,
        var_xeffs_dn4: f64,
        var_xeffs_dn6: f64,
        var_xeffs_dn7: f64,
        var_xeffs_dn8: f64,
        var_xeffs_dn9: f64,
        var_xsd: f64,
        var_xsd_dn4: f64,
        var_xsd_dn6: f64,
        var_xsd_dn7: f64,
        var_xsd_dn8: f64,
        var_xsd_dn9: f64,
        var_xth_1d: f64,
        var_xth_1d_dn4: f64,
        var_xth_1d_dn6: f64,
        var_xth_1d_dn7: f64,
        var_xth_1d_dn8: f64,
        var_xth_1d_dn9: f64,
        var_lambdab_slot: &mut f64,
        var_lambdab_dn4_slot: &mut f64,
        var_lambdab_dn6_slot: &mut f64,
        var_lambdab_dn7_slot: &mut f64,
        var_lambdab_dn8_slot: &mut f64,
        var_lambdab_dn9_slot: &mut f64,
        var_lambdaf_slot: &mut f64,
        var_lambdaf_dn4_slot: &mut f64,
        var_lambdaf_dn6_slot: &mut f64,
        var_lambdaf_dn7_slot: &mut f64,
        var_lambdaf_dn8_slot: &mut f64,
        var_lambdaf_dn9_slot: &mut f64,
        var_qbdif_slot: &mut f64,
        var_qbdif_dn4_slot: &mut f64,
        var_qbdif_dn6_slot: &mut f64,
        var_qbdif_dn7_slot: &mut f64,
        var_qbdif_dn8_slot: &mut f64,
        var_qbdif_dn9_slot: &mut f64,
        var_qbsif_slot: &mut f64,
        var_qbsif_dn4_slot: &mut f64,
        var_qbsif_dn6_slot: &mut f64,
        var_qbsif_dn7_slot: &mut f64,
        var_qbsif_dn8_slot: &mut f64,
        var_qbsif_dn9_slot: &mut f64,
        var_qgdif_slot: &mut f64,
        var_qgdif_dn4_slot: &mut f64,
        var_qgdif_dn6_slot: &mut f64,
        var_qgdif_dn7_slot: &mut f64,
        var_qgdif_dn8_slot: &mut f64,
        var_qgdif_dn9_slot: &mut f64,
        var_qgsif_slot: &mut f64,
        var_qgsif_dn4_slot: &mut f64,
        var_qgsif_dn6_slot: &mut f64,
        var_qgsif_dn7_slot: &mut f64,
        var_qgsif_dn8_slot: &mut f64,
        var_qgsif_dn9_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_xalphab_slot: &mut f64,
        var_xalphab_dn4_slot: &mut f64,
        var_xalphab_dn6_slot: &mut f64,
        var_xalphab_dn7_slot: &mut f64,
        var_xalphab_dn8_slot: &mut f64,
        var_xalphab_dn9_slot: &mut f64,
        var_xalphaf_slot: &mut f64,
        var_xalphaf_dn4_slot: &mut f64,
        var_xalphaf_dn6_slot: &mut f64,
        var_xalphaf_dn7_slot: &mut f64,
        var_xalphaf_dn8_slot: &mut f64,
        var_xalphaf_dn9_slot: &mut f64,
        var_xedgebd_slot: &mut f64,
        var_xedgebd_dn4_slot: &mut f64,
        var_xedgebd_dn6_slot: &mut f64,
        var_xedgebd_dn7_slot: &mut f64,
        var_xedgebd_dn8_slot: &mut f64,
        var_xedgebd_dn9_slot: &mut f64,
        var_xedgebs_slot: &mut f64,
        var_xedgebs_dn4_slot: &mut f64,
        var_xedgebs_dn6_slot: &mut f64,
        var_xedgebs_dn7_slot: &mut f64,
        var_xedgebs_dn8_slot: &mut f64,
        var_xedgebs_dn9_slot: &mut f64,
        var_xedgefd_slot: &mut f64,
        var_xedgefd_dn4_slot: &mut f64,
        var_xedgefd_dn6_slot: &mut f64,
        var_xedgefd_dn7_slot: &mut f64,
        var_xedgefd_dn8_slot: &mut f64,
        var_xedgefd_dn9_slot: &mut f64,
        var_xedgefs_slot: &mut f64,
        var_xedgefs_dn4_slot: &mut f64,
        var_xedgefs_dn6_slot: &mut f64,
        var_xedgefs_dn7_slot: &mut f64,
        var_xedgefs_dn8_slot: &mut f64,
        var_xedgefs_dn9_slot: &mut f64,
        var_xeffd_slot: &mut f64,
        var_xeffd_dn4_slot: &mut f64,
        var_xeffd_dn6_slot: &mut f64,
        var_xeffd_dn7_slot: &mut f64,
        var_xeffd_dn8_slot: &mut f64,
        var_xeffd_dn9_slot: &mut f64,
        var_xstard_slot: &mut f64,
        var_xstard_dn4_slot: &mut f64,
        var_xstard_dn6_slot: &mut f64,
        var_xstard_dn7_slot: &mut f64,
        var_xstard_dn8_slot: &mut f64,
        var_xstard_dn9_slot: &mut f64,
        var_xstars_slot: &mut f64,
        var_xstars_dn4_slot: &mut f64,
        var_xstars_dn6_slot: &mut f64,
        var_xstars_dn7_slot: &mut f64,
        var_xstars_dn8_slot: &mut f64,
        var_xstars_dn9_slot: &mut f64,
    ) {
        let mut var_lambdab: f64 = *var_lambdab_slot;
        let mut var_lambdab_dn4: f64 = *var_lambdab_dn4_slot;
        let mut var_lambdab_dn6: f64 = *var_lambdab_dn6_slot;
        let mut var_lambdab_dn7: f64 = *var_lambdab_dn7_slot;
        let mut var_lambdab_dn8: f64 = *var_lambdab_dn8_slot;
        let mut var_lambdab_dn9: f64 = *var_lambdab_dn9_slot;
        let mut var_lambdaf: f64 = *var_lambdaf_slot;
        let mut var_lambdaf_dn4: f64 = *var_lambdaf_dn4_slot;
        let mut var_lambdaf_dn6: f64 = *var_lambdaf_dn6_slot;
        let mut var_lambdaf_dn7: f64 = *var_lambdaf_dn7_slot;
        let mut var_lambdaf_dn8: f64 = *var_lambdaf_dn8_slot;
        let mut var_lambdaf_dn9: f64 = *var_lambdaf_dn9_slot;
        let mut var_qbdif: f64 = *var_qbdif_slot;
        let mut var_qbdif_dn4: f64 = *var_qbdif_dn4_slot;
        let mut var_qbdif_dn6: f64 = *var_qbdif_dn6_slot;
        let mut var_qbdif_dn7: f64 = *var_qbdif_dn7_slot;
        let mut var_qbdif_dn8: f64 = *var_qbdif_dn8_slot;
        let mut var_qbdif_dn9: f64 = *var_qbdif_dn9_slot;
        let mut var_qbsif: f64 = *var_qbsif_slot;
        let mut var_qbsif_dn4: f64 = *var_qbsif_dn4_slot;
        let mut var_qbsif_dn6: f64 = *var_qbsif_dn6_slot;
        let mut var_qbsif_dn7: f64 = *var_qbsif_dn7_slot;
        let mut var_qbsif_dn8: f64 = *var_qbsif_dn8_slot;
        let mut var_qbsif_dn9: f64 = *var_qbsif_dn9_slot;
        let mut var_qgdif: f64 = *var_qgdif_slot;
        let mut var_qgdif_dn4: f64 = *var_qgdif_dn4_slot;
        let mut var_qgdif_dn6: f64 = *var_qgdif_dn6_slot;
        let mut var_qgdif_dn7: f64 = *var_qgdif_dn7_slot;
        let mut var_qgdif_dn8: f64 = *var_qgdif_dn8_slot;
        let mut var_qgdif_dn9: f64 = *var_qgdif_dn9_slot;
        let mut var_qgsif: f64 = *var_qgsif_slot;
        let mut var_qgsif_dn4: f64 = *var_qgsif_dn4_slot;
        let mut var_qgsif_dn6: f64 = *var_qgsif_dn6_slot;
        let mut var_qgsif_dn7: f64 = *var_qgsif_dn7_slot;
        let mut var_qgsif_dn8: f64 = *var_qgsif_dn8_slot;
        let mut var_qgsif_dn9: f64 = *var_qgsif_dn9_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_xalphab: f64 = *var_xalphab_slot;
        let mut var_xalphab_dn4: f64 = *var_xalphab_dn4_slot;
        let mut var_xalphab_dn6: f64 = *var_xalphab_dn6_slot;
        let mut var_xalphab_dn7: f64 = *var_xalphab_dn7_slot;
        let mut var_xalphab_dn8: f64 = *var_xalphab_dn8_slot;
        let mut var_xalphab_dn9: f64 = *var_xalphab_dn9_slot;
        let mut var_xalphaf: f64 = *var_xalphaf_slot;
        let mut var_xalphaf_dn4: f64 = *var_xalphaf_dn4_slot;
        let mut var_xalphaf_dn6: f64 = *var_xalphaf_dn6_slot;
        let mut var_xalphaf_dn7: f64 = *var_xalphaf_dn7_slot;
        let mut var_xalphaf_dn8: f64 = *var_xalphaf_dn8_slot;
        let mut var_xalphaf_dn9: f64 = *var_xalphaf_dn9_slot;
        let mut var_xedgebd: f64 = *var_xedgebd_slot;
        let mut var_xedgebd_dn4: f64 = *var_xedgebd_dn4_slot;
        let mut var_xedgebd_dn6: f64 = *var_xedgebd_dn6_slot;
        let mut var_xedgebd_dn7: f64 = *var_xedgebd_dn7_slot;
        let mut var_xedgebd_dn8: f64 = *var_xedgebd_dn8_slot;
        let mut var_xedgebd_dn9: f64 = *var_xedgebd_dn9_slot;
        let mut var_xedgebs: f64 = *var_xedgebs_slot;
        let mut var_xedgebs_dn4: f64 = *var_xedgebs_dn4_slot;
        let mut var_xedgebs_dn6: f64 = *var_xedgebs_dn6_slot;
        let mut var_xedgebs_dn7: f64 = *var_xedgebs_dn7_slot;
        let mut var_xedgebs_dn8: f64 = *var_xedgebs_dn8_slot;
        let mut var_xedgebs_dn9: f64 = *var_xedgebs_dn9_slot;
        let mut var_xedgefd: f64 = *var_xedgefd_slot;
        let mut var_xedgefd_dn4: f64 = *var_xedgefd_dn4_slot;
        let mut var_xedgefd_dn6: f64 = *var_xedgefd_dn6_slot;
        let mut var_xedgefd_dn7: f64 = *var_xedgefd_dn7_slot;
        let mut var_xedgefd_dn8: f64 = *var_xedgefd_dn8_slot;
        let mut var_xedgefd_dn9: f64 = *var_xedgefd_dn9_slot;
        let mut var_xedgefs: f64 = *var_xedgefs_slot;
        let mut var_xedgefs_dn4: f64 = *var_xedgefs_dn4_slot;
        let mut var_xedgefs_dn6: f64 = *var_xedgefs_dn6_slot;
        let mut var_xedgefs_dn7: f64 = *var_xedgefs_dn7_slot;
        let mut var_xedgefs_dn8: f64 = *var_xedgefs_dn8_slot;
        let mut var_xedgefs_dn9: f64 = *var_xedgefs_dn9_slot;
        let mut var_xeffd: f64 = *var_xeffd_slot;
        let mut var_xeffd_dn4: f64 = *var_xeffd_dn4_slot;
        let mut var_xeffd_dn6: f64 = *var_xeffd_dn6_slot;
        let mut var_xeffd_dn7: f64 = *var_xeffd_dn7_slot;
        let mut var_xeffd_dn8: f64 = *var_xeffd_dn8_slot;
        let mut var_xeffd_dn9: f64 = *var_xeffd_dn9_slot;
        let mut var_xstard: f64 = *var_xstard_slot;
        let mut var_xstard_dn4: f64 = *var_xstard_dn4_slot;
        let mut var_xstard_dn6: f64 = *var_xstard_dn6_slot;
        let mut var_xstard_dn7: f64 = *var_xstard_dn7_slot;
        let mut var_xstard_dn8: f64 = *var_xstard_dn8_slot;
        let mut var_xstard_dn9: f64 = *var_xstard_dn9_slot;
        let mut var_xstars: f64 = *var_xstars_slot;
        let mut var_xstars_dn4: f64 = *var_xstars_dn4_slot;
        let mut var_xstars_dn6: f64 = *var_xstars_dn6_slot;
        let mut var_xstars_dn7: f64 = *var_xstars_dn7_slot;
        let mut var_xstars_dn8: f64 = *var_xstars_dn8_slot;
        let mut var_xstars_dn9: f64 = *var_xstars_dn9_slot;

        let (assign42370_e47796, assign42370_e47796_d_n4, assign42370_e47796_d_n6, assign42370_e47796_d_n7, assign42370_e47796_d_n8, assign42370_e47796_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42370_e47794: f64 = (var_xdriftd_ac + var_temp);
        (assign42370_e47794, (var_xdriftd_ac_dn4 + var_temp_dn4), (var_xdriftd_ac_dn6 + var_temp_dn6), (var_xdriftd_ac_dn7 + var_temp_dn7), (var_xdriftd_ac_dn8 + var_temp_dn8), (var_xdriftd_ac_dn9 + var_temp_dn9),)
    } else {
        (var_xeffd, var_xeffd_dn4, var_xeffd_dn6, var_xeffd_dn7, var_xeffd_dn8, var_xeffd_dn9,)
    }
};
        var_xeffd = assign42370_e47796;
        var_xeffd_dn4 = assign42370_e47796_d_n4;
        var_xeffd_dn6 = assign42370_e47796_d_n6;
        var_xeffd_dn7 = assign42370_e47796_d_n7;
        var_xeffd_dn8 = assign42370_e47796_d_n8;
        var_xeffd_dn9 = assign42370_e47796_d_n9;

        let (assign42380_e47815, assign42380_e47815_d_n4, assign42380_e47815_d_n6, assign42380_e47815_d_n7, assign42380_e47815_d_n8, assign42380_e47815_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42380_e47801: f64 = (var_xeffs + var_xth_1d);
        let assign42380_e47804: f64 = (var_xeffs - var_xth_1d);
        let assign42380_e47807: f64 = (var_xeffs - var_xth_1d);
        let assign42380_e47808: f64 = (assign42380_e47804 * assign42380_e47807);
        let assign42380_e47810: f64 = (assign42380_e47808 + 9.0);
        let assign42380_e47811: f64 = (assign42380_e47810).sqrt();
        let assign42380_e47812: f64 = (assign42380_e47801 - assign42380_e47811);
        let assign42380_e47813: f64 = (0.5 * assign42380_e47812);
        (assign42380_e47813, (0.5 * ((var_xeffs_dn4 + var_xth_1d_dn4) - ((((var_xeffs_dn4 - var_xth_1d_dn4) * assign42380_e47807) + (assign42380_e47804 * (var_xeffs_dn4 - var_xth_1d_dn4))) / (2.0 * assign42380_e47811)))), (0.5 * ((var_xeffs_dn6 + var_xth_1d_dn6) - ((((var_xeffs_dn6 - var_xth_1d_dn6) * assign42380_e47807) + (assign42380_e47804 * (var_xeffs_dn6 - var_xth_1d_dn6))) / (2.0 * assign42380_e47811)))), (0.5 * ((var_xeffs_dn7 + var_xth_1d_dn7) - ((((var_xeffs_dn7 - var_xth_1d_dn7) * assign42380_e47807) + (assign42380_e47804 * (var_xeffs_dn7 - var_xth_1d_dn7))) / (2.0 * assign42380_e47811)))), (0.5 * ((var_xeffs_dn8 + var_xth_1d_dn8) - ((((var_xeffs_dn8 - var_xth_1d_dn8) * assign42380_e47807) + (assign42380_e47804 * (var_xeffs_dn8 - var_xth_1d_dn8))) / (2.0 * assign42380_e47811)))), (0.5 * ((var_xeffs_dn9 + var_xth_1d_dn9) - ((((var_xeffs_dn9 - var_xth_1d_dn9) * assign42380_e47807) + (assign42380_e47804 * (var_xeffs_dn9 - var_xth_1d_dn9))) / (2.0 * assign42380_e47811)))),)
    } else {
        (var_xstars, var_xstars_dn4, var_xstars_dn6, var_xstars_dn7, var_xstars_dn8, var_xstars_dn9,)
    }
};
        var_xstars = assign42380_e47815;
        var_xstars_dn4 = assign42380_e47815_d_n4;
        var_xstars_dn6 = assign42380_e47815_d_n6;
        var_xstars_dn7 = assign42380_e47815_d_n7;
        var_xstars_dn8 = assign42380_e47815_d_n8;
        var_xstars_dn9 = assign42380_e47815_d_n9;

        let (assign42390_e47840, assign42390_e47840_d_n4, assign42390_e47840_d_n6, assign42390_e47840_d_n7, assign42390_e47840_d_n8, assign42390_e47840_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42390_e47821: f64 = (var_xth_1d + var_xd);
        let assign42390_e47822: f64 = (var_xeffd + assign42390_e47821);
        let assign42390_e47826: f64 = (var_xth_1d + var_xd);
        let assign42390_e47827: f64 = (var_xeffd - assign42390_e47826);
        let assign42390_e47831: f64 = (var_xth_1d + var_xd);
        let assign42390_e47832: f64 = (var_xeffd - assign42390_e47831);
        let assign42390_e47833: f64 = (assign42390_e47827 * assign42390_e47832);
        let assign42390_e47835: f64 = (assign42390_e47833 + 9.0);
        let assign42390_e47836: f64 = (assign42390_e47835).sqrt();
        let assign42390_e47837: f64 = (assign42390_e47822 - assign42390_e47836);
        let assign42390_e47838: f64 = (0.5 * assign42390_e47837);
        (assign42390_e47838, (0.5 * ((var_xeffd_dn4 + (var_xth_1d_dn4 + var_xd_dn4)) - ((((var_xeffd_dn4 - (var_xth_1d_dn4 + var_xd_dn4)) * assign42390_e47832) + (assign42390_e47827 * (var_xeffd_dn4 - (var_xth_1d_dn4 + var_xd_dn4)))) / (2.0 * assign42390_e47836)))), (0.5 * ((var_xeffd_dn6 + (var_xth_1d_dn6 + var_xd_dn6)) - ((((var_xeffd_dn6 - (var_xth_1d_dn6 + var_xd_dn6)) * assign42390_e47832) + (assign42390_e47827 * (var_xeffd_dn6 - (var_xth_1d_dn6 + var_xd_dn6)))) / (2.0 * assign42390_e47836)))), (0.5 * ((var_xeffd_dn7 + (var_xth_1d_dn7 + var_xd_dn7)) - ((((var_xeffd_dn7 - (var_xth_1d_dn7 + var_xd_dn7)) * assign42390_e47832) + (assign42390_e47827 * (var_xeffd_dn7 - (var_xth_1d_dn7 + var_xd_dn7)))) / (2.0 * assign42390_e47836)))), (0.5 * ((var_xeffd_dn8 + (var_xth_1d_dn8 + var_xd_dn8)) - ((((var_xeffd_dn8 - (var_xth_1d_dn8 + var_xd_dn8)) * assign42390_e47832) + (assign42390_e47827 * (var_xeffd_dn8 - (var_xth_1d_dn8 + var_xd_dn8)))) / (2.0 * assign42390_e47836)))), (0.5 * ((var_xeffd_dn9 + (var_xth_1d_dn9 + var_xd_dn9)) - ((((var_xeffd_dn9 - (var_xth_1d_dn9 + var_xd_dn9)) * assign42390_e47832) + (assign42390_e47827 * (var_xeffd_dn9 - (var_xth_1d_dn9 + var_xd_dn9)))) / (2.0 * assign42390_e47836)))),)
    } else {
        (var_xstard, var_xstard_dn4, var_xstard_dn6, var_xstard_dn7, var_xstard_dn8, var_xstard_dn9,)
    }
};
        var_xstard = assign42390_e47840;
        var_xstard_dn4 = assign42390_e47840_d_n4;
        var_xstard_dn6 = assign42390_e47840_d_n6;
        var_xstard_dn7 = assign42390_e47840_d_n7;
        var_xstard_dn8 = assign42390_e47840_d_n8;
        var_xstard_dn9 = assign42390_e47840_d_n9;

        let (assign42400_e47851, assign42400_e47851_d_n4, assign42400_e47851_d_n6, assign42400_e47851_d_n7, assign42400_e47851_d_n8, assign42400_e47851_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42400_e47846: f64 = (0.5 + var_inv_k2_ac);
        let assign42400_e47847: f64 = (var_keq_ac * assign42400_e47846);
        let assign42400_e47848: f64 = (assign42400_e47847).sqrt();
        let assign42400_e47849: f64 = (var_lambda2d * assign42400_e47848);
        (assign42400_e47849, (var_lambda2d * (((var_keq_ac_dn4 * assign42400_e47846) + (var_keq_ac * var_inv_k2_ac_dn4)) / (2.0 * assign42400_e47848))), (var_lambda2d * (((var_keq_ac_dn6 * assign42400_e47846) + (var_keq_ac * var_inv_k2_ac_dn6)) / (2.0 * assign42400_e47848))), (var_lambda2d * (((var_keq_ac_dn7 * assign42400_e47846) + (var_keq_ac * var_inv_k2_ac_dn7)) / (2.0 * assign42400_e47848))), (var_lambda2d * (((var_keq_ac_dn8 * assign42400_e47846) + (var_keq_ac * var_inv_k2_ac_dn8)) / (2.0 * assign42400_e47848))), (var_lambda2d * (((var_keq_ac_dn9 * assign42400_e47846) + (var_keq_ac * var_inv_k2_ac_dn9)) / (2.0 * assign42400_e47848))),)
    } else {
        (var_lambdaf, var_lambdaf_dn4, var_lambdaf_dn6, var_lambdaf_dn7, var_lambdaf_dn8, var_lambdaf_dn9,)
    }
};
        var_lambdaf = assign42400_e47851;
        var_lambdaf_dn4 = assign42400_e47851_d_n4;
        var_lambdaf_dn6 = assign42400_e47851_d_n6;
        var_lambdaf_dn7 = assign42400_e47851_d_n7;
        var_lambdaf_dn8 = assign42400_e47851_d_n8;
        var_lambdaf_dn9 = assign42400_e47851_d_n9;

        let (assign42410_e47866, assign42410_e47866_d_n4, assign42410_e47866_d_n6, assign42410_e47866_d_n7, assign42410_e47866_d_n8, assign42410_e47866_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42410_e47856: f64 = (var_keq_ac * var_k1_ac);
        let assign42410_e47858: f64 = (assign42410_e47856 * var_inv_k2_ac);
        let assign42410_e47861: f64 = (0.5 + var_inv_k1_ac);
        let assign42410_e47862: f64 = (assign42410_e47858 * assign42410_e47861);
        let assign42410_e47863: f64 = (assign42410_e47862).sqrt();
        let assign42410_e47864: f64 = (var_lambda2d * assign42410_e47863);
        (assign42410_e47864, (var_lambda2d * (((((((var_keq_ac_dn4 * var_k1_ac) + (var_keq_ac * var_k1_ac_dn4)) * var_inv_k2_ac) + (assign42410_e47856 * var_inv_k2_ac_dn4)) * assign42410_e47861) + (assign42410_e47858 * var_inv_k1_ac_dn4)) / (2.0 * assign42410_e47863))), (var_lambda2d * (((((((var_keq_ac_dn6 * var_k1_ac) + (var_keq_ac * var_k1_ac_dn6)) * var_inv_k2_ac) + (assign42410_e47856 * var_inv_k2_ac_dn6)) * assign42410_e47861) + (assign42410_e47858 * var_inv_k1_ac_dn6)) / (2.0 * assign42410_e47863))), (var_lambda2d * (((((((var_keq_ac_dn7 * var_k1_ac) + (var_keq_ac * var_k1_ac_dn7)) * var_inv_k2_ac) + (assign42410_e47856 * var_inv_k2_ac_dn7)) * assign42410_e47861) + (assign42410_e47858 * var_inv_k1_ac_dn7)) / (2.0 * assign42410_e47863))), (var_lambda2d * (((((((var_keq_ac_dn8 * var_k1_ac) + (var_keq_ac * var_k1_ac_dn8)) * var_inv_k2_ac) + (assign42410_e47856 * var_inv_k2_ac_dn8)) * assign42410_e47861) + (assign42410_e47858 * var_inv_k1_ac_dn8)) / (2.0 * assign42410_e47863))), (var_lambda2d * (((((((var_keq_ac_dn9 * var_k1_ac) + (var_keq_ac * var_k1_ac_dn9)) * var_inv_k2_ac) + (assign42410_e47856 * var_inv_k2_ac_dn9)) * assign42410_e47861) + (assign42410_e47858 * var_inv_k1_ac_dn9)) / (2.0 * assign42410_e47863))),)
    } else {
        (var_lambdab, var_lambdab_dn4, var_lambdab_dn6, var_lambdab_dn7, var_lambdab_dn8, var_lambdab_dn9,)
    }
};
        var_lambdab = assign42410_e47866;
        var_lambdab_dn4 = assign42410_e47866_d_n4;
        var_lambdab_dn6 = assign42410_e47866_d_n6;
        var_lambdab_dn7 = assign42410_e47866_d_n7;
        var_lambdab_dn8 = assign42410_e47866_d_n8;
        var_lambdab_dn9 = assign42410_e47866_d_n9;

        let (assign42420_e47874, assign42420_e47874_d_n4, assign42420_e47874_d_n6, assign42420_e47874_d_n7, assign42420_e47874_d_n8, assign42420_e47874_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42420_e47870: f64 = (var_lambdaf * var_lambdaf);
        let assign42420_e47872: f64 = (assign42420_e47870 * var_inner_sd);
        (assign42420_e47872, ((((var_lambdaf_dn4 * var_lambdaf) + (var_lambdaf * var_lambdaf_dn4)) * var_inner_sd) + (assign42420_e47870 * var_inner_sd_dn4)), ((((var_lambdaf_dn6 * var_lambdaf) + (var_lambdaf * var_lambdaf_dn6)) * var_inner_sd) + (assign42420_e47870 * var_inner_sd_dn6)), ((((var_lambdaf_dn7 * var_lambdaf) + (var_lambdaf * var_lambdaf_dn7)) * var_inner_sd) + (assign42420_e47870 * var_inner_sd_dn7)), ((((var_lambdaf_dn8 * var_lambdaf) + (var_lambdaf * var_lambdaf_dn8)) * var_inner_sd) + (assign42420_e47870 * var_inner_sd_dn8)), ((((var_lambdaf_dn9 * var_lambdaf) + (var_lambdaf * var_lambdaf_dn9)) * var_inner_sd) + (assign42420_e47870 * var_inner_sd_dn9)),)
    } else {
        (var_xalphaf, var_xalphaf_dn4, var_xalphaf_dn6, var_xalphaf_dn7, var_xalphaf_dn8, var_xalphaf_dn9,)
    }
};
        var_xalphaf = assign42420_e47874;
        var_xalphaf_dn4 = assign42420_e47874_d_n4;
        var_xalphaf_dn6 = assign42420_e47874_d_n6;
        var_xalphaf_dn7 = assign42420_e47874_d_n7;
        var_xalphaf_dn8 = assign42420_e47874_d_n8;
        var_xalphaf_dn9 = assign42420_e47874_d_n9;

        let (assign42430_e47882, assign42430_e47882_d_n4, assign42430_e47882_d_n6, assign42430_e47882_d_n7, assign42430_e47882_d_n8, assign42430_e47882_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42430_e47878: f64 = (var_lambdab * var_lambdab);
        let assign42430_e47880: f64 = (assign42430_e47878 * var_inner_sd);
        (assign42430_e47880, ((((var_lambdab_dn4 * var_lambdab) + (var_lambdab * var_lambdab_dn4)) * var_inner_sd) + (assign42430_e47878 * var_inner_sd_dn4)), ((((var_lambdab_dn6 * var_lambdab) + (var_lambdab * var_lambdab_dn6)) * var_inner_sd) + (assign42430_e47878 * var_inner_sd_dn6)), ((((var_lambdab_dn7 * var_lambdab) + (var_lambdab * var_lambdab_dn7)) * var_inner_sd) + (assign42430_e47878 * var_inner_sd_dn7)), ((((var_lambdab_dn8 * var_lambdab) + (var_lambdab * var_lambdab_dn8)) * var_inner_sd) + (assign42430_e47878 * var_inner_sd_dn8)), ((((var_lambdab_dn9 * var_lambdab) + (var_lambdab * var_lambdab_dn9)) * var_inner_sd) + (assign42430_e47878 * var_inner_sd_dn9)),)
    } else {
        (var_xalphab, var_xalphab_dn4, var_xalphab_dn6, var_xalphab_dn7, var_xalphab_dn8, var_xalphab_dn9,)
    }
};
        var_xalphab = assign42430_e47882;
        var_xalphab_dn4 = assign42430_e47882_d_n4;
        var_xalphab_dn6 = assign42430_e47882_d_n6;
        var_xalphab_dn7 = assign42430_e47882_d_n7;
        var_xalphab_dn8 = assign42430_e47882_d_n8;
        var_xalphab_dn9 = assign42430_e47882_d_n9;

        let (assign42440_e47888, assign42440_e47888_d_n4, assign42440_e47888_d_n6, assign42440_e47888_d_n7, assign42440_e47888_d_n8, assign42440_e47888_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42440_e47886: f64 = (var_xsd - var_xstars);
        (assign42440_e47886, (var_xsd_dn4 - var_xstars_dn4), (var_xsd_dn6 - var_xstars_dn6), (var_xsd_dn7 - var_xstars_dn7), (var_xsd_dn8 - var_xstars_dn8), (var_xsd_dn9 - var_xstars_dn9),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign42440_e47888;
        var_temp1_dn4 = assign42440_e47888_d_n4;
        var_temp1_dn6 = assign42440_e47888_d_n6;
        var_temp1_dn7 = assign42440_e47888_d_n7;
        var_temp1_dn8 = assign42440_e47888_d_n8;
        var_temp1_dn9 = assign42440_e47888_d_n9;

        let (assign42450_e47896, assign42450_e47896_d_n4, assign42450_e47896_d_n6, assign42450_e47896_d_n7, assign42450_e47896_d_n8, assign42450_e47896_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42450_e47892: f64 = (var_xsd + var_xd);
        let assign42450_e47894: f64 = (assign42450_e47892 - var_xstard);
        (assign42450_e47894, ((var_xsd_dn4 + var_xd_dn4) - var_xstard_dn4), ((var_xsd_dn6 + var_xd_dn6) - var_xstard_dn6), ((var_xsd_dn7 + var_xd_dn7) - var_xstard_dn7), ((var_xsd_dn8 + var_xd_dn8) - var_xstard_dn8), ((var_xsd_dn9 + var_xd_dn9) - var_xstard_dn9),)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign42450_e47896;
        var_temp2_dn4 = assign42450_e47896_d_n4;
        var_temp2_dn6 = assign42450_e47896_d_n6;
        var_temp2_dn7 = assign42450_e47896_d_n7;
        var_temp2_dn8 = assign42450_e47896_d_n8;
        var_temp2_dn9 = assign42450_e47896_d_n9;

        let (assign42460_e47902, assign42460_e47902_d_n4, assign42460_e47902_d_n6, assign42460_e47902_d_n7, assign42460_e47902_d_n8, assign42460_e47902_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42460_e47900: f64 = (2.0 * var_xalphaf);
        (assign42460_e47900, (2.0 * var_xalphaf_dn4), (2.0 * var_xalphaf_dn6), (2.0 * var_xalphaf_dn7), (2.0 * var_xalphaf_dn8), (2.0 * var_xalphaf_dn9),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign42460_e47902;
        var_temp_dn4 = assign42460_e47902_d_n4;
        var_temp_dn6 = assign42460_e47902_d_n6;
        var_temp_dn7 = assign42460_e47902_d_n7;
        var_temp_dn8 = assign42460_e47902_d_n8;
        var_temp_dn9 = assign42460_e47902_d_n9;

        let (assign42470_e47917, assign42470_e47917_d_n4, assign42470_e47917_d_n6, assign42470_e47917_d_n7, assign42470_e47917_d_n8, assign42470_e47917_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42470_e47909: f64 = (var_temp1 / var_xalphaf);
        let assign42470_e47910: f64 = (1.0 + assign42470_e47909);
        let assign42470_e47911: f64 = (assign42470_e47910).sqrt();
        let assign42470_e47913: f64 = (assign42470_e47911 - 1.0);
        let assign42470_e47914: f64 = (var_temp * assign42470_e47913);
        let assign42470_e47915: f64 = (var_xstars + assign42470_e47914);
        (assign42470_e47915, (var_xstars_dn4 + ((var_temp_dn4 * assign42470_e47913) + (var_temp * ((((var_temp1_dn4 * var_xalphaf) - (var_temp1 * var_xalphaf_dn4)) / (var_xalphaf * var_xalphaf)) / (2.0 * assign42470_e47911))))), (var_xstars_dn6 + ((var_temp_dn6 * assign42470_e47913) + (var_temp * ((((var_temp1_dn6 * var_xalphaf) - (var_temp1 * var_xalphaf_dn6)) / (var_xalphaf * var_xalphaf)) / (2.0 * assign42470_e47911))))), (var_xstars_dn7 + ((var_temp_dn7 * assign42470_e47913) + (var_temp * ((((var_temp1_dn7 * var_xalphaf) - (var_temp1 * var_xalphaf_dn7)) / (var_xalphaf * var_xalphaf)) / (2.0 * assign42470_e47911))))), (var_xstars_dn8 + ((var_temp_dn8 * assign42470_e47913) + (var_temp * ((((var_temp1_dn8 * var_xalphaf) - (var_temp1 * var_xalphaf_dn8)) / (var_xalphaf * var_xalphaf)) / (2.0 * assign42470_e47911))))), (var_xstars_dn9 + ((var_temp_dn9 * assign42470_e47913) + (var_temp * ((((var_temp1_dn9 * var_xalphaf) - (var_temp1 * var_xalphaf_dn9)) / (var_xalphaf * var_xalphaf)) / (2.0 * assign42470_e47911))))),)
    } else {
        (var_xedgefs, var_xedgefs_dn4, var_xedgefs_dn6, var_xedgefs_dn7, var_xedgefs_dn8, var_xedgefs_dn9,)
    }
};
        var_xedgefs = assign42470_e47917;
        var_xedgefs_dn4 = assign42470_e47917_d_n4;
        var_xedgefs_dn6 = assign42470_e47917_d_n6;
        var_xedgefs_dn7 = assign42470_e47917_d_n7;
        var_xedgefs_dn8 = assign42470_e47917_d_n8;
        var_xedgefs_dn9 = assign42470_e47917_d_n9;

        let (assign42480_e47932, assign42480_e47932_d_n4, assign42480_e47932_d_n6, assign42480_e47932_d_n7, assign42480_e47932_d_n8, assign42480_e47932_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42480_e47924: f64 = (var_temp2 / var_xalphaf);
        let assign42480_e47925: f64 = (1.0 + assign42480_e47924);
        let assign42480_e47926: f64 = (assign42480_e47925).sqrt();
        let assign42480_e47928: f64 = (assign42480_e47926 - 1.0);
        let assign42480_e47929: f64 = (var_temp * assign42480_e47928);
        let assign42480_e47930: f64 = (var_xstard + assign42480_e47929);
        (assign42480_e47930, (var_xstard_dn4 + ((var_temp_dn4 * assign42480_e47928) + (var_temp * ((((var_temp2_dn4 * var_xalphaf) - (var_temp2 * var_xalphaf_dn4)) / (var_xalphaf * var_xalphaf)) / (2.0 * assign42480_e47926))))), (var_xstard_dn6 + ((var_temp_dn6 * assign42480_e47928) + (var_temp * ((((var_temp2_dn6 * var_xalphaf) - (var_temp2 * var_xalphaf_dn6)) / (var_xalphaf * var_xalphaf)) / (2.0 * assign42480_e47926))))), (var_xstard_dn7 + ((var_temp_dn7 * assign42480_e47928) + (var_temp * ((((var_temp2_dn7 * var_xalphaf) - (var_temp2 * var_xalphaf_dn7)) / (var_xalphaf * var_xalphaf)) / (2.0 * assign42480_e47926))))), (var_xstard_dn8 + ((var_temp_dn8 * assign42480_e47928) + (var_temp * ((((var_temp2_dn8 * var_xalphaf) - (var_temp2 * var_xalphaf_dn8)) / (var_xalphaf * var_xalphaf)) / (2.0 * assign42480_e47926))))), (var_xstard_dn9 + ((var_temp_dn9 * assign42480_e47928) + (var_temp * ((((var_temp2_dn9 * var_xalphaf) - (var_temp2 * var_xalphaf_dn9)) / (var_xalphaf * var_xalphaf)) / (2.0 * assign42480_e47926))))),)
    } else {
        (var_xedgefd, var_xedgefd_dn4, var_xedgefd_dn6, var_xedgefd_dn7, var_xedgefd_dn8, var_xedgefd_dn9,)
    }
};
        var_xedgefd = assign42480_e47932;
        var_xedgefd_dn4 = assign42480_e47932_d_n4;
        var_xedgefd_dn6 = assign42480_e47932_d_n6;
        var_xedgefd_dn7 = assign42480_e47932_d_n7;
        var_xedgefd_dn8 = assign42480_e47932_d_n8;
        var_xedgefd_dn9 = assign42480_e47932_d_n9;

        let (assign42490_e47938, assign42490_e47938_d_n4, assign42490_e47938_d_n6, assign42490_e47938_d_n7, assign42490_e47938_d_n8, assign42490_e47938_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42490_e47936: f64 = (2.0 * var_xalphab);
        (assign42490_e47936, (2.0 * var_xalphab_dn4), (2.0 * var_xalphab_dn6), (2.0 * var_xalphab_dn7), (2.0 * var_xalphab_dn8), (2.0 * var_xalphab_dn9),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign42490_e47938;
        var_temp_dn4 = assign42490_e47938_d_n4;
        var_temp_dn6 = assign42490_e47938_d_n6;
        var_temp_dn7 = assign42490_e47938_d_n7;
        var_temp_dn8 = assign42490_e47938_d_n8;
        var_temp_dn9 = assign42490_e47938_d_n9;

        let (assign42500_e47953, assign42500_e47953_d_n4, assign42500_e47953_d_n6, assign42500_e47953_d_n7, assign42500_e47953_d_n8, assign42500_e47953_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42500_e47945: f64 = (var_temp1 / var_xalphab);
        let assign42500_e47946: f64 = (1.0 + assign42500_e47945);
        let assign42500_e47947: f64 = (assign42500_e47946).sqrt();
        let assign42500_e47949: f64 = (assign42500_e47947 - 1.0);
        let assign42500_e47950: f64 = (var_temp * assign42500_e47949);
        let assign42500_e47951: f64 = (var_xstars + assign42500_e47950);
        (assign42500_e47951, (var_xstars_dn4 + ((var_temp_dn4 * assign42500_e47949) + (var_temp * ((((var_temp1_dn4 * var_xalphab) - (var_temp1 * var_xalphab_dn4)) / (var_xalphab * var_xalphab)) / (2.0 * assign42500_e47947))))), (var_xstars_dn6 + ((var_temp_dn6 * assign42500_e47949) + (var_temp * ((((var_temp1_dn6 * var_xalphab) - (var_temp1 * var_xalphab_dn6)) / (var_xalphab * var_xalphab)) / (2.0 * assign42500_e47947))))), (var_xstars_dn7 + ((var_temp_dn7 * assign42500_e47949) + (var_temp * ((((var_temp1_dn7 * var_xalphab) - (var_temp1 * var_xalphab_dn7)) / (var_xalphab * var_xalphab)) / (2.0 * assign42500_e47947))))), (var_xstars_dn8 + ((var_temp_dn8 * assign42500_e47949) + (var_temp * ((((var_temp1_dn8 * var_xalphab) - (var_temp1 * var_xalphab_dn8)) / (var_xalphab * var_xalphab)) / (2.0 * assign42500_e47947))))), (var_xstars_dn9 + ((var_temp_dn9 * assign42500_e47949) + (var_temp * ((((var_temp1_dn9 * var_xalphab) - (var_temp1 * var_xalphab_dn9)) / (var_xalphab * var_xalphab)) / (2.0 * assign42500_e47947))))),)
    } else {
        (var_xedgebs, var_xedgebs_dn4, var_xedgebs_dn6, var_xedgebs_dn7, var_xedgebs_dn8, var_xedgebs_dn9,)
    }
};
        var_xedgebs = assign42500_e47953;
        var_xedgebs_dn4 = assign42500_e47953_d_n4;
        var_xedgebs_dn6 = assign42500_e47953_d_n6;
        var_xedgebs_dn7 = assign42500_e47953_d_n7;
        var_xedgebs_dn8 = assign42500_e47953_d_n8;
        var_xedgebs_dn9 = assign42500_e47953_d_n9;

        let (assign42510_e47968, assign42510_e47968_d_n4, assign42510_e47968_d_n6, assign42510_e47968_d_n7, assign42510_e47968_d_n8, assign42510_e47968_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42510_e47960: f64 = (var_temp2 / var_xalphab);
        let assign42510_e47961: f64 = (1.0 + assign42510_e47960);
        let assign42510_e47962: f64 = (assign42510_e47961).sqrt();
        let assign42510_e47964: f64 = (assign42510_e47962 - 1.0);
        let assign42510_e47965: f64 = (var_temp * assign42510_e47964);
        let assign42510_e47966: f64 = (var_xstard + assign42510_e47965);
        (assign42510_e47966, (var_xstard_dn4 + ((var_temp_dn4 * assign42510_e47964) + (var_temp * ((((var_temp2_dn4 * var_xalphab) - (var_temp2 * var_xalphab_dn4)) / (var_xalphab * var_xalphab)) / (2.0 * assign42510_e47962))))), (var_xstard_dn6 + ((var_temp_dn6 * assign42510_e47964) + (var_temp * ((((var_temp2_dn6 * var_xalphab) - (var_temp2 * var_xalphab_dn6)) / (var_xalphab * var_xalphab)) / (2.0 * assign42510_e47962))))), (var_xstard_dn7 + ((var_temp_dn7 * assign42510_e47964) + (var_temp * ((((var_temp2_dn7 * var_xalphab) - (var_temp2 * var_xalphab_dn7)) / (var_xalphab * var_xalphab)) / (2.0 * assign42510_e47962))))), (var_xstard_dn8 + ((var_temp_dn8 * assign42510_e47964) + (var_temp * ((((var_temp2_dn8 * var_xalphab) - (var_temp2 * var_xalphab_dn8)) / (var_xalphab * var_xalphab)) / (2.0 * assign42510_e47962))))), (var_xstard_dn9 + ((var_temp_dn9 * assign42510_e47964) + (var_temp * ((((var_temp2_dn9 * var_xalphab) - (var_temp2 * var_xalphab_dn9)) / (var_xalphab * var_xalphab)) / (2.0 * assign42510_e47962))))),)
    } else {
        (var_xedgebd, var_xedgebd_dn4, var_xedgebd_dn6, var_xedgebd_dn7, var_xedgebd_dn8, var_xedgebd_dn9,)
    }
};
        var_xedgebd = assign42510_e47968;
        var_xedgebd_dn4 = assign42510_e47968_d_n4;
        var_xedgebd_dn6 = assign42510_e47968_d_n6;
        var_xedgebd_dn7 = assign42510_e47968_d_n7;
        var_xedgebd_dn8 = assign42510_e47968_d_n8;
        var_xedgebd_dn9 = assign42510_e47968_d_n9;

        let (assign42520_e47974, assign42520_e47974_d_n4, assign42520_e47974_d_n6, assign42520_e47974_d_n7, assign42520_e47974_d_n8, assign42520_e47974_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42520_e47972: f64 = (var_fif_phit * var_csiprime_ac);
        (assign42520_e47972, ((var_fif_phit_dn4 * var_csiprime_ac) + (var_fif_phit * var_csiprime_ac_dn4)), ((var_fif_phit_dn6 * var_csiprime_ac) + (var_fif_phit * var_csiprime_ac_dn6)), ((var_fif_phit_dn7 * var_csiprime_ac) + (var_fif_phit * var_csiprime_ac_dn7)), ((var_fif_phit_dn8 * var_csiprime_ac) + (var_fif_phit * var_csiprime_ac_dn8)), ((var_fif_phit_dn9 * var_csiprime_ac) + (var_fif_phit * var_csiprime_ac_dn9)),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign42520_e47974;
        var_temp_dn4 = assign42520_e47974_d_n4;
        var_temp_dn6 = assign42520_e47974_d_n6;
        var_temp_dn7 = assign42520_e47974_d_n7;
        var_temp_dn8 = assign42520_e47974_d_n8;
        var_temp_dn9 = assign42520_e47974_d_n9;

        let (assign42530_e47985, assign42530_e47985_d_n4, assign42530_e47985_d_n6, assign42530_e47985_d_n7, assign42530_e47985_d_n8, assign42530_e47985_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42530_e47977: f64 = (-var_temp);
        let assign42530_e47979: f64 = (assign42530_e47977 * var_lambdaf);
        let assign42530_e47981: f64 = (assign42530_e47979 * var_k1_ac);
        let assign42530_e47983: f64 = (assign42530_e47981 * var_sce1_ac);
        (assign42530_e47983, (((((((-var_temp_dn4) * var_lambdaf) + (assign42530_e47977 * var_lambdaf_dn4)) * var_k1_ac) + (assign42530_e47979 * var_k1_ac_dn4)) * var_sce1_ac) + (assign42530_e47981 * var_sce1_ac_dn4)), (((((((-var_temp_dn6) * var_lambdaf) + (assign42530_e47977 * var_lambdaf_dn6)) * var_k1_ac) + (assign42530_e47979 * var_k1_ac_dn6)) * var_sce1_ac) + (assign42530_e47981 * var_sce1_ac_dn6)), (((((((-var_temp_dn7) * var_lambdaf) + (assign42530_e47977 * var_lambdaf_dn7)) * var_k1_ac) + (assign42530_e47979 * var_k1_ac_dn7)) * var_sce1_ac) + (assign42530_e47981 * var_sce1_ac_dn7)), (((((((-var_temp_dn8) * var_lambdaf) + (assign42530_e47977 * var_lambdaf_dn8)) * var_k1_ac) + (assign42530_e47979 * var_k1_ac_dn8)) * var_sce1_ac) + (assign42530_e47981 * var_sce1_ac_dn8)), (((((((-var_temp_dn9) * var_lambdaf) + (assign42530_e47977 * var_lambdaf_dn9)) * var_k1_ac) + (assign42530_e47979 * var_k1_ac_dn9)) * var_sce1_ac) + (assign42530_e47981 * var_sce1_ac_dn9)),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign42530_e47985;
        var_temp1_dn4 = assign42530_e47985_d_n4;
        var_temp1_dn6 = assign42530_e47985_d_n6;
        var_temp1_dn7 = assign42530_e47985_d_n7;
        var_temp1_dn8 = assign42530_e47985_d_n8;
        var_temp1_dn9 = assign42530_e47985_d_n9;

        let (assign42540_e47996, assign42540_e47996_d_n4, assign42540_e47996_d_n6, assign42540_e47996_d_n7, assign42540_e47996_d_n8, assign42540_e47996_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42540_e47988: f64 = (-var_temp);
        let assign42540_e47990: f64 = (assign42540_e47988 * var_lambdab);
        let assign42540_e47992: f64 = (assign42540_e47990 * var_k2_ac);
        let assign42540_e47994: f64 = (assign42540_e47992 * var_sce2_ac);
        (assign42540_e47994, (((((((-var_temp_dn4) * var_lambdab) + (assign42540_e47988 * var_lambdab_dn4)) * var_k2_ac) + (assign42540_e47990 * var_k2_ac_dn4)) * var_sce2_ac) + (assign42540_e47992 * var_sce2_ac_dn4)), (((((((-var_temp_dn6) * var_lambdab) + (assign42540_e47988 * var_lambdab_dn6)) * var_k2_ac) + (assign42540_e47990 * var_k2_ac_dn6)) * var_sce2_ac) + (assign42540_e47992 * var_sce2_ac_dn6)), (((((((-var_temp_dn7) * var_lambdab) + (assign42540_e47988 * var_lambdab_dn7)) * var_k2_ac) + (assign42540_e47990 * var_k2_ac_dn7)) * var_sce2_ac) + (assign42540_e47992 * var_sce2_ac_dn7)), (((((((-var_temp_dn8) * var_lambdab) + (assign42540_e47988 * var_lambdab_dn8)) * var_k2_ac) + (assign42540_e47990 * var_k2_ac_dn8)) * var_sce2_ac) + (assign42540_e47992 * var_sce2_ac_dn8)), (((((((-var_temp_dn9) * var_lambdab) + (assign42540_e47988 * var_lambdab_dn9)) * var_k2_ac) + (assign42540_e47990 * var_k2_ac_dn9)) * var_sce2_ac) + (assign42540_e47992 * var_sce2_ac_dn9)),)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign42540_e47996;
        var_temp2_dn4 = assign42540_e47996_d_n4;
        var_temp2_dn6 = assign42540_e47996_d_n6;
        var_temp2_dn7 = assign42540_e47996_d_n7;
        var_temp2_dn8 = assign42540_e47996_d_n8;
        var_temp2_dn9 = assign42540_e47996_d_n9;

        let (assign42550_e48021, assign42550_e48021_d_n4, assign42550_e48021_d_n6, assign42550_e48021_d_n7, assign42550_e48021_d_n8, assign42550_e48021_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42550_e48001: f64 = (var_xedgefs - var_xeffs);
        let assign42550_e48003: f64 = assign42550_e48001;
        let assign42550_e48006: f64 = (var_xedgefs - var_xeffs);
        let assign42550_e48008: f64 = assign42550_e48006;
        let assign42550_e48011: f64 = (var_xedgefs - var_xeffs);
        let assign42550_e48013: f64 = assign42550_e48011;
        let assign42550_e48014: f64 = (assign42550_e48008 * assign42550_e48013);
        let assign42550_e48016: f64 = (assign42550_e48014 + 1.0);
        let assign42550_e48017: f64 = (assign42550_e48016).sqrt();
        let assign42550_e48018: f64 = (assign42550_e48003 + assign42550_e48017);
        let assign42550_e48019: f64 = (0.5 * assign42550_e48018);
        (assign42550_e48019, (0.5 * ((var_xedgefs_dn4 - var_xeffs_dn4) + ((((var_xedgefs_dn4 - var_xeffs_dn4) * assign42550_e48013) + (assign42550_e48008 * (var_xedgefs_dn4 - var_xeffs_dn4))) / (2.0 * assign42550_e48017)))), (0.5 * ((var_xedgefs_dn6 - var_xeffs_dn6) + ((((var_xedgefs_dn6 - var_xeffs_dn6) * assign42550_e48013) + (assign42550_e48008 * (var_xedgefs_dn6 - var_xeffs_dn6))) / (2.0 * assign42550_e48017)))), (0.5 * ((var_xedgefs_dn7 - var_xeffs_dn7) + ((((var_xedgefs_dn7 - var_xeffs_dn7) * assign42550_e48013) + (assign42550_e48008 * (var_xedgefs_dn7 - var_xeffs_dn7))) / (2.0 * assign42550_e48017)))), (0.5 * ((var_xedgefs_dn8 - var_xeffs_dn8) + ((((var_xedgefs_dn8 - var_xeffs_dn8) * assign42550_e48013) + (assign42550_e48008 * (var_xedgefs_dn8 - var_xeffs_dn8))) / (2.0 * assign42550_e48017)))), (0.5 * ((var_xedgefs_dn9 - var_xeffs_dn9) + ((((var_xedgefs_dn9 - var_xeffs_dn9) * assign42550_e48013) + (assign42550_e48008 * (var_xedgefs_dn9 - var_xeffs_dn9))) / (2.0 * assign42550_e48017)))),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign42550_e48021;
        var_temp_dn4 = assign42550_e48021_d_n4;
        var_temp_dn6 = assign42550_e48021_d_n6;
        var_temp_dn7 = assign42550_e48021_d_n7;
        var_temp_dn8 = assign42550_e48021_d_n8;
        var_temp_dn9 = assign42550_e48021_d_n9;

        let (assign42560_e48033, assign42560_e48033_d_n4, assign42560_e48033_d_n6, assign42560_e48033_d_n7, assign42560_e48033_d_n8, assign42560_e48033_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42560_e48025: f64 = (var_temp1 * var_temp);
        let assign42560_e48027: f64 = (assign42560_e48025 * var_temp);
        let assign42560_e48030: f64 = (var_xedgefs - var_xstars);
        let assign42560_e48031: f64 = (assign42560_e48027 / assign42560_e48030);
        (assign42560_e48031, (((((((var_temp1_dn4 * var_temp) + (var_temp1 * var_temp_dn4)) * var_temp) + (assign42560_e48025 * var_temp_dn4)) * assign42560_e48030) - (assign42560_e48027 * (var_xedgefs_dn4 - var_xstars_dn4))) / (assign42560_e48030 * assign42560_e48030)), (((((((var_temp1_dn6 * var_temp) + (var_temp1 * var_temp_dn6)) * var_temp) + (assign42560_e48025 * var_temp_dn6)) * assign42560_e48030) - (assign42560_e48027 * (var_xedgefs_dn6 - var_xstars_dn6))) / (assign42560_e48030 * assign42560_e48030)), (((((((var_temp1_dn7 * var_temp) + (var_temp1 * var_temp_dn7)) * var_temp) + (assign42560_e48025 * var_temp_dn7)) * assign42560_e48030) - (assign42560_e48027 * (var_xedgefs_dn7 - var_xstars_dn7))) / (assign42560_e48030 * assign42560_e48030)), (((((((var_temp1_dn8 * var_temp) + (var_temp1 * var_temp_dn8)) * var_temp) + (assign42560_e48025 * var_temp_dn8)) * assign42560_e48030) - (assign42560_e48027 * (var_xedgefs_dn8 - var_xstars_dn8))) / (assign42560_e48030 * assign42560_e48030)), (((((((var_temp1_dn9 * var_temp) + (var_temp1 * var_temp_dn9)) * var_temp) + (assign42560_e48025 * var_temp_dn9)) * assign42560_e48030) - (assign42560_e48027 * (var_xedgefs_dn9 - var_xstars_dn9))) / (assign42560_e48030 * assign42560_e48030)),)
    } else {
        (var_qgsif, var_qgsif_dn4, var_qgsif_dn6, var_qgsif_dn7, var_qgsif_dn8, var_qgsif_dn9,)
    }
};
        var_qgsif = assign42560_e48033;
        var_qgsif_dn4 = assign42560_e48033_d_n4;
        var_qgsif_dn6 = assign42560_e48033_d_n6;
        var_qgsif_dn7 = assign42560_e48033_d_n7;
        var_qgsif_dn8 = assign42560_e48033_d_n8;
        var_qgsif_dn9 = assign42560_e48033_d_n9;

        let (assign42570_e48058, assign42570_e48058_d_n4, assign42570_e48058_d_n6, assign42570_e48058_d_n7, assign42570_e48058_d_n8, assign42570_e48058_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42570_e48038: f64 = (var_xedgefd - var_xeffd);
        let assign42570_e48040: f64 = assign42570_e48038;
        let assign42570_e48043: f64 = (var_xedgefd - var_xeffd);
        let assign42570_e48045: f64 = assign42570_e48043;
        let assign42570_e48048: f64 = (var_xedgefd - var_xeffd);
        let assign42570_e48050: f64 = assign42570_e48048;
        let assign42570_e48051: f64 = (assign42570_e48045 * assign42570_e48050);
        let assign42570_e48053: f64 = (assign42570_e48051 + 1.0);
        let assign42570_e48054: f64 = (assign42570_e48053).sqrt();
        let assign42570_e48055: f64 = (assign42570_e48040 + assign42570_e48054);
        let assign42570_e48056: f64 = (0.5 * assign42570_e48055);
        (assign42570_e48056, (0.5 * ((var_xedgefd_dn4 - var_xeffd_dn4) + ((((var_xedgefd_dn4 - var_xeffd_dn4) * assign42570_e48050) + (assign42570_e48045 * (var_xedgefd_dn4 - var_xeffd_dn4))) / (2.0 * assign42570_e48054)))), (0.5 * ((var_xedgefd_dn6 - var_xeffd_dn6) + ((((var_xedgefd_dn6 - var_xeffd_dn6) * assign42570_e48050) + (assign42570_e48045 * (var_xedgefd_dn6 - var_xeffd_dn6))) / (2.0 * assign42570_e48054)))), (0.5 * ((var_xedgefd_dn7 - var_xeffd_dn7) + ((((var_xedgefd_dn7 - var_xeffd_dn7) * assign42570_e48050) + (assign42570_e48045 * (var_xedgefd_dn7 - var_xeffd_dn7))) / (2.0 * assign42570_e48054)))), (0.5 * ((var_xedgefd_dn8 - var_xeffd_dn8) + ((((var_xedgefd_dn8 - var_xeffd_dn8) * assign42570_e48050) + (assign42570_e48045 * (var_xedgefd_dn8 - var_xeffd_dn8))) / (2.0 * assign42570_e48054)))), (0.5 * ((var_xedgefd_dn9 - var_xeffd_dn9) + ((((var_xedgefd_dn9 - var_xeffd_dn9) * assign42570_e48050) + (assign42570_e48045 * (var_xedgefd_dn9 - var_xeffd_dn9))) / (2.0 * assign42570_e48054)))),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign42570_e48058;
        var_temp_dn4 = assign42570_e48058_d_n4;
        var_temp_dn6 = assign42570_e48058_d_n6;
        var_temp_dn7 = assign42570_e48058_d_n7;
        var_temp_dn8 = assign42570_e48058_d_n8;
        var_temp_dn9 = assign42570_e48058_d_n9;

        let (assign42580_e48070, assign42580_e48070_d_n4, assign42580_e48070_d_n6, assign42580_e48070_d_n7, assign42580_e48070_d_n8, assign42580_e48070_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42580_e48062: f64 = (var_temp1 * var_temp);
        let assign42580_e48064: f64 = (assign42580_e48062 * var_temp);
        let assign42580_e48067: f64 = (var_xedgefd - var_xstard);
        let assign42580_e48068: f64 = (assign42580_e48064 / assign42580_e48067);
        (assign42580_e48068, (((((((var_temp1_dn4 * var_temp) + (var_temp1 * var_temp_dn4)) * var_temp) + (assign42580_e48062 * var_temp_dn4)) * assign42580_e48067) - (assign42580_e48064 * (var_xedgefd_dn4 - var_xstard_dn4))) / (assign42580_e48067 * assign42580_e48067)), (((((((var_temp1_dn6 * var_temp) + (var_temp1 * var_temp_dn6)) * var_temp) + (assign42580_e48062 * var_temp_dn6)) * assign42580_e48067) - (assign42580_e48064 * (var_xedgefd_dn6 - var_xstard_dn6))) / (assign42580_e48067 * assign42580_e48067)), (((((((var_temp1_dn7 * var_temp) + (var_temp1 * var_temp_dn7)) * var_temp) + (assign42580_e48062 * var_temp_dn7)) * assign42580_e48067) - (assign42580_e48064 * (var_xedgefd_dn7 - var_xstard_dn7))) / (assign42580_e48067 * assign42580_e48067)), (((((((var_temp1_dn8 * var_temp) + (var_temp1 * var_temp_dn8)) * var_temp) + (assign42580_e48062 * var_temp_dn8)) * assign42580_e48067) - (assign42580_e48064 * (var_xedgefd_dn8 - var_xstard_dn8))) / (assign42580_e48067 * assign42580_e48067)), (((((((var_temp1_dn9 * var_temp) + (var_temp1 * var_temp_dn9)) * var_temp) + (assign42580_e48062 * var_temp_dn9)) * assign42580_e48067) - (assign42580_e48064 * (var_xedgefd_dn9 - var_xstard_dn9))) / (assign42580_e48067 * assign42580_e48067)),)
    } else {
        (var_qgdif, var_qgdif_dn4, var_qgdif_dn6, var_qgdif_dn7, var_qgdif_dn8, var_qgdif_dn9,)
    }
};
        var_qgdif = assign42580_e48070;
        var_qgdif_dn4 = assign42580_e48070_d_n4;
        var_qgdif_dn6 = assign42580_e48070_d_n6;
        var_qgdif_dn7 = assign42580_e48070_d_n7;
        var_qgdif_dn8 = assign42580_e48070_d_n8;
        var_qgdif_dn9 = assign42580_e48070_d_n9;

        let (assign42590_e48095, assign42590_e48095_d_n4, assign42590_e48095_d_n6, assign42590_e48095_d_n7, assign42590_e48095_d_n8, assign42590_e48095_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42590_e48075: f64 = (var_xedgebs - var_xeffs);
        let assign42590_e48077: f64 = assign42590_e48075;
        let assign42590_e48080: f64 = (var_xedgebs - var_xeffs);
        let assign42590_e48082: f64 = assign42590_e48080;
        let assign42590_e48085: f64 = (var_xedgebs - var_xeffs);
        let assign42590_e48087: f64 = assign42590_e48085;
        let assign42590_e48088: f64 = (assign42590_e48082 * assign42590_e48087);
        let assign42590_e48090: f64 = (assign42590_e48088 + 1.0);
        let assign42590_e48091: f64 = (assign42590_e48090).sqrt();
        let assign42590_e48092: f64 = (assign42590_e48077 + assign42590_e48091);
        let assign42590_e48093: f64 = (0.5 * assign42590_e48092);
        (assign42590_e48093, (0.5 * ((var_xedgebs_dn4 - var_xeffs_dn4) + ((((var_xedgebs_dn4 - var_xeffs_dn4) * assign42590_e48087) + (assign42590_e48082 * (var_xedgebs_dn4 - var_xeffs_dn4))) / (2.0 * assign42590_e48091)))), (0.5 * ((var_xedgebs_dn6 - var_xeffs_dn6) + ((((var_xedgebs_dn6 - var_xeffs_dn6) * assign42590_e48087) + (assign42590_e48082 * (var_xedgebs_dn6 - var_xeffs_dn6))) / (2.0 * assign42590_e48091)))), (0.5 * ((var_xedgebs_dn7 - var_xeffs_dn7) + ((((var_xedgebs_dn7 - var_xeffs_dn7) * assign42590_e48087) + (assign42590_e48082 * (var_xedgebs_dn7 - var_xeffs_dn7))) / (2.0 * assign42590_e48091)))), (0.5 * ((var_xedgebs_dn8 - var_xeffs_dn8) + ((((var_xedgebs_dn8 - var_xeffs_dn8) * assign42590_e48087) + (assign42590_e48082 * (var_xedgebs_dn8 - var_xeffs_dn8))) / (2.0 * assign42590_e48091)))), (0.5 * ((var_xedgebs_dn9 - var_xeffs_dn9) + ((((var_xedgebs_dn9 - var_xeffs_dn9) * assign42590_e48087) + (assign42590_e48082 * (var_xedgebs_dn9 - var_xeffs_dn9))) / (2.0 * assign42590_e48091)))),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign42590_e48095;
        var_temp_dn4 = assign42590_e48095_d_n4;
        var_temp_dn6 = assign42590_e48095_d_n6;
        var_temp_dn7 = assign42590_e48095_d_n7;
        var_temp_dn8 = assign42590_e48095_d_n8;
        var_temp_dn9 = assign42590_e48095_d_n9;

        let (assign42600_e48107, assign42600_e48107_d_n4, assign42600_e48107_d_n6, assign42600_e48107_d_n7, assign42600_e48107_d_n8, assign42600_e48107_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42600_e48099: f64 = (var_temp2 * var_temp);
        let assign42600_e48101: f64 = (assign42600_e48099 * var_temp);
        let assign42600_e48104: f64 = (var_xedgebs - var_xstars);
        let assign42600_e48105: f64 = (assign42600_e48101 / assign42600_e48104);
        (assign42600_e48105, (((((((var_temp2_dn4 * var_temp) + (var_temp2 * var_temp_dn4)) * var_temp) + (assign42600_e48099 * var_temp_dn4)) * assign42600_e48104) - (assign42600_e48101 * (var_xedgebs_dn4 - var_xstars_dn4))) / (assign42600_e48104 * assign42600_e48104)), (((((((var_temp2_dn6 * var_temp) + (var_temp2 * var_temp_dn6)) * var_temp) + (assign42600_e48099 * var_temp_dn6)) * assign42600_e48104) - (assign42600_e48101 * (var_xedgebs_dn6 - var_xstars_dn6))) / (assign42600_e48104 * assign42600_e48104)), (((((((var_temp2_dn7 * var_temp) + (var_temp2 * var_temp_dn7)) * var_temp) + (assign42600_e48099 * var_temp_dn7)) * assign42600_e48104) - (assign42600_e48101 * (var_xedgebs_dn7 - var_xstars_dn7))) / (assign42600_e48104 * assign42600_e48104)), (((((((var_temp2_dn8 * var_temp) + (var_temp2 * var_temp_dn8)) * var_temp) + (assign42600_e48099 * var_temp_dn8)) * assign42600_e48104) - (assign42600_e48101 * (var_xedgebs_dn8 - var_xstars_dn8))) / (assign42600_e48104 * assign42600_e48104)), (((((((var_temp2_dn9 * var_temp) + (var_temp2 * var_temp_dn9)) * var_temp) + (assign42600_e48099 * var_temp_dn9)) * assign42600_e48104) - (assign42600_e48101 * (var_xedgebs_dn9 - var_xstars_dn9))) / (assign42600_e48104 * assign42600_e48104)),)
    } else {
        (var_qbsif, var_qbsif_dn4, var_qbsif_dn6, var_qbsif_dn7, var_qbsif_dn8, var_qbsif_dn9,)
    }
};
        var_qbsif = assign42600_e48107;
        var_qbsif_dn4 = assign42600_e48107_d_n4;
        var_qbsif_dn6 = assign42600_e48107_d_n6;
        var_qbsif_dn7 = assign42600_e48107_d_n7;
        var_qbsif_dn8 = assign42600_e48107_d_n8;
        var_qbsif_dn9 = assign42600_e48107_d_n9;

        let (assign42610_e48132, assign42610_e48132_d_n4, assign42610_e48132_d_n6, assign42610_e48132_d_n7, assign42610_e48132_d_n8, assign42610_e48132_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42610_e48112: f64 = (var_xedgebd - var_xeffd);
        let assign42610_e48114: f64 = assign42610_e48112;
        let assign42610_e48117: f64 = (var_xedgebd - var_xeffd);
        let assign42610_e48119: f64 = assign42610_e48117;
        let assign42610_e48122: f64 = (var_xedgebd - var_xeffd);
        let assign42610_e48124: f64 = assign42610_e48122;
        let assign42610_e48125: f64 = (assign42610_e48119 * assign42610_e48124);
        let assign42610_e48127: f64 = (assign42610_e48125 + 1.0);
        let assign42610_e48128: f64 = (assign42610_e48127).sqrt();
        let assign42610_e48129: f64 = (assign42610_e48114 + assign42610_e48128);
        let assign42610_e48130: f64 = (0.5 * assign42610_e48129);
        (assign42610_e48130, (0.5 * ((var_xedgebd_dn4 - var_xeffd_dn4) + ((((var_xedgebd_dn4 - var_xeffd_dn4) * assign42610_e48124) + (assign42610_e48119 * (var_xedgebd_dn4 - var_xeffd_dn4))) / (2.0 * assign42610_e48128)))), (0.5 * ((var_xedgebd_dn6 - var_xeffd_dn6) + ((((var_xedgebd_dn6 - var_xeffd_dn6) * assign42610_e48124) + (assign42610_e48119 * (var_xedgebd_dn6 - var_xeffd_dn6))) / (2.0 * assign42610_e48128)))), (0.5 * ((var_xedgebd_dn7 - var_xeffd_dn7) + ((((var_xedgebd_dn7 - var_xeffd_dn7) * assign42610_e48124) + (assign42610_e48119 * (var_xedgebd_dn7 - var_xeffd_dn7))) / (2.0 * assign42610_e48128)))), (0.5 * ((var_xedgebd_dn8 - var_xeffd_dn8) + ((((var_xedgebd_dn8 - var_xeffd_dn8) * assign42610_e48124) + (assign42610_e48119 * (var_xedgebd_dn8 - var_xeffd_dn8))) / (2.0 * assign42610_e48128)))), (0.5 * ((var_xedgebd_dn9 - var_xeffd_dn9) + ((((var_xedgebd_dn9 - var_xeffd_dn9) * assign42610_e48124) + (assign42610_e48119 * (var_xedgebd_dn9 - var_xeffd_dn9))) / (2.0 * assign42610_e48128)))),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign42610_e48132;
        var_temp_dn4 = assign42610_e48132_d_n4;
        var_temp_dn6 = assign42610_e48132_d_n6;
        var_temp_dn7 = assign42610_e48132_d_n7;
        var_temp_dn8 = assign42610_e48132_d_n8;
        var_temp_dn9 = assign42610_e48132_d_n9;

        let (assign42620_e48144, assign42620_e48144_d_n4, assign42620_e48144_d_n6, assign42620_e48144_d_n7, assign42620_e48144_d_n8, assign42620_e48144_d_n9,) = {
    if (var_guard1235 != 0.0) {
        let assign42620_e48136: f64 = (var_temp2 * var_temp);
        let assign42620_e48138: f64 = (assign42620_e48136 * var_temp);
        let assign42620_e48141: f64 = (var_xedgebd - var_xstard);
        let assign42620_e48142: f64 = (assign42620_e48138 / assign42620_e48141);
        (assign42620_e48142, (((((((var_temp2_dn4 * var_temp) + (var_temp2 * var_temp_dn4)) * var_temp) + (assign42620_e48136 * var_temp_dn4)) * assign42620_e48141) - (assign42620_e48138 * (var_xedgebd_dn4 - var_xstard_dn4))) / (assign42620_e48141 * assign42620_e48141)), (((((((var_temp2_dn6 * var_temp) + (var_temp2 * var_temp_dn6)) * var_temp) + (assign42620_e48136 * var_temp_dn6)) * assign42620_e48141) - (assign42620_e48138 * (var_xedgebd_dn6 - var_xstard_dn6))) / (assign42620_e48141 * assign42620_e48141)), (((((((var_temp2_dn7 * var_temp) + (var_temp2 * var_temp_dn7)) * var_temp) + (assign42620_e48136 * var_temp_dn7)) * assign42620_e48141) - (assign42620_e48138 * (var_xedgebd_dn7 - var_xstard_dn7))) / (assign42620_e48141 * assign42620_e48141)), (((((((var_temp2_dn8 * var_temp) + (var_temp2 * var_temp_dn8)) * var_temp) + (assign42620_e48136 * var_temp_dn8)) * assign42620_e48141) - (assign42620_e48138 * (var_xedgebd_dn8 - var_xstard_dn8))) / (assign42620_e48141 * assign42620_e48141)), (((((((var_temp2_dn9 * var_temp) + (var_temp2 * var_temp_dn9)) * var_temp) + (assign42620_e48136 * var_temp_dn9)) * assign42620_e48141) - (assign42620_e48138 * (var_xedgebd_dn9 - var_xstard_dn9))) / (assign42620_e48141 * assign42620_e48141)),)
    } else {
        (var_qbdif, var_qbdif_dn4, var_qbdif_dn6, var_qbdif_dn7, var_qbdif_dn8, var_qbdif_dn9,)
    }
};
        var_qbdif = assign42620_e48144;
        var_qbdif_dn4 = assign42620_e48144_d_n4;
        var_qbdif_dn6 = assign42620_e48144_d_n6;
        var_qbdif_dn7 = assign42620_e48144_d_n7;
        var_qbdif_dn8 = assign42620_e48144_d_n8;
        var_qbdif_dn9 = assign42620_e48144_d_n9;

        let (assign42630_e48149, assign42630_e48149_d_n4, assign42630_e48149_d_n6, assign42630_e48149_d_n7, assign42630_e48149_d_n8, assign42630_e48149_d_n9,) = {
    if (var_guard1235 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qgsif, var_qgsif_dn4, var_qgsif_dn6, var_qgsif_dn7, var_qgsif_dn8, var_qgsif_dn9,)
    }
};
        var_qgsif = assign42630_e48149;
        var_qgsif_dn4 = assign42630_e48149_d_n4;
        var_qgsif_dn6 = assign42630_e48149_d_n6;
        var_qgsif_dn7 = assign42630_e48149_d_n7;
        var_qgsif_dn8 = assign42630_e48149_d_n8;
        var_qgsif_dn9 = assign42630_e48149_d_n9;

        *var_lambdab_slot = var_lambdab;
        *var_lambdab_dn4_slot = var_lambdab_dn4;
        *var_lambdab_dn6_slot = var_lambdab_dn6;
        *var_lambdab_dn7_slot = var_lambdab_dn7;
        *var_lambdab_dn8_slot = var_lambdab_dn8;
        *var_lambdab_dn9_slot = var_lambdab_dn9;
        *var_lambdaf_slot = var_lambdaf;
        *var_lambdaf_dn4_slot = var_lambdaf_dn4;
        *var_lambdaf_dn6_slot = var_lambdaf_dn6;
        *var_lambdaf_dn7_slot = var_lambdaf_dn7;
        *var_lambdaf_dn8_slot = var_lambdaf_dn8;
        *var_lambdaf_dn9_slot = var_lambdaf_dn9;
        *var_qbdif_slot = var_qbdif;
        *var_qbdif_dn4_slot = var_qbdif_dn4;
        *var_qbdif_dn6_slot = var_qbdif_dn6;
        *var_qbdif_dn7_slot = var_qbdif_dn7;
        *var_qbdif_dn8_slot = var_qbdif_dn8;
        *var_qbdif_dn9_slot = var_qbdif_dn9;
        *var_qbsif_slot = var_qbsif;
        *var_qbsif_dn4_slot = var_qbsif_dn4;
        *var_qbsif_dn6_slot = var_qbsif_dn6;
        *var_qbsif_dn7_slot = var_qbsif_dn7;
        *var_qbsif_dn8_slot = var_qbsif_dn8;
        *var_qbsif_dn9_slot = var_qbsif_dn9;
        *var_qgdif_slot = var_qgdif;
        *var_qgdif_dn4_slot = var_qgdif_dn4;
        *var_qgdif_dn6_slot = var_qgdif_dn6;
        *var_qgdif_dn7_slot = var_qgdif_dn7;
        *var_qgdif_dn8_slot = var_qgdif_dn8;
        *var_qgdif_dn9_slot = var_qgdif_dn9;
        *var_qgsif_slot = var_qgsif;
        *var_qgsif_dn4_slot = var_qgsif_dn4;
        *var_qgsif_dn6_slot = var_qgsif_dn6;
        *var_qgsif_dn7_slot = var_qgsif_dn7;
        *var_qgsif_dn8_slot = var_qgsif_dn8;
        *var_qgsif_dn9_slot = var_qgsif_dn9;
        *var_temp_slot = var_temp;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_xalphab_slot = var_xalphab;
        *var_xalphab_dn4_slot = var_xalphab_dn4;
        *var_xalphab_dn6_slot = var_xalphab_dn6;
        *var_xalphab_dn7_slot = var_xalphab_dn7;
        *var_xalphab_dn8_slot = var_xalphab_dn8;
        *var_xalphab_dn9_slot = var_xalphab_dn9;
        *var_xalphaf_slot = var_xalphaf;
        *var_xalphaf_dn4_slot = var_xalphaf_dn4;
        *var_xalphaf_dn6_slot = var_xalphaf_dn6;
        *var_xalphaf_dn7_slot = var_xalphaf_dn7;
        *var_xalphaf_dn8_slot = var_xalphaf_dn8;
        *var_xalphaf_dn9_slot = var_xalphaf_dn9;
        *var_xedgebd_slot = var_xedgebd;
        *var_xedgebd_dn4_slot = var_xedgebd_dn4;
        *var_xedgebd_dn6_slot = var_xedgebd_dn6;
        *var_xedgebd_dn7_slot = var_xedgebd_dn7;
        *var_xedgebd_dn8_slot = var_xedgebd_dn8;
        *var_xedgebd_dn9_slot = var_xedgebd_dn9;
        *var_xedgebs_slot = var_xedgebs;
        *var_xedgebs_dn4_slot = var_xedgebs_dn4;
        *var_xedgebs_dn6_slot = var_xedgebs_dn6;
        *var_xedgebs_dn7_slot = var_xedgebs_dn7;
        *var_xedgebs_dn8_slot = var_xedgebs_dn8;
        *var_xedgebs_dn9_slot = var_xedgebs_dn9;
        *var_xedgefd_slot = var_xedgefd;
        *var_xedgefd_dn4_slot = var_xedgefd_dn4;
        *var_xedgefd_dn6_slot = var_xedgefd_dn6;
        *var_xedgefd_dn7_slot = var_xedgefd_dn7;
        *var_xedgefd_dn8_slot = var_xedgefd_dn8;
        *var_xedgefd_dn9_slot = var_xedgefd_dn9;
        *var_xedgefs_slot = var_xedgefs;
        *var_xedgefs_dn4_slot = var_xedgefs_dn4;
        *var_xedgefs_dn6_slot = var_xedgefs_dn6;
        *var_xedgefs_dn7_slot = var_xedgefs_dn7;
        *var_xedgefs_dn8_slot = var_xedgefs_dn8;
        *var_xedgefs_dn9_slot = var_xedgefs_dn9;
        *var_xeffd_slot = var_xeffd;
        *var_xeffd_dn4_slot = var_xeffd_dn4;
        *var_xeffd_dn6_slot = var_xeffd_dn6;
        *var_xeffd_dn7_slot = var_xeffd_dn7;
        *var_xeffd_dn8_slot = var_xeffd_dn8;
        *var_xeffd_dn9_slot = var_xeffd_dn9;
        *var_xstard_slot = var_xstard;
        *var_xstard_dn4_slot = var_xstard_dn4;
        *var_xstard_dn6_slot = var_xstard_dn6;
        *var_xstard_dn7_slot = var_xstard_dn7;
        *var_xstard_dn8_slot = var_xstard_dn8;
        *var_xstard_dn9_slot = var_xstard_dn9;
        *var_xstars_slot = var_xstars;
        *var_xstars_dn4_slot = var_xstars_dn4;
        *var_xstars_dn6_slot = var_xstars_dn6;
        *var_xstars_dn7_slot = var_xstars_dn7;
        *var_xstars_dn8_slot = var_xstars_dn8;
        *var_xstars_dn9_slot = var_xstars_dn9;
    }

    pub(super) fn stamp_transient_block_119(
        p: &Parameters,
        var_adrain_i: f64,
        var_asource_i: f64,
        var_cfr_i: f64,
        var_cfr_i_dn4: f64,
        var_cfr_i_dn6: f64,
        var_cfr_i_dn7: f64,
        var_cfr_i_dn8: f64,
        var_cfr_i_dn9: f64,
        var_cfrd_i: f64,
        var_cfrd_i_dn4: f64,
        var_cfrd_i_dn6: f64,
        var_cfrd_i_dn7: f64,
        var_cfrd_i_dn8: f64,
        var_cfrd_i_dn9: f64,
        var_cgbov_i: f64,
        var_cgbov_i_dn4: f64,
        var_cgbov_i_dn6: f64,
        var_cgbov_i_dn7: f64,
        var_cgbov_i_dn8: f64,
        var_cgbov_i_dn9: f64,
        var_cov_i: f64,
        var_cov_i_dn4: f64,
        var_cov_i_dn6: f64,
        var_cov_i_dn7: f64,
        var_cov_i_dn8: f64,
        var_cov_i_dn9: f64,
        var_covd_i: f64,
        var_covd_i_dn4: f64,
        var_covd_i_dn6: f64,
        var_covd_i_dn7: f64,
        var_covd_i_dn8: f64,
        var_covd_i_dn9: f64,
        var_covdl_i: f64,
        var_covdlb_i: f64,
        var_cox2init: f64,
        var_csd_i: f64,
        var_csdbp_i: f64,
        var_cth_i: f64,
        var_cth_i_dn4: f64,
        var_cth_i_dn6: f64,
        var_cth_i_dn7: f64,
        var_cth_i_dn8: f64,
        var_cth_i_dn9: f64,
        var_dleff_ac: f64,
        var_dleff_ac_dn4: f64,
        var_dleff_ac_dn6: f64,
        var_dleff_ac_dn7: f64,
        var_dleff_ac_dn8: f64,
        var_dleff_ac_dn9: f64,
        var_dtc: f64,
        var_dtc_dn4: f64,
        var_guard1235: f64,
        var_ids: f64,
        var_ids_dn4: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_ids_dn8: f64,
        var_ids_dn9: f64,
        var_ids_edge: f64,
        var_ids_edge_dn4: f64,
        var_ids_edge_dn6: f64,
        var_ids_edge_dn7: f64,
        var_ids_edge_dn8: f64,
        var_ids_edge_dn9: f64,
        var_igd: f64,
        var_igd_dn4: f64,
        var_igd_dn6: f64,
        var_igd_dn7: f64,
        var_igd_dn8: f64,
        var_igd_dn9: f64,
        var_igidl: f64,
        var_igidl_dn4: f64,
        var_igidl_dn6: f64,
        var_igidl_dn7: f64,
        var_igidl_dn8: f64,
        var_igidl_dn9: f64,
        var_igisl: f64,
        var_igisl_dn4: f64,
        var_igisl_dn6: f64,
        var_igisl_dn7: f64,
        var_igisl_dn8: f64,
        var_igisl_dn9: f64,
        var_igs: f64,
        var_igs_dn4: f64,
        var_igs_dn6: f64,
        var_igs_dn7: f64,
        var_igs_dn8: f64,
        var_igs_dn9: f64,
        var_iimpact: f64,
        var_iimpact_dn4: f64,
        var_iimpact_dn6: f64,
        var_iimpact_dn7: f64,
        var_iimpact_dn8: f64,
        var_iimpact_dn9: f64,
        var_ithpwr: f64,
        var_ithpwr_dn4: f64,
        var_ithpwr_dn6: f64,
        var_ithpwr_dn7: f64,
        var_ithpwr_dn8: f64,
        var_ithpwr_dn9: f64,
        var_ithrc: f64,
        var_ithrc_dn4: f64,
        var_ithrc_dn6: f64,
        var_ithrc_dn7: f64,
        var_ithrc_dn8: f64,
        var_ithrc_dn9: f64,
        var_mult_i_int: f64,
        var_pdrain_i: f64,
        var_psource_i: f64,
        var_sigvds: f64,
        var_swshe_i: f64,
        var_vdbu: f64,
        var_vdbu_dn6: f64,
        var_vdbu_dn7: f64,
        var_vdbu_dn8: f64,
        var_vds: f64,
        var_vds_dn6: f64,
        var_vds_dn7: f64,
        var_vgb: f64,
        var_vgb_dn6: f64,
        var_vgb_dn7: f64,
        var_vgb_dn8: f64,
        var_vgb_dn9: f64,
        var_vgdu: f64,
        var_vgdu_dn6: f64,
        var_vgdu_dn7: f64,
        var_vgdu_dn9: f64,
        var_vgsu: f64,
        var_vgsu_dn6: f64,
        var_vgsu_dn9: f64,
        var_vovdcv: f64,
        var_vovdcv_dn4: f64,
        var_vovdcv_dn6: f64,
        var_vovdcv_dn7: f64,
        var_vovdcv_dn8: f64,
        var_vovdcv_dn9: f64,
        var_vovscv: f64,
        var_vovscv_dn4: f64,
        var_vovscv_dn6: f64,
        var_vovscv_dn7: f64,
        var_vovscv_dn8: f64,
        var_vovscv_dn9: f64,
        var_vsbu: f64,
        var_vsbu_dn6: f64,
        var_vsbu_dn8: f64,
        var_xg20shift_ac: f64,
        var_xg20shift_ac_dn4: f64,
        var_xg20shift_ac_dn6: f64,
        var_xg20shift_ac_dn7: f64,
        var_xg20shift_ac_dn8: f64,
        var_xg20shift_ac_dn9: f64,
        var_guard1236_slot: &mut f64,
        var_guard1239_slot: &mut f64,
        var_guard1245_slot: &mut f64,
        var_idse_slot: &mut f64,
        var_idse_dn4_slot: &mut f64,
        var_idse_dn6_slot: &mut f64,
        var_idse_dn7_slot: &mut f64,
        var_idse_dn8_slot: &mut f64,
        var_idse_dn9_slot: &mut f64,
        var_igde_slot: &mut f64,
        var_igde_dn4_slot: &mut f64,
        var_igde_dn6_slot: &mut f64,
        var_igde_dn7_slot: &mut f64,
        var_igde_dn8_slot: &mut f64,
        var_igde_dn9_slot: &mut f64,
        var_igidle_slot: &mut f64,
        var_igidle_dn4_slot: &mut f64,
        var_igidle_dn6_slot: &mut f64,
        var_igidle_dn7_slot: &mut f64,
        var_igidle_dn8_slot: &mut f64,
        var_igidle_dn9_slot: &mut f64,
        var_igisle_slot: &mut f64,
        var_igisle_dn4_slot: &mut f64,
        var_igisle_dn6_slot: &mut f64,
        var_igisle_dn7_slot: &mut f64,
        var_igisle_dn8_slot: &mut f64,
        var_igisle_dn9_slot: &mut f64,
        var_igse_slot: &mut f64,
        var_igse_dn4_slot: &mut f64,
        var_igse_dn6_slot: &mut f64,
        var_igse_dn7_slot: &mut f64,
        var_igse_dn8_slot: &mut f64,
        var_igse_dn9_slot: &mut f64,
        var_ithpwre_slot: &mut f64,
        var_ithpwre_dn4_slot: &mut f64,
        var_ithpwre_dn6_slot: &mut f64,
        var_ithpwre_dn7_slot: &mut f64,
        var_ithpwre_dn8_slot: &mut f64,
        var_ithpwre_dn9_slot: &mut f64,
        var_ithrce_slot: &mut f64,
        var_ithrce_dn4_slot: &mut f64,
        var_ithrce_dn6_slot: &mut f64,
        var_ithrce_dn7_slot: &mut f64,
        var_ithrce_dn8_slot: &mut f64,
        var_ithrce_dn9_slot: &mut f64,
        var_qb_slot: &mut f64,
        var_qb_dn4_slot: &mut f64,
        var_qb_dn6_slot: &mut f64,
        var_qb_dn7_slot: &mut f64,
        var_qb_dn8_slot: &mut f64,
        var_qb_dn9_slot: &mut f64,
        var_qbdif_slot: &mut f64,
        var_qbdif_dn4_slot: &mut f64,
        var_qbdif_dn6_slot: &mut f64,
        var_qbdif_dn7_slot: &mut f64,
        var_qbdif_dn8_slot: &mut f64,
        var_qbdif_dn9_slot: &mut f64,
        var_qbsif_slot: &mut f64,
        var_qbsif_dn4_slot: &mut f64,
        var_qbsif_dn6_slot: &mut f64,
        var_qbsif_dn7_slot: &mut f64,
        var_qbsif_dn8_slot: &mut f64,
        var_qbsif_dn9_slot: &mut f64,
        var_qd_slot: &mut f64,
        var_qd_dn4_slot: &mut f64,
        var_qd_dn6_slot: &mut f64,
        var_qd_dn7_slot: &mut f64,
        var_qd_dn8_slot: &mut f64,
        var_qd_dn9_slot: &mut f64,
        var_qdse_slot: &mut f64,
        var_qdse_dn6_slot: &mut f64,
        var_qdse_dn7_slot: &mut f64,
        var_qdsub_slot: &mut f64,
        var_qdsub_dn6_slot: &mut f64,
        var_qdsub_dn7_slot: &mut f64,
        var_qdsub_dn8_slot: &mut f64,
        var_qg_slot: &mut f64,
        var_qg_dn4_slot: &mut f64,
        var_qg_dn6_slot: &mut f64,
        var_qg_dn7_slot: &mut f64,
        var_qg_dn8_slot: &mut f64,
        var_qg_dn9_slot: &mut f64,
        var_qgbe_slot: &mut f64,
        var_qgbe_dn4_slot: &mut f64,
        var_qgbe_dn6_slot: &mut f64,
        var_qgbe_dn7_slot: &mut f64,
        var_qgbe_dn8_slot: &mut f64,
        var_qgbe_dn9_slot: &mut f64,
        var_qgde_slot: &mut f64,
        var_qgde_dn4_slot: &mut f64,
        var_qgde_dn6_slot: &mut f64,
        var_qgde_dn7_slot: &mut f64,
        var_qgde_dn8_slot: &mut f64,
        var_qgde_dn9_slot: &mut f64,
        var_qgdif_slot: &mut f64,
        var_qgdif_dn4_slot: &mut f64,
        var_qgdif_dn6_slot: &mut f64,
        var_qgdif_dn7_slot: &mut f64,
        var_qgdif_dn8_slot: &mut f64,
        var_qgdif_dn9_slot: &mut f64,
        var_qgse_slot: &mut f64,
        var_qgse_dn4_slot: &mut f64,
        var_qgse_dn6_slot: &mut f64,
        var_qgse_dn7_slot: &mut f64,
        var_qgse_dn8_slot: &mut f64,
        var_qgse_dn9_slot: &mut f64,
        var_qgsif_slot: &mut f64,
        var_qgsif_dn4_slot: &mut f64,
        var_qgsif_dn6_slot: &mut f64,
        var_qgsif_dn7_slot: &mut f64,
        var_qgsif_dn8_slot: &mut f64,
        var_qgsif_dn9_slot: &mut f64,
        var_qovd_slot: &mut f64,
        var_qovd_dn4_slot: &mut f64,
        var_qovd_dn6_slot: &mut f64,
        var_qovd_dn7_slot: &mut f64,
        var_qovd_dn8_slot: &mut f64,
        var_qovd_dn9_slot: &mut f64,
        var_qovs_slot: &mut f64,
        var_qovs_dn4_slot: &mut f64,
        var_qovs_dn6_slot: &mut f64,
        var_qovs_dn7_slot: &mut f64,
        var_qovs_dn8_slot: &mut f64,
        var_qovs_dn9_slot: &mut f64,
        var_qs_slot: &mut f64,
        var_qs_dn4_slot: &mut f64,
        var_qs_dn6_slot: &mut f64,
        var_qs_dn7_slot: &mut f64,
        var_qs_dn8_slot: &mut f64,
        var_qs_dn9_slot: &mut f64,
        var_qssub_slot: &mut f64,
        var_qssub_dn6_slot: &mut f64,
        var_qssub_dn8_slot: &mut f64,
        var_qth_slot: &mut f64,
        var_qth_dn4_slot: &mut f64,
        var_qth_dn6_slot: &mut f64,
        var_qth_dn7_slot: &mut f64,
        var_qth_dn8_slot: &mut f64,
        var_qth_dn9_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_temp_q_slot: &mut f64,
        var_temp_q_dn4_slot: &mut f64,
        var_temp_q_dn6_slot: &mut f64,
        var_temp_q_dn7_slot: &mut f64,
        var_temp_q_dn8_slot: &mut f64,
        var_temp_q_dn9_slot: &mut f64,
    ) {
        let mut var_guard1236: f64 = *var_guard1236_slot;
        let mut var_guard1239: f64 = *var_guard1239_slot;
        let mut var_guard1245: f64 = *var_guard1245_slot;
        let mut var_idse: f64 = *var_idse_slot;
        let mut var_idse_dn4: f64 = *var_idse_dn4_slot;
        let mut var_idse_dn6: f64 = *var_idse_dn6_slot;
        let mut var_idse_dn7: f64 = *var_idse_dn7_slot;
        let mut var_idse_dn8: f64 = *var_idse_dn8_slot;
        let mut var_idse_dn9: f64 = *var_idse_dn9_slot;
        let mut var_igde: f64 = *var_igde_slot;
        let mut var_igde_dn4: f64 = *var_igde_dn4_slot;
        let mut var_igde_dn6: f64 = *var_igde_dn6_slot;
        let mut var_igde_dn7: f64 = *var_igde_dn7_slot;
        let mut var_igde_dn8: f64 = *var_igde_dn8_slot;
        let mut var_igde_dn9: f64 = *var_igde_dn9_slot;
        let mut var_igidle: f64 = *var_igidle_slot;
        let mut var_igidle_dn4: f64 = *var_igidle_dn4_slot;
        let mut var_igidle_dn6: f64 = *var_igidle_dn6_slot;
        let mut var_igidle_dn7: f64 = *var_igidle_dn7_slot;
        let mut var_igidle_dn8: f64 = *var_igidle_dn8_slot;
        let mut var_igidle_dn9: f64 = *var_igidle_dn9_slot;
        let mut var_igisle: f64 = *var_igisle_slot;
        let mut var_igisle_dn4: f64 = *var_igisle_dn4_slot;
        let mut var_igisle_dn6: f64 = *var_igisle_dn6_slot;
        let mut var_igisle_dn7: f64 = *var_igisle_dn7_slot;
        let mut var_igisle_dn8: f64 = *var_igisle_dn8_slot;
        let mut var_igisle_dn9: f64 = *var_igisle_dn9_slot;
        let mut var_igse: f64 = *var_igse_slot;
        let mut var_igse_dn4: f64 = *var_igse_dn4_slot;
        let mut var_igse_dn6: f64 = *var_igse_dn6_slot;
        let mut var_igse_dn7: f64 = *var_igse_dn7_slot;
        let mut var_igse_dn8: f64 = *var_igse_dn8_slot;
        let mut var_igse_dn9: f64 = *var_igse_dn9_slot;
        let mut var_ithpwre: f64 = *var_ithpwre_slot;
        let mut var_ithpwre_dn4: f64 = *var_ithpwre_dn4_slot;
        let mut var_ithpwre_dn6: f64 = *var_ithpwre_dn6_slot;
        let mut var_ithpwre_dn7: f64 = *var_ithpwre_dn7_slot;
        let mut var_ithpwre_dn8: f64 = *var_ithpwre_dn8_slot;
        let mut var_ithpwre_dn9: f64 = *var_ithpwre_dn9_slot;
        let mut var_ithrce: f64 = *var_ithrce_slot;
        let mut var_ithrce_dn4: f64 = *var_ithrce_dn4_slot;
        let mut var_ithrce_dn6: f64 = *var_ithrce_dn6_slot;
        let mut var_ithrce_dn7: f64 = *var_ithrce_dn7_slot;
        let mut var_ithrce_dn8: f64 = *var_ithrce_dn8_slot;
        let mut var_ithrce_dn9: f64 = *var_ithrce_dn9_slot;
        let mut var_qb: f64 = *var_qb_slot;
        let mut var_qb_dn4: f64 = *var_qb_dn4_slot;
        let mut var_qb_dn6: f64 = *var_qb_dn6_slot;
        let mut var_qb_dn7: f64 = *var_qb_dn7_slot;
        let mut var_qb_dn8: f64 = *var_qb_dn8_slot;
        let mut var_qb_dn9: f64 = *var_qb_dn9_slot;
        let mut var_qbdif: f64 = *var_qbdif_slot;
        let mut var_qbdif_dn4: f64 = *var_qbdif_dn4_slot;
        let mut var_qbdif_dn6: f64 = *var_qbdif_dn6_slot;
        let mut var_qbdif_dn7: f64 = *var_qbdif_dn7_slot;
        let mut var_qbdif_dn8: f64 = *var_qbdif_dn8_slot;
        let mut var_qbdif_dn9: f64 = *var_qbdif_dn9_slot;
        let mut var_qbsif: f64 = *var_qbsif_slot;
        let mut var_qbsif_dn4: f64 = *var_qbsif_dn4_slot;
        let mut var_qbsif_dn6: f64 = *var_qbsif_dn6_slot;
        let mut var_qbsif_dn7: f64 = *var_qbsif_dn7_slot;
        let mut var_qbsif_dn8: f64 = *var_qbsif_dn8_slot;
        let mut var_qbsif_dn9: f64 = *var_qbsif_dn9_slot;
        let mut var_qd: f64 = *var_qd_slot;
        let mut var_qd_dn4: f64 = *var_qd_dn4_slot;
        let mut var_qd_dn6: f64 = *var_qd_dn6_slot;
        let mut var_qd_dn7: f64 = *var_qd_dn7_slot;
        let mut var_qd_dn8: f64 = *var_qd_dn8_slot;
        let mut var_qd_dn9: f64 = *var_qd_dn9_slot;
        let mut var_qdse: f64 = *var_qdse_slot;
        let mut var_qdse_dn6: f64 = *var_qdse_dn6_slot;
        let mut var_qdse_dn7: f64 = *var_qdse_dn7_slot;
        let mut var_qdsub: f64 = *var_qdsub_slot;
        let mut var_qdsub_dn6: f64 = *var_qdsub_dn6_slot;
        let mut var_qdsub_dn7: f64 = *var_qdsub_dn7_slot;
        let mut var_qdsub_dn8: f64 = *var_qdsub_dn8_slot;
        let mut var_qg: f64 = *var_qg_slot;
        let mut var_qg_dn4: f64 = *var_qg_dn4_slot;
        let mut var_qg_dn6: f64 = *var_qg_dn6_slot;
        let mut var_qg_dn7: f64 = *var_qg_dn7_slot;
        let mut var_qg_dn8: f64 = *var_qg_dn8_slot;
        let mut var_qg_dn9: f64 = *var_qg_dn9_slot;
        let mut var_qgbe: f64 = *var_qgbe_slot;
        let mut var_qgbe_dn4: f64 = *var_qgbe_dn4_slot;
        let mut var_qgbe_dn6: f64 = *var_qgbe_dn6_slot;
        let mut var_qgbe_dn7: f64 = *var_qgbe_dn7_slot;
        let mut var_qgbe_dn8: f64 = *var_qgbe_dn8_slot;
        let mut var_qgbe_dn9: f64 = *var_qgbe_dn9_slot;
        let mut var_qgde: f64 = *var_qgde_slot;
        let mut var_qgde_dn4: f64 = *var_qgde_dn4_slot;
        let mut var_qgde_dn6: f64 = *var_qgde_dn6_slot;
        let mut var_qgde_dn7: f64 = *var_qgde_dn7_slot;
        let mut var_qgde_dn8: f64 = *var_qgde_dn8_slot;
        let mut var_qgde_dn9: f64 = *var_qgde_dn9_slot;
        let mut var_qgdif: f64 = *var_qgdif_slot;
        let mut var_qgdif_dn4: f64 = *var_qgdif_dn4_slot;
        let mut var_qgdif_dn6: f64 = *var_qgdif_dn6_slot;
        let mut var_qgdif_dn7: f64 = *var_qgdif_dn7_slot;
        let mut var_qgdif_dn8: f64 = *var_qgdif_dn8_slot;
        let mut var_qgdif_dn9: f64 = *var_qgdif_dn9_slot;
        let mut var_qgse: f64 = *var_qgse_slot;
        let mut var_qgse_dn4: f64 = *var_qgse_dn4_slot;
        let mut var_qgse_dn6: f64 = *var_qgse_dn6_slot;
        let mut var_qgse_dn7: f64 = *var_qgse_dn7_slot;
        let mut var_qgse_dn8: f64 = *var_qgse_dn8_slot;
        let mut var_qgse_dn9: f64 = *var_qgse_dn9_slot;
        let mut var_qgsif: f64 = *var_qgsif_slot;
        let mut var_qgsif_dn4: f64 = *var_qgsif_dn4_slot;
        let mut var_qgsif_dn6: f64 = *var_qgsif_dn6_slot;
        let mut var_qgsif_dn7: f64 = *var_qgsif_dn7_slot;
        let mut var_qgsif_dn8: f64 = *var_qgsif_dn8_slot;
        let mut var_qgsif_dn9: f64 = *var_qgsif_dn9_slot;
        let mut var_qovd: f64 = *var_qovd_slot;
        let mut var_qovd_dn4: f64 = *var_qovd_dn4_slot;
        let mut var_qovd_dn6: f64 = *var_qovd_dn6_slot;
        let mut var_qovd_dn7: f64 = *var_qovd_dn7_slot;
        let mut var_qovd_dn8: f64 = *var_qovd_dn8_slot;
        let mut var_qovd_dn9: f64 = *var_qovd_dn9_slot;
        let mut var_qovs: f64 = *var_qovs_slot;
        let mut var_qovs_dn4: f64 = *var_qovs_dn4_slot;
        let mut var_qovs_dn6: f64 = *var_qovs_dn6_slot;
        let mut var_qovs_dn7: f64 = *var_qovs_dn7_slot;
        let mut var_qovs_dn8: f64 = *var_qovs_dn8_slot;
        let mut var_qovs_dn9: f64 = *var_qovs_dn9_slot;
        let mut var_qs: f64 = *var_qs_slot;
        let mut var_qs_dn4: f64 = *var_qs_dn4_slot;
        let mut var_qs_dn6: f64 = *var_qs_dn6_slot;
        let mut var_qs_dn7: f64 = *var_qs_dn7_slot;
        let mut var_qs_dn8: f64 = *var_qs_dn8_slot;
        let mut var_qs_dn9: f64 = *var_qs_dn9_slot;
        let mut var_qssub: f64 = *var_qssub_slot;
        let mut var_qssub_dn6: f64 = *var_qssub_dn6_slot;
        let mut var_qssub_dn8: f64 = *var_qssub_dn8_slot;
        let mut var_qth: f64 = *var_qth_slot;
        let mut var_qth_dn4: f64 = *var_qth_dn4_slot;
        let mut var_qth_dn6: f64 = *var_qth_dn6_slot;
        let mut var_qth_dn7: f64 = *var_qth_dn7_slot;
        let mut var_qth_dn8: f64 = *var_qth_dn8_slot;
        let mut var_qth_dn9: f64 = *var_qth_dn9_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_temp_q: f64 = *var_temp_q_slot;
        let mut var_temp_q_dn4: f64 = *var_temp_q_dn4_slot;
        let mut var_temp_q_dn6: f64 = *var_temp_q_dn6_slot;
        let mut var_temp_q_dn7: f64 = *var_temp_q_dn7_slot;
        let mut var_temp_q_dn8: f64 = *var_temp_q_dn8_slot;
        let mut var_temp_q_dn9: f64 = *var_temp_q_dn9_slot;

        let (assign42640_e48154, assign42640_e48154_d_n4, assign42640_e48154_d_n6, assign42640_e48154_d_n7, assign42640_e48154_d_n8, assign42640_e48154_d_n9,) = {
    if (var_guard1235 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qgdif, var_qgdif_dn4, var_qgdif_dn6, var_qgdif_dn7, var_qgdif_dn8, var_qgdif_dn9,)
    }
};
        var_qgdif = assign42640_e48154;
        var_qgdif_dn4 = assign42640_e48154_d_n4;
        var_qgdif_dn6 = assign42640_e48154_d_n6;
        var_qgdif_dn7 = assign42640_e48154_d_n7;
        var_qgdif_dn8 = assign42640_e48154_d_n8;
        var_qgdif_dn9 = assign42640_e48154_d_n9;

        let (assign42650_e48159, assign42650_e48159_d_n4, assign42650_e48159_d_n6, assign42650_e48159_d_n7, assign42650_e48159_d_n8, assign42650_e48159_d_n9,) = {
    if (var_guard1235 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qbsif, var_qbsif_dn4, var_qbsif_dn6, var_qbsif_dn7, var_qbsif_dn8, var_qbsif_dn9,)
    }
};
        var_qbsif = assign42650_e48159;
        var_qbsif_dn4 = assign42650_e48159_d_n4;
        var_qbsif_dn6 = assign42650_e48159_d_n6;
        var_qbsif_dn7 = assign42650_e48159_d_n7;
        var_qbsif_dn8 = assign42650_e48159_d_n8;
        var_qbsif_dn9 = assign42650_e48159_d_n9;

        let (assign42660_e48164, assign42660_e48164_d_n4, assign42660_e48164_d_n6, assign42660_e48164_d_n7, assign42660_e48164_d_n8, assign42660_e48164_d_n9,) = {
    if (var_guard1235 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qbdif, var_qbdif_dn4, var_qbdif_dn6, var_qbdif_dn7, var_qbdif_dn8, var_qbdif_dn9,)
    }
};
        var_qbdif = assign42660_e48164;
        var_qbdif_dn4 = assign42660_e48164_d_n4;
        var_qbdif_dn6 = assign42660_e48164_d_n6;
        var_qbdif_dn7 = assign42660_e48164_d_n7;
        var_qbdif_dn8 = assign42660_e48164_d_n8;
        var_qbdif_dn9 = assign42660_e48164_d_n9;

        let assign42670_e48167: f64 = (var_cfr_i * var_vgsu);
        var_qgse = assign42670_e48167;
        var_qgse_dn4 = (var_cfr_i_dn4 * var_vgsu);
        var_qgse_dn6 = ((var_cfr_i_dn6 * var_vgsu) + (var_cfr_i * var_vgsu_dn6));
        var_qgse_dn7 = (var_cfr_i_dn7 * var_vgsu);
        var_qgse_dn8 = (var_cfr_i_dn8 * var_vgsu);
        var_qgse_dn9 = ((var_cfr_i_dn9 * var_vgsu) + (var_cfr_i * var_vgsu_dn9));

        let assign42680_e48170: f64 = (var_cfrd_i * var_vgdu);
        var_qgde = assign42680_e48170;
        var_qgde_dn4 = (var_cfrd_i_dn4 * var_vgdu);
        var_qgde_dn6 = ((var_cfrd_i_dn6 * var_vgdu) + (var_cfrd_i * var_vgdu_dn6));
        var_qgde_dn7 = ((var_cfrd_i_dn7 * var_vgdu) + (var_cfrd_i * var_vgdu_dn7));
        var_qgde_dn8 = (var_cfrd_i_dn8 * var_vgdu);
        var_qgde_dn9 = ((var_cfrd_i_dn9 * var_vgdu) + (var_cfrd_i * var_vgdu_dn9));

        let assign42690_e48175: f64 = (var_covdl_i * var_dleff_ac);
        let assign42690_e48179: f64 = (var_covdlb_i * var_xg20shift_ac);
        let assign42690_e48180: f64 = (1.0 - assign42690_e48179);
        let assign42690_e48181: f64 = (assign42690_e48175 * assign42690_e48180);
        let assign42690_e48182: f64 = (1.0 - assign42690_e48181);
        let assign42690_e48184: f64 = assign42690_e48182;
        let assign42690_e48188: f64 = (var_covdl_i * var_dleff_ac);
        let assign42690_e48192: f64 = (var_covdlb_i * var_xg20shift_ac);
        let assign42690_e48193: f64 = (1.0 - assign42690_e48192);
        let assign42690_e48194: f64 = (assign42690_e48188 * assign42690_e48193);
        let assign42690_e48195: f64 = (1.0 - assign42690_e48194);
        let assign42690_e48197: f64 = assign42690_e48195;
        let assign42690_e48201: f64 = (var_covdl_i * var_dleff_ac);
        let assign42690_e48205: f64 = (var_covdlb_i * var_xg20shift_ac);
        let assign42690_e48206: f64 = (1.0 - assign42690_e48205);
        let assign42690_e48207: f64 = (assign42690_e48201 * assign42690_e48206);
        let assign42690_e48208: f64 = (1.0 - assign42690_e48207);
        let assign42690_e48210: f64 = assign42690_e48208;
        let assign42690_e48211: f64 = (assign42690_e48197 * assign42690_e48210);
        let assign42690_e48213: f64 = (assign42690_e48211 + 0.2);
        let assign42690_e48214: f64 = (assign42690_e48213).sqrt();
        let assign42690_e48215: f64 = (assign42690_e48184 + assign42690_e48214);
        let assign42690_e48216: f64 = (0.5 * assign42690_e48215);
        var_temp = assign42690_e48216;
        var_temp_dn4 = (0.5 * ((-(((var_covdl_i * var_dleff_ac_dn4) * assign42690_e48180) + (assign42690_e48175 * (-(var_covdlb_i * var_xg20shift_ac_dn4))))) + ((((-(((var_covdl_i * var_dleff_ac_dn4) * assign42690_e48193) + (assign42690_e48188 * (-(var_covdlb_i * var_xg20shift_ac_dn4))))) * assign42690_e48210) + (assign42690_e48197 * (-(((var_covdl_i * var_dleff_ac_dn4) * assign42690_e48206) + (assign42690_e48201 * (-(var_covdlb_i * var_xg20shift_ac_dn4))))))) / (2.0 * assign42690_e48214))));
        var_temp_dn6 = (0.5 * ((-(((var_covdl_i * var_dleff_ac_dn6) * assign42690_e48180) + (assign42690_e48175 * (-(var_covdlb_i * var_xg20shift_ac_dn6))))) + ((((-(((var_covdl_i * var_dleff_ac_dn6) * assign42690_e48193) + (assign42690_e48188 * (-(var_covdlb_i * var_xg20shift_ac_dn6))))) * assign42690_e48210) + (assign42690_e48197 * (-(((var_covdl_i * var_dleff_ac_dn6) * assign42690_e48206) + (assign42690_e48201 * (-(var_covdlb_i * var_xg20shift_ac_dn6))))))) / (2.0 * assign42690_e48214))));
        var_temp_dn7 = (0.5 * ((-(((var_covdl_i * var_dleff_ac_dn7) * assign42690_e48180) + (assign42690_e48175 * (-(var_covdlb_i * var_xg20shift_ac_dn7))))) + ((((-(((var_covdl_i * var_dleff_ac_dn7) * assign42690_e48193) + (assign42690_e48188 * (-(var_covdlb_i * var_xg20shift_ac_dn7))))) * assign42690_e48210) + (assign42690_e48197 * (-(((var_covdl_i * var_dleff_ac_dn7) * assign42690_e48206) + (assign42690_e48201 * (-(var_covdlb_i * var_xg20shift_ac_dn7))))))) / (2.0 * assign42690_e48214))));
        var_temp_dn8 = (0.5 * ((-(((var_covdl_i * var_dleff_ac_dn8) * assign42690_e48180) + (assign42690_e48175 * (-(var_covdlb_i * var_xg20shift_ac_dn8))))) + ((((-(((var_covdl_i * var_dleff_ac_dn8) * assign42690_e48193) + (assign42690_e48188 * (-(var_covdlb_i * var_xg20shift_ac_dn8))))) * assign42690_e48210) + (assign42690_e48197 * (-(((var_covdl_i * var_dleff_ac_dn8) * assign42690_e48206) + (assign42690_e48201 * (-(var_covdlb_i * var_xg20shift_ac_dn8))))))) / (2.0 * assign42690_e48214))));
        var_temp_dn9 = (0.5 * ((-(((var_covdl_i * var_dleff_ac_dn9) * assign42690_e48180) + (assign42690_e48175 * (-(var_covdlb_i * var_xg20shift_ac_dn9))))) + ((((-(((var_covdl_i * var_dleff_ac_dn9) * assign42690_e48193) + (assign42690_e48188 * (-(var_covdlb_i * var_xg20shift_ac_dn9))))) * assign42690_e48210) + (assign42690_e48197 * (-(((var_covdl_i * var_dleff_ac_dn9) * assign42690_e48206) + (assign42690_e48201 * (-(var_covdlb_i * var_xg20shift_ac_dn9))))))) / (2.0 * assign42690_e48214))));

        let assign42700_e48219: f64 = (var_cov_i * var_vovscv);
        let assign42700_e48221: f64 = (assign42700_e48219 * var_temp);
        var_qovs = assign42700_e48221;
        var_qovs_dn4 = ((((var_cov_i_dn4 * var_vovscv) + (var_cov_i * var_vovscv_dn4)) * var_temp) + (assign42700_e48219 * var_temp_dn4));
        var_qovs_dn6 = ((((var_cov_i_dn6 * var_vovscv) + (var_cov_i * var_vovscv_dn6)) * var_temp) + (assign42700_e48219 * var_temp_dn6));
        var_qovs_dn7 = ((((var_cov_i_dn7 * var_vovscv) + (var_cov_i * var_vovscv_dn7)) * var_temp) + (assign42700_e48219 * var_temp_dn7));
        var_qovs_dn8 = ((((var_cov_i_dn8 * var_vovscv) + (var_cov_i * var_vovscv_dn8)) * var_temp) + (assign42700_e48219 * var_temp_dn8));
        var_qovs_dn9 = ((((var_cov_i_dn9 * var_vovscv) + (var_cov_i * var_vovscv_dn9)) * var_temp) + (assign42700_e48219 * var_temp_dn9));

        let assign42710_e48224: f64 = (var_covd_i * var_vovdcv);
        let assign42710_e48226: f64 = (assign42710_e48224 * var_temp);
        var_qovd = assign42710_e48226;
        var_qovd_dn4 = ((((var_covd_i_dn4 * var_vovdcv) + (var_covd_i * var_vovdcv_dn4)) * var_temp) + (assign42710_e48224 * var_temp_dn4));
        var_qovd_dn6 = ((((var_covd_i_dn6 * var_vovdcv) + (var_covd_i * var_vovdcv_dn6)) * var_temp) + (assign42710_e48224 * var_temp_dn6));
        var_qovd_dn7 = ((((var_covd_i_dn7 * var_vovdcv) + (var_covd_i * var_vovdcv_dn7)) * var_temp) + (assign42710_e48224 * var_temp_dn7));
        var_qovd_dn8 = ((((var_covd_i_dn8 * var_vovdcv) + (var_covd_i * var_vovdcv_dn8)) * var_temp) + (assign42710_e48224 * var_temp_dn8));
        var_qovd_dn9 = ((((var_covd_i_dn9 * var_vovdcv) + (var_covd_i * var_vovdcv_dn9)) * var_temp) + (assign42710_e48224 * var_temp_dn9));

        let assign42720_e48229: f64 = (var_cgbov_i * var_vgb);
        var_qgbe = assign42720_e48229;
        var_qgbe_dn4 = (var_cgbov_i_dn4 * var_vgb);
        var_qgbe_dn6 = ((var_cgbov_i_dn6 * var_vgb) + (var_cgbov_i * var_vgb_dn6));
        var_qgbe_dn7 = ((var_cgbov_i_dn7 * var_vgb) + (var_cgbov_i * var_vgb_dn7));
        var_qgbe_dn8 = ((var_cgbov_i_dn8 * var_vgb) + (var_cgbov_i * var_vgb_dn8));
        var_qgbe_dn9 = ((var_cgbov_i_dn9 * var_vgb) + (var_cgbov_i * var_vgb_dn9));

        let assign42730_e48232: f64 = (var_csd_i * var_vds);
        var_qdse = assign42730_e48232;
        var_qdse_dn6 = (var_csd_i * var_vds_dn6);
        var_qdse_dn7 = (var_csd_i * var_vds_dn7);

        let assign42740_e48235: f64 = (var_cox2init * var_asource_i);
        let assign42740_e48238: f64 = (var_csdbp_i * var_psource_i);
        let assign42740_e48239: f64 = (assign42740_e48235 + assign42740_e48238);
        let assign42740_e48240: f64 = (-assign42740_e48239);
        let assign42740_e48242: f64 = (assign42740_e48240 * var_vsbu);
        var_qssub = assign42740_e48242;
        var_qssub_dn6 = (assign42740_e48240 * var_vsbu_dn6);
        var_qssub_dn8 = (assign42740_e48240 * var_vsbu_dn8);

        let assign42750_e48245: f64 = (var_cox2init * var_adrain_i);
        let assign42750_e48248: f64 = (var_csdbp_i * var_pdrain_i);
        let assign42750_e48249: f64 = (assign42750_e48245 + assign42750_e48248);
        let assign42750_e48250: f64 = (-assign42750_e48249);
        let assign42750_e48252: f64 = (assign42750_e48250 * var_vdbu);
        var_qdsub = assign42750_e48252;
        var_qdsub_dn6 = (assign42750_e48250 * var_vdbu_dn6);
        var_qdsub_dn7 = (assign42750_e48250 * var_vdbu_dn7);
        var_qdsub_dn8 = (assign42750_e48250 * var_vdbu_dn8);

        let assign42760_e48255: f64 = if var_swshe_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1236 = assign42760_e48255;

        let (assign42770_e48261, assign42770_e48261_d_n4, assign42770_e48261_d_n6, assign42770_e48261_d_n7, assign42770_e48261_d_n8, assign42770_e48261_d_n9,) = {
    if (var_guard1236 != 0.0) {
        let assign42770_e48259: f64 = (var_cth_i * var_dtc);
        (assign42770_e48259, ((var_cth_i_dn4 * var_dtc) + (var_cth_i * var_dtc_dn4)), (var_cth_i_dn6 * var_dtc), (var_cth_i_dn7 * var_dtc), (var_cth_i_dn8 * var_dtc), (var_cth_i_dn9 * var_dtc),)
    } else {
        (var_qth, var_qth_dn4, var_qth_dn6, var_qth_dn7, var_qth_dn8, var_qth_dn9,)
    }
};
        var_qth = assign42770_e48261;
        var_qth_dn4 = assign42770_e48261_d_n4;
        var_qth_dn6 = assign42770_e48261_d_n6;
        var_qth_dn7 = assign42770_e48261_d_n7;
        var_qth_dn8 = assign42770_e48261_d_n8;
        var_qth_dn9 = assign42770_e48261_d_n9;

        let (assign42780_e48266, assign42780_e48266_d_n4, assign42780_e48266_d_n6, assign42780_e48266_d_n7, assign42780_e48266_d_n8, assign42780_e48266_d_n9,) = {
    if (var_guard1236 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qth, var_qth_dn4, var_qth_dn6, var_qth_dn7, var_qth_dn8, var_qth_dn9,)
    }
};
        var_qth = assign42780_e48266;
        var_qth_dn4 = assign42780_e48266_d_n4;
        var_qth_dn6 = assign42780_e48266_d_n6;
        var_qth_dn7 = assign42780_e48266_d_n7;
        var_qth_dn8 = assign42780_e48266_d_n8;
        var_qth_dn9 = assign42780_e48266_d_n9;

        let assign42790_e48269: f64 = (p.p31 * var_mult_i_int);
        let assign42790_e48272: f64 = (var_ids + var_ids_edge);
        let assign42790_e48274: f64 = (assign42790_e48272 + var_iimpact);
        let assign42790_e48275: f64 = (assign42790_e48269 * assign42790_e48274);
        var_idse = assign42790_e48275;
        var_idse_dn4 = (assign42790_e48269 * ((var_ids_dn4 + var_ids_edge_dn4) + var_iimpact_dn4));
        var_idse_dn6 = (assign42790_e48269 * ((var_ids_dn6 + var_ids_edge_dn6) + var_iimpact_dn6));
        var_idse_dn7 = (assign42790_e48269 * ((var_ids_dn7 + var_ids_edge_dn7) + var_iimpact_dn7));
        var_idse_dn8 = (assign42790_e48269 * ((var_ids_dn8 + var_ids_edge_dn8) + var_iimpact_dn8));
        var_idse_dn9 = (assign42790_e48269 * ((var_ids_dn9 + var_ids_edge_dn9) + var_iimpact_dn9));

        let assign42800_e48278: f64 = (p.p31 * var_mult_i_int);
        let assign42800_e48280: f64 = (assign42800_e48278 * var_igs);
        var_igse = assign42800_e48280;
        var_igse_dn4 = (assign42800_e48278 * var_igs_dn4);
        var_igse_dn6 = (assign42800_e48278 * var_igs_dn6);
        var_igse_dn7 = (assign42800_e48278 * var_igs_dn7);
        var_igse_dn8 = (assign42800_e48278 * var_igs_dn8);
        var_igse_dn9 = (assign42800_e48278 * var_igs_dn9);

        let assign42810_e48283: f64 = (p.p31 * var_mult_i_int);
        let assign42810_e48285: f64 = (assign42810_e48283 * var_igd);
        var_igde = assign42810_e48285;
        var_igde_dn4 = (assign42810_e48283 * var_igd_dn4);
        var_igde_dn6 = (assign42810_e48283 * var_igd_dn6);
        var_igde_dn7 = (assign42810_e48283 * var_igd_dn7);
        var_igde_dn8 = (assign42810_e48283 * var_igd_dn8);
        var_igde_dn9 = (assign42810_e48283 * var_igd_dn9);

        let assign42820_e48288: f64 = (p.p31 * var_mult_i_int);
        let assign42820_e48290: f64 = (assign42820_e48288 * var_igidl);
        var_igidle = assign42820_e48290;
        var_igidle_dn4 = (assign42820_e48288 * var_igidl_dn4);
        var_igidle_dn6 = (assign42820_e48288 * var_igidl_dn6);
        var_igidle_dn7 = (assign42820_e48288 * var_igidl_dn7);
        var_igidle_dn8 = (assign42820_e48288 * var_igidl_dn8);
        var_igidle_dn9 = (assign42820_e48288 * var_igidl_dn9);

        let assign42830_e48293: f64 = (p.p31 * var_mult_i_int);
        let assign42830_e48295: f64 = (assign42830_e48293 * var_igisl);
        var_igisle = assign42830_e48295;
        var_igisle_dn4 = (assign42830_e48293 * var_igisl_dn4);
        var_igisle_dn6 = (assign42830_e48293 * var_igisl_dn6);
        var_igisle_dn7 = (assign42830_e48293 * var_igisl_dn7);
        var_igisle_dn8 = (assign42830_e48293 * var_igisl_dn8);
        var_igisle_dn9 = (assign42830_e48293 * var_igisl_dn9);

        let assign42840_e48298: f64 = (var_mult_i_int * var_ithpwr);
        var_ithpwre = assign42840_e48298;
        var_ithpwre_dn4 = (var_mult_i_int * var_ithpwr_dn4);
        var_ithpwre_dn6 = (var_mult_i_int * var_ithpwr_dn6);
        var_ithpwre_dn7 = (var_mult_i_int * var_ithpwr_dn7);
        var_ithpwre_dn8 = (var_mult_i_int * var_ithpwr_dn8);
        var_ithpwre_dn9 = (var_mult_i_int * var_ithpwr_dn9);

        let assign42850_e48301: f64 = (var_mult_i_int * var_ithrc);
        var_ithrce = assign42850_e48301;
        var_ithrce_dn4 = (var_mult_i_int * var_ithrc_dn4);
        var_ithrce_dn6 = (var_mult_i_int * var_ithrc_dn6);
        var_ithrce_dn7 = (var_mult_i_int * var_ithrc_dn7);
        var_ithrce_dn8 = (var_mult_i_int * var_ithrc_dn8);
        var_ithrce_dn9 = (var_mult_i_int * var_ithrc_dn9);

        let assign42860_e48304: f64 = if var_sigvds < 0.0 { 1.0 } else { 0.0 };
        var_guard1239 = assign42860_e48304;

        let assign42910_e48319: f64 = (p.p32 * var_mult_i_int);
        let assign42910_e48321: f64 = (assign42910_e48319 * var_qg);
        var_qg = assign42910_e48321;
        var_qg_dn4 = (assign42910_e48319 * var_qg_dn4);
        var_qg_dn6 = (assign42910_e48319 * var_qg_dn6);
        var_qg_dn7 = (assign42910_e48319 * var_qg_dn7);
        var_qg_dn8 = (assign42910_e48319 * var_qg_dn8);
        var_qg_dn9 = (assign42910_e48319 * var_qg_dn9);

        let assign42920_e48324: f64 = (p.p32 * var_mult_i_int);
        let assign42920_e48326: f64 = (assign42920_e48324 * var_qb);
        var_qb = assign42920_e48326;
        var_qb_dn4 = (assign42920_e48324 * var_qb_dn4);
        var_qb_dn6 = (assign42920_e48324 * var_qb_dn6);
        var_qb_dn7 = (assign42920_e48324 * var_qb_dn7);
        var_qb_dn8 = (assign42920_e48324 * var_qb_dn8);
        var_qb_dn9 = (assign42920_e48324 * var_qb_dn9);

        let assign42930_e48329: f64 = (p.p32 * var_mult_i_int);
        let assign42930_e48331: f64 = (assign42930_e48329 * var_qd);
        var_qd = assign42930_e48331;
        var_qd_dn4 = (assign42930_e48329 * var_qd_dn4);
        var_qd_dn6 = (assign42930_e48329 * var_qd_dn6);
        var_qd_dn7 = (assign42930_e48329 * var_qd_dn7);
        var_qd_dn8 = (assign42930_e48329 * var_qd_dn8);
        var_qd_dn9 = (assign42930_e48329 * var_qd_dn9);

        let assign42940_e48334: f64 = (var_qg + var_qb);
        let assign42940_e48336: f64 = (assign42940_e48334 + var_qd);
        let assign42940_e48337: f64 = (-assign42940_e48336);
        var_qs = assign42940_e48337;
        var_qs_dn4 = (-((var_qg_dn4 + var_qb_dn4) + var_qd_dn4));
        var_qs_dn6 = (-((var_qg_dn6 + var_qb_dn6) + var_qd_dn6));
        var_qs_dn7 = (-((var_qg_dn7 + var_qb_dn7) + var_qd_dn7));
        var_qs_dn8 = (-((var_qg_dn8 + var_qb_dn8) + var_qd_dn8));
        var_qs_dn9 = (-((var_qg_dn9 + var_qb_dn9) + var_qd_dn9));

        let assign42950_e48340: f64 = (p.p32 * var_mult_i_int);
        let assign42950_e48342: f64 = (assign42950_e48340 * var_qgsif);
        var_qgsif = assign42950_e48342;
        var_qgsif_dn4 = (assign42950_e48340 * var_qgsif_dn4);
        var_qgsif_dn6 = (assign42950_e48340 * var_qgsif_dn6);
        var_qgsif_dn7 = (assign42950_e48340 * var_qgsif_dn7);
        var_qgsif_dn8 = (assign42950_e48340 * var_qgsif_dn8);
        var_qgsif_dn9 = (assign42950_e48340 * var_qgsif_dn9);

        let assign42960_e48345: f64 = (p.p32 * var_mult_i_int);
        let assign42960_e48347: f64 = (assign42960_e48345 * var_qgdif);
        var_qgdif = assign42960_e48347;
        var_qgdif_dn4 = (assign42960_e48345 * var_qgdif_dn4);
        var_qgdif_dn6 = (assign42960_e48345 * var_qgdif_dn6);
        var_qgdif_dn7 = (assign42960_e48345 * var_qgdif_dn7);
        var_qgdif_dn8 = (assign42960_e48345 * var_qgdif_dn8);
        var_qgdif_dn9 = (assign42960_e48345 * var_qgdif_dn9);

        let assign42970_e48350: f64 = (p.p32 * var_mult_i_int);
        let assign42970_e48352: f64 = (assign42970_e48350 * var_qbsif);
        var_qbsif = assign42970_e48352;
        var_qbsif_dn4 = (assign42970_e48350 * var_qbsif_dn4);
        var_qbsif_dn6 = (assign42970_e48350 * var_qbsif_dn6);
        var_qbsif_dn7 = (assign42970_e48350 * var_qbsif_dn7);
        var_qbsif_dn8 = (assign42970_e48350 * var_qbsif_dn8);
        var_qbsif_dn9 = (assign42970_e48350 * var_qbsif_dn9);

        let assign42980_e48355: f64 = (p.p32 * var_mult_i_int);
        let assign42980_e48357: f64 = (assign42980_e48355 * var_qbdif);
        var_qbdif = assign42980_e48357;
        var_qbdif_dn4 = (assign42980_e48355 * var_qbdif_dn4);
        var_qbdif_dn6 = (assign42980_e48355 * var_qbdif_dn6);
        var_qbdif_dn7 = (assign42980_e48355 * var_qbdif_dn7);
        var_qbdif_dn8 = (assign42980_e48355 * var_qbdif_dn8);
        var_qbdif_dn9 = (assign42980_e48355 * var_qbdif_dn9);

        let assign42990_e48360: f64 = (p.p32 * var_mult_i_int);
        let assign42990_e48362: f64 = (assign42990_e48360 * var_qgse);
        var_qgse = assign42990_e48362;
        var_qgse_dn4 = (assign42990_e48360 * var_qgse_dn4);
        var_qgse_dn6 = (assign42990_e48360 * var_qgse_dn6);
        var_qgse_dn7 = (assign42990_e48360 * var_qgse_dn7);
        var_qgse_dn8 = (assign42990_e48360 * var_qgse_dn8);
        var_qgse_dn9 = (assign42990_e48360 * var_qgse_dn9);

        let assign43000_e48365: f64 = (p.p32 * var_mult_i_int);
        let assign43000_e48367: f64 = (assign43000_e48365 * var_qgde);
        var_qgde = assign43000_e48367;
        var_qgde_dn4 = (assign43000_e48365 * var_qgde_dn4);
        var_qgde_dn6 = (assign43000_e48365 * var_qgde_dn6);
        var_qgde_dn7 = (assign43000_e48365 * var_qgde_dn7);
        var_qgde_dn8 = (assign43000_e48365 * var_qgde_dn8);
        var_qgde_dn9 = (assign43000_e48365 * var_qgde_dn9);

        let assign43010_e48370: f64 = (p.p32 * var_mult_i_int);
        let assign43010_e48372: f64 = (assign43010_e48370 * var_qovs);
        var_qovs = assign43010_e48372;
        var_qovs_dn4 = (assign43010_e48370 * var_qovs_dn4);
        var_qovs_dn6 = (assign43010_e48370 * var_qovs_dn6);
        var_qovs_dn7 = (assign43010_e48370 * var_qovs_dn7);
        var_qovs_dn8 = (assign43010_e48370 * var_qovs_dn8);
        var_qovs_dn9 = (assign43010_e48370 * var_qovs_dn9);

        let assign43020_e48375: f64 = (p.p32 * var_mult_i_int);
        let assign43020_e48377: f64 = (assign43020_e48375 * var_qovd);
        var_qovd = assign43020_e48377;
        var_qovd_dn4 = (assign43020_e48375 * var_qovd_dn4);
        var_qovd_dn6 = (assign43020_e48375 * var_qovd_dn6);
        var_qovd_dn7 = (assign43020_e48375 * var_qovd_dn7);
        var_qovd_dn8 = (assign43020_e48375 * var_qovd_dn8);
        var_qovd_dn9 = (assign43020_e48375 * var_qovd_dn9);

        let assign43030_e48380: f64 = (p.p32 * var_mult_i_int);
        let assign43030_e48382: f64 = (assign43030_e48380 * var_qgbe);
        var_qgbe = assign43030_e48382;
        var_qgbe_dn4 = (assign43030_e48380 * var_qgbe_dn4);
        var_qgbe_dn6 = (assign43030_e48380 * var_qgbe_dn6);
        var_qgbe_dn7 = (assign43030_e48380 * var_qgbe_dn7);
        var_qgbe_dn8 = (assign43030_e48380 * var_qgbe_dn8);
        var_qgbe_dn9 = (assign43030_e48380 * var_qgbe_dn9);

        let assign43040_e48385: f64 = (p.p32 * var_mult_i_int);
        let assign43040_e48387: f64 = (assign43040_e48385 * var_qssub);
        var_qssub = assign43040_e48387;
        var_qssub_dn6 = (assign43040_e48385 * var_qssub_dn6);
        var_qssub_dn8 = (assign43040_e48385 * var_qssub_dn8);

        let assign43050_e48390: f64 = (p.p32 * var_mult_i_int);
        let assign43050_e48392: f64 = (assign43050_e48390 * var_qdsub);
        var_qdsub = assign43050_e48392;
        var_qdsub_dn6 = (assign43050_e48390 * var_qdsub_dn6);
        var_qdsub_dn7 = (assign43050_e48390 * var_qdsub_dn7);
        var_qdsub_dn8 = (assign43050_e48390 * var_qdsub_dn8);

        let assign43060_e48395: f64 = (p.p32 * var_mult_i_int);
        let assign43060_e48397: f64 = (assign43060_e48395 * var_qdse);
        var_qdse = assign43060_e48397;
        var_qdse_dn6 = (assign43060_e48395 * var_qdse_dn6);
        var_qdse_dn7 = (assign43060_e48395 * var_qdse_dn7);

        let assign43070_e48400: f64 = (var_mult_i_int * var_qth);
        var_qth = assign43070_e48400;
        var_qth_dn4 = (var_mult_i_int * var_qth_dn4);
        var_qth_dn6 = (var_mult_i_int * var_qth_dn6);
        var_qth_dn7 = (var_mult_i_int * var_qth_dn7);
        var_qth_dn8 = (var_mult_i_int * var_qth_dn8);
        var_qth_dn9 = (var_mult_i_int * var_qth_dn9);

        let assign43080_e48403: f64 = if var_sigvds < 0.0 { 1.0 } else { 0.0 };
        var_guard1245 = assign43080_e48403;

        let (assign43090_e48407, assign43090_e48407_d_n4, assign43090_e48407_d_n6, assign43090_e48407_d_n7, assign43090_e48407_d_n8, assign43090_e48407_d_n9,) = {
    if (var_guard1245 != 0.0) {
        (var_qd, var_qd_dn4, var_qd_dn6, var_qd_dn7, var_qd_dn8, var_qd_dn9,)
    } else {
        (var_temp_q, var_temp_q_dn4, var_temp_q_dn6, var_temp_q_dn7, var_temp_q_dn8, var_temp_q_dn9,)
    }
};
        var_temp_q = assign43090_e48407;
        var_temp_q_dn4 = assign43090_e48407_d_n4;
        var_temp_q_dn6 = assign43090_e48407_d_n6;
        var_temp_q_dn7 = assign43090_e48407_d_n7;
        var_temp_q_dn8 = assign43090_e48407_d_n8;
        var_temp_q_dn9 = assign43090_e48407_d_n9;

        let (assign43100_e48411, assign43100_e48411_d_n4, assign43100_e48411_d_n6, assign43100_e48411_d_n7, assign43100_e48411_d_n8, assign43100_e48411_d_n9,) = {
    if (var_guard1245 != 0.0) {
        (var_qs, var_qs_dn4, var_qs_dn6, var_qs_dn7, var_qs_dn8, var_qs_dn9,)
    } else {
        (var_qd, var_qd_dn4, var_qd_dn6, var_qd_dn7, var_qd_dn8, var_qd_dn9,)
    }
};
        var_qd = assign43100_e48411;
        var_qd_dn4 = assign43100_e48411_d_n4;
        var_qd_dn6 = assign43100_e48411_d_n6;
        var_qd_dn7 = assign43100_e48411_d_n7;
        var_qd_dn8 = assign43100_e48411_d_n8;
        var_qd_dn9 = assign43100_e48411_d_n9;

        let (assign43110_e48415, assign43110_e48415_d_n4, assign43110_e48415_d_n6, assign43110_e48415_d_n7, assign43110_e48415_d_n8, assign43110_e48415_d_n9,) = {
    if (var_guard1245 != 0.0) {
        (var_temp_q, var_temp_q_dn4, var_temp_q_dn6, var_temp_q_dn7, var_temp_q_dn8, var_temp_q_dn9,)
    } else {
        (var_qs, var_qs_dn4, var_qs_dn6, var_qs_dn7, var_qs_dn8, var_qs_dn9,)
    }
};
        var_qs = assign43110_e48415;
        var_qs_dn4 = assign43110_e48415_d_n4;
        var_qs_dn6 = assign43110_e48415_d_n6;
        var_qs_dn7 = assign43110_e48415_d_n7;
        var_qs_dn8 = assign43110_e48415_d_n8;
        var_qs_dn9 = assign43110_e48415_d_n9;

        let (assign43120_e48420, assign43120_e48420_d_n6, assign43120_e48420_d_n7,) = {
    if (var_guard1245 != 0.0) {
        let assign43120_e48418: f64 = (-var_qdse);
        (assign43120_e48418, (-var_qdse_dn6), (-var_qdse_dn7),)
    } else {
        (var_qdse, var_qdse_dn6, var_qdse_dn7,)
    }
};
        var_qdse = assign43120_e48420;
        var_qdse_dn6 = assign43120_e48420_d_n6;
        var_qdse_dn7 = assign43120_e48420_d_n7;

        let (assign43130_e48424, assign43130_e48424_d_n4, assign43130_e48424_d_n6, assign43130_e48424_d_n7, assign43130_e48424_d_n8, assign43130_e48424_d_n9,) = {
    if (var_guard1245 != 0.0) {
        (var_qgdif, var_qgdif_dn4, var_qgdif_dn6, var_qgdif_dn7, var_qgdif_dn8, var_qgdif_dn9,)
    } else {
        (var_temp_q, var_temp_q_dn4, var_temp_q_dn6, var_temp_q_dn7, var_temp_q_dn8, var_temp_q_dn9,)
    }
};
        var_temp_q = assign43130_e48424;
        var_temp_q_dn4 = assign43130_e48424_d_n4;
        var_temp_q_dn6 = assign43130_e48424_d_n6;
        var_temp_q_dn7 = assign43130_e48424_d_n7;
        var_temp_q_dn8 = assign43130_e48424_d_n8;
        var_temp_q_dn9 = assign43130_e48424_d_n9;

        let (assign43140_e48428, assign43140_e48428_d_n4, assign43140_e48428_d_n6, assign43140_e48428_d_n7, assign43140_e48428_d_n8, assign43140_e48428_d_n9,) = {
    if (var_guard1245 != 0.0) {
        (var_qgsif, var_qgsif_dn4, var_qgsif_dn6, var_qgsif_dn7, var_qgsif_dn8, var_qgsif_dn9,)
    } else {
        (var_qgdif, var_qgdif_dn4, var_qgdif_dn6, var_qgdif_dn7, var_qgdif_dn8, var_qgdif_dn9,)
    }
};
        var_qgdif = assign43140_e48428;
        var_qgdif_dn4 = assign43140_e48428_d_n4;
        var_qgdif_dn6 = assign43140_e48428_d_n6;
        var_qgdif_dn7 = assign43140_e48428_d_n7;
        var_qgdif_dn8 = assign43140_e48428_d_n8;
        var_qgdif_dn9 = assign43140_e48428_d_n9;

        let (assign43150_e48432, assign43150_e48432_d_n4, assign43150_e48432_d_n6, assign43150_e48432_d_n7, assign43150_e48432_d_n8, assign43150_e48432_d_n9,) = {
    if (var_guard1245 != 0.0) {
        (var_temp_q, var_temp_q_dn4, var_temp_q_dn6, var_temp_q_dn7, var_temp_q_dn8, var_temp_q_dn9,)
    } else {
        (var_qgsif, var_qgsif_dn4, var_qgsif_dn6, var_qgsif_dn7, var_qgsif_dn8, var_qgsif_dn9,)
    }
};
        var_qgsif = assign43150_e48432;
        var_qgsif_dn4 = assign43150_e48432_d_n4;
        var_qgsif_dn6 = assign43150_e48432_d_n6;
        var_qgsif_dn7 = assign43150_e48432_d_n7;
        var_qgsif_dn8 = assign43150_e48432_d_n8;
        var_qgsif_dn9 = assign43150_e48432_d_n9;

        let (assign43160_e48436, assign43160_e48436_d_n4, assign43160_e48436_d_n6, assign43160_e48436_d_n7, assign43160_e48436_d_n8, assign43160_e48436_d_n9,) = {
    if (var_guard1245 != 0.0) {
        (var_qbdif, var_qbdif_dn4, var_qbdif_dn6, var_qbdif_dn7, var_qbdif_dn8, var_qbdif_dn9,)
    } else {
        (var_temp_q, var_temp_q_dn4, var_temp_q_dn6, var_temp_q_dn7, var_temp_q_dn8, var_temp_q_dn9,)
    }
};
        var_temp_q = assign43160_e48436;
        var_temp_q_dn4 = assign43160_e48436_d_n4;
        var_temp_q_dn6 = assign43160_e48436_d_n6;
        var_temp_q_dn7 = assign43160_e48436_d_n7;
        var_temp_q_dn8 = assign43160_e48436_d_n8;
        var_temp_q_dn9 = assign43160_e48436_d_n9;

        let (assign43170_e48440, assign43170_e48440_d_n4, assign43170_e48440_d_n6, assign43170_e48440_d_n7, assign43170_e48440_d_n8, assign43170_e48440_d_n9,) = {
    if (var_guard1245 != 0.0) {
        (var_qbsif, var_qbsif_dn4, var_qbsif_dn6, var_qbsif_dn7, var_qbsif_dn8, var_qbsif_dn9,)
    } else {
        (var_qbdif, var_qbdif_dn4, var_qbdif_dn6, var_qbdif_dn7, var_qbdif_dn8, var_qbdif_dn9,)
    }
};
        var_qbdif = assign43170_e48440;
        var_qbdif_dn4 = assign43170_e48440_d_n4;
        var_qbdif_dn6 = assign43170_e48440_d_n6;
        var_qbdif_dn7 = assign43170_e48440_d_n7;
        var_qbdif_dn8 = assign43170_e48440_d_n8;
        var_qbdif_dn9 = assign43170_e48440_d_n9;

        *var_guard1236_slot = var_guard1236;
        *var_guard1239_slot = var_guard1239;
        *var_guard1245_slot = var_guard1245;
        *var_idse_slot = var_idse;
        *var_idse_dn4_slot = var_idse_dn4;
        *var_idse_dn6_slot = var_idse_dn6;
        *var_idse_dn7_slot = var_idse_dn7;
        *var_idse_dn8_slot = var_idse_dn8;
        *var_idse_dn9_slot = var_idse_dn9;
        *var_igde_slot = var_igde;
        *var_igde_dn4_slot = var_igde_dn4;
        *var_igde_dn6_slot = var_igde_dn6;
        *var_igde_dn7_slot = var_igde_dn7;
        *var_igde_dn8_slot = var_igde_dn8;
        *var_igde_dn9_slot = var_igde_dn9;
        *var_igidle_slot = var_igidle;
        *var_igidle_dn4_slot = var_igidle_dn4;
        *var_igidle_dn6_slot = var_igidle_dn6;
        *var_igidle_dn7_slot = var_igidle_dn7;
        *var_igidle_dn8_slot = var_igidle_dn8;
        *var_igidle_dn9_slot = var_igidle_dn9;
        *var_igisle_slot = var_igisle;
        *var_igisle_dn4_slot = var_igisle_dn4;
        *var_igisle_dn6_slot = var_igisle_dn6;
        *var_igisle_dn7_slot = var_igisle_dn7;
        *var_igisle_dn8_slot = var_igisle_dn8;
        *var_igisle_dn9_slot = var_igisle_dn9;
        *var_igse_slot = var_igse;
        *var_igse_dn4_slot = var_igse_dn4;
        *var_igse_dn6_slot = var_igse_dn6;
        *var_igse_dn7_slot = var_igse_dn7;
        *var_igse_dn8_slot = var_igse_dn8;
        *var_igse_dn9_slot = var_igse_dn9;
        *var_ithpwre_slot = var_ithpwre;
        *var_ithpwre_dn4_slot = var_ithpwre_dn4;
        *var_ithpwre_dn6_slot = var_ithpwre_dn6;
        *var_ithpwre_dn7_slot = var_ithpwre_dn7;
        *var_ithpwre_dn8_slot = var_ithpwre_dn8;
        *var_ithpwre_dn9_slot = var_ithpwre_dn9;
        *var_ithrce_slot = var_ithrce;
        *var_ithrce_dn4_slot = var_ithrce_dn4;
        *var_ithrce_dn6_slot = var_ithrce_dn6;
        *var_ithrce_dn7_slot = var_ithrce_dn7;
        *var_ithrce_dn8_slot = var_ithrce_dn8;
        *var_ithrce_dn9_slot = var_ithrce_dn9;
        *var_qb_slot = var_qb;
        *var_qb_dn4_slot = var_qb_dn4;
        *var_qb_dn6_slot = var_qb_dn6;
        *var_qb_dn7_slot = var_qb_dn7;
        *var_qb_dn8_slot = var_qb_dn8;
        *var_qb_dn9_slot = var_qb_dn9;
        *var_qbdif_slot = var_qbdif;
        *var_qbdif_dn4_slot = var_qbdif_dn4;
        *var_qbdif_dn6_slot = var_qbdif_dn6;
        *var_qbdif_dn7_slot = var_qbdif_dn7;
        *var_qbdif_dn8_slot = var_qbdif_dn8;
        *var_qbdif_dn9_slot = var_qbdif_dn9;
        *var_qbsif_slot = var_qbsif;
        *var_qbsif_dn4_slot = var_qbsif_dn4;
        *var_qbsif_dn6_slot = var_qbsif_dn6;
        *var_qbsif_dn7_slot = var_qbsif_dn7;
        *var_qbsif_dn8_slot = var_qbsif_dn8;
        *var_qbsif_dn9_slot = var_qbsif_dn9;
        *var_qd_slot = var_qd;
        *var_qd_dn4_slot = var_qd_dn4;
        *var_qd_dn6_slot = var_qd_dn6;
        *var_qd_dn7_slot = var_qd_dn7;
        *var_qd_dn8_slot = var_qd_dn8;
        *var_qd_dn9_slot = var_qd_dn9;
        *var_qdse_slot = var_qdse;
        *var_qdse_dn6_slot = var_qdse_dn6;
        *var_qdse_dn7_slot = var_qdse_dn7;
        *var_qdsub_slot = var_qdsub;
        *var_qdsub_dn6_slot = var_qdsub_dn6;
        *var_qdsub_dn7_slot = var_qdsub_dn7;
        *var_qdsub_dn8_slot = var_qdsub_dn8;
        *var_qg_slot = var_qg;
        *var_qg_dn4_slot = var_qg_dn4;
        *var_qg_dn6_slot = var_qg_dn6;
        *var_qg_dn7_slot = var_qg_dn7;
        *var_qg_dn8_slot = var_qg_dn8;
        *var_qg_dn9_slot = var_qg_dn9;
        *var_qgbe_slot = var_qgbe;
        *var_qgbe_dn4_slot = var_qgbe_dn4;
        *var_qgbe_dn6_slot = var_qgbe_dn6;
        *var_qgbe_dn7_slot = var_qgbe_dn7;
        *var_qgbe_dn8_slot = var_qgbe_dn8;
        *var_qgbe_dn9_slot = var_qgbe_dn9;
        *var_qgde_slot = var_qgde;
        *var_qgde_dn4_slot = var_qgde_dn4;
        *var_qgde_dn6_slot = var_qgde_dn6;
        *var_qgde_dn7_slot = var_qgde_dn7;
        *var_qgde_dn8_slot = var_qgde_dn8;
        *var_qgde_dn9_slot = var_qgde_dn9;
        *var_qgdif_slot = var_qgdif;
        *var_qgdif_dn4_slot = var_qgdif_dn4;
        *var_qgdif_dn6_slot = var_qgdif_dn6;
        *var_qgdif_dn7_slot = var_qgdif_dn7;
        *var_qgdif_dn8_slot = var_qgdif_dn8;
        *var_qgdif_dn9_slot = var_qgdif_dn9;
        *var_qgse_slot = var_qgse;
        *var_qgse_dn4_slot = var_qgse_dn4;
        *var_qgse_dn6_slot = var_qgse_dn6;
        *var_qgse_dn7_slot = var_qgse_dn7;
        *var_qgse_dn8_slot = var_qgse_dn8;
        *var_qgse_dn9_slot = var_qgse_dn9;
        *var_qgsif_slot = var_qgsif;
        *var_qgsif_dn4_slot = var_qgsif_dn4;
        *var_qgsif_dn6_slot = var_qgsif_dn6;
        *var_qgsif_dn7_slot = var_qgsif_dn7;
        *var_qgsif_dn8_slot = var_qgsif_dn8;
        *var_qgsif_dn9_slot = var_qgsif_dn9;
        *var_qovd_slot = var_qovd;
        *var_qovd_dn4_slot = var_qovd_dn4;
        *var_qovd_dn6_slot = var_qovd_dn6;
        *var_qovd_dn7_slot = var_qovd_dn7;
        *var_qovd_dn8_slot = var_qovd_dn8;
        *var_qovd_dn9_slot = var_qovd_dn9;
        *var_qovs_slot = var_qovs;
        *var_qovs_dn4_slot = var_qovs_dn4;
        *var_qovs_dn6_slot = var_qovs_dn6;
        *var_qovs_dn7_slot = var_qovs_dn7;
        *var_qovs_dn8_slot = var_qovs_dn8;
        *var_qovs_dn9_slot = var_qovs_dn9;
        *var_qs_slot = var_qs;
        *var_qs_dn4_slot = var_qs_dn4;
        *var_qs_dn6_slot = var_qs_dn6;
        *var_qs_dn7_slot = var_qs_dn7;
        *var_qs_dn8_slot = var_qs_dn8;
        *var_qs_dn9_slot = var_qs_dn9;
        *var_qssub_slot = var_qssub;
        *var_qssub_dn6_slot = var_qssub_dn6;
        *var_qssub_dn8_slot = var_qssub_dn8;
        *var_qth_slot = var_qth;
        *var_qth_dn4_slot = var_qth_dn4;
        *var_qth_dn6_slot = var_qth_dn6;
        *var_qth_dn7_slot = var_qth_dn7;
        *var_qth_dn8_slot = var_qth_dn8;
        *var_qth_dn9_slot = var_qth_dn9;
        *var_temp_slot = var_temp;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_temp_q_slot = var_temp_q;
        *var_temp_q_dn4_slot = var_temp_q_dn4;
        *var_temp_q_dn6_slot = var_temp_q_dn6;
        *var_temp_q_dn7_slot = var_temp_q_dn7;
        *var_temp_q_dn8_slot = var_temp_q_dn8;
        *var_temp_q_dn9_slot = var_temp_q_dn9;
    }

    pub(super) fn stamp_transient_block_120(
        p: &Parameters,
        var_areaq_i: f64,
        var_betneff: f64,
        var_betneff_dn4: f64,
        var_betneff_dn6: f64,
        var_betneff_dn7: f64,
        var_betneff_dn8: f64,
        var_betneff_dn9: f64,
        var_csiprime_ac: f64,
        var_csiprime_ac_dn4: f64,
        var_csiprime_ac_dn6: f64,
        var_csiprime_ac_dn7: f64,
        var_csiprime_ac_dn8: f64,
        var_csiprime_ac_dn9: f64,
        var_csiprime_dc: f64,
        var_csiprime_dc_dn4: f64,
        var_csiprime_dc_dn6: f64,
        var_csiprime_dc_dn7: f64,
        var_csiprime_dc_dn8: f64,
        var_csiprime_dc_dn9: f64,
        var_dd_dc: f64,
        var_dd_dc_dn4: f64,
        var_dd_dc_dn6: f64,
        var_dd_dc_dn7: f64,
        var_dd_dc_dn8: f64,
        var_dd_dc_dn9: f64,
        var_delta_k1q1_dc: f64,
        var_delta_k1q1_dc_dn4: f64,
        var_delta_k1q1_dc_dn6: f64,
        var_delta_k1q1_dc_dn7: f64,
        var_delta_k1q1_dc_dn8: f64,
        var_delta_k1q1_dc_dn9: f64,
        var_ds_dc: f64,
        var_ds_dc_dn4: f64,
        var_ds_dc_dn6: f64,
        var_ds_dc_dn7: f64,
        var_ds_dc_dn8: f64,
        var_ds_dc_dn9: f64,
        var_esurf1_dc: f64,
        var_esurf1_dc_dn4: f64,
        var_esurf1_dc_dn6: f64,
        var_esurf1_dc_dn7: f64,
        var_esurf1_dc_dn8: f64,
        var_esurf1_dc_dn9: f64,
        var_esurf2_dc: f64,
        var_esurf2_dc_dn4: f64,
        var_esurf2_dc_dn6: f64,
        var_esurf2_dc_dn7: f64,
        var_esurf2_dc_dn8: f64,
        var_esurf2_dc_dn9: f64,
        var_fac_exc: f64,
        var_fact_ids: f64,
        var_fact_ids_dn4: f64,
        var_fact_ids_dn6: f64,
        var_fact_ids_dn7: f64,
        var_fact_ids_dn8: f64,
        var_fact_ids_dn9: f64,
        var_fdl: f64,
        var_fdl_dn4: f64,
        var_fdl_dn6: f64,
        var_fdl_dn7: f64,
        var_fdl_dn8: f64,
        var_fdl_dn9: f64,
        var_fntexc_i: f64,
        var_ggamma_dc: f64,
        var_gmob_dc: f64,
        var_guard1245: f64,
        var_gvsat: f64,
        var_gvsat_dn4: f64,
        var_gvsat_dn6: f64,
        var_gvsat_dn7: f64,
        var_gvsat_dn8: f64,
        var_gvsat_dn9: f64,
        var_hsat_dc: f64,
        var_hsat_dc_dn4: f64,
        var_hsat_dc_dn6: f64,
        var_hsat_dc_dn7: f64,
        var_hsat_dc_dn8: f64,
        var_hsat_dc_dn9: f64,
        var_ids: f64,
        var_ids_dn4: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_ids_dn8: f64,
        var_ids_dn9: f64,
        var_inv_k1h1_0_dc: f64,
        var_inv_k1h1_0_dc_dn4: f64,
        var_inv_k1h1_0_dc_dn6: f64,
        var_inv_k1h1_0_dc_dn7: f64,
        var_inv_k1h1_0_dc_dn8: f64,
        var_inv_k1h1_0_dc_dn9: f64,
        var_k1_ac: f64,
        var_k1_ac_dn4: f64,
        var_k1_ac_dn6: f64,
        var_k1_ac_dn7: f64,
        var_k1_ac_dn8: f64,
        var_k1_ac_dn9: f64,
        var_nfa_i: f64,
        var_nfb_i: f64,
        var_nfc_i: f64,
        var_nfe_i: f64,
        var_nfeb_i: f64,
        var_nt0: f64,
        var_one_m_xge: f64,
        var_phit: f64,
        var_phit0: f64,
        var_phit_dn4: f64,
        var_phit_dn6: f64,
        var_phit_dn7: f64,
        var_phit_dn8: f64,
        var_phit_dn9: f64,
        var_qid_dc: f64,
        var_qid_dc_dn4: f64,
        var_qid_dc_dn6: f64,
        var_qid_dc_dn7: f64,
        var_qid_dc_dn8: f64,
        var_qid_dc_dn9: f64,
        var_qim_dc: f64,
        var_qim_dc_dn4: f64,
        var_qim_dc_dn6: f64,
        var_qim_dc_dn7: f64,
        var_qim_dc_dn8: f64,
        var_qim_dc_dn9: f64,
        var_qis_dc: f64,
        var_qis_dc_dn4: f64,
        var_qis_dc_dn6: f64,
        var_qis_dc_dn7: f64,
        var_qis_dc_dn8: f64,
        var_qis_dc_dn9: f64,
        var_qmfact: f64,
        var_qmfact1_ac: f64,
        var_qmfact1_ac_dn4: f64,
        var_qmfact1_ac_dn6: f64,
        var_qmfact1_ac_dn7: f64,
        var_qmfact1_ac_dn8: f64,
        var_qmfact1_ac_dn9: f64,
        var_qmfact_dn4: f64,
        var_qmfact_dn6: f64,
        var_qmfact_dn7: f64,
        var_qmfact_dn8: f64,
        var_qmfact_dn9: f64,
        var_sigvds: f64,
        var_temp_q: f64,
        var_temp_q_dn4: f64,
        var_temp_q_dn6: f64,
        var_temp_q_dn7: f64,
        var_temp_q_dn8: f64,
        var_temp_q_dn9: f64,
        var_tkd: f64,
        var_tkd_dn4: f64,
        var_tkd_dn6: f64,
        var_tkd_dn7: f64,
        var_tkd_dn8: f64,
        var_tkd_dn9: f64,
        var_xdeff_dc: f64,
        var_xge_i: f64,
        var_zsat_ac: f64,
        var_zsat_ac_dn4: f64,
        var_zsat_ac_dn6: f64,
        var_zsat_ac_dn7: f64,
        var_zsat_ac_dn8: f64,
        var_zsat_ac_dn9: f64,
        var_cdgeff_slot: &mut f64,
        var_cdgeff_dn4_slot: &mut f64,
        var_cdgeff_dn6_slot: &mut f64,
        var_cdgeff_dn7_slot: &mut f64,
        var_cdgeff_dn8_slot: &mut f64,
        var_cdgeff_dn9_slot: &mut f64,
        var_cgeff_slot: &mut f64,
        var_cgeff_dn4_slot: &mut f64,
        var_cgeff_dn6_slot: &mut f64,
        var_cgeff_dn7_slot: &mut f64,
        var_cgeff_dn8_slot: &mut f64,
        var_cgeff_dn9_slot: &mut f64,
        var_cox_qm_slot: &mut f64,
        var_cox_qm_dn4_slot: &mut f64,
        var_cox_qm_dn6_slot: &mut f64,
        var_cox_qm_dn7_slot: &mut f64,
        var_cox_qm_dn8_slot: &mut f64,
        var_cox_qm_dn9_slot: &mut f64,
        var_csgeff_slot: &mut f64,
        var_csgeff_dn4_slot: &mut f64,
        var_csgeff_dn6_slot: &mut f64,
        var_csgeff_dn7_slot: &mut f64,
        var_csgeff_dn8_slot: &mut f64,
        var_csgeff_dn9_slot: &mut f64,
        var_deg_op_slot: &mut f64,
        var_deg_op_dn4_slot: &mut f64,
        var_deg_op_dn6_slot: &mut f64,
        var_deg_op_dn7_slot: &mut f64,
        var_deg_op_dn8_slot: &mut f64,
        var_deg_op_dn9_slot: &mut f64,
        var_deltan_slot: &mut f64,
        var_deltan_dn4_slot: &mut f64,
        var_deltan_dn6_slot: &mut f64,
        var_deltan_dn7_slot: &mut f64,
        var_deltan_dn8_slot: &mut f64,
        var_deltan_dn9_slot: &mut f64,
        var_dm_slot: &mut f64,
        var_dm_dn4_slot: &mut f64,
        var_dm_dn6_slot: &mut f64,
        var_dm_dn7_slot: &mut f64,
        var_dm_dn8_slot: &mut f64,
        var_dm_dn9_slot: &mut f64,
        var_dvfbch_op_slot: &mut f64,
        var_dvfbch_op_dn4_slot: &mut f64,
        var_dvfbch_op_dn6_slot: &mut f64,
        var_dvfbch_op_dn7_slot: &mut f64,
        var_dvfbch_op_dn8_slot: &mut f64,
        var_dvfbch_op_dn9_slot: &mut f64,
        var_eg_2phit0_op_slot: &mut f64,
        var_eg_2phit0_op_dn4_slot: &mut f64,
        var_eg_2phit0_op_dn6_slot: &mut f64,
        var_eg_2phit0_op_dn7_slot: &mut f64,
        var_eg_2phit0_op_dn8_slot: &mut f64,
        var_eg_2phit0_op_dn9_slot: &mut f64,
        var_eg_op_slot: &mut f64,
        var_eg_op_dn4_slot: &mut f64,
        var_eg_op_dn6_slot: &mut f64,
        var_eg_op_dn7_slot: &mut f64,
        var_eg_op_dn8_slot: &mut f64,
        var_eg_op_dn9_slot: &mut f64,
        var_egge_op_slot: &mut f64,
        var_egge_op_dn4_slot: &mut f64,
        var_egge_op_dn6_slot: &mut f64,
        var_egge_op_dn7_slot: &mut f64,
        var_egge_op_dn8_slot: &mut f64,
        var_egge_op_dn9_slot: &mut f64,
        var_egsi_op_slot: &mut f64,
        var_egsi_op_dn4_slot: &mut f64,
        var_egsi_op_dn6_slot: &mut f64,
        var_egsi_op_dn7_slot: &mut f64,
        var_egsi_op_dn8_slot: &mut f64,
        var_egsi_op_dn9_slot: &mut f64,
        var_g_ideal_slot: &mut f64,
        var_g_ideal_dn4_slot: &mut f64,
        var_g_ideal_dn6_slot: &mut f64,
        var_g_ideal_dn7_slot: &mut f64,
        var_g_ideal_dn8_slot: &mut f64,
        var_g_ideal_dn9_slot: &mut f64,
        var_gsid_slot: &mut f64,
        var_gsig_slot: &mut f64,
        var_gsig_dn4_slot: &mut f64,
        var_gsig_dn6_slot: &mut f64,
        var_gsig_dn7_slot: &mut f64,
        var_gsig_dn8_slot: &mut f64,
        var_gsig_dn9_slot: &mut f64,
        var_guard1278_slot: &mut f64,
        var_guard1279_slot: &mut f64,
        var_guard1280_slot: &mut f64,
        var_inv_phit0_op_slot: &mut f64,
        var_inv_phit0_op_dn4_slot: &mut f64,
        var_inv_phit0_op_dn6_slot: &mut f64,
        var_inv_phit0_op_dn7_slot: &mut f64,
        var_inv_phit0_op_dn8_slot: &mut f64,
        var_inv_phit0_op_dn9_slot: &mut f64,
        var_lc_slot: &mut f64,
        var_lc_dn4_slot: &mut f64,
        var_lc_dn6_slot: &mut f64,
        var_lc_dn7_slot: &mut f64,
        var_lc_dn8_slot: &mut f64,
        var_lc_dn9_slot: &mut f64,
        var_lcinv2_slot: &mut f64,
        var_lcinv2_dn4_slot: &mut f64,
        var_lcinv2_dn6_slot: &mut f64,
        var_lcinv2_dn7_slot: &mut f64,
        var_lcinv2_dn8_slot: &mut f64,
        var_lcinv2_dn9_slot: &mut f64,
        var_migid_slot: &mut f64,
        var_migid_dn4_slot: &mut f64,
        var_migid_dn6_slot: &mut f64,
        var_migid_dn7_slot: &mut f64,
        var_migid_dn8_slot: &mut f64,
        var_migid_dn9_slot: &mut f64,
        var_nmstar_slot: &mut f64,
        var_nmstar_dn4_slot: &mut f64,
        var_nmstar_dn6_slot: &mut f64,
        var_nmstar_dn7_slot: &mut f64,
        var_nmstar_dn8_slot: &mut f64,
        var_nmstar_dn9_slot: &mut f64,
        var_nstar_slot: &mut f64,
        var_nstar_dn4_slot: &mut f64,
        var_nstar_dn6_slot: &mut f64,
        var_nstar_dn7_slot: &mut f64,
        var_nstar_dn8_slot: &mut f64,
        var_nstar_dn9_slot: &mut f64,
        var_nunit_slot: &mut f64,
        var_nunit_dn4_slot: &mut f64,
        var_nunit_dn6_slot: &mut f64,
        var_nunit_dn7_slot: &mut f64,
        var_nunit_dn8_slot: &mut f64,
        var_nunit_dn9_slot: &mut f64,
        var_qbsif_slot: &mut f64,
        var_qbsif_dn4_slot: &mut f64,
        var_qbsif_dn6_slot: &mut f64,
        var_qbsif_dn7_slot: &mut f64,
        var_qbsif_dn8_slot: &mut f64,
        var_qbsif_dn9_slot: &mut f64,
        var_qimstar_slot: &mut f64,
        var_qimstar_dn4_slot: &mut f64,
        var_qimstar_dn6_slot: &mut f64,
        var_qimstar_dn7_slot: &mut f64,
        var_qimstar_dn8_slot: &mut f64,
        var_qimstar_dn9_slot: &mut f64,
        var_r_slot: &mut f64,
        var_r_dn4_slot: &mut f64,
        var_r_dn6_slot: &mut f64,
        var_r_dn7_slot: &mut f64,
        var_r_dn8_slot: &mut f64,
        var_r_dn9_slot: &mut f64,
        var_sidexc_slot: &mut f64,
        var_sqrt_t2_slot: &mut f64,
        var_sqrt_t2_dn4_slot: &mut f64,
        var_sqrt_t2_dn6_slot: &mut f64,
        var_sqrt_t2_dn7_slot: &mut f64,
        var_sqrt_t2_dn8_slot: &mut f64,
        var_sqrt_t2_dn9_slot: &mut f64,
        var_sqrt_zsatexc_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2x12_slot: &mut f64,
        var_t2x12_dn4_slot: &mut f64,
        var_t2x12_dn6_slot: &mut f64,
        var_t2x12_dn7_slot: &mut f64,
        var_t2x12_dn8_slot: &mut f64,
        var_t2x12_dn9_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp3_slot: &mut f64,
        var_temp3_dn4_slot: &mut f64,
        var_temp3_dn6_slot: &mut f64,
        var_temp3_dn7_slot: &mut f64,
        var_temp3_dn8_slot: &mut f64,
        var_temp3_dn9_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
    ) {
        let mut var_cdgeff: f64 = *var_cdgeff_slot;
        let mut var_cdgeff_dn4: f64 = *var_cdgeff_dn4_slot;
        let mut var_cdgeff_dn6: f64 = *var_cdgeff_dn6_slot;
        let mut var_cdgeff_dn7: f64 = *var_cdgeff_dn7_slot;
        let mut var_cdgeff_dn8: f64 = *var_cdgeff_dn8_slot;
        let mut var_cdgeff_dn9: f64 = *var_cdgeff_dn9_slot;
        let mut var_cgeff: f64 = *var_cgeff_slot;
        let mut var_cgeff_dn4: f64 = *var_cgeff_dn4_slot;
        let mut var_cgeff_dn6: f64 = *var_cgeff_dn6_slot;
        let mut var_cgeff_dn7: f64 = *var_cgeff_dn7_slot;
        let mut var_cgeff_dn8: f64 = *var_cgeff_dn8_slot;
        let mut var_cgeff_dn9: f64 = *var_cgeff_dn9_slot;
        let mut var_cox_qm: f64 = *var_cox_qm_slot;
        let mut var_cox_qm_dn4: f64 = *var_cox_qm_dn4_slot;
        let mut var_cox_qm_dn6: f64 = *var_cox_qm_dn6_slot;
        let mut var_cox_qm_dn7: f64 = *var_cox_qm_dn7_slot;
        let mut var_cox_qm_dn8: f64 = *var_cox_qm_dn8_slot;
        let mut var_cox_qm_dn9: f64 = *var_cox_qm_dn9_slot;
        let mut var_csgeff: f64 = *var_csgeff_slot;
        let mut var_csgeff_dn4: f64 = *var_csgeff_dn4_slot;
        let mut var_csgeff_dn6: f64 = *var_csgeff_dn6_slot;
        let mut var_csgeff_dn7: f64 = *var_csgeff_dn7_slot;
        let mut var_csgeff_dn8: f64 = *var_csgeff_dn8_slot;
        let mut var_csgeff_dn9: f64 = *var_csgeff_dn9_slot;
        let mut var_deg_op: f64 = *var_deg_op_slot;
        let mut var_deg_op_dn4: f64 = *var_deg_op_dn4_slot;
        let mut var_deg_op_dn6: f64 = *var_deg_op_dn6_slot;
        let mut var_deg_op_dn7: f64 = *var_deg_op_dn7_slot;
        let mut var_deg_op_dn8: f64 = *var_deg_op_dn8_slot;
        let mut var_deg_op_dn9: f64 = *var_deg_op_dn9_slot;
        let mut var_deltan: f64 = *var_deltan_slot;
        let mut var_deltan_dn4: f64 = *var_deltan_dn4_slot;
        let mut var_deltan_dn6: f64 = *var_deltan_dn6_slot;
        let mut var_deltan_dn7: f64 = *var_deltan_dn7_slot;
        let mut var_deltan_dn8: f64 = *var_deltan_dn8_slot;
        let mut var_deltan_dn9: f64 = *var_deltan_dn9_slot;
        let mut var_dm: f64 = *var_dm_slot;
        let mut var_dm_dn4: f64 = *var_dm_dn4_slot;
        let mut var_dm_dn6: f64 = *var_dm_dn6_slot;
        let mut var_dm_dn7: f64 = *var_dm_dn7_slot;
        let mut var_dm_dn8: f64 = *var_dm_dn8_slot;
        let mut var_dm_dn9: f64 = *var_dm_dn9_slot;
        let mut var_dvfbch_op: f64 = *var_dvfbch_op_slot;
        let mut var_dvfbch_op_dn4: f64 = *var_dvfbch_op_dn4_slot;
        let mut var_dvfbch_op_dn6: f64 = *var_dvfbch_op_dn6_slot;
        let mut var_dvfbch_op_dn7: f64 = *var_dvfbch_op_dn7_slot;
        let mut var_dvfbch_op_dn8: f64 = *var_dvfbch_op_dn8_slot;
        let mut var_dvfbch_op_dn9: f64 = *var_dvfbch_op_dn9_slot;
        let mut var_eg_2phit0_op: f64 = *var_eg_2phit0_op_slot;
        let mut var_eg_2phit0_op_dn4: f64 = *var_eg_2phit0_op_dn4_slot;
        let mut var_eg_2phit0_op_dn6: f64 = *var_eg_2phit0_op_dn6_slot;
        let mut var_eg_2phit0_op_dn7: f64 = *var_eg_2phit0_op_dn7_slot;
        let mut var_eg_2phit0_op_dn8: f64 = *var_eg_2phit0_op_dn8_slot;
        let mut var_eg_2phit0_op_dn9: f64 = *var_eg_2phit0_op_dn9_slot;
        let mut var_eg_op: f64 = *var_eg_op_slot;
        let mut var_eg_op_dn4: f64 = *var_eg_op_dn4_slot;
        let mut var_eg_op_dn6: f64 = *var_eg_op_dn6_slot;
        let mut var_eg_op_dn7: f64 = *var_eg_op_dn7_slot;
        let mut var_eg_op_dn8: f64 = *var_eg_op_dn8_slot;
        let mut var_eg_op_dn9: f64 = *var_eg_op_dn9_slot;
        let mut var_egge_op: f64 = *var_egge_op_slot;
        let mut var_egge_op_dn4: f64 = *var_egge_op_dn4_slot;
        let mut var_egge_op_dn6: f64 = *var_egge_op_dn6_slot;
        let mut var_egge_op_dn7: f64 = *var_egge_op_dn7_slot;
        let mut var_egge_op_dn8: f64 = *var_egge_op_dn8_slot;
        let mut var_egge_op_dn9: f64 = *var_egge_op_dn9_slot;
        let mut var_egsi_op: f64 = *var_egsi_op_slot;
        let mut var_egsi_op_dn4: f64 = *var_egsi_op_dn4_slot;
        let mut var_egsi_op_dn6: f64 = *var_egsi_op_dn6_slot;
        let mut var_egsi_op_dn7: f64 = *var_egsi_op_dn7_slot;
        let mut var_egsi_op_dn8: f64 = *var_egsi_op_dn8_slot;
        let mut var_egsi_op_dn9: f64 = *var_egsi_op_dn9_slot;
        let mut var_g_ideal: f64 = *var_g_ideal_slot;
        let mut var_g_ideal_dn4: f64 = *var_g_ideal_dn4_slot;
        let mut var_g_ideal_dn6: f64 = *var_g_ideal_dn6_slot;
        let mut var_g_ideal_dn7: f64 = *var_g_ideal_dn7_slot;
        let mut var_g_ideal_dn8: f64 = *var_g_ideal_dn8_slot;
        let mut var_g_ideal_dn9: f64 = *var_g_ideal_dn9_slot;
        let mut var_gsid: f64 = *var_gsid_slot;
        let mut var_gsig: f64 = *var_gsig_slot;
        let mut var_gsig_dn4: f64 = *var_gsig_dn4_slot;
        let mut var_gsig_dn6: f64 = *var_gsig_dn6_slot;
        let mut var_gsig_dn7: f64 = *var_gsig_dn7_slot;
        let mut var_gsig_dn8: f64 = *var_gsig_dn8_slot;
        let mut var_gsig_dn9: f64 = *var_gsig_dn9_slot;
        let mut var_guard1278: f64 = *var_guard1278_slot;
        let mut var_guard1279: f64 = *var_guard1279_slot;
        let mut var_guard1280: f64 = *var_guard1280_slot;
        let mut var_inv_phit0_op: f64 = *var_inv_phit0_op_slot;
        let mut var_inv_phit0_op_dn4: f64 = *var_inv_phit0_op_dn4_slot;
        let mut var_inv_phit0_op_dn6: f64 = *var_inv_phit0_op_dn6_slot;
        let mut var_inv_phit0_op_dn7: f64 = *var_inv_phit0_op_dn7_slot;
        let mut var_inv_phit0_op_dn8: f64 = *var_inv_phit0_op_dn8_slot;
        let mut var_inv_phit0_op_dn9: f64 = *var_inv_phit0_op_dn9_slot;
        let mut var_lc: f64 = *var_lc_slot;
        let mut var_lc_dn4: f64 = *var_lc_dn4_slot;
        let mut var_lc_dn6: f64 = *var_lc_dn6_slot;
        let mut var_lc_dn7: f64 = *var_lc_dn7_slot;
        let mut var_lc_dn8: f64 = *var_lc_dn8_slot;
        let mut var_lc_dn9: f64 = *var_lc_dn9_slot;
        let mut var_lcinv2: f64 = *var_lcinv2_slot;
        let mut var_lcinv2_dn4: f64 = *var_lcinv2_dn4_slot;
        let mut var_lcinv2_dn6: f64 = *var_lcinv2_dn6_slot;
        let mut var_lcinv2_dn7: f64 = *var_lcinv2_dn7_slot;
        let mut var_lcinv2_dn8: f64 = *var_lcinv2_dn8_slot;
        let mut var_lcinv2_dn9: f64 = *var_lcinv2_dn9_slot;
        let mut var_migid: f64 = *var_migid_slot;
        let mut var_migid_dn4: f64 = *var_migid_dn4_slot;
        let mut var_migid_dn6: f64 = *var_migid_dn6_slot;
        let mut var_migid_dn7: f64 = *var_migid_dn7_slot;
        let mut var_migid_dn8: f64 = *var_migid_dn8_slot;
        let mut var_migid_dn9: f64 = *var_migid_dn9_slot;
        let mut var_nmstar: f64 = *var_nmstar_slot;
        let mut var_nmstar_dn4: f64 = *var_nmstar_dn4_slot;
        let mut var_nmstar_dn6: f64 = *var_nmstar_dn6_slot;
        let mut var_nmstar_dn7: f64 = *var_nmstar_dn7_slot;
        let mut var_nmstar_dn8: f64 = *var_nmstar_dn8_slot;
        let mut var_nmstar_dn9: f64 = *var_nmstar_dn9_slot;
        let mut var_nstar: f64 = *var_nstar_slot;
        let mut var_nstar_dn4: f64 = *var_nstar_dn4_slot;
        let mut var_nstar_dn6: f64 = *var_nstar_dn6_slot;
        let mut var_nstar_dn7: f64 = *var_nstar_dn7_slot;
        let mut var_nstar_dn8: f64 = *var_nstar_dn8_slot;
        let mut var_nstar_dn9: f64 = *var_nstar_dn9_slot;
        let mut var_nunit: f64 = *var_nunit_slot;
        let mut var_nunit_dn4: f64 = *var_nunit_dn4_slot;
        let mut var_nunit_dn6: f64 = *var_nunit_dn6_slot;
        let mut var_nunit_dn7: f64 = *var_nunit_dn7_slot;
        let mut var_nunit_dn8: f64 = *var_nunit_dn8_slot;
        let mut var_nunit_dn9: f64 = *var_nunit_dn9_slot;
        let mut var_qbsif: f64 = *var_qbsif_slot;
        let mut var_qbsif_dn4: f64 = *var_qbsif_dn4_slot;
        let mut var_qbsif_dn6: f64 = *var_qbsif_dn6_slot;
        let mut var_qbsif_dn7: f64 = *var_qbsif_dn7_slot;
        let mut var_qbsif_dn8: f64 = *var_qbsif_dn8_slot;
        let mut var_qbsif_dn9: f64 = *var_qbsif_dn9_slot;
        let mut var_qimstar: f64 = *var_qimstar_slot;
        let mut var_qimstar_dn4: f64 = *var_qimstar_dn4_slot;
        let mut var_qimstar_dn6: f64 = *var_qimstar_dn6_slot;
        let mut var_qimstar_dn7: f64 = *var_qimstar_dn7_slot;
        let mut var_qimstar_dn8: f64 = *var_qimstar_dn8_slot;
        let mut var_qimstar_dn9: f64 = *var_qimstar_dn9_slot;
        let mut var_r: f64 = *var_r_slot;
        let mut var_r_dn4: f64 = *var_r_dn4_slot;
        let mut var_r_dn6: f64 = *var_r_dn6_slot;
        let mut var_r_dn7: f64 = *var_r_dn7_slot;
        let mut var_r_dn8: f64 = *var_r_dn8_slot;
        let mut var_r_dn9: f64 = *var_r_dn9_slot;
        let mut var_sidexc: f64 = *var_sidexc_slot;
        let mut var_sqrt_t2: f64 = *var_sqrt_t2_slot;
        let mut var_sqrt_t2_dn4: f64 = *var_sqrt_t2_dn4_slot;
        let mut var_sqrt_t2_dn6: f64 = *var_sqrt_t2_dn6_slot;
        let mut var_sqrt_t2_dn7: f64 = *var_sqrt_t2_dn7_slot;
        let mut var_sqrt_t2_dn8: f64 = *var_sqrt_t2_dn8_slot;
        let mut var_sqrt_t2_dn9: f64 = *var_sqrt_t2_dn9_slot;
        let mut var_sqrt_zsatexc: f64 = *var_sqrt_zsatexc_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2x12: f64 = *var_t2x12_slot;
        let mut var_t2x12_dn4: f64 = *var_t2x12_dn4_slot;
        let mut var_t2x12_dn6: f64 = *var_t2x12_dn6_slot;
        let mut var_t2x12_dn7: f64 = *var_t2x12_dn7_slot;
        let mut var_t2x12_dn8: f64 = *var_t2x12_dn8_slot;
        let mut var_t2x12_dn9: f64 = *var_t2x12_dn9_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp3: f64 = *var_temp3_slot;
        let mut var_temp3_dn4: f64 = *var_temp3_dn4_slot;
        let mut var_temp3_dn6: f64 = *var_temp3_dn6_slot;
        let mut var_temp3_dn7: f64 = *var_temp3_dn7_slot;
        let mut var_temp3_dn8: f64 = *var_temp3_dn8_slot;
        let mut var_temp3_dn9: f64 = *var_temp3_dn9_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;

        let (assign43180_e48444, assign43180_e48444_d_n4, assign43180_e48444_d_n6, assign43180_e48444_d_n7, assign43180_e48444_d_n8, assign43180_e48444_d_n9,) = {
    if (var_guard1245 != 0.0) {
        (var_temp_q, var_temp_q_dn4, var_temp_q_dn6, var_temp_q_dn7, var_temp_q_dn8, var_temp_q_dn9,)
    } else {
        (var_qbsif, var_qbsif_dn4, var_qbsif_dn6, var_qbsif_dn7, var_qbsif_dn8, var_qbsif_dn9,)
    }
};
        var_qbsif = assign43180_e48444;
        var_qbsif_dn4 = assign43180_e48444_d_n4;
        var_qbsif_dn6 = assign43180_e48444_d_n6;
        var_qbsif_dn7 = assign43180_e48444_d_n7;
        var_qbsif_dn8 = assign43180_e48444_d_n8;
        var_qbsif_dn9 = assign43180_e48444_d_n9;

        let assign43190_e48447: f64 = (var_csiprime_dc / 1.602176565e-19);
        let assign43190_e48449: f64 = (assign43190_e48447 * var_phit);
        var_nunit = assign43190_e48449;
        var_nunit_dn4 = (((var_csiprime_dc_dn4 / 1.602176565e-19) * var_phit) + (assign43190_e48447 * var_phit_dn4));
        var_nunit_dn6 = (((var_csiprime_dc_dn6 / 1.602176565e-19) * var_phit) + (assign43190_e48447 * var_phit_dn6));
        var_nunit_dn7 = (((var_csiprime_dc_dn7 / 1.602176565e-19) * var_phit) + (assign43190_e48447 * var_phit_dn7));
        var_nunit_dn8 = (((var_csiprime_dc_dn8 / 1.602176565e-19) * var_phit) + (assign43190_e48447 * var_phit_dn8));
        var_nunit_dn9 = (((var_csiprime_dc_dn9 / 1.602176565e-19) * var_phit) + (assign43190_e48447 * var_phit_dn9));

        let assign43200_e48451: f64 = (-0.5);
        let assign43200_e48454: f64 = (var_ds_dc + var_dd_dc);
        let assign43200_e48455: f64 = (assign43200_e48451 * assign43200_e48454);
        var_dm = assign43200_e48455;
        var_dm_dn4 = (assign43200_e48451 * (var_ds_dc_dn4 + var_dd_dc_dn4));
        var_dm_dn6 = (assign43200_e48451 * (var_ds_dc_dn6 + var_dd_dc_dn6));
        var_dm_dn7 = (assign43200_e48451 * (var_ds_dc_dn7 + var_dd_dc_dn7));
        var_dm_dn8 = (assign43200_e48451 * (var_ds_dc_dn8 + var_dd_dc_dn8));
        var_dm_dn9 = (assign43200_e48451 * (var_ds_dc_dn9 + var_dd_dc_dn9));

        let assign43210_e48458: f64 = (var_qim_dc + var_dm);
        var_qimstar = assign43210_e48458;
        var_qimstar_dn4 = (var_qim_dc_dn4 + var_dm_dn4);
        var_qimstar_dn6 = (var_qim_dc_dn6 + var_dm_dn6);
        var_qimstar_dn7 = (var_qim_dc_dn7 + var_dm_dn7);
        var_qimstar_dn8 = (var_qim_dc_dn8 + var_dm_dn8);
        var_qimstar_dn9 = (var_qim_dc_dn9 + var_dm_dn9);

        let assign43220_e48461: f64 = (var_qim_dc / var_qimstar);
        var_temp = assign43220_e48461;
        var_temp_dn4 = (((var_qim_dc_dn4 * var_qimstar) - (var_qim_dc * var_qimstar_dn4)) / (var_qimstar * var_qimstar));
        var_temp_dn6 = (((var_qim_dc_dn6 * var_qimstar) - (var_qim_dc * var_qimstar_dn6)) / (var_qimstar * var_qimstar));
        var_temp_dn7 = (((var_qim_dc_dn7 * var_qimstar) - (var_qim_dc * var_qimstar_dn7)) / (var_qimstar * var_qimstar));
        var_temp_dn8 = (((var_qim_dc_dn8 * var_qimstar) - (var_qim_dc * var_qimstar_dn8)) / (var_qimstar * var_qimstar));
        var_temp_dn9 = (((var_qim_dc_dn9 * var_qimstar) - (var_qim_dc * var_qimstar_dn9)) / (var_qimstar * var_qimstar));

        let assign43230_e48465: f64 = var_temp;
        let assign43230_e48468: f64 = var_temp;
        let assign43230_e48471: f64 = var_temp;
        let assign43230_e48472: f64 = (assign43230_e48468 * assign43230_e48471);
        let assign43230_e48474: f64 = (assign43230_e48472 + 1e-20);
        let assign43230_e48475: f64 = (assign43230_e48474).sqrt();
        let assign43230_e48476: f64 = (assign43230_e48465 + assign43230_e48475);
        let assign43230_e48477: f64 = (0.5 * assign43230_e48476);
        var_t1 = assign43230_e48477;
        var_t1_dn4 = (0.5 * (var_temp_dn4 + (((var_temp_dn4 * assign43230_e48471) + (assign43230_e48468 * var_temp_dn4)) / (2.0 * assign43230_e48475))));
        var_t1_dn6 = (0.5 * (var_temp_dn6 + (((var_temp_dn6 * assign43230_e48471) + (assign43230_e48468 * var_temp_dn6)) / (2.0 * assign43230_e48475))));
        var_t1_dn7 = (0.5 * (var_temp_dn7 + (((var_temp_dn7 * assign43230_e48471) + (assign43230_e48468 * var_temp_dn7)) / (2.0 * assign43230_e48475))));
        var_t1_dn8 = (0.5 * (var_temp_dn8 + (((var_temp_dn8 * assign43230_e48471) + (assign43230_e48468 * var_temp_dn8)) / (2.0 * assign43230_e48475))));
        var_t1_dn9 = (0.5 * (var_temp_dn9 + (((var_temp_dn9 * assign43230_e48471) + (assign43230_e48468 * var_temp_dn9)) / (2.0 * assign43230_e48475))));

        let assign43240_e48479: f64 = (-0.1666666666667);
        let assign43240_e48481: f64 = (assign43240_e48479 * var_delta_k1q1_dc);
        let assign43240_e48483: f64 = (assign43240_e48481 * var_inv_k1h1_0_dc);
        var_sqrt_t2 = assign43240_e48483;
        var_sqrt_t2_dn4 = (((assign43240_e48479 * var_delta_k1q1_dc_dn4) * var_inv_k1h1_0_dc) + (assign43240_e48481 * var_inv_k1h1_0_dc_dn4));
        var_sqrt_t2_dn6 = (((assign43240_e48479 * var_delta_k1q1_dc_dn6) * var_inv_k1h1_0_dc) + (assign43240_e48481 * var_inv_k1h1_0_dc_dn6));
        var_sqrt_t2_dn7 = (((assign43240_e48479 * var_delta_k1q1_dc_dn7) * var_inv_k1h1_0_dc) + (assign43240_e48481 * var_inv_k1h1_0_dc_dn7));
        var_sqrt_t2_dn8 = (((assign43240_e48479 * var_delta_k1q1_dc_dn8) * var_inv_k1h1_0_dc) + (assign43240_e48481 * var_inv_k1h1_0_dc_dn8));
        var_sqrt_t2_dn9 = (((assign43240_e48479 * var_delta_k1q1_dc_dn9) * var_inv_k1h1_0_dc) + (assign43240_e48481 * var_inv_k1h1_0_dc_dn9));

        let assign43250_e48486: f64 = (var_sqrt_t2 * var_sqrt_t2);
        var_t2 = assign43250_e48486;
        var_t2_dn4 = ((var_sqrt_t2_dn4 * var_sqrt_t2) + (var_sqrt_t2 * var_sqrt_t2_dn4));
        var_t2_dn6 = ((var_sqrt_t2_dn6 * var_sqrt_t2) + (var_sqrt_t2 * var_sqrt_t2_dn6));
        var_t2_dn7 = ((var_sqrt_t2_dn7 * var_sqrt_t2) + (var_sqrt_t2 * var_sqrt_t2_dn7));
        var_t2_dn8 = ((var_sqrt_t2_dn8 * var_sqrt_t2) + (var_sqrt_t2 * var_sqrt_t2_dn8));
        var_t2_dn9 = ((var_sqrt_t2_dn9 * var_sqrt_t2) + (var_sqrt_t2 * var_sqrt_t2_dn9));

        let assign43260_e48489: f64 = (var_hsat_dc - 1.0);
        var_r = assign43260_e48489;
        var_r_dn4 = var_hsat_dc_dn4;
        var_r_dn6 = var_hsat_dc_dn6;
        var_r_dn7 = var_hsat_dc_dn7;
        var_r_dn8 = var_hsat_dc_dn8;
        var_r_dn9 = var_hsat_dc_dn9;

        let assign43270_e48493: f64 = (12.0 * var_r);
        let assign43270_e48495: f64 = (assign43270_e48493 * var_t2);
        let assign43270_e48496: f64 = (1.0 - assign43270_e48495);
        let assign43270_e48498: f64 = (assign43270_e48496).max(1e-20);
        var_lc = assign43270_e48498;
        var_lc_dn4 = if assign43270_e48496 >= 1e-20 { (-(((12.0 * var_r_dn4) * var_t2) + (assign43270_e48493 * var_t2_dn4))) } else { 0.0 };
        var_lc_dn6 = if assign43270_e48496 >= 1e-20 { (-(((12.0 * var_r_dn6) * var_t2) + (assign43270_e48493 * var_t2_dn6))) } else { 0.0 };
        var_lc_dn7 = if assign43270_e48496 >= 1e-20 { (-(((12.0 * var_r_dn7) * var_t2) + (assign43270_e48493 * var_t2_dn7))) } else { 0.0 };
        var_lc_dn8 = if assign43270_e48496 >= 1e-20 { (-(((12.0 * var_r_dn8) * var_t2) + (assign43270_e48493 * var_t2_dn8))) } else { 0.0 };
        var_lc_dn9 = if assign43270_e48496 >= 1e-20 { (-(((12.0 * var_r_dn9) * var_t2) + (assign43270_e48493 * var_t2_dn9))) } else { 0.0 };

        let assign43280_e48502: f64 = (var_lc * var_lc);
        let assign43280_e48503: f64 = (1.0 / assign43280_e48502);
        var_lcinv2 = assign43280_e48503;
        var_lcinv2_dn4 = (-(((var_lc_dn4 * var_lc) + (var_lc * var_lc_dn4)) / (assign43280_e48502 * assign43280_e48502)));
        var_lcinv2_dn6 = (-(((var_lc_dn6 * var_lc) + (var_lc * var_lc_dn6)) / (assign43280_e48502 * assign43280_e48502)));
        var_lcinv2_dn7 = (-(((var_lc_dn7 * var_lc) + (var_lc * var_lc_dn7)) / (assign43280_e48502 * assign43280_e48502)));
        var_lcinv2_dn8 = (-(((var_lc_dn8 * var_lc) + (var_lc * var_lc_dn8)) / (assign43280_e48502 * assign43280_e48502)));
        var_lcinv2_dn9 = (-(((var_lc_dn9 * var_lc) + (var_lc * var_lc_dn9)) / (assign43280_e48502 * assign43280_e48502)));

        let assign43290_e48506: f64 = (var_betneff * var_csiprime_dc);
        let assign43290_e48508: f64 = (assign43290_e48506 * var_phit);
        let assign43290_e48510: f64 = (assign43290_e48508 * var_qimstar);
        let assign43290_e48512: f64 = (assign43290_e48510 * var_fdl);
        let assign43290_e48514: f64 = (assign43290_e48512 / var_gvsat);
        let assign43290_e48516: f64 = (assign43290_e48514 / var_qmfact);
        var_g_ideal = assign43290_e48516;
        var_g_ideal_dn4 = ((((((((((((((var_betneff_dn4 * var_csiprime_dc) + (var_betneff * var_csiprime_dc_dn4)) * var_phit) + (assign43290_e48506 * var_phit_dn4)) * var_qimstar) + (assign43290_e48508 * var_qimstar_dn4)) * var_fdl) + (assign43290_e48510 * var_fdl_dn4)) * var_gvsat) - (assign43290_e48512 * var_gvsat_dn4)) / (var_gvsat * var_gvsat)) * var_qmfact) - (assign43290_e48514 * var_qmfact_dn4)) / (var_qmfact * var_qmfact));
        var_g_ideal_dn6 = ((((((((((((((var_betneff_dn6 * var_csiprime_dc) + (var_betneff * var_csiprime_dc_dn6)) * var_phit) + (assign43290_e48506 * var_phit_dn6)) * var_qimstar) + (assign43290_e48508 * var_qimstar_dn6)) * var_fdl) + (assign43290_e48510 * var_fdl_dn6)) * var_gvsat) - (assign43290_e48512 * var_gvsat_dn6)) / (var_gvsat * var_gvsat)) * var_qmfact) - (assign43290_e48514 * var_qmfact_dn6)) / (var_qmfact * var_qmfact));
        var_g_ideal_dn7 = ((((((((((((((var_betneff_dn7 * var_csiprime_dc) + (var_betneff * var_csiprime_dc_dn7)) * var_phit) + (assign43290_e48506 * var_phit_dn7)) * var_qimstar) + (assign43290_e48508 * var_qimstar_dn7)) * var_fdl) + (assign43290_e48510 * var_fdl_dn7)) * var_gvsat) - (assign43290_e48512 * var_gvsat_dn7)) / (var_gvsat * var_gvsat)) * var_qmfact) - (assign43290_e48514 * var_qmfact_dn7)) / (var_qmfact * var_qmfact));
        var_g_ideal_dn8 = ((((((((((((((var_betneff_dn8 * var_csiprime_dc) + (var_betneff * var_csiprime_dc_dn8)) * var_phit) + (assign43290_e48506 * var_phit_dn8)) * var_qimstar) + (assign43290_e48508 * var_qimstar_dn8)) * var_fdl) + (assign43290_e48510 * var_fdl_dn8)) * var_gvsat) - (assign43290_e48512 * var_gvsat_dn8)) / (var_gvsat * var_gvsat)) * var_qmfact) - (assign43290_e48514 * var_qmfact_dn8)) / (var_qmfact * var_qmfact));
        var_g_ideal_dn9 = ((((((((((((((var_betneff_dn9 * var_csiprime_dc) + (var_betneff * var_csiprime_dc_dn9)) * var_phit) + (assign43290_e48506 * var_phit_dn9)) * var_qimstar) + (assign43290_e48508 * var_qimstar_dn9)) * var_fdl) + (assign43290_e48510 * var_fdl_dn9)) * var_gvsat) - (assign43290_e48512 * var_gvsat_dn9)) / (var_gvsat * var_gvsat)) * var_qmfact) - (assign43290_e48514 * var_qmfact_dn9)) / (var_qmfact * var_qmfact));

        let assign43300_e48519: f64 = (12.0 * var_t2);
        var_t2x12 = assign43300_e48519;
        var_t2x12_dn4 = (12.0 * var_t2_dn4);
        var_t2x12_dn6 = (12.0 * var_t2_dn6);
        var_t2x12_dn7 = (12.0 * var_t2_dn7);
        var_t2x12_dn8 = (12.0 * var_t2_dn8);
        var_t2x12_dn9 = (12.0 * var_t2_dn9);

        let assign43310_e48522: f64 = (var_t1 + var_t2x12);
        let assign43310_e48526: f64 = (1.0 + var_t1);
        let assign43310_e48527: f64 = (2.0 * assign43310_e48526);
        let assign43310_e48529: f64 = (assign43310_e48527 * var_t2x12);
        let assign43310_e48531: f64 = (assign43310_e48529 * var_r);
        let assign43310_e48532: f64 = (assign43310_e48522 - assign43310_e48531);
        var_temp1 = assign43310_e48532;
        var_temp1_dn4 = ((var_t1_dn4 + var_t2x12_dn4) - (((((2.0 * var_t1_dn4) * var_t2x12) + (assign43310_e48527 * var_t2x12_dn4)) * var_r) + (assign43310_e48529 * var_r_dn4)));
        var_temp1_dn6 = ((var_t1_dn6 + var_t2x12_dn6) - (((((2.0 * var_t1_dn6) * var_t2x12) + (assign43310_e48527 * var_t2x12_dn6)) * var_r) + (assign43310_e48529 * var_r_dn6)));
        var_temp1_dn7 = ((var_t1_dn7 + var_t2x12_dn7) - (((((2.0 * var_t1_dn7) * var_t2x12) + (assign43310_e48527 * var_t2x12_dn7)) * var_r) + (assign43310_e48529 * var_r_dn7)));
        var_temp1_dn8 = ((var_t1_dn8 + var_t2x12_dn8) - (((((2.0 * var_t1_dn8) * var_t2x12) + (assign43310_e48527 * var_t2x12_dn8)) * var_r) + (assign43310_e48529 * var_r_dn8)));
        var_temp1_dn9 = ((var_t1_dn9 + var_t2x12_dn9) - (((((2.0 * var_t1_dn9) * var_t2x12) + (assign43310_e48527 * var_t2x12_dn9)) * var_r) + (assign43310_e48529 * var_r_dn9)));

        let assign43320_e48535: f64 = (var_temp1).max(1e-40);
        var_temp2 = assign43320_e48535;
        var_temp2_dn4 = if var_temp1 >= 1e-40 { var_temp1_dn4 } else { 0.0 };
        var_temp2_dn6 = if var_temp1 >= 1e-40 { var_temp1_dn6 } else { 0.0 };
        var_temp2_dn7 = if var_temp1 >= 1e-40 { var_temp1_dn7 } else { 0.0 };
        var_temp2_dn8 = if var_temp1 >= 1e-40 { var_temp1_dn8 } else { 0.0 };
        var_temp2_dn9 = if var_temp1 >= 1e-40 { var_temp1_dn9 } else { 0.0 };

        let assign43330_e48538: f64 = (var_g_ideal * var_lcinv2);
        let assign43330_e48540: f64 = (assign43330_e48538 * var_temp2);
        var_gsid = assign43330_e48540;

        let assign43340_e48543: f64 = if var_fntexc_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1278 = assign43340_e48543;

        let (assign43350_e48549,) = {
    if (var_guard1278 != 0.0) {
        let assign43350_e48547: f64 = (var_ggamma_dc / var_gmob_dc);
        (assign43350_e48547,)
    } else {
        (var_sqrt_zsatexc,)
    }
};
        var_sqrt_zsatexc = assign43350_e48549;

        let (assign43360_e48569,) = {
    if (var_guard1278 != 0.0) {
        let assign43360_e48553: f64 = (var_fac_exc * var_ids);
        let assign43360_e48555: f64 = (assign43360_e48553 * var_xdeff_dc);
        let assign43360_e48557: f64 = (assign43360_e48555 * var_phit0);
        let assign43360_e48561: f64 = (var_sqrt_zsatexc * var_sqrt_zsatexc);
        let assign43360_e48562: f64 = (1.0 + assign43360_e48561);
        let assign43360_e48564: f64 = (assign43360_e48562 * var_lc);
        let assign43360_e48566: f64 = (assign43360_e48564 * var_lc);
        let assign43360_e48567: f64 = (assign43360_e48557 / assign43360_e48566);
        (assign43360_e48567,)
    } else {
        (var_sidexc,)
    }
};
        var_sidexc = assign43360_e48569;

        let (assign43370_e48577,) = {
    if (var_guard1278 != 0.0) {
        let assign43370_e48574: f64 = (var_sidexc / var_nt0);
        let assign43370_e48575: f64 = (var_gsid + assign43370_e48574);
        (assign43370_e48575,)
    } else {
        (var_gsid,)
    }
};
        var_gsid = assign43370_e48577;

        let assign43390_e48587: f64 = (var_k1_ac * var_csiprime_ac);
        let assign43390_e48589: f64 = (assign43390_e48587 * var_areaq_i);
        let assign43390_e48591: f64 = (assign43390_e48589 / var_qmfact1_ac);
        var_cox_qm = assign43390_e48591;
        var_cox_qm_dn4 = ((((((var_k1_ac_dn4 * var_csiprime_ac) + (var_k1_ac * var_csiprime_ac_dn4)) * var_areaq_i) * var_qmfact1_ac) - (assign43390_e48589 * var_qmfact1_ac_dn4)) / (var_qmfact1_ac * var_qmfact1_ac));
        var_cox_qm_dn6 = ((((((var_k1_ac_dn6 * var_csiprime_ac) + (var_k1_ac * var_csiprime_ac_dn6)) * var_areaq_i) * var_qmfact1_ac) - (assign43390_e48589 * var_qmfact1_ac_dn6)) / (var_qmfact1_ac * var_qmfact1_ac));
        var_cox_qm_dn7 = ((((((var_k1_ac_dn7 * var_csiprime_ac) + (var_k1_ac * var_csiprime_ac_dn7)) * var_areaq_i) * var_qmfact1_ac) - (assign43390_e48589 * var_qmfact1_ac_dn7)) / (var_qmfact1_ac * var_qmfact1_ac));
        var_cox_qm_dn8 = ((((((var_k1_ac_dn8 * var_csiprime_ac) + (var_k1_ac * var_csiprime_ac_dn8)) * var_areaq_i) * var_qmfact1_ac) - (assign43390_e48589 * var_qmfact1_ac_dn8)) / (var_qmfact1_ac * var_qmfact1_ac));
        var_cox_qm_dn9 = ((((((var_k1_ac_dn9 * var_csiprime_ac) + (var_k1_ac * var_csiprime_ac_dn9)) * var_areaq_i) * var_qmfact1_ac) - (assign43390_e48589 * var_qmfact1_ac_dn9)) / (var_qmfact1_ac * var_qmfact1_ac));

        let assign43400_e48594: f64 = (1.0 + var_zsat_ac);
        let assign43400_e48596: f64 = (assign43400_e48594 * var_cox_qm);
        var_cgeff = assign43400_e48596;
        var_cgeff_dn4 = ((var_zsat_ac_dn4 * var_cox_qm) + (assign43400_e48594 * var_cox_qm_dn4));
        var_cgeff_dn6 = ((var_zsat_ac_dn6 * var_cox_qm) + (assign43400_e48594 * var_cox_qm_dn6));
        var_cgeff_dn7 = ((var_zsat_ac_dn7 * var_cox_qm) + (assign43400_e48594 * var_cox_qm_dn7));
        var_cgeff_dn8 = ((var_zsat_ac_dn8 * var_cox_qm) + (assign43400_e48594 * var_cox_qm_dn8));
        var_cgeff_dn9 = ((var_zsat_ac_dn9 * var_cox_qm) + (assign43400_e48594 * var_cox_qm_dn9));

        let assign43410_e48601: f64 = (0.25 * var_sigvds);
        let assign43410_e48603: f64 = (assign43410_e48601 * var_sqrt_t2);
        let assign43410_e48604: f64 = (0.5 - assign43410_e48603);
        let assign43410_e48605: f64 = (var_cgeff * assign43410_e48604);
        var_cdgeff = assign43410_e48605;
        var_cdgeff_dn4 = ((var_cgeff_dn4 * assign43410_e48604) + (var_cgeff * (-(assign43410_e48601 * var_sqrt_t2_dn4))));
        var_cdgeff_dn6 = ((var_cgeff_dn6 * assign43410_e48604) + (var_cgeff * (-(assign43410_e48601 * var_sqrt_t2_dn6))));
        var_cdgeff_dn7 = ((var_cgeff_dn7 * assign43410_e48604) + (var_cgeff * (-(assign43410_e48601 * var_sqrt_t2_dn7))));
        var_cdgeff_dn8 = ((var_cgeff_dn8 * assign43410_e48604) + (var_cgeff * (-(assign43410_e48601 * var_sqrt_t2_dn8))));
        var_cdgeff_dn9 = ((var_cgeff_dn9 * assign43410_e48604) + (var_cgeff * (-(assign43410_e48601 * var_sqrt_t2_dn9))));

        let assign43420_e48608: f64 = (var_cgeff - var_cdgeff);
        var_csgeff = assign43420_e48608;
        var_csgeff_dn4 = (var_cgeff_dn4 - var_cdgeff_dn4);
        var_csgeff_dn6 = (var_cgeff_dn6 - var_cdgeff_dn6);
        var_csgeff_dn7 = (var_cgeff_dn7 - var_cdgeff_dn7);
        var_csgeff_dn8 = (var_cgeff_dn8 - var_cdgeff_dn8);
        var_csgeff_dn9 = (var_cgeff_dn9 - var_cdgeff_dn9);

        var_migid = 0.0;
        var_migid_dn4 = 0.0;
        var_migid_dn6 = 0.0;
        var_migid_dn7 = 0.0;
        var_migid_dn8 = 0.0;
        var_migid_dn9 = 0.0;

        let assign43450_e48613: f64 = if p.p6 > 0.0 { 1.0 } else { 0.0 };
        var_guard1279 = assign43450_e48613;

        let (assign43460_e48639, assign43460_e48639_d_n4, assign43460_e48639_d_n6, assign43460_e48639_d_n7, assign43460_e48639_d_n8, assign43460_e48639_d_n9,) = {
    if (var_guard1279 != 0.0) {
        let assign43460_e48617: f64 = (var_t1 / 12.0);
        let assign43460_e48621: f64 = (var_t1 + 0.2);
        let assign43460_e48623: f64 = (assign43460_e48621 - var_t2x12);
        let assign43460_e48624: f64 = (var_t2 * assign43460_e48623);
        let assign43460_e48625: f64 = (assign43460_e48617 - assign43460_e48624);
        let assign43460_e48628: f64 = (1.6 * var_t2);
        let assign43460_e48631: f64 = (var_t1 + 1.0);
        let assign43460_e48633: f64 = (assign43460_e48631 - var_t2x12);
        let assign43460_e48634: f64 = (assign43460_e48628 * assign43460_e48633);
        let assign43460_e48636: f64 = (assign43460_e48634 * var_r);
        let assign43460_e48637: f64 = (assign43460_e48625 - assign43460_e48636);
        (assign43460_e48637, (((var_t1_dn4 / 12.0) - ((var_t2_dn4 * assign43460_e48623) + (var_t2 * (var_t1_dn4 - var_t2x12_dn4)))) - (((((1.6 * var_t2_dn4) * assign43460_e48633) + (assign43460_e48628 * (var_t1_dn4 - var_t2x12_dn4))) * var_r) + (assign43460_e48634 * var_r_dn4))), (((var_t1_dn6 / 12.0) - ((var_t2_dn6 * assign43460_e48623) + (var_t2 * (var_t1_dn6 - var_t2x12_dn6)))) - (((((1.6 * var_t2_dn6) * assign43460_e48633) + (assign43460_e48628 * (var_t1_dn6 - var_t2x12_dn6))) * var_r) + (assign43460_e48634 * var_r_dn6))), (((var_t1_dn7 / 12.0) - ((var_t2_dn7 * assign43460_e48623) + (var_t2 * (var_t1_dn7 - var_t2x12_dn7)))) - (((((1.6 * var_t2_dn7) * assign43460_e48633) + (assign43460_e48628 * (var_t1_dn7 - var_t2x12_dn7))) * var_r) + (assign43460_e48634 * var_r_dn7))), (((var_t1_dn8 / 12.0) - ((var_t2_dn8 * assign43460_e48623) + (var_t2 * (var_t1_dn8 - var_t2x12_dn8)))) - (((((1.6 * var_t2_dn8) * assign43460_e48633) + (assign43460_e48628 * (var_t1_dn8 - var_t2x12_dn8))) * var_r) + (assign43460_e48634 * var_r_dn8))), (((var_t1_dn9 / 12.0) - ((var_t2_dn9 * assign43460_e48623) + (var_t2 * (var_t1_dn9 - var_t2x12_dn9)))) - (((((1.6 * var_t2_dn9) * assign43460_e48633) + (assign43460_e48628 * (var_t1_dn9 - var_t2x12_dn9))) * var_r) + (assign43460_e48634 * var_r_dn9))),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign43460_e48639;
        var_temp1_dn4 = assign43460_e48639_d_n4;
        var_temp1_dn6 = assign43460_e48639_d_n6;
        var_temp1_dn7 = assign43460_e48639_d_n7;
        var_temp1_dn8 = assign43460_e48639_d_n8;
        var_temp1_dn9 = assign43460_e48639_d_n9;

        let (assign43470_e48645, assign43470_e48645_d_n4, assign43470_e48645_d_n6, assign43470_e48645_d_n7, assign43470_e48645_d_n8, assign43470_e48645_d_n9,) = {
    if (var_guard1279 != 0.0) {
        let assign43470_e48643: f64 = (var_temp1).max(1e-40);
        (assign43470_e48643, if var_temp1 >= 1e-40 { var_temp1_dn4 } else { 0.0 }, if var_temp1 >= 1e-40 { var_temp1_dn6 } else { 0.0 }, if var_temp1 >= 1e-40 { var_temp1_dn7 } else { 0.0 }, if var_temp1 >= 1e-40 { var_temp1_dn8 } else { 0.0 }, if var_temp1 >= 1e-40 { var_temp1_dn9 } else { 0.0 },)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign43470_e48645;
        var_temp2_dn4 = assign43470_e48645_d_n4;
        var_temp2_dn6 = assign43470_e48645_d_n6;
        var_temp2_dn7 = assign43470_e48645_d_n7;
        var_temp2_dn8 = assign43470_e48645_d_n8;
        var_temp2_dn9 = assign43470_e48645_d_n9;

        let (assign43480_e48655, assign43480_e48655_d_n4, assign43480_e48655_d_n6, assign43480_e48655_d_n7, assign43480_e48655_d_n8, assign43480_e48655_d_n9,) = {
    if (var_guard1279 != 0.0) {
        let assign43480_e48649: f64 = (var_g_ideal * var_lc);
        let assign43480_e48651: f64 = (assign43480_e48649 * var_lc);
        let assign43480_e48653: f64 = (assign43480_e48651 / var_temp2);
        (assign43480_e48653, (((((((var_g_ideal_dn4 * var_lc) + (var_g_ideal * var_lc_dn4)) * var_lc) + (assign43480_e48649 * var_lc_dn4)) * var_temp2) - (assign43480_e48651 * var_temp2_dn4)) / (var_temp2 * var_temp2)), (((((((var_g_ideal_dn6 * var_lc) + (var_g_ideal * var_lc_dn6)) * var_lc) + (assign43480_e48649 * var_lc_dn6)) * var_temp2) - (assign43480_e48651 * var_temp2_dn6)) / (var_temp2 * var_temp2)), (((((((var_g_ideal_dn7 * var_lc) + (var_g_ideal * var_lc_dn7)) * var_lc) + (assign43480_e48649 * var_lc_dn7)) * var_temp2) - (assign43480_e48651 * var_temp2_dn7)) / (var_temp2 * var_temp2)), (((((((var_g_ideal_dn8 * var_lc) + (var_g_ideal * var_lc_dn8)) * var_lc) + (assign43480_e48649 * var_lc_dn8)) * var_temp2) - (assign43480_e48651 * var_temp2_dn8)) / (var_temp2 * var_temp2)), (((((((var_g_ideal_dn9 * var_lc) + (var_g_ideal * var_lc_dn9)) * var_lc) + (assign43480_e48649 * var_lc_dn9)) * var_temp2) - (assign43480_e48651 * var_temp2_dn9)) / (var_temp2 * var_temp2)),)
    } else {
        (var_gsig, var_gsig_dn4, var_gsig_dn6, var_gsig_dn7, var_gsig_dn8, var_gsig_dn9,)
    }
};
        var_gsig = assign43480_e48655;
        var_gsig_dn4 = assign43480_e48655_d_n4;
        var_gsig_dn6 = assign43480_e48655_d_n6;
        var_gsig_dn7 = assign43480_e48655_d_n7;
        var_gsig_dn8 = assign43480_e48655_d_n8;
        var_gsig_dn9 = assign43480_e48655_d_n9;

        let assign43500_e48668: f64 = if var_gsid > 0.0 { 1.0 } else { 0.0 };
        var_guard1280 = assign43500_e48668;

        let (assign43510_e48692, assign43510_e48692_d_n4, assign43510_e48692_d_n6, assign43510_e48692_d_n7, assign43510_e48692_d_n8, assign43510_e48692_d_n9,) = {
    if ((var_guard1279 != 0.0) && (var_guard1280 != 0.0)) {
        let assign43510_e48674: f64 = (var_lcinv2 * var_sqrt_t2);
        let assign43510_e48677: f64 = (1.0 - var_t2x12);
        let assign43510_e48681: f64 = (19.2 * var_t2);
        let assign43510_e48682: f64 = (var_t1 + assign43510_e48681);
        let assign43510_e48685: f64 = (var_t1 * var_t2x12);
        let assign43510_e48686: f64 = (assign43510_e48682 - assign43510_e48685);
        let assign43510_e48688: f64 = (assign43510_e48686 * var_r);
        let assign43510_e48689: f64 = (assign43510_e48677 - assign43510_e48688);
        let assign43510_e48690: f64 = (assign43510_e48674 * assign43510_e48689);
        (assign43510_e48690, ((((var_lcinv2_dn4 * var_sqrt_t2) + (var_lcinv2 * var_sqrt_t2_dn4)) * assign43510_e48689) + (assign43510_e48674 * ((-var_t2x12_dn4) - ((((var_t1_dn4 + (19.2 * var_t2_dn4)) - ((var_t1_dn4 * var_t2x12) + (var_t1 * var_t2x12_dn4))) * var_r) + (assign43510_e48686 * var_r_dn4))))), ((((var_lcinv2_dn6 * var_sqrt_t2) + (var_lcinv2 * var_sqrt_t2_dn6)) * assign43510_e48689) + (assign43510_e48674 * ((-var_t2x12_dn6) - ((((var_t1_dn6 + (19.2 * var_t2_dn6)) - ((var_t1_dn6 * var_t2x12) + (var_t1 * var_t2x12_dn6))) * var_r) + (assign43510_e48686 * var_r_dn6))))), ((((var_lcinv2_dn7 * var_sqrt_t2) + (var_lcinv2 * var_sqrt_t2_dn7)) * assign43510_e48689) + (assign43510_e48674 * ((-var_t2x12_dn7) - ((((var_t1_dn7 + (19.2 * var_t2_dn7)) - ((var_t1_dn7 * var_t2x12) + (var_t1 * var_t2x12_dn7))) * var_r) + (assign43510_e48686 * var_r_dn7))))), ((((var_lcinv2_dn8 * var_sqrt_t2) + (var_lcinv2 * var_sqrt_t2_dn8)) * assign43510_e48689) + (assign43510_e48674 * ((-var_t2x12_dn8) - ((((var_t1_dn8 + (19.2 * var_t2_dn8)) - ((var_t1_dn8 * var_t2x12) + (var_t1 * var_t2x12_dn8))) * var_r) + (assign43510_e48686 * var_r_dn8))))), ((((var_lcinv2_dn9 * var_sqrt_t2) + (var_lcinv2 * var_sqrt_t2_dn9)) * assign43510_e48689) + (assign43510_e48674 * ((-var_t2x12_dn9) - ((((var_t1_dn9 + (19.2 * var_t2_dn9)) - ((var_t1_dn9 * var_t2x12) + (var_t1 * var_t2x12_dn9))) * var_r) + (assign43510_e48686 * var_r_dn9))))),)
    } else {
        (var_migid, var_migid_dn4, var_migid_dn6, var_migid_dn7, var_migid_dn8, var_migid_dn9,)
    }
};
        var_migid = assign43510_e48692;
        var_migid_dn4 = assign43510_e48692_d_n4;
        var_migid_dn6 = assign43510_e48692_d_n6;
        var_migid_dn7 = assign43510_e48692_d_n7;
        var_migid_dn8 = assign43510_e48692_d_n8;
        var_migid_dn9 = assign43510_e48692_d_n9;

        let (assign43540_e48775, assign43540_e48775_d_n4, assign43540_e48775_d_n6, assign43540_e48775_d_n7, assign43540_e48775_d_n8, assign43540_e48775_d_n9,) = {
    if (var_guard1279 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_gsig, var_gsig_dn4, var_gsig_dn6, var_gsig_dn7, var_gsig_dn8, var_gsig_dn9,)
    }
};
        var_gsig = assign43540_e48775;
        var_gsig_dn4 = assign43540_e48775_d_n4;
        var_gsig_dn6 = assign43540_e48775_d_n6;
        var_gsig_dn7 = assign43540_e48775_d_n7;
        var_gsig_dn8 = assign43540_e48775_d_n8;
        var_gsig_dn9 = assign43540_e48775_d_n9;

        var_nstar = var_nunit;
        var_nstar_dn4 = var_nunit_dn4;
        var_nstar_dn6 = var_nunit_dn6;
        var_nstar_dn7 = var_nunit_dn7;
        var_nstar_dn8 = var_nunit_dn8;
        var_nstar_dn9 = var_nunit_dn9;

        let assign43580_e48790: f64 = (var_qim_dc + 1.0);
        let assign43580_e48791: f64 = (var_nunit * assign43580_e48790);
        var_nmstar = assign43580_e48791;
        var_nmstar_dn4 = ((var_nunit_dn4 * assign43580_e48790) + (var_nunit * var_qim_dc_dn4));
        var_nmstar_dn6 = ((var_nunit_dn6 * assign43580_e48790) + (var_nunit * var_qim_dc_dn6));
        var_nmstar_dn7 = ((var_nunit_dn7 * assign43580_e48790) + (var_nunit * var_qim_dc_dn7));
        var_nmstar_dn8 = ((var_nunit_dn8 * assign43580_e48790) + (var_nunit * var_qim_dc_dn8));
        var_nmstar_dn9 = ((var_nunit_dn9 * assign43580_e48790) + (var_nunit * var_qim_dc_dn9));

        let assign43590_e48795: f64 = (var_qis_dc - var_qid_dc);
        let assign43590_e48796: f64 = (var_nunit * assign43590_e48795);
        var_deltan = assign43590_e48796;
        var_deltan_dn4 = ((var_nunit_dn4 * assign43590_e48795) + (var_nunit * (var_qis_dc_dn4 - var_qid_dc_dn4)));
        var_deltan_dn6 = ((var_nunit_dn6 * assign43590_e48795) + (var_nunit * (var_qis_dc_dn6 - var_qid_dc_dn6)));
        var_deltan_dn7 = ((var_nunit_dn7 * assign43590_e48795) + (var_nunit * (var_qis_dc_dn7 - var_qid_dc_dn7)));
        var_deltan_dn8 = ((var_nunit_dn8 * assign43590_e48795) + (var_nunit * (var_qis_dc_dn8 - var_qid_dc_dn8)));
        var_deltan_dn9 = ((var_nunit_dn9 * assign43590_e48795) + (var_nunit * (var_qis_dc_dn9 - var_qid_dc_dn9)));

        let assign43600_e48800: f64 = (var_nfb_i * var_nstar);
        let assign43600_e48801: f64 = (var_nfa_i - assign43600_e48800);
        let assign43600_e48804: f64 = (var_nfc_i * var_nstar);
        let assign43600_e48806: f64 = (assign43600_e48804 * var_nstar);
        let assign43600_e48807: f64 = (assign43600_e48801 + assign43600_e48806);
        let assign43600_e48811: f64 = (0.5 * var_deltan);
        let assign43600_e48812: f64 = (var_nmstar + assign43600_e48811);
        let assign43600_e48816: f64 = (0.5 * var_deltan);
        let assign43600_e48817: f64 = (var_nmstar - assign43600_e48816);
        let assign43600_e48818: f64 = (assign43600_e48812 / assign43600_e48817);
        let assign43600_e48819: f64 = (assign43600_e48818).ln();
        let assign43600_e48820: f64 = (assign43600_e48807 * assign43600_e48819);
        var_temp1 = assign43600_e48820;
        var_temp1_dn4 = ((((-(var_nfb_i * var_nstar_dn4)) + (((var_nfc_i * var_nstar_dn4) * var_nstar) + (assign43600_e48804 * var_nstar_dn4))) * assign43600_e48819) + (assign43600_e48807 * (((((var_nmstar_dn4 + (0.5 * var_deltan_dn4)) * assign43600_e48817) - (assign43600_e48812 * (var_nmstar_dn4 - (0.5 * var_deltan_dn4)))) / (assign43600_e48817 * assign43600_e48817)) / assign43600_e48818)));
        var_temp1_dn6 = ((((-(var_nfb_i * var_nstar_dn6)) + (((var_nfc_i * var_nstar_dn6) * var_nstar) + (assign43600_e48804 * var_nstar_dn6))) * assign43600_e48819) + (assign43600_e48807 * (((((var_nmstar_dn6 + (0.5 * var_deltan_dn6)) * assign43600_e48817) - (assign43600_e48812 * (var_nmstar_dn6 - (0.5 * var_deltan_dn6)))) / (assign43600_e48817 * assign43600_e48817)) / assign43600_e48818)));
        var_temp1_dn7 = ((((-(var_nfb_i * var_nstar_dn7)) + (((var_nfc_i * var_nstar_dn7) * var_nstar) + (assign43600_e48804 * var_nstar_dn7))) * assign43600_e48819) + (assign43600_e48807 * (((((var_nmstar_dn7 + (0.5 * var_deltan_dn7)) * assign43600_e48817) - (assign43600_e48812 * (var_nmstar_dn7 - (0.5 * var_deltan_dn7)))) / (assign43600_e48817 * assign43600_e48817)) / assign43600_e48818)));
        var_temp1_dn8 = ((((-(var_nfb_i * var_nstar_dn8)) + (((var_nfc_i * var_nstar_dn8) * var_nstar) + (assign43600_e48804 * var_nstar_dn8))) * assign43600_e48819) + (assign43600_e48807 * (((((var_nmstar_dn8 + (0.5 * var_deltan_dn8)) * assign43600_e48817) - (assign43600_e48812 * (var_nmstar_dn8 - (0.5 * var_deltan_dn8)))) / (assign43600_e48817 * assign43600_e48817)) / assign43600_e48818)));
        var_temp1_dn9 = ((((-(var_nfb_i * var_nstar_dn9)) + (((var_nfc_i * var_nstar_dn9) * var_nstar) + (assign43600_e48804 * var_nstar_dn9))) * assign43600_e48819) + (assign43600_e48807 * (((((var_nmstar_dn9 + (0.5 * var_deltan_dn9)) * assign43600_e48817) - (assign43600_e48812 * (var_nmstar_dn9 - (0.5 * var_deltan_dn9)))) / (assign43600_e48817 * assign43600_e48817)) / assign43600_e48818)));

        let assign43610_e48827: f64 = (2.0 * var_nstar);
        let assign43610_e48828: f64 = (var_nmstar - assign43610_e48827);
        let assign43610_e48829: f64 = (var_nfc_i * assign43610_e48828);
        let assign43610_e48830: f64 = (var_nfb_i + assign43610_e48829);
        let assign43610_e48832: f64 = (assign43610_e48830 * var_deltan);
        let assign43610_e48833: f64 = (var_temp1 + assign43610_e48832);
        var_temp2 = assign43610_e48833;
        var_temp2_dn4 = (var_temp1_dn4 + (((var_nfc_i * (var_nmstar_dn4 - (2.0 * var_nstar_dn4))) * var_deltan) + (assign43610_e48830 * var_deltan_dn4)));
        var_temp2_dn6 = (var_temp1_dn6 + (((var_nfc_i * (var_nmstar_dn6 - (2.0 * var_nstar_dn6))) * var_deltan) + (assign43610_e48830 * var_deltan_dn6)));
        var_temp2_dn7 = (var_temp1_dn7 + (((var_nfc_i * (var_nmstar_dn7 - (2.0 * var_nstar_dn7))) * var_deltan) + (assign43610_e48830 * var_deltan_dn7)));
        var_temp2_dn8 = (var_temp1_dn8 + (((var_nfc_i * (var_nmstar_dn8 - (2.0 * var_nstar_dn8))) * var_deltan) + (assign43610_e48830 * var_deltan_dn8)));
        var_temp2_dn9 = (var_temp1_dn9 + (((var_nfc_i * (var_nmstar_dn9 - (2.0 * var_nstar_dn9))) * var_deltan) + (assign43610_e48830 * var_deltan_dn9)));

        let assign43620_e48837: f64 = (var_nfe_i * var_esurf1_dc);
        let assign43620_e48840: f64 = (var_nfeb_i * var_esurf2_dc);
        let assign43620_e48841: f64 = (assign43620_e48837 + assign43620_e48840);
        let assign43620_e48844: f64 = (var_qim_dc + 1.0);
        let assign43620_e48845: f64 = (assign43620_e48841 / assign43620_e48844);
        let assign43620_e48846: f64 = (1.0 + assign43620_e48845);
        var_temp = assign43620_e48846;
        var_temp_dn4 = (((((var_nfe_i * var_esurf1_dc_dn4) + (var_nfeb_i * var_esurf2_dc_dn4)) * assign43620_e48844) - (assign43620_e48841 * var_qim_dc_dn4)) / (assign43620_e48844 * assign43620_e48844));
        var_temp_dn6 = (((((var_nfe_i * var_esurf1_dc_dn6) + (var_nfeb_i * var_esurf2_dc_dn6)) * assign43620_e48844) - (assign43620_e48841 * var_qim_dc_dn6)) / (assign43620_e48844 * assign43620_e48844));
        var_temp_dn7 = (((((var_nfe_i * var_esurf1_dc_dn7) + (var_nfeb_i * var_esurf2_dc_dn7)) * assign43620_e48844) - (assign43620_e48841 * var_qim_dc_dn7)) / (assign43620_e48844 * assign43620_e48844));
        var_temp_dn8 = (((((var_nfe_i * var_esurf1_dc_dn8) + (var_nfeb_i * var_esurf2_dc_dn8)) * assign43620_e48844) - (assign43620_e48841 * var_qim_dc_dn8)) / (assign43620_e48844 * assign43620_e48844));
        var_temp_dn9 = (((((var_nfe_i * var_esurf1_dc_dn9) + (var_nfeb_i * var_esurf2_dc_dn9)) * assign43620_e48844) - (assign43620_e48841 * var_qim_dc_dn9)) / (assign43620_e48844 * assign43620_e48844));

        let assign43630_e48850: f64 = (var_temp + 0.01);
        let assign43630_e48853: f64 = (var_temp - 0.01);
        let assign43630_e48856: f64 = (var_temp - 0.01);
        let assign43630_e48857: f64 = (assign43630_e48853 * assign43630_e48856);
        let assign43630_e48859: f64 = (assign43630_e48857 + 0.0001);
        let assign43630_e48860: f64 = (assign43630_e48859).sqrt();
        let assign43630_e48861: f64 = (assign43630_e48850 + assign43630_e48860);
        let assign43630_e48862: f64 = (0.5 * assign43630_e48861);
        var_temp3 = assign43630_e48862;
        var_temp3_dn4 = (0.5 * (var_temp_dn4 + (((var_temp_dn4 * assign43630_e48856) + (assign43630_e48853 * var_temp_dn4)) / (2.0 * assign43630_e48860))));
        var_temp3_dn6 = (0.5 * (var_temp_dn6 + (((var_temp_dn6 * assign43630_e48856) + (assign43630_e48853 * var_temp_dn6)) / (2.0 * assign43630_e48860))));
        var_temp3_dn7 = (0.5 * (var_temp_dn7 + (((var_temp_dn7 * assign43630_e48856) + (assign43630_e48853 * var_temp_dn7)) / (2.0 * assign43630_e48860))));
        var_temp3_dn8 = (0.5 * (var_temp_dn8 + (((var_temp_dn8 * assign43630_e48856) + (assign43630_e48853 * var_temp_dn8)) / (2.0 * assign43630_e48860))));
        var_temp3_dn9 = (0.5 * (var_temp_dn9 + (((var_temp_dn9 * assign43630_e48856) + (assign43630_e48853 * var_temp_dn9)) / (2.0 * assign43630_e48860))));

        let assign43640_e48865: f64 = (1.602176565e-19 * var_fact_ids);
        let assign43640_e48867: f64 = (assign43640_e48865 * var_ids);
        let assign43640_e48869: f64 = (assign43640_e48867 / var_gvsat);
        let assign43640_e48871: f64 = (assign43640_e48869 * var_temp2);
        let assign43640_e48873: f64 = (assign43640_e48871 / var_nstar);
        let assign43640_e48875: f64 = (assign43640_e48873 * var_temp3);
        var_temp = assign43640_e48875;
        var_temp_dn4 = (((((((((((((1.602176565e-19 * var_fact_ids_dn4) * var_ids) + (assign43640_e48865 * var_ids_dn4)) * var_gvsat) - (assign43640_e48867 * var_gvsat_dn4)) / (var_gvsat * var_gvsat)) * var_temp2) + (assign43640_e48869 * var_temp2_dn4)) * var_nstar) - (assign43640_e48871 * var_nstar_dn4)) / (var_nstar * var_nstar)) * var_temp3) + (assign43640_e48873 * var_temp3_dn4));
        var_temp_dn6 = (((((((((((((1.602176565e-19 * var_fact_ids_dn6) * var_ids) + (assign43640_e48865 * var_ids_dn6)) * var_gvsat) - (assign43640_e48867 * var_gvsat_dn6)) / (var_gvsat * var_gvsat)) * var_temp2) + (assign43640_e48869 * var_temp2_dn6)) * var_nstar) - (assign43640_e48871 * var_nstar_dn6)) / (var_nstar * var_nstar)) * var_temp3) + (assign43640_e48873 * var_temp3_dn6));
        var_temp_dn7 = (((((((((((((1.602176565e-19 * var_fact_ids_dn7) * var_ids) + (assign43640_e48865 * var_ids_dn7)) * var_gvsat) - (assign43640_e48867 * var_gvsat_dn7)) / (var_gvsat * var_gvsat)) * var_temp2) + (assign43640_e48869 * var_temp2_dn7)) * var_nstar) - (assign43640_e48871 * var_nstar_dn7)) / (var_nstar * var_nstar)) * var_temp3) + (assign43640_e48873 * var_temp3_dn7));
        var_temp_dn8 = (((((((((((((1.602176565e-19 * var_fact_ids_dn8) * var_ids) + (assign43640_e48865 * var_ids_dn8)) * var_gvsat) - (assign43640_e48867 * var_gvsat_dn8)) / (var_gvsat * var_gvsat)) * var_temp2) + (assign43640_e48869 * var_temp2_dn8)) * var_nstar) - (assign43640_e48871 * var_nstar_dn8)) / (var_nstar * var_nstar)) * var_temp3) + (assign43640_e48873 * var_temp3_dn8));
        var_temp_dn9 = (((((((((((((1.602176565e-19 * var_fact_ids_dn9) * var_ids) + (assign43640_e48865 * var_ids_dn9)) * var_gvsat) - (assign43640_e48867 * var_gvsat_dn9)) / (var_gvsat * var_gvsat)) * var_temp2) + (assign43640_e48869 * var_temp2_dn9)) * var_nstar) - (assign43640_e48871 * var_nstar_dn9)) / (var_nstar * var_nstar)) * var_temp3) + (assign43640_e48873 * var_temp3_dn9));

        let assign43710_e48935: f64 = (var_tkd * 8.617332384961e-5);
        let assign43710_e48936: f64 = (1.0 / assign43710_e48935);
        var_inv_phit0_op = assign43710_e48936;
        var_inv_phit0_op_dn4 = (-((var_tkd_dn4 * 8.617332384961e-5) / (assign43710_e48935 * assign43710_e48935)));
        var_inv_phit0_op_dn6 = (-((var_tkd_dn6 * 8.617332384961e-5) / (assign43710_e48935 * assign43710_e48935)));
        var_inv_phit0_op_dn7 = (-((var_tkd_dn7 * 8.617332384961e-5) / (assign43710_e48935 * assign43710_e48935)));
        var_inv_phit0_op_dn8 = (-((var_tkd_dn8 * 8.617332384961e-5) / (assign43710_e48935 * assign43710_e48935)));
        var_inv_phit0_op_dn9 = (-((var_tkd_dn9 * 8.617332384961e-5) / (assign43710_e48935 * assign43710_e48935)));

        let assign43720_e48940: f64 = (0.000473 * var_tkd);
        let assign43720_e48942: f64 = (assign43720_e48940 * var_tkd);
        let assign43720_e48945: f64 = (636.0 + var_tkd);
        let assign43720_e48946: f64 = (assign43720_e48942 / assign43720_e48945);
        let assign43720_e48947: f64 = (1.17 - assign43720_e48946);
        var_egsi_op = assign43720_e48947;
        var_egsi_op_dn4 = (-((((((0.000473 * var_tkd_dn4) * var_tkd) + (assign43720_e48940 * var_tkd_dn4)) * assign43720_e48945) - (assign43720_e48942 * var_tkd_dn4)) / (assign43720_e48945 * assign43720_e48945)));
        var_egsi_op_dn6 = (-((((((0.000473 * var_tkd_dn6) * var_tkd) + (assign43720_e48940 * var_tkd_dn6)) * assign43720_e48945) - (assign43720_e48942 * var_tkd_dn6)) / (assign43720_e48945 * assign43720_e48945)));
        var_egsi_op_dn7 = (-((((((0.000473 * var_tkd_dn7) * var_tkd) + (assign43720_e48940 * var_tkd_dn7)) * assign43720_e48945) - (assign43720_e48942 * var_tkd_dn7)) / (assign43720_e48945 * assign43720_e48945)));
        var_egsi_op_dn8 = (-((((((0.000473 * var_tkd_dn8) * var_tkd) + (assign43720_e48940 * var_tkd_dn8)) * assign43720_e48945) - (assign43720_e48942 * var_tkd_dn8)) / (assign43720_e48945 * assign43720_e48945)));
        var_egsi_op_dn9 = (-((((((0.000473 * var_tkd_dn9) * var_tkd) + (assign43720_e48940 * var_tkd_dn9)) * assign43720_e48945) - (assign43720_e48942 * var_tkd_dn9)) / (assign43720_e48945 * assign43720_e48945)));

        let assign43730_e48951: f64 = (0.0004774 * var_tkd);
        let assign43730_e48953: f64 = (assign43730_e48951 * var_tkd);
        let assign43730_e48956: f64 = (235.0 + var_tkd);
        let assign43730_e48957: f64 = (assign43730_e48953 / assign43730_e48956);
        let assign43730_e48958: f64 = (0.744 - assign43730_e48957);
        var_egge_op = assign43730_e48958;
        var_egge_op_dn4 = (-((((((0.0004774 * var_tkd_dn4) * var_tkd) + (assign43730_e48951 * var_tkd_dn4)) * assign43730_e48956) - (assign43730_e48953 * var_tkd_dn4)) / (assign43730_e48956 * assign43730_e48956)));
        var_egge_op_dn6 = (-((((((0.0004774 * var_tkd_dn6) * var_tkd) + (assign43730_e48951 * var_tkd_dn6)) * assign43730_e48956) - (assign43730_e48953 * var_tkd_dn6)) / (assign43730_e48956 * assign43730_e48956)));
        var_egge_op_dn7 = (-((((((0.0004774 * var_tkd_dn7) * var_tkd) + (assign43730_e48951 * var_tkd_dn7)) * assign43730_e48956) - (assign43730_e48953 * var_tkd_dn7)) / (assign43730_e48956 * assign43730_e48956)));
        var_egge_op_dn8 = (-((((((0.0004774 * var_tkd_dn8) * var_tkd) + (assign43730_e48951 * var_tkd_dn8)) * assign43730_e48956) - (assign43730_e48953 * var_tkd_dn8)) / (assign43730_e48956 * assign43730_e48956)));
        var_egge_op_dn9 = (-((((((0.0004774 * var_tkd_dn9) * var_tkd) + (assign43730_e48951 * var_tkd_dn9)) * assign43730_e48956) - (assign43730_e48953 * var_tkd_dn9)) / (assign43730_e48956 * assign43730_e48956)));

        let assign43740_e48961: f64 = (var_egge_op - var_egsi_op);
        let assign43740_e48963: f64 = (-0.4);
        let assign43740_e48965: f64 = (assign43740_e48963 * var_one_m_xge);
        let assign43740_e48966: f64 = (assign43740_e48961 + assign43740_e48965);
        let assign43740_e48968: f64 = (assign43740_e48966 * var_xge_i);
        var_deg_op = assign43740_e48968;
        var_deg_op_dn4 = ((var_egge_op_dn4 - var_egsi_op_dn4) * var_xge_i);
        var_deg_op_dn6 = ((var_egge_op_dn6 - var_egsi_op_dn6) * var_xge_i);
        var_deg_op_dn7 = ((var_egge_op_dn7 - var_egsi_op_dn7) * var_xge_i);
        var_deg_op_dn8 = ((var_egge_op_dn8 - var_egsi_op_dn8) * var_xge_i);
        var_deg_op_dn9 = ((var_egge_op_dn9 - var_egsi_op_dn9) * var_xge_i);

        let assign43750_e48971: f64 = (var_egsi_op + var_deg_op);
        var_eg_op = assign43750_e48971;
        var_eg_op_dn4 = (var_egsi_op_dn4 + var_deg_op_dn4);
        var_eg_op_dn6 = (var_egsi_op_dn6 + var_deg_op_dn6);
        var_eg_op_dn7 = (var_egsi_op_dn7 + var_deg_op_dn7);
        var_eg_op_dn8 = (var_egsi_op_dn8 + var_deg_op_dn8);
        var_eg_op_dn9 = (var_egsi_op_dn9 + var_deg_op_dn9);

        let assign43760_e48974: f64 = (0.5 * var_eg_op);
        let assign43760_e48976: f64 = (assign43760_e48974 * var_inv_phit0_op);
        var_eg_2phit0_op = assign43760_e48976;
        var_eg_2phit0_op_dn4 = (((0.5 * var_eg_op_dn4) * var_inv_phit0_op) + (assign43760_e48974 * var_inv_phit0_op_dn4));
        var_eg_2phit0_op_dn6 = (((0.5 * var_eg_op_dn6) * var_inv_phit0_op) + (assign43760_e48974 * var_inv_phit0_op_dn6));
        var_eg_2phit0_op_dn7 = (((0.5 * var_eg_op_dn7) * var_inv_phit0_op) + (assign43760_e48974 * var_inv_phit0_op_dn7));
        var_eg_2phit0_op_dn8 = (((0.5 * var_eg_op_dn8) * var_inv_phit0_op) + (assign43760_e48974 * var_inv_phit0_op_dn8));
        var_eg_2phit0_op_dn9 = (((0.5 * var_eg_op_dn9) * var_inv_phit0_op) + (assign43760_e48974 * var_inv_phit0_op_dn9));

        let assign43770_e48979: f64 = (0.05 * var_xge_i);
        let assign43770_e48982: f64 = (0.5 * var_deg_op);
        let assign43770_e48983: f64 = (assign43770_e48979 - assign43770_e48982);
        var_dvfbch_op = assign43770_e48983;
        var_dvfbch_op_dn4 = (-(0.5 * var_deg_op_dn4));
        var_dvfbch_op_dn6 = (-(0.5 * var_deg_op_dn6));
        var_dvfbch_op_dn7 = (-(0.5 * var_deg_op_dn7));
        var_dvfbch_op_dn8 = (-(0.5 * var_deg_op_dn8));
        var_dvfbch_op_dn9 = (-(0.5 * var_deg_op_dn9));

        *var_cdgeff_slot = var_cdgeff;
        *var_cdgeff_dn4_slot = var_cdgeff_dn4;
        *var_cdgeff_dn6_slot = var_cdgeff_dn6;
        *var_cdgeff_dn7_slot = var_cdgeff_dn7;
        *var_cdgeff_dn8_slot = var_cdgeff_dn8;
        *var_cdgeff_dn9_slot = var_cdgeff_dn9;
        *var_cgeff_slot = var_cgeff;
        *var_cgeff_dn4_slot = var_cgeff_dn4;
        *var_cgeff_dn6_slot = var_cgeff_dn6;
        *var_cgeff_dn7_slot = var_cgeff_dn7;
        *var_cgeff_dn8_slot = var_cgeff_dn8;
        *var_cgeff_dn9_slot = var_cgeff_dn9;
        *var_cox_qm_slot = var_cox_qm;
        *var_cox_qm_dn4_slot = var_cox_qm_dn4;
        *var_cox_qm_dn6_slot = var_cox_qm_dn6;
        *var_cox_qm_dn7_slot = var_cox_qm_dn7;
        *var_cox_qm_dn8_slot = var_cox_qm_dn8;
        *var_cox_qm_dn9_slot = var_cox_qm_dn9;
        *var_csgeff_slot = var_csgeff;
        *var_csgeff_dn4_slot = var_csgeff_dn4;
        *var_csgeff_dn6_slot = var_csgeff_dn6;
        *var_csgeff_dn7_slot = var_csgeff_dn7;
        *var_csgeff_dn8_slot = var_csgeff_dn8;
        *var_csgeff_dn9_slot = var_csgeff_dn9;
        *var_deg_op_slot = var_deg_op;
        *var_deg_op_dn4_slot = var_deg_op_dn4;
        *var_deg_op_dn6_slot = var_deg_op_dn6;
        *var_deg_op_dn7_slot = var_deg_op_dn7;
        *var_deg_op_dn8_slot = var_deg_op_dn8;
        *var_deg_op_dn9_slot = var_deg_op_dn9;
        *var_deltan_slot = var_deltan;
        *var_deltan_dn4_slot = var_deltan_dn4;
        *var_deltan_dn6_slot = var_deltan_dn6;
        *var_deltan_dn7_slot = var_deltan_dn7;
        *var_deltan_dn8_slot = var_deltan_dn8;
        *var_deltan_dn9_slot = var_deltan_dn9;
        *var_dm_slot = var_dm;
        *var_dm_dn4_slot = var_dm_dn4;
        *var_dm_dn6_slot = var_dm_dn6;
        *var_dm_dn7_slot = var_dm_dn7;
        *var_dm_dn8_slot = var_dm_dn8;
        *var_dm_dn9_slot = var_dm_dn9;
        *var_dvfbch_op_slot = var_dvfbch_op;
        *var_dvfbch_op_dn4_slot = var_dvfbch_op_dn4;
        *var_dvfbch_op_dn6_slot = var_dvfbch_op_dn6;
        *var_dvfbch_op_dn7_slot = var_dvfbch_op_dn7;
        *var_dvfbch_op_dn8_slot = var_dvfbch_op_dn8;
        *var_dvfbch_op_dn9_slot = var_dvfbch_op_dn9;
        *var_eg_2phit0_op_slot = var_eg_2phit0_op;
        *var_eg_2phit0_op_dn4_slot = var_eg_2phit0_op_dn4;
        *var_eg_2phit0_op_dn6_slot = var_eg_2phit0_op_dn6;
        *var_eg_2phit0_op_dn7_slot = var_eg_2phit0_op_dn7;
        *var_eg_2phit0_op_dn8_slot = var_eg_2phit0_op_dn8;
        *var_eg_2phit0_op_dn9_slot = var_eg_2phit0_op_dn9;
        *var_eg_op_slot = var_eg_op;
        *var_eg_op_dn4_slot = var_eg_op_dn4;
        *var_eg_op_dn6_slot = var_eg_op_dn6;
        *var_eg_op_dn7_slot = var_eg_op_dn7;
        *var_eg_op_dn8_slot = var_eg_op_dn8;
        *var_eg_op_dn9_slot = var_eg_op_dn9;
        *var_egge_op_slot = var_egge_op;
        *var_egge_op_dn4_slot = var_egge_op_dn4;
        *var_egge_op_dn6_slot = var_egge_op_dn6;
        *var_egge_op_dn7_slot = var_egge_op_dn7;
        *var_egge_op_dn8_slot = var_egge_op_dn8;
        *var_egge_op_dn9_slot = var_egge_op_dn9;
        *var_egsi_op_slot = var_egsi_op;
        *var_egsi_op_dn4_slot = var_egsi_op_dn4;
        *var_egsi_op_dn6_slot = var_egsi_op_dn6;
        *var_egsi_op_dn7_slot = var_egsi_op_dn7;
        *var_egsi_op_dn8_slot = var_egsi_op_dn8;
        *var_egsi_op_dn9_slot = var_egsi_op_dn9;
        *var_g_ideal_slot = var_g_ideal;
        *var_g_ideal_dn4_slot = var_g_ideal_dn4;
        *var_g_ideal_dn6_slot = var_g_ideal_dn6;
        *var_g_ideal_dn7_slot = var_g_ideal_dn7;
        *var_g_ideal_dn8_slot = var_g_ideal_dn8;
        *var_g_ideal_dn9_slot = var_g_ideal_dn9;
        *var_gsid_slot = var_gsid;
        *var_gsig_slot = var_gsig;
        *var_gsig_dn4_slot = var_gsig_dn4;
        *var_gsig_dn6_slot = var_gsig_dn6;
        *var_gsig_dn7_slot = var_gsig_dn7;
        *var_gsig_dn8_slot = var_gsig_dn8;
        *var_gsig_dn9_slot = var_gsig_dn9;
        *var_guard1278_slot = var_guard1278;
        *var_guard1279_slot = var_guard1279;
        *var_guard1280_slot = var_guard1280;
        *var_inv_phit0_op_slot = var_inv_phit0_op;
        *var_inv_phit0_op_dn4_slot = var_inv_phit0_op_dn4;
        *var_inv_phit0_op_dn6_slot = var_inv_phit0_op_dn6;
        *var_inv_phit0_op_dn7_slot = var_inv_phit0_op_dn7;
        *var_inv_phit0_op_dn8_slot = var_inv_phit0_op_dn8;
        *var_inv_phit0_op_dn9_slot = var_inv_phit0_op_dn9;
        *var_lc_slot = var_lc;
        *var_lc_dn4_slot = var_lc_dn4;
        *var_lc_dn6_slot = var_lc_dn6;
        *var_lc_dn7_slot = var_lc_dn7;
        *var_lc_dn8_slot = var_lc_dn8;
        *var_lc_dn9_slot = var_lc_dn9;
        *var_lcinv2_slot = var_lcinv2;
        *var_lcinv2_dn4_slot = var_lcinv2_dn4;
        *var_lcinv2_dn6_slot = var_lcinv2_dn6;
        *var_lcinv2_dn7_slot = var_lcinv2_dn7;
        *var_lcinv2_dn8_slot = var_lcinv2_dn8;
        *var_lcinv2_dn9_slot = var_lcinv2_dn9;
        *var_migid_slot = var_migid;
        *var_migid_dn4_slot = var_migid_dn4;
        *var_migid_dn6_slot = var_migid_dn6;
        *var_migid_dn7_slot = var_migid_dn7;
        *var_migid_dn8_slot = var_migid_dn8;
        *var_migid_dn9_slot = var_migid_dn9;
        *var_nmstar_slot = var_nmstar;
        *var_nmstar_dn4_slot = var_nmstar_dn4;
        *var_nmstar_dn6_slot = var_nmstar_dn6;
        *var_nmstar_dn7_slot = var_nmstar_dn7;
        *var_nmstar_dn8_slot = var_nmstar_dn8;
        *var_nmstar_dn9_slot = var_nmstar_dn9;
        *var_nstar_slot = var_nstar;
        *var_nstar_dn4_slot = var_nstar_dn4;
        *var_nstar_dn6_slot = var_nstar_dn6;
        *var_nstar_dn7_slot = var_nstar_dn7;
        *var_nstar_dn8_slot = var_nstar_dn8;
        *var_nstar_dn9_slot = var_nstar_dn9;
        *var_nunit_slot = var_nunit;
        *var_nunit_dn4_slot = var_nunit_dn4;
        *var_nunit_dn6_slot = var_nunit_dn6;
        *var_nunit_dn7_slot = var_nunit_dn7;
        *var_nunit_dn8_slot = var_nunit_dn8;
        *var_nunit_dn9_slot = var_nunit_dn9;
        *var_qbsif_slot = var_qbsif;
        *var_qbsif_dn4_slot = var_qbsif_dn4;
        *var_qbsif_dn6_slot = var_qbsif_dn6;
        *var_qbsif_dn7_slot = var_qbsif_dn7;
        *var_qbsif_dn8_slot = var_qbsif_dn8;
        *var_qbsif_dn9_slot = var_qbsif_dn9;
        *var_qimstar_slot = var_qimstar;
        *var_qimstar_dn4_slot = var_qimstar_dn4;
        *var_qimstar_dn6_slot = var_qimstar_dn6;
        *var_qimstar_dn7_slot = var_qimstar_dn7;
        *var_qimstar_dn8_slot = var_qimstar_dn8;
        *var_qimstar_dn9_slot = var_qimstar_dn9;
        *var_r_slot = var_r;
        *var_r_dn4_slot = var_r_dn4;
        *var_r_dn6_slot = var_r_dn6;
        *var_r_dn7_slot = var_r_dn7;
        *var_r_dn8_slot = var_r_dn8;
        *var_r_dn9_slot = var_r_dn9;
        *var_sidexc_slot = var_sidexc;
        *var_sqrt_t2_slot = var_sqrt_t2;
        *var_sqrt_t2_dn4_slot = var_sqrt_t2_dn4;
        *var_sqrt_t2_dn6_slot = var_sqrt_t2_dn6;
        *var_sqrt_t2_dn7_slot = var_sqrt_t2_dn7;
        *var_sqrt_t2_dn8_slot = var_sqrt_t2_dn8;
        *var_sqrt_t2_dn9_slot = var_sqrt_t2_dn9;
        *var_sqrt_zsatexc_slot = var_sqrt_zsatexc;
        *var_t1_slot = var_t1;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t2_slot = var_t2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2x12_slot = var_t2x12;
        *var_t2x12_dn4_slot = var_t2x12_dn4;
        *var_t2x12_dn6_slot = var_t2x12_dn6;
        *var_t2x12_dn7_slot = var_t2x12_dn7;
        *var_t2x12_dn8_slot = var_t2x12_dn8;
        *var_t2x12_dn9_slot = var_t2x12_dn9;
        *var_temp_slot = var_temp;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp3_slot = var_temp3;
        *var_temp3_dn4_slot = var_temp3_dn4;
        *var_temp3_dn6_slot = var_temp3_dn6;
        *var_temp3_dn7_slot = var_temp3_dn7;
        *var_temp3_dn8_slot = var_temp3_dn8;
        *var_temp3_dn9_slot = var_temp3_dn9;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
    }

    pub(super) fn stamp_transient_block_121(
        p: &Parameters,
        var_a0_dc: f64,
        var_a0_dc_dn4: f64,
        var_a0_dc_dn6: f64,
        var_a0_dc_dn7: f64,
        var_a0_dc_dn8: f64,
        var_a0_dc_dn9: f64,
        var_cfd_i: f64,
        var_cic1_i: f64,
        var_cox1prime: f64,
        var_cox2prime: f64,
        var_csiprime_0: f64,
        var_ct_i: f64,
        var_diff_min_dc: f64,
        var_diff_min_dc_dn4: f64,
        var_diff_min_dc_dn6: f64,
        var_diff_min_dc_dn7: f64,
        var_diff_min_dc_dn8: f64,
        var_diff_min_dc_dn9: f64,
        var_dvfb1nch: f64,
        var_dvfb1nch_dn4: f64,
        var_dvfb1nch_dn6: f64,
        var_dvfb1nch_dn7: f64,
        var_dvfb1nch_dn8: f64,
        var_dvfb1nch_dn9: f64,
        var_dvfb2nch: f64,
        var_dvfb2nch_dn4: f64,
        var_dvfb2nch_dn6: f64,
        var_dvfb2nch_dn7: f64,
        var_dvfb2nch_dn8: f64,
        var_dvfb2nch_dn9: f64,
        var_dvfbch_op: f64,
        var_dvfbch_op_dn4: f64,
        var_dvfbch_op_dn6: f64,
        var_dvfbch_op_dn7: f64,
        var_dvfbch_op_dn8: f64,
        var_dvfbch_op_dn9: f64,
        var_dvfbqm: f64,
        var_dxg1_dibl_dc: f64,
        var_dxg1_dibl_dc_dn4: f64,
        var_dxg1_dibl_dc_dn6: f64,
        var_dxg1_dibl_dc_dn7: f64,
        var_dxg1_dibl_dc_dn8: f64,
        var_dxg1_dibl_dc_dn9: f64,
        var_eg_2phit0_op: f64,
        var_eg_2phit0_op_dn4: f64,
        var_eg_2phit0_op_dn6: f64,
        var_eg_2phit0_op_dn7: f64,
        var_eg_2phit0_op_dn8: f64,
        var_eg_2phit0_op_dn9: f64,
        var_emin: f64,
        var_emin_dn4: f64,
        var_emin_dn6: f64,
        var_emin_dn7: f64,
        var_emin_dn8: f64,
        var_emin_dn9: f64,
        var_epsch: f64,
        var_gfsub: f64,
        var_gfsub_dn4: f64,
        var_gfsub_dn6: f64,
        var_gfsub_dn7: f64,
        var_gfsub_dn8: f64,
        var_gfsub_dn9: f64,
        var_inv_phit0_op: f64,
        var_inv_phit0_op_dn4: f64,
        var_inv_phit0_op_dn6: f64,
        var_inv_phit0_op_dn7: f64,
        var_inv_phit0_op_dn8: f64,
        var_inv_phit0_op_dn9: f64,
        var_k1_dc: f64,
        var_k1_dc_dn4: f64,
        var_k1_dc_dn6: f64,
        var_k1_dc_dn7: f64,
        var_k1_dc_dn8: f64,
        var_k1_dc_dn9: f64,
        var_k2_dc: f64,
        var_k2_dc_dn4: f64,
        var_k2_dc_dn6: f64,
        var_k2_dc_dn7: f64,
        var_k2_dc_dn8: f64,
        var_k2_dc_dn9: f64,
        var_keq_1d: f64,
        var_neff_poly: f64,
        var_neff_poly_dn4: f64,
        var_neff_poly_dn6: f64,
        var_neff_poly_dn7: f64,
        var_neff_poly_dn8: f64,
        var_neff_poly_dn9: f64,
        var_niratio: f64,
        var_np_i: f64,
        var_np_i_dn4: f64,
        var_np_i_dn6: f64,
        var_np_i_dn7: f64,
        var_np_i_dn8: f64,
        var_np_i_dn9: f64,
        var_nsddc_i: f64,
        var_phit: f64,
        var_phit_dn4: f64,
        var_phit_dn6: f64,
        var_phit_dn7: f64,
        var_phit_dn8: f64,
        var_phit_dn9: f64,
        var_sce1_dc: f64,
        var_sce1_dc_dn4: f64,
        var_sce1_dc_dn6: f64,
        var_sce1_dc_dn7: f64,
        var_sce1_dc_dn8: f64,
        var_sce1_dc_dn9: f64,
        var_stcf_i: f64,
        var_stcf_i_dn4: f64,
        var_stcf_i_dn6: f64,
        var_stcf_i_dn7: f64,
        var_stcf_i_dn8: f64,
        var_stcf_i_dn9: f64,
        var_stvfb_i: f64,
        var_tkd: f64,
        var_tkd_dn4: f64,
        var_tkd_dn6: f64,
        var_tkd_dn7: f64,
        var_tkd_dn8: f64,
        var_tkd_dn9: f64,
        var_tkr: f64,
        var_tsi_i: f64,
        var_tsisq: f64,
        var_typesub_i: f64,
        var_vds: f64,
        var_vds_dn6: f64,
        var_vds_dn7: f64,
        var_vfb1_i: f64,
        var_vfb1_i_dn4: f64,
        var_vfb1_i_dn6: f64,
        var_vfb1_i_dn7: f64,
        var_vfb1_i_dn8: f64,
        var_vfb1_i_dn9: f64,
        var_vfb1_t: f64,
        var_vfb1_t_dn4: f64,
        var_vfb1_t_dn6: f64,
        var_vfb1_t_dn7: f64,
        var_vfb1_t_dn8: f64,
        var_vfb1_t_dn9: f64,
        var_vfb2_t: f64,
        var_vfb2_t_dn4: f64,
        var_vfb2_t_dn6: f64,
        var_vfb2_t_dn7: f64,
        var_vfb2_t_dn8: f64,
        var_vfb2_t_dn9: f64,
        var_vsb: f64,
        var_vsb_dn6: f64,
        var_vsb_dn7: f64,
        var_vsb_dn8: f64,
        var_xedge_dc: f64,
        var_xedge_dc_dn4: f64,
        var_xedge_dc_dn6: f64,
        var_xedge_dc_dn7: f64,
        var_xedge_dc_dn8: f64,
        var_xedge_dc_dn9: f64,
        var_xg2_dc: f64,
        var_xg2_dc_dn4: f64,
        var_xg2_dc_dn6: f64,
        var_xg2_dc_dn7: f64,
        var_xg2_dc_dn8: f64,
        var_xg2_dc_dn9: f64,
        var_xg2x_dc: f64,
        var_xg2x_dc_dn4: f64,
        var_xg2x_dc_dn6: f64,
        var_xg2x_dc_dn7: f64,
        var_xg2x_dc_dn8: f64,
        var_xg2x_dc_dn9: f64,
        var_a0_csisq_op_slot: &mut f64,
        var_a0_csisq_op_dn4_slot: &mut f64,
        var_a0_csisq_op_dn6_slot: &mut f64,
        var_a0_csisq_op_dn7_slot: &mut f64,
        var_a0_csisq_op_dn8_slot: &mut f64,
        var_a0_csisq_op_dn9_slot: &mut f64,
        var_dvfbpdep_op_slot: &mut f64,
        var_dvfbpdep_op_dn4_slot: &mut f64,
        var_dvfbpdep_op_dn6_slot: &mut f64,
        var_dvfbpdep_op_dn7_slot: &mut f64,
        var_dvfbpdep_op_dn8_slot: &mut f64,
        var_dvfbpdep_op_dn9_slot: &mut f64,
        var_dxdsx_op_slot: &mut f64,
        var_dxdsx_op_dn4_slot: &mut f64,
        var_dxdsx_op_dn6_slot: &mut f64,
        var_dxdsx_op_dn7_slot: &mut f64,
        var_dxdsx_op_dn8_slot: &mut f64,
        var_dxdsx_op_dn9_slot: &mut f64,
        var_e1_op_slot: &mut f64,
        var_e1_op_dn4_slot: &mut f64,
        var_e1_op_dn6_slot: &mut f64,
        var_e1_op_dn7_slot: &mut f64,
        var_e1_op_dn8_slot: &mut f64,
        var_e1_op_dn9_slot: &mut f64,
        var_guard1350_slot: &mut f64,
        var_guard1351_slot: &mut f64,
        var_guard1352_slot: &mut f64,
        var_guard1353_slot: &mut f64,
        var_guard1354_slot: &mut f64,
        var_guard1355_slot: &mut f64,
        var_inv_phit_op_slot: &mut f64,
        var_inv_phit_op_dn4_slot: &mut f64,
        var_inv_phit_op_dn6_slot: &mut f64,
        var_inv_phit_op_dn7_slot: &mut f64,
        var_inv_phit_op_dn8_slot: &mut f64,
        var_inv_phit_op_dn9_slot: &mut f64,
        var_neff_op_slot: &mut f64,
        var_neff_op_dn4_slot: &mut f64,
        var_neff_op_dn6_slot: &mut f64,
        var_neff_op_dn7_slot: &mut f64,
        var_neff_op_dn8_slot: &mut f64,
        var_neff_op_dn9_slot: &mut f64,
        var_qq_op_slot: &mut f64,
        var_qq_op_dn4_slot: &mut f64,
        var_qq_op_dn6_slot: &mut f64,
        var_qq_op_dn7_slot: &mut f64,
        var_qq_op_dn8_slot: &mut f64,
        var_qq_op_dn9_slot: &mut f64,
        var_r1init_op_slot: &mut f64,
        var_r1init_op_dn4_slot: &mut f64,
        var_r1init_op_dn6_slot: &mut f64,
        var_r1init_op_dn7_slot: &mut f64,
        var_r1init_op_dn8_slot: &mut f64,
        var_r1init_op_dn9_slot: &mut f64,
        var_r2init_op_slot: &mut f64,
        var_r2init_op_dn4_slot: &mut f64,
        var_r2init_op_dn6_slot: &mut f64,
        var_r2init_op_dn7_slot: &mut f64,
        var_r2init_op_dn8_slot: &mut f64,
        var_r2init_op_dn9_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_vfb1_op_slot: &mut f64,
        var_vfb1_op_dn4_slot: &mut f64,
        var_vfb1_op_dn6_slot: &mut f64,
        var_vfb1_op_dn7_slot: &mut f64,
        var_vfb1_op_dn8_slot: &mut f64,
        var_vfb1_op_dn9_slot: &mut f64,
        var_vfb2_op_slot: &mut f64,
        var_vfb2_op_dn4_slot: &mut f64,
        var_vfb2_op_dn6_slot: &mut f64,
        var_vfb2_op_dn7_slot: &mut f64,
        var_vfb2_op_dn8_slot: &mut f64,
        var_vfb2_op_dn9_slot: &mut f64,
        var_vthinit_op_slot: &mut f64,
        var_vthinit_op_dn4_slot: &mut f64,
        var_vthinit_op_dn6_slot: &mut f64,
        var_vthinit_op_dn7_slot: &mut f64,
        var_vthinit_op_dn8_slot: &mut f64,
        var_vthinit_op_dn9_slot: &mut f64,
        var_x1init_op_slot: &mut f64,
        var_x1init_op_dn4_slot: &mut f64,
        var_x1init_op_dn6_slot: &mut f64,
        var_x1init_op_dn7_slot: &mut f64,
        var_x1init_op_dn8_slot: &mut f64,
        var_x1init_op_dn9_slot: &mut f64,
        var_x2init_op_slot: &mut f64,
        var_x2init_op_dn4_slot: &mut f64,
        var_x2init_op_dn6_slot: &mut f64,
        var_x2init_op_dn7_slot: &mut f64,
        var_x2init_op_dn8_slot: &mut f64,
        var_x2init_op_dn9_slot: &mut f64,
        var_xd0_op_slot: &mut f64,
        var_xd0_op_dn4_slot: &mut f64,
        var_xd0_op_dn6_slot: &mut f64,
        var_xd0_op_dn7_slot: &mut f64,
        var_xd0_op_dn8_slot: &mut f64,
        var_xd0_op_dn9_slot: &mut f64,
        var_xd_op_slot: &mut f64,
        var_xd_op_dn4_slot: &mut f64,
        var_xd_op_dn6_slot: &mut f64,
        var_xd_op_dn7_slot: &mut f64,
        var_xd_op_dn8_slot: &mut f64,
        var_xd_op_dn9_slot: &mut f64,
        var_xdsx_op_slot: &mut f64,
        var_xdsx_op_dn4_slot: &mut f64,
        var_xdsx_op_dn6_slot: &mut f64,
        var_xdsx_op_dn7_slot: &mut f64,
        var_xdsx_op_dn8_slot: &mut f64,
        var_xdsx_op_dn9_slot: &mut f64,
        var_xg10_op_slot: &mut f64,
        var_xg10_op_dn4_slot: &mut f64,
        var_xg10_op_dn6_slot: &mut f64,
        var_xg10_op_dn7_slot: &mut f64,
        var_xg10_op_dn8_slot: &mut f64,
        var_xg10_op_dn9_slot: &mut f64,
        var_xg1thinit_op_slot: &mut f64,
        var_xg1thinit_op_dn4_slot: &mut f64,
        var_xg1thinit_op_dn6_slot: &mut f64,
        var_xg1thinit_op_dn7_slot: &mut f64,
        var_xg1thinit_op_dn8_slot: &mut f64,
        var_xg1thinit_op_dn9_slot: &mut f64,
        var_xg20_op_slot: &mut f64,
        var_xg20_op_dn4_slot: &mut f64,
        var_xg20_op_dn6_slot: &mut f64,
        var_xg20_op_dn7_slot: &mut f64,
        var_xg20_op_dn8_slot: &mut f64,
        var_xg20_op_dn9_slot: &mut f64,
        var_xg2eff_op_slot: &mut f64,
        var_xg2eff_op_dn4_slot: &mut f64,
        var_xg2eff_op_dn6_slot: &mut f64,
        var_xg2eff_op_dn7_slot: &mut f64,
        var_xg2eff_op_dn8_slot: &mut f64,
        var_xg2eff_op_dn9_slot: &mut f64,
        var_xsddep_op_slot: &mut f64,
        var_xsddep_op_dn4_slot: &mut f64,
        var_xsddep_op_dn6_slot: &mut f64,
        var_xsddep_op_dn7_slot: &mut f64,
        var_xsddep_op_dn8_slot: &mut f64,
        var_xsddep_op_dn9_slot: &mut f64,
        var_xth1init_op_slot: &mut f64,
        var_xth1init_op_dn4_slot: &mut f64,
        var_xth1init_op_dn6_slot: &mut f64,
        var_xth1init_op_dn7_slot: &mut f64,
        var_xth1init_op_dn8_slot: &mut f64,
        var_xth1init_op_dn9_slot: &mut f64,
        var_xth2init_op_slot: &mut f64,
        var_xth2init_op_dn4_slot: &mut f64,
        var_xth2init_op_dn6_slot: &mut f64,
        var_xth2init_op_dn7_slot: &mut f64,
        var_xth2init_op_dn8_slot: &mut f64,
        var_xth2init_op_dn9_slot: &mut f64,
        var_xth_1d_op_slot: &mut f64,
        var_xth_1d_op_dn4_slot: &mut f64,
        var_xth_1d_op_dn6_slot: &mut f64,
        var_xth_1d_op_dn7_slot: &mut f64,
        var_xth_1d_op_dn8_slot: &mut f64,
        var_xth_1d_op_dn9_slot: &mut f64,
    ) {
        let mut var_a0_csisq_op: f64 = *var_a0_csisq_op_slot;
        let mut var_a0_csisq_op_dn4: f64 = *var_a0_csisq_op_dn4_slot;
        let mut var_a0_csisq_op_dn6: f64 = *var_a0_csisq_op_dn6_slot;
        let mut var_a0_csisq_op_dn7: f64 = *var_a0_csisq_op_dn7_slot;
        let mut var_a0_csisq_op_dn8: f64 = *var_a0_csisq_op_dn8_slot;
        let mut var_a0_csisq_op_dn9: f64 = *var_a0_csisq_op_dn9_slot;
        let mut var_dvfbpdep_op: f64 = *var_dvfbpdep_op_slot;
        let mut var_dvfbpdep_op_dn4: f64 = *var_dvfbpdep_op_dn4_slot;
        let mut var_dvfbpdep_op_dn6: f64 = *var_dvfbpdep_op_dn6_slot;
        let mut var_dvfbpdep_op_dn7: f64 = *var_dvfbpdep_op_dn7_slot;
        let mut var_dvfbpdep_op_dn8: f64 = *var_dvfbpdep_op_dn8_slot;
        let mut var_dvfbpdep_op_dn9: f64 = *var_dvfbpdep_op_dn9_slot;
        let mut var_dxdsx_op: f64 = *var_dxdsx_op_slot;
        let mut var_dxdsx_op_dn4: f64 = *var_dxdsx_op_dn4_slot;
        let mut var_dxdsx_op_dn6: f64 = *var_dxdsx_op_dn6_slot;
        let mut var_dxdsx_op_dn7: f64 = *var_dxdsx_op_dn7_slot;
        let mut var_dxdsx_op_dn8: f64 = *var_dxdsx_op_dn8_slot;
        let mut var_dxdsx_op_dn9: f64 = *var_dxdsx_op_dn9_slot;
        let mut var_e1_op: f64 = *var_e1_op_slot;
        let mut var_e1_op_dn4: f64 = *var_e1_op_dn4_slot;
        let mut var_e1_op_dn6: f64 = *var_e1_op_dn6_slot;
        let mut var_e1_op_dn7: f64 = *var_e1_op_dn7_slot;
        let mut var_e1_op_dn8: f64 = *var_e1_op_dn8_slot;
        let mut var_e1_op_dn9: f64 = *var_e1_op_dn9_slot;
        let mut var_guard1350: f64 = *var_guard1350_slot;
        let mut var_guard1351: f64 = *var_guard1351_slot;
        let mut var_guard1352: f64 = *var_guard1352_slot;
        let mut var_guard1353: f64 = *var_guard1353_slot;
        let mut var_guard1354: f64 = *var_guard1354_slot;
        let mut var_guard1355: f64 = *var_guard1355_slot;
        let mut var_inv_phit_op: f64 = *var_inv_phit_op_slot;
        let mut var_inv_phit_op_dn4: f64 = *var_inv_phit_op_dn4_slot;
        let mut var_inv_phit_op_dn6: f64 = *var_inv_phit_op_dn6_slot;
        let mut var_inv_phit_op_dn7: f64 = *var_inv_phit_op_dn7_slot;
        let mut var_inv_phit_op_dn8: f64 = *var_inv_phit_op_dn8_slot;
        let mut var_inv_phit_op_dn9: f64 = *var_inv_phit_op_dn9_slot;
        let mut var_neff_op: f64 = *var_neff_op_slot;
        let mut var_neff_op_dn4: f64 = *var_neff_op_dn4_slot;
        let mut var_neff_op_dn6: f64 = *var_neff_op_dn6_slot;
        let mut var_neff_op_dn7: f64 = *var_neff_op_dn7_slot;
        let mut var_neff_op_dn8: f64 = *var_neff_op_dn8_slot;
        let mut var_neff_op_dn9: f64 = *var_neff_op_dn9_slot;
        let mut var_qq_op: f64 = *var_qq_op_slot;
        let mut var_qq_op_dn4: f64 = *var_qq_op_dn4_slot;
        let mut var_qq_op_dn6: f64 = *var_qq_op_dn6_slot;
        let mut var_qq_op_dn7: f64 = *var_qq_op_dn7_slot;
        let mut var_qq_op_dn8: f64 = *var_qq_op_dn8_slot;
        let mut var_qq_op_dn9: f64 = *var_qq_op_dn9_slot;
        let mut var_r1init_op: f64 = *var_r1init_op_slot;
        let mut var_r1init_op_dn4: f64 = *var_r1init_op_dn4_slot;
        let mut var_r1init_op_dn6: f64 = *var_r1init_op_dn6_slot;
        let mut var_r1init_op_dn7: f64 = *var_r1init_op_dn7_slot;
        let mut var_r1init_op_dn8: f64 = *var_r1init_op_dn8_slot;
        let mut var_r1init_op_dn9: f64 = *var_r1init_op_dn9_slot;
        let mut var_r2init_op: f64 = *var_r2init_op_slot;
        let mut var_r2init_op_dn4: f64 = *var_r2init_op_dn4_slot;
        let mut var_r2init_op_dn6: f64 = *var_r2init_op_dn6_slot;
        let mut var_r2init_op_dn7: f64 = *var_r2init_op_dn7_slot;
        let mut var_r2init_op_dn8: f64 = *var_r2init_op_dn8_slot;
        let mut var_r2init_op_dn9: f64 = *var_r2init_op_dn9_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_vfb1_op: f64 = *var_vfb1_op_slot;
        let mut var_vfb1_op_dn4: f64 = *var_vfb1_op_dn4_slot;
        let mut var_vfb1_op_dn6: f64 = *var_vfb1_op_dn6_slot;
        let mut var_vfb1_op_dn7: f64 = *var_vfb1_op_dn7_slot;
        let mut var_vfb1_op_dn8: f64 = *var_vfb1_op_dn8_slot;
        let mut var_vfb1_op_dn9: f64 = *var_vfb1_op_dn9_slot;
        let mut var_vfb2_op: f64 = *var_vfb2_op_slot;
        let mut var_vfb2_op_dn4: f64 = *var_vfb2_op_dn4_slot;
        let mut var_vfb2_op_dn6: f64 = *var_vfb2_op_dn6_slot;
        let mut var_vfb2_op_dn7: f64 = *var_vfb2_op_dn7_slot;
        let mut var_vfb2_op_dn8: f64 = *var_vfb2_op_dn8_slot;
        let mut var_vfb2_op_dn9: f64 = *var_vfb2_op_dn9_slot;
        let mut var_vthinit_op: f64 = *var_vthinit_op_slot;
        let mut var_vthinit_op_dn4: f64 = *var_vthinit_op_dn4_slot;
        let mut var_vthinit_op_dn6: f64 = *var_vthinit_op_dn6_slot;
        let mut var_vthinit_op_dn7: f64 = *var_vthinit_op_dn7_slot;
        let mut var_vthinit_op_dn8: f64 = *var_vthinit_op_dn8_slot;
        let mut var_vthinit_op_dn9: f64 = *var_vthinit_op_dn9_slot;
        let mut var_x1init_op: f64 = *var_x1init_op_slot;
        let mut var_x1init_op_dn4: f64 = *var_x1init_op_dn4_slot;
        let mut var_x1init_op_dn6: f64 = *var_x1init_op_dn6_slot;
        let mut var_x1init_op_dn7: f64 = *var_x1init_op_dn7_slot;
        let mut var_x1init_op_dn8: f64 = *var_x1init_op_dn8_slot;
        let mut var_x1init_op_dn9: f64 = *var_x1init_op_dn9_slot;
        let mut var_x2init_op: f64 = *var_x2init_op_slot;
        let mut var_x2init_op_dn4: f64 = *var_x2init_op_dn4_slot;
        let mut var_x2init_op_dn6: f64 = *var_x2init_op_dn6_slot;
        let mut var_x2init_op_dn7: f64 = *var_x2init_op_dn7_slot;
        let mut var_x2init_op_dn8: f64 = *var_x2init_op_dn8_slot;
        let mut var_x2init_op_dn9: f64 = *var_x2init_op_dn9_slot;
        let mut var_xd0_op: f64 = *var_xd0_op_slot;
        let mut var_xd0_op_dn4: f64 = *var_xd0_op_dn4_slot;
        let mut var_xd0_op_dn6: f64 = *var_xd0_op_dn6_slot;
        let mut var_xd0_op_dn7: f64 = *var_xd0_op_dn7_slot;
        let mut var_xd0_op_dn8: f64 = *var_xd0_op_dn8_slot;
        let mut var_xd0_op_dn9: f64 = *var_xd0_op_dn9_slot;
        let mut var_xd_op: f64 = *var_xd_op_slot;
        let mut var_xd_op_dn4: f64 = *var_xd_op_dn4_slot;
        let mut var_xd_op_dn6: f64 = *var_xd_op_dn6_slot;
        let mut var_xd_op_dn7: f64 = *var_xd_op_dn7_slot;
        let mut var_xd_op_dn8: f64 = *var_xd_op_dn8_slot;
        let mut var_xd_op_dn9: f64 = *var_xd_op_dn9_slot;
        let mut var_xdsx_op: f64 = *var_xdsx_op_slot;
        let mut var_xdsx_op_dn4: f64 = *var_xdsx_op_dn4_slot;
        let mut var_xdsx_op_dn6: f64 = *var_xdsx_op_dn6_slot;
        let mut var_xdsx_op_dn7: f64 = *var_xdsx_op_dn7_slot;
        let mut var_xdsx_op_dn8: f64 = *var_xdsx_op_dn8_slot;
        let mut var_xdsx_op_dn9: f64 = *var_xdsx_op_dn9_slot;
        let mut var_xg10_op: f64 = *var_xg10_op_slot;
        let mut var_xg10_op_dn4: f64 = *var_xg10_op_dn4_slot;
        let mut var_xg10_op_dn6: f64 = *var_xg10_op_dn6_slot;
        let mut var_xg10_op_dn7: f64 = *var_xg10_op_dn7_slot;
        let mut var_xg10_op_dn8: f64 = *var_xg10_op_dn8_slot;
        let mut var_xg10_op_dn9: f64 = *var_xg10_op_dn9_slot;
        let mut var_xg1thinit_op: f64 = *var_xg1thinit_op_slot;
        let mut var_xg1thinit_op_dn4: f64 = *var_xg1thinit_op_dn4_slot;
        let mut var_xg1thinit_op_dn6: f64 = *var_xg1thinit_op_dn6_slot;
        let mut var_xg1thinit_op_dn7: f64 = *var_xg1thinit_op_dn7_slot;
        let mut var_xg1thinit_op_dn8: f64 = *var_xg1thinit_op_dn8_slot;
        let mut var_xg1thinit_op_dn9: f64 = *var_xg1thinit_op_dn9_slot;
        let mut var_xg20_op: f64 = *var_xg20_op_slot;
        let mut var_xg20_op_dn4: f64 = *var_xg20_op_dn4_slot;
        let mut var_xg20_op_dn6: f64 = *var_xg20_op_dn6_slot;
        let mut var_xg20_op_dn7: f64 = *var_xg20_op_dn7_slot;
        let mut var_xg20_op_dn8: f64 = *var_xg20_op_dn8_slot;
        let mut var_xg20_op_dn9: f64 = *var_xg20_op_dn9_slot;
        let mut var_xg2eff_op: f64 = *var_xg2eff_op_slot;
        let mut var_xg2eff_op_dn4: f64 = *var_xg2eff_op_dn4_slot;
        let mut var_xg2eff_op_dn6: f64 = *var_xg2eff_op_dn6_slot;
        let mut var_xg2eff_op_dn7: f64 = *var_xg2eff_op_dn7_slot;
        let mut var_xg2eff_op_dn8: f64 = *var_xg2eff_op_dn8_slot;
        let mut var_xg2eff_op_dn9: f64 = *var_xg2eff_op_dn9_slot;
        let mut var_xsddep_op: f64 = *var_xsddep_op_slot;
        let mut var_xsddep_op_dn4: f64 = *var_xsddep_op_dn4_slot;
        let mut var_xsddep_op_dn6: f64 = *var_xsddep_op_dn6_slot;
        let mut var_xsddep_op_dn7: f64 = *var_xsddep_op_dn7_slot;
        let mut var_xsddep_op_dn8: f64 = *var_xsddep_op_dn8_slot;
        let mut var_xsddep_op_dn9: f64 = *var_xsddep_op_dn9_slot;
        let mut var_xth1init_op: f64 = *var_xth1init_op_slot;
        let mut var_xth1init_op_dn4: f64 = *var_xth1init_op_dn4_slot;
        let mut var_xth1init_op_dn6: f64 = *var_xth1init_op_dn6_slot;
        let mut var_xth1init_op_dn7: f64 = *var_xth1init_op_dn7_slot;
        let mut var_xth1init_op_dn8: f64 = *var_xth1init_op_dn8_slot;
        let mut var_xth1init_op_dn9: f64 = *var_xth1init_op_dn9_slot;
        let mut var_xth2init_op: f64 = *var_xth2init_op_slot;
        let mut var_xth2init_op_dn4: f64 = *var_xth2init_op_dn4_slot;
        let mut var_xth2init_op_dn6: f64 = *var_xth2init_op_dn6_slot;
        let mut var_xth2init_op_dn7: f64 = *var_xth2init_op_dn7_slot;
        let mut var_xth2init_op_dn8: f64 = *var_xth2init_op_dn8_slot;
        let mut var_xth2init_op_dn9: f64 = *var_xth2init_op_dn9_slot;
        let mut var_xth_1d_op: f64 = *var_xth_1d_op_slot;
        let mut var_xth_1d_op_dn4: f64 = *var_xth_1d_op_dn4_slot;
        let mut var_xth_1d_op_dn6: f64 = *var_xth_1d_op_dn6_slot;
        let mut var_xth_1d_op_dn7: f64 = *var_xth_1d_op_dn7_slot;
        let mut var_xth_1d_op_dn8: f64 = *var_xth_1d_op_dn8_slot;
        let mut var_xth_1d_op_dn9: f64 = *var_xth_1d_op_dn9_slot;

        let assign43780_e48986: f64 = (var_tkd * 0.0033333333333);
        let assign43780_e48987: f64 = (assign43780_e48986).sqrt();
        var_temp = assign43780_e48987;
        var_temp_dn4 = ((var_tkd_dn4 * 0.0033333333333) / (2.0 * assign43780_e48987));
        var_temp_dn6 = ((var_tkd_dn6 * 0.0033333333333) / (2.0 * assign43780_e48987));
        var_temp_dn7 = ((var_tkd_dn7 * 0.0033333333333) / (2.0 * assign43780_e48987));
        var_temp_dn8 = ((var_tkd_dn8 * 0.0033333333333) / (2.0 * assign43780_e48987));
        var_temp_dn9 = ((var_tkd_dn9 * 0.0033333333333) / (2.0 * assign43780_e48987));

        let assign43790_e48990: f64 = (4.05e25 * var_temp);
        let assign43790_e48992: f64 = (assign43790_e48990 * var_temp);
        let assign43790_e48994: f64 = (assign43790_e48992 * var_temp);
        var_temp1 = assign43790_e48994;
        var_temp1_dn4 = (((((4.05e25 * var_temp_dn4) * var_temp) + (assign43790_e48990 * var_temp_dn4)) * var_temp) + (assign43790_e48992 * var_temp_dn4));
        var_temp1_dn6 = (((((4.05e25 * var_temp_dn6) * var_temp) + (assign43790_e48990 * var_temp_dn6)) * var_temp) + (assign43790_e48992 * var_temp_dn6));
        var_temp1_dn7 = (((((4.05e25 * var_temp_dn7) * var_temp) + (assign43790_e48990 * var_temp_dn7)) * var_temp) + (assign43790_e48992 * var_temp_dn7));
        var_temp1_dn8 = (((((4.05e25 * var_temp_dn8) * var_temp) + (assign43790_e48990 * var_temp_dn8)) * var_temp) + (assign43790_e48992 * var_temp_dn8));
        var_temp1_dn9 = (((((4.05e25 * var_temp_dn9) * var_temp) + (assign43790_e48990 * var_temp_dn9)) * var_temp) + (assign43790_e48992 * var_temp_dn9));

        let assign43800_e48997: f64 = (var_temp1 * var_niratio);
        var_neff_op = assign43800_e48997;
        var_neff_op_dn4 = (var_temp1_dn4 * var_niratio);
        var_neff_op_dn6 = (var_temp1_dn6 * var_niratio);
        var_neff_op_dn7 = (var_temp1_dn7 * var_niratio);
        var_neff_op_dn8 = (var_temp1_dn8 * var_niratio);
        var_neff_op_dn9 = (var_temp1_dn9 * var_niratio);

        let assign43810_e49002: f64 = (var_ct_i * var_tkr);
        let assign43810_e49004: f64 = (assign43810_e49002 / var_tkd);
        let assign43810_e49005: f64 = (1.0 + assign43810_e49004);
        let assign43810_e49006: f64 = (var_inv_phit0_op / assign43810_e49005);
        var_inv_phit_op = assign43810_e49006;
        var_inv_phit_op_dn4 = (((var_inv_phit0_op_dn4 * assign43810_e49005) - (var_inv_phit0_op * (-((assign43810_e49002 * var_tkd_dn4) / (var_tkd * var_tkd))))) / (assign43810_e49005 * assign43810_e49005));
        var_inv_phit_op_dn6 = (((var_inv_phit0_op_dn6 * assign43810_e49005) - (var_inv_phit0_op * (-((assign43810_e49002 * var_tkd_dn6) / (var_tkd * var_tkd))))) / (assign43810_e49005 * assign43810_e49005));
        var_inv_phit_op_dn7 = (((var_inv_phit0_op_dn7 * assign43810_e49005) - (var_inv_phit0_op * (-((assign43810_e49002 * var_tkd_dn7) / (var_tkd * var_tkd))))) / (assign43810_e49005 * assign43810_e49005));
        var_inv_phit_op_dn8 = (((var_inv_phit0_op_dn8 * assign43810_e49005) - (var_inv_phit0_op * (-((assign43810_e49002 * var_tkd_dn8) / (var_tkd * var_tkd))))) / (assign43810_e49005 * assign43810_e49005));
        var_inv_phit_op_dn9 = (((var_inv_phit0_op_dn9 * assign43810_e49005) - (var_inv_phit0_op * (-((assign43810_e49002 * var_tkd_dn9) / (var_tkd * var_tkd))))) / (assign43810_e49005 * assign43810_e49005));

        let assign43820_e49009: f64 = (2.0 * 1.602176565e-19);
        let assign43820_e49011: f64 = (assign43820_e49009 * var_neff_op);
        let assign43820_e49013: f64 = (assign43820_e49011 * var_epsch);
        let assign43820_e49015: f64 = (assign43820_e49013 * var_inv_phit_op);
        var_a0_csisq_op = assign43820_e49015;
        var_a0_csisq_op_dn4 = ((((assign43820_e49009 * var_neff_op_dn4) * var_epsch) * var_inv_phit_op) + (assign43820_e49013 * var_inv_phit_op_dn4));
        var_a0_csisq_op_dn6 = ((((assign43820_e49009 * var_neff_op_dn6) * var_epsch) * var_inv_phit_op) + (assign43820_e49013 * var_inv_phit_op_dn6));
        var_a0_csisq_op_dn7 = ((((assign43820_e49009 * var_neff_op_dn7) * var_epsch) * var_inv_phit_op) + (assign43820_e49013 * var_inv_phit_op_dn7));
        var_a0_csisq_op_dn8 = ((((assign43820_e49009 * var_neff_op_dn8) * var_epsch) * var_inv_phit_op) + (assign43820_e49013 * var_inv_phit_op_dn8));
        var_a0_csisq_op_dn9 = ((((assign43820_e49009 * var_neff_op_dn9) * var_epsch) * var_inv_phit_op) + (assign43820_e49013 * var_inv_phit_op_dn9));

        let assign43830_e49018: f64 = (var_csiprime_0 * var_csiprime_0);
        let assign43830_e49020: f64 = (assign43830_e49018 / var_a0_csisq_op);
        let assign43830_e49021: f64 = (assign43830_e49020).ln();
        let assign43830_e49023: f64 = (assign43830_e49021 - 0.6931471805599);
        let assign43830_e49025: f64 = (assign43830_e49023 + var_eg_2phit0_op);
        var_xth_1d_op = assign43830_e49025;
        var_xth_1d_op_dn4 = (((-((assign43830_e49018 * var_a0_csisq_op_dn4) / (var_a0_csisq_op * var_a0_csisq_op))) / assign43830_e49020) + var_eg_2phit0_op_dn4);
        var_xth_1d_op_dn6 = (((-((assign43830_e49018 * var_a0_csisq_op_dn6) / (var_a0_csisq_op * var_a0_csisq_op))) / assign43830_e49020) + var_eg_2phit0_op_dn6);
        var_xth_1d_op_dn7 = (((-((assign43830_e49018 * var_a0_csisq_op_dn7) / (var_a0_csisq_op * var_a0_csisq_op))) / assign43830_e49020) + var_eg_2phit0_op_dn7);
        var_xth_1d_op_dn8 = (((-((assign43830_e49018 * var_a0_csisq_op_dn8) / (var_a0_csisq_op * var_a0_csisq_op))) / assign43830_e49020) + var_eg_2phit0_op_dn8);
        var_xth_1d_op_dn9 = (((-((assign43830_e49018 * var_a0_csisq_op_dn9) / (var_a0_csisq_op * var_a0_csisq_op))) / assign43830_e49020) + var_eg_2phit0_op_dn9);

        let assign43840_e49028: f64 = (0.5 * 1.602176565e-19);
        let assign43840_e49030: f64 = (assign43840_e49028 * var_nsddc_i);
        let assign43840_e49032: f64 = (assign43840_e49030 * var_tsi_i);
        let assign43840_e49035: f64 = (var_cox1prime + var_cox2prime);
        let assign43840_e49036: f64 = (assign43840_e49032 / assign43840_e49035);
        let assign43840_e49038: f64 = (assign43840_e49036 * var_inv_phit_op);
        var_xsddep_op = assign43840_e49038;
        var_xsddep_op_dn4 = (assign43840_e49036 * var_inv_phit_op_dn4);
        var_xsddep_op_dn6 = (assign43840_e49036 * var_inv_phit_op_dn6);
        var_xsddep_op_dn7 = (assign43840_e49036 * var_inv_phit_op_dn7);
        var_xsddep_op_dn8 = (assign43840_e49036 * var_inv_phit_op_dn8);
        var_xsddep_op_dn9 = (assign43840_e49036 * var_inv_phit_op_dn9);

        let assign43850_e49041: f64 = (var_cfd_i * var_inv_phit_op);
        var_xd0_op = assign43850_e49041;
        var_xd0_op_dn4 = (var_cfd_i * var_inv_phit_op_dn4);
        var_xd0_op_dn6 = (var_cfd_i * var_inv_phit_op_dn6);
        var_xd0_op_dn7 = (var_cfd_i * var_inv_phit_op_dn7);
        var_xd0_op_dn8 = (var_cfd_i * var_inv_phit_op_dn8);
        var_xd0_op_dn9 = (var_cfd_i * var_inv_phit_op_dn9);

        var_qq_op = 0.0;
        var_qq_op_dn4 = 0.0;
        var_qq_op_dn6 = 0.0;
        var_qq_op_dn7 = 0.0;
        var_qq_op_dn8 = 0.0;
        var_qq_op_dn9 = 0.0;

        var_dvfbpdep_op = 0.0;
        var_dvfbpdep_op_dn4 = 0.0;
        var_dvfbpdep_op_dn6 = 0.0;
        var_dvfbpdep_op_dn7 = 0.0;
        var_dvfbpdep_op_dn8 = 0.0;
        var_dvfbpdep_op_dn9 = 0.0;

        let assign43880_e49046: f64 = if p.p9 > 0.0 { 1.0 } else { 0.0 };
        var_guard1350 = assign43880_e49046;

        let (assign43890_e49057, assign43890_e49057_d_n4, assign43890_e49057_d_n6, assign43890_e49057_d_n7, assign43890_e49057_d_n8, assign43890_e49057_d_n9,) = {
    if (var_guard1350 != 0.0) {
        let assign43890_e49050: f64 = (1.0 / var_inv_phit0_op);
        let assign43890_e49053: f64 = (var_np_i / var_neff_poly);
        let assign43890_e49054: f64 = (assign43890_e49053).ln();
        let assign43890_e49055: f64 = (assign43890_e49050 * assign43890_e49054);
        (assign43890_e49055, (((-(var_inv_phit0_op_dn4 / (var_inv_phit0_op * var_inv_phit0_op))) * assign43890_e49054) + (assign43890_e49050 * ((((var_np_i_dn4 * var_neff_poly) - (var_np_i * var_neff_poly_dn4)) / (var_neff_poly * var_neff_poly)) / assign43890_e49053))), (((-(var_inv_phit0_op_dn6 / (var_inv_phit0_op * var_inv_phit0_op))) * assign43890_e49054) + (assign43890_e49050 * ((((var_np_i_dn6 * var_neff_poly) - (var_np_i * var_neff_poly_dn6)) / (var_neff_poly * var_neff_poly)) / assign43890_e49053))), (((-(var_inv_phit0_op_dn7 / (var_inv_phit0_op * var_inv_phit0_op))) * assign43890_e49054) + (assign43890_e49050 * ((((var_np_i_dn7 * var_neff_poly) - (var_np_i * var_neff_poly_dn7)) / (var_neff_poly * var_neff_poly)) / assign43890_e49053))), (((-(var_inv_phit0_op_dn8 / (var_inv_phit0_op * var_inv_phit0_op))) * assign43890_e49054) + (assign43890_e49050 * ((((var_np_i_dn8 * var_neff_poly) - (var_np_i * var_neff_poly_dn8)) / (var_neff_poly * var_neff_poly)) / assign43890_e49053))), (((-(var_inv_phit0_op_dn9 / (var_inv_phit0_op * var_inv_phit0_op))) * assign43890_e49054) + (assign43890_e49050 * ((((var_np_i_dn9 * var_neff_poly) - (var_np_i * var_neff_poly_dn9)) / (var_neff_poly * var_neff_poly)) / assign43890_e49053))),)
    } else {
        (var_dvfbpdep_op, var_dvfbpdep_op_dn4, var_dvfbpdep_op_dn6, var_dvfbpdep_op_dn7, var_dvfbpdep_op_dn8, var_dvfbpdep_op_dn9,)
    }
};
        var_dvfbpdep_op = assign43890_e49057;
        var_dvfbpdep_op_dn4 = assign43890_e49057_d_n4;
        var_dvfbpdep_op_dn6 = assign43890_e49057_d_n6;
        var_dvfbpdep_op_dn7 = assign43890_e49057_d_n7;
        var_dvfbpdep_op_dn8 = assign43890_e49057_d_n8;
        var_dvfbpdep_op_dn9 = assign43890_e49057_d_n9;

        let assign43900_e49060: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        var_guard1351 = assign43900_e49060;

        let assign43910_e49063: f64 = 1.0;
        let assign43910_e49064: f64 = if p.p14 == assign43910_e49063 { 1.0 } else { 0.0 };
        var_guard1352 = assign43910_e49064;

        let (assign43920_e49083, assign43920_e49083_d_n4, assign43920_e49083_d_n6, assign43920_e49083_d_n7, assign43920_e49083_d_n8, assign43920_e49083_d_n9,) = {
    if ((var_guard1351 != 0.0) && (var_guard1352 != 0.0)) {
        let assign43920_e49070: f64 = (0.4 * p.p13);
        let assign43920_e49072: f64 = (assign43920_e49070 * 1.27520989);
        let assign43920_e49074: f64 = (-0.3333333333333);
        let assign43920_e49077: f64 = (var_tsisq / var_inv_phit_op);
        let assign43920_e49078: f64 = (assign43920_e49077).ln();
        let assign43920_e49079: f64 = (assign43920_e49074 * assign43920_e49078);
        let assign43920_e49080: f64 = (assign43920_e49079).exp();
        let assign43920_e49081: f64 = (assign43920_e49072 * assign43920_e49080);
        (assign43920_e49081, (assign43920_e49072 * (assign43920_e49080 * (assign43920_e49074 * ((-((var_tsisq * var_inv_phit_op_dn4) / (var_inv_phit_op * var_inv_phit_op))) / assign43920_e49077)))), (assign43920_e49072 * (assign43920_e49080 * (assign43920_e49074 * ((-((var_tsisq * var_inv_phit_op_dn6) / (var_inv_phit_op * var_inv_phit_op))) / assign43920_e49077)))), (assign43920_e49072 * (assign43920_e49080 * (assign43920_e49074 * ((-((var_tsisq * var_inv_phit_op_dn7) / (var_inv_phit_op * var_inv_phit_op))) / assign43920_e49077)))), (assign43920_e49072 * (assign43920_e49080 * (assign43920_e49074 * ((-((var_tsisq * var_inv_phit_op_dn8) / (var_inv_phit_op * var_inv_phit_op))) / assign43920_e49077)))), (assign43920_e49072 * (assign43920_e49080 * (assign43920_e49074 * ((-((var_tsisq * var_inv_phit_op_dn9) / (var_inv_phit_op * var_inv_phit_op))) / assign43920_e49077)))),)
    } else {
        (var_qq_op, var_qq_op_dn4, var_qq_op_dn6, var_qq_op_dn7, var_qq_op_dn8, var_qq_op_dn9,)
    }
};
        var_qq_op = assign43920_e49083;
        var_qq_op_dn4 = assign43920_e49083_d_n4;
        var_qq_op_dn6 = assign43920_e49083_d_n6;
        var_qq_op_dn7 = assign43920_e49083_d_n7;
        var_qq_op_dn8 = assign43920_e49083_d_n8;
        var_qq_op_dn9 = assign43920_e49083_d_n9;

        let (assign43930_e49103, assign43930_e49103_d_n4, assign43930_e49103_d_n6, assign43930_e49103_d_n7, assign43930_e49103_d_n8, assign43930_e49103_d_n9,) = {
    if ((var_guard1351 != 0.0) && (var_guard1352 == 0.0)) {
        let assign43930_e49090: f64 = (0.4 * p.p13);
        let assign43930_e49092: f64 = (assign43930_e49090 * 1.5412087);
        let assign43930_e49094: f64 = (-0.3333333333333);
        let assign43930_e49097: f64 = (var_tsisq / var_inv_phit_op);
        let assign43930_e49098: f64 = (assign43930_e49097).ln();
        let assign43930_e49099: f64 = (assign43930_e49094 * assign43930_e49098);
        let assign43930_e49100: f64 = (assign43930_e49099).exp();
        let assign43930_e49101: f64 = (assign43930_e49092 * assign43930_e49100);
        (assign43930_e49101, (assign43930_e49092 * (assign43930_e49100 * (assign43930_e49094 * ((-((var_tsisq * var_inv_phit_op_dn4) / (var_inv_phit_op * var_inv_phit_op))) / assign43930_e49097)))), (assign43930_e49092 * (assign43930_e49100 * (assign43930_e49094 * ((-((var_tsisq * var_inv_phit_op_dn6) / (var_inv_phit_op * var_inv_phit_op))) / assign43930_e49097)))), (assign43930_e49092 * (assign43930_e49100 * (assign43930_e49094 * ((-((var_tsisq * var_inv_phit_op_dn7) / (var_inv_phit_op * var_inv_phit_op))) / assign43930_e49097)))), (assign43930_e49092 * (assign43930_e49100 * (assign43930_e49094 * ((-((var_tsisq * var_inv_phit_op_dn8) / (var_inv_phit_op * var_inv_phit_op))) / assign43930_e49097)))), (assign43930_e49092 * (assign43930_e49100 * (assign43930_e49094 * ((-((var_tsisq * var_inv_phit_op_dn9) / (var_inv_phit_op * var_inv_phit_op))) / assign43930_e49097)))),)
    } else {
        (var_qq_op, var_qq_op_dn4, var_qq_op_dn6, var_qq_op_dn7, var_qq_op_dn8, var_qq_op_dn9,)
    }
};
        var_qq_op = assign43930_e49103;
        var_qq_op_dn4 = assign43930_e49103_d_n4;
        var_qq_op_dn6 = assign43930_e49103_d_n6;
        var_qq_op_dn7 = assign43930_e49103_d_n7;
        var_qq_op_dn8 = assign43930_e49103_d_n8;
        var_qq_op_dn9 = assign43930_e49103_d_n9;

        let assign43940_e49106: f64 = (var_vds * var_inv_phit_op);
        var_xd_op = assign43940_e49106;
        var_xd_op_dn4 = (var_vds * var_inv_phit_op_dn4);
        var_xd_op_dn6 = ((var_vds_dn6 * var_inv_phit_op) + (var_vds * var_inv_phit_op_dn6));
        var_xd_op_dn7 = ((var_vds_dn7 * var_inv_phit_op) + (var_vds * var_inv_phit_op_dn7));
        var_xd_op_dn8 = (var_vds * var_inv_phit_op_dn8);
        var_xd_op_dn9 = (var_vds * var_inv_phit_op_dn9);

        let assign43950_e49109: f64 = (var_vds * var_vds);
        let assign43950_e49111: f64 = (assign43950_e49109 + 0.01);
        let assign43950_e49112: f64 = (assign43950_e49111).sqrt();
        let assign43950_e49114: f64 = (assign43950_e49112 - 0.1);
        let assign43950_e49116: f64 = (assign43950_e49114 * var_inv_phit_op);
        var_xdsx_op = assign43950_e49116;
        var_xdsx_op_dn4 = (assign43950_e49114 * var_inv_phit_op_dn4);
        var_xdsx_op_dn6 = (((((var_vds_dn6 * var_vds) + (var_vds * var_vds_dn6)) / (2.0 * assign43950_e49112)) * var_inv_phit_op) + (assign43950_e49114 * var_inv_phit_op_dn6));
        var_xdsx_op_dn7 = (((((var_vds_dn7 * var_vds) + (var_vds * var_vds_dn7)) / (2.0 * assign43950_e49112)) * var_inv_phit_op) + (assign43950_e49114 * var_inv_phit_op_dn7));
        var_xdsx_op_dn8 = (assign43950_e49114 * var_inv_phit_op_dn8);
        var_xdsx_op_dn9 = (assign43950_e49114 * var_inv_phit_op_dn9);

        let assign43960_e49120: f64 = (var_xd_op - var_xdsx_op);
        let assign43960_e49121: f64 = (0.5 * assign43960_e49120);
        var_dxdsx_op = assign43960_e49121;
        var_dxdsx_op_dn4 = (0.5 * (var_xd_op_dn4 - var_xdsx_op_dn4));
        var_dxdsx_op_dn6 = (0.5 * (var_xd_op_dn6 - var_xdsx_op_dn6));
        var_dxdsx_op_dn7 = (0.5 * (var_xd_op_dn7 - var_xdsx_op_dn7));
        var_dxdsx_op_dn8 = (0.5 * (var_xd_op_dn8 - var_xdsx_op_dn8));
        var_dxdsx_op_dn9 = (0.5 * (var_xd_op_dn9 - var_xdsx_op_dn9));

        let assign43970_e49124: f64 = (var_k2_dc / var_k1_dc);
        let assign43970_e49127: f64 = (1.0 + var_k2_dc);
        let assign43970_e49128: f64 = (assign43970_e49124 / assign43970_e49127);
        var_r1init_op = assign43970_e49128;
        var_r1init_op_dn4 = ((((((var_k2_dc_dn4 * var_k1_dc) - (var_k2_dc * var_k1_dc_dn4)) / (var_k1_dc * var_k1_dc)) * assign43970_e49127) - (assign43970_e49124 * var_k2_dc_dn4)) / (assign43970_e49127 * assign43970_e49127));
        var_r1init_op_dn6 = ((((((var_k2_dc_dn6 * var_k1_dc) - (var_k2_dc * var_k1_dc_dn6)) / (var_k1_dc * var_k1_dc)) * assign43970_e49127) - (assign43970_e49124 * var_k2_dc_dn6)) / (assign43970_e49127 * assign43970_e49127));
        var_r1init_op_dn7 = ((((((var_k2_dc_dn7 * var_k1_dc) - (var_k2_dc * var_k1_dc_dn7)) / (var_k1_dc * var_k1_dc)) * assign43970_e49127) - (assign43970_e49124 * var_k2_dc_dn7)) / (assign43970_e49127 * assign43970_e49127));
        var_r1init_op_dn8 = ((((((var_k2_dc_dn8 * var_k1_dc) - (var_k2_dc * var_k1_dc_dn8)) / (var_k1_dc * var_k1_dc)) * assign43970_e49127) - (assign43970_e49124 * var_k2_dc_dn8)) / (assign43970_e49127 * assign43970_e49127));
        var_r1init_op_dn9 = ((((((var_k2_dc_dn9 * var_k1_dc) - (var_k2_dc * var_k1_dc_dn9)) / (var_k1_dc * var_k1_dc)) * assign43970_e49127) - (assign43970_e49124 * var_k2_dc_dn9)) / (assign43970_e49127 * assign43970_e49127));

        let assign43980_e49131: f64 = (var_k1_dc / var_k2_dc);
        let assign43980_e49134: f64 = (1.0 + var_k1_dc);
        let assign43980_e49135: f64 = (assign43980_e49131 / assign43980_e49134);
        var_r2init_op = assign43980_e49135;
        var_r2init_op_dn4 = ((((((var_k1_dc_dn4 * var_k2_dc) - (var_k1_dc * var_k2_dc_dn4)) / (var_k2_dc * var_k2_dc)) * assign43980_e49134) - (assign43980_e49131 * var_k1_dc_dn4)) / (assign43980_e49134 * assign43980_e49134));
        var_r2init_op_dn6 = ((((((var_k1_dc_dn6 * var_k2_dc) - (var_k1_dc * var_k2_dc_dn6)) / (var_k2_dc * var_k2_dc)) * assign43980_e49134) - (assign43980_e49131 * var_k1_dc_dn6)) / (assign43980_e49134 * assign43980_e49134));
        var_r2init_op_dn7 = ((((((var_k1_dc_dn7 * var_k2_dc) - (var_k1_dc * var_k2_dc_dn7)) / (var_k2_dc * var_k2_dc)) * assign43980_e49134) - (assign43980_e49131 * var_k1_dc_dn7)) / (assign43980_e49134 * assign43980_e49134));
        var_r2init_op_dn8 = ((((((var_k1_dc_dn8 * var_k2_dc) - (var_k1_dc * var_k2_dc_dn8)) / (var_k2_dc * var_k2_dc)) * assign43980_e49134) - (assign43980_e49131 * var_k1_dc_dn8)) / (assign43980_e49134 * assign43980_e49134));
        var_r2init_op_dn9 = ((((((var_k1_dc_dn9 * var_k2_dc) - (var_k1_dc * var_k2_dc_dn9)) / (var_k2_dc * var_k2_dc)) * assign43980_e49134) - (assign43980_e49131 * var_k1_dc_dn9)) / (assign43980_e49134 * assign43980_e49134));

        let assign43990_e49139: f64 = (1.0 + var_r1init_op);
        let assign43990_e49140: f64 = (var_k1_dc * assign43990_e49139);
        let assign43990_e49142: f64 = (assign43990_e49140 * var_diff_min_dc);
        let assign43990_e49144: f64 = (assign43990_e49142 / var_a0_dc);
        let assign43990_e49145: f64 = (assign43990_e49144).ln();
        let assign43990_e49147: f64 = (assign43990_e49145 + 2.0);
        var_x1init_op = assign43990_e49147;
        var_x1init_op_dn4 = ((((((((var_k1_dc_dn4 * assign43990_e49139) + (var_k1_dc * var_r1init_op_dn4)) * var_diff_min_dc) + (assign43990_e49140 * var_diff_min_dc_dn4)) * var_a0_dc) - (assign43990_e49142 * var_a0_dc_dn4)) / (var_a0_dc * var_a0_dc)) / assign43990_e49144);
        var_x1init_op_dn6 = ((((((((var_k1_dc_dn6 * assign43990_e49139) + (var_k1_dc * var_r1init_op_dn6)) * var_diff_min_dc) + (assign43990_e49140 * var_diff_min_dc_dn6)) * var_a0_dc) - (assign43990_e49142 * var_a0_dc_dn6)) / (var_a0_dc * var_a0_dc)) / assign43990_e49144);
        var_x1init_op_dn7 = ((((((((var_k1_dc_dn7 * assign43990_e49139) + (var_k1_dc * var_r1init_op_dn7)) * var_diff_min_dc) + (assign43990_e49140 * var_diff_min_dc_dn7)) * var_a0_dc) - (assign43990_e49142 * var_a0_dc_dn7)) / (var_a0_dc * var_a0_dc)) / assign43990_e49144);
        var_x1init_op_dn8 = ((((((((var_k1_dc_dn8 * assign43990_e49139) + (var_k1_dc * var_r1init_op_dn8)) * var_diff_min_dc) + (assign43990_e49140 * var_diff_min_dc_dn8)) * var_a0_dc) - (assign43990_e49142 * var_a0_dc_dn8)) / (var_a0_dc * var_a0_dc)) / assign43990_e49144);
        var_x1init_op_dn9 = ((((((((var_k1_dc_dn9 * assign43990_e49139) + (var_k1_dc * var_r1init_op_dn9)) * var_diff_min_dc) + (assign43990_e49140 * var_diff_min_dc_dn9)) * var_a0_dc) - (assign43990_e49142 * var_a0_dc_dn9)) / (var_a0_dc * var_a0_dc)) / assign43990_e49144);

        let assign44000_e49151: f64 = (1.0 + var_r2init_op);
        let assign44000_e49152: f64 = (var_k2_dc * assign44000_e49151);
        let assign44000_e49154: f64 = (assign44000_e49152 * var_diff_min_dc);
        let assign44000_e49156: f64 = (assign44000_e49154 / var_a0_dc);
        let assign44000_e49157: f64 = (assign44000_e49156).ln();
        let assign44000_e49159: f64 = (assign44000_e49157 + 2.0);
        var_x2init_op = assign44000_e49159;
        var_x2init_op_dn4 = ((((((((var_k2_dc_dn4 * assign44000_e49151) + (var_k2_dc * var_r2init_op_dn4)) * var_diff_min_dc) + (assign44000_e49152 * var_diff_min_dc_dn4)) * var_a0_dc) - (assign44000_e49154 * var_a0_dc_dn4)) / (var_a0_dc * var_a0_dc)) / assign44000_e49156);
        var_x2init_op_dn6 = ((((((((var_k2_dc_dn6 * assign44000_e49151) + (var_k2_dc * var_r2init_op_dn6)) * var_diff_min_dc) + (assign44000_e49152 * var_diff_min_dc_dn6)) * var_a0_dc) - (assign44000_e49154 * var_a0_dc_dn6)) / (var_a0_dc * var_a0_dc)) / assign44000_e49156);
        var_x2init_op_dn7 = ((((((((var_k2_dc_dn7 * assign44000_e49151) + (var_k2_dc * var_r2init_op_dn7)) * var_diff_min_dc) + (assign44000_e49152 * var_diff_min_dc_dn7)) * var_a0_dc) - (assign44000_e49154 * var_a0_dc_dn7)) / (var_a0_dc * var_a0_dc)) / assign44000_e49156);
        var_x2init_op_dn8 = ((((((((var_k2_dc_dn8 * assign44000_e49151) + (var_k2_dc * var_r2init_op_dn8)) * var_diff_min_dc) + (assign44000_e49152 * var_diff_min_dc_dn8)) * var_a0_dc) - (assign44000_e49154 * var_a0_dc_dn8)) / (var_a0_dc * var_a0_dc)) / assign44000_e49156);
        var_x2init_op_dn9 = ((((((((var_k2_dc_dn9 * assign44000_e49151) + (var_k2_dc * var_r2init_op_dn9)) * var_diff_min_dc) + (assign44000_e49152 * var_diff_min_dc_dn9)) * var_a0_dc) - (assign44000_e49154 * var_a0_dc_dn9)) / (var_a0_dc * var_a0_dc)) / assign44000_e49156);

        let assign44010_e49162: f64 = (1.0 + var_r1init_op);
        let assign44010_e49164: f64 = (assign44010_e49162 * var_x1init_op);
        let assign44010_e49167: f64 = (var_xg2x_dc * var_r1init_op);
        let assign44010_e49168: f64 = (assign44010_e49164 - assign44010_e49167);
        var_xth1init_op = assign44010_e49168;
        var_xth1init_op_dn4 = (((var_r1init_op_dn4 * var_x1init_op) + (assign44010_e49162 * var_x1init_op_dn4)) - ((var_xg2x_dc_dn4 * var_r1init_op) + (var_xg2x_dc * var_r1init_op_dn4)));
        var_xth1init_op_dn6 = (((var_r1init_op_dn6 * var_x1init_op) + (assign44010_e49162 * var_x1init_op_dn6)) - ((var_xg2x_dc_dn6 * var_r1init_op) + (var_xg2x_dc * var_r1init_op_dn6)));
        var_xth1init_op_dn7 = (((var_r1init_op_dn7 * var_x1init_op) + (assign44010_e49162 * var_x1init_op_dn7)) - ((var_xg2x_dc_dn7 * var_r1init_op) + (var_xg2x_dc * var_r1init_op_dn7)));
        var_xth1init_op_dn8 = (((var_r1init_op_dn8 * var_x1init_op) + (assign44010_e49162 * var_x1init_op_dn8)) - ((var_xg2x_dc_dn8 * var_r1init_op) + (var_xg2x_dc * var_r1init_op_dn8)));
        var_xth1init_op_dn9 = (((var_r1init_op_dn9 * var_x1init_op) + (assign44010_e49162 * var_x1init_op_dn9)) - ((var_xg2x_dc_dn9 * var_r1init_op) + (var_xg2x_dc * var_r1init_op_dn9)));

        let assign44020_e49172: f64 = (1.0 / var_r2init_op);
        let assign44020_e49173: f64 = (1.0 + assign44020_e49172);
        let assign44020_e49175: f64 = (assign44020_e49173 * var_x2init_op);
        let assign44020_e49178: f64 = (var_xg2x_dc / var_r2init_op);
        let assign44020_e49179: f64 = (assign44020_e49175 - assign44020_e49178);
        var_xth2init_op = assign44020_e49179;
        var_xth2init_op_dn4 = ((((-(var_r2init_op_dn4 / (var_r2init_op * var_r2init_op))) * var_x2init_op) + (assign44020_e49173 * var_x2init_op_dn4)) - (((var_xg2x_dc_dn4 * var_r2init_op) - (var_xg2x_dc * var_r2init_op_dn4)) / (var_r2init_op * var_r2init_op)));
        var_xth2init_op_dn6 = ((((-(var_r2init_op_dn6 / (var_r2init_op * var_r2init_op))) * var_x2init_op) + (assign44020_e49173 * var_x2init_op_dn6)) - (((var_xg2x_dc_dn6 * var_r2init_op) - (var_xg2x_dc * var_r2init_op_dn6)) / (var_r2init_op * var_r2init_op)));
        var_xth2init_op_dn7 = ((((-(var_r2init_op_dn7 / (var_r2init_op * var_r2init_op))) * var_x2init_op) + (assign44020_e49173 * var_x2init_op_dn7)) - (((var_xg2x_dc_dn7 * var_r2init_op) - (var_xg2x_dc * var_r2init_op_dn7)) / (var_r2init_op * var_r2init_op)));
        var_xth2init_op_dn8 = ((((-(var_r2init_op_dn8 / (var_r2init_op * var_r2init_op))) * var_x2init_op) + (assign44020_e49173 * var_x2init_op_dn8)) - (((var_xg2x_dc_dn8 * var_r2init_op) - (var_xg2x_dc * var_r2init_op_dn8)) / (var_r2init_op * var_r2init_op)));
        var_xth2init_op_dn9 = ((((-(var_r2init_op_dn9 / (var_r2init_op * var_r2init_op))) * var_x2init_op) + (assign44020_e49173 * var_x2init_op_dn9)) - (((var_xg2x_dc_dn9 * var_r2init_op) - (var_xg2x_dc * var_r2init_op_dn9)) / (var_r2init_op * var_r2init_op)));

        let assign44030_e49183: f64 = (var_xth1init_op + var_xth2init_op);
        let assign44030_e49186: f64 = (var_xth1init_op - var_xth2init_op);
        let assign44030_e49189: f64 = (var_xth1init_op - var_xth2init_op);
        let assign44030_e49190: f64 = (assign44030_e49186 * assign44030_e49189);
        let assign44030_e49192: f64 = (assign44030_e49190 + 38.0);
        let assign44030_e49193: f64 = (assign44030_e49192).sqrt();
        let assign44030_e49194: f64 = (assign44030_e49183 - assign44030_e49193);
        let assign44030_e49195: f64 = (0.5 * assign44030_e49194);
        let assign44030_e49197: f64 = (assign44030_e49195 - var_xg2_dc);
        let assign44030_e49199: f64 = (assign44030_e49197 / var_cic1_i);
        let assign44030_e49201: f64 = (assign44030_e49199 + var_xg2_dc);
        var_xg1thinit_op = assign44030_e49201;
        var_xg1thinit_op_dn4 = ((((0.5 * ((var_xth1init_op_dn4 + var_xth2init_op_dn4) - ((((var_xth1init_op_dn4 - var_xth2init_op_dn4) * assign44030_e49189) + (assign44030_e49186 * (var_xth1init_op_dn4 - var_xth2init_op_dn4))) / (2.0 * assign44030_e49193)))) - var_xg2_dc_dn4) / var_cic1_i) + var_xg2_dc_dn4);
        var_xg1thinit_op_dn6 = ((((0.5 * ((var_xth1init_op_dn6 + var_xth2init_op_dn6) - ((((var_xth1init_op_dn6 - var_xth2init_op_dn6) * assign44030_e49189) + (assign44030_e49186 * (var_xth1init_op_dn6 - var_xth2init_op_dn6))) / (2.0 * assign44030_e49193)))) - var_xg2_dc_dn6) / var_cic1_i) + var_xg2_dc_dn6);
        var_xg1thinit_op_dn7 = ((((0.5 * ((var_xth1init_op_dn7 + var_xth2init_op_dn7) - ((((var_xth1init_op_dn7 - var_xth2init_op_dn7) * assign44030_e49189) + (assign44030_e49186 * (var_xth1init_op_dn7 - var_xth2init_op_dn7))) / (2.0 * assign44030_e49193)))) - var_xg2_dc_dn7) / var_cic1_i) + var_xg2_dc_dn7);
        var_xg1thinit_op_dn8 = ((((0.5 * ((var_xth1init_op_dn8 + var_xth2init_op_dn8) - ((((var_xth1init_op_dn8 - var_xth2init_op_dn8) * assign44030_e49189) + (assign44030_e49186 * (var_xth1init_op_dn8 - var_xth2init_op_dn8))) / (2.0 * assign44030_e49193)))) - var_xg2_dc_dn8) / var_cic1_i) + var_xg2_dc_dn8);
        var_xg1thinit_op_dn9 = ((((0.5 * ((var_xth1init_op_dn9 + var_xth2init_op_dn9) - ((((var_xth1init_op_dn9 - var_xth2init_op_dn9) * assign44030_e49189) + (assign44030_e49186 * (var_xth1init_op_dn9 - var_xth2init_op_dn9))) / (2.0 * assign44030_e49193)))) - var_xg2_dc_dn9) / var_cic1_i) + var_xg2_dc_dn9);

        let assign44040_e49205: f64 = (var_xg1thinit_op - var_xedge_dc);
        let assign44040_e49207: f64 = (assign44040_e49205 / var_sce1_dc);
        let assign44040_e49209: f64 = (assign44040_e49207 - var_dxg1_dibl_dc);
        let assign44040_e49211: f64 = (assign44040_e49209 + var_xedge_dc);
        let assign44040_e49212: f64 = (var_phit * assign44040_e49211);
        let assign44040_e49214: f64 = (assign44040_e49212 + var_vfb1_i);
        var_vthinit_op = assign44040_e49214;
        var_vthinit_op_dn4 = (((var_phit_dn4 * assign44040_e49211) + (var_phit * ((((((var_xg1thinit_op_dn4 - var_xedge_dc_dn4) * var_sce1_dc) - (assign44040_e49205 * var_sce1_dc_dn4)) / (var_sce1_dc * var_sce1_dc)) - var_dxg1_dibl_dc_dn4) + var_xedge_dc_dn4))) + var_vfb1_i_dn4);
        var_vthinit_op_dn6 = (((var_phit_dn6 * assign44040_e49211) + (var_phit * ((((((var_xg1thinit_op_dn6 - var_xedge_dc_dn6) * var_sce1_dc) - (assign44040_e49205 * var_sce1_dc_dn6)) / (var_sce1_dc * var_sce1_dc)) - var_dxg1_dibl_dc_dn6) + var_xedge_dc_dn6))) + var_vfb1_i_dn6);
        var_vthinit_op_dn7 = (((var_phit_dn7 * assign44040_e49211) + (var_phit * ((((((var_xg1thinit_op_dn7 - var_xedge_dc_dn7) * var_sce1_dc) - (assign44040_e49205 * var_sce1_dc_dn7)) / (var_sce1_dc * var_sce1_dc)) - var_dxg1_dibl_dc_dn7) + var_xedge_dc_dn7))) + var_vfb1_i_dn7);
        var_vthinit_op_dn8 = (((var_phit_dn8 * assign44040_e49211) + (var_phit * ((((((var_xg1thinit_op_dn8 - var_xedge_dc_dn8) * var_sce1_dc) - (assign44040_e49205 * var_sce1_dc_dn8)) / (var_sce1_dc * var_sce1_dc)) - var_dxg1_dibl_dc_dn8) + var_xedge_dc_dn8))) + var_vfb1_i_dn8);
        var_vthinit_op_dn9 = (((var_phit_dn9 * assign44040_e49211) + (var_phit * ((((((var_xg1thinit_op_dn9 - var_xedge_dc_dn9) * var_sce1_dc) - (assign44040_e49205 * var_sce1_dc_dn9)) / (var_sce1_dc * var_sce1_dc)) - var_dxg1_dibl_dc_dn9) + var_xedge_dc_dn9))) + var_vfb1_i_dn9);

        let assign44050_e49218: f64 = (var_tkd - var_tkr);
        let assign44050_e49219: f64 = (var_stcf_i * assign44050_e49218);
        var_temp = assign44050_e49219;
        var_temp_dn4 = ((var_stcf_i_dn4 * assign44050_e49218) + (var_stcf_i * var_tkd_dn4));
        var_temp_dn6 = ((var_stcf_i_dn6 * assign44050_e49218) + (var_stcf_i * var_tkd_dn6));
        var_temp_dn7 = ((var_stcf_i_dn7 * assign44050_e49218) + (var_stcf_i * var_tkd_dn7));
        var_temp_dn8 = ((var_stcf_i_dn8 * assign44050_e49218) + (var_stcf_i * var_tkd_dn8));
        var_temp_dn9 = ((var_stcf_i_dn9 * assign44050_e49218) + (var_stcf_i * var_tkd_dn9));

        let assign44080_e49228: f64 = (p.p14 * var_stvfb_i);
        let assign44080_e49231: f64 = (var_tkd - var_tkr);
        let assign44080_e49232: f64 = (assign44080_e49228 * assign44080_e49231);
        let assign44080_e49234: f64 = (assign44080_e49232 + var_dvfbqm);
        var_temp = assign44080_e49234;
        var_temp_dn4 = (assign44080_e49228 * var_tkd_dn4);
        var_temp_dn6 = (assign44080_e49228 * var_tkd_dn6);
        var_temp_dn7 = (assign44080_e49228 * var_tkd_dn7);
        var_temp_dn8 = (assign44080_e49228 * var_tkd_dn8);
        var_temp_dn9 = (assign44080_e49228 * var_tkd_dn9);

        let assign44090_e49238: f64 = (var_vfb1_t + var_dvfbch_op);
        let assign44090_e49240: f64 = (assign44090_e49238 + var_dvfb1nch);
        let assign44090_e49241: f64 = (p.p14 * assign44090_e49240);
        let assign44090_e49243: f64 = (assign44090_e49241 + var_temp);
        let assign44090_e49245: f64 = (assign44090_e49243 + p.p34);
        let assign44090_e49247: f64 = (assign44090_e49245 - var_dvfbpdep_op);
        var_vfb1_op = assign44090_e49247;
        var_vfb1_op_dn4 = (((p.p14 * ((var_vfb1_t_dn4 + var_dvfbch_op_dn4) + var_dvfb1nch_dn4)) + var_temp_dn4) - var_dvfbpdep_op_dn4);
        var_vfb1_op_dn6 = (((p.p14 * ((var_vfb1_t_dn6 + var_dvfbch_op_dn6) + var_dvfb1nch_dn6)) + var_temp_dn6) - var_dvfbpdep_op_dn6);
        var_vfb1_op_dn7 = (((p.p14 * ((var_vfb1_t_dn7 + var_dvfbch_op_dn7) + var_dvfb1nch_dn7)) + var_temp_dn7) - var_dvfbpdep_op_dn7);
        var_vfb1_op_dn8 = (((p.p14 * ((var_vfb1_t_dn8 + var_dvfbch_op_dn8) + var_dvfb1nch_dn8)) + var_temp_dn8) - var_dvfbpdep_op_dn8);
        var_vfb1_op_dn9 = (((p.p14 * ((var_vfb1_t_dn9 + var_dvfbch_op_dn9) + var_dvfb1nch_dn9)) + var_temp_dn9) - var_dvfbpdep_op_dn9);

        let assign44100_e49251: f64 = (var_vfb2_t + var_dvfbch_op);
        let assign44100_e49253: f64 = (assign44100_e49251 + var_dvfb2nch);
        let assign44100_e49254: f64 = (p.p14 * assign44100_e49253);
        let assign44100_e49256: f64 = (assign44100_e49254 + var_temp);
        var_vfb2_op = assign44100_e49256;
        var_vfb2_op_dn4 = ((p.p14 * ((var_vfb2_t_dn4 + var_dvfbch_op_dn4) + var_dvfb2nch_dn4)) + var_temp_dn4);
        var_vfb2_op_dn6 = ((p.p14 * ((var_vfb2_t_dn6 + var_dvfbch_op_dn6) + var_dvfb2nch_dn6)) + var_temp_dn6);
        var_vfb2_op_dn7 = ((p.p14 * ((var_vfb2_t_dn7 + var_dvfbch_op_dn7) + var_dvfb2nch_dn7)) + var_temp_dn7);
        var_vfb2_op_dn8 = ((p.p14 * ((var_vfb2_t_dn8 + var_dvfbch_op_dn8) + var_dvfb2nch_dn8)) + var_temp_dn8);
        var_vfb2_op_dn9 = ((p.p14 * ((var_vfb2_t_dn9 + var_dvfbch_op_dn9) + var_dvfb2nch_dn9)) + var_temp_dn9);

        let assign44110_e49259: f64 = (var_vthinit_op - var_vfb1_op);
        let assign44110_e49261: f64 = (assign44110_e49259 * var_inv_phit_op);
        let assign44110_e49263: f64 = (assign44110_e49261 - var_dxdsx_op);
        var_xg10_op = assign44110_e49263;
        var_xg10_op_dn4 = ((((var_vthinit_op_dn4 - var_vfb1_op_dn4) * var_inv_phit_op) + (assign44110_e49259 * var_inv_phit_op_dn4)) - var_dxdsx_op_dn4);
        var_xg10_op_dn6 = ((((var_vthinit_op_dn6 - var_vfb1_op_dn6) * var_inv_phit_op) + (assign44110_e49259 * var_inv_phit_op_dn6)) - var_dxdsx_op_dn6);
        var_xg10_op_dn7 = ((((var_vthinit_op_dn7 - var_vfb1_op_dn7) * var_inv_phit_op) + (assign44110_e49259 * var_inv_phit_op_dn7)) - var_dxdsx_op_dn7);
        var_xg10_op_dn8 = ((((var_vthinit_op_dn8 - var_vfb1_op_dn8) * var_inv_phit_op) + (assign44110_e49259 * var_inv_phit_op_dn8)) - var_dxdsx_op_dn8);
        var_xg10_op_dn9 = ((((var_vthinit_op_dn9 - var_vfb1_op_dn9) * var_inv_phit_op) + (assign44110_e49259 * var_inv_phit_op_dn9)) - var_dxdsx_op_dn9);

        let assign44120_e49265: f64 = (-var_vsb);
        let assign44120_e49267: f64 = (assign44120_e49265 - var_vfb2_op);
        let assign44120_e49269: f64 = (assign44120_e49267 * var_inv_phit_op);
        let assign44120_e49271: f64 = (assign44120_e49269 - var_dxdsx_op);
        var_xg20_op = assign44120_e49271;
        var_xg20_op_dn4 = ((((-var_vfb2_op_dn4) * var_inv_phit_op) + (assign44120_e49267 * var_inv_phit_op_dn4)) - var_dxdsx_op_dn4);
        var_xg20_op_dn6 = (((((-var_vsb_dn6) - var_vfb2_op_dn6) * var_inv_phit_op) + (assign44120_e49267 * var_inv_phit_op_dn6)) - var_dxdsx_op_dn6);
        var_xg20_op_dn7 = (((((-var_vsb_dn7) - var_vfb2_op_dn7) * var_inv_phit_op) + (assign44120_e49267 * var_inv_phit_op_dn7)) - var_dxdsx_op_dn7);
        var_xg20_op_dn8 = (((((-var_vsb_dn8) - var_vfb2_op_dn8) * var_inv_phit_op) + (assign44120_e49267 * var_inv_phit_op_dn8)) - var_dxdsx_op_dn8);
        var_xg20_op_dn9 = ((((-var_vfb2_op_dn9) * var_inv_phit_op) + (assign44120_e49267 * var_inv_phit_op_dn9)) - var_dxdsx_op_dn9);

        let assign44130_e49274: f64 = if p.p2 > 0.0 { 1.0 } else { 0.0 };
        var_guard1353 = assign44130_e49274;

        let (assign44140_e49286, assign44140_e49286_d_n4, assign44140_e49286_d_n6, assign44140_e49286_d_n7, assign44140_e49286_d_n8, assign44140_e49286_d_n9,) = {
    if (var_guard1353 != 0.0) {
        let assign44140_e49278: f64 = (p.p14 * var_typesub_i);
        let assign44140_e49281: f64 = (var_xg10_op - var_xg20_op);
        let assign44140_e49282: f64 = (assign44140_e49278 * assign44140_e49281);
        let assign44140_e49284: f64 = (assign44140_e49282 / var_gfsub);
        (assign44140_e49284, ((((assign44140_e49278 * (var_xg10_op_dn4 - var_xg20_op_dn4)) * var_gfsub) - (assign44140_e49282 * var_gfsub_dn4)) / (var_gfsub * var_gfsub)), ((((assign44140_e49278 * (var_xg10_op_dn6 - var_xg20_op_dn6)) * var_gfsub) - (assign44140_e49282 * var_gfsub_dn6)) / (var_gfsub * var_gfsub)), ((((assign44140_e49278 * (var_xg10_op_dn7 - var_xg20_op_dn7)) * var_gfsub) - (assign44140_e49282 * var_gfsub_dn7)) / (var_gfsub * var_gfsub)), ((((assign44140_e49278 * (var_xg10_op_dn8 - var_xg20_op_dn8)) * var_gfsub) - (assign44140_e49282 * var_gfsub_dn8)) / (var_gfsub * var_gfsub)), ((((assign44140_e49278 * (var_xg10_op_dn9 - var_xg20_op_dn9)) * var_gfsub) - (assign44140_e49282 * var_gfsub_dn9)) / (var_gfsub * var_gfsub)),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign44140_e49286;
        var_temp_dn4 = assign44140_e49286_d_n4;
        var_temp_dn6 = assign44140_e49286_d_n6;
        var_temp_dn7 = assign44140_e49286_d_n7;
        var_temp_dn8 = assign44140_e49286_d_n8;
        var_temp_dn9 = assign44140_e49286_d_n9;

        let assign44150_e49289: f64 = if var_temp < 0.0 { 1.0 } else { 0.0 };
        var_guard1354 = assign44150_e49289;

        let (assign44160_e49301, assign44160_e49301_d_n4, assign44160_e49301_d_n6, assign44160_e49301_d_n7, assign44160_e49301_d_n8, assign44160_e49301_d_n9,) = {
    if ((var_guard1353 != 0.0) && (var_guard1354 != 0.0)) {
        let assign44160_e49294: f64 = (-2.0);
        let assign44160_e49297: f64 = (1.0 - var_temp);
        let assign44160_e49298: f64 = (assign44160_e49297).ln();
        let assign44160_e49299: f64 = (assign44160_e49294 * assign44160_e49298);
        (assign44160_e49299, (assign44160_e49294 * ((-var_temp_dn4) / assign44160_e49297)), (assign44160_e49294 * ((-var_temp_dn6) / assign44160_e49297)), (assign44160_e49294 * ((-var_temp_dn7) / assign44160_e49297)), (assign44160_e49294 * ((-var_temp_dn8) / assign44160_e49297)), (assign44160_e49294 * ((-var_temp_dn9) / assign44160_e49297)),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign44160_e49301;
        var_temp1_dn4 = assign44160_e49301_d_n4;
        var_temp1_dn6 = assign44160_e49301_d_n6;
        var_temp1_dn7 = assign44160_e49301_d_n7;
        var_temp1_dn8 = assign44160_e49301_d_n8;
        var_temp1_dn9 = assign44160_e49301_d_n9;

        let (assign44170_e49318, assign44170_e49318_d_n4, assign44170_e49318_d_n6, assign44170_e49318_d_n7, assign44170_e49318_d_n8, assign44170_e49318_d_n9,) = {
    if ((var_guard1353 != 0.0) && (var_guard1354 == 0.0)) {
        let assign44170_e49308: f64 = (var_temp * var_temp);
        let assign44170_e49312: f64 = (2.0 * var_temp);
        let assign44170_e49314: f64 = (assign44170_e49312 / var_gfsub);
        let assign44170_e49315: f64 = (1.0 + assign44170_e49314);
        let assign44170_e49316: f64 = (assign44170_e49308 / assign44170_e49315);
        (assign44170_e49316, (((((var_temp_dn4 * var_temp) + (var_temp * var_temp_dn4)) * assign44170_e49315) - (assign44170_e49308 * ((((2.0 * var_temp_dn4) * var_gfsub) - (assign44170_e49312 * var_gfsub_dn4)) / (var_gfsub * var_gfsub)))) / (assign44170_e49315 * assign44170_e49315)), (((((var_temp_dn6 * var_temp) + (var_temp * var_temp_dn6)) * assign44170_e49315) - (assign44170_e49308 * ((((2.0 * var_temp_dn6) * var_gfsub) - (assign44170_e49312 * var_gfsub_dn6)) / (var_gfsub * var_gfsub)))) / (assign44170_e49315 * assign44170_e49315)), (((((var_temp_dn7 * var_temp) + (var_temp * var_temp_dn7)) * assign44170_e49315) - (assign44170_e49308 * ((((2.0 * var_temp_dn7) * var_gfsub) - (assign44170_e49312 * var_gfsub_dn7)) / (var_gfsub * var_gfsub)))) / (assign44170_e49315 * assign44170_e49315)), (((((var_temp_dn8 * var_temp) + (var_temp * var_temp_dn8)) * assign44170_e49315) - (assign44170_e49308 * ((((2.0 * var_temp_dn8) * var_gfsub) - (assign44170_e49312 * var_gfsub_dn8)) / (var_gfsub * var_gfsub)))) / (assign44170_e49315 * assign44170_e49315)), (((((var_temp_dn9 * var_temp) + (var_temp * var_temp_dn9)) * assign44170_e49315) - (assign44170_e49308 * ((((2.0 * var_temp_dn9) * var_gfsub) - (assign44170_e49312 * var_gfsub_dn9)) / (var_gfsub * var_gfsub)))) / (assign44170_e49315 * assign44170_e49315)),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign44170_e49318;
        var_temp1_dn4 = assign44170_e49318_d_n4;
        var_temp1_dn6 = assign44170_e49318_d_n6;
        var_temp1_dn7 = assign44170_e49318_d_n7;
        var_temp1_dn8 = assign44170_e49318_d_n8;
        var_temp1_dn9 = assign44170_e49318_d_n9;

        let (assign44180_e49328, assign44180_e49328_d_n4, assign44180_e49328_d_n6, assign44180_e49328_d_n7, assign44180_e49328_d_n8, assign44180_e49328_d_n9,) = {
    if (var_guard1353 != 0.0) {
        let assign44180_e49323: f64 = (p.p14 * var_typesub_i);
        let assign44180_e49325: f64 = (assign44180_e49323 * var_temp1);
        let assign44180_e49326: f64 = (var_xg20_op + assign44180_e49325);
        (assign44180_e49326, (var_xg20_op_dn4 + (assign44180_e49323 * var_temp1_dn4)), (var_xg20_op_dn6 + (assign44180_e49323 * var_temp1_dn6)), (var_xg20_op_dn7 + (assign44180_e49323 * var_temp1_dn7)), (var_xg20_op_dn8 + (assign44180_e49323 * var_temp1_dn8)), (var_xg20_op_dn9 + (assign44180_e49323 * var_temp1_dn9)),)
    } else {
        (var_xg2eff_op, var_xg2eff_op_dn4, var_xg2eff_op_dn6, var_xg2eff_op_dn7, var_xg2eff_op_dn8, var_xg2eff_op_dn9,)
    }
};
        var_xg2eff_op = assign44180_e49328;
        var_xg2eff_op_dn4 = assign44180_e49328_d_n4;
        var_xg2eff_op_dn6 = assign44180_e49328_d_n6;
        var_xg2eff_op_dn7 = assign44180_e49328_d_n7;
        var_xg2eff_op_dn8 = assign44180_e49328_d_n8;
        var_xg2eff_op_dn9 = assign44180_e49328_d_n9;

        let (assign44190_e49333, assign44190_e49333_d_n4, assign44190_e49333_d_n6, assign44190_e49333_d_n7, assign44190_e49333_d_n8, assign44190_e49333_d_n9,) = {
    if (var_guard1353 == 0.0) {
        (var_xg20_op, var_xg20_op_dn4, var_xg20_op_dn6, var_xg20_op_dn7, var_xg20_op_dn8, var_xg20_op_dn9,)
    } else {
        (var_xg2eff_op, var_xg2eff_op_dn4, var_xg2eff_op_dn6, var_xg2eff_op_dn7, var_xg2eff_op_dn8, var_xg2eff_op_dn9,)
    }
};
        var_xg2eff_op = assign44190_e49333;
        var_xg2eff_op_dn4 = assign44190_e49333_d_n4;
        var_xg2eff_op_dn6 = assign44190_e49333_d_n6;
        var_xg2eff_op_dn7 = assign44190_e49333_d_n7;
        var_xg2eff_op_dn8 = assign44190_e49333_d_n8;
        var_xg2eff_op_dn9 = assign44190_e49333_d_n9;

        let assign44200_e49337: f64 = (var_xg10_op - var_xg2eff_op);
        let assign44200_e49338: f64 = (var_keq_1d * assign44200_e49337);
        var_temp = assign44200_e49338;
        var_temp_dn4 = (var_keq_1d * (var_xg10_op_dn4 - var_xg2eff_op_dn4));
        var_temp_dn6 = (var_keq_1d * (var_xg10_op_dn6 - var_xg2eff_op_dn6));
        var_temp_dn7 = (var_keq_1d * (var_xg10_op_dn7 - var_xg2eff_op_dn7));
        var_temp_dn8 = (var_keq_1d * (var_xg10_op_dn8 - var_xg2eff_op_dn8));
        var_temp_dn9 = (var_keq_1d * (var_xg10_op_dn9 - var_xg2eff_op_dn9));

        let assign44210_e49341: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        var_guard1355 = assign44210_e49341;

        let (assign44220_e49362, assign44220_e49362_d_n4, assign44220_e49362_d_n6, assign44220_e49362_d_n7, assign44220_e49362_d_n8, assign44220_e49362_d_n9,) = {
    if (var_guard1355 != 0.0) {
        let assign44220_e49346: f64 = (var_temp + var_emin);
        let assign44220_e49349: f64 = (var_temp - var_emin);
        let assign44220_e49352: f64 = (var_temp - var_emin);
        let assign44220_e49353: f64 = (assign44220_e49349 * assign44220_e49352);
        let assign44220_e49356: f64 = (var_emin * var_emin);
        let assign44220_e49357: f64 = (assign44220_e49353 + assign44220_e49356);
        let assign44220_e49358: f64 = (assign44220_e49357).sqrt();
        let assign44220_e49359: f64 = (assign44220_e49346 + assign44220_e49358);
        let assign44220_e49360: f64 = (0.5 * assign44220_e49359);
        (assign44220_e49360, (0.5 * ((var_temp_dn4 + var_emin_dn4) + (((((var_temp_dn4 - var_emin_dn4) * assign44220_e49352) + (assign44220_e49349 * (var_temp_dn4 - var_emin_dn4))) + ((var_emin_dn4 * var_emin) + (var_emin * var_emin_dn4))) / (2.0 * assign44220_e49358)))), (0.5 * ((var_temp_dn6 + var_emin_dn6) + (((((var_temp_dn6 - var_emin_dn6) * assign44220_e49352) + (assign44220_e49349 * (var_temp_dn6 - var_emin_dn6))) + ((var_emin_dn6 * var_emin) + (var_emin * var_emin_dn6))) / (2.0 * assign44220_e49358)))), (0.5 * ((var_temp_dn7 + var_emin_dn7) + (((((var_temp_dn7 - var_emin_dn7) * assign44220_e49352) + (assign44220_e49349 * (var_temp_dn7 - var_emin_dn7))) + ((var_emin_dn7 * var_emin) + (var_emin * var_emin_dn7))) / (2.0 * assign44220_e49358)))), (0.5 * ((var_temp_dn8 + var_emin_dn8) + (((((var_temp_dn8 - var_emin_dn8) * assign44220_e49352) + (assign44220_e49349 * (var_temp_dn8 - var_emin_dn8))) + ((var_emin_dn8 * var_emin) + (var_emin * var_emin_dn8))) / (2.0 * assign44220_e49358)))), (0.5 * ((var_temp_dn9 + var_emin_dn9) + (((((var_temp_dn9 - var_emin_dn9) * assign44220_e49352) + (assign44220_e49349 * (var_temp_dn9 - var_emin_dn9))) + ((var_emin_dn9 * var_emin) + (var_emin * var_emin_dn9))) / (2.0 * assign44220_e49358)))),)
    } else {
        (var_e1_op, var_e1_op_dn4, var_e1_op_dn6, var_e1_op_dn7, var_e1_op_dn8, var_e1_op_dn9,)
    }
};
        var_e1_op = assign44220_e49362;
        var_e1_op_dn4 = assign44220_e49362_d_n4;
        var_e1_op_dn6 = assign44220_e49362_d_n6;
        var_e1_op_dn7 = assign44220_e49362_d_n7;
        var_e1_op_dn8 = assign44220_e49362_d_n8;
        var_e1_op_dn9 = assign44220_e49362_d_n9;

        *var_a0_csisq_op_slot = var_a0_csisq_op;
        *var_a0_csisq_op_dn4_slot = var_a0_csisq_op_dn4;
        *var_a0_csisq_op_dn6_slot = var_a0_csisq_op_dn6;
        *var_a0_csisq_op_dn7_slot = var_a0_csisq_op_dn7;
        *var_a0_csisq_op_dn8_slot = var_a0_csisq_op_dn8;
        *var_a0_csisq_op_dn9_slot = var_a0_csisq_op_dn9;
        *var_dvfbpdep_op_slot = var_dvfbpdep_op;
        *var_dvfbpdep_op_dn4_slot = var_dvfbpdep_op_dn4;
        *var_dvfbpdep_op_dn6_slot = var_dvfbpdep_op_dn6;
        *var_dvfbpdep_op_dn7_slot = var_dvfbpdep_op_dn7;
        *var_dvfbpdep_op_dn8_slot = var_dvfbpdep_op_dn8;
        *var_dvfbpdep_op_dn9_slot = var_dvfbpdep_op_dn9;
        *var_dxdsx_op_slot = var_dxdsx_op;
        *var_dxdsx_op_dn4_slot = var_dxdsx_op_dn4;
        *var_dxdsx_op_dn6_slot = var_dxdsx_op_dn6;
        *var_dxdsx_op_dn7_slot = var_dxdsx_op_dn7;
        *var_dxdsx_op_dn8_slot = var_dxdsx_op_dn8;
        *var_dxdsx_op_dn9_slot = var_dxdsx_op_dn9;
        *var_e1_op_slot = var_e1_op;
        *var_e1_op_dn4_slot = var_e1_op_dn4;
        *var_e1_op_dn6_slot = var_e1_op_dn6;
        *var_e1_op_dn7_slot = var_e1_op_dn7;
        *var_e1_op_dn8_slot = var_e1_op_dn8;
        *var_e1_op_dn9_slot = var_e1_op_dn9;
        *var_guard1350_slot = var_guard1350;
        *var_guard1351_slot = var_guard1351;
        *var_guard1352_slot = var_guard1352;
        *var_guard1353_slot = var_guard1353;
        *var_guard1354_slot = var_guard1354;
        *var_guard1355_slot = var_guard1355;
        *var_inv_phit_op_slot = var_inv_phit_op;
        *var_inv_phit_op_dn4_slot = var_inv_phit_op_dn4;
        *var_inv_phit_op_dn6_slot = var_inv_phit_op_dn6;
        *var_inv_phit_op_dn7_slot = var_inv_phit_op_dn7;
        *var_inv_phit_op_dn8_slot = var_inv_phit_op_dn8;
        *var_inv_phit_op_dn9_slot = var_inv_phit_op_dn9;
        *var_neff_op_slot = var_neff_op;
        *var_neff_op_dn4_slot = var_neff_op_dn4;
        *var_neff_op_dn6_slot = var_neff_op_dn6;
        *var_neff_op_dn7_slot = var_neff_op_dn7;
        *var_neff_op_dn8_slot = var_neff_op_dn8;
        *var_neff_op_dn9_slot = var_neff_op_dn9;
        *var_qq_op_slot = var_qq_op;
        *var_qq_op_dn4_slot = var_qq_op_dn4;
        *var_qq_op_dn6_slot = var_qq_op_dn6;
        *var_qq_op_dn7_slot = var_qq_op_dn7;
        *var_qq_op_dn8_slot = var_qq_op_dn8;
        *var_qq_op_dn9_slot = var_qq_op_dn9;
        *var_r1init_op_slot = var_r1init_op;
        *var_r1init_op_dn4_slot = var_r1init_op_dn4;
        *var_r1init_op_dn6_slot = var_r1init_op_dn6;
        *var_r1init_op_dn7_slot = var_r1init_op_dn7;
        *var_r1init_op_dn8_slot = var_r1init_op_dn8;
        *var_r1init_op_dn9_slot = var_r1init_op_dn9;
        *var_r2init_op_slot = var_r2init_op;
        *var_r2init_op_dn4_slot = var_r2init_op_dn4;
        *var_r2init_op_dn6_slot = var_r2init_op_dn6;
        *var_r2init_op_dn7_slot = var_r2init_op_dn7;
        *var_r2init_op_dn8_slot = var_r2init_op_dn8;
        *var_r2init_op_dn9_slot = var_r2init_op_dn9;
        *var_temp_slot = var_temp;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_vfb1_op_slot = var_vfb1_op;
        *var_vfb1_op_dn4_slot = var_vfb1_op_dn4;
        *var_vfb1_op_dn6_slot = var_vfb1_op_dn6;
        *var_vfb1_op_dn7_slot = var_vfb1_op_dn7;
        *var_vfb1_op_dn8_slot = var_vfb1_op_dn8;
        *var_vfb1_op_dn9_slot = var_vfb1_op_dn9;
        *var_vfb2_op_slot = var_vfb2_op;
        *var_vfb2_op_dn4_slot = var_vfb2_op_dn4;
        *var_vfb2_op_dn6_slot = var_vfb2_op_dn6;
        *var_vfb2_op_dn7_slot = var_vfb2_op_dn7;
        *var_vfb2_op_dn8_slot = var_vfb2_op_dn8;
        *var_vfb2_op_dn9_slot = var_vfb2_op_dn9;
        *var_vthinit_op_slot = var_vthinit_op;
        *var_vthinit_op_dn4_slot = var_vthinit_op_dn4;
        *var_vthinit_op_dn6_slot = var_vthinit_op_dn6;
        *var_vthinit_op_dn7_slot = var_vthinit_op_dn7;
        *var_vthinit_op_dn8_slot = var_vthinit_op_dn8;
        *var_vthinit_op_dn9_slot = var_vthinit_op_dn9;
        *var_x1init_op_slot = var_x1init_op;
        *var_x1init_op_dn4_slot = var_x1init_op_dn4;
        *var_x1init_op_dn6_slot = var_x1init_op_dn6;
        *var_x1init_op_dn7_slot = var_x1init_op_dn7;
        *var_x1init_op_dn8_slot = var_x1init_op_dn8;
        *var_x1init_op_dn9_slot = var_x1init_op_dn9;
        *var_x2init_op_slot = var_x2init_op;
        *var_x2init_op_dn4_slot = var_x2init_op_dn4;
        *var_x2init_op_dn6_slot = var_x2init_op_dn6;
        *var_x2init_op_dn7_slot = var_x2init_op_dn7;
        *var_x2init_op_dn8_slot = var_x2init_op_dn8;
        *var_x2init_op_dn9_slot = var_x2init_op_dn9;
        *var_xd0_op_slot = var_xd0_op;
        *var_xd0_op_dn4_slot = var_xd0_op_dn4;
        *var_xd0_op_dn6_slot = var_xd0_op_dn6;
        *var_xd0_op_dn7_slot = var_xd0_op_dn7;
        *var_xd0_op_dn8_slot = var_xd0_op_dn8;
        *var_xd0_op_dn9_slot = var_xd0_op_dn9;
        *var_xd_op_slot = var_xd_op;
        *var_xd_op_dn4_slot = var_xd_op_dn4;
        *var_xd_op_dn6_slot = var_xd_op_dn6;
        *var_xd_op_dn7_slot = var_xd_op_dn7;
        *var_xd_op_dn8_slot = var_xd_op_dn8;
        *var_xd_op_dn9_slot = var_xd_op_dn9;
        *var_xdsx_op_slot = var_xdsx_op;
        *var_xdsx_op_dn4_slot = var_xdsx_op_dn4;
        *var_xdsx_op_dn6_slot = var_xdsx_op_dn6;
        *var_xdsx_op_dn7_slot = var_xdsx_op_dn7;
        *var_xdsx_op_dn8_slot = var_xdsx_op_dn8;
        *var_xdsx_op_dn9_slot = var_xdsx_op_dn9;
        *var_xg10_op_slot = var_xg10_op;
        *var_xg10_op_dn4_slot = var_xg10_op_dn4;
        *var_xg10_op_dn6_slot = var_xg10_op_dn6;
        *var_xg10_op_dn7_slot = var_xg10_op_dn7;
        *var_xg10_op_dn8_slot = var_xg10_op_dn8;
        *var_xg10_op_dn9_slot = var_xg10_op_dn9;
        *var_xg1thinit_op_slot = var_xg1thinit_op;
        *var_xg1thinit_op_dn4_slot = var_xg1thinit_op_dn4;
        *var_xg1thinit_op_dn6_slot = var_xg1thinit_op_dn6;
        *var_xg1thinit_op_dn7_slot = var_xg1thinit_op_dn7;
        *var_xg1thinit_op_dn8_slot = var_xg1thinit_op_dn8;
        *var_xg1thinit_op_dn9_slot = var_xg1thinit_op_dn9;
        *var_xg20_op_slot = var_xg20_op;
        *var_xg20_op_dn4_slot = var_xg20_op_dn4;
        *var_xg20_op_dn6_slot = var_xg20_op_dn6;
        *var_xg20_op_dn7_slot = var_xg20_op_dn7;
        *var_xg20_op_dn8_slot = var_xg20_op_dn8;
        *var_xg20_op_dn9_slot = var_xg20_op_dn9;
        *var_xg2eff_op_slot = var_xg2eff_op;
        *var_xg2eff_op_dn4_slot = var_xg2eff_op_dn4;
        *var_xg2eff_op_dn6_slot = var_xg2eff_op_dn6;
        *var_xg2eff_op_dn7_slot = var_xg2eff_op_dn7;
        *var_xg2eff_op_dn8_slot = var_xg2eff_op_dn8;
        *var_xg2eff_op_dn9_slot = var_xg2eff_op_dn9;
        *var_xsddep_op_slot = var_xsddep_op;
        *var_xsddep_op_dn4_slot = var_xsddep_op_dn4;
        *var_xsddep_op_dn6_slot = var_xsddep_op_dn6;
        *var_xsddep_op_dn7_slot = var_xsddep_op_dn7;
        *var_xsddep_op_dn8_slot = var_xsddep_op_dn8;
        *var_xsddep_op_dn9_slot = var_xsddep_op_dn9;
        *var_xth1init_op_slot = var_xth1init_op;
        *var_xth1init_op_dn4_slot = var_xth1init_op_dn4;
        *var_xth1init_op_dn6_slot = var_xth1init_op_dn6;
        *var_xth1init_op_dn7_slot = var_xth1init_op_dn7;
        *var_xth1init_op_dn8_slot = var_xth1init_op_dn8;
        *var_xth1init_op_dn9_slot = var_xth1init_op_dn9;
        *var_xth2init_op_slot = var_xth2init_op;
        *var_xth2init_op_dn4_slot = var_xth2init_op_dn4;
        *var_xth2init_op_dn6_slot = var_xth2init_op_dn6;
        *var_xth2init_op_dn7_slot = var_xth2init_op_dn7;
        *var_xth2init_op_dn8_slot = var_xth2init_op_dn8;
        *var_xth2init_op_dn9_slot = var_xth2init_op_dn9;
        *var_xth_1d_op_slot = var_xth_1d_op;
        *var_xth_1d_op_dn4_slot = var_xth_1d_op_dn4;
        *var_xth_1d_op_dn6_slot = var_xth_1d_op_dn6;
        *var_xth_1d_op_dn7_slot = var_xth_1d_op_dn7;
        *var_xth_1d_op_dn8_slot = var_xth_1d_op_dn8;
        *var_xth_1d_op_dn9_slot = var_xth_1d_op_dn9;
    }

    pub(super) fn stamp_transient_block_122(
        p: &Parameters,
        var_a0_ac: f64,
        var_a0_ac_dn4: f64,
        var_a0_ac_dn6: f64,
        var_a0_ac_dn7: f64,
        var_a0_ac_dn8: f64,
        var_a0_ac_dn9: f64,
        var_cfdl_i: f64,
        var_cfdlb_i: f64,
        var_cic1_i: f64,
        var_diff_min_ac: f64,
        var_diff_min_ac_dn4: f64,
        var_diff_min_ac_dn6: f64,
        var_diff_min_ac_dn7: f64,
        var_diff_min_ac_dn8: f64,
        var_diff_min_ac_dn9: f64,
        var_e1_op: f64,
        var_e1_op_dn4: f64,
        var_e1_op_dn6: f64,
        var_e1_op_dn7: f64,
        var_e1_op_dn8: f64,
        var_e1_op_dn9: f64,
        var_emin: f64,
        var_emin_dn4: f64,
        var_emin_dn6: f64,
        var_emin_dn7: f64,
        var_emin_dn8: f64,
        var_emin_dn9: f64,
        var_guard1355: f64,
        var_k1_1d: f64,
        var_k1_ac: f64,
        var_k1_ac_dn4: f64,
        var_k1_ac_dn6: f64,
        var_k1_ac_dn7: f64,
        var_k1_ac_dn8: f64,
        var_k1_ac_dn9: f64,
        var_k2_1d: f64,
        var_k2_ac: f64,
        var_k2_ac_dn4: f64,
        var_k2_ac_dn6: f64,
        var_k2_ac_dn7: f64,
        var_k2_ac_dn8: f64,
        var_k2_ac_dn9: f64,
        var_keq_1d: f64,
        var_pscedlb_i: f64,
        var_qq_op: f64,
        var_qq_op_dn4: f64,
        var_qq_op_dn6: f64,
        var_qq_op_dn7: f64,
        var_qq_op_dn8: f64,
        var_qq_op_dn9: f64,
        var_xd0_op: f64,
        var_xd0_op_dn4: f64,
        var_xd0_op_dn6: f64,
        var_xd0_op_dn7: f64,
        var_xd0_op_dn8: f64,
        var_xd0_op_dn9: f64,
        var_xdsx_op: f64,
        var_xdsx_op_dn4: f64,
        var_xdsx_op_dn6: f64,
        var_xdsx_op_dn7: f64,
        var_xdsx_op_dn8: f64,
        var_xdsx_op_dn9: f64,
        var_xg10_op: f64,
        var_xg10_op_dn4: f64,
        var_xg10_op_dn6: f64,
        var_xg10_op_dn7: f64,
        var_xg10_op_dn8: f64,
        var_xg10_op_dn9: f64,
        var_xg20_op: f64,
        var_xg20_op_dn4: f64,
        var_xg20_op_dn6: f64,
        var_xg20_op_dn7: f64,
        var_xg20_op_dn8: f64,
        var_xg20_op_dn9: f64,
        var_xg2_ac: f64,
        var_xg2_ac_dn4: f64,
        var_xg2_ac_dn6: f64,
        var_xg2_ac_dn7: f64,
        var_xg2_ac_dn8: f64,
        var_xg2_ac_dn9: f64,
        var_xg2eff_op: f64,
        var_xg2eff_op_dn4: f64,
        var_xg2eff_op_dn6: f64,
        var_xg2eff_op_dn7: f64,
        var_xg2eff_op_dn8: f64,
        var_xg2eff_op_dn9: f64,
        var_xg2x_ac: f64,
        var_xg2x_ac_dn4: f64,
        var_xg2x_ac_dn6: f64,
        var_xg2x_ac_dn7: f64,
        var_xg2x_ac_dn8: f64,
        var_xg2x_ac_dn9: f64,
        var_xsddep_op: f64,
        var_xsddep_op_dn4: f64,
        var_xsddep_op_dn6: f64,
        var_xsddep_op_dn7: f64,
        var_xsddep_op_dn8: f64,
        var_xsddep_op_dn9: f64,
        var_xth_1d_op: f64,
        var_xth_1d_op_dn4: f64,
        var_xth_1d_op_dn6: f64,
        var_xth_1d_op_dn7: f64,
        var_xth_1d_op_dn8: f64,
        var_xth_1d_op_dn9: f64,
        var_dleff_op_slot: &mut f64,
        var_dleff_op_dn4_slot: &mut f64,
        var_dleff_op_dn6_slot: &mut f64,
        var_dleff_op_dn7_slot: &mut f64,
        var_dleff_op_dn8_slot: &mut f64,
        var_dleff_op_dn9_slot: &mut f64,
        var_dx_wi_1d_op_slot: &mut f64,
        var_dx_wi_1d_op_dn4_slot: &mut f64,
        var_dx_wi_1d_op_dn6_slot: &mut f64,
        var_dx_wi_1d_op_dn7_slot: &mut f64,
        var_dx_wi_1d_op_dn8_slot: &mut f64,
        var_dx_wi_1d_op_dn9_slot: &mut f64,
        var_e2_op_slot: &mut f64,
        var_e2_op_dn4_slot: &mut f64,
        var_e2_op_dn6_slot: &mut f64,
        var_e2_op_dn7_slot: &mut f64,
        var_e2_op_dn8_slot: &mut f64,
        var_e2_op_dn9_slot: &mut f64,
        var_guard1356_slot: &mut f64,
        var_guard1357_slot: &mut f64,
        var_guard1358_slot: &mut f64,
        var_guard1360_slot: &mut f64,
        var_k1_1d_qm_op_slot: &mut f64,
        var_k1_1d_qm_op_dn4_slot: &mut f64,
        var_k1_1d_qm_op_dn6_slot: &mut f64,
        var_k1_1d_qm_op_dn7_slot: &mut f64,
        var_k1_1d_qm_op_dn8_slot: &mut f64,
        var_k1_1d_qm_op_dn9_slot: &mut f64,
        var_k2_1d_qm_op_slot: &mut f64,
        var_k2_1d_qm_op_dn4_slot: &mut f64,
        var_k2_1d_qm_op_dn6_slot: &mut f64,
        var_k2_1d_qm_op_dn7_slot: &mut f64,
        var_k2_1d_qm_op_dn8_slot: &mut f64,
        var_k2_1d_qm_op_dn9_slot: &mut f64,
        var_keq_1d_qm_op_slot: &mut f64,
        var_keq_1d_qm_op_dn4_slot: &mut f64,
        var_keq_1d_qm_op_dn6_slot: &mut f64,
        var_keq_1d_qm_op_dn7_slot: &mut f64,
        var_keq_1d_qm_op_dn8_slot: &mut f64,
        var_keq_1d_qm_op_dn9_slot: &mut f64,
        var_r1init_op_slot: &mut f64,
        var_r1init_op_dn4_slot: &mut f64,
        var_r1init_op_dn6_slot: &mut f64,
        var_r1init_op_dn7_slot: &mut f64,
        var_r1init_op_dn8_slot: &mut f64,
        var_r1init_op_dn9_slot: &mut f64,
        var_r2init_op_slot: &mut f64,
        var_r2init_op_dn4_slot: &mut f64,
        var_r2init_op_dn6_slot: &mut f64,
        var_r2init_op_dn7_slot: &mut f64,
        var_r2init_op_dn8_slot: &mut f64,
        var_r2init_op_dn9_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp3_slot: &mut f64,
        var_temp3_dn4_slot: &mut f64,
        var_temp3_dn6_slot: &mut f64,
        var_temp3_dn7_slot: &mut f64,
        var_temp3_dn8_slot: &mut f64,
        var_temp3_dn9_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_x1init_op_slot: &mut f64,
        var_x1init_op_dn4_slot: &mut f64,
        var_x1init_op_dn6_slot: &mut f64,
        var_x1init_op_dn7_slot: &mut f64,
        var_x1init_op_dn8_slot: &mut f64,
        var_x1init_op_dn9_slot: &mut f64,
        var_x2init_op_slot: &mut f64,
        var_x2init_op_dn4_slot: &mut f64,
        var_x2init_op_dn6_slot: &mut f64,
        var_x2init_op_dn7_slot: &mut f64,
        var_x2init_op_dn8_slot: &mut f64,
        var_x2init_op_dn9_slot: &mut f64,
        var_x_1d_op_slot: &mut f64,
        var_x_1d_op_dn4_slot: &mut f64,
        var_x_1d_op_dn6_slot: &mut f64,
        var_x_1d_op_dn7_slot: &mut f64,
        var_x_1d_op_dn8_slot: &mut f64,
        var_x_1d_op_dn9_slot: &mut f64,
        var_x_wi_1d_op_slot: &mut f64,
        var_x_wi_1d_op_dn4_slot: &mut f64,
        var_x_wi_1d_op_dn6_slot: &mut f64,
        var_x_wi_1d_op_dn7_slot: &mut f64,
        var_x_wi_1d_op_dn8_slot: &mut f64,
        var_x_wi_1d_op_dn9_slot: &mut f64,
        var_xg1thinit_op_slot: &mut f64,
        var_xg1thinit_op_dn4_slot: &mut f64,
        var_xg1thinit_op_dn6_slot: &mut f64,
        var_xg1thinit_op_dn7_slot: &mut f64,
        var_xg1thinit_op_dn8_slot: &mut f64,
        var_xg1thinit_op_dn9_slot: &mut f64,
        var_xth1init_op_slot: &mut f64,
        var_xth1init_op_dn4_slot: &mut f64,
        var_xth1init_op_dn6_slot: &mut f64,
        var_xth1init_op_dn7_slot: &mut f64,
        var_xth1init_op_dn8_slot: &mut f64,
        var_xth1init_op_dn9_slot: &mut f64,
        var_xth2init_op_slot: &mut f64,
        var_xth2init_op_dn4_slot: &mut f64,
        var_xth2init_op_dn6_slot: &mut f64,
        var_xth2init_op_dn7_slot: &mut f64,
        var_xth2init_op_dn8_slot: &mut f64,
        var_xth2init_op_dn9_slot: &mut f64,
    ) {
        let mut var_dleff_op: f64 = *var_dleff_op_slot;
        let mut var_dleff_op_dn4: f64 = *var_dleff_op_dn4_slot;
        let mut var_dleff_op_dn6: f64 = *var_dleff_op_dn6_slot;
        let mut var_dleff_op_dn7: f64 = *var_dleff_op_dn7_slot;
        let mut var_dleff_op_dn8: f64 = *var_dleff_op_dn8_slot;
        let mut var_dleff_op_dn9: f64 = *var_dleff_op_dn9_slot;
        let mut var_dx_wi_1d_op: f64 = *var_dx_wi_1d_op_slot;
        let mut var_dx_wi_1d_op_dn4: f64 = *var_dx_wi_1d_op_dn4_slot;
        let mut var_dx_wi_1d_op_dn6: f64 = *var_dx_wi_1d_op_dn6_slot;
        let mut var_dx_wi_1d_op_dn7: f64 = *var_dx_wi_1d_op_dn7_slot;
        let mut var_dx_wi_1d_op_dn8: f64 = *var_dx_wi_1d_op_dn8_slot;
        let mut var_dx_wi_1d_op_dn9: f64 = *var_dx_wi_1d_op_dn9_slot;
        let mut var_e2_op: f64 = *var_e2_op_slot;
        let mut var_e2_op_dn4: f64 = *var_e2_op_dn4_slot;
        let mut var_e2_op_dn6: f64 = *var_e2_op_dn6_slot;
        let mut var_e2_op_dn7: f64 = *var_e2_op_dn7_slot;
        let mut var_e2_op_dn8: f64 = *var_e2_op_dn8_slot;
        let mut var_e2_op_dn9: f64 = *var_e2_op_dn9_slot;
        let mut var_guard1356: f64 = *var_guard1356_slot;
        let mut var_guard1357: f64 = *var_guard1357_slot;
        let mut var_guard1358: f64 = *var_guard1358_slot;
        let mut var_guard1360: f64 = *var_guard1360_slot;
        let mut var_k1_1d_qm_op: f64 = *var_k1_1d_qm_op_slot;
        let mut var_k1_1d_qm_op_dn4: f64 = *var_k1_1d_qm_op_dn4_slot;
        let mut var_k1_1d_qm_op_dn6: f64 = *var_k1_1d_qm_op_dn6_slot;
        let mut var_k1_1d_qm_op_dn7: f64 = *var_k1_1d_qm_op_dn7_slot;
        let mut var_k1_1d_qm_op_dn8: f64 = *var_k1_1d_qm_op_dn8_slot;
        let mut var_k1_1d_qm_op_dn9: f64 = *var_k1_1d_qm_op_dn9_slot;
        let mut var_k2_1d_qm_op: f64 = *var_k2_1d_qm_op_slot;
        let mut var_k2_1d_qm_op_dn4: f64 = *var_k2_1d_qm_op_dn4_slot;
        let mut var_k2_1d_qm_op_dn6: f64 = *var_k2_1d_qm_op_dn6_slot;
        let mut var_k2_1d_qm_op_dn7: f64 = *var_k2_1d_qm_op_dn7_slot;
        let mut var_k2_1d_qm_op_dn8: f64 = *var_k2_1d_qm_op_dn8_slot;
        let mut var_k2_1d_qm_op_dn9: f64 = *var_k2_1d_qm_op_dn9_slot;
        let mut var_keq_1d_qm_op: f64 = *var_keq_1d_qm_op_slot;
        let mut var_keq_1d_qm_op_dn4: f64 = *var_keq_1d_qm_op_dn4_slot;
        let mut var_keq_1d_qm_op_dn6: f64 = *var_keq_1d_qm_op_dn6_slot;
        let mut var_keq_1d_qm_op_dn7: f64 = *var_keq_1d_qm_op_dn7_slot;
        let mut var_keq_1d_qm_op_dn8: f64 = *var_keq_1d_qm_op_dn8_slot;
        let mut var_keq_1d_qm_op_dn9: f64 = *var_keq_1d_qm_op_dn9_slot;
        let mut var_r1init_op: f64 = *var_r1init_op_slot;
        let mut var_r1init_op_dn4: f64 = *var_r1init_op_dn4_slot;
        let mut var_r1init_op_dn6: f64 = *var_r1init_op_dn6_slot;
        let mut var_r1init_op_dn7: f64 = *var_r1init_op_dn7_slot;
        let mut var_r1init_op_dn8: f64 = *var_r1init_op_dn8_slot;
        let mut var_r1init_op_dn9: f64 = *var_r1init_op_dn9_slot;
        let mut var_r2init_op: f64 = *var_r2init_op_slot;
        let mut var_r2init_op_dn4: f64 = *var_r2init_op_dn4_slot;
        let mut var_r2init_op_dn6: f64 = *var_r2init_op_dn6_slot;
        let mut var_r2init_op_dn7: f64 = *var_r2init_op_dn7_slot;
        let mut var_r2init_op_dn8: f64 = *var_r2init_op_dn8_slot;
        let mut var_r2init_op_dn9: f64 = *var_r2init_op_dn9_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp3: f64 = *var_temp3_slot;
        let mut var_temp3_dn4: f64 = *var_temp3_dn4_slot;
        let mut var_temp3_dn6: f64 = *var_temp3_dn6_slot;
        let mut var_temp3_dn7: f64 = *var_temp3_dn7_slot;
        let mut var_temp3_dn8: f64 = *var_temp3_dn8_slot;
        let mut var_temp3_dn9: f64 = *var_temp3_dn9_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_x1init_op: f64 = *var_x1init_op_slot;
        let mut var_x1init_op_dn4: f64 = *var_x1init_op_dn4_slot;
        let mut var_x1init_op_dn6: f64 = *var_x1init_op_dn6_slot;
        let mut var_x1init_op_dn7: f64 = *var_x1init_op_dn7_slot;
        let mut var_x1init_op_dn8: f64 = *var_x1init_op_dn8_slot;
        let mut var_x1init_op_dn9: f64 = *var_x1init_op_dn9_slot;
        let mut var_x2init_op: f64 = *var_x2init_op_slot;
        let mut var_x2init_op_dn4: f64 = *var_x2init_op_dn4_slot;
        let mut var_x2init_op_dn6: f64 = *var_x2init_op_dn6_slot;
        let mut var_x2init_op_dn7: f64 = *var_x2init_op_dn7_slot;
        let mut var_x2init_op_dn8: f64 = *var_x2init_op_dn8_slot;
        let mut var_x2init_op_dn9: f64 = *var_x2init_op_dn9_slot;
        let mut var_x_1d_op: f64 = *var_x_1d_op_slot;
        let mut var_x_1d_op_dn4: f64 = *var_x_1d_op_dn4_slot;
        let mut var_x_1d_op_dn6: f64 = *var_x_1d_op_dn6_slot;
        let mut var_x_1d_op_dn7: f64 = *var_x_1d_op_dn7_slot;
        let mut var_x_1d_op_dn8: f64 = *var_x_1d_op_dn8_slot;
        let mut var_x_1d_op_dn9: f64 = *var_x_1d_op_dn9_slot;
        let mut var_x_wi_1d_op: f64 = *var_x_wi_1d_op_slot;
        let mut var_x_wi_1d_op_dn4: f64 = *var_x_wi_1d_op_dn4_slot;
        let mut var_x_wi_1d_op_dn6: f64 = *var_x_wi_1d_op_dn6_slot;
        let mut var_x_wi_1d_op_dn7: f64 = *var_x_wi_1d_op_dn7_slot;
        let mut var_x_wi_1d_op_dn8: f64 = *var_x_wi_1d_op_dn8_slot;
        let mut var_x_wi_1d_op_dn9: f64 = *var_x_wi_1d_op_dn9_slot;
        let mut var_xg1thinit_op: f64 = *var_xg1thinit_op_slot;
        let mut var_xg1thinit_op_dn4: f64 = *var_xg1thinit_op_dn4_slot;
        let mut var_xg1thinit_op_dn6: f64 = *var_xg1thinit_op_dn6_slot;
        let mut var_xg1thinit_op_dn7: f64 = *var_xg1thinit_op_dn7_slot;
        let mut var_xg1thinit_op_dn8: f64 = *var_xg1thinit_op_dn8_slot;
        let mut var_xg1thinit_op_dn9: f64 = *var_xg1thinit_op_dn9_slot;
        let mut var_xth1init_op: f64 = *var_xth1init_op_slot;
        let mut var_xth1init_op_dn4: f64 = *var_xth1init_op_dn4_slot;
        let mut var_xth1init_op_dn6: f64 = *var_xth1init_op_dn6_slot;
        let mut var_xth1init_op_dn7: f64 = *var_xth1init_op_dn7_slot;
        let mut var_xth1init_op_dn8: f64 = *var_xth1init_op_dn8_slot;
        let mut var_xth1init_op_dn9: f64 = *var_xth1init_op_dn9_slot;
        let mut var_xth2init_op: f64 = *var_xth2init_op_slot;
        let mut var_xth2init_op_dn4: f64 = *var_xth2init_op_dn4_slot;
        let mut var_xth2init_op_dn6: f64 = *var_xth2init_op_dn6_slot;
        let mut var_xth2init_op_dn7: f64 = *var_xth2init_op_dn7_slot;
        let mut var_xth2init_op_dn8: f64 = *var_xth2init_op_dn8_slot;
        let mut var_xth2init_op_dn9: f64 = *var_xth2init_op_dn9_slot;

        let (assign44230_e49386, assign44230_e49386_d_n4, assign44230_e49386_d_n6, assign44230_e49386_d_n7, assign44230_e49386_d_n8, assign44230_e49386_d_n9,) = {
    if (var_guard1355 != 0.0) {
        let assign44230_e49366: f64 = (-var_temp);
        let assign44230_e49368: f64 = (assign44230_e49366 + var_emin);
        let assign44230_e49370: f64 = (-var_temp);
        let assign44230_e49372: f64 = (assign44230_e49370 - var_emin);
        let assign44230_e49374: f64 = (-var_temp);
        let assign44230_e49376: f64 = (assign44230_e49374 - var_emin);
        let assign44230_e49377: f64 = (assign44230_e49372 * assign44230_e49376);
        let assign44230_e49380: f64 = (var_emin * var_emin);
        let assign44230_e49381: f64 = (assign44230_e49377 + assign44230_e49380);
        let assign44230_e49382: f64 = (assign44230_e49381).sqrt();
        let assign44230_e49383: f64 = (assign44230_e49368 + assign44230_e49382);
        let assign44230_e49384: f64 = (0.5 * assign44230_e49383);
        (assign44230_e49384, (0.5 * (((-var_temp_dn4) + var_emin_dn4) + ((((((-var_temp_dn4) - var_emin_dn4) * assign44230_e49376) + (assign44230_e49372 * ((-var_temp_dn4) - var_emin_dn4))) + ((var_emin_dn4 * var_emin) + (var_emin * var_emin_dn4))) / (2.0 * assign44230_e49382)))), (0.5 * (((-var_temp_dn6) + var_emin_dn6) + ((((((-var_temp_dn6) - var_emin_dn6) * assign44230_e49376) + (assign44230_e49372 * ((-var_temp_dn6) - var_emin_dn6))) + ((var_emin_dn6 * var_emin) + (var_emin * var_emin_dn6))) / (2.0 * assign44230_e49382)))), (0.5 * (((-var_temp_dn7) + var_emin_dn7) + ((((((-var_temp_dn7) - var_emin_dn7) * assign44230_e49376) + (assign44230_e49372 * ((-var_temp_dn7) - var_emin_dn7))) + ((var_emin_dn7 * var_emin) + (var_emin * var_emin_dn7))) / (2.0 * assign44230_e49382)))), (0.5 * (((-var_temp_dn8) + var_emin_dn8) + ((((((-var_temp_dn8) - var_emin_dn8) * assign44230_e49376) + (assign44230_e49372 * ((-var_temp_dn8) - var_emin_dn8))) + ((var_emin_dn8 * var_emin) + (var_emin * var_emin_dn8))) / (2.0 * assign44230_e49382)))), (0.5 * (((-var_temp_dn9) + var_emin_dn9) + ((((((-var_temp_dn9) - var_emin_dn9) * assign44230_e49376) + (assign44230_e49372 * ((-var_temp_dn9) - var_emin_dn9))) + ((var_emin_dn9 * var_emin) + (var_emin * var_emin_dn9))) / (2.0 * assign44230_e49382)))),)
    } else {
        (var_e2_op, var_e2_op_dn4, var_e2_op_dn6, var_e2_op_dn7, var_e2_op_dn8, var_e2_op_dn9,)
    }
};
        var_e2_op = assign44230_e49386;
        var_e2_op_dn4 = assign44230_e49386_d_n4;
        var_e2_op_dn6 = assign44230_e49386_d_n6;
        var_e2_op_dn7 = assign44230_e49386_d_n7;
        var_e2_op_dn8 = assign44230_e49386_d_n8;
        var_e2_op_dn9 = assign44230_e49386_d_n9;

        let (assign44240_e49397, assign44240_e49397_d_n4, assign44240_e49397_d_n6, assign44240_e49397_d_n7, assign44240_e49397_d_n8, assign44240_e49397_d_n9,) = {
    if (var_guard1355 != 0.0) {
        let assign44240_e49390: f64 = (-0.3333333333333);
        let assign44240_e49392: f64 = (var_e1_op).ln();
        let assign44240_e49393: f64 = (assign44240_e49390 * assign44240_e49392);
        let assign44240_e49394: f64 = (assign44240_e49393).exp();
        let assign44240_e49395: f64 = (var_qq_op * assign44240_e49394);
        (assign44240_e49395, ((var_qq_op_dn4 * assign44240_e49394) + (var_qq_op * (assign44240_e49394 * (assign44240_e49390 * (var_e1_op_dn4 / var_e1_op))))), ((var_qq_op_dn6 * assign44240_e49394) + (var_qq_op * (assign44240_e49394 * (assign44240_e49390 * (var_e1_op_dn6 / var_e1_op))))), ((var_qq_op_dn7 * assign44240_e49394) + (var_qq_op * (assign44240_e49394 * (assign44240_e49390 * (var_e1_op_dn7 / var_e1_op))))), ((var_qq_op_dn8 * assign44240_e49394) + (var_qq_op * (assign44240_e49394 * (assign44240_e49390 * (var_e1_op_dn8 / var_e1_op))))), ((var_qq_op_dn9 * assign44240_e49394) + (var_qq_op * (assign44240_e49394 * (assign44240_e49390 * (var_e1_op_dn9 / var_e1_op))))),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign44240_e49397;
        var_temp1_dn4 = assign44240_e49397_d_n4;
        var_temp1_dn6 = assign44240_e49397_d_n6;
        var_temp1_dn7 = assign44240_e49397_d_n7;
        var_temp1_dn8 = assign44240_e49397_d_n8;
        var_temp1_dn9 = assign44240_e49397_d_n9;

        let (assign44250_e49408, assign44250_e49408_d_n4, assign44250_e49408_d_n6, assign44250_e49408_d_n7, assign44250_e49408_d_n8, assign44250_e49408_d_n9,) = {
    if (var_guard1355 != 0.0) {
        let assign44250_e49401: f64 = (-0.3333333333333);
        let assign44250_e49403: f64 = (var_e2_op).ln();
        let assign44250_e49404: f64 = (assign44250_e49401 * assign44250_e49403);
        let assign44250_e49405: f64 = (assign44250_e49404).exp();
        let assign44250_e49406: f64 = (var_qq_op * assign44250_e49405);
        (assign44250_e49406, ((var_qq_op_dn4 * assign44250_e49405) + (var_qq_op * (assign44250_e49405 * (assign44250_e49401 * (var_e2_op_dn4 / var_e2_op))))), ((var_qq_op_dn6 * assign44250_e49405) + (var_qq_op * (assign44250_e49405 * (assign44250_e49401 * (var_e2_op_dn6 / var_e2_op))))), ((var_qq_op_dn7 * assign44250_e49405) + (var_qq_op * (assign44250_e49405 * (assign44250_e49401 * (var_e2_op_dn7 / var_e2_op))))), ((var_qq_op_dn8 * assign44250_e49405) + (var_qq_op * (assign44250_e49405 * (assign44250_e49401 * (var_e2_op_dn8 / var_e2_op))))), ((var_qq_op_dn9 * assign44250_e49405) + (var_qq_op * (assign44250_e49405 * (assign44250_e49401 * (var_e2_op_dn9 / var_e2_op))))),)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign44250_e49408;
        var_temp2_dn4 = assign44250_e49408_d_n4;
        var_temp2_dn6 = assign44250_e49408_d_n6;
        var_temp2_dn7 = assign44250_e49408_d_n7;
        var_temp2_dn8 = assign44250_e49408_d_n8;
        var_temp2_dn9 = assign44250_e49408_d_n9;

        let (assign44260_e49416, assign44260_e49416_d_n4, assign44260_e49416_d_n6, assign44260_e49416_d_n7, assign44260_e49416_d_n8, assign44260_e49416_d_n9,) = {
    if (var_guard1355 != 0.0) {
        let assign44260_e49412: f64 = (1.0 - var_temp1);
        let assign44260_e49414: f64 = (assign44260_e49412 - var_temp2);
        (assign44260_e49414, ((-var_temp1_dn4) - var_temp2_dn4), ((-var_temp1_dn6) - var_temp2_dn6), ((-var_temp1_dn7) - var_temp2_dn7), ((-var_temp1_dn8) - var_temp2_dn8), ((-var_temp1_dn9) - var_temp2_dn9),)
    } else {
        (var_temp3, var_temp3_dn4, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9,)
    }
};
        var_temp3 = assign44260_e49416;
        var_temp3_dn4 = assign44260_e49416_d_n4;
        var_temp3_dn6 = assign44260_e49416_d_n6;
        var_temp3_dn7 = assign44260_e49416_d_n7;
        var_temp3_dn8 = assign44260_e49416_d_n8;
        var_temp3_dn9 = assign44260_e49416_d_n9;

        let (assign44280_e49434, assign44280_e49434_d_n4, assign44280_e49434_d_n6, assign44280_e49434_d_n7, assign44280_e49434_d_n8, assign44280_e49434_d_n9,) = {
    if (var_guard1355 != 0.0) {
        let assign44280_e49426: f64 = (var_k1_1d * var_temp3);
        let assign44280_e49430: f64 = (var_k1_1d * var_temp1);
        let assign44280_e49431: f64 = (1.0 + assign44280_e49430);
        let assign44280_e49432: f64 = (assign44280_e49426 / assign44280_e49431);
        (assign44280_e49432, ((((var_k1_1d * var_temp3_dn4) * assign44280_e49431) - (assign44280_e49426 * (var_k1_1d * var_temp1_dn4))) / (assign44280_e49431 * assign44280_e49431)), ((((var_k1_1d * var_temp3_dn6) * assign44280_e49431) - (assign44280_e49426 * (var_k1_1d * var_temp1_dn6))) / (assign44280_e49431 * assign44280_e49431)), ((((var_k1_1d * var_temp3_dn7) * assign44280_e49431) - (assign44280_e49426 * (var_k1_1d * var_temp1_dn7))) / (assign44280_e49431 * assign44280_e49431)), ((((var_k1_1d * var_temp3_dn8) * assign44280_e49431) - (assign44280_e49426 * (var_k1_1d * var_temp1_dn8))) / (assign44280_e49431 * assign44280_e49431)), ((((var_k1_1d * var_temp3_dn9) * assign44280_e49431) - (assign44280_e49426 * (var_k1_1d * var_temp1_dn9))) / (assign44280_e49431 * assign44280_e49431)),)
    } else {
        (var_k1_1d_qm_op, var_k1_1d_qm_op_dn4, var_k1_1d_qm_op_dn6, var_k1_1d_qm_op_dn7, var_k1_1d_qm_op_dn8, var_k1_1d_qm_op_dn9,)
    }
};
        var_k1_1d_qm_op = assign44280_e49434;
        var_k1_1d_qm_op_dn4 = assign44280_e49434_d_n4;
        var_k1_1d_qm_op_dn6 = assign44280_e49434_d_n6;
        var_k1_1d_qm_op_dn7 = assign44280_e49434_d_n7;
        var_k1_1d_qm_op_dn8 = assign44280_e49434_d_n8;
        var_k1_1d_qm_op_dn9 = assign44280_e49434_d_n9;

        let (assign44290_e49446, assign44290_e49446_d_n4, assign44290_e49446_d_n6, assign44290_e49446_d_n7, assign44290_e49446_d_n8, assign44290_e49446_d_n9,) = {
    if (var_guard1355 != 0.0) {
        let assign44290_e49438: f64 = (var_k2_1d * var_temp3);
        let assign44290_e49442: f64 = (var_k2_1d * var_temp2);
        let assign44290_e49443: f64 = (1.0 + assign44290_e49442);
        let assign44290_e49444: f64 = (assign44290_e49438 / assign44290_e49443);
        (assign44290_e49444, ((((var_k2_1d * var_temp3_dn4) * assign44290_e49443) - (assign44290_e49438 * (var_k2_1d * var_temp2_dn4))) / (assign44290_e49443 * assign44290_e49443)), ((((var_k2_1d * var_temp3_dn6) * assign44290_e49443) - (assign44290_e49438 * (var_k2_1d * var_temp2_dn6))) / (assign44290_e49443 * assign44290_e49443)), ((((var_k2_1d * var_temp3_dn7) * assign44290_e49443) - (assign44290_e49438 * (var_k2_1d * var_temp2_dn7))) / (assign44290_e49443 * assign44290_e49443)), ((((var_k2_1d * var_temp3_dn8) * assign44290_e49443) - (assign44290_e49438 * (var_k2_1d * var_temp2_dn8))) / (assign44290_e49443 * assign44290_e49443)), ((((var_k2_1d * var_temp3_dn9) * assign44290_e49443) - (assign44290_e49438 * (var_k2_1d * var_temp2_dn9))) / (assign44290_e49443 * assign44290_e49443)),)
    } else {
        (var_k2_1d_qm_op, var_k2_1d_qm_op_dn4, var_k2_1d_qm_op_dn6, var_k2_1d_qm_op_dn7, var_k2_1d_qm_op_dn8, var_k2_1d_qm_op_dn9,)
    }
};
        var_k2_1d_qm_op = assign44290_e49446;
        var_k2_1d_qm_op_dn4 = assign44290_e49446_d_n4;
        var_k2_1d_qm_op_dn6 = assign44290_e49446_d_n6;
        var_k2_1d_qm_op_dn7 = assign44290_e49446_d_n7;
        var_k2_1d_qm_op_dn8 = assign44290_e49446_d_n8;
        var_k2_1d_qm_op_dn9 = assign44290_e49446_d_n9;

        let (assign44300_e49460, assign44300_e49460_d_n4, assign44300_e49460_d_n6, assign44300_e49460_d_n7, assign44300_e49460_d_n8, assign44300_e49460_d_n9,) = {
    if (var_guard1355 != 0.0) {
        let assign44300_e49452: f64 = (1.0 / var_k1_1d_qm_op);
        let assign44300_e49453: f64 = (1.0 + assign44300_e49452);
        let assign44300_e49456: f64 = (1.0 / var_k2_1d_qm_op);
        let assign44300_e49457: f64 = (assign44300_e49453 + assign44300_e49456);
        let assign44300_e49458: f64 = (1.0 / assign44300_e49457);
        (assign44300_e49458, (-(((-(var_k1_1d_qm_op_dn4 / (var_k1_1d_qm_op * var_k1_1d_qm_op))) + (-(var_k2_1d_qm_op_dn4 / (var_k2_1d_qm_op * var_k2_1d_qm_op)))) / (assign44300_e49457 * assign44300_e49457))), (-(((-(var_k1_1d_qm_op_dn6 / (var_k1_1d_qm_op * var_k1_1d_qm_op))) + (-(var_k2_1d_qm_op_dn6 / (var_k2_1d_qm_op * var_k2_1d_qm_op)))) / (assign44300_e49457 * assign44300_e49457))), (-(((-(var_k1_1d_qm_op_dn7 / (var_k1_1d_qm_op * var_k1_1d_qm_op))) + (-(var_k2_1d_qm_op_dn7 / (var_k2_1d_qm_op * var_k2_1d_qm_op)))) / (assign44300_e49457 * assign44300_e49457))), (-(((-(var_k1_1d_qm_op_dn8 / (var_k1_1d_qm_op * var_k1_1d_qm_op))) + (-(var_k2_1d_qm_op_dn8 / (var_k2_1d_qm_op * var_k2_1d_qm_op)))) / (assign44300_e49457 * assign44300_e49457))), (-(((-(var_k1_1d_qm_op_dn9 / (var_k1_1d_qm_op * var_k1_1d_qm_op))) + (-(var_k2_1d_qm_op_dn9 / (var_k2_1d_qm_op * var_k2_1d_qm_op)))) / (assign44300_e49457 * assign44300_e49457))),)
    } else {
        (var_keq_1d_qm_op, var_keq_1d_qm_op_dn4, var_keq_1d_qm_op_dn6, var_keq_1d_qm_op_dn7, var_keq_1d_qm_op_dn8, var_keq_1d_qm_op_dn9,)
    }
};
        var_keq_1d_qm_op = assign44300_e49460;
        var_keq_1d_qm_op_dn4 = assign44300_e49460_d_n4;
        var_keq_1d_qm_op_dn6 = assign44300_e49460_d_n6;
        var_keq_1d_qm_op_dn7 = assign44300_e49460_d_n7;
        var_keq_1d_qm_op_dn8 = assign44300_e49460_d_n8;
        var_keq_1d_qm_op_dn9 = assign44300_e49460_d_n9;

        let (assign44320_e49470, assign44320_e49470_d_n4, assign44320_e49470_d_n6, assign44320_e49470_d_n7, assign44320_e49470_d_n8, assign44320_e49470_d_n9,) = {
    if (var_guard1355 == 0.0) {
        (var_k1_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_k1_1d_qm_op, var_k1_1d_qm_op_dn4, var_k1_1d_qm_op_dn6, var_k1_1d_qm_op_dn7, var_k1_1d_qm_op_dn8, var_k1_1d_qm_op_dn9,)
    }
};
        var_k1_1d_qm_op = assign44320_e49470;
        var_k1_1d_qm_op_dn4 = assign44320_e49470_d_n4;
        var_k1_1d_qm_op_dn6 = assign44320_e49470_d_n6;
        var_k1_1d_qm_op_dn7 = assign44320_e49470_d_n7;
        var_k1_1d_qm_op_dn8 = assign44320_e49470_d_n8;
        var_k1_1d_qm_op_dn9 = assign44320_e49470_d_n9;

        let (assign44330_e49475, assign44330_e49475_d_n4, assign44330_e49475_d_n6, assign44330_e49475_d_n7, assign44330_e49475_d_n8, assign44330_e49475_d_n9,) = {
    if (var_guard1355 == 0.0) {
        (var_k2_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_k2_1d_qm_op, var_k2_1d_qm_op_dn4, var_k2_1d_qm_op_dn6, var_k2_1d_qm_op_dn7, var_k2_1d_qm_op_dn8, var_k2_1d_qm_op_dn9,)
    }
};
        var_k2_1d_qm_op = assign44330_e49475;
        var_k2_1d_qm_op_dn4 = assign44330_e49475_d_n4;
        var_k2_1d_qm_op_dn6 = assign44330_e49475_d_n6;
        var_k2_1d_qm_op_dn7 = assign44330_e49475_d_n7;
        var_k2_1d_qm_op_dn8 = assign44330_e49475_d_n8;
        var_k2_1d_qm_op_dn9 = assign44330_e49475_d_n9;

        let (assign44340_e49480, assign44340_e49480_d_n4, assign44340_e49480_d_n6, assign44340_e49480_d_n7, assign44340_e49480_d_n8, assign44340_e49480_d_n9,) = {
    if (var_guard1355 == 0.0) {
        (var_keq_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_keq_1d_qm_op, var_keq_1d_qm_op_dn4, var_keq_1d_qm_op_dn6, var_keq_1d_qm_op_dn7, var_keq_1d_qm_op_dn8, var_keq_1d_qm_op_dn9,)
    }
};
        var_keq_1d_qm_op = assign44340_e49480;
        var_keq_1d_qm_op_dn4 = assign44340_e49480_d_n4;
        var_keq_1d_qm_op_dn6 = assign44340_e49480_d_n6;
        var_keq_1d_qm_op_dn7 = assign44340_e49480_d_n7;
        var_keq_1d_qm_op_dn8 = assign44340_e49480_d_n8;
        var_keq_1d_qm_op_dn9 = assign44340_e49480_d_n9;

        let assign44350_e49484: f64 = (var_xg10_op - var_xg2eff_op);
        let assign44350_e49485: f64 = (var_keq_1d_qm_op * assign44350_e49484);
        var_dx_wi_1d_op = assign44350_e49485;
        var_dx_wi_1d_op_dn4 = ((var_keq_1d_qm_op_dn4 * assign44350_e49484) + (var_keq_1d_qm_op * (var_xg10_op_dn4 - var_xg2eff_op_dn4)));
        var_dx_wi_1d_op_dn6 = ((var_keq_1d_qm_op_dn6 * assign44350_e49484) + (var_keq_1d_qm_op * (var_xg10_op_dn6 - var_xg2eff_op_dn6)));
        var_dx_wi_1d_op_dn7 = ((var_keq_1d_qm_op_dn7 * assign44350_e49484) + (var_keq_1d_qm_op * (var_xg10_op_dn7 - var_xg2eff_op_dn7)));
        var_dx_wi_1d_op_dn8 = ((var_keq_1d_qm_op_dn8 * assign44350_e49484) + (var_keq_1d_qm_op * (var_xg10_op_dn8 - var_xg2eff_op_dn8)));
        var_dx_wi_1d_op_dn9 = ((var_keq_1d_qm_op_dn9 * assign44350_e49484) + (var_keq_1d_qm_op * (var_xg10_op_dn9 - var_xg2eff_op_dn9)));

        let assign44360_e49488: f64 = if var_dx_wi_1d_op > 0.0 { 1.0 } else { 0.0 };
        var_guard1356 = assign44360_e49488;

        let assign44370_e49490: f64 = (-var_dx_wi_1d_op);
        let assign44370_e49492: f64 = if assign44370_e49490 < 80.0 { 1.0 } else { 0.0 };
        var_guard1357 = assign44370_e49492;

        let (assign44380_e49503, assign44380_e49503_d_n4, assign44380_e49503_d_n6, assign44380_e49503_d_n7, assign44380_e49503_d_n8, assign44380_e49503_d_n9,) = {
    if ((var_guard1356 != 0.0) && (var_guard1357 != 0.0)) {
        let assign44380_e49498: f64 = (-var_dx_wi_1d_op);
        let assign44380_e49499: f64 = (assign44380_e49498).exp();
        let assign44380_e49500: f64 = (1.0 + assign44380_e49499);
        let assign44380_e49501: f64 = (assign44380_e49500).ln();
        (assign44380_e49501, ((assign44380_e49499 * (-var_dx_wi_1d_op_dn4)) / assign44380_e49500), ((assign44380_e49499 * (-var_dx_wi_1d_op_dn6)) / assign44380_e49500), ((assign44380_e49499 * (-var_dx_wi_1d_op_dn7)) / assign44380_e49500), ((assign44380_e49499 * (-var_dx_wi_1d_op_dn8)) / assign44380_e49500), ((assign44380_e49499 * (-var_dx_wi_1d_op_dn9)) / assign44380_e49500),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign44380_e49503;
        var_temp_dn4 = assign44380_e49503_d_n4;
        var_temp_dn6 = assign44380_e49503_d_n6;
        var_temp_dn7 = assign44380_e49503_d_n7;
        var_temp_dn8 = assign44380_e49503_d_n8;
        var_temp_dn9 = assign44380_e49503_d_n9;

        let (assign44390_e49511, assign44390_e49511_d_n4, assign44390_e49511_d_n6, assign44390_e49511_d_n7, assign44390_e49511_d_n8, assign44390_e49511_d_n9,) = {
    if ((var_guard1356 != 0.0) && (var_guard1357 == 0.0)) {
        let assign44390_e49509: f64 = (-var_dx_wi_1d_op);
        (assign44390_e49509, (-var_dx_wi_1d_op_dn4), (-var_dx_wi_1d_op_dn6), (-var_dx_wi_1d_op_dn7), (-var_dx_wi_1d_op_dn8), (-var_dx_wi_1d_op_dn9),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign44390_e49511;
        var_temp_dn4 = assign44390_e49511_d_n4;
        var_temp_dn6 = assign44390_e49511_d_n6;
        var_temp_dn7 = assign44390_e49511_d_n7;
        var_temp_dn8 = assign44390_e49511_d_n8;
        var_temp_dn9 = assign44390_e49511_d_n9;

        let (assign44400_e49523, assign44400_e49523_d_n4, assign44400_e49523_d_n6, assign44400_e49523_d_n7, assign44400_e49523_d_n8, assign44400_e49523_d_n9,) = {
    if (var_guard1356 != 0.0) {
        let assign44400_e49516: f64 = (var_dx_wi_1d_op / var_k1_1d_qm_op);
        let assign44400_e49517: f64 = (var_xg10_op - assign44400_e49516);
        let assign44400_e49519: f64 = (assign44400_e49517 + var_temp);
        let assign44400_e49521: f64 = (assign44400_e49519 - 0.6931471805599);
        (assign44400_e49521, ((var_xg10_op_dn4 - (((var_dx_wi_1d_op_dn4 * var_k1_1d_qm_op) - (var_dx_wi_1d_op * var_k1_1d_qm_op_dn4)) / (var_k1_1d_qm_op * var_k1_1d_qm_op))) + var_temp_dn4), ((var_xg10_op_dn6 - (((var_dx_wi_1d_op_dn6 * var_k1_1d_qm_op) - (var_dx_wi_1d_op * var_k1_1d_qm_op_dn6)) / (var_k1_1d_qm_op * var_k1_1d_qm_op))) + var_temp_dn6), ((var_xg10_op_dn7 - (((var_dx_wi_1d_op_dn7 * var_k1_1d_qm_op) - (var_dx_wi_1d_op * var_k1_1d_qm_op_dn7)) / (var_k1_1d_qm_op * var_k1_1d_qm_op))) + var_temp_dn7), ((var_xg10_op_dn8 - (((var_dx_wi_1d_op_dn8 * var_k1_1d_qm_op) - (var_dx_wi_1d_op * var_k1_1d_qm_op_dn8)) / (var_k1_1d_qm_op * var_k1_1d_qm_op))) + var_temp_dn8), ((var_xg10_op_dn9 - (((var_dx_wi_1d_op_dn9 * var_k1_1d_qm_op) - (var_dx_wi_1d_op * var_k1_1d_qm_op_dn9)) / (var_k1_1d_qm_op * var_k1_1d_qm_op))) + var_temp_dn9),)
    } else {
        (var_x_wi_1d_op, var_x_wi_1d_op_dn4, var_x_wi_1d_op_dn6, var_x_wi_1d_op_dn7, var_x_wi_1d_op_dn8, var_x_wi_1d_op_dn9,)
    }
};
        var_x_wi_1d_op = assign44400_e49523;
        var_x_wi_1d_op_dn4 = assign44400_e49523_d_n4;
        var_x_wi_1d_op_dn6 = assign44400_e49523_d_n6;
        var_x_wi_1d_op_dn7 = assign44400_e49523_d_n7;
        var_x_wi_1d_op_dn8 = assign44400_e49523_d_n8;
        var_x_wi_1d_op_dn9 = assign44400_e49523_d_n9;

        let assign44410_e49526: f64 = if var_dx_wi_1d_op < 80.0 { 1.0 } else { 0.0 };
        var_guard1358 = assign44410_e49526;

        let (assign44420_e49537, assign44420_e49537_d_n4, assign44420_e49537_d_n6, assign44420_e49537_d_n7, assign44420_e49537_d_n8, assign44420_e49537_d_n9,) = {
    if ((var_guard1356 == 0.0) && (var_guard1358 != 0.0)) {
        let assign44420_e49533: f64 = (var_dx_wi_1d_op).exp();
        let assign44420_e49534: f64 = (1.0 + assign44420_e49533);
        let assign44420_e49535: f64 = (assign44420_e49534).ln();
        (assign44420_e49535, ((assign44420_e49533 * var_dx_wi_1d_op_dn4) / assign44420_e49534), ((assign44420_e49533 * var_dx_wi_1d_op_dn6) / assign44420_e49534), ((assign44420_e49533 * var_dx_wi_1d_op_dn7) / assign44420_e49534), ((assign44420_e49533 * var_dx_wi_1d_op_dn8) / assign44420_e49534), ((assign44420_e49533 * var_dx_wi_1d_op_dn9) / assign44420_e49534),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign44420_e49537;
        var_temp_dn4 = assign44420_e49537_d_n4;
        var_temp_dn6 = assign44420_e49537_d_n6;
        var_temp_dn7 = assign44420_e49537_d_n7;
        var_temp_dn8 = assign44420_e49537_d_n8;
        var_temp_dn9 = assign44420_e49537_d_n9;

        let (assign44430_e49545, assign44430_e49545_d_n4, assign44430_e49545_d_n6, assign44430_e49545_d_n7, assign44430_e49545_d_n8, assign44430_e49545_d_n9,) = {
    if ((var_guard1356 == 0.0) && (var_guard1358 == 0.0)) {
        (var_dx_wi_1d_op, var_dx_wi_1d_op_dn4, var_dx_wi_1d_op_dn6, var_dx_wi_1d_op_dn7, var_dx_wi_1d_op_dn8, var_dx_wi_1d_op_dn9,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign44430_e49545;
        var_temp_dn4 = assign44430_e49545_d_n4;
        var_temp_dn6 = assign44430_e49545_d_n6;
        var_temp_dn7 = assign44430_e49545_d_n7;
        var_temp_dn8 = assign44430_e49545_d_n8;
        var_temp_dn9 = assign44430_e49545_d_n9;

        let (assign44440_e49558, assign44440_e49558_d_n4, assign44440_e49558_d_n6, assign44440_e49558_d_n7, assign44440_e49558_d_n8, assign44440_e49558_d_n9,) = {
    if (var_guard1356 == 0.0) {
        let assign44440_e49551: f64 = (var_dx_wi_1d_op / var_k2_1d_qm_op);
        let assign44440_e49552: f64 = (var_xg2eff_op + assign44440_e49551);
        let assign44440_e49554: f64 = (assign44440_e49552 + var_temp);
        let assign44440_e49556: f64 = (assign44440_e49554 - 0.6931471805599);
        (assign44440_e49556, ((var_xg2eff_op_dn4 + (((var_dx_wi_1d_op_dn4 * var_k2_1d_qm_op) - (var_dx_wi_1d_op * var_k2_1d_qm_op_dn4)) / (var_k2_1d_qm_op * var_k2_1d_qm_op))) + var_temp_dn4), ((var_xg2eff_op_dn6 + (((var_dx_wi_1d_op_dn6 * var_k2_1d_qm_op) - (var_dx_wi_1d_op * var_k2_1d_qm_op_dn6)) / (var_k2_1d_qm_op * var_k2_1d_qm_op))) + var_temp_dn6), ((var_xg2eff_op_dn7 + (((var_dx_wi_1d_op_dn7 * var_k2_1d_qm_op) - (var_dx_wi_1d_op * var_k2_1d_qm_op_dn7)) / (var_k2_1d_qm_op * var_k2_1d_qm_op))) + var_temp_dn7), ((var_xg2eff_op_dn8 + (((var_dx_wi_1d_op_dn8 * var_k2_1d_qm_op) - (var_dx_wi_1d_op * var_k2_1d_qm_op_dn8)) / (var_k2_1d_qm_op * var_k2_1d_qm_op))) + var_temp_dn8), ((var_xg2eff_op_dn9 + (((var_dx_wi_1d_op_dn9 * var_k2_1d_qm_op) - (var_dx_wi_1d_op * var_k2_1d_qm_op_dn9)) / (var_k2_1d_qm_op * var_k2_1d_qm_op))) + var_temp_dn9),)
    } else {
        (var_x_wi_1d_op, var_x_wi_1d_op_dn4, var_x_wi_1d_op_dn6, var_x_wi_1d_op_dn7, var_x_wi_1d_op_dn8, var_x_wi_1d_op_dn9,)
    }
};
        var_x_wi_1d_op = assign44440_e49558;
        var_x_wi_1d_op_dn4 = assign44440_e49558_d_n4;
        var_x_wi_1d_op_dn6 = assign44440_e49558_d_n6;
        var_x_wi_1d_op_dn7 = assign44440_e49558_d_n7;
        var_x_wi_1d_op_dn8 = assign44440_e49558_d_n8;
        var_x_wi_1d_op_dn9 = assign44440_e49558_d_n9;

        let assign44450_e49562: f64 = (var_x_wi_1d_op + var_xth_1d_op);
        let assign44450_e49565: f64 = (var_x_wi_1d_op - var_xth_1d_op);
        let assign44450_e49568: f64 = (var_x_wi_1d_op - var_xth_1d_op);
        let assign44450_e49569: f64 = (assign44450_e49565 * assign44450_e49568);
        let assign44450_e49571: f64 = (assign44450_e49569 + 4.0);
        let assign44450_e49572: f64 = (assign44450_e49571).sqrt();
        let assign44450_e49573: f64 = (assign44450_e49562 - assign44450_e49572);
        let assign44450_e49574: f64 = (0.5 * assign44450_e49573);
        var_x_1d_op = assign44450_e49574;
        var_x_1d_op_dn4 = (0.5 * ((var_x_wi_1d_op_dn4 + var_xth_1d_op_dn4) - ((((var_x_wi_1d_op_dn4 - var_xth_1d_op_dn4) * assign44450_e49568) + (assign44450_e49565 * (var_x_wi_1d_op_dn4 - var_xth_1d_op_dn4))) / (2.0 * assign44450_e49572))));
        var_x_1d_op_dn6 = (0.5 * ((var_x_wi_1d_op_dn6 + var_xth_1d_op_dn6) - ((((var_x_wi_1d_op_dn6 - var_xth_1d_op_dn6) * assign44450_e49568) + (assign44450_e49565 * (var_x_wi_1d_op_dn6 - var_xth_1d_op_dn6))) / (2.0 * assign44450_e49572))));
        var_x_1d_op_dn7 = (0.5 * ((var_x_wi_1d_op_dn7 + var_xth_1d_op_dn7) - ((((var_x_wi_1d_op_dn7 - var_xth_1d_op_dn7) * assign44450_e49568) + (assign44450_e49565 * (var_x_wi_1d_op_dn7 - var_xth_1d_op_dn7))) / (2.0 * assign44450_e49572))));
        var_x_1d_op_dn8 = (0.5 * ((var_x_wi_1d_op_dn8 + var_xth_1d_op_dn8) - ((((var_x_wi_1d_op_dn8 - var_xth_1d_op_dn8) * assign44450_e49568) + (assign44450_e49565 * (var_x_wi_1d_op_dn8 - var_xth_1d_op_dn8))) / (2.0 * assign44450_e49572))));
        var_x_1d_op_dn9 = (0.5 * ((var_x_wi_1d_op_dn9 + var_xth_1d_op_dn9) - ((((var_x_wi_1d_op_dn9 - var_xth_1d_op_dn9) * assign44450_e49568) + (assign44450_e49565 * (var_x_wi_1d_op_dn9 - var_xth_1d_op_dn9))) / (2.0 * assign44450_e49572))));

        let assign44460_e49579: f64 = (var_xth_1d_op - var_x_1d_op);
        let assign44460_e49580: f64 = (2.0 * assign44460_e49579);
        let assign44460_e49582: f64 = (assign44460_e49580 / var_xsddep_op);
        let assign44460_e49583: f64 = (1.0 + assign44460_e49582);
        let assign44460_e49584: f64 = (assign44460_e49583).sqrt();
        let assign44460_e49586: f64 = (assign44460_e49584 - 1.0);
        var_dleff_op = assign44460_e49586;
        var_dleff_op_dn4 = (((((2.0 * (var_xth_1d_op_dn4 - var_x_1d_op_dn4)) * var_xsddep_op) - (assign44460_e49580 * var_xsddep_op_dn4)) / (var_xsddep_op * var_xsddep_op)) / (2.0 * assign44460_e49584));
        var_dleff_op_dn6 = (((((2.0 * (var_xth_1d_op_dn6 - var_x_1d_op_dn6)) * var_xsddep_op) - (assign44460_e49580 * var_xsddep_op_dn6)) / (var_xsddep_op * var_xsddep_op)) / (2.0 * assign44460_e49584));
        var_dleff_op_dn7 = (((((2.0 * (var_xth_1d_op_dn7 - var_x_1d_op_dn7)) * var_xsddep_op) - (assign44460_e49580 * var_xsddep_op_dn7)) / (var_xsddep_op * var_xsddep_op)) / (2.0 * assign44460_e49584));
        var_dleff_op_dn8 = (((((2.0 * (var_xth_1d_op_dn8 - var_x_1d_op_dn8)) * var_xsddep_op) - (assign44460_e49580 * var_xsddep_op_dn8)) / (var_xsddep_op * var_xsddep_op)) / (2.0 * assign44460_e49584));
        var_dleff_op_dn9 = (((((2.0 * (var_xth_1d_op_dn9 - var_x_1d_op_dn9)) * var_xsddep_op) - (assign44460_e49580 * var_xsddep_op_dn9)) / (var_xsddep_op * var_xsddep_op)) / (2.0 * assign44460_e49584));

        let assign44480_e49596: f64 = (var_pscedlb_i * var_xg20_op);
        let assign44480_e49597: f64 = (1.0 + assign44480_e49596);
        let assign44480_e49599: f64 = (assign44480_e49597 + 0.5);
        let assign44480_e49603: f64 = (var_pscedlb_i * var_xg20_op);
        let assign44480_e49604: f64 = (1.0 + assign44480_e49603);
        let assign44480_e49606: f64 = (assign44480_e49604 - 0.5);
        let assign44480_e49610: f64 = (var_pscedlb_i * var_xg20_op);
        let assign44480_e49611: f64 = (1.0 + assign44480_e49610);
        let assign44480_e49613: f64 = (assign44480_e49611 - 0.5);
        let assign44480_e49614: f64 = (assign44480_e49606 * assign44480_e49613);
        let assign44480_e49616: f64 = (assign44480_e49614 + 0.01);
        let assign44480_e49617: f64 = (assign44480_e49616).sqrt();
        let assign44480_e49618: f64 = (assign44480_e49599 + assign44480_e49617);
        let assign44480_e49619: f64 = (0.5 * assign44480_e49618);
        var_temp = assign44480_e49619;
        var_temp_dn4 = (0.5 * ((var_pscedlb_i * var_xg20_op_dn4) + ((((var_pscedlb_i * var_xg20_op_dn4) * assign44480_e49613) + (assign44480_e49606 * (var_pscedlb_i * var_xg20_op_dn4))) / (2.0 * assign44480_e49617))));
        var_temp_dn6 = (0.5 * ((var_pscedlb_i * var_xg20_op_dn6) + ((((var_pscedlb_i * var_xg20_op_dn6) * assign44480_e49613) + (assign44480_e49606 * (var_pscedlb_i * var_xg20_op_dn6))) / (2.0 * assign44480_e49617))));
        var_temp_dn7 = (0.5 * ((var_pscedlb_i * var_xg20_op_dn7) + ((((var_pscedlb_i * var_xg20_op_dn7) * assign44480_e49613) + (assign44480_e49606 * (var_pscedlb_i * var_xg20_op_dn7))) / (2.0 * assign44480_e49617))));
        var_temp_dn8 = (0.5 * ((var_pscedlb_i * var_xg20_op_dn8) + ((((var_pscedlb_i * var_xg20_op_dn8) * assign44480_e49613) + (assign44480_e49606 * (var_pscedlb_i * var_xg20_op_dn8))) / (2.0 * assign44480_e49617))));
        var_temp_dn9 = (0.5 * ((var_pscedlb_i * var_xg20_op_dn9) + ((((var_pscedlb_i * var_xg20_op_dn9) * assign44480_e49613) + (assign44480_e49606 * (var_pscedlb_i * var_xg20_op_dn9))) / (2.0 * assign44480_e49617))));

        let assign44510_e49636: f64 = (2.0 * var_xd0_op);
        let assign44510_e49640: f64 = (var_xdsx_op / var_xd0_op);
        let assign44510_e49641: f64 = (1.0 + assign44510_e49640);
        let assign44510_e49642: f64 = (assign44510_e49641).sqrt();
        let assign44510_e49644: f64 = (assign44510_e49642 - 1.0);
        let assign44510_e49645: f64 = (assign44510_e49636 * assign44510_e49644);
        let assign44510_e49649: f64 = (var_cfdl_i * var_dleff_op);
        let assign44510_e49650: f64 = (1.0 + assign44510_e49649);
        let assign44510_e49651: f64 = (assign44510_e49645 * assign44510_e49650);
        let assign44510_e49655: f64 = (var_cfdlb_i * var_xg20_op);
        let assign44510_e49656: f64 = (1.0 + assign44510_e49655);
        let assign44510_e49657: f64 = (assign44510_e49651 * assign44510_e49656);
        var_temp = assign44510_e49657;
        var_temp_dn4 = (((((((2.0 * var_xd0_op_dn4) * assign44510_e49644) + (assign44510_e49636 * ((((var_xdsx_op_dn4 * var_xd0_op) - (var_xdsx_op * var_xd0_op_dn4)) / (var_xd0_op * var_xd0_op)) / (2.0 * assign44510_e49642)))) * assign44510_e49650) + (assign44510_e49645 * (var_cfdl_i * var_dleff_op_dn4))) * assign44510_e49656) + (assign44510_e49651 * (var_cfdlb_i * var_xg20_op_dn4)));
        var_temp_dn6 = (((((((2.0 * var_xd0_op_dn6) * assign44510_e49644) + (assign44510_e49636 * ((((var_xdsx_op_dn6 * var_xd0_op) - (var_xdsx_op * var_xd0_op_dn6)) / (var_xd0_op * var_xd0_op)) / (2.0 * assign44510_e49642)))) * assign44510_e49650) + (assign44510_e49645 * (var_cfdl_i * var_dleff_op_dn6))) * assign44510_e49656) + (assign44510_e49651 * (var_cfdlb_i * var_xg20_op_dn6)));
        var_temp_dn7 = (((((((2.0 * var_xd0_op_dn7) * assign44510_e49644) + (assign44510_e49636 * ((((var_xdsx_op_dn7 * var_xd0_op) - (var_xdsx_op * var_xd0_op_dn7)) / (var_xd0_op * var_xd0_op)) / (2.0 * assign44510_e49642)))) * assign44510_e49650) + (assign44510_e49645 * (var_cfdl_i * var_dleff_op_dn7))) * assign44510_e49656) + (assign44510_e49651 * (var_cfdlb_i * var_xg20_op_dn7)));
        var_temp_dn8 = (((((((2.0 * var_xd0_op_dn8) * assign44510_e49644) + (assign44510_e49636 * ((((var_xdsx_op_dn8 * var_xd0_op) - (var_xdsx_op * var_xd0_op_dn8)) / (var_xd0_op * var_xd0_op)) / (2.0 * assign44510_e49642)))) * assign44510_e49650) + (assign44510_e49645 * (var_cfdl_i * var_dleff_op_dn8))) * assign44510_e49656) + (assign44510_e49651 * (var_cfdlb_i * var_xg20_op_dn8)));
        var_temp_dn9 = (((((((2.0 * var_xd0_op_dn9) * assign44510_e49644) + (assign44510_e49636 * ((((var_xdsx_op_dn9 * var_xd0_op) - (var_xdsx_op * var_xd0_op_dn9)) / (var_xd0_op * var_xd0_op)) / (2.0 * assign44510_e49642)))) * assign44510_e49650) + (assign44510_e49645 * (var_cfdl_i * var_dleff_op_dn9))) * assign44510_e49656) + (assign44510_e49651 * (var_cfdlb_i * var_xg20_op_dn9)));

        let assign44750_e49840: f64 = if p.p11 > 0.0 { 1.0 } else { 0.0 };
        var_guard1360 = assign44750_e49840;

        let (assign44760_e49850, assign44760_e49850_d_n4, assign44760_e49850_d_n6, assign44760_e49850_d_n7, assign44760_e49850_d_n8, assign44760_e49850_d_n9,) = {
    if (var_guard1360 != 0.0) {
        let assign44760_e49844: f64 = (var_k2_ac / var_k1_ac);
        let assign44760_e49847: f64 = (1.0 + var_k2_ac);
        let assign44760_e49848: f64 = (assign44760_e49844 / assign44760_e49847);
        (assign44760_e49848, ((((((var_k2_ac_dn4 * var_k1_ac) - (var_k2_ac * var_k1_ac_dn4)) / (var_k1_ac * var_k1_ac)) * assign44760_e49847) - (assign44760_e49844 * var_k2_ac_dn4)) / (assign44760_e49847 * assign44760_e49847)), ((((((var_k2_ac_dn6 * var_k1_ac) - (var_k2_ac * var_k1_ac_dn6)) / (var_k1_ac * var_k1_ac)) * assign44760_e49847) - (assign44760_e49844 * var_k2_ac_dn6)) / (assign44760_e49847 * assign44760_e49847)), ((((((var_k2_ac_dn7 * var_k1_ac) - (var_k2_ac * var_k1_ac_dn7)) / (var_k1_ac * var_k1_ac)) * assign44760_e49847) - (assign44760_e49844 * var_k2_ac_dn7)) / (assign44760_e49847 * assign44760_e49847)), ((((((var_k2_ac_dn8 * var_k1_ac) - (var_k2_ac * var_k1_ac_dn8)) / (var_k1_ac * var_k1_ac)) * assign44760_e49847) - (assign44760_e49844 * var_k2_ac_dn8)) / (assign44760_e49847 * assign44760_e49847)), ((((((var_k2_ac_dn9 * var_k1_ac) - (var_k2_ac * var_k1_ac_dn9)) / (var_k1_ac * var_k1_ac)) * assign44760_e49847) - (assign44760_e49844 * var_k2_ac_dn9)) / (assign44760_e49847 * assign44760_e49847)),)
    } else {
        (var_r1init_op, var_r1init_op_dn4, var_r1init_op_dn6, var_r1init_op_dn7, var_r1init_op_dn8, var_r1init_op_dn9,)
    }
};
        var_r1init_op = assign44760_e49850;
        var_r1init_op_dn4 = assign44760_e49850_d_n4;
        var_r1init_op_dn6 = assign44760_e49850_d_n6;
        var_r1init_op_dn7 = assign44760_e49850_d_n7;
        var_r1init_op_dn8 = assign44760_e49850_d_n8;
        var_r1init_op_dn9 = assign44760_e49850_d_n9;

        let (assign44770_e49860, assign44770_e49860_d_n4, assign44770_e49860_d_n6, assign44770_e49860_d_n7, assign44770_e49860_d_n8, assign44770_e49860_d_n9,) = {
    if (var_guard1360 != 0.0) {
        let assign44770_e49854: f64 = (var_k1_ac / var_k2_ac);
        let assign44770_e49857: f64 = (1.0 + var_k1_ac);
        let assign44770_e49858: f64 = (assign44770_e49854 / assign44770_e49857);
        (assign44770_e49858, ((((((var_k1_ac_dn4 * var_k2_ac) - (var_k1_ac * var_k2_ac_dn4)) / (var_k2_ac * var_k2_ac)) * assign44770_e49857) - (assign44770_e49854 * var_k1_ac_dn4)) / (assign44770_e49857 * assign44770_e49857)), ((((((var_k1_ac_dn6 * var_k2_ac) - (var_k1_ac * var_k2_ac_dn6)) / (var_k2_ac * var_k2_ac)) * assign44770_e49857) - (assign44770_e49854 * var_k1_ac_dn6)) / (assign44770_e49857 * assign44770_e49857)), ((((((var_k1_ac_dn7 * var_k2_ac) - (var_k1_ac * var_k2_ac_dn7)) / (var_k2_ac * var_k2_ac)) * assign44770_e49857) - (assign44770_e49854 * var_k1_ac_dn7)) / (assign44770_e49857 * assign44770_e49857)), ((((((var_k1_ac_dn8 * var_k2_ac) - (var_k1_ac * var_k2_ac_dn8)) / (var_k2_ac * var_k2_ac)) * assign44770_e49857) - (assign44770_e49854 * var_k1_ac_dn8)) / (assign44770_e49857 * assign44770_e49857)), ((((((var_k1_ac_dn9 * var_k2_ac) - (var_k1_ac * var_k2_ac_dn9)) / (var_k2_ac * var_k2_ac)) * assign44770_e49857) - (assign44770_e49854 * var_k1_ac_dn9)) / (assign44770_e49857 * assign44770_e49857)),)
    } else {
        (var_r2init_op, var_r2init_op_dn4, var_r2init_op_dn6, var_r2init_op_dn7, var_r2init_op_dn8, var_r2init_op_dn9,)
    }
};
        var_r2init_op = assign44770_e49860;
        var_r2init_op_dn4 = assign44770_e49860_d_n4;
        var_r2init_op_dn6 = assign44770_e49860_d_n6;
        var_r2init_op_dn7 = assign44770_e49860_d_n7;
        var_r2init_op_dn8 = assign44770_e49860_d_n8;
        var_r2init_op_dn9 = assign44770_e49860_d_n9;

        let (assign44780_e49875, assign44780_e49875_d_n4, assign44780_e49875_d_n6, assign44780_e49875_d_n7, assign44780_e49875_d_n8, assign44780_e49875_d_n9,) = {
    if (var_guard1360 != 0.0) {
        let assign44780_e49865: f64 = (1.0 + var_r1init_op);
        let assign44780_e49866: f64 = (var_k1_ac * assign44780_e49865);
        let assign44780_e49868: f64 = (assign44780_e49866 * var_diff_min_ac);
        let assign44780_e49870: f64 = (assign44780_e49868 / var_a0_ac);
        let assign44780_e49871: f64 = (assign44780_e49870).ln();
        let assign44780_e49873: f64 = (assign44780_e49871 + 2.0);
        (assign44780_e49873, ((((((((var_k1_ac_dn4 * assign44780_e49865) + (var_k1_ac * var_r1init_op_dn4)) * var_diff_min_ac) + (assign44780_e49866 * var_diff_min_ac_dn4)) * var_a0_ac) - (assign44780_e49868 * var_a0_ac_dn4)) / (var_a0_ac * var_a0_ac)) / assign44780_e49870), ((((((((var_k1_ac_dn6 * assign44780_e49865) + (var_k1_ac * var_r1init_op_dn6)) * var_diff_min_ac) + (assign44780_e49866 * var_diff_min_ac_dn6)) * var_a0_ac) - (assign44780_e49868 * var_a0_ac_dn6)) / (var_a0_ac * var_a0_ac)) / assign44780_e49870), ((((((((var_k1_ac_dn7 * assign44780_e49865) + (var_k1_ac * var_r1init_op_dn7)) * var_diff_min_ac) + (assign44780_e49866 * var_diff_min_ac_dn7)) * var_a0_ac) - (assign44780_e49868 * var_a0_ac_dn7)) / (var_a0_ac * var_a0_ac)) / assign44780_e49870), ((((((((var_k1_ac_dn8 * assign44780_e49865) + (var_k1_ac * var_r1init_op_dn8)) * var_diff_min_ac) + (assign44780_e49866 * var_diff_min_ac_dn8)) * var_a0_ac) - (assign44780_e49868 * var_a0_ac_dn8)) / (var_a0_ac * var_a0_ac)) / assign44780_e49870), ((((((((var_k1_ac_dn9 * assign44780_e49865) + (var_k1_ac * var_r1init_op_dn9)) * var_diff_min_ac) + (assign44780_e49866 * var_diff_min_ac_dn9)) * var_a0_ac) - (assign44780_e49868 * var_a0_ac_dn9)) / (var_a0_ac * var_a0_ac)) / assign44780_e49870),)
    } else {
        (var_x1init_op, var_x1init_op_dn4, var_x1init_op_dn6, var_x1init_op_dn7, var_x1init_op_dn8, var_x1init_op_dn9,)
    }
};
        var_x1init_op = assign44780_e49875;
        var_x1init_op_dn4 = assign44780_e49875_d_n4;
        var_x1init_op_dn6 = assign44780_e49875_d_n6;
        var_x1init_op_dn7 = assign44780_e49875_d_n7;
        var_x1init_op_dn8 = assign44780_e49875_d_n8;
        var_x1init_op_dn9 = assign44780_e49875_d_n9;

        let (assign44790_e49890, assign44790_e49890_d_n4, assign44790_e49890_d_n6, assign44790_e49890_d_n7, assign44790_e49890_d_n8, assign44790_e49890_d_n9,) = {
    if (var_guard1360 != 0.0) {
        let assign44790_e49880: f64 = (1.0 + var_r2init_op);
        let assign44790_e49881: f64 = (var_k2_ac * assign44790_e49880);
        let assign44790_e49883: f64 = (assign44790_e49881 * var_diff_min_ac);
        let assign44790_e49885: f64 = (assign44790_e49883 / var_a0_ac);
        let assign44790_e49886: f64 = (assign44790_e49885).ln();
        let assign44790_e49888: f64 = (assign44790_e49886 + 2.0);
        (assign44790_e49888, ((((((((var_k2_ac_dn4 * assign44790_e49880) + (var_k2_ac * var_r2init_op_dn4)) * var_diff_min_ac) + (assign44790_e49881 * var_diff_min_ac_dn4)) * var_a0_ac) - (assign44790_e49883 * var_a0_ac_dn4)) / (var_a0_ac * var_a0_ac)) / assign44790_e49885), ((((((((var_k2_ac_dn6 * assign44790_e49880) + (var_k2_ac * var_r2init_op_dn6)) * var_diff_min_ac) + (assign44790_e49881 * var_diff_min_ac_dn6)) * var_a0_ac) - (assign44790_e49883 * var_a0_ac_dn6)) / (var_a0_ac * var_a0_ac)) / assign44790_e49885), ((((((((var_k2_ac_dn7 * assign44790_e49880) + (var_k2_ac * var_r2init_op_dn7)) * var_diff_min_ac) + (assign44790_e49881 * var_diff_min_ac_dn7)) * var_a0_ac) - (assign44790_e49883 * var_a0_ac_dn7)) / (var_a0_ac * var_a0_ac)) / assign44790_e49885), ((((((((var_k2_ac_dn8 * assign44790_e49880) + (var_k2_ac * var_r2init_op_dn8)) * var_diff_min_ac) + (assign44790_e49881 * var_diff_min_ac_dn8)) * var_a0_ac) - (assign44790_e49883 * var_a0_ac_dn8)) / (var_a0_ac * var_a0_ac)) / assign44790_e49885), ((((((((var_k2_ac_dn9 * assign44790_e49880) + (var_k2_ac * var_r2init_op_dn9)) * var_diff_min_ac) + (assign44790_e49881 * var_diff_min_ac_dn9)) * var_a0_ac) - (assign44790_e49883 * var_a0_ac_dn9)) / (var_a0_ac * var_a0_ac)) / assign44790_e49885),)
    } else {
        (var_x2init_op, var_x2init_op_dn4, var_x2init_op_dn6, var_x2init_op_dn7, var_x2init_op_dn8, var_x2init_op_dn9,)
    }
};
        var_x2init_op = assign44790_e49890;
        var_x2init_op_dn4 = assign44790_e49890_d_n4;
        var_x2init_op_dn6 = assign44790_e49890_d_n6;
        var_x2init_op_dn7 = assign44790_e49890_d_n7;
        var_x2init_op_dn8 = assign44790_e49890_d_n8;
        var_x2init_op_dn9 = assign44790_e49890_d_n9;

        let (assign44800_e49902, assign44800_e49902_d_n4, assign44800_e49902_d_n6, assign44800_e49902_d_n7, assign44800_e49902_d_n8, assign44800_e49902_d_n9,) = {
    if (var_guard1360 != 0.0) {
        let assign44800_e49894: f64 = (1.0 + var_r1init_op);
        let assign44800_e49896: f64 = (assign44800_e49894 * var_x1init_op);
        let assign44800_e49899: f64 = (var_xg2x_ac * var_r1init_op);
        let assign44800_e49900: f64 = (assign44800_e49896 - assign44800_e49899);
        (assign44800_e49900, (((var_r1init_op_dn4 * var_x1init_op) + (assign44800_e49894 * var_x1init_op_dn4)) - ((var_xg2x_ac_dn4 * var_r1init_op) + (var_xg2x_ac * var_r1init_op_dn4))), (((var_r1init_op_dn6 * var_x1init_op) + (assign44800_e49894 * var_x1init_op_dn6)) - ((var_xg2x_ac_dn6 * var_r1init_op) + (var_xg2x_ac * var_r1init_op_dn6))), (((var_r1init_op_dn7 * var_x1init_op) + (assign44800_e49894 * var_x1init_op_dn7)) - ((var_xg2x_ac_dn7 * var_r1init_op) + (var_xg2x_ac * var_r1init_op_dn7))), (((var_r1init_op_dn8 * var_x1init_op) + (assign44800_e49894 * var_x1init_op_dn8)) - ((var_xg2x_ac_dn8 * var_r1init_op) + (var_xg2x_ac * var_r1init_op_dn8))), (((var_r1init_op_dn9 * var_x1init_op) + (assign44800_e49894 * var_x1init_op_dn9)) - ((var_xg2x_ac_dn9 * var_r1init_op) + (var_xg2x_ac * var_r1init_op_dn9))),)
    } else {
        (var_xth1init_op, var_xth1init_op_dn4, var_xth1init_op_dn6, var_xth1init_op_dn7, var_xth1init_op_dn8, var_xth1init_op_dn9,)
    }
};
        var_xth1init_op = assign44800_e49902;
        var_xth1init_op_dn4 = assign44800_e49902_d_n4;
        var_xth1init_op_dn6 = assign44800_e49902_d_n6;
        var_xth1init_op_dn7 = assign44800_e49902_d_n7;
        var_xth1init_op_dn8 = assign44800_e49902_d_n8;
        var_xth1init_op_dn9 = assign44800_e49902_d_n9;

        let (assign44810_e49916, assign44810_e49916_d_n4, assign44810_e49916_d_n6, assign44810_e49916_d_n7, assign44810_e49916_d_n8, assign44810_e49916_d_n9,) = {
    if (var_guard1360 != 0.0) {
        let assign44810_e49907: f64 = (1.0 / var_r2init_op);
        let assign44810_e49908: f64 = (1.0 + assign44810_e49907);
        let assign44810_e49910: f64 = (assign44810_e49908 * var_x2init_op);
        let assign44810_e49913: f64 = (var_xg2x_ac / var_r2init_op);
        let assign44810_e49914: f64 = (assign44810_e49910 - assign44810_e49913);
        (assign44810_e49914, ((((-(var_r2init_op_dn4 / (var_r2init_op * var_r2init_op))) * var_x2init_op) + (assign44810_e49908 * var_x2init_op_dn4)) - (((var_xg2x_ac_dn4 * var_r2init_op) - (var_xg2x_ac * var_r2init_op_dn4)) / (var_r2init_op * var_r2init_op))), ((((-(var_r2init_op_dn6 / (var_r2init_op * var_r2init_op))) * var_x2init_op) + (assign44810_e49908 * var_x2init_op_dn6)) - (((var_xg2x_ac_dn6 * var_r2init_op) - (var_xg2x_ac * var_r2init_op_dn6)) / (var_r2init_op * var_r2init_op))), ((((-(var_r2init_op_dn7 / (var_r2init_op * var_r2init_op))) * var_x2init_op) + (assign44810_e49908 * var_x2init_op_dn7)) - (((var_xg2x_ac_dn7 * var_r2init_op) - (var_xg2x_ac * var_r2init_op_dn7)) / (var_r2init_op * var_r2init_op))), ((((-(var_r2init_op_dn8 / (var_r2init_op * var_r2init_op))) * var_x2init_op) + (assign44810_e49908 * var_x2init_op_dn8)) - (((var_xg2x_ac_dn8 * var_r2init_op) - (var_xg2x_ac * var_r2init_op_dn8)) / (var_r2init_op * var_r2init_op))), ((((-(var_r2init_op_dn9 / (var_r2init_op * var_r2init_op))) * var_x2init_op) + (assign44810_e49908 * var_x2init_op_dn9)) - (((var_xg2x_ac_dn9 * var_r2init_op) - (var_xg2x_ac * var_r2init_op_dn9)) / (var_r2init_op * var_r2init_op))),)
    } else {
        (var_xth2init_op, var_xth2init_op_dn4, var_xth2init_op_dn6, var_xth2init_op_dn7, var_xth2init_op_dn8, var_xth2init_op_dn9,)
    }
};
        var_xth2init_op = assign44810_e49916;
        var_xth2init_op_dn4 = assign44810_e49916_d_n4;
        var_xth2init_op_dn6 = assign44810_e49916_d_n6;
        var_xth2init_op_dn7 = assign44810_e49916_d_n7;
        var_xth2init_op_dn8 = assign44810_e49916_d_n8;
        var_xth2init_op_dn9 = assign44810_e49916_d_n9;

        let (assign44820_e49941, assign44820_e49941_d_n4, assign44820_e49941_d_n6, assign44820_e49941_d_n7, assign44820_e49941_d_n8, assign44820_e49941_d_n9,) = {
    if (var_guard1360 != 0.0) {
        let assign44820_e49921: f64 = (var_xth1init_op + var_xth2init_op);
        let assign44820_e49924: f64 = (var_xth1init_op - var_xth2init_op);
        let assign44820_e49927: f64 = (var_xth1init_op - var_xth2init_op);
        let assign44820_e49928: f64 = (assign44820_e49924 * assign44820_e49927);
        let assign44820_e49930: f64 = (assign44820_e49928 + 38.0);
        let assign44820_e49931: f64 = (assign44820_e49930).sqrt();
        let assign44820_e49932: f64 = (assign44820_e49921 - assign44820_e49931);
        let assign44820_e49933: f64 = (0.5 * assign44820_e49932);
        let assign44820_e49935: f64 = (assign44820_e49933 - var_xg2_ac);
        let assign44820_e49937: f64 = (assign44820_e49935 / var_cic1_i);
        let assign44820_e49939: f64 = (assign44820_e49937 + var_xg2_ac);
        (assign44820_e49939, ((((0.5 * ((var_xth1init_op_dn4 + var_xth2init_op_dn4) - ((((var_xth1init_op_dn4 - var_xth2init_op_dn4) * assign44820_e49927) + (assign44820_e49924 * (var_xth1init_op_dn4 - var_xth2init_op_dn4))) / (2.0 * assign44820_e49931)))) - var_xg2_ac_dn4) / var_cic1_i) + var_xg2_ac_dn4), ((((0.5 * ((var_xth1init_op_dn6 + var_xth2init_op_dn6) - ((((var_xth1init_op_dn6 - var_xth2init_op_dn6) * assign44820_e49927) + (assign44820_e49924 * (var_xth1init_op_dn6 - var_xth2init_op_dn6))) / (2.0 * assign44820_e49931)))) - var_xg2_ac_dn6) / var_cic1_i) + var_xg2_ac_dn6), ((((0.5 * ((var_xth1init_op_dn7 + var_xth2init_op_dn7) - ((((var_xth1init_op_dn7 - var_xth2init_op_dn7) * assign44820_e49927) + (assign44820_e49924 * (var_xth1init_op_dn7 - var_xth2init_op_dn7))) / (2.0 * assign44820_e49931)))) - var_xg2_ac_dn7) / var_cic1_i) + var_xg2_ac_dn7), ((((0.5 * ((var_xth1init_op_dn8 + var_xth2init_op_dn8) - ((((var_xth1init_op_dn8 - var_xth2init_op_dn8) * assign44820_e49927) + (assign44820_e49924 * (var_xth1init_op_dn8 - var_xth2init_op_dn8))) / (2.0 * assign44820_e49931)))) - var_xg2_ac_dn8) / var_cic1_i) + var_xg2_ac_dn8), ((((0.5 * ((var_xth1init_op_dn9 + var_xth2init_op_dn9) - ((((var_xth1init_op_dn9 - var_xth2init_op_dn9) * assign44820_e49927) + (assign44820_e49924 * (var_xth1init_op_dn9 - var_xth2init_op_dn9))) / (2.0 * assign44820_e49931)))) - var_xg2_ac_dn9) / var_cic1_i) + var_xg2_ac_dn9),)
    } else {
        (var_xg1thinit_op, var_xg1thinit_op_dn4, var_xg1thinit_op_dn6, var_xg1thinit_op_dn7, var_xg1thinit_op_dn8, var_xg1thinit_op_dn9,)
    }
};
        var_xg1thinit_op = assign44820_e49941;
        var_xg1thinit_op_dn4 = assign44820_e49941_d_n4;
        var_xg1thinit_op_dn6 = assign44820_e49941_d_n6;
        var_xg1thinit_op_dn7 = assign44820_e49941_d_n7;
        var_xg1thinit_op_dn8 = assign44820_e49941_d_n8;
        var_xg1thinit_op_dn9 = assign44820_e49941_d_n9;

        *var_dleff_op_slot = var_dleff_op;
        *var_dleff_op_dn4_slot = var_dleff_op_dn4;
        *var_dleff_op_dn6_slot = var_dleff_op_dn6;
        *var_dleff_op_dn7_slot = var_dleff_op_dn7;
        *var_dleff_op_dn8_slot = var_dleff_op_dn8;
        *var_dleff_op_dn9_slot = var_dleff_op_dn9;
        *var_dx_wi_1d_op_slot = var_dx_wi_1d_op;
        *var_dx_wi_1d_op_dn4_slot = var_dx_wi_1d_op_dn4;
        *var_dx_wi_1d_op_dn6_slot = var_dx_wi_1d_op_dn6;
        *var_dx_wi_1d_op_dn7_slot = var_dx_wi_1d_op_dn7;
        *var_dx_wi_1d_op_dn8_slot = var_dx_wi_1d_op_dn8;
        *var_dx_wi_1d_op_dn9_slot = var_dx_wi_1d_op_dn9;
        *var_e2_op_slot = var_e2_op;
        *var_e2_op_dn4_slot = var_e2_op_dn4;
        *var_e2_op_dn6_slot = var_e2_op_dn6;
        *var_e2_op_dn7_slot = var_e2_op_dn7;
        *var_e2_op_dn8_slot = var_e2_op_dn8;
        *var_e2_op_dn9_slot = var_e2_op_dn9;
        *var_guard1356_slot = var_guard1356;
        *var_guard1357_slot = var_guard1357;
        *var_guard1358_slot = var_guard1358;
        *var_guard1360_slot = var_guard1360;
        *var_k1_1d_qm_op_slot = var_k1_1d_qm_op;
        *var_k1_1d_qm_op_dn4_slot = var_k1_1d_qm_op_dn4;
        *var_k1_1d_qm_op_dn6_slot = var_k1_1d_qm_op_dn6;
        *var_k1_1d_qm_op_dn7_slot = var_k1_1d_qm_op_dn7;
        *var_k1_1d_qm_op_dn8_slot = var_k1_1d_qm_op_dn8;
        *var_k1_1d_qm_op_dn9_slot = var_k1_1d_qm_op_dn9;
        *var_k2_1d_qm_op_slot = var_k2_1d_qm_op;
        *var_k2_1d_qm_op_dn4_slot = var_k2_1d_qm_op_dn4;
        *var_k2_1d_qm_op_dn6_slot = var_k2_1d_qm_op_dn6;
        *var_k2_1d_qm_op_dn7_slot = var_k2_1d_qm_op_dn7;
        *var_k2_1d_qm_op_dn8_slot = var_k2_1d_qm_op_dn8;
        *var_k2_1d_qm_op_dn9_slot = var_k2_1d_qm_op_dn9;
        *var_keq_1d_qm_op_slot = var_keq_1d_qm_op;
        *var_keq_1d_qm_op_dn4_slot = var_keq_1d_qm_op_dn4;
        *var_keq_1d_qm_op_dn6_slot = var_keq_1d_qm_op_dn6;
        *var_keq_1d_qm_op_dn7_slot = var_keq_1d_qm_op_dn7;
        *var_keq_1d_qm_op_dn8_slot = var_keq_1d_qm_op_dn8;
        *var_keq_1d_qm_op_dn9_slot = var_keq_1d_qm_op_dn9;
        *var_r1init_op_slot = var_r1init_op;
        *var_r1init_op_dn4_slot = var_r1init_op_dn4;
        *var_r1init_op_dn6_slot = var_r1init_op_dn6;
        *var_r1init_op_dn7_slot = var_r1init_op_dn7;
        *var_r1init_op_dn8_slot = var_r1init_op_dn8;
        *var_r1init_op_dn9_slot = var_r1init_op_dn9;
        *var_r2init_op_slot = var_r2init_op;
        *var_r2init_op_dn4_slot = var_r2init_op_dn4;
        *var_r2init_op_dn6_slot = var_r2init_op_dn6;
        *var_r2init_op_dn7_slot = var_r2init_op_dn7;
        *var_r2init_op_dn8_slot = var_r2init_op_dn8;
        *var_r2init_op_dn9_slot = var_r2init_op_dn9;
        *var_temp_slot = var_temp;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp3_slot = var_temp3;
        *var_temp3_dn4_slot = var_temp3_dn4;
        *var_temp3_dn6_slot = var_temp3_dn6;
        *var_temp3_dn7_slot = var_temp3_dn7;
        *var_temp3_dn8_slot = var_temp3_dn8;
        *var_temp3_dn9_slot = var_temp3_dn9;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_x1init_op_slot = var_x1init_op;
        *var_x1init_op_dn4_slot = var_x1init_op_dn4;
        *var_x1init_op_dn6_slot = var_x1init_op_dn6;
        *var_x1init_op_dn7_slot = var_x1init_op_dn7;
        *var_x1init_op_dn8_slot = var_x1init_op_dn8;
        *var_x1init_op_dn9_slot = var_x1init_op_dn9;
        *var_x2init_op_slot = var_x2init_op;
        *var_x2init_op_dn4_slot = var_x2init_op_dn4;
        *var_x2init_op_dn6_slot = var_x2init_op_dn6;
        *var_x2init_op_dn7_slot = var_x2init_op_dn7;
        *var_x2init_op_dn8_slot = var_x2init_op_dn8;
        *var_x2init_op_dn9_slot = var_x2init_op_dn9;
        *var_x_1d_op_slot = var_x_1d_op;
        *var_x_1d_op_dn4_slot = var_x_1d_op_dn4;
        *var_x_1d_op_dn6_slot = var_x_1d_op_dn6;
        *var_x_1d_op_dn7_slot = var_x_1d_op_dn7;
        *var_x_1d_op_dn8_slot = var_x_1d_op_dn8;
        *var_x_1d_op_dn9_slot = var_x_1d_op_dn9;
        *var_x_wi_1d_op_slot = var_x_wi_1d_op;
        *var_x_wi_1d_op_dn4_slot = var_x_wi_1d_op_dn4;
        *var_x_wi_1d_op_dn6_slot = var_x_wi_1d_op_dn6;
        *var_x_wi_1d_op_dn7_slot = var_x_wi_1d_op_dn7;
        *var_x_wi_1d_op_dn8_slot = var_x_wi_1d_op_dn8;
        *var_x_wi_1d_op_dn9_slot = var_x_wi_1d_op_dn9;
        *var_xg1thinit_op_slot = var_xg1thinit_op;
        *var_xg1thinit_op_dn4_slot = var_xg1thinit_op_dn4;
        *var_xg1thinit_op_dn6_slot = var_xg1thinit_op_dn6;
        *var_xg1thinit_op_dn7_slot = var_xg1thinit_op_dn7;
        *var_xg1thinit_op_dn8_slot = var_xg1thinit_op_dn8;
        *var_xg1thinit_op_dn9_slot = var_xg1thinit_op_dn9;
        *var_xth1init_op_slot = var_xth1init_op;
        *var_xth1init_op_dn4_slot = var_xth1init_op_dn4;
        *var_xth1init_op_dn6_slot = var_xth1init_op_dn6;
        *var_xth1init_op_dn7_slot = var_xth1init_op_dn7;
        *var_xth1init_op_dn8_slot = var_xth1init_op_dn8;
        *var_xth1init_op_dn9_slot = var_xth1init_op_dn9;
        *var_xth2init_op_slot = var_xth2init_op;
        *var_xth2init_op_dn4_slot = var_xth2init_op_dn4;
        *var_xth2init_op_dn6_slot = var_xth2init_op_dn6;
        *var_xth2init_op_dn7_slot = var_xth2init_op_dn7;
        *var_xth2init_op_dn8_slot = var_xth2init_op_dn8;
        *var_xth2init_op_dn9_slot = var_xth2init_op_dn9;
    }

    pub(super) fn stamp_transient_block_123(
        p: &Parameters,
        var_dvfb1nch: f64,
        var_dvfb1nch_dn4: f64,
        var_dvfb1nch_dn6: f64,
        var_dvfb1nch_dn7: f64,
        var_dvfb1nch_dn8: f64,
        var_dvfb1nch_dn9: f64,
        var_dvfb2nch: f64,
        var_dvfb2nch_dn4: f64,
        var_dvfb2nch_dn6: f64,
        var_dvfb2nch_dn7: f64,
        var_dvfb2nch_dn8: f64,
        var_dvfb2nch_dn9: f64,
        var_dvfbch_op: f64,
        var_dvfbch_op_dn4: f64,
        var_dvfbch_op_dn6: f64,
        var_dvfbch_op_dn7: f64,
        var_dvfbch_op_dn8: f64,
        var_dvfbch_op_dn9: f64,
        var_dvfbpdep_op: f64,
        var_dvfbpdep_op_dn4: f64,
        var_dvfbpdep_op_dn6: f64,
        var_dvfbpdep_op_dn7: f64,
        var_dvfbpdep_op_dn8: f64,
        var_dvfbpdep_op_dn9: f64,
        var_dvfbqm: f64,
        var_dxdsx_op: f64,
        var_dxdsx_op_dn4: f64,
        var_dxdsx_op_dn6: f64,
        var_dxdsx_op_dn7: f64,
        var_dxdsx_op_dn8: f64,
        var_dxdsx_op_dn9: f64,
        var_dxg1_dibl_ac: f64,
        var_dxg1_dibl_ac_dn4: f64,
        var_dxg1_dibl_ac_dn6: f64,
        var_dxg1_dibl_ac_dn7: f64,
        var_dxg1_dibl_ac_dn8: f64,
        var_dxg1_dibl_ac_dn9: f64,
        var_emin: f64,
        var_emin_dn4: f64,
        var_emin_dn6: f64,
        var_emin_dn7: f64,
        var_emin_dn8: f64,
        var_emin_dn9: f64,
        var_gfsub: f64,
        var_gfsub_dn4: f64,
        var_gfsub_dn6: f64,
        var_gfsub_dn7: f64,
        var_gfsub_dn8: f64,
        var_gfsub_dn9: f64,
        var_guard1360: f64,
        var_inv_phit_op: f64,
        var_inv_phit_op_dn4: f64,
        var_inv_phit_op_dn6: f64,
        var_inv_phit_op_dn7: f64,
        var_inv_phit_op_dn8: f64,
        var_inv_phit_op_dn9: f64,
        var_k1_1d: f64,
        var_k2_1d: f64,
        var_keq_1d: f64,
        var_phit: f64,
        var_phit_dn4: f64,
        var_phit_dn6: f64,
        var_phit_dn7: f64,
        var_phit_dn8: f64,
        var_phit_dn9: f64,
        var_qq_op: f64,
        var_qq_op_dn4: f64,
        var_qq_op_dn6: f64,
        var_qq_op_dn7: f64,
        var_qq_op_dn8: f64,
        var_qq_op_dn9: f64,
        var_sce1_ac: f64,
        var_sce1_ac_dn4: f64,
        var_sce1_ac_dn6: f64,
        var_sce1_ac_dn7: f64,
        var_sce1_ac_dn8: f64,
        var_sce1_ac_dn9: f64,
        var_stcf_i: f64,
        var_stcf_i_dn4: f64,
        var_stcf_i_dn6: f64,
        var_stcf_i_dn7: f64,
        var_stcf_i_dn8: f64,
        var_stcf_i_dn9: f64,
        var_stvfb_i: f64,
        var_tkd: f64,
        var_tkd_dn4: f64,
        var_tkd_dn6: f64,
        var_tkd_dn7: f64,
        var_tkd_dn8: f64,
        var_tkd_dn9: f64,
        var_tkr: f64,
        var_typesub_i: f64,
        var_vfbac1_i: f64,
        var_vfbac1_i_dn4: f64,
        var_vfbac1_i_dn6: f64,
        var_vfbac1_i_dn7: f64,
        var_vfbac1_i_dn8: f64,
        var_vfbac1_i_dn9: f64,
        var_vfbac1_t: f64,
        var_vfbac1_t_dn4: f64,
        var_vfbac1_t_dn6: f64,
        var_vfbac1_t_dn7: f64,
        var_vfbac1_t_dn8: f64,
        var_vfbac1_t_dn9: f64,
        var_vfbac2_t: f64,
        var_vfbac2_t_dn4: f64,
        var_vfbac2_t_dn6: f64,
        var_vfbac2_t_dn7: f64,
        var_vfbac2_t_dn8: f64,
        var_vfbac2_t_dn9: f64,
        var_vsb: f64,
        var_vsb_dn6: f64,
        var_vsb_dn7: f64,
        var_vsb_dn8: f64,
        var_xedge_ac: f64,
        var_xedge_ac_dn4: f64,
        var_xedge_ac_dn6: f64,
        var_xedge_ac_dn7: f64,
        var_xedge_ac_dn8: f64,
        var_xedge_ac_dn9: f64,
        var_xg1thinit_op: f64,
        var_xg1thinit_op_dn4: f64,
        var_xg1thinit_op_dn6: f64,
        var_xg1thinit_op_dn7: f64,
        var_xg1thinit_op_dn8: f64,
        var_xg1thinit_op_dn9: f64,
        var_dx_wi_1d_op_slot: &mut f64,
        var_dx_wi_1d_op_dn4_slot: &mut f64,
        var_dx_wi_1d_op_dn6_slot: &mut f64,
        var_dx_wi_1d_op_dn7_slot: &mut f64,
        var_dx_wi_1d_op_dn8_slot: &mut f64,
        var_dx_wi_1d_op_dn9_slot: &mut f64,
        var_e1_op_slot: &mut f64,
        var_e1_op_dn4_slot: &mut f64,
        var_e1_op_dn6_slot: &mut f64,
        var_e1_op_dn7_slot: &mut f64,
        var_e1_op_dn8_slot: &mut f64,
        var_e1_op_dn9_slot: &mut f64,
        var_e2_op_slot: &mut f64,
        var_e2_op_dn4_slot: &mut f64,
        var_e2_op_dn6_slot: &mut f64,
        var_e2_op_dn7_slot: &mut f64,
        var_e2_op_dn8_slot: &mut f64,
        var_e2_op_dn9_slot: &mut f64,
        var_guard1361_slot: &mut f64,
        var_guard1362_slot: &mut f64,
        var_guard1363_slot: &mut f64,
        var_guard1364_slot: &mut f64,
        var_guard1365_slot: &mut f64,
        var_k1_1d_qm_op_slot: &mut f64,
        var_k1_1d_qm_op_dn4_slot: &mut f64,
        var_k1_1d_qm_op_dn6_slot: &mut f64,
        var_k1_1d_qm_op_dn7_slot: &mut f64,
        var_k1_1d_qm_op_dn8_slot: &mut f64,
        var_k1_1d_qm_op_dn9_slot: &mut f64,
        var_k2_1d_qm_op_slot: &mut f64,
        var_k2_1d_qm_op_dn4_slot: &mut f64,
        var_k2_1d_qm_op_dn6_slot: &mut f64,
        var_k2_1d_qm_op_dn7_slot: &mut f64,
        var_k2_1d_qm_op_dn8_slot: &mut f64,
        var_k2_1d_qm_op_dn9_slot: &mut f64,
        var_keq_1d_qm_op_slot: &mut f64,
        var_keq_1d_qm_op_dn4_slot: &mut f64,
        var_keq_1d_qm_op_dn6_slot: &mut f64,
        var_keq_1d_qm_op_dn7_slot: &mut f64,
        var_keq_1d_qm_op_dn8_slot: &mut f64,
        var_keq_1d_qm_op_dn9_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp3_slot: &mut f64,
        var_temp3_dn4_slot: &mut f64,
        var_temp3_dn6_slot: &mut f64,
        var_temp3_dn7_slot: &mut f64,
        var_temp3_dn8_slot: &mut f64,
        var_temp3_dn9_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_vfb1_op_slot: &mut f64,
        var_vfb1_op_dn4_slot: &mut f64,
        var_vfb1_op_dn6_slot: &mut f64,
        var_vfb1_op_dn7_slot: &mut f64,
        var_vfb1_op_dn8_slot: &mut f64,
        var_vfb1_op_dn9_slot: &mut f64,
        var_vfb2_op_slot: &mut f64,
        var_vfb2_op_dn4_slot: &mut f64,
        var_vfb2_op_dn6_slot: &mut f64,
        var_vfb2_op_dn7_slot: &mut f64,
        var_vfb2_op_dn8_slot: &mut f64,
        var_vfb2_op_dn9_slot: &mut f64,
        var_vthinit_op_slot: &mut f64,
        var_vthinit_op_dn4_slot: &mut f64,
        var_vthinit_op_dn6_slot: &mut f64,
        var_vthinit_op_dn7_slot: &mut f64,
        var_vthinit_op_dn8_slot: &mut f64,
        var_vthinit_op_dn9_slot: &mut f64,
        var_x_wi_1d_op_slot: &mut f64,
        var_x_wi_1d_op_dn4_slot: &mut f64,
        var_x_wi_1d_op_dn6_slot: &mut f64,
        var_x_wi_1d_op_dn7_slot: &mut f64,
        var_x_wi_1d_op_dn8_slot: &mut f64,
        var_x_wi_1d_op_dn9_slot: &mut f64,
        var_xg10_op_slot: &mut f64,
        var_xg10_op_dn4_slot: &mut f64,
        var_xg10_op_dn6_slot: &mut f64,
        var_xg10_op_dn7_slot: &mut f64,
        var_xg10_op_dn8_slot: &mut f64,
        var_xg10_op_dn9_slot: &mut f64,
        var_xg20_op_slot: &mut f64,
        var_xg20_op_dn4_slot: &mut f64,
        var_xg20_op_dn6_slot: &mut f64,
        var_xg20_op_dn7_slot: &mut f64,
        var_xg20_op_dn8_slot: &mut f64,
        var_xg20_op_dn9_slot: &mut f64,
        var_xg2eff_op_slot: &mut f64,
        var_xg2eff_op_dn4_slot: &mut f64,
        var_xg2eff_op_dn6_slot: &mut f64,
        var_xg2eff_op_dn7_slot: &mut f64,
        var_xg2eff_op_dn8_slot: &mut f64,
        var_xg2eff_op_dn9_slot: &mut f64,
    ) {
        let mut var_dx_wi_1d_op: f64 = *var_dx_wi_1d_op_slot;
        let mut var_dx_wi_1d_op_dn4: f64 = *var_dx_wi_1d_op_dn4_slot;
        let mut var_dx_wi_1d_op_dn6: f64 = *var_dx_wi_1d_op_dn6_slot;
        let mut var_dx_wi_1d_op_dn7: f64 = *var_dx_wi_1d_op_dn7_slot;
        let mut var_dx_wi_1d_op_dn8: f64 = *var_dx_wi_1d_op_dn8_slot;
        let mut var_dx_wi_1d_op_dn9: f64 = *var_dx_wi_1d_op_dn9_slot;
        let mut var_e1_op: f64 = *var_e1_op_slot;
        let mut var_e1_op_dn4: f64 = *var_e1_op_dn4_slot;
        let mut var_e1_op_dn6: f64 = *var_e1_op_dn6_slot;
        let mut var_e1_op_dn7: f64 = *var_e1_op_dn7_slot;
        let mut var_e1_op_dn8: f64 = *var_e1_op_dn8_slot;
        let mut var_e1_op_dn9: f64 = *var_e1_op_dn9_slot;
        let mut var_e2_op: f64 = *var_e2_op_slot;
        let mut var_e2_op_dn4: f64 = *var_e2_op_dn4_slot;
        let mut var_e2_op_dn6: f64 = *var_e2_op_dn6_slot;
        let mut var_e2_op_dn7: f64 = *var_e2_op_dn7_slot;
        let mut var_e2_op_dn8: f64 = *var_e2_op_dn8_slot;
        let mut var_e2_op_dn9: f64 = *var_e2_op_dn9_slot;
        let mut var_guard1361: f64 = *var_guard1361_slot;
        let mut var_guard1362: f64 = *var_guard1362_slot;
        let mut var_guard1363: f64 = *var_guard1363_slot;
        let mut var_guard1364: f64 = *var_guard1364_slot;
        let mut var_guard1365: f64 = *var_guard1365_slot;
        let mut var_k1_1d_qm_op: f64 = *var_k1_1d_qm_op_slot;
        let mut var_k1_1d_qm_op_dn4: f64 = *var_k1_1d_qm_op_dn4_slot;
        let mut var_k1_1d_qm_op_dn6: f64 = *var_k1_1d_qm_op_dn6_slot;
        let mut var_k1_1d_qm_op_dn7: f64 = *var_k1_1d_qm_op_dn7_slot;
        let mut var_k1_1d_qm_op_dn8: f64 = *var_k1_1d_qm_op_dn8_slot;
        let mut var_k1_1d_qm_op_dn9: f64 = *var_k1_1d_qm_op_dn9_slot;
        let mut var_k2_1d_qm_op: f64 = *var_k2_1d_qm_op_slot;
        let mut var_k2_1d_qm_op_dn4: f64 = *var_k2_1d_qm_op_dn4_slot;
        let mut var_k2_1d_qm_op_dn6: f64 = *var_k2_1d_qm_op_dn6_slot;
        let mut var_k2_1d_qm_op_dn7: f64 = *var_k2_1d_qm_op_dn7_slot;
        let mut var_k2_1d_qm_op_dn8: f64 = *var_k2_1d_qm_op_dn8_slot;
        let mut var_k2_1d_qm_op_dn9: f64 = *var_k2_1d_qm_op_dn9_slot;
        let mut var_keq_1d_qm_op: f64 = *var_keq_1d_qm_op_slot;
        let mut var_keq_1d_qm_op_dn4: f64 = *var_keq_1d_qm_op_dn4_slot;
        let mut var_keq_1d_qm_op_dn6: f64 = *var_keq_1d_qm_op_dn6_slot;
        let mut var_keq_1d_qm_op_dn7: f64 = *var_keq_1d_qm_op_dn7_slot;
        let mut var_keq_1d_qm_op_dn8: f64 = *var_keq_1d_qm_op_dn8_slot;
        let mut var_keq_1d_qm_op_dn9: f64 = *var_keq_1d_qm_op_dn9_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp3: f64 = *var_temp3_slot;
        let mut var_temp3_dn4: f64 = *var_temp3_dn4_slot;
        let mut var_temp3_dn6: f64 = *var_temp3_dn6_slot;
        let mut var_temp3_dn7: f64 = *var_temp3_dn7_slot;
        let mut var_temp3_dn8: f64 = *var_temp3_dn8_slot;
        let mut var_temp3_dn9: f64 = *var_temp3_dn9_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_vfb1_op: f64 = *var_vfb1_op_slot;
        let mut var_vfb1_op_dn4: f64 = *var_vfb1_op_dn4_slot;
        let mut var_vfb1_op_dn6: f64 = *var_vfb1_op_dn6_slot;
        let mut var_vfb1_op_dn7: f64 = *var_vfb1_op_dn7_slot;
        let mut var_vfb1_op_dn8: f64 = *var_vfb1_op_dn8_slot;
        let mut var_vfb1_op_dn9: f64 = *var_vfb1_op_dn9_slot;
        let mut var_vfb2_op: f64 = *var_vfb2_op_slot;
        let mut var_vfb2_op_dn4: f64 = *var_vfb2_op_dn4_slot;
        let mut var_vfb2_op_dn6: f64 = *var_vfb2_op_dn6_slot;
        let mut var_vfb2_op_dn7: f64 = *var_vfb2_op_dn7_slot;
        let mut var_vfb2_op_dn8: f64 = *var_vfb2_op_dn8_slot;
        let mut var_vfb2_op_dn9: f64 = *var_vfb2_op_dn9_slot;
        let mut var_vthinit_op: f64 = *var_vthinit_op_slot;
        let mut var_vthinit_op_dn4: f64 = *var_vthinit_op_dn4_slot;
        let mut var_vthinit_op_dn6: f64 = *var_vthinit_op_dn6_slot;
        let mut var_vthinit_op_dn7: f64 = *var_vthinit_op_dn7_slot;
        let mut var_vthinit_op_dn8: f64 = *var_vthinit_op_dn8_slot;
        let mut var_vthinit_op_dn9: f64 = *var_vthinit_op_dn9_slot;
        let mut var_x_wi_1d_op: f64 = *var_x_wi_1d_op_slot;
        let mut var_x_wi_1d_op_dn4: f64 = *var_x_wi_1d_op_dn4_slot;
        let mut var_x_wi_1d_op_dn6: f64 = *var_x_wi_1d_op_dn6_slot;
        let mut var_x_wi_1d_op_dn7: f64 = *var_x_wi_1d_op_dn7_slot;
        let mut var_x_wi_1d_op_dn8: f64 = *var_x_wi_1d_op_dn8_slot;
        let mut var_x_wi_1d_op_dn9: f64 = *var_x_wi_1d_op_dn9_slot;
        let mut var_xg10_op: f64 = *var_xg10_op_slot;
        let mut var_xg10_op_dn4: f64 = *var_xg10_op_dn4_slot;
        let mut var_xg10_op_dn6: f64 = *var_xg10_op_dn6_slot;
        let mut var_xg10_op_dn7: f64 = *var_xg10_op_dn7_slot;
        let mut var_xg10_op_dn8: f64 = *var_xg10_op_dn8_slot;
        let mut var_xg10_op_dn9: f64 = *var_xg10_op_dn9_slot;
        let mut var_xg20_op: f64 = *var_xg20_op_slot;
        let mut var_xg20_op_dn4: f64 = *var_xg20_op_dn4_slot;
        let mut var_xg20_op_dn6: f64 = *var_xg20_op_dn6_slot;
        let mut var_xg20_op_dn7: f64 = *var_xg20_op_dn7_slot;
        let mut var_xg20_op_dn8: f64 = *var_xg20_op_dn8_slot;
        let mut var_xg20_op_dn9: f64 = *var_xg20_op_dn9_slot;
        let mut var_xg2eff_op: f64 = *var_xg2eff_op_slot;
        let mut var_xg2eff_op_dn4: f64 = *var_xg2eff_op_dn4_slot;
        let mut var_xg2eff_op_dn6: f64 = *var_xg2eff_op_dn6_slot;
        let mut var_xg2eff_op_dn7: f64 = *var_xg2eff_op_dn7_slot;
        let mut var_xg2eff_op_dn8: f64 = *var_xg2eff_op_dn8_slot;
        let mut var_xg2eff_op_dn9: f64 = *var_xg2eff_op_dn9_slot;

        let (assign44830_e49957, assign44830_e49957_d_n4, assign44830_e49957_d_n6, assign44830_e49957_d_n7, assign44830_e49957_d_n8, assign44830_e49957_d_n9,) = {
    if (var_guard1360 != 0.0) {
        let assign44830_e49946: f64 = (var_xg1thinit_op - var_xedge_ac);
        let assign44830_e49948: f64 = (assign44830_e49946 / var_sce1_ac);
        let assign44830_e49950: f64 = (assign44830_e49948 - var_dxg1_dibl_ac);
        let assign44830_e49952: f64 = (assign44830_e49950 + var_xedge_ac);
        let assign44830_e49953: f64 = (var_phit * assign44830_e49952);
        let assign44830_e49955: f64 = (assign44830_e49953 + var_vfbac1_i);
        (assign44830_e49955, (((var_phit_dn4 * assign44830_e49952) + (var_phit * ((((((var_xg1thinit_op_dn4 - var_xedge_ac_dn4) * var_sce1_ac) - (assign44830_e49946 * var_sce1_ac_dn4)) / (var_sce1_ac * var_sce1_ac)) - var_dxg1_dibl_ac_dn4) + var_xedge_ac_dn4))) + var_vfbac1_i_dn4), (((var_phit_dn6 * assign44830_e49952) + (var_phit * ((((((var_xg1thinit_op_dn6 - var_xedge_ac_dn6) * var_sce1_ac) - (assign44830_e49946 * var_sce1_ac_dn6)) / (var_sce1_ac * var_sce1_ac)) - var_dxg1_dibl_ac_dn6) + var_xedge_ac_dn6))) + var_vfbac1_i_dn6), (((var_phit_dn7 * assign44830_e49952) + (var_phit * ((((((var_xg1thinit_op_dn7 - var_xedge_ac_dn7) * var_sce1_ac) - (assign44830_e49946 * var_sce1_ac_dn7)) / (var_sce1_ac * var_sce1_ac)) - var_dxg1_dibl_ac_dn7) + var_xedge_ac_dn7))) + var_vfbac1_i_dn7), (((var_phit_dn8 * assign44830_e49952) + (var_phit * ((((((var_xg1thinit_op_dn8 - var_xedge_ac_dn8) * var_sce1_ac) - (assign44830_e49946 * var_sce1_ac_dn8)) / (var_sce1_ac * var_sce1_ac)) - var_dxg1_dibl_ac_dn8) + var_xedge_ac_dn8))) + var_vfbac1_i_dn8), (((var_phit_dn9 * assign44830_e49952) + (var_phit * ((((((var_xg1thinit_op_dn9 - var_xedge_ac_dn9) * var_sce1_ac) - (assign44830_e49946 * var_sce1_ac_dn9)) / (var_sce1_ac * var_sce1_ac)) - var_dxg1_dibl_ac_dn9) + var_xedge_ac_dn9))) + var_vfbac1_i_dn9),)
    } else {
        (var_vthinit_op, var_vthinit_op_dn4, var_vthinit_op_dn6, var_vthinit_op_dn7, var_vthinit_op_dn8, var_vthinit_op_dn9,)
    }
};
        var_vthinit_op = assign44830_e49957;
        var_vthinit_op_dn4 = assign44830_e49957_d_n4;
        var_vthinit_op_dn6 = assign44830_e49957_d_n6;
        var_vthinit_op_dn7 = assign44830_e49957_d_n7;
        var_vthinit_op_dn8 = assign44830_e49957_d_n8;
        var_vthinit_op_dn9 = assign44830_e49957_d_n9;

        let (assign44840_e49965, assign44840_e49965_d_n4, assign44840_e49965_d_n6, assign44840_e49965_d_n7, assign44840_e49965_d_n8, assign44840_e49965_d_n9,) = {
    if (var_guard1360 != 0.0) {
        let assign44840_e49962: f64 = (var_tkd - var_tkr);
        let assign44840_e49963: f64 = (var_stcf_i * assign44840_e49962);
        (assign44840_e49963, ((var_stcf_i_dn4 * assign44840_e49962) + (var_stcf_i * var_tkd_dn4)), ((var_stcf_i_dn6 * assign44840_e49962) + (var_stcf_i * var_tkd_dn6)), ((var_stcf_i_dn7 * assign44840_e49962) + (var_stcf_i * var_tkd_dn7)), ((var_stcf_i_dn8 * assign44840_e49962) + (var_stcf_i * var_tkd_dn8)), ((var_stcf_i_dn9 * assign44840_e49962) + (var_stcf_i * var_tkd_dn9)),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign44840_e49965;
        var_temp_dn4 = assign44840_e49965_d_n4;
        var_temp_dn6 = assign44840_e49965_d_n6;
        var_temp_dn7 = assign44840_e49965_d_n7;
        var_temp_dn8 = assign44840_e49965_d_n8;
        var_temp_dn9 = assign44840_e49965_d_n9;

        let (assign44870_e49989, assign44870_e49989_d_n4, assign44870_e49989_d_n6, assign44870_e49989_d_n7, assign44870_e49989_d_n8, assign44870_e49989_d_n9,) = {
    if (var_guard1360 != 0.0) {
        let assign44870_e49981: f64 = (p.p14 * var_stvfb_i);
        let assign44870_e49984: f64 = (var_tkd - var_tkr);
        let assign44870_e49985: f64 = (assign44870_e49981 * assign44870_e49984);
        let assign44870_e49987: f64 = (assign44870_e49985 + var_dvfbqm);
        (assign44870_e49987, (assign44870_e49981 * var_tkd_dn4), (assign44870_e49981 * var_tkd_dn6), (assign44870_e49981 * var_tkd_dn7), (assign44870_e49981 * var_tkd_dn8), (assign44870_e49981 * var_tkd_dn9),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign44870_e49989;
        var_temp_dn4 = assign44870_e49989_d_n4;
        var_temp_dn6 = assign44870_e49989_d_n6;
        var_temp_dn7 = assign44870_e49989_d_n7;
        var_temp_dn8 = assign44870_e49989_d_n8;
        var_temp_dn9 = assign44870_e49989_d_n9;

        let (assign44880_e50005, assign44880_e50005_d_n4, assign44880_e50005_d_n6, assign44880_e50005_d_n7, assign44880_e50005_d_n8, assign44880_e50005_d_n9,) = {
    if (var_guard1360 != 0.0) {
        let assign44880_e49994: f64 = (var_vfbac1_t + var_dvfbch_op);
        let assign44880_e49996: f64 = (assign44880_e49994 + var_dvfb1nch);
        let assign44880_e49997: f64 = (p.p14 * assign44880_e49996);
        let assign44880_e49999: f64 = (assign44880_e49997 + var_temp);
        let assign44880_e50001: f64 = (assign44880_e49999 + p.p34);
        let assign44880_e50003: f64 = (assign44880_e50001 - var_dvfbpdep_op);
        (assign44880_e50003, (((p.p14 * ((var_vfbac1_t_dn4 + var_dvfbch_op_dn4) + var_dvfb1nch_dn4)) + var_temp_dn4) - var_dvfbpdep_op_dn4), (((p.p14 * ((var_vfbac1_t_dn6 + var_dvfbch_op_dn6) + var_dvfb1nch_dn6)) + var_temp_dn6) - var_dvfbpdep_op_dn6), (((p.p14 * ((var_vfbac1_t_dn7 + var_dvfbch_op_dn7) + var_dvfb1nch_dn7)) + var_temp_dn7) - var_dvfbpdep_op_dn7), (((p.p14 * ((var_vfbac1_t_dn8 + var_dvfbch_op_dn8) + var_dvfb1nch_dn8)) + var_temp_dn8) - var_dvfbpdep_op_dn8), (((p.p14 * ((var_vfbac1_t_dn9 + var_dvfbch_op_dn9) + var_dvfb1nch_dn9)) + var_temp_dn9) - var_dvfbpdep_op_dn9),)
    } else {
        (var_vfb1_op, var_vfb1_op_dn4, var_vfb1_op_dn6, var_vfb1_op_dn7, var_vfb1_op_dn8, var_vfb1_op_dn9,)
    }
};
        var_vfb1_op = assign44880_e50005;
        var_vfb1_op_dn4 = assign44880_e50005_d_n4;
        var_vfb1_op_dn6 = assign44880_e50005_d_n6;
        var_vfb1_op_dn7 = assign44880_e50005_d_n7;
        var_vfb1_op_dn8 = assign44880_e50005_d_n8;
        var_vfb1_op_dn9 = assign44880_e50005_d_n9;

        let (assign44890_e50017, assign44890_e50017_d_n4, assign44890_e50017_d_n6, assign44890_e50017_d_n7, assign44890_e50017_d_n8, assign44890_e50017_d_n9,) = {
    if (var_guard1360 != 0.0) {
        let assign44890_e50010: f64 = (var_vfbac2_t + var_dvfbch_op);
        let assign44890_e50012: f64 = (assign44890_e50010 + var_dvfb2nch);
        let assign44890_e50013: f64 = (p.p14 * assign44890_e50012);
        let assign44890_e50015: f64 = (assign44890_e50013 + var_temp);
        (assign44890_e50015, ((p.p14 * ((var_vfbac2_t_dn4 + var_dvfbch_op_dn4) + var_dvfb2nch_dn4)) + var_temp_dn4), ((p.p14 * ((var_vfbac2_t_dn6 + var_dvfbch_op_dn6) + var_dvfb2nch_dn6)) + var_temp_dn6), ((p.p14 * ((var_vfbac2_t_dn7 + var_dvfbch_op_dn7) + var_dvfb2nch_dn7)) + var_temp_dn7), ((p.p14 * ((var_vfbac2_t_dn8 + var_dvfbch_op_dn8) + var_dvfb2nch_dn8)) + var_temp_dn8), ((p.p14 * ((var_vfbac2_t_dn9 + var_dvfbch_op_dn9) + var_dvfb2nch_dn9)) + var_temp_dn9),)
    } else {
        (var_vfb2_op, var_vfb2_op_dn4, var_vfb2_op_dn6, var_vfb2_op_dn7, var_vfb2_op_dn8, var_vfb2_op_dn9,)
    }
};
        var_vfb2_op = assign44890_e50017;
        var_vfb2_op_dn4 = assign44890_e50017_d_n4;
        var_vfb2_op_dn6 = assign44890_e50017_d_n6;
        var_vfb2_op_dn7 = assign44890_e50017_d_n7;
        var_vfb2_op_dn8 = assign44890_e50017_d_n8;
        var_vfb2_op_dn9 = assign44890_e50017_d_n9;

        let (assign44900_e50027, assign44900_e50027_d_n4, assign44900_e50027_d_n6, assign44900_e50027_d_n7, assign44900_e50027_d_n8, assign44900_e50027_d_n9,) = {
    if (var_guard1360 != 0.0) {
        let assign44900_e50021: f64 = (var_vthinit_op - var_vfb1_op);
        let assign44900_e50023: f64 = (assign44900_e50021 * var_inv_phit_op);
        let assign44900_e50025: f64 = (assign44900_e50023 - var_dxdsx_op);
        (assign44900_e50025, ((((var_vthinit_op_dn4 - var_vfb1_op_dn4) * var_inv_phit_op) + (assign44900_e50021 * var_inv_phit_op_dn4)) - var_dxdsx_op_dn4), ((((var_vthinit_op_dn6 - var_vfb1_op_dn6) * var_inv_phit_op) + (assign44900_e50021 * var_inv_phit_op_dn6)) - var_dxdsx_op_dn6), ((((var_vthinit_op_dn7 - var_vfb1_op_dn7) * var_inv_phit_op) + (assign44900_e50021 * var_inv_phit_op_dn7)) - var_dxdsx_op_dn7), ((((var_vthinit_op_dn8 - var_vfb1_op_dn8) * var_inv_phit_op) + (assign44900_e50021 * var_inv_phit_op_dn8)) - var_dxdsx_op_dn8), ((((var_vthinit_op_dn9 - var_vfb1_op_dn9) * var_inv_phit_op) + (assign44900_e50021 * var_inv_phit_op_dn9)) - var_dxdsx_op_dn9),)
    } else {
        (var_xg10_op, var_xg10_op_dn4, var_xg10_op_dn6, var_xg10_op_dn7, var_xg10_op_dn8, var_xg10_op_dn9,)
    }
};
        var_xg10_op = assign44900_e50027;
        var_xg10_op_dn4 = assign44900_e50027_d_n4;
        var_xg10_op_dn6 = assign44900_e50027_d_n6;
        var_xg10_op_dn7 = assign44900_e50027_d_n7;
        var_xg10_op_dn8 = assign44900_e50027_d_n8;
        var_xg10_op_dn9 = assign44900_e50027_d_n9;

        let (assign44910_e50038, assign44910_e50038_d_n4, assign44910_e50038_d_n6, assign44910_e50038_d_n7, assign44910_e50038_d_n8, assign44910_e50038_d_n9,) = {
    if (var_guard1360 != 0.0) {
        let assign44910_e50030: f64 = (-var_vsb);
        let assign44910_e50032: f64 = (assign44910_e50030 - var_vfb2_op);
        let assign44910_e50034: f64 = (assign44910_e50032 * var_inv_phit_op);
        let assign44910_e50036: f64 = (assign44910_e50034 - var_dxdsx_op);
        (assign44910_e50036, ((((-var_vfb2_op_dn4) * var_inv_phit_op) + (assign44910_e50032 * var_inv_phit_op_dn4)) - var_dxdsx_op_dn4), (((((-var_vsb_dn6) - var_vfb2_op_dn6) * var_inv_phit_op) + (assign44910_e50032 * var_inv_phit_op_dn6)) - var_dxdsx_op_dn6), (((((-var_vsb_dn7) - var_vfb2_op_dn7) * var_inv_phit_op) + (assign44910_e50032 * var_inv_phit_op_dn7)) - var_dxdsx_op_dn7), (((((-var_vsb_dn8) - var_vfb2_op_dn8) * var_inv_phit_op) + (assign44910_e50032 * var_inv_phit_op_dn8)) - var_dxdsx_op_dn8), ((((-var_vfb2_op_dn9) * var_inv_phit_op) + (assign44910_e50032 * var_inv_phit_op_dn9)) - var_dxdsx_op_dn9),)
    } else {
        (var_xg20_op, var_xg20_op_dn4, var_xg20_op_dn6, var_xg20_op_dn7, var_xg20_op_dn8, var_xg20_op_dn9,)
    }
};
        var_xg20_op = assign44910_e50038;
        var_xg20_op_dn4 = assign44910_e50038_d_n4;
        var_xg20_op_dn6 = assign44910_e50038_d_n6;
        var_xg20_op_dn7 = assign44910_e50038_d_n7;
        var_xg20_op_dn8 = assign44910_e50038_d_n8;
        var_xg20_op_dn9 = assign44910_e50038_d_n9;

        let assign44920_e50041: f64 = if p.p2 > 0.0 { 1.0 } else { 0.0 };
        var_guard1361 = assign44920_e50041;

        let (assign44930_e50055, assign44930_e50055_d_n4, assign44930_e50055_d_n6, assign44930_e50055_d_n7, assign44930_e50055_d_n8, assign44930_e50055_d_n9,) = {
    if ((var_guard1360 != 0.0) && (var_guard1361 != 0.0)) {
        let assign44930_e50047: f64 = (p.p14 * var_typesub_i);
        let assign44930_e50050: f64 = (var_xg10_op - var_xg20_op);
        let assign44930_e50051: f64 = (assign44930_e50047 * assign44930_e50050);
        let assign44930_e50053: f64 = (assign44930_e50051 / var_gfsub);
        (assign44930_e50053, ((((assign44930_e50047 * (var_xg10_op_dn4 - var_xg20_op_dn4)) * var_gfsub) - (assign44930_e50051 * var_gfsub_dn4)) / (var_gfsub * var_gfsub)), ((((assign44930_e50047 * (var_xg10_op_dn6 - var_xg20_op_dn6)) * var_gfsub) - (assign44930_e50051 * var_gfsub_dn6)) / (var_gfsub * var_gfsub)), ((((assign44930_e50047 * (var_xg10_op_dn7 - var_xg20_op_dn7)) * var_gfsub) - (assign44930_e50051 * var_gfsub_dn7)) / (var_gfsub * var_gfsub)), ((((assign44930_e50047 * (var_xg10_op_dn8 - var_xg20_op_dn8)) * var_gfsub) - (assign44930_e50051 * var_gfsub_dn8)) / (var_gfsub * var_gfsub)), ((((assign44930_e50047 * (var_xg10_op_dn9 - var_xg20_op_dn9)) * var_gfsub) - (assign44930_e50051 * var_gfsub_dn9)) / (var_gfsub * var_gfsub)),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign44930_e50055;
        var_temp_dn4 = assign44930_e50055_d_n4;
        var_temp_dn6 = assign44930_e50055_d_n6;
        var_temp_dn7 = assign44930_e50055_d_n7;
        var_temp_dn8 = assign44930_e50055_d_n8;
        var_temp_dn9 = assign44930_e50055_d_n9;

        let assign44940_e50058: f64 = if var_temp < 0.0 { 1.0 } else { 0.0 };
        var_guard1362 = assign44940_e50058;

        let (assign44950_e50072, assign44950_e50072_d_n4, assign44950_e50072_d_n6, assign44950_e50072_d_n7, assign44950_e50072_d_n8, assign44950_e50072_d_n9,) = {
    if (((var_guard1360 != 0.0) && (var_guard1361 != 0.0)) && (var_guard1362 != 0.0)) {
        let assign44950_e50065: f64 = (-2.0);
        let assign44950_e50068: f64 = (1.0 - var_temp);
        let assign44950_e50069: f64 = (assign44950_e50068).ln();
        let assign44950_e50070: f64 = (assign44950_e50065 * assign44950_e50069);
        (assign44950_e50070, (assign44950_e50065 * ((-var_temp_dn4) / assign44950_e50068)), (assign44950_e50065 * ((-var_temp_dn6) / assign44950_e50068)), (assign44950_e50065 * ((-var_temp_dn7) / assign44950_e50068)), (assign44950_e50065 * ((-var_temp_dn8) / assign44950_e50068)), (assign44950_e50065 * ((-var_temp_dn9) / assign44950_e50068)),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign44950_e50072;
        var_temp1_dn4 = assign44950_e50072_d_n4;
        var_temp1_dn6 = assign44950_e50072_d_n6;
        var_temp1_dn7 = assign44950_e50072_d_n7;
        var_temp1_dn8 = assign44950_e50072_d_n8;
        var_temp1_dn9 = assign44950_e50072_d_n9;

        let (assign44960_e50091, assign44960_e50091_d_n4, assign44960_e50091_d_n6, assign44960_e50091_d_n7, assign44960_e50091_d_n8, assign44960_e50091_d_n9,) = {
    if (((var_guard1360 != 0.0) && (var_guard1361 != 0.0)) && (var_guard1362 == 0.0)) {
        let assign44960_e50081: f64 = (var_temp * var_temp);
        let assign44960_e50085: f64 = (2.0 * var_temp);
        let assign44960_e50087: f64 = (assign44960_e50085 / var_gfsub);
        let assign44960_e50088: f64 = (1.0 + assign44960_e50087);
        let assign44960_e50089: f64 = (assign44960_e50081 / assign44960_e50088);
        (assign44960_e50089, (((((var_temp_dn4 * var_temp) + (var_temp * var_temp_dn4)) * assign44960_e50088) - (assign44960_e50081 * ((((2.0 * var_temp_dn4) * var_gfsub) - (assign44960_e50085 * var_gfsub_dn4)) / (var_gfsub * var_gfsub)))) / (assign44960_e50088 * assign44960_e50088)), (((((var_temp_dn6 * var_temp) + (var_temp * var_temp_dn6)) * assign44960_e50088) - (assign44960_e50081 * ((((2.0 * var_temp_dn6) * var_gfsub) - (assign44960_e50085 * var_gfsub_dn6)) / (var_gfsub * var_gfsub)))) / (assign44960_e50088 * assign44960_e50088)), (((((var_temp_dn7 * var_temp) + (var_temp * var_temp_dn7)) * assign44960_e50088) - (assign44960_e50081 * ((((2.0 * var_temp_dn7) * var_gfsub) - (assign44960_e50085 * var_gfsub_dn7)) / (var_gfsub * var_gfsub)))) / (assign44960_e50088 * assign44960_e50088)), (((((var_temp_dn8 * var_temp) + (var_temp * var_temp_dn8)) * assign44960_e50088) - (assign44960_e50081 * ((((2.0 * var_temp_dn8) * var_gfsub) - (assign44960_e50085 * var_gfsub_dn8)) / (var_gfsub * var_gfsub)))) / (assign44960_e50088 * assign44960_e50088)), (((((var_temp_dn9 * var_temp) + (var_temp * var_temp_dn9)) * assign44960_e50088) - (assign44960_e50081 * ((((2.0 * var_temp_dn9) * var_gfsub) - (assign44960_e50085 * var_gfsub_dn9)) / (var_gfsub * var_gfsub)))) / (assign44960_e50088 * assign44960_e50088)),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign44960_e50091;
        var_temp1_dn4 = assign44960_e50091_d_n4;
        var_temp1_dn6 = assign44960_e50091_d_n6;
        var_temp1_dn7 = assign44960_e50091_d_n7;
        var_temp1_dn8 = assign44960_e50091_d_n8;
        var_temp1_dn9 = assign44960_e50091_d_n9;

        let (assign44970_e50103, assign44970_e50103_d_n4, assign44970_e50103_d_n6, assign44970_e50103_d_n7, assign44970_e50103_d_n8, assign44970_e50103_d_n9,) = {
    if ((var_guard1360 != 0.0) && (var_guard1361 != 0.0)) {
        let assign44970_e50098: f64 = (p.p14 * var_typesub_i);
        let assign44970_e50100: f64 = (assign44970_e50098 * var_temp1);
        let assign44970_e50101: f64 = (var_xg20_op + assign44970_e50100);
        (assign44970_e50101, (var_xg20_op_dn4 + (assign44970_e50098 * var_temp1_dn4)), (var_xg20_op_dn6 + (assign44970_e50098 * var_temp1_dn6)), (var_xg20_op_dn7 + (assign44970_e50098 * var_temp1_dn7)), (var_xg20_op_dn8 + (assign44970_e50098 * var_temp1_dn8)), (var_xg20_op_dn9 + (assign44970_e50098 * var_temp1_dn9)),)
    } else {
        (var_xg2eff_op, var_xg2eff_op_dn4, var_xg2eff_op_dn6, var_xg2eff_op_dn7, var_xg2eff_op_dn8, var_xg2eff_op_dn9,)
    }
};
        var_xg2eff_op = assign44970_e50103;
        var_xg2eff_op_dn4 = assign44970_e50103_d_n4;
        var_xg2eff_op_dn6 = assign44970_e50103_d_n6;
        var_xg2eff_op_dn7 = assign44970_e50103_d_n7;
        var_xg2eff_op_dn8 = assign44970_e50103_d_n8;
        var_xg2eff_op_dn9 = assign44970_e50103_d_n9;

        let (assign44980_e50110, assign44980_e50110_d_n4, assign44980_e50110_d_n6, assign44980_e50110_d_n7, assign44980_e50110_d_n8, assign44980_e50110_d_n9,) = {
    if ((var_guard1360 != 0.0) && (var_guard1361 == 0.0)) {
        (var_xg20_op, var_xg20_op_dn4, var_xg20_op_dn6, var_xg20_op_dn7, var_xg20_op_dn8, var_xg20_op_dn9,)
    } else {
        (var_xg2eff_op, var_xg2eff_op_dn4, var_xg2eff_op_dn6, var_xg2eff_op_dn7, var_xg2eff_op_dn8, var_xg2eff_op_dn9,)
    }
};
        var_xg2eff_op = assign44980_e50110;
        var_xg2eff_op_dn4 = assign44980_e50110_d_n4;
        var_xg2eff_op_dn6 = assign44980_e50110_d_n6;
        var_xg2eff_op_dn7 = assign44980_e50110_d_n7;
        var_xg2eff_op_dn8 = assign44980_e50110_d_n8;
        var_xg2eff_op_dn9 = assign44980_e50110_d_n9;

        let (assign44990_e50118, assign44990_e50118_d_n4, assign44990_e50118_d_n6, assign44990_e50118_d_n7, assign44990_e50118_d_n8, assign44990_e50118_d_n9,) = {
    if (var_guard1360 != 0.0) {
        let assign44990_e50115: f64 = (var_xg10_op - var_xg2eff_op);
        let assign44990_e50116: f64 = (var_keq_1d * assign44990_e50115);
        (assign44990_e50116, (var_keq_1d * (var_xg10_op_dn4 - var_xg2eff_op_dn4)), (var_keq_1d * (var_xg10_op_dn6 - var_xg2eff_op_dn6)), (var_keq_1d * (var_xg10_op_dn7 - var_xg2eff_op_dn7)), (var_keq_1d * (var_xg10_op_dn8 - var_xg2eff_op_dn8)), (var_keq_1d * (var_xg10_op_dn9 - var_xg2eff_op_dn9)),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign44990_e50118;
        var_temp_dn4 = assign44990_e50118_d_n4;
        var_temp_dn6 = assign44990_e50118_d_n6;
        var_temp_dn7 = assign44990_e50118_d_n7;
        var_temp_dn8 = assign44990_e50118_d_n8;
        var_temp_dn9 = assign44990_e50118_d_n9;

        let assign45000_e50121: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        var_guard1363 = assign45000_e50121;

        let (assign45010_e50144, assign45010_e50144_d_n4, assign45010_e50144_d_n6, assign45010_e50144_d_n7, assign45010_e50144_d_n8, assign45010_e50144_d_n9,) = {
    if ((var_guard1360 != 0.0) && (var_guard1363 != 0.0)) {
        let assign45010_e50128: f64 = (var_temp + var_emin);
        let assign45010_e50131: f64 = (var_temp - var_emin);
        let assign45010_e50134: f64 = (var_temp - var_emin);
        let assign45010_e50135: f64 = (assign45010_e50131 * assign45010_e50134);
        let assign45010_e50138: f64 = (var_emin * var_emin);
        let assign45010_e50139: f64 = (assign45010_e50135 + assign45010_e50138);
        let assign45010_e50140: f64 = (assign45010_e50139).sqrt();
        let assign45010_e50141: f64 = (assign45010_e50128 + assign45010_e50140);
        let assign45010_e50142: f64 = (0.5 * assign45010_e50141);
        (assign45010_e50142, (0.5 * ((var_temp_dn4 + var_emin_dn4) + (((((var_temp_dn4 - var_emin_dn4) * assign45010_e50134) + (assign45010_e50131 * (var_temp_dn4 - var_emin_dn4))) + ((var_emin_dn4 * var_emin) + (var_emin * var_emin_dn4))) / (2.0 * assign45010_e50140)))), (0.5 * ((var_temp_dn6 + var_emin_dn6) + (((((var_temp_dn6 - var_emin_dn6) * assign45010_e50134) + (assign45010_e50131 * (var_temp_dn6 - var_emin_dn6))) + ((var_emin_dn6 * var_emin) + (var_emin * var_emin_dn6))) / (2.0 * assign45010_e50140)))), (0.5 * ((var_temp_dn7 + var_emin_dn7) + (((((var_temp_dn7 - var_emin_dn7) * assign45010_e50134) + (assign45010_e50131 * (var_temp_dn7 - var_emin_dn7))) + ((var_emin_dn7 * var_emin) + (var_emin * var_emin_dn7))) / (2.0 * assign45010_e50140)))), (0.5 * ((var_temp_dn8 + var_emin_dn8) + (((((var_temp_dn8 - var_emin_dn8) * assign45010_e50134) + (assign45010_e50131 * (var_temp_dn8 - var_emin_dn8))) + ((var_emin_dn8 * var_emin) + (var_emin * var_emin_dn8))) / (2.0 * assign45010_e50140)))), (0.5 * ((var_temp_dn9 + var_emin_dn9) + (((((var_temp_dn9 - var_emin_dn9) * assign45010_e50134) + (assign45010_e50131 * (var_temp_dn9 - var_emin_dn9))) + ((var_emin_dn9 * var_emin) + (var_emin * var_emin_dn9))) / (2.0 * assign45010_e50140)))),)
    } else {
        (var_e1_op, var_e1_op_dn4, var_e1_op_dn6, var_e1_op_dn7, var_e1_op_dn8, var_e1_op_dn9,)
    }
};
        var_e1_op = assign45010_e50144;
        var_e1_op_dn4 = assign45010_e50144_d_n4;
        var_e1_op_dn6 = assign45010_e50144_d_n6;
        var_e1_op_dn7 = assign45010_e50144_d_n7;
        var_e1_op_dn8 = assign45010_e50144_d_n8;
        var_e1_op_dn9 = assign45010_e50144_d_n9;

        let (assign45020_e50170, assign45020_e50170_d_n4, assign45020_e50170_d_n6, assign45020_e50170_d_n7, assign45020_e50170_d_n8, assign45020_e50170_d_n9,) = {
    if ((var_guard1360 != 0.0) && (var_guard1363 != 0.0)) {
        let assign45020_e50150: f64 = (-var_temp);
        let assign45020_e50152: f64 = (assign45020_e50150 + var_emin);
        let assign45020_e50154: f64 = (-var_temp);
        let assign45020_e50156: f64 = (assign45020_e50154 - var_emin);
        let assign45020_e50158: f64 = (-var_temp);
        let assign45020_e50160: f64 = (assign45020_e50158 - var_emin);
        let assign45020_e50161: f64 = (assign45020_e50156 * assign45020_e50160);
        let assign45020_e50164: f64 = (var_emin * var_emin);
        let assign45020_e50165: f64 = (assign45020_e50161 + assign45020_e50164);
        let assign45020_e50166: f64 = (assign45020_e50165).sqrt();
        let assign45020_e50167: f64 = (assign45020_e50152 + assign45020_e50166);
        let assign45020_e50168: f64 = (0.5 * assign45020_e50167);
        (assign45020_e50168, (0.5 * (((-var_temp_dn4) + var_emin_dn4) + ((((((-var_temp_dn4) - var_emin_dn4) * assign45020_e50160) + (assign45020_e50156 * ((-var_temp_dn4) - var_emin_dn4))) + ((var_emin_dn4 * var_emin) + (var_emin * var_emin_dn4))) / (2.0 * assign45020_e50166)))), (0.5 * (((-var_temp_dn6) + var_emin_dn6) + ((((((-var_temp_dn6) - var_emin_dn6) * assign45020_e50160) + (assign45020_e50156 * ((-var_temp_dn6) - var_emin_dn6))) + ((var_emin_dn6 * var_emin) + (var_emin * var_emin_dn6))) / (2.0 * assign45020_e50166)))), (0.5 * (((-var_temp_dn7) + var_emin_dn7) + ((((((-var_temp_dn7) - var_emin_dn7) * assign45020_e50160) + (assign45020_e50156 * ((-var_temp_dn7) - var_emin_dn7))) + ((var_emin_dn7 * var_emin) + (var_emin * var_emin_dn7))) / (2.0 * assign45020_e50166)))), (0.5 * (((-var_temp_dn8) + var_emin_dn8) + ((((((-var_temp_dn8) - var_emin_dn8) * assign45020_e50160) + (assign45020_e50156 * ((-var_temp_dn8) - var_emin_dn8))) + ((var_emin_dn8 * var_emin) + (var_emin * var_emin_dn8))) / (2.0 * assign45020_e50166)))), (0.5 * (((-var_temp_dn9) + var_emin_dn9) + ((((((-var_temp_dn9) - var_emin_dn9) * assign45020_e50160) + (assign45020_e50156 * ((-var_temp_dn9) - var_emin_dn9))) + ((var_emin_dn9 * var_emin) + (var_emin * var_emin_dn9))) / (2.0 * assign45020_e50166)))),)
    } else {
        (var_e2_op, var_e2_op_dn4, var_e2_op_dn6, var_e2_op_dn7, var_e2_op_dn8, var_e2_op_dn9,)
    }
};
        var_e2_op = assign45020_e50170;
        var_e2_op_dn4 = assign45020_e50170_d_n4;
        var_e2_op_dn6 = assign45020_e50170_d_n6;
        var_e2_op_dn7 = assign45020_e50170_d_n7;
        var_e2_op_dn8 = assign45020_e50170_d_n8;
        var_e2_op_dn9 = assign45020_e50170_d_n9;

        let (assign45030_e50183, assign45030_e50183_d_n4, assign45030_e50183_d_n6, assign45030_e50183_d_n7, assign45030_e50183_d_n8, assign45030_e50183_d_n9,) = {
    if ((var_guard1360 != 0.0) && (var_guard1363 != 0.0)) {
        let assign45030_e50176: f64 = (-0.3333333333333);
        let assign45030_e50178: f64 = (var_e1_op).ln();
        let assign45030_e50179: f64 = (assign45030_e50176 * assign45030_e50178);
        let assign45030_e50180: f64 = (assign45030_e50179).exp();
        let assign45030_e50181: f64 = (var_qq_op * assign45030_e50180);
        (assign45030_e50181, ((var_qq_op_dn4 * assign45030_e50180) + (var_qq_op * (assign45030_e50180 * (assign45030_e50176 * (var_e1_op_dn4 / var_e1_op))))), ((var_qq_op_dn6 * assign45030_e50180) + (var_qq_op * (assign45030_e50180 * (assign45030_e50176 * (var_e1_op_dn6 / var_e1_op))))), ((var_qq_op_dn7 * assign45030_e50180) + (var_qq_op * (assign45030_e50180 * (assign45030_e50176 * (var_e1_op_dn7 / var_e1_op))))), ((var_qq_op_dn8 * assign45030_e50180) + (var_qq_op * (assign45030_e50180 * (assign45030_e50176 * (var_e1_op_dn8 / var_e1_op))))), ((var_qq_op_dn9 * assign45030_e50180) + (var_qq_op * (assign45030_e50180 * (assign45030_e50176 * (var_e1_op_dn9 / var_e1_op))))),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign45030_e50183;
        var_temp1_dn4 = assign45030_e50183_d_n4;
        var_temp1_dn6 = assign45030_e50183_d_n6;
        var_temp1_dn7 = assign45030_e50183_d_n7;
        var_temp1_dn8 = assign45030_e50183_d_n8;
        var_temp1_dn9 = assign45030_e50183_d_n9;

        let (assign45040_e50196, assign45040_e50196_d_n4, assign45040_e50196_d_n6, assign45040_e50196_d_n7, assign45040_e50196_d_n8, assign45040_e50196_d_n9,) = {
    if ((var_guard1360 != 0.0) && (var_guard1363 != 0.0)) {
        let assign45040_e50189: f64 = (-0.3333333333333);
        let assign45040_e50191: f64 = (var_e2_op).ln();
        let assign45040_e50192: f64 = (assign45040_e50189 * assign45040_e50191);
        let assign45040_e50193: f64 = (assign45040_e50192).exp();
        let assign45040_e50194: f64 = (var_qq_op * assign45040_e50193);
        (assign45040_e50194, ((var_qq_op_dn4 * assign45040_e50193) + (var_qq_op * (assign45040_e50193 * (assign45040_e50189 * (var_e2_op_dn4 / var_e2_op))))), ((var_qq_op_dn6 * assign45040_e50193) + (var_qq_op * (assign45040_e50193 * (assign45040_e50189 * (var_e2_op_dn6 / var_e2_op))))), ((var_qq_op_dn7 * assign45040_e50193) + (var_qq_op * (assign45040_e50193 * (assign45040_e50189 * (var_e2_op_dn7 / var_e2_op))))), ((var_qq_op_dn8 * assign45040_e50193) + (var_qq_op * (assign45040_e50193 * (assign45040_e50189 * (var_e2_op_dn8 / var_e2_op))))), ((var_qq_op_dn9 * assign45040_e50193) + (var_qq_op * (assign45040_e50193 * (assign45040_e50189 * (var_e2_op_dn9 / var_e2_op))))),)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign45040_e50196;
        var_temp2_dn4 = assign45040_e50196_d_n4;
        var_temp2_dn6 = assign45040_e50196_d_n6;
        var_temp2_dn7 = assign45040_e50196_d_n7;
        var_temp2_dn8 = assign45040_e50196_d_n8;
        var_temp2_dn9 = assign45040_e50196_d_n9;

        let (assign45050_e50206, assign45050_e50206_d_n4, assign45050_e50206_d_n6, assign45050_e50206_d_n7, assign45050_e50206_d_n8, assign45050_e50206_d_n9,) = {
    if ((var_guard1360 != 0.0) && (var_guard1363 != 0.0)) {
        let assign45050_e50202: f64 = (1.0 - var_temp1);
        let assign45050_e50204: f64 = (assign45050_e50202 - var_temp2);
        (assign45050_e50204, ((-var_temp1_dn4) - var_temp2_dn4), ((-var_temp1_dn6) - var_temp2_dn6), ((-var_temp1_dn7) - var_temp2_dn7), ((-var_temp1_dn8) - var_temp2_dn8), ((-var_temp1_dn9) - var_temp2_dn9),)
    } else {
        (var_temp3, var_temp3_dn4, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9,)
    }
};
        var_temp3 = assign45050_e50206;
        var_temp3_dn4 = assign45050_e50206_d_n4;
        var_temp3_dn6 = assign45050_e50206_d_n6;
        var_temp3_dn7 = assign45050_e50206_d_n7;
        var_temp3_dn8 = assign45050_e50206_d_n8;
        var_temp3_dn9 = assign45050_e50206_d_n9;

        let (assign45070_e50228, assign45070_e50228_d_n4, assign45070_e50228_d_n6, assign45070_e50228_d_n7, assign45070_e50228_d_n8, assign45070_e50228_d_n9,) = {
    if ((var_guard1360 != 0.0) && (var_guard1363 != 0.0)) {
        let assign45070_e50220: f64 = (var_k1_1d * var_temp3);
        let assign45070_e50224: f64 = (var_k1_1d * var_temp1);
        let assign45070_e50225: f64 = (1.0 + assign45070_e50224);
        let assign45070_e50226: f64 = (assign45070_e50220 / assign45070_e50225);
        (assign45070_e50226, ((((var_k1_1d * var_temp3_dn4) * assign45070_e50225) - (assign45070_e50220 * (var_k1_1d * var_temp1_dn4))) / (assign45070_e50225 * assign45070_e50225)), ((((var_k1_1d * var_temp3_dn6) * assign45070_e50225) - (assign45070_e50220 * (var_k1_1d * var_temp1_dn6))) / (assign45070_e50225 * assign45070_e50225)), ((((var_k1_1d * var_temp3_dn7) * assign45070_e50225) - (assign45070_e50220 * (var_k1_1d * var_temp1_dn7))) / (assign45070_e50225 * assign45070_e50225)), ((((var_k1_1d * var_temp3_dn8) * assign45070_e50225) - (assign45070_e50220 * (var_k1_1d * var_temp1_dn8))) / (assign45070_e50225 * assign45070_e50225)), ((((var_k1_1d * var_temp3_dn9) * assign45070_e50225) - (assign45070_e50220 * (var_k1_1d * var_temp1_dn9))) / (assign45070_e50225 * assign45070_e50225)),)
    } else {
        (var_k1_1d_qm_op, var_k1_1d_qm_op_dn4, var_k1_1d_qm_op_dn6, var_k1_1d_qm_op_dn7, var_k1_1d_qm_op_dn8, var_k1_1d_qm_op_dn9,)
    }
};
        var_k1_1d_qm_op = assign45070_e50228;
        var_k1_1d_qm_op_dn4 = assign45070_e50228_d_n4;
        var_k1_1d_qm_op_dn6 = assign45070_e50228_d_n6;
        var_k1_1d_qm_op_dn7 = assign45070_e50228_d_n7;
        var_k1_1d_qm_op_dn8 = assign45070_e50228_d_n8;
        var_k1_1d_qm_op_dn9 = assign45070_e50228_d_n9;

        let (assign45080_e50242, assign45080_e50242_d_n4, assign45080_e50242_d_n6, assign45080_e50242_d_n7, assign45080_e50242_d_n8, assign45080_e50242_d_n9,) = {
    if ((var_guard1360 != 0.0) && (var_guard1363 != 0.0)) {
        let assign45080_e50234: f64 = (var_k2_1d * var_temp3);
        let assign45080_e50238: f64 = (var_k2_1d * var_temp2);
        let assign45080_e50239: f64 = (1.0 + assign45080_e50238);
        let assign45080_e50240: f64 = (assign45080_e50234 / assign45080_e50239);
        (assign45080_e50240, ((((var_k2_1d * var_temp3_dn4) * assign45080_e50239) - (assign45080_e50234 * (var_k2_1d * var_temp2_dn4))) / (assign45080_e50239 * assign45080_e50239)), ((((var_k2_1d * var_temp3_dn6) * assign45080_e50239) - (assign45080_e50234 * (var_k2_1d * var_temp2_dn6))) / (assign45080_e50239 * assign45080_e50239)), ((((var_k2_1d * var_temp3_dn7) * assign45080_e50239) - (assign45080_e50234 * (var_k2_1d * var_temp2_dn7))) / (assign45080_e50239 * assign45080_e50239)), ((((var_k2_1d * var_temp3_dn8) * assign45080_e50239) - (assign45080_e50234 * (var_k2_1d * var_temp2_dn8))) / (assign45080_e50239 * assign45080_e50239)), ((((var_k2_1d * var_temp3_dn9) * assign45080_e50239) - (assign45080_e50234 * (var_k2_1d * var_temp2_dn9))) / (assign45080_e50239 * assign45080_e50239)),)
    } else {
        (var_k2_1d_qm_op, var_k2_1d_qm_op_dn4, var_k2_1d_qm_op_dn6, var_k2_1d_qm_op_dn7, var_k2_1d_qm_op_dn8, var_k2_1d_qm_op_dn9,)
    }
};
        var_k2_1d_qm_op = assign45080_e50242;
        var_k2_1d_qm_op_dn4 = assign45080_e50242_d_n4;
        var_k2_1d_qm_op_dn6 = assign45080_e50242_d_n6;
        var_k2_1d_qm_op_dn7 = assign45080_e50242_d_n7;
        var_k2_1d_qm_op_dn8 = assign45080_e50242_d_n8;
        var_k2_1d_qm_op_dn9 = assign45080_e50242_d_n9;

        let (assign45090_e50258, assign45090_e50258_d_n4, assign45090_e50258_d_n6, assign45090_e50258_d_n7, assign45090_e50258_d_n8, assign45090_e50258_d_n9,) = {
    if ((var_guard1360 != 0.0) && (var_guard1363 != 0.0)) {
        let assign45090_e50250: f64 = (1.0 / var_k1_1d_qm_op);
        let assign45090_e50251: f64 = (1.0 + assign45090_e50250);
        let assign45090_e50254: f64 = (1.0 / var_k2_1d_qm_op);
        let assign45090_e50255: f64 = (assign45090_e50251 + assign45090_e50254);
        let assign45090_e50256: f64 = (1.0 / assign45090_e50255);
        (assign45090_e50256, (-(((-(var_k1_1d_qm_op_dn4 / (var_k1_1d_qm_op * var_k1_1d_qm_op))) + (-(var_k2_1d_qm_op_dn4 / (var_k2_1d_qm_op * var_k2_1d_qm_op)))) / (assign45090_e50255 * assign45090_e50255))), (-(((-(var_k1_1d_qm_op_dn6 / (var_k1_1d_qm_op * var_k1_1d_qm_op))) + (-(var_k2_1d_qm_op_dn6 / (var_k2_1d_qm_op * var_k2_1d_qm_op)))) / (assign45090_e50255 * assign45090_e50255))), (-(((-(var_k1_1d_qm_op_dn7 / (var_k1_1d_qm_op * var_k1_1d_qm_op))) + (-(var_k2_1d_qm_op_dn7 / (var_k2_1d_qm_op * var_k2_1d_qm_op)))) / (assign45090_e50255 * assign45090_e50255))), (-(((-(var_k1_1d_qm_op_dn8 / (var_k1_1d_qm_op * var_k1_1d_qm_op))) + (-(var_k2_1d_qm_op_dn8 / (var_k2_1d_qm_op * var_k2_1d_qm_op)))) / (assign45090_e50255 * assign45090_e50255))), (-(((-(var_k1_1d_qm_op_dn9 / (var_k1_1d_qm_op * var_k1_1d_qm_op))) + (-(var_k2_1d_qm_op_dn9 / (var_k2_1d_qm_op * var_k2_1d_qm_op)))) / (assign45090_e50255 * assign45090_e50255))),)
    } else {
        (var_keq_1d_qm_op, var_keq_1d_qm_op_dn4, var_keq_1d_qm_op_dn6, var_keq_1d_qm_op_dn7, var_keq_1d_qm_op_dn8, var_keq_1d_qm_op_dn9,)
    }
};
        var_keq_1d_qm_op = assign45090_e50258;
        var_keq_1d_qm_op_dn4 = assign45090_e50258_d_n4;
        var_keq_1d_qm_op_dn6 = assign45090_e50258_d_n6;
        var_keq_1d_qm_op_dn7 = assign45090_e50258_d_n7;
        var_keq_1d_qm_op_dn8 = assign45090_e50258_d_n8;
        var_keq_1d_qm_op_dn9 = assign45090_e50258_d_n9;

        let (assign45110_e50272, assign45110_e50272_d_n4, assign45110_e50272_d_n6, assign45110_e50272_d_n7, assign45110_e50272_d_n8, assign45110_e50272_d_n9,) = {
    if ((var_guard1360 != 0.0) && (var_guard1363 == 0.0)) {
        (var_k1_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_k1_1d_qm_op, var_k1_1d_qm_op_dn4, var_k1_1d_qm_op_dn6, var_k1_1d_qm_op_dn7, var_k1_1d_qm_op_dn8, var_k1_1d_qm_op_dn9,)
    }
};
        var_k1_1d_qm_op = assign45110_e50272;
        var_k1_1d_qm_op_dn4 = assign45110_e50272_d_n4;
        var_k1_1d_qm_op_dn6 = assign45110_e50272_d_n6;
        var_k1_1d_qm_op_dn7 = assign45110_e50272_d_n7;
        var_k1_1d_qm_op_dn8 = assign45110_e50272_d_n8;
        var_k1_1d_qm_op_dn9 = assign45110_e50272_d_n9;

        let (assign45120_e50279, assign45120_e50279_d_n4, assign45120_e50279_d_n6, assign45120_e50279_d_n7, assign45120_e50279_d_n8, assign45120_e50279_d_n9,) = {
    if ((var_guard1360 != 0.0) && (var_guard1363 == 0.0)) {
        (var_k2_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_k2_1d_qm_op, var_k2_1d_qm_op_dn4, var_k2_1d_qm_op_dn6, var_k2_1d_qm_op_dn7, var_k2_1d_qm_op_dn8, var_k2_1d_qm_op_dn9,)
    }
};
        var_k2_1d_qm_op = assign45120_e50279;
        var_k2_1d_qm_op_dn4 = assign45120_e50279_d_n4;
        var_k2_1d_qm_op_dn6 = assign45120_e50279_d_n6;
        var_k2_1d_qm_op_dn7 = assign45120_e50279_d_n7;
        var_k2_1d_qm_op_dn8 = assign45120_e50279_d_n8;
        var_k2_1d_qm_op_dn9 = assign45120_e50279_d_n9;

        let (assign45130_e50286, assign45130_e50286_d_n4, assign45130_e50286_d_n6, assign45130_e50286_d_n7, assign45130_e50286_d_n8, assign45130_e50286_d_n9,) = {
    if ((var_guard1360 != 0.0) && (var_guard1363 == 0.0)) {
        (var_keq_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_keq_1d_qm_op, var_keq_1d_qm_op_dn4, var_keq_1d_qm_op_dn6, var_keq_1d_qm_op_dn7, var_keq_1d_qm_op_dn8, var_keq_1d_qm_op_dn9,)
    }
};
        var_keq_1d_qm_op = assign45130_e50286;
        var_keq_1d_qm_op_dn4 = assign45130_e50286_d_n4;
        var_keq_1d_qm_op_dn6 = assign45130_e50286_d_n6;
        var_keq_1d_qm_op_dn7 = assign45130_e50286_d_n7;
        var_keq_1d_qm_op_dn8 = assign45130_e50286_d_n8;
        var_keq_1d_qm_op_dn9 = assign45130_e50286_d_n9;

        let (assign45140_e50294, assign45140_e50294_d_n4, assign45140_e50294_d_n6, assign45140_e50294_d_n7, assign45140_e50294_d_n8, assign45140_e50294_d_n9,) = {
    if (var_guard1360 != 0.0) {
        let assign45140_e50291: f64 = (var_xg10_op - var_xg2eff_op);
        let assign45140_e50292: f64 = (var_keq_1d_qm_op * assign45140_e50291);
        (assign45140_e50292, ((var_keq_1d_qm_op_dn4 * assign45140_e50291) + (var_keq_1d_qm_op * (var_xg10_op_dn4 - var_xg2eff_op_dn4))), ((var_keq_1d_qm_op_dn6 * assign45140_e50291) + (var_keq_1d_qm_op * (var_xg10_op_dn6 - var_xg2eff_op_dn6))), ((var_keq_1d_qm_op_dn7 * assign45140_e50291) + (var_keq_1d_qm_op * (var_xg10_op_dn7 - var_xg2eff_op_dn7))), ((var_keq_1d_qm_op_dn8 * assign45140_e50291) + (var_keq_1d_qm_op * (var_xg10_op_dn8 - var_xg2eff_op_dn8))), ((var_keq_1d_qm_op_dn9 * assign45140_e50291) + (var_keq_1d_qm_op * (var_xg10_op_dn9 - var_xg2eff_op_dn9))),)
    } else {
        (var_dx_wi_1d_op, var_dx_wi_1d_op_dn4, var_dx_wi_1d_op_dn6, var_dx_wi_1d_op_dn7, var_dx_wi_1d_op_dn8, var_dx_wi_1d_op_dn9,)
    }
};
        var_dx_wi_1d_op = assign45140_e50294;
        var_dx_wi_1d_op_dn4 = assign45140_e50294_d_n4;
        var_dx_wi_1d_op_dn6 = assign45140_e50294_d_n6;
        var_dx_wi_1d_op_dn7 = assign45140_e50294_d_n7;
        var_dx_wi_1d_op_dn8 = assign45140_e50294_d_n8;
        var_dx_wi_1d_op_dn9 = assign45140_e50294_d_n9;

        let assign45150_e50297: f64 = if var_dx_wi_1d_op > 0.0 { 1.0 } else { 0.0 };
        var_guard1364 = assign45150_e50297;

        let assign45160_e50299: f64 = (-var_dx_wi_1d_op);
        let assign45160_e50301: f64 = if assign45160_e50299 < 80.0 { 1.0 } else { 0.0 };
        var_guard1365 = assign45160_e50301;

        let (assign45170_e50314, assign45170_e50314_d_n4, assign45170_e50314_d_n6, assign45170_e50314_d_n7, assign45170_e50314_d_n8, assign45170_e50314_d_n9,) = {
    if (((var_guard1360 != 0.0) && (var_guard1364 != 0.0)) && (var_guard1365 != 0.0)) {
        let assign45170_e50309: f64 = (-var_dx_wi_1d_op);
        let assign45170_e50310: f64 = (assign45170_e50309).exp();
        let assign45170_e50311: f64 = (1.0 + assign45170_e50310);
        let assign45170_e50312: f64 = (assign45170_e50311).ln();
        (assign45170_e50312, ((assign45170_e50310 * (-var_dx_wi_1d_op_dn4)) / assign45170_e50311), ((assign45170_e50310 * (-var_dx_wi_1d_op_dn6)) / assign45170_e50311), ((assign45170_e50310 * (-var_dx_wi_1d_op_dn7)) / assign45170_e50311), ((assign45170_e50310 * (-var_dx_wi_1d_op_dn8)) / assign45170_e50311), ((assign45170_e50310 * (-var_dx_wi_1d_op_dn9)) / assign45170_e50311),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign45170_e50314;
        var_temp_dn4 = assign45170_e50314_d_n4;
        var_temp_dn6 = assign45170_e50314_d_n6;
        var_temp_dn7 = assign45170_e50314_d_n7;
        var_temp_dn8 = assign45170_e50314_d_n8;
        var_temp_dn9 = assign45170_e50314_d_n9;

        let (assign45180_e50324, assign45180_e50324_d_n4, assign45180_e50324_d_n6, assign45180_e50324_d_n7, assign45180_e50324_d_n8, assign45180_e50324_d_n9,) = {
    if (((var_guard1360 != 0.0) && (var_guard1364 != 0.0)) && (var_guard1365 == 0.0)) {
        let assign45180_e50322: f64 = (-var_dx_wi_1d_op);
        (assign45180_e50322, (-var_dx_wi_1d_op_dn4), (-var_dx_wi_1d_op_dn6), (-var_dx_wi_1d_op_dn7), (-var_dx_wi_1d_op_dn8), (-var_dx_wi_1d_op_dn9),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign45180_e50324;
        var_temp_dn4 = assign45180_e50324_d_n4;
        var_temp_dn6 = assign45180_e50324_d_n6;
        var_temp_dn7 = assign45180_e50324_d_n7;
        var_temp_dn8 = assign45180_e50324_d_n8;
        var_temp_dn9 = assign45180_e50324_d_n9;

        let (assign45190_e50338, assign45190_e50338_d_n4, assign45190_e50338_d_n6, assign45190_e50338_d_n7, assign45190_e50338_d_n8, assign45190_e50338_d_n9,) = {
    if ((var_guard1360 != 0.0) && (var_guard1364 != 0.0)) {
        let assign45190_e50331: f64 = (var_dx_wi_1d_op / var_k1_1d_qm_op);
        let assign45190_e50332: f64 = (var_xg10_op - assign45190_e50331);
        let assign45190_e50334: f64 = (assign45190_e50332 + var_temp);
        let assign45190_e50336: f64 = (assign45190_e50334 - 0.6931471805599);
        (assign45190_e50336, ((var_xg10_op_dn4 - (((var_dx_wi_1d_op_dn4 * var_k1_1d_qm_op) - (var_dx_wi_1d_op * var_k1_1d_qm_op_dn4)) / (var_k1_1d_qm_op * var_k1_1d_qm_op))) + var_temp_dn4), ((var_xg10_op_dn6 - (((var_dx_wi_1d_op_dn6 * var_k1_1d_qm_op) - (var_dx_wi_1d_op * var_k1_1d_qm_op_dn6)) / (var_k1_1d_qm_op * var_k1_1d_qm_op))) + var_temp_dn6), ((var_xg10_op_dn7 - (((var_dx_wi_1d_op_dn7 * var_k1_1d_qm_op) - (var_dx_wi_1d_op * var_k1_1d_qm_op_dn7)) / (var_k1_1d_qm_op * var_k1_1d_qm_op))) + var_temp_dn7), ((var_xg10_op_dn8 - (((var_dx_wi_1d_op_dn8 * var_k1_1d_qm_op) - (var_dx_wi_1d_op * var_k1_1d_qm_op_dn8)) / (var_k1_1d_qm_op * var_k1_1d_qm_op))) + var_temp_dn8), ((var_xg10_op_dn9 - (((var_dx_wi_1d_op_dn9 * var_k1_1d_qm_op) - (var_dx_wi_1d_op * var_k1_1d_qm_op_dn9)) / (var_k1_1d_qm_op * var_k1_1d_qm_op))) + var_temp_dn9),)
    } else {
        (var_x_wi_1d_op, var_x_wi_1d_op_dn4, var_x_wi_1d_op_dn6, var_x_wi_1d_op_dn7, var_x_wi_1d_op_dn8, var_x_wi_1d_op_dn9,)
    }
};
        var_x_wi_1d_op = assign45190_e50338;
        var_x_wi_1d_op_dn4 = assign45190_e50338_d_n4;
        var_x_wi_1d_op_dn6 = assign45190_e50338_d_n6;
        var_x_wi_1d_op_dn7 = assign45190_e50338_d_n7;
        var_x_wi_1d_op_dn8 = assign45190_e50338_d_n8;
        var_x_wi_1d_op_dn9 = assign45190_e50338_d_n9;

        *var_dx_wi_1d_op_slot = var_dx_wi_1d_op;
        *var_dx_wi_1d_op_dn4_slot = var_dx_wi_1d_op_dn4;
        *var_dx_wi_1d_op_dn6_slot = var_dx_wi_1d_op_dn6;
        *var_dx_wi_1d_op_dn7_slot = var_dx_wi_1d_op_dn7;
        *var_dx_wi_1d_op_dn8_slot = var_dx_wi_1d_op_dn8;
        *var_dx_wi_1d_op_dn9_slot = var_dx_wi_1d_op_dn9;
        *var_e1_op_slot = var_e1_op;
        *var_e1_op_dn4_slot = var_e1_op_dn4;
        *var_e1_op_dn6_slot = var_e1_op_dn6;
        *var_e1_op_dn7_slot = var_e1_op_dn7;
        *var_e1_op_dn8_slot = var_e1_op_dn8;
        *var_e1_op_dn9_slot = var_e1_op_dn9;
        *var_e2_op_slot = var_e2_op;
        *var_e2_op_dn4_slot = var_e2_op_dn4;
        *var_e2_op_dn6_slot = var_e2_op_dn6;
        *var_e2_op_dn7_slot = var_e2_op_dn7;
        *var_e2_op_dn8_slot = var_e2_op_dn8;
        *var_e2_op_dn9_slot = var_e2_op_dn9;
        *var_guard1361_slot = var_guard1361;
        *var_guard1362_slot = var_guard1362;
        *var_guard1363_slot = var_guard1363;
        *var_guard1364_slot = var_guard1364;
        *var_guard1365_slot = var_guard1365;
        *var_k1_1d_qm_op_slot = var_k1_1d_qm_op;
        *var_k1_1d_qm_op_dn4_slot = var_k1_1d_qm_op_dn4;
        *var_k1_1d_qm_op_dn6_slot = var_k1_1d_qm_op_dn6;
        *var_k1_1d_qm_op_dn7_slot = var_k1_1d_qm_op_dn7;
        *var_k1_1d_qm_op_dn8_slot = var_k1_1d_qm_op_dn8;
        *var_k1_1d_qm_op_dn9_slot = var_k1_1d_qm_op_dn9;
        *var_k2_1d_qm_op_slot = var_k2_1d_qm_op;
        *var_k2_1d_qm_op_dn4_slot = var_k2_1d_qm_op_dn4;
        *var_k2_1d_qm_op_dn6_slot = var_k2_1d_qm_op_dn6;
        *var_k2_1d_qm_op_dn7_slot = var_k2_1d_qm_op_dn7;
        *var_k2_1d_qm_op_dn8_slot = var_k2_1d_qm_op_dn8;
        *var_k2_1d_qm_op_dn9_slot = var_k2_1d_qm_op_dn9;
        *var_keq_1d_qm_op_slot = var_keq_1d_qm_op;
        *var_keq_1d_qm_op_dn4_slot = var_keq_1d_qm_op_dn4;
        *var_keq_1d_qm_op_dn6_slot = var_keq_1d_qm_op_dn6;
        *var_keq_1d_qm_op_dn7_slot = var_keq_1d_qm_op_dn7;
        *var_keq_1d_qm_op_dn8_slot = var_keq_1d_qm_op_dn8;
        *var_keq_1d_qm_op_dn9_slot = var_keq_1d_qm_op_dn9;
        *var_temp_slot = var_temp;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp3_slot = var_temp3;
        *var_temp3_dn4_slot = var_temp3_dn4;
        *var_temp3_dn6_slot = var_temp3_dn6;
        *var_temp3_dn7_slot = var_temp3_dn7;
        *var_temp3_dn8_slot = var_temp3_dn8;
        *var_temp3_dn9_slot = var_temp3_dn9;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_vfb1_op_slot = var_vfb1_op;
        *var_vfb1_op_dn4_slot = var_vfb1_op_dn4;
        *var_vfb1_op_dn6_slot = var_vfb1_op_dn6;
        *var_vfb1_op_dn7_slot = var_vfb1_op_dn7;
        *var_vfb1_op_dn8_slot = var_vfb1_op_dn8;
        *var_vfb1_op_dn9_slot = var_vfb1_op_dn9;
        *var_vfb2_op_slot = var_vfb2_op;
        *var_vfb2_op_dn4_slot = var_vfb2_op_dn4;
        *var_vfb2_op_dn6_slot = var_vfb2_op_dn6;
        *var_vfb2_op_dn7_slot = var_vfb2_op_dn7;
        *var_vfb2_op_dn8_slot = var_vfb2_op_dn8;
        *var_vfb2_op_dn9_slot = var_vfb2_op_dn9;
        *var_vthinit_op_slot = var_vthinit_op;
        *var_vthinit_op_dn4_slot = var_vthinit_op_dn4;
        *var_vthinit_op_dn6_slot = var_vthinit_op_dn6;
        *var_vthinit_op_dn7_slot = var_vthinit_op_dn7;
        *var_vthinit_op_dn8_slot = var_vthinit_op_dn8;
        *var_vthinit_op_dn9_slot = var_vthinit_op_dn9;
        *var_x_wi_1d_op_slot = var_x_wi_1d_op;
        *var_x_wi_1d_op_dn4_slot = var_x_wi_1d_op_dn4;
        *var_x_wi_1d_op_dn6_slot = var_x_wi_1d_op_dn6;
        *var_x_wi_1d_op_dn7_slot = var_x_wi_1d_op_dn7;
        *var_x_wi_1d_op_dn8_slot = var_x_wi_1d_op_dn8;
        *var_x_wi_1d_op_dn9_slot = var_x_wi_1d_op_dn9;
        *var_xg10_op_slot = var_xg10_op;
        *var_xg10_op_dn4_slot = var_xg10_op_dn4;
        *var_xg10_op_dn6_slot = var_xg10_op_dn6;
        *var_xg10_op_dn7_slot = var_xg10_op_dn7;
        *var_xg10_op_dn8_slot = var_xg10_op_dn8;
        *var_xg10_op_dn9_slot = var_xg10_op_dn9;
        *var_xg20_op_slot = var_xg20_op;
        *var_xg20_op_dn4_slot = var_xg20_op_dn4;
        *var_xg20_op_dn6_slot = var_xg20_op_dn6;
        *var_xg20_op_dn7_slot = var_xg20_op_dn7;
        *var_xg20_op_dn8_slot = var_xg20_op_dn8;
        *var_xg20_op_dn9_slot = var_xg20_op_dn9;
        *var_xg2eff_op_slot = var_xg2eff_op;
        *var_xg2eff_op_dn4_slot = var_xg2eff_op_dn4;
        *var_xg2eff_op_dn6_slot = var_xg2eff_op_dn6;
        *var_xg2eff_op_dn7_slot = var_xg2eff_op_dn7;
        *var_xg2eff_op_dn8_slot = var_xg2eff_op_dn8;
        *var_xg2eff_op_dn9_slot = var_xg2eff_op_dn9;
    }

    pub(super) fn stamp_transient_block_124(
        var_cfdl_i: f64,
        var_cfdlb_i: f64,
        var_dx_wi_1d_op: f64,
        var_dx_wi_1d_op_dn4: f64,
        var_dx_wi_1d_op_dn6: f64,
        var_dx_wi_1d_op_dn7: f64,
        var_dx_wi_1d_op_dn8: f64,
        var_dx_wi_1d_op_dn9: f64,
        var_guard1360: f64,
        var_guard1364: f64,
        var_k2_1d_qm_op: f64,
        var_k2_1d_qm_op_dn4: f64,
        var_k2_1d_qm_op_dn6: f64,
        var_k2_1d_qm_op_dn7: f64,
        var_k2_1d_qm_op_dn8: f64,
        var_k2_1d_qm_op_dn9: f64,
        var_pscedlb_i: f64,
        var_xd0_op: f64,
        var_xd0_op_dn4: f64,
        var_xd0_op_dn6: f64,
        var_xd0_op_dn7: f64,
        var_xd0_op_dn8: f64,
        var_xd0_op_dn9: f64,
        var_xdsx_op: f64,
        var_xdsx_op_dn4: f64,
        var_xdsx_op_dn6: f64,
        var_xdsx_op_dn7: f64,
        var_xdsx_op_dn8: f64,
        var_xdsx_op_dn9: f64,
        var_xg20_op: f64,
        var_xg20_op_dn4: f64,
        var_xg20_op_dn6: f64,
        var_xg20_op_dn7: f64,
        var_xg20_op_dn8: f64,
        var_xg20_op_dn9: f64,
        var_xg2eff_op: f64,
        var_xg2eff_op_dn4: f64,
        var_xg2eff_op_dn6: f64,
        var_xg2eff_op_dn7: f64,
        var_xg2eff_op_dn8: f64,
        var_xg2eff_op_dn9: f64,
        var_xsddep_op: f64,
        var_xsddep_op_dn4: f64,
        var_xsddep_op_dn6: f64,
        var_xsddep_op_dn7: f64,
        var_xsddep_op_dn8: f64,
        var_xsddep_op_dn9: f64,
        var_xth_1d_op: f64,
        var_xth_1d_op_dn4: f64,
        var_xth_1d_op_dn6: f64,
        var_xth_1d_op_dn7: f64,
        var_xth_1d_op_dn8: f64,
        var_xth_1d_op_dn9: f64,
        var_dleff_op_slot: &mut f64,
        var_dleff_op_dn4_slot: &mut f64,
        var_dleff_op_dn6_slot: &mut f64,
        var_dleff_op_dn7_slot: &mut f64,
        var_dleff_op_dn8_slot: &mut f64,
        var_dleff_op_dn9_slot: &mut f64,
        var_guard1366_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_x_1d_op_slot: &mut f64,
        var_x_1d_op_dn4_slot: &mut f64,
        var_x_1d_op_dn6_slot: &mut f64,
        var_x_1d_op_dn7_slot: &mut f64,
        var_x_1d_op_dn8_slot: &mut f64,
        var_x_1d_op_dn9_slot: &mut f64,
        var_x_wi_1d_op_slot: &mut f64,
        var_x_wi_1d_op_dn4_slot: &mut f64,
        var_x_wi_1d_op_dn6_slot: &mut f64,
        var_x_wi_1d_op_dn7_slot: &mut f64,
        var_x_wi_1d_op_dn8_slot: &mut f64,
        var_x_wi_1d_op_dn9_slot: &mut f64,
    ) {
        let mut var_dleff_op: f64 = *var_dleff_op_slot;
        let mut var_dleff_op_dn4: f64 = *var_dleff_op_dn4_slot;
        let mut var_dleff_op_dn6: f64 = *var_dleff_op_dn6_slot;
        let mut var_dleff_op_dn7: f64 = *var_dleff_op_dn7_slot;
        let mut var_dleff_op_dn8: f64 = *var_dleff_op_dn8_slot;
        let mut var_dleff_op_dn9: f64 = *var_dleff_op_dn9_slot;
        let mut var_guard1366: f64 = *var_guard1366_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_x_1d_op: f64 = *var_x_1d_op_slot;
        let mut var_x_1d_op_dn4: f64 = *var_x_1d_op_dn4_slot;
        let mut var_x_1d_op_dn6: f64 = *var_x_1d_op_dn6_slot;
        let mut var_x_1d_op_dn7: f64 = *var_x_1d_op_dn7_slot;
        let mut var_x_1d_op_dn8: f64 = *var_x_1d_op_dn8_slot;
        let mut var_x_1d_op_dn9: f64 = *var_x_1d_op_dn9_slot;
        let mut var_x_wi_1d_op: f64 = *var_x_wi_1d_op_slot;
        let mut var_x_wi_1d_op_dn4: f64 = *var_x_wi_1d_op_dn4_slot;
        let mut var_x_wi_1d_op_dn6: f64 = *var_x_wi_1d_op_dn6_slot;
        let mut var_x_wi_1d_op_dn7: f64 = *var_x_wi_1d_op_dn7_slot;
        let mut var_x_wi_1d_op_dn8: f64 = *var_x_wi_1d_op_dn8_slot;
        let mut var_x_wi_1d_op_dn9: f64 = *var_x_wi_1d_op_dn9_slot;

        let assign45200_e50341: f64 = if var_dx_wi_1d_op < 80.0 { 1.0 } else { 0.0 };
        var_guard1366 = assign45200_e50341;

        let (assign45210_e50354, assign45210_e50354_d_n4, assign45210_e50354_d_n6, assign45210_e50354_d_n7, assign45210_e50354_d_n8, assign45210_e50354_d_n9,) = {
    if (((var_guard1360 != 0.0) && (var_guard1364 == 0.0)) && (var_guard1366 != 0.0)) {
        let assign45210_e50350: f64 = (var_dx_wi_1d_op).exp();
        let assign45210_e50351: f64 = (1.0 + assign45210_e50350);
        let assign45210_e50352: f64 = (assign45210_e50351).ln();
        (assign45210_e50352, ((assign45210_e50350 * var_dx_wi_1d_op_dn4) / assign45210_e50351), ((assign45210_e50350 * var_dx_wi_1d_op_dn6) / assign45210_e50351), ((assign45210_e50350 * var_dx_wi_1d_op_dn7) / assign45210_e50351), ((assign45210_e50350 * var_dx_wi_1d_op_dn8) / assign45210_e50351), ((assign45210_e50350 * var_dx_wi_1d_op_dn9) / assign45210_e50351),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign45210_e50354;
        var_temp_dn4 = assign45210_e50354_d_n4;
        var_temp_dn6 = assign45210_e50354_d_n6;
        var_temp_dn7 = assign45210_e50354_d_n7;
        var_temp_dn8 = assign45210_e50354_d_n8;
        var_temp_dn9 = assign45210_e50354_d_n9;

        let (assign45220_e50364, assign45220_e50364_d_n4, assign45220_e50364_d_n6, assign45220_e50364_d_n7, assign45220_e50364_d_n8, assign45220_e50364_d_n9,) = {
    if (((var_guard1360 != 0.0) && (var_guard1364 == 0.0)) && (var_guard1366 == 0.0)) {
        (var_dx_wi_1d_op, var_dx_wi_1d_op_dn4, var_dx_wi_1d_op_dn6, var_dx_wi_1d_op_dn7, var_dx_wi_1d_op_dn8, var_dx_wi_1d_op_dn9,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign45220_e50364;
        var_temp_dn4 = assign45220_e50364_d_n4;
        var_temp_dn6 = assign45220_e50364_d_n6;
        var_temp_dn7 = assign45220_e50364_d_n7;
        var_temp_dn8 = assign45220_e50364_d_n8;
        var_temp_dn9 = assign45220_e50364_d_n9;

        let (assign45230_e50379, assign45230_e50379_d_n4, assign45230_e50379_d_n6, assign45230_e50379_d_n7, assign45230_e50379_d_n8, assign45230_e50379_d_n9,) = {
    if ((var_guard1360 != 0.0) && (var_guard1364 == 0.0)) {
        let assign45230_e50372: f64 = (var_dx_wi_1d_op / var_k2_1d_qm_op);
        let assign45230_e50373: f64 = (var_xg2eff_op + assign45230_e50372);
        let assign45230_e50375: f64 = (assign45230_e50373 + var_temp);
        let assign45230_e50377: f64 = (assign45230_e50375 - 0.6931471805599);
        (assign45230_e50377, ((var_xg2eff_op_dn4 + (((var_dx_wi_1d_op_dn4 * var_k2_1d_qm_op) - (var_dx_wi_1d_op * var_k2_1d_qm_op_dn4)) / (var_k2_1d_qm_op * var_k2_1d_qm_op))) + var_temp_dn4), ((var_xg2eff_op_dn6 + (((var_dx_wi_1d_op_dn6 * var_k2_1d_qm_op) - (var_dx_wi_1d_op * var_k2_1d_qm_op_dn6)) / (var_k2_1d_qm_op * var_k2_1d_qm_op))) + var_temp_dn6), ((var_xg2eff_op_dn7 + (((var_dx_wi_1d_op_dn7 * var_k2_1d_qm_op) - (var_dx_wi_1d_op * var_k2_1d_qm_op_dn7)) / (var_k2_1d_qm_op * var_k2_1d_qm_op))) + var_temp_dn7), ((var_xg2eff_op_dn8 + (((var_dx_wi_1d_op_dn8 * var_k2_1d_qm_op) - (var_dx_wi_1d_op * var_k2_1d_qm_op_dn8)) / (var_k2_1d_qm_op * var_k2_1d_qm_op))) + var_temp_dn8), ((var_xg2eff_op_dn9 + (((var_dx_wi_1d_op_dn9 * var_k2_1d_qm_op) - (var_dx_wi_1d_op * var_k2_1d_qm_op_dn9)) / (var_k2_1d_qm_op * var_k2_1d_qm_op))) + var_temp_dn9),)
    } else {
        (var_x_wi_1d_op, var_x_wi_1d_op_dn4, var_x_wi_1d_op_dn6, var_x_wi_1d_op_dn7, var_x_wi_1d_op_dn8, var_x_wi_1d_op_dn9,)
    }
};
        var_x_wi_1d_op = assign45230_e50379;
        var_x_wi_1d_op_dn4 = assign45230_e50379_d_n4;
        var_x_wi_1d_op_dn6 = assign45230_e50379_d_n6;
        var_x_wi_1d_op_dn7 = assign45230_e50379_d_n7;
        var_x_wi_1d_op_dn8 = assign45230_e50379_d_n8;
        var_x_wi_1d_op_dn9 = assign45230_e50379_d_n9;

        let (assign45240_e50398, assign45240_e50398_d_n4, assign45240_e50398_d_n6, assign45240_e50398_d_n7, assign45240_e50398_d_n8, assign45240_e50398_d_n9,) = {
    if (var_guard1360 != 0.0) {
        let assign45240_e50384: f64 = (var_x_wi_1d_op + var_xth_1d_op);
        let assign45240_e50387: f64 = (var_x_wi_1d_op - var_xth_1d_op);
        let assign45240_e50390: f64 = (var_x_wi_1d_op - var_xth_1d_op);
        let assign45240_e50391: f64 = (assign45240_e50387 * assign45240_e50390);
        let assign45240_e50393: f64 = (assign45240_e50391 + 4.0);
        let assign45240_e50394: f64 = (assign45240_e50393).sqrt();
        let assign45240_e50395: f64 = (assign45240_e50384 - assign45240_e50394);
        let assign45240_e50396: f64 = (0.5 * assign45240_e50395);
        (assign45240_e50396, (0.5 * ((var_x_wi_1d_op_dn4 + var_xth_1d_op_dn4) - ((((var_x_wi_1d_op_dn4 - var_xth_1d_op_dn4) * assign45240_e50390) + (assign45240_e50387 * (var_x_wi_1d_op_dn4 - var_xth_1d_op_dn4))) / (2.0 * assign45240_e50394)))), (0.5 * ((var_x_wi_1d_op_dn6 + var_xth_1d_op_dn6) - ((((var_x_wi_1d_op_dn6 - var_xth_1d_op_dn6) * assign45240_e50390) + (assign45240_e50387 * (var_x_wi_1d_op_dn6 - var_xth_1d_op_dn6))) / (2.0 * assign45240_e50394)))), (0.5 * ((var_x_wi_1d_op_dn7 + var_xth_1d_op_dn7) - ((((var_x_wi_1d_op_dn7 - var_xth_1d_op_dn7) * assign45240_e50390) + (assign45240_e50387 * (var_x_wi_1d_op_dn7 - var_xth_1d_op_dn7))) / (2.0 * assign45240_e50394)))), (0.5 * ((var_x_wi_1d_op_dn8 + var_xth_1d_op_dn8) - ((((var_x_wi_1d_op_dn8 - var_xth_1d_op_dn8) * assign45240_e50390) + (assign45240_e50387 * (var_x_wi_1d_op_dn8 - var_xth_1d_op_dn8))) / (2.0 * assign45240_e50394)))), (0.5 * ((var_x_wi_1d_op_dn9 + var_xth_1d_op_dn9) - ((((var_x_wi_1d_op_dn9 - var_xth_1d_op_dn9) * assign45240_e50390) + (assign45240_e50387 * (var_x_wi_1d_op_dn9 - var_xth_1d_op_dn9))) / (2.0 * assign45240_e50394)))),)
    } else {
        (var_x_1d_op, var_x_1d_op_dn4, var_x_1d_op_dn6, var_x_1d_op_dn7, var_x_1d_op_dn8, var_x_1d_op_dn9,)
    }
};
        var_x_1d_op = assign45240_e50398;
        var_x_1d_op_dn4 = assign45240_e50398_d_n4;
        var_x_1d_op_dn6 = assign45240_e50398_d_n6;
        var_x_1d_op_dn7 = assign45240_e50398_d_n7;
        var_x_1d_op_dn8 = assign45240_e50398_d_n8;
        var_x_1d_op_dn9 = assign45240_e50398_d_n9;

        let (assign45250_e50413, assign45250_e50413_d_n4, assign45250_e50413_d_n6, assign45250_e50413_d_n7, assign45250_e50413_d_n8, assign45250_e50413_d_n9,) = {
    if (var_guard1360 != 0.0) {
        let assign45250_e50404: f64 = (var_xth_1d_op - var_x_1d_op);
        let assign45250_e50405: f64 = (2.0 * assign45250_e50404);
        let assign45250_e50407: f64 = (assign45250_e50405 / var_xsddep_op);
        let assign45250_e50408: f64 = (1.0 + assign45250_e50407);
        let assign45250_e50409: f64 = (assign45250_e50408).sqrt();
        let assign45250_e50411: f64 = (assign45250_e50409 - 1.0);
        (assign45250_e50411, (((((2.0 * (var_xth_1d_op_dn4 - var_x_1d_op_dn4)) * var_xsddep_op) - (assign45250_e50405 * var_xsddep_op_dn4)) / (var_xsddep_op * var_xsddep_op)) / (2.0 * assign45250_e50409)), (((((2.0 * (var_xth_1d_op_dn6 - var_x_1d_op_dn6)) * var_xsddep_op) - (assign45250_e50405 * var_xsddep_op_dn6)) / (var_xsddep_op * var_xsddep_op)) / (2.0 * assign45250_e50409)), (((((2.0 * (var_xth_1d_op_dn7 - var_x_1d_op_dn7)) * var_xsddep_op) - (assign45250_e50405 * var_xsddep_op_dn7)) / (var_xsddep_op * var_xsddep_op)) / (2.0 * assign45250_e50409)), (((((2.0 * (var_xth_1d_op_dn8 - var_x_1d_op_dn8)) * var_xsddep_op) - (assign45250_e50405 * var_xsddep_op_dn8)) / (var_xsddep_op * var_xsddep_op)) / (2.0 * assign45250_e50409)), (((((2.0 * (var_xth_1d_op_dn9 - var_x_1d_op_dn9)) * var_xsddep_op) - (assign45250_e50405 * var_xsddep_op_dn9)) / (var_xsddep_op * var_xsddep_op)) / (2.0 * assign45250_e50409)),)
    } else {
        (var_dleff_op, var_dleff_op_dn4, var_dleff_op_dn6, var_dleff_op_dn7, var_dleff_op_dn8, var_dleff_op_dn9,)
    }
};
        var_dleff_op = assign45250_e50413;
        var_dleff_op_dn4 = assign45250_e50413_d_n4;
        var_dleff_op_dn6 = assign45250_e50413_d_n6;
        var_dleff_op_dn7 = assign45250_e50413_d_n7;
        var_dleff_op_dn8 = assign45250_e50413_d_n8;
        var_dleff_op_dn9 = assign45250_e50413_d_n9;

        let (assign45270_e50452, assign45270_e50452_d_n4, assign45270_e50452_d_n6, assign45270_e50452_d_n7, assign45270_e50452_d_n8, assign45270_e50452_d_n9,) = {
    if (var_guard1360 != 0.0) {
        let assign45270_e50427: f64 = (var_pscedlb_i * var_xg20_op);
        let assign45270_e50428: f64 = (1.0 + assign45270_e50427);
        let assign45270_e50430: f64 = (assign45270_e50428 + 0.5);
        let assign45270_e50434: f64 = (var_pscedlb_i * var_xg20_op);
        let assign45270_e50435: f64 = (1.0 + assign45270_e50434);
        let assign45270_e50437: f64 = (assign45270_e50435 - 0.5);
        let assign45270_e50441: f64 = (var_pscedlb_i * var_xg20_op);
        let assign45270_e50442: f64 = (1.0 + assign45270_e50441);
        let assign45270_e50444: f64 = (assign45270_e50442 - 0.5);
        let assign45270_e50445: f64 = (assign45270_e50437 * assign45270_e50444);
        let assign45270_e50447: f64 = (assign45270_e50445 + 0.01);
        let assign45270_e50448: f64 = (assign45270_e50447).sqrt();
        let assign45270_e50449: f64 = (assign45270_e50430 + assign45270_e50448);
        let assign45270_e50450: f64 = (0.5 * assign45270_e50449);
        (assign45270_e50450, (0.5 * ((var_pscedlb_i * var_xg20_op_dn4) + ((((var_pscedlb_i * var_xg20_op_dn4) * assign45270_e50444) + (assign45270_e50437 * (var_pscedlb_i * var_xg20_op_dn4))) / (2.0 * assign45270_e50448)))), (0.5 * ((var_pscedlb_i * var_xg20_op_dn6) + ((((var_pscedlb_i * var_xg20_op_dn6) * assign45270_e50444) + (assign45270_e50437 * (var_pscedlb_i * var_xg20_op_dn6))) / (2.0 * assign45270_e50448)))), (0.5 * ((var_pscedlb_i * var_xg20_op_dn7) + ((((var_pscedlb_i * var_xg20_op_dn7) * assign45270_e50444) + (assign45270_e50437 * (var_pscedlb_i * var_xg20_op_dn7))) / (2.0 * assign45270_e50448)))), (0.5 * ((var_pscedlb_i * var_xg20_op_dn8) + ((((var_pscedlb_i * var_xg20_op_dn8) * assign45270_e50444) + (assign45270_e50437 * (var_pscedlb_i * var_xg20_op_dn8))) / (2.0 * assign45270_e50448)))), (0.5 * ((var_pscedlb_i * var_xg20_op_dn9) + ((((var_pscedlb_i * var_xg20_op_dn9) * assign45270_e50444) + (assign45270_e50437 * (var_pscedlb_i * var_xg20_op_dn9))) / (2.0 * assign45270_e50448)))),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign45270_e50452;
        var_temp_dn4 = assign45270_e50452_d_n4;
        var_temp_dn6 = assign45270_e50452_d_n6;
        var_temp_dn7 = assign45270_e50452_d_n7;
        var_temp_dn8 = assign45270_e50452_d_n8;
        var_temp_dn9 = assign45270_e50452_d_n9;

        let (assign45300_e50499, assign45300_e50499_d_n4, assign45300_e50499_d_n6, assign45300_e50499_d_n7, assign45300_e50499_d_n8, assign45300_e50499_d_n9,) = {
    if (var_guard1360 != 0.0) {
        let assign45300_e50476: f64 = (2.0 * var_xd0_op);
        let assign45300_e50480: f64 = (var_xdsx_op / var_xd0_op);
        let assign45300_e50481: f64 = (1.0 + assign45300_e50480);
        let assign45300_e50482: f64 = (assign45300_e50481).sqrt();
        let assign45300_e50484: f64 = (assign45300_e50482 - 1.0);
        let assign45300_e50485: f64 = (assign45300_e50476 * assign45300_e50484);
        let assign45300_e50489: f64 = (var_cfdl_i * var_dleff_op);
        let assign45300_e50490: f64 = (1.0 + assign45300_e50489);
        let assign45300_e50491: f64 = (assign45300_e50485 * assign45300_e50490);
        let assign45300_e50495: f64 = (var_cfdlb_i * var_xg20_op);
        let assign45300_e50496: f64 = (1.0 + assign45300_e50495);
        let assign45300_e50497: f64 = (assign45300_e50491 * assign45300_e50496);
        (assign45300_e50497, (((((((2.0 * var_xd0_op_dn4) * assign45300_e50484) + (assign45300_e50476 * ((((var_xdsx_op_dn4 * var_xd0_op) - (var_xdsx_op * var_xd0_op_dn4)) / (var_xd0_op * var_xd0_op)) / (2.0 * assign45300_e50482)))) * assign45300_e50490) + (assign45300_e50485 * (var_cfdl_i * var_dleff_op_dn4))) * assign45300_e50496) + (assign45300_e50491 * (var_cfdlb_i * var_xg20_op_dn4))), (((((((2.0 * var_xd0_op_dn6) * assign45300_e50484) + (assign45300_e50476 * ((((var_xdsx_op_dn6 * var_xd0_op) - (var_xdsx_op * var_xd0_op_dn6)) / (var_xd0_op * var_xd0_op)) / (2.0 * assign45300_e50482)))) * assign45300_e50490) + (assign45300_e50485 * (var_cfdl_i * var_dleff_op_dn6))) * assign45300_e50496) + (assign45300_e50491 * (var_cfdlb_i * var_xg20_op_dn6))), (((((((2.0 * var_xd0_op_dn7) * assign45300_e50484) + (assign45300_e50476 * ((((var_xdsx_op_dn7 * var_xd0_op) - (var_xdsx_op * var_xd0_op_dn7)) / (var_xd0_op * var_xd0_op)) / (2.0 * assign45300_e50482)))) * assign45300_e50490) + (assign45300_e50485 * (var_cfdl_i * var_dleff_op_dn7))) * assign45300_e50496) + (assign45300_e50491 * (var_cfdlb_i * var_xg20_op_dn7))), (((((((2.0 * var_xd0_op_dn8) * assign45300_e50484) + (assign45300_e50476 * ((((var_xdsx_op_dn8 * var_xd0_op) - (var_xdsx_op * var_xd0_op_dn8)) / (var_xd0_op * var_xd0_op)) / (2.0 * assign45300_e50482)))) * assign45300_e50490) + (assign45300_e50485 * (var_cfdl_i * var_dleff_op_dn8))) * assign45300_e50496) + (assign45300_e50491 * (var_cfdlb_i * var_xg20_op_dn8))), (((((((2.0 * var_xd0_op_dn9) * assign45300_e50484) + (assign45300_e50476 * ((((var_xdsx_op_dn9 * var_xd0_op) - (var_xdsx_op * var_xd0_op_dn9)) / (var_xd0_op * var_xd0_op)) / (2.0 * assign45300_e50482)))) * assign45300_e50490) + (assign45300_e50485 * (var_cfdl_i * var_dleff_op_dn9))) * assign45300_e50496) + (assign45300_e50491 * (var_cfdlb_i * var_xg20_op_dn9))),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign45300_e50499;
        var_temp_dn4 = assign45300_e50499_d_n4;
        var_temp_dn6 = assign45300_e50499_d_n6;
        var_temp_dn7 = assign45300_e50499_d_n7;
        var_temp_dn8 = assign45300_e50499_d_n8;
        var_temp_dn9 = assign45300_e50499_d_n9;

        *var_dleff_op_slot = var_dleff_op;
        *var_dleff_op_dn4_slot = var_dleff_op_dn4;
        *var_dleff_op_dn6_slot = var_dleff_op_dn6;
        *var_dleff_op_dn7_slot = var_dleff_op_dn7;
        *var_dleff_op_dn8_slot = var_dleff_op_dn8;
        *var_dleff_op_dn9_slot = var_dleff_op_dn9;
        *var_guard1366_slot = var_guard1366;
        *var_temp_slot = var_temp;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_x_1d_op_slot = var_x_1d_op;
        *var_x_1d_op_dn4_slot = var_x_1d_op_dn4;
        *var_x_1d_op_dn6_slot = var_x_1d_op_dn6;
        *var_x_1d_op_dn7_slot = var_x_1d_op_dn7;
        *var_x_1d_op_dn8_slot = var_x_1d_op_dn8;
        *var_x_1d_op_dn9_slot = var_x_1d_op_dn9;
        *var_x_wi_1d_op_slot = var_x_wi_1d_op;
        *var_x_wi_1d_op_dn4_slot = var_x_wi_1d_op_dn4;
        *var_x_wi_1d_op_dn6_slot = var_x_wi_1d_op_dn6;
        *var_x_wi_1d_op_dn7_slot = var_x_wi_1d_op_dn7;
        *var_x_wi_1d_op_dn8_slot = var_x_wi_1d_op_dn8;
        *var_x_wi_1d_op_dn9_slot = var_x_wi_1d_op_dn9;
    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        s.store_scalar(7, (273.15 + p.p15));

        s.store_scalar(0, ((ctx_temp + p.p36)).min(1000.0));

        s.b[525] = (p.p10 == 1.0);
        s.store_scalar(525, if s.b[525] { 1.0 } else { 0.0 });

        if s.b[525] {
            s.store_scalar(8, (0.5 * ((s.v[0] + (p.p17 + (p.p18 * s.v[0]))) + (((((s.v[0] - (p.p17 + (p.p18 * s.v[0]))) * (s.v[0] - (p.p17 + (p.p18 * s.v[0])))) + p.p19)) as f64).sqrt())));
            s.store_scaled_add_offset_sqrt_square_offset_ad(221, A::div_from_scalar(10.0, A::scale(s.ad_value(8), 8.617332384961e-5)), 600.0, (-600.0), 0.01, 0.5);
        }

        if (!s.b[525]) {
            s.store_scalar(8, (0.5 * ((s.v[0] + 1.0) + (((((s.v[0] - 1.0) * (s.v[0] - 1.0)) + 0.001)) as f64).sqrt())));
            s.store_scalar(221, 600.0);
        }

        s.b[526] = (((p.p0 == 0.0) && (p.p172 > 0.0)) || ((p.p0 > 0.0) && (p.p439 > 0.0)));
        s.store_scalar(526, if s.b[526] { 1.0 } else { 0.0 });

        if s.b[526] {
            s.store_scalar(6, p.p5);
        }

        if (!s.b[526]) {
            s.store_scalar(6, 0.0);
        }

        s.store_scalar(215, 0.0);

        s.copy_ad(213, 8);

        s.store_square(214, 213);

        s.store_offset(216, 213, (-s.v[7]));

        s.store_scale(217, 213, 1.0 / (s.v[7]));

        s.store_div_from_scalar(218, s.v[7], 213);

        s.store_scale(219, 213, 8.617332384961e-5);

        s.store_div_from_scalar(220, 1.0, 219);

        s.b[607] = (p.p0 == 0.0);
        s.store_scalar(607, if s.b[607] { 1.0 } else { 0.0 });

        if s.b[607] {
            s.store_scalar(10, p.p23);
            s.store_scalar(9, p.p22);
            s.store_scalar(12, p.p25);
            s.store_scalar(11, p.p24);
            s.store_scalar(13, p.p30);
            s.store_scalar(529, p.p41);
            s.store_scalar(14, p.p42);
            s.store_scalar(15, p.p43);
            s.store_scalar(530, p.p44);
            s.store_scalar(531, 1.0);
        }

        s.b[608] = (p.p45 < 0.0);
        s.store_scalar(608, if s.b[608] { 1.0 } else { 0.0 });

        if (s.b[607] && s.b[608]) {
            s.store_scalar(531, (-1.0));
        }

        if s.b[607] {
            s.store_scalar(532, ((((p.p45) as f64).abs()).min(1e19) * 1000000.0));
            s.store_scalar(16, 1.0);
        }

        s.b[609] = (p.p46 < 0.0);
        s.store_scalar(609, if s.b[609] { 1.0 } else { 0.0 });

        if (s.b[607] && s.b[609]) {
            s.store_scalar(16, (-1.0));
        }

        if s.b[607] {
            s.store_scalar(533, (((((p.p46) as f64).abs()).max(1e16)).min(1e21) * 1000000.0));
            s.store_scalar(17, p.p47);
            s.store_scalar(18, p.p48);
            s.store_scalar(19, (p.p49 * 1000000.0));
            s.store_scalar(20, (p.p50 * 1000000.0));
            s.store_scalar(179, p.p51);
            s.store_scalar(180, p.p52);
            s.store_scalar(23, p.p53);
            s.store_scalar(24, (p.p54 * 1000000.0));
            s.store_scalar(25, p.p55);
            s.store_scalar(26, p.p56);
            s.store_scalar(27, p.p57);
            s.store_div_scaled_product_indices(28, 27, 530, p.p58, 529, 1.0);
            s.store_scalar(29, (p.p59 * 1000000.0));
            s.store_scalar(30, p.p60);
            s.store_scalar(534, p.p61);
            s.store_scalar(183, p.p62);
            s.store_div_scaled_product_indices(184, 183, 530, p.p63, 529, 1.0);
            s.store_scalar(34, p.p64);
            s.store_scalar(35, p.p65);
            s.store_scalar(36, p.p66);
            s.store_scalar(37, p.p67);
            s.store_scalar(187, p.p68);
            s.store_scale(188, 187, p.p69);
            s.store_scalar(40, p.p70);
            s.store_scalar(191, p.p71);
            s.store_scalar(41, p.p72);
            s.store_scalar(42, p.p73);
            s.store_scalar(43, p.p74);
            s.store_scalar(192, p.p75);
            s.store_scalar(45, p.p76);
            s.store_scalar(535, p.p77);
            s.store_scalar(536, p.p78);
            s.store_scalar(189, p.p79);
            s.store_scalar(48, p.p80);
            s.store_scalar(190, p.p81);
            s.store_scalar(49, p.p82);
            s.store_scalar(193, p.p83);
            s.store_scalar(51, p.p84);
            s.store_scalar(52, p.p85);
            s.store_scalar(537, p.p86);
            s.store_scalar(194, p.p87);
            s.store_scalar(54, p.p88);
            s.store_scalar(55, p.p89);
            s.store_scalar(56, p.p90);
            s.store_scalar(57, p.p91);
            s.store_scalar(58, p.p92);
            s.store_scalar(195, p.p93);
            s.store_scalar(60, p.p94);
            s.store_scalar(61, p.p95);
            s.store_scalar(62, p.p96);
            s.store_scalar(538, p.p97);
            s.store_scalar(63, p.p98);
            s.store_scalar(64, p.p99);
            s.store_scalar(65, p.p100);
            s.store_scalar(66, p.p101);
            s.store_scalar(67, p.p102);
            s.store_scalar(75, p.p103);
            s.store_scalar(197, p.p104);
            s.store_scalar(198, p.p105);
            s.store_scalar(199, p.p106);
            s.store_scalar(200, p.p107);
            s.store_scalar(201, p.p108);
            s.store_scalar(76, p.p109);
            s.store_scalar(77, p.p123);
            s.store_scalar(78, p.p110);
            s.store_scalar(79, p.p111);
            s.store_scalar(80, p.p112);
            s.store_scalar(81, p.p122);
            s.store_scalar(82, p.p113);
            s.store_scalar(83, p.p114);
            s.store_scalar(84, p.p115);
            s.store_scalar(85, p.p116);
            s.store_scalar(86, p.p117);
            s.store_scalar(87, p.p118);
            s.store_scalar(88, p.p119);
            s.store_scalar(89, p.p124);
            s.store_scalar(90, p.p125);
            s.store_scalar(204, p.p126);
            s.store_scalar(205, p.p127);
            s.store_scalar(93, p.p128);
            s.store_scalar(94, p.p129);
            s.store_scalar(95, p.p130);
            s.store_scalar(96, p.p131);
            s.store_scalar(97, p.p132);
            s.store_scalar(98, p.p133);
            s.store_scalar(206, p.p148);
            s.store_scalar(114, p.p149);
            s.store_scalar(115, p.p150);
            s.store_scalar(99, p.p134);
            s.store_scalar(207, p.p135);
            s.store_scalar(208, p.p136);
            s.store_scalar(102, p.p137);
            s.store_scalar(103, p.p138);
            s.store_scalar(104, p.p139);
            s.store_scalar(105, p.p140);
            s.store_div_scaled_product_indices(106, 105, 530, p.p141, 529, 1.0);
            s.store_scalar(107, p.p142);
            s.store_div_scaled_product_indices(108, 107, 530, p.p143, 529, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[607] {
            s.store_scalar(109, p.p144);
            s.store_scalar(209, p.p145);
            s.store_scalar(111, p.p146);
            s.store_scalar(116, p.p151);
            s.store_scalar(117, p.p152);
            s.store_scalar(118, (p.p153 * 1000000.0));
            s.store_scalar(119, p.p154);
            s.store_scalar(120, p.p155);
            s.copy_ad(181, 179);
            s.copy_ad(182, 180);
            s.copy_ad(135, 27);
            s.copy_ad(136, 28);
            s.copy_ad(185, 183);
            s.copy_ad(186, 184);
            s.copy_ad(196, 195);
            s.copy_ad(539, 538);
            s.copy_ad(158, 63);
        }

        s.b[610] = (p.p11 > 0.0);
        s.store_scalar(610, if s.b[610] { 1.0 } else { 0.0 });

        if (s.b[607] && s.b[610]) {
            s.store_scalar(181, p.p51);
        }

        s.b[611] = param_given[156];
        s.store_scalar(611, if s.b[611] { 1.0 } else { 0.0 });

        if ((s.b[607] && s.b[610]) && s.b[611]) {
            s.store_scalar(181, p.p156);
        }

        if (s.b[607] && s.b[610]) {
            s.store_scalar(182, p.p52);
        }

        s.b[612] = param_given[157];
        s.store_scalar(612, if s.b[612] { 1.0 } else { 0.0 });

        if ((s.b[607] && s.b[610]) && s.b[612]) {
            s.store_scalar(182, p.p157);
        }

        if (s.b[607] && s.b[610]) {
            s.store_scalar(135, p.p57);
        }

        s.b[613] = param_given[158];
        s.store_scalar(613, if s.b[613] { 1.0 } else { 0.0 });

        if ((s.b[607] && s.b[610]) && s.b[613]) {
            s.store_scalar(135, p.p158);
        }

        if (s.b[607] && s.b[610]) {
            s.store_div_scaled_product_indices(136, 135, 530, p.p58, 529, 1.0);
            s.store_scalar(185, p.p62);
        }

        s.b[614] = param_given[159];
        s.store_scalar(614, if s.b[614] { 1.0 } else { 0.0 });

        if ((s.b[607] && s.b[610]) && s.b[614]) {
            s.store_scalar(185, p.p159);
        }

        if (s.b[607] && s.b[610]) {
            s.store_div_scaled_product_indices(186, 185, 530, p.p63, 529, 1.0);
            s.store_scalar(196, p.p93);
        }

        s.b[615] = param_given[160];
        s.store_scalar(615, if s.b[615] { 1.0 } else { 0.0 });

        if ((s.b[607] && s.b[610]) && s.b[615]) {
            s.store_scalar(196, p.p160);
        }

        if (s.b[607] && s.b[610]) {
            s.store_scalar(539, p.p97);
        }

        s.b[616] = param_given[161];
        s.store_scalar(616, if s.b[616] { 1.0 } else { 0.0 });

        if ((s.b[607] && s.b[610]) && s.b[616]) {
            s.store_scalar(539, p.p161);
        }

        if (s.b[607] && s.b[610]) {
            s.store_scalar(158, p.p98);
        }

        s.b[617] = param_given[162];
        s.store_scalar(617, if s.b[617] { 1.0 } else { 0.0 });

        if ((s.b[607] && s.b[610]) && s.b[617]) {
            s.store_scalar(158, p.p162);
        }

        if s.b[607] {
            s.store_scalar(159, p.p163);
            s.store_scalar(160, p.p164);
            s.store_scalar(161, p.p165);
            s.store_scalar(162, p.p166);
            s.store_scalar(163, p.p167);
            s.store_scalar(164, p.p168);
            s.store_scalar(165, p.p169);
            s.store_scalar(166, p.p170);
            s.store_scalar(167, p.p171);
            s.store_scalar(210, p.p172);
            s.store_scalar(169, p.p173);
            s.store_scalar(170, p.p174);
            s.store_scalar(173, p.p177);
            s.store_scalar(174, p.p178);
            s.store_scalar(175, p.p179);
            s.store_scalar(176, p.p180);
            s.store_scalar(177, p.p181);
        }

        if (!s.b[607]) {
            s.store_scalar(584, (1.0 / p.p29));
            s.store_max_with_scalar_ad(528, A::scale(s.ad_value(584), p.p21), 1e-9);
            s.store_scale(10, 584, p.p23);
            s.store_scale(9, 584, p.p22);
            s.store_scale(12, 584, p.p25);
            s.store_scale(11, 584, p.p24);
            s.store_scalar(13, (p.p30 * p.p29));
            s.store_scalar(565, 1e-6);
            s.store_scalar(566, 1e-6);
            s.store_scale(567, 565, 1.0 / (p.p20));
            s.store_div(568, 566, 528);
            s.store_scaled_mul_scale_offset_inputs(569, 567, p.p188, 1.0, 568, p.p189, 1.0, p.p187);
            s.store_scaled_mul_scale_offset_inputs(570, 568, p.p193, 1.0, 567, p.p192, 1.0, p.p191);
            s.store_max_with_scalar_ad(571, A::offset(s.ad_value(569), ((p.p20) + ((-(2.0 * p.p190))))), 1e-9);
            s.store_max_with_scalar_ad(572, A::offset(A::add(s.ad_value(528), s.ad_value(570)), (-(2.0 * p.p194))), 1e-9);
            s.store_max_with_scalar_ad(573, A::offset(s.ad_value(569), ((((p.p20) + ((-(2.0 * p.p190))))) + (p.p195))), 1e-9);
            s.store_max_with_scalar_ad(574, A::offset(A::add(s.ad_value(528), s.ad_value(570)), (((-(2.0 * p.p194))) + (p.p196))), 1e-9);
            s.store_div(575, 565, 571);
            s.store_div(576, 566, 572);
            s.store_mul(577, 575, 576);
            s.store_max_with_scalar_ad(0, A::offset(s.ad_value(569), p.p20), 1e-9);
            s.store_div(578, 0, 565);
            s.store_max_with_scalar_ad(0, A::add(s.ad_value(528), s.ad_value(570)), 1e-9);
            s.store_div(579, 0, 566);
            s.store_scalar(529, p.p197);
            s.store_scalar(14, p.p198);
            s.store_scalar(15, p.p199);
            s.store_scalar(530, p.p200);
            s.store_scalar(531, 1.0);
        }

        s.b[618] = (p.p201 < 0.0);
        s.store_scalar(618, if s.b[618] { 1.0 } else { 0.0 });

        if ((!s.b[607]) && s.b[618]) {
            s.store_scalar(531, (-1.0));
        }

        if (!s.b[607]) {
            s.store_scalar(532, ((((p.p201) as f64).abs()).min(1e19) * 1000000.0));
            s.store_scalar(16, 1.0);
        }

        s.b[619] = (p.p202 < 0.0);
        s.store_scalar(619, if s.b[619] { 1.0 } else { 0.0 });

        if ((!s.b[607]) && s.b[619]) {
            s.store_scalar(16, (-1.0));
        }

        if (!s.b[607]) {
            s.store_scalar(533, (((((p.p202) as f64).abs()).max(1e16)).min(1e21) * 1000000.0));
            s.store_scalar(17, p.p203);
            s.store_scalar(18, p.p204);
            s.store_scalar(19, (p.p205 * 1000000.0));
            s.store_scalar(20, (p.p206 * 1000000.0));
            s.store_div_scaled_inputs(0, A::powf(s.ad_value(575), p.p209), p.p208, A::scale_offset(A::powf(s.ad_value(575), p.p211), p.p210, 1.0), 1.0);
            s.store_add_scaled_inputs3_offset_indices(179, 0, 1.0, 576, p.p212, 577, p.p213, p.p207);
            s.store_offset_mul_ad(180, A::div_scaled_inputs(s.ad_value(530), p.p215, s.ad_value(529), 1.0), s.ad_value(0), p.p214);
            s.store_mul3_ad_scaled_output(23, A::scale_offset(s.ad_value(575), p.p217, 1.0), A::scale_offset(s.ad_value(576), p.p218, 1.0), A::scale_offset(s.ad_value(577), p.p219, 1.0), p.p216);
            s.store_offset_scaled(603, 575, ((p.p221) * ((p.p220 * 1000000.0))), (p.p220 * 1000000.0));
            s.store_min_with_scalar_ad(24, A::max_with_scalar(s.ad_value(603), 1e25), 1e28);
            s.store_scalar(25, p.p222);
            s.store_scalar(26, p.p223);
            s.store_sub_from_scalar(224, 1.0, 15);
            s.store_add_scaled_inputs(225, 224, 1.04479e-10, 15, 1.43438e-10);
            s.store_div_ad_lhs(580, A::sqrt(A::mul3_scaled_output(s.ad_value(225), s.ad_value(14), A::offset(s.ad_value(529), 4e-10), 1.0 / (3.45313e-11))), 571);
            s.store_scaled_mul_scale_offset_rhs_ad(540, A::powf(s.ad_value(580), p.p225), 576, p.p226, 1.0, (p.p224 * 2.0));
            s.store_min_with_scalar_ad(27, A::max_with_scalar(s.ad_value(540), 0.0), 5.0);
            s.store_div_scaled_product_indices(28, 27, 530, p.p227, 529, 1.0);
            s.store_scalar(29, (p.p228 * 1000000.0));
            s.store_scalar(30, p.p229);
            s.store_scale(545, 576, p.p230);
            s.store_min_with_scalar_ad(534, A::max_with_scalar(s.ad_value(545), (-1.0)), 1.0);
            s.store_mul_powf_mixed_ai(0, A::scale_offset(s.ad_value(576), p.p233, 1.0), 580, p.p232);
            s.store_scale(542, 0, p.p231);
            s.store_max_with_scalar(183, 542, 0.0);
            s.store_div_scaled_product_indices(184, 183, 530, p.p234, 529, 1.0);
            s.store_scale(34, 0, p.p235);
            s.store_scalar(35, p.p236);
            s.store_div_scaled_inputs_mixed_ia(36, 575, p.p237, A::max_with_scalar(A::scale_offset(s.ad_value(576), p.p238, 1.0), 0.001), 1.0);
            s.store_scalar(37, p.p239);
            s.store_div_scaled_inputs_mixed_ia(2, 571, -1.0, A::max_with_scalar(A::scale_offset(s.ad_value(576), p.p244, 1.0), 0.001), p.p243);
        }

        s.b[620] = (s.v[2] > (-80.0));
        s.store_scalar(620, if s.b[620] { 1.0 } else { 0.0 });

        if ((!s.b[607]) && s.b[620]) {
            s.store_exp(3, 2);
        }

        if ((!s.b[607]) && (!s.b[620])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(3, 1.80485e-35, A::neg(s.ad_value(2)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if (!s.b[607]) {
            s.store_scale(4, 571, (-1.0 / (p.p246)));
        }

        s.b[621] = (s.v[4] > (-80.0));
        s.store_scalar(621, if s.b[621] { 1.0 } else { 0.0 });

        if ((!s.b[607]) && s.b[621]) {
            s.store_exp(5, 4);
        }

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((!s.b[607]) && (!s.b[621])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mul_scaled_ad(5, 1.80485e-35, A::neg(s.ad_value(4)), (-80.0), 0.3333333333333, (((((-80.0)) * (0.3333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if (!s.b[607]) {
            s.store_max_with_scalar_ad(581, A::add(A::offset(A::div_scaled_product_offset_rhs(A::scale_offset(s.ad_value(576), p.p242, 1.0), s.ad_value(3), (-1.0), p.p241, s.ad_value(2), 1.0), 1.0), A::div_scaled_offset_numerator(s.ad_value(5), p.p245, ((-1.0) * p.p245), s.ad_value(4), 1.0)), 1e-6);
            s.store_max_with_scalar_ad(582, A::add_scaled_product(A::scale_offset(s.ad_value(576), p.p247, 1.0), 1.0, s.ad_value(576), A::ln(A::scale_offset(s.ad_value(572), 1.0 / (p.p249), 1.0)), p.p248), 1e-6);
            s.store_mul_div_from_scalar_lhs(583, p.p240, 581, 582);
            s.store_div_scaled_product_indices(544, 583, 572, 1.0, 571, 1.0);
            s.store_max_with_scalar(187, 544, 1e-10);
            s.store_scale(188, 187, p.p250);
            s.store_mul3_ad_scaled_output(40, A::scale_offset(s.ad_value(575), p.p252, 1.0), A::scale_offset(s.ad_value(576), p.p253, 1.0), A::scale_offset(s.ad_value(577), p.p254, 1.0), p.p251);
            s.store_mul3_ad(546, A::scale_offset(A::powf(s.ad_value(575), p.p257), p.p256, p.p255), A::scale_offset(s.ad_value(576), p.p258, 1.0), A::scale_offset(s.ad_value(577), p.p259, 1.0));
            s.store_max_with_scalar(191, 546, 0.0);
            s.store_scalar(41, p.p260);
            s.store_scalar(42, p.p261);
            s.store_mul3_ad_scaled_output(43, A::scale_offset(s.ad_value(575), p.p263, 1.0), A::scale_offset(s.ad_value(576), p.p264, 1.0), A::scale_offset(s.ad_value(577), p.p265, 1.0), p.p262);
            s.store_scalar(192, p.p266);
            s.store_scalar(45, p.p267);
            s.store_scalar(535, p.p268);
            s.store_scalar(536, p.p269);
            s.store_scalar(189, p.p270);
            s.store_scalar(48, p.p271);
            s.store_scalar(190, p.p272);
            s.store_scalar(49, p.p273);
            s.store_mul3_ad(193, A::scale_offset(A::powf(s.ad_value(575), p.p276), p.p275, p.p274), A::scale_offset(s.ad_value(576), p.p277, 1.0), A::scale_offset(s.ad_value(577), p.p278, 1.0));
            s.store_scalar(51, p.p279);
            s.store_scalar(52, p.p280);
            s.store_scalar(537, p.p281);
            s.store_mul_scale_offset_rhs(547, 576, 576, ((p.p283) * (p.p282)), p.p282);
            s.store_max_with_scalar(194, 547, 0.0);
            s.store_scalar(54, p.p284);
            s.store_scalar(55, p.p285);
            s.store_scalar(56, p.p286);
            s.store_scalar(57, p.p287);
            s.store_scalar(58, p.p288);
            s.store_mul_product3_mixed_aiaa(548, A::scale_offset(s.ad_value(577), p.p293, 1.0), 583, A::scale_offset(A::powf(s.ad_value(575), p.p291), p.p290, p.p289), A::scale_offset(s.ad_value(576), p.p292, 1.0), 1.0);
            s.store_max_with_scalar(195, 548, 0.0);
            s.store_mul3_ad_scaled_output(60, A::scale_offset(s.ad_value(575), p.p295, 1.0), A::scale_offset(s.ad_value(576), p.p296, 1.0), A::scale_offset(s.ad_value(577), p.p297, 1.0), p.p294);
            s.store_scalar(61, p.p298);
            s.store_scalar(62, p.p299);
            s.store_div_from_scalar_offset_ad(550, p.p300, A::div_scaled_inputs(A::powf(s.ad_value(575), p.p302), p.p301, A::scale_offset(A::powf(s.ad_value(575), p.p304), p.p303, 1.0), 1.0), 1.0);
            s.store_min_with_scalar_ad(538, A::max_with_scalar(s.ad_value(550), 1.0), 16.0);
            s.store_div_scaled_product(553, A::powf(s.ad_value(575), p.p306), A::scale_offset(s.ad_value(576), p.p309, 1.0), p.p305, A::scale_offset(A::powf(s.ad_value(575), p.p308), p.p307, 1.0), 1.0);
            s.store_max_with_scalar(63, 553, 0.0);
            s.store_div_scaled_product(554, A::powf(s.ad_value(575), p.p311), A::scale_offset(s.ad_value(576), p.p314, 1.0), p.p310, A::scale_offset(A::powf(s.ad_value(575), p.p313), p.p312, 1.0), 1.0);
            s.store_max_with_scalar(64, 554, 0.0);
            s.store_scalar(65, p.p315);
            s.store_scalar(66, p.p316);
            s.store_scalar(67, p.p317);
            s.store_scalar(75, p.p318);
            s.store_div_from_scalar(197, p.p319, 577);
            s.store_div_from_scalar(198, p.p320, 576);
            s.store_div_from_scalar(199, p.p321, 576);
            s.store_div_from_scalar(200, p.p322, 576);
            s.store_div_from_scalar(201, p.p323, 576);
            s.store_scalar(76, p.p324);
            s.store_scalar(77, p.p338);
            s.store_scalar(78, p.p325);
            s.store_scalar(79, p.p326);
            s.store_scalar(80, p.p327);
            s.store_scalar(81, p.p337);
            s.store_scalar(82, p.p328);
            s.store_scalar(83, p.p329);
            s.store_scalar(84, p.p330);
            s.store_scale(85, 575, p.p331);
            s.store_scalar(86, p.p332);
            s.store_scalar(87, p.p333);
            s.store_scalar(88, p.p334);
            s.store_offset_div_from_scalar_ad(555, p.p341, s.ad_value(576), p.p339);
            s.store_max_with_scalar(89, 555, 0.0);
            s.store_offset_div_from_scalar_ad(556, p.p342, s.ad_value(576), p.p340);
            s.store_max_with_scalar(90, 556, 0.0);
            s.store_scalar(204, p.p343);
            s.store_scalar(205, p.p344);
            s.store_scalar(93, p.p345);
            s.store_scalar(94, p.p346);
            s.store_scalar(95, p.p347);
            s.store_scalar(96, p.p348);
            s.store_offset_scaled(97, 575, p.p351, p.p349);
            s.store_offset_scaled(98, 575, p.p352, p.p350);
            s.store_scalar(206, p.p387);
            s.store_scalar(114, p.p388);
            s.store_scaled_mul_scale_offset_inputs(558, 575, p.p390, 1.0, 576, p.p391, 1.0, p.p389);
            s.store_max_with_scalar(115, 558, 0.0);
            s.store_offset_scaled(585, 572, p.p354, (2.0 * p.p353));
            s.store_scalar(99, p.p355);
            s.store_scale_ad(0, A::powf(s.ad_value(575), p.p358), p.p357);
            s.store_add_scaled_inputs3_offset_indices(207, 0, 1.0, 576, p.p359, 577, p.p360, p.p356);
            s.store_scalar(208, p.p361);
            s.store_mul3_ad_scaled_output(102, A::scale_offset(s.ad_value(575), p.p363, 1.0), A::scale_offset(s.ad_value(576), p.p364, 1.0), A::scale_offset(s.ad_value(577), p.p365, 1.0), p.p362);
            s.store_scalar(103, p.p366);
            s.store_scalar(104, p.p367);
            s.store_scaled_mul_scale_offset_rhs_ad(0, A::powf(s.ad_value(580), p.p369), 576, p.p370, 1.0, (p.p368 * 2.0));
            s.store_min_with_scalar_ad(105, A::max_with_scalar(s.ad_value(0), 0.0), 5.0);
            s.store_div_scaled_product_indices(106, 105, 530, p.p371, 529, 1.0);
            s.store_mul_powf_mixed_ai(0, A::scale_offset(s.ad_value(576), p.p374, 1.0), 580, p.p373);
            s.store_scale(0, 0, p.p372);
            s.store_max_with_scalar(107, 0, 0.0);
            s.store_div_scaled_product_indices(108, 107, 530, p.p375, 529, 1.0);
            s.store_scalar(109, p.p376);
            s.store_offset_ad(0, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p377 * p.p378), s.ad_value(571)), 1.0, A::exp_scaled_input(s.ad_value(571), (-1.0 / (p.p378)))), 1.0);
            s.store_max_with_scalar(0, 0, 1e-15);
            s.store_mul_div_scaled_inputs_mixed_aia(209, A::scale_offset(s.ad_value(576), p.p379, 1.0), 585, p.p240, A::mul(s.ad_value(0), s.ad_value(571)), 1.0);
            s.store_add_scaled_inputs_product_first_ad(111, A::scale_offset(s.ad_value(575), p.p381, p.p380), 1.0, 576, p.p382, 575, 576, p.p383);
            s.store_mul(116, 574, 573);
            s.store_offset_scaled(559, 578, p.p393, p.p392);
            s.store_max_with_scalar(117, 559, 0.0);
            s.store_scalar(118, (p.p394 * 1000000.0));
            s.store_div_scaled_inputs_indices(119, 574, p.p395, 566, 1.0);
            s.store_scalar(120, p.p396);
            s.copy_ad(181, 179);
            s.copy_ad(182, 180);
            s.copy_ad(135, 27);
            s.copy_ad(136, 28);
            s.copy_ad(543, 542);
            s.copy_ad(185, 183);
            s.copy_ad(186, 184);
            s.copy_ad(549, 548);
            s.copy_ad(196, 195);
            s.copy_ad(539, 538);
            s.copy_ad(158, 63);
        }

        s.b[622] = (p.p11 > 0.0);
        s.store_scalar(622, if s.b[622] { 1.0 } else { 0.0 });

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(121, p.p207);
        }

        s.b[623] = param_given[397];
        s.store_scalar(623, if s.b[623] { 1.0 } else { 0.0 });

        if (((!s.b[607]) && s.b[622]) && s.b[623]) {
            s.store_scalar(121, p.p397);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(122, p.p208);
        }

        s.b[624] = param_given[398];
        s.store_scalar(624, if s.b[624] { 1.0 } else { 0.0 });

        if (((!s.b[607]) && s.b[622]) && s.b[624]) {
            s.store_scalar(122, p.p398);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(123, p.p209);
        }

        s.b[625] = param_given[399];
        s.store_scalar(625, if s.b[625] { 1.0 } else { 0.0 });

        if (((!s.b[607]) && s.b[622]) && s.b[625]) {
            s.store_scalar(123, p.p399);
        }

        if ((!s.b[607]) && s.b[622]) {
            s.store_scalar(124, p.p212);
        }

    }
}
