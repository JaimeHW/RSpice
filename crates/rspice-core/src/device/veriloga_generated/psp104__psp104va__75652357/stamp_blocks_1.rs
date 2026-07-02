#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        (locals.var_xs_ov, locals.var_xs_ov_dn5, locals.var_xs_ov_dn6, locals.var_xs_ov_dn7, ) = (0.0, 0.0, 0.0, 0.0, );

        (locals.var_xd_ov, locals.var_xd_ov_dn5, locals.var_xd_ov_dn6, locals.var_xd_ov_dn7, ) = (0.0, 0.0, 0.0, 0.0, );

        (locals.var_vovs, locals.var_vovs_dn5, locals.var_vovs_dn6, locals.var_vovs_dn7, ) = (0.0, 0.0, 0.0, 0.0, );

        (locals.var_vovd, locals.var_vovd_dn5, locals.var_vovd_dn6, locals.var_vovd_dn7, ) = (0.0, 0.0, 0.0, 0.0, );

        let assign45890_e58899: f64 = if (((((p.p40 != 0.0) && ((locals.var_igov_i > 0.0) || (locals.var_igovd_i > 0.0))) || ((p.p42 != 0.0) && ((locals.var_agidl_i > 0.0) || (locals.var_agidld_i > 0.0)))) || (locals.var_cgov_i > 0.0)) || (locals.var_cgovd_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1220 = assign45890_e58899;

        if (locals.var_guard1220 != 0.0) {
            let assign45900_e58905: f64 = (locals.var_xgs_ov * locals.var_xgs_ov);
            let assign45900_e58907: f64 = (assign45900_e58905 + locals.var_sp_ov_eps2_s);
            let assign45900_e58908: f64 = (assign45900_e58907).sqrt();
            let assign45900_e58909: f64 = (locals.var_xgs_ov + assign45900_e58908);
            let assign45900_e58910: f64 = (0.5 * assign45900_e58909);
            (locals.var_sp_ov_xg, locals.var_sp_ov_xg_dn5, locals.var_sp_ov_xg_dn6, locals.var_sp_ov_xg_dn7, ) = (assign45900_e58910, (0.5 * (locals.var_xgs_ov_dn5 + (((locals.var_xgs_ov_dn5 * locals.var_xgs_ov) + (locals.var_xgs_ov * locals.var_xgs_ov_dn5)) / (2.0 * assign45900_e58908)))), (0.5 * (locals.var_xgs_ov_dn6 + (((locals.var_xgs_ov_dn6 * locals.var_xgs_ov) + (locals.var_xgs_ov * locals.var_xgs_ov_dn6)) / (2.0 * assign45900_e58908)))), (0.5 * (locals.var_xgs_ov_dn7 + (((locals.var_xgs_ov_dn7 * locals.var_xgs_ov) + (locals.var_xgs_ov * locals.var_xgs_ov_dn7)) / (2.0 * assign45900_e58908)))), );
        }

        if (locals.var_guard1220 != 0.0) {
            let assign45910_e58915: f64 = (-locals.var_sp_ov_xg);
            let assign45910_e58918: f64 = (locals.var_gov2_s * 0.5);
            let assign45910_e58919: f64 = (assign45910_e58915 - assign45910_e58918);
            let assign45910_e58924: f64 = (locals.var_gov2_s * 0.25);
            let assign45910_e58925: f64 = (locals.var_sp_ov_xg + assign45910_e58924);
            let assign45910_e58927: f64 = (assign45910_e58925 + locals.var_sp_ov_a_s);
            let assign45910_e58928: f64 = (assign45910_e58927).sqrt();
            let assign45910_e58929: f64 = (locals.var_gov_s * assign45910_e58928);
            let assign45910_e58930: f64 = (assign45910_e58919 + assign45910_e58929);
            let assign45910_e58932: f64 = (assign45910_e58930 + locals.var_sp_ov_delta1_s);
            (locals.var_xs_ov, locals.var_xs_ov_dn5, locals.var_xs_ov_dn6, locals.var_xs_ov_dn7, ) = (assign45910_e58932, ((-locals.var_sp_ov_xg_dn5) + (locals.var_gov_s * (locals.var_sp_ov_xg_dn5 / (2.0 * assign45910_e58928)))), ((-locals.var_sp_ov_xg_dn6) + (locals.var_gov_s * (locals.var_sp_ov_xg_dn6 / (2.0 * assign45910_e58928)))), ((-locals.var_sp_ov_xg_dn7) + (locals.var_gov_s * (locals.var_sp_ov_xg_dn7 / (2.0 * assign45910_e58928)))), );
        }

        if (locals.var_guard1220 != 0.0) {
            let assign45920_e58940: f64 = (locals.var_xgd_ov * locals.var_xgd_ov);
            let assign45920_e58942: f64 = (assign45920_e58940 + locals.var_sp_ov_eps2_d);
            let assign45920_e58943: f64 = (assign45920_e58942).sqrt();
            let assign45920_e58944: f64 = (locals.var_xgd_ov + assign45920_e58943);
            let assign45920_e58945: f64 = (0.5 * assign45920_e58944);
            (locals.var_sp_ov_xg, locals.var_sp_ov_xg_dn5, locals.var_sp_ov_xg_dn6, locals.var_sp_ov_xg_dn7, ) = (assign45920_e58945, (0.5 * (locals.var_xgd_ov_dn5 + (((locals.var_xgd_ov_dn5 * locals.var_xgd_ov) + (locals.var_xgd_ov * locals.var_xgd_ov_dn5)) / (2.0 * assign45920_e58943)))), (0.5 * (locals.var_xgd_ov_dn6 + (((locals.var_xgd_ov_dn6 * locals.var_xgd_ov) + (locals.var_xgd_ov * locals.var_xgd_ov_dn6)) / (2.0 * assign45920_e58943)))), (0.5 * (locals.var_xgd_ov_dn7 + (((locals.var_xgd_ov_dn7 * locals.var_xgd_ov) + (locals.var_xgd_ov * locals.var_xgd_ov_dn7)) / (2.0 * assign45920_e58943)))), );
        }

        if (locals.var_guard1220 != 0.0) {
            let assign45930_e58950: f64 = (-locals.var_sp_ov_xg);
            let assign45930_e58953: f64 = (locals.var_gov2_d * 0.5);
            let assign45930_e58954: f64 = (assign45930_e58950 - assign45930_e58953);
            let assign45930_e58959: f64 = (locals.var_gov2_d * 0.25);
            let assign45930_e58960: f64 = (locals.var_sp_ov_xg + assign45930_e58959);
            let assign45930_e58962: f64 = (assign45930_e58960 + locals.var_sp_ov_a_d);
            let assign45930_e58963: f64 = (assign45930_e58962).sqrt();
            let assign45930_e58964: f64 = (locals.var_gov_d * assign45930_e58963);
            let assign45930_e58965: f64 = (assign45930_e58954 + assign45930_e58964);
            let assign45930_e58967: f64 = (assign45930_e58965 + locals.var_sp_ov_delta1_d);
            (locals.var_xd_ov, locals.var_xd_ov_dn5, locals.var_xd_ov_dn6, locals.var_xd_ov_dn7, ) = (assign45930_e58967, ((-locals.var_sp_ov_xg_dn5) + (locals.var_gov_d * (locals.var_sp_ov_xg_dn5 / (2.0 * assign45930_e58963)))), ((-locals.var_sp_ov_xg_dn6) + (locals.var_gov_d * (locals.var_sp_ov_xg_dn6 / (2.0 * assign45930_e58963)))), ((-locals.var_sp_ov_xg_dn7) + (locals.var_gov_d * (locals.var_sp_ov_xg_dn7 / (2.0 * assign45930_e58963)))), );
        }

        if (locals.var_guard1220 != 0.0) {
            let assign45940_e58972: f64 = (-locals.var_phita);
            let assign45940_e58975: f64 = (locals.var_xgs_ov + locals.var_xs_ov);
            let assign45940_e58976: f64 = (assign45940_e58972 * assign45940_e58975);
            (locals.var_vovs, locals.var_vovs_dn5, locals.var_vovs_dn6, locals.var_vovs_dn7, ) = (assign45940_e58976, (assign45940_e58972 * (locals.var_xgs_ov_dn5 + locals.var_xs_ov_dn5)), (assign45940_e58972 * (locals.var_xgs_ov_dn6 + locals.var_xs_ov_dn6)), (assign45940_e58972 * (locals.var_xgs_ov_dn7 + locals.var_xs_ov_dn7)), );
        }

        if (locals.var_guard1220 != 0.0) {
            let assign45950_e58981: f64 = (-locals.var_phita);
            let assign45950_e58984: f64 = (locals.var_xgd_ov + locals.var_xd_ov);
            let assign45950_e58985: f64 = (assign45950_e58981 * assign45950_e58984);
            (locals.var_vovd, locals.var_vovd_dn5, locals.var_vovd_dn6, locals.var_vovd_dn7, ) = (assign45950_e58985, (assign45950_e58981 * (locals.var_xgd_ov_dn5 + locals.var_xd_ov_dn5)), (assign45950_e58981 * (locals.var_xgd_ov_dn6 + locals.var_xd_ov_dn6)), (assign45950_e58981 * (locals.var_xgd_ov_dn7 + locals.var_xd_ov_dn7)), );
        }

        (locals.var_igsov, locals.var_igsov_dn5, locals.var_igsov_dn6, locals.var_igsov_dn7, locals.var_igsov_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_igdov, locals.var_igdov_dn5, locals.var_igdov_dn6, locals.var_igdov_dn7, locals.var_igdov_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_igc_1, locals.var_igc_1_dn5, locals.var_igc_1_dn6, locals.var_igc_1_dn7, locals.var_igc_1_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_i_gb, locals.var_i_gb_dn5, locals.var_i_gb_dn6, locals.var_i_gb_dn7, locals.var_i_gb_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_i_gcs, locals.var_i_gcs_dn5, locals.var_i_gcs_dn6, locals.var_i_gcs_dn7, locals.var_i_gcs_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_i_gcd, locals.var_i_gcd_dn5, locals.var_i_gcd_dn6, locals.var_i_gcd_dn7, locals.var_i_gcd_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign46020_e58996: f64 = if p.p40 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1221 = assign46020_e58996;

        let assign46030_e58999: f64 = if locals.var_igov_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1222 = assign46030_e58999;

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
            let assign46040_e59005: f64 = (locals.var_vovs * locals.var_vovs);
            let assign46040_e59007: f64 = (assign46040_e59005 + 1e-6);
            let assign46040_e59008: f64 = (assign46040_e59007).sqrt();
            let assign46040_e59010: f64 = (assign46040_e59008 * locals.var_inv_chib);
            (locals.var_zg, locals.var_zg_dn5, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, ) = (assign46040_e59010, ((((locals.var_vovs_dn5 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn5)) / (2.0 * assign46040_e59008)) * locals.var_inv_chib), ((((locals.var_vovs_dn6 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn6)) / (2.0 * assign46040_e59008)) * locals.var_inv_chib), ((((locals.var_vovs_dn7 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn7)) / (2.0 * assign46040_e59008)) * locals.var_inv_chib), 0.0, );
        }

        let assign46050_e59015: f64 = if locals.var_gc3ov_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1223 = assign46050_e59015;

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) && (locals.var_guard1223 != 0.0)) {
            let assign46060_e59024: f64 = (locals.var_zg + locals.var_gcqov);
            let assign46060_e59027: f64 = (locals.var_zg - locals.var_gcqov);
            let assign46060_e59030: f64 = (locals.var_zg - locals.var_gcqov);
            let assign46060_e59031: f64 = (assign46060_e59027 * assign46060_e59030);
            let assign46060_e59033: f64 = (assign46060_e59031 + 1e-6);
            let assign46060_e59034: f64 = (assign46060_e59033).sqrt();
            let assign46060_e59035: f64 = (assign46060_e59024 - assign46060_e59034);
            let assign46060_e59036: f64 = (0.5 * assign46060_e59035);
            (locals.var_zg, locals.var_zg_dn5, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, ) = (assign46060_e59036, (0.5 * (locals.var_zg_dn5 - (((locals.var_zg_dn5 * assign46060_e59030) + (assign46060_e59027 * locals.var_zg_dn5)) / (2.0 * assign46060_e59034)))), (0.5 * (locals.var_zg_dn6 - (((locals.var_zg_dn6 * assign46060_e59030) + (assign46060_e59027 * locals.var_zg_dn6)) / (2.0 * assign46060_e59034)))), (0.5 * (locals.var_zg_dn7 - (((locals.var_zg_dn7 * assign46060_e59030) + (assign46060_e59027 * locals.var_zg_dn7)) / (2.0 * assign46060_e59034)))), (0.5 * (locals.var_zg_dn8 - (((locals.var_zg_dn8 * assign46060_e59030) + (assign46060_e59027 * locals.var_zg_dn8)) / (2.0 * assign46060_e59034)))), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
            let assign46070_e59044: f64 = (-1.5);
            let assign46070_e59049: f64 = (locals.var_gc3ov_i * locals.var_zg);
            let assign46070_e59050: f64 = (locals.var_gc2ov_i + assign46070_e59049);
            let assign46070_e59051: f64 = (locals.var_zg * assign46070_e59050);
            let assign46070_e59052: f64 = (assign46070_e59044 + assign46070_e59051);
            let assign46070_e59053: f64 = (locals.var_bov * assign46070_e59052);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign46070_e59053, (locals.var_bov * ((locals.var_zg_dn5 * assign46070_e59050) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn5)))), (locals.var_bov * ((locals.var_zg_dn6 * assign46070_e59050) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn6)))), (locals.var_bov * ((locals.var_zg_dn7 * assign46070_e59050) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn7)))), (locals.var_bov * ((locals.var_zg_dn8 * assign46070_e59050) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn8)))), );
        }

        let assign46080_e59058: f64 = if locals.var_temp__blk936 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1224 = assign46080_e59058;

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) && (locals.var_guard1224 != 0.0)) {
            let assign46090_e59072: f64 = (locals.var_temp__blk936 * 0.3333333333333333);
            let assign46090_e59073: f64 = (1.0 + assign46090_e59072);
            let assign46090_e59074: f64 = (locals.var_temp__blk936 * assign46090_e59073);
            let assign46090_e59075: f64 = (0.5 * assign46090_e59074);
            let assign46090_e59076: f64 = (1.0 + assign46090_e59075);
            let assign46090_e59077: f64 = (locals.var_temp__blk936 * assign46090_e59076);
            let assign46090_e59078: f64 = (1.0 + assign46090_e59077);
            (locals.var_tp, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, ) = (assign46090_e59078, ((locals.var_temp__blk936_dn5 * assign46090_e59076) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn5 * assign46090_e59073) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn5 * 0.3333333333333333)))))), ((locals.var_temp__blk936_dn6 * assign46090_e59076) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn6 * assign46090_e59073) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn6 * 0.3333333333333333)))))), ((locals.var_temp__blk936_dn7 * assign46090_e59076) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn7 * assign46090_e59073) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn7 * 0.3333333333333333)))))), ((locals.var_temp__blk936_dn8 * assign46090_e59076) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn8 * assign46090_e59073) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn8 * 0.3333333333333333)))))), );
        }

        let assign46100_e59083: f64 = (-230.25850929940458);
        let assign46100_e59084: f64 = if locals.var_temp__blk936 > assign46100_e59083 { 1.0 } else { 0.0 };
        locals.var_guard1225 = assign46100_e59084;

        if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) && (locals.var_guard1224 == 0.0)) && (locals.var_guard1225 != 0.0)) {
            let assign46110_e59094: f64 = (locals.var_temp__blk936).exp();
            (locals.var_tp, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, ) = (assign46110_e59094, (assign46110_e59094 * locals.var_temp__blk936_dn5), (assign46110_e59094 * locals.var_temp__blk936_dn6), (assign46110_e59094 * locals.var_temp__blk936_dn7), (assign46110_e59094 * locals.var_temp__blk936_dn8), );
        }

        if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) && (locals.var_guard1224 == 0.0)) && (locals.var_guard1225 == 0.0)) {
            let assign46120_e59109: f64 = (-230.25850929940458);
            let assign46120_e59111: f64 = (assign46120_e59109 - locals.var_temp__blk936);
            let assign46120_e59115: f64 = (-230.25850929940458);
            let assign46120_e59117: f64 = (assign46120_e59115 - locals.var_temp__blk936);
            let assign46120_e59120: f64 = (-230.25850929940458);
            let assign46120_e59122: f64 = (assign46120_e59120 - locals.var_temp__blk936);
            let assign46120_e59124: f64 = (assign46120_e59122 * 0.3333333333333333);
            let assign46120_e59125: f64 = (1.0 + assign46120_e59124);
            let assign46120_e59126: f64 = (assign46120_e59117 * assign46120_e59125);
            let assign46120_e59127: f64 = (0.5 * assign46120_e59126);
            let assign46120_e59128: f64 = (1.0 + assign46120_e59127);
            let assign46120_e59129: f64 = (assign46120_e59111 * assign46120_e59128);
            let assign46120_e59130: f64 = (1.0 + assign46120_e59129);
            let assign46120_e59131: f64 = (1e-100 / assign46120_e59130);
            (locals.var_tp, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, ) = (assign46120_e59131, (-((1e-100 * (((-locals.var_temp__blk936_dn5) * assign46120_e59128) + (assign46120_e59111 * (0.5 * (((-locals.var_temp__blk936_dn5) * assign46120_e59125) + (assign46120_e59117 * ((-locals.var_temp__blk936_dn5) * 0.3333333333333333))))))) / (assign46120_e59130 * assign46120_e59130))), (-((1e-100 * (((-locals.var_temp__blk936_dn6) * assign46120_e59128) + (assign46120_e59111 * (0.5 * (((-locals.var_temp__blk936_dn6) * assign46120_e59125) + (assign46120_e59117 * ((-locals.var_temp__blk936_dn6) * 0.3333333333333333))))))) / (assign46120_e59130 * assign46120_e59130))), (-((1e-100 * (((-locals.var_temp__blk936_dn7) * assign46120_e59128) + (assign46120_e59111 * (0.5 * (((-locals.var_temp__blk936_dn7) * assign46120_e59125) + (assign46120_e59117 * ((-locals.var_temp__blk936_dn7) * 0.3333333333333333))))))) / (assign46120_e59130 * assign46120_e59130))), (-((1e-100 * (((-locals.var_temp__blk936_dn8) * assign46120_e59128) + (assign46120_e59111 * (0.5 * (((-locals.var_temp__blk936_dn8) * assign46120_e59125) + (assign46120_e59117 * ((-locals.var_temp__blk936_dn8) * 0.3333333333333333))))))) / (assign46120_e59130 * assign46120_e59130))), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
            let assign46130_e59139: f64 = (3.0 + locals.var_xs_ov);
            (locals.var_fs1, locals.var_fs1_dn5, locals.var_fs1_dn6, locals.var_fs1_dn7, ) = (assign46130_e59139, locals.var_xs_ov_dn5, locals.var_xs_ov_dn6, locals.var_xs_ov_dn7, );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
            let assign46140_e59146: f64 = (-3.0);
            let assign46140_e59148: f64 = (assign46140_e59146 - locals.var_gco_i);
            locals.var_fs2 = assign46140_e59148;
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
            let assign46150_e59156: f64 = (30.0 * locals.var_vgsprime);
            (locals.var_fs3, locals.var_fs3_dn5, locals.var_fs3_dn6, locals.var_fs3_dn7, ) = (assign46150_e59156, (30.0 * locals.var_vgsprime_dn5), (30.0 * locals.var_vgsprime_dn6), (30.0 * locals.var_vgsprime_dn7), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
            let assign46160_e59164: f64 = (4.0 - 0.9);
            locals.var_tme1 = assign46160_e59164;
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
            let assign46170_e59172: f64 = (locals.var_fs1 + locals.var_fs3);
            (locals.var_tme2, locals.var_tme2_dn5, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8, ) = (assign46170_e59172, (locals.var_fs1_dn5 + locals.var_fs3_dn5), (locals.var_fs1_dn6 + locals.var_fs3_dn6), (locals.var_fs1_dn7 + locals.var_fs3_dn7), 0.0, );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
            let assign46180_e59180: f64 = (2.0 / locals.var_tme1);
            let assign46180_e59184: f64 = (locals.var_tme2 * locals.var_tme2);
            let assign46180_e59187: f64 = (locals.var_tme1 * locals.var_fs1);
            let assign46180_e59189: f64 = (assign46180_e59187 * locals.var_fs3);
            let assign46180_e59190: f64 = (assign46180_e59184 - assign46180_e59189);
            let assign46180_e59191: f64 = (assign46180_e59190).sqrt();
            let assign46180_e59192: f64 = (locals.var_tme2 - assign46180_e59191);
            let assign46180_e59193: f64 = (assign46180_e59180 * assign46180_e59192);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign46180_e59193, (assign46180_e59180 * (locals.var_tme2_dn5 - ((((locals.var_tme2_dn5 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn5)) - (((locals.var_tme1 * locals.var_fs1_dn5) * locals.var_fs3) + (assign46180_e59187 * locals.var_fs3_dn5))) / (2.0 * assign46180_e59191)))), (assign46180_e59180 * (locals.var_tme2_dn6 - ((((locals.var_tme2_dn6 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn6)) - (((locals.var_tme1 * locals.var_fs1_dn6) * locals.var_fs3) + (assign46180_e59187 * locals.var_fs3_dn6))) / (2.0 * assign46180_e59191)))), (assign46180_e59180 * (locals.var_tme2_dn7 - ((((locals.var_tme2_dn7 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn7)) - (((locals.var_tme1 * locals.var_fs1_dn7) * locals.var_fs3) + (assign46180_e59187 * locals.var_fs3_dn7))) / (2.0 * assign46180_e59191)))), (assign46180_e59180 * (locals.var_tme2_dn8 - (((locals.var_tme2_dn8 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn8)) / (2.0 * assign46180_e59191)))), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
            let assign46190_e59201: f64 = (4.0 - 0.3);
            locals.var_tme1 = assign46190_e59201;
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
            let assign46200_e59209: f64 = (locals.var_fs2 + locals.var_temp__blk936);
            (locals.var_tme2, locals.var_tme2_dn5, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8, ) = (assign46200_e59209, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
            let assign46210_e59217: f64 = (2.0 / locals.var_tme1);
            let assign46210_e59221: f64 = (locals.var_tme2 * locals.var_tme2);
            let assign46210_e59224: f64 = (locals.var_tme1 * locals.var_fs2);
            let assign46210_e59226: f64 = (assign46210_e59224 * locals.var_temp__blk936);
            let assign46210_e59227: f64 = (assign46210_e59221 - assign46210_e59226);
            let assign46210_e59228: f64 = (assign46210_e59227).sqrt();
            let assign46210_e59229: f64 = (locals.var_tme2 + assign46210_e59228);
            let assign46210_e59230: f64 = (assign46210_e59217 * assign46210_e59229);
            (locals.var_fs, locals.var_fs_dn5, locals.var_fs_dn6, locals.var_fs_dn7, locals.var_fs_dn8, ) = (assign46210_e59230, (assign46210_e59217 * (locals.var_tme2_dn5 + ((((locals.var_tme2_dn5 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn5)) - (assign46210_e59224 * locals.var_temp__blk936_dn5)) / (2.0 * assign46210_e59228)))), (assign46210_e59217 * (locals.var_tme2_dn6 + ((((locals.var_tme2_dn6 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn6)) - (assign46210_e59224 * locals.var_temp__blk936_dn6)) / (2.0 * assign46210_e59228)))), (assign46210_e59217 * (locals.var_tme2_dn7 + ((((locals.var_tme2_dn7 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn7)) - (assign46210_e59224 * locals.var_temp__blk936_dn7)) / (2.0 * assign46210_e59228)))), (assign46210_e59217 * (locals.var_tme2_dn8 + ((((locals.var_tme2_dn8 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn8)) - (assign46210_e59224 * locals.var_temp__blk936_dn8)) / (2.0 * assign46210_e59228)))), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1222 != 0.0)) {
            let assign46220_e59239: f64 = (locals.var_tp * locals.var_fs);
            let assign46220_e59240: f64 = (locals.var_igov_i * assign46220_e59239);
            (locals.var_igsov, locals.var_igsov_dn5, locals.var_igsov_dn6, locals.var_igsov_dn7, locals.var_igsov_dn8, ) = (assign46220_e59240, (locals.var_igov_i * ((locals.var_tp_dn5 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn5))), (locals.var_igov_i * ((locals.var_tp_dn6 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn6))), (locals.var_igov_i * ((locals.var_tp_dn7 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn7))), (locals.var_igov_i * ((locals.var_tp_dn8 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn8))), );
        }

        let assign46230_e59245: f64 = if locals.var_igovd_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1226 = assign46230_e59245;

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
            let assign46240_e59251: f64 = (locals.var_vovd * locals.var_vovd);
            let assign46240_e59253: f64 = (assign46240_e59251 + 1e-6);
            let assign46240_e59254: f64 = (assign46240_e59253).sqrt();
            let assign46240_e59256: f64 = (assign46240_e59254 * locals.var_inv_chib);
            (locals.var_zg, locals.var_zg_dn5, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, ) = (assign46240_e59256, ((((locals.var_vovd_dn5 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn5)) / (2.0 * assign46240_e59254)) * locals.var_inv_chib), ((((locals.var_vovd_dn6 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn6)) / (2.0 * assign46240_e59254)) * locals.var_inv_chib), ((((locals.var_vovd_dn7 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn7)) / (2.0 * assign46240_e59254)) * locals.var_inv_chib), 0.0, );
        }

        let assign46250_e59261: f64 = if locals.var_gc3ovd_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1227 = assign46250_e59261;

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1227 != 0.0)) {
            let assign46260_e59270: f64 = (locals.var_zg + locals.var_gcqovd);
            let assign46260_e59273: f64 = (locals.var_zg - locals.var_gcqovd);
            let assign46260_e59276: f64 = (locals.var_zg - locals.var_gcqovd);
            let assign46260_e59277: f64 = (assign46260_e59273 * assign46260_e59276);
            let assign46260_e59279: f64 = (assign46260_e59277 + 1e-6);
            let assign46260_e59280: f64 = (assign46260_e59279).sqrt();
            let assign46260_e59281: f64 = (assign46260_e59270 - assign46260_e59280);
            let assign46260_e59282: f64 = (0.5 * assign46260_e59281);
            (locals.var_zg, locals.var_zg_dn5, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, ) = (assign46260_e59282, (0.5 * (locals.var_zg_dn5 - (((locals.var_zg_dn5 * assign46260_e59276) + (assign46260_e59273 * locals.var_zg_dn5)) / (2.0 * assign46260_e59280)))), (0.5 * (locals.var_zg_dn6 - (((locals.var_zg_dn6 * assign46260_e59276) + (assign46260_e59273 * locals.var_zg_dn6)) / (2.0 * assign46260_e59280)))), (0.5 * (locals.var_zg_dn7 - (((locals.var_zg_dn7 * assign46260_e59276) + (assign46260_e59273 * locals.var_zg_dn7)) / (2.0 * assign46260_e59280)))), (0.5 * (locals.var_zg_dn8 - (((locals.var_zg_dn8 * assign46260_e59276) + (assign46260_e59273 * locals.var_zg_dn8)) / (2.0 * assign46260_e59280)))), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
            let assign46270_e59290: f64 = (-1.5);
            let assign46270_e59295: f64 = (locals.var_gc3ovd_i * locals.var_zg);
            let assign46270_e59296: f64 = (locals.var_gc2ovd_i + assign46270_e59295);
            let assign46270_e59297: f64 = (locals.var_zg * assign46270_e59296);
            let assign46270_e59298: f64 = (assign46270_e59290 + assign46270_e59297);
            let assign46270_e59299: f64 = (locals.var_bov_d * assign46270_e59298);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign46270_e59299, (locals.var_bov_d * ((locals.var_zg_dn5 * assign46270_e59296) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn5)))), (locals.var_bov_d * ((locals.var_zg_dn6 * assign46270_e59296) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn6)))), (locals.var_bov_d * ((locals.var_zg_dn7 * assign46270_e59296) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn7)))), (locals.var_bov_d * ((locals.var_zg_dn8 * assign46270_e59296) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn8)))), );
        }

        let assign46280_e59304: f64 = if locals.var_temp__blk936 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1228 = assign46280_e59304;

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1228 != 0.0)) {
            let assign46290_e59318: f64 = (locals.var_temp__blk936 * 0.3333333333333333);
            let assign46290_e59319: f64 = (1.0 + assign46290_e59318);
            let assign46290_e59320: f64 = (locals.var_temp__blk936 * assign46290_e59319);
            let assign46290_e59321: f64 = (0.5 * assign46290_e59320);
            let assign46290_e59322: f64 = (1.0 + assign46290_e59321);
            let assign46290_e59323: f64 = (locals.var_temp__blk936 * assign46290_e59322);
            let assign46290_e59324: f64 = (1.0 + assign46290_e59323);
            (locals.var_tp, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, ) = (assign46290_e59324, ((locals.var_temp__blk936_dn5 * assign46290_e59322) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn5 * assign46290_e59319) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn5 * 0.3333333333333333)))))), ((locals.var_temp__blk936_dn6 * assign46290_e59322) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn6 * assign46290_e59319) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn6 * 0.3333333333333333)))))), ((locals.var_temp__blk936_dn7 * assign46290_e59322) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn7 * assign46290_e59319) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn7 * 0.3333333333333333)))))), ((locals.var_temp__blk936_dn8 * assign46290_e59322) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn8 * assign46290_e59319) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn8 * 0.3333333333333333)))))), );
        }

        let assign46300_e59329: f64 = (-230.25850929940458);
        let assign46300_e59330: f64 = if locals.var_temp__blk936 > assign46300_e59329 { 1.0 } else { 0.0 };
        locals.var_guard1229 = assign46300_e59330;

        if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1228 == 0.0)) && (locals.var_guard1229 != 0.0)) {
            let assign46310_e59340: f64 = (locals.var_temp__blk936).exp();
            (locals.var_tp, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, ) = (assign46310_e59340, (assign46310_e59340 * locals.var_temp__blk936_dn5), (assign46310_e59340 * locals.var_temp__blk936_dn6), (assign46310_e59340 * locals.var_temp__blk936_dn7), (assign46310_e59340 * locals.var_temp__blk936_dn8), );
        }

        if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1228 == 0.0)) && (locals.var_guard1229 == 0.0)) {
            let assign46320_e59355: f64 = (-230.25850929940458);
            let assign46320_e59357: f64 = (assign46320_e59355 - locals.var_temp__blk936);
            let assign46320_e59361: f64 = (-230.25850929940458);
            let assign46320_e59363: f64 = (assign46320_e59361 - locals.var_temp__blk936);
            let assign46320_e59366: f64 = (-230.25850929940458);
            let assign46320_e59368: f64 = (assign46320_e59366 - locals.var_temp__blk936);
            let assign46320_e59370: f64 = (assign46320_e59368 * 0.3333333333333333);
            let assign46320_e59371: f64 = (1.0 + assign46320_e59370);
            let assign46320_e59372: f64 = (assign46320_e59363 * assign46320_e59371);
            let assign46320_e59373: f64 = (0.5 * assign46320_e59372);
            let assign46320_e59374: f64 = (1.0 + assign46320_e59373);
            let assign46320_e59375: f64 = (assign46320_e59357 * assign46320_e59374);
            let assign46320_e59376: f64 = (1.0 + assign46320_e59375);
            let assign46320_e59377: f64 = (1e-100 / assign46320_e59376);
            (locals.var_tp, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, ) = (assign46320_e59377, (-((1e-100 * (((-locals.var_temp__blk936_dn5) * assign46320_e59374) + (assign46320_e59357 * (0.5 * (((-locals.var_temp__blk936_dn5) * assign46320_e59371) + (assign46320_e59363 * ((-locals.var_temp__blk936_dn5) * 0.3333333333333333))))))) / (assign46320_e59376 * assign46320_e59376))), (-((1e-100 * (((-locals.var_temp__blk936_dn6) * assign46320_e59374) + (assign46320_e59357 * (0.5 * (((-locals.var_temp__blk936_dn6) * assign46320_e59371) + (assign46320_e59363 * ((-locals.var_temp__blk936_dn6) * 0.3333333333333333))))))) / (assign46320_e59376 * assign46320_e59376))), (-((1e-100 * (((-locals.var_temp__blk936_dn7) * assign46320_e59374) + (assign46320_e59357 * (0.5 * (((-locals.var_temp__blk936_dn7) * assign46320_e59371) + (assign46320_e59363 * ((-locals.var_temp__blk936_dn7) * 0.3333333333333333))))))) / (assign46320_e59376 * assign46320_e59376))), (-((1e-100 * (((-locals.var_temp__blk936_dn8) * assign46320_e59374) + (assign46320_e59357 * (0.5 * (((-locals.var_temp__blk936_dn8) * assign46320_e59371) + (assign46320_e59363 * ((-locals.var_temp__blk936_dn8) * 0.3333333333333333))))))) / (assign46320_e59376 * assign46320_e59376))), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
            let assign46330_e59385: f64 = (3.0 + locals.var_xd_ov);
            (locals.var_fs1, locals.var_fs1_dn5, locals.var_fs1_dn6, locals.var_fs1_dn7, ) = (assign46330_e59385, locals.var_xd_ov_dn5, locals.var_xd_ov_dn6, locals.var_xd_ov_dn7, );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
            let assign46340_e59392: f64 = (-3.0);
            let assign46340_e59394: f64 = (assign46340_e59392 - locals.var_gco_i);
            locals.var_fs2 = assign46340_e59394;
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
            let assign46350_e59402: f64 = (30.0 * locals.var_vgdprime);
            (locals.var_fs3, locals.var_fs3_dn5, locals.var_fs3_dn6, locals.var_fs3_dn7, ) = (assign46350_e59402, (30.0 * locals.var_vgdprime_dn5), (30.0 * locals.var_vgdprime_dn6), (30.0 * locals.var_vgdprime_dn7), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
            let assign46360_e59410: f64 = (4.0 - 0.9);
            locals.var_tme1 = assign46360_e59410;
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
            let assign46370_e59418: f64 = (locals.var_fs1 + locals.var_fs3);
            (locals.var_tme2, locals.var_tme2_dn5, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8, ) = (assign46370_e59418, (locals.var_fs1_dn5 + locals.var_fs3_dn5), (locals.var_fs1_dn6 + locals.var_fs3_dn6), (locals.var_fs1_dn7 + locals.var_fs3_dn7), 0.0, );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
            let assign46380_e59426: f64 = (2.0 / locals.var_tme1);
            let assign46380_e59430: f64 = (locals.var_tme2 * locals.var_tme2);
            let assign46380_e59433: f64 = (locals.var_tme1 * locals.var_fs1);
            let assign46380_e59435: f64 = (assign46380_e59433 * locals.var_fs3);
            let assign46380_e59436: f64 = (assign46380_e59430 - assign46380_e59435);
            let assign46380_e59437: f64 = (assign46380_e59436).sqrt();
            let assign46380_e59438: f64 = (locals.var_tme2 - assign46380_e59437);
            let assign46380_e59439: f64 = (assign46380_e59426 * assign46380_e59438);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign46380_e59439, (assign46380_e59426 * (locals.var_tme2_dn5 - ((((locals.var_tme2_dn5 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn5)) - (((locals.var_tme1 * locals.var_fs1_dn5) * locals.var_fs3) + (assign46380_e59433 * locals.var_fs3_dn5))) / (2.0 * assign46380_e59437)))), (assign46380_e59426 * (locals.var_tme2_dn6 - ((((locals.var_tme2_dn6 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn6)) - (((locals.var_tme1 * locals.var_fs1_dn6) * locals.var_fs3) + (assign46380_e59433 * locals.var_fs3_dn6))) / (2.0 * assign46380_e59437)))), (assign46380_e59426 * (locals.var_tme2_dn7 - ((((locals.var_tme2_dn7 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn7)) - (((locals.var_tme1 * locals.var_fs1_dn7) * locals.var_fs3) + (assign46380_e59433 * locals.var_fs3_dn7))) / (2.0 * assign46380_e59437)))), (assign46380_e59426 * (locals.var_tme2_dn8 - (((locals.var_tme2_dn8 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn8)) / (2.0 * assign46380_e59437)))), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
            let assign46390_e59447: f64 = (4.0 - 0.3);
            locals.var_tme1 = assign46390_e59447;
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
            let assign46400_e59455: f64 = (locals.var_fs2 + locals.var_temp__blk936);
            (locals.var_tme2, locals.var_tme2_dn5, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8, ) = (assign46400_e59455, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
            let assign46410_e59463: f64 = (2.0 / locals.var_tme1);
            let assign46410_e59467: f64 = (locals.var_tme2 * locals.var_tme2);
            let assign46410_e59470: f64 = (locals.var_tme1 * locals.var_fs2);
            let assign46410_e59472: f64 = (assign46410_e59470 * locals.var_temp__blk936);
            let assign46410_e59473: f64 = (assign46410_e59467 - assign46410_e59472);
            let assign46410_e59474: f64 = (assign46410_e59473).sqrt();
            let assign46410_e59475: f64 = (locals.var_tme2 + assign46410_e59474);
            let assign46410_e59476: f64 = (assign46410_e59463 * assign46410_e59475);
            (locals.var_fs, locals.var_fs_dn5, locals.var_fs_dn6, locals.var_fs_dn7, locals.var_fs_dn8, ) = (assign46410_e59476, (assign46410_e59463 * (locals.var_tme2_dn5 + ((((locals.var_tme2_dn5 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn5)) - (assign46410_e59470 * locals.var_temp__blk936_dn5)) / (2.0 * assign46410_e59474)))), (assign46410_e59463 * (locals.var_tme2_dn6 + ((((locals.var_tme2_dn6 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn6)) - (assign46410_e59470 * locals.var_temp__blk936_dn6)) / (2.0 * assign46410_e59474)))), (assign46410_e59463 * (locals.var_tme2_dn7 + ((((locals.var_tme2_dn7 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn7)) - (assign46410_e59470 * locals.var_temp__blk936_dn7)) / (2.0 * assign46410_e59474)))), (assign46410_e59463 * (locals.var_tme2_dn8 + ((((locals.var_tme2_dn8 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn8)) - (assign46410_e59470 * locals.var_temp__blk936_dn8)) / (2.0 * assign46410_e59474)))), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1226 != 0.0)) {
            let assign46420_e59485: f64 = (locals.var_tp * locals.var_fs);
            let assign46420_e59486: f64 = (locals.var_igovd_i * assign46420_e59485);
            (locals.var_igdov, locals.var_igdov_dn5, locals.var_igdov_dn6, locals.var_igdov_dn7, locals.var_igdov_dn8, ) = (assign46420_e59486, (locals.var_igovd_i * ((locals.var_tp_dn5 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn5))), (locals.var_igovd_i * ((locals.var_tp_dn6 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn6))), (locals.var_igovd_i * ((locals.var_tp_dn7 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn7))), (locals.var_igovd_i * ((locals.var_tp_dn8 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn8))), );
        }

        let assign46430_e59491: f64 = if locals.var_iginv_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1230 = assign46430_e59491;

        let assign46440_e59494: f64 = if locals.var_xg_dc <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1231 = assign46440_e59494;

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1231 != 0.0)) {
            let assign46450_e59502: f64 = (1.0 + locals.var_ar);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign46450_e59502, 0.0, 0.0, 0.0, 0.0, );
        }

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1231 != 0.0)) {
            let assign46460_e59511: f64 = (locals.var_temp__blk936).sqrt();
            let assign46460_e59513: f64 = (assign46460_e59511 * locals.var_v_ds);
            let assign46460_e59515: f64 = (assign46460_e59513 / locals.var_vdsat_lim_dc);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign46460_e59515, (((((locals.var_temp__blk936_dn5 / (2.0 * assign46460_e59511)) * locals.var_v_ds) * locals.var_vdsat_lim_dc) - (assign46460_e59513 * locals.var_vdsat_lim_dc_dn5)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), ((((((locals.var_temp__blk936_dn6 / (2.0 * assign46460_e59511)) * locals.var_v_ds) + (assign46460_e59511 * locals.var_v_ds_dn6)) * locals.var_vdsat_lim_dc) - (assign46460_e59513 * locals.var_vdsat_lim_dc_dn6)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), ((((((locals.var_temp__blk936_dn7 / (2.0 * assign46460_e59511)) * locals.var_v_ds) + (assign46460_e59511 * locals.var_v_ds_dn7)) * locals.var_vdsat_lim_dc) - (assign46460_e59513 * locals.var_vdsat_lim_dc_dn7)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), (((((locals.var_temp__blk936_dn8 / (2.0 * assign46460_e59511)) * locals.var_v_ds) * locals.var_vdsat_lim_dc) - (assign46460_e59513 * locals.var_vdsat_lim_dc_dn8)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), );
        }

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1231 != 0.0)) {
            let assign46470_e59525: f64 = (locals.var_temp1 * locals.var_temp1);
            let assign46470_e59527: f64 = (assign46470_e59525 + locals.var_temp__blk936);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign46470_e59527, (((locals.var_temp1_dn5 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn5)) + locals.var_temp__blk936_dn5), (((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)) + locals.var_temp__blk936_dn6), (((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)) + locals.var_temp__blk936_dn7), (((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)) + locals.var_temp__blk936_dn8), );
        }

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1231 != 0.0)) {
            let assign46480_e59537: f64 = (2.0 * locals.var_temp1);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign46480_e59537, (2.0 * locals.var_temp1_dn5), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), );
        }

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1231 != 0.0)) {
            let assign46490_e59547: f64 = (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc);
            let assign46490_e59549: f64 = (assign46490_e59547 * locals.var_temp__blk936);
            let assign46490_e59552: f64 = (locals.var_temp2 - locals.var_temp__blk936);
            let assign46490_e59553: f64 = (assign46490_e59552).sqrt();
            let assign46490_e59556: f64 = (locals.var_temp2 + locals.var_temp__blk936);
            let assign46490_e59557: f64 = (assign46490_e59556).sqrt();
            let assign46490_e59558: f64 = (assign46490_e59553 + assign46490_e59557);
            let assign46490_e59559: f64 = (assign46490_e59549 / assign46490_e59558);
            (locals.var_udse_dc, locals.var_udse_dc_dn5, locals.var_udse_dc_dn6, locals.var_udse_dc_dn7, locals.var_udse_dc_dn8, ) = (assign46490_e59559, (((((((locals.var_vdsat_lim_dc_dn5 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn5)) * locals.var_temp__blk936) + (assign46490_e59547 * locals.var_temp__blk936_dn5)) * assign46490_e59558) - (assign46490_e59549 * (((locals.var_temp2_dn5 - locals.var_temp__blk936_dn5) / (2.0 * assign46490_e59553)) + ((locals.var_temp2_dn5 + locals.var_temp__blk936_dn5) / (2.0 * assign46490_e59557))))) / (assign46490_e59558 * assign46490_e59558)), (((((((locals.var_vdsat_lim_dc_dn6 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn6)) * locals.var_temp__blk936) + (assign46490_e59547 * locals.var_temp__blk936_dn6)) * assign46490_e59558) - (assign46490_e59549 * (((locals.var_temp2_dn6 - locals.var_temp__blk936_dn6) / (2.0 * assign46490_e59553)) + ((locals.var_temp2_dn6 + locals.var_temp__blk936_dn6) / (2.0 * assign46490_e59557))))) / (assign46490_e59558 * assign46490_e59558)), (((((((locals.var_vdsat_lim_dc_dn7 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn7)) * locals.var_temp__blk936) + (assign46490_e59547 * locals.var_temp__blk936_dn7)) * assign46490_e59558) - (assign46490_e59549 * (((locals.var_temp2_dn7 - locals.var_temp__blk936_dn7) / (2.0 * assign46490_e59553)) + ((locals.var_temp2_dn7 + locals.var_temp__blk936_dn7) / (2.0 * assign46490_e59557))))) / (assign46490_e59558 * assign46490_e59558)), (((((((locals.var_vdsat_lim_dc_dn8 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn8)) * locals.var_temp__blk936) + (assign46490_e59547 * locals.var_temp__blk936_dn8)) * assign46490_e59558) - (assign46490_e59549 * (((locals.var_temp2_dn8 - locals.var_temp__blk936_dn8) / (2.0 * assign46490_e59553)) + ((locals.var_temp2_dn8 + locals.var_temp__blk936_dn8) / (2.0 * assign46490_e59557))))) / (assign46490_e59558 * assign46490_e59558)), );
        }

        let assign46500_e59564: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
        let assign46500_e59566: f64 = (-230.25850929940458);
        let assign46500_e59567: f64 = if assign46500_e59564 > assign46500_e59566 { 1.0 } else { 0.0 };
        locals.var_guard1232 = assign46500_e59567;

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1232 != 0.0)) {
            let assign46510_e59575: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
            let assign46510_e59576: f64 = (assign46510_e59575).exp();
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign46510_e59576, (assign46510_e59576 * (locals.var_x_ds_dc_dn5 - locals.var_udse_dc_dn5)), (assign46510_e59576 * (locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)), (assign46510_e59576 * (locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)), (assign46510_e59576 * (locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)), );
        }

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1232 == 0.0)) {
            let assign46520_e59588: f64 = (-230.25850929940458);
            let assign46520_e59591: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
            let assign46520_e59592: f64 = (assign46520_e59588 - assign46520_e59591);
            let assign46520_e59596: f64 = (-230.25850929940458);
            let assign46520_e59599: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
            let assign46520_e59600: f64 = (assign46520_e59596 - assign46520_e59599);
            let assign46520_e59603: f64 = (-230.25850929940458);
            let assign46520_e59606: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
            let assign46520_e59607: f64 = (assign46520_e59603 - assign46520_e59606);
            let assign46520_e59609: f64 = (assign46520_e59607 * 0.3333333333333333);
            let assign46520_e59610: f64 = (1.0 + assign46520_e59609);
            let assign46520_e59611: f64 = (assign46520_e59600 * assign46520_e59610);
            let assign46520_e59612: f64 = (0.5 * assign46520_e59611);
            let assign46520_e59613: f64 = (1.0 + assign46520_e59612);
            let assign46520_e59614: f64 = (assign46520_e59592 * assign46520_e59613);
            let assign46520_e59615: f64 = (1.0 + assign46520_e59614);
            let assign46520_e59616: f64 = (1e-100 / assign46520_e59615);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign46520_e59616, (-((1e-100 * (((-(locals.var_x_ds_dc_dn5 - locals.var_udse_dc_dn5)) * assign46520_e59613) + (assign46520_e59592 * (0.5 * (((-(locals.var_x_ds_dc_dn5 - locals.var_udse_dc_dn5)) * assign46520_e59610) + (assign46520_e59600 * ((-(locals.var_x_ds_dc_dn5 - locals.var_udse_dc_dn5)) * 0.3333333333333333))))))) / (assign46520_e59615 * assign46520_e59615))), (-((1e-100 * (((-(locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)) * assign46520_e59613) + (assign46520_e59592 * (0.5 * (((-(locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)) * assign46520_e59610) + (assign46520_e59600 * ((-(locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)) * 0.3333333333333333))))))) / (assign46520_e59615 * assign46520_e59615))), (-((1e-100 * (((-(locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)) * assign46520_e59613) + (assign46520_e59592 * (0.5 * (((-(locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)) * assign46520_e59610) + (assign46520_e59600 * ((-(locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)) * 0.3333333333333333))))))) / (assign46520_e59615 * assign46520_e59615))), (-((1e-100 * (((-(locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)) * assign46520_e59613) + (assign46520_e59592 * (0.5 * (((-(locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)) * assign46520_e59610) + (assign46520_e59600 * ((-(locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)) * 0.3333333333333333))))))) / (assign46520_e59615 * assign46520_e59615))), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
            let assign46530_e59626: f64 = (0.5 * locals.var_x_ds_dc);
            let assign46530_e59630: f64 = (1.0 + locals.var_temp__blk936);
            let assign46530_e59631: f64 = (0.5 * assign46530_e59630);
            let assign46530_e59632: f64 = (assign46530_e59631).ln();
            let assign46530_e59633: f64 = (assign46530_e59626 - assign46530_e59632);
            let assign46530_e59634: f64 = (locals.var_phit1_dc * assign46530_e59633);
            let assign46530_e59635: f64 = (locals.var_vsbstar_dc + assign46530_e59634);
            (locals.var_vm, locals.var_vm_dn5, locals.var_vm_dn6, locals.var_vm_dn7, locals.var_vm_dn8, ) = (assign46530_e59635, (locals.var_vsbstar_dc_dn5 + ((locals.var_phit1_dc_dn5 * assign46530_e59633) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn5) - ((0.5 * locals.var_temp__blk936_dn5) / assign46530_e59631))))), (locals.var_vsbstar_dc_dn6 + ((locals.var_phit1_dc_dn6 * assign46530_e59633) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn6) - ((0.5 * locals.var_temp__blk936_dn6) / assign46530_e59631))))), (locals.var_vsbstar_dc_dn7 + ((locals.var_phit1_dc_dn7 * assign46530_e59633) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn7) - ((0.5 * locals.var_temp__blk936_dn7) / assign46530_e59631))))), (locals.var_vsbstar_dc_dn8 + ((locals.var_phit1_dc_dn8 * assign46530_e59633) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn8) - ((0.5 * locals.var_temp__blk936_dn8) / assign46530_e59631))))), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
            let assign46540_e59643: f64 = (locals.var_gco_i * locals.var_phit1_dc);
            (locals.var_dch, locals.var_dch_dn5, locals.var_dch_dn6, locals.var_dch_dn7, locals.var_dch_dn8, ) = (assign46540_e59643, (locals.var_gco_i * locals.var_phit1_dc_dn5), (locals.var_gco_i * locals.var_phit1_dc_dn6), (locals.var_gco_i * locals.var_phit1_dc_dn7), (locals.var_gco_i * locals.var_phit1_dc_dn8), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
            let assign46550_e59651: f64 = (locals.var_voxm_dc + locals.var_dch);
            (locals.var_arg2mina, locals.var_arg2mina_dn5, locals.var_arg2mina_dn6, locals.var_arg2mina_dn7, locals.var_arg2mina_dn8, ) = (assign46550_e59651, (locals.var_voxm_dc_dn5 + locals.var_dch_dn5), (locals.var_voxm_dc_dn6 + locals.var_dch_dn6), (locals.var_voxm_dc_dn7 + locals.var_dch_dn7), (locals.var_voxm_dc_dn8 + locals.var_dch_dn8), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
            let assign46560_e59660: f64 = locals.var_arg2mina;
            let assign46560_e59663: f64 = (-locals.var_arg2mina);
            let assign46560_e59666: f64 = (-locals.var_arg2mina);
            let assign46560_e59667: f64 = (assign46560_e59663 * assign46560_e59666);
            let assign46560_e59669: f64 = (assign46560_e59667 + 0.01);
            let assign46560_e59670: f64 = (assign46560_e59669).sqrt();
            let assign46560_e59671: f64 = (assign46560_e59660 - assign46560_e59670);
            let assign46560_e59672: f64 = (0.5 * assign46560_e59671);
            (locals.var_psi_t, locals.var_psi_t_dn5, locals.var_psi_t_dn6, locals.var_psi_t_dn7, locals.var_psi_t_dn8, ) = (assign46560_e59672, (0.5 * (locals.var_arg2mina_dn5 - ((((-locals.var_arg2mina_dn5) * assign46560_e59666) + (assign46560_e59663 * (-locals.var_arg2mina_dn5))) / (2.0 * assign46560_e59670)))), (0.5 * (locals.var_arg2mina_dn6 - ((((-locals.var_arg2mina_dn6) * assign46560_e59666) + (assign46560_e59663 * (-locals.var_arg2mina_dn6))) / (2.0 * assign46560_e59670)))), (0.5 * (locals.var_arg2mina_dn7 - ((((-locals.var_arg2mina_dn7) * assign46560_e59666) + (assign46560_e59663 * (-locals.var_arg2mina_dn7))) / (2.0 * assign46560_e59670)))), (0.5 * (locals.var_arg2mina_dn8 - ((((-locals.var_arg2mina_dn8) * assign46560_e59666) + (assign46560_e59663 * (-locals.var_arg2mina_dn8))) / (2.0 * assign46560_e59670)))), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
            let assign46570_e59680: f64 = (locals.var_voxm_dc * locals.var_voxm_dc);
            let assign46570_e59682: f64 = (assign46570_e59680 + 1e-6);
            let assign46570_e59683: f64 = (assign46570_e59682).sqrt();
            let assign46570_e59685: f64 = (assign46570_e59683 * locals.var_inv_chib);
            (locals.var_zg, locals.var_zg_dn5, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, ) = (assign46570_e59685, ((((locals.var_voxm_dc_dn5 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn5)) / (2.0 * assign46570_e59683)) * locals.var_inv_chib), ((((locals.var_voxm_dc_dn6 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn6)) / (2.0 * assign46570_e59683)) * locals.var_inv_chib), ((((locals.var_voxm_dc_dn7 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn7)) / (2.0 * assign46570_e59683)) * locals.var_inv_chib), ((((locals.var_voxm_dc_dn8 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn8)) / (2.0 * assign46570_e59683)) * locals.var_inv_chib), );
        }

        let assign46580_e59690: f64 = if locals.var_gc3_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1233 = assign46580_e59690;

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1233 != 0.0)) {
            let assign46590_e59699: f64 = (locals.var_zg + locals.var_gcq);
            let assign46590_e59702: f64 = (locals.var_zg - locals.var_gcq);
            let assign46590_e59705: f64 = (locals.var_zg - locals.var_gcq);
            let assign46590_e59706: f64 = (assign46590_e59702 * assign46590_e59705);
            let assign46590_e59708: f64 = (assign46590_e59706 + 1e-6);
            let assign46590_e59709: f64 = (assign46590_e59708).sqrt();
            let assign46590_e59710: f64 = (assign46590_e59699 - assign46590_e59709);
            let assign46590_e59711: f64 = (0.5 * assign46590_e59710);
            (locals.var_zg, locals.var_zg_dn5, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, ) = (assign46590_e59711, (0.5 * (locals.var_zg_dn5 - (((locals.var_zg_dn5 * assign46590_e59705) + (assign46590_e59702 * locals.var_zg_dn5)) / (2.0 * assign46590_e59709)))), (0.5 * (locals.var_zg_dn6 - (((locals.var_zg_dn6 * assign46590_e59705) + (assign46590_e59702 * locals.var_zg_dn6)) / (2.0 * assign46590_e59709)))), (0.5 * (locals.var_zg_dn7 - (((locals.var_zg_dn7 * assign46590_e59705) + (assign46590_e59702 * locals.var_zg_dn7)) / (2.0 * assign46590_e59709)))), (0.5 * (locals.var_zg_dn8 - (((locals.var_zg_dn8 * assign46590_e59705) + (assign46590_e59702 * locals.var_zg_dn8)) / (2.0 * assign46590_e59709)))), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
            let assign46600_e59720: f64 = (locals.var_psi_t - locals.var_alpha_b);
            let assign46600_e59722: f64 = (assign46600_e59720 - locals.var_vm);
            let assign46600_e59724: f64 = (assign46600_e59722 * locals.var_inv_phit1_dc);
            let assign46600_e59725: f64 = (locals.var_x_m_dc + assign46600_e59724);
            (locals.var_arg1, locals.var_arg1_dn5, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8, ) = (assign46600_e59725, (locals.var_x_m_dc_dn5 + (((locals.var_psi_t_dn5 - locals.var_vm_dn5) * locals.var_inv_phit1_dc) + (assign46600_e59722 * locals.var_inv_phit1_dc_dn5))), (locals.var_x_m_dc_dn6 + (((locals.var_psi_t_dn6 - locals.var_vm_dn6) * locals.var_inv_phit1_dc) + (assign46600_e59722 * locals.var_inv_phit1_dc_dn6))), (locals.var_x_m_dc_dn7 + (((locals.var_psi_t_dn7 - locals.var_vm_dn7) * locals.var_inv_phit1_dc) + (assign46600_e59722 * locals.var_inv_phit1_dc_dn7))), (locals.var_x_m_dc_dn8 + (((locals.var_psi_t_dn8 - locals.var_vm_dn8) * locals.var_inv_phit1_dc) + (assign46600_e59722 * locals.var_inv_phit1_dc_dn8))), );
        }

    }

    pub(super) fn stamp_transient_block_17(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign46610_e59729: f64 = (locals.var_arg1).abs();
        let assign46610_e59731: f64 = if assign46610_e59729 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1234 = assign46610_e59731;

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1234 != 0.0)) {
            let assign46620_e59738: f64 = (locals.var_arg1).exp();
            (locals.var_dsi, locals.var_dsi_dn5, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, ) = (assign46620_e59738, (assign46620_e59738 * locals.var_arg1_dn5), (assign46620_e59738 * locals.var_arg1_dn6), (assign46620_e59738 * locals.var_arg1_dn7), (assign46620_e59738 * locals.var_arg1_dn8), );
        }

        let assign46630_e59743: f64 = if locals.var_arg1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1235 = assign46630_e59743;

        if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1234 == 0.0)) && (locals.var_guard1235 != 0.0)) {
            let assign46640_e59755: f64 = (-230.25850929940458);
            let assign46640_e59757: f64 = (assign46640_e59755 - locals.var_arg1);
            let assign46640_e59761: f64 = (-230.25850929940458);
            let assign46640_e59763: f64 = (assign46640_e59761 - locals.var_arg1);
            let assign46640_e59766: f64 = (-230.25850929940458);
            let assign46640_e59768: f64 = (assign46640_e59766 - locals.var_arg1);
            let assign46640_e59770: f64 = (assign46640_e59768 * 0.3333333333333333);
            let assign46640_e59771: f64 = (1.0 + assign46640_e59770);
            let assign46640_e59772: f64 = (assign46640_e59763 * assign46640_e59771);
            let assign46640_e59773: f64 = (0.5 * assign46640_e59772);
            let assign46640_e59774: f64 = (1.0 + assign46640_e59773);
            let assign46640_e59775: f64 = (assign46640_e59757 * assign46640_e59774);
            let assign46640_e59776: f64 = (1.0 + assign46640_e59775);
            let assign46640_e59777: f64 = (1e-100 / assign46640_e59776);
            (locals.var_dsi, locals.var_dsi_dn5, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, ) = (assign46640_e59777, (-((1e-100 * (((-locals.var_arg1_dn5) * assign46640_e59774) + (assign46640_e59757 * (0.5 * (((-locals.var_arg1_dn5) * assign46640_e59771) + (assign46640_e59763 * ((-locals.var_arg1_dn5) * 0.3333333333333333))))))) / (assign46640_e59776 * assign46640_e59776))), (-((1e-100 * (((-locals.var_arg1_dn6) * assign46640_e59774) + (assign46640_e59757 * (0.5 * (((-locals.var_arg1_dn6) * assign46640_e59771) + (assign46640_e59763 * ((-locals.var_arg1_dn6) * 0.3333333333333333))))))) / (assign46640_e59776 * assign46640_e59776))), (-((1e-100 * (((-locals.var_arg1_dn7) * assign46640_e59774) + (assign46640_e59757 * (0.5 * (((-locals.var_arg1_dn7) * assign46640_e59771) + (assign46640_e59763 * ((-locals.var_arg1_dn7) * 0.3333333333333333))))))) / (assign46640_e59776 * assign46640_e59776))), (-((1e-100 * (((-locals.var_arg1_dn8) * assign46640_e59774) + (assign46640_e59757 * (0.5 * (((-locals.var_arg1_dn8) * assign46640_e59771) + (assign46640_e59763 * ((-locals.var_arg1_dn8) * 0.3333333333333333))))))) / (assign46640_e59776 * assign46640_e59776))), );
        }

        if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1234 == 0.0)) && (locals.var_guard1235 == 0.0)) {
            let assign46650_e59793: f64 = (locals.var_arg1 - 230.25850929940458);
            let assign46650_e59798: f64 = (locals.var_arg1 - 230.25850929940458);
            let assign46650_e59802: f64 = (locals.var_arg1 - 230.25850929940458);
            let assign46650_e59804: f64 = (assign46650_e59802 * 0.3333333333333333);
            let assign46650_e59805: f64 = (1.0 + assign46650_e59804);
            let assign46650_e59806: f64 = (assign46650_e59798 * assign46650_e59805);
            let assign46650_e59807: f64 = (0.5 * assign46650_e59806);
            let assign46650_e59808: f64 = (1.0 + assign46650_e59807);
            let assign46650_e59809: f64 = (assign46650_e59793 * assign46650_e59808);
            let assign46650_e59810: f64 = (1.0 + assign46650_e59809);
            let assign46650_e59811: f64 = (1e100 * assign46650_e59810);
            (locals.var_dsi, locals.var_dsi_dn5, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, ) = (assign46650_e59811, (1e100 * ((locals.var_arg1_dn5 * assign46650_e59808) + (assign46650_e59793 * (0.5 * ((locals.var_arg1_dn5 * assign46650_e59805) + (assign46650_e59798 * (locals.var_arg1_dn5 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn6 * assign46650_e59808) + (assign46650_e59793 * (0.5 * ((locals.var_arg1_dn6 * assign46650_e59805) + (assign46650_e59798 * (locals.var_arg1_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn7 * assign46650_e59808) + (assign46650_e59793 * (0.5 * ((locals.var_arg1_dn7 * assign46650_e59805) + (assign46650_e59798 * (locals.var_arg1_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn8 * assign46650_e59808) + (assign46650_e59793 * (0.5 * ((locals.var_arg1_dn8 * assign46650_e59805) + (assign46650_e59798 * (locals.var_arg1_dn8 * 0.3333333333333333))))))), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
            let assign46660_e59819: f64 = (locals.var_v_gs + locals.var_vsbstar_dc);
            let assign46660_e59821: f64 = (assign46660_e59819 - locals.var_vm);
            let assign46660_e59822: f64 = (-assign46660_e59821);
            let assign46660_e59824: f64 = (assign46660_e59822 * locals.var_inv_phit1_dc);
            (locals.var_arg1, locals.var_arg1_dn5, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8, ) = (assign46660_e59824, (((-((locals.var_v_gs_dn5 + locals.var_vsbstar_dc_dn5) - locals.var_vm_dn5)) * locals.var_inv_phit1_dc) + (assign46660_e59822 * locals.var_inv_phit1_dc_dn5)), (((-((locals.var_v_gs_dn6 + locals.var_vsbstar_dc_dn6) - locals.var_vm_dn6)) * locals.var_inv_phit1_dc) + (assign46660_e59822 * locals.var_inv_phit1_dc_dn6)), (((-((locals.var_v_gs_dn7 + locals.var_vsbstar_dc_dn7) - locals.var_vm_dn7)) * locals.var_inv_phit1_dc) + (assign46660_e59822 * locals.var_inv_phit1_dc_dn7)), (((-(locals.var_vsbstar_dc_dn8 - locals.var_vm_dn8)) * locals.var_inv_phit1_dc) + (assign46660_e59822 * locals.var_inv_phit1_dc_dn8)), );
        }

        let assign46670_e59828: f64 = (locals.var_arg1).abs();
        let assign46670_e59830: f64 = if assign46670_e59828 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1236 = assign46670_e59830;

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1236 != 0.0)) {
            let assign46680_e59837: f64 = (locals.var_arg1).exp();
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign46680_e59837, (assign46680_e59837 * locals.var_arg1_dn5), (assign46680_e59837 * locals.var_arg1_dn6), (assign46680_e59837 * locals.var_arg1_dn7), (assign46680_e59837 * locals.var_arg1_dn8), );
        }

        let assign46690_e59842: f64 = if locals.var_arg1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1237 = assign46690_e59842;

        if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1236 == 0.0)) && (locals.var_guard1237 != 0.0)) {
            let assign46700_e59854: f64 = (-230.25850929940458);
            let assign46700_e59856: f64 = (assign46700_e59854 - locals.var_arg1);
            let assign46700_e59860: f64 = (-230.25850929940458);
            let assign46700_e59862: f64 = (assign46700_e59860 - locals.var_arg1);
            let assign46700_e59865: f64 = (-230.25850929940458);
            let assign46700_e59867: f64 = (assign46700_e59865 - locals.var_arg1);
            let assign46700_e59869: f64 = (assign46700_e59867 * 0.3333333333333333);
            let assign46700_e59870: f64 = (1.0 + assign46700_e59869);
            let assign46700_e59871: f64 = (assign46700_e59862 * assign46700_e59870);
            let assign46700_e59872: f64 = (0.5 * assign46700_e59871);
            let assign46700_e59873: f64 = (1.0 + assign46700_e59872);
            let assign46700_e59874: f64 = (assign46700_e59856 * assign46700_e59873);
            let assign46700_e59875: f64 = (1.0 + assign46700_e59874);
            let assign46700_e59876: f64 = (1e-100 / assign46700_e59875);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign46700_e59876, (-((1e-100 * (((-locals.var_arg1_dn5) * assign46700_e59873) + (assign46700_e59856 * (0.5 * (((-locals.var_arg1_dn5) * assign46700_e59870) + (assign46700_e59862 * ((-locals.var_arg1_dn5) * 0.3333333333333333))))))) / (assign46700_e59875 * assign46700_e59875))), (-((1e-100 * (((-locals.var_arg1_dn6) * assign46700_e59873) + (assign46700_e59856 * (0.5 * (((-locals.var_arg1_dn6) * assign46700_e59870) + (assign46700_e59862 * ((-locals.var_arg1_dn6) * 0.3333333333333333))))))) / (assign46700_e59875 * assign46700_e59875))), (-((1e-100 * (((-locals.var_arg1_dn7) * assign46700_e59873) + (assign46700_e59856 * (0.5 * (((-locals.var_arg1_dn7) * assign46700_e59870) + (assign46700_e59862 * ((-locals.var_arg1_dn7) * 0.3333333333333333))))))) / (assign46700_e59875 * assign46700_e59875))), (-((1e-100 * (((-locals.var_arg1_dn8) * assign46700_e59873) + (assign46700_e59856 * (0.5 * (((-locals.var_arg1_dn8) * assign46700_e59870) + (assign46700_e59862 * ((-locals.var_arg1_dn8) * 0.3333333333333333))))))) / (assign46700_e59875 * assign46700_e59875))), );
        }

        if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1236 == 0.0)) && (locals.var_guard1237 == 0.0)) {
            let assign46710_e59892: f64 = (locals.var_arg1 - 230.25850929940458);
            let assign46710_e59897: f64 = (locals.var_arg1 - 230.25850929940458);
            let assign46710_e59901: f64 = (locals.var_arg1 - 230.25850929940458);
            let assign46710_e59903: f64 = (assign46710_e59901 * 0.3333333333333333);
            let assign46710_e59904: f64 = (1.0 + assign46710_e59903);
            let assign46710_e59905: f64 = (assign46710_e59897 * assign46710_e59904);
            let assign46710_e59906: f64 = (0.5 * assign46710_e59905);
            let assign46710_e59907: f64 = (1.0 + assign46710_e59906);
            let assign46710_e59908: f64 = (assign46710_e59892 * assign46710_e59907);
            let assign46710_e59909: f64 = (1.0 + assign46710_e59908);
            let assign46710_e59910: f64 = (1e100 * assign46710_e59909);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign46710_e59910, (1e100 * ((locals.var_arg1_dn5 * assign46710_e59907) + (assign46710_e59892 * (0.5 * ((locals.var_arg1_dn5 * assign46710_e59904) + (assign46710_e59897 * (locals.var_arg1_dn5 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn6 * assign46710_e59907) + (assign46710_e59892 * (0.5 * ((locals.var_arg1_dn6 * assign46710_e59904) + (assign46710_e59897 * (locals.var_arg1_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn7 * assign46710_e59907) + (assign46710_e59892 * (0.5 * ((locals.var_arg1_dn7 * assign46710_e59904) + (assign46710_e59897 * (locals.var_arg1_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn8 * assign46710_e59907) + (assign46710_e59892 * (0.5 * ((locals.var_arg1_dn8 * assign46710_e59904) + (assign46710_e59897 * (locals.var_arg1_dn8 * 0.3333333333333333))))))), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
            let assign46720_e59918: f64 = (locals.var_dsi * locals.var_temp__blk936);
            (locals.var_dgate, locals.var_dgate_dn5, locals.var_dgate_dn6, locals.var_dgate_dn7, locals.var_dgate_dn8, ) = (assign46720_e59918, ((locals.var_dsi_dn5 * locals.var_temp__blk936) + (locals.var_dsi * locals.var_temp__blk936_dn5)), ((locals.var_dsi_dn6 * locals.var_temp__blk936) + (locals.var_dsi * locals.var_temp__blk936_dn6)), ((locals.var_dsi_dn7 * locals.var_temp__blk936) + (locals.var_dsi * locals.var_temp__blk936_dn7)), ((locals.var_dsi_dn8 * locals.var_temp__blk936) + (locals.var_dsi * locals.var_temp__blk936_dn8)), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
            let assign46730_e59926: f64 = (-1.5);
            let assign46730_e59931: f64 = (locals.var_gc3_i * locals.var_zg);
            let assign46730_e59932: f64 = (locals.var_gc2_i + assign46730_e59931);
            let assign46730_e59933: f64 = (locals.var_zg * assign46730_e59932);
            let assign46730_e59934: f64 = (assign46730_e59926 + assign46730_e59933);
            let assign46730_e59935: f64 = (locals.var_bch * assign46730_e59934);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign46730_e59935, (locals.var_bch * ((locals.var_zg_dn5 * assign46730_e59932) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn5)))), (locals.var_bch * ((locals.var_zg_dn6 * assign46730_e59932) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn6)))), (locals.var_bch * ((locals.var_zg_dn7 * assign46730_e59932) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn7)))), (locals.var_bch * ((locals.var_zg_dn8 * assign46730_e59932) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn8)))), );
        }

        let assign46740_e59940: f64 = if locals.var_temp__blk936 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1238 = assign46740_e59940;

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1238 != 0.0)) {
            let assign46750_e59954: f64 = (locals.var_temp__blk936 * 0.3333333333333333);
            let assign46750_e59955: f64 = (1.0 + assign46750_e59954);
            let assign46750_e59956: f64 = (locals.var_temp__blk936 * assign46750_e59955);
            let assign46750_e59957: f64 = (0.5 * assign46750_e59956);
            let assign46750_e59958: f64 = (1.0 + assign46750_e59957);
            let assign46750_e59959: f64 = (locals.var_temp__blk936 * assign46750_e59958);
            let assign46750_e59960: f64 = (1.0 + assign46750_e59959);
            (locals.var_tp, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, ) = (assign46750_e59960, ((locals.var_temp__blk936_dn5 * assign46750_e59958) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn5 * assign46750_e59955) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn5 * 0.3333333333333333)))))), ((locals.var_temp__blk936_dn6 * assign46750_e59958) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn6 * assign46750_e59955) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn6 * 0.3333333333333333)))))), ((locals.var_temp__blk936_dn7 * assign46750_e59958) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn7 * assign46750_e59955) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn7 * 0.3333333333333333)))))), ((locals.var_temp__blk936_dn8 * assign46750_e59958) + (locals.var_temp__blk936 * (0.5 * ((locals.var_temp__blk936_dn8 * assign46750_e59955) + (locals.var_temp__blk936 * (locals.var_temp__blk936_dn8 * 0.3333333333333333)))))), );
        }

        let assign46760_e59965: f64 = (-230.25850929940458);
        let assign46760_e59966: f64 = if locals.var_temp__blk936 > assign46760_e59965 { 1.0 } else { 0.0 };
        locals.var_guard1239 = assign46760_e59966;

        if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1238 == 0.0)) && (locals.var_guard1239 != 0.0)) {
            let assign46770_e59976: f64 = (locals.var_temp__blk936).exp();
            (locals.var_tp, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, ) = (assign46770_e59976, (assign46770_e59976 * locals.var_temp__blk936_dn5), (assign46770_e59976 * locals.var_temp__blk936_dn6), (assign46770_e59976 * locals.var_temp__blk936_dn7), (assign46770_e59976 * locals.var_temp__blk936_dn8), );
        }

        if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1238 == 0.0)) && (locals.var_guard1239 == 0.0)) {
            let assign46780_e59991: f64 = (-230.25850929940458);
            let assign46780_e59993: f64 = (assign46780_e59991 - locals.var_temp__blk936);
            let assign46780_e59997: f64 = (-230.25850929940458);
            let assign46780_e59999: f64 = (assign46780_e59997 - locals.var_temp__blk936);
            let assign46780_e60002: f64 = (-230.25850929940458);
            let assign46780_e60004: f64 = (assign46780_e60002 - locals.var_temp__blk936);
            let assign46780_e60006: f64 = (assign46780_e60004 * 0.3333333333333333);
            let assign46780_e60007: f64 = (1.0 + assign46780_e60006);
            let assign46780_e60008: f64 = (assign46780_e59999 * assign46780_e60007);
            let assign46780_e60009: f64 = (0.5 * assign46780_e60008);
            let assign46780_e60010: f64 = (1.0 + assign46780_e60009);
            let assign46780_e60011: f64 = (assign46780_e59993 * assign46780_e60010);
            let assign46780_e60012: f64 = (1.0 + assign46780_e60011);
            let assign46780_e60013: f64 = (1e-100 / assign46780_e60012);
            (locals.var_tp, locals.var_tp_dn5, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, ) = (assign46780_e60013, (-((1e-100 * (((-locals.var_temp__blk936_dn5) * assign46780_e60010) + (assign46780_e59993 * (0.5 * (((-locals.var_temp__blk936_dn5) * assign46780_e60007) + (assign46780_e59999 * ((-locals.var_temp__blk936_dn5) * 0.3333333333333333))))))) / (assign46780_e60012 * assign46780_e60012))), (-((1e-100 * (((-locals.var_temp__blk936_dn6) * assign46780_e60010) + (assign46780_e59993 * (0.5 * (((-locals.var_temp__blk936_dn6) * assign46780_e60007) + (assign46780_e59999 * ((-locals.var_temp__blk936_dn6) * 0.3333333333333333))))))) / (assign46780_e60012 * assign46780_e60012))), (-((1e-100 * (((-locals.var_temp__blk936_dn7) * assign46780_e60010) + (assign46780_e59993 * (0.5 * (((-locals.var_temp__blk936_dn7) * assign46780_e60007) + (assign46780_e59999 * ((-locals.var_temp__blk936_dn7) * 0.3333333333333333))))))) / (assign46780_e60012 * assign46780_e60012))), (-((1e-100 * (((-locals.var_temp__blk936_dn8) * assign46780_e60010) + (assign46780_e59993 * (0.5 * (((-locals.var_temp__blk936_dn8) * assign46780_e60007) + (assign46780_e59999 * ((-locals.var_temp__blk936_dn8) * 0.3333333333333333))))))) / (assign46780_e60012 * assign46780_e60012))), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
            let assign46790_e60023: f64 = (1.0 + locals.var_dsi);
            let assign46790_e60026: f64 = (1.0 + locals.var_dgate);
            let assign46790_e60027: f64 = (assign46790_e60023 / assign46790_e60026);
            let assign46790_e60028: f64 = (assign46790_e60027).ln();
            let assign46790_e60029: f64 = (locals.var_tp * assign46790_e60028);
            let assign46790_e60030: f64 = (locals.var_iginv_i * assign46790_e60029);
            (locals.var_igc0, locals.var_igc0_dn5, locals.var_igc0_dn6, locals.var_igc0_dn7, locals.var_igc0_dn8, ) = (assign46790_e60030, (locals.var_iginv_i * ((locals.var_tp_dn5 * assign46790_e60028) + (locals.var_tp * ((((locals.var_dsi_dn5 * assign46790_e60026) - (assign46790_e60023 * locals.var_dgate_dn5)) / (assign46790_e60026 * assign46790_e60026)) / assign46790_e60027)))), (locals.var_iginv_i * ((locals.var_tp_dn6 * assign46790_e60028) + (locals.var_tp * ((((locals.var_dsi_dn6 * assign46790_e60026) - (assign46790_e60023 * locals.var_dgate_dn6)) / (assign46790_e60026 * assign46790_e60026)) / assign46790_e60027)))), (locals.var_iginv_i * ((locals.var_tp_dn7 * assign46790_e60028) + (locals.var_tp * ((((locals.var_dsi_dn7 * assign46790_e60026) - (assign46790_e60023 * locals.var_dgate_dn7)) / (assign46790_e60026 * assign46790_e60026)) / assign46790_e60027)))), (locals.var_iginv_i * ((locals.var_tp_dn8 * assign46790_e60028) + (locals.var_tp * ((((locals.var_dsi_dn8 * assign46790_e60026) - (assign46790_e60023 * locals.var_dgate_dn8)) / (assign46790_e60026 * assign46790_e60026)) / assign46790_e60027)))), );
        }

        let assign46800_e60043: f64 = if ((locals.var_xg_dc <= 0.0) || ((locals.var_gc2_i == 0.0) && (locals.var_gc3_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1240 = assign46800_e60043;

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 != 0.0)) {
            (locals.var_igc, locals.var_igc_dn5, locals.var_igc_dn6, locals.var_igc_dn7, locals.var_igc_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_igcd_h, locals.var_igcd_h_dn5, locals.var_igcd_h_dn6, locals.var_igcd_h_dn7, locals.var_igcd_h_dn8, ) = (0.5, 0.0, 0.0, 0.0, 0.0, );
        }

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) {
            let assign46830_e60069: f64 = (2.0 * locals.var_gc3_i);
            let assign46830_e60071: f64 = (assign46830_e60069 * locals.var_zg);
            let assign46830_e60072: f64 = (locals.var_gc2_i + assign46830_e60071);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign46830_e60072, (assign46830_e60069 * locals.var_zg_dn5), (assign46830_e60069 * locals.var_zg_dn6), (assign46830_e60069 * locals.var_zg_dn7), (assign46830_e60069 * locals.var_zg_dn8), );
        }

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) {
            let assign46840_e60084: f64 = (locals.var_temp__blk936 * locals.var_bch);
            let assign46840_e60085: f64 = (locals.var_chib_i / assign46840_e60084);
            (locals.var_u0, locals.var_u0_dn5, locals.var_u0_dn6, locals.var_u0_dn7, locals.var_u0_dn8, ) = (assign46840_e60085, (-((locals.var_chib_i * (locals.var_temp__blk936_dn5 * locals.var_bch)) / (assign46840_e60084 * assign46840_e60084))), (-((locals.var_chib_i * (locals.var_temp__blk936_dn6 * locals.var_bch)) / (assign46840_e60084 * assign46840_e60084))), (-((locals.var_chib_i * (locals.var_temp__blk936_dn7 * locals.var_bch)) / (assign46840_e60084 * assign46840_e60084))), (-((locals.var_chib_i * (locals.var_temp__blk936_dn8 * locals.var_bch)) / (assign46840_e60084 * assign46840_e60084))), );
        }

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) {
            let assign46850_e60097: f64 = (locals.var_dps_dc / locals.var_u0);
            let assign46850_e60098: f64 = (0.5 * assign46850_e60097);
            (locals.var_x, locals.var_x_dn5, locals.var_x_dn6, locals.var_x_dn7, locals.var_x_dn8, ) = (assign46850_e60098, (0.5 * (((locals.var_dps_dc_dn5 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn5)) / (locals.var_u0 * locals.var_u0))), (0.5 * (((locals.var_dps_dc_dn6 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn6)) / (locals.var_u0 * locals.var_u0))), (0.5 * (((locals.var_dps_dc_dn7 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn7)) / (locals.var_u0 * locals.var_u0))), (0.5 * (((locals.var_dps_dc_dn8 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn8)) / (locals.var_u0 * locals.var_u0))), );
        }

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) {
            let assign46860_e60109: f64 = (locals.var_u0 / locals.var_h_dc);
            (locals.var_u0_div_h, locals.var_u0_div_h_dn5, locals.var_u0_div_h_dn6, locals.var_u0_div_h_dn7, locals.var_u0_div_h_dn8, ) = (assign46860_e60109, (((locals.var_u0_dn5 * locals.var_h_dc) - (locals.var_u0 * locals.var_h_dc_dn5)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_u0_dn6 * locals.var_h_dc) - (locals.var_u0 * locals.var_h_dc_dn6)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_u0_dn7 * locals.var_h_dc) - (locals.var_u0 * locals.var_h_dc_dn7)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_u0_dn8 * locals.var_h_dc) - (locals.var_u0 * locals.var_h_dc_dn8)) / (locals.var_h_dc * locals.var_h_dc)), );
        }

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) {
            let assign46870_e60121: f64 = (1.0 - locals.var_u0_div_h);
            let assign46870_e60122: f64 = (locals.var_u0_div_h * assign46870_e60121);
            let assign46870_e60124: f64 = (assign46870_e60122 * 0.5);
            (locals.var_bg, locals.var_bg_dn5, locals.var_bg_dn6, locals.var_bg_dn7, locals.var_bg_dn8, ) = (assign46870_e60124, (((locals.var_u0_div_h_dn5 * assign46870_e60121) + (locals.var_u0_div_h * (-locals.var_u0_div_h_dn5))) * 0.5), (((locals.var_u0_div_h_dn6 * assign46870_e60121) + (locals.var_u0_div_h * (-locals.var_u0_div_h_dn6))) * 0.5), (((locals.var_u0_div_h_dn7 * assign46870_e60121) + (locals.var_u0_div_h * (-locals.var_u0_div_h_dn7))) * 0.5), (((locals.var_u0_div_h_dn8 * assign46870_e60121) + (locals.var_u0_div_h * (-locals.var_u0_div_h_dn8))) * 0.5), );
        }

        if (((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) {
            let assign46880_e60136: f64 = (3.0 * locals.var_bg);
            let assign46880_e60137: f64 = (0.5 - assign46880_e60136);
            (locals.var_ag, locals.var_ag_dn5, locals.var_ag_dn6, locals.var_ag_dn7, locals.var_ag_dn8, ) = (assign46880_e60137, (-(3.0 * locals.var_bg_dn5)), (-(3.0 * locals.var_bg_dn6)), (-(3.0 * locals.var_bg_dn7)), (-(3.0 * locals.var_bg_dn8)), );
        }

        let assign46890_e60142: f64 = if locals.var_x < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard1241 = assign46890_e60142;

        if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 != 0.0)) {
            let assign46900_e60153: f64 = (locals.var_x * locals.var_x);
            (locals.var_xsq, locals.var_xsq_dn5, locals.var_xsq_dn6, locals.var_xsq_dn7, locals.var_xsq_dn8, ) = (assign46900_e60153, ((locals.var_x_dn5 * locals.var_x) + (locals.var_x * locals.var_x_dn5)), ((locals.var_x_dn6 * locals.var_x) + (locals.var_x * locals.var_x_dn6)), ((locals.var_x_dn7 * locals.var_x) + (locals.var_x * locals.var_x_dn7)), ((locals.var_x_dn8 * locals.var_x) + (locals.var_x * locals.var_x_dn8)), );
        }

        if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 != 0.0)) {
            let assign46910_e60169: f64 = (locals.var_u0_div_h * 0.3333333333333333);
            let assign46910_e60170: f64 = (0.16666666666666666 + assign46910_e60169);
            let assign46910_e60176: f64 = (0.2 * locals.var_u0_div_h);
            let assign46910_e60177: f64 = (0.05 + assign46910_e60176);
            let assign46910_e60178: f64 = (locals.var_xsq * assign46910_e60177);
            let assign46910_e60179: f64 = (0.16666666666666666 * assign46910_e60178);
            let assign46910_e60180: f64 = (assign46910_e60170 + assign46910_e60179);
            let assign46910_e60181: f64 = (locals.var_xsq * assign46910_e60180);
            let assign46910_e60182: f64 = (1.0 + assign46910_e60181);
            (locals.var_igc, locals.var_igc_dn5, locals.var_igc_dn6, locals.var_igc_dn7, locals.var_igc_dn8, ) = (assign46910_e60182, ((locals.var_xsq_dn5 * assign46910_e60180) + (locals.var_xsq * ((locals.var_u0_div_h_dn5 * 0.3333333333333333) + (0.16666666666666666 * ((locals.var_xsq_dn5 * assign46910_e60177) + (locals.var_xsq * (0.2 * locals.var_u0_div_h_dn5))))))), ((locals.var_xsq_dn6 * assign46910_e60180) + (locals.var_xsq * ((locals.var_u0_div_h_dn6 * 0.3333333333333333) + (0.16666666666666666 * ((locals.var_xsq_dn6 * assign46910_e60177) + (locals.var_xsq * (0.2 * locals.var_u0_div_h_dn6))))))), ((locals.var_xsq_dn7 * assign46910_e60180) + (locals.var_xsq * ((locals.var_u0_div_h_dn7 * 0.3333333333333333) + (0.16666666666666666 * ((locals.var_xsq_dn7 * assign46910_e60177) + (locals.var_xsq * (0.2 * locals.var_u0_div_h_dn7))))))), ((locals.var_xsq_dn8 * assign46910_e60180) + (locals.var_xsq * ((locals.var_u0_div_h_dn8 * 0.3333333333333333) + (0.16666666666666666 * ((locals.var_xsq_dn8 * assign46910_e60177) + (locals.var_xsq * (0.2 * locals.var_u0_div_h_dn8))))))), );
        }

        if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 != 0.0)) {
            let assign46920_e60195: f64 = (0.5 * locals.var_igc);
            let assign46920_e60203: f64 = (locals.var_bg + 0.25);
            let assign46920_e60204: f64 = (0.4 * assign46920_e60203);
            let assign46920_e60209: f64 = (0.125 + locals.var_bg);
            let assign46920_e60210: f64 = (locals.var_xsq * assign46920_e60209);
            let assign46920_e60211: f64 = (0.0285714285714 * assign46920_e60210);
            let assign46920_e60212: f64 = (assign46920_e60204 + assign46920_e60211);
            let assign46920_e60213: f64 = (locals.var_xsq * assign46920_e60212);
            let assign46920_e60214: f64 = (1.0 + assign46920_e60213);
            let assign46920_e60215: f64 = (locals.var_x * assign46920_e60214);
            let assign46920_e60216: f64 = (0.16666666666666666 * assign46920_e60215);
            let assign46920_e60217: f64 = (assign46920_e60195 - assign46920_e60216);
            (locals.var_igcd_h, locals.var_igcd_h_dn5, locals.var_igcd_h_dn6, locals.var_igcd_h_dn7, locals.var_igcd_h_dn8, ) = (assign46920_e60217, ((0.5 * locals.var_igc_dn5) - (0.16666666666666666 * ((locals.var_x_dn5 * assign46920_e60214) + (locals.var_x * ((locals.var_xsq_dn5 * assign46920_e60212) + (locals.var_xsq * ((0.4 * locals.var_bg_dn5) + (0.0285714285714 * ((locals.var_xsq_dn5 * assign46920_e60209) + (locals.var_xsq * locals.var_bg_dn5)))))))))), ((0.5 * locals.var_igc_dn6) - (0.16666666666666666 * ((locals.var_x_dn6 * assign46920_e60214) + (locals.var_x * ((locals.var_xsq_dn6 * assign46920_e60212) + (locals.var_xsq * ((0.4 * locals.var_bg_dn6) + (0.0285714285714 * ((locals.var_xsq_dn6 * assign46920_e60209) + (locals.var_xsq * locals.var_bg_dn6)))))))))), ((0.5 * locals.var_igc_dn7) - (0.16666666666666666 * ((locals.var_x_dn7 * assign46920_e60214) + (locals.var_x * ((locals.var_xsq_dn7 * assign46920_e60212) + (locals.var_xsq * ((0.4 * locals.var_bg_dn7) + (0.0285714285714 * ((locals.var_xsq_dn7 * assign46920_e60209) + (locals.var_xsq * locals.var_bg_dn7)))))))))), ((0.5 * locals.var_igc_dn8) - (0.16666666666666666 * ((locals.var_x_dn8 * assign46920_e60214) + (locals.var_x * ((locals.var_xsq_dn8 * assign46920_e60212) + (locals.var_xsq * ((0.4 * locals.var_bg_dn8) + (0.0285714285714 * ((locals.var_xsq_dn8 * assign46920_e60209) + (locals.var_xsq * locals.var_bg_dn8)))))))))), );
        }

        if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) {
            let assign46930_e60231: f64 = (1.0 / locals.var_x);
            (locals.var_inv_x, locals.var_inv_x_dn5, locals.var_inv_x_dn6, locals.var_inv_x_dn7, locals.var_inv_x_dn8, ) = (assign46930_e60231, (-(locals.var_x_dn5 / (locals.var_x * locals.var_x))), (-(locals.var_x_dn6 / (locals.var_x * locals.var_x))), (-(locals.var_x_dn7 / (locals.var_x * locals.var_x))), (-(locals.var_x_dn8 / (locals.var_x * locals.var_x))), );
        }

        let assign46940_e60235: f64 = (locals.var_x).abs();
        let assign46940_e60237: f64 = if assign46940_e60235 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1242 = assign46940_e60237;

        if (((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) && (locals.var_guard1242 != 0.0)) {
            let assign46950_e60250: f64 = (locals.var_x).exp();
            (locals.var_ex, locals.var_ex_dn5, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8, ) = (assign46950_e60250, (assign46950_e60250 * locals.var_x_dn5), (assign46950_e60250 * locals.var_x_dn6), (assign46950_e60250 * locals.var_x_dn7), (assign46950_e60250 * locals.var_x_dn8), );
        }

        let assign46960_e60255: f64 = if locals.var_x < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1243 = assign46960_e60255;

        if ((((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) && (locals.var_guard1242 == 0.0)) && (locals.var_guard1243 != 0.0)) {
            let assign46970_e60273: f64 = (-230.25850929940458);
            let assign46970_e60275: f64 = (assign46970_e60273 - locals.var_x);
            let assign46970_e60279: f64 = (-230.25850929940458);
            let assign46970_e60281: f64 = (assign46970_e60279 - locals.var_x);
            let assign46970_e60284: f64 = (-230.25850929940458);
            let assign46970_e60286: f64 = (assign46970_e60284 - locals.var_x);
            let assign46970_e60288: f64 = (assign46970_e60286 * 0.3333333333333333);
            let assign46970_e60289: f64 = (1.0 + assign46970_e60288);
            let assign46970_e60290: f64 = (assign46970_e60281 * assign46970_e60289);
            let assign46970_e60291: f64 = (0.5 * assign46970_e60290);
            let assign46970_e60292: f64 = (1.0 + assign46970_e60291);
            let assign46970_e60293: f64 = (assign46970_e60275 * assign46970_e60292);
            let assign46970_e60294: f64 = (1.0 + assign46970_e60293);
            let assign46970_e60295: f64 = (1e-100 / assign46970_e60294);
            (locals.var_ex, locals.var_ex_dn5, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8, ) = (assign46970_e60295, (-((1e-100 * (((-locals.var_x_dn5) * assign46970_e60292) + (assign46970_e60275 * (0.5 * (((-locals.var_x_dn5) * assign46970_e60289) + (assign46970_e60281 * ((-locals.var_x_dn5) * 0.3333333333333333))))))) / (assign46970_e60294 * assign46970_e60294))), (-((1e-100 * (((-locals.var_x_dn6) * assign46970_e60292) + (assign46970_e60275 * (0.5 * (((-locals.var_x_dn6) * assign46970_e60289) + (assign46970_e60281 * ((-locals.var_x_dn6) * 0.3333333333333333))))))) / (assign46970_e60294 * assign46970_e60294))), (-((1e-100 * (((-locals.var_x_dn7) * assign46970_e60292) + (assign46970_e60275 * (0.5 * (((-locals.var_x_dn7) * assign46970_e60289) + (assign46970_e60281 * ((-locals.var_x_dn7) * 0.3333333333333333))))))) / (assign46970_e60294 * assign46970_e60294))), (-((1e-100 * (((-locals.var_x_dn8) * assign46970_e60292) + (assign46970_e60275 * (0.5 * (((-locals.var_x_dn8) * assign46970_e60289) + (assign46970_e60281 * ((-locals.var_x_dn8) * 0.3333333333333333))))))) / (assign46970_e60294 * assign46970_e60294))), );
        }

        if ((((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) && (locals.var_guard1242 == 0.0)) && (locals.var_guard1243 == 0.0)) {
            let assign46980_e60317: f64 = (locals.var_x - 230.25850929940458);
            let assign46980_e60322: f64 = (locals.var_x - 230.25850929940458);
            let assign46980_e60326: f64 = (locals.var_x - 230.25850929940458);
            let assign46980_e60328: f64 = (assign46980_e60326 * 0.3333333333333333);
            let assign46980_e60329: f64 = (1.0 + assign46980_e60328);
            let assign46980_e60330: f64 = (assign46980_e60322 * assign46980_e60329);
            let assign46980_e60331: f64 = (0.5 * assign46980_e60330);
            let assign46980_e60332: f64 = (1.0 + assign46980_e60331);
            let assign46980_e60333: f64 = (assign46980_e60317 * assign46980_e60332);
            let assign46980_e60334: f64 = (1.0 + assign46980_e60333);
            let assign46980_e60335: f64 = (1e100 * assign46980_e60334);
            (locals.var_ex, locals.var_ex_dn5, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8, ) = (assign46980_e60335, (1e100 * ((locals.var_x_dn5 * assign46980_e60332) + (assign46980_e60317 * (0.5 * ((locals.var_x_dn5 * assign46980_e60329) + (assign46980_e60322 * (locals.var_x_dn5 * 0.3333333333333333))))))), (1e100 * ((locals.var_x_dn6 * assign46980_e60332) + (assign46980_e60317 * (0.5 * ((locals.var_x_dn6 * assign46980_e60329) + (assign46980_e60322 * (locals.var_x_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_x_dn7 * assign46980_e60332) + (assign46980_e60317 * (0.5 * ((locals.var_x_dn7 * assign46980_e60329) + (assign46980_e60322 * (locals.var_x_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_x_dn8 * assign46980_e60332) + (assign46980_e60317 * (0.5 * ((locals.var_x_dn8 * assign46980_e60329) + (assign46980_e60322 * (locals.var_x_dn8 * 0.3333333333333333))))))), );
        }

        if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) {
            let assign46990_e60349: f64 = (1.0 / locals.var_ex);
            (locals.var_inv_ex, locals.var_inv_ex_dn5, locals.var_inv_ex_dn6, locals.var_inv_ex_dn7, locals.var_inv_ex_dn8, ) = (assign46990_e60349, (-(locals.var_ex_dn5 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn6 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn7 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn8 / (locals.var_ex * locals.var_ex))), );
        }

        if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) {
            let assign47000_e60363: f64 = (locals.var_ex - locals.var_inv_ex);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign47000_e60363, (locals.var_ex_dn5 - locals.var_inv_ex_dn5), (locals.var_ex_dn6 - locals.var_inv_ex_dn6), (locals.var_ex_dn7 - locals.var_inv_ex_dn7), (locals.var_ex_dn8 - locals.var_inv_ex_dn8), );
        }

        if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) {
            let assign47010_e60377: f64 = (locals.var_ex + locals.var_inv_ex);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign47010_e60377, (locals.var_ex_dn5 + locals.var_inv_ex_dn5), (locals.var_ex_dn6 + locals.var_inv_ex_dn6), (locals.var_ex_dn7 + locals.var_inv_ex_dn7), (locals.var_ex_dn8 + locals.var_inv_ex_dn8), );
        }

        if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) {
            let assign47020_e60392: f64 = (1.0 - locals.var_u0_div_h);
            let assign47020_e60394: f64 = (assign47020_e60392 * locals.var_temp__blk936);
            let assign47020_e60396: f64 = (assign47020_e60394 * locals.var_inv_x);
            let assign47020_e60399: f64 = (locals.var_u0_div_h * locals.var_temp2);
            let assign47020_e60400: f64 = (assign47020_e60396 + assign47020_e60399);
            let assign47020_e60401: f64 = (0.5 * assign47020_e60400);
            (locals.var_igc, locals.var_igc_dn5, locals.var_igc_dn6, locals.var_igc_dn7, locals.var_igc_dn8, ) = (assign47020_e60401, (0.5 * ((((((-locals.var_u0_div_h_dn5) * locals.var_temp__blk936) + (assign47020_e60392 * locals.var_temp__blk936_dn5)) * locals.var_inv_x) + (assign47020_e60394 * locals.var_inv_x_dn5)) + ((locals.var_u0_div_h_dn5 * locals.var_temp2) + (locals.var_u0_div_h * locals.var_temp2_dn5)))), (0.5 * ((((((-locals.var_u0_div_h_dn6) * locals.var_temp__blk936) + (assign47020_e60392 * locals.var_temp__blk936_dn6)) * locals.var_inv_x) + (assign47020_e60394 * locals.var_inv_x_dn6)) + ((locals.var_u0_div_h_dn6 * locals.var_temp2) + (locals.var_u0_div_h * locals.var_temp2_dn6)))), (0.5 * ((((((-locals.var_u0_div_h_dn7) * locals.var_temp__blk936) + (assign47020_e60392 * locals.var_temp__blk936_dn7)) * locals.var_inv_x) + (assign47020_e60394 * locals.var_inv_x_dn7)) + ((locals.var_u0_div_h_dn7 * locals.var_temp2) + (locals.var_u0_div_h * locals.var_temp2_dn7)))), (0.5 * ((((((-locals.var_u0_div_h_dn8) * locals.var_temp__blk936) + (assign47020_e60392 * locals.var_temp__blk936_dn8)) * locals.var_inv_x) + (assign47020_e60394 * locals.var_inv_x_dn8)) + ((locals.var_u0_div_h_dn8 * locals.var_temp2) + (locals.var_u0_div_h * locals.var_temp2_dn8)))), );
        }

        if ((((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1240 == 0.0)) && (locals.var_guard1241 == 0.0)) {
            let assign47030_e60419: f64 = (locals.var_ag * locals.var_inv_x);
            let assign47030_e60421: f64 = (assign47030_e60419 * locals.var_inv_x);
            let assign47030_e60422: f64 = (locals.var_bg - assign47030_e60421);
            let assign47030_e60423: f64 = (locals.var_temp__blk936 * assign47030_e60422);
            let assign47030_e60424: f64 = (locals.var_igc - assign47030_e60423);
            let assign47030_e60427: f64 = (locals.var_ag * locals.var_temp2);
            let assign47030_e60429: f64 = (assign47030_e60427 * locals.var_inv_x);
            let assign47030_e60430: f64 = (assign47030_e60424 - assign47030_e60429);
            let assign47030_e60431: f64 = (0.5 * assign47030_e60430);
            (locals.var_igcd_h, locals.var_igcd_h_dn5, locals.var_igcd_h_dn6, locals.var_igcd_h_dn7, locals.var_igcd_h_dn8, ) = (assign47030_e60431, (0.5 * ((locals.var_igc_dn5 - ((locals.var_temp__blk936_dn5 * assign47030_e60422) + (locals.var_temp__blk936 * (locals.var_bg_dn5 - ((((locals.var_ag_dn5 * locals.var_inv_x) + (locals.var_ag * locals.var_inv_x_dn5)) * locals.var_inv_x) + (assign47030_e60419 * locals.var_inv_x_dn5)))))) - ((((locals.var_ag_dn5 * locals.var_temp2) + (locals.var_ag * locals.var_temp2_dn5)) * locals.var_inv_x) + (assign47030_e60427 * locals.var_inv_x_dn5)))), (0.5 * ((locals.var_igc_dn6 - ((locals.var_temp__blk936_dn6 * assign47030_e60422) + (locals.var_temp__blk936 * (locals.var_bg_dn6 - ((((locals.var_ag_dn6 * locals.var_inv_x) + (locals.var_ag * locals.var_inv_x_dn6)) * locals.var_inv_x) + (assign47030_e60419 * locals.var_inv_x_dn6)))))) - ((((locals.var_ag_dn6 * locals.var_temp2) + (locals.var_ag * locals.var_temp2_dn6)) * locals.var_inv_x) + (assign47030_e60427 * locals.var_inv_x_dn6)))), (0.5 * ((locals.var_igc_dn7 - ((locals.var_temp__blk936_dn7 * assign47030_e60422) + (locals.var_temp__blk936 * (locals.var_bg_dn7 - ((((locals.var_ag_dn7 * locals.var_inv_x) + (locals.var_ag * locals.var_inv_x_dn7)) * locals.var_inv_x) + (assign47030_e60419 * locals.var_inv_x_dn7)))))) - ((((locals.var_ag_dn7 * locals.var_temp2) + (locals.var_ag * locals.var_temp2_dn7)) * locals.var_inv_x) + (assign47030_e60427 * locals.var_inv_x_dn7)))), (0.5 * ((locals.var_igc_dn8 - ((locals.var_temp__blk936_dn8 * assign47030_e60422) + (locals.var_temp__blk936 * (locals.var_bg_dn8 - ((((locals.var_ag_dn8 * locals.var_inv_x) + (locals.var_ag * locals.var_inv_x_dn8)) * locals.var_inv_x) + (assign47030_e60419 * locals.var_inv_x_dn8)))))) - ((((locals.var_ag_dn8 * locals.var_temp2) + (locals.var_ag * locals.var_temp2_dn8)) * locals.var_inv_x) + (assign47030_e60427 * locals.var_inv_x_dn8)))), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
            let assign47040_e60442: f64 = (locals.var_xg_dc * locals.var_xg_dc);
            let assign47040_e60444: f64 = (assign47040_e60442 + 1e-6);
            let assign47040_e60445: f64 = (assign47040_e60444).sqrt();
            let assign47040_e60446: f64 = (locals.var_xg_dc / assign47040_e60445);
            let assign47040_e60447: f64 = (1.0 + assign47040_e60446);
            let assign47040_e60448: f64 = (0.5 * assign47040_e60447);
            (locals.var_sg, locals.var_sg_dn5, locals.var_sg_dn6, locals.var_sg_dn7, locals.var_sg_dn8, ) = (assign47040_e60448, (0.5 * (((locals.var_xg_dc_dn5 * assign47040_e60445) - (locals.var_xg_dc * (((locals.var_xg_dc_dn5 * locals.var_xg_dc) + (locals.var_xg_dc * locals.var_xg_dc_dn5)) / (2.0 * assign47040_e60445)))) / (assign47040_e60445 * assign47040_e60445))), (0.5 * (((locals.var_xg_dc_dn6 * assign47040_e60445) - (locals.var_xg_dc * (((locals.var_xg_dc_dn6 * locals.var_xg_dc) + (locals.var_xg_dc * locals.var_xg_dc_dn6)) / (2.0 * assign47040_e60445)))) / (assign47040_e60445 * assign47040_e60445))), (0.5 * (((locals.var_xg_dc_dn7 * assign47040_e60445) - (locals.var_xg_dc * (((locals.var_xg_dc_dn7 * locals.var_xg_dc) + (locals.var_xg_dc * locals.var_xg_dc_dn7)) / (2.0 * assign47040_e60445)))) / (assign47040_e60445 * assign47040_e60445))), (0.5 * (((locals.var_xg_dc_dn8 * assign47040_e60445) - (locals.var_xg_dc * (((locals.var_xg_dc_dn8 * locals.var_xg_dc) + (locals.var_xg_dc * locals.var_xg_dc_dn8)) / (2.0 * assign47040_e60445)))) / (assign47040_e60445 * assign47040_e60445))), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
            let assign47050_e60456: f64 = (locals.var_igc0 * locals.var_igc);
            let assign47050_e60458: f64 = (assign47050_e60456 * locals.var_sg);
            (locals.var_igc_1, locals.var_igc_1_dn5, locals.var_igc_1_dn6, locals.var_igc_1_dn7, locals.var_igc_1_dn8, ) = (assign47050_e60458, ((((locals.var_igc0_dn5 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn5)) * locals.var_sg) + (assign47050_e60456 * locals.var_sg_dn5)), ((((locals.var_igc0_dn6 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn6)) * locals.var_sg) + (assign47050_e60456 * locals.var_sg_dn6)), ((((locals.var_igc0_dn7 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn7)) * locals.var_sg) + (assign47050_e60456 * locals.var_sg_dn7)), ((((locals.var_igc0_dn8 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn8)) * locals.var_sg) + (assign47050_e60456 * locals.var_sg_dn8)), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
            let assign47060_e60466: f64 = (locals.var_igc0 * locals.var_igcd_h);
            let assign47060_e60468: f64 = (assign47060_e60466 * locals.var_sg);
            (locals.var_i_gcd, locals.var_i_gcd_dn5, locals.var_i_gcd_dn6, locals.var_i_gcd_dn7, locals.var_i_gcd_dn8, ) = (assign47060_e60468, ((((locals.var_igc0_dn5 * locals.var_igcd_h) + (locals.var_igc0 * locals.var_igcd_h_dn5)) * locals.var_sg) + (assign47060_e60466 * locals.var_sg_dn5)), ((((locals.var_igc0_dn6 * locals.var_igcd_h) + (locals.var_igc0 * locals.var_igcd_h_dn6)) * locals.var_sg) + (assign47060_e60466 * locals.var_sg_dn6)), ((((locals.var_igc0_dn7 * locals.var_igcd_h) + (locals.var_igc0 * locals.var_igcd_h_dn7)) * locals.var_sg) + (assign47060_e60466 * locals.var_sg_dn7)), ((((locals.var_igc0_dn8 * locals.var_igcd_h) + (locals.var_igc0 * locals.var_igcd_h_dn8)) * locals.var_sg) + (assign47060_e60466 * locals.var_sg_dn8)), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
            let assign47070_e60476: f64 = (locals.var_igc_1 - locals.var_i_gcd);
            (locals.var_i_gcs, locals.var_i_gcs_dn5, locals.var_i_gcs_dn6, locals.var_i_gcs_dn7, locals.var_i_gcs_dn8, ) = (assign47070_e60476, (locals.var_igc_1_dn5 - locals.var_i_gcd_dn5), (locals.var_igc_1_dn6 - locals.var_i_gcd_dn6), (locals.var_igc_1_dn7 - locals.var_i_gcd_dn7), (locals.var_igc_1_dn8 - locals.var_i_gcd_dn8), );
        }

        if ((locals.var_guard1221 != 0.0) && (locals.var_guard1230 != 0.0)) {
            let assign47080_e60484: f64 = (locals.var_igc0 * locals.var_igc);
            let assign47080_e60487: f64 = (1.0 - locals.var_sg);
            let assign47080_e60488: f64 = (assign47080_e60484 * assign47080_e60487);
            (locals.var_i_gb, locals.var_i_gb_dn5, locals.var_i_gb_dn6, locals.var_i_gb_dn7, locals.var_i_gb_dn8, ) = (assign47080_e60488, ((((locals.var_igc0_dn5 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn5)) * assign47080_e60487) + (assign47080_e60484 * (-locals.var_sg_dn5))), ((((locals.var_igc0_dn6 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn6)) * assign47080_e60487) + (assign47080_e60484 * (-locals.var_sg_dn6))), ((((locals.var_igc0_dn7 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn7)) * assign47080_e60487) + (assign47080_e60484 * (-locals.var_sg_dn7))), ((((locals.var_igc0_dn8 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn8)) * assign47080_e60487) + (assign47080_e60484 * (-locals.var_sg_dn8))), );
        }

        (locals.var_i_gidl, locals.var_i_gidl_dn5, locals.var_i_gidl_dn6, locals.var_i_gidl_dn7, locals.var_i_gidl_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_i_gisl, locals.var_i_gisl_dn5, locals.var_i_gisl_dn6, locals.var_i_gisl_dn7, locals.var_i_gisl_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign47110_e60495: f64 = if p.p42 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1244 = assign47110_e60495;

        let assign47120_e60502: f64 = if ((locals.var_agidld_i > 0.0) && (locals.var_vovd < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1245 = assign47120_e60502;

        if ((locals.var_guard1244 != 0.0) && (locals.var_guard1245 != 0.0)) {
            let assign47130_e60508: f64 = (locals.var_vovd * locals.var_vovd);
            let assign47130_e60511: f64 = (locals.var_cgidld_i * locals.var_cgidld_i);
            let assign47130_e60514: f64 = (locals.var_vdbprime * locals.var_vdbprime);
            let assign47130_e60515: f64 = (assign47130_e60511 * assign47130_e60514);
            let assign47130_e60516: f64 = (assign47130_e60508 + assign47130_e60515);
            let assign47130_e60518: f64 = (assign47130_e60516 + 1e-6);
            let assign47130_e60519: f64 = (assign47130_e60518).sqrt();
            (locals.var_vtovd, locals.var_vtovd_dn5, locals.var_vtovd_dn6, locals.var_vtovd_dn7, locals.var_vtovd_dn8, ) = (assign47130_e60519, (((locals.var_vovd_dn5 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn5)) / (2.0 * assign47130_e60519)), ((((locals.var_vovd_dn6 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn6)) + (assign47130_e60511 * ((locals.var_vdbprime_dn6 * locals.var_vdbprime) + (locals.var_vdbprime * locals.var_vdbprime_dn6)))) / (2.0 * assign47130_e60519)), ((((locals.var_vovd_dn7 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn7)) + (assign47130_e60511 * ((locals.var_vdbprime_dn7 * locals.var_vdbprime) + (locals.var_vdbprime * locals.var_vdbprime_dn7)))) / (2.0 * assign47130_e60519)), ((assign47130_e60511 * ((locals.var_vdbprime_dn8 * locals.var_vdbprime) + (locals.var_vdbprime * locals.var_vdbprime_dn8))) / (2.0 * assign47130_e60519)), );
        }

        if ((locals.var_guard1244 != 0.0) && (locals.var_guard1245 != 0.0)) {
            let assign47140_e60526: f64 = (-locals.var_bgidlds);
            let assign47140_e60528: f64 = (assign47140_e60526 / locals.var_vtovd);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign47140_e60528, (-((assign47140_e60526 * locals.var_vtovd_dn5) / (locals.var_vtovd * locals.var_vtovd))), (-((assign47140_e60526 * locals.var_vtovd_dn6) / (locals.var_vtovd * locals.var_vtovd))), (-((assign47140_e60526 * locals.var_vtovd_dn7) / (locals.var_vtovd * locals.var_vtovd))), (-((assign47140_e60526 * locals.var_vtovd_dn8) / (locals.var_vtovd * locals.var_vtovd))), );
        }

        let assign47150_e60533: f64 = (-230.25850929940458);
        let assign47150_e60534: f64 = if locals.var_temp__blk936 > assign47150_e60533 { 1.0 } else { 0.0 };
        locals.var_guard1246 = assign47150_e60534;

        if (((locals.var_guard1244 != 0.0) && (locals.var_guard1245 != 0.0)) && (locals.var_guard1246 != 0.0)) {
            let assign47160_e60541: f64 = (locals.var_temp__blk936).exp();
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign47160_e60541, (assign47160_e60541 * locals.var_temp__blk936_dn5), (assign47160_e60541 * locals.var_temp__blk936_dn6), (assign47160_e60541 * locals.var_temp__blk936_dn7), (assign47160_e60541 * locals.var_temp__blk936_dn8), );
        }

        if (((locals.var_guard1244 != 0.0) && (locals.var_guard1245 != 0.0)) && (locals.var_guard1246 == 0.0)) {
            let assign47170_e60553: f64 = (-230.25850929940458);
            let assign47170_e60555: f64 = (assign47170_e60553 - locals.var_temp__blk936);
            let assign47170_e60559: f64 = (-230.25850929940458);
            let assign47170_e60561: f64 = (assign47170_e60559 - locals.var_temp__blk936);
            let assign47170_e60564: f64 = (-230.25850929940458);
            let assign47170_e60566: f64 = (assign47170_e60564 - locals.var_temp__blk936);
            let assign47170_e60568: f64 = (assign47170_e60566 * 0.3333333333333333);
            let assign47170_e60569: f64 = (1.0 + assign47170_e60568);
            let assign47170_e60570: f64 = (assign47170_e60561 * assign47170_e60569);
            let assign47170_e60571: f64 = (0.5 * assign47170_e60570);
            let assign47170_e60572: f64 = (1.0 + assign47170_e60571);
            let assign47170_e60573: f64 = (assign47170_e60555 * assign47170_e60572);
            let assign47170_e60574: f64 = (1.0 + assign47170_e60573);
            let assign47170_e60575: f64 = (1e-100 / assign47170_e60574);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign47170_e60575, (-((1e-100 * (((-locals.var_temp__blk936_dn5) * assign47170_e60572) + (assign47170_e60555 * (0.5 * (((-locals.var_temp__blk936_dn5) * assign47170_e60569) + (assign47170_e60561 * ((-locals.var_temp__blk936_dn5) * 0.3333333333333333))))))) / (assign47170_e60574 * assign47170_e60574))), (-((1e-100 * (((-locals.var_temp__blk936_dn6) * assign47170_e60572) + (assign47170_e60555 * (0.5 * (((-locals.var_temp__blk936_dn6) * assign47170_e60569) + (assign47170_e60561 * ((-locals.var_temp__blk936_dn6) * 0.3333333333333333))))))) / (assign47170_e60574 * assign47170_e60574))), (-((1e-100 * (((-locals.var_temp__blk936_dn7) * assign47170_e60572) + (assign47170_e60555 * (0.5 * (((-locals.var_temp__blk936_dn7) * assign47170_e60569) + (assign47170_e60561 * ((-locals.var_temp__blk936_dn7) * 0.3333333333333333))))))) / (assign47170_e60574 * assign47170_e60574))), (-((1e-100 * (((-locals.var_temp__blk936_dn8) * assign47170_e60572) + (assign47170_e60555 * (0.5 * (((-locals.var_temp__blk936_dn8) * assign47170_e60569) + (assign47170_e60561 * ((-locals.var_temp__blk936_dn8) * 0.3333333333333333))))))) / (assign47170_e60574 * assign47170_e60574))), );
        }

        if ((locals.var_guard1244 != 0.0) && (locals.var_guard1245 != 0.0)) {
            let assign47180_e60582: f64 = (-locals.var_agidlds);
            let assign47180_e60585: f64 = (locals.var_vdbprime * locals.var_vovd);
            let assign47180_e60587: f64 = (assign47180_e60585 * locals.var_vtovd);
            let assign47180_e60589: f64 = (assign47180_e60587 * locals.var_temp2);
            let assign47180_e60590: f64 = (assign47180_e60582 * assign47180_e60589);
            (locals.var_i_gidl, locals.var_i_gidl_dn5, locals.var_i_gidl_dn6, locals.var_i_gidl_dn7, locals.var_i_gidl_dn8, ) = (assign47180_e60590, (assign47180_e60582 * (((((locals.var_vdbprime * locals.var_vovd_dn5) * locals.var_vtovd) + (assign47180_e60585 * locals.var_vtovd_dn5)) * locals.var_temp2) + (assign47180_e60587 * locals.var_temp2_dn5))), (assign47180_e60582 * ((((((locals.var_vdbprime_dn6 * locals.var_vovd) + (locals.var_vdbprime * locals.var_vovd_dn6)) * locals.var_vtovd) + (assign47180_e60585 * locals.var_vtovd_dn6)) * locals.var_temp2) + (assign47180_e60587 * locals.var_temp2_dn6))), (assign47180_e60582 * ((((((locals.var_vdbprime_dn7 * locals.var_vovd) + (locals.var_vdbprime * locals.var_vovd_dn7)) * locals.var_vtovd) + (assign47180_e60585 * locals.var_vtovd_dn7)) * locals.var_temp2) + (assign47180_e60587 * locals.var_temp2_dn7))), (assign47180_e60582 * (((((locals.var_vdbprime_dn8 * locals.var_vovd) * locals.var_vtovd) + (assign47180_e60585 * locals.var_vtovd_dn8)) * locals.var_temp2) + (assign47180_e60587 * locals.var_temp2_dn8))), );
        }

        let assign47190_e60599: f64 = if ((locals.var_agidl_i > 0.0) && (locals.var_vovs < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1247 = assign47190_e60599;

        if ((locals.var_guard1244 != 0.0) && (locals.var_guard1247 != 0.0)) {
            let assign47200_e60605: f64 = (locals.var_vovs * locals.var_vovs);
            let assign47200_e60608: f64 = (locals.var_cgidl_i * locals.var_cgidl_i);
            let assign47200_e60611: f64 = (locals.var_vsbprime * locals.var_vsbprime);
            let assign47200_e60612: f64 = (assign47200_e60608 * assign47200_e60611);
            let assign47200_e60613: f64 = (assign47200_e60605 + assign47200_e60612);
            let assign47200_e60615: f64 = (assign47200_e60613 + 1e-6);
            let assign47200_e60616: f64 = (assign47200_e60615).sqrt();
            (locals.var_vtovs, locals.var_vtovs_dn5, locals.var_vtovs_dn6, locals.var_vtovs_dn7, locals.var_vtovs_dn8, ) = (assign47200_e60616, (((locals.var_vovs_dn5 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn5)) / (2.0 * assign47200_e60616)), ((((locals.var_vovs_dn6 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn6)) + (assign47200_e60608 * ((locals.var_vsbprime_dn6 * locals.var_vsbprime) + (locals.var_vsbprime * locals.var_vsbprime_dn6)))) / (2.0 * assign47200_e60616)), ((((locals.var_vovs_dn7 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn7)) + (assign47200_e60608 * ((locals.var_vsbprime_dn7 * locals.var_vsbprime) + (locals.var_vsbprime * locals.var_vsbprime_dn7)))) / (2.0 * assign47200_e60616)), ((assign47200_e60608 * ((locals.var_vsbprime_dn8 * locals.var_vsbprime) + (locals.var_vsbprime * locals.var_vsbprime_dn8))) / (2.0 * assign47200_e60616)), );
        }

        if ((locals.var_guard1244 != 0.0) && (locals.var_guard1247 != 0.0)) {
            let assign47210_e60623: f64 = (-locals.var_bgidls);
            let assign47210_e60625: f64 = (assign47210_e60623 / locals.var_vtovs);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign47210_e60625, (-((assign47210_e60623 * locals.var_vtovs_dn5) / (locals.var_vtovs * locals.var_vtovs))), (-((assign47210_e60623 * locals.var_vtovs_dn6) / (locals.var_vtovs * locals.var_vtovs))), (-((assign47210_e60623 * locals.var_vtovs_dn7) / (locals.var_vtovs * locals.var_vtovs))), (-((assign47210_e60623 * locals.var_vtovs_dn8) / (locals.var_vtovs * locals.var_vtovs))), );
        }

        let assign47220_e60630: f64 = (-230.25850929940458);
        let assign47220_e60631: f64 = if locals.var_temp__blk936 > assign47220_e60630 { 1.0 } else { 0.0 };
        locals.var_guard1248 = assign47220_e60631;

        if (((locals.var_guard1244 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1248 != 0.0)) {
            let assign47230_e60638: f64 = (locals.var_temp__blk936).exp();
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign47230_e60638, (assign47230_e60638 * locals.var_temp__blk936_dn5), (assign47230_e60638 * locals.var_temp__blk936_dn6), (assign47230_e60638 * locals.var_temp__blk936_dn7), (assign47230_e60638 * locals.var_temp__blk936_dn8), );
        }

        if (((locals.var_guard1244 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1248 == 0.0)) {
            let assign47240_e60650: f64 = (-230.25850929940458);
            let assign47240_e60652: f64 = (assign47240_e60650 - locals.var_temp__blk936);
            let assign47240_e60656: f64 = (-230.25850929940458);
            let assign47240_e60658: f64 = (assign47240_e60656 - locals.var_temp__blk936);
            let assign47240_e60661: f64 = (-230.25850929940458);
            let assign47240_e60663: f64 = (assign47240_e60661 - locals.var_temp__blk936);
            let assign47240_e60665: f64 = (assign47240_e60663 * 0.3333333333333333);
            let assign47240_e60666: f64 = (1.0 + assign47240_e60665);
            let assign47240_e60667: f64 = (assign47240_e60658 * assign47240_e60666);
            let assign47240_e60668: f64 = (0.5 * assign47240_e60667);
            let assign47240_e60669: f64 = (1.0 + assign47240_e60668);
            let assign47240_e60670: f64 = (assign47240_e60652 * assign47240_e60669);
            let assign47240_e60671: f64 = (1.0 + assign47240_e60670);
            let assign47240_e60672: f64 = (1e-100 / assign47240_e60671);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign47240_e60672, (-((1e-100 * (((-locals.var_temp__blk936_dn5) * assign47240_e60669) + (assign47240_e60652 * (0.5 * (((-locals.var_temp__blk936_dn5) * assign47240_e60666) + (assign47240_e60658 * ((-locals.var_temp__blk936_dn5) * 0.3333333333333333))))))) / (assign47240_e60671 * assign47240_e60671))), (-((1e-100 * (((-locals.var_temp__blk936_dn6) * assign47240_e60669) + (assign47240_e60652 * (0.5 * (((-locals.var_temp__blk936_dn6) * assign47240_e60666) + (assign47240_e60658 * ((-locals.var_temp__blk936_dn6) * 0.3333333333333333))))))) / (assign47240_e60671 * assign47240_e60671))), (-((1e-100 * (((-locals.var_temp__blk936_dn7) * assign47240_e60669) + (assign47240_e60652 * (0.5 * (((-locals.var_temp__blk936_dn7) * assign47240_e60666) + (assign47240_e60658 * ((-locals.var_temp__blk936_dn7) * 0.3333333333333333))))))) / (assign47240_e60671 * assign47240_e60671))), (-((1e-100 * (((-locals.var_temp__blk936_dn8) * assign47240_e60669) + (assign47240_e60652 * (0.5 * (((-locals.var_temp__blk936_dn8) * assign47240_e60666) + (assign47240_e60658 * ((-locals.var_temp__blk936_dn8) * 0.3333333333333333))))))) / (assign47240_e60671 * assign47240_e60671))), );
        }

        if ((locals.var_guard1244 != 0.0) && (locals.var_guard1247 != 0.0)) {
            let assign47250_e60679: f64 = (-locals.var_agidls);
            let assign47250_e60682: f64 = (locals.var_vsbprime * locals.var_vovs);
            let assign47250_e60684: f64 = (assign47250_e60682 * locals.var_vtovs);
            let assign47250_e60686: f64 = (assign47250_e60684 * locals.var_temp2);
            let assign47250_e60687: f64 = (assign47250_e60679 * assign47250_e60686);
            (locals.var_i_gisl, locals.var_i_gisl_dn5, locals.var_i_gisl_dn6, locals.var_i_gisl_dn7, locals.var_i_gisl_dn8, ) = (assign47250_e60687, (assign47250_e60679 * (((((locals.var_vsbprime * locals.var_vovs_dn5) * locals.var_vtovs) + (assign47250_e60682 * locals.var_vtovs_dn5)) * locals.var_temp2) + (assign47250_e60684 * locals.var_temp2_dn5))), (assign47250_e60679 * ((((((locals.var_vsbprime_dn6 * locals.var_vovs) + (locals.var_vsbprime * locals.var_vovs_dn6)) * locals.var_vtovs) + (assign47250_e60682 * locals.var_vtovs_dn6)) * locals.var_temp2) + (assign47250_e60684 * locals.var_temp2_dn6))), (assign47250_e60679 * ((((((locals.var_vsbprime_dn7 * locals.var_vovs) + (locals.var_vsbprime * locals.var_vovs_dn7)) * locals.var_vtovs) + (assign47250_e60682 * locals.var_vtovs_dn7)) * locals.var_temp2) + (assign47250_e60684 * locals.var_temp2_dn7))), (assign47250_e60679 * (((((locals.var_vsbprime_dn8 * locals.var_vovs) * locals.var_vtovs) + (assign47250_e60682 * locals.var_vtovs_dn8)) * locals.var_temp2) + (assign47250_e60684 * locals.var_temp2_dn8))), );
        }

        (locals.var_phit1edge, locals.var_phit1edge_dn5, locals.var_phit1edge_dn6, locals.var_phit1edge_dn7, locals.var_phit1edge_dn8, ) = (locals.var_phit, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_xgedge, locals.var_xgedge_dn5, locals.var_xgedge_dn6, locals.var_xgedge_dn7, locals.var_xgedge_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_qdseffedge, locals.var_qdseffedge_dn5, locals.var_qdseffedge_dn6, locals.var_qdseffedge_dn7, locals.var_qdseffedge_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_qmeffedge, locals.var_qmeffedge_dn5, locals.var_qmeffedge_dn6, locals.var_qmeffedge_dn7, locals.var_qmeffedge_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_dsqredge, locals.var_dsqredge_dn5, locals.var_dsqredge_dn6, locals.var_dsqredge_dn7, locals.var_dsqredge_dn8, ) = (1e-40, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_alphabmedge, locals.var_alphabmedge_dn5, locals.var_alphabmedge_dn6, locals.var_alphabmedge_dn7, locals.var_alphabmedge_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_i_dsedge, locals.var_i_dsedge_dn5, locals.var_i_dsedge_dn6, locals.var_i_dsedge_dn7, locals.var_i_dsedge_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign47330_e60703: f64 = if ((p.p46 != 0.0) && (locals.var_betnedge_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1249 = assign47330_e60703;

    }

    pub(super) fn stamp_transient_block_18(
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard1249 != 0.0) {
            let assign47340_e60708: f64 = (locals.var_v_db + locals.var_v_sb);
            let assign47340_e60711: f64 = (locals.var_v_db - locals.var_v_sb);
            let assign47340_e60714: f64 = (locals.var_v_db - locals.var_v_sb);
            let assign47340_e60715: f64 = (assign47340_e60711 * assign47340_e60714);
            let assign47340_e60717: f64 = (assign47340_e60715 + locals.var_bphiedge);
            let assign47340_e60718: f64 = (assign47340_e60717).sqrt();
            let assign47340_e60719: f64 = (assign47340_e60708 - assign47340_e60718);
            let assign47340_e60720: f64 = (0.5 * assign47340_e60719);
            let assign47340_e60722: f64 = (assign47340_e60720 + locals.var_phixedge);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign47340_e60722, 0.0, (0.5 * ((locals.var_v_db_dn6 + locals.var_v_sb_dn6) - ((((locals.var_v_db_dn6 - locals.var_v_sb_dn6) * assign47340_e60714) + (assign47340_e60711 * (locals.var_v_db_dn6 - locals.var_v_sb_dn6))) / (2.0 * assign47340_e60718)))), (0.5 * ((locals.var_v_db_dn7 + locals.var_v_sb_dn7) - ((((locals.var_v_db_dn7 - locals.var_v_sb_dn7) * assign47340_e60714) + (assign47340_e60711 * (locals.var_v_db_dn7 - locals.var_v_sb_dn7))) / (2.0 * assign47340_e60718)))), (0.5 * ((locals.var_v_db_dn8 + locals.var_v_sb_dn8) - ((((locals.var_v_db_dn8 - locals.var_v_sb_dn8) * assign47340_e60714) + (assign47340_e60711 * (locals.var_v_db_dn8 - locals.var_v_sb_dn8))) / (2.0 * assign47340_e60718)))), );
        }

        if (locals.var_guard1249 != 0.0) {
            let assign47350_e60730: f64 = locals.var_temp__blk936;
            let assign47350_e60733: f64 = locals.var_temp__blk936;
            let assign47350_e60736: f64 = locals.var_temp__blk936;
            let assign47350_e60737: f64 = (assign47350_e60733 * assign47350_e60736);
            let assign47350_e60739: f64 = (assign47350_e60737 + locals.var_aphiedge);
            let assign47350_e60740: f64 = (assign47350_e60739).sqrt();
            let assign47350_e60741: f64 = (assign47350_e60730 - assign47350_e60740);
            let assign47350_e60742: f64 = (0.5 * assign47350_e60741);
            let assign47350_e60743: f64 = (locals.var_v_sb - assign47350_e60742);
            let assign47350_e60745: f64 = (assign47350_e60743 + locals.var_phix1edge);
            (locals.var_vsbstaredge, locals.var_vsbstaredge_dn5, locals.var_vsbstaredge_dn6, locals.var_vsbstaredge_dn7, locals.var_vsbstaredge_dn8, ) = (assign47350_e60745, (-(0.5 * (locals.var_temp__blk936_dn5 - (((locals.var_temp__blk936_dn5 * assign47350_e60736) + (assign47350_e60733 * locals.var_temp__blk936_dn5)) / (2.0 * assign47350_e60740))))), (locals.var_v_sb_dn6 - (0.5 * (locals.var_temp__blk936_dn6 - (((locals.var_temp__blk936_dn6 * assign47350_e60736) + (assign47350_e60733 * locals.var_temp__blk936_dn6)) / (2.0 * assign47350_e60740))))), (locals.var_v_sb_dn7 - (0.5 * (locals.var_temp__blk936_dn7 - (((locals.var_temp__blk936_dn7 * assign47350_e60736) + (assign47350_e60733 * locals.var_temp__blk936_dn7)) / (2.0 * assign47350_e60740))))), (locals.var_v_sb_dn8 - (0.5 * (locals.var_temp__blk936_dn8 - (((locals.var_temp__blk936_dn8 * assign47350_e60736) + (assign47350_e60733 * locals.var_temp__blk936_dn8)) / (2.0 * assign47350_e60740))))), );
        }

        if (locals.var_guard1249 != 0.0) {
            let assign47360_e60753: f64 = (locals.var_v_ds - locals.var_vdsx);
            let assign47360_e60754: f64 = (0.5 * assign47360_e60753);
            let assign47360_e60755: f64 = (locals.var_vsbstaredge + assign47360_e60754);
            (locals.var_vsbxedge, locals.var_vsbxedge_dn5, locals.var_vsbxedge_dn6, locals.var_vsbxedge_dn7, locals.var_vsbxedge_dn8, ) = (assign47360_e60755, locals.var_vsbstaredge_dn5, (locals.var_vsbstaredge_dn6 + (0.5 * (locals.var_v_ds_dn6 - locals.var_vdsx_dn6))), (locals.var_vsbstaredge_dn7 + (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7))), locals.var_vsbstaredge_dn8, );
        }

        if (locals.var_guard1249 != 0.0) {
            let assign47370_e60763: f64 = (locals.var_pscededge_i * locals.var_vdsx);
            let assign47370_e60764: f64 = (1.0 + assign47370_e60763);
            let assign47370_e60765: f64 = (locals.var_psceedge_i * assign47370_e60764);
            let assign47370_e60769: f64 = (locals.var_pscebedge_i * locals.var_vsbxedge);
            let assign47370_e60770: f64 = (1.0 + assign47370_e60769);
            let assign47370_e60771: f64 = (assign47370_e60765 * assign47370_e60770);
            (locals.var_dphit1edge, locals.var_dphit1edge_dn5, locals.var_dphit1edge_dn6, locals.var_dphit1edge_dn7, locals.var_dphit1edge_dn8, ) = (assign47370_e60771, (assign47370_e60765 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn5)), (((locals.var_psceedge_i * (locals.var_pscededge_i * locals.var_vdsx_dn6)) * assign47370_e60770) + (assign47370_e60765 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn6))), (((locals.var_psceedge_i * (locals.var_pscededge_i * locals.var_vdsx_dn7)) * assign47370_e60770) + (assign47370_e60765 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn7))), (assign47370_e60765 * (locals.var_pscebedge_i * locals.var_vsbxedge_dn8)), );
        }

        if (locals.var_guard1249 != 0.0) {
            let assign47380_e60778: f64 = (1.0 + locals.var_dphit1edge);
            let assign47380_e60779: f64 = (locals.var_phit0edge * assign47380_e60778);
            (locals.var_phit1edge, locals.var_phit1edge_dn5, locals.var_phit1edge_dn6, locals.var_phit1edge_dn7, locals.var_phit1edge_dn8, ) = (assign47380_e60779, (locals.var_phit0edge * locals.var_dphit1edge_dn5), (locals.var_phit0edge * locals.var_dphit1edge_dn6), (locals.var_phit0edge * locals.var_dphit1edge_dn7), (locals.var_phit0edge * locals.var_dphit1edge_dn8), );
        }

        if (locals.var_guard1249 != 0.0) {
            let assign47390_e60785: f64 = (1.0 / locals.var_phit1edge);
            (locals.var_inv_phit1edge, locals.var_inv_phit1edge_dn5, locals.var_inv_phit1edge_dn6, locals.var_inv_phit1edge_dn7, locals.var_inv_phit1edge_dn8, ) = (assign47390_e60785, (-(locals.var_phit1edge_dn5 / (locals.var_phit1edge * locals.var_phit1edge))), (-(locals.var_phit1edge_dn6 / (locals.var_phit1edge * locals.var_phit1edge))), (-(locals.var_phit1edge_dn7 / (locals.var_phit1edge * locals.var_phit1edge))), (-(locals.var_phit1edge_dn8 / (locals.var_phit1edge * locals.var_phit1edge))), );
        }

        if (locals.var_guard1249 != 0.0) {
            let assign47400_e60791: f64 = (2.0 * locals.var_vdsx);
            let assign47400_e60796: f64 = (locals.var_cfdedge_i * locals.var_vdsx);
            let assign47400_e60797: f64 = (1.0 + assign47400_e60796);
            let assign47400_e60798: f64 = (assign47400_e60797).sqrt();
            let assign47400_e60799: f64 = (1.0 + assign47400_e60798);
            let assign47400_e60800: f64 = (assign47400_e60791 / assign47400_e60799);
            (locals.var_vdspedge, locals.var_vdspedge_dn6, locals.var_vdspedge_dn7, ) = (assign47400_e60800, ((((2.0 * locals.var_vdsx_dn6) * assign47400_e60799) - (assign47400_e60791 * ((locals.var_cfdedge_i * locals.var_vdsx_dn6) / (2.0 * assign47400_e60798)))) / (assign47400_e60799 * assign47400_e60799)), ((((2.0 * locals.var_vdsx_dn7) * assign47400_e60799) - (assign47400_e60791 * ((locals.var_cfdedge_i * locals.var_vdsx_dn7) / (2.0 * assign47400_e60798)))) / (assign47400_e60799 * assign47400_e60799)), );
        }

        if (locals.var_guard1249 != 0.0) {
            let assign47410_e60806: f64 = (locals.var_cfedge_i * locals.var_vdspedge);
            let assign47410_e60810: f64 = (locals.var_cfbedge_i * locals.var_vsbxedge);
            let assign47410_e60811: f64 = (1.0 + assign47410_e60810);
            let assign47410_e60812: f64 = (assign47410_e60806 * assign47410_e60811);
            (locals.var_delvgedge, locals.var_delvgedge_dn5, locals.var_delvgedge_dn6, locals.var_delvgedge_dn7, locals.var_delvgedge_dn8, ) = (assign47410_e60812, (assign47410_e60806 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn5)), (((locals.var_cfedge_i * locals.var_vdspedge_dn6) * assign47410_e60811) + (assign47410_e60806 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn6))), (((locals.var_cfedge_i * locals.var_vdspedge_dn7) * assign47410_e60811) + (assign47410_e60806 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn7))), (assign47410_e60806 * (locals.var_cfbedge_i * locals.var_vsbxedge_dn8)), );
        }

        if (locals.var_guard1249 != 0.0) {
            let assign47420_e60819: f64 = (locals.var_vgb + locals.var_delvgedge);
            let assign47420_e60821: f64 = (assign47420_e60819 - locals.var_vfbedge_t);
            let assign47420_e60822: f64 = (locals.var_inv_phit1edge * assign47420_e60821);
            (locals.var_xgedge, locals.var_xgedge_dn5, locals.var_xgedge_dn6, locals.var_xgedge_dn7, locals.var_xgedge_dn8, ) = (assign47420_e60822, ((locals.var_inv_phit1edge_dn5 * assign47420_e60821) + (locals.var_inv_phit1edge * (locals.var_vgb_dn5 + locals.var_delvgedge_dn5))), ((locals.var_inv_phit1edge_dn6 * assign47420_e60821) + (locals.var_inv_phit1edge * (locals.var_vgb_dn6 + locals.var_delvgedge_dn6))), ((locals.var_inv_phit1edge_dn7 * assign47420_e60821) + (locals.var_inv_phit1edge * (locals.var_vgb_dn7 + locals.var_delvgedge_dn7))), ((locals.var_inv_phit1edge_dn8 * assign47420_e60821) + (locals.var_inv_phit1edge * (locals.var_vgb_dn8 + locals.var_delvgedge_dn8))), );
        }

        if (locals.var_guard1249 != 0.0) {
            let assign47430_e60828: f64 = (locals.var_inv_phit1edge * locals.var_phibedge);
            (locals.var_xbedge, locals.var_xbedge_dn5, locals.var_xbedge_dn6, locals.var_xbedge_dn7, locals.var_xbedge_dn8, ) = (assign47430_e60828, (locals.var_inv_phit1edge_dn5 * locals.var_phibedge), (locals.var_inv_phit1edge_dn6 * locals.var_phibedge), (locals.var_inv_phit1edge_dn7 * locals.var_phibedge), (locals.var_inv_phit1edge_dn8 * locals.var_phibedge), );
        }

        if (locals.var_guard1249 != 0.0) {
            let assign47440_e60835: f64 = (locals.var_xbedge / locals.var_gfedge);
            let assign47440_e60837: f64 = (locals.var_xbedge).sqrt();
            let assign47440_e60838: f64 = (assign47440_e60835 + assign47440_e60837);
            let assign47440_e60839: f64 = (assign47440_e60838).ln();
            let assign47440_e60840: f64 = (2.0 * assign47440_e60839);
            (locals.var_dxthedge, locals.var_dxthedge_dn5, locals.var_dxthedge_dn6, locals.var_dxthedge_dn7, locals.var_dxthedge_dn8, ) = (assign47440_e60840, (2.0 * (((locals.var_xbedge_dn5 / locals.var_gfedge) + (locals.var_xbedge_dn5 / (2.0 * assign47440_e60837))) / assign47440_e60838)), (2.0 * (((locals.var_xbedge_dn6 / locals.var_gfedge) + (locals.var_xbedge_dn6 / (2.0 * assign47440_e60837))) / assign47440_e60838)), (2.0 * (((locals.var_xbedge_dn7 / locals.var_gfedge) + (locals.var_xbedge_dn7 / (2.0 * assign47440_e60837))) / assign47440_e60838)), (2.0 * (((locals.var_xbedge_dn8 / locals.var_gfedge) + (locals.var_xbedge_dn8 / (2.0 * assign47440_e60837))) / assign47440_e60838)), );
        }

        if (locals.var_guard1249 != 0.0) {
            let assign47450_e60846: f64 = (locals.var_inv_phit1edge * locals.var_vsbstaredge);
            (locals.var_xnedge_s, locals.var_xnedge_s_dn5, locals.var_xnedge_s_dn6, locals.var_xnedge_s_dn7, locals.var_xnedge_s_dn8, ) = (assign47450_e60846, ((locals.var_inv_phit1edge_dn5 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn5)), ((locals.var_inv_phit1edge_dn6 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn6)), ((locals.var_inv_phit1edge_dn7 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn7)), ((locals.var_inv_phit1edge_dn8 * locals.var_vsbstaredge) + (locals.var_inv_phit1edge * locals.var_vsbstaredge_dn8)), );
        }

        if (locals.var_guard1249 != 0.0) {
            let assign47460_e60852: f64 = (locals.var_xbedge + locals.var_xnedge_s);
            (locals.var_q_edge_xsth, locals.var_q_edge_xsth_dn5, locals.var_q_edge_xsth_dn6, locals.var_q_edge_xsth_dn7, locals.var_q_edge_xsth_dn8, ) = (assign47460_e60852, (locals.var_xbedge_dn5 + locals.var_xnedge_s_dn5), (locals.var_xbedge_dn6 + locals.var_xnedge_s_dn6), (locals.var_xbedge_dn7 + locals.var_xnedge_s_dn7), (locals.var_xbedge_dn8 + locals.var_xnedge_s_dn8), );
        }

        if (locals.var_guard1249 != 0.0) {
            let assign47470_e60859: f64 = (locals.var_q_edge_xsth).sqrt();
            let assign47470_e60860: f64 = (locals.var_gfedge * assign47470_e60859);
            let assign47470_e60861: f64 = (locals.var_q_edge_xsth + assign47470_e60860);
            (locals.var_q_edge_xth0, locals.var_q_edge_xth0_dn5, locals.var_q_edge_xth0_dn6, locals.var_q_edge_xth0_dn7, locals.var_q_edge_xth0_dn8, ) = (assign47470_e60861, (locals.var_q_edge_xsth_dn5 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn5 / (2.0 * assign47470_e60859)))), (locals.var_q_edge_xsth_dn6 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47470_e60859)))), (locals.var_q_edge_xsth_dn7 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47470_e60859)))), (locals.var_q_edge_xsth_dn8 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47470_e60859)))), );
        }

        if (locals.var_guard1249 != 0.0) {
            let assign47480_e60867: f64 = (locals.var_q_edge_xth0 + locals.var_dxthedge);
            (locals.var_q_edge_xth, locals.var_q_edge_xth_dn5, locals.var_q_edge_xth_dn6, locals.var_q_edge_xth_dn7, locals.var_q_edge_xth_dn8, ) = (assign47480_e60867, (locals.var_q_edge_xth0_dn5 + locals.var_dxthedge_dn5), (locals.var_q_edge_xth0_dn6 + locals.var_dxthedge_dn6), (locals.var_q_edge_xth0_dn7 + locals.var_dxthedge_dn7), (locals.var_q_edge_xth0_dn8 + locals.var_dxthedge_dn8), );
        }

        if (locals.var_guard1249 != 0.0) {
            let assign47490_e60875: f64 = (locals.var_q_edge_xsth).sqrt();
            let assign47490_e60876: f64 = (2.0 * assign47490_e60875);
            let assign47490_e60877: f64 = (locals.var_gfedge / assign47490_e60876);
            let assign47490_e60878: f64 = (1.0 + assign47490_e60877);
            (locals.var_q_edge_n, locals.var_q_edge_n_dn5, locals.var_q_edge_n_dn6, locals.var_q_edge_n_dn7, locals.var_q_edge_n_dn8, ) = (assign47490_e60878, (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn5 / (2.0 * assign47490_e60875)))) / (assign47490_e60876 * assign47490_e60876))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47490_e60875)))) / (assign47490_e60876 * assign47490_e60876))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47490_e60875)))) / (assign47490_e60876 * assign47490_e60876))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47490_e60875)))) / (assign47490_e60876 * assign47490_e60876))), );
        }

        if (locals.var_guard1249 != 0.0) {
            let assign47500_e60884: f64 = (1.0 / locals.var_q_edge_n);
            (locals.var_q_edge_n_inv, locals.var_q_edge_n_inv_dn5, locals.var_q_edge_n_inv_dn6, locals.var_q_edge_n_inv_dn7, locals.var_q_edge_n_inv_dn8, ) = (assign47500_e60884, (-(locals.var_q_edge_n_dn5 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn6 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn7 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn8 / (locals.var_q_edge_n * locals.var_q_edge_n))), );
        }

        if (locals.var_guard1249 != 0.0) {
            let assign47510_e60890: f64 = (locals.var_xgedge - locals.var_q_edge_xth);
            (locals.var_q_edge_xgt, locals.var_q_edge_xgt_dn5, locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8, ) = (assign47510_e60890, (locals.var_xgedge_dn5 - locals.var_q_edge_xth_dn5), (locals.var_xgedge_dn6 - locals.var_q_edge_xth_dn6), (locals.var_xgedge_dn7 - locals.var_q_edge_xth_dn7), (locals.var_xgedge_dn8 - locals.var_q_edge_xth_dn8), );
        }

        let assign47520_e60895: f64 = (-12.0);
        let assign47520_e60896: f64 = if locals.var_q_edge_xgt > assign47520_e60895 { 1.0 } else { 0.0 };
        locals.var_guard1250 = assign47520_e60896;

        if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
            let assign47530_e60902: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
            let assign47530_e60904: f64 = (assign47530_e60902 - 1.0);
            (locals.var_q_edge_xgt0, locals.var_q_edge_xgt0_dn5, locals.var_q_edge_xgt0_dn6, locals.var_q_edge_xgt0_dn7, locals.var_q_edge_xgt0_dn8, ) = (assign47530_e60904, locals.var_q_edge_xgt_dn5, locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8, );
        }

        if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
            let assign47540_e60914: f64 = (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0);
            let assign47540_e60916: f64 = (assign47540_e60914 + 10.0);
            let assign47540_e60917: f64 = (assign47540_e60916).sqrt();
            let assign47540_e60918: f64 = (locals.var_q_edge_xgt0 + assign47540_e60917);
            let assign47540_e60919: f64 = (0.5 * assign47540_e60918);
            (locals.var_q_edge_xgt0e, locals.var_q_edge_xgt0e_dn5, locals.var_q_edge_xgt0e_dn6, locals.var_q_edge_xgt0e_dn7, locals.var_q_edge_xgt0e_dn8, ) = (assign47540_e60919, (0.5 * (locals.var_q_edge_xgt0_dn5 + (((locals.var_q_edge_xgt0_dn5 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn5)) / (2.0 * assign47540_e60917)))), (0.5 * (locals.var_q_edge_xgt0_dn6 + (((locals.var_q_edge_xgt0_dn6 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn6)) / (2.0 * assign47540_e60917)))), (0.5 * (locals.var_q_edge_xgt0_dn7 + (((locals.var_q_edge_xgt0_dn7 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn7)) / (2.0 * assign47540_e60917)))), (0.5 * (locals.var_q_edge_xgt0_dn8 + (((locals.var_q_edge_xgt0_dn8 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn8)) / (2.0 * assign47540_e60917)))), );
        }

        if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
            let assign47550_e60928: f64 = (locals.var_q_edge_xgt0e).ln();
            let assign47550_e60929: f64 = (locals.var_q_edge_n * assign47550_e60928);
            let assign47550_e60930: f64 = (locals.var_q_edge_xgt - assign47550_e60929);
            let assign47550_e60932: f64 = (assign47550_e60930 + locals.var_lngfedge2);
            (locals.var_q_edge_qi0si, locals.var_q_edge_qi0si_dn5, locals.var_q_edge_qi0si_dn6, locals.var_q_edge_qi0si_dn7, locals.var_q_edge_qi0si_dn8, ) = (assign47550_e60932, (locals.var_q_edge_xgt_dn5 - ((locals.var_q_edge_n_dn5 * assign47550_e60928) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn5 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn6 - ((locals.var_q_edge_n_dn6 * assign47550_e60928) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn6 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn7 - ((locals.var_q_edge_n_dn7 * assign47550_e60928) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn7 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn8 - ((locals.var_q_edge_n_dn8 * assign47550_e60928) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn8 / locals.var_q_edge_xgt0e)))), );
        }

        if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
            let assign47560_e60942: f64 = (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si);
            let assign47560_e60944: f64 = (assign47560_e60942 + 2.0);
            let assign47560_e60945: f64 = (assign47560_e60944).sqrt();
            let assign47560_e60946: f64 = (locals.var_q_edge_qi0si + assign47560_e60945);
            let assign47560_e60947: f64 = (0.5 * assign47560_e60946);
            (locals.var_q_edge_qi0, locals.var_q_edge_qi0_dn5, locals.var_q_edge_qi0_dn6, locals.var_q_edge_qi0_dn7, locals.var_q_edge_qi0_dn8, ) = (assign47560_e60947, (0.5 * (locals.var_q_edge_qi0si_dn5 + (((locals.var_q_edge_qi0si_dn5 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn5)) / (2.0 * assign47560_e60945)))), (0.5 * (locals.var_q_edge_qi0si_dn6 + (((locals.var_q_edge_qi0si_dn6 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn6)) / (2.0 * assign47560_e60945)))), (0.5 * (locals.var_q_edge_qi0si_dn7 + (((locals.var_q_edge_qi0si_dn7 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn7)) / (2.0 * assign47560_e60945)))), (0.5 * (locals.var_q_edge_qi0si_dn8 + (((locals.var_q_edge_qi0si_dn8 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn8)) / (2.0 * assign47560_e60945)))), );
        }

        let assign47570_e60952: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47570_e60954: f64 = if assign47570_e60952 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1251 = assign47570_e60954;

        if (((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) && (locals.var_guard1251 != 0.0)) {
            let assign47580_e60962: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
            let assign47580_e60963: f64 = (assign47580_e60962).exp();
            (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn5, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8, ) = (assign47580_e60963, (assign47580_e60963 * (locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5)), (assign47580_e60963 * (locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6)), (assign47580_e60963 * (locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7)), (assign47580_e60963 * (locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8)), );
        }

        if (((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) && (locals.var_guard1251 == 0.0)) {
            let assign47590_e60976: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
            let assign47590_e60978: f64 = (assign47590_e60976 - 230.25850929940458);
            let assign47590_e60983: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
            let assign47590_e60985: f64 = (assign47590_e60983 - 230.25850929940458);
            let assign47590_e60989: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
            let assign47590_e60991: f64 = (assign47590_e60989 - 230.25850929940458);
            let assign47590_e60993: f64 = (assign47590_e60991 * 0.3333333333333333);
            let assign47590_e60994: f64 = (1.0 + assign47590_e60993);
            let assign47590_e60995: f64 = (assign47590_e60985 * assign47590_e60994);
            let assign47590_e60996: f64 = (0.5 * assign47590_e60995);
            let assign47590_e60997: f64 = (1.0 + assign47590_e60996);
            let assign47590_e60998: f64 = (assign47590_e60978 * assign47590_e60997);
            let assign47590_e60999: f64 = (1.0 + assign47590_e60998);
            let assign47590_e61000: f64 = (1e100 * assign47590_e60999);
            (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn5, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8, ) = (assign47590_e61000, (1e100 * (((locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5) * assign47590_e60997) + (assign47590_e60978 * (0.5 * (((locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5) * assign47590_e60994) + (assign47590_e60985 * ((locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign47590_e60997) + (assign47590_e60978 * (0.5 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign47590_e60994) + (assign47590_e60985 * ((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign47590_e60997) + (assign47590_e60978 * (0.5 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign47590_e60994) + (assign47590_e60985 * ((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign47590_e60997) + (assign47590_e60978 * (0.5 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign47590_e60994) + (assign47590_e60985 * ((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * 0.3333333333333333))))))), );
        }

        if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
            let assign47600_e61008: f64 = (locals.var_gfedge2 * locals.var_q_edge_exp_x);
            (locals.var_q_edge_d0, locals.var_q_edge_d0_dn5, locals.var_q_edge_d0_dn6, locals.var_q_edge_d0_dn7, locals.var_q_edge_d0_dn8, ) = (assign47600_e61008, (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn5), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn6), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn7), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn8), );
        }

        if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
            let assign47610_e61016: f64 = (locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv);
            (locals.var_q_edge_d0p, locals.var_q_edge_d0p_dn5, locals.var_q_edge_d0p_dn6, locals.var_q_edge_d0p_dn7, locals.var_q_edge_d0p_dn8, ) = (assign47610_e61016, if locals.var_q_edge_n_inv_dn5 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn5)) } } else { (assign47610_e61016 * ((locals.var_q_edge_n_inv_dn5 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn5 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn6 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn6)) } } else { (assign47610_e61016 * ((locals.var_q_edge_n_inv_dn6 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn6 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn7 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn7)) } } else { (assign47610_e61016 * ((locals.var_q_edge_n_inv_dn7 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn7 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn8 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn8)) } } else { (assign47610_e61016 * ((locals.var_q_edge_n_inv_dn8 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn8 / locals.var_q_edge_d0)))) }, );
        }

        if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
            let assign47620_e61024: f64 = (locals.var_q_edge_n * locals.var_q_edge_n);
            let assign47620_e61028: f64 = (locals.var_q_edge_qi0 + locals.var_q_edge_n);
            let assign47620_e61029: f64 = (2.0 * assign47620_e61028);
            let assign47620_e61031: f64 = (assign47620_e61029 - locals.var_q_edge_d0p);
            let assign47620_e61033: f64 = (assign47620_e61031 * locals.var_q_edge_d0p);
            let assign47620_e61034: f64 = (assign47620_e61024 + assign47620_e61033);
            (locals.var_q_edge_sqerr, locals.var_q_edge_sqerr_dn5, locals.var_q_edge_sqerr_dn6, locals.var_q_edge_sqerr_dn7, locals.var_q_edge_sqerr_dn8, ) = (assign47620_e61034, (((locals.var_q_edge_n_dn5 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn5)) + ((((2.0 * (locals.var_q_edge_qi0_dn5 + locals.var_q_edge_n_dn5)) - locals.var_q_edge_d0p_dn5) * locals.var_q_edge_d0p) + (assign47620_e61031 * locals.var_q_edge_d0p_dn5))), (((locals.var_q_edge_n_dn6 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn6)) + ((((2.0 * (locals.var_q_edge_qi0_dn6 + locals.var_q_edge_n_dn6)) - locals.var_q_edge_d0p_dn6) * locals.var_q_edge_d0p) + (assign47620_e61031 * locals.var_q_edge_d0p_dn6))), (((locals.var_q_edge_n_dn7 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn7)) + ((((2.0 * (locals.var_q_edge_qi0_dn7 + locals.var_q_edge_n_dn7)) - locals.var_q_edge_d0p_dn7) * locals.var_q_edge_d0p) + (assign47620_e61031 * locals.var_q_edge_d0p_dn7))), (((locals.var_q_edge_n_dn8 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn8)) + ((((2.0 * (locals.var_q_edge_qi0_dn8 + locals.var_q_edge_n_dn8)) - locals.var_q_edge_d0p_dn8) * locals.var_q_edge_d0p) + (assign47620_e61031 * locals.var_q_edge_d0p_dn8))), );
        }

        if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
            let assign47630_e61042: f64 = (locals.var_q_edge_sqerr).sqrt();
            let assign47630_e61044: f64 = (assign47630_e61042 - locals.var_q_edge_n);
            let assign47630_e61046: f64 = (assign47630_e61044 / locals.var_q_edge_d0p);
            let assign47630_e61048: f64 = (assign47630_e61046 - 1.0);
            let assign47630_e61049: f64 = (locals.var_q_edge_n * assign47630_e61048);
            (locals.var_q_edge_errq, locals.var_q_edge_errq_dn5, locals.var_q_edge_errq_dn6, locals.var_q_edge_errq_dn7, locals.var_q_edge_errq_dn8, ) = (assign47630_e61049, ((locals.var_q_edge_n_dn5 * assign47630_e61048) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn5 / (2.0 * assign47630_e61042)) - locals.var_q_edge_n_dn5) * locals.var_q_edge_d0p) - (assign47630_e61044 * locals.var_q_edge_d0p_dn5)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn6 * assign47630_e61048) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn6 / (2.0 * assign47630_e61042)) - locals.var_q_edge_n_dn6) * locals.var_q_edge_d0p) - (assign47630_e61044 * locals.var_q_edge_d0p_dn6)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn7 * assign47630_e61048) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn7 / (2.0 * assign47630_e61042)) - locals.var_q_edge_n_dn7) * locals.var_q_edge_d0p) - (assign47630_e61044 * locals.var_q_edge_d0p_dn7)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn8 * assign47630_e61048) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn8 / (2.0 * assign47630_e61042)) - locals.var_q_edge_n_dn8) * locals.var_q_edge_d0p) - (assign47630_e61044 * locals.var_q_edge_d0p_dn8)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), );
        }

        if ((locals.var_guard1249 != 0.0) && (locals.var_guard1250 != 0.0)) {
            let assign47640_e61057: f64 = (locals.var_q_edge_qi0 - locals.var_q_edge_errq);
            (locals.var_qseffedge, locals.var_qseffedge_dn5, locals.var_qseffedge_dn6, locals.var_qseffedge_dn7, locals.var_qseffedge_dn8, ) = (assign47640_e61057, (locals.var_q_edge_qi0_dn5 - locals.var_q_edge_errq_dn5), (locals.var_q_edge_qi0_dn6 - locals.var_q_edge_errq_dn6), (locals.var_q_edge_qi0_dn7 - locals.var_q_edge_errq_dn7), (locals.var_q_edge_qi0_dn8 - locals.var_q_edge_errq_dn8), );
        }

        let assign47650_e61063: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47650_e61064: f64 = (locals.var_q_edge_n_inv * assign47650_e61063);
        let assign47650_e61066: f64 = (-230.25850929940458);
        let assign47650_e61067: f64 = if assign47650_e61064 > assign47650_e61066 { 1.0 } else { 0.0 };
        locals.var_guard1252 = assign47650_e61067;

        if (((locals.var_guard1249 != 0.0) && (locals.var_guard1250 == 0.0)) && (locals.var_guard1252 != 0.0)) {
            let assign47660_e61077: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
            let assign47660_e61078: f64 = (locals.var_q_edge_n_inv * assign47660_e61077);
            let assign47660_e61079: f64 = (assign47660_e61078).exp();
            (locals.var_qseffedge, locals.var_qseffedge_dn5, locals.var_qseffedge_dn6, locals.var_qseffedge_dn7, locals.var_qseffedge_dn8, ) = (assign47660_e61079, (assign47660_e61079 * ((locals.var_q_edge_n_inv_dn5 * assign47660_e61077) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))), (assign47660_e61079 * ((locals.var_q_edge_n_inv_dn6 * assign47660_e61077) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))), (assign47660_e61079 * ((locals.var_q_edge_n_inv_dn7 * assign47660_e61077) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))), (assign47660_e61079 * ((locals.var_q_edge_n_inv_dn8 * assign47660_e61077) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))), );
        }

        if (((locals.var_guard1249 != 0.0) && (locals.var_guard1250 == 0.0)) && (locals.var_guard1252 == 0.0)) {
            let assign47670_e61092: f64 = (-230.25850929940458);
            let assign47670_e61096: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
            let assign47670_e61097: f64 = (locals.var_q_edge_n_inv * assign47670_e61096);
            let assign47670_e61098: f64 = (assign47670_e61092 - assign47670_e61097);
            let assign47670_e61102: f64 = (-230.25850929940458);
            let assign47670_e61106: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
            let assign47670_e61107: f64 = (locals.var_q_edge_n_inv * assign47670_e61106);
            let assign47670_e61108: f64 = (assign47670_e61102 - assign47670_e61107);
            let assign47670_e61111: f64 = (-230.25850929940458);
            let assign47670_e61115: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
            let assign47670_e61116: f64 = (locals.var_q_edge_n_inv * assign47670_e61115);
            let assign47670_e61117: f64 = (assign47670_e61111 - assign47670_e61116);
            let assign47670_e61119: f64 = (assign47670_e61117 * 0.3333333333333333);
            let assign47670_e61120: f64 = (1.0 + assign47670_e61119);
            let assign47670_e61121: f64 = (assign47670_e61108 * assign47670_e61120);
            let assign47670_e61122: f64 = (0.5 * assign47670_e61121);
            let assign47670_e61123: f64 = (1.0 + assign47670_e61122);
            let assign47670_e61124: f64 = (assign47670_e61098 * assign47670_e61123);
            let assign47670_e61125: f64 = (1.0 + assign47670_e61124);
            let assign47670_e61126: f64 = (1e-100 / assign47670_e61125);
            (locals.var_qseffedge, locals.var_qseffedge_dn5, locals.var_qseffedge_dn6, locals.var_qseffedge_dn7, locals.var_qseffedge_dn8, ) = (assign47670_e61126, (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn5 * assign47670_e61096) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))) * assign47670_e61123) + (assign47670_e61098 * (0.5 * (((-((locals.var_q_edge_n_inv_dn5 * assign47670_e61106) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))) * assign47670_e61120) + (assign47670_e61108 * ((-((locals.var_q_edge_n_inv_dn5 * assign47670_e61115) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))) * 0.3333333333333333))))))) / (assign47670_e61125 * assign47670_e61125))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn6 * assign47670_e61096) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign47670_e61123) + (assign47670_e61098 * (0.5 * (((-((locals.var_q_edge_n_inv_dn6 * assign47670_e61106) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign47670_e61120) + (assign47670_e61108 * ((-((locals.var_q_edge_n_inv_dn6 * assign47670_e61115) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * 0.3333333333333333))))))) / (assign47670_e61125 * assign47670_e61125))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn7 * assign47670_e61096) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign47670_e61123) + (assign47670_e61098 * (0.5 * (((-((locals.var_q_edge_n_inv_dn7 * assign47670_e61106) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign47670_e61120) + (assign47670_e61108 * ((-((locals.var_q_edge_n_inv_dn7 * assign47670_e61115) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * 0.3333333333333333))))))) / (assign47670_e61125 * assign47670_e61125))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn8 * assign47670_e61096) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign47670_e61123) + (assign47670_e61098 * (0.5 * (((-((locals.var_q_edge_n_inv_dn8 * assign47670_e61106) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign47670_e61120) + (assign47670_e61108 * ((-((locals.var_q_edge_n_inv_dn8 * assign47670_e61115) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * 0.3333333333333333))))))) / (assign47670_e61125 * assign47670_e61125))), );
        }

        if (locals.var_guard1249 != 0.0) {
            let assign47680_e61133: f64 = (locals.var_vdse_dc + locals.var_vsbstaredge);
            let assign47680_e61134: f64 = (locals.var_inv_phit1edge * assign47680_e61133);
            (locals.var_xnedge_d, locals.var_xnedge_d_dn5, locals.var_xnedge_d_dn6, locals.var_xnedge_d_dn7, locals.var_xnedge_d_dn8, ) = (assign47680_e61134, ((locals.var_inv_phit1edge_dn5 * assign47680_e61133) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn5 + locals.var_vsbstaredge_dn5))), ((locals.var_inv_phit1edge_dn6 * assign47680_e61133) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn6 + locals.var_vsbstaredge_dn6))), ((locals.var_inv_phit1edge_dn7 * assign47680_e61133) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn7 + locals.var_vsbstaredge_dn7))), ((locals.var_inv_phit1edge_dn8 * assign47680_e61133) + (locals.var_inv_phit1edge * (locals.var_vdse_dc_dn8 + locals.var_vsbstaredge_dn8))), );
        }

        let assign47690_e61143: f64 = if ((locals.var_qseffedge < 0.001) && (locals.var_vdse_dc < 1e-6)) { 1.0 } else { 0.0 };
        locals.var_guard1253 = assign47690_e61143;

        let assign47700_e61145: f64 = (-locals.var_xnedge_d);
        let assign47700_e61147: f64 = (assign47700_e61145 + locals.var_xnedge_s);
        let assign47700_e61149: f64 = (-230.25850929940458);
        let assign47700_e61150: f64 = if assign47700_e61147 > assign47700_e61149 { 1.0 } else { 0.0 };
        locals.var_guard1254 = assign47700_e61150;

        if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 != 0.0)) && (locals.var_guard1254 != 0.0)) {
            let assign47710_e61157: f64 = (-locals.var_xnedge_d);
            let assign47710_e61159: f64 = (assign47710_e61157 + locals.var_xnedge_s);
            let assign47710_e61160: f64 = (assign47710_e61159).exp();
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign47710_e61160, (assign47710_e61160 * ((-locals.var_xnedge_d_dn5) + locals.var_xnedge_s_dn5)), (assign47710_e61160 * ((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)), (assign47710_e61160 * ((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)), (assign47710_e61160 * ((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)), );
        }

        if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 != 0.0)) && (locals.var_guard1254 == 0.0)) {
            let assign47720_e61172: f64 = (-230.25850929940458);
            let assign47720_e61174: f64 = (-locals.var_xnedge_d);
            let assign47720_e61176: f64 = (assign47720_e61174 + locals.var_xnedge_s);
            let assign47720_e61177: f64 = (assign47720_e61172 - assign47720_e61176);
            let assign47720_e61181: f64 = (-230.25850929940458);
            let assign47720_e61183: f64 = (-locals.var_xnedge_d);
            let assign47720_e61185: f64 = (assign47720_e61183 + locals.var_xnedge_s);
            let assign47720_e61186: f64 = (assign47720_e61181 - assign47720_e61185);
            let assign47720_e61189: f64 = (-230.25850929940458);
            let assign47720_e61191: f64 = (-locals.var_xnedge_d);
            let assign47720_e61193: f64 = (assign47720_e61191 + locals.var_xnedge_s);
            let assign47720_e61194: f64 = (assign47720_e61189 - assign47720_e61193);
            let assign47720_e61196: f64 = (assign47720_e61194 * 0.3333333333333333);
            let assign47720_e61197: f64 = (1.0 + assign47720_e61196);
            let assign47720_e61198: f64 = (assign47720_e61186 * assign47720_e61197);
            let assign47720_e61199: f64 = (0.5 * assign47720_e61198);
            let assign47720_e61200: f64 = (1.0 + assign47720_e61199);
            let assign47720_e61201: f64 = (assign47720_e61177 * assign47720_e61200);
            let assign47720_e61202: f64 = (1.0 + assign47720_e61201);
            let assign47720_e61203: f64 = (1e-100 / assign47720_e61202);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign47720_e61203, (-((1e-100 * (((-((-locals.var_xnedge_d_dn5) + locals.var_xnedge_s_dn5)) * assign47720_e61200) + (assign47720_e61177 * (0.5 * (((-((-locals.var_xnedge_d_dn5) + locals.var_xnedge_s_dn5)) * assign47720_e61197) + (assign47720_e61186 * ((-((-locals.var_xnedge_d_dn5) + locals.var_xnedge_s_dn5)) * 0.3333333333333333))))))) / (assign47720_e61202 * assign47720_e61202))), (-((1e-100 * (((-((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)) * assign47720_e61200) + (assign47720_e61177 * (0.5 * (((-((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)) * assign47720_e61197) + (assign47720_e61186 * ((-((-locals.var_xnedge_d_dn6) + locals.var_xnedge_s_dn6)) * 0.3333333333333333))))))) / (assign47720_e61202 * assign47720_e61202))), (-((1e-100 * (((-((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)) * assign47720_e61200) + (assign47720_e61177 * (0.5 * (((-((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)) * assign47720_e61197) + (assign47720_e61186 * ((-((-locals.var_xnedge_d_dn7) + locals.var_xnedge_s_dn7)) * 0.3333333333333333))))))) / (assign47720_e61202 * assign47720_e61202))), (-((1e-100 * (((-((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)) * assign47720_e61200) + (assign47720_e61177 * (0.5 * (((-((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)) * assign47720_e61197) + (assign47720_e61186 * ((-((-locals.var_xnedge_d_dn8) + locals.var_xnedge_s_dn8)) * 0.3333333333333333))))))) / (assign47720_e61202 * assign47720_e61202))), );
        }

        if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 != 0.0)) {
            let assign47730_e61212: f64 = (locals.var_temp__blk936 - 1.0);
            let assign47730_e61213: f64 = (locals.var_qseffedge * assign47730_e61212);
            (locals.var_qdseffedge, locals.var_qdseffedge_dn5, locals.var_qdseffedge_dn6, locals.var_qdseffedge_dn7, locals.var_qdseffedge_dn8, ) = (assign47730_e61213, ((locals.var_qseffedge_dn5 * assign47730_e61212) + (locals.var_qseffedge * locals.var_temp__blk936_dn5)), ((locals.var_qseffedge_dn6 * assign47730_e61212) + (locals.var_qseffedge * locals.var_temp__blk936_dn6)), ((locals.var_qseffedge_dn7 * assign47730_e61212) + (locals.var_qseffedge * locals.var_temp__blk936_dn7)), ((locals.var_qseffedge_dn8 * assign47730_e61212) + (locals.var_qseffedge * locals.var_temp__blk936_dn8)), );
        }

        if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 != 0.0)) {
            let assign47740_e61221: f64 = (locals.var_qdseffedge + locals.var_qseffedge);
            (locals.var_qdeffedge, locals.var_qdeffedge_dn5, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8, ) = (assign47740_e61221, (locals.var_qdseffedge_dn5 + locals.var_qseffedge_dn5), (locals.var_qdseffedge_dn6 + locals.var_qseffedge_dn6), (locals.var_qdseffedge_dn7 + locals.var_qseffedge_dn7), (locals.var_qdseffedge_dn8 + locals.var_qseffedge_dn8), );
        }

        if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) {
            let assign47750_e61230: f64 = (locals.var_xbedge + locals.var_xnedge_d);
            (locals.var_q_edge_xsth, locals.var_q_edge_xsth_dn5, locals.var_q_edge_xsth_dn6, locals.var_q_edge_xsth_dn7, locals.var_q_edge_xsth_dn8, ) = (assign47750_e61230, (locals.var_xbedge_dn5 + locals.var_xnedge_d_dn5), (locals.var_xbedge_dn6 + locals.var_xnedge_d_dn6), (locals.var_xbedge_dn7 + locals.var_xnedge_d_dn7), (locals.var_xbedge_dn8 + locals.var_xnedge_d_dn8), );
        }

        if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) {
            let assign47760_e61240: f64 = (locals.var_q_edge_xsth).sqrt();
            let assign47760_e61241: f64 = (locals.var_gfedge * assign47760_e61240);
            let assign47760_e61242: f64 = (locals.var_q_edge_xsth + assign47760_e61241);
            (locals.var_q_edge_xth0, locals.var_q_edge_xth0_dn5, locals.var_q_edge_xth0_dn6, locals.var_q_edge_xth0_dn7, locals.var_q_edge_xth0_dn8, ) = (assign47760_e61242, (locals.var_q_edge_xsth_dn5 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn5 / (2.0 * assign47760_e61240)))), (locals.var_q_edge_xsth_dn6 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47760_e61240)))), (locals.var_q_edge_xsth_dn7 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47760_e61240)))), (locals.var_q_edge_xsth_dn8 + (locals.var_gfedge * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47760_e61240)))), );
        }

        if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) {
            let assign47770_e61251: f64 = (locals.var_q_edge_xth0 + locals.var_dxthedge);
            (locals.var_q_edge_xth, locals.var_q_edge_xth_dn5, locals.var_q_edge_xth_dn6, locals.var_q_edge_xth_dn7, locals.var_q_edge_xth_dn8, ) = (assign47770_e61251, (locals.var_q_edge_xth0_dn5 + locals.var_dxthedge_dn5), (locals.var_q_edge_xth0_dn6 + locals.var_dxthedge_dn6), (locals.var_q_edge_xth0_dn7 + locals.var_dxthedge_dn7), (locals.var_q_edge_xth0_dn8 + locals.var_dxthedge_dn8), );
        }

        if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) {
            let assign47780_e61262: f64 = (locals.var_q_edge_xsth).sqrt();
            let assign47780_e61263: f64 = (2.0 * assign47780_e61262);
            let assign47780_e61264: f64 = (locals.var_gfedge / assign47780_e61263);
            let assign47780_e61265: f64 = (1.0 + assign47780_e61264);
            (locals.var_q_edge_n, locals.var_q_edge_n_dn5, locals.var_q_edge_n_dn6, locals.var_q_edge_n_dn7, locals.var_q_edge_n_dn8, ) = (assign47780_e61265, (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn5 / (2.0 * assign47780_e61262)))) / (assign47780_e61263 * assign47780_e61263))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn6 / (2.0 * assign47780_e61262)))) / (assign47780_e61263 * assign47780_e61263))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn7 / (2.0 * assign47780_e61262)))) / (assign47780_e61263 * assign47780_e61263))), (-((locals.var_gfedge * (2.0 * (locals.var_q_edge_xsth_dn8 / (2.0 * assign47780_e61262)))) / (assign47780_e61263 * assign47780_e61263))), );
        }

        if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) {
            let assign47790_e61274: f64 = (1.0 / locals.var_q_edge_n);
            (locals.var_q_edge_n_inv, locals.var_q_edge_n_inv_dn5, locals.var_q_edge_n_inv_dn6, locals.var_q_edge_n_inv_dn7, locals.var_q_edge_n_inv_dn8, ) = (assign47790_e61274, (-(locals.var_q_edge_n_dn5 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn6 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn7 / (locals.var_q_edge_n * locals.var_q_edge_n))), (-(locals.var_q_edge_n_dn8 / (locals.var_q_edge_n * locals.var_q_edge_n))), );
        }

        if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) {
            let assign47800_e61283: f64 = (locals.var_xgedge - locals.var_q_edge_xth);
            (locals.var_q_edge_xgt, locals.var_q_edge_xgt_dn5, locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8, ) = (assign47800_e61283, (locals.var_xgedge_dn5 - locals.var_q_edge_xth_dn5), (locals.var_xgedge_dn6 - locals.var_q_edge_xth_dn6), (locals.var_xgedge_dn7 - locals.var_q_edge_xth_dn7), (locals.var_xgedge_dn8 - locals.var_q_edge_xth_dn8), );
        }

        let assign47810_e61288: f64 = (-12.0);
        let assign47810_e61289: f64 = if locals.var_q_edge_xgt > assign47810_e61288 { 1.0 } else { 0.0 };
        locals.var_guard1255 = assign47810_e61289;

        if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
            let assign47820_e61298: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
            let assign47820_e61300: f64 = (assign47820_e61298 - 1.0);
            (locals.var_q_edge_xgt0, locals.var_q_edge_xgt0_dn5, locals.var_q_edge_xgt0_dn6, locals.var_q_edge_xgt0_dn7, locals.var_q_edge_xgt0_dn8, ) = (assign47820_e61300, locals.var_q_edge_xgt_dn5, locals.var_q_edge_xgt_dn6, locals.var_q_edge_xgt_dn7, locals.var_q_edge_xgt_dn8, );
        }

        if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
            let assign47830_e61313: f64 = (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0);
            let assign47830_e61315: f64 = (assign47830_e61313 + 10.0);
            let assign47830_e61316: f64 = (assign47830_e61315).sqrt();
            let assign47830_e61317: f64 = (locals.var_q_edge_xgt0 + assign47830_e61316);
            let assign47830_e61318: f64 = (0.5 * assign47830_e61317);
            (locals.var_q_edge_xgt0e, locals.var_q_edge_xgt0e_dn5, locals.var_q_edge_xgt0e_dn6, locals.var_q_edge_xgt0e_dn7, locals.var_q_edge_xgt0e_dn8, ) = (assign47830_e61318, (0.5 * (locals.var_q_edge_xgt0_dn5 + (((locals.var_q_edge_xgt0_dn5 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn5)) / (2.0 * assign47830_e61316)))), (0.5 * (locals.var_q_edge_xgt0_dn6 + (((locals.var_q_edge_xgt0_dn6 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn6)) / (2.0 * assign47830_e61316)))), (0.5 * (locals.var_q_edge_xgt0_dn7 + (((locals.var_q_edge_xgt0_dn7 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn7)) / (2.0 * assign47830_e61316)))), (0.5 * (locals.var_q_edge_xgt0_dn8 + (((locals.var_q_edge_xgt0_dn8 * locals.var_q_edge_xgt0) + (locals.var_q_edge_xgt0 * locals.var_q_edge_xgt0_dn8)) / (2.0 * assign47830_e61316)))), );
        }

        if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
            let assign47840_e61330: f64 = (locals.var_q_edge_xgt0e).ln();
            let assign47840_e61331: f64 = (locals.var_q_edge_n * assign47840_e61330);
            let assign47840_e61332: f64 = (locals.var_q_edge_xgt - assign47840_e61331);
            let assign47840_e61334: f64 = (assign47840_e61332 + locals.var_lngfedge2);
            (locals.var_q_edge_qi0si, locals.var_q_edge_qi0si_dn5, locals.var_q_edge_qi0si_dn6, locals.var_q_edge_qi0si_dn7, locals.var_q_edge_qi0si_dn8, ) = (assign47840_e61334, (locals.var_q_edge_xgt_dn5 - ((locals.var_q_edge_n_dn5 * assign47840_e61330) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn5 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn6 - ((locals.var_q_edge_n_dn6 * assign47840_e61330) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn6 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn7 - ((locals.var_q_edge_n_dn7 * assign47840_e61330) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn7 / locals.var_q_edge_xgt0e)))), (locals.var_q_edge_xgt_dn8 - ((locals.var_q_edge_n_dn8 * assign47840_e61330) + (locals.var_q_edge_n * (locals.var_q_edge_xgt0e_dn8 / locals.var_q_edge_xgt0e)))), );
        }

        if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
            let assign47850_e61347: f64 = (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si);
            let assign47850_e61349: f64 = (assign47850_e61347 + 2.0);
            let assign47850_e61350: f64 = (assign47850_e61349).sqrt();
            let assign47850_e61351: f64 = (locals.var_q_edge_qi0si + assign47850_e61350);
            let assign47850_e61352: f64 = (0.5 * assign47850_e61351);
            (locals.var_q_edge_qi0, locals.var_q_edge_qi0_dn5, locals.var_q_edge_qi0_dn6, locals.var_q_edge_qi0_dn7, locals.var_q_edge_qi0_dn8, ) = (assign47850_e61352, (0.5 * (locals.var_q_edge_qi0si_dn5 + (((locals.var_q_edge_qi0si_dn5 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn5)) / (2.0 * assign47850_e61350)))), (0.5 * (locals.var_q_edge_qi0si_dn6 + (((locals.var_q_edge_qi0si_dn6 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn6)) / (2.0 * assign47850_e61350)))), (0.5 * (locals.var_q_edge_qi0si_dn7 + (((locals.var_q_edge_qi0si_dn7 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn7)) / (2.0 * assign47850_e61350)))), (0.5 * (locals.var_q_edge_qi0si_dn8 + (((locals.var_q_edge_qi0si_dn8 * locals.var_q_edge_qi0si) + (locals.var_q_edge_qi0si * locals.var_q_edge_qi0si_dn8)) / (2.0 * assign47850_e61350)))), );
        }

        let assign47860_e61357: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
        let assign47860_e61359: f64 = if assign47860_e61357 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1256 = assign47860_e61359;

        if ((((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) && (locals.var_guard1256 != 0.0)) {
            let assign47870_e61370: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
            let assign47870_e61371: f64 = (assign47870_e61370).exp();
            (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn5, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8, ) = (assign47870_e61371, (assign47870_e61371 * (locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5)), (assign47870_e61371 * (locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6)), (assign47870_e61371 * (locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7)), (assign47870_e61371 * (locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8)), );
        }

        if ((((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) && (locals.var_guard1256 == 0.0)) {
            let assign47880_e61387: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
            let assign47880_e61389: f64 = (assign47880_e61387 - 230.25850929940458);
            let assign47880_e61394: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
            let assign47880_e61396: f64 = (assign47880_e61394 - 230.25850929940458);
            let assign47880_e61400: f64 = (locals.var_q_edge_xgt - locals.var_q_edge_qi0);
            let assign47880_e61402: f64 = (assign47880_e61400 - 230.25850929940458);
            let assign47880_e61404: f64 = (assign47880_e61402 * 0.3333333333333333);
            let assign47880_e61405: f64 = (1.0 + assign47880_e61404);
            let assign47880_e61406: f64 = (assign47880_e61396 * assign47880_e61405);
            let assign47880_e61407: f64 = (0.5 * assign47880_e61406);
            let assign47880_e61408: f64 = (1.0 + assign47880_e61407);
            let assign47880_e61409: f64 = (assign47880_e61389 * assign47880_e61408);
            let assign47880_e61410: f64 = (1.0 + assign47880_e61409);
            let assign47880_e61411: f64 = (1e100 * assign47880_e61410);
            (locals.var_q_edge_exp_x, locals.var_q_edge_exp_x_dn5, locals.var_q_edge_exp_x_dn6, locals.var_q_edge_exp_x_dn7, locals.var_q_edge_exp_x_dn8, ) = (assign47880_e61411, (1e100 * (((locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5) * assign47880_e61408) + (assign47880_e61389 * (0.5 * (((locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5) * assign47880_e61405) + (assign47880_e61396 * ((locals.var_q_edge_xgt_dn5 - locals.var_q_edge_qi0_dn5) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign47880_e61408) + (assign47880_e61389 * (0.5 * (((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * assign47880_e61405) + (assign47880_e61396 * ((locals.var_q_edge_xgt_dn6 - locals.var_q_edge_qi0_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign47880_e61408) + (assign47880_e61389 * (0.5 * (((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * assign47880_e61405) + (assign47880_e61396 * ((locals.var_q_edge_xgt_dn7 - locals.var_q_edge_qi0_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign47880_e61408) + (assign47880_e61389 * (0.5 * (((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * assign47880_e61405) + (assign47880_e61396 * ((locals.var_q_edge_xgt_dn8 - locals.var_q_edge_qi0_dn8) * 0.3333333333333333))))))), );
        }

        if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
            let assign47890_e61422: f64 = (locals.var_gfedge2 * locals.var_q_edge_exp_x);
            (locals.var_q_edge_d0, locals.var_q_edge_d0_dn5, locals.var_q_edge_d0_dn6, locals.var_q_edge_d0_dn7, locals.var_q_edge_d0_dn8, ) = (assign47890_e61422, (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn5), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn6), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn7), (locals.var_gfedge2 * locals.var_q_edge_exp_x_dn8), );
        }

        if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
            let assign47900_e61433: f64 = (locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv);
            (locals.var_q_edge_d0p, locals.var_q_edge_d0p_dn5, locals.var_q_edge_d0p_dn6, locals.var_q_edge_d0p_dn7, locals.var_q_edge_d0p_dn8, ) = (assign47900_e61433, if locals.var_q_edge_n_inv_dn5 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn5)) } } else { (assign47900_e61433 * ((locals.var_q_edge_n_inv_dn5 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn5 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn6 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn6)) } } else { (assign47900_e61433 * ((locals.var_q_edge_n_inv_dn6 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn6 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn7 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn7)) } } else { (assign47900_e61433 * ((locals.var_q_edge_n_inv_dn7 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn7 / locals.var_q_edge_d0)))) }, if locals.var_q_edge_n_inv_dn8 == 0.0 && ((locals.var_q_edge_n_inv) as f64).is_finite() && ((locals.var_q_edge_n_inv) as f64).fract() == 0.0 { if locals.var_q_edge_n_inv == 0.0 { 0.0 } else { (locals.var_q_edge_n_inv * ((locals.var_q_edge_d0).powf(locals.var_q_edge_n_inv - 1.0) * locals.var_q_edge_d0_dn8)) } } else { (assign47900_e61433 * ((locals.var_q_edge_n_inv_dn8 * (locals.var_q_edge_d0).ln()) + (locals.var_q_edge_n_inv * (locals.var_q_edge_d0_dn8 / locals.var_q_edge_d0)))) }, );
        }

        if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
            let assign47910_e61444: f64 = (locals.var_q_edge_n * locals.var_q_edge_n);
            let assign47910_e61448: f64 = (locals.var_q_edge_qi0 + locals.var_q_edge_n);
            let assign47910_e61449: f64 = (2.0 * assign47910_e61448);
            let assign47910_e61451: f64 = (assign47910_e61449 - locals.var_q_edge_d0p);
            let assign47910_e61453: f64 = (assign47910_e61451 * locals.var_q_edge_d0p);
            let assign47910_e61454: f64 = (assign47910_e61444 + assign47910_e61453);
            (locals.var_q_edge_sqerr, locals.var_q_edge_sqerr_dn5, locals.var_q_edge_sqerr_dn6, locals.var_q_edge_sqerr_dn7, locals.var_q_edge_sqerr_dn8, ) = (assign47910_e61454, (((locals.var_q_edge_n_dn5 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn5)) + ((((2.0 * (locals.var_q_edge_qi0_dn5 + locals.var_q_edge_n_dn5)) - locals.var_q_edge_d0p_dn5) * locals.var_q_edge_d0p) + (assign47910_e61451 * locals.var_q_edge_d0p_dn5))), (((locals.var_q_edge_n_dn6 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn6)) + ((((2.0 * (locals.var_q_edge_qi0_dn6 + locals.var_q_edge_n_dn6)) - locals.var_q_edge_d0p_dn6) * locals.var_q_edge_d0p) + (assign47910_e61451 * locals.var_q_edge_d0p_dn6))), (((locals.var_q_edge_n_dn7 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn7)) + ((((2.0 * (locals.var_q_edge_qi0_dn7 + locals.var_q_edge_n_dn7)) - locals.var_q_edge_d0p_dn7) * locals.var_q_edge_d0p) + (assign47910_e61451 * locals.var_q_edge_d0p_dn7))), (((locals.var_q_edge_n_dn8 * locals.var_q_edge_n) + (locals.var_q_edge_n * locals.var_q_edge_n_dn8)) + ((((2.0 * (locals.var_q_edge_qi0_dn8 + locals.var_q_edge_n_dn8)) - locals.var_q_edge_d0p_dn8) * locals.var_q_edge_d0p) + (assign47910_e61451 * locals.var_q_edge_d0p_dn8))), );
        }

        if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
            let assign47920_e61465: f64 = (locals.var_q_edge_sqerr).sqrt();
            let assign47920_e61467: f64 = (assign47920_e61465 - locals.var_q_edge_n);
            let assign47920_e61469: f64 = (assign47920_e61467 / locals.var_q_edge_d0p);
            let assign47920_e61471: f64 = (assign47920_e61469 - 1.0);
            let assign47920_e61472: f64 = (locals.var_q_edge_n * assign47920_e61471);
            (locals.var_q_edge_errq, locals.var_q_edge_errq_dn5, locals.var_q_edge_errq_dn6, locals.var_q_edge_errq_dn7, locals.var_q_edge_errq_dn8, ) = (assign47920_e61472, ((locals.var_q_edge_n_dn5 * assign47920_e61471) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn5 / (2.0 * assign47920_e61465)) - locals.var_q_edge_n_dn5) * locals.var_q_edge_d0p) - (assign47920_e61467 * locals.var_q_edge_d0p_dn5)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn6 * assign47920_e61471) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn6 / (2.0 * assign47920_e61465)) - locals.var_q_edge_n_dn6) * locals.var_q_edge_d0p) - (assign47920_e61467 * locals.var_q_edge_d0p_dn6)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn7 * assign47920_e61471) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn7 / (2.0 * assign47920_e61465)) - locals.var_q_edge_n_dn7) * locals.var_q_edge_d0p) - (assign47920_e61467 * locals.var_q_edge_d0p_dn7)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), ((locals.var_q_edge_n_dn8 * assign47920_e61471) + (locals.var_q_edge_n * (((((locals.var_q_edge_sqerr_dn8 / (2.0 * assign47920_e61465)) - locals.var_q_edge_n_dn8) * locals.var_q_edge_d0p) - (assign47920_e61467 * locals.var_q_edge_d0p_dn8)) / (locals.var_q_edge_d0p * locals.var_q_edge_d0p)))), );
        }

        if (((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 != 0.0)) {
            let assign47930_e61483: f64 = (locals.var_q_edge_qi0 - locals.var_q_edge_errq);
            (locals.var_qdeffedge, locals.var_qdeffedge_dn5, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8, ) = (assign47930_e61483, (locals.var_q_edge_qi0_dn5 - locals.var_q_edge_errq_dn5), (locals.var_q_edge_qi0_dn6 - locals.var_q_edge_errq_dn6), (locals.var_q_edge_qi0_dn7 - locals.var_q_edge_errq_dn7), (locals.var_q_edge_qi0_dn8 - locals.var_q_edge_errq_dn8), );
        }

        let assign47940_e61489: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
        let assign47940_e61490: f64 = (locals.var_q_edge_n_inv * assign47940_e61489);
        let assign47940_e61492: f64 = (-230.25850929940458);
        let assign47940_e61493: f64 = if assign47940_e61490 > assign47940_e61492 { 1.0 } else { 0.0 };
        locals.var_guard1257 = assign47940_e61493;

        if ((((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 == 0.0)) && (locals.var_guard1257 != 0.0)) {
            let assign47950_e61506: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
            let assign47950_e61507: f64 = (locals.var_q_edge_n_inv * assign47950_e61506);
            let assign47950_e61508: f64 = (assign47950_e61507).exp();
            (locals.var_qdeffedge, locals.var_qdeffedge_dn5, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8, ) = (assign47950_e61508, (assign47950_e61508 * ((locals.var_q_edge_n_inv_dn5 * assign47950_e61506) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))), (assign47950_e61508 * ((locals.var_q_edge_n_inv_dn6 * assign47950_e61506) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))), (assign47950_e61508 * ((locals.var_q_edge_n_inv_dn7 * assign47950_e61506) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))), (assign47950_e61508 * ((locals.var_q_edge_n_inv_dn8 * assign47950_e61506) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))), );
        }

        if ((((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1255 == 0.0)) && (locals.var_guard1257 == 0.0)) {
            let assign47960_e61524: f64 = (-230.25850929940458);
            let assign47960_e61528: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
            let assign47960_e61529: f64 = (locals.var_q_edge_n_inv * assign47960_e61528);
            let assign47960_e61530: f64 = (assign47960_e61524 - assign47960_e61529);
            let assign47960_e61534: f64 = (-230.25850929940458);
            let assign47960_e61538: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
            let assign47960_e61539: f64 = (locals.var_q_edge_n_inv * assign47960_e61538);
            let assign47960_e61540: f64 = (assign47960_e61534 - assign47960_e61539);
            let assign47960_e61543: f64 = (-230.25850929940458);
            let assign47960_e61547: f64 = (locals.var_q_edge_xgt + locals.var_lngfedge2);
            let assign47960_e61548: f64 = (locals.var_q_edge_n_inv * assign47960_e61547);
            let assign47960_e61549: f64 = (assign47960_e61543 - assign47960_e61548);
            let assign47960_e61551: f64 = (assign47960_e61549 * 0.3333333333333333);
            let assign47960_e61552: f64 = (1.0 + assign47960_e61551);
            let assign47960_e61553: f64 = (assign47960_e61540 * assign47960_e61552);
            let assign47960_e61554: f64 = (0.5 * assign47960_e61553);
            let assign47960_e61555: f64 = (1.0 + assign47960_e61554);
            let assign47960_e61556: f64 = (assign47960_e61530 * assign47960_e61555);
            let assign47960_e61557: f64 = (1.0 + assign47960_e61556);
            let assign47960_e61558: f64 = (1e-100 / assign47960_e61557);
            (locals.var_qdeffedge, locals.var_qdeffedge_dn5, locals.var_qdeffedge_dn6, locals.var_qdeffedge_dn7, locals.var_qdeffedge_dn8, ) = (assign47960_e61558, (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn5 * assign47960_e61528) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))) * assign47960_e61555) + (assign47960_e61530 * (0.5 * (((-((locals.var_q_edge_n_inv_dn5 * assign47960_e61538) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))) * assign47960_e61552) + (assign47960_e61540 * ((-((locals.var_q_edge_n_inv_dn5 * assign47960_e61547) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn5))) * 0.3333333333333333))))))) / (assign47960_e61557 * assign47960_e61557))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn6 * assign47960_e61528) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign47960_e61555) + (assign47960_e61530 * (0.5 * (((-((locals.var_q_edge_n_inv_dn6 * assign47960_e61538) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * assign47960_e61552) + (assign47960_e61540 * ((-((locals.var_q_edge_n_inv_dn6 * assign47960_e61547) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn6))) * 0.3333333333333333))))))) / (assign47960_e61557 * assign47960_e61557))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn7 * assign47960_e61528) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign47960_e61555) + (assign47960_e61530 * (0.5 * (((-((locals.var_q_edge_n_inv_dn7 * assign47960_e61538) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * assign47960_e61552) + (assign47960_e61540 * ((-((locals.var_q_edge_n_inv_dn7 * assign47960_e61547) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn7))) * 0.3333333333333333))))))) / (assign47960_e61557 * assign47960_e61557))), (-((1e-100 * (((-((locals.var_q_edge_n_inv_dn8 * assign47960_e61528) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign47960_e61555) + (assign47960_e61530 * (0.5 * (((-((locals.var_q_edge_n_inv_dn8 * assign47960_e61538) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * assign47960_e61552) + (assign47960_e61540 * ((-((locals.var_q_edge_n_inv_dn8 * assign47960_e61547) + (locals.var_q_edge_n_inv * locals.var_q_edge_xgt_dn8))) * 0.3333333333333333))))))) / (assign47960_e61557 * assign47960_e61557))), );
        }

        if ((locals.var_guard1249 != 0.0) && (locals.var_guard1253 == 0.0)) {
            let assign47970_e61567: f64 = (locals.var_qdeffedge - locals.var_qseffedge);
            (locals.var_qdseffedge, locals.var_qdseffedge_dn5, locals.var_qdseffedge_dn6, locals.var_qdseffedge_dn7, locals.var_qdseffedge_dn8, ) = (assign47970_e61567, (locals.var_qdeffedge_dn5 - locals.var_qseffedge_dn5), (locals.var_qdeffedge_dn6 - locals.var_qseffedge_dn6), (locals.var_qdeffedge_dn7 - locals.var_qseffedge_dn7), (locals.var_qdeffedge_dn8 - locals.var_qseffedge_dn8), );
        }

        if (locals.var_guard1249 != 0.0) {
            let assign47980_e61574: f64 = (locals.var_qdeffedge + locals.var_qseffedge);
            let assign47980_e61575: f64 = (0.5 * assign47980_e61574);
            (locals.var_qmeffedge, locals.var_qmeffedge_dn5, locals.var_qmeffedge_dn6, locals.var_qmeffedge_dn7, locals.var_qmeffedge_dn8, ) = (assign47980_e61575, (0.5 * (locals.var_qdeffedge_dn5 + locals.var_qseffedge_dn5)), (0.5 * (locals.var_qdeffedge_dn6 + locals.var_qseffedge_dn6)), (0.5 * (locals.var_qdeffedge_dn7 + locals.var_qseffedge_dn7)), (0.5 * (locals.var_qdeffedge_dn8 + locals.var_qseffedge_dn8)), );
        }

    }

    pub(super) fn stamp_transient_block_19(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard1249 != 0.0) {
            let assign47990_e61581: f64 = (locals.var_xgedge - locals.var_qmeffedge);
            let (assign47990_e61588, assign47990_e61588_d_n5, assign47990_e61588_d_n6, assign47990_e61588_d_n7, assign47990_e61588_d_n8,) = {
    if (assign47990_e61581 > 1e-40) {
        let assign47990_e61586: f64 = (locals.var_xgedge - locals.var_qmeffedge);
        (assign47990_e61586, (locals.var_xgedge_dn5 - locals.var_qmeffedge_dn5), (locals.var_xgedge_dn6 - locals.var_qmeffedge_dn6), (locals.var_xgedge_dn7 - locals.var_qmeffedge_dn7), (locals.var_xgedge_dn8 - locals.var_qmeffedge_dn8),)
    } else {
        (1e-40, 0.0, 0.0, 0.0, 0.0,)
    }
};
            (locals.var_dsqredge, locals.var_dsqredge_dn5, locals.var_dsqredge_dn6, locals.var_dsqredge_dn7, locals.var_dsqredge_dn8, ) = (assign47990_e61588, assign47990_e61588_d_n5, assign47990_e61588_d_n6, assign47990_e61588_d_n7, assign47990_e61588_d_n8, );
        }

        if (locals.var_guard1249 != 0.0) {
            let assign48000_e61595: f64 = (0.5 * locals.var_gfedge);
            let assign48000_e61599: f64 = (0.25 * locals.var_gfedge2);
            let assign48000_e61600: f64 = (locals.var_dsqredge + assign48000_e61599);
            let assign48000_e61601: f64 = (assign48000_e61600).sqrt();
            let assign48000_e61602: f64 = (assign48000_e61595 / assign48000_e61601);
            let assign48000_e61603: f64 = (1.0 - assign48000_e61602);
            (locals.var_alphabmedge, locals.var_alphabmedge_dn5, locals.var_alphabmedge_dn6, locals.var_alphabmedge_dn7, locals.var_alphabmedge_dn8, ) = (assign48000_e61603, (-(-((assign48000_e61595 * (locals.var_dsqredge_dn5 / (2.0 * assign48000_e61601))) / (assign48000_e61601 * assign48000_e61601)))), (-(-((assign48000_e61595 * (locals.var_dsqredge_dn6 / (2.0 * assign48000_e61601))) / (assign48000_e61601 * assign48000_e61601)))), (-(-((assign48000_e61595 * (locals.var_dsqredge_dn7 / (2.0 * assign48000_e61601))) / (assign48000_e61601 * assign48000_e61601)))), (-(-((assign48000_e61595 * (locals.var_dsqredge_dn8 / (2.0 * assign48000_e61601))) / (assign48000_e61601 * assign48000_e61601)))), );
        }

        if (locals.var_guard1249 != 0.0) {
            let assign48010_e61608: f64 = (-locals.var_betedge_i);
            let assign48010_e61610: f64 = (assign48010_e61608 * locals.var_phit1edge);
            let assign48010_e61612: f64 = (assign48010_e61610 * locals.var_phit1edge);
            let assign48010_e61615: f64 = (locals.var_alphabmedge * locals.var_qmeffedge);
            let assign48010_e61617: f64 = (assign48010_e61615 + 1.0);
            let assign48010_e61618: f64 = (assign48010_e61612 * assign48010_e61617);
            let assign48010_e61620: f64 = (assign48010_e61618 * locals.var_qdseffedge);
            let assign48010_e61622: f64 = (assign48010_e61620 / locals.var_gmob_dc);
            (locals.var_i_dsedge, locals.var_i_dsedge_dn5, locals.var_i_dsedge_dn6, locals.var_i_dsedge_dn7, locals.var_i_dsedge_dn8, ) = (assign48010_e61622, ((((((((((assign48010_e61608 * locals.var_phit1edge_dn5) * locals.var_phit1edge) + (assign48010_e61610 * locals.var_phit1edge_dn5)) * assign48010_e61617) + (assign48010_e61612 * ((locals.var_alphabmedge_dn5 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn5)))) * locals.var_qdseffedge) + (assign48010_e61618 * locals.var_qdseffedge_dn5)) * locals.var_gmob_dc) - (assign48010_e61620 * locals.var_gmob_dc_dn5)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((((((assign48010_e61608 * locals.var_phit1edge_dn6) * locals.var_phit1edge) + (assign48010_e61610 * locals.var_phit1edge_dn6)) * assign48010_e61617) + (assign48010_e61612 * ((locals.var_alphabmedge_dn6 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn6)))) * locals.var_qdseffedge) + (assign48010_e61618 * locals.var_qdseffedge_dn6)) * locals.var_gmob_dc) - (assign48010_e61620 * locals.var_gmob_dc_dn6)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((((((assign48010_e61608 * locals.var_phit1edge_dn7) * locals.var_phit1edge) + (assign48010_e61610 * locals.var_phit1edge_dn7)) * assign48010_e61617) + (assign48010_e61612 * ((locals.var_alphabmedge_dn7 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn7)))) * locals.var_qdseffedge) + (assign48010_e61618 * locals.var_qdseffedge_dn7)) * locals.var_gmob_dc) - (assign48010_e61620 * locals.var_gmob_dc_dn7)) / (locals.var_gmob_dc * locals.var_gmob_dc)), ((((((((((assign48010_e61608 * locals.var_phit1edge_dn8) * locals.var_phit1edge) + (assign48010_e61610 * locals.var_phit1edge_dn8)) * assign48010_e61617) + (assign48010_e61612 * ((locals.var_alphabmedge_dn8 * locals.var_qmeffedge) + (locals.var_alphabmedge * locals.var_qmeffedge_dn8)))) * locals.var_qdseffedge) + (assign48010_e61618 * locals.var_qdseffedge_dn8)) * locals.var_gmob_dc) - (assign48010_e61620 * locals.var_gmob_dc_dn8)) / (locals.var_gmob_dc * locals.var_gmob_dc)), );
        }

        (locals.var_mavl, locals.var_mavl_dn5, locals.var_mavl_dn6, locals.var_mavl_dn7, locals.var_mavl_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_iimpact, locals.var_iimpact_dn5, locals.var_iimpact_dn6, locals.var_iimpact_dn7, locals.var_iimpact_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign48040_e61633: f64 = if ((locals.var_xg_dc > 0.0) && (p.p41 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1258 = assign48040_e61633;

        if (locals.var_guard1258 != 0.0) {
            let assign48050_e61638: f64 = (locals.var_a3_i * locals.var_dps_dc);
            let assign48050_e61639: f64 = (locals.var_v_ds - assign48050_e61638);
            (locals.var_delvsat, locals.var_delvsat_dn5, locals.var_delvsat_dn6, locals.var_delvsat_dn7, locals.var_delvsat_dn8, ) = (assign48050_e61639, (-(locals.var_a3_i * locals.var_dps_dc_dn5)), (locals.var_v_ds_dn6 - (locals.var_a3_i * locals.var_dps_dc_dn6)), (locals.var_v_ds_dn7 - (locals.var_a3_i * locals.var_dps_dc_dn7)), (-(locals.var_a3_i * locals.var_dps_dc_dn8)), );
        }

        let assign48060_e61644: f64 = if locals.var_delvsat > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1259 = assign48060_e61644;

        if ((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) {
            let assign48070_e61653: f64 = (locals.var_phib_dc + locals.var_vsbstar_dc);
            let assign48070_e61654: f64 = (assign48070_e61653).sqrt();
            let assign48070_e61656: f64 = (assign48070_e61654 - locals.var_sqrt_phib_dc);
            let assign48070_e61657: f64 = (locals.var_a4_i * assign48070_e61656);
            let assign48070_e61658: f64 = (1.0 + assign48070_e61657);
            let assign48070_e61661: f64 = (locals.var_delvsat + 1e-30);
            let assign48070_e61662: f64 = (assign48070_e61658 / assign48070_e61661);
            let assign48070_e61663: f64 = (locals.var_a2_t * assign48070_e61662);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign48070_e61663, (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn5 / (2.0 * assign48070_e61654))) * assign48070_e61661) - (assign48070_e61658 * locals.var_delvsat_dn5)) / (assign48070_e61661 * assign48070_e61661))), (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn6 / (2.0 * assign48070_e61654))) * assign48070_e61661) - (assign48070_e61658 * locals.var_delvsat_dn6)) / (assign48070_e61661 * assign48070_e61661))), (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn7 / (2.0 * assign48070_e61654))) * assign48070_e61661) - (assign48070_e61658 * locals.var_delvsat_dn7)) / (assign48070_e61661 * assign48070_e61661))), (locals.var_a2_t * ((((locals.var_a4_i * (locals.var_vsbstar_dc_dn8 / (2.0 * assign48070_e61654))) * assign48070_e61661) - (assign48070_e61658 * locals.var_delvsat_dn8)) / (assign48070_e61661 * assign48070_e61661))), );
        }

        let assign48080_e61667: f64 = (-locals.var_temp2);
        let assign48080_e61668: f64 = (assign48080_e61667).abs();
        let assign48080_e61670: f64 = if assign48080_e61668 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1260 = assign48080_e61670;

        if (((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) && (locals.var_guard1260 != 0.0)) {
            let assign48090_e61677: f64 = (-locals.var_temp2);
            let assign48090_e61678: f64 = (assign48090_e61677).exp();
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign48090_e61678, (assign48090_e61678 * (-locals.var_temp2_dn5)), (assign48090_e61678 * (-locals.var_temp2_dn6)), (assign48090_e61678 * (-locals.var_temp2_dn7)), (assign48090_e61678 * (-locals.var_temp2_dn8)), );
        }

        let assign48100_e61682: f64 = (-locals.var_temp2);
        let assign48100_e61684: f64 = if assign48100_e61682 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1261 = assign48100_e61684;

        if ((((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) && (locals.var_guard1260 == 0.0)) && (locals.var_guard1261 != 0.0)) {
            let assign48110_e61696: f64 = (-230.25850929940458);
            let assign48110_e61698: f64 = (-locals.var_temp2);
            let assign48110_e61699: f64 = (assign48110_e61696 - assign48110_e61698);
            let assign48110_e61703: f64 = (-230.25850929940458);
            let assign48110_e61705: f64 = (-locals.var_temp2);
            let assign48110_e61706: f64 = (assign48110_e61703 - assign48110_e61705);
            let assign48110_e61709: f64 = (-230.25850929940458);
            let assign48110_e61711: f64 = (-locals.var_temp2);
            let assign48110_e61712: f64 = (assign48110_e61709 - assign48110_e61711);
            let assign48110_e61714: f64 = (assign48110_e61712 * 0.3333333333333333);
            let assign48110_e61715: f64 = (1.0 + assign48110_e61714);
            let assign48110_e61716: f64 = (assign48110_e61706 * assign48110_e61715);
            let assign48110_e61717: f64 = (0.5 * assign48110_e61716);
            let assign48110_e61718: f64 = (1.0 + assign48110_e61717);
            let assign48110_e61719: f64 = (assign48110_e61699 * assign48110_e61718);
            let assign48110_e61720: f64 = (1.0 + assign48110_e61719);
            let assign48110_e61721: f64 = (1e-100 / assign48110_e61720);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign48110_e61721, (-((1e-100 * (((-(-locals.var_temp2_dn5)) * assign48110_e61718) + (assign48110_e61699 * (0.5 * (((-(-locals.var_temp2_dn5)) * assign48110_e61715) + (assign48110_e61706 * ((-(-locals.var_temp2_dn5)) * 0.3333333333333333))))))) / (assign48110_e61720 * assign48110_e61720))), (-((1e-100 * (((-(-locals.var_temp2_dn6)) * assign48110_e61718) + (assign48110_e61699 * (0.5 * (((-(-locals.var_temp2_dn6)) * assign48110_e61715) + (assign48110_e61706 * ((-(-locals.var_temp2_dn6)) * 0.3333333333333333))))))) / (assign48110_e61720 * assign48110_e61720))), (-((1e-100 * (((-(-locals.var_temp2_dn7)) * assign48110_e61718) + (assign48110_e61699 * (0.5 * (((-(-locals.var_temp2_dn7)) * assign48110_e61715) + (assign48110_e61706 * ((-(-locals.var_temp2_dn7)) * 0.3333333333333333))))))) / (assign48110_e61720 * assign48110_e61720))), (-((1e-100 * (((-(-locals.var_temp2_dn8)) * assign48110_e61718) + (assign48110_e61699 * (0.5 * (((-(-locals.var_temp2_dn8)) * assign48110_e61715) + (assign48110_e61706 * ((-(-locals.var_temp2_dn8)) * 0.3333333333333333))))))) / (assign48110_e61720 * assign48110_e61720))), );
        }

        if ((((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) && (locals.var_guard1260 == 0.0)) && (locals.var_guard1261 == 0.0)) {
            let assign48120_e61736: f64 = (-locals.var_temp2);
            let assign48120_e61738: f64 = (assign48120_e61736 - 230.25850929940458);
            let assign48120_e61742: f64 = (-locals.var_temp2);
            let assign48120_e61744: f64 = (assign48120_e61742 - 230.25850929940458);
            let assign48120_e61747: f64 = (-locals.var_temp2);
            let assign48120_e61749: f64 = (assign48120_e61747 - 230.25850929940458);
            let assign48120_e61751: f64 = (assign48120_e61749 * 0.3333333333333333);
            let assign48120_e61752: f64 = (1.0 + assign48120_e61751);
            let assign48120_e61753: f64 = (assign48120_e61744 * assign48120_e61752);
            let assign48120_e61754: f64 = (0.5 * assign48120_e61753);
            let assign48120_e61755: f64 = (1.0 + assign48120_e61754);
            let assign48120_e61756: f64 = (assign48120_e61738 * assign48120_e61755);
            let assign48120_e61757: f64 = (1.0 + assign48120_e61756);
            let assign48120_e61758: f64 = (1e100 * assign48120_e61757);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign48120_e61758, (1e100 * (((-locals.var_temp2_dn5) * assign48120_e61755) + (assign48120_e61738 * (0.5 * (((-locals.var_temp2_dn5) * assign48120_e61752) + (assign48120_e61744 * ((-locals.var_temp2_dn5) * 0.3333333333333333))))))), (1e100 * (((-locals.var_temp2_dn6) * assign48120_e61755) + (assign48120_e61738 * (0.5 * (((-locals.var_temp2_dn6) * assign48120_e61752) + (assign48120_e61744 * ((-locals.var_temp2_dn6) * 0.3333333333333333))))))), (1e100 * (((-locals.var_temp2_dn7) * assign48120_e61755) + (assign48120_e61738 * (0.5 * (((-locals.var_temp2_dn7) * assign48120_e61752) + (assign48120_e61744 * ((-locals.var_temp2_dn7) * 0.3333333333333333))))))), (1e100 * (((-locals.var_temp2_dn8) * assign48120_e61755) + (assign48120_e61738 * (0.5 * (((-locals.var_temp2_dn8) * assign48120_e61752) + (assign48120_e61744 * ((-locals.var_temp2_dn8) * 0.3333333333333333))))))), );
        }

        if ((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) {
            let assign48130_e61767: f64 = (locals.var_delvsat * locals.var_temp__blk936);
            let assign48130_e61768: f64 = (locals.var_a1_i * assign48130_e61767);
            (locals.var_mavl, locals.var_mavl_dn5, locals.var_mavl_dn6, locals.var_mavl_dn7, locals.var_mavl_dn8, ) = (assign48130_e61768, (locals.var_a1_i * ((locals.var_delvsat_dn5 * locals.var_temp__blk936) + (locals.var_delvsat * locals.var_temp__blk936_dn5))), (locals.var_a1_i * ((locals.var_delvsat_dn6 * locals.var_temp__blk936) + (locals.var_delvsat * locals.var_temp__blk936_dn6))), (locals.var_a1_i * ((locals.var_delvsat_dn7 * locals.var_temp__blk936) + (locals.var_delvsat * locals.var_temp__blk936_dn7))), (locals.var_a1_i * ((locals.var_delvsat_dn8 * locals.var_temp__blk936) + (locals.var_delvsat * locals.var_temp__blk936_dn8))), );
        }

        if ((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) {
            let assign48140_e61777: f64 = (locals.var_i_ds + locals.var_i_dsedge);
            let assign48140_e61778: f64 = (locals.var_mavl * assign48140_e61777);
            (locals.var_iimpact, locals.var_iimpact_dn5, locals.var_iimpact_dn6, locals.var_iimpact_dn7, locals.var_iimpact_dn8, ) = (assign48140_e61778, ((locals.var_mavl_dn5 * assign48140_e61777) + (locals.var_mavl * (locals.var_i_ds_dn5 + locals.var_i_dsedge_dn5))), ((locals.var_mavl_dn6 * assign48140_e61777) + (locals.var_mavl * (locals.var_i_ds_dn6 + locals.var_i_dsedge_dn6))), ((locals.var_mavl_dn7 * assign48140_e61777) + (locals.var_mavl * (locals.var_i_ds_dn7 + locals.var_i_dsedge_dn7))), ((locals.var_mavl_dn8 * assign48140_e61777) + (locals.var_mavl * (locals.var_i_ds_dn8 + locals.var_i_dsedge_dn8))), );
        }

        let assign48150_e61784: f64 = (0.5 * locals.var_imaxii_i);
        let assign48150_e61785: f64 = if locals.var_iimpact > assign48150_e61784 { 1.0 } else { 0.0 };
        locals.var_guard1262 = assign48150_e61785;

        if (((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) && (locals.var_guard1262 != 0.0)) {
            let assign48160_e61793: f64 = (2.0 * locals.var_iimpact);
            let assign48160_e61795: f64 = (assign48160_e61793 / locals.var_imaxii_i);
            let assign48160_e61797: f64 = (assign48160_e61795 - 1.0);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign48160_e61797, ((2.0 * locals.var_iimpact_dn5) / locals.var_imaxii_i), ((2.0 * locals.var_iimpact_dn6) / locals.var_imaxii_i), ((2.0 * locals.var_iimpact_dn7) / locals.var_imaxii_i), ((2.0 * locals.var_iimpact_dn8) / locals.var_imaxii_i), );
        }

        if (((locals.var_guard1258 != 0.0) && (locals.var_guard1259 != 0.0)) && (locals.var_guard1262 != 0.0)) {
            let assign48170_e61807: f64 = (0.5 * locals.var_imaxii_i);
            let assign48170_e61813: f64 = (locals.var_temp__blk936 * locals.var_temp__blk936);
            let assign48170_e61814: f64 = (1.0 + assign48170_e61813);
            let assign48170_e61815: f64 = (assign48170_e61814).sqrt();
            let assign48170_e61816: f64 = (locals.var_temp__blk936 / assign48170_e61815);
            let assign48170_e61817: f64 = (1.0 + assign48170_e61816);
            let assign48170_e61818: f64 = (assign48170_e61807 * assign48170_e61817);
            (locals.var_iimpact, locals.var_iimpact_dn5, locals.var_iimpact_dn6, locals.var_iimpact_dn7, locals.var_iimpact_dn8, ) = (assign48170_e61818, (assign48170_e61807 * (((locals.var_temp__blk936_dn5 * assign48170_e61815) - (locals.var_temp__blk936 * (((locals.var_temp__blk936_dn5 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn5)) / (2.0 * assign48170_e61815)))) / (assign48170_e61815 * assign48170_e61815))), (assign48170_e61807 * (((locals.var_temp__blk936_dn6 * assign48170_e61815) - (locals.var_temp__blk936 * (((locals.var_temp__blk936_dn6 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn6)) / (2.0 * assign48170_e61815)))) / (assign48170_e61815 * assign48170_e61815))), (assign48170_e61807 * (((locals.var_temp__blk936_dn7 * assign48170_e61815) - (locals.var_temp__blk936 * (((locals.var_temp__blk936_dn7 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn7)) / (2.0 * assign48170_e61815)))) / (assign48170_e61815 * assign48170_e61815))), (assign48170_e61807 * (((locals.var_temp__blk936_dn8 * assign48170_e61815) - (locals.var_temp__blk936 * (((locals.var_temp__blk936_dn8 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn8)) / (2.0 * assign48170_e61815)))) / (assign48170_e61815 * assign48170_e61815))), );
        }

        let assign48180_e61831: f64 = if (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1456 = assign48180_e61831;

        let assign48190_e61838: f64 = if ((p.p45 > 0.0) || (p.p47 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1457 = assign48190_e61838;

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            locals.var_phib__blk1297 = locals.var_phib_dc;
            locals.var_aphi__blk1298 = locals.var_aphi_dc;
            locals.var_g_0__blk1299 = locals.var_g_0_dc;
            (locals.var_v_xb__blk1300, locals.var_v_xb__blk1300_dn6, locals.var_v_xb__blk1300_dn7, locals.var_v_xb__blk1300_dn8, ) = (locals.var_v_xb_dc_tmp, locals.var_v_xb_dc_tmp_dn6, locals.var_v_xb_dc_tmp_dn7, locals.var_v_xb_dc_tmp_dn8, );
            (locals.var_vsbstar__blk1301, locals.var_vsbstar__blk1301_dn5, locals.var_vsbstar__blk1301_dn6, locals.var_vsbstar__blk1301_dn7, locals.var_vsbstar__blk1301_dn8, ) = (locals.var_vsbstar_dc_tmp, locals.var_vsbstar_dc_tmp_dn5, locals.var_vsbstar_dc_tmp_dn6, locals.var_vsbstar_dc_tmp_dn7, locals.var_vsbstar_dc_tmp_dn8, );
            locals.var_dvbstar__blk1305 = 0.0;
        }

        let assign48260_e61877: f64 = if p.p47 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1458 = assign48260_e61877;

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1458 != 0.0)) {
            let assign48270_e61886: f64 = (locals.var_v_db + locals.var_v_sb);
            let assign48270_e61889: f64 = (locals.var_v_db - locals.var_v_sb);
            let assign48270_e61892: f64 = (locals.var_v_db - locals.var_v_sb);
            let assign48270_e61893: f64 = (assign48270_e61889 * assign48270_e61892);
            let assign48270_e61895: f64 = (assign48270_e61893 + locals.var_bphi_ac);
            let assign48270_e61896: f64 = (assign48270_e61895).sqrt();
            let assign48270_e61897: f64 = (assign48270_e61886 - assign48270_e61896);
            let assign48270_e61898: f64 = (0.5 * assign48270_e61897);
            let assign48270_e61900: f64 = (assign48270_e61898 + locals.var_phix_ac);
            (locals.var_v_xb__blk1300, locals.var_v_xb__blk1300_dn6, locals.var_v_xb__blk1300_dn7, locals.var_v_xb__blk1300_dn8, ) = (assign48270_e61900, (0.5 * ((locals.var_v_db_dn6 + locals.var_v_sb_dn6) - ((((locals.var_v_db_dn6 - locals.var_v_sb_dn6) * assign48270_e61892) + (assign48270_e61889 * (locals.var_v_db_dn6 - locals.var_v_sb_dn6))) / (2.0 * assign48270_e61896)))), (0.5 * ((locals.var_v_db_dn7 + locals.var_v_sb_dn7) - ((((locals.var_v_db_dn7 - locals.var_v_sb_dn7) * assign48270_e61892) + (assign48270_e61889 * (locals.var_v_db_dn7 - locals.var_v_sb_dn7))) / (2.0 * assign48270_e61896)))), (0.5 * ((locals.var_v_db_dn8 + locals.var_v_sb_dn8) - ((((locals.var_v_db_dn8 - locals.var_v_sb_dn8) * assign48270_e61892) + (assign48270_e61889 * (locals.var_v_db_dn8 - locals.var_v_sb_dn8))) / (2.0 * assign48270_e61896)))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1458 != 0.0)) {
            let assign48280_e61912: f64 = locals.var_v_xb__blk1300;
            let assign48280_e61915: f64 = locals.var_v_xb__blk1300;
            let assign48280_e61918: f64 = locals.var_v_xb__blk1300;
            let assign48280_e61919: f64 = (assign48280_e61915 * assign48280_e61918);
            let assign48280_e61921: f64 = (assign48280_e61919 + locals.var_aphi_ac);
            let assign48280_e61922: f64 = (assign48280_e61921).sqrt();
            let assign48280_e61923: f64 = (assign48280_e61912 - assign48280_e61922);
            let assign48280_e61924: f64 = (0.5 * assign48280_e61923);
            let assign48280_e61925: f64 = (locals.var_v_sb - assign48280_e61924);
            let assign48280_e61927: f64 = (assign48280_e61925 + locals.var_phix1_ac);
            (locals.var_vsbstar_ac, locals.var_vsbstar_ac_dn6, locals.var_vsbstar_ac_dn7, locals.var_vsbstar_ac_dn8, ) = (assign48280_e61927, (locals.var_v_sb_dn6 - (0.5 * (locals.var_v_xb__blk1300_dn6 - (((locals.var_v_xb__blk1300_dn6 * assign48280_e61918) + (assign48280_e61915 * locals.var_v_xb__blk1300_dn6)) / (2.0 * assign48280_e61922))))), (locals.var_v_sb_dn7 - (0.5 * (locals.var_v_xb__blk1300_dn7 - (((locals.var_v_xb__blk1300_dn7 * assign48280_e61918) + (assign48280_e61915 * locals.var_v_xb__blk1300_dn7)) / (2.0 * assign48280_e61922))))), (locals.var_v_sb_dn8 - (0.5 * (locals.var_v_xb__blk1300_dn8 - (((locals.var_v_xb__blk1300_dn8 * assign48280_e61918) + (assign48280_e61915 * locals.var_v_xb__blk1300_dn8)) / (2.0 * assign48280_e61922))))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1458 != 0.0)) {
            (locals.var_vsbstar__blk1301, locals.var_vsbstar__blk1301_dn5, locals.var_vsbstar__blk1301_dn6, locals.var_vsbstar__blk1301_dn7, locals.var_vsbstar__blk1301_dn8, ) = (locals.var_vsbstar_ac, 0.0, locals.var_vsbstar_ac_dn6, locals.var_vsbstar_ac_dn7, locals.var_vsbstar_ac_dn8, );
            locals.var_phib__blk1297 = locals.var_phib_ac;
            locals.var_aphi__blk1298 = locals.var_aphi_ac;
            locals.var_g_0__blk1299 = locals.var_g_0_ac;
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48330_e61967: f64 = (locals.var_vgb - locals.var_dvbstar__blk1305);
            let assign48330_e61969: f64 = (assign48330_e61967 - locals.var_vfb_t);
            (locals.var_vgb1__blk1304, locals.var_vgb1__blk1304_dn5, locals.var_vgb1__blk1304_dn6, locals.var_vgb1__blk1304_dn7, locals.var_vgb1__blk1304_dn8, ) = (assign48330_e61969, locals.var_vgb_dn5, locals.var_vgb_dn6, locals.var_vgb_dn7, locals.var_vgb_dn8, );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48340_e61979: f64 = (locals.var_v_ds - locals.var_vdsx);
            let assign48340_e61980: f64 = (0.5 * assign48340_e61979);
            let assign48340_e61981: f64 = (locals.var_vsbstar__blk1301 + assign48340_e61980);
            (locals.var_vsbx__blk1306, locals.var_vsbx__blk1306_dn5, locals.var_vsbx__blk1306_dn6, locals.var_vsbx__blk1306_dn7, locals.var_vsbx__blk1306_dn8, ) = (assign48340_e61981, locals.var_vsbstar__blk1301_dn5, (locals.var_vsbstar__blk1301_dn6 + (0.5 * (locals.var_v_ds_dn6 - locals.var_vdsx_dn6))), (locals.var_vsbstar__blk1301_dn7 + (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7))), locals.var_vsbstar__blk1301_dn8, );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            (locals.var_dctg__blk1318, locals.var_dctg__blk1318_dn5, locals.var_dctg__blk1318_dn6, locals.var_dctg__blk1318_dn7, locals.var_dctg__blk1318_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign48360_e61992: f64 = if locals.var_ctg_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1459 = assign48360_e61992;

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
            let assign48370_e62000: f64 = (locals.var_phib__blk1297 * locals.var_inv_phit);
            locals.var_xbct__blk1309 = assign48370_e62000;
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
            let assign48380_e62010: f64 = (locals.var_vsbx__blk1306 * locals.var_inv_phit);
            (locals.var_xsbstar__blk1310, locals.var_xsbstar__blk1310_dn5, locals.var_xsbstar__blk1310_dn6, locals.var_xsbstar__blk1310_dn7, locals.var_xsbstar__blk1310_dn8, ) = (assign48380_e62010, (locals.var_vsbx__blk1306_dn5 * locals.var_inv_phit), (locals.var_vsbx__blk1306_dn6 * locals.var_inv_phit), (locals.var_vsbx__blk1306_dn7 * locals.var_inv_phit), (locals.var_vsbx__blk1306_dn8 * locals.var_inv_phit), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
            let assign48390_e62020: f64 = (locals.var_vgb1__blk1304 * locals.var_inv_phit);
            (locals.var_xgct__blk1311, locals.var_xgct__blk1311_dn5, locals.var_xgct__blk1311_dn6, locals.var_xgct__blk1311_dn7, locals.var_xgct__blk1311_dn8, ) = (assign48390_e62020, (locals.var_vgb1__blk1304_dn5 * locals.var_inv_phit), (locals.var_vgb1__blk1304_dn6 * locals.var_inv_phit), (locals.var_vgb1__blk1304_dn7 * locals.var_inv_phit), (locals.var_vgb1__blk1304_dn8 * locals.var_inv_phit), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
            let assign48400_e62031: f64 = (0.5 * locals.var_g_0__blk1299);
            let assign48400_e62033: f64 = (locals.var_xbct__blk1309).sqrt();
            let assign48400_e62034: f64 = (assign48400_e62031 / assign48400_e62033);
            let assign48400_e62035: f64 = (1.0 + assign48400_e62034);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign48400_e62035, 0.0, 0.0, 0.0, 0.0, );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
            let assign48410_e62046: f64 = (locals.var_xbct__blk1309).sqrt();
            let assign48410_e62047: f64 = (locals.var_g_0__blk1299 * assign48410_e62046);
            let assign48410_e62048: f64 = (locals.var_xbct__blk1309 + assign48410_e62047);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign48410_e62048, 0.0, 0.0, 0.0, 0.0, );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
            let assign48420_e62058: f64 = (locals.var_xgct__blk1311 - locals.var_temp2);
            let assign48420_e62060: f64 = (assign48420_e62058 / locals.var_temp1);
            let assign48420_e62063: f64 = (0.5 * locals.var_xbct__blk1309);
            let assign48420_e62064: f64 = (assign48420_e62060 + assign48420_e62063);
            let assign48420_e62067: f64 = (1.0 + locals.var_ctb_i);
            let assign48420_e62069: f64 = (assign48420_e62067 * locals.var_xsbstar__blk1310);
            let assign48420_e62070: f64 = (assign48420_e62064 - assign48420_e62069);
            (locals.var_xwict__blk1312, locals.var_xwict__blk1312_dn5, locals.var_xwict__blk1312_dn6, locals.var_xwict__blk1312_dn7, locals.var_xwict__blk1312_dn8, ) = (assign48420_e62070, (((((locals.var_xgct__blk1311_dn5 - locals.var_temp2_dn5) * locals.var_temp1) - (assign48420_e62058 * locals.var_temp1_dn5)) / (locals.var_temp1 * locals.var_temp1)) - (assign48420_e62067 * locals.var_xsbstar__blk1310_dn5)), (((((locals.var_xgct__blk1311_dn6 - locals.var_temp2_dn6) * locals.var_temp1) - (assign48420_e62058 * locals.var_temp1_dn6)) / (locals.var_temp1 * locals.var_temp1)) - (assign48420_e62067 * locals.var_xsbstar__blk1310_dn6)), (((((locals.var_xgct__blk1311_dn7 - locals.var_temp2_dn7) * locals.var_temp1) - (assign48420_e62058 * locals.var_temp1_dn7)) / (locals.var_temp1 * locals.var_temp1)) - (assign48420_e62067 * locals.var_xsbstar__blk1310_dn7)), (((((locals.var_xgct__blk1311_dn8 - locals.var_temp2_dn8) * locals.var_temp1) - (assign48420_e62058 * locals.var_temp1_dn8)) / (locals.var_temp1 * locals.var_temp1)) - (assign48420_e62067 * locals.var_xsbstar__blk1310_dn8)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
            let assign48430_e62080: f64 = (0.5 * locals.var_xbct__blk1309);
            let assign48430_e62082: f64 = (assign48430_e62080 + 2.0);
            locals.var_xctmax__blk1313 = assign48430_e62082;
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
            let assign48440_e62092: f64 = (locals.var_xbct__blk1309 + locals.var_xsbstar__blk1310);
            (locals.var_xnct__blk1314, locals.var_xnct__blk1314_dn5, locals.var_xnct__blk1314_dn6, locals.var_xnct__blk1314_dn7, locals.var_xnct__blk1314_dn8, ) = (assign48440_e62092, locals.var_xsbstar__blk1310_dn5, locals.var_xsbstar__blk1310_dn6, locals.var_xsbstar__blk1310_dn7, locals.var_xsbstar__blk1310_dn8, );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
            let assign48450_e62102: f64 = (locals.var_xgct__blk1311 - locals.var_xnct__blk1314);
            let assign48450_e62105: f64 = (locals.var_xnct__blk1314).sqrt();
            let assign48450_e62106: f64 = (locals.var_g_0__blk1299 * assign48450_e62105);
            let assign48450_e62107: f64 = (assign48450_e62102 - assign48450_e62106);
            let assign48450_e62111: f64 = (locals.var_xbct__blk1309 / locals.var_g_0__blk1299);
            let assign48450_e62113: f64 = (locals.var_xbct__blk1309).sqrt();
            let assign48450_e62114: f64 = (assign48450_e62111 + assign48450_e62113);
            let assign48450_e62115: f64 = (assign48450_e62114).ln();
            let assign48450_e62116: f64 = (2.0 * assign48450_e62115);
            let assign48450_e62117: f64 = (assign48450_e62107 - assign48450_e62116);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign48450_e62117, ((locals.var_xgct__blk1311_dn5 - locals.var_xnct__blk1314_dn5) - (locals.var_g_0__blk1299 * (locals.var_xnct__blk1314_dn5 / (2.0 * assign48450_e62105)))), ((locals.var_xgct__blk1311_dn6 - locals.var_xnct__blk1314_dn6) - (locals.var_g_0__blk1299 * (locals.var_xnct__blk1314_dn6 / (2.0 * assign48450_e62105)))), ((locals.var_xgct__blk1311_dn7 - locals.var_xnct__blk1314_dn7) - (locals.var_g_0__blk1299 * (locals.var_xnct__blk1314_dn7 / (2.0 * assign48450_e62105)))), ((locals.var_xgct__blk1311_dn8 - locals.var_xnct__blk1314_dn8) - (locals.var_g_0__blk1299 * (locals.var_xnct__blk1314_dn8 / (2.0 * assign48450_e62105)))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
            let assign48460_e62127: f64 = (2.0 * locals.var_temp1);
            let assign48460_e62129: f64 = (assign48460_e62127 + locals.var_xctmax__blk1313);
            (locals.var_xmict__blk1315, locals.var_xmict__blk1315_dn5, locals.var_xmict__blk1315_dn6, locals.var_xmict__blk1315_dn7, locals.var_xmict__blk1315_dn8, ) = (assign48460_e62129, (2.0 * locals.var_temp1_dn5), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
            let assign48470_e62140: f64 = (locals.var_xwict__blk1312 + locals.var_xmict__blk1315);
            let assign48470_e62143: f64 = (locals.var_xwict__blk1312 - locals.var_xmict__blk1315);
            let assign48470_e62146: f64 = (locals.var_xwict__blk1312 - locals.var_xmict__blk1315);
            let assign48470_e62147: f64 = (assign48470_e62143 * assign48470_e62146);
            let assign48470_e62149: f64 = (assign48470_e62147 + 20.0);
            let assign48470_e62150: f64 = (assign48470_e62149).sqrt();
            let assign48470_e62151: f64 = (assign48470_e62140 + assign48470_e62150);
            let assign48470_e62152: f64 = (0.5 * assign48470_e62151);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign48470_e62152, (0.5 * ((locals.var_xwict__blk1312_dn5 + locals.var_xmict__blk1315_dn5) + ((((locals.var_xwict__blk1312_dn5 - locals.var_xmict__blk1315_dn5) * assign48470_e62146) + (assign48470_e62143 * (locals.var_xwict__blk1312_dn5 - locals.var_xmict__blk1315_dn5))) / (2.0 * assign48470_e62150)))), (0.5 * ((locals.var_xwict__blk1312_dn6 + locals.var_xmict__blk1315_dn6) + ((((locals.var_xwict__blk1312_dn6 - locals.var_xmict__blk1315_dn6) * assign48470_e62146) + (assign48470_e62143 * (locals.var_xwict__blk1312_dn6 - locals.var_xmict__blk1315_dn6))) / (2.0 * assign48470_e62150)))), (0.5 * ((locals.var_xwict__blk1312_dn7 + locals.var_xmict__blk1315_dn7) + ((((locals.var_xwict__blk1312_dn7 - locals.var_xmict__blk1315_dn7) * assign48470_e62146) + (assign48470_e62143 * (locals.var_xwict__blk1312_dn7 - locals.var_xmict__blk1315_dn7))) / (2.0 * assign48470_e62150)))), (0.5 * ((locals.var_xwict__blk1312_dn8 + locals.var_xmict__blk1315_dn8) + ((((locals.var_xwict__blk1312_dn8 - locals.var_xmict__blk1315_dn8) * assign48470_e62146) + (assign48470_e62143 * (locals.var_xwict__blk1312_dn8 - locals.var_xmict__blk1315_dn8))) / (2.0 * assign48470_e62150)))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
            let assign48480_e62163: f64 = (locals.var_xgct__blk1311 - locals.var_xsbstar__blk1310);
            let assign48480_e62164: f64 = (2.0 * assign48480_e62163);
            let assign48480_e62166: f64 = (assign48480_e62164 - locals.var_xctmax__blk1313);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign48480_e62166, (2.0 * (locals.var_xgct__blk1311_dn5 - locals.var_xsbstar__blk1310_dn5)), (2.0 * (locals.var_xgct__blk1311_dn6 - locals.var_xsbstar__blk1310_dn6)), (2.0 * (locals.var_xgct__blk1311_dn7 - locals.var_xsbstar__blk1310_dn7)), (2.0 * (locals.var_xgct__blk1311_dn8 - locals.var_xsbstar__blk1310_dn8)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
            let assign48490_e62177: f64 = (locals.var_temp1 + locals.var_temp2);
            let assign48490_e62180: f64 = (locals.var_temp1 - locals.var_temp2);
            let assign48490_e62183: f64 = (locals.var_temp1 - locals.var_temp2);
            let assign48490_e62184: f64 = (assign48490_e62180 * assign48490_e62183);
            let assign48490_e62186: f64 = (assign48490_e62184 + 20.0);
            let assign48490_e62187: f64 = (assign48490_e62186).sqrt();
            let assign48490_e62188: f64 = (assign48490_e62177 - assign48490_e62187);
            let assign48490_e62189: f64 = (0.5 * assign48490_e62188);
            (locals.var_xsubct__blk1316, locals.var_xsubct__blk1316_dn5, locals.var_xsubct__blk1316_dn6, locals.var_xsubct__blk1316_dn7, locals.var_xsubct__blk1316_dn8, ) = (assign48490_e62189, (0.5 * ((locals.var_temp1_dn5 + locals.var_temp2_dn5) - ((((locals.var_temp1_dn5 - locals.var_temp2_dn5) * assign48490_e62183) + (assign48490_e62180 * (locals.var_temp1_dn5 - locals.var_temp2_dn5))) / (2.0 * assign48490_e62187)))), (0.5 * ((locals.var_temp1_dn6 + locals.var_temp2_dn6) - ((((locals.var_temp1_dn6 - locals.var_temp2_dn6) * assign48490_e62183) + (assign48490_e62180 * (locals.var_temp1_dn6 - locals.var_temp2_dn6))) / (2.0 * assign48490_e62187)))), (0.5 * ((locals.var_temp1_dn7 + locals.var_temp2_dn7) - ((((locals.var_temp1_dn7 - locals.var_temp2_dn7) * assign48490_e62183) + (assign48490_e62180 * (locals.var_temp1_dn7 - locals.var_temp2_dn7))) / (2.0 * assign48490_e62187)))), (0.5 * ((locals.var_temp1_dn8 + locals.var_temp2_dn8) - ((((locals.var_temp1_dn8 - locals.var_temp2_dn8) * assign48490_e62183) + (assign48490_e62180 * (locals.var_temp1_dn8 - locals.var_temp2_dn8))) / (2.0 * assign48490_e62187)))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
            let assign48500_e62200: f64 = (locals.var_xsubct__blk1316 + locals.var_xctmax__blk1313);
            let assign48500_e62203: f64 = (locals.var_xsubct__blk1316 - locals.var_xctmax__blk1313);
            let assign48500_e62206: f64 = (locals.var_xsubct__blk1316 - locals.var_xctmax__blk1313);
            let assign48500_e62207: f64 = (assign48500_e62203 * assign48500_e62206);
            let assign48500_e62209: f64 = (assign48500_e62207 + 5.0);
            let assign48500_e62210: f64 = (assign48500_e62209).sqrt();
            let assign48500_e62211: f64 = (assign48500_e62200 - assign48500_e62210);
            let assign48500_e62212: f64 = (0.5 * assign48500_e62211);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign48500_e62212, (0.5 * (locals.var_xsubct__blk1316_dn5 - (((locals.var_xsubct__blk1316_dn5 * assign48500_e62206) + (assign48500_e62203 * locals.var_xsubct__blk1316_dn5)) / (2.0 * assign48500_e62210)))), (0.5 * (locals.var_xsubct__blk1316_dn6 - (((locals.var_xsubct__blk1316_dn6 * assign48500_e62206) + (assign48500_e62203 * locals.var_xsubct__blk1316_dn6)) / (2.0 * assign48500_e62210)))), (0.5 * (locals.var_xsubct__blk1316_dn7 - (((locals.var_xsubct__blk1316_dn7 * assign48500_e62206) + (assign48500_e62203 * locals.var_xsubct__blk1316_dn7)) / (2.0 * assign48500_e62210)))), (0.5 * (locals.var_xsubct__blk1316_dn8 - (((locals.var_xsubct__blk1316_dn8 * assign48500_e62206) + (assign48500_e62203 * locals.var_xsubct__blk1316_dn8)) / (2.0 * assign48500_e62210)))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
            let assign48510_e62223: f64 = (-locals.var_xctmax__blk1313);
            let assign48510_e62224: f64 = (locals.var_temp1 + assign48510_e62223);
            let assign48510_e62227: f64 = (-locals.var_xctmax__blk1313);
            let assign48510_e62228: f64 = (locals.var_temp1 - assign48510_e62227);
            let assign48510_e62231: f64 = (-locals.var_xctmax__blk1313);
            let assign48510_e62232: f64 = (locals.var_temp1 - assign48510_e62231);
            let assign48510_e62233: f64 = (assign48510_e62228 * assign48510_e62232);
            let assign48510_e62235: f64 = (assign48510_e62233 + 20.0);
            let assign48510_e62236: f64 = (assign48510_e62235).sqrt();
            let assign48510_e62237: f64 = (assign48510_e62224 + assign48510_e62236);
            let assign48510_e62238: f64 = (0.5 * assign48510_e62237);
            (locals.var_xct__blk1317, locals.var_xct__blk1317_dn5, locals.var_xct__blk1317_dn6, locals.var_xct__blk1317_dn7, locals.var_xct__blk1317_dn8, ) = (assign48510_e62238, (0.5 * (locals.var_temp1_dn5 + (((locals.var_temp1_dn5 * assign48510_e62232) + (assign48510_e62228 * locals.var_temp1_dn5)) / (2.0 * assign48510_e62236)))), (0.5 * (locals.var_temp1_dn6 + (((locals.var_temp1_dn6 * assign48510_e62232) + (assign48510_e62228 * locals.var_temp1_dn6)) / (2.0 * assign48510_e62236)))), (0.5 * (locals.var_temp1_dn7 + (((locals.var_temp1_dn7 * assign48510_e62232) + (assign48510_e62228 * locals.var_temp1_dn7)) / (2.0 * assign48510_e62236)))), (0.5 * (locals.var_temp1_dn8 + (((locals.var_temp1_dn8 * assign48510_e62232) + (assign48510_e62228 * locals.var_temp1_dn8)) / (2.0 * assign48510_e62236)))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) {
            let assign48520_e62249: f64 = (locals.var_xct__blk1317 / locals.var_xctmax__blk1313);
            let assign48520_e62251: f64 = (assign48520_e62249 + 1.0);
            let assign48520_e62252: f64 = (locals.var_ctg_t * assign48520_e62251);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign48520_e62252, (locals.var_ctg_t * (locals.var_xct__blk1317_dn5 / locals.var_xctmax__blk1313)), (locals.var_ctg_t * (locals.var_xct__blk1317_dn6 / locals.var_xctmax__blk1313)), (locals.var_ctg_t * (locals.var_xct__blk1317_dn7 / locals.var_xctmax__blk1313)), (locals.var_ctg_t * (locals.var_xct__blk1317_dn8 / locals.var_xctmax__blk1313)), );
        }

        let assign48530_e62257: f64 = (-230.25850929940458);
        let assign48530_e62258: f64 = if locals.var_temp2 > assign48530_e62257 { 1.0 } else { 0.0 };
        locals.var_guard1460 = assign48530_e62258;

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) && (locals.var_guard1460 != 0.0)) {
            let assign48540_e62267: f64 = (locals.var_temp2).exp();
            (locals.var_dctg__blk1318, locals.var_dctg__blk1318_dn5, locals.var_dctg__blk1318_dn6, locals.var_dctg__blk1318_dn7, locals.var_dctg__blk1318_dn8, ) = (assign48540_e62267, (assign48540_e62267 * locals.var_temp2_dn5), (assign48540_e62267 * locals.var_temp2_dn6), (assign48540_e62267 * locals.var_temp2_dn7), (assign48540_e62267 * locals.var_temp2_dn8), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1459 != 0.0)) && (locals.var_guard1460 == 0.0)) {
            let assign48550_e62281: f64 = (-230.25850929940458);
            let assign48550_e62283: f64 = (assign48550_e62281 - locals.var_temp2);
            let assign48550_e62287: f64 = (-230.25850929940458);
            let assign48550_e62289: f64 = (assign48550_e62287 - locals.var_temp2);
            let assign48550_e62292: f64 = (-230.25850929940458);
            let assign48550_e62294: f64 = (assign48550_e62292 - locals.var_temp2);
            let assign48550_e62296: f64 = (assign48550_e62294 * 0.3333333333333333);
            let assign48550_e62297: f64 = (1.0 + assign48550_e62296);
            let assign48550_e62298: f64 = (assign48550_e62289 * assign48550_e62297);
            let assign48550_e62299: f64 = (0.5 * assign48550_e62298);
            let assign48550_e62300: f64 = (1.0 + assign48550_e62299);
            let assign48550_e62301: f64 = (assign48550_e62283 * assign48550_e62300);
            let assign48550_e62302: f64 = (1.0 + assign48550_e62301);
            let assign48550_e62303: f64 = (1e-100 / assign48550_e62302);
            (locals.var_dctg__blk1318, locals.var_dctg__blk1318_dn5, locals.var_dctg__blk1318_dn6, locals.var_dctg__blk1318_dn7, locals.var_dctg__blk1318_dn8, ) = (assign48550_e62303, (-((1e-100 * (((-locals.var_temp2_dn5) * assign48550_e62300) + (assign48550_e62283 * (0.5 * (((-locals.var_temp2_dn5) * assign48550_e62297) + (assign48550_e62289 * ((-locals.var_temp2_dn5) * 0.3333333333333333))))))) / (assign48550_e62302 * assign48550_e62302))), (-((1e-100 * (((-locals.var_temp2_dn6) * assign48550_e62300) + (assign48550_e62283 * (0.5 * (((-locals.var_temp2_dn6) * assign48550_e62297) + (assign48550_e62289 * ((-locals.var_temp2_dn6) * 0.3333333333333333))))))) / (assign48550_e62302 * assign48550_e62302))), (-((1e-100 * (((-locals.var_temp2_dn7) * assign48550_e62300) + (assign48550_e62283 * (0.5 * (((-locals.var_temp2_dn7) * assign48550_e62297) + (assign48550_e62289 * ((-locals.var_temp2_dn7) * 0.3333333333333333))))))) / (assign48550_e62302 * assign48550_e62302))), (-((1e-100 * (((-locals.var_temp2_dn8) * assign48550_e62300) + (assign48550_e62283 * (0.5 * (((-locals.var_temp2_dn8) * assign48550_e62297) + (assign48550_e62289 * ((-locals.var_temp2_dn8) * 0.3333333333333333))))))) / (assign48550_e62302 * assign48550_e62302))), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48560_e62312: f64 = (locals.var_ct_t * locals.var_dctg__blk1318);
            let assign48560_e62313: f64 = (1.0 + assign48560_e62312);
            (locals.var_ct_fact__blk1319, locals.var_ct_fact__blk1319_dn5, locals.var_ct_fact__blk1319_dn6, locals.var_ct_fact__blk1319_dn7, locals.var_ct_fact__blk1319_dn8, ) = (assign48560_e62313, (locals.var_ct_t * locals.var_dctg__blk1318_dn5), (locals.var_ct_t * locals.var_dctg__blk1318_dn6), (locals.var_ct_t * locals.var_dctg__blk1318_dn7), (locals.var_ct_t * locals.var_dctg__blk1318_dn8), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48570_e62321: f64 = (locals.var_phit * locals.var_ct_fact__blk1319);
            (locals.var_phitct__blk1320, locals.var_phitct__blk1320_dn5, locals.var_phitct__blk1320_dn6, locals.var_phitct__blk1320_dn7, locals.var_phitct__blk1320_dn8, ) = (assign48570_e62321, (locals.var_phit * locals.var_ct_fact__blk1319_dn5), (locals.var_phit * locals.var_ct_fact__blk1319_dn6), (locals.var_phit * locals.var_ct_fact__blk1319_dn7), (locals.var_phit * locals.var_ct_fact__blk1319_dn8), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48580_e62331: f64 = (locals.var_psced_i * locals.var_vdsx);
            let assign48580_e62332: f64 = (1.0 + assign48580_e62331);
            let assign48580_e62333: f64 = (locals.var_psce_i * assign48580_e62332);
            let assign48580_e62337: f64 = (locals.var_psceb_i * locals.var_vsbx__blk1306);
            let assign48580_e62338: f64 = (1.0 + assign48580_e62337);
            let assign48580_e62339: f64 = (assign48580_e62333 * assign48580_e62338);
            (locals.var_dphit1__blk1321, locals.var_dphit1__blk1321_dn5, locals.var_dphit1__blk1321_dn6, locals.var_dphit1__blk1321_dn7, locals.var_dphit1__blk1321_dn8, ) = (assign48580_e62339, (assign48580_e62333 * (locals.var_psceb_i * locals.var_vsbx__blk1306_dn5)), (((locals.var_psce_i * (locals.var_psced_i * locals.var_vdsx_dn6)) * assign48580_e62338) + (assign48580_e62333 * (locals.var_psceb_i * locals.var_vsbx__blk1306_dn6))), (((locals.var_psce_i * (locals.var_psced_i * locals.var_vdsx_dn7)) * assign48580_e62338) + (assign48580_e62333 * (locals.var_psceb_i * locals.var_vsbx__blk1306_dn7))), (assign48580_e62333 * (locals.var_psceb_i * locals.var_vsbx__blk1306_dn8)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48590_e62348: f64 = (1.0 + locals.var_dphit1__blk1321);
            let assign48590_e62349: f64 = (locals.var_phitct__blk1320 * assign48590_e62348);
            (locals.var_phit1__blk1322, locals.var_phit1__blk1322_dn5, locals.var_phit1__blk1322_dn6, locals.var_phit1__blk1322_dn7, locals.var_phit1__blk1322_dn8, ) = (assign48590_e62349, ((locals.var_phitct__blk1320_dn5 * assign48590_e62348) + (locals.var_phitct__blk1320 * locals.var_dphit1__blk1321_dn5)), ((locals.var_phitct__blk1320_dn6 * assign48590_e62348) + (locals.var_phitct__blk1320 * locals.var_dphit1__blk1321_dn6)), ((locals.var_phitct__blk1320_dn7 * assign48590_e62348) + (locals.var_phitct__blk1320 * locals.var_dphit1__blk1321_dn7)), ((locals.var_phitct__blk1320_dn8 * assign48590_e62348) + (locals.var_phitct__blk1320 * locals.var_dphit1__blk1321_dn8)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48600_e62357: f64 = (1.0 / locals.var_phit1__blk1322);
            (locals.var_inv_phit1__blk1323, locals.var_inv_phit1__blk1323_dn5, locals.var_inv_phit1__blk1323_dn6, locals.var_inv_phit1__blk1323_dn7, locals.var_inv_phit1__blk1323_dn8, ) = (assign48600_e62357, (-(locals.var_phit1__blk1322_dn5 / (locals.var_phit1__blk1322 * locals.var_phit1__blk1322))), (-(locals.var_phit1__blk1322_dn6 / (locals.var_phit1__blk1322 * locals.var_phit1__blk1322))), (-(locals.var_phit1__blk1322_dn7 / (locals.var_phit1__blk1322 * locals.var_phit1__blk1322))), (-(locals.var_phit1__blk1322_dn8 / (locals.var_phit1__blk1322 * locals.var_phit1__blk1322))), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48610_e62366: f64 = (locals.var_phit * locals.var_inv_phit1__blk1323);
            let assign48610_e62367: f64 = (assign48610_e62366).sqrt();
            let assign48610_e62368: f64 = (locals.var_g_0__blk1299 * assign48610_e62367);
            (locals.var_gf__blk1307, locals.var_gf__blk1307_dn5, locals.var_gf__blk1307_dn6, locals.var_gf__blk1307_dn7, locals.var_gf__blk1307_dn8, ) = (assign48610_e62368, (locals.var_g_0__blk1299 * ((locals.var_phit * locals.var_inv_phit1__blk1323_dn5) / (2.0 * assign48610_e62367))), (locals.var_g_0__blk1299 * ((locals.var_phit * locals.var_inv_phit1__blk1323_dn6) / (2.0 * assign48610_e62367))), (locals.var_g_0__blk1299 * ((locals.var_phit * locals.var_inv_phit1__blk1323_dn7) / (2.0 * assign48610_e62367))), (locals.var_g_0__blk1299 * ((locals.var_phit * locals.var_inv_phit1__blk1323_dn8) / (2.0 * assign48610_e62367))), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48620_e62376: f64 = (locals.var_gf__blk1307 * locals.var_gf__blk1307);
            (locals.var_gf2__blk1308, locals.var_gf2__blk1308_dn5, locals.var_gf2__blk1308_dn6, locals.var_gf2__blk1308_dn7, locals.var_gf2__blk1308_dn8, ) = (assign48620_e62376, ((locals.var_gf__blk1307_dn5 * locals.var_gf__blk1307) + (locals.var_gf__blk1307 * locals.var_gf__blk1307_dn5)), ((locals.var_gf__blk1307_dn6 * locals.var_gf__blk1307) + (locals.var_gf__blk1307 * locals.var_gf__blk1307_dn6)), ((locals.var_gf__blk1307_dn7 * locals.var_gf__blk1307) + (locals.var_gf__blk1307 * locals.var_gf__blk1307_dn7)), ((locals.var_gf__blk1307_dn8 * locals.var_gf__blk1307) + (locals.var_gf__blk1307 * locals.var_gf__blk1307_dn8)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48630_e62384: f64 = (1.0 / locals.var_gf2__blk1308);
            (locals.var_inv_gf2__blk1324, locals.var_inv_gf2__blk1324_dn5, locals.var_inv_gf2__blk1324_dn6, locals.var_inv_gf2__blk1324_dn7, locals.var_inv_gf2__blk1324_dn8, ) = (assign48630_e62384, (-(locals.var_gf2__blk1308_dn5 / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))), (-(locals.var_gf2__blk1308_dn6 / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))), (-(locals.var_gf2__blk1308_dn7 / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))), (-(locals.var_gf2__blk1308_dn8 / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48640_e62392: f64 = (locals.var_vsbstar__blk1301 * locals.var_inv_phit1__blk1323);
            (locals.var_ux__blk1325, locals.var_ux__blk1325_dn5, locals.var_ux__blk1325_dn6, locals.var_ux__blk1325_dn7, locals.var_ux__blk1325_dn8, ) = (assign48640_e62392, ((locals.var_vsbstar__blk1301_dn5 * locals.var_inv_phit1__blk1323) + (locals.var_vsbstar__blk1301 * locals.var_inv_phit1__blk1323_dn5)), ((locals.var_vsbstar__blk1301_dn6 * locals.var_inv_phit1__blk1323) + (locals.var_vsbstar__blk1301 * locals.var_inv_phit1__blk1323_dn6)), ((locals.var_vsbstar__blk1301_dn7 * locals.var_inv_phit1__blk1323) + (locals.var_vsbstar__blk1301 * locals.var_inv_phit1__blk1323_dn7)), ((locals.var_vsbstar__blk1301_dn8 * locals.var_inv_phit1__blk1323) + (locals.var_vsbstar__blk1301 * locals.var_inv_phit1__blk1323_dn8)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48650_e62400: f64 = (locals.var_vgb1__blk1304 * locals.var_inv_phit1__blk1323);
            (locals.var_xg__blk1326, locals.var_xg__blk1326_dn5, locals.var_xg__blk1326_dn6, locals.var_xg__blk1326_dn7, locals.var_xg__blk1326_dn8, ) = (assign48650_e62400, ((locals.var_vgb1__blk1304_dn5 * locals.var_inv_phit1__blk1323) + (locals.var_vgb1__blk1304 * locals.var_inv_phit1__blk1323_dn5)), ((locals.var_vgb1__blk1304_dn6 * locals.var_inv_phit1__blk1323) + (locals.var_vgb1__blk1304 * locals.var_inv_phit1__blk1323_dn6)), ((locals.var_vgb1__blk1304_dn7 * locals.var_inv_phit1__blk1323) + (locals.var_vgb1__blk1304 * locals.var_inv_phit1__blk1323_dn7)), ((locals.var_vgb1__blk1304_dn8 * locals.var_inv_phit1__blk1323) + (locals.var_vgb1__blk1304 * locals.var_inv_phit1__blk1323_dn8)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48660_e62408: f64 = (2.0 * locals.var_vdsx);
            let assign48660_e62413: f64 = (locals.var_cfd_i * locals.var_vdsx);
            let assign48660_e62414: f64 = (1.0 + assign48660_e62413);
            let assign48660_e62415: f64 = (assign48660_e62414).sqrt();
            let assign48660_e62416: f64 = (1.0 + assign48660_e62415);
            let assign48660_e62417: f64 = (assign48660_e62408 / assign48660_e62416);
            (locals.var_vdsp__blk1327, locals.var_vdsp__blk1327_dn6, locals.var_vdsp__blk1327_dn7, ) = (assign48660_e62417, ((((2.0 * locals.var_vdsx_dn6) * assign48660_e62416) - (assign48660_e62408 * ((locals.var_cfd_i * locals.var_vdsx_dn6) / (2.0 * assign48660_e62415)))) / (assign48660_e62416 * assign48660_e62416)), ((((2.0 * locals.var_vdsx_dn7) * assign48660_e62416) - (assign48660_e62408 * ((locals.var_cfd_i * locals.var_vdsx_dn7) / (2.0 * assign48660_e62415)))) / (assign48660_e62416 * assign48660_e62416)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48670_e62425: f64 = (locals.var_cf_i * locals.var_vdsp__blk1327);
            let assign48670_e62429: f64 = (locals.var_cfb_i * locals.var_vsbx__blk1306);
            let assign48670_e62430: f64 = (1.0 + assign48670_e62429);
            let assign48670_e62431: f64 = (assign48670_e62425 * assign48670_e62430);
            (locals.var_delphib__blk1328, locals.var_delphib__blk1328_dn5, locals.var_delphib__blk1328_dn6, locals.var_delphib__blk1328_dn7, locals.var_delphib__blk1328_dn8, ) = (assign48670_e62431, (assign48670_e62425 * (locals.var_cfb_i * locals.var_vsbx__blk1306_dn5)), (((locals.var_cf_i * locals.var_vdsp__blk1327_dn6) * assign48670_e62430) + (assign48670_e62425 * (locals.var_cfb_i * locals.var_vsbx__blk1306_dn6))), (((locals.var_cf_i * locals.var_vdsp__blk1327_dn7) * assign48670_e62430) + (assign48670_e62425 * (locals.var_cfb_i * locals.var_vsbx__blk1306_dn7))), (assign48670_e62425 * (locals.var_cfb_i * locals.var_vsbx__blk1306_dn8)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48680_e62439: f64 = (locals.var_phib__blk1297 * locals.var_inv_phit1__blk1323);
            (locals.var_xb__blk1329, locals.var_xb__blk1329_dn5, locals.var_xb__blk1329_dn6, locals.var_xb__blk1329_dn7, locals.var_xb__blk1329_dn8, ) = (assign48680_e62439, (locals.var_phib__blk1297 * locals.var_inv_phit1__blk1323_dn5), (locals.var_phib__blk1297 * locals.var_inv_phit1__blk1323_dn6), (locals.var_phib__blk1297 * locals.var_inv_phit1__blk1323_dn7), (locals.var_phib__blk1297 * locals.var_inv_phit1__blk1323_dn8), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48690_e62447: f64 = (locals.var_v_xb__blk1300 * locals.var_v_xb__blk1300);
            let assign48690_e62449: f64 = (assign48690_e62447 + locals.var_aphi__blk1298);
            let assign48690_e62450: f64 = (assign48690_e62449).sqrt();
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign48690_e62450, 0.0, (((locals.var_v_xb__blk1300_dn6 * locals.var_v_xb__blk1300) + (locals.var_v_xb__blk1300 * locals.var_v_xb__blk1300_dn6)) / (2.0 * assign48690_e62450)), (((locals.var_v_xb__blk1300_dn7 * locals.var_v_xb__blk1300) + (locals.var_v_xb__blk1300 * locals.var_v_xb__blk1300_dn7)) / (2.0 * assign48690_e62450)), (((locals.var_v_xb__blk1300_dn8 * locals.var_v_xb__blk1300) + (locals.var_v_xb__blk1300 * locals.var_v_xb__blk1300_dn8)) / (2.0 * assign48690_e62450)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48700_e62458: f64 = (locals.var_v_xb__blk1300 - locals.var_delphib__blk1328);
            let assign48700_e62461: f64 = (locals.var_v_xb__blk1300 - locals.var_delphib__blk1328);
            let assign48700_e62462: f64 = (assign48700_e62458 * assign48700_e62461);
            let assign48700_e62464: f64 = (assign48700_e62462 + locals.var_aphi__blk1298);
            let assign48700_e62465: f64 = (assign48700_e62464).sqrt();
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign48700_e62465, ((((-locals.var_delphib__blk1328_dn5) * assign48700_e62461) + (assign48700_e62458 * (-locals.var_delphib__blk1328_dn5))) / (2.0 * assign48700_e62465)), ((((locals.var_v_xb__blk1300_dn6 - locals.var_delphib__blk1328_dn6) * assign48700_e62461) + (assign48700_e62458 * (locals.var_v_xb__blk1300_dn6 - locals.var_delphib__blk1328_dn6))) / (2.0 * assign48700_e62465)), ((((locals.var_v_xb__blk1300_dn7 - locals.var_delphib__blk1328_dn7) * assign48700_e62461) + (assign48700_e62458 * (locals.var_v_xb__blk1300_dn7 - locals.var_delphib__blk1328_dn7))) / (2.0 * assign48700_e62465)), ((((locals.var_v_xb__blk1300_dn8 - locals.var_delphib__blk1328_dn8) * assign48700_e62461) + (assign48700_e62458 * (locals.var_v_xb__blk1300_dn8 - locals.var_delphib__blk1328_dn8))) / (2.0 * assign48700_e62465)), );
        }

    }

    pub(super) fn stamp_transient_block_20(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48710_e62473: f64 = (0.5 * locals.var_inv_phit1__blk1323);
            let assign48710_e62476: f64 = (locals.var_delphib__blk1328 + locals.var_temp1);
            let assign48710_e62478: f64 = (assign48710_e62476 - locals.var_temp2);
            let assign48710_e62479: f64 = (assign48710_e62473 * assign48710_e62478);
            (locals.var_delxb__blk1330, locals.var_delxb__blk1330_dn5, locals.var_delxb__blk1330_dn6, locals.var_delxb__blk1330_dn7, locals.var_delxb__blk1330_dn8, ) = (assign48710_e62479, (((0.5 * locals.var_inv_phit1__blk1323_dn5) * assign48710_e62478) + (assign48710_e62473 * ((locals.var_delphib__blk1328_dn5 + locals.var_temp1_dn5) - locals.var_temp2_dn5))), (((0.5 * locals.var_inv_phit1__blk1323_dn6) * assign48710_e62478) + (assign48710_e62473 * ((locals.var_delphib__blk1328_dn6 + locals.var_temp1_dn6) - locals.var_temp2_dn6))), (((0.5 * locals.var_inv_phit1__blk1323_dn7) * assign48710_e62478) + (assign48710_e62473 * ((locals.var_delphib__blk1328_dn7 + locals.var_temp1_dn7) - locals.var_temp2_dn7))), (((0.5 * locals.var_inv_phit1__blk1323_dn8) * assign48710_e62478) + (assign48710_e62473 * ((locals.var_delphib__blk1328_dn8 + locals.var_temp1_dn8) - locals.var_temp2_dn8))), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48720_e62487: f64 = (locals.var_xb__blk1329 + locals.var_ux__blk1325);
            (locals.var_xno_s__blk1331, locals.var_xno_s__blk1331_dn5, locals.var_xno_s__blk1331_dn6, locals.var_xno_s__blk1331_dn7, locals.var_xno_s__blk1331_dn8, ) = (assign48720_e62487, (locals.var_xb__blk1329_dn5 + locals.var_ux__blk1325_dn5), (locals.var_xb__blk1329_dn6 + locals.var_ux__blk1325_dn6), (locals.var_xb__blk1329_dn7 + locals.var_ux__blk1325_dn7), (locals.var_xb__blk1329_dn8 + locals.var_ux__blk1325_dn8), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48730_e62495: f64 = (locals.var_xno_s__blk1331 - locals.var_delxb__blk1330);
            (locals.var_xn_s__blk1332, locals.var_xn_s__blk1332_dn5, locals.var_xn_s__blk1332_dn6, locals.var_xn_s__blk1332_dn7, locals.var_xn_s__blk1332_dn8, ) = (assign48730_e62495, (locals.var_xno_s__blk1331_dn5 - locals.var_delxb__blk1330_dn5), (locals.var_xno_s__blk1331_dn6 - locals.var_delxb__blk1330_dn6), (locals.var_xno_s__blk1331_dn7 - locals.var_delxb__blk1330_dn7), (locals.var_xno_s__blk1331_dn8 - locals.var_delxb__blk1330_dn8), );
        }

        let assign48740_e62500: f64 = if p.p45 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1461 = assign48740_e62500;

        let assign48750_e62502: f64 = (locals.var_xn_s__blk1332).abs();
        let assign48750_e62504: f64 = if assign48750_e62502 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1462 = assign48750_e62504;

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 != 0.0)) {
            let assign48760_e62517: f64 = (0.5 * locals.var_xn_s__blk1332);
            let assign48760_e62521: f64 = (0.3125 * locals.var_xn_s__blk1332);
            let assign48760_e62522: f64 = (1.0 - assign48760_e62521);
            let assign48760_e62523: f64 = (assign48760_e62517 * assign48760_e62522);
            let assign48760_e62524: f64 = (1.0 - assign48760_e62523);
            let assign48760_e62525: f64 = (locals.var_gf__blk1307 * assign48760_e62524);
            let assign48760_e62526: f64 = (1.0 + assign48760_e62525);
            (locals.var_nscr__blk1333, locals.var_nscr__blk1333_dn5, locals.var_nscr__blk1333_dn6, locals.var_nscr__blk1333_dn7, locals.var_nscr__blk1333_dn8, ) = (assign48760_e62526, ((locals.var_gf__blk1307_dn5 * assign48760_e62524) + (locals.var_gf__blk1307 * (-(((0.5 * locals.var_xn_s__blk1332_dn5) * assign48760_e62522) + (assign48760_e62517 * (-(0.3125 * locals.var_xn_s__blk1332_dn5))))))), ((locals.var_gf__blk1307_dn6 * assign48760_e62524) + (locals.var_gf__blk1307 * (-(((0.5 * locals.var_xn_s__blk1332_dn6) * assign48760_e62522) + (assign48760_e62517 * (-(0.3125 * locals.var_xn_s__blk1332_dn6))))))), ((locals.var_gf__blk1307_dn7 * assign48760_e62524) + (locals.var_gf__blk1307 * (-(((0.5 * locals.var_xn_s__blk1332_dn7) * assign48760_e62522) + (assign48760_e62517 * (-(0.3125 * locals.var_xn_s__blk1332_dn7))))))), ((locals.var_gf__blk1307_dn8 * assign48760_e62524) + (locals.var_gf__blk1307 * (-(((0.5 * locals.var_xn_s__blk1332_dn8) * assign48760_e62522) + (assign48760_e62517 * (-(0.3125 * locals.var_xn_s__blk1332_dn8))))))), );
        }

        let assign48770_e62531: f64 = if locals.var_xn_s__blk1332 < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1463 = assign48770_e62531;

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1463 != 0.0)) {
            let assign48780_e62543: f64 = (-locals.var_xn_s__blk1332);
            let assign48780_e62544: f64 = (assign48780_e62543).exp();
            (locals.var_delta_ns__blk1347, locals.var_delta_ns__blk1347_dn5, locals.var_delta_ns__blk1347_dn6, locals.var_delta_ns__blk1347_dn7, locals.var_delta_ns__blk1347_dn8, ) = (assign48780_e62544, (assign48780_e62544 * (-locals.var_xn_s__blk1332_dn5)), (assign48780_e62544 * (-locals.var_xn_s__blk1332_dn6)), (assign48780_e62544 * (-locals.var_xn_s__blk1332_dn7)), (assign48780_e62544 * (-locals.var_xn_s__blk1332_dn8)), );
        }

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) && (locals.var_guard1463 == 0.0)) {
            let assign48790_e62562: f64 = (locals.var_xn_s__blk1332 - 460.51701859880916);
            let assign48790_e62567: f64 = (locals.var_xn_s__blk1332 - 460.51701859880916);
            let assign48790_e62571: f64 = (locals.var_xn_s__blk1332 - 460.51701859880916);
            let assign48790_e62573: f64 = (assign48790_e62571 * 0.3333333333333333);
            let assign48790_e62574: f64 = (1.0 + assign48790_e62573);
            let assign48790_e62575: f64 = (assign48790_e62567 * assign48790_e62574);
            let assign48790_e62576: f64 = (0.5 * assign48790_e62575);
            let assign48790_e62577: f64 = (1.0 + assign48790_e62576);
            let assign48790_e62578: f64 = (assign48790_e62562 * assign48790_e62577);
            let assign48790_e62579: f64 = (1.0 + assign48790_e62578);
            let assign48790_e62580: f64 = (1e-200 / assign48790_e62579);
            (locals.var_delta_ns__blk1347, locals.var_delta_ns__blk1347_dn5, locals.var_delta_ns__blk1347_dn6, locals.var_delta_ns__blk1347_dn7, locals.var_delta_ns__blk1347_dn8, ) = (assign48790_e62580, (-((1e-200 * ((locals.var_xn_s__blk1332_dn5 * assign48790_e62577) + (assign48790_e62562 * (0.5 * ((locals.var_xn_s__blk1332_dn5 * assign48790_e62574) + (assign48790_e62567 * (locals.var_xn_s__blk1332_dn5 * 0.3333333333333333))))))) / (assign48790_e62579 * assign48790_e62579))), (-((1e-200 * ((locals.var_xn_s__blk1332_dn6 * assign48790_e62577) + (assign48790_e62562 * (0.5 * ((locals.var_xn_s__blk1332_dn6 * assign48790_e62574) + (assign48790_e62567 * (locals.var_xn_s__blk1332_dn6 * 0.3333333333333333))))))) / (assign48790_e62579 * assign48790_e62579))), (-((1e-200 * ((locals.var_xn_s__blk1332_dn7 * assign48790_e62577) + (assign48790_e62562 * (0.5 * ((locals.var_xn_s__blk1332_dn7 * assign48790_e62574) + (assign48790_e62567 * (locals.var_xn_s__blk1332_dn7 * 0.3333333333333333))))))) / (assign48790_e62579 * assign48790_e62579))), (-((1e-200 * ((locals.var_xn_s__blk1332_dn8 * assign48790_e62577) + (assign48790_e62562 * (0.5 * ((locals.var_xn_s__blk1332_dn8 * assign48790_e62574) + (assign48790_e62567 * (locals.var_xn_s__blk1332_dn8 * 0.3333333333333333))))))) / (assign48790_e62579 * assign48790_e62579))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
            let (assign48800_e62597,) = {
    if (locals.var_xn_s__blk1332 > 0.0) {
        (1.0,)
    } else {
        let assign48800_e62596: f64 = (-1.0);
        (assign48800_e62596,)
    }
};
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign48800_e62597, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1461 != 0.0)) && (locals.var_guard1462 == 0.0)) {
            let assign48810_e62611: f64 = (locals.var_temp__blk936 * locals.var_gf__blk1307);
            let assign48810_e62616: f64 = (1.0 - locals.var_xn_s__blk1332);
            let assign48810_e62617: f64 = (locals.var_delta_ns__blk1347 * assign48810_e62616);
            let assign48810_e62618: f64 = (1.0 - assign48810_e62617);
            let assign48810_e62619: f64 = (assign48810_e62611 * assign48810_e62618);
            let assign48810_e62624: f64 = (1.0 - locals.var_delta_ns__blk1347);
            let assign48810_e62625: f64 = (locals.var_xn_s__blk1332 * assign48810_e62624);
            let assign48810_e62626: f64 = (assign48810_e62625).sqrt();
            let assign48810_e62627: f64 = (2.0 * assign48810_e62626);
            let assign48810_e62628: f64 = (assign48810_e62619 / assign48810_e62627);
            let assign48810_e62629: f64 = (1.0 + assign48810_e62628);
            (locals.var_nscr__blk1333, locals.var_nscr__blk1333_dn5, locals.var_nscr__blk1333_dn6, locals.var_nscr__blk1333_dn7, locals.var_nscr__blk1333_dn8, ) = (assign48810_e62629, (((((((locals.var_temp__blk936_dn5 * locals.var_gf__blk1307) + (locals.var_temp__blk936 * locals.var_gf__blk1307_dn5)) * assign48810_e62618) + (assign48810_e62611 * (-((locals.var_delta_ns__blk1347_dn5 * assign48810_e62616) + (locals.var_delta_ns__blk1347 * (-locals.var_xn_s__blk1332_dn5)))))) * assign48810_e62627) - (assign48810_e62619 * (2.0 * (((locals.var_xn_s__blk1332_dn5 * assign48810_e62624) + (locals.var_xn_s__blk1332 * (-locals.var_delta_ns__blk1347_dn5))) / (2.0 * assign48810_e62626))))) / (assign48810_e62627 * assign48810_e62627)), (((((((locals.var_temp__blk936_dn6 * locals.var_gf__blk1307) + (locals.var_temp__blk936 * locals.var_gf__blk1307_dn6)) * assign48810_e62618) + (assign48810_e62611 * (-((locals.var_delta_ns__blk1347_dn6 * assign48810_e62616) + (locals.var_delta_ns__blk1347 * (-locals.var_xn_s__blk1332_dn6)))))) * assign48810_e62627) - (assign48810_e62619 * (2.0 * (((locals.var_xn_s__blk1332_dn6 * assign48810_e62624) + (locals.var_xn_s__blk1332 * (-locals.var_delta_ns__blk1347_dn6))) / (2.0 * assign48810_e62626))))) / (assign48810_e62627 * assign48810_e62627)), (((((((locals.var_temp__blk936_dn7 * locals.var_gf__blk1307) + (locals.var_temp__blk936 * locals.var_gf__blk1307_dn7)) * assign48810_e62618) + (assign48810_e62611 * (-((locals.var_delta_ns__blk1347_dn7 * assign48810_e62616) + (locals.var_delta_ns__blk1347 * (-locals.var_xn_s__blk1332_dn7)))))) * assign48810_e62627) - (assign48810_e62619 * (2.0 * (((locals.var_xn_s__blk1332_dn7 * assign48810_e62624) + (locals.var_xn_s__blk1332 * (-locals.var_delta_ns__blk1347_dn7))) / (2.0 * assign48810_e62626))))) / (assign48810_e62627 * assign48810_e62627)), (((((((locals.var_temp__blk936_dn8 * locals.var_gf__blk1307) + (locals.var_temp__blk936 * locals.var_gf__blk1307_dn8)) * assign48810_e62618) + (assign48810_e62611 * (-((locals.var_delta_ns__blk1347_dn8 * assign48810_e62616) + (locals.var_delta_ns__blk1347 * (-locals.var_xn_s__blk1332_dn8)))))) * assign48810_e62627) - (assign48810_e62619 * (2.0 * (((locals.var_xn_s__blk1332_dn8 * assign48810_e62624) + (locals.var_xn_s__blk1332 * (-locals.var_delta_ns__blk1347_dn8))) / (2.0 * assign48810_e62626))))) / (assign48810_e62627 * assign48810_e62627)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1461 == 0.0)) {
            let assign48820_e62641: f64 = (0.5 * locals.var_gf__blk1307);
            let assign48820_e62643: f64 = (locals.var_xn_s__blk1332).sqrt();
            let assign48820_e62644: f64 = (assign48820_e62641 / assign48820_e62643);
            let assign48820_e62645: f64 = (1.0 + assign48820_e62644);
            (locals.var_nscr__blk1333, locals.var_nscr__blk1333_dn5, locals.var_nscr__blk1333_dn6, locals.var_nscr__blk1333_dn7, locals.var_nscr__blk1333_dn8, ) = (assign48820_e62645, ((((0.5 * locals.var_gf__blk1307_dn5) * assign48820_e62643) - (assign48820_e62641 * (locals.var_xn_s__blk1332_dn5 / (2.0 * assign48820_e62643)))) / (assign48820_e62643 * assign48820_e62643)), ((((0.5 * locals.var_gf__blk1307_dn6) * assign48820_e62643) - (assign48820_e62641 * (locals.var_xn_s__blk1332_dn6 / (2.0 * assign48820_e62643)))) / (assign48820_e62643 * assign48820_e62643)), ((((0.5 * locals.var_gf__blk1307_dn7) * assign48820_e62643) - (assign48820_e62641 * (locals.var_xn_s__blk1332_dn7 / (2.0 * assign48820_e62643)))) / (assign48820_e62643 * assign48820_e62643)), ((((0.5 * locals.var_gf__blk1307_dn8) * assign48820_e62643) - (assign48820_e62641 * (locals.var_xn_s__blk1332_dn8 / (2.0 * assign48820_e62643)))) / (assign48820_e62643 * assign48820_e62643)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48830_e62654: f64 = (locals.var_xn_s__blk1332).sqrt();
            let assign48830_e62655: f64 = (locals.var_gf__blk1307 * assign48830_e62654);
            let assign48830_e62656: f64 = (locals.var_xn_s__blk1332 + assign48830_e62655);
            let assign48830_e62660: f64 = (locals.var_nscr__blk1333 - 1.0);
            let assign48830_e62661: f64 = (assign48830_e62660).ln();
            let assign48830_e62662: f64 = (locals.var_nscr__blk1333 * assign48830_e62661);
            let assign48830_e62663: f64 = (assign48830_e62656 - assign48830_e62662);
            (locals.var_xthscr__blk1334, locals.var_xthscr__blk1334_dn5, locals.var_xthscr__blk1334_dn6, locals.var_xthscr__blk1334_dn7, locals.var_xthscr__blk1334_dn8, ) = (assign48830_e62663, ((locals.var_xn_s__blk1332_dn5 + ((locals.var_gf__blk1307_dn5 * assign48830_e62654) + (locals.var_gf__blk1307 * (locals.var_xn_s__blk1332_dn5 / (2.0 * assign48830_e62654))))) - ((locals.var_nscr__blk1333_dn5 * assign48830_e62661) + (locals.var_nscr__blk1333 * (locals.var_nscr__blk1333_dn5 / assign48830_e62660)))), ((locals.var_xn_s__blk1332_dn6 + ((locals.var_gf__blk1307_dn6 * assign48830_e62654) + (locals.var_gf__blk1307 * (locals.var_xn_s__blk1332_dn6 / (2.0 * assign48830_e62654))))) - ((locals.var_nscr__blk1333_dn6 * assign48830_e62661) + (locals.var_nscr__blk1333 * (locals.var_nscr__blk1333_dn6 / assign48830_e62660)))), ((locals.var_xn_s__blk1332_dn7 + ((locals.var_gf__blk1307_dn7 * assign48830_e62654) + (locals.var_gf__blk1307 * (locals.var_xn_s__blk1332_dn7 / (2.0 * assign48830_e62654))))) - ((locals.var_nscr__blk1333_dn7 * assign48830_e62661) + (locals.var_nscr__blk1333 * (locals.var_nscr__blk1333_dn7 / assign48830_e62660)))), ((locals.var_xn_s__blk1332_dn8 + ((locals.var_gf__blk1307_dn8 * assign48830_e62654) + (locals.var_gf__blk1307 * (locals.var_xn_s__blk1332_dn8 / (2.0 * assign48830_e62654))))) - ((locals.var_nscr__blk1333_dn8 * assign48830_e62661) + (locals.var_nscr__blk1333 * (locals.var_nscr__blk1333_dn8 / assign48830_e62660)))), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48840_e62671: f64 = (locals.var_xg__blk1326 - locals.var_xthscr__blk1334);
            let assign48840_e62673: f64 = (assign48840_e62671 / locals.var_nscr__blk1333);
            (locals.var_xgtscr__blk1335, locals.var_xgtscr__blk1335_dn5, locals.var_xgtscr__blk1335_dn6, locals.var_xgtscr__blk1335_dn7, locals.var_xgtscr__blk1335_dn8, ) = (assign48840_e62673, ((((locals.var_xg__blk1326_dn5 - locals.var_xthscr__blk1334_dn5) * locals.var_nscr__blk1333) - (assign48840_e62671 * locals.var_nscr__blk1333_dn5)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)), ((((locals.var_xg__blk1326_dn6 - locals.var_xthscr__blk1334_dn6) * locals.var_nscr__blk1333) - (assign48840_e62671 * locals.var_nscr__blk1333_dn6)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)), ((((locals.var_xg__blk1326_dn7 - locals.var_xthscr__blk1334_dn7) * locals.var_nscr__blk1333) - (assign48840_e62671 * locals.var_nscr__blk1333_dn7)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)), ((((locals.var_xg__blk1326_dn8 - locals.var_xthscr__blk1334_dn8) * locals.var_nscr__blk1333) - (assign48840_e62671 * locals.var_nscr__blk1333_dn8)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign48850_e62681: f64 = (0.5 * locals.var_gf2__blk1308);
            let assign48850_e62685: f64 = (8.0 / locals.var_gf2__blk1308);
            let assign48850_e62686: f64 = (1.0 + assign48850_e62685);
            let assign48850_e62687: f64 = (assign48850_e62686).sqrt();
            let assign48850_e62689: f64 = (assign48850_e62687 - 1.0);
            let assign48850_e62690: f64 = (assign48850_e62681 * assign48850_e62689);
            (locals.var_qbscr__blk1341, locals.var_qbscr__blk1341_dn5, locals.var_qbscr__blk1341_dn6, locals.var_qbscr__blk1341_dn7, locals.var_qbscr__blk1341_dn8, ) = (assign48850_e62690, (((0.5 * locals.var_gf2__blk1308_dn5) * assign48850_e62689) + (assign48850_e62681 * ((-((8.0 * locals.var_gf2__blk1308_dn5) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) / (2.0 * assign48850_e62687)))), (((0.5 * locals.var_gf2__blk1308_dn6) * assign48850_e62689) + (assign48850_e62681 * ((-((8.0 * locals.var_gf2__blk1308_dn6) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) / (2.0 * assign48850_e62687)))), (((0.5 * locals.var_gf2__blk1308_dn7) * assign48850_e62689) + (assign48850_e62681 * ((-((8.0 * locals.var_gf2__blk1308_dn7) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) / (2.0 * assign48850_e62687)))), (((0.5 * locals.var_gf2__blk1308_dn8) * assign48850_e62689) + (assign48850_e62681 * ((-((8.0 * locals.var_gf2__blk1308_dn8) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) / (2.0 * assign48850_e62687)))), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            (locals.var_qiscr__blk1340, locals.var_qiscr__blk1340_dn5, locals.var_qiscr__blk1340_dn6, locals.var_qiscr__blk1340_dn7, locals.var_qiscr__blk1340_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_fscr__blk1342, locals.var_fscr__blk1342_dn5, locals.var_fscr__blk1342_dn6, locals.var_fscr__blk1342_dn7, locals.var_fscr__blk1342_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign48880_e62707: f64 = (-30.0);
        let assign48880_e62708: f64 = if locals.var_xgtscr__blk1335 > assign48880_e62707 { 1.0 } else { 0.0 };
        locals.var_guard1464 = assign48880_e62708;

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
            let assign48890_e62716: f64 = (locals.var_nscr__blk1333 * locals.var_xgtscr__blk1335);
            let assign48890_e62718: f64 = (assign48890_e62716 - 1.0);
            (locals.var_xgtscr0__blk1336, locals.var_xgtscr0__blk1336_dn5, locals.var_xgtscr0__blk1336_dn6, locals.var_xgtscr0__blk1336_dn7, locals.var_xgtscr0__blk1336_dn8, ) = (assign48890_e62718, ((locals.var_nscr__blk1333_dn5 * locals.var_xgtscr__blk1335) + (locals.var_nscr__blk1333 * locals.var_xgtscr__blk1335_dn5)), ((locals.var_nscr__blk1333_dn6 * locals.var_xgtscr__blk1335) + (locals.var_nscr__blk1333 * locals.var_xgtscr__blk1335_dn6)), ((locals.var_nscr__blk1333_dn7 * locals.var_xgtscr__blk1335) + (locals.var_nscr__blk1333 * locals.var_xgtscr__blk1335_dn7)), ((locals.var_nscr__blk1333_dn8 * locals.var_xgtscr__blk1335) + (locals.var_nscr__blk1333 * locals.var_xgtscr__blk1335_dn8)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
            let assign48900_e62730: f64 = (locals.var_xgtscr0__blk1336 * locals.var_xgtscr0__blk1336);
            let assign48900_e62732: f64 = (assign48900_e62730 + 10.0);
            let assign48900_e62733: f64 = (assign48900_e62732).sqrt();
            let assign48900_e62734: f64 = (locals.var_xgtscr0__blk1336 + assign48900_e62733);
            let assign48900_e62735: f64 = (0.5 * assign48900_e62734);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign48900_e62735, (0.5 * (locals.var_xgtscr0__blk1336_dn5 + (((locals.var_xgtscr0__blk1336_dn5 * locals.var_xgtscr0__blk1336) + (locals.var_xgtscr0__blk1336 * locals.var_xgtscr0__blk1336_dn5)) / (2.0 * assign48900_e62733)))), (0.5 * (locals.var_xgtscr0__blk1336_dn6 + (((locals.var_xgtscr0__blk1336_dn6 * locals.var_xgtscr0__blk1336) + (locals.var_xgtscr0__blk1336 * locals.var_xgtscr0__blk1336_dn6)) / (2.0 * assign48900_e62733)))), (0.5 * (locals.var_xgtscr0__blk1336_dn7 + (((locals.var_xgtscr0__blk1336_dn7 * locals.var_xgtscr0__blk1336) + (locals.var_xgtscr0__blk1336 * locals.var_xgtscr0__blk1336_dn7)) / (2.0 * assign48900_e62733)))), (0.5 * (locals.var_xgtscr0__blk1336_dn8 + (((locals.var_xgtscr0__blk1336_dn8 * locals.var_xgtscr0__blk1336) + (locals.var_xgtscr0__blk1336 * locals.var_xgtscr0__blk1336_dn8)) / (2.0 * assign48900_e62733)))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
            let assign48910_e62745: f64 = (locals.var_temp__blk936).ln();
            let assign48910_e62746: f64 = (locals.var_xgtscr__blk1335 - assign48910_e62745);
            (locals.var_qiscr0si__blk1337, locals.var_qiscr0si__blk1337_dn5, locals.var_qiscr0si__blk1337_dn6, locals.var_qiscr0si__blk1337_dn7, locals.var_qiscr0si__blk1337_dn8, ) = (assign48910_e62746, (locals.var_xgtscr__blk1335_dn5 - (locals.var_temp__blk936_dn5 / locals.var_temp__blk936)), (locals.var_xgtscr__blk1335_dn6 - (locals.var_temp__blk936_dn6 / locals.var_temp__blk936)), (locals.var_xgtscr__blk1335_dn7 - (locals.var_temp__blk936_dn7 / locals.var_temp__blk936)), (locals.var_xgtscr__blk1335_dn8 - (locals.var_temp__blk936_dn8 / locals.var_temp__blk936)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
            let assign48920_e62758: f64 = (locals.var_qiscr0si__blk1337 * locals.var_qiscr0si__blk1337);
            let assign48920_e62760: f64 = (assign48920_e62758 + 2.0);
            let assign48920_e62761: f64 = (assign48920_e62760).sqrt();
            let assign48920_e62762: f64 = (locals.var_qiscr0si__blk1337 + assign48920_e62761);
            let assign48920_e62763: f64 = (0.5 * assign48920_e62762);
            (locals.var_qiscr0__blk1338, locals.var_qiscr0__blk1338_dn5, locals.var_qiscr0__blk1338_dn6, locals.var_qiscr0__blk1338_dn7, locals.var_qiscr0__blk1338_dn8, ) = (assign48920_e62763, (0.5 * (locals.var_qiscr0si__blk1337_dn5 + (((locals.var_qiscr0si__blk1337_dn5 * locals.var_qiscr0si__blk1337) + (locals.var_qiscr0si__blk1337 * locals.var_qiscr0si__blk1337_dn5)) / (2.0 * assign48920_e62761)))), (0.5 * (locals.var_qiscr0si__blk1337_dn6 + (((locals.var_qiscr0si__blk1337_dn6 * locals.var_qiscr0si__blk1337) + (locals.var_qiscr0si__blk1337 * locals.var_qiscr0si__blk1337_dn6)) / (2.0 * assign48920_e62761)))), (0.5 * (locals.var_qiscr0si__blk1337_dn7 + (((locals.var_qiscr0si__blk1337_dn7 * locals.var_qiscr0si__blk1337) + (locals.var_qiscr0si__blk1337 * locals.var_qiscr0si__blk1337_dn7)) / (2.0 * assign48920_e62761)))), (0.5 * (locals.var_qiscr0si__blk1337_dn8 + (((locals.var_qiscr0si__blk1337_dn8 * locals.var_qiscr0si__blk1337) + (locals.var_qiscr0si__blk1337 * locals.var_qiscr0si__blk1337_dn8)) / (2.0 * assign48920_e62761)))), );
        }

        let assign48930_e62768: f64 = (locals.var_xgtscr__blk1335 - locals.var_qiscr0__blk1338);
        let assign48930_e62770: f64 = if assign48930_e62768 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1465 = assign48930_e62770;

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 != 0.0)) {
            let assign48940_e62780: f64 = (locals.var_xgtscr__blk1335 - locals.var_qiscr0__blk1338);
            let assign48940_e62781: f64 = (assign48940_e62780).exp();
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign48940_e62781, (assign48940_e62781 * (locals.var_xgtscr__blk1335_dn5 - locals.var_qiscr0__blk1338_dn5)), (assign48940_e62781 * (locals.var_xgtscr__blk1335_dn6 - locals.var_qiscr0__blk1338_dn6)), (assign48940_e62781 * (locals.var_xgtscr__blk1335_dn7 - locals.var_qiscr0__blk1338_dn7)), (assign48940_e62781 * (locals.var_xgtscr__blk1335_dn8 - locals.var_qiscr0__blk1338_dn8)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1465 == 0.0)) {
            let assign48950_e62796: f64 = (locals.var_xgtscr__blk1335 - locals.var_qiscr0__blk1338);
            let assign48950_e62798: f64 = (assign48950_e62796 - 230.25850929940458);
            let assign48950_e62803: f64 = (locals.var_xgtscr__blk1335 - locals.var_qiscr0__blk1338);
            let assign48950_e62805: f64 = (assign48950_e62803 - 230.25850929940458);
            let assign48950_e62809: f64 = (locals.var_xgtscr__blk1335 - locals.var_qiscr0__blk1338);
            let assign48950_e62811: f64 = (assign48950_e62809 - 230.25850929940458);
            let assign48950_e62813: f64 = (assign48950_e62811 * 0.3333333333333333);
            let assign48950_e62814: f64 = (1.0 + assign48950_e62813);
            let assign48950_e62815: f64 = (assign48950_e62805 * assign48950_e62814);
            let assign48950_e62816: f64 = (0.5 * assign48950_e62815);
            let assign48950_e62817: f64 = (1.0 + assign48950_e62816);
            let assign48950_e62818: f64 = (assign48950_e62798 * assign48950_e62817);
            let assign48950_e62819: f64 = (1.0 + assign48950_e62818);
            let assign48950_e62820: f64 = (1e100 * assign48950_e62819);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign48950_e62820, (1e100 * (((locals.var_xgtscr__blk1335_dn5 - locals.var_qiscr0__blk1338_dn5) * assign48950_e62817) + (assign48950_e62798 * (0.5 * (((locals.var_xgtscr__blk1335_dn5 - locals.var_qiscr0__blk1338_dn5) * assign48950_e62814) + (assign48950_e62805 * ((locals.var_xgtscr__blk1335_dn5 - locals.var_qiscr0__blk1338_dn5) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr__blk1335_dn6 - locals.var_qiscr0__blk1338_dn6) * assign48950_e62817) + (assign48950_e62798 * (0.5 * (((locals.var_xgtscr__blk1335_dn6 - locals.var_qiscr0__blk1338_dn6) * assign48950_e62814) + (assign48950_e62805 * ((locals.var_xgtscr__blk1335_dn6 - locals.var_qiscr0__blk1338_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr__blk1335_dn7 - locals.var_qiscr0__blk1338_dn7) * assign48950_e62817) + (assign48950_e62798 * (0.5 * (((locals.var_xgtscr__blk1335_dn7 - locals.var_qiscr0__blk1338_dn7) * assign48950_e62814) + (assign48950_e62805 * ((locals.var_xgtscr__blk1335_dn7 - locals.var_qiscr0__blk1338_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr__blk1335_dn8 - locals.var_qiscr0__blk1338_dn8) * assign48950_e62817) + (assign48950_e62798 * (0.5 * (((locals.var_xgtscr__blk1335_dn8 - locals.var_qiscr0__blk1338_dn8) * assign48950_e62814) + (assign48950_e62805 * ((locals.var_xgtscr__blk1335_dn8 - locals.var_qiscr0__blk1338_dn8) * 0.3333333333333333))))))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
            let assign48960_e62830: f64 = (locals.var_temp__blk936 / locals.var_nscr__blk1333);
            (locals.var_dscr0__blk1339, locals.var_dscr0__blk1339_dn5, locals.var_dscr0__blk1339_dn6, locals.var_dscr0__blk1339_dn7, locals.var_dscr0__blk1339_dn8, ) = (assign48960_e62830, (((locals.var_temp__blk936_dn5 * locals.var_nscr__blk1333) - (locals.var_temp__blk936 * locals.var_nscr__blk1333_dn5)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)), (((locals.var_temp__blk936_dn6 * locals.var_nscr__blk1333) - (locals.var_temp__blk936 * locals.var_nscr__blk1333_dn6)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)), (((locals.var_temp__blk936_dn7 * locals.var_nscr__blk1333) - (locals.var_temp__blk936 * locals.var_nscr__blk1333_dn7)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)), (((locals.var_temp__blk936_dn8 * locals.var_nscr__blk1333) - (locals.var_temp__blk936 * locals.var_nscr__blk1333_dn8)) / (locals.var_nscr__blk1333 * locals.var_nscr__blk1333)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
            let assign48970_e62841: f64 = (locals.var_qiscr0__blk1338 + 1.0);
            let assign48970_e62842: f64 = (2.0 * assign48970_e62841);
            let assign48970_e62844: f64 = (assign48970_e62842 - locals.var_dscr0__blk1339);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign48970_e62844, ((2.0 * locals.var_qiscr0__blk1338_dn5) - locals.var_dscr0__blk1339_dn5), ((2.0 * locals.var_qiscr0__blk1338_dn6) - locals.var_dscr0__blk1339_dn6), ((2.0 * locals.var_qiscr0__blk1338_dn7) - locals.var_dscr0__blk1339_dn7), ((2.0 * locals.var_qiscr0__blk1338_dn8) - locals.var_dscr0__blk1339_dn8), );
        }

        let assign48980_e62849: f64 = if locals.var_dscr0__blk1339 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1466 = assign48980_e62849;

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1466 != 0.0)) {
            let assign48990_e62862: f64 = (locals.var_dscr0__blk1339 * locals.var_temp__blk936);
            let assign48990_e62863: f64 = (1.0 + assign48990_e62862);
            let assign48990_e62864: f64 = (assign48990_e62863).sqrt();
            let assign48990_e62866: f64 = (assign48990_e62864 - 1.0);
            let assign48990_e62868: f64 = (assign48990_e62866 / locals.var_dscr0__blk1339);
            let assign48990_e62869: f64 = (locals.var_qiscr0__blk1338 - assign48990_e62868);
            let assign48990_e62871: f64 = (assign48990_e62869 + 1.0);
            let assign48990_e62872: f64 = (locals.var_nscr__blk1333 * assign48990_e62871);
            (locals.var_qiscr__blk1340, locals.var_qiscr__blk1340_dn5, locals.var_qiscr__blk1340_dn6, locals.var_qiscr__blk1340_dn7, locals.var_qiscr__blk1340_dn8, ) = (assign48990_e62872, ((locals.var_nscr__blk1333_dn5 * assign48990_e62871) + (locals.var_nscr__blk1333 * (locals.var_qiscr0__blk1338_dn5 - ((((((locals.var_dscr0__blk1339_dn5 * locals.var_temp__blk936) + (locals.var_dscr0__blk1339 * locals.var_temp__blk936_dn5)) / (2.0 * assign48990_e62864)) * locals.var_dscr0__blk1339) - (assign48990_e62866 * locals.var_dscr0__blk1339_dn5)) / (locals.var_dscr0__blk1339 * locals.var_dscr0__blk1339))))), ((locals.var_nscr__blk1333_dn6 * assign48990_e62871) + (locals.var_nscr__blk1333 * (locals.var_qiscr0__blk1338_dn6 - ((((((locals.var_dscr0__blk1339_dn6 * locals.var_temp__blk936) + (locals.var_dscr0__blk1339 * locals.var_temp__blk936_dn6)) / (2.0 * assign48990_e62864)) * locals.var_dscr0__blk1339) - (assign48990_e62866 * locals.var_dscr0__blk1339_dn6)) / (locals.var_dscr0__blk1339 * locals.var_dscr0__blk1339))))), ((locals.var_nscr__blk1333_dn7 * assign48990_e62871) + (locals.var_nscr__blk1333 * (locals.var_qiscr0__blk1338_dn7 - ((((((locals.var_dscr0__blk1339_dn7 * locals.var_temp__blk936) + (locals.var_dscr0__blk1339 * locals.var_temp__blk936_dn7)) / (2.0 * assign48990_e62864)) * locals.var_dscr0__blk1339) - (assign48990_e62866 * locals.var_dscr0__blk1339_dn7)) / (locals.var_dscr0__blk1339 * locals.var_dscr0__blk1339))))), ((locals.var_nscr__blk1333_dn8 * assign48990_e62871) + (locals.var_nscr__blk1333 * (locals.var_qiscr0__blk1338_dn8 - ((((((locals.var_dscr0__blk1339_dn8 * locals.var_temp__blk936) + (locals.var_dscr0__blk1339 * locals.var_temp__blk936_dn8)) / (2.0 * assign48990_e62864)) * locals.var_dscr0__blk1339) - (assign48990_e62866 * locals.var_dscr0__blk1339_dn8)) / (locals.var_dscr0__blk1339 * locals.var_dscr0__blk1339))))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) && (locals.var_guard1466 == 0.0)) {
            let assign49000_e62885: f64 = (locals.var_nscr__blk1333 * 0.5);
            let assign49000_e62887: f64 = (assign49000_e62885 * locals.var_dscr0__blk1339);
            let assign49000_e62891: f64 = (0.25 * locals.var_temp__blk936);
            let assign49000_e62893: f64 = (assign49000_e62891 * locals.var_temp__blk936);
            let assign49000_e62894: f64 = (1.0 + assign49000_e62893);
            let assign49000_e62895: f64 = (assign49000_e62887 * assign49000_e62894);
            (locals.var_qiscr__blk1340, locals.var_qiscr__blk1340_dn5, locals.var_qiscr__blk1340_dn6, locals.var_qiscr__blk1340_dn7, locals.var_qiscr__blk1340_dn8, ) = (assign49000_e62895, (((((locals.var_nscr__blk1333_dn5 * 0.5) * locals.var_dscr0__blk1339) + (assign49000_e62885 * locals.var_dscr0__blk1339_dn5)) * assign49000_e62894) + (assign49000_e62887 * (((0.25 * locals.var_temp__blk936_dn5) * locals.var_temp__blk936) + (assign49000_e62891 * locals.var_temp__blk936_dn5)))), (((((locals.var_nscr__blk1333_dn6 * 0.5) * locals.var_dscr0__blk1339) + (assign49000_e62885 * locals.var_dscr0__blk1339_dn6)) * assign49000_e62894) + (assign49000_e62887 * (((0.25 * locals.var_temp__blk936_dn6) * locals.var_temp__blk936) + (assign49000_e62891 * locals.var_temp__blk936_dn6)))), (((((locals.var_nscr__blk1333_dn7 * 0.5) * locals.var_dscr0__blk1339) + (assign49000_e62885 * locals.var_dscr0__blk1339_dn7)) * assign49000_e62894) + (assign49000_e62887 * (((0.25 * locals.var_temp__blk936_dn7) * locals.var_temp__blk936) + (assign49000_e62891 * locals.var_temp__blk936_dn7)))), (((((locals.var_nscr__blk1333_dn8 * 0.5) * locals.var_dscr0__blk1339) + (assign49000_e62885 * locals.var_dscr0__blk1339_dn8)) * assign49000_e62894) + (assign49000_e62887 * (((0.25 * locals.var_temp__blk936_dn8) * locals.var_temp__blk936) + (assign49000_e62891 * locals.var_temp__blk936_dn8)))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
            let assign49010_e62906: f64 = (locals.var_xg__blk1326 - locals.var_qiscr__blk1340);
            let assign49010_e62908: f64 = (assign49010_e62906 + 2.0);
            let assign49010_e62911: f64 = (locals.var_xg__blk1326 - locals.var_qiscr__blk1340);
            let assign49010_e62913: f64 = (assign49010_e62911 - 2.0);
            let assign49010_e62916: f64 = (locals.var_xg__blk1326 - locals.var_qiscr__blk1340);
            let assign49010_e62918: f64 = (assign49010_e62916 - 2.0);
            let assign49010_e62919: f64 = (assign49010_e62913 * assign49010_e62918);
            let assign49010_e62921: f64 = (assign49010_e62919 + 1.0);
            let assign49010_e62922: f64 = (assign49010_e62921).sqrt();
            let assign49010_e62923: f64 = (assign49010_e62908 + assign49010_e62922);
            let assign49010_e62924: f64 = (0.5 * assign49010_e62923);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign49010_e62924, (0.5 * ((locals.var_xg__blk1326_dn5 - locals.var_qiscr__blk1340_dn5) + ((((locals.var_xg__blk1326_dn5 - locals.var_qiscr__blk1340_dn5) * assign49010_e62918) + (assign49010_e62913 * (locals.var_xg__blk1326_dn5 - locals.var_qiscr__blk1340_dn5))) / (2.0 * assign49010_e62922)))), (0.5 * ((locals.var_xg__blk1326_dn6 - locals.var_qiscr__blk1340_dn6) + ((((locals.var_xg__blk1326_dn6 - locals.var_qiscr__blk1340_dn6) * assign49010_e62918) + (assign49010_e62913 * (locals.var_xg__blk1326_dn6 - locals.var_qiscr__blk1340_dn6))) / (2.0 * assign49010_e62922)))), (0.5 * ((locals.var_xg__blk1326_dn7 - locals.var_qiscr__blk1340_dn7) + ((((locals.var_xg__blk1326_dn7 - locals.var_qiscr__blk1340_dn7) * assign49010_e62918) + (assign49010_e62913 * (locals.var_xg__blk1326_dn7 - locals.var_qiscr__blk1340_dn7))) / (2.0 * assign49010_e62922)))), (0.5 * ((locals.var_xg__blk1326_dn8 - locals.var_qiscr__blk1340_dn8) + ((((locals.var_xg__blk1326_dn8 - locals.var_qiscr__blk1340_dn8) * assign49010_e62918) + (assign49010_e62913 * (locals.var_xg__blk1326_dn8 - locals.var_qiscr__blk1340_dn8))) / (2.0 * assign49010_e62922)))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
            let assign49020_e62934: f64 = (0.5 * locals.var_gf2__blk1308);
            let assign49020_e62938: f64 = (4.0 / locals.var_gf2__blk1308);
            let assign49020_e62940: f64 = (assign49020_e62938 * locals.var_temp__blk936);
            let assign49020_e62941: f64 = (1.0 + assign49020_e62940);
            let assign49020_e62942: f64 = (assign49020_e62941).sqrt();
            let assign49020_e62944: f64 = (assign49020_e62942 - 1.0);
            let assign49020_e62945: f64 = (assign49020_e62934 * assign49020_e62944);
            (locals.var_qbscr__blk1341, locals.var_qbscr__blk1341_dn5, locals.var_qbscr__blk1341_dn6, locals.var_qbscr__blk1341_dn7, locals.var_qbscr__blk1341_dn8, ) = (assign49020_e62945, (((0.5 * locals.var_gf2__blk1308_dn5) * assign49020_e62944) + (assign49020_e62934 * ((((-((4.0 * locals.var_gf2__blk1308_dn5) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) * locals.var_temp__blk936) + (assign49020_e62938 * locals.var_temp__blk936_dn5)) / (2.0 * assign49020_e62942)))), (((0.5 * locals.var_gf2__blk1308_dn6) * assign49020_e62944) + (assign49020_e62934 * ((((-((4.0 * locals.var_gf2__blk1308_dn6) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) * locals.var_temp__blk936) + (assign49020_e62938 * locals.var_temp__blk936_dn6)) / (2.0 * assign49020_e62942)))), (((0.5 * locals.var_gf2__blk1308_dn7) * assign49020_e62944) + (assign49020_e62934 * ((((-((4.0 * locals.var_gf2__blk1308_dn7) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) * locals.var_temp__blk936) + (assign49020_e62938 * locals.var_temp__blk936_dn7)) / (2.0 * assign49020_e62942)))), (((0.5 * locals.var_gf2__blk1308_dn8) * assign49020_e62944) + (assign49020_e62934 * ((((-((4.0 * locals.var_gf2__blk1308_dn8) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308))) * locals.var_temp__blk936) + (assign49020_e62938 * locals.var_temp__blk936_dn8)) / (2.0 * assign49020_e62942)))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
            let assign49030_e62956: f64 = (locals.var_qbscr__blk1341 + locals.var_qiscr__blk1340);
            let assign49030_e62957: f64 = (locals.var_qbscr__blk1341 / assign49030_e62956);
            (locals.var_fscr__blk1342, locals.var_fscr__blk1342_dn5, locals.var_fscr__blk1342_dn6, locals.var_fscr__blk1342_dn7, locals.var_fscr__blk1342_dn8, ) = (assign49030_e62957, (((locals.var_qbscr__blk1341_dn5 * assign49030_e62956) - (locals.var_qbscr__blk1341 * (locals.var_qbscr__blk1341_dn5 + locals.var_qiscr__blk1340_dn5))) / (assign49030_e62956 * assign49030_e62956)), (((locals.var_qbscr__blk1341_dn6 * assign49030_e62956) - (locals.var_qbscr__blk1341 * (locals.var_qbscr__blk1341_dn6 + locals.var_qiscr__blk1340_dn6))) / (assign49030_e62956 * assign49030_e62956)), (((locals.var_qbscr__blk1341_dn7 * assign49030_e62956) - (locals.var_qbscr__blk1341 * (locals.var_qbscr__blk1341_dn7 + locals.var_qiscr__blk1340_dn7))) / (assign49030_e62956 * assign49030_e62956)), (((locals.var_qbscr__blk1341_dn8 * assign49030_e62956) - (locals.var_qbscr__blk1341 * (locals.var_qbscr__blk1341_dn8 + locals.var_qiscr__blk1340_dn8))) / (assign49030_e62956 * assign49030_e62956)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1464 != 0.0)) {
            let assign49040_e62968: f64 = (locals.var_fscr__blk1342 * locals.var_delxb__blk1330);
            let assign49040_e62969: f64 = (locals.var_xno_s__blk1331 - assign49040_e62968);
            (locals.var_xn_s__blk1332, locals.var_xn_s__blk1332_dn5, locals.var_xn_s__blk1332_dn6, locals.var_xn_s__blk1332_dn7, locals.var_xn_s__blk1332_dn8, ) = (assign49040_e62969, (locals.var_xno_s__blk1331_dn5 - ((locals.var_fscr__blk1342_dn5 * locals.var_delxb__blk1330) + (locals.var_fscr__blk1342 * locals.var_delxb__blk1330_dn5))), (locals.var_xno_s__blk1331_dn6 - ((locals.var_fscr__blk1342_dn6 * locals.var_delxb__blk1330) + (locals.var_fscr__blk1342 * locals.var_delxb__blk1330_dn6))), (locals.var_xno_s__blk1331_dn7 - ((locals.var_fscr__blk1342_dn7 * locals.var_delxb__blk1330) + (locals.var_fscr__blk1342 * locals.var_delxb__blk1330_dn7))), (locals.var_xno_s__blk1331_dn8 - ((locals.var_fscr__blk1342_dn8 * locals.var_delxb__blk1330) + (locals.var_fscr__blk1342 * locals.var_delxb__blk1330_dn8))), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign49050_e62978: f64 = (locals.var_gf__blk1307 * 0.7071067811865475);
            let assign49050_e62979: f64 = (1.0 + assign49050_e62978);
            (locals.var_xi__blk1343, locals.var_xi__blk1343_dn5, locals.var_xi__blk1343_dn6, locals.var_xi__blk1343_dn7, locals.var_xi__blk1343_dn8, ) = (assign49050_e62979, (locals.var_gf__blk1307_dn5 * 0.7071067811865475), (locals.var_gf__blk1307_dn6 * 0.7071067811865475), (locals.var_gf__blk1307_dn7 * 0.7071067811865475), (locals.var_gf__blk1307_dn8 * 0.7071067811865475), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign49060_e62987: f64 = (1e-5 * locals.var_xi__blk1343);
            locals.var_margin__blk1344 = assign49060_e62987;
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign49070_e62995: f64 = (1.0 / locals.var_xi__blk1343);
            (locals.var_inv_xi__blk1345, locals.var_inv_xi__blk1345_dn5, locals.var_inv_xi__blk1345_dn6, locals.var_inv_xi__blk1345_dn7, locals.var_inv_xi__blk1345_dn8, ) = (assign49070_e62995, (-(locals.var_xi__blk1343_dn5 / (locals.var_xi__blk1343 * locals.var_xi__blk1343))), (-(locals.var_xi__blk1343_dn6 / (locals.var_xi__blk1343 * locals.var_xi__blk1343))), (-(locals.var_xi__blk1343_dn7 / (locals.var_xi__blk1343 * locals.var_xi__blk1343))), (-(locals.var_xi__blk1343_dn8 / (locals.var_xi__blk1343 * locals.var_xi__blk1343))), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            (locals.var_sp_s_x1__blk1452, locals.var_sp_s_x1__blk1452_dn5, locals.var_sp_s_x1__blk1452_dn6, locals.var_sp_s_x1__blk1452_dn7, locals.var_sp_s_x1__blk1452_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_x_s__blk1346, locals.var_x_s__blk1346_dn5, locals.var_x_s__blk1346_dn6, locals.var_x_s__blk1346_dn7, locals.var_x_s__blk1346_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign49100_e63012: f64 = if locals.var_xn_s__blk1332 < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1467 = assign49100_e63012;

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1467 != 0.0)) {
            let assign49110_e63019: f64 = (-locals.var_xn_s__blk1332);
            let assign49110_e63020: f64 = (assign49110_e63019).exp();
            (locals.var_delta_ns__blk1347, locals.var_delta_ns__blk1347_dn5, locals.var_delta_ns__blk1347_dn6, locals.var_delta_ns__blk1347_dn7, locals.var_delta_ns__blk1347_dn8, ) = (assign49110_e63020, (assign49110_e63020 * (-locals.var_xn_s__blk1332_dn5)), (assign49110_e63020 * (-locals.var_xn_s__blk1332_dn6)), (assign49110_e63020 * (-locals.var_xn_s__blk1332_dn7)), (assign49110_e63020 * (-locals.var_xn_s__blk1332_dn8)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1467 == 0.0)) {
            let assign49120_e63033: f64 = (locals.var_xn_s__blk1332 - 460.51701859880916);
            let assign49120_e63038: f64 = (locals.var_xn_s__blk1332 - 460.51701859880916);
            let assign49120_e63042: f64 = (locals.var_xn_s__blk1332 - 460.51701859880916);
            let assign49120_e63044: f64 = (assign49120_e63042 * 0.3333333333333333);
            let assign49120_e63045: f64 = (1.0 + assign49120_e63044);
            let assign49120_e63046: f64 = (assign49120_e63038 * assign49120_e63045);
            let assign49120_e63047: f64 = (0.5 * assign49120_e63046);
            let assign49120_e63048: f64 = (1.0 + assign49120_e63047);
            let assign49120_e63049: f64 = (assign49120_e63033 * assign49120_e63048);
            let assign49120_e63050: f64 = (1.0 + assign49120_e63049);
            let assign49120_e63051: f64 = (1e-200 / assign49120_e63050);
            (locals.var_delta_ns__blk1347, locals.var_delta_ns__blk1347_dn5, locals.var_delta_ns__blk1347_dn6, locals.var_delta_ns__blk1347_dn7, locals.var_delta_ns__blk1347_dn8, ) = (assign49120_e63051, (-((1e-200 * ((locals.var_xn_s__blk1332_dn5 * assign49120_e63048) + (assign49120_e63033 * (0.5 * ((locals.var_xn_s__blk1332_dn5 * assign49120_e63045) + (assign49120_e63038 * (locals.var_xn_s__blk1332_dn5 * 0.3333333333333333))))))) / (assign49120_e63050 * assign49120_e63050))), (-((1e-200 * ((locals.var_xn_s__blk1332_dn6 * assign49120_e63048) + (assign49120_e63033 * (0.5 * ((locals.var_xn_s__blk1332_dn6 * assign49120_e63045) + (assign49120_e63038 * (locals.var_xn_s__blk1332_dn6 * 0.3333333333333333))))))) / (assign49120_e63050 * assign49120_e63050))), (-((1e-200 * ((locals.var_xn_s__blk1332_dn7 * assign49120_e63048) + (assign49120_e63033 * (0.5 * ((locals.var_xn_s__blk1332_dn7 * assign49120_e63045) + (assign49120_e63038 * (locals.var_xn_s__blk1332_dn7 * 0.3333333333333333))))))) / (assign49120_e63050 * assign49120_e63050))), (-((1e-200 * ((locals.var_xn_s__blk1332_dn8 * assign49120_e63048) + (assign49120_e63033 * (0.5 * ((locals.var_xn_s__blk1332_dn8 * assign49120_e63045) + (assign49120_e63038 * (locals.var_xn_s__blk1332_dn8 * 0.3333333333333333))))))) / (assign49120_e63050 * assign49120_e63050))), );
        }

        let assign49130_e63055: f64 = (locals.var_xg__blk1326).abs();
        let assign49130_e63057: f64 = if assign49130_e63055 <= locals.var_margin__blk1344 { 1.0 } else { 0.0 };
        locals.var_guard1468 = assign49130_e63057;

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 != 0.0)) {
            let assign49140_e63065: f64 = (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345);
            let assign49140_e63067: f64 = (assign49140_e63065 * 0.16666666666666666);
            let assign49140_e63069: f64 = (assign49140_e63067 * 0.7071067811865475);
            (locals.var_sp_s_temp1__blk1432, locals.var_sp_s_temp1__blk1432_dn5, locals.var_sp_s_temp1__blk1432_dn6, locals.var_sp_s_temp1__blk1432_dn7, locals.var_sp_s_temp1__blk1432_dn8, ) = (assign49140_e63069, ((((locals.var_inv_xi__blk1345_dn5 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn5)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1345_dn6 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1345_dn7 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1345_dn8 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn8)) * 0.16666666666666666) * 0.7071067811865475), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 != 0.0)) {
            let assign49150_e63079: f64 = (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345);
            let assign49150_e63084: f64 = (1.0 - locals.var_delta_ns__blk1347);
            let assign49150_e63085: f64 = (locals.var_xg__blk1326 * assign49150_e63084);
            let assign49150_e63087: f64 = (assign49150_e63085 * locals.var_gf__blk1307);
            let assign49150_e63089: f64 = (assign49150_e63087 * locals.var_sp_s_temp1__blk1432);
            let assign49150_e63090: f64 = (1.0 + assign49150_e63089);
            let assign49150_e63091: f64 = (assign49150_e63079 * assign49150_e63090);
            (locals.var_x_s__blk1346, locals.var_x_s__blk1346_dn5, locals.var_x_s__blk1346_dn6, locals.var_x_s__blk1346_dn7, locals.var_x_s__blk1346_dn8, ) = (assign49150_e63091, ((((locals.var_xg__blk1326_dn5 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn5)) * assign49150_e63090) + (assign49150_e63079 * ((((((locals.var_xg__blk1326_dn5 * assign49150_e63084) + (locals.var_xg__blk1326 * (-locals.var_delta_ns__blk1347_dn5))) * locals.var_gf__blk1307) + (assign49150_e63085 * locals.var_gf__blk1307_dn5)) * locals.var_sp_s_temp1__blk1432) + (assign49150_e63087 * locals.var_sp_s_temp1__blk1432_dn5)))), ((((locals.var_xg__blk1326_dn6 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn6)) * assign49150_e63090) + (assign49150_e63079 * ((((((locals.var_xg__blk1326_dn6 * assign49150_e63084) + (locals.var_xg__blk1326 * (-locals.var_delta_ns__blk1347_dn6))) * locals.var_gf__blk1307) + (assign49150_e63085 * locals.var_gf__blk1307_dn6)) * locals.var_sp_s_temp1__blk1432) + (assign49150_e63087 * locals.var_sp_s_temp1__blk1432_dn6)))), ((((locals.var_xg__blk1326_dn7 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn7)) * assign49150_e63090) + (assign49150_e63079 * ((((((locals.var_xg__blk1326_dn7 * assign49150_e63084) + (locals.var_xg__blk1326 * (-locals.var_delta_ns__blk1347_dn7))) * locals.var_gf__blk1307) + (assign49150_e63085 * locals.var_gf__blk1307_dn7)) * locals.var_sp_s_temp1__blk1432) + (assign49150_e63087 * locals.var_sp_s_temp1__blk1432_dn7)))), ((((locals.var_xg__blk1326_dn8 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn8)) * assign49150_e63090) + (assign49150_e63079 * ((((((locals.var_xg__blk1326_dn8 * assign49150_e63084) + (locals.var_xg__blk1326 * (-locals.var_delta_ns__blk1347_dn8))) * locals.var_gf__blk1307) + (assign49150_e63085 * locals.var_gf__blk1307_dn8)) * locals.var_sp_s_temp1__blk1432) + (assign49150_e63087 * locals.var_sp_s_temp1__blk1432_dn8)))), );
        }

        let assign49160_e63096: f64 = (-locals.var_margin__blk1344);
        let assign49160_e63097: f64 = if locals.var_xg__blk1326 < assign49160_e63096 { 1.0 } else { 0.0 };
        locals.var_guard1469 = assign49160_e63097;

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
            let assign49170_e63107: f64 = (-locals.var_xg__blk1326);
            (locals.var_sp_s_yg__blk1434, locals.var_sp_s_yg__blk1434_dn5, locals.var_sp_s_yg__blk1434_dn6, locals.var_sp_s_yg__blk1434_dn7, locals.var_sp_s_yg__blk1434_dn8, ) = (assign49170_e63107, (-locals.var_xg__blk1326_dn5), (-locals.var_xg__blk1326_dn6), (-locals.var_xg__blk1326_dn7), (-locals.var_xg__blk1326_dn8), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
            let assign49180_e63121: f64 = (locals.var_sp_s_yg__blk1434 * locals.var_inv_xi__blk1345);
            let assign49180_e63122: f64 = (1.25 * assign49180_e63121);
            (locals.var_sp_s_ysub__blk1435, locals.var_sp_s_ysub__blk1435_dn5, locals.var_sp_s_ysub__blk1435_dn6, locals.var_sp_s_ysub__blk1435_dn7, locals.var_sp_s_ysub__blk1435_dn8, ) = (assign49180_e63122, (1.25 * ((locals.var_sp_s_yg__blk1434_dn5 * locals.var_inv_xi__blk1345) + (locals.var_sp_s_yg__blk1434 * locals.var_inv_xi__blk1345_dn5))), (1.25 * ((locals.var_sp_s_yg__blk1434_dn6 * locals.var_inv_xi__blk1345) + (locals.var_sp_s_yg__blk1434 * locals.var_inv_xi__blk1345_dn6))), (1.25 * ((locals.var_sp_s_yg__blk1434_dn7 * locals.var_inv_xi__blk1345) + (locals.var_sp_s_yg__blk1434 * locals.var_inv_xi__blk1345_dn7))), (1.25 * ((locals.var_sp_s_yg__blk1434_dn8 * locals.var_inv_xi__blk1345) + (locals.var_sp_s_yg__blk1434 * locals.var_inv_xi__blk1345_dn8))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
            let assign49190_e63136: f64 = (locals.var_sp_s_ysub__blk1435 + 10.0);
            let assign49190_e63139: f64 = (locals.var_sp_s_ysub__blk1435 - 6.0);
            let assign49190_e63142: f64 = (locals.var_sp_s_ysub__blk1435 - 6.0);
            let assign49190_e63143: f64 = (assign49190_e63139 * assign49190_e63142);
            let assign49190_e63145: f64 = (assign49190_e63143 + 64.0);
            let assign49190_e63146: f64 = (assign49190_e63145).sqrt();
            let assign49190_e63147: f64 = (assign49190_e63136 - assign49190_e63146);
            let assign49190_e63148: f64 = (0.5 * assign49190_e63147);
            (locals.var_sp_s_eta__blk1436, locals.var_sp_s_eta__blk1436_dn5, locals.var_sp_s_eta__blk1436_dn6, locals.var_sp_s_eta__blk1436_dn7, locals.var_sp_s_eta__blk1436_dn8, ) = (assign49190_e63148, (0.5 * (locals.var_sp_s_ysub__blk1435_dn5 - (((locals.var_sp_s_ysub__blk1435_dn5 * assign49190_e63142) + (assign49190_e63139 * locals.var_sp_s_ysub__blk1435_dn5)) / (2.0 * assign49190_e63146)))), (0.5 * (locals.var_sp_s_ysub__blk1435_dn6 - (((locals.var_sp_s_ysub__blk1435_dn6 * assign49190_e63142) + (assign49190_e63139 * locals.var_sp_s_ysub__blk1435_dn6)) / (2.0 * assign49190_e63146)))), (0.5 * (locals.var_sp_s_ysub__blk1435_dn7 - (((locals.var_sp_s_ysub__blk1435_dn7 * assign49190_e63142) + (assign49190_e63139 * locals.var_sp_s_ysub__blk1435_dn7)) / (2.0 * assign49190_e63146)))), (0.5 * (locals.var_sp_s_ysub__blk1435_dn8 - (((locals.var_sp_s_ysub__blk1435_dn8 * assign49190_e63142) + (assign49190_e63139 * locals.var_sp_s_ysub__blk1435_dn8)) / (2.0 * assign49190_e63146)))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
            let assign49200_e63161: f64 = (locals.var_sp_s_yg__blk1434 - locals.var_sp_s_eta__blk1436);
            (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8, ) = (assign49200_e63161, (locals.var_sp_s_yg__blk1434_dn5 - locals.var_sp_s_eta__blk1436_dn5), (locals.var_sp_s_yg__blk1434_dn6 - locals.var_sp_s_eta__blk1436_dn6), (locals.var_sp_s_yg__blk1434_dn7 - locals.var_sp_s_eta__blk1436_dn7), (locals.var_sp_s_yg__blk1434_dn8 - locals.var_sp_s_eta__blk1436_dn8), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
            let assign49210_e63174: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
            let assign49210_e63178: f64 = (locals.var_sp_s_eta__blk1436 + 1.0);
            let assign49210_e63179: f64 = (locals.var_gf2__blk1308 * assign49210_e63178);
            let assign49210_e63180: f64 = (assign49210_e63174 + assign49210_e63179);
            (locals.var_sp_s_a__blk1437, locals.var_sp_s_a__blk1437_dn5, locals.var_sp_s_a__blk1437_dn6, locals.var_sp_s_a__blk1437_dn7, locals.var_sp_s_a__blk1437_dn8, ) = (assign49210_e63180, (((locals.var_sp_s_temp__blk1431_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn5)) + ((locals.var_gf2__blk1308_dn5 * assign49210_e63178) + (locals.var_gf2__blk1308 * locals.var_sp_s_eta__blk1436_dn5))), (((locals.var_sp_s_temp__blk1431_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn6)) + ((locals.var_gf2__blk1308_dn6 * assign49210_e63178) + (locals.var_gf2__blk1308 * locals.var_sp_s_eta__blk1436_dn6))), (((locals.var_sp_s_temp__blk1431_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn7)) + ((locals.var_gf2__blk1308_dn7 * assign49210_e63178) + (locals.var_gf2__blk1308 * locals.var_sp_s_eta__blk1436_dn7))), (((locals.var_sp_s_temp__blk1431_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn8)) + ((locals.var_gf2__blk1308_dn8 * assign49210_e63178) + (locals.var_gf2__blk1308 * locals.var_sp_s_eta__blk1436_dn8))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
            let assign49220_e63193: f64 = (2.0 * locals.var_sp_s_temp__blk1431);
            let assign49220_e63195: f64 = (assign49220_e63193 - locals.var_gf2__blk1308);
            (locals.var_sp_s_c__blk1438, locals.var_sp_s_c__blk1438_dn5, locals.var_sp_s_c__blk1438_dn6, locals.var_sp_s_c__blk1438_dn7, locals.var_sp_s_c__blk1438_dn8, ) = (assign49220_e63195, ((2.0 * locals.var_sp_s_temp__blk1431_dn5) - locals.var_gf2__blk1308_dn5), ((2.0 * locals.var_sp_s_temp__blk1431_dn6) - locals.var_gf2__blk1308_dn6), ((2.0 * locals.var_sp_s_temp__blk1431_dn7) - locals.var_gf2__blk1308_dn7), ((2.0 * locals.var_sp_s_temp__blk1431_dn8) - locals.var_gf2__blk1308_dn8), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
            let assign49230_e63207: f64 = (-locals.var_sp_s_eta__blk1436);
            let assign49230_e63210: f64 = (locals.var_sp_s_a__blk1437 * locals.var_inv_gf2__blk1324);
            let assign49230_e63211: f64 = (assign49230_e63210).ln();
            let assign49230_e63212: f64 = (assign49230_e63207 + assign49230_e63211);
            (locals.var_sp_s_tau__blk1439, locals.var_sp_s_tau__blk1439_dn5, locals.var_sp_s_tau__blk1439_dn6, locals.var_sp_s_tau__blk1439_dn7, locals.var_sp_s_tau__blk1439_dn8, ) = (assign49230_e63212, ((-locals.var_sp_s_eta__blk1436_dn5) + (((locals.var_sp_s_a__blk1437_dn5 * locals.var_inv_gf2__blk1324) + (locals.var_sp_s_a__blk1437 * locals.var_inv_gf2__blk1324_dn5)) / assign49230_e63210)), ((-locals.var_sp_s_eta__blk1436_dn6) + (((locals.var_sp_s_a__blk1437_dn6 * locals.var_inv_gf2__blk1324) + (locals.var_sp_s_a__blk1437 * locals.var_inv_gf2__blk1324_dn6)) / assign49230_e63210)), ((-locals.var_sp_s_eta__blk1436_dn7) + (((locals.var_sp_s_a__blk1437_dn7 * locals.var_inv_gf2__blk1324) + (locals.var_sp_s_a__blk1437 * locals.var_inv_gf2__blk1324_dn7)) / assign49230_e63210)), ((-locals.var_sp_s_eta__blk1436_dn8) + (((locals.var_sp_s_a__blk1437_dn8 * locals.var_inv_gf2__blk1324) + (locals.var_sp_s_a__blk1437 * locals.var_inv_gf2__blk1324_dn8)) / assign49230_e63210)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
            let assign49240_e63225: f64 = (locals.var_sp_s_a__blk1437 + locals.var_sp_s_c__blk1438);
            (locals.var_nu, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, ) = (assign49240_e63225, (locals.var_sp_s_a__blk1437_dn5 + locals.var_sp_s_c__blk1438_dn5), (locals.var_sp_s_a__blk1437_dn6 + locals.var_sp_s_c__blk1438_dn6), (locals.var_sp_s_a__blk1437_dn7 + locals.var_sp_s_c__blk1438_dn7), (locals.var_sp_s_a__blk1437_dn8 + locals.var_sp_s_c__blk1438_dn8), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
            let assign49250_e63238: f64 = (locals.var_nu * locals.var_nu);
            let assign49250_e63243: f64 = (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438);
            let assign49250_e63244: f64 = (0.5 * assign49250_e63243);
            let assign49250_e63246: f64 = (assign49250_e63244 - locals.var_sp_s_a__blk1437);
            let assign49250_e63247: f64 = (locals.var_sp_s_tau__blk1439 * assign49250_e63246);
            let assign49250_e63248: f64 = (assign49250_e63238 + assign49250_e63247);
            (locals.var_mutau, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, ) = (assign49250_e63248, (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_sp_s_tau__blk1439_dn5 * assign49250_e63246) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn5 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn5))) - locals.var_sp_s_a__blk1437_dn5)))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau__blk1439_dn6 * assign49250_e63246) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn6 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn6))) - locals.var_sp_s_a__blk1437_dn6)))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau__blk1439_dn7 * assign49250_e63246) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn7 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn7))) - locals.var_sp_s_a__blk1437_dn7)))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau__blk1439_dn8 * assign49250_e63246) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn8 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn8))) - locals.var_sp_s_a__blk1437_dn8)))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
            let assign49260_e63262: f64 = (locals.var_sp_s_a__blk1437 * locals.var_nu);
            let assign49260_e63264: f64 = (assign49260_e63262 * locals.var_sp_s_tau__blk1439);
            let assign49260_e63268: f64 = (locals.var_nu / locals.var_mutau);
            let assign49260_e63270: f64 = (assign49260_e63268 * locals.var_sp_s_tau__blk1439);
            let assign49260_e63272: f64 = (assign49260_e63270 * locals.var_sp_s_tau__blk1439);
            let assign49260_e63274: f64 = (assign49260_e63272 * locals.var_sp_s_c__blk1438);
            let assign49260_e63277: f64 = (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438);
            let assign49260_e63279: f64 = (assign49260_e63277 * 0.3333333333333333);
            let assign49260_e63281: f64 = (assign49260_e63279 - locals.var_sp_s_a__blk1437);
            let assign49260_e63282: f64 = (assign49260_e63274 * assign49260_e63281);
            let assign49260_e63283: f64 = (locals.var_mutau + assign49260_e63282);
            let assign49260_e63284: f64 = (assign49260_e63264 / assign49260_e63283);
            let assign49260_e63285: f64 = (locals.var_sp_s_eta__blk1436 + assign49260_e63284);
            (locals.var_sp_s_y0__blk1440, locals.var_sp_s_y0__blk1440_dn5, locals.var_sp_s_y0__blk1440_dn6, locals.var_sp_s_y0__blk1440_dn7, locals.var_sp_s_y0__blk1440_dn8, ) = (assign49260_e63285, (locals.var_sp_s_eta__blk1436_dn5 + (((((((locals.var_sp_s_a__blk1437_dn5 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn5)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63262 * locals.var_sp_s_tau__blk1439_dn5)) * assign49260_e63283) - (assign49260_e63264 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63268 * locals.var_sp_s_tau__blk1439_dn5)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63270 * locals.var_sp_s_tau__blk1439_dn5)) * locals.var_sp_s_c__blk1438) + (assign49260_e63272 * locals.var_sp_s_c__blk1438_dn5)) * assign49260_e63281) + (assign49260_e63274 * ((((locals.var_sp_s_c__blk1438_dn5 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn5)) * 0.3333333333333333) - locals.var_sp_s_a__blk1437_dn5)))))) / (assign49260_e63283 * assign49260_e63283))), (locals.var_sp_s_eta__blk1436_dn6 + (((((((locals.var_sp_s_a__blk1437_dn6 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn6)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63262 * locals.var_sp_s_tau__blk1439_dn6)) * assign49260_e63283) - (assign49260_e63264 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63268 * locals.var_sp_s_tau__blk1439_dn6)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63270 * locals.var_sp_s_tau__blk1439_dn6)) * locals.var_sp_s_c__blk1438) + (assign49260_e63272 * locals.var_sp_s_c__blk1438_dn6)) * assign49260_e63281) + (assign49260_e63274 * ((((locals.var_sp_s_c__blk1438_dn6 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn6)) * 0.3333333333333333) - locals.var_sp_s_a__blk1437_dn6)))))) / (assign49260_e63283 * assign49260_e63283))), (locals.var_sp_s_eta__blk1436_dn7 + (((((((locals.var_sp_s_a__blk1437_dn7 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn7)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63262 * locals.var_sp_s_tau__blk1439_dn7)) * assign49260_e63283) - (assign49260_e63264 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63268 * locals.var_sp_s_tau__blk1439_dn7)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63270 * locals.var_sp_s_tau__blk1439_dn7)) * locals.var_sp_s_c__blk1438) + (assign49260_e63272 * locals.var_sp_s_c__blk1438_dn7)) * assign49260_e63281) + (assign49260_e63274 * ((((locals.var_sp_s_c__blk1438_dn7 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn7)) * 0.3333333333333333) - locals.var_sp_s_a__blk1437_dn7)))))) / (assign49260_e63283 * assign49260_e63283))), (locals.var_sp_s_eta__blk1436_dn8 + (((((((locals.var_sp_s_a__blk1437_dn8 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn8)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63262 * locals.var_sp_s_tau__blk1439_dn8)) * assign49260_e63283) - (assign49260_e63264 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63268 * locals.var_sp_s_tau__blk1439_dn8)) * locals.var_sp_s_tau__blk1439) + (assign49260_e63270 * locals.var_sp_s_tau__blk1439_dn8)) * locals.var_sp_s_c__blk1438) + (assign49260_e63272 * locals.var_sp_s_c__blk1438_dn8)) * assign49260_e63281) + (assign49260_e63274 * ((((locals.var_sp_s_c__blk1438_dn8 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn8)) * 0.3333333333333333) - locals.var_sp_s_a__blk1437_dn8)))))) / (assign49260_e63283 * assign49260_e63283))), );
        }

        let assign49270_e63290: f64 = if locals.var_sp_s_y0__blk1440 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1470 = assign49270_e63290;

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) && (locals.var_guard1470 != 0.0)) {
            let assign49280_e63302: f64 = (locals.var_sp_s_y0__blk1440).exp();
            (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8, ) = (assign49280_e63302, (assign49280_e63302 * locals.var_sp_s_y0__blk1440_dn5), (assign49280_e63302 * locals.var_sp_s_y0__blk1440_dn6), (assign49280_e63302 * locals.var_sp_s_y0__blk1440_dn7), (assign49280_e63302 * locals.var_sp_s_y0__blk1440_dn8), );
        }

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) && (locals.var_guard1470 == 0.0)) {
            let assign49290_e63320: f64 = (locals.var_sp_s_y0__blk1440 - 230.25850929940458);
            let assign49290_e63325: f64 = (locals.var_sp_s_y0__blk1440 - 230.25850929940458);
            let assign49290_e63329: f64 = (locals.var_sp_s_y0__blk1440 - 230.25850929940458);
            let assign49290_e63331: f64 = (assign49290_e63329 * 0.3333333333333333);
            let assign49290_e63332: f64 = (1.0 + assign49290_e63331);
            let assign49290_e63333: f64 = (assign49290_e63325 * assign49290_e63332);
            let assign49290_e63334: f64 = (0.5 * assign49290_e63333);
            let assign49290_e63335: f64 = (1.0 + assign49290_e63334);
            let assign49290_e63336: f64 = (assign49290_e63320 * assign49290_e63335);
            let assign49290_e63337: f64 = (1.0 + assign49290_e63336);
            let assign49290_e63338: f64 = (1e100 * assign49290_e63337);
            (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8, ) = (assign49290_e63338, (1e100 * ((locals.var_sp_s_y0__blk1440_dn5 * assign49290_e63335) + (assign49290_e63320 * (0.5 * ((locals.var_sp_s_y0__blk1440_dn5 * assign49290_e63332) + (assign49290_e63325 * (locals.var_sp_s_y0__blk1440_dn5 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0__blk1440_dn6 * assign49290_e63335) + (assign49290_e63320 * (0.5 * ((locals.var_sp_s_y0__blk1440_dn6 * assign49290_e63332) + (assign49290_e63325 * (locals.var_sp_s_y0__blk1440_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0__blk1440_dn7 * assign49290_e63335) + (assign49290_e63320 * (0.5 * ((locals.var_sp_s_y0__blk1440_dn7 * assign49290_e63332) + (assign49290_e63325 * (locals.var_sp_s_y0__blk1440_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0__blk1440_dn8 * assign49290_e63335) + (assign49290_e63320 * (0.5 * ((locals.var_sp_s_y0__blk1440_dn8 * assign49290_e63332) + (assign49290_e63325 * (locals.var_sp_s_y0__blk1440_dn8 * 0.3333333333333333))))))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
            let assign49300_e63351: f64 = (1.0 / locals.var_sp_s_delta0__blk1441);
            (locals.var_sp_s_delta1__blk1442, locals.var_sp_s_delta1__blk1442_dn5, locals.var_sp_s_delta1__blk1442_dn6, locals.var_sp_s_delta1__blk1442_dn7, locals.var_sp_s_delta1__blk1442_dn8, ) = (assign49300_e63351, (-(locals.var_sp_s_delta0__blk1441_dn5 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn6 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn7 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn8 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
            let assign49310_e63366: f64 = (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440);
            let assign49310_e63367: f64 = (2.0 + assign49310_e63366);
            let assign49310_e63368: f64 = (1.0 / assign49310_e63367);
            (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8, ) = (assign49310_e63368, (-(((locals.var_sp_s_y0__blk1440_dn5 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn5)) / (assign49310_e63367 * assign49310_e63367))), (-(((locals.var_sp_s_y0__blk1440_dn6 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn6)) / (assign49310_e63367 * assign49310_e63367))), (-(((locals.var_sp_s_y0__blk1440_dn7 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn7)) / (assign49310_e63367 * assign49310_e63367))), (-(((locals.var_sp_s_y0__blk1440_dn8 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn8)) / (assign49310_e63367 * assign49310_e63367))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
            let assign49320_e63381: f64 = (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440);
            let assign49320_e63383: f64 = (assign49320_e63381 * locals.var_sp_s_temp__blk1431);
            (locals.var_sp_s_xi0__blk1443, locals.var_sp_s_xi0__blk1443_dn5, locals.var_sp_s_xi0__blk1443_dn6, locals.var_sp_s_xi0__blk1443_dn7, locals.var_sp_s_xi0__blk1443_dn8, ) = (assign49320_e63383, ((((locals.var_sp_s_y0__blk1440_dn5 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49320_e63381 * locals.var_sp_s_temp__blk1431_dn5)), ((((locals.var_sp_s_y0__blk1440_dn6 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49320_e63381 * locals.var_sp_s_temp__blk1431_dn6)), ((((locals.var_sp_s_y0__blk1440_dn7 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49320_e63381 * locals.var_sp_s_temp__blk1431_dn7)), ((((locals.var_sp_s_y0__blk1440_dn8 * locals.var_sp_s_y0__blk1440) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_y0__blk1440_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49320_e63381 * locals.var_sp_s_temp__blk1431_dn8)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
            let assign49330_e63397: f64 = (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_temp__blk1431);
            let assign49330_e63399: f64 = (assign49330_e63397 * locals.var_sp_s_temp__blk1431);
            let assign49330_e63400: f64 = (4.0 * assign49330_e63399);
            (locals.var_sp_s_xi1__blk1444, locals.var_sp_s_xi1__blk1444_dn5, locals.var_sp_s_xi1__blk1444_dn6, locals.var_sp_s_xi1__blk1444_dn7, locals.var_sp_s_xi1__blk1444_dn8, ) = (assign49330_e63400, (4.0 * ((((locals.var_sp_s_y0__blk1440_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_temp__blk1431_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49330_e63397 * locals.var_sp_s_temp__blk1431_dn5))), (4.0 * ((((locals.var_sp_s_y0__blk1440_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_temp__blk1431_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49330_e63397 * locals.var_sp_s_temp__blk1431_dn6))), (4.0 * ((((locals.var_sp_s_y0__blk1440_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_temp__blk1431_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49330_e63397 * locals.var_sp_s_temp__blk1431_dn7))), (4.0 * ((((locals.var_sp_s_y0__blk1440_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_y0__blk1440 * locals.var_sp_s_temp__blk1431_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49330_e63397 * locals.var_sp_s_temp__blk1431_dn8))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
            let assign49340_e63413: f64 = (8.0 * locals.var_sp_s_temp__blk1431);
            let assign49340_e63416: f64 = (12.0 * locals.var_sp_s_xi0__blk1443);
            let assign49340_e63417: f64 = (assign49340_e63413 - assign49340_e63416);
            let assign49340_e63419: f64 = (assign49340_e63417 * locals.var_sp_s_temp__blk1431);
            let assign49340_e63421: f64 = (assign49340_e63419 * locals.var_sp_s_temp__blk1431);
            (locals.var_sp_s_xi2__blk1445, locals.var_sp_s_xi2__blk1445_dn5, locals.var_sp_s_xi2__blk1445_dn6, locals.var_sp_s_xi2__blk1445_dn7, locals.var_sp_s_xi2__blk1445_dn8, ) = (assign49340_e63421, ((((((8.0 * locals.var_sp_s_temp__blk1431_dn5) - (12.0 * locals.var_sp_s_xi0__blk1443_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63417 * locals.var_sp_s_temp__blk1431_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63419 * locals.var_sp_s_temp__blk1431_dn5)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn6) - (12.0 * locals.var_sp_s_xi0__blk1443_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63417 * locals.var_sp_s_temp__blk1431_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63419 * locals.var_sp_s_temp__blk1431_dn6)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn7) - (12.0 * locals.var_sp_s_xi0__blk1443_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63417 * locals.var_sp_s_temp__blk1431_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63419 * locals.var_sp_s_temp__blk1431_dn7)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn8) - (12.0 * locals.var_sp_s_xi0__blk1443_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63417 * locals.var_sp_s_temp__blk1431_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49340_e63419 * locals.var_sp_s_temp__blk1431_dn8)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
            let assign49350_e63434: f64 = (locals.var_sp_s_yg__blk1434 - locals.var_sp_s_y0__blk1440);
            (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8, ) = (assign49350_e63434, (locals.var_sp_s_yg__blk1434_dn5 - locals.var_sp_s_y0__blk1440_dn5), (locals.var_sp_s_yg__blk1434_dn6 - locals.var_sp_s_y0__blk1440_dn6), (locals.var_sp_s_yg__blk1434_dn7 - locals.var_sp_s_y0__blk1440_dn7), (locals.var_sp_s_yg__blk1434_dn8 - locals.var_sp_s_y0__blk1440_dn8), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
            let assign49360_e63447: f64 = (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta1__blk1442);
            (locals.var_sp_s_temp1__blk1432, locals.var_sp_s_temp1__blk1432_dn5, locals.var_sp_s_temp1__blk1432_dn6, locals.var_sp_s_temp1__blk1432_dn7, locals.var_sp_s_temp1__blk1432_dn8, ) = (assign49360_e63447, ((locals.var_delta_ns__blk1347_dn5 * locals.var_sp_s_delta1__blk1442) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta1__blk1442_dn5)), ((locals.var_delta_ns__blk1347_dn6 * locals.var_sp_s_delta1__blk1442) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta1__blk1442_dn6)), ((locals.var_delta_ns__blk1347_dn7 * locals.var_sp_s_delta1__blk1442) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta1__blk1442_dn7)), ((locals.var_delta_ns__blk1347_dn8 * locals.var_sp_s_delta1__blk1442) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta1__blk1442_dn8)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
            let assign49370_e63460: f64 = (2.0 * locals.var_sp_s_temp__blk1431);
            let assign49370_e63464: f64 = (locals.var_sp_s_delta0__blk1441 - 1.0);
            let assign49370_e63466: f64 = (assign49370_e63464 - locals.var_sp_s_temp1__blk1432);
            let assign49370_e63470: f64 = (1.0 - locals.var_sp_s_xi1__blk1444);
            let assign49370_e63471: f64 = (locals.var_delta_ns__blk1347 * assign49370_e63470);
            let assign49370_e63472: f64 = (assign49370_e63466 + assign49370_e63471);
            let assign49370_e63473: f64 = (locals.var_gf2__blk1308 * assign49370_e63472);
            let assign49370_e63474: f64 = (assign49370_e63460 + assign49370_e63473);
            (locals.var_sp_s_pc__blk1446, locals.var_sp_s_pc__blk1446_dn5, locals.var_sp_s_pc__blk1446_dn6, locals.var_sp_s_pc__blk1446_dn7, locals.var_sp_s_pc__blk1446_dn8, ) = (assign49370_e63474, ((2.0 * locals.var_sp_s_temp__blk1431_dn5) + ((locals.var_gf2__blk1308_dn5 * assign49370_e63472) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn5 - locals.var_sp_s_temp1__blk1432_dn5) + ((locals.var_delta_ns__blk1347_dn5 * assign49370_e63470) + (locals.var_delta_ns__blk1347 * (-locals.var_sp_s_xi1__blk1444_dn5))))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn6) + ((locals.var_gf2__blk1308_dn6 * assign49370_e63472) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn6 - locals.var_sp_s_temp1__blk1432_dn6) + ((locals.var_delta_ns__blk1347_dn6 * assign49370_e63470) + (locals.var_delta_ns__blk1347 * (-locals.var_sp_s_xi1__blk1444_dn6))))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn7) + ((locals.var_gf2__blk1308_dn7 * assign49370_e63472) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn7 - locals.var_sp_s_temp1__blk1432_dn7) + ((locals.var_delta_ns__blk1347_dn7 * assign49370_e63470) + (locals.var_delta_ns__blk1347 * (-locals.var_sp_s_xi1__blk1444_dn7))))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn8) + ((locals.var_gf2__blk1308_dn8 * assign49370_e63472) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn8 - locals.var_sp_s_temp1__blk1432_dn8) + ((locals.var_delta_ns__blk1347_dn8 * assign49370_e63470) + (locals.var_delta_ns__blk1347 * (-locals.var_sp_s_xi1__blk1444_dn8))))))), );
        }

    }

    pub(super) fn stamp_transient_block_21(
        locals: &mut StampLocals,
    ) {
        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
            let assign49380_e63487: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
            let assign49380_e63491: f64 = (locals.var_sp_s_delta0__blk1441 - locals.var_sp_s_y0__blk1440);
            let assign49380_e63493: f64 = (assign49380_e63491 - 1.0);
            let assign49380_e63495: f64 = (assign49380_e63493 + locals.var_sp_s_temp1__blk1432);
            let assign49380_e63499: f64 = (locals.var_sp_s_y0__blk1440 - 1.0);
            let assign49380_e63501: f64 = (assign49380_e63499 - locals.var_sp_s_xi0__blk1443);
            let assign49380_e63502: f64 = (locals.var_delta_ns__blk1347 * assign49380_e63501);
            let assign49380_e63503: f64 = (assign49380_e63495 + assign49380_e63502);
            let assign49380_e63504: f64 = (locals.var_gf2__blk1308 * assign49380_e63503);
            let assign49380_e63505: f64 = (assign49380_e63487 - assign49380_e63504);
            (locals.var_sp_s_qc__blk1447, locals.var_sp_s_qc__blk1447_dn5, locals.var_sp_s_qc__blk1447_dn6, locals.var_sp_s_qc__blk1447_dn7, locals.var_sp_s_qc__blk1447_dn8, ) = (assign49380_e63505, (((locals.var_sp_s_temp__blk1431_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn5)) - ((locals.var_gf2__blk1308_dn5 * assign49380_e63503) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta0__blk1441_dn5 - locals.var_sp_s_y0__blk1440_dn5) + locals.var_sp_s_temp1__blk1432_dn5) + ((locals.var_delta_ns__blk1347_dn5 * assign49380_e63501) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_y0__blk1440_dn5 - locals.var_sp_s_xi0__blk1443_dn5))))))), (((locals.var_sp_s_temp__blk1431_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn6)) - ((locals.var_gf2__blk1308_dn6 * assign49380_e63503) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta0__blk1441_dn6 - locals.var_sp_s_y0__blk1440_dn6) + locals.var_sp_s_temp1__blk1432_dn6) + ((locals.var_delta_ns__blk1347_dn6 * assign49380_e63501) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_y0__blk1440_dn6 - locals.var_sp_s_xi0__blk1443_dn6))))))), (((locals.var_sp_s_temp__blk1431_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn7)) - ((locals.var_gf2__blk1308_dn7 * assign49380_e63503) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta0__blk1441_dn7 - locals.var_sp_s_y0__blk1440_dn7) + locals.var_sp_s_temp1__blk1432_dn7) + ((locals.var_delta_ns__blk1347_dn7 * assign49380_e63501) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_y0__blk1440_dn7 - locals.var_sp_s_xi0__blk1443_dn7))))))), (((locals.var_sp_s_temp__blk1431_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn8)) - ((locals.var_gf2__blk1308_dn8 * assign49380_e63503) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta0__blk1441_dn8 - locals.var_sp_s_y0__blk1440_dn8) + locals.var_sp_s_temp1__blk1432_dn8) + ((locals.var_delta_ns__blk1347_dn8 * assign49380_e63501) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_y0__blk1440_dn8 - locals.var_sp_s_xi0__blk1443_dn8))))))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
            let assign49390_e63520: f64 = (locals.var_sp_s_delta0__blk1441 + locals.var_sp_s_temp1__blk1432);
            let assign49390_e63523: f64 = (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445);
            let assign49390_e63524: f64 = (assign49390_e63520 - assign49390_e63523);
            let assign49390_e63525: f64 = (locals.var_gf2__blk1308 * assign49390_e63524);
            let assign49390_e63526: f64 = (2.0 - assign49390_e63525);
            (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8, ) = (assign49390_e63526, (-((locals.var_gf2__blk1308_dn5 * assign49390_e63524) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn5 + locals.var_sp_s_temp1__blk1432_dn5) - ((locals.var_delta_ns__blk1347_dn5 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn5)))))), (-((locals.var_gf2__blk1308_dn6 * assign49390_e63524) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn6 + locals.var_sp_s_temp1__blk1432_dn6) - ((locals.var_delta_ns__blk1347_dn6 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn6)))))), (-((locals.var_gf2__blk1308_dn7 * assign49390_e63524) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn7 + locals.var_sp_s_temp1__blk1432_dn7) - ((locals.var_delta_ns__blk1347_dn7 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn7)))))), (-((locals.var_gf2__blk1308_dn8 * assign49390_e63524) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta0__blk1441_dn8 + locals.var_sp_s_temp1__blk1432_dn8) - ((locals.var_delta_ns__blk1347_dn8 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn8)))))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
            let assign49400_e63539: f64 = (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446);
            let assign49400_e63543: f64 = (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431);
            let assign49400_e63544: f64 = (2.0 * assign49400_e63543);
            let assign49400_e63545: f64 = (assign49400_e63539 - assign49400_e63544);
            (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8, ) = (assign49400_e63545, (((locals.var_sp_s_pc__blk1446_dn5 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn5)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn5)))), (((locals.var_sp_s_pc__blk1446_dn6 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn6)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn6)))), (((locals.var_sp_s_pc__blk1446_dn7 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn7)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn7)))), (((locals.var_sp_s_pc__blk1446_dn8 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn8)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn8)))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 != 0.0)) {
            let assign49410_e63557: f64 = (-locals.var_sp_s_y0__blk1440);
            let assign49410_e63562: f64 = (locals.var_sp_s_temp__blk1431).sqrt();
            let assign49410_e63563: f64 = (locals.var_sp_s_pc__blk1446 + assign49410_e63562);
            let assign49410_e63564: f64 = (locals.var_sp_s_qc__blk1447 / assign49410_e63563);
            let assign49410_e63565: f64 = (2.0 * assign49410_e63564);
            let assign49410_e63566: f64 = (assign49410_e63557 - assign49410_e63565);
            (locals.var_x_s__blk1346, locals.var_x_s__blk1346_dn5, locals.var_x_s__blk1346_dn6, locals.var_x_s__blk1346_dn7, locals.var_x_s__blk1346_dn8, ) = (assign49410_e63566, ((-locals.var_sp_s_y0__blk1440_dn5) - (2.0 * (((locals.var_sp_s_qc__blk1447_dn5 * assign49410_e63563) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn5 + (locals.var_sp_s_temp__blk1431_dn5 / (2.0 * assign49410_e63562))))) / (assign49410_e63563 * assign49410_e63563)))), ((-locals.var_sp_s_y0__blk1440_dn6) - (2.0 * (((locals.var_sp_s_qc__blk1447_dn6 * assign49410_e63563) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn6 + (locals.var_sp_s_temp__blk1431_dn6 / (2.0 * assign49410_e63562))))) / (assign49410_e63563 * assign49410_e63563)))), ((-locals.var_sp_s_y0__blk1440_dn7) - (2.0 * (((locals.var_sp_s_qc__blk1447_dn7 * assign49410_e63563) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn7 + (locals.var_sp_s_temp__blk1431_dn7 / (2.0 * assign49410_e63562))))) / (assign49410_e63563 * assign49410_e63563)))), ((-locals.var_sp_s_y0__blk1440_dn8) - (2.0 * (((locals.var_sp_s_qc__blk1447_dn8 * assign49410_e63563) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn8 + (locals.var_sp_s_temp__blk1431_dn8 / (2.0 * assign49410_e63562))))) / (assign49410_e63563 * assign49410_e63563)))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49420_e63582: f64 = (locals.var_gf__blk1307 * 0.7324648775608221);
            let assign49420_e63583: f64 = (1.25 + assign49420_e63582);
            let assign49420_e63584: f64 = (1.0 / assign49420_e63583);
            (locals.var_sp_xg1__blk1448, locals.var_sp_xg1__blk1448_dn5, locals.var_sp_xg1__blk1448_dn6, locals.var_sp_xg1__blk1448_dn7, locals.var_sp_xg1__blk1448_dn8, ) = (assign49420_e63584, (-((locals.var_gf__blk1307_dn5 * 0.7324648775608221) / (assign49420_e63583 * assign49420_e63583))), (-((locals.var_gf__blk1307_dn6 * 0.7324648775608221) / (assign49420_e63583 * assign49420_e63583))), (-((locals.var_gf__blk1307_dn7 * 0.7324648775608221) / (assign49420_e63583 * assign49420_e63583))), (-((locals.var_gf__blk1307_dn8 * 0.7324648775608221) / (assign49420_e63583 * assign49420_e63583))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49430_e63598: f64 = (locals.var_xi__blk1343 * 1.25);
            let assign49430_e63600: f64 = (assign49430_e63598 * locals.var_sp_xg1__blk1448);
            let assign49430_e63602: f64 = (assign49430_e63600 - 1.0);
            let assign49430_e63604: f64 = (assign49430_e63602 * locals.var_sp_xg1__blk1448);
            (locals.var_sp_s_a_fac__blk1449, locals.var_sp_s_a_fac__blk1449_dn5, locals.var_sp_s_a_fac__blk1449_dn6, locals.var_sp_s_a_fac__blk1449_dn7, locals.var_sp_s_a_fac__blk1449_dn8, ) = (assign49430_e63604, (((((locals.var_xi__blk1343_dn5 * 1.25) * locals.var_sp_xg1__blk1448) + (assign49430_e63598 * locals.var_sp_xg1__blk1448_dn5)) * locals.var_sp_xg1__blk1448) + (assign49430_e63602 * locals.var_sp_xg1__blk1448_dn5)), (((((locals.var_xi__blk1343_dn6 * 1.25) * locals.var_sp_xg1__blk1448) + (assign49430_e63598 * locals.var_sp_xg1__blk1448_dn6)) * locals.var_sp_xg1__blk1448) + (assign49430_e63602 * locals.var_sp_xg1__blk1448_dn6)), (((((locals.var_xi__blk1343_dn7 * 1.25) * locals.var_sp_xg1__blk1448) + (assign49430_e63598 * locals.var_sp_xg1__blk1448_dn7)) * locals.var_sp_xg1__blk1448) + (assign49430_e63602 * locals.var_sp_xg1__blk1448_dn7)), (((((locals.var_xi__blk1343_dn8 * 1.25) * locals.var_sp_xg1__blk1448) + (assign49430_e63598 * locals.var_sp_xg1__blk1448_dn8)) * locals.var_sp_xg1__blk1448) + (assign49430_e63602 * locals.var_sp_xg1__blk1448_dn8)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49440_e63618: f64 = (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345);
            let assign49440_e63622: f64 = (locals.var_sp_s_a_fac__blk1449 * locals.var_xg__blk1326);
            let assign49440_e63623: f64 = (1.0 + assign49440_e63622);
            let assign49440_e63624: f64 = (assign49440_e63618 * assign49440_e63623);
            (locals.var_sp_s_xbar__blk1450, locals.var_sp_s_xbar__blk1450_dn5, locals.var_sp_s_xbar__blk1450_dn6, locals.var_sp_s_xbar__blk1450_dn7, locals.var_sp_s_xbar__blk1450_dn8, ) = (assign49440_e63624, ((((locals.var_xg__blk1326_dn5 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn5)) * assign49440_e63623) + (assign49440_e63618 * ((locals.var_sp_s_a_fac__blk1449_dn5 * locals.var_xg__blk1326) + (locals.var_sp_s_a_fac__blk1449 * locals.var_xg__blk1326_dn5)))), ((((locals.var_xg__blk1326_dn6 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn6)) * assign49440_e63623) + (assign49440_e63618 * ((locals.var_sp_s_a_fac__blk1449_dn6 * locals.var_xg__blk1326) + (locals.var_sp_s_a_fac__blk1449 * locals.var_xg__blk1326_dn6)))), ((((locals.var_xg__blk1326_dn7 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn7)) * assign49440_e63623) + (assign49440_e63618 * ((locals.var_sp_s_a_fac__blk1449_dn7 * locals.var_xg__blk1326) + (locals.var_sp_s_a_fac__blk1449 * locals.var_xg__blk1326_dn7)))), ((((locals.var_xg__blk1326_dn8 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn8)) * assign49440_e63623) + (assign49440_e63618 * ((locals.var_sp_s_a_fac__blk1449_dn8 * locals.var_xg__blk1326) + (locals.var_sp_s_a_fac__blk1449 * locals.var_xg__blk1326_dn8)))), );
        }

        let assign49450_e63628: f64 = (-locals.var_sp_s_xbar__blk1450);
        let assign49450_e63630: f64 = (-230.25850929940458);
        let assign49450_e63631: f64 = if assign49450_e63628 > assign49450_e63630 { 1.0 } else { 0.0 };
        locals.var_guard1471 = assign49450_e63631;

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1471 != 0.0)) {
            let assign49460_e63644: f64 = (-locals.var_sp_s_xbar__blk1450);
            let assign49460_e63645: f64 = (assign49460_e63644).exp();
            (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8, ) = (assign49460_e63645, (assign49460_e63645 * (-locals.var_sp_s_xbar__blk1450_dn5)), (assign49460_e63645 * (-locals.var_sp_s_xbar__blk1450_dn6)), (assign49460_e63645 * (-locals.var_sp_s_xbar__blk1450_dn7)), (assign49460_e63645 * (-locals.var_sp_s_xbar__blk1450_dn8)), );
        }

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1471 == 0.0)) {
            let assign49470_e63663: f64 = (-230.25850929940458);
            let assign49470_e63665: f64 = (-locals.var_sp_s_xbar__blk1450);
            let assign49470_e63666: f64 = (assign49470_e63663 - assign49470_e63665);
            let assign49470_e63670: f64 = (-230.25850929940458);
            let assign49470_e63672: f64 = (-locals.var_sp_s_xbar__blk1450);
            let assign49470_e63673: f64 = (assign49470_e63670 - assign49470_e63672);
            let assign49470_e63676: f64 = (-230.25850929940458);
            let assign49470_e63678: f64 = (-locals.var_sp_s_xbar__blk1450);
            let assign49470_e63679: f64 = (assign49470_e63676 - assign49470_e63678);
            let assign49470_e63681: f64 = (assign49470_e63679 * 0.3333333333333333);
            let assign49470_e63682: f64 = (1.0 + assign49470_e63681);
            let assign49470_e63683: f64 = (assign49470_e63673 * assign49470_e63682);
            let assign49470_e63684: f64 = (0.5 * assign49470_e63683);
            let assign49470_e63685: f64 = (1.0 + assign49470_e63684);
            let assign49470_e63686: f64 = (assign49470_e63666 * assign49470_e63685);
            let assign49470_e63687: f64 = (1.0 + assign49470_e63686);
            let assign49470_e63688: f64 = (1e-100 / assign49470_e63687);
            (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8, ) = (assign49470_e63688, (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1450_dn5)) * assign49470_e63685) + (assign49470_e63666 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1450_dn5)) * assign49470_e63682) + (assign49470_e63673 * ((-(-locals.var_sp_s_xbar__blk1450_dn5)) * 0.3333333333333333))))))) / (assign49470_e63687 * assign49470_e63687))), (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1450_dn6)) * assign49470_e63685) + (assign49470_e63666 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1450_dn6)) * assign49470_e63682) + (assign49470_e63673 * ((-(-locals.var_sp_s_xbar__blk1450_dn6)) * 0.3333333333333333))))))) / (assign49470_e63687 * assign49470_e63687))), (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1450_dn7)) * assign49470_e63685) + (assign49470_e63666 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1450_dn7)) * assign49470_e63682) + (assign49470_e63673 * ((-(-locals.var_sp_s_xbar__blk1450_dn7)) * 0.3333333333333333))))))) / (assign49470_e63687 * assign49470_e63687))), (-((1e-100 * (((-(-locals.var_sp_s_xbar__blk1450_dn8)) * assign49470_e63685) + (assign49470_e63666 * (0.5 * (((-(-locals.var_sp_s_xbar__blk1450_dn8)) * assign49470_e63682) + (assign49470_e63673 * ((-(-locals.var_sp_s_xbar__blk1450_dn8)) * 0.3333333333333333))))))) / (assign49470_e63687 * assign49470_e63687))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49480_e63702: f64 = (1.0 - locals.var_sp_s_temp__blk1431);
            (locals.var_sp_s_w__blk1451, locals.var_sp_s_w__blk1451_dn5, locals.var_sp_s_w__blk1451_dn6, locals.var_sp_s_w__blk1451_dn7, locals.var_sp_s_w__blk1451_dn8, ) = (assign49480_e63702, (-locals.var_sp_s_temp__blk1431_dn5), (-locals.var_sp_s_temp__blk1431_dn6), (-locals.var_sp_s_temp__blk1431_dn7), (-locals.var_sp_s_temp__blk1431_dn8), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49490_e63717: f64 = (locals.var_gf2__blk1308 * 0.5);
            let assign49490_e63718: f64 = (locals.var_xg__blk1326 + assign49490_e63717);
            let assign49490_e63723: f64 = (locals.var_gf2__blk1308 * 0.25);
            let assign49490_e63724: f64 = (locals.var_xg__blk1326 + assign49490_e63723);
            let assign49490_e63726: f64 = (assign49490_e63724 - locals.var_sp_s_w__blk1451);
            let assign49490_e63727: f64 = (assign49490_e63726).sqrt();
            let assign49490_e63728: f64 = (locals.var_gf__blk1307 * assign49490_e63727);
            let assign49490_e63729: f64 = (assign49490_e63718 - assign49490_e63728);
            (locals.var_sp_s_x1__blk1452, locals.var_sp_s_x1__blk1452_dn5, locals.var_sp_s_x1__blk1452_dn6, locals.var_sp_s_x1__blk1452_dn7, locals.var_sp_s_x1__blk1452_dn8, ) = (assign49490_e63729, ((locals.var_xg__blk1326_dn5 + (locals.var_gf2__blk1308_dn5 * 0.5)) - ((locals.var_gf__blk1307_dn5 * assign49490_e63727) + (locals.var_gf__blk1307 * (((locals.var_xg__blk1326_dn5 + (locals.var_gf2__blk1308_dn5 * 0.25)) - locals.var_sp_s_w__blk1451_dn5) / (2.0 * assign49490_e63727))))), ((locals.var_xg__blk1326_dn6 + (locals.var_gf2__blk1308_dn6 * 0.5)) - ((locals.var_gf__blk1307_dn6 * assign49490_e63727) + (locals.var_gf__blk1307 * (((locals.var_xg__blk1326_dn6 + (locals.var_gf2__blk1308_dn6 * 0.25)) - locals.var_sp_s_w__blk1451_dn6) / (2.0 * assign49490_e63727))))), ((locals.var_xg__blk1326_dn7 + (locals.var_gf2__blk1308_dn7 * 0.5)) - ((locals.var_gf__blk1307_dn7 * assign49490_e63727) + (locals.var_gf__blk1307 * (((locals.var_xg__blk1326_dn7 + (locals.var_gf2__blk1308_dn7 * 0.25)) - locals.var_sp_s_w__blk1451_dn7) / (2.0 * assign49490_e63727))))), ((locals.var_xg__blk1326_dn8 + (locals.var_gf2__blk1308_dn8 * 0.5)) - ((locals.var_gf__blk1307_dn8 * assign49490_e63727) + (locals.var_gf__blk1307 * (((locals.var_xg__blk1326_dn8 + (locals.var_gf2__blk1308_dn8 * 0.25)) - locals.var_sp_s_w__blk1451_dn8) / (2.0 * assign49490_e63727))))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49500_e63743: f64 = (locals.var_xn_s__blk1332 + 3.0);
            (locals.var_sp_s_bx__blk1453, locals.var_sp_s_bx__blk1453_dn5, locals.var_sp_s_bx__blk1453_dn6, locals.var_sp_s_bx__blk1453_dn7, locals.var_sp_s_bx__blk1453_dn8, ) = (assign49500_e63743, locals.var_xn_s__blk1332_dn5, locals.var_xn_s__blk1332_dn6, locals.var_xn_s__blk1332_dn7, locals.var_xn_s__blk1332_dn8, );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49510_e63758: f64 = (locals.var_sp_s_x1__blk1452 + locals.var_sp_s_bx__blk1453);
            let assign49510_e63761: f64 = (locals.var_sp_s_x1__blk1452 - locals.var_sp_s_bx__blk1453);
            let assign49510_e63764: f64 = (locals.var_sp_s_x1__blk1452 - locals.var_sp_s_bx__blk1453);
            let assign49510_e63765: f64 = (assign49510_e63761 * assign49510_e63764);
            let assign49510_e63767: f64 = (assign49510_e63765 + 5.0);
            let assign49510_e63768: f64 = (assign49510_e63767).sqrt();
            let assign49510_e63769: f64 = (assign49510_e63758 - assign49510_e63768);
            let assign49510_e63770: f64 = (0.5 * assign49510_e63769);
            let assign49510_e63775: f64 = (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453);
            let assign49510_e63777: f64 = (assign49510_e63775 + 5.0);
            let assign49510_e63778: f64 = (assign49510_e63777).sqrt();
            let assign49510_e63779: f64 = (locals.var_sp_s_bx__blk1453 - assign49510_e63778);
            let assign49510_e63780: f64 = (0.5 * assign49510_e63779);
            let assign49510_e63781: f64 = (assign49510_e63770 - assign49510_e63780);
            (locals.var_sp_s_eta__blk1436, locals.var_sp_s_eta__blk1436_dn5, locals.var_sp_s_eta__blk1436_dn6, locals.var_sp_s_eta__blk1436_dn7, locals.var_sp_s_eta__blk1436_dn8, ) = (assign49510_e63781, ((0.5 * ((locals.var_sp_s_x1__blk1452_dn5 + locals.var_sp_s_bx__blk1453_dn5) - ((((locals.var_sp_s_x1__blk1452_dn5 - locals.var_sp_s_bx__blk1453_dn5) * assign49510_e63764) + (assign49510_e63761 * (locals.var_sp_s_x1__blk1452_dn5 - locals.var_sp_s_bx__blk1453_dn5))) / (2.0 * assign49510_e63768)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn5 - (((locals.var_sp_s_bx__blk1453_dn5 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn5)) / (2.0 * assign49510_e63778))))), ((0.5 * ((locals.var_sp_s_x1__blk1452_dn6 + locals.var_sp_s_bx__blk1453_dn6) - ((((locals.var_sp_s_x1__blk1452_dn6 - locals.var_sp_s_bx__blk1453_dn6) * assign49510_e63764) + (assign49510_e63761 * (locals.var_sp_s_x1__blk1452_dn6 - locals.var_sp_s_bx__blk1453_dn6))) / (2.0 * assign49510_e63768)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn6 - (((locals.var_sp_s_bx__blk1453_dn6 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn6)) / (2.0 * assign49510_e63778))))), ((0.5 * ((locals.var_sp_s_x1__blk1452_dn7 + locals.var_sp_s_bx__blk1453_dn7) - ((((locals.var_sp_s_x1__blk1452_dn7 - locals.var_sp_s_bx__blk1453_dn7) * assign49510_e63764) + (assign49510_e63761 * (locals.var_sp_s_x1__blk1452_dn7 - locals.var_sp_s_bx__blk1453_dn7))) / (2.0 * assign49510_e63768)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn7 - (((locals.var_sp_s_bx__blk1453_dn7 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn7)) / (2.0 * assign49510_e63778))))), ((0.5 * ((locals.var_sp_s_x1__blk1452_dn8 + locals.var_sp_s_bx__blk1453_dn8) - ((((locals.var_sp_s_x1__blk1452_dn8 - locals.var_sp_s_bx__blk1453_dn8) * assign49510_e63764) + (assign49510_e63761 * (locals.var_sp_s_x1__blk1452_dn8 - locals.var_sp_s_bx__blk1453_dn8))) / (2.0 * assign49510_e63768)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn8 - (((locals.var_sp_s_bx__blk1453_dn8 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn8)) / (2.0 * assign49510_e63778))))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49520_e63795: f64 = (locals.var_xg__blk1326 - locals.var_sp_s_eta__blk1436);
            (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8, ) = (assign49520_e63795, (locals.var_xg__blk1326_dn5 - locals.var_sp_s_eta__blk1436_dn5), (locals.var_xg__blk1326_dn6 - locals.var_sp_s_eta__blk1436_dn6), (locals.var_xg__blk1326_dn7 - locals.var_sp_s_eta__blk1436_dn7), (locals.var_xg__blk1326_dn8 - locals.var_sp_s_eta__blk1436_dn8), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49530_e63808: f64 = (-locals.var_sp_s_eta__blk1436);
            let assign49530_e63809: f64 = (assign49530_e63808).exp();
            (locals.var_sp_s_temp1__blk1432, locals.var_sp_s_temp1__blk1432_dn5, locals.var_sp_s_temp1__blk1432_dn6, locals.var_sp_s_temp1__blk1432_dn7, locals.var_sp_s_temp1__blk1432_dn8, ) = (assign49530_e63809, (assign49530_e63809 * (-locals.var_sp_s_eta__blk1436_dn5)), (assign49530_e63809 * (-locals.var_sp_s_eta__blk1436_dn6)), (assign49530_e63809 * (-locals.var_sp_s_eta__blk1436_dn7)), (assign49530_e63809 * (-locals.var_sp_s_eta__blk1436_dn8)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49540_e63825: f64 = (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436);
            let assign49540_e63826: f64 = (2.0 + assign49540_e63825);
            let assign49540_e63827: f64 = (1.0 / assign49540_e63826);
            (locals.var_sp_s_temp2__blk1433, locals.var_sp_s_temp2__blk1433_dn5, locals.var_sp_s_temp2__blk1433_dn6, locals.var_sp_s_temp2__blk1433_dn7, locals.var_sp_s_temp2__blk1433_dn8, ) = (assign49540_e63827, (-(((locals.var_sp_s_eta__blk1436_dn5 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn5)) / (assign49540_e63826 * assign49540_e63826))), (-(((locals.var_sp_s_eta__blk1436_dn6 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn6)) / (assign49540_e63826 * assign49540_e63826))), (-(((locals.var_sp_s_eta__blk1436_dn7 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn7)) / (assign49540_e63826 * assign49540_e63826))), (-(((locals.var_sp_s_eta__blk1436_dn8 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn8)) / (assign49540_e63826 * assign49540_e63826))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49550_e63841: f64 = (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436);
            let assign49550_e63843: f64 = (assign49550_e63841 * locals.var_sp_s_temp2__blk1433);
            (locals.var_sp_s_xi0__blk1443, locals.var_sp_s_xi0__blk1443_dn5, locals.var_sp_s_xi0__blk1443_dn6, locals.var_sp_s_xi0__blk1443_dn7, locals.var_sp_s_xi0__blk1443_dn8, ) = (assign49550_e63843, ((((locals.var_sp_s_eta__blk1436_dn5 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign49550_e63841 * locals.var_sp_s_temp2__blk1433_dn5)), ((((locals.var_sp_s_eta__blk1436_dn6 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign49550_e63841 * locals.var_sp_s_temp2__blk1433_dn6)), ((((locals.var_sp_s_eta__blk1436_dn7 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign49550_e63841 * locals.var_sp_s_temp2__blk1433_dn7)), ((((locals.var_sp_s_eta__blk1436_dn8 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign49550_e63841 * locals.var_sp_s_temp2__blk1433_dn8)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49560_e63858: f64 = (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433);
            let assign49560_e63860: f64 = (assign49560_e63858 * locals.var_sp_s_temp2__blk1433);
            let assign49560_e63861: f64 = (4.0 * assign49560_e63860);
            (locals.var_sp_s_xi1__blk1444, locals.var_sp_s_xi1__blk1444_dn5, locals.var_sp_s_xi1__blk1444_dn6, locals.var_sp_s_xi1__blk1444_dn7, locals.var_sp_s_xi1__blk1444_dn8, ) = (assign49560_e63861, (4.0 * ((((locals.var_sp_s_eta__blk1436_dn5 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign49560_e63858 * locals.var_sp_s_temp2__blk1433_dn5))), (4.0 * ((((locals.var_sp_s_eta__blk1436_dn6 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign49560_e63858 * locals.var_sp_s_temp2__blk1433_dn6))), (4.0 * ((((locals.var_sp_s_eta__blk1436_dn7 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign49560_e63858 * locals.var_sp_s_temp2__blk1433_dn7))), (4.0 * ((((locals.var_sp_s_eta__blk1436_dn8 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign49560_e63858 * locals.var_sp_s_temp2__blk1433_dn8))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49570_e63875: f64 = (8.0 * locals.var_sp_s_temp2__blk1433);
            let assign49570_e63878: f64 = (12.0 * locals.var_sp_s_xi0__blk1443);
            let assign49570_e63879: f64 = (assign49570_e63875 - assign49570_e63878);
            let assign49570_e63881: f64 = (assign49570_e63879 * locals.var_sp_s_temp2__blk1433);
            let assign49570_e63883: f64 = (assign49570_e63881 * locals.var_sp_s_temp2__blk1433);
            (locals.var_sp_s_xi2__blk1445, locals.var_sp_s_xi2__blk1445_dn5, locals.var_sp_s_xi2__blk1445_dn6, locals.var_sp_s_xi2__blk1445_dn7, locals.var_sp_s_xi2__blk1445_dn8, ) = (assign49570_e63883, ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn5) - (12.0 * locals.var_sp_s_xi0__blk1443_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63879 * locals.var_sp_s_temp2__blk1433_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63881 * locals.var_sp_s_temp2__blk1433_dn5)), ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn6) - (12.0 * locals.var_sp_s_xi0__blk1443_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63879 * locals.var_sp_s_temp2__blk1433_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63881 * locals.var_sp_s_temp2__blk1433_dn6)), ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn7) - (12.0 * locals.var_sp_s_xi0__blk1443_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63879 * locals.var_sp_s_temp2__blk1433_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63881 * locals.var_sp_s_temp2__blk1433_dn7)), ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn8) - (12.0 * locals.var_sp_s_xi0__blk1443_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63879 * locals.var_sp_s_temp2__blk1433_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign49570_e63881 * locals.var_sp_s_temp2__blk1433_dn8)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49580_e63898: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
            let assign49580_e63902: f64 = (locals.var_sp_s_temp1__blk1432 + locals.var_sp_s_eta__blk1436);
            let assign49580_e63904: f64 = (assign49580_e63902 - 1.0);
            let assign49580_e63908: f64 = (locals.var_sp_s_eta__blk1436 + 1.0);
            let assign49580_e63910: f64 = (assign49580_e63908 + locals.var_sp_s_xi0__blk1443);
            let assign49580_e63911: f64 = (locals.var_delta_ns__blk1347 * assign49580_e63910);
            let assign49580_e63912: f64 = (assign49580_e63904 - assign49580_e63911);
            let assign49580_e63913: f64 = (locals.var_gf2__blk1308 * assign49580_e63912);
            let assign49580_e63914: f64 = (assign49580_e63898 - assign49580_e63913);
            let (assign49580_e63936, assign49580_e63936_d_n5, assign49580_e63936_d_n6, assign49580_e63936_d_n7, assign49580_e63936_d_n8,) = {
    if (1e-40 > assign49580_e63914) {
        (1e-40, 0.0, 0.0, 0.0, 0.0,)
    } else {
        let assign49580_e63919: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
        let assign49580_e63923: f64 = (locals.var_sp_s_temp1__blk1432 + locals.var_sp_s_eta__blk1436);
        let assign49580_e63925: f64 = (assign49580_e63923 - 1.0);
        let assign49580_e63929: f64 = (locals.var_sp_s_eta__blk1436 + 1.0);
        let assign49580_e63931: f64 = (assign49580_e63929 + locals.var_sp_s_xi0__blk1443);
        let assign49580_e63932: f64 = (locals.var_delta_ns__blk1347 * assign49580_e63931);
        let assign49580_e63933: f64 = (assign49580_e63925 - assign49580_e63932);
        let assign49580_e63934: f64 = (locals.var_gf2__blk1308 * assign49580_e63933);
        let assign49580_e63935: f64 = (assign49580_e63919 - assign49580_e63934);
        (assign49580_e63935, (((locals.var_sp_s_temp__blk1431_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn5)) - ((locals.var_gf2__blk1308_dn5 * assign49580_e63933) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn5 + locals.var_sp_s_eta__blk1436_dn5) - ((locals.var_delta_ns__blk1347_dn5 * assign49580_e63931) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_eta__blk1436_dn5 + locals.var_sp_s_xi0__blk1443_dn5))))))), (((locals.var_sp_s_temp__blk1431_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn6)) - ((locals.var_gf2__blk1308_dn6 * assign49580_e63933) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn6 + locals.var_sp_s_eta__blk1436_dn6) - ((locals.var_delta_ns__blk1347_dn6 * assign49580_e63931) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_eta__blk1436_dn6 + locals.var_sp_s_xi0__blk1443_dn6))))))), (((locals.var_sp_s_temp__blk1431_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn7)) - ((locals.var_gf2__blk1308_dn7 * assign49580_e63933) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn7 + locals.var_sp_s_eta__blk1436_dn7) - ((locals.var_delta_ns__blk1347_dn7 * assign49580_e63931) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_eta__blk1436_dn7 + locals.var_sp_s_xi0__blk1443_dn7))))))), (((locals.var_sp_s_temp__blk1431_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn8)) - ((locals.var_gf2__blk1308_dn8 * assign49580_e63933) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn8 + locals.var_sp_s_eta__blk1436_dn8) - ((locals.var_delta_ns__blk1347_dn8 * assign49580_e63931) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_eta__blk1436_dn8 + locals.var_sp_s_xi0__blk1443_dn8))))))),)
    }
};
            (locals.var_sp_s_a__blk1437, locals.var_sp_s_a__blk1437_dn5, locals.var_sp_s_a__blk1437_dn6, locals.var_sp_s_a__blk1437_dn7, locals.var_sp_s_a__blk1437_dn8, ) = (assign49580_e63936, assign49580_e63936_d_n5, assign49580_e63936_d_n6, assign49580_e63936_d_n7, assign49580_e63936_d_n8, );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49590_e63954: f64 = (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445);
            let assign49590_e63955: f64 = (locals.var_sp_s_temp1__blk1432 - assign49590_e63954);
            let assign49590_e63956: f64 = (locals.var_gf2__blk1308 * assign49590_e63955);
            let assign49590_e63957: f64 = (0.5 * assign49590_e63956);
            let assign49590_e63958: f64 = (1.0 - assign49590_e63957);
            (locals.var_sp_s_b__blk1454, locals.var_sp_s_b__blk1454_dn5, locals.var_sp_s_b__blk1454_dn6, locals.var_sp_s_b__blk1454_dn7, locals.var_sp_s_b__blk1454_dn8, ) = (assign49590_e63958, (-(0.5 * ((locals.var_gf2__blk1308_dn5 * assign49590_e63955) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn5 - ((locals.var_delta_ns__blk1347_dn5 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn5))))))), (-(0.5 * ((locals.var_gf2__blk1308_dn6 * assign49590_e63955) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn6 - ((locals.var_delta_ns__blk1347_dn6 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn6))))))), (-(0.5 * ((locals.var_gf2__blk1308_dn7 * assign49590_e63955) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn7 - ((locals.var_delta_ns__blk1347_dn7 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn7))))))), (-(0.5 * ((locals.var_gf2__blk1308_dn8 * assign49590_e63955) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn8 - ((locals.var_delta_ns__blk1347_dn8 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn8))))))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49600_e63972: f64 = (2.0 * locals.var_sp_s_temp__blk1431);
            let assign49600_e63976: f64 = (1.0 - locals.var_sp_s_temp1__blk1432);
            let assign49600_e63980: f64 = (1.0 + locals.var_sp_s_xi1__blk1444);
            let assign49600_e63981: f64 = (locals.var_delta_ns__blk1347 * assign49600_e63980);
            let assign49600_e63982: f64 = (assign49600_e63976 - assign49600_e63981);
            let assign49600_e63983: f64 = (locals.var_gf2__blk1308 * assign49600_e63982);
            let assign49600_e63984: f64 = (assign49600_e63972 + assign49600_e63983);
            (locals.var_sp_s_c__blk1438, locals.var_sp_s_c__blk1438_dn5, locals.var_sp_s_c__blk1438_dn6, locals.var_sp_s_c__blk1438_dn7, locals.var_sp_s_c__blk1438_dn8, ) = (assign49600_e63984, ((2.0 * locals.var_sp_s_temp__blk1431_dn5) + ((locals.var_gf2__blk1308_dn5 * assign49600_e63982) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn5) - ((locals.var_delta_ns__blk1347_dn5 * assign49600_e63980) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn5)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn6) + ((locals.var_gf2__blk1308_dn6 * assign49600_e63982) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn6) - ((locals.var_delta_ns__blk1347_dn6 * assign49600_e63980) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn7) + ((locals.var_gf2__blk1308_dn7 * assign49600_e63982) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn7) - ((locals.var_delta_ns__blk1347_dn7 * assign49600_e63980) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn8) + ((locals.var_gf2__blk1308_dn8 * assign49600_e63982) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn8) - ((locals.var_delta_ns__blk1347_dn8 * assign49600_e63980) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn8)))))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49610_e63998: f64 = (locals.var_xn_s__blk1332 - locals.var_sp_s_eta__blk1436);
            let assign49610_e64001: f64 = (locals.var_sp_s_a__blk1437 / locals.var_gf2__blk1308);
            let assign49610_e64002: f64 = (assign49610_e64001).ln();
            let assign49610_e64003: f64 = (assign49610_e63998 + assign49610_e64002);
            (locals.var_sp_s_tau__blk1439, locals.var_sp_s_tau__blk1439_dn5, locals.var_sp_s_tau__blk1439_dn6, locals.var_sp_s_tau__blk1439_dn7, locals.var_sp_s_tau__blk1439_dn8, ) = (assign49610_e64003, ((locals.var_xn_s__blk1332_dn5 - locals.var_sp_s_eta__blk1436_dn5) + ((((locals.var_sp_s_a__blk1437_dn5 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn5)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign49610_e64001)), ((locals.var_xn_s__blk1332_dn6 - locals.var_sp_s_eta__blk1436_dn6) + ((((locals.var_sp_s_a__blk1437_dn6 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn6)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign49610_e64001)), ((locals.var_xn_s__blk1332_dn7 - locals.var_sp_s_eta__blk1436_dn7) + ((((locals.var_sp_s_a__blk1437_dn7 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn7)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign49610_e64001)), ((locals.var_xn_s__blk1332_dn8 - locals.var_sp_s_eta__blk1436_dn8) + ((((locals.var_sp_s_a__blk1437_dn8 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn8)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign49610_e64001)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49620_e64017: f64 = (locals.var_sp_s_a__blk1437 + locals.var_sp_s_c__blk1438);
            (locals.var_nu, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, ) = (assign49620_e64017, (locals.var_sp_s_a__blk1437_dn5 + locals.var_sp_s_c__blk1438_dn5), (locals.var_sp_s_a__blk1437_dn6 + locals.var_sp_s_c__blk1438_dn6), (locals.var_sp_s_a__blk1437_dn7 + locals.var_sp_s_c__blk1438_dn7), (locals.var_sp_s_a__blk1437_dn8 + locals.var_sp_s_c__blk1438_dn8), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49630_e64031: f64 = (locals.var_nu * locals.var_nu);
            let assign49630_e64036: f64 = (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438);
            let assign49630_e64037: f64 = (0.5 * assign49630_e64036);
            let assign49630_e64040: f64 = (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454);
            let assign49630_e64041: f64 = (assign49630_e64037 - assign49630_e64040);
            let assign49630_e64042: f64 = (locals.var_sp_s_tau__blk1439 * assign49630_e64041);
            let assign49630_e64043: f64 = (assign49630_e64031 + assign49630_e64042);
            (locals.var_mutau, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, ) = (assign49630_e64043, (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_sp_s_tau__blk1439_dn5 * assign49630_e64041) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn5 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn5))) - ((locals.var_sp_s_a__blk1437_dn5 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn5)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau__blk1439_dn6 * assign49630_e64041) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn6 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn6))) - ((locals.var_sp_s_a__blk1437_dn6 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau__blk1439_dn7 * assign49630_e64041) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn7 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn7))) - ((locals.var_sp_s_a__blk1437_dn7 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau__blk1439_dn8 * assign49630_e64041) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn8 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn8))) - ((locals.var_sp_s_a__blk1437_dn8 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn8)))))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49640_e64058: f64 = (locals.var_sp_s_a__blk1437 * locals.var_nu);
            let assign49640_e64060: f64 = (assign49640_e64058 * locals.var_sp_s_tau__blk1439);
            let assign49640_e64064: f64 = (locals.var_nu / locals.var_mutau);
            let assign49640_e64066: f64 = (assign49640_e64064 * locals.var_sp_s_tau__blk1439);
            let assign49640_e64068: f64 = (assign49640_e64066 * locals.var_sp_s_tau__blk1439);
            let assign49640_e64070: f64 = (assign49640_e64068 * locals.var_sp_s_c__blk1438);
            let assign49640_e64073: f64 = (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438);
            let assign49640_e64075: f64 = (assign49640_e64073 * 0.3333333333333333);
            let assign49640_e64078: f64 = (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454);
            let assign49640_e64079: f64 = (assign49640_e64075 - assign49640_e64078);
            let assign49640_e64080: f64 = (assign49640_e64070 * assign49640_e64079);
            let assign49640_e64081: f64 = (locals.var_mutau + assign49640_e64080);
            let assign49640_e64082: f64 = (assign49640_e64060 / assign49640_e64081);
            let assign49640_e64083: f64 = (locals.var_sp_s_eta__blk1436 + assign49640_e64082);
            (locals.var_sp_s_x0__blk1455, locals.var_sp_s_x0__blk1455_dn5, locals.var_sp_s_x0__blk1455_dn6, locals.var_sp_s_x0__blk1455_dn7, locals.var_sp_s_x0__blk1455_dn8, ) = (assign49640_e64083, (locals.var_sp_s_eta__blk1436_dn5 + (((((((locals.var_sp_s_a__blk1437_dn5 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn5)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64058 * locals.var_sp_s_tau__blk1439_dn5)) * assign49640_e64081) - (assign49640_e64060 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64064 * locals.var_sp_s_tau__blk1439_dn5)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64066 * locals.var_sp_s_tau__blk1439_dn5)) * locals.var_sp_s_c__blk1438) + (assign49640_e64068 * locals.var_sp_s_c__blk1438_dn5)) * assign49640_e64079) + (assign49640_e64070 * ((((locals.var_sp_s_c__blk1438_dn5 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn5)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn5 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn5)))))))) / (assign49640_e64081 * assign49640_e64081))), (locals.var_sp_s_eta__blk1436_dn6 + (((((((locals.var_sp_s_a__blk1437_dn6 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn6)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64058 * locals.var_sp_s_tau__blk1439_dn6)) * assign49640_e64081) - (assign49640_e64060 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64064 * locals.var_sp_s_tau__blk1439_dn6)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64066 * locals.var_sp_s_tau__blk1439_dn6)) * locals.var_sp_s_c__blk1438) + (assign49640_e64068 * locals.var_sp_s_c__blk1438_dn6)) * assign49640_e64079) + (assign49640_e64070 * ((((locals.var_sp_s_c__blk1438_dn6 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn6)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn6 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn6)))))))) / (assign49640_e64081 * assign49640_e64081))), (locals.var_sp_s_eta__blk1436_dn7 + (((((((locals.var_sp_s_a__blk1437_dn7 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn7)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64058 * locals.var_sp_s_tau__blk1439_dn7)) * assign49640_e64081) - (assign49640_e64060 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64064 * locals.var_sp_s_tau__blk1439_dn7)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64066 * locals.var_sp_s_tau__blk1439_dn7)) * locals.var_sp_s_c__blk1438) + (assign49640_e64068 * locals.var_sp_s_c__blk1438_dn7)) * assign49640_e64079) + (assign49640_e64070 * ((((locals.var_sp_s_c__blk1438_dn7 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn7)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn7 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn7)))))))) / (assign49640_e64081 * assign49640_e64081))), (locals.var_sp_s_eta__blk1436_dn8 + (((((((locals.var_sp_s_a__blk1437_dn8 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn8)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64058 * locals.var_sp_s_tau__blk1439_dn8)) * assign49640_e64081) - (assign49640_e64060 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64064 * locals.var_sp_s_tau__blk1439_dn8)) * locals.var_sp_s_tau__blk1439) + (assign49640_e64066 * locals.var_sp_s_tau__blk1439_dn8)) * locals.var_sp_s_c__blk1438) + (assign49640_e64068 * locals.var_sp_s_c__blk1438_dn8)) * assign49640_e64079) + (assign49640_e64070 * ((((locals.var_sp_s_c__blk1438_dn8 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn8)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn8 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn8)))))))) / (assign49640_e64081 * assign49640_e64081))), );
        }

        let assign49650_e64088: f64 = if locals.var_sp_s_x0__blk1455 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1472 = assign49650_e64088;

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1472 != 0.0)) {
            let assign49660_e64101: f64 = (locals.var_sp_s_x0__blk1455).exp();
            (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8, ) = (assign49660_e64101, (assign49660_e64101 * locals.var_sp_s_x0__blk1455_dn5), (assign49660_e64101 * locals.var_sp_s_x0__blk1455_dn6), (assign49660_e64101 * locals.var_sp_s_x0__blk1455_dn7), (assign49660_e64101 * locals.var_sp_s_x0__blk1455_dn8), );
        }

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1472 != 0.0)) {
            let assign49670_e64117: f64 = (1.0 / locals.var_sp_s_delta0__blk1441);
            (locals.var_sp_s_delta1__blk1442, locals.var_sp_s_delta1__blk1442_dn5, locals.var_sp_s_delta1__blk1442_dn6, locals.var_sp_s_delta1__blk1442_dn7, locals.var_sp_s_delta1__blk1442_dn8, ) = (assign49670_e64117, (-(locals.var_sp_s_delta0__blk1441_dn5 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn6 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn7 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn8 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), );
        }

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1472 != 0.0)) {
            let assign49680_e64133: f64 = (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441);
            (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8, ) = (assign49680_e64133, ((locals.var_delta_ns__blk1347_dn5 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn5)), ((locals.var_delta_ns__blk1347_dn6 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn6)), ((locals.var_delta_ns__blk1347_dn7 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn7)), ((locals.var_delta_ns__blk1347_dn8 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn8)), );
        }

        let assign49690_e64139: f64 = (locals.var_xn_s__blk1332 - 230.25850929940458);
        let assign49690_e64140: f64 = if locals.var_sp_s_x0__blk1455 > assign49690_e64139 { 1.0 } else { 0.0 };
        locals.var_guard1473 = assign49690_e64140;

        if ((((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1472 == 0.0)) && (locals.var_guard1473 != 0.0)) {
            let assign49700_e64157: f64 = (locals.var_sp_s_x0__blk1455 - locals.var_xn_s__blk1332);
            let assign49700_e64158: f64 = (assign49700_e64157).exp();
            (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8, ) = (assign49700_e64158, (assign49700_e64158 * (locals.var_sp_s_x0__blk1455_dn5 - locals.var_xn_s__blk1332_dn5)), (assign49700_e64158 * (locals.var_sp_s_x0__blk1455_dn6 - locals.var_xn_s__blk1332_dn6)), (assign49700_e64158 * (locals.var_sp_s_x0__blk1455_dn7 - locals.var_xn_s__blk1332_dn7)), (assign49700_e64158 * (locals.var_sp_s_x0__blk1455_dn8 - locals.var_xn_s__blk1332_dn8)), );
        }

        if ((((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1472 == 0.0)) && (locals.var_guard1473 != 0.0)) {
            let assign49710_e64177: f64 = (locals.var_delta_ns__blk1347 / locals.var_sp_s_delta0__blk1441);
            (locals.var_sp_s_delta1__blk1442, locals.var_sp_s_delta1__blk1442_dn5, locals.var_sp_s_delta1__blk1442_dn6, locals.var_sp_s_delta1__blk1442_dn7, locals.var_sp_s_delta1__blk1442_dn8, ) = (assign49710_e64177, (((locals.var_delta_ns__blk1347_dn5 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn5)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)), (((locals.var_delta_ns__blk1347_dn6 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn6)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)), (((locals.var_delta_ns__blk1347_dn7 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn7)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)), (((locals.var_delta_ns__blk1347_dn8 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_ns__blk1347 * locals.var_sp_s_delta0__blk1441_dn8)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)), );
        }

        if ((((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1472 == 0.0)) && (locals.var_guard1473 == 0.0)) {
            let assign49720_e64199: f64 = (locals.var_xn_s__blk1332 - locals.var_sp_s_x0__blk1455);
            let assign49720_e64201: f64 = (assign49720_e64199 - 230.25850929940458);
            let assign49720_e64206: f64 = (locals.var_xn_s__blk1332 - locals.var_sp_s_x0__blk1455);
            let assign49720_e64208: f64 = (assign49720_e64206 - 230.25850929940458);
            let assign49720_e64212: f64 = (locals.var_xn_s__blk1332 - locals.var_sp_s_x0__blk1455);
            let assign49720_e64214: f64 = (assign49720_e64212 - 230.25850929940458);
            let assign49720_e64216: f64 = (assign49720_e64214 * 0.3333333333333333);
            let assign49720_e64217: f64 = (1.0 + assign49720_e64216);
            let assign49720_e64218: f64 = (assign49720_e64208 * assign49720_e64217);
            let assign49720_e64219: f64 = (0.5 * assign49720_e64218);
            let assign49720_e64220: f64 = (1.0 + assign49720_e64219);
            let assign49720_e64221: f64 = (assign49720_e64201 * assign49720_e64220);
            let assign49720_e64222: f64 = (1.0 + assign49720_e64221);
            let assign49720_e64223: f64 = (1e-100 / assign49720_e64222);
            (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8, ) = (assign49720_e64223, (-((1e-100 * (((locals.var_xn_s__blk1332_dn5 - locals.var_sp_s_x0__blk1455_dn5) * assign49720_e64220) + (assign49720_e64201 * (0.5 * (((locals.var_xn_s__blk1332_dn5 - locals.var_sp_s_x0__blk1455_dn5) * assign49720_e64217) + (assign49720_e64208 * ((locals.var_xn_s__blk1332_dn5 - locals.var_sp_s_x0__blk1455_dn5) * 0.3333333333333333))))))) / (assign49720_e64222 * assign49720_e64222))), (-((1e-100 * (((locals.var_xn_s__blk1332_dn6 - locals.var_sp_s_x0__blk1455_dn6) * assign49720_e64220) + (assign49720_e64201 * (0.5 * (((locals.var_xn_s__blk1332_dn6 - locals.var_sp_s_x0__blk1455_dn6) * assign49720_e64217) + (assign49720_e64208 * ((locals.var_xn_s__blk1332_dn6 - locals.var_sp_s_x0__blk1455_dn6) * 0.3333333333333333))))))) / (assign49720_e64222 * assign49720_e64222))), (-((1e-100 * (((locals.var_xn_s__blk1332_dn7 - locals.var_sp_s_x0__blk1455_dn7) * assign49720_e64220) + (assign49720_e64201 * (0.5 * (((locals.var_xn_s__blk1332_dn7 - locals.var_sp_s_x0__blk1455_dn7) * assign49720_e64217) + (assign49720_e64208 * ((locals.var_xn_s__blk1332_dn7 - locals.var_sp_s_x0__blk1455_dn7) * 0.3333333333333333))))))) / (assign49720_e64222 * assign49720_e64222))), (-((1e-100 * (((locals.var_xn_s__blk1332_dn8 - locals.var_sp_s_x0__blk1455_dn8) * assign49720_e64220) + (assign49720_e64201 * (0.5 * (((locals.var_xn_s__blk1332_dn8 - locals.var_sp_s_x0__blk1455_dn8) * assign49720_e64217) + (assign49720_e64208 * ((locals.var_xn_s__blk1332_dn8 - locals.var_sp_s_x0__blk1455_dn8) * 0.3333333333333333))))))) / (assign49720_e64222 * assign49720_e64222))), );
        }

        if ((((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1472 == 0.0)) && (locals.var_guard1473 == 0.0)) {
            let assign49730_e64245: f64 = (locals.var_sp_s_x0__blk1455 - 230.25850929940458);
            let assign49730_e64250: f64 = (locals.var_sp_s_x0__blk1455 - 230.25850929940458);
            let assign49730_e64254: f64 = (locals.var_sp_s_x0__blk1455 - 230.25850929940458);
            let assign49730_e64256: f64 = (assign49730_e64254 * 0.3333333333333333);
            let assign49730_e64257: f64 = (1.0 + assign49730_e64256);
            let assign49730_e64258: f64 = (assign49730_e64250 * assign49730_e64257);
            let assign49730_e64259: f64 = (0.5 * assign49730_e64258);
            let assign49730_e64260: f64 = (1.0 + assign49730_e64259);
            let assign49730_e64261: f64 = (assign49730_e64245 * assign49730_e64260);
            let assign49730_e64262: f64 = (1.0 + assign49730_e64261);
            let assign49730_e64263: f64 = (1e-100 / assign49730_e64262);
            (locals.var_sp_s_delta1__blk1442, locals.var_sp_s_delta1__blk1442_dn5, locals.var_sp_s_delta1__blk1442_dn6, locals.var_sp_s_delta1__blk1442_dn7, locals.var_sp_s_delta1__blk1442_dn8, ) = (assign49730_e64263, (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn5 * assign49730_e64260) + (assign49730_e64245 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn5 * assign49730_e64257) + (assign49730_e64250 * (locals.var_sp_s_x0__blk1455_dn5 * 0.3333333333333333))))))) / (assign49730_e64262 * assign49730_e64262))), (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn6 * assign49730_e64260) + (assign49730_e64245 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn6 * assign49730_e64257) + (assign49730_e64250 * (locals.var_sp_s_x0__blk1455_dn6 * 0.3333333333333333))))))) / (assign49730_e64262 * assign49730_e64262))), (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn7 * assign49730_e64260) + (assign49730_e64245 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn7 * assign49730_e64257) + (assign49730_e64250 * (locals.var_sp_s_x0__blk1455_dn7 * 0.3333333333333333))))))) / (assign49730_e64262 * assign49730_e64262))), (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn8 * assign49730_e64260) + (assign49730_e64245 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn8 * assign49730_e64257) + (assign49730_e64250 * (locals.var_sp_s_x0__blk1455_dn8 * 0.3333333333333333))))))) / (assign49730_e64262 * assign49730_e64262))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49740_e64279: f64 = (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455);
            let assign49740_e64280: f64 = (2.0 + assign49740_e64279);
            let assign49740_e64281: f64 = (1.0 / assign49740_e64280);
            (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8, ) = (assign49740_e64281, (-(((locals.var_sp_s_x0__blk1455_dn5 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn5)) / (assign49740_e64280 * assign49740_e64280))), (-(((locals.var_sp_s_x0__blk1455_dn6 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn6)) / (assign49740_e64280 * assign49740_e64280))), (-(((locals.var_sp_s_x0__blk1455_dn7 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn7)) / (assign49740_e64280 * assign49740_e64280))), (-(((locals.var_sp_s_x0__blk1455_dn8 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn8)) / (assign49740_e64280 * assign49740_e64280))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49750_e64295: f64 = (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455);
            let assign49750_e64297: f64 = (assign49750_e64295 * locals.var_sp_s_temp__blk1431);
            (locals.var_sp_s_xi0__blk1443, locals.var_sp_s_xi0__blk1443_dn5, locals.var_sp_s_xi0__blk1443_dn6, locals.var_sp_s_xi0__blk1443_dn7, locals.var_sp_s_xi0__blk1443_dn8, ) = (assign49750_e64297, ((((locals.var_sp_s_x0__blk1455_dn5 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49750_e64295 * locals.var_sp_s_temp__blk1431_dn5)), ((((locals.var_sp_s_x0__blk1455_dn6 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49750_e64295 * locals.var_sp_s_temp__blk1431_dn6)), ((((locals.var_sp_s_x0__blk1455_dn7 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49750_e64295 * locals.var_sp_s_temp__blk1431_dn7)), ((((locals.var_sp_s_x0__blk1455_dn8 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49750_e64295 * locals.var_sp_s_temp__blk1431_dn8)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49760_e64312: f64 = (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431);
            let assign49760_e64314: f64 = (assign49760_e64312 * locals.var_sp_s_temp__blk1431);
            let assign49760_e64315: f64 = (4.0 * assign49760_e64314);
            (locals.var_sp_s_xi1__blk1444, locals.var_sp_s_xi1__blk1444_dn5, locals.var_sp_s_xi1__blk1444_dn6, locals.var_sp_s_xi1__blk1444_dn7, locals.var_sp_s_xi1__blk1444_dn8, ) = (assign49760_e64315, (4.0 * ((((locals.var_sp_s_x0__blk1455_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49760_e64312 * locals.var_sp_s_temp__blk1431_dn5))), (4.0 * ((((locals.var_sp_s_x0__blk1455_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49760_e64312 * locals.var_sp_s_temp__blk1431_dn6))), (4.0 * ((((locals.var_sp_s_x0__blk1455_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49760_e64312 * locals.var_sp_s_temp__blk1431_dn7))), (4.0 * ((((locals.var_sp_s_x0__blk1455_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49760_e64312 * locals.var_sp_s_temp__blk1431_dn8))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49770_e64329: f64 = (8.0 * locals.var_sp_s_temp__blk1431);
            let assign49770_e64332: f64 = (12.0 * locals.var_sp_s_xi0__blk1443);
            let assign49770_e64333: f64 = (assign49770_e64329 - assign49770_e64332);
            let assign49770_e64335: f64 = (assign49770_e64333 * locals.var_sp_s_temp__blk1431);
            let assign49770_e64337: f64 = (assign49770_e64335 * locals.var_sp_s_temp__blk1431);
            (locals.var_sp_s_xi2__blk1445, locals.var_sp_s_xi2__blk1445_dn5, locals.var_sp_s_xi2__blk1445_dn6, locals.var_sp_s_xi2__blk1445_dn7, locals.var_sp_s_xi2__blk1445_dn8, ) = (assign49770_e64337, ((((((8.0 * locals.var_sp_s_temp__blk1431_dn5) - (12.0 * locals.var_sp_s_xi0__blk1443_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64333 * locals.var_sp_s_temp__blk1431_dn5)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64335 * locals.var_sp_s_temp__blk1431_dn5)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn6) - (12.0 * locals.var_sp_s_xi0__blk1443_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64333 * locals.var_sp_s_temp__blk1431_dn6)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64335 * locals.var_sp_s_temp__blk1431_dn6)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn7) - (12.0 * locals.var_sp_s_xi0__blk1443_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64333 * locals.var_sp_s_temp__blk1431_dn7)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64335 * locals.var_sp_s_temp__blk1431_dn7)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn8) - (12.0 * locals.var_sp_s_xi0__blk1443_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64333 * locals.var_sp_s_temp__blk1431_dn8)) * locals.var_sp_s_temp__blk1431) + (assign49770_e64335 * locals.var_sp_s_temp__blk1431_dn8)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49780_e64351: f64 = (locals.var_xg__blk1326 - locals.var_sp_s_x0__blk1455);
            (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8, ) = (assign49780_e64351, (locals.var_xg__blk1326_dn5 - locals.var_sp_s_x0__blk1455_dn5), (locals.var_xg__blk1326_dn6 - locals.var_sp_s_x0__blk1455_dn6), (locals.var_xg__blk1326_dn7 - locals.var_sp_s_x0__blk1455_dn7), (locals.var_xg__blk1326_dn8 - locals.var_sp_s_x0__blk1455_dn8), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49790_e64365: f64 = (2.0 * locals.var_sp_s_temp__blk1431);
            let assign49790_e64369: f64 = (1.0 - locals.var_sp_s_delta1__blk1442);
            let assign49790_e64371: f64 = (assign49790_e64369 + locals.var_sp_s_delta0__blk1441);
            let assign49790_e64375: f64 = (1.0 + locals.var_sp_s_xi1__blk1444);
            let assign49790_e64376: f64 = (locals.var_delta_ns__blk1347 * assign49790_e64375);
            let assign49790_e64377: f64 = (assign49790_e64371 - assign49790_e64376);
            let assign49790_e64378: f64 = (locals.var_gf2__blk1308 * assign49790_e64377);
            let assign49790_e64379: f64 = (assign49790_e64365 + assign49790_e64378);
            (locals.var_sp_s_pc__blk1446, locals.var_sp_s_pc__blk1446_dn5, locals.var_sp_s_pc__blk1446_dn6, locals.var_sp_s_pc__blk1446_dn7, locals.var_sp_s_pc__blk1446_dn8, ) = (assign49790_e64379, ((2.0 * locals.var_sp_s_temp__blk1431_dn5) + ((locals.var_gf2__blk1308_dn5 * assign49790_e64377) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn5) + locals.var_sp_s_delta0__blk1441_dn5) - ((locals.var_delta_ns__blk1347_dn5 * assign49790_e64375) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn5)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn6) + ((locals.var_gf2__blk1308_dn6 * assign49790_e64377) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn6) + locals.var_sp_s_delta0__blk1441_dn6) - ((locals.var_delta_ns__blk1347_dn6 * assign49790_e64375) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn7) + ((locals.var_gf2__blk1308_dn7 * assign49790_e64377) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn7) + locals.var_sp_s_delta0__blk1441_dn7) - ((locals.var_delta_ns__blk1347_dn7 * assign49790_e64375) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn8) + ((locals.var_gf2__blk1308_dn8 * assign49790_e64377) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn8) + locals.var_sp_s_delta0__blk1441_dn8) - ((locals.var_delta_ns__blk1347_dn8 * assign49790_e64375) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi1__blk1444_dn8)))))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49800_e64393: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
            let assign49800_e64397: f64 = (locals.var_sp_s_delta1__blk1442 + locals.var_sp_s_x0__blk1455);
            let assign49800_e64399: f64 = (assign49800_e64397 - 1.0);
            let assign49800_e64401: f64 = (assign49800_e64399 + locals.var_sp_s_delta0__blk1441);
            let assign49800_e64405: f64 = (locals.var_sp_s_x0__blk1455 + 1.0);
            let assign49800_e64407: f64 = (assign49800_e64405 + locals.var_sp_s_xi0__blk1443);
            let assign49800_e64408: f64 = (locals.var_delta_ns__blk1347 * assign49800_e64407);
            let assign49800_e64409: f64 = (assign49800_e64401 - assign49800_e64408);
            let assign49800_e64410: f64 = (locals.var_gf2__blk1308 * assign49800_e64409);
            let assign49800_e64411: f64 = (assign49800_e64393 - assign49800_e64410);
            (locals.var_sp_s_qc__blk1447, locals.var_sp_s_qc__blk1447_dn5, locals.var_sp_s_qc__blk1447_dn6, locals.var_sp_s_qc__blk1447_dn7, locals.var_sp_s_qc__blk1447_dn8, ) = (assign49800_e64411, (((locals.var_sp_s_temp__blk1431_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn5)) - ((locals.var_gf2__blk1308_dn5 * assign49800_e64409) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn5 + locals.var_sp_s_x0__blk1455_dn5) + locals.var_sp_s_delta0__blk1441_dn5) - ((locals.var_delta_ns__blk1347_dn5 * assign49800_e64407) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_x0__blk1455_dn5 + locals.var_sp_s_xi0__blk1443_dn5))))))), (((locals.var_sp_s_temp__blk1431_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn6)) - ((locals.var_gf2__blk1308_dn6 * assign49800_e64409) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn6 + locals.var_sp_s_x0__blk1455_dn6) + locals.var_sp_s_delta0__blk1441_dn6) - ((locals.var_delta_ns__blk1347_dn6 * assign49800_e64407) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_x0__blk1455_dn6 + locals.var_sp_s_xi0__blk1443_dn6))))))), (((locals.var_sp_s_temp__blk1431_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn7)) - ((locals.var_gf2__blk1308_dn7 * assign49800_e64409) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn7 + locals.var_sp_s_x0__blk1455_dn7) + locals.var_sp_s_delta0__blk1441_dn7) - ((locals.var_delta_ns__blk1347_dn7 * assign49800_e64407) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_x0__blk1455_dn7 + locals.var_sp_s_xi0__blk1443_dn7))))))), (((locals.var_sp_s_temp__blk1431_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn8)) - ((locals.var_gf2__blk1308_dn8 * assign49800_e64409) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn8 + locals.var_sp_s_x0__blk1455_dn8) + locals.var_sp_s_delta0__blk1441_dn8) - ((locals.var_delta_ns__blk1347_dn8 * assign49800_e64407) + (locals.var_delta_ns__blk1347 * (locals.var_sp_s_x0__blk1455_dn8 + locals.var_sp_s_xi0__blk1443_dn8))))))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49810_e64427: f64 = (locals.var_sp_s_delta1__blk1442 + locals.var_sp_s_delta0__blk1441);
            let assign49810_e64430: f64 = (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445);
            let assign49810_e64431: f64 = (assign49810_e64427 - assign49810_e64430);
            let assign49810_e64432: f64 = (locals.var_gf2__blk1308 * assign49810_e64431);
            let assign49810_e64433: f64 = (2.0 - assign49810_e64432);
            (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8, ) = (assign49810_e64433, (-((locals.var_gf2__blk1308_dn5 * assign49810_e64431) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn5 + locals.var_sp_s_delta0__blk1441_dn5) - ((locals.var_delta_ns__blk1347_dn5 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn5)))))), (-((locals.var_gf2__blk1308_dn6 * assign49810_e64431) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn6 + locals.var_sp_s_delta0__blk1441_dn6) - ((locals.var_delta_ns__blk1347_dn6 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn6)))))), (-((locals.var_gf2__blk1308_dn7 * assign49810_e64431) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn7 + locals.var_sp_s_delta0__blk1441_dn7) - ((locals.var_delta_ns__blk1347_dn7 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn7)))))), (-((locals.var_gf2__blk1308_dn8 * assign49810_e64431) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn8 + locals.var_sp_s_delta0__blk1441_dn8) - ((locals.var_delta_ns__blk1347_dn8 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_ns__blk1347 * locals.var_sp_s_xi2__blk1445_dn8)))))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49820_e64447: f64 = (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446);
            let assign49820_e64451: f64 = (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431);
            let assign49820_e64452: f64 = (2.0 * assign49820_e64451);
            let assign49820_e64453: f64 = (assign49820_e64447 - assign49820_e64452);
            (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8, ) = (assign49820_e64453, (((locals.var_sp_s_pc__blk1446_dn5 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn5)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn5)))), (((locals.var_sp_s_pc__blk1446_dn6 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn6)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn6)))), (((locals.var_sp_s_pc__blk1446_dn7 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn7)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn7)))), (((locals.var_sp_s_pc__blk1446_dn8 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn8)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn8)))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1468 == 0.0)) && (locals.var_guard1469 == 0.0)) {
            let assign49830_e64470: f64 = (locals.var_sp_s_temp__blk1431).sqrt();
            let assign49830_e64471: f64 = (locals.var_sp_s_pc__blk1446 + assign49830_e64470);
            let assign49830_e64472: f64 = (locals.var_sp_s_qc__blk1447 / assign49830_e64471);
            let assign49830_e64473: f64 = (2.0 * assign49830_e64472);
            let assign49830_e64474: f64 = (locals.var_sp_s_x0__blk1455 + assign49830_e64473);
            (locals.var_x_s__blk1346, locals.var_x_s__blk1346_dn5, locals.var_x_s__blk1346_dn6, locals.var_x_s__blk1346_dn7, locals.var_x_s__blk1346_dn8, ) = (assign49830_e64474, (locals.var_sp_s_x0__blk1455_dn5 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn5 * assign49830_e64471) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn5 + (locals.var_sp_s_temp__blk1431_dn5 / (2.0 * assign49830_e64470))))) / (assign49830_e64471 * assign49830_e64471)))), (locals.var_sp_s_x0__blk1455_dn6 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn6 * assign49830_e64471) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn6 + (locals.var_sp_s_temp__blk1431_dn6 / (2.0 * assign49830_e64470))))) / (assign49830_e64471 * assign49830_e64471)))), (locals.var_sp_s_x0__blk1455_dn7 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn7 * assign49830_e64471) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn7 + (locals.var_sp_s_temp__blk1431_dn7 / (2.0 * assign49830_e64470))))) / (assign49830_e64471 * assign49830_e64471)))), (locals.var_sp_s_x0__blk1455_dn8 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn8 * assign49830_e64471) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn8 + (locals.var_sp_s_temp__blk1431_dn8 / (2.0 * assign49830_e64470))))) / (assign49830_e64471 * assign49830_e64471)))), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            (locals.var_xi1s__blk1349, locals.var_xi1s__blk1349_dn5, locals.var_xi1s__blk1349_dn6, locals.var_xi1s__blk1349_dn7, locals.var_xi1s__blk1349_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_xi2s__blk1350, locals.var_xi2s__blk1350_dn5, locals.var_xi2s__blk1350_dn6, locals.var_xi2s__blk1350_dn7, locals.var_xi2s__blk1350_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_delta_1s__blk1351, locals.var_delta_1s__blk1351_dn5, locals.var_delta_1s__blk1351_dn6, locals.var_delta_1s__blk1351_dn7, locals.var_delta_1s__blk1351_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_es__blk1352, locals.var_es__blk1352_dn5, locals.var_es__blk1352_dn6, locals.var_es__blk1352_dn7, locals.var_es__blk1352_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_ds__blk1353, locals.var_ds__blk1353_dn5, locals.var_ds__blk1353_dn6, locals.var_ds__blk1353_dn7, locals.var_ds__blk1353_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_ps__blk1354, locals.var_ps__blk1354_dn5, locals.var_ps__blk1354_dn6, locals.var_ps__blk1354_dn7, locals.var_ps__blk1354_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_sqs__blk1355, locals.var_sqs__blk1355_dn5, locals.var_sqs__blk1355_dn6, locals.var_sqs__blk1355_dn7, locals.var_sqs__blk1355_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_alphas__blk1356, locals.var_alphas__blk1356_dn5, locals.var_alphas__blk1356_dn6, locals.var_alphas__blk1356_dn7, locals.var_alphas__blk1356_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_rxcor__blk1357, locals.var_rxcor__blk1357_dn5, locals.var_rxcor__blk1357_dn6, locals.var_rxcor__blk1357_dn7, locals.var_rxcor__blk1357_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign49930_e64536: f64 = (locals.var_xg__blk1326 - locals.var_x_s__blk1346);
            (locals.var_xgs__blk1358, locals.var_xgs__blk1358_dn5, locals.var_xgs__blk1358_dn6, locals.var_xgs__blk1358_dn7, locals.var_xgs__blk1358_dn8, ) = (assign49930_e64536, (locals.var_xg__blk1326_dn5 - locals.var_x_s__blk1346_dn5), (locals.var_xg__blk1326_dn6 - locals.var_x_s__blk1346_dn6), (locals.var_xg__blk1326_dn7 - locals.var_x_s__blk1346_dn7), (locals.var_xg__blk1326_dn8 - locals.var_x_s__blk1346_dn8), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            (locals.var_qis__blk1359, locals.var_qis__blk1359_dn5, locals.var_qis__blk1359_dn6, locals.var_qis__blk1359_dn7, locals.var_qis__blk1359_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            let assign49950_e64550: f64 = (locals.var_phit1__blk1322 * locals.var_xgs__blk1358);
            (locals.var_qbs__blk1360, locals.var_qbs__blk1360_dn5, locals.var_qbs__blk1360_dn6, locals.var_qbs__blk1360_dn7, locals.var_qbs__blk1360_dn8, ) = (assign49950_e64550, ((locals.var_phit1__blk1322_dn5 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn5)), ((locals.var_phit1__blk1322_dn6 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn6)), ((locals.var_phit1__blk1322_dn7 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn7)), ((locals.var_phit1__blk1322_dn8 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn8)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) {
            (locals.var_rhob__blk1361, locals.var_rhob__blk1361_dn5, locals.var_rhob__blk1361_dn6, locals.var_rhob__blk1361_dn7, locals.var_rhob__blk1361_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_rhog__blk1362, locals.var_rhog__blk1362_dn5, locals.var_rhog__blk1362_dn6, locals.var_rhog__blk1362_dn7, locals.var_rhog__blk1362_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_gmobs__blk1366, locals.var_gmobs__blk1366_dn5, locals.var_gmobs__blk1366_dn6, locals.var_gmobs__blk1366_dn7, locals.var_gmobs__blk1366_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_xitsb__blk1367, locals.var_xitsb__blk1367_dn5, locals.var_xitsb__blk1367_dn6, locals.var_xitsb__blk1367_dn7, locals.var_xitsb__blk1367_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_factheta__blk1369, locals.var_factheta__blk1369_dn5, locals.var_factheta__blk1369_dn6, locals.var_factheta__blk1369_dn7, locals.var_factheta__blk1369_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign50010_e64585: f64 = if locals.var_xg__blk1326 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1474 = assign50010_e64585;

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) {
            let assign50020_e64595: f64 = (locals.var_x_s__blk1346 * locals.var_x_s__blk1346);
            let assign50020_e64596: f64 = (2.0 + assign50020_e64595);
            let assign50020_e64597: f64 = (1.0 / assign50020_e64596);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign50020_e64597, (-(((locals.var_x_s__blk1346_dn5 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn5)) / (assign50020_e64596 * assign50020_e64596))), (-(((locals.var_x_s__blk1346_dn6 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn6)) / (assign50020_e64596 * assign50020_e64596))), (-(((locals.var_x_s__blk1346_dn7 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn7)) / (assign50020_e64596 * assign50020_e64596))), (-(((locals.var_x_s__blk1346_dn8 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn8)) / (assign50020_e64596 * assign50020_e64596))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) {
            let assign50030_e64607: f64 = (locals.var_x_s__blk1346 * locals.var_x_s__blk1346);
            let assign50030_e64609: f64 = (assign50030_e64607 * locals.var_temp__blk936);
            (locals.var_xi0s__blk1348, locals.var_xi0s__blk1348_dn5, locals.var_xi0s__blk1348_dn6, locals.var_xi0s__blk1348_dn7, locals.var_xi0s__blk1348_dn8, ) = (assign50030_e64609, ((((locals.var_x_s__blk1346_dn5 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn5)) * locals.var_temp__blk936) + (assign50030_e64607 * locals.var_temp__blk936_dn5)), ((((locals.var_x_s__blk1346_dn6 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn6)) * locals.var_temp__blk936) + (assign50030_e64607 * locals.var_temp__blk936_dn6)), ((((locals.var_x_s__blk1346_dn7 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn7)) * locals.var_temp__blk936) + (assign50030_e64607 * locals.var_temp__blk936_dn7)), ((((locals.var_x_s__blk1346_dn8 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn8)) * locals.var_temp__blk936) + (assign50030_e64607 * locals.var_temp__blk936_dn8)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) {
            let assign50040_e64620: f64 = (locals.var_x_s__blk1346 * locals.var_temp__blk936);
            let assign50040_e64622: f64 = (assign50040_e64620 * locals.var_temp__blk936);
            let assign50040_e64623: f64 = (4.0 * assign50040_e64622);
            (locals.var_xi1s__blk1349, locals.var_xi1s__blk1349_dn5, locals.var_xi1s__blk1349_dn6, locals.var_xi1s__blk1349_dn7, locals.var_xi1s__blk1349_dn8, ) = (assign50040_e64623, (4.0 * ((((locals.var_x_s__blk1346_dn5 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn5)) * locals.var_temp__blk936) + (assign50040_e64620 * locals.var_temp__blk936_dn5))), (4.0 * ((((locals.var_x_s__blk1346_dn6 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn6)) * locals.var_temp__blk936) + (assign50040_e64620 * locals.var_temp__blk936_dn6))), (4.0 * ((((locals.var_x_s__blk1346_dn7 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn7)) * locals.var_temp__blk936) + (assign50040_e64620 * locals.var_temp__blk936_dn7))), (4.0 * ((((locals.var_x_s__blk1346_dn8 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn8)) * locals.var_temp__blk936) + (assign50040_e64620 * locals.var_temp__blk936_dn8))), );
        }

    }

    pub(super) fn stamp_transient_block_22(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) {
            let assign50050_e64633: f64 = (8.0 * locals.var_temp__blk936);
            let assign50050_e64636: f64 = (12.0 * locals.var_xi0s__blk1348);
            let assign50050_e64637: f64 = (assign50050_e64633 - assign50050_e64636);
            let assign50050_e64639: f64 = (assign50050_e64637 * locals.var_temp__blk936);
            let assign50050_e64641: f64 = (assign50050_e64639 * locals.var_temp__blk936);
            (locals.var_xi2s__blk1350, locals.var_xi2s__blk1350_dn5, locals.var_xi2s__blk1350_dn6, locals.var_xi2s__blk1350_dn7, locals.var_xi2s__blk1350_dn8, ) = (assign50050_e64641, ((((((8.0 * locals.var_temp__blk936_dn5) - (12.0 * locals.var_xi0s__blk1348_dn5)) * locals.var_temp__blk936) + (assign50050_e64637 * locals.var_temp__blk936_dn5)) * locals.var_temp__blk936) + (assign50050_e64639 * locals.var_temp__blk936_dn5)), ((((((8.0 * locals.var_temp__blk936_dn6) - (12.0 * locals.var_xi0s__blk1348_dn6)) * locals.var_temp__blk936) + (assign50050_e64637 * locals.var_temp__blk936_dn6)) * locals.var_temp__blk936) + (assign50050_e64639 * locals.var_temp__blk936_dn6)), ((((((8.0 * locals.var_temp__blk936_dn7) - (12.0 * locals.var_xi0s__blk1348_dn7)) * locals.var_temp__blk936) + (assign50050_e64637 * locals.var_temp__blk936_dn7)) * locals.var_temp__blk936) + (assign50050_e64639 * locals.var_temp__blk936_dn7)), ((((((8.0 * locals.var_temp__blk936_dn8) - (12.0 * locals.var_xi0s__blk1348_dn8)) * locals.var_temp__blk936) + (assign50050_e64637 * locals.var_temp__blk936_dn8)) * locals.var_temp__blk936) + (assign50050_e64639 * locals.var_temp__blk936_dn8)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) {
            (locals.var_delta_1s__blk1351, locals.var_delta_1s__blk1351_dn5, locals.var_delta_1s__blk1351_dn6, locals.var_delta_1s__blk1351_dn7, locals.var_delta_1s__blk1351_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign50070_e64654: f64 = if locals.var_x_s__blk1346 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1475 = assign50070_e64654;

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
            let assign50080_e64663: f64 = (locals.var_x_s__blk1346).exp();
            (locals.var_delta_1s__blk1351, locals.var_delta_1s__blk1351_dn5, locals.var_delta_1s__blk1351_dn6, locals.var_delta_1s__blk1351_dn7, locals.var_delta_1s__blk1351_dn8, ) = (assign50080_e64663, (assign50080_e64663 * locals.var_x_s__blk1346_dn5), (assign50080_e64663 * locals.var_x_s__blk1346_dn6), (assign50080_e64663 * locals.var_x_s__blk1346_dn7), (assign50080_e64663 * locals.var_x_s__blk1346_dn8), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
            let assign50090_e64675: f64 = (1.0 / locals.var_delta_1s__blk1351);
            (locals.var_es__blk1352, locals.var_es__blk1352_dn5, locals.var_es__blk1352_dn6, locals.var_es__blk1352_dn7, locals.var_es__blk1352_dn8, ) = (assign50090_e64675, (-(locals.var_delta_1s__blk1351_dn5 / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351))), (-(locals.var_delta_1s__blk1351_dn6 / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351))), (-(locals.var_delta_1s__blk1351_dn7 / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351))), (-(locals.var_delta_1s__blk1351_dn8 / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
            let assign50100_e64687: f64 = (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351);
            (locals.var_delta_1s__blk1351, locals.var_delta_1s__blk1351_dn5, locals.var_delta_1s__blk1351_dn6, locals.var_delta_1s__blk1351_dn7, locals.var_delta_1s__blk1351_dn8, ) = (assign50100_e64687, ((locals.var_delta_ns__blk1347_dn5 * locals.var_delta_1s__blk1351) + (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn5)), ((locals.var_delta_ns__blk1347_dn6 * locals.var_delta_1s__blk1351) + (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn6)), ((locals.var_delta_ns__blk1347_dn7 * locals.var_delta_1s__blk1351) + (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn7)), ((locals.var_delta_ns__blk1347_dn8 * locals.var_delta_1s__blk1351) + (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn8)), );
        }

        let assign50110_e64693: f64 = (locals.var_xn_s__blk1332 - 230.25850929940458);
        let assign50110_e64694: f64 = if locals.var_x_s__blk1346 > assign50110_e64693 { 1.0 } else { 0.0 };
        locals.var_guard1476 = assign50110_e64694;

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 == 0.0)) && (locals.var_guard1476 != 0.0)) {
            let assign50120_e64707: f64 = (locals.var_x_s__blk1346 - locals.var_xn_s__blk1332);
            let assign50120_e64708: f64 = (assign50120_e64707).exp();
            (locals.var_delta_1s__blk1351, locals.var_delta_1s__blk1351_dn5, locals.var_delta_1s__blk1351_dn6, locals.var_delta_1s__blk1351_dn7, locals.var_delta_1s__blk1351_dn8, ) = (assign50120_e64708, (assign50120_e64708 * (locals.var_x_s__blk1346_dn5 - locals.var_xn_s__blk1332_dn5)), (assign50120_e64708 * (locals.var_x_s__blk1346_dn6 - locals.var_xn_s__blk1332_dn6)), (assign50120_e64708 * (locals.var_x_s__blk1346_dn7 - locals.var_xn_s__blk1332_dn7)), (assign50120_e64708 * (locals.var_x_s__blk1346_dn8 - locals.var_xn_s__blk1332_dn8)), );
        }

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 == 0.0)) && (locals.var_guard1476 != 0.0)) {
            let assign50130_e64723: f64 = (locals.var_delta_ns__blk1347 / locals.var_delta_1s__blk1351);
            (locals.var_es__blk1352, locals.var_es__blk1352_dn5, locals.var_es__blk1352_dn6, locals.var_es__blk1352_dn7, locals.var_es__blk1352_dn8, ) = (assign50130_e64723, (((locals.var_delta_ns__blk1347_dn5 * locals.var_delta_1s__blk1351) - (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn5)) / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351)), (((locals.var_delta_ns__blk1347_dn6 * locals.var_delta_1s__blk1351) - (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn6)) / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351)), (((locals.var_delta_ns__blk1347_dn7 * locals.var_delta_1s__blk1351) - (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn7)) / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351)), (((locals.var_delta_ns__blk1347_dn8 * locals.var_delta_1s__blk1351) - (locals.var_delta_ns__blk1347 * locals.var_delta_1s__blk1351_dn8)) / (locals.var_delta_1s__blk1351 * locals.var_delta_1s__blk1351)), );
        }

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 == 0.0)) && (locals.var_guard1476 == 0.0)) {
            let assign50140_e64741: f64 = (locals.var_xn_s__blk1332 - locals.var_x_s__blk1346);
            let assign50140_e64743: f64 = (assign50140_e64741 - 230.25850929940458);
            let assign50140_e64748: f64 = (locals.var_xn_s__blk1332 - locals.var_x_s__blk1346);
            let assign50140_e64750: f64 = (assign50140_e64748 - 230.25850929940458);
            let assign50140_e64754: f64 = (locals.var_xn_s__blk1332 - locals.var_x_s__blk1346);
            let assign50140_e64756: f64 = (assign50140_e64754 - 230.25850929940458);
            let assign50140_e64758: f64 = (assign50140_e64756 * 0.3333333333333333);
            let assign50140_e64759: f64 = (1.0 + assign50140_e64758);
            let assign50140_e64760: f64 = (assign50140_e64750 * assign50140_e64759);
            let assign50140_e64761: f64 = (0.5 * assign50140_e64760);
            let assign50140_e64762: f64 = (1.0 + assign50140_e64761);
            let assign50140_e64763: f64 = (assign50140_e64743 * assign50140_e64762);
            let assign50140_e64764: f64 = (1.0 + assign50140_e64763);
            let assign50140_e64765: f64 = (1e-100 / assign50140_e64764);
            (locals.var_delta_1s__blk1351, locals.var_delta_1s__blk1351_dn5, locals.var_delta_1s__blk1351_dn6, locals.var_delta_1s__blk1351_dn7, locals.var_delta_1s__blk1351_dn8, ) = (assign50140_e64765, (-((1e-100 * (((locals.var_xn_s__blk1332_dn5 - locals.var_x_s__blk1346_dn5) * assign50140_e64762) + (assign50140_e64743 * (0.5 * (((locals.var_xn_s__blk1332_dn5 - locals.var_x_s__blk1346_dn5) * assign50140_e64759) + (assign50140_e64750 * ((locals.var_xn_s__blk1332_dn5 - locals.var_x_s__blk1346_dn5) * 0.3333333333333333))))))) / (assign50140_e64764 * assign50140_e64764))), (-((1e-100 * (((locals.var_xn_s__blk1332_dn6 - locals.var_x_s__blk1346_dn6) * assign50140_e64762) + (assign50140_e64743 * (0.5 * (((locals.var_xn_s__blk1332_dn6 - locals.var_x_s__blk1346_dn6) * assign50140_e64759) + (assign50140_e64750 * ((locals.var_xn_s__blk1332_dn6 - locals.var_x_s__blk1346_dn6) * 0.3333333333333333))))))) / (assign50140_e64764 * assign50140_e64764))), (-((1e-100 * (((locals.var_xn_s__blk1332_dn7 - locals.var_x_s__blk1346_dn7) * assign50140_e64762) + (assign50140_e64743 * (0.5 * (((locals.var_xn_s__blk1332_dn7 - locals.var_x_s__blk1346_dn7) * assign50140_e64759) + (assign50140_e64750 * ((locals.var_xn_s__blk1332_dn7 - locals.var_x_s__blk1346_dn7) * 0.3333333333333333))))))) / (assign50140_e64764 * assign50140_e64764))), (-((1e-100 * (((locals.var_xn_s__blk1332_dn8 - locals.var_x_s__blk1346_dn8) * assign50140_e64762) + (assign50140_e64743 * (0.5 * (((locals.var_xn_s__blk1332_dn8 - locals.var_x_s__blk1346_dn8) * assign50140_e64759) + (assign50140_e64750 * ((locals.var_xn_s__blk1332_dn8 - locals.var_x_s__blk1346_dn8) * 0.3333333333333333))))))) / (assign50140_e64764 * assign50140_e64764))), );
        }

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 == 0.0)) && (locals.var_guard1476 == 0.0)) {
            let assign50150_e64783: f64 = (locals.var_x_s__blk1346 - 230.25850929940458);
            let assign50150_e64788: f64 = (locals.var_x_s__blk1346 - 230.25850929940458);
            let assign50150_e64792: f64 = (locals.var_x_s__blk1346 - 230.25850929940458);
            let assign50150_e64794: f64 = (assign50150_e64792 * 0.3333333333333333);
            let assign50150_e64795: f64 = (1.0 + assign50150_e64794);
            let assign50150_e64796: f64 = (assign50150_e64788 * assign50150_e64795);
            let assign50150_e64797: f64 = (0.5 * assign50150_e64796);
            let assign50150_e64798: f64 = (1.0 + assign50150_e64797);
            let assign50150_e64799: f64 = (assign50150_e64783 * assign50150_e64798);
            let assign50150_e64800: f64 = (1.0 + assign50150_e64799);
            let assign50150_e64801: f64 = (1e-100 / assign50150_e64800);
            (locals.var_es__blk1352, locals.var_es__blk1352_dn5, locals.var_es__blk1352_dn6, locals.var_es__blk1352_dn7, locals.var_es__blk1352_dn8, ) = (assign50150_e64801, (-((1e-100 * ((locals.var_x_s__blk1346_dn5 * assign50150_e64798) + (assign50150_e64783 * (0.5 * ((locals.var_x_s__blk1346_dn5 * assign50150_e64795) + (assign50150_e64788 * (locals.var_x_s__blk1346_dn5 * 0.3333333333333333))))))) / (assign50150_e64800 * assign50150_e64800))), (-((1e-100 * ((locals.var_x_s__blk1346_dn6 * assign50150_e64798) + (assign50150_e64783 * (0.5 * ((locals.var_x_s__blk1346_dn6 * assign50150_e64795) + (assign50150_e64788 * (locals.var_x_s__blk1346_dn6 * 0.3333333333333333))))))) / (assign50150_e64800 * assign50150_e64800))), (-((1e-100 * ((locals.var_x_s__blk1346_dn7 * assign50150_e64798) + (assign50150_e64783 * (0.5 * ((locals.var_x_s__blk1346_dn7 * assign50150_e64795) + (assign50150_e64788 * (locals.var_x_s__blk1346_dn7 * 0.3333333333333333))))))) / (assign50150_e64800 * assign50150_e64800))), (-((1e-100 * ((locals.var_x_s__blk1346_dn8 * assign50150_e64798) + (assign50150_e64783 * (0.5 * ((locals.var_x_s__blk1346_dn8 * assign50150_e64795) + (assign50150_e64788 * (locals.var_x_s__blk1346_dn8 * 0.3333333333333333))))))) / (assign50150_e64800 * assign50150_e64800))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) {
            let assign50160_e64813: f64 = (locals.var_x_s__blk1346 + 1.0);
            let assign50160_e64815: f64 = (assign50160_e64813 + locals.var_xi0s__blk1348);
            let assign50160_e64816: f64 = (locals.var_delta_ns__blk1347 * assign50160_e64815);
            let assign50160_e64817: f64 = (locals.var_delta_1s__blk1351 - assign50160_e64816);
            (locals.var_ds__blk1353, locals.var_ds__blk1353_dn5, locals.var_ds__blk1353_dn6, locals.var_ds__blk1353_dn7, locals.var_ds__blk1353_dn8, ) = (assign50160_e64817, (locals.var_delta_1s__blk1351_dn5 - ((locals.var_delta_ns__blk1347_dn5 * assign50160_e64815) + (locals.var_delta_ns__blk1347 * (locals.var_x_s__blk1346_dn5 + locals.var_xi0s__blk1348_dn5)))), (locals.var_delta_1s__blk1351_dn6 - ((locals.var_delta_ns__blk1347_dn6 * assign50160_e64815) + (locals.var_delta_ns__blk1347 * (locals.var_x_s__blk1346_dn6 + locals.var_xi0s__blk1348_dn6)))), (locals.var_delta_1s__blk1351_dn7 - ((locals.var_delta_ns__blk1347_dn7 * assign50160_e64815) + (locals.var_delta_ns__blk1347 * (locals.var_x_s__blk1346_dn7 + locals.var_xi0s__blk1348_dn7)))), (locals.var_delta_1s__blk1351_dn8 - ((locals.var_delta_ns__blk1347_dn8 * assign50160_e64815) + (locals.var_delta_ns__blk1347 * (locals.var_x_s__blk1346_dn8 + locals.var_xi0s__blk1348_dn8)))), );
        }

        let assign50170_e64822: f64 = if locals.var_x_s__blk1346 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1477 = assign50170_e64822;

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 != 0.0)) {
            let assign50180_e64833: f64 = (locals.var_x_s__blk1346 * locals.var_x_s__blk1346);
            let assign50180_e64840: f64 = (0.25 * locals.var_x_s__blk1346);
            let assign50180_e64841: f64 = (1.0 - assign50180_e64840);
            let assign50180_e64842: f64 = (locals.var_x_s__blk1346 * assign50180_e64841);
            let assign50180_e64843: f64 = (0.3333333333333333 * assign50180_e64842);
            let assign50180_e64844: f64 = (1.0 - assign50180_e64843);
            let assign50180_e64845: f64 = (assign50180_e64833 * assign50180_e64844);
            let assign50180_e64846: f64 = (0.5 * assign50180_e64845);
            (locals.var_ps__blk1354, locals.var_ps__blk1354_dn5, locals.var_ps__blk1354_dn6, locals.var_ps__blk1354_dn7, locals.var_ps__blk1354_dn8, ) = (assign50180_e64846, (0.5 * ((((locals.var_x_s__blk1346_dn5 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn5)) * assign50180_e64844) + (assign50180_e64833 * (-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn5 * assign50180_e64841) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn5))))))))), (0.5 * ((((locals.var_x_s__blk1346_dn6 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn6)) * assign50180_e64844) + (assign50180_e64833 * (-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn6 * assign50180_e64841) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn6))))))))), (0.5 * ((((locals.var_x_s__blk1346_dn7 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn7)) * assign50180_e64844) + (assign50180_e64833 * (-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn7 * assign50180_e64841) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn7))))))))), (0.5 * ((((locals.var_x_s__blk1346_dn8 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn8)) * assign50180_e64844) + (assign50180_e64833 * (-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn8 * assign50180_e64841) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn8))))))))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 != 0.0)) {
            let assign50190_e64859: f64 = (locals.var_delta_ns__blk1347 * locals.var_x_s__blk1346);
            let assign50190_e64861: f64 = (assign50190_e64859 * locals.var_x_s__blk1346);
            let assign50190_e64863: f64 = (assign50190_e64861 * locals.var_x_s__blk1346);
            let assign50190_e64867: f64 = (1.75 * locals.var_x_s__blk1346);
            let assign50190_e64868: f64 = (1.0 + assign50190_e64867);
            let assign50190_e64869: f64 = (assign50190_e64863 * assign50190_e64868);
            let assign50190_e64870: f64 = (0.16666666666666666 * assign50190_e64869);
            (locals.var_ds__blk1353, locals.var_ds__blk1353_dn5, locals.var_ds__blk1353_dn6, locals.var_ds__blk1353_dn7, locals.var_ds__blk1353_dn8, ) = (assign50190_e64870, (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1347_dn5 * locals.var_x_s__blk1346) + (locals.var_delta_ns__blk1347 * locals.var_x_s__blk1346_dn5)) * locals.var_x_s__blk1346) + (assign50190_e64859 * locals.var_x_s__blk1346_dn5)) * locals.var_x_s__blk1346) + (assign50190_e64861 * locals.var_x_s__blk1346_dn5)) * assign50190_e64868) + (assign50190_e64863 * (1.75 * locals.var_x_s__blk1346_dn5)))), (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1347_dn6 * locals.var_x_s__blk1346) + (locals.var_delta_ns__blk1347 * locals.var_x_s__blk1346_dn6)) * locals.var_x_s__blk1346) + (assign50190_e64859 * locals.var_x_s__blk1346_dn6)) * locals.var_x_s__blk1346) + (assign50190_e64861 * locals.var_x_s__blk1346_dn6)) * assign50190_e64868) + (assign50190_e64863 * (1.75 * locals.var_x_s__blk1346_dn6)))), (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1347_dn7 * locals.var_x_s__blk1346) + (locals.var_delta_ns__blk1347 * locals.var_x_s__blk1346_dn7)) * locals.var_x_s__blk1346) + (assign50190_e64859 * locals.var_x_s__blk1346_dn7)) * locals.var_x_s__blk1346) + (assign50190_e64861 * locals.var_x_s__blk1346_dn7)) * assign50190_e64868) + (assign50190_e64863 * (1.75 * locals.var_x_s__blk1346_dn7)))), (0.16666666666666666 * ((((((((locals.var_delta_ns__blk1347_dn8 * locals.var_x_s__blk1346) + (locals.var_delta_ns__blk1347 * locals.var_x_s__blk1346_dn8)) * locals.var_x_s__blk1346) + (assign50190_e64859 * locals.var_x_s__blk1346_dn8)) * locals.var_x_s__blk1346) + (assign50190_e64861 * locals.var_x_s__blk1346_dn8)) * assign50190_e64868) + (assign50190_e64863 * (1.75 * locals.var_x_s__blk1346_dn8)))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 != 0.0)) {
            let assign50200_e64886: f64 = (0.25 * locals.var_x_s__blk1346);
            let assign50200_e64887: f64 = (1.0 - assign50200_e64886);
            let assign50200_e64888: f64 = (locals.var_x_s__blk1346 * assign50200_e64887);
            let assign50200_e64889: f64 = (0.3333333333333333 * assign50200_e64888);
            let assign50200_e64890: f64 = (1.0 - assign50200_e64889);
            let assign50200_e64891: f64 = (assign50200_e64890).sqrt();
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign50200_e64891, ((-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn5 * assign50200_e64887) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn5)))))) / (2.0 * assign50200_e64891)), ((-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn6 * assign50200_e64887) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn6)))))) / (2.0 * assign50200_e64891)), ((-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn7 * assign50200_e64887) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn7)))))) / (2.0 * assign50200_e64891)), ((-(0.3333333333333333 * ((locals.var_x_s__blk1346_dn8 * assign50200_e64887) + (locals.var_x_s__blk1346 * (-(0.25 * locals.var_x_s__blk1346_dn8)))))) / (2.0 * assign50200_e64891)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 != 0.0)) {
            let assign50210_e64904: f64 = (locals.var_x_s__blk1346 * locals.var_temp__blk936);
            let assign50210_e64905: f64 = (0.7071067811865475 * assign50210_e64904);
            (locals.var_sqs__blk1355, locals.var_sqs__blk1355_dn5, locals.var_sqs__blk1355_dn6, locals.var_sqs__blk1355_dn7, locals.var_sqs__blk1355_dn8, ) = (assign50210_e64905, (0.7071067811865475 * ((locals.var_x_s__blk1346_dn5 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn5))), (0.7071067811865475 * ((locals.var_x_s__blk1346_dn6 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn6))), (0.7071067811865475 * ((locals.var_x_s__blk1346_dn7 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn7))), (0.7071067811865475 * ((locals.var_x_s__blk1346_dn8 * locals.var_temp__blk936) + (locals.var_x_s__blk1346 * locals.var_temp__blk936_dn8))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 != 0.0)) {
            let assign50220_e64921: f64 = (0.5 * locals.var_x_s__blk1346);
            let assign50220_e64922: f64 = (1.0 - assign50220_e64921);
            let assign50220_e64926: f64 = (locals.var_x_s__blk1346 * locals.var_x_s__blk1346);
            let assign50220_e64927: f64 = (0.16666666666666666 * assign50220_e64926);
            let assign50220_e64928: f64 = (assign50220_e64922 + assign50220_e64927);
            let assign50220_e64929: f64 = (locals.var_gf__blk1307 * assign50220_e64928);
            let assign50220_e64931: f64 = (assign50220_e64929 / locals.var_temp__blk936);
            let assign50220_e64932: f64 = (0.7071067811865475 * assign50220_e64931);
            let assign50220_e64933: f64 = (1.0 + assign50220_e64932);
            (locals.var_alphas__blk1356, locals.var_alphas__blk1356_dn5, locals.var_alphas__blk1356_dn6, locals.var_alphas__blk1356_dn7, locals.var_alphas__blk1356_dn8, ) = (assign50220_e64933, (0.7071067811865475 * (((((locals.var_gf__blk1307_dn5 * assign50220_e64928) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_s__blk1346_dn5)) + (0.16666666666666666 * ((locals.var_x_s__blk1346_dn5 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn5)))))) * locals.var_temp__blk936) - (assign50220_e64929 * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (0.7071067811865475 * (((((locals.var_gf__blk1307_dn6 * assign50220_e64928) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_s__blk1346_dn6)) + (0.16666666666666666 * ((locals.var_x_s__blk1346_dn6 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn6)))))) * locals.var_temp__blk936) - (assign50220_e64929 * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (0.7071067811865475 * (((((locals.var_gf__blk1307_dn7 * assign50220_e64928) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_s__blk1346_dn7)) + (0.16666666666666666 * ((locals.var_x_s__blk1346_dn7 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn7)))))) * locals.var_temp__blk936) - (assign50220_e64929 * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (0.7071067811865475 * (((((locals.var_gf__blk1307_dn8 * assign50220_e64928) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_s__blk1346_dn8)) + (0.16666666666666666 * ((locals.var_x_s__blk1346_dn8 * locals.var_x_s__blk1346) + (locals.var_x_s__blk1346 * locals.var_x_s__blk1346_dn8)))))) * locals.var_temp__blk936) - (assign50220_e64929 * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 == 0.0)) {
            let assign50230_e64946: f64 = (locals.var_x_s__blk1346 - 1.0);
            let assign50230_e64948: f64 = (assign50230_e64946 + locals.var_es__blk1352);
            (locals.var_ps__blk1354, locals.var_ps__blk1354_dn5, locals.var_ps__blk1354_dn6, locals.var_ps__blk1354_dn7, locals.var_ps__blk1354_dn8, ) = (assign50230_e64948, (locals.var_x_s__blk1346_dn5 + locals.var_es__blk1352_dn5), (locals.var_x_s__blk1346_dn6 + locals.var_es__blk1352_dn6), (locals.var_x_s__blk1346_dn7 + locals.var_es__blk1352_dn7), (locals.var_x_s__blk1346_dn8 + locals.var_es__blk1352_dn8), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 == 0.0)) {
            let assign50240_e64960: f64 = (locals.var_ps__blk1354).sqrt();
            (locals.var_sqs__blk1355, locals.var_sqs__blk1355_dn5, locals.var_sqs__blk1355_dn6, locals.var_sqs__blk1355_dn7, locals.var_sqs__blk1355_dn8, ) = (assign50240_e64960, (locals.var_ps__blk1354_dn5 / (2.0 * assign50240_e64960)), (locals.var_ps__blk1354_dn6 / (2.0 * assign50240_e64960)), (locals.var_ps__blk1354_dn7 / (2.0 * assign50240_e64960)), (locals.var_ps__blk1354_dn8 / (2.0 * assign50240_e64960)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1477 == 0.0)) {
            let assign50250_e64976: f64 = (1.0 - locals.var_es__blk1352);
            let assign50250_e64977: f64 = (locals.var_gf__blk1307 * assign50250_e64976);
            let assign50250_e64979: f64 = (assign50250_e64977 / locals.var_sqs__blk1355);
            let assign50250_e64980: f64 = (0.5 * assign50250_e64979);
            let assign50250_e64981: f64 = (1.0 + assign50250_e64980);
            (locals.var_alphas__blk1356, locals.var_alphas__blk1356_dn5, locals.var_alphas__blk1356_dn6, locals.var_alphas__blk1356_dn7, locals.var_alphas__blk1356_dn8, ) = (assign50250_e64981, (0.5 * (((((locals.var_gf__blk1307_dn5 * assign50250_e64976) + (locals.var_gf__blk1307 * (-locals.var_es__blk1352_dn5))) * locals.var_sqs__blk1355) - (assign50250_e64977 * locals.var_sqs__blk1355_dn5)) / (locals.var_sqs__blk1355 * locals.var_sqs__blk1355))), (0.5 * (((((locals.var_gf__blk1307_dn6 * assign50250_e64976) + (locals.var_gf__blk1307 * (-locals.var_es__blk1352_dn6))) * locals.var_sqs__blk1355) - (assign50250_e64977 * locals.var_sqs__blk1355_dn6)) / (locals.var_sqs__blk1355 * locals.var_sqs__blk1355))), (0.5 * (((((locals.var_gf__blk1307_dn7 * assign50250_e64976) + (locals.var_gf__blk1307 * (-locals.var_es__blk1352_dn7))) * locals.var_sqs__blk1355) - (assign50250_e64977 * locals.var_sqs__blk1355_dn7)) / (locals.var_sqs__blk1355 * locals.var_sqs__blk1355))), (0.5 * (((((locals.var_gf__blk1307_dn8 * assign50250_e64976) + (locals.var_gf__blk1307 * (-locals.var_es__blk1352_dn8))) * locals.var_sqs__blk1355) - (assign50250_e64977 * locals.var_sqs__blk1355_dn8)) / (locals.var_sqs__blk1355 * locals.var_sqs__blk1355))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) {
            let assign50260_e64992: f64 = (0.2 * locals.var_xcor_t);
            let assign50260_e64994: f64 = (assign50260_e64992 * locals.var_vsbx__blk1306);
            let assign50260_e64995: f64 = (1.0 + assign50260_e64994);
            let assign50260_e64999: f64 = (locals.var_xcor_t * locals.var_vsbx__blk1306);
            let assign50260_e65000: f64 = (1.0 + assign50260_e64999);
            let assign50260_e65001: f64 = (assign50260_e64995 / assign50260_e65000);
            (locals.var_rxcor__blk1357, locals.var_rxcor__blk1357_dn5, locals.var_rxcor__blk1357_dn6, locals.var_rxcor__blk1357_dn7, locals.var_rxcor__blk1357_dn8, ) = (assign50260_e65001, ((((assign50260_e64992 * locals.var_vsbx__blk1306_dn5) * assign50260_e65000) - (assign50260_e64995 * (locals.var_xcor_t * locals.var_vsbx__blk1306_dn5))) / (assign50260_e65000 * assign50260_e65000)), ((((assign50260_e64992 * locals.var_vsbx__blk1306_dn6) * assign50260_e65000) - (assign50260_e64995 * (locals.var_xcor_t * locals.var_vsbx__blk1306_dn6))) / (assign50260_e65000 * assign50260_e65000)), ((((assign50260_e64992 * locals.var_vsbx__blk1306_dn7) * assign50260_e65000) - (assign50260_e64995 * (locals.var_xcor_t * locals.var_vsbx__blk1306_dn7))) / (assign50260_e65000 * assign50260_e65000)), ((((assign50260_e64992 * locals.var_vsbx__blk1306_dn8) * assign50260_e65000) - (assign50260_e64995 * (locals.var_xcor_t * locals.var_vsbx__blk1306_dn8))) / (assign50260_e65000 * assign50260_e65000)), );
        }

        let assign50270_e65006: f64 = if locals.var_ds__blk1353 > 1e-100 { 1.0 } else { 0.0 };
        locals.var_guard1478 = assign50270_e65006;

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
            let assign50280_e65017: f64 = (locals.var_ps__blk1354 + locals.var_ds__blk1353);
            let assign50280_e65018: f64 = (assign50280_e65017).sqrt();
            let assign50280_e65019: f64 = (locals.var_gf__blk1307 * assign50280_e65018);
            (locals.var_xgs__blk1358, locals.var_xgs__blk1358_dn5, locals.var_xgs__blk1358_dn6, locals.var_xgs__blk1358_dn7, locals.var_xgs__blk1358_dn8, ) = (assign50280_e65019, ((locals.var_gf__blk1307_dn5 * assign50280_e65018) + (locals.var_gf__blk1307 * ((locals.var_ps__blk1354_dn5 + locals.var_ds__blk1353_dn5) / (2.0 * assign50280_e65018)))), ((locals.var_gf__blk1307_dn6 * assign50280_e65018) + (locals.var_gf__blk1307 * ((locals.var_ps__blk1354_dn6 + locals.var_ds__blk1353_dn6) / (2.0 * assign50280_e65018)))), ((locals.var_gf__blk1307_dn7 * assign50280_e65018) + (locals.var_gf__blk1307 * ((locals.var_ps__blk1354_dn7 + locals.var_ds__blk1353_dn7) / (2.0 * assign50280_e65018)))), ((locals.var_gf__blk1307_dn8 * assign50280_e65018) + (locals.var_gf__blk1307 * ((locals.var_ps__blk1354_dn8 + locals.var_ds__blk1353_dn8) / (2.0 * assign50280_e65018)))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
            let assign50290_e65031: f64 = (locals.var_gf2__blk1308 * locals.var_ds__blk1353);
            let assign50290_e65033: f64 = (assign50290_e65031 * locals.var_phit1__blk1322);
            let assign50290_e65037: f64 = (locals.var_gf__blk1307 * locals.var_sqs__blk1355);
            let assign50290_e65038: f64 = (locals.var_xgs__blk1358 + assign50290_e65037);
            let assign50290_e65039: f64 = (assign50290_e65033 / assign50290_e65038);
            (locals.var_qis__blk1359, locals.var_qis__blk1359_dn5, locals.var_qis__blk1359_dn6, locals.var_qis__blk1359_dn7, locals.var_qis__blk1359_dn8, ) = (assign50290_e65039, (((((((locals.var_gf2__blk1308_dn5 * locals.var_ds__blk1353) + (locals.var_gf2__blk1308 * locals.var_ds__blk1353_dn5)) * locals.var_phit1__blk1322) + (assign50290_e65031 * locals.var_phit1__blk1322_dn5)) * assign50290_e65038) - (assign50290_e65033 * (locals.var_xgs__blk1358_dn5 + ((locals.var_gf__blk1307_dn5 * locals.var_sqs__blk1355) + (locals.var_gf__blk1307 * locals.var_sqs__blk1355_dn5))))) / (assign50290_e65038 * assign50290_e65038)), (((((((locals.var_gf2__blk1308_dn6 * locals.var_ds__blk1353) + (locals.var_gf2__blk1308 * locals.var_ds__blk1353_dn6)) * locals.var_phit1__blk1322) + (assign50290_e65031 * locals.var_phit1__blk1322_dn6)) * assign50290_e65038) - (assign50290_e65033 * (locals.var_xgs__blk1358_dn6 + ((locals.var_gf__blk1307_dn6 * locals.var_sqs__blk1355) + (locals.var_gf__blk1307 * locals.var_sqs__blk1355_dn6))))) / (assign50290_e65038 * assign50290_e65038)), (((((((locals.var_gf2__blk1308_dn7 * locals.var_ds__blk1353) + (locals.var_gf2__blk1308 * locals.var_ds__blk1353_dn7)) * locals.var_phit1__blk1322) + (assign50290_e65031 * locals.var_phit1__blk1322_dn7)) * assign50290_e65038) - (assign50290_e65033 * (locals.var_xgs__blk1358_dn7 + ((locals.var_gf__blk1307_dn7 * locals.var_sqs__blk1355) + (locals.var_gf__blk1307 * locals.var_sqs__blk1355_dn7))))) / (assign50290_e65038 * assign50290_e65038)), (((((((locals.var_gf2__blk1308_dn8 * locals.var_ds__blk1353) + (locals.var_gf2__blk1308 * locals.var_ds__blk1353_dn8)) * locals.var_phit1__blk1322) + (assign50290_e65031 * locals.var_phit1__blk1322_dn8)) * assign50290_e65038) - (assign50290_e65033 * (locals.var_xgs__blk1358_dn8 + ((locals.var_gf__blk1307_dn8 * locals.var_sqs__blk1355) + (locals.var_gf__blk1307 * locals.var_sqs__blk1355_dn8))))) / (assign50290_e65038 * assign50290_e65038)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
            let assign50300_e65051: f64 = (locals.var_sqs__blk1355 * locals.var_gf__blk1307);
            let assign50300_e65053: f64 = (assign50300_e65051 * locals.var_phit1__blk1322);
            (locals.var_qbs__blk1360, locals.var_qbs__blk1360_dn5, locals.var_qbs__blk1360_dn6, locals.var_qbs__blk1360_dn7, locals.var_qbs__blk1360_dn8, ) = (assign50300_e65053, ((((locals.var_sqs__blk1355_dn5 * locals.var_gf__blk1307) + (locals.var_sqs__blk1355 * locals.var_gf__blk1307_dn5)) * locals.var_phit1__blk1322) + (assign50300_e65051 * locals.var_phit1__blk1322_dn5)), ((((locals.var_sqs__blk1355_dn6 * locals.var_gf__blk1307) + (locals.var_sqs__blk1355 * locals.var_gf__blk1307_dn6)) * locals.var_phit1__blk1322) + (assign50300_e65051 * locals.var_phit1__blk1322_dn6)), ((((locals.var_sqs__blk1355_dn7 * locals.var_gf__blk1307) + (locals.var_sqs__blk1355 * locals.var_gf__blk1307_dn7)) * locals.var_phit1__blk1322) + (assign50300_e65051 * locals.var_phit1__blk1322_dn7)), ((((locals.var_sqs__blk1355_dn8 * locals.var_gf__blk1307) + (locals.var_sqs__blk1355 * locals.var_gf__blk1307_dn8)) * locals.var_phit1__blk1322) + (assign50300_e65051 * locals.var_phit1__blk1322_dn8)), );
        }

        let assign50310_e65058: f64 = if locals.var_rsb_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1479 = assign50310_e65058;

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1479 != 0.0)) {
            let assign50320_e65072: f64 = (locals.var_rsb_i * locals.var_vsbx__blk1306);
            let assign50320_e65073: f64 = (1.0 - assign50320_e65072);
            let assign50320_e65074: f64 = (1.0 / assign50320_e65073);
            (locals.var_rhob__blk1361, locals.var_rhob__blk1361_dn5, locals.var_rhob__blk1361_dn6, locals.var_rhob__blk1361_dn7, locals.var_rhob__blk1361_dn8, ) = (assign50320_e65074, (-((-(locals.var_rsb_i * locals.var_vsbx__blk1306_dn5)) / (assign50320_e65073 * assign50320_e65073))), (-((-(locals.var_rsb_i * locals.var_vsbx__blk1306_dn6)) / (assign50320_e65073 * assign50320_e65073))), (-((-(locals.var_rsb_i * locals.var_vsbx__blk1306_dn7)) / (assign50320_e65073 * assign50320_e65073))), (-((-(locals.var_rsb_i * locals.var_vsbx__blk1306_dn8)) / (assign50320_e65073 * assign50320_e65073))), );
        }

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1479 == 0.0)) {
            let assign50330_e65090: f64 = (locals.var_rsb_i * locals.var_vsbx__blk1306);
            let assign50330_e65091: f64 = (1.0 + assign50330_e65090);
            (locals.var_rhob__blk1361, locals.var_rhob__blk1361_dn5, locals.var_rhob__blk1361_dn6, locals.var_rhob__blk1361_dn7, locals.var_rhob__blk1361_dn8, ) = (assign50330_e65091, (locals.var_rsb_i * locals.var_vsbx__blk1306_dn5), (locals.var_rsb_i * locals.var_vsbx__blk1306_dn6), (locals.var_rsb_i * locals.var_vsbx__blk1306_dn7), (locals.var_rsb_i * locals.var_vsbx__blk1306_dn8), );
        }

        let assign50340_e65096: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1480 = assign50340_e65096;

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1480 != 0.0)) {
            let assign50350_e65109: f64 = (locals.var_rsg_i * locals.var_qis__blk1359);
            let assign50350_e65110: f64 = (1.0 - assign50350_e65109);
            (locals.var_rhog__blk1362, locals.var_rhog__blk1362_dn5, locals.var_rhog__blk1362_dn6, locals.var_rhog__blk1362_dn7, locals.var_rhog__blk1362_dn8, ) = (assign50350_e65110, (-(locals.var_rsg_i * locals.var_qis__blk1359_dn5)), (-(locals.var_rsg_i * locals.var_qis__blk1359_dn6)), (-(locals.var_rsg_i * locals.var_qis__blk1359_dn7)), (-(locals.var_rsg_i * locals.var_qis__blk1359_dn8)), );
        }

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1480 == 0.0)) {
            let assign50360_e65127: f64 = (locals.var_rsg_i * locals.var_qis__blk1359);
            let assign50360_e65128: f64 = (1.0 + assign50360_e65127);
            let assign50360_e65129: f64 = (1.0 / assign50360_e65128);
            (locals.var_rhog__blk1362, locals.var_rhog__blk1362_dn5, locals.var_rhog__blk1362_dn6, locals.var_rhog__blk1362_dn7, locals.var_rhog__blk1362_dn8, ) = (assign50360_e65129, (-((locals.var_rsg_i * locals.var_qis__blk1359_dn5) / (assign50360_e65128 * assign50360_e65128))), (-((locals.var_rsg_i * locals.var_qis__blk1359_dn6) / (assign50360_e65128 * assign50360_e65128))), (-((locals.var_rsg_i * locals.var_qis__blk1359_dn7) / (assign50360_e65128 * assign50360_e65128))), (-((locals.var_rsg_i * locals.var_qis__blk1359_dn8) / (assign50360_e65128 * assign50360_e65128))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
            let assign50370_e65141: f64 = (locals.var_ther_i * locals.var_rhob__blk1361);
            let assign50370_e65143: f64 = (assign50370_e65141 * locals.var_rhog__blk1362);
            let assign50370_e65145: f64 = (assign50370_e65143 * locals.var_qis__blk1359);
            (locals.var_gr__blk1363, locals.var_gr__blk1363_dn5, locals.var_gr__blk1363_dn6, locals.var_gr__blk1363_dn7, locals.var_gr__blk1363_dn8, ) = (assign50370_e65145, (((((locals.var_ther_i * locals.var_rhob__blk1361_dn5) * locals.var_rhog__blk1362) + (assign50370_e65141 * locals.var_rhog__blk1362_dn5)) * locals.var_qis__blk1359) + (assign50370_e65143 * locals.var_qis__blk1359_dn5)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn6) * locals.var_rhog__blk1362) + (assign50370_e65141 * locals.var_rhog__blk1362_dn6)) * locals.var_qis__blk1359) + (assign50370_e65143 * locals.var_qis__blk1359_dn6)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn7) * locals.var_rhog__blk1362) + (assign50370_e65141 * locals.var_rhog__blk1362_dn7)) * locals.var_qis__blk1359) + (assign50370_e65143 * locals.var_qis__blk1359_dn7)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn8) * locals.var_rhog__blk1362) + (assign50370_e65141 * locals.var_rhog__blk1362_dn8)) * locals.var_qis__blk1359) + (assign50370_e65143 * locals.var_qis__blk1359_dn8)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
            let assign50380_e65159: f64 = (locals.var_eta_mu * locals.var_qis__blk1359);
            let assign50380_e65160: f64 = (locals.var_qbs__blk1360 + assign50380_e65159);
            let assign50380_e65161: f64 = (locals.var_e_eff0 * assign50380_e65160);
            (locals.var_eeffs__blk1364, locals.var_eeffs__blk1364_dn5, locals.var_eeffs__blk1364_dn6, locals.var_eeffs__blk1364_dn7, locals.var_eeffs__blk1364_dn8, ) = (assign50380_e65161, (locals.var_e_eff0 * (locals.var_qbs__blk1360_dn5 + (locals.var_eta_mu * locals.var_qis__blk1359_dn5))), (locals.var_e_eff0 * (locals.var_qbs__blk1360_dn6 + (locals.var_eta_mu * locals.var_qis__blk1359_dn6))), (locals.var_e_eff0 * (locals.var_qbs__blk1360_dn7 + (locals.var_eta_mu * locals.var_qis__blk1359_dn7))), (locals.var_e_eff0 * (locals.var_qbs__blk1360_dn8 + (locals.var_eta_mu * locals.var_qis__blk1359_dn8))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
            let assign50390_e65174: f64 = (locals.var_ps__blk1354 + locals.var_ds__blk1353);
            let assign50390_e65176: f64 = (assign50390_e65174 + 1e-14);
            let assign50390_e65177: f64 = (locals.var_ps__blk1354 / assign50390_e65176);
            let assign50390_e65178: f64 = (assign50390_e65177).ln();
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign50390_e65178, ((((locals.var_ps__blk1354_dn5 * assign50390_e65176) - (locals.var_ps__blk1354 * (locals.var_ps__blk1354_dn5 + locals.var_ds__blk1353_dn5))) / (assign50390_e65176 * assign50390_e65176)) / assign50390_e65177), ((((locals.var_ps__blk1354_dn6 * assign50390_e65176) - (locals.var_ps__blk1354 * (locals.var_ps__blk1354_dn6 + locals.var_ds__blk1353_dn6))) / (assign50390_e65176 * assign50390_e65176)) / assign50390_e65177), ((((locals.var_ps__blk1354_dn7 * assign50390_e65176) - (locals.var_ps__blk1354 * (locals.var_ps__blk1354_dn7 + locals.var_ds__blk1353_dn7))) / (assign50390_e65176 * assign50390_e65176)) / assign50390_e65177), ((((locals.var_ps__blk1354_dn8 * assign50390_e65176) - (locals.var_ps__blk1354 * (locals.var_ps__blk1354_dn8 + locals.var_ds__blk1353_dn8))) / (assign50390_e65176 * assign50390_e65176)) / assign50390_e65177), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
            let assign50400_e65190: f64 = (locals.var_eeffs__blk1364 * locals.var_mue_t);
            let assign50400_e65192: f64 = (assign50400_e65190).powf(locals.var_themu_t);
            let assign50400_e65196: f64 = (0.5 * locals.var_thecs_t);
            let assign50400_e65198: f64 = (assign50400_e65196 * locals.var_temp1);
            let assign50400_e65199: f64 = (assign50400_e65198).exp();
            let assign50400_e65200: f64 = (locals.var_cs_t * assign50400_e65199);
            let assign50400_e65201: f64 = (assign50400_e65192 + assign50400_e65200);
            (locals.var_mutmp__blk1365, locals.var_mutmp__blk1365_dn5, locals.var_mutmp__blk1365_dn6, locals.var_mutmp__blk1365_dn7, locals.var_mutmp__blk1365_dn8, ) = (assign50400_e65201, (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50400_e65190).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1364_dn5 * locals.var_mue_t))) } } else { (assign50400_e65192 * (locals.var_themu_t * ((locals.var_eeffs__blk1364_dn5 * locals.var_mue_t) / assign50400_e65190))) } + (locals.var_cs_t * (assign50400_e65199 * (assign50400_e65196 * locals.var_temp1_dn5)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50400_e65190).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1364_dn6 * locals.var_mue_t))) } } else { (assign50400_e65192 * (locals.var_themu_t * ((locals.var_eeffs__blk1364_dn6 * locals.var_mue_t) / assign50400_e65190))) } + (locals.var_cs_t * (assign50400_e65199 * (assign50400_e65196 * locals.var_temp1_dn6)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50400_e65190).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1364_dn7 * locals.var_mue_t))) } } else { (assign50400_e65192 * (locals.var_themu_t * ((locals.var_eeffs__blk1364_dn7 * locals.var_mue_t) / assign50400_e65190))) } + (locals.var_cs_t * (assign50400_e65199 * (assign50400_e65196 * locals.var_temp1_dn7)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign50400_e65190).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs__blk1364_dn8 * locals.var_mue_t))) } } else { (assign50400_e65192 * (locals.var_themu_t * ((locals.var_eeffs__blk1364_dn8 * locals.var_mue_t) / assign50400_e65190))) } + (locals.var_cs_t * (assign50400_e65199 * (assign50400_e65196 * locals.var_temp1_dn8)))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
            let assign50410_e65213: f64 = (1.0 + locals.var_mutmp__blk1365);
            let assign50410_e65215: f64 = (assign50410_e65213 + locals.var_gr__blk1363);
            let assign50410_e65217: f64 = (assign50410_e65215 * locals.var_rxcor__blk1357);
            (locals.var_gmobs__blk1366, locals.var_gmobs__blk1366_dn5, locals.var_gmobs__blk1366_dn6, locals.var_gmobs__blk1366_dn7, locals.var_gmobs__blk1366_dn8, ) = (assign50410_e65217, (((locals.var_mutmp__blk1365_dn5 + locals.var_gr__blk1363_dn5) * locals.var_rxcor__blk1357) + (assign50410_e65215 * locals.var_rxcor__blk1357_dn5)), (((locals.var_mutmp__blk1365_dn6 + locals.var_gr__blk1363_dn6) * locals.var_rxcor__blk1357) + (assign50410_e65215 * locals.var_rxcor__blk1357_dn6)), (((locals.var_mutmp__blk1365_dn7 + locals.var_gr__blk1363_dn7) * locals.var_rxcor__blk1357) + (assign50410_e65215 * locals.var_rxcor__blk1357_dn7)), (((locals.var_mutmp__blk1365_dn8 + locals.var_gr__blk1363_dn8) * locals.var_rxcor__blk1357) + (assign50410_e65215 * locals.var_rxcor__blk1357_dn8)), );
        }

        let assign50420_e65222: f64 = if locals.var_thesatb_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1481 = assign50420_e65222;

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1481 != 0.0)) {
            let assign50430_e65236: f64 = (locals.var_thesatb_i * locals.var_vsbx__blk1306);
            let assign50430_e65237: f64 = (1.0 - assign50430_e65236);
            let assign50430_e65238: f64 = (1.0 / assign50430_e65237);
            (locals.var_xitsb__blk1367, locals.var_xitsb__blk1367_dn5, locals.var_xitsb__blk1367_dn6, locals.var_xitsb__blk1367_dn7, locals.var_xitsb__blk1367_dn8, ) = (assign50430_e65238, (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1306_dn5)) / (assign50430_e65237 * assign50430_e65237))), (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1306_dn6)) / (assign50430_e65237 * assign50430_e65237))), (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1306_dn7)) / (assign50430_e65237 * assign50430_e65237))), (-((-(locals.var_thesatb_i * locals.var_vsbx__blk1306_dn8)) / (assign50430_e65237 * assign50430_e65237))), );
        }

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1481 == 0.0)) {
            let assign50440_e65254: f64 = (locals.var_thesatb_i * locals.var_vsbx__blk1306);
            let assign50440_e65255: f64 = (1.0 + assign50440_e65254);
            (locals.var_xitsb__blk1367, locals.var_xitsb__blk1367_dn5, locals.var_xitsb__blk1367_dn6, locals.var_xitsb__blk1367_dn7, locals.var_xitsb__blk1367_dn8, ) = (assign50440_e65255, (locals.var_thesatb_i * locals.var_vsbx__blk1306_dn5), (locals.var_thesatb_i * locals.var_vsbx__blk1306_dn6), (locals.var_thesatb_i * locals.var_vsbx__blk1306_dn7), (locals.var_thesatb_i * locals.var_vsbx__blk1306_dn8), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
            let assign50450_e65267: f64 = (locals.var_qis__blk1359 * locals.var_xitsb__blk1367);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign50450_e65267, ((locals.var_qis__blk1359_dn5 * locals.var_xitsb__blk1367) + (locals.var_qis__blk1359 * locals.var_xitsb__blk1367_dn5)), ((locals.var_qis__blk1359_dn6 * locals.var_xitsb__blk1367) + (locals.var_qis__blk1359 * locals.var_xitsb__blk1367_dn6)), ((locals.var_qis__blk1359_dn7 * locals.var_xitsb__blk1367) + (locals.var_qis__blk1359 * locals.var_xitsb__blk1367_dn7)), ((locals.var_qis__blk1359_dn8 * locals.var_xitsb__blk1367) + (locals.var_qis__blk1359 * locals.var_xitsb__blk1367_dn8)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) {
            let assign50460_e65280: f64 = (locals.var_thesatt_i + locals.var_temp2);
            let assign50460_e65281: f64 = (locals.var_temp2 / assign50460_e65280);
            (locals.var_wsat__blk1368, locals.var_wsat__blk1368_dn5, locals.var_wsat__blk1368_dn6, locals.var_wsat__blk1368_dn7, locals.var_wsat__blk1368_dn8, ) = (assign50460_e65281, (((locals.var_temp2_dn5 * assign50460_e65280) - (locals.var_temp2 * locals.var_temp2_dn5)) / (assign50460_e65280 * assign50460_e65280)), (((locals.var_temp2_dn6 * assign50460_e65280) - (locals.var_temp2 * locals.var_temp2_dn6)) / (assign50460_e65280 * assign50460_e65280)), (((locals.var_temp2_dn7 * assign50460_e65280) - (locals.var_temp2 * locals.var_temp2_dn7)) / (assign50460_e65280 * assign50460_e65280)), (((locals.var_temp2_dn8 * assign50460_e65280) - (locals.var_temp2 * locals.var_temp2_dn8)) / (assign50460_e65280 * assign50460_e65280)), );
        }

        let assign50470_e65286: f64 = if locals.var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1482 = assign50470_e65286;

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1482 != 0.0)) {
            let assign50480_e65300: f64 = (locals.var_thesatg_i * locals.var_wsat__blk1368);
            let assign50480_e65301: f64 = (1.0 - assign50480_e65300);
            let assign50480_e65302: f64 = (1.0 / assign50480_e65301);
            (locals.var_factheta__blk1369, locals.var_factheta__blk1369_dn5, locals.var_factheta__blk1369_dn6, locals.var_factheta__blk1369_dn7, locals.var_factheta__blk1369_dn8, ) = (assign50480_e65302, (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn5)) / (assign50480_e65301 * assign50480_e65301))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn6)) / (assign50480_e65301 * assign50480_e65301))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn7)) / (assign50480_e65301 * assign50480_e65301))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn8)) / (assign50480_e65301 * assign50480_e65301))), );
        }

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1457 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1478 != 0.0)) && (locals.var_guard1482 == 0.0)) {
            let assign50490_e65318: f64 = (locals.var_thesatg_i * locals.var_wsat__blk1368);
            let assign50490_e65319: f64 = (1.0 + assign50490_e65318);
            (locals.var_factheta__blk1369, locals.var_factheta__blk1369_dn5, locals.var_factheta__blk1369_dn6, locals.var_factheta__blk1369_dn7, locals.var_factheta__blk1369_dn8, ) = (assign50490_e65319, (locals.var_thesatg_i * locals.var_wsat__blk1368_dn5), (locals.var_thesatg_i * locals.var_wsat__blk1368_dn6), (locals.var_thesatg_i * locals.var_wsat__blk1368_dn7), (locals.var_thesatg_i * locals.var_wsat__blk1368_dn8), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1457 == 0.0)) {
            (locals.var_vgb1__blk1304, locals.var_vgb1__blk1304_dn5, locals.var_vgb1__blk1304_dn6, locals.var_vgb1__blk1304_dn7, locals.var_vgb1__blk1304_dn8, ) = (locals.var_vgb1_dc, locals.var_vgb1_dc_dn5, locals.var_vgb1_dc_dn6, locals.var_vgb1_dc_dn7, locals.var_vgb1_dc_dn8, );
            (locals.var_vsbx__blk1306, locals.var_vsbx__blk1306_dn5, locals.var_vsbx__blk1306_dn6, locals.var_vsbx__blk1306_dn7, locals.var_vsbx__blk1306_dn8, ) = (locals.var_vsbx_dc, locals.var_vsbx_dc_dn5, locals.var_vsbx_dc_dn6, locals.var_vsbx_dc_dn7, locals.var_vsbx_dc_dn8, );
            (locals.var_phit1__blk1322, locals.var_phit1__blk1322_dn5, locals.var_phit1__blk1322_dn6, locals.var_phit1__blk1322_dn7, locals.var_phit1__blk1322_dn8, ) = (locals.var_phit1_dc, locals.var_phit1_dc_dn5, locals.var_phit1_dc_dn6, locals.var_phit1_dc_dn7, locals.var_phit1_dc_dn8, );
            (locals.var_inv_phit1__blk1323, locals.var_inv_phit1__blk1323_dn5, locals.var_inv_phit1__blk1323_dn6, locals.var_inv_phit1__blk1323_dn7, locals.var_inv_phit1__blk1323_dn8, ) = (locals.var_inv_phit1_dc, locals.var_inv_phit1_dc_dn5, locals.var_inv_phit1_dc_dn6, locals.var_inv_phit1_dc_dn7, locals.var_inv_phit1_dc_dn8, );
            (locals.var_gf__blk1307, locals.var_gf__blk1307_dn5, locals.var_gf__blk1307_dn6, locals.var_gf__blk1307_dn7, locals.var_gf__blk1307_dn8, ) = (locals.var_gf_dc, locals.var_gf_dc_dn5, locals.var_gf_dc_dn6, locals.var_gf_dc_dn7, locals.var_gf_dc_dn8, );
            (locals.var_gf2__blk1308, locals.var_gf2__blk1308_dn5, locals.var_gf2__blk1308_dn6, locals.var_gf2__blk1308_dn7, locals.var_gf2__blk1308_dn8, ) = (locals.var_gf2_dc, locals.var_gf2_dc_dn5, locals.var_gf2_dc_dn6, locals.var_gf2_dc_dn7, locals.var_gf2_dc_dn8, );
            (locals.var_inv_gf2__blk1324, locals.var_inv_gf2__blk1324_dn5, locals.var_inv_gf2__blk1324_dn6, locals.var_inv_gf2__blk1324_dn7, locals.var_inv_gf2__blk1324_dn8, ) = (locals.var_inv_gf2_dc, locals.var_inv_gf2_dc_dn5, locals.var_inv_gf2_dc_dn6, locals.var_inv_gf2_dc_dn7, locals.var_inv_gf2_dc_dn8, );
            (locals.var_xg__blk1326, locals.var_xg__blk1326_dn5, locals.var_xg__blk1326_dn6, locals.var_xg__blk1326_dn7, locals.var_xg__blk1326_dn8, ) = (locals.var_xg_dc, locals.var_xg_dc_dn5, locals.var_xg_dc_dn6, locals.var_xg_dc_dn7, locals.var_xg_dc_dn8, );
            (locals.var_xno_s__blk1331, locals.var_xno_s__blk1331_dn5, locals.var_xno_s__blk1331_dn6, locals.var_xno_s__blk1331_dn7, locals.var_xno_s__blk1331_dn8, ) = (locals.var_xno_s_dc, locals.var_xno_s_dc_dn5, locals.var_xno_s_dc_dn6, locals.var_xno_s_dc_dn7, locals.var_xno_s_dc_dn8, );
            (locals.var_xn_s__blk1332, locals.var_xn_s__blk1332_dn5, locals.var_xn_s__blk1332_dn6, locals.var_xn_s__blk1332_dn7, locals.var_xn_s__blk1332_dn8, ) = (locals.var_xn_s_dc, locals.var_xn_s_dc_dn5, locals.var_xn_s_dc_dn6, locals.var_xn_s_dc_dn7, locals.var_xn_s_dc_dn8, );
            (locals.var_xi__blk1343, locals.var_xi__blk1343_dn5, locals.var_xi__blk1343_dn6, locals.var_xi__blk1343_dn7, locals.var_xi__blk1343_dn8, ) = (locals.var_xi_dc, locals.var_xi_dc_dn5, locals.var_xi_dc_dn6, locals.var_xi_dc_dn7, locals.var_xi_dc_dn8, );
            locals.var_margin__blk1344 = locals.var_margin_dc;
            (locals.var_inv_xi__blk1345, locals.var_inv_xi__blk1345_dn5, locals.var_inv_xi__blk1345_dn6, locals.var_inv_xi__blk1345_dn7, locals.var_inv_xi__blk1345_dn8, ) = (locals.var_inv_xi_dc, locals.var_inv_xi_dc_dn5, locals.var_inv_xi_dc_dn6, locals.var_inv_xi_dc_dn7, locals.var_inv_xi_dc_dn8, );
            (locals.var_sp_s_x1__blk1452, locals.var_sp_s_x1__blk1452_dn5, locals.var_sp_s_x1__blk1452_dn6, locals.var_sp_s_x1__blk1452_dn7, locals.var_sp_s_x1__blk1452_dn8, ) = (locals.var_sp_s_x1_dc, locals.var_sp_s_x1_dc_dn5, locals.var_sp_s_x1_dc_dn6, locals.var_sp_s_x1_dc_dn7, locals.var_sp_s_x1_dc_dn8, );
            (locals.var_delta_ns__blk1347, locals.var_delta_ns__blk1347_dn5, locals.var_delta_ns__blk1347_dn6, locals.var_delta_ns__blk1347_dn7, locals.var_delta_ns__blk1347_dn8, ) = (locals.var_delta_ns_dc, locals.var_delta_ns_dc_dn5, locals.var_delta_ns_dc_dn6, locals.var_delta_ns_dc_dn7, locals.var_delta_ns_dc_dn8, );
            (locals.var_x_s__blk1346, locals.var_x_s__blk1346_dn5, locals.var_x_s__blk1346_dn6, locals.var_x_s__blk1346_dn7, locals.var_x_s__blk1346_dn8, ) = (locals.var_x_s_dc, locals.var_x_s_dc_dn5, locals.var_x_s_dc_dn6, locals.var_x_s_dc_dn7, locals.var_x_s_dc_dn8, );
            (locals.var_xi1s__blk1349, locals.var_xi1s__blk1349_dn5, locals.var_xi1s__blk1349_dn6, locals.var_xi1s__blk1349_dn7, locals.var_xi1s__blk1349_dn8, ) = (locals.var_xi1s_dc, locals.var_xi1s_dc_dn5, locals.var_xi1s_dc_dn6, locals.var_xi1s_dc_dn7, locals.var_xi1s_dc_dn8, );
            (locals.var_xi2s__blk1350, locals.var_xi2s__blk1350_dn5, locals.var_xi2s__blk1350_dn6, locals.var_xi2s__blk1350_dn7, locals.var_xi2s__blk1350_dn8, ) = (locals.var_xi2s_dc, locals.var_xi2s_dc_dn5, locals.var_xi2s_dc_dn6, locals.var_xi2s_dc_dn7, locals.var_xi2s_dc_dn8, );
            (locals.var_delta_1s__blk1351, locals.var_delta_1s__blk1351_dn5, locals.var_delta_1s__blk1351_dn6, locals.var_delta_1s__blk1351_dn7, locals.var_delta_1s__blk1351_dn8, ) = (locals.var_delta_1s_dc, locals.var_delta_1s_dc_dn5, locals.var_delta_1s_dc_dn6, locals.var_delta_1s_dc_dn7, locals.var_delta_1s_dc_dn8, );
            (locals.var_es__blk1352, locals.var_es__blk1352_dn5, locals.var_es__blk1352_dn6, locals.var_es__blk1352_dn7, locals.var_es__blk1352_dn8, ) = (locals.var_es_dc, locals.var_es_dc_dn5, locals.var_es_dc_dn6, locals.var_es_dc_dn7, locals.var_es_dc_dn8, );
            (locals.var_ps__blk1354, locals.var_ps__blk1354_dn5, locals.var_ps__blk1354_dn6, locals.var_ps__blk1354_dn7, locals.var_ps__blk1354_dn8, ) = (locals.var_ps_dc, locals.var_ps_dc_dn5, locals.var_ps_dc_dn6, locals.var_ps_dc_dn7, locals.var_ps_dc_dn8, );
            (locals.var_ds__blk1353, locals.var_ds__blk1353_dn5, locals.var_ds__blk1353_dn6, locals.var_ds__blk1353_dn7, locals.var_ds__blk1353_dn8, ) = (locals.var_ds_dc, locals.var_ds_dc_dn5, locals.var_ds_dc_dn6, locals.var_ds_dc_dn7, locals.var_ds_dc_dn8, );
            (locals.var_sqs__blk1355, locals.var_sqs__blk1355_dn5, locals.var_sqs__blk1355_dn6, locals.var_sqs__blk1355_dn7, locals.var_sqs__blk1355_dn8, ) = (locals.var_sqs_dc, locals.var_sqs_dc_dn5, locals.var_sqs_dc_dn6, locals.var_sqs_dc_dn7, locals.var_sqs_dc_dn8, );
            (locals.var_alphas__blk1356, locals.var_alphas__blk1356_dn5, locals.var_alphas__blk1356_dn6, locals.var_alphas__blk1356_dn7, locals.var_alphas__blk1356_dn8, ) = (locals.var_alphas_dc, locals.var_alphas_dc_dn5, locals.var_alphas_dc_dn6, locals.var_alphas_dc_dn7, locals.var_alphas_dc_dn8, );
            (locals.var_rxcor__blk1357, locals.var_rxcor__blk1357_dn5, locals.var_rxcor__blk1357_dn6, locals.var_rxcor__blk1357_dn7, locals.var_rxcor__blk1357_dn8, ) = (locals.var_rxcor_dc, locals.var_rxcor_dc_dn5, locals.var_rxcor_dc_dn6, locals.var_rxcor_dc_dn7, locals.var_rxcor_dc_dn8, );
            (locals.var_xgs__blk1358, locals.var_xgs__blk1358_dn5, locals.var_xgs__blk1358_dn6, locals.var_xgs__blk1358_dn7, locals.var_xgs__blk1358_dn8, ) = (locals.var_xgs_dc, locals.var_xgs_dc_dn5, locals.var_xgs_dc_dn6, locals.var_xgs_dc_dn7, locals.var_xgs_dc_dn8, );
            (locals.var_qis__blk1359, locals.var_qis__blk1359_dn5, locals.var_qis__blk1359_dn6, locals.var_qis__blk1359_dn7, locals.var_qis__blk1359_dn8, ) = (locals.var_qis_dc, locals.var_qis_dc_dn5, locals.var_qis_dc_dn6, locals.var_qis_dc_dn7, locals.var_qis_dc_dn8, );
            (locals.var_qbs__blk1360, locals.var_qbs__blk1360_dn5, locals.var_qbs__blk1360_dn6, locals.var_qbs__blk1360_dn7, locals.var_qbs__blk1360_dn8, ) = (locals.var_qbs_dc, locals.var_qbs_dc_dn5, locals.var_qbs_dc_dn6, locals.var_qbs_dc_dn7, locals.var_qbs_dc_dn8, );
            (locals.var_rhob__blk1361, locals.var_rhob__blk1361_dn5, locals.var_rhob__blk1361_dn6, locals.var_rhob__blk1361_dn7, locals.var_rhob__blk1361_dn8, ) = (locals.var_rhob_dc, locals.var_rhob_dc_dn5, locals.var_rhob_dc_dn6, locals.var_rhob_dc_dn7, locals.var_rhob_dc_dn8, );
            (locals.var_rhog__blk1362, locals.var_rhog__blk1362_dn5, locals.var_rhog__blk1362_dn6, locals.var_rhog__blk1362_dn7, locals.var_rhog__blk1362_dn8, ) = (locals.var_rhog_dc, locals.var_rhog_dc_dn5, locals.var_rhog_dc_dn6, locals.var_rhog_dc_dn7, locals.var_rhog_dc_dn8, );
            (locals.var_gmobs__blk1366, locals.var_gmobs__blk1366_dn5, locals.var_gmobs__blk1366_dn6, locals.var_gmobs__blk1366_dn7, locals.var_gmobs__blk1366_dn8, ) = (locals.var_gmobs_dc, locals.var_gmobs_dc_dn5, locals.var_gmobs_dc_dn6, locals.var_gmobs_dc_dn7, locals.var_gmobs_dc_dn8, );
            (locals.var_xitsb__blk1367, locals.var_xitsb__blk1367_dn5, locals.var_xitsb__blk1367_dn6, locals.var_xitsb__blk1367_dn7, locals.var_xitsb__blk1367_dn8, ) = (locals.var_xitsb_dc, locals.var_xitsb_dc_dn5, locals.var_xitsb_dc_dn6, locals.var_xitsb_dc_dn7, locals.var_xitsb_dc_dn8, );
            (locals.var_factheta__blk1369, locals.var_factheta__blk1369_dn5, locals.var_factheta__blk1369_dn6, locals.var_factheta__blk1369_dn7, locals.var_factheta__blk1369_dn8, ) = (locals.var_factheta_dc, locals.var_factheta_dc_dn5, locals.var_factheta_dc_dn6, locals.var_factheta_dc_dn7, locals.var_factheta_dc_dn8, );
        }

        if (locals.var_guard1456 != 0.0) {
            locals.var_thesatloc__blk1302 = locals.var_thesat_t;
            locals.var_arloc__blk1303 = locals.var_ar;
        }

        let assign50950_e65662: f64 = if p.p48 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1483 = assign50950_e65662;

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1483 != 0.0)) {
            locals.var_thesatloc__blk1302 = locals.var_thesatac_t;
            locals.var_arloc__blk1303 = locals.var_arac;
        }

        if (locals.var_guard1456 != 0.0) {
            (locals.var_thesat1__blk1371, locals.var_thesat1__blk1371_dn5, locals.var_thesat1__blk1371_dn6, locals.var_thesat1__blk1371_dn7, locals.var_thesat1__blk1371_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard1456 != 0.0) {
            let assign50990_e65682: f64 = (locals.var_phit1__blk1322 * 4.60517018598809);
            (locals.var_vdsat_lim__blk1370, locals.var_vdsat_lim__blk1370_dn5, locals.var_vdsat_lim__blk1370_dn6, locals.var_vdsat_lim__blk1370_dn7, locals.var_vdsat_lim__blk1370_dn8, ) = (assign50990_e65682, (locals.var_phit1__blk1322_dn5 * 4.60517018598809), (locals.var_phit1__blk1322_dn6 * 4.60517018598809), (locals.var_phit1__blk1322_dn7 * 4.60517018598809), (locals.var_phit1__blk1322_dn8 * 4.60517018598809), );
        }

        if (locals.var_guard1456 != 0.0) {
            (locals.var_v_dsat__blk1387, locals.var_v_dsat__blk1387_dn5, locals.var_v_dsat__blk1387_dn6, locals.var_v_dsat__blk1387_dn7, locals.var_v_dsat__blk1387_dn8, ) = (locals.var_vdsat_lim__blk1370, locals.var_vdsat_lim__blk1370_dn5, locals.var_vdsat_lim__blk1370_dn6, locals.var_vdsat_lim__blk1370_dn7, locals.var_vdsat_lim__blk1370_dn8, );
            (locals.var_vdse__blk1388, locals.var_vdse__blk1388_dn5, locals.var_vdse__blk1388_dn6, locals.var_vdse__blk1388_dn7, locals.var_vdse__blk1388_dn8, ) = (locals.var_v_ds, 0.0, locals.var_v_ds_dn6, locals.var_v_ds_dn7, 0.0, );
        }

        if (locals.var_guard1456 != 0.0) {
            let assign51020_e65696: f64 = (locals.var_v_ds * locals.var_inv_phit1__blk1323);
            (locals.var_udse__blk1389, locals.var_udse__blk1389_dn5, locals.var_udse__blk1389_dn6, locals.var_udse__blk1389_dn7, locals.var_udse__blk1389_dn8, ) = (assign51020_e65696, (locals.var_v_ds * locals.var_inv_phit1__blk1323_dn5), ((locals.var_v_ds_dn6 * locals.var_inv_phit1__blk1323) + (locals.var_v_ds * locals.var_inv_phit1__blk1323_dn6)), ((locals.var_v_ds_dn7 * locals.var_inv_phit1__blk1323) + (locals.var_v_ds * locals.var_inv_phit1__blk1323_dn7)), (locals.var_v_ds * locals.var_inv_phit1__blk1323_dn8), );
        }

        if (locals.var_guard1456 != 0.0) {
            (locals.var_x_d__blk1393, locals.var_x_d__blk1393_dn5, locals.var_x_d__blk1393_dn6, locals.var_x_d__blk1393_dn7, locals.var_x_d__blk1393_dn8, ) = (locals.var_x_s__blk1346, locals.var_x_s__blk1346_dn5, locals.var_x_s__blk1346_dn6, locals.var_x_s__blk1346_dn7, locals.var_x_s__blk1346_dn8, );
            (locals.var_x_ds__blk1394, locals.var_x_ds__blk1394_dn5, locals.var_x_ds__blk1394_dn6, locals.var_x_ds__blk1394_dn7, locals.var_x_ds__blk1394_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_dps__blk1397, locals.var_dps__blk1397_dn5, locals.var_dps__blk1397_dn6, locals.var_dps__blk1397_dn7, locals.var_dps__blk1397_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_ed__blk1399, locals.var_ed__blk1399_dn5, locals.var_ed__blk1399_dn6, locals.var_ed__blk1399_dn7, locals.var_ed__blk1399_dn8, ) = (locals.var_es__blk1352, locals.var_es__blk1352_dn5, locals.var_es__blk1352_dn6, locals.var_es__blk1352_dn7, locals.var_es__blk1352_dn8, );
            (locals.var_pd__blk1400, locals.var_pd__blk1400_dn5, locals.var_pd__blk1400_dn6, locals.var_pd__blk1400_dn7, locals.var_pd__blk1400_dn8, ) = (locals.var_ps__blk1354, locals.var_ps__blk1354_dn5, locals.var_ps__blk1354_dn6, locals.var_ps__blk1354_dn7, locals.var_ps__blk1354_dn8, );
            (locals.var_dd__blk1402, locals.var_dd__blk1402_dn5, locals.var_dd__blk1402_dn6, locals.var_dd__blk1402_dn7, locals.var_dd__blk1402_dn8, ) = (locals.var_ds__blk1353, locals.var_ds__blk1353_dn5, locals.var_ds__blk1353_dn6, locals.var_ds__blk1353_dn7, locals.var_ds__blk1353_dn8, );
        }

    }

    pub(super) fn stamp_transient_block_23(
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard1456 != 0.0) {
            (locals.var_qbd__blk1403, locals.var_qbd__blk1403_dn5, locals.var_qbd__blk1403_dn6, locals.var_qbd__blk1403_dn7, locals.var_qbd__blk1403_dn8, ) = (locals.var_qbs__blk1360, locals.var_qbs__blk1360_dn5, locals.var_qbs__blk1360_dn6, locals.var_qbs__blk1360_dn7, locals.var_qbs__blk1360_dn8, );
            (locals.var_x_m__blk1404, locals.var_x_m__blk1404_dn5, locals.var_x_m__blk1404_dn6, locals.var_x_m__blk1404_dn7, locals.var_x_m__blk1404_dn8, ) = (locals.var_x_s__blk1346, locals.var_x_s__blk1346_dn5, locals.var_x_s__blk1346_dn6, locals.var_x_s__blk1346_dn7, locals.var_x_s__blk1346_dn8, );
            (locals.var_em__blk1405, locals.var_em__blk1405_dn5, locals.var_em__blk1405_dn6, locals.var_em__blk1405_dn7, locals.var_em__blk1405_dn8, ) = (locals.var_es__blk1352, locals.var_es__blk1352_dn5, locals.var_es__blk1352_dn6, locals.var_es__blk1352_dn7, locals.var_es__blk1352_dn8, );
            (locals.var_dm__blk1407, locals.var_dm__blk1407_dn5, locals.var_dm__blk1407_dn6, locals.var_dm__blk1407_dn7, locals.var_dm__blk1407_dn8, ) = (locals.var_ds__blk1353, locals.var_ds__blk1353_dn5, locals.var_ds__blk1353_dn6, locals.var_ds__blk1353_dn7, locals.var_ds__blk1353_dn8, );
            (locals.var_pm__blk1408, locals.var_pm__blk1408_dn5, locals.var_pm__blk1408_dn6, locals.var_pm__blk1408_dn7, locals.var_pm__blk1408_dn8, ) = (locals.var_ps__blk1354, locals.var_ps__blk1354_dn5, locals.var_ps__blk1354_dn6, locals.var_ps__blk1354_dn7, locals.var_ps__blk1354_dn8, );
        }

        if (locals.var_guard1456 != 0.0) {
            let assign51140_e65746: f64 = (locals.var_xg__blk1326 - locals.var_x_s__blk1346);
            (locals.var_xgm__blk1409, locals.var_xgm__blk1409_dn5, locals.var_xgm__blk1409_dn6, locals.var_xgm__blk1409_dn7, locals.var_xgm__blk1409_dn8, ) = (assign51140_e65746, (locals.var_xg__blk1326_dn5 - locals.var_x_s__blk1346_dn5), (locals.var_xg__blk1326_dn6 - locals.var_x_s__blk1346_dn6), (locals.var_xg__blk1326_dn7 - locals.var_x_s__blk1346_dn7), (locals.var_xg__blk1326_dn8 - locals.var_x_s__blk1346_dn8), );
        }

        if (locals.var_guard1456 != 0.0) {
            (locals.var_eta_p__blk1410, locals.var_eta_p__blk1410_dn5, locals.var_eta_p__blk1410_dn6, locals.var_eta_p__blk1410_dn7, locals.var_eta_p__blk1410_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_alpha__blk1412, locals.var_alpha__blk1412_dn5, locals.var_alpha__blk1412_dn6, locals.var_alpha__blk1412_dn7, locals.var_alpha__blk1412_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_sqm__blk1411, locals.var_sqm__blk1411_dn5, locals.var_sqm__blk1411_dn6, locals.var_sqm__blk1411_dn7, locals.var_sqm__blk1411_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_qim__blk1421, locals.var_qim__blk1421_dn5, locals.var_qim__blk1421_dn6, locals.var_qim__blk1421_dn7, locals.var_qim__blk1421_dn8, ) = (locals.var_qis__blk1359, locals.var_qis__blk1359_dn5, locals.var_qis__blk1359_dn6, locals.var_qis__blk1359_dn7, locals.var_qis__blk1359_dn8, );
        }

        if (locals.var_guard1456 != 0.0) {
            let assign51190_e65768: f64 = (locals.var_xgm__blk1409 * locals.var_phit1__blk1322);
            (locals.var_qeff1__blk1425, locals.var_qeff1__blk1425_dn5, locals.var_qeff1__blk1425_dn6, locals.var_qeff1__blk1425_dn7, locals.var_qeff1__blk1425_dn8, ) = (assign51190_e65768, ((locals.var_xgm__blk1409_dn5 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn5)), ((locals.var_xgm__blk1409_dn6 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn6)), ((locals.var_xgm__blk1409_dn7 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn7)), ((locals.var_xgm__blk1409_dn8 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn8)), );
        }

        if (locals.var_guard1456 != 0.0) {
            (locals.var_qim1__blk1422, locals.var_qim1__blk1422_dn5, locals.var_qim1__blk1422_dn6, locals.var_qim1__blk1422_dn7, locals.var_qim1__blk1422_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_qbm__blk1423, locals.var_qbm__blk1423_dn5, locals.var_qbm__blk1423_dn6, locals.var_qbm__blk1423_dn7, locals.var_qbm__blk1423_dn8, ) = (locals.var_qbs__blk1360, locals.var_qbs__blk1360_dn5, locals.var_qbs__blk1360_dn6, locals.var_qbs__blk1360_dn7, locals.var_qbs__blk1360_dn8, );
            (locals.var_s1__blk1428, locals.var_s1__blk1428_dn5, locals.var_s1__blk1428_dn6, locals.var_s1__blk1428_dn7, locals.var_s1__blk1428_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_gmob__blk1427, locals.var_gmob__blk1427_dn5, locals.var_gmob__blk1427_dn6, locals.var_gmob__blk1427_dn7, locals.var_gmob__blk1427_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_thesateff__blk1430, locals.var_thesateff__blk1430_dn5, locals.var_thesateff__blk1430_dn6, locals.var_thesateff__blk1430_dn7, locals.var_thesateff__blk1430_dn8, ) = (locals.var_thesatloc__blk1302, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_voxm__blk1429, locals.var_voxm__blk1429_dn5, locals.var_voxm__blk1429_dn6, locals.var_voxm__blk1429_dn7, locals.var_voxm__blk1429_dn8, ) = (locals.var_qeff1__blk1425, locals.var_qeff1__blk1425_dn5, locals.var_qeff1__blk1425_dn6, locals.var_qeff1__blk1425_dn7, locals.var_qeff1__blk1425_dn8, );
        }

        let assign51260_e65797: f64 = if locals.var_xg__blk1326 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1484 = assign51260_e65797;

        let assign51270_e65800: f64 = if locals.var_ds__blk1353 > 1e-100 { 1.0 } else { 0.0 };
        locals.var_guard1485 = assign51270_e65800;

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
            let assign51280_e65808: f64 = (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369);
            (locals.var_thesateff__blk1430, locals.var_thesateff__blk1430_dn5, locals.var_thesateff__blk1430_dn6, locals.var_thesateff__blk1430_dn7, locals.var_thesateff__blk1430_dn8, ) = (assign51280_e65808, (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn5), (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn6), (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn7), (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn8), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
            let assign51290_e65818: f64 = (locals.var_thesateff__blk1430 / locals.var_gmobs__blk1366);
            (locals.var_thesat1__blk1371, locals.var_thesat1__blk1371_dn5, locals.var_thesat1__blk1371_dn6, locals.var_thesat1__blk1371_dn7, locals.var_thesat1__blk1371_dn8, ) = (assign51290_e65818, (((locals.var_thesateff__blk1430_dn5 * locals.var_gmobs__blk1366) - (locals.var_thesateff__blk1430 * locals.var_gmobs__blk1366_dn5)) / (locals.var_gmobs__blk1366 * locals.var_gmobs__blk1366)), (((locals.var_thesateff__blk1430_dn6 * locals.var_gmobs__blk1366) - (locals.var_thesateff__blk1430 * locals.var_gmobs__blk1366_dn6)) / (locals.var_gmobs__blk1366 * locals.var_gmobs__blk1366)), (((locals.var_thesateff__blk1430_dn7 * locals.var_gmobs__blk1366) - (locals.var_thesateff__blk1430 * locals.var_gmobs__blk1366_dn7)) / (locals.var_gmobs__blk1366 * locals.var_gmobs__blk1366)), (((locals.var_thesateff__blk1430_dn8 * locals.var_gmobs__blk1366) - (locals.var_thesateff__blk1430 * locals.var_gmobs__blk1366_dn8)) / (locals.var_gmobs__blk1366 * locals.var_gmobs__blk1366)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
            let assign51300_e65829: f64 = (0.5 * locals.var_gf2__blk1308);
            let assign51300_e65830: f64 = (locals.var_xgs__blk1358 + assign51300_e65829);
            (locals.var_asat__blk1372, locals.var_asat__blk1372_dn5, locals.var_asat__blk1372_dn6, locals.var_asat__blk1372_dn7, locals.var_asat__blk1372_dn8, ) = (assign51300_e65830, (locals.var_xgs__blk1358_dn5 + (0.5 * locals.var_gf2__blk1308_dn5)), (locals.var_xgs__blk1358_dn6 + (0.5 * locals.var_gf2__blk1308_dn6)), (locals.var_xgs__blk1358_dn7 + (0.5 * locals.var_gf2__blk1308_dn7)), (locals.var_xgs__blk1358_dn8 + (0.5 * locals.var_gf2__blk1308_dn8)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
            let assign51310_e65840: f64 = (locals.var_gf2__blk1308 * locals.var_delta_1s__blk1351);
            let __rspice_inv_cse_0: f64 = 1.0 / locals.var_asat__blk1372;
            let assign51310_e65842: f64 = (assign51310_e65840 * __rspice_inv_cse_0);
            let assign51310_e65844: f64 = (assign51310_e65842 * __rspice_inv_cse_0);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign51310_e65844, ((((((((locals.var_gf2__blk1308_dn5 * locals.var_delta_1s__blk1351) + (locals.var_gf2__blk1308 * locals.var_delta_1s__blk1351_dn5)) * locals.var_asat__blk1372) - (assign51310_e65840 * locals.var_asat__blk1372_dn5)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)) * locals.var_asat__blk1372) - (assign51310_e65842 * locals.var_asat__blk1372_dn5)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)), ((((((((locals.var_gf2__blk1308_dn6 * locals.var_delta_1s__blk1351) + (locals.var_gf2__blk1308 * locals.var_delta_1s__blk1351_dn6)) * locals.var_asat__blk1372) - (assign51310_e65840 * locals.var_asat__blk1372_dn6)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)) * locals.var_asat__blk1372) - (assign51310_e65842 * locals.var_asat__blk1372_dn6)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)), ((((((((locals.var_gf2__blk1308_dn7 * locals.var_delta_1s__blk1351) + (locals.var_gf2__blk1308 * locals.var_delta_1s__blk1351_dn7)) * locals.var_asat__blk1372) - (assign51310_e65840 * locals.var_asat__blk1372_dn7)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)) * locals.var_asat__blk1372) - (assign51310_e65842 * locals.var_asat__blk1372_dn7)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)), ((((((((locals.var_gf2__blk1308_dn8 * locals.var_delta_1s__blk1351) + (locals.var_gf2__blk1308 * locals.var_delta_1s__blk1351_dn8)) * locals.var_asat__blk1372) - (assign51310_e65840 * locals.var_asat__blk1372_dn8)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)) * locals.var_asat__blk1372) - (assign51310_e65842 * locals.var_asat__blk1372_dn8)) / (locals.var_asat__blk1372 * locals.var_asat__blk1372)), );
        }

        let assign51320_e65849: f64 = if locals.var_temp__blk936 > 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard1486 = assign51320_e65849;

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1486 != 0.0)) {
            let assign51330_e65859: f64 = (1.0 - locals.var_temp__blk936);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign51330_e65859, (-locals.var_temp__blk936_dn5), (-locals.var_temp__blk936_dn6), (-locals.var_temp__blk936_dn7), (-locals.var_temp__blk936_dn8), );
        }

        let assign51340_e65864: f64 = if locals.var_temp1 < 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1487 = assign51340_e65864;

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1486 != 0.0)) && (locals.var_guard1487 != 0.0)) {
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1486 != 0.0)) && (locals.var_guard1487 == 0.0)) {
            let assign51360_e65889: f64 = (locals.var_temp1).sqrt();
            let assign51360_e65890: f64 = (1.0 - assign51360_e65889);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign51360_e65890, (-(locals.var_temp1_dn5 / (2.0 * assign51360_e65889))), (-(locals.var_temp1_dn6 / (2.0 * assign51360_e65889))), (-(locals.var_temp1_dn7 / (2.0 * assign51360_e65889))), (-(locals.var_temp1_dn8 / (2.0 * assign51360_e65889))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1486 == 0.0)) {
            let assign51370_e65903: f64 = (0.5 * locals.var_temp__blk936);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign51370_e65903, (0.5 * locals.var_temp__blk936_dn5), (0.5 * locals.var_temp__blk936_dn6), (0.5 * locals.var_temp__blk936_dn7), (0.5 * locals.var_temp__blk936_dn8), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
            let assign51380_e65913: f64 = (locals.var_temp2 * locals.var_asat__blk1372);
            (locals.var_x_inf0__blk1373, locals.var_x_inf0__blk1373_dn5, locals.var_x_inf0__blk1373_dn6, locals.var_x_inf0__blk1373_dn7, locals.var_x_inf0__blk1373_dn8, ) = (assign51380_e65913, ((locals.var_temp2_dn5 * locals.var_asat__blk1372) + (locals.var_temp2 * locals.var_asat__blk1372_dn5)), ((locals.var_temp2_dn6 * locals.var_asat__blk1372) + (locals.var_temp2 * locals.var_asat__blk1372_dn6)), ((locals.var_temp2_dn7 * locals.var_asat__blk1372) + (locals.var_temp2 * locals.var_asat__blk1372_dn7)), ((locals.var_temp2_dn8 * locals.var_asat__blk1372) + (locals.var_temp2 * locals.var_asat__blk1372_dn8)), );
        }

        let assign51390_e65922: f64 = if ((locals.var_cs_t > 0.0) && (locals.var_thecs_t > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1488 = assign51390_e65922;

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
            let assign51400_e65932: f64 = (0.475 * locals.var_phit1__blk1322);
            let assign51400_e65934: f64 = (assign51400_e65932 * locals.var_x_inf0__blk1373);
            (locals.var_midphi0__blk1374, locals.var_midphi0__blk1374_dn5, locals.var_midphi0__blk1374_dn6, locals.var_midphi0__blk1374_dn7, locals.var_midphi0__blk1374_dn8, ) = (assign51400_e65934, (((0.475 * locals.var_phit1__blk1322_dn5) * locals.var_x_inf0__blk1373) + (assign51400_e65932 * locals.var_x_inf0__blk1373_dn5)), (((0.475 * locals.var_phit1__blk1322_dn6) * locals.var_x_inf0__blk1373) + (assign51400_e65932 * locals.var_x_inf0__blk1373_dn6)), (((0.475 * locals.var_phit1__blk1322_dn7) * locals.var_x_inf0__blk1373) + (assign51400_e65932 * locals.var_x_inf0__blk1373_dn7)), (((0.475 * locals.var_phit1__blk1322_dn8) * locals.var_x_inf0__blk1373) + (assign51400_e65932 * locals.var_x_inf0__blk1373_dn8)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
            let assign51410_e65947: f64 = (locals.var_alphas__blk1356 * locals.var_midphi0__blk1374);
            let assign51410_e65948: f64 = (locals.var_qis__blk1359 - assign51410_e65947);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign51410_e65948, (locals.var_qis__blk1359_dn5 - ((locals.var_alphas__blk1356_dn5 * locals.var_midphi0__blk1374) + (locals.var_alphas__blk1356 * locals.var_midphi0__blk1374_dn5))), (locals.var_qis__blk1359_dn6 - ((locals.var_alphas__blk1356_dn6 * locals.var_midphi0__blk1374) + (locals.var_alphas__blk1356 * locals.var_midphi0__blk1374_dn6))), (locals.var_qis__blk1359_dn7 - ((locals.var_alphas__blk1356_dn7 * locals.var_midphi0__blk1374) + (locals.var_alphas__blk1356 * locals.var_midphi0__blk1374_dn7))), (locals.var_qis__blk1359_dn8 - ((locals.var_alphas__blk1356_dn8 * locals.var_midphi0__blk1374) + (locals.var_alphas__blk1356 * locals.var_midphi0__blk1374_dn8))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
            let assign51420_e65962: f64 = (locals.var_temp__blk936 * locals.var_temp__blk936);
            let assign51420_e65964: f64 = (assign51420_e65962 + 1e-12);
            let assign51420_e65965: f64 = (assign51420_e65964).sqrt();
            let assign51420_e65966: f64 = (locals.var_temp__blk936 + assign51420_e65965);
            let assign51420_e65967: f64 = (0.5 * assign51420_e65966);
            (locals.var_qisat__blk1375, locals.var_qisat__blk1375_dn5, locals.var_qisat__blk1375_dn6, locals.var_qisat__blk1375_dn7, locals.var_qisat__blk1375_dn8, ) = (assign51420_e65967, (0.5 * (locals.var_temp__blk936_dn5 + (((locals.var_temp__blk936_dn5 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn5)) / (2.0 * assign51420_e65965)))), (0.5 * (locals.var_temp__blk936_dn6 + (((locals.var_temp__blk936_dn6 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn6)) / (2.0 * assign51420_e65965)))), (0.5 * (locals.var_temp__blk936_dn7 + (((locals.var_temp__blk936_dn7 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn7)) / (2.0 * assign51420_e65965)))), (0.5 * (locals.var_temp__blk936_dn8 + (((locals.var_temp__blk936_dn8 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn8)) / (2.0 * assign51420_e65965)))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
            let assign51430_e65979: f64 = (locals.var_phit1__blk1322 * locals.var_xgs__blk1358);
            let assign51430_e65981: f64 = (assign51430_e65979 - locals.var_qis__blk1359);
            let assign51430_e65984: f64 = (locals.var_alphas__blk1356 - 1.0);
            let assign51430_e65986: f64 = (assign51430_e65984 * locals.var_midphi0__blk1374);
            let assign51430_e65987: f64 = (assign51430_e65981 + assign51430_e65986);
            (locals.var_qbsat__blk1376, locals.var_qbsat__blk1376_dn5, locals.var_qbsat__blk1376_dn6, locals.var_qbsat__blk1376_dn7, locals.var_qbsat__blk1376_dn8, ) = (assign51430_e65987, ((((locals.var_phit1__blk1322_dn5 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn5)) - locals.var_qis__blk1359_dn5) + ((locals.var_alphas__blk1356_dn5 * locals.var_midphi0__blk1374) + (assign51430_e65984 * locals.var_midphi0__blk1374_dn5))), ((((locals.var_phit1__blk1322_dn6 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn6)) - locals.var_qis__blk1359_dn6) + ((locals.var_alphas__blk1356_dn6 * locals.var_midphi0__blk1374) + (assign51430_e65984 * locals.var_midphi0__blk1374_dn6))), ((((locals.var_phit1__blk1322_dn7 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn7)) - locals.var_qis__blk1359_dn7) + ((locals.var_alphas__blk1356_dn7 * locals.var_midphi0__blk1374) + (assign51430_e65984 * locals.var_midphi0__blk1374_dn7))), ((((locals.var_phit1__blk1322_dn8 * locals.var_xgs__blk1358) + (locals.var_phit1__blk1322 * locals.var_xgs__blk1358_dn8)) - locals.var_qis__blk1359_dn8) + ((locals.var_alphas__blk1356_dn8 * locals.var_midphi0__blk1374) + (assign51430_e65984 * locals.var_midphi0__blk1374_dn8))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
            let assign51440_e66000: f64 = (0.5 * locals.var_gf2__blk1308);
            let assign51440_e66002: f64 = (assign51440_e66000 * locals.var_phit1__blk1322);
            let assign51440_e66004: f64 = (assign51440_e66002 / locals.var_qbsat__blk1376);
            let assign51440_e66005: f64 = (1.0 + assign51440_e66004);
            (locals.var_alphasat__blk1377, locals.var_alphasat__blk1377_dn5, locals.var_alphasat__blk1377_dn6, locals.var_alphasat__blk1377_dn7, locals.var_alphasat__blk1377_dn8, ) = (assign51440_e66005, ((((((0.5 * locals.var_gf2__blk1308_dn5) * locals.var_phit1__blk1322) + (assign51440_e66000 * locals.var_phit1__blk1322_dn5)) * locals.var_qbsat__blk1376) - (assign51440_e66002 * locals.var_qbsat__blk1376_dn5)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)), ((((((0.5 * locals.var_gf2__blk1308_dn6) * locals.var_phit1__blk1322) + (assign51440_e66000 * locals.var_phit1__blk1322_dn6)) * locals.var_qbsat__blk1376) - (assign51440_e66002 * locals.var_qbsat__blk1376_dn6)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)), ((((((0.5 * locals.var_gf2__blk1308_dn7) * locals.var_phit1__blk1322) + (assign51440_e66000 * locals.var_phit1__blk1322_dn7)) * locals.var_qbsat__blk1376) - (assign51440_e66002 * locals.var_qbsat__blk1376_dn7)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)), ((((((0.5 * locals.var_gf2__blk1308_dn8) * locals.var_phit1__blk1322) + (assign51440_e66000 * locals.var_phit1__blk1322_dn8)) * locals.var_qbsat__blk1376) - (assign51440_e66002 * locals.var_qbsat__blk1376_dn8)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
            let assign51450_e66018: f64 = (locals.var_eta_mu * locals.var_qisat__blk1375);
            let assign51450_e66019: f64 = (locals.var_qbsat__blk1376 + assign51450_e66018);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign51450_e66019, (locals.var_qbsat__blk1376_dn5 + (locals.var_eta_mu * locals.var_qisat__blk1375_dn5)), (locals.var_qbsat__blk1376_dn6 + (locals.var_eta_mu * locals.var_qisat__blk1375_dn6)), (locals.var_qbsat__blk1376_dn7 + (locals.var_eta_mu * locals.var_qisat__blk1375_dn7)), (locals.var_qbsat__blk1376_dn8 + (locals.var_eta_mu * locals.var_qisat__blk1375_dn8)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
            let assign51460_e66031: f64 = (locals.var_e_eff0 * locals.var_temp__blk936);
            let assign51460_e66033: f64 = (assign51460_e66031 * locals.var_mue_t);
            let assign51460_e66035: f64 = (assign51460_e66033).powf(locals.var_themu_t);
            (locals.var_gmobmusat__blk1378, locals.var_gmobmusat__blk1378_dn5, locals.var_gmobmusat__blk1378_dn6, locals.var_gmobmusat__blk1378_dn7, locals.var_gmobmusat__blk1378_dn8, ) = (assign51460_e66035, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51460_e66033).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk936_dn5) * locals.var_mue_t))) } } else { (assign51460_e66035 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk936_dn5) * locals.var_mue_t) / assign51460_e66033))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51460_e66033).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk936_dn6) * locals.var_mue_t))) } } else { (assign51460_e66035 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk936_dn6) * locals.var_mue_t) / assign51460_e66033))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51460_e66033).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk936_dn7) * locals.var_mue_t))) } } else { (assign51460_e66035 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk936_dn7) * locals.var_mue_t) / assign51460_e66033))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign51460_e66033).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk936_dn8) * locals.var_mue_t))) } } else { (assign51460_e66035 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk936_dn8) * locals.var_mue_t) / assign51460_e66033))) }, );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
            let assign51470_e66049: f64 = (1.0 - locals.var_eta_mu);
            let assign51470_e66050: f64 = (locals.var_alphasat__blk1377 * assign51470_e66049);
            let assign51470_e66052: f64 = (assign51470_e66050 - 1.0);
            let assign51470_e66053: f64 = (locals.var_themu_t * assign51470_e66052);
            let assign51470_e66055: f64 = (assign51470_e66053 / locals.var_temp__blk936);
            let assign51470_e66057: f64 = (assign51470_e66055 * locals.var_gmobmusat__blk1378);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign51470_e66057, ((((((locals.var_themu_t * (locals.var_alphasat__blk1377_dn5 * assign51470_e66049)) * locals.var_temp__blk936) - (assign51470_e66053 * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936)) * locals.var_gmobmusat__blk1378) + (assign51470_e66055 * locals.var_gmobmusat__blk1378_dn5)), ((((((locals.var_themu_t * (locals.var_alphasat__blk1377_dn6 * assign51470_e66049)) * locals.var_temp__blk936) - (assign51470_e66053 * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936)) * locals.var_gmobmusat__blk1378) + (assign51470_e66055 * locals.var_gmobmusat__blk1378_dn6)), ((((((locals.var_themu_t * (locals.var_alphasat__blk1377_dn7 * assign51470_e66049)) * locals.var_temp__blk936) - (assign51470_e66053 * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936)) * locals.var_gmobmusat__blk1378) + (assign51470_e66055 * locals.var_gmobmusat__blk1378_dn7)), ((((((locals.var_themu_t * (locals.var_alphasat__blk1377_dn8 * assign51470_e66049)) * locals.var_temp__blk936) - (assign51470_e66053 * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936)) * locals.var_gmobmusat__blk1378) + (assign51470_e66055 * locals.var_gmobmusat__blk1378_dn8)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
            let assign51480_e66069: f64 = (locals.var_qisat__blk1375 / locals.var_qbsat__blk1376);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign51480_e66069, (((locals.var_qisat__blk1375_dn5 * locals.var_qbsat__blk1376) - (locals.var_qisat__blk1375 * locals.var_qbsat__blk1376_dn5)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)), (((locals.var_qisat__blk1375_dn6 * locals.var_qbsat__blk1376) - (locals.var_qisat__blk1375 * locals.var_qbsat__blk1376_dn6)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)), (((locals.var_qisat__blk1375_dn7 * locals.var_qbsat__blk1376) - (locals.var_qisat__blk1375 * locals.var_qbsat__blk1376_dn7)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)), (((locals.var_qisat__blk1375_dn8 * locals.var_qbsat__blk1376) - (locals.var_qisat__blk1375 * locals.var_qbsat__blk1376_dn8)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
            let assign51490_e66082: f64 = (1.0 + locals.var_temp__blk936);
            let assign51490_e66084: f64 = (-locals.var_thecs_t);
            let assign51490_e66085: f64 = (assign51490_e66082).powf(assign51490_e66084);
            let assign51490_e66086: f64 = (locals.var_cs_t * assign51490_e66085);
            (locals.var_gmobcssat__blk1379, locals.var_gmobcssat__blk1379_dn5, locals.var_gmobcssat__blk1379_dn6, locals.var_gmobcssat__blk1379_dn7, locals.var_gmobcssat__blk1379_dn8, ) = (assign51490_e66086, (locals.var_cs_t * if 0.0 == 0.0 && ((assign51490_e66084) as f64).is_finite() && ((assign51490_e66084) as f64).fract() == 0.0 { if assign51490_e66084 == 0.0 { 0.0 } else { (assign51490_e66084 * ((assign51490_e66082).powf(assign51490_e66084 - 1.0) * locals.var_temp__blk936_dn5)) } } else { (assign51490_e66085 * (assign51490_e66084 * (locals.var_temp__blk936_dn5 / assign51490_e66082))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign51490_e66084) as f64).is_finite() && ((assign51490_e66084) as f64).fract() == 0.0 { if assign51490_e66084 == 0.0 { 0.0 } else { (assign51490_e66084 * ((assign51490_e66082).powf(assign51490_e66084 - 1.0) * locals.var_temp__blk936_dn6)) } } else { (assign51490_e66085 * (assign51490_e66084 * (locals.var_temp__blk936_dn6 / assign51490_e66082))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign51490_e66084) as f64).is_finite() && ((assign51490_e66084) as f64).fract() == 0.0 { if assign51490_e66084 == 0.0 { 0.0 } else { (assign51490_e66084 * ((assign51490_e66082).powf(assign51490_e66084 - 1.0) * locals.var_temp__blk936_dn7)) } } else { (assign51490_e66085 * (assign51490_e66084 * (locals.var_temp__blk936_dn7 / assign51490_e66082))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign51490_e66084) as f64).is_finite() && ((assign51490_e66084) as f64).fract() == 0.0 { if assign51490_e66084 == 0.0 { 0.0 } else { (assign51490_e66084 * ((assign51490_e66082).powf(assign51490_e66084 - 1.0) * locals.var_temp__blk936_dn8)) } } else { (assign51490_e66085 * (assign51490_e66084 * (locals.var_temp__blk936_dn8 / assign51490_e66082))) }), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
            let assign51500_e66099: f64 = (locals.var_alphasat__blk1377 - 1.0);
            let assign51500_e66103: f64 = (locals.var_temp__blk936 + 1.0);
            let assign51500_e66104: f64 = (1.0 / assign51500_e66103);
            let assign51500_e66105: f64 = (assign51500_e66099 + assign51500_e66104);
            let assign51500_e66106: f64 = (locals.var_thecs_t * assign51500_e66105);
            let assign51500_e66108: f64 = (assign51500_e66106 / locals.var_qbsat__blk1376);
            let assign51500_e66110: f64 = (assign51500_e66108 * locals.var_gmobcssat__blk1379);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign51500_e66110, ((((((locals.var_thecs_t * (locals.var_alphasat__blk1377_dn5 + (-(locals.var_temp__blk936_dn5 / (assign51500_e66103 * assign51500_e66103))))) * locals.var_qbsat__blk1376) - (assign51500_e66106 * locals.var_qbsat__blk1376_dn5)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)) * locals.var_gmobcssat__blk1379) + (assign51500_e66108 * locals.var_gmobcssat__blk1379_dn5)), ((((((locals.var_thecs_t * (locals.var_alphasat__blk1377_dn6 + (-(locals.var_temp__blk936_dn6 / (assign51500_e66103 * assign51500_e66103))))) * locals.var_qbsat__blk1376) - (assign51500_e66106 * locals.var_qbsat__blk1376_dn6)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)) * locals.var_gmobcssat__blk1379) + (assign51500_e66108 * locals.var_gmobcssat__blk1379_dn6)), ((((((locals.var_thecs_t * (locals.var_alphasat__blk1377_dn7 + (-(locals.var_temp__blk936_dn7 / (assign51500_e66103 * assign51500_e66103))))) * locals.var_qbsat__blk1376) - (assign51500_e66106 * locals.var_qbsat__blk1376_dn7)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)) * locals.var_gmobcssat__blk1379) + (assign51500_e66108 * locals.var_gmobcssat__blk1379_dn7)), ((((((locals.var_thecs_t * (locals.var_alphasat__blk1377_dn8 + (-(locals.var_temp__blk936_dn8 / (assign51500_e66103 * assign51500_e66103))))) * locals.var_qbsat__blk1376) - (assign51500_e66106 * locals.var_qbsat__blk1376_dn8)) / (locals.var_qbsat__blk1376 * locals.var_qbsat__blk1376)) * locals.var_gmobcssat__blk1379) + (assign51500_e66108 * locals.var_gmobcssat__blk1379_dn8)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
            let assign51510_e66122: f64 = (locals.var_ther_i * locals.var_rhob__blk1361);
            let assign51510_e66124: f64 = (assign51510_e66122 * locals.var_rhog__blk1362);
            let assign51510_e66126: f64 = (assign51510_e66124 * locals.var_qisat__blk1375);
            (locals.var_grsat__blk1380, locals.var_grsat__blk1380_dn5, locals.var_grsat__blk1380_dn6, locals.var_grsat__blk1380_dn7, locals.var_grsat__blk1380_dn8, ) = (assign51510_e66126, (((((locals.var_ther_i * locals.var_rhob__blk1361_dn5) * locals.var_rhog__blk1362) + (assign51510_e66122 * locals.var_rhog__blk1362_dn5)) * locals.var_qisat__blk1375) + (assign51510_e66124 * locals.var_qisat__blk1375_dn5)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn6) * locals.var_rhog__blk1362) + (assign51510_e66122 * locals.var_rhog__blk1362_dn6)) * locals.var_qisat__blk1375) + (assign51510_e66124 * locals.var_qisat__blk1375_dn6)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn7) * locals.var_rhog__blk1362) + (assign51510_e66122 * locals.var_rhog__blk1362_dn7)) * locals.var_qisat__blk1375) + (assign51510_e66124 * locals.var_qisat__blk1375_dn7)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn8) * locals.var_rhog__blk1362) + (assign51510_e66122 * locals.var_rhog__blk1362_dn8)) * locals.var_qisat__blk1375) + (assign51510_e66124 * locals.var_qisat__blk1375_dn8)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
            let assign51520_e66140: f64 = (locals.var_ther_i * locals.var_rhob__blk1361);
            let assign51520_e66142: f64 = (assign51520_e66140 * locals.var_rhog__blk1362);
            let assign51520_e66144: f64 = (assign51520_e66142 * locals.var_alphasat__blk1377);
            let assign51520_e66145: f64 = (locals.var_temp1 - assign51520_e66144);
            let assign51520_e66147: f64 = (assign51520_e66145 / locals.var_temp2);
            let assign51520_e66148: f64 = (1.0 + assign51520_e66147);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign51520_e66148, ((((locals.var_temp1_dn5 - (((((locals.var_ther_i * locals.var_rhob__blk1361_dn5) * locals.var_rhog__blk1362) + (assign51520_e66140 * locals.var_rhog__blk1362_dn5)) * locals.var_alphasat__blk1377) + (assign51520_e66142 * locals.var_alphasat__blk1377_dn5))) * locals.var_temp2) - (assign51520_e66145 * locals.var_temp2_dn5)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn6 - (((((locals.var_ther_i * locals.var_rhob__blk1361_dn6) * locals.var_rhog__blk1362) + (assign51520_e66140 * locals.var_rhog__blk1362_dn6)) * locals.var_alphasat__blk1377) + (assign51520_e66142 * locals.var_alphasat__blk1377_dn6))) * locals.var_temp2) - (assign51520_e66145 * locals.var_temp2_dn6)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn7 - (((((locals.var_ther_i * locals.var_rhob__blk1361_dn7) * locals.var_rhog__blk1362) + (assign51520_e66140 * locals.var_rhog__blk1362_dn7)) * locals.var_alphasat__blk1377) + (assign51520_e66142 * locals.var_alphasat__blk1377_dn7))) * locals.var_temp2) - (assign51520_e66145 * locals.var_temp2_dn7)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn8 - (((((locals.var_ther_i * locals.var_rhob__blk1361_dn8) * locals.var_rhog__blk1362) + (assign51520_e66140 * locals.var_rhog__blk1362_dn8)) * locals.var_alphasat__blk1377) + (assign51520_e66142 * locals.var_alphasat__blk1377_dn8))) * locals.var_temp2) - (assign51520_e66145 * locals.var_temp2_dn8)) / (locals.var_temp2 * locals.var_temp2)), );
        }

        let assign51530_e66153: f64 = if locals.var_temp__blk936 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1489 = assign51530_e66153;

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) && (locals.var_guard1489 != 0.0)) {
            let assign51540_e66167: f64 = (2.0 * locals.var_temp__blk936);
            let assign51540_e66168: f64 = (assign51540_e66167).exp();
            let assign51540_e66169: f64 = (1.0 + assign51540_e66168);
            let assign51540_e66170: f64 = (assign51540_e66169).ln();
            let assign51540_e66171: f64 = (0.5 * assign51540_e66170);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign51540_e66171, (0.5 * ((assign51540_e66168 * (2.0 * locals.var_temp__blk936_dn5)) / assign51540_e66169)), (0.5 * ((assign51540_e66168 * (2.0 * locals.var_temp__blk936_dn6)) / assign51540_e66169)), (0.5 * ((assign51540_e66168 * (2.0 * locals.var_temp__blk936_dn7)) / assign51540_e66169)), (0.5 * ((assign51540_e66168 * (2.0 * locals.var_temp__blk936_dn8)) / assign51540_e66169)), );
        }

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) && (locals.var_guard1489 == 0.0)) {
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
            let assign51560_e66195: f64 = (-locals.var_midphi0__blk1374);
            let assign51560_e66197: f64 = (assign51560_e66195 * locals.var_temp2);
            let assign51560_e66199: f64 = (assign51560_e66197 * locals.var_temp1);
            let assign51560_e66202: f64 = (1.0 + locals.var_gmobmusat__blk1378);
            let assign51560_e66204: f64 = (assign51560_e66202 + locals.var_gmobcssat__blk1379);
            let assign51560_e66206: f64 = (assign51560_e66204 + locals.var_grsat__blk1380);
            let assign51560_e66207: f64 = (assign51560_e66199 / assign51560_e66206);
            (locals.var_delta_gmob__blk1381, locals.var_delta_gmob__blk1381_dn5, locals.var_delta_gmob__blk1381_dn6, locals.var_delta_gmob__blk1381_dn7, locals.var_delta_gmob__blk1381_dn8, ) = (assign51560_e66207, ((((((((-locals.var_midphi0__blk1374_dn5) * locals.var_temp2) + (assign51560_e66195 * locals.var_temp2_dn5)) * locals.var_temp1) + (assign51560_e66197 * locals.var_temp1_dn5)) * assign51560_e66206) - (assign51560_e66199 * ((locals.var_gmobmusat__blk1378_dn5 + locals.var_gmobcssat__blk1379_dn5) + locals.var_grsat__blk1380_dn5))) / (assign51560_e66206 * assign51560_e66206)), ((((((((-locals.var_midphi0__blk1374_dn6) * locals.var_temp2) + (assign51560_e66195 * locals.var_temp2_dn6)) * locals.var_temp1) + (assign51560_e66197 * locals.var_temp1_dn6)) * assign51560_e66206) - (assign51560_e66199 * ((locals.var_gmobmusat__blk1378_dn6 + locals.var_gmobcssat__blk1379_dn6) + locals.var_grsat__blk1380_dn6))) / (assign51560_e66206 * assign51560_e66206)), ((((((((-locals.var_midphi0__blk1374_dn7) * locals.var_temp2) + (assign51560_e66195 * locals.var_temp2_dn7)) * locals.var_temp1) + (assign51560_e66197 * locals.var_temp1_dn7)) * assign51560_e66206) - (assign51560_e66199 * ((locals.var_gmobmusat__blk1378_dn7 + locals.var_gmobcssat__blk1379_dn7) + locals.var_grsat__blk1380_dn7))) / (assign51560_e66206 * assign51560_e66206)), ((((((((-locals.var_midphi0__blk1374_dn8) * locals.var_temp2) + (assign51560_e66195 * locals.var_temp2_dn8)) * locals.var_temp1) + (assign51560_e66197 * locals.var_temp1_dn8)) * assign51560_e66206) - (assign51560_e66199 * ((locals.var_gmobmusat__blk1378_dn8 + locals.var_gmobcssat__blk1379_dn8) + locals.var_grsat__blk1380_dn8))) / (assign51560_e66206 * assign51560_e66206)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 != 0.0)) {
            let assign51570_e66224: f64 = (locals.var_delta_gmob__blk1381 * locals.var_delta_gmob__blk1381);
            let assign51570_e66225: f64 = (1.0 + assign51570_e66224);
            let assign51570_e66226: f64 = (assign51570_e66225).sqrt();
            let assign51570_e66227: f64 = (1.0 + assign51570_e66226);
            let assign51570_e66228: f64 = (locals.var_delta_gmob__blk1381 / assign51570_e66227);
            let assign51570_e66229: f64 = (1.0 + assign51570_e66228);
            let assign51570_e66230: f64 = (locals.var_x_inf0__blk1373 * assign51570_e66229);
            (locals.var_x_inf__blk1382, locals.var_x_inf__blk1382_dn5, locals.var_x_inf__blk1382_dn6, locals.var_x_inf__blk1382_dn7, locals.var_x_inf__blk1382_dn8, ) = (assign51570_e66230, ((locals.var_x_inf0__blk1373_dn5 * assign51570_e66229) + (locals.var_x_inf0__blk1373 * (((locals.var_delta_gmob__blk1381_dn5 * assign51570_e66227) - (locals.var_delta_gmob__blk1381 * (((locals.var_delta_gmob__blk1381_dn5 * locals.var_delta_gmob__blk1381) + (locals.var_delta_gmob__blk1381 * locals.var_delta_gmob__blk1381_dn5)) / (2.0 * assign51570_e66226)))) / (assign51570_e66227 * assign51570_e66227)))), ((locals.var_x_inf0__blk1373_dn6 * assign51570_e66229) + (locals.var_x_inf0__blk1373 * (((locals.var_delta_gmob__blk1381_dn6 * assign51570_e66227) - (locals.var_delta_gmob__blk1381 * (((locals.var_delta_gmob__blk1381_dn6 * locals.var_delta_gmob__blk1381) + (locals.var_delta_gmob__blk1381 * locals.var_delta_gmob__blk1381_dn6)) / (2.0 * assign51570_e66226)))) / (assign51570_e66227 * assign51570_e66227)))), ((locals.var_x_inf0__blk1373_dn7 * assign51570_e66229) + (locals.var_x_inf0__blk1373 * (((locals.var_delta_gmob__blk1381_dn7 * assign51570_e66227) - (locals.var_delta_gmob__blk1381 * (((locals.var_delta_gmob__blk1381_dn7 * locals.var_delta_gmob__blk1381) + (locals.var_delta_gmob__blk1381 * locals.var_delta_gmob__blk1381_dn7)) / (2.0 * assign51570_e66226)))) / (assign51570_e66227 * assign51570_e66227)))), ((locals.var_x_inf0__blk1373_dn8 * assign51570_e66229) + (locals.var_x_inf0__blk1373 * (((locals.var_delta_gmob__blk1381_dn8 * assign51570_e66227) - (locals.var_delta_gmob__blk1381 * (((locals.var_delta_gmob__blk1381_dn8 * locals.var_delta_gmob__blk1381) + (locals.var_delta_gmob__blk1381 * locals.var_delta_gmob__blk1381_dn8)) / (2.0 * assign51570_e66226)))) / (assign51570_e66227 * assign51570_e66227)))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1488 == 0.0)) {
            (locals.var_x_inf__blk1382, locals.var_x_inf__blk1382_dn5, locals.var_x_inf__blk1382_dn6, locals.var_x_inf__blk1382_dn7, locals.var_x_inf__blk1382_dn8, ) = (locals.var_x_inf0__blk1373, locals.var_x_inf0__blk1373_dn5, locals.var_x_inf0__blk1373_dn6, locals.var_x_inf0__blk1373_dn7, locals.var_x_inf0__blk1373_dn8, );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
            let assign51590_e66251: f64 = (locals.var_phit1__blk1322 * locals.var_thesat1__blk1371);
            let assign51590_e66253: f64 = (assign51590_e66251 * locals.var_x_inf__blk1382);
            let assign51590_e66255: f64 = (assign51590_e66253 * 0.7071067811865475);
            (locals.var_ysat__blk1383, locals.var_ysat__blk1383_dn5, locals.var_ysat__blk1383_dn6, locals.var_ysat__blk1383_dn7, locals.var_ysat__blk1383_dn8, ) = (assign51590_e66255, (((((locals.var_phit1__blk1322_dn5 * locals.var_thesat1__blk1371) + (locals.var_phit1__blk1322 * locals.var_thesat1__blk1371_dn5)) * locals.var_x_inf__blk1382) + (assign51590_e66251 * locals.var_x_inf__blk1382_dn5)) * 0.7071067811865475), (((((locals.var_phit1__blk1322_dn6 * locals.var_thesat1__blk1371) + (locals.var_phit1__blk1322 * locals.var_thesat1__blk1371_dn6)) * locals.var_x_inf__blk1382) + (assign51590_e66251 * locals.var_x_inf__blk1382_dn6)) * 0.7071067811865475), (((((locals.var_phit1__blk1322_dn7 * locals.var_thesat1__blk1371) + (locals.var_phit1__blk1322 * locals.var_thesat1__blk1371_dn7)) * locals.var_x_inf__blk1382) + (assign51590_e66251 * locals.var_x_inf__blk1382_dn7)) * 0.7071067811865475), (((((locals.var_phit1__blk1322_dn8 * locals.var_thesat1__blk1371) + (locals.var_phit1__blk1322 * locals.var_thesat1__blk1371_dn8)) * locals.var_x_inf__blk1382) + (assign51590_e66251 * locals.var_x_inf__blk1382_dn8)) * 0.7071067811865475), );
        }

        let assign51600_e66260: f64 = (-1.0);
        let assign51600_e66261: f64 = if locals.var_chnl_type == assign51600_e66260 { 1.0 } else { 0.0 };
        locals.var_guard1490 = assign51600_e66261;

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1490 != 0.0)) {
            let assign51610_e66272: f64 = (1.0 + locals.var_ysat__blk1383);
            let assign51610_e66273: f64 = (assign51610_e66272).sqrt();
            let assign51610_e66274: f64 = (locals.var_ysat__blk1383 / assign51610_e66273);
            (locals.var_ysat__blk1383, locals.var_ysat__blk1383_dn5, locals.var_ysat__blk1383_dn6, locals.var_ysat__blk1383_dn7, locals.var_ysat__blk1383_dn8, ) = (assign51610_e66274, (((locals.var_ysat__blk1383_dn5 * assign51610_e66273) - (locals.var_ysat__blk1383 * (locals.var_ysat__blk1383_dn5 / (2.0 * assign51610_e66273)))) / (assign51610_e66273 * assign51610_e66273)), (((locals.var_ysat__blk1383_dn6 * assign51610_e66273) - (locals.var_ysat__blk1383 * (locals.var_ysat__blk1383_dn6 / (2.0 * assign51610_e66273)))) / (assign51610_e66273 * assign51610_e66273)), (((locals.var_ysat__blk1383_dn7 * assign51610_e66273) - (locals.var_ysat__blk1383 * (locals.var_ysat__blk1383_dn7 / (2.0 * assign51610_e66273)))) / (assign51610_e66273 * assign51610_e66273)), (((locals.var_ysat__blk1383_dn8 * assign51610_e66273) - (locals.var_ysat__blk1383 * (locals.var_ysat__blk1383_dn8 / (2.0 * assign51610_e66273)))) / (assign51610_e66273 * assign51610_e66273)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
            let assign51620_e66287: f64 = (4.0 * locals.var_ysat__blk1383);
            let assign51620_e66288: f64 = (1.0 + assign51620_e66287);
            let assign51620_e66289: f64 = (assign51620_e66288).sqrt();
            let assign51620_e66290: f64 = (1.0 + assign51620_e66289);
            let assign51620_e66291: f64 = (2.0 / assign51620_e66290);
            (locals.var_za__blk1384, locals.var_za__blk1384_dn5, locals.var_za__blk1384_dn6, locals.var_za__blk1384_dn7, locals.var_za__blk1384_dn8, ) = (assign51620_e66291, (-((2.0 * ((4.0 * locals.var_ysat__blk1383_dn5) / (2.0 * assign51620_e66289))) / (assign51620_e66290 * assign51620_e66290))), (-((2.0 * ((4.0 * locals.var_ysat__blk1383_dn6) / (2.0 * assign51620_e66289))) / (assign51620_e66290 * assign51620_e66290))), (-((2.0 * ((4.0 * locals.var_ysat__blk1383_dn7) / (2.0 * assign51620_e66289))) / (assign51620_e66290 * assign51620_e66290))), (-((2.0 * ((4.0 * locals.var_ysat__blk1383_dn8) / (2.0 * assign51620_e66289))) / (assign51620_e66290 * assign51620_e66290))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
            let assign51630_e66301: f64 = (locals.var_za__blk1384 * locals.var_ysat__blk1383);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign51630_e66301, ((locals.var_za__blk1384_dn5 * locals.var_ysat__blk1383) + (locals.var_za__blk1384 * locals.var_ysat__blk1383_dn5)), ((locals.var_za__blk1384_dn6 * locals.var_ysat__blk1383) + (locals.var_za__blk1384 * locals.var_ysat__blk1383_dn6)), ((locals.var_za__blk1384_dn7 * locals.var_ysat__blk1383) + (locals.var_za__blk1384 * locals.var_ysat__blk1383_dn7)), ((locals.var_za__blk1384_dn8 * locals.var_ysat__blk1383) + (locals.var_za__blk1384 * locals.var_ysat__blk1383_dn8)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
            let assign51640_e66311: f64 = (locals.var_x_inf__blk1382 * locals.var_za__blk1384);
            let assign51640_e66315: f64 = (0.86 * locals.var_temp__blk936);
            let assign51640_e66319: f64 = (locals.var_temp__blk936 * locals.var_za__blk1384);
            let assign51640_e66320: f64 = (1.0 - assign51640_e66319);
            let assign51640_e66321: f64 = (assign51640_e66315 * assign51640_e66320);
            let assign51640_e66325: f64 = (4.0 * locals.var_temp__blk936);
            let assign51640_e66327: f64 = (assign51640_e66325 * locals.var_temp__blk936);
            let assign51640_e66329: f64 = (assign51640_e66327 * locals.var_za__blk1384);
            let assign51640_e66330: f64 = (1.0 + assign51640_e66329);
            let assign51640_e66331: f64 = (assign51640_e66321 / assign51640_e66330);
            let assign51640_e66332: f64 = (1.0 + assign51640_e66331);
            let assign51640_e66333: f64 = (assign51640_e66311 * assign51640_e66332);
            (locals.var_x_0__blk1385, locals.var_x_0__blk1385_dn5, locals.var_x_0__blk1385_dn6, locals.var_x_0__blk1385_dn7, locals.var_x_0__blk1385_dn8, ) = (assign51640_e66333, ((((locals.var_x_inf__blk1382_dn5 * locals.var_za__blk1384) + (locals.var_x_inf__blk1382 * locals.var_za__blk1384_dn5)) * assign51640_e66332) + (assign51640_e66311 * ((((((0.86 * locals.var_temp__blk936_dn5) * assign51640_e66320) + (assign51640_e66315 * (-((locals.var_temp__blk936_dn5 * locals.var_za__blk1384) + (locals.var_temp__blk936 * locals.var_za__blk1384_dn5))))) * assign51640_e66330) - (assign51640_e66321 * (((((4.0 * locals.var_temp__blk936_dn5) * locals.var_temp__blk936) + (assign51640_e66325 * locals.var_temp__blk936_dn5)) * locals.var_za__blk1384) + (assign51640_e66327 * locals.var_za__blk1384_dn5)))) / (assign51640_e66330 * assign51640_e66330)))), ((((locals.var_x_inf__blk1382_dn6 * locals.var_za__blk1384) + (locals.var_x_inf__blk1382 * locals.var_za__blk1384_dn6)) * assign51640_e66332) + (assign51640_e66311 * ((((((0.86 * locals.var_temp__blk936_dn6) * assign51640_e66320) + (assign51640_e66315 * (-((locals.var_temp__blk936_dn6 * locals.var_za__blk1384) + (locals.var_temp__blk936 * locals.var_za__blk1384_dn6))))) * assign51640_e66330) - (assign51640_e66321 * (((((4.0 * locals.var_temp__blk936_dn6) * locals.var_temp__blk936) + (assign51640_e66325 * locals.var_temp__blk936_dn6)) * locals.var_za__blk1384) + (assign51640_e66327 * locals.var_za__blk1384_dn6)))) / (assign51640_e66330 * assign51640_e66330)))), ((((locals.var_x_inf__blk1382_dn7 * locals.var_za__blk1384) + (locals.var_x_inf__blk1382 * locals.var_za__blk1384_dn7)) * assign51640_e66332) + (assign51640_e66311 * ((((((0.86 * locals.var_temp__blk936_dn7) * assign51640_e66320) + (assign51640_e66315 * (-((locals.var_temp__blk936_dn7 * locals.var_za__blk1384) + (locals.var_temp__blk936 * locals.var_za__blk1384_dn7))))) * assign51640_e66330) - (assign51640_e66321 * (((((4.0 * locals.var_temp__blk936_dn7) * locals.var_temp__blk936) + (assign51640_e66325 * locals.var_temp__blk936_dn7)) * locals.var_za__blk1384) + (assign51640_e66327 * locals.var_za__blk1384_dn7)))) / (assign51640_e66330 * assign51640_e66330)))), ((((locals.var_x_inf__blk1382_dn8 * locals.var_za__blk1384) + (locals.var_x_inf__blk1382 * locals.var_za__blk1384_dn8)) * assign51640_e66332) + (assign51640_e66311 * ((((((0.86 * locals.var_temp__blk936_dn8) * assign51640_e66320) + (assign51640_e66315 * (-((locals.var_temp__blk936_dn8 * locals.var_za__blk1384) + (locals.var_temp__blk936 * locals.var_za__blk1384_dn8))))) * assign51640_e66330) - (assign51640_e66321 * (((((4.0 * locals.var_temp__blk936_dn8) * locals.var_temp__blk936) + (assign51640_e66325 * locals.var_temp__blk936_dn8)) * locals.var_za__blk1384) + (assign51640_e66327 * locals.var_za__blk1384_dn8)))) / (assign51640_e66330 * assign51640_e66330)))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
            let assign51650_e66343: f64 = (0.99 * locals.var_x_0__blk1385);
            (locals.var_x_sat__blk1386, locals.var_x_sat__blk1386_dn5, locals.var_x_sat__blk1386_dn6, locals.var_x_sat__blk1386_dn7, locals.var_x_sat__blk1386_dn8, ) = (assign51650_e66343, (0.99 * locals.var_x_0__blk1385_dn5), (0.99 * locals.var_x_0__blk1385_dn6), (0.99 * locals.var_x_0__blk1385_dn7), (0.99 * locals.var_x_0__blk1385_dn8), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
            let assign51660_e66355: f64 = (2.0 * locals.var_asat__blk1372);
            let assign51660_e66356: f64 = (locals.var_x_sat__blk1386 - assign51660_e66355);
            let assign51660_e66357: f64 = (locals.var_x_sat__blk1386 * assign51660_e66356);
            let assign51660_e66359: f64 = (assign51660_e66357 * locals.var_inv_gf2__blk1324);
            let assign51660_e66361: f64 = (assign51660_e66359 / locals.var_ds__blk1353);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign51660_e66361, (((((((locals.var_x_sat__blk1386_dn5 * assign51660_e66356) + (locals.var_x_sat__blk1386 * (locals.var_x_sat__blk1386_dn5 - (2.0 * locals.var_asat__blk1372_dn5)))) * locals.var_inv_gf2__blk1324) + (assign51660_e66357 * locals.var_inv_gf2__blk1324_dn5)) * locals.var_ds__blk1353) - (assign51660_e66359 * locals.var_ds__blk1353_dn5)) / (locals.var_ds__blk1353 * locals.var_ds__blk1353)), (((((((locals.var_x_sat__blk1386_dn6 * assign51660_e66356) + (locals.var_x_sat__blk1386 * (locals.var_x_sat__blk1386_dn6 - (2.0 * locals.var_asat__blk1372_dn6)))) * locals.var_inv_gf2__blk1324) + (assign51660_e66357 * locals.var_inv_gf2__blk1324_dn6)) * locals.var_ds__blk1353) - (assign51660_e66359 * locals.var_ds__blk1353_dn6)) / (locals.var_ds__blk1353 * locals.var_ds__blk1353)), (((((((locals.var_x_sat__blk1386_dn7 * assign51660_e66356) + (locals.var_x_sat__blk1386 * (locals.var_x_sat__blk1386_dn7 - (2.0 * locals.var_asat__blk1372_dn7)))) * locals.var_inv_gf2__blk1324) + (assign51660_e66357 * locals.var_inv_gf2__blk1324_dn7)) * locals.var_ds__blk1353) - (assign51660_e66359 * locals.var_ds__blk1353_dn7)) / (locals.var_ds__blk1353 * locals.var_ds__blk1353)), (((((((locals.var_x_sat__blk1386_dn8 * assign51660_e66356) + (locals.var_x_sat__blk1386 * (locals.var_x_sat__blk1386_dn8 - (2.0 * locals.var_asat__blk1372_dn8)))) * locals.var_inv_gf2__blk1324) + (assign51660_e66357 * locals.var_inv_gf2__blk1324_dn8)) * locals.var_ds__blk1353) - (assign51660_e66359 * locals.var_ds__blk1353_dn8)) / (locals.var_ds__blk1353 * locals.var_ds__blk1353)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
            let assign51670_e66374: f64 = (-0.99);
            let (assign51670_e66379, assign51670_e66379_d_n5, assign51670_e66379_d_n6, assign51670_e66379_d_n7, assign51670_e66379_d_n8,) = {
    if (locals.var_temp__blk936 > assign51670_e66374) {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    } else {
        let assign51670_e66378: f64 = (-0.99);
        (assign51670_e66378, 0.0, 0.0, 0.0, 0.0,)
    }
};
            let assign51670_e66380: f64 = (1.0 + assign51670_e66379);
            let assign51670_e66381: f64 = (assign51670_e66380).ln();
            let assign51670_e66382: f64 = (locals.var_x_sat__blk1386 - assign51670_e66381);
            let assign51670_e66383: f64 = (locals.var_phit1__blk1322 * assign51670_e66382);
            (locals.var_v_dsat__blk1387, locals.var_v_dsat__blk1387_dn5, locals.var_v_dsat__blk1387_dn6, locals.var_v_dsat__blk1387_dn7, locals.var_v_dsat__blk1387_dn8, ) = (assign51670_e66383, ((locals.var_phit1__blk1322_dn5 * assign51670_e66382) + (locals.var_phit1__blk1322 * (locals.var_x_sat__blk1386_dn5 - (assign51670_e66379_d_n5 / assign51670_e66380)))), ((locals.var_phit1__blk1322_dn6 * assign51670_e66382) + (locals.var_phit1__blk1322 * (locals.var_x_sat__blk1386_dn6 - (assign51670_e66379_d_n6 / assign51670_e66380)))), ((locals.var_phit1__blk1322_dn7 * assign51670_e66382) + (locals.var_phit1__blk1322 * (locals.var_x_sat__blk1386_dn7 - (assign51670_e66379_d_n7 / assign51670_e66380)))), ((locals.var_phit1__blk1322_dn8 * assign51670_e66382) + (locals.var_phit1__blk1322 * (locals.var_x_sat__blk1386_dn8 - (assign51670_e66379_d_n8 / assign51670_e66380)))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 == 0.0)) {
            (locals.var_v_dsat__blk1387, locals.var_v_dsat__blk1387_dn5, locals.var_v_dsat__blk1387_dn6, locals.var_v_dsat__blk1387_dn7, locals.var_v_dsat__blk1387_dn8, ) = (locals.var_vdsat_lim__blk1370, locals.var_vdsat_lim__blk1370_dn5, locals.var_vdsat_lim__blk1370_dn6, locals.var_vdsat_lim__blk1370_dn7, locals.var_vdsat_lim__blk1370_dn8, );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign51690_e66400: f64 = (1.0 + locals.var_arloc__blk1303);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign51690_e66400, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign51700_e66407: f64 = (locals.var_temp__blk936).sqrt();
            let assign51700_e66409: f64 = (assign51700_e66407 * locals.var_v_ds);
            let assign51700_e66411: f64 = (assign51700_e66409 / locals.var_v_dsat__blk1387);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign51700_e66411, (((((locals.var_temp__blk936_dn5 / (2.0 * assign51700_e66407)) * locals.var_v_ds) * locals.var_v_dsat__blk1387) - (assign51700_e66409 * locals.var_v_dsat__blk1387_dn5)) / (locals.var_v_dsat__blk1387 * locals.var_v_dsat__blk1387)), ((((((locals.var_temp__blk936_dn6 / (2.0 * assign51700_e66407)) * locals.var_v_ds) + (assign51700_e66407 * locals.var_v_ds_dn6)) * locals.var_v_dsat__blk1387) - (assign51700_e66409 * locals.var_v_dsat__blk1387_dn6)) / (locals.var_v_dsat__blk1387 * locals.var_v_dsat__blk1387)), ((((((locals.var_temp__blk936_dn7 / (2.0 * assign51700_e66407)) * locals.var_v_ds) + (assign51700_e66407 * locals.var_v_ds_dn7)) * locals.var_v_dsat__blk1387) - (assign51700_e66409 * locals.var_v_dsat__blk1387_dn7)) / (locals.var_v_dsat__blk1387 * locals.var_v_dsat__blk1387)), (((((locals.var_temp__blk936_dn8 / (2.0 * assign51700_e66407)) * locals.var_v_ds) * locals.var_v_dsat__blk1387) - (assign51700_e66409 * locals.var_v_dsat__blk1387_dn8)) / (locals.var_v_dsat__blk1387 * locals.var_v_dsat__blk1387)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign51710_e66419: f64 = (locals.var_temp1 * locals.var_temp1);
            let assign51710_e66421: f64 = (assign51710_e66419 + locals.var_temp__blk936);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign51710_e66421, (((locals.var_temp1_dn5 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn5)) + locals.var_temp__blk936_dn5), (((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)) + locals.var_temp__blk936_dn6), (((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)) + locals.var_temp__blk936_dn7), (((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)) + locals.var_temp__blk936_dn8), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign51720_e66429: f64 = (2.0 * locals.var_temp1);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign51720_e66429, (2.0 * locals.var_temp1_dn5), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign51730_e66437: f64 = (locals.var_v_dsat__blk1387 * locals.var_temp__blk936);
            let assign51730_e66440: f64 = (locals.var_temp2 - locals.var_temp__blk936);
            let assign51730_e66441: f64 = (assign51730_e66440).sqrt();
            let assign51730_e66444: f64 = (locals.var_temp2 + locals.var_temp__blk936);
            let assign51730_e66445: f64 = (assign51730_e66444).sqrt();
            let assign51730_e66446: f64 = (assign51730_e66441 + assign51730_e66445);
            let assign51730_e66447: f64 = (assign51730_e66437 / assign51730_e66446);
            (locals.var_vdse__blk1388, locals.var_vdse__blk1388_dn5, locals.var_vdse__blk1388_dn6, locals.var_vdse__blk1388_dn7, locals.var_vdse__blk1388_dn8, ) = (assign51730_e66447, (((((locals.var_v_dsat__blk1387_dn5 * locals.var_temp__blk936) + (locals.var_v_dsat__blk1387 * locals.var_temp__blk936_dn5)) * assign51730_e66446) - (assign51730_e66437 * (((locals.var_temp2_dn5 - locals.var_temp__blk936_dn5) / (2.0 * assign51730_e66441)) + ((locals.var_temp2_dn5 + locals.var_temp__blk936_dn5) / (2.0 * assign51730_e66445))))) / (assign51730_e66446 * assign51730_e66446)), (((((locals.var_v_dsat__blk1387_dn6 * locals.var_temp__blk936) + (locals.var_v_dsat__blk1387 * locals.var_temp__blk936_dn6)) * assign51730_e66446) - (assign51730_e66437 * (((locals.var_temp2_dn6 - locals.var_temp__blk936_dn6) / (2.0 * assign51730_e66441)) + ((locals.var_temp2_dn6 + locals.var_temp__blk936_dn6) / (2.0 * assign51730_e66445))))) / (assign51730_e66446 * assign51730_e66446)), (((((locals.var_v_dsat__blk1387_dn7 * locals.var_temp__blk936) + (locals.var_v_dsat__blk1387 * locals.var_temp__blk936_dn7)) * assign51730_e66446) - (assign51730_e66437 * (((locals.var_temp2_dn7 - locals.var_temp__blk936_dn7) / (2.0 * assign51730_e66441)) + ((locals.var_temp2_dn7 + locals.var_temp__blk936_dn7) / (2.0 * assign51730_e66445))))) / (assign51730_e66446 * assign51730_e66446)), (((((locals.var_v_dsat__blk1387_dn8 * locals.var_temp__blk936) + (locals.var_v_dsat__blk1387 * locals.var_temp__blk936_dn8)) * assign51730_e66446) - (assign51730_e66437 * (((locals.var_temp2_dn8 - locals.var_temp__blk936_dn8) / (2.0 * assign51730_e66441)) + ((locals.var_temp2_dn8 + locals.var_temp__blk936_dn8) / (2.0 * assign51730_e66445))))) / (assign51730_e66446 * assign51730_e66446)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign51740_e66455: f64 = (locals.var_vdse__blk1388 * locals.var_inv_phit1__blk1323);
            (locals.var_udse__blk1389, locals.var_udse__blk1389_dn5, locals.var_udse__blk1389_dn6, locals.var_udse__blk1389_dn7, locals.var_udse__blk1389_dn8, ) = (assign51740_e66455, ((locals.var_vdse__blk1388_dn5 * locals.var_inv_phit1__blk1323) + (locals.var_vdse__blk1388 * locals.var_inv_phit1__blk1323_dn5)), ((locals.var_vdse__blk1388_dn6 * locals.var_inv_phit1__blk1323) + (locals.var_vdse__blk1388 * locals.var_inv_phit1__blk1323_dn6)), ((locals.var_vdse__blk1388_dn7 * locals.var_inv_phit1__blk1323) + (locals.var_vdse__blk1388 * locals.var_inv_phit1__blk1323_dn7)), ((locals.var_vdse__blk1388_dn8 * locals.var_inv_phit1__blk1323) + (locals.var_vdse__blk1388 * locals.var_inv_phit1__blk1323_dn8)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign51750_e66463: f64 = (locals.var_xn_s__blk1332 + locals.var_udse__blk1389);
            (locals.var_xn_d__blk1390, locals.var_xn_d__blk1390_dn5, locals.var_xn_d__blk1390_dn6, locals.var_xn_d__blk1390_dn7, locals.var_xn_d__blk1390_dn8, ) = (assign51750_e66463, (locals.var_xn_s__blk1332_dn5 + locals.var_udse__blk1389_dn5), (locals.var_xn_s__blk1332_dn6 + locals.var_udse__blk1389_dn6), (locals.var_xn_s__blk1332_dn7 + locals.var_udse__blk1389_dn7), (locals.var_xn_s__blk1332_dn8 + locals.var_udse__blk1389_dn8), );
        }

        let assign51760_e66468: f64 = if locals.var_udse__blk1389 < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1491 = assign51760_e66468;

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1491 != 0.0)) {
            let assign51770_e66475: f64 = (-locals.var_udse__blk1389);
            let assign51770_e66476: f64 = (assign51770_e66475).exp();
            (locals.var_k_ds__blk1391, locals.var_k_ds__blk1391_dn5, locals.var_k_ds__blk1391_dn6, locals.var_k_ds__blk1391_dn7, locals.var_k_ds__blk1391_dn8, ) = (assign51770_e66476, (assign51770_e66476 * (-locals.var_udse__blk1389_dn5)), (assign51770_e66476 * (-locals.var_udse__blk1389_dn6)), (assign51770_e66476 * (-locals.var_udse__blk1389_dn7)), (assign51770_e66476 * (-locals.var_udse__blk1389_dn8)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1491 == 0.0)) {
            let assign51780_e66489: f64 = (locals.var_udse__blk1389 - 460.51701859880916);
            let assign51780_e66494: f64 = (locals.var_udse__blk1389 - 460.51701859880916);
            let assign51780_e66498: f64 = (locals.var_udse__blk1389 - 460.51701859880916);
            let assign51780_e66500: f64 = (assign51780_e66498 * 0.3333333333333333);
            let assign51780_e66501: f64 = (1.0 + assign51780_e66500);
            let assign51780_e66502: f64 = (assign51780_e66494 * assign51780_e66501);
            let assign51780_e66503: f64 = (0.5 * assign51780_e66502);
            let assign51780_e66504: f64 = (1.0 + assign51780_e66503);
            let assign51780_e66505: f64 = (assign51780_e66489 * assign51780_e66504);
            let assign51780_e66506: f64 = (1.0 + assign51780_e66505);
            let assign51780_e66507: f64 = (1e-200 / assign51780_e66506);
            (locals.var_k_ds__blk1391, locals.var_k_ds__blk1391_dn5, locals.var_k_ds__blk1391_dn6, locals.var_k_ds__blk1391_dn7, locals.var_k_ds__blk1391_dn8, ) = (assign51780_e66507, (-((1e-200 * ((locals.var_udse__blk1389_dn5 * assign51780_e66504) + (assign51780_e66489 * (0.5 * ((locals.var_udse__blk1389_dn5 * assign51780_e66501) + (assign51780_e66494 * (locals.var_udse__blk1389_dn5 * 0.3333333333333333))))))) / (assign51780_e66506 * assign51780_e66506))), (-((1e-200 * ((locals.var_udse__blk1389_dn6 * assign51780_e66504) + (assign51780_e66489 * (0.5 * ((locals.var_udse__blk1389_dn6 * assign51780_e66501) + (assign51780_e66494 * (locals.var_udse__blk1389_dn6 * 0.3333333333333333))))))) / (assign51780_e66506 * assign51780_e66506))), (-((1e-200 * ((locals.var_udse__blk1389_dn7 * assign51780_e66504) + (assign51780_e66489 * (0.5 * ((locals.var_udse__blk1389_dn7 * assign51780_e66501) + (assign51780_e66494 * (locals.var_udse__blk1389_dn7 * 0.3333333333333333))))))) / (assign51780_e66506 * assign51780_e66506))), (-((1e-200 * ((locals.var_udse__blk1389_dn8 * assign51780_e66504) + (assign51780_e66489 * (0.5 * ((locals.var_udse__blk1389_dn8 * assign51780_e66501) + (assign51780_e66494 * (locals.var_udse__blk1389_dn8 * 0.3333333333333333))))))) / (assign51780_e66506 * assign51780_e66506))), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign51790_e66515: f64 = (locals.var_delta_ns__blk1347 * locals.var_k_ds__blk1391);
            (locals.var_delta_nd__blk1392, locals.var_delta_nd__blk1392_dn5, locals.var_delta_nd__blk1392_dn6, locals.var_delta_nd__blk1392_dn7, locals.var_delta_nd__blk1392_dn8, ) = (assign51790_e66515, ((locals.var_delta_ns__blk1347_dn5 * locals.var_k_ds__blk1391) + (locals.var_delta_ns__blk1347 * locals.var_k_ds__blk1391_dn5)), ((locals.var_delta_ns__blk1347_dn6 * locals.var_k_ds__blk1391) + (locals.var_delta_ns__blk1347 * locals.var_k_ds__blk1391_dn6)), ((locals.var_delta_ns__blk1347_dn7 * locals.var_k_ds__blk1391) + (locals.var_delta_ns__blk1347 * locals.var_k_ds__blk1391_dn7)), ((locals.var_delta_ns__blk1347_dn8 * locals.var_k_ds__blk1391) + (locals.var_delta_ns__blk1347 * locals.var_k_ds__blk1391_dn8)), );
        }

        let assign51800_e66519: f64 = (locals.var_xg__blk1326).abs();
        let assign51800_e66521: f64 = if assign51800_e66519 <= locals.var_margin__blk1344 { 1.0 } else { 0.0 };
        locals.var_guard1492 = assign51800_e66521;

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 != 0.0)) {
            let assign51810_e66529: f64 = (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345);
            let assign51810_e66531: f64 = (assign51810_e66529 * 0.16666666666666666);
            let assign51810_e66533: f64 = (assign51810_e66531 * 0.7071067811865475);
            (locals.var_sp_s_temp1__blk1432, locals.var_sp_s_temp1__blk1432_dn5, locals.var_sp_s_temp1__blk1432_dn6, locals.var_sp_s_temp1__blk1432_dn7, locals.var_sp_s_temp1__blk1432_dn8, ) = (assign51810_e66533, ((((locals.var_inv_xi__blk1345_dn5 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn5)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1345_dn6 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1345_dn7 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi__blk1345_dn8 * locals.var_inv_xi__blk1345) + (locals.var_inv_xi__blk1345 * locals.var_inv_xi__blk1345_dn8)) * 0.16666666666666666) * 0.7071067811865475), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 != 0.0)) {
            let assign51820_e66543: f64 = (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345);
            let assign51820_e66548: f64 = (1.0 - locals.var_delta_nd__blk1392);
            let assign51820_e66549: f64 = (locals.var_xg__blk1326 * assign51820_e66548);
            let assign51820_e66551: f64 = (assign51820_e66549 * locals.var_gf__blk1307);
            let assign51820_e66553: f64 = (assign51820_e66551 * locals.var_sp_s_temp1__blk1432);
            let assign51820_e66554: f64 = (1.0 + assign51820_e66553);
            let assign51820_e66555: f64 = (assign51820_e66543 * assign51820_e66554);
            (locals.var_x_d__blk1393, locals.var_x_d__blk1393_dn5, locals.var_x_d__blk1393_dn6, locals.var_x_d__blk1393_dn7, locals.var_x_d__blk1393_dn8, ) = (assign51820_e66555, ((((locals.var_xg__blk1326_dn5 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn5)) * assign51820_e66554) + (assign51820_e66543 * ((((((locals.var_xg__blk1326_dn5 * assign51820_e66548) + (locals.var_xg__blk1326 * (-locals.var_delta_nd__blk1392_dn5))) * locals.var_gf__blk1307) + (assign51820_e66549 * locals.var_gf__blk1307_dn5)) * locals.var_sp_s_temp1__blk1432) + (assign51820_e66551 * locals.var_sp_s_temp1__blk1432_dn5)))), ((((locals.var_xg__blk1326_dn6 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn6)) * assign51820_e66554) + (assign51820_e66543 * ((((((locals.var_xg__blk1326_dn6 * assign51820_e66548) + (locals.var_xg__blk1326 * (-locals.var_delta_nd__blk1392_dn6))) * locals.var_gf__blk1307) + (assign51820_e66549 * locals.var_gf__blk1307_dn6)) * locals.var_sp_s_temp1__blk1432) + (assign51820_e66551 * locals.var_sp_s_temp1__blk1432_dn6)))), ((((locals.var_xg__blk1326_dn7 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn7)) * assign51820_e66554) + (assign51820_e66543 * ((((((locals.var_xg__blk1326_dn7 * assign51820_e66548) + (locals.var_xg__blk1326 * (-locals.var_delta_nd__blk1392_dn7))) * locals.var_gf__blk1307) + (assign51820_e66549 * locals.var_gf__blk1307_dn7)) * locals.var_sp_s_temp1__blk1432) + (assign51820_e66551 * locals.var_sp_s_temp1__blk1432_dn7)))), ((((locals.var_xg__blk1326_dn8 * locals.var_inv_xi__blk1345) + (locals.var_xg__blk1326 * locals.var_inv_xi__blk1345_dn8)) * assign51820_e66554) + (assign51820_e66543 * ((((((locals.var_xg__blk1326_dn8 * assign51820_e66548) + (locals.var_xg__blk1326 * (-locals.var_delta_nd__blk1392_dn8))) * locals.var_gf__blk1307) + (assign51820_e66549 * locals.var_gf__blk1307_dn8)) * locals.var_sp_s_temp1__blk1432) + (assign51820_e66551 * locals.var_sp_s_temp1__blk1432_dn8)))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign51830_e66566: f64 = (locals.var_xn_d__blk1390 + 3.0);
            (locals.var_sp_s_bx__blk1453, locals.var_sp_s_bx__blk1453_dn5, locals.var_sp_s_bx__blk1453_dn6, locals.var_sp_s_bx__blk1453_dn7, locals.var_sp_s_bx__blk1453_dn8, ) = (assign51830_e66566, locals.var_xn_d__blk1390_dn5, locals.var_xn_d__blk1390_dn6, locals.var_xn_d__blk1390_dn7, locals.var_xn_d__blk1390_dn8, );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign51840_e66578: f64 = (locals.var_sp_s_x1__blk1452 + locals.var_sp_s_bx__blk1453);
            let assign51840_e66581: f64 = (locals.var_sp_s_x1__blk1452 - locals.var_sp_s_bx__blk1453);
            let assign51840_e66584: f64 = (locals.var_sp_s_x1__blk1452 - locals.var_sp_s_bx__blk1453);
            let assign51840_e66585: f64 = (assign51840_e66581 * assign51840_e66584);
            let assign51840_e66587: f64 = (assign51840_e66585 + 5.0);
            let assign51840_e66588: f64 = (assign51840_e66587).sqrt();
            let assign51840_e66589: f64 = (assign51840_e66578 - assign51840_e66588);
            let assign51840_e66590: f64 = (0.5 * assign51840_e66589);
            let assign51840_e66595: f64 = (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453);
            let assign51840_e66597: f64 = (assign51840_e66595 + 5.0);
            let assign51840_e66598: f64 = (assign51840_e66597).sqrt();
            let assign51840_e66599: f64 = (locals.var_sp_s_bx__blk1453 - assign51840_e66598);
            let assign51840_e66600: f64 = (0.5 * assign51840_e66599);
            let assign51840_e66601: f64 = (assign51840_e66590 - assign51840_e66600);
            (locals.var_sp_s_eta__blk1436, locals.var_sp_s_eta__blk1436_dn5, locals.var_sp_s_eta__blk1436_dn6, locals.var_sp_s_eta__blk1436_dn7, locals.var_sp_s_eta__blk1436_dn8, ) = (assign51840_e66601, ((0.5 * ((locals.var_sp_s_x1__blk1452_dn5 + locals.var_sp_s_bx__blk1453_dn5) - ((((locals.var_sp_s_x1__blk1452_dn5 - locals.var_sp_s_bx__blk1453_dn5) * assign51840_e66584) + (assign51840_e66581 * (locals.var_sp_s_x1__blk1452_dn5 - locals.var_sp_s_bx__blk1453_dn5))) / (2.0 * assign51840_e66588)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn5 - (((locals.var_sp_s_bx__blk1453_dn5 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn5)) / (2.0 * assign51840_e66598))))), ((0.5 * ((locals.var_sp_s_x1__blk1452_dn6 + locals.var_sp_s_bx__blk1453_dn6) - ((((locals.var_sp_s_x1__blk1452_dn6 - locals.var_sp_s_bx__blk1453_dn6) * assign51840_e66584) + (assign51840_e66581 * (locals.var_sp_s_x1__blk1452_dn6 - locals.var_sp_s_bx__blk1453_dn6))) / (2.0 * assign51840_e66588)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn6 - (((locals.var_sp_s_bx__blk1453_dn6 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn6)) / (2.0 * assign51840_e66598))))), ((0.5 * ((locals.var_sp_s_x1__blk1452_dn7 + locals.var_sp_s_bx__blk1453_dn7) - ((((locals.var_sp_s_x1__blk1452_dn7 - locals.var_sp_s_bx__blk1453_dn7) * assign51840_e66584) + (assign51840_e66581 * (locals.var_sp_s_x1__blk1452_dn7 - locals.var_sp_s_bx__blk1453_dn7))) / (2.0 * assign51840_e66588)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn7 - (((locals.var_sp_s_bx__blk1453_dn7 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn7)) / (2.0 * assign51840_e66598))))), ((0.5 * ((locals.var_sp_s_x1__blk1452_dn8 + locals.var_sp_s_bx__blk1453_dn8) - ((((locals.var_sp_s_x1__blk1452_dn8 - locals.var_sp_s_bx__blk1453_dn8) * assign51840_e66584) + (assign51840_e66581 * (locals.var_sp_s_x1__blk1452_dn8 - locals.var_sp_s_bx__blk1453_dn8))) / (2.0 * assign51840_e66588)))) - (0.5 * (locals.var_sp_s_bx__blk1453_dn8 - (((locals.var_sp_s_bx__blk1453_dn8 * locals.var_sp_s_bx__blk1453) + (locals.var_sp_s_bx__blk1453 * locals.var_sp_s_bx__blk1453_dn8)) / (2.0 * assign51840_e66598))))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign51850_e66612: f64 = (locals.var_xg__blk1326 - locals.var_sp_s_eta__blk1436);
            (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8, ) = (assign51850_e66612, (locals.var_xg__blk1326_dn5 - locals.var_sp_s_eta__blk1436_dn5), (locals.var_xg__blk1326_dn6 - locals.var_sp_s_eta__blk1436_dn6), (locals.var_xg__blk1326_dn7 - locals.var_sp_s_eta__blk1436_dn7), (locals.var_xg__blk1326_dn8 - locals.var_sp_s_eta__blk1436_dn8), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign51860_e66622: f64 = (-locals.var_sp_s_eta__blk1436);
            let assign51860_e66623: f64 = (assign51860_e66622).exp();
            (locals.var_sp_s_temp1__blk1432, locals.var_sp_s_temp1__blk1432_dn5, locals.var_sp_s_temp1__blk1432_dn6, locals.var_sp_s_temp1__blk1432_dn7, locals.var_sp_s_temp1__blk1432_dn8, ) = (assign51860_e66623, (assign51860_e66623 * (-locals.var_sp_s_eta__blk1436_dn5)), (assign51860_e66623 * (-locals.var_sp_s_eta__blk1436_dn6)), (assign51860_e66623 * (-locals.var_sp_s_eta__blk1436_dn7)), (assign51860_e66623 * (-locals.var_sp_s_eta__blk1436_dn8)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign51870_e66636: f64 = (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436);
            let assign51870_e66637: f64 = (2.0 + assign51870_e66636);
            let assign51870_e66638: f64 = (1.0 / assign51870_e66637);
            (locals.var_sp_s_temp2__blk1433, locals.var_sp_s_temp2__blk1433_dn5, locals.var_sp_s_temp2__blk1433_dn6, locals.var_sp_s_temp2__blk1433_dn7, locals.var_sp_s_temp2__blk1433_dn8, ) = (assign51870_e66638, (-(((locals.var_sp_s_eta__blk1436_dn5 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn5)) / (assign51870_e66637 * assign51870_e66637))), (-(((locals.var_sp_s_eta__blk1436_dn6 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn6)) / (assign51870_e66637 * assign51870_e66637))), (-(((locals.var_sp_s_eta__blk1436_dn7 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn7)) / (assign51870_e66637 * assign51870_e66637))), (-(((locals.var_sp_s_eta__blk1436_dn8 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn8)) / (assign51870_e66637 * assign51870_e66637))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign51880_e66649: f64 = (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436);
            let assign51880_e66651: f64 = (assign51880_e66649 * locals.var_sp_s_temp2__blk1433);
            (locals.var_sp_s_xi0__blk1443, locals.var_sp_s_xi0__blk1443_dn5, locals.var_sp_s_xi0__blk1443_dn6, locals.var_sp_s_xi0__blk1443_dn7, locals.var_sp_s_xi0__blk1443_dn8, ) = (assign51880_e66651, ((((locals.var_sp_s_eta__blk1436_dn5 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign51880_e66649 * locals.var_sp_s_temp2__blk1433_dn5)), ((((locals.var_sp_s_eta__blk1436_dn6 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign51880_e66649 * locals.var_sp_s_temp2__blk1433_dn6)), ((((locals.var_sp_s_eta__blk1436_dn7 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign51880_e66649 * locals.var_sp_s_temp2__blk1433_dn7)), ((((locals.var_sp_s_eta__blk1436_dn8 * locals.var_sp_s_eta__blk1436) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_eta__blk1436_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign51880_e66649 * locals.var_sp_s_temp2__blk1433_dn8)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign51890_e66663: f64 = (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433);
            let assign51890_e66665: f64 = (assign51890_e66663 * locals.var_sp_s_temp2__blk1433);
            let assign51890_e66666: f64 = (4.0 * assign51890_e66665);
            (locals.var_sp_s_xi1__blk1444, locals.var_sp_s_xi1__blk1444_dn5, locals.var_sp_s_xi1__blk1444_dn6, locals.var_sp_s_xi1__blk1444_dn7, locals.var_sp_s_xi1__blk1444_dn8, ) = (assign51890_e66666, (4.0 * ((((locals.var_sp_s_eta__blk1436_dn5 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign51890_e66663 * locals.var_sp_s_temp2__blk1433_dn5))), (4.0 * ((((locals.var_sp_s_eta__blk1436_dn6 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign51890_e66663 * locals.var_sp_s_temp2__blk1433_dn6))), (4.0 * ((((locals.var_sp_s_eta__blk1436_dn7 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign51890_e66663 * locals.var_sp_s_temp2__blk1433_dn7))), (4.0 * ((((locals.var_sp_s_eta__blk1436_dn8 * locals.var_sp_s_temp2__blk1433) + (locals.var_sp_s_eta__blk1436 * locals.var_sp_s_temp2__blk1433_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign51890_e66663 * locals.var_sp_s_temp2__blk1433_dn8))), );
        }

    }

    pub(super) fn stamp_transient_block_24(
        locals: &mut StampLocals,
    ) {
        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign51900_e66677: f64 = (8.0 * locals.var_sp_s_temp2__blk1433);
            let assign51900_e66680: f64 = (12.0 * locals.var_sp_s_xi0__blk1443);
            let assign51900_e66681: f64 = (assign51900_e66677 - assign51900_e66680);
            let assign51900_e66683: f64 = (assign51900_e66681 * locals.var_sp_s_temp2__blk1433);
            let assign51900_e66685: f64 = (assign51900_e66683 * locals.var_sp_s_temp2__blk1433);
            (locals.var_sp_s_xi2__blk1445, locals.var_sp_s_xi2__blk1445_dn5, locals.var_sp_s_xi2__blk1445_dn6, locals.var_sp_s_xi2__blk1445_dn7, locals.var_sp_s_xi2__blk1445_dn8, ) = (assign51900_e66685, ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn5) - (12.0 * locals.var_sp_s_xi0__blk1443_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66681 * locals.var_sp_s_temp2__blk1433_dn5)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66683 * locals.var_sp_s_temp2__blk1433_dn5)), ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn6) - (12.0 * locals.var_sp_s_xi0__blk1443_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66681 * locals.var_sp_s_temp2__blk1433_dn6)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66683 * locals.var_sp_s_temp2__blk1433_dn6)), ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn7) - (12.0 * locals.var_sp_s_xi0__blk1443_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66681 * locals.var_sp_s_temp2__blk1433_dn7)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66683 * locals.var_sp_s_temp2__blk1433_dn7)), ((((((8.0 * locals.var_sp_s_temp2__blk1433_dn8) - (12.0 * locals.var_sp_s_xi0__blk1443_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66681 * locals.var_sp_s_temp2__blk1433_dn8)) * locals.var_sp_s_temp2__blk1433) + (assign51900_e66683 * locals.var_sp_s_temp2__blk1433_dn8)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign51910_e66697: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
            let assign51910_e66701: f64 = (locals.var_sp_s_temp1__blk1432 + locals.var_sp_s_eta__blk1436);
            let assign51910_e66703: f64 = (assign51910_e66701 - 1.0);
            let assign51910_e66707: f64 = (locals.var_sp_s_eta__blk1436 + 1.0);
            let assign51910_e66709: f64 = (assign51910_e66707 + locals.var_sp_s_xi0__blk1443);
            let assign51910_e66710: f64 = (locals.var_delta_nd__blk1392 * assign51910_e66709);
            let assign51910_e66711: f64 = (assign51910_e66703 - assign51910_e66710);
            let assign51910_e66712: f64 = (locals.var_gf2__blk1308 * assign51910_e66711);
            let assign51910_e66713: f64 = (assign51910_e66697 - assign51910_e66712);
            let (assign51910_e66735, assign51910_e66735_d_n5, assign51910_e66735_d_n6, assign51910_e66735_d_n7, assign51910_e66735_d_n8,) = {
    if (1e-40 > assign51910_e66713) {
        (1e-40, 0.0, 0.0, 0.0, 0.0,)
    } else {
        let assign51910_e66718: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
        let assign51910_e66722: f64 = (locals.var_sp_s_temp1__blk1432 + locals.var_sp_s_eta__blk1436);
        let assign51910_e66724: f64 = (assign51910_e66722 - 1.0);
        let assign51910_e66728: f64 = (locals.var_sp_s_eta__blk1436 + 1.0);
        let assign51910_e66730: f64 = (assign51910_e66728 + locals.var_sp_s_xi0__blk1443);
        let assign51910_e66731: f64 = (locals.var_delta_nd__blk1392 * assign51910_e66730);
        let assign51910_e66732: f64 = (assign51910_e66724 - assign51910_e66731);
        let assign51910_e66733: f64 = (locals.var_gf2__blk1308 * assign51910_e66732);
        let assign51910_e66734: f64 = (assign51910_e66718 - assign51910_e66733);
        (assign51910_e66734, (((locals.var_sp_s_temp__blk1431_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn5)) - ((locals.var_gf2__blk1308_dn5 * assign51910_e66732) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn5 + locals.var_sp_s_eta__blk1436_dn5) - ((locals.var_delta_nd__blk1392_dn5 * assign51910_e66730) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_eta__blk1436_dn5 + locals.var_sp_s_xi0__blk1443_dn5))))))), (((locals.var_sp_s_temp__blk1431_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn6)) - ((locals.var_gf2__blk1308_dn6 * assign51910_e66732) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn6 + locals.var_sp_s_eta__blk1436_dn6) - ((locals.var_delta_nd__blk1392_dn6 * assign51910_e66730) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_eta__blk1436_dn6 + locals.var_sp_s_xi0__blk1443_dn6))))))), (((locals.var_sp_s_temp__blk1431_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn7)) - ((locals.var_gf2__blk1308_dn7 * assign51910_e66732) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn7 + locals.var_sp_s_eta__blk1436_dn7) - ((locals.var_delta_nd__blk1392_dn7 * assign51910_e66730) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_eta__blk1436_dn7 + locals.var_sp_s_xi0__blk1443_dn7))))))), (((locals.var_sp_s_temp__blk1431_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn8)) - ((locals.var_gf2__blk1308_dn8 * assign51910_e66732) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_temp1__blk1432_dn8 + locals.var_sp_s_eta__blk1436_dn8) - ((locals.var_delta_nd__blk1392_dn8 * assign51910_e66730) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_eta__blk1436_dn8 + locals.var_sp_s_xi0__blk1443_dn8))))))),)
    }
};
            (locals.var_sp_s_a__blk1437, locals.var_sp_s_a__blk1437_dn5, locals.var_sp_s_a__blk1437_dn6, locals.var_sp_s_a__blk1437_dn7, locals.var_sp_s_a__blk1437_dn8, ) = (assign51910_e66735, assign51910_e66735_d_n5, assign51910_e66735_d_n6, assign51910_e66735_d_n7, assign51910_e66735_d_n8, );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign51920_e66750: f64 = (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445);
            let assign51920_e66751: f64 = (locals.var_sp_s_temp1__blk1432 - assign51920_e66750);
            let assign51920_e66752: f64 = (locals.var_gf2__blk1308 * assign51920_e66751);
            let assign51920_e66753: f64 = (0.5 * assign51920_e66752);
            let assign51920_e66754: f64 = (1.0 - assign51920_e66753);
            (locals.var_sp_s_b__blk1454, locals.var_sp_s_b__blk1454_dn5, locals.var_sp_s_b__blk1454_dn6, locals.var_sp_s_b__blk1454_dn7, locals.var_sp_s_b__blk1454_dn8, ) = (assign51920_e66754, (-(0.5 * ((locals.var_gf2__blk1308_dn5 * assign51920_e66751) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn5 - ((locals.var_delta_nd__blk1392_dn5 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn5))))))), (-(0.5 * ((locals.var_gf2__blk1308_dn6 * assign51920_e66751) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn6 - ((locals.var_delta_nd__blk1392_dn6 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn6))))))), (-(0.5 * ((locals.var_gf2__blk1308_dn7 * assign51920_e66751) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn7 - ((locals.var_delta_nd__blk1392_dn7 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn7))))))), (-(0.5 * ((locals.var_gf2__blk1308_dn8 * assign51920_e66751) + (locals.var_gf2__blk1308 * (locals.var_sp_s_temp1__blk1432_dn8 - ((locals.var_delta_nd__blk1392_dn8 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn8))))))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign51930_e66765: f64 = (2.0 * locals.var_sp_s_temp__blk1431);
            let assign51930_e66769: f64 = (1.0 - locals.var_sp_s_temp1__blk1432);
            let assign51930_e66773: f64 = (1.0 + locals.var_sp_s_xi1__blk1444);
            let assign51930_e66774: f64 = (locals.var_delta_nd__blk1392 * assign51930_e66773);
            let assign51930_e66775: f64 = (assign51930_e66769 - assign51930_e66774);
            let assign51930_e66776: f64 = (locals.var_gf2__blk1308 * assign51930_e66775);
            let assign51930_e66777: f64 = (assign51930_e66765 + assign51930_e66776);
            (locals.var_sp_s_c__blk1438, locals.var_sp_s_c__blk1438_dn5, locals.var_sp_s_c__blk1438_dn6, locals.var_sp_s_c__blk1438_dn7, locals.var_sp_s_c__blk1438_dn8, ) = (assign51930_e66777, ((2.0 * locals.var_sp_s_temp__blk1431_dn5) + ((locals.var_gf2__blk1308_dn5 * assign51930_e66775) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn5) - ((locals.var_delta_nd__blk1392_dn5 * assign51930_e66773) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn5)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn6) + ((locals.var_gf2__blk1308_dn6 * assign51930_e66775) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn6) - ((locals.var_delta_nd__blk1392_dn6 * assign51930_e66773) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn7) + ((locals.var_gf2__blk1308_dn7 * assign51930_e66775) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn7) - ((locals.var_delta_nd__blk1392_dn7 * assign51930_e66773) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn8) + ((locals.var_gf2__blk1308_dn8 * assign51930_e66775) + (locals.var_gf2__blk1308 * ((-locals.var_sp_s_temp1__blk1432_dn8) - ((locals.var_delta_nd__blk1392_dn8 * assign51930_e66773) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn8)))))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign51940_e66788: f64 = (locals.var_xn_d__blk1390 - locals.var_sp_s_eta__blk1436);
            let assign51940_e66791: f64 = (locals.var_sp_s_a__blk1437 / locals.var_gf2__blk1308);
            let assign51940_e66792: f64 = (assign51940_e66791).ln();
            let assign51940_e66793: f64 = (assign51940_e66788 + assign51940_e66792);
            (locals.var_sp_s_tau__blk1439, locals.var_sp_s_tau__blk1439_dn5, locals.var_sp_s_tau__blk1439_dn6, locals.var_sp_s_tau__blk1439_dn7, locals.var_sp_s_tau__blk1439_dn8, ) = (assign51940_e66793, ((locals.var_xn_d__blk1390_dn5 - locals.var_sp_s_eta__blk1436_dn5) + ((((locals.var_sp_s_a__blk1437_dn5 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn5)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign51940_e66791)), ((locals.var_xn_d__blk1390_dn6 - locals.var_sp_s_eta__blk1436_dn6) + ((((locals.var_sp_s_a__blk1437_dn6 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn6)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign51940_e66791)), ((locals.var_xn_d__blk1390_dn7 - locals.var_sp_s_eta__blk1436_dn7) + ((((locals.var_sp_s_a__blk1437_dn7 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn7)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign51940_e66791)), ((locals.var_xn_d__blk1390_dn8 - locals.var_sp_s_eta__blk1436_dn8) + ((((locals.var_sp_s_a__blk1437_dn8 * locals.var_gf2__blk1308) - (locals.var_sp_s_a__blk1437 * locals.var_gf2__blk1308_dn8)) / (locals.var_gf2__blk1308 * locals.var_gf2__blk1308)) / assign51940_e66791)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign51950_e66804: f64 = (locals.var_sp_s_a__blk1437 + locals.var_sp_s_c__blk1438);
            (locals.var_nu, locals.var_nu_dn5, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, ) = (assign51950_e66804, (locals.var_sp_s_a__blk1437_dn5 + locals.var_sp_s_c__blk1438_dn5), (locals.var_sp_s_a__blk1437_dn6 + locals.var_sp_s_c__blk1438_dn6), (locals.var_sp_s_a__blk1437_dn7 + locals.var_sp_s_c__blk1438_dn7), (locals.var_sp_s_a__blk1437_dn8 + locals.var_sp_s_c__blk1438_dn8), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign51960_e66815: f64 = (locals.var_nu * locals.var_nu);
            let assign51960_e66820: f64 = (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438);
            let assign51960_e66821: f64 = (0.5 * assign51960_e66820);
            let assign51960_e66824: f64 = (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454);
            let assign51960_e66825: f64 = (assign51960_e66821 - assign51960_e66824);
            let assign51960_e66826: f64 = (locals.var_sp_s_tau__blk1439 * assign51960_e66825);
            let assign51960_e66827: f64 = (assign51960_e66815 + assign51960_e66826);
            (locals.var_mutau, locals.var_mutau_dn5, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, ) = (assign51960_e66827, (((locals.var_nu_dn5 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn5)) + ((locals.var_sp_s_tau__blk1439_dn5 * assign51960_e66825) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn5 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn5))) - ((locals.var_sp_s_a__blk1437_dn5 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn5)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau__blk1439_dn6 * assign51960_e66825) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn6 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn6))) - ((locals.var_sp_s_a__blk1437_dn6 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau__blk1439_dn7 * assign51960_e66825) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn7 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn7))) - ((locals.var_sp_s_a__blk1437_dn7 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau__blk1439_dn8 * assign51960_e66825) + (locals.var_sp_s_tau__blk1439 * ((0.5 * ((locals.var_sp_s_c__blk1438_dn8 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn8))) - ((locals.var_sp_s_a__blk1437_dn8 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn8)))))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign51970_e66839: f64 = (locals.var_sp_s_a__blk1437 * locals.var_nu);
            let assign51970_e66841: f64 = (assign51970_e66839 * locals.var_sp_s_tau__blk1439);
            let assign51970_e66845: f64 = (locals.var_nu / locals.var_mutau);
            let assign51970_e66847: f64 = (assign51970_e66845 * locals.var_sp_s_tau__blk1439);
            let assign51970_e66849: f64 = (assign51970_e66847 * locals.var_sp_s_tau__blk1439);
            let assign51970_e66851: f64 = (assign51970_e66849 * locals.var_sp_s_c__blk1438);
            let assign51970_e66854: f64 = (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438);
            let assign51970_e66856: f64 = (assign51970_e66854 * 0.3333333333333333);
            let assign51970_e66859: f64 = (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454);
            let assign51970_e66860: f64 = (assign51970_e66856 - assign51970_e66859);
            let assign51970_e66861: f64 = (assign51970_e66851 * assign51970_e66860);
            let assign51970_e66862: f64 = (locals.var_mutau + assign51970_e66861);
            let assign51970_e66863: f64 = (assign51970_e66841 / assign51970_e66862);
            let assign51970_e66864: f64 = (locals.var_sp_s_eta__blk1436 + assign51970_e66863);
            (locals.var_sp_s_x0__blk1455, locals.var_sp_s_x0__blk1455_dn5, locals.var_sp_s_x0__blk1455_dn6, locals.var_sp_s_x0__blk1455_dn7, locals.var_sp_s_x0__blk1455_dn8, ) = (assign51970_e66864, (locals.var_sp_s_eta__blk1436_dn5 + (((((((locals.var_sp_s_a__blk1437_dn5 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn5)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66839 * locals.var_sp_s_tau__blk1439_dn5)) * assign51970_e66862) - (assign51970_e66841 * (locals.var_mutau_dn5 + (((((((((((locals.var_nu_dn5 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn5)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66845 * locals.var_sp_s_tau__blk1439_dn5)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66847 * locals.var_sp_s_tau__blk1439_dn5)) * locals.var_sp_s_c__blk1438) + (assign51970_e66849 * locals.var_sp_s_c__blk1438_dn5)) * assign51970_e66860) + (assign51970_e66851 * ((((locals.var_sp_s_c__blk1438_dn5 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn5)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn5 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn5)))))))) / (assign51970_e66862 * assign51970_e66862))), (locals.var_sp_s_eta__blk1436_dn6 + (((((((locals.var_sp_s_a__blk1437_dn6 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn6)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66839 * locals.var_sp_s_tau__blk1439_dn6)) * assign51970_e66862) - (assign51970_e66841 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66845 * locals.var_sp_s_tau__blk1439_dn6)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66847 * locals.var_sp_s_tau__blk1439_dn6)) * locals.var_sp_s_c__blk1438) + (assign51970_e66849 * locals.var_sp_s_c__blk1438_dn6)) * assign51970_e66860) + (assign51970_e66851 * ((((locals.var_sp_s_c__blk1438_dn6 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn6)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn6 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn6)))))))) / (assign51970_e66862 * assign51970_e66862))), (locals.var_sp_s_eta__blk1436_dn7 + (((((((locals.var_sp_s_a__blk1437_dn7 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn7)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66839 * locals.var_sp_s_tau__blk1439_dn7)) * assign51970_e66862) - (assign51970_e66841 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66845 * locals.var_sp_s_tau__blk1439_dn7)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66847 * locals.var_sp_s_tau__blk1439_dn7)) * locals.var_sp_s_c__blk1438) + (assign51970_e66849 * locals.var_sp_s_c__blk1438_dn7)) * assign51970_e66860) + (assign51970_e66851 * ((((locals.var_sp_s_c__blk1438_dn7 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn7)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn7 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn7)))))))) / (assign51970_e66862 * assign51970_e66862))), (locals.var_sp_s_eta__blk1436_dn8 + (((((((locals.var_sp_s_a__blk1437_dn8 * locals.var_nu) + (locals.var_sp_s_a__blk1437 * locals.var_nu_dn8)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66839 * locals.var_sp_s_tau__blk1439_dn8)) * assign51970_e66862) - (assign51970_e66841 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66845 * locals.var_sp_s_tau__blk1439_dn8)) * locals.var_sp_s_tau__blk1439) + (assign51970_e66847 * locals.var_sp_s_tau__blk1439_dn8)) * locals.var_sp_s_c__blk1438) + (assign51970_e66849 * locals.var_sp_s_c__blk1438_dn8)) * assign51970_e66860) + (assign51970_e66851 * ((((locals.var_sp_s_c__blk1438_dn8 * locals.var_sp_s_c__blk1438) + (locals.var_sp_s_c__blk1438 * locals.var_sp_s_c__blk1438_dn8)) * 0.3333333333333333) - ((locals.var_sp_s_a__blk1437_dn8 * locals.var_sp_s_b__blk1454) + (locals.var_sp_s_a__blk1437 * locals.var_sp_s_b__blk1454_dn8)))))))) / (assign51970_e66862 * assign51970_e66862))), );
        }

        let assign51980_e66869: f64 = if locals.var_sp_s_x0__blk1455 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1493 = assign51980_e66869;

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 != 0.0)) {
            let assign51990_e66879: f64 = (locals.var_sp_s_x0__blk1455).exp();
            (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8, ) = (assign51990_e66879, (assign51990_e66879 * locals.var_sp_s_x0__blk1455_dn5), (assign51990_e66879 * locals.var_sp_s_x0__blk1455_dn6), (assign51990_e66879 * locals.var_sp_s_x0__blk1455_dn7), (assign51990_e66879 * locals.var_sp_s_x0__blk1455_dn8), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 != 0.0)) {
            let assign52000_e66892: f64 = (1.0 / locals.var_sp_s_delta0__blk1441);
            (locals.var_sp_s_delta1__blk1442, locals.var_sp_s_delta1__blk1442_dn5, locals.var_sp_s_delta1__blk1442_dn6, locals.var_sp_s_delta1__blk1442_dn7, locals.var_sp_s_delta1__blk1442_dn8, ) = (assign52000_e66892, (-(locals.var_sp_s_delta0__blk1441_dn5 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn6 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn7 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), (-(locals.var_sp_s_delta0__blk1441_dn8 / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 != 0.0)) {
            let assign52010_e66905: f64 = (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441);
            (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8, ) = (assign52010_e66905, ((locals.var_delta_nd__blk1392_dn5 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn5)), ((locals.var_delta_nd__blk1392_dn6 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn6)), ((locals.var_delta_nd__blk1392_dn7 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn7)), ((locals.var_delta_nd__blk1392_dn8 * locals.var_sp_s_delta0__blk1441) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn8)), );
        }

        let assign52020_e66911: f64 = (locals.var_xn_d__blk1390 - 230.25850929940458);
        let assign52020_e66912: f64 = if locals.var_sp_s_x0__blk1455 > assign52020_e66911 { 1.0 } else { 0.0 };
        locals.var_guard1494 = assign52020_e66912;

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 == 0.0)) && (locals.var_guard1494 != 0.0)) {
            let assign52030_e66926: f64 = (locals.var_sp_s_x0__blk1455 - locals.var_xn_d__blk1390);
            let assign52030_e66927: f64 = (assign52030_e66926).exp();
            (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8, ) = (assign52030_e66927, (assign52030_e66927 * (locals.var_sp_s_x0__blk1455_dn5 - locals.var_xn_d__blk1390_dn5)), (assign52030_e66927 * (locals.var_sp_s_x0__blk1455_dn6 - locals.var_xn_d__blk1390_dn6)), (assign52030_e66927 * (locals.var_sp_s_x0__blk1455_dn7 - locals.var_xn_d__blk1390_dn7)), (assign52030_e66927 * (locals.var_sp_s_x0__blk1455_dn8 - locals.var_xn_d__blk1390_dn8)), );
        }

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 == 0.0)) && (locals.var_guard1494 != 0.0)) {
            let assign52040_e66943: f64 = (locals.var_delta_nd__blk1392 / locals.var_sp_s_delta0__blk1441);
            (locals.var_sp_s_delta1__blk1442, locals.var_sp_s_delta1__blk1442_dn5, locals.var_sp_s_delta1__blk1442_dn6, locals.var_sp_s_delta1__blk1442_dn7, locals.var_sp_s_delta1__blk1442_dn8, ) = (assign52040_e66943, (((locals.var_delta_nd__blk1392_dn5 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn5)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)), (((locals.var_delta_nd__blk1392_dn6 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn6)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)), (((locals.var_delta_nd__blk1392_dn7 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn7)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)), (((locals.var_delta_nd__blk1392_dn8 * locals.var_sp_s_delta0__blk1441) - (locals.var_delta_nd__blk1392 * locals.var_sp_s_delta0__blk1441_dn8)) / (locals.var_sp_s_delta0__blk1441 * locals.var_sp_s_delta0__blk1441)), );
        }

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 == 0.0)) && (locals.var_guard1494 == 0.0)) {
            let assign52050_e66962: f64 = (locals.var_xn_d__blk1390 - locals.var_sp_s_x0__blk1455);
            let assign52050_e66964: f64 = (assign52050_e66962 - 230.25850929940458);
            let assign52050_e66969: f64 = (locals.var_xn_d__blk1390 - locals.var_sp_s_x0__blk1455);
            let assign52050_e66971: f64 = (assign52050_e66969 - 230.25850929940458);
            let assign52050_e66975: f64 = (locals.var_xn_d__blk1390 - locals.var_sp_s_x0__blk1455);
            let assign52050_e66977: f64 = (assign52050_e66975 - 230.25850929940458);
            let assign52050_e66979: f64 = (assign52050_e66977 * 0.3333333333333333);
            let assign52050_e66980: f64 = (1.0 + assign52050_e66979);
            let assign52050_e66981: f64 = (assign52050_e66971 * assign52050_e66980);
            let assign52050_e66982: f64 = (0.5 * assign52050_e66981);
            let assign52050_e66983: f64 = (1.0 + assign52050_e66982);
            let assign52050_e66984: f64 = (assign52050_e66964 * assign52050_e66983);
            let assign52050_e66985: f64 = (1.0 + assign52050_e66984);
            let assign52050_e66986: f64 = (1e-100 / assign52050_e66985);
            (locals.var_sp_s_delta0__blk1441, locals.var_sp_s_delta0__blk1441_dn5, locals.var_sp_s_delta0__blk1441_dn6, locals.var_sp_s_delta0__blk1441_dn7, locals.var_sp_s_delta0__blk1441_dn8, ) = (assign52050_e66986, (-((1e-100 * (((locals.var_xn_d__blk1390_dn5 - locals.var_sp_s_x0__blk1455_dn5) * assign52050_e66983) + (assign52050_e66964 * (0.5 * (((locals.var_xn_d__blk1390_dn5 - locals.var_sp_s_x0__blk1455_dn5) * assign52050_e66980) + (assign52050_e66971 * ((locals.var_xn_d__blk1390_dn5 - locals.var_sp_s_x0__blk1455_dn5) * 0.3333333333333333))))))) / (assign52050_e66985 * assign52050_e66985))), (-((1e-100 * (((locals.var_xn_d__blk1390_dn6 - locals.var_sp_s_x0__blk1455_dn6) * assign52050_e66983) + (assign52050_e66964 * (0.5 * (((locals.var_xn_d__blk1390_dn6 - locals.var_sp_s_x0__blk1455_dn6) * assign52050_e66980) + (assign52050_e66971 * ((locals.var_xn_d__blk1390_dn6 - locals.var_sp_s_x0__blk1455_dn6) * 0.3333333333333333))))))) / (assign52050_e66985 * assign52050_e66985))), (-((1e-100 * (((locals.var_xn_d__blk1390_dn7 - locals.var_sp_s_x0__blk1455_dn7) * assign52050_e66983) + (assign52050_e66964 * (0.5 * (((locals.var_xn_d__blk1390_dn7 - locals.var_sp_s_x0__blk1455_dn7) * assign52050_e66980) + (assign52050_e66971 * ((locals.var_xn_d__blk1390_dn7 - locals.var_sp_s_x0__blk1455_dn7) * 0.3333333333333333))))))) / (assign52050_e66985 * assign52050_e66985))), (-((1e-100 * (((locals.var_xn_d__blk1390_dn8 - locals.var_sp_s_x0__blk1455_dn8) * assign52050_e66983) + (assign52050_e66964 * (0.5 * (((locals.var_xn_d__blk1390_dn8 - locals.var_sp_s_x0__blk1455_dn8) * assign52050_e66980) + (assign52050_e66971 * ((locals.var_xn_d__blk1390_dn8 - locals.var_sp_s_x0__blk1455_dn8) * 0.3333333333333333))))))) / (assign52050_e66985 * assign52050_e66985))), );
        }

        if (((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) && (locals.var_guard1493 == 0.0)) && (locals.var_guard1494 == 0.0)) {
            let assign52060_e67005: f64 = (locals.var_sp_s_x0__blk1455 - 230.25850929940458);
            let assign52060_e67010: f64 = (locals.var_sp_s_x0__blk1455 - 230.25850929940458);
            let assign52060_e67014: f64 = (locals.var_sp_s_x0__blk1455 - 230.25850929940458);
            let assign52060_e67016: f64 = (assign52060_e67014 * 0.3333333333333333);
            let assign52060_e67017: f64 = (1.0 + assign52060_e67016);
            let assign52060_e67018: f64 = (assign52060_e67010 * assign52060_e67017);
            let assign52060_e67019: f64 = (0.5 * assign52060_e67018);
            let assign52060_e67020: f64 = (1.0 + assign52060_e67019);
            let assign52060_e67021: f64 = (assign52060_e67005 * assign52060_e67020);
            let assign52060_e67022: f64 = (1.0 + assign52060_e67021);
            let assign52060_e67023: f64 = (1e-100 / assign52060_e67022);
            (locals.var_sp_s_delta1__blk1442, locals.var_sp_s_delta1__blk1442_dn5, locals.var_sp_s_delta1__blk1442_dn6, locals.var_sp_s_delta1__blk1442_dn7, locals.var_sp_s_delta1__blk1442_dn8, ) = (assign52060_e67023, (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn5 * assign52060_e67020) + (assign52060_e67005 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn5 * assign52060_e67017) + (assign52060_e67010 * (locals.var_sp_s_x0__blk1455_dn5 * 0.3333333333333333))))))) / (assign52060_e67022 * assign52060_e67022))), (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn6 * assign52060_e67020) + (assign52060_e67005 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn6 * assign52060_e67017) + (assign52060_e67010 * (locals.var_sp_s_x0__blk1455_dn6 * 0.3333333333333333))))))) / (assign52060_e67022 * assign52060_e67022))), (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn7 * assign52060_e67020) + (assign52060_e67005 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn7 * assign52060_e67017) + (assign52060_e67010 * (locals.var_sp_s_x0__blk1455_dn7 * 0.3333333333333333))))))) / (assign52060_e67022 * assign52060_e67022))), (-((1e-100 * ((locals.var_sp_s_x0__blk1455_dn8 * assign52060_e67020) + (assign52060_e67005 * (0.5 * ((locals.var_sp_s_x0__blk1455_dn8 * assign52060_e67017) + (assign52060_e67010 * (locals.var_sp_s_x0__blk1455_dn8 * 0.3333333333333333))))))) / (assign52060_e67022 * assign52060_e67022))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign52070_e67036: f64 = (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455);
            let assign52070_e67037: f64 = (2.0 + assign52070_e67036);
            let assign52070_e67038: f64 = (1.0 / assign52070_e67037);
            (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8, ) = (assign52070_e67038, (-(((locals.var_sp_s_x0__blk1455_dn5 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn5)) / (assign52070_e67037 * assign52070_e67037))), (-(((locals.var_sp_s_x0__blk1455_dn6 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn6)) / (assign52070_e67037 * assign52070_e67037))), (-(((locals.var_sp_s_x0__blk1455_dn7 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn7)) / (assign52070_e67037 * assign52070_e67037))), (-(((locals.var_sp_s_x0__blk1455_dn8 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn8)) / (assign52070_e67037 * assign52070_e67037))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign52080_e67049: f64 = (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455);
            let assign52080_e67051: f64 = (assign52080_e67049 * locals.var_sp_s_temp__blk1431);
            (locals.var_sp_s_xi0__blk1443, locals.var_sp_s_xi0__blk1443_dn5, locals.var_sp_s_xi0__blk1443_dn6, locals.var_sp_s_xi0__blk1443_dn7, locals.var_sp_s_xi0__blk1443_dn8, ) = (assign52080_e67051, ((((locals.var_sp_s_x0__blk1455_dn5 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn5)) * locals.var_sp_s_temp__blk1431) + (assign52080_e67049 * locals.var_sp_s_temp__blk1431_dn5)), ((((locals.var_sp_s_x0__blk1455_dn6 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn6)) * locals.var_sp_s_temp__blk1431) + (assign52080_e67049 * locals.var_sp_s_temp__blk1431_dn6)), ((((locals.var_sp_s_x0__blk1455_dn7 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn7)) * locals.var_sp_s_temp__blk1431) + (assign52080_e67049 * locals.var_sp_s_temp__blk1431_dn7)), ((((locals.var_sp_s_x0__blk1455_dn8 * locals.var_sp_s_x0__blk1455) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_x0__blk1455_dn8)) * locals.var_sp_s_temp__blk1431) + (assign52080_e67049 * locals.var_sp_s_temp__blk1431_dn8)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign52090_e67063: f64 = (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431);
            let assign52090_e67065: f64 = (assign52090_e67063 * locals.var_sp_s_temp__blk1431);
            let assign52090_e67066: f64 = (4.0 * assign52090_e67065);
            (locals.var_sp_s_xi1__blk1444, locals.var_sp_s_xi1__blk1444_dn5, locals.var_sp_s_xi1__blk1444_dn6, locals.var_sp_s_xi1__blk1444_dn7, locals.var_sp_s_xi1__blk1444_dn8, ) = (assign52090_e67066, (4.0 * ((((locals.var_sp_s_x0__blk1455_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn5)) * locals.var_sp_s_temp__blk1431) + (assign52090_e67063 * locals.var_sp_s_temp__blk1431_dn5))), (4.0 * ((((locals.var_sp_s_x0__blk1455_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn6)) * locals.var_sp_s_temp__blk1431) + (assign52090_e67063 * locals.var_sp_s_temp__blk1431_dn6))), (4.0 * ((((locals.var_sp_s_x0__blk1455_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn7)) * locals.var_sp_s_temp__blk1431) + (assign52090_e67063 * locals.var_sp_s_temp__blk1431_dn7))), (4.0 * ((((locals.var_sp_s_x0__blk1455_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_x0__blk1455 * locals.var_sp_s_temp__blk1431_dn8)) * locals.var_sp_s_temp__blk1431) + (assign52090_e67063 * locals.var_sp_s_temp__blk1431_dn8))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign52100_e67077: f64 = (8.0 * locals.var_sp_s_temp__blk1431);
            let assign52100_e67080: f64 = (12.0 * locals.var_sp_s_xi0__blk1443);
            let assign52100_e67081: f64 = (assign52100_e67077 - assign52100_e67080);
            let assign52100_e67083: f64 = (assign52100_e67081 * locals.var_sp_s_temp__blk1431);
            let assign52100_e67085: f64 = (assign52100_e67083 * locals.var_sp_s_temp__blk1431);
            (locals.var_sp_s_xi2__blk1445, locals.var_sp_s_xi2__blk1445_dn5, locals.var_sp_s_xi2__blk1445_dn6, locals.var_sp_s_xi2__blk1445_dn7, locals.var_sp_s_xi2__blk1445_dn8, ) = (assign52100_e67085, ((((((8.0 * locals.var_sp_s_temp__blk1431_dn5) - (12.0 * locals.var_sp_s_xi0__blk1443_dn5)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67081 * locals.var_sp_s_temp__blk1431_dn5)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67083 * locals.var_sp_s_temp__blk1431_dn5)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn6) - (12.0 * locals.var_sp_s_xi0__blk1443_dn6)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67081 * locals.var_sp_s_temp__blk1431_dn6)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67083 * locals.var_sp_s_temp__blk1431_dn6)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn7) - (12.0 * locals.var_sp_s_xi0__blk1443_dn7)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67081 * locals.var_sp_s_temp__blk1431_dn7)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67083 * locals.var_sp_s_temp__blk1431_dn7)), ((((((8.0 * locals.var_sp_s_temp__blk1431_dn8) - (12.0 * locals.var_sp_s_xi0__blk1443_dn8)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67081 * locals.var_sp_s_temp__blk1431_dn8)) * locals.var_sp_s_temp__blk1431) + (assign52100_e67083 * locals.var_sp_s_temp__blk1431_dn8)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign52110_e67096: f64 = (locals.var_xg__blk1326 - locals.var_sp_s_x0__blk1455);
            (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8, ) = (assign52110_e67096, (locals.var_xg__blk1326_dn5 - locals.var_sp_s_x0__blk1455_dn5), (locals.var_xg__blk1326_dn6 - locals.var_sp_s_x0__blk1455_dn6), (locals.var_xg__blk1326_dn7 - locals.var_sp_s_x0__blk1455_dn7), (locals.var_xg__blk1326_dn8 - locals.var_sp_s_x0__blk1455_dn8), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign52120_e67107: f64 = (2.0 * locals.var_sp_s_temp__blk1431);
            let assign52120_e67111: f64 = (1.0 - locals.var_sp_s_delta1__blk1442);
            let assign52120_e67113: f64 = (assign52120_e67111 + locals.var_sp_s_delta0__blk1441);
            let assign52120_e67117: f64 = (1.0 + locals.var_sp_s_xi1__blk1444);
            let assign52120_e67118: f64 = (locals.var_delta_nd__blk1392 * assign52120_e67117);
            let assign52120_e67119: f64 = (assign52120_e67113 - assign52120_e67118);
            let assign52120_e67120: f64 = (locals.var_gf2__blk1308 * assign52120_e67119);
            let assign52120_e67121: f64 = (assign52120_e67107 + assign52120_e67120);
            (locals.var_sp_s_pc__blk1446, locals.var_sp_s_pc__blk1446_dn5, locals.var_sp_s_pc__blk1446_dn6, locals.var_sp_s_pc__blk1446_dn7, locals.var_sp_s_pc__blk1446_dn8, ) = (assign52120_e67121, ((2.0 * locals.var_sp_s_temp__blk1431_dn5) + ((locals.var_gf2__blk1308_dn5 * assign52120_e67119) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn5) + locals.var_sp_s_delta0__blk1441_dn5) - ((locals.var_delta_nd__blk1392_dn5 * assign52120_e67117) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn5)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn6) + ((locals.var_gf2__blk1308_dn6 * assign52120_e67119) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn6) + locals.var_sp_s_delta0__blk1441_dn6) - ((locals.var_delta_nd__blk1392_dn6 * assign52120_e67117) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn6)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn7) + ((locals.var_gf2__blk1308_dn7 * assign52120_e67119) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn7) + locals.var_sp_s_delta0__blk1441_dn7) - ((locals.var_delta_nd__blk1392_dn7 * assign52120_e67117) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn7)))))), ((2.0 * locals.var_sp_s_temp__blk1431_dn8) + ((locals.var_gf2__blk1308_dn8 * assign52120_e67119) + (locals.var_gf2__blk1308 * (((-locals.var_sp_s_delta1__blk1442_dn8) + locals.var_sp_s_delta0__blk1441_dn8) - ((locals.var_delta_nd__blk1392_dn8 * assign52120_e67117) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi1__blk1444_dn8)))))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign52130_e67132: f64 = (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431);
            let assign52130_e67136: f64 = (locals.var_sp_s_delta1__blk1442 + locals.var_sp_s_x0__blk1455);
            let assign52130_e67138: f64 = (assign52130_e67136 - 1.0);
            let assign52130_e67140: f64 = (assign52130_e67138 + locals.var_sp_s_delta0__blk1441);
            let assign52130_e67144: f64 = (locals.var_sp_s_x0__blk1455 + 1.0);
            let assign52130_e67146: f64 = (assign52130_e67144 + locals.var_sp_s_xi0__blk1443);
            let assign52130_e67147: f64 = (locals.var_delta_nd__blk1392 * assign52130_e67146);
            let assign52130_e67148: f64 = (assign52130_e67140 - assign52130_e67147);
            let assign52130_e67149: f64 = (locals.var_gf2__blk1308 * assign52130_e67148);
            let assign52130_e67150: f64 = (assign52130_e67132 - assign52130_e67149);
            (locals.var_sp_s_qc__blk1447, locals.var_sp_s_qc__blk1447_dn5, locals.var_sp_s_qc__blk1447_dn6, locals.var_sp_s_qc__blk1447_dn7, locals.var_sp_s_qc__blk1447_dn8, ) = (assign52130_e67150, (((locals.var_sp_s_temp__blk1431_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn5)) - ((locals.var_gf2__blk1308_dn5 * assign52130_e67148) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn5 + locals.var_sp_s_x0__blk1455_dn5) + locals.var_sp_s_delta0__blk1441_dn5) - ((locals.var_delta_nd__blk1392_dn5 * assign52130_e67146) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_x0__blk1455_dn5 + locals.var_sp_s_xi0__blk1443_dn5))))))), (((locals.var_sp_s_temp__blk1431_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn6)) - ((locals.var_gf2__blk1308_dn6 * assign52130_e67148) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn6 + locals.var_sp_s_x0__blk1455_dn6) + locals.var_sp_s_delta0__blk1441_dn6) - ((locals.var_delta_nd__blk1392_dn6 * assign52130_e67146) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_x0__blk1455_dn6 + locals.var_sp_s_xi0__blk1443_dn6))))))), (((locals.var_sp_s_temp__blk1431_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn7)) - ((locals.var_gf2__blk1308_dn7 * assign52130_e67148) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn7 + locals.var_sp_s_x0__blk1455_dn7) + locals.var_sp_s_delta0__blk1441_dn7) - ((locals.var_delta_nd__blk1392_dn7 * assign52130_e67146) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_x0__blk1455_dn7 + locals.var_sp_s_xi0__blk1443_dn7))))))), (((locals.var_sp_s_temp__blk1431_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_temp__blk1431 * locals.var_sp_s_temp__blk1431_dn8)) - ((locals.var_gf2__blk1308_dn8 * assign52130_e67148) + (locals.var_gf2__blk1308 * (((locals.var_sp_s_delta1__blk1442_dn8 + locals.var_sp_s_x0__blk1455_dn8) + locals.var_sp_s_delta0__blk1441_dn8) - ((locals.var_delta_nd__blk1392_dn8 * assign52130_e67146) + (locals.var_delta_nd__blk1392 * (locals.var_sp_s_x0__blk1455_dn8 + locals.var_sp_s_xi0__blk1443_dn8))))))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign52140_e67163: f64 = (locals.var_sp_s_delta1__blk1442 + locals.var_sp_s_delta0__blk1441);
            let assign52140_e67166: f64 = (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445);
            let assign52140_e67167: f64 = (assign52140_e67163 - assign52140_e67166);
            let assign52140_e67168: f64 = (locals.var_gf2__blk1308 * assign52140_e67167);
            let assign52140_e67169: f64 = (2.0 - assign52140_e67168);
            (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8, ) = (assign52140_e67169, (-((locals.var_gf2__blk1308_dn5 * assign52140_e67167) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn5 + locals.var_sp_s_delta0__blk1441_dn5) - ((locals.var_delta_nd__blk1392_dn5 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn5)))))), (-((locals.var_gf2__blk1308_dn6 * assign52140_e67167) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn6 + locals.var_sp_s_delta0__blk1441_dn6) - ((locals.var_delta_nd__blk1392_dn6 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn6)))))), (-((locals.var_gf2__blk1308_dn7 * assign52140_e67167) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn7 + locals.var_sp_s_delta0__blk1441_dn7) - ((locals.var_delta_nd__blk1392_dn7 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn7)))))), (-((locals.var_gf2__blk1308_dn8 * assign52140_e67167) + (locals.var_gf2__blk1308 * ((locals.var_sp_s_delta1__blk1442_dn8 + locals.var_sp_s_delta0__blk1441_dn8) - ((locals.var_delta_nd__blk1392_dn8 * locals.var_sp_s_xi2__blk1445) + (locals.var_delta_nd__blk1392 * locals.var_sp_s_xi2__blk1445_dn8)))))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign52150_e67180: f64 = (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446);
            let assign52150_e67184: f64 = (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431);
            let assign52150_e67185: f64 = (2.0 * assign52150_e67184);
            let assign52150_e67186: f64 = (assign52150_e67180 - assign52150_e67185);
            (locals.var_sp_s_temp__blk1431, locals.var_sp_s_temp__blk1431_dn5, locals.var_sp_s_temp__blk1431_dn6, locals.var_sp_s_temp__blk1431_dn7, locals.var_sp_s_temp__blk1431_dn8, ) = (assign52150_e67186, (((locals.var_sp_s_pc__blk1446_dn5 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn5)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn5 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn5)))), (((locals.var_sp_s_pc__blk1446_dn6 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn6)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn6 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn6)))), (((locals.var_sp_s_pc__blk1446_dn7 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn7)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn7 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn7)))), (((locals.var_sp_s_pc__blk1446_dn8 * locals.var_sp_s_pc__blk1446) + (locals.var_sp_s_pc__blk1446 * locals.var_sp_s_pc__blk1446_dn8)) - (2.0 * ((locals.var_sp_s_qc__blk1447_dn8 * locals.var_sp_s_temp__blk1431) + (locals.var_sp_s_qc__blk1447 * locals.var_sp_s_temp__blk1431_dn8)))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1492 == 0.0)) {
            let assign52160_e67200: f64 = (locals.var_sp_s_temp__blk1431).sqrt();
            let assign52160_e67201: f64 = (locals.var_sp_s_pc__blk1446 + assign52160_e67200);
            let assign52160_e67202: f64 = (locals.var_sp_s_qc__blk1447 / assign52160_e67201);
            let assign52160_e67203: f64 = (2.0 * assign52160_e67202);
            let assign52160_e67204: f64 = (locals.var_sp_s_x0__blk1455 + assign52160_e67203);
            (locals.var_x_d__blk1393, locals.var_x_d__blk1393_dn5, locals.var_x_d__blk1393_dn6, locals.var_x_d__blk1393_dn7, locals.var_x_d__blk1393_dn8, ) = (assign52160_e67204, (locals.var_sp_s_x0__blk1455_dn5 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn5 * assign52160_e67201) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn5 + (locals.var_sp_s_temp__blk1431_dn5 / (2.0 * assign52160_e67200))))) / (assign52160_e67201 * assign52160_e67201)))), (locals.var_sp_s_x0__blk1455_dn6 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn6 * assign52160_e67201) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn6 + (locals.var_sp_s_temp__blk1431_dn6 / (2.0 * assign52160_e67200))))) / (assign52160_e67201 * assign52160_e67201)))), (locals.var_sp_s_x0__blk1455_dn7 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn7 * assign52160_e67201) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn7 + (locals.var_sp_s_temp__blk1431_dn7 / (2.0 * assign52160_e67200))))) / (assign52160_e67201 * assign52160_e67201)))), (locals.var_sp_s_x0__blk1455_dn8 + (2.0 * (((locals.var_sp_s_qc__blk1447_dn8 * assign52160_e67201) - (locals.var_sp_s_qc__blk1447 * (locals.var_sp_s_pc__blk1446_dn8 + (locals.var_sp_s_temp__blk1431_dn8 / (2.0 * assign52160_e67200))))) / (assign52160_e67201 * assign52160_e67201)))), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign52170_e67212: f64 = (locals.var_x_d__blk1393 - locals.var_x_s__blk1346);
            (locals.var_x_ds__blk1394, locals.var_x_ds__blk1394_dn5, locals.var_x_ds__blk1394_dn6, locals.var_x_ds__blk1394_dn7, locals.var_x_ds__blk1394_dn8, ) = (assign52170_e67212, (locals.var_x_d__blk1393_dn5 - locals.var_x_s__blk1346_dn5), (locals.var_x_d__blk1393_dn6 - locals.var_x_s__blk1346_dn6), (locals.var_x_d__blk1393_dn7 - locals.var_x_s__blk1346_dn7), (locals.var_x_d__blk1393_dn8 - locals.var_x_s__blk1346_dn8), );
        }

        let assign52180_e67217: f64 = if locals.var_x_ds__blk1394 < 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1495 = assign52180_e67217;

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1495 != 0.0)) {
            let assign52190_e67226: f64 = (locals.var_xg__blk1326 - locals.var_x_s__blk1346);
            let assign52190_e67227: f64 = (2.0 * assign52190_e67226);
            let assign52190_e67231: f64 = (1.0 - locals.var_es__blk1352);
            let assign52190_e67234: f64 = (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391);
            let assign52190_e67235: f64 = (assign52190_e67231 + assign52190_e67234);
            let assign52190_e67239: f64 = (1.0 + locals.var_xi1s__blk1349);
            let assign52190_e67240: f64 = (locals.var_delta_nd__blk1392 * assign52190_e67239);
            let assign52190_e67241: f64 = (assign52190_e67235 - assign52190_e67240);
            let assign52190_e67242: f64 = (locals.var_gf2__blk1308 * assign52190_e67241);
            let assign52190_e67243: f64 = (assign52190_e67227 + assign52190_e67242);
            (locals.var_pc__blk1395, locals.var_pc__blk1395_dn5, locals.var_pc__blk1395_dn6, locals.var_pc__blk1395_dn7, locals.var_pc__blk1395_dn8, ) = (assign52190_e67243, ((2.0 * (locals.var_xg__blk1326_dn5 - locals.var_x_s__blk1346_dn5)) + ((locals.var_gf2__blk1308_dn5 * assign52190_e67241) + (locals.var_gf2__blk1308 * (((-locals.var_es__blk1352_dn5) + ((locals.var_delta_1s__blk1351_dn5 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn5))) - ((locals.var_delta_nd__blk1392_dn5 * assign52190_e67239) + (locals.var_delta_nd__blk1392 * locals.var_xi1s__blk1349_dn5)))))), ((2.0 * (locals.var_xg__blk1326_dn6 - locals.var_x_s__blk1346_dn6)) + ((locals.var_gf2__blk1308_dn6 * assign52190_e67241) + (locals.var_gf2__blk1308 * (((-locals.var_es__blk1352_dn6) + ((locals.var_delta_1s__blk1351_dn6 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn6))) - ((locals.var_delta_nd__blk1392_dn6 * assign52190_e67239) + (locals.var_delta_nd__blk1392 * locals.var_xi1s__blk1349_dn6)))))), ((2.0 * (locals.var_xg__blk1326_dn7 - locals.var_x_s__blk1346_dn7)) + ((locals.var_gf2__blk1308_dn7 * assign52190_e67241) + (locals.var_gf2__blk1308 * (((-locals.var_es__blk1352_dn7) + ((locals.var_delta_1s__blk1351_dn7 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn7))) - ((locals.var_delta_nd__blk1392_dn7 * assign52190_e67239) + (locals.var_delta_nd__blk1392 * locals.var_xi1s__blk1349_dn7)))))), ((2.0 * (locals.var_xg__blk1326_dn8 - locals.var_x_s__blk1346_dn8)) + ((locals.var_gf2__blk1308_dn8 * assign52190_e67241) + (locals.var_gf2__blk1308 * (((-locals.var_es__blk1352_dn8) + ((locals.var_delta_1s__blk1351_dn8 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn8))) - ((locals.var_delta_nd__blk1392_dn8 * assign52190_e67239) + (locals.var_delta_nd__blk1392 * locals.var_xi1s__blk1349_dn8)))))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1495 != 0.0)) {
            let assign52200_e67254: f64 = (1.0 - locals.var_k_ds__blk1391);
            let assign52200_e67255: f64 = (locals.var_gf2__blk1308 * assign52200_e67254);
            let assign52200_e67257: f64 = (assign52200_e67255 * locals.var_ds__blk1353);
            (locals.var_qc__blk1396, locals.var_qc__blk1396_dn5, locals.var_qc__blk1396_dn6, locals.var_qc__blk1396_dn7, locals.var_qc__blk1396_dn8, ) = (assign52200_e67257, ((((locals.var_gf2__blk1308_dn5 * assign52200_e67254) + (locals.var_gf2__blk1308 * (-locals.var_k_ds__blk1391_dn5))) * locals.var_ds__blk1353) + (assign52200_e67255 * locals.var_ds__blk1353_dn5)), ((((locals.var_gf2__blk1308_dn6 * assign52200_e67254) + (locals.var_gf2__blk1308 * (-locals.var_k_ds__blk1391_dn6))) * locals.var_ds__blk1353) + (assign52200_e67255 * locals.var_ds__blk1353_dn6)), ((((locals.var_gf2__blk1308_dn7 * assign52200_e67254) + (locals.var_gf2__blk1308 * (-locals.var_k_ds__blk1391_dn7))) * locals.var_ds__blk1353) + (assign52200_e67255 * locals.var_ds__blk1353_dn7)), ((((locals.var_gf2__blk1308_dn8 * assign52200_e67254) + (locals.var_gf2__blk1308 * (-locals.var_k_ds__blk1391_dn8))) * locals.var_ds__blk1353) + (assign52200_e67255 * locals.var_ds__blk1353_dn8)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1495 != 0.0)) {
            let assign52210_e67270: f64 = (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391);
            let assign52210_e67271: f64 = (locals.var_es__blk1352 + assign52210_e67270);
            let assign52210_e67274: f64 = (locals.var_delta_nd__blk1392 * locals.var_xi2s__blk1350);
            let assign52210_e67275: f64 = (assign52210_e67271 - assign52210_e67274);
            let assign52210_e67276: f64 = (locals.var_gf2__blk1308 * assign52210_e67275);
            let assign52210_e67277: f64 = (2.0 - assign52210_e67276);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign52210_e67277, (-((locals.var_gf2__blk1308_dn5 * assign52210_e67275) + (locals.var_gf2__blk1308 * ((locals.var_es__blk1352_dn5 + ((locals.var_delta_1s__blk1351_dn5 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn5))) - ((locals.var_delta_nd__blk1392_dn5 * locals.var_xi2s__blk1350) + (locals.var_delta_nd__blk1392 * locals.var_xi2s__blk1350_dn5)))))), (-((locals.var_gf2__blk1308_dn6 * assign52210_e67275) + (locals.var_gf2__blk1308 * ((locals.var_es__blk1352_dn6 + ((locals.var_delta_1s__blk1351_dn6 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn6))) - ((locals.var_delta_nd__blk1392_dn6 * locals.var_xi2s__blk1350) + (locals.var_delta_nd__blk1392 * locals.var_xi2s__blk1350_dn6)))))), (-((locals.var_gf2__blk1308_dn7 * assign52210_e67275) + (locals.var_gf2__blk1308 * ((locals.var_es__blk1352_dn7 + ((locals.var_delta_1s__blk1351_dn7 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn7))) - ((locals.var_delta_nd__blk1392_dn7 * locals.var_xi2s__blk1350) + (locals.var_delta_nd__blk1392 * locals.var_xi2s__blk1350_dn7)))))), (-((locals.var_gf2__blk1308_dn8 * assign52210_e67275) + (locals.var_gf2__blk1308 * ((locals.var_es__blk1352_dn8 + ((locals.var_delta_1s__blk1351_dn8 * locals.var_k_ds__blk1391) + (locals.var_delta_1s__blk1351 * locals.var_k_ds__blk1391_dn8))) - ((locals.var_delta_nd__blk1392_dn8 * locals.var_xi2s__blk1350) + (locals.var_delta_nd__blk1392 * locals.var_xi2s__blk1350_dn8)))))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1495 != 0.0)) {
            let assign52220_e67287: f64 = (locals.var_pc__blk1395 * locals.var_pc__blk1395);
            let assign52220_e67291: f64 = (locals.var_temp__blk936 * locals.var_qc__blk1396);
            let assign52220_e67292: f64 = (2.0 * assign52220_e67291);
            let assign52220_e67293: f64 = (assign52220_e67287 - assign52220_e67292);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign52220_e67293, (((locals.var_pc__blk1395_dn5 * locals.var_pc__blk1395) + (locals.var_pc__blk1395 * locals.var_pc__blk1395_dn5)) - (2.0 * ((locals.var_temp__blk936_dn5 * locals.var_qc__blk1396) + (locals.var_temp__blk936 * locals.var_qc__blk1396_dn5)))), (((locals.var_pc__blk1395_dn6 * locals.var_pc__blk1395) + (locals.var_pc__blk1395 * locals.var_pc__blk1395_dn6)) - (2.0 * ((locals.var_temp__blk936_dn6 * locals.var_qc__blk1396) + (locals.var_temp__blk936 * locals.var_qc__blk1396_dn6)))), (((locals.var_pc__blk1395_dn7 * locals.var_pc__blk1395) + (locals.var_pc__blk1395 * locals.var_pc__blk1395_dn7)) - (2.0 * ((locals.var_temp__blk936_dn7 * locals.var_qc__blk1396) + (locals.var_temp__blk936 * locals.var_qc__blk1396_dn7)))), (((locals.var_pc__blk1395_dn8 * locals.var_pc__blk1395) + (locals.var_pc__blk1395 * locals.var_pc__blk1395_dn8)) - (2.0 * ((locals.var_temp__blk936_dn8 * locals.var_qc__blk1396) + (locals.var_temp__blk936 * locals.var_qc__blk1396_dn8)))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1495 != 0.0)) {
            let assign52230_e67305: f64 = (locals.var_temp__blk936).sqrt();
            let assign52230_e67306: f64 = (locals.var_pc__blk1395 + assign52230_e67305);
            let assign52230_e67307: f64 = (locals.var_qc__blk1396 / assign52230_e67306);
            let assign52230_e67308: f64 = (2.0 * assign52230_e67307);
            (locals.var_x_ds__blk1394, locals.var_x_ds__blk1394_dn5, locals.var_x_ds__blk1394_dn6, locals.var_x_ds__blk1394_dn7, locals.var_x_ds__blk1394_dn8, ) = (assign52230_e67308, (2.0 * (((locals.var_qc__blk1396_dn5 * assign52230_e67306) - (locals.var_qc__blk1396 * (locals.var_pc__blk1395_dn5 + (locals.var_temp__blk936_dn5 / (2.0 * assign52230_e67305))))) / (assign52230_e67306 * assign52230_e67306))), (2.0 * (((locals.var_qc__blk1396_dn6 * assign52230_e67306) - (locals.var_qc__blk1396 * (locals.var_pc__blk1395_dn6 + (locals.var_temp__blk936_dn6 / (2.0 * assign52230_e67305))))) / (assign52230_e67306 * assign52230_e67306))), (2.0 * (((locals.var_qc__blk1396_dn7 * assign52230_e67306) - (locals.var_qc__blk1396 * (locals.var_pc__blk1395_dn7 + (locals.var_temp__blk936_dn7 / (2.0 * assign52230_e67305))))) / (assign52230_e67306 * assign52230_e67306))), (2.0 * (((locals.var_qc__blk1396_dn8 * assign52230_e67306) - (locals.var_qc__blk1396 * (locals.var_pc__blk1395_dn8 + (locals.var_temp__blk936_dn8 / (2.0 * assign52230_e67305))))) / (assign52230_e67306 * assign52230_e67306))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1495 != 0.0)) {
            let assign52240_e67318: f64 = (locals.var_x_s__blk1346 + locals.var_x_ds__blk1394);
            (locals.var_x_d__blk1393, locals.var_x_d__blk1393_dn5, locals.var_x_d__blk1393_dn6, locals.var_x_d__blk1393_dn7, locals.var_x_d__blk1393_dn8, ) = (assign52240_e67318, (locals.var_x_s__blk1346_dn5 + locals.var_x_ds__blk1394_dn5), (locals.var_x_s__blk1346_dn6 + locals.var_x_ds__blk1394_dn6), (locals.var_x_s__blk1346_dn7 + locals.var_x_ds__blk1394_dn7), (locals.var_x_s__blk1346_dn8 + locals.var_x_ds__blk1394_dn8), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign52250_e67326: f64 = (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322);
            (locals.var_dps__blk1397, locals.var_dps__blk1397_dn5, locals.var_dps__blk1397_dn6, locals.var_dps__blk1397_dn7, locals.var_dps__blk1397_dn8, ) = (assign52250_e67326, ((locals.var_x_ds__blk1394_dn5 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn5)), ((locals.var_x_ds__blk1394_dn6 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn6)), ((locals.var_x_ds__blk1394_dn7 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn7)), ((locals.var_x_ds__blk1394_dn8 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn8)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign52260_e67334: f64 = (locals.var_x_d__blk1393 * locals.var_x_d__blk1393);
            let assign52260_e67338: f64 = (locals.var_x_d__blk1393 * locals.var_x_d__blk1393);
            let assign52260_e67339: f64 = (2.0 + assign52260_e67338);
            let assign52260_e67340: f64 = (assign52260_e67334 / assign52260_e67339);
            (locals.var_xi0d__blk1398, locals.var_xi0d__blk1398_dn5, locals.var_xi0d__blk1398_dn6, locals.var_xi0d__blk1398_dn7, locals.var_xi0d__blk1398_dn8, ) = (assign52260_e67340, (((((locals.var_x_d__blk1393_dn5 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn5)) * assign52260_e67339) - (assign52260_e67334 * ((locals.var_x_d__blk1393_dn5 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn5)))) / (assign52260_e67339 * assign52260_e67339)), (((((locals.var_x_d__blk1393_dn6 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn6)) * assign52260_e67339) - (assign52260_e67334 * ((locals.var_x_d__blk1393_dn6 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn6)))) / (assign52260_e67339 * assign52260_e67339)), (((((locals.var_x_d__blk1393_dn7 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn7)) * assign52260_e67339) - (assign52260_e67334 * ((locals.var_x_d__blk1393_dn7 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn7)))) / (assign52260_e67339 * assign52260_e67339)), (((((locals.var_x_d__blk1393_dn8 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn8)) * assign52260_e67339) - (assign52260_e67334 * ((locals.var_x_d__blk1393_dn8 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn8)))) / (assign52260_e67339 * assign52260_e67339)), );
        }

        let assign52270_e67345: f64 = if locals.var_x_d__blk1393 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1496 = assign52270_e67345;

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) {
            let assign52280_e67352: f64 = (-locals.var_x_d__blk1393);
            let assign52280_e67353: f64 = (assign52280_e67352).exp();
            (locals.var_ed__blk1399, locals.var_ed__blk1399_dn5, locals.var_ed__blk1399_dn6, locals.var_ed__blk1399_dn7, locals.var_ed__blk1399_dn8, ) = (assign52280_e67353, (assign52280_e67353 * (-locals.var_x_d__blk1393_dn5)), (assign52280_e67353 * (-locals.var_x_d__blk1393_dn6)), (assign52280_e67353 * (-locals.var_x_d__blk1393_dn7)), (assign52280_e67353 * (-locals.var_x_d__blk1393_dn8)), );
        }

        let assign52290_e67358: f64 = if locals.var_x_d__blk1393 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1497 = assign52290_e67358;

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) && (locals.var_guard1497 != 0.0)) {
            let assign52300_e67369: f64 = (locals.var_x_d__blk1393 * locals.var_x_d__blk1393);
            let assign52300_e67376: f64 = (0.25 * locals.var_x_d__blk1393);
            let assign52300_e67377: f64 = (1.0 - assign52300_e67376);
            let assign52300_e67378: f64 = (locals.var_x_d__blk1393 * assign52300_e67377);
            let assign52300_e67379: f64 = (0.3333333333333333 * assign52300_e67378);
            let assign52300_e67380: f64 = (1.0 - assign52300_e67379);
            let assign52300_e67381: f64 = (assign52300_e67369 * assign52300_e67380);
            let assign52300_e67382: f64 = (0.5 * assign52300_e67381);
            (locals.var_pd__blk1400, locals.var_pd__blk1400_dn5, locals.var_pd__blk1400_dn6, locals.var_pd__blk1400_dn7, locals.var_pd__blk1400_dn8, ) = (assign52300_e67382, (0.5 * ((((locals.var_x_d__blk1393_dn5 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn5)) * assign52300_e67380) + (assign52300_e67369 * (-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn5 * assign52300_e67377) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn5))))))))), (0.5 * ((((locals.var_x_d__blk1393_dn6 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn6)) * assign52300_e67380) + (assign52300_e67369 * (-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn6 * assign52300_e67377) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn6))))))))), (0.5 * ((((locals.var_x_d__blk1393_dn7 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn7)) * assign52300_e67380) + (assign52300_e67369 * (-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn7 * assign52300_e67377) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn7))))))))), (0.5 * ((((locals.var_x_d__blk1393_dn8 * locals.var_x_d__blk1393) + (locals.var_x_d__blk1393 * locals.var_x_d__blk1393_dn8)) * assign52300_e67380) + (assign52300_e67369 * (-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn8 * assign52300_e67377) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn8))))))))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) && (locals.var_guard1497 != 0.0)) {
            let assign52310_e67398: f64 = (0.25 * locals.var_x_d__blk1393);
            let assign52310_e67399: f64 = (1.0 - assign52310_e67398);
            let assign52310_e67400: f64 = (locals.var_x_d__blk1393 * assign52310_e67399);
            let assign52310_e67401: f64 = (0.3333333333333333 * assign52310_e67400);
            let assign52310_e67402: f64 = (1.0 - assign52310_e67401);
            let assign52310_e67403: f64 = (assign52310_e67402).sqrt();
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign52310_e67403, ((-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn5 * assign52310_e67399) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn5)))))) / (2.0 * assign52310_e67403)), ((-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn6 * assign52310_e67399) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn6)))))) / (2.0 * assign52310_e67403)), ((-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn7 * assign52310_e67399) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn7)))))) / (2.0 * assign52310_e67403)), ((-(0.3333333333333333 * ((locals.var_x_d__blk1393_dn8 * assign52310_e67399) + (locals.var_x_d__blk1393 * (-(0.25 * locals.var_x_d__blk1393_dn8)))))) / (2.0 * assign52310_e67403)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) && (locals.var_guard1497 != 0.0)) {
            let assign52320_e67416: f64 = (locals.var_x_d__blk1393 * locals.var_temp__blk936);
            let assign52320_e67417: f64 = (0.7071067811865475 * assign52320_e67416);
            (locals.var_sqd__blk1401, locals.var_sqd__blk1401_dn5, locals.var_sqd__blk1401_dn6, locals.var_sqd__blk1401_dn7, locals.var_sqd__blk1401_dn8, ) = (assign52320_e67417, (0.7071067811865475 * ((locals.var_x_d__blk1393_dn5 * locals.var_temp__blk936) + (locals.var_x_d__blk1393 * locals.var_temp__blk936_dn5))), (0.7071067811865475 * ((locals.var_x_d__blk1393_dn6 * locals.var_temp__blk936) + (locals.var_x_d__blk1393 * locals.var_temp__blk936_dn6))), (0.7071067811865475 * ((locals.var_x_d__blk1393_dn7 * locals.var_temp__blk936) + (locals.var_x_d__blk1393 * locals.var_temp__blk936_dn7))), (0.7071067811865475 * ((locals.var_x_d__blk1393_dn8 * locals.var_temp__blk936) + (locals.var_x_d__blk1393 * locals.var_temp__blk936_dn8))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) && (locals.var_guard1497 != 0.0)) {
            let assign52330_e67429: f64 = (0.16666666666666666 * locals.var_delta_nd__blk1392);
            let assign52330_e67431: f64 = (assign52330_e67429 * locals.var_x_d__blk1393);
            let assign52330_e67433: f64 = (assign52330_e67431 * locals.var_x_d__blk1393);
            let assign52330_e67435: f64 = (assign52330_e67433 * locals.var_x_d__blk1393);
            let assign52330_e67439: f64 = (1.75 * locals.var_x_d__blk1393);
            let assign52330_e67440: f64 = (1.0 + assign52330_e67439);
            let assign52330_e67441: f64 = (assign52330_e67435 * assign52330_e67440);
            (locals.var_dd__blk1402, locals.var_dd__blk1402_dn5, locals.var_dd__blk1402_dn6, locals.var_dd__blk1402_dn7, locals.var_dd__blk1402_dn8, ) = (assign52330_e67441, (((((((((0.16666666666666666 * locals.var_delta_nd__blk1392_dn5) * locals.var_x_d__blk1393) + (assign52330_e67429 * locals.var_x_d__blk1393_dn5)) * locals.var_x_d__blk1393) + (assign52330_e67431 * locals.var_x_d__blk1393_dn5)) * locals.var_x_d__blk1393) + (assign52330_e67433 * locals.var_x_d__blk1393_dn5)) * assign52330_e67440) + (assign52330_e67435 * (1.75 * locals.var_x_d__blk1393_dn5))), (((((((((0.16666666666666666 * locals.var_delta_nd__blk1392_dn6) * locals.var_x_d__blk1393) + (assign52330_e67429 * locals.var_x_d__blk1393_dn6)) * locals.var_x_d__blk1393) + (assign52330_e67431 * locals.var_x_d__blk1393_dn6)) * locals.var_x_d__blk1393) + (assign52330_e67433 * locals.var_x_d__blk1393_dn6)) * assign52330_e67440) + (assign52330_e67435 * (1.75 * locals.var_x_d__blk1393_dn6))), (((((((((0.16666666666666666 * locals.var_delta_nd__blk1392_dn7) * locals.var_x_d__blk1393) + (assign52330_e67429 * locals.var_x_d__blk1393_dn7)) * locals.var_x_d__blk1393) + (assign52330_e67431 * locals.var_x_d__blk1393_dn7)) * locals.var_x_d__blk1393) + (assign52330_e67433 * locals.var_x_d__blk1393_dn7)) * assign52330_e67440) + (assign52330_e67435 * (1.75 * locals.var_x_d__blk1393_dn7))), (((((((((0.16666666666666666 * locals.var_delta_nd__blk1392_dn8) * locals.var_x_d__blk1393) + (assign52330_e67429 * locals.var_x_d__blk1393_dn8)) * locals.var_x_d__blk1393) + (assign52330_e67431 * locals.var_x_d__blk1393_dn8)) * locals.var_x_d__blk1393) + (assign52330_e67433 * locals.var_x_d__blk1393_dn8)) * assign52330_e67440) + (assign52330_e67435 * (1.75 * locals.var_x_d__blk1393_dn8))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) && (locals.var_guard1497 == 0.0)) {
            let assign52340_e67454: f64 = (locals.var_x_d__blk1393 - 1.0);
            let assign52340_e67456: f64 = (assign52340_e67454 + locals.var_ed__blk1399);
            (locals.var_pd__blk1400, locals.var_pd__blk1400_dn5, locals.var_pd__blk1400_dn6, locals.var_pd__blk1400_dn7, locals.var_pd__blk1400_dn8, ) = (assign52340_e67456, (locals.var_x_d__blk1393_dn5 + locals.var_ed__blk1399_dn5), (locals.var_x_d__blk1393_dn6 + locals.var_ed__blk1399_dn6), (locals.var_x_d__blk1393_dn7 + locals.var_ed__blk1399_dn7), (locals.var_x_d__blk1393_dn8 + locals.var_ed__blk1399_dn8), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) && (locals.var_guard1497 == 0.0)) {
            let assign52350_e67468: f64 = (locals.var_pd__blk1400).sqrt();
            (locals.var_sqd__blk1401, locals.var_sqd__blk1401_dn5, locals.var_sqd__blk1401_dn6, locals.var_sqd__blk1401_dn7, locals.var_sqd__blk1401_dn8, ) = (assign52350_e67468, (locals.var_pd__blk1400_dn5 / (2.0 * assign52350_e67468)), (locals.var_pd__blk1400_dn6 / (2.0 * assign52350_e67468)), (locals.var_pd__blk1400_dn7 / (2.0 * assign52350_e67468)), (locals.var_pd__blk1400_dn8 / (2.0 * assign52350_e67468)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 != 0.0)) && (locals.var_guard1497 == 0.0)) {
            let assign52360_e67482: f64 = (1.0 / locals.var_ed__blk1399);
            let assign52360_e67484: f64 = (assign52360_e67482 - locals.var_x_d__blk1393);
            let assign52360_e67486: f64 = (assign52360_e67484 - 1.0);
            let assign52360_e67488: f64 = (assign52360_e67486 - locals.var_xi0d__blk1398);
            let assign52360_e67489: f64 = (locals.var_delta_nd__blk1392 * assign52360_e67488);
            (locals.var_dd__blk1402, locals.var_dd__blk1402_dn5, locals.var_dd__blk1402_dn6, locals.var_dd__blk1402_dn7, locals.var_dd__blk1402_dn8, ) = (assign52360_e67489, ((locals.var_delta_nd__blk1392_dn5 * assign52360_e67488) + (locals.var_delta_nd__blk1392 * (((-(locals.var_ed__blk1399_dn5 / (locals.var_ed__blk1399 * locals.var_ed__blk1399))) - locals.var_x_d__blk1393_dn5) - locals.var_xi0d__blk1398_dn5))), ((locals.var_delta_nd__blk1392_dn6 * assign52360_e67488) + (locals.var_delta_nd__blk1392 * (((-(locals.var_ed__blk1399_dn6 / (locals.var_ed__blk1399 * locals.var_ed__blk1399))) - locals.var_x_d__blk1393_dn6) - locals.var_xi0d__blk1398_dn6))), ((locals.var_delta_nd__blk1392_dn7 * assign52360_e67488) + (locals.var_delta_nd__blk1392 * (((-(locals.var_ed__blk1399_dn7 / (locals.var_ed__blk1399 * locals.var_ed__blk1399))) - locals.var_x_d__blk1393_dn7) - locals.var_xi0d__blk1398_dn7))), ((locals.var_delta_nd__blk1392_dn8 * assign52360_e67488) + (locals.var_delta_nd__blk1392 * (((-(locals.var_ed__blk1399_dn8 / (locals.var_ed__blk1399 * locals.var_ed__blk1399))) - locals.var_x_d__blk1393_dn8) - locals.var_xi0d__blk1398_dn8))), );
        }

        let assign52370_e67495: f64 = (locals.var_xn_d__blk1390 - 230.25850929940458);
        let assign52370_e67496: f64 = if locals.var_x_d__blk1393 > assign52370_e67495 { 1.0 } else { 0.0 };
        locals.var_guard1498 = assign52370_e67496;

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) && (locals.var_guard1498 != 0.0)) {
            let assign52380_e67507: f64 = (locals.var_x_d__blk1393 - locals.var_xn_d__blk1390);
            let assign52380_e67508: f64 = (assign52380_e67507).exp();
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign52380_e67508, (assign52380_e67508 * (locals.var_x_d__blk1393_dn5 - locals.var_xn_d__blk1390_dn5)), (assign52380_e67508 * (locals.var_x_d__blk1393_dn6 - locals.var_xn_d__blk1390_dn6)), (assign52380_e67508 * (locals.var_x_d__blk1393_dn7 - locals.var_xn_d__blk1390_dn7)), (assign52380_e67508 * (locals.var_x_d__blk1393_dn8 - locals.var_xn_d__blk1390_dn8)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) && (locals.var_guard1498 != 0.0)) {
            let assign52390_e67521: f64 = (locals.var_delta_nd__blk1392 / locals.var_temp__blk936);
            (locals.var_ed__blk1399, locals.var_ed__blk1399_dn5, locals.var_ed__blk1399_dn6, locals.var_ed__blk1399_dn7, locals.var_ed__blk1399_dn8, ) = (assign52390_e67521, (((locals.var_delta_nd__blk1392_dn5 * locals.var_temp__blk936) - (locals.var_delta_nd__blk1392 * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_delta_nd__blk1392_dn6 * locals.var_temp__blk936) - (locals.var_delta_nd__blk1392 * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_delta_nd__blk1392_dn7 * locals.var_temp__blk936) - (locals.var_delta_nd__blk1392 * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_delta_nd__blk1392_dn8 * locals.var_temp__blk936) - (locals.var_delta_nd__blk1392 * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) && (locals.var_guard1498 != 0.0)) {
            let assign52400_e67536: f64 = (locals.var_x_d__blk1393 + 1.0);
            let assign52400_e67538: f64 = (assign52400_e67536 + locals.var_xi0d__blk1398);
            let assign52400_e67539: f64 = (locals.var_delta_nd__blk1392 * assign52400_e67538);
            let assign52400_e67540: f64 = (locals.var_temp__blk936 - assign52400_e67539);
            (locals.var_dd__blk1402, locals.var_dd__blk1402_dn5, locals.var_dd__blk1402_dn6, locals.var_dd__blk1402_dn7, locals.var_dd__blk1402_dn8, ) = (assign52400_e67540, (locals.var_temp__blk936_dn5 - ((locals.var_delta_nd__blk1392_dn5 * assign52400_e67538) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn5 + locals.var_xi0d__blk1398_dn5)))), (locals.var_temp__blk936_dn6 - ((locals.var_delta_nd__blk1392_dn6 * assign52400_e67538) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn6 + locals.var_xi0d__blk1398_dn6)))), (locals.var_temp__blk936_dn7 - ((locals.var_delta_nd__blk1392_dn7 * assign52400_e67538) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn7 + locals.var_xi0d__blk1398_dn7)))), (locals.var_temp__blk936_dn8 - ((locals.var_delta_nd__blk1392_dn8 * assign52400_e67538) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn8 + locals.var_xi0d__blk1398_dn8)))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) && (locals.var_guard1498 == 0.0)) {
            let assign52410_e67556: f64 = (locals.var_x_d__blk1393 - 230.25850929940458);
            let assign52410_e67561: f64 = (locals.var_x_d__blk1393 - 230.25850929940458);
            let assign52410_e67565: f64 = (locals.var_x_d__blk1393 - 230.25850929940458);
            let assign52410_e67567: f64 = (assign52410_e67565 * 0.3333333333333333);
            let assign52410_e67568: f64 = (1.0 + assign52410_e67567);
            let assign52410_e67569: f64 = (assign52410_e67561 * assign52410_e67568);
            let assign52410_e67570: f64 = (0.5 * assign52410_e67569);
            let assign52410_e67571: f64 = (1.0 + assign52410_e67570);
            let assign52410_e67572: f64 = (assign52410_e67556 * assign52410_e67571);
            let assign52410_e67573: f64 = (1.0 + assign52410_e67572);
            let assign52410_e67574: f64 = (1e-100 / assign52410_e67573);
            (locals.var_ed__blk1399, locals.var_ed__blk1399_dn5, locals.var_ed__blk1399_dn6, locals.var_ed__blk1399_dn7, locals.var_ed__blk1399_dn8, ) = (assign52410_e67574, (-((1e-100 * ((locals.var_x_d__blk1393_dn5 * assign52410_e67571) + (assign52410_e67556 * (0.5 * ((locals.var_x_d__blk1393_dn5 * assign52410_e67568) + (assign52410_e67561 * (locals.var_x_d__blk1393_dn5 * 0.3333333333333333))))))) / (assign52410_e67573 * assign52410_e67573))), (-((1e-100 * ((locals.var_x_d__blk1393_dn6 * assign52410_e67571) + (assign52410_e67556 * (0.5 * ((locals.var_x_d__blk1393_dn6 * assign52410_e67568) + (assign52410_e67561 * (locals.var_x_d__blk1393_dn6 * 0.3333333333333333))))))) / (assign52410_e67573 * assign52410_e67573))), (-((1e-100 * ((locals.var_x_d__blk1393_dn7 * assign52410_e67571) + (assign52410_e67556 * (0.5 * ((locals.var_x_d__blk1393_dn7 * assign52410_e67568) + (assign52410_e67561 * (locals.var_x_d__blk1393_dn7 * 0.3333333333333333))))))) / (assign52410_e67573 * assign52410_e67573))), (-((1e-100 * ((locals.var_x_d__blk1393_dn8 * assign52410_e67571) + (assign52410_e67556 * (0.5 * ((locals.var_x_d__blk1393_dn8 * assign52410_e67568) + (assign52410_e67561 * (locals.var_x_d__blk1393_dn8 * 0.3333333333333333))))))) / (assign52410_e67573 * assign52410_e67573))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) && (locals.var_guard1498 == 0.0)) {
            let assign52420_e67590: f64 = (locals.var_xn_d__blk1390 - locals.var_x_d__blk1393);
            let assign52420_e67592: f64 = (assign52420_e67590 - 230.25850929940458);
            let assign52420_e67597: f64 = (locals.var_xn_d__blk1390 - locals.var_x_d__blk1393);
            let assign52420_e67599: f64 = (assign52420_e67597 - 230.25850929940458);
            let assign52420_e67603: f64 = (locals.var_xn_d__blk1390 - locals.var_x_d__blk1393);
            let assign52420_e67605: f64 = (assign52420_e67603 - 230.25850929940458);
            let assign52420_e67607: f64 = (assign52420_e67605 * 0.3333333333333333);
            let assign52420_e67608: f64 = (1.0 + assign52420_e67607);
            let assign52420_e67609: f64 = (assign52420_e67599 * assign52420_e67608);
            let assign52420_e67610: f64 = (0.5 * assign52420_e67609);
            let assign52420_e67611: f64 = (1.0 + assign52420_e67610);
            let assign52420_e67612: f64 = (assign52420_e67592 * assign52420_e67611);
            let assign52420_e67613: f64 = (1.0 + assign52420_e67612);
            let assign52420_e67614: f64 = (1e-100 / assign52420_e67613);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign52420_e67614, (-((1e-100 * (((locals.var_xn_d__blk1390_dn5 - locals.var_x_d__blk1393_dn5) * assign52420_e67611) + (assign52420_e67592 * (0.5 * (((locals.var_xn_d__blk1390_dn5 - locals.var_x_d__blk1393_dn5) * assign52420_e67608) + (assign52420_e67599 * ((locals.var_xn_d__blk1390_dn5 - locals.var_x_d__blk1393_dn5) * 0.3333333333333333))))))) / (assign52420_e67613 * assign52420_e67613))), (-((1e-100 * (((locals.var_xn_d__blk1390_dn6 - locals.var_x_d__blk1393_dn6) * assign52420_e67611) + (assign52420_e67592 * (0.5 * (((locals.var_xn_d__blk1390_dn6 - locals.var_x_d__blk1393_dn6) * assign52420_e67608) + (assign52420_e67599 * ((locals.var_xn_d__blk1390_dn6 - locals.var_x_d__blk1393_dn6) * 0.3333333333333333))))))) / (assign52420_e67613 * assign52420_e67613))), (-((1e-100 * (((locals.var_xn_d__blk1390_dn7 - locals.var_x_d__blk1393_dn7) * assign52420_e67611) + (assign52420_e67592 * (0.5 * (((locals.var_xn_d__blk1390_dn7 - locals.var_x_d__blk1393_dn7) * assign52420_e67608) + (assign52420_e67599 * ((locals.var_xn_d__blk1390_dn7 - locals.var_x_d__blk1393_dn7) * 0.3333333333333333))))))) / (assign52420_e67613 * assign52420_e67613))), (-((1e-100 * (((locals.var_xn_d__blk1390_dn8 - locals.var_x_d__blk1393_dn8) * assign52420_e67611) + (assign52420_e67592 * (0.5 * (((locals.var_xn_d__blk1390_dn8 - locals.var_x_d__blk1393_dn8) * assign52420_e67608) + (assign52420_e67599 * ((locals.var_xn_d__blk1390_dn8 - locals.var_x_d__blk1393_dn8) * 0.3333333333333333))))))) / (assign52420_e67613 * assign52420_e67613))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) && (locals.var_guard1498 == 0.0)) {
            let assign52430_e67630: f64 = (locals.var_x_d__blk1393 + 1.0);
            let assign52430_e67632: f64 = (assign52430_e67630 + locals.var_xi0d__blk1398);
            let assign52430_e67633: f64 = (locals.var_delta_nd__blk1392 * assign52430_e67632);
            let assign52430_e67634: f64 = (locals.var_temp__blk936 - assign52430_e67633);
            (locals.var_dd__blk1402, locals.var_dd__blk1402_dn5, locals.var_dd__blk1402_dn6, locals.var_dd__blk1402_dn7, locals.var_dd__blk1402_dn8, ) = (assign52430_e67634, (locals.var_temp__blk936_dn5 - ((locals.var_delta_nd__blk1392_dn5 * assign52430_e67632) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn5 + locals.var_xi0d__blk1398_dn5)))), (locals.var_temp__blk936_dn6 - ((locals.var_delta_nd__blk1392_dn6 * assign52430_e67632) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn6 + locals.var_xi0d__blk1398_dn6)))), (locals.var_temp__blk936_dn7 - ((locals.var_delta_nd__blk1392_dn7 * assign52430_e67632) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn7 + locals.var_xi0d__blk1398_dn7)))), (locals.var_temp__blk936_dn8 - ((locals.var_delta_nd__blk1392_dn8 * assign52430_e67632) + (locals.var_delta_nd__blk1392 * (locals.var_x_d__blk1393_dn8 + locals.var_xi0d__blk1398_dn8)))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) {
            let assign52440_e67645: f64 = (locals.var_x_d__blk1393 - 1.0);
            let assign52440_e67647: f64 = (assign52440_e67645 + locals.var_ed__blk1399);
            (locals.var_pd__blk1400, locals.var_pd__blk1400_dn5, locals.var_pd__blk1400_dn6, locals.var_pd__blk1400_dn7, locals.var_pd__blk1400_dn8, ) = (assign52440_e67647, (locals.var_x_d__blk1393_dn5 + locals.var_ed__blk1399_dn5), (locals.var_x_d__blk1393_dn6 + locals.var_ed__blk1399_dn6), (locals.var_x_d__blk1393_dn7 + locals.var_ed__blk1399_dn7), (locals.var_x_d__blk1393_dn8 + locals.var_ed__blk1399_dn8), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1496 == 0.0)) {
            let assign52450_e67657: f64 = (locals.var_pd__blk1400).sqrt();
            (locals.var_sqd__blk1401, locals.var_sqd__blk1401_dn5, locals.var_sqd__blk1401_dn6, locals.var_sqd__blk1401_dn7, locals.var_sqd__blk1401_dn8, ) = (assign52450_e67657, (locals.var_pd__blk1400_dn5 / (2.0 * assign52450_e67657)), (locals.var_pd__blk1400_dn6 / (2.0 * assign52450_e67657)), (locals.var_pd__blk1400_dn7 / (2.0 * assign52450_e67657)), (locals.var_pd__blk1400_dn8 / (2.0 * assign52450_e67657)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign52460_e67665: f64 = (locals.var_sqd__blk1401 * locals.var_gf__blk1307);
            let assign52460_e67667: f64 = (assign52460_e67665 * locals.var_phit1__blk1322);
            (locals.var_qbd__blk1403, locals.var_qbd__blk1403_dn5, locals.var_qbd__blk1403_dn6, locals.var_qbd__blk1403_dn7, locals.var_qbd__blk1403_dn8, ) = (assign52460_e67667, ((((locals.var_sqd__blk1401_dn5 * locals.var_gf__blk1307) + (locals.var_sqd__blk1401 * locals.var_gf__blk1307_dn5)) * locals.var_phit1__blk1322) + (assign52460_e67665 * locals.var_phit1__blk1322_dn5)), ((((locals.var_sqd__blk1401_dn6 * locals.var_gf__blk1307) + (locals.var_sqd__blk1401 * locals.var_gf__blk1307_dn6)) * locals.var_phit1__blk1322) + (assign52460_e67665 * locals.var_phit1__blk1322_dn6)), ((((locals.var_sqd__blk1401_dn7 * locals.var_gf__blk1307) + (locals.var_sqd__blk1401 * locals.var_gf__blk1307_dn7)) * locals.var_phit1__blk1322) + (assign52460_e67665 * locals.var_phit1__blk1322_dn7)), ((((locals.var_sqd__blk1401_dn8 * locals.var_gf__blk1307) + (locals.var_sqd__blk1401 * locals.var_gf__blk1307_dn8)) * locals.var_phit1__blk1322) + (assign52460_e67665 * locals.var_phit1__blk1322_dn8)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign52470_e67676: f64 = (locals.var_x_s__blk1346 + locals.var_x_d__blk1393);
            let assign52470_e67677: f64 = (0.5 * assign52470_e67676);
            (locals.var_x_m__blk1404, locals.var_x_m__blk1404_dn5, locals.var_x_m__blk1404_dn6, locals.var_x_m__blk1404_dn7, locals.var_x_m__blk1404_dn8, ) = (assign52470_e67677, (0.5 * (locals.var_x_s__blk1346_dn5 + locals.var_x_d__blk1393_dn5)), (0.5 * (locals.var_x_s__blk1346_dn6 + locals.var_x_d__blk1393_dn6)), (0.5 * (locals.var_x_s__blk1346_dn7 + locals.var_x_d__blk1393_dn7)), (0.5 * (locals.var_x_s__blk1346_dn8 + locals.var_x_d__blk1393_dn8)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            (locals.var_em__blk1405, locals.var_em__blk1405_dn5, locals.var_em__blk1405_dn6, locals.var_em__blk1405_dn7, locals.var_em__blk1405_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign52490_e67691: f64 = (locals.var_ed__blk1399 * locals.var_es__blk1352);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign52490_e67691, ((locals.var_ed__blk1399_dn5 * locals.var_es__blk1352) + (locals.var_ed__blk1399 * locals.var_es__blk1352_dn5)), ((locals.var_ed__blk1399_dn6 * locals.var_es__blk1352) + (locals.var_ed__blk1399 * locals.var_es__blk1352_dn6)), ((locals.var_ed__blk1399_dn7 * locals.var_es__blk1352) + (locals.var_ed__blk1399 * locals.var_es__blk1352_dn7)), ((locals.var_ed__blk1399_dn8 * locals.var_es__blk1352) + (locals.var_ed__blk1399 * locals.var_es__blk1352_dn8)), );
        }

        let assign52500_e67696: f64 = if locals.var_temp__blk936 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1499 = assign52500_e67696;

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1499 != 0.0)) {
            let assign52510_e67703: f64 = (locals.var_temp__blk936).sqrt();
            (locals.var_em__blk1405, locals.var_em__blk1405_dn5, locals.var_em__blk1405_dn6, locals.var_em__blk1405_dn7, locals.var_em__blk1405_dn8, ) = (assign52510_e67703, (locals.var_temp__blk936_dn5 / (2.0 * assign52510_e67703)), (locals.var_temp__blk936_dn6 / (2.0 * assign52510_e67703)), (locals.var_temp__blk936_dn7 / (2.0 * assign52510_e67703)), (locals.var_temp__blk936_dn8 / (2.0 * assign52510_e67703)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign52520_e67712: f64 = (locals.var_ds__blk1353 + locals.var_dd__blk1402);
            let assign52520_e67713: f64 = (0.5 * assign52520_e67712);
            (locals.var_d_bar__blk1406, locals.var_d_bar__blk1406_dn5, locals.var_d_bar__blk1406_dn6, locals.var_d_bar__blk1406_dn7, locals.var_d_bar__blk1406_dn8, ) = (assign52520_e67713, (0.5 * (locals.var_ds__blk1353_dn5 + locals.var_dd__blk1402_dn5)), (0.5 * (locals.var_ds__blk1353_dn6 + locals.var_dd__blk1402_dn6)), (0.5 * (locals.var_ds__blk1353_dn7 + locals.var_dd__blk1402_dn7)), (0.5 * (locals.var_ds__blk1353_dn8 + locals.var_dd__blk1402_dn8)), );
        }

    }

    pub(super) fn stamp_transient_block_25(
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign52530_e67723: f64 = (locals.var_x_ds__blk1394 * locals.var_x_ds__blk1394);
            let assign52530_e67727: f64 = (2.0 * locals.var_inv_gf2__blk1324);
            let assign52530_e67728: f64 = (locals.var_em__blk1405 - assign52530_e67727);
            let assign52530_e67729: f64 = (assign52530_e67723 * assign52530_e67728);
            let assign52530_e67730: f64 = (0.125 * assign52530_e67729);
            let assign52530_e67731: f64 = (locals.var_d_bar__blk1406 + assign52530_e67730);
            (locals.var_dm__blk1407, locals.var_dm__blk1407_dn5, locals.var_dm__blk1407_dn6, locals.var_dm__blk1407_dn7, locals.var_dm__blk1407_dn8, ) = (assign52530_e67731, (locals.var_d_bar__blk1406_dn5 + (0.125 * ((((locals.var_x_ds__blk1394_dn5 * locals.var_x_ds__blk1394) + (locals.var_x_ds__blk1394 * locals.var_x_ds__blk1394_dn5)) * assign52530_e67728) + (assign52530_e67723 * (locals.var_em__blk1405_dn5 - (2.0 * locals.var_inv_gf2__blk1324_dn5)))))), (locals.var_d_bar__blk1406_dn6 + (0.125 * ((((locals.var_x_ds__blk1394_dn6 * locals.var_x_ds__blk1394) + (locals.var_x_ds__blk1394 * locals.var_x_ds__blk1394_dn6)) * assign52530_e67728) + (assign52530_e67723 * (locals.var_em__blk1405_dn6 - (2.0 * locals.var_inv_gf2__blk1324_dn6)))))), (locals.var_d_bar__blk1406_dn7 + (0.125 * ((((locals.var_x_ds__blk1394_dn7 * locals.var_x_ds__blk1394) + (locals.var_x_ds__blk1394 * locals.var_x_ds__blk1394_dn7)) * assign52530_e67728) + (assign52530_e67723 * (locals.var_em__blk1405_dn7 - (2.0 * locals.var_inv_gf2__blk1324_dn7)))))), (locals.var_d_bar__blk1406_dn8 + (0.125 * ((((locals.var_x_ds__blk1394_dn8 * locals.var_x_ds__blk1394) + (locals.var_x_ds__blk1394 * locals.var_x_ds__blk1394_dn8)) * assign52530_e67728) + (assign52530_e67723 * (locals.var_em__blk1405_dn8 - (2.0 * locals.var_inv_gf2__blk1324_dn8)))))), );
        }

        let assign52540_e67736: f64 = if locals.var_x_m__blk1404 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1500 = assign52540_e67736;

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 != 0.0)) {
            let assign52550_e67745: f64 = (locals.var_x_m__blk1404 * locals.var_x_m__blk1404);
            let assign52550_e67752: f64 = (0.25 * locals.var_x_m__blk1404);
            let assign52550_e67753: f64 = (1.0 - assign52550_e67752);
            let assign52550_e67754: f64 = (locals.var_x_m__blk1404 * assign52550_e67753);
            let assign52550_e67755: f64 = (0.3333333333333333 * assign52550_e67754);
            let assign52550_e67756: f64 = (1.0 - assign52550_e67755);
            let assign52550_e67757: f64 = (assign52550_e67745 * assign52550_e67756);
            let assign52550_e67758: f64 = (0.5 * assign52550_e67757);
            (locals.var_pm__blk1408, locals.var_pm__blk1408_dn5, locals.var_pm__blk1408_dn6, locals.var_pm__blk1408_dn7, locals.var_pm__blk1408_dn8, ) = (assign52550_e67758, (0.5 * ((((locals.var_x_m__blk1404_dn5 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn5)) * assign52550_e67756) + (assign52550_e67745 * (-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn5 * assign52550_e67753) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn5))))))))), (0.5 * ((((locals.var_x_m__blk1404_dn6 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn6)) * assign52550_e67756) + (assign52550_e67745 * (-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn6 * assign52550_e67753) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn6))))))))), (0.5 * ((((locals.var_x_m__blk1404_dn7 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn7)) * assign52550_e67756) + (assign52550_e67745 * (-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn7 * assign52550_e67753) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn7))))))))), (0.5 * ((((locals.var_x_m__blk1404_dn8 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn8)) * assign52550_e67756) + (assign52550_e67745 * (-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn8 * assign52550_e67753) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn8))))))))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 != 0.0)) {
            let assign52560_e67769: f64 = (locals.var_dm__blk1407 + locals.var_pm__blk1408);
            let assign52560_e67770: f64 = (assign52560_e67769).sqrt();
            let assign52560_e67771: f64 = (locals.var_gf__blk1307 * assign52560_e67770);
            (locals.var_xgm__blk1409, locals.var_xgm__blk1409_dn5, locals.var_xgm__blk1409_dn6, locals.var_xgm__blk1409_dn7, locals.var_xgm__blk1409_dn8, ) = (assign52560_e67771, ((locals.var_gf__blk1307_dn5 * assign52560_e67770) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn5 + locals.var_pm__blk1408_dn5) / (2.0 * assign52560_e67770)))), ((locals.var_gf__blk1307_dn6 * assign52560_e67770) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn6 + locals.var_pm__blk1408_dn6) / (2.0 * assign52560_e67770)))), ((locals.var_gf__blk1307_dn7 * assign52560_e67770) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn7 + locals.var_pm__blk1408_dn7) / (2.0 * assign52560_e67770)))), ((locals.var_gf__blk1307_dn8 * assign52560_e67770) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn8 + locals.var_pm__blk1408_dn8) / (2.0 * assign52560_e67770)))), );
        }

        let assign52570_e67776: f64 = if locals.var_kp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1501 = assign52570_e67776;

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 != 0.0)) && (locals.var_guard1501 != 0.0)) {
            let assign52580_e67788: f64 = (locals.var_kp * locals.var_xgm__blk1409);
            let assign52580_e67789: f64 = (1.0 + assign52580_e67788);
            let assign52580_e67790: f64 = (assign52580_e67789).sqrt();
            let assign52580_e67791: f64 = (1.0 / assign52580_e67790);
            (locals.var_eta_p__blk1410, locals.var_eta_p__blk1410_dn5, locals.var_eta_p__blk1410_dn6, locals.var_eta_p__blk1410_dn7, locals.var_eta_p__blk1410_dn8, ) = (assign52580_e67791, (-(((locals.var_kp * locals.var_xgm__blk1409_dn5) / (2.0 * assign52580_e67790)) / (assign52580_e67790 * assign52580_e67790))), (-(((locals.var_kp * locals.var_xgm__blk1409_dn6) / (2.0 * assign52580_e67790)) / (assign52580_e67790 * assign52580_e67790))), (-(((locals.var_kp * locals.var_xgm__blk1409_dn7) / (2.0 * assign52580_e67790)) / (assign52580_e67790 * assign52580_e67790))), (-(((locals.var_kp * locals.var_xgm__blk1409_dn8) / (2.0 * assign52580_e67790)) / (assign52580_e67790 * assign52580_e67790))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 != 0.0)) {
            let assign52590_e67805: f64 = (0.25 * locals.var_x_m__blk1404);
            let assign52590_e67806: f64 = (1.0 - assign52590_e67805);
            let assign52590_e67807: f64 = (locals.var_x_m__blk1404 * assign52590_e67806);
            let assign52590_e67808: f64 = (0.3333333333333333 * assign52590_e67807);
            let assign52590_e67809: f64 = (1.0 - assign52590_e67808);
            let assign52590_e67810: f64 = (assign52590_e67809).sqrt();
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign52590_e67810, ((-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn5 * assign52590_e67806) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn5)))))) / (2.0 * assign52590_e67810)), ((-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn6 * assign52590_e67806) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn6)))))) / (2.0 * assign52590_e67810)), ((-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn7 * assign52590_e67806) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn7)))))) / (2.0 * assign52590_e67810)), ((-(0.3333333333333333 * ((locals.var_x_m__blk1404_dn8 * assign52590_e67806) + (locals.var_x_m__blk1404 * (-(0.25 * locals.var_x_m__blk1404_dn8)))))) / (2.0 * assign52590_e67810)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 != 0.0)) {
            let assign52600_e67821: f64 = (locals.var_x_m__blk1404 * locals.var_temp__blk936);
            let assign52600_e67822: f64 = (0.7071067811865475 * assign52600_e67821);
            (locals.var_sqm__blk1411, locals.var_sqm__blk1411_dn5, locals.var_sqm__blk1411_dn6, locals.var_sqm__blk1411_dn7, locals.var_sqm__blk1411_dn8, ) = (assign52600_e67822, (0.7071067811865475 * ((locals.var_x_m__blk1404_dn5 * locals.var_temp__blk936) + (locals.var_x_m__blk1404 * locals.var_temp__blk936_dn5))), (0.7071067811865475 * ((locals.var_x_m__blk1404_dn6 * locals.var_temp__blk936) + (locals.var_x_m__blk1404 * locals.var_temp__blk936_dn6))), (0.7071067811865475 * ((locals.var_x_m__blk1404_dn7 * locals.var_temp__blk936) + (locals.var_x_m__blk1404 * locals.var_temp__blk936_dn7))), (0.7071067811865475 * ((locals.var_x_m__blk1404_dn8 * locals.var_temp__blk936) + (locals.var_x_m__blk1404 * locals.var_temp__blk936_dn8))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 != 0.0)) {
            let assign52610_e67836: f64 = (0.5 * locals.var_x_m__blk1404);
            let assign52610_e67837: f64 = (1.0 - assign52610_e67836);
            let assign52610_e67841: f64 = (locals.var_x_m__blk1404 * locals.var_x_m__blk1404);
            let assign52610_e67842: f64 = (0.16666666666666666 * assign52610_e67841);
            let assign52610_e67843: f64 = (assign52610_e67837 + assign52610_e67842);
            let assign52610_e67844: f64 = (locals.var_gf__blk1307 * assign52610_e67843);
            let assign52610_e67846: f64 = (assign52610_e67844 / locals.var_temp__blk936);
            let assign52610_e67847: f64 = (0.7071067811865475 * assign52610_e67846);
            let assign52610_e67848: f64 = (locals.var_eta_p__blk1410 + assign52610_e67847);
            (locals.var_alpha__blk1412, locals.var_alpha__blk1412_dn5, locals.var_alpha__blk1412_dn6, locals.var_alpha__blk1412_dn7, locals.var_alpha__blk1412_dn8, ) = (assign52610_e67848, (locals.var_eta_p__blk1410_dn5 + (0.7071067811865475 * (((((locals.var_gf__blk1307_dn5 * assign52610_e67843) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_m__blk1404_dn5)) + (0.16666666666666666 * ((locals.var_x_m__blk1404_dn5 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn5)))))) * locals.var_temp__blk936) - (assign52610_e67844 * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))), (locals.var_eta_p__blk1410_dn6 + (0.7071067811865475 * (((((locals.var_gf__blk1307_dn6 * assign52610_e67843) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_m__blk1404_dn6)) + (0.16666666666666666 * ((locals.var_x_m__blk1404_dn6 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn6)))))) * locals.var_temp__blk936) - (assign52610_e67844 * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))), (locals.var_eta_p__blk1410_dn7 + (0.7071067811865475 * (((((locals.var_gf__blk1307_dn7 * assign52610_e67843) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_m__blk1404_dn7)) + (0.16666666666666666 * ((locals.var_x_m__blk1404_dn7 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn7)))))) * locals.var_temp__blk936) - (assign52610_e67844 * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))), (locals.var_eta_p__blk1410_dn8 + (0.7071067811865475 * (((((locals.var_gf__blk1307_dn8 * assign52610_e67843) + (locals.var_gf__blk1307 * ((-(0.5 * locals.var_x_m__blk1404_dn8)) + (0.16666666666666666 * ((locals.var_x_m__blk1404_dn8 * locals.var_x_m__blk1404) + (locals.var_x_m__blk1404 * locals.var_x_m__blk1404_dn8)))))) * locals.var_temp__blk936) - (assign52610_e67844 * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936)))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) {
            let assign52620_e67859: f64 = (locals.var_x_m__blk1404 - 1.0);
            let assign52620_e67861: f64 = (assign52620_e67859 + locals.var_em__blk1405);
            (locals.var_pm__blk1408, locals.var_pm__blk1408_dn5, locals.var_pm__blk1408_dn6, locals.var_pm__blk1408_dn7, locals.var_pm__blk1408_dn8, ) = (assign52620_e67861, (locals.var_x_m__blk1404_dn5 + locals.var_em__blk1405_dn5), (locals.var_x_m__blk1404_dn6 + locals.var_em__blk1405_dn6), (locals.var_x_m__blk1404_dn7 + locals.var_em__blk1405_dn7), (locals.var_x_m__blk1404_dn8 + locals.var_em__blk1405_dn8), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) {
            let assign52630_e67873: f64 = (locals.var_dm__blk1407 + locals.var_pm__blk1408);
            let assign52630_e67874: f64 = (assign52630_e67873).sqrt();
            let assign52630_e67875: f64 = (locals.var_gf__blk1307 * assign52630_e67874);
            (locals.var_xgm__blk1409, locals.var_xgm__blk1409_dn5, locals.var_xgm__blk1409_dn6, locals.var_xgm__blk1409_dn7, locals.var_xgm__blk1409_dn8, ) = (assign52630_e67875, ((locals.var_gf__blk1307_dn5 * assign52630_e67874) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn5 + locals.var_pm__blk1408_dn5) / (2.0 * assign52630_e67874)))), ((locals.var_gf__blk1307_dn6 * assign52630_e67874) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn6 + locals.var_pm__blk1408_dn6) / (2.0 * assign52630_e67874)))), ((locals.var_gf__blk1307_dn7 * assign52630_e67874) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn7 + locals.var_pm__blk1408_dn7) / (2.0 * assign52630_e67874)))), ((locals.var_gf__blk1307_dn8 * assign52630_e67874) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn8 + locals.var_pm__blk1408_dn8) / (2.0 * assign52630_e67874)))), );
        }

        let assign52640_e67880: f64 = if locals.var_kp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1502 = assign52640_e67880;

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign52650_e67891: f64 = (1.0 - locals.var_em__blk1405);
            let assign52650_e67895: f64 = (locals.var_xgm__blk1409 * locals.var_inv_gf2__blk1324);
            let assign52650_e67896: f64 = (2.0 * assign52650_e67895);
            let assign52650_e67897: f64 = (assign52650_e67891 + assign52650_e67896);
            (locals.var_d0__blk1413, locals.var_d0__blk1413_dn5, locals.var_d0__blk1413_dn6, locals.var_d0__blk1413_dn7, locals.var_d0__blk1413_dn8, ) = (assign52650_e67897, ((-locals.var_em__blk1405_dn5) + (2.0 * ((locals.var_xgm__blk1409_dn5 * locals.var_inv_gf2__blk1324) + (locals.var_xgm__blk1409 * locals.var_inv_gf2__blk1324_dn5)))), ((-locals.var_em__blk1405_dn6) + (2.0 * ((locals.var_xgm__blk1409_dn6 * locals.var_inv_gf2__blk1324) + (locals.var_xgm__blk1409 * locals.var_inv_gf2__blk1324_dn6)))), ((-locals.var_em__blk1405_dn7) + (2.0 * ((locals.var_xgm__blk1409_dn7 * locals.var_inv_gf2__blk1324) + (locals.var_xgm__blk1409 * locals.var_inv_gf2__blk1324_dn7)))), ((-locals.var_em__blk1405_dn8) + (2.0 * ((locals.var_xgm__blk1409_dn8 * locals.var_inv_gf2__blk1324) + (locals.var_xgm__blk1409 * locals.var_inv_gf2__blk1324_dn8)))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign52660_e67912: f64 = (locals.var_kp * locals.var_xgm__blk1409);
            let assign52660_e67913: f64 = (1.0 + assign52660_e67912);
            let assign52660_e67914: f64 = (assign52660_e67913).sqrt();
            let assign52660_e67915: f64 = (1.0 / assign52660_e67914);
            (locals.var_eta_p__blk1410, locals.var_eta_p__blk1410_dn5, locals.var_eta_p__blk1410_dn6, locals.var_eta_p__blk1410_dn7, locals.var_eta_p__blk1410_dn8, ) = (assign52660_e67915, (-(((locals.var_kp * locals.var_xgm__blk1409_dn5) / (2.0 * assign52660_e67914)) / (assign52660_e67914 * assign52660_e67914))), (-(((locals.var_kp * locals.var_xgm__blk1409_dn6) / (2.0 * assign52660_e67914)) / (assign52660_e67914 * assign52660_e67914))), (-(((locals.var_kp * locals.var_xgm__blk1409_dn7) / (2.0 * assign52660_e67914)) / (assign52660_e67914 * assign52660_e67914))), (-(((locals.var_kp * locals.var_xgm__blk1409_dn8) / (2.0 * assign52660_e67914)) / (assign52660_e67914 * assign52660_e67914))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign52670_e67929: f64 = (locals.var_eta_p__blk1410 + 1.0);
            let assign52670_e67930: f64 = (locals.var_eta_p__blk1410 / assign52670_e67929);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign52670_e67930, (((locals.var_eta_p__blk1410_dn5 * assign52670_e67929) - (locals.var_eta_p__blk1410 * locals.var_eta_p__blk1410_dn5)) / (assign52670_e67929 * assign52670_e67929)), (((locals.var_eta_p__blk1410_dn6 * assign52670_e67929) - (locals.var_eta_p__blk1410 * locals.var_eta_p__blk1410_dn6)) / (assign52670_e67929 * assign52670_e67929)), (((locals.var_eta_p__blk1410_dn7 * assign52670_e67929) - (locals.var_eta_p__blk1410 * locals.var_eta_p__blk1410_dn7)) / (assign52670_e67929 * assign52670_e67929)), (((locals.var_eta_p__blk1410_dn8 * assign52670_e67929) - (locals.var_eta_p__blk1410 * locals.var_eta_p__blk1410_dn8)) / (assign52670_e67929 * assign52670_e67929)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign52680_e67944: f64 = (locals.var_temp__blk936 * locals.var_temp__blk936);
            let assign52680_e67946: f64 = (assign52680_e67944 * locals.var_gf2__blk1308);
            let assign52680_e67948: f64 = (assign52680_e67946 * locals.var_dm__blk1407);
            let assign52680_e67949: f64 = (locals.var_kp * assign52680_e67948);
            (locals.var_x_pm__blk1414, locals.var_x_pm__blk1414_dn5, locals.var_x_pm__blk1414_dn6, locals.var_x_pm__blk1414_dn7, locals.var_x_pm__blk1414_dn8, ) = (assign52680_e67949, (locals.var_kp * ((((((locals.var_temp__blk936_dn5 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn5)) * locals.var_gf2__blk1308) + (assign52680_e67944 * locals.var_gf2__blk1308_dn5)) * locals.var_dm__blk1407) + (assign52680_e67946 * locals.var_dm__blk1407_dn5))), (locals.var_kp * ((((((locals.var_temp__blk936_dn6 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn6)) * locals.var_gf2__blk1308) + (assign52680_e67944 * locals.var_gf2__blk1308_dn6)) * locals.var_dm__blk1407) + (assign52680_e67946 * locals.var_dm__blk1407_dn6))), (locals.var_kp * ((((((locals.var_temp__blk936_dn7 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn7)) * locals.var_gf2__blk1308) + (assign52680_e67944 * locals.var_gf2__blk1308_dn7)) * locals.var_dm__blk1407) + (assign52680_e67946 * locals.var_dm__blk1407_dn7))), (locals.var_kp * ((((((locals.var_temp__blk936_dn8 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn8)) * locals.var_gf2__blk1308) + (assign52680_e67944 * locals.var_gf2__blk1308_dn8)) * locals.var_dm__blk1407) + (assign52680_e67946 * locals.var_dm__blk1407_dn8))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign52690_e67963: f64 = (locals.var_xgm__blk1409 - locals.var_x_pm__blk1414);
            let assign52690_e67964: f64 = (2.0 * assign52690_e67963);
            let assign52690_e67968: f64 = (1.0 - locals.var_em__blk1405);
            let assign52690_e67970: f64 = (assign52690_e67968 + locals.var_dm__blk1407);
            let assign52690_e67971: f64 = (locals.var_gf2__blk1308 * assign52690_e67970);
            let assign52690_e67972: f64 = (assign52690_e67964 + assign52690_e67971);
            (locals.var_p_pd__blk1415, locals.var_p_pd__blk1415_dn5, locals.var_p_pd__blk1415_dn6, locals.var_p_pd__blk1415_dn7, locals.var_p_pd__blk1415_dn8, ) = (assign52690_e67972, ((2.0 * (locals.var_xgm__blk1409_dn5 - locals.var_x_pm__blk1414_dn5)) + ((locals.var_gf2__blk1308_dn5 * assign52690_e67970) + (locals.var_gf2__blk1308 * ((-locals.var_em__blk1405_dn5) + locals.var_dm__blk1407_dn5)))), ((2.0 * (locals.var_xgm__blk1409_dn6 - locals.var_x_pm__blk1414_dn6)) + ((locals.var_gf2__blk1308_dn6 * assign52690_e67970) + (locals.var_gf2__blk1308 * ((-locals.var_em__blk1405_dn6) + locals.var_dm__blk1407_dn6)))), ((2.0 * (locals.var_xgm__blk1409_dn7 - locals.var_x_pm__blk1414_dn7)) + ((locals.var_gf2__blk1308_dn7 * assign52690_e67970) + (locals.var_gf2__blk1308 * ((-locals.var_em__blk1405_dn7) + locals.var_dm__blk1407_dn7)))), ((2.0 * (locals.var_xgm__blk1409_dn8 - locals.var_x_pm__blk1414_dn8)) + ((locals.var_gf2__blk1308_dn8 * assign52690_e67970) + (locals.var_gf2__blk1308 * ((-locals.var_em__blk1405_dn8) + locals.var_dm__blk1407_dn8)))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign52700_e67987: f64 = (2.0 * locals.var_xgm__blk1409);
            let assign52700_e67988: f64 = (locals.var_x_pm__blk1414 - assign52700_e67987);
            let assign52700_e67989: f64 = (locals.var_x_pm__blk1414 * assign52700_e67988);
            (locals.var_q_pd__blk1416, locals.var_q_pd__blk1416_dn5, locals.var_q_pd__blk1416_dn6, locals.var_q_pd__blk1416_dn7, locals.var_q_pd__blk1416_dn8, ) = (assign52700_e67989, ((locals.var_x_pm__blk1414_dn5 * assign52700_e67988) + (locals.var_x_pm__blk1414 * (locals.var_x_pm__blk1414_dn5 - (2.0 * locals.var_xgm__blk1409_dn5)))), ((locals.var_x_pm__blk1414_dn6 * assign52700_e67988) + (locals.var_x_pm__blk1414 * (locals.var_x_pm__blk1414_dn6 - (2.0 * locals.var_xgm__blk1409_dn6)))), ((locals.var_x_pm__blk1414_dn7 * assign52700_e67988) + (locals.var_x_pm__blk1414 * (locals.var_x_pm__blk1414_dn7 - (2.0 * locals.var_xgm__blk1409_dn7)))), ((locals.var_x_pm__blk1414_dn8 * assign52700_e67988) + (locals.var_x_pm__blk1414 * (locals.var_x_pm__blk1414_dn8 - (2.0 * locals.var_xgm__blk1409_dn8)))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign52710_e68005: f64 = (locals.var_em__blk1405 + locals.var_dm__blk1407);
            let assign52710_e68006: f64 = (locals.var_gf2__blk1308 * assign52710_e68005);
            let assign52710_e68007: f64 = (0.5 * assign52710_e68006);
            let assign52710_e68008: f64 = (1.0 - assign52710_e68007);
            (locals.var_xi_pd__blk1417, locals.var_xi_pd__blk1417_dn5, locals.var_xi_pd__blk1417_dn6, locals.var_xi_pd__blk1417_dn7, locals.var_xi_pd__blk1417_dn8, ) = (assign52710_e68008, (-(0.5 * ((locals.var_gf2__blk1308_dn5 * assign52710_e68005) + (locals.var_gf2__blk1308 * (locals.var_em__blk1405_dn5 + locals.var_dm__blk1407_dn5))))), (-(0.5 * ((locals.var_gf2__blk1308_dn6 * assign52710_e68005) + (locals.var_gf2__blk1308 * (locals.var_em__blk1405_dn6 + locals.var_dm__blk1407_dn6))))), (-(0.5 * ((locals.var_gf2__blk1308_dn7 * assign52710_e68005) + (locals.var_gf2__blk1308 * (locals.var_em__blk1405_dn7 + locals.var_dm__blk1407_dn7))))), (-(0.5 * ((locals.var_gf2__blk1308_dn8 * assign52710_e68005) + (locals.var_gf2__blk1308 * (locals.var_em__blk1405_dn8 + locals.var_dm__blk1407_dn8))))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign52720_e68021: f64 = (locals.var_q_pd__blk1416 * locals.var_p_pd__blk1415);
            let assign52720_e68024: f64 = (locals.var_p_pd__blk1415 * locals.var_p_pd__blk1415);
            let assign52720_e68027: f64 = (locals.var_xi_pd__blk1417 * locals.var_q_pd__blk1416);
            let assign52720_e68028: f64 = (assign52720_e68024 - assign52720_e68027);
            let assign52720_e68029: f64 = (assign52720_e68021 / assign52720_e68028);
            (locals.var_u_pd__blk1418, locals.var_u_pd__blk1418_dn5, locals.var_u_pd__blk1418_dn6, locals.var_u_pd__blk1418_dn7, locals.var_u_pd__blk1418_dn8, ) = (assign52720_e68029, (((((locals.var_q_pd__blk1416_dn5 * locals.var_p_pd__blk1415) + (locals.var_q_pd__blk1416 * locals.var_p_pd__blk1415_dn5)) * assign52720_e68028) - (assign52720_e68021 * (((locals.var_p_pd__blk1415_dn5 * locals.var_p_pd__blk1415) + (locals.var_p_pd__blk1415 * locals.var_p_pd__blk1415_dn5)) - ((locals.var_xi_pd__blk1417_dn5 * locals.var_q_pd__blk1416) + (locals.var_xi_pd__blk1417 * locals.var_q_pd__blk1416_dn5))))) / (assign52720_e68028 * assign52720_e68028)), (((((locals.var_q_pd__blk1416_dn6 * locals.var_p_pd__blk1415) + (locals.var_q_pd__blk1416 * locals.var_p_pd__blk1415_dn6)) * assign52720_e68028) - (assign52720_e68021 * (((locals.var_p_pd__blk1415_dn6 * locals.var_p_pd__blk1415) + (locals.var_p_pd__blk1415 * locals.var_p_pd__blk1415_dn6)) - ((locals.var_xi_pd__blk1417_dn6 * locals.var_q_pd__blk1416) + (locals.var_xi_pd__blk1417 * locals.var_q_pd__blk1416_dn6))))) / (assign52720_e68028 * assign52720_e68028)), (((((locals.var_q_pd__blk1416_dn7 * locals.var_p_pd__blk1415) + (locals.var_q_pd__blk1416 * locals.var_p_pd__blk1415_dn7)) * assign52720_e68028) - (assign52720_e68021 * (((locals.var_p_pd__blk1415_dn7 * locals.var_p_pd__blk1415) + (locals.var_p_pd__blk1415 * locals.var_p_pd__blk1415_dn7)) - ((locals.var_xi_pd__blk1417_dn7 * locals.var_q_pd__blk1416) + (locals.var_xi_pd__blk1417 * locals.var_q_pd__blk1416_dn7))))) / (assign52720_e68028 * assign52720_e68028)), (((((locals.var_q_pd__blk1416_dn8 * locals.var_p_pd__blk1415) + (locals.var_q_pd__blk1416 * locals.var_p_pd__blk1415_dn8)) * assign52720_e68028) - (assign52720_e68021 * (((locals.var_p_pd__blk1415_dn8 * locals.var_p_pd__blk1415) + (locals.var_p_pd__blk1415 * locals.var_p_pd__blk1415_dn8)) - ((locals.var_xi_pd__blk1417_dn8 * locals.var_q_pd__blk1416) + (locals.var_xi_pd__blk1417 * locals.var_q_pd__blk1416_dn8))))) / (assign52720_e68028 * assign52720_e68028)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign52730_e68042: f64 = (locals.var_x_m__blk1404 + locals.var_u_pd__blk1418);
            (locals.var_x_m__blk1404, locals.var_x_m__blk1404_dn5, locals.var_x_m__blk1404_dn6, locals.var_x_m__blk1404_dn7, locals.var_x_m__blk1404_dn8, ) = (assign52730_e68042, (locals.var_x_m__blk1404_dn5 + locals.var_u_pd__blk1418_dn5), (locals.var_x_m__blk1404_dn6 + locals.var_u_pd__blk1418_dn6), (locals.var_x_m__blk1404_dn7 + locals.var_u_pd__blk1418_dn7), (locals.var_x_m__blk1404_dn8 + locals.var_u_pd__blk1418_dn8), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign52740_e68054: f64 = (locals.var_u_pd__blk1418).exp();
            (locals.var_km__blk1419, locals.var_km__blk1419_dn5, locals.var_km__blk1419_dn6, locals.var_km__blk1419_dn7, locals.var_km__blk1419_dn8, ) = (assign52740_e68054, (assign52740_e68054 * locals.var_u_pd__blk1418_dn5), (assign52740_e68054 * locals.var_u_pd__blk1418_dn6), (assign52740_e68054 * locals.var_u_pd__blk1418_dn7), (assign52740_e68054 * locals.var_u_pd__blk1418_dn8), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign52750_e68067: f64 = (locals.var_em__blk1405 / locals.var_km__blk1419);
            (locals.var_em__blk1405, locals.var_em__blk1405_dn5, locals.var_em__blk1405_dn6, locals.var_em__blk1405_dn7, locals.var_em__blk1405_dn8, ) = (assign52750_e68067, (((locals.var_em__blk1405_dn5 * locals.var_km__blk1419) - (locals.var_em__blk1405 * locals.var_km__blk1419_dn5)) / (locals.var_km__blk1419 * locals.var_km__blk1419)), (((locals.var_em__blk1405_dn6 * locals.var_km__blk1419) - (locals.var_em__blk1405 * locals.var_km__blk1419_dn6)) / (locals.var_km__blk1419 * locals.var_km__blk1419)), (((locals.var_em__blk1405_dn7 * locals.var_km__blk1419) - (locals.var_em__blk1405 * locals.var_km__blk1419_dn7)) / (locals.var_km__blk1419 * locals.var_km__blk1419)), (((locals.var_em__blk1405_dn8 * locals.var_km__blk1419) - (locals.var_em__blk1405 * locals.var_km__blk1419_dn8)) / (locals.var_km__blk1419 * locals.var_km__blk1419)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign52760_e68080: f64 = (locals.var_dm__blk1407 * locals.var_km__blk1419);
            (locals.var_dm__blk1407, locals.var_dm__blk1407_dn5, locals.var_dm__blk1407_dn6, locals.var_dm__blk1407_dn7, locals.var_dm__blk1407_dn8, ) = (assign52760_e68080, ((locals.var_dm__blk1407_dn5 * locals.var_km__blk1419) + (locals.var_dm__blk1407 * locals.var_km__blk1419_dn5)), ((locals.var_dm__blk1407_dn6 * locals.var_km__blk1419) + (locals.var_dm__blk1407 * locals.var_km__blk1419_dn6)), ((locals.var_dm__blk1407_dn7 * locals.var_km__blk1419) + (locals.var_dm__blk1407 * locals.var_km__blk1419_dn7)), ((locals.var_dm__blk1407_dn8 * locals.var_km__blk1419) + (locals.var_dm__blk1407 * locals.var_km__blk1419_dn8)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign52770_e68093: f64 = (locals.var_x_m__blk1404 - 1.0);
            let assign52770_e68095: f64 = (assign52770_e68093 + locals.var_em__blk1405);
            (locals.var_pm__blk1408, locals.var_pm__blk1408_dn5, locals.var_pm__blk1408_dn6, locals.var_pm__blk1408_dn7, locals.var_pm__blk1408_dn8, ) = (assign52770_e68095, (locals.var_x_m__blk1404_dn5 + locals.var_em__blk1405_dn5), (locals.var_x_m__blk1404_dn6 + locals.var_em__blk1405_dn6), (locals.var_x_m__blk1404_dn7 + locals.var_em__blk1405_dn7), (locals.var_x_m__blk1404_dn8 + locals.var_em__blk1405_dn8), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign52780_e68109: f64 = (locals.var_dm__blk1407 + locals.var_pm__blk1408);
            let assign52780_e68110: f64 = (assign52780_e68109).sqrt();
            let assign52780_e68111: f64 = (locals.var_gf__blk1307 * assign52780_e68110);
            (locals.var_xgm__blk1409, locals.var_xgm__blk1409_dn5, locals.var_xgm__blk1409_dn6, locals.var_xgm__blk1409_dn7, locals.var_xgm__blk1409_dn8, ) = (assign52780_e68111, ((locals.var_gf__blk1307_dn5 * assign52780_e68110) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn5 + locals.var_pm__blk1408_dn5) / (2.0 * assign52780_e68110)))), ((locals.var_gf__blk1307_dn6 * assign52780_e68110) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn6 + locals.var_pm__blk1408_dn6) / (2.0 * assign52780_e68110)))), ((locals.var_gf__blk1307_dn7 * assign52780_e68110) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn7 + locals.var_pm__blk1408_dn7) / (2.0 * assign52780_e68110)))), ((locals.var_gf__blk1307_dn8 * assign52780_e68110) + (locals.var_gf__blk1307 * ((locals.var_dm__blk1407_dn8 + locals.var_pm__blk1408_dn8) / (2.0 * assign52780_e68110)))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign52790_e68124: f64 = (1.0 - locals.var_em__blk1405);
            let assign52790_e68128: f64 = (locals.var_xgm__blk1409 * locals.var_eta_p__blk1410);
            let assign52790_e68130: f64 = (assign52790_e68128 * locals.var_inv_gf2__blk1324);
            let assign52790_e68131: f64 = (2.0 * assign52790_e68130);
            let assign52790_e68132: f64 = (assign52790_e68124 + assign52790_e68131);
            (locals.var_km0__blk1420, locals.var_km0__blk1420_dn5, locals.var_km0__blk1420_dn6, locals.var_km0__blk1420_dn7, locals.var_km0__blk1420_dn8, ) = (assign52790_e68132, ((-locals.var_em__blk1405_dn5) + (2.0 * ((((locals.var_xgm__blk1409_dn5 * locals.var_eta_p__blk1410) + (locals.var_xgm__blk1409 * locals.var_eta_p__blk1410_dn5)) * locals.var_inv_gf2__blk1324) + (assign52790_e68128 * locals.var_inv_gf2__blk1324_dn5)))), ((-locals.var_em__blk1405_dn6) + (2.0 * ((((locals.var_xgm__blk1409_dn6 * locals.var_eta_p__blk1410) + (locals.var_xgm__blk1409 * locals.var_eta_p__blk1410_dn6)) * locals.var_inv_gf2__blk1324) + (assign52790_e68128 * locals.var_inv_gf2__blk1324_dn6)))), ((-locals.var_em__blk1405_dn7) + (2.0 * ((((locals.var_xgm__blk1409_dn7 * locals.var_eta_p__blk1410) + (locals.var_xgm__blk1409 * locals.var_eta_p__blk1410_dn7)) * locals.var_inv_gf2__blk1324) + (assign52790_e68128 * locals.var_inv_gf2__blk1324_dn7)))), ((-locals.var_em__blk1405_dn8) + (2.0 * ((((locals.var_xgm__blk1409_dn8 * locals.var_eta_p__blk1410) + (locals.var_xgm__blk1409 * locals.var_eta_p__blk1410_dn8)) * locals.var_inv_gf2__blk1324) + (assign52790_e68128 * locals.var_inv_gf2__blk1324_dn8)))), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign52800_e68145: f64 = (locals.var_x_ds__blk1394 * locals.var_km__blk1419);
            let assign52800_e68148: f64 = (locals.var_d0__blk1413 + locals.var_d_bar__blk1406);
            let assign52800_e68149: f64 = (assign52800_e68145 * assign52800_e68148);
            let assign52800_e68153: f64 = (locals.var_km__blk1419 * locals.var_d_bar__blk1406);
            let assign52800_e68154: f64 = (locals.var_km0__blk1420 + assign52800_e68153);
            let assign52800_e68155: f64 = (assign52800_e68149 / assign52800_e68154);
            (locals.var_x_ds__blk1394, locals.var_x_ds__blk1394_dn5, locals.var_x_ds__blk1394_dn6, locals.var_x_ds__blk1394_dn7, locals.var_x_ds__blk1394_dn8, ) = (assign52800_e68155, (((((((locals.var_x_ds__blk1394_dn5 * locals.var_km__blk1419) + (locals.var_x_ds__blk1394 * locals.var_km__blk1419_dn5)) * assign52800_e68148) + (assign52800_e68145 * (locals.var_d0__blk1413_dn5 + locals.var_d_bar__blk1406_dn5))) * assign52800_e68154) - (assign52800_e68149 * (locals.var_km0__blk1420_dn5 + ((locals.var_km__blk1419_dn5 * locals.var_d_bar__blk1406) + (locals.var_km__blk1419 * locals.var_d_bar__blk1406_dn5))))) / (assign52800_e68154 * assign52800_e68154)), (((((((locals.var_x_ds__blk1394_dn6 * locals.var_km__blk1419) + (locals.var_x_ds__blk1394 * locals.var_km__blk1419_dn6)) * assign52800_e68148) + (assign52800_e68145 * (locals.var_d0__blk1413_dn6 + locals.var_d_bar__blk1406_dn6))) * assign52800_e68154) - (assign52800_e68149 * (locals.var_km0__blk1420_dn6 + ((locals.var_km__blk1419_dn6 * locals.var_d_bar__blk1406) + (locals.var_km__blk1419 * locals.var_d_bar__blk1406_dn6))))) / (assign52800_e68154 * assign52800_e68154)), (((((((locals.var_x_ds__blk1394_dn7 * locals.var_km__blk1419) + (locals.var_x_ds__blk1394 * locals.var_km__blk1419_dn7)) * assign52800_e68148) + (assign52800_e68145 * (locals.var_d0__blk1413_dn7 + locals.var_d_bar__blk1406_dn7))) * assign52800_e68154) - (assign52800_e68149 * (locals.var_km0__blk1420_dn7 + ((locals.var_km__blk1419_dn7 * locals.var_d_bar__blk1406) + (locals.var_km__blk1419 * locals.var_d_bar__blk1406_dn7))))) / (assign52800_e68154 * assign52800_e68154)), (((((((locals.var_x_ds__blk1394_dn8 * locals.var_km__blk1419) + (locals.var_x_ds__blk1394 * locals.var_km__blk1419_dn8)) * assign52800_e68148) + (assign52800_e68145 * (locals.var_d0__blk1413_dn8 + locals.var_d_bar__blk1406_dn8))) * assign52800_e68154) - (assign52800_e68149 * (locals.var_km0__blk1420_dn8 + ((locals.var_km__blk1419_dn8 * locals.var_d_bar__blk1406) + (locals.var_km__blk1419 * locals.var_d_bar__blk1406_dn8))))) / (assign52800_e68154 * assign52800_e68154)), );
        }

        if ((((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) && (locals.var_guard1502 != 0.0)) {
            let assign52810_e68168: f64 = (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322);
            (locals.var_dps__blk1397, locals.var_dps__blk1397_dn5, locals.var_dps__blk1397_dn6, locals.var_dps__blk1397_dn7, locals.var_dps__blk1397_dn8, ) = (assign52810_e68168, ((locals.var_x_ds__blk1394_dn5 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn5)), ((locals.var_x_ds__blk1394_dn6 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn6)), ((locals.var_x_ds__blk1394_dn7 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn7)), ((locals.var_x_ds__blk1394_dn8 * locals.var_phit1__blk1322) + (locals.var_x_ds__blk1394 * locals.var_phit1__blk1322_dn8)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) {
            let assign52820_e68178: f64 = (locals.var_pm__blk1408).sqrt();
            (locals.var_sqm__blk1411, locals.var_sqm__blk1411_dn5, locals.var_sqm__blk1411_dn6, locals.var_sqm__blk1411_dn7, locals.var_sqm__blk1411_dn8, ) = (assign52820_e68178, (locals.var_pm__blk1408_dn5 / (2.0 * assign52820_e68178)), (locals.var_pm__blk1408_dn6 / (2.0 * assign52820_e68178)), (locals.var_pm__blk1408_dn7 / (2.0 * assign52820_e68178)), (locals.var_pm__blk1408_dn8 / (2.0 * assign52820_e68178)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1500 == 0.0)) {
            let assign52830_e68192: f64 = (1.0 - locals.var_em__blk1405);
            let assign52830_e68193: f64 = (locals.var_gf__blk1307 * assign52830_e68192);
            let assign52830_e68195: f64 = (assign52830_e68193 / locals.var_sqm__blk1411);
            let assign52830_e68196: f64 = (0.5 * assign52830_e68195);
            let assign52830_e68197: f64 = (locals.var_eta_p__blk1410 + assign52830_e68196);
            (locals.var_alpha__blk1412, locals.var_alpha__blk1412_dn5, locals.var_alpha__blk1412_dn6, locals.var_alpha__blk1412_dn7, locals.var_alpha__blk1412_dn8, ) = (assign52830_e68197, (locals.var_eta_p__blk1410_dn5 + (0.5 * (((((locals.var_gf__blk1307_dn5 * assign52830_e68192) + (locals.var_gf__blk1307 * (-locals.var_em__blk1405_dn5))) * locals.var_sqm__blk1411) - (assign52830_e68193 * locals.var_sqm__blk1411_dn5)) / (locals.var_sqm__blk1411 * locals.var_sqm__blk1411)))), (locals.var_eta_p__blk1410_dn6 + (0.5 * (((((locals.var_gf__blk1307_dn6 * assign52830_e68192) + (locals.var_gf__blk1307 * (-locals.var_em__blk1405_dn6))) * locals.var_sqm__blk1411) - (assign52830_e68193 * locals.var_sqm__blk1411_dn6)) / (locals.var_sqm__blk1411 * locals.var_sqm__blk1411)))), (locals.var_eta_p__blk1410_dn7 + (0.5 * (((((locals.var_gf__blk1307_dn7 * assign52830_e68192) + (locals.var_gf__blk1307 * (-locals.var_em__blk1405_dn7))) * locals.var_sqm__blk1411) - (assign52830_e68193 * locals.var_sqm__blk1411_dn7)) / (locals.var_sqm__blk1411 * locals.var_sqm__blk1411)))), (locals.var_eta_p__blk1410_dn8 + (0.5 * (((((locals.var_gf__blk1307_dn8 * assign52830_e68192) + (locals.var_gf__blk1307 * (-locals.var_em__blk1405_dn8))) * locals.var_sqm__blk1411) - (assign52830_e68193 * locals.var_sqm__blk1411_dn8)) / (locals.var_sqm__blk1411 * locals.var_sqm__blk1411)))), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign52840_e68206: f64 = (locals.var_gf2__blk1308 * locals.var_dm__blk1407);
            let assign52840_e68210: f64 = (locals.var_gf__blk1307 * locals.var_sqm__blk1411);
            let assign52840_e68211: f64 = (locals.var_xgm__blk1409 + assign52840_e68210);
            let assign52840_e68212: f64 = (assign52840_e68206 / assign52840_e68211);
            let assign52840_e68213: f64 = (locals.var_phit1__blk1322 * assign52840_e68212);
            (locals.var_qim__blk1421, locals.var_qim__blk1421_dn5, locals.var_qim__blk1421_dn6, locals.var_qim__blk1421_dn7, locals.var_qim__blk1421_dn8, ) = (assign52840_e68213, ((locals.var_phit1__blk1322_dn5 * assign52840_e68212) + (locals.var_phit1__blk1322 * (((((locals.var_gf2__blk1308_dn5 * locals.var_dm__blk1407) + (locals.var_gf2__blk1308 * locals.var_dm__blk1407_dn5)) * assign52840_e68211) - (assign52840_e68206 * (locals.var_xgm__blk1409_dn5 + ((locals.var_gf__blk1307_dn5 * locals.var_sqm__blk1411) + (locals.var_gf__blk1307 * locals.var_sqm__blk1411_dn5))))) / (assign52840_e68211 * assign52840_e68211)))), ((locals.var_phit1__blk1322_dn6 * assign52840_e68212) + (locals.var_phit1__blk1322 * (((((locals.var_gf2__blk1308_dn6 * locals.var_dm__blk1407) + (locals.var_gf2__blk1308 * locals.var_dm__blk1407_dn6)) * assign52840_e68211) - (assign52840_e68206 * (locals.var_xgm__blk1409_dn6 + ((locals.var_gf__blk1307_dn6 * locals.var_sqm__blk1411) + (locals.var_gf__blk1307 * locals.var_sqm__blk1411_dn6))))) / (assign52840_e68211 * assign52840_e68211)))), ((locals.var_phit1__blk1322_dn7 * assign52840_e68212) + (locals.var_phit1__blk1322 * (((((locals.var_gf2__blk1308_dn7 * locals.var_dm__blk1407) + (locals.var_gf2__blk1308 * locals.var_dm__blk1407_dn7)) * assign52840_e68211) - (assign52840_e68206 * (locals.var_xgm__blk1409_dn7 + ((locals.var_gf__blk1307_dn7 * locals.var_sqm__blk1411) + (locals.var_gf__blk1307 * locals.var_sqm__blk1411_dn7))))) / (assign52840_e68211 * assign52840_e68211)))), ((locals.var_phit1__blk1322_dn8 * assign52840_e68212) + (locals.var_phit1__blk1322 * (((((locals.var_gf2__blk1308_dn8 * locals.var_dm__blk1407) + (locals.var_gf2__blk1308 * locals.var_dm__blk1407_dn8)) * assign52840_e68211) - (assign52840_e68206 * (locals.var_xgm__blk1409_dn8 + ((locals.var_gf__blk1307_dn8 * locals.var_sqm__blk1411) + (locals.var_gf__blk1307 * locals.var_sqm__blk1411_dn8))))) / (assign52840_e68211 * assign52840_e68211)))), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign52850_e68222: f64 = (locals.var_phit1__blk1322 * locals.var_alpha__blk1412);
            let assign52850_e68223: f64 = (locals.var_qim__blk1421 + assign52850_e68222);
            (locals.var_qim1__blk1422, locals.var_qim1__blk1422_dn5, locals.var_qim1__blk1422_dn6, locals.var_qim1__blk1422_dn7, locals.var_qim1__blk1422_dn8, ) = (assign52850_e68223, (locals.var_qim__blk1421_dn5 + ((locals.var_phit1__blk1322_dn5 * locals.var_alpha__blk1412) + (locals.var_phit1__blk1322 * locals.var_alpha__blk1412_dn5))), (locals.var_qim__blk1421_dn6 + ((locals.var_phit1__blk1322_dn6 * locals.var_alpha__blk1412) + (locals.var_phit1__blk1322 * locals.var_alpha__blk1412_dn6))), (locals.var_qim__blk1421_dn7 + ((locals.var_phit1__blk1322_dn7 * locals.var_alpha__blk1412) + (locals.var_phit1__blk1322 * locals.var_alpha__blk1412_dn7))), (locals.var_qim__blk1421_dn8 + ((locals.var_phit1__blk1322_dn8 * locals.var_alpha__blk1412) + (locals.var_phit1__blk1322 * locals.var_alpha__blk1412_dn8))), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign52860_e68231: f64 = (locals.var_sqm__blk1411 * locals.var_gf__blk1307);
            let assign52860_e68233: f64 = (assign52860_e68231 * locals.var_phit1__blk1322);
            (locals.var_qbm__blk1423, locals.var_qbm__blk1423_dn5, locals.var_qbm__blk1423_dn6, locals.var_qbm__blk1423_dn7, locals.var_qbm__blk1423_dn8, ) = (assign52860_e68233, ((((locals.var_sqm__blk1411_dn5 * locals.var_gf__blk1307) + (locals.var_sqm__blk1411 * locals.var_gf__blk1307_dn5)) * locals.var_phit1__blk1322) + (assign52860_e68231 * locals.var_phit1__blk1322_dn5)), ((((locals.var_sqm__blk1411_dn6 * locals.var_gf__blk1307) + (locals.var_sqm__blk1411 * locals.var_gf__blk1307_dn6)) * locals.var_phit1__blk1322) + (assign52860_e68231 * locals.var_phit1__blk1322_dn6)), ((((locals.var_sqm__blk1411_dn7 * locals.var_gf__blk1307) + (locals.var_sqm__blk1411 * locals.var_gf__blk1307_dn7)) * locals.var_phit1__blk1322) + (assign52860_e68231 * locals.var_phit1__blk1322_dn7)), ((((locals.var_sqm__blk1411_dn8 * locals.var_gf__blk1307) + (locals.var_sqm__blk1411 * locals.var_gf__blk1307_dn8)) * locals.var_phit1__blk1322) + (assign52860_e68231 * locals.var_phit1__blk1322_dn8)), );
        }

        let assign52870_e68238: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1503 = assign52870_e68238;

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1503 != 0.0)) {
            let assign52880_e68247: f64 = (locals.var_rsg_i * locals.var_qim__blk1421);
            let assign52880_e68248: f64 = (1.0 - assign52880_e68247);
            (locals.var_rhog__blk1362, locals.var_rhog__blk1362_dn5, locals.var_rhog__blk1362_dn6, locals.var_rhog__blk1362_dn7, locals.var_rhog__blk1362_dn8, ) = (assign52880_e68248, (-(locals.var_rsg_i * locals.var_qim__blk1421_dn5)), (-(locals.var_rsg_i * locals.var_qim__blk1421_dn6)), (-(locals.var_rsg_i * locals.var_qim__blk1421_dn7)), (-(locals.var_rsg_i * locals.var_qim__blk1421_dn8)), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1503 == 0.0)) {
            let assign52890_e68261: f64 = (locals.var_rsg_i * locals.var_qim__blk1421);
            let assign52890_e68262: f64 = (1.0 + assign52890_e68261);
            let assign52890_e68263: f64 = (1.0 / assign52890_e68262);
            (locals.var_rhog__blk1362, locals.var_rhog__blk1362_dn5, locals.var_rhog__blk1362_dn6, locals.var_rhog__blk1362_dn7, locals.var_rhog__blk1362_dn8, ) = (assign52890_e68263, (-((locals.var_rsg_i * locals.var_qim__blk1421_dn5) / (assign52890_e68262 * assign52890_e68262))), (-((locals.var_rsg_i * locals.var_qim__blk1421_dn6) / (assign52890_e68262 * assign52890_e68262))), (-((locals.var_rsg_i * locals.var_qim__blk1421_dn7) / (assign52890_e68262 * assign52890_e68262))), (-((locals.var_rsg_i * locals.var_qim__blk1421_dn8) / (assign52890_e68262 * assign52890_e68262))), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign52900_e68271: f64 = (locals.var_ther_i * locals.var_rhob__blk1361);
            let assign52900_e68273: f64 = (assign52900_e68271 * locals.var_rhog__blk1362);
            let assign52900_e68275: f64 = (assign52900_e68273 * locals.var_qim__blk1421);
            (locals.var_gr__blk1363, locals.var_gr__blk1363_dn5, locals.var_gr__blk1363_dn6, locals.var_gr__blk1363_dn7, locals.var_gr__blk1363_dn8, ) = (assign52900_e68275, (((((locals.var_ther_i * locals.var_rhob__blk1361_dn5) * locals.var_rhog__blk1362) + (assign52900_e68271 * locals.var_rhog__blk1362_dn5)) * locals.var_qim__blk1421) + (assign52900_e68273 * locals.var_qim__blk1421_dn5)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn6) * locals.var_rhog__blk1362) + (assign52900_e68271 * locals.var_rhog__blk1362_dn6)) * locals.var_qim__blk1421) + (assign52900_e68273 * locals.var_qim__blk1421_dn6)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn7) * locals.var_rhog__blk1362) + (assign52900_e68271 * locals.var_rhog__blk1362_dn7)) * locals.var_qim__blk1421) + (assign52900_e68273 * locals.var_qim__blk1421_dn7)), (((((locals.var_ther_i * locals.var_rhob__blk1361_dn8) * locals.var_rhog__blk1362) + (assign52900_e68271 * locals.var_rhog__blk1362_dn8)) * locals.var_qim__blk1421) + (assign52900_e68273 * locals.var_qim__blk1421_dn8)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign52910_e68284: f64 = (locals.var_eta_mu * locals.var_qim__blk1421);
            let assign52910_e68285: f64 = (locals.var_qbm__blk1423 + assign52910_e68284);
            (locals.var_qeff__blk1424, locals.var_qeff__blk1424_dn5, locals.var_qeff__blk1424_dn6, locals.var_qeff__blk1424_dn7, locals.var_qeff__blk1424_dn8, ) = (assign52910_e68285, (locals.var_qbm__blk1423_dn5 + (locals.var_eta_mu * locals.var_qim__blk1421_dn5)), (locals.var_qbm__blk1423_dn6 + (locals.var_eta_mu * locals.var_qim__blk1421_dn6)), (locals.var_qbm__blk1423_dn7 + (locals.var_eta_mu * locals.var_qim__blk1421_dn7)), (locals.var_qbm__blk1423_dn8 + (locals.var_eta_mu * locals.var_qim__blk1421_dn8)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign52920_e68294: f64 = (locals.var_eta_mu1 * locals.var_qim__blk1421);
            let assign52920_e68295: f64 = (locals.var_qbm__blk1423 + assign52920_e68294);
            (locals.var_qeff1__blk1425, locals.var_qeff1__blk1425_dn5, locals.var_qeff1__blk1425_dn6, locals.var_qeff1__blk1425_dn7, locals.var_qeff1__blk1425_dn8, ) = (assign52920_e68295, (locals.var_qbm__blk1423_dn5 + (locals.var_eta_mu1 * locals.var_qim__blk1421_dn5)), (locals.var_qbm__blk1423_dn6 + (locals.var_eta_mu1 * locals.var_qim__blk1421_dn6)), (locals.var_qbm__blk1423_dn7 + (locals.var_eta_mu1 * locals.var_qim__blk1421_dn7)), (locals.var_qbm__blk1423_dn8 + (locals.var_eta_mu1 * locals.var_qim__blk1421_dn8)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign52930_e68303: f64 = (locals.var_e_eff0 * locals.var_qeff__blk1424);
            (locals.var_eeffm__blk1426, locals.var_eeffm__blk1426_dn5, locals.var_eeffm__blk1426_dn6, locals.var_eeffm__blk1426_dn7, locals.var_eeffm__blk1426_dn8, ) = (assign52930_e68303, (locals.var_e_eff0 * locals.var_qeff__blk1424_dn5), (locals.var_e_eff0 * locals.var_qeff__blk1424_dn6), (locals.var_e_eff0 * locals.var_qeff__blk1424_dn7), (locals.var_e_eff0 * locals.var_qeff__blk1424_dn8), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign52940_e68312: f64 = (locals.var_pm__blk1408 + locals.var_dm__blk1407);
            let assign52940_e68314: f64 = (assign52940_e68312 + 1e-14);
            let assign52940_e68315: f64 = (locals.var_pm__blk1408 / assign52940_e68314);
            let assign52940_e68316: f64 = (assign52940_e68315).ln();
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign52940_e68316, ((((locals.var_pm__blk1408_dn5 * assign52940_e68314) - (locals.var_pm__blk1408 * (locals.var_pm__blk1408_dn5 + locals.var_dm__blk1407_dn5))) / (assign52940_e68314 * assign52940_e68314)) / assign52940_e68315), ((((locals.var_pm__blk1408_dn6 * assign52940_e68314) - (locals.var_pm__blk1408 * (locals.var_pm__blk1408_dn6 + locals.var_dm__blk1407_dn6))) / (assign52940_e68314 * assign52940_e68314)) / assign52940_e68315), ((((locals.var_pm__blk1408_dn7 * assign52940_e68314) - (locals.var_pm__blk1408 * (locals.var_pm__blk1408_dn7 + locals.var_dm__blk1407_dn7))) / (assign52940_e68314 * assign52940_e68314)) / assign52940_e68315), ((((locals.var_pm__blk1408_dn8 * assign52940_e68314) - (locals.var_pm__blk1408 * (locals.var_pm__blk1408_dn8 + locals.var_dm__blk1407_dn8))) / (assign52940_e68314 * assign52940_e68314)) / assign52940_e68315), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign52950_e68324: f64 = (locals.var_eeffm__blk1426 * locals.var_mue_t);
            let assign52950_e68326: f64 = (assign52950_e68324).powf(locals.var_themu_t);
            let assign52950_e68330: f64 = (0.5 * locals.var_thecs_t);
            let assign52950_e68332: f64 = (assign52950_e68330 * locals.var_temp1);
            let assign52950_e68333: f64 = (assign52950_e68332).exp();
            let assign52950_e68334: f64 = (locals.var_cs_t * assign52950_e68333);
            let assign52950_e68335: f64 = (assign52950_e68326 + assign52950_e68334);
            (locals.var_mutmp__blk1365, locals.var_mutmp__blk1365_dn5, locals.var_mutmp__blk1365_dn6, locals.var_mutmp__blk1365_dn7, locals.var_mutmp__blk1365_dn8, ) = (assign52950_e68335, (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign52950_e68324).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm__blk1426_dn5 * locals.var_mue_t))) } } else { (assign52950_e68326 * (locals.var_themu_t * ((locals.var_eeffm__blk1426_dn5 * locals.var_mue_t) / assign52950_e68324))) } + (locals.var_cs_t * (assign52950_e68333 * (assign52950_e68330 * locals.var_temp1_dn5)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign52950_e68324).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm__blk1426_dn6 * locals.var_mue_t))) } } else { (assign52950_e68326 * (locals.var_themu_t * ((locals.var_eeffm__blk1426_dn6 * locals.var_mue_t) / assign52950_e68324))) } + (locals.var_cs_t * (assign52950_e68333 * (assign52950_e68330 * locals.var_temp1_dn6)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign52950_e68324).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm__blk1426_dn7 * locals.var_mue_t))) } } else { (assign52950_e68326 * (locals.var_themu_t * ((locals.var_eeffm__blk1426_dn7 * locals.var_mue_t) / assign52950_e68324))) } + (locals.var_cs_t * (assign52950_e68333 * (assign52950_e68330 * locals.var_temp1_dn7)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign52950_e68324).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm__blk1426_dn8 * locals.var_mue_t))) } } else { (assign52950_e68326 * (locals.var_themu_t * ((locals.var_eeffm__blk1426_dn8 * locals.var_mue_t) / assign52950_e68324))) } + (locals.var_cs_t * (assign52950_e68333 * (assign52950_e68330 * locals.var_temp1_dn8)))), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign52960_e68343: f64 = (1.0 + locals.var_mutmp__blk1365);
            let assign52960_e68345: f64 = (assign52960_e68343 + locals.var_gr__blk1363);
            let assign52960_e68347: f64 = (assign52960_e68345 * locals.var_rxcor__blk1357);
            (locals.var_gmob__blk1427, locals.var_gmob__blk1427_dn5, locals.var_gmob__blk1427_dn6, locals.var_gmob__blk1427_dn7, locals.var_gmob__blk1427_dn8, ) = (assign52960_e68347, (((locals.var_mutmp__blk1365_dn5 + locals.var_gr__blk1363_dn5) * locals.var_rxcor__blk1357) + (assign52960_e68345 * locals.var_rxcor__blk1357_dn5)), (((locals.var_mutmp__blk1365_dn6 + locals.var_gr__blk1363_dn6) * locals.var_rxcor__blk1357) + (assign52960_e68345 * locals.var_rxcor__blk1357_dn6)), (((locals.var_mutmp__blk1365_dn7 + locals.var_gr__blk1363_dn7) * locals.var_rxcor__blk1357) + (assign52960_e68345 * locals.var_rxcor__blk1357_dn7)), (((locals.var_mutmp__blk1365_dn8 + locals.var_gr__blk1363_dn8) * locals.var_rxcor__blk1357) + (assign52960_e68345 * locals.var_rxcor__blk1357_dn8)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign52970_e68356: f64 = (locals.var_v_ds - locals.var_dps__blk1397);
            let assign52970_e68358: f64 = (assign52970_e68356 * locals.var_inv_vp);
            let assign52970_e68359: f64 = (1.0 + assign52970_e68358);
            let assign52970_e68363: f64 = (locals.var_vdse__blk1388 - locals.var_dps__blk1397);
            let assign52970_e68365: f64 = (assign52970_e68363 * locals.var_inv_vp);
            let assign52970_e68366: f64 = (1.0 + assign52970_e68365);
            let assign52970_e68367: f64 = (assign52970_e68359 / assign52970_e68366);
            let assign52970_e68368: f64 = (assign52970_e68367).ln();
            (locals.var_s1__blk1428, locals.var_s1__blk1428_dn5, locals.var_s1__blk1428_dn6, locals.var_s1__blk1428_dn7, locals.var_s1__blk1428_dn8, ) = (assign52970_e68368, ((((((-locals.var_dps__blk1397_dn5) * locals.var_inv_vp) * assign52970_e68366) - (assign52970_e68359 * ((locals.var_vdse__blk1388_dn5 - locals.var_dps__blk1397_dn5) * locals.var_inv_vp))) / (assign52970_e68366 * assign52970_e68366)) / assign52970_e68367), ((((((locals.var_v_ds_dn6 - locals.var_dps__blk1397_dn6) * locals.var_inv_vp) * assign52970_e68366) - (assign52970_e68359 * ((locals.var_vdse__blk1388_dn6 - locals.var_dps__blk1397_dn6) * locals.var_inv_vp))) / (assign52970_e68366 * assign52970_e68366)) / assign52970_e68367), ((((((locals.var_v_ds_dn7 - locals.var_dps__blk1397_dn7) * locals.var_inv_vp) * assign52970_e68366) - (assign52970_e68359 * ((locals.var_vdse__blk1388_dn7 - locals.var_dps__blk1397_dn7) * locals.var_inv_vp))) / (assign52970_e68366 * assign52970_e68366)) / assign52970_e68367), ((((((-locals.var_dps__blk1397_dn8) * locals.var_inv_vp) * assign52970_e68366) - (assign52970_e68359 * ((locals.var_vdse__blk1388_dn8 - locals.var_dps__blk1397_dn8) * locals.var_inv_vp))) / (assign52970_e68366 * assign52970_e68366)) / assign52970_e68367), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign52980_e68376: f64 = (locals.var_qim__blk1421 * locals.var_xitsb__blk1367);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign52980_e68376, ((locals.var_qim__blk1421_dn5 * locals.var_xitsb__blk1367) + (locals.var_qim__blk1421 * locals.var_xitsb__blk1367_dn5)), ((locals.var_qim__blk1421_dn6 * locals.var_xitsb__blk1367) + (locals.var_qim__blk1421 * locals.var_xitsb__blk1367_dn6)), ((locals.var_qim__blk1421_dn7 * locals.var_xitsb__blk1367) + (locals.var_qim__blk1421 * locals.var_xitsb__blk1367_dn7)), ((locals.var_qim__blk1421_dn8 * locals.var_xitsb__blk1367) + (locals.var_qim__blk1421 * locals.var_xitsb__blk1367_dn8)), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign52990_e68385: f64 = (locals.var_thesatt_i + locals.var_temp2);
            let assign52990_e68386: f64 = (locals.var_temp2 / assign52990_e68385);
            (locals.var_wsat__blk1368, locals.var_wsat__blk1368_dn5, locals.var_wsat__blk1368_dn6, locals.var_wsat__blk1368_dn7, locals.var_wsat__blk1368_dn8, ) = (assign52990_e68386, (((locals.var_temp2_dn5 * assign52990_e68385) - (locals.var_temp2 * locals.var_temp2_dn5)) / (assign52990_e68385 * assign52990_e68385)), (((locals.var_temp2_dn6 * assign52990_e68385) - (locals.var_temp2 * locals.var_temp2_dn6)) / (assign52990_e68385 * assign52990_e68385)), (((locals.var_temp2_dn7 * assign52990_e68385) - (locals.var_temp2 * locals.var_temp2_dn7)) / (assign52990_e68385 * assign52990_e68385)), (((locals.var_temp2_dn8 * assign52990_e68385) - (locals.var_temp2 * locals.var_temp2_dn8)) / (assign52990_e68385 * assign52990_e68385)), );
        }

        let assign53000_e68391: f64 = if locals.var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1504 = assign53000_e68391;

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1504 != 0.0)) {
            let assign53010_e68401: f64 = (locals.var_thesatg_i * locals.var_wsat__blk1368);
            let assign53010_e68402: f64 = (1.0 - assign53010_e68401);
            let assign53010_e68403: f64 = (1.0 / assign53010_e68402);
            (locals.var_factheta__blk1369, locals.var_factheta__blk1369_dn5, locals.var_factheta__blk1369_dn6, locals.var_factheta__blk1369_dn7, locals.var_factheta__blk1369_dn8, ) = (assign53010_e68403, (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn5)) / (assign53010_e68402 * assign53010_e68402))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn6)) / (assign53010_e68402 * assign53010_e68402))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn7)) / (assign53010_e68402 * assign53010_e68402))), (-((-(locals.var_thesatg_i * locals.var_wsat__blk1368_dn8)) / (assign53010_e68402 * assign53010_e68402))), );
        }

        if (((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1504 == 0.0)) {
            let assign53020_e68415: f64 = (locals.var_thesatg_i * locals.var_wsat__blk1368);
            let assign53020_e68416: f64 = (1.0 + assign53020_e68415);
            (locals.var_factheta__blk1369, locals.var_factheta__blk1369_dn5, locals.var_factheta__blk1369_dn6, locals.var_factheta__blk1369_dn7, locals.var_factheta__blk1369_dn8, ) = (assign53020_e68416, (locals.var_thesatg_i * locals.var_wsat__blk1368_dn5), (locals.var_thesatg_i * locals.var_wsat__blk1368_dn6), (locals.var_thesatg_i * locals.var_wsat__blk1368_dn7), (locals.var_thesatg_i * locals.var_wsat__blk1368_dn8), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign53030_e68424: f64 = (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369);
            (locals.var_thesateff__blk1430, locals.var_thesateff__blk1430_dn5, locals.var_thesateff__blk1430_dn6, locals.var_thesateff__blk1430_dn7, locals.var_thesateff__blk1430_dn8, ) = (assign53030_e68424, (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn5), (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn6), (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn7), (locals.var_thesatloc__blk1302 * locals.var_factheta__blk1369_dn8), );
        }

        if ((locals.var_guard1456 != 0.0) && (locals.var_guard1484 != 0.0)) {
            let assign53040_e68432: f64 = (locals.var_xgm__blk1409 * locals.var_phit1__blk1322);
            (locals.var_voxm__blk1429, locals.var_voxm__blk1429_dn5, locals.var_voxm__blk1429_dn6, locals.var_voxm__blk1429_dn7, locals.var_voxm__blk1429_dn8, ) = (assign53040_e68432, ((locals.var_xgm__blk1409_dn5 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn5)), ((locals.var_xgm__blk1409_dn6 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn6)), ((locals.var_xgm__blk1409_dn7 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn7)), ((locals.var_xgm__blk1409_dn8 * locals.var_phit1__blk1322) + (locals.var_xgm__blk1409 * locals.var_phit1__blk1322_dn8)), );
        }

        if (locals.var_guard1456 != 0.0) {
            (locals.var_vgb1_ac, locals.var_vgb1_ac_dn5, locals.var_vgb1_ac_dn6, locals.var_vgb1_ac_dn7, locals.var_vgb1_ac_dn8, ) = (locals.var_vgb1__blk1304, locals.var_vgb1__blk1304_dn5, locals.var_vgb1__blk1304_dn6, locals.var_vgb1__blk1304_dn7, locals.var_vgb1__blk1304_dn8, );
            (locals.var_phit1_ac, locals.var_phit1_ac_dn5, locals.var_phit1_ac_dn6, locals.var_phit1_ac_dn7, locals.var_phit1_ac_dn8, ) = (locals.var_phit1__blk1322, locals.var_phit1__blk1322_dn5, locals.var_phit1__blk1322_dn6, locals.var_phit1__blk1322_dn7, locals.var_phit1__blk1322_dn8, );
            (locals.var_gf_ac, locals.var_gf_ac_dn5, locals.var_gf_ac_dn6, locals.var_gf_ac_dn7, locals.var_gf_ac_dn8, ) = (locals.var_gf__blk1307, locals.var_gf__blk1307_dn5, locals.var_gf__blk1307_dn6, locals.var_gf__blk1307_dn7, locals.var_gf__blk1307_dn8, );
            locals.var_xg_ac = locals.var_xg__blk1326;
            (locals.var_xno_s_ac, locals.var_xno_s_ac_dn5, locals.var_xno_s_ac_dn6, locals.var_xno_s_ac_dn7, locals.var_xno_s_ac_dn8, ) = (locals.var_xno_s__blk1331, locals.var_xno_s__blk1331_dn5, locals.var_xno_s__blk1331_dn6, locals.var_xno_s__blk1331_dn7, locals.var_xno_s__blk1331_dn8, );
            (locals.var_qbs_ac, locals.var_qbs_ac_dn5, locals.var_qbs_ac_dn6, locals.var_qbs_ac_dn7, locals.var_qbs_ac_dn8, ) = (locals.var_qbs__blk1360, locals.var_qbs__blk1360_dn5, locals.var_qbs__blk1360_dn6, locals.var_qbs__blk1360_dn7, locals.var_qbs__blk1360_dn8, );
            (locals.var_dps_ac, locals.var_dps_ac_dn5, locals.var_dps_ac_dn6, locals.var_dps_ac_dn7, locals.var_dps_ac_dn8, ) = (locals.var_dps__blk1397, locals.var_dps__blk1397_dn5, locals.var_dps__blk1397_dn6, locals.var_dps__blk1397_dn7, locals.var_dps__blk1397_dn8, );
            (locals.var_qbd_ac, locals.var_qbd_ac_dn5, locals.var_qbd_ac_dn6, locals.var_qbd_ac_dn7, locals.var_qbd_ac_dn8, ) = (locals.var_qbd__blk1403, locals.var_qbd__blk1403_dn5, locals.var_qbd__blk1403_dn6, locals.var_qbd__blk1403_dn7, locals.var_qbd__blk1403_dn8, );
            (locals.var_eta_p_ac, locals.var_eta_p_ac_dn5, locals.var_eta_p_ac_dn6, locals.var_eta_p_ac_dn7, locals.var_eta_p_ac_dn8, ) = (locals.var_eta_p__blk1410, locals.var_eta_p__blk1410_dn5, locals.var_eta_p__blk1410_dn6, locals.var_eta_p__blk1410_dn7, locals.var_eta_p__blk1410_dn8, );
            (locals.var_alpha_ac, locals.var_alpha_ac_dn5, locals.var_alpha_ac_dn6, locals.var_alpha_ac_dn7, locals.var_alpha_ac_dn8, ) = (locals.var_alpha__blk1412, locals.var_alpha__blk1412_dn5, locals.var_alpha__blk1412_dn6, locals.var_alpha__blk1412_dn7, locals.var_alpha__blk1412_dn8, );
            (locals.var_qim_ac, locals.var_qim_ac_dn5, locals.var_qim_ac_dn6, locals.var_qim_ac_dn7, locals.var_qim_ac_dn8, ) = (locals.var_qim__blk1421, locals.var_qim__blk1421_dn5, locals.var_qim__blk1421_dn6, locals.var_qim__blk1421_dn7, locals.var_qim__blk1421_dn8, );
            (locals.var_qim1_ac, locals.var_qim1_ac_dn5, locals.var_qim1_ac_dn6, locals.var_qim1_ac_dn7, locals.var_qim1_ac_dn8, ) = (locals.var_qim1__blk1422, locals.var_qim1__blk1422_dn5, locals.var_qim1__blk1422_dn6, locals.var_qim1__blk1422_dn7, locals.var_qim1__blk1422_dn8, );
            (locals.var_qeff1_ac, locals.var_qeff1_ac_dn5, locals.var_qeff1_ac_dn6, locals.var_qeff1_ac_dn7, locals.var_qeff1_ac_dn8, ) = (locals.var_qeff1__blk1425, locals.var_qeff1__blk1425_dn5, locals.var_qeff1__blk1425_dn6, locals.var_qeff1__blk1425_dn7, locals.var_qeff1__blk1425_dn8, );
            (locals.var_gmob_ac, locals.var_gmob_ac_dn5, locals.var_gmob_ac_dn6, locals.var_gmob_ac_dn7, locals.var_gmob_ac_dn8, ) = (locals.var_gmob__blk1427, locals.var_gmob__blk1427_dn5, locals.var_gmob__blk1427_dn6, locals.var_gmob__blk1427_dn7, locals.var_gmob__blk1427_dn8, );
            (locals.var_s1_ac, locals.var_s1_ac_dn5, locals.var_s1_ac_dn6, locals.var_s1_ac_dn7, locals.var_s1_ac_dn8, ) = (locals.var_s1__blk1428, locals.var_s1__blk1428_dn5, locals.var_s1__blk1428_dn6, locals.var_s1__blk1428_dn7, locals.var_s1__blk1428_dn8, );
            (locals.var_thesateff_ac, locals.var_thesateff_ac_dn5, locals.var_thesateff_ac_dn6, locals.var_thesateff_ac_dn7, locals.var_thesateff_ac_dn8, ) = (locals.var_thesateff__blk1430, locals.var_thesateff__blk1430_dn5, locals.var_thesateff__blk1430_dn6, locals.var_thesateff__blk1430_dn7, locals.var_thesateff__blk1430_dn8, );
            (locals.var_voxm_ac, locals.var_voxm_ac_dn5, locals.var_voxm_ac_dn6, locals.var_voxm_ac_dn7, locals.var_voxm_ac_dn8, ) = (locals.var_voxm__blk1429, locals.var_voxm__blk1429_dn5, locals.var_voxm__blk1429_dn6, locals.var_voxm__blk1429_dn7, locals.var_voxm__blk1429_dn8, );
        }

        if (locals.var_guard1456 == 0.0) {
            locals.var_phib_ac = locals.var_phib_dc;
            (locals.var_vgb1_ac, locals.var_vgb1_ac_dn5, locals.var_vgb1_ac_dn6, locals.var_vgb1_ac_dn7, locals.var_vgb1_ac_dn8, ) = (locals.var_vgb1_dc, locals.var_vgb1_dc_dn5, locals.var_vgb1_dc_dn6, locals.var_vgb1_dc_dn7, locals.var_vgb1_dc_dn8, );
            (locals.var_phit1_ac, locals.var_phit1_ac_dn5, locals.var_phit1_ac_dn6, locals.var_phit1_ac_dn7, locals.var_phit1_ac_dn8, ) = (locals.var_phit1_dc, locals.var_phit1_dc_dn5, locals.var_phit1_dc_dn6, locals.var_phit1_dc_dn7, locals.var_phit1_dc_dn8, );
            (locals.var_gf_ac, locals.var_gf_ac_dn5, locals.var_gf_ac_dn6, locals.var_gf_ac_dn7, locals.var_gf_ac_dn8, ) = (locals.var_gf_dc, locals.var_gf_dc_dn5, locals.var_gf_dc_dn6, locals.var_gf_dc_dn7, locals.var_gf_dc_dn8, );
            locals.var_xg_ac = locals.var_xg_dc;
            (locals.var_xno_s_ac, locals.var_xno_s_ac_dn5, locals.var_xno_s_ac_dn6, locals.var_xno_s_ac_dn7, locals.var_xno_s_ac_dn8, ) = (locals.var_xno_s_dc, locals.var_xno_s_dc_dn5, locals.var_xno_s_dc_dn6, locals.var_xno_s_dc_dn7, locals.var_xno_s_dc_dn8, );
            (locals.var_qbs_ac, locals.var_qbs_ac_dn5, locals.var_qbs_ac_dn6, locals.var_qbs_ac_dn7, locals.var_qbs_ac_dn8, ) = (locals.var_qbs_dc, locals.var_qbs_dc_dn5, locals.var_qbs_dc_dn6, locals.var_qbs_dc_dn7, locals.var_qbs_dc_dn8, );
            (locals.var_dps_ac, locals.var_dps_ac_dn5, locals.var_dps_ac_dn6, locals.var_dps_ac_dn7, locals.var_dps_ac_dn8, ) = (locals.var_dps_dc, locals.var_dps_dc_dn5, locals.var_dps_dc_dn6, locals.var_dps_dc_dn7, locals.var_dps_dc_dn8, );
            (locals.var_qbd_ac, locals.var_qbd_ac_dn5, locals.var_qbd_ac_dn6, locals.var_qbd_ac_dn7, locals.var_qbd_ac_dn8, ) = (locals.var_qbd_dc, locals.var_qbd_dc_dn5, locals.var_qbd_dc_dn6, locals.var_qbd_dc_dn7, locals.var_qbd_dc_dn8, );
            (locals.var_eta_p_ac, locals.var_eta_p_ac_dn5, locals.var_eta_p_ac_dn6, locals.var_eta_p_ac_dn7, locals.var_eta_p_ac_dn8, ) = (locals.var_eta_p_dc, locals.var_eta_p_dc_dn5, locals.var_eta_p_dc_dn6, locals.var_eta_p_dc_dn7, locals.var_eta_p_dc_dn8, );
            (locals.var_alpha_ac, locals.var_alpha_ac_dn5, locals.var_alpha_ac_dn6, locals.var_alpha_ac_dn7, locals.var_alpha_ac_dn8, ) = (locals.var_alpha_dc, locals.var_alpha_dc_dn5, locals.var_alpha_dc_dn6, locals.var_alpha_dc_dn7, locals.var_alpha_dc_dn8, );
            (locals.var_qim_ac, locals.var_qim_ac_dn5, locals.var_qim_ac_dn6, locals.var_qim_ac_dn7, locals.var_qim_ac_dn8, ) = (locals.var_qim_dc, locals.var_qim_dc_dn5, locals.var_qim_dc_dn6, locals.var_qim_dc_dn7, locals.var_qim_dc_dn8, );
            (locals.var_qim1_ac, locals.var_qim1_ac_dn5, locals.var_qim1_ac_dn6, locals.var_qim1_ac_dn7, locals.var_qim1_ac_dn8, ) = (locals.var_qim1_dc, locals.var_qim1_dc_dn5, locals.var_qim1_dc_dn6, locals.var_qim1_dc_dn7, locals.var_qim1_dc_dn8, );
            (locals.var_qeff1_ac, locals.var_qeff1_ac_dn5, locals.var_qeff1_ac_dn6, locals.var_qeff1_ac_dn7, locals.var_qeff1_ac_dn8, ) = (locals.var_qeff1_dc, locals.var_qeff1_dc_dn5, locals.var_qeff1_dc_dn6, locals.var_qeff1_dc_dn7, locals.var_qeff1_dc_dn8, );
            (locals.var_gmob_ac, locals.var_gmob_ac_dn5, locals.var_gmob_ac_dn6, locals.var_gmob_ac_dn7, locals.var_gmob_ac_dn8, ) = (locals.var_gmob_dc, locals.var_gmob_dc_dn5, locals.var_gmob_dc_dn6, locals.var_gmob_dc_dn7, locals.var_gmob_dc_dn8, );
            (locals.var_s1_ac, locals.var_s1_ac_dn5, locals.var_s1_ac_dn6, locals.var_s1_ac_dn7, locals.var_s1_ac_dn8, ) = (locals.var_s1_dc, locals.var_s1_dc_dn5, locals.var_s1_dc_dn6, locals.var_s1_dc_dn7, locals.var_s1_dc_dn8, );
            (locals.var_thesateff_ac, locals.var_thesateff_ac_dn5, locals.var_thesateff_ac_dn6, locals.var_thesateff_ac_dn7, locals.var_thesateff_ac_dn8, ) = (locals.var_thesateff_dc, locals.var_thesateff_dc_dn5, locals.var_thesateff_dc_dn6, locals.var_thesateff_dc_dn7, locals.var_thesateff_dc_dn8, );
            (locals.var_voxm_ac, locals.var_voxm_ac_dn5, locals.var_voxm_ac_dn6, locals.var_voxm_ac_dn7, locals.var_voxm_ac_dn8, ) = (locals.var_voxm_dc, locals.var_voxm_dc_dn5, locals.var_voxm_dc_dn6, locals.var_voxm_dc_dn7, locals.var_voxm_dc_dn8, );
        }

        (locals.var_cox_qm, locals.var_cox_qm_dn5, locals.var_cox_qm_dn6, locals.var_cox_qm_dn7, locals.var_cox_qm_dn8, ) = (locals.var_cox_i, 0.0, 0.0, 0.0, 0.0, );

        let assign53420_e68601: f64 = if locals.var_qq > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1505 = assign53420_e68601;

    }

    pub(super) fn stamp_transient_block_26(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard1505 != 0.0) {
            let assign53430_e68608: f64 = (locals.var_qeff1_ac * locals.var_qeff1_ac);
            let assign53430_e68610: f64 = (assign53430_e68608 + locals.var_qlim2);
            let assign53430_e68612: f64 = (-1.0);
            let assign53430_e68614: f64 = (assign53430_e68612 * 0.16666666666666666);
            let assign53430_e68615: f64 = (assign53430_e68610).powf(assign53430_e68614);
            let assign53430_e68616: f64 = (locals.var_qq * assign53430_e68615);
            let assign53430_e68617: f64 = (1.0 + assign53430_e68616);
            let assign53430_e68618: f64 = (locals.var_cox_i / assign53430_e68617);
            (locals.var_cox_qm, locals.var_cox_qm_dn5, locals.var_cox_qm_dn6, locals.var_cox_qm_dn7, locals.var_cox_qm_dn8, ) = (assign53430_e68618, (-((locals.var_cox_i * (locals.var_qq * if 0.0 == 0.0 && ((assign53430_e68614) as f64).is_finite() && ((assign53430_e68614) as f64).fract() == 0.0 { if assign53430_e68614 == 0.0 { 0.0 } else { (assign53430_e68614 * ((assign53430_e68610).powf(assign53430_e68614 - 1.0) * ((locals.var_qeff1_ac_dn5 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn5)))) } } else { (assign53430_e68615 * (assign53430_e68614 * (((locals.var_qeff1_ac_dn5 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn5)) / assign53430_e68610))) })) / (assign53430_e68617 * assign53430_e68617))), (-((locals.var_cox_i * (locals.var_qq * if 0.0 == 0.0 && ((assign53430_e68614) as f64).is_finite() && ((assign53430_e68614) as f64).fract() == 0.0 { if assign53430_e68614 == 0.0 { 0.0 } else { (assign53430_e68614 * ((assign53430_e68610).powf(assign53430_e68614 - 1.0) * ((locals.var_qeff1_ac_dn6 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn6)))) } } else { (assign53430_e68615 * (assign53430_e68614 * (((locals.var_qeff1_ac_dn6 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn6)) / assign53430_e68610))) })) / (assign53430_e68617 * assign53430_e68617))), (-((locals.var_cox_i * (locals.var_qq * if 0.0 == 0.0 && ((assign53430_e68614) as f64).is_finite() && ((assign53430_e68614) as f64).fract() == 0.0 { if assign53430_e68614 == 0.0 { 0.0 } else { (assign53430_e68614 * ((assign53430_e68610).powf(assign53430_e68614 - 1.0) * ((locals.var_qeff1_ac_dn7 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn7)))) } } else { (assign53430_e68615 * (assign53430_e68614 * (((locals.var_qeff1_ac_dn7 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn7)) / assign53430_e68610))) })) / (assign53430_e68617 * assign53430_e68617))), (-((locals.var_cox_i * (locals.var_qq * if 0.0 == 0.0 && ((assign53430_e68614) as f64).is_finite() && ((assign53430_e68614) as f64).fract() == 0.0 { if assign53430_e68614 == 0.0 { 0.0 } else { (assign53430_e68614 * ((assign53430_e68610).powf(assign53430_e68614 - 1.0) * ((locals.var_qeff1_ac_dn8 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn8)))) } } else { (assign53430_e68615 * (assign53430_e68614 * (((locals.var_qeff1_ac_dn8 * locals.var_qeff1_ac) + (locals.var_qeff1_ac * locals.var_qeff1_ac_dn8)) / assign53430_e68610))) })) / (assign53430_e68617 * assign53430_e68617))), );
        }

        (locals.var_gdl_ac, locals.var_gdl_ac_dn5, locals.var_gdl_ac_dn6, locals.var_gdl_ac_dn7, locals.var_gdl_ac_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_gmob_dl_ac, locals.var_gmob_dl_ac_dn5, locals.var_gmob_dl_ac_dn6, locals.var_gmob_dl_ac_dn7, locals.var_gmob_dl_ac_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_thesat1_ac, locals.var_thesat1_ac_dn5, locals.var_thesat1_ac_dn6, locals.var_thesat1_ac_dn7, locals.var_thesat1_ac_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_gvsat_ac, locals.var_gvsat_ac_dn5, locals.var_gvsat_ac_dn6, locals.var_gvsat_ac_dn7, locals.var_gvsat_ac_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_h_ac, locals.var_h_ac_dn5, locals.var_h_ac_dn6, locals.var_h_ac_dn7, locals.var_h_ac_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_qg_1, locals.var_qg_1_dn5, locals.var_qg_1_dn6, locals.var_qg_1_dn7, locals.var_qg_1_dn8, ) = (locals.var_voxm_ac, locals.var_voxm_ac_dn5, locals.var_voxm_ac_dn6, locals.var_voxm_ac_dn7, locals.var_voxm_ac_dn8, );

        (locals.var_qi, locals.var_qi_dn5, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_qd_1, locals.var_qd_1_dn5, locals.var_qd_1_dn6, locals.var_qd_1_dn7, locals.var_qd_1_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_qb_1, locals.var_qb_1_dn5, locals.var_qb_1_dn6, locals.var_qb_1_dn7, locals.var_qb_1_dn8, ) = (locals.var_qg_1, locals.var_qg_1_dn5, locals.var_qg_1_dn6, locals.var_qg_1_dn7, locals.var_qg_1_dn8, );

        let assign53530_e68632: f64 = if locals.var_xg_ac > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1506 = assign53530_e68632;

        if (locals.var_guard1506 != 0.0) {
            let assign53540_e68637: f64 = (locals.var_alp1ac_i / locals.var_qim1_ac);
            let assign53540_e68638: f64 = (locals.var_alpac_i + assign53540_e68637);
            let assign53540_e68640: f64 = (assign53540_e68638 * locals.var_qim_ac);
            let assign53540_e68642: f64 = (assign53540_e68640 / locals.var_qim1_ac);
            let assign53540_e68644: f64 = (assign53540_e68642 * locals.var_s1_ac);
            (locals.var_dl__blk1263, locals.var_dl__blk1263_dn5, locals.var_dl__blk1263_dn6, locals.var_dl__blk1263_dn7, locals.var_dl__blk1263_dn8, ) = (assign53540_e68644, ((((((((-((locals.var_alp1ac_i * locals.var_qim1_ac_dn5) / (locals.var_qim1_ac * locals.var_qim1_ac))) * locals.var_qim_ac) + (assign53540_e68638 * locals.var_qim_ac_dn5)) * locals.var_qim1_ac) - (assign53540_e68640 * locals.var_qim1_ac_dn5)) / (locals.var_qim1_ac * locals.var_qim1_ac)) * locals.var_s1_ac) + (assign53540_e68642 * locals.var_s1_ac_dn5)), ((((((((-((locals.var_alp1ac_i * locals.var_qim1_ac_dn6) / (locals.var_qim1_ac * locals.var_qim1_ac))) * locals.var_qim_ac) + (assign53540_e68638 * locals.var_qim_ac_dn6)) * locals.var_qim1_ac) - (assign53540_e68640 * locals.var_qim1_ac_dn6)) / (locals.var_qim1_ac * locals.var_qim1_ac)) * locals.var_s1_ac) + (assign53540_e68642 * locals.var_s1_ac_dn6)), ((((((((-((locals.var_alp1ac_i * locals.var_qim1_ac_dn7) / (locals.var_qim1_ac * locals.var_qim1_ac))) * locals.var_qim_ac) + (assign53540_e68638 * locals.var_qim_ac_dn7)) * locals.var_qim1_ac) - (assign53540_e68640 * locals.var_qim1_ac_dn7)) / (locals.var_qim1_ac * locals.var_qim1_ac)) * locals.var_s1_ac) + (assign53540_e68642 * locals.var_s1_ac_dn7)), ((((((((-((locals.var_alp1ac_i * locals.var_qim1_ac_dn8) / (locals.var_qim1_ac * locals.var_qim1_ac))) * locals.var_qim_ac) + (assign53540_e68638 * locals.var_qim_ac_dn8)) * locals.var_qim1_ac) - (assign53540_e68640 * locals.var_qim1_ac_dn8)) / (locals.var_qim1_ac * locals.var_qim1_ac)) * locals.var_s1_ac) + (assign53540_e68642 * locals.var_s1_ac_dn8)), );
        }

        let assign53550_e68649: f64 = if locals.var_dl__blk1263 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1507 = assign53550_e68649;

        if ((locals.var_guard1506 != 0.0) && (locals.var_guard1507 != 0.0)) {
            let assign53560_e68656: f64 = (1.0 + locals.var_dl__blk1263);
            let assign53560_e68659: f64 = (locals.var_dl__blk1263 * locals.var_dl__blk1263);
            let assign53560_e68660: f64 = (assign53560_e68656 + assign53560_e68659);
            let assign53560_e68661: f64 = (1.0 / assign53560_e68660);
            (locals.var_gdl_ac, locals.var_gdl_ac_dn5, locals.var_gdl_ac_dn6, locals.var_gdl_ac_dn7, locals.var_gdl_ac_dn8, ) = (assign53560_e68661, (-((locals.var_dl__blk1263_dn5 + ((locals.var_dl__blk1263_dn5 * locals.var_dl__blk1263) + (locals.var_dl__blk1263 * locals.var_dl__blk1263_dn5))) / (assign53560_e68660 * assign53560_e68660))), (-((locals.var_dl__blk1263_dn6 + ((locals.var_dl__blk1263_dn6 * locals.var_dl__blk1263) + (locals.var_dl__blk1263 * locals.var_dl__blk1263_dn6))) / (assign53560_e68660 * assign53560_e68660))), (-((locals.var_dl__blk1263_dn7 + ((locals.var_dl__blk1263_dn7 * locals.var_dl__blk1263) + (locals.var_dl__blk1263 * locals.var_dl__blk1263_dn7))) / (assign53560_e68660 * assign53560_e68660))), (-((locals.var_dl__blk1263_dn8 + ((locals.var_dl__blk1263_dn8 * locals.var_dl__blk1263) + (locals.var_dl__blk1263 * locals.var_dl__blk1263_dn8))) / (assign53560_e68660 * assign53560_e68660))), );
        }

        if ((locals.var_guard1506 != 0.0) && (locals.var_guard1507 == 0.0)) {
            let assign53570_e68670: f64 = (1.0 - locals.var_dl__blk1263);
            (locals.var_gdl_ac, locals.var_gdl_ac_dn5, locals.var_gdl_ac_dn6, locals.var_gdl_ac_dn7, locals.var_gdl_ac_dn8, ) = (assign53570_e68670, (-locals.var_dl__blk1263_dn5), (-locals.var_dl__blk1263_dn6), (-locals.var_dl__blk1263_dn7), (-locals.var_dl__blk1263_dn8), );
        }

        if (locals.var_guard1506 != 0.0) {
            let assign53580_e68676: f64 = (locals.var_gmob_ac * locals.var_gdl_ac);
            (locals.var_gmob_dl_ac, locals.var_gmob_dl_ac_dn5, locals.var_gmob_dl_ac_dn6, locals.var_gmob_dl_ac_dn7, locals.var_gmob_dl_ac_dn8, ) = (assign53580_e68676, ((locals.var_gmob_ac_dn5 * locals.var_gdl_ac) + (locals.var_gmob_ac * locals.var_gdl_ac_dn5)), ((locals.var_gmob_ac_dn6 * locals.var_gdl_ac) + (locals.var_gmob_ac * locals.var_gdl_ac_dn6)), ((locals.var_gmob_ac_dn7 * locals.var_gdl_ac) + (locals.var_gmob_ac * locals.var_gdl_ac_dn7)), ((locals.var_gmob_ac_dn8 * locals.var_gdl_ac) + (locals.var_gmob_ac * locals.var_gdl_ac_dn8)), );
        }

        if (locals.var_guard1506 != 0.0) {
            let assign53590_e68682: f64 = (locals.var_thesateff_ac / locals.var_gmob_dl_ac);
            (locals.var_thesat1_ac, locals.var_thesat1_ac_dn5, locals.var_thesat1_ac_dn6, locals.var_thesat1_ac_dn7, locals.var_thesat1_ac_dn8, ) = (assign53590_e68682, (((locals.var_thesateff_ac_dn5 * locals.var_gmob_dl_ac) - (locals.var_thesateff_ac * locals.var_gmob_dl_ac_dn5)) / (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac)), (((locals.var_thesateff_ac_dn6 * locals.var_gmob_dl_ac) - (locals.var_thesateff_ac * locals.var_gmob_dl_ac_dn6)) / (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac)), (((locals.var_thesateff_ac_dn7 * locals.var_gmob_dl_ac) - (locals.var_thesateff_ac * locals.var_gmob_dl_ac_dn7)) / (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac)), (((locals.var_thesateff_ac_dn8 * locals.var_gmob_dl_ac) - (locals.var_thesateff_ac * locals.var_gmob_dl_ac_dn8)) / (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac)), );
        }

        if (locals.var_guard1506 != 0.0) {
            let assign53600_e68688: f64 = (locals.var_thesat1_ac * locals.var_thesat1_ac);
            let assign53600_e68690: f64 = (assign53600_e68688 * locals.var_dps_ac);
            let assign53600_e68692: f64 = (assign53600_e68690 * locals.var_dps_ac);
            (locals.var_zsat__blk1264, locals.var_zsat__blk1264_dn5, locals.var_zsat__blk1264_dn6, locals.var_zsat__blk1264_dn7, locals.var_zsat__blk1264_dn8, ) = (assign53600_e68692, ((((((locals.var_thesat1_ac_dn5 * locals.var_thesat1_ac) + (locals.var_thesat1_ac * locals.var_thesat1_ac_dn5)) * locals.var_dps_ac) + (assign53600_e68688 * locals.var_dps_ac_dn5)) * locals.var_dps_ac) + (assign53600_e68690 * locals.var_dps_ac_dn5)), ((((((locals.var_thesat1_ac_dn6 * locals.var_thesat1_ac) + (locals.var_thesat1_ac * locals.var_thesat1_ac_dn6)) * locals.var_dps_ac) + (assign53600_e68688 * locals.var_dps_ac_dn6)) * locals.var_dps_ac) + (assign53600_e68690 * locals.var_dps_ac_dn6)), ((((((locals.var_thesat1_ac_dn7 * locals.var_thesat1_ac) + (locals.var_thesat1_ac * locals.var_thesat1_ac_dn7)) * locals.var_dps_ac) + (assign53600_e68688 * locals.var_dps_ac_dn7)) * locals.var_dps_ac) + (assign53600_e68690 * locals.var_dps_ac_dn7)), ((((((locals.var_thesat1_ac_dn8 * locals.var_thesat1_ac) + (locals.var_thesat1_ac * locals.var_thesat1_ac_dn8)) * locals.var_dps_ac) + (assign53600_e68688 * locals.var_dps_ac_dn8)) * locals.var_dps_ac) + (assign53600_e68690 * locals.var_dps_ac_dn8)), );
        }

        let assign53610_e68697: f64 = (-1.0);
        let assign53610_e68698: f64 = if locals.var_chnl_type == assign53610_e68697 { 1.0 } else { 0.0 };
        locals.var_guard1508 = assign53610_e68698;

        if ((locals.var_guard1506 != 0.0) && (locals.var_guard1508 != 0.0)) {
            let assign53620_e68706: f64 = (locals.var_thesat1_ac * locals.var_dps_ac);
            let assign53620_e68707: f64 = (1.0 + assign53620_e68706);
            let assign53620_e68708: f64 = (locals.var_zsat__blk1264 / assign53620_e68707);
            (locals.var_zsat__blk1264, locals.var_zsat__blk1264_dn5, locals.var_zsat__blk1264_dn6, locals.var_zsat__blk1264_dn7, locals.var_zsat__blk1264_dn8, ) = (assign53620_e68708, (((locals.var_zsat__blk1264_dn5 * assign53620_e68707) - (locals.var_zsat__blk1264 * ((locals.var_thesat1_ac_dn5 * locals.var_dps_ac) + (locals.var_thesat1_ac * locals.var_dps_ac_dn5)))) / (assign53620_e68707 * assign53620_e68707)), (((locals.var_zsat__blk1264_dn6 * assign53620_e68707) - (locals.var_zsat__blk1264 * ((locals.var_thesat1_ac_dn6 * locals.var_dps_ac) + (locals.var_thesat1_ac * locals.var_dps_ac_dn6)))) / (assign53620_e68707 * assign53620_e68707)), (((locals.var_zsat__blk1264_dn7 * assign53620_e68707) - (locals.var_zsat__blk1264 * ((locals.var_thesat1_ac_dn7 * locals.var_dps_ac) + (locals.var_thesat1_ac * locals.var_dps_ac_dn7)))) / (assign53620_e68707 * assign53620_e68707)), (((locals.var_zsat__blk1264_dn8 * assign53620_e68707) - (locals.var_zsat__blk1264 * ((locals.var_thesat1_ac_dn8 * locals.var_dps_ac) + (locals.var_thesat1_ac * locals.var_dps_ac_dn8)))) / (assign53620_e68707 * assign53620_e68707)), );
        }

        if (locals.var_guard1506 != 0.0) {
            let assign53630_e68718: f64 = (2.0 * locals.var_zsat__blk1264);
            let assign53630_e68719: f64 = (1.0 + assign53630_e68718);
            let assign53630_e68720: f64 = (assign53630_e68719).sqrt();
            let assign53630_e68721: f64 = (1.0 + assign53630_e68720);
            let assign53630_e68722: f64 = (locals.var_gmob_dl_ac * assign53630_e68721);
            let assign53630_e68723: f64 = (0.5 * assign53630_e68722);
            (locals.var_gvsat_ac, locals.var_gvsat_ac_dn5, locals.var_gvsat_ac_dn6, locals.var_gvsat_ac_dn7, locals.var_gvsat_ac_dn8, ) = (assign53630_e68723, (0.5 * ((locals.var_gmob_dl_ac_dn5 * assign53630_e68721) + (locals.var_gmob_dl_ac * ((2.0 * locals.var_zsat__blk1264_dn5) / (2.0 * assign53630_e68720))))), (0.5 * ((locals.var_gmob_dl_ac_dn6 * assign53630_e68721) + (locals.var_gmob_dl_ac * ((2.0 * locals.var_zsat__blk1264_dn6) / (2.0 * assign53630_e68720))))), (0.5 * ((locals.var_gmob_dl_ac_dn7 * assign53630_e68721) + (locals.var_gmob_dl_ac * ((2.0 * locals.var_zsat__blk1264_dn7) / (2.0 * assign53630_e68720))))), (0.5 * ((locals.var_gmob_dl_ac_dn8 * assign53630_e68721) + (locals.var_gmob_dl_ac * ((2.0 * locals.var_zsat__blk1264_dn8) / (2.0 * assign53630_e68720))))), );
        }

        if (locals.var_guard1506 != 0.0) {
            let assign53640_e68729: f64 = (locals.var_gmob_dl_ac / locals.var_gvsat_ac);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign53640_e68729, (((locals.var_gmob_dl_ac_dn5 * locals.var_gvsat_ac) - (locals.var_gmob_dl_ac * locals.var_gvsat_ac_dn5)) / (locals.var_gvsat_ac * locals.var_gvsat_ac)), (((locals.var_gmob_dl_ac_dn6 * locals.var_gvsat_ac) - (locals.var_gmob_dl_ac * locals.var_gvsat_ac_dn6)) / (locals.var_gvsat_ac * locals.var_gvsat_ac)), (((locals.var_gmob_dl_ac_dn7 * locals.var_gvsat_ac) - (locals.var_gmob_dl_ac * locals.var_gvsat_ac_dn7)) / (locals.var_gvsat_ac * locals.var_gvsat_ac)), (((locals.var_gmob_dl_ac_dn8 * locals.var_gvsat_ac) - (locals.var_gmob_dl_ac * locals.var_gvsat_ac_dn8)) / (locals.var_gvsat_ac * locals.var_gvsat_ac)), );
        }

        if (locals.var_guard1506 != 0.0) {
            let assign53650_e68738: f64 = (locals.var_zsat__blk1264 * locals.var_temp__blk936);
            let assign53650_e68740: f64 = (assign53650_e68738 * locals.var_temp__blk936);
            let assign53650_e68741: f64 = (0.5 * assign53650_e68740);
            let assign53650_e68742: f64 = (1.0 + assign53650_e68741);
            let assign53650_e68743: f64 = (locals.var_alpha_ac * assign53650_e68742);
            (locals.var_alpha1__blk1265, locals.var_alpha1__blk1265_dn5, locals.var_alpha1__blk1265_dn6, locals.var_alpha1__blk1265_dn7, locals.var_alpha1__blk1265_dn8, ) = (assign53650_e68743, ((locals.var_alpha_ac_dn5 * assign53650_e68742) + (locals.var_alpha_ac * (0.5 * ((((locals.var_zsat__blk1264_dn5 * locals.var_temp__blk936) + (locals.var_zsat__blk1264 * locals.var_temp__blk936_dn5)) * locals.var_temp__blk936) + (assign53650_e68738 * locals.var_temp__blk936_dn5))))), ((locals.var_alpha_ac_dn6 * assign53650_e68742) + (locals.var_alpha_ac * (0.5 * ((((locals.var_zsat__blk1264_dn6 * locals.var_temp__blk936) + (locals.var_zsat__blk1264 * locals.var_temp__blk936_dn6)) * locals.var_temp__blk936) + (assign53650_e68738 * locals.var_temp__blk936_dn6))))), ((locals.var_alpha_ac_dn7 * assign53650_e68742) + (locals.var_alpha_ac * (0.5 * ((((locals.var_zsat__blk1264_dn7 * locals.var_temp__blk936) + (locals.var_zsat__blk1264 * locals.var_temp__blk936_dn7)) * locals.var_temp__blk936) + (assign53650_e68738 * locals.var_temp__blk936_dn7))))), ((locals.var_alpha_ac_dn8 * assign53650_e68742) + (locals.var_alpha_ac * (0.5 * ((((locals.var_zsat__blk1264_dn8 * locals.var_temp__blk936) + (locals.var_zsat__blk1264 * locals.var_temp__blk936_dn8)) * locals.var_temp__blk936) + (assign53650_e68738 * locals.var_temp__blk936_dn8))))), );
        }

        if (locals.var_guard1506 != 0.0) {
            let assign53660_e68749: f64 = (locals.var_temp__blk936 * locals.var_qim1_ac);
            let assign53660_e68751: f64 = (assign53660_e68749 / locals.var_alpha1__blk1265);
            (locals.var_h_ac, locals.var_h_ac_dn5, locals.var_h_ac_dn6, locals.var_h_ac_dn7, locals.var_h_ac_dn8, ) = (assign53660_e68751, (((((locals.var_temp__blk936_dn5 * locals.var_qim1_ac) + (locals.var_temp__blk936 * locals.var_qim1_ac_dn5)) * locals.var_alpha1__blk1265) - (assign53660_e68749 * locals.var_alpha1__blk1265_dn5)) / (locals.var_alpha1__blk1265 * locals.var_alpha1__blk1265)), (((((locals.var_temp__blk936_dn6 * locals.var_qim1_ac) + (locals.var_temp__blk936 * locals.var_qim1_ac_dn6)) * locals.var_alpha1__blk1265) - (assign53660_e68749 * locals.var_alpha1__blk1265_dn6)) / (locals.var_alpha1__blk1265 * locals.var_alpha1__blk1265)), (((((locals.var_temp__blk936_dn7 * locals.var_qim1_ac) + (locals.var_temp__blk936 * locals.var_qim1_ac_dn7)) * locals.var_alpha1__blk1265) - (assign53660_e68749 * locals.var_alpha1__blk1265_dn7)) / (locals.var_alpha1__blk1265 * locals.var_alpha1__blk1265)), (((((locals.var_temp__blk936_dn8 * locals.var_qim1_ac) + (locals.var_temp__blk936 * locals.var_qim1_ac_dn8)) * locals.var_alpha1__blk1265) - (assign53660_e68749 * locals.var_alpha1__blk1265_dn8)) / (locals.var_alpha1__blk1265 * locals.var_alpha1__blk1265)), );
        }

        if (locals.var_guard1506 != 0.0) {
            let assign53670_e68758: f64 = (locals.var_dps_ac / locals.var_h_ac);
            let assign53670_e68759: f64 = (0.5 * assign53670_e68758);
            (locals.var_fj, locals.var_fj_dn5, locals.var_fj_dn6, locals.var_fj_dn7, locals.var_fj_dn8, ) = (assign53670_e68759, (0.5 * (((locals.var_dps_ac_dn5 * locals.var_h_ac) - (locals.var_dps_ac * locals.var_h_ac_dn5)) / (locals.var_h_ac * locals.var_h_ac))), (0.5 * (((locals.var_dps_ac_dn6 * locals.var_h_ac) - (locals.var_dps_ac * locals.var_h_ac_dn6)) / (locals.var_h_ac * locals.var_h_ac))), (0.5 * (((locals.var_dps_ac_dn7 * locals.var_h_ac) - (locals.var_dps_ac * locals.var_h_ac_dn7)) / (locals.var_h_ac * locals.var_h_ac))), (0.5 * (((locals.var_dps_ac_dn8 * locals.var_h_ac) - (locals.var_dps_ac * locals.var_h_ac_dn8)) / (locals.var_h_ac * locals.var_h_ac))), );
        }

        if (locals.var_guard1506 != 0.0) {
            let assign53680_e68765: f64 = (locals.var_fj * locals.var_fj);
            (locals.var_fj2, locals.var_fj2_dn5, locals.var_fj2_dn6, locals.var_fj2_dn7, locals.var_fj2_dn8, ) = (assign53680_e68765, ((locals.var_fj_dn5 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn5)), ((locals.var_fj_dn6 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn6)), ((locals.var_fj_dn7 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn7)), ((locals.var_fj_dn8 * locals.var_fj) + (locals.var_fj * locals.var_fj_dn8)), );
        }

        if (locals.var_guard1506 != 0.0) {
            let assign53690_e68773: f64 = (locals.var_eta_p_ac * locals.var_dps_ac);
            let assign53690_e68776: f64 = (locals.var_fj * locals.var_gdl_ac);
            let assign53690_e68778: f64 = (assign53690_e68776 * 0.3333333333333333);
            let assign53690_e68780: f64 = (assign53690_e68778 - 1.0);
            let assign53690_e68782: f64 = (assign53690_e68780 + locals.var_gdl_ac);
            let assign53690_e68783: f64 = (assign53690_e68773 * assign53690_e68782);
            let assign53690_e68784: f64 = (0.5 * assign53690_e68783);
            let assign53690_e68785: f64 = (locals.var_voxm_ac + assign53690_e68784);
            (locals.var_qg_1, locals.var_qg_1_dn5, locals.var_qg_1_dn6, locals.var_qg_1_dn7, locals.var_qg_1_dn8, ) = (assign53690_e68785, (locals.var_voxm_ac_dn5 + (0.5 * ((((locals.var_eta_p_ac_dn5 * locals.var_dps_ac) + (locals.var_eta_p_ac * locals.var_dps_ac_dn5)) * assign53690_e68782) + (assign53690_e68773 * ((((locals.var_fj_dn5 * locals.var_gdl_ac) + (locals.var_fj * locals.var_gdl_ac_dn5)) * 0.3333333333333333) + locals.var_gdl_ac_dn5))))), (locals.var_voxm_ac_dn6 + (0.5 * ((((locals.var_eta_p_ac_dn6 * locals.var_dps_ac) + (locals.var_eta_p_ac * locals.var_dps_ac_dn6)) * assign53690_e68782) + (assign53690_e68773 * ((((locals.var_fj_dn6 * locals.var_gdl_ac) + (locals.var_fj * locals.var_gdl_ac_dn6)) * 0.3333333333333333) + locals.var_gdl_ac_dn6))))), (locals.var_voxm_ac_dn7 + (0.5 * ((((locals.var_eta_p_ac_dn7 * locals.var_dps_ac) + (locals.var_eta_p_ac * locals.var_dps_ac_dn7)) * assign53690_e68782) + (assign53690_e68773 * ((((locals.var_fj_dn7 * locals.var_gdl_ac) + (locals.var_fj * locals.var_gdl_ac_dn7)) * 0.3333333333333333) + locals.var_gdl_ac_dn7))))), (locals.var_voxm_ac_dn8 + (0.5 * ((((locals.var_eta_p_ac_dn8 * locals.var_dps_ac) + (locals.var_eta_p_ac * locals.var_dps_ac_dn8)) * assign53690_e68782) + (assign53690_e68773 * ((((locals.var_fj_dn8 * locals.var_gdl_ac) + (locals.var_fj * locals.var_gdl_ac_dn8)) * 0.3333333333333333) + locals.var_gdl_ac_dn8))))), );
        }

        if (locals.var_guard1506 != 0.0) {
            let assign53700_e68791: f64 = (locals.var_alpha_ac * locals.var_dps_ac);
            let assign53700_e68793: f64 = (assign53700_e68791 * 0.16666666666666666);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign53700_e68793, (((locals.var_alpha_ac_dn5 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn5)) * 0.16666666666666666), (((locals.var_alpha_ac_dn6 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn6)) * 0.16666666666666666), (((locals.var_alpha_ac_dn7 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn7)) * 0.16666666666666666), (((locals.var_alpha_ac_dn8 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn8)) * 0.16666666666666666), );
        }

        let assign53710_e68798: f64 = if p.p49 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1509 = assign53710_e68798;

        if ((locals.var_guard1506 != 0.0) && (locals.var_guard1509 != 0.0)) {
            (locals.var_qclm, locals.var_qclm_dn5, locals.var_qclm_dn6, locals.var_qclm_dn7, locals.var_qclm_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((locals.var_guard1506 != 0.0) && (locals.var_guard1509 != 0.0)) {
            let assign53730_e68810: f64 = (0.5 * locals.var_gdl_ac);
            let assign53730_e68812: f64 = (assign53730_e68810 * locals.var_gdl_ac);
            let assign53730_e68816: f64 = (3.0 * locals.var_temp__blk936);
            let assign53730_e68819: f64 = (2.0 - locals.var_fj);
            let assign53730_e68820: f64 = (assign53730_e68816 * assign53730_e68819);
            let assign53730_e68821: f64 = (locals.var_qim_ac - assign53730_e68820);
            let assign53730_e68822: f64 = (assign53730_e68812 * assign53730_e68821);
            (locals.var_qd_1, locals.var_qd_1_dn5, locals.var_qd_1_dn6, locals.var_qd_1_dn7, locals.var_qd_1_dn8, ) = (assign53730_e68822, (((((0.5 * locals.var_gdl_ac_dn5) * locals.var_gdl_ac) + (assign53730_e68810 * locals.var_gdl_ac_dn5)) * assign53730_e68821) + (assign53730_e68812 * (locals.var_qim_ac_dn5 - (((3.0 * locals.var_temp__blk936_dn5) * assign53730_e68819) + (assign53730_e68816 * (-locals.var_fj_dn5)))))), (((((0.5 * locals.var_gdl_ac_dn6) * locals.var_gdl_ac) + (assign53730_e68810 * locals.var_gdl_ac_dn6)) * assign53730_e68821) + (assign53730_e68812 * (locals.var_qim_ac_dn6 - (((3.0 * locals.var_temp__blk936_dn6) * assign53730_e68819) + (assign53730_e68816 * (-locals.var_fj_dn6)))))), (((((0.5 * locals.var_gdl_ac_dn7) * locals.var_gdl_ac) + (assign53730_e68810 * locals.var_gdl_ac_dn7)) * assign53730_e68821) + (assign53730_e68812 * (locals.var_qim_ac_dn7 - (((3.0 * locals.var_temp__blk936_dn7) * assign53730_e68819) + (assign53730_e68816 * (-locals.var_fj_dn7)))))), (((((0.5 * locals.var_gdl_ac_dn8) * locals.var_gdl_ac) + (assign53730_e68810 * locals.var_gdl_ac_dn8)) * assign53730_e68821) + (assign53730_e68812 * (locals.var_qim_ac_dn8 - (((3.0 * locals.var_temp__blk936_dn8) * assign53730_e68819) + (assign53730_e68816 * (-locals.var_fj_dn8)))))), );
        }

        if ((locals.var_guard1506 != 0.0) && (locals.var_guard1509 == 0.0)) {
            let assign53740_e68831: f64 = (1.0 - locals.var_gdl_ac);
            let assign53740_e68836: f64 = (locals.var_alpha_ac * locals.var_dps_ac);
            let assign53740_e68837: f64 = (0.5 * assign53740_e68836);
            let assign53740_e68838: f64 = (locals.var_qim_ac - assign53740_e68837);
            let assign53740_e68839: f64 = (assign53740_e68831 * assign53740_e68838);
            (locals.var_qclm, locals.var_qclm_dn5, locals.var_qclm_dn6, locals.var_qclm_dn7, locals.var_qclm_dn8, ) = (assign53740_e68839, (((-locals.var_gdl_ac_dn5) * assign53740_e68838) + (assign53740_e68831 * (locals.var_qim_ac_dn5 - (0.5 * ((locals.var_alpha_ac_dn5 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn5)))))), (((-locals.var_gdl_ac_dn6) * assign53740_e68838) + (assign53740_e68831 * (locals.var_qim_ac_dn6 - (0.5 * ((locals.var_alpha_ac_dn6 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn6)))))), (((-locals.var_gdl_ac_dn7) * assign53740_e68838) + (assign53740_e68831 * (locals.var_qim_ac_dn7 - (0.5 * ((locals.var_alpha_ac_dn7 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn7)))))), (((-locals.var_gdl_ac_dn8) * assign53740_e68838) + (assign53740_e68831 * (locals.var_qim_ac_dn8 - (0.5 * ((locals.var_alpha_ac_dn8 * locals.var_dps_ac) + (locals.var_alpha_ac * locals.var_dps_ac_dn8)))))), );
        }

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
            (locals.var_qd_1, locals.var_qd_1_dn5, locals.var_qd_1_dn6, locals.var_qd_1_dn7, locals.var_qd_1_dn8, ) = (assign53750_e68868, (0.5 * (((((locals.var_gdl_ac_dn5 * locals.var_gdl_ac) + (locals.var_gdl_ac * locals.var_gdl_ac_dn5)) * assign53750_e68860) + (assign53750_e68849 * (locals.var_qim_ac_dn5 - ((locals.var_temp__blk936_dn5 * assign53750_e68858) + (locals.var_temp__blk936 * ((-locals.var_fj_dn5) - (0.2 * locals.var_fj2_dn5))))))) + ((locals.var_qclm_dn5 * assign53750_e68865) + (locals.var_qclm * locals.var_gdl_ac_dn5)))), (0.5 * (((((locals.var_gdl_ac_dn6 * locals.var_gdl_ac) + (locals.var_gdl_ac * locals.var_gdl_ac_dn6)) * assign53750_e68860) + (assign53750_e68849 * (locals.var_qim_ac_dn6 - ((locals.var_temp__blk936_dn6 * assign53750_e68858) + (locals.var_temp__blk936 * ((-locals.var_fj_dn6) - (0.2 * locals.var_fj2_dn6))))))) + ((locals.var_qclm_dn6 * assign53750_e68865) + (locals.var_qclm * locals.var_gdl_ac_dn6)))), (0.5 * (((((locals.var_gdl_ac_dn7 * locals.var_gdl_ac) + (locals.var_gdl_ac * locals.var_gdl_ac_dn7)) * assign53750_e68860) + (assign53750_e68849 * (locals.var_qim_ac_dn7 - ((locals.var_temp__blk936_dn7 * assign53750_e68858) + (locals.var_temp__blk936 * ((-locals.var_fj_dn7) - (0.2 * locals.var_fj2_dn7))))))) + ((locals.var_qclm_dn7 * assign53750_e68865) + (locals.var_qclm * locals.var_gdl_ac_dn7)))), (0.5 * (((((locals.var_gdl_ac_dn8 * locals.var_gdl_ac) + (locals.var_gdl_ac * locals.var_gdl_ac_dn8)) * assign53750_e68860) + (assign53750_e68849 * (locals.var_qim_ac_dn8 - ((locals.var_temp__blk936_dn8 * assign53750_e68858) + (locals.var_temp__blk936 * ((-locals.var_fj_dn8) - (0.2 * locals.var_fj2_dn8))))))) + ((locals.var_qclm_dn8 * assign53750_e68865) + (locals.var_qclm * locals.var_gdl_ac_dn8)))), );
        }

        if (locals.var_guard1506 != 0.0) {
            let assign53760_e68876: f64 = (locals.var_temp__blk936 * locals.var_fj);
            let assign53760_e68877: f64 = (locals.var_qim_ac + assign53760_e68876);
            let assign53760_e68878: f64 = (locals.var_gdl_ac * assign53760_e68877);
            let assign53760_e68880: f64 = (assign53760_e68878 + locals.var_qclm);
            (locals.var_qi, locals.var_qi_dn5, locals.var_qi_dn6, locals.var_qi_dn7, locals.var_qi_dn8, ) = (assign53760_e68880, (((locals.var_gdl_ac_dn5 * assign53760_e68877) + (locals.var_gdl_ac * (locals.var_qim_ac_dn5 + ((locals.var_temp__blk936_dn5 * locals.var_fj) + (locals.var_temp__blk936 * locals.var_fj_dn5))))) + locals.var_qclm_dn5), (((locals.var_gdl_ac_dn6 * assign53760_e68877) + (locals.var_gdl_ac * (locals.var_qim_ac_dn6 + ((locals.var_temp__blk936_dn6 * locals.var_fj) + (locals.var_temp__blk936 * locals.var_fj_dn6))))) + locals.var_qclm_dn6), (((locals.var_gdl_ac_dn7 * assign53760_e68877) + (locals.var_gdl_ac * (locals.var_qim_ac_dn7 + ((locals.var_temp__blk936_dn7 * locals.var_fj) + (locals.var_temp__blk936 * locals.var_fj_dn7))))) + locals.var_qclm_dn7), (((locals.var_gdl_ac_dn8 * assign53760_e68877) + (locals.var_gdl_ac * (locals.var_qim_ac_dn8 + ((locals.var_temp__blk936_dn8 * locals.var_fj) + (locals.var_temp__blk936 * locals.var_fj_dn8))))) + locals.var_qclm_dn8), );
        }

        if (locals.var_guard1506 != 0.0) {
            let assign53770_e68886: f64 = (locals.var_qg_1 - locals.var_qi);
            (locals.var_qb_1, locals.var_qb_1_dn5, locals.var_qb_1_dn6, locals.var_qb_1_dn7, locals.var_qb_1_dn8, ) = (assign53770_e68886, (locals.var_qg_1_dn5 - locals.var_qi_dn5), (locals.var_qg_1_dn6 - locals.var_qi_dn6), (locals.var_qg_1_dn7 - locals.var_qi_dn7), (locals.var_qg_1_dn8 - locals.var_qi_dn8), );
        }

        let assign53780_e68891: f64 = (locals.var_qg_1 * locals.var_cox_qm);
        (locals.var_qg, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, ) = (assign53780_e68891, ((locals.var_qg_1_dn5 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn5)), ((locals.var_qg_1_dn6 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn6)), ((locals.var_qg_1_dn7 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn7)), ((locals.var_qg_1_dn8 * locals.var_cox_qm) + (locals.var_qg_1 * locals.var_cox_qm_dn8)), );

        let assign53790_e68893: f64 = (-locals.var_qd_1);
        let assign53790_e68895: f64 = (assign53790_e68893 * locals.var_cox_qm);
        (locals.var_qd, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, ) = (assign53790_e68895, (((-locals.var_qd_1_dn5) * locals.var_cox_qm) + (assign53790_e68893 * locals.var_cox_qm_dn5)), (((-locals.var_qd_1_dn6) * locals.var_cox_qm) + (assign53790_e68893 * locals.var_cox_qm_dn6)), (((-locals.var_qd_1_dn7) * locals.var_cox_qm) + (assign53790_e68893 * locals.var_cox_qm_dn7)), (((-locals.var_qd_1_dn8) * locals.var_cox_qm) + (assign53790_e68893 * locals.var_cox_qm_dn8)), );

        let assign53800_e68897: f64 = (-locals.var_qb_1);
        let assign53800_e68899: f64 = (assign53800_e68897 * locals.var_cox_qm);
        (locals.var_qb, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, ) = (assign53800_e68899, (((-locals.var_qb_1_dn5) * locals.var_cox_qm) + (assign53800_e68897 * locals.var_cox_qm_dn5)), (((-locals.var_qb_1_dn6) * locals.var_cox_qm) + (assign53800_e68897 * locals.var_cox_qm_dn6)), (((-locals.var_qb_1_dn7) * locals.var_cox_qm) + (assign53800_e68897 * locals.var_cox_qm_dn7)), (((-locals.var_qb_1_dn8) * locals.var_cox_qm) + (assign53800_e68897 * locals.var_cox_qm_dn8)), );

        (locals.var_qsinr, locals.var_qsinr_dn5, locals.var_qsinr_dn6, locals.var_qsinr_dn7, locals.var_qsinr_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_qdinr, locals.var_qdinr_dn5, locals.var_qdinr_dn6, locals.var_qdinr_dn7, locals.var_qdinr_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_qginr, locals.var_qginr_dn5, locals.var_qginr_dn6, locals.var_qginr_dn7, locals.var_qginr_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign53840_e68909: f64 = if ((locals.var_cinr_i > 0.0) || (locals.var_cinrd_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1510 = assign53840_e68909;

        if (locals.var_guard1510 != 0.0) {
            (locals.var_finracc, locals.var_finracc_dn5, locals.var_finracc_dn6, locals.var_finracc_dn7, locals.var_finracc_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_dvinracc, locals.var_dvinracc_dn5, locals.var_dvinracc_dn6, locals.var_dvinracc_dn7, locals.var_dvinracc_dn8, ) = (locals.var_vgb1_ac, locals.var_vgb1_ac_dn5, locals.var_vgb1_ac_dn6, locals.var_vgb1_ac_dn7, locals.var_vgb1_ac_dn8, );
        }

        let assign53870_e68920: f64 = if locals.var_fcinracc_i > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1511 = assign53870_e68920;

        if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
            let assign53880_e68926: f64 = (locals.var_vgb1_ac - locals.var_dvfbinr_i);
            let assign53880_e68928: f64 = (assign53880_e68926 + locals.var_vinr_max);
            (locals.var_vginr, locals.var_vginr_dn5, locals.var_vginr_dn6, locals.var_vginr_dn7, locals.var_vginr_dn8, ) = (assign53880_e68928, locals.var_vgb1_ac_dn5, locals.var_vgb1_ac_dn6, locals.var_vgb1_ac_dn7, locals.var_vgb1_ac_dn8, );
        }

        if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
            let assign53890_e68937: f64 = (locals.var_vginr + locals.var_vinr_max);
            let assign53890_e68940: f64 = (locals.var_vginr - locals.var_vinr_max);
            let assign53890_e68943: f64 = (locals.var_vginr - locals.var_vinr_max);
            let assign53890_e68944: f64 = (assign53890_e68940 * assign53890_e68943);
            let assign53890_e68946: f64 = (assign53890_e68944 + locals.var_ainr);
            let assign53890_e68947: f64 = (assign53890_e68946).sqrt();
            let assign53890_e68948: f64 = (assign53890_e68937 + assign53890_e68947);
            let assign53890_e68949: f64 = (0.5 * assign53890_e68948);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign53890_e68949, (0.5 * (locals.var_vginr_dn5 + (((locals.var_vginr_dn5 * assign53890_e68943) + (assign53890_e68940 * locals.var_vginr_dn5)) / (2.0 * assign53890_e68947)))), (0.5 * (locals.var_vginr_dn6 + (((locals.var_vginr_dn6 * assign53890_e68943) + (assign53890_e68940 * locals.var_vginr_dn6)) / (2.0 * assign53890_e68947)))), (0.5 * (locals.var_vginr_dn7 + (((locals.var_vginr_dn7 * assign53890_e68943) + (assign53890_e68940 * locals.var_vginr_dn7)) / (2.0 * assign53890_e68947)))), (0.5 * (locals.var_vginr_dn8 + (((locals.var_vginr_dn8 * assign53890_e68943) + (assign53890_e68940 * locals.var_vginr_dn8)) / (2.0 * assign53890_e68947)))), );
        }

        if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
            let assign53900_e68958: f64 = (2.0 * locals.var_temp__blk936);
            let assign53900_e68960: f64 = (assign53900_e68958 - locals.var_vinr_max);
            let assign53900_e68962: f64 = (assign53900_e68960 - locals.var_vginr);
            let assign53900_e68963: f64 = (locals.var_temp__blk936 * assign53900_e68962);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign53900_e68963, ((locals.var_temp__blk936_dn5 * assign53900_e68962) + (locals.var_temp__blk936 * ((2.0 * locals.var_temp__blk936_dn5) - locals.var_vginr_dn5))), ((locals.var_temp__blk936_dn6 * assign53900_e68962) + (locals.var_temp__blk936 * ((2.0 * locals.var_temp__blk936_dn6) - locals.var_vginr_dn6))), ((locals.var_temp__blk936_dn7 * assign53900_e68962) + (locals.var_temp__blk936 * ((2.0 * locals.var_temp__blk936_dn7) - locals.var_vginr_dn7))), ((locals.var_temp__blk936_dn8 * assign53900_e68962) + (locals.var_temp__blk936 * ((2.0 * locals.var_temp__blk936_dn8) - locals.var_vginr_dn8))), );
        }

        if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
            let assign53910_e68971: f64 = (locals.var_vinr_max / locals.var_temp__blk936);
            (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, ) = (assign53910_e68971, (-((locals.var_vinr_max * locals.var_temp__blk936_dn5) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (-((locals.var_vinr_max * locals.var_temp__blk936_dn6) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (-((locals.var_vinr_max * locals.var_temp__blk936_dn7) / (locals.var_temp__blk936 * locals.var_temp__blk936))), (-((locals.var_vinr_max * locals.var_temp__blk936_dn8) / (locals.var_temp__blk936 * locals.var_temp__blk936))), );
        }

        if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
            let assign53920_e68979: f64 = (locals.var_vginr * locals.var_temp2);
            (locals.var_vginreff, locals.var_vginreff_dn5, locals.var_vginreff_dn6, locals.var_vginreff_dn7, locals.var_vginreff_dn8, ) = (assign53920_e68979, ((locals.var_vginr_dn5 * locals.var_temp2) + (locals.var_vginr * locals.var_temp2_dn5)), ((locals.var_vginr_dn6 * locals.var_temp2) + (locals.var_vginr * locals.var_temp2_dn6)), ((locals.var_vginr_dn7 * locals.var_temp2) + (locals.var_vginr * locals.var_temp2_dn7)), ((locals.var_vginr_dn8 * locals.var_temp2) + (locals.var_vginr * locals.var_temp2_dn8)), );
        }

        if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
            let assign53930_e68988: f64 = (locals.var_vginreff * locals.var_fcinracc_i);
            let assign53930_e68989: f64 = (1.0 - assign53930_e68988);
            let assign53930_e68990: f64 = (assign53930_e68989).sqrt();
            (locals.var_fqinr, locals.var_fqinr_dn5, locals.var_fqinr_dn6, locals.var_fqinr_dn7, locals.var_fqinr_dn8, ) = (assign53930_e68990, ((-(locals.var_vginreff_dn5 * locals.var_fcinracc_i)) / (2.0 * assign53930_e68990)), ((-(locals.var_vginreff_dn6 * locals.var_fcinracc_i)) / (2.0 * assign53930_e68990)), ((-(locals.var_vginreff_dn7 * locals.var_fcinracc_i)) / (2.0 * assign53930_e68990)), ((-(locals.var_vginreff_dn8 * locals.var_fcinracc_i)) / (2.0 * assign53930_e68990)), );
        }

        if ((locals.var_guard1510 != 0.0) && (locals.var_guard1511 != 0.0)) {
            let assign53940_e68998: f64 = (1.0 - locals.var_fqinr);
            let assign53940_e69000: f64 = (assign53940_e68998 / locals.var_fcinracc_i);
            let assign53940_e69002: f64 = (assign53940_e69000 + locals.var_vginr);
            let assign53940_e69004: f64 = (assign53940_e69002 - locals.var_vginreff);
            (locals.var_dvinracc, locals.var_dvinracc_dn5, locals.var_dvinracc_dn6, locals.var_dvinracc_dn7, locals.var_dvinracc_dn8, ) = (assign53940_e69004, ((((-locals.var_fqinr_dn5) / locals.var_fcinracc_i) + locals.var_vginr_dn5) - locals.var_vginreff_dn5), ((((-locals.var_fqinr_dn6) / locals.var_fcinracc_i) + locals.var_vginr_dn6) - locals.var_vginreff_dn6), ((((-locals.var_fqinr_dn7) / locals.var_fcinracc_i) + locals.var_vginr_dn7) - locals.var_vginreff_dn7), ((((-locals.var_fqinr_dn8) / locals.var_fcinracc_i) + locals.var_vginr_dn8) - locals.var_vginreff_dn8), );
        }

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
            (locals.var_finracc, locals.var_finracc_dn5, locals.var_finracc_dn6, locals.var_finracc_dn7, locals.var_finracc_dn8, ) = (assign53950_e69028, ((((((((-((0.5 * locals.var_fqinr_dn5) / (locals.var_fqinr * locals.var_fqinr))) * assign53950_e69021) + (assign53950_e69014 * (locals.var_temp1_dn5 + ((locals.var_vginr_dn5 * assign53950_e69019) + (locals.var_vginr * (-locals.var_temp__blk936_dn5)))))) * locals.var_temp2) + (assign53950_e69022 * locals.var_temp2_dn5)) * locals.var_temp1) - (assign53950_e69024 * locals.var_temp1_dn5)) / (locals.var_temp1 * locals.var_temp1)), ((((((((-((0.5 * locals.var_fqinr_dn6) / (locals.var_fqinr * locals.var_fqinr))) * assign53950_e69021) + (assign53950_e69014 * (locals.var_temp1_dn6 + ((locals.var_vginr_dn6 * assign53950_e69019) + (locals.var_vginr * (-locals.var_temp__blk936_dn6)))))) * locals.var_temp2) + (assign53950_e69022 * locals.var_temp2_dn6)) * locals.var_temp1) - (assign53950_e69024 * locals.var_temp1_dn6)) / (locals.var_temp1 * locals.var_temp1)), ((((((((-((0.5 * locals.var_fqinr_dn7) / (locals.var_fqinr * locals.var_fqinr))) * assign53950_e69021) + (assign53950_e69014 * (locals.var_temp1_dn7 + ((locals.var_vginr_dn7 * assign53950_e69019) + (locals.var_vginr * (-locals.var_temp__blk936_dn7)))))) * locals.var_temp2) + (assign53950_e69022 * locals.var_temp2_dn7)) * locals.var_temp1) - (assign53950_e69024 * locals.var_temp1_dn7)) / (locals.var_temp1 * locals.var_temp1)), ((((((((-((0.5 * locals.var_fqinr_dn8) / (locals.var_fqinr * locals.var_fqinr))) * assign53950_e69021) + (assign53950_e69014 * (locals.var_temp1_dn8 + ((locals.var_vginr_dn8 * assign53950_e69019) + (locals.var_vginr * (-locals.var_temp__blk936_dn8)))))) * locals.var_temp2) + (assign53950_e69022 * locals.var_temp2_dn8)) * locals.var_temp1) - (assign53950_e69024 * locals.var_temp1_dn8)) / (locals.var_temp1 * locals.var_temp1)), );
        }

        if (locals.var_guard1510 != 0.0) {
            (locals.var_finrdep, locals.var_finrdep_dn5, locals.var_finrdep_dn6, locals.var_finrdep_dn7, locals.var_finrdep_dn8, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_dvinrdep, locals.var_dvinrdep_dn5, locals.var_dvinrdep_dn6, locals.var_dvinrdep_dn7, locals.var_dvinrdep_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign53980_e69041: f64 = if locals.var_fcinrdep_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1512 = assign53980_e69041;

        if ((locals.var_guard1510 != 0.0) && (locals.var_guard1512 != 0.0)) {
            let assign53990_e69047: f64 = (0.5 * locals.var_phib_ac);
            let assign53990_e69052: f64 = (locals.var_gf_ac * 0.7071067811865475);
            let assign53990_e69053: f64 = (1.0 + assign53990_e69052);
            let assign53990_e69054: f64 = (locals.var_phit1_ac * assign53990_e69053);
            let assign53990_e69055: f64 = (assign53990_e69047 + assign53990_e69054);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign53990_e69055, ((locals.var_phit1_ac_dn5 * assign53990_e69053) + (locals.var_phit1_ac * (locals.var_gf_ac_dn5 * 0.7071067811865475))), ((locals.var_phit1_ac_dn6 * assign53990_e69053) + (locals.var_phit1_ac * (locals.var_gf_ac_dn6 * 0.7071067811865475))), ((locals.var_phit1_ac_dn7 * assign53990_e69053) + (locals.var_phit1_ac * (locals.var_gf_ac_dn7 * 0.7071067811865475))), ((locals.var_phit1_ac_dn8 * assign53990_e69053) + (locals.var_phit1_ac * (locals.var_gf_ac_dn8 * 0.7071067811865475))), );
        }

        if ((locals.var_guard1510 != 0.0) && (locals.var_guard1512 != 0.0)) {
            let assign54000_e69063: f64 = (locals.var_vgb1_ac / locals.var_temp__blk936);
            (locals.var_xginrdep, locals.var_xginrdep_dn5, locals.var_xginrdep_dn6, locals.var_xginrdep_dn7, locals.var_xginrdep_dn8, ) = (assign54000_e69063, (((locals.var_vgb1_ac_dn5 * locals.var_temp__blk936) - (locals.var_vgb1_ac * locals.var_temp__blk936_dn5)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_vgb1_ac_dn6 * locals.var_temp__blk936) - (locals.var_vgb1_ac * locals.var_temp__blk936_dn6)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_vgb1_ac_dn7 * locals.var_temp__blk936) - (locals.var_vgb1_ac * locals.var_temp__blk936_dn7)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), (((locals.var_vgb1_ac_dn8 * locals.var_temp__blk936) - (locals.var_vgb1_ac * locals.var_temp__blk936_dn8)) / (locals.var_temp__blk936 * locals.var_temp__blk936)), );
        }

        let assign54010_e69067: f64 = (locals.var_xginrdep).abs();
        let assign54010_e69069: f64 = if assign54010_e69067 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1513 = assign54010_e69069;

        if (((locals.var_guard1510 != 0.0) && (locals.var_guard1512 != 0.0)) && (locals.var_guard1513 != 0.0)) {
            let assign54020_e69078: f64 = (-locals.var_xginrdep);
            let assign54020_e69079: f64 = (assign54020_e69078).exp();
            let assign54020_e69080: f64 = (1.0 + assign54020_e69079);
            let assign54020_e69081: f64 = (1.0 / assign54020_e69080);
            (locals.var_finrdep, locals.var_finrdep_dn5, locals.var_finrdep_dn6, locals.var_finrdep_dn7, locals.var_finrdep_dn8, ) = (assign54020_e69081, (-((assign54020_e69079 * (-locals.var_xginrdep_dn5)) / (assign54020_e69080 * assign54020_e69080))), (-((assign54020_e69079 * (-locals.var_xginrdep_dn6)) / (assign54020_e69080 * assign54020_e69080))), (-((assign54020_e69079 * (-locals.var_xginrdep_dn7)) / (assign54020_e69080 * assign54020_e69080))), (-((assign54020_e69079 * (-locals.var_xginrdep_dn8)) / (assign54020_e69080 * assign54020_e69080))), );
        }

        let assign54030_e69086: f64 = if locals.var_xginrdep < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1514 = assign54030_e69086;

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
            (locals.var_finrdep, locals.var_finrdep_dn5, locals.var_finrdep_dn6, locals.var_finrdep_dn7, locals.var_finrdep_dn8, ) = (assign54040_e69120, (-((1e-100 * ((locals.var_xginrdep_dn5 * assign54040_e69117) + (assign54040_e69100 * (0.5 * ((locals.var_xginrdep_dn5 * assign54040_e69114) + (assign54040_e69106 * (locals.var_xginrdep_dn5 * 0.3333333333333333))))))) / (assign54040_e69119 * assign54040_e69119))), (-((1e-100 * ((locals.var_xginrdep_dn6 * assign54040_e69117) + (assign54040_e69100 * (0.5 * ((locals.var_xginrdep_dn6 * assign54040_e69114) + (assign54040_e69106 * (locals.var_xginrdep_dn6 * 0.3333333333333333))))))) / (assign54040_e69119 * assign54040_e69119))), (-((1e-100 * ((locals.var_xginrdep_dn7 * assign54040_e69117) + (assign54040_e69100 * (0.5 * ((locals.var_xginrdep_dn7 * assign54040_e69114) + (assign54040_e69106 * (locals.var_xginrdep_dn7 * 0.3333333333333333))))))) / (assign54040_e69119 * assign54040_e69119))), (-((1e-100 * ((locals.var_xginrdep_dn8 * assign54040_e69117) + (assign54040_e69100 * (0.5 * ((locals.var_xginrdep_dn8 * assign54040_e69114) + (assign54040_e69106 * (locals.var_xginrdep_dn8 * 0.3333333333333333))))))) / (assign54040_e69119 * assign54040_e69119))), );
        }

        let assign54050_e69125: f64 = if locals.var_xginrdep < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1515 = assign54050_e69125;

        if (((locals.var_guard1510 != 0.0) && (locals.var_guard1512 != 0.0)) && (locals.var_guard1515 != 0.0)) {
            let assign54060_e69133: f64 = (locals.var_xginrdep).exp();
            let assign54060_e69134: f64 = (1.0 + assign54060_e69133);
            let assign54060_e69135: f64 = (assign54060_e69134).ln();
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign54060_e69135, ((assign54060_e69133 * locals.var_xginrdep_dn5) / assign54060_e69134), ((assign54060_e69133 * locals.var_xginrdep_dn6) / assign54060_e69134), ((assign54060_e69133 * locals.var_xginrdep_dn7) / assign54060_e69134), ((assign54060_e69133 * locals.var_xginrdep_dn8) / assign54060_e69134), );
        }

        if (((locals.var_guard1510 != 0.0) && (locals.var_guard1512 != 0.0)) && (locals.var_guard1515 == 0.0)) {
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (locals.var_xginrdep, locals.var_xginrdep_dn5, locals.var_xginrdep_dn6, locals.var_xginrdep_dn7, locals.var_xginrdep_dn8, );
        }

        if ((locals.var_guard1510 != 0.0) && (locals.var_guard1512 != 0.0)) {
            let assign54080_e69152: f64 = (locals.var_temp__blk936 * locals.var_temp1);
            (locals.var_dvinrdep, locals.var_dvinrdep_dn5, locals.var_dvinrdep_dn6, locals.var_dvinrdep_dn7, locals.var_dvinrdep_dn8, ) = (assign54080_e69152, ((locals.var_temp__blk936_dn5 * locals.var_temp1) + (locals.var_temp__blk936 * locals.var_temp1_dn5)), ((locals.var_temp__blk936_dn6 * locals.var_temp1) + (locals.var_temp__blk936 * locals.var_temp1_dn6)), ((locals.var_temp__blk936_dn7 * locals.var_temp1) + (locals.var_temp__blk936 * locals.var_temp1_dn7)), ((locals.var_temp__blk936_dn8 * locals.var_temp1) + (locals.var_temp__blk936 * locals.var_temp1_dn8)), );
        }

        if (locals.var_guard1510 != 0.0) {
            let assign54090_e69159: f64 = (locals.var_finrdep - locals.var_finracc);
            let assign54090_e69160: f64 = (locals.var_fcinrdep_i * assign54090_e69159);
            let assign54090_e69162: f64 = (assign54090_e69160 + locals.var_finracc);
            (locals.var_finr, locals.var_finr_dn5, locals.var_finr_dn6, locals.var_finr_dn7, locals.var_finr_dn8, ) = (assign54090_e69162, ((locals.var_fcinrdep_i * (locals.var_finrdep_dn5 - locals.var_finracc_dn5)) + locals.var_finracc_dn5), ((locals.var_fcinrdep_i * (locals.var_finrdep_dn6 - locals.var_finracc_dn6)) + locals.var_finracc_dn6), ((locals.var_fcinrdep_i * (locals.var_finrdep_dn7 - locals.var_finracc_dn7)) + locals.var_finracc_dn7), ((locals.var_fcinrdep_i * (locals.var_finrdep_dn8 - locals.var_finracc_dn8)) + locals.var_finracc_dn8), );
        }

        if (locals.var_guard1510 != 0.0) {
            let assign54100_e69169: f64 = (locals.var_dvinrdep - locals.var_dvinracc);
            let assign54100_e69170: f64 = (locals.var_fcinrdep_i * assign54100_e69169);
            let assign54100_e69172: f64 = (assign54100_e69170 + locals.var_dvinracc);
            (locals.var_dvinr, locals.var_dvinr_dn5, locals.var_dvinr_dn6, locals.var_dvinr_dn7, locals.var_dvinr_dn8, ) = (assign54100_e69172, ((locals.var_fcinrdep_i * (locals.var_dvinrdep_dn5 - locals.var_dvinracc_dn5)) + locals.var_dvinracc_dn5), ((locals.var_fcinrdep_i * (locals.var_dvinrdep_dn6 - locals.var_dvinracc_dn6)) + locals.var_dvinracc_dn6), ((locals.var_fcinrdep_i * (locals.var_dvinrdep_dn7 - locals.var_dvinracc_dn7)) + locals.var_dvinracc_dn7), ((locals.var_fcinrdep_i * (locals.var_dvinrdep_dn8 - locals.var_dvinracc_dn8)) + locals.var_dvinracc_dn8), );
        }

        if (locals.var_guard1510 != 0.0) {
            let assign54110_e69179: f64 = (locals.var_phit1_ac * locals.var_xno_s_ac);
            let assign54110_e69180: f64 = (locals.var_vgb1_ac - assign54110_e69179);
            let assign54110_e69182: f64 = (assign54110_e69180 - locals.var_voxm_ac);
            let assign54110_e69185: f64 = (0.5 * locals.var_dps_ac);
            let assign54110_e69186: f64 = (assign54110_e69182 - assign54110_e69185);
            (locals.var_vgsinr, locals.var_vgsinr_dn5, locals.var_vgsinr_dn6, locals.var_vgsinr_dn7, locals.var_vgsinr_dn8, ) = (assign54110_e69186, (((locals.var_vgb1_ac_dn5 - ((locals.var_phit1_ac_dn5 * locals.var_xno_s_ac) + (locals.var_phit1_ac * locals.var_xno_s_ac_dn5))) - locals.var_voxm_ac_dn5) - (0.5 * locals.var_dps_ac_dn5)), (((locals.var_vgb1_ac_dn6 - ((locals.var_phit1_ac_dn6 * locals.var_xno_s_ac) + (locals.var_phit1_ac * locals.var_xno_s_ac_dn6))) - locals.var_voxm_ac_dn6) - (0.5 * locals.var_dps_ac_dn6)), (((locals.var_vgb1_ac_dn7 - ((locals.var_phit1_ac_dn7 * locals.var_xno_s_ac) + (locals.var_phit1_ac * locals.var_xno_s_ac_dn7))) - locals.var_voxm_ac_dn7) - (0.5 * locals.var_dps_ac_dn7)), (((locals.var_vgb1_ac_dn8 - ((locals.var_phit1_ac_dn8 * locals.var_xno_s_ac) + (locals.var_phit1_ac * locals.var_xno_s_ac_dn8))) - locals.var_voxm_ac_dn8) - (0.5 * locals.var_dps_ac_dn8)), );
        }

        if (locals.var_guard1510 != 0.0) {
            let assign54120_e69192: f64 = (locals.var_vgb1_ac - locals.var_vgsinr);
            let assign54120_e69194: f64 = (assign54120_e69192 - locals.var_qbs_ac);
            (locals.var_vsginr, locals.var_vsginr_dn5, locals.var_vsginr_dn6, locals.var_vsginr_dn7, locals.var_vsginr_dn8, ) = (assign54120_e69194, ((locals.var_vgb1_ac_dn5 - locals.var_vgsinr_dn5) - locals.var_qbs_ac_dn5), ((locals.var_vgb1_ac_dn6 - locals.var_vgsinr_dn6) - locals.var_qbs_ac_dn6), ((locals.var_vgb1_ac_dn7 - locals.var_vgsinr_dn7) - locals.var_qbs_ac_dn7), ((locals.var_vgb1_ac_dn8 - locals.var_vgsinr_dn8) - locals.var_qbs_ac_dn8), );
        }

        if (locals.var_guard1510 != 0.0) {
            let assign54130_e69200: f64 = (locals.var_dps_ac + locals.var_vgsinr);
            let assign54130_e69202: f64 = (assign54130_e69200 - locals.var_v_ds);
            (locals.var_vgdinr, locals.var_vgdinr_dn5, locals.var_vgdinr_dn6, locals.var_vgdinr_dn7, locals.var_vgdinr_dn8, ) = (assign54130_e69202, (locals.var_dps_ac_dn5 + locals.var_vgsinr_dn5), ((locals.var_dps_ac_dn6 + locals.var_vgsinr_dn6) - locals.var_v_ds_dn6), ((locals.var_dps_ac_dn7 + locals.var_vgsinr_dn7) - locals.var_v_ds_dn7), (locals.var_dps_ac_dn8 + locals.var_vgsinr_dn8), );
        }

        if (locals.var_guard1510 != 0.0) {
            let assign54140_e69208: f64 = (locals.var_vgb1_ac - locals.var_vgdinr);
            let assign54140_e69210: f64 = (assign54140_e69208 - locals.var_qbd_ac);
            (locals.var_vdginr, locals.var_vdginr_dn5, locals.var_vdginr_dn6, locals.var_vdginr_dn7, locals.var_vdginr_dn8, ) = (assign54140_e69210, ((locals.var_vgb1_ac_dn5 - locals.var_vgdinr_dn5) - locals.var_qbd_ac_dn5), ((locals.var_vgb1_ac_dn6 - locals.var_vgdinr_dn6) - locals.var_qbd_ac_dn6), ((locals.var_vgb1_ac_dn7 - locals.var_vgdinr_dn7) - locals.var_qbd_ac_dn7), ((locals.var_vgb1_ac_dn8 - locals.var_vgdinr_dn8) - locals.var_qbd_ac_dn8), );
        }

        let assign54150_e69215: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1516 = assign54150_e69215;

        if ((locals.var_guard1510 != 0.0) && (locals.var_guard1516 != 0.0)) {
            let assign54160_e69222: f64 = (locals.var_cinrd_i * locals.var_vgdinr);
            let assign54160_e69225: f64 = (locals.var_cinr_i * locals.var_vgsinr);
            let assign54160_e69226: f64 = (assign54160_e69222 + assign54160_e69225);
            let assign54160_e69227: f64 = (locals.var_finr * assign54160_e69226);
            (locals.var_qginr, locals.var_qginr_dn5, locals.var_qginr_dn6, locals.var_qginr_dn7, locals.var_qginr_dn8, ) = (assign54160_e69227, ((locals.var_finr_dn5 * assign54160_e69226) + (locals.var_finr * ((locals.var_cinrd_i * locals.var_vgdinr_dn5) + (locals.var_cinr_i * locals.var_vgsinr_dn5)))), ((locals.var_finr_dn6 * assign54160_e69226) + (locals.var_finr * ((locals.var_cinrd_i * locals.var_vgdinr_dn6) + (locals.var_cinr_i * locals.var_vgsinr_dn6)))), ((locals.var_finr_dn7 * assign54160_e69226) + (locals.var_finr * ((locals.var_cinrd_i * locals.var_vgdinr_dn7) + (locals.var_cinr_i * locals.var_vgsinr_dn7)))), ((locals.var_finr_dn8 * assign54160_e69226) + (locals.var_finr * ((locals.var_cinrd_i * locals.var_vgdinr_dn8) + (locals.var_cinr_i * locals.var_vgsinr_dn8)))), );
        }

        if ((locals.var_guard1510 != 0.0) && (locals.var_guard1516 != 0.0)) {
            let assign54170_e69236: f64 = (locals.var_vsginr - locals.var_dvinr);
            let assign54170_e69237: f64 = (locals.var_cinr_i * assign54170_e69236);
            (locals.var_qsinr, locals.var_qsinr_dn5, locals.var_qsinr_dn6, locals.var_qsinr_dn7, locals.var_qsinr_dn8, ) = (assign54170_e69237, (locals.var_cinr_i * (locals.var_vsginr_dn5 - locals.var_dvinr_dn5)), (locals.var_cinr_i * (locals.var_vsginr_dn6 - locals.var_dvinr_dn6)), (locals.var_cinr_i * (locals.var_vsginr_dn7 - locals.var_dvinr_dn7)), (locals.var_cinr_i * (locals.var_vsginr_dn8 - locals.var_dvinr_dn8)), );
        }

        if ((locals.var_guard1510 != 0.0) && (locals.var_guard1516 != 0.0)) {
            let assign54180_e69246: f64 = (locals.var_vdginr - locals.var_dvinr);
            let assign54180_e69247: f64 = (locals.var_cinrd_i * assign54180_e69246);
            (locals.var_qdinr, locals.var_qdinr_dn5, locals.var_qdinr_dn6, locals.var_qdinr_dn7, locals.var_qdinr_dn8, ) = (assign54180_e69247, (locals.var_cinrd_i * (locals.var_vdginr_dn5 - locals.var_dvinr_dn5)), (locals.var_cinrd_i * (locals.var_vdginr_dn6 - locals.var_dvinr_dn6)), (locals.var_cinrd_i * (locals.var_vdginr_dn7 - locals.var_dvinr_dn7)), (locals.var_cinrd_i * (locals.var_vdginr_dn8 - locals.var_dvinr_dn8)), );
        }

        if ((locals.var_guard1510 != 0.0) && (locals.var_guard1516 == 0.0)) {
            let assign54190_e69257: f64 = (locals.var_cinr_i * locals.var_vgdinr);
            let assign54190_e69260: f64 = (locals.var_cinrd_i * locals.var_vgsinr);
            let assign54190_e69261: f64 = (assign54190_e69257 + assign54190_e69260);
            let assign54190_e69262: f64 = (locals.var_finr * assign54190_e69261);
            (locals.var_qginr, locals.var_qginr_dn5, locals.var_qginr_dn6, locals.var_qginr_dn7, locals.var_qginr_dn8, ) = (assign54190_e69262, ((locals.var_finr_dn5 * assign54190_e69261) + (locals.var_finr * ((locals.var_cinr_i * locals.var_vgdinr_dn5) + (locals.var_cinrd_i * locals.var_vgsinr_dn5)))), ((locals.var_finr_dn6 * assign54190_e69261) + (locals.var_finr * ((locals.var_cinr_i * locals.var_vgdinr_dn6) + (locals.var_cinrd_i * locals.var_vgsinr_dn6)))), ((locals.var_finr_dn7 * assign54190_e69261) + (locals.var_finr * ((locals.var_cinr_i * locals.var_vgdinr_dn7) + (locals.var_cinrd_i * locals.var_vgsinr_dn7)))), ((locals.var_finr_dn8 * assign54190_e69261) + (locals.var_finr * ((locals.var_cinr_i * locals.var_vgdinr_dn8) + (locals.var_cinrd_i * locals.var_vgsinr_dn8)))), );
        }

        if ((locals.var_guard1510 != 0.0) && (locals.var_guard1516 == 0.0)) {
            let assign54200_e69272: f64 = (locals.var_vsginr - locals.var_dvinr);
            let assign54200_e69273: f64 = (locals.var_cinrd_i * assign54200_e69272);
            (locals.var_qsinr, locals.var_qsinr_dn5, locals.var_qsinr_dn6, locals.var_qsinr_dn7, locals.var_qsinr_dn8, ) = (assign54200_e69273, (locals.var_cinrd_i * (locals.var_vsginr_dn5 - locals.var_dvinr_dn5)), (locals.var_cinrd_i * (locals.var_vsginr_dn6 - locals.var_dvinr_dn6)), (locals.var_cinrd_i * (locals.var_vsginr_dn7 - locals.var_dvinr_dn7)), (locals.var_cinrd_i * (locals.var_vsginr_dn8 - locals.var_dvinr_dn8)), );
        }

        if ((locals.var_guard1510 != 0.0) && (locals.var_guard1516 == 0.0)) {
            let assign54210_e69283: f64 = (locals.var_vdginr - locals.var_dvinr);
            let assign54210_e69284: f64 = (locals.var_cinr_i * assign54210_e69283);
            (locals.var_qdinr, locals.var_qdinr_dn5, locals.var_qdinr_dn6, locals.var_qdinr_dn7, locals.var_qdinr_dn8, ) = (assign54210_e69284, (locals.var_cinr_i * (locals.var_vdginr_dn5 - locals.var_dvinr_dn5)), (locals.var_cinr_i * (locals.var_vdginr_dn6 - locals.var_dvinr_dn6)), (locals.var_cinr_i * (locals.var_vdginr_dn7 - locals.var_dvinr_dn7)), (locals.var_cinr_i * (locals.var_vdginr_dn8 - locals.var_dvinr_dn8)), );
        }

        if (locals.var_guard1510 != 0.0) {
            let assign54220_e69290: f64 = (locals.var_qg + locals.var_qginr);
            (locals.var_qg, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, ) = (assign54220_e69290, (locals.var_qg_dn5 + locals.var_qginr_dn5), (locals.var_qg_dn6 + locals.var_qginr_dn6), (locals.var_qg_dn7 + locals.var_qginr_dn7), (locals.var_qg_dn8 + locals.var_qginr_dn8), );
        }

        if (locals.var_guard1510 != 0.0) {
            let assign54230_e69296: f64 = (locals.var_qd + locals.var_qdinr);
            (locals.var_qd, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, ) = (assign54230_e69296, (locals.var_qd_dn5 + locals.var_qdinr_dn5), (locals.var_qd_dn6 + locals.var_qdinr_dn6), (locals.var_qd_dn7 + locals.var_qdinr_dn7), (locals.var_qd_dn8 + locals.var_qdinr_dn8), );
        }

        if (locals.var_guard1510 != 0.0) {
            let assign54240_e69302: f64 = (locals.var_qb - locals.var_qginr);
            let assign54240_e69304: f64 = (assign54240_e69302 - locals.var_qdinr);
            let assign54240_e69306: f64 = (assign54240_e69304 - locals.var_qsinr);
            (locals.var_qb, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, ) = (assign54240_e69306, (((locals.var_qb_dn5 - locals.var_qginr_dn5) - locals.var_qdinr_dn5) - locals.var_qsinr_dn5), (((locals.var_qb_dn6 - locals.var_qginr_dn6) - locals.var_qdinr_dn6) - locals.var_qsinr_dn6), (((locals.var_qb_dn7 - locals.var_qginr_dn7) - locals.var_qdinr_dn7) - locals.var_qsinr_dn7), (((locals.var_qb_dn8 - locals.var_qginr_dn8) - locals.var_qdinr_dn8) - locals.var_qsinr_dn8), );
        }

        (locals.var_qg_ov_s, locals.var_qg_ov_s_dn5, locals.var_qg_ov_s_dn6, locals.var_qg_ov_s_dn7, locals.var_qg_ov_s_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_yb_ov_s, locals.var_yb_ov_s_dn5, locals.var_yb_ov_s_dn6, locals.var_yb_ov_s_dn7, locals.var_yb_ov_s_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign54290_e69323: f64 = if ((locals.var_cgov_i > 0.0) && (locals.var_fcgovacc_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1517 = assign54290_e69323;

        if (locals.var_guard1517 != 0.0) {
            let assign54300_e69328: f64 = (0.5 * locals.var_xgb_ov);
            let assign54300_e69330: f64 = (assign54300_e69328 + locals.var_dxgb_ov_s);
            let assign54300_e69331: f64 = (locals.var_cgovaccg_i * assign54300_e69330);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign54300_e69331, (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn5)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn6)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn7)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn8)), );
        }

        let assign54310_e69336: f64 = if locals.var_temp__blk936 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1518 = assign54310_e69336;

        let assign54320_e69339: f64 = (-230.25850929940458);
        let assign54320_e69340: f64 = if locals.var_temp__blk936 > assign54320_e69339 { 1.0 } else { 0.0 };
        locals.var_guard1519 = assign54320_e69340;

        if (((locals.var_guard1517 != 0.0) && (locals.var_guard1518 != 0.0)) && (locals.var_guard1519 != 0.0)) {
            let assign54330_e69347: f64 = (locals.var_temp__blk936).exp();
            (locals.var_yb_ov_s, locals.var_yb_ov_s_dn5, locals.var_yb_ov_s_dn6, locals.var_yb_ov_s_dn7, locals.var_yb_ov_s_dn8, ) = (assign54330_e69347, (assign54330_e69347 * locals.var_temp__blk936_dn5), (assign54330_e69347 * locals.var_temp__blk936_dn6), (assign54330_e69347 * locals.var_temp__blk936_dn7), (assign54330_e69347 * locals.var_temp__blk936_dn8), );
        }

    }

    pub(super) fn stamp_transient_block_27(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
            (locals.var_yb_ov_s, locals.var_yb_ov_s_dn5, locals.var_yb_ov_s_dn6, locals.var_yb_ov_s_dn7, locals.var_yb_ov_s_dn8, ) = (assign54340_e69381, (-((1e-100 * (((-locals.var_temp__blk936_dn5) * assign54340_e69378) + (assign54340_e69361 * (0.5 * (((-locals.var_temp__blk936_dn5) * assign54340_e69375) + (assign54340_e69367 * ((-locals.var_temp__blk936_dn5) * 0.3333333333333333))))))) / (assign54340_e69380 * assign54340_e69380))), (-((1e-100 * (((-locals.var_temp__blk936_dn6) * assign54340_e69378) + (assign54340_e69361 * (0.5 * (((-locals.var_temp__blk936_dn6) * assign54340_e69375) + (assign54340_e69367 * ((-locals.var_temp__blk936_dn6) * 0.3333333333333333))))))) / (assign54340_e69380 * assign54340_e69380))), (-((1e-100 * (((-locals.var_temp__blk936_dn7) * assign54340_e69378) + (assign54340_e69361 * (0.5 * (((-locals.var_temp__blk936_dn7) * assign54340_e69375) + (assign54340_e69367 * ((-locals.var_temp__blk936_dn7) * 0.3333333333333333))))))) / (assign54340_e69380 * assign54340_e69380))), (-((1e-100 * (((-locals.var_temp__blk936_dn8) * assign54340_e69378) + (assign54340_e69361 * (0.5 * (((-locals.var_temp__blk936_dn8) * assign54340_e69375) + (assign54340_e69367 * ((-locals.var_temp__blk936_dn8) * 0.3333333333333333))))))) / (assign54340_e69380 * assign54340_e69380))), );
        }

        let assign54350_e69386: f64 = if locals.var_yb_ov_s > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1520 = assign54350_e69386;

        if (((locals.var_guard1517 != 0.0) && (locals.var_guard1518 != 0.0)) && (locals.var_guard1520 != 0.0)) {
            let assign54360_e69394: f64 = (1.0 + locals.var_yb_ov_s);
            let assign54360_e69395: f64 = (assign54360_e69394).ln();
            (locals.var_xgbeff_ov_s, locals.var_xgbeff_ov_s_dn5, locals.var_xgbeff_ov_s_dn6, locals.var_xgbeff_ov_s_dn7, locals.var_xgbeff_ov_s_dn8, ) = (assign54360_e69395, (locals.var_yb_ov_s_dn5 / assign54360_e69394), (locals.var_yb_ov_s_dn6 / assign54360_e69394), (locals.var_yb_ov_s_dn7 / assign54360_e69394), (locals.var_yb_ov_s_dn8 / assign54360_e69394), );
        }

        if (((locals.var_guard1517 != 0.0) && (locals.var_guard1518 != 0.0)) && (locals.var_guard1520 != 0.0)) {
            let assign54370_e69407: f64 = (1.0 + locals.var_xgbeff_ov_s);
            let assign54370_e69408: f64 = (assign54370_e69407).ln();
            let assign54370_e69411: f64 = (2.0 + locals.var_xgbeff_ov_s);
            let assign54370_e69412: f64 = (assign54370_e69408 / assign54370_e69411);
            let assign54370_e69413: f64 = (1.0 - assign54370_e69412);
            let assign54370_e69414: f64 = (locals.var_xgbeff_ov_s * assign54370_e69413);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign54370_e69414, ((locals.var_xgbeff_ov_s_dn5 * assign54370_e69413) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn5 / assign54370_e69407) * assign54370_e69411) - (assign54370_e69408 * locals.var_xgbeff_ov_s_dn5)) / (assign54370_e69411 * assign54370_e69411))))), ((locals.var_xgbeff_ov_s_dn6 * assign54370_e69413) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn6 / assign54370_e69407) * assign54370_e69411) - (assign54370_e69408 * locals.var_xgbeff_ov_s_dn6)) / (assign54370_e69411 * assign54370_e69411))))), ((locals.var_xgbeff_ov_s_dn7 * assign54370_e69413) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn7 / assign54370_e69407) * assign54370_e69411) - (assign54370_e69408 * locals.var_xgbeff_ov_s_dn7)) / (assign54370_e69411 * assign54370_e69411))))), ((locals.var_xgbeff_ov_s_dn8 * assign54370_e69413) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn8 / assign54370_e69407) * assign54370_e69411) - (assign54370_e69408 * locals.var_xgbeff_ov_s_dn8)) / (assign54370_e69411 * assign54370_e69411))))), );
        }

        if (((locals.var_guard1517 != 0.0) && (locals.var_guard1518 != 0.0)) && (locals.var_guard1520 == 0.0)) {
            (locals.var_xgbeff_ov_s, locals.var_xgbeff_ov_s_dn5, locals.var_xgbeff_ov_s_dn6, locals.var_xgbeff_ov_s_dn7, locals.var_xgbeff_ov_s_dn8, ) = (locals.var_yb_ov_s, locals.var_yb_ov_s_dn5, locals.var_yb_ov_s_dn6, locals.var_yb_ov_s_dn7, locals.var_yb_ov_s_dn8, );
        }

        if (((locals.var_guard1517 != 0.0) && (locals.var_guard1518 != 0.0)) && (locals.var_guard1520 == 0.0)) {
            let assign54390_e69434: f64 = (2.0 * locals.var_xgbeff_ov_s);
            let assign54390_e69437: f64 = (2.0 + locals.var_xgbeff_ov_s);
            let assign54390_e69438: f64 = (assign54390_e69434 / assign54390_e69437);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign54390_e69438, ((((2.0 * locals.var_xgbeff_ov_s_dn5) * assign54390_e69437) - (assign54390_e69434 * locals.var_xgbeff_ov_s_dn5)) / (assign54390_e69437 * assign54390_e69437)), ((((2.0 * locals.var_xgbeff_ov_s_dn6) * assign54390_e69437) - (assign54390_e69434 * locals.var_xgbeff_ov_s_dn6)) / (assign54390_e69437 * assign54390_e69437)), ((((2.0 * locals.var_xgbeff_ov_s_dn7) * assign54390_e69437) - (assign54390_e69434 * locals.var_xgbeff_ov_s_dn7)) / (assign54390_e69437 * assign54390_e69437)), ((((2.0 * locals.var_xgbeff_ov_s_dn8) * assign54390_e69437) - (assign54390_e69434 * locals.var_xgbeff_ov_s_dn8)) / (assign54390_e69437 * assign54390_e69437)), );
        }

        if ((locals.var_guard1517 != 0.0) && (locals.var_guard1518 == 0.0)) {
            (locals.var_xgbeff_ov_s, locals.var_xgbeff_ov_s_dn5, locals.var_xgbeff_ov_s_dn6, locals.var_xgbeff_ov_s_dn7, locals.var_xgbeff_ov_s_dn8, ) = (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, );
        }

        if ((locals.var_guard1517 != 0.0) && (locals.var_guard1518 == 0.0)) {
            let assign54410_e69456: f64 = (1.0 + locals.var_xgbeff_ov_s);
            let assign54410_e69457: f64 = (assign54410_e69456).ln();
            let assign54410_e69460: f64 = (2.0 + locals.var_xgbeff_ov_s);
            let assign54410_e69461: f64 = (assign54410_e69457 / assign54410_e69460);
            let assign54410_e69462: f64 = (1.0 - assign54410_e69461);
            let assign54410_e69463: f64 = (locals.var_xgbeff_ov_s * assign54410_e69462);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign54410_e69463, ((locals.var_xgbeff_ov_s_dn5 * assign54410_e69462) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn5 / assign54410_e69456) * assign54410_e69460) - (assign54410_e69457 * locals.var_xgbeff_ov_s_dn5)) / (assign54410_e69460 * assign54410_e69460))))), ((locals.var_xgbeff_ov_s_dn6 * assign54410_e69462) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn6 / assign54410_e69456) * assign54410_e69460) - (assign54410_e69457 * locals.var_xgbeff_ov_s_dn6)) / (assign54410_e69460 * assign54410_e69460))))), ((locals.var_xgbeff_ov_s_dn7 * assign54410_e69462) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn7 / assign54410_e69456) * assign54410_e69460) - (assign54410_e69457 * locals.var_xgbeff_ov_s_dn7)) / (assign54410_e69460 * assign54410_e69460))))), ((locals.var_xgbeff_ov_s_dn8 * assign54410_e69462) + (locals.var_xgbeff_ov_s * (-((((locals.var_xgbeff_ov_s_dn8 / assign54410_e69456) * assign54410_e69460) - (assign54410_e69457 * locals.var_xgbeff_ov_s_dn8)) / (assign54410_e69460 * assign54410_e69460))))), );
        }

        if (locals.var_guard1517 != 0.0) {
            let assign54420_e69468: f64 = (-2.0);
            let assign54420_e69470: f64 = (assign54420_e69468 * locals.var_fcgovacc_i);
            let assign54420_e69472: f64 = (assign54420_e69470 / locals.var_cgovaccg_i);
            let assign54420_e69474: f64 = (assign54420_e69472 * locals.var_cgov_i);
            let assign54420_e69476: f64 = (assign54420_e69474 * locals.var_phita);
            let assign54420_e69478: f64 = (assign54420_e69476 * locals.var_temp1);
            (locals.var_qg_ov_s, locals.var_qg_ov_s_dn5, locals.var_qg_ov_s_dn6, locals.var_qg_ov_s_dn7, locals.var_qg_ov_s_dn8, ) = (assign54420_e69478, (assign54420_e69476 * locals.var_temp1_dn5), (assign54420_e69476 * locals.var_temp1_dn6), (assign54420_e69476 * locals.var_temp1_dn7), (assign54420_e69476 * locals.var_temp1_dn8), );
        }

        (locals.var_qg_ov_d, locals.var_qg_ov_d_dn5, locals.var_qg_ov_d_dn6, locals.var_qg_ov_d_dn7, locals.var_qg_ov_d_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_yb_ov_d, locals.var_yb_ov_d_dn5, locals.var_yb_ov_d_dn6, locals.var_yb_ov_d_dn7, locals.var_yb_ov_d_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign54450_e69489: f64 = if ((locals.var_cgovd_i > 0.0) && (locals.var_fcgovaccd_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1521 = assign54450_e69489;

        if (locals.var_guard1521 != 0.0) {
            let assign54460_e69494: f64 = (0.5 * locals.var_xgb_ov);
            let assign54460_e69496: f64 = (assign54460_e69494 + locals.var_dxgb_ov_d);
            let assign54460_e69497: f64 = (locals.var_cgovaccg_i * assign54460_e69496);
            (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, ) = (assign54460_e69497, (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn5)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn6)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn7)), (locals.var_cgovaccg_i * (0.5 * locals.var_xgb_ov_dn8)), );
        }

        let assign54470_e69502: f64 = if locals.var_temp__blk936 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1522 = assign54470_e69502;

        let assign54480_e69505: f64 = (-230.25850929940458);
        let assign54480_e69506: f64 = if locals.var_temp__blk936 > assign54480_e69505 { 1.0 } else { 0.0 };
        locals.var_guard1523 = assign54480_e69506;

        if (((locals.var_guard1521 != 0.0) && (locals.var_guard1522 != 0.0)) && (locals.var_guard1523 != 0.0)) {
            let assign54490_e69513: f64 = (locals.var_temp__blk936).exp();
            (locals.var_yb_ov_d, locals.var_yb_ov_d_dn5, locals.var_yb_ov_d_dn6, locals.var_yb_ov_d_dn7, locals.var_yb_ov_d_dn8, ) = (assign54490_e69513, (assign54490_e69513 * locals.var_temp__blk936_dn5), (assign54490_e69513 * locals.var_temp__blk936_dn6), (assign54490_e69513 * locals.var_temp__blk936_dn7), (assign54490_e69513 * locals.var_temp__blk936_dn8), );
        }

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
            (locals.var_yb_ov_d, locals.var_yb_ov_d_dn5, locals.var_yb_ov_d_dn6, locals.var_yb_ov_d_dn7, locals.var_yb_ov_d_dn8, ) = (assign54500_e69547, (-((1e-100 * (((-locals.var_temp__blk936_dn5) * assign54500_e69544) + (assign54500_e69527 * (0.5 * (((-locals.var_temp__blk936_dn5) * assign54500_e69541) + (assign54500_e69533 * ((-locals.var_temp__blk936_dn5) * 0.3333333333333333))))))) / (assign54500_e69546 * assign54500_e69546))), (-((1e-100 * (((-locals.var_temp__blk936_dn6) * assign54500_e69544) + (assign54500_e69527 * (0.5 * (((-locals.var_temp__blk936_dn6) * assign54500_e69541) + (assign54500_e69533 * ((-locals.var_temp__blk936_dn6) * 0.3333333333333333))))))) / (assign54500_e69546 * assign54500_e69546))), (-((1e-100 * (((-locals.var_temp__blk936_dn7) * assign54500_e69544) + (assign54500_e69527 * (0.5 * (((-locals.var_temp__blk936_dn7) * assign54500_e69541) + (assign54500_e69533 * ((-locals.var_temp__blk936_dn7) * 0.3333333333333333))))))) / (assign54500_e69546 * assign54500_e69546))), (-((1e-100 * (((-locals.var_temp__blk936_dn8) * assign54500_e69544) + (assign54500_e69527 * (0.5 * (((-locals.var_temp__blk936_dn8) * assign54500_e69541) + (assign54500_e69533 * ((-locals.var_temp__blk936_dn8) * 0.3333333333333333))))))) / (assign54500_e69546 * assign54500_e69546))), );
        }

        let assign54510_e69552: f64 = if locals.var_yb_ov_d > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1524 = assign54510_e69552;

        if (((locals.var_guard1521 != 0.0) && (locals.var_guard1522 != 0.0)) && (locals.var_guard1524 != 0.0)) {
            let assign54520_e69560: f64 = (1.0 + locals.var_yb_ov_d);
            let assign54520_e69561: f64 = (assign54520_e69560).ln();
            (locals.var_xgbeff_ov_d, locals.var_xgbeff_ov_d_dn5, locals.var_xgbeff_ov_d_dn6, locals.var_xgbeff_ov_d_dn7, locals.var_xgbeff_ov_d_dn8, ) = (assign54520_e69561, (locals.var_yb_ov_d_dn5 / assign54520_e69560), (locals.var_yb_ov_d_dn6 / assign54520_e69560), (locals.var_yb_ov_d_dn7 / assign54520_e69560), (locals.var_yb_ov_d_dn8 / assign54520_e69560), );
        }

        if (((locals.var_guard1521 != 0.0) && (locals.var_guard1522 != 0.0)) && (locals.var_guard1524 != 0.0)) {
            let assign54530_e69573: f64 = (1.0 + locals.var_xgbeff_ov_d);
            let assign54530_e69574: f64 = (assign54530_e69573).ln();
            let assign54530_e69577: f64 = (2.0 + locals.var_xgbeff_ov_d);
            let assign54530_e69578: f64 = (assign54530_e69574 / assign54530_e69577);
            let assign54530_e69579: f64 = (1.0 - assign54530_e69578);
            let assign54530_e69580: f64 = (locals.var_xgbeff_ov_d * assign54530_e69579);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign54530_e69580, ((locals.var_xgbeff_ov_d_dn5 * assign54530_e69579) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn5 / assign54530_e69573) * assign54530_e69577) - (assign54530_e69574 * locals.var_xgbeff_ov_d_dn5)) / (assign54530_e69577 * assign54530_e69577))))), ((locals.var_xgbeff_ov_d_dn6 * assign54530_e69579) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn6 / assign54530_e69573) * assign54530_e69577) - (assign54530_e69574 * locals.var_xgbeff_ov_d_dn6)) / (assign54530_e69577 * assign54530_e69577))))), ((locals.var_xgbeff_ov_d_dn7 * assign54530_e69579) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn7 / assign54530_e69573) * assign54530_e69577) - (assign54530_e69574 * locals.var_xgbeff_ov_d_dn7)) / (assign54530_e69577 * assign54530_e69577))))), ((locals.var_xgbeff_ov_d_dn8 * assign54530_e69579) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn8 / assign54530_e69573) * assign54530_e69577) - (assign54530_e69574 * locals.var_xgbeff_ov_d_dn8)) / (assign54530_e69577 * assign54530_e69577))))), );
        }

        if (((locals.var_guard1521 != 0.0) && (locals.var_guard1522 != 0.0)) && (locals.var_guard1524 == 0.0)) {
            (locals.var_xgbeff_ov_d, locals.var_xgbeff_ov_d_dn5, locals.var_xgbeff_ov_d_dn6, locals.var_xgbeff_ov_d_dn7, locals.var_xgbeff_ov_d_dn8, ) = (locals.var_yb_ov_d, locals.var_yb_ov_d_dn5, locals.var_yb_ov_d_dn6, locals.var_yb_ov_d_dn7, locals.var_yb_ov_d_dn8, );
        }

        if (((locals.var_guard1521 != 0.0) && (locals.var_guard1522 != 0.0)) && (locals.var_guard1524 == 0.0)) {
            let assign54550_e69600: f64 = (2.0 * locals.var_xgbeff_ov_d);
            let assign54550_e69603: f64 = (2.0 + locals.var_xgbeff_ov_d);
            let assign54550_e69604: f64 = (assign54550_e69600 / assign54550_e69603);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign54550_e69604, ((((2.0 * locals.var_xgbeff_ov_d_dn5) * assign54550_e69603) - (assign54550_e69600 * locals.var_xgbeff_ov_d_dn5)) / (assign54550_e69603 * assign54550_e69603)), ((((2.0 * locals.var_xgbeff_ov_d_dn6) * assign54550_e69603) - (assign54550_e69600 * locals.var_xgbeff_ov_d_dn6)) / (assign54550_e69603 * assign54550_e69603)), ((((2.0 * locals.var_xgbeff_ov_d_dn7) * assign54550_e69603) - (assign54550_e69600 * locals.var_xgbeff_ov_d_dn7)) / (assign54550_e69603 * assign54550_e69603)), ((((2.0 * locals.var_xgbeff_ov_d_dn8) * assign54550_e69603) - (assign54550_e69600 * locals.var_xgbeff_ov_d_dn8)) / (assign54550_e69603 * assign54550_e69603)), );
        }

        if ((locals.var_guard1521 != 0.0) && (locals.var_guard1522 == 0.0)) {
            (locals.var_xgbeff_ov_d, locals.var_xgbeff_ov_d_dn5, locals.var_xgbeff_ov_d_dn6, locals.var_xgbeff_ov_d_dn7, locals.var_xgbeff_ov_d_dn8, ) = (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8, );
        }

        if ((locals.var_guard1521 != 0.0) && (locals.var_guard1522 == 0.0)) {
            let assign54570_e69622: f64 = (1.0 + locals.var_xgbeff_ov_d);
            let assign54570_e69623: f64 = (assign54570_e69622).ln();
            let assign54570_e69626: f64 = (2.0 + locals.var_xgbeff_ov_d);
            let assign54570_e69627: f64 = (assign54570_e69623 / assign54570_e69626);
            let assign54570_e69628: f64 = (1.0 - assign54570_e69627);
            let assign54570_e69629: f64 = (locals.var_xgbeff_ov_d * assign54570_e69628);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign54570_e69629, ((locals.var_xgbeff_ov_d_dn5 * assign54570_e69628) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn5 / assign54570_e69622) * assign54570_e69626) - (assign54570_e69623 * locals.var_xgbeff_ov_d_dn5)) / (assign54570_e69626 * assign54570_e69626))))), ((locals.var_xgbeff_ov_d_dn6 * assign54570_e69628) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn6 / assign54570_e69622) * assign54570_e69626) - (assign54570_e69623 * locals.var_xgbeff_ov_d_dn6)) / (assign54570_e69626 * assign54570_e69626))))), ((locals.var_xgbeff_ov_d_dn7 * assign54570_e69628) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn7 / assign54570_e69622) * assign54570_e69626) - (assign54570_e69623 * locals.var_xgbeff_ov_d_dn7)) / (assign54570_e69626 * assign54570_e69626))))), ((locals.var_xgbeff_ov_d_dn8 * assign54570_e69628) + (locals.var_xgbeff_ov_d * (-((((locals.var_xgbeff_ov_d_dn8 / assign54570_e69622) * assign54570_e69626) - (assign54570_e69623 * locals.var_xgbeff_ov_d_dn8)) / (assign54570_e69626 * assign54570_e69626))))), );
        }

        if (locals.var_guard1521 != 0.0) {
            let assign54580_e69634: f64 = (-2.0);
            let assign54580_e69636: f64 = (assign54580_e69634 * locals.var_fcgovaccd_i);
            let assign54580_e69638: f64 = (assign54580_e69636 / locals.var_cgovaccg_i);
            let assign54580_e69640: f64 = (assign54580_e69638 * locals.var_cgovd_i);
            let assign54580_e69642: f64 = (assign54580_e69640 * locals.var_phita);
            let assign54580_e69644: f64 = (assign54580_e69642 * locals.var_temp1);
            (locals.var_qg_ov_d, locals.var_qg_ov_d_dn5, locals.var_qg_ov_d_dn6, locals.var_qg_ov_d_dn7, locals.var_qg_ov_d_dn8, ) = (assign54580_e69644, (assign54580_e69642 * locals.var_temp1_dn5), (assign54580_e69642 * locals.var_temp1_dn6), (assign54580_e69642 * locals.var_temp1_dn7), (assign54580_e69642 * locals.var_temp1_dn8), );
        }

        let assign54590_e69649: f64 = (locals.var_qg_ov_s + locals.var_qg_ov_d);
        (locals.var_qg_ov, locals.var_qg_ov_dn5, locals.var_qg_ov_dn6, locals.var_qg_ov_dn7, locals.var_qg_ov_dn8, ) = (assign54590_e69649, (locals.var_qg_ov_s_dn5 + locals.var_qg_ov_d_dn5), (locals.var_qg_ov_s_dn6 + locals.var_qg_ov_d_dn6), (locals.var_qg_ov_s_dn7 + locals.var_qg_ov_d_dn7), (locals.var_qg_ov_s_dn8 + locals.var_qg_ov_d_dn8), );

        let assign54600_e69652: f64 = (locals.var_cgbov_i * locals.var_vgb);
        let assign54600_e69654: f64 = (assign54600_e69652 + locals.var_qg_ov);
        (locals.var_qgb_ov, locals.var_qgb_ov_dn5, locals.var_qgb_ov_dn6, locals.var_qgb_ov_dn7, locals.var_qgb_ov_dn8, ) = (assign54600_e69654, ((locals.var_cgbov_i * locals.var_vgb_dn5) + locals.var_qg_ov_dn5), ((locals.var_cgbov_i * locals.var_vgb_dn6) + locals.var_qg_ov_dn6), ((locals.var_cgbov_i * locals.var_vgb_dn7) + locals.var_qg_ov_dn7), ((locals.var_cgbov_i * locals.var_vgb_dn8) + locals.var_qg_ov_dn8), );

        let assign61890_e80509: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1718 = assign61890_e80509;

        let assign61970_e80533: f64 = (locals.var_qg + locals.var_qb);
        let assign61970_e80535: f64 = (assign61970_e80533 + locals.var_qd);
        let assign61970_e80536: f64 = (-assign61970_e80535);
        (locals.var_qs, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, ) = (assign61970_e80536, (-((locals.var_qg_dn5 + locals.var_qb_dn5) + locals.var_qd_dn5)), (-((locals.var_qg_dn6 + locals.var_qb_dn6) + locals.var_qd_dn6)), (-((locals.var_qg_dn7 + locals.var_qb_dn7) + locals.var_qd_dn7)), (-((locals.var_qg_dn8 + locals.var_qb_dn8) + locals.var_qd_dn8)), );

        let assign62020_e80567: f64 = if locals.var_sigvds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1727 = assign62020_e80567;

        if (locals.var_guard1727 != 0.0) {
            (locals.var_temp__blk1726, locals.var_temp__blk1726_dn5, locals.var_temp__blk1726_dn6, locals.var_temp__blk1726_dn7, locals.var_temp__blk1726_dn8, ) = (locals.var_qd, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, );
            (locals.var_qd, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, ) = (locals.var_qs, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, );
            (locals.var_qs, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, ) = (locals.var_temp__blk1726, locals.var_temp__blk1726_dn5, locals.var_temp__blk1726_dn6, locals.var_temp__blk1726_dn7, locals.var_temp__blk1726_dn8, );
        }

        (locals.var_sidexc, locals.var_sidexc_dn5, locals.var_sidexc_dn6, locals.var_sidexc_dn7, locals.var_sidexc_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_mid, locals.var_mid_dn5, locals.var_mid_dn6, locals.var_mid_dn7, locals.var_mid_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_mig, locals.var_mig_dn5, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8, ) = (1e-40, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_migid, locals.var_migid_dn5, locals.var_migid_dn6, locals.var_migid_dn7, locals.var_migid_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_c_igid, locals.var_c_igid_dn5, locals.var_c_igid_dn6, locals.var_c_igid_dn7, locals.var_c_igid_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign62120_e80588: f64 = (locals.var_cox_qm * locals.var_eta_p_ac);
        (locals.var_cgeff, locals.var_cgeff_dn5, locals.var_cgeff_dn6, locals.var_cgeff_dn7, locals.var_cgeff_dn8, ) = (assign62120_e80588, ((locals.var_cox_qm_dn5 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn5)), ((locals.var_cox_qm_dn6 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn6)), ((locals.var_cox_qm_dn7 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn7)), ((locals.var_cox_qm_dn8 * locals.var_eta_p_ac) + (locals.var_cox_qm * locals.var_eta_p_ac_dn8)), );

        (locals.var_sqid, locals.var_sqid_dn5, locals.var_sqid_dn6, locals.var_sqid_dn7, locals.var_sqid_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_sqig, locals.var_sqig_dn5, locals.var_sqig_dn6, locals.var_sqig_dn7, locals.var_sqig_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign62180_e80600: f64 = if ((locals.var_xg_dc > 0.0) && (locals.var_bet_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1760 = assign62180_e80600;

        let assign62270_e80706: f64 = if p.p32 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1762 = assign62270_e80706;

        if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
            let assign62280_e80712: f64 = (locals.var_qim1_dc / locals.var_alpha_dc);
            (locals.var_h0, locals.var_h0_dn5, locals.var_h0_dn6, locals.var_h0_dn7, locals.var_h0_dn8, ) = (assign62280_e80712, (((locals.var_qim1_dc_dn5 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn5)) / (locals.var_alpha_dc * locals.var_alpha_dc)), (((locals.var_qim1_dc_dn6 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn6)) / (locals.var_alpha_dc * locals.var_alpha_dc)), (((locals.var_qim1_dc_dn7 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn7)) / (locals.var_alpha_dc * locals.var_alpha_dc)), (((locals.var_qim1_dc_dn8 * locals.var_alpha_dc) - (locals.var_qim1_dc * locals.var_alpha_dc_dn8)) / (locals.var_alpha_dc * locals.var_alpha_dc)), );
        }

        if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
            let assign62290_e80720: f64 = (locals.var_qim_dc / locals.var_qim1_dc);
            (locals.var_t1, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, ) = (assign62290_e80720, (((locals.var_qim_dc_dn5 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn5)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((locals.var_qim_dc_dn6 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn6)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((locals.var_qim_dc_dn7 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn7)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((locals.var_qim_dc_dn8 * locals.var_qim1_dc) - (locals.var_qim_dc * locals.var_qim1_dc_dn8)) / (locals.var_qim1_dc * locals.var_qim1_dc)), );
        }

        if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
            let assign62300_e80728: f64 = (0.5 * 0.16666666666666666);
            let assign62300_e80731: f64 = (locals.var_dps_dc / locals.var_h0);
            let assign62300_e80732: f64 = (assign62300_e80728 * assign62300_e80731);
            (locals.var_sqt2, locals.var_sqt2_dn5, locals.var_sqt2_dn6, locals.var_sqt2_dn7, locals.var_sqt2_dn8, ) = (assign62300_e80732, (assign62300_e80728 * (((locals.var_dps_dc_dn5 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn5)) / (locals.var_h0 * locals.var_h0))), (assign62300_e80728 * (((locals.var_dps_dc_dn6 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn6)) / (locals.var_h0 * locals.var_h0))), (assign62300_e80728 * (((locals.var_dps_dc_dn7 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn7)) / (locals.var_h0 * locals.var_h0))), (assign62300_e80728 * (((locals.var_dps_dc_dn8 * locals.var_h0) - (locals.var_dps_dc * locals.var_h0_dn8)) / (locals.var_h0 * locals.var_h0))), );
        }

        if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
            let assign62310_e80740: f64 = (locals.var_sqt2 * locals.var_sqt2);
            (locals.var_t2, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, ) = (assign62310_e80740, ((locals.var_sqt2_dn5 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn5)), ((locals.var_sqt2_dn6 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn6)), ((locals.var_sqt2_dn7 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn7)), ((locals.var_sqt2_dn8 * locals.var_sqt2) + (locals.var_sqt2 * locals.var_sqt2_dn8)), );
        }

        if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
            let assign62320_e80748: f64 = (locals.var_h0 / locals.var_h_dc);
            let assign62320_e80750: f64 = (assign62320_e80748 - 1.0);
            (locals.var_r, locals.var_r_dn5, locals.var_r_dn6, locals.var_r_dn7, locals.var_r_dn8, ) = (assign62320_e80750, (((locals.var_h0_dn5 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn5)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_h0_dn6 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn6)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_h0_dn7 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn7)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_h0_dn8 * locals.var_h_dc) - (locals.var_h0 * locals.var_h_dc_dn8)) / (locals.var_h_dc * locals.var_h_dc)), );
        }

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
            (locals.var_lc, locals.var_lc_dn5, locals.var_lc_dn6, locals.var_lc_dn7, locals.var_lc_dn8, ) = (assign62330_e80773, assign62330_e80773_d_n5, assign62330_e80773_d_n6, assign62330_e80773_d_n7, assign62330_e80773_d_n8, );
        }

        if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
            let assign62340_e80782: f64 = (locals.var_lc * locals.var_lc);
            let assign62340_e80783: f64 = (1.0 / assign62340_e80782);
            (locals.var_lcinv2, locals.var_lcinv2_dn5, locals.var_lcinv2_dn6, locals.var_lcinv2_dn7, locals.var_lcinv2_dn8, ) = (assign62340_e80783, (-(((locals.var_lc_dn5 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn5)) / (assign62340_e80782 * assign62340_e80782))), (-(((locals.var_lc_dn6 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn6)) / (assign62340_e80782 * assign62340_e80782))), (-(((locals.var_lc_dn7 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn7)) / (assign62340_e80782 * assign62340_e80782))), (-(((locals.var_lc_dn8 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn8)) / (assign62340_e80782 * assign62340_e80782))), );
        }

        if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
            let assign62350_e80791: f64 = (locals.var_bet_i * locals.var_qim1_dc);
            let assign62350_e80793: f64 = (assign62350_e80791 * locals.var_gvsatinv_dc);
            (locals.var_g_ideal, locals.var_g_ideal_dn5, locals.var_g_ideal_dn6, locals.var_g_ideal_dn7, locals.var_g_ideal_dn8, ) = (assign62350_e80793, (((locals.var_bet_i * locals.var_qim1_dc_dn5) * locals.var_gvsatinv_dc) + (assign62350_e80791 * locals.var_gvsatinv_dc_dn5)), (((locals.var_bet_i * locals.var_qim1_dc_dn6) * locals.var_gvsatinv_dc) + (assign62350_e80791 * locals.var_gvsatinv_dc_dn6)), (((locals.var_bet_i * locals.var_qim1_dc_dn7) * locals.var_gvsatinv_dc) + (assign62350_e80791 * locals.var_gvsatinv_dc_dn7)), (((locals.var_bet_i * locals.var_qim1_dc_dn8) * locals.var_gvsatinv_dc) + (assign62350_e80791 * locals.var_gvsatinv_dc_dn8)), );
        }

        if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
            let assign62360_e80802: f64 = (12.0 * locals.var_t2);
            let assign62360_e80803: f64 = (locals.var_t1 + assign62360_e80802);
            let assign62360_e80807: f64 = (1.0 + locals.var_t1);
            let assign62360_e80809: f64 = (assign62360_e80807 * locals.var_t2);
            let assign62360_e80811: f64 = (assign62360_e80809 * locals.var_r);
            let assign62360_e80812: f64 = (24.0 * assign62360_e80811);
            let assign62360_e80813: f64 = (assign62360_e80803 - assign62360_e80812);
            (locals.var_mid, locals.var_mid_dn5, locals.var_mid_dn6, locals.var_mid_dn7, locals.var_mid_dn8, ) = (assign62360_e80813, ((locals.var_t1_dn5 + (12.0 * locals.var_t2_dn5)) - (24.0 * ((((locals.var_t1_dn5 * locals.var_t2) + (assign62360_e80807 * locals.var_t2_dn5)) * locals.var_r) + (assign62360_e80809 * locals.var_r_dn5)))), ((locals.var_t1_dn6 + (12.0 * locals.var_t2_dn6)) - (24.0 * ((((locals.var_t1_dn6 * locals.var_t2) + (assign62360_e80807 * locals.var_t2_dn6)) * locals.var_r) + (assign62360_e80809 * locals.var_r_dn6)))), ((locals.var_t1_dn7 + (12.0 * locals.var_t2_dn7)) - (24.0 * ((((locals.var_t1_dn7 * locals.var_t2) + (assign62360_e80807 * locals.var_t2_dn7)) * locals.var_r) + (assign62360_e80809 * locals.var_r_dn7)))), ((locals.var_t1_dn8 + (12.0 * locals.var_t2_dn8)) - (24.0 * ((((locals.var_t1_dn8 * locals.var_t2) + (assign62360_e80807 * locals.var_t2_dn8)) * locals.var_r) + (assign62360_e80809 * locals.var_r_dn8)))), );
        }

        if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
            let (assign62370_e80824, assign62370_e80824_d_n5, assign62370_e80824_d_n6, assign62370_e80824_d_n7, assign62370_e80824_d_n8,) = {
    if (locals.var_mid > 1e-40) {
        (locals.var_mid, locals.var_mid_dn5, locals.var_mid_dn6, locals.var_mid_dn7, locals.var_mid_dn8,)
    } else {
        (1e-40, 0.0, 0.0, 0.0, 0.0,)
    }
};
            (locals.var_mid, locals.var_mid_dn5, locals.var_mid_dn6, locals.var_mid_dn7, locals.var_mid_dn8, ) = (assign62370_e80824, assign62370_e80824_d_n5, assign62370_e80824_d_n6, assign62370_e80824_d_n7, assign62370_e80824_d_n8, );
        }

        if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
            let assign62380_e80832: f64 = (locals.var_g_ideal * locals.var_lcinv2);
            let assign62380_e80834: f64 = (assign62380_e80832 * locals.var_mid);
            (locals.var_mid, locals.var_mid_dn5, locals.var_mid_dn6, locals.var_mid_dn7, locals.var_mid_dn8, ) = (assign62380_e80834, ((((locals.var_g_ideal_dn5 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn5)) * locals.var_mid) + (assign62380_e80832 * locals.var_mid_dn5)), ((((locals.var_g_ideal_dn6 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn6)) * locals.var_mid) + (assign62380_e80832 * locals.var_mid_dn6)), ((((locals.var_g_ideal_dn7 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn7)) * locals.var_mid) + (assign62380_e80832 * locals.var_mid_dn7)), ((((locals.var_g_ideal_dn8 * locals.var_lcinv2) + (locals.var_g_ideal * locals.var_lcinv2_dn8)) * locals.var_mid) + (assign62380_e80832 * locals.var_mid_dn8)), );
        }

        let assign62390_e80839: f64 = if locals.var_fntexc_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1763 = assign62390_e80839;

        if (((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) && (locals.var_guard1763 != 0.0)) {
            let assign62400_e80847: f64 = (locals.var_thesateff_dc / locals.var_gmob_dc);
            (locals.var_thesat1_exc, locals.var_thesat1_exc_dn5, locals.var_thesat1_exc_dn6, locals.var_thesat1_exc_dn7, locals.var_thesat1_exc_dn8, ) = (assign62400_e80847, (((locals.var_thesateff_dc_dn5 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn5)) / (locals.var_gmob_dc * locals.var_gmob_dc)), (((locals.var_thesateff_dc_dn6 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn6)) / (locals.var_gmob_dc * locals.var_gmob_dc)), (((locals.var_thesateff_dc_dn7 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn7)) / (locals.var_gmob_dc * locals.var_gmob_dc)), (((locals.var_thesateff_dc_dn8 * locals.var_gmob_dc) - (locals.var_thesateff_dc * locals.var_gmob_dc_dn8)) / (locals.var_gmob_dc * locals.var_gmob_dc)), );
        }

        if (((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) && (locals.var_guard1763 != 0.0)) {
            let assign62410_e80857: f64 = (locals.var_thesat1_exc * locals.var_thesat1_exc);
            let assign62410_e80859: f64 = (assign62410_e80857 * locals.var_dps_dc);
            let assign62410_e80861: f64 = (assign62410_e80859 * locals.var_dps_dc);
            (locals.var_zsat_exc, locals.var_zsat_exc_dn5, locals.var_zsat_exc_dn6, locals.var_zsat_exc_dn7, locals.var_zsat_exc_dn8, ) = (assign62410_e80861, ((((((locals.var_thesat1_exc_dn5 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn5)) * locals.var_dps_dc) + (assign62410_e80857 * locals.var_dps_dc_dn5)) * locals.var_dps_dc) + (assign62410_e80859 * locals.var_dps_dc_dn5)), ((((((locals.var_thesat1_exc_dn6 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn6)) * locals.var_dps_dc) + (assign62410_e80857 * locals.var_dps_dc_dn6)) * locals.var_dps_dc) + (assign62410_e80859 * locals.var_dps_dc_dn6)), ((((((locals.var_thesat1_exc_dn7 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn7)) * locals.var_dps_dc) + (assign62410_e80857 * locals.var_dps_dc_dn7)) * locals.var_dps_dc) + (assign62410_e80859 * locals.var_dps_dc_dn7)), ((((((locals.var_thesat1_exc_dn8 * locals.var_thesat1_exc) + (locals.var_thesat1_exc * locals.var_thesat1_exc_dn8)) * locals.var_dps_dc) + (assign62410_e80857 * locals.var_dps_dc_dn8)) * locals.var_dps_dc) + (assign62410_e80859 * locals.var_dps_dc_dn8)), );
        }

        let assign62420_e80866: f64 = (-1.0);
        let assign62420_e80867: f64 = if locals.var_chnl_type == assign62420_e80866 { 1.0 } else { 0.0 };
        locals.var_guard1764 = assign62420_e80867;

        if ((((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) && (locals.var_guard1763 != 0.0)) && (locals.var_guard1764 != 0.0)) {
            let assign62430_e80879: f64 = (locals.var_thesat1_exc * locals.var_dps_dc);
            let assign62430_e80880: f64 = (1.0 + assign62430_e80879);
            let assign62430_e80881: f64 = (locals.var_zsat_exc / assign62430_e80880);
            (locals.var_zsat_exc, locals.var_zsat_exc_dn5, locals.var_zsat_exc_dn6, locals.var_zsat_exc_dn7, locals.var_zsat_exc_dn8, ) = (assign62430_e80881, (((locals.var_zsat_exc_dn5 * assign62430_e80880) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn5 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn5)))) / (assign62430_e80880 * assign62430_e80880)), (((locals.var_zsat_exc_dn6 * assign62430_e80880) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn6 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn6)))) / (assign62430_e80880 * assign62430_e80880)), (((locals.var_zsat_exc_dn7 * assign62430_e80880) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn7 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn7)))) / (assign62430_e80880 * assign62430_e80880)), (((locals.var_zsat_exc_dn8 * assign62430_e80880) - (locals.var_zsat_exc * ((locals.var_thesat1_exc_dn8 * locals.var_dps_dc) + (locals.var_thesat1_exc * locals.var_dps_dc_dn8)))) / (assign62430_e80880 * assign62430_e80880)), );
        }

        if (((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) && (locals.var_guard1763 != 0.0)) {
            let assign62440_e80895: f64 = (2.0 * locals.var_zsat_exc);
            let assign62440_e80896: f64 = (1.0 + assign62440_e80895);
            let assign62440_e80897: f64 = (assign62440_e80896).sqrt();
            let assign62440_e80898: f64 = (1.0 + assign62440_e80897);
            let assign62440_e80899: f64 = (locals.var_gmob_dc * assign62440_e80898);
            let assign62440_e80900: f64 = (0.5 * assign62440_e80899);
            (locals.var_gvsat_exc, locals.var_gvsat_exc_dn5, locals.var_gvsat_exc_dn6, locals.var_gvsat_exc_dn7, locals.var_gvsat_exc_dn8, ) = (assign62440_e80900, (0.5 * ((locals.var_gmob_dc_dn5 * assign62440_e80898) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn5) / (2.0 * assign62440_e80897))))), (0.5 * ((locals.var_gmob_dc_dn6 * assign62440_e80898) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn6) / (2.0 * assign62440_e80897))))), (0.5 * ((locals.var_gmob_dc_dn7 * assign62440_e80898) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn7) / (2.0 * assign62440_e80897))))), (0.5 * ((locals.var_gmob_dc_dn8 * assign62440_e80898) + (locals.var_gmob_dc * ((2.0 * locals.var_zsat_exc_dn8) / (2.0 * assign62440_e80897))))), );
        }

        if (((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) && (locals.var_guard1763 != 0.0)) {
            let assign62450_e80911: f64 = (locals.var_gvsat_exc * locals.var_lc);
            let assign62450_e80912: f64 = (locals.var_gmob_dc / assign62450_e80911);
            (locals.var_gfac, locals.var_gfac_dn5, locals.var_gfac_dn6, locals.var_gfac_dn7, locals.var_gfac_dn8, ) = (assign62450_e80912, (((locals.var_gmob_dc_dn5 * assign62450_e80911) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn5 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn5)))) / (assign62450_e80911 * assign62450_e80911)), (((locals.var_gmob_dc_dn6 * assign62450_e80911) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn6 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn6)))) / (assign62450_e80911 * assign62450_e80911)), (((locals.var_gmob_dc_dn7 * assign62450_e80911) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn7 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn7)))) / (assign62450_e80911 * assign62450_e80911)), (((locals.var_gmob_dc_dn8 * assign62450_e80911) - (locals.var_gmob_dc * ((locals.var_gvsat_exc_dn8 * locals.var_lc) + (locals.var_gvsat_exc * locals.var_lc_dn8)))) / (assign62450_e80911 * assign62450_e80911)), );
        }

        if (((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) && (locals.var_guard1763 != 0.0)) {
            let assign62460_e80922: f64 = (locals.var_fac_exc * locals.var_i_ds);
            let assign62460_e80924: f64 = (assign62460_e80922 * locals.var_vdse_dc);
            let assign62460_e80926: f64 = (assign62460_e80924 * locals.var_gfac);
            let assign62460_e80928: f64 = (assign62460_e80926 * locals.var_gfac);
            (locals.var_sidexc, locals.var_sidexc_dn5, locals.var_sidexc_dn6, locals.var_sidexc_dn7, locals.var_sidexc_dn8, ) = (assign62460_e80928, (((((((locals.var_fac_exc * locals.var_i_ds_dn5) * locals.var_vdse_dc) + (assign62460_e80922 * locals.var_vdse_dc_dn5)) * locals.var_gfac) + (assign62460_e80924 * locals.var_gfac_dn5)) * locals.var_gfac) + (assign62460_e80926 * locals.var_gfac_dn5)), (((((((locals.var_fac_exc * locals.var_i_ds_dn6) * locals.var_vdse_dc) + (assign62460_e80922 * locals.var_vdse_dc_dn6)) * locals.var_gfac) + (assign62460_e80924 * locals.var_gfac_dn6)) * locals.var_gfac) + (assign62460_e80926 * locals.var_gfac_dn6)), (((((((locals.var_fac_exc * locals.var_i_ds_dn7) * locals.var_vdse_dc) + (assign62460_e80922 * locals.var_vdse_dc_dn7)) * locals.var_gfac) + (assign62460_e80924 * locals.var_gfac_dn7)) * locals.var_gfac) + (assign62460_e80926 * locals.var_gfac_dn7)), (((((((locals.var_fac_exc * locals.var_i_ds_dn8) * locals.var_vdse_dc) + (assign62460_e80922 * locals.var_vdse_dc_dn8)) * locals.var_gfac) + (assign62460_e80924 * locals.var_gfac_dn8)) * locals.var_gfac) + (assign62460_e80926 * locals.var_gfac_dn8)), );
        }

        if (((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) && (locals.var_guard1763 != 0.0)) {
            let assign62470_e80939: f64 = (locals.var_sidexc / locals.var_nt0);
            let assign62470_e80940: f64 = (locals.var_mid + assign62470_e80939);
            (locals.var_mid, locals.var_mid_dn5, locals.var_mid_dn6, locals.var_mid_dn7, locals.var_mid_dn8, ) = (assign62470_e80940, (locals.var_mid_dn5 + (locals.var_sidexc_dn5 / locals.var_nt0)), (locals.var_mid_dn6 + (locals.var_sidexc_dn6 / locals.var_nt0)), (locals.var_mid_dn7 + (locals.var_sidexc_dn7 / locals.var_nt0)), (locals.var_mid_dn8 + (locals.var_sidexc_dn8 / locals.var_nt0)), );
        }

        if ((locals.var_guard1760 != 0.0) && (locals.var_guard1762 != 0.0)) {
            let assign62480_e80948: f64 = (locals.var_nt * locals.var_mid);
            let assign62480_e80949: f64 = (assign62480_e80948).sqrt();
            (locals.var_sqid, locals.var_sqid_dn5, locals.var_sqid_dn6, locals.var_sqid_dn7, locals.var_sqid_dn8, ) = (assign62480_e80949, ((locals.var_nt * locals.var_mid_dn5) / (2.0 * assign62480_e80949)), ((locals.var_nt * locals.var_mid_dn6) / (2.0 * assign62480_e80949)), ((locals.var_nt * locals.var_mid_dn7) / (2.0 * assign62480_e80949)), ((locals.var_nt * locals.var_mid_dn8) / (2.0 * assign62480_e80949)), );
        }

        let assign62490_e80966: f64 = if ((((p.p50 == 1.0) && (locals.var_nt > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1765 = assign62490_e80966;

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
            (locals.var_mig, locals.var_mig_dn5, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8, ) = (assign62500_e80996, (((locals.var_t1_dn5 / 12.0) - ((locals.var_t2_dn5 * assign62500_e80980) + (locals.var_t2 * (locals.var_t1_dn5 - (12.0 * locals.var_t2_dn5))))) - (1.6 * ((((locals.var_t2_dn5 * assign62500_e80991) + (locals.var_t2 * (locals.var_t1_dn5 - (12.0 * locals.var_t2_dn5)))) * locals.var_r) + (assign62500_e80992 * locals.var_r_dn5)))), (((locals.var_t1_dn6 / 12.0) - ((locals.var_t2_dn6 * assign62500_e80980) + (locals.var_t2 * (locals.var_t1_dn6 - (12.0 * locals.var_t2_dn6))))) - (1.6 * ((((locals.var_t2_dn6 * assign62500_e80991) + (locals.var_t2 * (locals.var_t1_dn6 - (12.0 * locals.var_t2_dn6)))) * locals.var_r) + (assign62500_e80992 * locals.var_r_dn6)))), (((locals.var_t1_dn7 / 12.0) - ((locals.var_t2_dn7 * assign62500_e80980) + (locals.var_t2 * (locals.var_t1_dn7 - (12.0 * locals.var_t2_dn7))))) - (1.6 * ((((locals.var_t2_dn7 * assign62500_e80991) + (locals.var_t2 * (locals.var_t1_dn7 - (12.0 * locals.var_t2_dn7)))) * locals.var_r) + (assign62500_e80992 * locals.var_r_dn7)))), (((locals.var_t1_dn8 / 12.0) - ((locals.var_t2_dn8 * assign62500_e80980) + (locals.var_t2 * (locals.var_t1_dn8 - (12.0 * locals.var_t2_dn8))))) - (1.6 * ((((locals.var_t2_dn8 * assign62500_e80991) + (locals.var_t2 * (locals.var_t1_dn8 - (12.0 * locals.var_t2_dn8)))) * locals.var_r) + (assign62500_e80992 * locals.var_r_dn8)))), );
        }

        if ((locals.var_guard1760 != 0.0) && (locals.var_guard1765 != 0.0)) {
            let (assign62510_e81007, assign62510_e81007_d_n5, assign62510_e81007_d_n6, assign62510_e81007_d_n7, assign62510_e81007_d_n8,) = {
    if (locals.var_mig > 1e-40) {
        (locals.var_mig, locals.var_mig_dn5, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8,)
    } else {
        (1e-40, 0.0, 0.0, 0.0, 0.0,)
    }
};
            (locals.var_mig, locals.var_mig_dn5, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8, ) = (assign62510_e81007, assign62510_e81007_d_n5, assign62510_e81007_d_n6, assign62510_e81007_d_n7, assign62510_e81007_d_n8, );
        }

        if ((locals.var_guard1760 != 0.0) && (locals.var_guard1765 != 0.0)) {
            let assign62520_e81015: f64 = (locals.var_lcinv2 / locals.var_g_ideal);
            let assign62520_e81017: f64 = (assign62520_e81015 * locals.var_mig);
            (locals.var_mig, locals.var_mig_dn5, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8, ) = (assign62520_e81017, (((((locals.var_lcinv2_dn5 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn5)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign62520_e81015 * locals.var_mig_dn5)), (((((locals.var_lcinv2_dn6 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn6)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign62520_e81015 * locals.var_mig_dn6)), (((((locals.var_lcinv2_dn7 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn7)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign62520_e81015 * locals.var_mig_dn7)), (((((locals.var_lcinv2_dn8 * locals.var_g_ideal) - (locals.var_lcinv2 * locals.var_g_ideal_dn8)) / (locals.var_g_ideal * locals.var_g_ideal)) * locals.var_mig) + (assign62520_e81015 * locals.var_mig_dn8)), );
        }

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
            (locals.var_migid0, locals.var_migid0_dn5, locals.var_migid0_dn6, locals.var_migid0_dn7, locals.var_migid0_dn8, ) = (assign62530_e81045, ((((locals.var_lcinv2_dn5 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn5)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * locals.var_t2_dn5)) - ((((locals.var_t1_dn5 + (19.2 * locals.var_t2_dn5)) - (12.0 * ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)))) * locals.var_r) + (assign62530_e81041 * locals.var_r_dn5))))), ((((locals.var_lcinv2_dn6 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn6)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * locals.var_t2_dn6)) - ((((locals.var_t1_dn6 + (19.2 * locals.var_t2_dn6)) - (12.0 * ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)))) * locals.var_r) + (assign62530_e81041 * locals.var_r_dn6))))), ((((locals.var_lcinv2_dn7 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn7)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * locals.var_t2_dn7)) - ((((locals.var_t1_dn7 + (19.2 * locals.var_t2_dn7)) - (12.0 * ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)))) * locals.var_r) + (assign62530_e81041 * locals.var_r_dn7))))), ((((locals.var_lcinv2_dn8 * locals.var_sqt2) + (locals.var_lcinv2 * locals.var_sqt2_dn8)) * assign62530_e81044) + (assign62530_e81025 * ((-(12.0 * locals.var_t2_dn8)) - ((((locals.var_t1_dn8 + (19.2 * locals.var_t2_dn8)) - (12.0 * ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)))) * locals.var_r) + (assign62530_e81041 * locals.var_r_dn8))))), );
        }

        if ((locals.var_guard1760 != 0.0) && (locals.var_guard1765 != 0.0)) {
            let assign62540_e81053: f64 = (locals.var_gvsat_ac * locals.var_gvsat_ac);
            let assign62540_e81055: f64 = (assign62540_e81053 * locals.var_cox_qm);
            let assign62540_e81057: f64 = (assign62540_e81055 * locals.var_eta_p_ac);
            let assign62540_e81060: f64 = (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac);
            let assign62540_e81061: f64 = (assign62540_e81057 / assign62540_e81060);
            (locals.var_cgeff, locals.var_cgeff_dn5, locals.var_cgeff_dn6, locals.var_cgeff_dn7, locals.var_cgeff_dn8, ) = (assign62540_e81061, (((((((((locals.var_gvsat_ac_dn5 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn5)) * locals.var_cox_qm) + (assign62540_e81053 * locals.var_cox_qm_dn5)) * locals.var_eta_p_ac) + (assign62540_e81055 * locals.var_eta_p_ac_dn5)) * assign62540_e81060) - (assign62540_e81057 * ((locals.var_gmob_dl_ac_dn5 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn5)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((locals.var_gvsat_ac_dn6 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn6)) * locals.var_cox_qm) + (assign62540_e81053 * locals.var_cox_qm_dn6)) * locals.var_eta_p_ac) + (assign62540_e81055 * locals.var_eta_p_ac_dn6)) * assign62540_e81060) - (assign62540_e81057 * ((locals.var_gmob_dl_ac_dn6 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn6)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((locals.var_gvsat_ac_dn7 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn7)) * locals.var_cox_qm) + (assign62540_e81053 * locals.var_cox_qm_dn7)) * locals.var_eta_p_ac) + (assign62540_e81055 * locals.var_eta_p_ac_dn7)) * assign62540_e81060) - (assign62540_e81057 * ((locals.var_gmob_dl_ac_dn7 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn7)))) / (assign62540_e81060 * assign62540_e81060)), (((((((((locals.var_gvsat_ac_dn8 * locals.var_gvsat_ac) + (locals.var_gvsat_ac * locals.var_gvsat_ac_dn8)) * locals.var_cox_qm) + (assign62540_e81053 * locals.var_cox_qm_dn8)) * locals.var_eta_p_ac) + (assign62540_e81055 * locals.var_eta_p_ac_dn8)) * assign62540_e81060) - (assign62540_e81057 * ((locals.var_gmob_dl_ac_dn8 * locals.var_gmob_dl_ac) + (locals.var_gmob_dl_ac * locals.var_gmob_dl_ac_dn8)))) / (assign62540_e81060 * assign62540_e81060)), );
        }

        let assign62550_e81066: f64 = if locals.var_fntexc_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1766 = assign62550_e81066;

        if (((locals.var_guard1760 != 0.0) && (locals.var_guard1765 != 0.0)) && (locals.var_guard1766 != 0.0)) {
            let assign62560_e81077: f64 = (12.0 * locals.var_t2);
            let assign62560_e81078: f64 = (1.0 + assign62560_e81077);
            let assign62560_e81079: f64 = (locals.var_sidexc * assign62560_e81078);
            let assign62560_e81082: f64 = (12.0 * locals.var_g_ideal);
            let assign62560_e81084: f64 = (assign62560_e81082 * locals.var_g_ideal);
            let assign62560_e81086: f64 = (assign62560_e81084 * locals.var_nt0);
            let assign62560_e81087: f64 = (assign62560_e81079 / assign62560_e81086);
            let assign62560_e81088: f64 = (locals.var_mig + assign62560_e81087);
            (locals.var_mig, locals.var_mig_dn5, locals.var_mig_dn6, locals.var_mig_dn7, locals.var_mig_dn8, ) = (assign62560_e81088, (locals.var_mig_dn5 + (((((locals.var_sidexc_dn5 * assign62560_e81078) + (locals.var_sidexc * (12.0 * locals.var_t2_dn5))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * locals.var_g_ideal_dn5) * locals.var_g_ideal) + (assign62560_e81082 * locals.var_g_ideal_dn5)) * locals.var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (locals.var_mig_dn6 + (((((locals.var_sidexc_dn6 * assign62560_e81078) + (locals.var_sidexc * (12.0 * locals.var_t2_dn6))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * locals.var_g_ideal_dn6) * locals.var_g_ideal) + (assign62560_e81082 * locals.var_g_ideal_dn6)) * locals.var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (locals.var_mig_dn7 + (((((locals.var_sidexc_dn7 * assign62560_e81078) + (locals.var_sidexc * (12.0 * locals.var_t2_dn7))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * locals.var_g_ideal_dn7) * locals.var_g_ideal) + (assign62560_e81082 * locals.var_g_ideal_dn7)) * locals.var_nt0))) / (assign62560_e81086 * assign62560_e81086))), (locals.var_mig_dn8 + (((((locals.var_sidexc_dn8 * assign62560_e81078) + (locals.var_sidexc * (12.0 * locals.var_t2_dn8))) * assign62560_e81086) - (assign62560_e81079 * ((((12.0 * locals.var_g_ideal_dn8) * locals.var_g_ideal) + (assign62560_e81082 * locals.var_g_ideal_dn8)) * locals.var_nt0))) / (assign62560_e81086 * assign62560_e81086))), );
        }

        if (((locals.var_guard1760 != 0.0) && (locals.var_guard1765 != 0.0)) && (locals.var_guard1766 != 0.0)) {
            let assign62570_e81099: f64 = (locals.var_sidexc * locals.var_sqt2);
            let assign62570_e81102: f64 = (1.0 + locals.var_r);
            let assign62570_e81103: f64 = (assign62570_e81099 * assign62570_e81102);
            let assign62570_e81106: f64 = (locals.var_g_ideal * locals.var_nt0);
            let assign62570_e81107: f64 = (assign62570_e81103 / assign62570_e81106);
            let assign62570_e81108: f64 = (locals.var_migid0 - assign62570_e81107);
            (locals.var_migid0, locals.var_migid0_dn5, locals.var_migid0_dn6, locals.var_migid0_dn7, locals.var_migid0_dn8, ) = (assign62570_e81108, (locals.var_migid0_dn5 - (((((((locals.var_sidexc_dn5 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn5)) * assign62570_e81102) + (assign62570_e81099 * locals.var_r_dn5)) * assign62570_e81106) - (assign62570_e81103 * (locals.var_g_ideal_dn5 * locals.var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (locals.var_migid0_dn6 - (((((((locals.var_sidexc_dn6 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn6)) * assign62570_e81102) + (assign62570_e81099 * locals.var_r_dn6)) * assign62570_e81106) - (assign62570_e81103 * (locals.var_g_ideal_dn6 * locals.var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (locals.var_migid0_dn7 - (((((((locals.var_sidexc_dn7 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn7)) * assign62570_e81102) + (assign62570_e81099 * locals.var_r_dn7)) * assign62570_e81106) - (assign62570_e81103 * (locals.var_g_ideal_dn7 * locals.var_nt0))) / (assign62570_e81106 * assign62570_e81106))), (locals.var_migid0_dn8 - (((((((locals.var_sidexc_dn8 * locals.var_sqt2) + (locals.var_sidexc * locals.var_sqt2_dn8)) * assign62570_e81102) + (assign62570_e81099 * locals.var_r_dn8)) * assign62570_e81106) - (assign62570_e81103 * (locals.var_g_ideal_dn8 * locals.var_nt0))) / (assign62570_e81106 * assign62570_e81106))), );
        }

        if ((locals.var_guard1760 != 0.0) && (locals.var_guard1765 != 0.0)) {
            let assign62580_e81116: f64 = (locals.var_nt / locals.var_mig);
            let assign62580_e81117: f64 = (assign62580_e81116).sqrt();
            (locals.var_sqig, locals.var_sqig_dn5, locals.var_sqig_dn6, locals.var_sqig_dn7, locals.var_sqig_dn8, ) = (assign62580_e81117, ((-((locals.var_nt * locals.var_mig_dn5) / (locals.var_mig * locals.var_mig))) / (2.0 * assign62580_e81117)), ((-((locals.var_nt * locals.var_mig_dn6) / (locals.var_mig * locals.var_mig))) / (2.0 * assign62580_e81117)), ((-((locals.var_nt * locals.var_mig_dn7) / (locals.var_mig * locals.var_mig))) / (2.0 * assign62580_e81117)), ((-((locals.var_nt * locals.var_mig_dn8) / (locals.var_mig * locals.var_mig))) / (2.0 * assign62580_e81117)), );
        }

        let assign62590_e81122: f64 = if locals.var_sqid <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1767 = assign62590_e81122;

        if (((locals.var_guard1760 != 0.0) && (locals.var_guard1765 != 0.0)) && (locals.var_guard1767 != 0.0)) {
            (locals.var_c_igid, locals.var_c_igid_dn5, locals.var_c_igid_dn6, locals.var_c_igid_dn7, locals.var_c_igid_dn8, ) = (0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (((locals.var_guard1760 != 0.0) && (locals.var_guard1765 != 0.0)) && (locals.var_guard1767 == 0.0)) {
            let assign62610_e81139: f64 = (locals.var_migid0 * locals.var_sqig);
            let assign62610_e81141: f64 = (assign62610_e81139 / locals.var_sqid);
            (locals.var_c_igid, locals.var_c_igid_dn5, locals.var_c_igid_dn6, locals.var_c_igid_dn7, locals.var_c_igid_dn8, ) = (assign62610_e81141, (((((locals.var_migid0_dn5 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn5)) * locals.var_sqid) - (assign62610_e81139 * locals.var_sqid_dn5)) / (locals.var_sqid * locals.var_sqid)), (((((locals.var_migid0_dn6 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn6)) * locals.var_sqid) - (assign62610_e81139 * locals.var_sqid_dn6)) / (locals.var_sqid * locals.var_sqid)), (((((locals.var_migid0_dn7 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn7)) * locals.var_sqid) - (assign62610_e81139 * locals.var_sqid_dn7)) / (locals.var_sqid * locals.var_sqid)), (((((locals.var_migid0_dn8 * locals.var_sqig) + (locals.var_migid0 * locals.var_sqig_dn8)) * locals.var_sqid) - (assign62610_e81139 * locals.var_sqid_dn8)) / (locals.var_sqid * locals.var_sqid)), );
        }

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
            (locals.var_c_igid, locals.var_c_igid_dn5, locals.var_c_igid_dn6, locals.var_c_igid_dn7, locals.var_c_igid_dn8, ) = (assign62620_e81157, assign62620_e81157_d_n5, assign62620_e81157_d_n6, assign62620_e81157_d_n7, assign62620_e81157_d_n8, );
        }

        if ((locals.var_guard1760 != 0.0) && (locals.var_guard1765 != 0.0)) {
            let assign62630_e81165: f64 = (locals.var_c_igid * locals.var_sqid);
            let assign62630_e81167: f64 = (assign62630_e81165 / locals.var_sqig);
            (locals.var_migid, locals.var_migid_dn5, locals.var_migid_dn6, locals.var_migid_dn7, locals.var_migid_dn8, ) = (assign62630_e81167, (((((locals.var_c_igid_dn5 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn5)) * locals.var_sqig) - (assign62630_e81165 * locals.var_sqig_dn5)) / (locals.var_sqig * locals.var_sqig)), (((((locals.var_c_igid_dn6 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn6)) * locals.var_sqig) - (assign62630_e81165 * locals.var_sqig_dn6)) / (locals.var_sqig * locals.var_sqig)), (((((locals.var_c_igid_dn7 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn7)) * locals.var_sqig) - (assign62630_e81165 * locals.var_sqig_dn7)) / (locals.var_sqig * locals.var_sqig)), (((((locals.var_c_igid_dn8 * locals.var_sqid) + (locals.var_c_igid * locals.var_sqid_dn8)) * locals.var_sqig) - (assign62630_e81165 * locals.var_sqig_dn8)) / (locals.var_sqig * locals.var_sqig)), );
        }

        let assign62800_e81277: f64 = if (((p.p46 != 0.0) && (locals.var_betnedge_i > 0.0)) && (locals.var_xgedge > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1769 = assign62800_e81277;

        if (locals.var_guard1769 != 0.0) {
            let assign62810_e81281: f64 = (4.0 * locals.var_dsqredge);
            let assign62810_e81283: f64 = (assign62810_e81281 / locals.var_gfedge2);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign62810_e81283, ((4.0 * locals.var_dsqredge_dn5) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn6) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn7) / locals.var_gfedge2), ((4.0 * locals.var_dsqredge_dn8) / locals.var_gfedge2), );
        }

    }

    pub(super) fn stamp_transient_block_28(
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard1769 != 0.0) {
            let assign62830_e81303: f64 = (locals.var_cox_over_q * locals.var_phit);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign62830_e81303, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard1769 != 0.0) {
            let assign62960_e81443: f64 = (locals.var_alpha_dc * locals.var_h_dc);
            (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, ) = (assign62960_e81443, ((locals.var_alpha_dc_dn5 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn5)), ((locals.var_alpha_dc_dn6 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn6)), ((locals.var_alpha_dc_dn7 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn7)), ((locals.var_alpha_dc_dn8 * locals.var_h_dc) + (locals.var_alpha_dc * locals.var_h_dc_dn8)), );
        }

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

        if (locals.var_guard1 != 0.0) {
            let assign10_e1448: f64 = 1.0;
            locals.var_chnl_type = assign10_e1448;
            locals.var_chnl_type_rv = 0.0;
        }

        if (locals.var_guard1 == 0.0) {
            let assign20_e1454: f64 = (-1.0);
            locals.var_chnl_type = assign20_e1454;
            locals.var_chnl_type_rv = 0.0;
        }

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

        if (!(locals.var_phibfac > 0.001)) {
            locals.var_phibfac = 0.001;
            locals.var_phibfac_rv = 0.0;
        }

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

        if (locals.var_guard29 != 0.0) {
            let (assign3510_e3425,) = {
    if (p.p9 > 1.0) {
        (p.p9,)
    } else {
        (1.0,)
    }
};
            locals.var_nf_i = assign3510_e3425;
            locals.var_nf_i_rv = 0.0;
        }

        if (locals.var_guard29 != 0.0) {
            let assign3520_e3431: f64 = (locals.var_nf_i + 0.5);
            let assign3520_e3432: f64 = (assign3520_e3431).floor();
            locals.var_nf_i = assign3520_e3432;
            locals.var_nf_i_rv = 0.0;
        }

        if (locals.var_guard29 != 0.0) {
            let assign3530_e3438: f64 = (1.0 / locals.var_nf_i);
            locals.var_invnf = assign3530_e3438;
            locals.var_invnf_rv = 0.0;
        }

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

    }

    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        locals.var_gc3_p = p.p120;
        locals.var_gc3_p_rv = 0.0;

        locals.var_gc2ov_p = p.p119;
        locals.var_gc2ov_p_rv = 0.0;

        let assign4480_e3738: f64 = if param_given[121] { 1.0 } else { 0.0 };
        let assign4480_e3740: f64 = if assign4480_e3738 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard30 = assign4480_e3740;
        locals.var_guard30_rv = 0.0;

        if (locals.var_guard30 != 0.0) {
            locals.var_gc2ov_p = p.p121;
            locals.var_gc2ov_p_rv = 0.0;
        }

        locals.var_gc3ov_p = p.p120;
        locals.var_gc3ov_p_rv = 0.0;

        let assign4510_e3747: f64 = if param_given[122] { 1.0 } else { 0.0 };
        let assign4510_e3749: f64 = if assign4510_e3747 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard31 = assign4510_e3749;
        locals.var_guard31_rv = 0.0;

        if (locals.var_guard31 != 0.0) {
            locals.var_gc3ov_p = p.p122;
            locals.var_gc3ov_p_rv = 0.0;
        }

        locals.var_gc2ovd_p = locals.var_gc2ov_p;
        locals.var_gc2ovd_p_rv = 0.0;

        let assign4540_e3756: f64 = if param_given[123] { 1.0 } else { 0.0 };
        let assign4540_e3758: f64 = if assign4540_e3756 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign4540_e3758;
        locals.var_guard32_rv = 0.0;

        if (locals.var_guard32 != 0.0) {
            locals.var_gc2ovd_p = p.p123;
            locals.var_gc2ovd_p_rv = 0.0;
        }

        locals.var_gc3ovd_p = locals.var_gc3ov_p;
        locals.var_gc3ovd_p_rv = 0.0;

        let assign4570_e3765: f64 = if param_given[124] { 1.0 } else { 0.0 };
        let assign4570_e3767: f64 = if assign4570_e3765 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard33 = assign4570_e3767;
        locals.var_guard33_rv = 0.0;

        if (locals.var_guard33 != 0.0) {
            locals.var_gc3ovd_p = p.p124;
            locals.var_gc3ovd_p_rv = 0.0;
        }

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

        if (locals.var_guard34 != 0.0) {
            locals.var_thesatac_p = p.p137;
            locals.var_thesatac_p_rv = 0.0;
        }

        locals.var_axac_p = p.p103;
        locals.var_axac_p_rv = 0.0;

        let assign4750_e3795: f64 = if param_given[138] { 1.0 } else { 0.0 };
        let assign4750_e3797: f64 = if assign4750_e3795 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard35 = assign4750_e3797;
        locals.var_guard35_rv = 0.0;

        if (locals.var_guard35 != 0.0) {
            locals.var_axac_p = p.p138;
            locals.var_axac_p_rv = 0.0;
        }

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

        if (locals.var_guard36 != 0.0) {
            let assign5250_e3857: f64 = (locals.var_ile).powf(p.p198);
            let assign5250_e3858: f64 = (p.p197 * assign5250_e3857);
            let assign5250_e3859: f64 = (p.p196 + assign5250_e3858);
            let assign5250_e3862: f64 = (p.p199 * locals.var_iwe);
            let assign5250_e3863: f64 = (assign5250_e3859 + assign5250_e3862);
            let assign5250_e3866: f64 = (p.p200 * locals.var_iae);
            let assign5250_e3867: f64 = (assign5250_e3863 + assign5250_e3866);
            locals.var_vfb_p = assign5250_e3867;
            locals.var_vfb_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign5260_e3874: f64 = (p.p202 * locals.var_ile);
            let assign5260_e3875: f64 = (p.p201 + assign5260_e3874);
            let assign5260_e3878: f64 = (p.p203 * locals.var_iwe);
            let assign5260_e3879: f64 = (assign5260_e3875 + assign5260_e3878);
            let assign5260_e3882: f64 = (p.p204 * locals.var_iae);
            let assign5260_e3883: f64 = (assign5260_e3879 + assign5260_e3882);
            locals.var_stvfb_p = assign5260_e3883;
            locals.var_stvfb_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_st2vfb_p = p.p205;
            locals.var_st2vfb_p_rv = 0.0;
            locals.var_tox_p = p.p206;
            locals.var_tox_p_rv = 0.0;
            locals.var_epsrox_p = p.p207;
            locals.var_epsrox_p_rv = 0.0;
        }

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
            locals.var_nsub0e = assign5300_e3928;
            locals.var_nsub0e_rv = 0.0;
        }

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
            locals.var_npcke = assign5310_e3961;
            locals.var_npcke_rv = 0.0;
        }

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
            locals.var_lpcke = assign5320_e3994;
            locals.var_lpcke_rv = 0.0;
        }

        let assign5330_e4000: f64 = (2.0 * locals.var_lpcke);
        let assign5330_e4001: f64 = if locals.var_le > assign5330_e4000 { 1.0 } else { 0.0 };
        locals.var_guard37 = assign5330_e4001;
        locals.var_guard37_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard37 != 0.0)) {
            locals.var_aa = 75000000000.0;
            locals.var_aa_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard37 != 0.0)) {
            let assign5350_e4014: f64 = (0.5 * locals.var_npcke);
            let assign5350_e4015: f64 = (locals.var_nsub0e + assign5350_e4014);
            let assign5350_e4016: f64 = (assign5350_e4015).sqrt();
            let assign5350_e4018: f64 = (locals.var_nsub0e).sqrt();
            let assign5350_e4019: f64 = (assign5350_e4016 - assign5350_e4018);
            locals.var_bb = assign5350_e4019;
            locals.var_bb_rv = 0.0;
        }

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
            locals.var_nsub = assign5360_e4044;
            locals.var_nsub_rv = 0.0;
        }

        if ((locals.var_guard36 != 0.0) && (locals.var_guard37 != 0.0)) {
            let assign5370_e4052: f64 = (locals.var_nsub * locals.var_nsub);
            locals.var_nsub = assign5370_e4052;
            locals.var_nsub_rv = 0.0;
        }

        let assign5380_e4057: f64 = if locals.var_le >= locals.var_lpcke { 1.0 } else { 0.0 };
        locals.var_guard38 = assign5380_e4057;
        locals.var_guard38_rv = 0.0;

        if (((locals.var_guard36 != 0.0) && (locals.var_guard37 == 0.0)) && (locals.var_guard38 != 0.0)) {
            let assign5390_e4067: f64 = (locals.var_npcke * locals.var_lpcke);
            let assign5390_e4069: f64 = (assign5390_e4067 / locals.var_le);
            let assign5390_e4070: f64 = (locals.var_nsub0e + assign5390_e4069);
            locals.var_nsub = assign5390_e4070;
            locals.var_nsub_rv = 0.0;
        }

        if (((locals.var_guard36 != 0.0) && (locals.var_guard37 == 0.0)) && (locals.var_guard38 == 0.0)) {
            let assign5400_e4085: f64 = (locals.var_le / locals.var_lpcke);
            let assign5400_e4086: f64 = (2.0 - assign5400_e4085);
            let assign5400_e4087: f64 = (locals.var_npcke * assign5400_e4086);
            let assign5400_e4088: f64 = (locals.var_nsub0e + assign5400_e4087);
            locals.var_nsub = assign5400_e4088;
            locals.var_nsub_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign5410_e4096: f64 = (p.p216 * locals.var_ile);
            let assign5410_e4097: f64 = (1.0 - assign5410_e4096);
            let assign5410_e4100: f64 = (p.p217 * locals.var_ile2);
            let assign5410_e4101: f64 = (assign5410_e4097 - assign5410_e4100);
            let assign5410_e4102: f64 = (locals.var_nsub * assign5410_e4101);
            locals.var_neff_p = assign5410_e4102;
            locals.var_neff_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign5420_e4110: f64 = (locals.var_ile).powf(p.p220);
            let assign5420_e4111: f64 = (p.p219 * assign5420_e4110);
            let assign5420_e4112: f64 = (p.p218 + assign5420_e4111);
            let assign5420_e4115: f64 = (p.p221 * locals.var_iwe);
            let assign5420_e4116: f64 = (assign5420_e4112 + assign5420_e4115);
            let assign5420_e4119: f64 = (p.p222 * locals.var_iae);
            let assign5420_e4120: f64 = (assign5420_e4116 + assign5420_e4119);
            locals.var_gfacnud_p = assign5420_e4120;
            locals.var_gfacnud_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_vsbnud_p = p.p223;
            locals.var_vsbnud_p_rv = 0.0;
            locals.var_dvsbnud_p = p.p224;
            locals.var_dvsbnud_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign5450_e4136: f64 = (locals.var_ile).powf(p.p227);
            let assign5450_e4137: f64 = (p.p226 * assign5450_e4136);
            let assign5450_e4138: f64 = (p.p225 + assign5450_e4137);
            let assign5450_e4141: f64 = (p.p228 * locals.var_iwe);
            let assign5450_e4142: f64 = (assign5450_e4138 + assign5450_e4141);
            let assign5450_e4145: f64 = (p.p229 * locals.var_iae);
            let assign5450_e4146: f64 = (assign5450_e4142 + assign5450_e4145);
            locals.var_dphib_p = assign5450_e4146;
            locals.var_dphib_p_rv = 0.0;
        }

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
            locals.var_np_p = assign5460_e4165;
            locals.var_np_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_toxov_p = p.p232;
            locals.var_toxov_p_rv = 0.0;
            locals.var_toxovd_p = p.p233;
            locals.var_toxovd_p_rv = 0.0;
            locals.var_nov_p = p.p236;
            locals.var_nov_p_rv = 0.0;
            locals.var_novd_p = p.p237;
            locals.var_novd_p_rv = 0.0;
        }

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
            locals.var_ct_p = assign5510_e4203;
            locals.var_ct_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_ctg_p = p.p244;
            locals.var_ctg_p_rv = 0.0;
            locals.var_ctb_p = p.p243;
            locals.var_ctb_p_rv = 0.0;
            locals.var_stct_p = p.p245;
            locals.var_stct_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign5550_e4222: f64 = (locals.var_ile).powf(p.p247);
            let assign5550_e4223: f64 = (p.p246 * assign5550_e4222);
            let assign5550_e4227: f64 = (p.p248 * locals.var_iwe);
            let assign5550_e4228: f64 = (1.0 + assign5550_e4227);
            let assign5550_e4229: f64 = (assign5550_e4223 * assign5550_e4228);
            locals.var_cf_p = assign5550_e4229;
            locals.var_cf_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_cfd_p = p.p250;
            locals.var_cfd_p_rv = 0.0;
            locals.var_cfb_p = p.p249;
            locals.var_cfb_p_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard36 != 0.0) {
            let assign5580_e4244: f64 = (locals.var_ile).powf(p.p252);
            let assign5580_e4245: f64 = (p.p251 * assign5580_e4244);
            let assign5580_e4249: f64 = (p.p253 * locals.var_iwe);
            let assign5580_e4250: f64 = (1.0 + assign5580_e4249);
            let assign5580_e4251: f64 = (assign5580_e4245 * assign5580_e4250);
            locals.var_psce_p = assign5580_e4251;
            locals.var_psce_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_psced_p = p.p255;
            locals.var_psced_p_rv = 0.0;
            locals.var_psceb_p = p.p254;
            locals.var_psceb_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign5610_e4267: f64 = (p.p258 * locals.var_iwe);
            let assign5610_e4268: f64 = (1.0 + assign5610_e4267);
            let assign5610_e4269: f64 = (p.p257 * assign5610_e4268);
            locals.var_fbet1e = assign5610_e4269;
            locals.var_fbet1e_rv = 0.0;
        }

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
            locals.var_lp1e = assign5620_e4288;
            locals.var_lp1e_rv = 0.0;
        }

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
            locals.var_gpe = assign5630_e4320;
            locals.var_gpe_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let (assign5640_e4329,) = {
    if (locals.var_gpe > 1e-15) {
        (locals.var_gpe,)
    } else {
        (1e-15,)
    }
};
            locals.var_gpe = assign5640_e4329;
            locals.var_gpe_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign5650_e4336: f64 = (p.p263 * locals.var_iwe);
            let assign5650_e4337: f64 = (1.0 + assign5650_e4336);
            let assign5650_e4340: f64 = (p.p264 * locals.var_iwe);
            let assign5650_e4344: f64 = (locals.var_we / p.p265);
            let assign5650_e4345: f64 = (1.0 + assign5650_e4344);
            let assign5650_e4346: f64 = (assign5650_e4345).ln();
            let assign5650_e4347: f64 = (assign5650_e4340 * assign5650_e4346);
            let assign5650_e4348: f64 = (assign5650_e4337 + assign5650_e4347);
            locals.var_gwe = assign5650_e4348;
            locals.var_gwe_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign5660_e4354: f64 = (p.p256 * locals.var_we);
            let assign5660_e4357: f64 = (locals.var_gpe * locals.var_le);
            let assign5660_e4358: f64 = (assign5660_e4354 / assign5660_e4357);
            let assign5660_e4360: f64 = (assign5660_e4358 * locals.var_gwe);
            locals.var_betn_p = assign5660_e4360;
            locals.var_betn_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign5670_e4367: f64 = (p.p267 * locals.var_ile);
            let assign5670_e4368: f64 = (p.p266 + assign5670_e4367);
            let assign5670_e4371: f64 = (p.p268 * locals.var_iwe);
            let assign5670_e4372: f64 = (assign5670_e4368 + assign5670_e4371);
            let assign5670_e4375: f64 = (p.p269 * locals.var_iae);
            let assign5670_e4376: f64 = (assign5670_e4372 + assign5670_e4375);
            locals.var_stbet_p = assign5670_e4376;
            locals.var_stbet_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign5680_e4384: f64 = (p.p271 * locals.var_iwe);
            let assign5680_e4385: f64 = (1.0 + assign5680_e4384);
            let assign5680_e4386: f64 = (p.p270 * assign5680_e4385);
            locals.var_mue_p = assign5680_e4386;
            locals.var_mue_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_stmue_p = p.p272;
            locals.var_stmue_p_rv = 0.0;
            locals.var_themu_p = p.p273;
            locals.var_themu_p_rv = 0.0;
            locals.var_stthemu_p = p.p274;
            locals.var_stthemu_p_rv = 0.0;
        }

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
            locals.var_cs_p = assign5720_e4420;
            locals.var_cs_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_stcs_p = p.p280;
            locals.var_stcs_p_rv = 0.0;
            locals.var_thecs_p = p.p281;
            locals.var_thecs_p_rv = 0.0;
            locals.var_stthecs_p = p.p282;
            locals.var_stthecs_p_rv = 0.0;
        }

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
            locals.var_xcor_p = assign5760_e4454;
            locals.var_xcor_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_stxcor_p = p.p287;
            locals.var_stxcor_p_rv = 0.0;
            locals.var_feta_p = p.p288;
            locals.var_feta_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign5790_e4468: f64 = (p.p289 * locals.var_iwe);
            let assign5790_e4472: f64 = (p.p290 * locals.var_iwe);
            let assign5790_e4473: f64 = (1.0 + assign5790_e4472);
            let assign5790_e4474: f64 = (assign5790_e4468 * assign5790_e4473);
            locals.var_rs_p = assign5790_e4474;
            locals.var_rs_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_strs_p = p.p291;
            locals.var_strs_p_rv = 0.0;
            locals.var_rsb_p = p.p292;
            locals.var_rsb_p_rv = 0.0;
            locals.var_rsg_p = p.p293;
            locals.var_rsg_p_rv = 0.0;
        }

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
            locals.var_thesat_p = assign5830_e4512;
            locals.var_thesat_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign5840_e4519: f64 = (p.p300 * locals.var_ile);
            let assign5840_e4520: f64 = (p.p299 + assign5840_e4519);
            let assign5840_e4523: f64 = (p.p301 * locals.var_iwe);
            let assign5840_e4524: f64 = (assign5840_e4520 + assign5840_e4523);
            let assign5840_e4527: f64 = (p.p302 * locals.var_iae);
            let assign5840_e4528: f64 = (assign5840_e4524 + assign5840_e4527);
            locals.var_stthesat_p = assign5840_e4528;
            locals.var_stthesat_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_thesatb_p = p.p303;
            locals.var_thesatb_p_rv = 0.0;
            locals.var_thesatg_p = p.p304;
            locals.var_thesatg_p_rv = 0.0;
            locals.var_thesatt_p = p.p305;
            locals.var_thesatt_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign5880_e4548: f64 = (p.p307 * locals.var_ile);
            let assign5880_e4549: f64 = (1.0 + assign5880_e4548);
            let assign5880_e4550: f64 = (p.p306 / assign5880_e4549);
            locals.var_ax_p = assign5880_e4550;
            locals.var_ax_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign5890_e4557: f64 = (locals.var_ile).powf(p.p309);
            let assign5890_e4558: f64 = (p.p308 * assign5890_e4557);
            let assign5890_e4562: f64 = (p.p310 * locals.var_iwe);
            let assign5890_e4563: f64 = (1.0 + assign5890_e4562);
            let assign5890_e4564: f64 = (assign5890_e4558 * assign5890_e4563);
            locals.var_alp_p = assign5890_e4564;
            locals.var_alp_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign5900_e4570: f64 = (locals.var_ile).powf(p.p312);
            locals.var_tmpx = assign5900_e4570;
            locals.var_tmpx_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign5910_e4576: f64 = (p.p311 * locals.var_tmpx);
            let assign5910_e4580: f64 = (p.p314 * locals.var_iwe);
            let assign5910_e4581: f64 = (1.0 + assign5910_e4580);
            let assign5910_e4582: f64 = (assign5910_e4576 * assign5910_e4581);
            let assign5910_e4586: f64 = (p.p313 * locals.var_ile);
            let assign5910_e4588: f64 = (assign5910_e4586 * locals.var_tmpx);
            let assign5910_e4589: f64 = (1.0 + assign5910_e4588);
            let assign5910_e4590: f64 = (assign5910_e4582 / assign5910_e4589);
            locals.var_alp1_p = assign5910_e4590;
            locals.var_alp1_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign5920_e4596: f64 = (locals.var_ile).powf(p.p316);
            locals.var_tmpx = assign5920_e4596;
            locals.var_tmpx_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign5930_e4602: f64 = (p.p315 * locals.var_tmpx);
            let assign5930_e4606: f64 = (p.p318 * locals.var_iwe);
            let assign5930_e4607: f64 = (1.0 + assign5930_e4606);
            let assign5930_e4608: f64 = (assign5930_e4602 * assign5930_e4607);
            let assign5930_e4612: f64 = (p.p317 * locals.var_ile);
            let assign5930_e4614: f64 = (assign5930_e4612 * locals.var_tmpx);
            let assign5930_e4615: f64 = (1.0 + assign5930_e4614);
            let assign5930_e4616: f64 = (assign5930_e4608 / assign5930_e4615);
            locals.var_alp2_p = assign5930_e4616;
            locals.var_alp2_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_vp_p = p.p319;
            locals.var_vp_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign5950_e4628: f64 = (p.p321 * locals.var_ile);
            let assign5950_e4629: f64 = (1.0 + assign5950_e4628);
            let assign5950_e4630: f64 = (p.p320 * assign5950_e4629);
            let assign5950_e4634: f64 = (p.p322 * locals.var_iwe);
            let assign5950_e4635: f64 = (1.0 + assign5950_e4634);
            let assign5950_e4636: f64 = (assign5950_e4630 * assign5950_e4635);
            locals.var_a1_p = assign5950_e4636;
            locals.var_a1_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_a2_p = p.p323;
            locals.var_a2_p_rv = 0.0;
            locals.var_sta2_p = p.p324;
            locals.var_sta2_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign5980_e4652: f64 = (p.p326 * locals.var_ile);
            let assign5980_e4653: f64 = (1.0 + assign5980_e4652);
            let assign5980_e4654: f64 = (p.p325 * assign5980_e4653);
            let assign5980_e4658: f64 = (p.p327 * locals.var_iwe);
            let assign5980_e4659: f64 = (1.0 + assign5980_e4658);
            let assign5980_e4660: f64 = (assign5980_e4654 * assign5980_e4659);
            locals.var_a3_p = assign5980_e4660;
            locals.var_a3_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign5990_e4668: f64 = (p.p329 * locals.var_ile);
            let assign5990_e4669: f64 = (1.0 + assign5990_e4668);
            let assign5990_e4670: f64 = (p.p328 * assign5990_e4669);
            let assign5990_e4674: f64 = (p.p330 * locals.var_iwe);
            let assign5990_e4675: f64 = (1.0 + assign5990_e4674);
            let assign5990_e4676: f64 = (assign5990_e4670 * assign5990_e4675);
            locals.var_a4_p = assign5990_e4676;
            locals.var_a4_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_imaxii_p = p.p331;
            locals.var_imaxii_p_rv = 0.0;
            locals.var_gco_p = p.p332;
            locals.var_gco_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6020_e4690: f64 = (p.p333 / locals.var_iae);
            locals.var_iginv_p = assign6020_e4690;
            locals.var_iginv_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6030_e4696: f64 = (p.p334 * p.p234);
            let assign6030_e4699: f64 = (1e-6 * locals.var_iwe);
            let assign6030_e4700: f64 = (assign6030_e4696 / assign6030_e4699);
            locals.var_igov_p = assign6030_e4700;
            locals.var_igov_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6040_e4706: f64 = (p.p335 * p.p235);
            let assign6040_e4709: f64 = (1e-6 * locals.var_iwe);
            let assign6040_e4710: f64 = (assign6040_e4706 / assign6040_e4709);
            locals.var_igovd_p = assign6040_e4710;
            locals.var_igovd_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_stig_p = p.p336;
            locals.var_stig_p_rv = 0.0;
            locals.var_gc2_p = p.p337;
            locals.var_gc2_p_rv = 0.0;
            locals.var_gc3_p = p.p338;
            locals.var_gc3_p_rv = 0.0;
            locals.var_gc2ov_p = p.p337;
            locals.var_gc2ov_p_rv = 0.0;
        }

        let assign6090_e4730: f64 = if param_given[339] { 1.0 } else { 0.0 };
        let assign6090_e4732: f64 = if assign6090_e4730 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard39 = assign6090_e4732;
        locals.var_guard39_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard39 != 0.0)) {
            locals.var_gc2ov_p = p.p339;
            locals.var_gc2ov_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_gc3ov_p = p.p338;
            locals.var_gc3ov_p_rv = 0.0;
        }

        let assign6120_e4744: f64 = if param_given[340] { 1.0 } else { 0.0 };
        let assign6120_e4746: f64 = if assign6120_e4744 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard40 = assign6120_e4746;
        locals.var_guard40_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard40 != 0.0)) {
            locals.var_gc3ov_p = p.p340;
            locals.var_gc3ov_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_gc2ovd_p = locals.var_gc2ov_p;
            locals.var_gc2ovd_p_rv = 0.0;
        }

        let assign6150_e4758: f64 = if param_given[341] { 1.0 } else { 0.0 };
        let assign6150_e4760: f64 = if assign6150_e4758 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard41 = assign6150_e4760;
        locals.var_guard41_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard41 != 0.0)) {
            locals.var_gc2ovd_p = p.p341;
            locals.var_gc2ovd_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_gc3ovd_p = locals.var_gc3ov_p;
            locals.var_gc3ovd_p_rv = 0.0;
        }

        let assign6180_e4772: f64 = if param_given[342] { 1.0 } else { 0.0 };
        let assign6180_e4774: f64 = if assign6180_e4772 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard42 = assign6180_e4774;
        locals.var_guard42_rv = 0.0;

        if ((locals.var_guard36 != 0.0) && (locals.var_guard42 != 0.0)) {
            locals.var_gc3ovd_p = p.p342;
            locals.var_gc3ovd_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_chib_p = p.p343;
            locals.var_chib_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6210_e4788: f64 = (p.p344 * p.p234);
            let assign6210_e4791: f64 = (1e-6 * locals.var_iwe);
            let assign6210_e4792: f64 = (assign6210_e4788 / assign6210_e4791);
            locals.var_agidl_p = assign6210_e4792;
            locals.var_agidl_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            let assign6220_e4798: f64 = (p.p345 * p.p235);
            let assign6220_e4801: f64 = (1e-6 * locals.var_iwe);
            let assign6220_e4802: f64 = (assign6220_e4798 / assign6220_e4801);
            locals.var_agidld_p = assign6220_e4802;
            locals.var_agidld_p_rv = 0.0;
        }

        if (locals.var_guard36 != 0.0) {
            locals.var_bgidl_p = p.p346;
            locals.var_bgidl_p_rv = 0.0;
            locals.var_bgidld_p = p.p347;
            locals.var_bgidld_p_rv = 0.0;
            locals.var_stbgidl_p = p.p348;
            locals.var_stbgidl_p_rv = 0.0;
            locals.var_stbgidld_p = p.p349;
            locals.var_stbgidld_p_rv = 0.0;
            locals.var_cgidl_p = p.p350;
            locals.var_cgidl_p_rv = 0.0;
        }

    }
}
