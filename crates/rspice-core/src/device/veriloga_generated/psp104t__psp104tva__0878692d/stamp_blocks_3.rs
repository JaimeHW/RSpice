#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_19(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
            let assign46360_e59406: f64 = (2.0 / locals.var_tme1);
            let assign46360_e59410: f64 = (locals.var_tme2 * locals.var_tme2);
            let assign46360_e59413: f64 = (locals.var_tme1 * locals.var_fs1);
            let assign46360_e59415: f64 = (assign46360_e59413 * locals.var_fs3);
            let assign46360_e59416: f64 = (assign46360_e59410 - assign46360_e59415);
            let assign46360_e59417: f64 = (assign46360_e59416).sqrt();
            let assign46360_e59418: f64 = (locals.var_tme2 - assign46360_e59417);
            let assign46360_e59419: f64 = (assign46360_e59406 * assign46360_e59418);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign46360_e59419, (assign46360_e59406 * (locals.var_tme2_dn4 - (((locals.var_tme2_dn4 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn4)) / (2.0 * assign46360_e59417)))), (assign46360_e59406 * (locals.var_tme2_dn6 - ((((locals.var_tme2_dn6 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn6)) - (((locals.var_tme1 * locals.var_fs1_dn6) * locals.var_fs3) + (assign46360_e59413 * locals.var_fs3_dn6))) / (2.0 * assign46360_e59417)))), (assign46360_e59406 * (locals.var_tme2_dn7 - ((((locals.var_tme2_dn7 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn7)) - (((locals.var_tme1 * locals.var_fs1_dn7) * locals.var_fs3) + (assign46360_e59413 * locals.var_fs3_dn7))) / (2.0 * assign46360_e59417)))), (assign46360_e59406 * (locals.var_tme2_dn8 - ((((locals.var_tme2_dn8 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn8)) - (((locals.var_tme1 * locals.var_fs1_dn8) * locals.var_fs3) + (assign46360_e59413 * locals.var_fs3_dn8))) / (2.0 * assign46360_e59417)))), (assign46360_e59406 * (locals.var_tme2_dn9 - (((locals.var_tme2_dn9 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn9)) / (2.0 * assign46360_e59417)))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
            let assign46370_e59427: f64 = (4.0 - 0.3);
            locals.var_tme1 = assign46370_e59427;
            locals.var_tme1_rv = 0.0;
        }

        if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
            let assign46380_e59435: f64 = (locals.var_fs2 + locals.var_temp__blk949);
            (locals.var_tme2, locals.var_tme2_dn4, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8, locals.var_tme2_dn9, ) = (assign46380_e59435, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, );
            locals.var_tme2_rv = 0.0;
        }

        let assign46410_e59471: f64 = if locals.var_igovd_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1243 = assign46410_e59471;
        locals.var_guard1243_rv = 0.0;

        if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
            let assign46420_e59477: f64 = (locals.var_vovd * locals.var_vovd);
            let assign46420_e59479: f64 = (assign46420_e59477 + 1e-6);
            let assign46420_e59480: f64 = (assign46420_e59479).sqrt();
            let assign46420_e59482: f64 = (assign46420_e59480 * locals.var_inv_chib);
            (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9, ) = (assign46420_e59482, 0.0, ((((locals.var_vovd_dn6 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn6)) / (2.0 * assign46420_e59480)) * locals.var_inv_chib), ((((locals.var_vovd_dn7 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn7)) / (2.0 * assign46420_e59480)) * locals.var_inv_chib), ((((locals.var_vovd_dn8 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn8)) / (2.0 * assign46420_e59480)) * locals.var_inv_chib), 0.0, );
            locals.var_zg_rv = 0.0;
        }

        let assign46430_e59487: f64 = if locals.var_gc3ovd_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1244 = assign46430_e59487;
        locals.var_guard1244_rv = 0.0;

        if (((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) && (locals.var_guard1244 != 0.0)) {
            let assign46440_e59496: f64 = (locals.var_zg + locals.var_gcqovd);
            let assign46440_e59499: f64 = (locals.var_zg - locals.var_gcqovd);
            let assign46440_e59502: f64 = (locals.var_zg - locals.var_gcqovd);
            let assign46440_e59503: f64 = (assign46440_e59499 * assign46440_e59502);
            let assign46440_e59505: f64 = (assign46440_e59503 + 1e-6);
            let assign46440_e59506: f64 = (assign46440_e59505).sqrt();
            let assign46440_e59507: f64 = (assign46440_e59496 - assign46440_e59506);
            let assign46440_e59508: f64 = (0.5 * assign46440_e59507);
            (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9, ) = (assign46440_e59508, (0.5 * (locals.var_zg_dn4 - (((locals.var_zg_dn4 * assign46440_e59502) + (assign46440_e59499 * locals.var_zg_dn4)) / (2.0 * assign46440_e59506)))), (0.5 * (locals.var_zg_dn6 - (((locals.var_zg_dn6 * assign46440_e59502) + (assign46440_e59499 * locals.var_zg_dn6)) / (2.0 * assign46440_e59506)))), (0.5 * (locals.var_zg_dn7 - (((locals.var_zg_dn7 * assign46440_e59502) + (assign46440_e59499 * locals.var_zg_dn7)) / (2.0 * assign46440_e59506)))), (0.5 * (locals.var_zg_dn8 - (((locals.var_zg_dn8 * assign46440_e59502) + (assign46440_e59499 * locals.var_zg_dn8)) / (2.0 * assign46440_e59506)))), (0.5 * (locals.var_zg_dn9 - (((locals.var_zg_dn9 * assign46440_e59502) + (assign46440_e59499 * locals.var_zg_dn9)) / (2.0 * assign46440_e59506)))), );
            locals.var_zg_rv = 0.0;
        }

        if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
            let assign46450_e59516: f64 = (-1.5);
            let assign46450_e59521: f64 = (locals.var_gc3ovd_i * locals.var_zg);
            let assign46450_e59522: f64 = (locals.var_gc2ovd_i + assign46450_e59521);
            let assign46450_e59523: f64 = (locals.var_zg * assign46450_e59522);
            let assign46450_e59524: f64 = (assign46450_e59516 + assign46450_e59523);
            let assign46450_e59525: f64 = (locals.var_bov_d * assign46450_e59524);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign46450_e59525, (locals.var_bov_d * ((locals.var_zg_dn4 * assign46450_e59522) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn4)))), (locals.var_bov_d * ((locals.var_zg_dn6 * assign46450_e59522) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn6)))), (locals.var_bov_d * ((locals.var_zg_dn7 * assign46450_e59522) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn7)))), (locals.var_bov_d * ((locals.var_zg_dn8 * assign46450_e59522) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn8)))), (locals.var_bov_d * ((locals.var_zg_dn9 * assign46450_e59522) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn9)))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
            let assign46510_e59611: f64 = (3.0 + locals.var_xd_ov);
            (locals.var_fs1, locals.var_fs1_dn6, locals.var_fs1_dn7, locals.var_fs1_dn8, ) = (assign46510_e59611, locals.var_xd_ov_dn6, locals.var_xd_ov_dn7, locals.var_xd_ov_dn8, );
            locals.var_fs1_rv = 0.0;
        }

        if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
            let assign46520_e59618: f64 = (-3.0);
            let assign46520_e59620: f64 = (assign46520_e59618 - locals.var_gco_i);
            locals.var_fs2 = assign46520_e59620;
            locals.var_fs2_rv = 0.0;
        }

        if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
            let assign46530_e59628: f64 = (30.0 * locals.var_vgdprime);
            (locals.var_fs3, locals.var_fs3_dn6, locals.var_fs3_dn7, locals.var_fs3_dn8, ) = (assign46530_e59628, (30.0 * locals.var_vgdprime_dn6), (30.0 * locals.var_vgdprime_dn7), (30.0 * locals.var_vgdprime_dn8), );
            locals.var_fs3_rv = 0.0;
        }

        if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
            let assign46540_e59636: f64 = (4.0 - 0.9);
            locals.var_tme1 = assign46540_e59636;
            locals.var_tme1_rv = 0.0;
        }

        if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
            let assign46550_e59644: f64 = (locals.var_fs1 + locals.var_fs3);
            (locals.var_tme2, locals.var_tme2_dn4, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8, locals.var_tme2_dn9, ) = (assign46550_e59644, 0.0, (locals.var_fs1_dn6 + locals.var_fs3_dn6), (locals.var_fs1_dn7 + locals.var_fs3_dn7), (locals.var_fs1_dn8 + locals.var_fs3_dn8), 0.0, );
            locals.var_tme2_rv = 0.0;
        }

        if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
            let assign46560_e59652: f64 = (2.0 / locals.var_tme1);
            let assign46560_e59656: f64 = (locals.var_tme2 * locals.var_tme2);
            let assign46560_e59659: f64 = (locals.var_tme1 * locals.var_fs1);
            let assign46560_e59661: f64 = (assign46560_e59659 * locals.var_fs3);
            let assign46560_e59662: f64 = (assign46560_e59656 - assign46560_e59661);
            let assign46560_e59663: f64 = (assign46560_e59662).sqrt();
            let assign46560_e59664: f64 = (locals.var_tme2 - assign46560_e59663);
            let assign46560_e59665: f64 = (assign46560_e59652 * assign46560_e59664);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign46560_e59665, (assign46560_e59652 * (locals.var_tme2_dn4 - (((locals.var_tme2_dn4 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn4)) / (2.0 * assign46560_e59663)))), (assign46560_e59652 * (locals.var_tme2_dn6 - ((((locals.var_tme2_dn6 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn6)) - (((locals.var_tme1 * locals.var_fs1_dn6) * locals.var_fs3) + (assign46560_e59659 * locals.var_fs3_dn6))) / (2.0 * assign46560_e59663)))), (assign46560_e59652 * (locals.var_tme2_dn7 - ((((locals.var_tme2_dn7 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn7)) - (((locals.var_tme1 * locals.var_fs1_dn7) * locals.var_fs3) + (assign46560_e59659 * locals.var_fs3_dn7))) / (2.0 * assign46560_e59663)))), (assign46560_e59652 * (locals.var_tme2_dn8 - ((((locals.var_tme2_dn8 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn8)) - (((locals.var_tme1 * locals.var_fs1_dn8) * locals.var_fs3) + (assign46560_e59659 * locals.var_fs3_dn8))) / (2.0 * assign46560_e59663)))), (assign46560_e59652 * (locals.var_tme2_dn9 - (((locals.var_tme2_dn9 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn9)) / (2.0 * assign46560_e59663)))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
            let assign46570_e59673: f64 = (4.0 - 0.3);
            locals.var_tme1 = assign46570_e59673;
            locals.var_tme1_rv = 0.0;
        }

        if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
            let assign46580_e59681: f64 = (locals.var_fs2 + locals.var_temp__blk949);
            (locals.var_tme2, locals.var_tme2_dn4, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8, locals.var_tme2_dn9, ) = (assign46580_e59681, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, );
            locals.var_tme2_rv = 0.0;
        }

        let assign46610_e59717: f64 = if locals.var_iginv_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1247 = assign46610_e59717;
        locals.var_guard1247_rv = 0.0;

        let assign46620_e59720: f64 = if locals.var_xg_dc <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1248 = assign46620_e59720;
        locals.var_guard1248_rv = 0.0;

        if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1248 != 0.0)) {
            let assign46630_e59728: f64 = (1.0 + locals.var_ar);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign46630_e59728, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp__blk949_rv = 0.0;
        }

        if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1248 != 0.0)) {
            let assign46640_e59737: f64 = (locals.var_temp__blk949).sqrt();
            let assign46640_e59739: f64 = (assign46640_e59737 * locals.var_v_ds);
            let assign46640_e59741: f64 = (assign46640_e59739 / locals.var_vdsat_lim_dc);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign46640_e59741, (((((locals.var_temp__blk949_dn4 / (2.0 * assign46640_e59737)) * locals.var_v_ds) * locals.var_vdsat_lim_dc) - (assign46640_e59739 * locals.var_vdsat_lim_dc_dn4)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), (((((locals.var_temp__blk949_dn6 / (2.0 * assign46640_e59737)) * locals.var_v_ds) * locals.var_vdsat_lim_dc) - (assign46640_e59739 * locals.var_vdsat_lim_dc_dn6)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), ((((((locals.var_temp__blk949_dn7 / (2.0 * assign46640_e59737)) * locals.var_v_ds) + (assign46640_e59737 * locals.var_v_ds_dn7)) * locals.var_vdsat_lim_dc) - (assign46640_e59739 * locals.var_vdsat_lim_dc_dn7)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), ((((((locals.var_temp__blk949_dn8 / (2.0 * assign46640_e59737)) * locals.var_v_ds) + (assign46640_e59737 * locals.var_v_ds_dn8)) * locals.var_vdsat_lim_dc) - (assign46640_e59739 * locals.var_vdsat_lim_dc_dn8)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), (((((locals.var_temp__blk949_dn9 / (2.0 * assign46640_e59737)) * locals.var_v_ds) * locals.var_vdsat_lim_dc) - (assign46640_e59739 * locals.var_vdsat_lim_dc_dn9)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), );
            locals.var_temp1_rv = 0.0;
        }

        if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1248 != 0.0)) {
            let assign46650_e59751: f64 = (locals.var_temp1 * locals.var_temp1);
            let assign46650_e59753: f64 = (assign46650_e59751 + locals.var_temp__blk949);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign46650_e59753, (((locals.var_temp1_dn4 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn4)) + locals.var_temp__blk949_dn4), (((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)) + locals.var_temp__blk949_dn6), (((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)) + locals.var_temp__blk949_dn7), (((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)) + locals.var_temp__blk949_dn8), (((locals.var_temp1_dn9 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn9)) + locals.var_temp__blk949_dn9), );
            locals.var_temp2_rv = 0.0;
        }

        if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1248 != 0.0)) {
            let assign46660_e59763: f64 = (2.0 * locals.var_temp1);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign46660_e59763, (2.0 * locals.var_temp1_dn4), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), (2.0 * locals.var_temp1_dn9), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1248 != 0.0)) {
            let assign46670_e59773: f64 = (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc);
            let assign46670_e59775: f64 = (assign46670_e59773 * locals.var_temp__blk949);
            let assign46670_e59778: f64 = (locals.var_temp2 - locals.var_temp__blk949);
            let assign46670_e59779: f64 = (assign46670_e59778).sqrt();
            let assign46670_e59782: f64 = (locals.var_temp2 + locals.var_temp__blk949);
            let assign46670_e59783: f64 = (assign46670_e59782).sqrt();
            let assign46670_e59784: f64 = (assign46670_e59779 + assign46670_e59783);
            let assign46670_e59785: f64 = (assign46670_e59775 / assign46670_e59784);
            (locals.var_udse_dc, locals.var_udse_dc_dn4, locals.var_udse_dc_dn6, locals.var_udse_dc_dn7, locals.var_udse_dc_dn8, locals.var_udse_dc_dn9, ) = (assign46670_e59785, (((((((locals.var_vdsat_lim_dc_dn4 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn4)) * locals.var_temp__blk949) + (assign46670_e59773 * locals.var_temp__blk949_dn4)) * assign46670_e59784) - (assign46670_e59775 * (((locals.var_temp2_dn4 - locals.var_temp__blk949_dn4) / (2.0 * assign46670_e59779)) + ((locals.var_temp2_dn4 + locals.var_temp__blk949_dn4) / (2.0 * assign46670_e59783))))) / (assign46670_e59784 * assign46670_e59784)), (((((((locals.var_vdsat_lim_dc_dn6 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn6)) * locals.var_temp__blk949) + (assign46670_e59773 * locals.var_temp__blk949_dn6)) * assign46670_e59784) - (assign46670_e59775 * (((locals.var_temp2_dn6 - locals.var_temp__blk949_dn6) / (2.0 * assign46670_e59779)) + ((locals.var_temp2_dn6 + locals.var_temp__blk949_dn6) / (2.0 * assign46670_e59783))))) / (assign46670_e59784 * assign46670_e59784)), (((((((locals.var_vdsat_lim_dc_dn7 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn7)) * locals.var_temp__blk949) + (assign46670_e59773 * locals.var_temp__blk949_dn7)) * assign46670_e59784) - (assign46670_e59775 * (((locals.var_temp2_dn7 - locals.var_temp__blk949_dn7) / (2.0 * assign46670_e59779)) + ((locals.var_temp2_dn7 + locals.var_temp__blk949_dn7) / (2.0 * assign46670_e59783))))) / (assign46670_e59784 * assign46670_e59784)), (((((((locals.var_vdsat_lim_dc_dn8 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn8)) * locals.var_temp__blk949) + (assign46670_e59773 * locals.var_temp__blk949_dn8)) * assign46670_e59784) - (assign46670_e59775 * (((locals.var_temp2_dn8 - locals.var_temp__blk949_dn8) / (2.0 * assign46670_e59779)) + ((locals.var_temp2_dn8 + locals.var_temp__blk949_dn8) / (2.0 * assign46670_e59783))))) / (assign46670_e59784 * assign46670_e59784)), (((((((locals.var_vdsat_lim_dc_dn9 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn9)) * locals.var_temp__blk949) + (assign46670_e59773 * locals.var_temp__blk949_dn9)) * assign46670_e59784) - (assign46670_e59775 * (((locals.var_temp2_dn9 - locals.var_temp__blk949_dn9) / (2.0 * assign46670_e59779)) + ((locals.var_temp2_dn9 + locals.var_temp__blk949_dn9) / (2.0 * assign46670_e59783))))) / (assign46670_e59784 * assign46670_e59784)), );
            locals.var_udse_dc_rv = 0.0;
        }

        let assign46680_e59790: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
        let assign46680_e59792: f64 = (-230.25850929940458);
        let assign46680_e59793: f64 = if assign46680_e59790 > assign46680_e59792 { 1.0 } else { 0.0 };
        locals.var_guard1249 = assign46680_e59793;
        locals.var_guard1249_rv = 0.0;

        if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1249 != 0.0)) {
            let assign46690_e59801: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
            let assign46690_e59802: f64 = (assign46690_e59801).exp();
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign46690_e59802, (assign46690_e59802 * (locals.var_x_ds_dc_dn4 - locals.var_udse_dc_dn4)), (assign46690_e59802 * (locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)), (assign46690_e59802 * (locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)), (assign46690_e59802 * (locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)), (assign46690_e59802 * (locals.var_x_ds_dc_dn9 - locals.var_udse_dc_dn9)), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1249 == 0.0)) {
            let assign46700_e59814: f64 = (-230.25850929940458);
            let assign46700_e59817: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
            let assign46700_e59818: f64 = (assign46700_e59814 - assign46700_e59817);
            let assign46700_e59822: f64 = (-230.25850929940458);
            let assign46700_e59825: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
            let assign46700_e59826: f64 = (assign46700_e59822 - assign46700_e59825);
            let assign46700_e59829: f64 = (-230.25850929940458);
            let assign46700_e59832: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
            let assign46700_e59833: f64 = (assign46700_e59829 - assign46700_e59832);
            let assign46700_e59835: f64 = (assign46700_e59833 * 0.3333333333333333);
            let assign46700_e59836: f64 = (1.0 + assign46700_e59835);
            let assign46700_e59837: f64 = (assign46700_e59826 * assign46700_e59836);
            let assign46700_e59838: f64 = (0.5 * assign46700_e59837);
            let assign46700_e59839: f64 = (1.0 + assign46700_e59838);
            let assign46700_e59840: f64 = (assign46700_e59818 * assign46700_e59839);
            let assign46700_e59841: f64 = (1.0 + assign46700_e59840);
            let assign46700_e59842: f64 = (1e-100 / assign46700_e59841);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign46700_e59842, (-((1e-100 * (((-(locals.var_x_ds_dc_dn4 - locals.var_udse_dc_dn4)) * assign46700_e59839) + (assign46700_e59818 * (0.5 * (((-(locals.var_x_ds_dc_dn4 - locals.var_udse_dc_dn4)) * assign46700_e59836) + (assign46700_e59826 * ((-(locals.var_x_ds_dc_dn4 - locals.var_udse_dc_dn4)) * 0.3333333333333333))))))) / (assign46700_e59841 * assign46700_e59841))), (-((1e-100 * (((-(locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)) * assign46700_e59839) + (assign46700_e59818 * (0.5 * (((-(locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)) * assign46700_e59836) + (assign46700_e59826 * ((-(locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)) * 0.3333333333333333))))))) / (assign46700_e59841 * assign46700_e59841))), (-((1e-100 * (((-(locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)) * assign46700_e59839) + (assign46700_e59818 * (0.5 * (((-(locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)) * assign46700_e59836) + (assign46700_e59826 * ((-(locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)) * 0.3333333333333333))))))) / (assign46700_e59841 * assign46700_e59841))), (-((1e-100 * (((-(locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)) * assign46700_e59839) + (assign46700_e59818 * (0.5 * (((-(locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)) * assign46700_e59836) + (assign46700_e59826 * ((-(locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)) * 0.3333333333333333))))))) / (assign46700_e59841 * assign46700_e59841))), (-((1e-100 * (((-(locals.var_x_ds_dc_dn9 - locals.var_udse_dc_dn9)) * assign46700_e59839) + (assign46700_e59818 * (0.5 * (((-(locals.var_x_ds_dc_dn9 - locals.var_udse_dc_dn9)) * assign46700_e59836) + (assign46700_e59826 * ((-(locals.var_x_ds_dc_dn9 - locals.var_udse_dc_dn9)) * 0.3333333333333333))))))) / (assign46700_e59841 * assign46700_e59841))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
            let assign46710_e59852: f64 = (0.5 * locals.var_x_ds_dc);
            let assign46710_e59856: f64 = (1.0 + locals.var_temp__blk949);
            let assign46710_e59857: f64 = (0.5 * assign46710_e59856);
            let assign46710_e59858: f64 = (assign46710_e59857).ln();
            let assign46710_e59859: f64 = (assign46710_e59852 - assign46710_e59858);
            let assign46710_e59860: f64 = (locals.var_phit1_dc * assign46710_e59859);
            let assign46710_e59861: f64 = (locals.var_vsbstar_dc + assign46710_e59860);
            (locals.var_vm, locals.var_vm_dn4, locals.var_vm_dn6, locals.var_vm_dn7, locals.var_vm_dn8, locals.var_vm_dn9, ) = (assign46710_e59861, (locals.var_vsbstar_dc_dn4 + ((locals.var_phit1_dc_dn4 * assign46710_e59859) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn4) - ((0.5 * locals.var_temp__blk949_dn4) / assign46710_e59857))))), (locals.var_vsbstar_dc_dn6 + ((locals.var_phit1_dc_dn6 * assign46710_e59859) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn6) - ((0.5 * locals.var_temp__blk949_dn6) / assign46710_e59857))))), (locals.var_vsbstar_dc_dn7 + ((locals.var_phit1_dc_dn7 * assign46710_e59859) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn7) - ((0.5 * locals.var_temp__blk949_dn7) / assign46710_e59857))))), (locals.var_vsbstar_dc_dn8 + ((locals.var_phit1_dc_dn8 * assign46710_e59859) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn8) - ((0.5 * locals.var_temp__blk949_dn8) / assign46710_e59857))))), (locals.var_vsbstar_dc_dn9 + ((locals.var_phit1_dc_dn9 * assign46710_e59859) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn9) - ((0.5 * locals.var_temp__blk949_dn9) / assign46710_e59857))))), );
            locals.var_vm_rv = 0.0;
        }

        if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
            let assign46720_e59869: f64 = (locals.var_gco_i * locals.var_phit1_dc);
            (locals.var_dch, locals.var_dch_dn4, locals.var_dch_dn6, locals.var_dch_dn7, locals.var_dch_dn8, locals.var_dch_dn9, ) = (assign46720_e59869, (locals.var_gco_i * locals.var_phit1_dc_dn4), (locals.var_gco_i * locals.var_phit1_dc_dn6), (locals.var_gco_i * locals.var_phit1_dc_dn7), (locals.var_gco_i * locals.var_phit1_dc_dn8), (locals.var_gco_i * locals.var_phit1_dc_dn9), );
            locals.var_dch_rv = 0.0;
        }

        if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
            let assign46730_e59877: f64 = (locals.var_voxm_dc + locals.var_dch);
            (locals.var_arg2mina, locals.var_arg2mina_dn4, locals.var_arg2mina_dn6, locals.var_arg2mina_dn7, locals.var_arg2mina_dn8, locals.var_arg2mina_dn9, ) = (assign46730_e59877, (locals.var_voxm_dc_dn4 + locals.var_dch_dn4), (locals.var_voxm_dc_dn6 + locals.var_dch_dn6), (locals.var_voxm_dc_dn7 + locals.var_dch_dn7), (locals.var_voxm_dc_dn8 + locals.var_dch_dn8), (locals.var_voxm_dc_dn9 + locals.var_dch_dn9), );
            locals.var_arg2mina_rv = 0.0;
        }

        if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
            let assign46740_e59886: f64 = locals.var_arg2mina;
            let assign46740_e59889: f64 = (-locals.var_arg2mina);
            let assign46740_e59892: f64 = (-locals.var_arg2mina);
            let assign46740_e59893: f64 = (assign46740_e59889 * assign46740_e59892);
            let assign46740_e59895: f64 = (assign46740_e59893 + 0.01);
            let assign46740_e59896: f64 = (assign46740_e59895).sqrt();
            let assign46740_e59897: f64 = (assign46740_e59886 - assign46740_e59896);
            let assign46740_e59898: f64 = (0.5 * assign46740_e59897);
            (locals.var_psi_t, locals.var_psi_t_dn4, locals.var_psi_t_dn6, locals.var_psi_t_dn7, locals.var_psi_t_dn8, locals.var_psi_t_dn9, ) = (assign46740_e59898, (0.5 * (locals.var_arg2mina_dn4 - ((((-locals.var_arg2mina_dn4) * assign46740_e59892) + (assign46740_e59889 * (-locals.var_arg2mina_dn4))) / (2.0 * assign46740_e59896)))), (0.5 * (locals.var_arg2mina_dn6 - ((((-locals.var_arg2mina_dn6) * assign46740_e59892) + (assign46740_e59889 * (-locals.var_arg2mina_dn6))) / (2.0 * assign46740_e59896)))), (0.5 * (locals.var_arg2mina_dn7 - ((((-locals.var_arg2mina_dn7) * assign46740_e59892) + (assign46740_e59889 * (-locals.var_arg2mina_dn7))) / (2.0 * assign46740_e59896)))), (0.5 * (locals.var_arg2mina_dn8 - ((((-locals.var_arg2mina_dn8) * assign46740_e59892) + (assign46740_e59889 * (-locals.var_arg2mina_dn8))) / (2.0 * assign46740_e59896)))), (0.5 * (locals.var_arg2mina_dn9 - ((((-locals.var_arg2mina_dn9) * assign46740_e59892) + (assign46740_e59889 * (-locals.var_arg2mina_dn9))) / (2.0 * assign46740_e59896)))), );
            locals.var_psi_t_rv = 0.0;
        }

        if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
            let assign46750_e59906: f64 = (locals.var_voxm_dc * locals.var_voxm_dc);
            let assign46750_e59908: f64 = (assign46750_e59906 + 1e-6);
            let assign46750_e59909: f64 = (assign46750_e59908).sqrt();
            let assign46750_e59911: f64 = (assign46750_e59909 * locals.var_inv_chib);
            (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9, ) = (assign46750_e59911, ((((locals.var_voxm_dc_dn4 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn4)) / (2.0 * assign46750_e59909)) * locals.var_inv_chib), ((((locals.var_voxm_dc_dn6 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn6)) / (2.0 * assign46750_e59909)) * locals.var_inv_chib), ((((locals.var_voxm_dc_dn7 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn7)) / (2.0 * assign46750_e59909)) * locals.var_inv_chib), ((((locals.var_voxm_dc_dn8 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn8)) / (2.0 * assign46750_e59909)) * locals.var_inv_chib), ((((locals.var_voxm_dc_dn9 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn9)) / (2.0 * assign46750_e59909)) * locals.var_inv_chib), );
            locals.var_zg_rv = 0.0;
        }

        let assign46760_e59916: f64 = if locals.var_gc3_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1250 = assign46760_e59916;
        locals.var_guard1250_rv = 0.0;

        if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1250 != 0.0)) {
            let assign46770_e59925: f64 = (locals.var_zg + locals.var_gcq);
            let assign46770_e59928: f64 = (locals.var_zg - locals.var_gcq);
            let assign46770_e59931: f64 = (locals.var_zg - locals.var_gcq);
            let assign46770_e59932: f64 = (assign46770_e59928 * assign46770_e59931);
            let assign46770_e59934: f64 = (assign46770_e59932 + 1e-6);
            let assign46770_e59935: f64 = (assign46770_e59934).sqrt();
            let assign46770_e59936: f64 = (assign46770_e59925 - assign46770_e59935);
            let assign46770_e59937: f64 = (0.5 * assign46770_e59936);
            (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9, ) = (assign46770_e59937, (0.5 * (locals.var_zg_dn4 - (((locals.var_zg_dn4 * assign46770_e59931) + (assign46770_e59928 * locals.var_zg_dn4)) / (2.0 * assign46770_e59935)))), (0.5 * (locals.var_zg_dn6 - (((locals.var_zg_dn6 * assign46770_e59931) + (assign46770_e59928 * locals.var_zg_dn6)) / (2.0 * assign46770_e59935)))), (0.5 * (locals.var_zg_dn7 - (((locals.var_zg_dn7 * assign46770_e59931) + (assign46770_e59928 * locals.var_zg_dn7)) / (2.0 * assign46770_e59935)))), (0.5 * (locals.var_zg_dn8 - (((locals.var_zg_dn8 * assign46770_e59931) + (assign46770_e59928 * locals.var_zg_dn8)) / (2.0 * assign46770_e59935)))), (0.5 * (locals.var_zg_dn9 - (((locals.var_zg_dn9 * assign46770_e59931) + (assign46770_e59928 * locals.var_zg_dn9)) / (2.0 * assign46770_e59935)))), );
            locals.var_zg_rv = 0.0;
        }

        if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
            let assign46780_e59946: f64 = (locals.var_psi_t - locals.var_alpha_b);
            let assign46780_e59948: f64 = (assign46780_e59946 - locals.var_vm);
            let assign46780_e59950: f64 = (assign46780_e59948 * locals.var_inv_phit1_dc);
            let assign46780_e59951: f64 = (locals.var_x_m_dc + assign46780_e59950);
            (locals.var_arg1, locals.var_arg1_dn4, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8, locals.var_arg1_dn9, ) = (assign46780_e59951, (locals.var_x_m_dc_dn4 + ((((locals.var_psi_t_dn4 - locals.var_alpha_b_dn4) - locals.var_vm_dn4) * locals.var_inv_phit1_dc) + (assign46780_e59948 * locals.var_inv_phit1_dc_dn4))), (locals.var_x_m_dc_dn6 + (((locals.var_psi_t_dn6 - locals.var_vm_dn6) * locals.var_inv_phit1_dc) + (assign46780_e59948 * locals.var_inv_phit1_dc_dn6))), (locals.var_x_m_dc_dn7 + (((locals.var_psi_t_dn7 - locals.var_vm_dn7) * locals.var_inv_phit1_dc) + (assign46780_e59948 * locals.var_inv_phit1_dc_dn7))), (locals.var_x_m_dc_dn8 + (((locals.var_psi_t_dn8 - locals.var_vm_dn8) * locals.var_inv_phit1_dc) + (assign46780_e59948 * locals.var_inv_phit1_dc_dn8))), (locals.var_x_m_dc_dn9 + (((locals.var_psi_t_dn9 - locals.var_vm_dn9) * locals.var_inv_phit1_dc) + (assign46780_e59948 * locals.var_inv_phit1_dc_dn9))), );
            locals.var_arg1_rv = 0.0;
        }

        if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
            let assign46840_e60045: f64 = (locals.var_v_gs + locals.var_vsbstar_dc);
            let assign46840_e60047: f64 = (assign46840_e60045 - locals.var_vm);
            let assign46840_e60048: f64 = (-assign46840_e60047);
            let assign46840_e60050: f64 = (assign46840_e60048 * locals.var_inv_phit1_dc);
            (locals.var_arg1, locals.var_arg1_dn4, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8, locals.var_arg1_dn9, ) = (assign46840_e60050, (((-(locals.var_vsbstar_dc_dn4 - locals.var_vm_dn4)) * locals.var_inv_phit1_dc) + (assign46840_e60048 * locals.var_inv_phit1_dc_dn4)), (((-((locals.var_v_gs_dn6 + locals.var_vsbstar_dc_dn6) - locals.var_vm_dn6)) * locals.var_inv_phit1_dc) + (assign46840_e60048 * locals.var_inv_phit1_dc_dn6)), (((-((locals.var_v_gs_dn7 + locals.var_vsbstar_dc_dn7) - locals.var_vm_dn7)) * locals.var_inv_phit1_dc) + (assign46840_e60048 * locals.var_inv_phit1_dc_dn7)), (((-((locals.var_v_gs_dn8 + locals.var_vsbstar_dc_dn8) - locals.var_vm_dn8)) * locals.var_inv_phit1_dc) + (assign46840_e60048 * locals.var_inv_phit1_dc_dn8)), (((-(locals.var_vsbstar_dc_dn9 - locals.var_vm_dn9)) * locals.var_inv_phit1_dc) + (assign46840_e60048 * locals.var_inv_phit1_dc_dn9)), );
            locals.var_arg1_rv = 0.0;
        }

        let assign46850_e60054: f64 = (locals.var_arg1).abs();
        let assign46850_e60056: f64 = if assign46850_e60054 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1253 = assign46850_e60056;
        locals.var_guard1253_rv = 0.0;

        if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1253 != 0.0)) {
            let assign46860_e60063: f64 = (locals.var_arg1).exp();
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign46860_e60063, (assign46860_e60063 * locals.var_arg1_dn4), (assign46860_e60063 * locals.var_arg1_dn6), (assign46860_e60063 * locals.var_arg1_dn7), (assign46860_e60063 * locals.var_arg1_dn8), (assign46860_e60063 * locals.var_arg1_dn9), );
            locals.var_temp__blk949_rv = 0.0;
        }

        let assign46870_e60068: f64 = if locals.var_arg1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1254 = assign46870_e60068;
        locals.var_guard1254_rv = 0.0;

        if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1254 != 0.0)) {
            let assign46880_e60080: f64 = (-230.25850929940458);
            let assign46880_e60082: f64 = (assign46880_e60080 - locals.var_arg1);
            let assign46880_e60086: f64 = (-230.25850929940458);
            let assign46880_e60088: f64 = (assign46880_e60086 - locals.var_arg1);
            let assign46880_e60091: f64 = (-230.25850929940458);
            let assign46880_e60093: f64 = (assign46880_e60091 - locals.var_arg1);
            let assign46880_e60095: f64 = (assign46880_e60093 * 0.3333333333333333);
            let assign46880_e60096: f64 = (1.0 + assign46880_e60095);
            let assign46880_e60097: f64 = (assign46880_e60088 * assign46880_e60096);
            let assign46880_e60098: f64 = (0.5 * assign46880_e60097);
            let assign46880_e60099: f64 = (1.0 + assign46880_e60098);
            let assign46880_e60100: f64 = (assign46880_e60082 * assign46880_e60099);
            let assign46880_e60101: f64 = (1.0 + assign46880_e60100);
            let assign46880_e60102: f64 = (1e-100 / assign46880_e60101);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign46880_e60102, (-((1e-100 * (((-locals.var_arg1_dn4) * assign46880_e60099) + (assign46880_e60082 * (0.5 * (((-locals.var_arg1_dn4) * assign46880_e60096) + (assign46880_e60088 * ((-locals.var_arg1_dn4) * 0.3333333333333333))))))) / (assign46880_e60101 * assign46880_e60101))), (-((1e-100 * (((-locals.var_arg1_dn6) * assign46880_e60099) + (assign46880_e60082 * (0.5 * (((-locals.var_arg1_dn6) * assign46880_e60096) + (assign46880_e60088 * ((-locals.var_arg1_dn6) * 0.3333333333333333))))))) / (assign46880_e60101 * assign46880_e60101))), (-((1e-100 * (((-locals.var_arg1_dn7) * assign46880_e60099) + (assign46880_e60082 * (0.5 * (((-locals.var_arg1_dn7) * assign46880_e60096) + (assign46880_e60088 * ((-locals.var_arg1_dn7) * 0.3333333333333333))))))) / (assign46880_e60101 * assign46880_e60101))), (-((1e-100 * (((-locals.var_arg1_dn8) * assign46880_e60099) + (assign46880_e60082 * (0.5 * (((-locals.var_arg1_dn8) * assign46880_e60096) + (assign46880_e60088 * ((-locals.var_arg1_dn8) * 0.3333333333333333))))))) / (assign46880_e60101 * assign46880_e60101))), (-((1e-100 * (((-locals.var_arg1_dn9) * assign46880_e60099) + (assign46880_e60082 * (0.5 * (((-locals.var_arg1_dn9) * assign46880_e60096) + (assign46880_e60088 * ((-locals.var_arg1_dn9) * 0.3333333333333333))))))) / (assign46880_e60101 * assign46880_e60101))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1254 == 0.0)) {
            let assign46890_e60118: f64 = (locals.var_arg1 - 230.25850929940458);
            let assign46890_e60123: f64 = (locals.var_arg1 - 230.25850929940458);
            let assign46890_e60127: f64 = (locals.var_arg1 - 230.25850929940458);
            let assign46890_e60129: f64 = (assign46890_e60127 * 0.3333333333333333);
            let assign46890_e60130: f64 = (1.0 + assign46890_e60129);
            let assign46890_e60131: f64 = (assign46890_e60123 * assign46890_e60130);
            let assign46890_e60132: f64 = (0.5 * assign46890_e60131);
            let assign46890_e60133: f64 = (1.0 + assign46890_e60132);
            let assign46890_e60134: f64 = (assign46890_e60118 * assign46890_e60133);
            let assign46890_e60135: f64 = (1.0 + assign46890_e60134);
            let assign46890_e60136: f64 = (1e100 * assign46890_e60135);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign46890_e60136, (1e100 * ((locals.var_arg1_dn4 * assign46890_e60133) + (assign46890_e60118 * (0.5 * ((locals.var_arg1_dn4 * assign46890_e60130) + (assign46890_e60123 * (locals.var_arg1_dn4 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn6 * assign46890_e60133) + (assign46890_e60118 * (0.5 * ((locals.var_arg1_dn6 * assign46890_e60130) + (assign46890_e60123 * (locals.var_arg1_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn7 * assign46890_e60133) + (assign46890_e60118 * (0.5 * ((locals.var_arg1_dn7 * assign46890_e60130) + (assign46890_e60123 * (locals.var_arg1_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn8 * assign46890_e60133) + (assign46890_e60118 * (0.5 * ((locals.var_arg1_dn8 * assign46890_e60130) + (assign46890_e60123 * (locals.var_arg1_dn8 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn9 * assign46890_e60133) + (assign46890_e60118 * (0.5 * ((locals.var_arg1_dn9 * assign46890_e60130) + (assign46890_e60123 * (locals.var_arg1_dn9 * 0.3333333333333333))))))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
            let assign46910_e60152: f64 = (-1.5);
            let assign46910_e60157: f64 = (locals.var_gc3_i * locals.var_zg);
            let assign46910_e60158: f64 = (locals.var_gc2_i + assign46910_e60157);
            let assign46910_e60159: f64 = (locals.var_zg * assign46910_e60158);
            let assign46910_e60160: f64 = (assign46910_e60152 + assign46910_e60159);
            let assign46910_e60161: f64 = (locals.var_bch * assign46910_e60160);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign46910_e60161, (locals.var_bch * ((locals.var_zg_dn4 * assign46910_e60158) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn4)))), (locals.var_bch * ((locals.var_zg_dn6 * assign46910_e60158) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn6)))), (locals.var_bch * ((locals.var_zg_dn7 * assign46910_e60158) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn7)))), (locals.var_bch * ((locals.var_zg_dn8 * assign46910_e60158) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn8)))), (locals.var_bch * ((locals.var_zg_dn9 * assign46910_e60158) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn9)))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        let assign46980_e60269: f64 = if ((locals.var_xg_dc <= 0.0) || ((locals.var_gc2_i == 0.0) && (locals.var_gc3_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1257 = assign46980_e60269;
        locals.var_guard1257_rv = 0.0;

        if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) {
            let assign47010_e60295: f64 = (2.0 * locals.var_gc3_i);
            let assign47010_e60297: f64 = (assign47010_e60295 * locals.var_zg);
            let assign47010_e60298: f64 = (locals.var_gc2_i + assign47010_e60297);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign47010_e60298, (assign47010_e60295 * locals.var_zg_dn4), (assign47010_e60295 * locals.var_zg_dn6), (assign47010_e60295 * locals.var_zg_dn7), (assign47010_e60295 * locals.var_zg_dn8), (assign47010_e60295 * locals.var_zg_dn9), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) {
            let assign47020_e60310: f64 = (locals.var_temp__blk949 * locals.var_bch);
            let assign47020_e60311: f64 = (locals.var_chib_i / assign47020_e60310);
            (locals.var_u0, locals.var_u0_dn4, locals.var_u0_dn6, locals.var_u0_dn7, locals.var_u0_dn8, locals.var_u0_dn9, ) = (assign47020_e60311, (-((locals.var_chib_i * (locals.var_temp__blk949_dn4 * locals.var_bch)) / (assign47020_e60310 * assign47020_e60310))), (-((locals.var_chib_i * (locals.var_temp__blk949_dn6 * locals.var_bch)) / (assign47020_e60310 * assign47020_e60310))), (-((locals.var_chib_i * (locals.var_temp__blk949_dn7 * locals.var_bch)) / (assign47020_e60310 * assign47020_e60310))), (-((locals.var_chib_i * (locals.var_temp__blk949_dn8 * locals.var_bch)) / (assign47020_e60310 * assign47020_e60310))), (-((locals.var_chib_i * (locals.var_temp__blk949_dn9 * locals.var_bch)) / (assign47020_e60310 * assign47020_e60310))), );
            locals.var_u0_rv = 0.0;
        }

        if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) {
            let assign47030_e60323: f64 = (locals.var_dps_dc / locals.var_u0);
            let assign47030_e60324: f64 = (0.5 * assign47030_e60323);
            (locals.var_x, locals.var_x_dn4, locals.var_x_dn6, locals.var_x_dn7, locals.var_x_dn8, locals.var_x_dn9, ) = (assign47030_e60324, (0.5 * (((locals.var_dps_dc_dn4 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn4)) / (locals.var_u0 * locals.var_u0))), (0.5 * (((locals.var_dps_dc_dn6 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn6)) / (locals.var_u0 * locals.var_u0))), (0.5 * (((locals.var_dps_dc_dn7 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn7)) / (locals.var_u0 * locals.var_u0))), (0.5 * (((locals.var_dps_dc_dn8 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn8)) / (locals.var_u0 * locals.var_u0))), (0.5 * (((locals.var_dps_dc_dn9 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn9)) / (locals.var_u0 * locals.var_u0))), );
            locals.var_x_rv = 0.0;
        }

        let assign47070_e60368: f64 = if locals.var_x < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard1258 = assign47070_e60368;
        locals.var_guard1258_rv = 0.0;

        let assign47120_e60461: f64 = (locals.var_x).abs();
        let assign47120_e60463: f64 = if assign47120_e60461 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1259 = assign47120_e60463;
        locals.var_guard1259_rv = 0.0;

        if (((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) && (locals.var_guard1258 == 0.0)) && (locals.var_guard1259 != 0.0)) {
            let assign47130_e60476: f64 = (locals.var_x).exp();
            (locals.var_ex, locals.var_ex_dn4, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8, locals.var_ex_dn9, ) = (assign47130_e60476, (assign47130_e60476 * locals.var_x_dn4), (assign47130_e60476 * locals.var_x_dn6), (assign47130_e60476 * locals.var_x_dn7), (assign47130_e60476 * locals.var_x_dn8), (assign47130_e60476 * locals.var_x_dn9), );
            locals.var_ex_rv = 0.0;
        }

        let assign47140_e60481: f64 = if locals.var_x < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1260 = assign47140_e60481;
        locals.var_guard1260_rv = 0.0;

        if ((((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) && (locals.var_guard1258 == 0.0)) && (locals.var_guard1259 == 0.0)) && (locals.var_guard1260 != 0.0)) {
            let assign47150_e60499: f64 = (-230.25850929940458);
            let assign47150_e60501: f64 = (assign47150_e60499 - locals.var_x);
            let assign47150_e60505: f64 = (-230.25850929940458);
            let assign47150_e60507: f64 = (assign47150_e60505 - locals.var_x);
            let assign47150_e60510: f64 = (-230.25850929940458);
            let assign47150_e60512: f64 = (assign47150_e60510 - locals.var_x);
            let assign47150_e60514: f64 = (assign47150_e60512 * 0.3333333333333333);
            let assign47150_e60515: f64 = (1.0 + assign47150_e60514);
            let assign47150_e60516: f64 = (assign47150_e60507 * assign47150_e60515);
            let assign47150_e60517: f64 = (0.5 * assign47150_e60516);
            let assign47150_e60518: f64 = (1.0 + assign47150_e60517);
            let assign47150_e60519: f64 = (assign47150_e60501 * assign47150_e60518);
            let assign47150_e60520: f64 = (1.0 + assign47150_e60519);
            let assign47150_e60521: f64 = (1e-100 / assign47150_e60520);
            (locals.var_ex, locals.var_ex_dn4, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8, locals.var_ex_dn9, ) = (assign47150_e60521, (-((1e-100 * (((-locals.var_x_dn4) * assign47150_e60518) + (assign47150_e60501 * (0.5 * (((-locals.var_x_dn4) * assign47150_e60515) + (assign47150_e60507 * ((-locals.var_x_dn4) * 0.3333333333333333))))))) / (assign47150_e60520 * assign47150_e60520))), (-((1e-100 * (((-locals.var_x_dn6) * assign47150_e60518) + (assign47150_e60501 * (0.5 * (((-locals.var_x_dn6) * assign47150_e60515) + (assign47150_e60507 * ((-locals.var_x_dn6) * 0.3333333333333333))))))) / (assign47150_e60520 * assign47150_e60520))), (-((1e-100 * (((-locals.var_x_dn7) * assign47150_e60518) + (assign47150_e60501 * (0.5 * (((-locals.var_x_dn7) * assign47150_e60515) + (assign47150_e60507 * ((-locals.var_x_dn7) * 0.3333333333333333))))))) / (assign47150_e60520 * assign47150_e60520))), (-((1e-100 * (((-locals.var_x_dn8) * assign47150_e60518) + (assign47150_e60501 * (0.5 * (((-locals.var_x_dn8) * assign47150_e60515) + (assign47150_e60507 * ((-locals.var_x_dn8) * 0.3333333333333333))))))) / (assign47150_e60520 * assign47150_e60520))), (-((1e-100 * (((-locals.var_x_dn9) * assign47150_e60518) + (assign47150_e60501 * (0.5 * (((-locals.var_x_dn9) * assign47150_e60515) + (assign47150_e60507 * ((-locals.var_x_dn9) * 0.3333333333333333))))))) / (assign47150_e60520 * assign47150_e60520))), );
            locals.var_ex_rv = 0.0;
        }

        if ((((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) && (locals.var_guard1258 == 0.0)) && (locals.var_guard1259 == 0.0)) && (locals.var_guard1260 == 0.0)) {
            let assign47160_e60543: f64 = (locals.var_x - 230.25850929940458);
            let assign47160_e60548: f64 = (locals.var_x - 230.25850929940458);
            let assign47160_e60552: f64 = (locals.var_x - 230.25850929940458);
            let assign47160_e60554: f64 = (assign47160_e60552 * 0.3333333333333333);
            let assign47160_e60555: f64 = (1.0 + assign47160_e60554);
            let assign47160_e60556: f64 = (assign47160_e60548 * assign47160_e60555);
            let assign47160_e60557: f64 = (0.5 * assign47160_e60556);
            let assign47160_e60558: f64 = (1.0 + assign47160_e60557);
            let assign47160_e60559: f64 = (assign47160_e60543 * assign47160_e60558);
            let assign47160_e60560: f64 = (1.0 + assign47160_e60559);
            let assign47160_e60561: f64 = (1e100 * assign47160_e60560);
            (locals.var_ex, locals.var_ex_dn4, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8, locals.var_ex_dn9, ) = (assign47160_e60561, (1e100 * ((locals.var_x_dn4 * assign47160_e60558) + (assign47160_e60543 * (0.5 * ((locals.var_x_dn4 * assign47160_e60555) + (assign47160_e60548 * (locals.var_x_dn4 * 0.3333333333333333))))))), (1e100 * ((locals.var_x_dn6 * assign47160_e60558) + (assign47160_e60543 * (0.5 * ((locals.var_x_dn6 * assign47160_e60555) + (assign47160_e60548 * (locals.var_x_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_x_dn7 * assign47160_e60558) + (assign47160_e60543 * (0.5 * ((locals.var_x_dn7 * assign47160_e60555) + (assign47160_e60548 * (locals.var_x_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_x_dn8 * assign47160_e60558) + (assign47160_e60543 * (0.5 * ((locals.var_x_dn8 * assign47160_e60555) + (assign47160_e60548 * (locals.var_x_dn8 * 0.3333333333333333))))))), (1e100 * ((locals.var_x_dn9 * assign47160_e60558) + (assign47160_e60543 * (0.5 * ((locals.var_x_dn9 * assign47160_e60555) + (assign47160_e60548 * (locals.var_x_dn9 * 0.3333333333333333))))))), );
            locals.var_ex_rv = 0.0;
        }

        if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) && (locals.var_guard1258 == 0.0)) {
            let assign47170_e60575: f64 = (1.0 / locals.var_ex);
            (locals.var_inv_ex, locals.var_inv_ex_dn4, locals.var_inv_ex_dn6, locals.var_inv_ex_dn7, locals.var_inv_ex_dn8, locals.var_inv_ex_dn9, ) = (assign47170_e60575, (-(locals.var_ex_dn4 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn6 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn7 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn8 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn9 / (locals.var_ex * locals.var_ex))), );
            locals.var_inv_ex_rv = 0.0;
        }

        if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) && (locals.var_guard1258 == 0.0)) {
            let assign47180_e60589: f64 = (locals.var_ex - locals.var_inv_ex);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign47180_e60589, (locals.var_ex_dn4 - locals.var_inv_ex_dn4), (locals.var_ex_dn6 - locals.var_inv_ex_dn6), (locals.var_ex_dn7 - locals.var_inv_ex_dn7), (locals.var_ex_dn8 - locals.var_inv_ex_dn8), (locals.var_ex_dn9 - locals.var_inv_ex_dn9), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) && (locals.var_guard1258 == 0.0)) {
            let assign47190_e60603: f64 = (locals.var_ex + locals.var_inv_ex);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign47190_e60603, (locals.var_ex_dn4 + locals.var_inv_ex_dn4), (locals.var_ex_dn6 + locals.var_inv_ex_dn6), (locals.var_ex_dn7 + locals.var_inv_ex_dn7), (locals.var_ex_dn8 + locals.var_inv_ex_dn8), (locals.var_ex_dn9 + locals.var_inv_ex_dn9), );
            locals.var_temp2_rv = 0.0;
        }

        let assign47290_e60721: f64 = if p.p42 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1261 = assign47290_e60721;
        locals.var_guard1261_rv = 0.0;

        let assign47300_e60728: f64 = if ((locals.var_agidld_i > 0.0) && (locals.var_vovd < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1262 = assign47300_e60728;
        locals.var_guard1262_rv = 0.0;

        if ((locals.var_guard1261 != 0.0) && (locals.var_guard1262 != 0.0)) {
            let assign47310_e60734: f64 = (locals.var_vovd * locals.var_vovd);
            let assign47310_e60737: f64 = (locals.var_cgidld_i * locals.var_cgidld_i);
            let assign47310_e60740: f64 = (locals.var_vdbprime * locals.var_vdbprime);
            let assign47310_e60741: f64 = (assign47310_e60737 * assign47310_e60740);
            let assign47310_e60742: f64 = (assign47310_e60734 + assign47310_e60741);
            let assign47310_e60744: f64 = (assign47310_e60742 + 1e-6);
            let assign47310_e60745: f64 = (assign47310_e60744).sqrt();
            (locals.var_vtovd, locals.var_vtovd_dn6, locals.var_vtovd_dn7, locals.var_vtovd_dn8, locals.var_vtovd_dn9, ) = (assign47310_e60745, (((locals.var_vovd_dn6 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn6)) / (2.0 * assign47310_e60745)), ((((locals.var_vovd_dn7 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn7)) + (assign47310_e60737 * ((locals.var_vdbprime_dn7 * locals.var_vdbprime) + (locals.var_vdbprime * locals.var_vdbprime_dn7)))) / (2.0 * assign47310_e60745)), ((((locals.var_vovd_dn8 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn8)) + (assign47310_e60737 * ((locals.var_vdbprime_dn8 * locals.var_vdbprime) + (locals.var_vdbprime * locals.var_vdbprime_dn8)))) / (2.0 * assign47310_e60745)), ((assign47310_e60737 * ((locals.var_vdbprime_dn9 * locals.var_vdbprime) + (locals.var_vdbprime * locals.var_vdbprime_dn9))) / (2.0 * assign47310_e60745)), );
            locals.var_vtovd_rv = 0.0;
        }

        if ((locals.var_guard1261 != 0.0) && (locals.var_guard1262 != 0.0)) {
            let assign47320_e60752: f64 = (-locals.var_bgidlds);
            let assign47320_e60754: f64 = (assign47320_e60752 / locals.var_vtovd);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign47320_e60754, 0.0, (-((assign47320_e60752 * locals.var_vtovd_dn6) / (locals.var_vtovd * locals.var_vtovd))), (-((assign47320_e60752 * locals.var_vtovd_dn7) / (locals.var_vtovd * locals.var_vtovd))), (-((assign47320_e60752 * locals.var_vtovd_dn8) / (locals.var_vtovd * locals.var_vtovd))), (-((assign47320_e60752 * locals.var_vtovd_dn9) / (locals.var_vtovd * locals.var_vtovd))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        let assign47330_e60759: f64 = (-230.25850929940458);
        let assign47330_e60760: f64 = if locals.var_temp__blk949 > assign47330_e60759 { 1.0 } else { 0.0 };
        locals.var_guard1263 = assign47330_e60760;
        locals.var_guard1263_rv = 0.0;

        if (((locals.var_guard1261 != 0.0) && (locals.var_guard1262 != 0.0)) && (locals.var_guard1263 != 0.0)) {
            let assign47340_e60767: f64 = (locals.var_temp__blk949).exp();
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign47340_e60767, (assign47340_e60767 * locals.var_temp__blk949_dn4), (assign47340_e60767 * locals.var_temp__blk949_dn6), (assign47340_e60767 * locals.var_temp__blk949_dn7), (assign47340_e60767 * locals.var_temp__blk949_dn8), (assign47340_e60767 * locals.var_temp__blk949_dn9), );
            locals.var_temp2_rv = 0.0;
        }

        if (((locals.var_guard1261 != 0.0) && (locals.var_guard1262 != 0.0)) && (locals.var_guard1263 == 0.0)) {
            let assign47350_e60779: f64 = (-230.25850929940458);
            let assign47350_e60781: f64 = (assign47350_e60779 - locals.var_temp__blk949);
            let assign47350_e60785: f64 = (-230.25850929940458);
            let assign47350_e60787: f64 = (assign47350_e60785 - locals.var_temp__blk949);
            let assign47350_e60790: f64 = (-230.25850929940458);
            let assign47350_e60792: f64 = (assign47350_e60790 - locals.var_temp__blk949);
            let assign47350_e60794: f64 = (assign47350_e60792 * 0.3333333333333333);
            let assign47350_e60795: f64 = (1.0 + assign47350_e60794);
            let assign47350_e60796: f64 = (assign47350_e60787 * assign47350_e60795);
            let assign47350_e60797: f64 = (0.5 * assign47350_e60796);
            let assign47350_e60798: f64 = (1.0 + assign47350_e60797);
            let assign47350_e60799: f64 = (assign47350_e60781 * assign47350_e60798);
            let assign47350_e60800: f64 = (1.0 + assign47350_e60799);
            let assign47350_e60801: f64 = (1e-100 / assign47350_e60800);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign47350_e60801, (-((1e-100 * (((-locals.var_temp__blk949_dn4) * assign47350_e60798) + (assign47350_e60781 * (0.5 * (((-locals.var_temp__blk949_dn4) * assign47350_e60795) + (assign47350_e60787 * ((-locals.var_temp__blk949_dn4) * 0.3333333333333333))))))) / (assign47350_e60800 * assign47350_e60800))), (-((1e-100 * (((-locals.var_temp__blk949_dn6) * assign47350_e60798) + (assign47350_e60781 * (0.5 * (((-locals.var_temp__blk949_dn6) * assign47350_e60795) + (assign47350_e60787 * ((-locals.var_temp__blk949_dn6) * 0.3333333333333333))))))) / (assign47350_e60800 * assign47350_e60800))), (-((1e-100 * (((-locals.var_temp__blk949_dn7) * assign47350_e60798) + (assign47350_e60781 * (0.5 * (((-locals.var_temp__blk949_dn7) * assign47350_e60795) + (assign47350_e60787 * ((-locals.var_temp__blk949_dn7) * 0.3333333333333333))))))) / (assign47350_e60800 * assign47350_e60800))), (-((1e-100 * (((-locals.var_temp__blk949_dn8) * assign47350_e60798) + (assign47350_e60781 * (0.5 * (((-locals.var_temp__blk949_dn8) * assign47350_e60795) + (assign47350_e60787 * ((-locals.var_temp__blk949_dn8) * 0.3333333333333333))))))) / (assign47350_e60800 * assign47350_e60800))), (-((1e-100 * (((-locals.var_temp__blk949_dn9) * assign47350_e60798) + (assign47350_e60781 * (0.5 * (((-locals.var_temp__blk949_dn9) * assign47350_e60795) + (assign47350_e60787 * ((-locals.var_temp__blk949_dn9) * 0.3333333333333333))))))) / (assign47350_e60800 * assign47350_e60800))), );
            locals.var_temp2_rv = 0.0;
        }

        let assign47370_e60825: f64 = if ((locals.var_agidl_i > 0.0) && (locals.var_vovs < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1264 = assign47370_e60825;
        locals.var_guard1264_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_20(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard1261 != 0.0) && (locals.var_guard1264 != 0.0)) {
            let assign47380_e60831: f64 = (locals.var_vovs * locals.var_vovs);
            let assign47380_e60834: f64 = (locals.var_cgidl_i * locals.var_cgidl_i);
            let assign47380_e60837: f64 = (locals.var_vsbprime * locals.var_vsbprime);
            let assign47380_e60838: f64 = (assign47380_e60834 * assign47380_e60837);
            let assign47380_e60839: f64 = (assign47380_e60831 + assign47380_e60838);
            let assign47380_e60841: f64 = (assign47380_e60839 + 1e-6);
            let assign47380_e60842: f64 = (assign47380_e60841).sqrt();
            (locals.var_vtovs, locals.var_vtovs_dn6, locals.var_vtovs_dn7, locals.var_vtovs_dn8, locals.var_vtovs_dn9, ) = (assign47380_e60842, (((locals.var_vovs_dn6 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn6)) / (2.0 * assign47380_e60842)), ((((locals.var_vovs_dn7 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn7)) + (assign47380_e60834 * ((locals.var_vsbprime_dn7 * locals.var_vsbprime) + (locals.var_vsbprime * locals.var_vsbprime_dn7)))) / (2.0 * assign47380_e60842)), ((((locals.var_vovs_dn8 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn8)) + (assign47380_e60834 * ((locals.var_vsbprime_dn8 * locals.var_vsbprime) + (locals.var_vsbprime * locals.var_vsbprime_dn8)))) / (2.0 * assign47380_e60842)), ((assign47380_e60834 * ((locals.var_vsbprime_dn9 * locals.var_vsbprime) + (locals.var_vsbprime * locals.var_vsbprime_dn9))) / (2.0 * assign47380_e60842)), );
            locals.var_vtovs_rv = 0.0;
        }

        if ((locals.var_guard1261 != 0.0) && (locals.var_guard1264 != 0.0)) {
            let assign47390_e60849: f64 = (-locals.var_bgidls);
            let assign47390_e60851: f64 = (assign47390_e60849 / locals.var_vtovs);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign47390_e60851, 0.0, (-((assign47390_e60849 * locals.var_vtovs_dn6) / (locals.var_vtovs * locals.var_vtovs))), (-((assign47390_e60849 * locals.var_vtovs_dn7) / (locals.var_vtovs * locals.var_vtovs))), (-((assign47390_e60849 * locals.var_vtovs_dn8) / (locals.var_vtovs * locals.var_vtovs))), (-((assign47390_e60849 * locals.var_vtovs_dn9) / (locals.var_vtovs * locals.var_vtovs))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        let assign47400_e60856: f64 = (-230.25850929940458);
        let assign47400_e60857: f64 = if locals.var_temp__blk949 > assign47400_e60856 { 1.0 } else { 0.0 };
        locals.var_guard1265 = assign47400_e60857;
        locals.var_guard1265_rv = 0.0;

        if (((locals.var_guard1261 != 0.0) && (locals.var_guard1264 != 0.0)) && (locals.var_guard1265 != 0.0)) {
            let assign47410_e60864: f64 = (locals.var_temp__blk949).exp();
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign47410_e60864, (assign47410_e60864 * locals.var_temp__blk949_dn4), (assign47410_e60864 * locals.var_temp__blk949_dn6), (assign47410_e60864 * locals.var_temp__blk949_dn7), (assign47410_e60864 * locals.var_temp__blk949_dn8), (assign47410_e60864 * locals.var_temp__blk949_dn9), );
            locals.var_temp2_rv = 0.0;
        }

        if (((locals.var_guard1261 != 0.0) && (locals.var_guard1264 != 0.0)) && (locals.var_guard1265 == 0.0)) {
            let assign47420_e60876: f64 = (-230.25850929940458);
            let assign47420_e60878: f64 = (assign47420_e60876 - locals.var_temp__blk949);
            let assign47420_e60882: f64 = (-230.25850929940458);
            let assign47420_e60884: f64 = (assign47420_e60882 - locals.var_temp__blk949);
            let assign47420_e60887: f64 = (-230.25850929940458);
            let assign47420_e60889: f64 = (assign47420_e60887 - locals.var_temp__blk949);
            let assign47420_e60891: f64 = (assign47420_e60889 * 0.3333333333333333);
            let assign47420_e60892: f64 = (1.0 + assign47420_e60891);
            let assign47420_e60893: f64 = (assign47420_e60884 * assign47420_e60892);
            let assign47420_e60894: f64 = (0.5 * assign47420_e60893);
            let assign47420_e60895: f64 = (1.0 + assign47420_e60894);
            let assign47420_e60896: f64 = (assign47420_e60878 * assign47420_e60895);
            let assign47420_e60897: f64 = (1.0 + assign47420_e60896);
            let assign47420_e60898: f64 = (1e-100 / assign47420_e60897);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign47420_e60898, (-((1e-100 * (((-locals.var_temp__blk949_dn4) * assign47420_e60895) + (assign47420_e60878 * (0.5 * (((-locals.var_temp__blk949_dn4) * assign47420_e60892) + (assign47420_e60884 * ((-locals.var_temp__blk949_dn4) * 0.3333333333333333))))))) / (assign47420_e60897 * assign47420_e60897))), (-((1e-100 * (((-locals.var_temp__blk949_dn6) * assign47420_e60895) + (assign47420_e60878 * (0.5 * (((-locals.var_temp__blk949_dn6) * assign47420_e60892) + (assign47420_e60884 * ((-locals.var_temp__blk949_dn6) * 0.3333333333333333))))))) / (assign47420_e60897 * assign47420_e60897))), (-((1e-100 * (((-locals.var_temp__blk949_dn7) * assign47420_e60895) + (assign47420_e60878 * (0.5 * (((-locals.var_temp__blk949_dn7) * assign47420_e60892) + (assign47420_e60884 * ((-locals.var_temp__blk949_dn7) * 0.3333333333333333))))))) / (assign47420_e60897 * assign47420_e60897))), (-((1e-100 * (((-locals.var_temp__blk949_dn8) * assign47420_e60895) + (assign47420_e60878 * (0.5 * (((-locals.var_temp__blk949_dn8) * assign47420_e60892) + (assign47420_e60884 * ((-locals.var_temp__blk949_dn8) * 0.3333333333333333))))))) / (assign47420_e60897 * assign47420_e60897))), (-((1e-100 * (((-locals.var_temp__blk949_dn9) * assign47420_e60895) + (assign47420_e60878 * (0.5 * (((-locals.var_temp__blk949_dn9) * assign47420_e60892) + (assign47420_e60884 * ((-locals.var_temp__blk949_dn9) * 0.3333333333333333))))))) / (assign47420_e60897 * assign47420_e60897))), );
            locals.var_temp2_rv = 0.0;
        }

        (locals.var_phit1edge, locals.var_phit1edge_dn4, locals.var_phit1edge_dn6, locals.var_phit1edge_dn7, locals.var_phit1edge_dn8, locals.var_phit1edge_dn9, ) = (locals.var_phit, locals.var_phit_dn4, 0.0, 0.0, 0.0, 0.0, );
        locals.var_phit1edge_rv = 0.0;

        (locals.var_xgedge, locals.var_xgedge_dn4, locals.var_xgedge_dn6, locals.var_xgedge_dn7, locals.var_xgedge_dn8, locals.var_xgedge_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_xgedge_rv = 0.0;

        (locals.var_qdseffedge, locals.var_qdseffedge_dn4, locals.var_qdseffedge_dn6, locals.var_qdseffedge_dn7, locals.var_qdseffedge_dn8, locals.var_qdseffedge_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_qdseffedge_rv = 0.0;

        (locals.var_qmeffedge, locals.var_qmeffedge_dn4, locals.var_qmeffedge_dn6, locals.var_qmeffedge_dn7, locals.var_qmeffedge_dn8, locals.var_qmeffedge_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_qmeffedge_rv = 0.0;

        (locals.var_dsqredge, locals.var_dsqredge_dn4, locals.var_dsqredge_dn6, locals.var_dsqredge_dn7, locals.var_dsqredge_dn8, locals.var_dsqredge_dn9, ) = (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_dsqredge_rv = 0.0;

        (locals.var_alphabmedge, locals.var_alphabmedge_dn4, locals.var_alphabmedge_dn6, locals.var_alphabmedge_dn7, locals.var_alphabmedge_dn8, locals.var_alphabmedge_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_alphabmedge_rv = 0.0;

        (locals.var_i_dsedge, locals.var_i_dsedge_dn4, locals.var_i_dsedge_dn6, locals.var_i_dsedge_dn7, locals.var_i_dsedge_dn8, locals.var_i_dsedge_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_i_dsedge_rv = 0.0;

        let assign47510_e60929: f64 = if ((p.p46 != 0.0) && (locals.var_betnedge_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1266 = assign47510_e60929;
        locals.var_guard1266_rv = 0.0;

        if (locals.var_guard1266 != 0.0) {
            let assign47520_e60934: f64 = (locals.var_v_db + locals.var_v_sb);
            let assign47520_e60937: f64 = (locals.var_v_db - locals.var_v_sb);
            let assign47520_e60940: f64 = (locals.var_v_db - locals.var_v_sb);
            let assign47520_e60941: f64 = (assign47520_e60937 * assign47520_e60940);
            let assign47520_e60943: f64 = (assign47520_e60941 + locals.var_bphiedge);
            let assign47520_e60944: f64 = (assign47520_e60943).sqrt();
            let assign47520_e60945: f64 = (assign47520_e60934 - assign47520_e60944);
            let assign47520_e60946: f64 = (0.5 * assign47520_e60945);
            let assign47520_e60948: f64 = (assign47520_e60946 + locals.var_phixedge);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign47520_e60948, ((0.5 * (-(locals.var_bphiedge_dn4 / (2.0 * assign47520_e60944)))) + locals.var_phixedge_dn4), 0.0, (0.5 * ((locals.var_v_db_dn7 + locals.var_v_sb_dn7) - ((((locals.var_v_db_dn7 - locals.var_v_sb_dn7) * assign47520_e60940) + (assign47520_e60937 * (locals.var_v_db_dn7 - locals.var_v_sb_dn7))) / (2.0 * assign47520_e60944)))), (0.5 * ((locals.var_v_db_dn8 + locals.var_v_sb_dn8) - ((((locals.var_v_db_dn8 - locals.var_v_sb_dn8) * assign47520_e60940) + (assign47520_e60937 * (locals.var_v_db_dn8 - locals.var_v_sb_dn8))) / (2.0 * assign47520_e60944)))), (0.5 * ((locals.var_v_db_dn9 + locals.var_v_sb_dn9) - ((((locals.var_v_db_dn9 - locals.var_v_sb_dn9) * assign47520_e60940) + (assign47520_e60937 * (locals.var_v_db_dn9 - locals.var_v_sb_dn9))) / (2.0 * assign47520_e60944)))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if (locals.var_guard1266 != 0.0) {
            let assign47530_e60956: f64 = locals.var_temp__blk949;
            let assign47530_e60959: f64 = locals.var_temp__blk949;
            let assign47530_e60962: f64 = locals.var_temp__blk949;
            let assign47530_e60963: f64 = (assign47530_e60959 * assign47530_e60962);
            let assign47530_e60965: f64 = (assign47530_e60963 + locals.var_aphiedge);
            let assign47530_e60966: f64 = (assign47530_e60965).sqrt();
            let assign47530_e60967: f64 = (assign47530_e60956 - assign47530_e60966);
            let assign47530_e60968: f64 = (0.5 * assign47530_e60967);
            let assign47530_e60969: f64 = (locals.var_v_sb - assign47530_e60968);
            let assign47530_e60971: f64 = (assign47530_e60969 + locals.var_phix1edge);
            (locals.var_vsbstaredge, locals.var_vsbstaredge_dn4, locals.var_vsbstaredge_dn6, locals.var_vsbstaredge_dn7, locals.var_vsbstaredge_dn8, locals.var_vsbstaredge_dn9, ) = (assign47530_e60971, ((-(0.5 * (locals.var_temp__blk949_dn4 - ((((locals.var_temp__blk949_dn4 * assign47530_e60962) + (assign47530_e60959 * locals.var_temp__blk949_dn4)) + locals.var_aphiedge_dn4) / (2.0 * assign47530_e60966))))) + locals.var_phix1edge_dn4), (-(0.5 * (locals.var_temp__blk949_dn6 - (((locals.var_temp__blk949_dn6 * assign47530_e60962) + (assign47530_e60959 * locals.var_temp__blk949_dn6)) / (2.0 * assign47530_e60966))))), (locals.var_v_sb_dn7 - (0.5 * (locals.var_temp__blk949_dn7 - (((locals.var_temp__blk949_dn7 * assign47530_e60962) + (assign47530_e60959 * locals.var_temp__blk949_dn7)) / (2.0 * assign47530_e60966))))), (locals.var_v_sb_dn8 - (0.5 * (locals.var_temp__blk949_dn8 - (((locals.var_temp__blk949_dn8 * assign47530_e60962) + (assign47530_e60959 * locals.var_temp__blk949_dn8)) / (2.0 * assign47530_e60966))))), (locals.var_v_sb_dn9 - (0.5 * (locals.var_temp__blk949_dn9 - (((locals.var_temp__blk949_dn9 * assign47530_e60962) + (assign47530_e60959 * locals.var_temp__blk949_dn9)) / (2.0 * assign47530_e60966))))), );
            locals.var_vsbstaredge_rv = 0.0;
        }

        if (locals.var_guard1266 != 0.0) {
            let assign47540_e60979: f64 = (locals.var_v_ds - locals.var_vdsx);
            let assign47540_e60980: f64 = (0.5 * assign47540_e60979);
            let assign47540_e60981: f64 = (locals.var_vsbstaredge + assign47540_e60980);
            (locals.var_vsbxedge, locals.var_vsbxedge_dn4, locals.var_vsbxedge_dn6, locals.var_vsbxedge_dn7, locals.var_vsbxedge_dn8, locals.var_vsbxedge_dn9, ) = (assign47540_e60981, locals.var_vsbstaredge_dn4, locals.var_vsbstaredge_dn6, (locals.var_vsbstaredge_dn7 + (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7))), (locals.var_vsbstaredge_dn8 + (0.5 * (locals.var_v_ds_dn8 - locals.var_vdsx_dn8))), locals.var_vsbstaredge_dn9, );
            locals.var_vsbxedge_rv = 0.0;
        }

        if (locals.var_guard1266 != 0.0) {
            let assign47550_e60989: f64 = (locals.var_pscededge_i * locals.var_vdsx);
            let assign47550_e60990: f64 = (1.0 + assign47550_e60989);
            let assign47550_e60991: f64 = (locals.var_psceedge_i * assign47550_e60990);
            let assign47550_e60995: f64 = (locals.var_pscebedge_i * locals.var_vsbxedge);
            let assign47550_e60996: f64 = (1.0 + assign47550_e60995);
            let assign47550_e60997: f64 = (assign47550_e60991 * assign47550_e60996);
            (locals.var_dphit1edge, locals.var_dphit1edge_dn4, locals.var_dphit1edge_dn6, locals.var_dphit1edge_dn7, locals.var_dphit1edge_dn8, locals.var_dphit1edge_dn9, ) = (assign47550_e60997, (assign47550_e60991 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn4)), (assign47550_e60991 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn6)), (((locals.var_psceedge_i * (locals.var_pscededge_i * locals.var_vdsx_dn7)) * assign47550_e60996) + (assign47550_e60991 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn7))), (((locals.var_psceedge_i * (locals.var_pscededge_i * locals.var_vdsx_dn8)) * assign47550_e60996) + (assign47550_e60991 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn8))), (assign47550_e60991 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn9)), );
            locals.var_dphit1edge_rv = 0.0;
        }

        if (locals.var_guard1266 != 0.0) {
            let assign47560_e61004: f64 = (1.0 + locals.var_dphit1edge);
            let assign47560_e61005: f64 = (locals.var_phit0edge * assign47560_e61004);
            (locals.var_phit1edge, locals.var_phit1edge_dn4, locals.var_phit1edge_dn6, locals.var_phit1edge_dn7, locals.var_phit1edge_dn8, locals.var_phit1edge_dn9, ) = (assign47560_e61005, ((locals.var_phit0edge_dn4 * assign47560_e61004) + (locals.var_phit0edge * locals.var_dphit1edge_dn4)), (locals.var_phit0edge * locals.var_dphit1edge_dn6), (locals.var_phit0edge * locals.var_dphit1edge_dn7), (locals.var_phit0edge * locals.var_dphit1edge_dn8), (locals.var_phit0edge * locals.var_dphit1edge_dn9), );
            locals.var_phit1edge_rv = 0.0;
        }

        if (locals.var_guard1266 != 0.0) {
            let assign47570_e61011: f64 = (1.0 / locals.var_phit1edge);
            (locals.var_inv_phit1edge, locals.var_inv_phit1edge_dn4, locals.var_inv_phit1edge_dn6, locals.var_inv_phit1edge_dn7, locals.var_inv_phit1edge_dn8, locals.var_inv_phit1edge_dn9, ) = (assign47570_e61011, (-(locals.var_phit1edge_dn4 / (locals.var_phit1edge * locals.var_phit1edge))), (-(locals.var_phit1edge_dn6 / (locals.var_phit1edge * locals.var_phit1edge))), (-(locals.var_phit1edge_dn7 / (locals.var_phit1edge * locals.var_phit1edge))), (-(locals.var_phit1edge_dn8 / (locals.var_phit1edge * locals.var_phit1edge))), (-(locals.var_phit1edge_dn9 / (locals.var_phit1edge * locals.var_phit1edge))), );
            locals.var_inv_phit1edge_rv = 0.0;
        }

        if (locals.var_guard1266 != 0.0) {
            let assign47580_e61017: f64 = (2.0 * locals.var_vdsx);
            let assign47580_e61022: f64 = (locals.var_cfdedge_i * locals.var_vdsx);
            let assign47580_e61023: f64 = (1.0 + assign47580_e61022);
            let assign47580_e61024: f64 = (assign47580_e61023).sqrt();
            let assign47580_e61025: f64 = (1.0 + assign47580_e61024);
            let assign47580_e61026: f64 = (assign47580_e61017 / assign47580_e61025);
            (locals.var_vdspedge, locals.var_vdspedge_dn7, locals.var_vdspedge_dn8, ) = (assign47580_e61026, ((((2.0 * locals.var_vdsx_dn7) * assign47580_e61025) - (assign47580_e61017 * ((locals.var_cfdedge_i * locals.var_vdsx_dn7) / (2.0 * assign47580_e61024)))) / (assign47580_e61025 * assign47580_e61025)), ((((2.0 * locals.var_vdsx_dn8) * assign47580_e61025) - (assign47580_e61017 * ((locals.var_cfdedge_i * locals.var_vdsx_dn8) / (2.0 * assign47580_e61024)))) / (assign47580_e61025 * assign47580_e61025)), );
            locals.var_vdspedge_rv = 0.0;
        }

        if (locals.var_guard1266 != 0.0) {
            let assign47590_e61032: f64 = (locals.var_cfedge_i * locals.var_vdspedge);
            let assign47590_e61036: f64 = (locals.var_cfbedge_i * locals.var_vsbxedge);
            let assign47590_e61037: f64 = (1.0 + assign47590_e61036);
            let assign47590_e61038: f64 = (assign47590_e61032 * assign47590_e61037);
            (locals.var_delvgedge, locals.var_delvgedge_dn4, locals.var_delvgedge_dn6, locals.var_delvgedge_dn7, locals.var_delvgedge_dn8, locals.var_delvgedge_dn9, ) = (assign47590_e61038, (assign47590_e61032 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn4)), (assign47590_e61032 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn6)), (((locals.var_cfedge_i * locals.var_vdspedge_dn7) * assign47590_e61037) + (assign47590_e61032 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn7))), (((locals.var_cfedge_i * locals.var_vdspedge_dn8) * assign47590_e61037) + (assign47590_e61032 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn8))), (assign47590_e61032 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn9)), );
            locals.var_delvgedge_rv = 0.0;
        }

        if (locals.var_guard1266 != 0.0) {
            let assign47600_e61045: f64 = (locals.var_vgb + locals.var_delvgedge);
            let assign47600_e61047: f64 = (assign47600_e61045 - locals.var_vfbedge_t);
            let assign47600_e61048: f64 = (locals.var_inv_phit1edge * assign47600_e61047);
            (locals.var_xgedge, locals.var_xgedge_dn4, locals.var_xgedge_dn6, locals.var_xgedge_dn7, locals.var_xgedge_dn8, locals.var_xgedge_dn9, ) = (assign47600_e61048, ((locals.var_inv_phit1edge_dn4 * assign47600_e61047) + (locals.var_inv_phit1edge * (locals.var_delvgedge_dn4 - locals.var_vfbedge_t_dn4))), ((locals.var_inv_phit1edge_dn6 * assign47600_e61047) + (locals.var_inv_phit1edge * (locals.var_vgb_dn6 + locals.var_delvgedge_dn6))), ((locals.var_inv_phit1edge_dn7 * assign47600_e61047) + (locals.var_inv_phit1edge * (locals.var_vgb_dn7 + locals.var_delvgedge_dn7))), ((locals.var_inv_phit1edge_dn8 * assign47600_e61047) + (locals.var_inv_phit1edge * (locals.var_vgb_dn8 + locals.var_delvgedge_dn8))), ((locals.var_inv_phit1edge_dn9 * assign47600_e61047) + (locals.var_inv_phit1edge * (locals.var_vgb_dn9 + locals.var_delvgedge_dn9))), );
            locals.var_xgedge_rv = 0.0;
        }

        if (locals.var_guard1266 != 0.0) {
            let assign47610_e61054: f64 = (locals.var_inv_phit1edge * locals.var_phibedge);
            (locals.var_xbedge, locals.var_xbedge_dn4, locals.var_xbedge_dn6, locals.var_xbedge_dn7, locals.var_xbedge_dn8, locals.var_xbedge_dn9, ) = (assign47610_e61054, ((locals.var_inv_phit1edge_dn4 * locals.var_phibedge) + (locals.var_inv_phit1edge * locals.var_phibedge_dn4)), (locals.var_inv_phit1edge_dn6 * locals.var_phibedge), (locals.var_inv_phit1edge_dn7 * locals.var_phibedge), (locals.var_inv_phit1edge_dn8 * locals.var_phibedge), (locals.var_inv_phit1edge_dn9 * locals.var_phibedge), );
            locals.var_xbedge_rv = 0.0;
        }

        if (locals.var_guard1266 != 0.0) {
            let assign47620_e61061: f64 = (locals.var_xbedge / locals.var_gfedge);
            let assign47620_e61063: f64 = (locals.var_xbedge).sqrt();
            let assign47620_e61064: f64 = (assign47620_e61061 + assign47620_e61063);
            let assign47620_e61065: f64 = (assign47620_e61064).ln();
            let assign47620_e61066: f64 = (2.0 * assign47620_e61065);
            (locals.var_dxthedge, locals.var_dxthedge_dn4, locals.var_dxthedge_dn6, locals.var_dxthedge_dn7, locals.var_dxthedge_dn8, locals.var_dxthedge_dn9, ) = (assign47620_e61066, (2.0 * (((((locals.var_xbedge_dn4 * locals.var_gfedge) - (locals.var_xbedge * locals.var_gfedge_dn4)) / (locals.var_gfedge * locals.var_gfedge)) + (locals.var_xbedge_dn4 / (2.0 * assign47620_e61063))) / assign47620_e61064)), (2.0 * (((locals.var_xbedge_dn6 / locals.var_gfedge) + (locals.var_xbedge_dn6 / (2.0 * assign47620_e61063))) / assign47620_e61064)), (2.0 * (((locals.var_xbedge_dn7 / locals.var_gfedge) + (locals.var_xbedge_dn7 / (2.0 * assign47620_e61063))) / assign47620_e61064)), (2.0 * (((locals.var_xbedge_dn8 / locals.var_gfedge) + (locals.var_xbedge_dn8 / (2.0 * assign47620_e61063))) / assign47620_e61064)), (2.0 * (((locals.var_xbedge_dn9 / locals.var_gfedge) + (locals.var_xbedge_dn9 / (2.0 * assign47620_e61063))) / assign47620_e61064)), );
            locals.var_dxthedge_rv = 0.0;
        }

        if (locals.var_guard1266 != 0.0) {
            let assign47630_e61072: f64 = (locals.var_inv_phit1edge * locals.var_vsbstaredge);
            (locals.var_xnedge_s, locals.var_xnedge_s_dn4, locals.var_xnedge_s_dn6, locals.var_xnedge_s_dn7, locals.var_xnedge_s_dn8, locals.var_xnedge_s_dn9, ) = (assign47630_e61072, ((locals.var_inv_phit1edge_dn4 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn4)), ((locals.var_inv_phit1edge_dn6 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn6)), ((locals.var_inv_phit1edge_dn7 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn7)), ((locals.var_inv_phit1edge_dn8 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn8)), ((locals.var_inv_phit1edge_dn9 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn9)), );
            locals.var_xnedge_s_rv = 0.0;
        }

        if (locals.var_guard1266 != 0.0) {
            let assign47640_e61078: f64 = (locals.var_xbedge + locals.var_xnedge_s);
            (locals.var_q_edge_xsth, locals.var_q_edge_xsth_dn4, locals.var_q_edge_xsth_dn6, locals.var_q_edge_xsth_dn7, locals.var_q_edge_xsth_dn8, locals.var_q_edge_xsth_dn9, ) = (assign47640_e61078, (locals.var_xbedge_dn4 + locals.var_xnedge_s_dn4), (locals.var_xbedge_dn6 + locals.var_xnedge_s_dn6), (locals.var_xbedge_dn7 + locals.var_xnedge_s_dn7), (locals.var_xbedge_dn8 + locals.var_xnedge_s_dn8), (locals.var_xbedge_dn9 + locals.var_xnedge_s_dn9), );
            locals.var_q_edge_xsth_rv = 0.0;
        }

        if (locals.var_guard1266 != 0.0) {
            let assign47650_e61085: f64 = (locals.var_q_edge_xsth).sqrt();
            let assign47650_e61086: f64 = (locals.var_gfedge * assign47650_e61085);
            let assign47650_e61087: f64 = (locals.var_q_edge_xsth + assign47650_e61086);
            (locals.var_q_edge_xth0, locals.var_q_edge_xth0_dn4, locals.var_q_edge_xth0_dn6, locals.var_q_edge_xth0_dn7, locals.var_q_edge_xth0_dn8, locals.var_q_edge_xth0_dn9, ) = (assign47650_e61087, (locals.var_q_edge_xsth_dn4 + ((locals.var_gfedge_dn4 * assign47650_e61085) + (locals.var_gfedge * (locals.var_q_edge_xsth_dn4 / (2.0 * assign47650_e61085))))), (locals.var_q_edge_xsth_dn6 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47650_e61085)))), (locals.var_q_edge_xsth_dn7 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47650_e61085)))), (locals.var_q_edge_xsth_dn8 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47650_e61085)))), (locals.var_q_edge_xsth_dn9 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn9 / (2.0 * assign47650_e61085)))), );
            locals.var_q_edge_xth0_rv = 0.0;
        }

        if (locals.var_guard1266 != 0.0) {
            let assign47660_e61093: f64 = (locals.var_q_edge_xth0 + locals.var_dxthedge);
            (locals.var_q_edge_xth, locals.var_q_edge_xth_dn4, locals.var_q_edge_xth_dn6, locals.var_q_edge_xth_dn7, locals.var_q_edge_xth_dn8, locals.var_q_edge_xth_dn9, ) = (assign47660_e61093, (locals.var_q_edge_xth0_dn4 + locals.var_dxthedge_dn4), (locals.var_q_edge_xth0_dn6 + locals.var_dxthedge_dn6), (locals.var_q_edge_xth0_dn7 + locals.var_dxthedge_dn7), (locals.var_q_edge_xth0_dn8 + locals.var_dxthedge_dn8), (locals.var_q_edge_xth0_dn9 + locals.var_dxthedge_dn9), );
            locals.var_q_edge_xth_rv = 0.0;
        }

        if (locals.var_guard1266 != 0.0) {
            let assign47670_e61101: f64 = (locals.var_q_edge_xsth).sqrt();
            let assign47670_e61102: f64 = (2.0 * assign47670_e61101);
            let assign47670_e61103: f64 = (locals.var_gfedge / assign47670_e61102);
            let assign47670_e61104: f64 = (1.0 + assign47670_e61103);
            (locals.var_q_edge_n, locals.var_q_edge_n_dn4, locals.var_q_edge_n_dn6, locals.var_q_edge_n_dn7, locals.var_q_edge_n_dn8, locals.var_q_edge_n_dn9, ) = (assign47670_e61104, (((locals.var_gfedge_dn4 * assign47670_e61102) - (locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn4 / (2.0 * assign47670_e61101))))) / (assign47670_e61102 * assign47670_e61102)), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47670_e61101)))) / (assign47670_e61102 * assign47670_e61102))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47670_e61101)))) / (assign47670_e61102 * assign47670_e61102))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47670_e61101)))) / (assign47670_e61102 * assign47670_e61102))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn9 / (2.0 * assign47670_e61101)))) / (assign47670_e61102 * assign47670_e61102))), );
            locals.var_q_edge_n_rv = 0.0;
        }

        if (locals.var_guard1266 != 0.0) {
            let assign47680_e61110: f64 = (1.0 / locals.var_q_edge_n);
            (locals.var_q_edge_n_inv, locals.var_q_edge_n_inv_dn4, locals.var_q_edge_n_inv_dn6, locals.var_q_edge_n_inv_dn7, locals.var_q_edge_n_inv_dn8, locals.var_q_edge_n_inv_dn9, ) = (assign47680_e61110, (-(locals.var_q_edge_n_dn4 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn6 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn7 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn8 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn9 / (locals.var_q_edge_n * locals.var_q_edge_n))), );
            locals.var_q_edge_n_inv_rv = 0.0;
        }

        if (locals.var_guard1266 != 0.0) {
            let assign47690_e61116: f64 = (locals.var_xgedge - locals.var_q_edge_xth);
            (locals.var_q_edge_xgt, locals.var_q_edge_xgt_dn4, locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8, locals.var_q_edge_xgt_dn9, ) = (assign47690_e61116, (locals.var_xgedge_dn4 - locals.var_q_edge_xth_dn4), (locals.var_xgedge_dn6 - locals.var_q_edge_xth_dn6), (locals.var_xgedge_dn7 - locals.var_q_edge_xth_dn7), (locals.var_xgedge_dn8 - locals.var_q_edge_xth_dn8), (locals.var_xgedge_dn9 - locals.var_q_edge_xth_dn9), );
            locals.var_q_edge_xgt_rv = 0.0;
        }

        let assign47700_e61121: f64 = (-12.0);
        let assign47700_e61122: f64 = if locals.var_q_edge_xgt > assign47700_e61121 { 1.0 } else { 0.0 };
        locals.var_guard1267 = assign47700_e61122;
        locals.var_guard1267_rv = 0.0;

        if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
            let assign47710_e61128: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
            let assign47710_e61130: f64 = (assign47710_e61128 - 1.0);
            (locals.var_q_edge_xgt0, locals.var_q_edge_xgt0_dn4, locals.var_q_edge_xgt0_dn6, locals.var_q_edge_xgt0_dn7, locals.var_q_edge_xgt0_dn8, locals.var_q_edge_xgt0_dn9, ) = (assign47710_e61130, (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4), locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8, locals.var_q_edge_xgt_dn9, );
            locals.var_q_edge_xgt0_rv = 0.0;
        }

        if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
            let assign47720_e61140: f64 = (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0);
            let assign47720_e61142: f64 = (assign47720_e61140 + 10.0);
            let assign47720_e61143: f64 = (assign47720_e61142).sqrt();
            let assign47720_e61144: f64 = (locals.var_q_edge_xgt0 + assign47720_e61143);
            let assign47720_e61145: f64 = (0.5 * assign47720_e61144);
            (locals.var_q_edge_xgt0e, locals.var_q_edge_xgt0e_dn4, locals.var_q_edge_xgt0e_dn6, locals.var_q_edge_xgt0e_dn7, locals.var_q_edge_xgt0e_dn8, locals.var_q_edge_xgt0e_dn9, ) = (assign47720_e61145, (0.5 * (locals.var_q_edge_xgt0_dn4 + (((locals.var_q_edge_xgt0_dn4 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn4)) / (2.0 * assign47720_e61143)))), (0.5 * (locals.var_q_edge_xgt0_dn6 + (((locals.var_q_edge_xgt0_dn6 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn6)) / (2.0 * assign47720_e61143)))), (0.5 * (locals.var_q_edge_xgt0_dn7 + (((locals.var_q_edge_xgt0_dn7 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn7)) / (2.0 * assign47720_e61143)))), (0.5 * (locals.var_q_edge_xgt0_dn8 + (((locals.var_q_edge_xgt0_dn8 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn8)) / (2.0 * assign47720_e61143)))), (0.5 * (locals.var_q_edge_xgt0_dn9 + (((locals.var_q_edge_xgt0_dn9 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn9)) / (2.0 * assign47720_e61143)))), );
            locals.var_q_edge_xgt0e_rv = 0.0;
        }

        if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
            let assign47730_e61154: f64 = (locals.var_q_edge_xgt0e).ln();
            let assign47730_e61155: f64 = (locals.var_q_edge_n * assign47730_e61154);
            let assign47730_e61156: f64 = (locals.var_q_edge_xgt - assign47730_e61155);
            let assign47730_e61158: f64 = (assign47730_e61156 + locals.var_lngfedge2);
            (locals.var_q_edge_qi0si, locals.var_q_edge_qi0si_dn4, locals.var_q_edge_qi0si_dn6, locals.var_q_edge_qi0si_dn7, locals.var_q_edge_qi0si_dn8, locals.var_q_edge_qi0si_dn9, ) = (assign47730_e61158, ((locals.var_q_edge_xgt_dn4 - ((locals.var_q_edge_n_dn4 * assign47730_e61154) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn4 / locals.var_q_edge_xgt0e)))) + locals.var_lngfedge2_dn4), (locals.var_q_edge_xgt_dn6 - ((locals.var_q_edge_n_dn6 * assign47730_e61154) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn6 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn7 - ((locals.var_q_edge_n_dn7 * assign47730_e61154) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn7 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn8 - ((locals.var_q_edge_n_dn8 * assign47730_e61154) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn8 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn9 - ((locals.var_q_edge_n_dn9 * assign47730_e61154) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn9 / locals.var_q_edge_xgt0e)))), );
            locals.var_q_edge_qi0si_rv = 0.0;
        }

        if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
            let assign47740_e61168: f64 = (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si);
            let assign47740_e61170: f64 = (assign47740_e61168 + 2.0);
            let assign47740_e61171: f64 = (assign47740_e61170).sqrt();
            let assign47740_e61172: f64 = (locals.var_q_edge_qi0si + assign47740_e61171);
            let assign47740_e61173: f64 = (0.5 * assign47740_e61172);
            (locals.var_q_edge_qi0, locals.var_q_edge_qi0_dn4, locals.var_q_edge_qi0_dn6, locals.var_q_edge_qi0_dn7, locals.var_q_edge_qi0_dn8, locals.var_q_edge_qi0_dn9, ) = (assign47740_e61173, (0.5 * (locals.var_q_edge_qi0si_dn4 + (((locals.var_q_edge_qi0si_dn4 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn4)) / (2.0 * assign47740_e61171)))), (0.5 * (locals.var_q_edge_qi0si_dn6 + (((locals.var_q_edge_qi0si_dn6 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn6)) / (2.0 * assign47740_e61171)))), (0.5 * (locals.var_q_edge_qi0si_dn7 + (((locals.var_q_edge_qi0si_dn7 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn7)) / (2.0 * assign47740_e61171)))), (0.5 * (locals.var_q_edge_qi0si_dn8 + (((locals.var_q_edge_qi0si_dn8 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn8)) / (2.0 * assign47740_e61171)))), (0.5 * (locals.var_q_edge_qi0si_dn9 + (((locals.var_q_edge_qi0si_dn9 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn9)) / (2.0 * assign47740_e61171)))), );
            locals.var_q_edge_qi0_rv = 0.0;
        }

        let assign47750_e61178: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47750_e61180: f64 = if assign47750_e61178 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1268 = assign47750_e61180;
        locals.var_guard1268_rv = 0.0;

        if (((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) && (locals.var_guard1268 != 0.0)) {
            let assign47760_e61188: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
            let assign47760_e61189: f64 = (assign47760_e61188).exp();
            (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn4, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8, locals.var_q_edge_exp_x_dn9, ) = (assign47760_e61189, (assign47760_e61189 * (locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4)), (assign47760_e61189 * (locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6)), (assign47760_e61189 * (locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7)), (assign47760_e61189 * (locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8)), (assign47760_e61189 * (locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9)), );
            locals.var_q_edge_exp_x_rv = 0.0;
        }

        if (((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) && (locals.var_guard1268 == 0.0)) {
            let assign47770_e61202: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
            let assign47770_e61204: f64 = (assign47770_e61202 - 230.25850929940458);
            let assign47770_e61209: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
            let assign47770_e61211: f64 = (assign47770_e61209 - 230.25850929940458);
            let assign47770_e61215: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
            let assign47770_e61217: f64 = (assign47770_e61215 - 230.25850929940458);
            let assign47770_e61219: f64 = (assign47770_e61217 * 0.3333333333333333);
            let assign47770_e61220: f64 = (1.0 + assign47770_e61219);
            let assign47770_e61221: f64 = (assign47770_e61211 * assign47770_e61220);
            let assign47770_e61222: f64 = (0.5 * assign47770_e61221);
            let assign47770_e61223: f64 = (1.0 + assign47770_e61222);
            let assign47770_e61224: f64 = (assign47770_e61204 * assign47770_e61223);
            let assign47770_e61225: f64 = (1.0 + assign47770_e61224);
            let assign47770_e61226: f64 = (1e100 * assign47770_e61225);
            (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn4, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8, locals.var_q_edge_exp_x_dn9, ) = (assign47770_e61226, (1e100 * (((locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4) * assign47770_e61223) + (assign47770_e61204 * (0.5 * (((locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4) * assign47770_e61220) + (assign47770_e61211 * ((locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign47770_e61223) + (assign47770_e61204 * (0.5 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign47770_e61220) + (assign47770_e61211 * ((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign47770_e61223) + (assign47770_e61204 * (0.5 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign47770_e61220) + (assign47770_e61211 * ((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign47770_e61223) + (assign47770_e61204 * (0.5 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign47770_e61220) + (assign47770_e61211 * ((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9) * assign47770_e61223) + (assign47770_e61204 * (0.5 * (((locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9) * assign47770_e61220) + (assign47770_e61211 * ((locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9) * 0.3333333333333333))))))), );
            locals.var_q_edge_exp_x_rv = 0.0;
        }

        if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
            let assign47780_e61234: f64 = (locals.var_gfedge2 * locals.var_q_edge_exp_x);
            (locals.var_q_edge_d0, locals.var_q_edge_d0_dn4, locals.var_q_edge_d0_dn6, locals.var_q_edge_d0_dn7, locals.var_q_edge_d0_dn8, locals.var_q_edge_d0_dn9, ) = (assign47780_e61234, ((locals.var_gfedge2_dn4 * locals.var_q_edge_exp_x) + (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn4)), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn6), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn7), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn8), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn9), );
            locals.var_q_edge_d0_rv = 0.0;
        }

        if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
            let assign47790_e61242: f64 = (locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv);
            (locals.var_q_edge_d0p, locals.var_q_edge_d0p_dn4, locals.var_q_edge_d0p_dn6, locals.var_q_edge_d0p_dn7, locals.var_q_edge_d0p_dn8, locals.var_q_edge_d0p_dn9, ) = (assign47790_e61242, if locals.var_q_edge_n_inv_dn4 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn4)) } } else { (assign47790_e61242 * ((locals.var_q_edge_n_inv_dn4 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn4 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn6 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn6)) } } else { (assign47790_e61242 * ((locals.var_q_edge_n_inv_dn6 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn6 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn7 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn7)) } } else { (assign47790_e61242 * ((locals.var_q_edge_n_inv_dn7 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn7 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn8 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn8)) } } else { (assign47790_e61242 * ((locals.var_q_edge_n_inv_dn8 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn8 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn9 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn9)) } } else { (assign47790_e61242 * ((locals.var_q_edge_n_inv_dn9 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn9 / locals.var_q_edge_d0)))) }, );
            locals.var_q_edge_d0p_rv = 0.0;
        }

        if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
            let assign47800_e61250: f64 = (locals.var_q_edge_n * locals.var_q_edge_n);
            let assign47800_e61254: f64 = (locals.var_q_edge_qi0 + locals.var_q_edge_n);
            let assign47800_e61255: f64 = (2.0 * assign47800_e61254);
            let assign47800_e61257: f64 = (assign47800_e61255 - locals.var_q_edge_d0p);
            let assign47800_e61259: f64 = (assign47800_e61257 * locals.var_q_edge_d0p);
            let assign47800_e61260: f64 = (assign47800_e61250 + assign47800_e61259);
            (locals.var_q_edge_sqerr, locals.var_q_edge_sqerr_dn4, locals.var_q_edge_sqerr_dn6, locals.var_q_edge_sqerr_dn7, locals.var_q_edge_sqerr_dn8, locals.var_q_edge_sqerr_dn9, ) = (assign47800_e61260, (((locals.var_q_edge_n_dn4 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn4)) + ((((2.0 * (locals.var_q_edge_qi0_dn4 + locals.var_q_edge_n_dn4)) - locals.var_q_edge_d0p_dn4) * locals.var_q_edge_d0p) + (assign47800_e61257 * locals.var_q_edge_d0p_dn4))), (((locals.var_q_edge_n_dn6 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn6)) + ((((2.0 * (locals.var_q_edge_qi0_dn6 + locals.var_q_edge_n_dn6)) - locals.var_q_edge_d0p_dn6) * locals.var_q_edge_d0p) + (assign47800_e61257 * locals.var_q_edge_d0p_dn6))), (((locals.var_q_edge_n_dn7 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn7)) + ((((2.0 * (locals.var_q_edge_qi0_dn7 + locals.var_q_edge_n_dn7)) - locals.var_q_edge_d0p_dn7) * locals.var_q_edge_d0p) + (assign47800_e61257 * locals.var_q_edge_d0p_dn7))), (((locals.var_q_edge_n_dn8 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn8)) + ((((2.0 * (locals.var_q_edge_qi0_dn8 + locals.var_q_edge_n_dn8)) - locals.var_q_edge_d0p_dn8) * locals.var_q_edge_d0p) + (assign47800_e61257 * locals.var_q_edge_d0p_dn8))), (((locals.var_q_edge_n_dn9 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn9)) + ((((2.0 * (locals.var_q_edge_qi0_dn9 + locals.var_q_edge_n_dn9)) - locals.var_q_edge_d0p_dn9) * locals.var_q_edge_d0p) + (assign47800_e61257 * locals.var_q_edge_d0p_dn9))), );
            locals.var_q_edge_sqerr_rv = 0.0;
        }

        if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
            let assign47810_e61268: f64 = (locals.var_q_edge_sqerr).sqrt();
            let assign47810_e61270: f64 = (assign47810_e61268 - locals.var_q_edge_n);
            let assign47810_e61272: f64 = (assign47810_e61270 / locals.var_q_edge_d0p);
            let assign47810_e61274: f64 = (assign47810_e61272 - 1.0);
            let assign47810_e61275: f64 = (locals.var_q_edge_n * assign47810_e61274);
            (locals.var_q_edge_errq, locals.var_q_edge_errq_dn4, locals.var_q_edge_errq_dn6, locals.var_q_edge_errq_dn7, locals.var_q_edge_errq_dn8, locals.var_q_edge_errq_dn9, ) = (assign47810_e61275, ((locals.var_q_edge_n_dn4 * assign47810_e61274) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn4 / (2.0 * assign47810_e61268)) - locals.var_q_edge_n_dn4) * locals.var_q_edge_d0p) - (assign47810_e61270 * locals.var_q_edge_d0p_dn4)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn6 * assign47810_e61274) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn6 / (2.0 * assign47810_e61268)) - locals.var_q_edge_n_dn6) * locals.var_q_edge_d0p) - (assign47810_e61270 * locals.var_q_edge_d0p_dn6)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn7 * assign47810_e61274) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn7 / (2.0 * assign47810_e61268)) - locals.var_q_edge_n_dn7) * locals.var_q_edge_d0p) - (assign47810_e61270 * locals.var_q_edge_d0p_dn7)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn8 * assign47810_e61274) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn8 / (2.0 * assign47810_e61268)) - locals.var_q_edge_n_dn8) * locals.var_q_edge_d0p) - (assign47810_e61270 * locals.var_q_edge_d0p_dn8)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn9 * assign47810_e61274) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn9 / (2.0 * assign47810_e61268)) - locals.var_q_edge_n_dn9) * locals.var_q_edge_d0p) - (assign47810_e61270 * locals.var_q_edge_d0p_dn9)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), );
            locals.var_q_edge_errq_rv = 0.0;
        }

        if ((locals.var_guard1266 != 0.0) && (locals.var_guard1267 != 0.0)) {
            let assign47820_e61283: f64 = (locals.var_q_edge_qi0 - locals.var_q_edge_errq);
            (locals.var_qseffedge, locals.var_qseffedge_dn4, locals.var_qseffedge_dn6, locals.var_qseffedge_dn7, locals.var_qseffedge_dn8, locals.var_qseffedge_dn9, ) = (assign47820_e61283, (locals.var_q_edge_qi0_dn4 - locals.var_q_edge_errq_dn4), (locals.var_q_edge_qi0_dn6 - locals.var_q_edge_errq_dn6), (locals.var_q_edge_qi0_dn7 - locals.var_q_edge_errq_dn7), (locals.var_q_edge_qi0_dn8 - locals.var_q_edge_errq_dn8), (locals.var_q_edge_qi0_dn9 - locals.var_q_edge_errq_dn9), );
            locals.var_qseffedge_rv = 0.0;
        }

        let assign47830_e61289: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47830_e61290: f64 = (locals.var_q_edge_n_inv * assign47830_e61289);
        let assign47830_e61292: f64 = (-230.25850929940458);
        let assign47830_e61293: f64 = if assign47830_e61290 > assign47830_e61292 { 1.0 } else { 0.0 };
        locals.var_guard1269 = assign47830_e61293;
        locals.var_guard1269_rv = 0.0;

        if (((locals.var_guard1266 != 0.0) && (locals.var_guard1267 == 0.0)) && (locals.var_guard1269 != 0.0)) {
            let assign47840_e61303: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
            let assign47840_e61304: f64 = (locals.var_q_edge_n_inv * assign47840_e61303);
            let assign47840_e61305: f64 = (assign47840_e61304).exp();
            (locals.var_qseffedge, locals.var_qseffedge_dn4, locals.var_qseffedge_dn6, locals.var_qseffedge_dn7, locals.var_qseffedge_dn8, locals.var_qseffedge_dn9, ) = (assign47840_e61305, (assign47840_e61305 * ((locals.var_q_edge_n_inv_dn4 * assign47840_e61303) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))), (assign47840_e61305 * ((locals.var_q_edge_n_inv_dn6 * assign47840_e61303) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))), (assign47840_e61305 * ((locals.var_q_edge_n_inv_dn7 * assign47840_e61303) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))), (assign47840_e61305 * ((locals.var_q_edge_n_inv_dn8 * assign47840_e61303) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))), (assign47840_e61305 * ((locals.var_q_edge_n_inv_dn9 * assign47840_e61303) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))), );
            locals.var_qseffedge_rv = 0.0;
        }

        if (((locals.var_guard1266 != 0.0) && (locals.var_guard1267 == 0.0)) && (locals.var_guard1269 == 0.0)) {
            let assign47850_e61318: f64 = (-230.25850929940458);
            let assign47850_e61322: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
            let assign47850_e61323: f64 = (locals.var_q_edge_n_inv * assign47850_e61322);
            let assign47850_e61324: f64 = (assign47850_e61318 - assign47850_e61323);
            let assign47850_e61328: f64 = (-230.25850929940458);
            let assign47850_e61332: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
            let assign47850_e61333: f64 = (locals.var_q_edge_n_inv * assign47850_e61332);
            let assign47850_e61334: f64 = (assign47850_e61328 - assign47850_e61333);
            let assign47850_e61337: f64 = (-230.25850929940458);
            let assign47850_e61341: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
            let assign47850_e61342: f64 = (locals.var_q_edge_n_inv * assign47850_e61341);
            let assign47850_e61343: f64 = (assign47850_e61337 - assign47850_e61342);
            let assign47850_e61345: f64 = (assign47850_e61343 * 0.3333333333333333);
            let assign47850_e61346: f64 = (1.0 + assign47850_e61345);
            let assign47850_e61347: f64 = (assign47850_e61334 * assign47850_e61346);
            let assign47850_e61348: f64 = (0.5 * assign47850_e61347);
            let assign47850_e61349: f64 = (1.0 + assign47850_e61348);
            let assign47850_e61350: f64 = (assign47850_e61324 * assign47850_e61349);
            let assign47850_e61351: f64 = (1.0 + assign47850_e61350);
            let assign47850_e61352: f64 = (1e-100 / assign47850_e61351);
            (locals.var_qseffedge, locals.var_qseffedge_dn4, locals.var_qseffedge_dn6, locals.var_qseffedge_dn7, locals.var_qseffedge_dn8, locals.var_qseffedge_dn9, ) = (assign47850_e61352, (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn4 * assign47850_e61322) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))) * assign47850_e61349) + (assign47850_e61324 * (0.5 * (((-((locals.var_q_edge_n_inv_dn4 * assign47850_e61332) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))) * assign47850_e61346) + (assign47850_e61334 * ((-((locals.var_q_edge_n_inv_dn4 * assign47850_e61341) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))) * 0.3333333333333333))))))) / (assign47850_e61351 * assign47850_e61351))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn6 * assign47850_e61322) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign47850_e61349) + (assign47850_e61324 * (0.5 * (((-((locals.var_q_edge_n_inv_dn6 * assign47850_e61332) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign47850_e61346) + (assign47850_e61334 * ((-((locals.var_q_edge_n_inv_dn6 * assign47850_e61341) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * 0.3333333333333333))))))) / (assign47850_e61351 * assign47850_e61351))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn7 * assign47850_e61322) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign47850_e61349) + (assign47850_e61324 * (0.5 * (((-((locals.var_q_edge_n_inv_dn7 * assign47850_e61332) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign47850_e61346) + (assign47850_e61334 * ((-((locals.var_q_edge_n_inv_dn7 * assign47850_e61341) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * 0.3333333333333333))))))) / (assign47850_e61351 * assign47850_e61351))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn8 * assign47850_e61322) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign47850_e61349) + (assign47850_e61324 * (0.5 * (((-((locals.var_q_edge_n_inv_dn8 * assign47850_e61332) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign47850_e61346) + (assign47850_e61334 * ((-((locals.var_q_edge_n_inv_dn8 * assign47850_e61341) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * 0.3333333333333333))))))) / (assign47850_e61351 * assign47850_e61351))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn9 * assign47850_e61322) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))) * assign47850_e61349) + (assign47850_e61324 * (0.5 * (((-((locals.var_q_edge_n_inv_dn9 * assign47850_e61332) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))) * assign47850_e61346) + (assign47850_e61334 * ((-((locals.var_q_edge_n_inv_dn9 * assign47850_e61341) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))) * 0.3333333333333333))))))) / (assign47850_e61351 * assign47850_e61351))), );
            locals.var_qseffedge_rv = 0.0;
        }

        if (locals.var_guard1266 != 0.0) {
            let assign47860_e61359: f64 = (locals.var_vdse_dc + locals.var_vsbstaredge);
            let assign47860_e61360: f64 = (locals.var_inv_phit1edge * assign47860_e61359);
            (locals.var_xnedge_d, locals.var_xnedge_d_dn4, locals.var_xnedge_d_dn6, locals.var_xnedge_d_dn7, locals.var_xnedge_d_dn8, locals.var_xnedge_d_dn9, ) = (assign47860_e61360, ((locals.var_inv_phit1edge_dn4 * assign47860_e61359) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn4 + locals.var_vsbstaredge_dn4))), ((locals.var_inv_phit1edge_dn6 * assign47860_e61359) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn6 + locals.var_vsbstaredge_dn6))), ((locals.var_inv_phit1edge_dn7 * assign47860_e61359) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn7 + locals.var_vsbstaredge_dn7))), ((locals.var_inv_phit1edge_dn8 * assign47860_e61359) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn8 + locals.var_vsbstaredge_dn8))), ((locals.var_inv_phit1edge_dn9 * assign47860_e61359) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn9 + locals.var_vsbstaredge_dn9))), );
            locals.var_xnedge_d_rv = 0.0;
        }

        let assign47870_e61369: f64 = if ((locals.var_qseffedge < 0.001) && (locals.var_vdse_dc < 1e-6)) { 1.0 } else { 0.0 };
        locals.var_guard1270 = assign47870_e61369;
        locals.var_guard1270_rv = 0.0;

        let assign47880_e61371: f64 = (-locals.var_xnedge_d);
        let assign47880_e61373: f64 = (assign47880_e61371 + locals.var_xnedge_s);
        let assign47880_e61375: f64 = (-230.25850929940458);
        let assign47880_e61376: f64 = if assign47880_e61373 > assign47880_e61375 { 1.0 } else { 0.0 };
        locals.var_guard1271 = assign47880_e61376;
        locals.var_guard1271_rv = 0.0;

        if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 != 0.0)) && (locals.var_guard1271 != 0.0)) {
            let assign47890_e61383: f64 = (-locals.var_xnedge_d);
            let assign47890_e61385: f64 = (assign47890_e61383 + locals.var_xnedge_s);
            let assign47890_e61386: f64 = (assign47890_e61385).exp();
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign47890_e61386, (assign47890_e61386 * ((-locals.var_xnedge_d_dn4) + locals.var_xnedge_s_dn4)), (assign47890_e61386 * ((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)), (assign47890_e61386 * ((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)), (assign47890_e61386 * ((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)), (assign47890_e61386 * ((-locals.var_xnedge_d_dn9) + locals.var_xnedge_s_dn9)), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 != 0.0)) && (locals.var_guard1271 == 0.0)) {
            let assign47900_e61398: f64 = (-230.25850929940458);
            let assign47900_e61400: f64 = (-locals.var_xnedge_d);
            let assign47900_e61402: f64 = (assign47900_e61400 + locals.var_xnedge_s);
            let assign47900_e61403: f64 = (assign47900_e61398 - assign47900_e61402);
            let assign47900_e61407: f64 = (-230.25850929940458);
            let assign47900_e61409: f64 = (-locals.var_xnedge_d);
            let assign47900_e61411: f64 = (assign47900_e61409 + locals.var_xnedge_s);
            let assign47900_e61412: f64 = (assign47900_e61407 - assign47900_e61411);
            let assign47900_e61415: f64 = (-230.25850929940458);
            let assign47900_e61417: f64 = (-locals.var_xnedge_d);
            let assign47900_e61419: f64 = (assign47900_e61417 + locals.var_xnedge_s);
            let assign47900_e61420: f64 = (assign47900_e61415 - assign47900_e61419);
            let assign47900_e61422: f64 = (assign47900_e61420 * 0.3333333333333333);
            let assign47900_e61423: f64 = (1.0 + assign47900_e61422);
            let assign47900_e61424: f64 = (assign47900_e61412 * assign47900_e61423);
            let assign47900_e61425: f64 = (0.5 * assign47900_e61424);
            let assign47900_e61426: f64 = (1.0 + assign47900_e61425);
            let assign47900_e61427: f64 = (assign47900_e61403 * assign47900_e61426);
            let assign47900_e61428: f64 = (1.0 + assign47900_e61427);
            let assign47900_e61429: f64 = (1e-100 / assign47900_e61428);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign47900_e61429, (-((1e-100 * (((-((-locals.var_xnedge_d_dn4) + locals.var_xnedge_s_dn4)) * assign47900_e61426) + (assign47900_e61403 * (0.5 * (((-((-locals.var_xnedge_d_dn4) + locals.var_xnedge_s_dn4)) * assign47900_e61423) + (assign47900_e61412 * ((-((-locals.var_xnedge_d_dn4) + locals.var_xnedge_s_dn4)) * 0.3333333333333333))))))) / (assign47900_e61428 * assign47900_e61428))), (-((1e-100 * (((-((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)) * assign47900_e61426) + (assign47900_e61403 * (0.5 * (((-((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)) * assign47900_e61423) + (assign47900_e61412 * ((-((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)) * 0.3333333333333333))))))) / (assign47900_e61428 * assign47900_e61428))), (-((1e-100 * (((-((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)) * assign47900_e61426) + (assign47900_e61403 * (0.5 * (((-((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)) * assign47900_e61423) + (assign47900_e61412 * ((-((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)) * 0.3333333333333333))))))) / (assign47900_e61428 * assign47900_e61428))), (-((1e-100 * (((-((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)) * assign47900_e61426) + (assign47900_e61403 * (0.5 * (((-((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)) * assign47900_e61423) + (assign47900_e61412 * ((-((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)) * 0.3333333333333333))))))) / (assign47900_e61428 * assign47900_e61428))), (-((1e-100 * (((-((-locals.var_xnedge_d_dn9) + locals.var_xnedge_s_dn9)) * assign47900_e61426) + (assign47900_e61403 * (0.5 * (((-((-locals.var_xnedge_d_dn9) + locals.var_xnedge_s_dn9)) * assign47900_e61423) + (assign47900_e61412 * ((-((-locals.var_xnedge_d_dn9) + locals.var_xnedge_s_dn9)) * 0.3333333333333333))))))) / (assign47900_e61428 * assign47900_e61428))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 != 0.0)) {
            let assign47910_e61438: f64 = (locals.var_temp__blk949 - 1.0);
            let assign47910_e61439: f64 = (locals.var_qseffedge * assign47910_e61438);
            (locals.var_qdseffedge, locals.var_qdseffedge_dn4, locals.var_qdseffedge_dn6, locals.var_qdseffedge_dn7, locals.var_qdseffedge_dn8, locals.var_qdseffedge_dn9, ) = (assign47910_e61439, ((locals.var_qseffedge_dn4 * assign47910_e61438) + (locals.var_qseffedge * locals.var_temp__blk949_dn4)), ((locals.var_qseffedge_dn6 * assign47910_e61438) + (locals.var_qseffedge * locals.var_temp__blk949_dn6)), ((locals.var_qseffedge_dn7 * assign47910_e61438) + (locals.var_qseffedge * locals.var_temp__blk949_dn7)), ((locals.var_qseffedge_dn8 * assign47910_e61438) + (locals.var_qseffedge * locals.var_temp__blk949_dn8)), ((locals.var_qseffedge_dn9 * assign47910_e61438) + (locals.var_qseffedge * locals.var_temp__blk949_dn9)), );
            locals.var_qdseffedge_rv = 0.0;
        }

        if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 != 0.0)) {
            let assign47920_e61447: f64 = (locals.var_qdseffedge + locals.var_qseffedge);
            (locals.var_qdeffedge, locals.var_qdeffedge_dn4, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8, locals.var_qdeffedge_dn9, ) = (assign47920_e61447, (locals.var_qdseffedge_dn4 + locals.var_qseffedge_dn4), (locals.var_qdseffedge_dn6 + locals.var_qseffedge_dn6), (locals.var_qdseffedge_dn7 + locals.var_qseffedge_dn7), (locals.var_qdseffedge_dn8 + locals.var_qseffedge_dn8), (locals.var_qdseffedge_dn9 + locals.var_qseffedge_dn9), );
            locals.var_qdeffedge_rv = 0.0;
        }

        if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) {
            let assign47930_e61456: f64 = (locals.var_xbedge + locals.var_xnedge_d);
            (locals.var_q_edge_xsth, locals.var_q_edge_xsth_dn4, locals.var_q_edge_xsth_dn6, locals.var_q_edge_xsth_dn7, locals.var_q_edge_xsth_dn8, locals.var_q_edge_xsth_dn9, ) = (assign47930_e61456, (locals.var_xbedge_dn4 + locals.var_xnedge_d_dn4), (locals.var_xbedge_dn6 + locals.var_xnedge_d_dn6), (locals.var_xbedge_dn7 + locals.var_xnedge_d_dn7), (locals.var_xbedge_dn8 + locals.var_xnedge_d_dn8), (locals.var_xbedge_dn9 + locals.var_xnedge_d_dn9), );
            locals.var_q_edge_xsth_rv = 0.0;
        }

        if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) {
            let assign47940_e61466: f64 = (locals.var_q_edge_xsth).sqrt();
            let assign47940_e61467: f64 = (locals.var_gfedge * assign47940_e61466);
            let assign47940_e61468: f64 = (locals.var_q_edge_xsth + assign47940_e61467);
            (locals.var_q_edge_xth0, locals.var_q_edge_xth0_dn4, locals.var_q_edge_xth0_dn6, locals.var_q_edge_xth0_dn7, locals.var_q_edge_xth0_dn8, locals.var_q_edge_xth0_dn9, ) = (assign47940_e61468, (locals.var_q_edge_xsth_dn4 + ((locals.var_gfedge_dn4 * assign47940_e61466) + (locals.var_gfedge * (locals.var_q_edge_xsth_dn4 / (2.0 * assign47940_e61466))))), (locals.var_q_edge_xsth_dn6 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47940_e61466)))), (locals.var_q_edge_xsth_dn7 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47940_e61466)))), (locals.var_q_edge_xsth_dn8 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47940_e61466)))), (locals.var_q_edge_xsth_dn9 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn9 / (2.0 * assign47940_e61466)))), );
            locals.var_q_edge_xth0_rv = 0.0;
        }

        if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) {
            let assign47950_e61477: f64 = (locals.var_q_edge_xth0 + locals.var_dxthedge);
            (locals.var_q_edge_xth, locals.var_q_edge_xth_dn4, locals.var_q_edge_xth_dn6, locals.var_q_edge_xth_dn7, locals.var_q_edge_xth_dn8, locals.var_q_edge_xth_dn9, ) = (assign47950_e61477, (locals.var_q_edge_xth0_dn4 + locals.var_dxthedge_dn4), (locals.var_q_edge_xth0_dn6 + locals.var_dxthedge_dn6), (locals.var_q_edge_xth0_dn7 + locals.var_dxthedge_dn7), (locals.var_q_edge_xth0_dn8 + locals.var_dxthedge_dn8), (locals.var_q_edge_xth0_dn9 + locals.var_dxthedge_dn9), );
            locals.var_q_edge_xth_rv = 0.0;
        }

        if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) {
            let assign47960_e61488: f64 = (locals.var_q_edge_xsth).sqrt();
            let assign47960_e61489: f64 = (2.0 * assign47960_e61488);
            let assign47960_e61490: f64 = (locals.var_gfedge / assign47960_e61489);
            let assign47960_e61491: f64 = (1.0 + assign47960_e61490);
            (locals.var_q_edge_n, locals.var_q_edge_n_dn4, locals.var_q_edge_n_dn6, locals.var_q_edge_n_dn7, locals.var_q_edge_n_dn8, locals.var_q_edge_n_dn9, ) = (assign47960_e61491, (((locals.var_gfedge_dn4 * assign47960_e61489) - (locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn4 / (2.0 * assign47960_e61488))))) / (assign47960_e61489 * assign47960_e61489)), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47960_e61488)))) / (assign47960_e61489 * assign47960_e61489))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47960_e61488)))) / (assign47960_e61489 * assign47960_e61489))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47960_e61488)))) / (assign47960_e61489 * assign47960_e61489))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn9 / (2.0 * assign47960_e61488)))) / (assign47960_e61489 * assign47960_e61489))), );
            locals.var_q_edge_n_rv = 0.0;
        }

        if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) {
            let assign47970_e61500: f64 = (1.0 / locals.var_q_edge_n);
            (locals.var_q_edge_n_inv, locals.var_q_edge_n_inv_dn4, locals.var_q_edge_n_inv_dn6, locals.var_q_edge_n_inv_dn7, locals.var_q_edge_n_inv_dn8, locals.var_q_edge_n_inv_dn9, ) = (assign47970_e61500, (-(locals.var_q_edge_n_dn4 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn6 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn7 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn8 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn9 / (locals.var_q_edge_n * locals.var_q_edge_n))), );
            locals.var_q_edge_n_inv_rv = 0.0;
        }

        if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) {
            let assign47980_e61509: f64 = (locals.var_xgedge - locals.var_q_edge_xth);
            (locals.var_q_edge_xgt, locals.var_q_edge_xgt_dn4, locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8, locals.var_q_edge_xgt_dn9, ) = (assign47980_e61509, (locals.var_xgedge_dn4 - locals.var_q_edge_xth_dn4), (locals.var_xgedge_dn6 - locals.var_q_edge_xth_dn6), (locals.var_xgedge_dn7 - locals.var_q_edge_xth_dn7), (locals.var_xgedge_dn8 - locals.var_q_edge_xth_dn8), (locals.var_xgedge_dn9 - locals.var_q_edge_xth_dn9), );
            locals.var_q_edge_xgt_rv = 0.0;
        }

        let assign47990_e61514: f64 = (-12.0);
        let assign47990_e61515: f64 = if locals.var_q_edge_xgt > assign47990_e61514 { 1.0 } else { 0.0 };
        locals.var_guard1272 = assign47990_e61515;
        locals.var_guard1272_rv = 0.0;

        if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
            let assign48000_e61524: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
            let assign48000_e61526: f64 = (assign48000_e61524 - 1.0);
            (locals.var_q_edge_xgt0, locals.var_q_edge_xgt0_dn4, locals.var_q_edge_xgt0_dn6, locals.var_q_edge_xgt0_dn7, locals.var_q_edge_xgt0_dn8, locals.var_q_edge_xgt0_dn9, ) = (assign48000_e61526, (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4), locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8, locals.var_q_edge_xgt_dn9, );
            locals.var_q_edge_xgt0_rv = 0.0;
        }

        if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
            let assign48010_e61539: f64 = (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0);
            let assign48010_e61541: f64 = (assign48010_e61539 + 10.0);
            let assign48010_e61542: f64 = (assign48010_e61541).sqrt();
            let assign48010_e61543: f64 = (locals.var_q_edge_xgt0 + assign48010_e61542);
            let assign48010_e61544: f64 = (0.5 * assign48010_e61543);
            (locals.var_q_edge_xgt0e, locals.var_q_edge_xgt0e_dn4, locals.var_q_edge_xgt0e_dn6, locals.var_q_edge_xgt0e_dn7, locals.var_q_edge_xgt0e_dn8, locals.var_q_edge_xgt0e_dn9, ) = (assign48010_e61544, (0.5 * (locals.var_q_edge_xgt0_dn4 + (((locals.var_q_edge_xgt0_dn4 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn4)) / (2.0 * assign48010_e61542)))), (0.5 * (locals.var_q_edge_xgt0_dn6 + (((locals.var_q_edge_xgt0_dn6 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn6)) / (2.0 * assign48010_e61542)))), (0.5 * (locals.var_q_edge_xgt0_dn7 + (((locals.var_q_edge_xgt0_dn7 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn7)) / (2.0 * assign48010_e61542)))), (0.5 * (locals.var_q_edge_xgt0_dn8 + (((locals.var_q_edge_xgt0_dn8 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn8)) / (2.0 * assign48010_e61542)))), (0.5 * (locals.var_q_edge_xgt0_dn9 + (((locals.var_q_edge_xgt0_dn9 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn9)) / (2.0 * assign48010_e61542)))), );
            locals.var_q_edge_xgt0e_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_21(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
            let assign48020_e61556: f64 = (locals.var_q_edge_xgt0e).ln();
            let assign48020_e61557: f64 = (locals.var_q_edge_n * assign48020_e61556);
            let assign48020_e61558: f64 = (locals.var_q_edge_xgt - assign48020_e61557);
            let assign48020_e61560: f64 = (assign48020_e61558 + locals.var_lngfedge2);
            (locals.var_q_edge_qi0si, locals.var_q_edge_qi0si_dn4, locals.var_q_edge_qi0si_dn6, locals.var_q_edge_qi0si_dn7, locals.var_q_edge_qi0si_dn8, locals.var_q_edge_qi0si_dn9, ) = (assign48020_e61560, ((locals.var_q_edge_xgt_dn4 - ((locals.var_q_edge_n_dn4 * assign48020_e61556) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn4 / locals.var_q_edge_xgt0e)))) + locals.var_lngfedge2_dn4), (locals.var_q_edge_xgt_dn6 - ((locals.var_q_edge_n_dn6 * assign48020_e61556) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn6 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn7 - ((locals.var_q_edge_n_dn7 * assign48020_e61556) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn7 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn8 - ((locals.var_q_edge_n_dn8 * assign48020_e61556) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn8 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn9 - ((locals.var_q_edge_n_dn9 * assign48020_e61556) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn9 / locals.var_q_edge_xgt0e)))), );
            locals.var_q_edge_qi0si_rv = 0.0;
        }

        if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
            let assign48030_e61573: f64 = (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si);
            let assign48030_e61575: f64 = (assign48030_e61573 + 2.0);
            let assign48030_e61576: f64 = (assign48030_e61575).sqrt();
            let assign48030_e61577: f64 = (locals.var_q_edge_qi0si + assign48030_e61576);
            let assign48030_e61578: f64 = (0.5 * assign48030_e61577);
            (locals.var_q_edge_qi0, locals.var_q_edge_qi0_dn4, locals.var_q_edge_qi0_dn6, locals.var_q_edge_qi0_dn7, locals.var_q_edge_qi0_dn8, locals.var_q_edge_qi0_dn9, ) = (assign48030_e61578, (0.5 * (locals.var_q_edge_qi0si_dn4 + (((locals.var_q_edge_qi0si_dn4 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn4)) / (2.0 * assign48030_e61576)))), (0.5 * (locals.var_q_edge_qi0si_dn6 + (((locals.var_q_edge_qi0si_dn6 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn6)) / (2.0 * assign48030_e61576)))), (0.5 * (locals.var_q_edge_qi0si_dn7 + (((locals.var_q_edge_qi0si_dn7 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn7)) / (2.0 * assign48030_e61576)))), (0.5 * (locals.var_q_edge_qi0si_dn8 + (((locals.var_q_edge_qi0si_dn8 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn8)) / (2.0 * assign48030_e61576)))), (0.5 * (locals.var_q_edge_qi0si_dn9 + (((locals.var_q_edge_qi0si_dn9 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn9)) / (2.0 * assign48030_e61576)))), );
            locals.var_q_edge_qi0_rv = 0.0;
        }

        let assign48040_e61583: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign48040_e61585: f64 = if assign48040_e61583 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1273 = assign48040_e61585;
        locals.var_guard1273_rv = 0.0;

        if ((((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) && (locals.var_guard1273 != 0.0)) {
            let assign48050_e61596: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
            let assign48050_e61597: f64 = (assign48050_e61596).exp();
            (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn4, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8, locals.var_q_edge_exp_x_dn9, ) = (assign48050_e61597, (assign48050_e61597 * (locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4)), (assign48050_e61597 * (locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6)), (assign48050_e61597 * (locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7)), (assign48050_e61597 * (locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8)), (assign48050_e61597 * (locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9)), );
            locals.var_q_edge_exp_x_rv = 0.0;
        }

        if ((((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) && (locals.var_guard1273 == 0.0)) {
            let assign48060_e61613: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
            let assign48060_e61615: f64 = (assign48060_e61613 - 230.25850929940458);
            let assign48060_e61620: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
            let assign48060_e61622: f64 = (assign48060_e61620 - 230.25850929940458);
            let assign48060_e61626: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
            let assign48060_e61628: f64 = (assign48060_e61626 - 230.25850929940458);
            let assign48060_e61630: f64 = (assign48060_e61628 * 0.3333333333333333);
            let assign48060_e61631: f64 = (1.0 + assign48060_e61630);
            let assign48060_e61632: f64 = (assign48060_e61622 * assign48060_e61631);
            let assign48060_e61633: f64 = (0.5 * assign48060_e61632);
            let assign48060_e61634: f64 = (1.0 + assign48060_e61633);
            let assign48060_e61635: f64 = (assign48060_e61615 * assign48060_e61634);
            let assign48060_e61636: f64 = (1.0 + assign48060_e61635);
            let assign48060_e61637: f64 = (1e100 * assign48060_e61636);
            (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn4, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8, locals.var_q_edge_exp_x_dn9, ) = (assign48060_e61637, (1e100 * (((locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4) * assign48060_e61634) + (assign48060_e61615 * (0.5 * (((locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4) * assign48060_e61631) + (assign48060_e61622 * ((locals.var_q_edge_xgt_dn4 - locals.var_q_edge_qi0_dn4) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign48060_e61634) + (assign48060_e61615 * (0.5 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign48060_e61631) + (assign48060_e61622 * ((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign48060_e61634) + (assign48060_e61615 * (0.5 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign48060_e61631) + (assign48060_e61622 * ((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign48060_e61634) + (assign48060_e61615 * (0.5 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign48060_e61631) + (assign48060_e61622 * ((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9) * assign48060_e61634) + (assign48060_e61615 * (0.5 * (((locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9) * assign48060_e61631) + (assign48060_e61622 * ((locals.var_q_edge_xgt_dn9 - locals.var_q_edge_qi0_dn9) * 0.3333333333333333))))))), );
            locals.var_q_edge_exp_x_rv = 0.0;
        }

        if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
            let assign48070_e61648: f64 = (locals.var_gfedge2 * locals.var_q_edge_exp_x);
            (locals.var_q_edge_d0, locals.var_q_edge_d0_dn4, locals.var_q_edge_d0_dn6, locals.var_q_edge_d0_dn7, locals.var_q_edge_d0_dn8, locals.var_q_edge_d0_dn9, ) = (assign48070_e61648, ((locals.var_gfedge2_dn4 * locals.var_q_edge_exp_x) + (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn4)), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn6), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn7), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn8), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn9), );
            locals.var_q_edge_d0_rv = 0.0;
        }

        if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
            let assign48080_e61659: f64 = (locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv);
            (locals.var_q_edge_d0p, locals.var_q_edge_d0p_dn4, locals.var_q_edge_d0p_dn6, locals.var_q_edge_d0p_dn7, locals.var_q_edge_d0p_dn8, locals.var_q_edge_d0p_dn9, ) = (assign48080_e61659, if locals.var_q_edge_n_inv_dn4 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn4)) } } else { (assign48080_e61659 * ((locals.var_q_edge_n_inv_dn4 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn4 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn6 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn6)) } } else { (assign48080_e61659 * ((locals.var_q_edge_n_inv_dn6 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn6 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn7 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn7)) } } else { (assign48080_e61659 * ((locals.var_q_edge_n_inv_dn7 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn7 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn8 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn8)) } } else { (assign48080_e61659 * ((locals.var_q_edge_n_inv_dn8 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn8 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn9 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn9)) } } else { (assign48080_e61659 * ((locals.var_q_edge_n_inv_dn9 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn9 / locals.var_q_edge_d0)))) }, );
            locals.var_q_edge_d0p_rv = 0.0;
        }

        if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
            let assign48090_e61670: f64 = (locals.var_q_edge_n * locals.var_q_edge_n);
            let assign48090_e61674: f64 = (locals.var_q_edge_qi0 + locals.var_q_edge_n);
            let assign48090_e61675: f64 = (2.0 * assign48090_e61674);
            let assign48090_e61677: f64 = (assign48090_e61675 - locals.var_q_edge_d0p);
            let assign48090_e61679: f64 = (assign48090_e61677 * locals.var_q_edge_d0p);
            let assign48090_e61680: f64 = (assign48090_e61670 + assign48090_e61679);
            (locals.var_q_edge_sqerr, locals.var_q_edge_sqerr_dn4, locals.var_q_edge_sqerr_dn6, locals.var_q_edge_sqerr_dn7, locals.var_q_edge_sqerr_dn8, locals.var_q_edge_sqerr_dn9, ) = (assign48090_e61680, (((locals.var_q_edge_n_dn4 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn4)) + ((((2.0 * (locals.var_q_edge_qi0_dn4 + locals.var_q_edge_n_dn4)) - locals.var_q_edge_d0p_dn4) * locals.var_q_edge_d0p) + (assign48090_e61677 * locals.var_q_edge_d0p_dn4))), (((locals.var_q_edge_n_dn6 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn6)) + ((((2.0 * (locals.var_q_edge_qi0_dn6 + locals.var_q_edge_n_dn6)) - locals.var_q_edge_d0p_dn6) * locals.var_q_edge_d0p) + (assign48090_e61677 * locals.var_q_edge_d0p_dn6))), (((locals.var_q_edge_n_dn7 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn7)) + ((((2.0 * (locals.var_q_edge_qi0_dn7 + locals.var_q_edge_n_dn7)) - locals.var_q_edge_d0p_dn7) * locals.var_q_edge_d0p) + (assign48090_e61677 * locals.var_q_edge_d0p_dn7))), (((locals.var_q_edge_n_dn8 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn8)) + ((((2.0 * (locals.var_q_edge_qi0_dn8 + locals.var_q_edge_n_dn8)) - locals.var_q_edge_d0p_dn8) * locals.var_q_edge_d0p) + (assign48090_e61677 * locals.var_q_edge_d0p_dn8))), (((locals.var_q_edge_n_dn9 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn9)) + ((((2.0 * (locals.var_q_edge_qi0_dn9 + locals.var_q_edge_n_dn9)) - locals.var_q_edge_d0p_dn9) * locals.var_q_edge_d0p) + (assign48090_e61677 * locals.var_q_edge_d0p_dn9))), );
            locals.var_q_edge_sqerr_rv = 0.0;
        }

        if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
            let assign48100_e61691: f64 = (locals.var_q_edge_sqerr).sqrt();
            let assign48100_e61693: f64 = (assign48100_e61691 - locals.var_q_edge_n);
            let assign48100_e61695: f64 = (assign48100_e61693 / locals.var_q_edge_d0p);
            let assign48100_e61697: f64 = (assign48100_e61695 - 1.0);
            let assign48100_e61698: f64 = (locals.var_q_edge_n * assign48100_e61697);
            (locals.var_q_edge_errq, locals.var_q_edge_errq_dn4, locals.var_q_edge_errq_dn6, locals.var_q_edge_errq_dn7, locals.var_q_edge_errq_dn8, locals.var_q_edge_errq_dn9, ) = (assign48100_e61698, ((locals.var_q_edge_n_dn4 * assign48100_e61697) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn4 / (2.0 * assign48100_e61691)) - locals.var_q_edge_n_dn4) * locals.var_q_edge_d0p) - (assign48100_e61693 * locals.var_q_edge_d0p_dn4)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn6 * assign48100_e61697) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn6 / (2.0 * assign48100_e61691)) - locals.var_q_edge_n_dn6) * locals.var_q_edge_d0p) - (assign48100_e61693 * locals.var_q_edge_d0p_dn6)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn7 * assign48100_e61697) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn7 / (2.0 * assign48100_e61691)) - locals.var_q_edge_n_dn7) * locals.var_q_edge_d0p) - (assign48100_e61693 * locals.var_q_edge_d0p_dn7)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn8 * assign48100_e61697) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn8 / (2.0 * assign48100_e61691)) - locals.var_q_edge_n_dn8) * locals.var_q_edge_d0p) - (assign48100_e61693 * locals.var_q_edge_d0p_dn8)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn9 * assign48100_e61697) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn9 / (2.0 * assign48100_e61691)) - locals.var_q_edge_n_dn9) * locals.var_q_edge_d0p) - (assign48100_e61693 * locals.var_q_edge_d0p_dn9)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), );
            locals.var_q_edge_errq_rv = 0.0;
        }

        if (((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 != 0.0)) {
            let assign48110_e61709: f64 = (locals.var_q_edge_qi0 - locals.var_q_edge_errq);
            (locals.var_qdeffedge, locals.var_qdeffedge_dn4, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8, locals.var_qdeffedge_dn9, ) = (assign48110_e61709, (locals.var_q_edge_qi0_dn4 - locals.var_q_edge_errq_dn4), (locals.var_q_edge_qi0_dn6 - locals.var_q_edge_errq_dn6), (locals.var_q_edge_qi0_dn7 - locals.var_q_edge_errq_dn7), (locals.var_q_edge_qi0_dn8 - locals.var_q_edge_errq_dn8), (locals.var_q_edge_qi0_dn9 - locals.var_q_edge_errq_dn9), );
            locals.var_qdeffedge_rv = 0.0;
        }

        let assign48120_e61715: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign48120_e61716: f64 = (locals.var_q_edge_n_inv * assign48120_e61715);
        let assign48120_e61718: f64 = (-230.25850929940458);
        let assign48120_e61719: f64 = if assign48120_e61716 > assign48120_e61718 { 1.0 } else { 0.0 };
        locals.var_guard1274 = assign48120_e61719;
        locals.var_guard1274_rv = 0.0;

        if ((((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 == 0.0)) && (locals.var_guard1274 != 0.0)) {
            let assign48130_e61732: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
            let assign48130_e61733: f64 = (locals.var_q_edge_n_inv * assign48130_e61732);
            let assign48130_e61734: f64 = (assign48130_e61733).exp();
            (locals.var_qdeffedge, locals.var_qdeffedge_dn4, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8, locals.var_qdeffedge_dn9, ) = (assign48130_e61734, (assign48130_e61734 * ((locals.var_q_edge_n_inv_dn4 * assign48130_e61732) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))), (assign48130_e61734 * ((locals.var_q_edge_n_inv_dn6 * assign48130_e61732) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))), (assign48130_e61734 * ((locals.var_q_edge_n_inv_dn7 * assign48130_e61732) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))), (assign48130_e61734 * ((locals.var_q_edge_n_inv_dn8 * assign48130_e61732) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))), (assign48130_e61734 * ((locals.var_q_edge_n_inv_dn9 * assign48130_e61732) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))), );
            locals.var_qdeffedge_rv = 0.0;
        }

        if ((((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) && (locals.var_guard1272 == 0.0)) && (locals.var_guard1274 == 0.0)) {
            let assign48140_e61750: f64 = (-230.25850929940458);
            let assign48140_e61754: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
            let assign48140_e61755: f64 = (locals.var_q_edge_n_inv * assign48140_e61754);
            let assign48140_e61756: f64 = (assign48140_e61750 - assign48140_e61755);
            let assign48140_e61760: f64 = (-230.25850929940458);
            let assign48140_e61764: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
            let assign48140_e61765: f64 = (locals.var_q_edge_n_inv * assign48140_e61764);
            let assign48140_e61766: f64 = (assign48140_e61760 - assign48140_e61765);
            let assign48140_e61769: f64 = (-230.25850929940458);
            let assign48140_e61773: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
            let assign48140_e61774: f64 = (locals.var_q_edge_n_inv * assign48140_e61773);
            let assign48140_e61775: f64 = (assign48140_e61769 - assign48140_e61774);
            let assign48140_e61777: f64 = (assign48140_e61775 * 0.3333333333333333);
            let assign48140_e61778: f64 = (1.0 + assign48140_e61777);
            let assign48140_e61779: f64 = (assign48140_e61766 * assign48140_e61778);
            let assign48140_e61780: f64 = (0.5 * assign48140_e61779);
            let assign48140_e61781: f64 = (1.0 + assign48140_e61780);
            let assign48140_e61782: f64 = (assign48140_e61756 * assign48140_e61781);
            let assign48140_e61783: f64 = (1.0 + assign48140_e61782);
            let assign48140_e61784: f64 = (1e-100 / assign48140_e61783);
            (locals.var_qdeffedge, locals.var_qdeffedge_dn4, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8, locals.var_qdeffedge_dn9, ) = (assign48140_e61784, (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn4 * assign48140_e61754) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))) * assign48140_e61781) + (assign48140_e61756 * (0.5 * (((-((locals.var_q_edge_n_inv_dn4 * assign48140_e61764) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))) * assign48140_e61778) + (assign48140_e61766 * ((-((locals.var_q_edge_n_inv_dn4 * assign48140_e61773) + (locals.var_q_edge_n_inv * (locals.var_q_edge_xgt_dn4 + locals.var_lngfedge2_dn4)))) * 0.3333333333333333))))))) / (assign48140_e61783 * assign48140_e61783))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn6 * assign48140_e61754) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign48140_e61781) + (assign48140_e61756 * (0.5 * (((-((locals.var_q_edge_n_inv_dn6 * assign48140_e61764) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign48140_e61778) + (assign48140_e61766 * ((-((locals.var_q_edge_n_inv_dn6 * assign48140_e61773) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * 0.3333333333333333))))))) / (assign48140_e61783 * assign48140_e61783))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn7 * assign48140_e61754) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign48140_e61781) + (assign48140_e61756 * (0.5 * (((-((locals.var_q_edge_n_inv_dn7 * assign48140_e61764) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign48140_e61778) + (assign48140_e61766 * ((-((locals.var_q_edge_n_inv_dn7 * assign48140_e61773) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * 0.3333333333333333))))))) / (assign48140_e61783 * assign48140_e61783))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn8 * assign48140_e61754) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign48140_e61781) + (assign48140_e61756 * (0.5 * (((-((locals.var_q_edge_n_inv_dn8 * assign48140_e61764) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign48140_e61778) + (assign48140_e61766 * ((-((locals.var_q_edge_n_inv_dn8 * assign48140_e61773) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * 0.3333333333333333))))))) / (assign48140_e61783 * assign48140_e61783))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn9 * assign48140_e61754) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))) * assign48140_e61781) + (assign48140_e61756 * (0.5 * (((-((locals.var_q_edge_n_inv_dn9 * assign48140_e61764) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))) * assign48140_e61778) + (assign48140_e61766 * ((-((locals.var_q_edge_n_inv_dn9 * assign48140_e61773) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn9))) * 0.3333333333333333))))))) / (assign48140_e61783 * assign48140_e61783))), );
            locals.var_qdeffedge_rv = 0.0;
        }

        if ((locals.var_guard1266 != 0.0) && (locals.var_guard1270 == 0.0)) {
            let assign48150_e61793: f64 = (locals.var_qdeffedge - locals.var_qseffedge);
            (locals.var_qdseffedge, locals.var_qdseffedge_dn4, locals.var_qdseffedge_dn6, locals.var_qdseffedge_dn7, locals.var_qdseffedge_dn8, locals.var_qdseffedge_dn9, ) = (assign48150_e61793, (locals.var_qdeffedge_dn4 - locals.var_qseffedge_dn4), (locals.var_qdeffedge_dn6 - locals.var_qseffedge_dn6), (locals.var_qdeffedge_dn7 - locals.var_qseffedge_dn7), (locals.var_qdeffedge_dn8 - locals.var_qseffedge_dn8), (locals.var_qdeffedge_dn9 - locals.var_qseffedge_dn9), );
            locals.var_qdseffedge_rv = 0.0;
        }

        if (locals.var_guard1266 != 0.0) {
            let assign48160_e61800: f64 = (locals.var_qdeffedge + locals.var_qseffedge);
            let assign48160_e61801: f64 = (0.5 * assign48160_e61800);
            (locals.var_qmeffedge, locals.var_qmeffedge_dn4, locals.var_qmeffedge_dn6, locals.var_qmeffedge_dn7, locals.var_qmeffedge_dn8, locals.var_qmeffedge_dn9, ) = (assign48160_e61801, (0.5 * (locals.var_qdeffedge_dn4 + locals.var_qseffedge_dn4)), (0.5 * (locals.var_qdeffedge_dn6 + locals.var_qseffedge_dn6)), (0.5 * (locals.var_qdeffedge_dn7 + locals.var_qseffedge_dn7)), (0.5 * (locals.var_qdeffedge_dn8 + locals.var_qseffedge_dn8)), (0.5 * (locals.var_qdeffedge_dn9 + locals.var_qseffedge_dn9)), );
            locals.var_qmeffedge_rv = 0.0;
        }

        if (locals.var_guard1266 != 0.0) {
            let assign48170_e61807: f64 = (locals.var_xgedge - locals.var_qmeffedge);
            let (assign48170_e61814, assign48170_e61814_d_n4, assign48170_e61814_d_n6, assign48170_e61814_d_n7, assign48170_e61814_d_n8, assign48170_e61814_d_n9,) = {
    if (assign48170_e61807 > 1e-40) {
        let assign48170_e61812: f64 = (locals.var_xgedge - locals.var_qmeffedge);
        (assign48170_e61812, (locals.var_xgedge_dn4 - locals.var_qmeffedge_dn4), (locals.var_xgedge_dn6 - locals.var_qmeffedge_dn6), (locals.var_xgedge_dn7 - locals.var_qmeffedge_dn7), (locals.var_xgedge_dn8 - locals.var_qmeffedge_dn8), (locals.var_xgedge_dn9 - locals.var_qmeffedge_dn9),)
    } else {
        (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
            (locals.var_dsqredge, locals.var_dsqredge_dn4, locals.var_dsqredge_dn6, locals.var_dsqredge_dn7, locals.var_dsqredge_dn8, locals.var_dsqredge_dn9, ) = (assign48170_e61814, assign48170_e61814_d_n4, assign48170_e61814_d_n6, assign48170_e61814_d_n7, assign48170_e61814_d_n8, assign48170_e61814_d_n9, );
            locals.var_dsqredge_rv = 0.0;
        }

        if (locals.var_guard1266 != 0.0) {
            let assign48180_e61821: f64 = (0.5 * locals.var_gfedge);
            let assign48180_e61825: f64 = (0.25 * locals.var_gfedge2);
            let assign48180_e61826: f64 = (locals.var_dsqredge + assign48180_e61825);
            let assign48180_e61827: f64 = (assign48180_e61826).sqrt();
            let assign48180_e61828: f64 = (assign48180_e61821 / assign48180_e61827);
            let assign48180_e61829: f64 = (1.0 - assign48180_e61828);
            (locals.var_alphabmedge, locals.var_alphabmedge_dn4, locals.var_alphabmedge_dn6, locals.var_alphabmedge_dn7, locals.var_alphabmedge_dn8, locals.var_alphabmedge_dn9, ) = (assign48180_e61829, (-((((0.5 * locals.var_gfedge_dn4) * assign48180_e61827) - (assign48180_e61821 * ((locals.var_dsqredge_dn4 + (0.25 * locals.var_gfedge2_dn4)) / (2.0 * assign48180_e61827)))) / (assign48180_e61827 * assign48180_e61827))), (-(-((assign48180_e61821 * (locals.var_dsqredge_dn6 / (2.0 * assign48180_e61827))) / (assign48180_e61827 * assign48180_e61827)))), (-(-((assign48180_e61821 * (locals.var_dsqredge_dn7 / (2.0 * assign48180_e61827))) / (assign48180_e61827 * assign48180_e61827)))), (-(-((assign48180_e61821 * (locals.var_dsqredge_dn8 / (2.0 * assign48180_e61827))) / (assign48180_e61827 * assign48180_e61827)))), (-(-((assign48180_e61821 * (locals.var_dsqredge_dn9 / (2.0 * assign48180_e61827))) / (assign48180_e61827 * assign48180_e61827)))), );
            locals.var_alphabmedge_rv = 0.0;
        }

        if (locals.var_guard1266 != 0.0) {
            let assign48190_e61834: f64 = (-locals.var_betedge_i);
            let assign48190_e61836: f64 = (assign48190_e61834 * locals.var_phit1edge);
            let assign48190_e61838: f64 = (assign48190_e61836 * locals.var_phit1edge);
            let assign48190_e61841: f64 = (locals.var_alphabmedge * locals.var_qmeffedge);
            let assign48190_e61843: f64 = (assign48190_e61841 + 1.0);
            let assign48190_e61844: f64 = (assign48190_e61838 * assign48190_e61843);
            let assign48190_e61846: f64 = (assign48190_e61844 * locals.var_qdseffedge);
            let assign48190_e61848: f64 = (assign48190_e61846 / locals.var_gmob_dc);
            (locals.var_i_dsedge, locals.var_i_dsedge_dn4, locals.var_i_dsedge_dn6, locals.var_i_dsedge_dn7, locals.var_i_dsedge_dn8, locals.var_i_dsedge_dn9, ) = (assign48190_e61848, ((((((((((((-locals.var_betedge_i_dn4) * locals.var_phit1edge) + (assign48190_e61834 * locals.var_phit1edge_dn4)) * locals.var_phit1edge) + (assign48190_e61836 * locals.var_phit1edge_dn4)) * assign48190_e61843) + (assign48190_e61838 * ((locals.var_alphabmedge_dn4 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn4)))) * locals.var_qdseffedge) + (assign48190_e61844 * locals.var_qdseffedge_dn4)) * locals.var_gmob_dc) - (assign48190_e61846 * locals.var_gmob_dc_dn4)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((((((assign48190_e61834 * locals.var_phit1edge_dn6) * locals.var_phit1edge) + (assign48190_e61836 * locals.var_phit1edge_dn6)) * assign48190_e61843) + (assign48190_e61838 * ((locals.var_alphabmedge_dn6 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn6)))) * locals.var_qdseffedge) + (assign48190_e61844 * locals.var_qdseffedge_dn6)) * locals.var_gmob_dc) - (assign48190_e61846 * locals.var_gmob_dc_dn6)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((((((assign48190_e61834 * locals.var_phit1edge_dn7) * locals.var_phit1edge) + (assign48190_e61836 * locals.var_phit1edge_dn7)) * assign48190_e61843) + (assign48190_e61838 * ((locals.var_alphabmedge_dn7 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn7)))) * locals.var_qdseffedge) + (assign48190_e61844 * locals.var_qdseffedge_dn7)) * locals.var_gmob_dc) - (assign48190_e61846 * locals.var_gmob_dc_dn7)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((((((assign48190_e61834 * locals.var_phit1edge_dn8) * locals.var_phit1edge) + (assign48190_e61836 * locals.var_phit1edge_dn8)) * assign48190_e61843) + (assign48190_e61838 * ((locals.var_alphabmedge_dn8 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn8)))) * locals.var_qdseffedge) + (assign48190_e61844 * locals.var_qdseffedge_dn8)) * locals.var_gmob_dc) - (assign48190_e61846 * locals.var_gmob_dc_dn8)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((((((assign48190_e61834 * locals.var_phit1edge_dn9) * locals.var_phit1edge) + (assign48190_e61836 * locals.var_phit1edge_dn9)) * assign48190_e61843) + (assign48190_e61838 * ((locals.var_alphabmedge_dn9 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn9)))) * locals.var_qdseffedge) + (assign48190_e61844 * locals.var_qdseffedge_dn9)) * locals.var_gmob_dc) - (assign48190_e61846 * locals.var_gmob_dc_dn9)) / (locals.var_gmob_dc * locals.var_gmob_dc)), );
            locals.var_i_dsedge_rv = 0.0;
        }

        (locals.var_mavl, locals.var_mavl_dn4, locals.var_mavl_dn6, locals.var_mavl_dn7, locals.var_mavl_dn8, locals.var_mavl_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_mavl_rv = 0.0;

        (locals.var_iimpact, locals.var_iimpact_dn4, locals.var_iimpact_dn6, locals.var_iimpact_dn7, locals.var_iimpact_dn8, locals.var_iimpact_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_iimpact_rv = 0.0;

        let assign48220_e61859: f64 = if ((locals.var_xg_dc > 0.0) && (p.p41 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1275 = assign48220_e61859;
        locals.var_guard1275_rv = 0.0;

        if (locals.var_guard1275 != 0.0) {
            let assign48230_e61864: f64 = (locals.var_a3_i * locals.var_dps_dc);
            let assign48230_e61865: f64 = (locals.var_v_ds - assign48230_e61864);
            (locals.var_delvsat, locals.var_delvsat_dn4, locals.var_delvsat_dn6, locals.var_delvsat_dn7, locals.var_delvsat_dn8, locals.var_delvsat_dn9, ) = (assign48230_e61865, (-(locals.var_a3_i * locals.var_dps_dc_dn4)), (-(locals.var_a3_i * locals.var_dps_dc_dn6)), (locals.var_v_ds_dn7 - (locals.var_a3_i * locals.var_dps_dc_dn7)), (locals.var_v_ds_dn8 - (locals.var_a3_i * locals.var_dps_dc_dn8)), (-(locals.var_a3_i * locals.var_dps_dc_dn9)), );
            locals.var_delvsat_rv = 0.0;
        }

        let assign48240_e61870: f64 = if locals.var_delvsat > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1276 = assign48240_e61870;
        locals.var_guard1276_rv = 0.0;

        if ((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) {
            let assign48250_e61879: f64 = (locals.var_phib_dc + locals.var_vsbstar_dc);
            let assign48250_e61880: f64 = (assign48250_e61879).sqrt();
            let assign48250_e61882: f64 = (assign48250_e61880 - locals.var_sqrt_phib_dc);
            let assign48250_e61883: f64 = (locals.var_a4_i * assign48250_e61882);
            let assign48250_e61884: f64 = (1.0 + assign48250_e61883);
            let assign48250_e61887: f64 = (locals.var_delvsat + 1e-30);
            let assign48250_e61888: f64 = (assign48250_e61884 / assign48250_e61887);
            let assign48250_e61889: f64 = (locals.var_a2_t * assign48250_e61888);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign48250_e61889, ((locals.var_a2_t_dn4 * assign48250_e61888) + (locals.var_a2_t * ((((locals.var_a4_i * (((locals.var_phib_dc_dn4 + locals.var_vsbstar_dc_dn4) / (2.0 * assign48250_e61880)) - locals.var_sqrt_phib_dc_dn4)) * assign48250_e61887) - (assign48250_e61884 * locals.var_delvsat_dn4)) / (assign48250_e61887 * assign48250_e61887)))), (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn6 / (2.0 * assign48250_e61880))) * assign48250_e61887) - (assign48250_e61884 * locals.var_delvsat_dn6)) / (assign48250_e61887 * assign48250_e61887))), (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn7 / (2.0 * assign48250_e61880))) * assign48250_e61887) - (assign48250_e61884 * locals.var_delvsat_dn7)) / (assign48250_e61887 * assign48250_e61887))), (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn8 / (2.0 * assign48250_e61880))) * assign48250_e61887) - (assign48250_e61884 * locals.var_delvsat_dn8)) / (assign48250_e61887 * assign48250_e61887))), (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn9 / (2.0 * assign48250_e61880))) * assign48250_e61887) - (assign48250_e61884 * locals.var_delvsat_dn9)) / (assign48250_e61887 * assign48250_e61887))), );
            locals.var_temp2_rv = 0.0;
        }

        let assign48260_e61893: f64 = (-locals.var_temp2);
        let assign48260_e61894: f64 = (assign48260_e61893).abs();
        let assign48260_e61896: f64 = if assign48260_e61894 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1277 = assign48260_e61896;
        locals.var_guard1277_rv = 0.0;

        if (((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) && (locals.var_guard1277 != 0.0)) {
            let assign48270_e61903: f64 = (-locals.var_temp2);
            let assign48270_e61904: f64 = (assign48270_e61903).exp();
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign48270_e61904, (assign48270_e61904 * (-locals.var_temp2_dn4)), (assign48270_e61904 * (-locals.var_temp2_dn6)), (assign48270_e61904 * (-locals.var_temp2_dn7)), (assign48270_e61904 * (-locals.var_temp2_dn8)), (assign48270_e61904 * (-locals.var_temp2_dn9)), );
            locals.var_temp__blk949_rv = 0.0;
        }

        let assign48280_e61908: f64 = (-locals.var_temp2);
        let assign48280_e61910: f64 = if assign48280_e61908 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1278 = assign48280_e61910;
        locals.var_guard1278_rv = 0.0;

        if ((((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) && (locals.var_guard1277 == 0.0)) && (locals.var_guard1278 != 0.0)) {
            let assign48290_e61922: f64 = (-230.25850929940458);
            let assign48290_e61924: f64 = (-locals.var_temp2);
            let assign48290_e61925: f64 = (assign48290_e61922 - assign48290_e61924);
            let assign48290_e61929: f64 = (-230.25850929940458);
            let assign48290_e61931: f64 = (-locals.var_temp2);
            let assign48290_e61932: f64 = (assign48290_e61929 - assign48290_e61931);
            let assign48290_e61935: f64 = (-230.25850929940458);
            let assign48290_e61937: f64 = (-locals.var_temp2);
            let assign48290_e61938: f64 = (assign48290_e61935 - assign48290_e61937);
            let assign48290_e61940: f64 = (assign48290_e61938 * 0.3333333333333333);
            let assign48290_e61941: f64 = (1.0 + assign48290_e61940);
            let assign48290_e61942: f64 = (assign48290_e61932 * assign48290_e61941);
            let assign48290_e61943: f64 = (0.5 * assign48290_e61942);
            let assign48290_e61944: f64 = (1.0 + assign48290_e61943);
            let assign48290_e61945: f64 = (assign48290_e61925 * assign48290_e61944);
            let assign48290_e61946: f64 = (1.0 + assign48290_e61945);
            let assign48290_e61947: f64 = (1e-100 / assign48290_e61946);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign48290_e61947, (-((1e-100 * (((-(-locals.var_temp2_dn4)) * assign48290_e61944) + (assign48290_e61925 * (0.5 * (((-(-locals.var_temp2_dn4)) * assign48290_e61941) + (assign48290_e61932 * ((-(-locals.var_temp2_dn4)) * 0.3333333333333333))))))) / (assign48290_e61946 * assign48290_e61946))), (-((1e-100 * (((-(-locals.var_temp2_dn6)) * assign48290_e61944) + (assign48290_e61925 * (0.5 * (((-(-locals.var_temp2_dn6)) * assign48290_e61941) + (assign48290_e61932 * ((-(-locals.var_temp2_dn6)) * 0.3333333333333333))))))) / (assign48290_e61946 * assign48290_e61946))), (-((1e-100 * (((-(-locals.var_temp2_dn7)) * assign48290_e61944) + (assign48290_e61925 * (0.5 * (((-(-locals.var_temp2_dn7)) * assign48290_e61941) + (assign48290_e61932 * ((-(-locals.var_temp2_dn7)) * 0.3333333333333333))))))) / (assign48290_e61946 * assign48290_e61946))), (-((1e-100 * (((-(-locals.var_temp2_dn8)) * assign48290_e61944) + (assign48290_e61925 * (0.5 * (((-(-locals.var_temp2_dn8)) * assign48290_e61941) + (assign48290_e61932 * ((-(-locals.var_temp2_dn8)) * 0.3333333333333333))))))) / (assign48290_e61946 * assign48290_e61946))), (-((1e-100 * (((-(-locals.var_temp2_dn9)) * assign48290_e61944) + (assign48290_e61925 * (0.5 * (((-(-locals.var_temp2_dn9)) * assign48290_e61941) + (assign48290_e61932 * ((-(-locals.var_temp2_dn9)) * 0.3333333333333333))))))) / (assign48290_e61946 * assign48290_e61946))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) && (locals.var_guard1277 == 0.0)) && (locals.var_guard1278 == 0.0)) {
            let assign48300_e61962: f64 = (-locals.var_temp2);
            let assign48300_e61964: f64 = (assign48300_e61962 - 230.25850929940458);
            let assign48300_e61968: f64 = (-locals.var_temp2);
            let assign48300_e61970: f64 = (assign48300_e61968 - 230.25850929940458);
            let assign48300_e61973: f64 = (-locals.var_temp2);
            let assign48300_e61975: f64 = (assign48300_e61973 - 230.25850929940458);
            let assign48300_e61977: f64 = (assign48300_e61975 * 0.3333333333333333);
            let assign48300_e61978: f64 = (1.0 + assign48300_e61977);
            let assign48300_e61979: f64 = (assign48300_e61970 * assign48300_e61978);
            let assign48300_e61980: f64 = (0.5 * assign48300_e61979);
            let assign48300_e61981: f64 = (1.0 + assign48300_e61980);
            let assign48300_e61982: f64 = (assign48300_e61964 * assign48300_e61981);
            let assign48300_e61983: f64 = (1.0 + assign48300_e61982);
            let assign48300_e61984: f64 = (1e100 * assign48300_e61983);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign48300_e61984, (1e100 * (((-locals.var_temp2_dn4) * assign48300_e61981) + (assign48300_e61964 * (0.5 * (((-locals.var_temp2_dn4) * assign48300_e61978) + (assign48300_e61970 * ((-locals.var_temp2_dn4) * 0.3333333333333333))))))), (1e100 * (((-locals.var_temp2_dn6) * assign48300_e61981) + (assign48300_e61964 * (0.5 * (((-locals.var_temp2_dn6) * assign48300_e61978) + (assign48300_e61970 * ((-locals.var_temp2_dn6) * 0.3333333333333333))))))), (1e100 * (((-locals.var_temp2_dn7) * assign48300_e61981) + (assign48300_e61964 * (0.5 * (((-locals.var_temp2_dn7) * assign48300_e61978) + (assign48300_e61970 * ((-locals.var_temp2_dn7) * 0.3333333333333333))))))), (1e100 * (((-locals.var_temp2_dn8) * assign48300_e61981) + (assign48300_e61964 * (0.5 * (((-locals.var_temp2_dn8) * assign48300_e61978) + (assign48300_e61970 * ((-locals.var_temp2_dn8) * 0.3333333333333333))))))), (1e100 * (((-locals.var_temp2_dn9) * assign48300_e61981) + (assign48300_e61964 * (0.5 * (((-locals.var_temp2_dn9) * assign48300_e61978) + (assign48300_e61970 * ((-locals.var_temp2_dn9) * 0.3333333333333333))))))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) {
            let assign48310_e61993: f64 = (locals.var_delvsat * locals.var_temp__blk949);
            let assign48310_e61994: f64 = (locals.var_a1_i * assign48310_e61993);
            (locals.var_mavl, locals.var_mavl_dn4, locals.var_mavl_dn6, locals.var_mavl_dn7, locals.var_mavl_dn8, locals.var_mavl_dn9, ) = (assign48310_e61994, (locals.var_a1_i * ((locals.var_delvsat_dn4 * locals.var_temp__blk949) + (locals.var_delvsat * locals.var_temp__blk949_dn4))), (locals.var_a1_i * ((locals.var_delvsat_dn6 * locals.var_temp__blk949) + (locals.var_delvsat * locals.var_temp__blk949_dn6))), (locals.var_a1_i * ((locals.var_delvsat_dn7 * locals.var_temp__blk949) + (locals.var_delvsat * locals.var_temp__blk949_dn7))), (locals.var_a1_i * ((locals.var_delvsat_dn8 * locals.var_temp__blk949) + (locals.var_delvsat * locals.var_temp__blk949_dn8))), (locals.var_a1_i * ((locals.var_delvsat_dn9 * locals.var_temp__blk949) + (locals.var_delvsat * locals.var_temp__blk949_dn9))), );
            locals.var_mavl_rv = 0.0;
        }

        if ((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) {
            let assign48320_e62003: f64 = (locals.var_i_ds + locals.var_i_dsedge);
            let assign48320_e62004: f64 = (locals.var_mavl * assign48320_e62003);
            (locals.var_iimpact, locals.var_iimpact_dn4, locals.var_iimpact_dn6, locals.var_iimpact_dn7, locals.var_iimpact_dn8, locals.var_iimpact_dn9, ) = (assign48320_e62004, ((locals.var_mavl_dn4 * assign48320_e62003) + (locals.var_mavl * (locals.var_i_ds_dn4 + locals.var_i_dsedge_dn4))), ((locals.var_mavl_dn6 * assign48320_e62003) + (locals.var_mavl * (locals.var_i_ds_dn6 + locals.var_i_dsedge_dn6))), ((locals.var_mavl_dn7 * assign48320_e62003) + (locals.var_mavl * (locals.var_i_ds_dn7 + locals.var_i_dsedge_dn7))), ((locals.var_mavl_dn8 * assign48320_e62003) + (locals.var_mavl * (locals.var_i_ds_dn8 + locals.var_i_dsedge_dn8))), ((locals.var_mavl_dn9 * assign48320_e62003) + (locals.var_mavl * (locals.var_i_ds_dn9 + locals.var_i_dsedge_dn9))), );
            locals.var_iimpact_rv = 0.0;
        }

        let assign48330_e62010: f64 = (0.5 * locals.var_imaxii_i);
        let assign48330_e62011: f64 = if locals.var_iimpact > assign48330_e62010 { 1.0 } else { 0.0 };
        locals.var_guard1279 = assign48330_e62011;
        locals.var_guard1279_rv = 0.0;

        if (((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) && (locals.var_guard1279 != 0.0)) {
            let assign48340_e62019: f64 = (2.0 * locals.var_iimpact);
            let assign48340_e62021: f64 = (assign48340_e62019 / locals.var_imaxii_i);
            let assign48340_e62023: f64 = (assign48340_e62021 - 1.0);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign48340_e62023, ((2.0 * locals.var_iimpact_dn4) / locals.var_imaxii_i), ((2.0 * locals.var_iimpact_dn6) / locals.var_imaxii_i), ((2.0 * locals.var_iimpact_dn7) / locals.var_imaxii_i), ((2.0 * locals.var_iimpact_dn8) / locals.var_imaxii_i), ((2.0 * locals.var_iimpact_dn9) / locals.var_imaxii_i), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if (((locals.var_guard1275 != 0.0) && (locals.var_guard1276 != 0.0)) && (locals.var_guard1279 != 0.0)) {
            let assign48350_e62033: f64 = (0.5 * locals.var_imaxii_i);
            let assign48350_e62039: f64 = (locals.var_temp__blk949 * locals.var_temp__blk949);
            let assign48350_e62040: f64 = (1.0 + assign48350_e62039);
            let assign48350_e62041: f64 = (assign48350_e62040).sqrt();
            let assign48350_e62042: f64 = (locals.var_temp__blk949 / assign48350_e62041);
            let assign48350_e62043: f64 = (1.0 + assign48350_e62042);
            let assign48350_e62044: f64 = (assign48350_e62033 * assign48350_e62043);
            (locals.var_iimpact, locals.var_iimpact_dn4, locals.var_iimpact_dn6, locals.var_iimpact_dn7, locals.var_iimpact_dn8, locals.var_iimpact_dn9, ) = (assign48350_e62044, (assign48350_e62033 * (((locals.var_temp__blk949_dn4 * assign48350_e62041) - (locals.var_temp__blk949 * (((locals.var_temp__blk949_dn4 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn4)) / (2.0 * assign48350_e62041)))) / (assign48350_e62041 * assign48350_e62041))), (assign48350_e62033 * (((locals.var_temp__blk949_dn6 * assign48350_e62041) - (locals.var_temp__blk949 * (((locals.var_temp__blk949_dn6 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn6)) / (2.0 * assign48350_e62041)))) / (assign48350_e62041 * assign48350_e62041))), (assign48350_e62033 * (((locals.var_temp__blk949_dn7 * assign48350_e62041) - (locals.var_temp__blk949 * (((locals.var_temp__blk949_dn7 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn7)) / (2.0 * assign48350_e62041)))) / (assign48350_e62041 * assign48350_e62041))), (assign48350_e62033 * (((locals.var_temp__blk949_dn8 * assign48350_e62041) - (locals.var_temp__blk949 * (((locals.var_temp__blk949_dn8 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn8)) / (2.0 * assign48350_e62041)))) / (assign48350_e62041 * assign48350_e62041))), (assign48350_e62033 * (((locals.var_temp__blk949_dn9 * assign48350_e62041) - (locals.var_temp__blk949 * (((locals.var_temp__blk949_dn9 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn9)) / (2.0 * assign48350_e62041)))) / (assign48350_e62041 * assign48350_e62041))), );
            locals.var_iimpact_rv = 0.0;
        }

        let assign48360_e62057: f64 = if (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1473 = assign48360_e62057;
        locals.var_guard1473_rv = 0.0;

        let assign48370_e62064: f64 = if ((p.p45 > 0.0) || (p.p47 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1474 = assign48370_e62064;
        locals.var_guard1474_rv = 0.0;

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            (locals.var_phib__blk1314, locals.var_phib__blk1314_dn4, ) = (locals.var_phib_dc, locals.var_phib_dc_dn4, );
            locals.var_phib__blk1314_rv = 0.0;
            (locals.var_aphi__blk1315, locals.var_aphi__blk1315_dn4, ) = (locals.var_aphi_dc, locals.var_aphi_dc_dn4, );
            locals.var_aphi__blk1315_rv = 0.0;
            (locals.var_g_0__blk1316, locals.var_g_0__blk1316_dn4, ) = (locals.var_g_0_dc, locals.var_g_0_dc_dn4, );
            locals.var_g_0__blk1316_rv = 0.0;
            (locals.var_v_xb__blk1317, locals.var_v_xb__blk1317_dn4, locals.var_v_xb__blk1317_dn7, locals.var_v_xb__blk1317_dn8, locals.var_v_xb__blk1317_dn9, ) = (locals.var_v_xb_dc_tmp, locals.var_v_xb_dc_tmp_dn4, locals.var_v_xb_dc_tmp_dn7, locals.var_v_xb_dc_tmp_dn8, locals.var_v_xb_dc_tmp_dn9, );
            locals.var_v_xb__blk1317_rv = 0.0;
            (locals.var_vsbstar__blk1318, locals.var_vsbstar__blk1318_dn4, locals.var_vsbstar__blk1318_dn6, locals.var_vsbstar__blk1318_dn7, locals.var_vsbstar__blk1318_dn8, locals.var_vsbstar__blk1318_dn9, ) = (locals.var_vsbstar_dc_tmp, locals.var_vsbstar_dc_tmp_dn4, locals.var_vsbstar_dc_tmp_dn6, locals.var_vsbstar_dc_tmp_dn7, locals.var_vsbstar_dc_tmp_dn8, locals.var_vsbstar_dc_tmp_dn9, );
            locals.var_vsbstar__blk1318_rv = 0.0;
            locals.var_dvbstar__blk1322 = 0.0;
            locals.var_dvbstar__blk1322_rv = 0.0;
        }

        let assign48440_e62103: f64 = if p.p47 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1475 = assign48440_e62103;
        locals.var_guard1475_rv = 0.0;

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
            let assign48450_e62112: f64 = (locals.var_v_db + locals.var_v_sb);
            let assign48450_e62115: f64 = (locals.var_v_db - locals.var_v_sb);
            let assign48450_e62118: f64 = (locals.var_v_db - locals.var_v_sb);
            let assign48450_e62119: f64 = (assign48450_e62115 * assign48450_e62118);
            let assign48450_e62121: f64 = (assign48450_e62119 + locals.var_bphi_ac);
            let assign48450_e62122: f64 = (assign48450_e62121).sqrt();
            let assign48450_e62123: f64 = (assign48450_e62112 - assign48450_e62122);
            let assign48450_e62124: f64 = (0.5 * assign48450_e62123);
            let assign48450_e62126: f64 = (assign48450_e62124 + locals.var_phix_ac);
            (locals.var_v_xb__blk1317, locals.var_v_xb__blk1317_dn4, locals.var_v_xb__blk1317_dn7, locals.var_v_xb__blk1317_dn8, locals.var_v_xb__blk1317_dn9, ) = (assign48450_e62126, ((0.5 * (-(locals.var_bphi_ac_dn4 / (2.0 * assign48450_e62122)))) + locals.var_phix_ac_dn4), (0.5 * ((locals.var_v_db_dn7 + locals.var_v_sb_dn7) - ((((locals.var_v_db_dn7 - locals.var_v_sb_dn7) * assign48450_e62118) + (assign48450_e62115 * (locals.var_v_db_dn7 - locals.var_v_sb_dn7))) / (2.0 * assign48450_e62122)))), (0.5 * ((locals.var_v_db_dn8 + locals.var_v_sb_dn8) - ((((locals.var_v_db_dn8 - locals.var_v_sb_dn8) * assign48450_e62118) + (assign48450_e62115 * (locals.var_v_db_dn8 - locals.var_v_sb_dn8))) / (2.0 * assign48450_e62122)))), (0.5 * ((locals.var_v_db_dn9 + locals.var_v_sb_dn9) - ((((locals.var_v_db_dn9 - locals.var_v_sb_dn9) * assign48450_e62118) + (assign48450_e62115 * (locals.var_v_db_dn9 - locals.var_v_sb_dn9))) / (2.0 * assign48450_e62122)))), );
            locals.var_v_xb__blk1317_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
            let assign48460_e62138: f64 = locals.var_v_xb__blk1317;
            let assign48460_e62141: f64 = locals.var_v_xb__blk1317;
            let assign48460_e62144: f64 = locals.var_v_xb__blk1317;
            let assign48460_e62145: f64 = (assign48460_e62141 * assign48460_e62144);
            let assign48460_e62147: f64 = (assign48460_e62145 + locals.var_aphi_ac);
            let assign48460_e62148: f64 = (assign48460_e62147).sqrt();
            let assign48460_e62149: f64 = (assign48460_e62138 - assign48460_e62148);
            let assign48460_e62150: f64 = (0.5 * assign48460_e62149);
            let assign48460_e62151: f64 = (locals.var_v_sb - assign48460_e62150);
            let assign48460_e62153: f64 = (assign48460_e62151 + locals.var_phix1_ac);
            (locals.var_vsbstar_ac, locals.var_vsbstar_ac_dn4, locals.var_vsbstar_ac_dn7, locals.var_vsbstar_ac_dn8, locals.var_vsbstar_ac_dn9, ) = (assign48460_e62153, ((-(0.5 * (locals.var_v_xb__blk1317_dn4 - ((((locals.var_v_xb__blk1317_dn4 * assign48460_e62144) + (assign48460_e62141 * locals.var_v_xb__blk1317_dn4)) + locals.var_aphi_ac_dn4) / (2.0 * assign48460_e62148))))) + locals.var_phix1_ac_dn4), (locals.var_v_sb_dn7 - (0.5 * (locals.var_v_xb__blk1317_dn7 - (((locals.var_v_xb__blk1317_dn7 * assign48460_e62144) + (assign48460_e62141 * locals.var_v_xb__blk1317_dn7)) / (2.0 * assign48460_e62148))))), (locals.var_v_sb_dn8 - (0.5 * (locals.var_v_xb__blk1317_dn8 - (((locals.var_v_xb__blk1317_dn8 * assign48460_e62144) + (assign48460_e62141 * locals.var_v_xb__blk1317_dn8)) / (2.0 * assign48460_e62148))))), (locals.var_v_sb_dn9 - (0.5 * (locals.var_v_xb__blk1317_dn9 - (((locals.var_v_xb__blk1317_dn9 * assign48460_e62144) + (assign48460_e62141 * locals.var_v_xb__blk1317_dn9)) / (2.0 * assign48460_e62148))))), );
            locals.var_vsbstar_ac_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
            (locals.var_vsbstar__blk1318, locals.var_vsbstar__blk1318_dn4, locals.var_vsbstar__blk1318_dn6, locals.var_vsbstar__blk1318_dn7, locals.var_vsbstar__blk1318_dn8, locals.var_vsbstar__blk1318_dn9, ) = (locals.var_vsbstar_ac, locals.var_vsbstar_ac_dn4, 0.0, locals.var_vsbstar_ac_dn7, locals.var_vsbstar_ac_dn8, locals.var_vsbstar_ac_dn9, );
            locals.var_vsbstar__blk1318_rv = 0.0;
            (locals.var_phib__blk1314, locals.var_phib__blk1314_dn4, ) = (locals.var_phib_ac, locals.var_phib_ac_dn4, );
            locals.var_phib__blk1314_rv = 0.0;
            (locals.var_aphi__blk1315, locals.var_aphi__blk1315_dn4, ) = (locals.var_aphi_ac, locals.var_aphi_ac_dn4, );
            locals.var_aphi__blk1315_rv = 0.0;
            (locals.var_g_0__blk1316, locals.var_g_0__blk1316_dn4, ) = (locals.var_g_0_ac, locals.var_g_0_ac_dn4, );
            locals.var_g_0__blk1316_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign48510_e62193: f64 = (locals.var_vgb - locals.var_dvbstar__blk1322);
            let assign48510_e62195: f64 = (assign48510_e62193 - locals.var_vfb_t);
            (locals.var_vgb1__blk1321, locals.var_vgb1__blk1321_dn4, locals.var_vgb1__blk1321_dn6, locals.var_vgb1__blk1321_dn7, locals.var_vgb1__blk1321_dn8, locals.var_vgb1__blk1321_dn9, ) = (assign48510_e62195, (-locals.var_vfb_t_dn4), locals.var_vgb_dn6, locals.var_vgb_dn7, locals.var_vgb_dn8, locals.var_vgb_dn9, );
            locals.var_vgb1__blk1321_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign48520_e62205: f64 = (locals.var_v_ds - locals.var_vdsx);
            let assign48520_e62206: f64 = (0.5 * assign48520_e62205);
            let assign48520_e62207: f64 = (locals.var_vsbstar__blk1318 + assign48520_e62206);
            (locals.var_vsbx__blk1323, locals.var_vsbx__blk1323_dn4, locals.var_vsbx__blk1323_dn6, locals.var_vsbx__blk1323_dn7, locals.var_vsbx__blk1323_dn8, locals.var_vsbx__blk1323_dn9, ) = (assign48520_e62207, locals.var_vsbstar__blk1318_dn4, locals.var_vsbstar__blk1318_dn6, (locals.var_vsbstar__blk1318_dn7 + (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7))), (locals.var_vsbstar__blk1318_dn8 + (0.5 * (locals.var_v_ds_dn8 - locals.var_vdsx_dn8))), locals.var_vsbstar__blk1318_dn9, );
            locals.var_vsbx__blk1323_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            (locals.var_dctg__blk1335, locals.var_dctg__blk1335_dn4, locals.var_dctg__blk1335_dn6, locals.var_dctg__blk1335_dn7, locals.var_dctg__blk1335_dn8, locals.var_dctg__blk1335_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_dctg__blk1335_rv = 0.0;
        }

        let assign48540_e62218: f64 = if locals.var_ctg_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1476 = assign48540_e62218;
        locals.var_guard1476_rv = 0.0;

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
            let assign48550_e62226: f64 = (locals.var_phib__blk1314 * locals.var_inv_phit);
            (locals.var_xbct__blk1326, locals.var_xbct__blk1326_dn4, ) = (assign48550_e62226, ((locals.var_phib__blk1314_dn4 * locals.var_inv_phit) + (locals.var_phib__blk1314 * locals.var_inv_phit_dn4)), );
            locals.var_xbct__blk1326_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
            let assign48560_e62236: f64 = (locals.var_vsbx__blk1323 * locals.var_inv_phit);
            (locals.var_xsbstar__blk1327, locals.var_xsbstar__blk1327_dn4, locals.var_xsbstar__blk1327_dn6, locals.var_xsbstar__blk1327_dn7, locals.var_xsbstar__blk1327_dn8, locals.var_xsbstar__blk1327_dn9, ) = (assign48560_e62236, ((locals.var_vsbx__blk1323_dn4 * locals.var_inv_phit) + (locals.var_vsbx__blk1323 * locals.var_inv_phit_dn4)), (locals.var_vsbx__blk1323_dn6 * locals.var_inv_phit), (locals.var_vsbx__blk1323_dn7 * locals.var_inv_phit), (locals.var_vsbx__blk1323_dn8 * locals.var_inv_phit), (locals.var_vsbx__blk1323_dn9 * locals.var_inv_phit), );
            locals.var_xsbstar__blk1327_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
            let assign48570_e62246: f64 = (locals.var_vgb1__blk1321 * locals.var_inv_phit);
            (locals.var_xgct__blk1328, locals.var_xgct__blk1328_dn4, locals.var_xgct__blk1328_dn6, locals.var_xgct__blk1328_dn7, locals.var_xgct__blk1328_dn8, locals.var_xgct__blk1328_dn9, ) = (assign48570_e62246, ((locals.var_vgb1__blk1321_dn4 * locals.var_inv_phit) + (locals.var_vgb1__blk1321 * locals.var_inv_phit_dn4)), (locals.var_vgb1__blk1321_dn6 * locals.var_inv_phit), (locals.var_vgb1__blk1321_dn7 * locals.var_inv_phit), (locals.var_vgb1__blk1321_dn8 * locals.var_inv_phit), (locals.var_vgb1__blk1321_dn9 * locals.var_inv_phit), );
            locals.var_xgct__blk1328_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
            let assign48580_e62257: f64 = (0.5 * locals.var_g_0__blk1316);
            let assign48580_e62259: f64 = (locals.var_xbct__blk1326).sqrt();
            let assign48580_e62260: f64 = (assign48580_e62257 / assign48580_e62259);
            let assign48580_e62261: f64 = (1.0 + assign48580_e62260);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign48580_e62261, ((((0.5 * locals.var_g_0__blk1316_dn4) * assign48580_e62259) - (assign48580_e62257 * (locals.var_xbct__blk1326_dn4 / (2.0 * assign48580_e62259)))) / (assign48580_e62259 * assign48580_e62259)), 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp1_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
            let assign48590_e62272: f64 = (locals.var_xbct__blk1326).sqrt();
            let assign48590_e62273: f64 = (locals.var_g_0__blk1316 * assign48590_e62272);
            let assign48590_e62274: f64 = (locals.var_xbct__blk1326 + assign48590_e62273);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign48590_e62274, (locals.var_xbct__blk1326_dn4 + ((locals.var_g_0__blk1316_dn4 * assign48590_e62272) + (locals.var_g_0__blk1316 * (locals.var_xbct__blk1326_dn4 / (2.0 * assign48590_e62272))))), 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp2_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
            let assign48600_e62284: f64 = (locals.var_xgct__blk1328 - locals.var_temp2);
            let assign48600_e62286: f64 = (assign48600_e62284 / locals.var_temp1);
            let assign48600_e62289: f64 = (0.5 * locals.var_xbct__blk1326);
            let assign48600_e62290: f64 = (assign48600_e62286 + assign48600_e62289);
            let assign48600_e62293: f64 = (1.0 + locals.var_ctb_i);
            let assign48600_e62295: f64 = (assign48600_e62293 * locals.var_xsbstar__blk1327);
            let assign48600_e62296: f64 = (assign48600_e62290 - assign48600_e62295);
            (locals.var_xwict__blk1329, locals.var_xwict__blk1329_dn4, locals.var_xwict__blk1329_dn6, locals.var_xwict__blk1329_dn7, locals.var_xwict__blk1329_dn8, locals.var_xwict__blk1329_dn9, ) = (assign48600_e62296, ((((((locals.var_xgct__blk1328_dn4 - locals.var_temp2_dn4) * locals.var_temp1) - (assign48600_e62284 * locals.var_temp1_dn4)) / (locals.var_temp1 * locals.var_temp1)) + (0.5 * locals.var_xbct__blk1326_dn4)) - (assign48600_e62293 * locals.var_xsbstar__blk1327_dn4)), (((((locals.var_xgct__blk1328_dn6 - locals.var_temp2_dn6) * locals.var_temp1) - (assign48600_e62284 * locals.var_temp1_dn6)) / (locals.var_temp1 * locals.var_temp1)) - (assign48600_e62293 * locals.var_xsbstar__blk1327_dn6)), (((((locals.var_xgct__blk1328_dn7 - locals.var_temp2_dn7) * locals.var_temp1) - (assign48600_e62284 * locals.var_temp1_dn7)) / (locals.var_temp1 * locals.var_temp1)) - (assign48600_e62293 * locals.var_xsbstar__blk1327_dn7)), (((((locals.var_xgct__blk1328_dn8 - locals.var_temp2_dn8) * locals.var_temp1) - (assign48600_e62284 * locals.var_temp1_dn8)) / (locals.var_temp1 * locals.var_temp1)) - (assign48600_e62293 * locals.var_xsbstar__blk1327_dn8)), (((((locals.var_xgct__blk1328_dn9 - locals.var_temp2_dn9) * locals.var_temp1) - (assign48600_e62284 * locals.var_temp1_dn9)) / (locals.var_temp1 * locals.var_temp1)) - (assign48600_e62293 * locals.var_xsbstar__blk1327_dn9)), );
            locals.var_xwict__blk1329_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
            let assign48610_e62306: f64 = (0.5 * locals.var_xbct__blk1326);
            let assign48610_e62308: f64 = (assign48610_e62306 + 2.0);
            (locals.var_xctmax__blk1330, locals.var_xctmax__blk1330_dn4, ) = (assign48610_e62308, (0.5 * locals.var_xbct__blk1326_dn4), );
            locals.var_xctmax__blk1330_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
            let assign48620_e62318: f64 = (locals.var_xbct__blk1326 + locals.var_xsbstar__blk1327);
            (locals.var_xnct__blk1331, locals.var_xnct__blk1331_dn4, locals.var_xnct__blk1331_dn6, locals.var_xnct__blk1331_dn7, locals.var_xnct__blk1331_dn8, locals.var_xnct__blk1331_dn9, ) = (assign48620_e62318, (locals.var_xbct__blk1326_dn4 + locals.var_xsbstar__blk1327_dn4), locals.var_xsbstar__blk1327_dn6, locals.var_xsbstar__blk1327_dn7, locals.var_xsbstar__blk1327_dn8, locals.var_xsbstar__blk1327_dn9, );
            locals.var_xnct__blk1331_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
            let assign48630_e62328: f64 = (locals.var_xgct__blk1328 - locals.var_xnct__blk1331);
            let assign48630_e62331: f64 = (locals.var_xnct__blk1331).sqrt();
            let assign48630_e62332: f64 = (locals.var_g_0__blk1316 * assign48630_e62331);
            let assign48630_e62333: f64 = (assign48630_e62328 - assign48630_e62332);
            let assign48630_e62337: f64 = (locals.var_xbct__blk1326 / locals.var_g_0__blk1316);
            let assign48630_e62339: f64 = (locals.var_xbct__blk1326).sqrt();
            let assign48630_e62340: f64 = (assign48630_e62337 + assign48630_e62339);
            let assign48630_e62341: f64 = (assign48630_e62340).ln();
            let assign48630_e62342: f64 = (2.0 * assign48630_e62341);
            let assign48630_e62343: f64 = (assign48630_e62333 - assign48630_e62342);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign48630_e62343, (((locals.var_xgct__blk1328_dn4 - locals.var_xnct__blk1331_dn4) - ((locals.var_g_0__blk1316_dn4 * assign48630_e62331) + (locals.var_g_0__blk1316 * (locals.var_xnct__blk1331_dn4 / (2.0 * assign48630_e62331))))) - (2.0 * (((((locals.var_xbct__blk1326_dn4 * locals.var_g_0__blk1316) - (locals.var_xbct__blk1326 * locals.var_g_0__blk1316_dn4)) / (locals.var_g_0__blk1316 * locals.var_g_0__blk1316)) + (locals.var_xbct__blk1326_dn4 / (2.0 * assign48630_e62339))) / assign48630_e62340))), ((locals.var_xgct__blk1328_dn6 - locals.var_xnct__blk1331_dn6) - (locals.var_g_0__blk1316 * (locals.var_xnct__blk1331_dn6 / (2.0 * assign48630_e62331)))), ((locals.var_xgct__blk1328_dn7 - locals.var_xnct__blk1331_dn7) - (locals.var_g_0__blk1316 * (locals.var_xnct__blk1331_dn7 / (2.0 * assign48630_e62331)))), ((locals.var_xgct__blk1328_dn8 - locals.var_xnct__blk1331_dn8) - (locals.var_g_0__blk1316 * (locals.var_xnct__blk1331_dn8 / (2.0 * assign48630_e62331)))), ((locals.var_xgct__blk1328_dn9 - locals.var_xnct__blk1331_dn9) - (locals.var_g_0__blk1316 * (locals.var_xnct__blk1331_dn9 / (2.0 * assign48630_e62331)))), );
            locals.var_temp1_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
            let assign48640_e62353: f64 = (2.0 * locals.var_temp1);
            let assign48640_e62355: f64 = (assign48640_e62353 + locals.var_xctmax__blk1330);
            (locals.var_xmict__blk1332, locals.var_xmict__blk1332_dn4, locals.var_xmict__blk1332_dn6, locals.var_xmict__blk1332_dn7, locals.var_xmict__blk1332_dn8, locals.var_xmict__blk1332_dn9, ) = (assign48640_e62355, ((2.0 * locals.var_temp1_dn4) + locals.var_xctmax__blk1330_dn4), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), (2.0 * locals.var_temp1_dn9), );
            locals.var_xmict__blk1332_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_22(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
            let assign48650_e62366: f64 = (locals.var_xwict__blk1329 + locals.var_xmict__blk1332);
            let assign48650_e62369: f64 = (locals.var_xwict__blk1329 - locals.var_xmict__blk1332);
            let assign48650_e62372: f64 = (locals.var_xwict__blk1329 - locals.var_xmict__blk1332);
            let assign48650_e62373: f64 = (assign48650_e62369 * assign48650_e62372);
            let assign48650_e62375: f64 = (assign48650_e62373 + 20.0);
            let assign48650_e62376: f64 = (assign48650_e62375).sqrt();
            let assign48650_e62377: f64 = (assign48650_e62366 + assign48650_e62376);
            let assign48650_e62378: f64 = (0.5 * assign48650_e62377);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign48650_e62378, (0.5 * ((locals.var_xwict__blk1329_dn4 + locals.var_xmict__blk1332_dn4) + ((((locals.var_xwict__blk1329_dn4 - locals.var_xmict__blk1332_dn4) * assign48650_e62372) + (assign48650_e62369 * (locals.var_xwict__blk1329_dn4 - locals.var_xmict__blk1332_dn4))) / (2.0 * assign48650_e62376)))), (0.5 * ((locals.var_xwict__blk1329_dn6 + locals.var_xmict__blk1332_dn6) + ((((locals.var_xwict__blk1329_dn6 - locals.var_xmict__blk1332_dn6) * assign48650_e62372) + (assign48650_e62369 * (locals.var_xwict__blk1329_dn6 - locals.var_xmict__blk1332_dn6))) / (2.0 * assign48650_e62376)))), (0.5 * ((locals.var_xwict__blk1329_dn7 + locals.var_xmict__blk1332_dn7) + ((((locals.var_xwict__blk1329_dn7 - locals.var_xmict__blk1332_dn7) * assign48650_e62372) + (assign48650_e62369 * (locals.var_xwict__blk1329_dn7 - locals.var_xmict__blk1332_dn7))) / (2.0 * assign48650_e62376)))), (0.5 * ((locals.var_xwict__blk1329_dn8 + locals.var_xmict__blk1332_dn8) + ((((locals.var_xwict__blk1329_dn8 - locals.var_xmict__blk1332_dn8) * assign48650_e62372) + (assign48650_e62369 * (locals.var_xwict__blk1329_dn8 - locals.var_xmict__blk1332_dn8))) / (2.0 * assign48650_e62376)))), (0.5 * ((locals.var_xwict__blk1329_dn9 + locals.var_xmict__blk1332_dn9) + ((((locals.var_xwict__blk1329_dn9 - locals.var_xmict__blk1332_dn9) * assign48650_e62372) + (assign48650_e62369 * (locals.var_xwict__blk1329_dn9 - locals.var_xmict__blk1332_dn9))) / (2.0 * assign48650_e62376)))), );
            locals.var_temp1_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
            let assign48660_e62389: f64 = (locals.var_xgct__blk1328 - locals.var_xsbstar__blk1327);
            let assign48660_e62390: f64 = (2.0 * assign48660_e62389);
            let assign48660_e62392: f64 = (assign48660_e62390 - locals.var_xctmax__blk1330);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign48660_e62392, ((2.0 * (locals.var_xgct__blk1328_dn4 - locals.var_xsbstar__blk1327_dn4)) - locals.var_xctmax__blk1330_dn4), (2.0 * (locals.var_xgct__blk1328_dn6 - locals.var_xsbstar__blk1327_dn6)), (2.0 * (locals.var_xgct__blk1328_dn7 - locals.var_xsbstar__blk1327_dn7)), (2.0 * (locals.var_xgct__blk1328_dn8 - locals.var_xsbstar__blk1327_dn8)), (2.0 * (locals.var_xgct__blk1328_dn9 - locals.var_xsbstar__blk1327_dn9)), );
            locals.var_temp2_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
            let assign48670_e62403: f64 = (locals.var_temp1 + locals.var_temp2);
            let assign48670_e62406: f64 = (locals.var_temp1 - locals.var_temp2);
            let assign48670_e62409: f64 = (locals.var_temp1 - locals.var_temp2);
            let assign48670_e62410: f64 = (assign48670_e62406 * assign48670_e62409);
            let assign48670_e62412: f64 = (assign48670_e62410 + 20.0);
            let assign48670_e62413: f64 = (assign48670_e62412).sqrt();
            let assign48670_e62414: f64 = (assign48670_e62403 - assign48670_e62413);
            let assign48670_e62415: f64 = (0.5 * assign48670_e62414);
            (locals.var_xsubct__blk1333, locals.var_xsubct__blk1333_dn4, locals.var_xsubct__blk1333_dn6, locals.var_xsubct__blk1333_dn7, locals.var_xsubct__blk1333_dn8, locals.var_xsubct__blk1333_dn9, ) = (assign48670_e62415, (0.5 * ((locals.var_temp1_dn4 + locals.var_temp2_dn4) - ((((locals.var_temp1_dn4 - locals.var_temp2_dn4) * assign48670_e62409) + (assign48670_e62406 * (locals.var_temp1_dn4 - locals.var_temp2_dn4))) / (2.0 * assign48670_e62413)))), (0.5 * ((locals.var_temp1_dn6 + locals.var_temp2_dn6) - ((((locals.var_temp1_dn6 - locals.var_temp2_dn6) * assign48670_e62409) + (assign48670_e62406 * (locals.var_temp1_dn6 - locals.var_temp2_dn6))) / (2.0 * assign48670_e62413)))), (0.5 * ((locals.var_temp1_dn7 + locals.var_temp2_dn7) - ((((locals.var_temp1_dn7 - locals.var_temp2_dn7) * assign48670_e62409) + (assign48670_e62406 * (locals.var_temp1_dn7 - locals.var_temp2_dn7))) / (2.0 * assign48670_e62413)))), (0.5 * ((locals.var_temp1_dn8 + locals.var_temp2_dn8) - ((((locals.var_temp1_dn8 - locals.var_temp2_dn8) * assign48670_e62409) + (assign48670_e62406 * (locals.var_temp1_dn8 - locals.var_temp2_dn8))) / (2.0 * assign48670_e62413)))), (0.5 * ((locals.var_temp1_dn9 + locals.var_temp2_dn9) - ((((locals.var_temp1_dn9 - locals.var_temp2_dn9) * assign48670_e62409) + (assign48670_e62406 * (locals.var_temp1_dn9 - locals.var_temp2_dn9))) / (2.0 * assign48670_e62413)))), );
            locals.var_xsubct__blk1333_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
            let assign48680_e62426: f64 = (locals.var_xsubct__blk1333 + locals.var_xctmax__blk1330);
            let assign48680_e62429: f64 = (locals.var_xsubct__blk1333 - locals.var_xctmax__blk1330);
            let assign48680_e62432: f64 = (locals.var_xsubct__blk1333 - locals.var_xctmax__blk1330);
            let assign48680_e62433: f64 = (assign48680_e62429 * assign48680_e62432);
            let assign48680_e62435: f64 = (assign48680_e62433 + 5.0);
            let assign48680_e62436: f64 = (assign48680_e62435).sqrt();
            let assign48680_e62437: f64 = (assign48680_e62426 - assign48680_e62436);
            let assign48680_e62438: f64 = (0.5 * assign48680_e62437);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign48680_e62438, (0.5 * ((locals.var_xsubct__blk1333_dn4 + locals.var_xctmax__blk1330_dn4) - ((((locals.var_xsubct__blk1333_dn4 - locals.var_xctmax__blk1330_dn4) * assign48680_e62432) + (assign48680_e62429 * (locals.var_xsubct__blk1333_dn4 - locals.var_xctmax__blk1330_dn4))) / (2.0 * assign48680_e62436)))), (0.5 * (locals.var_xsubct__blk1333_dn6 - (((locals.var_xsubct__blk1333_dn6 * assign48680_e62432) + (assign48680_e62429 * locals.var_xsubct__blk1333_dn6)) / (2.0 * assign48680_e62436)))), (0.5 * (locals.var_xsubct__blk1333_dn7 - (((locals.var_xsubct__blk1333_dn7 * assign48680_e62432) + (assign48680_e62429 * locals.var_xsubct__blk1333_dn7)) / (2.0 * assign48680_e62436)))), (0.5 * (locals.var_xsubct__blk1333_dn8 - (((locals.var_xsubct__blk1333_dn8 * assign48680_e62432) + (assign48680_e62429 * locals.var_xsubct__blk1333_dn8)) / (2.0 * assign48680_e62436)))), (0.5 * (locals.var_xsubct__blk1333_dn9 - (((locals.var_xsubct__blk1333_dn9 * assign48680_e62432) + (assign48680_e62429 * locals.var_xsubct__blk1333_dn9)) / (2.0 * assign48680_e62436)))), );
            locals.var_temp1_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
            let assign48690_e62449: f64 = (-locals.var_xctmax__blk1330);
            let assign48690_e62450: f64 = (locals.var_temp1 + assign48690_e62449);
            let assign48690_e62453: f64 = (-locals.var_xctmax__blk1330);
            let assign48690_e62454: f64 = (locals.var_temp1 - assign48690_e62453);
            let assign48690_e62457: f64 = (-locals.var_xctmax__blk1330);
            let assign48690_e62458: f64 = (locals.var_temp1 - assign48690_e62457);
            let assign48690_e62459: f64 = (assign48690_e62454 * assign48690_e62458);
            let assign48690_e62461: f64 = (assign48690_e62459 + 20.0);
            let assign48690_e62462: f64 = (assign48690_e62461).sqrt();
            let assign48690_e62463: f64 = (assign48690_e62450 + assign48690_e62462);
            let assign48690_e62464: f64 = (0.5 * assign48690_e62463);
            (locals.var_xct__blk1334, locals.var_xct__blk1334_dn4, locals.var_xct__blk1334_dn6, locals.var_xct__blk1334_dn7, locals.var_xct__blk1334_dn8, locals.var_xct__blk1334_dn9, ) = (assign48690_e62464, (0.5 * ((locals.var_temp1_dn4 + (-locals.var_xctmax__blk1330_dn4)) + ((((locals.var_temp1_dn4 - (-locals.var_xctmax__blk1330_dn4)) * assign48690_e62458) + (assign48690_e62454 * (locals.var_temp1_dn4 - (-locals.var_xctmax__blk1330_dn4)))) / (2.0 * assign48690_e62462)))), (0.5 * (locals.var_temp1_dn6 + (((locals.var_temp1_dn6 * assign48690_e62458) + (assign48690_e62454 * locals.var_temp1_dn6)) / (2.0 * assign48690_e62462)))), (0.5 * (locals.var_temp1_dn7 + (((locals.var_temp1_dn7 * assign48690_e62458) + (assign48690_e62454 * locals.var_temp1_dn7)) / (2.0 * assign48690_e62462)))), (0.5 * (locals.var_temp1_dn8 + (((locals.var_temp1_dn8 * assign48690_e62458) + (assign48690_e62454 * locals.var_temp1_dn8)) / (2.0 * assign48690_e62462)))), (0.5 * (locals.var_temp1_dn9 + (((locals.var_temp1_dn9 * assign48690_e62458) + (assign48690_e62454 * locals.var_temp1_dn9)) / (2.0 * assign48690_e62462)))), );
            locals.var_xct__blk1334_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) {
            let assign48700_e62475: f64 = (locals.var_xct__blk1334 / locals.var_xctmax__blk1330);
            let assign48700_e62477: f64 = (assign48700_e62475 + 1.0);
            let assign48700_e62478: f64 = (locals.var_ctg_t * assign48700_e62477);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign48700_e62478, ((locals.var_ctg_t_dn4 * assign48700_e62477) + (locals.var_ctg_t * (((locals.var_xct__blk1334_dn4 * locals.var_xctmax__blk1330) - (locals.var_xct__blk1334 * locals.var_xctmax__blk1330_dn4)) / (locals.var_xctmax__blk1330 * locals.var_xctmax__blk1330)))), (locals.var_ctg_t * (locals.var_xct__blk1334_dn6 / locals.var_xctmax__blk1330)), (locals.var_ctg_t * (locals.var_xct__blk1334_dn7 / locals.var_xctmax__blk1330)), (locals.var_ctg_t * (locals.var_xct__blk1334_dn8 / locals.var_xctmax__blk1330)), (locals.var_ctg_t * (locals.var_xct__blk1334_dn9 / locals.var_xctmax__blk1330)), );
            locals.var_temp2_rv = 0.0;
        }

        let assign48710_e62483: f64 = (-230.25850929940458);
        let assign48710_e62484: f64 = if locals.var_temp2 > assign48710_e62483 { 1.0 } else { 0.0 };
        locals.var_guard1477 = assign48710_e62484;
        locals.var_guard1477_rv = 0.0;

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) && (locals.var_guard1477 != 0.0)) {
            let assign48720_e62493: f64 = (locals.var_temp2).exp();
            (locals.var_dctg__blk1335, locals.var_dctg__blk1335_dn4, locals.var_dctg__blk1335_dn6, locals.var_dctg__blk1335_dn7, locals.var_dctg__blk1335_dn8, locals.var_dctg__blk1335_dn9, ) = (assign48720_e62493, (assign48720_e62493 * locals.var_temp2_dn4), (assign48720_e62493 * locals.var_temp2_dn6), (assign48720_e62493 * locals.var_temp2_dn7), (assign48720_e62493 * locals.var_temp2_dn8), (assign48720_e62493 * locals.var_temp2_dn9), );
            locals.var_dctg__blk1335_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1476 != 0.0)) && (locals.var_guard1477 == 0.0)) {
            let assign48730_e62507: f64 = (-230.25850929940458);
            let assign48730_e62509: f64 = (assign48730_e62507 - locals.var_temp2);
            let assign48730_e62513: f64 = (-230.25850929940458);
            let assign48730_e62515: f64 = (assign48730_e62513 - locals.var_temp2);
            let assign48730_e62518: f64 = (-230.25850929940458);
            let assign48730_e62520: f64 = (assign48730_e62518 - locals.var_temp2);
            let assign48730_e62522: f64 = (assign48730_e62520 * 0.3333333333333333);
            let assign48730_e62523: f64 = (1.0 + assign48730_e62522);
            let assign48730_e62524: f64 = (assign48730_e62515 * assign48730_e62523);
            let assign48730_e62525: f64 = (0.5 * assign48730_e62524);
            let assign48730_e62526: f64 = (1.0 + assign48730_e62525);
            let assign48730_e62527: f64 = (assign48730_e62509 * assign48730_e62526);
            let assign48730_e62528: f64 = (1.0 + assign48730_e62527);
            let assign48730_e62529: f64 = (1e-100 / assign48730_e62528);
            (locals.var_dctg__blk1335, locals.var_dctg__blk1335_dn4, locals.var_dctg__blk1335_dn6, locals.var_dctg__blk1335_dn7, locals.var_dctg__blk1335_dn8, locals.var_dctg__blk1335_dn9, ) = (assign48730_e62529, (-((1e-100 * (((-locals.var_temp2_dn4) * assign48730_e62526) + (assign48730_e62509 * (0.5 * (((-locals.var_temp2_dn4) * assign48730_e62523) + (assign48730_e62515 * ((-locals.var_temp2_dn4) * 0.3333333333333333))))))) / (assign48730_e62528 * assign48730_e62528))), (-((1e-100 * (((-locals.var_temp2_dn6) * assign48730_e62526) + (assign48730_e62509 * (0.5 * (((-locals.var_temp2_dn6) * assign48730_e62523) + (assign48730_e62515 * ((-locals.var_temp2_dn6) * 0.3333333333333333))))))) / (assign48730_e62528 * assign48730_e62528))), (-((1e-100 * (((-locals.var_temp2_dn7) * assign48730_e62526) + (assign48730_e62509 * (0.5 * (((-locals.var_temp2_dn7) * assign48730_e62523) + (assign48730_e62515 * ((-locals.var_temp2_dn7) * 0.3333333333333333))))))) / (assign48730_e62528 * assign48730_e62528))), (-((1e-100 * (((-locals.var_temp2_dn8) * assign48730_e62526) + (assign48730_e62509 * (0.5 * (((-locals.var_temp2_dn8) * assign48730_e62523) + (assign48730_e62515 * ((-locals.var_temp2_dn8) * 0.3333333333333333))))))) / (assign48730_e62528 * assign48730_e62528))), (-((1e-100 * (((-locals.var_temp2_dn9) * assign48730_e62526) + (assign48730_e62509 * (0.5 * (((-locals.var_temp2_dn9) * assign48730_e62523) + (assign48730_e62515 * ((-locals.var_temp2_dn9) * 0.3333333333333333))))))) / (assign48730_e62528 * assign48730_e62528))), );
            locals.var_dctg__blk1335_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign48740_e62538: f64 = (locals.var_ct_t * locals.var_dctg__blk1335);
            let assign48740_e62539: f64 = (1.0 + assign48740_e62538);
            (locals.var_ct_fact__blk1336, locals.var_ct_fact__blk1336_dn4, locals.var_ct_fact__blk1336_dn6, locals.var_ct_fact__blk1336_dn7, locals.var_ct_fact__blk1336_dn8, locals.var_ct_fact__blk1336_dn9, ) = (assign48740_e62539, ((locals.var_ct_t_dn4 * locals.var_dctg__blk1335) + (locals.var_ct_t * locals.var_dctg__blk1335_dn4)), (locals.var_ct_t * locals.var_dctg__blk1335_dn6), (locals.var_ct_t * locals.var_dctg__blk1335_dn7), (locals.var_ct_t * locals.var_dctg__blk1335_dn8), (locals.var_ct_t * locals.var_dctg__blk1335_dn9), );
            locals.var_ct_fact__blk1336_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign48750_e62547: f64 = (locals.var_phit * locals.var_ct_fact__blk1336);
            (locals.var_phitct__blk1337, locals.var_phitct__blk1337_dn4, locals.var_phitct__blk1337_dn6, locals.var_phitct__blk1337_dn7, locals.var_phitct__blk1337_dn8, locals.var_phitct__blk1337_dn9, ) = (assign48750_e62547, ((locals.var_phit_dn4 * locals.var_ct_fact__blk1336) + (locals.var_phit * locals.var_ct_fact__blk1336_dn4)), (locals.var_phit * locals.var_ct_fact__blk1336_dn6), (locals.var_phit * locals.var_ct_fact__blk1336_dn7), (locals.var_phit * locals.var_ct_fact__blk1336_dn8), (locals.var_phit * locals.var_ct_fact__blk1336_dn9), );
            locals.var_phitct__blk1337_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign48760_e62557: f64 = (locals.var_psced_i * locals.var_vdsx);
            let assign48760_e62558: f64 = (1.0 + assign48760_e62557);
            let assign48760_e62559: f64 = (locals.var_psce_i * assign48760_e62558);
            let assign48760_e62563: f64 = (locals.var_psceb_i * locals.var_vsbx__blk1323);
            let assign48760_e62564: f64 = (1.0 + assign48760_e62563);
            let assign48760_e62565: f64 = (assign48760_e62559 * assign48760_e62564);
            (locals.var_dphit1__blk1338, locals.var_dphit1__blk1338_dn4, locals.var_dphit1__blk1338_dn6, locals.var_dphit1__blk1338_dn7, locals.var_dphit1__blk1338_dn8, locals.var_dphit1__blk1338_dn9, ) = (assign48760_e62565, (assign48760_e62559 * (locals.var_psceb_i * locals.var_vsbx__blk1323_dn4)), (assign48760_e62559 * (locals.var_psceb_i * locals.var_vsbx__blk1323_dn6)), (((locals.var_psce_i * (locals.var_psced_i * locals.var_vdsx_dn7)) * assign48760_e62564) + (assign48760_e62559 * (locals.var_psceb_i * locals.var_vsbx__blk1323_dn7))), (((locals.var_psce_i * (locals.var_psced_i * locals.var_vdsx_dn8)) * assign48760_e62564) + (assign48760_e62559 * (locals.var_psceb_i * locals.var_vsbx__blk1323_dn8))), (assign48760_e62559 * (locals.var_psceb_i * locals.var_vsbx__blk1323_dn9)), );
            locals.var_dphit1__blk1338_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign48770_e62574: f64 = (1.0 + locals.var_dphit1__blk1338);
            let assign48770_e62575: f64 = (locals.var_phitct__blk1337 * assign48770_e62574);
            (locals.var_phit1__blk1339, locals.var_phit1__blk1339_dn4, locals.var_phit1__blk1339_dn6, locals.var_phit1__blk1339_dn7, locals.var_phit1__blk1339_dn8, locals.var_phit1__blk1339_dn9, ) = (assign48770_e62575, ((locals.var_phitct__blk1337_dn4 * assign48770_e62574) + (locals.var_phitct__blk1337 * locals.var_dphit1__blk1338_dn4)), ((locals.var_phitct__blk1337_dn6 * assign48770_e62574) + (locals.var_phitct__blk1337 * locals.var_dphit1__blk1338_dn6)), ((locals.var_phitct__blk1337_dn7 * assign48770_e62574) + (locals.var_phitct__blk1337 * locals.var_dphit1__blk1338_dn7)), ((locals.var_phitct__blk1337_dn8 * assign48770_e62574) + (locals.var_phitct__blk1337 * locals.var_dphit1__blk1338_dn8)), ((locals.var_phitct__blk1337_dn9 * assign48770_e62574) + (locals.var_phitct__blk1337 * locals.var_dphit1__blk1338_dn9)), );
            locals.var_phit1__blk1339_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign48780_e62583: f64 = (1.0 / locals.var_phit1__blk1339);
            (locals.var_inv_phit1__blk1340, locals.var_inv_phit1__blk1340_dn4, locals.var_inv_phit1__blk1340_dn6, locals.var_inv_phit1__blk1340_dn7, locals.var_inv_phit1__blk1340_dn8, locals.var_inv_phit1__blk1340_dn9, ) = (assign48780_e62583, (-(locals.var_phit1__blk1339_dn4 / (locals.var_phit1__blk1339 * locals.var_phit1__blk1339))), (-(locals.var_phit1__blk1339_dn6 / (locals.var_phit1__blk1339 * locals.var_phit1__blk1339))), (-(locals.var_phit1__blk1339_dn7 / (locals.var_phit1__blk1339 * locals.var_phit1__blk1339))), (-(locals.var_phit1__blk1339_dn8 / (locals.var_phit1__blk1339 * locals.var_phit1__blk1339))), (-(locals.var_phit1__blk1339_dn9 / (locals.var_phit1__blk1339 * locals.var_phit1__blk1339))), );
            locals.var_inv_phit1__blk1340_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign48790_e62592: f64 = (locals.var_phit * locals.var_inv_phit1__blk1340);
            let assign48790_e62593: f64 = (assign48790_e62592).sqrt();
            let assign48790_e62594: f64 = (locals.var_g_0__blk1316 * assign48790_e62593);
            (locals.var_gf__blk1324, locals.var_gf__blk1324_dn4, locals.var_gf__blk1324_dn6, locals.var_gf__blk1324_dn7, locals.var_gf__blk1324_dn8, locals.var_gf__blk1324_dn9, ) = (assign48790_e62594, ((locals.var_g_0__blk1316_dn4 * assign48790_e62593) + (locals.var_g_0__blk1316 * (((locals.var_phit_dn4 * locals.var_inv_phit1__blk1340) + (locals.var_phit * locals.var_inv_phit1__blk1340_dn4)) / (2.0 * assign48790_e62593)))), (locals.var_g_0__blk1316 * ((locals.var_phit * locals.var_inv_phit1__blk1340_dn6) / (2.0 * assign48790_e62593))), (locals.var_g_0__blk1316 * ((locals.var_phit * locals.var_inv_phit1__blk1340_dn7) / (2.0 * assign48790_e62593))), (locals.var_g_0__blk1316 * ((locals.var_phit * locals.var_inv_phit1__blk1340_dn8) / (2.0 * assign48790_e62593))), (locals.var_g_0__blk1316 * ((locals.var_phit * locals.var_inv_phit1__blk1340_dn9) / (2.0 * assign48790_e62593))), );
            locals.var_gf__blk1324_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign48800_e62602: f64 = (locals.var_gf__blk1324 * locals.var_gf__blk1324);
            (locals.var_gf2__blk1325, locals.var_gf2__blk1325_dn4, locals.var_gf2__blk1325_dn6, locals.var_gf2__blk1325_dn7, locals.var_gf2__blk1325_dn8, locals.var_gf2__blk1325_dn9, ) = (assign48800_e62602, ((locals.var_gf__blk1324_dn4 * locals.var_gf__blk1324) + (locals.var_gf__blk1324 * locals.var_gf__blk1324_dn4)), ((locals.var_gf__blk1324_dn6 * locals.var_gf__blk1324) + (locals.var_gf__blk1324 * locals.var_gf__blk1324_dn6)), ((locals.var_gf__blk1324_dn7 * locals.var_gf__blk1324) + (locals.var_gf__blk1324 * locals.var_gf__blk1324_dn7)), ((locals.var_gf__blk1324_dn8 * locals.var_gf__blk1324) + (locals.var_gf__blk1324 * locals.var_gf__blk1324_dn8)), ((locals.var_gf__blk1324_dn9 * locals.var_gf__blk1324) + (locals.var_gf__blk1324 * locals.var_gf__blk1324_dn9)), );
            locals.var_gf2__blk1325_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign48810_e62610: f64 = (1.0 / locals.var_gf2__blk1325);
            (locals.var_inv_gf2__blk1341, locals.var_inv_gf2__blk1341_dn4, locals.var_inv_gf2__blk1341_dn6, locals.var_inv_gf2__blk1341_dn7, locals.var_inv_gf2__blk1341_dn8, locals.var_inv_gf2__blk1341_dn9, ) = (assign48810_e62610, (-(locals.var_gf2__blk1325_dn4 / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))), (-(locals.var_gf2__blk1325_dn6 / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))), (-(locals.var_gf2__blk1325_dn7 / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))), (-(locals.var_gf2__blk1325_dn8 / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))), (-(locals.var_gf2__blk1325_dn9 / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))), );
            locals.var_inv_gf2__blk1341_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign48820_e62618: f64 = (locals.var_vsbstar__blk1318 * locals.var_inv_phit1__blk1340);
            (locals.var_ux__blk1342, locals.var_ux__blk1342_dn4, locals.var_ux__blk1342_dn6, locals.var_ux__blk1342_dn7, locals.var_ux__blk1342_dn8, locals.var_ux__blk1342_dn9, ) = (assign48820_e62618, ((locals.var_vsbstar__blk1318_dn4 * locals.var_inv_phit1__blk1340) + (locals.var_vsbstar__blk1318 * locals.var_inv_phit1__blk1340_dn4)), ((locals.var_vsbstar__blk1318_dn6 * locals.var_inv_phit1__blk1340) + (locals.var_vsbstar__blk1318 * locals.var_inv_phit1__blk1340_dn6)), ((locals.var_vsbstar__blk1318_dn7 * locals.var_inv_phit1__blk1340) + (locals.var_vsbstar__blk1318 * locals.var_inv_phit1__blk1340_dn7)), ((locals.var_vsbstar__blk1318_dn8 * locals.var_inv_phit1__blk1340) + (locals.var_vsbstar__blk1318 * locals.var_inv_phit1__blk1340_dn8)), ((locals.var_vsbstar__blk1318_dn9 * locals.var_inv_phit1__blk1340) + (locals.var_vsbstar__blk1318 * locals.var_inv_phit1__blk1340_dn9)), );
            locals.var_ux__blk1342_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign48830_e62626: f64 = (locals.var_vgb1__blk1321 * locals.var_inv_phit1__blk1340);
            (locals.var_xg__blk1343, locals.var_xg__blk1343_dn4, locals.var_xg__blk1343_dn6, locals.var_xg__blk1343_dn7, locals.var_xg__blk1343_dn8, locals.var_xg__blk1343_dn9, ) = (assign48830_e62626, ((locals.var_vgb1__blk1321_dn4 * locals.var_inv_phit1__blk1340) + (locals.var_vgb1__blk1321 * locals.var_inv_phit1__blk1340_dn4)), ((locals.var_vgb1__blk1321_dn6 * locals.var_inv_phit1__blk1340) + (locals.var_vgb1__blk1321 * locals.var_inv_phit1__blk1340_dn6)), ((locals.var_vgb1__blk1321_dn7 * locals.var_inv_phit1__blk1340) + (locals.var_vgb1__blk1321 * locals.var_inv_phit1__blk1340_dn7)), ((locals.var_vgb1__blk1321_dn8 * locals.var_inv_phit1__blk1340) + (locals.var_vgb1__blk1321 * locals.var_inv_phit1__blk1340_dn8)), ((locals.var_vgb1__blk1321_dn9 * locals.var_inv_phit1__blk1340) + (locals.var_vgb1__blk1321 * locals.var_inv_phit1__blk1340_dn9)), );
            locals.var_xg__blk1343_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign48840_e62634: f64 = (2.0 * locals.var_vdsx);
            let assign48840_e62639: f64 = (locals.var_cfd_i * locals.var_vdsx);
            let assign48840_e62640: f64 = (1.0 + assign48840_e62639);
            let assign48840_e62641: f64 = (assign48840_e62640).sqrt();
            let assign48840_e62642: f64 = (1.0 + assign48840_e62641);
            let assign48840_e62643: f64 = (assign48840_e62634 / assign48840_e62642);
            (locals.var_vdsp__blk1344, locals.var_vdsp__blk1344_dn7, locals.var_vdsp__blk1344_dn8, ) = (assign48840_e62643, ((((2.0 * locals.var_vdsx_dn7) * assign48840_e62642) - (assign48840_e62634 * ((locals.var_cfd_i * locals.var_vdsx_dn7) / (2.0 * assign48840_e62641)))) / (assign48840_e62642 * assign48840_e62642)), ((((2.0 * locals.var_vdsx_dn8) * assign48840_e62642) - (assign48840_e62634 * ((locals.var_cfd_i * locals.var_vdsx_dn8) / (2.0 * assign48840_e62641)))) / (assign48840_e62642 * assign48840_e62642)), );
            locals.var_vdsp__blk1344_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign48850_e62651: f64 = (locals.var_cf_i * locals.var_vdsp__blk1344);
            let assign48850_e62655: f64 = (locals.var_cfb_i * locals.var_vsbx__blk1323);
            let assign48850_e62656: f64 = (1.0 + assign48850_e62655);
            let assign48850_e62657: f64 = (assign48850_e62651 * assign48850_e62656);
            (locals.var_delphib__blk1345, locals.var_delphib__blk1345_dn4, locals.var_delphib__blk1345_dn6, locals.var_delphib__blk1345_dn7, locals.var_delphib__blk1345_dn8, locals.var_delphib__blk1345_dn9, ) = (assign48850_e62657, (assign48850_e62651 * (locals.var_cfb_i * locals.var_vsbx__blk1323_dn4)), (assign48850_e62651 * (locals.var_cfb_i * locals.var_vsbx__blk1323_dn6)), (((locals.var_cf_i * locals.var_vdsp__blk1344_dn7) * assign48850_e62656) + (assign48850_e62651 * (locals.var_cfb_i * locals.var_vsbx__blk1323_dn7))), (((locals.var_cf_i * locals.var_vdsp__blk1344_dn8) * assign48850_e62656) + (assign48850_e62651 * (locals.var_cfb_i * locals.var_vsbx__blk1323_dn8))), (assign48850_e62651 * (locals.var_cfb_i * locals.var_vsbx__blk1323_dn9)), );
            locals.var_delphib__blk1345_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign48860_e62665: f64 = (locals.var_phib__blk1314 * locals.var_inv_phit1__blk1340);
            (locals.var_xb__blk1346, locals.var_xb__blk1346_dn4, locals.var_xb__blk1346_dn6, locals.var_xb__blk1346_dn7, locals.var_xb__blk1346_dn8, locals.var_xb__blk1346_dn9, ) = (assign48860_e62665, ((locals.var_phib__blk1314_dn4 * locals.var_inv_phit1__blk1340) + (locals.var_phib__blk1314 * locals.var_inv_phit1__blk1340_dn4)), (locals.var_phib__blk1314 * locals.var_inv_phit1__blk1340_dn6), (locals.var_phib__blk1314 * locals.var_inv_phit1__blk1340_dn7), (locals.var_phib__blk1314 * locals.var_inv_phit1__blk1340_dn8), (locals.var_phib__blk1314 * locals.var_inv_phit1__blk1340_dn9), );
            locals.var_xb__blk1346_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign48870_e62673: f64 = (locals.var_v_xb__blk1317 * locals.var_v_xb__blk1317);
            let assign48870_e62675: f64 = (assign48870_e62673 + locals.var_aphi__blk1315);
            let assign48870_e62676: f64 = (assign48870_e62675).sqrt();
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign48870_e62676, ((((locals.var_v_xb__blk1317_dn4 * locals.var_v_xb__blk1317) + (locals.var_v_xb__blk1317 * locals.var_v_xb__blk1317_dn4)) + locals.var_aphi__blk1315_dn4) / (2.0 * assign48870_e62676)), 0.0, (((locals.var_v_xb__blk1317_dn7 * locals.var_v_xb__blk1317) + (locals.var_v_xb__blk1317 * locals.var_v_xb__blk1317_dn7)) / (2.0 * assign48870_e62676)), (((locals.var_v_xb__blk1317_dn8 * locals.var_v_xb__blk1317) + (locals.var_v_xb__blk1317 * locals.var_v_xb__blk1317_dn8)) / (2.0 * assign48870_e62676)), (((locals.var_v_xb__blk1317_dn9 * locals.var_v_xb__blk1317) + (locals.var_v_xb__blk1317 * locals.var_v_xb__blk1317_dn9)) / (2.0 * assign48870_e62676)), );
            locals.var_temp1_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign48880_e62684: f64 = (locals.var_v_xb__blk1317 - locals.var_delphib__blk1345);
            let assign48880_e62687: f64 = (locals.var_v_xb__blk1317 - locals.var_delphib__blk1345);
            let assign48880_e62688: f64 = (assign48880_e62684 * assign48880_e62687);
            let assign48880_e62690: f64 = (assign48880_e62688 + locals.var_aphi__blk1315);
            let assign48880_e62691: f64 = (assign48880_e62690).sqrt();
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign48880_e62691, (((((locals.var_v_xb__blk1317_dn4 - locals.var_delphib__blk1345_dn4) * assign48880_e62687) + (assign48880_e62684 * (locals.var_v_xb__blk1317_dn4 - locals.var_delphib__blk1345_dn4))) + locals.var_aphi__blk1315_dn4) / (2.0 * assign48880_e62691)), ((((-locals.var_delphib__blk1345_dn6) * assign48880_e62687) + (assign48880_e62684 * (-locals.var_delphib__blk1345_dn6))) / (2.0 * assign48880_e62691)), ((((locals.var_v_xb__blk1317_dn7 - locals.var_delphib__blk1345_dn7) * assign48880_e62687) + (assign48880_e62684 * (locals.var_v_xb__blk1317_dn7 - locals.var_delphib__blk1345_dn7))) / (2.0 * assign48880_e62691)), ((((locals.var_v_xb__blk1317_dn8 - locals.var_delphib__blk1345_dn8) * assign48880_e62687) + (assign48880_e62684 * (locals.var_v_xb__blk1317_dn8 - locals.var_delphib__blk1345_dn8))) / (2.0 * assign48880_e62691)), ((((locals.var_v_xb__blk1317_dn9 - locals.var_delphib__blk1345_dn9) * assign48880_e62687) + (assign48880_e62684 * (locals.var_v_xb__blk1317_dn9 - locals.var_delphib__blk1345_dn9))) / (2.0 * assign48880_e62691)), );
            locals.var_temp2_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign48890_e62699: f64 = (0.5 * locals.var_inv_phit1__blk1340);
            let assign48890_e62702: f64 = (locals.var_delphib__blk1345 + locals.var_temp1);
            let assign48890_e62704: f64 = (assign48890_e62702 - locals.var_temp2);
            let assign48890_e62705: f64 = (assign48890_e62699 * assign48890_e62704);
            (locals.var_delxb__blk1347, locals.var_delxb__blk1347_dn4, locals.var_delxb__blk1347_dn6, locals.var_delxb__blk1347_dn7, locals.var_delxb__blk1347_dn8, locals.var_delxb__blk1347_dn9, ) = (assign48890_e62705, (((0.5 * locals.var_inv_phit1__blk1340_dn4) * assign48890_e62704) + (assign48890_e62699 * ((locals.var_delphib__blk1345_dn4 + locals.var_temp1_dn4) - locals.var_temp2_dn4))), (((0.5 * locals.var_inv_phit1__blk1340_dn6) * assign48890_e62704) + (assign48890_e62699 * ((locals.var_delphib__blk1345_dn6 + locals.var_temp1_dn6) - locals.var_temp2_dn6))), (((0.5 * locals.var_inv_phit1__blk1340_dn7) * assign48890_e62704) + (assign48890_e62699 * ((locals.var_delphib__blk1345_dn7 + locals.var_temp1_dn7) - locals.var_temp2_dn7))), (((0.5 * locals.var_inv_phit1__blk1340_dn8) * assign48890_e62704) + (assign48890_e62699 * ((locals.var_delphib__blk1345_dn8 + locals.var_temp1_dn8) - locals.var_temp2_dn8))), (((0.5 * locals.var_inv_phit1__blk1340_dn9) * assign48890_e62704) + (assign48890_e62699 * ((locals.var_delphib__blk1345_dn9 + locals.var_temp1_dn9) - locals.var_temp2_dn9))), );
            locals.var_delxb__blk1347_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign48900_e62713: f64 = (locals.var_xb__blk1346 + locals.var_ux__blk1342);
            (locals.var_xno_s__blk1348, locals.var_xno_s__blk1348_dn4, locals.var_xno_s__blk1348_dn6, locals.var_xno_s__blk1348_dn7, locals.var_xno_s__blk1348_dn8, locals.var_xno_s__blk1348_dn9, ) = (assign48900_e62713, (locals.var_xb__blk1346_dn4 + locals.var_ux__blk1342_dn4), (locals.var_xb__blk1346_dn6 + locals.var_ux__blk1342_dn6), (locals.var_xb__blk1346_dn7 + locals.var_ux__blk1342_dn7), (locals.var_xb__blk1346_dn8 + locals.var_ux__blk1342_dn8), (locals.var_xb__blk1346_dn9 + locals.var_ux__blk1342_dn9), );
            locals.var_xno_s__blk1348_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign48910_e62721: f64 = (locals.var_xno_s__blk1348 - locals.var_delxb__blk1347);
            (locals.var_xn_s__blk1349, locals.var_xn_s__blk1349_dn4, locals.var_xn_s__blk1349_dn6, locals.var_xn_s__blk1349_dn7, locals.var_xn_s__blk1349_dn8, locals.var_xn_s__blk1349_dn9, ) = (assign48910_e62721, (locals.var_xno_s__blk1348_dn4 - locals.var_delxb__blk1347_dn4), (locals.var_xno_s__blk1348_dn6 - locals.var_delxb__blk1347_dn6), (locals.var_xno_s__blk1348_dn7 - locals.var_delxb__blk1347_dn7), (locals.var_xno_s__blk1348_dn8 - locals.var_delxb__blk1347_dn8), (locals.var_xno_s__blk1348_dn9 - locals.var_delxb__blk1347_dn9), );
            locals.var_xn_s__blk1349_rv = 0.0;
        }

        let assign48920_e62726: f64 = if p.p45 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1478 = assign48920_e62726;
        locals.var_guard1478_rv = 0.0;

        let assign48930_e62728: f64 = (locals.var_xn_s__blk1349).abs();
        let assign48930_e62730: f64 = if assign48930_e62728 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1479 = assign48930_e62730;
        locals.var_guard1479_rv = 0.0;

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1479 != 0.0)) {
            let assign48940_e62743: f64 = (0.5 * locals.var_xn_s__blk1349);
            let assign48940_e62747: f64 = (0.3125 * locals.var_xn_s__blk1349);
            let assign48940_e62748: f64 = (1.0 - assign48940_e62747);
            let assign48940_e62749: f64 = (assign48940_e62743 * assign48940_e62748);
            let assign48940_e62750: f64 = (1.0 - assign48940_e62749);
            let assign48940_e62751: f64 = (locals.var_gf__blk1324 * assign48940_e62750);
            let assign48940_e62752: f64 = (1.0 + assign48940_e62751);
            (locals.var_nscr__blk1350, locals.var_nscr__blk1350_dn4, locals.var_nscr__blk1350_dn6, locals.var_nscr__blk1350_dn7, locals.var_nscr__blk1350_dn8, locals.var_nscr__blk1350_dn9, ) = (assign48940_e62752, ((locals.var_gf__blk1324_dn4 * assign48940_e62750) + (locals.var_gf__blk1324 * (-(((0.5 * locals.var_xn_s__blk1349_dn4) * assign48940_e62748) + (assign48940_e62743 * (-(0.3125 * locals.var_xn_s__blk1349_dn4))))))), ((locals.var_gf__blk1324_dn6 * assign48940_e62750) + (locals.var_gf__blk1324 * (-(((0.5 * locals.var_xn_s__blk1349_dn6) * assign48940_e62748) + (assign48940_e62743 * (-(0.3125 * locals.var_xn_s__blk1349_dn6))))))), ((locals.var_gf__blk1324_dn7 * assign48940_e62750) + (locals.var_gf__blk1324 * (-(((0.5 * locals.var_xn_s__blk1349_dn7) * assign48940_e62748) + (assign48940_e62743 * (-(0.3125 * locals.var_xn_s__blk1349_dn7))))))), ((locals.var_gf__blk1324_dn8 * assign48940_e62750) + (locals.var_gf__blk1324 * (-(((0.5 * locals.var_xn_s__blk1349_dn8) * assign48940_e62748) + (assign48940_e62743 * (-(0.3125 * locals.var_xn_s__blk1349_dn8))))))), ((locals.var_gf__blk1324_dn9 * assign48940_e62750) + (locals.var_gf__blk1324 * (-(((0.5 * locals.var_xn_s__blk1349_dn9) * assign48940_e62748) + (assign48940_e62743 * (-(0.3125 * locals.var_xn_s__blk1349_dn9))))))), );
            locals.var_nscr__blk1350_rv = 0.0;
        }

        let assign48950_e62757: f64 = if locals.var_xn_s__blk1349 < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1480 = assign48950_e62757;
        locals.var_guard1480_rv = 0.0;

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1479 == 0.0)) && (locals.var_guard1480 != 0.0)) {
            let assign48960_e62769: f64 = (-locals.var_xn_s__blk1349);
            let assign48960_e62770: f64 = (assign48960_e62769).exp();
            (locals.var_delta_ns__blk1364, locals.var_delta_ns__blk1364_dn4, locals.var_delta_ns__blk1364_dn6, locals.var_delta_ns__blk1364_dn7, locals.var_delta_ns__blk1364_dn8, locals.var_delta_ns__blk1364_dn9, ) = (assign48960_e62770, (assign48960_e62770 * (-locals.var_xn_s__blk1349_dn4)), (assign48960_e62770 * (-locals.var_xn_s__blk1349_dn6)), (assign48960_e62770 * (-locals.var_xn_s__blk1349_dn7)), (assign48960_e62770 * (-locals.var_xn_s__blk1349_dn8)), (assign48960_e62770 * (-locals.var_xn_s__blk1349_dn9)), );
            locals.var_delta_ns__blk1364_rv = 0.0;
        }

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1479 == 0.0)) && (locals.var_guard1480 == 0.0)) {
            let assign48970_e62788: f64 = (locals.var_xn_s__blk1349 - 460.51701859880916);
            let assign48970_e62793: f64 = (locals.var_xn_s__blk1349 - 460.51701859880916);
            let assign48970_e62797: f64 = (locals.var_xn_s__blk1349 - 460.51701859880916);
            let assign48970_e62799: f64 = (assign48970_e62797 * 0.3333333333333333);
            let assign48970_e62800: f64 = (1.0 + assign48970_e62799);
            let assign48970_e62801: f64 = (assign48970_e62793 * assign48970_e62800);
            let assign48970_e62802: f64 = (0.5 * assign48970_e62801);
            let assign48970_e62803: f64 = (1.0 + assign48970_e62802);
            let assign48970_e62804: f64 = (assign48970_e62788 * assign48970_e62803);
            let assign48970_e62805: f64 = (1.0 + assign48970_e62804);
            let assign48970_e62806: f64 = (1e-200 / assign48970_e62805);
            (locals.var_delta_ns__blk1364, locals.var_delta_ns__blk1364_dn4, locals.var_delta_ns__blk1364_dn6, locals.var_delta_ns__blk1364_dn7, locals.var_delta_ns__blk1364_dn8, locals.var_delta_ns__blk1364_dn9, ) = (assign48970_e62806, (-((1e-200 * ((locals.var_xn_s__blk1349_dn4 * assign48970_e62803) + (assign48970_e62788 * (0.5 * ((locals.var_xn_s__blk1349_dn4 * assign48970_e62800) + (assign48970_e62793 * (locals.var_xn_s__blk1349_dn4 * 0.3333333333333333))))))) / (assign48970_e62805 * assign48970_e62805))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn6 * assign48970_e62803) + (assign48970_e62788 * (0.5 * ((locals.var_xn_s__blk1349_dn6 * assign48970_e62800) + (assign48970_e62793 * (locals.var_xn_s__blk1349_dn6 * 0.3333333333333333))))))) / (assign48970_e62805 * assign48970_e62805))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn7 * assign48970_e62803) + (assign48970_e62788 * (0.5 * ((locals.var_xn_s__blk1349_dn7 * assign48970_e62800) + (assign48970_e62793 * (locals.var_xn_s__blk1349_dn7 * 0.3333333333333333))))))) / (assign48970_e62805 * assign48970_e62805))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn8 * assign48970_e62803) + (assign48970_e62788 * (0.5 * ((locals.var_xn_s__blk1349_dn8 * assign48970_e62800) + (assign48970_e62793 * (locals.var_xn_s__blk1349_dn8 * 0.3333333333333333))))))) / (assign48970_e62805 * assign48970_e62805))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn9 * assign48970_e62803) + (assign48970_e62788 * (0.5 * ((locals.var_xn_s__blk1349_dn9 * assign48970_e62800) + (assign48970_e62793 * (locals.var_xn_s__blk1349_dn9 * 0.3333333333333333))))))) / (assign48970_e62805 * assign48970_e62805))), );
            locals.var_delta_ns__blk1364_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1479 == 0.0)) {
            let (assign48980_e62823,) = {
    if (locals.var_xn_s__blk1349 > 0.0) {
        (1.0,)
    } else {
        let assign48980_e62822: f64 = (-1.0);
        (assign48980_e62822,)
    }
};
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign48980_e62823, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1479 == 0.0)) {
            let assign48990_e62837: f64 = (locals.var_temp__blk949 * locals.var_gf__blk1324);
            let assign48990_e62842: f64 = (1.0 - locals.var_xn_s__blk1349);
            let assign48990_e62843: f64 = (locals.var_delta_ns__blk1364 * assign48990_e62842);
            let assign48990_e62844: f64 = (1.0 - assign48990_e62843);
            let assign48990_e62845: f64 = (assign48990_e62837 * assign48990_e62844);
            let assign48990_e62850: f64 = (1.0 - locals.var_delta_ns__blk1364);
            let assign48990_e62851: f64 = (locals.var_xn_s__blk1349 * assign48990_e62850);
            let assign48990_e62852: f64 = (assign48990_e62851).sqrt();
            let assign48990_e62853: f64 = (2.0 * assign48990_e62852);
            let assign48990_e62854: f64 = (assign48990_e62845 / assign48990_e62853);
            let assign48990_e62855: f64 = (1.0 + assign48990_e62854);
            (locals.var_nscr__blk1350, locals.var_nscr__blk1350_dn4, locals.var_nscr__blk1350_dn6, locals.var_nscr__blk1350_dn7, locals.var_nscr__blk1350_dn8, locals.var_nscr__blk1350_dn9, ) = (assign48990_e62855, (((((((locals.var_temp__blk949_dn4 * locals.var_gf__blk1324) + (locals.var_temp__blk949 * locals.var_gf__blk1324_dn4)) * assign48990_e62844) + (assign48990_e62837 * (-((locals.var_delta_ns__blk1364_dn4 * assign48990_e62842) + (locals.var_delta_ns__blk1364 * (-locals.var_xn_s__blk1349_dn4)))))) * assign48990_e62853) - (assign48990_e62845 * (2.0 * (((locals.var_xn_s__blk1349_dn4 * assign48990_e62850) + (locals.var_xn_s__blk1349 * (-locals.var_delta_ns__blk1364_dn4))) / (2.0 * assign48990_e62852))))) / (assign48990_e62853 * assign48990_e62853)), (((((((locals.var_temp__blk949_dn6 * locals.var_gf__blk1324) + (locals.var_temp__blk949 * locals.var_gf__blk1324_dn6)) * assign48990_e62844) + (assign48990_e62837 * (-((locals.var_delta_ns__blk1364_dn6 * assign48990_e62842) + (locals.var_delta_ns__blk1364 * (-locals.var_xn_s__blk1349_dn6)))))) * assign48990_e62853) - (assign48990_e62845 * (2.0 * (((locals.var_xn_s__blk1349_dn6 * assign48990_e62850) + (locals.var_xn_s__blk1349 * (-locals.var_delta_ns__blk1364_dn6))) / (2.0 * assign48990_e62852))))) / (assign48990_e62853 * assign48990_e62853)), (((((((locals.var_temp__blk949_dn7 * locals.var_gf__blk1324) + (locals.var_temp__blk949 * locals.var_gf__blk1324_dn7)) * assign48990_e62844) + (assign48990_e62837 * (-((locals.var_delta_ns__blk1364_dn7 * assign48990_e62842) + (locals.var_delta_ns__blk1364 * (-locals.var_xn_s__blk1349_dn7)))))) * assign48990_e62853) - (assign48990_e62845 * (2.0 * (((locals.var_xn_s__blk1349_dn7 * assign48990_e62850) + (locals.var_xn_s__blk1349 * (-locals.var_delta_ns__blk1364_dn7))) / (2.0 * assign48990_e62852))))) / (assign48990_e62853 * assign48990_e62853)), (((((((locals.var_temp__blk949_dn8 * locals.var_gf__blk1324) + (locals.var_temp__blk949 * locals.var_gf__blk1324_dn8)) * assign48990_e62844) + (assign48990_e62837 * (-((locals.var_delta_ns__blk1364_dn8 * assign48990_e62842) + (locals.var_delta_ns__blk1364 * (-locals.var_xn_s__blk1349_dn8)))))) * assign48990_e62853) - (assign48990_e62845 * (2.0 * (((locals.var_xn_s__blk1349_dn8 * assign48990_e62850) + (locals.var_xn_s__blk1349 * (-locals.var_delta_ns__blk1364_dn8))) / (2.0 * assign48990_e62852))))) / (assign48990_e62853 * assign48990_e62853)), (((((((locals.var_temp__blk949_dn9 * locals.var_gf__blk1324) + (locals.var_temp__blk949 * locals.var_gf__blk1324_dn9)) * assign48990_e62844) + (assign48990_e62837 * (-((locals.var_delta_ns__blk1364_dn9 * assign48990_e62842) + (locals.var_delta_ns__blk1364 * (-locals.var_xn_s__blk1349_dn9)))))) * assign48990_e62853) - (assign48990_e62845 * (2.0 * (((locals.var_xn_s__blk1349_dn9 * assign48990_e62850) + (locals.var_xn_s__blk1349 * (-locals.var_delta_ns__blk1364_dn9))) / (2.0 * assign48990_e62852))))) / (assign48990_e62853 * assign48990_e62853)), );
            locals.var_nscr__blk1350_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 == 0.0)) {
            let assign49000_e62867: f64 = (0.5 * locals.var_gf__blk1324);
            let assign49000_e62869: f64 = (locals.var_xn_s__blk1349).sqrt();
            let assign49000_e62870: f64 = (assign49000_e62867 / assign49000_e62869);
            let assign49000_e62871: f64 = (1.0 + assign49000_e62870);
            (locals.var_nscr__blk1350, locals.var_nscr__blk1350_dn4, locals.var_nscr__blk1350_dn6, locals.var_nscr__blk1350_dn7, locals.var_nscr__blk1350_dn8, locals.var_nscr__blk1350_dn9, ) = (assign49000_e62871, ((((0.5 * locals.var_gf__blk1324_dn4) * assign49000_e62869) - (assign49000_e62867 * (locals.var_xn_s__blk1349_dn4 / (2.0 * assign49000_e62869)))) / (assign49000_e62869 * assign49000_e62869)), ((((0.5 * locals.var_gf__blk1324_dn6) * assign49000_e62869) - (assign49000_e62867 * (locals.var_xn_s__blk1349_dn6 / (2.0 * assign49000_e62869)))) / (assign49000_e62869 * assign49000_e62869)), ((((0.5 * locals.var_gf__blk1324_dn7) * assign49000_e62869) - (assign49000_e62867 * (locals.var_xn_s__blk1349_dn7 / (2.0 * assign49000_e62869)))) / (assign49000_e62869 * assign49000_e62869)), ((((0.5 * locals.var_gf__blk1324_dn8) * assign49000_e62869) - (assign49000_e62867 * (locals.var_xn_s__blk1349_dn8 / (2.0 * assign49000_e62869)))) / (assign49000_e62869 * assign49000_e62869)), ((((0.5 * locals.var_gf__blk1324_dn9) * assign49000_e62869) - (assign49000_e62867 * (locals.var_xn_s__blk1349_dn9 / (2.0 * assign49000_e62869)))) / (assign49000_e62869 * assign49000_e62869)), );
            locals.var_nscr__blk1350_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign49010_e62880: f64 = (locals.var_xn_s__blk1349).sqrt();
            let assign49010_e62881: f64 = (locals.var_gf__blk1324 * assign49010_e62880);
            let assign49010_e62882: f64 = (locals.var_xn_s__blk1349 + assign49010_e62881);
            let assign49010_e62886: f64 = (locals.var_nscr__blk1350 - 1.0);
            let assign49010_e62887: f64 = (assign49010_e62886).ln();
            let assign49010_e62888: f64 = (locals.var_nscr__blk1350 * assign49010_e62887);
            let assign49010_e62889: f64 = (assign49010_e62882 - assign49010_e62888);
            (locals.var_xthscr__blk1351, locals.var_xthscr__blk1351_dn4, locals.var_xthscr__blk1351_dn6, locals.var_xthscr__blk1351_dn7, locals.var_xthscr__blk1351_dn8, locals.var_xthscr__blk1351_dn9, ) = (assign49010_e62889, ((locals.var_xn_s__blk1349_dn4 + ((locals.var_gf__blk1324_dn4 * assign49010_e62880) + (locals.var_gf__blk1324 * (locals.var_xn_s__blk1349_dn4 / (2.0 * assign49010_e62880))))) - ((locals.var_nscr__blk1350_dn4 * assign49010_e62887) + (locals.var_nscr__blk1350 * (locals.var_nscr__blk1350_dn4 / assign49010_e62886)))), ((locals.var_xn_s__blk1349_dn6 + ((locals.var_gf__blk1324_dn6 * assign49010_e62880) + (locals.var_gf__blk1324 * (locals.var_xn_s__blk1349_dn6 / (2.0 * assign49010_e62880))))) - ((locals.var_nscr__blk1350_dn6 * assign49010_e62887) + (locals.var_nscr__blk1350 * (locals.var_nscr__blk1350_dn6 / assign49010_e62886)))), ((locals.var_xn_s__blk1349_dn7 + ((locals.var_gf__blk1324_dn7 * assign49010_e62880) + (locals.var_gf__blk1324 * (locals.var_xn_s__blk1349_dn7 / (2.0 * assign49010_e62880))))) - ((locals.var_nscr__blk1350_dn7 * assign49010_e62887) + (locals.var_nscr__blk1350 * (locals.var_nscr__blk1350_dn7 / assign49010_e62886)))), ((locals.var_xn_s__blk1349_dn8 + ((locals.var_gf__blk1324_dn8 * assign49010_e62880) + (locals.var_gf__blk1324 * (locals.var_xn_s__blk1349_dn8 / (2.0 * assign49010_e62880))))) - ((locals.var_nscr__blk1350_dn8 * assign49010_e62887) + (locals.var_nscr__blk1350 * (locals.var_nscr__blk1350_dn8 / assign49010_e62886)))), ((locals.var_xn_s__blk1349_dn9 + ((locals.var_gf__blk1324_dn9 * assign49010_e62880) + (locals.var_gf__blk1324 * (locals.var_xn_s__blk1349_dn9 / (2.0 * assign49010_e62880))))) - ((locals.var_nscr__blk1350_dn9 * assign49010_e62887) + (locals.var_nscr__blk1350 * (locals.var_nscr__blk1350_dn9 / assign49010_e62886)))), );
            locals.var_xthscr__blk1351_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign49020_e62897: f64 = (locals.var_xg__blk1343 - locals.var_xthscr__blk1351);
            let assign49020_e62899: f64 = (assign49020_e62897 / locals.var_nscr__blk1350);
            (locals.var_xgtscr__blk1352, locals.var_xgtscr__blk1352_dn4, locals.var_xgtscr__blk1352_dn6, locals.var_xgtscr__blk1352_dn7, locals.var_xgtscr__blk1352_dn8, locals.var_xgtscr__blk1352_dn9, ) = (assign49020_e62899, ((((locals.var_xg__blk1343_dn4 - locals.var_xthscr__blk1351_dn4) * locals.var_nscr__blk1350) - (assign49020_e62897 * locals.var_nscr__blk1350_dn4)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), ((((locals.var_xg__blk1343_dn6 - locals.var_xthscr__blk1351_dn6) * locals.var_nscr__blk1350) - (assign49020_e62897 * locals.var_nscr__blk1350_dn6)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), ((((locals.var_xg__blk1343_dn7 - locals.var_xthscr__blk1351_dn7) * locals.var_nscr__blk1350) - (assign49020_e62897 * locals.var_nscr__blk1350_dn7)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), ((((locals.var_xg__blk1343_dn8 - locals.var_xthscr__blk1351_dn8) * locals.var_nscr__blk1350) - (assign49020_e62897 * locals.var_nscr__blk1350_dn8)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), ((((locals.var_xg__blk1343_dn9 - locals.var_xthscr__blk1351_dn9) * locals.var_nscr__blk1350) - (assign49020_e62897 * locals.var_nscr__blk1350_dn9)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), );
            locals.var_xgtscr__blk1352_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign49030_e62907: f64 = (0.5 * locals.var_gf2__blk1325);
            let assign49030_e62911: f64 = (8.0 / locals.var_gf2__blk1325);
            let assign49030_e62912: f64 = (1.0 + assign49030_e62911);
            let assign49030_e62913: f64 = (assign49030_e62912).sqrt();
            let assign49030_e62915: f64 = (assign49030_e62913 - 1.0);
            let assign49030_e62916: f64 = (assign49030_e62907 * assign49030_e62915);
            (locals.var_qbscr__blk1358, locals.var_qbscr__blk1358_dn4, locals.var_qbscr__blk1358_dn6, locals.var_qbscr__blk1358_dn7, locals.var_qbscr__blk1358_dn8, locals.var_qbscr__blk1358_dn9, ) = (assign49030_e62916, (((0.5 * locals.var_gf2__blk1325_dn4) * assign49030_e62915) + (assign49030_e62907 * ((-((8.0 * locals.var_gf2__blk1325_dn4) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) / (2.0 * assign49030_e62913)))), (((0.5 * locals.var_gf2__blk1325_dn6) * assign49030_e62915) + (assign49030_e62907 * ((-((8.0 * locals.var_gf2__blk1325_dn6) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) / (2.0 * assign49030_e62913)))), (((0.5 * locals.var_gf2__blk1325_dn7) * assign49030_e62915) + (assign49030_e62907 * ((-((8.0 * locals.var_gf2__blk1325_dn7) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) / (2.0 * assign49030_e62913)))), (((0.5 * locals.var_gf2__blk1325_dn8) * assign49030_e62915) + (assign49030_e62907 * ((-((8.0 * locals.var_gf2__blk1325_dn8) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) / (2.0 * assign49030_e62913)))), (((0.5 * locals.var_gf2__blk1325_dn9) * assign49030_e62915) + (assign49030_e62907 * ((-((8.0 * locals.var_gf2__blk1325_dn9) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) / (2.0 * assign49030_e62913)))), );
            locals.var_qbscr__blk1358_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            (locals.var_qiscr__blk1357, locals.var_qiscr__blk1357_dn4, locals.var_qiscr__blk1357_dn6, locals.var_qiscr__blk1357_dn7, locals.var_qiscr__blk1357_dn8, locals.var_qiscr__blk1357_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qiscr__blk1357_rv = 0.0;
            (locals.var_fscr__blk1359, locals.var_fscr__blk1359_dn4, locals.var_fscr__blk1359_dn6, locals.var_fscr__blk1359_dn7, locals.var_fscr__blk1359_dn8, locals.var_fscr__blk1359_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_fscr__blk1359_rv = 0.0;
        }

        let assign49060_e62933: f64 = (-30.0);
        let assign49060_e62934: f64 = if locals.var_xgtscr__blk1352 > assign49060_e62933 { 1.0 } else { 0.0 };
        locals.var_guard1481 = assign49060_e62934;
        locals.var_guard1481_rv = 0.0;

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
            let assign49070_e62942: f64 = (locals.var_nscr__blk1350 * locals.var_xgtscr__blk1352);
            let assign49070_e62944: f64 = (assign49070_e62942 - 1.0);
            (locals.var_xgtscr0__blk1353, locals.var_xgtscr0__blk1353_dn4, locals.var_xgtscr0__blk1353_dn6, locals.var_xgtscr0__blk1353_dn7, locals.var_xgtscr0__blk1353_dn8, locals.var_xgtscr0__blk1353_dn9, ) = (assign49070_e62944, ((locals.var_nscr__blk1350_dn4 * locals.var_xgtscr__blk1352) + (locals.var_nscr__blk1350 * locals.var_xgtscr__blk1352_dn4)), ((locals.var_nscr__blk1350_dn6 * locals.var_xgtscr__blk1352) + (locals.var_nscr__blk1350 * locals.var_xgtscr__blk1352_dn6)), ((locals.var_nscr__blk1350_dn7 * locals.var_xgtscr__blk1352) + (locals.var_nscr__blk1350 * locals.var_xgtscr__blk1352_dn7)), ((locals.var_nscr__blk1350_dn8 * locals.var_xgtscr__blk1352) + (locals.var_nscr__blk1350 * locals.var_xgtscr__blk1352_dn8)), ((locals.var_nscr__blk1350_dn9 * locals.var_xgtscr__blk1352) + (locals.var_nscr__blk1350 * locals.var_xgtscr__blk1352_dn9)), );
            locals.var_xgtscr0__blk1353_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
            let assign49080_e62956: f64 = (locals.var_xgtscr0__blk1353 * locals.var_xgtscr0__blk1353);
            let assign49080_e62958: f64 = (assign49080_e62956 + 10.0);
            let assign49080_e62959: f64 = (assign49080_e62958).sqrt();
            let assign49080_e62960: f64 = (locals.var_xgtscr0__blk1353 + assign49080_e62959);
            let assign49080_e62961: f64 = (0.5 * assign49080_e62960);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign49080_e62961, (0.5 * (locals.var_xgtscr0__blk1353_dn4 + (((locals.var_xgtscr0__blk1353_dn4 * locals.var_xgtscr0__blk1353) + (locals.var_xgtscr0__blk1353 * locals.var_xgtscr0__blk1353_dn4)) / (2.0 * assign49080_e62959)))), (0.5 * (locals.var_xgtscr0__blk1353_dn6 + (((locals.var_xgtscr0__blk1353_dn6 * locals.var_xgtscr0__blk1353) + (locals.var_xgtscr0__blk1353 * locals.var_xgtscr0__blk1353_dn6)) / (2.0 * assign49080_e62959)))), (0.5 * (locals.var_xgtscr0__blk1353_dn7 + (((locals.var_xgtscr0__blk1353_dn7 * locals.var_xgtscr0__blk1353) + (locals.var_xgtscr0__blk1353 * locals.var_xgtscr0__blk1353_dn7)) / (2.0 * assign49080_e62959)))), (0.5 * (locals.var_xgtscr0__blk1353_dn8 + (((locals.var_xgtscr0__blk1353_dn8 * locals.var_xgtscr0__blk1353) + (locals.var_xgtscr0__blk1353 * locals.var_xgtscr0__blk1353_dn8)) / (2.0 * assign49080_e62959)))), (0.5 * (locals.var_xgtscr0__blk1353_dn9 + (((locals.var_xgtscr0__blk1353_dn9 * locals.var_xgtscr0__blk1353) + (locals.var_xgtscr0__blk1353 * locals.var_xgtscr0__blk1353_dn9)) / (2.0 * assign49080_e62959)))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
            let assign49090_e62971: f64 = (locals.var_temp__blk949).ln();
            let assign49090_e62972: f64 = (locals.var_xgtscr__blk1352 - assign49090_e62971);
            (locals.var_qiscr0si__blk1354, locals.var_qiscr0si__blk1354_dn4, locals.var_qiscr0si__blk1354_dn6, locals.var_qiscr0si__blk1354_dn7, locals.var_qiscr0si__blk1354_dn8, locals.var_qiscr0si__blk1354_dn9, ) = (assign49090_e62972, (locals.var_xgtscr__blk1352_dn4 - (locals.var_temp__blk949_dn4 / locals.var_temp__blk949)), (locals.var_xgtscr__blk1352_dn6 - (locals.var_temp__blk949_dn6 / locals.var_temp__blk949)), (locals.var_xgtscr__blk1352_dn7 - (locals.var_temp__blk949_dn7 / locals.var_temp__blk949)), (locals.var_xgtscr__blk1352_dn8 - (locals.var_temp__blk949_dn8 / locals.var_temp__blk949)), (locals.var_xgtscr__blk1352_dn9 - (locals.var_temp__blk949_dn9 / locals.var_temp__blk949)), );
            locals.var_qiscr0si__blk1354_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
            let assign49100_e62984: f64 = (locals.var_qiscr0si__blk1354 * locals.var_qiscr0si__blk1354);
            let assign49100_e62986: f64 = (assign49100_e62984 + 2.0);
            let assign49100_e62987: f64 = (assign49100_e62986).sqrt();
            let assign49100_e62988: f64 = (locals.var_qiscr0si__blk1354 + assign49100_e62987);
            let assign49100_e62989: f64 = (0.5 * assign49100_e62988);
            (locals.var_qiscr0__blk1355, locals.var_qiscr0__blk1355_dn4, locals.var_qiscr0__blk1355_dn6, locals.var_qiscr0__blk1355_dn7, locals.var_qiscr0__blk1355_dn8, locals.var_qiscr0__blk1355_dn9, ) = (assign49100_e62989, (0.5 * (locals.var_qiscr0si__blk1354_dn4 + (((locals.var_qiscr0si__blk1354_dn4 * locals.var_qiscr0si__blk1354) + (locals.var_qiscr0si__blk1354 * locals.var_qiscr0si__blk1354_dn4)) / (2.0 * assign49100_e62987)))), (0.5 * (locals.var_qiscr0si__blk1354_dn6 + (((locals.var_qiscr0si__blk1354_dn6 * locals.var_qiscr0si__blk1354) + (locals.var_qiscr0si__blk1354 * locals.var_qiscr0si__blk1354_dn6)) / (2.0 * assign49100_e62987)))), (0.5 * (locals.var_qiscr0si__blk1354_dn7 + (((locals.var_qiscr0si__blk1354_dn7 * locals.var_qiscr0si__blk1354) + (locals.var_qiscr0si__blk1354 * locals.var_qiscr0si__blk1354_dn7)) / (2.0 * assign49100_e62987)))), (0.5 * (locals.var_qiscr0si__blk1354_dn8 + (((locals.var_qiscr0si__blk1354_dn8 * locals.var_qiscr0si__blk1354) + (locals.var_qiscr0si__blk1354 * locals.var_qiscr0si__blk1354_dn8)) / (2.0 * assign49100_e62987)))), (0.5 * (locals.var_qiscr0si__blk1354_dn9 + (((locals.var_qiscr0si__blk1354_dn9 * locals.var_qiscr0si__blk1354) + (locals.var_qiscr0si__blk1354 * locals.var_qiscr0si__blk1354_dn9)) / (2.0 * assign49100_e62987)))), );
            locals.var_qiscr0__blk1355_rv = 0.0;
        }

        let assign49110_e62994: f64 = (locals.var_xgtscr__blk1352 - locals.var_qiscr0__blk1355);
        let assign49110_e62996: f64 = if assign49110_e62994 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1482 = assign49110_e62996;
        locals.var_guard1482_rv = 0.0;

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) && (locals.var_guard1482 != 0.0)) {
            let assign49120_e63006: f64 = (locals.var_xgtscr__blk1352 - locals.var_qiscr0__blk1355);
            let assign49120_e63007: f64 = (assign49120_e63006).exp();
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign49120_e63007, (assign49120_e63007 * (locals.var_xgtscr__blk1352_dn4 - locals.var_qiscr0__blk1355_dn4)), (assign49120_e63007 * (locals.var_xgtscr__blk1352_dn6 - locals.var_qiscr0__blk1355_dn6)), (assign49120_e63007 * (locals.var_xgtscr__blk1352_dn7 - locals.var_qiscr0__blk1355_dn7)), (assign49120_e63007 * (locals.var_xgtscr__blk1352_dn8 - locals.var_qiscr0__blk1355_dn8)), (assign49120_e63007 * (locals.var_xgtscr__blk1352_dn9 - locals.var_qiscr0__blk1355_dn9)), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) && (locals.var_guard1482 == 0.0)) {
            let assign49130_e63022: f64 = (locals.var_xgtscr__blk1352 - locals.var_qiscr0__blk1355);
            let assign49130_e63024: f64 = (assign49130_e63022 - 230.25850929940458);
            let assign49130_e63029: f64 = (locals.var_xgtscr__blk1352 - locals.var_qiscr0__blk1355);
            let assign49130_e63031: f64 = (assign49130_e63029 - 230.25850929940458);
            let assign49130_e63035: f64 = (locals.var_xgtscr__blk1352 - locals.var_qiscr0__blk1355);
            let assign49130_e63037: f64 = (assign49130_e63035 - 230.25850929940458);
            let assign49130_e63039: f64 = (assign49130_e63037 * 0.3333333333333333);
            let assign49130_e63040: f64 = (1.0 + assign49130_e63039);
            let assign49130_e63041: f64 = (assign49130_e63031 * assign49130_e63040);
            let assign49130_e63042: f64 = (0.5 * assign49130_e63041);
            let assign49130_e63043: f64 = (1.0 + assign49130_e63042);
            let assign49130_e63044: f64 = (assign49130_e63024 * assign49130_e63043);
            let assign49130_e63045: f64 = (1.0 + assign49130_e63044);
            let assign49130_e63046: f64 = (1e100 * assign49130_e63045);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign49130_e63046, (1e100 * (((locals.var_xgtscr__blk1352_dn4 - locals.var_qiscr0__blk1355_dn4) * assign49130_e63043) + (assign49130_e63024 * (0.5 * (((locals.var_xgtscr__blk1352_dn4 - locals.var_qiscr0__blk1355_dn4) * assign49130_e63040) + (assign49130_e63031 * ((locals.var_xgtscr__blk1352_dn4 - locals.var_qiscr0__blk1355_dn4) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr__blk1352_dn6 - locals.var_qiscr0__blk1355_dn6) * assign49130_e63043) + (assign49130_e63024 * (0.5 * (((locals.var_xgtscr__blk1352_dn6 - locals.var_qiscr0__blk1355_dn6) * assign49130_e63040) + (assign49130_e63031 * ((locals.var_xgtscr__blk1352_dn6 - locals.var_qiscr0__blk1355_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr__blk1352_dn7 - locals.var_qiscr0__blk1355_dn7) * assign49130_e63043) + (assign49130_e63024 * (0.5 * (((locals.var_xgtscr__blk1352_dn7 - locals.var_qiscr0__blk1355_dn7) * assign49130_e63040) + (assign49130_e63031 * ((locals.var_xgtscr__blk1352_dn7 - locals.var_qiscr0__blk1355_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr__blk1352_dn8 - locals.var_qiscr0__blk1355_dn8) * assign49130_e63043) + (assign49130_e63024 * (0.5 * (((locals.var_xgtscr__blk1352_dn8 - locals.var_qiscr0__blk1355_dn8) * assign49130_e63040) + (assign49130_e63031 * ((locals.var_xgtscr__blk1352_dn8 - locals.var_qiscr0__blk1355_dn8) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr__blk1352_dn9 - locals.var_qiscr0__blk1355_dn9) * assign49130_e63043) + (assign49130_e63024 * (0.5 * (((locals.var_xgtscr__blk1352_dn9 - locals.var_qiscr0__blk1355_dn9) * assign49130_e63040) + (assign49130_e63031 * ((locals.var_xgtscr__blk1352_dn9 - locals.var_qiscr0__blk1355_dn9) * 0.3333333333333333))))))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
            let assign49140_e63056: f64 = (locals.var_temp__blk949 / locals.var_nscr__blk1350);
            (locals.var_dscr0__blk1356, locals.var_dscr0__blk1356_dn4, locals.var_dscr0__blk1356_dn6, locals.var_dscr0__blk1356_dn7, locals.var_dscr0__blk1356_dn8, locals.var_dscr0__blk1356_dn9, ) = (assign49140_e63056, (((locals.var_temp__blk949_dn4 * locals.var_nscr__blk1350) - (locals.var_temp__blk949 * locals.var_nscr__blk1350_dn4)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), (((locals.var_temp__blk949_dn6 * locals.var_nscr__blk1350) - (locals.var_temp__blk949 * locals.var_nscr__blk1350_dn6)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), (((locals.var_temp__blk949_dn7 * locals.var_nscr__blk1350) - (locals.var_temp__blk949 * locals.var_nscr__blk1350_dn7)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), (((locals.var_temp__blk949_dn8 * locals.var_nscr__blk1350) - (locals.var_temp__blk949 * locals.var_nscr__blk1350_dn8)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), (((locals.var_temp__blk949_dn9 * locals.var_nscr__blk1350) - (locals.var_temp__blk949 * locals.var_nscr__blk1350_dn9)) / (locals.var_nscr__blk1350 * locals.var_nscr__blk1350)), );
            locals.var_dscr0__blk1356_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
            let assign49150_e63067: f64 = (locals.var_qiscr0__blk1355 + 1.0);
            let assign49150_e63068: f64 = (2.0 * assign49150_e63067);
            let assign49150_e63070: f64 = (assign49150_e63068 - locals.var_dscr0__blk1356);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign49150_e63070, ((2.0 * locals.var_qiscr0__blk1355_dn4) - locals.var_dscr0__blk1356_dn4), ((2.0 * locals.var_qiscr0__blk1355_dn6) - locals.var_dscr0__blk1356_dn6), ((2.0 * locals.var_qiscr0__blk1355_dn7) - locals.var_dscr0__blk1356_dn7), ((2.0 * locals.var_qiscr0__blk1355_dn8) - locals.var_dscr0__blk1356_dn8), ((2.0 * locals.var_qiscr0__blk1355_dn9) - locals.var_dscr0__blk1356_dn9), );
            locals.var_temp__blk949_rv = 0.0;
        }

        let assign49160_e63075: f64 = if locals.var_dscr0__blk1356 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1483 = assign49160_e63075;
        locals.var_guard1483_rv = 0.0;

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) && (locals.var_guard1483 != 0.0)) {
            let assign49170_e63088: f64 = (locals.var_dscr0__blk1356 * locals.var_temp__blk949);
            let assign49170_e63089: f64 = (1.0 + assign49170_e63088);
            let assign49170_e63090: f64 = (assign49170_e63089).sqrt();
            let assign49170_e63092: f64 = (assign49170_e63090 - 1.0);
            let assign49170_e63094: f64 = (assign49170_e63092 / locals.var_dscr0__blk1356);
            let assign49170_e63095: f64 = (locals.var_qiscr0__blk1355 - assign49170_e63094);
            let assign49170_e63097: f64 = (assign49170_e63095 + 1.0);
            let assign49170_e63098: f64 = (locals.var_nscr__blk1350 * assign49170_e63097);
            (locals.var_qiscr__blk1357, locals.var_qiscr__blk1357_dn4, locals.var_qiscr__blk1357_dn6, locals.var_qiscr__blk1357_dn7, locals.var_qiscr__blk1357_dn8, locals.var_qiscr__blk1357_dn9, ) = (assign49170_e63098, ((locals.var_nscr__blk1350_dn4 * assign49170_e63097) + (locals.var_nscr__blk1350 * (locals.var_qiscr0__blk1355_dn4 - ((((((locals.var_dscr0__blk1356_dn4 * locals.var_temp__blk949) + (locals.var_dscr0__blk1356 * locals.var_temp__blk949_dn4)) / (2.0 * assign49170_e63090)) * locals.var_dscr0__blk1356) - (assign49170_e63092 * locals.var_dscr0__blk1356_dn4)) / (locals.var_dscr0__blk1356 * locals.var_dscr0__blk1356))))), ((locals.var_nscr__blk1350_dn6 * assign49170_e63097) + (locals.var_nscr__blk1350 * (locals.var_qiscr0__blk1355_dn6 - ((((((locals.var_dscr0__blk1356_dn6 * locals.var_temp__blk949) + (locals.var_dscr0__blk1356 * locals.var_temp__blk949_dn6)) / (2.0 * assign49170_e63090)) * locals.var_dscr0__blk1356) - (assign49170_e63092 * locals.var_dscr0__blk1356_dn6)) / (locals.var_dscr0__blk1356 * locals.var_dscr0__blk1356))))), ((locals.var_nscr__blk1350_dn7 * assign49170_e63097) + (locals.var_nscr__blk1350 * (locals.var_qiscr0__blk1355_dn7 - ((((((locals.var_dscr0__blk1356_dn7 * locals.var_temp__blk949) + (locals.var_dscr0__blk1356 * locals.var_temp__blk949_dn7)) / (2.0 * assign49170_e63090)) * locals.var_dscr0__blk1356) - (assign49170_e63092 * locals.var_dscr0__blk1356_dn7)) / (locals.var_dscr0__blk1356 * locals.var_dscr0__blk1356))))), ((locals.var_nscr__blk1350_dn8 * assign49170_e63097) + (locals.var_nscr__blk1350 * (locals.var_qiscr0__blk1355_dn8 - ((((((locals.var_dscr0__blk1356_dn8 * locals.var_temp__blk949) + (locals.var_dscr0__blk1356 * locals.var_temp__blk949_dn8)) / (2.0 * assign49170_e63090)) * locals.var_dscr0__blk1356) - (assign49170_e63092 * locals.var_dscr0__blk1356_dn8)) / (locals.var_dscr0__blk1356 * locals.var_dscr0__blk1356))))), ((locals.var_nscr__blk1350_dn9 * assign49170_e63097) + (locals.var_nscr__blk1350 * (locals.var_qiscr0__blk1355_dn9 - ((((((locals.var_dscr0__blk1356_dn9 * locals.var_temp__blk949) + (locals.var_dscr0__blk1356 * locals.var_temp__blk949_dn9)) / (2.0 * assign49170_e63090)) * locals.var_dscr0__blk1356) - (assign49170_e63092 * locals.var_dscr0__blk1356_dn9)) / (locals.var_dscr0__blk1356 * locals.var_dscr0__blk1356))))), );
            locals.var_qiscr__blk1357_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) && (locals.var_guard1483 == 0.0)) {
            let assign49180_e63111: f64 = (locals.var_nscr__blk1350 * 0.5);
            let assign49180_e63113: f64 = (assign49180_e63111 * locals.var_dscr0__blk1356);
            let assign49180_e63117: f64 = (0.25 * locals.var_temp__blk949);
            let assign49180_e63119: f64 = (assign49180_e63117 * locals.var_temp__blk949);
            let assign49180_e63120: f64 = (1.0 + assign49180_e63119);
            let assign49180_e63121: f64 = (assign49180_e63113 * assign49180_e63120);
            (locals.var_qiscr__blk1357, locals.var_qiscr__blk1357_dn4, locals.var_qiscr__blk1357_dn6, locals.var_qiscr__blk1357_dn7, locals.var_qiscr__blk1357_dn8, locals.var_qiscr__blk1357_dn9, ) = (assign49180_e63121, (((((locals.var_nscr__blk1350_dn4 * 0.5) * locals.var_dscr0__blk1356) + (assign49180_e63111 * locals.var_dscr0__blk1356_dn4)) * assign49180_e63120) + (assign49180_e63113 * (((0.25 * locals.var_temp__blk949_dn4) * locals.var_temp__blk949) + (assign49180_e63117 * locals.var_temp__blk949_dn4)))), (((((locals.var_nscr__blk1350_dn6 * 0.5) * locals.var_dscr0__blk1356) + (assign49180_e63111 * locals.var_dscr0__blk1356_dn6)) * assign49180_e63120) + (assign49180_e63113 * (((0.25 * locals.var_temp__blk949_dn6) * locals.var_temp__blk949) + (assign49180_e63117 * locals.var_temp__blk949_dn6)))), (((((locals.var_nscr__blk1350_dn7 * 0.5) * locals.var_dscr0__blk1356) + (assign49180_e63111 * locals.var_dscr0__blk1356_dn7)) * assign49180_e63120) + (assign49180_e63113 * (((0.25 * locals.var_temp__blk949_dn7) * locals.var_temp__blk949) + (assign49180_e63117 * locals.var_temp__blk949_dn7)))), (((((locals.var_nscr__blk1350_dn8 * 0.5) * locals.var_dscr0__blk1356) + (assign49180_e63111 * locals.var_dscr0__blk1356_dn8)) * assign49180_e63120) + (assign49180_e63113 * (((0.25 * locals.var_temp__blk949_dn8) * locals.var_temp__blk949) + (assign49180_e63117 * locals.var_temp__blk949_dn8)))), (((((locals.var_nscr__blk1350_dn9 * 0.5) * locals.var_dscr0__blk1356) + (assign49180_e63111 * locals.var_dscr0__blk1356_dn9)) * assign49180_e63120) + (assign49180_e63113 * (((0.25 * locals.var_temp__blk949_dn9) * locals.var_temp__blk949) + (assign49180_e63117 * locals.var_temp__blk949_dn9)))), );
            locals.var_qiscr__blk1357_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
            let assign49190_e63132: f64 = (locals.var_xg__blk1343 - locals.var_qiscr__blk1357);
            let assign49190_e63134: f64 = (assign49190_e63132 + 2.0);
            let assign49190_e63137: f64 = (locals.var_xg__blk1343 - locals.var_qiscr__blk1357);
            let assign49190_e63139: f64 = (assign49190_e63137 - 2.0);
            let assign49190_e63142: f64 = (locals.var_xg__blk1343 - locals.var_qiscr__blk1357);
            let assign49190_e63144: f64 = (assign49190_e63142 - 2.0);
            let assign49190_e63145: f64 = (assign49190_e63139 * assign49190_e63144);
            let assign49190_e63147: f64 = (assign49190_e63145 + 1.0);
            let assign49190_e63148: f64 = (assign49190_e63147).sqrt();
            let assign49190_e63149: f64 = (assign49190_e63134 + assign49190_e63148);
            let assign49190_e63150: f64 = (0.5 * assign49190_e63149);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign49190_e63150, (0.5 * ((locals.var_xg__blk1343_dn4 - locals.var_qiscr__blk1357_dn4) + ((((locals.var_xg__blk1343_dn4 - locals.var_qiscr__blk1357_dn4) * assign49190_e63144) + (assign49190_e63139 * (locals.var_xg__blk1343_dn4 - locals.var_qiscr__blk1357_dn4))) / (2.0 * assign49190_e63148)))), (0.5 * ((locals.var_xg__blk1343_dn6 - locals.var_qiscr__blk1357_dn6) + ((((locals.var_xg__blk1343_dn6 - locals.var_qiscr__blk1357_dn6) * assign49190_e63144) + (assign49190_e63139 * (locals.var_xg__blk1343_dn6 - locals.var_qiscr__blk1357_dn6))) / (2.0 * assign49190_e63148)))), (0.5 * ((locals.var_xg__blk1343_dn7 - locals.var_qiscr__blk1357_dn7) + ((((locals.var_xg__blk1343_dn7 - locals.var_qiscr__blk1357_dn7) * assign49190_e63144) + (assign49190_e63139 * (locals.var_xg__blk1343_dn7 - locals.var_qiscr__blk1357_dn7))) / (2.0 * assign49190_e63148)))), (0.5 * ((locals.var_xg__blk1343_dn8 - locals.var_qiscr__blk1357_dn8) + ((((locals.var_xg__blk1343_dn8 - locals.var_qiscr__blk1357_dn8) * assign49190_e63144) + (assign49190_e63139 * (locals.var_xg__blk1343_dn8 - locals.var_qiscr__blk1357_dn8))) / (2.0 * assign49190_e63148)))), (0.5 * ((locals.var_xg__blk1343_dn9 - locals.var_qiscr__blk1357_dn9) + ((((locals.var_xg__blk1343_dn9 - locals.var_qiscr__blk1357_dn9) * assign49190_e63144) + (assign49190_e63139 * (locals.var_xg__blk1343_dn9 - locals.var_qiscr__blk1357_dn9))) / (2.0 * assign49190_e63148)))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
            let assign49200_e63160: f64 = (0.5 * locals.var_gf2__blk1325);
            let assign49200_e63164: f64 = (4.0 / locals.var_gf2__blk1325);
            let assign49200_e63166: f64 = (assign49200_e63164 * locals.var_temp__blk949);
            let assign49200_e63167: f64 = (1.0 + assign49200_e63166);
            let assign49200_e63168: f64 = (assign49200_e63167).sqrt();
            let assign49200_e63170: f64 = (assign49200_e63168 - 1.0);
            let assign49200_e63171: f64 = (assign49200_e63160 * assign49200_e63170);
            (locals.var_qbscr__blk1358, locals.var_qbscr__blk1358_dn4, locals.var_qbscr__blk1358_dn6, locals.var_qbscr__blk1358_dn7, locals.var_qbscr__blk1358_dn8, locals.var_qbscr__blk1358_dn9, ) = (assign49200_e63171, (((0.5 * locals.var_gf2__blk1325_dn4) * assign49200_e63170) + (assign49200_e63160 * ((((-((4.0 * locals.var_gf2__blk1325_dn4) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) * locals.var_temp__blk949) + (assign49200_e63164 * locals.var_temp__blk949_dn4)) / (2.0 * assign49200_e63168)))), (((0.5 * locals.var_gf2__blk1325_dn6) * assign49200_e63170) + (assign49200_e63160 * ((((-((4.0 * locals.var_gf2__blk1325_dn6) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) * locals.var_temp__blk949) + (assign49200_e63164 * locals.var_temp__blk949_dn6)) / (2.0 * assign49200_e63168)))), (((0.5 * locals.var_gf2__blk1325_dn7) * assign49200_e63170) + (assign49200_e63160 * ((((-((4.0 * locals.var_gf2__blk1325_dn7) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) * locals.var_temp__blk949) + (assign49200_e63164 * locals.var_temp__blk949_dn7)) / (2.0 * assign49200_e63168)))), (((0.5 * locals.var_gf2__blk1325_dn8) * assign49200_e63170) + (assign49200_e63160 * ((((-((4.0 * locals.var_gf2__blk1325_dn8) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) * locals.var_temp__blk949) + (assign49200_e63164 * locals.var_temp__blk949_dn8)) / (2.0 * assign49200_e63168)))), (((0.5 * locals.var_gf2__blk1325_dn9) * assign49200_e63170) + (assign49200_e63160 * ((((-((4.0 * locals.var_gf2__blk1325_dn9) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325))) * locals.var_temp__blk949) + (assign49200_e63164 * locals.var_temp__blk949_dn9)) / (2.0 * assign49200_e63168)))), );
            locals.var_qbscr__blk1358_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
            let assign49210_e63182: f64 = (locals.var_qbscr__blk1358 + locals.var_qiscr__blk1357);
            let assign49210_e63183: f64 = (locals.var_qbscr__blk1358 / assign49210_e63182);
            (locals.var_fscr__blk1359, locals.var_fscr__blk1359_dn4, locals.var_fscr__blk1359_dn6, locals.var_fscr__blk1359_dn7, locals.var_fscr__blk1359_dn8, locals.var_fscr__blk1359_dn9, ) = (assign49210_e63183, (((locals.var_qbscr__blk1358_dn4 * assign49210_e63182) - (locals.var_qbscr__blk1358 * (locals.var_qbscr__blk1358_dn4 + locals.var_qiscr__blk1357_dn4))) / (assign49210_e63182 * assign49210_e63182)), (((locals.var_qbscr__blk1358_dn6 * assign49210_e63182) - (locals.var_qbscr__blk1358 * (locals.var_qbscr__blk1358_dn6 + locals.var_qiscr__blk1357_dn6))) / (assign49210_e63182 * assign49210_e63182)), (((locals.var_qbscr__blk1358_dn7 * assign49210_e63182) - (locals.var_qbscr__blk1358 * (locals.var_qbscr__blk1358_dn7 + locals.var_qiscr__blk1357_dn7))) / (assign49210_e63182 * assign49210_e63182)), (((locals.var_qbscr__blk1358_dn8 * assign49210_e63182) - (locals.var_qbscr__blk1358 * (locals.var_qbscr__blk1358_dn8 + locals.var_qiscr__blk1357_dn8))) / (assign49210_e63182 * assign49210_e63182)), (((locals.var_qbscr__blk1358_dn9 * assign49210_e63182) - (locals.var_qbscr__blk1358 * (locals.var_qbscr__blk1358_dn9 + locals.var_qiscr__blk1357_dn9))) / (assign49210_e63182 * assign49210_e63182)), );
            locals.var_fscr__blk1359_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_23(
        locals: &mut StampLocals,
    ) {
        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1481 != 0.0)) {
            let assign49220_e63194: f64 = (locals.var_fscr__blk1359 * locals.var_delxb__blk1347);
            let assign49220_e63195: f64 = (locals.var_xno_s__blk1348 - assign49220_e63194);
            (locals.var_xn_s__blk1349, locals.var_xn_s__blk1349_dn4, locals.var_xn_s__blk1349_dn6, locals.var_xn_s__blk1349_dn7, locals.var_xn_s__blk1349_dn8, locals.var_xn_s__blk1349_dn9, ) = (assign49220_e63195, (locals.var_xno_s__blk1348_dn4 - ((locals.var_fscr__blk1359_dn4 * locals.var_delxb__blk1347) + (locals.var_fscr__blk1359 * locals.var_delxb__blk1347_dn4))), (locals.var_xno_s__blk1348_dn6 - ((locals.var_fscr__blk1359_dn6 * locals.var_delxb__blk1347) + (locals.var_fscr__blk1359 * locals.var_delxb__blk1347_dn6))), (locals.var_xno_s__blk1348_dn7 - ((locals.var_fscr__blk1359_dn7 * locals.var_delxb__blk1347) + (locals.var_fscr__blk1359 * locals.var_delxb__blk1347_dn7))), (locals.var_xno_s__blk1348_dn8 - ((locals.var_fscr__blk1359_dn8 * locals.var_delxb__blk1347) + (locals.var_fscr__blk1359 * locals.var_delxb__blk1347_dn8))), (locals.var_xno_s__blk1348_dn9 - ((locals.var_fscr__blk1359_dn9 * locals.var_delxb__blk1347) + (locals.var_fscr__blk1359 * locals.var_delxb__blk1347_dn9))), );
            locals.var_xn_s__blk1349_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign49230_e63204: f64 = (locals.var_gf__blk1324 * 0.7071067811865475);
            let assign49230_e63205: f64 = (1.0 + assign49230_e63204);
            (locals.var_xi__blk1360, locals.var_xi__blk1360_dn4, locals.var_xi__blk1360_dn6, locals.var_xi__blk1360_dn7, locals.var_xi__blk1360_dn8, locals.var_xi__blk1360_dn9, ) = (assign49230_e63205, (locals.var_gf__blk1324_dn4 * 0.7071067811865475), (locals.var_gf__blk1324_dn6 * 0.7071067811865475), (locals.var_gf__blk1324_dn7 * 0.7071067811865475), (locals.var_gf__blk1324_dn8 * 0.7071067811865475), (locals.var_gf__blk1324_dn9 * 0.7071067811865475), );
            locals.var_xi__blk1360_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign49240_e63213: f64 = (1e-5 * locals.var_xi__blk1360);
            (locals.var_margin__blk1361, locals.var_margin__blk1361_dn4, locals.var_margin__blk1361_dn6, locals.var_margin__blk1361_dn7, locals.var_margin__blk1361_dn8, locals.var_margin__blk1361_dn9, ) = (assign49240_e63213, (1e-5 * locals.var_xi__blk1360_dn4), (1e-5 * locals.var_xi__blk1360_dn6), (1e-5 * locals.var_xi__blk1360_dn7), (1e-5 * locals.var_xi__blk1360_dn8), (1e-5 * locals.var_xi__blk1360_dn9), );
            locals.var_margin__blk1361_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign49250_e63221: f64 = (1.0 / locals.var_xi__blk1360);
            (locals.var_inv_xi__blk1362, locals.var_inv_xi__blk1362_dn4, locals.var_inv_xi__blk1362_dn6, locals.var_inv_xi__blk1362_dn7, locals.var_inv_xi__blk1362_dn8, locals.var_inv_xi__blk1362_dn9, ) = (assign49250_e63221, (-(locals.var_xi__blk1360_dn4 / (locals.var_xi__blk1360 * locals.var_xi__blk1360))), (-(locals.var_xi__blk1360_dn6 / (locals.var_xi__blk1360 * locals.var_xi__blk1360))), (-(locals.var_xi__blk1360_dn7 / (locals.var_xi__blk1360 * locals.var_xi__blk1360))), (-(locals.var_xi__blk1360_dn8 / (locals.var_xi__blk1360 * locals.var_xi__blk1360))), (-(locals.var_xi__blk1360_dn9 / (locals.var_xi__blk1360 * locals.var_xi__blk1360))), );
            locals.var_inv_xi__blk1362_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            (locals.var_sp_s_x1__blk1469, locals.var_sp_s_x1__blk1469_dn4, locals.var_sp_s_x1__blk1469_dn6, locals.var_sp_s_x1__blk1469_dn7, locals.var_sp_s_x1__blk1469_dn8, locals.var_sp_s_x1__blk1469_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_sp_s_x1__blk1469_rv = 0.0;
            (locals.var_x_s__blk1363, locals.var_x_s__blk1363_dn4, locals.var_x_s__blk1363_dn6, locals.var_x_s__blk1363_dn7, locals.var_x_s__blk1363_dn8, locals.var_x_s__blk1363_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_x_s__blk1363_rv = 0.0;
        }

        let assign49280_e63238: f64 = if locals.var_xn_s__blk1349 < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1484 = assign49280_e63238;
        locals.var_guard1484_rv = 0.0;

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1484 != 0.0)) {
            let assign49290_e63245: f64 = (-locals.var_xn_s__blk1349);
            let assign49290_e63246: f64 = (assign49290_e63245).exp();
            (locals.var_delta_ns__blk1364, locals.var_delta_ns__blk1364_dn4, locals.var_delta_ns__blk1364_dn6, locals.var_delta_ns__blk1364_dn7, locals.var_delta_ns__blk1364_dn8, locals.var_delta_ns__blk1364_dn9, ) = (assign49290_e63246, (assign49290_e63246 * (-locals.var_xn_s__blk1349_dn4)), (assign49290_e63246 * (-locals.var_xn_s__blk1349_dn6)), (assign49290_e63246 * (-locals.var_xn_s__blk1349_dn7)), (assign49290_e63246 * (-locals.var_xn_s__blk1349_dn8)), (assign49290_e63246 * (-locals.var_xn_s__blk1349_dn9)), );
            locals.var_delta_ns__blk1364_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1484 == 0.0)) {
            let assign49300_e63259: f64 = (locals.var_xn_s__blk1349 - 460.51701859880916);
            let assign49300_e63264: f64 = (locals.var_xn_s__blk1349 - 460.51701859880916);
            let assign49300_e63268: f64 = (locals.var_xn_s__blk1349 - 460.51701859880916);
            let assign49300_e63270: f64 = (assign49300_e63268 * 0.3333333333333333);
            let assign49300_e63271: f64 = (1.0 + assign49300_e63270);
            let assign49300_e63272: f64 = (assign49300_e63264 * assign49300_e63271);
            let assign49300_e63273: f64 = (0.5 * assign49300_e63272);
            let assign49300_e63274: f64 = (1.0 + assign49300_e63273);
            let assign49300_e63275: f64 = (assign49300_e63259 * assign49300_e63274);
            let assign49300_e63276: f64 = (1.0 + assign49300_e63275);
            let assign49300_e63277: f64 = (1e-200 / assign49300_e63276);
            (locals.var_delta_ns__blk1364, locals.var_delta_ns__blk1364_dn4, locals.var_delta_ns__blk1364_dn6, locals.var_delta_ns__blk1364_dn7, locals.var_delta_ns__blk1364_dn8, locals.var_delta_ns__blk1364_dn9, ) = (assign49300_e63277, (-((1e-200 * ((locals.var_xn_s__blk1349_dn4 * assign49300_e63274) + (assign49300_e63259 * (0.5 * ((locals.var_xn_s__blk1349_dn4 * assign49300_e63271) + (assign49300_e63264 * (locals.var_xn_s__blk1349_dn4 * 0.3333333333333333))))))) / (assign49300_e63276 * assign49300_e63276))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn6 * assign49300_e63274) + (assign49300_e63259 * (0.5 * ((locals.var_xn_s__blk1349_dn6 * assign49300_e63271) + (assign49300_e63264 * (locals.var_xn_s__blk1349_dn6 * 0.3333333333333333))))))) / (assign49300_e63276 * assign49300_e63276))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn7 * assign49300_e63274) + (assign49300_e63259 * (0.5 * ((locals.var_xn_s__blk1349_dn7 * assign49300_e63271) + (assign49300_e63264 * (locals.var_xn_s__blk1349_dn7 * 0.3333333333333333))))))) / (assign49300_e63276 * assign49300_e63276))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn8 * assign49300_e63274) + (assign49300_e63259 * (0.5 * ((locals.var_xn_s__blk1349_dn8 * assign49300_e63271) + (assign49300_e63264 * (locals.var_xn_s__blk1349_dn8 * 0.3333333333333333))))))) / (assign49300_e63276 * assign49300_e63276))), (-((1e-200 * ((locals.var_xn_s__blk1349_dn9 * assign49300_e63274) + (assign49300_e63259 * (0.5 * ((locals.var_xn_s__blk1349_dn9 * assign49300_e63271) + (assign49300_e63264 * (locals.var_xn_s__blk1349_dn9 * 0.3333333333333333))))))) / (assign49300_e63276 * assign49300_e63276))), );
            locals.var_delta_ns__blk1364_rv = 0.0;
        }

        let assign49310_e63281: f64 = (locals.var_xg__blk1343).abs();
        let assign49310_e63283: f64 = if assign49310_e63281 <= locals.var_margin__blk1361 { 1.0 } else { 0.0 };
        locals.var_guard1485 = assign49310_e63283;
        locals.var_guard1485_rv = 0.0;

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 != 0.0)) {
            let assign49320_e63291: f64 = (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362);
            let assign49320_e63293: f64 = (assign49320_e63291 * 0.16666666666666666);
            let assign49320_e63295: f64 = (assign49320_e63293 * 0.7071067811865475);
            (locals.var_sp_s_temp1__blk1449, locals.var_sp_s_temp1__blk1449_dn4, locals.var_sp_s_temp1__blk1449_dn6, locals.var_sp_s_temp1__blk1449_dn7, locals.var_sp_s_temp1__blk1449_dn8, locals.var_sp_s_temp1__blk1449_dn9, ) = (assign49320_e63295, ((((locals.var_inv_xi__blk1362_dn4 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn4)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn6 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn7 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn8 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn8)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn9 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn9)) * 0.16666666666666666) * 0.7071067811865475), );
            locals.var_sp_s_temp1__blk1449_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 != 0.0)) {
            let assign49330_e63305: f64 = (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362);
            let assign49330_e63310: f64 = (1.0 - locals.var_delta_ns__blk1364);
            let assign49330_e63311: f64 = (locals.var_xg__blk1343 * assign49330_e63310);
            let assign49330_e63313: f64 = (assign49330_e63311 * locals.var_gf__blk1324);
            let assign49330_e63315: f64 = (assign49330_e63313 * locals.var_sp_s_temp1__blk1449);
            let assign49330_e63316: f64 = (1.0 + assign49330_e63315);
            let assign49330_e63317: f64 = (assign49330_e63305 * assign49330_e63316);
            (locals.var_x_s__blk1363, locals.var_x_s__blk1363_dn4, locals.var_x_s__blk1363_dn6, locals.var_x_s__blk1363_dn7, locals.var_x_s__blk1363_dn8, locals.var_x_s__blk1363_dn9, ) = (assign49330_e63317, ((((locals.var_xg__blk1343_dn4 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn4)) * assign49330_e63316) + (assign49330_e63305 * ((((((locals.var_xg__blk1343_dn4 * assign49330_e63310) + (locals.var_xg__blk1343 * (-locals.var_delta_ns__blk1364_dn4))) * locals.var_gf__blk1324) + (assign49330_e63311 * locals.var_gf__blk1324_dn4)) * locals.var_sp_s_temp1__blk1449) + (assign49330_e63313 * locals.var_sp_s_temp1__blk1449_dn4)))), ((((locals.var_xg__blk1343_dn6 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn6)) * assign49330_e63316) + (assign49330_e63305 * ((((((locals.var_xg__blk1343_dn6 * assign49330_e63310) + (locals.var_xg__blk1343 * (-locals.var_delta_ns__blk1364_dn6))) * locals.var_gf__blk1324) + (assign49330_e63311 * locals.var_gf__blk1324_dn6)) * locals.var_sp_s_temp1__blk1449) + (assign49330_e63313 * locals.var_sp_s_temp1__blk1449_dn6)))), ((((locals.var_xg__blk1343_dn7 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn7)) * assign49330_e63316) + (assign49330_e63305 * ((((((locals.var_xg__blk1343_dn7 * assign49330_e63310) + (locals.var_xg__blk1343 * (-locals.var_delta_ns__blk1364_dn7))) * locals.var_gf__blk1324) + (assign49330_e63311 * locals.var_gf__blk1324_dn7)) * locals.var_sp_s_temp1__blk1449) + (assign49330_e63313 * locals.var_sp_s_temp1__blk1449_dn7)))), ((((locals.var_xg__blk1343_dn8 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn8)) * assign49330_e63316) + (assign49330_e63305 * ((((((locals.var_xg__blk1343_dn8 * assign49330_e63310) + (locals.var_xg__blk1343 * (-locals.var_delta_ns__blk1364_dn8))) * locals.var_gf__blk1324) + (assign49330_e63311 * locals.var_gf__blk1324_dn8)) * locals.var_sp_s_temp1__blk1449) + (assign49330_e63313 * locals.var_sp_s_temp1__blk1449_dn8)))), ((((locals.var_xg__blk1343_dn9 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn9)) * assign49330_e63316) + (assign49330_e63305 * ((((((locals.var_xg__blk1343_dn9 * assign49330_e63310) + (locals.var_xg__blk1343 * (-locals.var_delta_ns__blk1364_dn9))) * locals.var_gf__blk1324) + (assign49330_e63311 * locals.var_gf__blk1324_dn9)) * locals.var_sp_s_temp1__blk1449) + (assign49330_e63313 * locals.var_sp_s_temp1__blk1449_dn9)))), );
            locals.var_x_s__blk1363_rv = 0.0;
        }

        let assign49340_e63322: f64 = (-locals.var_margin__blk1361);
        let assign49340_e63323: f64 = if locals.var_xg__blk1343 < assign49340_e63322 { 1.0 } else { 0.0 };
        locals.var_guard1486 = assign49340_e63323;
        locals.var_guard1486_rv = 0.0;

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign49350_e63333: f64 = (-locals.var_xg__blk1343);
            (locals.var_sp_s_yg__blk1451, locals.var_sp_s_yg__blk1451_dn4, locals.var_sp_s_yg__blk1451_dn6, locals.var_sp_s_yg__blk1451_dn7, locals.var_sp_s_yg__blk1451_dn8, locals.var_sp_s_yg__blk1451_dn9, ) = (assign49350_e63333, (-locals.var_xg__blk1343_dn4), (-locals.var_xg__blk1343_dn6), (-locals.var_xg__blk1343_dn7), (-locals.var_xg__blk1343_dn8), (-locals.var_xg__blk1343_dn9), );
            locals.var_sp_s_yg__blk1451_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign49360_e63347: f64 = (locals.var_sp_s_yg__blk1451 * locals.var_inv_xi__blk1362);
            let assign49360_e63348: f64 = (1.25 * assign49360_e63347);
            (locals.var_sp_s_ysub__blk1452, locals.var_sp_s_ysub__blk1452_dn4, locals.var_sp_s_ysub__blk1452_dn6, locals.var_sp_s_ysub__blk1452_dn7, locals.var_sp_s_ysub__blk1452_dn8, locals.var_sp_s_ysub__blk1452_dn9, ) = (assign49360_e63348, (1.25 * ((locals.var_sp_s_yg__blk1451_dn4 * locals.var_inv_xi__blk1362) + (locals.var_sp_s_yg__blk1451 * locals.var_inv_xi__blk1362_dn4))), (1.25 * ((locals.var_sp_s_yg__blk1451_dn6 * locals.var_inv_xi__blk1362) + (locals.var_sp_s_yg__blk1451 * locals.var_inv_xi__blk1362_dn6))), (1.25 * ((locals.var_sp_s_yg__blk1451_dn7 * locals.var_inv_xi__blk1362) + (locals.var_sp_s_yg__blk1451 * locals.var_inv_xi__blk1362_dn7))), (1.25 * ((locals.var_sp_s_yg__blk1451_dn8 * locals.var_inv_xi__blk1362) + (locals.var_sp_s_yg__blk1451 * locals.var_inv_xi__blk1362_dn8))), (1.25 * ((locals.var_sp_s_yg__blk1451_dn9 * locals.var_inv_xi__blk1362) + (locals.var_sp_s_yg__blk1451 * locals.var_inv_xi__blk1362_dn9))), );
            locals.var_sp_s_ysub__blk1452_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign49370_e63362: f64 = (locals.var_sp_s_ysub__blk1452 + 10.0);
            let assign49370_e63365: f64 = (locals.var_sp_s_ysub__blk1452 - 6.0);
            let assign49370_e63368: f64 = (locals.var_sp_s_ysub__blk1452 - 6.0);
            let assign49370_e63369: f64 = (assign49370_e63365 * assign49370_e63368);
            let assign49370_e63371: f64 = (assign49370_e63369 + 64.0);
            let assign49370_e63372: f64 = (assign49370_e63371).sqrt();
            let assign49370_e63373: f64 = (assign49370_e63362 - assign49370_e63372);
            let assign49370_e63374: f64 = (0.5 * assign49370_e63373);
            (locals.var_sp_s_eta__blk1453, locals.var_sp_s_eta__blk1453_dn4, locals.var_sp_s_eta__blk1453_dn6, locals.var_sp_s_eta__blk1453_dn7, locals.var_sp_s_eta__blk1453_dn8, locals.var_sp_s_eta__blk1453_dn9, ) = (assign49370_e63374, (0.5 * (locals.var_sp_s_ysub__blk1452_dn4 - (((locals.var_sp_s_ysub__blk1452_dn4 * assign49370_e63368) + (assign49370_e63365 * locals.var_sp_s_ysub__blk1452_dn4)) / (2.0 * assign49370_e63372)))), (0.5 * (locals.var_sp_s_ysub__blk1452_dn6 - (((locals.var_sp_s_ysub__blk1452_dn6 * assign49370_e63368) + (assign49370_e63365 * locals.var_sp_s_ysub__blk1452_dn6)) / (2.0 * assign49370_e63372)))), (0.5 * (locals.var_sp_s_ysub__blk1452_dn7 - (((locals.var_sp_s_ysub__blk1452_dn7 * assign49370_e63368) + (assign49370_e63365 * locals.var_sp_s_ysub__blk1452_dn7)) / (2.0 * assign49370_e63372)))), (0.5 * (locals.var_sp_s_ysub__blk1452_dn8 - (((locals.var_sp_s_ysub__blk1452_dn8 * assign49370_e63368) + (assign49370_e63365 * locals.var_sp_s_ysub__blk1452_dn8)) / (2.0 * assign49370_e63372)))), (0.5 * (locals.var_sp_s_ysub__blk1452_dn9 - (((locals.var_sp_s_ysub__blk1452_dn9 * assign49370_e63368) + (assign49370_e63365 * locals.var_sp_s_ysub__blk1452_dn9)) / (2.0 * assign49370_e63372)))), );
            locals.var_sp_s_eta__blk1453_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign49380_e63387: f64 = (locals.var_sp_s_yg__blk1451 - locals.var_sp_s_eta__blk1453);
            (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9, ) = (assign49380_e63387, (locals.var_sp_s_yg__blk1451_dn4 - locals.var_sp_s_eta__blk1453_dn4), (locals.var_sp_s_yg__blk1451_dn6 - locals.var_sp_s_eta__blk1453_dn6), (locals.var_sp_s_yg__blk1451_dn7 - locals.var_sp_s_eta__blk1453_dn7), (locals.var_sp_s_yg__blk1451_dn8 - locals.var_sp_s_eta__blk1453_dn8), (locals.var_sp_s_yg__blk1451_dn9 - locals.var_sp_s_eta__blk1453_dn9), );
            locals.var_sp_s_temp__blk1448_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign49390_e63400: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
            let assign49390_e63404: f64 = (locals.var_sp_s_eta__blk1453 + 1.0);
            let assign49390_e63405: f64 = (locals.var_gf2__blk1325 * assign49390_e63404);
            let assign49390_e63406: f64 = (assign49390_e63400 + assign49390_e63405);
            (locals.var_sp_s_a__blk1454, locals.var_sp_s_a__blk1454_dn4, locals.var_sp_s_a__blk1454_dn6, locals.var_sp_s_a__blk1454_dn7, locals.var_sp_s_a__blk1454_dn8, locals.var_sp_s_a__blk1454_dn9, ) = (assign49390_e63406, (((locals.var_sp_s_temp__blk1448_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn4)) + ((locals.var_gf2__blk1325_dn4 * assign49390_e63404) + (locals.var_gf2__blk1325 * locals.var_sp_s_eta__blk1453_dn4))), (((locals.var_sp_s_temp__blk1448_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn6)) + ((locals.var_gf2__blk1325_dn6 * assign49390_e63404) + (locals.var_gf2__blk1325 * locals.var_sp_s_eta__blk1453_dn6))), (((locals.var_sp_s_temp__blk1448_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn7)) + ((locals.var_gf2__blk1325_dn7 * assign49390_e63404) + (locals.var_gf2__blk1325 * locals.var_sp_s_eta__blk1453_dn7))), (((locals.var_sp_s_temp__blk1448_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn8)) + ((locals.var_gf2__blk1325_dn8 * assign49390_e63404) + (locals.var_gf2__blk1325 * locals.var_sp_s_eta__blk1453_dn8))), (((locals.var_sp_s_temp__blk1448_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn9)) + ((locals.var_gf2__blk1325_dn9 * assign49390_e63404) + (locals.var_gf2__blk1325 * locals.var_sp_s_eta__blk1453_dn9))), );
            locals.var_sp_s_a__blk1454_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign49400_e63419: f64 = (2.0 * locals.var_sp_s_temp__blk1448);
            let assign49400_e63421: f64 = (assign49400_e63419 - locals.var_gf2__blk1325);
            (locals.var_sp_s_c__blk1455, locals.var_sp_s_c__blk1455_dn4, locals.var_sp_s_c__blk1455_dn6, locals.var_sp_s_c__blk1455_dn7, locals.var_sp_s_c__blk1455_dn8, locals.var_sp_s_c__blk1455_dn9, ) = (assign49400_e63421, ((2.0 * locals.var_sp_s_temp__blk1448_dn4) - locals.var_gf2__blk1325_dn4), ((2.0 * locals.var_sp_s_temp__blk1448_dn6) - locals.var_gf2__blk1325_dn6), ((2.0 * locals.var_sp_s_temp__blk1448_dn7) - locals.var_gf2__blk1325_dn7), ((2.0 * locals.var_sp_s_temp__blk1448_dn8) - locals.var_gf2__blk1325_dn8), ((2.0 * locals.var_sp_s_temp__blk1448_dn9) - locals.var_gf2__blk1325_dn9), );
            locals.var_sp_s_c__blk1455_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign49410_e63433: f64 = (-locals.var_sp_s_eta__blk1453);
            let assign49410_e63436: f64 = (locals.var_sp_s_a__blk1454 * locals.var_inv_gf2__blk1341);
            let assign49410_e63437: f64 = (assign49410_e63436).ln();
            let assign49410_e63438: f64 = (assign49410_e63433 + assign49410_e63437);
            (locals.var_sp_s_tau__blk1456, locals.var_sp_s_tau__blk1456_dn4, locals.var_sp_s_tau__blk1456_dn6, locals.var_sp_s_tau__blk1456_dn7, locals.var_sp_s_tau__blk1456_dn8, locals.var_sp_s_tau__blk1456_dn9, ) = (assign49410_e63438, ((-locals.var_sp_s_eta__blk1453_dn4) + (((locals.var_sp_s_a__blk1454_dn4 * locals.var_inv_gf2__blk1341) + (locals.var_sp_s_a__blk1454 * locals.var_inv_gf2__blk1341_dn4)) / assign49410_e63436)), ((-locals.var_sp_s_eta__blk1453_dn6) + (((locals.var_sp_s_a__blk1454_dn6 * locals.var_inv_gf2__blk1341) + (locals.var_sp_s_a__blk1454 * locals.var_inv_gf2__blk1341_dn6)) / assign49410_e63436)), ((-locals.var_sp_s_eta__blk1453_dn7) + (((locals.var_sp_s_a__blk1454_dn7 * locals.var_inv_gf2__blk1341) + (locals.var_sp_s_a__blk1454 * locals.var_inv_gf2__blk1341_dn7)) / assign49410_e63436)), ((-locals.var_sp_s_eta__blk1453_dn8) + (((locals.var_sp_s_a__blk1454_dn8 * locals.var_inv_gf2__blk1341) + (locals.var_sp_s_a__blk1454 * locals.var_inv_gf2__blk1341_dn8)) / assign49410_e63436)), ((-locals.var_sp_s_eta__blk1453_dn9) + (((locals.var_sp_s_a__blk1454_dn9 * locals.var_inv_gf2__blk1341) + (locals.var_sp_s_a__blk1454 * locals.var_inv_gf2__blk1341_dn9)) / assign49410_e63436)), );
            locals.var_sp_s_tau__blk1456_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign49420_e63451: f64 = (locals.var_sp_s_a__blk1454 + locals.var_sp_s_c__blk1455);
            (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9, ) = (assign49420_e63451, (locals.var_sp_s_a__blk1454_dn4 + locals.var_sp_s_c__blk1455_dn4), (locals.var_sp_s_a__blk1454_dn6 + locals.var_sp_s_c__blk1455_dn6), (locals.var_sp_s_a__blk1454_dn7 + locals.var_sp_s_c__blk1455_dn7), (locals.var_sp_s_a__blk1454_dn8 + locals.var_sp_s_c__blk1455_dn8), (locals.var_sp_s_a__blk1454_dn9 + locals.var_sp_s_c__blk1455_dn9), );
            locals.var_nu_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign49430_e63464: f64 = (locals.var_nu * locals.var_nu);
            let assign49430_e63469: f64 = (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455);
            let assign49430_e63470: f64 = (0.5 * assign49430_e63469);
            let assign49430_e63472: f64 = (assign49430_e63470 - locals.var_sp_s_a__blk1454);
            let assign49430_e63473: f64 = (locals.var_sp_s_tau__blk1456 * assign49430_e63472);
            let assign49430_e63474: f64 = (assign49430_e63464 + assign49430_e63473);
            (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9, ) = (assign49430_e63474, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_sp_s_tau__blk1456_dn4 * assign49430_e63472) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn4 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn4))) - locals.var_sp_s_a__blk1454_dn4)))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau__blk1456_dn6 * assign49430_e63472) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn6 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn6))) - locals.var_sp_s_a__blk1454_dn6)))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau__blk1456_dn7 * assign49430_e63472) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn7 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn7))) - locals.var_sp_s_a__blk1454_dn7)))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau__blk1456_dn8 * assign49430_e63472) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn8 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn8))) - locals.var_sp_s_a__blk1454_dn8)))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_sp_s_tau__blk1456_dn9 * assign49430_e63472) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn9 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn9))) - locals.var_sp_s_a__blk1454_dn9)))), );
            locals.var_mutau_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign49440_e63488: f64 = (locals.var_sp_s_a__blk1454 * locals.var_nu);
            let assign49440_e63490: f64 = (assign49440_e63488 * locals.var_sp_s_tau__blk1456);
            let assign49440_e63494: f64 = (locals.var_nu / locals.var_mutau);
            let assign49440_e63496: f64 = (assign49440_e63494 * locals.var_sp_s_tau__blk1456);
            let assign49440_e63498: f64 = (assign49440_e63496 * locals.var_sp_s_tau__blk1456);
            let assign49440_e63500: f64 = (assign49440_e63498 * locals.var_sp_s_c__blk1455);
            let assign49440_e63503: f64 = (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455);
            let assign49440_e63505: f64 = (assign49440_e63503 * 0.3333333333333333);
            let assign49440_e63507: f64 = (assign49440_e63505 - locals.var_sp_s_a__blk1454);
            let assign49440_e63508: f64 = (assign49440_e63500 * assign49440_e63507);
            let assign49440_e63509: f64 = (locals.var_mutau + assign49440_e63508);
            let assign49440_e63510: f64 = (assign49440_e63490 / assign49440_e63509);
            let assign49440_e63511: f64 = (locals.var_sp_s_eta__blk1453 + assign49440_e63510);
            (locals.var_sp_s_y0__blk1457, locals.var_sp_s_y0__blk1457_dn4, locals.var_sp_s_y0__blk1457_dn6, locals.var_sp_s_y0__blk1457_dn7, locals.var_sp_s_y0__blk1457_dn8, locals.var_sp_s_y0__blk1457_dn9, ) = (assign49440_e63511, (locals.var_sp_s_eta__blk1453_dn4 + (((((((locals.var_sp_s_a__blk1454_dn4 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn4)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63488 * locals.var_sp_s_tau__blk1456_dn4)) * assign49440_e63509) - (assign49440_e63490 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63494 * locals.var_sp_s_tau__blk1456_dn4)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63496 * locals.var_sp_s_tau__blk1456_dn4)) * locals.var_sp_s_c__blk1455) + (assign49440_e63498 * locals.var_sp_s_c__blk1455_dn4)) * assign49440_e63507) + (assign49440_e63500 * ((((locals.var_sp_s_c__blk1455_dn4 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn4)) * 0.3333333333333333) - locals.var_sp_s_a__blk1454_dn4)))))) / (assign49440_e63509 * assign49440_e63509))), (locals.var_sp_s_eta__blk1453_dn6 + (((((((locals.var_sp_s_a__blk1454_dn6 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn6)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63488 * locals.var_sp_s_tau__blk1456_dn6)) * assign49440_e63509) - (assign49440_e63490 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63494 * locals.var_sp_s_tau__blk1456_dn6)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63496 * locals.var_sp_s_tau__blk1456_dn6)) * locals.var_sp_s_c__blk1455) + (assign49440_e63498 * locals.var_sp_s_c__blk1455_dn6)) * assign49440_e63507) + (assign49440_e63500 * ((((locals.var_sp_s_c__blk1455_dn6 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn6)) * 0.3333333333333333) - locals.var_sp_s_a__blk1454_dn6)))))) / (assign49440_e63509 * assign49440_e63509))), (locals.var_sp_s_eta__blk1453_dn7 + (((((((locals.var_sp_s_a__blk1454_dn7 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn7)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63488 * locals.var_sp_s_tau__blk1456_dn7)) * assign49440_e63509) - (assign49440_e63490 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63494 * locals.var_sp_s_tau__blk1456_dn7)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63496 * locals.var_sp_s_tau__blk1456_dn7)) * locals.var_sp_s_c__blk1455) + (assign49440_e63498 * locals.var_sp_s_c__blk1455_dn7)) * assign49440_e63507) + (assign49440_e63500 * ((((locals.var_sp_s_c__blk1455_dn7 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn7)) * 0.3333333333333333) - locals.var_sp_s_a__blk1454_dn7)))))) / (assign49440_e63509 * assign49440_e63509))), (locals.var_sp_s_eta__blk1453_dn8 + (((((((locals.var_sp_s_a__blk1454_dn8 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn8)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63488 * locals.var_sp_s_tau__blk1456_dn8)) * assign49440_e63509) - (assign49440_e63490 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63494 * locals.var_sp_s_tau__blk1456_dn8)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63496 * locals.var_sp_s_tau__blk1456_dn8)) * locals.var_sp_s_c__blk1455) + (assign49440_e63498 * locals.var_sp_s_c__blk1455_dn8)) * assign49440_e63507) + (assign49440_e63500 * ((((locals.var_sp_s_c__blk1455_dn8 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn8)) * 0.3333333333333333) - locals.var_sp_s_a__blk1454_dn8)))))) / (assign49440_e63509 * assign49440_e63509))), (locals.var_sp_s_eta__blk1453_dn9 + (((((((locals.var_sp_s_a__blk1454_dn9 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn9)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63488 * locals.var_sp_s_tau__blk1456_dn9)) * assign49440_e63509) - (assign49440_e63490 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63494 * locals.var_sp_s_tau__blk1456_dn9)) * locals.var_sp_s_tau__blk1456) + (assign49440_e63496 * locals.var_sp_s_tau__blk1456_dn9)) * locals.var_sp_s_c__blk1455) + (assign49440_e63498 * locals.var_sp_s_c__blk1455_dn9)) * assign49440_e63507) + (assign49440_e63500 * ((((locals.var_sp_s_c__blk1455_dn9 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn9)) * 0.3333333333333333) - locals.var_sp_s_a__blk1454_dn9)))))) / (assign49440_e63509 * assign49440_e63509))), );
            locals.var_sp_s_y0__blk1457_rv = 0.0;
        }

        let assign49450_e63516: f64 = if locals.var_sp_s_y0__blk1457 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1487 = assign49450_e63516;
        locals.var_guard1487_rv = 0.0;

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) && (locals.var_guard1487 != 0.0)) {
            let assign49460_e63528: f64 = (locals.var_sp_s_y0__blk1457).exp();
            (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9, ) = (assign49460_e63528, (assign49460_e63528 * locals.var_sp_s_y0__blk1457_dn4), (assign49460_e63528 * locals.var_sp_s_y0__blk1457_dn6), (assign49460_e63528 * locals.var_sp_s_y0__blk1457_dn7), (assign49460_e63528 * locals.var_sp_s_y0__blk1457_dn8), (assign49460_e63528 * locals.var_sp_s_y0__blk1457_dn9), );
            locals.var_sp_s_delta0__blk1458_rv = 0.0;
        }

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) && (locals.var_guard1487 == 0.0)) {
            let assign49470_e63546: f64 = (locals.var_sp_s_y0__blk1457 - 230.25850929940458);
            let assign49470_e63551: f64 = (locals.var_sp_s_y0__blk1457 - 230.25850929940458);
            let assign49470_e63555: f64 = (locals.var_sp_s_y0__blk1457 - 230.25850929940458);
            let assign49470_e63557: f64 = (assign49470_e63555 * 0.3333333333333333);
            let assign49470_e63558: f64 = (1.0 + assign49470_e63557);
            let assign49470_e63559: f64 = (assign49470_e63551 * assign49470_e63558);
            let assign49470_e63560: f64 = (0.5 * assign49470_e63559);
            let assign49470_e63561: f64 = (1.0 + assign49470_e63560);
            let assign49470_e63562: f64 = (assign49470_e63546 * assign49470_e63561);
            let assign49470_e63563: f64 = (1.0 + assign49470_e63562);
            let assign49470_e63564: f64 = (1e100 * assign49470_e63563);
            (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9, ) = (assign49470_e63564, (1e100 * ((locals.var_sp_s_y0__blk1457_dn4 * assign49470_e63561) + (assign49470_e63546 * (0.5 * ((locals.var_sp_s_y0__blk1457_dn4 * assign49470_e63558) + (assign49470_e63551 * (locals.var_sp_s_y0__blk1457_dn4 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0__blk1457_dn6 * assign49470_e63561) + (assign49470_e63546 * (0.5 * ((locals.var_sp_s_y0__blk1457_dn6 * assign49470_e63558) + (assign49470_e63551 * (locals.var_sp_s_y0__blk1457_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0__blk1457_dn7 * assign49470_e63561) + (assign49470_e63546 * (0.5 * ((locals.var_sp_s_y0__blk1457_dn7 * assign49470_e63558) + (assign49470_e63551 * (locals.var_sp_s_y0__blk1457_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0__blk1457_dn8 * assign49470_e63561) + (assign49470_e63546 * (0.5 * ((locals.var_sp_s_y0__blk1457_dn8 * assign49470_e63558) + (assign49470_e63551 * (locals.var_sp_s_y0__blk1457_dn8 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0__blk1457_dn9 * assign49470_e63561) + (assign49470_e63546 * (0.5 * ((locals.var_sp_s_y0__blk1457_dn9 * assign49470_e63558) + (assign49470_e63551 * (locals.var_sp_s_y0__blk1457_dn9 * 0.3333333333333333))))))), );
            locals.var_sp_s_delta0__blk1458_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign49480_e63577: f64 = (1.0 / locals.var_sp_s_delta0__blk1458);
            (locals.var_sp_s_delta1__blk1459, locals.var_sp_s_delta1__blk1459_dn4, locals.var_sp_s_delta1__blk1459_dn6, locals.var_sp_s_delta1__blk1459_dn7, locals.var_sp_s_delta1__blk1459_dn8, locals.var_sp_s_delta1__blk1459_dn9, ) = (assign49480_e63577, (-(locals.var_sp_s_delta0__blk1458_dn4 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn6 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn7 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn8 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn9 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), );
            locals.var_sp_s_delta1__blk1459_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign49490_e63592: f64 = (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457);
            let assign49490_e63593: f64 = (2.0 + assign49490_e63592);
            let assign49490_e63594: f64 = (1.0 / assign49490_e63593);
            (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9, ) = (assign49490_e63594, (-(((locals.var_sp_s_y0__blk1457_dn4 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn4)) / (assign49490_e63593 * assign49490_e63593))), (-(((locals.var_sp_s_y0__blk1457_dn6 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn6)) / (assign49490_e63593 * assign49490_e63593))), (-(((locals.var_sp_s_y0__blk1457_dn7 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn7)) / (assign49490_e63593 * assign49490_e63593))), (-(((locals.var_sp_s_y0__blk1457_dn8 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn8)) / (assign49490_e63593 * assign49490_e63593))), (-(((locals.var_sp_s_y0__blk1457_dn9 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn9)) / (assign49490_e63593 * assign49490_e63593))), );
            locals.var_sp_s_temp__blk1448_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign49500_e63607: f64 = (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457);
            let assign49500_e63609: f64 = (assign49500_e63607 * locals.var_sp_s_temp__blk1448);
            (locals.var_sp_s_xi0__blk1460, locals.var_sp_s_xi0__blk1460_dn4, locals.var_sp_s_xi0__blk1460_dn6, locals.var_sp_s_xi0__blk1460_dn7, locals.var_sp_s_xi0__blk1460_dn8, locals.var_sp_s_xi0__blk1460_dn9, ) = (assign49500_e63609, ((((locals.var_sp_s_y0__blk1457_dn4 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49500_e63607 * locals.var_sp_s_temp__blk1448_dn4)), ((((locals.var_sp_s_y0__blk1457_dn6 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49500_e63607 * locals.var_sp_s_temp__blk1448_dn6)), ((((locals.var_sp_s_y0__blk1457_dn7 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49500_e63607 * locals.var_sp_s_temp__blk1448_dn7)), ((((locals.var_sp_s_y0__blk1457_dn8 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49500_e63607 * locals.var_sp_s_temp__blk1448_dn8)), ((((locals.var_sp_s_y0__blk1457_dn9 * locals.var_sp_s_y0__blk1457) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_y0__blk1457_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49500_e63607 * locals.var_sp_s_temp__blk1448_dn9)), );
            locals.var_sp_s_xi0__blk1460_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign49510_e63623: f64 = (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_temp__blk1448);
            let assign49510_e63625: f64 = (assign49510_e63623 * locals.var_sp_s_temp__blk1448);
            let assign49510_e63626: f64 = (4.0 * assign49510_e63625);
            (locals.var_sp_s_xi1__blk1461, locals.var_sp_s_xi1__blk1461_dn4, locals.var_sp_s_xi1__blk1461_dn6, locals.var_sp_s_xi1__blk1461_dn7, locals.var_sp_s_xi1__blk1461_dn8, locals.var_sp_s_xi1__blk1461_dn9, ) = (assign49510_e63626, (4.0 * ((((locals.var_sp_s_y0__blk1457_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_temp__blk1448_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49510_e63623 * locals.var_sp_s_temp__blk1448_dn4))), (4.0 * ((((locals.var_sp_s_y0__blk1457_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_temp__blk1448_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49510_e63623 * locals.var_sp_s_temp__blk1448_dn6))), (4.0 * ((((locals.var_sp_s_y0__blk1457_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_temp__blk1448_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49510_e63623 * locals.var_sp_s_temp__blk1448_dn7))), (4.0 * ((((locals.var_sp_s_y0__blk1457_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_temp__blk1448_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49510_e63623 * locals.var_sp_s_temp__blk1448_dn8))), (4.0 * ((((locals.var_sp_s_y0__blk1457_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_y0__blk1457 * locals.var_sp_s_temp__blk1448_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49510_e63623 * locals.var_sp_s_temp__blk1448_dn9))), );
            locals.var_sp_s_xi1__blk1461_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign49520_e63639: f64 = (8.0 * locals.var_sp_s_temp__blk1448);
            let assign49520_e63642: f64 = (12.0 * locals.var_sp_s_xi0__blk1460);
            let assign49520_e63643: f64 = (assign49520_e63639 - assign49520_e63642);
            let assign49520_e63645: f64 = (assign49520_e63643 * locals.var_sp_s_temp__blk1448);
            let assign49520_e63647: f64 = (assign49520_e63645 * locals.var_sp_s_temp__blk1448);
            (locals.var_sp_s_xi2__blk1462, locals.var_sp_s_xi2__blk1462_dn4, locals.var_sp_s_xi2__blk1462_dn6, locals.var_sp_s_xi2__blk1462_dn7, locals.var_sp_s_xi2__blk1462_dn8, locals.var_sp_s_xi2__blk1462_dn9, ) = (assign49520_e63647, ((((((8.0 * locals.var_sp_s_temp__blk1448_dn4) - (12.0 * locals.var_sp_s_xi0__blk1460_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63643 * locals.var_sp_s_temp__blk1448_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63645 * locals.var_sp_s_temp__blk1448_dn4)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn6) - (12.0 * locals.var_sp_s_xi0__blk1460_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63643 * locals.var_sp_s_temp__blk1448_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63645 * locals.var_sp_s_temp__blk1448_dn6)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn7) - (12.0 * locals.var_sp_s_xi0__blk1460_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63643 * locals.var_sp_s_temp__blk1448_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63645 * locals.var_sp_s_temp__blk1448_dn7)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn8) - (12.0 * locals.var_sp_s_xi0__blk1460_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63643 * locals.var_sp_s_temp__blk1448_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63645 * locals.var_sp_s_temp__blk1448_dn8)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn9) - (12.0 * locals.var_sp_s_xi0__blk1460_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63643 * locals.var_sp_s_temp__blk1448_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49520_e63645 * locals.var_sp_s_temp__blk1448_dn9)), );
            locals.var_sp_s_xi2__blk1462_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign49530_e63660: f64 = (locals.var_sp_s_yg__blk1451 - locals.var_sp_s_y0__blk1457);
            (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9, ) = (assign49530_e63660, (locals.var_sp_s_yg__blk1451_dn4 - locals.var_sp_s_y0__blk1457_dn4), (locals.var_sp_s_yg__blk1451_dn6 - locals.var_sp_s_y0__blk1457_dn6), (locals.var_sp_s_yg__blk1451_dn7 - locals.var_sp_s_y0__blk1457_dn7), (locals.var_sp_s_yg__blk1451_dn8 - locals.var_sp_s_y0__blk1457_dn8), (locals.var_sp_s_yg__blk1451_dn9 - locals.var_sp_s_y0__blk1457_dn9), );
            locals.var_sp_s_temp__blk1448_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign49540_e63673: f64 = (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta1__blk1459);
            (locals.var_sp_s_temp1__blk1449, locals.var_sp_s_temp1__blk1449_dn4, locals.var_sp_s_temp1__blk1449_dn6, locals.var_sp_s_temp1__blk1449_dn7, locals.var_sp_s_temp1__blk1449_dn8, locals.var_sp_s_temp1__blk1449_dn9, ) = (assign49540_e63673, ((locals.var_delta_ns__blk1364_dn4 * locals.var_sp_s_delta1__blk1459) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta1__blk1459_dn4)), ((locals.var_delta_ns__blk1364_dn6 * locals.var_sp_s_delta1__blk1459) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta1__blk1459_dn6)), ((locals.var_delta_ns__blk1364_dn7 * locals.var_sp_s_delta1__blk1459) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta1__blk1459_dn7)), ((locals.var_delta_ns__blk1364_dn8 * locals.var_sp_s_delta1__blk1459) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta1__blk1459_dn8)), ((locals.var_delta_ns__blk1364_dn9 * locals.var_sp_s_delta1__blk1459) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta1__blk1459_dn9)), );
            locals.var_sp_s_temp1__blk1449_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign49550_e63686: f64 = (2.0 * locals.var_sp_s_temp__blk1448);
            let assign49550_e63690: f64 = (locals.var_sp_s_delta0__blk1458 - 1.0);
            let assign49550_e63692: f64 = (assign49550_e63690 - locals.var_sp_s_temp1__blk1449);
            let assign49550_e63696: f64 = (1.0 - locals.var_sp_s_xi1__blk1461);
            let assign49550_e63697: f64 = (locals.var_delta_ns__blk1364 * assign49550_e63696);
            let assign49550_e63698: f64 = (assign49550_e63692 + assign49550_e63697);
            let assign49550_e63699: f64 = (locals.var_gf2__blk1325 * assign49550_e63698);
            let assign49550_e63700: f64 = (assign49550_e63686 + assign49550_e63699);
            (locals.var_sp_s_pc__blk1463, locals.var_sp_s_pc__blk1463_dn4, locals.var_sp_s_pc__blk1463_dn6, locals.var_sp_s_pc__blk1463_dn7, locals.var_sp_s_pc__blk1463_dn8, locals.var_sp_s_pc__blk1463_dn9, ) = (assign49550_e63700, ((2.0 * locals.var_sp_s_temp__blk1448_dn4) + ((locals.var_gf2__blk1325_dn4 * assign49550_e63698) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn4 - locals.var_sp_s_temp1__blk1449_dn4) + ((locals.var_delta_ns__blk1364_dn4 * assign49550_e63696) + (locals.var_delta_ns__blk1364 * (-locals.var_sp_s_xi1__blk1461_dn4))))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn6) + ((locals.var_gf2__blk1325_dn6 * assign49550_e63698) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn6 - locals.var_sp_s_temp1__blk1449_dn6) + ((locals.var_delta_ns__blk1364_dn6 * assign49550_e63696) + (locals.var_delta_ns__blk1364 * (-locals.var_sp_s_xi1__blk1461_dn6))))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn7) + ((locals.var_gf2__blk1325_dn7 * assign49550_e63698) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn7 - locals.var_sp_s_temp1__blk1449_dn7) + ((locals.var_delta_ns__blk1364_dn7 * assign49550_e63696) + (locals.var_delta_ns__blk1364 * (-locals.var_sp_s_xi1__blk1461_dn7))))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn8) + ((locals.var_gf2__blk1325_dn8 * assign49550_e63698) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn8 - locals.var_sp_s_temp1__blk1449_dn8) + ((locals.var_delta_ns__blk1364_dn8 * assign49550_e63696) + (locals.var_delta_ns__blk1364 * (-locals.var_sp_s_xi1__blk1461_dn8))))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn9) + ((locals.var_gf2__blk1325_dn9 * assign49550_e63698) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn9 - locals.var_sp_s_temp1__blk1449_dn9) + ((locals.var_delta_ns__blk1364_dn9 * assign49550_e63696) + (locals.var_delta_ns__blk1364 * (-locals.var_sp_s_xi1__blk1461_dn9))))))), );
            locals.var_sp_s_pc__blk1463_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign49560_e63713: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
            let assign49560_e63717: f64 = (locals.var_sp_s_delta0__blk1458 - locals.var_sp_s_y0__blk1457);
            let assign49560_e63719: f64 = (assign49560_e63717 - 1.0);
            let assign49560_e63721: f64 = (assign49560_e63719 + locals.var_sp_s_temp1__blk1449);
            let assign49560_e63725: f64 = (locals.var_sp_s_y0__blk1457 - 1.0);
            let assign49560_e63727: f64 = (assign49560_e63725 - locals.var_sp_s_xi0__blk1460);
            let assign49560_e63728: f64 = (locals.var_delta_ns__blk1364 * assign49560_e63727);
            let assign49560_e63729: f64 = (assign49560_e63721 + assign49560_e63728);
            let assign49560_e63730: f64 = (locals.var_gf2__blk1325 * assign49560_e63729);
            let assign49560_e63731: f64 = (assign49560_e63713 - assign49560_e63730);
            (locals.var_sp_s_qc__blk1464, locals.var_sp_s_qc__blk1464_dn4, locals.var_sp_s_qc__blk1464_dn6, locals.var_sp_s_qc__blk1464_dn7, locals.var_sp_s_qc__blk1464_dn8, locals.var_sp_s_qc__blk1464_dn9, ) = (assign49560_e63731, (((locals.var_sp_s_temp__blk1448_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn4)) - ((locals.var_gf2__blk1325_dn4 * assign49560_e63729) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta0__blk1458_dn4 - locals.var_sp_s_y0__blk1457_dn4) + locals.var_sp_s_temp1__blk1449_dn4) + ((locals.var_delta_ns__blk1364_dn4 * assign49560_e63727) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_y0__blk1457_dn4 - locals.var_sp_s_xi0__blk1460_dn4))))))), (((locals.var_sp_s_temp__blk1448_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn6)) - ((locals.var_gf2__blk1325_dn6 * assign49560_e63729) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta0__blk1458_dn6 - locals.var_sp_s_y0__blk1457_dn6) + locals.var_sp_s_temp1__blk1449_dn6) + ((locals.var_delta_ns__blk1364_dn6 * assign49560_e63727) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_y0__blk1457_dn6 - locals.var_sp_s_xi0__blk1460_dn6))))))), (((locals.var_sp_s_temp__blk1448_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn7)) - ((locals.var_gf2__blk1325_dn7 * assign49560_e63729) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta0__blk1458_dn7 - locals.var_sp_s_y0__blk1457_dn7) + locals.var_sp_s_temp1__blk1449_dn7) + ((locals.var_delta_ns__blk1364_dn7 * assign49560_e63727) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_y0__blk1457_dn7 - locals.var_sp_s_xi0__blk1460_dn7))))))), (((locals.var_sp_s_temp__blk1448_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn8)) - ((locals.var_gf2__blk1325_dn8 * assign49560_e63729) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta0__blk1458_dn8 - locals.var_sp_s_y0__blk1457_dn8) + locals.var_sp_s_temp1__blk1449_dn8) + ((locals.var_delta_ns__blk1364_dn8 * assign49560_e63727) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_y0__blk1457_dn8 - locals.var_sp_s_xi0__blk1460_dn8))))))), (((locals.var_sp_s_temp__blk1448_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn9)) - ((locals.var_gf2__blk1325_dn9 * assign49560_e63729) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta0__blk1458_dn9 - locals.var_sp_s_y0__blk1457_dn9) + locals.var_sp_s_temp1__blk1449_dn9) + ((locals.var_delta_ns__blk1364_dn9 * assign49560_e63727) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_y0__blk1457_dn9 - locals.var_sp_s_xi0__blk1460_dn9))))))), );
            locals.var_sp_s_qc__blk1464_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign49570_e63746: f64 = (locals.var_sp_s_delta0__blk1458 + locals.var_sp_s_temp1__blk1449);
            let assign49570_e63749: f64 = (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462);
            let assign49570_e63750: f64 = (assign49570_e63746 - assign49570_e63749);
            let assign49570_e63751: f64 = (locals.var_gf2__blk1325 * assign49570_e63750);
            let assign49570_e63752: f64 = (2.0 - assign49570_e63751);
            (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9, ) = (assign49570_e63752, (-((locals.var_gf2__blk1325_dn4 * assign49570_e63750) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn4 + locals.var_sp_s_temp1__blk1449_dn4) - ((locals.var_delta_ns__blk1364_dn4 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn4)))))), (-((locals.var_gf2__blk1325_dn6 * assign49570_e63750) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn6 + locals.var_sp_s_temp1__blk1449_dn6) - ((locals.var_delta_ns__blk1364_dn6 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn6)))))), (-((locals.var_gf2__blk1325_dn7 * assign49570_e63750) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn7 + locals.var_sp_s_temp1__blk1449_dn7) - ((locals.var_delta_ns__blk1364_dn7 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn7)))))), (-((locals.var_gf2__blk1325_dn8 * assign49570_e63750) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn8 + locals.var_sp_s_temp1__blk1449_dn8) - ((locals.var_delta_ns__blk1364_dn8 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn8)))))), (-((locals.var_gf2__blk1325_dn9 * assign49570_e63750) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta0__blk1458_dn9 + locals.var_sp_s_temp1__blk1449_dn9) - ((locals.var_delta_ns__blk1364_dn9 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn9)))))), );
            locals.var_sp_s_temp__blk1448_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign49580_e63765: f64 = (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463);
            let assign49580_e63769: f64 = (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448);
            let assign49580_e63770: f64 = (2.0 * assign49580_e63769);
            let assign49580_e63771: f64 = (assign49580_e63765 - assign49580_e63770);
            (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9, ) = (assign49580_e63771, (((locals.var_sp_s_pc__blk1463_dn4 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn4)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn4)))), (((locals.var_sp_s_pc__blk1463_dn6 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn6)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn6)))), (((locals.var_sp_s_pc__blk1463_dn7 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn7)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn7)))), (((locals.var_sp_s_pc__blk1463_dn8 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn8)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn8)))), (((locals.var_sp_s_pc__blk1463_dn9 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn9)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn9)))), );
            locals.var_sp_s_temp__blk1448_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign49590_e63783: f64 = (-locals.var_sp_s_y0__blk1457);
            let assign49590_e63788: f64 = (locals.var_sp_s_temp__blk1448).sqrt();
            let assign49590_e63789: f64 = (locals.var_sp_s_pc__blk1463 + assign49590_e63788);
            let assign49590_e63790: f64 = (locals.var_sp_s_qc__blk1464 / assign49590_e63789);
            let assign49590_e63791: f64 = (2.0 * assign49590_e63790);
            let assign49590_e63792: f64 = (assign49590_e63783 - assign49590_e63791);
            (locals.var_x_s__blk1363, locals.var_x_s__blk1363_dn4, locals.var_x_s__blk1363_dn6, locals.var_x_s__blk1363_dn7, locals.var_x_s__blk1363_dn8, locals.var_x_s__blk1363_dn9, ) = (assign49590_e63792, ((-locals.var_sp_s_y0__blk1457_dn4) - (2.0 * (((locals.var_sp_s_qc__blk1464_dn4 * assign49590_e63789) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn4 + (locals.var_sp_s_temp__blk1448_dn4 / (2.0 * assign49590_e63788))))) / (assign49590_e63789 * assign49590_e63789)))), ((-locals.var_sp_s_y0__blk1457_dn6) - (2.0 * (((locals.var_sp_s_qc__blk1464_dn6 * assign49590_e63789) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn6 + (locals.var_sp_s_temp__blk1448_dn6 / (2.0 * assign49590_e63788))))) / (assign49590_e63789 * assign49590_e63789)))), ((-locals.var_sp_s_y0__blk1457_dn7) - (2.0 * (((locals.var_sp_s_qc__blk1464_dn7 * assign49590_e63789) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn7 + (locals.var_sp_s_temp__blk1448_dn7 / (2.0 * assign49590_e63788))))) / (assign49590_e63789 * assign49590_e63789)))), ((-locals.var_sp_s_y0__blk1457_dn8) - (2.0 * (((locals.var_sp_s_qc__blk1464_dn8 * assign49590_e63789) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn8 + (locals.var_sp_s_temp__blk1448_dn8 / (2.0 * assign49590_e63788))))) / (assign49590_e63789 * assign49590_e63789)))), ((-locals.var_sp_s_y0__blk1457_dn9) - (2.0 * (((locals.var_sp_s_qc__blk1464_dn9 * assign49590_e63789) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn9 + (locals.var_sp_s_temp__blk1448_dn9 / (2.0 * assign49590_e63788))))) / (assign49590_e63789 * assign49590_e63789)))), );
            locals.var_x_s__blk1363_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49600_e63808: f64 = (locals.var_gf__blk1324 * 0.7324648775608221);
            let assign49600_e63809: f64 = (1.25 + assign49600_e63808);
            let assign49600_e63810: f64 = (1.0 / assign49600_e63809);
            (locals.var_sp_xg1__blk1465, locals.var_sp_xg1__blk1465_dn4, locals.var_sp_xg1__blk1465_dn6, locals.var_sp_xg1__blk1465_dn7, locals.var_sp_xg1__blk1465_dn8, locals.var_sp_xg1__blk1465_dn9, ) = (assign49600_e63810, (-((locals.var_gf__blk1324_dn4 * 0.7324648775608221) / (assign49600_e63809 * assign49600_e63809))), (-((locals.var_gf__blk1324_dn6 * 0.7324648775608221) / (assign49600_e63809 * assign49600_e63809))), (-((locals.var_gf__blk1324_dn7 * 0.7324648775608221) / (assign49600_e63809 * assign49600_e63809))), (-((locals.var_gf__blk1324_dn8 * 0.7324648775608221) / (assign49600_e63809 * assign49600_e63809))), (-((locals.var_gf__blk1324_dn9 * 0.7324648775608221) / (assign49600_e63809 * assign49600_e63809))), );
            locals.var_sp_xg1__blk1465_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49610_e63824: f64 = (locals.var_xi__blk1360 * 1.25);
            let assign49610_e63826: f64 = (assign49610_e63824 * locals.var_sp_xg1__blk1465);
            let assign49610_e63828: f64 = (assign49610_e63826 - 1.0);
            let assign49610_e63830: f64 = (assign49610_e63828 * locals.var_sp_xg1__blk1465);
            (locals.var_sp_s_a_fac__blk1466, locals.var_sp_s_a_fac__blk1466_dn4, locals.var_sp_s_a_fac__blk1466_dn6, locals.var_sp_s_a_fac__blk1466_dn7, locals.var_sp_s_a_fac__blk1466_dn8, locals.var_sp_s_a_fac__blk1466_dn9, ) = (assign49610_e63830, (((((locals.var_xi__blk1360_dn4 * 1.25) * locals.var_sp_xg1__blk1465) + (assign49610_e63824 * locals.var_sp_xg1__blk1465_dn4)) * locals.var_sp_xg1__blk1465) + (assign49610_e63828 * locals.var_sp_xg1__blk1465_dn4)), (((((locals.var_xi__blk1360_dn6 * 1.25) * locals.var_sp_xg1__blk1465) + (assign49610_e63824 * locals.var_sp_xg1__blk1465_dn6)) * locals.var_sp_xg1__blk1465) + (assign49610_e63828 * locals.var_sp_xg1__blk1465_dn6)), (((((locals.var_xi__blk1360_dn7 * 1.25) * locals.var_sp_xg1__blk1465) + (assign49610_e63824 * locals.var_sp_xg1__blk1465_dn7)) * locals.var_sp_xg1__blk1465) + (assign49610_e63828 * locals.var_sp_xg1__blk1465_dn7)), (((((locals.var_xi__blk1360_dn8 * 1.25) * locals.var_sp_xg1__blk1465) + (assign49610_e63824 * locals.var_sp_xg1__blk1465_dn8)) * locals.var_sp_xg1__blk1465) + (assign49610_e63828 * locals.var_sp_xg1__blk1465_dn8)), (((((locals.var_xi__blk1360_dn9 * 1.25) * locals.var_sp_xg1__blk1465) + (assign49610_e63824 * locals.var_sp_xg1__blk1465_dn9)) * locals.var_sp_xg1__blk1465) + (assign49610_e63828 * locals.var_sp_xg1__blk1465_dn9)), );
            locals.var_sp_s_a_fac__blk1466_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49620_e63844: f64 = (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362);
            let assign49620_e63848: f64 = (locals.var_sp_s_a_fac__blk1466 * locals.var_xg__blk1343);
            let assign49620_e63849: f64 = (1.0 + assign49620_e63848);
            let assign49620_e63850: f64 = (assign49620_e63844 * assign49620_e63849);
            (locals.var_sp_s_xbar__blk1467, locals.var_sp_s_xbar__blk1467_dn4, locals.var_sp_s_xbar__blk1467_dn6, locals.var_sp_s_xbar__blk1467_dn7, locals.var_sp_s_xbar__blk1467_dn8, locals.var_sp_s_xbar__blk1467_dn9, ) = (assign49620_e63850, ((((locals.var_xg__blk1343_dn4 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn4)) * assign49620_e63849) + (assign49620_e63844 * ((locals.var_sp_s_a_fac__blk1466_dn4 * locals.var_xg__blk1343) + (locals.var_sp_s_a_fac__blk1466 * locals.var_xg__blk1343_dn4)))), ((((locals.var_xg__blk1343_dn6 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn6)) * assign49620_e63849) + (assign49620_e63844 * ((locals.var_sp_s_a_fac__blk1466_dn6 * locals.var_xg__blk1343) + (locals.var_sp_s_a_fac__blk1466 * locals.var_xg__blk1343_dn6)))), ((((locals.var_xg__blk1343_dn7 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn7)) * assign49620_e63849) + (assign49620_e63844 * ((locals.var_sp_s_a_fac__blk1466_dn7 * locals.var_xg__blk1343) + (locals.var_sp_s_a_fac__blk1466 * locals.var_xg__blk1343_dn7)))), ((((locals.var_xg__blk1343_dn8 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn8)) * assign49620_e63849) + (assign49620_e63844 * ((locals.var_sp_s_a_fac__blk1466_dn8 * locals.var_xg__blk1343) + (locals.var_sp_s_a_fac__blk1466 * locals.var_xg__blk1343_dn8)))), ((((locals.var_xg__blk1343_dn9 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn9)) * assign49620_e63849) + (assign49620_e63844 * ((locals.var_sp_s_a_fac__blk1466_dn9 * locals.var_xg__blk1343) + (locals.var_sp_s_a_fac__blk1466 * locals.var_xg__blk1343_dn9)))), );
            locals.var_sp_s_xbar__blk1467_rv = 0.0;
        }

        let assign49630_e63854: f64 = (-locals.var_sp_s_xbar__blk1467);
        let assign49630_e63856: f64 = (-230.25850929940458);
        let assign49630_e63857: f64 = if assign49630_e63854 > assign49630_e63856 { 1.0 } else { 0.0 };
        locals.var_guard1488 = assign49630_e63857;
        locals.var_guard1488_rv = 0.0;

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1488 != 0.0)) {
            let assign49640_e63870: f64 = (-locals.var_sp_s_xbar__blk1467);
            let assign49640_e63871: f64 = (assign49640_e63870).exp();
            (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9, ) = (assign49640_e63871, (assign49640_e63871 * (-locals.var_sp_s_xbar__blk1467_dn4)), (assign49640_e63871 * (-locals.var_sp_s_xbar__blk1467_dn6)), (assign49640_e63871 * (-locals.var_sp_s_xbar__blk1467_dn7)), (assign49640_e63871 * (-locals.var_sp_s_xbar__blk1467_dn8)), (assign49640_e63871 * (-locals.var_sp_s_xbar__blk1467_dn9)), );
            locals.var_sp_s_temp__blk1448_rv = 0.0;
        }

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1488 == 0.0)) {
            let assign49650_e63889: f64 = (-230.25850929940458);
            let assign49650_e63891: f64 = (-locals.var_sp_s_xbar__blk1467);
            let assign49650_e63892: f64 = (assign49650_e63889 - assign49650_e63891);
            let assign49650_e63896: f64 = (-230.25850929940458);
            let assign49650_e63898: f64 = (-locals.var_sp_s_xbar__blk1467);
            let assign49650_e63899: f64 = (assign49650_e63896 - assign49650_e63898);
            let assign49650_e63902: f64 = (-230.25850929940458);
            let assign49650_e63904: f64 = (-locals.var_sp_s_xbar__blk1467);
            let assign49650_e63905: f64 = (assign49650_e63902 - assign49650_e63904);
            let assign49650_e63907: f64 = (assign49650_e63905 * 0.3333333333333333);
            let assign49650_e63908: f64 = (1.0 + assign49650_e63907);
            let assign49650_e63909: f64 = (assign49650_e63899 * assign49650_e63908);
            let assign49650_e63910: f64 = (0.5 * assign49650_e63909);
            let assign49650_e63911: f64 = (1.0 + assign49650_e63910);
            let assign49650_e63912: f64 = (assign49650_e63892 * assign49650_e63911);
            let assign49650_e63913: f64 = (1.0 + assign49650_e63912);
            let assign49650_e63914: f64 = (1e-100 / assign49650_e63913);
            (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9, ) = (assign49650_e63914, (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1467_dn4)) * assign49650_e63911) + (assign49650_e63892 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1467_dn4)) * assign49650_e63908) + (assign49650_e63899 * ((-(-locals.var_sp_s_xbar__blk1467_dn4)) * 0.3333333333333333))))))) / (assign49650_e63913 * assign49650_e63913))), (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1467_dn6)) * assign49650_e63911) + (assign49650_e63892 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1467_dn6)) * assign49650_e63908) + (assign49650_e63899 * ((-(-locals.var_sp_s_xbar__blk1467_dn6)) * 0.3333333333333333))))))) / (assign49650_e63913 * assign49650_e63913))), (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1467_dn7)) * assign49650_e63911) + (assign49650_e63892 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1467_dn7)) * assign49650_e63908) + (assign49650_e63899 * ((-(-locals.var_sp_s_xbar__blk1467_dn7)) * 0.3333333333333333))))))) / (assign49650_e63913 * assign49650_e63913))), (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1467_dn8)) * assign49650_e63911) + (assign49650_e63892 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1467_dn8)) * assign49650_e63908) + (assign49650_e63899 * ((-(-locals.var_sp_s_xbar__blk1467_dn8)) * 0.3333333333333333))))))) / (assign49650_e63913 * assign49650_e63913))), (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1467_dn9)) * assign49650_e63911) + (assign49650_e63892 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1467_dn9)) * assign49650_e63908) + (assign49650_e63899 * ((-(-locals.var_sp_s_xbar__blk1467_dn9)) * 0.3333333333333333))))))) / (assign49650_e63913 * assign49650_e63913))), );
            locals.var_sp_s_temp__blk1448_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49660_e63928: f64 = (1.0 - locals.var_sp_s_temp__blk1448);
            (locals.var_sp_s_w__blk1468, locals.var_sp_s_w__blk1468_dn4, locals.var_sp_s_w__blk1468_dn6, locals.var_sp_s_w__blk1468_dn7, locals.var_sp_s_w__blk1468_dn8, locals.var_sp_s_w__blk1468_dn9, ) = (assign49660_e63928, (-locals.var_sp_s_temp__blk1448_dn4), (-locals.var_sp_s_temp__blk1448_dn6), (-locals.var_sp_s_temp__blk1448_dn7), (-locals.var_sp_s_temp__blk1448_dn8), (-locals.var_sp_s_temp__blk1448_dn9), );
            locals.var_sp_s_w__blk1468_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49670_e63943: f64 = (locals.var_gf2__blk1325 * 0.5);
            let assign49670_e63944: f64 = (locals.var_xg__blk1343 + assign49670_e63943);
            let assign49670_e63949: f64 = (locals.var_gf2__blk1325 * 0.25);
            let assign49670_e63950: f64 = (locals.var_xg__blk1343 + assign49670_e63949);
            let assign49670_e63952: f64 = (assign49670_e63950 - locals.var_sp_s_w__blk1468);
            let assign49670_e63953: f64 = (assign49670_e63952).sqrt();
            let assign49670_e63954: f64 = (locals.var_gf__blk1324 * assign49670_e63953);
            let assign49670_e63955: f64 = (assign49670_e63944 - assign49670_e63954);
            (locals.var_sp_s_x1__blk1469, locals.var_sp_s_x1__blk1469_dn4, locals.var_sp_s_x1__blk1469_dn6, locals.var_sp_s_x1__blk1469_dn7, locals.var_sp_s_x1__blk1469_dn8, locals.var_sp_s_x1__blk1469_dn9, ) = (assign49670_e63955, ((locals.var_xg__blk1343_dn4 + (locals.var_gf2__blk1325_dn4 * 0.5)) - ((locals.var_gf__blk1324_dn4 * assign49670_e63953) + (locals.var_gf__blk1324 * (((locals.var_xg__blk1343_dn4 + (locals.var_gf2__blk1325_dn4 * 0.25)) - locals.var_sp_s_w__blk1468_dn4) / (2.0 * assign49670_e63953))))), ((locals.var_xg__blk1343_dn6 + (locals.var_gf2__blk1325_dn6 * 0.5)) - ((locals.var_gf__blk1324_dn6 * assign49670_e63953) + (locals.var_gf__blk1324 * (((locals.var_xg__blk1343_dn6 + (locals.var_gf2__blk1325_dn6 * 0.25)) - locals.var_sp_s_w__blk1468_dn6) / (2.0 * assign49670_e63953))))), ((locals.var_xg__blk1343_dn7 + (locals.var_gf2__blk1325_dn7 * 0.5)) - ((locals.var_gf__blk1324_dn7 * assign49670_e63953) + (locals.var_gf__blk1324 * (((locals.var_xg__blk1343_dn7 + (locals.var_gf2__blk1325_dn7 * 0.25)) - locals.var_sp_s_w__blk1468_dn7) / (2.0 * assign49670_e63953))))), ((locals.var_xg__blk1343_dn8 + (locals.var_gf2__blk1325_dn8 * 0.5)) - ((locals.var_gf__blk1324_dn8 * assign49670_e63953) + (locals.var_gf__blk1324 * (((locals.var_xg__blk1343_dn8 + (locals.var_gf2__blk1325_dn8 * 0.25)) - locals.var_sp_s_w__blk1468_dn8) / (2.0 * assign49670_e63953))))), ((locals.var_xg__blk1343_dn9 + (locals.var_gf2__blk1325_dn9 * 0.5)) - ((locals.var_gf__blk1324_dn9 * assign49670_e63953) + (locals.var_gf__blk1324 * (((locals.var_xg__blk1343_dn9 + (locals.var_gf2__blk1325_dn9 * 0.25)) - locals.var_sp_s_w__blk1468_dn9) / (2.0 * assign49670_e63953))))), );
            locals.var_sp_s_x1__blk1469_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49680_e63969: f64 = (locals.var_xn_s__blk1349 + 3.0);
            (locals.var_sp_s_bx__blk1470, locals.var_sp_s_bx__blk1470_dn4, locals.var_sp_s_bx__blk1470_dn6, locals.var_sp_s_bx__blk1470_dn7, locals.var_sp_s_bx__blk1470_dn8, locals.var_sp_s_bx__blk1470_dn9, ) = (assign49680_e63969, locals.var_xn_s__blk1349_dn4, locals.var_xn_s__blk1349_dn6, locals.var_xn_s__blk1349_dn7, locals.var_xn_s__blk1349_dn8, locals.var_xn_s__blk1349_dn9, );
            locals.var_sp_s_bx__blk1470_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49690_e63984: f64 = (locals.var_sp_s_x1__blk1469 + locals.var_sp_s_bx__blk1470);
            let assign49690_e63987: f64 = (locals.var_sp_s_x1__blk1469 - locals.var_sp_s_bx__blk1470);
            let assign49690_e63990: f64 = (locals.var_sp_s_x1__blk1469 - locals.var_sp_s_bx__blk1470);
            let assign49690_e63991: f64 = (assign49690_e63987 * assign49690_e63990);
            let assign49690_e63993: f64 = (assign49690_e63991 + 5.0);
            let assign49690_e63994: f64 = (assign49690_e63993).sqrt();
            let assign49690_e63995: f64 = (assign49690_e63984 - assign49690_e63994);
            let assign49690_e63996: f64 = (0.5 * assign49690_e63995);
            let assign49690_e64001: f64 = (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470);
            let assign49690_e64003: f64 = (assign49690_e64001 + 5.0);
            let assign49690_e64004: f64 = (assign49690_e64003).sqrt();
            let assign49690_e64005: f64 = (locals.var_sp_s_bx__blk1470 - assign49690_e64004);
            let assign49690_e64006: f64 = (0.5 * assign49690_e64005);
            let assign49690_e64007: f64 = (assign49690_e63996 - assign49690_e64006);
            (locals.var_sp_s_eta__blk1453, locals.var_sp_s_eta__blk1453_dn4, locals.var_sp_s_eta__blk1453_dn6, locals.var_sp_s_eta__blk1453_dn7, locals.var_sp_s_eta__blk1453_dn8, locals.var_sp_s_eta__blk1453_dn9, ) = (assign49690_e64007, ((0.5 * ((locals.var_sp_s_x1__blk1469_dn4 + locals.var_sp_s_bx__blk1470_dn4) - ((((locals.var_sp_s_x1__blk1469_dn4 - locals.var_sp_s_bx__blk1470_dn4) * assign49690_e63990) + (assign49690_e63987 * (locals.var_sp_s_x1__blk1469_dn4 - locals.var_sp_s_bx__blk1470_dn4))) / (2.0 * assign49690_e63994)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn4 - (((locals.var_sp_s_bx__blk1470_dn4 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn4)) / (2.0 * assign49690_e64004))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn6 + locals.var_sp_s_bx__blk1470_dn6) - ((((locals.var_sp_s_x1__blk1469_dn6 - locals.var_sp_s_bx__blk1470_dn6) * assign49690_e63990) + (assign49690_e63987 * (locals.var_sp_s_x1__blk1469_dn6 - locals.var_sp_s_bx__blk1470_dn6))) / (2.0 * assign49690_e63994)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn6 - (((locals.var_sp_s_bx__blk1470_dn6 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn6)) / (2.0 * assign49690_e64004))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn7 + locals.var_sp_s_bx__blk1470_dn7) - ((((locals.var_sp_s_x1__blk1469_dn7 - locals.var_sp_s_bx__blk1470_dn7) * assign49690_e63990) + (assign49690_e63987 * (locals.var_sp_s_x1__blk1469_dn7 - locals.var_sp_s_bx__blk1470_dn7))) / (2.0 * assign49690_e63994)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn7 - (((locals.var_sp_s_bx__blk1470_dn7 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn7)) / (2.0 * assign49690_e64004))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn8 + locals.var_sp_s_bx__blk1470_dn8) - ((((locals.var_sp_s_x1__blk1469_dn8 - locals.var_sp_s_bx__blk1470_dn8) * assign49690_e63990) + (assign49690_e63987 * (locals.var_sp_s_x1__blk1469_dn8 - locals.var_sp_s_bx__blk1470_dn8))) / (2.0 * assign49690_e63994)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn8 - (((locals.var_sp_s_bx__blk1470_dn8 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn8)) / (2.0 * assign49690_e64004))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn9 + locals.var_sp_s_bx__blk1470_dn9) - ((((locals.var_sp_s_x1__blk1469_dn9 - locals.var_sp_s_bx__blk1470_dn9) * assign49690_e63990) + (assign49690_e63987 * (locals.var_sp_s_x1__blk1469_dn9 - locals.var_sp_s_bx__blk1470_dn9))) / (2.0 * assign49690_e63994)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn9 - (((locals.var_sp_s_bx__blk1470_dn9 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn9)) / (2.0 * assign49690_e64004))))), );
            locals.var_sp_s_eta__blk1453_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49700_e64021: f64 = (locals.var_xg__blk1343 - locals.var_sp_s_eta__blk1453);
            (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9, ) = (assign49700_e64021, (locals.var_xg__blk1343_dn4 - locals.var_sp_s_eta__blk1453_dn4), (locals.var_xg__blk1343_dn6 - locals.var_sp_s_eta__blk1453_dn6), (locals.var_xg__blk1343_dn7 - locals.var_sp_s_eta__blk1453_dn7), (locals.var_xg__blk1343_dn8 - locals.var_sp_s_eta__blk1453_dn8), (locals.var_xg__blk1343_dn9 - locals.var_sp_s_eta__blk1453_dn9), );
            locals.var_sp_s_temp__blk1448_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49710_e64034: f64 = (-locals.var_sp_s_eta__blk1453);
            let assign49710_e64035: f64 = (assign49710_e64034).exp();
            (locals.var_sp_s_temp1__blk1449, locals.var_sp_s_temp1__blk1449_dn4, locals.var_sp_s_temp1__blk1449_dn6, locals.var_sp_s_temp1__blk1449_dn7, locals.var_sp_s_temp1__blk1449_dn8, locals.var_sp_s_temp1__blk1449_dn9, ) = (assign49710_e64035, (assign49710_e64035 * (-locals.var_sp_s_eta__blk1453_dn4)), (assign49710_e64035 * (-locals.var_sp_s_eta__blk1453_dn6)), (assign49710_e64035 * (-locals.var_sp_s_eta__blk1453_dn7)), (assign49710_e64035 * (-locals.var_sp_s_eta__blk1453_dn8)), (assign49710_e64035 * (-locals.var_sp_s_eta__blk1453_dn9)), );
            locals.var_sp_s_temp1__blk1449_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49720_e64051: f64 = (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453);
            let assign49720_e64052: f64 = (2.0 + assign49720_e64051);
            let assign49720_e64053: f64 = (1.0 / assign49720_e64052);
            (locals.var_sp_s_temp2__blk1450, locals.var_sp_s_temp2__blk1450_dn4, locals.var_sp_s_temp2__blk1450_dn6, locals.var_sp_s_temp2__blk1450_dn7, locals.var_sp_s_temp2__blk1450_dn8, locals.var_sp_s_temp2__blk1450_dn9, ) = (assign49720_e64053, (-(((locals.var_sp_s_eta__blk1453_dn4 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn4)) / (assign49720_e64052 * assign49720_e64052))), (-(((locals.var_sp_s_eta__blk1453_dn6 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn6)) / (assign49720_e64052 * assign49720_e64052))), (-(((locals.var_sp_s_eta__blk1453_dn7 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn7)) / (assign49720_e64052 * assign49720_e64052))), (-(((locals.var_sp_s_eta__blk1453_dn8 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn8)) / (assign49720_e64052 * assign49720_e64052))), (-(((locals.var_sp_s_eta__blk1453_dn9 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn9)) / (assign49720_e64052 * assign49720_e64052))), );
            locals.var_sp_s_temp2__blk1450_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49730_e64067: f64 = (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453);
            let assign49730_e64069: f64 = (assign49730_e64067 * locals.var_sp_s_temp2__blk1450);
            (locals.var_sp_s_xi0__blk1460, locals.var_sp_s_xi0__blk1460_dn4, locals.var_sp_s_xi0__blk1460_dn6, locals.var_sp_s_xi0__blk1460_dn7, locals.var_sp_s_xi0__blk1460_dn8, locals.var_sp_s_xi0__blk1460_dn9, ) = (assign49730_e64069, ((((locals.var_sp_s_eta__blk1453_dn4 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign49730_e64067 * locals.var_sp_s_temp2__blk1450_dn4)), ((((locals.var_sp_s_eta__blk1453_dn6 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign49730_e64067 * locals.var_sp_s_temp2__blk1450_dn6)), ((((locals.var_sp_s_eta__blk1453_dn7 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign49730_e64067 * locals.var_sp_s_temp2__blk1450_dn7)), ((((locals.var_sp_s_eta__blk1453_dn8 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign49730_e64067 * locals.var_sp_s_temp2__blk1450_dn8)), ((((locals.var_sp_s_eta__blk1453_dn9 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign49730_e64067 * locals.var_sp_s_temp2__blk1450_dn9)), );
            locals.var_sp_s_xi0__blk1460_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49740_e64084: f64 = (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450);
            let assign49740_e64086: f64 = (assign49740_e64084 * locals.var_sp_s_temp2__blk1450);
            let assign49740_e64087: f64 = (4.0 * assign49740_e64086);
            (locals.var_sp_s_xi1__blk1461, locals.var_sp_s_xi1__blk1461_dn4, locals.var_sp_s_xi1__blk1461_dn6, locals.var_sp_s_xi1__blk1461_dn7, locals.var_sp_s_xi1__blk1461_dn8, locals.var_sp_s_xi1__blk1461_dn9, ) = (assign49740_e64087, (4.0 * ((((locals.var_sp_s_eta__blk1453_dn4 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign49740_e64084 * locals.var_sp_s_temp2__blk1450_dn4))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn6 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign49740_e64084 * locals.var_sp_s_temp2__blk1450_dn6))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn7 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign49740_e64084 * locals.var_sp_s_temp2__blk1450_dn7))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn8 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign49740_e64084 * locals.var_sp_s_temp2__blk1450_dn8))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn9 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign49740_e64084 * locals.var_sp_s_temp2__blk1450_dn9))), );
            locals.var_sp_s_xi1__blk1461_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49750_e64101: f64 = (8.0 * locals.var_sp_s_temp2__blk1450);
            let assign49750_e64104: f64 = (12.0 * locals.var_sp_s_xi0__blk1460);
            let assign49750_e64105: f64 = (assign49750_e64101 - assign49750_e64104);
            let assign49750_e64107: f64 = (assign49750_e64105 * locals.var_sp_s_temp2__blk1450);
            let assign49750_e64109: f64 = (assign49750_e64107 * locals.var_sp_s_temp2__blk1450);
            (locals.var_sp_s_xi2__blk1462, locals.var_sp_s_xi2__blk1462_dn4, locals.var_sp_s_xi2__blk1462_dn6, locals.var_sp_s_xi2__blk1462_dn7, locals.var_sp_s_xi2__blk1462_dn8, locals.var_sp_s_xi2__blk1462_dn9, ) = (assign49750_e64109, ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn4) - (12.0 * locals.var_sp_s_xi0__blk1460_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64105 * locals.var_sp_s_temp2__blk1450_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64107 * locals.var_sp_s_temp2__blk1450_dn4)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn6) - (12.0 * locals.var_sp_s_xi0__blk1460_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64105 * locals.var_sp_s_temp2__blk1450_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64107 * locals.var_sp_s_temp2__blk1450_dn6)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn7) - (12.0 * locals.var_sp_s_xi0__blk1460_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64105 * locals.var_sp_s_temp2__blk1450_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64107 * locals.var_sp_s_temp2__blk1450_dn7)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn8) - (12.0 * locals.var_sp_s_xi0__blk1460_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64105 * locals.var_sp_s_temp2__blk1450_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64107 * locals.var_sp_s_temp2__blk1450_dn8)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn9) - (12.0 * locals.var_sp_s_xi0__blk1460_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64105 * locals.var_sp_s_temp2__blk1450_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign49750_e64107 * locals.var_sp_s_temp2__blk1450_dn9)), );
            locals.var_sp_s_xi2__blk1462_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49760_e64124: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
            let assign49760_e64128: f64 = (locals.var_sp_s_temp1__blk1449 + locals.var_sp_s_eta__blk1453);
            let assign49760_e64130: f64 = (assign49760_e64128 - 1.0);
            let assign49760_e64134: f64 = (locals.var_sp_s_eta__blk1453 + 1.0);
            let assign49760_e64136: f64 = (assign49760_e64134 + locals.var_sp_s_xi0__blk1460);
            let assign49760_e64137: f64 = (locals.var_delta_ns__blk1364 * assign49760_e64136);
            let assign49760_e64138: f64 = (assign49760_e64130 - assign49760_e64137);
            let assign49760_e64139: f64 = (locals.var_gf2__blk1325 * assign49760_e64138);
            let assign49760_e64140: f64 = (assign49760_e64124 - assign49760_e64139);
            let (assign49760_e64162, assign49760_e64162_d_n4, assign49760_e64162_d_n6, assign49760_e64162_d_n7, assign49760_e64162_d_n8, assign49760_e64162_d_n9,) = {
    if (1e-40 > assign49760_e64140) {
        (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        let assign49760_e64145: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
        let assign49760_e64149: f64 = (locals.var_sp_s_temp1__blk1449 + locals.var_sp_s_eta__blk1453);
        let assign49760_e64151: f64 = (assign49760_e64149 - 1.0);
        let assign49760_e64155: f64 = (locals.var_sp_s_eta__blk1453 + 1.0);
        let assign49760_e64157: f64 = (assign49760_e64155 + locals.var_sp_s_xi0__blk1460);
        let assign49760_e64158: f64 = (locals.var_delta_ns__blk1364 * assign49760_e64157);
        let assign49760_e64159: f64 = (assign49760_e64151 - assign49760_e64158);
        let assign49760_e64160: f64 = (locals.var_gf2__blk1325 * assign49760_e64159);
        let assign49760_e64161: f64 = (assign49760_e64145 - assign49760_e64160);
        (assign49760_e64161, (((locals.var_sp_s_temp__blk1448_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn4)) - ((locals.var_gf2__blk1325_dn4 * assign49760_e64159) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn4 + locals.var_sp_s_eta__blk1453_dn4) - ((locals.var_delta_ns__blk1364_dn4 * assign49760_e64157) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_eta__blk1453_dn4 + locals.var_sp_s_xi0__blk1460_dn4))))))), (((locals.var_sp_s_temp__blk1448_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn6)) - ((locals.var_gf2__blk1325_dn6 * assign49760_e64159) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn6 + locals.var_sp_s_eta__blk1453_dn6) - ((locals.var_delta_ns__blk1364_dn6 * assign49760_e64157) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_eta__blk1453_dn6 + locals.var_sp_s_xi0__blk1460_dn6))))))), (((locals.var_sp_s_temp__blk1448_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn7)) - ((locals.var_gf2__blk1325_dn7 * assign49760_e64159) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn7 + locals.var_sp_s_eta__blk1453_dn7) - ((locals.var_delta_ns__blk1364_dn7 * assign49760_e64157) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_eta__blk1453_dn7 + locals.var_sp_s_xi0__blk1460_dn7))))))), (((locals.var_sp_s_temp__blk1448_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn8)) - ((locals.var_gf2__blk1325_dn8 * assign49760_e64159) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn8 + locals.var_sp_s_eta__blk1453_dn8) - ((locals.var_delta_ns__blk1364_dn8 * assign49760_e64157) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_eta__blk1453_dn8 + locals.var_sp_s_xi0__blk1460_dn8))))))), (((locals.var_sp_s_temp__blk1448_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn9)) - ((locals.var_gf2__blk1325_dn9 * assign49760_e64159) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn9 + locals.var_sp_s_eta__blk1453_dn9) - ((locals.var_delta_ns__blk1364_dn9 * assign49760_e64157) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_eta__blk1453_dn9 + locals.var_sp_s_xi0__blk1460_dn9))))))),)
    }
};
            (locals.var_sp_s_a__blk1454, locals.var_sp_s_a__blk1454_dn4, locals.var_sp_s_a__blk1454_dn6, locals.var_sp_s_a__blk1454_dn7, locals.var_sp_s_a__blk1454_dn8, locals.var_sp_s_a__blk1454_dn9, ) = (assign49760_e64162, assign49760_e64162_d_n4, assign49760_e64162_d_n6, assign49760_e64162_d_n7, assign49760_e64162_d_n8, assign49760_e64162_d_n9, );
            locals.var_sp_s_a__blk1454_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49770_e64180: f64 = (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462);
            let assign49770_e64181: f64 = (locals.var_sp_s_temp1__blk1449 - assign49770_e64180);
            let assign49770_e64182: f64 = (locals.var_gf2__blk1325 * assign49770_e64181);
            let assign49770_e64183: f64 = (0.5 * assign49770_e64182);
            let assign49770_e64184: f64 = (1.0 - assign49770_e64183);
            (locals.var_sp_s_b__blk1471, locals.var_sp_s_b__blk1471_dn4, locals.var_sp_s_b__blk1471_dn6, locals.var_sp_s_b__blk1471_dn7, locals.var_sp_s_b__blk1471_dn8, locals.var_sp_s_b__blk1471_dn9, ) = (assign49770_e64184, (-(0.5 * ((locals.var_gf2__blk1325_dn4 * assign49770_e64181) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn4 - ((locals.var_delta_ns__blk1364_dn4 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn4))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn6 * assign49770_e64181) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn6 - ((locals.var_delta_ns__blk1364_dn6 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn6))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn7 * assign49770_e64181) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn7 - ((locals.var_delta_ns__blk1364_dn7 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn7))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn8 * assign49770_e64181) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn8 - ((locals.var_delta_ns__blk1364_dn8 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn8))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn9 * assign49770_e64181) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn9 - ((locals.var_delta_ns__blk1364_dn9 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn9))))))), );
            locals.var_sp_s_b__blk1471_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_24(
        locals: &mut StampLocals,
    ) {
        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49780_e64198: f64 = (2.0 * locals.var_sp_s_temp__blk1448);
            let assign49780_e64202: f64 = (1.0 - locals.var_sp_s_temp1__blk1449);
            let assign49780_e64206: f64 = (1.0 + locals.var_sp_s_xi1__blk1461);
            let assign49780_e64207: f64 = (locals.var_delta_ns__blk1364 * assign49780_e64206);
            let assign49780_e64208: f64 = (assign49780_e64202 - assign49780_e64207);
            let assign49780_e64209: f64 = (locals.var_gf2__blk1325 * assign49780_e64208);
            let assign49780_e64210: f64 = (assign49780_e64198 + assign49780_e64209);
            (locals.var_sp_s_c__blk1455, locals.var_sp_s_c__blk1455_dn4, locals.var_sp_s_c__blk1455_dn6, locals.var_sp_s_c__blk1455_dn7, locals.var_sp_s_c__blk1455_dn8, locals.var_sp_s_c__blk1455_dn9, ) = (assign49780_e64210, ((2.0 * locals.var_sp_s_temp__blk1448_dn4) + ((locals.var_gf2__blk1325_dn4 * assign49780_e64208) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn4) - ((locals.var_delta_ns__blk1364_dn4 * assign49780_e64206) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn4)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn6) + ((locals.var_gf2__blk1325_dn6 * assign49780_e64208) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn6) - ((locals.var_delta_ns__blk1364_dn6 * assign49780_e64206) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn7) + ((locals.var_gf2__blk1325_dn7 * assign49780_e64208) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn7) - ((locals.var_delta_ns__blk1364_dn7 * assign49780_e64206) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn8) + ((locals.var_gf2__blk1325_dn8 * assign49780_e64208) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn8) - ((locals.var_delta_ns__blk1364_dn8 * assign49780_e64206) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn8)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn9) + ((locals.var_gf2__blk1325_dn9 * assign49780_e64208) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn9) - ((locals.var_delta_ns__blk1364_dn9 * assign49780_e64206) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn9)))))), );
            locals.var_sp_s_c__blk1455_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49790_e64224: f64 = (locals.var_xn_s__blk1349 - locals.var_sp_s_eta__blk1453);
            let assign49790_e64227: f64 = (locals.var_sp_s_a__blk1454 / locals.var_gf2__blk1325);
            let assign49790_e64228: f64 = (assign49790_e64227).ln();
            let assign49790_e64229: f64 = (assign49790_e64224 + assign49790_e64228);
            (locals.var_sp_s_tau__blk1456, locals.var_sp_s_tau__blk1456_dn4, locals.var_sp_s_tau__blk1456_dn6, locals.var_sp_s_tau__blk1456_dn7, locals.var_sp_s_tau__blk1456_dn8, locals.var_sp_s_tau__blk1456_dn9, ) = (assign49790_e64229, ((locals.var_xn_s__blk1349_dn4 - locals.var_sp_s_eta__blk1453_dn4) + ((((locals.var_sp_s_a__blk1454_dn4 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn4)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign49790_e64227)), ((locals.var_xn_s__blk1349_dn6 - locals.var_sp_s_eta__blk1453_dn6) + ((((locals.var_sp_s_a__blk1454_dn6 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn6)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign49790_e64227)), ((locals.var_xn_s__blk1349_dn7 - locals.var_sp_s_eta__blk1453_dn7) + ((((locals.var_sp_s_a__blk1454_dn7 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn7)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign49790_e64227)), ((locals.var_xn_s__blk1349_dn8 - locals.var_sp_s_eta__blk1453_dn8) + ((((locals.var_sp_s_a__blk1454_dn8 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn8)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign49790_e64227)), ((locals.var_xn_s__blk1349_dn9 - locals.var_sp_s_eta__blk1453_dn9) + ((((locals.var_sp_s_a__blk1454_dn9 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn9)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign49790_e64227)), );
            locals.var_sp_s_tau__blk1456_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49800_e64243: f64 = (locals.var_sp_s_a__blk1454 + locals.var_sp_s_c__blk1455);
            (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9, ) = (assign49800_e64243, (locals.var_sp_s_a__blk1454_dn4 + locals.var_sp_s_c__blk1455_dn4), (locals.var_sp_s_a__blk1454_dn6 + locals.var_sp_s_c__blk1455_dn6), (locals.var_sp_s_a__blk1454_dn7 + locals.var_sp_s_c__blk1455_dn7), (locals.var_sp_s_a__blk1454_dn8 + locals.var_sp_s_c__blk1455_dn8), (locals.var_sp_s_a__blk1454_dn9 + locals.var_sp_s_c__blk1455_dn9), );
            locals.var_nu_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49810_e64257: f64 = (locals.var_nu * locals.var_nu);
            let assign49810_e64262: f64 = (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455);
            let assign49810_e64263: f64 = (0.5 * assign49810_e64262);
            let assign49810_e64266: f64 = (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471);
            let assign49810_e64267: f64 = (assign49810_e64263 - assign49810_e64266);
            let assign49810_e64268: f64 = (locals.var_sp_s_tau__blk1456 * assign49810_e64267);
            let assign49810_e64269: f64 = (assign49810_e64257 + assign49810_e64268);
            (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9, ) = (assign49810_e64269, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_sp_s_tau__blk1456_dn4 * assign49810_e64267) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn4 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn4))) - ((locals.var_sp_s_a__blk1454_dn4 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn4)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau__blk1456_dn6 * assign49810_e64267) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn6 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn6))) - ((locals.var_sp_s_a__blk1454_dn6 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau__blk1456_dn7 * assign49810_e64267) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn7 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn7))) - ((locals.var_sp_s_a__blk1454_dn7 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau__blk1456_dn8 * assign49810_e64267) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn8 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn8))) - ((locals.var_sp_s_a__blk1454_dn8 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn8)))))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_sp_s_tau__blk1456_dn9 * assign49810_e64267) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn9 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn9))) - ((locals.var_sp_s_a__blk1454_dn9 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn9)))))), );
            locals.var_mutau_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49820_e64284: f64 = (locals.var_sp_s_a__blk1454 * locals.var_nu);
            let assign49820_e64286: f64 = (assign49820_e64284 * locals.var_sp_s_tau__blk1456);
            let assign49820_e64290: f64 = (locals.var_nu / locals.var_mutau);
            let assign49820_e64292: f64 = (assign49820_e64290 * locals.var_sp_s_tau__blk1456);
            let assign49820_e64294: f64 = (assign49820_e64292 * locals.var_sp_s_tau__blk1456);
            let assign49820_e64296: f64 = (assign49820_e64294 * locals.var_sp_s_c__blk1455);
            let assign49820_e64299: f64 = (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455);
            let assign49820_e64301: f64 = (assign49820_e64299 * 0.3333333333333333);
            let assign49820_e64304: f64 = (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471);
            let assign49820_e64305: f64 = (assign49820_e64301 - assign49820_e64304);
            let assign49820_e64306: f64 = (assign49820_e64296 * assign49820_e64305);
            let assign49820_e64307: f64 = (locals.var_mutau + assign49820_e64306);
            let assign49820_e64308: f64 = (assign49820_e64286 / assign49820_e64307);
            let assign49820_e64309: f64 = (locals.var_sp_s_eta__blk1453 + assign49820_e64308);
            (locals.var_sp_s_x0__blk1472, locals.var_sp_s_x0__blk1472_dn4, locals.var_sp_s_x0__blk1472_dn6, locals.var_sp_s_x0__blk1472_dn7, locals.var_sp_s_x0__blk1472_dn8, locals.var_sp_s_x0__blk1472_dn9, ) = (assign49820_e64309, (locals.var_sp_s_eta__blk1453_dn4 + (((((((locals.var_sp_s_a__blk1454_dn4 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn4)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64284 * locals.var_sp_s_tau__blk1456_dn4)) * assign49820_e64307) - (assign49820_e64286 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64290 * locals.var_sp_s_tau__blk1456_dn4)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64292 * locals.var_sp_s_tau__blk1456_dn4)) * locals.var_sp_s_c__blk1455) + (assign49820_e64294 * locals.var_sp_s_c__blk1455_dn4)) * assign49820_e64305) + (assign49820_e64296 * ((((locals.var_sp_s_c__blk1455_dn4 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn4)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn4 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn4)))))))) / (assign49820_e64307 * assign49820_e64307))), (locals.var_sp_s_eta__blk1453_dn6 + (((((((locals.var_sp_s_a__blk1454_dn6 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn6)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64284 * locals.var_sp_s_tau__blk1456_dn6)) * assign49820_e64307) - (assign49820_e64286 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64290 * locals.var_sp_s_tau__blk1456_dn6)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64292 * locals.var_sp_s_tau__blk1456_dn6)) * locals.var_sp_s_c__blk1455) + (assign49820_e64294 * locals.var_sp_s_c__blk1455_dn6)) * assign49820_e64305) + (assign49820_e64296 * ((((locals.var_sp_s_c__blk1455_dn6 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn6)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn6 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn6)))))))) / (assign49820_e64307 * assign49820_e64307))), (locals.var_sp_s_eta__blk1453_dn7 + (((((((locals.var_sp_s_a__blk1454_dn7 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn7)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64284 * locals.var_sp_s_tau__blk1456_dn7)) * assign49820_e64307) - (assign49820_e64286 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64290 * locals.var_sp_s_tau__blk1456_dn7)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64292 * locals.var_sp_s_tau__blk1456_dn7)) * locals.var_sp_s_c__blk1455) + (assign49820_e64294 * locals.var_sp_s_c__blk1455_dn7)) * assign49820_e64305) + (assign49820_e64296 * ((((locals.var_sp_s_c__blk1455_dn7 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn7)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn7 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn7)))))))) / (assign49820_e64307 * assign49820_e64307))), (locals.var_sp_s_eta__blk1453_dn8 + (((((((locals.var_sp_s_a__blk1454_dn8 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn8)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64284 * locals.var_sp_s_tau__blk1456_dn8)) * assign49820_e64307) - (assign49820_e64286 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64290 * locals.var_sp_s_tau__blk1456_dn8)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64292 * locals.var_sp_s_tau__blk1456_dn8)) * locals.var_sp_s_c__blk1455) + (assign49820_e64294 * locals.var_sp_s_c__blk1455_dn8)) * assign49820_e64305) + (assign49820_e64296 * ((((locals.var_sp_s_c__blk1455_dn8 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn8)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn8 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn8)))))))) / (assign49820_e64307 * assign49820_e64307))), (locals.var_sp_s_eta__blk1453_dn9 + (((((((locals.var_sp_s_a__blk1454_dn9 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn9)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64284 * locals.var_sp_s_tau__blk1456_dn9)) * assign49820_e64307) - (assign49820_e64286 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64290 * locals.var_sp_s_tau__blk1456_dn9)) * locals.var_sp_s_tau__blk1456) + (assign49820_e64292 * locals.var_sp_s_tau__blk1456_dn9)) * locals.var_sp_s_c__blk1455) + (assign49820_e64294 * locals.var_sp_s_c__blk1455_dn9)) * assign49820_e64305) + (assign49820_e64296 * ((((locals.var_sp_s_c__blk1455_dn9 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn9)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn9 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn9)))))))) / (assign49820_e64307 * assign49820_e64307))), );
            locals.var_sp_s_x0__blk1472_rv = 0.0;
        }

        let assign49830_e64314: f64 = if locals.var_sp_s_x0__blk1472 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1489 = assign49830_e64314;
        locals.var_guard1489_rv = 0.0;

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1489 != 0.0)) {
            let assign49840_e64327: f64 = (locals.var_sp_s_x0__blk1472).exp();
            (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9, ) = (assign49840_e64327, (assign49840_e64327 * locals.var_sp_s_x0__blk1472_dn4), (assign49840_e64327 * locals.var_sp_s_x0__blk1472_dn6), (assign49840_e64327 * locals.var_sp_s_x0__blk1472_dn7), (assign49840_e64327 * locals.var_sp_s_x0__blk1472_dn8), (assign49840_e64327 * locals.var_sp_s_x0__blk1472_dn9), );
            locals.var_sp_s_delta0__blk1458_rv = 0.0;
        }

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1489 != 0.0)) {
            let assign49850_e64343: f64 = (1.0 / locals.var_sp_s_delta0__blk1458);
            (locals.var_sp_s_delta1__blk1459, locals.var_sp_s_delta1__blk1459_dn4, locals.var_sp_s_delta1__blk1459_dn6, locals.var_sp_s_delta1__blk1459_dn7, locals.var_sp_s_delta1__blk1459_dn8, locals.var_sp_s_delta1__blk1459_dn9, ) = (assign49850_e64343, (-(locals.var_sp_s_delta0__blk1458_dn4 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn6 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn7 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn8 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn9 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), );
            locals.var_sp_s_delta1__blk1459_rv = 0.0;
        }

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1489 != 0.0)) {
            let assign49860_e64359: f64 = (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458);
            (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9, ) = (assign49860_e64359, ((locals.var_delta_ns__blk1364_dn4 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn4)), ((locals.var_delta_ns__blk1364_dn6 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn6)), ((locals.var_delta_ns__blk1364_dn7 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn7)), ((locals.var_delta_ns__blk1364_dn8 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn8)), ((locals.var_delta_ns__blk1364_dn9 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn9)), );
            locals.var_sp_s_delta0__blk1458_rv = 0.0;
        }

        let assign49870_e64365: f64 = (locals.var_xn_s__blk1349 - 230.25850929940458);
        let assign49870_e64366: f64 = if locals.var_sp_s_x0__blk1472 > assign49870_e64365 { 1.0 } else { 0.0 };
        locals.var_guard1490 = assign49870_e64366;
        locals.var_guard1490_rv = 0.0;

        if ((((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1489 == 0.0)) && (locals.var_guard1490 != 0.0)) {
            let assign49880_e64383: f64 = (locals.var_sp_s_x0__blk1472 - locals.var_xn_s__blk1349);
            let assign49880_e64384: f64 = (assign49880_e64383).exp();
            (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9, ) = (assign49880_e64384, (assign49880_e64384 * (locals.var_sp_s_x0__blk1472_dn4 - locals.var_xn_s__blk1349_dn4)), (assign49880_e64384 * (locals.var_sp_s_x0__blk1472_dn6 - locals.var_xn_s__blk1349_dn6)), (assign49880_e64384 * (locals.var_sp_s_x0__blk1472_dn7 - locals.var_xn_s__blk1349_dn7)), (assign49880_e64384 * (locals.var_sp_s_x0__blk1472_dn8 - locals.var_xn_s__blk1349_dn8)), (assign49880_e64384 * (locals.var_sp_s_x0__blk1472_dn9 - locals.var_xn_s__blk1349_dn9)), );
            locals.var_sp_s_delta0__blk1458_rv = 0.0;
        }

        if ((((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1489 == 0.0)) && (locals.var_guard1490 != 0.0)) {
            let assign49890_e64403: f64 = (locals.var_delta_ns__blk1364 / locals.var_sp_s_delta0__blk1458);
            (locals.var_sp_s_delta1__blk1459, locals.var_sp_s_delta1__blk1459_dn4, locals.var_sp_s_delta1__blk1459_dn6, locals.var_sp_s_delta1__blk1459_dn7, locals.var_sp_s_delta1__blk1459_dn8, locals.var_sp_s_delta1__blk1459_dn9, ) = (assign49890_e64403, (((locals.var_delta_ns__blk1364_dn4 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn4)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_ns__blk1364_dn6 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn6)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_ns__blk1364_dn7 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn7)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_ns__blk1364_dn8 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn8)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_ns__blk1364_dn9 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_ns__blk1364 * locals.var_sp_s_delta0__blk1458_dn9)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), );
            locals.var_sp_s_delta1__blk1459_rv = 0.0;
        }

        if ((((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1489 == 0.0)) && (locals.var_guard1490 == 0.0)) {
            let assign49900_e64425: f64 = (locals.var_xn_s__blk1349 - locals.var_sp_s_x0__blk1472);
            let assign49900_e64427: f64 = (assign49900_e64425 - 230.25850929940458);
            let assign49900_e64432: f64 = (locals.var_xn_s__blk1349 - locals.var_sp_s_x0__blk1472);
            let assign49900_e64434: f64 = (assign49900_e64432 - 230.25850929940458);
            let assign49900_e64438: f64 = (locals.var_xn_s__blk1349 - locals.var_sp_s_x0__blk1472);
            let assign49900_e64440: f64 = (assign49900_e64438 - 230.25850929940458);
            let assign49900_e64442: f64 = (assign49900_e64440 * 0.3333333333333333);
            let assign49900_e64443: f64 = (1.0 + assign49900_e64442);
            let assign49900_e64444: f64 = (assign49900_e64434 * assign49900_e64443);
            let assign49900_e64445: f64 = (0.5 * assign49900_e64444);
            let assign49900_e64446: f64 = (1.0 + assign49900_e64445);
            let assign49900_e64447: f64 = (assign49900_e64427 * assign49900_e64446);
            let assign49900_e64448: f64 = (1.0 + assign49900_e64447);
            let assign49900_e64449: f64 = (1e-100 / assign49900_e64448);
            (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9, ) = (assign49900_e64449, (-((1e-100 * (((locals.var_xn_s__blk1349_dn4 - locals.var_sp_s_x0__blk1472_dn4) * assign49900_e64446) + (assign49900_e64427 * (0.5 * (((locals.var_xn_s__blk1349_dn4 - locals.var_sp_s_x0__blk1472_dn4) * assign49900_e64443) + (assign49900_e64434 * ((locals.var_xn_s__blk1349_dn4 - locals.var_sp_s_x0__blk1472_dn4) * 0.3333333333333333))))))) / (assign49900_e64448 * assign49900_e64448))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn6 - locals.var_sp_s_x0__blk1472_dn6) * assign49900_e64446) + (assign49900_e64427 * (0.5 * (((locals.var_xn_s__blk1349_dn6 - locals.var_sp_s_x0__blk1472_dn6) * assign49900_e64443) + (assign49900_e64434 * ((locals.var_xn_s__blk1349_dn6 - locals.var_sp_s_x0__blk1472_dn6) * 0.3333333333333333))))))) / (assign49900_e64448 * assign49900_e64448))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn7 - locals.var_sp_s_x0__blk1472_dn7) * assign49900_e64446) + (assign49900_e64427 * (0.5 * (((locals.var_xn_s__blk1349_dn7 - locals.var_sp_s_x0__blk1472_dn7) * assign49900_e64443) + (assign49900_e64434 * ((locals.var_xn_s__blk1349_dn7 - locals.var_sp_s_x0__blk1472_dn7) * 0.3333333333333333))))))) / (assign49900_e64448 * assign49900_e64448))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn8 - locals.var_sp_s_x0__blk1472_dn8) * assign49900_e64446) + (assign49900_e64427 * (0.5 * (((locals.var_xn_s__blk1349_dn8 - locals.var_sp_s_x0__blk1472_dn8) * assign49900_e64443) + (assign49900_e64434 * ((locals.var_xn_s__blk1349_dn8 - locals.var_sp_s_x0__blk1472_dn8) * 0.3333333333333333))))))) / (assign49900_e64448 * assign49900_e64448))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn9 - locals.var_sp_s_x0__blk1472_dn9) * assign49900_e64446) + (assign49900_e64427 * (0.5 * (((locals.var_xn_s__blk1349_dn9 - locals.var_sp_s_x0__blk1472_dn9) * assign49900_e64443) + (assign49900_e64434 * ((locals.var_xn_s__blk1349_dn9 - locals.var_sp_s_x0__blk1472_dn9) * 0.3333333333333333))))))) / (assign49900_e64448 * assign49900_e64448))), );
            locals.var_sp_s_delta0__blk1458_rv = 0.0;
        }

        if ((((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1489 == 0.0)) && (locals.var_guard1490 == 0.0)) {
            let assign49910_e64471: f64 = (locals.var_sp_s_x0__blk1472 - 230.25850929940458);
            let assign49910_e64476: f64 = (locals.var_sp_s_x0__blk1472 - 230.25850929940458);
            let assign49910_e64480: f64 = (locals.var_sp_s_x0__blk1472 - 230.25850929940458);
            let assign49910_e64482: f64 = (assign49910_e64480 * 0.3333333333333333);
            let assign49910_e64483: f64 = (1.0 + assign49910_e64482);
            let assign49910_e64484: f64 = (assign49910_e64476 * assign49910_e64483);
            let assign49910_e64485: f64 = (0.5 * assign49910_e64484);
            let assign49910_e64486: f64 = (1.0 + assign49910_e64485);
            let assign49910_e64487: f64 = (assign49910_e64471 * assign49910_e64486);
            let assign49910_e64488: f64 = (1.0 + assign49910_e64487);
            let assign49910_e64489: f64 = (1e-100 / assign49910_e64488);
            (locals.var_sp_s_delta1__blk1459, locals.var_sp_s_delta1__blk1459_dn4, locals.var_sp_s_delta1__blk1459_dn6, locals.var_sp_s_delta1__blk1459_dn7, locals.var_sp_s_delta1__blk1459_dn8, locals.var_sp_s_delta1__blk1459_dn9, ) = (assign49910_e64489, (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn4 * assign49910_e64486) + (assign49910_e64471 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn4 * assign49910_e64483) + (assign49910_e64476 * (locals.var_sp_s_x0__blk1472_dn4 * 0.3333333333333333))))))) / (assign49910_e64488 * assign49910_e64488))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn6 * assign49910_e64486) + (assign49910_e64471 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn6 * assign49910_e64483) + (assign49910_e64476 * (locals.var_sp_s_x0__blk1472_dn6 * 0.3333333333333333))))))) / (assign49910_e64488 * assign49910_e64488))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn7 * assign49910_e64486) + (assign49910_e64471 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn7 * assign49910_e64483) + (assign49910_e64476 * (locals.var_sp_s_x0__blk1472_dn7 * 0.3333333333333333))))))) / (assign49910_e64488 * assign49910_e64488))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn8 * assign49910_e64486) + (assign49910_e64471 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn8 * assign49910_e64483) + (assign49910_e64476 * (locals.var_sp_s_x0__blk1472_dn8 * 0.3333333333333333))))))) / (assign49910_e64488 * assign49910_e64488))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn9 * assign49910_e64486) + (assign49910_e64471 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn9 * assign49910_e64483) + (assign49910_e64476 * (locals.var_sp_s_x0__blk1472_dn9 * 0.3333333333333333))))))) / (assign49910_e64488 * assign49910_e64488))), );
            locals.var_sp_s_delta1__blk1459_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49920_e64505: f64 = (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472);
            let assign49920_e64506: f64 = (2.0 + assign49920_e64505);
            let assign49920_e64507: f64 = (1.0 / assign49920_e64506);
            (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9, ) = (assign49920_e64507, (-(((locals.var_sp_s_x0__blk1472_dn4 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn4)) / (assign49920_e64506 * assign49920_e64506))), (-(((locals.var_sp_s_x0__blk1472_dn6 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn6)) / (assign49920_e64506 * assign49920_e64506))), (-(((locals.var_sp_s_x0__blk1472_dn7 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn7)) / (assign49920_e64506 * assign49920_e64506))), (-(((locals.var_sp_s_x0__blk1472_dn8 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn8)) / (assign49920_e64506 * assign49920_e64506))), (-(((locals.var_sp_s_x0__blk1472_dn9 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn9)) / (assign49920_e64506 * assign49920_e64506))), );
            locals.var_sp_s_temp__blk1448_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49930_e64521: f64 = (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472);
            let assign49930_e64523: f64 = (assign49930_e64521 * locals.var_sp_s_temp__blk1448);
            (locals.var_sp_s_xi0__blk1460, locals.var_sp_s_xi0__blk1460_dn4, locals.var_sp_s_xi0__blk1460_dn6, locals.var_sp_s_xi0__blk1460_dn7, locals.var_sp_s_xi0__blk1460_dn8, locals.var_sp_s_xi0__blk1460_dn9, ) = (assign49930_e64523, ((((locals.var_sp_s_x0__blk1472_dn4 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49930_e64521 * locals.var_sp_s_temp__blk1448_dn4)), ((((locals.var_sp_s_x0__blk1472_dn6 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49930_e64521 * locals.var_sp_s_temp__blk1448_dn6)), ((((locals.var_sp_s_x0__blk1472_dn7 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49930_e64521 * locals.var_sp_s_temp__blk1448_dn7)), ((((locals.var_sp_s_x0__blk1472_dn8 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49930_e64521 * locals.var_sp_s_temp__blk1448_dn8)), ((((locals.var_sp_s_x0__blk1472_dn9 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49930_e64521 * locals.var_sp_s_temp__blk1448_dn9)), );
            locals.var_sp_s_xi0__blk1460_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49940_e64538: f64 = (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448);
            let assign49940_e64540: f64 = (assign49940_e64538 * locals.var_sp_s_temp__blk1448);
            let assign49940_e64541: f64 = (4.0 * assign49940_e64540);
            (locals.var_sp_s_xi1__blk1461, locals.var_sp_s_xi1__blk1461_dn4, locals.var_sp_s_xi1__blk1461_dn6, locals.var_sp_s_xi1__blk1461_dn7, locals.var_sp_s_xi1__blk1461_dn8, locals.var_sp_s_xi1__blk1461_dn9, ) = (assign49940_e64541, (4.0 * ((((locals.var_sp_s_x0__blk1472_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49940_e64538 * locals.var_sp_s_temp__blk1448_dn4))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49940_e64538 * locals.var_sp_s_temp__blk1448_dn6))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49940_e64538 * locals.var_sp_s_temp__blk1448_dn7))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49940_e64538 * locals.var_sp_s_temp__blk1448_dn8))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49940_e64538 * locals.var_sp_s_temp__blk1448_dn9))), );
            locals.var_sp_s_xi1__blk1461_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49950_e64555: f64 = (8.0 * locals.var_sp_s_temp__blk1448);
            let assign49950_e64558: f64 = (12.0 * locals.var_sp_s_xi0__blk1460);
            let assign49950_e64559: f64 = (assign49950_e64555 - assign49950_e64558);
            let assign49950_e64561: f64 = (assign49950_e64559 * locals.var_sp_s_temp__blk1448);
            let assign49950_e64563: f64 = (assign49950_e64561 * locals.var_sp_s_temp__blk1448);
            (locals.var_sp_s_xi2__blk1462, locals.var_sp_s_xi2__blk1462_dn4, locals.var_sp_s_xi2__blk1462_dn6, locals.var_sp_s_xi2__blk1462_dn7, locals.var_sp_s_xi2__blk1462_dn8, locals.var_sp_s_xi2__blk1462_dn9, ) = (assign49950_e64563, ((((((8.0 * locals.var_sp_s_temp__blk1448_dn4) - (12.0 * locals.var_sp_s_xi0__blk1460_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64559 * locals.var_sp_s_temp__blk1448_dn4)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64561 * locals.var_sp_s_temp__blk1448_dn4)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn6) - (12.0 * locals.var_sp_s_xi0__blk1460_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64559 * locals.var_sp_s_temp__blk1448_dn6)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64561 * locals.var_sp_s_temp__blk1448_dn6)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn7) - (12.0 * locals.var_sp_s_xi0__blk1460_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64559 * locals.var_sp_s_temp__blk1448_dn7)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64561 * locals.var_sp_s_temp__blk1448_dn7)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn8) - (12.0 * locals.var_sp_s_xi0__blk1460_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64559 * locals.var_sp_s_temp__blk1448_dn8)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64561 * locals.var_sp_s_temp__blk1448_dn8)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn9) - (12.0 * locals.var_sp_s_xi0__blk1460_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64559 * locals.var_sp_s_temp__blk1448_dn9)) * locals.var_sp_s_temp__blk1448) + (assign49950_e64561 * locals.var_sp_s_temp__blk1448_dn9)), );
            locals.var_sp_s_xi2__blk1462_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49960_e64577: f64 = (locals.var_xg__blk1343 - locals.var_sp_s_x0__blk1472);
            (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9, ) = (assign49960_e64577, (locals.var_xg__blk1343_dn4 - locals.var_sp_s_x0__blk1472_dn4), (locals.var_xg__blk1343_dn6 - locals.var_sp_s_x0__blk1472_dn6), (locals.var_xg__blk1343_dn7 - locals.var_sp_s_x0__blk1472_dn7), (locals.var_xg__blk1343_dn8 - locals.var_sp_s_x0__blk1472_dn8), (locals.var_xg__blk1343_dn9 - locals.var_sp_s_x0__blk1472_dn9), );
            locals.var_sp_s_temp__blk1448_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49970_e64591: f64 = (2.0 * locals.var_sp_s_temp__blk1448);
            let assign49970_e64595: f64 = (1.0 - locals.var_sp_s_delta1__blk1459);
            let assign49970_e64597: f64 = (assign49970_e64595 + locals.var_sp_s_delta0__blk1458);
            let assign49970_e64601: f64 = (1.0 + locals.var_sp_s_xi1__blk1461);
            let assign49970_e64602: f64 = (locals.var_delta_ns__blk1364 * assign49970_e64601);
            let assign49970_e64603: f64 = (assign49970_e64597 - assign49970_e64602);
            let assign49970_e64604: f64 = (locals.var_gf2__blk1325 * assign49970_e64603);
            let assign49970_e64605: f64 = (assign49970_e64591 + assign49970_e64604);
            (locals.var_sp_s_pc__blk1463, locals.var_sp_s_pc__blk1463_dn4, locals.var_sp_s_pc__blk1463_dn6, locals.var_sp_s_pc__blk1463_dn7, locals.var_sp_s_pc__blk1463_dn8, locals.var_sp_s_pc__blk1463_dn9, ) = (assign49970_e64605, ((2.0 * locals.var_sp_s_temp__blk1448_dn4) + ((locals.var_gf2__blk1325_dn4 * assign49970_e64603) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn4) + locals.var_sp_s_delta0__blk1458_dn4) - ((locals.var_delta_ns__blk1364_dn4 * assign49970_e64601) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn4)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn6) + ((locals.var_gf2__blk1325_dn6 * assign49970_e64603) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn6) + locals.var_sp_s_delta0__blk1458_dn6) - ((locals.var_delta_ns__blk1364_dn6 * assign49970_e64601) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn7) + ((locals.var_gf2__blk1325_dn7 * assign49970_e64603) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn7) + locals.var_sp_s_delta0__blk1458_dn7) - ((locals.var_delta_ns__blk1364_dn7 * assign49970_e64601) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn8) + ((locals.var_gf2__blk1325_dn8 * assign49970_e64603) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn8) + locals.var_sp_s_delta0__blk1458_dn8) - ((locals.var_delta_ns__blk1364_dn8 * assign49970_e64601) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn8)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn9) + ((locals.var_gf2__blk1325_dn9 * assign49970_e64603) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn9) + locals.var_sp_s_delta0__blk1458_dn9) - ((locals.var_delta_ns__blk1364_dn9 * assign49970_e64601) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi1__blk1461_dn9)))))), );
            locals.var_sp_s_pc__blk1463_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49980_e64619: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
            let assign49980_e64623: f64 = (locals.var_sp_s_delta1__blk1459 + locals.var_sp_s_x0__blk1472);
            let assign49980_e64625: f64 = (assign49980_e64623 - 1.0);
            let assign49980_e64627: f64 = (assign49980_e64625 + locals.var_sp_s_delta0__blk1458);
            let assign49980_e64631: f64 = (locals.var_sp_s_x0__blk1472 + 1.0);
            let assign49980_e64633: f64 = (assign49980_e64631 + locals.var_sp_s_xi0__blk1460);
            let assign49980_e64634: f64 = (locals.var_delta_ns__blk1364 * assign49980_e64633);
            let assign49980_e64635: f64 = (assign49980_e64627 - assign49980_e64634);
            let assign49980_e64636: f64 = (locals.var_gf2__blk1325 * assign49980_e64635);
            let assign49980_e64637: f64 = (assign49980_e64619 - assign49980_e64636);
            (locals.var_sp_s_qc__blk1464, locals.var_sp_s_qc__blk1464_dn4, locals.var_sp_s_qc__blk1464_dn6, locals.var_sp_s_qc__blk1464_dn7, locals.var_sp_s_qc__blk1464_dn8, locals.var_sp_s_qc__blk1464_dn9, ) = (assign49980_e64637, (((locals.var_sp_s_temp__blk1448_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn4)) - ((locals.var_gf2__blk1325_dn4 * assign49980_e64635) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn4 + locals.var_sp_s_x0__blk1472_dn4) + locals.var_sp_s_delta0__blk1458_dn4) - ((locals.var_delta_ns__blk1364_dn4 * assign49980_e64633) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_x0__blk1472_dn4 + locals.var_sp_s_xi0__blk1460_dn4))))))), (((locals.var_sp_s_temp__blk1448_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn6)) - ((locals.var_gf2__blk1325_dn6 * assign49980_e64635) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn6 + locals.var_sp_s_x0__blk1472_dn6) + locals.var_sp_s_delta0__blk1458_dn6) - ((locals.var_delta_ns__blk1364_dn6 * assign49980_e64633) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_x0__blk1472_dn6 + locals.var_sp_s_xi0__blk1460_dn6))))))), (((locals.var_sp_s_temp__blk1448_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn7)) - ((locals.var_gf2__blk1325_dn7 * assign49980_e64635) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn7 + locals.var_sp_s_x0__blk1472_dn7) + locals.var_sp_s_delta0__blk1458_dn7) - ((locals.var_delta_ns__blk1364_dn7 * assign49980_e64633) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_x0__blk1472_dn7 + locals.var_sp_s_xi0__blk1460_dn7))))))), (((locals.var_sp_s_temp__blk1448_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn8)) - ((locals.var_gf2__blk1325_dn8 * assign49980_e64635) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn8 + locals.var_sp_s_x0__blk1472_dn8) + locals.var_sp_s_delta0__blk1458_dn8) - ((locals.var_delta_ns__blk1364_dn8 * assign49980_e64633) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_x0__blk1472_dn8 + locals.var_sp_s_xi0__blk1460_dn8))))))), (((locals.var_sp_s_temp__blk1448_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn9)) - ((locals.var_gf2__blk1325_dn9 * assign49980_e64635) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn9 + locals.var_sp_s_x0__blk1472_dn9) + locals.var_sp_s_delta0__blk1458_dn9) - ((locals.var_delta_ns__blk1364_dn9 * assign49980_e64633) + (locals.var_delta_ns__blk1364 * (locals.var_sp_s_x0__blk1472_dn9 + locals.var_sp_s_xi0__blk1460_dn9))))))), );
            locals.var_sp_s_qc__blk1464_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign49990_e64653: f64 = (locals.var_sp_s_delta1__blk1459 + locals.var_sp_s_delta0__blk1458);
            let assign49990_e64656: f64 = (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462);
            let assign49990_e64657: f64 = (assign49990_e64653 - assign49990_e64656);
            let assign49990_e64658: f64 = (locals.var_gf2__blk1325 * assign49990_e64657);
            let assign49990_e64659: f64 = (2.0 - assign49990_e64658);
            (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9, ) = (assign49990_e64659, (-((locals.var_gf2__blk1325_dn4 * assign49990_e64657) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn4 + locals.var_sp_s_delta0__blk1458_dn4) - ((locals.var_delta_ns__blk1364_dn4 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn4)))))), (-((locals.var_gf2__blk1325_dn6 * assign49990_e64657) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn6 + locals.var_sp_s_delta0__blk1458_dn6) - ((locals.var_delta_ns__blk1364_dn6 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn6)))))), (-((locals.var_gf2__blk1325_dn7 * assign49990_e64657) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn7 + locals.var_sp_s_delta0__blk1458_dn7) - ((locals.var_delta_ns__blk1364_dn7 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn7)))))), (-((locals.var_gf2__blk1325_dn8 * assign49990_e64657) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn8 + locals.var_sp_s_delta0__blk1458_dn8) - ((locals.var_delta_ns__blk1364_dn8 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn8)))))), (-((locals.var_gf2__blk1325_dn9 * assign49990_e64657) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn9 + locals.var_sp_s_delta0__blk1458_dn9) - ((locals.var_delta_ns__blk1364_dn9 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_ns__blk1364 * locals.var_sp_s_xi2__blk1462_dn9)))))), );
            locals.var_sp_s_temp__blk1448_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign50000_e64673: f64 = (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463);
            let assign50000_e64677: f64 = (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448);
            let assign50000_e64678: f64 = (2.0 * assign50000_e64677);
            let assign50000_e64679: f64 = (assign50000_e64673 - assign50000_e64678);
            (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9, ) = (assign50000_e64679, (((locals.var_sp_s_pc__blk1463_dn4 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn4)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn4)))), (((locals.var_sp_s_pc__blk1463_dn6 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn6)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn6)))), (((locals.var_sp_s_pc__blk1463_dn7 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn7)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn7)))), (((locals.var_sp_s_pc__blk1463_dn8 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn8)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn8)))), (((locals.var_sp_s_pc__blk1463_dn9 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn9)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn9)))), );
            locals.var_sp_s_temp__blk1448_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1485 == 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign50010_e64696: f64 = (locals.var_sp_s_temp__blk1448).sqrt();
            let assign50010_e64697: f64 = (locals.var_sp_s_pc__blk1463 + assign50010_e64696);
            let assign50010_e64698: f64 = (locals.var_sp_s_qc__blk1464 / assign50010_e64697);
            let assign50010_e64699: f64 = (2.0 * assign50010_e64698);
            let assign50010_e64700: f64 = (locals.var_sp_s_x0__blk1472 + assign50010_e64699);
            (locals.var_x_s__blk1363, locals.var_x_s__blk1363_dn4, locals.var_x_s__blk1363_dn6, locals.var_x_s__blk1363_dn7, locals.var_x_s__blk1363_dn8, locals.var_x_s__blk1363_dn9, ) = (assign50010_e64700, (locals.var_sp_s_x0__blk1472_dn4 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn4 * assign50010_e64697) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn4 + (locals.var_sp_s_temp__blk1448_dn4 / (2.0 * assign50010_e64696))))) / (assign50010_e64697 * assign50010_e64697)))), (locals.var_sp_s_x0__blk1472_dn6 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn6 * assign50010_e64697) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn6 + (locals.var_sp_s_temp__blk1448_dn6 / (2.0 * assign50010_e64696))))) / (assign50010_e64697 * assign50010_e64697)))), (locals.var_sp_s_x0__blk1472_dn7 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn7 * assign50010_e64697) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn7 + (locals.var_sp_s_temp__blk1448_dn7 / (2.0 * assign50010_e64696))))) / (assign50010_e64697 * assign50010_e64697)))), (locals.var_sp_s_x0__blk1472_dn8 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn8 * assign50010_e64697) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn8 + (locals.var_sp_s_temp__blk1448_dn8 / (2.0 * assign50010_e64696))))) / (assign50010_e64697 * assign50010_e64697)))), (locals.var_sp_s_x0__blk1472_dn9 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn9 * assign50010_e64697) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn9 + (locals.var_sp_s_temp__blk1448_dn9 / (2.0 * assign50010_e64696))))) / (assign50010_e64697 * assign50010_e64697)))), );
            locals.var_x_s__blk1363_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            (locals.var_xi1s__blk1366, locals.var_xi1s__blk1366_dn4, locals.var_xi1s__blk1366_dn6, locals.var_xi1s__blk1366_dn7, locals.var_xi1s__blk1366_dn8, locals.var_xi1s__blk1366_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_xi1s__blk1366_rv = 0.0;
            (locals.var_xi2s__blk1367, locals.var_xi2s__blk1367_dn4, locals.var_xi2s__blk1367_dn6, locals.var_xi2s__blk1367_dn7, locals.var_xi2s__blk1367_dn8, locals.var_xi2s__blk1367_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_xi2s__blk1367_rv = 0.0;
            (locals.var_delta_1s__blk1368, locals.var_delta_1s__blk1368_dn4, locals.var_delta_1s__blk1368_dn6, locals.var_delta_1s__blk1368_dn7, locals.var_delta_1s__blk1368_dn8, locals.var_delta_1s__blk1368_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_delta_1s__blk1368_rv = 0.0;
            (locals.var_es__blk1369, locals.var_es__blk1369_dn4, locals.var_es__blk1369_dn6, locals.var_es__blk1369_dn7, locals.var_es__blk1369_dn8, locals.var_es__blk1369_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_es__blk1369_rv = 0.0;
            (locals.var_ds__blk1370, locals.var_ds__blk1370_dn4, locals.var_ds__blk1370_dn6, locals.var_ds__blk1370_dn7, locals.var_ds__blk1370_dn8, locals.var_ds__blk1370_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_ds__blk1370_rv = 0.0;
            (locals.var_ps__blk1371, locals.var_ps__blk1371_dn4, locals.var_ps__blk1371_dn6, locals.var_ps__blk1371_dn7, locals.var_ps__blk1371_dn8, locals.var_ps__blk1371_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_ps__blk1371_rv = 0.0;
            (locals.var_sqs__blk1372, locals.var_sqs__blk1372_dn4, locals.var_sqs__blk1372_dn6, locals.var_sqs__blk1372_dn7, locals.var_sqs__blk1372_dn8, locals.var_sqs__blk1372_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_sqs__blk1372_rv = 0.0;
            (locals.var_alphas__blk1373, locals.var_alphas__blk1373_dn4, locals.var_alphas__blk1373_dn6, locals.var_alphas__blk1373_dn7, locals.var_alphas__blk1373_dn8, locals.var_alphas__blk1373_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_alphas__blk1373_rv = 0.0;
            (locals.var_rxcor__blk1374, locals.var_rxcor__blk1374_dn4, locals.var_rxcor__blk1374_dn6, locals.var_rxcor__blk1374_dn7, locals.var_rxcor__blk1374_dn8, locals.var_rxcor__blk1374_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_rxcor__blk1374_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign50110_e64762: f64 = (locals.var_xg__blk1343 - locals.var_x_s__blk1363);
            (locals.var_xgs__blk1375, locals.var_xgs__blk1375_dn4, locals.var_xgs__blk1375_dn6, locals.var_xgs__blk1375_dn7, locals.var_xgs__blk1375_dn8, locals.var_xgs__blk1375_dn9, ) = (assign50110_e64762, (locals.var_xg__blk1343_dn4 - locals.var_x_s__blk1363_dn4), (locals.var_xg__blk1343_dn6 - locals.var_x_s__blk1363_dn6), (locals.var_xg__blk1343_dn7 - locals.var_x_s__blk1363_dn7), (locals.var_xg__blk1343_dn8 - locals.var_x_s__blk1363_dn8), (locals.var_xg__blk1343_dn9 - locals.var_x_s__blk1363_dn9), );
            locals.var_xgs__blk1375_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            (locals.var_qis__blk1376, locals.var_qis__blk1376_dn4, locals.var_qis__blk1376_dn6, locals.var_qis__blk1376_dn7, locals.var_qis__blk1376_dn8, locals.var_qis__blk1376_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qis__blk1376_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            let assign50130_e64776: f64 = (locals.var_phit1__blk1339 * locals.var_xgs__blk1375);
            (locals.var_qbs__blk1377, locals.var_qbs__blk1377_dn4, locals.var_qbs__blk1377_dn6, locals.var_qbs__blk1377_dn7, locals.var_qbs__blk1377_dn8, locals.var_qbs__blk1377_dn9, ) = (assign50130_e64776, ((locals.var_phit1__blk1339_dn4 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn4)), ((locals.var_phit1__blk1339_dn6 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn6)), ((locals.var_phit1__blk1339_dn7 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn7)), ((locals.var_phit1__blk1339_dn8 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn8)), ((locals.var_phit1__blk1339_dn9 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn9)), );
            locals.var_qbs__blk1377_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) {
            (locals.var_rhob__blk1378, locals.var_rhob__blk1378_dn4, locals.var_rhob__blk1378_dn6, locals.var_rhob__blk1378_dn7, locals.var_rhob__blk1378_dn8, locals.var_rhob__blk1378_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_rhob__blk1378_rv = 0.0;
            (locals.var_rhog__blk1379, locals.var_rhog__blk1379_dn4, locals.var_rhog__blk1379_dn6, locals.var_rhog__blk1379_dn7, locals.var_rhog__blk1379_dn8, locals.var_rhog__blk1379_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_rhog__blk1379_rv = 0.0;
            (locals.var_gmobs__blk1383, locals.var_gmobs__blk1383_dn4, locals.var_gmobs__blk1383_dn6, locals.var_gmobs__blk1383_dn7, locals.var_gmobs__blk1383_dn8, locals.var_gmobs__blk1383_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_gmobs__blk1383_rv = 0.0;
            (locals.var_xitsb__blk1384, locals.var_xitsb__blk1384_dn4, locals.var_xitsb__blk1384_dn6, locals.var_xitsb__blk1384_dn7, locals.var_xitsb__blk1384_dn8, locals.var_xitsb__blk1384_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_xitsb__blk1384_rv = 0.0;
            (locals.var_factheta__blk1386, locals.var_factheta__blk1386_dn4, locals.var_factheta__blk1386_dn6, locals.var_factheta__blk1386_dn7, locals.var_factheta__blk1386_dn8, locals.var_factheta__blk1386_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_factheta__blk1386_rv = 0.0;
        }

        let assign50190_e64811: f64 = if locals.var_xg__blk1343 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1491 = assign50190_e64811;
        locals.var_guard1491_rv = 0.0;

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) {
            let assign50200_e64821: f64 = (locals.var_x_s__blk1363 * locals.var_x_s__blk1363);
            let assign50200_e64822: f64 = (2.0 + assign50200_e64821);
            let assign50200_e64823: f64 = (1.0 / assign50200_e64822);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign50200_e64823, (-(((locals.var_x_s__blk1363_dn4 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn4)) / (assign50200_e64822 * assign50200_e64822))), (-(((locals.var_x_s__blk1363_dn6 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn6)) / (assign50200_e64822 * assign50200_e64822))), (-(((locals.var_x_s__blk1363_dn7 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn7)) / (assign50200_e64822 * assign50200_e64822))), (-(((locals.var_x_s__blk1363_dn8 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn8)) / (assign50200_e64822 * assign50200_e64822))), (-(((locals.var_x_s__blk1363_dn9 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn9)) / (assign50200_e64822 * assign50200_e64822))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) {
            let assign50210_e64833: f64 = (locals.var_x_s__blk1363 * locals.var_x_s__blk1363);
            let assign50210_e64835: f64 = (assign50210_e64833 * locals.var_temp__blk949);
            (locals.var_xi0s__blk1365, locals.var_xi0s__blk1365_dn4, locals.var_xi0s__blk1365_dn6, locals.var_xi0s__blk1365_dn7, locals.var_xi0s__blk1365_dn8, locals.var_xi0s__blk1365_dn9, ) = (assign50210_e64835, ((((locals.var_x_s__blk1363_dn4 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn4)) * locals.var_temp__blk949) + (assign50210_e64833 * locals.var_temp__blk949_dn4)), ((((locals.var_x_s__blk1363_dn6 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn6)) * locals.var_temp__blk949) + (assign50210_e64833 * locals.var_temp__blk949_dn6)), ((((locals.var_x_s__blk1363_dn7 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn7)) * locals.var_temp__blk949) + (assign50210_e64833 * locals.var_temp__blk949_dn7)), ((((locals.var_x_s__blk1363_dn8 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn8)) * locals.var_temp__blk949) + (assign50210_e64833 * locals.var_temp__blk949_dn8)), ((((locals.var_x_s__blk1363_dn9 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn9)) * locals.var_temp__blk949) + (assign50210_e64833 * locals.var_temp__blk949_dn9)), );
            locals.var_xi0s__blk1365_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) {
            let assign50220_e64846: f64 = (locals.var_x_s__blk1363 * locals.var_temp__blk949);
            let assign50220_e64848: f64 = (assign50220_e64846 * locals.var_temp__blk949);
            let assign50220_e64849: f64 = (4.0 * assign50220_e64848);
            (locals.var_xi1s__blk1366, locals.var_xi1s__blk1366_dn4, locals.var_xi1s__blk1366_dn6, locals.var_xi1s__blk1366_dn7, locals.var_xi1s__blk1366_dn8, locals.var_xi1s__blk1366_dn9, ) = (assign50220_e64849, (4.0 * ((((locals.var_x_s__blk1363_dn4 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn4)) * locals.var_temp__blk949) + (assign50220_e64846 * locals.var_temp__blk949_dn4))), (4.0 * ((((locals.var_x_s__blk1363_dn6 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn6)) * locals.var_temp__blk949) + (assign50220_e64846 * locals.var_temp__blk949_dn6))), (4.0 * ((((locals.var_x_s__blk1363_dn7 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn7)) * locals.var_temp__blk949) + (assign50220_e64846 * locals.var_temp__blk949_dn7))), (4.0 * ((((locals.var_x_s__blk1363_dn8 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn8)) * locals.var_temp__blk949) + (assign50220_e64846 * locals.var_temp__blk949_dn8))), (4.0 * ((((locals.var_x_s__blk1363_dn9 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn9)) * locals.var_temp__blk949) + (assign50220_e64846 * locals.var_temp__blk949_dn9))), );
            locals.var_xi1s__blk1366_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) {
            let assign50230_e64859: f64 = (8.0 * locals.var_temp__blk949);
            let assign50230_e64862: f64 = (12.0 * locals.var_xi0s__blk1365);
            let assign50230_e64863: f64 = (assign50230_e64859 - assign50230_e64862);
            let assign50230_e64865: f64 = (assign50230_e64863 * locals.var_temp__blk949);
            let assign50230_e64867: f64 = (assign50230_e64865 * locals.var_temp__blk949);
            (locals.var_xi2s__blk1367, locals.var_xi2s__blk1367_dn4, locals.var_xi2s__blk1367_dn6, locals.var_xi2s__blk1367_dn7, locals.var_xi2s__blk1367_dn8, locals.var_xi2s__blk1367_dn9, ) = (assign50230_e64867, ((((((8.0 * locals.var_temp__blk949_dn4) - (12.0 * locals.var_xi0s__blk1365_dn4)) * locals.var_temp__blk949) + (assign50230_e64863 * locals.var_temp__blk949_dn4)) * locals.var_temp__blk949) + (assign50230_e64865 * locals.var_temp__blk949_dn4)), ((((((8.0 * locals.var_temp__blk949_dn6) - (12.0 * locals.var_xi0s__blk1365_dn6)) * locals.var_temp__blk949) + (assign50230_e64863 * locals.var_temp__blk949_dn6)) * locals.var_temp__blk949) + (assign50230_e64865 * locals.var_temp__blk949_dn6)), ((((((8.0 * locals.var_temp__blk949_dn7) - (12.0 * locals.var_xi0s__blk1365_dn7)) * locals.var_temp__blk949) + (assign50230_e64863 * locals.var_temp__blk949_dn7)) * locals.var_temp__blk949) + (assign50230_e64865 * locals.var_temp__blk949_dn7)), ((((((8.0 * locals.var_temp__blk949_dn8) - (12.0 * locals.var_xi0s__blk1365_dn8)) * locals.var_temp__blk949) + (assign50230_e64863 * locals.var_temp__blk949_dn8)) * locals.var_temp__blk949) + (assign50230_e64865 * locals.var_temp__blk949_dn8)), ((((((8.0 * locals.var_temp__blk949_dn9) - (12.0 * locals.var_xi0s__blk1365_dn9)) * locals.var_temp__blk949) + (assign50230_e64863 * locals.var_temp__blk949_dn9)) * locals.var_temp__blk949) + (assign50230_e64865 * locals.var_temp__blk949_dn9)), );
            locals.var_xi2s__blk1367_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) {
            (locals.var_delta_1s__blk1368, locals.var_delta_1s__blk1368_dn4, locals.var_delta_1s__blk1368_dn6, locals.var_delta_1s__blk1368_dn7, locals.var_delta_1s__blk1368_dn8, locals.var_delta_1s__blk1368_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_delta_1s__blk1368_rv = 0.0;
        }

        let assign50250_e64880: f64 = if locals.var_x_s__blk1363 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1492 = assign50250_e64880;
        locals.var_guard1492_rv = 0.0;

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1492 != 0.0)) {
            let assign50260_e64889: f64 = (locals.var_x_s__blk1363).exp();
            (locals.var_delta_1s__blk1368, locals.var_delta_1s__blk1368_dn4, locals.var_delta_1s__blk1368_dn6, locals.var_delta_1s__blk1368_dn7, locals.var_delta_1s__blk1368_dn8, locals.var_delta_1s__blk1368_dn9, ) = (assign50260_e64889, (assign50260_e64889 * locals.var_x_s__blk1363_dn4), (assign50260_e64889 * locals.var_x_s__blk1363_dn6), (assign50260_e64889 * locals.var_x_s__blk1363_dn7), (assign50260_e64889 * locals.var_x_s__blk1363_dn8), (assign50260_e64889 * locals.var_x_s__blk1363_dn9), );
            locals.var_delta_1s__blk1368_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1492 != 0.0)) {
            let assign50270_e64901: f64 = (1.0 / locals.var_delta_1s__blk1368);
            (locals.var_es__blk1369, locals.var_es__blk1369_dn4, locals.var_es__blk1369_dn6, locals.var_es__blk1369_dn7, locals.var_es__blk1369_dn8, locals.var_es__blk1369_dn9, ) = (assign50270_e64901, (-(locals.var_delta_1s__blk1368_dn4 / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368))), (-(locals.var_delta_1s__blk1368_dn6 / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368))), (-(locals.var_delta_1s__blk1368_dn7 / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368))), (-(locals.var_delta_1s__blk1368_dn8 / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368))), (-(locals.var_delta_1s__blk1368_dn9 / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368))), );
            locals.var_es__blk1369_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1492 != 0.0)) {
            let assign50280_e64913: f64 = (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368);
            (locals.var_delta_1s__blk1368, locals.var_delta_1s__blk1368_dn4, locals.var_delta_1s__blk1368_dn6, locals.var_delta_1s__blk1368_dn7, locals.var_delta_1s__blk1368_dn8, locals.var_delta_1s__blk1368_dn9, ) = (assign50280_e64913, ((locals.var_delta_ns__blk1364_dn4 * locals.var_delta_1s__blk1368) + (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn4)), ((locals.var_delta_ns__blk1364_dn6 * locals.var_delta_1s__blk1368) + (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn6)), ((locals.var_delta_ns__blk1364_dn7 * locals.var_delta_1s__blk1368) + (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn7)), ((locals.var_delta_ns__blk1364_dn8 * locals.var_delta_1s__blk1368) + (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn8)), ((locals.var_delta_ns__blk1364_dn9 * locals.var_delta_1s__blk1368) + (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn9)), );
            locals.var_delta_1s__blk1368_rv = 0.0;
        }

        let assign50290_e64919: f64 = (locals.var_xn_s__blk1349 - 230.25850929940458);
        let assign50290_e64920: f64 = if locals.var_x_s__blk1363 > assign50290_e64919 { 1.0 } else { 0.0 };
        locals.var_guard1493 = assign50290_e64920;
        locals.var_guard1493_rv = 0.0;

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 != 0.0)) {
            let assign50300_e64933: f64 = (locals.var_x_s__blk1363 - locals.var_xn_s__blk1349);
            let assign50300_e64934: f64 = (assign50300_e64933).exp();
            (locals.var_delta_1s__blk1368, locals.var_delta_1s__blk1368_dn4, locals.var_delta_1s__blk1368_dn6, locals.var_delta_1s__blk1368_dn7, locals.var_delta_1s__blk1368_dn8, locals.var_delta_1s__blk1368_dn9, ) = (assign50300_e64934, (assign50300_e64934 * (locals.var_x_s__blk1363_dn4 - locals.var_xn_s__blk1349_dn4)), (assign50300_e64934 * (locals.var_x_s__blk1363_dn6 - locals.var_xn_s__blk1349_dn6)), (assign50300_e64934 * (locals.var_x_s__blk1363_dn7 - locals.var_xn_s__blk1349_dn7)), (assign50300_e64934 * (locals.var_x_s__blk1363_dn8 - locals.var_xn_s__blk1349_dn8)), (assign50300_e64934 * (locals.var_x_s__blk1363_dn9 - locals.var_xn_s__blk1349_dn9)), );
            locals.var_delta_1s__blk1368_rv = 0.0;
        }

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 != 0.0)) {
            let assign50310_e64949: f64 = (locals.var_delta_ns__blk1364 / locals.var_delta_1s__blk1368);
            (locals.var_es__blk1369, locals.var_es__blk1369_dn4, locals.var_es__blk1369_dn6, locals.var_es__blk1369_dn7, locals.var_es__blk1369_dn8, locals.var_es__blk1369_dn9, ) = (assign50310_e64949, (((locals.var_delta_ns__blk1364_dn4 * locals.var_delta_1s__blk1368) - (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn4)) / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368)), (((locals.var_delta_ns__blk1364_dn6 * locals.var_delta_1s__blk1368) - (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn6)) / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368)), (((locals.var_delta_ns__blk1364_dn7 * locals.var_delta_1s__blk1368) - (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn7)) / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368)), (((locals.var_delta_ns__blk1364_dn8 * locals.var_delta_1s__blk1368) - (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn8)) / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368)), (((locals.var_delta_ns__blk1364_dn9 * locals.var_delta_1s__blk1368) - (locals.var_delta_ns__blk1364 * locals.var_delta_1s__blk1368_dn9)) / (locals.var_delta_1s__blk1368 * locals.var_delta_1s__blk1368)), );
            locals.var_es__blk1369_rv = 0.0;
        }

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 == 0.0)) {
            let assign50320_e64967: f64 = (locals.var_xn_s__blk1349 - locals.var_x_s__blk1363);
            let assign50320_e64969: f64 = (assign50320_e64967 - 230.25850929940458);
            let assign50320_e64974: f64 = (locals.var_xn_s__blk1349 - locals.var_x_s__blk1363);
            let assign50320_e64976: f64 = (assign50320_e64974 - 230.25850929940458);
            let assign50320_e64980: f64 = (locals.var_xn_s__blk1349 - locals.var_x_s__blk1363);
            let assign50320_e64982: f64 = (assign50320_e64980 - 230.25850929940458);
            let assign50320_e64984: f64 = (assign50320_e64982 * 0.3333333333333333);
            let assign50320_e64985: f64 = (1.0 + assign50320_e64984);
            let assign50320_e64986: f64 = (assign50320_e64976 * assign50320_e64985);
            let assign50320_e64987: f64 = (0.5 * assign50320_e64986);
            let assign50320_e64988: f64 = (1.0 + assign50320_e64987);
            let assign50320_e64989: f64 = (assign50320_e64969 * assign50320_e64988);
            let assign50320_e64990: f64 = (1.0 + assign50320_e64989);
            let assign50320_e64991: f64 = (1e-100 / assign50320_e64990);
            (locals.var_delta_1s__blk1368, locals.var_delta_1s__blk1368_dn4, locals.var_delta_1s__blk1368_dn6, locals.var_delta_1s__blk1368_dn7, locals.var_delta_1s__blk1368_dn8, locals.var_delta_1s__blk1368_dn9, ) = (assign50320_e64991, (-((1e-100 * (((locals.var_xn_s__blk1349_dn4 - locals.var_x_s__blk1363_dn4) * assign50320_e64988) + (assign50320_e64969 * (0.5 * (((locals.var_xn_s__blk1349_dn4 - locals.var_x_s__blk1363_dn4) * assign50320_e64985) + (assign50320_e64976 * ((locals.var_xn_s__blk1349_dn4 - locals.var_x_s__blk1363_dn4) * 0.3333333333333333))))))) / (assign50320_e64990 * assign50320_e64990))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn6 - locals.var_x_s__blk1363_dn6) * assign50320_e64988) + (assign50320_e64969 * (0.5 * (((locals.var_xn_s__blk1349_dn6 - locals.var_x_s__blk1363_dn6) * assign50320_e64985) + (assign50320_e64976 * ((locals.var_xn_s__blk1349_dn6 - locals.var_x_s__blk1363_dn6) * 0.3333333333333333))))))) / (assign50320_e64990 * assign50320_e64990))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn7 - locals.var_x_s__blk1363_dn7) * assign50320_e64988) + (assign50320_e64969 * (0.5 * (((locals.var_xn_s__blk1349_dn7 - locals.var_x_s__blk1363_dn7) * assign50320_e64985) + (assign50320_e64976 * ((locals.var_xn_s__blk1349_dn7 - locals.var_x_s__blk1363_dn7) * 0.3333333333333333))))))) / (assign50320_e64990 * assign50320_e64990))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn8 - locals.var_x_s__blk1363_dn8) * assign50320_e64988) + (assign50320_e64969 * (0.5 * (((locals.var_xn_s__blk1349_dn8 - locals.var_x_s__blk1363_dn8) * assign50320_e64985) + (assign50320_e64976 * ((locals.var_xn_s__blk1349_dn8 - locals.var_x_s__blk1363_dn8) * 0.3333333333333333))))))) / (assign50320_e64990 * assign50320_e64990))), (-((1e-100 * (((locals.var_xn_s__blk1349_dn9 - locals.var_x_s__blk1363_dn9) * assign50320_e64988) + (assign50320_e64969 * (0.5 * (((locals.var_xn_s__blk1349_dn9 - locals.var_x_s__blk1363_dn9) * assign50320_e64985) + (assign50320_e64976 * ((locals.var_xn_s__blk1349_dn9 - locals.var_x_s__blk1363_dn9) * 0.3333333333333333))))))) / (assign50320_e64990 * assign50320_e64990))), );
            locals.var_delta_1s__blk1368_rv = 0.0;
        }

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 == 0.0)) {
            let assign50330_e65009: f64 = (locals.var_x_s__blk1363 - 230.25850929940458);
            let assign50330_e65014: f64 = (locals.var_x_s__blk1363 - 230.25850929940458);
            let assign50330_e65018: f64 = (locals.var_x_s__blk1363 - 230.25850929940458);
            let assign50330_e65020: f64 = (assign50330_e65018 * 0.3333333333333333);
            let assign50330_e65021: f64 = (1.0 + assign50330_e65020);
            let assign50330_e65022: f64 = (assign50330_e65014 * assign50330_e65021);
            let assign50330_e65023: f64 = (0.5 * assign50330_e65022);
            let assign50330_e65024: f64 = (1.0 + assign50330_e65023);
            let assign50330_e65025: f64 = (assign50330_e65009 * assign50330_e65024);
            let assign50330_e65026: f64 = (1.0 + assign50330_e65025);
            let assign50330_e65027: f64 = (1e-100 / assign50330_e65026);
            (locals.var_es__blk1369, locals.var_es__blk1369_dn4, locals.var_es__blk1369_dn6, locals.var_es__blk1369_dn7, locals.var_es__blk1369_dn8, locals.var_es__blk1369_dn9, ) = (assign50330_e65027, (-((1e-100 * ((locals.var_x_s__blk1363_dn4 * assign50330_e65024) + (assign50330_e65009 * (0.5 * ((locals.var_x_s__blk1363_dn4 * assign50330_e65021) + (assign50330_e65014 * (locals.var_x_s__blk1363_dn4 * 0.3333333333333333))))))) / (assign50330_e65026 * assign50330_e65026))), (-((1e-100 * ((locals.var_x_s__blk1363_dn6 * assign50330_e65024) + (assign50330_e65009 * (0.5 * ((locals.var_x_s__blk1363_dn6 * assign50330_e65021) + (assign50330_e65014 * (locals.var_x_s__blk1363_dn6 * 0.3333333333333333))))))) / (assign50330_e65026 * assign50330_e65026))), (-((1e-100 * ((locals.var_x_s__blk1363_dn7 * assign50330_e65024) + (assign50330_e65009 * (0.5 * ((locals.var_x_s__blk1363_dn7 * assign50330_e65021) + (assign50330_e65014 * (locals.var_x_s__blk1363_dn7 * 0.3333333333333333))))))) / (assign50330_e65026 * assign50330_e65026))), (-((1e-100 * ((locals.var_x_s__blk1363_dn8 * assign50330_e65024) + (assign50330_e65009 * (0.5 * ((locals.var_x_s__blk1363_dn8 * assign50330_e65021) + (assign50330_e65014 * (locals.var_x_s__blk1363_dn8 * 0.3333333333333333))))))) / (assign50330_e65026 * assign50330_e65026))), (-((1e-100 * ((locals.var_x_s__blk1363_dn9 * assign50330_e65024) + (assign50330_e65009 * (0.5 * ((locals.var_x_s__blk1363_dn9 * assign50330_e65021) + (assign50330_e65014 * (locals.var_x_s__blk1363_dn9 * 0.3333333333333333))))))) / (assign50330_e65026 * assign50330_e65026))), );
            locals.var_es__blk1369_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) {
            let assign50340_e65039: f64 = (locals.var_x_s__blk1363 + 1.0);
            let assign50340_e65041: f64 = (assign50340_e65039 + locals.var_xi0s__blk1365);
            let assign50340_e65042: f64 = (locals.var_delta_ns__blk1364 * assign50340_e65041);
            let assign50340_e65043: f64 = (locals.var_delta_1s__blk1368 - assign50340_e65042);
            (locals.var_ds__blk1370, locals.var_ds__blk1370_dn4, locals.var_ds__blk1370_dn6, locals.var_ds__blk1370_dn7, locals.var_ds__blk1370_dn8, locals.var_ds__blk1370_dn9, ) = (assign50340_e65043, (locals.var_delta_1s__blk1368_dn4 - ((locals.var_delta_ns__blk1364_dn4 * assign50340_e65041) + (locals.var_delta_ns__blk1364 * (locals.var_x_s__blk1363_dn4 + locals.var_xi0s__blk1365_dn4)))), (locals.var_delta_1s__blk1368_dn6 - ((locals.var_delta_ns__blk1364_dn6 * assign50340_e65041) + (locals.var_delta_ns__blk1364 * (locals.var_x_s__blk1363_dn6 + locals.var_xi0s__blk1365_dn6)))), (locals.var_delta_1s__blk1368_dn7 - ((locals.var_delta_ns__blk1364_dn7 * assign50340_e65041) + (locals.var_delta_ns__blk1364 * (locals.var_x_s__blk1363_dn7 + locals.var_xi0s__blk1365_dn7)))), (locals.var_delta_1s__blk1368_dn8 - ((locals.var_delta_ns__blk1364_dn8 * assign50340_e65041) + (locals.var_delta_ns__blk1364 * (locals.var_x_s__blk1363_dn8 + locals.var_xi0s__blk1365_dn8)))), (locals.var_delta_1s__blk1368_dn9 - ((locals.var_delta_ns__blk1364_dn9 * assign50340_e65041) + (locals.var_delta_ns__blk1364 * (locals.var_x_s__blk1363_dn9 + locals.var_xi0s__blk1365_dn9)))), );
            locals.var_ds__blk1370_rv = 0.0;
        }

        let assign50350_e65048: f64 = if locals.var_x_s__blk1363 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1494 = assign50350_e65048;
        locals.var_guard1494_rv = 0.0;

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 != 0.0)) {
            let assign50360_e65059: f64 = (locals.var_x_s__blk1363 * locals.var_x_s__blk1363);
            let assign50360_e65066: f64 = (0.25 * locals.var_x_s__blk1363);
            let assign50360_e65067: f64 = (1.0 - assign50360_e65066);
            let assign50360_e65068: f64 = (locals.var_x_s__blk1363 * assign50360_e65067);
            let assign50360_e65069: f64 = (0.3333333333333333 * assign50360_e65068);
            let assign50360_e65070: f64 = (1.0 - assign50360_e65069);
            let assign50360_e65071: f64 = (assign50360_e65059 * assign50360_e65070);
            let assign50360_e65072: f64 = (0.5 * assign50360_e65071);
            (locals.var_ps__blk1371, locals.var_ps__blk1371_dn4, locals.var_ps__blk1371_dn6, locals.var_ps__blk1371_dn7, locals.var_ps__blk1371_dn8, locals.var_ps__blk1371_dn9, ) = (assign50360_e65072, (0.5 * ((((locals.var_x_s__blk1363_dn4 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn4)) * assign50360_e65070) + (assign50360_e65059 * (-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn4 * assign50360_e65067) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn4))))))))), (0.5 * ((((locals.var_x_s__blk1363_dn6 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn6)) * assign50360_e65070) + (assign50360_e65059 * (-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn6 * assign50360_e65067) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn6))))))))), (0.5 * ((((locals.var_x_s__blk1363_dn7 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn7)) * assign50360_e65070) + (assign50360_e65059 * (-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn7 * assign50360_e65067) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn7))))))))), (0.5 * ((((locals.var_x_s__blk1363_dn8 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn8)) * assign50360_e65070) + (assign50360_e65059 * (-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn8 * assign50360_e65067) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn8))))))))), (0.5 * ((((locals.var_x_s__blk1363_dn9 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn9)) * assign50360_e65070) + (assign50360_e65059 * (-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn9 * assign50360_e65067) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn9))))))))), );
            locals.var_ps__blk1371_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 != 0.0)) {
            let assign50370_e65085: f64 = (locals.var_delta_ns__blk1364 * locals.var_x_s__blk1363);
            let assign50370_e65087: f64 = (assign50370_e65085 * locals.var_x_s__blk1363);
            let assign50370_e65089: f64 = (assign50370_e65087 * locals.var_x_s__blk1363);
            let assign50370_e65093: f64 = (1.75 * locals.var_x_s__blk1363);
            let assign50370_e65094: f64 = (1.0 + assign50370_e65093);
            let assign50370_e65095: f64 = (assign50370_e65089 * assign50370_e65094);
            let assign50370_e65096: f64 = (0.16666666666666666 * assign50370_e65095);
            (locals.var_ds__blk1370, locals.var_ds__blk1370_dn4, locals.var_ds__blk1370_dn6, locals.var_ds__blk1370_dn7, locals.var_ds__blk1370_dn8, locals.var_ds__blk1370_dn9, ) = (assign50370_e65096, (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1364_dn4 * locals.var_x_s__blk1363) + (locals.var_delta_ns__blk1364 * locals.var_x_s__blk1363_dn4)) * locals.var_x_s__blk1363) + (assign50370_e65085 * locals.var_x_s__blk1363_dn4)) * locals.var_x_s__blk1363) + (assign50370_e65087 * locals.var_x_s__blk1363_dn4)) * assign50370_e65094) + (assign50370_e65089 * (1.75 * locals.var_x_s__blk1363_dn4)))), (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1364_dn6 * locals.var_x_s__blk1363) + (locals.var_delta_ns__blk1364 * locals.var_x_s__blk1363_dn6)) * locals.var_x_s__blk1363) + (assign50370_e65085 * locals.var_x_s__blk1363_dn6)) * locals.var_x_s__blk1363) + (assign50370_e65087 * locals.var_x_s__blk1363_dn6)) * assign50370_e65094) + (assign50370_e65089 * (1.75 * locals.var_x_s__blk1363_dn6)))), (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1364_dn7 * locals.var_x_s__blk1363) + (locals.var_delta_ns__blk1364 * locals.var_x_s__blk1363_dn7)) * locals.var_x_s__blk1363) + (assign50370_e65085 * locals.var_x_s__blk1363_dn7)) * locals.var_x_s__blk1363) + (assign50370_e65087 * locals.var_x_s__blk1363_dn7)) * assign50370_e65094) + (assign50370_e65089 * (1.75 * locals.var_x_s__blk1363_dn7)))), (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1364_dn8 * locals.var_x_s__blk1363) + (locals.var_delta_ns__blk1364 * locals.var_x_s__blk1363_dn8)) * locals.var_x_s__blk1363) + (assign50370_e65085 * locals.var_x_s__blk1363_dn8)) * locals.var_x_s__blk1363) + (assign50370_e65087 * locals.var_x_s__blk1363_dn8)) * assign50370_e65094) + (assign50370_e65089 * (1.75 * locals.var_x_s__blk1363_dn8)))), (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1364_dn9 * locals.var_x_s__blk1363) + (locals.var_delta_ns__blk1364 * locals.var_x_s__blk1363_dn9)) * locals.var_x_s__blk1363) + (assign50370_e65085 * locals.var_x_s__blk1363_dn9)) * locals.var_x_s__blk1363) + (assign50370_e65087 * locals.var_x_s__blk1363_dn9)) * assign50370_e65094) + (assign50370_e65089 * (1.75 * locals.var_x_s__blk1363_dn9)))), );
            locals.var_ds__blk1370_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 != 0.0)) {
            let assign50380_e65112: f64 = (0.25 * locals.var_x_s__blk1363);
            let assign50380_e65113: f64 = (1.0 - assign50380_e65112);
            let assign50380_e65114: f64 = (locals.var_x_s__blk1363 * assign50380_e65113);
            let assign50380_e65115: f64 = (0.3333333333333333 * assign50380_e65114);
            let assign50380_e65116: f64 = (1.0 - assign50380_e65115);
            let assign50380_e65117: f64 = (assign50380_e65116).sqrt();
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign50380_e65117, ((-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn4 * assign50380_e65113) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn4)))))) / (2.0 * assign50380_e65117)), ((-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn6 * assign50380_e65113) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn6)))))) / (2.0 * assign50380_e65117)), ((-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn7 * assign50380_e65113) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn7)))))) / (2.0 * assign50380_e65117)), ((-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn8 * assign50380_e65113) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn8)))))) / (2.0 * assign50380_e65117)), ((-(0.3333333333333333 * ((locals.var_x_s__blk1363_dn9 * assign50380_e65113) + (locals.var_x_s__blk1363 * (-(0.25 * locals.var_x_s__blk1363_dn9)))))) / (2.0 * assign50380_e65117)), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 != 0.0)) {
            let assign50390_e65130: f64 = (locals.var_x_s__blk1363 * locals.var_temp__blk949);
            let assign50390_e65131: f64 = (0.7071067811865475 * assign50390_e65130);
            (locals.var_sqs__blk1372, locals.var_sqs__blk1372_dn4, locals.var_sqs__blk1372_dn6, locals.var_sqs__blk1372_dn7, locals.var_sqs__blk1372_dn8, locals.var_sqs__blk1372_dn9, ) = (assign50390_e65131, (0.7071067811865475 * ((locals.var_x_s__blk1363_dn4 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn4))), (0.7071067811865475 * ((locals.var_x_s__blk1363_dn6 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn6))), (0.7071067811865475 * ((locals.var_x_s__blk1363_dn7 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn7))), (0.7071067811865475 * ((locals.var_x_s__blk1363_dn8 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn8))), (0.7071067811865475 * ((locals.var_x_s__blk1363_dn9 * locals.var_temp__blk949) + (locals.var_x_s__blk1363 * locals.var_temp__blk949_dn9))), );
            locals.var_sqs__blk1372_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 != 0.0)) {
            let assign50400_e65147: f64 = (0.5 * locals.var_x_s__blk1363);
            let assign50400_e65148: f64 = (1.0 - assign50400_e65147);
            let assign50400_e65152: f64 = (locals.var_x_s__blk1363 * locals.var_x_s__blk1363);
            let assign50400_e65153: f64 = (0.16666666666666666 * assign50400_e65152);
            let assign50400_e65154: f64 = (assign50400_e65148 + assign50400_e65153);
            let assign50400_e65155: f64 = (locals.var_gf__blk1324 * assign50400_e65154);
            let assign50400_e65157: f64 = (assign50400_e65155 / locals.var_temp__blk949);
            let assign50400_e65158: f64 = (0.7071067811865475 * assign50400_e65157);
            let assign50400_e65159: f64 = (1.0 + assign50400_e65158);
            (locals.var_alphas__blk1373, locals.var_alphas__blk1373_dn4, locals.var_alphas__blk1373_dn6, locals.var_alphas__blk1373_dn7, locals.var_alphas__blk1373_dn8, locals.var_alphas__blk1373_dn9, ) = (assign50400_e65159, (0.7071067811865475 * (((((locals.var_gf__blk1324_dn4 * assign50400_e65154) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_s__blk1363_dn4)) + (0.16666666666666666 * ((locals.var_x_s__blk1363_dn4 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn4)))))) * locals.var_temp__blk949) - (assign50400_e65155 * locals.var_temp__blk949_dn4)) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (0.7071067811865475 * (((((locals.var_gf__blk1324_dn6 * assign50400_e65154) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_s__blk1363_dn6)) + (0.16666666666666666 * ((locals.var_x_s__blk1363_dn6 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn6)))))) * locals.var_temp__blk949) - (assign50400_e65155 * locals.var_temp__blk949_dn6)) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (0.7071067811865475 * (((((locals.var_gf__blk1324_dn7 * assign50400_e65154) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_s__blk1363_dn7)) + (0.16666666666666666 * ((locals.var_x_s__blk1363_dn7 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn7)))))) * locals.var_temp__blk949) - (assign50400_e65155 * locals.var_temp__blk949_dn7)) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (0.7071067811865475 * (((((locals.var_gf__blk1324_dn8 * assign50400_e65154) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_s__blk1363_dn8)) + (0.16666666666666666 * ((locals.var_x_s__blk1363_dn8 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn8)))))) * locals.var_temp__blk949) - (assign50400_e65155 * locals.var_temp__blk949_dn8)) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (0.7071067811865475 * (((((locals.var_gf__blk1324_dn9 * assign50400_e65154) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_s__blk1363_dn9)) + (0.16666666666666666 * ((locals.var_x_s__blk1363_dn9 * locals.var_x_s__blk1363) + (locals.var_x_s__blk1363 * locals.var_x_s__blk1363_dn9)))))) * locals.var_temp__blk949) - (assign50400_e65155 * locals.var_temp__blk949_dn9)) / (locals.var_temp__blk949 * locals.var_temp__blk949))), );
            locals.var_alphas__blk1373_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 == 0.0)) {
            let assign50410_e65172: f64 = (locals.var_x_s__blk1363 - 1.0);
            let assign50410_e65174: f64 = (assign50410_e65172 + locals.var_es__blk1369);
            (locals.var_ps__blk1371, locals.var_ps__blk1371_dn4, locals.var_ps__blk1371_dn6, locals.var_ps__blk1371_dn7, locals.var_ps__blk1371_dn8, locals.var_ps__blk1371_dn9, ) = (assign50410_e65174, (locals.var_x_s__blk1363_dn4 + locals.var_es__blk1369_dn4), (locals.var_x_s__blk1363_dn6 + locals.var_es__blk1369_dn6), (locals.var_x_s__blk1363_dn7 + locals.var_es__blk1369_dn7), (locals.var_x_s__blk1363_dn8 + locals.var_es__blk1369_dn8), (locals.var_x_s__blk1363_dn9 + locals.var_es__blk1369_dn9), );
            locals.var_ps__blk1371_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_25(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 == 0.0)) {
            let assign50420_e65186: f64 = (locals.var_ps__blk1371).sqrt();
            (locals.var_sqs__blk1372, locals.var_sqs__blk1372_dn4, locals.var_sqs__blk1372_dn6, locals.var_sqs__blk1372_dn7, locals.var_sqs__blk1372_dn8, locals.var_sqs__blk1372_dn9, ) = (assign50420_e65186, (locals.var_ps__blk1371_dn4 / (2.0 * assign50420_e65186)), (locals.var_ps__blk1371_dn6 / (2.0 * assign50420_e65186)), (locals.var_ps__blk1371_dn7 / (2.0 * assign50420_e65186)), (locals.var_ps__blk1371_dn8 / (2.0 * assign50420_e65186)), (locals.var_ps__blk1371_dn9 / (2.0 * assign50420_e65186)), );
            locals.var_sqs__blk1372_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1494 == 0.0)) {
            let assign50430_e65202: f64 = (1.0 - locals.var_es__blk1369);
            let assign50430_e65203: f64 = (locals.var_gf__blk1324 * assign50430_e65202);
            let assign50430_e65205: f64 = (assign50430_e65203 / locals.var_sqs__blk1372);
            let assign50430_e65206: f64 = (0.5 * assign50430_e65205);
            let assign50430_e65207: f64 = (1.0 + assign50430_e65206);
            (locals.var_alphas__blk1373, locals.var_alphas__blk1373_dn4, locals.var_alphas__blk1373_dn6, locals.var_alphas__blk1373_dn7, locals.var_alphas__blk1373_dn8, locals.var_alphas__blk1373_dn9, ) = (assign50430_e65207, (0.5 * (((((locals.var_gf__blk1324_dn4 * assign50430_e65202) + (locals.var_gf__blk1324 * (-locals.var_es__blk1369_dn4))) * locals.var_sqs__blk1372) - (assign50430_e65203 * locals.var_sqs__blk1372_dn4)) / (locals.var_sqs__blk1372 * locals.var_sqs__blk1372))), (0.5 * (((((locals.var_gf__blk1324_dn6 * assign50430_e65202) + (locals.var_gf__blk1324 * (-locals.var_es__blk1369_dn6))) * locals.var_sqs__blk1372) - (assign50430_e65203 * locals.var_sqs__blk1372_dn6)) / (locals.var_sqs__blk1372 * locals.var_sqs__blk1372))), (0.5 * (((((locals.var_gf__blk1324_dn7 * assign50430_e65202) + (locals.var_gf__blk1324 * (-locals.var_es__blk1369_dn7))) * locals.var_sqs__blk1372) - (assign50430_e65203 * locals.var_sqs__blk1372_dn7)) / (locals.var_sqs__blk1372 * locals.var_sqs__blk1372))), (0.5 * (((((locals.var_gf__blk1324_dn8 * assign50430_e65202) + (locals.var_gf__blk1324 * (-locals.var_es__blk1369_dn8))) * locals.var_sqs__blk1372) - (assign50430_e65203 * locals.var_sqs__blk1372_dn8)) / (locals.var_sqs__blk1372 * locals.var_sqs__blk1372))), (0.5 * (((((locals.var_gf__blk1324_dn9 * assign50430_e65202) + (locals.var_gf__blk1324 * (-locals.var_es__blk1369_dn9))) * locals.var_sqs__blk1372) - (assign50430_e65203 * locals.var_sqs__blk1372_dn9)) / (locals.var_sqs__blk1372 * locals.var_sqs__blk1372))), );
            locals.var_alphas__blk1373_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) {
            let assign50440_e65218: f64 = (0.2 * locals.var_xcor_t);
            let assign50440_e65220: f64 = (assign50440_e65218 * locals.var_vsbx__blk1323);
            let assign50440_e65221: f64 = (1.0 + assign50440_e65220);
            let assign50440_e65225: f64 = (locals.var_xcor_t * locals.var_vsbx__blk1323);
            let assign50440_e65226: f64 = (1.0 + assign50440_e65225);
            let assign50440_e65227: f64 = (assign50440_e65221 / assign50440_e65226);
            (locals.var_rxcor__blk1374, locals.var_rxcor__blk1374_dn4, locals.var_rxcor__blk1374_dn6, locals.var_rxcor__blk1374_dn7, locals.var_rxcor__blk1374_dn8, locals.var_rxcor__blk1374_dn9, ) = (assign50440_e65227, ((((((0.2 * locals.var_xcor_t_dn4) * locals.var_vsbx__blk1323) + (assign50440_e65218 * locals.var_vsbx__blk1323_dn4)) * assign50440_e65226) - (assign50440_e65221 * ((locals.var_xcor_t_dn4 * locals.var_vsbx__blk1323) + (locals.var_xcor_t * locals.var_vsbx__blk1323_dn4)))) / (assign50440_e65226 * assign50440_e65226)), ((((assign50440_e65218 * locals.var_vsbx__blk1323_dn6) * assign50440_e65226) - (assign50440_e65221 * (locals.var_xcor_t * locals.var_vsbx__blk1323_dn6))) / (assign50440_e65226 * assign50440_e65226)), ((((assign50440_e65218 * locals.var_vsbx__blk1323_dn7) * assign50440_e65226) - (assign50440_e65221 * (locals.var_xcor_t * locals.var_vsbx__blk1323_dn7))) / (assign50440_e65226 * assign50440_e65226)), ((((assign50440_e65218 * locals.var_vsbx__blk1323_dn8) * assign50440_e65226) - (assign50440_e65221 * (locals.var_xcor_t * locals.var_vsbx__blk1323_dn8))) / (assign50440_e65226 * assign50440_e65226)), ((((assign50440_e65218 * locals.var_vsbx__blk1323_dn9) * assign50440_e65226) - (assign50440_e65221 * (locals.var_xcor_t * locals.var_vsbx__blk1323_dn9))) / (assign50440_e65226 * assign50440_e65226)), );
            locals.var_rxcor__blk1374_rv = 0.0;
        }

        let assign50450_e65232: f64 = if locals.var_ds__blk1370 > 1e-100 { 1.0 } else { 0.0 };
        locals.var_guard1495 = assign50450_e65232;
        locals.var_guard1495_rv = 0.0;

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
            let assign50460_e65243: f64 = (locals.var_ps__blk1371 + locals.var_ds__blk1370);
            let assign50460_e65244: f64 = (assign50460_e65243).sqrt();
            let assign50460_e65245: f64 = (locals.var_gf__blk1324 * assign50460_e65244);
            (locals.var_xgs__blk1375, locals.var_xgs__blk1375_dn4, locals.var_xgs__blk1375_dn6, locals.var_xgs__blk1375_dn7, locals.var_xgs__blk1375_dn8, locals.var_xgs__blk1375_dn9, ) = (assign50460_e65245, ((locals.var_gf__blk1324_dn4 * assign50460_e65244) + (locals.var_gf__blk1324 * ((locals.var_ps__blk1371_dn4 + locals.var_ds__blk1370_dn4) / (2.0 * assign50460_e65244)))), ((locals.var_gf__blk1324_dn6 * assign50460_e65244) + (locals.var_gf__blk1324 * ((locals.var_ps__blk1371_dn6 + locals.var_ds__blk1370_dn6) / (2.0 * assign50460_e65244)))), ((locals.var_gf__blk1324_dn7 * assign50460_e65244) + (locals.var_gf__blk1324 * ((locals.var_ps__blk1371_dn7 + locals.var_ds__blk1370_dn7) / (2.0 * assign50460_e65244)))), ((locals.var_gf__blk1324_dn8 * assign50460_e65244) + (locals.var_gf__blk1324 * ((locals.var_ps__blk1371_dn8 + locals.var_ds__blk1370_dn8) / (2.0 * assign50460_e65244)))), ((locals.var_gf__blk1324_dn9 * assign50460_e65244) + (locals.var_gf__blk1324 * ((locals.var_ps__blk1371_dn9 + locals.var_ds__blk1370_dn9) / (2.0 * assign50460_e65244)))), );
            locals.var_xgs__blk1375_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
            let assign50470_e65257: f64 = (locals.var_gf2__blk1325 * locals.var_ds__blk1370);
            let assign50470_e65259: f64 = (assign50470_e65257 * locals.var_phit1__blk1339);
            let assign50470_e65263: f64 = (locals.var_gf__blk1324 * locals.var_sqs__blk1372);
            let assign50470_e65264: f64 = (locals.var_xgs__blk1375 + assign50470_e65263);
            let assign50470_e65265: f64 = (assign50470_e65259 / assign50470_e65264);
            (locals.var_qis__blk1376, locals.var_qis__blk1376_dn4, locals.var_qis__blk1376_dn6, locals.var_qis__blk1376_dn7, locals.var_qis__blk1376_dn8, locals.var_qis__blk1376_dn9, ) = (assign50470_e65265, (((((((locals.var_gf2__blk1325_dn4 * locals.var_ds__blk1370) + (locals.var_gf2__blk1325 * locals.var_ds__blk1370_dn4)) * locals.var_phit1__blk1339) + (assign50470_e65257 * locals.var_phit1__blk1339_dn4)) * assign50470_e65264) - (assign50470_e65259 * (locals.var_xgs__blk1375_dn4 + ((locals.var_gf__blk1324_dn4 * locals.var_sqs__blk1372) + (locals.var_gf__blk1324 * locals.var_sqs__blk1372_dn4))))) / (assign50470_e65264 * assign50470_e65264)), (((((((locals.var_gf2__blk1325_dn6 * locals.var_ds__blk1370) + (locals.var_gf2__blk1325 * locals.var_ds__blk1370_dn6)) * locals.var_phit1__blk1339) + (assign50470_e65257 * locals.var_phit1__blk1339_dn6)) * assign50470_e65264) - (assign50470_e65259 * (locals.var_xgs__blk1375_dn6 + ((locals.var_gf__blk1324_dn6 * locals.var_sqs__blk1372) + (locals.var_gf__blk1324 * locals.var_sqs__blk1372_dn6))))) / (assign50470_e65264 * assign50470_e65264)), (((((((locals.var_gf2__blk1325_dn7 * locals.var_ds__blk1370) + (locals.var_gf2__blk1325 * locals.var_ds__blk1370_dn7)) * locals.var_phit1__blk1339) + (assign50470_e65257 * locals.var_phit1__blk1339_dn7)) * assign50470_e65264) - (assign50470_e65259 * (locals.var_xgs__blk1375_dn7 + ((locals.var_gf__blk1324_dn7 * locals.var_sqs__blk1372) + (locals.var_gf__blk1324 * locals.var_sqs__blk1372_dn7))))) / (assign50470_e65264 * assign50470_e65264)), (((((((locals.var_gf2__blk1325_dn8 * locals.var_ds__blk1370) + (locals.var_gf2__blk1325 * locals.var_ds__blk1370_dn8)) * locals.var_phit1__blk1339) + (assign50470_e65257 * locals.var_phit1__blk1339_dn8)) * assign50470_e65264) - (assign50470_e65259 * (locals.var_xgs__blk1375_dn8 + ((locals.var_gf__blk1324_dn8 * locals.var_sqs__blk1372) + (locals.var_gf__blk1324 * locals.var_sqs__blk1372_dn8))))) / (assign50470_e65264 * assign50470_e65264)), (((((((locals.var_gf2__blk1325_dn9 * locals.var_ds__blk1370) + (locals.var_gf2__blk1325 * locals.var_ds__blk1370_dn9)) * locals.var_phit1__blk1339) + (assign50470_e65257 * locals.var_phit1__blk1339_dn9)) * assign50470_e65264) - (assign50470_e65259 * (locals.var_xgs__blk1375_dn9 + ((locals.var_gf__blk1324_dn9 * locals.var_sqs__blk1372) + (locals.var_gf__blk1324 * locals.var_sqs__blk1372_dn9))))) / (assign50470_e65264 * assign50470_e65264)), );
            locals.var_qis__blk1376_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
            let assign50480_e65277: f64 = (locals.var_sqs__blk1372 * locals.var_gf__blk1324);
            let assign50480_e65279: f64 = (assign50480_e65277 * locals.var_phit1__blk1339);
            (locals.var_qbs__blk1377, locals.var_qbs__blk1377_dn4, locals.var_qbs__blk1377_dn6, locals.var_qbs__blk1377_dn7, locals.var_qbs__blk1377_dn8, locals.var_qbs__blk1377_dn9, ) = (assign50480_e65279, ((((locals.var_sqs__blk1372_dn4 * locals.var_gf__blk1324) + (locals.var_sqs__blk1372 * locals.var_gf__blk1324_dn4)) * locals.var_phit1__blk1339) + (assign50480_e65277 * locals.var_phit1__blk1339_dn4)), ((((locals.var_sqs__blk1372_dn6 * locals.var_gf__blk1324) + (locals.var_sqs__blk1372 * locals.var_gf__blk1324_dn6)) * locals.var_phit1__blk1339) + (assign50480_e65277 * locals.var_phit1__blk1339_dn6)), ((((locals.var_sqs__blk1372_dn7 * locals.var_gf__blk1324) + (locals.var_sqs__blk1372 * locals.var_gf__blk1324_dn7)) * locals.var_phit1__blk1339) + (assign50480_e65277 * locals.var_phit1__blk1339_dn7)), ((((locals.var_sqs__blk1372_dn8 * locals.var_gf__blk1324) + (locals.var_sqs__blk1372 * locals.var_gf__blk1324_dn8)) * locals.var_phit1__blk1339) + (assign50480_e65277 * locals.var_phit1__blk1339_dn8)), ((((locals.var_sqs__blk1372_dn9 * locals.var_gf__blk1324) + (locals.var_sqs__blk1372 * locals.var_gf__blk1324_dn9)) * locals.var_phit1__blk1339) + (assign50480_e65277 * locals.var_phit1__blk1339_dn9)), );
            locals.var_qbs__blk1377_rv = 0.0;
        }

        let assign50490_e65284: f64 = if locals.var_rsb_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1496 = assign50490_e65284;
        locals.var_guard1496_rv = 0.0;

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1496 != 0.0)) {
            let assign50500_e65298: f64 = (locals.var_rsb_i * locals.var_vsbx__blk1323);
            let assign50500_e65299: f64 = (1.0 - assign50500_e65298);
            let assign50500_e65300: f64 = (1.0 / assign50500_e65299);
            (locals.var_rhob__blk1378, locals.var_rhob__blk1378_dn4, locals.var_rhob__blk1378_dn6, locals.var_rhob__blk1378_dn7, locals.var_rhob__blk1378_dn8, locals.var_rhob__blk1378_dn9, ) = (assign50500_e65300, (-((-(locals.var_rsb_i * locals.var_vsbx__blk1323_dn4)) / (assign50500_e65299 * assign50500_e65299))), (-((-(locals.var_rsb_i * locals.var_vsbx__blk1323_dn6)) / (assign50500_e65299 * assign50500_e65299))), (-((-(locals.var_rsb_i * locals.var_vsbx__blk1323_dn7)) / (assign50500_e65299 * assign50500_e65299))), (-((-(locals.var_rsb_i * locals.var_vsbx__blk1323_dn8)) / (assign50500_e65299 * assign50500_e65299))), (-((-(locals.var_rsb_i * locals.var_vsbx__blk1323_dn9)) / (assign50500_e65299 * assign50500_e65299))), );
            locals.var_rhob__blk1378_rv = 0.0;
        }

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1496 == 0.0)) {
            let assign50510_e65316: f64 = (locals.var_rsb_i * locals.var_vsbx__blk1323);
            let assign50510_e65317: f64 = (1.0 + assign50510_e65316);
            (locals.var_rhob__blk1378, locals.var_rhob__blk1378_dn4, locals.var_rhob__blk1378_dn6, locals.var_rhob__blk1378_dn7, locals.var_rhob__blk1378_dn8, locals.var_rhob__blk1378_dn9, ) = (assign50510_e65317, (locals.var_rsb_i * locals.var_vsbx__blk1323_dn4), (locals.var_rsb_i * locals.var_vsbx__blk1323_dn6), (locals.var_rsb_i * locals.var_vsbx__blk1323_dn7), (locals.var_rsb_i * locals.var_vsbx__blk1323_dn8), (locals.var_rsb_i * locals.var_vsbx__blk1323_dn9), );
            locals.var_rhob__blk1378_rv = 0.0;
        }

        let assign50520_e65322: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1497 = assign50520_e65322;
        locals.var_guard1497_rv = 0.0;

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1497 != 0.0)) {
            let assign50530_e65335: f64 = (locals.var_rsg_i * locals.var_qis__blk1376);
            let assign50530_e65336: f64 = (1.0 - assign50530_e65335);
            (locals.var_rhog__blk1379, locals.var_rhog__blk1379_dn4, locals.var_rhog__blk1379_dn6, locals.var_rhog__blk1379_dn7, locals.var_rhog__blk1379_dn8, locals.var_rhog__blk1379_dn9, ) = (assign50530_e65336, (-(locals.var_rsg_i * locals.var_qis__blk1376_dn4)), (-(locals.var_rsg_i * locals.var_qis__blk1376_dn6)), (-(locals.var_rsg_i * locals.var_qis__blk1376_dn7)), (-(locals.var_rsg_i * locals.var_qis__blk1376_dn8)), (-(locals.var_rsg_i * locals.var_qis__blk1376_dn9)), );
            locals.var_rhog__blk1379_rv = 0.0;
        }

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1497 == 0.0)) {
            let assign50540_e65353: f64 = (locals.var_rsg_i * locals.var_qis__blk1376);
            let assign50540_e65354: f64 = (1.0 + assign50540_e65353);
            let assign50540_e65355: f64 = (1.0 / assign50540_e65354);
            (locals.var_rhog__blk1379, locals.var_rhog__blk1379_dn4, locals.var_rhog__blk1379_dn6, locals.var_rhog__blk1379_dn7, locals.var_rhog__blk1379_dn8, locals.var_rhog__blk1379_dn9, ) = (assign50540_e65355, (-((locals.var_rsg_i * locals.var_qis__blk1376_dn4) / (assign50540_e65354 * assign50540_e65354))), (-((locals.var_rsg_i * locals.var_qis__blk1376_dn6) / (assign50540_e65354 * assign50540_e65354))), (-((locals.var_rsg_i * locals.var_qis__blk1376_dn7) / (assign50540_e65354 * assign50540_e65354))), (-((locals.var_rsg_i * locals.var_qis__blk1376_dn8) / (assign50540_e65354 * assign50540_e65354))), (-((locals.var_rsg_i * locals.var_qis__blk1376_dn9) / (assign50540_e65354 * assign50540_e65354))), );
            locals.var_rhog__blk1379_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
            let assign50550_e65367: f64 = (locals.var_ther_i * locals.var_rhob__blk1378);
            let assign50550_e65369: f64 = (assign50550_e65367 * locals.var_rhog__blk1379);
            let assign50550_e65371: f64 = (assign50550_e65369 * locals.var_qis__blk1376);
            (locals.var_gr__blk1380, locals.var_gr__blk1380_dn4, locals.var_gr__blk1380_dn6, locals.var_gr__blk1380_dn7, locals.var_gr__blk1380_dn8, locals.var_gr__blk1380_dn9, ) = (assign50550_e65371, ((((((locals.var_ther_i_dn4 * locals.var_rhob__blk1378) + (locals.var_ther_i * locals.var_rhob__blk1378_dn4)) * locals.var_rhog__blk1379) + (assign50550_e65367 * locals.var_rhog__blk1379_dn4)) * locals.var_qis__blk1376) + (assign50550_e65369 * locals.var_qis__blk1376_dn4)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn6) * locals.var_rhog__blk1379) + (assign50550_e65367 * locals.var_rhog__blk1379_dn6)) * locals.var_qis__blk1376) + (assign50550_e65369 * locals.var_qis__blk1376_dn6)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn7) * locals.var_rhog__blk1379) + (assign50550_e65367 * locals.var_rhog__blk1379_dn7)) * locals.var_qis__blk1376) + (assign50550_e65369 * locals.var_qis__blk1376_dn7)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn8) * locals.var_rhog__blk1379) + (assign50550_e65367 * locals.var_rhog__blk1379_dn8)) * locals.var_qis__blk1376) + (assign50550_e65369 * locals.var_qis__blk1376_dn8)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn9) * locals.var_rhog__blk1379) + (assign50550_e65367 * locals.var_rhog__blk1379_dn9)) * locals.var_qis__blk1376) + (assign50550_e65369 * locals.var_qis__blk1376_dn9)), );
            locals.var_gr__blk1380_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
            let assign50560_e65385: f64 = (locals.var_eta_mu * locals.var_qis__blk1376);
            let assign50560_e65386: f64 = (locals.var_qbs__blk1377 + assign50560_e65385);
            let assign50560_e65387: f64 = (locals.var_e_eff0 * assign50560_e65386);
            (locals.var_eeffs__blk1381, locals.var_eeffs__blk1381_dn4, locals.var_eeffs__blk1381_dn6, locals.var_eeffs__blk1381_dn7, locals.var_eeffs__blk1381_dn8, locals.var_eeffs__blk1381_dn9, ) = (assign50560_e65387, (locals.var_e_eff0 * (locals.var_qbs__blk1377_dn4 + (locals.var_eta_mu * locals.var_qis__blk1376_dn4))), (locals.var_e_eff0 * (locals.var_qbs__blk1377_dn6 + (locals.var_eta_mu * locals.var_qis__blk1376_dn6))), (locals.var_e_eff0 * (locals.var_qbs__blk1377_dn7 + (locals.var_eta_mu * locals.var_qis__blk1376_dn7))), (locals.var_e_eff0 * (locals.var_qbs__blk1377_dn8 + (locals.var_eta_mu * locals.var_qis__blk1376_dn8))), (locals.var_e_eff0 * (locals.var_qbs__blk1377_dn9 + (locals.var_eta_mu * locals.var_qis__blk1376_dn9))), );
            locals.var_eeffs__blk1381_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
            let assign50570_e65400: f64 = (locals.var_ps__blk1371 + locals.var_ds__blk1370);
            let assign50570_e65402: f64 = (assign50570_e65400 + 1e-14);
            let assign50570_e65403: f64 = (locals.var_ps__blk1371 / assign50570_e65402);
            let assign50570_e65404: f64 = (assign50570_e65403).ln();
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign50570_e65404, ((((locals.var_ps__blk1371_dn4 * assign50570_e65402) - (locals.var_ps__blk1371 * (locals.var_ps__blk1371_dn4 + locals.var_ds__blk1370_dn4))) / (assign50570_e65402 * assign50570_e65402)) / assign50570_e65403), ((((locals.var_ps__blk1371_dn6 * assign50570_e65402) - (locals.var_ps__blk1371 * (locals.var_ps__blk1371_dn6 + locals.var_ds__blk1370_dn6))) / (assign50570_e65402 * assign50570_e65402)) / assign50570_e65403), ((((locals.var_ps__blk1371_dn7 * assign50570_e65402) - (locals.var_ps__blk1371 * (locals.var_ps__blk1371_dn7 + locals.var_ds__blk1370_dn7))) / (assign50570_e65402 * assign50570_e65402)) / assign50570_e65403), ((((locals.var_ps__blk1371_dn8 * assign50570_e65402) - (locals.var_ps__blk1371 * (locals.var_ps__blk1371_dn8 + locals.var_ds__blk1370_dn8))) / (assign50570_e65402 * assign50570_e65402)) / assign50570_e65403), ((((locals.var_ps__blk1371_dn9 * assign50570_e65402) - (locals.var_ps__blk1371 * (locals.var_ps__blk1371_dn9 + locals.var_ds__blk1370_dn9))) / (assign50570_e65402 * assign50570_e65402)) / assign50570_e65403), );
            locals.var_temp1_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
            let assign50580_e65416: f64 = (locals.var_eeffs__blk1381 * locals.var_mue_t);
            let assign50580_e65418: f64 = (assign50580_e65416).powf(locals.var_themu_t);
            let assign50580_e65422: f64 = (0.5 * locals.var_thecs_t);
            let assign50580_e65424: f64 = (assign50580_e65422 * locals.var_temp1);
            let assign50580_e65425: f64 = (assign50580_e65424).exp();
            let assign50580_e65426: f64 = (locals.var_cs_t * assign50580_e65425);
            let assign50580_e65427: f64 = (assign50580_e65418 + assign50580_e65426);
            (locals.var_mutmp__blk1382, locals.var_mutmp__blk1382_dn4, locals.var_mutmp__blk1382_dn6, locals.var_mutmp__blk1382_dn7, locals.var_mutmp__blk1382_dn8, locals.var_mutmp__blk1382_dn9, ) = (assign50580_e65427, (if locals.var_themu_t_dn4 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50580_e65416).powf(locals.var_themu_t - 1.0) * ((locals.var_eeffs__blk1381_dn4 * locals.var_mue_t) + (locals.var_eeffs__blk1381 * locals.var_mue_t_dn4)))) } } else { (assign50580_e65418 * ((locals.var_themu_t_dn4 * (assign50580_e65416).ln()) + (locals.var_themu_t * (((locals.var_eeffs__blk1381_dn4 * locals.var_mue_t) + (locals.var_eeffs__blk1381 * locals.var_mue_t_dn4)) / assign50580_e65416)))) } + ((locals.var_cs_t_dn4 * assign50580_e65425) + (locals.var_cs_t * (assign50580_e65425 * (((0.5 * locals.var_thecs_t_dn4) * locals.var_temp1) + (assign50580_e65422 * locals.var_temp1_dn4)))))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50580_e65416).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1381_dn6 * locals.var_mue_t))) } } else { (assign50580_e65418 * (locals.var_themu_t * ((locals.var_eeffs__blk1381_dn6 * locals.var_mue_t) / assign50580_e65416))) } + (locals.var_cs_t * (assign50580_e65425 * (assign50580_e65422 * locals.var_temp1_dn6)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50580_e65416).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1381_dn7 * locals.var_mue_t))) } } else { (assign50580_e65418 * (locals.var_themu_t * ((locals.var_eeffs__blk1381_dn7 * locals.var_mue_t) / assign50580_e65416))) } + (locals.var_cs_t * (assign50580_e65425 * (assign50580_e65422 * locals.var_temp1_dn7)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50580_e65416).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1381_dn8 * locals.var_mue_t))) } } else { (assign50580_e65418 * (locals.var_themu_t * ((locals.var_eeffs__blk1381_dn8 * locals.var_mue_t) / assign50580_e65416))) } + (locals.var_cs_t * (assign50580_e65425 * (assign50580_e65422 * locals.var_temp1_dn8)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50580_e65416).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1381_dn9 * locals.var_mue_t))) } } else { (assign50580_e65418 * (locals.var_themu_t * ((locals.var_eeffs__blk1381_dn9 * locals.var_mue_t) / assign50580_e65416))) } + (locals.var_cs_t * (assign50580_e65425 * (assign50580_e65422 * locals.var_temp1_dn9)))), );
            locals.var_mutmp__blk1382_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
            let assign50590_e65439: f64 = (1.0 + locals.var_mutmp__blk1382);
            let assign50590_e65441: f64 = (assign50590_e65439 + locals.var_gr__blk1380);
            let assign50590_e65443: f64 = (assign50590_e65441 * locals.var_rxcor__blk1374);
            (locals.var_gmobs__blk1383, locals.var_gmobs__blk1383_dn4, locals.var_gmobs__blk1383_dn6, locals.var_gmobs__blk1383_dn7, locals.var_gmobs__blk1383_dn8, locals.var_gmobs__blk1383_dn9, ) = (assign50590_e65443, (((locals.var_mutmp__blk1382_dn4 + locals.var_gr__blk1380_dn4) * locals.var_rxcor__blk1374) + (assign50590_e65441 * locals.var_rxcor__blk1374_dn4)), (((locals.var_mutmp__blk1382_dn6 + locals.var_gr__blk1380_dn6) * locals.var_rxcor__blk1374) + (assign50590_e65441 * locals.var_rxcor__blk1374_dn6)), (((locals.var_mutmp__blk1382_dn7 + locals.var_gr__blk1380_dn7) * locals.var_rxcor__blk1374) + (assign50590_e65441 * locals.var_rxcor__blk1374_dn7)), (((locals.var_mutmp__blk1382_dn8 + locals.var_gr__blk1380_dn8) * locals.var_rxcor__blk1374) + (assign50590_e65441 * locals.var_rxcor__blk1374_dn8)), (((locals.var_mutmp__blk1382_dn9 + locals.var_gr__blk1380_dn9) * locals.var_rxcor__blk1374) + (assign50590_e65441 * locals.var_rxcor__blk1374_dn9)), );
            locals.var_gmobs__blk1383_rv = 0.0;
        }

        let assign50600_e65448: f64 = if locals.var_thesatb_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1498 = assign50600_e65448;
        locals.var_guard1498_rv = 0.0;

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1498 != 0.0)) {
            let assign50610_e65462: f64 = (locals.var_thesatb_i * locals.var_vsbx__blk1323);
            let assign50610_e65463: f64 = (1.0 - assign50610_e65462);
            let assign50610_e65464: f64 = (1.0 / assign50610_e65463);
            (locals.var_xitsb__blk1384, locals.var_xitsb__blk1384_dn4, locals.var_xitsb__blk1384_dn6, locals.var_xitsb__blk1384_dn7, locals.var_xitsb__blk1384_dn8, locals.var_xitsb__blk1384_dn9, ) = (assign50610_e65464, (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1323_dn4)) / (assign50610_e65463 * assign50610_e65463))), (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1323_dn6)) / (assign50610_e65463 * assign50610_e65463))), (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1323_dn7)) / (assign50610_e65463 * assign50610_e65463))), (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1323_dn8)) / (assign50610_e65463 * assign50610_e65463))), (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1323_dn9)) / (assign50610_e65463 * assign50610_e65463))), );
            locals.var_xitsb__blk1384_rv = 0.0;
        }

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1498 == 0.0)) {
            let assign50620_e65480: f64 = (locals.var_thesatb_i * locals.var_vsbx__blk1323);
            let assign50620_e65481: f64 = (1.0 + assign50620_e65480);
            (locals.var_xitsb__blk1384, locals.var_xitsb__blk1384_dn4, locals.var_xitsb__blk1384_dn6, locals.var_xitsb__blk1384_dn7, locals.var_xitsb__blk1384_dn8, locals.var_xitsb__blk1384_dn9, ) = (assign50620_e65481, (locals.var_thesatb_i * locals.var_vsbx__blk1323_dn4), (locals.var_thesatb_i * locals.var_vsbx__blk1323_dn6), (locals.var_thesatb_i * locals.var_vsbx__blk1323_dn7), (locals.var_thesatb_i * locals.var_vsbx__blk1323_dn8), (locals.var_thesatb_i * locals.var_vsbx__blk1323_dn9), );
            locals.var_xitsb__blk1384_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
            let assign50630_e65493: f64 = (locals.var_qis__blk1376 * locals.var_xitsb__blk1384);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign50630_e65493, ((locals.var_qis__blk1376_dn4 * locals.var_xitsb__blk1384) + (locals.var_qis__blk1376 * locals.var_xitsb__blk1384_dn4)), ((locals.var_qis__blk1376_dn6 * locals.var_xitsb__blk1384) + (locals.var_qis__blk1376 * locals.var_xitsb__blk1384_dn6)), ((locals.var_qis__blk1376_dn7 * locals.var_xitsb__blk1384) + (locals.var_qis__blk1376 * locals.var_xitsb__blk1384_dn7)), ((locals.var_qis__blk1376_dn8 * locals.var_xitsb__blk1384) + (locals.var_qis__blk1376 * locals.var_xitsb__blk1384_dn8)), ((locals.var_qis__blk1376_dn9 * locals.var_xitsb__blk1384) + (locals.var_qis__blk1376 * locals.var_xitsb__blk1384_dn9)), );
            locals.var_temp2_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) {
            let assign50640_e65506: f64 = (locals.var_thesatt_i + locals.var_temp2);
            let assign50640_e65507: f64 = (locals.var_temp2 / assign50640_e65506);
            (locals.var_wsat__blk1385, locals.var_wsat__blk1385_dn4, locals.var_wsat__blk1385_dn6, locals.var_wsat__blk1385_dn7, locals.var_wsat__blk1385_dn8, locals.var_wsat__blk1385_dn9, ) = (assign50640_e65507, (((locals.var_temp2_dn4 * assign50640_e65506) - (locals.var_temp2 * locals.var_temp2_dn4)) / (assign50640_e65506 * assign50640_e65506)), (((locals.var_temp2_dn6 * assign50640_e65506) - (locals.var_temp2 * locals.var_temp2_dn6)) / (assign50640_e65506 * assign50640_e65506)), (((locals.var_temp2_dn7 * assign50640_e65506) - (locals.var_temp2 * locals.var_temp2_dn7)) / (assign50640_e65506 * assign50640_e65506)), (((locals.var_temp2_dn8 * assign50640_e65506) - (locals.var_temp2 * locals.var_temp2_dn8)) / (assign50640_e65506 * assign50640_e65506)), (((locals.var_temp2_dn9 * assign50640_e65506) - (locals.var_temp2 * locals.var_temp2_dn9)) / (assign50640_e65506 * assign50640_e65506)), );
            locals.var_wsat__blk1385_rv = 0.0;
        }

        let assign50650_e65512: f64 = if locals.var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1499 = assign50650_e65512;
        locals.var_guard1499_rv = 0.0;

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1499 != 0.0)) {
            let assign50660_e65526: f64 = (locals.var_thesatg_i * locals.var_wsat__blk1385);
            let assign50660_e65527: f64 = (1.0 - assign50660_e65526);
            let assign50660_e65528: f64 = (1.0 / assign50660_e65527);
            (locals.var_factheta__blk1386, locals.var_factheta__blk1386_dn4, locals.var_factheta__blk1386_dn6, locals.var_factheta__blk1386_dn7, locals.var_factheta__blk1386_dn8, locals.var_factheta__blk1386_dn9, ) = (assign50660_e65528, (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn4)) / (assign50660_e65527 * assign50660_e65527))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn6)) / (assign50660_e65527 * assign50660_e65527))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn7)) / (assign50660_e65527 * assign50660_e65527))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn8)) / (assign50660_e65527 * assign50660_e65527))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn9)) / (assign50660_e65527 * assign50660_e65527))), );
            locals.var_factheta__blk1386_rv = 0.0;
        }

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1495 != 0.0)) && (locals.var_guard1499 == 0.0)) {
            let assign50670_e65544: f64 = (locals.var_thesatg_i * locals.var_wsat__blk1385);
            let assign50670_e65545: f64 = (1.0 + assign50670_e65544);
            (locals.var_factheta__blk1386, locals.var_factheta__blk1386_dn4, locals.var_factheta__blk1386_dn6, locals.var_factheta__blk1386_dn7, locals.var_factheta__blk1386_dn8, locals.var_factheta__blk1386_dn9, ) = (assign50670_e65545, (locals.var_thesatg_i * locals.var_wsat__blk1385_dn4), (locals.var_thesatg_i * locals.var_wsat__blk1385_dn6), (locals.var_thesatg_i * locals.var_wsat__blk1385_dn7), (locals.var_thesatg_i * locals.var_wsat__blk1385_dn8), (locals.var_thesatg_i * locals.var_wsat__blk1385_dn9), );
            locals.var_factheta__blk1386_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1474 == 0.0)) {
            (locals.var_vgb1__blk1321, locals.var_vgb1__blk1321_dn4, locals.var_vgb1__blk1321_dn6, locals.var_vgb1__blk1321_dn7, locals.var_vgb1__blk1321_dn8, locals.var_vgb1__blk1321_dn9, ) = (locals.var_vgb1_dc, locals.var_vgb1_dc_dn4, locals.var_vgb1_dc_dn6, locals.var_vgb1_dc_dn7, locals.var_vgb1_dc_dn8, locals.var_vgb1_dc_dn9, );
            locals.var_vgb1__blk1321_rv = 0.0;
            (locals.var_vsbx__blk1323, locals.var_vsbx__blk1323_dn4, locals.var_vsbx__blk1323_dn6, locals.var_vsbx__blk1323_dn7, locals.var_vsbx__blk1323_dn8, locals.var_vsbx__blk1323_dn9, ) = (locals.var_vsbx_dc, locals.var_vsbx_dc_dn4, locals.var_vsbx_dc_dn6, locals.var_vsbx_dc_dn7, locals.var_vsbx_dc_dn8, locals.var_vsbx_dc_dn9, );
            locals.var_vsbx__blk1323_rv = 0.0;
            (locals.var_phit1__blk1339, locals.var_phit1__blk1339_dn4, locals.var_phit1__blk1339_dn6, locals.var_phit1__blk1339_dn7, locals.var_phit1__blk1339_dn8, locals.var_phit1__blk1339_dn9, ) = (locals.var_phit1_dc, locals.var_phit1_dc_dn4, locals.var_phit1_dc_dn6, locals.var_phit1_dc_dn7, locals.var_phit1_dc_dn8, locals.var_phit1_dc_dn9, );
            locals.var_phit1__blk1339_rv = 0.0;
            (locals.var_inv_phit1__blk1340, locals.var_inv_phit1__blk1340_dn4, locals.var_inv_phit1__blk1340_dn6, locals.var_inv_phit1__blk1340_dn7, locals.var_inv_phit1__blk1340_dn8, locals.var_inv_phit1__blk1340_dn9, ) = (locals.var_inv_phit1_dc, locals.var_inv_phit1_dc_dn4, locals.var_inv_phit1_dc_dn6, locals.var_inv_phit1_dc_dn7, locals.var_inv_phit1_dc_dn8, locals.var_inv_phit1_dc_dn9, );
            locals.var_inv_phit1__blk1340_rv = 0.0;
            (locals.var_gf__blk1324, locals.var_gf__blk1324_dn4, locals.var_gf__blk1324_dn6, locals.var_gf__blk1324_dn7, locals.var_gf__blk1324_dn8, locals.var_gf__blk1324_dn9, ) = (locals.var_gf_dc, locals.var_gf_dc_dn4, locals.var_gf_dc_dn6, locals.var_gf_dc_dn7, locals.var_gf_dc_dn8, locals.var_gf_dc_dn9, );
            locals.var_gf__blk1324_rv = 0.0;
            (locals.var_gf2__blk1325, locals.var_gf2__blk1325_dn4, locals.var_gf2__blk1325_dn6, locals.var_gf2__blk1325_dn7, locals.var_gf2__blk1325_dn8, locals.var_gf2__blk1325_dn9, ) = (locals.var_gf2_dc, locals.var_gf2_dc_dn4, locals.var_gf2_dc_dn6, locals.var_gf2_dc_dn7, locals.var_gf2_dc_dn8, locals.var_gf2_dc_dn9, );
            locals.var_gf2__blk1325_rv = 0.0;
            (locals.var_inv_gf2__blk1341, locals.var_inv_gf2__blk1341_dn4, locals.var_inv_gf2__blk1341_dn6, locals.var_inv_gf2__blk1341_dn7, locals.var_inv_gf2__blk1341_dn8, locals.var_inv_gf2__blk1341_dn9, ) = (locals.var_inv_gf2_dc, locals.var_inv_gf2_dc_dn4, locals.var_inv_gf2_dc_dn6, locals.var_inv_gf2_dc_dn7, locals.var_inv_gf2_dc_dn8, locals.var_inv_gf2_dc_dn9, );
            locals.var_inv_gf2__blk1341_rv = 0.0;
            (locals.var_xg__blk1343, locals.var_xg__blk1343_dn4, locals.var_xg__blk1343_dn6, locals.var_xg__blk1343_dn7, locals.var_xg__blk1343_dn8, locals.var_xg__blk1343_dn9, ) = (locals.var_xg_dc, locals.var_xg_dc_dn4, locals.var_xg_dc_dn6, locals.var_xg_dc_dn7, locals.var_xg_dc_dn8, locals.var_xg_dc_dn9, );
            locals.var_xg__blk1343_rv = 0.0;
            (locals.var_xno_s__blk1348, locals.var_xno_s__blk1348_dn4, locals.var_xno_s__blk1348_dn6, locals.var_xno_s__blk1348_dn7, locals.var_xno_s__blk1348_dn8, locals.var_xno_s__blk1348_dn9, ) = (locals.var_xno_s_dc, locals.var_xno_s_dc_dn4, locals.var_xno_s_dc_dn6, locals.var_xno_s_dc_dn7, locals.var_xno_s_dc_dn8, locals.var_xno_s_dc_dn9, );
            locals.var_xno_s__blk1348_rv = 0.0;
            (locals.var_xn_s__blk1349, locals.var_xn_s__blk1349_dn4, locals.var_xn_s__blk1349_dn6, locals.var_xn_s__blk1349_dn7, locals.var_xn_s__blk1349_dn8, locals.var_xn_s__blk1349_dn9, ) = (locals.var_xn_s_dc, locals.var_xn_s_dc_dn4, locals.var_xn_s_dc_dn6, locals.var_xn_s_dc_dn7, locals.var_xn_s_dc_dn8, locals.var_xn_s_dc_dn9, );
            locals.var_xn_s__blk1349_rv = 0.0;
            (locals.var_xi__blk1360, locals.var_xi__blk1360_dn4, locals.var_xi__blk1360_dn6, locals.var_xi__blk1360_dn7, locals.var_xi__blk1360_dn8, locals.var_xi__blk1360_dn9, ) = (locals.var_xi_dc, locals.var_xi_dc_dn4, locals.var_xi_dc_dn6, locals.var_xi_dc_dn7, locals.var_xi_dc_dn8, locals.var_xi_dc_dn9, );
            locals.var_xi__blk1360_rv = 0.0;
            (locals.var_margin__blk1361, locals.var_margin__blk1361_dn4, locals.var_margin__blk1361_dn6, locals.var_margin__blk1361_dn7, locals.var_margin__blk1361_dn8, locals.var_margin__blk1361_dn9, ) = (locals.var_margin_dc, locals.var_margin_dc_dn4, locals.var_margin_dc_dn6, locals.var_margin_dc_dn7, locals.var_margin_dc_dn8, locals.var_margin_dc_dn9, );
            locals.var_margin__blk1361_rv = 0.0;
            (locals.var_inv_xi__blk1362, locals.var_inv_xi__blk1362_dn4, locals.var_inv_xi__blk1362_dn6, locals.var_inv_xi__blk1362_dn7, locals.var_inv_xi__blk1362_dn8, locals.var_inv_xi__blk1362_dn9, ) = (locals.var_inv_xi_dc, locals.var_inv_xi_dc_dn4, locals.var_inv_xi_dc_dn6, locals.var_inv_xi_dc_dn7, locals.var_inv_xi_dc_dn8, locals.var_inv_xi_dc_dn9, );
            locals.var_inv_xi__blk1362_rv = 0.0;
            (locals.var_sp_s_x1__blk1469, locals.var_sp_s_x1__blk1469_dn4, locals.var_sp_s_x1__blk1469_dn6, locals.var_sp_s_x1__blk1469_dn7, locals.var_sp_s_x1__blk1469_dn8, locals.var_sp_s_x1__blk1469_dn9, ) = (locals.var_sp_s_x1_dc, locals.var_sp_s_x1_dc_dn4, locals.var_sp_s_x1_dc_dn6, locals.var_sp_s_x1_dc_dn7, locals.var_sp_s_x1_dc_dn8, locals.var_sp_s_x1_dc_dn9, );
            locals.var_sp_s_x1__blk1469_rv = 0.0;
            (locals.var_delta_ns__blk1364, locals.var_delta_ns__blk1364_dn4, locals.var_delta_ns__blk1364_dn6, locals.var_delta_ns__blk1364_dn7, locals.var_delta_ns__blk1364_dn8, locals.var_delta_ns__blk1364_dn9, ) = (locals.var_delta_ns_dc, locals.var_delta_ns_dc_dn4, locals.var_delta_ns_dc_dn6, locals.var_delta_ns_dc_dn7, locals.var_delta_ns_dc_dn8, locals.var_delta_ns_dc_dn9, );
            locals.var_delta_ns__blk1364_rv = 0.0;
            (locals.var_x_s__blk1363, locals.var_x_s__blk1363_dn4, locals.var_x_s__blk1363_dn6, locals.var_x_s__blk1363_dn7, locals.var_x_s__blk1363_dn8, locals.var_x_s__blk1363_dn9, ) = (locals.var_x_s_dc, locals.var_x_s_dc_dn4, locals.var_x_s_dc_dn6, locals.var_x_s_dc_dn7, locals.var_x_s_dc_dn8, locals.var_x_s_dc_dn9, );
            locals.var_x_s__blk1363_rv = 0.0;
            (locals.var_xi1s__blk1366, locals.var_xi1s__blk1366_dn4, locals.var_xi1s__blk1366_dn6, locals.var_xi1s__blk1366_dn7, locals.var_xi1s__blk1366_dn8, locals.var_xi1s__blk1366_dn9, ) = (locals.var_xi1s_dc, locals.var_xi1s_dc_dn4, locals.var_xi1s_dc_dn6, locals.var_xi1s_dc_dn7, locals.var_xi1s_dc_dn8, locals.var_xi1s_dc_dn9, );
            locals.var_xi1s__blk1366_rv = 0.0;
            (locals.var_xi2s__blk1367, locals.var_xi2s__blk1367_dn4, locals.var_xi2s__blk1367_dn6, locals.var_xi2s__blk1367_dn7, locals.var_xi2s__blk1367_dn8, locals.var_xi2s__blk1367_dn9, ) = (locals.var_xi2s_dc, locals.var_xi2s_dc_dn4, locals.var_xi2s_dc_dn6, locals.var_xi2s_dc_dn7, locals.var_xi2s_dc_dn8, locals.var_xi2s_dc_dn9, );
            locals.var_xi2s__blk1367_rv = 0.0;
            (locals.var_delta_1s__blk1368, locals.var_delta_1s__blk1368_dn4, locals.var_delta_1s__blk1368_dn6, locals.var_delta_1s__blk1368_dn7, locals.var_delta_1s__blk1368_dn8, locals.var_delta_1s__blk1368_dn9, ) = (locals.var_delta_1s_dc, locals.var_delta_1s_dc_dn4, locals.var_delta_1s_dc_dn6, locals.var_delta_1s_dc_dn7, locals.var_delta_1s_dc_dn8, locals.var_delta_1s_dc_dn9, );
            locals.var_delta_1s__blk1368_rv = 0.0;
            (locals.var_es__blk1369, locals.var_es__blk1369_dn4, locals.var_es__blk1369_dn6, locals.var_es__blk1369_dn7, locals.var_es__blk1369_dn8, locals.var_es__blk1369_dn9, ) = (locals.var_es_dc, locals.var_es_dc_dn4, locals.var_es_dc_dn6, locals.var_es_dc_dn7, locals.var_es_dc_dn8, locals.var_es_dc_dn9, );
            locals.var_es__blk1369_rv = 0.0;
            (locals.var_ps__blk1371, locals.var_ps__blk1371_dn4, locals.var_ps__blk1371_dn6, locals.var_ps__blk1371_dn7, locals.var_ps__blk1371_dn8, locals.var_ps__blk1371_dn9, ) = (locals.var_ps_dc, locals.var_ps_dc_dn4, locals.var_ps_dc_dn6, locals.var_ps_dc_dn7, locals.var_ps_dc_dn8, locals.var_ps_dc_dn9, );
            locals.var_ps__blk1371_rv = 0.0;
            (locals.var_ds__blk1370, locals.var_ds__blk1370_dn4, locals.var_ds__blk1370_dn6, locals.var_ds__blk1370_dn7, locals.var_ds__blk1370_dn8, locals.var_ds__blk1370_dn9, ) = (locals.var_ds_dc, locals.var_ds_dc_dn4, locals.var_ds_dc_dn6, locals.var_ds_dc_dn7, locals.var_ds_dc_dn8, locals.var_ds_dc_dn9, );
            locals.var_ds__blk1370_rv = 0.0;
            (locals.var_sqs__blk1372, locals.var_sqs__blk1372_dn4, locals.var_sqs__blk1372_dn6, locals.var_sqs__blk1372_dn7, locals.var_sqs__blk1372_dn8, locals.var_sqs__blk1372_dn9, ) = (locals.var_sqs_dc, locals.var_sqs_dc_dn4, locals.var_sqs_dc_dn6, locals.var_sqs_dc_dn7, locals.var_sqs_dc_dn8, locals.var_sqs_dc_dn9, );
            locals.var_sqs__blk1372_rv = 0.0;
            (locals.var_alphas__blk1373, locals.var_alphas__blk1373_dn4, locals.var_alphas__blk1373_dn6, locals.var_alphas__blk1373_dn7, locals.var_alphas__blk1373_dn8, locals.var_alphas__blk1373_dn9, ) = (locals.var_alphas_dc, locals.var_alphas_dc_dn4, locals.var_alphas_dc_dn6, locals.var_alphas_dc_dn7, locals.var_alphas_dc_dn8, locals.var_alphas_dc_dn9, );
            locals.var_alphas__blk1373_rv = 0.0;
            (locals.var_rxcor__blk1374, locals.var_rxcor__blk1374_dn4, locals.var_rxcor__blk1374_dn6, locals.var_rxcor__blk1374_dn7, locals.var_rxcor__blk1374_dn8, locals.var_rxcor__blk1374_dn9, ) = (locals.var_rxcor_dc, locals.var_rxcor_dc_dn4, locals.var_rxcor_dc_dn6, locals.var_rxcor_dc_dn7, locals.var_rxcor_dc_dn8, locals.var_rxcor_dc_dn9, );
            locals.var_rxcor__blk1374_rv = 0.0;
            (locals.var_xgs__blk1375, locals.var_xgs__blk1375_dn4, locals.var_xgs__blk1375_dn6, locals.var_xgs__blk1375_dn7, locals.var_xgs__blk1375_dn8, locals.var_xgs__blk1375_dn9, ) = (locals.var_xgs_dc, locals.var_xgs_dc_dn4, locals.var_xgs_dc_dn6, locals.var_xgs_dc_dn7, locals.var_xgs_dc_dn8, locals.var_xgs_dc_dn9, );
            locals.var_xgs__blk1375_rv = 0.0;
            (locals.var_qis__blk1376, locals.var_qis__blk1376_dn4, locals.var_qis__blk1376_dn6, locals.var_qis__blk1376_dn7, locals.var_qis__blk1376_dn8, locals.var_qis__blk1376_dn9, ) = (locals.var_qis_dc, locals.var_qis_dc_dn4, locals.var_qis_dc_dn6, locals.var_qis_dc_dn7, locals.var_qis_dc_dn8, locals.var_qis_dc_dn9, );
            locals.var_qis__blk1376_rv = 0.0;
            (locals.var_qbs__blk1377, locals.var_qbs__blk1377_dn4, locals.var_qbs__blk1377_dn6, locals.var_qbs__blk1377_dn7, locals.var_qbs__blk1377_dn8, locals.var_qbs__blk1377_dn9, ) = (locals.var_qbs_dc, locals.var_qbs_dc_dn4, locals.var_qbs_dc_dn6, locals.var_qbs_dc_dn7, locals.var_qbs_dc_dn8, locals.var_qbs_dc_dn9, );
            locals.var_qbs__blk1377_rv = 0.0;
            (locals.var_rhob__blk1378, locals.var_rhob__blk1378_dn4, locals.var_rhob__blk1378_dn6, locals.var_rhob__blk1378_dn7, locals.var_rhob__blk1378_dn8, locals.var_rhob__blk1378_dn9, ) = (locals.var_rhob_dc, locals.var_rhob_dc_dn4, locals.var_rhob_dc_dn6, locals.var_rhob_dc_dn7, locals.var_rhob_dc_dn8, locals.var_rhob_dc_dn9, );
            locals.var_rhob__blk1378_rv = 0.0;
            (locals.var_rhog__blk1379, locals.var_rhog__blk1379_dn4, locals.var_rhog__blk1379_dn6, locals.var_rhog__blk1379_dn7, locals.var_rhog__blk1379_dn8, locals.var_rhog__blk1379_dn9, ) = (locals.var_rhog_dc, locals.var_rhog_dc_dn4, locals.var_rhog_dc_dn6, locals.var_rhog_dc_dn7, locals.var_rhog_dc_dn8, locals.var_rhog_dc_dn9, );
            locals.var_rhog__blk1379_rv = 0.0;
            (locals.var_gmobs__blk1383, locals.var_gmobs__blk1383_dn4, locals.var_gmobs__blk1383_dn6, locals.var_gmobs__blk1383_dn7, locals.var_gmobs__blk1383_dn8, locals.var_gmobs__blk1383_dn9, ) = (locals.var_gmobs_dc, locals.var_gmobs_dc_dn4, locals.var_gmobs_dc_dn6, locals.var_gmobs_dc_dn7, locals.var_gmobs_dc_dn8, locals.var_gmobs_dc_dn9, );
            locals.var_gmobs__blk1383_rv = 0.0;
            (locals.var_xitsb__blk1384, locals.var_xitsb__blk1384_dn4, locals.var_xitsb__blk1384_dn6, locals.var_xitsb__blk1384_dn7, locals.var_xitsb__blk1384_dn8, locals.var_xitsb__blk1384_dn9, ) = (locals.var_xitsb_dc, locals.var_xitsb_dc_dn4, locals.var_xitsb_dc_dn6, locals.var_xitsb_dc_dn7, locals.var_xitsb_dc_dn8, locals.var_xitsb_dc_dn9, );
            locals.var_xitsb__blk1384_rv = 0.0;
            (locals.var_factheta__blk1386, locals.var_factheta__blk1386_dn4, locals.var_factheta__blk1386_dn6, locals.var_factheta__blk1386_dn7, locals.var_factheta__blk1386_dn8, locals.var_factheta__blk1386_dn9, ) = (locals.var_factheta_dc, locals.var_factheta_dc_dn4, locals.var_factheta_dc_dn6, locals.var_factheta_dc_dn7, locals.var_factheta_dc_dn8, locals.var_factheta_dc_dn9, );
            locals.var_factheta__blk1386_rv = 0.0;
        }

        if (locals.var_guard1473 != 0.0) {
            (locals.var_thesatloc__blk1319, locals.var_thesatloc__blk1319_dn4, ) = (locals.var_thesat_t, locals.var_thesat_t_dn4, );
            locals.var_thesatloc__blk1319_rv = 0.0;
            locals.var_arloc__blk1320 = locals.var_ar;
            locals.var_arloc__blk1320_rv = 0.0;
        }

        let assign51130_e65888: f64 = if p.p48 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1500 = assign51130_e65888;
        locals.var_guard1500_rv = 0.0;

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1500 != 0.0)) {
            (locals.var_thesatloc__blk1319, locals.var_thesatloc__blk1319_dn4, ) = (locals.var_thesatac_t, locals.var_thesatac_t_dn4, );
            locals.var_thesatloc__blk1319_rv = 0.0;
            locals.var_arloc__blk1320 = locals.var_arac;
            locals.var_arloc__blk1320_rv = 0.0;
        }

        if (locals.var_guard1473 != 0.0) {
            (locals.var_thesat1__blk1388, locals.var_thesat1__blk1388_dn4, locals.var_thesat1__blk1388_dn6, locals.var_thesat1__blk1388_dn7, locals.var_thesat1__blk1388_dn8, locals.var_thesat1__blk1388_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_thesat1__blk1388_rv = 0.0;
        }

        if (locals.var_guard1473 != 0.0) {
            let assign51170_e65908: f64 = (locals.var_phit1__blk1339 * 4.60517018598809);
            (locals.var_vdsat_lim__blk1387, locals.var_vdsat_lim__blk1387_dn4, locals.var_vdsat_lim__blk1387_dn6, locals.var_vdsat_lim__blk1387_dn7, locals.var_vdsat_lim__blk1387_dn8, locals.var_vdsat_lim__blk1387_dn9, ) = (assign51170_e65908, (locals.var_phit1__blk1339_dn4 * 4.60517018598809), (locals.var_phit1__blk1339_dn6 * 4.60517018598809), (locals.var_phit1__blk1339_dn7 * 4.60517018598809), (locals.var_phit1__blk1339_dn8 * 4.60517018598809), (locals.var_phit1__blk1339_dn9 * 4.60517018598809), );
            locals.var_vdsat_lim__blk1387_rv = 0.0;
        }

        if (locals.var_guard1473 != 0.0) {
            (locals.var_v_dsat__blk1404, locals.var_v_dsat__blk1404_dn4, locals.var_v_dsat__blk1404_dn6, locals.var_v_dsat__blk1404_dn7, locals.var_v_dsat__blk1404_dn8, locals.var_v_dsat__blk1404_dn9, ) = (locals.var_vdsat_lim__blk1387, locals.var_vdsat_lim__blk1387_dn4, locals.var_vdsat_lim__blk1387_dn6, locals.var_vdsat_lim__blk1387_dn7, locals.var_vdsat_lim__blk1387_dn8, locals.var_vdsat_lim__blk1387_dn9, );
            locals.var_v_dsat__blk1404_rv = 0.0;
            (locals.var_vdse__blk1405, locals.var_vdse__blk1405_dn4, locals.var_vdse__blk1405_dn6, locals.var_vdse__blk1405_dn7, locals.var_vdse__blk1405_dn8, locals.var_vdse__blk1405_dn9, ) = (locals.var_v_ds, 0.0, 0.0, locals.var_v_ds_dn7, locals.var_v_ds_dn8, 0.0, );
            locals.var_vdse__blk1405_rv = 0.0;
        }

        if (locals.var_guard1473 != 0.0) {
            let assign51200_e65922: f64 = (locals.var_v_ds * locals.var_inv_phit1__blk1340);
            (locals.var_udse__blk1406, locals.var_udse__blk1406_dn4, locals.var_udse__blk1406_dn6, locals.var_udse__blk1406_dn7, locals.var_udse__blk1406_dn8, locals.var_udse__blk1406_dn9, ) = (assign51200_e65922, (locals.var_v_ds * locals.var_inv_phit1__blk1340_dn4), (locals.var_v_ds * locals.var_inv_phit1__blk1340_dn6), ((locals.var_v_ds_dn7 * locals.var_inv_phit1__blk1340) + (locals.var_v_ds * locals.var_inv_phit1__blk1340_dn7)), ((locals.var_v_ds_dn8 * locals.var_inv_phit1__blk1340) + (locals.var_v_ds * locals.var_inv_phit1__blk1340_dn8)), (locals.var_v_ds * locals.var_inv_phit1__blk1340_dn9), );
            locals.var_udse__blk1406_rv = 0.0;
        }

        if (locals.var_guard1473 != 0.0) {
            (locals.var_x_d__blk1410, locals.var_x_d__blk1410_dn4, locals.var_x_d__blk1410_dn6, locals.var_x_d__blk1410_dn7, locals.var_x_d__blk1410_dn8, locals.var_x_d__blk1410_dn9, ) = (locals.var_x_s__blk1363, locals.var_x_s__blk1363_dn4, locals.var_x_s__blk1363_dn6, locals.var_x_s__blk1363_dn7, locals.var_x_s__blk1363_dn8, locals.var_x_s__blk1363_dn9, );
            locals.var_x_d__blk1410_rv = 0.0;
            (locals.var_x_ds__blk1411, locals.var_x_ds__blk1411_dn4, locals.var_x_ds__blk1411_dn6, locals.var_x_ds__blk1411_dn7, locals.var_x_ds__blk1411_dn8, locals.var_x_ds__blk1411_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_x_ds__blk1411_rv = 0.0;
            (locals.var_dps__blk1414, locals.var_dps__blk1414_dn4, locals.var_dps__blk1414_dn6, locals.var_dps__blk1414_dn7, locals.var_dps__blk1414_dn8, locals.var_dps__blk1414_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_dps__blk1414_rv = 0.0;
            (locals.var_ed__blk1416, locals.var_ed__blk1416_dn4, locals.var_ed__blk1416_dn6, locals.var_ed__blk1416_dn7, locals.var_ed__blk1416_dn8, locals.var_ed__blk1416_dn9, ) = (locals.var_es__blk1369, locals.var_es__blk1369_dn4, locals.var_es__blk1369_dn6, locals.var_es__blk1369_dn7, locals.var_es__blk1369_dn8, locals.var_es__blk1369_dn9, );
            locals.var_ed__blk1416_rv = 0.0;
            (locals.var_pd__blk1417, locals.var_pd__blk1417_dn4, locals.var_pd__blk1417_dn6, locals.var_pd__blk1417_dn7, locals.var_pd__blk1417_dn8, locals.var_pd__blk1417_dn9, ) = (locals.var_ps__blk1371, locals.var_ps__blk1371_dn4, locals.var_ps__blk1371_dn6, locals.var_ps__blk1371_dn7, locals.var_ps__blk1371_dn8, locals.var_ps__blk1371_dn9, );
            locals.var_pd__blk1417_rv = 0.0;
            (locals.var_dd__blk1419, locals.var_dd__blk1419_dn4, locals.var_dd__blk1419_dn6, locals.var_dd__blk1419_dn7, locals.var_dd__blk1419_dn8, locals.var_dd__blk1419_dn9, ) = (locals.var_ds__blk1370, locals.var_ds__blk1370_dn4, locals.var_ds__blk1370_dn6, locals.var_ds__blk1370_dn7, locals.var_ds__blk1370_dn8, locals.var_ds__blk1370_dn9, );
            locals.var_dd__blk1419_rv = 0.0;
            (locals.var_qbd__blk1420, locals.var_qbd__blk1420_dn4, locals.var_qbd__blk1420_dn6, locals.var_qbd__blk1420_dn7, locals.var_qbd__blk1420_dn8, locals.var_qbd__blk1420_dn9, ) = (locals.var_qbs__blk1377, locals.var_qbs__blk1377_dn4, locals.var_qbs__blk1377_dn6, locals.var_qbs__blk1377_dn7, locals.var_qbs__blk1377_dn8, locals.var_qbs__blk1377_dn9, );
            locals.var_qbd__blk1420_rv = 0.0;
            (locals.var_x_m__blk1421, locals.var_x_m__blk1421_dn4, locals.var_x_m__blk1421_dn6, locals.var_x_m__blk1421_dn7, locals.var_x_m__blk1421_dn8, locals.var_x_m__blk1421_dn9, ) = (locals.var_x_s__blk1363, locals.var_x_s__blk1363_dn4, locals.var_x_s__blk1363_dn6, locals.var_x_s__blk1363_dn7, locals.var_x_s__blk1363_dn8, locals.var_x_s__blk1363_dn9, );
            locals.var_x_m__blk1421_rv = 0.0;
            (locals.var_em__blk1422, locals.var_em__blk1422_dn4, locals.var_em__blk1422_dn6, locals.var_em__blk1422_dn7, locals.var_em__blk1422_dn8, locals.var_em__blk1422_dn9, ) = (locals.var_es__blk1369, locals.var_es__blk1369_dn4, locals.var_es__blk1369_dn6, locals.var_es__blk1369_dn7, locals.var_es__blk1369_dn8, locals.var_es__blk1369_dn9, );
            locals.var_em__blk1422_rv = 0.0;
            (locals.var_dm__blk1424, locals.var_dm__blk1424_dn4, locals.var_dm__blk1424_dn6, locals.var_dm__blk1424_dn7, locals.var_dm__blk1424_dn8, locals.var_dm__blk1424_dn9, ) = (locals.var_ds__blk1370, locals.var_ds__blk1370_dn4, locals.var_ds__blk1370_dn6, locals.var_ds__blk1370_dn7, locals.var_ds__blk1370_dn8, locals.var_ds__blk1370_dn9, );
            locals.var_dm__blk1424_rv = 0.0;
            (locals.var_pm__blk1425, locals.var_pm__blk1425_dn4, locals.var_pm__blk1425_dn6, locals.var_pm__blk1425_dn7, locals.var_pm__blk1425_dn8, locals.var_pm__blk1425_dn9, ) = (locals.var_ps__blk1371, locals.var_ps__blk1371_dn4, locals.var_ps__blk1371_dn6, locals.var_ps__blk1371_dn7, locals.var_ps__blk1371_dn8, locals.var_ps__blk1371_dn9, );
            locals.var_pm__blk1425_rv = 0.0;
        }

        if (locals.var_guard1473 != 0.0) {
            let assign51320_e65972: f64 = (locals.var_xg__blk1343 - locals.var_x_s__blk1363);
            (locals.var_xgm__blk1426, locals.var_xgm__blk1426_dn4, locals.var_xgm__blk1426_dn6, locals.var_xgm__blk1426_dn7, locals.var_xgm__blk1426_dn8, locals.var_xgm__blk1426_dn9, ) = (assign51320_e65972, (locals.var_xg__blk1343_dn4 - locals.var_x_s__blk1363_dn4), (locals.var_xg__blk1343_dn6 - locals.var_x_s__blk1363_dn6), (locals.var_xg__blk1343_dn7 - locals.var_x_s__blk1363_dn7), (locals.var_xg__blk1343_dn8 - locals.var_x_s__blk1363_dn8), (locals.var_xg__blk1343_dn9 - locals.var_x_s__blk1363_dn9), );
            locals.var_xgm__blk1426_rv = 0.0;
        }

        if (locals.var_guard1473 != 0.0) {
            (locals.var_eta_p__blk1427, locals.var_eta_p__blk1427_dn4, locals.var_eta_p__blk1427_dn6, locals.var_eta_p__blk1427_dn7, locals.var_eta_p__blk1427_dn8, locals.var_eta_p__blk1427_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_eta_p__blk1427_rv = 0.0;
            (locals.var_alpha__blk1429, locals.var_alpha__blk1429_dn4, locals.var_alpha__blk1429_dn6, locals.var_alpha__blk1429_dn7, locals.var_alpha__blk1429_dn8, locals.var_alpha__blk1429_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_alpha__blk1429_rv = 0.0;
            (locals.var_sqm__blk1428, locals.var_sqm__blk1428_dn4, locals.var_sqm__blk1428_dn6, locals.var_sqm__blk1428_dn7, locals.var_sqm__blk1428_dn8, locals.var_sqm__blk1428_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_sqm__blk1428_rv = 0.0;
            (locals.var_qim__blk1438, locals.var_qim__blk1438_dn4, locals.var_qim__blk1438_dn6, locals.var_qim__blk1438_dn7, locals.var_qim__blk1438_dn8, locals.var_qim__blk1438_dn9, ) = (locals.var_qis__blk1376, locals.var_qis__blk1376_dn4, locals.var_qis__blk1376_dn6, locals.var_qis__blk1376_dn7, locals.var_qis__blk1376_dn8, locals.var_qis__blk1376_dn9, );
            locals.var_qim__blk1438_rv = 0.0;
        }

        if (locals.var_guard1473 != 0.0) {
            let assign51370_e65994: f64 = (locals.var_xgm__blk1426 * locals.var_phit1__blk1339);
            (locals.var_qeff1__blk1442, locals.var_qeff1__blk1442_dn4, locals.var_qeff1__blk1442_dn6, locals.var_qeff1__blk1442_dn7, locals.var_qeff1__blk1442_dn8, locals.var_qeff1__blk1442_dn9, ) = (assign51370_e65994, ((locals.var_xgm__blk1426_dn4 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn4)), ((locals.var_xgm__blk1426_dn6 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn6)), ((locals.var_xgm__blk1426_dn7 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn7)), ((locals.var_xgm__blk1426_dn8 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn8)), ((locals.var_xgm__blk1426_dn9 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn9)), );
            locals.var_qeff1__blk1442_rv = 0.0;
        }

        if (locals.var_guard1473 != 0.0) {
            (locals.var_qim1__blk1439, locals.var_qim1__blk1439_dn4, locals.var_qim1__blk1439_dn6, locals.var_qim1__blk1439_dn7, locals.var_qim1__blk1439_dn8, locals.var_qim1__blk1439_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qim1__blk1439_rv = 0.0;
            (locals.var_qbm__blk1440, locals.var_qbm__blk1440_dn4, locals.var_qbm__blk1440_dn6, locals.var_qbm__blk1440_dn7, locals.var_qbm__blk1440_dn8, locals.var_qbm__blk1440_dn9, ) = (locals.var_qbs__blk1377, locals.var_qbs__blk1377_dn4, locals.var_qbs__blk1377_dn6, locals.var_qbs__blk1377_dn7, locals.var_qbs__blk1377_dn8, locals.var_qbs__blk1377_dn9, );
            locals.var_qbm__blk1440_rv = 0.0;
            (locals.var_s1__blk1445, locals.var_s1__blk1445_dn4, locals.var_s1__blk1445_dn6, locals.var_s1__blk1445_dn7, locals.var_s1__blk1445_dn8, locals.var_s1__blk1445_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_s1__blk1445_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_26(
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard1473 != 0.0) {
            (locals.var_gmob__blk1444, locals.var_gmob__blk1444_dn4, locals.var_gmob__blk1444_dn6, locals.var_gmob__blk1444_dn7, locals.var_gmob__blk1444_dn8, locals.var_gmob__blk1444_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_gmob__blk1444_rv = 0.0;
            (locals.var_thesateff__blk1447, locals.var_thesateff__blk1447_dn4, locals.var_thesateff__blk1447_dn6, locals.var_thesateff__blk1447_dn7, locals.var_thesateff__blk1447_dn8, locals.var_thesateff__blk1447_dn9, ) = (locals.var_thesatloc__blk1319, locals.var_thesatloc__blk1319_dn4, 0.0, 0.0, 0.0, 0.0, );
            locals.var_thesateff__blk1447_rv = 0.0;
            (locals.var_voxm__blk1446, locals.var_voxm__blk1446_dn4, locals.var_voxm__blk1446_dn6, locals.var_voxm__blk1446_dn7, locals.var_voxm__blk1446_dn8, locals.var_voxm__blk1446_dn9, ) = (locals.var_qeff1__blk1442, locals.var_qeff1__blk1442_dn4, locals.var_qeff1__blk1442_dn6, locals.var_qeff1__blk1442_dn7, locals.var_qeff1__blk1442_dn8, locals.var_qeff1__blk1442_dn9, );
            locals.var_voxm__blk1446_rv = 0.0;
        }

        let assign51440_e66023: f64 = if locals.var_xg__blk1343 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1501 = assign51440_e66023;
        locals.var_guard1501_rv = 0.0;

        let assign51450_e66026: f64 = if locals.var_ds__blk1370 > 1e-100 { 1.0 } else { 0.0 };
        locals.var_guard1502 = assign51450_e66026;
        locals.var_guard1502_rv = 0.0;

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign51460_e66034: f64 = (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386);
            (locals.var_thesateff__blk1447, locals.var_thesateff__blk1447_dn4, locals.var_thesateff__blk1447_dn6, locals.var_thesateff__blk1447_dn7, locals.var_thesateff__blk1447_dn8, locals.var_thesateff__blk1447_dn9, ) = (assign51460_e66034, ((locals.var_thesatloc__blk1319_dn4 * locals.var_factheta__blk1386) + (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn4)), (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn6), (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn7), (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn8), (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn9), );
            locals.var_thesateff__blk1447_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign51470_e66044: f64 = (locals.var_thesateff__blk1447 / locals.var_gmobs__blk1383);
            (locals.var_thesat1__blk1388, locals.var_thesat1__blk1388_dn4, locals.var_thesat1__blk1388_dn6, locals.var_thesat1__blk1388_dn7, locals.var_thesat1__blk1388_dn8, locals.var_thesat1__blk1388_dn9, ) = (assign51470_e66044, (((locals.var_thesateff__blk1447_dn4 * locals.var_gmobs__blk1383) - (locals.var_thesateff__blk1447 * locals.var_gmobs__blk1383_dn4)) / (locals.var_gmobs__blk1383 * locals.var_gmobs__blk1383)), (((locals.var_thesateff__blk1447_dn6 * locals.var_gmobs__blk1383) - (locals.var_thesateff__blk1447 * locals.var_gmobs__blk1383_dn6)) / (locals.var_gmobs__blk1383 * locals.var_gmobs__blk1383)), (((locals.var_thesateff__blk1447_dn7 * locals.var_gmobs__blk1383) - (locals.var_thesateff__blk1447 * locals.var_gmobs__blk1383_dn7)) / (locals.var_gmobs__blk1383 * locals.var_gmobs__blk1383)), (((locals.var_thesateff__blk1447_dn8 * locals.var_gmobs__blk1383) - (locals.var_thesateff__blk1447 * locals.var_gmobs__blk1383_dn8)) / (locals.var_gmobs__blk1383 * locals.var_gmobs__blk1383)), (((locals.var_thesateff__blk1447_dn9 * locals.var_gmobs__blk1383) - (locals.var_thesateff__blk1447 * locals.var_gmobs__blk1383_dn9)) / (locals.var_gmobs__blk1383 * locals.var_gmobs__blk1383)), );
            locals.var_thesat1__blk1388_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign51480_e66055: f64 = (0.5 * locals.var_gf2__blk1325);
            let assign51480_e66056: f64 = (locals.var_xgs__blk1375 + assign51480_e66055);
            (locals.var_asat__blk1389, locals.var_asat__blk1389_dn4, locals.var_asat__blk1389_dn6, locals.var_asat__blk1389_dn7, locals.var_asat__blk1389_dn8, locals.var_asat__blk1389_dn9, ) = (assign51480_e66056, (locals.var_xgs__blk1375_dn4 + (0.5 * locals.var_gf2__blk1325_dn4)), (locals.var_xgs__blk1375_dn6 + (0.5 * locals.var_gf2__blk1325_dn6)), (locals.var_xgs__blk1375_dn7 + (0.5 * locals.var_gf2__blk1325_dn7)), (locals.var_xgs__blk1375_dn8 + (0.5 * locals.var_gf2__blk1325_dn8)), (locals.var_xgs__blk1375_dn9 + (0.5 * locals.var_gf2__blk1325_dn9)), );
            locals.var_asat__blk1389_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign51490_e66066: f64 = (locals.var_gf2__blk1325 * locals.var_delta_1s__blk1368);
            let __rspice_inv_cse_0: f64 = 1.0 / locals.var_asat__blk1389;
            let assign51490_e66068: f64 = (assign51490_e66066 * __rspice_inv_cse_0);
            let assign51490_e66070: f64 = (assign51490_e66068 * __rspice_inv_cse_0);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign51490_e66070, ((((((((locals.var_gf2__blk1325_dn4 * locals.var_delta_1s__blk1368) + (locals.var_gf2__blk1325 * locals.var_delta_1s__blk1368_dn4)) * locals.var_asat__blk1389) - (assign51490_e66066 * locals.var_asat__blk1389_dn4)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)) * locals.var_asat__blk1389) - (assign51490_e66068 * locals.var_asat__blk1389_dn4)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)), ((((((((locals.var_gf2__blk1325_dn6 * locals.var_delta_1s__blk1368) + (locals.var_gf2__blk1325 * locals.var_delta_1s__blk1368_dn6)) * locals.var_asat__blk1389) - (assign51490_e66066 * locals.var_asat__blk1389_dn6)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)) * locals.var_asat__blk1389) - (assign51490_e66068 * locals.var_asat__blk1389_dn6)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)), ((((((((locals.var_gf2__blk1325_dn7 * locals.var_delta_1s__blk1368) + (locals.var_gf2__blk1325 * locals.var_delta_1s__blk1368_dn7)) * locals.var_asat__blk1389) - (assign51490_e66066 * locals.var_asat__blk1389_dn7)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)) * locals.var_asat__blk1389) - (assign51490_e66068 * locals.var_asat__blk1389_dn7)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)), ((((((((locals.var_gf2__blk1325_dn8 * locals.var_delta_1s__blk1368) + (locals.var_gf2__blk1325 * locals.var_delta_1s__blk1368_dn8)) * locals.var_asat__blk1389) - (assign51490_e66066 * locals.var_asat__blk1389_dn8)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)) * locals.var_asat__blk1389) - (assign51490_e66068 * locals.var_asat__blk1389_dn8)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)), ((((((((locals.var_gf2__blk1325_dn9 * locals.var_delta_1s__blk1368) + (locals.var_gf2__blk1325 * locals.var_delta_1s__blk1368_dn9)) * locals.var_asat__blk1389) - (assign51490_e66066 * locals.var_asat__blk1389_dn9)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)) * locals.var_asat__blk1389) - (assign51490_e66068 * locals.var_asat__blk1389_dn9)) / (locals.var_asat__blk1389 * locals.var_asat__blk1389)), );
            locals.var_temp__blk949_rv = 0.0;
        }

        let assign51500_e66075: f64 = if locals.var_temp__blk949 > 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard1503 = assign51500_e66075;
        locals.var_guard1503_rv = 0.0;

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1503 != 0.0)) {
            let assign51510_e66085: f64 = (1.0 - locals.var_temp__blk949);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign51510_e66085, (-locals.var_temp__blk949_dn4), (-locals.var_temp__blk949_dn6), (-locals.var_temp__blk949_dn7), (-locals.var_temp__blk949_dn8), (-locals.var_temp__blk949_dn9), );
            locals.var_temp1_rv = 0.0;
        }

        let assign51520_e66090: f64 = if locals.var_temp1 < 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1504 = assign51520_e66090;
        locals.var_guard1504_rv = 0.0;

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 != 0.0)) {
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp2_rv = 0.0;
        }

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1503 != 0.0)) && (locals.var_guard1504 == 0.0)) {
            let assign51540_e66115: f64 = (locals.var_temp1).sqrt();
            let assign51540_e66116: f64 = (1.0 - assign51540_e66115);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign51540_e66116, (-(locals.var_temp1_dn4 / (2.0 * assign51540_e66115))), (-(locals.var_temp1_dn6 / (2.0 * assign51540_e66115))), (-(locals.var_temp1_dn7 / (2.0 * assign51540_e66115))), (-(locals.var_temp1_dn8 / (2.0 * assign51540_e66115))), (-(locals.var_temp1_dn9 / (2.0 * assign51540_e66115))), );
            locals.var_temp2_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1503 == 0.0)) {
            let assign51550_e66129: f64 = (0.5 * locals.var_temp__blk949);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign51550_e66129, (0.5 * locals.var_temp__blk949_dn4), (0.5 * locals.var_temp__blk949_dn6), (0.5 * locals.var_temp__blk949_dn7), (0.5 * locals.var_temp__blk949_dn8), (0.5 * locals.var_temp__blk949_dn9), );
            locals.var_temp2_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign51560_e66139: f64 = (locals.var_temp2 * locals.var_asat__blk1389);
            (locals.var_x_inf0__blk1390, locals.var_x_inf0__blk1390_dn4, locals.var_x_inf0__blk1390_dn6, locals.var_x_inf0__blk1390_dn7, locals.var_x_inf0__blk1390_dn8, locals.var_x_inf0__blk1390_dn9, ) = (assign51560_e66139, ((locals.var_temp2_dn4 * locals.var_asat__blk1389) + (locals.var_temp2 * locals.var_asat__blk1389_dn4)), ((locals.var_temp2_dn6 * locals.var_asat__blk1389) + (locals.var_temp2 * locals.var_asat__blk1389_dn6)), ((locals.var_temp2_dn7 * locals.var_asat__blk1389) + (locals.var_temp2 * locals.var_asat__blk1389_dn7)), ((locals.var_temp2_dn8 * locals.var_asat__blk1389) + (locals.var_temp2 * locals.var_asat__blk1389_dn8)), ((locals.var_temp2_dn9 * locals.var_asat__blk1389) + (locals.var_temp2 * locals.var_asat__blk1389_dn9)), );
            locals.var_x_inf0__blk1390_rv = 0.0;
        }

        let assign51570_e66148: f64 = if ((locals.var_cs_t > 0.0) && (locals.var_thecs_t > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1505 = assign51570_e66148;
        locals.var_guard1505_rv = 0.0;

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
            let assign51580_e66158: f64 = (0.475 * locals.var_phit1__blk1339);
            let assign51580_e66160: f64 = (assign51580_e66158 * locals.var_x_inf0__blk1390);
            (locals.var_midphi0__blk1391, locals.var_midphi0__blk1391_dn4, locals.var_midphi0__blk1391_dn6, locals.var_midphi0__blk1391_dn7, locals.var_midphi0__blk1391_dn8, locals.var_midphi0__blk1391_dn9, ) = (assign51580_e66160, (((0.475 * locals.var_phit1__blk1339_dn4) * locals.var_x_inf0__blk1390) + (assign51580_e66158 * locals.var_x_inf0__blk1390_dn4)), (((0.475 * locals.var_phit1__blk1339_dn6) * locals.var_x_inf0__blk1390) + (assign51580_e66158 * locals.var_x_inf0__blk1390_dn6)), (((0.475 * locals.var_phit1__blk1339_dn7) * locals.var_x_inf0__blk1390) + (assign51580_e66158 * locals.var_x_inf0__blk1390_dn7)), (((0.475 * locals.var_phit1__blk1339_dn8) * locals.var_x_inf0__blk1390) + (assign51580_e66158 * locals.var_x_inf0__blk1390_dn8)), (((0.475 * locals.var_phit1__blk1339_dn9) * locals.var_x_inf0__blk1390) + (assign51580_e66158 * locals.var_x_inf0__blk1390_dn9)), );
            locals.var_midphi0__blk1391_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
            let assign51590_e66173: f64 = (locals.var_alphas__blk1373 * locals.var_midphi0__blk1391);
            let assign51590_e66174: f64 = (locals.var_qis__blk1376 - assign51590_e66173);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign51590_e66174, (locals.var_qis__blk1376_dn4 - ((locals.var_alphas__blk1373_dn4 * locals.var_midphi0__blk1391) + (locals.var_alphas__blk1373 * locals.var_midphi0__blk1391_dn4))), (locals.var_qis__blk1376_dn6 - ((locals.var_alphas__blk1373_dn6 * locals.var_midphi0__blk1391) + (locals.var_alphas__blk1373 * locals.var_midphi0__blk1391_dn6))), (locals.var_qis__blk1376_dn7 - ((locals.var_alphas__blk1373_dn7 * locals.var_midphi0__blk1391) + (locals.var_alphas__blk1373 * locals.var_midphi0__blk1391_dn7))), (locals.var_qis__blk1376_dn8 - ((locals.var_alphas__blk1373_dn8 * locals.var_midphi0__blk1391) + (locals.var_alphas__blk1373 * locals.var_midphi0__blk1391_dn8))), (locals.var_qis__blk1376_dn9 - ((locals.var_alphas__blk1373_dn9 * locals.var_midphi0__blk1391) + (locals.var_alphas__blk1373 * locals.var_midphi0__blk1391_dn9))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
            let assign51600_e66188: f64 = (locals.var_temp__blk949 * locals.var_temp__blk949);
            let assign51600_e66190: f64 = (assign51600_e66188 + 1e-12);
            let assign51600_e66191: f64 = (assign51600_e66190).sqrt();
            let assign51600_e66192: f64 = (locals.var_temp__blk949 + assign51600_e66191);
            let assign51600_e66193: f64 = (0.5 * assign51600_e66192);
            (locals.var_qisat__blk1392, locals.var_qisat__blk1392_dn4, locals.var_qisat__blk1392_dn6, locals.var_qisat__blk1392_dn7, locals.var_qisat__blk1392_dn8, locals.var_qisat__blk1392_dn9, ) = (assign51600_e66193, (0.5 * (locals.var_temp__blk949_dn4 + (((locals.var_temp__blk949_dn4 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn4)) / (2.0 * assign51600_e66191)))), (0.5 * (locals.var_temp__blk949_dn6 + (((locals.var_temp__blk949_dn6 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn6)) / (2.0 * assign51600_e66191)))), (0.5 * (locals.var_temp__blk949_dn7 + (((locals.var_temp__blk949_dn7 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn7)) / (2.0 * assign51600_e66191)))), (0.5 * (locals.var_temp__blk949_dn8 + (((locals.var_temp__blk949_dn8 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn8)) / (2.0 * assign51600_e66191)))), (0.5 * (locals.var_temp__blk949_dn9 + (((locals.var_temp__blk949_dn9 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn9)) / (2.0 * assign51600_e66191)))), );
            locals.var_qisat__blk1392_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
            let assign51610_e66205: f64 = (locals.var_phit1__blk1339 * locals.var_xgs__blk1375);
            let assign51610_e66207: f64 = (assign51610_e66205 - locals.var_qis__blk1376);
            let assign51610_e66210: f64 = (locals.var_alphas__blk1373 - 1.0);
            let assign51610_e66212: f64 = (assign51610_e66210 * locals.var_midphi0__blk1391);
            let assign51610_e66213: f64 = (assign51610_e66207 + assign51610_e66212);
            (locals.var_qbsat__blk1393, locals.var_qbsat__blk1393_dn4, locals.var_qbsat__blk1393_dn6, locals.var_qbsat__blk1393_dn7, locals.var_qbsat__blk1393_dn8, locals.var_qbsat__blk1393_dn9, ) = (assign51610_e66213, ((((locals.var_phit1__blk1339_dn4 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn4)) - locals.var_qis__blk1376_dn4) + ((locals.var_alphas__blk1373_dn4 * locals.var_midphi0__blk1391) + (assign51610_e66210 * locals.var_midphi0__blk1391_dn4))), ((((locals.var_phit1__blk1339_dn6 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn6)) - locals.var_qis__blk1376_dn6) + ((locals.var_alphas__blk1373_dn6 * locals.var_midphi0__blk1391) + (assign51610_e66210 * locals.var_midphi0__blk1391_dn6))), ((((locals.var_phit1__blk1339_dn7 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn7)) - locals.var_qis__blk1376_dn7) + ((locals.var_alphas__blk1373_dn7 * locals.var_midphi0__blk1391) + (assign51610_e66210 * locals.var_midphi0__blk1391_dn7))), ((((locals.var_phit1__blk1339_dn8 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn8)) - locals.var_qis__blk1376_dn8) + ((locals.var_alphas__blk1373_dn8 * locals.var_midphi0__blk1391) + (assign51610_e66210 * locals.var_midphi0__blk1391_dn8))), ((((locals.var_phit1__blk1339_dn9 * locals.var_xgs__blk1375) + (locals.var_phit1__blk1339 * locals.var_xgs__blk1375_dn9)) - locals.var_qis__blk1376_dn9) + ((locals.var_alphas__blk1373_dn9 * locals.var_midphi0__blk1391) + (assign51610_e66210 * locals.var_midphi0__blk1391_dn9))), );
            locals.var_qbsat__blk1393_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
            let assign51620_e66226: f64 = (0.5 * locals.var_gf2__blk1325);
            let assign51620_e66228: f64 = (assign51620_e66226 * locals.var_phit1__blk1339);
            let assign51620_e66230: f64 = (assign51620_e66228 / locals.var_qbsat__blk1393);
            let assign51620_e66231: f64 = (1.0 + assign51620_e66230);
            (locals.var_alphasat__blk1394, locals.var_alphasat__blk1394_dn4, locals.var_alphasat__blk1394_dn6, locals.var_alphasat__blk1394_dn7, locals.var_alphasat__blk1394_dn8, locals.var_alphasat__blk1394_dn9, ) = (assign51620_e66231, ((((((0.5 * locals.var_gf2__blk1325_dn4) * locals.var_phit1__blk1339) + (assign51620_e66226 * locals.var_phit1__blk1339_dn4)) * locals.var_qbsat__blk1393) - (assign51620_e66228 * locals.var_qbsat__blk1393_dn4)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), ((((((0.5 * locals.var_gf2__blk1325_dn6) * locals.var_phit1__blk1339) + (assign51620_e66226 * locals.var_phit1__blk1339_dn6)) * locals.var_qbsat__blk1393) - (assign51620_e66228 * locals.var_qbsat__blk1393_dn6)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), ((((((0.5 * locals.var_gf2__blk1325_dn7) * locals.var_phit1__blk1339) + (assign51620_e66226 * locals.var_phit1__blk1339_dn7)) * locals.var_qbsat__blk1393) - (assign51620_e66228 * locals.var_qbsat__blk1393_dn7)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), ((((((0.5 * locals.var_gf2__blk1325_dn8) * locals.var_phit1__blk1339) + (assign51620_e66226 * locals.var_phit1__blk1339_dn8)) * locals.var_qbsat__blk1393) - (assign51620_e66228 * locals.var_qbsat__blk1393_dn8)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), ((((((0.5 * locals.var_gf2__blk1325_dn9) * locals.var_phit1__blk1339) + (assign51620_e66226 * locals.var_phit1__blk1339_dn9)) * locals.var_qbsat__blk1393) - (assign51620_e66228 * locals.var_qbsat__blk1393_dn9)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), );
            locals.var_alphasat__blk1394_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
            let assign51630_e66244: f64 = (locals.var_eta_mu * locals.var_qisat__blk1392);
            let assign51630_e66245: f64 = (locals.var_qbsat__blk1393 + assign51630_e66244);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign51630_e66245, (locals.var_qbsat__blk1393_dn4 + (locals.var_eta_mu * locals.var_qisat__blk1392_dn4)), (locals.var_qbsat__blk1393_dn6 + (locals.var_eta_mu * locals.var_qisat__blk1392_dn6)), (locals.var_qbsat__blk1393_dn7 + (locals.var_eta_mu * locals.var_qisat__blk1392_dn7)), (locals.var_qbsat__blk1393_dn8 + (locals.var_eta_mu * locals.var_qisat__blk1392_dn8)), (locals.var_qbsat__blk1393_dn9 + (locals.var_eta_mu * locals.var_qisat__blk1392_dn9)), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
            let assign51640_e66257: f64 = (locals.var_e_eff0 * locals.var_temp__blk949);
            let assign51640_e66259: f64 = (assign51640_e66257 * locals.var_mue_t);
            let assign51640_e66261: f64 = (assign51640_e66259).powf(locals.var_themu_t);
            (locals.var_gmobmusat__blk1395, locals.var_gmobmusat__blk1395_dn4, locals.var_gmobmusat__blk1395_dn6, locals.var_gmobmusat__blk1395_dn7, locals.var_gmobmusat__blk1395_dn8, locals.var_gmobmusat__blk1395_dn9, ) = (assign51640_e66261, if locals.var_themu_t_dn4 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51640_e66259).powf(locals.var_themu_t - 1.0) * (((locals.var_e_eff0 * locals.var_temp__blk949_dn4) * locals.var_mue_t) + (assign51640_e66257 * locals.var_mue_t_dn4)))) } } else { (assign51640_e66261 * ((locals.var_themu_t_dn4 * (assign51640_e66259).ln()) + (locals.var_themu_t * ((((locals.var_e_eff0 * locals.var_temp__blk949_dn4) * locals.var_mue_t) + (assign51640_e66257 * locals.var_mue_t_dn4)) / assign51640_e66259)))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51640_e66259).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk949_dn6) * locals.var_mue_t))) } } else { (assign51640_e66261 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk949_dn6) * locals.var_mue_t) / assign51640_e66259))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51640_e66259).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk949_dn7) * locals.var_mue_t))) } } else { (assign51640_e66261 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk949_dn7) * locals.var_mue_t) / assign51640_e66259))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51640_e66259).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk949_dn8) * locals.var_mue_t))) } } else { (assign51640_e66261 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk949_dn8) * locals.var_mue_t) / assign51640_e66259))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51640_e66259).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk949_dn9) * locals.var_mue_t))) } } else { (assign51640_e66261 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk949_dn9) * locals.var_mue_t) / assign51640_e66259))) }, );
            locals.var_gmobmusat__blk1395_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
            let assign51650_e66275: f64 = (1.0 - locals.var_eta_mu);
            let assign51650_e66276: f64 = (locals.var_alphasat__blk1394 * assign51650_e66275);
            let assign51650_e66278: f64 = (assign51650_e66276 - 1.0);
            let assign51650_e66279: f64 = (locals.var_themu_t * assign51650_e66278);
            let assign51650_e66281: f64 = (assign51650_e66279 / locals.var_temp__blk949);
            let assign51650_e66283: f64 = (assign51650_e66281 * locals.var_gmobmusat__blk1395);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign51650_e66283, (((((((locals.var_themu_t_dn4 * assign51650_e66278) + (locals.var_themu_t * (locals.var_alphasat__blk1394_dn4 * assign51650_e66275))) * locals.var_temp__blk949) - (assign51650_e66279 * locals.var_temp__blk949_dn4)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat__blk1395) + (assign51650_e66281 * locals.var_gmobmusat__blk1395_dn4)), ((((((locals.var_themu_t * (locals.var_alphasat__blk1394_dn6 * assign51650_e66275)) * locals.var_temp__blk949) - (assign51650_e66279 * locals.var_temp__blk949_dn6)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat__blk1395) + (assign51650_e66281 * locals.var_gmobmusat__blk1395_dn6)), ((((((locals.var_themu_t * (locals.var_alphasat__blk1394_dn7 * assign51650_e66275)) * locals.var_temp__blk949) - (assign51650_e66279 * locals.var_temp__blk949_dn7)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat__blk1395) + (assign51650_e66281 * locals.var_gmobmusat__blk1395_dn7)), ((((((locals.var_themu_t * (locals.var_alphasat__blk1394_dn8 * assign51650_e66275)) * locals.var_temp__blk949) - (assign51650_e66279 * locals.var_temp__blk949_dn8)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat__blk1395) + (assign51650_e66281 * locals.var_gmobmusat__blk1395_dn8)), ((((((locals.var_themu_t * (locals.var_alphasat__blk1394_dn9 * assign51650_e66275)) * locals.var_temp__blk949) - (assign51650_e66279 * locals.var_temp__blk949_dn9)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat__blk1395) + (assign51650_e66281 * locals.var_gmobmusat__blk1395_dn9)), );
            locals.var_temp1_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
            let assign51660_e66295: f64 = (locals.var_qisat__blk1392 / locals.var_qbsat__blk1393);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign51660_e66295, (((locals.var_qisat__blk1392_dn4 * locals.var_qbsat__blk1393) - (locals.var_qisat__blk1392 * locals.var_qbsat__blk1393_dn4)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), (((locals.var_qisat__blk1392_dn6 * locals.var_qbsat__blk1393) - (locals.var_qisat__blk1392 * locals.var_qbsat__blk1393_dn6)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), (((locals.var_qisat__blk1392_dn7 * locals.var_qbsat__blk1393) - (locals.var_qisat__blk1392 * locals.var_qbsat__blk1393_dn7)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), (((locals.var_qisat__blk1392_dn8 * locals.var_qbsat__blk1393) - (locals.var_qisat__blk1392 * locals.var_qbsat__blk1393_dn8)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), (((locals.var_qisat__blk1392_dn9 * locals.var_qbsat__blk1393) - (locals.var_qisat__blk1392 * locals.var_qbsat__blk1393_dn9)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
            let assign51670_e66308: f64 = (1.0 + locals.var_temp__blk949);
            let assign51670_e66310: f64 = (-locals.var_thecs_t);
            let assign51670_e66311: f64 = (assign51670_e66308).powf(assign51670_e66310);
            let assign51670_e66312: f64 = (locals.var_cs_t * assign51670_e66311);
            (locals.var_gmobcssat__blk1396, locals.var_gmobcssat__blk1396_dn4, locals.var_gmobcssat__blk1396_dn6, locals.var_gmobcssat__blk1396_dn7, locals.var_gmobcssat__blk1396_dn8, locals.var_gmobcssat__blk1396_dn9, ) = (assign51670_e66312, ((locals.var_cs_t_dn4 * assign51670_e66311) + (locals.var_cs_t * if (-locals.var_thecs_t_dn4) == 0.0 && ((assign51670_e66310) as f64).is_finite() && ((assign51670_e66310) as f64).fract() == 0.0 { if assign51670_e66310 == 0.0 { 0.0 } else { (assign51670_e66310 * ((assign51670_e66308).powf(assign51670_e66310 - 1.0) * locals.var_temp__blk949_dn4)) } } else { (assign51670_e66311 * (((-locals.var_thecs_t_dn4) * (assign51670_e66308).ln()) + (assign51670_e66310 * (locals.var_temp__blk949_dn4 / assign51670_e66308)))) })), (locals.var_cs_t * if 0.0 == 0.0 && ((assign51670_e66310) as f64).is_finite() && ((assign51670_e66310) as f64).fract() == 0.0 { if assign51670_e66310 == 0.0 { 0.0 } else { (assign51670_e66310 * ((assign51670_e66308).powf(assign51670_e66310 - 1.0) * locals.var_temp__blk949_dn6)) } } else { (assign51670_e66311 * (assign51670_e66310 * (locals.var_temp__blk949_dn6 / assign51670_e66308))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign51670_e66310) as f64).is_finite() && ((assign51670_e66310) as f64).fract() == 0.0 { if assign51670_e66310 == 0.0 { 0.0 } else { (assign51670_e66310 * ((assign51670_e66308).powf(assign51670_e66310 - 1.0) * locals.var_temp__blk949_dn7)) } } else { (assign51670_e66311 * (assign51670_e66310 * (locals.var_temp__blk949_dn7 / assign51670_e66308))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign51670_e66310) as f64).is_finite() && ((assign51670_e66310) as f64).fract() == 0.0 { if assign51670_e66310 == 0.0 { 0.0 } else { (assign51670_e66310 * ((assign51670_e66308).powf(assign51670_e66310 - 1.0) * locals.var_temp__blk949_dn8)) } } else { (assign51670_e66311 * (assign51670_e66310 * (locals.var_temp__blk949_dn8 / assign51670_e66308))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign51670_e66310) as f64).is_finite() && ((assign51670_e66310) as f64).fract() == 0.0 { if assign51670_e66310 == 0.0 { 0.0 } else { (assign51670_e66310 * ((assign51670_e66308).powf(assign51670_e66310 - 1.0) * locals.var_temp__blk949_dn9)) } } else { (assign51670_e66311 * (assign51670_e66310 * (locals.var_temp__blk949_dn9 / assign51670_e66308))) }), );
            locals.var_gmobcssat__blk1396_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
            let assign51680_e66325: f64 = (locals.var_alphasat__blk1394 - 1.0);
            let assign51680_e66329: f64 = (locals.var_temp__blk949 + 1.0);
            let assign51680_e66330: f64 = (1.0 / assign51680_e66329);
            let assign51680_e66331: f64 = (assign51680_e66325 + assign51680_e66330);
            let assign51680_e66332: f64 = (locals.var_thecs_t * assign51680_e66331);
            let assign51680_e66334: f64 = (assign51680_e66332 / locals.var_qbsat__blk1393);
            let assign51680_e66336: f64 = (assign51680_e66334 * locals.var_gmobcssat__blk1396);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign51680_e66336, (((((((locals.var_thecs_t_dn4 * assign51680_e66331) + (locals.var_thecs_t * (locals.var_alphasat__blk1394_dn4 + (-(locals.var_temp__blk949_dn4 / (assign51680_e66329 * assign51680_e66329)))))) * locals.var_qbsat__blk1393) - (assign51680_e66332 * locals.var_qbsat__blk1393_dn4)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)) * locals.var_gmobcssat__blk1396) + (assign51680_e66334 * locals.var_gmobcssat__blk1396_dn4)), ((((((locals.var_thecs_t * (locals.var_alphasat__blk1394_dn6 + (-(locals.var_temp__blk949_dn6 / (assign51680_e66329 * assign51680_e66329))))) * locals.var_qbsat__blk1393) - (assign51680_e66332 * locals.var_qbsat__blk1393_dn6)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)) * locals.var_gmobcssat__blk1396) + (assign51680_e66334 * locals.var_gmobcssat__blk1396_dn6)), ((((((locals.var_thecs_t * (locals.var_alphasat__blk1394_dn7 + (-(locals.var_temp__blk949_dn7 / (assign51680_e66329 * assign51680_e66329))))) * locals.var_qbsat__blk1393) - (assign51680_e66332 * locals.var_qbsat__blk1393_dn7)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)) * locals.var_gmobcssat__blk1396) + (assign51680_e66334 * locals.var_gmobcssat__blk1396_dn7)), ((((((locals.var_thecs_t * (locals.var_alphasat__blk1394_dn8 + (-(locals.var_temp__blk949_dn8 / (assign51680_e66329 * assign51680_e66329))))) * locals.var_qbsat__blk1393) - (assign51680_e66332 * locals.var_qbsat__blk1393_dn8)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)) * locals.var_gmobcssat__blk1396) + (assign51680_e66334 * locals.var_gmobcssat__blk1396_dn8)), ((((((locals.var_thecs_t * (locals.var_alphasat__blk1394_dn9 + (-(locals.var_temp__blk949_dn9 / (assign51680_e66329 * assign51680_e66329))))) * locals.var_qbsat__blk1393) - (assign51680_e66332 * locals.var_qbsat__blk1393_dn9)) / (locals.var_qbsat__blk1393 * locals.var_qbsat__blk1393)) * locals.var_gmobcssat__blk1396) + (assign51680_e66334 * locals.var_gmobcssat__blk1396_dn9)), );
            locals.var_temp2_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
            let assign51690_e66348: f64 = (locals.var_ther_i * locals.var_rhob__blk1378);
            let assign51690_e66350: f64 = (assign51690_e66348 * locals.var_rhog__blk1379);
            let assign51690_e66352: f64 = (assign51690_e66350 * locals.var_qisat__blk1392);
            (locals.var_grsat__blk1397, locals.var_grsat__blk1397_dn4, locals.var_grsat__blk1397_dn6, locals.var_grsat__blk1397_dn7, locals.var_grsat__blk1397_dn8, locals.var_grsat__blk1397_dn9, ) = (assign51690_e66352, ((((((locals.var_ther_i_dn4 * locals.var_rhob__blk1378) + (locals.var_ther_i * locals.var_rhob__blk1378_dn4)) * locals.var_rhog__blk1379) + (assign51690_e66348 * locals.var_rhog__blk1379_dn4)) * locals.var_qisat__blk1392) + (assign51690_e66350 * locals.var_qisat__blk1392_dn4)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn6) * locals.var_rhog__blk1379) + (assign51690_e66348 * locals.var_rhog__blk1379_dn6)) * locals.var_qisat__blk1392) + (assign51690_e66350 * locals.var_qisat__blk1392_dn6)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn7) * locals.var_rhog__blk1379) + (assign51690_e66348 * locals.var_rhog__blk1379_dn7)) * locals.var_qisat__blk1392) + (assign51690_e66350 * locals.var_qisat__blk1392_dn7)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn8) * locals.var_rhog__blk1379) + (assign51690_e66348 * locals.var_rhog__blk1379_dn8)) * locals.var_qisat__blk1392) + (assign51690_e66350 * locals.var_qisat__blk1392_dn8)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn9) * locals.var_rhog__blk1379) + (assign51690_e66348 * locals.var_rhog__blk1379_dn9)) * locals.var_qisat__blk1392) + (assign51690_e66350 * locals.var_qisat__blk1392_dn9)), );
            locals.var_grsat__blk1397_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
            let assign51700_e66366: f64 = (locals.var_ther_i * locals.var_rhob__blk1378);
            let assign51700_e66368: f64 = (assign51700_e66366 * locals.var_rhog__blk1379);
            let assign51700_e66370: f64 = (assign51700_e66368 * locals.var_alphasat__blk1394);
            let assign51700_e66371: f64 = (locals.var_temp1 - assign51700_e66370);
            let assign51700_e66373: f64 = (assign51700_e66371 / locals.var_temp2);
            let assign51700_e66374: f64 = (1.0 + assign51700_e66373);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign51700_e66374, ((((locals.var_temp1_dn4 - ((((((locals.var_ther_i_dn4 * locals.var_rhob__blk1378) + (locals.var_ther_i * locals.var_rhob__blk1378_dn4)) * locals.var_rhog__blk1379) + (assign51700_e66366 * locals.var_rhog__blk1379_dn4)) * locals.var_alphasat__blk1394) + (assign51700_e66368 * locals.var_alphasat__blk1394_dn4))) * locals.var_temp2) - (assign51700_e66371 * locals.var_temp2_dn4)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn6 - (((((locals.var_ther_i * locals.var_rhob__blk1378_dn6) * locals.var_rhog__blk1379) + (assign51700_e66366 * locals.var_rhog__blk1379_dn6)) * locals.var_alphasat__blk1394) + (assign51700_e66368 * locals.var_alphasat__blk1394_dn6))) * locals.var_temp2) - (assign51700_e66371 * locals.var_temp2_dn6)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn7 - (((((locals.var_ther_i * locals.var_rhob__blk1378_dn7) * locals.var_rhog__blk1379) + (assign51700_e66366 * locals.var_rhog__blk1379_dn7)) * locals.var_alphasat__blk1394) + (assign51700_e66368 * locals.var_alphasat__blk1394_dn7))) * locals.var_temp2) - (assign51700_e66371 * locals.var_temp2_dn7)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn8 - (((((locals.var_ther_i * locals.var_rhob__blk1378_dn8) * locals.var_rhog__blk1379) + (assign51700_e66366 * locals.var_rhog__blk1379_dn8)) * locals.var_alphasat__blk1394) + (assign51700_e66368 * locals.var_alphasat__blk1394_dn8))) * locals.var_temp2) - (assign51700_e66371 * locals.var_temp2_dn8)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn9 - (((((locals.var_ther_i * locals.var_rhob__blk1378_dn9) * locals.var_rhog__blk1379) + (assign51700_e66366 * locals.var_rhog__blk1379_dn9)) * locals.var_alphasat__blk1394) + (assign51700_e66368 * locals.var_alphasat__blk1394_dn9))) * locals.var_temp2) - (assign51700_e66371 * locals.var_temp2_dn9)) / (locals.var_temp2 * locals.var_temp2)), );
            locals.var_temp__blk949_rv = 0.0;
        }

        let assign51710_e66379: f64 = if locals.var_temp__blk949 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1506 = assign51710_e66379;
        locals.var_guard1506_rv = 0.0;

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1506 != 0.0)) {
            let assign51720_e66393: f64 = (2.0 * locals.var_temp__blk949);
            let assign51720_e66394: f64 = (assign51720_e66393).exp();
            let assign51720_e66395: f64 = (1.0 + assign51720_e66394);
            let assign51720_e66396: f64 = (assign51720_e66395).ln();
            let assign51720_e66397: f64 = (0.5 * assign51720_e66396);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign51720_e66397, (0.5 * ((assign51720_e66394 * (2.0 * locals.var_temp__blk949_dn4)) / assign51720_e66395)), (0.5 * ((assign51720_e66394 * (2.0 * locals.var_temp__blk949_dn6)) / assign51720_e66395)), (0.5 * ((assign51720_e66394 * (2.0 * locals.var_temp__blk949_dn7)) / assign51720_e66395)), (0.5 * ((assign51720_e66394 * (2.0 * locals.var_temp__blk949_dn8)) / assign51720_e66395)), (0.5 * ((assign51720_e66394 * (2.0 * locals.var_temp__blk949_dn9)) / assign51720_e66395)), );
            locals.var_temp1_rv = 0.0;
        }

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) && (locals.var_guard1506 == 0.0)) {
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, );
            locals.var_temp1_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
            let assign51740_e66421: f64 = (-locals.var_midphi0__blk1391);
            let assign51740_e66423: f64 = (assign51740_e66421 * locals.var_temp2);
            let assign51740_e66425: f64 = (assign51740_e66423 * locals.var_temp1);
            let assign51740_e66428: f64 = (1.0 + locals.var_gmobmusat__blk1395);
            let assign51740_e66430: f64 = (assign51740_e66428 + locals.var_gmobcssat__blk1396);
            let assign51740_e66432: f64 = (assign51740_e66430 + locals.var_grsat__blk1397);
            let assign51740_e66433: f64 = (assign51740_e66425 / assign51740_e66432);
            (locals.var_delta_gmob__blk1398, locals.var_delta_gmob__blk1398_dn4, locals.var_delta_gmob__blk1398_dn6, locals.var_delta_gmob__blk1398_dn7, locals.var_delta_gmob__blk1398_dn8, locals.var_delta_gmob__blk1398_dn9, ) = (assign51740_e66433, ((((((((-locals.var_midphi0__blk1391_dn4) * locals.var_temp2) + (assign51740_e66421 * locals.var_temp2_dn4)) * locals.var_temp1) + (assign51740_e66423 * locals.var_temp1_dn4)) * assign51740_e66432) - (assign51740_e66425 * ((locals.var_gmobmusat__blk1395_dn4 + locals.var_gmobcssat__blk1396_dn4) + locals.var_grsat__blk1397_dn4))) / (assign51740_e66432 * assign51740_e66432)), ((((((((-locals.var_midphi0__blk1391_dn6) * locals.var_temp2) + (assign51740_e66421 * locals.var_temp2_dn6)) * locals.var_temp1) + (assign51740_e66423 * locals.var_temp1_dn6)) * assign51740_e66432) - (assign51740_e66425 * ((locals.var_gmobmusat__blk1395_dn6 + locals.var_gmobcssat__blk1396_dn6) + locals.var_grsat__blk1397_dn6))) / (assign51740_e66432 * assign51740_e66432)), ((((((((-locals.var_midphi0__blk1391_dn7) * locals.var_temp2) + (assign51740_e66421 * locals.var_temp2_dn7)) * locals.var_temp1) + (assign51740_e66423 * locals.var_temp1_dn7)) * assign51740_e66432) - (assign51740_e66425 * ((locals.var_gmobmusat__blk1395_dn7 + locals.var_gmobcssat__blk1396_dn7) + locals.var_grsat__blk1397_dn7))) / (assign51740_e66432 * assign51740_e66432)), ((((((((-locals.var_midphi0__blk1391_dn8) * locals.var_temp2) + (assign51740_e66421 * locals.var_temp2_dn8)) * locals.var_temp1) + (assign51740_e66423 * locals.var_temp1_dn8)) * assign51740_e66432) - (assign51740_e66425 * ((locals.var_gmobmusat__blk1395_dn8 + locals.var_gmobcssat__blk1396_dn8) + locals.var_grsat__blk1397_dn8))) / (assign51740_e66432 * assign51740_e66432)), ((((((((-locals.var_midphi0__blk1391_dn9) * locals.var_temp2) + (assign51740_e66421 * locals.var_temp2_dn9)) * locals.var_temp1) + (assign51740_e66423 * locals.var_temp1_dn9)) * assign51740_e66432) - (assign51740_e66425 * ((locals.var_gmobmusat__blk1395_dn9 + locals.var_gmobcssat__blk1396_dn9) + locals.var_grsat__blk1397_dn9))) / (assign51740_e66432 * assign51740_e66432)), );
            locals.var_delta_gmob__blk1398_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 != 0.0)) {
            let assign51750_e66450: f64 = (locals.var_delta_gmob__blk1398 * locals.var_delta_gmob__blk1398);
            let assign51750_e66451: f64 = (1.0 + assign51750_e66450);
            let assign51750_e66452: f64 = (assign51750_e66451).sqrt();
            let assign51750_e66453: f64 = (1.0 + assign51750_e66452);
            let assign51750_e66454: f64 = (locals.var_delta_gmob__blk1398 / assign51750_e66453);
            let assign51750_e66455: f64 = (1.0 + assign51750_e66454);
            let assign51750_e66456: f64 = (locals.var_x_inf0__blk1390 * assign51750_e66455);
            (locals.var_x_inf__blk1399, locals.var_x_inf__blk1399_dn4, locals.var_x_inf__blk1399_dn6, locals.var_x_inf__blk1399_dn7, locals.var_x_inf__blk1399_dn8, locals.var_x_inf__blk1399_dn9, ) = (assign51750_e66456, ((locals.var_x_inf0__blk1390_dn4 * assign51750_e66455) + (locals.var_x_inf0__blk1390 * (((locals.var_delta_gmob__blk1398_dn4 * assign51750_e66453) - (locals.var_delta_gmob__blk1398 * (((locals.var_delta_gmob__blk1398_dn4 * locals.var_delta_gmob__blk1398) + (locals.var_delta_gmob__blk1398 * locals.var_delta_gmob__blk1398_dn4)) / (2.0 * assign51750_e66452)))) / (assign51750_e66453 * assign51750_e66453)))), ((locals.var_x_inf0__blk1390_dn6 * assign51750_e66455) + (locals.var_x_inf0__blk1390 * (((locals.var_delta_gmob__blk1398_dn6 * assign51750_e66453) - (locals.var_delta_gmob__blk1398 * (((locals.var_delta_gmob__blk1398_dn6 * locals.var_delta_gmob__blk1398) + (locals.var_delta_gmob__blk1398 * locals.var_delta_gmob__blk1398_dn6)) / (2.0 * assign51750_e66452)))) / (assign51750_e66453 * assign51750_e66453)))), ((locals.var_x_inf0__blk1390_dn7 * assign51750_e66455) + (locals.var_x_inf0__blk1390 * (((locals.var_delta_gmob__blk1398_dn7 * assign51750_e66453) - (locals.var_delta_gmob__blk1398 * (((locals.var_delta_gmob__blk1398_dn7 * locals.var_delta_gmob__blk1398) + (locals.var_delta_gmob__blk1398 * locals.var_delta_gmob__blk1398_dn7)) / (2.0 * assign51750_e66452)))) / (assign51750_e66453 * assign51750_e66453)))), ((locals.var_x_inf0__blk1390_dn8 * assign51750_e66455) + (locals.var_x_inf0__blk1390 * (((locals.var_delta_gmob__blk1398_dn8 * assign51750_e66453) - (locals.var_delta_gmob__blk1398 * (((locals.var_delta_gmob__blk1398_dn8 * locals.var_delta_gmob__blk1398) + (locals.var_delta_gmob__blk1398 * locals.var_delta_gmob__blk1398_dn8)) / (2.0 * assign51750_e66452)))) / (assign51750_e66453 * assign51750_e66453)))), ((locals.var_x_inf0__blk1390_dn9 * assign51750_e66455) + (locals.var_x_inf0__blk1390 * (((locals.var_delta_gmob__blk1398_dn9 * assign51750_e66453) - (locals.var_delta_gmob__blk1398 * (((locals.var_delta_gmob__blk1398_dn9 * locals.var_delta_gmob__blk1398) + (locals.var_delta_gmob__blk1398 * locals.var_delta_gmob__blk1398_dn9)) / (2.0 * assign51750_e66452)))) / (assign51750_e66453 * assign51750_e66453)))), );
            locals.var_x_inf__blk1399_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1505 == 0.0)) {
            (locals.var_x_inf__blk1399, locals.var_x_inf__blk1399_dn4, locals.var_x_inf__blk1399_dn6, locals.var_x_inf__blk1399_dn7, locals.var_x_inf__blk1399_dn8, locals.var_x_inf__blk1399_dn9, ) = (locals.var_x_inf0__blk1390, locals.var_x_inf0__blk1390_dn4, locals.var_x_inf0__blk1390_dn6, locals.var_x_inf0__blk1390_dn7, locals.var_x_inf0__blk1390_dn8, locals.var_x_inf0__blk1390_dn9, );
            locals.var_x_inf__blk1399_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign51770_e66477: f64 = (locals.var_phit1__blk1339 * locals.var_thesat1__blk1388);
            let assign51770_e66479: f64 = (assign51770_e66477 * locals.var_x_inf__blk1399);
            let assign51770_e66481: f64 = (assign51770_e66479 * 0.7071067811865475);
            (locals.var_ysat__blk1400, locals.var_ysat__blk1400_dn4, locals.var_ysat__blk1400_dn6, locals.var_ysat__blk1400_dn7, locals.var_ysat__blk1400_dn8, locals.var_ysat__blk1400_dn9, ) = (assign51770_e66481, (((((locals.var_phit1__blk1339_dn4 * locals.var_thesat1__blk1388) + (locals.var_phit1__blk1339 * locals.var_thesat1__blk1388_dn4)) * locals.var_x_inf__blk1399) + (assign51770_e66477 * locals.var_x_inf__blk1399_dn4)) * 0.7071067811865475), (((((locals.var_phit1__blk1339_dn6 * locals.var_thesat1__blk1388) + (locals.var_phit1__blk1339 * locals.var_thesat1__blk1388_dn6)) * locals.var_x_inf__blk1399) + (assign51770_e66477 * locals.var_x_inf__blk1399_dn6)) * 0.7071067811865475), (((((locals.var_phit1__blk1339_dn7 * locals.var_thesat1__blk1388) + (locals.var_phit1__blk1339 * locals.var_thesat1__blk1388_dn7)) * locals.var_x_inf__blk1399) + (assign51770_e66477 * locals.var_x_inf__blk1399_dn7)) * 0.7071067811865475), (((((locals.var_phit1__blk1339_dn8 * locals.var_thesat1__blk1388) + (locals.var_phit1__blk1339 * locals.var_thesat1__blk1388_dn8)) * locals.var_x_inf__blk1399) + (assign51770_e66477 * locals.var_x_inf__blk1399_dn8)) * 0.7071067811865475), (((((locals.var_phit1__blk1339_dn9 * locals.var_thesat1__blk1388) + (locals.var_phit1__blk1339 * locals.var_thesat1__blk1388_dn9)) * locals.var_x_inf__blk1399) + (assign51770_e66477 * locals.var_x_inf__blk1399_dn9)) * 0.7071067811865475), );
            locals.var_ysat__blk1400_rv = 0.0;
        }

        let assign51780_e66486: f64 = (-1.0);
        let assign51780_e66487: f64 = if locals.var_chnl_type == assign51780_e66486 { 1.0 } else { 0.0 };
        locals.var_guard1507 = assign51780_e66487;
        locals.var_guard1507_rv = 0.0;

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) && (locals.var_guard1507 != 0.0)) {
            let assign51790_e66498: f64 = (1.0 + locals.var_ysat__blk1400);
            let assign51790_e66499: f64 = (assign51790_e66498).sqrt();
            let assign51790_e66500: f64 = (locals.var_ysat__blk1400 / assign51790_e66499);
            (locals.var_ysat__blk1400, locals.var_ysat__blk1400_dn4, locals.var_ysat__blk1400_dn6, locals.var_ysat__blk1400_dn7, locals.var_ysat__blk1400_dn8, locals.var_ysat__blk1400_dn9, ) = (assign51790_e66500, (((locals.var_ysat__blk1400_dn4 * assign51790_e66499) - (locals.var_ysat__blk1400 * (locals.var_ysat__blk1400_dn4 / (2.0 * assign51790_e66499)))) / (assign51790_e66499 * assign51790_e66499)), (((locals.var_ysat__blk1400_dn6 * assign51790_e66499) - (locals.var_ysat__blk1400 * (locals.var_ysat__blk1400_dn6 / (2.0 * assign51790_e66499)))) / (assign51790_e66499 * assign51790_e66499)), (((locals.var_ysat__blk1400_dn7 * assign51790_e66499) - (locals.var_ysat__blk1400 * (locals.var_ysat__blk1400_dn7 / (2.0 * assign51790_e66499)))) / (assign51790_e66499 * assign51790_e66499)), (((locals.var_ysat__blk1400_dn8 * assign51790_e66499) - (locals.var_ysat__blk1400 * (locals.var_ysat__blk1400_dn8 / (2.0 * assign51790_e66499)))) / (assign51790_e66499 * assign51790_e66499)), (((locals.var_ysat__blk1400_dn9 * assign51790_e66499) - (locals.var_ysat__blk1400 * (locals.var_ysat__blk1400_dn9 / (2.0 * assign51790_e66499)))) / (assign51790_e66499 * assign51790_e66499)), );
            locals.var_ysat__blk1400_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign51800_e66513: f64 = (4.0 * locals.var_ysat__blk1400);
            let assign51800_e66514: f64 = (1.0 + assign51800_e66513);
            let assign51800_e66515: f64 = (assign51800_e66514).sqrt();
            let assign51800_e66516: f64 = (1.0 + assign51800_e66515);
            let assign51800_e66517: f64 = (2.0 / assign51800_e66516);
            (locals.var_za__blk1401, locals.var_za__blk1401_dn4, locals.var_za__blk1401_dn6, locals.var_za__blk1401_dn7, locals.var_za__blk1401_dn8, locals.var_za__blk1401_dn9, ) = (assign51800_e66517, (-((2.0 * ((4.0 * locals.var_ysat__blk1400_dn4) / (2.0 * assign51800_e66515))) / (assign51800_e66516 * assign51800_e66516))), (-((2.0 * ((4.0 * locals.var_ysat__blk1400_dn6) / (2.0 * assign51800_e66515))) / (assign51800_e66516 * assign51800_e66516))), (-((2.0 * ((4.0 * locals.var_ysat__blk1400_dn7) / (2.0 * assign51800_e66515))) / (assign51800_e66516 * assign51800_e66516))), (-((2.0 * ((4.0 * locals.var_ysat__blk1400_dn8) / (2.0 * assign51800_e66515))) / (assign51800_e66516 * assign51800_e66516))), (-((2.0 * ((4.0 * locals.var_ysat__blk1400_dn9) / (2.0 * assign51800_e66515))) / (assign51800_e66516 * assign51800_e66516))), );
            locals.var_za__blk1401_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign51810_e66527: f64 = (locals.var_za__blk1401 * locals.var_ysat__blk1400);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign51810_e66527, ((locals.var_za__blk1401_dn4 * locals.var_ysat__blk1400) + (locals.var_za__blk1401 * locals.var_ysat__blk1400_dn4)), ((locals.var_za__blk1401_dn6 * locals.var_ysat__blk1400) + (locals.var_za__blk1401 * locals.var_ysat__blk1400_dn6)), ((locals.var_za__blk1401_dn7 * locals.var_ysat__blk1400) + (locals.var_za__blk1401 * locals.var_ysat__blk1400_dn7)), ((locals.var_za__blk1401_dn8 * locals.var_ysat__blk1400) + (locals.var_za__blk1401 * locals.var_ysat__blk1400_dn8)), ((locals.var_za__blk1401_dn9 * locals.var_ysat__blk1400) + (locals.var_za__blk1401 * locals.var_ysat__blk1400_dn9)), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign51820_e66537: f64 = (locals.var_x_inf__blk1399 * locals.var_za__blk1401);
            let assign51820_e66541: f64 = (0.86 * locals.var_temp__blk949);
            let assign51820_e66545: f64 = (locals.var_temp__blk949 * locals.var_za__blk1401);
            let assign51820_e66546: f64 = (1.0 - assign51820_e66545);
            let assign51820_e66547: f64 = (assign51820_e66541 * assign51820_e66546);
            let assign51820_e66551: f64 = (4.0 * locals.var_temp__blk949);
            let assign51820_e66553: f64 = (assign51820_e66551 * locals.var_temp__blk949);
            let assign51820_e66555: f64 = (assign51820_e66553 * locals.var_za__blk1401);
            let assign51820_e66556: f64 = (1.0 + assign51820_e66555);
            let assign51820_e66557: f64 = (assign51820_e66547 / assign51820_e66556);
            let assign51820_e66558: f64 = (1.0 + assign51820_e66557);
            let assign51820_e66559: f64 = (assign51820_e66537 * assign51820_e66558);
            (locals.var_x_0__blk1402, locals.var_x_0__blk1402_dn4, locals.var_x_0__blk1402_dn6, locals.var_x_0__blk1402_dn7, locals.var_x_0__blk1402_dn8, locals.var_x_0__blk1402_dn9, ) = (assign51820_e66559, ((((locals.var_x_inf__blk1399_dn4 * locals.var_za__blk1401) + (locals.var_x_inf__blk1399 * locals.var_za__blk1401_dn4)) * assign51820_e66558) + (assign51820_e66537 * ((((((0.86 * locals.var_temp__blk949_dn4) * assign51820_e66546) + (assign51820_e66541 * (-((locals.var_temp__blk949_dn4 * locals.var_za__blk1401) + (locals.var_temp__blk949 * locals.var_za__blk1401_dn4))))) * assign51820_e66556) - (assign51820_e66547 * (((((4.0 * locals.var_temp__blk949_dn4) * locals.var_temp__blk949) + (assign51820_e66551 * locals.var_temp__blk949_dn4)) * locals.var_za__blk1401) + (assign51820_e66553 * locals.var_za__blk1401_dn4)))) / (assign51820_e66556 * assign51820_e66556)))), ((((locals.var_x_inf__blk1399_dn6 * locals.var_za__blk1401) + (locals.var_x_inf__blk1399 * locals.var_za__blk1401_dn6)) * assign51820_e66558) + (assign51820_e66537 * ((((((0.86 * locals.var_temp__blk949_dn6) * assign51820_e66546) + (assign51820_e66541 * (-((locals.var_temp__blk949_dn6 * locals.var_za__blk1401) + (locals.var_temp__blk949 * locals.var_za__blk1401_dn6))))) * assign51820_e66556) - (assign51820_e66547 * (((((4.0 * locals.var_temp__blk949_dn6) * locals.var_temp__blk949) + (assign51820_e66551 * locals.var_temp__blk949_dn6)) * locals.var_za__blk1401) + (assign51820_e66553 * locals.var_za__blk1401_dn6)))) / (assign51820_e66556 * assign51820_e66556)))), ((((locals.var_x_inf__blk1399_dn7 * locals.var_za__blk1401) + (locals.var_x_inf__blk1399 * locals.var_za__blk1401_dn7)) * assign51820_e66558) + (assign51820_e66537 * ((((((0.86 * locals.var_temp__blk949_dn7) * assign51820_e66546) + (assign51820_e66541 * (-((locals.var_temp__blk949_dn7 * locals.var_za__blk1401) + (locals.var_temp__blk949 * locals.var_za__blk1401_dn7))))) * assign51820_e66556) - (assign51820_e66547 * (((((4.0 * locals.var_temp__blk949_dn7) * locals.var_temp__blk949) + (assign51820_e66551 * locals.var_temp__blk949_dn7)) * locals.var_za__blk1401) + (assign51820_e66553 * locals.var_za__blk1401_dn7)))) / (assign51820_e66556 * assign51820_e66556)))), ((((locals.var_x_inf__blk1399_dn8 * locals.var_za__blk1401) + (locals.var_x_inf__blk1399 * locals.var_za__blk1401_dn8)) * assign51820_e66558) + (assign51820_e66537 * ((((((0.86 * locals.var_temp__blk949_dn8) * assign51820_e66546) + (assign51820_e66541 * (-((locals.var_temp__blk949_dn8 * locals.var_za__blk1401) + (locals.var_temp__blk949 * locals.var_za__blk1401_dn8))))) * assign51820_e66556) - (assign51820_e66547 * (((((4.0 * locals.var_temp__blk949_dn8) * locals.var_temp__blk949) + (assign51820_e66551 * locals.var_temp__blk949_dn8)) * locals.var_za__blk1401) + (assign51820_e66553 * locals.var_za__blk1401_dn8)))) / (assign51820_e66556 * assign51820_e66556)))), ((((locals.var_x_inf__blk1399_dn9 * locals.var_za__blk1401) + (locals.var_x_inf__blk1399 * locals.var_za__blk1401_dn9)) * assign51820_e66558) + (assign51820_e66537 * ((((((0.86 * locals.var_temp__blk949_dn9) * assign51820_e66546) + (assign51820_e66541 * (-((locals.var_temp__blk949_dn9 * locals.var_za__blk1401) + (locals.var_temp__blk949 * locals.var_za__blk1401_dn9))))) * assign51820_e66556) - (assign51820_e66547 * (((((4.0 * locals.var_temp__blk949_dn9) * locals.var_temp__blk949) + (assign51820_e66551 * locals.var_temp__blk949_dn9)) * locals.var_za__blk1401) + (assign51820_e66553 * locals.var_za__blk1401_dn9)))) / (assign51820_e66556 * assign51820_e66556)))), );
            locals.var_x_0__blk1402_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign51830_e66569: f64 = (0.99 * locals.var_x_0__blk1402);
            (locals.var_x_sat__blk1403, locals.var_x_sat__blk1403_dn4, locals.var_x_sat__blk1403_dn6, locals.var_x_sat__blk1403_dn7, locals.var_x_sat__blk1403_dn8, locals.var_x_sat__blk1403_dn9, ) = (assign51830_e66569, (0.99 * locals.var_x_0__blk1402_dn4), (0.99 * locals.var_x_0__blk1402_dn6), (0.99 * locals.var_x_0__blk1402_dn7), (0.99 * locals.var_x_0__blk1402_dn8), (0.99 * locals.var_x_0__blk1402_dn9), );
            locals.var_x_sat__blk1403_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign51840_e66581: f64 = (2.0 * locals.var_asat__blk1389);
            let assign51840_e66582: f64 = (locals.var_x_sat__blk1403 - assign51840_e66581);
            let assign51840_e66583: f64 = (locals.var_x_sat__blk1403 * assign51840_e66582);
            let assign51840_e66585: f64 = (assign51840_e66583 * locals.var_inv_gf2__blk1341);
            let assign51840_e66587: f64 = (assign51840_e66585 / locals.var_ds__blk1370);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign51840_e66587, (((((((locals.var_x_sat__blk1403_dn4 * assign51840_e66582) + (locals.var_x_sat__blk1403 * (locals.var_x_sat__blk1403_dn4 - (2.0 * locals.var_asat__blk1389_dn4)))) * locals.var_inv_gf2__blk1341) + (assign51840_e66583 * locals.var_inv_gf2__blk1341_dn4)) * locals.var_ds__blk1370) - (assign51840_e66585 * locals.var_ds__blk1370_dn4)) / (locals.var_ds__blk1370 * locals.var_ds__blk1370)), (((((((locals.var_x_sat__blk1403_dn6 * assign51840_e66582) + (locals.var_x_sat__blk1403 * (locals.var_x_sat__blk1403_dn6 - (2.0 * locals.var_asat__blk1389_dn6)))) * locals.var_inv_gf2__blk1341) + (assign51840_e66583 * locals.var_inv_gf2__blk1341_dn6)) * locals.var_ds__blk1370) - (assign51840_e66585 * locals.var_ds__blk1370_dn6)) / (locals.var_ds__blk1370 * locals.var_ds__blk1370)), (((((((locals.var_x_sat__blk1403_dn7 * assign51840_e66582) + (locals.var_x_sat__blk1403 * (locals.var_x_sat__blk1403_dn7 - (2.0 * locals.var_asat__blk1389_dn7)))) * locals.var_inv_gf2__blk1341) + (assign51840_e66583 * locals.var_inv_gf2__blk1341_dn7)) * locals.var_ds__blk1370) - (assign51840_e66585 * locals.var_ds__blk1370_dn7)) / (locals.var_ds__blk1370 * locals.var_ds__blk1370)), (((((((locals.var_x_sat__blk1403_dn8 * assign51840_e66582) + (locals.var_x_sat__blk1403 * (locals.var_x_sat__blk1403_dn8 - (2.0 * locals.var_asat__blk1389_dn8)))) * locals.var_inv_gf2__blk1341) + (assign51840_e66583 * locals.var_inv_gf2__blk1341_dn8)) * locals.var_ds__blk1370) - (assign51840_e66585 * locals.var_ds__blk1370_dn8)) / (locals.var_ds__blk1370 * locals.var_ds__blk1370)), (((((((locals.var_x_sat__blk1403_dn9 * assign51840_e66582) + (locals.var_x_sat__blk1403 * (locals.var_x_sat__blk1403_dn9 - (2.0 * locals.var_asat__blk1389_dn9)))) * locals.var_inv_gf2__blk1341) + (assign51840_e66583 * locals.var_inv_gf2__blk1341_dn9)) * locals.var_ds__blk1370) - (assign51840_e66585 * locals.var_ds__blk1370_dn9)) / (locals.var_ds__blk1370 * locals.var_ds__blk1370)), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign51850_e66600: f64 = (-0.99);
            let (assign51850_e66605, assign51850_e66605_d_n4, assign51850_e66605_d_n6, assign51850_e66605_d_n7, assign51850_e66605_d_n8, assign51850_e66605_d_n9,) = {
    if (locals.var_temp__blk949 > assign51850_e66600) {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    } else {
        let assign51850_e66604: f64 = (-0.99);
        (assign51850_e66604, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
            let assign51850_e66606: f64 = (1.0 + assign51850_e66605);
            let assign51850_e66607: f64 = (assign51850_e66606).ln();
            let assign51850_e66608: f64 = (locals.var_x_sat__blk1403 - assign51850_e66607);
            let assign51850_e66609: f64 = (locals.var_phit1__blk1339 * assign51850_e66608);
            (locals.var_v_dsat__blk1404, locals.var_v_dsat__blk1404_dn4, locals.var_v_dsat__blk1404_dn6, locals.var_v_dsat__blk1404_dn7, locals.var_v_dsat__blk1404_dn8, locals.var_v_dsat__blk1404_dn9, ) = (assign51850_e66609, ((locals.var_phit1__blk1339_dn4 * assign51850_e66608) + (locals.var_phit1__blk1339 * (locals.var_x_sat__blk1403_dn4 - (assign51850_e66605_d_n4 / assign51850_e66606)))), ((locals.var_phit1__blk1339_dn6 * assign51850_e66608) + (locals.var_phit1__blk1339 * (locals.var_x_sat__blk1403_dn6 - (assign51850_e66605_d_n6 / assign51850_e66606)))), ((locals.var_phit1__blk1339_dn7 * assign51850_e66608) + (locals.var_phit1__blk1339 * (locals.var_x_sat__blk1403_dn7 - (assign51850_e66605_d_n7 / assign51850_e66606)))), ((locals.var_phit1__blk1339_dn8 * assign51850_e66608) + (locals.var_phit1__blk1339 * (locals.var_x_sat__blk1403_dn8 - (assign51850_e66605_d_n8 / assign51850_e66606)))), ((locals.var_phit1__blk1339_dn9 * assign51850_e66608) + (locals.var_phit1__blk1339 * (locals.var_x_sat__blk1403_dn9 - (assign51850_e66605_d_n9 / assign51850_e66606)))), );
            locals.var_v_dsat__blk1404_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1502 == 0.0)) {
            (locals.var_v_dsat__blk1404, locals.var_v_dsat__blk1404_dn4, locals.var_v_dsat__blk1404_dn6, locals.var_v_dsat__blk1404_dn7, locals.var_v_dsat__blk1404_dn8, locals.var_v_dsat__blk1404_dn9, ) = (locals.var_vdsat_lim__blk1387, locals.var_vdsat_lim__blk1387_dn4, locals.var_vdsat_lim__blk1387_dn6, locals.var_vdsat_lim__blk1387_dn7, locals.var_vdsat_lim__blk1387_dn8, locals.var_vdsat_lim__blk1387_dn9, );
            locals.var_v_dsat__blk1404_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign51870_e66626: f64 = (1.0 + locals.var_arloc__blk1320);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign51870_e66626, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign51880_e66633: f64 = (locals.var_temp__blk949).sqrt();
            let assign51880_e66635: f64 = (assign51880_e66633 * locals.var_v_ds);
            let assign51880_e66637: f64 = (assign51880_e66635 / locals.var_v_dsat__blk1404);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign51880_e66637, (((((locals.var_temp__blk949_dn4 / (2.0 * assign51880_e66633)) * locals.var_v_ds) * locals.var_v_dsat__blk1404) - (assign51880_e66635 * locals.var_v_dsat__blk1404_dn4)) / (locals.var_v_dsat__blk1404 * locals.var_v_dsat__blk1404)), (((((locals.var_temp__blk949_dn6 / (2.0 * assign51880_e66633)) * locals.var_v_ds) * locals.var_v_dsat__blk1404) - (assign51880_e66635 * locals.var_v_dsat__blk1404_dn6)) / (locals.var_v_dsat__blk1404 * locals.var_v_dsat__blk1404)), ((((((locals.var_temp__blk949_dn7 / (2.0 * assign51880_e66633)) * locals.var_v_ds) + (assign51880_e66633 * locals.var_v_ds_dn7)) * locals.var_v_dsat__blk1404) - (assign51880_e66635 * locals.var_v_dsat__blk1404_dn7)) / (locals.var_v_dsat__blk1404 * locals.var_v_dsat__blk1404)), ((((((locals.var_temp__blk949_dn8 / (2.0 * assign51880_e66633)) * locals.var_v_ds) + (assign51880_e66633 * locals.var_v_ds_dn8)) * locals.var_v_dsat__blk1404) - (assign51880_e66635 * locals.var_v_dsat__blk1404_dn8)) / (locals.var_v_dsat__blk1404 * locals.var_v_dsat__blk1404)), (((((locals.var_temp__blk949_dn9 / (2.0 * assign51880_e66633)) * locals.var_v_ds) * locals.var_v_dsat__blk1404) - (assign51880_e66635 * locals.var_v_dsat__blk1404_dn9)) / (locals.var_v_dsat__blk1404 * locals.var_v_dsat__blk1404)), );
            locals.var_temp1_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign51890_e66645: f64 = (locals.var_temp1 * locals.var_temp1);
            let assign51890_e66647: f64 = (assign51890_e66645 + locals.var_temp__blk949);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign51890_e66647, (((locals.var_temp1_dn4 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn4)) + locals.var_temp__blk949_dn4), (((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)) + locals.var_temp__blk949_dn6), (((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)) + locals.var_temp__blk949_dn7), (((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)) + locals.var_temp__blk949_dn8), (((locals.var_temp1_dn9 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn9)) + locals.var_temp__blk949_dn9), );
            locals.var_temp2_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign51900_e66655: f64 = (2.0 * locals.var_temp1);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign51900_e66655, (2.0 * locals.var_temp1_dn4), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), (2.0 * locals.var_temp1_dn9), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign51910_e66663: f64 = (locals.var_v_dsat__blk1404 * locals.var_temp__blk949);
            let assign51910_e66666: f64 = (locals.var_temp2 - locals.var_temp__blk949);
            let assign51910_e66667: f64 = (assign51910_e66666).sqrt();
            let assign51910_e66670: f64 = (locals.var_temp2 + locals.var_temp__blk949);
            let assign51910_e66671: f64 = (assign51910_e66670).sqrt();
            let assign51910_e66672: f64 = (assign51910_e66667 + assign51910_e66671);
            let assign51910_e66673: f64 = (assign51910_e66663 / assign51910_e66672);
            (locals.var_vdse__blk1405, locals.var_vdse__blk1405_dn4, locals.var_vdse__blk1405_dn6, locals.var_vdse__blk1405_dn7, locals.var_vdse__blk1405_dn8, locals.var_vdse__blk1405_dn9, ) = (assign51910_e66673, (((((locals.var_v_dsat__blk1404_dn4 * locals.var_temp__blk949) + (locals.var_v_dsat__blk1404 * locals.var_temp__blk949_dn4)) * assign51910_e66672) - (assign51910_e66663 * (((locals.var_temp2_dn4 - locals.var_temp__blk949_dn4) / (2.0 * assign51910_e66667)) + ((locals.var_temp2_dn4 + locals.var_temp__blk949_dn4) / (2.0 * assign51910_e66671))))) / (assign51910_e66672 * assign51910_e66672)), (((((locals.var_v_dsat__blk1404_dn6 * locals.var_temp__blk949) + (locals.var_v_dsat__blk1404 * locals.var_temp__blk949_dn6)) * assign51910_e66672) - (assign51910_e66663 * (((locals.var_temp2_dn6 - locals.var_temp__blk949_dn6) / (2.0 * assign51910_e66667)) + ((locals.var_temp2_dn6 + locals.var_temp__blk949_dn6) / (2.0 * assign51910_e66671))))) / (assign51910_e66672 * assign51910_e66672)), (((((locals.var_v_dsat__blk1404_dn7 * locals.var_temp__blk949) + (locals.var_v_dsat__blk1404 * locals.var_temp__blk949_dn7)) * assign51910_e66672) - (assign51910_e66663 * (((locals.var_temp2_dn7 - locals.var_temp__blk949_dn7) / (2.0 * assign51910_e66667)) + ((locals.var_temp2_dn7 + locals.var_temp__blk949_dn7) / (2.0 * assign51910_e66671))))) / (assign51910_e66672 * assign51910_e66672)), (((((locals.var_v_dsat__blk1404_dn8 * locals.var_temp__blk949) + (locals.var_v_dsat__blk1404 * locals.var_temp__blk949_dn8)) * assign51910_e66672) - (assign51910_e66663 * (((locals.var_temp2_dn8 - locals.var_temp__blk949_dn8) / (2.0 * assign51910_e66667)) + ((locals.var_temp2_dn8 + locals.var_temp__blk949_dn8) / (2.0 * assign51910_e66671))))) / (assign51910_e66672 * assign51910_e66672)), (((((locals.var_v_dsat__blk1404_dn9 * locals.var_temp__blk949) + (locals.var_v_dsat__blk1404 * locals.var_temp__blk949_dn9)) * assign51910_e66672) - (assign51910_e66663 * (((locals.var_temp2_dn9 - locals.var_temp__blk949_dn9) / (2.0 * assign51910_e66667)) + ((locals.var_temp2_dn9 + locals.var_temp__blk949_dn9) / (2.0 * assign51910_e66671))))) / (assign51910_e66672 * assign51910_e66672)), );
            locals.var_vdse__blk1405_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign51920_e66681: f64 = (locals.var_vdse__blk1405 * locals.var_inv_phit1__blk1340);
            (locals.var_udse__blk1406, locals.var_udse__blk1406_dn4, locals.var_udse__blk1406_dn6, locals.var_udse__blk1406_dn7, locals.var_udse__blk1406_dn8, locals.var_udse__blk1406_dn9, ) = (assign51920_e66681, ((locals.var_vdse__blk1405_dn4 * locals.var_inv_phit1__blk1340) + (locals.var_vdse__blk1405 * locals.var_inv_phit1__blk1340_dn4)), ((locals.var_vdse__blk1405_dn6 * locals.var_inv_phit1__blk1340) + (locals.var_vdse__blk1405 * locals.var_inv_phit1__blk1340_dn6)), ((locals.var_vdse__blk1405_dn7 * locals.var_inv_phit1__blk1340) + (locals.var_vdse__blk1405 * locals.var_inv_phit1__blk1340_dn7)), ((locals.var_vdse__blk1405_dn8 * locals.var_inv_phit1__blk1340) + (locals.var_vdse__blk1405 * locals.var_inv_phit1__blk1340_dn8)), ((locals.var_vdse__blk1405_dn9 * locals.var_inv_phit1__blk1340) + (locals.var_vdse__blk1405 * locals.var_inv_phit1__blk1340_dn9)), );
            locals.var_udse__blk1406_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign51930_e66689: f64 = (locals.var_xn_s__blk1349 + locals.var_udse__blk1406);
            (locals.var_xn_d__blk1407, locals.var_xn_d__blk1407_dn4, locals.var_xn_d__blk1407_dn6, locals.var_xn_d__blk1407_dn7, locals.var_xn_d__blk1407_dn8, locals.var_xn_d__blk1407_dn9, ) = (assign51930_e66689, (locals.var_xn_s__blk1349_dn4 + locals.var_udse__blk1406_dn4), (locals.var_xn_s__blk1349_dn6 + locals.var_udse__blk1406_dn6), (locals.var_xn_s__blk1349_dn7 + locals.var_udse__blk1406_dn7), (locals.var_xn_s__blk1349_dn8 + locals.var_udse__blk1406_dn8), (locals.var_xn_s__blk1349_dn9 + locals.var_udse__blk1406_dn9), );
            locals.var_xn_d__blk1407_rv = 0.0;
        }

        let assign51940_e66694: f64 = if locals.var_udse__blk1406 < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1508 = assign51940_e66694;
        locals.var_guard1508_rv = 0.0;

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1508 != 0.0)) {
            let assign51950_e66701: f64 = (-locals.var_udse__blk1406);
            let assign51950_e66702: f64 = (assign51950_e66701).exp();
            (locals.var_k_ds__blk1408, locals.var_k_ds__blk1408_dn4, locals.var_k_ds__blk1408_dn6, locals.var_k_ds__blk1408_dn7, locals.var_k_ds__blk1408_dn8, locals.var_k_ds__blk1408_dn9, ) = (assign51950_e66702, (assign51950_e66702 * (-locals.var_udse__blk1406_dn4)), (assign51950_e66702 * (-locals.var_udse__blk1406_dn6)), (assign51950_e66702 * (-locals.var_udse__blk1406_dn7)), (assign51950_e66702 * (-locals.var_udse__blk1406_dn8)), (assign51950_e66702 * (-locals.var_udse__blk1406_dn9)), );
            locals.var_k_ds__blk1408_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1508 == 0.0)) {
            let assign51960_e66715: f64 = (locals.var_udse__blk1406 - 460.51701859880916);
            let assign51960_e66720: f64 = (locals.var_udse__blk1406 - 460.51701859880916);
            let assign51960_e66724: f64 = (locals.var_udse__blk1406 - 460.51701859880916);
            let assign51960_e66726: f64 = (assign51960_e66724 * 0.3333333333333333);
            let assign51960_e66727: f64 = (1.0 + assign51960_e66726);
            let assign51960_e66728: f64 = (assign51960_e66720 * assign51960_e66727);
            let assign51960_e66729: f64 = (0.5 * assign51960_e66728);
            let assign51960_e66730: f64 = (1.0 + assign51960_e66729);
            let assign51960_e66731: f64 = (assign51960_e66715 * assign51960_e66730);
            let assign51960_e66732: f64 = (1.0 + assign51960_e66731);
            let assign51960_e66733: f64 = (1e-200 / assign51960_e66732);
            (locals.var_k_ds__blk1408, locals.var_k_ds__blk1408_dn4, locals.var_k_ds__blk1408_dn6, locals.var_k_ds__blk1408_dn7, locals.var_k_ds__blk1408_dn8, locals.var_k_ds__blk1408_dn9, ) = (assign51960_e66733, (-((1e-200 * ((locals.var_udse__blk1406_dn4 * assign51960_e66730) + (assign51960_e66715 * (0.5 * ((locals.var_udse__blk1406_dn4 * assign51960_e66727) + (assign51960_e66720 * (locals.var_udse__blk1406_dn4 * 0.3333333333333333))))))) / (assign51960_e66732 * assign51960_e66732))), (-((1e-200 * ((locals.var_udse__blk1406_dn6 * assign51960_e66730) + (assign51960_e66715 * (0.5 * ((locals.var_udse__blk1406_dn6 * assign51960_e66727) + (assign51960_e66720 * (locals.var_udse__blk1406_dn6 * 0.3333333333333333))))))) / (assign51960_e66732 * assign51960_e66732))), (-((1e-200 * ((locals.var_udse__blk1406_dn7 * assign51960_e66730) + (assign51960_e66715 * (0.5 * ((locals.var_udse__blk1406_dn7 * assign51960_e66727) + (assign51960_e66720 * (locals.var_udse__blk1406_dn7 * 0.3333333333333333))))))) / (assign51960_e66732 * assign51960_e66732))), (-((1e-200 * ((locals.var_udse__blk1406_dn8 * assign51960_e66730) + (assign51960_e66715 * (0.5 * ((locals.var_udse__blk1406_dn8 * assign51960_e66727) + (assign51960_e66720 * (locals.var_udse__blk1406_dn8 * 0.3333333333333333))))))) / (assign51960_e66732 * assign51960_e66732))), (-((1e-200 * ((locals.var_udse__blk1406_dn9 * assign51960_e66730) + (assign51960_e66715 * (0.5 * ((locals.var_udse__blk1406_dn9 * assign51960_e66727) + (assign51960_e66720 * (locals.var_udse__blk1406_dn9 * 0.3333333333333333))))))) / (assign51960_e66732 * assign51960_e66732))), );
            locals.var_k_ds__blk1408_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign51970_e66741: f64 = (locals.var_delta_ns__blk1364 * locals.var_k_ds__blk1408);
            (locals.var_delta_nd__blk1409, locals.var_delta_nd__blk1409_dn4, locals.var_delta_nd__blk1409_dn6, locals.var_delta_nd__blk1409_dn7, locals.var_delta_nd__blk1409_dn8, locals.var_delta_nd__blk1409_dn9, ) = (assign51970_e66741, ((locals.var_delta_ns__blk1364_dn4 * locals.var_k_ds__blk1408) + (locals.var_delta_ns__blk1364 * locals.var_k_ds__blk1408_dn4)), ((locals.var_delta_ns__blk1364_dn6 * locals.var_k_ds__blk1408) + (locals.var_delta_ns__blk1364 * locals.var_k_ds__blk1408_dn6)), ((locals.var_delta_ns__blk1364_dn7 * locals.var_k_ds__blk1408) + (locals.var_delta_ns__blk1364 * locals.var_k_ds__blk1408_dn7)), ((locals.var_delta_ns__blk1364_dn8 * locals.var_k_ds__blk1408) + (locals.var_delta_ns__blk1364 * locals.var_k_ds__blk1408_dn8)), ((locals.var_delta_ns__blk1364_dn9 * locals.var_k_ds__blk1408) + (locals.var_delta_ns__blk1364 * locals.var_k_ds__blk1408_dn9)), );
            locals.var_delta_nd__blk1409_rv = 0.0;
        }

        let assign51980_e66745: f64 = (locals.var_xg__blk1343).abs();
        let assign51980_e66747: f64 = if assign51980_e66745 <= locals.var_margin__blk1361 { 1.0 } else { 0.0 };
        locals.var_guard1509 = assign51980_e66747;
        locals.var_guard1509_rv = 0.0;

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 != 0.0)) {
            let assign51990_e66755: f64 = (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362);
            let assign51990_e66757: f64 = (assign51990_e66755 * 0.16666666666666666);
            let assign51990_e66759: f64 = (assign51990_e66757 * 0.7071067811865475);
            (locals.var_sp_s_temp1__blk1449, locals.var_sp_s_temp1__blk1449_dn4, locals.var_sp_s_temp1__blk1449_dn6, locals.var_sp_s_temp1__blk1449_dn7, locals.var_sp_s_temp1__blk1449_dn8, locals.var_sp_s_temp1__blk1449_dn9, ) = (assign51990_e66759, ((((locals.var_inv_xi__blk1362_dn4 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn4)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn6 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn7 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn8 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn8)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1362_dn9 * locals.var_inv_xi__blk1362) + (locals.var_inv_xi__blk1362 * locals.var_inv_xi__blk1362_dn9)) * 0.16666666666666666) * 0.7071067811865475), );
            locals.var_sp_s_temp1__blk1449_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 != 0.0)) {
            let assign52000_e66769: f64 = (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362);
            let assign52000_e66774: f64 = (1.0 - locals.var_delta_nd__blk1409);
            let assign52000_e66775: f64 = (locals.var_xg__blk1343 * assign52000_e66774);
            let assign52000_e66777: f64 = (assign52000_e66775 * locals.var_gf__blk1324);
            let assign52000_e66779: f64 = (assign52000_e66777 * locals.var_sp_s_temp1__blk1449);
            let assign52000_e66780: f64 = (1.0 + assign52000_e66779);
            let assign52000_e66781: f64 = (assign52000_e66769 * assign52000_e66780);
            (locals.var_x_d__blk1410, locals.var_x_d__blk1410_dn4, locals.var_x_d__blk1410_dn6, locals.var_x_d__blk1410_dn7, locals.var_x_d__blk1410_dn8, locals.var_x_d__blk1410_dn9, ) = (assign52000_e66781, ((((locals.var_xg__blk1343_dn4 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn4)) * assign52000_e66780) + (assign52000_e66769 * ((((((locals.var_xg__blk1343_dn4 * assign52000_e66774) + (locals.var_xg__blk1343 * (-locals.var_delta_nd__blk1409_dn4))) * locals.var_gf__blk1324) + (assign52000_e66775 * locals.var_gf__blk1324_dn4)) * locals.var_sp_s_temp1__blk1449) + (assign52000_e66777 * locals.var_sp_s_temp1__blk1449_dn4)))), ((((locals.var_xg__blk1343_dn6 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn6)) * assign52000_e66780) + (assign52000_e66769 * ((((((locals.var_xg__blk1343_dn6 * assign52000_e66774) + (locals.var_xg__blk1343 * (-locals.var_delta_nd__blk1409_dn6))) * locals.var_gf__blk1324) + (assign52000_e66775 * locals.var_gf__blk1324_dn6)) * locals.var_sp_s_temp1__blk1449) + (assign52000_e66777 * locals.var_sp_s_temp1__blk1449_dn6)))), ((((locals.var_xg__blk1343_dn7 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn7)) * assign52000_e66780) + (assign52000_e66769 * ((((((locals.var_xg__blk1343_dn7 * assign52000_e66774) + (locals.var_xg__blk1343 * (-locals.var_delta_nd__blk1409_dn7))) * locals.var_gf__blk1324) + (assign52000_e66775 * locals.var_gf__blk1324_dn7)) * locals.var_sp_s_temp1__blk1449) + (assign52000_e66777 * locals.var_sp_s_temp1__blk1449_dn7)))), ((((locals.var_xg__blk1343_dn8 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn8)) * assign52000_e66780) + (assign52000_e66769 * ((((((locals.var_xg__blk1343_dn8 * assign52000_e66774) + (locals.var_xg__blk1343 * (-locals.var_delta_nd__blk1409_dn8))) * locals.var_gf__blk1324) + (assign52000_e66775 * locals.var_gf__blk1324_dn8)) * locals.var_sp_s_temp1__blk1449) + (assign52000_e66777 * locals.var_sp_s_temp1__blk1449_dn8)))), ((((locals.var_xg__blk1343_dn9 * locals.var_inv_xi__blk1362) + (locals.var_xg__blk1343 * locals.var_inv_xi__blk1362_dn9)) * assign52000_e66780) + (assign52000_e66769 * ((((((locals.var_xg__blk1343_dn9 * assign52000_e66774) + (locals.var_xg__blk1343 * (-locals.var_delta_nd__blk1409_dn9))) * locals.var_gf__blk1324) + (assign52000_e66775 * locals.var_gf__blk1324_dn9)) * locals.var_sp_s_temp1__blk1449) + (assign52000_e66777 * locals.var_sp_s_temp1__blk1449_dn9)))), );
            locals.var_x_d__blk1410_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52010_e66792: f64 = (locals.var_xn_d__blk1407 + 3.0);
            (locals.var_sp_s_bx__blk1470, locals.var_sp_s_bx__blk1470_dn4, locals.var_sp_s_bx__blk1470_dn6, locals.var_sp_s_bx__blk1470_dn7, locals.var_sp_s_bx__blk1470_dn8, locals.var_sp_s_bx__blk1470_dn9, ) = (assign52010_e66792, locals.var_xn_d__blk1407_dn4, locals.var_xn_d__blk1407_dn6, locals.var_xn_d__blk1407_dn7, locals.var_xn_d__blk1407_dn8, locals.var_xn_d__blk1407_dn9, );
            locals.var_sp_s_bx__blk1470_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52020_e66804: f64 = (locals.var_sp_s_x1__blk1469 + locals.var_sp_s_bx__blk1470);
            let assign52020_e66807: f64 = (locals.var_sp_s_x1__blk1469 - locals.var_sp_s_bx__blk1470);
            let assign52020_e66810: f64 = (locals.var_sp_s_x1__blk1469 - locals.var_sp_s_bx__blk1470);
            let assign52020_e66811: f64 = (assign52020_e66807 * assign52020_e66810);
            let assign52020_e66813: f64 = (assign52020_e66811 + 5.0);
            let assign52020_e66814: f64 = (assign52020_e66813).sqrt();
            let assign52020_e66815: f64 = (assign52020_e66804 - assign52020_e66814);
            let assign52020_e66816: f64 = (0.5 * assign52020_e66815);
            let assign52020_e66821: f64 = (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470);
            let assign52020_e66823: f64 = (assign52020_e66821 + 5.0);
            let assign52020_e66824: f64 = (assign52020_e66823).sqrt();
            let assign52020_e66825: f64 = (locals.var_sp_s_bx__blk1470 - assign52020_e66824);
            let assign52020_e66826: f64 = (0.5 * assign52020_e66825);
            let assign52020_e66827: f64 = (assign52020_e66816 - assign52020_e66826);
            (locals.var_sp_s_eta__blk1453, locals.var_sp_s_eta__blk1453_dn4, locals.var_sp_s_eta__blk1453_dn6, locals.var_sp_s_eta__blk1453_dn7, locals.var_sp_s_eta__blk1453_dn8, locals.var_sp_s_eta__blk1453_dn9, ) = (assign52020_e66827, ((0.5 * ((locals.var_sp_s_x1__blk1469_dn4 + locals.var_sp_s_bx__blk1470_dn4) - ((((locals.var_sp_s_x1__blk1469_dn4 - locals.var_sp_s_bx__blk1470_dn4) * assign52020_e66810) + (assign52020_e66807 * (locals.var_sp_s_x1__blk1469_dn4 - locals.var_sp_s_bx__blk1470_dn4))) / (2.0 * assign52020_e66814)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn4 - (((locals.var_sp_s_bx__blk1470_dn4 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn4)) / (2.0 * assign52020_e66824))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn6 + locals.var_sp_s_bx__blk1470_dn6) - ((((locals.var_sp_s_x1__blk1469_dn6 - locals.var_sp_s_bx__blk1470_dn6) * assign52020_e66810) + (assign52020_e66807 * (locals.var_sp_s_x1__blk1469_dn6 - locals.var_sp_s_bx__blk1470_dn6))) / (2.0 * assign52020_e66814)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn6 - (((locals.var_sp_s_bx__blk1470_dn6 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn6)) / (2.0 * assign52020_e66824))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn7 + locals.var_sp_s_bx__blk1470_dn7) - ((((locals.var_sp_s_x1__blk1469_dn7 - locals.var_sp_s_bx__blk1470_dn7) * assign52020_e66810) + (assign52020_e66807 * (locals.var_sp_s_x1__blk1469_dn7 - locals.var_sp_s_bx__blk1470_dn7))) / (2.0 * assign52020_e66814)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn7 - (((locals.var_sp_s_bx__blk1470_dn7 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn7)) / (2.0 * assign52020_e66824))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn8 + locals.var_sp_s_bx__blk1470_dn8) - ((((locals.var_sp_s_x1__blk1469_dn8 - locals.var_sp_s_bx__blk1470_dn8) * assign52020_e66810) + (assign52020_e66807 * (locals.var_sp_s_x1__blk1469_dn8 - locals.var_sp_s_bx__blk1470_dn8))) / (2.0 * assign52020_e66814)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn8 - (((locals.var_sp_s_bx__blk1470_dn8 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn8)) / (2.0 * assign52020_e66824))))), ((0.5 * ((locals.var_sp_s_x1__blk1469_dn9 + locals.var_sp_s_bx__blk1470_dn9) - ((((locals.var_sp_s_x1__blk1469_dn9 - locals.var_sp_s_bx__blk1470_dn9) * assign52020_e66810) + (assign52020_e66807 * (locals.var_sp_s_x1__blk1469_dn9 - locals.var_sp_s_bx__blk1470_dn9))) / (2.0 * assign52020_e66814)))) - (0.5 * (locals.var_sp_s_bx__blk1470_dn9 - (((locals.var_sp_s_bx__blk1470_dn9 * locals.var_sp_s_bx__blk1470) + (locals.var_sp_s_bx__blk1470 * locals.var_sp_s_bx__blk1470_dn9)) / (2.0 * assign52020_e66824))))), );
            locals.var_sp_s_eta__blk1453_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52030_e66838: f64 = (locals.var_xg__blk1343 - locals.var_sp_s_eta__blk1453);
            (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9, ) = (assign52030_e66838, (locals.var_xg__blk1343_dn4 - locals.var_sp_s_eta__blk1453_dn4), (locals.var_xg__blk1343_dn6 - locals.var_sp_s_eta__blk1453_dn6), (locals.var_xg__blk1343_dn7 - locals.var_sp_s_eta__blk1453_dn7), (locals.var_xg__blk1343_dn8 - locals.var_sp_s_eta__blk1453_dn8), (locals.var_xg__blk1343_dn9 - locals.var_sp_s_eta__blk1453_dn9), );
            locals.var_sp_s_temp__blk1448_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52040_e66848: f64 = (-locals.var_sp_s_eta__blk1453);
            let assign52040_e66849: f64 = (assign52040_e66848).exp();
            (locals.var_sp_s_temp1__blk1449, locals.var_sp_s_temp1__blk1449_dn4, locals.var_sp_s_temp1__blk1449_dn6, locals.var_sp_s_temp1__blk1449_dn7, locals.var_sp_s_temp1__blk1449_dn8, locals.var_sp_s_temp1__blk1449_dn9, ) = (assign52040_e66849, (assign52040_e66849 * (-locals.var_sp_s_eta__blk1453_dn4)), (assign52040_e66849 * (-locals.var_sp_s_eta__blk1453_dn6)), (assign52040_e66849 * (-locals.var_sp_s_eta__blk1453_dn7)), (assign52040_e66849 * (-locals.var_sp_s_eta__blk1453_dn8)), (assign52040_e66849 * (-locals.var_sp_s_eta__blk1453_dn9)), );
            locals.var_sp_s_temp1__blk1449_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52050_e66862: f64 = (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453);
            let assign52050_e66863: f64 = (2.0 + assign52050_e66862);
            let assign52050_e66864: f64 = (1.0 / assign52050_e66863);
            (locals.var_sp_s_temp2__blk1450, locals.var_sp_s_temp2__blk1450_dn4, locals.var_sp_s_temp2__blk1450_dn6, locals.var_sp_s_temp2__blk1450_dn7, locals.var_sp_s_temp2__blk1450_dn8, locals.var_sp_s_temp2__blk1450_dn9, ) = (assign52050_e66864, (-(((locals.var_sp_s_eta__blk1453_dn4 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn4)) / (assign52050_e66863 * assign52050_e66863))), (-(((locals.var_sp_s_eta__blk1453_dn6 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn6)) / (assign52050_e66863 * assign52050_e66863))), (-(((locals.var_sp_s_eta__blk1453_dn7 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn7)) / (assign52050_e66863 * assign52050_e66863))), (-(((locals.var_sp_s_eta__blk1453_dn8 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn8)) / (assign52050_e66863 * assign52050_e66863))), (-(((locals.var_sp_s_eta__blk1453_dn9 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn9)) / (assign52050_e66863 * assign52050_e66863))), );
            locals.var_sp_s_temp2__blk1450_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52060_e66875: f64 = (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453);
            let assign52060_e66877: f64 = (assign52060_e66875 * locals.var_sp_s_temp2__blk1450);
            (locals.var_sp_s_xi0__blk1460, locals.var_sp_s_xi0__blk1460_dn4, locals.var_sp_s_xi0__blk1460_dn6, locals.var_sp_s_xi0__blk1460_dn7, locals.var_sp_s_xi0__blk1460_dn8, locals.var_sp_s_xi0__blk1460_dn9, ) = (assign52060_e66877, ((((locals.var_sp_s_eta__blk1453_dn4 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign52060_e66875 * locals.var_sp_s_temp2__blk1450_dn4)), ((((locals.var_sp_s_eta__blk1453_dn6 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign52060_e66875 * locals.var_sp_s_temp2__blk1450_dn6)), ((((locals.var_sp_s_eta__blk1453_dn7 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign52060_e66875 * locals.var_sp_s_temp2__blk1450_dn7)), ((((locals.var_sp_s_eta__blk1453_dn8 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign52060_e66875 * locals.var_sp_s_temp2__blk1450_dn8)), ((((locals.var_sp_s_eta__blk1453_dn9 * locals.var_sp_s_eta__blk1453) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_eta__blk1453_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign52060_e66875 * locals.var_sp_s_temp2__blk1450_dn9)), );
            locals.var_sp_s_xi0__blk1460_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_27(
        locals: &mut StampLocals,
    ) {
        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52070_e66889: f64 = (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450);
            let assign52070_e66891: f64 = (assign52070_e66889 * locals.var_sp_s_temp2__blk1450);
            let assign52070_e66892: f64 = (4.0 * assign52070_e66891);
            (locals.var_sp_s_xi1__blk1461, locals.var_sp_s_xi1__blk1461_dn4, locals.var_sp_s_xi1__blk1461_dn6, locals.var_sp_s_xi1__blk1461_dn7, locals.var_sp_s_xi1__blk1461_dn8, locals.var_sp_s_xi1__blk1461_dn9, ) = (assign52070_e66892, (4.0 * ((((locals.var_sp_s_eta__blk1453_dn4 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign52070_e66889 * locals.var_sp_s_temp2__blk1450_dn4))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn6 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign52070_e66889 * locals.var_sp_s_temp2__blk1450_dn6))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn7 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign52070_e66889 * locals.var_sp_s_temp2__blk1450_dn7))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn8 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign52070_e66889 * locals.var_sp_s_temp2__blk1450_dn8))), (4.0 * ((((locals.var_sp_s_eta__blk1453_dn9 * locals.var_sp_s_temp2__blk1450) + (locals.var_sp_s_eta__blk1453 * locals.var_sp_s_temp2__blk1450_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign52070_e66889 * locals.var_sp_s_temp2__blk1450_dn9))), );
            locals.var_sp_s_xi1__blk1461_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52080_e66903: f64 = (8.0 * locals.var_sp_s_temp2__blk1450);
            let assign52080_e66906: f64 = (12.0 * locals.var_sp_s_xi0__blk1460);
            let assign52080_e66907: f64 = (assign52080_e66903 - assign52080_e66906);
            let assign52080_e66909: f64 = (assign52080_e66907 * locals.var_sp_s_temp2__blk1450);
            let assign52080_e66911: f64 = (assign52080_e66909 * locals.var_sp_s_temp2__blk1450);
            (locals.var_sp_s_xi2__blk1462, locals.var_sp_s_xi2__blk1462_dn4, locals.var_sp_s_xi2__blk1462_dn6, locals.var_sp_s_xi2__blk1462_dn7, locals.var_sp_s_xi2__blk1462_dn8, locals.var_sp_s_xi2__blk1462_dn9, ) = (assign52080_e66911, ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn4) - (12.0 * locals.var_sp_s_xi0__blk1460_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66907 * locals.var_sp_s_temp2__blk1450_dn4)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66909 * locals.var_sp_s_temp2__blk1450_dn4)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn6) - (12.0 * locals.var_sp_s_xi0__blk1460_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66907 * locals.var_sp_s_temp2__blk1450_dn6)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66909 * locals.var_sp_s_temp2__blk1450_dn6)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn7) - (12.0 * locals.var_sp_s_xi0__blk1460_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66907 * locals.var_sp_s_temp2__blk1450_dn7)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66909 * locals.var_sp_s_temp2__blk1450_dn7)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn8) - (12.0 * locals.var_sp_s_xi0__blk1460_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66907 * locals.var_sp_s_temp2__blk1450_dn8)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66909 * locals.var_sp_s_temp2__blk1450_dn8)), ((((((8.0 * locals.var_sp_s_temp2__blk1450_dn9) - (12.0 * locals.var_sp_s_xi0__blk1460_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66907 * locals.var_sp_s_temp2__blk1450_dn9)) * locals.var_sp_s_temp2__blk1450) + (assign52080_e66909 * locals.var_sp_s_temp2__blk1450_dn9)), );
            locals.var_sp_s_xi2__blk1462_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52090_e66923: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
            let assign52090_e66927: f64 = (locals.var_sp_s_temp1__blk1449 + locals.var_sp_s_eta__blk1453);
            let assign52090_e66929: f64 = (assign52090_e66927 - 1.0);
            let assign52090_e66933: f64 = (locals.var_sp_s_eta__blk1453 + 1.0);
            let assign52090_e66935: f64 = (assign52090_e66933 + locals.var_sp_s_xi0__blk1460);
            let assign52090_e66936: f64 = (locals.var_delta_nd__blk1409 * assign52090_e66935);
            let assign52090_e66937: f64 = (assign52090_e66929 - assign52090_e66936);
            let assign52090_e66938: f64 = (locals.var_gf2__blk1325 * assign52090_e66937);
            let assign52090_e66939: f64 = (assign52090_e66923 - assign52090_e66938);
            let (assign52090_e66961, assign52090_e66961_d_n4, assign52090_e66961_d_n6, assign52090_e66961_d_n7, assign52090_e66961_d_n8, assign52090_e66961_d_n9,) = {
    if (1e-40 > assign52090_e66939) {
        (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        let assign52090_e66944: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
        let assign52090_e66948: f64 = (locals.var_sp_s_temp1__blk1449 + locals.var_sp_s_eta__blk1453);
        let assign52090_e66950: f64 = (assign52090_e66948 - 1.0);
        let assign52090_e66954: f64 = (locals.var_sp_s_eta__blk1453 + 1.0);
        let assign52090_e66956: f64 = (assign52090_e66954 + locals.var_sp_s_xi0__blk1460);
        let assign52090_e66957: f64 = (locals.var_delta_nd__blk1409 * assign52090_e66956);
        let assign52090_e66958: f64 = (assign52090_e66950 - assign52090_e66957);
        let assign52090_e66959: f64 = (locals.var_gf2__blk1325 * assign52090_e66958);
        let assign52090_e66960: f64 = (assign52090_e66944 - assign52090_e66959);
        (assign52090_e66960, (((locals.var_sp_s_temp__blk1448_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn4)) - ((locals.var_gf2__blk1325_dn4 * assign52090_e66958) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn4 + locals.var_sp_s_eta__blk1453_dn4) - ((locals.var_delta_nd__blk1409_dn4 * assign52090_e66956) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_eta__blk1453_dn4 + locals.var_sp_s_xi0__blk1460_dn4))))))), (((locals.var_sp_s_temp__blk1448_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn6)) - ((locals.var_gf2__blk1325_dn6 * assign52090_e66958) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn6 + locals.var_sp_s_eta__blk1453_dn6) - ((locals.var_delta_nd__blk1409_dn6 * assign52090_e66956) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_eta__blk1453_dn6 + locals.var_sp_s_xi0__blk1460_dn6))))))), (((locals.var_sp_s_temp__blk1448_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn7)) - ((locals.var_gf2__blk1325_dn7 * assign52090_e66958) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn7 + locals.var_sp_s_eta__blk1453_dn7) - ((locals.var_delta_nd__blk1409_dn7 * assign52090_e66956) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_eta__blk1453_dn7 + locals.var_sp_s_xi0__blk1460_dn7))))))), (((locals.var_sp_s_temp__blk1448_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn8)) - ((locals.var_gf2__blk1325_dn8 * assign52090_e66958) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn8 + locals.var_sp_s_eta__blk1453_dn8) - ((locals.var_delta_nd__blk1409_dn8 * assign52090_e66956) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_eta__blk1453_dn8 + locals.var_sp_s_xi0__blk1460_dn8))))))), (((locals.var_sp_s_temp__blk1448_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn9)) - ((locals.var_gf2__blk1325_dn9 * assign52090_e66958) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_temp1__blk1449_dn9 + locals.var_sp_s_eta__blk1453_dn9) - ((locals.var_delta_nd__blk1409_dn9 * assign52090_e66956) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_eta__blk1453_dn9 + locals.var_sp_s_xi0__blk1460_dn9))))))),)
    }
};
            (locals.var_sp_s_a__blk1454, locals.var_sp_s_a__blk1454_dn4, locals.var_sp_s_a__blk1454_dn6, locals.var_sp_s_a__blk1454_dn7, locals.var_sp_s_a__blk1454_dn8, locals.var_sp_s_a__blk1454_dn9, ) = (assign52090_e66961, assign52090_e66961_d_n4, assign52090_e66961_d_n6, assign52090_e66961_d_n7, assign52090_e66961_d_n8, assign52090_e66961_d_n9, );
            locals.var_sp_s_a__blk1454_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52100_e66976: f64 = (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462);
            let assign52100_e66977: f64 = (locals.var_sp_s_temp1__blk1449 - assign52100_e66976);
            let assign52100_e66978: f64 = (locals.var_gf2__blk1325 * assign52100_e66977);
            let assign52100_e66979: f64 = (0.5 * assign52100_e66978);
            let assign52100_e66980: f64 = (1.0 - assign52100_e66979);
            (locals.var_sp_s_b__blk1471, locals.var_sp_s_b__blk1471_dn4, locals.var_sp_s_b__blk1471_dn6, locals.var_sp_s_b__blk1471_dn7, locals.var_sp_s_b__blk1471_dn8, locals.var_sp_s_b__blk1471_dn9, ) = (assign52100_e66980, (-(0.5 * ((locals.var_gf2__blk1325_dn4 * assign52100_e66977) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn4 - ((locals.var_delta_nd__blk1409_dn4 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn4))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn6 * assign52100_e66977) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn6 - ((locals.var_delta_nd__blk1409_dn6 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn6))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn7 * assign52100_e66977) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn7 - ((locals.var_delta_nd__blk1409_dn7 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn7))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn8 * assign52100_e66977) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn8 - ((locals.var_delta_nd__blk1409_dn8 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn8))))))), (-(0.5 * ((locals.var_gf2__blk1325_dn9 * assign52100_e66977) + (locals.var_gf2__blk1325 * (locals.var_sp_s_temp1__blk1449_dn9 - ((locals.var_delta_nd__blk1409_dn9 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn9))))))), );
            locals.var_sp_s_b__blk1471_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52110_e66991: f64 = (2.0 * locals.var_sp_s_temp__blk1448);
            let assign52110_e66995: f64 = (1.0 - locals.var_sp_s_temp1__blk1449);
            let assign52110_e66999: f64 = (1.0 + locals.var_sp_s_xi1__blk1461);
            let assign52110_e67000: f64 = (locals.var_delta_nd__blk1409 * assign52110_e66999);
            let assign52110_e67001: f64 = (assign52110_e66995 - assign52110_e67000);
            let assign52110_e67002: f64 = (locals.var_gf2__blk1325 * assign52110_e67001);
            let assign52110_e67003: f64 = (assign52110_e66991 + assign52110_e67002);
            (locals.var_sp_s_c__blk1455, locals.var_sp_s_c__blk1455_dn4, locals.var_sp_s_c__blk1455_dn6, locals.var_sp_s_c__blk1455_dn7, locals.var_sp_s_c__blk1455_dn8, locals.var_sp_s_c__blk1455_dn9, ) = (assign52110_e67003, ((2.0 * locals.var_sp_s_temp__blk1448_dn4) + ((locals.var_gf2__blk1325_dn4 * assign52110_e67001) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn4) - ((locals.var_delta_nd__blk1409_dn4 * assign52110_e66999) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn4)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn6) + ((locals.var_gf2__blk1325_dn6 * assign52110_e67001) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn6) - ((locals.var_delta_nd__blk1409_dn6 * assign52110_e66999) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn7) + ((locals.var_gf2__blk1325_dn7 * assign52110_e67001) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn7) - ((locals.var_delta_nd__blk1409_dn7 * assign52110_e66999) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn8) + ((locals.var_gf2__blk1325_dn8 * assign52110_e67001) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn8) - ((locals.var_delta_nd__blk1409_dn8 * assign52110_e66999) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn8)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn9) + ((locals.var_gf2__blk1325_dn9 * assign52110_e67001) + (locals.var_gf2__blk1325 * ((-locals.var_sp_s_temp1__blk1449_dn9) - ((locals.var_delta_nd__blk1409_dn9 * assign52110_e66999) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn9)))))), );
            locals.var_sp_s_c__blk1455_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52120_e67014: f64 = (locals.var_xn_d__blk1407 - locals.var_sp_s_eta__blk1453);
            let assign52120_e67017: f64 = (locals.var_sp_s_a__blk1454 / locals.var_gf2__blk1325);
            let assign52120_e67018: f64 = (assign52120_e67017).ln();
            let assign52120_e67019: f64 = (assign52120_e67014 + assign52120_e67018);
            (locals.var_sp_s_tau__blk1456, locals.var_sp_s_tau__blk1456_dn4, locals.var_sp_s_tau__blk1456_dn6, locals.var_sp_s_tau__blk1456_dn7, locals.var_sp_s_tau__blk1456_dn8, locals.var_sp_s_tau__blk1456_dn9, ) = (assign52120_e67019, ((locals.var_xn_d__blk1407_dn4 - locals.var_sp_s_eta__blk1453_dn4) + ((((locals.var_sp_s_a__blk1454_dn4 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn4)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign52120_e67017)), ((locals.var_xn_d__blk1407_dn6 - locals.var_sp_s_eta__blk1453_dn6) + ((((locals.var_sp_s_a__blk1454_dn6 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn6)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign52120_e67017)), ((locals.var_xn_d__blk1407_dn7 - locals.var_sp_s_eta__blk1453_dn7) + ((((locals.var_sp_s_a__blk1454_dn7 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn7)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign52120_e67017)), ((locals.var_xn_d__blk1407_dn8 - locals.var_sp_s_eta__blk1453_dn8) + ((((locals.var_sp_s_a__blk1454_dn8 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn8)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign52120_e67017)), ((locals.var_xn_d__blk1407_dn9 - locals.var_sp_s_eta__blk1453_dn9) + ((((locals.var_sp_s_a__blk1454_dn9 * locals.var_gf2__blk1325) - (locals.var_sp_s_a__blk1454 * locals.var_gf2__blk1325_dn9)) / (locals.var_gf2__blk1325 * locals.var_gf2__blk1325)) / assign52120_e67017)), );
            locals.var_sp_s_tau__blk1456_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52130_e67030: f64 = (locals.var_sp_s_a__blk1454 + locals.var_sp_s_c__blk1455);
            (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9, ) = (assign52130_e67030, (locals.var_sp_s_a__blk1454_dn4 + locals.var_sp_s_c__blk1455_dn4), (locals.var_sp_s_a__blk1454_dn6 + locals.var_sp_s_c__blk1455_dn6), (locals.var_sp_s_a__blk1454_dn7 + locals.var_sp_s_c__blk1455_dn7), (locals.var_sp_s_a__blk1454_dn8 + locals.var_sp_s_c__blk1455_dn8), (locals.var_sp_s_a__blk1454_dn9 + locals.var_sp_s_c__blk1455_dn9), );
            locals.var_nu_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52140_e67041: f64 = (locals.var_nu * locals.var_nu);
            let assign52140_e67046: f64 = (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455);
            let assign52140_e67047: f64 = (0.5 * assign52140_e67046);
            let assign52140_e67050: f64 = (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471);
            let assign52140_e67051: f64 = (assign52140_e67047 - assign52140_e67050);
            let assign52140_e67052: f64 = (locals.var_sp_s_tau__blk1456 * assign52140_e67051);
            let assign52140_e67053: f64 = (assign52140_e67041 + assign52140_e67052);
            (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9, ) = (assign52140_e67053, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_sp_s_tau__blk1456_dn4 * assign52140_e67051) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn4 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn4))) - ((locals.var_sp_s_a__blk1454_dn4 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn4)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau__blk1456_dn6 * assign52140_e67051) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn6 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn6))) - ((locals.var_sp_s_a__blk1454_dn6 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau__blk1456_dn7 * assign52140_e67051) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn7 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn7))) - ((locals.var_sp_s_a__blk1454_dn7 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau__blk1456_dn8 * assign52140_e67051) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn8 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn8))) - ((locals.var_sp_s_a__blk1454_dn8 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn8)))))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_sp_s_tau__blk1456_dn9 * assign52140_e67051) + (locals.var_sp_s_tau__blk1456 * ((0.5 * ((locals.var_sp_s_c__blk1455_dn9 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn9))) - ((locals.var_sp_s_a__blk1454_dn9 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn9)))))), );
            locals.var_mutau_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52150_e67065: f64 = (locals.var_sp_s_a__blk1454 * locals.var_nu);
            let assign52150_e67067: f64 = (assign52150_e67065 * locals.var_sp_s_tau__blk1456);
            let assign52150_e67071: f64 = (locals.var_nu / locals.var_mutau);
            let assign52150_e67073: f64 = (assign52150_e67071 * locals.var_sp_s_tau__blk1456);
            let assign52150_e67075: f64 = (assign52150_e67073 * locals.var_sp_s_tau__blk1456);
            let assign52150_e67077: f64 = (assign52150_e67075 * locals.var_sp_s_c__blk1455);
            let assign52150_e67080: f64 = (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455);
            let assign52150_e67082: f64 = (assign52150_e67080 * 0.3333333333333333);
            let assign52150_e67085: f64 = (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471);
            let assign52150_e67086: f64 = (assign52150_e67082 - assign52150_e67085);
            let assign52150_e67087: f64 = (assign52150_e67077 * assign52150_e67086);
            let assign52150_e67088: f64 = (locals.var_mutau + assign52150_e67087);
            let assign52150_e67089: f64 = (assign52150_e67067 / assign52150_e67088);
            let assign52150_e67090: f64 = (locals.var_sp_s_eta__blk1453 + assign52150_e67089);
            (locals.var_sp_s_x0__blk1472, locals.var_sp_s_x0__blk1472_dn4, locals.var_sp_s_x0__blk1472_dn6, locals.var_sp_s_x0__blk1472_dn7, locals.var_sp_s_x0__blk1472_dn8, locals.var_sp_s_x0__blk1472_dn9, ) = (assign52150_e67090, (locals.var_sp_s_eta__blk1453_dn4 + (((((((locals.var_sp_s_a__blk1454_dn4 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn4)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67065 * locals.var_sp_s_tau__blk1456_dn4)) * assign52150_e67088) - (assign52150_e67067 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67071 * locals.var_sp_s_tau__blk1456_dn4)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67073 * locals.var_sp_s_tau__blk1456_dn4)) * locals.var_sp_s_c__blk1455) + (assign52150_e67075 * locals.var_sp_s_c__blk1455_dn4)) * assign52150_e67086) + (assign52150_e67077 * ((((locals.var_sp_s_c__blk1455_dn4 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn4)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn4 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn4)))))))) / (assign52150_e67088 * assign52150_e67088))), (locals.var_sp_s_eta__blk1453_dn6 + (((((((locals.var_sp_s_a__blk1454_dn6 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn6)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67065 * locals.var_sp_s_tau__blk1456_dn6)) * assign52150_e67088) - (assign52150_e67067 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67071 * locals.var_sp_s_tau__blk1456_dn6)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67073 * locals.var_sp_s_tau__blk1456_dn6)) * locals.var_sp_s_c__blk1455) + (assign52150_e67075 * locals.var_sp_s_c__blk1455_dn6)) * assign52150_e67086) + (assign52150_e67077 * ((((locals.var_sp_s_c__blk1455_dn6 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn6)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn6 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn6)))))))) / (assign52150_e67088 * assign52150_e67088))), (locals.var_sp_s_eta__blk1453_dn7 + (((((((locals.var_sp_s_a__blk1454_dn7 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn7)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67065 * locals.var_sp_s_tau__blk1456_dn7)) * assign52150_e67088) - (assign52150_e67067 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67071 * locals.var_sp_s_tau__blk1456_dn7)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67073 * locals.var_sp_s_tau__blk1456_dn7)) * locals.var_sp_s_c__blk1455) + (assign52150_e67075 * locals.var_sp_s_c__blk1455_dn7)) * assign52150_e67086) + (assign52150_e67077 * ((((locals.var_sp_s_c__blk1455_dn7 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn7)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn7 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn7)))))))) / (assign52150_e67088 * assign52150_e67088))), (locals.var_sp_s_eta__blk1453_dn8 + (((((((locals.var_sp_s_a__blk1454_dn8 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn8)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67065 * locals.var_sp_s_tau__blk1456_dn8)) * assign52150_e67088) - (assign52150_e67067 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67071 * locals.var_sp_s_tau__blk1456_dn8)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67073 * locals.var_sp_s_tau__blk1456_dn8)) * locals.var_sp_s_c__blk1455) + (assign52150_e67075 * locals.var_sp_s_c__blk1455_dn8)) * assign52150_e67086) + (assign52150_e67077 * ((((locals.var_sp_s_c__blk1455_dn8 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn8)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn8 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn8)))))))) / (assign52150_e67088 * assign52150_e67088))), (locals.var_sp_s_eta__blk1453_dn9 + (((((((locals.var_sp_s_a__blk1454_dn9 * locals.var_nu) + (locals.var_sp_s_a__blk1454 * locals.var_nu_dn9)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67065 * locals.var_sp_s_tau__blk1456_dn9)) * assign52150_e67088) - (assign52150_e67067 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67071 * locals.var_sp_s_tau__blk1456_dn9)) * locals.var_sp_s_tau__blk1456) + (assign52150_e67073 * locals.var_sp_s_tau__blk1456_dn9)) * locals.var_sp_s_c__blk1455) + (assign52150_e67075 * locals.var_sp_s_c__blk1455_dn9)) * assign52150_e67086) + (assign52150_e67077 * ((((locals.var_sp_s_c__blk1455_dn9 * locals.var_sp_s_c__blk1455) + (locals.var_sp_s_c__blk1455 * locals.var_sp_s_c__blk1455_dn9)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1454_dn9 * locals.var_sp_s_b__blk1471) + (locals.var_sp_s_a__blk1454 * locals.var_sp_s_b__blk1471_dn9)))))))) / (assign52150_e67088 * assign52150_e67088))), );
            locals.var_sp_s_x0__blk1472_rv = 0.0;
        }

        let assign52160_e67095: f64 = if locals.var_sp_s_x0__blk1472 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1510 = assign52160_e67095;
        locals.var_guard1510_rv = 0.0;

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 != 0.0)) {
            let assign52170_e67105: f64 = (locals.var_sp_s_x0__blk1472).exp();
            (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9, ) = (assign52170_e67105, (assign52170_e67105 * locals.var_sp_s_x0__blk1472_dn4), (assign52170_e67105 * locals.var_sp_s_x0__blk1472_dn6), (assign52170_e67105 * locals.var_sp_s_x0__blk1472_dn7), (assign52170_e67105 * locals.var_sp_s_x0__blk1472_dn8), (assign52170_e67105 * locals.var_sp_s_x0__blk1472_dn9), );
            locals.var_sp_s_delta0__blk1458_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 != 0.0)) {
            let assign52180_e67118: f64 = (1.0 / locals.var_sp_s_delta0__blk1458);
            (locals.var_sp_s_delta1__blk1459, locals.var_sp_s_delta1__blk1459_dn4, locals.var_sp_s_delta1__blk1459_dn6, locals.var_sp_s_delta1__blk1459_dn7, locals.var_sp_s_delta1__blk1459_dn8, locals.var_sp_s_delta1__blk1459_dn9, ) = (assign52180_e67118, (-(locals.var_sp_s_delta0__blk1458_dn4 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn6 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn7 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn8 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), (-(locals.var_sp_s_delta0__blk1458_dn9 / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458))), );
            locals.var_sp_s_delta1__blk1459_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 != 0.0)) {
            let assign52190_e67131: f64 = (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458);
            (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9, ) = (assign52190_e67131, ((locals.var_delta_nd__blk1409_dn4 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn4)), ((locals.var_delta_nd__blk1409_dn6 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn6)), ((locals.var_delta_nd__blk1409_dn7 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn7)), ((locals.var_delta_nd__blk1409_dn8 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn8)), ((locals.var_delta_nd__blk1409_dn9 * locals.var_sp_s_delta0__blk1458) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn9)), );
            locals.var_sp_s_delta0__blk1458_rv = 0.0;
        }

        let assign52200_e67137: f64 = (locals.var_xn_d__blk1407 - 230.25850929940458);
        let assign52200_e67138: f64 = if locals.var_sp_s_x0__blk1472 > assign52200_e67137 { 1.0 } else { 0.0 };
        locals.var_guard1511 = assign52200_e67138;
        locals.var_guard1511_rv = 0.0;

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 == 0.0)) && (locals.var_guard1511 != 0.0)) {
            let assign52210_e67152: f64 = (locals.var_sp_s_x0__blk1472 - locals.var_xn_d__blk1407);
            let assign52210_e67153: f64 = (assign52210_e67152).exp();
            (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9, ) = (assign52210_e67153, (assign52210_e67153 * (locals.var_sp_s_x0__blk1472_dn4 - locals.var_xn_d__blk1407_dn4)), (assign52210_e67153 * (locals.var_sp_s_x0__blk1472_dn6 - locals.var_xn_d__blk1407_dn6)), (assign52210_e67153 * (locals.var_sp_s_x0__blk1472_dn7 - locals.var_xn_d__blk1407_dn7)), (assign52210_e67153 * (locals.var_sp_s_x0__blk1472_dn8 - locals.var_xn_d__blk1407_dn8)), (assign52210_e67153 * (locals.var_sp_s_x0__blk1472_dn9 - locals.var_xn_d__blk1407_dn9)), );
            locals.var_sp_s_delta0__blk1458_rv = 0.0;
        }

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 == 0.0)) && (locals.var_guard1511 != 0.0)) {
            let assign52220_e67169: f64 = (locals.var_delta_nd__blk1409 / locals.var_sp_s_delta0__blk1458);
            (locals.var_sp_s_delta1__blk1459, locals.var_sp_s_delta1__blk1459_dn4, locals.var_sp_s_delta1__blk1459_dn6, locals.var_sp_s_delta1__blk1459_dn7, locals.var_sp_s_delta1__blk1459_dn8, locals.var_sp_s_delta1__blk1459_dn9, ) = (assign52220_e67169, (((locals.var_delta_nd__blk1409_dn4 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn4)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_nd__blk1409_dn6 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn6)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_nd__blk1409_dn7 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn7)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_nd__blk1409_dn8 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn8)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), (((locals.var_delta_nd__blk1409_dn9 * locals.var_sp_s_delta0__blk1458) - (locals.var_delta_nd__blk1409 * locals.var_sp_s_delta0__blk1458_dn9)) / (locals.var_sp_s_delta0__blk1458 * locals.var_sp_s_delta0__blk1458)), );
            locals.var_sp_s_delta1__blk1459_rv = 0.0;
        }

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 == 0.0)) && (locals.var_guard1511 == 0.0)) {
            let assign52230_e67188: f64 = (locals.var_xn_d__blk1407 - locals.var_sp_s_x0__blk1472);
            let assign52230_e67190: f64 = (assign52230_e67188 - 230.25850929940458);
            let assign52230_e67195: f64 = (locals.var_xn_d__blk1407 - locals.var_sp_s_x0__blk1472);
            let assign52230_e67197: f64 = (assign52230_e67195 - 230.25850929940458);
            let assign52230_e67201: f64 = (locals.var_xn_d__blk1407 - locals.var_sp_s_x0__blk1472);
            let assign52230_e67203: f64 = (assign52230_e67201 - 230.25850929940458);
            let assign52230_e67205: f64 = (assign52230_e67203 * 0.3333333333333333);
            let assign52230_e67206: f64 = (1.0 + assign52230_e67205);
            let assign52230_e67207: f64 = (assign52230_e67197 * assign52230_e67206);
            let assign52230_e67208: f64 = (0.5 * assign52230_e67207);
            let assign52230_e67209: f64 = (1.0 + assign52230_e67208);
            let assign52230_e67210: f64 = (assign52230_e67190 * assign52230_e67209);
            let assign52230_e67211: f64 = (1.0 + assign52230_e67210);
            let assign52230_e67212: f64 = (1e-100 / assign52230_e67211);
            (locals.var_sp_s_delta0__blk1458, locals.var_sp_s_delta0__blk1458_dn4, locals.var_sp_s_delta0__blk1458_dn6, locals.var_sp_s_delta0__blk1458_dn7, locals.var_sp_s_delta0__blk1458_dn8, locals.var_sp_s_delta0__blk1458_dn9, ) = (assign52230_e67212, (-((1e-100 * (((locals.var_xn_d__blk1407_dn4 - locals.var_sp_s_x0__blk1472_dn4) * assign52230_e67209) + (assign52230_e67190 * (0.5 * (((locals.var_xn_d__blk1407_dn4 - locals.var_sp_s_x0__blk1472_dn4) * assign52230_e67206) + (assign52230_e67197 * ((locals.var_xn_d__blk1407_dn4 - locals.var_sp_s_x0__blk1472_dn4) * 0.3333333333333333))))))) / (assign52230_e67211 * assign52230_e67211))), (-((1e-100 * (((locals.var_xn_d__blk1407_dn6 - locals.var_sp_s_x0__blk1472_dn6) * assign52230_e67209) + (assign52230_e67190 * (0.5 * (((locals.var_xn_d__blk1407_dn6 - locals.var_sp_s_x0__blk1472_dn6) * assign52230_e67206) + (assign52230_e67197 * ((locals.var_xn_d__blk1407_dn6 - locals.var_sp_s_x0__blk1472_dn6) * 0.3333333333333333))))))) / (assign52230_e67211 * assign52230_e67211))), (-((1e-100 * (((locals.var_xn_d__blk1407_dn7 - locals.var_sp_s_x0__blk1472_dn7) * assign52230_e67209) + (assign52230_e67190 * (0.5 * (((locals.var_xn_d__blk1407_dn7 - locals.var_sp_s_x0__blk1472_dn7) * assign52230_e67206) + (assign52230_e67197 * ((locals.var_xn_d__blk1407_dn7 - locals.var_sp_s_x0__blk1472_dn7) * 0.3333333333333333))))))) / (assign52230_e67211 * assign52230_e67211))), (-((1e-100 * (((locals.var_xn_d__blk1407_dn8 - locals.var_sp_s_x0__blk1472_dn8) * assign52230_e67209) + (assign52230_e67190 * (0.5 * (((locals.var_xn_d__blk1407_dn8 - locals.var_sp_s_x0__blk1472_dn8) * assign52230_e67206) + (assign52230_e67197 * ((locals.var_xn_d__blk1407_dn8 - locals.var_sp_s_x0__blk1472_dn8) * 0.3333333333333333))))))) / (assign52230_e67211 * assign52230_e67211))), (-((1e-100 * (((locals.var_xn_d__blk1407_dn9 - locals.var_sp_s_x0__blk1472_dn9) * assign52230_e67209) + (assign52230_e67190 * (0.5 * (((locals.var_xn_d__blk1407_dn9 - locals.var_sp_s_x0__blk1472_dn9) * assign52230_e67206) + (assign52230_e67197 * ((locals.var_xn_d__blk1407_dn9 - locals.var_sp_s_x0__blk1472_dn9) * 0.3333333333333333))))))) / (assign52230_e67211 * assign52230_e67211))), );
            locals.var_sp_s_delta0__blk1458_rv = 0.0;
        }

        if (((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) && (locals.var_guard1510 == 0.0)) && (locals.var_guard1511 == 0.0)) {
            let assign52240_e67231: f64 = (locals.var_sp_s_x0__blk1472 - 230.25850929940458);
            let assign52240_e67236: f64 = (locals.var_sp_s_x0__blk1472 - 230.25850929940458);
            let assign52240_e67240: f64 = (locals.var_sp_s_x0__blk1472 - 230.25850929940458);
            let assign52240_e67242: f64 = (assign52240_e67240 * 0.3333333333333333);
            let assign52240_e67243: f64 = (1.0 + assign52240_e67242);
            let assign52240_e67244: f64 = (assign52240_e67236 * assign52240_e67243);
            let assign52240_e67245: f64 = (0.5 * assign52240_e67244);
            let assign52240_e67246: f64 = (1.0 + assign52240_e67245);
            let assign52240_e67247: f64 = (assign52240_e67231 * assign52240_e67246);
            let assign52240_e67248: f64 = (1.0 + assign52240_e67247);
            let assign52240_e67249: f64 = (1e-100 / assign52240_e67248);
            (locals.var_sp_s_delta1__blk1459, locals.var_sp_s_delta1__blk1459_dn4, locals.var_sp_s_delta1__blk1459_dn6, locals.var_sp_s_delta1__blk1459_dn7, locals.var_sp_s_delta1__blk1459_dn8, locals.var_sp_s_delta1__blk1459_dn9, ) = (assign52240_e67249, (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn4 * assign52240_e67246) + (assign52240_e67231 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn4 * assign52240_e67243) + (assign52240_e67236 * (locals.var_sp_s_x0__blk1472_dn4 * 0.3333333333333333))))))) / (assign52240_e67248 * assign52240_e67248))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn6 * assign52240_e67246) + (assign52240_e67231 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn6 * assign52240_e67243) + (assign52240_e67236 * (locals.var_sp_s_x0__blk1472_dn6 * 0.3333333333333333))))))) / (assign52240_e67248 * assign52240_e67248))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn7 * assign52240_e67246) + (assign52240_e67231 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn7 * assign52240_e67243) + (assign52240_e67236 * (locals.var_sp_s_x0__blk1472_dn7 * 0.3333333333333333))))))) / (assign52240_e67248 * assign52240_e67248))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn8 * assign52240_e67246) + (assign52240_e67231 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn8 * assign52240_e67243) + (assign52240_e67236 * (locals.var_sp_s_x0__blk1472_dn8 * 0.3333333333333333))))))) / (assign52240_e67248 * assign52240_e67248))), (-((1e-100 * ((locals.var_sp_s_x0__blk1472_dn9 * assign52240_e67246) + (assign52240_e67231 * (0.5 * ((locals.var_sp_s_x0__blk1472_dn9 * assign52240_e67243) + (assign52240_e67236 * (locals.var_sp_s_x0__blk1472_dn9 * 0.3333333333333333))))))) / (assign52240_e67248 * assign52240_e67248))), );
            locals.var_sp_s_delta1__blk1459_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52250_e67262: f64 = (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472);
            let assign52250_e67263: f64 = (2.0 + assign52250_e67262);
            let assign52250_e67264: f64 = (1.0 / assign52250_e67263);
            (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9, ) = (assign52250_e67264, (-(((locals.var_sp_s_x0__blk1472_dn4 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn4)) / (assign52250_e67263 * assign52250_e67263))), (-(((locals.var_sp_s_x0__blk1472_dn6 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn6)) / (assign52250_e67263 * assign52250_e67263))), (-(((locals.var_sp_s_x0__blk1472_dn7 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn7)) / (assign52250_e67263 * assign52250_e67263))), (-(((locals.var_sp_s_x0__blk1472_dn8 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn8)) / (assign52250_e67263 * assign52250_e67263))), (-(((locals.var_sp_s_x0__blk1472_dn9 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn9)) / (assign52250_e67263 * assign52250_e67263))), );
            locals.var_sp_s_temp__blk1448_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52260_e67275: f64 = (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472);
            let assign52260_e67277: f64 = (assign52260_e67275 * locals.var_sp_s_temp__blk1448);
            (locals.var_sp_s_xi0__blk1460, locals.var_sp_s_xi0__blk1460_dn4, locals.var_sp_s_xi0__blk1460_dn6, locals.var_sp_s_xi0__blk1460_dn7, locals.var_sp_s_xi0__blk1460_dn8, locals.var_sp_s_xi0__blk1460_dn9, ) = (assign52260_e67277, ((((locals.var_sp_s_x0__blk1472_dn4 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn4)) * locals.var_sp_s_temp__blk1448) + (assign52260_e67275 * locals.var_sp_s_temp__blk1448_dn4)), ((((locals.var_sp_s_x0__blk1472_dn6 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn6)) * locals.var_sp_s_temp__blk1448) + (assign52260_e67275 * locals.var_sp_s_temp__blk1448_dn6)), ((((locals.var_sp_s_x0__blk1472_dn7 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn7)) * locals.var_sp_s_temp__blk1448) + (assign52260_e67275 * locals.var_sp_s_temp__blk1448_dn7)), ((((locals.var_sp_s_x0__blk1472_dn8 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn8)) * locals.var_sp_s_temp__blk1448) + (assign52260_e67275 * locals.var_sp_s_temp__blk1448_dn8)), ((((locals.var_sp_s_x0__blk1472_dn9 * locals.var_sp_s_x0__blk1472) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_x0__blk1472_dn9)) * locals.var_sp_s_temp__blk1448) + (assign52260_e67275 * locals.var_sp_s_temp__blk1448_dn9)), );
            locals.var_sp_s_xi0__blk1460_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52270_e67289: f64 = (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448);
            let assign52270_e67291: f64 = (assign52270_e67289 * locals.var_sp_s_temp__blk1448);
            let assign52270_e67292: f64 = (4.0 * assign52270_e67291);
            (locals.var_sp_s_xi1__blk1461, locals.var_sp_s_xi1__blk1461_dn4, locals.var_sp_s_xi1__blk1461_dn6, locals.var_sp_s_xi1__blk1461_dn7, locals.var_sp_s_xi1__blk1461_dn8, locals.var_sp_s_xi1__blk1461_dn9, ) = (assign52270_e67292, (4.0 * ((((locals.var_sp_s_x0__blk1472_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn4)) * locals.var_sp_s_temp__blk1448) + (assign52270_e67289 * locals.var_sp_s_temp__blk1448_dn4))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn6)) * locals.var_sp_s_temp__blk1448) + (assign52270_e67289 * locals.var_sp_s_temp__blk1448_dn6))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn7)) * locals.var_sp_s_temp__blk1448) + (assign52270_e67289 * locals.var_sp_s_temp__blk1448_dn7))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn8)) * locals.var_sp_s_temp__blk1448) + (assign52270_e67289 * locals.var_sp_s_temp__blk1448_dn8))), (4.0 * ((((locals.var_sp_s_x0__blk1472_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_x0__blk1472 * locals.var_sp_s_temp__blk1448_dn9)) * locals.var_sp_s_temp__blk1448) + (assign52270_e67289 * locals.var_sp_s_temp__blk1448_dn9))), );
            locals.var_sp_s_xi1__blk1461_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52280_e67303: f64 = (8.0 * locals.var_sp_s_temp__blk1448);
            let assign52280_e67306: f64 = (12.0 * locals.var_sp_s_xi0__blk1460);
            let assign52280_e67307: f64 = (assign52280_e67303 - assign52280_e67306);
            let assign52280_e67309: f64 = (assign52280_e67307 * locals.var_sp_s_temp__blk1448);
            let assign52280_e67311: f64 = (assign52280_e67309 * locals.var_sp_s_temp__blk1448);
            (locals.var_sp_s_xi2__blk1462, locals.var_sp_s_xi2__blk1462_dn4, locals.var_sp_s_xi2__blk1462_dn6, locals.var_sp_s_xi2__blk1462_dn7, locals.var_sp_s_xi2__blk1462_dn8, locals.var_sp_s_xi2__blk1462_dn9, ) = (assign52280_e67311, ((((((8.0 * locals.var_sp_s_temp__blk1448_dn4) - (12.0 * locals.var_sp_s_xi0__blk1460_dn4)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67307 * locals.var_sp_s_temp__blk1448_dn4)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67309 * locals.var_sp_s_temp__blk1448_dn4)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn6) - (12.0 * locals.var_sp_s_xi0__blk1460_dn6)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67307 * locals.var_sp_s_temp__blk1448_dn6)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67309 * locals.var_sp_s_temp__blk1448_dn6)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn7) - (12.0 * locals.var_sp_s_xi0__blk1460_dn7)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67307 * locals.var_sp_s_temp__blk1448_dn7)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67309 * locals.var_sp_s_temp__blk1448_dn7)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn8) - (12.0 * locals.var_sp_s_xi0__blk1460_dn8)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67307 * locals.var_sp_s_temp__blk1448_dn8)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67309 * locals.var_sp_s_temp__blk1448_dn8)), ((((((8.0 * locals.var_sp_s_temp__blk1448_dn9) - (12.0 * locals.var_sp_s_xi0__blk1460_dn9)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67307 * locals.var_sp_s_temp__blk1448_dn9)) * locals.var_sp_s_temp__blk1448) + (assign52280_e67309 * locals.var_sp_s_temp__blk1448_dn9)), );
            locals.var_sp_s_xi2__blk1462_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52290_e67322: f64 = (locals.var_xg__blk1343 - locals.var_sp_s_x0__blk1472);
            (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9, ) = (assign52290_e67322, (locals.var_xg__blk1343_dn4 - locals.var_sp_s_x0__blk1472_dn4), (locals.var_xg__blk1343_dn6 - locals.var_sp_s_x0__blk1472_dn6), (locals.var_xg__blk1343_dn7 - locals.var_sp_s_x0__blk1472_dn7), (locals.var_xg__blk1343_dn8 - locals.var_sp_s_x0__blk1472_dn8), (locals.var_xg__blk1343_dn9 - locals.var_sp_s_x0__blk1472_dn9), );
            locals.var_sp_s_temp__blk1448_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52300_e67333: f64 = (2.0 * locals.var_sp_s_temp__blk1448);
            let assign52300_e67337: f64 = (1.0 - locals.var_sp_s_delta1__blk1459);
            let assign52300_e67339: f64 = (assign52300_e67337 + locals.var_sp_s_delta0__blk1458);
            let assign52300_e67343: f64 = (1.0 + locals.var_sp_s_xi1__blk1461);
            let assign52300_e67344: f64 = (locals.var_delta_nd__blk1409 * assign52300_e67343);
            let assign52300_e67345: f64 = (assign52300_e67339 - assign52300_e67344);
            let assign52300_e67346: f64 = (locals.var_gf2__blk1325 * assign52300_e67345);
            let assign52300_e67347: f64 = (assign52300_e67333 + assign52300_e67346);
            (locals.var_sp_s_pc__blk1463, locals.var_sp_s_pc__blk1463_dn4, locals.var_sp_s_pc__blk1463_dn6, locals.var_sp_s_pc__blk1463_dn7, locals.var_sp_s_pc__blk1463_dn8, locals.var_sp_s_pc__blk1463_dn9, ) = (assign52300_e67347, ((2.0 * locals.var_sp_s_temp__blk1448_dn4) + ((locals.var_gf2__blk1325_dn4 * assign52300_e67345) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn4) + locals.var_sp_s_delta0__blk1458_dn4) - ((locals.var_delta_nd__blk1409_dn4 * assign52300_e67343) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn4)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn6) + ((locals.var_gf2__blk1325_dn6 * assign52300_e67345) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn6) + locals.var_sp_s_delta0__blk1458_dn6) - ((locals.var_delta_nd__blk1409_dn6 * assign52300_e67343) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn7) + ((locals.var_gf2__blk1325_dn7 * assign52300_e67345) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn7) + locals.var_sp_s_delta0__blk1458_dn7) - ((locals.var_delta_nd__blk1409_dn7 * assign52300_e67343) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn8) + ((locals.var_gf2__blk1325_dn8 * assign52300_e67345) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn8) + locals.var_sp_s_delta0__blk1458_dn8) - ((locals.var_delta_nd__blk1409_dn8 * assign52300_e67343) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn8)))))), ((2.0 * locals.var_sp_s_temp__blk1448_dn9) + ((locals.var_gf2__blk1325_dn9 * assign52300_e67345) + (locals.var_gf2__blk1325 * (((-locals.var_sp_s_delta1__blk1459_dn9) + locals.var_sp_s_delta0__blk1458_dn9) - ((locals.var_delta_nd__blk1409_dn9 * assign52300_e67343) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi1__blk1461_dn9)))))), );
            locals.var_sp_s_pc__blk1463_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52310_e67358: f64 = (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448);
            let assign52310_e67362: f64 = (locals.var_sp_s_delta1__blk1459 + locals.var_sp_s_x0__blk1472);
            let assign52310_e67364: f64 = (assign52310_e67362 - 1.0);
            let assign52310_e67366: f64 = (assign52310_e67364 + locals.var_sp_s_delta0__blk1458);
            let assign52310_e67370: f64 = (locals.var_sp_s_x0__blk1472 + 1.0);
            let assign52310_e67372: f64 = (assign52310_e67370 + locals.var_sp_s_xi0__blk1460);
            let assign52310_e67373: f64 = (locals.var_delta_nd__blk1409 * assign52310_e67372);
            let assign52310_e67374: f64 = (assign52310_e67366 - assign52310_e67373);
            let assign52310_e67375: f64 = (locals.var_gf2__blk1325 * assign52310_e67374);
            let assign52310_e67376: f64 = (assign52310_e67358 - assign52310_e67375);
            (locals.var_sp_s_qc__blk1464, locals.var_sp_s_qc__blk1464_dn4, locals.var_sp_s_qc__blk1464_dn6, locals.var_sp_s_qc__blk1464_dn7, locals.var_sp_s_qc__blk1464_dn8, locals.var_sp_s_qc__blk1464_dn9, ) = (assign52310_e67376, (((locals.var_sp_s_temp__blk1448_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn4)) - ((locals.var_gf2__blk1325_dn4 * assign52310_e67374) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn4 + locals.var_sp_s_x0__blk1472_dn4) + locals.var_sp_s_delta0__blk1458_dn4) - ((locals.var_delta_nd__blk1409_dn4 * assign52310_e67372) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_x0__blk1472_dn4 + locals.var_sp_s_xi0__blk1460_dn4))))))), (((locals.var_sp_s_temp__blk1448_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn6)) - ((locals.var_gf2__blk1325_dn6 * assign52310_e67374) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn6 + locals.var_sp_s_x0__blk1472_dn6) + locals.var_sp_s_delta0__blk1458_dn6) - ((locals.var_delta_nd__blk1409_dn6 * assign52310_e67372) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_x0__blk1472_dn6 + locals.var_sp_s_xi0__blk1460_dn6))))))), (((locals.var_sp_s_temp__blk1448_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn7)) - ((locals.var_gf2__blk1325_dn7 * assign52310_e67374) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn7 + locals.var_sp_s_x0__blk1472_dn7) + locals.var_sp_s_delta0__blk1458_dn7) - ((locals.var_delta_nd__blk1409_dn7 * assign52310_e67372) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_x0__blk1472_dn7 + locals.var_sp_s_xi0__blk1460_dn7))))))), (((locals.var_sp_s_temp__blk1448_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn8)) - ((locals.var_gf2__blk1325_dn8 * assign52310_e67374) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn8 + locals.var_sp_s_x0__blk1472_dn8) + locals.var_sp_s_delta0__blk1458_dn8) - ((locals.var_delta_nd__blk1409_dn8 * assign52310_e67372) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_x0__blk1472_dn8 + locals.var_sp_s_xi0__blk1460_dn8))))))), (((locals.var_sp_s_temp__blk1448_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_temp__blk1448 * locals.var_sp_s_temp__blk1448_dn9)) - ((locals.var_gf2__blk1325_dn9 * assign52310_e67374) + (locals.var_gf2__blk1325 * (((locals.var_sp_s_delta1__blk1459_dn9 + locals.var_sp_s_x0__blk1472_dn9) + locals.var_sp_s_delta0__blk1458_dn9) - ((locals.var_delta_nd__blk1409_dn9 * assign52310_e67372) + (locals.var_delta_nd__blk1409 * (locals.var_sp_s_x0__blk1472_dn9 + locals.var_sp_s_xi0__blk1460_dn9))))))), );
            locals.var_sp_s_qc__blk1464_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52320_e67389: f64 = (locals.var_sp_s_delta1__blk1459 + locals.var_sp_s_delta0__blk1458);
            let assign52320_e67392: f64 = (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462);
            let assign52320_e67393: f64 = (assign52320_e67389 - assign52320_e67392);
            let assign52320_e67394: f64 = (locals.var_gf2__blk1325 * assign52320_e67393);
            let assign52320_e67395: f64 = (2.0 - assign52320_e67394);
            (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9, ) = (assign52320_e67395, (-((locals.var_gf2__blk1325_dn4 * assign52320_e67393) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn4 + locals.var_sp_s_delta0__blk1458_dn4) - ((locals.var_delta_nd__blk1409_dn4 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn4)))))), (-((locals.var_gf2__blk1325_dn6 * assign52320_e67393) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn6 + locals.var_sp_s_delta0__blk1458_dn6) - ((locals.var_delta_nd__blk1409_dn6 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn6)))))), (-((locals.var_gf2__blk1325_dn7 * assign52320_e67393) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn7 + locals.var_sp_s_delta0__blk1458_dn7) - ((locals.var_delta_nd__blk1409_dn7 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn7)))))), (-((locals.var_gf2__blk1325_dn8 * assign52320_e67393) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn8 + locals.var_sp_s_delta0__blk1458_dn8) - ((locals.var_delta_nd__blk1409_dn8 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn8)))))), (-((locals.var_gf2__blk1325_dn9 * assign52320_e67393) + (locals.var_gf2__blk1325 * ((locals.var_sp_s_delta1__blk1459_dn9 + locals.var_sp_s_delta0__blk1458_dn9) - ((locals.var_delta_nd__blk1409_dn9 * locals.var_sp_s_xi2__blk1462) + (locals.var_delta_nd__blk1409 * locals.var_sp_s_xi2__blk1462_dn9)))))), );
            locals.var_sp_s_temp__blk1448_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52330_e67406: f64 = (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463);
            let assign52330_e67410: f64 = (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448);
            let assign52330_e67411: f64 = (2.0 * assign52330_e67410);
            let assign52330_e67412: f64 = (assign52330_e67406 - assign52330_e67411);
            (locals.var_sp_s_temp__blk1448, locals.var_sp_s_temp__blk1448_dn4, locals.var_sp_s_temp__blk1448_dn6, locals.var_sp_s_temp__blk1448_dn7, locals.var_sp_s_temp__blk1448_dn8, locals.var_sp_s_temp__blk1448_dn9, ) = (assign52330_e67412, (((locals.var_sp_s_pc__blk1463_dn4 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn4)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn4 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn4)))), (((locals.var_sp_s_pc__blk1463_dn6 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn6)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn6 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn6)))), (((locals.var_sp_s_pc__blk1463_dn7 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn7)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn7 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn7)))), (((locals.var_sp_s_pc__blk1463_dn8 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn8)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn8 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn8)))), (((locals.var_sp_s_pc__blk1463_dn9 * locals.var_sp_s_pc__blk1463) + (locals.var_sp_s_pc__blk1463 * locals.var_sp_s_pc__blk1463_dn9)) - (2.0 * ((locals.var_sp_s_qc__blk1464_dn9 * locals.var_sp_s_temp__blk1448) + (locals.var_sp_s_qc__blk1464 * locals.var_sp_s_temp__blk1448_dn9)))), );
            locals.var_sp_s_temp__blk1448_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1509 == 0.0)) {
            let assign52340_e67426: f64 = (locals.var_sp_s_temp__blk1448).sqrt();
            let assign52340_e67427: f64 = (locals.var_sp_s_pc__blk1463 + assign52340_e67426);
            let assign52340_e67428: f64 = (locals.var_sp_s_qc__blk1464 / assign52340_e67427);
            let assign52340_e67429: f64 = (2.0 * assign52340_e67428);
            let assign52340_e67430: f64 = (locals.var_sp_s_x0__blk1472 + assign52340_e67429);
            (locals.var_x_d__blk1410, locals.var_x_d__blk1410_dn4, locals.var_x_d__blk1410_dn6, locals.var_x_d__blk1410_dn7, locals.var_x_d__blk1410_dn8, locals.var_x_d__blk1410_dn9, ) = (assign52340_e67430, (locals.var_sp_s_x0__blk1472_dn4 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn4 * assign52340_e67427) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn4 + (locals.var_sp_s_temp__blk1448_dn4 / (2.0 * assign52340_e67426))))) / (assign52340_e67427 * assign52340_e67427)))), (locals.var_sp_s_x0__blk1472_dn6 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn6 * assign52340_e67427) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn6 + (locals.var_sp_s_temp__blk1448_dn6 / (2.0 * assign52340_e67426))))) / (assign52340_e67427 * assign52340_e67427)))), (locals.var_sp_s_x0__blk1472_dn7 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn7 * assign52340_e67427) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn7 + (locals.var_sp_s_temp__blk1448_dn7 / (2.0 * assign52340_e67426))))) / (assign52340_e67427 * assign52340_e67427)))), (locals.var_sp_s_x0__blk1472_dn8 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn8 * assign52340_e67427) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn8 + (locals.var_sp_s_temp__blk1448_dn8 / (2.0 * assign52340_e67426))))) / (assign52340_e67427 * assign52340_e67427)))), (locals.var_sp_s_x0__blk1472_dn9 + (2.0 * (((locals.var_sp_s_qc__blk1464_dn9 * assign52340_e67427) - (locals.var_sp_s_qc__blk1464 * (locals.var_sp_s_pc__blk1463_dn9 + (locals.var_sp_s_temp__blk1448_dn9 / (2.0 * assign52340_e67426))))) / (assign52340_e67427 * assign52340_e67427)))), );
            locals.var_x_d__blk1410_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign52350_e67438: f64 = (locals.var_x_d__blk1410 - locals.var_x_s__blk1363);
            (locals.var_x_ds__blk1411, locals.var_x_ds__blk1411_dn4, locals.var_x_ds__blk1411_dn6, locals.var_x_ds__blk1411_dn7, locals.var_x_ds__blk1411_dn8, locals.var_x_ds__blk1411_dn9, ) = (assign52350_e67438, (locals.var_x_d__blk1410_dn4 - locals.var_x_s__blk1363_dn4), (locals.var_x_d__blk1410_dn6 - locals.var_x_s__blk1363_dn6), (locals.var_x_d__blk1410_dn7 - locals.var_x_s__blk1363_dn7), (locals.var_x_d__blk1410_dn8 - locals.var_x_s__blk1363_dn8), (locals.var_x_d__blk1410_dn9 - locals.var_x_s__blk1363_dn9), );
            locals.var_x_ds__blk1411_rv = 0.0;
        }

        let assign52360_e67443: f64 = if locals.var_x_ds__blk1411 < 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1512 = assign52360_e67443;
        locals.var_guard1512_rv = 0.0;

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1512 != 0.0)) {
            let assign52370_e67452: f64 = (locals.var_xg__blk1343 - locals.var_x_s__blk1363);
            let assign52370_e67453: f64 = (2.0 * assign52370_e67452);
            let assign52370_e67457: f64 = (1.0 - locals.var_es__blk1369);
            let assign52370_e67460: f64 = (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408);
            let assign52370_e67461: f64 = (assign52370_e67457 + assign52370_e67460);
            let assign52370_e67465: f64 = (1.0 + locals.var_xi1s__blk1366);
            let assign52370_e67466: f64 = (locals.var_delta_nd__blk1409 * assign52370_e67465);
            let assign52370_e67467: f64 = (assign52370_e67461 - assign52370_e67466);
            let assign52370_e67468: f64 = (locals.var_gf2__blk1325 * assign52370_e67467);
            let assign52370_e67469: f64 = (assign52370_e67453 + assign52370_e67468);
            (locals.var_pc__blk1412, locals.var_pc__blk1412_dn4, locals.var_pc__blk1412_dn6, locals.var_pc__blk1412_dn7, locals.var_pc__blk1412_dn8, locals.var_pc__blk1412_dn9, ) = (assign52370_e67469, ((2.0 * (locals.var_xg__blk1343_dn4 - locals.var_x_s__blk1363_dn4)) + ((locals.var_gf2__blk1325_dn4 * assign52370_e67467) + (locals.var_gf2__blk1325 * (((-locals.var_es__blk1369_dn4) + ((locals.var_delta_1s__blk1368_dn4 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn4))) - ((locals.var_delta_nd__blk1409_dn4 * assign52370_e67465) + (locals.var_delta_nd__blk1409 * locals.var_xi1s__blk1366_dn4)))))), ((2.0 * (locals.var_xg__blk1343_dn6 - locals.var_x_s__blk1363_dn6)) + ((locals.var_gf2__blk1325_dn6 * assign52370_e67467) + (locals.var_gf2__blk1325 * (((-locals.var_es__blk1369_dn6) + ((locals.var_delta_1s__blk1368_dn6 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn6))) - ((locals.var_delta_nd__blk1409_dn6 * assign52370_e67465) + (locals.var_delta_nd__blk1409 * locals.var_xi1s__blk1366_dn6)))))), ((2.0 * (locals.var_xg__blk1343_dn7 - locals.var_x_s__blk1363_dn7)) + ((locals.var_gf2__blk1325_dn7 * assign52370_e67467) + (locals.var_gf2__blk1325 * (((-locals.var_es__blk1369_dn7) + ((locals.var_delta_1s__blk1368_dn7 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn7))) - ((locals.var_delta_nd__blk1409_dn7 * assign52370_e67465) + (locals.var_delta_nd__blk1409 * locals.var_xi1s__blk1366_dn7)))))), ((2.0 * (locals.var_xg__blk1343_dn8 - locals.var_x_s__blk1363_dn8)) + ((locals.var_gf2__blk1325_dn8 * assign52370_e67467) + (locals.var_gf2__blk1325 * (((-locals.var_es__blk1369_dn8) + ((locals.var_delta_1s__blk1368_dn8 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn8))) - ((locals.var_delta_nd__blk1409_dn8 * assign52370_e67465) + (locals.var_delta_nd__blk1409 * locals.var_xi1s__blk1366_dn8)))))), ((2.0 * (locals.var_xg__blk1343_dn9 - locals.var_x_s__blk1363_dn9)) + ((locals.var_gf2__blk1325_dn9 * assign52370_e67467) + (locals.var_gf2__blk1325 * (((-locals.var_es__blk1369_dn9) + ((locals.var_delta_1s__blk1368_dn9 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn9))) - ((locals.var_delta_nd__blk1409_dn9 * assign52370_e67465) + (locals.var_delta_nd__blk1409 * locals.var_xi1s__blk1366_dn9)))))), );
            locals.var_pc__blk1412_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1512 != 0.0)) {
            let assign52380_e67480: f64 = (1.0 - locals.var_k_ds__blk1408);
            let assign52380_e67481: f64 = (locals.var_gf2__blk1325 * assign52380_e67480);
            let assign52380_e67483: f64 = (assign52380_e67481 * locals.var_ds__blk1370);
            (locals.var_qc__blk1413, locals.var_qc__blk1413_dn4, locals.var_qc__blk1413_dn6, locals.var_qc__blk1413_dn7, locals.var_qc__blk1413_dn8, locals.var_qc__blk1413_dn9, ) = (assign52380_e67483, ((((locals.var_gf2__blk1325_dn4 * assign52380_e67480) + (locals.var_gf2__blk1325 * (-locals.var_k_ds__blk1408_dn4))) * locals.var_ds__blk1370) + (assign52380_e67481 * locals.var_ds__blk1370_dn4)), ((((locals.var_gf2__blk1325_dn6 * assign52380_e67480) + (locals.var_gf2__blk1325 * (-locals.var_k_ds__blk1408_dn6))) * locals.var_ds__blk1370) + (assign52380_e67481 * locals.var_ds__blk1370_dn6)), ((((locals.var_gf2__blk1325_dn7 * assign52380_e67480) + (locals.var_gf2__blk1325 * (-locals.var_k_ds__blk1408_dn7))) * locals.var_ds__blk1370) + (assign52380_e67481 * locals.var_ds__blk1370_dn7)), ((((locals.var_gf2__blk1325_dn8 * assign52380_e67480) + (locals.var_gf2__blk1325 * (-locals.var_k_ds__blk1408_dn8))) * locals.var_ds__blk1370) + (assign52380_e67481 * locals.var_ds__blk1370_dn8)), ((((locals.var_gf2__blk1325_dn9 * assign52380_e67480) + (locals.var_gf2__blk1325 * (-locals.var_k_ds__blk1408_dn9))) * locals.var_ds__blk1370) + (assign52380_e67481 * locals.var_ds__blk1370_dn9)), );
            locals.var_qc__blk1413_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1512 != 0.0)) {
            let assign52390_e67496: f64 = (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408);
            let assign52390_e67497: f64 = (locals.var_es__blk1369 + assign52390_e67496);
            let assign52390_e67500: f64 = (locals.var_delta_nd__blk1409 * locals.var_xi2s__blk1367);
            let assign52390_e67501: f64 = (assign52390_e67497 - assign52390_e67500);
            let assign52390_e67502: f64 = (locals.var_gf2__blk1325 * assign52390_e67501);
            let assign52390_e67503: f64 = (2.0 - assign52390_e67502);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign52390_e67503, (-((locals.var_gf2__blk1325_dn4 * assign52390_e67501) + (locals.var_gf2__blk1325 * ((locals.var_es__blk1369_dn4 + ((locals.var_delta_1s__blk1368_dn4 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn4))) - ((locals.var_delta_nd__blk1409_dn4 * locals.var_xi2s__blk1367) + (locals.var_delta_nd__blk1409 * locals.var_xi2s__blk1367_dn4)))))), (-((locals.var_gf2__blk1325_dn6 * assign52390_e67501) + (locals.var_gf2__blk1325 * ((locals.var_es__blk1369_dn6 + ((locals.var_delta_1s__blk1368_dn6 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn6))) - ((locals.var_delta_nd__blk1409_dn6 * locals.var_xi2s__blk1367) + (locals.var_delta_nd__blk1409 * locals.var_xi2s__blk1367_dn6)))))), (-((locals.var_gf2__blk1325_dn7 * assign52390_e67501) + (locals.var_gf2__blk1325 * ((locals.var_es__blk1369_dn7 + ((locals.var_delta_1s__blk1368_dn7 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn7))) - ((locals.var_delta_nd__blk1409_dn7 * locals.var_xi2s__blk1367) + (locals.var_delta_nd__blk1409 * locals.var_xi2s__blk1367_dn7)))))), (-((locals.var_gf2__blk1325_dn8 * assign52390_e67501) + (locals.var_gf2__blk1325 * ((locals.var_es__blk1369_dn8 + ((locals.var_delta_1s__blk1368_dn8 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn8))) - ((locals.var_delta_nd__blk1409_dn8 * locals.var_xi2s__blk1367) + (locals.var_delta_nd__blk1409 * locals.var_xi2s__blk1367_dn8)))))), (-((locals.var_gf2__blk1325_dn9 * assign52390_e67501) + (locals.var_gf2__blk1325 * ((locals.var_es__blk1369_dn9 + ((locals.var_delta_1s__blk1368_dn9 * locals.var_k_ds__blk1408) + (locals.var_delta_1s__blk1368 * locals.var_k_ds__blk1408_dn9))) - ((locals.var_delta_nd__blk1409_dn9 * locals.var_xi2s__blk1367) + (locals.var_delta_nd__blk1409 * locals.var_xi2s__blk1367_dn9)))))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1512 != 0.0)) {
            let assign52400_e67513: f64 = (locals.var_pc__blk1412 * locals.var_pc__blk1412);
            let assign52400_e67517: f64 = (locals.var_temp__blk949 * locals.var_qc__blk1413);
            let assign52400_e67518: f64 = (2.0 * assign52400_e67517);
            let assign52400_e67519: f64 = (assign52400_e67513 - assign52400_e67518);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign52400_e67519, (((locals.var_pc__blk1412_dn4 * locals.var_pc__blk1412) + (locals.var_pc__blk1412 * locals.var_pc__blk1412_dn4)) - (2.0 * ((locals.var_temp__blk949_dn4 * locals.var_qc__blk1413) + (locals.var_temp__blk949 * locals.var_qc__blk1413_dn4)))), (((locals.var_pc__blk1412_dn6 * locals.var_pc__blk1412) + (locals.var_pc__blk1412 * locals.var_pc__blk1412_dn6)) - (2.0 * ((locals.var_temp__blk949_dn6 * locals.var_qc__blk1413) + (locals.var_temp__blk949 * locals.var_qc__blk1413_dn6)))), (((locals.var_pc__blk1412_dn7 * locals.var_pc__blk1412) + (locals.var_pc__blk1412 * locals.var_pc__blk1412_dn7)) - (2.0 * ((locals.var_temp__blk949_dn7 * locals.var_qc__blk1413) + (locals.var_temp__blk949 * locals.var_qc__blk1413_dn7)))), (((locals.var_pc__blk1412_dn8 * locals.var_pc__blk1412) + (locals.var_pc__blk1412 * locals.var_pc__blk1412_dn8)) - (2.0 * ((locals.var_temp__blk949_dn8 * locals.var_qc__blk1413) + (locals.var_temp__blk949 * locals.var_qc__blk1413_dn8)))), (((locals.var_pc__blk1412_dn9 * locals.var_pc__blk1412) + (locals.var_pc__blk1412 * locals.var_pc__blk1412_dn9)) - (2.0 * ((locals.var_temp__blk949_dn9 * locals.var_qc__blk1413) + (locals.var_temp__blk949 * locals.var_qc__blk1413_dn9)))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1512 != 0.0)) {
            let assign52410_e67531: f64 = (locals.var_temp__blk949).sqrt();
            let assign52410_e67532: f64 = (locals.var_pc__blk1412 + assign52410_e67531);
            let assign52410_e67533: f64 = (locals.var_qc__blk1413 / assign52410_e67532);
            let assign52410_e67534: f64 = (2.0 * assign52410_e67533);
            (locals.var_x_ds__blk1411, locals.var_x_ds__blk1411_dn4, locals.var_x_ds__blk1411_dn6, locals.var_x_ds__blk1411_dn7, locals.var_x_ds__blk1411_dn8, locals.var_x_ds__blk1411_dn9, ) = (assign52410_e67534, (2.0 * (((locals.var_qc__blk1413_dn4 * assign52410_e67532) - (locals.var_qc__blk1413 * (locals.var_pc__blk1412_dn4 + (locals.var_temp__blk949_dn4 / (2.0 * assign52410_e67531))))) / (assign52410_e67532 * assign52410_e67532))), (2.0 * (((locals.var_qc__blk1413_dn6 * assign52410_e67532) - (locals.var_qc__blk1413 * (locals.var_pc__blk1412_dn6 + (locals.var_temp__blk949_dn6 / (2.0 * assign52410_e67531))))) / (assign52410_e67532 * assign52410_e67532))), (2.0 * (((locals.var_qc__blk1413_dn7 * assign52410_e67532) - (locals.var_qc__blk1413 * (locals.var_pc__blk1412_dn7 + (locals.var_temp__blk949_dn7 / (2.0 * assign52410_e67531))))) / (assign52410_e67532 * assign52410_e67532))), (2.0 * (((locals.var_qc__blk1413_dn8 * assign52410_e67532) - (locals.var_qc__blk1413 * (locals.var_pc__blk1412_dn8 + (locals.var_temp__blk949_dn8 / (2.0 * assign52410_e67531))))) / (assign52410_e67532 * assign52410_e67532))), (2.0 * (((locals.var_qc__blk1413_dn9 * assign52410_e67532) - (locals.var_qc__blk1413 * (locals.var_pc__blk1412_dn9 + (locals.var_temp__blk949_dn9 / (2.0 * assign52410_e67531))))) / (assign52410_e67532 * assign52410_e67532))), );
            locals.var_x_ds__blk1411_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1512 != 0.0)) {
            let assign52420_e67544: f64 = (locals.var_x_s__blk1363 + locals.var_x_ds__blk1411);
            (locals.var_x_d__blk1410, locals.var_x_d__blk1410_dn4, locals.var_x_d__blk1410_dn6, locals.var_x_d__blk1410_dn7, locals.var_x_d__blk1410_dn8, locals.var_x_d__blk1410_dn9, ) = (assign52420_e67544, (locals.var_x_s__blk1363_dn4 + locals.var_x_ds__blk1411_dn4), (locals.var_x_s__blk1363_dn6 + locals.var_x_ds__blk1411_dn6), (locals.var_x_s__blk1363_dn7 + locals.var_x_ds__blk1411_dn7), (locals.var_x_s__blk1363_dn8 + locals.var_x_ds__blk1411_dn8), (locals.var_x_s__blk1363_dn9 + locals.var_x_ds__blk1411_dn9), );
            locals.var_x_d__blk1410_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign52430_e67552: f64 = (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339);
            (locals.var_dps__blk1414, locals.var_dps__blk1414_dn4, locals.var_dps__blk1414_dn6, locals.var_dps__blk1414_dn7, locals.var_dps__blk1414_dn8, locals.var_dps__blk1414_dn9, ) = (assign52430_e67552, ((locals.var_x_ds__blk1411_dn4 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn4)), ((locals.var_x_ds__blk1411_dn6 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn6)), ((locals.var_x_ds__blk1411_dn7 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn7)), ((locals.var_x_ds__blk1411_dn8 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn8)), ((locals.var_x_ds__blk1411_dn9 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn9)), );
            locals.var_dps__blk1414_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign52440_e67560: f64 = (locals.var_x_d__blk1410 * locals.var_x_d__blk1410);
            let assign52440_e67564: f64 = (locals.var_x_d__blk1410 * locals.var_x_d__blk1410);
            let assign52440_e67565: f64 = (2.0 + assign52440_e67564);
            let assign52440_e67566: f64 = (assign52440_e67560 / assign52440_e67565);
            (locals.var_xi0d__blk1415, locals.var_xi0d__blk1415_dn4, locals.var_xi0d__blk1415_dn6, locals.var_xi0d__blk1415_dn7, locals.var_xi0d__blk1415_dn8, locals.var_xi0d__blk1415_dn9, ) = (assign52440_e67566, (((((locals.var_x_d__blk1410_dn4 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn4)) * assign52440_e67565) - (assign52440_e67560 * ((locals.var_x_d__blk1410_dn4 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn4)))) / (assign52440_e67565 * assign52440_e67565)), (((((locals.var_x_d__blk1410_dn6 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn6)) * assign52440_e67565) - (assign52440_e67560 * ((locals.var_x_d__blk1410_dn6 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn6)))) / (assign52440_e67565 * assign52440_e67565)), (((((locals.var_x_d__blk1410_dn7 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn7)) * assign52440_e67565) - (assign52440_e67560 * ((locals.var_x_d__blk1410_dn7 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn7)))) / (assign52440_e67565 * assign52440_e67565)), (((((locals.var_x_d__blk1410_dn8 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn8)) * assign52440_e67565) - (assign52440_e67560 * ((locals.var_x_d__blk1410_dn8 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn8)))) / (assign52440_e67565 * assign52440_e67565)), (((((locals.var_x_d__blk1410_dn9 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn9)) * assign52440_e67565) - (assign52440_e67560 * ((locals.var_x_d__blk1410_dn9 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn9)))) / (assign52440_e67565 * assign52440_e67565)), );
            locals.var_xi0d__blk1415_rv = 0.0;
        }

        let assign52450_e67571: f64 = if locals.var_x_d__blk1410 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1513 = assign52450_e67571;
        locals.var_guard1513_rv = 0.0;

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) {
            let assign52460_e67578: f64 = (-locals.var_x_d__blk1410);
            let assign52460_e67579: f64 = (assign52460_e67578).exp();
            (locals.var_ed__blk1416, locals.var_ed__blk1416_dn4, locals.var_ed__blk1416_dn6, locals.var_ed__blk1416_dn7, locals.var_ed__blk1416_dn8, locals.var_ed__blk1416_dn9, ) = (assign52460_e67579, (assign52460_e67579 * (-locals.var_x_d__blk1410_dn4)), (assign52460_e67579 * (-locals.var_x_d__blk1410_dn6)), (assign52460_e67579 * (-locals.var_x_d__blk1410_dn7)), (assign52460_e67579 * (-locals.var_x_d__blk1410_dn8)), (assign52460_e67579 * (-locals.var_x_d__blk1410_dn9)), );
            locals.var_ed__blk1416_rv = 0.0;
        }

        let assign52470_e67584: f64 = if locals.var_x_d__blk1410 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1514 = assign52470_e67584;
        locals.var_guard1514_rv = 0.0;

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 != 0.0)) {
            let assign52480_e67595: f64 = (locals.var_x_d__blk1410 * locals.var_x_d__blk1410);
            let assign52480_e67602: f64 = (0.25 * locals.var_x_d__blk1410);
            let assign52480_e67603: f64 = (1.0 - assign52480_e67602);
            let assign52480_e67604: f64 = (locals.var_x_d__blk1410 * assign52480_e67603);
            let assign52480_e67605: f64 = (0.3333333333333333 * assign52480_e67604);
            let assign52480_e67606: f64 = (1.0 - assign52480_e67605);
            let assign52480_e67607: f64 = (assign52480_e67595 * assign52480_e67606);
            let assign52480_e67608: f64 = (0.5 * assign52480_e67607);
            (locals.var_pd__blk1417, locals.var_pd__blk1417_dn4, locals.var_pd__blk1417_dn6, locals.var_pd__blk1417_dn7, locals.var_pd__blk1417_dn8, locals.var_pd__blk1417_dn9, ) = (assign52480_e67608, (0.5 * ((((locals.var_x_d__blk1410_dn4 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn4)) * assign52480_e67606) + (assign52480_e67595 * (-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn4 * assign52480_e67603) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn4))))))))), (0.5 * ((((locals.var_x_d__blk1410_dn6 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn6)) * assign52480_e67606) + (assign52480_e67595 * (-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn6 * assign52480_e67603) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn6))))))))), (0.5 * ((((locals.var_x_d__blk1410_dn7 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn7)) * assign52480_e67606) + (assign52480_e67595 * (-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn7 * assign52480_e67603) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn7))))))))), (0.5 * ((((locals.var_x_d__blk1410_dn8 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn8)) * assign52480_e67606) + (assign52480_e67595 * (-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn8 * assign52480_e67603) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn8))))))))), (0.5 * ((((locals.var_x_d__blk1410_dn9 * locals.var_x_d__blk1410) + (locals.var_x_d__blk1410 * locals.var_x_d__blk1410_dn9)) * assign52480_e67606) + (assign52480_e67595 * (-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn9 * assign52480_e67603) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn9))))))))), );
            locals.var_pd__blk1417_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 != 0.0)) {
            let assign52490_e67624: f64 = (0.25 * locals.var_x_d__blk1410);
            let assign52490_e67625: f64 = (1.0 - assign52490_e67624);
            let assign52490_e67626: f64 = (locals.var_x_d__blk1410 * assign52490_e67625);
            let assign52490_e67627: f64 = (0.3333333333333333 * assign52490_e67626);
            let assign52490_e67628: f64 = (1.0 - assign52490_e67627);
            let assign52490_e67629: f64 = (assign52490_e67628).sqrt();
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign52490_e67629, ((-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn4 * assign52490_e67625) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn4)))))) / (2.0 * assign52490_e67629)), ((-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn6 * assign52490_e67625) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn6)))))) / (2.0 * assign52490_e67629)), ((-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn7 * assign52490_e67625) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn7)))))) / (2.0 * assign52490_e67629)), ((-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn8 * assign52490_e67625) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn8)))))) / (2.0 * assign52490_e67629)), ((-(0.3333333333333333 * ((locals.var_x_d__blk1410_dn9 * assign52490_e67625) + (locals.var_x_d__blk1410 * (-(0.25 * locals.var_x_d__blk1410_dn9)))))) / (2.0 * assign52490_e67629)), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 != 0.0)) {
            let assign52500_e67642: f64 = (locals.var_x_d__blk1410 * locals.var_temp__blk949);
            let assign52500_e67643: f64 = (0.7071067811865475 * assign52500_e67642);
            (locals.var_sqd__blk1418, locals.var_sqd__blk1418_dn4, locals.var_sqd__blk1418_dn6, locals.var_sqd__blk1418_dn7, locals.var_sqd__blk1418_dn8, locals.var_sqd__blk1418_dn9, ) = (assign52500_e67643, (0.7071067811865475 * ((locals.var_x_d__blk1410_dn4 * locals.var_temp__blk949) + (locals.var_x_d__blk1410 * locals.var_temp__blk949_dn4))), (0.7071067811865475 * ((locals.var_x_d__blk1410_dn6 * locals.var_temp__blk949) + (locals.var_x_d__blk1410 * locals.var_temp__blk949_dn6))), (0.7071067811865475 * ((locals.var_x_d__blk1410_dn7 * locals.var_temp__blk949) + (locals.var_x_d__blk1410 * locals.var_temp__blk949_dn7))), (0.7071067811865475 * ((locals.var_x_d__blk1410_dn8 * locals.var_temp__blk949) + (locals.var_x_d__blk1410 * locals.var_temp__blk949_dn8))), (0.7071067811865475 * ((locals.var_x_d__blk1410_dn9 * locals.var_temp__blk949) + (locals.var_x_d__blk1410 * locals.var_temp__blk949_dn9))), );
            locals.var_sqd__blk1418_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 != 0.0)) {
            let assign52510_e67655: f64 = (0.16666666666666666 * locals.var_delta_nd__blk1409);
            let assign52510_e67657: f64 = (assign52510_e67655 * locals.var_x_d__blk1410);
            let assign52510_e67659: f64 = (assign52510_e67657 * locals.var_x_d__blk1410);
            let assign52510_e67661: f64 = (assign52510_e67659 * locals.var_x_d__blk1410);
            let assign52510_e67665: f64 = (1.75 * locals.var_x_d__blk1410);
            let assign52510_e67666: f64 = (1.0 + assign52510_e67665);
            let assign52510_e67667: f64 = (assign52510_e67661 * assign52510_e67666);
            (locals.var_dd__blk1419, locals.var_dd__blk1419_dn4, locals.var_dd__blk1419_dn6, locals.var_dd__blk1419_dn7, locals.var_dd__blk1419_dn8, locals.var_dd__blk1419_dn9, ) = (assign52510_e67667, (((((((((0.16666666666666666 * locals.var_delta_nd__blk1409_dn4) * locals.var_x_d__blk1410) + (assign52510_e67655 * locals.var_x_d__blk1410_dn4)) * locals.var_x_d__blk1410) + (assign52510_e67657 * locals.var_x_d__blk1410_dn4)) * locals.var_x_d__blk1410) + (assign52510_e67659 * locals.var_x_d__blk1410_dn4)) * assign52510_e67666) + (assign52510_e67661 * (1.75 * locals.var_x_d__blk1410_dn4))), (((((((((0.16666666666666666 * locals.var_delta_nd__blk1409_dn6) * locals.var_x_d__blk1410) + (assign52510_e67655 * locals.var_x_d__blk1410_dn6)) * locals.var_x_d__blk1410) + (assign52510_e67657 * locals.var_x_d__blk1410_dn6)) * locals.var_x_d__blk1410) + (assign52510_e67659 * locals.var_x_d__blk1410_dn6)) * assign52510_e67666) + (assign52510_e67661 * (1.75 * locals.var_x_d__blk1410_dn6))), (((((((((0.16666666666666666 * locals.var_delta_nd__blk1409_dn7) * locals.var_x_d__blk1410) + (assign52510_e67655 * locals.var_x_d__blk1410_dn7)) * locals.var_x_d__blk1410) + (assign52510_e67657 * locals.var_x_d__blk1410_dn7)) * locals.var_x_d__blk1410) + (assign52510_e67659 * locals.var_x_d__blk1410_dn7)) * assign52510_e67666) + (assign52510_e67661 * (1.75 * locals.var_x_d__blk1410_dn7))), (((((((((0.16666666666666666 * locals.var_delta_nd__blk1409_dn8) * locals.var_x_d__blk1410) + (assign52510_e67655 * locals.var_x_d__blk1410_dn8)) * locals.var_x_d__blk1410) + (assign52510_e67657 * locals.var_x_d__blk1410_dn8)) * locals.var_x_d__blk1410) + (assign52510_e67659 * locals.var_x_d__blk1410_dn8)) * assign52510_e67666) + (assign52510_e67661 * (1.75 * locals.var_x_d__blk1410_dn8))), (((((((((0.16666666666666666 * locals.var_delta_nd__blk1409_dn9) * locals.var_x_d__blk1410) + (assign52510_e67655 * locals.var_x_d__blk1410_dn9)) * locals.var_x_d__blk1410) + (assign52510_e67657 * locals.var_x_d__blk1410_dn9)) * locals.var_x_d__blk1410) + (assign52510_e67659 * locals.var_x_d__blk1410_dn9)) * assign52510_e67666) + (assign52510_e67661 * (1.75 * locals.var_x_d__blk1410_dn9))), );
            locals.var_dd__blk1419_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 == 0.0)) {
            let assign52520_e67680: f64 = (locals.var_x_d__blk1410 - 1.0);
            let assign52520_e67682: f64 = (assign52520_e67680 + locals.var_ed__blk1416);
            (locals.var_pd__blk1417, locals.var_pd__blk1417_dn4, locals.var_pd__blk1417_dn6, locals.var_pd__blk1417_dn7, locals.var_pd__blk1417_dn8, locals.var_pd__blk1417_dn9, ) = (assign52520_e67682, (locals.var_x_d__blk1410_dn4 + locals.var_ed__blk1416_dn4), (locals.var_x_d__blk1410_dn6 + locals.var_ed__blk1416_dn6), (locals.var_x_d__blk1410_dn7 + locals.var_ed__blk1416_dn7), (locals.var_x_d__blk1410_dn8 + locals.var_ed__blk1416_dn8), (locals.var_x_d__blk1410_dn9 + locals.var_ed__blk1416_dn9), );
            locals.var_pd__blk1417_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 == 0.0)) {
            let assign52530_e67694: f64 = (locals.var_pd__blk1417).sqrt();
            (locals.var_sqd__blk1418, locals.var_sqd__blk1418_dn4, locals.var_sqd__blk1418_dn6, locals.var_sqd__blk1418_dn7, locals.var_sqd__blk1418_dn8, locals.var_sqd__blk1418_dn9, ) = (assign52530_e67694, (locals.var_pd__blk1417_dn4 / (2.0 * assign52530_e67694)), (locals.var_pd__blk1417_dn6 / (2.0 * assign52530_e67694)), (locals.var_pd__blk1417_dn7 / (2.0 * assign52530_e67694)), (locals.var_pd__blk1417_dn8 / (2.0 * assign52530_e67694)), (locals.var_pd__blk1417_dn9 / (2.0 * assign52530_e67694)), );
            locals.var_sqd__blk1418_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 != 0.0)) && (locals.var_guard1514 == 0.0)) {
            let assign52540_e67708: f64 = (1.0 / locals.var_ed__blk1416);
            let assign52540_e67710: f64 = (assign52540_e67708 - locals.var_x_d__blk1410);
            let assign52540_e67712: f64 = (assign52540_e67710 - 1.0);
            let assign52540_e67714: f64 = (assign52540_e67712 - locals.var_xi0d__blk1415);
            let assign52540_e67715: f64 = (locals.var_delta_nd__blk1409 * assign52540_e67714);
            (locals.var_dd__blk1419, locals.var_dd__blk1419_dn4, locals.var_dd__blk1419_dn6, locals.var_dd__blk1419_dn7, locals.var_dd__blk1419_dn8, locals.var_dd__blk1419_dn9, ) = (assign52540_e67715, ((locals.var_delta_nd__blk1409_dn4 * assign52540_e67714) + (locals.var_delta_nd__blk1409 * (((-(locals.var_ed__blk1416_dn4 / (locals.var_ed__blk1416 * locals.var_ed__blk1416))) - locals.var_x_d__blk1410_dn4) - locals.var_xi0d__blk1415_dn4))), ((locals.var_delta_nd__blk1409_dn6 * assign52540_e67714) + (locals.var_delta_nd__blk1409 * (((-(locals.var_ed__blk1416_dn6 / (locals.var_ed__blk1416 * locals.var_ed__blk1416))) - locals.var_x_d__blk1410_dn6) - locals.var_xi0d__blk1415_dn6))), ((locals.var_delta_nd__blk1409_dn7 * assign52540_e67714) + (locals.var_delta_nd__blk1409 * (((-(locals.var_ed__blk1416_dn7 / (locals.var_ed__blk1416 * locals.var_ed__blk1416))) - locals.var_x_d__blk1410_dn7) - locals.var_xi0d__blk1415_dn7))), ((locals.var_delta_nd__blk1409_dn8 * assign52540_e67714) + (locals.var_delta_nd__blk1409 * (((-(locals.var_ed__blk1416_dn8 / (locals.var_ed__blk1416 * locals.var_ed__blk1416))) - locals.var_x_d__blk1410_dn8) - locals.var_xi0d__blk1415_dn8))), ((locals.var_delta_nd__blk1409_dn9 * assign52540_e67714) + (locals.var_delta_nd__blk1409 * (((-(locals.var_ed__blk1416_dn9 / (locals.var_ed__blk1416 * locals.var_ed__blk1416))) - locals.var_x_d__blk1410_dn9) - locals.var_xi0d__blk1415_dn9))), );
            locals.var_dd__blk1419_rv = 0.0;
        }

        let assign52550_e67721: f64 = (locals.var_xn_d__blk1407 - 230.25850929940458);
        let assign52550_e67722: f64 = if locals.var_x_d__blk1410 > assign52550_e67721 { 1.0 } else { 0.0 };
        locals.var_guard1515 = assign52550_e67722;
        locals.var_guard1515_rv = 0.0;

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 == 0.0)) && (locals.var_guard1515 != 0.0)) {
            let assign52560_e67733: f64 = (locals.var_x_d__blk1410 - locals.var_xn_d__blk1407);
            let assign52560_e67734: f64 = (assign52560_e67733).exp();
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign52560_e67734, (assign52560_e67734 * (locals.var_x_d__blk1410_dn4 - locals.var_xn_d__blk1407_dn4)), (assign52560_e67734 * (locals.var_x_d__blk1410_dn6 - locals.var_xn_d__blk1407_dn6)), (assign52560_e67734 * (locals.var_x_d__blk1410_dn7 - locals.var_xn_d__blk1407_dn7)), (assign52560_e67734 * (locals.var_x_d__blk1410_dn8 - locals.var_xn_d__blk1407_dn8)), (assign52560_e67734 * (locals.var_x_d__blk1410_dn9 - locals.var_xn_d__blk1407_dn9)), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 == 0.0)) && (locals.var_guard1515 != 0.0)) {
            let assign52570_e67747: f64 = (locals.var_delta_nd__blk1409 / locals.var_temp__blk949);
            (locals.var_ed__blk1416, locals.var_ed__blk1416_dn4, locals.var_ed__blk1416_dn6, locals.var_ed__blk1416_dn7, locals.var_ed__blk1416_dn8, locals.var_ed__blk1416_dn9, ) = (assign52570_e67747, (((locals.var_delta_nd__blk1409_dn4 * locals.var_temp__blk949) - (locals.var_delta_nd__blk1409 * locals.var_temp__blk949_dn4)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_delta_nd__blk1409_dn6 * locals.var_temp__blk949) - (locals.var_delta_nd__blk1409 * locals.var_temp__blk949_dn6)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_delta_nd__blk1409_dn7 * locals.var_temp__blk949) - (locals.var_delta_nd__blk1409 * locals.var_temp__blk949_dn7)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_delta_nd__blk1409_dn8 * locals.var_temp__blk949) - (locals.var_delta_nd__blk1409 * locals.var_temp__blk949_dn8)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_delta_nd__blk1409_dn9 * locals.var_temp__blk949) - (locals.var_delta_nd__blk1409 * locals.var_temp__blk949_dn9)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), );
            locals.var_ed__blk1416_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 == 0.0)) && (locals.var_guard1515 != 0.0)) {
            let assign52580_e67762: f64 = (locals.var_x_d__blk1410 + 1.0);
            let assign52580_e67764: f64 = (assign52580_e67762 + locals.var_xi0d__blk1415);
            let assign52580_e67765: f64 = (locals.var_delta_nd__blk1409 * assign52580_e67764);
            let assign52580_e67766: f64 = (locals.var_temp__blk949 - assign52580_e67765);
            (locals.var_dd__blk1419, locals.var_dd__blk1419_dn4, locals.var_dd__blk1419_dn6, locals.var_dd__blk1419_dn7, locals.var_dd__blk1419_dn8, locals.var_dd__blk1419_dn9, ) = (assign52580_e67766, (locals.var_temp__blk949_dn4 - ((locals.var_delta_nd__blk1409_dn4 * assign52580_e67764) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn4 + locals.var_xi0d__blk1415_dn4)))), (locals.var_temp__blk949_dn6 - ((locals.var_delta_nd__blk1409_dn6 * assign52580_e67764) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn6 + locals.var_xi0d__blk1415_dn6)))), (locals.var_temp__blk949_dn7 - ((locals.var_delta_nd__blk1409_dn7 * assign52580_e67764) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn7 + locals.var_xi0d__blk1415_dn7)))), (locals.var_temp__blk949_dn8 - ((locals.var_delta_nd__blk1409_dn8 * assign52580_e67764) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn8 + locals.var_xi0d__blk1415_dn8)))), (locals.var_temp__blk949_dn9 - ((locals.var_delta_nd__blk1409_dn9 * assign52580_e67764) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn9 + locals.var_xi0d__blk1415_dn9)))), );
            locals.var_dd__blk1419_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 == 0.0)) && (locals.var_guard1515 == 0.0)) {
            let assign52590_e67782: f64 = (locals.var_x_d__blk1410 - 230.25850929940458);
            let assign52590_e67787: f64 = (locals.var_x_d__blk1410 - 230.25850929940458);
            let assign52590_e67791: f64 = (locals.var_x_d__blk1410 - 230.25850929940458);
            let assign52590_e67793: f64 = (assign52590_e67791 * 0.3333333333333333);
            let assign52590_e67794: f64 = (1.0 + assign52590_e67793);
            let assign52590_e67795: f64 = (assign52590_e67787 * assign52590_e67794);
            let assign52590_e67796: f64 = (0.5 * assign52590_e67795);
            let assign52590_e67797: f64 = (1.0 + assign52590_e67796);
            let assign52590_e67798: f64 = (assign52590_e67782 * assign52590_e67797);
            let assign52590_e67799: f64 = (1.0 + assign52590_e67798);
            let assign52590_e67800: f64 = (1e-100 / assign52590_e67799);
            (locals.var_ed__blk1416, locals.var_ed__blk1416_dn4, locals.var_ed__blk1416_dn6, locals.var_ed__blk1416_dn7, locals.var_ed__blk1416_dn8, locals.var_ed__blk1416_dn9, ) = (assign52590_e67800, (-((1e-100 * ((locals.var_x_d__blk1410_dn4 * assign52590_e67797) + (assign52590_e67782 * (0.5 * ((locals.var_x_d__blk1410_dn4 * assign52590_e67794) + (assign52590_e67787 * (locals.var_x_d__blk1410_dn4 * 0.3333333333333333))))))) / (assign52590_e67799 * assign52590_e67799))), (-((1e-100 * ((locals.var_x_d__blk1410_dn6 * assign52590_e67797) + (assign52590_e67782 * (0.5 * ((locals.var_x_d__blk1410_dn6 * assign52590_e67794) + (assign52590_e67787 * (locals.var_x_d__blk1410_dn6 * 0.3333333333333333))))))) / (assign52590_e67799 * assign52590_e67799))), (-((1e-100 * ((locals.var_x_d__blk1410_dn7 * assign52590_e67797) + (assign52590_e67782 * (0.5 * ((locals.var_x_d__blk1410_dn7 * assign52590_e67794) + (assign52590_e67787 * (locals.var_x_d__blk1410_dn7 * 0.3333333333333333))))))) / (assign52590_e67799 * assign52590_e67799))), (-((1e-100 * ((locals.var_x_d__blk1410_dn8 * assign52590_e67797) + (assign52590_e67782 * (0.5 * ((locals.var_x_d__blk1410_dn8 * assign52590_e67794) + (assign52590_e67787 * (locals.var_x_d__blk1410_dn8 * 0.3333333333333333))))))) / (assign52590_e67799 * assign52590_e67799))), (-((1e-100 * ((locals.var_x_d__blk1410_dn9 * assign52590_e67797) + (assign52590_e67782 * (0.5 * ((locals.var_x_d__blk1410_dn9 * assign52590_e67794) + (assign52590_e67787 * (locals.var_x_d__blk1410_dn9 * 0.3333333333333333))))))) / (assign52590_e67799 * assign52590_e67799))), );
            locals.var_ed__blk1416_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_28(
        locals: &mut StampLocals,
    ) {
        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 == 0.0)) && (locals.var_guard1515 == 0.0)) {
            let assign52600_e67816: f64 = (locals.var_xn_d__blk1407 - locals.var_x_d__blk1410);
            let assign52600_e67818: f64 = (assign52600_e67816 - 230.25850929940458);
            let assign52600_e67823: f64 = (locals.var_xn_d__blk1407 - locals.var_x_d__blk1410);
            let assign52600_e67825: f64 = (assign52600_e67823 - 230.25850929940458);
            let assign52600_e67829: f64 = (locals.var_xn_d__blk1407 - locals.var_x_d__blk1410);
            let assign52600_e67831: f64 = (assign52600_e67829 - 230.25850929940458);
            let assign52600_e67833: f64 = (assign52600_e67831 * 0.3333333333333333);
            let assign52600_e67834: f64 = (1.0 + assign52600_e67833);
            let assign52600_e67835: f64 = (assign52600_e67825 * assign52600_e67834);
            let assign52600_e67836: f64 = (0.5 * assign52600_e67835);
            let assign52600_e67837: f64 = (1.0 + assign52600_e67836);
            let assign52600_e67838: f64 = (assign52600_e67818 * assign52600_e67837);
            let assign52600_e67839: f64 = (1.0 + assign52600_e67838);
            let assign52600_e67840: f64 = (1e-100 / assign52600_e67839);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign52600_e67840, (-((1e-100 * (((locals.var_xn_d__blk1407_dn4 - locals.var_x_d__blk1410_dn4) * assign52600_e67837) + (assign52600_e67818 * (0.5 * (((locals.var_xn_d__blk1407_dn4 - locals.var_x_d__blk1410_dn4) * assign52600_e67834) + (assign52600_e67825 * ((locals.var_xn_d__blk1407_dn4 - locals.var_x_d__blk1410_dn4) * 0.3333333333333333))))))) / (assign52600_e67839 * assign52600_e67839))), (-((1e-100 * (((locals.var_xn_d__blk1407_dn6 - locals.var_x_d__blk1410_dn6) * assign52600_e67837) + (assign52600_e67818 * (0.5 * (((locals.var_xn_d__blk1407_dn6 - locals.var_x_d__blk1410_dn6) * assign52600_e67834) + (assign52600_e67825 * ((locals.var_xn_d__blk1407_dn6 - locals.var_x_d__blk1410_dn6) * 0.3333333333333333))))))) / (assign52600_e67839 * assign52600_e67839))), (-((1e-100 * (((locals.var_xn_d__blk1407_dn7 - locals.var_x_d__blk1410_dn7) * assign52600_e67837) + (assign52600_e67818 * (0.5 * (((locals.var_xn_d__blk1407_dn7 - locals.var_x_d__blk1410_dn7) * assign52600_e67834) + (assign52600_e67825 * ((locals.var_xn_d__blk1407_dn7 - locals.var_x_d__blk1410_dn7) * 0.3333333333333333))))))) / (assign52600_e67839 * assign52600_e67839))), (-((1e-100 * (((locals.var_xn_d__blk1407_dn8 - locals.var_x_d__blk1410_dn8) * assign52600_e67837) + (assign52600_e67818 * (0.5 * (((locals.var_xn_d__blk1407_dn8 - locals.var_x_d__blk1410_dn8) * assign52600_e67834) + (assign52600_e67825 * ((locals.var_xn_d__blk1407_dn8 - locals.var_x_d__blk1410_dn8) * 0.3333333333333333))))))) / (assign52600_e67839 * assign52600_e67839))), (-((1e-100 * (((locals.var_xn_d__blk1407_dn9 - locals.var_x_d__blk1410_dn9) * assign52600_e67837) + (assign52600_e67818 * (0.5 * (((locals.var_xn_d__blk1407_dn9 - locals.var_x_d__blk1410_dn9) * assign52600_e67834) + (assign52600_e67825 * ((locals.var_xn_d__blk1407_dn9 - locals.var_x_d__blk1410_dn9) * 0.3333333333333333))))))) / (assign52600_e67839 * assign52600_e67839))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 == 0.0)) && (locals.var_guard1515 == 0.0)) {
            let assign52610_e67856: f64 = (locals.var_x_d__blk1410 + 1.0);
            let assign52610_e67858: f64 = (assign52610_e67856 + locals.var_xi0d__blk1415);
            let assign52610_e67859: f64 = (locals.var_delta_nd__blk1409 * assign52610_e67858);
            let assign52610_e67860: f64 = (locals.var_temp__blk949 - assign52610_e67859);
            (locals.var_dd__blk1419, locals.var_dd__blk1419_dn4, locals.var_dd__blk1419_dn6, locals.var_dd__blk1419_dn7, locals.var_dd__blk1419_dn8, locals.var_dd__blk1419_dn9, ) = (assign52610_e67860, (locals.var_temp__blk949_dn4 - ((locals.var_delta_nd__blk1409_dn4 * assign52610_e67858) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn4 + locals.var_xi0d__blk1415_dn4)))), (locals.var_temp__blk949_dn6 - ((locals.var_delta_nd__blk1409_dn6 * assign52610_e67858) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn6 + locals.var_xi0d__blk1415_dn6)))), (locals.var_temp__blk949_dn7 - ((locals.var_delta_nd__blk1409_dn7 * assign52610_e67858) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn7 + locals.var_xi0d__blk1415_dn7)))), (locals.var_temp__blk949_dn8 - ((locals.var_delta_nd__blk1409_dn8 * assign52610_e67858) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn8 + locals.var_xi0d__blk1415_dn8)))), (locals.var_temp__blk949_dn9 - ((locals.var_delta_nd__blk1409_dn9 * assign52610_e67858) + (locals.var_delta_nd__blk1409 * (locals.var_x_d__blk1410_dn9 + locals.var_xi0d__blk1415_dn9)))), );
            locals.var_dd__blk1419_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 == 0.0)) {
            let assign52620_e67871: f64 = (locals.var_x_d__blk1410 - 1.0);
            let assign52620_e67873: f64 = (assign52620_e67871 + locals.var_ed__blk1416);
            (locals.var_pd__blk1417, locals.var_pd__blk1417_dn4, locals.var_pd__blk1417_dn6, locals.var_pd__blk1417_dn7, locals.var_pd__blk1417_dn8, locals.var_pd__blk1417_dn9, ) = (assign52620_e67873, (locals.var_x_d__blk1410_dn4 + locals.var_ed__blk1416_dn4), (locals.var_x_d__blk1410_dn6 + locals.var_ed__blk1416_dn6), (locals.var_x_d__blk1410_dn7 + locals.var_ed__blk1416_dn7), (locals.var_x_d__blk1410_dn8 + locals.var_ed__blk1416_dn8), (locals.var_x_d__blk1410_dn9 + locals.var_ed__blk1416_dn9), );
            locals.var_pd__blk1417_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1513 == 0.0)) {
            let assign52630_e67883: f64 = (locals.var_pd__blk1417).sqrt();
            (locals.var_sqd__blk1418, locals.var_sqd__blk1418_dn4, locals.var_sqd__blk1418_dn6, locals.var_sqd__blk1418_dn7, locals.var_sqd__blk1418_dn8, locals.var_sqd__blk1418_dn9, ) = (assign52630_e67883, (locals.var_pd__blk1417_dn4 / (2.0 * assign52630_e67883)), (locals.var_pd__blk1417_dn6 / (2.0 * assign52630_e67883)), (locals.var_pd__blk1417_dn7 / (2.0 * assign52630_e67883)), (locals.var_pd__blk1417_dn8 / (2.0 * assign52630_e67883)), (locals.var_pd__blk1417_dn9 / (2.0 * assign52630_e67883)), );
            locals.var_sqd__blk1418_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign52640_e67891: f64 = (locals.var_sqd__blk1418 * locals.var_gf__blk1324);
            let assign52640_e67893: f64 = (assign52640_e67891 * locals.var_phit1__blk1339);
            (locals.var_qbd__blk1420, locals.var_qbd__blk1420_dn4, locals.var_qbd__blk1420_dn6, locals.var_qbd__blk1420_dn7, locals.var_qbd__blk1420_dn8, locals.var_qbd__blk1420_dn9, ) = (assign52640_e67893, ((((locals.var_sqd__blk1418_dn4 * locals.var_gf__blk1324) + (locals.var_sqd__blk1418 * locals.var_gf__blk1324_dn4)) * locals.var_phit1__blk1339) + (assign52640_e67891 * locals.var_phit1__blk1339_dn4)), ((((locals.var_sqd__blk1418_dn6 * locals.var_gf__blk1324) + (locals.var_sqd__blk1418 * locals.var_gf__blk1324_dn6)) * locals.var_phit1__blk1339) + (assign52640_e67891 * locals.var_phit1__blk1339_dn6)), ((((locals.var_sqd__blk1418_dn7 * locals.var_gf__blk1324) + (locals.var_sqd__blk1418 * locals.var_gf__blk1324_dn7)) * locals.var_phit1__blk1339) + (assign52640_e67891 * locals.var_phit1__blk1339_dn7)), ((((locals.var_sqd__blk1418_dn8 * locals.var_gf__blk1324) + (locals.var_sqd__blk1418 * locals.var_gf__blk1324_dn8)) * locals.var_phit1__blk1339) + (assign52640_e67891 * locals.var_phit1__blk1339_dn8)), ((((locals.var_sqd__blk1418_dn9 * locals.var_gf__blk1324) + (locals.var_sqd__blk1418 * locals.var_gf__blk1324_dn9)) * locals.var_phit1__blk1339) + (assign52640_e67891 * locals.var_phit1__blk1339_dn9)), );
            locals.var_qbd__blk1420_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign52650_e67902: f64 = (locals.var_x_s__blk1363 + locals.var_x_d__blk1410);
            let assign52650_e67903: f64 = (0.5 * assign52650_e67902);
            (locals.var_x_m__blk1421, locals.var_x_m__blk1421_dn4, locals.var_x_m__blk1421_dn6, locals.var_x_m__blk1421_dn7, locals.var_x_m__blk1421_dn8, locals.var_x_m__blk1421_dn9, ) = (assign52650_e67903, (0.5 * (locals.var_x_s__blk1363_dn4 + locals.var_x_d__blk1410_dn4)), (0.5 * (locals.var_x_s__blk1363_dn6 + locals.var_x_d__blk1410_dn6)), (0.5 * (locals.var_x_s__blk1363_dn7 + locals.var_x_d__blk1410_dn7)), (0.5 * (locals.var_x_s__blk1363_dn8 + locals.var_x_d__blk1410_dn8)), (0.5 * (locals.var_x_s__blk1363_dn9 + locals.var_x_d__blk1410_dn9)), );
            locals.var_x_m__blk1421_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            (locals.var_em__blk1422, locals.var_em__blk1422_dn4, locals.var_em__blk1422_dn6, locals.var_em__blk1422_dn7, locals.var_em__blk1422_dn8, locals.var_em__blk1422_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_em__blk1422_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign52670_e67917: f64 = (locals.var_ed__blk1416 * locals.var_es__blk1369);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign52670_e67917, ((locals.var_ed__blk1416_dn4 * locals.var_es__blk1369) + (locals.var_ed__blk1416 * locals.var_es__blk1369_dn4)), ((locals.var_ed__blk1416_dn6 * locals.var_es__blk1369) + (locals.var_ed__blk1416 * locals.var_es__blk1369_dn6)), ((locals.var_ed__blk1416_dn7 * locals.var_es__blk1369) + (locals.var_ed__blk1416 * locals.var_es__blk1369_dn7)), ((locals.var_ed__blk1416_dn8 * locals.var_es__blk1369) + (locals.var_ed__blk1416 * locals.var_es__blk1369_dn8)), ((locals.var_ed__blk1416_dn9 * locals.var_es__blk1369) + (locals.var_ed__blk1416 * locals.var_es__blk1369_dn9)), );
            locals.var_temp__blk949_rv = 0.0;
        }

        let assign52680_e67922: f64 = if locals.var_temp__blk949 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1516 = assign52680_e67922;
        locals.var_guard1516_rv = 0.0;

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1516 != 0.0)) {
            let assign52690_e67929: f64 = (locals.var_temp__blk949).sqrt();
            (locals.var_em__blk1422, locals.var_em__blk1422_dn4, locals.var_em__blk1422_dn6, locals.var_em__blk1422_dn7, locals.var_em__blk1422_dn8, locals.var_em__blk1422_dn9, ) = (assign52690_e67929, (locals.var_temp__blk949_dn4 / (2.0 * assign52690_e67929)), (locals.var_temp__blk949_dn6 / (2.0 * assign52690_e67929)), (locals.var_temp__blk949_dn7 / (2.0 * assign52690_e67929)), (locals.var_temp__blk949_dn8 / (2.0 * assign52690_e67929)), (locals.var_temp__blk949_dn9 / (2.0 * assign52690_e67929)), );
            locals.var_em__blk1422_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign52700_e67938: f64 = (locals.var_ds__blk1370 + locals.var_dd__blk1419);
            let assign52700_e67939: f64 = (0.5 * assign52700_e67938);
            (locals.var_d_bar__blk1423, locals.var_d_bar__blk1423_dn4, locals.var_d_bar__blk1423_dn6, locals.var_d_bar__blk1423_dn7, locals.var_d_bar__blk1423_dn8, locals.var_d_bar__blk1423_dn9, ) = (assign52700_e67939, (0.5 * (locals.var_ds__blk1370_dn4 + locals.var_dd__blk1419_dn4)), (0.5 * (locals.var_ds__blk1370_dn6 + locals.var_dd__blk1419_dn6)), (0.5 * (locals.var_ds__blk1370_dn7 + locals.var_dd__blk1419_dn7)), (0.5 * (locals.var_ds__blk1370_dn8 + locals.var_dd__blk1419_dn8)), (0.5 * (locals.var_ds__blk1370_dn9 + locals.var_dd__blk1419_dn9)), );
            locals.var_d_bar__blk1423_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign52710_e67949: f64 = (locals.var_x_ds__blk1411 * locals.var_x_ds__blk1411);
            let assign52710_e67953: f64 = (2.0 * locals.var_inv_gf2__blk1341);
            let assign52710_e67954: f64 = (locals.var_em__blk1422 - assign52710_e67953);
            let assign52710_e67955: f64 = (assign52710_e67949 * assign52710_e67954);
            let assign52710_e67956: f64 = (0.125 * assign52710_e67955);
            let assign52710_e67957: f64 = (locals.var_d_bar__blk1423 + assign52710_e67956);
            (locals.var_dm__blk1424, locals.var_dm__blk1424_dn4, locals.var_dm__blk1424_dn6, locals.var_dm__blk1424_dn7, locals.var_dm__blk1424_dn8, locals.var_dm__blk1424_dn9, ) = (assign52710_e67957, (locals.var_d_bar__blk1423_dn4 + (0.125 * ((((locals.var_x_ds__blk1411_dn4 * locals.var_x_ds__blk1411) + (locals.var_x_ds__blk1411 * locals.var_x_ds__blk1411_dn4)) * assign52710_e67954) + (assign52710_e67949 * (locals.var_em__blk1422_dn4 - (2.0 * locals.var_inv_gf2__blk1341_dn4)))))), (locals.var_d_bar__blk1423_dn6 + (0.125 * ((((locals.var_x_ds__blk1411_dn6 * locals.var_x_ds__blk1411) + (locals.var_x_ds__blk1411 * locals.var_x_ds__blk1411_dn6)) * assign52710_e67954) + (assign52710_e67949 * (locals.var_em__blk1422_dn6 - (2.0 * locals.var_inv_gf2__blk1341_dn6)))))), (locals.var_d_bar__blk1423_dn7 + (0.125 * ((((locals.var_x_ds__blk1411_dn7 * locals.var_x_ds__blk1411) + (locals.var_x_ds__blk1411 * locals.var_x_ds__blk1411_dn7)) * assign52710_e67954) + (assign52710_e67949 * (locals.var_em__blk1422_dn7 - (2.0 * locals.var_inv_gf2__blk1341_dn7)))))), (locals.var_d_bar__blk1423_dn8 + (0.125 * ((((locals.var_x_ds__blk1411_dn8 * locals.var_x_ds__blk1411) + (locals.var_x_ds__blk1411 * locals.var_x_ds__blk1411_dn8)) * assign52710_e67954) + (assign52710_e67949 * (locals.var_em__blk1422_dn8 - (2.0 * locals.var_inv_gf2__blk1341_dn8)))))), (locals.var_d_bar__blk1423_dn9 + (0.125 * ((((locals.var_x_ds__blk1411_dn9 * locals.var_x_ds__blk1411) + (locals.var_x_ds__blk1411 * locals.var_x_ds__blk1411_dn9)) * assign52710_e67954) + (assign52710_e67949 * (locals.var_em__blk1422_dn9 - (2.0 * locals.var_inv_gf2__blk1341_dn9)))))), );
            locals.var_dm__blk1424_rv = 0.0;
        }

        let assign52720_e67962: f64 = if locals.var_x_m__blk1421 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1517 = assign52720_e67962;
        locals.var_guard1517_rv = 0.0;

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 != 0.0)) {
            let assign52730_e67971: f64 = (locals.var_x_m__blk1421 * locals.var_x_m__blk1421);
            let assign52730_e67978: f64 = (0.25 * locals.var_x_m__blk1421);
            let assign52730_e67979: f64 = (1.0 - assign52730_e67978);
            let assign52730_e67980: f64 = (locals.var_x_m__blk1421 * assign52730_e67979);
            let assign52730_e67981: f64 = (0.3333333333333333 * assign52730_e67980);
            let assign52730_e67982: f64 = (1.0 - assign52730_e67981);
            let assign52730_e67983: f64 = (assign52730_e67971 * assign52730_e67982);
            let assign52730_e67984: f64 = (0.5 * assign52730_e67983);
            (locals.var_pm__blk1425, locals.var_pm__blk1425_dn4, locals.var_pm__blk1425_dn6, locals.var_pm__blk1425_dn7, locals.var_pm__blk1425_dn8, locals.var_pm__blk1425_dn9, ) = (assign52730_e67984, (0.5 * ((((locals.var_x_m__blk1421_dn4 * locals.var_x_m__blk1421) + (locals.var_x_m__blk1421 * locals.var_x_m__blk1421_dn4)) * assign52730_e67982) + (assign52730_e67971 * (-(0.3333333333333333 * ((locals.var_x_m__blk1421_dn4 * assign52730_e67979) + (locals.var_x_m__blk1421 * (-(0.25 * locals.var_x_m__blk1421_dn4))))))))), (0.5 * ((((locals.var_x_m__blk1421_dn6 * locals.var_x_m__blk1421) + (locals.var_x_m__blk1421 * locals.var_x_m__blk1421_dn6)) * assign52730_e67982) + (assign52730_e67971 * (-(0.3333333333333333 * ((locals.var_x_m__blk1421_dn6 * assign52730_e67979) + (locals.var_x_m__blk1421 * (-(0.25 * locals.var_x_m__blk1421_dn6))))))))), (0.5 * ((((locals.var_x_m__blk1421_dn7 * locals.var_x_m__blk1421) + (locals.var_x_m__blk1421 * locals.var_x_m__blk1421_dn7)) * assign52730_e67982) + (assign52730_e67971 * (-(0.3333333333333333 * ((locals.var_x_m__blk1421_dn7 * assign52730_e67979) + (locals.var_x_m__blk1421 * (-(0.25 * locals.var_x_m__blk1421_dn7))))))))), (0.5 * ((((locals.var_x_m__blk1421_dn8 * locals.var_x_m__blk1421) + (locals.var_x_m__blk1421 * locals.var_x_m__blk1421_dn8)) * assign52730_e67982) + (assign52730_e67971 * (-(0.3333333333333333 * ((locals.var_x_m__blk1421_dn8 * assign52730_e67979) + (locals.var_x_m__blk1421 * (-(0.25 * locals.var_x_m__blk1421_dn8))))))))), (0.5 * ((((locals.var_x_m__blk1421_dn9 * locals.var_x_m__blk1421) + (locals.var_x_m__blk1421 * locals.var_x_m__blk1421_dn9)) * assign52730_e67982) + (assign52730_e67971 * (-(0.3333333333333333 * ((locals.var_x_m__blk1421_dn9 * assign52730_e67979) + (locals.var_x_m__blk1421 * (-(0.25 * locals.var_x_m__blk1421_dn9))))))))), );
            locals.var_pm__blk1425_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 != 0.0)) {
            let assign52740_e67995: f64 = (locals.var_dm__blk1424 + locals.var_pm__blk1425);
            let assign52740_e67996: f64 = (assign52740_e67995).sqrt();
            let assign52740_e67997: f64 = (locals.var_gf__blk1324 * assign52740_e67996);
            (locals.var_xgm__blk1426, locals.var_xgm__blk1426_dn4, locals.var_xgm__blk1426_dn6, locals.var_xgm__blk1426_dn7, locals.var_xgm__blk1426_dn8, locals.var_xgm__blk1426_dn9, ) = (assign52740_e67997, ((locals.var_gf__blk1324_dn4 * assign52740_e67996) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn4 + locals.var_pm__blk1425_dn4) / (2.0 * assign52740_e67996)))), ((locals.var_gf__blk1324_dn6 * assign52740_e67996) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn6 + locals.var_pm__blk1425_dn6) / (2.0 * assign52740_e67996)))), ((locals.var_gf__blk1324_dn7 * assign52740_e67996) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn7 + locals.var_pm__blk1425_dn7) / (2.0 * assign52740_e67996)))), ((locals.var_gf__blk1324_dn8 * assign52740_e67996) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn8 + locals.var_pm__blk1425_dn8) / (2.0 * assign52740_e67996)))), ((locals.var_gf__blk1324_dn9 * assign52740_e67996) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn9 + locals.var_pm__blk1425_dn9) / (2.0 * assign52740_e67996)))), );
            locals.var_xgm__blk1426_rv = 0.0;
        }

        let assign52750_e68002: f64 = if locals.var_kp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1518 = assign52750_e68002;
        locals.var_guard1518_rv = 0.0;

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 != 0.0)) && (locals.var_guard1518 != 0.0)) {
            let assign52760_e68014: f64 = (locals.var_kp * locals.var_xgm__blk1426);
            let assign52760_e68015: f64 = (1.0 + assign52760_e68014);
            let assign52760_e68016: f64 = (assign52760_e68015).sqrt();
            let assign52760_e68017: f64 = (1.0 / assign52760_e68016);
            (locals.var_eta_p__blk1427, locals.var_eta_p__blk1427_dn4, locals.var_eta_p__blk1427_dn6, locals.var_eta_p__blk1427_dn7, locals.var_eta_p__blk1427_dn8, locals.var_eta_p__blk1427_dn9, ) = (assign52760_e68017, (-((((locals.var_kp_dn4 * locals.var_xgm__blk1426) + (locals.var_kp * locals.var_xgm__blk1426_dn4)) / (2.0 * assign52760_e68016)) / (assign52760_e68016 * assign52760_e68016))), (-(((locals.var_kp * locals.var_xgm__blk1426_dn6) / (2.0 * assign52760_e68016)) / (assign52760_e68016 * assign52760_e68016))), (-(((locals.var_kp * locals.var_xgm__blk1426_dn7) / (2.0 * assign52760_e68016)) / (assign52760_e68016 * assign52760_e68016))), (-(((locals.var_kp * locals.var_xgm__blk1426_dn8) / (2.0 * assign52760_e68016)) / (assign52760_e68016 * assign52760_e68016))), (-(((locals.var_kp * locals.var_xgm__blk1426_dn9) / (2.0 * assign52760_e68016)) / (assign52760_e68016 * assign52760_e68016))), );
            locals.var_eta_p__blk1427_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 != 0.0)) {
            let assign52770_e68031: f64 = (0.25 * locals.var_x_m__blk1421);
            let assign52770_e68032: f64 = (1.0 - assign52770_e68031);
            let assign52770_e68033: f64 = (locals.var_x_m__blk1421 * assign52770_e68032);
            let assign52770_e68034: f64 = (0.3333333333333333 * assign52770_e68033);
            let assign52770_e68035: f64 = (1.0 - assign52770_e68034);
            let assign52770_e68036: f64 = (assign52770_e68035).sqrt();
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign52770_e68036, ((-(0.3333333333333333 * ((locals.var_x_m__blk1421_dn4 * assign52770_e68032) + (locals.var_x_m__blk1421 * (-(0.25 * locals.var_x_m__blk1421_dn4)))))) / (2.0 * assign52770_e68036)), ((-(0.3333333333333333 * ((locals.var_x_m__blk1421_dn6 * assign52770_e68032) + (locals.var_x_m__blk1421 * (-(0.25 * locals.var_x_m__blk1421_dn6)))))) / (2.0 * assign52770_e68036)), ((-(0.3333333333333333 * ((locals.var_x_m__blk1421_dn7 * assign52770_e68032) + (locals.var_x_m__blk1421 * (-(0.25 * locals.var_x_m__blk1421_dn7)))))) / (2.0 * assign52770_e68036)), ((-(0.3333333333333333 * ((locals.var_x_m__blk1421_dn8 * assign52770_e68032) + (locals.var_x_m__blk1421 * (-(0.25 * locals.var_x_m__blk1421_dn8)))))) / (2.0 * assign52770_e68036)), ((-(0.3333333333333333 * ((locals.var_x_m__blk1421_dn9 * assign52770_e68032) + (locals.var_x_m__blk1421 * (-(0.25 * locals.var_x_m__blk1421_dn9)))))) / (2.0 * assign52770_e68036)), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 != 0.0)) {
            let assign52780_e68047: f64 = (locals.var_x_m__blk1421 * locals.var_temp__blk949);
            let assign52780_e68048: f64 = (0.7071067811865475 * assign52780_e68047);
            (locals.var_sqm__blk1428, locals.var_sqm__blk1428_dn4, locals.var_sqm__blk1428_dn6, locals.var_sqm__blk1428_dn7, locals.var_sqm__blk1428_dn8, locals.var_sqm__blk1428_dn9, ) = (assign52780_e68048, (0.7071067811865475 * ((locals.var_x_m__blk1421_dn4 * locals.var_temp__blk949) + (locals.var_x_m__blk1421 * locals.var_temp__blk949_dn4))), (0.7071067811865475 * ((locals.var_x_m__blk1421_dn6 * locals.var_temp__blk949) + (locals.var_x_m__blk1421 * locals.var_temp__blk949_dn6))), (0.7071067811865475 * ((locals.var_x_m__blk1421_dn7 * locals.var_temp__blk949) + (locals.var_x_m__blk1421 * locals.var_temp__blk949_dn7))), (0.7071067811865475 * ((locals.var_x_m__blk1421_dn8 * locals.var_temp__blk949) + (locals.var_x_m__blk1421 * locals.var_temp__blk949_dn8))), (0.7071067811865475 * ((locals.var_x_m__blk1421_dn9 * locals.var_temp__blk949) + (locals.var_x_m__blk1421 * locals.var_temp__blk949_dn9))), );
            locals.var_sqm__blk1428_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 != 0.0)) {
            let assign52790_e68062: f64 = (0.5 * locals.var_x_m__blk1421);
            let assign52790_e68063: f64 = (1.0 - assign52790_e68062);
            let assign52790_e68067: f64 = (locals.var_x_m__blk1421 * locals.var_x_m__blk1421);
            let assign52790_e68068: f64 = (0.16666666666666666 * assign52790_e68067);
            let assign52790_e68069: f64 = (assign52790_e68063 + assign52790_e68068);
            let assign52790_e68070: f64 = (locals.var_gf__blk1324 * assign52790_e68069);
            let assign52790_e68072: f64 = (assign52790_e68070 / locals.var_temp__blk949);
            let assign52790_e68073: f64 = (0.7071067811865475 * assign52790_e68072);
            let assign52790_e68074: f64 = (locals.var_eta_p__blk1427 + assign52790_e68073);
            (locals.var_alpha__blk1429, locals.var_alpha__blk1429_dn4, locals.var_alpha__blk1429_dn6, locals.var_alpha__blk1429_dn7, locals.var_alpha__blk1429_dn8, locals.var_alpha__blk1429_dn9, ) = (assign52790_e68074, (locals.var_eta_p__blk1427_dn4 + (0.7071067811865475 * (((((locals.var_gf__blk1324_dn4 * assign52790_e68069) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_m__blk1421_dn4)) + (0.16666666666666666 * ((locals.var_x_m__blk1421_dn4 * locals.var_x_m__blk1421) + (locals.var_x_m__blk1421 * locals.var_x_m__blk1421_dn4)))))) * locals.var_temp__blk949) - (assign52790_e68070 * locals.var_temp__blk949_dn4)) / (locals.var_temp__blk949 * locals.var_temp__blk949)))), (locals.var_eta_p__blk1427_dn6 + (0.7071067811865475 * (((((locals.var_gf__blk1324_dn6 * assign52790_e68069) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_m__blk1421_dn6)) + (0.16666666666666666 * ((locals.var_x_m__blk1421_dn6 * locals.var_x_m__blk1421) + (locals.var_x_m__blk1421 * locals.var_x_m__blk1421_dn6)))))) * locals.var_temp__blk949) - (assign52790_e68070 * locals.var_temp__blk949_dn6)) / (locals.var_temp__blk949 * locals.var_temp__blk949)))), (locals.var_eta_p__blk1427_dn7 + (0.7071067811865475 * (((((locals.var_gf__blk1324_dn7 * assign52790_e68069) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_m__blk1421_dn7)) + (0.16666666666666666 * ((locals.var_x_m__blk1421_dn7 * locals.var_x_m__blk1421) + (locals.var_x_m__blk1421 * locals.var_x_m__blk1421_dn7)))))) * locals.var_temp__blk949) - (assign52790_e68070 * locals.var_temp__blk949_dn7)) / (locals.var_temp__blk949 * locals.var_temp__blk949)))), (locals.var_eta_p__blk1427_dn8 + (0.7071067811865475 * (((((locals.var_gf__blk1324_dn8 * assign52790_e68069) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_m__blk1421_dn8)) + (0.16666666666666666 * ((locals.var_x_m__blk1421_dn8 * locals.var_x_m__blk1421) + (locals.var_x_m__blk1421 * locals.var_x_m__blk1421_dn8)))))) * locals.var_temp__blk949) - (assign52790_e68070 * locals.var_temp__blk949_dn8)) / (locals.var_temp__blk949 * locals.var_temp__blk949)))), (locals.var_eta_p__blk1427_dn9 + (0.7071067811865475 * (((((locals.var_gf__blk1324_dn9 * assign52790_e68069) + (locals.var_gf__blk1324 * ((-(0.5 * locals.var_x_m__blk1421_dn9)) + (0.16666666666666666 * ((locals.var_x_m__blk1421_dn9 * locals.var_x_m__blk1421) + (locals.var_x_m__blk1421 * locals.var_x_m__blk1421_dn9)))))) * locals.var_temp__blk949) - (assign52790_e68070 * locals.var_temp__blk949_dn9)) / (locals.var_temp__blk949 * locals.var_temp__blk949)))), );
            locals.var_alpha__blk1429_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) {
            let assign52800_e68085: f64 = (locals.var_x_m__blk1421 - 1.0);
            let assign52800_e68087: f64 = (assign52800_e68085 + locals.var_em__blk1422);
            (locals.var_pm__blk1425, locals.var_pm__blk1425_dn4, locals.var_pm__blk1425_dn6, locals.var_pm__blk1425_dn7, locals.var_pm__blk1425_dn8, locals.var_pm__blk1425_dn9, ) = (assign52800_e68087, (locals.var_x_m__blk1421_dn4 + locals.var_em__blk1422_dn4), (locals.var_x_m__blk1421_dn6 + locals.var_em__blk1422_dn6), (locals.var_x_m__blk1421_dn7 + locals.var_em__blk1422_dn7), (locals.var_x_m__blk1421_dn8 + locals.var_em__blk1422_dn8), (locals.var_x_m__blk1421_dn9 + locals.var_em__blk1422_dn9), );
            locals.var_pm__blk1425_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) {
            let assign52810_e68099: f64 = (locals.var_dm__blk1424 + locals.var_pm__blk1425);
            let assign52810_e68100: f64 = (assign52810_e68099).sqrt();
            let assign52810_e68101: f64 = (locals.var_gf__blk1324 * assign52810_e68100);
            (locals.var_xgm__blk1426, locals.var_xgm__blk1426_dn4, locals.var_xgm__blk1426_dn6, locals.var_xgm__blk1426_dn7, locals.var_xgm__blk1426_dn8, locals.var_xgm__blk1426_dn9, ) = (assign52810_e68101, ((locals.var_gf__blk1324_dn4 * assign52810_e68100) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn4 + locals.var_pm__blk1425_dn4) / (2.0 * assign52810_e68100)))), ((locals.var_gf__blk1324_dn6 * assign52810_e68100) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn6 + locals.var_pm__blk1425_dn6) / (2.0 * assign52810_e68100)))), ((locals.var_gf__blk1324_dn7 * assign52810_e68100) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn7 + locals.var_pm__blk1425_dn7) / (2.0 * assign52810_e68100)))), ((locals.var_gf__blk1324_dn8 * assign52810_e68100) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn8 + locals.var_pm__blk1425_dn8) / (2.0 * assign52810_e68100)))), ((locals.var_gf__blk1324_dn9 * assign52810_e68100) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn9 + locals.var_pm__blk1425_dn9) / (2.0 * assign52810_e68100)))), );
            locals.var_xgm__blk1426_rv = 0.0;
        }

        let assign52820_e68106: f64 = if locals.var_kp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1519 = assign52820_e68106;
        locals.var_guard1519_rv = 0.0;

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
            let assign52830_e68117: f64 = (1.0 - locals.var_em__blk1422);
            let assign52830_e68121: f64 = (locals.var_xgm__blk1426 * locals.var_inv_gf2__blk1341);
            let assign52830_e68122: f64 = (2.0 * assign52830_e68121);
            let assign52830_e68123: f64 = (assign52830_e68117 + assign52830_e68122);
            (locals.var_d0__blk1430, locals.var_d0__blk1430_dn4, locals.var_d0__blk1430_dn6, locals.var_d0__blk1430_dn7, locals.var_d0__blk1430_dn8, locals.var_d0__blk1430_dn9, ) = (assign52830_e68123, ((-locals.var_em__blk1422_dn4) + (2.0 * ((locals.var_xgm__blk1426_dn4 * locals.var_inv_gf2__blk1341) + (locals.var_xgm__blk1426 * locals.var_inv_gf2__blk1341_dn4)))), ((-locals.var_em__blk1422_dn6) + (2.0 * ((locals.var_xgm__blk1426_dn6 * locals.var_inv_gf2__blk1341) + (locals.var_xgm__blk1426 * locals.var_inv_gf2__blk1341_dn6)))), ((-locals.var_em__blk1422_dn7) + (2.0 * ((locals.var_xgm__blk1426_dn7 * locals.var_inv_gf2__blk1341) + (locals.var_xgm__blk1426 * locals.var_inv_gf2__blk1341_dn7)))), ((-locals.var_em__blk1422_dn8) + (2.0 * ((locals.var_xgm__blk1426_dn8 * locals.var_inv_gf2__blk1341) + (locals.var_xgm__blk1426 * locals.var_inv_gf2__blk1341_dn8)))), ((-locals.var_em__blk1422_dn9) + (2.0 * ((locals.var_xgm__blk1426_dn9 * locals.var_inv_gf2__blk1341) + (locals.var_xgm__blk1426 * locals.var_inv_gf2__blk1341_dn9)))), );
            locals.var_d0__blk1430_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
            let assign52840_e68138: f64 = (locals.var_kp * locals.var_xgm__blk1426);
            let assign52840_e68139: f64 = (1.0 + assign52840_e68138);
            let assign52840_e68140: f64 = (assign52840_e68139).sqrt();
            let assign52840_e68141: f64 = (1.0 / assign52840_e68140);
            (locals.var_eta_p__blk1427, locals.var_eta_p__blk1427_dn4, locals.var_eta_p__blk1427_dn6, locals.var_eta_p__blk1427_dn7, locals.var_eta_p__blk1427_dn8, locals.var_eta_p__blk1427_dn9, ) = (assign52840_e68141, (-((((locals.var_kp_dn4 * locals.var_xgm__blk1426) + (locals.var_kp * locals.var_xgm__blk1426_dn4)) / (2.0 * assign52840_e68140)) / (assign52840_e68140 * assign52840_e68140))), (-(((locals.var_kp * locals.var_xgm__blk1426_dn6) / (2.0 * assign52840_e68140)) / (assign52840_e68140 * assign52840_e68140))), (-(((locals.var_kp * locals.var_xgm__blk1426_dn7) / (2.0 * assign52840_e68140)) / (assign52840_e68140 * assign52840_e68140))), (-(((locals.var_kp * locals.var_xgm__blk1426_dn8) / (2.0 * assign52840_e68140)) / (assign52840_e68140 * assign52840_e68140))), (-(((locals.var_kp * locals.var_xgm__blk1426_dn9) / (2.0 * assign52840_e68140)) / (assign52840_e68140 * assign52840_e68140))), );
            locals.var_eta_p__blk1427_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
            let assign52850_e68155: f64 = (locals.var_eta_p__blk1427 + 1.0);
            let assign52850_e68156: f64 = (locals.var_eta_p__blk1427 / assign52850_e68155);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign52850_e68156, (((locals.var_eta_p__blk1427_dn4 * assign52850_e68155) - (locals.var_eta_p__blk1427 * locals.var_eta_p__blk1427_dn4)) / (assign52850_e68155 * assign52850_e68155)), (((locals.var_eta_p__blk1427_dn6 * assign52850_e68155) - (locals.var_eta_p__blk1427 * locals.var_eta_p__blk1427_dn6)) / (assign52850_e68155 * assign52850_e68155)), (((locals.var_eta_p__blk1427_dn7 * assign52850_e68155) - (locals.var_eta_p__blk1427 * locals.var_eta_p__blk1427_dn7)) / (assign52850_e68155 * assign52850_e68155)), (((locals.var_eta_p__blk1427_dn8 * assign52850_e68155) - (locals.var_eta_p__blk1427 * locals.var_eta_p__blk1427_dn8)) / (assign52850_e68155 * assign52850_e68155)), (((locals.var_eta_p__blk1427_dn9 * assign52850_e68155) - (locals.var_eta_p__blk1427 * locals.var_eta_p__blk1427_dn9)) / (assign52850_e68155 * assign52850_e68155)), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
            let assign52860_e68170: f64 = (locals.var_temp__blk949 * locals.var_temp__blk949);
            let assign52860_e68172: f64 = (assign52860_e68170 * locals.var_gf2__blk1325);
            let assign52860_e68174: f64 = (assign52860_e68172 * locals.var_dm__blk1424);
            let assign52860_e68175: f64 = (locals.var_kp * assign52860_e68174);
            (locals.var_x_pm__blk1431, locals.var_x_pm__blk1431_dn4, locals.var_x_pm__blk1431_dn6, locals.var_x_pm__blk1431_dn7, locals.var_x_pm__blk1431_dn8, locals.var_x_pm__blk1431_dn9, ) = (assign52860_e68175, ((locals.var_kp_dn4 * assign52860_e68174) + (locals.var_kp * ((((((locals.var_temp__blk949_dn4 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn4)) * locals.var_gf2__blk1325) + (assign52860_e68170 * locals.var_gf2__blk1325_dn4)) * locals.var_dm__blk1424) + (assign52860_e68172 * locals.var_dm__blk1424_dn4)))), (locals.var_kp * ((((((locals.var_temp__blk949_dn6 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn6)) * locals.var_gf2__blk1325) + (assign52860_e68170 * locals.var_gf2__blk1325_dn6)) * locals.var_dm__blk1424) + (assign52860_e68172 * locals.var_dm__blk1424_dn6))), (locals.var_kp * ((((((locals.var_temp__blk949_dn7 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn7)) * locals.var_gf2__blk1325) + (assign52860_e68170 * locals.var_gf2__blk1325_dn7)) * locals.var_dm__blk1424) + (assign52860_e68172 * locals.var_dm__blk1424_dn7))), (locals.var_kp * ((((((locals.var_temp__blk949_dn8 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn8)) * locals.var_gf2__blk1325) + (assign52860_e68170 * locals.var_gf2__blk1325_dn8)) * locals.var_dm__blk1424) + (assign52860_e68172 * locals.var_dm__blk1424_dn8))), (locals.var_kp * ((((((locals.var_temp__blk949_dn9 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn9)) * locals.var_gf2__blk1325) + (assign52860_e68170 * locals.var_gf2__blk1325_dn9)) * locals.var_dm__blk1424) + (assign52860_e68172 * locals.var_dm__blk1424_dn9))), );
            locals.var_x_pm__blk1431_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
            let assign52870_e68189: f64 = (locals.var_xgm__blk1426 - locals.var_x_pm__blk1431);
            let assign52870_e68190: f64 = (2.0 * assign52870_e68189);
            let assign52870_e68194: f64 = (1.0 - locals.var_em__blk1422);
            let assign52870_e68196: f64 = (assign52870_e68194 + locals.var_dm__blk1424);
            let assign52870_e68197: f64 = (locals.var_gf2__blk1325 * assign52870_e68196);
            let assign52870_e68198: f64 = (assign52870_e68190 + assign52870_e68197);
            (locals.var_p_pd__blk1432, locals.var_p_pd__blk1432_dn4, locals.var_p_pd__blk1432_dn6, locals.var_p_pd__blk1432_dn7, locals.var_p_pd__blk1432_dn8, locals.var_p_pd__blk1432_dn9, ) = (assign52870_e68198, ((2.0 * (locals.var_xgm__blk1426_dn4 - locals.var_x_pm__blk1431_dn4)) + ((locals.var_gf2__blk1325_dn4 * assign52870_e68196) + (locals.var_gf2__blk1325 * ((-locals.var_em__blk1422_dn4) + locals.var_dm__blk1424_dn4)))), ((2.0 * (locals.var_xgm__blk1426_dn6 - locals.var_x_pm__blk1431_dn6)) + ((locals.var_gf2__blk1325_dn6 * assign52870_e68196) + (locals.var_gf2__blk1325 * ((-locals.var_em__blk1422_dn6) + locals.var_dm__blk1424_dn6)))), ((2.0 * (locals.var_xgm__blk1426_dn7 - locals.var_x_pm__blk1431_dn7)) + ((locals.var_gf2__blk1325_dn7 * assign52870_e68196) + (locals.var_gf2__blk1325 * ((-locals.var_em__blk1422_dn7) + locals.var_dm__blk1424_dn7)))), ((2.0 * (locals.var_xgm__blk1426_dn8 - locals.var_x_pm__blk1431_dn8)) + ((locals.var_gf2__blk1325_dn8 * assign52870_e68196) + (locals.var_gf2__blk1325 * ((-locals.var_em__blk1422_dn8) + locals.var_dm__blk1424_dn8)))), ((2.0 * (locals.var_xgm__blk1426_dn9 - locals.var_x_pm__blk1431_dn9)) + ((locals.var_gf2__blk1325_dn9 * assign52870_e68196) + (locals.var_gf2__blk1325 * ((-locals.var_em__blk1422_dn9) + locals.var_dm__blk1424_dn9)))), );
            locals.var_p_pd__blk1432_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
            let assign52880_e68213: f64 = (2.0 * locals.var_xgm__blk1426);
            let assign52880_e68214: f64 = (locals.var_x_pm__blk1431 - assign52880_e68213);
            let assign52880_e68215: f64 = (locals.var_x_pm__blk1431 * assign52880_e68214);
            (locals.var_q_pd__blk1433, locals.var_q_pd__blk1433_dn4, locals.var_q_pd__blk1433_dn6, locals.var_q_pd__blk1433_dn7, locals.var_q_pd__blk1433_dn8, locals.var_q_pd__blk1433_dn9, ) = (assign52880_e68215, ((locals.var_x_pm__blk1431_dn4 * assign52880_e68214) + (locals.var_x_pm__blk1431 * (locals.var_x_pm__blk1431_dn4 - (2.0 * locals.var_xgm__blk1426_dn4)))), ((locals.var_x_pm__blk1431_dn6 * assign52880_e68214) + (locals.var_x_pm__blk1431 * (locals.var_x_pm__blk1431_dn6 - (2.0 * locals.var_xgm__blk1426_dn6)))), ((locals.var_x_pm__blk1431_dn7 * assign52880_e68214) + (locals.var_x_pm__blk1431 * (locals.var_x_pm__blk1431_dn7 - (2.0 * locals.var_xgm__blk1426_dn7)))), ((locals.var_x_pm__blk1431_dn8 * assign52880_e68214) + (locals.var_x_pm__blk1431 * (locals.var_x_pm__blk1431_dn8 - (2.0 * locals.var_xgm__blk1426_dn8)))), ((locals.var_x_pm__blk1431_dn9 * assign52880_e68214) + (locals.var_x_pm__blk1431 * (locals.var_x_pm__blk1431_dn9 - (2.0 * locals.var_xgm__blk1426_dn9)))), );
            locals.var_q_pd__blk1433_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
            let assign52890_e68231: f64 = (locals.var_em__blk1422 + locals.var_dm__blk1424);
            let assign52890_e68232: f64 = (locals.var_gf2__blk1325 * assign52890_e68231);
            let assign52890_e68233: f64 = (0.5 * assign52890_e68232);
            let assign52890_e68234: f64 = (1.0 - assign52890_e68233);
            (locals.var_xi_pd__blk1434, locals.var_xi_pd__blk1434_dn4, locals.var_xi_pd__blk1434_dn6, locals.var_xi_pd__blk1434_dn7, locals.var_xi_pd__blk1434_dn8, locals.var_xi_pd__blk1434_dn9, ) = (assign52890_e68234, (-(0.5 * ((locals.var_gf2__blk1325_dn4 * assign52890_e68231) + (locals.var_gf2__blk1325 * (locals.var_em__blk1422_dn4 + locals.var_dm__blk1424_dn4))))), (-(0.5 * ((locals.var_gf2__blk1325_dn6 * assign52890_e68231) + (locals.var_gf2__blk1325 * (locals.var_em__blk1422_dn6 + locals.var_dm__blk1424_dn6))))), (-(0.5 * ((locals.var_gf2__blk1325_dn7 * assign52890_e68231) + (locals.var_gf2__blk1325 * (locals.var_em__blk1422_dn7 + locals.var_dm__blk1424_dn7))))), (-(0.5 * ((locals.var_gf2__blk1325_dn8 * assign52890_e68231) + (locals.var_gf2__blk1325 * (locals.var_em__blk1422_dn8 + locals.var_dm__blk1424_dn8))))), (-(0.5 * ((locals.var_gf2__blk1325_dn9 * assign52890_e68231) + (locals.var_gf2__blk1325 * (locals.var_em__blk1422_dn9 + locals.var_dm__blk1424_dn9))))), );
            locals.var_xi_pd__blk1434_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
            let assign52900_e68247: f64 = (locals.var_q_pd__blk1433 * locals.var_p_pd__blk1432);
            let assign52900_e68250: f64 = (locals.var_p_pd__blk1432 * locals.var_p_pd__blk1432);
            let assign52900_e68253: f64 = (locals.var_xi_pd__blk1434 * locals.var_q_pd__blk1433);
            let assign52900_e68254: f64 = (assign52900_e68250 - assign52900_e68253);
            let assign52900_e68255: f64 = (assign52900_e68247 / assign52900_e68254);
            (locals.var_u_pd__blk1435, locals.var_u_pd__blk1435_dn4, locals.var_u_pd__blk1435_dn6, locals.var_u_pd__blk1435_dn7, locals.var_u_pd__blk1435_dn8, locals.var_u_pd__blk1435_dn9, ) = (assign52900_e68255, (((((locals.var_q_pd__blk1433_dn4 * locals.var_p_pd__blk1432) + (locals.var_q_pd__blk1433 * locals.var_p_pd__blk1432_dn4)) * assign52900_e68254) - (assign52900_e68247 * (((locals.var_p_pd__blk1432_dn4 * locals.var_p_pd__blk1432) + (locals.var_p_pd__blk1432 * locals.var_p_pd__blk1432_dn4)) - ((locals.var_xi_pd__blk1434_dn4 * locals.var_q_pd__blk1433) + (locals.var_xi_pd__blk1434 * locals.var_q_pd__blk1433_dn4))))) / (assign52900_e68254 * assign52900_e68254)), (((((locals.var_q_pd__blk1433_dn6 * locals.var_p_pd__blk1432) + (locals.var_q_pd__blk1433 * locals.var_p_pd__blk1432_dn6)) * assign52900_e68254) - (assign52900_e68247 * (((locals.var_p_pd__blk1432_dn6 * locals.var_p_pd__blk1432) + (locals.var_p_pd__blk1432 * locals.var_p_pd__blk1432_dn6)) - ((locals.var_xi_pd__blk1434_dn6 * locals.var_q_pd__blk1433) + (locals.var_xi_pd__blk1434 * locals.var_q_pd__blk1433_dn6))))) / (assign52900_e68254 * assign52900_e68254)), (((((locals.var_q_pd__blk1433_dn7 * locals.var_p_pd__blk1432) + (locals.var_q_pd__blk1433 * locals.var_p_pd__blk1432_dn7)) * assign52900_e68254) - (assign52900_e68247 * (((locals.var_p_pd__blk1432_dn7 * locals.var_p_pd__blk1432) + (locals.var_p_pd__blk1432 * locals.var_p_pd__blk1432_dn7)) - ((locals.var_xi_pd__blk1434_dn7 * locals.var_q_pd__blk1433) + (locals.var_xi_pd__blk1434 * locals.var_q_pd__blk1433_dn7))))) / (assign52900_e68254 * assign52900_e68254)), (((((locals.var_q_pd__blk1433_dn8 * locals.var_p_pd__blk1432) + (locals.var_q_pd__blk1433 * locals.var_p_pd__blk1432_dn8)) * assign52900_e68254) - (assign52900_e68247 * (((locals.var_p_pd__blk1432_dn8 * locals.var_p_pd__blk1432) + (locals.var_p_pd__blk1432 * locals.var_p_pd__blk1432_dn8)) - ((locals.var_xi_pd__blk1434_dn8 * locals.var_q_pd__blk1433) + (locals.var_xi_pd__blk1434 * locals.var_q_pd__blk1433_dn8))))) / (assign52900_e68254 * assign52900_e68254)), (((((locals.var_q_pd__blk1433_dn9 * locals.var_p_pd__blk1432) + (locals.var_q_pd__blk1433 * locals.var_p_pd__blk1432_dn9)) * assign52900_e68254) - (assign52900_e68247 * (((locals.var_p_pd__blk1432_dn9 * locals.var_p_pd__blk1432) + (locals.var_p_pd__blk1432 * locals.var_p_pd__blk1432_dn9)) - ((locals.var_xi_pd__blk1434_dn9 * locals.var_q_pd__blk1433) + (locals.var_xi_pd__blk1434 * locals.var_q_pd__blk1433_dn9))))) / (assign52900_e68254 * assign52900_e68254)), );
            locals.var_u_pd__blk1435_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
            let assign52910_e68268: f64 = (locals.var_x_m__blk1421 + locals.var_u_pd__blk1435);
            (locals.var_x_m__blk1421, locals.var_x_m__blk1421_dn4, locals.var_x_m__blk1421_dn6, locals.var_x_m__blk1421_dn7, locals.var_x_m__blk1421_dn8, locals.var_x_m__blk1421_dn9, ) = (assign52910_e68268, (locals.var_x_m__blk1421_dn4 + locals.var_u_pd__blk1435_dn4), (locals.var_x_m__blk1421_dn6 + locals.var_u_pd__blk1435_dn6), (locals.var_x_m__blk1421_dn7 + locals.var_u_pd__blk1435_dn7), (locals.var_x_m__blk1421_dn8 + locals.var_u_pd__blk1435_dn8), (locals.var_x_m__blk1421_dn9 + locals.var_u_pd__blk1435_dn9), );
            locals.var_x_m__blk1421_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
            let assign52920_e68280: f64 = (locals.var_u_pd__blk1435).exp();
            (locals.var_km__blk1436, locals.var_km__blk1436_dn4, locals.var_km__blk1436_dn6, locals.var_km__blk1436_dn7, locals.var_km__blk1436_dn8, locals.var_km__blk1436_dn9, ) = (assign52920_e68280, (assign52920_e68280 * locals.var_u_pd__blk1435_dn4), (assign52920_e68280 * locals.var_u_pd__blk1435_dn6), (assign52920_e68280 * locals.var_u_pd__blk1435_dn7), (assign52920_e68280 * locals.var_u_pd__blk1435_dn8), (assign52920_e68280 * locals.var_u_pd__blk1435_dn9), );
            locals.var_km__blk1436_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
            let assign52930_e68293: f64 = (locals.var_em__blk1422 / locals.var_km__blk1436);
            (locals.var_em__blk1422, locals.var_em__blk1422_dn4, locals.var_em__blk1422_dn6, locals.var_em__blk1422_dn7, locals.var_em__blk1422_dn8, locals.var_em__blk1422_dn9, ) = (assign52930_e68293, (((locals.var_em__blk1422_dn4 * locals.var_km__blk1436) - (locals.var_em__blk1422 * locals.var_km__blk1436_dn4)) / (locals.var_km__blk1436 * locals.var_km__blk1436)), (((locals.var_em__blk1422_dn6 * locals.var_km__blk1436) - (locals.var_em__blk1422 * locals.var_km__blk1436_dn6)) / (locals.var_km__blk1436 * locals.var_km__blk1436)), (((locals.var_em__blk1422_dn7 * locals.var_km__blk1436) - (locals.var_em__blk1422 * locals.var_km__blk1436_dn7)) / (locals.var_km__blk1436 * locals.var_km__blk1436)), (((locals.var_em__blk1422_dn8 * locals.var_km__blk1436) - (locals.var_em__blk1422 * locals.var_km__blk1436_dn8)) / (locals.var_km__blk1436 * locals.var_km__blk1436)), (((locals.var_em__blk1422_dn9 * locals.var_km__blk1436) - (locals.var_em__blk1422 * locals.var_km__blk1436_dn9)) / (locals.var_km__blk1436 * locals.var_km__blk1436)), );
            locals.var_em__blk1422_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
            let assign52940_e68306: f64 = (locals.var_dm__blk1424 * locals.var_km__blk1436);
            (locals.var_dm__blk1424, locals.var_dm__blk1424_dn4, locals.var_dm__blk1424_dn6, locals.var_dm__blk1424_dn7, locals.var_dm__blk1424_dn8, locals.var_dm__blk1424_dn9, ) = (assign52940_e68306, ((locals.var_dm__blk1424_dn4 * locals.var_km__blk1436) + (locals.var_dm__blk1424 * locals.var_km__blk1436_dn4)), ((locals.var_dm__blk1424_dn6 * locals.var_km__blk1436) + (locals.var_dm__blk1424 * locals.var_km__blk1436_dn6)), ((locals.var_dm__blk1424_dn7 * locals.var_km__blk1436) + (locals.var_dm__blk1424 * locals.var_km__blk1436_dn7)), ((locals.var_dm__blk1424_dn8 * locals.var_km__blk1436) + (locals.var_dm__blk1424 * locals.var_km__blk1436_dn8)), ((locals.var_dm__blk1424_dn9 * locals.var_km__blk1436) + (locals.var_dm__blk1424 * locals.var_km__blk1436_dn9)), );
            locals.var_dm__blk1424_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
            let assign52950_e68319: f64 = (locals.var_x_m__blk1421 - 1.0);
            let assign52950_e68321: f64 = (assign52950_e68319 + locals.var_em__blk1422);
            (locals.var_pm__blk1425, locals.var_pm__blk1425_dn4, locals.var_pm__blk1425_dn6, locals.var_pm__blk1425_dn7, locals.var_pm__blk1425_dn8, locals.var_pm__blk1425_dn9, ) = (assign52950_e68321, (locals.var_x_m__blk1421_dn4 + locals.var_em__blk1422_dn4), (locals.var_x_m__blk1421_dn6 + locals.var_em__blk1422_dn6), (locals.var_x_m__blk1421_dn7 + locals.var_em__blk1422_dn7), (locals.var_x_m__blk1421_dn8 + locals.var_em__blk1422_dn8), (locals.var_x_m__blk1421_dn9 + locals.var_em__blk1422_dn9), );
            locals.var_pm__blk1425_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
            let assign52960_e68335: f64 = (locals.var_dm__blk1424 + locals.var_pm__blk1425);
            let assign52960_e68336: f64 = (assign52960_e68335).sqrt();
            let assign52960_e68337: f64 = (locals.var_gf__blk1324 * assign52960_e68336);
            (locals.var_xgm__blk1426, locals.var_xgm__blk1426_dn4, locals.var_xgm__blk1426_dn6, locals.var_xgm__blk1426_dn7, locals.var_xgm__blk1426_dn8, locals.var_xgm__blk1426_dn9, ) = (assign52960_e68337, ((locals.var_gf__blk1324_dn4 * assign52960_e68336) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn4 + locals.var_pm__blk1425_dn4) / (2.0 * assign52960_e68336)))), ((locals.var_gf__blk1324_dn6 * assign52960_e68336) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn6 + locals.var_pm__blk1425_dn6) / (2.0 * assign52960_e68336)))), ((locals.var_gf__blk1324_dn7 * assign52960_e68336) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn7 + locals.var_pm__blk1425_dn7) / (2.0 * assign52960_e68336)))), ((locals.var_gf__blk1324_dn8 * assign52960_e68336) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn8 + locals.var_pm__blk1425_dn8) / (2.0 * assign52960_e68336)))), ((locals.var_gf__blk1324_dn9 * assign52960_e68336) + (locals.var_gf__blk1324 * ((locals.var_dm__blk1424_dn9 + locals.var_pm__blk1425_dn9) / (2.0 * assign52960_e68336)))), );
            locals.var_xgm__blk1426_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
            let assign52970_e68350: f64 = (1.0 - locals.var_em__blk1422);
            let assign52970_e68354: f64 = (locals.var_xgm__blk1426 * locals.var_eta_p__blk1427);
            let assign52970_e68356: f64 = (assign52970_e68354 * locals.var_inv_gf2__blk1341);
            let assign52970_e68357: f64 = (2.0 * assign52970_e68356);
            let assign52970_e68358: f64 = (assign52970_e68350 + assign52970_e68357);
            (locals.var_km0__blk1437, locals.var_km0__blk1437_dn4, locals.var_km0__blk1437_dn6, locals.var_km0__blk1437_dn7, locals.var_km0__blk1437_dn8, locals.var_km0__blk1437_dn9, ) = (assign52970_e68358, ((-locals.var_em__blk1422_dn4) + (2.0 * ((((locals.var_xgm__blk1426_dn4 * locals.var_eta_p__blk1427) + (locals.var_xgm__blk1426 * locals.var_eta_p__blk1427_dn4)) * locals.var_inv_gf2__blk1341) + (assign52970_e68354 * locals.var_inv_gf2__blk1341_dn4)))), ((-locals.var_em__blk1422_dn6) + (2.0 * ((((locals.var_xgm__blk1426_dn6 * locals.var_eta_p__blk1427) + (locals.var_xgm__blk1426 * locals.var_eta_p__blk1427_dn6)) * locals.var_inv_gf2__blk1341) + (assign52970_e68354 * locals.var_inv_gf2__blk1341_dn6)))), ((-locals.var_em__blk1422_dn7) + (2.0 * ((((locals.var_xgm__blk1426_dn7 * locals.var_eta_p__blk1427) + (locals.var_xgm__blk1426 * locals.var_eta_p__blk1427_dn7)) * locals.var_inv_gf2__blk1341) + (assign52970_e68354 * locals.var_inv_gf2__blk1341_dn7)))), ((-locals.var_em__blk1422_dn8) + (2.0 * ((((locals.var_xgm__blk1426_dn8 * locals.var_eta_p__blk1427) + (locals.var_xgm__blk1426 * locals.var_eta_p__blk1427_dn8)) * locals.var_inv_gf2__blk1341) + (assign52970_e68354 * locals.var_inv_gf2__blk1341_dn8)))), ((-locals.var_em__blk1422_dn9) + (2.0 * ((((locals.var_xgm__blk1426_dn9 * locals.var_eta_p__blk1427) + (locals.var_xgm__blk1426 * locals.var_eta_p__blk1427_dn9)) * locals.var_inv_gf2__blk1341) + (assign52970_e68354 * locals.var_inv_gf2__blk1341_dn9)))), );
            locals.var_km0__blk1437_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
            let assign52980_e68371: f64 = (locals.var_x_ds__blk1411 * locals.var_km__blk1436);
            let assign52980_e68374: f64 = (locals.var_d0__blk1430 + locals.var_d_bar__blk1423);
            let assign52980_e68375: f64 = (assign52980_e68371 * assign52980_e68374);
            let assign52980_e68379: f64 = (locals.var_km__blk1436 * locals.var_d_bar__blk1423);
            let assign52980_e68380: f64 = (locals.var_km0__blk1437 + assign52980_e68379);
            let assign52980_e68381: f64 = (assign52980_e68375 / assign52980_e68380);
            (locals.var_x_ds__blk1411, locals.var_x_ds__blk1411_dn4, locals.var_x_ds__blk1411_dn6, locals.var_x_ds__blk1411_dn7, locals.var_x_ds__blk1411_dn8, locals.var_x_ds__blk1411_dn9, ) = (assign52980_e68381, (((((((locals.var_x_ds__blk1411_dn4 * locals.var_km__blk1436) + (locals.var_x_ds__blk1411 * locals.var_km__blk1436_dn4)) * assign52980_e68374) + (assign52980_e68371 * (locals.var_d0__blk1430_dn4 + locals.var_d_bar__blk1423_dn4))) * assign52980_e68380) - (assign52980_e68375 * (locals.var_km0__blk1437_dn4 + ((locals.var_km__blk1436_dn4 * locals.var_d_bar__blk1423) + (locals.var_km__blk1436 * locals.var_d_bar__blk1423_dn4))))) / (assign52980_e68380 * assign52980_e68380)), (((((((locals.var_x_ds__blk1411_dn6 * locals.var_km__blk1436) + (locals.var_x_ds__blk1411 * locals.var_km__blk1436_dn6)) * assign52980_e68374) + (assign52980_e68371 * (locals.var_d0__blk1430_dn6 + locals.var_d_bar__blk1423_dn6))) * assign52980_e68380) - (assign52980_e68375 * (locals.var_km0__blk1437_dn6 + ((locals.var_km__blk1436_dn6 * locals.var_d_bar__blk1423) + (locals.var_km__blk1436 * locals.var_d_bar__blk1423_dn6))))) / (assign52980_e68380 * assign52980_e68380)), (((((((locals.var_x_ds__blk1411_dn7 * locals.var_km__blk1436) + (locals.var_x_ds__blk1411 * locals.var_km__blk1436_dn7)) * assign52980_e68374) + (assign52980_e68371 * (locals.var_d0__blk1430_dn7 + locals.var_d_bar__blk1423_dn7))) * assign52980_e68380) - (assign52980_e68375 * (locals.var_km0__blk1437_dn7 + ((locals.var_km__blk1436_dn7 * locals.var_d_bar__blk1423) + (locals.var_km__blk1436 * locals.var_d_bar__blk1423_dn7))))) / (assign52980_e68380 * assign52980_e68380)), (((((((locals.var_x_ds__blk1411_dn8 * locals.var_km__blk1436) + (locals.var_x_ds__blk1411 * locals.var_km__blk1436_dn8)) * assign52980_e68374) + (assign52980_e68371 * (locals.var_d0__blk1430_dn8 + locals.var_d_bar__blk1423_dn8))) * assign52980_e68380) - (assign52980_e68375 * (locals.var_km0__blk1437_dn8 + ((locals.var_km__blk1436_dn8 * locals.var_d_bar__blk1423) + (locals.var_km__blk1436 * locals.var_d_bar__blk1423_dn8))))) / (assign52980_e68380 * assign52980_e68380)), (((((((locals.var_x_ds__blk1411_dn9 * locals.var_km__blk1436) + (locals.var_x_ds__blk1411 * locals.var_km__blk1436_dn9)) * assign52980_e68374) + (assign52980_e68371 * (locals.var_d0__blk1430_dn9 + locals.var_d_bar__blk1423_dn9))) * assign52980_e68380) - (assign52980_e68375 * (locals.var_km0__blk1437_dn9 + ((locals.var_km__blk1436_dn9 * locals.var_d_bar__blk1423) + (locals.var_km__blk1436 * locals.var_d_bar__blk1423_dn9))))) / (assign52980_e68380 * assign52980_e68380)), );
            locals.var_x_ds__blk1411_rv = 0.0;
        }

        if ((((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) && (locals.var_guard1519 != 0.0)) {
            let assign52990_e68394: f64 = (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339);
            (locals.var_dps__blk1414, locals.var_dps__blk1414_dn4, locals.var_dps__blk1414_dn6, locals.var_dps__blk1414_dn7, locals.var_dps__blk1414_dn8, locals.var_dps__blk1414_dn9, ) = (assign52990_e68394, ((locals.var_x_ds__blk1411_dn4 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn4)), ((locals.var_x_ds__blk1411_dn6 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn6)), ((locals.var_x_ds__blk1411_dn7 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn7)), ((locals.var_x_ds__blk1411_dn8 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn8)), ((locals.var_x_ds__blk1411_dn9 * locals.var_phit1__blk1339) + (locals.var_x_ds__blk1411 * locals.var_phit1__blk1339_dn9)), );
            locals.var_dps__blk1414_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) {
            let assign53000_e68404: f64 = (locals.var_pm__blk1425).sqrt();
            (locals.var_sqm__blk1428, locals.var_sqm__blk1428_dn4, locals.var_sqm__blk1428_dn6, locals.var_sqm__blk1428_dn7, locals.var_sqm__blk1428_dn8, locals.var_sqm__blk1428_dn9, ) = (assign53000_e68404, (locals.var_pm__blk1425_dn4 / (2.0 * assign53000_e68404)), (locals.var_pm__blk1425_dn6 / (2.0 * assign53000_e68404)), (locals.var_pm__blk1425_dn7 / (2.0 * assign53000_e68404)), (locals.var_pm__blk1425_dn8 / (2.0 * assign53000_e68404)), (locals.var_pm__blk1425_dn9 / (2.0 * assign53000_e68404)), );
            locals.var_sqm__blk1428_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1517 == 0.0)) {
            let assign53010_e68418: f64 = (1.0 - locals.var_em__blk1422);
            let assign53010_e68419: f64 = (locals.var_gf__blk1324 * assign53010_e68418);
            let assign53010_e68421: f64 = (assign53010_e68419 / locals.var_sqm__blk1428);
            let assign53010_e68422: f64 = (0.5 * assign53010_e68421);
            let assign53010_e68423: f64 = (locals.var_eta_p__blk1427 + assign53010_e68422);
            (locals.var_alpha__blk1429, locals.var_alpha__blk1429_dn4, locals.var_alpha__blk1429_dn6, locals.var_alpha__blk1429_dn7, locals.var_alpha__blk1429_dn8, locals.var_alpha__blk1429_dn9, ) = (assign53010_e68423, (locals.var_eta_p__blk1427_dn4 + (0.5 * (((((locals.var_gf__blk1324_dn4 * assign53010_e68418) + (locals.var_gf__blk1324 * (-locals.var_em__blk1422_dn4))) * locals.var_sqm__blk1428) - (assign53010_e68419 * locals.var_sqm__blk1428_dn4)) / (locals.var_sqm__blk1428 * locals.var_sqm__blk1428)))), (locals.var_eta_p__blk1427_dn6 + (0.5 * (((((locals.var_gf__blk1324_dn6 * assign53010_e68418) + (locals.var_gf__blk1324 * (-locals.var_em__blk1422_dn6))) * locals.var_sqm__blk1428) - (assign53010_e68419 * locals.var_sqm__blk1428_dn6)) / (locals.var_sqm__blk1428 * locals.var_sqm__blk1428)))), (locals.var_eta_p__blk1427_dn7 + (0.5 * (((((locals.var_gf__blk1324_dn7 * assign53010_e68418) + (locals.var_gf__blk1324 * (-locals.var_em__blk1422_dn7))) * locals.var_sqm__blk1428) - (assign53010_e68419 * locals.var_sqm__blk1428_dn7)) / (locals.var_sqm__blk1428 * locals.var_sqm__blk1428)))), (locals.var_eta_p__blk1427_dn8 + (0.5 * (((((locals.var_gf__blk1324_dn8 * assign53010_e68418) + (locals.var_gf__blk1324 * (-locals.var_em__blk1422_dn8))) * locals.var_sqm__blk1428) - (assign53010_e68419 * locals.var_sqm__blk1428_dn8)) / (locals.var_sqm__blk1428 * locals.var_sqm__blk1428)))), (locals.var_eta_p__blk1427_dn9 + (0.5 * (((((locals.var_gf__blk1324_dn9 * assign53010_e68418) + (locals.var_gf__blk1324 * (-locals.var_em__blk1422_dn9))) * locals.var_sqm__blk1428) - (assign53010_e68419 * locals.var_sqm__blk1428_dn9)) / (locals.var_sqm__blk1428 * locals.var_sqm__blk1428)))), );
            locals.var_alpha__blk1429_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign53020_e68432: f64 = (locals.var_gf2__blk1325 * locals.var_dm__blk1424);
            let assign53020_e68436: f64 = (locals.var_gf__blk1324 * locals.var_sqm__blk1428);
            let assign53020_e68437: f64 = (locals.var_xgm__blk1426 + assign53020_e68436);
            let assign53020_e68438: f64 = (assign53020_e68432 / assign53020_e68437);
            let assign53020_e68439: f64 = (locals.var_phit1__blk1339 * assign53020_e68438);
            (locals.var_qim__blk1438, locals.var_qim__blk1438_dn4, locals.var_qim__blk1438_dn6, locals.var_qim__blk1438_dn7, locals.var_qim__blk1438_dn8, locals.var_qim__blk1438_dn9, ) = (assign53020_e68439, ((locals.var_phit1__blk1339_dn4 * assign53020_e68438) + (locals.var_phit1__blk1339 * (((((locals.var_gf2__blk1325_dn4 * locals.var_dm__blk1424) + (locals.var_gf2__blk1325 * locals.var_dm__blk1424_dn4)) * assign53020_e68437) - (assign53020_e68432 * (locals.var_xgm__blk1426_dn4 + ((locals.var_gf__blk1324_dn4 * locals.var_sqm__blk1428) + (locals.var_gf__blk1324 * locals.var_sqm__blk1428_dn4))))) / (assign53020_e68437 * assign53020_e68437)))), ((locals.var_phit1__blk1339_dn6 * assign53020_e68438) + (locals.var_phit1__blk1339 * (((((locals.var_gf2__blk1325_dn6 * locals.var_dm__blk1424) + (locals.var_gf2__blk1325 * locals.var_dm__blk1424_dn6)) * assign53020_e68437) - (assign53020_e68432 * (locals.var_xgm__blk1426_dn6 + ((locals.var_gf__blk1324_dn6 * locals.var_sqm__blk1428) + (locals.var_gf__blk1324 * locals.var_sqm__blk1428_dn6))))) / (assign53020_e68437 * assign53020_e68437)))), ((locals.var_phit1__blk1339_dn7 * assign53020_e68438) + (locals.var_phit1__blk1339 * (((((locals.var_gf2__blk1325_dn7 * locals.var_dm__blk1424) + (locals.var_gf2__blk1325 * locals.var_dm__blk1424_dn7)) * assign53020_e68437) - (assign53020_e68432 * (locals.var_xgm__blk1426_dn7 + ((locals.var_gf__blk1324_dn7 * locals.var_sqm__blk1428) + (locals.var_gf__blk1324 * locals.var_sqm__blk1428_dn7))))) / (assign53020_e68437 * assign53020_e68437)))), ((locals.var_phit1__blk1339_dn8 * assign53020_e68438) + (locals.var_phit1__blk1339 * (((((locals.var_gf2__blk1325_dn8 * locals.var_dm__blk1424) + (locals.var_gf2__blk1325 * locals.var_dm__blk1424_dn8)) * assign53020_e68437) - (assign53020_e68432 * (locals.var_xgm__blk1426_dn8 + ((locals.var_gf__blk1324_dn8 * locals.var_sqm__blk1428) + (locals.var_gf__blk1324 * locals.var_sqm__blk1428_dn8))))) / (assign53020_e68437 * assign53020_e68437)))), ((locals.var_phit1__blk1339_dn9 * assign53020_e68438) + (locals.var_phit1__blk1339 * (((((locals.var_gf2__blk1325_dn9 * locals.var_dm__blk1424) + (locals.var_gf2__blk1325 * locals.var_dm__blk1424_dn9)) * assign53020_e68437) - (assign53020_e68432 * (locals.var_xgm__blk1426_dn9 + ((locals.var_gf__blk1324_dn9 * locals.var_sqm__blk1428) + (locals.var_gf__blk1324 * locals.var_sqm__blk1428_dn9))))) / (assign53020_e68437 * assign53020_e68437)))), );
            locals.var_qim__blk1438_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign53030_e68448: f64 = (locals.var_phit1__blk1339 * locals.var_alpha__blk1429);
            let assign53030_e68449: f64 = (locals.var_qim__blk1438 + assign53030_e68448);
            (locals.var_qim1__blk1439, locals.var_qim1__blk1439_dn4, locals.var_qim1__blk1439_dn6, locals.var_qim1__blk1439_dn7, locals.var_qim1__blk1439_dn8, locals.var_qim1__blk1439_dn9, ) = (assign53030_e68449, (locals.var_qim__blk1438_dn4 + ((locals.var_phit1__blk1339_dn4 * locals.var_alpha__blk1429) + (locals.var_phit1__blk1339 * locals.var_alpha__blk1429_dn4))), (locals.var_qim__blk1438_dn6 + ((locals.var_phit1__blk1339_dn6 * locals.var_alpha__blk1429) + (locals.var_phit1__blk1339 * locals.var_alpha__blk1429_dn6))), (locals.var_qim__blk1438_dn7 + ((locals.var_phit1__blk1339_dn7 * locals.var_alpha__blk1429) + (locals.var_phit1__blk1339 * locals.var_alpha__blk1429_dn7))), (locals.var_qim__blk1438_dn8 + ((locals.var_phit1__blk1339_dn8 * locals.var_alpha__blk1429) + (locals.var_phit1__blk1339 * locals.var_alpha__blk1429_dn8))), (locals.var_qim__blk1438_dn9 + ((locals.var_phit1__blk1339_dn9 * locals.var_alpha__blk1429) + (locals.var_phit1__blk1339 * locals.var_alpha__blk1429_dn9))), );
            locals.var_qim1__blk1439_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign53040_e68457: f64 = (locals.var_sqm__blk1428 * locals.var_gf__blk1324);
            let assign53040_e68459: f64 = (assign53040_e68457 * locals.var_phit1__blk1339);
            (locals.var_qbm__blk1440, locals.var_qbm__blk1440_dn4, locals.var_qbm__blk1440_dn6, locals.var_qbm__blk1440_dn7, locals.var_qbm__blk1440_dn8, locals.var_qbm__blk1440_dn9, ) = (assign53040_e68459, ((((locals.var_sqm__blk1428_dn4 * locals.var_gf__blk1324) + (locals.var_sqm__blk1428 * locals.var_gf__blk1324_dn4)) * locals.var_phit1__blk1339) + (assign53040_e68457 * locals.var_phit1__blk1339_dn4)), ((((locals.var_sqm__blk1428_dn6 * locals.var_gf__blk1324) + (locals.var_sqm__blk1428 * locals.var_gf__blk1324_dn6)) * locals.var_phit1__blk1339) + (assign53040_e68457 * locals.var_phit1__blk1339_dn6)), ((((locals.var_sqm__blk1428_dn7 * locals.var_gf__blk1324) + (locals.var_sqm__blk1428 * locals.var_gf__blk1324_dn7)) * locals.var_phit1__blk1339) + (assign53040_e68457 * locals.var_phit1__blk1339_dn7)), ((((locals.var_sqm__blk1428_dn8 * locals.var_gf__blk1324) + (locals.var_sqm__blk1428 * locals.var_gf__blk1324_dn8)) * locals.var_phit1__blk1339) + (assign53040_e68457 * locals.var_phit1__blk1339_dn8)), ((((locals.var_sqm__blk1428_dn9 * locals.var_gf__blk1324) + (locals.var_sqm__blk1428 * locals.var_gf__blk1324_dn9)) * locals.var_phit1__blk1339) + (assign53040_e68457 * locals.var_phit1__blk1339_dn9)), );
            locals.var_qbm__blk1440_rv = 0.0;
        }

        let assign53050_e68464: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1520 = assign53050_e68464;
        locals.var_guard1520_rv = 0.0;

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1520 != 0.0)) {
            let assign53060_e68473: f64 = (locals.var_rsg_i * locals.var_qim__blk1438);
            let assign53060_e68474: f64 = (1.0 - assign53060_e68473);
            (locals.var_rhog__blk1379, locals.var_rhog__blk1379_dn4, locals.var_rhog__blk1379_dn6, locals.var_rhog__blk1379_dn7, locals.var_rhog__blk1379_dn8, locals.var_rhog__blk1379_dn9, ) = (assign53060_e68474, (-(locals.var_rsg_i * locals.var_qim__blk1438_dn4)), (-(locals.var_rsg_i * locals.var_qim__blk1438_dn6)), (-(locals.var_rsg_i * locals.var_qim__blk1438_dn7)), (-(locals.var_rsg_i * locals.var_qim__blk1438_dn8)), (-(locals.var_rsg_i * locals.var_qim__blk1438_dn9)), );
            locals.var_rhog__blk1379_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1520 == 0.0)) {
            let assign53070_e68487: f64 = (locals.var_rsg_i * locals.var_qim__blk1438);
            let assign53070_e68488: f64 = (1.0 + assign53070_e68487);
            let assign53070_e68489: f64 = (1.0 / assign53070_e68488);
            (locals.var_rhog__blk1379, locals.var_rhog__blk1379_dn4, locals.var_rhog__blk1379_dn6, locals.var_rhog__blk1379_dn7, locals.var_rhog__blk1379_dn8, locals.var_rhog__blk1379_dn9, ) = (assign53070_e68489, (-((locals.var_rsg_i * locals.var_qim__blk1438_dn4) / (assign53070_e68488 * assign53070_e68488))), (-((locals.var_rsg_i * locals.var_qim__blk1438_dn6) / (assign53070_e68488 * assign53070_e68488))), (-((locals.var_rsg_i * locals.var_qim__blk1438_dn7) / (assign53070_e68488 * assign53070_e68488))), (-((locals.var_rsg_i * locals.var_qim__blk1438_dn8) / (assign53070_e68488 * assign53070_e68488))), (-((locals.var_rsg_i * locals.var_qim__blk1438_dn9) / (assign53070_e68488 * assign53070_e68488))), );
            locals.var_rhog__blk1379_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign53080_e68497: f64 = (locals.var_ther_i * locals.var_rhob__blk1378);
            let assign53080_e68499: f64 = (assign53080_e68497 * locals.var_rhog__blk1379);
            let assign53080_e68501: f64 = (assign53080_e68499 * locals.var_qim__blk1438);
            (locals.var_gr__blk1380, locals.var_gr__blk1380_dn4, locals.var_gr__blk1380_dn6, locals.var_gr__blk1380_dn7, locals.var_gr__blk1380_dn8, locals.var_gr__blk1380_dn9, ) = (assign53080_e68501, ((((((locals.var_ther_i_dn4 * locals.var_rhob__blk1378) + (locals.var_ther_i * locals.var_rhob__blk1378_dn4)) * locals.var_rhog__blk1379) + (assign53080_e68497 * locals.var_rhog__blk1379_dn4)) * locals.var_qim__blk1438) + (assign53080_e68499 * locals.var_qim__blk1438_dn4)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn6) * locals.var_rhog__blk1379) + (assign53080_e68497 * locals.var_rhog__blk1379_dn6)) * locals.var_qim__blk1438) + (assign53080_e68499 * locals.var_qim__blk1438_dn6)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn7) * locals.var_rhog__blk1379) + (assign53080_e68497 * locals.var_rhog__blk1379_dn7)) * locals.var_qim__blk1438) + (assign53080_e68499 * locals.var_qim__blk1438_dn7)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn8) * locals.var_rhog__blk1379) + (assign53080_e68497 * locals.var_rhog__blk1379_dn8)) * locals.var_qim__blk1438) + (assign53080_e68499 * locals.var_qim__blk1438_dn8)), (((((locals.var_ther_i * locals.var_rhob__blk1378_dn9) * locals.var_rhog__blk1379) + (assign53080_e68497 * locals.var_rhog__blk1379_dn9)) * locals.var_qim__blk1438) + (assign53080_e68499 * locals.var_qim__blk1438_dn9)), );
            locals.var_gr__blk1380_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign53090_e68510: f64 = (locals.var_eta_mu * locals.var_qim__blk1438);
            let assign53090_e68511: f64 = (locals.var_qbm__blk1440 + assign53090_e68510);
            (locals.var_qeff__blk1441, locals.var_qeff__blk1441_dn4, locals.var_qeff__blk1441_dn6, locals.var_qeff__blk1441_dn7, locals.var_qeff__blk1441_dn8, locals.var_qeff__blk1441_dn9, ) = (assign53090_e68511, (locals.var_qbm__blk1440_dn4 + (locals.var_eta_mu * locals.var_qim__blk1438_dn4)), (locals.var_qbm__blk1440_dn6 + (locals.var_eta_mu * locals.var_qim__blk1438_dn6)), (locals.var_qbm__blk1440_dn7 + (locals.var_eta_mu * locals.var_qim__blk1438_dn7)), (locals.var_qbm__blk1440_dn8 + (locals.var_eta_mu * locals.var_qim__blk1438_dn8)), (locals.var_qbm__blk1440_dn9 + (locals.var_eta_mu * locals.var_qim__blk1438_dn9)), );
            locals.var_qeff__blk1441_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign53100_e68520: f64 = (locals.var_eta_mu1 * locals.var_qim__blk1438);
            let assign53100_e68521: f64 = (locals.var_qbm__blk1440 + assign53100_e68520);
            (locals.var_qeff1__blk1442, locals.var_qeff1__blk1442_dn4, locals.var_qeff1__blk1442_dn6, locals.var_qeff1__blk1442_dn7, locals.var_qeff1__blk1442_dn8, locals.var_qeff1__blk1442_dn9, ) = (assign53100_e68521, (locals.var_qbm__blk1440_dn4 + (locals.var_eta_mu1 * locals.var_qim__blk1438_dn4)), (locals.var_qbm__blk1440_dn6 + (locals.var_eta_mu1 * locals.var_qim__blk1438_dn6)), (locals.var_qbm__blk1440_dn7 + (locals.var_eta_mu1 * locals.var_qim__blk1438_dn7)), (locals.var_qbm__blk1440_dn8 + (locals.var_eta_mu1 * locals.var_qim__blk1438_dn8)), (locals.var_qbm__blk1440_dn9 + (locals.var_eta_mu1 * locals.var_qim__blk1438_dn9)), );
            locals.var_qeff1__blk1442_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign53110_e68529: f64 = (locals.var_e_eff0 * locals.var_qeff__blk1441);
            (locals.var_eeffm__blk1443, locals.var_eeffm__blk1443_dn4, locals.var_eeffm__blk1443_dn6, locals.var_eeffm__blk1443_dn7, locals.var_eeffm__blk1443_dn8, locals.var_eeffm__blk1443_dn9, ) = (assign53110_e68529, (locals.var_e_eff0 * locals.var_qeff__blk1441_dn4), (locals.var_e_eff0 * locals.var_qeff__blk1441_dn6), (locals.var_e_eff0 * locals.var_qeff__blk1441_dn7), (locals.var_e_eff0 * locals.var_qeff__blk1441_dn8), (locals.var_e_eff0 * locals.var_qeff__blk1441_dn9), );
            locals.var_eeffm__blk1443_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign53120_e68538: f64 = (locals.var_pm__blk1425 + locals.var_dm__blk1424);
            let assign53120_e68540: f64 = (assign53120_e68538 + 1e-14);
            let assign53120_e68541: f64 = (locals.var_pm__blk1425 / assign53120_e68540);
            let assign53120_e68542: f64 = (assign53120_e68541).ln();
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign53120_e68542, ((((locals.var_pm__blk1425_dn4 * assign53120_e68540) - (locals.var_pm__blk1425 * (locals.var_pm__blk1425_dn4 + locals.var_dm__blk1424_dn4))) / (assign53120_e68540 * assign53120_e68540)) / assign53120_e68541), ((((locals.var_pm__blk1425_dn6 * assign53120_e68540) - (locals.var_pm__blk1425 * (locals.var_pm__blk1425_dn6 + locals.var_dm__blk1424_dn6))) / (assign53120_e68540 * assign53120_e68540)) / assign53120_e68541), ((((locals.var_pm__blk1425_dn7 * assign53120_e68540) - (locals.var_pm__blk1425 * (locals.var_pm__blk1425_dn7 + locals.var_dm__blk1424_dn7))) / (assign53120_e68540 * assign53120_e68540)) / assign53120_e68541), ((((locals.var_pm__blk1425_dn8 * assign53120_e68540) - (locals.var_pm__blk1425 * (locals.var_pm__blk1425_dn8 + locals.var_dm__blk1424_dn8))) / (assign53120_e68540 * assign53120_e68540)) / assign53120_e68541), ((((locals.var_pm__blk1425_dn9 * assign53120_e68540) - (locals.var_pm__blk1425 * (locals.var_pm__blk1425_dn9 + locals.var_dm__blk1424_dn9))) / (assign53120_e68540 * assign53120_e68540)) / assign53120_e68541), );
            locals.var_temp1_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign53130_e68550: f64 = (locals.var_eeffm__blk1443 * locals.var_mue_t);
            let assign53130_e68552: f64 = (assign53130_e68550).powf(locals.var_themu_t);
            let assign53130_e68556: f64 = (0.5 * locals.var_thecs_t);
            let assign53130_e68558: f64 = (assign53130_e68556 * locals.var_temp1);
            let assign53130_e68559: f64 = (assign53130_e68558).exp();
            let assign53130_e68560: f64 = (locals.var_cs_t * assign53130_e68559);
            let assign53130_e68561: f64 = (assign53130_e68552 + assign53130_e68560);
            (locals.var_mutmp__blk1382, locals.var_mutmp__blk1382_dn4, locals.var_mutmp__blk1382_dn6, locals.var_mutmp__blk1382_dn7, locals.var_mutmp__blk1382_dn8, locals.var_mutmp__blk1382_dn9, ) = (assign53130_e68561, (if locals.var_themu_t_dn4 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign53130_e68550).powf(locals.var_themu_t - 1.0) * ((locals.var_eeffm__blk1443_dn4 * locals.var_mue_t) + (locals.var_eeffm__blk1443 * locals.var_mue_t_dn4)))) } } else { (assign53130_e68552 * ((locals.var_themu_t_dn4 * (assign53130_e68550).ln()) + (locals.var_themu_t * (((locals.var_eeffm__blk1443_dn4 * locals.var_mue_t) + (locals.var_eeffm__blk1443 * locals.var_mue_t_dn4)) / assign53130_e68550)))) } + ((locals.var_cs_t_dn4 * assign53130_e68559) + (locals.var_cs_t * (assign53130_e68559 * (((0.5 * locals.var_thecs_t_dn4) * locals.var_temp1) + (assign53130_e68556 * locals.var_temp1_dn4)))))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign53130_e68550).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm__blk1443_dn6 * locals.var_mue_t))) } } else { (assign53130_e68552 * (locals.var_themu_t * ((locals.var_eeffm__blk1443_dn6 * locals.var_mue_t) / assign53130_e68550))) } + (locals.var_cs_t * (assign53130_e68559 * (assign53130_e68556 * locals.var_temp1_dn6)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign53130_e68550).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm__blk1443_dn7 * locals.var_mue_t))) } } else { (assign53130_e68552 * (locals.var_themu_t * ((locals.var_eeffm__blk1443_dn7 * locals.var_mue_t) / assign53130_e68550))) } + (locals.var_cs_t * (assign53130_e68559 * (assign53130_e68556 * locals.var_temp1_dn7)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign53130_e68550).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm__blk1443_dn8 * locals.var_mue_t))) } } else { (assign53130_e68552 * (locals.var_themu_t * ((locals.var_eeffm__blk1443_dn8 * locals.var_mue_t) / assign53130_e68550))) } + (locals.var_cs_t * (assign53130_e68559 * (assign53130_e68556 * locals.var_temp1_dn8)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign53130_e68550).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm__blk1443_dn9 * locals.var_mue_t))) } } else { (assign53130_e68552 * (locals.var_themu_t * ((locals.var_eeffm__blk1443_dn9 * locals.var_mue_t) / assign53130_e68550))) } + (locals.var_cs_t * (assign53130_e68559 * (assign53130_e68556 * locals.var_temp1_dn9)))), );
            locals.var_mutmp__blk1382_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign53140_e68569: f64 = (1.0 + locals.var_mutmp__blk1382);
            let assign53140_e68571: f64 = (assign53140_e68569 + locals.var_gr__blk1380);
            let assign53140_e68573: f64 = (assign53140_e68571 * locals.var_rxcor__blk1374);
            (locals.var_gmob__blk1444, locals.var_gmob__blk1444_dn4, locals.var_gmob__blk1444_dn6, locals.var_gmob__blk1444_dn7, locals.var_gmob__blk1444_dn8, locals.var_gmob__blk1444_dn9, ) = (assign53140_e68573, (((locals.var_mutmp__blk1382_dn4 + locals.var_gr__blk1380_dn4) * locals.var_rxcor__blk1374) + (assign53140_e68571 * locals.var_rxcor__blk1374_dn4)), (((locals.var_mutmp__blk1382_dn6 + locals.var_gr__blk1380_dn6) * locals.var_rxcor__blk1374) + (assign53140_e68571 * locals.var_rxcor__blk1374_dn6)), (((locals.var_mutmp__blk1382_dn7 + locals.var_gr__blk1380_dn7) * locals.var_rxcor__blk1374) + (assign53140_e68571 * locals.var_rxcor__blk1374_dn7)), (((locals.var_mutmp__blk1382_dn8 + locals.var_gr__blk1380_dn8) * locals.var_rxcor__blk1374) + (assign53140_e68571 * locals.var_rxcor__blk1374_dn8)), (((locals.var_mutmp__blk1382_dn9 + locals.var_gr__blk1380_dn9) * locals.var_rxcor__blk1374) + (assign53140_e68571 * locals.var_rxcor__blk1374_dn9)), );
            locals.var_gmob__blk1444_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign53150_e68582: f64 = (locals.var_v_ds - locals.var_dps__blk1414);
            let assign53150_e68584: f64 = (assign53150_e68582 * locals.var_inv_vp);
            let assign53150_e68585: f64 = (1.0 + assign53150_e68584);
            let assign53150_e68589: f64 = (locals.var_vdse__blk1405 - locals.var_dps__blk1414);
            let assign53150_e68591: f64 = (assign53150_e68589 * locals.var_inv_vp);
            let assign53150_e68592: f64 = (1.0 + assign53150_e68591);
            let assign53150_e68593: f64 = (assign53150_e68585 / assign53150_e68592);
            let assign53150_e68594: f64 = (assign53150_e68593).ln();
            (locals.var_s1__blk1445, locals.var_s1__blk1445_dn4, locals.var_s1__blk1445_dn6, locals.var_s1__blk1445_dn7, locals.var_s1__blk1445_dn8, locals.var_s1__blk1445_dn9, ) = (assign53150_e68594, ((((((-locals.var_dps__blk1414_dn4) * locals.var_inv_vp) * assign53150_e68592) - (assign53150_e68585 * ((locals.var_vdse__blk1405_dn4 - locals.var_dps__blk1414_dn4) * locals.var_inv_vp))) / (assign53150_e68592 * assign53150_e68592)) / assign53150_e68593), ((((((-locals.var_dps__blk1414_dn6) * locals.var_inv_vp) * assign53150_e68592) - (assign53150_e68585 * ((locals.var_vdse__blk1405_dn6 - locals.var_dps__blk1414_dn6) * locals.var_inv_vp))) / (assign53150_e68592 * assign53150_e68592)) / assign53150_e68593), ((((((locals.var_v_ds_dn7 - locals.var_dps__blk1414_dn7) * locals.var_inv_vp) * assign53150_e68592) - (assign53150_e68585 * ((locals.var_vdse__blk1405_dn7 - locals.var_dps__blk1414_dn7) * locals.var_inv_vp))) / (assign53150_e68592 * assign53150_e68592)) / assign53150_e68593), ((((((locals.var_v_ds_dn8 - locals.var_dps__blk1414_dn8) * locals.var_inv_vp) * assign53150_e68592) - (assign53150_e68585 * ((locals.var_vdse__blk1405_dn8 - locals.var_dps__blk1414_dn8) * locals.var_inv_vp))) / (assign53150_e68592 * assign53150_e68592)) / assign53150_e68593), ((((((-locals.var_dps__blk1414_dn9) * locals.var_inv_vp) * assign53150_e68592) - (assign53150_e68585 * ((locals.var_vdse__blk1405_dn9 - locals.var_dps__blk1414_dn9) * locals.var_inv_vp))) / (assign53150_e68592 * assign53150_e68592)) / assign53150_e68593), );
            locals.var_s1__blk1445_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign53160_e68602: f64 = (locals.var_qim__blk1438 * locals.var_xitsb__blk1384);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign53160_e68602, ((locals.var_qim__blk1438_dn4 * locals.var_xitsb__blk1384) + (locals.var_qim__blk1438 * locals.var_xitsb__blk1384_dn4)), ((locals.var_qim__blk1438_dn6 * locals.var_xitsb__blk1384) + (locals.var_qim__blk1438 * locals.var_xitsb__blk1384_dn6)), ((locals.var_qim__blk1438_dn7 * locals.var_xitsb__blk1384) + (locals.var_qim__blk1438 * locals.var_xitsb__blk1384_dn7)), ((locals.var_qim__blk1438_dn8 * locals.var_xitsb__blk1384) + (locals.var_qim__blk1438 * locals.var_xitsb__blk1384_dn8)), ((locals.var_qim__blk1438_dn9 * locals.var_xitsb__blk1384) + (locals.var_qim__blk1438 * locals.var_xitsb__blk1384_dn9)), );
            locals.var_temp2_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign53170_e68611: f64 = (locals.var_thesatt_i + locals.var_temp2);
            let assign53170_e68612: f64 = (locals.var_temp2 / assign53170_e68611);
            (locals.var_wsat__blk1385, locals.var_wsat__blk1385_dn4, locals.var_wsat__blk1385_dn6, locals.var_wsat__blk1385_dn7, locals.var_wsat__blk1385_dn8, locals.var_wsat__blk1385_dn9, ) = (assign53170_e68612, (((locals.var_temp2_dn4 * assign53170_e68611) - (locals.var_temp2 * locals.var_temp2_dn4)) / (assign53170_e68611 * assign53170_e68611)), (((locals.var_temp2_dn6 * assign53170_e68611) - (locals.var_temp2 * locals.var_temp2_dn6)) / (assign53170_e68611 * assign53170_e68611)), (((locals.var_temp2_dn7 * assign53170_e68611) - (locals.var_temp2 * locals.var_temp2_dn7)) / (assign53170_e68611 * assign53170_e68611)), (((locals.var_temp2_dn8 * assign53170_e68611) - (locals.var_temp2 * locals.var_temp2_dn8)) / (assign53170_e68611 * assign53170_e68611)), (((locals.var_temp2_dn9 * assign53170_e68611) - (locals.var_temp2 * locals.var_temp2_dn9)) / (assign53170_e68611 * assign53170_e68611)), );
            locals.var_wsat__blk1385_rv = 0.0;
        }

        let assign53180_e68617: f64 = if locals.var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1521 = assign53180_e68617;
        locals.var_guard1521_rv = 0.0;

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1521 != 0.0)) {
            let assign53190_e68627: f64 = (locals.var_thesatg_i * locals.var_wsat__blk1385);
            let assign53190_e68628: f64 = (1.0 - assign53190_e68627);
            let assign53190_e68629: f64 = (1.0 / assign53190_e68628);
            (locals.var_factheta__blk1386, locals.var_factheta__blk1386_dn4, locals.var_factheta__blk1386_dn6, locals.var_factheta__blk1386_dn7, locals.var_factheta__blk1386_dn8, locals.var_factheta__blk1386_dn9, ) = (assign53190_e68629, (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn4)) / (assign53190_e68628 * assign53190_e68628))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn6)) / (assign53190_e68628 * assign53190_e68628))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn7)) / (assign53190_e68628 * assign53190_e68628))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn8)) / (assign53190_e68628 * assign53190_e68628))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1385_dn9)) / (assign53190_e68628 * assign53190_e68628))), );
            locals.var_factheta__blk1386_rv = 0.0;
        }

        if (((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) && (locals.var_guard1521 == 0.0)) {
            let assign53200_e68641: f64 = (locals.var_thesatg_i * locals.var_wsat__blk1385);
            let assign53200_e68642: f64 = (1.0 + assign53200_e68641);
            (locals.var_factheta__blk1386, locals.var_factheta__blk1386_dn4, locals.var_factheta__blk1386_dn6, locals.var_factheta__blk1386_dn7, locals.var_factheta__blk1386_dn8, locals.var_factheta__blk1386_dn9, ) = (assign53200_e68642, (locals.var_thesatg_i * locals.var_wsat__blk1385_dn4), (locals.var_thesatg_i * locals.var_wsat__blk1385_dn6), (locals.var_thesatg_i * locals.var_wsat__blk1385_dn7), (locals.var_thesatg_i * locals.var_wsat__blk1385_dn8), (locals.var_thesatg_i * locals.var_wsat__blk1385_dn9), );
            locals.var_factheta__blk1386_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign53210_e68650: f64 = (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386);
            (locals.var_thesateff__blk1447, locals.var_thesateff__blk1447_dn4, locals.var_thesateff__blk1447_dn6, locals.var_thesateff__blk1447_dn7, locals.var_thesateff__blk1447_dn8, locals.var_thesateff__blk1447_dn9, ) = (assign53210_e68650, ((locals.var_thesatloc__blk1319_dn4 * locals.var_factheta__blk1386) + (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn4)), (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn6), (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn7), (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn8), (locals.var_thesatloc__blk1319 * locals.var_factheta__blk1386_dn9), );
            locals.var_thesateff__blk1447_rv = 0.0;
        }

        if ((locals.var_guard1473 != 0.0) && (locals.var_guard1501 != 0.0)) {
            let assign53220_e68658: f64 = (locals.var_xgm__blk1426 * locals.var_phit1__blk1339);
            (locals.var_voxm__blk1446, locals.var_voxm__blk1446_dn4, locals.var_voxm__blk1446_dn6, locals.var_voxm__blk1446_dn7, locals.var_voxm__blk1446_dn8, locals.var_voxm__blk1446_dn9, ) = (assign53220_e68658, ((locals.var_xgm__blk1426_dn4 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn4)), ((locals.var_xgm__blk1426_dn6 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn6)), ((locals.var_xgm__blk1426_dn7 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn7)), ((locals.var_xgm__blk1426_dn8 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn8)), ((locals.var_xgm__blk1426_dn9 * locals.var_phit1__blk1339) + (locals.var_xgm__blk1426 * locals.var_phit1__blk1339_dn9)), );
            locals.var_voxm__blk1446_rv = 0.0;
        }

        if (locals.var_guard1473 != 0.0) {
            (locals.var_vgb1_ac, locals.var_vgb1_ac_dn4, locals.var_vgb1_ac_dn6, locals.var_vgb1_ac_dn7, locals.var_vgb1_ac_dn8, locals.var_vgb1_ac_dn9, ) = (locals.var_vgb1__blk1321, locals.var_vgb1__blk1321_dn4, locals.var_vgb1__blk1321_dn6, locals.var_vgb1__blk1321_dn7, locals.var_vgb1__blk1321_dn8, locals.var_vgb1__blk1321_dn9, );
            locals.var_vgb1_ac_rv = 0.0;
            (locals.var_phit1_ac, locals.var_phit1_ac_dn4, locals.var_phit1_ac_dn6, locals.var_phit1_ac_dn7, locals.var_phit1_ac_dn8, locals.var_phit1_ac_dn9, ) = (locals.var_phit1__blk1339, locals.var_phit1__blk1339_dn4, locals.var_phit1__blk1339_dn6, locals.var_phit1__blk1339_dn7, locals.var_phit1__blk1339_dn8, locals.var_phit1__blk1339_dn9, );
            locals.var_phit1_ac_rv = 0.0;
            (locals.var_gf_ac, locals.var_gf_ac_dn4, locals.var_gf_ac_dn6, locals.var_gf_ac_dn7, locals.var_gf_ac_dn8, locals.var_gf_ac_dn9, ) = (locals.var_gf__blk1324, locals.var_gf__blk1324_dn4, locals.var_gf__blk1324_dn6, locals.var_gf__blk1324_dn7, locals.var_gf__blk1324_dn8, locals.var_gf__blk1324_dn9, );
            locals.var_gf_ac_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_29(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard1473 != 0.0) {
            (locals.var_xg_ac, locals.var_xg_ac_dn4, locals.var_xg_ac_dn6, locals.var_xg_ac_dn7, locals.var_xg_ac_dn8, locals.var_xg_ac_dn9, ) = (locals.var_xg__blk1343, locals.var_xg__blk1343_dn4, locals.var_xg__blk1343_dn6, locals.var_xg__blk1343_dn7, locals.var_xg__blk1343_dn8, locals.var_xg__blk1343_dn9, );
            locals.var_xg_ac_rv = 0.0;
            (locals.var_xno_s_ac, locals.var_xno_s_ac_dn4, locals.var_xno_s_ac_dn6, locals.var_xno_s_ac_dn7, locals.var_xno_s_ac_dn8, locals.var_xno_s_ac_dn9, ) = (locals.var_xno_s__blk1348, locals.var_xno_s__blk1348_dn4, locals.var_xno_s__blk1348_dn6, locals.var_xno_s__blk1348_dn7, locals.var_xno_s__blk1348_dn8, locals.var_xno_s__blk1348_dn9, );
            locals.var_xno_s_ac_rv = 0.0;
            (locals.var_qbs_ac, locals.var_qbs_ac_dn4, locals.var_qbs_ac_dn6, locals.var_qbs_ac_dn7, locals.var_qbs_ac_dn8, locals.var_qbs_ac_dn9, ) = (locals.var_qbs__blk1377, locals.var_qbs__blk1377_dn4, locals.var_qbs__blk1377_dn6, locals.var_qbs__blk1377_dn7, locals.var_qbs__blk1377_dn8, locals.var_qbs__blk1377_dn9, );
            locals.var_qbs_ac_rv = 0.0;
            (locals.var_dps_ac, locals.var_dps_ac_dn4, locals.var_dps_ac_dn6, locals.var_dps_ac_dn7, locals.var_dps_ac_dn8, locals.var_dps_ac_dn9, ) = (locals.var_dps__blk1414, locals.var_dps__blk1414_dn4, locals.var_dps__blk1414_dn6, locals.var_dps__blk1414_dn7, locals.var_dps__blk1414_dn8, locals.var_dps__blk1414_dn9, );
            locals.var_dps_ac_rv = 0.0;
            (locals.var_qbd_ac, locals.var_qbd_ac_dn4, locals.var_qbd_ac_dn6, locals.var_qbd_ac_dn7, locals.var_qbd_ac_dn8, locals.var_qbd_ac_dn9, ) = (locals.var_qbd__blk1420, locals.var_qbd__blk1420_dn4, locals.var_qbd__blk1420_dn6, locals.var_qbd__blk1420_dn7, locals.var_qbd__blk1420_dn8, locals.var_qbd__blk1420_dn9, );
            locals.var_qbd_ac_rv = 0.0;
            (locals.var_eta_p_ac, locals.var_eta_p_ac_dn4, locals.var_eta_p_ac_dn6, locals.var_eta_p_ac_dn7, locals.var_eta_p_ac_dn8, locals.var_eta_p_ac_dn9, ) = (locals.var_eta_p__blk1427, locals.var_eta_p__blk1427_dn4, locals.var_eta_p__blk1427_dn6, locals.var_eta_p__blk1427_dn7, locals.var_eta_p__blk1427_dn8, locals.var_eta_p__blk1427_dn9, );
            locals.var_eta_p_ac_rv = 0.0;
            (locals.var_alpha_ac, locals.var_alpha_ac_dn4, locals.var_alpha_ac_dn6, locals.var_alpha_ac_dn7, locals.var_alpha_ac_dn8, locals.var_alpha_ac_dn9, ) = (locals.var_alpha__blk1429, locals.var_alpha__blk1429_dn4, locals.var_alpha__blk1429_dn6, locals.var_alpha__blk1429_dn7, locals.var_alpha__blk1429_dn8, locals.var_alpha__blk1429_dn9, );
            locals.var_alpha_ac_rv = 0.0;
            (locals.var_qim_ac, locals.var_qim_ac_dn4, locals.var_qim_ac_dn6, locals.var_qim_ac_dn7, locals.var_qim_ac_dn8, locals.var_qim_ac_dn9, ) = (locals.var_qim__blk1438, locals.var_qim__blk1438_dn4, locals.var_qim__blk1438_dn6, locals.var_qim__blk1438_dn7, locals.var_qim__blk1438_dn8, locals.var_qim__blk1438_dn9, );
            locals.var_qim_ac_rv = 0.0;
            (locals.var_qim1_ac, locals.var_qim1_ac_dn4, locals.var_qim1_ac_dn6, locals.var_qim1_ac_dn7, locals.var_qim1_ac_dn8, locals.var_qim1_ac_dn9, ) = (locals.var_qim1__blk1439, locals.var_qim1__blk1439_dn4, locals.var_qim1__blk1439_dn6, locals.var_qim1__blk1439_dn7, locals.var_qim1__blk1439_dn8, locals.var_qim1__blk1439_dn9, );
            locals.var_qim1_ac_rv = 0.0;
            (locals.var_qeff1_ac, locals.var_qeff1_ac_dn4, locals.var_qeff1_ac_dn6, locals.var_qeff1_ac_dn7, locals.var_qeff1_ac_dn8, locals.var_qeff1_ac_dn9, ) = (locals.var_qeff1__blk1442, locals.var_qeff1__blk1442_dn4, locals.var_qeff1__blk1442_dn6, locals.var_qeff1__blk1442_dn7, locals.var_qeff1__blk1442_dn8, locals.var_qeff1__blk1442_dn9, );
            locals.var_qeff1_ac_rv = 0.0;
            (locals.var_gmob_ac, locals.var_gmob_ac_dn4, locals.var_gmob_ac_dn6, locals.var_gmob_ac_dn7, locals.var_gmob_ac_dn8, locals.var_gmob_ac_dn9, ) = (locals.var_gmob__blk1444, locals.var_gmob__blk1444_dn4, locals.var_gmob__blk1444_dn6, locals.var_gmob__blk1444_dn7, locals.var_gmob__blk1444_dn8, locals.var_gmob__blk1444_dn9, );
            locals.var_gmob_ac_rv = 0.0;
            (locals.var_s1_ac, locals.var_s1_ac_dn4, locals.var_s1_ac_dn6, locals.var_s1_ac_dn7, locals.var_s1_ac_dn8, locals.var_s1_ac_dn9, ) = (locals.var_s1__blk1445, locals.var_s1__blk1445_dn4, locals.var_s1__blk1445_dn6, locals.var_s1__blk1445_dn7, locals.var_s1__blk1445_dn8, locals.var_s1__blk1445_dn9, );
            locals.var_s1_ac_rv = 0.0;
            (locals.var_thesateff_ac, locals.var_thesateff_ac_dn4, locals.var_thesateff_ac_dn6, locals.var_thesateff_ac_dn7, locals.var_thesateff_ac_dn8, locals.var_thesateff_ac_dn9, ) = (locals.var_thesateff__blk1447, locals.var_thesateff__blk1447_dn4, locals.var_thesateff__blk1447_dn6, locals.var_thesateff__blk1447_dn7, locals.var_thesateff__blk1447_dn8, locals.var_thesateff__blk1447_dn9, );
            locals.var_thesateff_ac_rv = 0.0;
            (locals.var_voxm_ac, locals.var_voxm_ac_dn4, locals.var_voxm_ac_dn6, locals.var_voxm_ac_dn7, locals.var_voxm_ac_dn8, locals.var_voxm_ac_dn9, ) = (locals.var_voxm__blk1446, locals.var_voxm__blk1446_dn4, locals.var_voxm__blk1446_dn6, locals.var_voxm__blk1446_dn7, locals.var_voxm__blk1446_dn8, locals.var_voxm__blk1446_dn9, );
            locals.var_voxm_ac_rv = 0.0;
        }

        if (locals.var_guard1473 == 0.0) {
            (locals.var_phib_ac, locals.var_phib_ac_dn4, ) = (locals.var_phib_dc, locals.var_phib_dc_dn4, );
            locals.var_phib_ac_rv = 0.0;
            (locals.var_vgb1_ac, locals.var_vgb1_ac_dn4, locals.var_vgb1_ac_dn6, locals.var_vgb1_ac_dn7, locals.var_vgb1_ac_dn8, locals.var_vgb1_ac_dn9, ) = (locals.var_vgb1_dc, locals.var_vgb1_dc_dn4, locals.var_vgb1_dc_dn6, locals.var_vgb1_dc_dn7, locals.var_vgb1_dc_dn8, locals.var_vgb1_dc_dn9, );
            locals.var_vgb1_ac_rv = 0.0;
            (locals.var_phit1_ac, locals.var_phit1_ac_dn4, locals.var_phit1_ac_dn6, locals.var_phit1_ac_dn7, locals.var_phit1_ac_dn8, locals.var_phit1_ac_dn9, ) = (locals.var_phit1_dc, locals.var_phit1_dc_dn4, locals.var_phit1_dc_dn6, locals.var_phit1_dc_dn7, locals.var_phit1_dc_dn8, locals.var_phit1_dc_dn9, );
            locals.var_phit1_ac_rv = 0.0;
            (locals.var_gf_ac, locals.var_gf_ac_dn4, locals.var_gf_ac_dn6, locals.var_gf_ac_dn7, locals.var_gf_ac_dn8, locals.var_gf_ac_dn9, ) = (locals.var_gf_dc, locals.var_gf_dc_dn4, locals.var_gf_dc_dn6, locals.var_gf_dc_dn7, locals.var_gf_dc_dn8, locals.var_gf_dc_dn9, );
            locals.var_gf_ac_rv = 0.0;
            (locals.var_xg_ac, locals.var_xg_ac_dn4, locals.var_xg_ac_dn6, locals.var_xg_ac_dn7, locals.var_xg_ac_dn8, locals.var_xg_ac_dn9, ) = (locals.var_xg_dc, locals.var_xg_dc_dn4, locals.var_xg_dc_dn6, locals.var_xg_dc_dn7, locals.var_xg_dc_dn8, locals.var_xg_dc_dn9, );
            locals.var_xg_ac_rv = 0.0;
            (locals.var_xno_s_ac, locals.var_xno_s_ac_dn4, locals.var_xno_s_ac_dn6, locals.var_xno_s_ac_dn7, locals.var_xno_s_ac_dn8, locals.var_xno_s_ac_dn9, ) = (locals.var_xno_s_dc, locals.var_xno_s_dc_dn4, locals.var_xno_s_dc_dn6, locals.var_xno_s_dc_dn7, locals.var_xno_s_dc_dn8, locals.var_xno_s_dc_dn9, );
            locals.var_xno_s_ac_rv = 0.0;
            (locals.var_qbs_ac, locals.var_qbs_ac_dn4, locals.var_qbs_ac_dn6, locals.var_qbs_ac_dn7, locals.var_qbs_ac_dn8, locals.var_qbs_ac_dn9, ) = (locals.var_qbs_dc, locals.var_qbs_dc_dn4, locals.var_qbs_dc_dn6, locals.var_qbs_dc_dn7, locals.var_qbs_dc_dn8, locals.var_qbs_dc_dn9, );
            locals.var_qbs_ac_rv = 0.0;
            (locals.var_dps_ac, locals.var_dps_ac_dn4, locals.var_dps_ac_dn6, locals.var_dps_ac_dn7, locals.var_dps_ac_dn8, locals.var_dps_ac_dn9, ) = (locals.var_dps_dc, locals.var_dps_dc_dn4, locals.var_dps_dc_dn6, locals.var_dps_dc_dn7, locals.var_dps_dc_dn8, locals.var_dps_dc_dn9, );
            locals.var_dps_ac_rv = 0.0;
            (locals.var_qbd_ac, locals.var_qbd_ac_dn4, locals.var_qbd_ac_dn6, locals.var_qbd_ac_dn7, locals.var_qbd_ac_dn8, locals.var_qbd_ac_dn9, ) = (locals.var_qbd_dc, locals.var_qbd_dc_dn4, locals.var_qbd_dc_dn6, locals.var_qbd_dc_dn7, locals.var_qbd_dc_dn8, locals.var_qbd_dc_dn9, );
            locals.var_qbd_ac_rv = 0.0;
            (locals.var_eta_p_ac, locals.var_eta_p_ac_dn4, locals.var_eta_p_ac_dn6, locals.var_eta_p_ac_dn7, locals.var_eta_p_ac_dn8, locals.var_eta_p_ac_dn9, ) = (locals.var_eta_p_dc, locals.var_eta_p_dc_dn4, locals.var_eta_p_dc_dn6, locals.var_eta_p_dc_dn7, locals.var_eta_p_dc_dn8, locals.var_eta_p_dc_dn9, );
            locals.var_eta_p_ac_rv = 0.0;
            (locals.var_alpha_ac, locals.var_alpha_ac_dn4, locals.var_alpha_ac_dn6, locals.var_alpha_ac_dn7, locals.var_alpha_ac_dn8, locals.var_alpha_ac_dn9, ) = (locals.var_alpha_dc, locals.var_alpha_dc_dn4, locals.var_alpha_dc_dn6, locals.var_alpha_dc_dn7, locals.var_alpha_dc_dn8, locals.var_alpha_dc_dn9, );
            locals.var_alpha_ac_rv = 0.0;
            (locals.var_qim_ac, locals.var_qim_ac_dn4, locals.var_qim_ac_dn6, locals.var_qim_ac_dn7, locals.var_qim_ac_dn8, locals.var_qim_ac_dn9, ) = (locals.var_qim_dc, locals.var_qim_dc_dn4, locals.var_qim_dc_dn6, locals.var_qim_dc_dn7, locals.var_qim_dc_dn8, locals.var_qim_dc_dn9, );
            locals.var_qim_ac_rv = 0.0;
            (locals.var_qim1_ac, locals.var_qim1_ac_dn4, locals.var_qim1_ac_dn6, locals.var_qim1_ac_dn7, locals.var_qim1_ac_dn8, locals.var_qim1_ac_dn9, ) = (locals.var_qim1_dc, locals.var_qim1_dc_dn4, locals.var_qim1_dc_dn6, locals.var_qim1_dc_dn7, locals.var_qim1_dc_dn8, locals.var_qim1_dc_dn9, );
            locals.var_qim1_ac_rv = 0.0;
            (locals.var_qeff1_ac, locals.var_qeff1_ac_dn4, locals.var_qeff1_ac_dn6, locals.var_qeff1_ac_dn7, locals.var_qeff1_ac_dn8, locals.var_qeff1_ac_dn9, ) = (locals.var_qeff1_dc, locals.var_qeff1_dc_dn4, locals.var_qeff1_dc_dn6, locals.var_qeff1_dc_dn7, locals.var_qeff1_dc_dn8, locals.var_qeff1_dc_dn9, );
            locals.var_qeff1_ac_rv = 0.0;
            (locals.var_gmob_ac, locals.var_gmob_ac_dn4, locals.var_gmob_ac_dn6, locals.var_gmob_ac_dn7, locals.var_gmob_ac_dn8, locals.var_gmob_ac_dn9, ) = (locals.var_gmob_dc, locals.var_gmob_dc_dn4, locals.var_gmob_dc_dn6, locals.var_gmob_dc_dn7, locals.var_gmob_dc_dn8, locals.var_gmob_dc_dn9, );
            locals.var_gmob_ac_rv = 0.0;
            (locals.var_s1_ac, locals.var_s1_ac_dn4, locals.var_s1_ac_dn6, locals.var_s1_ac_dn7, locals.var_s1_ac_dn8, locals.var_s1_ac_dn9, ) = (locals.var_s1_dc, locals.var_s1_dc_dn4, locals.var_s1_dc_dn6, locals.var_s1_dc_dn7, locals.var_s1_dc_dn8, locals.var_s1_dc_dn9, );
            locals.var_s1_ac_rv = 0.0;
            (locals.var_thesateff_ac, locals.var_thesateff_ac_dn4, locals.var_thesateff_ac_dn6, locals.var_thesateff_ac_dn7, locals.var_thesateff_ac_dn8, locals.var_thesateff_ac_dn9, ) = (locals.var_thesateff_dc, locals.var_thesateff_dc_dn4, locals.var_thesateff_dc_dn6, locals.var_thesateff_dc_dn7, locals.var_thesateff_dc_dn8, locals.var_thesateff_dc_dn9, );
            locals.var_thesateff_ac_rv = 0.0;
            (locals.var_voxm_ac, locals.var_voxm_ac_dn4, locals.var_voxm_ac_dn6, locals.var_voxm_ac_dn7, locals.var_voxm_ac_dn8, locals.var_voxm_ac_dn9, ) = (locals.var_voxm_dc, locals.var_voxm_dc_dn4, locals.var_voxm_dc_dn6, locals.var_voxm_dc_dn7, locals.var_voxm_dc_dn8, locals.var_voxm_dc_dn9, );
            locals.var_voxm_ac_rv = 0.0;
        }

        (locals.var_cox_qm, locals.var_cox_qm_dn4, locals.var_cox_qm_dn6, locals.var_cox_qm_dn7, locals.var_cox_qm_dn8, locals.var_cox_qm_dn9, ) = (locals.var_cox_i, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_cox_qm_rv = 0.0;

        let assign53600_e68827: f64 = if locals.var_qq > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1522 = assign53600_e68827;
        locals.var_guard1522_rv = 0.0;

        if (locals.var_guard1522 != 0.0) {
            let assign53610_e68834: f64 = (locals.var_qeff1_ac * locals.var_qeff1_ac);
            let assign53610_e68836: f64 = (assign53610_e68834 + locals.var_qlim2);
            let assign53610_e68838: f64 = (-1.0);
            let assign53610_e68840: f64 = (assign53610_e68838 * 0.16666666666666666);
            let assign53610_e68841: f64 = (assign53610_e68836).powf(assign53610_e68840);
            let assign53610_e68842: f64 = (locals.var_qq * assign53610_e68841);
            let assign53610_e68843: f64 = (1.0 + assign53610_e68842);
            let assign53610_e68844: f64 = (locals.var_cox_i / assign53610_e68843);
            (locals.var_cox_qm, locals.var_cox_qm_dn4, locals.var_cox_qm_dn6, locals.var_cox_qm_dn7, locals.var_cox_qm_dn8, locals.var_cox_qm_dn9, ) = (assign53610_e68844, (-((locals.var_cox_i * (locals.var_qq * if 0.0 == 0.0 && ((assign53610_e68840) as f64).is_finite() && ((assign53610_e68840) as f64).fract() == 0.0 { if assign53610_e68840 == 0.0 { 0.0 } else { (assign53610_e68840 * ((assign53610_e68836).powf(assign53610_e68840 - 1.0) * (((locals.var_qeff1_ac_dn4 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn4)) + locals.var_qlim2_dn4))) } } else { (assign53610_e68841 * (assign53610_e68840 * ((((locals.var_qeff1_ac_dn4 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn4)) + locals.var_qlim2_dn4) / assign53610_e68836))) })) / (assign53610_e68843 * assign53610_e68843))), (-((locals.var_cox_i * (locals.var_qq * if 0.0 == 0.0 && ((assign53610_e68840) as f64).is_finite() && ((assign53610_e68840) as f64).fract() == 0.0 { if assign53610_e68840 == 0.0 { 0.0 } else { (assign53610_e68840 * ((assign53610_e68836).powf(assign53610_e68840 - 1.0) * ((locals.var_qeff1_ac_dn6 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn6)))) } } else { (assign53610_e68841 * (assign53610_e68840 * (((locals.var_qeff1_ac_dn6 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn6)) / assign53610_e68836))) })) / (assign53610_e68843 * assign53610_e68843))), (-((locals.var_cox_i * (locals.var_qq * if 0.0 == 0.0 && ((assign53610_e68840) as f64).is_finite() && ((assign53610_e68840) as f64).fract() == 0.0 { if assign53610_e68840 == 0.0 { 0.0 } else { (assign53610_e68840 * ((assign53610_e68836).powf(assign53610_e68840 - 1.0) * ((locals.var_qeff1_ac_dn7 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn7)))) } } else { (assign53610_e68841 * (assign53610_e68840 * (((locals.var_qeff1_ac_dn7 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn7)) / assign53610_e68836))) })) / (assign53610_e68843 * assign53610_e68843))), (-((locals.var_cox_i * (locals.var_qq * if 0.0 == 0.0 && ((assign53610_e68840) as f64).is_finite() && ((assign53610_e68840) as f64).fract() == 0.0 { if assign53610_e68840 == 0.0 { 0.0 } else { (assign53610_e68840 * ((assign53610_e68836).powf(assign53610_e68840 - 1.0) * ((locals.var_qeff1_ac_dn8 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn8)))) } } else { (assign53610_e68841 * (assign53610_e68840 * (((locals.var_qeff1_ac_dn8 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn8)) / assign53610_e68836))) })) / (assign53610_e68843 * assign53610_e68843))), (-((locals.var_cox_i * (locals.var_qq * if 0.0 == 0.0 && ((assign53610_e68840) as f64).is_finite() && ((assign53610_e68840) as f64).fract() == 0.0 { if assign53610_e68840 == 0.0 { 0.0 } else { (assign53610_e68840 * ((assign53610_e68836).powf(assign53610_e68840 - 1.0) * ((locals.var_qeff1_ac_dn9 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn9)))) } } else { (assign53610_e68841 * (assign53610_e68840 * (((locals.var_qeff1_ac_dn9 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn9)) / assign53610_e68836))) })) / (assign53610_e68843 * assign53610_e68843))), );
            locals.var_cox_qm_rv = 0.0;
        }

        (locals.var_gdl_ac, locals.var_gdl_ac_dn4, locals.var_gdl_ac_dn6, locals.var_gdl_ac_dn7, locals.var_gdl_ac_dn8, locals.var_gdl_ac_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_gdl_ac_rv = 0.0;

        (locals.var_gmob_dl_ac, locals.var_gmob_dl_ac_dn4, locals.var_gmob_dl_ac_dn6, locals.var_gmob_dl_ac_dn7, locals.var_gmob_dl_ac_dn8, locals.var_gmob_dl_ac_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_gmob_dl_ac_rv = 0.0;

        (locals.var_thesat1_ac, locals.var_thesat1_ac_dn4, locals.var_thesat1_ac_dn6, locals.var_thesat1_ac_dn7, locals.var_thesat1_ac_dn8, locals.var_thesat1_ac_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_thesat1_ac_rv = 0.0;

        (locals.var_gvsat_ac, locals.var_gvsat_ac_dn4, locals.var_gvsat_ac_dn6, locals.var_gvsat_ac_dn7, locals.var_gvsat_ac_dn8, locals.var_gvsat_ac_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_gvsat_ac_rv = 0.0;

        (locals.var_h_ac, locals.var_h_ac_dn4, locals.var_h_ac_dn6, locals.var_h_ac_dn7, locals.var_h_ac_dn8, locals.var_h_ac_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_h_ac_rv = 0.0;

        (locals.var_qg_1, locals.var_qg_1_dn4, locals.var_qg_1_dn6, locals.var_qg_1_dn7, locals.var_qg_1_dn8, locals.var_qg_1_dn9, ) = (locals.var_voxm_ac, locals.var_voxm_ac_dn4, locals.var_voxm_ac_dn6, locals.var_voxm_ac_dn7, locals.var_voxm_ac_dn8, locals.var_voxm_ac_dn9, );
        locals.var_qg_1_rv = 0.0;

        (locals.var_qi, locals.var_qi_dn4, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn8, locals.var_qi_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_qi_rv = 0.0;

        (locals.var_qd_1, locals.var_qd_1_dn4, locals.var_qd_1_dn6, locals.var_qd_1_dn7, locals.var_qd_1_dn8, locals.var_qd_1_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_qd_1_rv = 0.0;

        (locals.var_qb_1, locals.var_qb_1_dn4, locals.var_qb_1_dn6, locals.var_qb_1_dn7, locals.var_qb_1_dn8, locals.var_qb_1_dn9, ) = (locals.var_qg_1, locals.var_qg_1_dn4, locals.var_qg_1_dn6, locals.var_qg_1_dn7, locals.var_qg_1_dn8, locals.var_qg_1_dn9, );
        locals.var_qb_1_rv = 0.0;

        let assign53710_e68858: f64 = if locals.var_xg_ac > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1523 = assign53710_e68858;
        locals.var_guard1523_rv = 0.0;

        if (locals.var_guard1523 != 0.0) {
            let assign53720_e68863: f64 = (locals.var_alp1ac_i / locals.var_qim1_ac);
            let assign53720_e68864: f64 = (locals.var_alpac_i + assign53720_e68863);
            let assign53720_e68866: f64 = (assign53720_e68864 * locals.var_qim_ac);
            let assign53720_e68868: f64 = (assign53720_e68866 / locals.var_qim1_ac);
            let assign53720_e68870: f64 = (assign53720_e68868 * locals.var_s1_ac);
            (locals.var_dl__blk1280, locals.var_dl__blk1280_dn4, locals.var_dl__blk1280_dn6, locals.var_dl__blk1280_dn7, locals.var_dl__blk1280_dn8, locals.var_dl__blk1280_dn9, ) = (assign53720_e68870, ((((((((-((locals.var_alp1ac_i * locals.var_qim1_ac_dn4) / (locals.var_qim1_ac * locals.var_qim1_ac))) * locals.var_qim_ac) + (assign53720_e68864 * locals.var_qim_ac_dn4)) * locals.var_qim1_ac) - (assign53720_e68866 * locals.var_qim1_ac_dn4)) / (locals.var_qim1_ac * locals.var_qim1_ac)) * locals.var_s1_ac) + (assign53720_e68868 * locals.var_s1_ac_dn4)), ((((((((-((locals.var_alp1ac_i * locals.var_qim1_ac_dn6) / (locals.var_qim1_ac * locals.var_qim1_ac))) * locals.var_qim_ac) + (assign53720_e68864 * locals.var_qim_ac_dn6)) * locals.var_qim1_ac) - (assign53720_e68866 * locals.var_qim1_ac_dn6)) / (locals.var_qim1_ac * locals.var_qim1_ac)) * locals.var_s1_ac) + (assign53720_e68868 * locals.var_s1_ac_dn6)), ((((((((-((locals.var_alp1ac_i * locals.var_qim1_ac_dn7) / (locals.var_qim1_ac * locals.var_qim1_ac))) * locals.var_qim_ac) + (assign53720_e68864 * locals.var_qim_ac_dn7)) * locals.var_qim1_ac) - (assign53720_e68866 * locals.var_qim1_ac_dn7)) / (locals.var_qim1_ac * locals.var_qim1_ac)) * locals.var_s1_ac) + (assign53720_e68868 * locals.var_s1_ac_dn7)), ((((((((-((locals.var_alp1ac_i * locals.var_qim1_ac_dn8) / (locals.var_qim1_ac * locals.var_qim1_ac))) * locals.var_qim_ac) + (assign53720_e68864 * locals.var_qim_ac_dn8)) * locals.var_qim1_ac) - (assign53720_e68866 * locals.var_qim1_ac_dn8)) / (locals.var_qim1_ac * locals.var_qim1_ac)) * locals.var_s1_ac) + (assign53720_e68868 * locals.var_s1_ac_dn8)), ((((((((-((locals.var_alp1ac_i * locals.var_qim1_ac_dn9) / (locals.var_qim1_ac * locals.var_qim1_ac))) * locals.var_qim_ac) + (assign53720_e68864 * locals.var_qim_ac_dn9)) * locals.var_qim1_ac) - (assign53720_e68866 * locals.var_qim1_ac_dn9)) / (locals.var_qim1_ac * locals.var_qim1_ac)) * locals.var_s1_ac) + (assign53720_e68868 * locals.var_s1_ac_dn9)), );
            locals.var_dl__blk1280_rv = 0.0;
        }

        let assign53730_e68875: f64 = if locals.var_dl__blk1280 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1524 = assign53730_e68875;
        locals.var_guard1524_rv = 0.0;

        if ((locals.var_guard1523 != 0.0) && (locals.var_guard1524 != 0.0)) {
            let assign53740_e68882: f64 = (1.0 + locals.var_dl__blk1280);
            let assign53740_e68885: f64 = (locals.var_dl__blk1280 * locals.var_dl__blk1280);
            let assign53740_e68886: f64 = (assign53740_e68882 + assign53740_e68885);
            let assign53740_e68887: f64 = (1.0 / assign53740_e68886);
            (locals.var_gdl_ac, locals.var_gdl_ac_dn4, locals.var_gdl_ac_dn6, locals.var_gdl_ac_dn7, locals.var_gdl_ac_dn8, locals.var_gdl_ac_dn9, ) = (assign53740_e68887, (-((locals.var_dl__blk1280_dn4 + ((locals.var_dl__blk1280_dn4 * locals.var_dl__blk1280) + (locals.var_dl__blk1280 * locals.var_dl__blk1280_dn4))) / (assign53740_e68886 * assign53740_e68886))), (-((locals.var_dl__blk1280_dn6 + ((locals.var_dl__blk1280_dn6 * locals.var_dl__blk1280) + (locals.var_dl__blk1280 * locals.var_dl__blk1280_dn6))) / (assign53740_e68886 * assign53740_e68886))), (-((locals.var_dl__blk1280_dn7 + ((locals.var_dl__blk1280_dn7 * locals.var_dl__blk1280) + (locals.var_dl__blk1280 * locals.var_dl__blk1280_dn7))) / (assign53740_e68886 * assign53740_e68886))), (-((locals.var_dl__blk1280_dn8 + ((locals.var_dl__blk1280_dn8 * locals.var_dl__blk1280) + (locals.var_dl__blk1280 * locals.var_dl__blk1280_dn8))) / (assign53740_e68886 * assign53740_e68886))), (-((locals.var_dl__blk1280_dn9 + ((locals.var_dl__blk1280_dn9 * locals.var_dl__blk1280) + (locals.var_dl__blk1280 * locals.var_dl__blk1280_dn9))) / (assign53740_e68886 * assign53740_e68886))), );
            locals.var_gdl_ac_rv = 0.0;
        }

        if ((locals.var_guard1523 != 0.0) && (locals.var_guard1524 == 0.0)) {
            let assign53750_e68896: f64 = (1.0 - locals.var_dl__blk1280);
            (locals.var_gdl_ac, locals.var_gdl_ac_dn4, locals.var_gdl_ac_dn6, locals.var_gdl_ac_dn7, locals.var_gdl_ac_dn8, locals.var_gdl_ac_dn9, ) = (assign53750_e68896, (-locals.var_dl__blk1280_dn4), (-locals.var_dl__blk1280_dn6), (-locals.var_dl__blk1280_dn7), (-locals.var_dl__blk1280_dn8), (-locals.var_dl__blk1280_dn9), );
            locals.var_gdl_ac_rv = 0.0;
        }

        if (locals.var_guard1523 != 0.0) {
            let assign53760_e68902: f64 = (locals.var_gmob_ac * locals.var_gdl_ac);
            (locals.var_gmob_dl_ac, locals.var_gmob_dl_ac_dn4, locals.var_gmob_dl_ac_dn6, locals.var_gmob_dl_ac_dn7, locals.var_gmob_dl_ac_dn8, locals.var_gmob_dl_ac_dn9, ) = (assign53760_e68902, ((locals.var_gmob_ac_dn4 * locals.var_gdl_ac) + (locals.var_gmob_ac * locals.var_gdl_ac_dn4)), ((locals.var_gmob_ac_dn6 * locals.var_gdl_ac) + (locals.var_gmob_ac * locals.var_gdl_ac_dn6)), ((locals.var_gmob_ac_dn7 * locals.var_gdl_ac) + (locals.var_gmob_ac * locals.var_gdl_ac_dn7)), ((locals.var_gmob_ac_dn8 * locals.var_gdl_ac) + (locals.var_gmob_ac * locals.var_gdl_ac_dn8)), ((locals.var_gmob_ac_dn9 * locals.var_gdl_ac) + (locals.var_gmob_ac * locals.var_gdl_ac_dn9)), );
            locals.var_gmob_dl_ac_rv = 0.0;
        }

        if (locals.var_guard1523 != 0.0) {
            let assign53770_e68908: f64 = (locals.var_thesateff_ac / locals.var_gmob_dl_ac);
            (locals.var_thesat1_ac, locals.var_thesat1_ac_dn4, locals.var_thesat1_ac_dn6, locals.var_thesat1_ac_dn7, locals.var_thesat1_ac_dn8, locals.var_thesat1_ac_dn9, ) = (assign53770_e68908, (((locals.var_thesateff_ac_dn4 * locals.var_gmob_dl_ac) - (locals.var_thesateff_ac * locals.var_gmob_dl_ac_dn4)) / (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac)), (((locals.var_thesateff_ac_dn6 * locals.var_gmob_dl_ac) - (locals.var_thesateff_ac * locals.var_gmob_dl_ac_dn6)) / (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac)), (((locals.var_thesateff_ac_dn7 * locals.var_gmob_dl_ac) - (locals.var_thesateff_ac * locals.var_gmob_dl_ac_dn7)) / (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac)), (((locals.var_thesateff_ac_dn8 * locals.var_gmob_dl_ac) - (locals.var_thesateff_ac * locals.var_gmob_dl_ac_dn8)) / (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac)), (((locals.var_thesateff_ac_dn9 * locals.var_gmob_dl_ac) - (locals.var_thesateff_ac * locals.var_gmob_dl_ac_dn9)) / (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac)), );
            locals.var_thesat1_ac_rv = 0.0;
        }

        if (locals.var_guard1523 != 0.0) {
            let assign53780_e68914: f64 = (locals.var_thesat1_ac * locals.var_thesat1_ac);
            let assign53780_e68916: f64 = (assign53780_e68914 * locals.var_dps_ac);
            let assign53780_e68918: f64 = (assign53780_e68916 * locals.var_dps_ac);
            (locals.var_zsat__blk1281, locals.var_zsat__blk1281_dn4, locals.var_zsat__blk1281_dn6, locals.var_zsat__blk1281_dn7, locals.var_zsat__blk1281_dn8, locals.var_zsat__blk1281_dn9, ) = (assign53780_e68918, ((((((locals.var_thesat1_ac_dn4 * locals.var_thesat1_ac) + (locals.var_thesat1_ac * locals.var_thesat1_ac_dn4)) * locals.var_dps_ac) + (assign53780_e68914 * locals.var_dps_ac_dn4)) * locals.var_dps_ac) + (assign53780_e68916 * locals.var_dps_ac_dn4)), ((((((locals.var_thesat1_ac_dn6 * locals.var_thesat1_ac) + (locals.var_thesat1_ac * locals.var_thesat1_ac_dn6)) * locals.var_dps_ac) + (assign53780_e68914 * locals.var_dps_ac_dn6)) * locals.var_dps_ac) + (assign53780_e68916 * locals.var_dps_ac_dn6)), ((((((locals.var_thesat1_ac_dn7 * locals.var_thesat1_ac) + (locals.var_thesat1_ac * locals.var_thesat1_ac_dn7)) * locals.var_dps_ac) + (assign53780_e68914 * locals.var_dps_ac_dn7)) * locals.var_dps_ac) + (assign53780_e68916 * locals.var_dps_ac_dn7)), ((((((locals.var_thesat1_ac_dn8 * locals.var_thesat1_ac) + (locals.var_thesat1_ac * locals.var_thesat1_ac_dn8)) * locals.var_dps_ac) + (assign53780_e68914 * locals.var_dps_ac_dn8)) * locals.var_dps_ac) + (assign53780_e68916 * locals.var_dps_ac_dn8)), ((((((locals.var_thesat1_ac_dn9 * locals.var_thesat1_ac) + (locals.var_thesat1_ac * locals.var_thesat1_ac_dn9)) * locals.var_dps_ac) + (assign53780_e68914 * locals.var_dps_ac_dn9)) * locals.var_dps_ac) + (assign53780_e68916 * locals.var_dps_ac_dn9)), );
            locals.var_zsat__blk1281_rv = 0.0;
        }

        let assign53790_e68923: f64 = (-1.0);
        let assign53790_e68924: f64 = if locals.var_chnl_type == assign53790_e68923 { 1.0 } else { 0.0 };
        locals.var_guard1525 = assign53790_e68924;
        locals.var_guard1525_rv = 0.0;

        if ((locals.var_guard1523 != 0.0) && (locals.var_guard1525 != 0.0)) {
            let assign53800_e68932: f64 = (locals.var_thesat1_ac * locals.var_dps_ac);
            let assign53800_e68933: f64 = (1.0 + assign53800_e68932);
            let assign53800_e68934: f64 = (locals.var_zsat__blk1281 / assign53800_e68933);
            (locals.var_zsat__blk1281, locals.var_zsat__blk1281_dn4, locals.var_zsat__blk1281_dn6, locals.var_zsat__blk1281_dn7, locals.var_zsat__blk1281_dn8, locals.var_zsat__blk1281_dn9, ) = (assign53800_e68934, (((locals.var_zsat__blk1281_dn4 * assign53800_e68933) - (locals.var_zsat__blk1281 * ((locals.var_thesat1_ac_dn4 * locals.var_dps_ac) + (locals.var_thesat1_ac * locals.var_dps_ac_dn4)))) / (assign53800_e68933 * assign53800_e68933)), (((locals.var_zsat__blk1281_dn6 * assign53800_e68933) - (locals.var_zsat__blk1281 * ((locals.var_thesat1_ac_dn6 * locals.var_dps_ac) + (locals.var_thesat1_ac * locals.var_dps_ac_dn6)))) / (assign53800_e68933 * assign53800_e68933)), (((locals.var_zsat__blk1281_dn7 * assign53800_e68933) - (locals.var_zsat__blk1281 * ((locals.var_thesat1_ac_dn7 * locals.var_dps_ac) + (locals.var_thesat1_ac * locals.var_dps_ac_dn7)))) / (assign53800_e68933 * assign53800_e68933)), (((locals.var_zsat__blk1281_dn8 * assign53800_e68933) - (locals.var_zsat__blk1281 * ((locals.var_thesat1_ac_dn8 * locals.var_dps_ac) + (locals.var_thesat1_ac * locals.var_dps_ac_dn8)))) / (assign53800_e68933 * assign53800_e68933)), (((locals.var_zsat__blk1281_dn9 * assign53800_e68933) - (locals.var_zsat__blk1281 * ((locals.var_thesat1_ac_dn9 * locals.var_dps_ac) + (locals.var_thesat1_ac * locals.var_dps_ac_dn9)))) / (assign53800_e68933 * assign53800_e68933)), );
            locals.var_zsat__blk1281_rv = 0.0;
        }

        if (locals.var_guard1523 != 0.0) {
            let assign53810_e68944: f64 = (2.0 * locals.var_zsat__blk1281);
            let assign53810_e68945: f64 = (1.0 + assign53810_e68944);
            let assign53810_e68946: f64 = (assign53810_e68945).sqrt();
            let assign53810_e68947: f64 = (1.0 + assign53810_e68946);
            let assign53810_e68948: f64 = (locals.var_gmob_dl_ac * assign53810_e68947);
            let assign53810_e68949: f64 = (0.5 * assign53810_e68948);
            (locals.var_gvsat_ac, locals.var_gvsat_ac_dn4, locals.var_gvsat_ac_dn6, locals.var_gvsat_ac_dn7, locals.var_gvsat_ac_dn8, locals.var_gvsat_ac_dn9, ) = (assign53810_e68949, (0.5 * ((locals.var_gmob_dl_ac_dn4 * assign53810_e68947) + (locals.var_gmob_dl_ac * ((2.0 * locals.var_zsat__blk1281_dn4) / (2.0 * assign53810_e68946))))), (0.5 * ((locals.var_gmob_dl_ac_dn6 * assign53810_e68947) + (locals.var_gmob_dl_ac * ((2.0 * locals.var_zsat__blk1281_dn6) / (2.0 * assign53810_e68946))))), (0.5 * ((locals.var_gmob_dl_ac_dn7 * assign53810_e68947) + (locals.var_gmob_dl_ac * ((2.0 * locals.var_zsat__blk1281_dn7) / (2.0 * assign53810_e68946))))), (0.5 * ((locals.var_gmob_dl_ac_dn8 * assign53810_e68947) + (locals.var_gmob_dl_ac * ((2.0 * locals.var_zsat__blk1281_dn8) / (2.0 * assign53810_e68946))))), (0.5 * ((locals.var_gmob_dl_ac_dn9 * assign53810_e68947) + (locals.var_gmob_dl_ac * ((2.0 * locals.var_zsat__blk1281_dn9) / (2.0 * assign53810_e68946))))), );
            locals.var_gvsat_ac_rv = 0.0;
        }

        if (locals.var_guard1523 != 0.0) {
            let assign53820_e68955: f64 = (locals.var_gmob_dl_ac / locals.var_gvsat_ac);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign53820_e68955, (((locals.var_gmob_dl_ac_dn4 * locals.var_gvsat_ac) - (locals.var_gmob_dl_ac * locals.var_gvsat_ac_dn4)) / (locals.var_gvsat_ac * locals.var_gvsat_ac)), (((locals.var_gmob_dl_ac_dn6 * locals.var_gvsat_ac) - (locals.var_gmob_dl_ac * locals.var_gvsat_ac_dn6)) / (locals.var_gvsat_ac * locals.var_gvsat_ac)), (((locals.var_gmob_dl_ac_dn7 * locals.var_gvsat_ac) - (locals.var_gmob_dl_ac * locals.var_gvsat_ac_dn7)) / (locals.var_gvsat_ac * locals.var_gvsat_ac)), (((locals.var_gmob_dl_ac_dn8 * locals.var_gvsat_ac) - (locals.var_gmob_dl_ac * locals.var_gvsat_ac_dn8)) / (locals.var_gvsat_ac * locals.var_gvsat_ac)), (((locals.var_gmob_dl_ac_dn9 * locals.var_gvsat_ac) - (locals.var_gmob_dl_ac * locals.var_gvsat_ac_dn9)) / (locals.var_gvsat_ac * locals.var_gvsat_ac)), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if (locals.var_guard1523 != 0.0) {
            let assign53830_e68964: f64 = (locals.var_zsat__blk1281 * locals.var_temp__blk949);
            let assign53830_e68966: f64 = (assign53830_e68964 * locals.var_temp__blk949);
            let assign53830_e68967: f64 = (0.5 * assign53830_e68966);
            let assign53830_e68968: f64 = (1.0 + assign53830_e68967);
            let assign53830_e68969: f64 = (locals.var_alpha_ac * assign53830_e68968);
            (locals.var_alpha1__blk1282, locals.var_alpha1__blk1282_dn4, locals.var_alpha1__blk1282_dn6, locals.var_alpha1__blk1282_dn7, locals.var_alpha1__blk1282_dn8, locals.var_alpha1__blk1282_dn9, ) = (assign53830_e68969, ((locals.var_alpha_ac_dn4 * assign53830_e68968) + (locals.var_alpha_ac * (0.5 * ((((locals.var_zsat__blk1281_dn4 * locals.var_temp__blk949) + (locals.var_zsat__blk1281 * locals.var_temp__blk949_dn4)) * locals.var_temp__blk949) + (assign53830_e68964 * locals.var_temp__blk949_dn4))))), ((locals.var_alpha_ac_dn6 * assign53830_e68968) + (locals.var_alpha_ac * (0.5 * ((((locals.var_zsat__blk1281_dn6 * locals.var_temp__blk949) + (locals.var_zsat__blk1281 * locals.var_temp__blk949_dn6)) * locals.var_temp__blk949) + (assign53830_e68964 * locals.var_temp__blk949_dn6))))), ((locals.var_alpha_ac_dn7 * assign53830_e68968) + (locals.var_alpha_ac * (0.5 * ((((locals.var_zsat__blk1281_dn7 * locals.var_temp__blk949) + (locals.var_zsat__blk1281 * locals.var_temp__blk949_dn7)) * locals.var_temp__blk949) + (assign53830_e68964 * locals.var_temp__blk949_dn7))))), ((locals.var_alpha_ac_dn8 * assign53830_e68968) + (locals.var_alpha_ac * (0.5 * ((((locals.var_zsat__blk1281_dn8 * locals.var_temp__blk949) + (locals.var_zsat__blk1281 * locals.var_temp__blk949_dn8)) * locals.var_temp__blk949) + (assign53830_e68964 * locals.var_temp__blk949_dn8))))), ((locals.var_alpha_ac_dn9 * assign53830_e68968) + (locals.var_alpha_ac * (0.5 * ((((locals.var_zsat__blk1281_dn9 * locals.var_temp__blk949) + (locals.var_zsat__blk1281 * locals.var_temp__blk949_dn9)) * locals.var_temp__blk949) + (assign53830_e68964 * locals.var_temp__blk949_dn9))))), );
            locals.var_alpha1__blk1282_rv = 0.0;
        }

        if (locals.var_guard1523 != 0.0) {
            let assign53840_e68975: f64 = (locals.var_temp__blk949 * locals.var_qim1_ac);
            let assign53840_e68977: f64 = (assign53840_e68975 / locals.var_alpha1__blk1282);
            (locals.var_h_ac, locals.var_h_ac_dn4, locals.var_h_ac_dn6, locals.var_h_ac_dn7, locals.var_h_ac_dn8, locals.var_h_ac_dn9, ) = (assign53840_e68977, (((((locals.var_temp__blk949_dn4 * locals.var_qim1_ac) + (locals.var_temp__blk949 * locals.var_qim1_ac_dn4)) * locals.var_alpha1__blk1282) - (assign53840_e68975 * locals.var_alpha1__blk1282_dn4)) / (locals.var_alpha1__blk1282 * locals.var_alpha1__blk1282)), (((((locals.var_temp__blk949_dn6 * locals.var_qim1_ac) + (locals.var_temp__blk949 * locals.var_qim1_ac_dn6)) * locals.var_alpha1__blk1282) - (assign53840_e68975 * locals.var_alpha1__blk1282_dn6)) / (locals.var_alpha1__blk1282 * locals.var_alpha1__blk1282)), (((((locals.var_temp__blk949_dn7 * locals.var_qim1_ac) + (locals.var_temp__blk949 * locals.var_qim1_ac_dn7)) * locals.var_alpha1__blk1282) - (assign53840_e68975 * locals.var_alpha1__blk1282_dn7)) / (locals.var_alpha1__blk1282 * locals.var_alpha1__blk1282)), (((((locals.var_temp__blk949_dn8 * locals.var_qim1_ac) + (locals.var_temp__blk949 * locals.var_qim1_ac_dn8)) * locals.var_alpha1__blk1282) - (assign53840_e68975 * locals.var_alpha1__blk1282_dn8)) / (locals.var_alpha1__blk1282 * locals.var_alpha1__blk1282)), (((((locals.var_temp__blk949_dn9 * locals.var_qim1_ac) + (locals.var_temp__blk949 * locals.var_qim1_ac_dn9)) * locals.var_alpha1__blk1282) - (assign53840_e68975 * locals.var_alpha1__blk1282_dn9)) / (locals.var_alpha1__blk1282 * locals.var_alpha1__blk1282)), );
            locals.var_h_ac_rv = 0.0;
        }

        if (locals.var_guard1523 != 0.0) {
            let assign53850_e68984: f64 = (locals.var_dps_ac / locals.var_h_ac);
            let assign53850_e68985: f64 = (0.5 * assign53850_e68984);
            (locals.var_fj, locals.var_fj_dn4, locals.var_fj_dn6, locals.var_fj_dn7, locals.var_fj_dn8, locals.var_fj_dn9, ) = (assign53850_e68985, (0.5 * (((locals.var_dps_ac_dn4 * locals.var_h_ac) - (locals.var_dps_ac * locals.var_h_ac_dn4)) / (locals.var_h_ac * locals.var_h_ac))), (0.5 * (((locals.var_dps_ac_dn6 * locals.var_h_ac) - (locals.var_dps_ac * locals.var_h_ac_dn6)) / (locals.var_h_ac * locals.var_h_ac))), (0.5 * (((locals.var_dps_ac_dn7 * locals.var_h_ac) - (locals.var_dps_ac * locals.var_h_ac_dn7)) / (locals.var_h_ac * locals.var_h_ac))), (0.5 * (((locals.var_dps_ac_dn8 * locals.var_h_ac) - (locals.var_dps_ac * locals.var_h_ac_dn8)) / (locals.var_h_ac * locals.var_h_ac))), (0.5 * (((locals.var_dps_ac_dn9 * locals.var_h_ac) - (locals.var_dps_ac * locals.var_h_ac_dn9)) / (locals.var_h_ac * locals.var_h_ac))), );
            locals.var_fj_rv = 0.0;
        }

        if (locals.var_guard1523 != 0.0) {
            let assign53860_e68991: f64 = (locals.var_fj * locals.var_fj);
            (locals.var_fj2, locals.var_fj2_dn4, locals.var_fj2_dn6, locals.var_fj2_dn7, locals.var_fj2_dn8, locals.var_fj2_dn9, ) = (assign53860_e68991, ((locals.var_fj_dn4 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn4)), ((locals.var_fj_dn6 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn6)), ((locals.var_fj_dn7 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn7)), ((locals.var_fj_dn8 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn8)), ((locals.var_fj_dn9 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn9)), );
            locals.var_fj2_rv = 0.0;
        }

        if (locals.var_guard1523 != 0.0) {
            let assign53870_e68999: f64 = (locals.var_eta_p_ac * locals.var_dps_ac);
            let assign53870_e69002: f64 = (locals.var_fj * locals.var_gdl_ac);
            let assign53870_e69004: f64 = (assign53870_e69002 * 0.3333333333333333);
            let assign53870_e69006: f64 = (assign53870_e69004 - 1.0);
            let assign53870_e69008: f64 = (assign53870_e69006 + locals.var_gdl_ac);
            let assign53870_e69009: f64 = (assign53870_e68999 * assign53870_e69008);
            let assign53870_e69010: f64 = (0.5 * assign53870_e69009);
            let assign53870_e69011: f64 = (locals.var_voxm_ac + assign53870_e69010);
            (locals.var_qg_1, locals.var_qg_1_dn4, locals.var_qg_1_dn6, locals.var_qg_1_dn7, locals.var_qg_1_dn8, locals.var_qg_1_dn9, ) = (assign53870_e69011, (locals.var_voxm_ac_dn4 + (0.5 * ((((locals.var_eta_p_ac_dn4 * locals.var_dps_ac) + (locals.var_eta_p_ac * locals.var_dps_ac_dn4)) * assign53870_e69008) + (assign53870_e68999 * ((((locals.var_fj_dn4 * locals.var_gdl_ac) + (locals.var_fj * locals.var_gdl_ac_dn4)) * 0.3333333333333333) + locals.var_gdl_ac_dn4))))), (locals.var_voxm_ac_dn6 + (0.5 * ((((locals.var_eta_p_ac_dn6 * locals.var_dps_ac) + (locals.var_eta_p_ac * locals.var_dps_ac_dn6)) * assign53870_e69008) + (assign53870_e68999 * ((((locals.var_fj_dn6 * locals.var_gdl_ac) + (locals.var_fj * locals.var_gdl_ac_dn6)) * 0.3333333333333333) + locals.var_gdl_ac_dn6))))), (locals.var_voxm_ac_dn7 + (0.5 * ((((locals.var_eta_p_ac_dn7 * locals.var_dps_ac) + (locals.var_eta_p_ac * locals.var_dps_ac_dn7)) * assign53870_e69008) + (assign53870_e68999 * ((((locals.var_fj_dn7 * locals.var_gdl_ac) + (locals.var_fj * locals.var_gdl_ac_dn7)) * 0.3333333333333333) + locals.var_gdl_ac_dn7))))), (locals.var_voxm_ac_dn8 + (0.5 * ((((locals.var_eta_p_ac_dn8 * locals.var_dps_ac) + (locals.var_eta_p_ac * locals.var_dps_ac_dn8)) * assign53870_e69008) + (assign53870_e68999 * ((((locals.var_fj_dn8 * locals.var_gdl_ac) + (locals.var_fj * locals.var_gdl_ac_dn8)) * 0.3333333333333333) + locals.var_gdl_ac_dn8))))), (locals.var_voxm_ac_dn9 + (0.5 * ((((locals.var_eta_p_ac_dn9 * locals.var_dps_ac) + (locals.var_eta_p_ac * locals.var_dps_ac_dn9)) * assign53870_e69008) + (assign53870_e68999 * ((((locals.var_fj_dn9 * locals.var_gdl_ac) + (locals.var_fj * locals.var_gdl_ac_dn9)) * 0.3333333333333333) + locals.var_gdl_ac_dn9))))), );
            locals.var_qg_1_rv = 0.0;
        }

        if (locals.var_guard1523 != 0.0) {
            let assign53880_e69017: f64 = (locals.var_alpha_ac * locals.var_dps_ac);
            let assign53880_e69019: f64 = (assign53880_e69017 * 0.16666666666666666);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign53880_e69019, (((locals.var_alpha_ac_dn4 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn4)) * 0.16666666666666666), (((locals.var_alpha_ac_dn6 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn6)) * 0.16666666666666666), (((locals.var_alpha_ac_dn7 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn7)) * 0.16666666666666666), (((locals.var_alpha_ac_dn8 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn8)) * 0.16666666666666666), (((locals.var_alpha_ac_dn9 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn9)) * 0.16666666666666666), );
            locals.var_temp__blk949_rv = 0.0;
        }

        let assign53890_e69024: f64 = if p.p49 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1526 = assign53890_e69024;
        locals.var_guard1526_rv = 0.0;

        if ((locals.var_guard1523 != 0.0) && (locals.var_guard1526 != 0.0)) {
            (locals.var_qclm, locals.var_qclm_dn4, locals.var_qclm_dn6, locals.var_qclm_dn7, locals.var_qclm_dn8, locals.var_qclm_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_qclm_rv = 0.0;
        }

        if ((locals.var_guard1523 != 0.0) && (locals.var_guard1526 != 0.0)) {
            let assign53910_e69036: f64 = (0.5 * locals.var_gdl_ac);
            let assign53910_e69038: f64 = (assign53910_e69036 * locals.var_gdl_ac);
            let assign53910_e69042: f64 = (3.0 * locals.var_temp__blk949);
            let assign53910_e69045: f64 = (2.0 - locals.var_fj);
            let assign53910_e69046: f64 = (assign53910_e69042 * assign53910_e69045);
            let assign53910_e69047: f64 = (locals.var_qim_ac - assign53910_e69046);
            let assign53910_e69048: f64 = (assign53910_e69038 * assign53910_e69047);
            (locals.var_qd_1, locals.var_qd_1_dn4, locals.var_qd_1_dn6, locals.var_qd_1_dn7, locals.var_qd_1_dn8, locals.var_qd_1_dn9, ) = (assign53910_e69048, (((((0.5 * locals.var_gdl_ac_dn4) * locals.var_gdl_ac) + (assign53910_e69036 * locals.var_gdl_ac_dn4)) * assign53910_e69047) + (assign53910_e69038 * (locals.var_qim_ac_dn4 - (((3.0 * locals.var_temp__blk949_dn4) * assign53910_e69045) + (assign53910_e69042 * (-locals.var_fj_dn4)))))), (((((0.5 * locals.var_gdl_ac_dn6) * locals.var_gdl_ac) + (assign53910_e69036 * locals.var_gdl_ac_dn6)) * assign53910_e69047) + (assign53910_e69038 * (locals.var_qim_ac_dn6 - (((3.0 * locals.var_temp__blk949_dn6) * assign53910_e69045) + (assign53910_e69042 * (-locals.var_fj_dn6)))))), (((((0.5 * locals.var_gdl_ac_dn7) * locals.var_gdl_ac) + (assign53910_e69036 * locals.var_gdl_ac_dn7)) * assign53910_e69047) + (assign53910_e69038 * (locals.var_qim_ac_dn7 - (((3.0 * locals.var_temp__blk949_dn7) * assign53910_e69045) + (assign53910_e69042 * (-locals.var_fj_dn7)))))), (((((0.5 * locals.var_gdl_ac_dn8) * locals.var_gdl_ac) + (assign53910_e69036 * locals.var_gdl_ac_dn8)) * assign53910_e69047) + (assign53910_e69038 * (locals.var_qim_ac_dn8 - (((3.0 * locals.var_temp__blk949_dn8) * assign53910_e69045) + (assign53910_e69042 * (-locals.var_fj_dn8)))))), (((((0.5 * locals.var_gdl_ac_dn9) * locals.var_gdl_ac) + (assign53910_e69036 * locals.var_gdl_ac_dn9)) * assign53910_e69047) + (assign53910_e69038 * (locals.var_qim_ac_dn9 - (((3.0 * locals.var_temp__blk949_dn9) * assign53910_e69045) + (assign53910_e69042 * (-locals.var_fj_dn9)))))), );
            locals.var_qd_1_rv = 0.0;
        }

        if ((locals.var_guard1523 != 0.0) && (locals.var_guard1526 == 0.0)) {
            let assign53920_e69057: f64 = (1.0 - locals.var_gdl_ac);
            let assign53920_e69062: f64 = (locals.var_alpha_ac * locals.var_dps_ac);
            let assign53920_e69063: f64 = (0.5 * assign53920_e69062);
            let assign53920_e69064: f64 = (locals.var_qim_ac - assign53920_e69063);
            let assign53920_e69065: f64 = (assign53920_e69057 * assign53920_e69064);
            (locals.var_qclm, locals.var_qclm_dn4, locals.var_qclm_dn6, locals.var_qclm_dn7, locals.var_qclm_dn8, locals.var_qclm_dn9, ) = (assign53920_e69065, (((-locals.var_gdl_ac_dn4) * assign53920_e69064) + (assign53920_e69057 * (locals.var_qim_ac_dn4 - (0.5 * ((locals.var_alpha_ac_dn4 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn4)))))), (((-locals.var_gdl_ac_dn6) * assign53920_e69064) + (assign53920_e69057 * (locals.var_qim_ac_dn6 - (0.5 * ((locals.var_alpha_ac_dn6 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn6)))))), (((-locals.var_gdl_ac_dn7) * assign53920_e69064) + (assign53920_e69057 * (locals.var_qim_ac_dn7 - (0.5 * ((locals.var_alpha_ac_dn7 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn7)))))), (((-locals.var_gdl_ac_dn8) * assign53920_e69064) + (assign53920_e69057 * (locals.var_qim_ac_dn8 - (0.5 * ((locals.var_alpha_ac_dn8 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn8)))))), (((-locals.var_gdl_ac_dn9) * assign53920_e69064) + (assign53920_e69057 * (locals.var_qim_ac_dn9 - (0.5 * ((locals.var_alpha_ac_dn9 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn9)))))), );
            locals.var_qclm_rv = 0.0;
        }

        if ((locals.var_guard1523 != 0.0) && (locals.var_guard1526 == 0.0)) {
            let assign53930_e69075: f64 = (locals.var_gdl_ac * locals.var_gdl_ac);
            let assign53930_e69080: f64 = (1.0 - locals.var_fj);
            let assign53930_e69083: f64 = (0.2 * locals.var_fj2);
            let assign53930_e69084: f64 = (assign53930_e69080 - assign53930_e69083);
            let assign53930_e69085: f64 = (locals.var_temp__blk949 * assign53930_e69084);
            let assign53930_e69086: f64 = (locals.var_qim_ac - assign53930_e69085);
            let assign53930_e69087: f64 = (assign53930_e69075 * assign53930_e69086);
            let assign53930_e69091: f64 = (1.0 + locals.var_gdl_ac);
            let assign53930_e69092: f64 = (locals.var_qclm * assign53930_e69091);
            let assign53930_e69093: f64 = (assign53930_e69087 + assign53930_e69092);
            let assign53930_e69094: f64 = (0.5 * assign53930_e69093);
            (locals.var_qd_1, locals.var_qd_1_dn4, locals.var_qd_1_dn6, locals.var_qd_1_dn7, locals.var_qd_1_dn8, locals.var_qd_1_dn9, ) = (assign53930_e69094, (0.5 * (((((locals.var_gdl_ac_dn4 * locals.var_gdl_ac) + (locals.var_gdl_ac * locals.var_gdl_ac_dn4)) * assign53930_e69086) + (assign53930_e69075 * (locals.var_qim_ac_dn4 - ((locals.var_temp__blk949_dn4 * assign53930_e69084) + (locals.var_temp__blk949 * ((-locals.var_fj_dn4) - (0.2 * locals.var_fj2_dn4))))))) + ((locals.var_qclm_dn4 * assign53930_e69091) + (locals.var_qclm * locals.var_gdl_ac_dn4)))), (0.5 * (((((locals.var_gdl_ac_dn6 * locals.var_gdl_ac) + (locals.var_gdl_ac * locals.var_gdl_ac_dn6)) * assign53930_e69086) + (assign53930_e69075 * (locals.var_qim_ac_dn6 - ((locals.var_temp__blk949_dn6 * assign53930_e69084) + (locals.var_temp__blk949 * ((-locals.var_fj_dn6) - (0.2 * locals.var_fj2_dn6))))))) + ((locals.var_qclm_dn6 * assign53930_e69091) + (locals.var_qclm * locals.var_gdl_ac_dn6)))), (0.5 * (((((locals.var_gdl_ac_dn7 * locals.var_gdl_ac) + (locals.var_gdl_ac * locals.var_gdl_ac_dn7)) * assign53930_e69086) + (assign53930_e69075 * (locals.var_qim_ac_dn7 - ((locals.var_temp__blk949_dn7 * assign53930_e69084) + (locals.var_temp__blk949 * ((-locals.var_fj_dn7) - (0.2 * locals.var_fj2_dn7))))))) + ((locals.var_qclm_dn7 * assign53930_e69091) + (locals.var_qclm * locals.var_gdl_ac_dn7)))), (0.5 * (((((locals.var_gdl_ac_dn8 * locals.var_gdl_ac) + (locals.var_gdl_ac * locals.var_gdl_ac_dn8)) * assign53930_e69086) + (assign53930_e69075 * (locals.var_qim_ac_dn8 - ((locals.var_temp__blk949_dn8 * assign53930_e69084) + (locals.var_temp__blk949 * ((-locals.var_fj_dn8) - (0.2 * locals.var_fj2_dn8))))))) + ((locals.var_qclm_dn8 * assign53930_e69091) + (locals.var_qclm * locals.var_gdl_ac_dn8)))), (0.5 * (((((locals.var_gdl_ac_dn9 * locals.var_gdl_ac) + (locals.var_gdl_ac * locals.var_gdl_ac_dn9)) * assign53930_e69086) + (assign53930_e69075 * (locals.var_qim_ac_dn9 - ((locals.var_temp__blk949_dn9 * assign53930_e69084) + (locals.var_temp__blk949 * ((-locals.var_fj_dn9) - (0.2 * locals.var_fj2_dn9))))))) + ((locals.var_qclm_dn9 * assign53930_e69091) + (locals.var_qclm * locals.var_gdl_ac_dn9)))), );
            locals.var_qd_1_rv = 0.0;
        }

        if (locals.var_guard1523 != 0.0) {
            let assign53940_e69102: f64 = (locals.var_temp__blk949 * locals.var_fj);
            let assign53940_e69103: f64 = (locals.var_qim_ac + assign53940_e69102);
            let assign53940_e69104: f64 = (locals.var_gdl_ac * assign53940_e69103);
            let assign53940_e69106: f64 = (assign53940_e69104 + locals.var_qclm);
            (locals.var_qi, locals.var_qi_dn4, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn8, locals.var_qi_dn9, ) = (assign53940_e69106, (((locals.var_gdl_ac_dn4 * assign53940_e69103) + (locals.var_gdl_ac * (locals.var_qim_ac_dn4 + ((locals.var_temp__blk949_dn4 * locals.var_fj) + (locals.var_temp__blk949 * locals.var_fj_dn4))))) + locals.var_qclm_dn4), (((locals.var_gdl_ac_dn6 * assign53940_e69103) + (locals.var_gdl_ac * (locals.var_qim_ac_dn6 + ((locals.var_temp__blk949_dn6 * locals.var_fj) + (locals.var_temp__blk949 * locals.var_fj_dn6))))) + locals.var_qclm_dn6), (((locals.var_gdl_ac_dn7 * assign53940_e69103) + (locals.var_gdl_ac * (locals.var_qim_ac_dn7 + ((locals.var_temp__blk949_dn7 * locals.var_fj) + (locals.var_temp__blk949 * locals.var_fj_dn7))))) + locals.var_qclm_dn7), (((locals.var_gdl_ac_dn8 * assign53940_e69103) + (locals.var_gdl_ac * (locals.var_qim_ac_dn8 + ((locals.var_temp__blk949_dn8 * locals.var_fj) + (locals.var_temp__blk949 * locals.var_fj_dn8))))) + locals.var_qclm_dn8), (((locals.var_gdl_ac_dn9 * assign53940_e69103) + (locals.var_gdl_ac * (locals.var_qim_ac_dn9 + ((locals.var_temp__blk949_dn9 * locals.var_fj) + (locals.var_temp__blk949 * locals.var_fj_dn9))))) + locals.var_qclm_dn9), );
            locals.var_qi_rv = 0.0;
        }

        if (locals.var_guard1523 != 0.0) {
            let assign53950_e69112: f64 = (locals.var_qg_1 - locals.var_qi);
            (locals.var_qb_1, locals.var_qb_1_dn4, locals.var_qb_1_dn6, locals.var_qb_1_dn7, locals.var_qb_1_dn8, locals.var_qb_1_dn9, ) = (assign53950_e69112, (locals.var_qg_1_dn4 - locals.var_qi_dn4), (locals.var_qg_1_dn6 - locals.var_qi_dn6), (locals.var_qg_1_dn7 - locals.var_qi_dn7), (locals.var_qg_1_dn8 - locals.var_qi_dn8), (locals.var_qg_1_dn9 - locals.var_qi_dn9), );
            locals.var_qb_1_rv = 0.0;
        }

        let assign53960_e69117: f64 = (locals.var_qg_1 * locals.var_cox_qm);
        (locals.var_qg, locals.var_qg_dn4, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, ) = (assign53960_e69117, ((locals.var_qg_1_dn4 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn4)), ((locals.var_qg_1_dn6 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn6)), ((locals.var_qg_1_dn7 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn7)), ((locals.var_qg_1_dn8 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn8)), ((locals.var_qg_1_dn9 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn9)), );
        locals.var_qg_rv = 0.0;

        let assign53970_e69119: f64 = (-locals.var_qd_1);
        let assign53970_e69121: f64 = (assign53970_e69119 * locals.var_cox_qm);
        (locals.var_qd, locals.var_qd_dn4, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, ) = (assign53970_e69121, (((-locals.var_qd_1_dn4) * locals.var_cox_qm) + (assign53970_e69119 * locals.var_cox_qm_dn4)), (((-locals.var_qd_1_dn6) * locals.var_cox_qm) + (assign53970_e69119 * locals.var_cox_qm_dn6)), (((-locals.var_qd_1_dn7) * locals.var_cox_qm) + (assign53970_e69119 * locals.var_cox_qm_dn7)), (((-locals.var_qd_1_dn8) * locals.var_cox_qm) + (assign53970_e69119 * locals.var_cox_qm_dn8)), (((-locals.var_qd_1_dn9) * locals.var_cox_qm) + (assign53970_e69119 * locals.var_cox_qm_dn9)), );
        locals.var_qd_rv = 0.0;

        let assign53980_e69123: f64 = (-locals.var_qb_1);
        let assign53980_e69125: f64 = (assign53980_e69123 * locals.var_cox_qm);
        (locals.var_qb, locals.var_qb_dn4, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, ) = (assign53980_e69125, (((-locals.var_qb_1_dn4) * locals.var_cox_qm) + (assign53980_e69123 * locals.var_cox_qm_dn4)), (((-locals.var_qb_1_dn6) * locals.var_cox_qm) + (assign53980_e69123 * locals.var_cox_qm_dn6)), (((-locals.var_qb_1_dn7) * locals.var_cox_qm) + (assign53980_e69123 * locals.var_cox_qm_dn7)), (((-locals.var_qb_1_dn8) * locals.var_cox_qm) + (assign53980_e69123 * locals.var_cox_qm_dn8)), (((-locals.var_qb_1_dn9) * locals.var_cox_qm) + (assign53980_e69123 * locals.var_cox_qm_dn9)), );
        locals.var_qb_rv = 0.0;

        (locals.var_qsinr, locals.var_qsinr_dn4, locals.var_qsinr_dn6, locals.var_qsinr_dn7, locals.var_qsinr_dn8, locals.var_qsinr_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_qsinr_rv = 0.0;

        (locals.var_qdinr, locals.var_qdinr_dn4, locals.var_qdinr_dn6, locals.var_qdinr_dn7, locals.var_qdinr_dn8, locals.var_qdinr_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_qdinr_rv = 0.0;

        (locals.var_qginr, locals.var_qginr_dn4, locals.var_qginr_dn6, locals.var_qginr_dn7, locals.var_qginr_dn8, locals.var_qginr_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_qginr_rv = 0.0;

        let assign54020_e69135: f64 = if ((locals.var_cinr_i > 0.0) || (locals.var_cinrd_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1527 = assign54020_e69135;
        locals.var_guard1527_rv = 0.0;

        if (locals.var_guard1527 != 0.0) {
            (locals.var_finracc, locals.var_finracc_dn4, locals.var_finracc_dn6, locals.var_finracc_dn7, locals.var_finracc_dn8, locals.var_finracc_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_finracc_rv = 0.0;
            (locals.var_dvinracc, locals.var_dvinracc_dn4, locals.var_dvinracc_dn6, locals.var_dvinracc_dn7, locals.var_dvinracc_dn8, locals.var_dvinracc_dn9, ) = (locals.var_vgb1_ac, locals.var_vgb1_ac_dn4, locals.var_vgb1_ac_dn6, locals.var_vgb1_ac_dn7, locals.var_vgb1_ac_dn8, locals.var_vgb1_ac_dn9, );
            locals.var_dvinracc_rv = 0.0;
        }

        let assign54050_e69146: f64 = if locals.var_fcinracc_i > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1528 = assign54050_e69146;
        locals.var_guard1528_rv = 0.0;

        if ((locals.var_guard1527 != 0.0) && (locals.var_guard1528 != 0.0)) {
            let assign54060_e69152: f64 = (locals.var_vgb1_ac - locals.var_dvfbinr_i);
            let assign54060_e69154: f64 = (assign54060_e69152 + locals.var_vinr_max);
            (locals.var_vginr, locals.var_vginr_dn4, locals.var_vginr_dn6, locals.var_vginr_dn7, locals.var_vginr_dn8, locals.var_vginr_dn9, ) = (assign54060_e69154, locals.var_vgb1_ac_dn4, locals.var_vgb1_ac_dn6, locals.var_vgb1_ac_dn7, locals.var_vgb1_ac_dn8, locals.var_vgb1_ac_dn9, );
            locals.var_vginr_rv = 0.0;
        }

        if ((locals.var_guard1527 != 0.0) && (locals.var_guard1528 != 0.0)) {
            let assign54070_e69163: f64 = (locals.var_vginr + locals.var_vinr_max);
            let assign54070_e69166: f64 = (locals.var_vginr - locals.var_vinr_max);
            let assign54070_e69169: f64 = (locals.var_vginr - locals.var_vinr_max);
            let assign54070_e69170: f64 = (assign54070_e69166 * assign54070_e69169);
            let assign54070_e69172: f64 = (assign54070_e69170 + locals.var_ainr);
            let assign54070_e69173: f64 = (assign54070_e69172).sqrt();
            let assign54070_e69174: f64 = (assign54070_e69163 + assign54070_e69173);
            let assign54070_e69175: f64 = (0.5 * assign54070_e69174);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign54070_e69175, (0.5 * (locals.var_vginr_dn4 + (((locals.var_vginr_dn4 * assign54070_e69169) + (assign54070_e69166 * locals.var_vginr_dn4)) / (2.0 * assign54070_e69173)))), (0.5 * (locals.var_vginr_dn6 + (((locals.var_vginr_dn6 * assign54070_e69169) + (assign54070_e69166 * locals.var_vginr_dn6)) / (2.0 * assign54070_e69173)))), (0.5 * (locals.var_vginr_dn7 + (((locals.var_vginr_dn7 * assign54070_e69169) + (assign54070_e69166 * locals.var_vginr_dn7)) / (2.0 * assign54070_e69173)))), (0.5 * (locals.var_vginr_dn8 + (((locals.var_vginr_dn8 * assign54070_e69169) + (assign54070_e69166 * locals.var_vginr_dn8)) / (2.0 * assign54070_e69173)))), (0.5 * (locals.var_vginr_dn9 + (((locals.var_vginr_dn9 * assign54070_e69169) + (assign54070_e69166 * locals.var_vginr_dn9)) / (2.0 * assign54070_e69173)))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((locals.var_guard1527 != 0.0) && (locals.var_guard1528 != 0.0)) {
            let assign54080_e69184: f64 = (2.0 * locals.var_temp__blk949);
            let assign54080_e69186: f64 = (assign54080_e69184 - locals.var_vinr_max);
            let assign54080_e69188: f64 = (assign54080_e69186 - locals.var_vginr);
            let assign54080_e69189: f64 = (locals.var_temp__blk949 * assign54080_e69188);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign54080_e69189, ((locals.var_temp__blk949_dn4 * assign54080_e69188) + (locals.var_temp__blk949 * ((2.0 * locals.var_temp__blk949_dn4) - locals.var_vginr_dn4))), ((locals.var_temp__blk949_dn6 * assign54080_e69188) + (locals.var_temp__blk949 * ((2.0 * locals.var_temp__blk949_dn6) - locals.var_vginr_dn6))), ((locals.var_temp__blk949_dn7 * assign54080_e69188) + (locals.var_temp__blk949 * ((2.0 * locals.var_temp__blk949_dn7) - locals.var_vginr_dn7))), ((locals.var_temp__blk949_dn8 * assign54080_e69188) + (locals.var_temp__blk949 * ((2.0 * locals.var_temp__blk949_dn8) - locals.var_vginr_dn8))), ((locals.var_temp__blk949_dn9 * assign54080_e69188) + (locals.var_temp__blk949 * ((2.0 * locals.var_temp__blk949_dn9) - locals.var_vginr_dn9))), );
            locals.var_temp1_rv = 0.0;
        }

        if ((locals.var_guard1527 != 0.0) && (locals.var_guard1528 != 0.0)) {
            let assign54090_e69197: f64 = (locals.var_vinr_max / locals.var_temp__blk949);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign54090_e69197, (-((locals.var_vinr_max * locals.var_temp__blk949_dn4) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (-((locals.var_vinr_max * locals.var_temp__blk949_dn6) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (-((locals.var_vinr_max * locals.var_temp__blk949_dn7) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (-((locals.var_vinr_max * locals.var_temp__blk949_dn8) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (-((locals.var_vinr_max * locals.var_temp__blk949_dn9) / (locals.var_temp__blk949 * locals.var_temp__blk949))), );
            locals.var_temp2_rv = 0.0;
        }

        if ((locals.var_guard1527 != 0.0) && (locals.var_guard1528 != 0.0)) {
            let assign54100_e69205: f64 = (locals.var_vginr * locals.var_temp2);
            (locals.var_vginreff, locals.var_vginreff_dn4, locals.var_vginreff_dn6, locals.var_vginreff_dn7, locals.var_vginreff_dn8, locals.var_vginreff_dn9, ) = (assign54100_e69205, ((locals.var_vginr_dn4 * locals.var_temp2) + (locals.var_vginr * locals.var_temp2_dn4)), ((locals.var_vginr_dn6 * locals.var_temp2) + (locals.var_vginr * locals.var_temp2_dn6)), ((locals.var_vginr_dn7 * locals.var_temp2) + (locals.var_vginr * locals.var_temp2_dn7)), ((locals.var_vginr_dn8 * locals.var_temp2) + (locals.var_vginr * locals.var_temp2_dn8)), ((locals.var_vginr_dn9 * locals.var_temp2) + (locals.var_vginr * locals.var_temp2_dn9)), );
            locals.var_vginreff_rv = 0.0;
        }

        if ((locals.var_guard1527 != 0.0) && (locals.var_guard1528 != 0.0)) {
            let assign54110_e69214: f64 = (locals.var_vginreff * locals.var_fcinracc_i);
            let assign54110_e69215: f64 = (1.0 - assign54110_e69214);
            let assign54110_e69216: f64 = (assign54110_e69215).sqrt();
            (locals.var_fqinr, locals.var_fqinr_dn4, locals.var_fqinr_dn6, locals.var_fqinr_dn7, locals.var_fqinr_dn8, locals.var_fqinr_dn9, ) = (assign54110_e69216, ((-(locals.var_vginreff_dn4 * locals.var_fcinracc_i)) / (2.0 * assign54110_e69216)), ((-(locals.var_vginreff_dn6 * locals.var_fcinracc_i)) / (2.0 * assign54110_e69216)), ((-(locals.var_vginreff_dn7 * locals.var_fcinracc_i)) / (2.0 * assign54110_e69216)), ((-(locals.var_vginreff_dn8 * locals.var_fcinracc_i)) / (2.0 * assign54110_e69216)), ((-(locals.var_vginreff_dn9 * locals.var_fcinracc_i)) / (2.0 * assign54110_e69216)), );
            locals.var_fqinr_rv = 0.0;
        }

        if ((locals.var_guard1527 != 0.0) && (locals.var_guard1528 != 0.0)) {
            let assign54120_e69224: f64 = (1.0 - locals.var_fqinr);
            let assign54120_e69226: f64 = (assign54120_e69224 / locals.var_fcinracc_i);
            let assign54120_e69228: f64 = (assign54120_e69226 + locals.var_vginr);
            let assign54120_e69230: f64 = (assign54120_e69228 - locals.var_vginreff);
            (locals.var_dvinracc, locals.var_dvinracc_dn4, locals.var_dvinracc_dn6, locals.var_dvinracc_dn7, locals.var_dvinracc_dn8, locals.var_dvinracc_dn9, ) = (assign54120_e69230, ((((-locals.var_fqinr_dn4) / locals.var_fcinracc_i) + locals.var_vginr_dn4) - locals.var_vginreff_dn4), ((((-locals.var_fqinr_dn6) / locals.var_fcinracc_i) + locals.var_vginr_dn6) - locals.var_vginreff_dn6), ((((-locals.var_fqinr_dn7) / locals.var_fcinracc_i) + locals.var_vginr_dn7) - locals.var_vginreff_dn7), ((((-locals.var_fqinr_dn8) / locals.var_fcinracc_i) + locals.var_vginr_dn8) - locals.var_vginreff_dn8), ((((-locals.var_fqinr_dn9) / locals.var_fcinracc_i) + locals.var_vginr_dn9) - locals.var_vginreff_dn9), );
            locals.var_dvinracc_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_30(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard1527 != 0.0) && (locals.var_guard1528 != 0.0)) {
            let assign54130_e69238: f64 = (0.5 / locals.var_fqinr);
            let assign54130_e69240: f64 = (assign54130_e69238 - 1.0);
            let assign54130_e69245: f64 = (locals.var_vinr_max - locals.var_temp__blk949);
            let assign54130_e69246: f64 = (locals.var_vginr * assign54130_e69245);
            let assign54130_e69247: f64 = (locals.var_temp1 + assign54130_e69246);
            let assign54130_e69248: f64 = (assign54130_e69240 * assign54130_e69247);
            let assign54130_e69250: f64 = (assign54130_e69248 * locals.var_temp2);
            let assign54130_e69252: f64 = (assign54130_e69250 / locals.var_temp1);
            let assign54130_e69254: f64 = (assign54130_e69252 + 1.0);
            (locals.var_finracc, locals.var_finracc_dn4, locals.var_finracc_dn6, locals.var_finracc_dn7, locals.var_finracc_dn8, locals.var_finracc_dn9, ) = (assign54130_e69254, ((((((((-((0.5 * locals.var_fqinr_dn4) / (locals.var_fqinr * locals.var_fqinr))) * assign54130_e69247) + (assign54130_e69240 * (locals.var_temp1_dn4 + ((locals.var_vginr_dn4 * assign54130_e69245) + (locals.var_vginr * (-locals.var_temp__blk949_dn4)))))) * locals.var_temp2) + (assign54130_e69248 * locals.var_temp2_dn4)) * locals.var_temp1) - (assign54130_e69250 * locals.var_temp1_dn4)) / (locals.var_temp1 * locals.var_temp1)), ((((((((-((0.5 * locals.var_fqinr_dn6) / (locals.var_fqinr * locals.var_fqinr))) * assign54130_e69247) + (assign54130_e69240 * (locals.var_temp1_dn6 + ((locals.var_vginr_dn6 * assign54130_e69245) + (locals.var_vginr * (-locals.var_temp__blk949_dn6)))))) * locals.var_temp2) + (assign54130_e69248 * locals.var_temp2_dn6)) * locals.var_temp1) - (assign54130_e69250 * locals.var_temp1_dn6)) / (locals.var_temp1 * locals.var_temp1)), ((((((((-((0.5 * locals.var_fqinr_dn7) / (locals.var_fqinr * locals.var_fqinr))) * assign54130_e69247) + (assign54130_e69240 * (locals.var_temp1_dn7 + ((locals.var_vginr_dn7 * assign54130_e69245) + (locals.var_vginr * (-locals.var_temp__blk949_dn7)))))) * locals.var_temp2) + (assign54130_e69248 * locals.var_temp2_dn7)) * locals.var_temp1) - (assign54130_e69250 * locals.var_temp1_dn7)) / (locals.var_temp1 * locals.var_temp1)), ((((((((-((0.5 * locals.var_fqinr_dn8) / (locals.var_fqinr * locals.var_fqinr))) * assign54130_e69247) + (assign54130_e69240 * (locals.var_temp1_dn8 + ((locals.var_vginr_dn8 * assign54130_e69245) + (locals.var_vginr * (-locals.var_temp__blk949_dn8)))))) * locals.var_temp2) + (assign54130_e69248 * locals.var_temp2_dn8)) * locals.var_temp1) - (assign54130_e69250 * locals.var_temp1_dn8)) / (locals.var_temp1 * locals.var_temp1)), ((((((((-((0.5 * locals.var_fqinr_dn9) / (locals.var_fqinr * locals.var_fqinr))) * assign54130_e69247) + (assign54130_e69240 * (locals.var_temp1_dn9 + ((locals.var_vginr_dn9 * assign54130_e69245) + (locals.var_vginr * (-locals.var_temp__blk949_dn9)))))) * locals.var_temp2) + (assign54130_e69248 * locals.var_temp2_dn9)) * locals.var_temp1) - (assign54130_e69250 * locals.var_temp1_dn9)) / (locals.var_temp1 * locals.var_temp1)), );
            locals.var_finracc_rv = 0.0;
        }

        if (locals.var_guard1527 != 0.0) {
            (locals.var_finrdep, locals.var_finrdep_dn4, locals.var_finrdep_dn6, locals.var_finrdep_dn7, locals.var_finrdep_dn8, locals.var_finrdep_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_finrdep_rv = 0.0;
            (locals.var_dvinrdep, locals.var_dvinrdep_dn4, locals.var_dvinrdep_dn6, locals.var_dvinrdep_dn7, locals.var_dvinrdep_dn8, locals.var_dvinrdep_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_dvinrdep_rv = 0.0;
        }

        let assign54160_e69267: f64 = if locals.var_fcinrdep_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1529 = assign54160_e69267;
        locals.var_guard1529_rv = 0.0;

        if ((locals.var_guard1527 != 0.0) && (locals.var_guard1529 != 0.0)) {
            let assign54170_e69273: f64 = (0.5 * locals.var_phib_ac);
            let assign54170_e69278: f64 = (locals.var_gf_ac * 0.7071067811865475);
            let assign54170_e69279: f64 = (1.0 + assign54170_e69278);
            let assign54170_e69280: f64 = (locals.var_phit1_ac * assign54170_e69279);
            let assign54170_e69281: f64 = (assign54170_e69273 + assign54170_e69280);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign54170_e69281, ((0.5 * locals.var_phib_ac_dn4) + ((locals.var_phit1_ac_dn4 * assign54170_e69279) + (locals.var_phit1_ac * (locals.var_gf_ac_dn4 * 0.7071067811865475)))), ((locals.var_phit1_ac_dn6 * assign54170_e69279) + (locals.var_phit1_ac * (locals.var_gf_ac_dn6 * 0.7071067811865475))), ((locals.var_phit1_ac_dn7 * assign54170_e69279) + (locals.var_phit1_ac * (locals.var_gf_ac_dn7 * 0.7071067811865475))), ((locals.var_phit1_ac_dn8 * assign54170_e69279) + (locals.var_phit1_ac * (locals.var_gf_ac_dn8 * 0.7071067811865475))), ((locals.var_phit1_ac_dn9 * assign54170_e69279) + (locals.var_phit1_ac * (locals.var_gf_ac_dn9 * 0.7071067811865475))), );
            locals.var_temp__blk949_rv = 0.0;
        }

        if ((locals.var_guard1527 != 0.0) && (locals.var_guard1529 != 0.0)) {
            let assign54180_e69289: f64 = (locals.var_vgb1_ac / locals.var_temp__blk949);
            (locals.var_xginrdep, locals.var_xginrdep_dn4, locals.var_xginrdep_dn6, locals.var_xginrdep_dn7, locals.var_xginrdep_dn8, locals.var_xginrdep_dn9, ) = (assign54180_e69289, (((locals.var_vgb1_ac_dn4 * locals.var_temp__blk949) - (locals.var_vgb1_ac * locals.var_temp__blk949_dn4)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_vgb1_ac_dn6 * locals.var_temp__blk949) - (locals.var_vgb1_ac * locals.var_temp__blk949_dn6)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_vgb1_ac_dn7 * locals.var_temp__blk949) - (locals.var_vgb1_ac * locals.var_temp__blk949_dn7)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_vgb1_ac_dn8 * locals.var_temp__blk949) - (locals.var_vgb1_ac * locals.var_temp__blk949_dn8)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_vgb1_ac_dn9 * locals.var_temp__blk949) - (locals.var_vgb1_ac * locals.var_temp__blk949_dn9)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), );
            locals.var_xginrdep_rv = 0.0;
        }

        let assign54190_e69293: f64 = (locals.var_xginrdep).abs();
        let assign54190_e69295: f64 = if assign54190_e69293 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1530 = assign54190_e69295;
        locals.var_guard1530_rv = 0.0;

        if (((locals.var_guard1527 != 0.0) && (locals.var_guard1529 != 0.0)) && (locals.var_guard1530 != 0.0)) {
            let assign54200_e69304: f64 = (-locals.var_xginrdep);
            let assign54200_e69305: f64 = (assign54200_e69304).exp();
            let assign54200_e69306: f64 = (1.0 + assign54200_e69305);
            let assign54200_e69307: f64 = (1.0 / assign54200_e69306);
            (locals.var_finrdep, locals.var_finrdep_dn4, locals.var_finrdep_dn6, locals.var_finrdep_dn7, locals.var_finrdep_dn8, locals.var_finrdep_dn9, ) = (assign54200_e69307, (-((assign54200_e69305 * (-locals.var_xginrdep_dn4)) / (assign54200_e69306 * assign54200_e69306))), (-((assign54200_e69305 * (-locals.var_xginrdep_dn6)) / (assign54200_e69306 * assign54200_e69306))), (-((assign54200_e69305 * (-locals.var_xginrdep_dn7)) / (assign54200_e69306 * assign54200_e69306))), (-((assign54200_e69305 * (-locals.var_xginrdep_dn8)) / (assign54200_e69306 * assign54200_e69306))), (-((assign54200_e69305 * (-locals.var_xginrdep_dn9)) / (assign54200_e69306 * assign54200_e69306))), );
            locals.var_finrdep_rv = 0.0;
        }

        let assign54210_e69312: f64 = if locals.var_xginrdep < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1531 = assign54210_e69312;
        locals.var_guard1531_rv = 0.0;

        if ((((locals.var_guard1527 != 0.0) && (locals.var_guard1529 != 0.0)) && (locals.var_guard1530 == 0.0)) && (locals.var_guard1531 != 0.0)) {
            let assign54220_e69324: f64 = (-230.25850929940458);
            let assign54220_e69326: f64 = (assign54220_e69324 + locals.var_xginrdep);
            let assign54220_e69330: f64 = (-230.25850929940458);
            let assign54220_e69332: f64 = (assign54220_e69330 + locals.var_xginrdep);
            let assign54220_e69335: f64 = (-230.25850929940458);
            let assign54220_e69337: f64 = (assign54220_e69335 + locals.var_xginrdep);
            let assign54220_e69339: f64 = (assign54220_e69337 * 0.3333333333333333);
            let assign54220_e69340: f64 = (1.0 + assign54220_e69339);
            let assign54220_e69341: f64 = (assign54220_e69332 * assign54220_e69340);
            let assign54220_e69342: f64 = (0.5 * assign54220_e69341);
            let assign54220_e69343: f64 = (1.0 + assign54220_e69342);
            let assign54220_e69344: f64 = (assign54220_e69326 * assign54220_e69343);
            let assign54220_e69345: f64 = (1.0 + assign54220_e69344);
            let assign54220_e69346: f64 = (1e-100 / assign54220_e69345);
            (locals.var_finrdep, locals.var_finrdep_dn4, locals.var_finrdep_dn6, locals.var_finrdep_dn7, locals.var_finrdep_dn8, locals.var_finrdep_dn9, ) = (assign54220_e69346, (-((1e-100 * ((locals.var_xginrdep_dn4 * assign54220_e69343) + (assign54220_e69326 * (0.5 * ((locals.var_xginrdep_dn4 * assign54220_e69340) + (assign54220_e69332 * (locals.var_xginrdep_dn4 * 0.3333333333333333))))))) / (assign54220_e69345 * assign54220_e69345))), (-((1e-100 * ((locals.var_xginrdep_dn6 * assign54220_e69343) + (assign54220_e69326 * (0.5 * ((locals.var_xginrdep_dn6 * assign54220_e69340) + (assign54220_e69332 * (locals.var_xginrdep_dn6 * 0.3333333333333333))))))) / (assign54220_e69345 * assign54220_e69345))), (-((1e-100 * ((locals.var_xginrdep_dn7 * assign54220_e69343) + (assign54220_e69326 * (0.5 * ((locals.var_xginrdep_dn7 * assign54220_e69340) + (assign54220_e69332 * (locals.var_xginrdep_dn7 * 0.3333333333333333))))))) / (assign54220_e69345 * assign54220_e69345))), (-((1e-100 * ((locals.var_xginrdep_dn8 * assign54220_e69343) + (assign54220_e69326 * (0.5 * ((locals.var_xginrdep_dn8 * assign54220_e69340) + (assign54220_e69332 * (locals.var_xginrdep_dn8 * 0.3333333333333333))))))) / (assign54220_e69345 * assign54220_e69345))), (-((1e-100 * ((locals.var_xginrdep_dn9 * assign54220_e69343) + (assign54220_e69326 * (0.5 * ((locals.var_xginrdep_dn9 * assign54220_e69340) + (assign54220_e69332 * (locals.var_xginrdep_dn9 * 0.3333333333333333))))))) / (assign54220_e69345 * assign54220_e69345))), );
            locals.var_finrdep_rv = 0.0;
        }

        let assign54230_e69351: f64 = if locals.var_xginrdep < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1532 = assign54230_e69351;
        locals.var_guard1532_rv = 0.0;

        if (((locals.var_guard1527 != 0.0) && (locals.var_guard1529 != 0.0)) && (locals.var_guard1532 != 0.0)) {
            let assign54240_e69359: f64 = (locals.var_xginrdep).exp();
            let assign54240_e69360: f64 = (1.0 + assign54240_e69359);
            let assign54240_e69361: f64 = (assign54240_e69360).ln();
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign54240_e69361, ((assign54240_e69359 * locals.var_xginrdep_dn4) / assign54240_e69360), ((assign54240_e69359 * locals.var_xginrdep_dn6) / assign54240_e69360), ((assign54240_e69359 * locals.var_xginrdep_dn7) / assign54240_e69360), ((assign54240_e69359 * locals.var_xginrdep_dn8) / assign54240_e69360), ((assign54240_e69359 * locals.var_xginrdep_dn9) / assign54240_e69360), );
            locals.var_temp1_rv = 0.0;
        }

        if (((locals.var_guard1527 != 0.0) && (locals.var_guard1529 != 0.0)) && (locals.var_guard1532 == 0.0)) {
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (locals.var_xginrdep, locals.var_xginrdep_dn4, locals.var_xginrdep_dn6, locals.var_xginrdep_dn7, locals.var_xginrdep_dn8, locals.var_xginrdep_dn9, );
            locals.var_temp1_rv = 0.0;
        }

        if ((locals.var_guard1527 != 0.0) && (locals.var_guard1529 != 0.0)) {
            let assign54260_e69378: f64 = (locals.var_temp__blk949 * locals.var_temp1);
            (locals.var_dvinrdep, locals.var_dvinrdep_dn4, locals.var_dvinrdep_dn6, locals.var_dvinrdep_dn7, locals.var_dvinrdep_dn8, locals.var_dvinrdep_dn9, ) = (assign54260_e69378, ((locals.var_temp__blk949_dn4 * locals.var_temp1) + (locals.var_temp__blk949 * locals.var_temp1_dn4)), ((locals.var_temp__blk949_dn6 * locals.var_temp1) + (locals.var_temp__blk949 * locals.var_temp1_dn6)), ((locals.var_temp__blk949_dn7 * locals.var_temp1) + (locals.var_temp__blk949 * locals.var_temp1_dn7)), ((locals.var_temp__blk949_dn8 * locals.var_temp1) + (locals.var_temp__blk949 * locals.var_temp1_dn8)), ((locals.var_temp__blk949_dn9 * locals.var_temp1) + (locals.var_temp__blk949 * locals.var_temp1_dn9)), );
            locals.var_dvinrdep_rv = 0.0;
        }

        if (locals.var_guard1527 != 0.0) {
            let assign54270_e69385: f64 = (locals.var_finrdep - locals.var_finracc);
            let assign54270_e69386: f64 = (locals.var_fcinrdep_i * assign54270_e69385);
            let assign54270_e69388: f64 = (assign54270_e69386 + locals.var_finracc);
            (locals.var_finr, locals.var_finr_dn4, locals.var_finr_dn6, locals.var_finr_dn7, locals.var_finr_dn8, locals.var_finr_dn9, ) = (assign54270_e69388, ((locals.var_fcinrdep_i * (locals.var_finrdep_dn4 - locals.var_finracc_dn4)) + locals.var_finracc_dn4), ((locals.var_fcinrdep_i * (locals.var_finrdep_dn6 - locals.var_finracc_dn6)) + locals.var_finracc_dn6), ((locals.var_fcinrdep_i * (locals.var_finrdep_dn7 - locals.var_finracc_dn7)) + locals.var_finracc_dn7), ((locals.var_fcinrdep_i * (locals.var_finrdep_dn8 - locals.var_finracc_dn8)) + locals.var_finracc_dn8), ((locals.var_fcinrdep_i * (locals.var_finrdep_dn9 - locals.var_finracc_dn9)) + locals.var_finracc_dn9), );
            locals.var_finr_rv = 0.0;
        }

        if (locals.var_guard1527 != 0.0) {
            let assign54280_e69395: f64 = (locals.var_dvinrdep - locals.var_dvinracc);
            let assign54280_e69396: f64 = (locals.var_fcinrdep_i * assign54280_e69395);
            let assign54280_e69398: f64 = (assign54280_e69396 + locals.var_dvinracc);
            (locals.var_dvinr, locals.var_dvinr_dn4, locals.var_dvinr_dn6, locals.var_dvinr_dn7, locals.var_dvinr_dn8, locals.var_dvinr_dn9, ) = (assign54280_e69398, ((locals.var_fcinrdep_i * (locals.var_dvinrdep_dn4 - locals.var_dvinracc_dn4)) + locals.var_dvinracc_dn4), ((locals.var_fcinrdep_i * (locals.var_dvinrdep_dn6 - locals.var_dvinracc_dn6)) + locals.var_dvinracc_dn6), ((locals.var_fcinrdep_i * (locals.var_dvinrdep_dn7 - locals.var_dvinracc_dn7)) + locals.var_dvinracc_dn7), ((locals.var_fcinrdep_i * (locals.var_dvinrdep_dn8 - locals.var_dvinracc_dn8)) + locals.var_dvinracc_dn8), ((locals.var_fcinrdep_i * (locals.var_dvinrdep_dn9 - locals.var_dvinracc_dn9)) + locals.var_dvinracc_dn9), );
            locals.var_dvinr_rv = 0.0;
        }

        if (locals.var_guard1527 != 0.0) {
            let assign54290_e69405: f64 = (locals.var_phit1_ac * locals.var_xno_s_ac);
            let assign54290_e69406: f64 = (locals.var_vgb1_ac - assign54290_e69405);
            let assign54290_e69408: f64 = (assign54290_e69406 - locals.var_voxm_ac);
            let assign54290_e69411: f64 = (0.5 * locals.var_dps_ac);
            let assign54290_e69412: f64 = (assign54290_e69408 - assign54290_e69411);
            (locals.var_vgsinr, locals.var_vgsinr_dn4, locals.var_vgsinr_dn6, locals.var_vgsinr_dn7, locals.var_vgsinr_dn8, locals.var_vgsinr_dn9, ) = (assign54290_e69412, (((locals.var_vgb1_ac_dn4 - ((locals.var_phit1_ac_dn4 * locals.var_xno_s_ac) + (locals.var_phit1_ac * locals.var_xno_s_ac_dn4))) - locals.var_voxm_ac_dn4) - (0.5 * locals.var_dps_ac_dn4)), (((locals.var_vgb1_ac_dn6 - ((locals.var_phit1_ac_dn6 * locals.var_xno_s_ac) + (locals.var_phit1_ac * locals.var_xno_s_ac_dn6))) - locals.var_voxm_ac_dn6) - (0.5 * locals.var_dps_ac_dn6)), (((locals.var_vgb1_ac_dn7 - ((locals.var_phit1_ac_dn7 * locals.var_xno_s_ac) + (locals.var_phit1_ac * locals.var_xno_s_ac_dn7))) - locals.var_voxm_ac_dn7) - (0.5 * locals.var_dps_ac_dn7)), (((locals.var_vgb1_ac_dn8 - ((locals.var_phit1_ac_dn8 * locals.var_xno_s_ac) + (locals.var_phit1_ac * locals.var_xno_s_ac_dn8))) - locals.var_voxm_ac_dn8) - (0.5 * locals.var_dps_ac_dn8)), (((locals.var_vgb1_ac_dn9 - ((locals.var_phit1_ac_dn9 * locals.var_xno_s_ac) + (locals.var_phit1_ac * locals.var_xno_s_ac_dn9))) - locals.var_voxm_ac_dn9) - (0.5 * locals.var_dps_ac_dn9)), );
            locals.var_vgsinr_rv = 0.0;
        }

        if (locals.var_guard1527 != 0.0) {
            let assign54300_e69418: f64 = (locals.var_vgb1_ac - locals.var_vgsinr);
            let assign54300_e69420: f64 = (assign54300_e69418 - locals.var_qbs_ac);
            (locals.var_vsginr, locals.var_vsginr_dn4, locals.var_vsginr_dn6, locals.var_vsginr_dn7, locals.var_vsginr_dn8, locals.var_vsginr_dn9, ) = (assign54300_e69420, ((locals.var_vgb1_ac_dn4 - locals.var_vgsinr_dn4) - locals.var_qbs_ac_dn4), ((locals.var_vgb1_ac_dn6 - locals.var_vgsinr_dn6) - locals.var_qbs_ac_dn6), ((locals.var_vgb1_ac_dn7 - locals.var_vgsinr_dn7) - locals.var_qbs_ac_dn7), ((locals.var_vgb1_ac_dn8 - locals.var_vgsinr_dn8) - locals.var_qbs_ac_dn8), ((locals.var_vgb1_ac_dn9 - locals.var_vgsinr_dn9) - locals.var_qbs_ac_dn9), );
            locals.var_vsginr_rv = 0.0;
        }

        if (locals.var_guard1527 != 0.0) {
            let assign54310_e69426: f64 = (locals.var_dps_ac + locals.var_vgsinr);
            let assign54310_e69428: f64 = (assign54310_e69426 - locals.var_v_ds);
            (locals.var_vgdinr, locals.var_vgdinr_dn4, locals.var_vgdinr_dn6, locals.var_vgdinr_dn7, locals.var_vgdinr_dn8, locals.var_vgdinr_dn9, ) = (assign54310_e69428, (locals.var_dps_ac_dn4 + locals.var_vgsinr_dn4), (locals.var_dps_ac_dn6 + locals.var_vgsinr_dn6), ((locals.var_dps_ac_dn7 + locals.var_vgsinr_dn7) - locals.var_v_ds_dn7), ((locals.var_dps_ac_dn8 + locals.var_vgsinr_dn8) - locals.var_v_ds_dn8), (locals.var_dps_ac_dn9 + locals.var_vgsinr_dn9), );
            locals.var_vgdinr_rv = 0.0;
        }

        if (locals.var_guard1527 != 0.0) {
            let assign54320_e69434: f64 = (locals.var_vgb1_ac - locals.var_vgdinr);
            let assign54320_e69436: f64 = (assign54320_e69434 - locals.var_qbd_ac);
            (locals.var_vdginr, locals.var_vdginr_dn4, locals.var_vdginr_dn6, locals.var_vdginr_dn7, locals.var_vdginr_dn8, locals.var_vdginr_dn9, ) = (assign54320_e69436, ((locals.var_vgb1_ac_dn4 - locals.var_vgdinr_dn4) - locals.var_qbd_ac_dn4), ((locals.var_vgb1_ac_dn6 - locals.var_vgdinr_dn6) - locals.var_qbd_ac_dn6), ((locals.var_vgb1_ac_dn7 - locals.var_vgdinr_dn7) - locals.var_qbd_ac_dn7), ((locals.var_vgb1_ac_dn8 - locals.var_vgdinr_dn8) - locals.var_qbd_ac_dn8), ((locals.var_vgb1_ac_dn9 - locals.var_vgdinr_dn9) - locals.var_qbd_ac_dn9), );
            locals.var_vdginr_rv = 0.0;
        }

        let assign54330_e69441: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1533 = assign54330_e69441;
        locals.var_guard1533_rv = 0.0;

        if ((locals.var_guard1527 != 0.0) && (locals.var_guard1533 != 0.0)) {
            let assign54340_e69448: f64 = (locals.var_cinrd_i * locals.var_vgdinr);
            let assign54340_e69451: f64 = (locals.var_cinr_i * locals.var_vgsinr);
            let assign54340_e69452: f64 = (assign54340_e69448 + assign54340_e69451);
            let assign54340_e69453: f64 = (locals.var_finr * assign54340_e69452);
            (locals.var_qginr, locals.var_qginr_dn4, locals.var_qginr_dn6, locals.var_qginr_dn7, locals.var_qginr_dn8, locals.var_qginr_dn9, ) = (assign54340_e69453, ((locals.var_finr_dn4 * assign54340_e69452) + (locals.var_finr * ((locals.var_cinrd_i * locals.var_vgdinr_dn4) + (locals.var_cinr_i * locals.var_vgsinr_dn4)))), ((locals.var_finr_dn6 * assign54340_e69452) + (locals.var_finr * ((locals.var_cinrd_i * locals.var_vgdinr_dn6) + (locals.var_cinr_i * locals.var_vgsinr_dn6)))), ((locals.var_finr_dn7 * assign54340_e69452) + (locals.var_finr * ((locals.var_cinrd_i * locals.var_vgdinr_dn7) + (locals.var_cinr_i * locals.var_vgsinr_dn7)))), ((locals.var_finr_dn8 * assign54340_e69452) + (locals.var_finr * ((locals.var_cinrd_i * locals.var_vgdinr_dn8) + (locals.var_cinr_i * locals.var_vgsinr_dn8)))), ((locals.var_finr_dn9 * assign54340_e69452) + (locals.var_finr * ((locals.var_cinrd_i * locals.var_vgdinr_dn9) + (locals.var_cinr_i * locals.var_vgsinr_dn9)))), );
            locals.var_qginr_rv = 0.0;
        }

        if ((locals.var_guard1527 != 0.0) && (locals.var_guard1533 != 0.0)) {
            let assign54350_e69462: f64 = (locals.var_vsginr - locals.var_dvinr);
            let assign54350_e69463: f64 = (locals.var_cinr_i * assign54350_e69462);
            (locals.var_qsinr, locals.var_qsinr_dn4, locals.var_qsinr_dn6, locals.var_qsinr_dn7, locals.var_qsinr_dn8, locals.var_qsinr_dn9, ) = (assign54350_e69463, (locals.var_cinr_i * (locals.var_vsginr_dn4 - locals.var_dvinr_dn4)), (locals.var_cinr_i * (locals.var_vsginr_dn6 - locals.var_dvinr_dn6)), (locals.var_cinr_i * (locals.var_vsginr_dn7 - locals.var_dvinr_dn7)), (locals.var_cinr_i * (locals.var_vsginr_dn8 - locals.var_dvinr_dn8)), (locals.var_cinr_i * (locals.var_vsginr_dn9 - locals.var_dvinr_dn9)), );
            locals.var_qsinr_rv = 0.0;
        }

        if ((locals.var_guard1527 != 0.0) && (locals.var_guard1533 != 0.0)) {
            let assign54360_e69472: f64 = (locals.var_vdginr - locals.var_dvinr);
            let assign54360_e69473: f64 = (locals.var_cinrd_i * assign54360_e69472);
            (locals.var_qdinr, locals.var_qdinr_dn4, locals.var_qdinr_dn6, locals.var_qdinr_dn7, locals.var_qdinr_dn8, locals.var_qdinr_dn9, ) = (assign54360_e69473, (locals.var_cinrd_i * (locals.var_vdginr_dn4 - locals.var_dvinr_dn4)), (locals.var_cinrd_i * (locals.var_vdginr_dn6 - locals.var_dvinr_dn6)), (locals.var_cinrd_i * (locals.var_vdginr_dn7 - locals.var_dvinr_dn7)), (locals.var_cinrd_i * (locals.var_vdginr_dn8 - locals.var_dvinr_dn8)), (locals.var_cinrd_i * (locals.var_vdginr_dn9 - locals.var_dvinr_dn9)), );
            locals.var_qdinr_rv = 0.0;
        }

        if ((locals.var_guard1527 != 0.0) && (locals.var_guard1533 == 0.0)) {
            let assign54370_e69483: f64 = (locals.var_cinr_i * locals.var_vgdinr);
            let assign54370_e69486: f64 = (locals.var_cinrd_i * locals.var_vgsinr);
            let assign54370_e69487: f64 = (assign54370_e69483 + assign54370_e69486);
            let assign54370_e69488: f64 = (locals.var_finr * assign54370_e69487);
            (locals.var_qginr, locals.var_qginr_dn4, locals.var_qginr_dn6, locals.var_qginr_dn7, locals.var_qginr_dn8, locals.var_qginr_dn9, ) = (assign54370_e69488, ((locals.var_finr_dn4 * assign54370_e69487) + (locals.var_finr * ((locals.var_cinr_i * locals.var_vgdinr_dn4) + (locals.var_cinrd_i * locals.var_vgsinr_dn4)))), ((locals.var_finr_dn6 * assign54370_e69487) + (locals.var_finr * ((locals.var_cinr_i * locals.var_vgdinr_dn6) + (locals.var_cinrd_i * locals.var_vgsinr_dn6)))), ((locals.var_finr_dn7 * assign54370_e69487) + (locals.var_finr * ((locals.var_cinr_i * locals.var_vgdinr_dn7) + (locals.var_cinrd_i * locals.var_vgsinr_dn7)))), ((locals.var_finr_dn8 * assign54370_e69487) + (locals.var_finr * ((locals.var_cinr_i * locals.var_vgdinr_dn8) + (locals.var_cinrd_i * locals.var_vgsinr_dn8)))), ((locals.var_finr_dn9 * assign54370_e69487) + (locals.var_finr * ((locals.var_cinr_i * locals.var_vgdinr_dn9) + (locals.var_cinrd_i * locals.var_vgsinr_dn9)))), );
            locals.var_qginr_rv = 0.0;
        }

        if ((locals.var_guard1527 != 0.0) && (locals.var_guard1533 == 0.0)) {
            let assign54380_e69498: f64 = (locals.var_vsginr - locals.var_dvinr);
            let assign54380_e69499: f64 = (locals.var_cinrd_i * assign54380_e69498);
            (locals.var_qsinr, locals.var_qsinr_dn4, locals.var_qsinr_dn6, locals.var_qsinr_dn7, locals.var_qsinr_dn8, locals.var_qsinr_dn9, ) = (assign54380_e69499, (locals.var_cinrd_i * (locals.var_vsginr_dn4 - locals.var_dvinr_dn4)), (locals.var_cinrd_i * (locals.var_vsginr_dn6 - locals.var_dvinr_dn6)), (locals.var_cinrd_i * (locals.var_vsginr_dn7 - locals.var_dvinr_dn7)), (locals.var_cinrd_i * (locals.var_vsginr_dn8 - locals.var_dvinr_dn8)), (locals.var_cinrd_i * (locals.var_vsginr_dn9 - locals.var_dvinr_dn9)), );
            locals.var_qsinr_rv = 0.0;
        }

        if ((locals.var_guard1527 != 0.0) && (locals.var_guard1533 == 0.0)) {
            let assign54390_e69509: f64 = (locals.var_vdginr - locals.var_dvinr);
            let assign54390_e69510: f64 = (locals.var_cinr_i * assign54390_e69509);
            (locals.var_qdinr, locals.var_qdinr_dn4, locals.var_qdinr_dn6, locals.var_qdinr_dn7, locals.var_qdinr_dn8, locals.var_qdinr_dn9, ) = (assign54390_e69510, (locals.var_cinr_i * (locals.var_vdginr_dn4 - locals.var_dvinr_dn4)), (locals.var_cinr_i * (locals.var_vdginr_dn6 - locals.var_dvinr_dn6)), (locals.var_cinr_i * (locals.var_vdginr_dn7 - locals.var_dvinr_dn7)), (locals.var_cinr_i * (locals.var_vdginr_dn8 - locals.var_dvinr_dn8)), (locals.var_cinr_i * (locals.var_vdginr_dn9 - locals.var_dvinr_dn9)), );
            locals.var_qdinr_rv = 0.0;
        }

        if (locals.var_guard1527 != 0.0) {
            let assign54400_e69516: f64 = (locals.var_qg + locals.var_qginr);
            (locals.var_qg, locals.var_qg_dn4, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, ) = (assign54400_e69516, (locals.var_qg_dn4 + locals.var_qginr_dn4), (locals.var_qg_dn6 + locals.var_qginr_dn6), (locals.var_qg_dn7 + locals.var_qginr_dn7), (locals.var_qg_dn8 + locals.var_qginr_dn8), (locals.var_qg_dn9 + locals.var_qginr_dn9), );
            locals.var_qg_rv = 0.0;
        }

        if (locals.var_guard1527 != 0.0) {
            let assign54410_e69522: f64 = (locals.var_qd + locals.var_qdinr);
            (locals.var_qd, locals.var_qd_dn4, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, ) = (assign54410_e69522, (locals.var_qd_dn4 + locals.var_qdinr_dn4), (locals.var_qd_dn6 + locals.var_qdinr_dn6), (locals.var_qd_dn7 + locals.var_qdinr_dn7), (locals.var_qd_dn8 + locals.var_qdinr_dn8), (locals.var_qd_dn9 + locals.var_qdinr_dn9), );
            locals.var_qd_rv = 0.0;
        }

        if (locals.var_guard1527 != 0.0) {
            let assign54420_e69528: f64 = (locals.var_qb - locals.var_qginr);
            let assign54420_e69530: f64 = (assign54420_e69528 - locals.var_qdinr);
            let assign54420_e69532: f64 = (assign54420_e69530 - locals.var_qsinr);
            (locals.var_qb, locals.var_qb_dn4, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, ) = (assign54420_e69532, (((locals.var_qb_dn4 - locals.var_qginr_dn4) - locals.var_qdinr_dn4) - locals.var_qsinr_dn4), (((locals.var_qb_dn6 - locals.var_qginr_dn6) - locals.var_qdinr_dn6) - locals.var_qsinr_dn6), (((locals.var_qb_dn7 - locals.var_qginr_dn7) - locals.var_qdinr_dn7) - locals.var_qsinr_dn7), (((locals.var_qb_dn8 - locals.var_qginr_dn8) - locals.var_qdinr_dn8) - locals.var_qsinr_dn8), (((locals.var_qb_dn9 - locals.var_qginr_dn9) - locals.var_qdinr_dn9) - locals.var_qsinr_dn9), );
            locals.var_qb_rv = 0.0;
        }

        (locals.var_qg_ov_s, locals.var_qg_ov_s_dn4, locals.var_qg_ov_s_dn6, locals.var_qg_ov_s_dn7, locals.var_qg_ov_s_dn8, locals.var_qg_ov_s_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_qg_ov_s_rv = 0.0;

        (locals.var_yb_ov_s, locals.var_yb_ov_s_dn4, locals.var_yb_ov_s_dn6, locals.var_yb_ov_s_dn7, locals.var_yb_ov_s_dn8, locals.var_yb_ov_s_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_yb_ov_s_rv = 0.0;

        let assign54470_e69549: f64 = if ((locals.var_cgov_i > 0.0) && (locals.var_fcgovacc_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1534 = assign54470_e69549;
        locals.var_guard1534_rv = 0.0;

        if (locals.var_guard1534 != 0.0) {
            let assign54480_e69554: f64 = (0.5 * locals.var_xgb_ov);
            let assign54480_e69556: f64 = (assign54480_e69554 + locals.var_dxgb_ov_s);
            let assign54480_e69557: f64 = (locals.var_cgovaccg_i * assign54480_e69556);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign54480_e69557, (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn4)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn6)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn7)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn8)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn9)), );
            locals.var_temp__blk949_rv = 0.0;
        }

        let assign54490_e69562: f64 = if locals.var_temp__blk949 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1535 = assign54490_e69562;
        locals.var_guard1535_rv = 0.0;

        let assign54500_e69565: f64 = (-230.25850929940458);
        let assign54500_e69566: f64 = if locals.var_temp__blk949 > assign54500_e69565 { 1.0 } else { 0.0 };
        locals.var_guard1536 = assign54500_e69566;
        locals.var_guard1536_rv = 0.0;

        if (((locals.var_guard1534 != 0.0) && (locals.var_guard1535 != 0.0)) && (locals.var_guard1536 != 0.0)) {
            let assign54510_e69573: f64 = (locals.var_temp__blk949).exp();
            (locals.var_yb_ov_s, locals.var_yb_ov_s_dn4, locals.var_yb_ov_s_dn6, locals.var_yb_ov_s_dn7, locals.var_yb_ov_s_dn8, locals.var_yb_ov_s_dn9, ) = (assign54510_e69573, (assign54510_e69573 * locals.var_temp__blk949_dn4), (assign54510_e69573 * locals.var_temp__blk949_dn6), (assign54510_e69573 * locals.var_temp__blk949_dn7), (assign54510_e69573 * locals.var_temp__blk949_dn8), (assign54510_e69573 * locals.var_temp__blk949_dn9), );
            locals.var_yb_ov_s_rv = 0.0;
        }

        if (((locals.var_guard1534 != 0.0) && (locals.var_guard1535 != 0.0)) && (locals.var_guard1536 == 0.0)) {
            let assign54520_e69585: f64 = (-230.25850929940458);
            let assign54520_e69587: f64 = (assign54520_e69585 - locals.var_temp__blk949);
            let assign54520_e69591: f64 = (-230.25850929940458);
            let assign54520_e69593: f64 = (assign54520_e69591 - locals.var_temp__blk949);
            let assign54520_e69596: f64 = (-230.25850929940458);
            let assign54520_e69598: f64 = (assign54520_e69596 - locals.var_temp__blk949);
            let assign54520_e69600: f64 = (assign54520_e69598 * 0.3333333333333333);
            let assign54520_e69601: f64 = (1.0 + assign54520_e69600);
            let assign54520_e69602: f64 = (assign54520_e69593 * assign54520_e69601);
            let assign54520_e69603: f64 = (0.5 * assign54520_e69602);
            let assign54520_e69604: f64 = (1.0 + assign54520_e69603);
            let assign54520_e69605: f64 = (assign54520_e69587 * assign54520_e69604);
            let assign54520_e69606: f64 = (1.0 + assign54520_e69605);
            let assign54520_e69607: f64 = (1e-100 / assign54520_e69606);
            (locals.var_yb_ov_s, locals.var_yb_ov_s_dn4, locals.var_yb_ov_s_dn6, locals.var_yb_ov_s_dn7, locals.var_yb_ov_s_dn8, locals.var_yb_ov_s_dn9, ) = (assign54520_e69607, (-((1e-100 * (((-locals.var_temp__blk949_dn4) * assign54520_e69604) + (assign54520_e69587 * (0.5 * (((-locals.var_temp__blk949_dn4) * assign54520_e69601) + (assign54520_e69593 * ((-locals.var_temp__blk949_dn4) * 0.3333333333333333))))))) / (assign54520_e69606 * assign54520_e69606))), (-((1e-100 * (((-locals.var_temp__blk949_dn6) * assign54520_e69604) + (assign54520_e69587 * (0.5 * (((-locals.var_temp__blk949_dn6) * assign54520_e69601) + (assign54520_e69593 * ((-locals.var_temp__blk949_dn6) * 0.3333333333333333))))))) / (assign54520_e69606 * assign54520_e69606))), (-((1e-100 * (((-locals.var_temp__blk949_dn7) * assign54520_e69604) + (assign54520_e69587 * (0.5 * (((-locals.var_temp__blk949_dn7) * assign54520_e69601) + (assign54520_e69593 * ((-locals.var_temp__blk949_dn7) * 0.3333333333333333))))))) / (assign54520_e69606 * assign54520_e69606))), (-((1e-100 * (((-locals.var_temp__blk949_dn8) * assign54520_e69604) + (assign54520_e69587 * (0.5 * (((-locals.var_temp__blk949_dn8) * assign54520_e69601) + (assign54520_e69593 * ((-locals.var_temp__blk949_dn8) * 0.3333333333333333))))))) / (assign54520_e69606 * assign54520_e69606))), (-((1e-100 * (((-locals.var_temp__blk949_dn9) * assign54520_e69604) + (assign54520_e69587 * (0.5 * (((-locals.var_temp__blk949_dn9) * assign54520_e69601) + (assign54520_e69593 * ((-locals.var_temp__blk949_dn9) * 0.3333333333333333))))))) / (assign54520_e69606 * assign54520_e69606))), );
            locals.var_yb_ov_s_rv = 0.0;
        }

        let assign54530_e69612: f64 = if locals.var_yb_ov_s > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1537 = assign54530_e69612;
        locals.var_guard1537_rv = 0.0;

        if (((locals.var_guard1534 != 0.0) && (locals.var_guard1535 != 0.0)) && (locals.var_guard1537 != 0.0)) {
            let assign54540_e69620: f64 = (1.0 + locals.var_yb_ov_s);
            let assign54540_e69621: f64 = (assign54540_e69620).ln();
            (locals.var_xgbeff_ov_s, locals.var_xgbeff_ov_s_dn4, locals.var_xgbeff_ov_s_dn6, locals.var_xgbeff_ov_s_dn7, locals.var_xgbeff_ov_s_dn8, locals.var_xgbeff_ov_s_dn9, ) = (assign54540_e69621, (locals.var_yb_ov_s_dn4 / assign54540_e69620), (locals.var_yb_ov_s_dn6 / assign54540_e69620), (locals.var_yb_ov_s_dn7 / assign54540_e69620), (locals.var_yb_ov_s_dn8 / assign54540_e69620), (locals.var_yb_ov_s_dn9 / assign54540_e69620), );
            locals.var_xgbeff_ov_s_rv = 0.0;
        }

        if (((locals.var_guard1534 != 0.0) && (locals.var_guard1535 != 0.0)) && (locals.var_guard1537 != 0.0)) {
            let assign54550_e69633: f64 = (1.0 + locals.var_xgbeff_ov_s);
            let assign54550_e69634: f64 = (assign54550_e69633).ln();
            let assign54550_e69637: f64 = (2.0 + locals.var_xgbeff_ov_s);
            let assign54550_e69638: f64 = (assign54550_e69634 / assign54550_e69637);
            let assign54550_e69639: f64 = (1.0 - assign54550_e69638);
            let assign54550_e69640: f64 = (locals.var_xgbeff_ov_s * assign54550_e69639);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign54550_e69640, ((locals.var_xgbeff_ov_s_dn4 * assign54550_e69639) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn4 / assign54550_e69633) * assign54550_e69637) - (assign54550_e69634 * locals.var_xgbeff_ov_s_dn4)) / (assign54550_e69637 * assign54550_e69637))))), ((locals.var_xgbeff_ov_s_dn6 * assign54550_e69639) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn6 / assign54550_e69633) * assign54550_e69637) - (assign54550_e69634 * locals.var_xgbeff_ov_s_dn6)) / (assign54550_e69637 * assign54550_e69637))))), ((locals.var_xgbeff_ov_s_dn7 * assign54550_e69639) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn7 / assign54550_e69633) * assign54550_e69637) - (assign54550_e69634 * locals.var_xgbeff_ov_s_dn7)) / (assign54550_e69637 * assign54550_e69637))))), ((locals.var_xgbeff_ov_s_dn8 * assign54550_e69639) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn8 / assign54550_e69633) * assign54550_e69637) - (assign54550_e69634 * locals.var_xgbeff_ov_s_dn8)) / (assign54550_e69637 * assign54550_e69637))))), ((locals.var_xgbeff_ov_s_dn9 * assign54550_e69639) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn9 / assign54550_e69633) * assign54550_e69637) - (assign54550_e69634 * locals.var_xgbeff_ov_s_dn9)) / (assign54550_e69637 * assign54550_e69637))))), );
            locals.var_temp1_rv = 0.0;
        }

        if (((locals.var_guard1534 != 0.0) && (locals.var_guard1535 != 0.0)) && (locals.var_guard1537 == 0.0)) {
            (locals.var_xgbeff_ov_s, locals.var_xgbeff_ov_s_dn4, locals.var_xgbeff_ov_s_dn6, locals.var_xgbeff_ov_s_dn7, locals.var_xgbeff_ov_s_dn8, locals.var_xgbeff_ov_s_dn9, ) = (locals.var_yb_ov_s, locals.var_yb_ov_s_dn4, locals.var_yb_ov_s_dn6, locals.var_yb_ov_s_dn7, locals.var_yb_ov_s_dn8, locals.var_yb_ov_s_dn9, );
            locals.var_xgbeff_ov_s_rv = 0.0;
        }

        if (((locals.var_guard1534 != 0.0) && (locals.var_guard1535 != 0.0)) && (locals.var_guard1537 == 0.0)) {
            let assign54570_e69660: f64 = (2.0 * locals.var_xgbeff_ov_s);
            let assign54570_e69663: f64 = (2.0 + locals.var_xgbeff_ov_s);
            let assign54570_e69664: f64 = (assign54570_e69660 / assign54570_e69663);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign54570_e69664, ((((2.0 * locals.var_xgbeff_ov_s_dn4) * assign54570_e69663) - (assign54570_e69660 * locals.var_xgbeff_ov_s_dn4)) / (assign54570_e69663 * assign54570_e69663)), ((((2.0 * locals.var_xgbeff_ov_s_dn6) * assign54570_e69663) - (assign54570_e69660 * locals.var_xgbeff_ov_s_dn6)) / (assign54570_e69663 * assign54570_e69663)), ((((2.0 * locals.var_xgbeff_ov_s_dn7) * assign54570_e69663) - (assign54570_e69660 * locals.var_xgbeff_ov_s_dn7)) / (assign54570_e69663 * assign54570_e69663)), ((((2.0 * locals.var_xgbeff_ov_s_dn8) * assign54570_e69663) - (assign54570_e69660 * locals.var_xgbeff_ov_s_dn8)) / (assign54570_e69663 * assign54570_e69663)), ((((2.0 * locals.var_xgbeff_ov_s_dn9) * assign54570_e69663) - (assign54570_e69660 * locals.var_xgbeff_ov_s_dn9)) / (assign54570_e69663 * assign54570_e69663)), );
            locals.var_temp1_rv = 0.0;
        }

        if ((locals.var_guard1534 != 0.0) && (locals.var_guard1535 == 0.0)) {
            (locals.var_xgbeff_ov_s, locals.var_xgbeff_ov_s_dn4, locals.var_xgbeff_ov_s_dn6, locals.var_xgbeff_ov_s_dn7, locals.var_xgbeff_ov_s_dn8, locals.var_xgbeff_ov_s_dn9, ) = (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, );
            locals.var_xgbeff_ov_s_rv = 0.0;
        }

        if ((locals.var_guard1534 != 0.0) && (locals.var_guard1535 == 0.0)) {
            let assign54590_e69682: f64 = (1.0 + locals.var_xgbeff_ov_s);
            let assign54590_e69683: f64 = (assign54590_e69682).ln();
            let assign54590_e69686: f64 = (2.0 + locals.var_xgbeff_ov_s);
            let assign54590_e69687: f64 = (assign54590_e69683 / assign54590_e69686);
            let assign54590_e69688: f64 = (1.0 - assign54590_e69687);
            let assign54590_e69689: f64 = (locals.var_xgbeff_ov_s * assign54590_e69688);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign54590_e69689, ((locals.var_xgbeff_ov_s_dn4 * assign54590_e69688) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn4 / assign54590_e69682) * assign54590_e69686) - (assign54590_e69683 * locals.var_xgbeff_ov_s_dn4)) / (assign54590_e69686 * assign54590_e69686))))), ((locals.var_xgbeff_ov_s_dn6 * assign54590_e69688) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn6 / assign54590_e69682) * assign54590_e69686) - (assign54590_e69683 * locals.var_xgbeff_ov_s_dn6)) / (assign54590_e69686 * assign54590_e69686))))), ((locals.var_xgbeff_ov_s_dn7 * assign54590_e69688) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn7 / assign54590_e69682) * assign54590_e69686) - (assign54590_e69683 * locals.var_xgbeff_ov_s_dn7)) / (assign54590_e69686 * assign54590_e69686))))), ((locals.var_xgbeff_ov_s_dn8 * assign54590_e69688) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn8 / assign54590_e69682) * assign54590_e69686) - (assign54590_e69683 * locals.var_xgbeff_ov_s_dn8)) / (assign54590_e69686 * assign54590_e69686))))), ((locals.var_xgbeff_ov_s_dn9 * assign54590_e69688) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn9 / assign54590_e69682) * assign54590_e69686) - (assign54590_e69683 * locals.var_xgbeff_ov_s_dn9)) / (assign54590_e69686 * assign54590_e69686))))), );
            locals.var_temp1_rv = 0.0;
        }

        if (locals.var_guard1534 != 0.0) {
            let assign54600_e69694: f64 = (-2.0);
            let assign54600_e69696: f64 = (assign54600_e69694 * locals.var_fcgovacc_i);
            let assign54600_e69698: f64 = (assign54600_e69696 / locals.var_cgovaccg_i);
            let assign54600_e69700: f64 = (assign54600_e69698 * locals.var_cgov_i);
            let assign54600_e69702: f64 = (assign54600_e69700 * locals.var_phita);
            let assign54600_e69704: f64 = (assign54600_e69702 * locals.var_temp1);
            (locals.var_qg_ov_s, locals.var_qg_ov_s_dn4, locals.var_qg_ov_s_dn6, locals.var_qg_ov_s_dn7, locals.var_qg_ov_s_dn8, locals.var_qg_ov_s_dn9, ) = (assign54600_e69704, (assign54600_e69702 * locals.var_temp1_dn4), (assign54600_e69702 * locals.var_temp1_dn6), (assign54600_e69702 * locals.var_temp1_dn7), (assign54600_e69702 * locals.var_temp1_dn8), (assign54600_e69702 * locals.var_temp1_dn9), );
            locals.var_qg_ov_s_rv = 0.0;
        }

        (locals.var_qg_ov_d, locals.var_qg_ov_d_dn4, locals.var_qg_ov_d_dn6, locals.var_qg_ov_d_dn7, locals.var_qg_ov_d_dn8, locals.var_qg_ov_d_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_qg_ov_d_rv = 0.0;

        (locals.var_yb_ov_d, locals.var_yb_ov_d_dn4, locals.var_yb_ov_d_dn6, locals.var_yb_ov_d_dn7, locals.var_yb_ov_d_dn8, locals.var_yb_ov_d_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        locals.var_yb_ov_d_rv = 0.0;

        let assign54630_e69715: f64 = if ((locals.var_cgovd_i > 0.0) && (locals.var_fcgovaccd_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1538 = assign54630_e69715;
        locals.var_guard1538_rv = 0.0;

        if (locals.var_guard1538 != 0.0) {
            let assign54640_e69720: f64 = (0.5 * locals.var_xgb_ov);
            let assign54640_e69722: f64 = (assign54640_e69720 + locals.var_dxgb_ov_d);
            let assign54640_e69723: f64 = (locals.var_cgovaccg_i * assign54640_e69722);
            (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, ) = (assign54640_e69723, (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn4)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn6)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn7)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn8)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn9)), );
            locals.var_temp__blk949_rv = 0.0;
        }

        let assign54650_e69728: f64 = if locals.var_temp__blk949 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1539 = assign54650_e69728;
        locals.var_guard1539_rv = 0.0;

        let assign54660_e69731: f64 = (-230.25850929940458);
        let assign54660_e69732: f64 = if locals.var_temp__blk949 > assign54660_e69731 { 1.0 } else { 0.0 };
        locals.var_guard1540 = assign54660_e69732;
        locals.var_guard1540_rv = 0.0;

        if (((locals.var_guard1538 != 0.0) && (locals.var_guard1539 != 0.0)) && (locals.var_guard1540 != 0.0)) {
            let assign54670_e69739: f64 = (locals.var_temp__blk949).exp();
            (locals.var_yb_ov_d, locals.var_yb_ov_d_dn4, locals.var_yb_ov_d_dn6, locals.var_yb_ov_d_dn7, locals.var_yb_ov_d_dn8, locals.var_yb_ov_d_dn9, ) = (assign54670_e69739, (assign54670_e69739 * locals.var_temp__blk949_dn4), (assign54670_e69739 * locals.var_temp__blk949_dn6), (assign54670_e69739 * locals.var_temp__blk949_dn7), (assign54670_e69739 * locals.var_temp__blk949_dn8), (assign54670_e69739 * locals.var_temp__blk949_dn9), );
            locals.var_yb_ov_d_rv = 0.0;
        }

        if (((locals.var_guard1538 != 0.0) && (locals.var_guard1539 != 0.0)) && (locals.var_guard1540 == 0.0)) {
            let assign54680_e69751: f64 = (-230.25850929940458);
            let assign54680_e69753: f64 = (assign54680_e69751 - locals.var_temp__blk949);
            let assign54680_e69757: f64 = (-230.25850929940458);
            let assign54680_e69759: f64 = (assign54680_e69757 - locals.var_temp__blk949);
            let assign54680_e69762: f64 = (-230.25850929940458);
            let assign54680_e69764: f64 = (assign54680_e69762 - locals.var_temp__blk949);
            let assign54680_e69766: f64 = (assign54680_e69764 * 0.3333333333333333);
            let assign54680_e69767: f64 = (1.0 + assign54680_e69766);
            let assign54680_e69768: f64 = (assign54680_e69759 * assign54680_e69767);
            let assign54680_e69769: f64 = (0.5 * assign54680_e69768);
            let assign54680_e69770: f64 = (1.0 + assign54680_e69769);
            let assign54680_e69771: f64 = (assign54680_e69753 * assign54680_e69770);
            let assign54680_e69772: f64 = (1.0 + assign54680_e69771);
            let assign54680_e69773: f64 = (1e-100 / assign54680_e69772);
            (locals.var_yb_ov_d, locals.var_yb_ov_d_dn4, locals.var_yb_ov_d_dn6, locals.var_yb_ov_d_dn7, locals.var_yb_ov_d_dn8, locals.var_yb_ov_d_dn9, ) = (assign54680_e69773, (-((1e-100 * (((-locals.var_temp__blk949_dn4) * assign54680_e69770) + (assign54680_e69753 * (0.5 * (((-locals.var_temp__blk949_dn4) * assign54680_e69767) + (assign54680_e69759 * ((-locals.var_temp__blk949_dn4) * 0.3333333333333333))))))) / (assign54680_e69772 * assign54680_e69772))), (-((1e-100 * (((-locals.var_temp__blk949_dn6) * assign54680_e69770) + (assign54680_e69753 * (0.5 * (((-locals.var_temp__blk949_dn6) * assign54680_e69767) + (assign54680_e69759 * ((-locals.var_temp__blk949_dn6) * 0.3333333333333333))))))) / (assign54680_e69772 * assign54680_e69772))), (-((1e-100 * (((-locals.var_temp__blk949_dn7) * assign54680_e69770) + (assign54680_e69753 * (0.5 * (((-locals.var_temp__blk949_dn7) * assign54680_e69767) + (assign54680_e69759 * ((-locals.var_temp__blk949_dn7) * 0.3333333333333333))))))) / (assign54680_e69772 * assign54680_e69772))), (-((1e-100 * (((-locals.var_temp__blk949_dn8) * assign54680_e69770) + (assign54680_e69753 * (0.5 * (((-locals.var_temp__blk949_dn8) * assign54680_e69767) + (assign54680_e69759 * ((-locals.var_temp__blk949_dn8) * 0.3333333333333333))))))) / (assign54680_e69772 * assign54680_e69772))), (-((1e-100 * (((-locals.var_temp__blk949_dn9) * assign54680_e69770) + (assign54680_e69753 * (0.5 * (((-locals.var_temp__blk949_dn9) * assign54680_e69767) + (assign54680_e69759 * ((-locals.var_temp__blk949_dn9) * 0.3333333333333333))))))) / (assign54680_e69772 * assign54680_e69772))), );
            locals.var_yb_ov_d_rv = 0.0;
        }

        let assign54690_e69778: f64 = if locals.var_yb_ov_d > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1541 = assign54690_e69778;
        locals.var_guard1541_rv = 0.0;

        if (((locals.var_guard1538 != 0.0) && (locals.var_guard1539 != 0.0)) && (locals.var_guard1541 != 0.0)) {
            let assign54700_e69786: f64 = (1.0 + locals.var_yb_ov_d);
            let assign54700_e69787: f64 = (assign54700_e69786).ln();
            (locals.var_xgbeff_ov_d, locals.var_xgbeff_ov_d_dn4, locals.var_xgbeff_ov_d_dn6, locals.var_xgbeff_ov_d_dn7, locals.var_xgbeff_ov_d_dn8, locals.var_xgbeff_ov_d_dn9, ) = (assign54700_e69787, (locals.var_yb_ov_d_dn4 / assign54700_e69786), (locals.var_yb_ov_d_dn6 / assign54700_e69786), (locals.var_yb_ov_d_dn7 / assign54700_e69786), (locals.var_yb_ov_d_dn8 / assign54700_e69786), (locals.var_yb_ov_d_dn9 / assign54700_e69786), );
            locals.var_xgbeff_ov_d_rv = 0.0;
        }

        if (((locals.var_guard1538 != 0.0) && (locals.var_guard1539 != 0.0)) && (locals.var_guard1541 != 0.0)) {
            let assign54710_e69799: f64 = (1.0 + locals.var_xgbeff_ov_d);
            let assign54710_e69800: f64 = (assign54710_e69799).ln();
            let assign54710_e69803: f64 = (2.0 + locals.var_xgbeff_ov_d);
            let assign54710_e69804: f64 = (assign54710_e69800 / assign54710_e69803);
            let assign54710_e69805: f64 = (1.0 - assign54710_e69804);
            let assign54710_e69806: f64 = (locals.var_xgbeff_ov_d * assign54710_e69805);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign54710_e69806, ((locals.var_xgbeff_ov_d_dn4 * assign54710_e69805) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn4 / assign54710_e69799) * assign54710_e69803) - (assign54710_e69800 * locals.var_xgbeff_ov_d_dn4)) / (assign54710_e69803 * assign54710_e69803))))), ((locals.var_xgbeff_ov_d_dn6 * assign54710_e69805) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn6 / assign54710_e69799) * assign54710_e69803) - (assign54710_e69800 * locals.var_xgbeff_ov_d_dn6)) / (assign54710_e69803 * assign54710_e69803))))), ((locals.var_xgbeff_ov_d_dn7 * assign54710_e69805) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn7 / assign54710_e69799) * assign54710_e69803) - (assign54710_e69800 * locals.var_xgbeff_ov_d_dn7)) / (assign54710_e69803 * assign54710_e69803))))), ((locals.var_xgbeff_ov_d_dn8 * assign54710_e69805) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn8 / assign54710_e69799) * assign54710_e69803) - (assign54710_e69800 * locals.var_xgbeff_ov_d_dn8)) / (assign54710_e69803 * assign54710_e69803))))), ((locals.var_xgbeff_ov_d_dn9 * assign54710_e69805) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn9 / assign54710_e69799) * assign54710_e69803) - (assign54710_e69800 * locals.var_xgbeff_ov_d_dn9)) / (assign54710_e69803 * assign54710_e69803))))), );
            locals.var_temp1_rv = 0.0;
        }

        if (((locals.var_guard1538 != 0.0) && (locals.var_guard1539 != 0.0)) && (locals.var_guard1541 == 0.0)) {
            (locals.var_xgbeff_ov_d, locals.var_xgbeff_ov_d_dn4, locals.var_xgbeff_ov_d_dn6, locals.var_xgbeff_ov_d_dn7, locals.var_xgbeff_ov_d_dn8, locals.var_xgbeff_ov_d_dn9, ) = (locals.var_yb_ov_d, locals.var_yb_ov_d_dn4, locals.var_yb_ov_d_dn6, locals.var_yb_ov_d_dn7, locals.var_yb_ov_d_dn8, locals.var_yb_ov_d_dn9, );
            locals.var_xgbeff_ov_d_rv = 0.0;
        }

        if (((locals.var_guard1538 != 0.0) && (locals.var_guard1539 != 0.0)) && (locals.var_guard1541 == 0.0)) {
            let assign54730_e69826: f64 = (2.0 * locals.var_xgbeff_ov_d);
            let assign54730_e69829: f64 = (2.0 + locals.var_xgbeff_ov_d);
            let assign54730_e69830: f64 = (assign54730_e69826 / assign54730_e69829);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign54730_e69830, ((((2.0 * locals.var_xgbeff_ov_d_dn4) * assign54730_e69829) - (assign54730_e69826 * locals.var_xgbeff_ov_d_dn4)) / (assign54730_e69829 * assign54730_e69829)), ((((2.0 * locals.var_xgbeff_ov_d_dn6) * assign54730_e69829) - (assign54730_e69826 * locals.var_xgbeff_ov_d_dn6)) / (assign54730_e69829 * assign54730_e69829)), ((((2.0 * locals.var_xgbeff_ov_d_dn7) * assign54730_e69829) - (assign54730_e69826 * locals.var_xgbeff_ov_d_dn7)) / (assign54730_e69829 * assign54730_e69829)), ((((2.0 * locals.var_xgbeff_ov_d_dn8) * assign54730_e69829) - (assign54730_e69826 * locals.var_xgbeff_ov_d_dn8)) / (assign54730_e69829 * assign54730_e69829)), ((((2.0 * locals.var_xgbeff_ov_d_dn9) * assign54730_e69829) - (assign54730_e69826 * locals.var_xgbeff_ov_d_dn9)) / (assign54730_e69829 * assign54730_e69829)), );
            locals.var_temp1_rv = 0.0;
        }

        if ((locals.var_guard1538 != 0.0) && (locals.var_guard1539 == 0.0)) {
            (locals.var_xgbeff_ov_d, locals.var_xgbeff_ov_d_dn4, locals.var_xgbeff_ov_d_dn6, locals.var_xgbeff_ov_d_dn7, locals.var_xgbeff_ov_d_dn8, locals.var_xgbeff_ov_d_dn9, ) = (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9, );
            locals.var_xgbeff_ov_d_rv = 0.0;
        }

        if ((locals.var_guard1538 != 0.0) && (locals.var_guard1539 == 0.0)) {
            let assign54750_e69848: f64 = (1.0 + locals.var_xgbeff_ov_d);
            let assign54750_e69849: f64 = (assign54750_e69848).ln();
            let assign54750_e69852: f64 = (2.0 + locals.var_xgbeff_ov_d);
            let assign54750_e69853: f64 = (assign54750_e69849 / assign54750_e69852);
            let assign54750_e69854: f64 = (1.0 - assign54750_e69853);
            let assign54750_e69855: f64 = (locals.var_xgbeff_ov_d * assign54750_e69854);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign54750_e69855, ((locals.var_xgbeff_ov_d_dn4 * assign54750_e69854) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn4 / assign54750_e69848) * assign54750_e69852) - (assign54750_e69849 * locals.var_xgbeff_ov_d_dn4)) / (assign54750_e69852 * assign54750_e69852))))), ((locals.var_xgbeff_ov_d_dn6 * assign54750_e69854) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn6 / assign54750_e69848) * assign54750_e69852) - (assign54750_e69849 * locals.var_xgbeff_ov_d_dn6)) / (assign54750_e69852 * assign54750_e69852))))), ((locals.var_xgbeff_ov_d_dn7 * assign54750_e69854) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn7 / assign54750_e69848) * assign54750_e69852) - (assign54750_e69849 * locals.var_xgbeff_ov_d_dn7)) / (assign54750_e69852 * assign54750_e69852))))), ((locals.var_xgbeff_ov_d_dn8 * assign54750_e69854) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn8 / assign54750_e69848) * assign54750_e69852) - (assign54750_e69849 * locals.var_xgbeff_ov_d_dn8)) / (assign54750_e69852 * assign54750_e69852))))), ((locals.var_xgbeff_ov_d_dn9 * assign54750_e69854) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn9 / assign54750_e69848) * assign54750_e69852) - (assign54750_e69849 * locals.var_xgbeff_ov_d_dn9)) / (assign54750_e69852 * assign54750_e69852))))), );
            locals.var_temp1_rv = 0.0;
        }

        if (locals.var_guard1538 != 0.0) {
            let assign54760_e69860: f64 = (-2.0);
            let assign54760_e69862: f64 = (assign54760_e69860 * locals.var_fcgovaccd_i);
            let assign54760_e69864: f64 = (assign54760_e69862 / locals.var_cgovaccg_i);
            let assign54760_e69866: f64 = (assign54760_e69864 * locals.var_cgovd_i);
            let assign54760_e69868: f64 = (assign54760_e69866 * locals.var_phita);
            let assign54760_e69870: f64 = (assign54760_e69868 * locals.var_temp1);
            (locals.var_qg_ov_d, locals.var_qg_ov_d_dn4, locals.var_qg_ov_d_dn6, locals.var_qg_ov_d_dn7, locals.var_qg_ov_d_dn8, locals.var_qg_ov_d_dn9, ) = (assign54760_e69870, (assign54760_e69868 * locals.var_temp1_dn4), (assign54760_e69868 * locals.var_temp1_dn6), (assign54760_e69868 * locals.var_temp1_dn7), (assign54760_e69868 * locals.var_temp1_dn8), (assign54760_e69868 * locals.var_temp1_dn9), );
            locals.var_qg_ov_d_rv = 0.0;
        }

        let assign54770_e69875: f64 = (locals.var_qg_ov_s + locals.var_qg_ov_d);
        (locals.var_qg_ov, locals.var_qg_ov_dn4, locals.var_qg_ov_dn6, locals.var_qg_ov_dn7, locals.var_qg_ov_dn8, locals.var_qg_ov_dn9, ) = (assign54770_e69875, (locals.var_qg_ov_s_dn4 + locals.var_qg_ov_d_dn4), (locals.var_qg_ov_s_dn6 + locals.var_qg_ov_d_dn6), (locals.var_qg_ov_s_dn7 + locals.var_qg_ov_d_dn7), (locals.var_qg_ov_s_dn8 + locals.var_qg_ov_d_dn8), (locals.var_qg_ov_s_dn9 + locals.var_qg_ov_d_dn9), );
        locals.var_qg_ov_rv = 0.0;

        let assign54780_e69878: f64 = (locals.var_cgbov_i * locals.var_vgb);
        let assign54780_e69880: f64 = (assign54780_e69878 + locals.var_qg_ov);
        (locals.var_qgb_ov, locals.var_qgb_ov_dn4, locals.var_qgb_ov_dn6, locals.var_qgb_ov_dn7, locals.var_qgb_ov_dn8, locals.var_qgb_ov_dn9, ) = (assign54780_e69880, locals.var_qg_ov_dn4, ((locals.var_cgbov_i * locals.var_vgb_dn6) + locals.var_qg_ov_dn6), ((locals.var_cgbov_i * locals.var_vgb_dn7) + locals.var_qg_ov_dn7), ((locals.var_cgbov_i * locals.var_vgb_dn8) + locals.var_qg_ov_dn8), ((locals.var_cgbov_i * locals.var_vgb_dn9) + locals.var_qg_ov_dn9), );
        locals.var_qgb_ov_rv = 0.0;

        let assign62240_e80805: f64 = (locals.var_qg + locals.var_qb);
        let assign62240_e80807: f64 = (assign62240_e80805 + locals.var_qd);
        let assign62240_e80808: f64 = (-assign62240_e80807);
        (locals.var_qs, locals.var_qs_dn4, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, ) = (assign62240_e80808, (-((locals.var_qg_dn4 + locals.var_qb_dn4) + locals.var_qd_dn4)), (-((locals.var_qg_dn6 + locals.var_qb_dn6) + locals.var_qd_dn6)), (-((locals.var_qg_dn7 + locals.var_qb_dn7) + locals.var_qd_dn7)), (-((locals.var_qg_dn8 + locals.var_qb_dn8) + locals.var_qd_dn8)), (-((locals.var_qg_dn9 + locals.var_qb_dn9) + locals.var_qd_dn9)), );
        locals.var_qs_rv = 0.0;

        let assign62290_e80839: f64 = if locals.var_sigvds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1749 = assign62290_e80839;
        locals.var_guard1749_rv = 0.0;

        if (locals.var_guard1749 != 0.0) {
            (locals.var_temp__blk1748, locals.var_temp__blk1748_dn4, locals.var_temp__blk1748_dn6, locals.var_temp__blk1748_dn7, locals.var_temp__blk1748_dn8, locals.var_temp__blk1748_dn9, ) = (locals.var_qd, locals.var_qd_dn4, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, );
            locals.var_temp__blk1748_rv = 0.0;
            (locals.var_qd, locals.var_qd_dn4, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, ) = (locals.var_qs, locals.var_qs_dn4, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, );
            locals.var_qd_rv = 0.0;
            (locals.var_qs, locals.var_qs_dn4, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, ) = (locals.var_temp__blk1748, locals.var_temp__blk1748_dn4, locals.var_temp__blk1748_dn6, locals.var_temp__blk1748_dn7, locals.var_temp__blk1748_dn8, locals.var_temp__blk1748_dn9, );
            locals.var_qs_rv = 0.0;
        }

        let assign62390_e80860: f64 = (locals.var_cox_qm * locals.var_eta_p_ac);
        (locals.var_cgeff, locals.var_cgeff_dn4, locals.var_cgeff_dn6, locals.var_cgeff_dn7, locals.var_cgeff_dn8, locals.var_cgeff_dn9, ) = (assign62390_e80860, ((locals.var_cox_qm_dn4 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn4)), ((locals.var_cox_qm_dn6 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn6)), ((locals.var_cox_qm_dn7 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn7)), ((locals.var_cox_qm_dn8 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn8)), ((locals.var_cox_qm_dn9 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn9)), );
        locals.var_cgeff_rv = 0.0;

        let assign62450_e80872: f64 = if ((locals.var_xg_dc > 0.0) && (locals.var_bet_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1782 = assign62450_e80872;
        locals.var_guard1782_rv = 0.0;

        let assign62760_e81238: f64 = if ((((p.p50 == 1.0) && (locals.var_nt > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1787 = assign62760_e81238;
        locals.var_guard1787_rv = 0.0;

        if ((locals.var_guard1782 != 0.0) && (locals.var_guard1787 != 0.0)) {
            let assign62810_e81325: f64 = (locals.var_gvsat_ac * locals.var_gvsat_ac);
            let assign62810_e81327: f64 = (assign62810_e81325 * locals.var_cox_qm);
            let assign62810_e81329: f64 = (assign62810_e81327 * locals.var_eta_p_ac);
            let assign62810_e81332: f64 = (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac);
            let assign62810_e81333: f64 = (assign62810_e81329 / assign62810_e81332);
            (locals.var_cgeff, locals.var_cgeff_dn4, locals.var_cgeff_dn6, locals.var_cgeff_dn7, locals.var_cgeff_dn8, locals.var_cgeff_dn9, ) = (assign62810_e81333, (((((((((locals.var_gvsat_ac_dn4 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn4)) * locals.var_cox_qm) + (assign62810_e81325 * locals.var_cox_qm_dn4)) * locals.var_eta_p_ac) + (assign62810_e81327 * locals.var_eta_p_ac_dn4)) * assign62810_e81332) - (assign62810_e81329 * ((locals.var_gmob_dl_ac_dn4 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn4)))) / (assign62810_e81332 * assign62810_e81332)), (((((((((locals.var_gvsat_ac_dn6 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn6)) * locals.var_cox_qm) + (assign62810_e81325 * locals.var_cox_qm_dn6)) * locals.var_eta_p_ac) + (assign62810_e81327 * locals.var_eta_p_ac_dn6)) * assign62810_e81332) - (assign62810_e81329 * ((locals.var_gmob_dl_ac_dn6 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn6)))) / (assign62810_e81332 * assign62810_e81332)), (((((((((locals.var_gvsat_ac_dn7 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn7)) * locals.var_cox_qm) + (assign62810_e81325 * locals.var_cox_qm_dn7)) * locals.var_eta_p_ac) + (assign62810_e81327 * locals.var_eta_p_ac_dn7)) * assign62810_e81332) - (assign62810_e81329 * ((locals.var_gmob_dl_ac_dn7 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn7)))) / (assign62810_e81332 * assign62810_e81332)), (((((((((locals.var_gvsat_ac_dn8 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn8)) * locals.var_cox_qm) + (assign62810_e81325 * locals.var_cox_qm_dn8)) * locals.var_eta_p_ac) + (assign62810_e81327 * locals.var_eta_p_ac_dn8)) * assign62810_e81332) - (assign62810_e81329 * ((locals.var_gmob_dl_ac_dn8 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn8)))) / (assign62810_e81332 * assign62810_e81332)), (((((((((locals.var_gvsat_ac_dn9 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn9)) * locals.var_cox_qm) + (assign62810_e81325 * locals.var_cox_qm_dn9)) * locals.var_eta_p_ac) + (assign62810_e81327 * locals.var_eta_p_ac_dn9)) * assign62810_e81332) - (assign62810_e81329 * ((locals.var_gmob_dl_ac_dn9 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn9)))) / (assign62810_e81332 * assign62810_e81332)), );
            locals.var_cgeff_rv = 0.0;
        }

        let assign63070_e81549: f64 = if (((p.p46 != 0.0) && (locals.var_betnedge_i > 0.0)) && (locals.var_xgedge > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1791 = assign63070_e81549;
        locals.var_guard1791_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_31(
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard1791 != 0.0) {
            let assign63080_e81553: f64 = (4.0 * locals.var_dsqredge);
            let assign63080_e81555: f64 = (assign63080_e81553 / locals.var_gfedge2);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign63080_e81555, ((((4.0 * locals.var_dsqredge_dn4) * locals.var_gfedge2) - (assign63080_e81553 * locals.var_gfedge2_dn4)) / (locals.var_gfedge2 * locals.var_gfedge2)), ((4.0 * locals.var_dsqredge_dn6) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn7) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn8) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn9) / locals.var_gfedge2), );
            locals.var_temp1_rv = 0.0;
        }

        if (locals.var_guard1791 != 0.0) {
            let assign63100_e81575: f64 = (locals.var_cox_over_q * locals.var_phit);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign63100_e81575, (locals.var_cox_over_q * locals.var_phit_dn4), 0.0, 0.0, 0.0, 0.0, );
            locals.var_temp1_rv = 0.0;
        }

        if (locals.var_guard1791 != 0.0) {
            let assign63230_e81715: f64 = (locals.var_alpha_dc * locals.var_h_dc);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign63230_e81715, ((locals.var_alpha_dc_dn4 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn4)), ((locals.var_alpha_dc_dn6 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn6)), ((locals.var_alpha_dc_dn7 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn7)), ((locals.var_alpha_dc_dn8 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn8)), ((locals.var_alpha_dc_dn9 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn9)), );
            locals.var_temp1_rv = 0.0;
        }

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq0_e972, eq0_e972_d_n4, eq0_e972_d_n6, eq0_e972_d_n7, eq0_e972_d_n8, eq0_e972_d_n9,) = {
    if (locals.var_guard1735 != 0.0) {
        let eq0_e966: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq0_e968: f64 = (eq0_e966 * p.p32);
        let eq0_e970: f64 = (eq0_e968 * locals.var_iimpact);
        let eq0_e970_d_n4: f64 = (eq0_e968 * locals.var_iimpact_dn4);
        let eq0_e970_d_n6: f64 = (eq0_e968 * locals.var_iimpact_dn6);
        let eq0_e970_d_n7: f64 = (eq0_e968 * locals.var_iimpact_dn7);
        let eq0_e970_d_n8: f64 = (eq0_e968 * locals.var_iimpact_dn8);
        let eq0_e970_d_n9: f64 = (eq0_e968 * locals.var_iimpact_dn9);
        (eq0_e970, eq0_e970_d_n4, eq0_e970_d_n6, eq0_e970_d_n7, eq0_e970_d_n8, eq0_e970_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e972;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(9),
            multiplicity * (eq0_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq0_e972_d_n4), multiplicity * (eq0_e972_d_n6), multiplicity * (eq0_e972_d_n7), multiplicity * (eq0_e972_d_n8), multiplicity * (eq0_e972_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq1_e984, eq1_e984_d_n4, eq1_e984_d_n6, eq1_e984_d_n7, eq1_e984_d_n8, eq1_e984_d_n9,) = {
    if (locals.var_guard1735 != 0.0) {
        let eq1_e976: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq1_e978: f64 = (eq1_e976 * p.p32);
        let eq1_e981: f64 = (locals.var_i_ds + locals.var_i_dsedge);
        let eq1_e981_d_n4: f64 = (locals.var_i_ds_dn4 + locals.var_i_dsedge_dn4);
        let eq1_e981_d_n6: f64 = (locals.var_i_ds_dn6 + locals.var_i_dsedge_dn6);
        let eq1_e981_d_n7: f64 = (locals.var_i_ds_dn7 + locals.var_i_dsedge_dn7);
        let eq1_e981_d_n8: f64 = (locals.var_i_ds_dn8 + locals.var_i_dsedge_dn8);
        let eq1_e981_d_n9: f64 = (locals.var_i_ds_dn9 + locals.var_i_dsedge_dn9);
        let eq1_e982: f64 = (eq1_e978 * eq1_e981);
        let eq1_e982_d_n4: f64 = (eq1_e978 * eq1_e981_d_n4);
        let eq1_e982_d_n6: f64 = (eq1_e978 * eq1_e981_d_n6);
        let eq1_e982_d_n7: f64 = (eq1_e978 * eq1_e981_d_n7);
        let eq1_e982_d_n8: f64 = (eq1_e978 * eq1_e981_d_n8);
        let eq1_e982_d_n9: f64 = (eq1_e978 * eq1_e981_d_n9);
        (eq1_e982, eq1_e982_d_n4, eq1_e982_d_n6, eq1_e982_d_n7, eq1_e982_d_n8, eq1_e982_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e984;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq1_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq1_e984_d_n4), multiplicity * (eq1_e984_d_n6), multiplicity * (eq1_e984_d_n7), multiplicity * (eq1_e984_d_n8), multiplicity * (eq1_e984_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq2_e994, eq2_e994_d_n4, eq2_e994_d_n6, eq2_e994_d_n7, eq2_e994_d_n8, eq2_e994_d_n9,) = {
    if (locals.var_guard1735 != 0.0) {
        let eq2_e988: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq2_e990: f64 = (eq2_e988 * p.p32);
        let eq2_e992: f64 = (eq2_e990 * locals.var_i_gcs);
        let eq2_e992_d_n4: f64 = (eq2_e990 * locals.var_i_gcs_dn4);
        let eq2_e992_d_n6: f64 = (eq2_e990 * locals.var_i_gcs_dn6);
        let eq2_e992_d_n7: f64 = (eq2_e990 * locals.var_i_gcs_dn7);
        let eq2_e992_d_n8: f64 = (eq2_e990 * locals.var_i_gcs_dn8);
        let eq2_e992_d_n9: f64 = (eq2_e990 * locals.var_i_gcs_dn9);
        (eq2_e992, eq2_e992_d_n4, eq2_e992_d_n6, eq2_e992_d_n7, eq2_e992_d_n8, eq2_e992_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e994;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq2_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq2_e994_d_n4), multiplicity * (eq2_e994_d_n6), multiplicity * (eq2_e994_d_n7), multiplicity * (eq2_e994_d_n8), multiplicity * (eq2_e994_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq3_e1004, eq3_e1004_d_n4, eq3_e1004_d_n6, eq3_e1004_d_n7, eq3_e1004_d_n8, eq3_e1004_d_n9,) = {
    if (locals.var_guard1735 != 0.0) {
        let eq3_e998: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq3_e1000: f64 = (eq3_e998 * p.p32);
        let eq3_e1002: f64 = (eq3_e1000 * locals.var_i_gcd);
        let eq3_e1002_d_n4: f64 = (eq3_e1000 * locals.var_i_gcd_dn4);
        let eq3_e1002_d_n6: f64 = (eq3_e1000 * locals.var_i_gcd_dn6);
        let eq3_e1002_d_n7: f64 = (eq3_e1000 * locals.var_i_gcd_dn7);
        let eq3_e1002_d_n8: f64 = (eq3_e1000 * locals.var_i_gcd_dn8);
        let eq3_e1002_d_n9: f64 = (eq3_e1000 * locals.var_i_gcd_dn9);
        (eq3_e1002, eq3_e1002_d_n4, eq3_e1002_d_n6, eq3_e1002_d_n7, eq3_e1002_d_n8, eq3_e1002_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e1004;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq3_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq3_e1004_d_n4), multiplicity * (eq3_e1004_d_n6), multiplicity * (eq3_e1004_d_n7), multiplicity * (eq3_e1004_d_n8), multiplicity * (eq3_e1004_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq4_e1015, eq4_e1015_d_n4, eq4_e1015_d_n6, eq4_e1015_d_n7, eq4_e1015_d_n8, eq4_e1015_d_n9,) = {
    if (locals.var_guard1735 == 0.0) {
        let eq4_e1009: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq4_e1011: f64 = (eq4_e1009 * p.p32);
        let eq4_e1013: f64 = (eq4_e1011 * locals.var_iimpact);
        let eq4_e1013_d_n4: f64 = (eq4_e1011 * locals.var_iimpact_dn4);
        let eq4_e1013_d_n6: f64 = (eq4_e1011 * locals.var_iimpact_dn6);
        let eq4_e1013_d_n7: f64 = (eq4_e1011 * locals.var_iimpact_dn7);
        let eq4_e1013_d_n8: f64 = (eq4_e1011 * locals.var_iimpact_dn8);
        let eq4_e1013_d_n9: f64 = (eq4_e1011 * locals.var_iimpact_dn9);
        (eq4_e1013, eq4_e1013_d_n4, eq4_e1013_d_n6, eq4_e1013_d_n7, eq4_e1013_d_n8, eq4_e1013_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1015;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq4_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq4_e1015_d_n4), multiplicity * (eq4_e1015_d_n6), multiplicity * (eq4_e1015_d_n7), multiplicity * (eq4_e1015_d_n8), multiplicity * (eq4_e1015_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq5_e1028, eq5_e1028_d_n4, eq5_e1028_d_n6, eq5_e1028_d_n7, eq5_e1028_d_n8, eq5_e1028_d_n9,) = {
    if (locals.var_guard1735 == 0.0) {
        let eq5_e1020: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq5_e1022: f64 = (eq5_e1020 * p.p32);
        let eq5_e1025: f64 = (locals.var_i_ds + locals.var_i_dsedge);
        let eq5_e1025_d_n4: f64 = (locals.var_i_ds_dn4 + locals.var_i_dsedge_dn4);
        let eq5_e1025_d_n6: f64 = (locals.var_i_ds_dn6 + locals.var_i_dsedge_dn6);
        let eq5_e1025_d_n7: f64 = (locals.var_i_ds_dn7 + locals.var_i_dsedge_dn7);
        let eq5_e1025_d_n8: f64 = (locals.var_i_ds_dn8 + locals.var_i_dsedge_dn8);
        let eq5_e1025_d_n9: f64 = (locals.var_i_ds_dn9 + locals.var_i_dsedge_dn9);
        let eq5_e1026: f64 = (eq5_e1022 * eq5_e1025);
        let eq5_e1026_d_n4: f64 = (eq5_e1022 * eq5_e1025_d_n4);
        let eq5_e1026_d_n6: f64 = (eq5_e1022 * eq5_e1025_d_n6);
        let eq5_e1026_d_n7: f64 = (eq5_e1022 * eq5_e1025_d_n7);
        let eq5_e1026_d_n8: f64 = (eq5_e1022 * eq5_e1025_d_n8);
        let eq5_e1026_d_n9: f64 = (eq5_e1022 * eq5_e1025_d_n9);
        (eq5_e1026, eq5_e1026_d_n4, eq5_e1026_d_n6, eq5_e1026_d_n7, eq5_e1026_d_n8, eq5_e1026_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1028;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(8),
            multiplicity * (eq5_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq5_e1028_d_n4), multiplicity * (eq5_e1028_d_n6), multiplicity * (eq5_e1028_d_n7), multiplicity * (eq5_e1028_d_n8), multiplicity * (eq5_e1028_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq6_e1039, eq6_e1039_d_n4, eq6_e1039_d_n6, eq6_e1039_d_n7, eq6_e1039_d_n8, eq6_e1039_d_n9,) = {
    if (locals.var_guard1735 == 0.0) {
        let eq6_e1033: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq6_e1035: f64 = (eq6_e1033 * p.p32);
        let eq6_e1037: f64 = (eq6_e1035 * locals.var_i_gcs);
        let eq6_e1037_d_n4: f64 = (eq6_e1035 * locals.var_i_gcs_dn4);
        let eq6_e1037_d_n6: f64 = (eq6_e1035 * locals.var_i_gcs_dn6);
        let eq6_e1037_d_n7: f64 = (eq6_e1035 * locals.var_i_gcs_dn7);
        let eq6_e1037_d_n8: f64 = (eq6_e1035 * locals.var_i_gcs_dn8);
        let eq6_e1037_d_n9: f64 = (eq6_e1035 * locals.var_i_gcs_dn9);
        (eq6_e1037, eq6_e1037_d_n4, eq6_e1037_d_n6, eq6_e1037_d_n7, eq6_e1037_d_n8, eq6_e1037_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1039;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq6_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq6_e1039_d_n4), multiplicity * (eq6_e1039_d_n6), multiplicity * (eq6_e1039_d_n7), multiplicity * (eq6_e1039_d_n8), multiplicity * (eq6_e1039_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq7_e1050, eq7_e1050_d_n4, eq7_e1050_d_n6, eq7_e1050_d_n7, eq7_e1050_d_n8, eq7_e1050_d_n9,) = {
    if (locals.var_guard1735 == 0.0) {
        let eq7_e1044: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq7_e1046: f64 = (eq7_e1044 * p.p32);
        let eq7_e1048: f64 = (eq7_e1046 * locals.var_i_gcd);
        let eq7_e1048_d_n4: f64 = (eq7_e1046 * locals.var_i_gcd_dn4);
        let eq7_e1048_d_n6: f64 = (eq7_e1046 * locals.var_i_gcd_dn6);
        let eq7_e1048_d_n7: f64 = (eq7_e1046 * locals.var_i_gcd_dn7);
        let eq7_e1048_d_n8: f64 = (eq7_e1046 * locals.var_i_gcd_dn8);
        let eq7_e1048_d_n9: f64 = (eq7_e1046 * locals.var_i_gcd_dn9);
        (eq7_e1048, eq7_e1048_d_n4, eq7_e1048_d_n6, eq7_e1048_d_n7, eq7_e1048_d_n8, eq7_e1048_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1050;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq7_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq7_e1050_d_n4), multiplicity * (eq7_e1050_d_n6), multiplicity * (eq7_e1050_d_n7), multiplicity * (eq7_e1050_d_n8), multiplicity * (eq7_e1050_d_n9)],
            [],
            [],
            1.0,
        );
        let eq8_e1053: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq8_e1055: f64 = (eq8_e1053 * p.p32);
        let eq8_e1057: f64 = (eq8_e1055 * locals.var_i_gb);
        let eq8_e1057_d_n4: f64 = (eq8_e1055 * locals.var_i_gb_dn4);
        let eq8_e1057_d_n6: f64 = (eq8_e1055 * locals.var_i_gb_dn6);
        let eq8_e1057_d_n7: f64 = (eq8_e1055 * locals.var_i_gb_dn7);
        let eq8_e1057_d_n8: f64 = (eq8_e1055 * locals.var_i_gb_dn8);
        let eq8_e1057_d_n9: f64 = (eq8_e1055 * locals.var_i_gb_dn9);
        let eq8_value: f64 = eq8_e1057;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(9),
            multiplicity * (eq8_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq8_e1057_d_n4), multiplicity * (eq8_e1057_d_n6), multiplicity * (eq8_e1057_d_n7), multiplicity * (eq8_e1057_d_n8), multiplicity * (eq8_e1057_d_n9)],
            [],
            [],
            1.0,
        );
        let eq9_e1060: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq9_e1062: f64 = (eq9_e1060 * p.p32);
        let eq9_e1064: f64 = (eq9_e1062 * locals.var_igsov);
        let eq9_e1064_d_n4: f64 = (eq9_e1062 * locals.var_igsov_dn4);
        let eq9_e1064_d_n6: f64 = (eq9_e1062 * locals.var_igsov_dn6);
        let eq9_e1064_d_n7: f64 = (eq9_e1062 * locals.var_igsov_dn7);
        let eq9_e1064_d_n8: f64 = (eq9_e1062 * locals.var_igsov_dn8);
        let eq9_e1064_d_n9: f64 = (eq9_e1062 * locals.var_igsov_dn9);
        let eq9_value: f64 = eq9_e1064;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq9_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq9_e1064_d_n4), multiplicity * (eq9_e1064_d_n6), multiplicity * (eq9_e1064_d_n7), multiplicity * (eq9_e1064_d_n8), multiplicity * (eq9_e1064_d_n9)],
            [],
            [],
            1.0,
        );
        let eq10_e1067: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq10_e1069: f64 = (eq10_e1067 * p.p32);
        let eq10_e1071: f64 = (eq10_e1069 * locals.var_igdov);
        let eq10_e1071_d_n4: f64 = (eq10_e1069 * locals.var_igdov_dn4);
        let eq10_e1071_d_n6: f64 = (eq10_e1069 * locals.var_igdov_dn6);
        let eq10_e1071_d_n7: f64 = (eq10_e1069 * locals.var_igdov_dn7);
        let eq10_e1071_d_n8: f64 = (eq10_e1069 * locals.var_igdov_dn8);
        let eq10_e1071_d_n9: f64 = (eq10_e1069 * locals.var_igdov_dn9);
        let eq10_value: f64 = eq10_e1071;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq10_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq10_e1071_d_n4), multiplicity * (eq10_e1071_d_n6), multiplicity * (eq10_e1071_d_n7), multiplicity * (eq10_e1071_d_n8), multiplicity * (eq10_e1071_d_n9)],
            [],
            [],
            1.0,
        );
        let eq11_e1074: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq11_e1076: f64 = (eq11_e1074 * p.p32);
        let eq11_e1078: f64 = (eq11_e1076 * locals.var_i_gisl);
        let eq11_e1078_d_n4: f64 = (eq11_e1076 * locals.var_i_gisl_dn4);
        let eq11_e1078_d_n6: f64 = (eq11_e1076 * locals.var_i_gisl_dn6);
        let eq11_e1078_d_n7: f64 = (eq11_e1076 * locals.var_i_gisl_dn7);
        let eq11_e1078_d_n8: f64 = (eq11_e1076 * locals.var_i_gisl_dn8);
        let eq11_e1078_d_n9: f64 = (eq11_e1076 * locals.var_i_gisl_dn9);
        let eq11_value: f64 = eq11_e1078;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq11_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq11_e1078_d_n4), multiplicity * (eq11_e1078_d_n6), multiplicity * (eq11_e1078_d_n7), multiplicity * (eq11_e1078_d_n8), multiplicity * (eq11_e1078_d_n9)],
            [],
            [],
            1.0,
        );
        let eq12_e1081: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq12_e1083: f64 = (eq12_e1081 * p.p32);
        let eq12_e1085: f64 = (eq12_e1083 * locals.var_i_gidl);
        let eq12_e1085_d_n4: f64 = (eq12_e1083 * locals.var_i_gidl_dn4);
        let eq12_e1085_d_n6: f64 = (eq12_e1083 * locals.var_i_gidl_dn6);
        let eq12_e1085_d_n7: f64 = (eq12_e1083 * locals.var_i_gidl_dn7);
        let eq12_e1085_d_n8: f64 = (eq12_e1083 * locals.var_i_gidl_dn8);
        let eq12_e1085_d_n9: f64 = (eq12_e1083 * locals.var_i_gidl_dn9);
        let eq12_value: f64 = eq12_e1085;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(9),
            multiplicity * (eq12_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq12_e1085_d_n4), multiplicity * (eq12_e1085_d_n6), multiplicity * (eq12_e1085_d_n7), multiplicity * (eq12_e1085_d_n8), multiplicity * (eq12_e1085_d_n9)],
            [],
            [],
            1.0,
        );
        let eq38_e1286: f64 = (-locals.var_mult_inst);
        let eq38_e1288: f64 = (eq38_e1286 * locals.var_pdiss_1);
        let eq38_e1288_d_n0: f64 = (eq38_e1286 * locals.var_pdiss_1_dn0);
        let eq38_e1288_d_n2: f64 = (eq38_e1286 * locals.var_pdiss_1_dn2);
        let eq38_e1288_d_n4: f64 = (eq38_e1286 * locals.var_pdiss_1_dn4);
        let eq38_e1288_d_n6: f64 = (eq38_e1286 * locals.var_pdiss_1_dn6);
        let eq38_e1288_d_n7: f64 = (eq38_e1286 * locals.var_pdiss_1_dn7);
        let eq38_e1288_d_n8: f64 = (eq38_e1286 * locals.var_pdiss_1_dn8);
        let eq38_e1288_d_n9: f64 = (eq38_e1286 * locals.var_pdiss_1_dn9);
        let eq38_value: f64 = eq38_e1288;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(4),
            None,
            multiplicity * (eq38_value),
            [0, 2, 4, 6, 7, 8, 9],
            [multiplicity * (eq38_e1288_d_n0), multiplicity * (eq38_e1288_d_n2), multiplicity * (eq38_e1288_d_n4), multiplicity * (eq38_e1288_d_n6), multiplicity * (eq38_e1288_d_n7), multiplicity * (eq38_e1288_d_n8), multiplicity * (eq38_e1288_d_n9)],
            [],
            [],
            1.0,
        );
        let eq41_e1302: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq41_e1304: f64 = (eq41_e1302 * p.p33);
        let eq41_e1306: f64 = (eq41_e1304 * locals.var_qg);
        let eq41_e1306_d_n4: f64 = (eq41_e1304 * locals.var_qg_dn4);
        let eq41_e1306_d_n6: f64 = (eq41_e1304 * locals.var_qg_dn6);
        let eq41_e1306_d_n7: f64 = (eq41_e1304 * locals.var_qg_dn7);
        let eq41_e1306_d_n8: f64 = (eq41_e1304 * locals.var_qg_dn8);
        let eq41_e1306_d_n9: f64 = (eq41_e1304 * locals.var_qg_dn9);
        let eq41_e1307: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq41_e1306);
        let eq41_value: f64 = eq41_e1307;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq41_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq41_e1306_d_n4 * ddt_scale)), multiplicity * ((eq41_e1306_d_n6 * ddt_scale)), multiplicity * ((eq41_e1306_d_n7 * ddt_scale)), multiplicity * ((eq41_e1306_d_n8 * ddt_scale)), multiplicity * ((eq41_e1306_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq42_e1310: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq42_e1312: f64 = (eq42_e1310 * p.p33);
        let eq42_e1314: f64 = (eq42_e1312 * locals.var_qb);
        let eq42_e1314_d_n4: f64 = (eq42_e1312 * locals.var_qb_dn4);
        let eq42_e1314_d_n6: f64 = (eq42_e1312 * locals.var_qb_dn6);
        let eq42_e1314_d_n7: f64 = (eq42_e1312 * locals.var_qb_dn7);
        let eq42_e1314_d_n8: f64 = (eq42_e1312 * locals.var_qb_dn8);
        let eq42_e1314_d_n9: f64 = (eq42_e1312 * locals.var_qb_dn9);
        let eq42_e1315: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq42_e1314);
        let eq42_value: f64 = eq42_e1315;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq42_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq42_e1314_d_n4 * ddt_scale)), multiplicity * ((eq42_e1314_d_n6 * ddt_scale)), multiplicity * ((eq42_e1314_d_n7 * ddt_scale)), multiplicity * ((eq42_e1314_d_n8 * ddt_scale)), multiplicity * ((eq42_e1314_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq43_e1318: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq43_e1320: f64 = (eq43_e1318 * p.p33);
        let eq43_e1322: f64 = (eq43_e1320 * locals.var_qd);
        let eq43_e1322_d_n4: f64 = (eq43_e1320 * locals.var_qd_dn4);
        let eq43_e1322_d_n6: f64 = (eq43_e1320 * locals.var_qd_dn6);
        let eq43_e1322_d_n7: f64 = (eq43_e1320 * locals.var_qd_dn7);
        let eq43_e1322_d_n8: f64 = (eq43_e1320 * locals.var_qd_dn8);
        let eq43_e1322_d_n9: f64 = (eq43_e1320 * locals.var_qd_dn9);
        let eq43_e1323: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq43_e1322);
        let eq43_value: f64 = eq43_e1323;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq43_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq43_e1322_d_n4 * ddt_scale)), multiplicity * ((eq43_e1322_d_n6 * ddt_scale)), multiplicity * ((eq43_e1322_d_n7 * ddt_scale)), multiplicity * ((eq43_e1322_d_n8 * ddt_scale)), multiplicity * ((eq43_e1322_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq46_e1342: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq46_e1344: f64 = (eq46_e1342 * p.p33);
        let eq46_e1346: f64 = (eq46_e1344 * locals.var_qgb_ov);
        let eq46_e1346_d_n4: f64 = (eq46_e1344 * locals.var_qgb_ov_dn4);
        let eq46_e1346_d_n6: f64 = (eq46_e1344 * locals.var_qgb_ov_dn6);
        let eq46_e1346_d_n7: f64 = (eq46_e1344 * locals.var_qgb_ov_dn7);
        let eq46_e1346_d_n8: f64 = (eq46_e1344 * locals.var_qgb_ov_dn8);
        let eq46_e1346_d_n9: f64 = (eq46_e1344 * locals.var_qgb_ov_dn9);
        let eq46_e1347: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq46_e1346);
        let eq46_value: f64 = eq46_e1347;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(9),
            multiplicity * (eq46_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq46_e1346_d_n4 * ddt_scale)), multiplicity * ((eq46_e1346_d_n6 * ddt_scale)), multiplicity * ((eq46_e1346_d_n7 * ddt_scale)), multiplicity * ((eq46_e1346_d_n8 * ddt_scale)), multiplicity * ((eq46_e1346_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq50_e1371: f64 = ((nv5 - 0.0) / locals.var_mig);
        let eq50_e1371_d_n4: f64 = (-(((nv5 - 0.0) * locals.var_mig_dn4) / (locals.var_mig * locals.var_mig)));
        let eq50_e1371_d_n5: f64 = (1.0 / locals.var_mig);
        let eq50_e1371_d_n6: f64 = (-(((nv5 - 0.0) * locals.var_mig_dn6) / (locals.var_mig * locals.var_mig)));
        let eq50_e1371_d_n7: f64 = (-(((nv5 - 0.0) * locals.var_mig_dn7) / (locals.var_mig * locals.var_mig)));
        let eq50_e1371_d_n8: f64 = (-(((nv5 - 0.0) * locals.var_mig_dn8) / (locals.var_mig * locals.var_mig)));
        let eq50_e1371_d_n9: f64 = (-(((nv5 - 0.0) * locals.var_mig_dn9) / (locals.var_mig * locals.var_mig)));
        let eq50_value: f64 = eq50_e1371;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            None,
            multiplicity * (eq50_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq50_e1371_d_n4), multiplicity * (eq50_e1371_d_n5), multiplicity * (eq50_e1371_d_n6), multiplicity * (eq50_e1371_d_n7), multiplicity * (eq50_e1371_d_n8), multiplicity * (eq50_e1371_d_n9)],
            [],
            [],
            1.0,
        );
        let eq51_e1374: f64 = (locals.var_cgeff * (nv5 - 0.0));
        let eq51_e1374_d_n4: f64 = (locals.var_cgeff_dn4 * (nv5 - 0.0));
        let eq51_e1374_d_n6: f64 = (locals.var_cgeff_dn6 * (nv5 - 0.0));
        let eq51_e1374_d_n7: f64 = (locals.var_cgeff_dn7 * (nv5 - 0.0));
        let eq51_e1374_d_n8: f64 = (locals.var_cgeff_dn8 * (nv5 - 0.0));
        let eq51_e1374_d_n9: f64 = (locals.var_cgeff_dn9 * (nv5 - 0.0));
        let eq51_e1375: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq51_e1374);
        let eq51_value: f64 = eq51_e1375;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            None,
            multiplicity * (eq51_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq51_e1374_d_n4 * ddt_scale)), multiplicity * ((locals.var_cgeff * ddt_scale)), multiplicity * ((eq51_e1374_d_n6 * ddt_scale)), multiplicity * ((eq51_e1374_d_n7 * ddt_scale)), multiplicity * ((eq51_e1374_d_n8 * ddt_scale)), multiplicity * ((eq51_e1374_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq52_e1378: f64 = (locals.var_mult_inst * p.p32);
        let eq52_e1379: f64 = (eq52_e1378).sqrt();
        let eq52_e1381: f64 = (eq52_e1379 * 0.5);
        let eq52_e1383: f64 = (eq52_e1381 * locals.var_cgeff);
        let eq52_e1383_d_n4: f64 = (eq52_e1381 * locals.var_cgeff_dn4);
        let eq52_e1383_d_n6: f64 = (eq52_e1381 * locals.var_cgeff_dn6);
        let eq52_e1383_d_n7: f64 = (eq52_e1381 * locals.var_cgeff_dn7);
        let eq52_e1383_d_n8: f64 = (eq52_e1381 * locals.var_cgeff_dn8);
        let eq52_e1383_d_n9: f64 = (eq52_e1381 * locals.var_cgeff_dn9);
        let eq52_e1385: f64 = (eq52_e1383 * (nv5 - 0.0));
        let eq52_e1385_d_n4: f64 = (eq52_e1383_d_n4 * (nv5 - 0.0));
        let eq52_e1385_d_n6: f64 = (eq52_e1383_d_n6 * (nv5 - 0.0));
        let eq52_e1385_d_n7: f64 = (eq52_e1383_d_n7 * (nv5 - 0.0));
        let eq52_e1385_d_n8: f64 = (eq52_e1383_d_n8 * (nv5 - 0.0));
        let eq52_e1385_d_n9: f64 = (eq52_e1383_d_n9 * (nv5 - 0.0));
        let eq52_e1386: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq52_e1385);
        let eq52_e1387: f64 = (-eq52_e1386);
        let eq52_e1387_d_n4: f64 = (-(eq52_e1385_d_n4 * ddt_scale));
        let eq52_e1387_d_n5: f64 = (-(eq52_e1383 * ddt_scale));
        let eq52_e1387_d_n6: f64 = (-(eq52_e1385_d_n6 * ddt_scale));
        let eq52_e1387_d_n7: f64 = (-(eq52_e1385_d_n7 * ddt_scale));
        let eq52_e1387_d_n8: f64 = (-(eq52_e1385_d_n8 * ddt_scale));
        let eq52_e1387_d_n9: f64 = (-(eq52_e1385_d_n9 * ddt_scale));
        let eq52_value: f64 = eq52_e1387;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq52_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq52_e1387_d_n4), multiplicity * (eq52_e1387_d_n5), multiplicity * (eq52_e1387_d_n6), multiplicity * (eq52_e1387_d_n7), multiplicity * (eq52_e1387_d_n8), multiplicity * (eq52_e1387_d_n9)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq53_e1390: f64 = (locals.var_mult_inst * p.p32);
        let eq53_e1391: f64 = (eq53_e1390).sqrt();
        let eq53_e1393: f64 = (eq53_e1391 * 0.5);
        let eq53_e1395: f64 = (eq53_e1393 * locals.var_cgeff);
        let eq53_e1395_d_n4: f64 = (eq53_e1393 * locals.var_cgeff_dn4);
        let eq53_e1395_d_n6: f64 = (eq53_e1393 * locals.var_cgeff_dn6);
        let eq53_e1395_d_n7: f64 = (eq53_e1393 * locals.var_cgeff_dn7);
        let eq53_e1395_d_n8: f64 = (eq53_e1393 * locals.var_cgeff_dn8);
        let eq53_e1395_d_n9: f64 = (eq53_e1393 * locals.var_cgeff_dn9);
        let eq53_e1397: f64 = (eq53_e1395 * (nv5 - 0.0));
        let eq53_e1397_d_n4: f64 = (eq53_e1395_d_n4 * (nv5 - 0.0));
        let eq53_e1397_d_n6: f64 = (eq53_e1395_d_n6 * (nv5 - 0.0));
        let eq53_e1397_d_n7: f64 = (eq53_e1395_d_n7 * (nv5 - 0.0));
        let eq53_e1397_d_n8: f64 = (eq53_e1395_d_n8 * (nv5 - 0.0));
        let eq53_e1397_d_n9: f64 = (eq53_e1395_d_n9 * (nv5 - 0.0));
        let eq53_e1398: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq53_e1397);
        let eq53_e1399: f64 = (-eq53_e1398);
        let eq53_e1399_d_n4: f64 = (-(eq53_e1397_d_n4 * ddt_scale));
        let eq53_e1399_d_n5: f64 = (-(eq53_e1395 * ddt_scale));
        let eq53_e1399_d_n6: f64 = (-(eq53_e1397_d_n6 * ddt_scale));
        let eq53_e1399_d_n7: f64 = (-(eq53_e1397_d_n7 * ddt_scale));
        let eq53_e1399_d_n8: f64 = (-(eq53_e1397_d_n8 * ddt_scale));
        let eq53_e1399_d_n9: f64 = (-(eq53_e1397_d_n9 * ddt_scale));
        let eq53_value: f64 = eq53_e1399;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq53_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq53_e1399_d_n4), multiplicity * (eq53_e1399_d_n5), multiplicity * (eq53_e1399_d_n6), multiplicity * (eq53_e1399_d_n7), multiplicity * (eq53_e1399_d_n8), multiplicity * (eq53_e1399_d_n9)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq41_e1302: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq41_e1304: f64 = (eq41_e1302 * p.p33);
        let eq41_e1306: f64 = (eq41_e1304 * locals.var_qg);
        let eq41_e1306_d_n4: f64 = (eq41_e1304 * locals.var_qg_dn4);
        let eq41_e1306_d_n6: f64 = (eq41_e1304 * locals.var_qg_dn6);
        let eq41_e1306_d_n7: f64 = (eq41_e1304 * locals.var_qg_dn7);
        let eq41_e1306_d_n8: f64 = (eq41_e1304 * locals.var_qg_dn8);
        let eq41_e1306_d_n9: f64 = (eq41_e1304 * locals.var_qg_dn9);
        let eq41_e1307_q: f64 = eq41_e1306;
        let eq41_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, eq41_e1306_d_n4, 0.0, eq41_e1306_d_n6, eq41_e1306_d_n7, eq41_e1306_d_n8, eq41_e1306_d_n9, 0.0, 0.0, 0.0];
        let eq41_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e1310: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq42_e1312: f64 = (eq42_e1310 * p.p33);
        let eq42_e1314: f64 = (eq42_e1312 * locals.var_qb);
        let eq42_e1314_d_n4: f64 = (eq42_e1312 * locals.var_qb_dn4);
        let eq42_e1314_d_n6: f64 = (eq42_e1312 * locals.var_qb_dn6);
        let eq42_e1314_d_n7: f64 = (eq42_e1312 * locals.var_qb_dn7);
        let eq42_e1314_d_n8: f64 = (eq42_e1312 * locals.var_qb_dn8);
        let eq42_e1314_d_n9: f64 = (eq42_e1312 * locals.var_qb_dn9);
        let eq42_e1315_q: f64 = eq42_e1314;
        let eq42_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, eq42_e1314_d_n4, 0.0, eq42_e1314_d_n6, eq42_e1314_d_n7, eq42_e1314_d_n8, eq42_e1314_d_n9, 0.0, 0.0, 0.0];
        let eq42_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let eq43_e1318: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq43_e1320: f64 = (eq43_e1318 * p.p33);
        let eq43_e1322: f64 = (eq43_e1320 * locals.var_qd);
        let eq43_e1322_d_n4: f64 = (eq43_e1320 * locals.var_qd_dn4);
        let eq43_e1322_d_n6: f64 = (eq43_e1320 * locals.var_qd_dn6);
        let eq43_e1322_d_n7: f64 = (eq43_e1320 * locals.var_qd_dn7);
        let eq43_e1322_d_n8: f64 = (eq43_e1320 * locals.var_qd_dn8);
        let eq43_e1322_d_n9: f64 = (eq43_e1320 * locals.var_qd_dn9);
        let eq43_e1323_q: f64 = eq43_e1322;
        let eq43_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, eq43_e1322_d_n4, 0.0, eq43_e1322_d_n6, eq43_e1322_d_n7, eq43_e1322_d_n8, eq43_e1322_d_n9, 0.0, 0.0, 0.0];
        let eq43_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
        let eq46_e1342: f64 = (locals.var_chnl_type * locals.var_mult_inst);
        let eq46_e1344: f64 = (eq46_e1342 * p.p33);
        let eq46_e1346: f64 = (eq46_e1344 * locals.var_qgb_ov);
        let eq46_e1346_d_n4: f64 = (eq46_e1344 * locals.var_qgb_ov_dn4);
        let eq46_e1346_d_n6: f64 = (eq46_e1344 * locals.var_qgb_ov_dn6);
        let eq46_e1346_d_n7: f64 = (eq46_e1344 * locals.var_qgb_ov_dn7);
        let eq46_e1346_d_n8: f64 = (eq46_e1344 * locals.var_qgb_ov_dn8);
        let eq46_e1346_d_n9: f64 = (eq46_e1344 * locals.var_qgb_ov_dn9);
        let eq46_e1347_q: f64 = eq46_e1346;
        let eq46_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, eq46_e1346_d_n4, 0.0, eq46_e1346_d_n6, eq46_e1346_d_n7, eq46_e1346_d_n8, eq46_e1346_d_n9, 0.0, 0.0, 0.0];
        let eq46_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[9]),
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
        let eq51_e1374: f64 = (locals.var_cgeff * (nv5 - 0.0));
        let eq51_e1374_d_n4: f64 = (locals.var_cgeff_dn4 * (nv5 - 0.0));
        let eq51_e1374_d_n6: f64 = (locals.var_cgeff_dn6 * (nv5 - 0.0));
        let eq51_e1374_d_n7: f64 = (locals.var_cgeff_dn7 * (nv5 - 0.0));
        let eq51_e1374_d_n8: f64 = (locals.var_cgeff_dn8 * (nv5 - 0.0));
        let eq51_e1374_d_n9: f64 = (locals.var_cgeff_dn9 * (nv5 - 0.0));
        let eq51_e1375_q: f64 = eq51_e1374;
        let eq51_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, eq51_e1374_d_n4, locals.var_cgeff, eq51_e1374_d_n6, eq51_e1374_d_n7, eq51_e1374_d_n8, eq51_e1374_d_n9, 0.0, 0.0, 0.0];
        let eq51_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq51_reactive_node_derivatives,
            branches,
            &eq51_reactive_branch_derivatives,
            multiplicity,
        );
        let eq52_e1378: f64 = (locals.var_mult_inst * p.p32);
        let eq52_e1379: f64 = (eq52_e1378).sqrt();
        let eq52_e1381: f64 = (eq52_e1379 * 0.5);
        let eq52_e1383: f64 = (eq52_e1381 * locals.var_cgeff);
        let eq52_e1383_d_n4: f64 = (eq52_e1381 * locals.var_cgeff_dn4);
        let eq52_e1383_d_n6: f64 = (eq52_e1381 * locals.var_cgeff_dn6);
        let eq52_e1383_d_n7: f64 = (eq52_e1381 * locals.var_cgeff_dn7);
        let eq52_e1383_d_n8: f64 = (eq52_e1381 * locals.var_cgeff_dn8);
        let eq52_e1383_d_n9: f64 = (eq52_e1381 * locals.var_cgeff_dn9);
        let eq52_e1385: f64 = (eq52_e1383 * (nv5 - 0.0));
        let eq52_e1385_d_n4: f64 = (eq52_e1383_d_n4 * (nv5 - 0.0));
        let eq52_e1385_d_n6: f64 = (eq52_e1383_d_n6 * (nv5 - 0.0));
        let eq52_e1385_d_n7: f64 = (eq52_e1383_d_n7 * (nv5 - 0.0));
        let eq52_e1385_d_n8: f64 = (eq52_e1383_d_n8 * (nv5 - 0.0));
        let eq52_e1385_d_n9: f64 = (eq52_e1383_d_n9 * (nv5 - 0.0));
        let eq52_e1386_q: f64 = eq52_e1385;
        let eq52_e1387: f64 = (-eq52_e1385);
        let eq52_e1387_q: f64 = (-eq52_e1386_q);
        let eq52_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, (-eq52_e1385_d_n4), (-eq52_e1383), (-eq52_e1385_d_n6), (-eq52_e1385_d_n7), (-eq52_e1385_d_n8), (-eq52_e1385_d_n9), 0.0, 0.0, 0.0];
        let eq52_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq52_reactive_node_derivatives,
            branches,
            &eq52_reactive_branch_derivatives,
            multiplicity,
        );
        let eq53_e1390: f64 = (locals.var_mult_inst * p.p32);
        let eq53_e1391: f64 = (eq53_e1390).sqrt();
        let eq53_e1393: f64 = (eq53_e1391 * 0.5);
        let eq53_e1395: f64 = (eq53_e1393 * locals.var_cgeff);
        let eq53_e1395_d_n4: f64 = (eq53_e1393 * locals.var_cgeff_dn4);
        let eq53_e1395_d_n6: f64 = (eq53_e1393 * locals.var_cgeff_dn6);
        let eq53_e1395_d_n7: f64 = (eq53_e1393 * locals.var_cgeff_dn7);
        let eq53_e1395_d_n8: f64 = (eq53_e1393 * locals.var_cgeff_dn8);
        let eq53_e1395_d_n9: f64 = (eq53_e1393 * locals.var_cgeff_dn9);
        let eq53_e1397: f64 = (eq53_e1395 * (nv5 - 0.0));
        let eq53_e1397_d_n4: f64 = (eq53_e1395_d_n4 * (nv5 - 0.0));
        let eq53_e1397_d_n6: f64 = (eq53_e1395_d_n6 * (nv5 - 0.0));
        let eq53_e1397_d_n7: f64 = (eq53_e1395_d_n7 * (nv5 - 0.0));
        let eq53_e1397_d_n8: f64 = (eq53_e1395_d_n8 * (nv5 - 0.0));
        let eq53_e1397_d_n9: f64 = (eq53_e1395_d_n9 * (nv5 - 0.0));
        let eq53_e1398_q: f64 = eq53_e1397;
        let eq53_e1399: f64 = (-eq53_e1397);
        let eq53_e1399_q: f64 = (-eq53_e1398_q);
        let eq53_reactive_node_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, (-eq53_e1397_d_n4), (-eq53_e1395), (-eq53_e1397_d_n6), (-eq53_e1397_d_n7), (-eq53_e1397_d_n8), (-eq53_e1397_d_n9), 0.0, 0.0, 0.0];
        let eq53_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq53_reactive_node_derivatives,
            branches,
            &eq53_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
