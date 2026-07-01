#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        locals: &mut StampLocals,
    ) {
        let (assign41430_e54365, assign41430_e54365_d_n4, assign41430_e54365_d_n6, assign41430_e54365_d_n7, assign41430_e54365_d_n8, assign41430_e54365_d_n9,) = {
    if (((locals.var_guard1192 != 0.0) && (locals.var_guard1193 == 0.0)) && (locals.var_guard1194 == 0.0)) {
        let assign41430_e54345: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41430_e54350: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41430_e54354: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41430_e54356: f64 = (assign41430_e54354 * 0.3333333333333333);
        let assign41430_e54357: f64 = (1.0 + assign41430_e54356);
        let assign41430_e54358: f64 = (assign41430_e54350 * assign41430_e54357);
        let assign41430_e54359: f64 = (0.5 * assign41430_e54358);
        let assign41430_e54360: f64 = (1.0 + assign41430_e54359);
        let assign41430_e54361: f64 = (assign41430_e54345 * assign41430_e54360);
        let assign41430_e54362: f64 = (1.0 + assign41430_e54361);
        let assign41430_e54363: f64 = (1e-200 / assign41430_e54362);
        (assign41430_e54363, (-((1e-200 * ((locals.var_xn_s_dn4 * assign41430_e54360) + (assign41430_e54345 * (0.5 * ((locals.var_xn_s_dn4 * assign41430_e54357) + (assign41430_e54350 * (locals.var_xn_s_dn4 * 0.3333333333333333))))))) / (assign41430_e54362 * assign41430_e54362))), (-((1e-200 * ((locals.var_xn_s_dn6 * assign41430_e54360) + (assign41430_e54345 * (0.5 * ((locals.var_xn_s_dn6 * assign41430_e54357) + (assign41430_e54350 * (locals.var_xn_s_dn6 * 0.3333333333333333))))))) / (assign41430_e54362 * assign41430_e54362))), (-((1e-200 * ((locals.var_xn_s_dn7 * assign41430_e54360) + (assign41430_e54345 * (0.5 * ((locals.var_xn_s_dn7 * assign41430_e54357) + (assign41430_e54350 * (locals.var_xn_s_dn7 * 0.3333333333333333))))))) / (assign41430_e54362 * assign41430_e54362))), (-((1e-200 * ((locals.var_xn_s_dn8 * assign41430_e54360) + (assign41430_e54345 * (0.5 * ((locals.var_xn_s_dn8 * assign41430_e54357) + (assign41430_e54350 * (locals.var_xn_s_dn8 * 0.3333333333333333))))))) / (assign41430_e54362 * assign41430_e54362))), (-((1e-200 * ((locals.var_xn_s_dn9 * assign41430_e54360) + (assign41430_e54345 * (0.5 * ((locals.var_xn_s_dn9 * assign41430_e54357) + (assign41430_e54350 * (locals.var_xn_s_dn9 * 0.3333333333333333))))))) / (assign41430_e54362 * assign41430_e54362))),)
    } else {
        (locals.var_delta_ns, locals.var_delta_ns_dn4, locals.var_delta_ns_dn6, locals.var_delta_ns_dn7, locals.var_delta_ns_dn8, locals.var_delta_ns_dn9,)
    }
};
        locals.var_delta_ns = assign41430_e54365;
        locals.var_delta_ns_dn4 = assign41430_e54365_d_n4;
        locals.var_delta_ns_dn6 = assign41430_e54365_d_n6;
        locals.var_delta_ns_dn7 = assign41430_e54365_d_n7;
        locals.var_delta_ns_dn8 = assign41430_e54365_d_n8;
        locals.var_delta_ns_dn9 = assign41430_e54365_d_n9;

        let (assign41440_e54378, assign41440_e54378_d_n4, assign41440_e54378_d_n6, assign41440_e54378_d_n7, assign41440_e54378_d_n8, assign41440_e54378_d_n9,) = {
    if ((locals.var_guard1192 != 0.0) && (locals.var_guard1193 == 0.0)) {
        let (assign41440_e54376,) = {
            if (locals.var_xn_s > 0.0) {
                (1.0,)
            } else {
                let assign41440_e54375: f64 = (-1.0);
                (assign41440_e54375,)
            }
        };
        (assign41440_e54376, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign41440_e54378;
        locals.var_temp__blk949_dn4 = assign41440_e54378_d_n4;
        locals.var_temp__blk949_dn6 = assign41440_e54378_d_n6;
        locals.var_temp__blk949_dn7 = assign41440_e54378_d_n7;
        locals.var_temp__blk949_dn8 = assign41440_e54378_d_n8;
        locals.var_temp__blk949_dn9 = assign41440_e54378_d_n9;

        let (assign41450_e54406, assign41450_e54406_d_n4, assign41450_e54406_d_n6, assign41450_e54406_d_n7, assign41450_e54406_d_n8, assign41450_e54406_d_n9,) = {
    if ((locals.var_guard1192 != 0.0) && (locals.var_guard1193 == 0.0)) {
        let assign41450_e54386: f64 = (locals.var_temp__blk949 * locals.var_gf);
        let assign41450_e54391: f64 = (1.0 - locals.var_xn_s);
        let assign41450_e54392: f64 = (locals.var_delta_ns * assign41450_e54391);
        let assign41450_e54393: f64 = (1.0 - assign41450_e54392);
        let assign41450_e54394: f64 = (assign41450_e54386 * assign41450_e54393);
        let assign41450_e54399: f64 = (1.0 - locals.var_delta_ns);
        let assign41450_e54400: f64 = (locals.var_xn_s * assign41450_e54399);
        let assign41450_e54401: f64 = (assign41450_e54400).sqrt();
        let assign41450_e54402: f64 = (2.0 * assign41450_e54401);
        let assign41450_e54403: f64 = (assign41450_e54394 / assign41450_e54402);
        let assign41450_e54404: f64 = (1.0 + assign41450_e54403);
        (assign41450_e54404, (((((((locals.var_temp__blk949_dn4 * locals.var_gf) + (locals.var_temp__blk949 * locals.var_gf_dn4)) * assign41450_e54393) + (assign41450_e54386 * (-((locals.var_delta_ns_dn4 * assign41450_e54391) + (locals.var_delta_ns * (-locals.var_xn_s_dn4)))))) * assign41450_e54402) - (assign41450_e54394 * (2.0 * (((locals.var_xn_s_dn4 * assign41450_e54399) + (locals.var_xn_s * (-locals.var_delta_ns_dn4))) / (2.0 * assign41450_e54401))))) / (assign41450_e54402 * assign41450_e54402)), (((((((locals.var_temp__blk949_dn6 * locals.var_gf) + (locals.var_temp__blk949 * locals.var_gf_dn6)) * assign41450_e54393) + (assign41450_e54386 * (-((locals.var_delta_ns_dn6 * assign41450_e54391) + (locals.var_delta_ns * (-locals.var_xn_s_dn6)))))) * assign41450_e54402) - (assign41450_e54394 * (2.0 * (((locals.var_xn_s_dn6 * assign41450_e54399) + (locals.var_xn_s * (-locals.var_delta_ns_dn6))) / (2.0 * assign41450_e54401))))) / (assign41450_e54402 * assign41450_e54402)), (((((((locals.var_temp__blk949_dn7 * locals.var_gf) + (locals.var_temp__blk949 * locals.var_gf_dn7)) * assign41450_e54393) + (assign41450_e54386 * (-((locals.var_delta_ns_dn7 * assign41450_e54391) + (locals.var_delta_ns * (-locals.var_xn_s_dn7)))))) * assign41450_e54402) - (assign41450_e54394 * (2.0 * (((locals.var_xn_s_dn7 * assign41450_e54399) + (locals.var_xn_s * (-locals.var_delta_ns_dn7))) / (2.0 * assign41450_e54401))))) / (assign41450_e54402 * assign41450_e54402)), (((((((locals.var_temp__blk949_dn8 * locals.var_gf) + (locals.var_temp__blk949 * locals.var_gf_dn8)) * assign41450_e54393) + (assign41450_e54386 * (-((locals.var_delta_ns_dn8 * assign41450_e54391) + (locals.var_delta_ns * (-locals.var_xn_s_dn8)))))) * assign41450_e54402) - (assign41450_e54394 * (2.0 * (((locals.var_xn_s_dn8 * assign41450_e54399) + (locals.var_xn_s * (-locals.var_delta_ns_dn8))) / (2.0 * assign41450_e54401))))) / (assign41450_e54402 * assign41450_e54402)), (((((((locals.var_temp__blk949_dn9 * locals.var_gf) + (locals.var_temp__blk949 * locals.var_gf_dn9)) * assign41450_e54393) + (assign41450_e54386 * (-((locals.var_delta_ns_dn9 * assign41450_e54391) + (locals.var_delta_ns * (-locals.var_xn_s_dn9)))))) * assign41450_e54402) - (assign41450_e54394 * (2.0 * (((locals.var_xn_s_dn9 * assign41450_e54399) + (locals.var_xn_s * (-locals.var_delta_ns_dn9))) / (2.0 * assign41450_e54401))))) / (assign41450_e54402 * assign41450_e54402)),)
    } else {
        (locals.var_nscr, locals.var_nscr_dn4, locals.var_nscr_dn6, locals.var_nscr_dn7, locals.var_nscr_dn8, locals.var_nscr_dn9,)
    }
};
        locals.var_nscr = assign41450_e54406;
        locals.var_nscr_dn4 = assign41450_e54406_d_n4;
        locals.var_nscr_dn6 = assign41450_e54406_d_n6;
        locals.var_nscr_dn7 = assign41450_e54406_d_n7;
        locals.var_nscr_dn8 = assign41450_e54406_d_n8;
        locals.var_nscr_dn9 = assign41450_e54406_d_n9;

        let (assign41460_e54418, assign41460_e54418_d_n4, assign41460_e54418_d_n6, assign41460_e54418_d_n7, assign41460_e54418_d_n8, assign41460_e54418_d_n9,) = {
    if (locals.var_guard1192 == 0.0) {
        let assign41460_e54412: f64 = (0.5 * locals.var_gf);
        let assign41460_e54414: f64 = (locals.var_xn_s).sqrt();
        let assign41460_e54415: f64 = (assign41460_e54412 / assign41460_e54414);
        let assign41460_e54416: f64 = (1.0 + assign41460_e54415);
        (assign41460_e54416, ((((0.5 * locals.var_gf_dn4) * assign41460_e54414) - (assign41460_e54412 * (locals.var_xn_s_dn4 / (2.0 * assign41460_e54414)))) / (assign41460_e54414 * assign41460_e54414)), ((((0.5 * locals.var_gf_dn6) * assign41460_e54414) - (assign41460_e54412 * (locals.var_xn_s_dn6 / (2.0 * assign41460_e54414)))) / (assign41460_e54414 * assign41460_e54414)), ((((0.5 * locals.var_gf_dn7) * assign41460_e54414) - (assign41460_e54412 * (locals.var_xn_s_dn7 / (2.0 * assign41460_e54414)))) / (assign41460_e54414 * assign41460_e54414)), ((((0.5 * locals.var_gf_dn8) * assign41460_e54414) - (assign41460_e54412 * (locals.var_xn_s_dn8 / (2.0 * assign41460_e54414)))) / (assign41460_e54414 * assign41460_e54414)), ((((0.5 * locals.var_gf_dn9) * assign41460_e54414) - (assign41460_e54412 * (locals.var_xn_s_dn9 / (2.0 * assign41460_e54414)))) / (assign41460_e54414 * assign41460_e54414)),)
    } else {
        (locals.var_nscr, locals.var_nscr_dn4, locals.var_nscr_dn6, locals.var_nscr_dn7, locals.var_nscr_dn8, locals.var_nscr_dn9,)
    }
};
        locals.var_nscr = assign41460_e54418;
        locals.var_nscr_dn4 = assign41460_e54418_d_n4;
        locals.var_nscr_dn6 = assign41460_e54418_d_n6;
        locals.var_nscr_dn7 = assign41460_e54418_d_n7;
        locals.var_nscr_dn8 = assign41460_e54418_d_n8;
        locals.var_nscr_dn9 = assign41460_e54418_d_n9;

        let assign41470_e54422: f64 = (locals.var_xn_s).sqrt();
        let assign41470_e54423: f64 = (locals.var_gf * assign41470_e54422);
        let assign41470_e54424: f64 = (locals.var_xn_s + assign41470_e54423);
        let assign41470_e54428: f64 = (locals.var_nscr - 1.0);
        let assign41470_e54429: f64 = (assign41470_e54428).ln();
        let assign41470_e54430: f64 = (locals.var_nscr * assign41470_e54429);
        let assign41470_e54431: f64 = (assign41470_e54424 - assign41470_e54430);
        locals.var_xthscr = assign41470_e54431;
        locals.var_xthscr_dn4 = ((locals.var_xn_s_dn4 + ((locals.var_gf_dn4 * assign41470_e54422) + (locals.var_gf * (locals.var_xn_s_dn4 / (2.0 * assign41470_e54422))))) - ((locals.var_nscr_dn4 * assign41470_e54429) + (locals.var_nscr * (locals.var_nscr_dn4 / assign41470_e54428))));
        locals.var_xthscr_dn6 = ((locals.var_xn_s_dn6 + ((locals.var_gf_dn6 * assign41470_e54422) + (locals.var_gf * (locals.var_xn_s_dn6 / (2.0 * assign41470_e54422))))) - ((locals.var_nscr_dn6 * assign41470_e54429) + (locals.var_nscr * (locals.var_nscr_dn6 / assign41470_e54428))));
        locals.var_xthscr_dn7 = ((locals.var_xn_s_dn7 + ((locals.var_gf_dn7 * assign41470_e54422) + (locals.var_gf * (locals.var_xn_s_dn7 / (2.0 * assign41470_e54422))))) - ((locals.var_nscr_dn7 * assign41470_e54429) + (locals.var_nscr * (locals.var_nscr_dn7 / assign41470_e54428))));
        locals.var_xthscr_dn8 = ((locals.var_xn_s_dn8 + ((locals.var_gf_dn8 * assign41470_e54422) + (locals.var_gf * (locals.var_xn_s_dn8 / (2.0 * assign41470_e54422))))) - ((locals.var_nscr_dn8 * assign41470_e54429) + (locals.var_nscr * (locals.var_nscr_dn8 / assign41470_e54428))));
        locals.var_xthscr_dn9 = ((locals.var_xn_s_dn9 + ((locals.var_gf_dn9 * assign41470_e54422) + (locals.var_gf * (locals.var_xn_s_dn9 / (2.0 * assign41470_e54422))))) - ((locals.var_nscr_dn9 * assign41470_e54429) + (locals.var_nscr * (locals.var_nscr_dn9 / assign41470_e54428))));

        let assign41480_e54434: f64 = (locals.var_xg - locals.var_xthscr);
        let assign41480_e54436: f64 = (assign41480_e54434 / locals.var_nscr);
        locals.var_xgtscr = assign41480_e54436;
        locals.var_xgtscr_dn4 = ((((locals.var_xg_dn4 - locals.var_xthscr_dn4) * locals.var_nscr) - (assign41480_e54434 * locals.var_nscr_dn4)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn6 = ((((locals.var_xg_dn6 - locals.var_xthscr_dn6) * locals.var_nscr) - (assign41480_e54434 * locals.var_nscr_dn6)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn7 = ((((locals.var_xg_dn7 - locals.var_xthscr_dn7) * locals.var_nscr) - (assign41480_e54434 * locals.var_nscr_dn7)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn8 = ((((locals.var_xg_dn8 - locals.var_xthscr_dn8) * locals.var_nscr) - (assign41480_e54434 * locals.var_nscr_dn8)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn9 = ((((locals.var_xg_dn9 - locals.var_xthscr_dn9) * locals.var_nscr) - (assign41480_e54434 * locals.var_nscr_dn9)) / (locals.var_nscr * locals.var_nscr));

        let assign41490_e54439: f64 = (0.5 * locals.var_gf2);
        let assign41490_e54443: f64 = (8.0 / locals.var_gf2);
        let assign41490_e54444: f64 = (1.0 + assign41490_e54443);
        let assign41490_e54445: f64 = (assign41490_e54444).sqrt();
        let assign41490_e54447: f64 = (assign41490_e54445 - 1.0);
        let assign41490_e54448: f64 = (assign41490_e54439 * assign41490_e54447);
        locals.var_qbscr = assign41490_e54448;
        locals.var_qbscr_dn4 = (((0.5 * locals.var_gf2_dn4) * assign41490_e54447) + (assign41490_e54439 * ((-((8.0 * locals.var_gf2_dn4) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41490_e54445))));
        locals.var_qbscr_dn6 = (((0.5 * locals.var_gf2_dn6) * assign41490_e54447) + (assign41490_e54439 * ((-((8.0 * locals.var_gf2_dn6) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41490_e54445))));
        locals.var_qbscr_dn7 = (((0.5 * locals.var_gf2_dn7) * assign41490_e54447) + (assign41490_e54439 * ((-((8.0 * locals.var_gf2_dn7) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41490_e54445))));
        locals.var_qbscr_dn8 = (((0.5 * locals.var_gf2_dn8) * assign41490_e54447) + (assign41490_e54439 * ((-((8.0 * locals.var_gf2_dn8) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41490_e54445))));
        locals.var_qbscr_dn9 = (((0.5 * locals.var_gf2_dn9) * assign41490_e54447) + (assign41490_e54439 * ((-((8.0 * locals.var_gf2_dn9) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41490_e54445))));

        locals.var_qiscr = 0.0;
        locals.var_qiscr_dn4 = 0.0;
        locals.var_qiscr_dn6 = 0.0;
        locals.var_qiscr_dn7 = 0.0;
        locals.var_qiscr_dn8 = 0.0;
        locals.var_qiscr_dn9 = 0.0;

        locals.var_fscr = 1.0;
        locals.var_fscr_dn4 = 0.0;
        locals.var_fscr_dn6 = 0.0;
        locals.var_fscr_dn7 = 0.0;
        locals.var_fscr_dn8 = 0.0;
        locals.var_fscr_dn9 = 0.0;

        let assign41520_e54453: f64 = (-30.0);
        let assign41520_e54454: f64 = if locals.var_xgtscr > assign41520_e54453 { 1.0 } else { 0.0 };
        locals.var_guard1195 = assign41520_e54454;

        let (assign41530_e54462, assign41530_e54462_d_n4, assign41530_e54462_d_n6, assign41530_e54462_d_n7, assign41530_e54462_d_n8, assign41530_e54462_d_n9,) = {
    if (locals.var_guard1195 != 0.0) {
        let assign41530_e54458: f64 = (locals.var_nscr * locals.var_xgtscr);
        let assign41530_e54460: f64 = (assign41530_e54458 - 1.0);
        (assign41530_e54460, ((locals.var_nscr_dn4 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn4)), ((locals.var_nscr_dn6 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn6)), ((locals.var_nscr_dn7 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn7)), ((locals.var_nscr_dn8 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn8)), ((locals.var_nscr_dn9 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn9)),)
    } else {
        (locals.var_xgtscr0, locals.var_xgtscr0_dn4, locals.var_xgtscr0_dn6, locals.var_xgtscr0_dn7, locals.var_xgtscr0_dn8, locals.var_xgtscr0_dn9,)
    }
};
        locals.var_xgtscr0 = assign41530_e54462;
        locals.var_xgtscr0_dn4 = assign41530_e54462_d_n4;
        locals.var_xgtscr0_dn6 = assign41530_e54462_d_n6;
        locals.var_xgtscr0_dn7 = assign41530_e54462_d_n7;
        locals.var_xgtscr0_dn8 = assign41530_e54462_d_n8;
        locals.var_xgtscr0_dn9 = assign41530_e54462_d_n9;

        let (assign41540_e54475, assign41540_e54475_d_n4, assign41540_e54475_d_n6, assign41540_e54475_d_n7, assign41540_e54475_d_n8, assign41540_e54475_d_n9,) = {
    if (locals.var_guard1195 != 0.0) {
        let assign41540_e54468: f64 = (locals.var_xgtscr0 * locals.var_xgtscr0);
        let assign41540_e54470: f64 = (assign41540_e54468 + 10.0);
        let assign41540_e54471: f64 = (assign41540_e54470).sqrt();
        let assign41540_e54472: f64 = (locals.var_xgtscr0 + assign41540_e54471);
        let assign41540_e54473: f64 = (0.5 * assign41540_e54472);
        (assign41540_e54473, (0.5 * (locals.var_xgtscr0_dn4 + (((locals.var_xgtscr0_dn4 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn4)) / (2.0 * assign41540_e54471)))), (0.5 * (locals.var_xgtscr0_dn6 + (((locals.var_xgtscr0_dn6 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn6)) / (2.0 * assign41540_e54471)))), (0.5 * (locals.var_xgtscr0_dn7 + (((locals.var_xgtscr0_dn7 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn7)) / (2.0 * assign41540_e54471)))), (0.5 * (locals.var_xgtscr0_dn8 + (((locals.var_xgtscr0_dn8 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn8)) / (2.0 * assign41540_e54471)))), (0.5 * (locals.var_xgtscr0_dn9 + (((locals.var_xgtscr0_dn9 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn9)) / (2.0 * assign41540_e54471)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign41540_e54475;
        locals.var_temp__blk949_dn4 = assign41540_e54475_d_n4;
        locals.var_temp__blk949_dn6 = assign41540_e54475_d_n6;
        locals.var_temp__blk949_dn7 = assign41540_e54475_d_n7;
        locals.var_temp__blk949_dn8 = assign41540_e54475_d_n8;
        locals.var_temp__blk949_dn9 = assign41540_e54475_d_n9;

        let (assign41550_e54482, assign41550_e54482_d_n4, assign41550_e54482_d_n6, assign41550_e54482_d_n7, assign41550_e54482_d_n8, assign41550_e54482_d_n9,) = {
    if (locals.var_guard1195 != 0.0) {
        let assign41550_e54479: f64 = (locals.var_temp__blk949).ln();
        let assign41550_e54480: f64 = (locals.var_xgtscr - assign41550_e54479);
        (assign41550_e54480, (locals.var_xgtscr_dn4 - (locals.var_temp__blk949_dn4 / locals.var_temp__blk949)), (locals.var_xgtscr_dn6 - (locals.var_temp__blk949_dn6 / locals.var_temp__blk949)), (locals.var_xgtscr_dn7 - (locals.var_temp__blk949_dn7 / locals.var_temp__blk949)), (locals.var_xgtscr_dn8 - (locals.var_temp__blk949_dn8 / locals.var_temp__blk949)), (locals.var_xgtscr_dn9 - (locals.var_temp__blk949_dn9 / locals.var_temp__blk949)),)
    } else {
        (locals.var_qiscr0si, locals.var_qiscr0si_dn4, locals.var_qiscr0si_dn6, locals.var_qiscr0si_dn7, locals.var_qiscr0si_dn8, locals.var_qiscr0si_dn9,)
    }
};
        locals.var_qiscr0si = assign41550_e54482;
        locals.var_qiscr0si_dn4 = assign41550_e54482_d_n4;
        locals.var_qiscr0si_dn6 = assign41550_e54482_d_n6;
        locals.var_qiscr0si_dn7 = assign41550_e54482_d_n7;
        locals.var_qiscr0si_dn8 = assign41550_e54482_d_n8;
        locals.var_qiscr0si_dn9 = assign41550_e54482_d_n9;

        let (assign41560_e54495, assign41560_e54495_d_n4, assign41560_e54495_d_n6, assign41560_e54495_d_n7, assign41560_e54495_d_n8, assign41560_e54495_d_n9,) = {
    if (locals.var_guard1195 != 0.0) {
        let assign41560_e54488: f64 = (locals.var_qiscr0si * locals.var_qiscr0si);
        let assign41560_e54490: f64 = (assign41560_e54488 + 2.0);
        let assign41560_e54491: f64 = (assign41560_e54490).sqrt();
        let assign41560_e54492: f64 = (locals.var_qiscr0si + assign41560_e54491);
        let assign41560_e54493: f64 = (0.5 * assign41560_e54492);
        (assign41560_e54493, (0.5 * (locals.var_qiscr0si_dn4 + (((locals.var_qiscr0si_dn4 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn4)) / (2.0 * assign41560_e54491)))), (0.5 * (locals.var_qiscr0si_dn6 + (((locals.var_qiscr0si_dn6 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn6)) / (2.0 * assign41560_e54491)))), (0.5 * (locals.var_qiscr0si_dn7 + (((locals.var_qiscr0si_dn7 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn7)) / (2.0 * assign41560_e54491)))), (0.5 * (locals.var_qiscr0si_dn8 + (((locals.var_qiscr0si_dn8 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn8)) / (2.0 * assign41560_e54491)))), (0.5 * (locals.var_qiscr0si_dn9 + (((locals.var_qiscr0si_dn9 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn9)) / (2.0 * assign41560_e54491)))),)
    } else {
        (locals.var_qiscr0, locals.var_qiscr0_dn4, locals.var_qiscr0_dn6, locals.var_qiscr0_dn7, locals.var_qiscr0_dn8, locals.var_qiscr0_dn9,)
    }
};
        locals.var_qiscr0 = assign41560_e54495;
        locals.var_qiscr0_dn4 = assign41560_e54495_d_n4;
        locals.var_qiscr0_dn6 = assign41560_e54495_d_n6;
        locals.var_qiscr0_dn7 = assign41560_e54495_d_n7;
        locals.var_qiscr0_dn8 = assign41560_e54495_d_n8;
        locals.var_qiscr0_dn9 = assign41560_e54495_d_n9;

        let assign41570_e54498: f64 = (locals.var_xgtscr - locals.var_qiscr0);
        let assign41570_e54500: f64 = if assign41570_e54498 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1196 = assign41570_e54500;

        let (assign41580_e54509, assign41580_e54509_d_n4, assign41580_e54509_d_n6, assign41580_e54509_d_n7, assign41580_e54509_d_n8, assign41580_e54509_d_n9,) = {
    if ((locals.var_guard1195 != 0.0) && (locals.var_guard1196 != 0.0)) {
        let assign41580_e54506: f64 = (locals.var_xgtscr - locals.var_qiscr0);
        let assign41580_e54507: f64 = (assign41580_e54506).exp();
        (assign41580_e54507, (assign41580_e54507 * (locals.var_xgtscr_dn4 - locals.var_qiscr0_dn4)), (assign41580_e54507 * (locals.var_xgtscr_dn6 - locals.var_qiscr0_dn6)), (assign41580_e54507 * (locals.var_xgtscr_dn7 - locals.var_qiscr0_dn7)), (assign41580_e54507 * (locals.var_xgtscr_dn8 - locals.var_qiscr0_dn8)), (assign41580_e54507 * (locals.var_xgtscr_dn9 - locals.var_qiscr0_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign41580_e54509;
        locals.var_temp__blk949_dn4 = assign41580_e54509_d_n4;
        locals.var_temp__blk949_dn6 = assign41580_e54509_d_n6;
        locals.var_temp__blk949_dn7 = assign41580_e54509_d_n7;
        locals.var_temp__blk949_dn8 = assign41580_e54509_d_n8;
        locals.var_temp__blk949_dn9 = assign41580_e54509_d_n9;

        let (assign41590_e54544, assign41590_e54544_d_n4, assign41590_e54544_d_n6, assign41590_e54544_d_n7, assign41590_e54544_d_n8, assign41590_e54544_d_n9,) = {
    if ((locals.var_guard1195 != 0.0) && (locals.var_guard1196 == 0.0)) {
        let assign41590_e54518: f64 = (locals.var_xgtscr - locals.var_qiscr0);
        let assign41590_e54520: f64 = (assign41590_e54518 - 230.25850929940458);
        let assign41590_e54525: f64 = (locals.var_xgtscr - locals.var_qiscr0);
        let assign41590_e54527: f64 = (assign41590_e54525 - 230.25850929940458);
        let assign41590_e54531: f64 = (locals.var_xgtscr - locals.var_qiscr0);
        let assign41590_e54533: f64 = (assign41590_e54531 - 230.25850929940458);
        let assign41590_e54535: f64 = (assign41590_e54533 * 0.3333333333333333);
        let assign41590_e54536: f64 = (1.0 + assign41590_e54535);
        let assign41590_e54537: f64 = (assign41590_e54527 * assign41590_e54536);
        let assign41590_e54538: f64 = (0.5 * assign41590_e54537);
        let assign41590_e54539: f64 = (1.0 + assign41590_e54538);
        let assign41590_e54540: f64 = (assign41590_e54520 * assign41590_e54539);
        let assign41590_e54541: f64 = (1.0 + assign41590_e54540);
        let assign41590_e54542: f64 = (1e100 * assign41590_e54541);
        (assign41590_e54542, (1e100 * (((locals.var_xgtscr_dn4 - locals.var_qiscr0_dn4) * assign41590_e54539) + (assign41590_e54520 * (0.5 * (((locals.var_xgtscr_dn4 - locals.var_qiscr0_dn4) * assign41590_e54536) + (assign41590_e54527 * ((locals.var_xgtscr_dn4 - locals.var_qiscr0_dn4) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn6 - locals.var_qiscr0_dn6) * assign41590_e54539) + (assign41590_e54520 * (0.5 * (((locals.var_xgtscr_dn6 - locals.var_qiscr0_dn6) * assign41590_e54536) + (assign41590_e54527 * ((locals.var_xgtscr_dn6 - locals.var_qiscr0_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn7 - locals.var_qiscr0_dn7) * assign41590_e54539) + (assign41590_e54520 * (0.5 * (((locals.var_xgtscr_dn7 - locals.var_qiscr0_dn7) * assign41590_e54536) + (assign41590_e54527 * ((locals.var_xgtscr_dn7 - locals.var_qiscr0_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn8 - locals.var_qiscr0_dn8) * assign41590_e54539) + (assign41590_e54520 * (0.5 * (((locals.var_xgtscr_dn8 - locals.var_qiscr0_dn8) * assign41590_e54536) + (assign41590_e54527 * ((locals.var_xgtscr_dn8 - locals.var_qiscr0_dn8) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn9 - locals.var_qiscr0_dn9) * assign41590_e54539) + (assign41590_e54520 * (0.5 * (((locals.var_xgtscr_dn9 - locals.var_qiscr0_dn9) * assign41590_e54536) + (assign41590_e54527 * ((locals.var_xgtscr_dn9 - locals.var_qiscr0_dn9) * 0.3333333333333333))))))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign41590_e54544;
        locals.var_temp__blk949_dn4 = assign41590_e54544_d_n4;
        locals.var_temp__blk949_dn6 = assign41590_e54544_d_n6;
        locals.var_temp__blk949_dn7 = assign41590_e54544_d_n7;
        locals.var_temp__blk949_dn8 = assign41590_e54544_d_n8;
        locals.var_temp__blk949_dn9 = assign41590_e54544_d_n9;

        let (assign41600_e54550, assign41600_e54550_d_n4, assign41600_e54550_d_n6, assign41600_e54550_d_n7, assign41600_e54550_d_n8, assign41600_e54550_d_n9,) = {
    if (locals.var_guard1195 != 0.0) {
        let assign41600_e54548: f64 = (locals.var_temp__blk949 / locals.var_nscr);
        (assign41600_e54548, (((locals.var_temp__blk949_dn4 * locals.var_nscr) - (locals.var_temp__blk949 * locals.var_nscr_dn4)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk949_dn6 * locals.var_nscr) - (locals.var_temp__blk949 * locals.var_nscr_dn6)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk949_dn7 * locals.var_nscr) - (locals.var_temp__blk949 * locals.var_nscr_dn7)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk949_dn8 * locals.var_nscr) - (locals.var_temp__blk949 * locals.var_nscr_dn8)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk949_dn9 * locals.var_nscr) - (locals.var_temp__blk949 * locals.var_nscr_dn9)) / (locals.var_nscr * locals.var_nscr)),)
    } else {
        (locals.var_dscr0, locals.var_dscr0_dn4, locals.var_dscr0_dn6, locals.var_dscr0_dn7, locals.var_dscr0_dn8, locals.var_dscr0_dn9,)
    }
};
        locals.var_dscr0 = assign41600_e54550;
        locals.var_dscr0_dn4 = assign41600_e54550_d_n4;
        locals.var_dscr0_dn6 = assign41600_e54550_d_n6;
        locals.var_dscr0_dn7 = assign41600_e54550_d_n7;
        locals.var_dscr0_dn8 = assign41600_e54550_d_n8;
        locals.var_dscr0_dn9 = assign41600_e54550_d_n9;

        let (assign41610_e54560, assign41610_e54560_d_n4, assign41610_e54560_d_n6, assign41610_e54560_d_n7, assign41610_e54560_d_n8, assign41610_e54560_d_n9,) = {
    if (locals.var_guard1195 != 0.0) {
        let assign41610_e54555: f64 = (locals.var_qiscr0 + 1.0);
        let assign41610_e54556: f64 = (2.0 * assign41610_e54555);
        let assign41610_e54558: f64 = (assign41610_e54556 - locals.var_dscr0);
        (assign41610_e54558, ((2.0 * locals.var_qiscr0_dn4) - locals.var_dscr0_dn4), ((2.0 * locals.var_qiscr0_dn6) - locals.var_dscr0_dn6), ((2.0 * locals.var_qiscr0_dn7) - locals.var_dscr0_dn7), ((2.0 * locals.var_qiscr0_dn8) - locals.var_dscr0_dn8), ((2.0 * locals.var_qiscr0_dn9) - locals.var_dscr0_dn9),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign41610_e54560;
        locals.var_temp__blk949_dn4 = assign41610_e54560_d_n4;
        locals.var_temp__blk949_dn6 = assign41610_e54560_d_n6;
        locals.var_temp__blk949_dn7 = assign41610_e54560_d_n7;
        locals.var_temp__blk949_dn8 = assign41610_e54560_d_n8;
        locals.var_temp__blk949_dn9 = assign41610_e54560_d_n9;

        let assign41620_e54563: f64 = if locals.var_dscr0 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1197 = assign41620_e54563;

        let (assign41630_e54584, assign41630_e54584_d_n4, assign41630_e54584_d_n6, assign41630_e54584_d_n7, assign41630_e54584_d_n8, assign41630_e54584_d_n9,) = {
    if ((locals.var_guard1195 != 0.0) && (locals.var_guard1197 != 0.0)) {
        let assign41630_e54572: f64 = (locals.var_dscr0 * locals.var_temp__blk949);
        let assign41630_e54573: f64 = (1.0 + assign41630_e54572);
        let assign41630_e54574: f64 = (assign41630_e54573).sqrt();
        let assign41630_e54576: f64 = (assign41630_e54574 - 1.0);
        let assign41630_e54578: f64 = (assign41630_e54576 / locals.var_dscr0);
        let assign41630_e54579: f64 = (locals.var_qiscr0 - assign41630_e54578);
        let assign41630_e54581: f64 = (assign41630_e54579 + 1.0);
        let assign41630_e54582: f64 = (locals.var_nscr * assign41630_e54581);
        (assign41630_e54582, ((locals.var_nscr_dn4 * assign41630_e54581) + (locals.var_nscr * (locals.var_qiscr0_dn4 - ((((((locals.var_dscr0_dn4 * locals.var_temp__blk949) + (locals.var_dscr0 * locals.var_temp__blk949_dn4)) / (2.0 * assign41630_e54574)) * locals.var_dscr0) - (assign41630_e54576 * locals.var_dscr0_dn4)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn6 * assign41630_e54581) + (locals.var_nscr * (locals.var_qiscr0_dn6 - ((((((locals.var_dscr0_dn6 * locals.var_temp__blk949) + (locals.var_dscr0 * locals.var_temp__blk949_dn6)) / (2.0 * assign41630_e54574)) * locals.var_dscr0) - (assign41630_e54576 * locals.var_dscr0_dn6)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn7 * assign41630_e54581) + (locals.var_nscr * (locals.var_qiscr0_dn7 - ((((((locals.var_dscr0_dn7 * locals.var_temp__blk949) + (locals.var_dscr0 * locals.var_temp__blk949_dn7)) / (2.0 * assign41630_e54574)) * locals.var_dscr0) - (assign41630_e54576 * locals.var_dscr0_dn7)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn8 * assign41630_e54581) + (locals.var_nscr * (locals.var_qiscr0_dn8 - ((((((locals.var_dscr0_dn8 * locals.var_temp__blk949) + (locals.var_dscr0 * locals.var_temp__blk949_dn8)) / (2.0 * assign41630_e54574)) * locals.var_dscr0) - (assign41630_e54576 * locals.var_dscr0_dn8)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn9 * assign41630_e54581) + (locals.var_nscr * (locals.var_qiscr0_dn9 - ((((((locals.var_dscr0_dn9 * locals.var_temp__blk949) + (locals.var_dscr0 * locals.var_temp__blk949_dn9)) / (2.0 * assign41630_e54574)) * locals.var_dscr0) - (assign41630_e54576 * locals.var_dscr0_dn9)) / (locals.var_dscr0 * locals.var_dscr0))))),)
    } else {
        (locals.var_qiscr, locals.var_qiscr_dn4, locals.var_qiscr_dn6, locals.var_qiscr_dn7, locals.var_qiscr_dn8, locals.var_qiscr_dn9,)
    }
};
        locals.var_qiscr = assign41630_e54584;
        locals.var_qiscr_dn4 = assign41630_e54584_d_n4;
        locals.var_qiscr_dn6 = assign41630_e54584_d_n6;
        locals.var_qiscr_dn7 = assign41630_e54584_d_n7;
        locals.var_qiscr_dn8 = assign41630_e54584_d_n8;
        locals.var_qiscr_dn9 = assign41630_e54584_d_n9;

        let (assign41640_e54603, assign41640_e54603_d_n4, assign41640_e54603_d_n6, assign41640_e54603_d_n7, assign41640_e54603_d_n8, assign41640_e54603_d_n9,) = {
    if ((locals.var_guard1195 != 0.0) && (locals.var_guard1197 == 0.0)) {
        let assign41640_e54591: f64 = (locals.var_nscr * 0.5);
        let assign41640_e54593: f64 = (assign41640_e54591 * locals.var_dscr0);
        let assign41640_e54597: f64 = (0.25 * locals.var_temp__blk949);
        let assign41640_e54599: f64 = (assign41640_e54597 * locals.var_temp__blk949);
        let assign41640_e54600: f64 = (1.0 + assign41640_e54599);
        let assign41640_e54601: f64 = (assign41640_e54593 * assign41640_e54600);
        (assign41640_e54601, (((((locals.var_nscr_dn4 * 0.5) * locals.var_dscr0) + (assign41640_e54591 * locals.var_dscr0_dn4)) * assign41640_e54600) + (assign41640_e54593 * (((0.25 * locals.var_temp__blk949_dn4) * locals.var_temp__blk949) + (assign41640_e54597 * locals.var_temp__blk949_dn4)))), (((((locals.var_nscr_dn6 * 0.5) * locals.var_dscr0) + (assign41640_e54591 * locals.var_dscr0_dn6)) * assign41640_e54600) + (assign41640_e54593 * (((0.25 * locals.var_temp__blk949_dn6) * locals.var_temp__blk949) + (assign41640_e54597 * locals.var_temp__blk949_dn6)))), (((((locals.var_nscr_dn7 * 0.5) * locals.var_dscr0) + (assign41640_e54591 * locals.var_dscr0_dn7)) * assign41640_e54600) + (assign41640_e54593 * (((0.25 * locals.var_temp__blk949_dn7) * locals.var_temp__blk949) + (assign41640_e54597 * locals.var_temp__blk949_dn7)))), (((((locals.var_nscr_dn8 * 0.5) * locals.var_dscr0) + (assign41640_e54591 * locals.var_dscr0_dn8)) * assign41640_e54600) + (assign41640_e54593 * (((0.25 * locals.var_temp__blk949_dn8) * locals.var_temp__blk949) + (assign41640_e54597 * locals.var_temp__blk949_dn8)))), (((((locals.var_nscr_dn9 * 0.5) * locals.var_dscr0) + (assign41640_e54591 * locals.var_dscr0_dn9)) * assign41640_e54600) + (assign41640_e54593 * (((0.25 * locals.var_temp__blk949_dn9) * locals.var_temp__blk949) + (assign41640_e54597 * locals.var_temp__blk949_dn9)))),)
    } else {
        (locals.var_qiscr, locals.var_qiscr_dn4, locals.var_qiscr_dn6, locals.var_qiscr_dn7, locals.var_qiscr_dn8, locals.var_qiscr_dn9,)
    }
};
        locals.var_qiscr = assign41640_e54603;
        locals.var_qiscr_dn4 = assign41640_e54603_d_n4;
        locals.var_qiscr_dn6 = assign41640_e54603_d_n6;
        locals.var_qiscr_dn7 = assign41640_e54603_d_n7;
        locals.var_qiscr_dn8 = assign41640_e54603_d_n8;
        locals.var_qiscr_dn9 = assign41640_e54603_d_n9;

        let (assign41650_e54628, assign41650_e54628_d_n4, assign41650_e54628_d_n6, assign41650_e54628_d_n7, assign41650_e54628_d_n8, assign41650_e54628_d_n9,) = {
    if (locals.var_guard1195 != 0.0) {
        let assign41650_e54608: f64 = (locals.var_xg - locals.var_qiscr);
        let assign41650_e54610: f64 = (assign41650_e54608 + 2.0);
        let assign41650_e54613: f64 = (locals.var_xg - locals.var_qiscr);
        let assign41650_e54615: f64 = (assign41650_e54613 - 2.0);
        let assign41650_e54618: f64 = (locals.var_xg - locals.var_qiscr);
        let assign41650_e54620: f64 = (assign41650_e54618 - 2.0);
        let assign41650_e54621: f64 = (assign41650_e54615 * assign41650_e54620);
        let assign41650_e54623: f64 = (assign41650_e54621 + 1.0);
        let assign41650_e54624: f64 = (assign41650_e54623).sqrt();
        let assign41650_e54625: f64 = (assign41650_e54610 + assign41650_e54624);
        let assign41650_e54626: f64 = (0.5 * assign41650_e54625);
        (assign41650_e54626, (0.5 * ((locals.var_xg_dn4 - locals.var_qiscr_dn4) + ((((locals.var_xg_dn4 - locals.var_qiscr_dn4) * assign41650_e54620) + (assign41650_e54615 * (locals.var_xg_dn4 - locals.var_qiscr_dn4))) / (2.0 * assign41650_e54624)))), (0.5 * ((locals.var_xg_dn6 - locals.var_qiscr_dn6) + ((((locals.var_xg_dn6 - locals.var_qiscr_dn6) * assign41650_e54620) + (assign41650_e54615 * (locals.var_xg_dn6 - locals.var_qiscr_dn6))) / (2.0 * assign41650_e54624)))), (0.5 * ((locals.var_xg_dn7 - locals.var_qiscr_dn7) + ((((locals.var_xg_dn7 - locals.var_qiscr_dn7) * assign41650_e54620) + (assign41650_e54615 * (locals.var_xg_dn7 - locals.var_qiscr_dn7))) / (2.0 * assign41650_e54624)))), (0.5 * ((locals.var_xg_dn8 - locals.var_qiscr_dn8) + ((((locals.var_xg_dn8 - locals.var_qiscr_dn8) * assign41650_e54620) + (assign41650_e54615 * (locals.var_xg_dn8 - locals.var_qiscr_dn8))) / (2.0 * assign41650_e54624)))), (0.5 * ((locals.var_xg_dn9 - locals.var_qiscr_dn9) + ((((locals.var_xg_dn9 - locals.var_qiscr_dn9) * assign41650_e54620) + (assign41650_e54615 * (locals.var_xg_dn9 - locals.var_qiscr_dn9))) / (2.0 * assign41650_e54624)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign41650_e54628;
        locals.var_temp__blk949_dn4 = assign41650_e54628_d_n4;
        locals.var_temp__blk949_dn6 = assign41650_e54628_d_n6;
        locals.var_temp__blk949_dn7 = assign41650_e54628_d_n7;
        locals.var_temp__blk949_dn8 = assign41650_e54628_d_n8;
        locals.var_temp__blk949_dn9 = assign41650_e54628_d_n9;

        let (assign41660_e54645, assign41660_e54645_d_n4, assign41660_e54645_d_n6, assign41660_e54645_d_n7, assign41660_e54645_d_n8, assign41660_e54645_d_n9,) = {
    if (locals.var_guard1195 != 0.0) {
        let assign41660_e54632: f64 = (0.5 * locals.var_gf2);
        let assign41660_e54636: f64 = (4.0 / locals.var_gf2);
        let assign41660_e54638: f64 = (assign41660_e54636 * locals.var_temp__blk949);
        let assign41660_e54639: f64 = (1.0 + assign41660_e54638);
        let assign41660_e54640: f64 = (assign41660_e54639).sqrt();
        let assign41660_e54642: f64 = (assign41660_e54640 - 1.0);
        let assign41660_e54643: f64 = (assign41660_e54632 * assign41660_e54642);
        (assign41660_e54643, (((0.5 * locals.var_gf2_dn4) * assign41660_e54642) + (assign41660_e54632 * ((((-((4.0 * locals.var_gf2_dn4) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk949) + (assign41660_e54636 * locals.var_temp__blk949_dn4)) / (2.0 * assign41660_e54640)))), (((0.5 * locals.var_gf2_dn6) * assign41660_e54642) + (assign41660_e54632 * ((((-((4.0 * locals.var_gf2_dn6) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk949) + (assign41660_e54636 * locals.var_temp__blk949_dn6)) / (2.0 * assign41660_e54640)))), (((0.5 * locals.var_gf2_dn7) * assign41660_e54642) + (assign41660_e54632 * ((((-((4.0 * locals.var_gf2_dn7) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk949) + (assign41660_e54636 * locals.var_temp__blk949_dn7)) / (2.0 * assign41660_e54640)))), (((0.5 * locals.var_gf2_dn8) * assign41660_e54642) + (assign41660_e54632 * ((((-((4.0 * locals.var_gf2_dn8) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk949) + (assign41660_e54636 * locals.var_temp__blk949_dn8)) / (2.0 * assign41660_e54640)))), (((0.5 * locals.var_gf2_dn9) * assign41660_e54642) + (assign41660_e54632 * ((((-((4.0 * locals.var_gf2_dn9) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk949) + (assign41660_e54636 * locals.var_temp__blk949_dn9)) / (2.0 * assign41660_e54640)))),)
    } else {
        (locals.var_qbscr, locals.var_qbscr_dn4, locals.var_qbscr_dn6, locals.var_qbscr_dn7, locals.var_qbscr_dn8, locals.var_qbscr_dn9,)
    }
};
        locals.var_qbscr = assign41660_e54645;
        locals.var_qbscr_dn4 = assign41660_e54645_d_n4;
        locals.var_qbscr_dn6 = assign41660_e54645_d_n6;
        locals.var_qbscr_dn7 = assign41660_e54645_d_n7;
        locals.var_qbscr_dn8 = assign41660_e54645_d_n8;
        locals.var_qbscr_dn9 = assign41660_e54645_d_n9;

        let (assign41670_e54653, assign41670_e54653_d_n4, assign41670_e54653_d_n6, assign41670_e54653_d_n7, assign41670_e54653_d_n8, assign41670_e54653_d_n9,) = {
    if (locals.var_guard1195 != 0.0) {
        let assign41670_e54650: f64 = (locals.var_qbscr + locals.var_qiscr);
        let assign41670_e54651: f64 = (locals.var_qbscr / assign41670_e54650);
        (assign41670_e54651, (((locals.var_qbscr_dn4 * assign41670_e54650) - (locals.var_qbscr * (locals.var_qbscr_dn4 + locals.var_qiscr_dn4))) / (assign41670_e54650 * assign41670_e54650)), (((locals.var_qbscr_dn6 * assign41670_e54650) - (locals.var_qbscr * (locals.var_qbscr_dn6 + locals.var_qiscr_dn6))) / (assign41670_e54650 * assign41670_e54650)), (((locals.var_qbscr_dn7 * assign41670_e54650) - (locals.var_qbscr * (locals.var_qbscr_dn7 + locals.var_qiscr_dn7))) / (assign41670_e54650 * assign41670_e54650)), (((locals.var_qbscr_dn8 * assign41670_e54650) - (locals.var_qbscr * (locals.var_qbscr_dn8 + locals.var_qiscr_dn8))) / (assign41670_e54650 * assign41670_e54650)), (((locals.var_qbscr_dn9 * assign41670_e54650) - (locals.var_qbscr * (locals.var_qbscr_dn9 + locals.var_qiscr_dn9))) / (assign41670_e54650 * assign41670_e54650)),)
    } else {
        (locals.var_fscr, locals.var_fscr_dn4, locals.var_fscr_dn6, locals.var_fscr_dn7, locals.var_fscr_dn8, locals.var_fscr_dn9,)
    }
};
        locals.var_fscr = assign41670_e54653;
        locals.var_fscr_dn4 = assign41670_e54653_d_n4;
        locals.var_fscr_dn6 = assign41670_e54653_d_n6;
        locals.var_fscr_dn7 = assign41670_e54653_d_n7;
        locals.var_fscr_dn8 = assign41670_e54653_d_n8;
        locals.var_fscr_dn9 = assign41670_e54653_d_n9;

        let (assign41680_e54661, assign41680_e54661_d_n4, assign41680_e54661_d_n6, assign41680_e54661_d_n7, assign41680_e54661_d_n8, assign41680_e54661_d_n9,) = {
    if (locals.var_guard1195 != 0.0) {
        let assign41680_e54658: f64 = (locals.var_fscr * locals.var_delxb);
        let assign41680_e54659: f64 = (locals.var_xno_s - assign41680_e54658);
        (assign41680_e54659, (locals.var_xno_s_dn4 - ((locals.var_fscr_dn4 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn4))), (locals.var_xno_s_dn6 - ((locals.var_fscr_dn6 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn6))), (locals.var_xno_s_dn7 - ((locals.var_fscr_dn7 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn7))), (locals.var_xno_s_dn8 - ((locals.var_fscr_dn8 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn8))), (locals.var_xno_s_dn9 - ((locals.var_fscr_dn9 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn9))),)
    } else {
        (locals.var_xn_s, locals.var_xn_s_dn4, locals.var_xn_s_dn6, locals.var_xn_s_dn7, locals.var_xn_s_dn8, locals.var_xn_s_dn9,)
    }
};
        locals.var_xn_s = assign41680_e54661;
        locals.var_xn_s_dn4 = assign41680_e54661_d_n4;
        locals.var_xn_s_dn6 = assign41680_e54661_d_n6;
        locals.var_xn_s_dn7 = assign41680_e54661_d_n7;
        locals.var_xn_s_dn8 = assign41680_e54661_d_n8;
        locals.var_xn_s_dn9 = assign41680_e54661_d_n9;

        let assign41690_e54665: f64 = (locals.var_gf * 0.7071067811865475);
        let assign41690_e54666: f64 = (1.0 + assign41690_e54665);
        locals.var_xi = assign41690_e54666;
        locals.var_xi_dn4 = (locals.var_gf_dn4 * 0.7071067811865475);
        locals.var_xi_dn6 = (locals.var_gf_dn6 * 0.7071067811865475);
        locals.var_xi_dn7 = (locals.var_gf_dn7 * 0.7071067811865475);
        locals.var_xi_dn8 = (locals.var_gf_dn8 * 0.7071067811865475);
        locals.var_xi_dn9 = (locals.var_gf_dn9 * 0.7071067811865475);

        let assign41700_e54669: f64 = (1e-5 * locals.var_xi);
        locals.var_margin = assign41700_e54669;

        let assign41710_e54672: f64 = (1.0 / locals.var_xi);
        locals.var_inv_xi = assign41710_e54672;
        locals.var_inv_xi_dn4 = (-(locals.var_xi_dn4 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn6 = (-(locals.var_xi_dn6 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn7 = (-(locals.var_xi_dn7 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn8 = (-(locals.var_xi_dn8 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn9 = (-(locals.var_xi_dn9 / (locals.var_xi * locals.var_xi)));

        locals.var_sp_s_x1 = 0.0;
        locals.var_sp_s_x1_dn4 = 0.0;
        locals.var_sp_s_x1_dn6 = 0.0;
        locals.var_sp_s_x1_dn7 = 0.0;
        locals.var_sp_s_x1_dn8 = 0.0;
        locals.var_sp_s_x1_dn9 = 0.0;

        locals.var_x_s = 0.0;
        locals.var_x_s_dn4 = 0.0;
        locals.var_x_s_dn6 = 0.0;
        locals.var_x_s_dn7 = 0.0;
        locals.var_x_s_dn8 = 0.0;
        locals.var_x_s_dn9 = 0.0;

        let assign41740_e54677: f64 = if locals.var_xn_s < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1198 = assign41740_e54677;

        let (assign41750_e54683, assign41750_e54683_d_n4, assign41750_e54683_d_n6, assign41750_e54683_d_n7, assign41750_e54683_d_n8, assign41750_e54683_d_n9,) = {
    if (locals.var_guard1198 != 0.0) {
        let assign41750_e54680: f64 = (-locals.var_xn_s);
        let assign41750_e54681: f64 = (assign41750_e54680).exp();
        (assign41750_e54681, (assign41750_e54681 * (-locals.var_xn_s_dn4)), (assign41750_e54681 * (-locals.var_xn_s_dn6)), (assign41750_e54681 * (-locals.var_xn_s_dn7)), (assign41750_e54681 * (-locals.var_xn_s_dn8)), (assign41750_e54681 * (-locals.var_xn_s_dn9)),)
    } else {
        (locals.var_delta_ns, locals.var_delta_ns_dn4, locals.var_delta_ns_dn6, locals.var_delta_ns_dn7, locals.var_delta_ns_dn8, locals.var_delta_ns_dn9,)
    }
};
        locals.var_delta_ns = assign41750_e54683;
        locals.var_delta_ns_dn4 = assign41750_e54683_d_n4;
        locals.var_delta_ns_dn6 = assign41750_e54683_d_n6;
        locals.var_delta_ns_dn7 = assign41750_e54683_d_n7;
        locals.var_delta_ns_dn8 = assign41750_e54683_d_n8;
        locals.var_delta_ns_dn9 = assign41750_e54683_d_n9;

        let (assign41760_e54710, assign41760_e54710_d_n4, assign41760_e54710_d_n6, assign41760_e54710_d_n7, assign41760_e54710_d_n8, assign41760_e54710_d_n9,) = {
    if (locals.var_guard1198 == 0.0) {
        let assign41760_e54690: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41760_e54695: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41760_e54699: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41760_e54701: f64 = (assign41760_e54699 * 0.3333333333333333);
        let assign41760_e54702: f64 = (1.0 + assign41760_e54701);
        let assign41760_e54703: f64 = (assign41760_e54695 * assign41760_e54702);
        let assign41760_e54704: f64 = (0.5 * assign41760_e54703);
        let assign41760_e54705: f64 = (1.0 + assign41760_e54704);
        let assign41760_e54706: f64 = (assign41760_e54690 * assign41760_e54705);
        let assign41760_e54707: f64 = (1.0 + assign41760_e54706);
        let assign41760_e54708: f64 = (1e-200 / assign41760_e54707);
        (assign41760_e54708, (-((1e-200 * ((locals.var_xn_s_dn4 * assign41760_e54705) + (assign41760_e54690 * (0.5 * ((locals.var_xn_s_dn4 * assign41760_e54702) + (assign41760_e54695 * (locals.var_xn_s_dn4 * 0.3333333333333333))))))) / (assign41760_e54707 * assign41760_e54707))), (-((1e-200 * ((locals.var_xn_s_dn6 * assign41760_e54705) + (assign41760_e54690 * (0.5 * ((locals.var_xn_s_dn6 * assign41760_e54702) + (assign41760_e54695 * (locals.var_xn_s_dn6 * 0.3333333333333333))))))) / (assign41760_e54707 * assign41760_e54707))), (-((1e-200 * ((locals.var_xn_s_dn7 * assign41760_e54705) + (assign41760_e54690 * (0.5 * ((locals.var_xn_s_dn7 * assign41760_e54702) + (assign41760_e54695 * (locals.var_xn_s_dn7 * 0.3333333333333333))))))) / (assign41760_e54707 * assign41760_e54707))), (-((1e-200 * ((locals.var_xn_s_dn8 * assign41760_e54705) + (assign41760_e54690 * (0.5 * ((locals.var_xn_s_dn8 * assign41760_e54702) + (assign41760_e54695 * (locals.var_xn_s_dn8 * 0.3333333333333333))))))) / (assign41760_e54707 * assign41760_e54707))), (-((1e-200 * ((locals.var_xn_s_dn9 * assign41760_e54705) + (assign41760_e54690 * (0.5 * ((locals.var_xn_s_dn9 * assign41760_e54702) + (assign41760_e54695 * (locals.var_xn_s_dn9 * 0.3333333333333333))))))) / (assign41760_e54707 * assign41760_e54707))),)
    } else {
        (locals.var_delta_ns, locals.var_delta_ns_dn4, locals.var_delta_ns_dn6, locals.var_delta_ns_dn7, locals.var_delta_ns_dn8, locals.var_delta_ns_dn9,)
    }
};
        locals.var_delta_ns = assign41760_e54710;
        locals.var_delta_ns_dn4 = assign41760_e54710_d_n4;
        locals.var_delta_ns_dn6 = assign41760_e54710_d_n6;
        locals.var_delta_ns_dn7 = assign41760_e54710_d_n7;
        locals.var_delta_ns_dn8 = assign41760_e54710_d_n8;
        locals.var_delta_ns_dn9 = assign41760_e54710_d_n9;

        let assign41770_e54712: f64 = (locals.var_xg).abs();
        let assign41770_e54714: f64 = if assign41770_e54712 <= locals.var_margin { 1.0 } else { 0.0 };
        locals.var_guard1199 = assign41770_e54714;

    }

    pub(super) fn stamp_transient_block_17(
        locals: &mut StampLocals,
    ) {
        let (assign41780_e54724, assign41780_e54724_d_n4, assign41780_e54724_d_n6, assign41780_e54724_d_n7, assign41780_e54724_d_n8, assign41780_e54724_d_n9,) = {
    if (locals.var_guard1199 != 0.0) {
        let assign41780_e54718: f64 = (locals.var_inv_xi * locals.var_inv_xi);
        let assign41780_e54720: f64 = (assign41780_e54718 * 0.16666666666666666);
        let assign41780_e54722: f64 = (assign41780_e54720 * 0.7071067811865475);
        (assign41780_e54722, ((((locals.var_inv_xi_dn4 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn4)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn6 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn7 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn8 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn8)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn9 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn9)) * 0.16666666666666666) * 0.7071067811865475),)
    } else {
        (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn4, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8, locals.var_sp_s_temp1_dn9,)
    }
};
        locals.var_sp_s_temp1 = assign41780_e54724;
        locals.var_sp_s_temp1_dn4 = assign41780_e54724_d_n4;
        locals.var_sp_s_temp1_dn6 = assign41780_e54724_d_n6;
        locals.var_sp_s_temp1_dn7 = assign41780_e54724_d_n7;
        locals.var_sp_s_temp1_dn8 = assign41780_e54724_d_n8;
        locals.var_sp_s_temp1_dn9 = assign41780_e54724_d_n9;

        let (assign41790_e54742, assign41790_e54742_d_n4, assign41790_e54742_d_n6, assign41790_e54742_d_n7, assign41790_e54742_d_n8, assign41790_e54742_d_n9,) = {
    if (locals.var_guard1199 != 0.0) {
        let assign41790_e54728: f64 = (locals.var_xg * locals.var_inv_xi);
        let assign41790_e54733: f64 = (1.0 - locals.var_delta_ns);
        let assign41790_e54734: f64 = (locals.var_xg * assign41790_e54733);
        let assign41790_e54736: f64 = (assign41790_e54734 * locals.var_gf);
        let assign41790_e54738: f64 = (assign41790_e54736 * locals.var_sp_s_temp1);
        let assign41790_e54739: f64 = (1.0 + assign41790_e54738);
        let assign41790_e54740: f64 = (assign41790_e54728 * assign41790_e54739);
        (assign41790_e54740, ((((locals.var_xg_dn4 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn4)) * assign41790_e54739) + (assign41790_e54728 * ((((((locals.var_xg_dn4 * assign41790_e54733) + (locals.var_xg * (-locals.var_delta_ns_dn4))) * locals.var_gf) + (assign41790_e54734 * locals.var_gf_dn4)) * locals.var_sp_s_temp1) + (assign41790_e54736 * locals.var_sp_s_temp1_dn4)))), ((((locals.var_xg_dn6 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn6)) * assign41790_e54739) + (assign41790_e54728 * ((((((locals.var_xg_dn6 * assign41790_e54733) + (locals.var_xg * (-locals.var_delta_ns_dn6))) * locals.var_gf) + (assign41790_e54734 * locals.var_gf_dn6)) * locals.var_sp_s_temp1) + (assign41790_e54736 * locals.var_sp_s_temp1_dn6)))), ((((locals.var_xg_dn7 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn7)) * assign41790_e54739) + (assign41790_e54728 * ((((((locals.var_xg_dn7 * assign41790_e54733) + (locals.var_xg * (-locals.var_delta_ns_dn7))) * locals.var_gf) + (assign41790_e54734 * locals.var_gf_dn7)) * locals.var_sp_s_temp1) + (assign41790_e54736 * locals.var_sp_s_temp1_dn7)))), ((((locals.var_xg_dn8 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn8)) * assign41790_e54739) + (assign41790_e54728 * ((((((locals.var_xg_dn8 * assign41790_e54733) + (locals.var_xg * (-locals.var_delta_ns_dn8))) * locals.var_gf) + (assign41790_e54734 * locals.var_gf_dn8)) * locals.var_sp_s_temp1) + (assign41790_e54736 * locals.var_sp_s_temp1_dn8)))), ((((locals.var_xg_dn9 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn9)) * assign41790_e54739) + (assign41790_e54728 * ((((((locals.var_xg_dn9 * assign41790_e54733) + (locals.var_xg * (-locals.var_delta_ns_dn9))) * locals.var_gf) + (assign41790_e54734 * locals.var_gf_dn9)) * locals.var_sp_s_temp1) + (assign41790_e54736 * locals.var_sp_s_temp1_dn9)))),)
    } else {
        (locals.var_x_s, locals.var_x_s_dn4, locals.var_x_s_dn6, locals.var_x_s_dn7, locals.var_x_s_dn8, locals.var_x_s_dn9,)
    }
};
        locals.var_x_s = assign41790_e54742;
        locals.var_x_s_dn4 = assign41790_e54742_d_n4;
        locals.var_x_s_dn6 = assign41790_e54742_d_n6;
        locals.var_x_s_dn7 = assign41790_e54742_d_n7;
        locals.var_x_s_dn8 = assign41790_e54742_d_n8;
        locals.var_x_s_dn9 = assign41790_e54742_d_n9;

        let assign41800_e54745: f64 = (-locals.var_margin);
        let assign41800_e54746: f64 = if locals.var_xg < assign41800_e54745 { 1.0 } else { 0.0 };
        locals.var_guard1200 = assign41800_e54746;

        let (assign41810_e54754, assign41810_e54754_d_n4, assign41810_e54754_d_n6, assign41810_e54754_d_n7, assign41810_e54754_d_n8, assign41810_e54754_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41810_e54752: f64 = (-locals.var_xg);
        (assign41810_e54752, (-locals.var_xg_dn4), (-locals.var_xg_dn6), (-locals.var_xg_dn7), (-locals.var_xg_dn8), (-locals.var_xg_dn9),)
    } else {
        (locals.var_sp_s_yg, locals.var_sp_s_yg_dn4, locals.var_sp_s_yg_dn6, locals.var_sp_s_yg_dn7, locals.var_sp_s_yg_dn8, locals.var_sp_s_yg_dn9,)
    }
};
        locals.var_sp_s_yg = assign41810_e54754;
        locals.var_sp_s_yg_dn4 = assign41810_e54754_d_n4;
        locals.var_sp_s_yg_dn6 = assign41810_e54754_d_n6;
        locals.var_sp_s_yg_dn7 = assign41810_e54754_d_n7;
        locals.var_sp_s_yg_dn8 = assign41810_e54754_d_n8;
        locals.var_sp_s_yg_dn9 = assign41810_e54754_d_n9;

        let (assign41820_e54765, assign41820_e54765_d_n4, assign41820_e54765_d_n6, assign41820_e54765_d_n7, assign41820_e54765_d_n8, assign41820_e54765_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41820_e54762: f64 = (locals.var_sp_s_yg * locals.var_inv_xi);
        let assign41820_e54763: f64 = (1.25 * assign41820_e54762);
        (assign41820_e54763, (1.25 * ((locals.var_sp_s_yg_dn4 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn4))), (1.25 * ((locals.var_sp_s_yg_dn6 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn6))), (1.25 * ((locals.var_sp_s_yg_dn7 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn7))), (1.25 * ((locals.var_sp_s_yg_dn8 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn8))), (1.25 * ((locals.var_sp_s_yg_dn9 * locals.var_inv_xi) + (locals.var_sp_s_yg * locals.var_inv_xi_dn9))),)
    } else {
        (locals.var_sp_s_ysub, locals.var_sp_s_ysub_dn4, locals.var_sp_s_ysub_dn6, locals.var_sp_s_ysub_dn7, locals.var_sp_s_ysub_dn8, locals.var_sp_s_ysub_dn9,)
    }
};
        locals.var_sp_s_ysub = assign41820_e54765;
        locals.var_sp_s_ysub_dn4 = assign41820_e54765_d_n4;
        locals.var_sp_s_ysub_dn6 = assign41820_e54765_d_n6;
        locals.var_sp_s_ysub_dn7 = assign41820_e54765_d_n7;
        locals.var_sp_s_ysub_dn8 = assign41820_e54765_d_n8;
        locals.var_sp_s_ysub_dn9 = assign41820_e54765_d_n9;

        let (assign41830_e54787, assign41830_e54787_d_n4, assign41830_e54787_d_n6, assign41830_e54787_d_n7, assign41830_e54787_d_n8, assign41830_e54787_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41830_e54773: f64 = (locals.var_sp_s_ysub + 10.0);
        let assign41830_e54776: f64 = (locals.var_sp_s_ysub - 6.0);
        let assign41830_e54779: f64 = (locals.var_sp_s_ysub - 6.0);
        let assign41830_e54780: f64 = (assign41830_e54776 * assign41830_e54779);
        let assign41830_e54782: f64 = (assign41830_e54780 + 64.0);
        let assign41830_e54783: f64 = (assign41830_e54782).sqrt();
        let assign41830_e54784: f64 = (assign41830_e54773 - assign41830_e54783);
        let assign41830_e54785: f64 = (0.5 * assign41830_e54784);
        (assign41830_e54785, (0.5 * (locals.var_sp_s_ysub_dn4 - (((locals.var_sp_s_ysub_dn4 * assign41830_e54779) + (assign41830_e54776 * locals.var_sp_s_ysub_dn4)) / (2.0 * assign41830_e54783)))), (0.5 * (locals.var_sp_s_ysub_dn6 - (((locals.var_sp_s_ysub_dn6 * assign41830_e54779) + (assign41830_e54776 * locals.var_sp_s_ysub_dn6)) / (2.0 * assign41830_e54783)))), (0.5 * (locals.var_sp_s_ysub_dn7 - (((locals.var_sp_s_ysub_dn7 * assign41830_e54779) + (assign41830_e54776 * locals.var_sp_s_ysub_dn7)) / (2.0 * assign41830_e54783)))), (0.5 * (locals.var_sp_s_ysub_dn8 - (((locals.var_sp_s_ysub_dn8 * assign41830_e54779) + (assign41830_e54776 * locals.var_sp_s_ysub_dn8)) / (2.0 * assign41830_e54783)))), (0.5 * (locals.var_sp_s_ysub_dn9 - (((locals.var_sp_s_ysub_dn9 * assign41830_e54779) + (assign41830_e54776 * locals.var_sp_s_ysub_dn9)) / (2.0 * assign41830_e54783)))),)
    } else {
        (locals.var_sp_s_eta, locals.var_sp_s_eta_dn4, locals.var_sp_s_eta_dn6, locals.var_sp_s_eta_dn7, locals.var_sp_s_eta_dn8, locals.var_sp_s_eta_dn9,)
    }
};
        locals.var_sp_s_eta = assign41830_e54787;
        locals.var_sp_s_eta_dn4 = assign41830_e54787_d_n4;
        locals.var_sp_s_eta_dn6 = assign41830_e54787_d_n6;
        locals.var_sp_s_eta_dn7 = assign41830_e54787_d_n7;
        locals.var_sp_s_eta_dn8 = assign41830_e54787_d_n8;
        locals.var_sp_s_eta_dn9 = assign41830_e54787_d_n9;

        let (assign41840_e54796, assign41840_e54796_d_n4, assign41840_e54796_d_n6, assign41840_e54796_d_n7, assign41840_e54796_d_n8, assign41840_e54796_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41840_e54794: f64 = (locals.var_sp_s_yg - locals.var_sp_s_eta);
        (assign41840_e54794, (locals.var_sp_s_yg_dn4 - locals.var_sp_s_eta_dn4), (locals.var_sp_s_yg_dn6 - locals.var_sp_s_eta_dn6), (locals.var_sp_s_yg_dn7 - locals.var_sp_s_eta_dn7), (locals.var_sp_s_yg_dn8 - locals.var_sp_s_eta_dn8), (locals.var_sp_s_yg_dn9 - locals.var_sp_s_eta_dn9),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign41840_e54796;
        locals.var_sp_s_temp_dn4 = assign41840_e54796_d_n4;
        locals.var_sp_s_temp_dn6 = assign41840_e54796_d_n6;
        locals.var_sp_s_temp_dn7 = assign41840_e54796_d_n7;
        locals.var_sp_s_temp_dn8 = assign41840_e54796_d_n8;
        locals.var_sp_s_temp_dn9 = assign41840_e54796_d_n9;

        let (assign41850_e54811, assign41850_e54811_d_n4, assign41850_e54811_d_n6, assign41850_e54811_d_n7, assign41850_e54811_d_n8, assign41850_e54811_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41850_e54803: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign41850_e54807: f64 = (locals.var_sp_s_eta + 1.0);
        let assign41850_e54808: f64 = (locals.var_gf2 * assign41850_e54807);
        let assign41850_e54809: f64 = (assign41850_e54803 + assign41850_e54808);
        (assign41850_e54809, (((locals.var_sp_s_temp_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn4)) + ((locals.var_gf2_dn4 * assign41850_e54807) + (locals.var_gf2 * locals.var_sp_s_eta_dn4))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) + ((locals.var_gf2_dn6 * assign41850_e54807) + (locals.var_gf2 * locals.var_sp_s_eta_dn6))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) + ((locals.var_gf2_dn7 * assign41850_e54807) + (locals.var_gf2 * locals.var_sp_s_eta_dn7))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) + ((locals.var_gf2_dn8 * assign41850_e54807) + (locals.var_gf2 * locals.var_sp_s_eta_dn8))), (((locals.var_sp_s_temp_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn9)) + ((locals.var_gf2_dn9 * assign41850_e54807) + (locals.var_gf2 * locals.var_sp_s_eta_dn9))),)
    } else {
        (locals.var_sp_s_a, locals.var_sp_s_a_dn4, locals.var_sp_s_a_dn6, locals.var_sp_s_a_dn7, locals.var_sp_s_a_dn8, locals.var_sp_s_a_dn9,)
    }
};
        locals.var_sp_s_a = assign41850_e54811;
        locals.var_sp_s_a_dn4 = assign41850_e54811_d_n4;
        locals.var_sp_s_a_dn6 = assign41850_e54811_d_n6;
        locals.var_sp_s_a_dn7 = assign41850_e54811_d_n7;
        locals.var_sp_s_a_dn8 = assign41850_e54811_d_n8;
        locals.var_sp_s_a_dn9 = assign41850_e54811_d_n9;

        let (assign41860_e54822, assign41860_e54822_d_n4, assign41860_e54822_d_n6, assign41860_e54822_d_n7, assign41860_e54822_d_n8, assign41860_e54822_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41860_e54818: f64 = (2.0 * locals.var_sp_s_temp);
        let assign41860_e54820: f64 = (assign41860_e54818 - locals.var_gf2);
        (assign41860_e54820, ((2.0 * locals.var_sp_s_temp_dn4) - locals.var_gf2_dn4), ((2.0 * locals.var_sp_s_temp_dn6) - locals.var_gf2_dn6), ((2.0 * locals.var_sp_s_temp_dn7) - locals.var_gf2_dn7), ((2.0 * locals.var_sp_s_temp_dn8) - locals.var_gf2_dn8), ((2.0 * locals.var_sp_s_temp_dn9) - locals.var_gf2_dn9),)
    } else {
        (locals.var_sp_s_c, locals.var_sp_s_c_dn4, locals.var_sp_s_c_dn6, locals.var_sp_s_c_dn7, locals.var_sp_s_c_dn8, locals.var_sp_s_c_dn9,)
    }
};
        locals.var_sp_s_c = assign41860_e54822;
        locals.var_sp_s_c_dn4 = assign41860_e54822_d_n4;
        locals.var_sp_s_c_dn6 = assign41860_e54822_d_n6;
        locals.var_sp_s_c_dn7 = assign41860_e54822_d_n7;
        locals.var_sp_s_c_dn8 = assign41860_e54822_d_n8;
        locals.var_sp_s_c_dn9 = assign41860_e54822_d_n9;

        let (assign41870_e54835, assign41870_e54835_d_n4, assign41870_e54835_d_n6, assign41870_e54835_d_n7, assign41870_e54835_d_n8, assign41870_e54835_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41870_e54828: f64 = (-locals.var_sp_s_eta);
        let assign41870_e54831: f64 = (locals.var_sp_s_a * locals.var_inv_gf2);
        let assign41870_e54832: f64 = (assign41870_e54831).ln();
        let assign41870_e54833: f64 = (assign41870_e54828 + assign41870_e54832);
        (assign41870_e54833, ((-locals.var_sp_s_eta_dn4) + (((locals.var_sp_s_a_dn4 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn4)) / assign41870_e54831)), ((-locals.var_sp_s_eta_dn6) + (((locals.var_sp_s_a_dn6 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn6)) / assign41870_e54831)), ((-locals.var_sp_s_eta_dn7) + (((locals.var_sp_s_a_dn7 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn7)) / assign41870_e54831)), ((-locals.var_sp_s_eta_dn8) + (((locals.var_sp_s_a_dn8 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn8)) / assign41870_e54831)), ((-locals.var_sp_s_eta_dn9) + (((locals.var_sp_s_a_dn9 * locals.var_inv_gf2) + (locals.var_sp_s_a * locals.var_inv_gf2_dn9)) / assign41870_e54831)),)
    } else {
        (locals.var_sp_s_tau, locals.var_sp_s_tau_dn4, locals.var_sp_s_tau_dn6, locals.var_sp_s_tau_dn7, locals.var_sp_s_tau_dn8, locals.var_sp_s_tau_dn9,)
    }
};
        locals.var_sp_s_tau = assign41870_e54835;
        locals.var_sp_s_tau_dn4 = assign41870_e54835_d_n4;
        locals.var_sp_s_tau_dn6 = assign41870_e54835_d_n6;
        locals.var_sp_s_tau_dn7 = assign41870_e54835_d_n7;
        locals.var_sp_s_tau_dn8 = assign41870_e54835_d_n8;
        locals.var_sp_s_tau_dn9 = assign41870_e54835_d_n9;

        let (assign41880_e54844, assign41880_e54844_d_n4, assign41880_e54844_d_n6, assign41880_e54844_d_n7, assign41880_e54844_d_n8, assign41880_e54844_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41880_e54842: f64 = (locals.var_sp_s_a + locals.var_sp_s_c);
        (assign41880_e54842, (locals.var_sp_s_a_dn4 + locals.var_sp_s_c_dn4), (locals.var_sp_s_a_dn6 + locals.var_sp_s_c_dn6), (locals.var_sp_s_a_dn7 + locals.var_sp_s_c_dn7), (locals.var_sp_s_a_dn8 + locals.var_sp_s_c_dn8), (locals.var_sp_s_a_dn9 + locals.var_sp_s_c_dn9),)
    } else {
        (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9,)
    }
};
        locals.var_nu = assign41880_e54844;
        locals.var_nu_dn4 = assign41880_e54844_d_n4;
        locals.var_nu_dn6 = assign41880_e54844_d_n6;
        locals.var_nu_dn7 = assign41880_e54844_d_n7;
        locals.var_nu_dn8 = assign41880_e54844_d_n8;
        locals.var_nu_dn9 = assign41880_e54844_d_n9;

        let (assign41890_e54863, assign41890_e54863_d_n4, assign41890_e54863_d_n6, assign41890_e54863_d_n7, assign41890_e54863_d_n8, assign41890_e54863_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41890_e54851: f64 = (locals.var_nu * locals.var_nu);
        let assign41890_e54856: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign41890_e54857: f64 = (0.5 * assign41890_e54856);
        let assign41890_e54859: f64 = (assign41890_e54857 - locals.var_sp_s_a);
        let assign41890_e54860: f64 = (locals.var_sp_s_tau * assign41890_e54859);
        let assign41890_e54861: f64 = (assign41890_e54851 + assign41890_e54860);
        (assign41890_e54861, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_sp_s_tau_dn4 * assign41890_e54859) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn4 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn4))) - locals.var_sp_s_a_dn4)))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau_dn6 * assign41890_e54859) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6))) - locals.var_sp_s_a_dn6)))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau_dn7 * assign41890_e54859) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7))) - locals.var_sp_s_a_dn7)))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau_dn8 * assign41890_e54859) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8))) - locals.var_sp_s_a_dn8)))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_sp_s_tau_dn9 * assign41890_e54859) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn9 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn9))) - locals.var_sp_s_a_dn9)))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9,)
    }
};
        locals.var_mutau = assign41890_e54863;
        locals.var_mutau_dn4 = assign41890_e54863_d_n4;
        locals.var_mutau_dn6 = assign41890_e54863_d_n6;
        locals.var_mutau_dn7 = assign41890_e54863_d_n7;
        locals.var_mutau_dn8 = assign41890_e54863_d_n8;
        locals.var_mutau_dn9 = assign41890_e54863_d_n9;

        let (assign41900_e54896, assign41900_e54896_d_n4, assign41900_e54896_d_n6, assign41900_e54896_d_n7, assign41900_e54896_d_n8, assign41900_e54896_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41900_e54871: f64 = (locals.var_sp_s_a * locals.var_nu);
        let assign41900_e54873: f64 = (assign41900_e54871 * locals.var_sp_s_tau);
        let assign41900_e54877: f64 = (locals.var_nu / locals.var_mutau);
        let assign41900_e54879: f64 = (assign41900_e54877 * locals.var_sp_s_tau);
        let assign41900_e54881: f64 = (assign41900_e54879 * locals.var_sp_s_tau);
        let assign41900_e54883: f64 = (assign41900_e54881 * locals.var_sp_s_c);
        let assign41900_e54886: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign41900_e54888: f64 = (assign41900_e54886 * 0.3333333333333333);
        let assign41900_e54890: f64 = (assign41900_e54888 - locals.var_sp_s_a);
        let assign41900_e54891: f64 = (assign41900_e54883 * assign41900_e54890);
        let assign41900_e54892: f64 = (locals.var_mutau + assign41900_e54891);
        let assign41900_e54893: f64 = (assign41900_e54873 / assign41900_e54892);
        let assign41900_e54894: f64 = (locals.var_sp_s_eta + assign41900_e54893);
        (assign41900_e54894, (locals.var_sp_s_eta_dn4 + (((((((locals.var_sp_s_a_dn4 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn4)) * locals.var_sp_s_tau) + (assign41900_e54871 * locals.var_sp_s_tau_dn4)) * assign41900_e54892) - (assign41900_e54873 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41900_e54877 * locals.var_sp_s_tau_dn4)) * locals.var_sp_s_tau) + (assign41900_e54879 * locals.var_sp_s_tau_dn4)) * locals.var_sp_s_c) + (assign41900_e54881 * locals.var_sp_s_c_dn4)) * assign41900_e54890) + (assign41900_e54883 * ((((locals.var_sp_s_c_dn4 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn4)) * 0.3333333333333333) - locals.var_sp_s_a_dn4)))))) / (assign41900_e54892 * assign41900_e54892))), (locals.var_sp_s_eta_dn6 + (((((((locals.var_sp_s_a_dn6 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn6)) * locals.var_sp_s_tau) + (assign41900_e54871 * locals.var_sp_s_tau_dn6)) * assign41900_e54892) - (assign41900_e54873 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41900_e54877 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_tau) + (assign41900_e54879 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_c) + (assign41900_e54881 * locals.var_sp_s_c_dn6)) * assign41900_e54890) + (assign41900_e54883 * ((((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6)) * 0.3333333333333333) - locals.var_sp_s_a_dn6)))))) / (assign41900_e54892 * assign41900_e54892))), (locals.var_sp_s_eta_dn7 + (((((((locals.var_sp_s_a_dn7 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn7)) * locals.var_sp_s_tau) + (assign41900_e54871 * locals.var_sp_s_tau_dn7)) * assign41900_e54892) - (assign41900_e54873 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41900_e54877 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_tau) + (assign41900_e54879 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_c) + (assign41900_e54881 * locals.var_sp_s_c_dn7)) * assign41900_e54890) + (assign41900_e54883 * ((((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7)) * 0.3333333333333333) - locals.var_sp_s_a_dn7)))))) / (assign41900_e54892 * assign41900_e54892))), (locals.var_sp_s_eta_dn8 + (((((((locals.var_sp_s_a_dn8 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn8)) * locals.var_sp_s_tau) + (assign41900_e54871 * locals.var_sp_s_tau_dn8)) * assign41900_e54892) - (assign41900_e54873 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41900_e54877 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_tau) + (assign41900_e54879 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_c) + (assign41900_e54881 * locals.var_sp_s_c_dn8)) * assign41900_e54890) + (assign41900_e54883 * ((((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8)) * 0.3333333333333333) - locals.var_sp_s_a_dn8)))))) / (assign41900_e54892 * assign41900_e54892))), (locals.var_sp_s_eta_dn9 + (((((((locals.var_sp_s_a_dn9 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn9)) * locals.var_sp_s_tau) + (assign41900_e54871 * locals.var_sp_s_tau_dn9)) * assign41900_e54892) - (assign41900_e54873 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign41900_e54877 * locals.var_sp_s_tau_dn9)) * locals.var_sp_s_tau) + (assign41900_e54879 * locals.var_sp_s_tau_dn9)) * locals.var_sp_s_c) + (assign41900_e54881 * locals.var_sp_s_c_dn9)) * assign41900_e54890) + (assign41900_e54883 * ((((locals.var_sp_s_c_dn9 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn9)) * 0.3333333333333333) - locals.var_sp_s_a_dn9)))))) / (assign41900_e54892 * assign41900_e54892))),)
    } else {
        (locals.var_sp_s_y0, locals.var_sp_s_y0_dn4, locals.var_sp_s_y0_dn6, locals.var_sp_s_y0_dn7, locals.var_sp_s_y0_dn8, locals.var_sp_s_y0_dn9,)
    }
};
        locals.var_sp_s_y0 = assign41900_e54896;
        locals.var_sp_s_y0_dn4 = assign41900_e54896_d_n4;
        locals.var_sp_s_y0_dn6 = assign41900_e54896_d_n6;
        locals.var_sp_s_y0_dn7 = assign41900_e54896_d_n7;
        locals.var_sp_s_y0_dn8 = assign41900_e54896_d_n8;
        locals.var_sp_s_y0_dn9 = assign41900_e54896_d_n9;

        let assign41910_e54899: f64 = if locals.var_sp_s_y0 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1201 = assign41910_e54899;

        let (assign41920_e54909, assign41920_e54909_d_n4, assign41920_e54909_d_n6, assign41920_e54909_d_n7, assign41920_e54909_d_n8, assign41920_e54909_d_n9,) = {
    if (((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) && (locals.var_guard1201 != 0.0)) {
        let assign41920_e54907: f64 = (locals.var_sp_s_y0).exp();
        (assign41920_e54907, (assign41920_e54907 * locals.var_sp_s_y0_dn4), (assign41920_e54907 * locals.var_sp_s_y0_dn6), (assign41920_e54907 * locals.var_sp_s_y0_dn7), (assign41920_e54907 * locals.var_sp_s_y0_dn8), (assign41920_e54907 * locals.var_sp_s_y0_dn9),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9,)
    }
};
        locals.var_sp_s_delta0 = assign41920_e54909;
        locals.var_sp_s_delta0_dn4 = assign41920_e54909_d_n4;
        locals.var_sp_s_delta0_dn6 = assign41920_e54909_d_n6;
        locals.var_sp_s_delta0_dn7 = assign41920_e54909_d_n7;
        locals.var_sp_s_delta0_dn8 = assign41920_e54909_d_n8;
        locals.var_sp_s_delta0_dn9 = assign41920_e54909_d_n9;

        let (assign41930_e54941, assign41930_e54941_d_n4, assign41930_e54941_d_n6, assign41930_e54941_d_n7, assign41930_e54941_d_n8, assign41930_e54941_d_n9,) = {
    if (((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) && (locals.var_guard1201 == 0.0)) {
        let assign41930_e54921: f64 = (locals.var_sp_s_y0 - 230.25850929940458);
        let assign41930_e54926: f64 = (locals.var_sp_s_y0 - 230.25850929940458);
        let assign41930_e54930: f64 = (locals.var_sp_s_y0 - 230.25850929940458);
        let assign41930_e54932: f64 = (assign41930_e54930 * 0.3333333333333333);
        let assign41930_e54933: f64 = (1.0 + assign41930_e54932);
        let assign41930_e54934: f64 = (assign41930_e54926 * assign41930_e54933);
        let assign41930_e54935: f64 = (0.5 * assign41930_e54934);
        let assign41930_e54936: f64 = (1.0 + assign41930_e54935);
        let assign41930_e54937: f64 = (assign41930_e54921 * assign41930_e54936);
        let assign41930_e54938: f64 = (1.0 + assign41930_e54937);
        let assign41930_e54939: f64 = (1e100 * assign41930_e54938);
        (assign41930_e54939, (1e100 * ((locals.var_sp_s_y0_dn4 * assign41930_e54936) + (assign41930_e54921 * (0.5 * ((locals.var_sp_s_y0_dn4 * assign41930_e54933) + (assign41930_e54926 * (locals.var_sp_s_y0_dn4 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn6 * assign41930_e54936) + (assign41930_e54921 * (0.5 * ((locals.var_sp_s_y0_dn6 * assign41930_e54933) + (assign41930_e54926 * (locals.var_sp_s_y0_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn7 * assign41930_e54936) + (assign41930_e54921 * (0.5 * ((locals.var_sp_s_y0_dn7 * assign41930_e54933) + (assign41930_e54926 * (locals.var_sp_s_y0_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn8 * assign41930_e54936) + (assign41930_e54921 * (0.5 * ((locals.var_sp_s_y0_dn8 * assign41930_e54933) + (assign41930_e54926 * (locals.var_sp_s_y0_dn8 * 0.3333333333333333))))))), (1e100 * ((locals.var_sp_s_y0_dn9 * assign41930_e54936) + (assign41930_e54921 * (0.5 * ((locals.var_sp_s_y0_dn9 * assign41930_e54933) + (assign41930_e54926 * (locals.var_sp_s_y0_dn9 * 0.3333333333333333))))))),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9,)
    }
};
        locals.var_sp_s_delta0 = assign41930_e54941;
        locals.var_sp_s_delta0_dn4 = assign41930_e54941_d_n4;
        locals.var_sp_s_delta0_dn6 = assign41930_e54941_d_n6;
        locals.var_sp_s_delta0_dn7 = assign41930_e54941_d_n7;
        locals.var_sp_s_delta0_dn8 = assign41930_e54941_d_n8;
        locals.var_sp_s_delta0_dn9 = assign41930_e54941_d_n9;

        let (assign41940_e54950, assign41940_e54950_d_n4, assign41940_e54950_d_n6, assign41940_e54950_d_n7, assign41940_e54950_d_n8, assign41940_e54950_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41940_e54948: f64 = (1.0 / locals.var_sp_s_delta0);
        (assign41940_e54948, (-(locals.var_sp_s_delta0_dn4 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn6 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn7 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn8 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn9 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn4, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, locals.var_sp_s_delta1_dn9,)
    }
};
        locals.var_sp_s_delta1 = assign41940_e54950;
        locals.var_sp_s_delta1_dn4 = assign41940_e54950_d_n4;
        locals.var_sp_s_delta1_dn6 = assign41940_e54950_d_n6;
        locals.var_sp_s_delta1_dn7 = assign41940_e54950_d_n7;
        locals.var_sp_s_delta1_dn8 = assign41940_e54950_d_n8;
        locals.var_sp_s_delta1_dn9 = assign41940_e54950_d_n9;

        let (assign41950_e54963, assign41950_e54963_d_n4, assign41950_e54963_d_n6, assign41950_e54963_d_n7, assign41950_e54963_d_n8, assign41950_e54963_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41950_e54959: f64 = (locals.var_sp_s_y0 * locals.var_sp_s_y0);
        let assign41950_e54960: f64 = (2.0 + assign41950_e54959);
        let assign41950_e54961: f64 = (1.0 / assign41950_e54960);
        (assign41950_e54961, (-(((locals.var_sp_s_y0_dn4 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn4)) / (assign41950_e54960 * assign41950_e54960))), (-(((locals.var_sp_s_y0_dn6 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn6)) / (assign41950_e54960 * assign41950_e54960))), (-(((locals.var_sp_s_y0_dn7 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn7)) / (assign41950_e54960 * assign41950_e54960))), (-(((locals.var_sp_s_y0_dn8 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn8)) / (assign41950_e54960 * assign41950_e54960))), (-(((locals.var_sp_s_y0_dn9 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn9)) / (assign41950_e54960 * assign41950_e54960))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign41950_e54963;
        locals.var_sp_s_temp_dn4 = assign41950_e54963_d_n4;
        locals.var_sp_s_temp_dn6 = assign41950_e54963_d_n6;
        locals.var_sp_s_temp_dn7 = assign41950_e54963_d_n7;
        locals.var_sp_s_temp_dn8 = assign41950_e54963_d_n8;
        locals.var_sp_s_temp_dn9 = assign41950_e54963_d_n9;

        let (assign41960_e54974, assign41960_e54974_d_n4, assign41960_e54974_d_n6, assign41960_e54974_d_n7, assign41960_e54974_d_n8, assign41960_e54974_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41960_e54970: f64 = (locals.var_sp_s_y0 * locals.var_sp_s_y0);
        let assign41960_e54972: f64 = (assign41960_e54970 * locals.var_sp_s_temp);
        (assign41960_e54972, ((((locals.var_sp_s_y0_dn4 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn4)) * locals.var_sp_s_temp) + (assign41960_e54970 * locals.var_sp_s_temp_dn4)), ((((locals.var_sp_s_y0_dn6 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn6)) * locals.var_sp_s_temp) + (assign41960_e54970 * locals.var_sp_s_temp_dn6)), ((((locals.var_sp_s_y0_dn7 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn7)) * locals.var_sp_s_temp) + (assign41960_e54970 * locals.var_sp_s_temp_dn7)), ((((locals.var_sp_s_y0_dn8 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn8)) * locals.var_sp_s_temp) + (assign41960_e54970 * locals.var_sp_s_temp_dn8)), ((((locals.var_sp_s_y0_dn9 * locals.var_sp_s_y0) + (locals.var_sp_s_y0 * locals.var_sp_s_y0_dn9)) * locals.var_sp_s_temp) + (assign41960_e54970 * locals.var_sp_s_temp_dn9)),)
    } else {
        (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn4, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8, locals.var_sp_s_xi0_dn9,)
    }
};
        locals.var_sp_s_xi0 = assign41960_e54974;
        locals.var_sp_s_xi0_dn4 = assign41960_e54974_d_n4;
        locals.var_sp_s_xi0_dn6 = assign41960_e54974_d_n6;
        locals.var_sp_s_xi0_dn7 = assign41960_e54974_d_n7;
        locals.var_sp_s_xi0_dn8 = assign41960_e54974_d_n8;
        locals.var_sp_s_xi0_dn9 = assign41960_e54974_d_n9;

        let (assign41970_e54987, assign41970_e54987_d_n4, assign41970_e54987_d_n6, assign41970_e54987_d_n7, assign41970_e54987_d_n8, assign41970_e54987_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41970_e54982: f64 = (locals.var_sp_s_y0 * locals.var_sp_s_temp);
        let assign41970_e54984: f64 = (assign41970_e54982 * locals.var_sp_s_temp);
        let assign41970_e54985: f64 = (4.0 * assign41970_e54984);
        (assign41970_e54985, (4.0 * ((((locals.var_sp_s_y0_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn4)) * locals.var_sp_s_temp) + (assign41970_e54982 * locals.var_sp_s_temp_dn4))), (4.0 * ((((locals.var_sp_s_y0_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign41970_e54982 * locals.var_sp_s_temp_dn6))), (4.0 * ((((locals.var_sp_s_y0_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign41970_e54982 * locals.var_sp_s_temp_dn7))), (4.0 * ((((locals.var_sp_s_y0_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign41970_e54982 * locals.var_sp_s_temp_dn8))), (4.0 * ((((locals.var_sp_s_y0_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_y0 * locals.var_sp_s_temp_dn9)) * locals.var_sp_s_temp) + (assign41970_e54982 * locals.var_sp_s_temp_dn9))),)
    } else {
        (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn4, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8, locals.var_sp_s_xi1_dn9,)
    }
};
        locals.var_sp_s_xi1 = assign41970_e54987;
        locals.var_sp_s_xi1_dn4 = assign41970_e54987_d_n4;
        locals.var_sp_s_xi1_dn6 = assign41970_e54987_d_n6;
        locals.var_sp_s_xi1_dn7 = assign41970_e54987_d_n7;
        locals.var_sp_s_xi1_dn8 = assign41970_e54987_d_n8;
        locals.var_sp_s_xi1_dn9 = assign41970_e54987_d_n9;

        let (assign41980_e55004, assign41980_e55004_d_n4, assign41980_e55004_d_n6, assign41980_e55004_d_n7, assign41980_e55004_d_n8, assign41980_e55004_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41980_e54994: f64 = (8.0 * locals.var_sp_s_temp);
        let assign41980_e54997: f64 = (12.0 * locals.var_sp_s_xi0);
        let assign41980_e54998: f64 = (assign41980_e54994 - assign41980_e54997);
        let assign41980_e55000: f64 = (assign41980_e54998 * locals.var_sp_s_temp);
        let assign41980_e55002: f64 = (assign41980_e55000 * locals.var_sp_s_temp);
        (assign41980_e55002, ((((((8.0 * locals.var_sp_s_temp_dn4) - (12.0 * locals.var_sp_s_xi0_dn4)) * locals.var_sp_s_temp) + (assign41980_e54998 * locals.var_sp_s_temp_dn4)) * locals.var_sp_s_temp) + (assign41980_e55000 * locals.var_sp_s_temp_dn4)), ((((((8.0 * locals.var_sp_s_temp_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp) + (assign41980_e54998 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign41980_e55000 * locals.var_sp_s_temp_dn6)), ((((((8.0 * locals.var_sp_s_temp_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp) + (assign41980_e54998 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign41980_e55000 * locals.var_sp_s_temp_dn7)), ((((((8.0 * locals.var_sp_s_temp_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp) + (assign41980_e54998 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign41980_e55000 * locals.var_sp_s_temp_dn8)), ((((((8.0 * locals.var_sp_s_temp_dn9) - (12.0 * locals.var_sp_s_xi0_dn9)) * locals.var_sp_s_temp) + (assign41980_e54998 * locals.var_sp_s_temp_dn9)) * locals.var_sp_s_temp) + (assign41980_e55000 * locals.var_sp_s_temp_dn9)),)
    } else {
        (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn4, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8, locals.var_sp_s_xi2_dn9,)
    }
};
        locals.var_sp_s_xi2 = assign41980_e55004;
        locals.var_sp_s_xi2_dn4 = assign41980_e55004_d_n4;
        locals.var_sp_s_xi2_dn6 = assign41980_e55004_d_n6;
        locals.var_sp_s_xi2_dn7 = assign41980_e55004_d_n7;
        locals.var_sp_s_xi2_dn8 = assign41980_e55004_d_n8;
        locals.var_sp_s_xi2_dn9 = assign41980_e55004_d_n9;

        let (assign41990_e55013, assign41990_e55013_d_n4, assign41990_e55013_d_n6, assign41990_e55013_d_n7, assign41990_e55013_d_n8, assign41990_e55013_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign41990_e55011: f64 = (locals.var_sp_s_yg - locals.var_sp_s_y0);
        (assign41990_e55011, (locals.var_sp_s_yg_dn4 - locals.var_sp_s_y0_dn4), (locals.var_sp_s_yg_dn6 - locals.var_sp_s_y0_dn6), (locals.var_sp_s_yg_dn7 - locals.var_sp_s_y0_dn7), (locals.var_sp_s_yg_dn8 - locals.var_sp_s_y0_dn8), (locals.var_sp_s_yg_dn9 - locals.var_sp_s_y0_dn9),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign41990_e55013;
        locals.var_sp_s_temp_dn4 = assign41990_e55013_d_n4;
        locals.var_sp_s_temp_dn6 = assign41990_e55013_d_n6;
        locals.var_sp_s_temp_dn7 = assign41990_e55013_d_n7;
        locals.var_sp_s_temp_dn8 = assign41990_e55013_d_n8;
        locals.var_sp_s_temp_dn9 = assign41990_e55013_d_n9;

        let (assign42000_e55022, assign42000_e55022_d_n4, assign42000_e55022_d_n6, assign42000_e55022_d_n7, assign42000_e55022_d_n8, assign42000_e55022_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign42000_e55020: f64 = (locals.var_delta_ns * locals.var_sp_s_delta1);
        (assign42000_e55020, ((locals.var_delta_ns_dn4 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn4)), ((locals.var_delta_ns_dn6 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn6)), ((locals.var_delta_ns_dn7 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn7)), ((locals.var_delta_ns_dn8 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn8)), ((locals.var_delta_ns_dn9 * locals.var_sp_s_delta1) + (locals.var_delta_ns * locals.var_sp_s_delta1_dn9)),)
    } else {
        (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn4, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8, locals.var_sp_s_temp1_dn9,)
    }
};
        locals.var_sp_s_temp1 = assign42000_e55022;
        locals.var_sp_s_temp1_dn4 = assign42000_e55022_d_n4;
        locals.var_sp_s_temp1_dn6 = assign42000_e55022_d_n6;
        locals.var_sp_s_temp1_dn7 = assign42000_e55022_d_n7;
        locals.var_sp_s_temp1_dn8 = assign42000_e55022_d_n8;
        locals.var_sp_s_temp1_dn9 = assign42000_e55022_d_n9;

        let (assign42010_e55045, assign42010_e55045_d_n4, assign42010_e55045_d_n6, assign42010_e55045_d_n7, assign42010_e55045_d_n8, assign42010_e55045_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign42010_e55029: f64 = (2.0 * locals.var_sp_s_temp);
        let assign42010_e55033: f64 = (locals.var_sp_s_delta0 - 1.0);
        let assign42010_e55035: f64 = (assign42010_e55033 - locals.var_sp_s_temp1);
        let assign42010_e55039: f64 = (1.0 - locals.var_sp_s_xi1);
        let assign42010_e55040: f64 = (locals.var_delta_ns * assign42010_e55039);
        let assign42010_e55041: f64 = (assign42010_e55035 + assign42010_e55040);
        let assign42010_e55042: f64 = (locals.var_gf2 * assign42010_e55041);
        let assign42010_e55043: f64 = (assign42010_e55029 + assign42010_e55042);
        (assign42010_e55043, ((2.0 * locals.var_sp_s_temp_dn4) + ((locals.var_gf2_dn4 * assign42010_e55041) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn4 - locals.var_sp_s_temp1_dn4) + ((locals.var_delta_ns_dn4 * assign42010_e55039) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn4))))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign42010_e55041) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn6 - locals.var_sp_s_temp1_dn6) + ((locals.var_delta_ns_dn6 * assign42010_e55039) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn6))))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign42010_e55041) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn7 - locals.var_sp_s_temp1_dn7) + ((locals.var_delta_ns_dn7 * assign42010_e55039) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn7))))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign42010_e55041) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn8 - locals.var_sp_s_temp1_dn8) + ((locals.var_delta_ns_dn8 * assign42010_e55039) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn8))))))), ((2.0 * locals.var_sp_s_temp_dn9) + ((locals.var_gf2_dn9 * assign42010_e55041) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn9 - locals.var_sp_s_temp1_dn9) + ((locals.var_delta_ns_dn9 * assign42010_e55039) + (locals.var_delta_ns * (-locals.var_sp_s_xi1_dn9))))))),)
    } else {
        (locals.var_sp_s_pc, locals.var_sp_s_pc_dn4, locals.var_sp_s_pc_dn6, locals.var_sp_s_pc_dn7, locals.var_sp_s_pc_dn8, locals.var_sp_s_pc_dn9,)
    }
};
        locals.var_sp_s_pc = assign42010_e55045;
        locals.var_sp_s_pc_dn4 = assign42010_e55045_d_n4;
        locals.var_sp_s_pc_dn6 = assign42010_e55045_d_n6;
        locals.var_sp_s_pc_dn7 = assign42010_e55045_d_n7;
        locals.var_sp_s_pc_dn8 = assign42010_e55045_d_n8;
        locals.var_sp_s_pc_dn9 = assign42010_e55045_d_n9;

        let (assign42020_e55072, assign42020_e55072_d_n4, assign42020_e55072_d_n6, assign42020_e55072_d_n7, assign42020_e55072_d_n8, assign42020_e55072_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign42020_e55052: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign42020_e55056: f64 = (locals.var_sp_s_delta0 - locals.var_sp_s_y0);
        let assign42020_e55058: f64 = (assign42020_e55056 - 1.0);
        let assign42020_e55060: f64 = (assign42020_e55058 + locals.var_sp_s_temp1);
        let assign42020_e55064: f64 = (locals.var_sp_s_y0 - 1.0);
        let assign42020_e55066: f64 = (assign42020_e55064 - locals.var_sp_s_xi0);
        let assign42020_e55067: f64 = (locals.var_delta_ns * assign42020_e55066);
        let assign42020_e55068: f64 = (assign42020_e55060 + assign42020_e55067);
        let assign42020_e55069: f64 = (locals.var_gf2 * assign42020_e55068);
        let assign42020_e55070: f64 = (assign42020_e55052 - assign42020_e55069);
        (assign42020_e55070, (((locals.var_sp_s_temp_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn4)) - ((locals.var_gf2_dn4 * assign42020_e55068) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn4 - locals.var_sp_s_y0_dn4) + locals.var_sp_s_temp1_dn4) + ((locals.var_delta_ns_dn4 * assign42020_e55066) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn4 - locals.var_sp_s_xi0_dn4))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign42020_e55068) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn6 - locals.var_sp_s_y0_dn6) + locals.var_sp_s_temp1_dn6) + ((locals.var_delta_ns_dn6 * assign42020_e55066) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn6 - locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign42020_e55068) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn7 - locals.var_sp_s_y0_dn7) + locals.var_sp_s_temp1_dn7) + ((locals.var_delta_ns_dn7 * assign42020_e55066) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn7 - locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign42020_e55068) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn8 - locals.var_sp_s_y0_dn8) + locals.var_sp_s_temp1_dn8) + ((locals.var_delta_ns_dn8 * assign42020_e55066) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn8 - locals.var_sp_s_xi0_dn8))))))), (((locals.var_sp_s_temp_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn9)) - ((locals.var_gf2_dn9 * assign42020_e55068) + (locals.var_gf2 * (((locals.var_sp_s_delta0_dn9 - locals.var_sp_s_y0_dn9) + locals.var_sp_s_temp1_dn9) + ((locals.var_delta_ns_dn9 * assign42020_e55066) + (locals.var_delta_ns * (locals.var_sp_s_y0_dn9 - locals.var_sp_s_xi0_dn9))))))),)
    } else {
        (locals.var_sp_s_qc, locals.var_sp_s_qc_dn4, locals.var_sp_s_qc_dn6, locals.var_sp_s_qc_dn7, locals.var_sp_s_qc_dn8, locals.var_sp_s_qc_dn9,)
    }
};
        locals.var_sp_s_qc = assign42020_e55072;
        locals.var_sp_s_qc_dn4 = assign42020_e55072_d_n4;
        locals.var_sp_s_qc_dn6 = assign42020_e55072_d_n6;
        locals.var_sp_s_qc_dn7 = assign42020_e55072_d_n7;
        locals.var_sp_s_qc_dn8 = assign42020_e55072_d_n8;
        locals.var_sp_s_qc_dn9 = assign42020_e55072_d_n9;

        let (assign42030_e55089, assign42030_e55089_d_n4, assign42030_e55089_d_n6, assign42030_e55089_d_n7, assign42030_e55089_d_n8, assign42030_e55089_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign42030_e55081: f64 = (locals.var_sp_s_delta0 + locals.var_sp_s_temp1);
        let assign42030_e55084: f64 = (locals.var_delta_ns * locals.var_sp_s_xi2);
        let assign42030_e55085: f64 = (assign42030_e55081 - assign42030_e55084);
        let assign42030_e55086: f64 = (locals.var_gf2 * assign42030_e55085);
        let assign42030_e55087: f64 = (2.0 - assign42030_e55086);
        (assign42030_e55087, (-((locals.var_gf2_dn4 * assign42030_e55085) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn4 + locals.var_sp_s_temp1_dn4) - ((locals.var_delta_ns_dn4 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn4)))))), (-((locals.var_gf2_dn6 * assign42030_e55085) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn6 + locals.var_sp_s_temp1_dn6) - ((locals.var_delta_ns_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn6)))))), (-((locals.var_gf2_dn7 * assign42030_e55085) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn7 + locals.var_sp_s_temp1_dn7) - ((locals.var_delta_ns_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn7)))))), (-((locals.var_gf2_dn8 * assign42030_e55085) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn8 + locals.var_sp_s_temp1_dn8) - ((locals.var_delta_ns_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn8)))))), (-((locals.var_gf2_dn9 * assign42030_e55085) + (locals.var_gf2 * ((locals.var_sp_s_delta0_dn9 + locals.var_sp_s_temp1_dn9) - ((locals.var_delta_ns_dn9 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn9)))))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign42030_e55089;
        locals.var_sp_s_temp_dn4 = assign42030_e55089_d_n4;
        locals.var_sp_s_temp_dn6 = assign42030_e55089_d_n6;
        locals.var_sp_s_temp_dn7 = assign42030_e55089_d_n7;
        locals.var_sp_s_temp_dn8 = assign42030_e55089_d_n8;
        locals.var_sp_s_temp_dn9 = assign42030_e55089_d_n9;

        let (assign42040_e55104, assign42040_e55104_d_n4, assign42040_e55104_d_n6, assign42040_e55104_d_n7, assign42040_e55104_d_n8, assign42040_e55104_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign42040_e55096: f64 = (locals.var_sp_s_pc * locals.var_sp_s_pc);
        let assign42040_e55100: f64 = (locals.var_sp_s_qc * locals.var_sp_s_temp);
        let assign42040_e55101: f64 = (2.0 * assign42040_e55100);
        let assign42040_e55102: f64 = (assign42040_e55096 - assign42040_e55101);
        (assign42040_e55102, (((locals.var_sp_s_pc_dn4 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn4)) - (2.0 * ((locals.var_sp_s_qc_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn4)))), (((locals.var_sp_s_pc_dn6 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn6)) - (2.0 * ((locals.var_sp_s_qc_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn6)))), (((locals.var_sp_s_pc_dn7 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn7)) - (2.0 * ((locals.var_sp_s_qc_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn7)))), (((locals.var_sp_s_pc_dn8 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn8)) - (2.0 * ((locals.var_sp_s_qc_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn8)))), (((locals.var_sp_s_pc_dn9 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn9)) - (2.0 * ((locals.var_sp_s_qc_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn9)))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign42040_e55104;
        locals.var_sp_s_temp_dn4 = assign42040_e55104_d_n4;
        locals.var_sp_s_temp_dn6 = assign42040_e55104_d_n6;
        locals.var_sp_s_temp_dn7 = assign42040_e55104_d_n7;
        locals.var_sp_s_temp_dn8 = assign42040_e55104_d_n8;
        locals.var_sp_s_temp_dn9 = assign42040_e55104_d_n9;

        let (assign42050_e55121, assign42050_e55121_d_n4, assign42050_e55121_d_n6, assign42050_e55121_d_n7, assign42050_e55121_d_n8, assign42050_e55121_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 != 0.0)) {
        let assign42050_e55110: f64 = (-locals.var_sp_s_y0);
        let assign42050_e55115: f64 = (locals.var_sp_s_temp).sqrt();
        let assign42050_e55116: f64 = (locals.var_sp_s_pc + assign42050_e55115);
        let assign42050_e55117: f64 = (locals.var_sp_s_qc / assign42050_e55116);
        let assign42050_e55118: f64 = (2.0 * assign42050_e55117);
        let assign42050_e55119: f64 = (assign42050_e55110 - assign42050_e55118);
        (assign42050_e55119, ((-locals.var_sp_s_y0_dn4) - (2.0 * (((locals.var_sp_s_qc_dn4 * assign42050_e55116) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn4 + (locals.var_sp_s_temp_dn4 / (2.0 * assign42050_e55115))))) / (assign42050_e55116 * assign42050_e55116)))), ((-locals.var_sp_s_y0_dn6) - (2.0 * (((locals.var_sp_s_qc_dn6 * assign42050_e55116) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn6 + (locals.var_sp_s_temp_dn6 / (2.0 * assign42050_e55115))))) / (assign42050_e55116 * assign42050_e55116)))), ((-locals.var_sp_s_y0_dn7) - (2.0 * (((locals.var_sp_s_qc_dn7 * assign42050_e55116) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn7 + (locals.var_sp_s_temp_dn7 / (2.0 * assign42050_e55115))))) / (assign42050_e55116 * assign42050_e55116)))), ((-locals.var_sp_s_y0_dn8) - (2.0 * (((locals.var_sp_s_qc_dn8 * assign42050_e55116) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn8 + (locals.var_sp_s_temp_dn8 / (2.0 * assign42050_e55115))))) / (assign42050_e55116 * assign42050_e55116)))), ((-locals.var_sp_s_y0_dn9) - (2.0 * (((locals.var_sp_s_qc_dn9 * assign42050_e55116) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn9 + (locals.var_sp_s_temp_dn9 / (2.0 * assign42050_e55115))))) / (assign42050_e55116 * assign42050_e55116)))),)
    } else {
        (locals.var_x_s, locals.var_x_s_dn4, locals.var_x_s_dn6, locals.var_x_s_dn7, locals.var_x_s_dn8, locals.var_x_s_dn9,)
    }
};
        locals.var_x_s = assign42050_e55121;
        locals.var_x_s_dn4 = assign42050_e55121_d_n4;
        locals.var_x_s_dn6 = assign42050_e55121_d_n6;
        locals.var_x_s_dn7 = assign42050_e55121_d_n7;
        locals.var_x_s_dn8 = assign42050_e55121_d_n8;
        locals.var_x_s_dn9 = assign42050_e55121_d_n9;

        let (assign42060_e55135, assign42060_e55135_d_n4, assign42060_e55135_d_n6, assign42060_e55135_d_n7, assign42060_e55135_d_n8, assign42060_e55135_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42060_e55131: f64 = (locals.var_gf * 0.7324648775608221);
        let assign42060_e55132: f64 = (1.25 + assign42060_e55131);
        let assign42060_e55133: f64 = (1.0 / assign42060_e55132);
        (assign42060_e55133, (-((locals.var_gf_dn4 * 0.7324648775608221) / (assign42060_e55132 * assign42060_e55132))), (-((locals.var_gf_dn6 * 0.7324648775608221) / (assign42060_e55132 * assign42060_e55132))), (-((locals.var_gf_dn7 * 0.7324648775608221) / (assign42060_e55132 * assign42060_e55132))), (-((locals.var_gf_dn8 * 0.7324648775608221) / (assign42060_e55132 * assign42060_e55132))), (-((locals.var_gf_dn9 * 0.7324648775608221) / (assign42060_e55132 * assign42060_e55132))),)
    } else {
        (locals.var_sp_xg1, locals.var_sp_xg1_dn4, locals.var_sp_xg1_dn6, locals.var_sp_xg1_dn7, locals.var_sp_xg1_dn8, locals.var_sp_xg1_dn9,)
    }
};
        locals.var_sp_xg1 = assign42060_e55135;
        locals.var_sp_xg1_dn4 = assign42060_e55135_d_n4;
        locals.var_sp_xg1_dn6 = assign42060_e55135_d_n6;
        locals.var_sp_xg1_dn7 = assign42060_e55135_d_n7;
        locals.var_sp_xg1_dn8 = assign42060_e55135_d_n8;
        locals.var_sp_xg1_dn9 = assign42060_e55135_d_n9;

    }

    pub(super) fn stamp_transient_block_18(
        locals: &mut StampLocals,
    ) {
        let (assign42070_e55151, assign42070_e55151_d_n4, assign42070_e55151_d_n6, assign42070_e55151_d_n7, assign42070_e55151_d_n8, assign42070_e55151_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42070_e55143: f64 = (locals.var_xi * 1.25);
        let assign42070_e55145: f64 = (assign42070_e55143 * locals.var_sp_xg1);
        let assign42070_e55147: f64 = (assign42070_e55145 - 1.0);
        let assign42070_e55149: f64 = (assign42070_e55147 * locals.var_sp_xg1);
        (assign42070_e55149, (((((locals.var_xi_dn4 * 1.25) * locals.var_sp_xg1) + (assign42070_e55143 * locals.var_sp_xg1_dn4)) * locals.var_sp_xg1) + (assign42070_e55147 * locals.var_sp_xg1_dn4)), (((((locals.var_xi_dn6 * 1.25) * locals.var_sp_xg1) + (assign42070_e55143 * locals.var_sp_xg1_dn6)) * locals.var_sp_xg1) + (assign42070_e55147 * locals.var_sp_xg1_dn6)), (((((locals.var_xi_dn7 * 1.25) * locals.var_sp_xg1) + (assign42070_e55143 * locals.var_sp_xg1_dn7)) * locals.var_sp_xg1) + (assign42070_e55147 * locals.var_sp_xg1_dn7)), (((((locals.var_xi_dn8 * 1.25) * locals.var_sp_xg1) + (assign42070_e55143 * locals.var_sp_xg1_dn8)) * locals.var_sp_xg1) + (assign42070_e55147 * locals.var_sp_xg1_dn8)), (((((locals.var_xi_dn9 * 1.25) * locals.var_sp_xg1) + (assign42070_e55143 * locals.var_sp_xg1_dn9)) * locals.var_sp_xg1) + (assign42070_e55147 * locals.var_sp_xg1_dn9)),)
    } else {
        (locals.var_sp_s_a_fac, locals.var_sp_s_a_fac_dn4, locals.var_sp_s_a_fac_dn6, locals.var_sp_s_a_fac_dn7, locals.var_sp_s_a_fac_dn8, locals.var_sp_s_a_fac_dn9,)
    }
};
        locals.var_sp_s_a_fac = assign42070_e55151;
        locals.var_sp_s_a_fac_dn4 = assign42070_e55151_d_n4;
        locals.var_sp_s_a_fac_dn6 = assign42070_e55151_d_n6;
        locals.var_sp_s_a_fac_dn7 = assign42070_e55151_d_n7;
        locals.var_sp_s_a_fac_dn8 = assign42070_e55151_d_n8;
        locals.var_sp_s_a_fac_dn9 = assign42070_e55151_d_n9;

        let (assign42080_e55167, assign42080_e55167_d_n4, assign42080_e55167_d_n6, assign42080_e55167_d_n7, assign42080_e55167_d_n8, assign42080_e55167_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42080_e55159: f64 = (locals.var_xg * locals.var_inv_xi);
        let assign42080_e55163: f64 = (locals.var_sp_s_a_fac * locals.var_xg);
        let assign42080_e55164: f64 = (1.0 + assign42080_e55163);
        let assign42080_e55165: f64 = (assign42080_e55159 * assign42080_e55164);
        (assign42080_e55165, ((((locals.var_xg_dn4 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn4)) * assign42080_e55164) + (assign42080_e55159 * ((locals.var_sp_s_a_fac_dn4 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn4)))), ((((locals.var_xg_dn6 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn6)) * assign42080_e55164) + (assign42080_e55159 * ((locals.var_sp_s_a_fac_dn6 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn6)))), ((((locals.var_xg_dn7 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn7)) * assign42080_e55164) + (assign42080_e55159 * ((locals.var_sp_s_a_fac_dn7 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn7)))), ((((locals.var_xg_dn8 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn8)) * assign42080_e55164) + (assign42080_e55159 * ((locals.var_sp_s_a_fac_dn8 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn8)))), ((((locals.var_xg_dn9 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn9)) * assign42080_e55164) + (assign42080_e55159 * ((locals.var_sp_s_a_fac_dn9 * locals.var_xg) + (locals.var_sp_s_a_fac * locals.var_xg_dn9)))),)
    } else {
        (locals.var_sp_s_xbar, locals.var_sp_s_xbar_dn4, locals.var_sp_s_xbar_dn6, locals.var_sp_s_xbar_dn7, locals.var_sp_s_xbar_dn8, locals.var_sp_s_xbar_dn9,)
    }
};
        locals.var_sp_s_xbar = assign42080_e55167;
        locals.var_sp_s_xbar_dn4 = assign42080_e55167_d_n4;
        locals.var_sp_s_xbar_dn6 = assign42080_e55167_d_n6;
        locals.var_sp_s_xbar_dn7 = assign42080_e55167_d_n7;
        locals.var_sp_s_xbar_dn8 = assign42080_e55167_d_n8;
        locals.var_sp_s_xbar_dn9 = assign42080_e55167_d_n9;

        let assign42090_e55169: f64 = (-locals.var_sp_s_xbar);
        let assign42090_e55171: f64 = (-230.25850929940458);
        let assign42090_e55172: f64 = if assign42090_e55169 > assign42090_e55171 { 1.0 } else { 0.0 };
        locals.var_guard1202 = assign42090_e55172;

        let (assign42100_e55184, assign42100_e55184_d_n4, assign42100_e55184_d_n6, assign42100_e55184_d_n7, assign42100_e55184_d_n8, assign42100_e55184_d_n9,) = {
    if (((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) && (locals.var_guard1202 != 0.0)) {
        let assign42100_e55181: f64 = (-locals.var_sp_s_xbar);
        let assign42100_e55182: f64 = (assign42100_e55181).exp();
        (assign42100_e55182, (assign42100_e55182 * (-locals.var_sp_s_xbar_dn4)), (assign42100_e55182 * (-locals.var_sp_s_xbar_dn6)), (assign42100_e55182 * (-locals.var_sp_s_xbar_dn7)), (assign42100_e55182 * (-locals.var_sp_s_xbar_dn8)), (assign42100_e55182 * (-locals.var_sp_s_xbar_dn9)),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign42100_e55184;
        locals.var_sp_s_temp_dn4 = assign42100_e55184_d_n4;
        locals.var_sp_s_temp_dn6 = assign42100_e55184_d_n6;
        locals.var_sp_s_temp_dn7 = assign42100_e55184_d_n7;
        locals.var_sp_s_temp_dn8 = assign42100_e55184_d_n8;
        locals.var_sp_s_temp_dn9 = assign42100_e55184_d_n9;

        let (assign42110_e55223, assign42110_e55223_d_n4, assign42110_e55223_d_n6, assign42110_e55223_d_n7, assign42110_e55223_d_n8, assign42110_e55223_d_n9,) = {
    if (((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) && (locals.var_guard1202 == 0.0)) {
        let assign42110_e55196: f64 = (-230.25850929940458);
        let assign42110_e55198: f64 = (-locals.var_sp_s_xbar);
        let assign42110_e55199: f64 = (assign42110_e55196 - assign42110_e55198);
        let assign42110_e55203: f64 = (-230.25850929940458);
        let assign42110_e55205: f64 = (-locals.var_sp_s_xbar);
        let assign42110_e55206: f64 = (assign42110_e55203 - assign42110_e55205);
        let assign42110_e55209: f64 = (-230.25850929940458);
        let assign42110_e55211: f64 = (-locals.var_sp_s_xbar);
        let assign42110_e55212: f64 = (assign42110_e55209 - assign42110_e55211);
        let assign42110_e55214: f64 = (assign42110_e55212 * 0.3333333333333333);
        let assign42110_e55215: f64 = (1.0 + assign42110_e55214);
        let assign42110_e55216: f64 = (assign42110_e55206 * assign42110_e55215);
        let assign42110_e55217: f64 = (0.5 * assign42110_e55216);
        let assign42110_e55218: f64 = (1.0 + assign42110_e55217);
        let assign42110_e55219: f64 = (assign42110_e55199 * assign42110_e55218);
        let assign42110_e55220: f64 = (1.0 + assign42110_e55219);
        let assign42110_e55221: f64 = (1e-100 / assign42110_e55220);
        (assign42110_e55221, (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn4)) * assign42110_e55218) + (assign42110_e55199 * (0.5 * (((-(-locals.var_sp_s_xbar_dn4)) * assign42110_e55215) + (assign42110_e55206 * ((-(-locals.var_sp_s_xbar_dn4)) * 0.3333333333333333))))))) / (assign42110_e55220 * assign42110_e55220))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn6)) * assign42110_e55218) + (assign42110_e55199 * (0.5 * (((-(-locals.var_sp_s_xbar_dn6)) * assign42110_e55215) + (assign42110_e55206 * ((-(-locals.var_sp_s_xbar_dn6)) * 0.3333333333333333))))))) / (assign42110_e55220 * assign42110_e55220))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn7)) * assign42110_e55218) + (assign42110_e55199 * (0.5 * (((-(-locals.var_sp_s_xbar_dn7)) * assign42110_e55215) + (assign42110_e55206 * ((-(-locals.var_sp_s_xbar_dn7)) * 0.3333333333333333))))))) / (assign42110_e55220 * assign42110_e55220))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn8)) * assign42110_e55218) + (assign42110_e55199 * (0.5 * (((-(-locals.var_sp_s_xbar_dn8)) * assign42110_e55215) + (assign42110_e55206 * ((-(-locals.var_sp_s_xbar_dn8)) * 0.3333333333333333))))))) / (assign42110_e55220 * assign42110_e55220))), (-((1e-100 * (((-(-locals.var_sp_s_xbar_dn9)) * assign42110_e55218) + (assign42110_e55199 * (0.5 * (((-(-locals.var_sp_s_xbar_dn9)) * assign42110_e55215) + (assign42110_e55206 * ((-(-locals.var_sp_s_xbar_dn9)) * 0.3333333333333333))))))) / (assign42110_e55220 * assign42110_e55220))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign42110_e55223;
        locals.var_sp_s_temp_dn4 = assign42110_e55223_d_n4;
        locals.var_sp_s_temp_dn6 = assign42110_e55223_d_n6;
        locals.var_sp_s_temp_dn7 = assign42110_e55223_d_n7;
        locals.var_sp_s_temp_dn8 = assign42110_e55223_d_n8;
        locals.var_sp_s_temp_dn9 = assign42110_e55223_d_n9;

        let (assign42120_e55233, assign42120_e55233_d_n4, assign42120_e55233_d_n6, assign42120_e55233_d_n7, assign42120_e55233_d_n8, assign42120_e55233_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42120_e55231: f64 = (1.0 - locals.var_sp_s_temp);
        (assign42120_e55231, (-locals.var_sp_s_temp_dn4), (-locals.var_sp_s_temp_dn6), (-locals.var_sp_s_temp_dn7), (-locals.var_sp_s_temp_dn8), (-locals.var_sp_s_temp_dn9),)
    } else {
        (locals.var_sp_s_w, locals.var_sp_s_w_dn4, locals.var_sp_s_w_dn6, locals.var_sp_s_w_dn7, locals.var_sp_s_w_dn8, locals.var_sp_s_w_dn9,)
    }
};
        locals.var_sp_s_w = assign42120_e55233;
        locals.var_sp_s_w_dn4 = assign42120_e55233_d_n4;
        locals.var_sp_s_w_dn6 = assign42120_e55233_d_n6;
        locals.var_sp_s_w_dn7 = assign42120_e55233_d_n7;
        locals.var_sp_s_w_dn8 = assign42120_e55233_d_n8;
        locals.var_sp_s_w_dn9 = assign42120_e55233_d_n9;

        let (assign42130_e55256, assign42130_e55256_d_n4, assign42130_e55256_d_n6, assign42130_e55256_d_n7, assign42130_e55256_d_n8, assign42130_e55256_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42130_e55242: f64 = (locals.var_gf2 * 0.5);
        let assign42130_e55243: f64 = (locals.var_xg + assign42130_e55242);
        let assign42130_e55248: f64 = (locals.var_gf2 * 0.25);
        let assign42130_e55249: f64 = (locals.var_xg + assign42130_e55248);
        let assign42130_e55251: f64 = (assign42130_e55249 - locals.var_sp_s_w);
        let assign42130_e55252: f64 = (assign42130_e55251).sqrt();
        let assign42130_e55253: f64 = (locals.var_gf * assign42130_e55252);
        let assign42130_e55254: f64 = (assign42130_e55243 - assign42130_e55253);
        (assign42130_e55254, ((locals.var_xg_dn4 + (locals.var_gf2_dn4 * 0.5)) - ((locals.var_gf_dn4 * assign42130_e55252) + (locals.var_gf * (((locals.var_xg_dn4 + (locals.var_gf2_dn4 * 0.25)) - locals.var_sp_s_w_dn4) / (2.0 * assign42130_e55252))))), ((locals.var_xg_dn6 + (locals.var_gf2_dn6 * 0.5)) - ((locals.var_gf_dn6 * assign42130_e55252) + (locals.var_gf * (((locals.var_xg_dn6 + (locals.var_gf2_dn6 * 0.25)) - locals.var_sp_s_w_dn6) / (2.0 * assign42130_e55252))))), ((locals.var_xg_dn7 + (locals.var_gf2_dn7 * 0.5)) - ((locals.var_gf_dn7 * assign42130_e55252) + (locals.var_gf * (((locals.var_xg_dn7 + (locals.var_gf2_dn7 * 0.25)) - locals.var_sp_s_w_dn7) / (2.0 * assign42130_e55252))))), ((locals.var_xg_dn8 + (locals.var_gf2_dn8 * 0.5)) - ((locals.var_gf_dn8 * assign42130_e55252) + (locals.var_gf * (((locals.var_xg_dn8 + (locals.var_gf2_dn8 * 0.25)) - locals.var_sp_s_w_dn8) / (2.0 * assign42130_e55252))))), ((locals.var_xg_dn9 + (locals.var_gf2_dn9 * 0.5)) - ((locals.var_gf_dn9 * assign42130_e55252) + (locals.var_gf * (((locals.var_xg_dn9 + (locals.var_gf2_dn9 * 0.25)) - locals.var_sp_s_w_dn9) / (2.0 * assign42130_e55252))))),)
    } else {
        (locals.var_sp_s_x1, locals.var_sp_s_x1_dn4, locals.var_sp_s_x1_dn6, locals.var_sp_s_x1_dn7, locals.var_sp_s_x1_dn8, locals.var_sp_s_x1_dn9,)
    }
};
        locals.var_sp_s_x1 = assign42130_e55256;
        locals.var_sp_s_x1_dn4 = assign42130_e55256_d_n4;
        locals.var_sp_s_x1_dn6 = assign42130_e55256_d_n6;
        locals.var_sp_s_x1_dn7 = assign42130_e55256_d_n7;
        locals.var_sp_s_x1_dn8 = assign42130_e55256_d_n8;
        locals.var_sp_s_x1_dn9 = assign42130_e55256_d_n9;

        let (assign42140_e55266, assign42140_e55266_d_n4, assign42140_e55266_d_n6, assign42140_e55266_d_n7, assign42140_e55266_d_n8, assign42140_e55266_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42140_e55264: f64 = (locals.var_xn_s + 3.0);
        (assign42140_e55264, locals.var_xn_s_dn4, locals.var_xn_s_dn6, locals.var_xn_s_dn7, locals.var_xn_s_dn8, locals.var_xn_s_dn9,)
    } else {
        (locals.var_sp_s_bx, locals.var_sp_s_bx_dn4, locals.var_sp_s_bx_dn6, locals.var_sp_s_bx_dn7, locals.var_sp_s_bx_dn8, locals.var_sp_s_bx_dn9,)
    }
};
        locals.var_sp_s_bx = assign42140_e55266;
        locals.var_sp_s_bx_dn4 = assign42140_e55266_d_n4;
        locals.var_sp_s_bx_dn6 = assign42140_e55266_d_n6;
        locals.var_sp_s_bx_dn7 = assign42140_e55266_d_n7;
        locals.var_sp_s_bx_dn8 = assign42140_e55266_d_n8;
        locals.var_sp_s_bx_dn9 = assign42140_e55266_d_n9;

        let (assign42150_e55300, assign42150_e55300_d_n4, assign42150_e55300_d_n6, assign42150_e55300_d_n7, assign42150_e55300_d_n8, assign42150_e55300_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42150_e55275: f64 = (locals.var_sp_s_x1 + locals.var_sp_s_bx);
        let assign42150_e55278: f64 = (locals.var_sp_s_x1 - locals.var_sp_s_bx);
        let assign42150_e55281: f64 = (locals.var_sp_s_x1 - locals.var_sp_s_bx);
        let assign42150_e55282: f64 = (assign42150_e55278 * assign42150_e55281);
        let assign42150_e55284: f64 = (assign42150_e55282 + 5.0);
        let assign42150_e55285: f64 = (assign42150_e55284).sqrt();
        let assign42150_e55286: f64 = (assign42150_e55275 - assign42150_e55285);
        let assign42150_e55287: f64 = (0.5 * assign42150_e55286);
        let assign42150_e55292: f64 = (locals.var_sp_s_bx * locals.var_sp_s_bx);
        let assign42150_e55294: f64 = (assign42150_e55292 + 5.0);
        let assign42150_e55295: f64 = (assign42150_e55294).sqrt();
        let assign42150_e55296: f64 = (locals.var_sp_s_bx - assign42150_e55295);
        let assign42150_e55297: f64 = (0.5 * assign42150_e55296);
        let assign42150_e55298: f64 = (assign42150_e55287 - assign42150_e55297);
        (assign42150_e55298, ((0.5 * ((locals.var_sp_s_x1_dn4 + locals.var_sp_s_bx_dn4) - ((((locals.var_sp_s_x1_dn4 - locals.var_sp_s_bx_dn4) * assign42150_e55281) + (assign42150_e55278 * (locals.var_sp_s_x1_dn4 - locals.var_sp_s_bx_dn4))) / (2.0 * assign42150_e55285)))) - (0.5 * (locals.var_sp_s_bx_dn4 - (((locals.var_sp_s_bx_dn4 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn4)) / (2.0 * assign42150_e55295))))), ((0.5 * ((locals.var_sp_s_x1_dn6 + locals.var_sp_s_bx_dn6) - ((((locals.var_sp_s_x1_dn6 - locals.var_sp_s_bx_dn6) * assign42150_e55281) + (assign42150_e55278 * (locals.var_sp_s_x1_dn6 - locals.var_sp_s_bx_dn6))) / (2.0 * assign42150_e55285)))) - (0.5 * (locals.var_sp_s_bx_dn6 - (((locals.var_sp_s_bx_dn6 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn6)) / (2.0 * assign42150_e55295))))), ((0.5 * ((locals.var_sp_s_x1_dn7 + locals.var_sp_s_bx_dn7) - ((((locals.var_sp_s_x1_dn7 - locals.var_sp_s_bx_dn7) * assign42150_e55281) + (assign42150_e55278 * (locals.var_sp_s_x1_dn7 - locals.var_sp_s_bx_dn7))) / (2.0 * assign42150_e55285)))) - (0.5 * (locals.var_sp_s_bx_dn7 - (((locals.var_sp_s_bx_dn7 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn7)) / (2.0 * assign42150_e55295))))), ((0.5 * ((locals.var_sp_s_x1_dn8 + locals.var_sp_s_bx_dn8) - ((((locals.var_sp_s_x1_dn8 - locals.var_sp_s_bx_dn8) * assign42150_e55281) + (assign42150_e55278 * (locals.var_sp_s_x1_dn8 - locals.var_sp_s_bx_dn8))) / (2.0 * assign42150_e55285)))) - (0.5 * (locals.var_sp_s_bx_dn8 - (((locals.var_sp_s_bx_dn8 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn8)) / (2.0 * assign42150_e55295))))), ((0.5 * ((locals.var_sp_s_x1_dn9 + locals.var_sp_s_bx_dn9) - ((((locals.var_sp_s_x1_dn9 - locals.var_sp_s_bx_dn9) * assign42150_e55281) + (assign42150_e55278 * (locals.var_sp_s_x1_dn9 - locals.var_sp_s_bx_dn9))) / (2.0 * assign42150_e55285)))) - (0.5 * (locals.var_sp_s_bx_dn9 - (((locals.var_sp_s_bx_dn9 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn9)) / (2.0 * assign42150_e55295))))),)
    } else {
        (locals.var_sp_s_eta, locals.var_sp_s_eta_dn4, locals.var_sp_s_eta_dn6, locals.var_sp_s_eta_dn7, locals.var_sp_s_eta_dn8, locals.var_sp_s_eta_dn9,)
    }
};
        locals.var_sp_s_eta = assign42150_e55300;
        locals.var_sp_s_eta_dn4 = assign42150_e55300_d_n4;
        locals.var_sp_s_eta_dn6 = assign42150_e55300_d_n6;
        locals.var_sp_s_eta_dn7 = assign42150_e55300_d_n7;
        locals.var_sp_s_eta_dn8 = assign42150_e55300_d_n8;
        locals.var_sp_s_eta_dn9 = assign42150_e55300_d_n9;

        let (assign42160_e55310, assign42160_e55310_d_n4, assign42160_e55310_d_n6, assign42160_e55310_d_n7, assign42160_e55310_d_n8, assign42160_e55310_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42160_e55308: f64 = (locals.var_xg - locals.var_sp_s_eta);
        (assign42160_e55308, (locals.var_xg_dn4 - locals.var_sp_s_eta_dn4), (locals.var_xg_dn6 - locals.var_sp_s_eta_dn6), (locals.var_xg_dn7 - locals.var_sp_s_eta_dn7), (locals.var_xg_dn8 - locals.var_sp_s_eta_dn8), (locals.var_xg_dn9 - locals.var_sp_s_eta_dn9),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign42160_e55310;
        locals.var_sp_s_temp_dn4 = assign42160_e55310_d_n4;
        locals.var_sp_s_temp_dn6 = assign42160_e55310_d_n6;
        locals.var_sp_s_temp_dn7 = assign42160_e55310_d_n7;
        locals.var_sp_s_temp_dn8 = assign42160_e55310_d_n8;
        locals.var_sp_s_temp_dn9 = assign42160_e55310_d_n9;

        let (assign42170_e55320, assign42170_e55320_d_n4, assign42170_e55320_d_n6, assign42170_e55320_d_n7, assign42170_e55320_d_n8, assign42170_e55320_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42170_e55317: f64 = (-locals.var_sp_s_eta);
        let assign42170_e55318: f64 = (assign42170_e55317).exp();
        (assign42170_e55318, (assign42170_e55318 * (-locals.var_sp_s_eta_dn4)), (assign42170_e55318 * (-locals.var_sp_s_eta_dn6)), (assign42170_e55318 * (-locals.var_sp_s_eta_dn7)), (assign42170_e55318 * (-locals.var_sp_s_eta_dn8)), (assign42170_e55318 * (-locals.var_sp_s_eta_dn9)),)
    } else {
        (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn4, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8, locals.var_sp_s_temp1_dn9,)
    }
};
        locals.var_sp_s_temp1 = assign42170_e55320;
        locals.var_sp_s_temp1_dn4 = assign42170_e55320_d_n4;
        locals.var_sp_s_temp1_dn6 = assign42170_e55320_d_n6;
        locals.var_sp_s_temp1_dn7 = assign42170_e55320_d_n7;
        locals.var_sp_s_temp1_dn8 = assign42170_e55320_d_n8;
        locals.var_sp_s_temp1_dn9 = assign42170_e55320_d_n9;

        let (assign42180_e55334, assign42180_e55334_d_n4, assign42180_e55334_d_n6, assign42180_e55334_d_n7, assign42180_e55334_d_n8, assign42180_e55334_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42180_e55330: f64 = (locals.var_sp_s_eta * locals.var_sp_s_eta);
        let assign42180_e55331: f64 = (2.0 + assign42180_e55330);
        let assign42180_e55332: f64 = (1.0 / assign42180_e55331);
        (assign42180_e55332, (-(((locals.var_sp_s_eta_dn4 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn4)) / (assign42180_e55331 * assign42180_e55331))), (-(((locals.var_sp_s_eta_dn6 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn6)) / (assign42180_e55331 * assign42180_e55331))), (-(((locals.var_sp_s_eta_dn7 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn7)) / (assign42180_e55331 * assign42180_e55331))), (-(((locals.var_sp_s_eta_dn8 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn8)) / (assign42180_e55331 * assign42180_e55331))), (-(((locals.var_sp_s_eta_dn9 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn9)) / (assign42180_e55331 * assign42180_e55331))),)
    } else {
        (locals.var_sp_s_temp2, locals.var_sp_s_temp2_dn4, locals.var_sp_s_temp2_dn6, locals.var_sp_s_temp2_dn7, locals.var_sp_s_temp2_dn8, locals.var_sp_s_temp2_dn9,)
    }
};
        locals.var_sp_s_temp2 = assign42180_e55334;
        locals.var_sp_s_temp2_dn4 = assign42180_e55334_d_n4;
        locals.var_sp_s_temp2_dn6 = assign42180_e55334_d_n6;
        locals.var_sp_s_temp2_dn7 = assign42180_e55334_d_n7;
        locals.var_sp_s_temp2_dn8 = assign42180_e55334_d_n8;
        locals.var_sp_s_temp2_dn9 = assign42180_e55334_d_n9;

        let (assign42190_e55346, assign42190_e55346_d_n4, assign42190_e55346_d_n6, assign42190_e55346_d_n7, assign42190_e55346_d_n8, assign42190_e55346_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42190_e55342: f64 = (locals.var_sp_s_eta * locals.var_sp_s_eta);
        let assign42190_e55344: f64 = (assign42190_e55342 * locals.var_sp_s_temp2);
        (assign42190_e55344, ((((locals.var_sp_s_eta_dn4 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn4)) * locals.var_sp_s_temp2) + (assign42190_e55342 * locals.var_sp_s_temp2_dn4)), ((((locals.var_sp_s_eta_dn6 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn6)) * locals.var_sp_s_temp2) + (assign42190_e55342 * locals.var_sp_s_temp2_dn6)), ((((locals.var_sp_s_eta_dn7 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn7)) * locals.var_sp_s_temp2) + (assign42190_e55342 * locals.var_sp_s_temp2_dn7)), ((((locals.var_sp_s_eta_dn8 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn8)) * locals.var_sp_s_temp2) + (assign42190_e55342 * locals.var_sp_s_temp2_dn8)), ((((locals.var_sp_s_eta_dn9 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn9)) * locals.var_sp_s_temp2) + (assign42190_e55342 * locals.var_sp_s_temp2_dn9)),)
    } else {
        (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn4, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8, locals.var_sp_s_xi0_dn9,)
    }
};
        locals.var_sp_s_xi0 = assign42190_e55346;
        locals.var_sp_s_xi0_dn4 = assign42190_e55346_d_n4;
        locals.var_sp_s_xi0_dn6 = assign42190_e55346_d_n6;
        locals.var_sp_s_xi0_dn7 = assign42190_e55346_d_n7;
        locals.var_sp_s_xi0_dn8 = assign42190_e55346_d_n8;
        locals.var_sp_s_xi0_dn9 = assign42190_e55346_d_n9;

        let (assign42200_e55360, assign42200_e55360_d_n4, assign42200_e55360_d_n6, assign42200_e55360_d_n7, assign42200_e55360_d_n8, assign42200_e55360_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42200_e55355: f64 = (locals.var_sp_s_eta * locals.var_sp_s_temp2);
        let assign42200_e55357: f64 = (assign42200_e55355 * locals.var_sp_s_temp2);
        let assign42200_e55358: f64 = (4.0 * assign42200_e55357);
        (assign42200_e55358, (4.0 * ((((locals.var_sp_s_eta_dn4 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn4)) * locals.var_sp_s_temp2) + (assign42200_e55355 * locals.var_sp_s_temp2_dn4))), (4.0 * ((((locals.var_sp_s_eta_dn6 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn6)) * locals.var_sp_s_temp2) + (assign42200_e55355 * locals.var_sp_s_temp2_dn6))), (4.0 * ((((locals.var_sp_s_eta_dn7 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn7)) * locals.var_sp_s_temp2) + (assign42200_e55355 * locals.var_sp_s_temp2_dn7))), (4.0 * ((((locals.var_sp_s_eta_dn8 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn8)) * locals.var_sp_s_temp2) + (assign42200_e55355 * locals.var_sp_s_temp2_dn8))), (4.0 * ((((locals.var_sp_s_eta_dn9 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn9)) * locals.var_sp_s_temp2) + (assign42200_e55355 * locals.var_sp_s_temp2_dn9))),)
    } else {
        (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn4, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8, locals.var_sp_s_xi1_dn9,)
    }
};
        locals.var_sp_s_xi1 = assign42200_e55360;
        locals.var_sp_s_xi1_dn4 = assign42200_e55360_d_n4;
        locals.var_sp_s_xi1_dn6 = assign42200_e55360_d_n6;
        locals.var_sp_s_xi1_dn7 = assign42200_e55360_d_n7;
        locals.var_sp_s_xi1_dn8 = assign42200_e55360_d_n8;
        locals.var_sp_s_xi1_dn9 = assign42200_e55360_d_n9;

        let (assign42210_e55378, assign42210_e55378_d_n4, assign42210_e55378_d_n6, assign42210_e55378_d_n7, assign42210_e55378_d_n8, assign42210_e55378_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42210_e55368: f64 = (8.0 * locals.var_sp_s_temp2);
        let assign42210_e55371: f64 = (12.0 * locals.var_sp_s_xi0);
        let assign42210_e55372: f64 = (assign42210_e55368 - assign42210_e55371);
        let assign42210_e55374: f64 = (assign42210_e55372 * locals.var_sp_s_temp2);
        let assign42210_e55376: f64 = (assign42210_e55374 * locals.var_sp_s_temp2);
        (assign42210_e55376, ((((((8.0 * locals.var_sp_s_temp2_dn4) - (12.0 * locals.var_sp_s_xi0_dn4)) * locals.var_sp_s_temp2) + (assign42210_e55372 * locals.var_sp_s_temp2_dn4)) * locals.var_sp_s_temp2) + (assign42210_e55374 * locals.var_sp_s_temp2_dn4)), ((((((8.0 * locals.var_sp_s_temp2_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp2) + (assign42210_e55372 * locals.var_sp_s_temp2_dn6)) * locals.var_sp_s_temp2) + (assign42210_e55374 * locals.var_sp_s_temp2_dn6)), ((((((8.0 * locals.var_sp_s_temp2_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp2) + (assign42210_e55372 * locals.var_sp_s_temp2_dn7)) * locals.var_sp_s_temp2) + (assign42210_e55374 * locals.var_sp_s_temp2_dn7)), ((((((8.0 * locals.var_sp_s_temp2_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp2) + (assign42210_e55372 * locals.var_sp_s_temp2_dn8)) * locals.var_sp_s_temp2) + (assign42210_e55374 * locals.var_sp_s_temp2_dn8)), ((((((8.0 * locals.var_sp_s_temp2_dn9) - (12.0 * locals.var_sp_s_xi0_dn9)) * locals.var_sp_s_temp2) + (assign42210_e55372 * locals.var_sp_s_temp2_dn9)) * locals.var_sp_s_temp2) + (assign42210_e55374 * locals.var_sp_s_temp2_dn9)),)
    } else {
        (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn4, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8, locals.var_sp_s_xi2_dn9,)
    }
};
        locals.var_sp_s_xi2 = assign42210_e55378;
        locals.var_sp_s_xi2_dn4 = assign42210_e55378_d_n4;
        locals.var_sp_s_xi2_dn6 = assign42210_e55378_d_n6;
        locals.var_sp_s_xi2_dn7 = assign42210_e55378_d_n7;
        locals.var_sp_s_xi2_dn8 = assign42210_e55378_d_n8;
        locals.var_sp_s_xi2_dn9 = assign42210_e55378_d_n9;

        let (assign42220_e55427, assign42220_e55427_d_n4, assign42220_e55427_d_n6, assign42220_e55427_d_n7, assign42220_e55427_d_n8, assign42220_e55427_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42220_e55387: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign42220_e55391: f64 = (locals.var_sp_s_temp1 + locals.var_sp_s_eta);
        let assign42220_e55393: f64 = (assign42220_e55391 - 1.0);
        let assign42220_e55397: f64 = (locals.var_sp_s_eta + 1.0);
        let assign42220_e55399: f64 = (assign42220_e55397 + locals.var_sp_s_xi0);
        let assign42220_e55400: f64 = (locals.var_delta_ns * assign42220_e55399);
        let assign42220_e55401: f64 = (assign42220_e55393 - assign42220_e55400);
        let assign42220_e55402: f64 = (locals.var_gf2 * assign42220_e55401);
        let assign42220_e55403: f64 = (assign42220_e55387 - assign42220_e55402);
        let (assign42220_e55425, assign42220_e55425_d_n4, assign42220_e55425_d_n6, assign42220_e55425_d_n7, assign42220_e55425_d_n8, assign42220_e55425_d_n9,) = {
            if (1e-40 > assign42220_e55403) {
                (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign42220_e55408: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
                let assign42220_e55412: f64 = (locals.var_sp_s_temp1 + locals.var_sp_s_eta);
                let assign42220_e55414: f64 = (assign42220_e55412 - 1.0);
                let assign42220_e55418: f64 = (locals.var_sp_s_eta + 1.0);
                let assign42220_e55420: f64 = (assign42220_e55418 + locals.var_sp_s_xi0);
                let assign42220_e55421: f64 = (locals.var_delta_ns * assign42220_e55420);
                let assign42220_e55422: f64 = (assign42220_e55414 - assign42220_e55421);
                let assign42220_e55423: f64 = (locals.var_gf2 * assign42220_e55422);
                let assign42220_e55424: f64 = (assign42220_e55408 - assign42220_e55423);
                (assign42220_e55424, (((locals.var_sp_s_temp_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn4)) - ((locals.var_gf2_dn4 * assign42220_e55422) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn4 + locals.var_sp_s_eta_dn4) - ((locals.var_delta_ns_dn4 * assign42220_e55420) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn4 + locals.var_sp_s_xi0_dn4))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign42220_e55422) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn6 + locals.var_sp_s_eta_dn6) - ((locals.var_delta_ns_dn6 * assign42220_e55420) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn6 + locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign42220_e55422) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn7 + locals.var_sp_s_eta_dn7) - ((locals.var_delta_ns_dn7 * assign42220_e55420) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn7 + locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign42220_e55422) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn8 + locals.var_sp_s_eta_dn8) - ((locals.var_delta_ns_dn8 * assign42220_e55420) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn8 + locals.var_sp_s_xi0_dn8))))))), (((locals.var_sp_s_temp_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn9)) - ((locals.var_gf2_dn9 * assign42220_e55422) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn9 + locals.var_sp_s_eta_dn9) - ((locals.var_delta_ns_dn9 * assign42220_e55420) + (locals.var_delta_ns * (locals.var_sp_s_eta_dn9 + locals.var_sp_s_xi0_dn9))))))),)
            }
        };
        (assign42220_e55425, assign42220_e55425_d_n4, assign42220_e55425_d_n6, assign42220_e55425_d_n7, assign42220_e55425_d_n8, assign42220_e55425_d_n9,)
    } else {
        (locals.var_sp_s_a, locals.var_sp_s_a_dn4, locals.var_sp_s_a_dn6, locals.var_sp_s_a_dn7, locals.var_sp_s_a_dn8, locals.var_sp_s_a_dn9,)
    }
};
        locals.var_sp_s_a = assign42220_e55427;
        locals.var_sp_s_a_dn4 = assign42220_e55427_d_n4;
        locals.var_sp_s_a_dn6 = assign42220_e55427_d_n6;
        locals.var_sp_s_a_dn7 = assign42220_e55427_d_n7;
        locals.var_sp_s_a_dn8 = assign42220_e55427_d_n8;
        locals.var_sp_s_a_dn9 = assign42220_e55427_d_n9;

        let (assign42230_e55445, assign42230_e55445_d_n4, assign42230_e55445_d_n6, assign42230_e55445_d_n7, assign42230_e55445_d_n8, assign42230_e55445_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42230_e55439: f64 = (locals.var_delta_ns * locals.var_sp_s_xi2);
        let assign42230_e55440: f64 = (locals.var_sp_s_temp1 - assign42230_e55439);
        let assign42230_e55441: f64 = (locals.var_gf2 * assign42230_e55440);
        let assign42230_e55442: f64 = (0.5 * assign42230_e55441);
        let assign42230_e55443: f64 = (1.0 - assign42230_e55442);
        (assign42230_e55443, (-(0.5 * ((locals.var_gf2_dn4 * assign42230_e55440) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn4 - ((locals.var_delta_ns_dn4 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn4))))))), (-(0.5 * ((locals.var_gf2_dn6 * assign42230_e55440) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn6 - ((locals.var_delta_ns_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn6))))))), (-(0.5 * ((locals.var_gf2_dn7 * assign42230_e55440) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn7 - ((locals.var_delta_ns_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn7))))))), (-(0.5 * ((locals.var_gf2_dn8 * assign42230_e55440) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn8 - ((locals.var_delta_ns_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn8))))))), (-(0.5 * ((locals.var_gf2_dn9 * assign42230_e55440) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn9 - ((locals.var_delta_ns_dn9 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn9))))))),)
    } else {
        (locals.var_sp_s_b, locals.var_sp_s_b_dn4, locals.var_sp_s_b_dn6, locals.var_sp_s_b_dn7, locals.var_sp_s_b_dn8, locals.var_sp_s_b_dn9,)
    }
};
        locals.var_sp_s_b = assign42230_e55445;
        locals.var_sp_s_b_dn4 = assign42230_e55445_d_n4;
        locals.var_sp_s_b_dn6 = assign42230_e55445_d_n6;
        locals.var_sp_s_b_dn7 = assign42230_e55445_d_n7;
        locals.var_sp_s_b_dn8 = assign42230_e55445_d_n8;
        locals.var_sp_s_b_dn9 = assign42230_e55445_d_n9;

        let (assign42240_e55467, assign42240_e55467_d_n4, assign42240_e55467_d_n6, assign42240_e55467_d_n7, assign42240_e55467_d_n8, assign42240_e55467_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42240_e55453: f64 = (2.0 * locals.var_sp_s_temp);
        let assign42240_e55457: f64 = (1.0 - locals.var_sp_s_temp1);
        let assign42240_e55461: f64 = (1.0 + locals.var_sp_s_xi1);
        let assign42240_e55462: f64 = (locals.var_delta_ns * assign42240_e55461);
        let assign42240_e55463: f64 = (assign42240_e55457 - assign42240_e55462);
        let assign42240_e55464: f64 = (locals.var_gf2 * assign42240_e55463);
        let assign42240_e55465: f64 = (assign42240_e55453 + assign42240_e55464);
        (assign42240_e55465, ((2.0 * locals.var_sp_s_temp_dn4) + ((locals.var_gf2_dn4 * assign42240_e55463) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn4) - ((locals.var_delta_ns_dn4 * assign42240_e55461) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn4)))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign42240_e55463) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn6) - ((locals.var_delta_ns_dn6 * assign42240_e55461) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn6)))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign42240_e55463) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn7) - ((locals.var_delta_ns_dn7 * assign42240_e55461) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn7)))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign42240_e55463) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn8) - ((locals.var_delta_ns_dn8 * assign42240_e55461) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn8)))))), ((2.0 * locals.var_sp_s_temp_dn9) + ((locals.var_gf2_dn9 * assign42240_e55463) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn9) - ((locals.var_delta_ns_dn9 * assign42240_e55461) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn9)))))),)
    } else {
        (locals.var_sp_s_c, locals.var_sp_s_c_dn4, locals.var_sp_s_c_dn6, locals.var_sp_s_c_dn7, locals.var_sp_s_c_dn8, locals.var_sp_s_c_dn9,)
    }
};
        locals.var_sp_s_c = assign42240_e55467;
        locals.var_sp_s_c_dn4 = assign42240_e55467_d_n4;
        locals.var_sp_s_c_dn6 = assign42240_e55467_d_n6;
        locals.var_sp_s_c_dn7 = assign42240_e55467_d_n7;
        locals.var_sp_s_c_dn8 = assign42240_e55467_d_n8;
        locals.var_sp_s_c_dn9 = assign42240_e55467_d_n9;

        let (assign42250_e55482, assign42250_e55482_d_n4, assign42250_e55482_d_n6, assign42250_e55482_d_n7, assign42250_e55482_d_n8, assign42250_e55482_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42250_e55475: f64 = (locals.var_xn_s - locals.var_sp_s_eta);
        let assign42250_e55478: f64 = (locals.var_sp_s_a / locals.var_gf2);
        let assign42250_e55479: f64 = (assign42250_e55478).ln();
        let assign42250_e55480: f64 = (assign42250_e55475 + assign42250_e55479);
        (assign42250_e55480, ((locals.var_xn_s_dn4 - locals.var_sp_s_eta_dn4) + ((((locals.var_sp_s_a_dn4 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn4)) / (locals.var_gf2 * locals.var_gf2)) / assign42250_e55478)), ((locals.var_xn_s_dn6 - locals.var_sp_s_eta_dn6) + ((((locals.var_sp_s_a_dn6 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn6)) / (locals.var_gf2 * locals.var_gf2)) / assign42250_e55478)), ((locals.var_xn_s_dn7 - locals.var_sp_s_eta_dn7) + ((((locals.var_sp_s_a_dn7 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn7)) / (locals.var_gf2 * locals.var_gf2)) / assign42250_e55478)), ((locals.var_xn_s_dn8 - locals.var_sp_s_eta_dn8) + ((((locals.var_sp_s_a_dn8 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn8)) / (locals.var_gf2 * locals.var_gf2)) / assign42250_e55478)), ((locals.var_xn_s_dn9 - locals.var_sp_s_eta_dn9) + ((((locals.var_sp_s_a_dn9 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn9)) / (locals.var_gf2 * locals.var_gf2)) / assign42250_e55478)),)
    } else {
        (locals.var_sp_s_tau, locals.var_sp_s_tau_dn4, locals.var_sp_s_tau_dn6, locals.var_sp_s_tau_dn7, locals.var_sp_s_tau_dn8, locals.var_sp_s_tau_dn9,)
    }
};
        locals.var_sp_s_tau = assign42250_e55482;
        locals.var_sp_s_tau_dn4 = assign42250_e55482_d_n4;
        locals.var_sp_s_tau_dn6 = assign42250_e55482_d_n6;
        locals.var_sp_s_tau_dn7 = assign42250_e55482_d_n7;
        locals.var_sp_s_tau_dn8 = assign42250_e55482_d_n8;
        locals.var_sp_s_tau_dn9 = assign42250_e55482_d_n9;

        let (assign42260_e55492, assign42260_e55492_d_n4, assign42260_e55492_d_n6, assign42260_e55492_d_n7, assign42260_e55492_d_n8, assign42260_e55492_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42260_e55490: f64 = (locals.var_sp_s_a + locals.var_sp_s_c);
        (assign42260_e55490, (locals.var_sp_s_a_dn4 + locals.var_sp_s_c_dn4), (locals.var_sp_s_a_dn6 + locals.var_sp_s_c_dn6), (locals.var_sp_s_a_dn7 + locals.var_sp_s_c_dn7), (locals.var_sp_s_a_dn8 + locals.var_sp_s_c_dn8), (locals.var_sp_s_a_dn9 + locals.var_sp_s_c_dn9),)
    } else {
        (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9,)
    }
};
        locals.var_nu = assign42260_e55492;
        locals.var_nu_dn4 = assign42260_e55492_d_n4;
        locals.var_nu_dn6 = assign42260_e55492_d_n6;
        locals.var_nu_dn7 = assign42260_e55492_d_n7;
        locals.var_nu_dn8 = assign42260_e55492_d_n8;
        locals.var_nu_dn9 = assign42260_e55492_d_n9;

        let (assign42270_e55514, assign42270_e55514_d_n4, assign42270_e55514_d_n6, assign42270_e55514_d_n7, assign42270_e55514_d_n8, assign42270_e55514_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42270_e55500: f64 = (locals.var_nu * locals.var_nu);
        let assign42270_e55505: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign42270_e55506: f64 = (0.5 * assign42270_e55505);
        let assign42270_e55509: f64 = (locals.var_sp_s_a * locals.var_sp_s_b);
        let assign42270_e55510: f64 = (assign42270_e55506 - assign42270_e55509);
        let assign42270_e55511: f64 = (locals.var_sp_s_tau * assign42270_e55510);
        let assign42270_e55512: f64 = (assign42270_e55500 + assign42270_e55511);
        (assign42270_e55512, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_sp_s_tau_dn4 * assign42270_e55510) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn4 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn4))) - ((locals.var_sp_s_a_dn4 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn4)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau_dn6 * assign42270_e55510) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6))) - ((locals.var_sp_s_a_dn6 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau_dn7 * assign42270_e55510) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7))) - ((locals.var_sp_s_a_dn7 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau_dn8 * assign42270_e55510) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8))) - ((locals.var_sp_s_a_dn8 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn8)))))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_sp_s_tau_dn9 * assign42270_e55510) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn9 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn9))) - ((locals.var_sp_s_a_dn9 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn9)))))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9,)
    }
};
        locals.var_mutau = assign42270_e55514;
        locals.var_mutau_dn4 = assign42270_e55514_d_n4;
        locals.var_mutau_dn6 = assign42270_e55514_d_n6;
        locals.var_mutau_dn7 = assign42270_e55514_d_n7;
        locals.var_mutau_dn8 = assign42270_e55514_d_n8;
        locals.var_mutau_dn9 = assign42270_e55514_d_n9;

        let (assign42280_e55550, assign42280_e55550_d_n4, assign42280_e55550_d_n6, assign42280_e55550_d_n7, assign42280_e55550_d_n8, assign42280_e55550_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42280_e55523: f64 = (locals.var_sp_s_a * locals.var_nu);
        let assign42280_e55525: f64 = (assign42280_e55523 * locals.var_sp_s_tau);
        let assign42280_e55529: f64 = (locals.var_nu / locals.var_mutau);
        let assign42280_e55531: f64 = (assign42280_e55529 * locals.var_sp_s_tau);
        let assign42280_e55533: f64 = (assign42280_e55531 * locals.var_sp_s_tau);
        let assign42280_e55535: f64 = (assign42280_e55533 * locals.var_sp_s_c);
        let assign42280_e55538: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign42280_e55540: f64 = (assign42280_e55538 * 0.3333333333333333);
        let assign42280_e55543: f64 = (locals.var_sp_s_a * locals.var_sp_s_b);
        let assign42280_e55544: f64 = (assign42280_e55540 - assign42280_e55543);
        let assign42280_e55545: f64 = (assign42280_e55535 * assign42280_e55544);
        let assign42280_e55546: f64 = (locals.var_mutau + assign42280_e55545);
        let assign42280_e55547: f64 = (assign42280_e55525 / assign42280_e55546);
        let assign42280_e55548: f64 = (locals.var_sp_s_eta + assign42280_e55547);
        (assign42280_e55548, (locals.var_sp_s_eta_dn4 + (((((((locals.var_sp_s_a_dn4 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn4)) * locals.var_sp_s_tau) + (assign42280_e55523 * locals.var_sp_s_tau_dn4)) * assign42280_e55546) - (assign42280_e55525 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42280_e55529 * locals.var_sp_s_tau_dn4)) * locals.var_sp_s_tau) + (assign42280_e55531 * locals.var_sp_s_tau_dn4)) * locals.var_sp_s_c) + (assign42280_e55533 * locals.var_sp_s_c_dn4)) * assign42280_e55544) + (assign42280_e55535 * ((((locals.var_sp_s_c_dn4 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn4)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn4 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn4)))))))) / (assign42280_e55546 * assign42280_e55546))), (locals.var_sp_s_eta_dn6 + (((((((locals.var_sp_s_a_dn6 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn6)) * locals.var_sp_s_tau) + (assign42280_e55523 * locals.var_sp_s_tau_dn6)) * assign42280_e55546) - (assign42280_e55525 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42280_e55529 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_tau) + (assign42280_e55531 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_c) + (assign42280_e55533 * locals.var_sp_s_c_dn6)) * assign42280_e55544) + (assign42280_e55535 * ((((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn6 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn6)))))))) / (assign42280_e55546 * assign42280_e55546))), (locals.var_sp_s_eta_dn7 + (((((((locals.var_sp_s_a_dn7 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn7)) * locals.var_sp_s_tau) + (assign42280_e55523 * locals.var_sp_s_tau_dn7)) * assign42280_e55546) - (assign42280_e55525 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42280_e55529 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_tau) + (assign42280_e55531 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_c) + (assign42280_e55533 * locals.var_sp_s_c_dn7)) * assign42280_e55544) + (assign42280_e55535 * ((((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn7 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn7)))))))) / (assign42280_e55546 * assign42280_e55546))), (locals.var_sp_s_eta_dn8 + (((((((locals.var_sp_s_a_dn8 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn8)) * locals.var_sp_s_tau) + (assign42280_e55523 * locals.var_sp_s_tau_dn8)) * assign42280_e55546) - (assign42280_e55525 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42280_e55529 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_tau) + (assign42280_e55531 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_c) + (assign42280_e55533 * locals.var_sp_s_c_dn8)) * assign42280_e55544) + (assign42280_e55535 * ((((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn8 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn8)))))))) / (assign42280_e55546 * assign42280_e55546))), (locals.var_sp_s_eta_dn9 + (((((((locals.var_sp_s_a_dn9 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn9)) * locals.var_sp_s_tau) + (assign42280_e55523 * locals.var_sp_s_tau_dn9)) * assign42280_e55546) - (assign42280_e55525 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign42280_e55529 * locals.var_sp_s_tau_dn9)) * locals.var_sp_s_tau) + (assign42280_e55531 * locals.var_sp_s_tau_dn9)) * locals.var_sp_s_c) + (assign42280_e55533 * locals.var_sp_s_c_dn9)) * assign42280_e55544) + (assign42280_e55535 * ((((locals.var_sp_s_c_dn9 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn9)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn9 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn9)))))))) / (assign42280_e55546 * assign42280_e55546))),)
    } else {
        (locals.var_sp_s_x0, locals.var_sp_s_x0_dn4, locals.var_sp_s_x0_dn6, locals.var_sp_s_x0_dn7, locals.var_sp_s_x0_dn8, locals.var_sp_s_x0_dn9,)
    }
};
        locals.var_sp_s_x0 = assign42280_e55550;
        locals.var_sp_s_x0_dn4 = assign42280_e55550_d_n4;
        locals.var_sp_s_x0_dn6 = assign42280_e55550_d_n6;
        locals.var_sp_s_x0_dn7 = assign42280_e55550_d_n7;
        locals.var_sp_s_x0_dn8 = assign42280_e55550_d_n8;
        locals.var_sp_s_x0_dn9 = assign42280_e55550_d_n9;

        let assign42290_e55553: f64 = if locals.var_sp_s_x0 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1203 = assign42290_e55553;

        let (assign42300_e55564, assign42300_e55564_d_n4, assign42300_e55564_d_n6, assign42300_e55564_d_n7, assign42300_e55564_d_n8, assign42300_e55564_d_n9,) = {
    if (((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) && (locals.var_guard1203 != 0.0)) {
        let assign42300_e55562: f64 = (locals.var_sp_s_x0).exp();
        (assign42300_e55562, (assign42300_e55562 * locals.var_sp_s_x0_dn4), (assign42300_e55562 * locals.var_sp_s_x0_dn6), (assign42300_e55562 * locals.var_sp_s_x0_dn7), (assign42300_e55562 * locals.var_sp_s_x0_dn8), (assign42300_e55562 * locals.var_sp_s_x0_dn9),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9,)
    }
};
        locals.var_sp_s_delta0 = assign42300_e55564;
        locals.var_sp_s_delta0_dn4 = assign42300_e55564_d_n4;
        locals.var_sp_s_delta0_dn6 = assign42300_e55564_d_n6;
        locals.var_sp_s_delta0_dn7 = assign42300_e55564_d_n7;
        locals.var_sp_s_delta0_dn8 = assign42300_e55564_d_n8;
        locals.var_sp_s_delta0_dn9 = assign42300_e55564_d_n9;

        let (assign42310_e55576, assign42310_e55576_d_n4, assign42310_e55576_d_n6, assign42310_e55576_d_n7, assign42310_e55576_d_n8, assign42310_e55576_d_n9,) = {
    if (((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) && (locals.var_guard1203 != 0.0)) {
        let assign42310_e55574: f64 = (1.0 / locals.var_sp_s_delta0);
        (assign42310_e55574, (-(locals.var_sp_s_delta0_dn4 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn6 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn7 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn8 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn9 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn4, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, locals.var_sp_s_delta1_dn9,)
    }
};
        locals.var_sp_s_delta1 = assign42310_e55576;
        locals.var_sp_s_delta1_dn4 = assign42310_e55576_d_n4;
        locals.var_sp_s_delta1_dn6 = assign42310_e55576_d_n6;
        locals.var_sp_s_delta1_dn7 = assign42310_e55576_d_n7;
        locals.var_sp_s_delta1_dn8 = assign42310_e55576_d_n8;
        locals.var_sp_s_delta1_dn9 = assign42310_e55576_d_n9;

        let (assign42320_e55588, assign42320_e55588_d_n4, assign42320_e55588_d_n6, assign42320_e55588_d_n7, assign42320_e55588_d_n8, assign42320_e55588_d_n9,) = {
    if (((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) && (locals.var_guard1203 != 0.0)) {
        let assign42320_e55586: f64 = (locals.var_delta_ns * locals.var_sp_s_delta0);
        (assign42320_e55586, ((locals.var_delta_ns_dn4 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn4)), ((locals.var_delta_ns_dn6 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn6)), ((locals.var_delta_ns_dn7 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn7)), ((locals.var_delta_ns_dn8 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn8)), ((locals.var_delta_ns_dn9 * locals.var_sp_s_delta0) + (locals.var_delta_ns * locals.var_sp_s_delta0_dn9)),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9,)
    }
};
        locals.var_sp_s_delta0 = assign42320_e55588;
        locals.var_sp_s_delta0_dn4 = assign42320_e55588_d_n4;
        locals.var_sp_s_delta0_dn6 = assign42320_e55588_d_n6;
        locals.var_sp_s_delta0_dn7 = assign42320_e55588_d_n7;
        locals.var_sp_s_delta0_dn8 = assign42320_e55588_d_n8;
        locals.var_sp_s_delta0_dn9 = assign42320_e55588_d_n9;

        let assign42330_e55592: f64 = (locals.var_xn_s - 230.25850929940458);
        let assign42330_e55593: f64 = if locals.var_sp_s_x0 > assign42330_e55592 { 1.0 } else { 0.0 };
        locals.var_guard1204 = assign42330_e55593;

        let (assign42340_e55609, assign42340_e55609_d_n4, assign42340_e55609_d_n6, assign42340_e55609_d_n7, assign42340_e55609_d_n8, assign42340_e55609_d_n9,) = {
    if ((((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) && (locals.var_guard1203 == 0.0)) && (locals.var_guard1204 != 0.0)) {
        let assign42340_e55606: f64 = (locals.var_sp_s_x0 - locals.var_xn_s);
        let assign42340_e55607: f64 = (assign42340_e55606).exp();
        (assign42340_e55607, (assign42340_e55607 * (locals.var_sp_s_x0_dn4 - locals.var_xn_s_dn4)), (assign42340_e55607 * (locals.var_sp_s_x0_dn6 - locals.var_xn_s_dn6)), (assign42340_e55607 * (locals.var_sp_s_x0_dn7 - locals.var_xn_s_dn7)), (assign42340_e55607 * (locals.var_sp_s_x0_dn8 - locals.var_xn_s_dn8)), (assign42340_e55607 * (locals.var_sp_s_x0_dn9 - locals.var_xn_s_dn9)),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9,)
    }
};
        locals.var_sp_s_delta0 = assign42340_e55609;
        locals.var_sp_s_delta0_dn4 = assign42340_e55609_d_n4;
        locals.var_sp_s_delta0_dn6 = assign42340_e55609_d_n6;
        locals.var_sp_s_delta0_dn7 = assign42340_e55609_d_n7;
        locals.var_sp_s_delta0_dn8 = assign42340_e55609_d_n8;
        locals.var_sp_s_delta0_dn9 = assign42340_e55609_d_n9;

        let (assign42350_e55624, assign42350_e55624_d_n4, assign42350_e55624_d_n6, assign42350_e55624_d_n7, assign42350_e55624_d_n8, assign42350_e55624_d_n9,) = {
    if ((((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) && (locals.var_guard1203 == 0.0)) && (locals.var_guard1204 != 0.0)) {
        let assign42350_e55622: f64 = (locals.var_delta_ns / locals.var_sp_s_delta0);
        (assign42350_e55622, (((locals.var_delta_ns_dn4 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn4)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn6 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn6)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn7 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn7)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn8 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn8)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_ns_dn9 * locals.var_sp_s_delta0) - (locals.var_delta_ns * locals.var_sp_s_delta0_dn9)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn4, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, locals.var_sp_s_delta1_dn9,)
    }
};
        locals.var_sp_s_delta1 = assign42350_e55624;
        locals.var_sp_s_delta1_dn4 = assign42350_e55624_d_n4;
        locals.var_sp_s_delta1_dn6 = assign42350_e55624_d_n6;
        locals.var_sp_s_delta1_dn7 = assign42350_e55624_d_n7;
        locals.var_sp_s_delta1_dn8 = assign42350_e55624_d_n8;
        locals.var_sp_s_delta1_dn9 = assign42350_e55624_d_n9;

    }

    pub(super) fn stamp_transient_block_19(
        locals: &mut StampLocals,
    ) {
        let (assign42360_e55666, assign42360_e55666_d_n4, assign42360_e55666_d_n6, assign42360_e55666_d_n7, assign42360_e55666_d_n8, assign42360_e55666_d_n9,) = {
    if ((((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) && (locals.var_guard1203 == 0.0)) && (locals.var_guard1204 == 0.0)) {
        let assign42360_e55640: f64 = (locals.var_xn_s - locals.var_sp_s_x0);
        let assign42360_e55642: f64 = (assign42360_e55640 - 230.25850929940458);
        let assign42360_e55647: f64 = (locals.var_xn_s - locals.var_sp_s_x0);
        let assign42360_e55649: f64 = (assign42360_e55647 - 230.25850929940458);
        let assign42360_e55653: f64 = (locals.var_xn_s - locals.var_sp_s_x0);
        let assign42360_e55655: f64 = (assign42360_e55653 - 230.25850929940458);
        let assign42360_e55657: f64 = (assign42360_e55655 * 0.3333333333333333);
        let assign42360_e55658: f64 = (1.0 + assign42360_e55657);
        let assign42360_e55659: f64 = (assign42360_e55649 * assign42360_e55658);
        let assign42360_e55660: f64 = (0.5 * assign42360_e55659);
        let assign42360_e55661: f64 = (1.0 + assign42360_e55660);
        let assign42360_e55662: f64 = (assign42360_e55642 * assign42360_e55661);
        let assign42360_e55663: f64 = (1.0 + assign42360_e55662);
        let assign42360_e55664: f64 = (1e-100 / assign42360_e55663);
        (assign42360_e55664, (-((1e-100 * (((locals.var_xn_s_dn4 - locals.var_sp_s_x0_dn4) * assign42360_e55661) + (assign42360_e55642 * (0.5 * (((locals.var_xn_s_dn4 - locals.var_sp_s_x0_dn4) * assign42360_e55658) + (assign42360_e55649 * ((locals.var_xn_s_dn4 - locals.var_sp_s_x0_dn4) * 0.3333333333333333))))))) / (assign42360_e55663 * assign42360_e55663))), (-((1e-100 * (((locals.var_xn_s_dn6 - locals.var_sp_s_x0_dn6) * assign42360_e55661) + (assign42360_e55642 * (0.5 * (((locals.var_xn_s_dn6 - locals.var_sp_s_x0_dn6) * assign42360_e55658) + (assign42360_e55649 * ((locals.var_xn_s_dn6 - locals.var_sp_s_x0_dn6) * 0.3333333333333333))))))) / (assign42360_e55663 * assign42360_e55663))), (-((1e-100 * (((locals.var_xn_s_dn7 - locals.var_sp_s_x0_dn7) * assign42360_e55661) + (assign42360_e55642 * (0.5 * (((locals.var_xn_s_dn7 - locals.var_sp_s_x0_dn7) * assign42360_e55658) + (assign42360_e55649 * ((locals.var_xn_s_dn7 - locals.var_sp_s_x0_dn7) * 0.3333333333333333))))))) / (assign42360_e55663 * assign42360_e55663))), (-((1e-100 * (((locals.var_xn_s_dn8 - locals.var_sp_s_x0_dn8) * assign42360_e55661) + (assign42360_e55642 * (0.5 * (((locals.var_xn_s_dn8 - locals.var_sp_s_x0_dn8) * assign42360_e55658) + (assign42360_e55649 * ((locals.var_xn_s_dn8 - locals.var_sp_s_x0_dn8) * 0.3333333333333333))))))) / (assign42360_e55663 * assign42360_e55663))), (-((1e-100 * (((locals.var_xn_s_dn9 - locals.var_sp_s_x0_dn9) * assign42360_e55661) + (assign42360_e55642 * (0.5 * (((locals.var_xn_s_dn9 - locals.var_sp_s_x0_dn9) * assign42360_e55658) + (assign42360_e55649 * ((locals.var_xn_s_dn9 - locals.var_sp_s_x0_dn9) * 0.3333333333333333))))))) / (assign42360_e55663 * assign42360_e55663))),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9,)
    }
};
        locals.var_sp_s_delta0 = assign42360_e55666;
        locals.var_sp_s_delta0_dn4 = assign42360_e55666_d_n4;
        locals.var_sp_s_delta0_dn6 = assign42360_e55666_d_n6;
        locals.var_sp_s_delta0_dn7 = assign42360_e55666_d_n7;
        locals.var_sp_s_delta0_dn8 = assign42360_e55666_d_n8;
        locals.var_sp_s_delta0_dn9 = assign42360_e55666_d_n9;

        let (assign42370_e55702, assign42370_e55702_d_n4, assign42370_e55702_d_n6, assign42370_e55702_d_n7, assign42370_e55702_d_n8, assign42370_e55702_d_n9,) = {
    if ((((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) && (locals.var_guard1203 == 0.0)) && (locals.var_guard1204 == 0.0)) {
        let assign42370_e55682: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
        let assign42370_e55687: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
        let assign42370_e55691: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
        let assign42370_e55693: f64 = (assign42370_e55691 * 0.3333333333333333);
        let assign42370_e55694: f64 = (1.0 + assign42370_e55693);
        let assign42370_e55695: f64 = (assign42370_e55687 * assign42370_e55694);
        let assign42370_e55696: f64 = (0.5 * assign42370_e55695);
        let assign42370_e55697: f64 = (1.0 + assign42370_e55696);
        let assign42370_e55698: f64 = (assign42370_e55682 * assign42370_e55697);
        let assign42370_e55699: f64 = (1.0 + assign42370_e55698);
        let assign42370_e55700: f64 = (1e-100 / assign42370_e55699);
        (assign42370_e55700, (-((1e-100 * ((locals.var_sp_s_x0_dn4 * assign42370_e55697) + (assign42370_e55682 * (0.5 * ((locals.var_sp_s_x0_dn4 * assign42370_e55694) + (assign42370_e55687 * (locals.var_sp_s_x0_dn4 * 0.3333333333333333))))))) / (assign42370_e55699 * assign42370_e55699))), (-((1e-100 * ((locals.var_sp_s_x0_dn6 * assign42370_e55697) + (assign42370_e55682 * (0.5 * ((locals.var_sp_s_x0_dn6 * assign42370_e55694) + (assign42370_e55687 * (locals.var_sp_s_x0_dn6 * 0.3333333333333333))))))) / (assign42370_e55699 * assign42370_e55699))), (-((1e-100 * ((locals.var_sp_s_x0_dn7 * assign42370_e55697) + (assign42370_e55682 * (0.5 * ((locals.var_sp_s_x0_dn7 * assign42370_e55694) + (assign42370_e55687 * (locals.var_sp_s_x0_dn7 * 0.3333333333333333))))))) / (assign42370_e55699 * assign42370_e55699))), (-((1e-100 * ((locals.var_sp_s_x0_dn8 * assign42370_e55697) + (assign42370_e55682 * (0.5 * ((locals.var_sp_s_x0_dn8 * assign42370_e55694) + (assign42370_e55687 * (locals.var_sp_s_x0_dn8 * 0.3333333333333333))))))) / (assign42370_e55699 * assign42370_e55699))), (-((1e-100 * ((locals.var_sp_s_x0_dn9 * assign42370_e55697) + (assign42370_e55682 * (0.5 * ((locals.var_sp_s_x0_dn9 * assign42370_e55694) + (assign42370_e55687 * (locals.var_sp_s_x0_dn9 * 0.3333333333333333))))))) / (assign42370_e55699 * assign42370_e55699))),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn4, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, locals.var_sp_s_delta1_dn9,)
    }
};
        locals.var_sp_s_delta1 = assign42370_e55702;
        locals.var_sp_s_delta1_dn4 = assign42370_e55702_d_n4;
        locals.var_sp_s_delta1_dn6 = assign42370_e55702_d_n6;
        locals.var_sp_s_delta1_dn7 = assign42370_e55702_d_n7;
        locals.var_sp_s_delta1_dn8 = assign42370_e55702_d_n8;
        locals.var_sp_s_delta1_dn9 = assign42370_e55702_d_n9;

        let (assign42380_e55716, assign42380_e55716_d_n4, assign42380_e55716_d_n6, assign42380_e55716_d_n7, assign42380_e55716_d_n8, assign42380_e55716_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42380_e55712: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_x0);
        let assign42380_e55713: f64 = (2.0 + assign42380_e55712);
        let assign42380_e55714: f64 = (1.0 / assign42380_e55713);
        (assign42380_e55714, (-(((locals.var_sp_s_x0_dn4 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn4)) / (assign42380_e55713 * assign42380_e55713))), (-(((locals.var_sp_s_x0_dn6 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn6)) / (assign42380_e55713 * assign42380_e55713))), (-(((locals.var_sp_s_x0_dn7 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn7)) / (assign42380_e55713 * assign42380_e55713))), (-(((locals.var_sp_s_x0_dn8 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn8)) / (assign42380_e55713 * assign42380_e55713))), (-(((locals.var_sp_s_x0_dn9 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn9)) / (assign42380_e55713 * assign42380_e55713))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign42380_e55716;
        locals.var_sp_s_temp_dn4 = assign42380_e55716_d_n4;
        locals.var_sp_s_temp_dn6 = assign42380_e55716_d_n6;
        locals.var_sp_s_temp_dn7 = assign42380_e55716_d_n7;
        locals.var_sp_s_temp_dn8 = assign42380_e55716_d_n8;
        locals.var_sp_s_temp_dn9 = assign42380_e55716_d_n9;

        let (assign42390_e55728, assign42390_e55728_d_n4, assign42390_e55728_d_n6, assign42390_e55728_d_n7, assign42390_e55728_d_n8, assign42390_e55728_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42390_e55724: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_x0);
        let assign42390_e55726: f64 = (assign42390_e55724 * locals.var_sp_s_temp);
        (assign42390_e55726, ((((locals.var_sp_s_x0_dn4 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn4)) * locals.var_sp_s_temp) + (assign42390_e55724 * locals.var_sp_s_temp_dn4)), ((((locals.var_sp_s_x0_dn6 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn6)) * locals.var_sp_s_temp) + (assign42390_e55724 * locals.var_sp_s_temp_dn6)), ((((locals.var_sp_s_x0_dn7 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn7)) * locals.var_sp_s_temp) + (assign42390_e55724 * locals.var_sp_s_temp_dn7)), ((((locals.var_sp_s_x0_dn8 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn8)) * locals.var_sp_s_temp) + (assign42390_e55724 * locals.var_sp_s_temp_dn8)), ((((locals.var_sp_s_x0_dn9 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn9)) * locals.var_sp_s_temp) + (assign42390_e55724 * locals.var_sp_s_temp_dn9)),)
    } else {
        (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn4, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8, locals.var_sp_s_xi0_dn9,)
    }
};
        locals.var_sp_s_xi0 = assign42390_e55728;
        locals.var_sp_s_xi0_dn4 = assign42390_e55728_d_n4;
        locals.var_sp_s_xi0_dn6 = assign42390_e55728_d_n6;
        locals.var_sp_s_xi0_dn7 = assign42390_e55728_d_n7;
        locals.var_sp_s_xi0_dn8 = assign42390_e55728_d_n8;
        locals.var_sp_s_xi0_dn9 = assign42390_e55728_d_n9;

        let (assign42400_e55742, assign42400_e55742_d_n4, assign42400_e55742_d_n6, assign42400_e55742_d_n7, assign42400_e55742_d_n8, assign42400_e55742_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42400_e55737: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_temp);
        let assign42400_e55739: f64 = (assign42400_e55737 * locals.var_sp_s_temp);
        let assign42400_e55740: f64 = (4.0 * assign42400_e55739);
        (assign42400_e55740, (4.0 * ((((locals.var_sp_s_x0_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn4)) * locals.var_sp_s_temp) + (assign42400_e55737 * locals.var_sp_s_temp_dn4))), (4.0 * ((((locals.var_sp_s_x0_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign42400_e55737 * locals.var_sp_s_temp_dn6))), (4.0 * ((((locals.var_sp_s_x0_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign42400_e55737 * locals.var_sp_s_temp_dn7))), (4.0 * ((((locals.var_sp_s_x0_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign42400_e55737 * locals.var_sp_s_temp_dn8))), (4.0 * ((((locals.var_sp_s_x0_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn9)) * locals.var_sp_s_temp) + (assign42400_e55737 * locals.var_sp_s_temp_dn9))),)
    } else {
        (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn4, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8, locals.var_sp_s_xi1_dn9,)
    }
};
        locals.var_sp_s_xi1 = assign42400_e55742;
        locals.var_sp_s_xi1_dn4 = assign42400_e55742_d_n4;
        locals.var_sp_s_xi1_dn6 = assign42400_e55742_d_n6;
        locals.var_sp_s_xi1_dn7 = assign42400_e55742_d_n7;
        locals.var_sp_s_xi1_dn8 = assign42400_e55742_d_n8;
        locals.var_sp_s_xi1_dn9 = assign42400_e55742_d_n9;

        let (assign42410_e55760, assign42410_e55760_d_n4, assign42410_e55760_d_n6, assign42410_e55760_d_n7, assign42410_e55760_d_n8, assign42410_e55760_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42410_e55750: f64 = (8.0 * locals.var_sp_s_temp);
        let assign42410_e55753: f64 = (12.0 * locals.var_sp_s_xi0);
        let assign42410_e55754: f64 = (assign42410_e55750 - assign42410_e55753);
        let assign42410_e55756: f64 = (assign42410_e55754 * locals.var_sp_s_temp);
        let assign42410_e55758: f64 = (assign42410_e55756 * locals.var_sp_s_temp);
        (assign42410_e55758, ((((((8.0 * locals.var_sp_s_temp_dn4) - (12.0 * locals.var_sp_s_xi0_dn4)) * locals.var_sp_s_temp) + (assign42410_e55754 * locals.var_sp_s_temp_dn4)) * locals.var_sp_s_temp) + (assign42410_e55756 * locals.var_sp_s_temp_dn4)), ((((((8.0 * locals.var_sp_s_temp_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp) + (assign42410_e55754 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign42410_e55756 * locals.var_sp_s_temp_dn6)), ((((((8.0 * locals.var_sp_s_temp_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp) + (assign42410_e55754 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign42410_e55756 * locals.var_sp_s_temp_dn7)), ((((((8.0 * locals.var_sp_s_temp_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp) + (assign42410_e55754 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign42410_e55756 * locals.var_sp_s_temp_dn8)), ((((((8.0 * locals.var_sp_s_temp_dn9) - (12.0 * locals.var_sp_s_xi0_dn9)) * locals.var_sp_s_temp) + (assign42410_e55754 * locals.var_sp_s_temp_dn9)) * locals.var_sp_s_temp) + (assign42410_e55756 * locals.var_sp_s_temp_dn9)),)
    } else {
        (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn4, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8, locals.var_sp_s_xi2_dn9,)
    }
};
        locals.var_sp_s_xi2 = assign42410_e55760;
        locals.var_sp_s_xi2_dn4 = assign42410_e55760_d_n4;
        locals.var_sp_s_xi2_dn6 = assign42410_e55760_d_n6;
        locals.var_sp_s_xi2_dn7 = assign42410_e55760_d_n7;
        locals.var_sp_s_xi2_dn8 = assign42410_e55760_d_n8;
        locals.var_sp_s_xi2_dn9 = assign42410_e55760_d_n9;

        let (assign42420_e55770, assign42420_e55770_d_n4, assign42420_e55770_d_n6, assign42420_e55770_d_n7, assign42420_e55770_d_n8, assign42420_e55770_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42420_e55768: f64 = (locals.var_xg - locals.var_sp_s_x0);
        (assign42420_e55768, (locals.var_xg_dn4 - locals.var_sp_s_x0_dn4), (locals.var_xg_dn6 - locals.var_sp_s_x0_dn6), (locals.var_xg_dn7 - locals.var_sp_s_x0_dn7), (locals.var_xg_dn8 - locals.var_sp_s_x0_dn8), (locals.var_xg_dn9 - locals.var_sp_s_x0_dn9),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign42420_e55770;
        locals.var_sp_s_temp_dn4 = assign42420_e55770_d_n4;
        locals.var_sp_s_temp_dn6 = assign42420_e55770_d_n6;
        locals.var_sp_s_temp_dn7 = assign42420_e55770_d_n7;
        locals.var_sp_s_temp_dn8 = assign42420_e55770_d_n8;
        locals.var_sp_s_temp_dn9 = assign42420_e55770_d_n9;

        let (assign42430_e55794, assign42430_e55794_d_n4, assign42430_e55794_d_n6, assign42430_e55794_d_n7, assign42430_e55794_d_n8, assign42430_e55794_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42430_e55778: f64 = (2.0 * locals.var_sp_s_temp);
        let assign42430_e55782: f64 = (1.0 - locals.var_sp_s_delta1);
        let assign42430_e55784: f64 = (assign42430_e55782 + locals.var_sp_s_delta0);
        let assign42430_e55788: f64 = (1.0 + locals.var_sp_s_xi1);
        let assign42430_e55789: f64 = (locals.var_delta_ns * assign42430_e55788);
        let assign42430_e55790: f64 = (assign42430_e55784 - assign42430_e55789);
        let assign42430_e55791: f64 = (locals.var_gf2 * assign42430_e55790);
        let assign42430_e55792: f64 = (assign42430_e55778 + assign42430_e55791);
        (assign42430_e55792, ((2.0 * locals.var_sp_s_temp_dn4) + ((locals.var_gf2_dn4 * assign42430_e55790) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn4) + locals.var_sp_s_delta0_dn4) - ((locals.var_delta_ns_dn4 * assign42430_e55788) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn4)))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign42430_e55790) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn6) + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_ns_dn6 * assign42430_e55788) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn6)))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign42430_e55790) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn7) + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_ns_dn7 * assign42430_e55788) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn7)))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign42430_e55790) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn8) + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_ns_dn8 * assign42430_e55788) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn8)))))), ((2.0 * locals.var_sp_s_temp_dn9) + ((locals.var_gf2_dn9 * assign42430_e55790) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn9) + locals.var_sp_s_delta0_dn9) - ((locals.var_delta_ns_dn9 * assign42430_e55788) + (locals.var_delta_ns * locals.var_sp_s_xi1_dn9)))))),)
    } else {
        (locals.var_sp_s_pc, locals.var_sp_s_pc_dn4, locals.var_sp_s_pc_dn6, locals.var_sp_s_pc_dn7, locals.var_sp_s_pc_dn8, locals.var_sp_s_pc_dn9,)
    }
};
        locals.var_sp_s_pc = assign42430_e55794;
        locals.var_sp_s_pc_dn4 = assign42430_e55794_d_n4;
        locals.var_sp_s_pc_dn6 = assign42430_e55794_d_n6;
        locals.var_sp_s_pc_dn7 = assign42430_e55794_d_n7;
        locals.var_sp_s_pc_dn8 = assign42430_e55794_d_n8;
        locals.var_sp_s_pc_dn9 = assign42430_e55794_d_n9;

        let (assign42440_e55822, assign42440_e55822_d_n4, assign42440_e55822_d_n6, assign42440_e55822_d_n7, assign42440_e55822_d_n8, assign42440_e55822_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42440_e55802: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign42440_e55806: f64 = (locals.var_sp_s_delta1 + locals.var_sp_s_x0);
        let assign42440_e55808: f64 = (assign42440_e55806 - 1.0);
        let assign42440_e55810: f64 = (assign42440_e55808 + locals.var_sp_s_delta0);
        let assign42440_e55814: f64 = (locals.var_sp_s_x0 + 1.0);
        let assign42440_e55816: f64 = (assign42440_e55814 + locals.var_sp_s_xi0);
        let assign42440_e55817: f64 = (locals.var_delta_ns * assign42440_e55816);
        let assign42440_e55818: f64 = (assign42440_e55810 - assign42440_e55817);
        let assign42440_e55819: f64 = (locals.var_gf2 * assign42440_e55818);
        let assign42440_e55820: f64 = (assign42440_e55802 - assign42440_e55819);
        (assign42440_e55820, (((locals.var_sp_s_temp_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn4)) - ((locals.var_gf2_dn4 * assign42440_e55818) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn4 + locals.var_sp_s_x0_dn4) + locals.var_sp_s_delta0_dn4) - ((locals.var_delta_ns_dn4 * assign42440_e55816) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn4 + locals.var_sp_s_xi0_dn4))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign42440_e55818) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn6 + locals.var_sp_s_x0_dn6) + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_ns_dn6 * assign42440_e55816) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn6 + locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign42440_e55818) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn7 + locals.var_sp_s_x0_dn7) + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_ns_dn7 * assign42440_e55816) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn7 + locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign42440_e55818) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn8 + locals.var_sp_s_x0_dn8) + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_ns_dn8 * assign42440_e55816) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn8 + locals.var_sp_s_xi0_dn8))))))), (((locals.var_sp_s_temp_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn9)) - ((locals.var_gf2_dn9 * assign42440_e55818) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn9 + locals.var_sp_s_x0_dn9) + locals.var_sp_s_delta0_dn9) - ((locals.var_delta_ns_dn9 * assign42440_e55816) + (locals.var_delta_ns * (locals.var_sp_s_x0_dn9 + locals.var_sp_s_xi0_dn9))))))),)
    } else {
        (locals.var_sp_s_qc, locals.var_sp_s_qc_dn4, locals.var_sp_s_qc_dn6, locals.var_sp_s_qc_dn7, locals.var_sp_s_qc_dn8, locals.var_sp_s_qc_dn9,)
    }
};
        locals.var_sp_s_qc = assign42440_e55822;
        locals.var_sp_s_qc_dn4 = assign42440_e55822_d_n4;
        locals.var_sp_s_qc_dn6 = assign42440_e55822_d_n6;
        locals.var_sp_s_qc_dn7 = assign42440_e55822_d_n7;
        locals.var_sp_s_qc_dn8 = assign42440_e55822_d_n8;
        locals.var_sp_s_qc_dn9 = assign42440_e55822_d_n9;

        let (assign42450_e55840, assign42450_e55840_d_n4, assign42450_e55840_d_n6, assign42450_e55840_d_n7, assign42450_e55840_d_n8, assign42450_e55840_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42450_e55832: f64 = (locals.var_sp_s_delta1 + locals.var_sp_s_delta0);
        let assign42450_e55835: f64 = (locals.var_delta_ns * locals.var_sp_s_xi2);
        let assign42450_e55836: f64 = (assign42450_e55832 - assign42450_e55835);
        let assign42450_e55837: f64 = (locals.var_gf2 * assign42450_e55836);
        let assign42450_e55838: f64 = (2.0 - assign42450_e55837);
        (assign42450_e55838, (-((locals.var_gf2_dn4 * assign42450_e55836) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn4 + locals.var_sp_s_delta0_dn4) - ((locals.var_delta_ns_dn4 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn4)))))), (-((locals.var_gf2_dn6 * assign42450_e55836) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn6 + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_ns_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn6)))))), (-((locals.var_gf2_dn7 * assign42450_e55836) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn7 + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_ns_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn7)))))), (-((locals.var_gf2_dn8 * assign42450_e55836) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn8 + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_ns_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn8)))))), (-((locals.var_gf2_dn9 * assign42450_e55836) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn9 + locals.var_sp_s_delta0_dn9) - ((locals.var_delta_ns_dn9 * locals.var_sp_s_xi2) + (locals.var_delta_ns * locals.var_sp_s_xi2_dn9)))))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign42450_e55840;
        locals.var_sp_s_temp_dn4 = assign42450_e55840_d_n4;
        locals.var_sp_s_temp_dn6 = assign42450_e55840_d_n6;
        locals.var_sp_s_temp_dn7 = assign42450_e55840_d_n7;
        locals.var_sp_s_temp_dn8 = assign42450_e55840_d_n8;
        locals.var_sp_s_temp_dn9 = assign42450_e55840_d_n9;

        let (assign42460_e55856, assign42460_e55856_d_n4, assign42460_e55856_d_n6, assign42460_e55856_d_n7, assign42460_e55856_d_n8, assign42460_e55856_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42460_e55848: f64 = (locals.var_sp_s_pc * locals.var_sp_s_pc);
        let assign42460_e55852: f64 = (locals.var_sp_s_qc * locals.var_sp_s_temp);
        let assign42460_e55853: f64 = (2.0 * assign42460_e55852);
        let assign42460_e55854: f64 = (assign42460_e55848 - assign42460_e55853);
        (assign42460_e55854, (((locals.var_sp_s_pc_dn4 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn4)) - (2.0 * ((locals.var_sp_s_qc_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn4)))), (((locals.var_sp_s_pc_dn6 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn6)) - (2.0 * ((locals.var_sp_s_qc_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn6)))), (((locals.var_sp_s_pc_dn7 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn7)) - (2.0 * ((locals.var_sp_s_qc_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn7)))), (((locals.var_sp_s_pc_dn8 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn8)) - (2.0 * ((locals.var_sp_s_qc_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn8)))), (((locals.var_sp_s_pc_dn9 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn9)) - (2.0 * ((locals.var_sp_s_qc_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn9)))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign42460_e55856;
        locals.var_sp_s_temp_dn4 = assign42460_e55856_d_n4;
        locals.var_sp_s_temp_dn6 = assign42460_e55856_d_n6;
        locals.var_sp_s_temp_dn7 = assign42460_e55856_d_n7;
        locals.var_sp_s_temp_dn8 = assign42460_e55856_d_n8;
        locals.var_sp_s_temp_dn9 = assign42460_e55856_d_n9;

        let (assign42470_e55873, assign42470_e55873_d_n4, assign42470_e55873_d_n6, assign42470_e55873_d_n7, assign42470_e55873_d_n8, assign42470_e55873_d_n9,) = {
    if ((locals.var_guard1199 == 0.0) && (locals.var_guard1200 == 0.0)) {
        let assign42470_e55867: f64 = (locals.var_sp_s_temp).sqrt();
        let assign42470_e55868: f64 = (locals.var_sp_s_pc + assign42470_e55867);
        let assign42470_e55869: f64 = (locals.var_sp_s_qc / assign42470_e55868);
        let assign42470_e55870: f64 = (2.0 * assign42470_e55869);
        let assign42470_e55871: f64 = (locals.var_sp_s_x0 + assign42470_e55870);
        (assign42470_e55871, (locals.var_sp_s_x0_dn4 + (2.0 * (((locals.var_sp_s_qc_dn4 * assign42470_e55868) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn4 + (locals.var_sp_s_temp_dn4 / (2.0 * assign42470_e55867))))) / (assign42470_e55868 * assign42470_e55868)))), (locals.var_sp_s_x0_dn6 + (2.0 * (((locals.var_sp_s_qc_dn6 * assign42470_e55868) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn6 + (locals.var_sp_s_temp_dn6 / (2.0 * assign42470_e55867))))) / (assign42470_e55868 * assign42470_e55868)))), (locals.var_sp_s_x0_dn7 + (2.0 * (((locals.var_sp_s_qc_dn7 * assign42470_e55868) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn7 + (locals.var_sp_s_temp_dn7 / (2.0 * assign42470_e55867))))) / (assign42470_e55868 * assign42470_e55868)))), (locals.var_sp_s_x0_dn8 + (2.0 * (((locals.var_sp_s_qc_dn8 * assign42470_e55868) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn8 + (locals.var_sp_s_temp_dn8 / (2.0 * assign42470_e55867))))) / (assign42470_e55868 * assign42470_e55868)))), (locals.var_sp_s_x0_dn9 + (2.0 * (((locals.var_sp_s_qc_dn9 * assign42470_e55868) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn9 + (locals.var_sp_s_temp_dn9 / (2.0 * assign42470_e55867))))) / (assign42470_e55868 * assign42470_e55868)))),)
    } else {
        (locals.var_x_s, locals.var_x_s_dn4, locals.var_x_s_dn6, locals.var_x_s_dn7, locals.var_x_s_dn8, locals.var_x_s_dn9,)
    }
};
        locals.var_x_s = assign42470_e55873;
        locals.var_x_s_dn4 = assign42470_e55873_d_n4;
        locals.var_x_s_dn6 = assign42470_e55873_d_n6;
        locals.var_x_s_dn7 = assign42470_e55873_d_n7;
        locals.var_x_s_dn8 = assign42470_e55873_d_n8;
        locals.var_x_s_dn9 = assign42470_e55873_d_n9;

        locals.var_xi1s = 0.0;
        locals.var_xi1s_dn4 = 0.0;
        locals.var_xi1s_dn6 = 0.0;
        locals.var_xi1s_dn7 = 0.0;
        locals.var_xi1s_dn8 = 0.0;
        locals.var_xi1s_dn9 = 0.0;

        locals.var_xi2s = 0.0;
        locals.var_xi2s_dn4 = 0.0;
        locals.var_xi2s_dn6 = 0.0;
        locals.var_xi2s_dn7 = 0.0;
        locals.var_xi2s_dn8 = 0.0;
        locals.var_xi2s_dn9 = 0.0;

        locals.var_delta_1s = 0.0;
        locals.var_delta_1s_dn4 = 0.0;
        locals.var_delta_1s_dn6 = 0.0;
        locals.var_delta_1s_dn7 = 0.0;
        locals.var_delta_1s_dn8 = 0.0;
        locals.var_delta_1s_dn9 = 0.0;

        locals.var_es = 0.0;
        locals.var_es_dn4 = 0.0;
        locals.var_es_dn6 = 0.0;
        locals.var_es_dn7 = 0.0;
        locals.var_es_dn8 = 0.0;
        locals.var_es_dn9 = 0.0;

        locals.var_ds = 0.0;
        locals.var_ds_dn4 = 0.0;
        locals.var_ds_dn6 = 0.0;
        locals.var_ds_dn7 = 0.0;
        locals.var_ds_dn8 = 0.0;
        locals.var_ds_dn9 = 0.0;

        locals.var_ps = 0.0;
        locals.var_ps_dn4 = 0.0;
        locals.var_ps_dn6 = 0.0;
        locals.var_ps_dn7 = 0.0;
        locals.var_ps_dn8 = 0.0;
        locals.var_ps_dn9 = 0.0;

        locals.var_sqs = 0.0;
        locals.var_sqs_dn4 = 0.0;
        locals.var_sqs_dn6 = 0.0;
        locals.var_sqs_dn7 = 0.0;
        locals.var_sqs_dn8 = 0.0;
        locals.var_sqs_dn9 = 0.0;

        locals.var_alphas = 1.0;
        locals.var_alphas_dn4 = 0.0;
        locals.var_alphas_dn6 = 0.0;
        locals.var_alphas_dn7 = 0.0;
        locals.var_alphas_dn8 = 0.0;
        locals.var_alphas_dn9 = 0.0;

        locals.var_rxcor = 1.0;
        locals.var_rxcor_dn4 = 0.0;
        locals.var_rxcor_dn6 = 0.0;
        locals.var_rxcor_dn7 = 0.0;
        locals.var_rxcor_dn8 = 0.0;
        locals.var_rxcor_dn9 = 0.0;

        let assign42570_e55885: f64 = (locals.var_xg - locals.var_x_s);
        locals.var_xgs = assign42570_e55885;
        locals.var_xgs_dn4 = (locals.var_xg_dn4 - locals.var_x_s_dn4);
        locals.var_xgs_dn6 = (locals.var_xg_dn6 - locals.var_x_s_dn6);
        locals.var_xgs_dn7 = (locals.var_xg_dn7 - locals.var_x_s_dn7);
        locals.var_xgs_dn8 = (locals.var_xg_dn8 - locals.var_x_s_dn8);
        locals.var_xgs_dn9 = (locals.var_xg_dn9 - locals.var_x_s_dn9);

        locals.var_qis = 0.0;
        locals.var_qis_dn4 = 0.0;
        locals.var_qis_dn6 = 0.0;
        locals.var_qis_dn7 = 0.0;
        locals.var_qis_dn8 = 0.0;
        locals.var_qis_dn9 = 0.0;

        let assign42590_e55889: f64 = (locals.var_phit1 * locals.var_xgs);
        locals.var_qbs = assign42590_e55889;
        locals.var_qbs_dn4 = ((locals.var_phit1_dn4 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn4));
        locals.var_qbs_dn6 = ((locals.var_phit1_dn6 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn6));
        locals.var_qbs_dn7 = ((locals.var_phit1_dn7 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn7));
        locals.var_qbs_dn8 = ((locals.var_phit1_dn8 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn8));
        locals.var_qbs_dn9 = ((locals.var_phit1_dn9 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn9));

        locals.var_rhob = 1.0;
        locals.var_rhob_dn4 = 0.0;
        locals.var_rhob_dn6 = 0.0;
        locals.var_rhob_dn7 = 0.0;
        locals.var_rhob_dn8 = 0.0;
        locals.var_rhob_dn9 = 0.0;

        locals.var_rhog = 1.0;
        locals.var_rhog_dn4 = 0.0;
        locals.var_rhog_dn6 = 0.0;
        locals.var_rhog_dn7 = 0.0;
        locals.var_rhog_dn8 = 0.0;
        locals.var_rhog_dn9 = 0.0;

        locals.var_gmobs = 1.0;
        locals.var_gmobs_dn4 = 0.0;
        locals.var_gmobs_dn6 = 0.0;
        locals.var_gmobs_dn7 = 0.0;
        locals.var_gmobs_dn8 = 0.0;
        locals.var_gmobs_dn9 = 0.0;

        locals.var_xitsb = 1.0;
        locals.var_xitsb_dn4 = 0.0;
        locals.var_xitsb_dn6 = 0.0;
        locals.var_xitsb_dn7 = 0.0;
        locals.var_xitsb_dn8 = 0.0;
        locals.var_xitsb_dn9 = 0.0;

        locals.var_factheta = 1.0;
        locals.var_factheta_dn4 = 0.0;
        locals.var_factheta_dn6 = 0.0;
        locals.var_factheta_dn7 = 0.0;
        locals.var_factheta_dn8 = 0.0;
        locals.var_factheta_dn9 = 0.0;

        let assign42650_e55897: f64 = if locals.var_xg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1205 = assign42650_e55897;

        let (assign42660_e55907, assign42660_e55907_d_n4, assign42660_e55907_d_n6, assign42660_e55907_d_n7, assign42660_e55907_d_n8, assign42660_e55907_d_n9,) = {
    if (locals.var_guard1205 != 0.0) {
        let assign42660_e55903: f64 = (locals.var_x_s * locals.var_x_s);
        let assign42660_e55904: f64 = (2.0 + assign42660_e55903);
        let assign42660_e55905: f64 = (1.0 / assign42660_e55904);
        (assign42660_e55905, (-(((locals.var_x_s_dn4 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn4)) / (assign42660_e55904 * assign42660_e55904))), (-(((locals.var_x_s_dn6 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn6)) / (assign42660_e55904 * assign42660_e55904))), (-(((locals.var_x_s_dn7 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn7)) / (assign42660_e55904 * assign42660_e55904))), (-(((locals.var_x_s_dn8 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn8)) / (assign42660_e55904 * assign42660_e55904))), (-(((locals.var_x_s_dn9 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn9)) / (assign42660_e55904 * assign42660_e55904))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign42660_e55907;
        locals.var_temp__blk949_dn4 = assign42660_e55907_d_n4;
        locals.var_temp__blk949_dn6 = assign42660_e55907_d_n6;
        locals.var_temp__blk949_dn7 = assign42660_e55907_d_n7;
        locals.var_temp__blk949_dn8 = assign42660_e55907_d_n8;
        locals.var_temp__blk949_dn9 = assign42660_e55907_d_n9;

        let (assign42670_e55915, assign42670_e55915_d_n4, assign42670_e55915_d_n6, assign42670_e55915_d_n7, assign42670_e55915_d_n8, assign42670_e55915_d_n9,) = {
    if (locals.var_guard1205 != 0.0) {
        let assign42670_e55911: f64 = (locals.var_x_s * locals.var_x_s);
        let assign42670_e55913: f64 = (assign42670_e55911 * locals.var_temp__blk949);
        (assign42670_e55913, ((((locals.var_x_s_dn4 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn4)) * locals.var_temp__blk949) + (assign42670_e55911 * locals.var_temp__blk949_dn4)), ((((locals.var_x_s_dn6 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn6)) * locals.var_temp__blk949) + (assign42670_e55911 * locals.var_temp__blk949_dn6)), ((((locals.var_x_s_dn7 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn7)) * locals.var_temp__blk949) + (assign42670_e55911 * locals.var_temp__blk949_dn7)), ((((locals.var_x_s_dn8 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn8)) * locals.var_temp__blk949) + (assign42670_e55911 * locals.var_temp__blk949_dn8)), ((((locals.var_x_s_dn9 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn9)) * locals.var_temp__blk949) + (assign42670_e55911 * locals.var_temp__blk949_dn9)),)
    } else {
        (locals.var_xi0s, locals.var_xi0s_dn4, locals.var_xi0s_dn6, locals.var_xi0s_dn7, locals.var_xi0s_dn8, locals.var_xi0s_dn9,)
    }
};
        locals.var_xi0s = assign42670_e55915;
        locals.var_xi0s_dn4 = assign42670_e55915_d_n4;
        locals.var_xi0s_dn6 = assign42670_e55915_d_n6;
        locals.var_xi0s_dn7 = assign42670_e55915_d_n7;
        locals.var_xi0s_dn8 = assign42670_e55915_d_n8;
        locals.var_xi0s_dn9 = assign42670_e55915_d_n9;

        let (assign42680_e55925, assign42680_e55925_d_n4, assign42680_e55925_d_n6, assign42680_e55925_d_n7, assign42680_e55925_d_n8, assign42680_e55925_d_n9,) = {
    if (locals.var_guard1205 != 0.0) {
        let assign42680_e55920: f64 = (locals.var_x_s * locals.var_temp__blk949);
        let assign42680_e55922: f64 = (assign42680_e55920 * locals.var_temp__blk949);
        let assign42680_e55923: f64 = (4.0 * assign42680_e55922);
        (assign42680_e55923, (4.0 * ((((locals.var_x_s_dn4 * locals.var_temp__blk949) + (locals.var_x_s * locals.var_temp__blk949_dn4)) * locals.var_temp__blk949) + (assign42680_e55920 * locals.var_temp__blk949_dn4))), (4.0 * ((((locals.var_x_s_dn6 * locals.var_temp__blk949) + (locals.var_x_s * locals.var_temp__blk949_dn6)) * locals.var_temp__blk949) + (assign42680_e55920 * locals.var_temp__blk949_dn6))), (4.0 * ((((locals.var_x_s_dn7 * locals.var_temp__blk949) + (locals.var_x_s * locals.var_temp__blk949_dn7)) * locals.var_temp__blk949) + (assign42680_e55920 * locals.var_temp__blk949_dn7))), (4.0 * ((((locals.var_x_s_dn8 * locals.var_temp__blk949) + (locals.var_x_s * locals.var_temp__blk949_dn8)) * locals.var_temp__blk949) + (assign42680_e55920 * locals.var_temp__blk949_dn8))), (4.0 * ((((locals.var_x_s_dn9 * locals.var_temp__blk949) + (locals.var_x_s * locals.var_temp__blk949_dn9)) * locals.var_temp__blk949) + (assign42680_e55920 * locals.var_temp__blk949_dn9))),)
    } else {
        (locals.var_xi1s, locals.var_xi1s_dn4, locals.var_xi1s_dn6, locals.var_xi1s_dn7, locals.var_xi1s_dn8, locals.var_xi1s_dn9,)
    }
};
        locals.var_xi1s = assign42680_e55925;
        locals.var_xi1s_dn4 = assign42680_e55925_d_n4;
        locals.var_xi1s_dn6 = assign42680_e55925_d_n6;
        locals.var_xi1s_dn7 = assign42680_e55925_d_n7;
        locals.var_xi1s_dn8 = assign42680_e55925_d_n8;
        locals.var_xi1s_dn9 = assign42680_e55925_d_n9;

        let (assign42690_e55939, assign42690_e55939_d_n4, assign42690_e55939_d_n6, assign42690_e55939_d_n7, assign42690_e55939_d_n8, assign42690_e55939_d_n9,) = {
    if (locals.var_guard1205 != 0.0) {
        let assign42690_e55929: f64 = (8.0 * locals.var_temp__blk949);
        let assign42690_e55932: f64 = (12.0 * locals.var_xi0s);
        let assign42690_e55933: f64 = (assign42690_e55929 - assign42690_e55932);
        let assign42690_e55935: f64 = (assign42690_e55933 * locals.var_temp__blk949);
        let assign42690_e55937: f64 = (assign42690_e55935 * locals.var_temp__blk949);
        (assign42690_e55937, ((((((8.0 * locals.var_temp__blk949_dn4) - (12.0 * locals.var_xi0s_dn4)) * locals.var_temp__blk949) + (assign42690_e55933 * locals.var_temp__blk949_dn4)) * locals.var_temp__blk949) + (assign42690_e55935 * locals.var_temp__blk949_dn4)), ((((((8.0 * locals.var_temp__blk949_dn6) - (12.0 * locals.var_xi0s_dn6)) * locals.var_temp__blk949) + (assign42690_e55933 * locals.var_temp__blk949_dn6)) * locals.var_temp__blk949) + (assign42690_e55935 * locals.var_temp__blk949_dn6)), ((((((8.0 * locals.var_temp__blk949_dn7) - (12.0 * locals.var_xi0s_dn7)) * locals.var_temp__blk949) + (assign42690_e55933 * locals.var_temp__blk949_dn7)) * locals.var_temp__blk949) + (assign42690_e55935 * locals.var_temp__blk949_dn7)), ((((((8.0 * locals.var_temp__blk949_dn8) - (12.0 * locals.var_xi0s_dn8)) * locals.var_temp__blk949) + (assign42690_e55933 * locals.var_temp__blk949_dn8)) * locals.var_temp__blk949) + (assign42690_e55935 * locals.var_temp__blk949_dn8)), ((((((8.0 * locals.var_temp__blk949_dn9) - (12.0 * locals.var_xi0s_dn9)) * locals.var_temp__blk949) + (assign42690_e55933 * locals.var_temp__blk949_dn9)) * locals.var_temp__blk949) + (assign42690_e55935 * locals.var_temp__blk949_dn9)),)
    } else {
        (locals.var_xi2s, locals.var_xi2s_dn4, locals.var_xi2s_dn6, locals.var_xi2s_dn7, locals.var_xi2s_dn8, locals.var_xi2s_dn9,)
    }
};
        locals.var_xi2s = assign42690_e55939;
        locals.var_xi2s_dn4 = assign42690_e55939_d_n4;
        locals.var_xi2s_dn6 = assign42690_e55939_d_n6;
        locals.var_xi2s_dn7 = assign42690_e55939_d_n7;
        locals.var_xi2s_dn8 = assign42690_e55939_d_n8;
        locals.var_xi2s_dn9 = assign42690_e55939_d_n9;

        let (assign42700_e55943, assign42700_e55943_d_n4, assign42700_e55943_d_n6, assign42700_e55943_d_n7, assign42700_e55943_d_n8, assign42700_e55943_d_n9,) = {
    if (locals.var_guard1205 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delta_1s, locals.var_delta_1s_dn4, locals.var_delta_1s_dn6, locals.var_delta_1s_dn7, locals.var_delta_1s_dn8, locals.var_delta_1s_dn9,)
    }
};
        locals.var_delta_1s = assign42700_e55943;
        locals.var_delta_1s_dn4 = assign42700_e55943_d_n4;
        locals.var_delta_1s_dn6 = assign42700_e55943_d_n6;
        locals.var_delta_1s_dn7 = assign42700_e55943_d_n7;
        locals.var_delta_1s_dn8 = assign42700_e55943_d_n8;
        locals.var_delta_1s_dn9 = assign42700_e55943_d_n9;

        let assign42710_e55946: f64 = if locals.var_x_s < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1206 = assign42710_e55946;

        let (assign42720_e55953, assign42720_e55953_d_n4, assign42720_e55953_d_n6, assign42720_e55953_d_n7, assign42720_e55953_d_n8, assign42720_e55953_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1206 != 0.0)) {
        let assign42720_e55951: f64 = (locals.var_x_s).exp();
        (assign42720_e55951, (assign42720_e55951 * locals.var_x_s_dn4), (assign42720_e55951 * locals.var_x_s_dn6), (assign42720_e55951 * locals.var_x_s_dn7), (assign42720_e55951 * locals.var_x_s_dn8), (assign42720_e55951 * locals.var_x_s_dn9),)
    } else {
        (locals.var_delta_1s, locals.var_delta_1s_dn4, locals.var_delta_1s_dn6, locals.var_delta_1s_dn7, locals.var_delta_1s_dn8, locals.var_delta_1s_dn9,)
    }
};
        locals.var_delta_1s = assign42720_e55953;
        locals.var_delta_1s_dn4 = assign42720_e55953_d_n4;
        locals.var_delta_1s_dn6 = assign42720_e55953_d_n6;
        locals.var_delta_1s_dn7 = assign42720_e55953_d_n7;
        locals.var_delta_1s_dn8 = assign42720_e55953_d_n8;
        locals.var_delta_1s_dn9 = assign42720_e55953_d_n9;

        let (assign42730_e55961, assign42730_e55961_d_n4, assign42730_e55961_d_n6, assign42730_e55961_d_n7, assign42730_e55961_d_n8, assign42730_e55961_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1206 != 0.0)) {
        let assign42730_e55959: f64 = (1.0 / locals.var_delta_1s);
        (assign42730_e55959, (-(locals.var_delta_1s_dn4 / (locals.var_delta_1s * locals.var_delta_1s))), (-(locals.var_delta_1s_dn6 / (locals.var_delta_1s * locals.var_delta_1s))), (-(locals.var_delta_1s_dn7 / (locals.var_delta_1s * locals.var_delta_1s))), (-(locals.var_delta_1s_dn8 / (locals.var_delta_1s * locals.var_delta_1s))), (-(locals.var_delta_1s_dn9 / (locals.var_delta_1s * locals.var_delta_1s))),)
    } else {
        (locals.var_es, locals.var_es_dn4, locals.var_es_dn6, locals.var_es_dn7, locals.var_es_dn8, locals.var_es_dn9,)
    }
};
        locals.var_es = assign42730_e55961;
        locals.var_es_dn4 = assign42730_e55961_d_n4;
        locals.var_es_dn6 = assign42730_e55961_d_n6;
        locals.var_es_dn7 = assign42730_e55961_d_n7;
        locals.var_es_dn8 = assign42730_e55961_d_n8;
        locals.var_es_dn9 = assign42730_e55961_d_n9;

        let (assign42740_e55969, assign42740_e55969_d_n4, assign42740_e55969_d_n6, assign42740_e55969_d_n7, assign42740_e55969_d_n8, assign42740_e55969_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1206 != 0.0)) {
        let assign42740_e55967: f64 = (locals.var_delta_ns * locals.var_delta_1s);
        (assign42740_e55967, ((locals.var_delta_ns_dn4 * locals.var_delta_1s) + (locals.var_delta_ns * locals.var_delta_1s_dn4)), ((locals.var_delta_ns_dn6 * locals.var_delta_1s) + (locals.var_delta_ns * locals.var_delta_1s_dn6)), ((locals.var_delta_ns_dn7 * locals.var_delta_1s) + (locals.var_delta_ns * locals.var_delta_1s_dn7)), ((locals.var_delta_ns_dn8 * locals.var_delta_1s) + (locals.var_delta_ns * locals.var_delta_1s_dn8)), ((locals.var_delta_ns_dn9 * locals.var_delta_1s) + (locals.var_delta_ns * locals.var_delta_1s_dn9)),)
    } else {
        (locals.var_delta_1s, locals.var_delta_1s_dn4, locals.var_delta_1s_dn6, locals.var_delta_1s_dn7, locals.var_delta_1s_dn8, locals.var_delta_1s_dn9,)
    }
};
        locals.var_delta_1s = assign42740_e55969;
        locals.var_delta_1s_dn4 = assign42740_e55969_d_n4;
        locals.var_delta_1s_dn6 = assign42740_e55969_d_n6;
        locals.var_delta_1s_dn7 = assign42740_e55969_d_n7;
        locals.var_delta_1s_dn8 = assign42740_e55969_d_n8;
        locals.var_delta_1s_dn9 = assign42740_e55969_d_n9;

        let assign42750_e55973: f64 = (locals.var_xn_s - 230.25850929940458);
        let assign42750_e55974: f64 = if locals.var_x_s > assign42750_e55973 { 1.0 } else { 0.0 };
        locals.var_guard1207 = assign42750_e55974;

    }

    pub(super) fn stamp_transient_block_20(
        locals: &mut StampLocals,
    ) {
        let (assign42760_e55986, assign42760_e55986_d_n4, assign42760_e55986_d_n6, assign42760_e55986_d_n7, assign42760_e55986_d_n8, assign42760_e55986_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 != 0.0)) {
        let assign42760_e55983: f64 = (locals.var_x_s - locals.var_xn_s);
        let assign42760_e55984: f64 = (assign42760_e55983).exp();
        (assign42760_e55984, (assign42760_e55984 * (locals.var_x_s_dn4 - locals.var_xn_s_dn4)), (assign42760_e55984 * (locals.var_x_s_dn6 - locals.var_xn_s_dn6)), (assign42760_e55984 * (locals.var_x_s_dn7 - locals.var_xn_s_dn7)), (assign42760_e55984 * (locals.var_x_s_dn8 - locals.var_xn_s_dn8)), (assign42760_e55984 * (locals.var_x_s_dn9 - locals.var_xn_s_dn9)),)
    } else {
        (locals.var_delta_1s, locals.var_delta_1s_dn4, locals.var_delta_1s_dn6, locals.var_delta_1s_dn7, locals.var_delta_1s_dn8, locals.var_delta_1s_dn9,)
    }
};
        locals.var_delta_1s = assign42760_e55986;
        locals.var_delta_1s_dn4 = assign42760_e55986_d_n4;
        locals.var_delta_1s_dn6 = assign42760_e55986_d_n6;
        locals.var_delta_1s_dn7 = assign42760_e55986_d_n7;
        locals.var_delta_1s_dn8 = assign42760_e55986_d_n8;
        locals.var_delta_1s_dn9 = assign42760_e55986_d_n9;

        let (assign42770_e55997, assign42770_e55997_d_n4, assign42770_e55997_d_n6, assign42770_e55997_d_n7, assign42770_e55997_d_n8, assign42770_e55997_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 != 0.0)) {
        let assign42770_e55995: f64 = (locals.var_delta_ns / locals.var_delta_1s);
        (assign42770_e55995, (((locals.var_delta_ns_dn4 * locals.var_delta_1s) - (locals.var_delta_ns * locals.var_delta_1s_dn4)) / (locals.var_delta_1s * locals.var_delta_1s)), (((locals.var_delta_ns_dn6 * locals.var_delta_1s) - (locals.var_delta_ns * locals.var_delta_1s_dn6)) / (locals.var_delta_1s * locals.var_delta_1s)), (((locals.var_delta_ns_dn7 * locals.var_delta_1s) - (locals.var_delta_ns * locals.var_delta_1s_dn7)) / (locals.var_delta_1s * locals.var_delta_1s)), (((locals.var_delta_ns_dn8 * locals.var_delta_1s) - (locals.var_delta_ns * locals.var_delta_1s_dn8)) / (locals.var_delta_1s * locals.var_delta_1s)), (((locals.var_delta_ns_dn9 * locals.var_delta_1s) - (locals.var_delta_ns * locals.var_delta_1s_dn9)) / (locals.var_delta_1s * locals.var_delta_1s)),)
    } else {
        (locals.var_es, locals.var_es_dn4, locals.var_es_dn6, locals.var_es_dn7, locals.var_es_dn8, locals.var_es_dn9,)
    }
};
        locals.var_es = assign42770_e55997;
        locals.var_es_dn4 = assign42770_e55997_d_n4;
        locals.var_es_dn6 = assign42770_e55997_d_n6;
        locals.var_es_dn7 = assign42770_e55997_d_n7;
        locals.var_es_dn8 = assign42770_e55997_d_n8;
        locals.var_es_dn9 = assign42770_e55997_d_n9;

        let (assign42780_e56035, assign42780_e56035_d_n4, assign42780_e56035_d_n6, assign42780_e56035_d_n7, assign42780_e56035_d_n8, assign42780_e56035_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) {
        let assign42780_e56009: f64 = (locals.var_xn_s - locals.var_x_s);
        let assign42780_e56011: f64 = (assign42780_e56009 - 230.25850929940458);
        let assign42780_e56016: f64 = (locals.var_xn_s - locals.var_x_s);
        let assign42780_e56018: f64 = (assign42780_e56016 - 230.25850929940458);
        let assign42780_e56022: f64 = (locals.var_xn_s - locals.var_x_s);
        let assign42780_e56024: f64 = (assign42780_e56022 - 230.25850929940458);
        let assign42780_e56026: f64 = (assign42780_e56024 * 0.3333333333333333);
        let assign42780_e56027: f64 = (1.0 + assign42780_e56026);
        let assign42780_e56028: f64 = (assign42780_e56018 * assign42780_e56027);
        let assign42780_e56029: f64 = (0.5 * assign42780_e56028);
        let assign42780_e56030: f64 = (1.0 + assign42780_e56029);
        let assign42780_e56031: f64 = (assign42780_e56011 * assign42780_e56030);
        let assign42780_e56032: f64 = (1.0 + assign42780_e56031);
        let assign42780_e56033: f64 = (1e-100 / assign42780_e56032);
        (assign42780_e56033, (-((1e-100 * (((locals.var_xn_s_dn4 - locals.var_x_s_dn4) * assign42780_e56030) + (assign42780_e56011 * (0.5 * (((locals.var_xn_s_dn4 - locals.var_x_s_dn4) * assign42780_e56027) + (assign42780_e56018 * ((locals.var_xn_s_dn4 - locals.var_x_s_dn4) * 0.3333333333333333))))))) / (assign42780_e56032 * assign42780_e56032))), (-((1e-100 * (((locals.var_xn_s_dn6 - locals.var_x_s_dn6) * assign42780_e56030) + (assign42780_e56011 * (0.5 * (((locals.var_xn_s_dn6 - locals.var_x_s_dn6) * assign42780_e56027) + (assign42780_e56018 * ((locals.var_xn_s_dn6 - locals.var_x_s_dn6) * 0.3333333333333333))))))) / (assign42780_e56032 * assign42780_e56032))), (-((1e-100 * (((locals.var_xn_s_dn7 - locals.var_x_s_dn7) * assign42780_e56030) + (assign42780_e56011 * (0.5 * (((locals.var_xn_s_dn7 - locals.var_x_s_dn7) * assign42780_e56027) + (assign42780_e56018 * ((locals.var_xn_s_dn7 - locals.var_x_s_dn7) * 0.3333333333333333))))))) / (assign42780_e56032 * assign42780_e56032))), (-((1e-100 * (((locals.var_xn_s_dn8 - locals.var_x_s_dn8) * assign42780_e56030) + (assign42780_e56011 * (0.5 * (((locals.var_xn_s_dn8 - locals.var_x_s_dn8) * assign42780_e56027) + (assign42780_e56018 * ((locals.var_xn_s_dn8 - locals.var_x_s_dn8) * 0.3333333333333333))))))) / (assign42780_e56032 * assign42780_e56032))), (-((1e-100 * (((locals.var_xn_s_dn9 - locals.var_x_s_dn9) * assign42780_e56030) + (assign42780_e56011 * (0.5 * (((locals.var_xn_s_dn9 - locals.var_x_s_dn9) * assign42780_e56027) + (assign42780_e56018 * ((locals.var_xn_s_dn9 - locals.var_x_s_dn9) * 0.3333333333333333))))))) / (assign42780_e56032 * assign42780_e56032))),)
    } else {
        (locals.var_delta_1s, locals.var_delta_1s_dn4, locals.var_delta_1s_dn6, locals.var_delta_1s_dn7, locals.var_delta_1s_dn8, locals.var_delta_1s_dn9,)
    }
};
        locals.var_delta_1s = assign42780_e56035;
        locals.var_delta_1s_dn4 = assign42780_e56035_d_n4;
        locals.var_delta_1s_dn6 = assign42780_e56035_d_n6;
        locals.var_delta_1s_dn7 = assign42780_e56035_d_n7;
        locals.var_delta_1s_dn8 = assign42780_e56035_d_n8;
        locals.var_delta_1s_dn9 = assign42780_e56035_d_n9;

        let (assign42790_e56067, assign42790_e56067_d_n4, assign42790_e56067_d_n6, assign42790_e56067_d_n7, assign42790_e56067_d_n8, assign42790_e56067_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1206 == 0.0)) && (locals.var_guard1207 == 0.0)) {
        let assign42790_e56047: f64 = (locals.var_x_s - 230.25850929940458);
        let assign42790_e56052: f64 = (locals.var_x_s - 230.25850929940458);
        let assign42790_e56056: f64 = (locals.var_x_s - 230.25850929940458);
        let assign42790_e56058: f64 = (assign42790_e56056 * 0.3333333333333333);
        let assign42790_e56059: f64 = (1.0 + assign42790_e56058);
        let assign42790_e56060: f64 = (assign42790_e56052 * assign42790_e56059);
        let assign42790_e56061: f64 = (0.5 * assign42790_e56060);
        let assign42790_e56062: f64 = (1.0 + assign42790_e56061);
        let assign42790_e56063: f64 = (assign42790_e56047 * assign42790_e56062);
        let assign42790_e56064: f64 = (1.0 + assign42790_e56063);
        let assign42790_e56065: f64 = (1e-100 / assign42790_e56064);
        (assign42790_e56065, (-((1e-100 * ((locals.var_x_s_dn4 * assign42790_e56062) + (assign42790_e56047 * (0.5 * ((locals.var_x_s_dn4 * assign42790_e56059) + (assign42790_e56052 * (locals.var_x_s_dn4 * 0.3333333333333333))))))) / (assign42790_e56064 * assign42790_e56064))), (-((1e-100 * ((locals.var_x_s_dn6 * assign42790_e56062) + (assign42790_e56047 * (0.5 * ((locals.var_x_s_dn6 * assign42790_e56059) + (assign42790_e56052 * (locals.var_x_s_dn6 * 0.3333333333333333))))))) / (assign42790_e56064 * assign42790_e56064))), (-((1e-100 * ((locals.var_x_s_dn7 * assign42790_e56062) + (assign42790_e56047 * (0.5 * ((locals.var_x_s_dn7 * assign42790_e56059) + (assign42790_e56052 * (locals.var_x_s_dn7 * 0.3333333333333333))))))) / (assign42790_e56064 * assign42790_e56064))), (-((1e-100 * ((locals.var_x_s_dn8 * assign42790_e56062) + (assign42790_e56047 * (0.5 * ((locals.var_x_s_dn8 * assign42790_e56059) + (assign42790_e56052 * (locals.var_x_s_dn8 * 0.3333333333333333))))))) / (assign42790_e56064 * assign42790_e56064))), (-((1e-100 * ((locals.var_x_s_dn9 * assign42790_e56062) + (assign42790_e56047 * (0.5 * ((locals.var_x_s_dn9 * assign42790_e56059) + (assign42790_e56052 * (locals.var_x_s_dn9 * 0.3333333333333333))))))) / (assign42790_e56064 * assign42790_e56064))),)
    } else {
        (locals.var_es, locals.var_es_dn4, locals.var_es_dn6, locals.var_es_dn7, locals.var_es_dn8, locals.var_es_dn9,)
    }
};
        locals.var_es = assign42790_e56067;
        locals.var_es_dn4 = assign42790_e56067_d_n4;
        locals.var_es_dn6 = assign42790_e56067_d_n6;
        locals.var_es_dn7 = assign42790_e56067_d_n7;
        locals.var_es_dn8 = assign42790_e56067_d_n8;
        locals.var_es_dn9 = assign42790_e56067_d_n9;

        let (assign42800_e56079, assign42800_e56079_d_n4, assign42800_e56079_d_n6, assign42800_e56079_d_n7, assign42800_e56079_d_n8, assign42800_e56079_d_n9,) = {
    if (locals.var_guard1205 != 0.0) {
        let assign42800_e56073: f64 = (locals.var_x_s + 1.0);
        let assign42800_e56075: f64 = (assign42800_e56073 + locals.var_xi0s);
        let assign42800_e56076: f64 = (locals.var_delta_ns * assign42800_e56075);
        let assign42800_e56077: f64 = (locals.var_delta_1s - assign42800_e56076);
        (assign42800_e56077, (locals.var_delta_1s_dn4 - ((locals.var_delta_ns_dn4 * assign42800_e56075) + (locals.var_delta_ns * (locals.var_x_s_dn4 + locals.var_xi0s_dn4)))), (locals.var_delta_1s_dn6 - ((locals.var_delta_ns_dn6 * assign42800_e56075) + (locals.var_delta_ns * (locals.var_x_s_dn6 + locals.var_xi0s_dn6)))), (locals.var_delta_1s_dn7 - ((locals.var_delta_ns_dn7 * assign42800_e56075) + (locals.var_delta_ns * (locals.var_x_s_dn7 + locals.var_xi0s_dn7)))), (locals.var_delta_1s_dn8 - ((locals.var_delta_ns_dn8 * assign42800_e56075) + (locals.var_delta_ns * (locals.var_x_s_dn8 + locals.var_xi0s_dn8)))), (locals.var_delta_1s_dn9 - ((locals.var_delta_ns_dn9 * assign42800_e56075) + (locals.var_delta_ns * (locals.var_x_s_dn9 + locals.var_xi0s_dn9)))),)
    } else {
        (locals.var_ds, locals.var_ds_dn4, locals.var_ds_dn6, locals.var_ds_dn7, locals.var_ds_dn8, locals.var_ds_dn9,)
    }
};
        locals.var_ds = assign42800_e56079;
        locals.var_ds_dn4 = assign42800_e56079_d_n4;
        locals.var_ds_dn6 = assign42800_e56079_d_n6;
        locals.var_ds_dn7 = assign42800_e56079_d_n7;
        locals.var_ds_dn8 = assign42800_e56079_d_n8;
        locals.var_ds_dn9 = assign42800_e56079_d_n9;

        let assign42810_e56082: f64 = if locals.var_x_s < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1208 = assign42810_e56082;

        let (assign42820_e56104, assign42820_e56104_d_n4, assign42820_e56104_d_n6, assign42820_e56104_d_n7, assign42820_e56104_d_n8, assign42820_e56104_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1208 != 0.0)) {
        let assign42820_e56089: f64 = (locals.var_x_s * locals.var_x_s);
        let assign42820_e56096: f64 = (0.25 * locals.var_x_s);
        let assign42820_e56097: f64 = (1.0 - assign42820_e56096);
        let assign42820_e56098: f64 = (locals.var_x_s * assign42820_e56097);
        let assign42820_e56099: f64 = (0.3333333333333333 * assign42820_e56098);
        let assign42820_e56100: f64 = (1.0 - assign42820_e56099);
        let assign42820_e56101: f64 = (assign42820_e56089 * assign42820_e56100);
        let assign42820_e56102: f64 = (0.5 * assign42820_e56101);
        (assign42820_e56102, (0.5 * ((((locals.var_x_s_dn4 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn4)) * assign42820_e56100) + (assign42820_e56089 * (-(0.3333333333333333 * ((locals.var_x_s_dn4 * assign42820_e56097) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn4))))))))), (0.5 * ((((locals.var_x_s_dn6 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn6)) * assign42820_e56100) + (assign42820_e56089 * (-(0.3333333333333333 * ((locals.var_x_s_dn6 * assign42820_e56097) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn6))))))))), (0.5 * ((((locals.var_x_s_dn7 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn7)) * assign42820_e56100) + (assign42820_e56089 * (-(0.3333333333333333 * ((locals.var_x_s_dn7 * assign42820_e56097) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn7))))))))), (0.5 * ((((locals.var_x_s_dn8 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn8)) * assign42820_e56100) + (assign42820_e56089 * (-(0.3333333333333333 * ((locals.var_x_s_dn8 * assign42820_e56097) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn8))))))))), (0.5 * ((((locals.var_x_s_dn9 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn9)) * assign42820_e56100) + (assign42820_e56089 * (-(0.3333333333333333 * ((locals.var_x_s_dn9 * assign42820_e56097) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn9))))))))),)
    } else {
        (locals.var_ps, locals.var_ps_dn4, locals.var_ps_dn6, locals.var_ps_dn7, locals.var_ps_dn8, locals.var_ps_dn9,)
    }
};
        locals.var_ps = assign42820_e56104;
        locals.var_ps_dn4 = assign42820_e56104_d_n4;
        locals.var_ps_dn6 = assign42820_e56104_d_n6;
        locals.var_ps_dn7 = assign42820_e56104_d_n7;
        locals.var_ps_dn8 = assign42820_e56104_d_n8;
        locals.var_ps_dn9 = assign42820_e56104_d_n9;

        let (assign42830_e56124, assign42830_e56124_d_n4, assign42830_e56124_d_n6, assign42830_e56124_d_n7, assign42830_e56124_d_n8, assign42830_e56124_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1208 != 0.0)) {
        let assign42830_e56111: f64 = (locals.var_delta_ns * locals.var_x_s);
        let assign42830_e56113: f64 = (assign42830_e56111 * locals.var_x_s);
        let assign42830_e56115: f64 = (assign42830_e56113 * locals.var_x_s);
        let assign42830_e56119: f64 = (1.75 * locals.var_x_s);
        let assign42830_e56120: f64 = (1.0 + assign42830_e56119);
        let assign42830_e56121: f64 = (assign42830_e56115 * assign42830_e56120);
        let assign42830_e56122: f64 = (0.16666666666666666 * assign42830_e56121);
        (assign42830_e56122, (0.16666666666666666 * ((((((((locals.var_delta_ns_dn4 * locals.var_x_s) + (locals.var_delta_ns * locals.var_x_s_dn4)) * locals.var_x_s) + (assign42830_e56111 * locals.var_x_s_dn4)) * locals.var_x_s) + (assign42830_e56113 * locals.var_x_s_dn4)) * assign42830_e56120) + (assign42830_e56115 * (1.75 * locals.var_x_s_dn4)))), (0.16666666666666666 * ((((((((locals.var_delta_ns_dn6 * locals.var_x_s) + (locals.var_delta_ns * locals.var_x_s_dn6)) * locals.var_x_s) + (assign42830_e56111 * locals.var_x_s_dn6)) * locals.var_x_s) + (assign42830_e56113 * locals.var_x_s_dn6)) * assign42830_e56120) + (assign42830_e56115 * (1.75 * locals.var_x_s_dn6)))), (0.16666666666666666 * ((((((((locals.var_delta_ns_dn7 * locals.var_x_s) + (locals.var_delta_ns * locals.var_x_s_dn7)) * locals.var_x_s) + (assign42830_e56111 * locals.var_x_s_dn7)) * locals.var_x_s) + (assign42830_e56113 * locals.var_x_s_dn7)) * assign42830_e56120) + (assign42830_e56115 * (1.75 * locals.var_x_s_dn7)))), (0.16666666666666666 * ((((((((locals.var_delta_ns_dn8 * locals.var_x_s) + (locals.var_delta_ns * locals.var_x_s_dn8)) * locals.var_x_s) + (assign42830_e56111 * locals.var_x_s_dn8)) * locals.var_x_s) + (assign42830_e56113 * locals.var_x_s_dn8)) * assign42830_e56120) + (assign42830_e56115 * (1.75 * locals.var_x_s_dn8)))), (0.16666666666666666 * ((((((((locals.var_delta_ns_dn9 * locals.var_x_s) + (locals.var_delta_ns * locals.var_x_s_dn9)) * locals.var_x_s) + (assign42830_e56111 * locals.var_x_s_dn9)) * locals.var_x_s) + (assign42830_e56113 * locals.var_x_s_dn9)) * assign42830_e56120) + (assign42830_e56115 * (1.75 * locals.var_x_s_dn9)))),)
    } else {
        (locals.var_ds, locals.var_ds_dn4, locals.var_ds_dn6, locals.var_ds_dn7, locals.var_ds_dn8, locals.var_ds_dn9,)
    }
};
        locals.var_ds = assign42830_e56124;
        locals.var_ds_dn4 = assign42830_e56124_d_n4;
        locals.var_ds_dn6 = assign42830_e56124_d_n6;
        locals.var_ds_dn7 = assign42830_e56124_d_n7;
        locals.var_ds_dn8 = assign42830_e56124_d_n8;
        locals.var_ds_dn9 = assign42830_e56124_d_n9;

        let (assign42840_e56141, assign42840_e56141_d_n4, assign42840_e56141_d_n6, assign42840_e56141_d_n7, assign42840_e56141_d_n8, assign42840_e56141_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1208 != 0.0)) {
        let assign42840_e56134: f64 = (0.25 * locals.var_x_s);
        let assign42840_e56135: f64 = (1.0 - assign42840_e56134);
        let assign42840_e56136: f64 = (locals.var_x_s * assign42840_e56135);
        let assign42840_e56137: f64 = (0.3333333333333333 * assign42840_e56136);
        let assign42840_e56138: f64 = (1.0 - assign42840_e56137);
        let assign42840_e56139: f64 = (assign42840_e56138).sqrt();
        (assign42840_e56139, ((-(0.3333333333333333 * ((locals.var_x_s_dn4 * assign42840_e56135) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn4)))))) / (2.0 * assign42840_e56139)), ((-(0.3333333333333333 * ((locals.var_x_s_dn6 * assign42840_e56135) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn6)))))) / (2.0 * assign42840_e56139)), ((-(0.3333333333333333 * ((locals.var_x_s_dn7 * assign42840_e56135) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn7)))))) / (2.0 * assign42840_e56139)), ((-(0.3333333333333333 * ((locals.var_x_s_dn8 * assign42840_e56135) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn8)))))) / (2.0 * assign42840_e56139)), ((-(0.3333333333333333 * ((locals.var_x_s_dn9 * assign42840_e56135) + (locals.var_x_s * (-(0.25 * locals.var_x_s_dn9)))))) / (2.0 * assign42840_e56139)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign42840_e56141;
        locals.var_temp__blk949_dn4 = assign42840_e56141_d_n4;
        locals.var_temp__blk949_dn6 = assign42840_e56141_d_n6;
        locals.var_temp__blk949_dn7 = assign42840_e56141_d_n7;
        locals.var_temp__blk949_dn8 = assign42840_e56141_d_n8;
        locals.var_temp__blk949_dn9 = assign42840_e56141_d_n9;

        let (assign42850_e56151, assign42850_e56151_d_n4, assign42850_e56151_d_n6, assign42850_e56151_d_n7, assign42850_e56151_d_n8, assign42850_e56151_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1208 != 0.0)) {
        let assign42850_e56148: f64 = (locals.var_x_s * locals.var_temp__blk949);
        let assign42850_e56149: f64 = (0.7071067811865475 * assign42850_e56148);
        (assign42850_e56149, (0.7071067811865475 * ((locals.var_x_s_dn4 * locals.var_temp__blk949) + (locals.var_x_s * locals.var_temp__blk949_dn4))), (0.7071067811865475 * ((locals.var_x_s_dn6 * locals.var_temp__blk949) + (locals.var_x_s * locals.var_temp__blk949_dn6))), (0.7071067811865475 * ((locals.var_x_s_dn7 * locals.var_temp__blk949) + (locals.var_x_s * locals.var_temp__blk949_dn7))), (0.7071067811865475 * ((locals.var_x_s_dn8 * locals.var_temp__blk949) + (locals.var_x_s * locals.var_temp__blk949_dn8))), (0.7071067811865475 * ((locals.var_x_s_dn9 * locals.var_temp__blk949) + (locals.var_x_s * locals.var_temp__blk949_dn9))),)
    } else {
        (locals.var_sqs, locals.var_sqs_dn4, locals.var_sqs_dn6, locals.var_sqs_dn7, locals.var_sqs_dn8, locals.var_sqs_dn9,)
    }
};
        locals.var_sqs = assign42850_e56151;
        locals.var_sqs_dn4 = assign42850_e56151_d_n4;
        locals.var_sqs_dn6 = assign42850_e56151_d_n6;
        locals.var_sqs_dn7 = assign42850_e56151_d_n7;
        locals.var_sqs_dn8 = assign42850_e56151_d_n8;
        locals.var_sqs_dn9 = assign42850_e56151_d_n9;

        let (assign42860_e56175, assign42860_e56175_d_n4, assign42860_e56175_d_n6, assign42860_e56175_d_n7, assign42860_e56175_d_n8, assign42860_e56175_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1208 != 0.0)) {
        let assign42860_e56161: f64 = (0.5 * locals.var_x_s);
        let assign42860_e56162: f64 = (1.0 - assign42860_e56161);
        let assign42860_e56166: f64 = (locals.var_x_s * locals.var_x_s);
        let assign42860_e56167: f64 = (0.16666666666666666 * assign42860_e56166);
        let assign42860_e56168: f64 = (assign42860_e56162 + assign42860_e56167);
        let assign42860_e56169: f64 = (locals.var_gf * assign42860_e56168);
        let assign42860_e56171: f64 = (assign42860_e56169 / locals.var_temp__blk949);
        let assign42860_e56172: f64 = (0.7071067811865475 * assign42860_e56171);
        let assign42860_e56173: f64 = (1.0 + assign42860_e56172);
        (assign42860_e56173, (0.7071067811865475 * (((((locals.var_gf_dn4 * assign42860_e56168) + (locals.var_gf * ((-(0.5 * locals.var_x_s_dn4)) + (0.16666666666666666 * ((locals.var_x_s_dn4 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn4)))))) * locals.var_temp__blk949) - (assign42860_e56169 * locals.var_temp__blk949_dn4)) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (0.7071067811865475 * (((((locals.var_gf_dn6 * assign42860_e56168) + (locals.var_gf * ((-(0.5 * locals.var_x_s_dn6)) + (0.16666666666666666 * ((locals.var_x_s_dn6 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn6)))))) * locals.var_temp__blk949) - (assign42860_e56169 * locals.var_temp__blk949_dn6)) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (0.7071067811865475 * (((((locals.var_gf_dn7 * assign42860_e56168) + (locals.var_gf * ((-(0.5 * locals.var_x_s_dn7)) + (0.16666666666666666 * ((locals.var_x_s_dn7 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn7)))))) * locals.var_temp__blk949) - (assign42860_e56169 * locals.var_temp__blk949_dn7)) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (0.7071067811865475 * (((((locals.var_gf_dn8 * assign42860_e56168) + (locals.var_gf * ((-(0.5 * locals.var_x_s_dn8)) + (0.16666666666666666 * ((locals.var_x_s_dn8 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn8)))))) * locals.var_temp__blk949) - (assign42860_e56169 * locals.var_temp__blk949_dn8)) / (locals.var_temp__blk949 * locals.var_temp__blk949))), (0.7071067811865475 * (((((locals.var_gf_dn9 * assign42860_e56168) + (locals.var_gf * ((-(0.5 * locals.var_x_s_dn9)) + (0.16666666666666666 * ((locals.var_x_s_dn9 * locals.var_x_s) + (locals.var_x_s * locals.var_x_s_dn9)))))) * locals.var_temp__blk949) - (assign42860_e56169 * locals.var_temp__blk949_dn9)) / (locals.var_temp__blk949 * locals.var_temp__blk949))),)
    } else {
        (locals.var_alphas, locals.var_alphas_dn4, locals.var_alphas_dn6, locals.var_alphas_dn7, locals.var_alphas_dn8, locals.var_alphas_dn9,)
    }
};
        locals.var_alphas = assign42860_e56175;
        locals.var_alphas_dn4 = assign42860_e56175_d_n4;
        locals.var_alphas_dn6 = assign42860_e56175_d_n6;
        locals.var_alphas_dn7 = assign42860_e56175_d_n7;
        locals.var_alphas_dn8 = assign42860_e56175_d_n8;
        locals.var_alphas_dn9 = assign42860_e56175_d_n9;

        let (assign42870_e56186, assign42870_e56186_d_n4, assign42870_e56186_d_n6, assign42870_e56186_d_n7, assign42870_e56186_d_n8, assign42870_e56186_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1208 == 0.0)) {
        let assign42870_e56182: f64 = (locals.var_x_s - 1.0);
        let assign42870_e56184: f64 = (assign42870_e56182 + locals.var_es);
        (assign42870_e56184, (locals.var_x_s_dn4 + locals.var_es_dn4), (locals.var_x_s_dn6 + locals.var_es_dn6), (locals.var_x_s_dn7 + locals.var_es_dn7), (locals.var_x_s_dn8 + locals.var_es_dn8), (locals.var_x_s_dn9 + locals.var_es_dn9),)
    } else {
        (locals.var_ps, locals.var_ps_dn4, locals.var_ps_dn6, locals.var_ps_dn7, locals.var_ps_dn8, locals.var_ps_dn9,)
    }
};
        locals.var_ps = assign42870_e56186;
        locals.var_ps_dn4 = assign42870_e56186_d_n4;
        locals.var_ps_dn6 = assign42870_e56186_d_n6;
        locals.var_ps_dn7 = assign42870_e56186_d_n7;
        locals.var_ps_dn8 = assign42870_e56186_d_n8;
        locals.var_ps_dn9 = assign42870_e56186_d_n9;

        let (assign42880_e56194, assign42880_e56194_d_n4, assign42880_e56194_d_n6, assign42880_e56194_d_n7, assign42880_e56194_d_n8, assign42880_e56194_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1208 == 0.0)) {
        let assign42880_e56192: f64 = (locals.var_ps).sqrt();
        (assign42880_e56192, (locals.var_ps_dn4 / (2.0 * assign42880_e56192)), (locals.var_ps_dn6 / (2.0 * assign42880_e56192)), (locals.var_ps_dn7 / (2.0 * assign42880_e56192)), (locals.var_ps_dn8 / (2.0 * assign42880_e56192)), (locals.var_ps_dn9 / (2.0 * assign42880_e56192)),)
    } else {
        (locals.var_sqs, locals.var_sqs_dn4, locals.var_sqs_dn6, locals.var_sqs_dn7, locals.var_sqs_dn8, locals.var_sqs_dn9,)
    }
};
        locals.var_sqs = assign42880_e56194;
        locals.var_sqs_dn4 = assign42880_e56194_d_n4;
        locals.var_sqs_dn6 = assign42880_e56194_d_n6;
        locals.var_sqs_dn7 = assign42880_e56194_d_n7;
        locals.var_sqs_dn8 = assign42880_e56194_d_n8;
        locals.var_sqs_dn9 = assign42880_e56194_d_n9;

        let (assign42890_e56211, assign42890_e56211_d_n4, assign42890_e56211_d_n6, assign42890_e56211_d_n7, assign42890_e56211_d_n8, assign42890_e56211_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1208 == 0.0)) {
        let assign42890_e56204: f64 = (1.0 - locals.var_es);
        let assign42890_e56205: f64 = (locals.var_gf * assign42890_e56204);
        let assign42890_e56207: f64 = (assign42890_e56205 / locals.var_sqs);
        let assign42890_e56208: f64 = (0.5 * assign42890_e56207);
        let assign42890_e56209: f64 = (1.0 + assign42890_e56208);
        (assign42890_e56209, (0.5 * (((((locals.var_gf_dn4 * assign42890_e56204) + (locals.var_gf * (-locals.var_es_dn4))) * locals.var_sqs) - (assign42890_e56205 * locals.var_sqs_dn4)) / (locals.var_sqs * locals.var_sqs))), (0.5 * (((((locals.var_gf_dn6 * assign42890_e56204) + (locals.var_gf * (-locals.var_es_dn6))) * locals.var_sqs) - (assign42890_e56205 * locals.var_sqs_dn6)) / (locals.var_sqs * locals.var_sqs))), (0.5 * (((((locals.var_gf_dn7 * assign42890_e56204) + (locals.var_gf * (-locals.var_es_dn7))) * locals.var_sqs) - (assign42890_e56205 * locals.var_sqs_dn7)) / (locals.var_sqs * locals.var_sqs))), (0.5 * (((((locals.var_gf_dn8 * assign42890_e56204) + (locals.var_gf * (-locals.var_es_dn8))) * locals.var_sqs) - (assign42890_e56205 * locals.var_sqs_dn8)) / (locals.var_sqs * locals.var_sqs))), (0.5 * (((((locals.var_gf_dn9 * assign42890_e56204) + (locals.var_gf * (-locals.var_es_dn9))) * locals.var_sqs) - (assign42890_e56205 * locals.var_sqs_dn9)) / (locals.var_sqs * locals.var_sqs))),)
    } else {
        (locals.var_alphas, locals.var_alphas_dn4, locals.var_alphas_dn6, locals.var_alphas_dn7, locals.var_alphas_dn8, locals.var_alphas_dn9,)
    }
};
        locals.var_alphas = assign42890_e56211;
        locals.var_alphas_dn4 = assign42890_e56211_d_n4;
        locals.var_alphas_dn6 = assign42890_e56211_d_n6;
        locals.var_alphas_dn7 = assign42890_e56211_d_n7;
        locals.var_alphas_dn8 = assign42890_e56211_d_n8;
        locals.var_alphas_dn9 = assign42890_e56211_d_n9;

        let (assign42900_e56227, assign42900_e56227_d_n4, assign42900_e56227_d_n6, assign42900_e56227_d_n7, assign42900_e56227_d_n8, assign42900_e56227_d_n9,) = {
    if (locals.var_guard1205 != 0.0) {
        let assign42900_e56216: f64 = (0.2 * locals.var_xcor_t);
        let assign42900_e56218: f64 = (assign42900_e56216 * locals.var_vsbx);
        let assign42900_e56219: f64 = (1.0 + assign42900_e56218);
        let assign42900_e56223: f64 = (locals.var_xcor_t * locals.var_vsbx);
        let assign42900_e56224: f64 = (1.0 + assign42900_e56223);
        let assign42900_e56225: f64 = (assign42900_e56219 / assign42900_e56224);
        (assign42900_e56225, ((((((0.2 * locals.var_xcor_t_dn4) * locals.var_vsbx) + (assign42900_e56216 * locals.var_vsbx_dn4)) * assign42900_e56224) - (assign42900_e56219 * ((locals.var_xcor_t_dn4 * locals.var_vsbx) + (locals.var_xcor_t * locals.var_vsbx_dn4)))) / (assign42900_e56224 * assign42900_e56224)), ((((assign42900_e56216 * locals.var_vsbx_dn6) * assign42900_e56224) - (assign42900_e56219 * (locals.var_xcor_t * locals.var_vsbx_dn6))) / (assign42900_e56224 * assign42900_e56224)), ((((assign42900_e56216 * locals.var_vsbx_dn7) * assign42900_e56224) - (assign42900_e56219 * (locals.var_xcor_t * locals.var_vsbx_dn7))) / (assign42900_e56224 * assign42900_e56224)), ((((assign42900_e56216 * locals.var_vsbx_dn8) * assign42900_e56224) - (assign42900_e56219 * (locals.var_xcor_t * locals.var_vsbx_dn8))) / (assign42900_e56224 * assign42900_e56224)), ((((assign42900_e56216 * locals.var_vsbx_dn9) * assign42900_e56224) - (assign42900_e56219 * (locals.var_xcor_t * locals.var_vsbx_dn9))) / (assign42900_e56224 * assign42900_e56224)),)
    } else {
        (locals.var_rxcor, locals.var_rxcor_dn4, locals.var_rxcor_dn6, locals.var_rxcor_dn7, locals.var_rxcor_dn8, locals.var_rxcor_dn9,)
    }
};
        locals.var_rxcor = assign42900_e56227;
        locals.var_rxcor_dn4 = assign42900_e56227_d_n4;
        locals.var_rxcor_dn6 = assign42900_e56227_d_n6;
        locals.var_rxcor_dn7 = assign42900_e56227_d_n7;
        locals.var_rxcor_dn8 = assign42900_e56227_d_n8;
        locals.var_rxcor_dn9 = assign42900_e56227_d_n9;

        let assign42910_e56230: f64 = if locals.var_ds > 1e-100 { 1.0 } else { 0.0 };
        locals.var_guard1209 = assign42910_e56230;

        let (assign42920_e56241, assign42920_e56241_d_n4, assign42920_e56241_d_n6, assign42920_e56241_d_n7, assign42920_e56241_d_n8, assign42920_e56241_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) {
        let assign42920_e56237: f64 = (locals.var_ps + locals.var_ds);
        let assign42920_e56238: f64 = (assign42920_e56237).sqrt();
        let assign42920_e56239: f64 = (locals.var_gf * assign42920_e56238);
        (assign42920_e56239, ((locals.var_gf_dn4 * assign42920_e56238) + (locals.var_gf * ((locals.var_ps_dn4 + locals.var_ds_dn4) / (2.0 * assign42920_e56238)))), ((locals.var_gf_dn6 * assign42920_e56238) + (locals.var_gf * ((locals.var_ps_dn6 + locals.var_ds_dn6) / (2.0 * assign42920_e56238)))), ((locals.var_gf_dn7 * assign42920_e56238) + (locals.var_gf * ((locals.var_ps_dn7 + locals.var_ds_dn7) / (2.0 * assign42920_e56238)))), ((locals.var_gf_dn8 * assign42920_e56238) + (locals.var_gf * ((locals.var_ps_dn8 + locals.var_ds_dn8) / (2.0 * assign42920_e56238)))), ((locals.var_gf_dn9 * assign42920_e56238) + (locals.var_gf * ((locals.var_ps_dn9 + locals.var_ds_dn9) / (2.0 * assign42920_e56238)))),)
    } else {
        (locals.var_xgs, locals.var_xgs_dn4, locals.var_xgs_dn6, locals.var_xgs_dn7, locals.var_xgs_dn8, locals.var_xgs_dn9,)
    }
};
        locals.var_xgs = assign42920_e56241;
        locals.var_xgs_dn4 = assign42920_e56241_d_n4;
        locals.var_xgs_dn6 = assign42920_e56241_d_n6;
        locals.var_xgs_dn7 = assign42920_e56241_d_n7;
        locals.var_xgs_dn8 = assign42920_e56241_d_n8;
        locals.var_xgs_dn9 = assign42920_e56241_d_n9;

        let (assign42930_e56257, assign42930_e56257_d_n4, assign42930_e56257_d_n6, assign42930_e56257_d_n7, assign42930_e56257_d_n8, assign42930_e56257_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) {
        let assign42930_e56247: f64 = (locals.var_gf2 * locals.var_ds);
        let assign42930_e56249: f64 = (assign42930_e56247 * locals.var_phit1);
        let assign42930_e56253: f64 = (locals.var_gf * locals.var_sqs);
        let assign42930_e56254: f64 = (locals.var_xgs + assign42930_e56253);
        let assign42930_e56255: f64 = (assign42930_e56249 / assign42930_e56254);
        (assign42930_e56255, (((((((locals.var_gf2_dn4 * locals.var_ds) + (locals.var_gf2 * locals.var_ds_dn4)) * locals.var_phit1) + (assign42930_e56247 * locals.var_phit1_dn4)) * assign42930_e56254) - (assign42930_e56249 * (locals.var_xgs_dn4 + ((locals.var_gf_dn4 * locals.var_sqs) + (locals.var_gf * locals.var_sqs_dn4))))) / (assign42930_e56254 * assign42930_e56254)), (((((((locals.var_gf2_dn6 * locals.var_ds) + (locals.var_gf2 * locals.var_ds_dn6)) * locals.var_phit1) + (assign42930_e56247 * locals.var_phit1_dn6)) * assign42930_e56254) - (assign42930_e56249 * (locals.var_xgs_dn6 + ((locals.var_gf_dn6 * locals.var_sqs) + (locals.var_gf * locals.var_sqs_dn6))))) / (assign42930_e56254 * assign42930_e56254)), (((((((locals.var_gf2_dn7 * locals.var_ds) + (locals.var_gf2 * locals.var_ds_dn7)) * locals.var_phit1) + (assign42930_e56247 * locals.var_phit1_dn7)) * assign42930_e56254) - (assign42930_e56249 * (locals.var_xgs_dn7 + ((locals.var_gf_dn7 * locals.var_sqs) + (locals.var_gf * locals.var_sqs_dn7))))) / (assign42930_e56254 * assign42930_e56254)), (((((((locals.var_gf2_dn8 * locals.var_ds) + (locals.var_gf2 * locals.var_ds_dn8)) * locals.var_phit1) + (assign42930_e56247 * locals.var_phit1_dn8)) * assign42930_e56254) - (assign42930_e56249 * (locals.var_xgs_dn8 + ((locals.var_gf_dn8 * locals.var_sqs) + (locals.var_gf * locals.var_sqs_dn8))))) / (assign42930_e56254 * assign42930_e56254)), (((((((locals.var_gf2_dn9 * locals.var_ds) + (locals.var_gf2 * locals.var_ds_dn9)) * locals.var_phit1) + (assign42930_e56247 * locals.var_phit1_dn9)) * assign42930_e56254) - (assign42930_e56249 * (locals.var_xgs_dn9 + ((locals.var_gf_dn9 * locals.var_sqs) + (locals.var_gf * locals.var_sqs_dn9))))) / (assign42930_e56254 * assign42930_e56254)),)
    } else {
        (locals.var_qis, locals.var_qis_dn4, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, locals.var_qis_dn9,)
    }
};
        locals.var_qis = assign42930_e56257;
        locals.var_qis_dn4 = assign42930_e56257_d_n4;
        locals.var_qis_dn6 = assign42930_e56257_d_n6;
        locals.var_qis_dn7 = assign42930_e56257_d_n7;
        locals.var_qis_dn8 = assign42930_e56257_d_n8;
        locals.var_qis_dn9 = assign42930_e56257_d_n9;

        let (assign42940_e56267, assign42940_e56267_d_n4, assign42940_e56267_d_n6, assign42940_e56267_d_n7, assign42940_e56267_d_n8, assign42940_e56267_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) {
        let assign42940_e56263: f64 = (locals.var_sqs * locals.var_gf);
        let assign42940_e56265: f64 = (assign42940_e56263 * locals.var_phit1);
        (assign42940_e56265, ((((locals.var_sqs_dn4 * locals.var_gf) + (locals.var_sqs * locals.var_gf_dn4)) * locals.var_phit1) + (assign42940_e56263 * locals.var_phit1_dn4)), ((((locals.var_sqs_dn6 * locals.var_gf) + (locals.var_sqs * locals.var_gf_dn6)) * locals.var_phit1) + (assign42940_e56263 * locals.var_phit1_dn6)), ((((locals.var_sqs_dn7 * locals.var_gf) + (locals.var_sqs * locals.var_gf_dn7)) * locals.var_phit1) + (assign42940_e56263 * locals.var_phit1_dn7)), ((((locals.var_sqs_dn8 * locals.var_gf) + (locals.var_sqs * locals.var_gf_dn8)) * locals.var_phit1) + (assign42940_e56263 * locals.var_phit1_dn8)), ((((locals.var_sqs_dn9 * locals.var_gf) + (locals.var_sqs * locals.var_gf_dn9)) * locals.var_phit1) + (assign42940_e56263 * locals.var_phit1_dn9)),)
    } else {
        (locals.var_qbs, locals.var_qbs_dn4, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn8, locals.var_qbs_dn9,)
    }
};
        locals.var_qbs = assign42940_e56267;
        locals.var_qbs_dn4 = assign42940_e56267_d_n4;
        locals.var_qbs_dn6 = assign42940_e56267_d_n6;
        locals.var_qbs_dn7 = assign42940_e56267_d_n7;
        locals.var_qbs_dn8 = assign42940_e56267_d_n8;
        locals.var_qbs_dn9 = assign42940_e56267_d_n9;

        let assign42950_e56270: f64 = if locals.var_rsb_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1210 = assign42950_e56270;

        let (assign42960_e56284, assign42960_e56284_d_n4, assign42960_e56284_d_n6, assign42960_e56284_d_n7, assign42960_e56284_d_n8, assign42960_e56284_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 != 0.0)) {
        let assign42960_e56280: f64 = (locals.var_rsb_i * locals.var_vsbx);
        let assign42960_e56281: f64 = (1.0 - assign42960_e56280);
        let assign42960_e56282: f64 = (1.0 / assign42960_e56281);
        (assign42960_e56282, (-((-(locals.var_rsb_i * locals.var_vsbx_dn4)) / (assign42960_e56281 * assign42960_e56281))), (-((-(locals.var_rsb_i * locals.var_vsbx_dn6)) / (assign42960_e56281 * assign42960_e56281))), (-((-(locals.var_rsb_i * locals.var_vsbx_dn7)) / (assign42960_e56281 * assign42960_e56281))), (-((-(locals.var_rsb_i * locals.var_vsbx_dn8)) / (assign42960_e56281 * assign42960_e56281))), (-((-(locals.var_rsb_i * locals.var_vsbx_dn9)) / (assign42960_e56281 * assign42960_e56281))),)
    } else {
        (locals.var_rhob, locals.var_rhob_dn4, locals.var_rhob_dn6, locals.var_rhob_dn7, locals.var_rhob_dn8, locals.var_rhob_dn9,)
    }
};
        locals.var_rhob = assign42960_e56284;
        locals.var_rhob_dn4 = assign42960_e56284_d_n4;
        locals.var_rhob_dn6 = assign42960_e56284_d_n6;
        locals.var_rhob_dn7 = assign42960_e56284_d_n7;
        locals.var_rhob_dn8 = assign42960_e56284_d_n8;
        locals.var_rhob_dn9 = assign42960_e56284_d_n9;

        let (assign42970_e56297, assign42970_e56297_d_n4, assign42970_e56297_d_n6, assign42970_e56297_d_n7, assign42970_e56297_d_n8, assign42970_e56297_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1210 == 0.0)) {
        let assign42970_e56294: f64 = (locals.var_rsb_i * locals.var_vsbx);
        let assign42970_e56295: f64 = (1.0 + assign42970_e56294);
        (assign42970_e56295, (locals.var_rsb_i * locals.var_vsbx_dn4), (locals.var_rsb_i * locals.var_vsbx_dn6), (locals.var_rsb_i * locals.var_vsbx_dn7), (locals.var_rsb_i * locals.var_vsbx_dn8), (locals.var_rsb_i * locals.var_vsbx_dn9),)
    } else {
        (locals.var_rhob, locals.var_rhob_dn4, locals.var_rhob_dn6, locals.var_rhob_dn7, locals.var_rhob_dn8, locals.var_rhob_dn9,)
    }
};
        locals.var_rhob = assign42970_e56297;
        locals.var_rhob_dn4 = assign42970_e56297_d_n4;
        locals.var_rhob_dn6 = assign42970_e56297_d_n6;
        locals.var_rhob_dn7 = assign42970_e56297_d_n7;
        locals.var_rhob_dn8 = assign42970_e56297_d_n8;
        locals.var_rhob_dn9 = assign42970_e56297_d_n9;

        let assign42980_e56300: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1211 = assign42980_e56300;

        let (assign42990_e56312, assign42990_e56312_d_n4, assign42990_e56312_d_n6, assign42990_e56312_d_n7, assign42990_e56312_d_n8, assign42990_e56312_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1211 != 0.0)) {
        let assign42990_e56309: f64 = (locals.var_rsg_i * locals.var_qis);
        let assign42990_e56310: f64 = (1.0 - assign42990_e56309);
        (assign42990_e56310, (-(locals.var_rsg_i * locals.var_qis_dn4)), (-(locals.var_rsg_i * locals.var_qis_dn6)), (-(locals.var_rsg_i * locals.var_qis_dn7)), (-(locals.var_rsg_i * locals.var_qis_dn8)), (-(locals.var_rsg_i * locals.var_qis_dn9)),)
    } else {
        (locals.var_rhog, locals.var_rhog_dn4, locals.var_rhog_dn6, locals.var_rhog_dn7, locals.var_rhog_dn8, locals.var_rhog_dn9,)
    }
};
        locals.var_rhog = assign42990_e56312;
        locals.var_rhog_dn4 = assign42990_e56312_d_n4;
        locals.var_rhog_dn6 = assign42990_e56312_d_n6;
        locals.var_rhog_dn7 = assign42990_e56312_d_n7;
        locals.var_rhog_dn8 = assign42990_e56312_d_n8;
        locals.var_rhog_dn9 = assign42990_e56312_d_n9;

        let (assign43000_e56327, assign43000_e56327_d_n4, assign43000_e56327_d_n6, assign43000_e56327_d_n7, assign43000_e56327_d_n8, assign43000_e56327_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1211 == 0.0)) {
        let assign43000_e56323: f64 = (locals.var_rsg_i * locals.var_qis);
        let assign43000_e56324: f64 = (1.0 + assign43000_e56323);
        let assign43000_e56325: f64 = (1.0 / assign43000_e56324);
        (assign43000_e56325, (-((locals.var_rsg_i * locals.var_qis_dn4) / (assign43000_e56324 * assign43000_e56324))), (-((locals.var_rsg_i * locals.var_qis_dn6) / (assign43000_e56324 * assign43000_e56324))), (-((locals.var_rsg_i * locals.var_qis_dn7) / (assign43000_e56324 * assign43000_e56324))), (-((locals.var_rsg_i * locals.var_qis_dn8) / (assign43000_e56324 * assign43000_e56324))), (-((locals.var_rsg_i * locals.var_qis_dn9) / (assign43000_e56324 * assign43000_e56324))),)
    } else {
        (locals.var_rhog, locals.var_rhog_dn4, locals.var_rhog_dn6, locals.var_rhog_dn7, locals.var_rhog_dn8, locals.var_rhog_dn9,)
    }
};
        locals.var_rhog = assign43000_e56327;
        locals.var_rhog_dn4 = assign43000_e56327_d_n4;
        locals.var_rhog_dn6 = assign43000_e56327_d_n6;
        locals.var_rhog_dn7 = assign43000_e56327_d_n7;
        locals.var_rhog_dn8 = assign43000_e56327_d_n8;
        locals.var_rhog_dn9 = assign43000_e56327_d_n9;

        let (assign43010_e56339, assign43010_e56339_d_n4, assign43010_e56339_d_n6, assign43010_e56339_d_n7, assign43010_e56339_d_n8, assign43010_e56339_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) {
        let assign43010_e56333: f64 = (locals.var_ther_i * locals.var_rhob);
        let assign43010_e56335: f64 = (assign43010_e56333 * locals.var_rhog);
        let assign43010_e56337: f64 = (assign43010_e56335 * locals.var_qis);
        (assign43010_e56337, ((((((locals.var_ther_i_dn4 * locals.var_rhob) + (locals.var_ther_i * locals.var_rhob_dn4)) * locals.var_rhog) + (assign43010_e56333 * locals.var_rhog_dn4)) * locals.var_qis) + (assign43010_e56335 * locals.var_qis_dn4)), (((((locals.var_ther_i * locals.var_rhob_dn6) * locals.var_rhog) + (assign43010_e56333 * locals.var_rhog_dn6)) * locals.var_qis) + (assign43010_e56335 * locals.var_qis_dn6)), (((((locals.var_ther_i * locals.var_rhob_dn7) * locals.var_rhog) + (assign43010_e56333 * locals.var_rhog_dn7)) * locals.var_qis) + (assign43010_e56335 * locals.var_qis_dn7)), (((((locals.var_ther_i * locals.var_rhob_dn8) * locals.var_rhog) + (assign43010_e56333 * locals.var_rhog_dn8)) * locals.var_qis) + (assign43010_e56335 * locals.var_qis_dn8)), (((((locals.var_ther_i * locals.var_rhob_dn9) * locals.var_rhog) + (assign43010_e56333 * locals.var_rhog_dn9)) * locals.var_qis) + (assign43010_e56335 * locals.var_qis_dn9)),)
    } else {
        (locals.var_gr, locals.var_gr_dn4, locals.var_gr_dn6, locals.var_gr_dn7, locals.var_gr_dn8, locals.var_gr_dn9,)
    }
};
        locals.var_gr = assign43010_e56339;
        locals.var_gr_dn4 = assign43010_e56339_d_n4;
        locals.var_gr_dn6 = assign43010_e56339_d_n6;
        locals.var_gr_dn7 = assign43010_e56339_d_n7;
        locals.var_gr_dn8 = assign43010_e56339_d_n8;
        locals.var_gr_dn9 = assign43010_e56339_d_n9;

        let (assign43020_e56351, assign43020_e56351_d_n4, assign43020_e56351_d_n6, assign43020_e56351_d_n7, assign43020_e56351_d_n8, assign43020_e56351_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) {
        let assign43020_e56347: f64 = (locals.var_eta_mu * locals.var_qis);
        let assign43020_e56348: f64 = (locals.var_qbs + assign43020_e56347);
        let assign43020_e56349: f64 = (locals.var_e_eff0 * assign43020_e56348);
        (assign43020_e56349, (locals.var_e_eff0 * (locals.var_qbs_dn4 + (locals.var_eta_mu * locals.var_qis_dn4))), (locals.var_e_eff0 * (locals.var_qbs_dn6 + (locals.var_eta_mu * locals.var_qis_dn6))), (locals.var_e_eff0 * (locals.var_qbs_dn7 + (locals.var_eta_mu * locals.var_qis_dn7))), (locals.var_e_eff0 * (locals.var_qbs_dn8 + (locals.var_eta_mu * locals.var_qis_dn8))), (locals.var_e_eff0 * (locals.var_qbs_dn9 + (locals.var_eta_mu * locals.var_qis_dn9))),)
    } else {
        (locals.var_eeffs, locals.var_eeffs_dn4, locals.var_eeffs_dn6, locals.var_eeffs_dn7, locals.var_eeffs_dn8, locals.var_eeffs_dn9,)
    }
};
        locals.var_eeffs = assign43020_e56351;
        locals.var_eeffs_dn4 = assign43020_e56351_d_n4;
        locals.var_eeffs_dn6 = assign43020_e56351_d_n6;
        locals.var_eeffs_dn7 = assign43020_e56351_d_n7;
        locals.var_eeffs_dn8 = assign43020_e56351_d_n8;
        locals.var_eeffs_dn9 = assign43020_e56351_d_n9;

        let (assign43030_e56364, assign43030_e56364_d_n4, assign43030_e56364_d_n6, assign43030_e56364_d_n7, assign43030_e56364_d_n8, assign43030_e56364_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) {
        let assign43030_e56358: f64 = (locals.var_ps + locals.var_ds);
        let assign43030_e56360: f64 = (assign43030_e56358 + 1e-14);
        let assign43030_e56361: f64 = (locals.var_ps / assign43030_e56360);
        let assign43030_e56362: f64 = (assign43030_e56361).ln();
        (assign43030_e56362, ((((locals.var_ps_dn4 * assign43030_e56360) - (locals.var_ps * (locals.var_ps_dn4 + locals.var_ds_dn4))) / (assign43030_e56360 * assign43030_e56360)) / assign43030_e56361), ((((locals.var_ps_dn6 * assign43030_e56360) - (locals.var_ps * (locals.var_ps_dn6 + locals.var_ds_dn6))) / (assign43030_e56360 * assign43030_e56360)) / assign43030_e56361), ((((locals.var_ps_dn7 * assign43030_e56360) - (locals.var_ps * (locals.var_ps_dn7 + locals.var_ds_dn7))) / (assign43030_e56360 * assign43030_e56360)) / assign43030_e56361), ((((locals.var_ps_dn8 * assign43030_e56360) - (locals.var_ps * (locals.var_ps_dn8 + locals.var_ds_dn8))) / (assign43030_e56360 * assign43030_e56360)) / assign43030_e56361), ((((locals.var_ps_dn9 * assign43030_e56360) - (locals.var_ps * (locals.var_ps_dn9 + locals.var_ds_dn9))) / (assign43030_e56360 * assign43030_e56360)) / assign43030_e56361),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign43030_e56364;
        locals.var_temp1_dn4 = assign43030_e56364_d_n4;
        locals.var_temp1_dn6 = assign43030_e56364_d_n6;
        locals.var_temp1_dn7 = assign43030_e56364_d_n7;
        locals.var_temp1_dn8 = assign43030_e56364_d_n8;
        locals.var_temp1_dn9 = assign43030_e56364_d_n9;

        let (assign43040_e56383, assign43040_e56383_d_n4, assign43040_e56383_d_n6, assign43040_e56383_d_n7, assign43040_e56383_d_n8, assign43040_e56383_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) {
        let assign43040_e56370: f64 = (locals.var_eeffs * locals.var_mue_t);
        let assign43040_e56372: f64 = (assign43040_e56370).powf(locals.var_themu_t);
        let assign43040_e56376: f64 = (0.5 * locals.var_thecs_t);
        let assign43040_e56378: f64 = (assign43040_e56376 * locals.var_temp1);
        let assign43040_e56379: f64 = (assign43040_e56378).exp();
        let assign43040_e56380: f64 = (locals.var_cs_t * assign43040_e56379);
        let assign43040_e56381: f64 = (assign43040_e56372 + assign43040_e56380);
        (assign43040_e56381, (if locals.var_themu_t_dn4 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43040_e56370).powf(locals.var_themu_t - 1.0) * ((locals.var_eeffs_dn4 * locals.var_mue_t) + (locals.var_eeffs * locals.var_mue_t_dn4)))) } } else { (assign43040_e56372 * ((locals.var_themu_t_dn4 * (assign43040_e56370).ln()) + (locals.var_themu_t * (((locals.var_eeffs_dn4 * locals.var_mue_t) + (locals.var_eeffs * locals.var_mue_t_dn4)) / assign43040_e56370)))) } + ((locals.var_cs_t_dn4 * assign43040_e56379) + (locals.var_cs_t * (assign43040_e56379 * (((0.5 * locals.var_thecs_t_dn4) * locals.var_temp1) + (assign43040_e56376 * locals.var_temp1_dn4)))))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43040_e56370).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs_dn6 * locals.var_mue_t))) } } else { (assign43040_e56372 * (locals.var_themu_t * ((locals.var_eeffs_dn6 * locals.var_mue_t) / assign43040_e56370))) } + (locals.var_cs_t * (assign43040_e56379 * (assign43040_e56376 * locals.var_temp1_dn6)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43040_e56370).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs_dn7 * locals.var_mue_t))) } } else { (assign43040_e56372 * (locals.var_themu_t * ((locals.var_eeffs_dn7 * locals.var_mue_t) / assign43040_e56370))) } + (locals.var_cs_t * (assign43040_e56379 * (assign43040_e56376 * locals.var_temp1_dn7)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43040_e56370).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs_dn8 * locals.var_mue_t))) } } else { (assign43040_e56372 * (locals.var_themu_t * ((locals.var_eeffs_dn8 * locals.var_mue_t) / assign43040_e56370))) } + (locals.var_cs_t * (assign43040_e56379 * (assign43040_e56376 * locals.var_temp1_dn8)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43040_e56370).powf(locals.var_themu_t - 1.0) * (locals.var_eeffs_dn9 * locals.var_mue_t))) } } else { (assign43040_e56372 * (locals.var_themu_t * ((locals.var_eeffs_dn9 * locals.var_mue_t) / assign43040_e56370))) } + (locals.var_cs_t * (assign43040_e56379 * (assign43040_e56376 * locals.var_temp1_dn9)))),)
    } else {
        (locals.var_mutmp, locals.var_mutmp_dn4, locals.var_mutmp_dn6, locals.var_mutmp_dn7, locals.var_mutmp_dn8, locals.var_mutmp_dn9,)
    }
};
        locals.var_mutmp = assign43040_e56383;
        locals.var_mutmp_dn4 = assign43040_e56383_d_n4;
        locals.var_mutmp_dn6 = assign43040_e56383_d_n6;
        locals.var_mutmp_dn7 = assign43040_e56383_d_n7;
        locals.var_mutmp_dn8 = assign43040_e56383_d_n8;
        locals.var_mutmp_dn9 = assign43040_e56383_d_n9;

        let (assign43050_e56395, assign43050_e56395_d_n4, assign43050_e56395_d_n6, assign43050_e56395_d_n7, assign43050_e56395_d_n8, assign43050_e56395_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) {
        let assign43050_e56389: f64 = (1.0 + locals.var_mutmp);
        let assign43050_e56391: f64 = (assign43050_e56389 + locals.var_gr);
        let assign43050_e56393: f64 = (assign43050_e56391 * locals.var_rxcor);
        (assign43050_e56393, (((locals.var_mutmp_dn4 + locals.var_gr_dn4) * locals.var_rxcor) + (assign43050_e56391 * locals.var_rxcor_dn4)), (((locals.var_mutmp_dn6 + locals.var_gr_dn6) * locals.var_rxcor) + (assign43050_e56391 * locals.var_rxcor_dn6)), (((locals.var_mutmp_dn7 + locals.var_gr_dn7) * locals.var_rxcor) + (assign43050_e56391 * locals.var_rxcor_dn7)), (((locals.var_mutmp_dn8 + locals.var_gr_dn8) * locals.var_rxcor) + (assign43050_e56391 * locals.var_rxcor_dn8)), (((locals.var_mutmp_dn9 + locals.var_gr_dn9) * locals.var_rxcor) + (assign43050_e56391 * locals.var_rxcor_dn9)),)
    } else {
        (locals.var_gmobs, locals.var_gmobs_dn4, locals.var_gmobs_dn6, locals.var_gmobs_dn7, locals.var_gmobs_dn8, locals.var_gmobs_dn9,)
    }
};
        locals.var_gmobs = assign43050_e56395;
        locals.var_gmobs_dn4 = assign43050_e56395_d_n4;
        locals.var_gmobs_dn6 = assign43050_e56395_d_n6;
        locals.var_gmobs_dn7 = assign43050_e56395_d_n7;
        locals.var_gmobs_dn8 = assign43050_e56395_d_n8;
        locals.var_gmobs_dn9 = assign43050_e56395_d_n9;

        let assign43060_e56398: f64 = if locals.var_thesatb_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1212 = assign43060_e56398;

    }

    pub(super) fn stamp_transient_block_21(
        locals: &mut StampLocals,
    ) {
        let (assign43070_e56412, assign43070_e56412_d_n4, assign43070_e56412_d_n6, assign43070_e56412_d_n7, assign43070_e56412_d_n8, assign43070_e56412_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1212 != 0.0)) {
        let assign43070_e56408: f64 = (locals.var_thesatb_i * locals.var_vsbx);
        let assign43070_e56409: f64 = (1.0 - assign43070_e56408);
        let assign43070_e56410: f64 = (1.0 / assign43070_e56409);
        (assign43070_e56410, (-((-(locals.var_thesatb_i * locals.var_vsbx_dn4)) / (assign43070_e56409 * assign43070_e56409))), (-((-(locals.var_thesatb_i * locals.var_vsbx_dn6)) / (assign43070_e56409 * assign43070_e56409))), (-((-(locals.var_thesatb_i * locals.var_vsbx_dn7)) / (assign43070_e56409 * assign43070_e56409))), (-((-(locals.var_thesatb_i * locals.var_vsbx_dn8)) / (assign43070_e56409 * assign43070_e56409))), (-((-(locals.var_thesatb_i * locals.var_vsbx_dn9)) / (assign43070_e56409 * assign43070_e56409))),)
    } else {
        (locals.var_xitsb, locals.var_xitsb_dn4, locals.var_xitsb_dn6, locals.var_xitsb_dn7, locals.var_xitsb_dn8, locals.var_xitsb_dn9,)
    }
};
        locals.var_xitsb = assign43070_e56412;
        locals.var_xitsb_dn4 = assign43070_e56412_d_n4;
        locals.var_xitsb_dn6 = assign43070_e56412_d_n6;
        locals.var_xitsb_dn7 = assign43070_e56412_d_n7;
        locals.var_xitsb_dn8 = assign43070_e56412_d_n8;
        locals.var_xitsb_dn9 = assign43070_e56412_d_n9;

        let (assign43080_e56425, assign43080_e56425_d_n4, assign43080_e56425_d_n6, assign43080_e56425_d_n7, assign43080_e56425_d_n8, assign43080_e56425_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1212 == 0.0)) {
        let assign43080_e56422: f64 = (locals.var_thesatb_i * locals.var_vsbx);
        let assign43080_e56423: f64 = (1.0 + assign43080_e56422);
        (assign43080_e56423, (locals.var_thesatb_i * locals.var_vsbx_dn4), (locals.var_thesatb_i * locals.var_vsbx_dn6), (locals.var_thesatb_i * locals.var_vsbx_dn7), (locals.var_thesatb_i * locals.var_vsbx_dn8), (locals.var_thesatb_i * locals.var_vsbx_dn9),)
    } else {
        (locals.var_xitsb, locals.var_xitsb_dn4, locals.var_xitsb_dn6, locals.var_xitsb_dn7, locals.var_xitsb_dn8, locals.var_xitsb_dn9,)
    }
};
        locals.var_xitsb = assign43080_e56425;
        locals.var_xitsb_dn4 = assign43080_e56425_d_n4;
        locals.var_xitsb_dn6 = assign43080_e56425_d_n6;
        locals.var_xitsb_dn7 = assign43080_e56425_d_n7;
        locals.var_xitsb_dn8 = assign43080_e56425_d_n8;
        locals.var_xitsb_dn9 = assign43080_e56425_d_n9;

        let (assign43090_e56433, assign43090_e56433_d_n4, assign43090_e56433_d_n6, assign43090_e56433_d_n7, assign43090_e56433_d_n8, assign43090_e56433_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) {
        let assign43090_e56431: f64 = (locals.var_qis * locals.var_xitsb);
        (assign43090_e56431, ((locals.var_qis_dn4 * locals.var_xitsb) + (locals.var_qis * locals.var_xitsb_dn4)), ((locals.var_qis_dn6 * locals.var_xitsb) + (locals.var_qis * locals.var_xitsb_dn6)), ((locals.var_qis_dn7 * locals.var_xitsb) + (locals.var_qis * locals.var_xitsb_dn7)), ((locals.var_qis_dn8 * locals.var_xitsb) + (locals.var_qis * locals.var_xitsb_dn8)), ((locals.var_qis_dn9 * locals.var_xitsb) + (locals.var_qis * locals.var_xitsb_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign43090_e56433;
        locals.var_temp2_dn4 = assign43090_e56433_d_n4;
        locals.var_temp2_dn6 = assign43090_e56433_d_n6;
        locals.var_temp2_dn7 = assign43090_e56433_d_n7;
        locals.var_temp2_dn8 = assign43090_e56433_d_n8;
        locals.var_temp2_dn9 = assign43090_e56433_d_n9;

        let (assign43100_e56443, assign43100_e56443_d_n4, assign43100_e56443_d_n6, assign43100_e56443_d_n7, assign43100_e56443_d_n8, assign43100_e56443_d_n9,) = {
    if ((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) {
        let assign43100_e56440: f64 = (locals.var_thesatt_i + locals.var_temp2);
        let assign43100_e56441: f64 = (locals.var_temp2 / assign43100_e56440);
        (assign43100_e56441, (((locals.var_temp2_dn4 * assign43100_e56440) - (locals.var_temp2 * locals.var_temp2_dn4)) / (assign43100_e56440 * assign43100_e56440)), (((locals.var_temp2_dn6 * assign43100_e56440) - (locals.var_temp2 * locals.var_temp2_dn6)) / (assign43100_e56440 * assign43100_e56440)), (((locals.var_temp2_dn7 * assign43100_e56440) - (locals.var_temp2 * locals.var_temp2_dn7)) / (assign43100_e56440 * assign43100_e56440)), (((locals.var_temp2_dn8 * assign43100_e56440) - (locals.var_temp2 * locals.var_temp2_dn8)) / (assign43100_e56440 * assign43100_e56440)), (((locals.var_temp2_dn9 * assign43100_e56440) - (locals.var_temp2 * locals.var_temp2_dn9)) / (assign43100_e56440 * assign43100_e56440)),)
    } else {
        (locals.var_wsat, locals.var_wsat_dn4, locals.var_wsat_dn6, locals.var_wsat_dn7, locals.var_wsat_dn8, locals.var_wsat_dn9,)
    }
};
        locals.var_wsat = assign43100_e56443;
        locals.var_wsat_dn4 = assign43100_e56443_d_n4;
        locals.var_wsat_dn6 = assign43100_e56443_d_n6;
        locals.var_wsat_dn7 = assign43100_e56443_d_n7;
        locals.var_wsat_dn8 = assign43100_e56443_d_n8;
        locals.var_wsat_dn9 = assign43100_e56443_d_n9;

        let assign43110_e56446: f64 = if locals.var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1213 = assign43110_e56446;

        let (assign43120_e56460, assign43120_e56460_d_n4, assign43120_e56460_d_n6, assign43120_e56460_d_n7, assign43120_e56460_d_n8, assign43120_e56460_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1213 != 0.0)) {
        let assign43120_e56456: f64 = (locals.var_thesatg_i * locals.var_wsat);
        let assign43120_e56457: f64 = (1.0 - assign43120_e56456);
        let assign43120_e56458: f64 = (1.0 / assign43120_e56457);
        (assign43120_e56458, (-((-(locals.var_thesatg_i * locals.var_wsat_dn4)) / (assign43120_e56457 * assign43120_e56457))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn6)) / (assign43120_e56457 * assign43120_e56457))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn7)) / (assign43120_e56457 * assign43120_e56457))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn8)) / (assign43120_e56457 * assign43120_e56457))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn9)) / (assign43120_e56457 * assign43120_e56457))),)
    } else {
        (locals.var_factheta, locals.var_factheta_dn4, locals.var_factheta_dn6, locals.var_factheta_dn7, locals.var_factheta_dn8, locals.var_factheta_dn9,)
    }
};
        locals.var_factheta = assign43120_e56460;
        locals.var_factheta_dn4 = assign43120_e56460_d_n4;
        locals.var_factheta_dn6 = assign43120_e56460_d_n6;
        locals.var_factheta_dn7 = assign43120_e56460_d_n7;
        locals.var_factheta_dn8 = assign43120_e56460_d_n8;
        locals.var_factheta_dn9 = assign43120_e56460_d_n9;

        let (assign43130_e56473, assign43130_e56473_d_n4, assign43130_e56473_d_n6, assign43130_e56473_d_n7, assign43130_e56473_d_n8, assign43130_e56473_d_n9,) = {
    if (((locals.var_guard1205 != 0.0) && (locals.var_guard1209 != 0.0)) && (locals.var_guard1213 == 0.0)) {
        let assign43130_e56470: f64 = (locals.var_thesatg_i * locals.var_wsat);
        let assign43130_e56471: f64 = (1.0 + assign43130_e56470);
        (assign43130_e56471, (locals.var_thesatg_i * locals.var_wsat_dn4), (locals.var_thesatg_i * locals.var_wsat_dn6), (locals.var_thesatg_i * locals.var_wsat_dn7), (locals.var_thesatg_i * locals.var_wsat_dn8), (locals.var_thesatg_i * locals.var_wsat_dn9),)
    } else {
        (locals.var_factheta, locals.var_factheta_dn4, locals.var_factheta_dn6, locals.var_factheta_dn7, locals.var_factheta_dn8, locals.var_factheta_dn9,)
    }
};
        locals.var_factheta = assign43130_e56473;
        locals.var_factheta_dn4 = assign43130_e56473_d_n4;
        locals.var_factheta_dn6 = assign43130_e56473_d_n6;
        locals.var_factheta_dn7 = assign43130_e56473_d_n7;
        locals.var_factheta_dn8 = assign43130_e56473_d_n8;
        locals.var_factheta_dn9 = assign43130_e56473_d_n9;

        locals.var_vgb1_dc = locals.var_vgb1;
        locals.var_vgb1_dc_dn4 = locals.var_vgb1_dn4;
        locals.var_vgb1_dc_dn6 = locals.var_vgb1_dn6;
        locals.var_vgb1_dc_dn7 = locals.var_vgb1_dn7;
        locals.var_vgb1_dc_dn8 = locals.var_vgb1_dn8;
        locals.var_vgb1_dc_dn9 = locals.var_vgb1_dn9;

        locals.var_vsbx_dc = locals.var_vsbx;
        locals.var_vsbx_dc_dn4 = locals.var_vsbx_dn4;
        locals.var_vsbx_dc_dn6 = locals.var_vsbx_dn6;
        locals.var_vsbx_dc_dn7 = locals.var_vsbx_dn7;
        locals.var_vsbx_dc_dn8 = locals.var_vsbx_dn8;
        locals.var_vsbx_dc_dn9 = locals.var_vsbx_dn9;

        locals.var_phit1_dc = locals.var_phit1;
        locals.var_phit1_dc_dn4 = locals.var_phit1_dn4;
        locals.var_phit1_dc_dn6 = locals.var_phit1_dn6;
        locals.var_phit1_dc_dn7 = locals.var_phit1_dn7;
        locals.var_phit1_dc_dn8 = locals.var_phit1_dn8;
        locals.var_phit1_dc_dn9 = locals.var_phit1_dn9;

        locals.var_inv_phit1_dc = locals.var_inv_phit1;
        locals.var_inv_phit1_dc_dn4 = locals.var_inv_phit1_dn4;
        locals.var_inv_phit1_dc_dn6 = locals.var_inv_phit1_dn6;
        locals.var_inv_phit1_dc_dn7 = locals.var_inv_phit1_dn7;
        locals.var_inv_phit1_dc_dn8 = locals.var_inv_phit1_dn8;
        locals.var_inv_phit1_dc_dn9 = locals.var_inv_phit1_dn9;

        locals.var_gf_dc = locals.var_gf;
        locals.var_gf_dc_dn4 = locals.var_gf_dn4;
        locals.var_gf_dc_dn6 = locals.var_gf_dn6;
        locals.var_gf_dc_dn7 = locals.var_gf_dn7;
        locals.var_gf_dc_dn8 = locals.var_gf_dn8;
        locals.var_gf_dc_dn9 = locals.var_gf_dn9;

        locals.var_gf2_dc = locals.var_gf2;
        locals.var_gf2_dc_dn4 = locals.var_gf2_dn4;
        locals.var_gf2_dc_dn6 = locals.var_gf2_dn6;
        locals.var_gf2_dc_dn7 = locals.var_gf2_dn7;
        locals.var_gf2_dc_dn8 = locals.var_gf2_dn8;
        locals.var_gf2_dc_dn9 = locals.var_gf2_dn9;

        locals.var_inv_gf2_dc = locals.var_inv_gf2;
        locals.var_inv_gf2_dc_dn4 = locals.var_inv_gf2_dn4;
        locals.var_inv_gf2_dc_dn6 = locals.var_inv_gf2_dn6;
        locals.var_inv_gf2_dc_dn7 = locals.var_inv_gf2_dn7;
        locals.var_inv_gf2_dc_dn8 = locals.var_inv_gf2_dn8;
        locals.var_inv_gf2_dc_dn9 = locals.var_inv_gf2_dn9;

        locals.var_xg_dc = locals.var_xg;
        locals.var_xg_dc_dn4 = locals.var_xg_dn4;
        locals.var_xg_dc_dn6 = locals.var_xg_dn6;
        locals.var_xg_dc_dn7 = locals.var_xg_dn7;
        locals.var_xg_dc_dn8 = locals.var_xg_dn8;
        locals.var_xg_dc_dn9 = locals.var_xg_dn9;

        locals.var_xno_s_dc = locals.var_xno_s;
        locals.var_xno_s_dc_dn4 = locals.var_xno_s_dn4;
        locals.var_xno_s_dc_dn6 = locals.var_xno_s_dn6;
        locals.var_xno_s_dc_dn7 = locals.var_xno_s_dn7;
        locals.var_xno_s_dc_dn8 = locals.var_xno_s_dn8;
        locals.var_xno_s_dc_dn9 = locals.var_xno_s_dn9;

        locals.var_xn_s_dc = locals.var_xn_s;
        locals.var_xn_s_dc_dn4 = locals.var_xn_s_dn4;
        locals.var_xn_s_dc_dn6 = locals.var_xn_s_dn6;
        locals.var_xn_s_dc_dn7 = locals.var_xn_s_dn7;
        locals.var_xn_s_dc_dn8 = locals.var_xn_s_dn8;
        locals.var_xn_s_dc_dn9 = locals.var_xn_s_dn9;

        locals.var_xi_dc = locals.var_xi;
        locals.var_xi_dc_dn4 = locals.var_xi_dn4;
        locals.var_xi_dc_dn6 = locals.var_xi_dn6;
        locals.var_xi_dc_dn7 = locals.var_xi_dn7;
        locals.var_xi_dc_dn8 = locals.var_xi_dn8;
        locals.var_xi_dc_dn9 = locals.var_xi_dn9;

        locals.var_margin_dc = locals.var_margin;

        locals.var_inv_xi_dc = locals.var_inv_xi;
        locals.var_inv_xi_dc_dn4 = locals.var_inv_xi_dn4;
        locals.var_inv_xi_dc_dn6 = locals.var_inv_xi_dn6;
        locals.var_inv_xi_dc_dn7 = locals.var_inv_xi_dn7;
        locals.var_inv_xi_dc_dn8 = locals.var_inv_xi_dn8;
        locals.var_inv_xi_dc_dn9 = locals.var_inv_xi_dn9;

        locals.var_sp_s_x1_dc = locals.var_sp_s_x1;
        locals.var_sp_s_x1_dc_dn4 = locals.var_sp_s_x1_dn4;
        locals.var_sp_s_x1_dc_dn6 = locals.var_sp_s_x1_dn6;
        locals.var_sp_s_x1_dc_dn7 = locals.var_sp_s_x1_dn7;
        locals.var_sp_s_x1_dc_dn8 = locals.var_sp_s_x1_dn8;
        locals.var_sp_s_x1_dc_dn9 = locals.var_sp_s_x1_dn9;

        locals.var_delta_ns_dc = locals.var_delta_ns;
        locals.var_delta_ns_dc_dn4 = locals.var_delta_ns_dn4;
        locals.var_delta_ns_dc_dn6 = locals.var_delta_ns_dn6;
        locals.var_delta_ns_dc_dn7 = locals.var_delta_ns_dn7;
        locals.var_delta_ns_dc_dn8 = locals.var_delta_ns_dn8;
        locals.var_delta_ns_dc_dn9 = locals.var_delta_ns_dn9;

        locals.var_x_s_dc = locals.var_x_s;
        locals.var_x_s_dc_dn4 = locals.var_x_s_dn4;
        locals.var_x_s_dc_dn6 = locals.var_x_s_dn6;
        locals.var_x_s_dc_dn7 = locals.var_x_s_dn7;
        locals.var_x_s_dc_dn8 = locals.var_x_s_dn8;
        locals.var_x_s_dc_dn9 = locals.var_x_s_dn9;

        locals.var_xi1s_dc = locals.var_xi1s;
        locals.var_xi1s_dc_dn4 = locals.var_xi1s_dn4;
        locals.var_xi1s_dc_dn6 = locals.var_xi1s_dn6;
        locals.var_xi1s_dc_dn7 = locals.var_xi1s_dn7;
        locals.var_xi1s_dc_dn8 = locals.var_xi1s_dn8;
        locals.var_xi1s_dc_dn9 = locals.var_xi1s_dn9;

        locals.var_xi2s_dc = locals.var_xi2s;
        locals.var_xi2s_dc_dn4 = locals.var_xi2s_dn4;
        locals.var_xi2s_dc_dn6 = locals.var_xi2s_dn6;
        locals.var_xi2s_dc_dn7 = locals.var_xi2s_dn7;
        locals.var_xi2s_dc_dn8 = locals.var_xi2s_dn8;
        locals.var_xi2s_dc_dn9 = locals.var_xi2s_dn9;

        locals.var_delta_1s_dc = locals.var_delta_1s;
        locals.var_delta_1s_dc_dn4 = locals.var_delta_1s_dn4;
        locals.var_delta_1s_dc_dn6 = locals.var_delta_1s_dn6;
        locals.var_delta_1s_dc_dn7 = locals.var_delta_1s_dn7;
        locals.var_delta_1s_dc_dn8 = locals.var_delta_1s_dn8;
        locals.var_delta_1s_dc_dn9 = locals.var_delta_1s_dn9;

        locals.var_es_dc = locals.var_es;
        locals.var_es_dc_dn4 = locals.var_es_dn4;
        locals.var_es_dc_dn6 = locals.var_es_dn6;
        locals.var_es_dc_dn7 = locals.var_es_dn7;
        locals.var_es_dc_dn8 = locals.var_es_dn8;
        locals.var_es_dc_dn9 = locals.var_es_dn9;

        locals.var_ps_dc = locals.var_ps;
        locals.var_ps_dc_dn4 = locals.var_ps_dn4;
        locals.var_ps_dc_dn6 = locals.var_ps_dn6;
        locals.var_ps_dc_dn7 = locals.var_ps_dn7;
        locals.var_ps_dc_dn8 = locals.var_ps_dn8;
        locals.var_ps_dc_dn9 = locals.var_ps_dn9;

        locals.var_ds_dc = locals.var_ds;
        locals.var_ds_dc_dn4 = locals.var_ds_dn4;
        locals.var_ds_dc_dn6 = locals.var_ds_dn6;
        locals.var_ds_dc_dn7 = locals.var_ds_dn7;
        locals.var_ds_dc_dn8 = locals.var_ds_dn8;
        locals.var_ds_dc_dn9 = locals.var_ds_dn9;

        locals.var_sqs_dc = locals.var_sqs;
        locals.var_sqs_dc_dn4 = locals.var_sqs_dn4;
        locals.var_sqs_dc_dn6 = locals.var_sqs_dn6;
        locals.var_sqs_dc_dn7 = locals.var_sqs_dn7;
        locals.var_sqs_dc_dn8 = locals.var_sqs_dn8;
        locals.var_sqs_dc_dn9 = locals.var_sqs_dn9;

        locals.var_alphas_dc = locals.var_alphas;
        locals.var_alphas_dc_dn4 = locals.var_alphas_dn4;
        locals.var_alphas_dc_dn6 = locals.var_alphas_dn6;
        locals.var_alphas_dc_dn7 = locals.var_alphas_dn7;
        locals.var_alphas_dc_dn8 = locals.var_alphas_dn8;
        locals.var_alphas_dc_dn9 = locals.var_alphas_dn9;

        locals.var_rxcor_dc = locals.var_rxcor;
        locals.var_rxcor_dc_dn4 = locals.var_rxcor_dn4;
        locals.var_rxcor_dc_dn6 = locals.var_rxcor_dn6;
        locals.var_rxcor_dc_dn7 = locals.var_rxcor_dn7;
        locals.var_rxcor_dc_dn8 = locals.var_rxcor_dn8;
        locals.var_rxcor_dc_dn9 = locals.var_rxcor_dn9;

        locals.var_xgs_dc = locals.var_xgs;
        locals.var_xgs_dc_dn4 = locals.var_xgs_dn4;
        locals.var_xgs_dc_dn6 = locals.var_xgs_dn6;
        locals.var_xgs_dc_dn7 = locals.var_xgs_dn7;
        locals.var_xgs_dc_dn8 = locals.var_xgs_dn8;
        locals.var_xgs_dc_dn9 = locals.var_xgs_dn9;

        locals.var_qis_dc = locals.var_qis;
        locals.var_qis_dc_dn4 = locals.var_qis_dn4;
        locals.var_qis_dc_dn6 = locals.var_qis_dn6;
        locals.var_qis_dc_dn7 = locals.var_qis_dn7;
        locals.var_qis_dc_dn8 = locals.var_qis_dn8;
        locals.var_qis_dc_dn9 = locals.var_qis_dn9;

        locals.var_qbs_dc = locals.var_qbs;
        locals.var_qbs_dc_dn4 = locals.var_qbs_dn4;
        locals.var_qbs_dc_dn6 = locals.var_qbs_dn6;
        locals.var_qbs_dc_dn7 = locals.var_qbs_dn7;
        locals.var_qbs_dc_dn8 = locals.var_qbs_dn8;
        locals.var_qbs_dc_dn9 = locals.var_qbs_dn9;

        locals.var_rhob_dc = locals.var_rhob;
        locals.var_rhob_dc_dn4 = locals.var_rhob_dn4;
        locals.var_rhob_dc_dn6 = locals.var_rhob_dn6;
        locals.var_rhob_dc_dn7 = locals.var_rhob_dn7;
        locals.var_rhob_dc_dn8 = locals.var_rhob_dn8;
        locals.var_rhob_dc_dn9 = locals.var_rhob_dn9;

        locals.var_rhog_dc = locals.var_rhog;
        locals.var_rhog_dc_dn4 = locals.var_rhog_dn4;
        locals.var_rhog_dc_dn6 = locals.var_rhog_dn6;
        locals.var_rhog_dc_dn7 = locals.var_rhog_dn7;
        locals.var_rhog_dc_dn8 = locals.var_rhog_dn8;
        locals.var_rhog_dc_dn9 = locals.var_rhog_dn9;

        locals.var_gmobs_dc = locals.var_gmobs;
        locals.var_gmobs_dc_dn4 = locals.var_gmobs_dn4;
        locals.var_gmobs_dc_dn6 = locals.var_gmobs_dn6;
        locals.var_gmobs_dc_dn7 = locals.var_gmobs_dn7;
        locals.var_gmobs_dc_dn8 = locals.var_gmobs_dn8;
        locals.var_gmobs_dc_dn9 = locals.var_gmobs_dn9;

        locals.var_xitsb_dc = locals.var_xitsb;
        locals.var_xitsb_dc_dn4 = locals.var_xitsb_dn4;
        locals.var_xitsb_dc_dn6 = locals.var_xitsb_dn6;
        locals.var_xitsb_dc_dn7 = locals.var_xitsb_dn7;
        locals.var_xitsb_dc_dn8 = locals.var_xitsb_dn8;
        locals.var_xitsb_dc_dn9 = locals.var_xitsb_dn9;

        locals.var_factheta_dc = locals.var_factheta;
        locals.var_factheta_dc_dn4 = locals.var_factheta_dn4;
        locals.var_factheta_dc_dn6 = locals.var_factheta_dn6;
        locals.var_factheta_dc_dn7 = locals.var_factheta_dn7;
        locals.var_factheta_dc_dn8 = locals.var_factheta_dn8;
        locals.var_factheta_dc_dn9 = locals.var_factheta_dn9;

        locals.var_thesat1 = 0.0;
        locals.var_thesat1_dn4 = 0.0;
        locals.var_thesat1_dn6 = 0.0;
        locals.var_thesat1_dn7 = 0.0;
        locals.var_thesat1_dn8 = 0.0;
        locals.var_thesat1_dn9 = 0.0;

        let assign43480_e56510: f64 = (locals.var_phit1 * 4.60517018598809);
        locals.var_vdsat_lim = assign43480_e56510;
        locals.var_vdsat_lim_dn4 = (locals.var_phit1_dn4 * 4.60517018598809);
        locals.var_vdsat_lim_dn6 = (locals.var_phit1_dn6 * 4.60517018598809);
        locals.var_vdsat_lim_dn7 = (locals.var_phit1_dn7 * 4.60517018598809);
        locals.var_vdsat_lim_dn8 = (locals.var_phit1_dn8 * 4.60517018598809);
        locals.var_vdsat_lim_dn9 = (locals.var_phit1_dn9 * 4.60517018598809);

        locals.var_v_dsat = locals.var_vdsat_lim;
        locals.var_v_dsat_dn4 = locals.var_vdsat_lim_dn4;
        locals.var_v_dsat_dn6 = locals.var_vdsat_lim_dn6;
        locals.var_v_dsat_dn7 = locals.var_vdsat_lim_dn7;
        locals.var_v_dsat_dn8 = locals.var_vdsat_lim_dn8;
        locals.var_v_dsat_dn9 = locals.var_vdsat_lim_dn9;

        locals.var_vdse = locals.var_v_ds;
        locals.var_vdse_dn4 = 0.0;
        locals.var_vdse_dn6 = 0.0;
        locals.var_vdse_dn7 = locals.var_v_ds_dn7;
        locals.var_vdse_dn8 = locals.var_v_ds_dn8;
        locals.var_vdse_dn9 = 0.0;

        let assign43510_e56515: f64 = (locals.var_v_ds * locals.var_inv_phit1);
        locals.var_udse = assign43510_e56515;
        locals.var_udse_dn4 = (locals.var_v_ds * locals.var_inv_phit1_dn4);
        locals.var_udse_dn6 = (locals.var_v_ds * locals.var_inv_phit1_dn6);
        locals.var_udse_dn7 = ((locals.var_v_ds_dn7 * locals.var_inv_phit1) + (locals.var_v_ds * locals.var_inv_phit1_dn7));
        locals.var_udse_dn8 = ((locals.var_v_ds_dn8 * locals.var_inv_phit1) + (locals.var_v_ds * locals.var_inv_phit1_dn8));
        locals.var_udse_dn9 = (locals.var_v_ds * locals.var_inv_phit1_dn9);

        locals.var_x_d = locals.var_x_s;
        locals.var_x_d_dn4 = locals.var_x_s_dn4;
        locals.var_x_d_dn6 = locals.var_x_s_dn6;
        locals.var_x_d_dn7 = locals.var_x_s_dn7;
        locals.var_x_d_dn8 = locals.var_x_s_dn8;
        locals.var_x_d_dn9 = locals.var_x_s_dn9;

        locals.var_x_ds = 0.0;
        locals.var_x_ds_dn4 = 0.0;
        locals.var_x_ds_dn6 = 0.0;
        locals.var_x_ds_dn7 = 0.0;
        locals.var_x_ds_dn8 = 0.0;
        locals.var_x_ds_dn9 = 0.0;

        locals.var_dps = 0.0;
        locals.var_dps_dn4 = 0.0;
        locals.var_dps_dn6 = 0.0;
        locals.var_dps_dn7 = 0.0;
        locals.var_dps_dn8 = 0.0;
        locals.var_dps_dn9 = 0.0;

        locals.var_ed = locals.var_es;
        locals.var_ed_dn4 = locals.var_es_dn4;
        locals.var_ed_dn6 = locals.var_es_dn6;
        locals.var_ed_dn7 = locals.var_es_dn7;
        locals.var_ed_dn8 = locals.var_es_dn8;
        locals.var_ed_dn9 = locals.var_es_dn9;

        locals.var_pd = locals.var_ps;
        locals.var_pd_dn4 = locals.var_ps_dn4;
        locals.var_pd_dn6 = locals.var_ps_dn6;
        locals.var_pd_dn7 = locals.var_ps_dn7;
        locals.var_pd_dn8 = locals.var_ps_dn8;
        locals.var_pd_dn9 = locals.var_ps_dn9;

        locals.var_dd = locals.var_ds;
        locals.var_dd_dn4 = locals.var_ds_dn4;
        locals.var_dd_dn6 = locals.var_ds_dn6;
        locals.var_dd_dn7 = locals.var_ds_dn7;
        locals.var_dd_dn8 = locals.var_ds_dn8;
        locals.var_dd_dn9 = locals.var_ds_dn9;

        locals.var_qbd = locals.var_qbs;
        locals.var_qbd_dn4 = locals.var_qbs_dn4;
        locals.var_qbd_dn6 = locals.var_qbs_dn6;
        locals.var_qbd_dn7 = locals.var_qbs_dn7;
        locals.var_qbd_dn8 = locals.var_qbs_dn8;
        locals.var_qbd_dn9 = locals.var_qbs_dn9;

        locals.var_x_m = locals.var_x_s;
        locals.var_x_m_dn4 = locals.var_x_s_dn4;
        locals.var_x_m_dn6 = locals.var_x_s_dn6;
        locals.var_x_m_dn7 = locals.var_x_s_dn7;
        locals.var_x_m_dn8 = locals.var_x_s_dn8;
        locals.var_x_m_dn9 = locals.var_x_s_dn9;

        locals.var_em = locals.var_es;
        locals.var_em_dn4 = locals.var_es_dn4;
        locals.var_em_dn6 = locals.var_es_dn6;
        locals.var_em_dn7 = locals.var_es_dn7;
        locals.var_em_dn8 = locals.var_es_dn8;
        locals.var_em_dn9 = locals.var_es_dn9;

        locals.var_dm = locals.var_ds;
        locals.var_dm_dn4 = locals.var_ds_dn4;
        locals.var_dm_dn6 = locals.var_ds_dn6;
        locals.var_dm_dn7 = locals.var_ds_dn7;
        locals.var_dm_dn8 = locals.var_ds_dn8;
        locals.var_dm_dn9 = locals.var_ds_dn9;

        locals.var_pm = locals.var_ps;
        locals.var_pm_dn4 = locals.var_ps_dn4;
        locals.var_pm_dn6 = locals.var_ps_dn6;
        locals.var_pm_dn7 = locals.var_ps_dn7;
        locals.var_pm_dn8 = locals.var_ps_dn8;
        locals.var_pm_dn9 = locals.var_ps_dn9;

        let assign43630_e56529: f64 = (locals.var_xg - locals.var_x_s);
        locals.var_xgm = assign43630_e56529;
        locals.var_xgm_dn4 = (locals.var_xg_dn4 - locals.var_x_s_dn4);
        locals.var_xgm_dn6 = (locals.var_xg_dn6 - locals.var_x_s_dn6);
        locals.var_xgm_dn7 = (locals.var_xg_dn7 - locals.var_x_s_dn7);
        locals.var_xgm_dn8 = (locals.var_xg_dn8 - locals.var_x_s_dn8);
        locals.var_xgm_dn9 = (locals.var_xg_dn9 - locals.var_x_s_dn9);

        locals.var_eta_p = 1.0;
        locals.var_eta_p_dn4 = 0.0;
        locals.var_eta_p_dn6 = 0.0;
        locals.var_eta_p_dn7 = 0.0;
        locals.var_eta_p_dn8 = 0.0;
        locals.var_eta_p_dn9 = 0.0;

        locals.var_alpha = 1.0;
        locals.var_alpha_dn4 = 0.0;
        locals.var_alpha_dn6 = 0.0;
        locals.var_alpha_dn7 = 0.0;
        locals.var_alpha_dn8 = 0.0;
        locals.var_alpha_dn9 = 0.0;

        locals.var_sqm = 0.0;
        locals.var_sqm_dn4 = 0.0;
        locals.var_sqm_dn6 = 0.0;
        locals.var_sqm_dn7 = 0.0;
        locals.var_sqm_dn8 = 0.0;
        locals.var_sqm_dn9 = 0.0;

        locals.var_qim = locals.var_qis;
        locals.var_qim_dn4 = locals.var_qis_dn4;
        locals.var_qim_dn6 = locals.var_qis_dn6;
        locals.var_qim_dn7 = locals.var_qis_dn7;
        locals.var_qim_dn8 = locals.var_qis_dn8;
        locals.var_qim_dn9 = locals.var_qis_dn9;

        let assign43680_e56536: f64 = (locals.var_xgm * locals.var_phit1);
        locals.var_qeff1 = assign43680_e56536;
        locals.var_qeff1_dn4 = ((locals.var_xgm_dn4 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn4));
        locals.var_qeff1_dn6 = ((locals.var_xgm_dn6 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn6));
        locals.var_qeff1_dn7 = ((locals.var_xgm_dn7 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn7));
        locals.var_qeff1_dn8 = ((locals.var_xgm_dn8 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn8));
        locals.var_qeff1_dn9 = ((locals.var_xgm_dn9 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn9));

        locals.var_qim1 = 0.0;
        locals.var_qim1_dn4 = 0.0;
        locals.var_qim1_dn6 = 0.0;
        locals.var_qim1_dn7 = 0.0;
        locals.var_qim1_dn8 = 0.0;
        locals.var_qim1_dn9 = 0.0;

        locals.var_qbm = locals.var_qbs;
        locals.var_qbm_dn4 = locals.var_qbs_dn4;
        locals.var_qbm_dn6 = locals.var_qbs_dn6;
        locals.var_qbm_dn7 = locals.var_qbs_dn7;
        locals.var_qbm_dn8 = locals.var_qbs_dn8;
        locals.var_qbm_dn9 = locals.var_qbs_dn9;

        locals.var_s1 = 0.0;
        locals.var_s1_dn4 = 0.0;
        locals.var_s1_dn6 = 0.0;
        locals.var_s1_dn7 = 0.0;
        locals.var_s1_dn8 = 0.0;
        locals.var_s1_dn9 = 0.0;

        locals.var_gmob = 1.0;
        locals.var_gmob_dn4 = 0.0;
        locals.var_gmob_dn6 = 0.0;
        locals.var_gmob_dn7 = 0.0;
        locals.var_gmob_dn8 = 0.0;
        locals.var_gmob_dn9 = 0.0;

    }

    pub(super) fn stamp_transient_block_22(
        locals: &mut StampLocals,
    ) {
        locals.var_thesateff = locals.var_thesatloc;
        locals.var_thesateff_dn4 = locals.var_thesatloc_dn4;
        locals.var_thesateff_dn6 = 0.0;
        locals.var_thesateff_dn7 = 0.0;
        locals.var_thesateff_dn8 = 0.0;
        locals.var_thesateff_dn9 = 0.0;

        locals.var_voxm = locals.var_qeff1;
        locals.var_voxm_dn4 = locals.var_qeff1_dn4;
        locals.var_voxm_dn6 = locals.var_qeff1_dn6;
        locals.var_voxm_dn7 = locals.var_qeff1_dn7;
        locals.var_voxm_dn8 = locals.var_qeff1_dn8;
        locals.var_voxm_dn9 = locals.var_qeff1_dn9;

        let assign43750_e56545: f64 = if locals.var_xg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1214 = assign43750_e56545;

        let assign43760_e56548: f64 = if locals.var_ds > 1e-100 { 1.0 } else { 0.0 };
        locals.var_guard1215 = assign43760_e56548;

        let (assign43770_e56556, assign43770_e56556_d_n4, assign43770_e56556_d_n6, assign43770_e56556_d_n7, assign43770_e56556_d_n8, assign43770_e56556_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign43770_e56554: f64 = (locals.var_thesatloc * locals.var_factheta);
        (assign43770_e56554, ((locals.var_thesatloc_dn4 * locals.var_factheta) + (locals.var_thesatloc * locals.var_factheta_dn4)), (locals.var_thesatloc * locals.var_factheta_dn6), (locals.var_thesatloc * locals.var_factheta_dn7), (locals.var_thesatloc * locals.var_factheta_dn8), (locals.var_thesatloc * locals.var_factheta_dn9),)
    } else {
        (locals.var_thesateff, locals.var_thesateff_dn4, locals.var_thesateff_dn6, locals.var_thesateff_dn7, locals.var_thesateff_dn8, locals.var_thesateff_dn9,)
    }
};
        locals.var_thesateff = assign43770_e56556;
        locals.var_thesateff_dn4 = assign43770_e56556_d_n4;
        locals.var_thesateff_dn6 = assign43770_e56556_d_n6;
        locals.var_thesateff_dn7 = assign43770_e56556_d_n7;
        locals.var_thesateff_dn8 = assign43770_e56556_d_n8;
        locals.var_thesateff_dn9 = assign43770_e56556_d_n9;

        let (assign43780_e56564, assign43780_e56564_d_n4, assign43780_e56564_d_n6, assign43780_e56564_d_n7, assign43780_e56564_d_n8, assign43780_e56564_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign43780_e56562: f64 = (locals.var_thesateff / locals.var_gmobs);
        (assign43780_e56562, (((locals.var_thesateff_dn4 * locals.var_gmobs) - (locals.var_thesateff * locals.var_gmobs_dn4)) / (locals.var_gmobs * locals.var_gmobs)), (((locals.var_thesateff_dn6 * locals.var_gmobs) - (locals.var_thesateff * locals.var_gmobs_dn6)) / (locals.var_gmobs * locals.var_gmobs)), (((locals.var_thesateff_dn7 * locals.var_gmobs) - (locals.var_thesateff * locals.var_gmobs_dn7)) / (locals.var_gmobs * locals.var_gmobs)), (((locals.var_thesateff_dn8 * locals.var_gmobs) - (locals.var_thesateff * locals.var_gmobs_dn8)) / (locals.var_gmobs * locals.var_gmobs)), (((locals.var_thesateff_dn9 * locals.var_gmobs) - (locals.var_thesateff * locals.var_gmobs_dn9)) / (locals.var_gmobs * locals.var_gmobs)),)
    } else {
        (locals.var_thesat1, locals.var_thesat1_dn4, locals.var_thesat1_dn6, locals.var_thesat1_dn7, locals.var_thesat1_dn8, locals.var_thesat1_dn9,)
    }
};
        locals.var_thesat1 = assign43780_e56564;
        locals.var_thesat1_dn4 = assign43780_e56564_d_n4;
        locals.var_thesat1_dn6 = assign43780_e56564_d_n6;
        locals.var_thesat1_dn7 = assign43780_e56564_d_n7;
        locals.var_thesat1_dn8 = assign43780_e56564_d_n8;
        locals.var_thesat1_dn9 = assign43780_e56564_d_n9;

        let (assign43790_e56574, assign43790_e56574_d_n4, assign43790_e56574_d_n6, assign43790_e56574_d_n7, assign43790_e56574_d_n8, assign43790_e56574_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign43790_e56571: f64 = (0.5 * locals.var_gf2);
        let assign43790_e56572: f64 = (locals.var_xgs + assign43790_e56571);
        (assign43790_e56572, (locals.var_xgs_dn4 + (0.5 * locals.var_gf2_dn4)), (locals.var_xgs_dn6 + (0.5 * locals.var_gf2_dn6)), (locals.var_xgs_dn7 + (0.5 * locals.var_gf2_dn7)), (locals.var_xgs_dn8 + (0.5 * locals.var_gf2_dn8)), (locals.var_xgs_dn9 + (0.5 * locals.var_gf2_dn9)),)
    } else {
        (locals.var_asat, locals.var_asat_dn4, locals.var_asat_dn6, locals.var_asat_dn7, locals.var_asat_dn8, locals.var_asat_dn9,)
    }
};
        locals.var_asat = assign43790_e56574;
        locals.var_asat_dn4 = assign43790_e56574_d_n4;
        locals.var_asat_dn6 = assign43790_e56574_d_n6;
        locals.var_asat_dn7 = assign43790_e56574_d_n7;
        locals.var_asat_dn8 = assign43790_e56574_d_n8;
        locals.var_asat_dn9 = assign43790_e56574_d_n9;

        let (assign43800_e56586, assign43800_e56586_d_n4, assign43800_e56586_d_n6, assign43800_e56586_d_n7, assign43800_e56586_d_n8, assign43800_e56586_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign43800_e56580: f64 = (locals.var_gf2 * locals.var_delta_1s);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_asat;
        let assign43800_e56582: f64 = (assign43800_e56580 * __rspice_inv_cse_0);
        let assign43800_e56584: f64 = (assign43800_e56582 * __rspice_inv_cse_0);
        (assign43800_e56584, ((((((((locals.var_gf2_dn4 * locals.var_delta_1s) + (locals.var_gf2 * locals.var_delta_1s_dn4)) * locals.var_asat) - (assign43800_e56580 * locals.var_asat_dn4)) / (locals.var_asat * locals.var_asat)) * locals.var_asat) - (assign43800_e56582 * locals.var_asat_dn4)) / (locals.var_asat * locals.var_asat)), ((((((((locals.var_gf2_dn6 * locals.var_delta_1s) + (locals.var_gf2 * locals.var_delta_1s_dn6)) * locals.var_asat) - (assign43800_e56580 * locals.var_asat_dn6)) / (locals.var_asat * locals.var_asat)) * locals.var_asat) - (assign43800_e56582 * locals.var_asat_dn6)) / (locals.var_asat * locals.var_asat)), ((((((((locals.var_gf2_dn7 * locals.var_delta_1s) + (locals.var_gf2 * locals.var_delta_1s_dn7)) * locals.var_asat) - (assign43800_e56580 * locals.var_asat_dn7)) / (locals.var_asat * locals.var_asat)) * locals.var_asat) - (assign43800_e56582 * locals.var_asat_dn7)) / (locals.var_asat * locals.var_asat)), ((((((((locals.var_gf2_dn8 * locals.var_delta_1s) + (locals.var_gf2 * locals.var_delta_1s_dn8)) * locals.var_asat) - (assign43800_e56580 * locals.var_asat_dn8)) / (locals.var_asat * locals.var_asat)) * locals.var_asat) - (assign43800_e56582 * locals.var_asat_dn8)) / (locals.var_asat * locals.var_asat)), ((((((((locals.var_gf2_dn9 * locals.var_delta_1s) + (locals.var_gf2 * locals.var_delta_1s_dn9)) * locals.var_asat) - (assign43800_e56580 * locals.var_asat_dn9)) / (locals.var_asat * locals.var_asat)) * locals.var_asat) - (assign43800_e56582 * locals.var_asat_dn9)) / (locals.var_asat * locals.var_asat)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign43800_e56586;
        locals.var_temp__blk949_dn4 = assign43800_e56586_d_n4;
        locals.var_temp__blk949_dn6 = assign43800_e56586_d_n6;
        locals.var_temp__blk949_dn7 = assign43800_e56586_d_n7;
        locals.var_temp__blk949_dn8 = assign43800_e56586_d_n8;
        locals.var_temp__blk949_dn9 = assign43800_e56586_d_n9;

        let assign43810_e56589: f64 = if locals.var_temp__blk949 > 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard1216 = assign43810_e56589;

        let (assign43820_e56599, assign43820_e56599_d_n4, assign43820_e56599_d_n6, assign43820_e56599_d_n7, assign43820_e56599_d_n8, assign43820_e56599_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1216 != 0.0)) {
        let assign43820_e56597: f64 = (1.0 - locals.var_temp__blk949);
        (assign43820_e56597, (-locals.var_temp__blk949_dn4), (-locals.var_temp__blk949_dn6), (-locals.var_temp__blk949_dn7), (-locals.var_temp__blk949_dn8), (-locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign43820_e56599;
        locals.var_temp1_dn4 = assign43820_e56599_d_n4;
        locals.var_temp1_dn6 = assign43820_e56599_d_n6;
        locals.var_temp1_dn7 = assign43820_e56599_d_n7;
        locals.var_temp1_dn8 = assign43820_e56599_d_n8;
        locals.var_temp1_dn9 = assign43820_e56599_d_n9;

        let assign43830_e56602: f64 = if locals.var_temp1 < 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1217 = assign43830_e56602;

        let (assign43840_e56612, assign43840_e56612_d_n4, assign43840_e56612_d_n6, assign43840_e56612_d_n7, assign43840_e56612_d_n8, assign43840_e56612_d_n9,) = {
    if ((((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1216 != 0.0)) && (locals.var_guard1217 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign43840_e56612;
        locals.var_temp2_dn4 = assign43840_e56612_d_n4;
        locals.var_temp2_dn6 = assign43840_e56612_d_n6;
        locals.var_temp2_dn7 = assign43840_e56612_d_n7;
        locals.var_temp2_dn8 = assign43840_e56612_d_n8;
        locals.var_temp2_dn9 = assign43840_e56612_d_n9;

        let (assign43850_e56626, assign43850_e56626_d_n4, assign43850_e56626_d_n6, assign43850_e56626_d_n7, assign43850_e56626_d_n8, assign43850_e56626_d_n9,) = {
    if ((((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1216 != 0.0)) && (locals.var_guard1217 == 0.0)) {
        let assign43850_e56623: f64 = (locals.var_temp1).sqrt();
        let assign43850_e56624: f64 = (1.0 - assign43850_e56623);
        (assign43850_e56624, (-(locals.var_temp1_dn4 / (2.0 * assign43850_e56623))), (-(locals.var_temp1_dn6 / (2.0 * assign43850_e56623))), (-(locals.var_temp1_dn7 / (2.0 * assign43850_e56623))), (-(locals.var_temp1_dn8 / (2.0 * assign43850_e56623))), (-(locals.var_temp1_dn9 / (2.0 * assign43850_e56623))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign43850_e56626;
        locals.var_temp2_dn4 = assign43850_e56626_d_n4;
        locals.var_temp2_dn6 = assign43850_e56626_d_n6;
        locals.var_temp2_dn7 = assign43850_e56626_d_n7;
        locals.var_temp2_dn8 = assign43850_e56626_d_n8;
        locals.var_temp2_dn9 = assign43850_e56626_d_n9;

        let (assign43860_e56637, assign43860_e56637_d_n4, assign43860_e56637_d_n6, assign43860_e56637_d_n7, assign43860_e56637_d_n8, assign43860_e56637_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1216 == 0.0)) {
        let assign43860_e56635: f64 = (0.5 * locals.var_temp__blk949);
        (assign43860_e56635, (0.5 * locals.var_temp__blk949_dn4), (0.5 * locals.var_temp__blk949_dn6), (0.5 * locals.var_temp__blk949_dn7), (0.5 * locals.var_temp__blk949_dn8), (0.5 * locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign43860_e56637;
        locals.var_temp2_dn4 = assign43860_e56637_d_n4;
        locals.var_temp2_dn6 = assign43860_e56637_d_n6;
        locals.var_temp2_dn7 = assign43860_e56637_d_n7;
        locals.var_temp2_dn8 = assign43860_e56637_d_n8;
        locals.var_temp2_dn9 = assign43860_e56637_d_n9;

        let (assign43870_e56645, assign43870_e56645_d_n4, assign43870_e56645_d_n6, assign43870_e56645_d_n7, assign43870_e56645_d_n8, assign43870_e56645_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign43870_e56643: f64 = (locals.var_temp2 * locals.var_asat);
        (assign43870_e56643, ((locals.var_temp2_dn4 * locals.var_asat) + (locals.var_temp2 * locals.var_asat_dn4)), ((locals.var_temp2_dn6 * locals.var_asat) + (locals.var_temp2 * locals.var_asat_dn6)), ((locals.var_temp2_dn7 * locals.var_asat) + (locals.var_temp2 * locals.var_asat_dn7)), ((locals.var_temp2_dn8 * locals.var_asat) + (locals.var_temp2 * locals.var_asat_dn8)), ((locals.var_temp2_dn9 * locals.var_asat) + (locals.var_temp2 * locals.var_asat_dn9)),)
    } else {
        (locals.var_x_inf0, locals.var_x_inf0_dn4, locals.var_x_inf0_dn6, locals.var_x_inf0_dn7, locals.var_x_inf0_dn8, locals.var_x_inf0_dn9,)
    }
};
        locals.var_x_inf0 = assign43870_e56645;
        locals.var_x_inf0_dn4 = assign43870_e56645_d_n4;
        locals.var_x_inf0_dn6 = assign43870_e56645_d_n6;
        locals.var_x_inf0_dn7 = assign43870_e56645_d_n7;
        locals.var_x_inf0_dn8 = assign43870_e56645_d_n8;
        locals.var_x_inf0_dn9 = assign43870_e56645_d_n9;

        let assign43880_e56652: f64 = if ((locals.var_cs_t > 0.0) && (locals.var_thecs_t > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1218 = assign43880_e56652;

        let (assign43890_e56664, assign43890_e56664_d_n4, assign43890_e56664_d_n6, assign43890_e56664_d_n7, assign43890_e56664_d_n8, assign43890_e56664_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign43890_e56660: f64 = (0.475 * locals.var_phit1);
        let assign43890_e56662: f64 = (assign43890_e56660 * locals.var_x_inf0);
        (assign43890_e56662, (((0.475 * locals.var_phit1_dn4) * locals.var_x_inf0) + (assign43890_e56660 * locals.var_x_inf0_dn4)), (((0.475 * locals.var_phit1_dn6) * locals.var_x_inf0) + (assign43890_e56660 * locals.var_x_inf0_dn6)), (((0.475 * locals.var_phit1_dn7) * locals.var_x_inf0) + (assign43890_e56660 * locals.var_x_inf0_dn7)), (((0.475 * locals.var_phit1_dn8) * locals.var_x_inf0) + (assign43890_e56660 * locals.var_x_inf0_dn8)), (((0.475 * locals.var_phit1_dn9) * locals.var_x_inf0) + (assign43890_e56660 * locals.var_x_inf0_dn9)),)
    } else {
        (locals.var_midphi0, locals.var_midphi0_dn4, locals.var_midphi0_dn6, locals.var_midphi0_dn7, locals.var_midphi0_dn8, locals.var_midphi0_dn9,)
    }
};
        locals.var_midphi0 = assign43890_e56664;
        locals.var_midphi0_dn4 = assign43890_e56664_d_n4;
        locals.var_midphi0_dn6 = assign43890_e56664_d_n6;
        locals.var_midphi0_dn7 = assign43890_e56664_d_n7;
        locals.var_midphi0_dn8 = assign43890_e56664_d_n8;
        locals.var_midphi0_dn9 = assign43890_e56664_d_n9;

        let (assign43900_e56676, assign43900_e56676_d_n4, assign43900_e56676_d_n6, assign43900_e56676_d_n7, assign43900_e56676_d_n8, assign43900_e56676_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign43900_e56673: f64 = (locals.var_alphas * locals.var_midphi0);
        let assign43900_e56674: f64 = (locals.var_qis - assign43900_e56673);
        (assign43900_e56674, (locals.var_qis_dn4 - ((locals.var_alphas_dn4 * locals.var_midphi0) + (locals.var_alphas * locals.var_midphi0_dn4))), (locals.var_qis_dn6 - ((locals.var_alphas_dn6 * locals.var_midphi0) + (locals.var_alphas * locals.var_midphi0_dn6))), (locals.var_qis_dn7 - ((locals.var_alphas_dn7 * locals.var_midphi0) + (locals.var_alphas * locals.var_midphi0_dn7))), (locals.var_qis_dn8 - ((locals.var_alphas_dn8 * locals.var_midphi0) + (locals.var_alphas * locals.var_midphi0_dn8))), (locals.var_qis_dn9 - ((locals.var_alphas_dn9 * locals.var_midphi0) + (locals.var_alphas * locals.var_midphi0_dn9))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign43900_e56676;
        locals.var_temp__blk949_dn4 = assign43900_e56676_d_n4;
        locals.var_temp__blk949_dn6 = assign43900_e56676_d_n6;
        locals.var_temp__blk949_dn7 = assign43900_e56676_d_n7;
        locals.var_temp__blk949_dn8 = assign43900_e56676_d_n8;
        locals.var_temp__blk949_dn9 = assign43900_e56676_d_n9;

        let (assign43910_e56693, assign43910_e56693_d_n4, assign43910_e56693_d_n6, assign43910_e56693_d_n7, assign43910_e56693_d_n8, assign43910_e56693_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign43910_e56686: f64 = (locals.var_temp__blk949 * locals.var_temp__blk949);
        let assign43910_e56688: f64 = (assign43910_e56686 + 1e-12);
        let assign43910_e56689: f64 = (assign43910_e56688).sqrt();
        let assign43910_e56690: f64 = (locals.var_temp__blk949 + assign43910_e56689);
        let assign43910_e56691: f64 = (0.5 * assign43910_e56690);
        (assign43910_e56691, (0.5 * (locals.var_temp__blk949_dn4 + (((locals.var_temp__blk949_dn4 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn4)) / (2.0 * assign43910_e56689)))), (0.5 * (locals.var_temp__blk949_dn6 + (((locals.var_temp__blk949_dn6 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn6)) / (2.0 * assign43910_e56689)))), (0.5 * (locals.var_temp__blk949_dn7 + (((locals.var_temp__blk949_dn7 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn7)) / (2.0 * assign43910_e56689)))), (0.5 * (locals.var_temp__blk949_dn8 + (((locals.var_temp__blk949_dn8 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn8)) / (2.0 * assign43910_e56689)))), (0.5 * (locals.var_temp__blk949_dn9 + (((locals.var_temp__blk949_dn9 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn9)) / (2.0 * assign43910_e56689)))),)
    } else {
        (locals.var_qisat, locals.var_qisat_dn4, locals.var_qisat_dn6, locals.var_qisat_dn7, locals.var_qisat_dn8, locals.var_qisat_dn9,)
    }
};
        locals.var_qisat = assign43910_e56693;
        locals.var_qisat_dn4 = assign43910_e56693_d_n4;
        locals.var_qisat_dn6 = assign43910_e56693_d_n6;
        locals.var_qisat_dn7 = assign43910_e56693_d_n7;
        locals.var_qisat_dn8 = assign43910_e56693_d_n8;
        locals.var_qisat_dn9 = assign43910_e56693_d_n9;

        let (assign43920_e56711, assign43920_e56711_d_n4, assign43920_e56711_d_n6, assign43920_e56711_d_n7, assign43920_e56711_d_n8, assign43920_e56711_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign43920_e56701: f64 = (locals.var_phit1 * locals.var_xgs);
        let assign43920_e56703: f64 = (assign43920_e56701 - locals.var_qis);
        let assign43920_e56706: f64 = (locals.var_alphas - 1.0);
        let assign43920_e56708: f64 = (assign43920_e56706 * locals.var_midphi0);
        let assign43920_e56709: f64 = (assign43920_e56703 + assign43920_e56708);
        (assign43920_e56709, ((((locals.var_phit1_dn4 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn4)) - locals.var_qis_dn4) + ((locals.var_alphas_dn4 * locals.var_midphi0) + (assign43920_e56706 * locals.var_midphi0_dn4))), ((((locals.var_phit1_dn6 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn6)) - locals.var_qis_dn6) + ((locals.var_alphas_dn6 * locals.var_midphi0) + (assign43920_e56706 * locals.var_midphi0_dn6))), ((((locals.var_phit1_dn7 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn7)) - locals.var_qis_dn7) + ((locals.var_alphas_dn7 * locals.var_midphi0) + (assign43920_e56706 * locals.var_midphi0_dn7))), ((((locals.var_phit1_dn8 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn8)) - locals.var_qis_dn8) + ((locals.var_alphas_dn8 * locals.var_midphi0) + (assign43920_e56706 * locals.var_midphi0_dn8))), ((((locals.var_phit1_dn9 * locals.var_xgs) + (locals.var_phit1 * locals.var_xgs_dn9)) - locals.var_qis_dn9) + ((locals.var_alphas_dn9 * locals.var_midphi0) + (assign43920_e56706 * locals.var_midphi0_dn9))),)
    } else {
        (locals.var_qbsat, locals.var_qbsat_dn4, locals.var_qbsat_dn6, locals.var_qbsat_dn7, locals.var_qbsat_dn8, locals.var_qbsat_dn9,)
    }
};
        locals.var_qbsat = assign43920_e56711;
        locals.var_qbsat_dn4 = assign43920_e56711_d_n4;
        locals.var_qbsat_dn6 = assign43920_e56711_d_n6;
        locals.var_qbsat_dn7 = assign43920_e56711_d_n7;
        locals.var_qbsat_dn8 = assign43920_e56711_d_n8;
        locals.var_qbsat_dn9 = assign43920_e56711_d_n9;

        let (assign43930_e56727, assign43930_e56727_d_n4, assign43930_e56727_d_n6, assign43930_e56727_d_n7, assign43930_e56727_d_n8, assign43930_e56727_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign43930_e56720: f64 = (0.5 * locals.var_gf2);
        let assign43930_e56722: f64 = (assign43930_e56720 * locals.var_phit1);
        let assign43930_e56724: f64 = (assign43930_e56722 / locals.var_qbsat);
        let assign43930_e56725: f64 = (1.0 + assign43930_e56724);
        (assign43930_e56725, ((((((0.5 * locals.var_gf2_dn4) * locals.var_phit1) + (assign43930_e56720 * locals.var_phit1_dn4)) * locals.var_qbsat) - (assign43930_e56722 * locals.var_qbsat_dn4)) / (locals.var_qbsat * locals.var_qbsat)), ((((((0.5 * locals.var_gf2_dn6) * locals.var_phit1) + (assign43930_e56720 * locals.var_phit1_dn6)) * locals.var_qbsat) - (assign43930_e56722 * locals.var_qbsat_dn6)) / (locals.var_qbsat * locals.var_qbsat)), ((((((0.5 * locals.var_gf2_dn7) * locals.var_phit1) + (assign43930_e56720 * locals.var_phit1_dn7)) * locals.var_qbsat) - (assign43930_e56722 * locals.var_qbsat_dn7)) / (locals.var_qbsat * locals.var_qbsat)), ((((((0.5 * locals.var_gf2_dn8) * locals.var_phit1) + (assign43930_e56720 * locals.var_phit1_dn8)) * locals.var_qbsat) - (assign43930_e56722 * locals.var_qbsat_dn8)) / (locals.var_qbsat * locals.var_qbsat)), ((((((0.5 * locals.var_gf2_dn9) * locals.var_phit1) + (assign43930_e56720 * locals.var_phit1_dn9)) * locals.var_qbsat) - (assign43930_e56722 * locals.var_qbsat_dn9)) / (locals.var_qbsat * locals.var_qbsat)),)
    } else {
        (locals.var_alphasat, locals.var_alphasat_dn4, locals.var_alphasat_dn6, locals.var_alphasat_dn7, locals.var_alphasat_dn8, locals.var_alphasat_dn9,)
    }
};
        locals.var_alphasat = assign43930_e56727;
        locals.var_alphasat_dn4 = assign43930_e56727_d_n4;
        locals.var_alphasat_dn6 = assign43930_e56727_d_n6;
        locals.var_alphasat_dn7 = assign43930_e56727_d_n7;
        locals.var_alphasat_dn8 = assign43930_e56727_d_n8;
        locals.var_alphasat_dn9 = assign43930_e56727_d_n9;

        let (assign43940_e56739, assign43940_e56739_d_n4, assign43940_e56739_d_n6, assign43940_e56739_d_n7, assign43940_e56739_d_n8, assign43940_e56739_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign43940_e56736: f64 = (locals.var_eta_mu * locals.var_qisat);
        let assign43940_e56737: f64 = (locals.var_qbsat + assign43940_e56736);
        (assign43940_e56737, (locals.var_qbsat_dn4 + (locals.var_eta_mu * locals.var_qisat_dn4)), (locals.var_qbsat_dn6 + (locals.var_eta_mu * locals.var_qisat_dn6)), (locals.var_qbsat_dn7 + (locals.var_eta_mu * locals.var_qisat_dn7)), (locals.var_qbsat_dn8 + (locals.var_eta_mu * locals.var_qisat_dn8)), (locals.var_qbsat_dn9 + (locals.var_eta_mu * locals.var_qisat_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign43940_e56739;
        locals.var_temp__blk949_dn4 = assign43940_e56739_d_n4;
        locals.var_temp__blk949_dn6 = assign43940_e56739_d_n6;
        locals.var_temp__blk949_dn7 = assign43940_e56739_d_n7;
        locals.var_temp__blk949_dn8 = assign43940_e56739_d_n8;
        locals.var_temp__blk949_dn9 = assign43940_e56739_d_n9;

        let (assign43950_e56753, assign43950_e56753_d_n4, assign43950_e56753_d_n6, assign43950_e56753_d_n7, assign43950_e56753_d_n8, assign43950_e56753_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign43950_e56747: f64 = (locals.var_e_eff0 * locals.var_temp__blk949);
        let assign43950_e56749: f64 = (assign43950_e56747 * locals.var_mue_t);
        let assign43950_e56751: f64 = (assign43950_e56749).powf(locals.var_themu_t);
        (assign43950_e56751, if locals.var_themu_t_dn4 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43950_e56749).powf(locals.var_themu_t - 1.0) * (((locals.var_e_eff0 * locals.var_temp__blk949_dn4) * locals.var_mue_t) + (assign43950_e56747 * locals.var_mue_t_dn4)))) } } else { (assign43950_e56751 * ((locals.var_themu_t_dn4 * (assign43950_e56749).ln()) + (locals.var_themu_t * ((((locals.var_e_eff0 * locals.var_temp__blk949_dn4) * locals.var_mue_t) + (assign43950_e56747 * locals.var_mue_t_dn4)) / assign43950_e56749)))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43950_e56749).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk949_dn6) * locals.var_mue_t))) } } else { (assign43950_e56751 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk949_dn6) * locals.var_mue_t) / assign43950_e56749))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43950_e56749).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk949_dn7) * locals.var_mue_t))) } } else { (assign43950_e56751 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk949_dn7) * locals.var_mue_t) / assign43950_e56749))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43950_e56749).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk949_dn8) * locals.var_mue_t))) } } else { (assign43950_e56751 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk949_dn8) * locals.var_mue_t) / assign43950_e56749))) }, if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign43950_e56749).powf(locals.var_themu_t - 1.0) * ((locals.var_e_eff0 * locals.var_temp__blk949_dn9) * locals.var_mue_t))) } } else { (assign43950_e56751 * (locals.var_themu_t * (((locals.var_e_eff0 * locals.var_temp__blk949_dn9) * locals.var_mue_t) / assign43950_e56749))) },)
    } else {
        (locals.var_gmobmusat, locals.var_gmobmusat_dn4, locals.var_gmobmusat_dn6, locals.var_gmobmusat_dn7, locals.var_gmobmusat_dn8, locals.var_gmobmusat_dn9,)
    }
};
        locals.var_gmobmusat = assign43950_e56753;
        locals.var_gmobmusat_dn4 = assign43950_e56753_d_n4;
        locals.var_gmobmusat_dn6 = assign43950_e56753_d_n6;
        locals.var_gmobmusat_dn7 = assign43950_e56753_d_n7;
        locals.var_gmobmusat_dn8 = assign43950_e56753_d_n8;
        locals.var_gmobmusat_dn9 = assign43950_e56753_d_n9;

        let (assign43960_e56773, assign43960_e56773_d_n4, assign43960_e56773_d_n6, assign43960_e56773_d_n7, assign43960_e56773_d_n8, assign43960_e56773_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign43960_e56763: f64 = (1.0 - locals.var_eta_mu);
        let assign43960_e56764: f64 = (locals.var_alphasat * assign43960_e56763);
        let assign43960_e56766: f64 = (assign43960_e56764 - 1.0);
        let assign43960_e56767: f64 = (locals.var_themu_t * assign43960_e56766);
        let assign43960_e56769: f64 = (assign43960_e56767 / locals.var_temp__blk949);
        let assign43960_e56771: f64 = (assign43960_e56769 * locals.var_gmobmusat);
        (assign43960_e56771, (((((((locals.var_themu_t_dn4 * assign43960_e56766) + (locals.var_themu_t * (locals.var_alphasat_dn4 * assign43960_e56763))) * locals.var_temp__blk949) - (assign43960_e56767 * locals.var_temp__blk949_dn4)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat) + (assign43960_e56769 * locals.var_gmobmusat_dn4)), ((((((locals.var_themu_t * (locals.var_alphasat_dn6 * assign43960_e56763)) * locals.var_temp__blk949) - (assign43960_e56767 * locals.var_temp__blk949_dn6)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat) + (assign43960_e56769 * locals.var_gmobmusat_dn6)), ((((((locals.var_themu_t * (locals.var_alphasat_dn7 * assign43960_e56763)) * locals.var_temp__blk949) - (assign43960_e56767 * locals.var_temp__blk949_dn7)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat) + (assign43960_e56769 * locals.var_gmobmusat_dn7)), ((((((locals.var_themu_t * (locals.var_alphasat_dn8 * assign43960_e56763)) * locals.var_temp__blk949) - (assign43960_e56767 * locals.var_temp__blk949_dn8)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat) + (assign43960_e56769 * locals.var_gmobmusat_dn8)), ((((((locals.var_themu_t * (locals.var_alphasat_dn9 * assign43960_e56763)) * locals.var_temp__blk949) - (assign43960_e56767 * locals.var_temp__blk949_dn9)) / (locals.var_temp__blk949 * locals.var_temp__blk949)) * locals.var_gmobmusat) + (assign43960_e56769 * locals.var_gmobmusat_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign43960_e56773;
        locals.var_temp1_dn4 = assign43960_e56773_d_n4;
        locals.var_temp1_dn6 = assign43960_e56773_d_n6;
        locals.var_temp1_dn7 = assign43960_e56773_d_n7;
        locals.var_temp1_dn8 = assign43960_e56773_d_n8;
        locals.var_temp1_dn9 = assign43960_e56773_d_n9;

        let (assign43970_e56783, assign43970_e56783_d_n4, assign43970_e56783_d_n6, assign43970_e56783_d_n7, assign43970_e56783_d_n8, assign43970_e56783_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign43970_e56781: f64 = (locals.var_qisat / locals.var_qbsat);
        (assign43970_e56781, (((locals.var_qisat_dn4 * locals.var_qbsat) - (locals.var_qisat * locals.var_qbsat_dn4)) / (locals.var_qbsat * locals.var_qbsat)), (((locals.var_qisat_dn6 * locals.var_qbsat) - (locals.var_qisat * locals.var_qbsat_dn6)) / (locals.var_qbsat * locals.var_qbsat)), (((locals.var_qisat_dn7 * locals.var_qbsat) - (locals.var_qisat * locals.var_qbsat_dn7)) / (locals.var_qbsat * locals.var_qbsat)), (((locals.var_qisat_dn8 * locals.var_qbsat) - (locals.var_qisat * locals.var_qbsat_dn8)) / (locals.var_qbsat * locals.var_qbsat)), (((locals.var_qisat_dn9 * locals.var_qbsat) - (locals.var_qisat * locals.var_qbsat_dn9)) / (locals.var_qbsat * locals.var_qbsat)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign43970_e56783;
        locals.var_temp__blk949_dn4 = assign43970_e56783_d_n4;
        locals.var_temp__blk949_dn6 = assign43970_e56783_d_n6;
        locals.var_temp__blk949_dn7 = assign43970_e56783_d_n7;
        locals.var_temp__blk949_dn8 = assign43970_e56783_d_n8;
        locals.var_temp__blk949_dn9 = assign43970_e56783_d_n9;

        let (assign43980_e56798, assign43980_e56798_d_n4, assign43980_e56798_d_n6, assign43980_e56798_d_n7, assign43980_e56798_d_n8, assign43980_e56798_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign43980_e56792: f64 = (1.0 + locals.var_temp__blk949);
        let assign43980_e56794: f64 = (-locals.var_thecs_t);
        let assign43980_e56795: f64 = (assign43980_e56792).powf(assign43980_e56794);
        let assign43980_e56796: f64 = (locals.var_cs_t * assign43980_e56795);
        (assign43980_e56796, ((locals.var_cs_t_dn4 * assign43980_e56795) + (locals.var_cs_t * if (-locals.var_thecs_t_dn4) == 0.0 && ((assign43980_e56794) as f64).is_finite() && ((assign43980_e56794) as f64).fract() == 0.0 { if assign43980_e56794 == 0.0 { 0.0 } else { (assign43980_e56794 * ((assign43980_e56792).powf(assign43980_e56794 - 1.0) * locals.var_temp__blk949_dn4)) } } else { (assign43980_e56795 * (((-locals.var_thecs_t_dn4) * (assign43980_e56792).ln()) + (assign43980_e56794 * (locals.var_temp__blk949_dn4 / assign43980_e56792)))) })), (locals.var_cs_t * if 0.0 == 0.0 && ((assign43980_e56794) as f64).is_finite() && ((assign43980_e56794) as f64).fract() == 0.0 { if assign43980_e56794 == 0.0 { 0.0 } else { (assign43980_e56794 * ((assign43980_e56792).powf(assign43980_e56794 - 1.0) * locals.var_temp__blk949_dn6)) } } else { (assign43980_e56795 * (assign43980_e56794 * (locals.var_temp__blk949_dn6 / assign43980_e56792))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign43980_e56794) as f64).is_finite() && ((assign43980_e56794) as f64).fract() == 0.0 { if assign43980_e56794 == 0.0 { 0.0 } else { (assign43980_e56794 * ((assign43980_e56792).powf(assign43980_e56794 - 1.0) * locals.var_temp__blk949_dn7)) } } else { (assign43980_e56795 * (assign43980_e56794 * (locals.var_temp__blk949_dn7 / assign43980_e56792))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign43980_e56794) as f64).is_finite() && ((assign43980_e56794) as f64).fract() == 0.0 { if assign43980_e56794 == 0.0 { 0.0 } else { (assign43980_e56794 * ((assign43980_e56792).powf(assign43980_e56794 - 1.0) * locals.var_temp__blk949_dn8)) } } else { (assign43980_e56795 * (assign43980_e56794 * (locals.var_temp__blk949_dn8 / assign43980_e56792))) }), (locals.var_cs_t * if 0.0 == 0.0 && ((assign43980_e56794) as f64).is_finite() && ((assign43980_e56794) as f64).fract() == 0.0 { if assign43980_e56794 == 0.0 { 0.0 } else { (assign43980_e56794 * ((assign43980_e56792).powf(assign43980_e56794 - 1.0) * locals.var_temp__blk949_dn9)) } } else { (assign43980_e56795 * (assign43980_e56794 * (locals.var_temp__blk949_dn9 / assign43980_e56792))) }),)
    } else {
        (locals.var_gmobcssat, locals.var_gmobcssat_dn4, locals.var_gmobcssat_dn6, locals.var_gmobcssat_dn7, locals.var_gmobcssat_dn8, locals.var_gmobcssat_dn9,)
    }
};
        locals.var_gmobcssat = assign43980_e56798;
        locals.var_gmobcssat_dn4 = assign43980_e56798_d_n4;
        locals.var_gmobcssat_dn6 = assign43980_e56798_d_n6;
        locals.var_gmobcssat_dn7 = assign43980_e56798_d_n7;
        locals.var_gmobcssat_dn8 = assign43980_e56798_d_n8;
        locals.var_gmobcssat_dn9 = assign43980_e56798_d_n9;

        let (assign43990_e56820, assign43990_e56820_d_n4, assign43990_e56820_d_n6, assign43990_e56820_d_n7, assign43990_e56820_d_n8, assign43990_e56820_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign43990_e56807: f64 = (locals.var_alphasat - 1.0);
        let assign43990_e56811: f64 = (locals.var_temp__blk949 + 1.0);
        let assign43990_e56812: f64 = (1.0 / assign43990_e56811);
        let assign43990_e56813: f64 = (assign43990_e56807 + assign43990_e56812);
        let assign43990_e56814: f64 = (locals.var_thecs_t * assign43990_e56813);
        let assign43990_e56816: f64 = (assign43990_e56814 / locals.var_qbsat);
        let assign43990_e56818: f64 = (assign43990_e56816 * locals.var_gmobcssat);
        (assign43990_e56818, (((((((locals.var_thecs_t_dn4 * assign43990_e56813) + (locals.var_thecs_t * (locals.var_alphasat_dn4 + (-(locals.var_temp__blk949_dn4 / (assign43990_e56811 * assign43990_e56811)))))) * locals.var_qbsat) - (assign43990_e56814 * locals.var_qbsat_dn4)) / (locals.var_qbsat * locals.var_qbsat)) * locals.var_gmobcssat) + (assign43990_e56816 * locals.var_gmobcssat_dn4)), ((((((locals.var_thecs_t * (locals.var_alphasat_dn6 + (-(locals.var_temp__blk949_dn6 / (assign43990_e56811 * assign43990_e56811))))) * locals.var_qbsat) - (assign43990_e56814 * locals.var_qbsat_dn6)) / (locals.var_qbsat * locals.var_qbsat)) * locals.var_gmobcssat) + (assign43990_e56816 * locals.var_gmobcssat_dn6)), ((((((locals.var_thecs_t * (locals.var_alphasat_dn7 + (-(locals.var_temp__blk949_dn7 / (assign43990_e56811 * assign43990_e56811))))) * locals.var_qbsat) - (assign43990_e56814 * locals.var_qbsat_dn7)) / (locals.var_qbsat * locals.var_qbsat)) * locals.var_gmobcssat) + (assign43990_e56816 * locals.var_gmobcssat_dn7)), ((((((locals.var_thecs_t * (locals.var_alphasat_dn8 + (-(locals.var_temp__blk949_dn8 / (assign43990_e56811 * assign43990_e56811))))) * locals.var_qbsat) - (assign43990_e56814 * locals.var_qbsat_dn8)) / (locals.var_qbsat * locals.var_qbsat)) * locals.var_gmobcssat) + (assign43990_e56816 * locals.var_gmobcssat_dn8)), ((((((locals.var_thecs_t * (locals.var_alphasat_dn9 + (-(locals.var_temp__blk949_dn9 / (assign43990_e56811 * assign43990_e56811))))) * locals.var_qbsat) - (assign43990_e56814 * locals.var_qbsat_dn9)) / (locals.var_qbsat * locals.var_qbsat)) * locals.var_gmobcssat) + (assign43990_e56816 * locals.var_gmobcssat_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign43990_e56820;
        locals.var_temp2_dn4 = assign43990_e56820_d_n4;
        locals.var_temp2_dn6 = assign43990_e56820_d_n6;
        locals.var_temp2_dn7 = assign43990_e56820_d_n7;
        locals.var_temp2_dn8 = assign43990_e56820_d_n8;
        locals.var_temp2_dn9 = assign43990_e56820_d_n9;

        let (assign44000_e56834, assign44000_e56834_d_n4, assign44000_e56834_d_n6, assign44000_e56834_d_n7, assign44000_e56834_d_n8, assign44000_e56834_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign44000_e56828: f64 = (locals.var_ther_i * locals.var_rhob);
        let assign44000_e56830: f64 = (assign44000_e56828 * locals.var_rhog);
        let assign44000_e56832: f64 = (assign44000_e56830 * locals.var_qisat);
        (assign44000_e56832, ((((((locals.var_ther_i_dn4 * locals.var_rhob) + (locals.var_ther_i * locals.var_rhob_dn4)) * locals.var_rhog) + (assign44000_e56828 * locals.var_rhog_dn4)) * locals.var_qisat) + (assign44000_e56830 * locals.var_qisat_dn4)), (((((locals.var_ther_i * locals.var_rhob_dn6) * locals.var_rhog) + (assign44000_e56828 * locals.var_rhog_dn6)) * locals.var_qisat) + (assign44000_e56830 * locals.var_qisat_dn6)), (((((locals.var_ther_i * locals.var_rhob_dn7) * locals.var_rhog) + (assign44000_e56828 * locals.var_rhog_dn7)) * locals.var_qisat) + (assign44000_e56830 * locals.var_qisat_dn7)), (((((locals.var_ther_i * locals.var_rhob_dn8) * locals.var_rhog) + (assign44000_e56828 * locals.var_rhog_dn8)) * locals.var_qisat) + (assign44000_e56830 * locals.var_qisat_dn8)), (((((locals.var_ther_i * locals.var_rhob_dn9) * locals.var_rhog) + (assign44000_e56828 * locals.var_rhog_dn9)) * locals.var_qisat) + (assign44000_e56830 * locals.var_qisat_dn9)),)
    } else {
        (locals.var_grsat, locals.var_grsat_dn4, locals.var_grsat_dn6, locals.var_grsat_dn7, locals.var_grsat_dn8, locals.var_grsat_dn9,)
    }
};
        locals.var_grsat = assign44000_e56834;
        locals.var_grsat_dn4 = assign44000_e56834_d_n4;
        locals.var_grsat_dn6 = assign44000_e56834_d_n6;
        locals.var_grsat_dn7 = assign44000_e56834_d_n7;
        locals.var_grsat_dn8 = assign44000_e56834_d_n8;
        locals.var_grsat_dn9 = assign44000_e56834_d_n9;

        let (assign44010_e56854, assign44010_e56854_d_n4, assign44010_e56854_d_n6, assign44010_e56854_d_n7, assign44010_e56854_d_n8, assign44010_e56854_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign44010_e56844: f64 = (locals.var_ther_i * locals.var_rhob);
        let assign44010_e56846: f64 = (assign44010_e56844 * locals.var_rhog);
        let assign44010_e56848: f64 = (assign44010_e56846 * locals.var_alphasat);
        let assign44010_e56849: f64 = (locals.var_temp1 - assign44010_e56848);
        let assign44010_e56851: f64 = (assign44010_e56849 / locals.var_temp2);
        let assign44010_e56852: f64 = (1.0 + assign44010_e56851);
        (assign44010_e56852, ((((locals.var_temp1_dn4 - ((((((locals.var_ther_i_dn4 * locals.var_rhob) + (locals.var_ther_i * locals.var_rhob_dn4)) * locals.var_rhog) + (assign44010_e56844 * locals.var_rhog_dn4)) * locals.var_alphasat) + (assign44010_e56846 * locals.var_alphasat_dn4))) * locals.var_temp2) - (assign44010_e56849 * locals.var_temp2_dn4)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn6 - (((((locals.var_ther_i * locals.var_rhob_dn6) * locals.var_rhog) + (assign44010_e56844 * locals.var_rhog_dn6)) * locals.var_alphasat) + (assign44010_e56846 * locals.var_alphasat_dn6))) * locals.var_temp2) - (assign44010_e56849 * locals.var_temp2_dn6)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn7 - (((((locals.var_ther_i * locals.var_rhob_dn7) * locals.var_rhog) + (assign44010_e56844 * locals.var_rhog_dn7)) * locals.var_alphasat) + (assign44010_e56846 * locals.var_alphasat_dn7))) * locals.var_temp2) - (assign44010_e56849 * locals.var_temp2_dn7)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn8 - (((((locals.var_ther_i * locals.var_rhob_dn8) * locals.var_rhog) + (assign44010_e56844 * locals.var_rhog_dn8)) * locals.var_alphasat) + (assign44010_e56846 * locals.var_alphasat_dn8))) * locals.var_temp2) - (assign44010_e56849 * locals.var_temp2_dn8)) / (locals.var_temp2 * locals.var_temp2)), ((((locals.var_temp1_dn9 - (((((locals.var_ther_i * locals.var_rhob_dn9) * locals.var_rhog) + (assign44010_e56844 * locals.var_rhog_dn9)) * locals.var_alphasat) + (assign44010_e56846 * locals.var_alphasat_dn9))) * locals.var_temp2) - (assign44010_e56849 * locals.var_temp2_dn9)) / (locals.var_temp2 * locals.var_temp2)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign44010_e56854;
        locals.var_temp__blk949_dn4 = assign44010_e56854_d_n4;
        locals.var_temp__blk949_dn6 = assign44010_e56854_d_n6;
        locals.var_temp__blk949_dn7 = assign44010_e56854_d_n7;
        locals.var_temp__blk949_dn8 = assign44010_e56854_d_n8;
        locals.var_temp__blk949_dn9 = assign44010_e56854_d_n9;

        let assign44020_e56857: f64 = if locals.var_temp__blk949 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1219 = assign44020_e56857;

        let (assign44030_e56875, assign44030_e56875_d_n4, assign44030_e56875_d_n6, assign44030_e56875_d_n7, assign44030_e56875_d_n8, assign44030_e56875_d_n9,) = {
    if ((((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) && (locals.var_guard1219 != 0.0)) {
        let assign44030_e56869: f64 = (2.0 * locals.var_temp__blk949);
        let assign44030_e56870: f64 = (assign44030_e56869).exp();
        let assign44030_e56871: f64 = (1.0 + assign44030_e56870);
        let assign44030_e56872: f64 = (assign44030_e56871).ln();
        let assign44030_e56873: f64 = (0.5 * assign44030_e56872);
        (assign44030_e56873, (0.5 * ((assign44030_e56870 * (2.0 * locals.var_temp__blk949_dn4)) / assign44030_e56871)), (0.5 * ((assign44030_e56870 * (2.0 * locals.var_temp__blk949_dn6)) / assign44030_e56871)), (0.5 * ((assign44030_e56870 * (2.0 * locals.var_temp__blk949_dn7)) / assign44030_e56871)), (0.5 * ((assign44030_e56870 * (2.0 * locals.var_temp__blk949_dn8)) / assign44030_e56871)), (0.5 * ((assign44030_e56870 * (2.0 * locals.var_temp__blk949_dn9)) / assign44030_e56871)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign44030_e56875;
        locals.var_temp1_dn4 = assign44030_e56875_d_n4;
        locals.var_temp1_dn6 = assign44030_e56875_d_n6;
        locals.var_temp1_dn7 = assign44030_e56875_d_n7;
        locals.var_temp1_dn8 = assign44030_e56875_d_n8;
        locals.var_temp1_dn9 = assign44030_e56875_d_n9;

        let (assign44040_e56886, assign44040_e56886_d_n4, assign44040_e56886_d_n6, assign44040_e56886_d_n7, assign44040_e56886_d_n8, assign44040_e56886_d_n9,) = {
    if ((((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) && (locals.var_guard1219 == 0.0)) {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign44040_e56886;
        locals.var_temp1_dn4 = assign44040_e56886_d_n4;
        locals.var_temp1_dn6 = assign44040_e56886_d_n6;
        locals.var_temp1_dn7 = assign44040_e56886_d_n7;
        locals.var_temp1_dn8 = assign44040_e56886_d_n8;
        locals.var_temp1_dn9 = assign44040_e56886_d_n9;

        let (assign44050_e56907, assign44050_e56907_d_n4, assign44050_e56907_d_n6, assign44050_e56907_d_n7, assign44050_e56907_d_n8, assign44050_e56907_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign44050_e56893: f64 = (-locals.var_midphi0);
        let assign44050_e56895: f64 = (assign44050_e56893 * locals.var_temp2);
        let assign44050_e56897: f64 = (assign44050_e56895 * locals.var_temp1);
        let assign44050_e56900: f64 = (1.0 + locals.var_gmobmusat);
        let assign44050_e56902: f64 = (assign44050_e56900 + locals.var_gmobcssat);
        let assign44050_e56904: f64 = (assign44050_e56902 + locals.var_grsat);
        let assign44050_e56905: f64 = (assign44050_e56897 / assign44050_e56904);
        (assign44050_e56905, ((((((((-locals.var_midphi0_dn4) * locals.var_temp2) + (assign44050_e56893 * locals.var_temp2_dn4)) * locals.var_temp1) + (assign44050_e56895 * locals.var_temp1_dn4)) * assign44050_e56904) - (assign44050_e56897 * ((locals.var_gmobmusat_dn4 + locals.var_gmobcssat_dn4) + locals.var_grsat_dn4))) / (assign44050_e56904 * assign44050_e56904)), ((((((((-locals.var_midphi0_dn6) * locals.var_temp2) + (assign44050_e56893 * locals.var_temp2_dn6)) * locals.var_temp1) + (assign44050_e56895 * locals.var_temp1_dn6)) * assign44050_e56904) - (assign44050_e56897 * ((locals.var_gmobmusat_dn6 + locals.var_gmobcssat_dn6) + locals.var_grsat_dn6))) / (assign44050_e56904 * assign44050_e56904)), ((((((((-locals.var_midphi0_dn7) * locals.var_temp2) + (assign44050_e56893 * locals.var_temp2_dn7)) * locals.var_temp1) + (assign44050_e56895 * locals.var_temp1_dn7)) * assign44050_e56904) - (assign44050_e56897 * ((locals.var_gmobmusat_dn7 + locals.var_gmobcssat_dn7) + locals.var_grsat_dn7))) / (assign44050_e56904 * assign44050_e56904)), ((((((((-locals.var_midphi0_dn8) * locals.var_temp2) + (assign44050_e56893 * locals.var_temp2_dn8)) * locals.var_temp1) + (assign44050_e56895 * locals.var_temp1_dn8)) * assign44050_e56904) - (assign44050_e56897 * ((locals.var_gmobmusat_dn8 + locals.var_gmobcssat_dn8) + locals.var_grsat_dn8))) / (assign44050_e56904 * assign44050_e56904)), ((((((((-locals.var_midphi0_dn9) * locals.var_temp2) + (assign44050_e56893 * locals.var_temp2_dn9)) * locals.var_temp1) + (assign44050_e56895 * locals.var_temp1_dn9)) * assign44050_e56904) - (assign44050_e56897 * ((locals.var_gmobmusat_dn9 + locals.var_gmobcssat_dn9) + locals.var_grsat_dn9))) / (assign44050_e56904 * assign44050_e56904)),)
    } else {
        (locals.var_delta_gmob, locals.var_delta_gmob_dn4, locals.var_delta_gmob_dn6, locals.var_delta_gmob_dn7, locals.var_delta_gmob_dn8, locals.var_delta_gmob_dn9,)
    }
};
        locals.var_delta_gmob = assign44050_e56907;
        locals.var_delta_gmob_dn4 = assign44050_e56907_d_n4;
        locals.var_delta_gmob_dn6 = assign44050_e56907_d_n6;
        locals.var_delta_gmob_dn7 = assign44050_e56907_d_n7;
        locals.var_delta_gmob_dn8 = assign44050_e56907_d_n8;
        locals.var_delta_gmob_dn9 = assign44050_e56907_d_n9;

        let (assign44060_e56928, assign44060_e56928_d_n4, assign44060_e56928_d_n6, assign44060_e56928_d_n7, assign44060_e56928_d_n8, assign44060_e56928_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 != 0.0)) {
        let assign44060_e56920: f64 = (locals.var_delta_gmob * locals.var_delta_gmob);
        let assign44060_e56921: f64 = (1.0 + assign44060_e56920);
        let assign44060_e56922: f64 = (assign44060_e56921).sqrt();
        let assign44060_e56923: f64 = (1.0 + assign44060_e56922);
        let assign44060_e56924: f64 = (locals.var_delta_gmob / assign44060_e56923);
        let assign44060_e56925: f64 = (1.0 + assign44060_e56924);
        let assign44060_e56926: f64 = (locals.var_x_inf0 * assign44060_e56925);
        (assign44060_e56926, ((locals.var_x_inf0_dn4 * assign44060_e56925) + (locals.var_x_inf0 * (((locals.var_delta_gmob_dn4 * assign44060_e56923) - (locals.var_delta_gmob * (((locals.var_delta_gmob_dn4 * locals.var_delta_gmob) + (locals.var_delta_gmob * locals.var_delta_gmob_dn4)) / (2.0 * assign44060_e56922)))) / (assign44060_e56923 * assign44060_e56923)))), ((locals.var_x_inf0_dn6 * assign44060_e56925) + (locals.var_x_inf0 * (((locals.var_delta_gmob_dn6 * assign44060_e56923) - (locals.var_delta_gmob * (((locals.var_delta_gmob_dn6 * locals.var_delta_gmob) + (locals.var_delta_gmob * locals.var_delta_gmob_dn6)) / (2.0 * assign44060_e56922)))) / (assign44060_e56923 * assign44060_e56923)))), ((locals.var_x_inf0_dn7 * assign44060_e56925) + (locals.var_x_inf0 * (((locals.var_delta_gmob_dn7 * assign44060_e56923) - (locals.var_delta_gmob * (((locals.var_delta_gmob_dn7 * locals.var_delta_gmob) + (locals.var_delta_gmob * locals.var_delta_gmob_dn7)) / (2.0 * assign44060_e56922)))) / (assign44060_e56923 * assign44060_e56923)))), ((locals.var_x_inf0_dn8 * assign44060_e56925) + (locals.var_x_inf0 * (((locals.var_delta_gmob_dn8 * assign44060_e56923) - (locals.var_delta_gmob * (((locals.var_delta_gmob_dn8 * locals.var_delta_gmob) + (locals.var_delta_gmob * locals.var_delta_gmob_dn8)) / (2.0 * assign44060_e56922)))) / (assign44060_e56923 * assign44060_e56923)))), ((locals.var_x_inf0_dn9 * assign44060_e56925) + (locals.var_x_inf0 * (((locals.var_delta_gmob_dn9 * assign44060_e56923) - (locals.var_delta_gmob * (((locals.var_delta_gmob_dn9 * locals.var_delta_gmob) + (locals.var_delta_gmob * locals.var_delta_gmob_dn9)) / (2.0 * assign44060_e56922)))) / (assign44060_e56923 * assign44060_e56923)))),)
    } else {
        (locals.var_x_inf, locals.var_x_inf_dn4, locals.var_x_inf_dn6, locals.var_x_inf_dn7, locals.var_x_inf_dn8, locals.var_x_inf_dn9,)
    }
};
        locals.var_x_inf = assign44060_e56928;
        locals.var_x_inf_dn4 = assign44060_e56928_d_n4;
        locals.var_x_inf_dn6 = assign44060_e56928_d_n6;
        locals.var_x_inf_dn7 = assign44060_e56928_d_n7;
        locals.var_x_inf_dn8 = assign44060_e56928_d_n8;
        locals.var_x_inf_dn9 = assign44060_e56928_d_n9;

        let (assign44070_e56937, assign44070_e56937_d_n4, assign44070_e56937_d_n6, assign44070_e56937_d_n7, assign44070_e56937_d_n8, assign44070_e56937_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1218 == 0.0)) {
        (locals.var_x_inf0, locals.var_x_inf0_dn4, locals.var_x_inf0_dn6, locals.var_x_inf0_dn7, locals.var_x_inf0_dn8, locals.var_x_inf0_dn9,)
    } else {
        (locals.var_x_inf, locals.var_x_inf_dn4, locals.var_x_inf_dn6, locals.var_x_inf_dn7, locals.var_x_inf_dn8, locals.var_x_inf_dn9,)
    }
};
        locals.var_x_inf = assign44070_e56937;
        locals.var_x_inf_dn4 = assign44070_e56937_d_n4;
        locals.var_x_inf_dn6 = assign44070_e56937_d_n6;
        locals.var_x_inf_dn7 = assign44070_e56937_d_n7;
        locals.var_x_inf_dn8 = assign44070_e56937_d_n8;
        locals.var_x_inf_dn9 = assign44070_e56937_d_n9;

        let (assign44080_e56949, assign44080_e56949_d_n4, assign44080_e56949_d_n6, assign44080_e56949_d_n7, assign44080_e56949_d_n8, assign44080_e56949_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign44080_e56943: f64 = (locals.var_phit1 * locals.var_thesat1);
        let assign44080_e56945: f64 = (assign44080_e56943 * locals.var_x_inf);
        let assign44080_e56947: f64 = (assign44080_e56945 * 0.7071067811865475);
        (assign44080_e56947, (((((locals.var_phit1_dn4 * locals.var_thesat1) + (locals.var_phit1 * locals.var_thesat1_dn4)) * locals.var_x_inf) + (assign44080_e56943 * locals.var_x_inf_dn4)) * 0.7071067811865475), (((((locals.var_phit1_dn6 * locals.var_thesat1) + (locals.var_phit1 * locals.var_thesat1_dn6)) * locals.var_x_inf) + (assign44080_e56943 * locals.var_x_inf_dn6)) * 0.7071067811865475), (((((locals.var_phit1_dn7 * locals.var_thesat1) + (locals.var_phit1 * locals.var_thesat1_dn7)) * locals.var_x_inf) + (assign44080_e56943 * locals.var_x_inf_dn7)) * 0.7071067811865475), (((((locals.var_phit1_dn8 * locals.var_thesat1) + (locals.var_phit1 * locals.var_thesat1_dn8)) * locals.var_x_inf) + (assign44080_e56943 * locals.var_x_inf_dn8)) * 0.7071067811865475), (((((locals.var_phit1_dn9 * locals.var_thesat1) + (locals.var_phit1 * locals.var_thesat1_dn9)) * locals.var_x_inf) + (assign44080_e56943 * locals.var_x_inf_dn9)) * 0.7071067811865475),)
    } else {
        (locals.var_ysat, locals.var_ysat_dn4, locals.var_ysat_dn6, locals.var_ysat_dn7, locals.var_ysat_dn8, locals.var_ysat_dn9,)
    }
};
        locals.var_ysat = assign44080_e56949;
        locals.var_ysat_dn4 = assign44080_e56949_d_n4;
        locals.var_ysat_dn6 = assign44080_e56949_d_n6;
        locals.var_ysat_dn7 = assign44080_e56949_d_n7;
        locals.var_ysat_dn8 = assign44080_e56949_d_n8;
        locals.var_ysat_dn9 = assign44080_e56949_d_n9;

        let assign44090_e56952: f64 = (-1.0);
        let assign44090_e56953: f64 = if locals.var_chnl_type == assign44090_e56952 { 1.0 } else { 0.0 };
        locals.var_guard1220 = assign44090_e56953;

    }

    pub(super) fn stamp_transient_block_23(
        locals: &mut StampLocals,
    ) {
        let (assign44100_e56966, assign44100_e56966_d_n4, assign44100_e56966_d_n6, assign44100_e56966_d_n7, assign44100_e56966_d_n8, assign44100_e56966_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) && (locals.var_guard1220 != 0.0)) {
        let assign44100_e56962: f64 = (1.0 + locals.var_ysat);
        let assign44100_e56963: f64 = (assign44100_e56962).sqrt();
        let assign44100_e56964: f64 = (locals.var_ysat / assign44100_e56963);
        (assign44100_e56964, (((locals.var_ysat_dn4 * assign44100_e56963) - (locals.var_ysat * (locals.var_ysat_dn4 / (2.0 * assign44100_e56963)))) / (assign44100_e56963 * assign44100_e56963)), (((locals.var_ysat_dn6 * assign44100_e56963) - (locals.var_ysat * (locals.var_ysat_dn6 / (2.0 * assign44100_e56963)))) / (assign44100_e56963 * assign44100_e56963)), (((locals.var_ysat_dn7 * assign44100_e56963) - (locals.var_ysat * (locals.var_ysat_dn7 / (2.0 * assign44100_e56963)))) / (assign44100_e56963 * assign44100_e56963)), (((locals.var_ysat_dn8 * assign44100_e56963) - (locals.var_ysat * (locals.var_ysat_dn8 / (2.0 * assign44100_e56963)))) / (assign44100_e56963 * assign44100_e56963)), (((locals.var_ysat_dn9 * assign44100_e56963) - (locals.var_ysat * (locals.var_ysat_dn9 / (2.0 * assign44100_e56963)))) / (assign44100_e56963 * assign44100_e56963)),)
    } else {
        (locals.var_ysat, locals.var_ysat_dn4, locals.var_ysat_dn6, locals.var_ysat_dn7, locals.var_ysat_dn8, locals.var_ysat_dn9,)
    }
};
        locals.var_ysat = assign44100_e56966;
        locals.var_ysat_dn4 = assign44100_e56966_d_n4;
        locals.var_ysat_dn6 = assign44100_e56966_d_n6;
        locals.var_ysat_dn7 = assign44100_e56966_d_n7;
        locals.var_ysat_dn8 = assign44100_e56966_d_n8;
        locals.var_ysat_dn9 = assign44100_e56966_d_n9;

        let (assign44110_e56981, assign44110_e56981_d_n4, assign44110_e56981_d_n6, assign44110_e56981_d_n7, assign44110_e56981_d_n8, assign44110_e56981_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign44110_e56975: f64 = (4.0 * locals.var_ysat);
        let assign44110_e56976: f64 = (1.0 + assign44110_e56975);
        let assign44110_e56977: f64 = (assign44110_e56976).sqrt();
        let assign44110_e56978: f64 = (1.0 + assign44110_e56977);
        let assign44110_e56979: f64 = (2.0 / assign44110_e56978);
        (assign44110_e56979, (-((2.0 * ((4.0 * locals.var_ysat_dn4) / (2.0 * assign44110_e56977))) / (assign44110_e56978 * assign44110_e56978))), (-((2.0 * ((4.0 * locals.var_ysat_dn6) / (2.0 * assign44110_e56977))) / (assign44110_e56978 * assign44110_e56978))), (-((2.0 * ((4.0 * locals.var_ysat_dn7) / (2.0 * assign44110_e56977))) / (assign44110_e56978 * assign44110_e56978))), (-((2.0 * ((4.0 * locals.var_ysat_dn8) / (2.0 * assign44110_e56977))) / (assign44110_e56978 * assign44110_e56978))), (-((2.0 * ((4.0 * locals.var_ysat_dn9) / (2.0 * assign44110_e56977))) / (assign44110_e56978 * assign44110_e56978))),)
    } else {
        (locals.var_za, locals.var_za_dn4, locals.var_za_dn6, locals.var_za_dn7, locals.var_za_dn8, locals.var_za_dn9,)
    }
};
        locals.var_za = assign44110_e56981;
        locals.var_za_dn4 = assign44110_e56981_d_n4;
        locals.var_za_dn6 = assign44110_e56981_d_n6;
        locals.var_za_dn7 = assign44110_e56981_d_n7;
        locals.var_za_dn8 = assign44110_e56981_d_n8;
        locals.var_za_dn9 = assign44110_e56981_d_n9;

        let (assign44120_e56989, assign44120_e56989_d_n4, assign44120_e56989_d_n6, assign44120_e56989_d_n7, assign44120_e56989_d_n8, assign44120_e56989_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign44120_e56987: f64 = (locals.var_za * locals.var_ysat);
        (assign44120_e56987, ((locals.var_za_dn4 * locals.var_ysat) + (locals.var_za * locals.var_ysat_dn4)), ((locals.var_za_dn6 * locals.var_ysat) + (locals.var_za * locals.var_ysat_dn6)), ((locals.var_za_dn7 * locals.var_ysat) + (locals.var_za * locals.var_ysat_dn7)), ((locals.var_za_dn8 * locals.var_ysat) + (locals.var_za * locals.var_ysat_dn8)), ((locals.var_za_dn9 * locals.var_ysat) + (locals.var_za * locals.var_ysat_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign44120_e56989;
        locals.var_temp__blk949_dn4 = assign44120_e56989_d_n4;
        locals.var_temp__blk949_dn6 = assign44120_e56989_d_n6;
        locals.var_temp__blk949_dn7 = assign44120_e56989_d_n7;
        locals.var_temp__blk949_dn8 = assign44120_e56989_d_n8;
        locals.var_temp__blk949_dn9 = assign44120_e56989_d_n9;

        let (assign44130_e57019, assign44130_e57019_d_n4, assign44130_e57019_d_n6, assign44130_e57019_d_n7, assign44130_e57019_d_n8, assign44130_e57019_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign44130_e56995: f64 = (locals.var_x_inf * locals.var_za);
        let assign44130_e56999: f64 = (0.86 * locals.var_temp__blk949);
        let assign44130_e57003: f64 = (locals.var_temp__blk949 * locals.var_za);
        let assign44130_e57004: f64 = (1.0 - assign44130_e57003);
        let assign44130_e57005: f64 = (assign44130_e56999 * assign44130_e57004);
        let assign44130_e57009: f64 = (4.0 * locals.var_temp__blk949);
        let assign44130_e57011: f64 = (assign44130_e57009 * locals.var_temp__blk949);
        let assign44130_e57013: f64 = (assign44130_e57011 * locals.var_za);
        let assign44130_e57014: f64 = (1.0 + assign44130_e57013);
        let assign44130_e57015: f64 = (assign44130_e57005 / assign44130_e57014);
        let assign44130_e57016: f64 = (1.0 + assign44130_e57015);
        let assign44130_e57017: f64 = (assign44130_e56995 * assign44130_e57016);
        (assign44130_e57017, ((((locals.var_x_inf_dn4 * locals.var_za) + (locals.var_x_inf * locals.var_za_dn4)) * assign44130_e57016) + (assign44130_e56995 * ((((((0.86 * locals.var_temp__blk949_dn4) * assign44130_e57004) + (assign44130_e56999 * (-((locals.var_temp__blk949_dn4 * locals.var_za) + (locals.var_temp__blk949 * locals.var_za_dn4))))) * assign44130_e57014) - (assign44130_e57005 * (((((4.0 * locals.var_temp__blk949_dn4) * locals.var_temp__blk949) + (assign44130_e57009 * locals.var_temp__blk949_dn4)) * locals.var_za) + (assign44130_e57011 * locals.var_za_dn4)))) / (assign44130_e57014 * assign44130_e57014)))), ((((locals.var_x_inf_dn6 * locals.var_za) + (locals.var_x_inf * locals.var_za_dn6)) * assign44130_e57016) + (assign44130_e56995 * ((((((0.86 * locals.var_temp__blk949_dn6) * assign44130_e57004) + (assign44130_e56999 * (-((locals.var_temp__blk949_dn6 * locals.var_za) + (locals.var_temp__blk949 * locals.var_za_dn6))))) * assign44130_e57014) - (assign44130_e57005 * (((((4.0 * locals.var_temp__blk949_dn6) * locals.var_temp__blk949) + (assign44130_e57009 * locals.var_temp__blk949_dn6)) * locals.var_za) + (assign44130_e57011 * locals.var_za_dn6)))) / (assign44130_e57014 * assign44130_e57014)))), ((((locals.var_x_inf_dn7 * locals.var_za) + (locals.var_x_inf * locals.var_za_dn7)) * assign44130_e57016) + (assign44130_e56995 * ((((((0.86 * locals.var_temp__blk949_dn7) * assign44130_e57004) + (assign44130_e56999 * (-((locals.var_temp__blk949_dn7 * locals.var_za) + (locals.var_temp__blk949 * locals.var_za_dn7))))) * assign44130_e57014) - (assign44130_e57005 * (((((4.0 * locals.var_temp__blk949_dn7) * locals.var_temp__blk949) + (assign44130_e57009 * locals.var_temp__blk949_dn7)) * locals.var_za) + (assign44130_e57011 * locals.var_za_dn7)))) / (assign44130_e57014 * assign44130_e57014)))), ((((locals.var_x_inf_dn8 * locals.var_za) + (locals.var_x_inf * locals.var_za_dn8)) * assign44130_e57016) + (assign44130_e56995 * ((((((0.86 * locals.var_temp__blk949_dn8) * assign44130_e57004) + (assign44130_e56999 * (-((locals.var_temp__blk949_dn8 * locals.var_za) + (locals.var_temp__blk949 * locals.var_za_dn8))))) * assign44130_e57014) - (assign44130_e57005 * (((((4.0 * locals.var_temp__blk949_dn8) * locals.var_temp__blk949) + (assign44130_e57009 * locals.var_temp__blk949_dn8)) * locals.var_za) + (assign44130_e57011 * locals.var_za_dn8)))) / (assign44130_e57014 * assign44130_e57014)))), ((((locals.var_x_inf_dn9 * locals.var_za) + (locals.var_x_inf * locals.var_za_dn9)) * assign44130_e57016) + (assign44130_e56995 * ((((((0.86 * locals.var_temp__blk949_dn9) * assign44130_e57004) + (assign44130_e56999 * (-((locals.var_temp__blk949_dn9 * locals.var_za) + (locals.var_temp__blk949 * locals.var_za_dn9))))) * assign44130_e57014) - (assign44130_e57005 * (((((4.0 * locals.var_temp__blk949_dn9) * locals.var_temp__blk949) + (assign44130_e57009 * locals.var_temp__blk949_dn9)) * locals.var_za) + (assign44130_e57011 * locals.var_za_dn9)))) / (assign44130_e57014 * assign44130_e57014)))),)
    } else {
        (locals.var_x_0, locals.var_x_0_dn4, locals.var_x_0_dn6, locals.var_x_0_dn7, locals.var_x_0_dn8, locals.var_x_0_dn9,)
    }
};
        locals.var_x_0 = assign44130_e57019;
        locals.var_x_0_dn4 = assign44130_e57019_d_n4;
        locals.var_x_0_dn6 = assign44130_e57019_d_n6;
        locals.var_x_0_dn7 = assign44130_e57019_d_n7;
        locals.var_x_0_dn8 = assign44130_e57019_d_n8;
        locals.var_x_0_dn9 = assign44130_e57019_d_n9;

        let (assign44140_e57027, assign44140_e57027_d_n4, assign44140_e57027_d_n6, assign44140_e57027_d_n7, assign44140_e57027_d_n8, assign44140_e57027_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign44140_e57025: f64 = (0.99 * locals.var_x_0);
        (assign44140_e57025, (0.99 * locals.var_x_0_dn4), (0.99 * locals.var_x_0_dn6), (0.99 * locals.var_x_0_dn7), (0.99 * locals.var_x_0_dn8), (0.99 * locals.var_x_0_dn9),)
    } else {
        (locals.var_x_sat, locals.var_x_sat_dn4, locals.var_x_sat_dn6, locals.var_x_sat_dn7, locals.var_x_sat_dn8, locals.var_x_sat_dn9,)
    }
};
        locals.var_x_sat = assign44140_e57027;
        locals.var_x_sat_dn4 = assign44140_e57027_d_n4;
        locals.var_x_sat_dn6 = assign44140_e57027_d_n6;
        locals.var_x_sat_dn7 = assign44140_e57027_d_n7;
        locals.var_x_sat_dn8 = assign44140_e57027_d_n8;
        locals.var_x_sat_dn9 = assign44140_e57027_d_n9;

        let (assign44150_e57043, assign44150_e57043_d_n4, assign44150_e57043_d_n6, assign44150_e57043_d_n7, assign44150_e57043_d_n8, assign44150_e57043_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign44150_e57035: f64 = (2.0 * locals.var_asat);
        let assign44150_e57036: f64 = (locals.var_x_sat - assign44150_e57035);
        let assign44150_e57037: f64 = (locals.var_x_sat * assign44150_e57036);
        let assign44150_e57039: f64 = (assign44150_e57037 * locals.var_inv_gf2);
        let assign44150_e57041: f64 = (assign44150_e57039 / locals.var_ds);
        (assign44150_e57041, (((((((locals.var_x_sat_dn4 * assign44150_e57036) + (locals.var_x_sat * (locals.var_x_sat_dn4 - (2.0 * locals.var_asat_dn4)))) * locals.var_inv_gf2) + (assign44150_e57037 * locals.var_inv_gf2_dn4)) * locals.var_ds) - (assign44150_e57039 * locals.var_ds_dn4)) / (locals.var_ds * locals.var_ds)), (((((((locals.var_x_sat_dn6 * assign44150_e57036) + (locals.var_x_sat * (locals.var_x_sat_dn6 - (2.0 * locals.var_asat_dn6)))) * locals.var_inv_gf2) + (assign44150_e57037 * locals.var_inv_gf2_dn6)) * locals.var_ds) - (assign44150_e57039 * locals.var_ds_dn6)) / (locals.var_ds * locals.var_ds)), (((((((locals.var_x_sat_dn7 * assign44150_e57036) + (locals.var_x_sat * (locals.var_x_sat_dn7 - (2.0 * locals.var_asat_dn7)))) * locals.var_inv_gf2) + (assign44150_e57037 * locals.var_inv_gf2_dn7)) * locals.var_ds) - (assign44150_e57039 * locals.var_ds_dn7)) / (locals.var_ds * locals.var_ds)), (((((((locals.var_x_sat_dn8 * assign44150_e57036) + (locals.var_x_sat * (locals.var_x_sat_dn8 - (2.0 * locals.var_asat_dn8)))) * locals.var_inv_gf2) + (assign44150_e57037 * locals.var_inv_gf2_dn8)) * locals.var_ds) - (assign44150_e57039 * locals.var_ds_dn8)) / (locals.var_ds * locals.var_ds)), (((((((locals.var_x_sat_dn9 * assign44150_e57036) + (locals.var_x_sat * (locals.var_x_sat_dn9 - (2.0 * locals.var_asat_dn9)))) * locals.var_inv_gf2) + (assign44150_e57037 * locals.var_inv_gf2_dn9)) * locals.var_ds) - (assign44150_e57039 * locals.var_ds_dn9)) / (locals.var_ds * locals.var_ds)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign44150_e57043;
        locals.var_temp__blk949_dn4 = assign44150_e57043_d_n4;
        locals.var_temp__blk949_dn6 = assign44150_e57043_d_n6;
        locals.var_temp__blk949_dn7 = assign44150_e57043_d_n7;
        locals.var_temp__blk949_dn8 = assign44150_e57043_d_n8;
        locals.var_temp__blk949_dn9 = assign44150_e57043_d_n9;

        let (assign44160_e57063, assign44160_e57063_d_n4, assign44160_e57063_d_n6, assign44160_e57063_d_n7, assign44160_e57063_d_n8, assign44160_e57063_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 != 0.0)) {
        let assign44160_e57052: f64 = (-0.99);
        let (assign44160_e57057, assign44160_e57057_d_n4, assign44160_e57057_d_n6, assign44160_e57057_d_n7, assign44160_e57057_d_n8, assign44160_e57057_d_n9,) = {
            if (locals.var_temp__blk949 > assign44160_e57052) {
                (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
            } else {
                let assign44160_e57056: f64 = (-0.99);
                (assign44160_e57056, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign44160_e57058: f64 = (1.0 + assign44160_e57057);
        let assign44160_e57059: f64 = (assign44160_e57058).ln();
        let assign44160_e57060: f64 = (locals.var_x_sat - assign44160_e57059);
        let assign44160_e57061: f64 = (locals.var_phit1 * assign44160_e57060);
        (assign44160_e57061, ((locals.var_phit1_dn4 * assign44160_e57060) + (locals.var_phit1 * (locals.var_x_sat_dn4 - (assign44160_e57057_d_n4 / assign44160_e57058)))), ((locals.var_phit1_dn6 * assign44160_e57060) + (locals.var_phit1 * (locals.var_x_sat_dn6 - (assign44160_e57057_d_n6 / assign44160_e57058)))), ((locals.var_phit1_dn7 * assign44160_e57060) + (locals.var_phit1 * (locals.var_x_sat_dn7 - (assign44160_e57057_d_n7 / assign44160_e57058)))), ((locals.var_phit1_dn8 * assign44160_e57060) + (locals.var_phit1 * (locals.var_x_sat_dn8 - (assign44160_e57057_d_n8 / assign44160_e57058)))), ((locals.var_phit1_dn9 * assign44160_e57060) + (locals.var_phit1 * (locals.var_x_sat_dn9 - (assign44160_e57057_d_n9 / assign44160_e57058)))),)
    } else {
        (locals.var_v_dsat, locals.var_v_dsat_dn4, locals.var_v_dsat_dn6, locals.var_v_dsat_dn7, locals.var_v_dsat_dn8, locals.var_v_dsat_dn9,)
    }
};
        locals.var_v_dsat = assign44160_e57063;
        locals.var_v_dsat_dn4 = assign44160_e57063_d_n4;
        locals.var_v_dsat_dn6 = assign44160_e57063_d_n6;
        locals.var_v_dsat_dn7 = assign44160_e57063_d_n7;
        locals.var_v_dsat_dn8 = assign44160_e57063_d_n8;
        locals.var_v_dsat_dn9 = assign44160_e57063_d_n9;

        let (assign44170_e57070, assign44170_e57070_d_n4, assign44170_e57070_d_n6, assign44170_e57070_d_n7, assign44170_e57070_d_n8, assign44170_e57070_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1215 == 0.0)) {
        (locals.var_vdsat_lim, locals.var_vdsat_lim_dn4, locals.var_vdsat_lim_dn6, locals.var_vdsat_lim_dn7, locals.var_vdsat_lim_dn8, locals.var_vdsat_lim_dn9,)
    } else {
        (locals.var_v_dsat, locals.var_v_dsat_dn4, locals.var_v_dsat_dn6, locals.var_v_dsat_dn7, locals.var_v_dsat_dn8, locals.var_v_dsat_dn9,)
    }
};
        locals.var_v_dsat = assign44170_e57070;
        locals.var_v_dsat_dn4 = assign44170_e57070_d_n4;
        locals.var_v_dsat_dn6 = assign44170_e57070_d_n6;
        locals.var_v_dsat_dn7 = assign44170_e57070_d_n7;
        locals.var_v_dsat_dn8 = assign44170_e57070_d_n8;
        locals.var_v_dsat_dn9 = assign44170_e57070_d_n9;

        let (assign44180_e57076, assign44180_e57076_d_n4, assign44180_e57076_d_n6, assign44180_e57076_d_n7, assign44180_e57076_d_n8, assign44180_e57076_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44180_e57074: f64 = (1.0 + locals.var_arloc);
        (assign44180_e57074, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign44180_e57076;
        locals.var_temp__blk949_dn4 = assign44180_e57076_d_n4;
        locals.var_temp__blk949_dn6 = assign44180_e57076_d_n6;
        locals.var_temp__blk949_dn7 = assign44180_e57076_d_n7;
        locals.var_temp__blk949_dn8 = assign44180_e57076_d_n8;
        locals.var_temp__blk949_dn9 = assign44180_e57076_d_n9;

        let (assign44190_e57085, assign44190_e57085_d_n4, assign44190_e57085_d_n6, assign44190_e57085_d_n7, assign44190_e57085_d_n8, assign44190_e57085_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44190_e57079: f64 = (locals.var_temp__blk949).sqrt();
        let assign44190_e57081: f64 = (assign44190_e57079 * locals.var_v_ds);
        let assign44190_e57083: f64 = (assign44190_e57081 / locals.var_v_dsat);
        (assign44190_e57083, (((((locals.var_temp__blk949_dn4 / (2.0 * assign44190_e57079)) * locals.var_v_ds) * locals.var_v_dsat) - (assign44190_e57081 * locals.var_v_dsat_dn4)) / (locals.var_v_dsat * locals.var_v_dsat)), (((((locals.var_temp__blk949_dn6 / (2.0 * assign44190_e57079)) * locals.var_v_ds) * locals.var_v_dsat) - (assign44190_e57081 * locals.var_v_dsat_dn6)) / (locals.var_v_dsat * locals.var_v_dsat)), ((((((locals.var_temp__blk949_dn7 / (2.0 * assign44190_e57079)) * locals.var_v_ds) + (assign44190_e57079 * locals.var_v_ds_dn7)) * locals.var_v_dsat) - (assign44190_e57081 * locals.var_v_dsat_dn7)) / (locals.var_v_dsat * locals.var_v_dsat)), ((((((locals.var_temp__blk949_dn8 / (2.0 * assign44190_e57079)) * locals.var_v_ds) + (assign44190_e57079 * locals.var_v_ds_dn8)) * locals.var_v_dsat) - (assign44190_e57081 * locals.var_v_dsat_dn8)) / (locals.var_v_dsat * locals.var_v_dsat)), (((((locals.var_temp__blk949_dn9 / (2.0 * assign44190_e57079)) * locals.var_v_ds) * locals.var_v_dsat) - (assign44190_e57081 * locals.var_v_dsat_dn9)) / (locals.var_v_dsat * locals.var_v_dsat)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign44190_e57085;
        locals.var_temp1_dn4 = assign44190_e57085_d_n4;
        locals.var_temp1_dn6 = assign44190_e57085_d_n6;
        locals.var_temp1_dn7 = assign44190_e57085_d_n7;
        locals.var_temp1_dn8 = assign44190_e57085_d_n8;
        locals.var_temp1_dn9 = assign44190_e57085_d_n9;

        let (assign44200_e57093, assign44200_e57093_d_n4, assign44200_e57093_d_n6, assign44200_e57093_d_n7, assign44200_e57093_d_n8, assign44200_e57093_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44200_e57089: f64 = (locals.var_temp1 * locals.var_temp1);
        let assign44200_e57091: f64 = (assign44200_e57089 + locals.var_temp__blk949);
        (assign44200_e57091, (((locals.var_temp1_dn4 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn4)) + locals.var_temp__blk949_dn4), (((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)) + locals.var_temp__blk949_dn6), (((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)) + locals.var_temp__blk949_dn7), (((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)) + locals.var_temp__blk949_dn8), (((locals.var_temp1_dn9 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn9)) + locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign44200_e57093;
        locals.var_temp2_dn4 = assign44200_e57093_d_n4;
        locals.var_temp2_dn6 = assign44200_e57093_d_n6;
        locals.var_temp2_dn7 = assign44200_e57093_d_n7;
        locals.var_temp2_dn8 = assign44200_e57093_d_n8;
        locals.var_temp2_dn9 = assign44200_e57093_d_n9;

        let (assign44210_e57099, assign44210_e57099_d_n4, assign44210_e57099_d_n6, assign44210_e57099_d_n7, assign44210_e57099_d_n8, assign44210_e57099_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44210_e57097: f64 = (2.0 * locals.var_temp1);
        (assign44210_e57097, (2.0 * locals.var_temp1_dn4), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), (2.0 * locals.var_temp1_dn9),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign44210_e57099;
        locals.var_temp__blk949_dn4 = assign44210_e57099_d_n4;
        locals.var_temp__blk949_dn6 = assign44210_e57099_d_n6;
        locals.var_temp__blk949_dn7 = assign44210_e57099_d_n7;
        locals.var_temp__blk949_dn8 = assign44210_e57099_d_n8;
        locals.var_temp__blk949_dn9 = assign44210_e57099_d_n9;

        let (assign44220_e57115, assign44220_e57115_d_n4, assign44220_e57115_d_n6, assign44220_e57115_d_n7, assign44220_e57115_d_n8, assign44220_e57115_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44220_e57103: f64 = (locals.var_v_dsat * locals.var_temp__blk949);
        let assign44220_e57106: f64 = (locals.var_temp2 - locals.var_temp__blk949);
        let assign44220_e57107: f64 = (assign44220_e57106).sqrt();
        let assign44220_e57110: f64 = (locals.var_temp2 + locals.var_temp__blk949);
        let assign44220_e57111: f64 = (assign44220_e57110).sqrt();
        let assign44220_e57112: f64 = (assign44220_e57107 + assign44220_e57111);
        let assign44220_e57113: f64 = (assign44220_e57103 / assign44220_e57112);
        (assign44220_e57113, (((((locals.var_v_dsat_dn4 * locals.var_temp__blk949) + (locals.var_v_dsat * locals.var_temp__blk949_dn4)) * assign44220_e57112) - (assign44220_e57103 * (((locals.var_temp2_dn4 - locals.var_temp__blk949_dn4) / (2.0 * assign44220_e57107)) + ((locals.var_temp2_dn4 + locals.var_temp__blk949_dn4) / (2.0 * assign44220_e57111))))) / (assign44220_e57112 * assign44220_e57112)), (((((locals.var_v_dsat_dn6 * locals.var_temp__blk949) + (locals.var_v_dsat * locals.var_temp__blk949_dn6)) * assign44220_e57112) - (assign44220_e57103 * (((locals.var_temp2_dn6 - locals.var_temp__blk949_dn6) / (2.0 * assign44220_e57107)) + ((locals.var_temp2_dn6 + locals.var_temp__blk949_dn6) / (2.0 * assign44220_e57111))))) / (assign44220_e57112 * assign44220_e57112)), (((((locals.var_v_dsat_dn7 * locals.var_temp__blk949) + (locals.var_v_dsat * locals.var_temp__blk949_dn7)) * assign44220_e57112) - (assign44220_e57103 * (((locals.var_temp2_dn7 - locals.var_temp__blk949_dn7) / (2.0 * assign44220_e57107)) + ((locals.var_temp2_dn7 + locals.var_temp__blk949_dn7) / (2.0 * assign44220_e57111))))) / (assign44220_e57112 * assign44220_e57112)), (((((locals.var_v_dsat_dn8 * locals.var_temp__blk949) + (locals.var_v_dsat * locals.var_temp__blk949_dn8)) * assign44220_e57112) - (assign44220_e57103 * (((locals.var_temp2_dn8 - locals.var_temp__blk949_dn8) / (2.0 * assign44220_e57107)) + ((locals.var_temp2_dn8 + locals.var_temp__blk949_dn8) / (2.0 * assign44220_e57111))))) / (assign44220_e57112 * assign44220_e57112)), (((((locals.var_v_dsat_dn9 * locals.var_temp__blk949) + (locals.var_v_dsat * locals.var_temp__blk949_dn9)) * assign44220_e57112) - (assign44220_e57103 * (((locals.var_temp2_dn9 - locals.var_temp__blk949_dn9) / (2.0 * assign44220_e57107)) + ((locals.var_temp2_dn9 + locals.var_temp__blk949_dn9) / (2.0 * assign44220_e57111))))) / (assign44220_e57112 * assign44220_e57112)),)
    } else {
        (locals.var_vdse, locals.var_vdse_dn4, locals.var_vdse_dn6, locals.var_vdse_dn7, locals.var_vdse_dn8, locals.var_vdse_dn9,)
    }
};
        locals.var_vdse = assign44220_e57115;
        locals.var_vdse_dn4 = assign44220_e57115_d_n4;
        locals.var_vdse_dn6 = assign44220_e57115_d_n6;
        locals.var_vdse_dn7 = assign44220_e57115_d_n7;
        locals.var_vdse_dn8 = assign44220_e57115_d_n8;
        locals.var_vdse_dn9 = assign44220_e57115_d_n9;

        let (assign44230_e57121, assign44230_e57121_d_n4, assign44230_e57121_d_n6, assign44230_e57121_d_n7, assign44230_e57121_d_n8, assign44230_e57121_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44230_e57119: f64 = (locals.var_vdse * locals.var_inv_phit1);
        (assign44230_e57119, ((locals.var_vdse_dn4 * locals.var_inv_phit1) + (locals.var_vdse * locals.var_inv_phit1_dn4)), ((locals.var_vdse_dn6 * locals.var_inv_phit1) + (locals.var_vdse * locals.var_inv_phit1_dn6)), ((locals.var_vdse_dn7 * locals.var_inv_phit1) + (locals.var_vdse * locals.var_inv_phit1_dn7)), ((locals.var_vdse_dn8 * locals.var_inv_phit1) + (locals.var_vdse * locals.var_inv_phit1_dn8)), ((locals.var_vdse_dn9 * locals.var_inv_phit1) + (locals.var_vdse * locals.var_inv_phit1_dn9)),)
    } else {
        (locals.var_udse, locals.var_udse_dn4, locals.var_udse_dn6, locals.var_udse_dn7, locals.var_udse_dn8, locals.var_udse_dn9,)
    }
};
        locals.var_udse = assign44230_e57121;
        locals.var_udse_dn4 = assign44230_e57121_d_n4;
        locals.var_udse_dn6 = assign44230_e57121_d_n6;
        locals.var_udse_dn7 = assign44230_e57121_d_n7;
        locals.var_udse_dn8 = assign44230_e57121_d_n8;
        locals.var_udse_dn9 = assign44230_e57121_d_n9;

        let (assign44240_e57127, assign44240_e57127_d_n4, assign44240_e57127_d_n6, assign44240_e57127_d_n7, assign44240_e57127_d_n8, assign44240_e57127_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44240_e57125: f64 = (locals.var_xn_s + locals.var_udse);
        (assign44240_e57125, (locals.var_xn_s_dn4 + locals.var_udse_dn4), (locals.var_xn_s_dn6 + locals.var_udse_dn6), (locals.var_xn_s_dn7 + locals.var_udse_dn7), (locals.var_xn_s_dn8 + locals.var_udse_dn8), (locals.var_xn_s_dn9 + locals.var_udse_dn9),)
    } else {
        (locals.var_xn_d, locals.var_xn_d_dn4, locals.var_xn_d_dn6, locals.var_xn_d_dn7, locals.var_xn_d_dn8, locals.var_xn_d_dn9,)
    }
};
        locals.var_xn_d = assign44240_e57127;
        locals.var_xn_d_dn4 = assign44240_e57127_d_n4;
        locals.var_xn_d_dn6 = assign44240_e57127_d_n6;
        locals.var_xn_d_dn7 = assign44240_e57127_d_n7;
        locals.var_xn_d_dn8 = assign44240_e57127_d_n8;
        locals.var_xn_d_dn9 = assign44240_e57127_d_n9;

        let assign44250_e57130: f64 = if locals.var_udse < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1221 = assign44250_e57130;

        let (assign44260_e57138, assign44260_e57138_d_n4, assign44260_e57138_d_n6, assign44260_e57138_d_n7, assign44260_e57138_d_n8, assign44260_e57138_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1221 != 0.0)) {
        let assign44260_e57135: f64 = (-locals.var_udse);
        let assign44260_e57136: f64 = (assign44260_e57135).exp();
        (assign44260_e57136, (assign44260_e57136 * (-locals.var_udse_dn4)), (assign44260_e57136 * (-locals.var_udse_dn6)), (assign44260_e57136 * (-locals.var_udse_dn7)), (assign44260_e57136 * (-locals.var_udse_dn8)), (assign44260_e57136 * (-locals.var_udse_dn9)),)
    } else {
        (locals.var_k_ds, locals.var_k_ds_dn4, locals.var_k_ds_dn6, locals.var_k_ds_dn7, locals.var_k_ds_dn8, locals.var_k_ds_dn9,)
    }
};
        locals.var_k_ds = assign44260_e57138;
        locals.var_k_ds_dn4 = assign44260_e57138_d_n4;
        locals.var_k_ds_dn6 = assign44260_e57138_d_n6;
        locals.var_k_ds_dn7 = assign44260_e57138_d_n7;
        locals.var_k_ds_dn8 = assign44260_e57138_d_n8;
        locals.var_k_ds_dn9 = assign44260_e57138_d_n9;

        let (assign44270_e57167, assign44270_e57167_d_n4, assign44270_e57167_d_n6, assign44270_e57167_d_n7, assign44270_e57167_d_n8, assign44270_e57167_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1221 == 0.0)) {
        let assign44270_e57147: f64 = (locals.var_udse - 460.51701859880916);
        let assign44270_e57152: f64 = (locals.var_udse - 460.51701859880916);
        let assign44270_e57156: f64 = (locals.var_udse - 460.51701859880916);
        let assign44270_e57158: f64 = (assign44270_e57156 * 0.3333333333333333);
        let assign44270_e57159: f64 = (1.0 + assign44270_e57158);
        let assign44270_e57160: f64 = (assign44270_e57152 * assign44270_e57159);
        let assign44270_e57161: f64 = (0.5 * assign44270_e57160);
        let assign44270_e57162: f64 = (1.0 + assign44270_e57161);
        let assign44270_e57163: f64 = (assign44270_e57147 * assign44270_e57162);
        let assign44270_e57164: f64 = (1.0 + assign44270_e57163);
        let assign44270_e57165: f64 = (1e-200 / assign44270_e57164);
        (assign44270_e57165, (-((1e-200 * ((locals.var_udse_dn4 * assign44270_e57162) + (assign44270_e57147 * (0.5 * ((locals.var_udse_dn4 * assign44270_e57159) + (assign44270_e57152 * (locals.var_udse_dn4 * 0.3333333333333333))))))) / (assign44270_e57164 * assign44270_e57164))), (-((1e-200 * ((locals.var_udse_dn6 * assign44270_e57162) + (assign44270_e57147 * (0.5 * ((locals.var_udse_dn6 * assign44270_e57159) + (assign44270_e57152 * (locals.var_udse_dn6 * 0.3333333333333333))))))) / (assign44270_e57164 * assign44270_e57164))), (-((1e-200 * ((locals.var_udse_dn7 * assign44270_e57162) + (assign44270_e57147 * (0.5 * ((locals.var_udse_dn7 * assign44270_e57159) + (assign44270_e57152 * (locals.var_udse_dn7 * 0.3333333333333333))))))) / (assign44270_e57164 * assign44270_e57164))), (-((1e-200 * ((locals.var_udse_dn8 * assign44270_e57162) + (assign44270_e57147 * (0.5 * ((locals.var_udse_dn8 * assign44270_e57159) + (assign44270_e57152 * (locals.var_udse_dn8 * 0.3333333333333333))))))) / (assign44270_e57164 * assign44270_e57164))), (-((1e-200 * ((locals.var_udse_dn9 * assign44270_e57162) + (assign44270_e57147 * (0.5 * ((locals.var_udse_dn9 * assign44270_e57159) + (assign44270_e57152 * (locals.var_udse_dn9 * 0.3333333333333333))))))) / (assign44270_e57164 * assign44270_e57164))),)
    } else {
        (locals.var_k_ds, locals.var_k_ds_dn4, locals.var_k_ds_dn6, locals.var_k_ds_dn7, locals.var_k_ds_dn8, locals.var_k_ds_dn9,)
    }
};
        locals.var_k_ds = assign44270_e57167;
        locals.var_k_ds_dn4 = assign44270_e57167_d_n4;
        locals.var_k_ds_dn6 = assign44270_e57167_d_n6;
        locals.var_k_ds_dn7 = assign44270_e57167_d_n7;
        locals.var_k_ds_dn8 = assign44270_e57167_d_n8;
        locals.var_k_ds_dn9 = assign44270_e57167_d_n9;

        let (assign44280_e57173, assign44280_e57173_d_n4, assign44280_e57173_d_n6, assign44280_e57173_d_n7, assign44280_e57173_d_n8, assign44280_e57173_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44280_e57171: f64 = (locals.var_delta_ns * locals.var_k_ds);
        (assign44280_e57171, ((locals.var_delta_ns_dn4 * locals.var_k_ds) + (locals.var_delta_ns * locals.var_k_ds_dn4)), ((locals.var_delta_ns_dn6 * locals.var_k_ds) + (locals.var_delta_ns * locals.var_k_ds_dn6)), ((locals.var_delta_ns_dn7 * locals.var_k_ds) + (locals.var_delta_ns * locals.var_k_ds_dn7)), ((locals.var_delta_ns_dn8 * locals.var_k_ds) + (locals.var_delta_ns * locals.var_k_ds_dn8)), ((locals.var_delta_ns_dn9 * locals.var_k_ds) + (locals.var_delta_ns * locals.var_k_ds_dn9)),)
    } else {
        (locals.var_delta_nd, locals.var_delta_nd_dn4, locals.var_delta_nd_dn6, locals.var_delta_nd_dn7, locals.var_delta_nd_dn8, locals.var_delta_nd_dn9,)
    }
};
        locals.var_delta_nd = assign44280_e57173;
        locals.var_delta_nd_dn4 = assign44280_e57173_d_n4;
        locals.var_delta_nd_dn6 = assign44280_e57173_d_n6;
        locals.var_delta_nd_dn7 = assign44280_e57173_d_n7;
        locals.var_delta_nd_dn8 = assign44280_e57173_d_n8;
        locals.var_delta_nd_dn9 = assign44280_e57173_d_n9;

        let assign44290_e57175: f64 = (locals.var_xg).abs();
        let assign44290_e57177: f64 = if assign44290_e57175 <= locals.var_margin { 1.0 } else { 0.0 };
        locals.var_guard1222 = assign44290_e57177;

        let (assign44300_e57189, assign44300_e57189_d_n4, assign44300_e57189_d_n6, assign44300_e57189_d_n7, assign44300_e57189_d_n8, assign44300_e57189_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign44300_e57183: f64 = (locals.var_inv_xi * locals.var_inv_xi);
        let assign44300_e57185: f64 = (assign44300_e57183 * 0.16666666666666666);
        let assign44300_e57187: f64 = (assign44300_e57185 * 0.7071067811865475);
        (assign44300_e57187, ((((locals.var_inv_xi_dn4 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn4)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn6 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn7 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn8 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn8)) * 0.16666666666666666) * 0.7071067811865475), ((((locals.var_inv_xi_dn9 * locals.var_inv_xi) + (locals.var_inv_xi * locals.var_inv_xi_dn9)) * 0.16666666666666666) * 0.7071067811865475),)
    } else {
        (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn4, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8, locals.var_sp_s_temp1_dn9,)
    }
};
        locals.var_sp_s_temp1 = assign44300_e57189;
        locals.var_sp_s_temp1_dn4 = assign44300_e57189_d_n4;
        locals.var_sp_s_temp1_dn6 = assign44300_e57189_d_n6;
        locals.var_sp_s_temp1_dn7 = assign44300_e57189_d_n7;
        locals.var_sp_s_temp1_dn8 = assign44300_e57189_d_n8;
        locals.var_sp_s_temp1_dn9 = assign44300_e57189_d_n9;

        let (assign44310_e57209, assign44310_e57209_d_n4, assign44310_e57209_d_n6, assign44310_e57209_d_n7, assign44310_e57209_d_n8, assign44310_e57209_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign44310_e57195: f64 = (locals.var_xg * locals.var_inv_xi);
        let assign44310_e57200: f64 = (1.0 - locals.var_delta_nd);
        let assign44310_e57201: f64 = (locals.var_xg * assign44310_e57200);
        let assign44310_e57203: f64 = (assign44310_e57201 * locals.var_gf);
        let assign44310_e57205: f64 = (assign44310_e57203 * locals.var_sp_s_temp1);
        let assign44310_e57206: f64 = (1.0 + assign44310_e57205);
        let assign44310_e57207: f64 = (assign44310_e57195 * assign44310_e57206);
        (assign44310_e57207, ((((locals.var_xg_dn4 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn4)) * assign44310_e57206) + (assign44310_e57195 * ((((((locals.var_xg_dn4 * assign44310_e57200) + (locals.var_xg * (-locals.var_delta_nd_dn4))) * locals.var_gf) + (assign44310_e57201 * locals.var_gf_dn4)) * locals.var_sp_s_temp1) + (assign44310_e57203 * locals.var_sp_s_temp1_dn4)))), ((((locals.var_xg_dn6 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn6)) * assign44310_e57206) + (assign44310_e57195 * ((((((locals.var_xg_dn6 * assign44310_e57200) + (locals.var_xg * (-locals.var_delta_nd_dn6))) * locals.var_gf) + (assign44310_e57201 * locals.var_gf_dn6)) * locals.var_sp_s_temp1) + (assign44310_e57203 * locals.var_sp_s_temp1_dn6)))), ((((locals.var_xg_dn7 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn7)) * assign44310_e57206) + (assign44310_e57195 * ((((((locals.var_xg_dn7 * assign44310_e57200) + (locals.var_xg * (-locals.var_delta_nd_dn7))) * locals.var_gf) + (assign44310_e57201 * locals.var_gf_dn7)) * locals.var_sp_s_temp1) + (assign44310_e57203 * locals.var_sp_s_temp1_dn7)))), ((((locals.var_xg_dn8 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn8)) * assign44310_e57206) + (assign44310_e57195 * ((((((locals.var_xg_dn8 * assign44310_e57200) + (locals.var_xg * (-locals.var_delta_nd_dn8))) * locals.var_gf) + (assign44310_e57201 * locals.var_gf_dn8)) * locals.var_sp_s_temp1) + (assign44310_e57203 * locals.var_sp_s_temp1_dn8)))), ((((locals.var_xg_dn9 * locals.var_inv_xi) + (locals.var_xg * locals.var_inv_xi_dn9)) * assign44310_e57206) + (assign44310_e57195 * ((((((locals.var_xg_dn9 * assign44310_e57200) + (locals.var_xg * (-locals.var_delta_nd_dn9))) * locals.var_gf) + (assign44310_e57201 * locals.var_gf_dn9)) * locals.var_sp_s_temp1) + (assign44310_e57203 * locals.var_sp_s_temp1_dn9)))),)
    } else {
        (locals.var_x_d, locals.var_x_d_dn4, locals.var_x_d_dn6, locals.var_x_d_dn7, locals.var_x_d_dn8, locals.var_x_d_dn9,)
    }
};
        locals.var_x_d = assign44310_e57209;
        locals.var_x_d_dn4 = assign44310_e57209_d_n4;
        locals.var_x_d_dn6 = assign44310_e57209_d_n6;
        locals.var_x_d_dn7 = assign44310_e57209_d_n7;
        locals.var_x_d_dn8 = assign44310_e57209_d_n8;
        locals.var_x_d_dn9 = assign44310_e57209_d_n9;

        let (assign44320_e57218, assign44320_e57218_d_n4, assign44320_e57218_d_n6, assign44320_e57218_d_n7, assign44320_e57218_d_n8, assign44320_e57218_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44320_e57216: f64 = (locals.var_xn_d + 3.0);
        (assign44320_e57216, locals.var_xn_d_dn4, locals.var_xn_d_dn6, locals.var_xn_d_dn7, locals.var_xn_d_dn8, locals.var_xn_d_dn9,)
    } else {
        (locals.var_sp_s_bx, locals.var_sp_s_bx_dn4, locals.var_sp_s_bx_dn6, locals.var_sp_s_bx_dn7, locals.var_sp_s_bx_dn8, locals.var_sp_s_bx_dn9,)
    }
};
        locals.var_sp_s_bx = assign44320_e57218;
        locals.var_sp_s_bx_dn4 = assign44320_e57218_d_n4;
        locals.var_sp_s_bx_dn6 = assign44320_e57218_d_n6;
        locals.var_sp_s_bx_dn7 = assign44320_e57218_d_n7;
        locals.var_sp_s_bx_dn8 = assign44320_e57218_d_n8;
        locals.var_sp_s_bx_dn9 = assign44320_e57218_d_n9;

        let (assign44330_e57251, assign44330_e57251_d_n4, assign44330_e57251_d_n6, assign44330_e57251_d_n7, assign44330_e57251_d_n8, assign44330_e57251_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44330_e57226: f64 = (locals.var_sp_s_x1 + locals.var_sp_s_bx);
        let assign44330_e57229: f64 = (locals.var_sp_s_x1 - locals.var_sp_s_bx);
        let assign44330_e57232: f64 = (locals.var_sp_s_x1 - locals.var_sp_s_bx);
        let assign44330_e57233: f64 = (assign44330_e57229 * assign44330_e57232);
        let assign44330_e57235: f64 = (assign44330_e57233 + 5.0);
        let assign44330_e57236: f64 = (assign44330_e57235).sqrt();
        let assign44330_e57237: f64 = (assign44330_e57226 - assign44330_e57236);
        let assign44330_e57238: f64 = (0.5 * assign44330_e57237);
        let assign44330_e57243: f64 = (locals.var_sp_s_bx * locals.var_sp_s_bx);
        let assign44330_e57245: f64 = (assign44330_e57243 + 5.0);
        let assign44330_e57246: f64 = (assign44330_e57245).sqrt();
        let assign44330_e57247: f64 = (locals.var_sp_s_bx - assign44330_e57246);
        let assign44330_e57248: f64 = (0.5 * assign44330_e57247);
        let assign44330_e57249: f64 = (assign44330_e57238 - assign44330_e57248);
        (assign44330_e57249, ((0.5 * ((locals.var_sp_s_x1_dn4 + locals.var_sp_s_bx_dn4) - ((((locals.var_sp_s_x1_dn4 - locals.var_sp_s_bx_dn4) * assign44330_e57232) + (assign44330_e57229 * (locals.var_sp_s_x1_dn4 - locals.var_sp_s_bx_dn4))) / (2.0 * assign44330_e57236)))) - (0.5 * (locals.var_sp_s_bx_dn4 - (((locals.var_sp_s_bx_dn4 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn4)) / (2.0 * assign44330_e57246))))), ((0.5 * ((locals.var_sp_s_x1_dn6 + locals.var_sp_s_bx_dn6) - ((((locals.var_sp_s_x1_dn6 - locals.var_sp_s_bx_dn6) * assign44330_e57232) + (assign44330_e57229 * (locals.var_sp_s_x1_dn6 - locals.var_sp_s_bx_dn6))) / (2.0 * assign44330_e57236)))) - (0.5 * (locals.var_sp_s_bx_dn6 - (((locals.var_sp_s_bx_dn6 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn6)) / (2.0 * assign44330_e57246))))), ((0.5 * ((locals.var_sp_s_x1_dn7 + locals.var_sp_s_bx_dn7) - ((((locals.var_sp_s_x1_dn7 - locals.var_sp_s_bx_dn7) * assign44330_e57232) + (assign44330_e57229 * (locals.var_sp_s_x1_dn7 - locals.var_sp_s_bx_dn7))) / (2.0 * assign44330_e57236)))) - (0.5 * (locals.var_sp_s_bx_dn7 - (((locals.var_sp_s_bx_dn7 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn7)) / (2.0 * assign44330_e57246))))), ((0.5 * ((locals.var_sp_s_x1_dn8 + locals.var_sp_s_bx_dn8) - ((((locals.var_sp_s_x1_dn8 - locals.var_sp_s_bx_dn8) * assign44330_e57232) + (assign44330_e57229 * (locals.var_sp_s_x1_dn8 - locals.var_sp_s_bx_dn8))) / (2.0 * assign44330_e57236)))) - (0.5 * (locals.var_sp_s_bx_dn8 - (((locals.var_sp_s_bx_dn8 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn8)) / (2.0 * assign44330_e57246))))), ((0.5 * ((locals.var_sp_s_x1_dn9 + locals.var_sp_s_bx_dn9) - ((((locals.var_sp_s_x1_dn9 - locals.var_sp_s_bx_dn9) * assign44330_e57232) + (assign44330_e57229 * (locals.var_sp_s_x1_dn9 - locals.var_sp_s_bx_dn9))) / (2.0 * assign44330_e57236)))) - (0.5 * (locals.var_sp_s_bx_dn9 - (((locals.var_sp_s_bx_dn9 * locals.var_sp_s_bx) + (locals.var_sp_s_bx * locals.var_sp_s_bx_dn9)) / (2.0 * assign44330_e57246))))),)
    } else {
        (locals.var_sp_s_eta, locals.var_sp_s_eta_dn4, locals.var_sp_s_eta_dn6, locals.var_sp_s_eta_dn7, locals.var_sp_s_eta_dn8, locals.var_sp_s_eta_dn9,)
    }
};
        locals.var_sp_s_eta = assign44330_e57251;
        locals.var_sp_s_eta_dn4 = assign44330_e57251_d_n4;
        locals.var_sp_s_eta_dn6 = assign44330_e57251_d_n6;
        locals.var_sp_s_eta_dn7 = assign44330_e57251_d_n7;
        locals.var_sp_s_eta_dn8 = assign44330_e57251_d_n8;
        locals.var_sp_s_eta_dn9 = assign44330_e57251_d_n9;

        let (assign44340_e57260, assign44340_e57260_d_n4, assign44340_e57260_d_n6, assign44340_e57260_d_n7, assign44340_e57260_d_n8, assign44340_e57260_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44340_e57258: f64 = (locals.var_xg - locals.var_sp_s_eta);
        (assign44340_e57258, (locals.var_xg_dn4 - locals.var_sp_s_eta_dn4), (locals.var_xg_dn6 - locals.var_sp_s_eta_dn6), (locals.var_xg_dn7 - locals.var_sp_s_eta_dn7), (locals.var_xg_dn8 - locals.var_sp_s_eta_dn8), (locals.var_xg_dn9 - locals.var_sp_s_eta_dn9),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign44340_e57260;
        locals.var_sp_s_temp_dn4 = assign44340_e57260_d_n4;
        locals.var_sp_s_temp_dn6 = assign44340_e57260_d_n6;
        locals.var_sp_s_temp_dn7 = assign44340_e57260_d_n7;
        locals.var_sp_s_temp_dn8 = assign44340_e57260_d_n8;
        locals.var_sp_s_temp_dn9 = assign44340_e57260_d_n9;

        let (assign44350_e57269, assign44350_e57269_d_n4, assign44350_e57269_d_n6, assign44350_e57269_d_n7, assign44350_e57269_d_n8, assign44350_e57269_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44350_e57266: f64 = (-locals.var_sp_s_eta);
        let assign44350_e57267: f64 = (assign44350_e57266).exp();
        (assign44350_e57267, (assign44350_e57267 * (-locals.var_sp_s_eta_dn4)), (assign44350_e57267 * (-locals.var_sp_s_eta_dn6)), (assign44350_e57267 * (-locals.var_sp_s_eta_dn7)), (assign44350_e57267 * (-locals.var_sp_s_eta_dn8)), (assign44350_e57267 * (-locals.var_sp_s_eta_dn9)),)
    } else {
        (locals.var_sp_s_temp1, locals.var_sp_s_temp1_dn4, locals.var_sp_s_temp1_dn6, locals.var_sp_s_temp1_dn7, locals.var_sp_s_temp1_dn8, locals.var_sp_s_temp1_dn9,)
    }
};
        locals.var_sp_s_temp1 = assign44350_e57269;
        locals.var_sp_s_temp1_dn4 = assign44350_e57269_d_n4;
        locals.var_sp_s_temp1_dn6 = assign44350_e57269_d_n6;
        locals.var_sp_s_temp1_dn7 = assign44350_e57269_d_n7;
        locals.var_sp_s_temp1_dn8 = assign44350_e57269_d_n8;
        locals.var_sp_s_temp1_dn9 = assign44350_e57269_d_n9;

        let (assign44360_e57282, assign44360_e57282_d_n4, assign44360_e57282_d_n6, assign44360_e57282_d_n7, assign44360_e57282_d_n8, assign44360_e57282_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44360_e57278: f64 = (locals.var_sp_s_eta * locals.var_sp_s_eta);
        let assign44360_e57279: f64 = (2.0 + assign44360_e57278);
        let assign44360_e57280: f64 = (1.0 / assign44360_e57279);
        (assign44360_e57280, (-(((locals.var_sp_s_eta_dn4 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn4)) / (assign44360_e57279 * assign44360_e57279))), (-(((locals.var_sp_s_eta_dn6 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn6)) / (assign44360_e57279 * assign44360_e57279))), (-(((locals.var_sp_s_eta_dn7 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn7)) / (assign44360_e57279 * assign44360_e57279))), (-(((locals.var_sp_s_eta_dn8 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn8)) / (assign44360_e57279 * assign44360_e57279))), (-(((locals.var_sp_s_eta_dn9 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn9)) / (assign44360_e57279 * assign44360_e57279))),)
    } else {
        (locals.var_sp_s_temp2, locals.var_sp_s_temp2_dn4, locals.var_sp_s_temp2_dn6, locals.var_sp_s_temp2_dn7, locals.var_sp_s_temp2_dn8, locals.var_sp_s_temp2_dn9,)
    }
};
        locals.var_sp_s_temp2 = assign44360_e57282;
        locals.var_sp_s_temp2_dn4 = assign44360_e57282_d_n4;
        locals.var_sp_s_temp2_dn6 = assign44360_e57282_d_n6;
        locals.var_sp_s_temp2_dn7 = assign44360_e57282_d_n7;
        locals.var_sp_s_temp2_dn8 = assign44360_e57282_d_n8;
        locals.var_sp_s_temp2_dn9 = assign44360_e57282_d_n9;

        let (assign44370_e57293, assign44370_e57293_d_n4, assign44370_e57293_d_n6, assign44370_e57293_d_n7, assign44370_e57293_d_n8, assign44370_e57293_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44370_e57289: f64 = (locals.var_sp_s_eta * locals.var_sp_s_eta);
        let assign44370_e57291: f64 = (assign44370_e57289 * locals.var_sp_s_temp2);
        (assign44370_e57291, ((((locals.var_sp_s_eta_dn4 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn4)) * locals.var_sp_s_temp2) + (assign44370_e57289 * locals.var_sp_s_temp2_dn4)), ((((locals.var_sp_s_eta_dn6 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn6)) * locals.var_sp_s_temp2) + (assign44370_e57289 * locals.var_sp_s_temp2_dn6)), ((((locals.var_sp_s_eta_dn7 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn7)) * locals.var_sp_s_temp2) + (assign44370_e57289 * locals.var_sp_s_temp2_dn7)), ((((locals.var_sp_s_eta_dn8 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn8)) * locals.var_sp_s_temp2) + (assign44370_e57289 * locals.var_sp_s_temp2_dn8)), ((((locals.var_sp_s_eta_dn9 * locals.var_sp_s_eta) + (locals.var_sp_s_eta * locals.var_sp_s_eta_dn9)) * locals.var_sp_s_temp2) + (assign44370_e57289 * locals.var_sp_s_temp2_dn9)),)
    } else {
        (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn4, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8, locals.var_sp_s_xi0_dn9,)
    }
};
        locals.var_sp_s_xi0 = assign44370_e57293;
        locals.var_sp_s_xi0_dn4 = assign44370_e57293_d_n4;
        locals.var_sp_s_xi0_dn6 = assign44370_e57293_d_n6;
        locals.var_sp_s_xi0_dn7 = assign44370_e57293_d_n7;
        locals.var_sp_s_xi0_dn8 = assign44370_e57293_d_n8;
        locals.var_sp_s_xi0_dn9 = assign44370_e57293_d_n9;

        let (assign44380_e57306, assign44380_e57306_d_n4, assign44380_e57306_d_n6, assign44380_e57306_d_n7, assign44380_e57306_d_n8, assign44380_e57306_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44380_e57301: f64 = (locals.var_sp_s_eta * locals.var_sp_s_temp2);
        let assign44380_e57303: f64 = (assign44380_e57301 * locals.var_sp_s_temp2);
        let assign44380_e57304: f64 = (4.0 * assign44380_e57303);
        (assign44380_e57304, (4.0 * ((((locals.var_sp_s_eta_dn4 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn4)) * locals.var_sp_s_temp2) + (assign44380_e57301 * locals.var_sp_s_temp2_dn4))), (4.0 * ((((locals.var_sp_s_eta_dn6 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn6)) * locals.var_sp_s_temp2) + (assign44380_e57301 * locals.var_sp_s_temp2_dn6))), (4.0 * ((((locals.var_sp_s_eta_dn7 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn7)) * locals.var_sp_s_temp2) + (assign44380_e57301 * locals.var_sp_s_temp2_dn7))), (4.0 * ((((locals.var_sp_s_eta_dn8 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn8)) * locals.var_sp_s_temp2) + (assign44380_e57301 * locals.var_sp_s_temp2_dn8))), (4.0 * ((((locals.var_sp_s_eta_dn9 * locals.var_sp_s_temp2) + (locals.var_sp_s_eta * locals.var_sp_s_temp2_dn9)) * locals.var_sp_s_temp2) + (assign44380_e57301 * locals.var_sp_s_temp2_dn9))),)
    } else {
        (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn4, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8, locals.var_sp_s_xi1_dn9,)
    }
};
        locals.var_sp_s_xi1 = assign44380_e57306;
        locals.var_sp_s_xi1_dn4 = assign44380_e57306_d_n4;
        locals.var_sp_s_xi1_dn6 = assign44380_e57306_d_n6;
        locals.var_sp_s_xi1_dn7 = assign44380_e57306_d_n7;
        locals.var_sp_s_xi1_dn8 = assign44380_e57306_d_n8;
        locals.var_sp_s_xi1_dn9 = assign44380_e57306_d_n9;

        let (assign44390_e57323, assign44390_e57323_d_n4, assign44390_e57323_d_n6, assign44390_e57323_d_n7, assign44390_e57323_d_n8, assign44390_e57323_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44390_e57313: f64 = (8.0 * locals.var_sp_s_temp2);
        let assign44390_e57316: f64 = (12.0 * locals.var_sp_s_xi0);
        let assign44390_e57317: f64 = (assign44390_e57313 - assign44390_e57316);
        let assign44390_e57319: f64 = (assign44390_e57317 * locals.var_sp_s_temp2);
        let assign44390_e57321: f64 = (assign44390_e57319 * locals.var_sp_s_temp2);
        (assign44390_e57321, ((((((8.0 * locals.var_sp_s_temp2_dn4) - (12.0 * locals.var_sp_s_xi0_dn4)) * locals.var_sp_s_temp2) + (assign44390_e57317 * locals.var_sp_s_temp2_dn4)) * locals.var_sp_s_temp2) + (assign44390_e57319 * locals.var_sp_s_temp2_dn4)), ((((((8.0 * locals.var_sp_s_temp2_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp2) + (assign44390_e57317 * locals.var_sp_s_temp2_dn6)) * locals.var_sp_s_temp2) + (assign44390_e57319 * locals.var_sp_s_temp2_dn6)), ((((((8.0 * locals.var_sp_s_temp2_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp2) + (assign44390_e57317 * locals.var_sp_s_temp2_dn7)) * locals.var_sp_s_temp2) + (assign44390_e57319 * locals.var_sp_s_temp2_dn7)), ((((((8.0 * locals.var_sp_s_temp2_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp2) + (assign44390_e57317 * locals.var_sp_s_temp2_dn8)) * locals.var_sp_s_temp2) + (assign44390_e57319 * locals.var_sp_s_temp2_dn8)), ((((((8.0 * locals.var_sp_s_temp2_dn9) - (12.0 * locals.var_sp_s_xi0_dn9)) * locals.var_sp_s_temp2) + (assign44390_e57317 * locals.var_sp_s_temp2_dn9)) * locals.var_sp_s_temp2) + (assign44390_e57319 * locals.var_sp_s_temp2_dn9)),)
    } else {
        (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn4, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8, locals.var_sp_s_xi2_dn9,)
    }
};
        locals.var_sp_s_xi2 = assign44390_e57323;
        locals.var_sp_s_xi2_dn4 = assign44390_e57323_d_n4;
        locals.var_sp_s_xi2_dn6 = assign44390_e57323_d_n6;
        locals.var_sp_s_xi2_dn7 = assign44390_e57323_d_n7;
        locals.var_sp_s_xi2_dn8 = assign44390_e57323_d_n8;
        locals.var_sp_s_xi2_dn9 = assign44390_e57323_d_n9;

    }

    pub(super) fn stamp_transient_block_24(
        locals: &mut StampLocals,
    ) {
        let (assign44400_e57371, assign44400_e57371_d_n4, assign44400_e57371_d_n6, assign44400_e57371_d_n7, assign44400_e57371_d_n8, assign44400_e57371_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44400_e57331: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign44400_e57335: f64 = (locals.var_sp_s_temp1 + locals.var_sp_s_eta);
        let assign44400_e57337: f64 = (assign44400_e57335 - 1.0);
        let assign44400_e57341: f64 = (locals.var_sp_s_eta + 1.0);
        let assign44400_e57343: f64 = (assign44400_e57341 + locals.var_sp_s_xi0);
        let assign44400_e57344: f64 = (locals.var_delta_nd * assign44400_e57343);
        let assign44400_e57345: f64 = (assign44400_e57337 - assign44400_e57344);
        let assign44400_e57346: f64 = (locals.var_gf2 * assign44400_e57345);
        let assign44400_e57347: f64 = (assign44400_e57331 - assign44400_e57346);
        let (assign44400_e57369, assign44400_e57369_d_n4, assign44400_e57369_d_n6, assign44400_e57369_d_n7, assign44400_e57369_d_n8, assign44400_e57369_d_n9,) = {
            if (1e-40 > assign44400_e57347) {
                (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign44400_e57352: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
                let assign44400_e57356: f64 = (locals.var_sp_s_temp1 + locals.var_sp_s_eta);
                let assign44400_e57358: f64 = (assign44400_e57356 - 1.0);
                let assign44400_e57362: f64 = (locals.var_sp_s_eta + 1.0);
                let assign44400_e57364: f64 = (assign44400_e57362 + locals.var_sp_s_xi0);
                let assign44400_e57365: f64 = (locals.var_delta_nd * assign44400_e57364);
                let assign44400_e57366: f64 = (assign44400_e57358 - assign44400_e57365);
                let assign44400_e57367: f64 = (locals.var_gf2 * assign44400_e57366);
                let assign44400_e57368: f64 = (assign44400_e57352 - assign44400_e57367);
                (assign44400_e57368, (((locals.var_sp_s_temp_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn4)) - ((locals.var_gf2_dn4 * assign44400_e57366) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn4 + locals.var_sp_s_eta_dn4) - ((locals.var_delta_nd_dn4 * assign44400_e57364) + (locals.var_delta_nd * (locals.var_sp_s_eta_dn4 + locals.var_sp_s_xi0_dn4))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign44400_e57366) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn6 + locals.var_sp_s_eta_dn6) - ((locals.var_delta_nd_dn6 * assign44400_e57364) + (locals.var_delta_nd * (locals.var_sp_s_eta_dn6 + locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign44400_e57366) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn7 + locals.var_sp_s_eta_dn7) - ((locals.var_delta_nd_dn7 * assign44400_e57364) + (locals.var_delta_nd * (locals.var_sp_s_eta_dn7 + locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign44400_e57366) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn8 + locals.var_sp_s_eta_dn8) - ((locals.var_delta_nd_dn8 * assign44400_e57364) + (locals.var_delta_nd * (locals.var_sp_s_eta_dn8 + locals.var_sp_s_xi0_dn8))))))), (((locals.var_sp_s_temp_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn9)) - ((locals.var_gf2_dn9 * assign44400_e57366) + (locals.var_gf2 * ((locals.var_sp_s_temp1_dn9 + locals.var_sp_s_eta_dn9) - ((locals.var_delta_nd_dn9 * assign44400_e57364) + (locals.var_delta_nd * (locals.var_sp_s_eta_dn9 + locals.var_sp_s_xi0_dn9))))))),)
            }
        };
        (assign44400_e57369, assign44400_e57369_d_n4, assign44400_e57369_d_n6, assign44400_e57369_d_n7, assign44400_e57369_d_n8, assign44400_e57369_d_n9,)
    } else {
        (locals.var_sp_s_a, locals.var_sp_s_a_dn4, locals.var_sp_s_a_dn6, locals.var_sp_s_a_dn7, locals.var_sp_s_a_dn8, locals.var_sp_s_a_dn9,)
    }
};
        locals.var_sp_s_a = assign44400_e57371;
        locals.var_sp_s_a_dn4 = assign44400_e57371_d_n4;
        locals.var_sp_s_a_dn6 = assign44400_e57371_d_n6;
        locals.var_sp_s_a_dn7 = assign44400_e57371_d_n7;
        locals.var_sp_s_a_dn8 = assign44400_e57371_d_n8;
        locals.var_sp_s_a_dn9 = assign44400_e57371_d_n9;

        let (assign44410_e57388, assign44410_e57388_d_n4, assign44410_e57388_d_n6, assign44410_e57388_d_n7, assign44410_e57388_d_n8, assign44410_e57388_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44410_e57382: f64 = (locals.var_delta_nd * locals.var_sp_s_xi2);
        let assign44410_e57383: f64 = (locals.var_sp_s_temp1 - assign44410_e57382);
        let assign44410_e57384: f64 = (locals.var_gf2 * assign44410_e57383);
        let assign44410_e57385: f64 = (0.5 * assign44410_e57384);
        let assign44410_e57386: f64 = (1.0 - assign44410_e57385);
        (assign44410_e57386, (-(0.5 * ((locals.var_gf2_dn4 * assign44410_e57383) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn4 - ((locals.var_delta_nd_dn4 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn4))))))), (-(0.5 * ((locals.var_gf2_dn6 * assign44410_e57383) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn6 - ((locals.var_delta_nd_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn6))))))), (-(0.5 * ((locals.var_gf2_dn7 * assign44410_e57383) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn7 - ((locals.var_delta_nd_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn7))))))), (-(0.5 * ((locals.var_gf2_dn8 * assign44410_e57383) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn8 - ((locals.var_delta_nd_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn8))))))), (-(0.5 * ((locals.var_gf2_dn9 * assign44410_e57383) + (locals.var_gf2 * (locals.var_sp_s_temp1_dn9 - ((locals.var_delta_nd_dn9 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn9))))))),)
    } else {
        (locals.var_sp_s_b, locals.var_sp_s_b_dn4, locals.var_sp_s_b_dn6, locals.var_sp_s_b_dn7, locals.var_sp_s_b_dn8, locals.var_sp_s_b_dn9,)
    }
};
        locals.var_sp_s_b = assign44410_e57388;
        locals.var_sp_s_b_dn4 = assign44410_e57388_d_n4;
        locals.var_sp_s_b_dn6 = assign44410_e57388_d_n6;
        locals.var_sp_s_b_dn7 = assign44410_e57388_d_n7;
        locals.var_sp_s_b_dn8 = assign44410_e57388_d_n8;
        locals.var_sp_s_b_dn9 = assign44410_e57388_d_n9;

        let (assign44420_e57409, assign44420_e57409_d_n4, assign44420_e57409_d_n6, assign44420_e57409_d_n7, assign44420_e57409_d_n8, assign44420_e57409_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44420_e57395: f64 = (2.0 * locals.var_sp_s_temp);
        let assign44420_e57399: f64 = (1.0 - locals.var_sp_s_temp1);
        let assign44420_e57403: f64 = (1.0 + locals.var_sp_s_xi1);
        let assign44420_e57404: f64 = (locals.var_delta_nd * assign44420_e57403);
        let assign44420_e57405: f64 = (assign44420_e57399 - assign44420_e57404);
        let assign44420_e57406: f64 = (locals.var_gf2 * assign44420_e57405);
        let assign44420_e57407: f64 = (assign44420_e57395 + assign44420_e57406);
        (assign44420_e57407, ((2.0 * locals.var_sp_s_temp_dn4) + ((locals.var_gf2_dn4 * assign44420_e57405) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn4) - ((locals.var_delta_nd_dn4 * assign44420_e57403) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn4)))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign44420_e57405) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn6) - ((locals.var_delta_nd_dn6 * assign44420_e57403) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn6)))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign44420_e57405) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn7) - ((locals.var_delta_nd_dn7 * assign44420_e57403) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn7)))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign44420_e57405) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn8) - ((locals.var_delta_nd_dn8 * assign44420_e57403) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn8)))))), ((2.0 * locals.var_sp_s_temp_dn9) + ((locals.var_gf2_dn9 * assign44420_e57405) + (locals.var_gf2 * ((-locals.var_sp_s_temp1_dn9) - ((locals.var_delta_nd_dn9 * assign44420_e57403) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn9)))))),)
    } else {
        (locals.var_sp_s_c, locals.var_sp_s_c_dn4, locals.var_sp_s_c_dn6, locals.var_sp_s_c_dn7, locals.var_sp_s_c_dn8, locals.var_sp_s_c_dn9,)
    }
};
        locals.var_sp_s_c = assign44420_e57409;
        locals.var_sp_s_c_dn4 = assign44420_e57409_d_n4;
        locals.var_sp_s_c_dn6 = assign44420_e57409_d_n6;
        locals.var_sp_s_c_dn7 = assign44420_e57409_d_n7;
        locals.var_sp_s_c_dn8 = assign44420_e57409_d_n8;
        locals.var_sp_s_c_dn9 = assign44420_e57409_d_n9;

        let (assign44430_e57423, assign44430_e57423_d_n4, assign44430_e57423_d_n6, assign44430_e57423_d_n7, assign44430_e57423_d_n8, assign44430_e57423_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44430_e57416: f64 = (locals.var_xn_d - locals.var_sp_s_eta);
        let assign44430_e57419: f64 = (locals.var_sp_s_a / locals.var_gf2);
        let assign44430_e57420: f64 = (assign44430_e57419).ln();
        let assign44430_e57421: f64 = (assign44430_e57416 + assign44430_e57420);
        (assign44430_e57421, ((locals.var_xn_d_dn4 - locals.var_sp_s_eta_dn4) + ((((locals.var_sp_s_a_dn4 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn4)) / (locals.var_gf2 * locals.var_gf2)) / assign44430_e57419)), ((locals.var_xn_d_dn6 - locals.var_sp_s_eta_dn6) + ((((locals.var_sp_s_a_dn6 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn6)) / (locals.var_gf2 * locals.var_gf2)) / assign44430_e57419)), ((locals.var_xn_d_dn7 - locals.var_sp_s_eta_dn7) + ((((locals.var_sp_s_a_dn7 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn7)) / (locals.var_gf2 * locals.var_gf2)) / assign44430_e57419)), ((locals.var_xn_d_dn8 - locals.var_sp_s_eta_dn8) + ((((locals.var_sp_s_a_dn8 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn8)) / (locals.var_gf2 * locals.var_gf2)) / assign44430_e57419)), ((locals.var_xn_d_dn9 - locals.var_sp_s_eta_dn9) + ((((locals.var_sp_s_a_dn9 * locals.var_gf2) - (locals.var_sp_s_a * locals.var_gf2_dn9)) / (locals.var_gf2 * locals.var_gf2)) / assign44430_e57419)),)
    } else {
        (locals.var_sp_s_tau, locals.var_sp_s_tau_dn4, locals.var_sp_s_tau_dn6, locals.var_sp_s_tau_dn7, locals.var_sp_s_tau_dn8, locals.var_sp_s_tau_dn9,)
    }
};
        locals.var_sp_s_tau = assign44430_e57423;
        locals.var_sp_s_tau_dn4 = assign44430_e57423_d_n4;
        locals.var_sp_s_tau_dn6 = assign44430_e57423_d_n6;
        locals.var_sp_s_tau_dn7 = assign44430_e57423_d_n7;
        locals.var_sp_s_tau_dn8 = assign44430_e57423_d_n8;
        locals.var_sp_s_tau_dn9 = assign44430_e57423_d_n9;

        let (assign44440_e57432, assign44440_e57432_d_n4, assign44440_e57432_d_n6, assign44440_e57432_d_n7, assign44440_e57432_d_n8, assign44440_e57432_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44440_e57430: f64 = (locals.var_sp_s_a + locals.var_sp_s_c);
        (assign44440_e57430, (locals.var_sp_s_a_dn4 + locals.var_sp_s_c_dn4), (locals.var_sp_s_a_dn6 + locals.var_sp_s_c_dn6), (locals.var_sp_s_a_dn7 + locals.var_sp_s_c_dn7), (locals.var_sp_s_a_dn8 + locals.var_sp_s_c_dn8), (locals.var_sp_s_a_dn9 + locals.var_sp_s_c_dn9),)
    } else {
        (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9,)
    }
};
        locals.var_nu = assign44440_e57432;
        locals.var_nu_dn4 = assign44440_e57432_d_n4;
        locals.var_nu_dn6 = assign44440_e57432_d_n6;
        locals.var_nu_dn7 = assign44440_e57432_d_n7;
        locals.var_nu_dn8 = assign44440_e57432_d_n8;
        locals.var_nu_dn9 = assign44440_e57432_d_n9;

        let (assign44450_e57453, assign44450_e57453_d_n4, assign44450_e57453_d_n6, assign44450_e57453_d_n7, assign44450_e57453_d_n8, assign44450_e57453_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44450_e57439: f64 = (locals.var_nu * locals.var_nu);
        let assign44450_e57444: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign44450_e57445: f64 = (0.5 * assign44450_e57444);
        let assign44450_e57448: f64 = (locals.var_sp_s_a * locals.var_sp_s_b);
        let assign44450_e57449: f64 = (assign44450_e57445 - assign44450_e57448);
        let assign44450_e57450: f64 = (locals.var_sp_s_tau * assign44450_e57449);
        let assign44450_e57451: f64 = (assign44450_e57439 + assign44450_e57450);
        (assign44450_e57451, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_sp_s_tau_dn4 * assign44450_e57449) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn4 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn4))) - ((locals.var_sp_s_a_dn4 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn4)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_sp_s_tau_dn6 * assign44450_e57449) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6))) - ((locals.var_sp_s_a_dn6 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_sp_s_tau_dn7 * assign44450_e57449) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7))) - ((locals.var_sp_s_a_dn7 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_sp_s_tau_dn8 * assign44450_e57449) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8))) - ((locals.var_sp_s_a_dn8 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn8)))))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_sp_s_tau_dn9 * assign44450_e57449) + (locals.var_sp_s_tau * ((0.5 * ((locals.var_sp_s_c_dn9 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn9))) - ((locals.var_sp_s_a_dn9 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn9)))))),)
    } else {
        (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9,)
    }
};
        locals.var_mutau = assign44450_e57453;
        locals.var_mutau_dn4 = assign44450_e57453_d_n4;
        locals.var_mutau_dn6 = assign44450_e57453_d_n6;
        locals.var_mutau_dn7 = assign44450_e57453_d_n7;
        locals.var_mutau_dn8 = assign44450_e57453_d_n8;
        locals.var_mutau_dn9 = assign44450_e57453_d_n9;

        let (assign44460_e57488, assign44460_e57488_d_n4, assign44460_e57488_d_n6, assign44460_e57488_d_n7, assign44460_e57488_d_n8, assign44460_e57488_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44460_e57461: f64 = (locals.var_sp_s_a * locals.var_nu);
        let assign44460_e57463: f64 = (assign44460_e57461 * locals.var_sp_s_tau);
        let assign44460_e57467: f64 = (locals.var_nu / locals.var_mutau);
        let assign44460_e57469: f64 = (assign44460_e57467 * locals.var_sp_s_tau);
        let assign44460_e57471: f64 = (assign44460_e57469 * locals.var_sp_s_tau);
        let assign44460_e57473: f64 = (assign44460_e57471 * locals.var_sp_s_c);
        let assign44460_e57476: f64 = (locals.var_sp_s_c * locals.var_sp_s_c);
        let assign44460_e57478: f64 = (assign44460_e57476 * 0.3333333333333333);
        let assign44460_e57481: f64 = (locals.var_sp_s_a * locals.var_sp_s_b);
        let assign44460_e57482: f64 = (assign44460_e57478 - assign44460_e57481);
        let assign44460_e57483: f64 = (assign44460_e57473 * assign44460_e57482);
        let assign44460_e57484: f64 = (locals.var_mutau + assign44460_e57483);
        let assign44460_e57485: f64 = (assign44460_e57463 / assign44460_e57484);
        let assign44460_e57486: f64 = (locals.var_sp_s_eta + assign44460_e57485);
        (assign44460_e57486, (locals.var_sp_s_eta_dn4 + (((((((locals.var_sp_s_a_dn4 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn4)) * locals.var_sp_s_tau) + (assign44460_e57461 * locals.var_sp_s_tau_dn4)) * assign44460_e57484) - (assign44460_e57463 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign44460_e57467 * locals.var_sp_s_tau_dn4)) * locals.var_sp_s_tau) + (assign44460_e57469 * locals.var_sp_s_tau_dn4)) * locals.var_sp_s_c) + (assign44460_e57471 * locals.var_sp_s_c_dn4)) * assign44460_e57482) + (assign44460_e57473 * ((((locals.var_sp_s_c_dn4 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn4)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn4 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn4)))))))) / (assign44460_e57484 * assign44460_e57484))), (locals.var_sp_s_eta_dn6 + (((((((locals.var_sp_s_a_dn6 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn6)) * locals.var_sp_s_tau) + (assign44460_e57461 * locals.var_sp_s_tau_dn6)) * assign44460_e57484) - (assign44460_e57463 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign44460_e57467 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_tau) + (assign44460_e57469 * locals.var_sp_s_tau_dn6)) * locals.var_sp_s_c) + (assign44460_e57471 * locals.var_sp_s_c_dn6)) * assign44460_e57482) + (assign44460_e57473 * ((((locals.var_sp_s_c_dn6 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn6)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn6 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn6)))))))) / (assign44460_e57484 * assign44460_e57484))), (locals.var_sp_s_eta_dn7 + (((((((locals.var_sp_s_a_dn7 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn7)) * locals.var_sp_s_tau) + (assign44460_e57461 * locals.var_sp_s_tau_dn7)) * assign44460_e57484) - (assign44460_e57463 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign44460_e57467 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_tau) + (assign44460_e57469 * locals.var_sp_s_tau_dn7)) * locals.var_sp_s_c) + (assign44460_e57471 * locals.var_sp_s_c_dn7)) * assign44460_e57482) + (assign44460_e57473 * ((((locals.var_sp_s_c_dn7 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn7)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn7 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn7)))))))) / (assign44460_e57484 * assign44460_e57484))), (locals.var_sp_s_eta_dn8 + (((((((locals.var_sp_s_a_dn8 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn8)) * locals.var_sp_s_tau) + (assign44460_e57461 * locals.var_sp_s_tau_dn8)) * assign44460_e57484) - (assign44460_e57463 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign44460_e57467 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_tau) + (assign44460_e57469 * locals.var_sp_s_tau_dn8)) * locals.var_sp_s_c) + (assign44460_e57471 * locals.var_sp_s_c_dn8)) * assign44460_e57482) + (assign44460_e57473 * ((((locals.var_sp_s_c_dn8 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn8)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn8 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn8)))))))) / (assign44460_e57484 * assign44460_e57484))), (locals.var_sp_s_eta_dn9 + (((((((locals.var_sp_s_a_dn9 * locals.var_nu) + (locals.var_sp_s_a * locals.var_nu_dn9)) * locals.var_sp_s_tau) + (assign44460_e57461 * locals.var_sp_s_tau_dn9)) * assign44460_e57484) - (assign44460_e57463 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_sp_s_tau) + (assign44460_e57467 * locals.var_sp_s_tau_dn9)) * locals.var_sp_s_tau) + (assign44460_e57469 * locals.var_sp_s_tau_dn9)) * locals.var_sp_s_c) + (assign44460_e57471 * locals.var_sp_s_c_dn9)) * assign44460_e57482) + (assign44460_e57473 * ((((locals.var_sp_s_c_dn9 * locals.var_sp_s_c) + (locals.var_sp_s_c * locals.var_sp_s_c_dn9)) * 0.3333333333333333) - ((locals.var_sp_s_a_dn9 * locals.var_sp_s_b) + (locals.var_sp_s_a * locals.var_sp_s_b_dn9)))))))) / (assign44460_e57484 * assign44460_e57484))),)
    } else {
        (locals.var_sp_s_x0, locals.var_sp_s_x0_dn4, locals.var_sp_s_x0_dn6, locals.var_sp_s_x0_dn7, locals.var_sp_s_x0_dn8, locals.var_sp_s_x0_dn9,)
    }
};
        locals.var_sp_s_x0 = assign44460_e57488;
        locals.var_sp_s_x0_dn4 = assign44460_e57488_d_n4;
        locals.var_sp_s_x0_dn6 = assign44460_e57488_d_n6;
        locals.var_sp_s_x0_dn7 = assign44460_e57488_d_n7;
        locals.var_sp_s_x0_dn8 = assign44460_e57488_d_n8;
        locals.var_sp_s_x0_dn9 = assign44460_e57488_d_n9;

        let assign44470_e57491: f64 = if locals.var_sp_s_x0 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1223 = assign44470_e57491;

        let (assign44480_e57501, assign44480_e57501_d_n4, assign44480_e57501_d_n6, assign44480_e57501_d_n7, assign44480_e57501_d_n8, assign44480_e57501_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 != 0.0)) {
        let assign44480_e57499: f64 = (locals.var_sp_s_x0).exp();
        (assign44480_e57499, (assign44480_e57499 * locals.var_sp_s_x0_dn4), (assign44480_e57499 * locals.var_sp_s_x0_dn6), (assign44480_e57499 * locals.var_sp_s_x0_dn7), (assign44480_e57499 * locals.var_sp_s_x0_dn8), (assign44480_e57499 * locals.var_sp_s_x0_dn9),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9,)
    }
};
        locals.var_sp_s_delta0 = assign44480_e57501;
        locals.var_sp_s_delta0_dn4 = assign44480_e57501_d_n4;
        locals.var_sp_s_delta0_dn6 = assign44480_e57501_d_n6;
        locals.var_sp_s_delta0_dn7 = assign44480_e57501_d_n7;
        locals.var_sp_s_delta0_dn8 = assign44480_e57501_d_n8;
        locals.var_sp_s_delta0_dn9 = assign44480_e57501_d_n9;

        let (assign44490_e57512, assign44490_e57512_d_n4, assign44490_e57512_d_n6, assign44490_e57512_d_n7, assign44490_e57512_d_n8, assign44490_e57512_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 != 0.0)) {
        let assign44490_e57510: f64 = (1.0 / locals.var_sp_s_delta0);
        (assign44490_e57510, (-(locals.var_sp_s_delta0_dn4 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn6 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn7 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn8 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))), (-(locals.var_sp_s_delta0_dn9 / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0))),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn4, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, locals.var_sp_s_delta1_dn9,)
    }
};
        locals.var_sp_s_delta1 = assign44490_e57512;
        locals.var_sp_s_delta1_dn4 = assign44490_e57512_d_n4;
        locals.var_sp_s_delta1_dn6 = assign44490_e57512_d_n6;
        locals.var_sp_s_delta1_dn7 = assign44490_e57512_d_n7;
        locals.var_sp_s_delta1_dn8 = assign44490_e57512_d_n8;
        locals.var_sp_s_delta1_dn9 = assign44490_e57512_d_n9;

        let (assign44500_e57523, assign44500_e57523_d_n4, assign44500_e57523_d_n6, assign44500_e57523_d_n7, assign44500_e57523_d_n8, assign44500_e57523_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 != 0.0)) {
        let assign44500_e57521: f64 = (locals.var_delta_nd * locals.var_sp_s_delta0);
        (assign44500_e57521, ((locals.var_delta_nd_dn4 * locals.var_sp_s_delta0) + (locals.var_delta_nd * locals.var_sp_s_delta0_dn4)), ((locals.var_delta_nd_dn6 * locals.var_sp_s_delta0) + (locals.var_delta_nd * locals.var_sp_s_delta0_dn6)), ((locals.var_delta_nd_dn7 * locals.var_sp_s_delta0) + (locals.var_delta_nd * locals.var_sp_s_delta0_dn7)), ((locals.var_delta_nd_dn8 * locals.var_sp_s_delta0) + (locals.var_delta_nd * locals.var_sp_s_delta0_dn8)), ((locals.var_delta_nd_dn9 * locals.var_sp_s_delta0) + (locals.var_delta_nd * locals.var_sp_s_delta0_dn9)),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9,)
    }
};
        locals.var_sp_s_delta0 = assign44500_e57523;
        locals.var_sp_s_delta0_dn4 = assign44500_e57523_d_n4;
        locals.var_sp_s_delta0_dn6 = assign44500_e57523_d_n6;
        locals.var_sp_s_delta0_dn7 = assign44500_e57523_d_n7;
        locals.var_sp_s_delta0_dn8 = assign44500_e57523_d_n8;
        locals.var_sp_s_delta0_dn9 = assign44500_e57523_d_n9;

        let assign44510_e57527: f64 = (locals.var_xn_d - 230.25850929940458);
        let assign44510_e57528: f64 = if locals.var_sp_s_x0 > assign44510_e57527 { 1.0 } else { 0.0 };
        locals.var_guard1224 = assign44510_e57528;

        let (assign44520_e57543, assign44520_e57543_d_n4, assign44520_e57543_d_n6, assign44520_e57543_d_n7, assign44520_e57543_d_n8, assign44520_e57543_d_n9,) = {
    if ((((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 == 0.0)) && (locals.var_guard1224 != 0.0)) {
        let assign44520_e57540: f64 = (locals.var_sp_s_x0 - locals.var_xn_d);
        let assign44520_e57541: f64 = (assign44520_e57540).exp();
        (assign44520_e57541, (assign44520_e57541 * (locals.var_sp_s_x0_dn4 - locals.var_xn_d_dn4)), (assign44520_e57541 * (locals.var_sp_s_x0_dn6 - locals.var_xn_d_dn6)), (assign44520_e57541 * (locals.var_sp_s_x0_dn7 - locals.var_xn_d_dn7)), (assign44520_e57541 * (locals.var_sp_s_x0_dn8 - locals.var_xn_d_dn8)), (assign44520_e57541 * (locals.var_sp_s_x0_dn9 - locals.var_xn_d_dn9)),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9,)
    }
};
        locals.var_sp_s_delta0 = assign44520_e57543;
        locals.var_sp_s_delta0_dn4 = assign44520_e57543_d_n4;
        locals.var_sp_s_delta0_dn6 = assign44520_e57543_d_n6;
        locals.var_sp_s_delta0_dn7 = assign44520_e57543_d_n7;
        locals.var_sp_s_delta0_dn8 = assign44520_e57543_d_n8;
        locals.var_sp_s_delta0_dn9 = assign44520_e57543_d_n9;

        let (assign44530_e57557, assign44530_e57557_d_n4, assign44530_e57557_d_n6, assign44530_e57557_d_n7, assign44530_e57557_d_n8, assign44530_e57557_d_n9,) = {
    if ((((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 == 0.0)) && (locals.var_guard1224 != 0.0)) {
        let assign44530_e57555: f64 = (locals.var_delta_nd / locals.var_sp_s_delta0);
        (assign44530_e57555, (((locals.var_delta_nd_dn4 * locals.var_sp_s_delta0) - (locals.var_delta_nd * locals.var_sp_s_delta0_dn4)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_nd_dn6 * locals.var_sp_s_delta0) - (locals.var_delta_nd * locals.var_sp_s_delta0_dn6)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_nd_dn7 * locals.var_sp_s_delta0) - (locals.var_delta_nd * locals.var_sp_s_delta0_dn7)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_nd_dn8 * locals.var_sp_s_delta0) - (locals.var_delta_nd * locals.var_sp_s_delta0_dn8)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)), (((locals.var_delta_nd_dn9 * locals.var_sp_s_delta0) - (locals.var_delta_nd * locals.var_sp_s_delta0_dn9)) / (locals.var_sp_s_delta0 * locals.var_sp_s_delta0)),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn4, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, locals.var_sp_s_delta1_dn9,)
    }
};
        locals.var_sp_s_delta1 = assign44530_e57557;
        locals.var_sp_s_delta1_dn4 = assign44530_e57557_d_n4;
        locals.var_sp_s_delta1_dn6 = assign44530_e57557_d_n6;
        locals.var_sp_s_delta1_dn7 = assign44530_e57557_d_n7;
        locals.var_sp_s_delta1_dn8 = assign44530_e57557_d_n8;
        locals.var_sp_s_delta1_dn9 = assign44530_e57557_d_n9;

        let (assign44540_e57598, assign44540_e57598_d_n4, assign44540_e57598_d_n6, assign44540_e57598_d_n7, assign44540_e57598_d_n8, assign44540_e57598_d_n9,) = {
    if ((((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 == 0.0)) && (locals.var_guard1224 == 0.0)) {
        let assign44540_e57572: f64 = (locals.var_xn_d - locals.var_sp_s_x0);
        let assign44540_e57574: f64 = (assign44540_e57572 - 230.25850929940458);
        let assign44540_e57579: f64 = (locals.var_xn_d - locals.var_sp_s_x0);
        let assign44540_e57581: f64 = (assign44540_e57579 - 230.25850929940458);
        let assign44540_e57585: f64 = (locals.var_xn_d - locals.var_sp_s_x0);
        let assign44540_e57587: f64 = (assign44540_e57585 - 230.25850929940458);
        let assign44540_e57589: f64 = (assign44540_e57587 * 0.3333333333333333);
        let assign44540_e57590: f64 = (1.0 + assign44540_e57589);
        let assign44540_e57591: f64 = (assign44540_e57581 * assign44540_e57590);
        let assign44540_e57592: f64 = (0.5 * assign44540_e57591);
        let assign44540_e57593: f64 = (1.0 + assign44540_e57592);
        let assign44540_e57594: f64 = (assign44540_e57574 * assign44540_e57593);
        let assign44540_e57595: f64 = (1.0 + assign44540_e57594);
        let assign44540_e57596: f64 = (1e-100 / assign44540_e57595);
        (assign44540_e57596, (-((1e-100 * (((locals.var_xn_d_dn4 - locals.var_sp_s_x0_dn4) * assign44540_e57593) + (assign44540_e57574 * (0.5 * (((locals.var_xn_d_dn4 - locals.var_sp_s_x0_dn4) * assign44540_e57590) + (assign44540_e57581 * ((locals.var_xn_d_dn4 - locals.var_sp_s_x0_dn4) * 0.3333333333333333))))))) / (assign44540_e57595 * assign44540_e57595))), (-((1e-100 * (((locals.var_xn_d_dn6 - locals.var_sp_s_x0_dn6) * assign44540_e57593) + (assign44540_e57574 * (0.5 * (((locals.var_xn_d_dn6 - locals.var_sp_s_x0_dn6) * assign44540_e57590) + (assign44540_e57581 * ((locals.var_xn_d_dn6 - locals.var_sp_s_x0_dn6) * 0.3333333333333333))))))) / (assign44540_e57595 * assign44540_e57595))), (-((1e-100 * (((locals.var_xn_d_dn7 - locals.var_sp_s_x0_dn7) * assign44540_e57593) + (assign44540_e57574 * (0.5 * (((locals.var_xn_d_dn7 - locals.var_sp_s_x0_dn7) * assign44540_e57590) + (assign44540_e57581 * ((locals.var_xn_d_dn7 - locals.var_sp_s_x0_dn7) * 0.3333333333333333))))))) / (assign44540_e57595 * assign44540_e57595))), (-((1e-100 * (((locals.var_xn_d_dn8 - locals.var_sp_s_x0_dn8) * assign44540_e57593) + (assign44540_e57574 * (0.5 * (((locals.var_xn_d_dn8 - locals.var_sp_s_x0_dn8) * assign44540_e57590) + (assign44540_e57581 * ((locals.var_xn_d_dn8 - locals.var_sp_s_x0_dn8) * 0.3333333333333333))))))) / (assign44540_e57595 * assign44540_e57595))), (-((1e-100 * (((locals.var_xn_d_dn9 - locals.var_sp_s_x0_dn9) * assign44540_e57593) + (assign44540_e57574 * (0.5 * (((locals.var_xn_d_dn9 - locals.var_sp_s_x0_dn9) * assign44540_e57590) + (assign44540_e57581 * ((locals.var_xn_d_dn9 - locals.var_sp_s_x0_dn9) * 0.3333333333333333))))))) / (assign44540_e57595 * assign44540_e57595))),)
    } else {
        (locals.var_sp_s_delta0, locals.var_sp_s_delta0_dn4, locals.var_sp_s_delta0_dn6, locals.var_sp_s_delta0_dn7, locals.var_sp_s_delta0_dn8, locals.var_sp_s_delta0_dn9,)
    }
};
        locals.var_sp_s_delta0 = assign44540_e57598;
        locals.var_sp_s_delta0_dn4 = assign44540_e57598_d_n4;
        locals.var_sp_s_delta0_dn6 = assign44540_e57598_d_n6;
        locals.var_sp_s_delta0_dn7 = assign44540_e57598_d_n7;
        locals.var_sp_s_delta0_dn8 = assign44540_e57598_d_n8;
        locals.var_sp_s_delta0_dn9 = assign44540_e57598_d_n9;

        let (assign44550_e57633, assign44550_e57633_d_n4, assign44550_e57633_d_n6, assign44550_e57633_d_n7, assign44550_e57633_d_n8, assign44550_e57633_d_n9,) = {
    if ((((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) && (locals.var_guard1223 == 0.0)) && (locals.var_guard1224 == 0.0)) {
        let assign44550_e57613: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
        let assign44550_e57618: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
        let assign44550_e57622: f64 = (locals.var_sp_s_x0 - 230.25850929940458);
        let assign44550_e57624: f64 = (assign44550_e57622 * 0.3333333333333333);
        let assign44550_e57625: f64 = (1.0 + assign44550_e57624);
        let assign44550_e57626: f64 = (assign44550_e57618 * assign44550_e57625);
        let assign44550_e57627: f64 = (0.5 * assign44550_e57626);
        let assign44550_e57628: f64 = (1.0 + assign44550_e57627);
        let assign44550_e57629: f64 = (assign44550_e57613 * assign44550_e57628);
        let assign44550_e57630: f64 = (1.0 + assign44550_e57629);
        let assign44550_e57631: f64 = (1e-100 / assign44550_e57630);
        (assign44550_e57631, (-((1e-100 * ((locals.var_sp_s_x0_dn4 * assign44550_e57628) + (assign44550_e57613 * (0.5 * ((locals.var_sp_s_x0_dn4 * assign44550_e57625) + (assign44550_e57618 * (locals.var_sp_s_x0_dn4 * 0.3333333333333333))))))) / (assign44550_e57630 * assign44550_e57630))), (-((1e-100 * ((locals.var_sp_s_x0_dn6 * assign44550_e57628) + (assign44550_e57613 * (0.5 * ((locals.var_sp_s_x0_dn6 * assign44550_e57625) + (assign44550_e57618 * (locals.var_sp_s_x0_dn6 * 0.3333333333333333))))))) / (assign44550_e57630 * assign44550_e57630))), (-((1e-100 * ((locals.var_sp_s_x0_dn7 * assign44550_e57628) + (assign44550_e57613 * (0.5 * ((locals.var_sp_s_x0_dn7 * assign44550_e57625) + (assign44550_e57618 * (locals.var_sp_s_x0_dn7 * 0.3333333333333333))))))) / (assign44550_e57630 * assign44550_e57630))), (-((1e-100 * ((locals.var_sp_s_x0_dn8 * assign44550_e57628) + (assign44550_e57613 * (0.5 * ((locals.var_sp_s_x0_dn8 * assign44550_e57625) + (assign44550_e57618 * (locals.var_sp_s_x0_dn8 * 0.3333333333333333))))))) / (assign44550_e57630 * assign44550_e57630))), (-((1e-100 * ((locals.var_sp_s_x0_dn9 * assign44550_e57628) + (assign44550_e57613 * (0.5 * ((locals.var_sp_s_x0_dn9 * assign44550_e57625) + (assign44550_e57618 * (locals.var_sp_s_x0_dn9 * 0.3333333333333333))))))) / (assign44550_e57630 * assign44550_e57630))),)
    } else {
        (locals.var_sp_s_delta1, locals.var_sp_s_delta1_dn4, locals.var_sp_s_delta1_dn6, locals.var_sp_s_delta1_dn7, locals.var_sp_s_delta1_dn8, locals.var_sp_s_delta1_dn9,)
    }
};
        locals.var_sp_s_delta1 = assign44550_e57633;
        locals.var_sp_s_delta1_dn4 = assign44550_e57633_d_n4;
        locals.var_sp_s_delta1_dn6 = assign44550_e57633_d_n6;
        locals.var_sp_s_delta1_dn7 = assign44550_e57633_d_n7;
        locals.var_sp_s_delta1_dn8 = assign44550_e57633_d_n8;
        locals.var_sp_s_delta1_dn9 = assign44550_e57633_d_n9;

        let (assign44560_e57646, assign44560_e57646_d_n4, assign44560_e57646_d_n6, assign44560_e57646_d_n7, assign44560_e57646_d_n8, assign44560_e57646_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44560_e57642: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_x0);
        let assign44560_e57643: f64 = (2.0 + assign44560_e57642);
        let assign44560_e57644: f64 = (1.0 / assign44560_e57643);
        (assign44560_e57644, (-(((locals.var_sp_s_x0_dn4 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn4)) / (assign44560_e57643 * assign44560_e57643))), (-(((locals.var_sp_s_x0_dn6 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn6)) / (assign44560_e57643 * assign44560_e57643))), (-(((locals.var_sp_s_x0_dn7 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn7)) / (assign44560_e57643 * assign44560_e57643))), (-(((locals.var_sp_s_x0_dn8 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn8)) / (assign44560_e57643 * assign44560_e57643))), (-(((locals.var_sp_s_x0_dn9 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn9)) / (assign44560_e57643 * assign44560_e57643))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign44560_e57646;
        locals.var_sp_s_temp_dn4 = assign44560_e57646_d_n4;
        locals.var_sp_s_temp_dn6 = assign44560_e57646_d_n6;
        locals.var_sp_s_temp_dn7 = assign44560_e57646_d_n7;
        locals.var_sp_s_temp_dn8 = assign44560_e57646_d_n8;
        locals.var_sp_s_temp_dn9 = assign44560_e57646_d_n9;

        let (assign44570_e57657, assign44570_e57657_d_n4, assign44570_e57657_d_n6, assign44570_e57657_d_n7, assign44570_e57657_d_n8, assign44570_e57657_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44570_e57653: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_x0);
        let assign44570_e57655: f64 = (assign44570_e57653 * locals.var_sp_s_temp);
        (assign44570_e57655, ((((locals.var_sp_s_x0_dn4 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn4)) * locals.var_sp_s_temp) + (assign44570_e57653 * locals.var_sp_s_temp_dn4)), ((((locals.var_sp_s_x0_dn6 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn6)) * locals.var_sp_s_temp) + (assign44570_e57653 * locals.var_sp_s_temp_dn6)), ((((locals.var_sp_s_x0_dn7 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn7)) * locals.var_sp_s_temp) + (assign44570_e57653 * locals.var_sp_s_temp_dn7)), ((((locals.var_sp_s_x0_dn8 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn8)) * locals.var_sp_s_temp) + (assign44570_e57653 * locals.var_sp_s_temp_dn8)), ((((locals.var_sp_s_x0_dn9 * locals.var_sp_s_x0) + (locals.var_sp_s_x0 * locals.var_sp_s_x0_dn9)) * locals.var_sp_s_temp) + (assign44570_e57653 * locals.var_sp_s_temp_dn9)),)
    } else {
        (locals.var_sp_s_xi0, locals.var_sp_s_xi0_dn4, locals.var_sp_s_xi0_dn6, locals.var_sp_s_xi0_dn7, locals.var_sp_s_xi0_dn8, locals.var_sp_s_xi0_dn9,)
    }
};
        locals.var_sp_s_xi0 = assign44570_e57657;
        locals.var_sp_s_xi0_dn4 = assign44570_e57657_d_n4;
        locals.var_sp_s_xi0_dn6 = assign44570_e57657_d_n6;
        locals.var_sp_s_xi0_dn7 = assign44570_e57657_d_n7;
        locals.var_sp_s_xi0_dn8 = assign44570_e57657_d_n8;
        locals.var_sp_s_xi0_dn9 = assign44570_e57657_d_n9;

        let (assign44580_e57670, assign44580_e57670_d_n4, assign44580_e57670_d_n6, assign44580_e57670_d_n7, assign44580_e57670_d_n8, assign44580_e57670_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44580_e57665: f64 = (locals.var_sp_s_x0 * locals.var_sp_s_temp);
        let assign44580_e57667: f64 = (assign44580_e57665 * locals.var_sp_s_temp);
        let assign44580_e57668: f64 = (4.0 * assign44580_e57667);
        (assign44580_e57668, (4.0 * ((((locals.var_sp_s_x0_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn4)) * locals.var_sp_s_temp) + (assign44580_e57665 * locals.var_sp_s_temp_dn4))), (4.0 * ((((locals.var_sp_s_x0_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign44580_e57665 * locals.var_sp_s_temp_dn6))), (4.0 * ((((locals.var_sp_s_x0_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign44580_e57665 * locals.var_sp_s_temp_dn7))), (4.0 * ((((locals.var_sp_s_x0_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign44580_e57665 * locals.var_sp_s_temp_dn8))), (4.0 * ((((locals.var_sp_s_x0_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_x0 * locals.var_sp_s_temp_dn9)) * locals.var_sp_s_temp) + (assign44580_e57665 * locals.var_sp_s_temp_dn9))),)
    } else {
        (locals.var_sp_s_xi1, locals.var_sp_s_xi1_dn4, locals.var_sp_s_xi1_dn6, locals.var_sp_s_xi1_dn7, locals.var_sp_s_xi1_dn8, locals.var_sp_s_xi1_dn9,)
    }
};
        locals.var_sp_s_xi1 = assign44580_e57670;
        locals.var_sp_s_xi1_dn4 = assign44580_e57670_d_n4;
        locals.var_sp_s_xi1_dn6 = assign44580_e57670_d_n6;
        locals.var_sp_s_xi1_dn7 = assign44580_e57670_d_n7;
        locals.var_sp_s_xi1_dn8 = assign44580_e57670_d_n8;
        locals.var_sp_s_xi1_dn9 = assign44580_e57670_d_n9;

        let (assign44590_e57687, assign44590_e57687_d_n4, assign44590_e57687_d_n6, assign44590_e57687_d_n7, assign44590_e57687_d_n8, assign44590_e57687_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44590_e57677: f64 = (8.0 * locals.var_sp_s_temp);
        let assign44590_e57680: f64 = (12.0 * locals.var_sp_s_xi0);
        let assign44590_e57681: f64 = (assign44590_e57677 - assign44590_e57680);
        let assign44590_e57683: f64 = (assign44590_e57681 * locals.var_sp_s_temp);
        let assign44590_e57685: f64 = (assign44590_e57683 * locals.var_sp_s_temp);
        (assign44590_e57685, ((((((8.0 * locals.var_sp_s_temp_dn4) - (12.0 * locals.var_sp_s_xi0_dn4)) * locals.var_sp_s_temp) + (assign44590_e57681 * locals.var_sp_s_temp_dn4)) * locals.var_sp_s_temp) + (assign44590_e57683 * locals.var_sp_s_temp_dn4)), ((((((8.0 * locals.var_sp_s_temp_dn6) - (12.0 * locals.var_sp_s_xi0_dn6)) * locals.var_sp_s_temp) + (assign44590_e57681 * locals.var_sp_s_temp_dn6)) * locals.var_sp_s_temp) + (assign44590_e57683 * locals.var_sp_s_temp_dn6)), ((((((8.0 * locals.var_sp_s_temp_dn7) - (12.0 * locals.var_sp_s_xi0_dn7)) * locals.var_sp_s_temp) + (assign44590_e57681 * locals.var_sp_s_temp_dn7)) * locals.var_sp_s_temp) + (assign44590_e57683 * locals.var_sp_s_temp_dn7)), ((((((8.0 * locals.var_sp_s_temp_dn8) - (12.0 * locals.var_sp_s_xi0_dn8)) * locals.var_sp_s_temp) + (assign44590_e57681 * locals.var_sp_s_temp_dn8)) * locals.var_sp_s_temp) + (assign44590_e57683 * locals.var_sp_s_temp_dn8)), ((((((8.0 * locals.var_sp_s_temp_dn9) - (12.0 * locals.var_sp_s_xi0_dn9)) * locals.var_sp_s_temp) + (assign44590_e57681 * locals.var_sp_s_temp_dn9)) * locals.var_sp_s_temp) + (assign44590_e57683 * locals.var_sp_s_temp_dn9)),)
    } else {
        (locals.var_sp_s_xi2, locals.var_sp_s_xi2_dn4, locals.var_sp_s_xi2_dn6, locals.var_sp_s_xi2_dn7, locals.var_sp_s_xi2_dn8, locals.var_sp_s_xi2_dn9,)
    }
};
        locals.var_sp_s_xi2 = assign44590_e57687;
        locals.var_sp_s_xi2_dn4 = assign44590_e57687_d_n4;
        locals.var_sp_s_xi2_dn6 = assign44590_e57687_d_n6;
        locals.var_sp_s_xi2_dn7 = assign44590_e57687_d_n7;
        locals.var_sp_s_xi2_dn8 = assign44590_e57687_d_n8;
        locals.var_sp_s_xi2_dn9 = assign44590_e57687_d_n9;

        let (assign44600_e57696, assign44600_e57696_d_n4, assign44600_e57696_d_n6, assign44600_e57696_d_n7, assign44600_e57696_d_n8, assign44600_e57696_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44600_e57694: f64 = (locals.var_xg - locals.var_sp_s_x0);
        (assign44600_e57694, (locals.var_xg_dn4 - locals.var_sp_s_x0_dn4), (locals.var_xg_dn6 - locals.var_sp_s_x0_dn6), (locals.var_xg_dn7 - locals.var_sp_s_x0_dn7), (locals.var_xg_dn8 - locals.var_sp_s_x0_dn8), (locals.var_xg_dn9 - locals.var_sp_s_x0_dn9),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign44600_e57696;
        locals.var_sp_s_temp_dn4 = assign44600_e57696_d_n4;
        locals.var_sp_s_temp_dn6 = assign44600_e57696_d_n6;
        locals.var_sp_s_temp_dn7 = assign44600_e57696_d_n7;
        locals.var_sp_s_temp_dn8 = assign44600_e57696_d_n8;
        locals.var_sp_s_temp_dn9 = assign44600_e57696_d_n9;

        let (assign44610_e57719, assign44610_e57719_d_n4, assign44610_e57719_d_n6, assign44610_e57719_d_n7, assign44610_e57719_d_n8, assign44610_e57719_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44610_e57703: f64 = (2.0 * locals.var_sp_s_temp);
        let assign44610_e57707: f64 = (1.0 - locals.var_sp_s_delta1);
        let assign44610_e57709: f64 = (assign44610_e57707 + locals.var_sp_s_delta0);
        let assign44610_e57713: f64 = (1.0 + locals.var_sp_s_xi1);
        let assign44610_e57714: f64 = (locals.var_delta_nd * assign44610_e57713);
        let assign44610_e57715: f64 = (assign44610_e57709 - assign44610_e57714);
        let assign44610_e57716: f64 = (locals.var_gf2 * assign44610_e57715);
        let assign44610_e57717: f64 = (assign44610_e57703 + assign44610_e57716);
        (assign44610_e57717, ((2.0 * locals.var_sp_s_temp_dn4) + ((locals.var_gf2_dn4 * assign44610_e57715) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn4) + locals.var_sp_s_delta0_dn4) - ((locals.var_delta_nd_dn4 * assign44610_e57713) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn4)))))), ((2.0 * locals.var_sp_s_temp_dn6) + ((locals.var_gf2_dn6 * assign44610_e57715) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn6) + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_nd_dn6 * assign44610_e57713) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn6)))))), ((2.0 * locals.var_sp_s_temp_dn7) + ((locals.var_gf2_dn7 * assign44610_e57715) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn7) + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_nd_dn7 * assign44610_e57713) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn7)))))), ((2.0 * locals.var_sp_s_temp_dn8) + ((locals.var_gf2_dn8 * assign44610_e57715) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn8) + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_nd_dn8 * assign44610_e57713) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn8)))))), ((2.0 * locals.var_sp_s_temp_dn9) + ((locals.var_gf2_dn9 * assign44610_e57715) + (locals.var_gf2 * (((-locals.var_sp_s_delta1_dn9) + locals.var_sp_s_delta0_dn9) - ((locals.var_delta_nd_dn9 * assign44610_e57713) + (locals.var_delta_nd * locals.var_sp_s_xi1_dn9)))))),)
    } else {
        (locals.var_sp_s_pc, locals.var_sp_s_pc_dn4, locals.var_sp_s_pc_dn6, locals.var_sp_s_pc_dn7, locals.var_sp_s_pc_dn8, locals.var_sp_s_pc_dn9,)
    }
};
        locals.var_sp_s_pc = assign44610_e57719;
        locals.var_sp_s_pc_dn4 = assign44610_e57719_d_n4;
        locals.var_sp_s_pc_dn6 = assign44610_e57719_d_n6;
        locals.var_sp_s_pc_dn7 = assign44610_e57719_d_n7;
        locals.var_sp_s_pc_dn8 = assign44610_e57719_d_n8;
        locals.var_sp_s_pc_dn9 = assign44610_e57719_d_n9;

        let (assign44620_e57746, assign44620_e57746_d_n4, assign44620_e57746_d_n6, assign44620_e57746_d_n7, assign44620_e57746_d_n8, assign44620_e57746_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44620_e57726: f64 = (locals.var_sp_s_temp * locals.var_sp_s_temp);
        let assign44620_e57730: f64 = (locals.var_sp_s_delta1 + locals.var_sp_s_x0);
        let assign44620_e57732: f64 = (assign44620_e57730 - 1.0);
        let assign44620_e57734: f64 = (assign44620_e57732 + locals.var_sp_s_delta0);
        let assign44620_e57738: f64 = (locals.var_sp_s_x0 + 1.0);
        let assign44620_e57740: f64 = (assign44620_e57738 + locals.var_sp_s_xi0);
        let assign44620_e57741: f64 = (locals.var_delta_nd * assign44620_e57740);
        let assign44620_e57742: f64 = (assign44620_e57734 - assign44620_e57741);
        let assign44620_e57743: f64 = (locals.var_gf2 * assign44620_e57742);
        let assign44620_e57744: f64 = (assign44620_e57726 - assign44620_e57743);
        (assign44620_e57744, (((locals.var_sp_s_temp_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn4)) - ((locals.var_gf2_dn4 * assign44620_e57742) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn4 + locals.var_sp_s_x0_dn4) + locals.var_sp_s_delta0_dn4) - ((locals.var_delta_nd_dn4 * assign44620_e57740) + (locals.var_delta_nd * (locals.var_sp_s_x0_dn4 + locals.var_sp_s_xi0_dn4))))))), (((locals.var_sp_s_temp_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn6)) - ((locals.var_gf2_dn6 * assign44620_e57742) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn6 + locals.var_sp_s_x0_dn6) + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_nd_dn6 * assign44620_e57740) + (locals.var_delta_nd * (locals.var_sp_s_x0_dn6 + locals.var_sp_s_xi0_dn6))))))), (((locals.var_sp_s_temp_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn7)) - ((locals.var_gf2_dn7 * assign44620_e57742) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn7 + locals.var_sp_s_x0_dn7) + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_nd_dn7 * assign44620_e57740) + (locals.var_delta_nd * (locals.var_sp_s_x0_dn7 + locals.var_sp_s_xi0_dn7))))))), (((locals.var_sp_s_temp_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn8)) - ((locals.var_gf2_dn8 * assign44620_e57742) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn8 + locals.var_sp_s_x0_dn8) + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_nd_dn8 * assign44620_e57740) + (locals.var_delta_nd * (locals.var_sp_s_x0_dn8 + locals.var_sp_s_xi0_dn8))))))), (((locals.var_sp_s_temp_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_temp * locals.var_sp_s_temp_dn9)) - ((locals.var_gf2_dn9 * assign44620_e57742) + (locals.var_gf2 * (((locals.var_sp_s_delta1_dn9 + locals.var_sp_s_x0_dn9) + locals.var_sp_s_delta0_dn9) - ((locals.var_delta_nd_dn9 * assign44620_e57740) + (locals.var_delta_nd * (locals.var_sp_s_x0_dn9 + locals.var_sp_s_xi0_dn9))))))),)
    } else {
        (locals.var_sp_s_qc, locals.var_sp_s_qc_dn4, locals.var_sp_s_qc_dn6, locals.var_sp_s_qc_dn7, locals.var_sp_s_qc_dn8, locals.var_sp_s_qc_dn9,)
    }
};
        locals.var_sp_s_qc = assign44620_e57746;
        locals.var_sp_s_qc_dn4 = assign44620_e57746_d_n4;
        locals.var_sp_s_qc_dn6 = assign44620_e57746_d_n6;
        locals.var_sp_s_qc_dn7 = assign44620_e57746_d_n7;
        locals.var_sp_s_qc_dn8 = assign44620_e57746_d_n8;
        locals.var_sp_s_qc_dn9 = assign44620_e57746_d_n9;

        let (assign44630_e57763, assign44630_e57763_d_n4, assign44630_e57763_d_n6, assign44630_e57763_d_n7, assign44630_e57763_d_n8, assign44630_e57763_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44630_e57755: f64 = (locals.var_sp_s_delta1 + locals.var_sp_s_delta0);
        let assign44630_e57758: f64 = (locals.var_delta_nd * locals.var_sp_s_xi2);
        let assign44630_e57759: f64 = (assign44630_e57755 - assign44630_e57758);
        let assign44630_e57760: f64 = (locals.var_gf2 * assign44630_e57759);
        let assign44630_e57761: f64 = (2.0 - assign44630_e57760);
        (assign44630_e57761, (-((locals.var_gf2_dn4 * assign44630_e57759) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn4 + locals.var_sp_s_delta0_dn4) - ((locals.var_delta_nd_dn4 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn4)))))), (-((locals.var_gf2_dn6 * assign44630_e57759) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn6 + locals.var_sp_s_delta0_dn6) - ((locals.var_delta_nd_dn6 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn6)))))), (-((locals.var_gf2_dn7 * assign44630_e57759) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn7 + locals.var_sp_s_delta0_dn7) - ((locals.var_delta_nd_dn7 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn7)))))), (-((locals.var_gf2_dn8 * assign44630_e57759) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn8 + locals.var_sp_s_delta0_dn8) - ((locals.var_delta_nd_dn8 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn8)))))), (-((locals.var_gf2_dn9 * assign44630_e57759) + (locals.var_gf2 * ((locals.var_sp_s_delta1_dn9 + locals.var_sp_s_delta0_dn9) - ((locals.var_delta_nd_dn9 * locals.var_sp_s_xi2) + (locals.var_delta_nd * locals.var_sp_s_xi2_dn9)))))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign44630_e57763;
        locals.var_sp_s_temp_dn4 = assign44630_e57763_d_n4;
        locals.var_sp_s_temp_dn6 = assign44630_e57763_d_n6;
        locals.var_sp_s_temp_dn7 = assign44630_e57763_d_n7;
        locals.var_sp_s_temp_dn8 = assign44630_e57763_d_n8;
        locals.var_sp_s_temp_dn9 = assign44630_e57763_d_n9;

        let (assign44640_e57778, assign44640_e57778_d_n4, assign44640_e57778_d_n6, assign44640_e57778_d_n7, assign44640_e57778_d_n8, assign44640_e57778_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44640_e57770: f64 = (locals.var_sp_s_pc * locals.var_sp_s_pc);
        let assign44640_e57774: f64 = (locals.var_sp_s_qc * locals.var_sp_s_temp);
        let assign44640_e57775: f64 = (2.0 * assign44640_e57774);
        let assign44640_e57776: f64 = (assign44640_e57770 - assign44640_e57775);
        (assign44640_e57776, (((locals.var_sp_s_pc_dn4 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn4)) - (2.0 * ((locals.var_sp_s_qc_dn4 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn4)))), (((locals.var_sp_s_pc_dn6 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn6)) - (2.0 * ((locals.var_sp_s_qc_dn6 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn6)))), (((locals.var_sp_s_pc_dn7 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn7)) - (2.0 * ((locals.var_sp_s_qc_dn7 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn7)))), (((locals.var_sp_s_pc_dn8 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn8)) - (2.0 * ((locals.var_sp_s_qc_dn8 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn8)))), (((locals.var_sp_s_pc_dn9 * locals.var_sp_s_pc) + (locals.var_sp_s_pc * locals.var_sp_s_pc_dn9)) - (2.0 * ((locals.var_sp_s_qc_dn9 * locals.var_sp_s_temp) + (locals.var_sp_s_qc * locals.var_sp_s_temp_dn9)))),)
    } else {
        (locals.var_sp_s_temp, locals.var_sp_s_temp_dn4, locals.var_sp_s_temp_dn6, locals.var_sp_s_temp_dn7, locals.var_sp_s_temp_dn8, locals.var_sp_s_temp_dn9,)
    }
};
        locals.var_sp_s_temp = assign44640_e57778;
        locals.var_sp_s_temp_dn4 = assign44640_e57778_d_n4;
        locals.var_sp_s_temp_dn6 = assign44640_e57778_d_n6;
        locals.var_sp_s_temp_dn7 = assign44640_e57778_d_n7;
        locals.var_sp_s_temp_dn8 = assign44640_e57778_d_n8;
        locals.var_sp_s_temp_dn9 = assign44640_e57778_d_n9;

        let (assign44650_e57794, assign44650_e57794_d_n4, assign44650_e57794_d_n6, assign44650_e57794_d_n7, assign44650_e57794_d_n8, assign44650_e57794_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign44650_e57788: f64 = (locals.var_sp_s_temp).sqrt();
        let assign44650_e57789: f64 = (locals.var_sp_s_pc + assign44650_e57788);
        let assign44650_e57790: f64 = (locals.var_sp_s_qc / assign44650_e57789);
        let assign44650_e57791: f64 = (2.0 * assign44650_e57790);
        let assign44650_e57792: f64 = (locals.var_sp_s_x0 + assign44650_e57791);
        (assign44650_e57792, (locals.var_sp_s_x0_dn4 + (2.0 * (((locals.var_sp_s_qc_dn4 * assign44650_e57789) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn4 + (locals.var_sp_s_temp_dn4 / (2.0 * assign44650_e57788))))) / (assign44650_e57789 * assign44650_e57789)))), (locals.var_sp_s_x0_dn6 + (2.0 * (((locals.var_sp_s_qc_dn6 * assign44650_e57789) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn6 + (locals.var_sp_s_temp_dn6 / (2.0 * assign44650_e57788))))) / (assign44650_e57789 * assign44650_e57789)))), (locals.var_sp_s_x0_dn7 + (2.0 * (((locals.var_sp_s_qc_dn7 * assign44650_e57789) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn7 + (locals.var_sp_s_temp_dn7 / (2.0 * assign44650_e57788))))) / (assign44650_e57789 * assign44650_e57789)))), (locals.var_sp_s_x0_dn8 + (2.0 * (((locals.var_sp_s_qc_dn8 * assign44650_e57789) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn8 + (locals.var_sp_s_temp_dn8 / (2.0 * assign44650_e57788))))) / (assign44650_e57789 * assign44650_e57789)))), (locals.var_sp_s_x0_dn9 + (2.0 * (((locals.var_sp_s_qc_dn9 * assign44650_e57789) - (locals.var_sp_s_qc * (locals.var_sp_s_pc_dn9 + (locals.var_sp_s_temp_dn9 / (2.0 * assign44650_e57788))))) / (assign44650_e57789 * assign44650_e57789)))),)
    } else {
        (locals.var_x_d, locals.var_x_d_dn4, locals.var_x_d_dn6, locals.var_x_d_dn7, locals.var_x_d_dn8, locals.var_x_d_dn9,)
    }
};
        locals.var_x_d = assign44650_e57794;
        locals.var_x_d_dn4 = assign44650_e57794_d_n4;
        locals.var_x_d_dn6 = assign44650_e57794_d_n6;
        locals.var_x_d_dn7 = assign44650_e57794_d_n7;
        locals.var_x_d_dn8 = assign44650_e57794_d_n8;
        locals.var_x_d_dn9 = assign44650_e57794_d_n9;

        let (assign44660_e57800, assign44660_e57800_d_n4, assign44660_e57800_d_n6, assign44660_e57800_d_n7, assign44660_e57800_d_n8, assign44660_e57800_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44660_e57798: f64 = (locals.var_x_d - locals.var_x_s);
        (assign44660_e57798, (locals.var_x_d_dn4 - locals.var_x_s_dn4), (locals.var_x_d_dn6 - locals.var_x_s_dn6), (locals.var_x_d_dn7 - locals.var_x_s_dn7), (locals.var_x_d_dn8 - locals.var_x_s_dn8), (locals.var_x_d_dn9 - locals.var_x_s_dn9),)
    } else {
        (locals.var_x_ds, locals.var_x_ds_dn4, locals.var_x_ds_dn6, locals.var_x_ds_dn7, locals.var_x_ds_dn8, locals.var_x_ds_dn9,)
    }
};
        locals.var_x_ds = assign44660_e57800;
        locals.var_x_ds_dn4 = assign44660_e57800_d_n4;
        locals.var_x_ds_dn6 = assign44660_e57800_d_n6;
        locals.var_x_ds_dn7 = assign44660_e57800_d_n7;
        locals.var_x_ds_dn8 = assign44660_e57800_d_n8;
        locals.var_x_ds_dn9 = assign44660_e57800_d_n9;

        let assign44670_e57803: f64 = if locals.var_x_ds < 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard1225 = assign44670_e57803;

    }

    pub(super) fn stamp_transient_block_25(
        locals: &mut StampLocals,
    ) {
        let (assign44680_e57829, assign44680_e57829_d_n4, assign44680_e57829_d_n6, assign44680_e57829_d_n7, assign44680_e57829_d_n8, assign44680_e57829_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1225 != 0.0)) {
        let assign44680_e57810: f64 = (locals.var_xg - locals.var_x_s);
        let assign44680_e57811: f64 = (2.0 * assign44680_e57810);
        let assign44680_e57815: f64 = (1.0 - locals.var_es);
        let assign44680_e57818: f64 = (locals.var_delta_1s * locals.var_k_ds);
        let assign44680_e57819: f64 = (assign44680_e57815 + assign44680_e57818);
        let assign44680_e57823: f64 = (1.0 + locals.var_xi1s);
        let assign44680_e57824: f64 = (locals.var_delta_nd * assign44680_e57823);
        let assign44680_e57825: f64 = (assign44680_e57819 - assign44680_e57824);
        let assign44680_e57826: f64 = (locals.var_gf2 * assign44680_e57825);
        let assign44680_e57827: f64 = (assign44680_e57811 + assign44680_e57826);
        (assign44680_e57827, ((2.0 * (locals.var_xg_dn4 - locals.var_x_s_dn4)) + ((locals.var_gf2_dn4 * assign44680_e57825) + (locals.var_gf2 * (((-locals.var_es_dn4) + ((locals.var_delta_1s_dn4 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn4))) - ((locals.var_delta_nd_dn4 * assign44680_e57823) + (locals.var_delta_nd * locals.var_xi1s_dn4)))))), ((2.0 * (locals.var_xg_dn6 - locals.var_x_s_dn6)) + ((locals.var_gf2_dn6 * assign44680_e57825) + (locals.var_gf2 * (((-locals.var_es_dn6) + ((locals.var_delta_1s_dn6 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn6))) - ((locals.var_delta_nd_dn6 * assign44680_e57823) + (locals.var_delta_nd * locals.var_xi1s_dn6)))))), ((2.0 * (locals.var_xg_dn7 - locals.var_x_s_dn7)) + ((locals.var_gf2_dn7 * assign44680_e57825) + (locals.var_gf2 * (((-locals.var_es_dn7) + ((locals.var_delta_1s_dn7 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn7))) - ((locals.var_delta_nd_dn7 * assign44680_e57823) + (locals.var_delta_nd * locals.var_xi1s_dn7)))))), ((2.0 * (locals.var_xg_dn8 - locals.var_x_s_dn8)) + ((locals.var_gf2_dn8 * assign44680_e57825) + (locals.var_gf2 * (((-locals.var_es_dn8) + ((locals.var_delta_1s_dn8 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn8))) - ((locals.var_delta_nd_dn8 * assign44680_e57823) + (locals.var_delta_nd * locals.var_xi1s_dn8)))))), ((2.0 * (locals.var_xg_dn9 - locals.var_x_s_dn9)) + ((locals.var_gf2_dn9 * assign44680_e57825) + (locals.var_gf2 * (((-locals.var_es_dn9) + ((locals.var_delta_1s_dn9 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn9))) - ((locals.var_delta_nd_dn9 * assign44680_e57823) + (locals.var_delta_nd * locals.var_xi1s_dn9)))))),)
    } else {
        (locals.var_pc, locals.var_pc_dn4, locals.var_pc_dn6, locals.var_pc_dn7, locals.var_pc_dn8, locals.var_pc_dn9,)
    }
};
        locals.var_pc = assign44680_e57829;
        locals.var_pc_dn4 = assign44680_e57829_d_n4;
        locals.var_pc_dn6 = assign44680_e57829_d_n6;
        locals.var_pc_dn7 = assign44680_e57829_d_n7;
        locals.var_pc_dn8 = assign44680_e57829_d_n8;
        locals.var_pc_dn9 = assign44680_e57829_d_n9;

        let (assign44690_e57841, assign44690_e57841_d_n4, assign44690_e57841_d_n6, assign44690_e57841_d_n7, assign44690_e57841_d_n8, assign44690_e57841_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1225 != 0.0)) {
        let assign44690_e57836: f64 = (1.0 - locals.var_k_ds);
        let assign44690_e57837: f64 = (locals.var_gf2 * assign44690_e57836);
        let assign44690_e57839: f64 = (assign44690_e57837 * locals.var_ds);
        (assign44690_e57839, ((((locals.var_gf2_dn4 * assign44690_e57836) + (locals.var_gf2 * (-locals.var_k_ds_dn4))) * locals.var_ds) + (assign44690_e57837 * locals.var_ds_dn4)), ((((locals.var_gf2_dn6 * assign44690_e57836) + (locals.var_gf2 * (-locals.var_k_ds_dn6))) * locals.var_ds) + (assign44690_e57837 * locals.var_ds_dn6)), ((((locals.var_gf2_dn7 * assign44690_e57836) + (locals.var_gf2 * (-locals.var_k_ds_dn7))) * locals.var_ds) + (assign44690_e57837 * locals.var_ds_dn7)), ((((locals.var_gf2_dn8 * assign44690_e57836) + (locals.var_gf2 * (-locals.var_k_ds_dn8))) * locals.var_ds) + (assign44690_e57837 * locals.var_ds_dn8)), ((((locals.var_gf2_dn9 * assign44690_e57836) + (locals.var_gf2 * (-locals.var_k_ds_dn9))) * locals.var_ds) + (assign44690_e57837 * locals.var_ds_dn9)),)
    } else {
        (locals.var_qc, locals.var_qc_dn4, locals.var_qc_dn6, locals.var_qc_dn7, locals.var_qc_dn8, locals.var_qc_dn9,)
    }
};
        locals.var_qc = assign44690_e57841;
        locals.var_qc_dn4 = assign44690_e57841_d_n4;
        locals.var_qc_dn6 = assign44690_e57841_d_n6;
        locals.var_qc_dn7 = assign44690_e57841_d_n7;
        locals.var_qc_dn8 = assign44690_e57841_d_n8;
        locals.var_qc_dn9 = assign44690_e57841_d_n9;

        let (assign44700_e57859, assign44700_e57859_d_n4, assign44700_e57859_d_n6, assign44700_e57859_d_n7, assign44700_e57859_d_n8, assign44700_e57859_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1225 != 0.0)) {
        let assign44700_e57850: f64 = (locals.var_delta_1s * locals.var_k_ds);
        let assign44700_e57851: f64 = (locals.var_es + assign44700_e57850);
        let assign44700_e57854: f64 = (locals.var_delta_nd * locals.var_xi2s);
        let assign44700_e57855: f64 = (assign44700_e57851 - assign44700_e57854);
        let assign44700_e57856: f64 = (locals.var_gf2 * assign44700_e57855);
        let assign44700_e57857: f64 = (2.0 - assign44700_e57856);
        (assign44700_e57857, (-((locals.var_gf2_dn4 * assign44700_e57855) + (locals.var_gf2 * ((locals.var_es_dn4 + ((locals.var_delta_1s_dn4 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn4))) - ((locals.var_delta_nd_dn4 * locals.var_xi2s) + (locals.var_delta_nd * locals.var_xi2s_dn4)))))), (-((locals.var_gf2_dn6 * assign44700_e57855) + (locals.var_gf2 * ((locals.var_es_dn6 + ((locals.var_delta_1s_dn6 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn6))) - ((locals.var_delta_nd_dn6 * locals.var_xi2s) + (locals.var_delta_nd * locals.var_xi2s_dn6)))))), (-((locals.var_gf2_dn7 * assign44700_e57855) + (locals.var_gf2 * ((locals.var_es_dn7 + ((locals.var_delta_1s_dn7 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn7))) - ((locals.var_delta_nd_dn7 * locals.var_xi2s) + (locals.var_delta_nd * locals.var_xi2s_dn7)))))), (-((locals.var_gf2_dn8 * assign44700_e57855) + (locals.var_gf2 * ((locals.var_es_dn8 + ((locals.var_delta_1s_dn8 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn8))) - ((locals.var_delta_nd_dn8 * locals.var_xi2s) + (locals.var_delta_nd * locals.var_xi2s_dn8)))))), (-((locals.var_gf2_dn9 * assign44700_e57855) + (locals.var_gf2 * ((locals.var_es_dn9 + ((locals.var_delta_1s_dn9 * locals.var_k_ds) + (locals.var_delta_1s * locals.var_k_ds_dn9))) - ((locals.var_delta_nd_dn9 * locals.var_xi2s) + (locals.var_delta_nd * locals.var_xi2s_dn9)))))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign44700_e57859;
        locals.var_temp__blk949_dn4 = assign44700_e57859_d_n4;
        locals.var_temp__blk949_dn6 = assign44700_e57859_d_n6;
        locals.var_temp__blk949_dn7 = assign44700_e57859_d_n7;
        locals.var_temp__blk949_dn8 = assign44700_e57859_d_n8;
        locals.var_temp__blk949_dn9 = assign44700_e57859_d_n9;

        let (assign44710_e57873, assign44710_e57873_d_n4, assign44710_e57873_d_n6, assign44710_e57873_d_n7, assign44710_e57873_d_n8, assign44710_e57873_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1225 != 0.0)) {
        let assign44710_e57865: f64 = (locals.var_pc * locals.var_pc);
        let assign44710_e57869: f64 = (locals.var_temp__blk949 * locals.var_qc);
        let assign44710_e57870: f64 = (2.0 * assign44710_e57869);
        let assign44710_e57871: f64 = (assign44710_e57865 - assign44710_e57870);
        (assign44710_e57871, (((locals.var_pc_dn4 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn4)) - (2.0 * ((locals.var_temp__blk949_dn4 * locals.var_qc) + (locals.var_temp__blk949 * locals.var_qc_dn4)))), (((locals.var_pc_dn6 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn6)) - (2.0 * ((locals.var_temp__blk949_dn6 * locals.var_qc) + (locals.var_temp__blk949 * locals.var_qc_dn6)))), (((locals.var_pc_dn7 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn7)) - (2.0 * ((locals.var_temp__blk949_dn7 * locals.var_qc) + (locals.var_temp__blk949 * locals.var_qc_dn7)))), (((locals.var_pc_dn8 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn8)) - (2.0 * ((locals.var_temp__blk949_dn8 * locals.var_qc) + (locals.var_temp__blk949 * locals.var_qc_dn8)))), (((locals.var_pc_dn9 * locals.var_pc) + (locals.var_pc * locals.var_pc_dn9)) - (2.0 * ((locals.var_temp__blk949_dn9 * locals.var_qc) + (locals.var_temp__blk949 * locals.var_qc_dn9)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign44710_e57873;
        locals.var_temp__blk949_dn4 = assign44710_e57873_d_n4;
        locals.var_temp__blk949_dn6 = assign44710_e57873_d_n6;
        locals.var_temp__blk949_dn7 = assign44710_e57873_d_n7;
        locals.var_temp__blk949_dn8 = assign44710_e57873_d_n8;
        locals.var_temp__blk949_dn9 = assign44710_e57873_d_n9;

        let (assign44720_e57886, assign44720_e57886_d_n4, assign44720_e57886_d_n6, assign44720_e57886_d_n7, assign44720_e57886_d_n8, assign44720_e57886_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1225 != 0.0)) {
        let assign44720_e57881: f64 = (locals.var_temp__blk949).sqrt();
        let assign44720_e57882: f64 = (locals.var_pc + assign44720_e57881);
        let assign44720_e57883: f64 = (locals.var_qc / assign44720_e57882);
        let assign44720_e57884: f64 = (2.0 * assign44720_e57883);
        (assign44720_e57884, (2.0 * (((locals.var_qc_dn4 * assign44720_e57882) - (locals.var_qc * (locals.var_pc_dn4 + (locals.var_temp__blk949_dn4 / (2.0 * assign44720_e57881))))) / (assign44720_e57882 * assign44720_e57882))), (2.0 * (((locals.var_qc_dn6 * assign44720_e57882) - (locals.var_qc * (locals.var_pc_dn6 + (locals.var_temp__blk949_dn6 / (2.0 * assign44720_e57881))))) / (assign44720_e57882 * assign44720_e57882))), (2.0 * (((locals.var_qc_dn7 * assign44720_e57882) - (locals.var_qc * (locals.var_pc_dn7 + (locals.var_temp__blk949_dn7 / (2.0 * assign44720_e57881))))) / (assign44720_e57882 * assign44720_e57882))), (2.0 * (((locals.var_qc_dn8 * assign44720_e57882) - (locals.var_qc * (locals.var_pc_dn8 + (locals.var_temp__blk949_dn8 / (2.0 * assign44720_e57881))))) / (assign44720_e57882 * assign44720_e57882))), (2.0 * (((locals.var_qc_dn9 * assign44720_e57882) - (locals.var_qc * (locals.var_pc_dn9 + (locals.var_temp__blk949_dn9 / (2.0 * assign44720_e57881))))) / (assign44720_e57882 * assign44720_e57882))),)
    } else {
        (locals.var_x_ds, locals.var_x_ds_dn4, locals.var_x_ds_dn6, locals.var_x_ds_dn7, locals.var_x_ds_dn8, locals.var_x_ds_dn9,)
    }
};
        locals.var_x_ds = assign44720_e57886;
        locals.var_x_ds_dn4 = assign44720_e57886_d_n4;
        locals.var_x_ds_dn6 = assign44720_e57886_d_n6;
        locals.var_x_ds_dn7 = assign44720_e57886_d_n7;
        locals.var_x_ds_dn8 = assign44720_e57886_d_n8;
        locals.var_x_ds_dn9 = assign44720_e57886_d_n9;

        let (assign44730_e57894, assign44730_e57894_d_n4, assign44730_e57894_d_n6, assign44730_e57894_d_n7, assign44730_e57894_d_n8, assign44730_e57894_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1225 != 0.0)) {
        let assign44730_e57892: f64 = (locals.var_x_s + locals.var_x_ds);
        (assign44730_e57892, (locals.var_x_s_dn4 + locals.var_x_ds_dn4), (locals.var_x_s_dn6 + locals.var_x_ds_dn6), (locals.var_x_s_dn7 + locals.var_x_ds_dn7), (locals.var_x_s_dn8 + locals.var_x_ds_dn8), (locals.var_x_s_dn9 + locals.var_x_ds_dn9),)
    } else {
        (locals.var_x_d, locals.var_x_d_dn4, locals.var_x_d_dn6, locals.var_x_d_dn7, locals.var_x_d_dn8, locals.var_x_d_dn9,)
    }
};
        locals.var_x_d = assign44730_e57894;
        locals.var_x_d_dn4 = assign44730_e57894_d_n4;
        locals.var_x_d_dn6 = assign44730_e57894_d_n6;
        locals.var_x_d_dn7 = assign44730_e57894_d_n7;
        locals.var_x_d_dn8 = assign44730_e57894_d_n8;
        locals.var_x_d_dn9 = assign44730_e57894_d_n9;

        let (assign44740_e57900, assign44740_e57900_d_n4, assign44740_e57900_d_n6, assign44740_e57900_d_n7, assign44740_e57900_d_n8, assign44740_e57900_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44740_e57898: f64 = (locals.var_x_ds * locals.var_phit1);
        (assign44740_e57898, ((locals.var_x_ds_dn4 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn4)), ((locals.var_x_ds_dn6 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn6)), ((locals.var_x_ds_dn7 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn7)), ((locals.var_x_ds_dn8 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn8)), ((locals.var_x_ds_dn9 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn9)),)
    } else {
        (locals.var_dps, locals.var_dps_dn4, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8, locals.var_dps_dn9,)
    }
};
        locals.var_dps = assign44740_e57900;
        locals.var_dps_dn4 = assign44740_e57900_d_n4;
        locals.var_dps_dn6 = assign44740_e57900_d_n6;
        locals.var_dps_dn7 = assign44740_e57900_d_n7;
        locals.var_dps_dn8 = assign44740_e57900_d_n8;
        locals.var_dps_dn9 = assign44740_e57900_d_n9;

        let (assign44750_e57912, assign44750_e57912_d_n4, assign44750_e57912_d_n6, assign44750_e57912_d_n7, assign44750_e57912_d_n8, assign44750_e57912_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44750_e57904: f64 = (locals.var_x_d * locals.var_x_d);
        let assign44750_e57908: f64 = (locals.var_x_d * locals.var_x_d);
        let assign44750_e57909: f64 = (2.0 + assign44750_e57908);
        let assign44750_e57910: f64 = (assign44750_e57904 / assign44750_e57909);
        (assign44750_e57910, (((((locals.var_x_d_dn4 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn4)) * assign44750_e57909) - (assign44750_e57904 * ((locals.var_x_d_dn4 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn4)))) / (assign44750_e57909 * assign44750_e57909)), (((((locals.var_x_d_dn6 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn6)) * assign44750_e57909) - (assign44750_e57904 * ((locals.var_x_d_dn6 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn6)))) / (assign44750_e57909 * assign44750_e57909)), (((((locals.var_x_d_dn7 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn7)) * assign44750_e57909) - (assign44750_e57904 * ((locals.var_x_d_dn7 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn7)))) / (assign44750_e57909 * assign44750_e57909)), (((((locals.var_x_d_dn8 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn8)) * assign44750_e57909) - (assign44750_e57904 * ((locals.var_x_d_dn8 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn8)))) / (assign44750_e57909 * assign44750_e57909)), (((((locals.var_x_d_dn9 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn9)) * assign44750_e57909) - (assign44750_e57904 * ((locals.var_x_d_dn9 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn9)))) / (assign44750_e57909 * assign44750_e57909)),)
    } else {
        (locals.var_xi0d, locals.var_xi0d_dn4, locals.var_xi0d_dn6, locals.var_xi0d_dn7, locals.var_xi0d_dn8, locals.var_xi0d_dn9,)
    }
};
        locals.var_xi0d = assign44750_e57912;
        locals.var_xi0d_dn4 = assign44750_e57912_d_n4;
        locals.var_xi0d_dn6 = assign44750_e57912_d_n6;
        locals.var_xi0d_dn7 = assign44750_e57912_d_n7;
        locals.var_xi0d_dn8 = assign44750_e57912_d_n8;
        locals.var_xi0d_dn9 = assign44750_e57912_d_n9;

        let assign44760_e57915: f64 = if locals.var_x_d < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1226 = assign44760_e57915;

        let (assign44770_e57923, assign44770_e57923_d_n4, assign44770_e57923_d_n6, assign44770_e57923_d_n7, assign44770_e57923_d_n8, assign44770_e57923_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign44770_e57920: f64 = (-locals.var_x_d);
        let assign44770_e57921: f64 = (assign44770_e57920).exp();
        (assign44770_e57921, (assign44770_e57921 * (-locals.var_x_d_dn4)), (assign44770_e57921 * (-locals.var_x_d_dn6)), (assign44770_e57921 * (-locals.var_x_d_dn7)), (assign44770_e57921 * (-locals.var_x_d_dn8)), (assign44770_e57921 * (-locals.var_x_d_dn9)),)
    } else {
        (locals.var_ed, locals.var_ed_dn4, locals.var_ed_dn6, locals.var_ed_dn7, locals.var_ed_dn8, locals.var_ed_dn9,)
    }
};
        locals.var_ed = assign44770_e57923;
        locals.var_ed_dn4 = assign44770_e57923_d_n4;
        locals.var_ed_dn6 = assign44770_e57923_d_n6;
        locals.var_ed_dn7 = assign44770_e57923_d_n7;
        locals.var_ed_dn8 = assign44770_e57923_d_n8;
        locals.var_ed_dn9 = assign44770_e57923_d_n9;

        let assign44780_e57926: f64 = if locals.var_x_d < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1227 = assign44780_e57926;

        let (assign44790_e57950, assign44790_e57950_d_n4, assign44790_e57950_d_n6, assign44790_e57950_d_n7, assign44790_e57950_d_n8, assign44790_e57950_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1227 != 0.0)) {
        let assign44790_e57935: f64 = (locals.var_x_d * locals.var_x_d);
        let assign44790_e57942: f64 = (0.25 * locals.var_x_d);
        let assign44790_e57943: f64 = (1.0 - assign44790_e57942);
        let assign44790_e57944: f64 = (locals.var_x_d * assign44790_e57943);
        let assign44790_e57945: f64 = (0.3333333333333333 * assign44790_e57944);
        let assign44790_e57946: f64 = (1.0 - assign44790_e57945);
        let assign44790_e57947: f64 = (assign44790_e57935 * assign44790_e57946);
        let assign44790_e57948: f64 = (0.5 * assign44790_e57947);
        (assign44790_e57948, (0.5 * ((((locals.var_x_d_dn4 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn4)) * assign44790_e57946) + (assign44790_e57935 * (-(0.3333333333333333 * ((locals.var_x_d_dn4 * assign44790_e57943) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn4))))))))), (0.5 * ((((locals.var_x_d_dn6 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn6)) * assign44790_e57946) + (assign44790_e57935 * (-(0.3333333333333333 * ((locals.var_x_d_dn6 * assign44790_e57943) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn6))))))))), (0.5 * ((((locals.var_x_d_dn7 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn7)) * assign44790_e57946) + (assign44790_e57935 * (-(0.3333333333333333 * ((locals.var_x_d_dn7 * assign44790_e57943) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn7))))))))), (0.5 * ((((locals.var_x_d_dn8 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn8)) * assign44790_e57946) + (assign44790_e57935 * (-(0.3333333333333333 * ((locals.var_x_d_dn8 * assign44790_e57943) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn8))))))))), (0.5 * ((((locals.var_x_d_dn9 * locals.var_x_d) + (locals.var_x_d * locals.var_x_d_dn9)) * assign44790_e57946) + (assign44790_e57935 * (-(0.3333333333333333 * ((locals.var_x_d_dn9 * assign44790_e57943) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn9))))))))),)
    } else {
        (locals.var_pd, locals.var_pd_dn4, locals.var_pd_dn6, locals.var_pd_dn7, locals.var_pd_dn8, locals.var_pd_dn9,)
    }
};
        locals.var_pd = assign44790_e57950;
        locals.var_pd_dn4 = assign44790_e57950_d_n4;
        locals.var_pd_dn6 = assign44790_e57950_d_n6;
        locals.var_pd_dn7 = assign44790_e57950_d_n7;
        locals.var_pd_dn8 = assign44790_e57950_d_n8;
        locals.var_pd_dn9 = assign44790_e57950_d_n9;

        let (assign44800_e57969, assign44800_e57969_d_n4, assign44800_e57969_d_n6, assign44800_e57969_d_n7, assign44800_e57969_d_n8, assign44800_e57969_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1227 != 0.0)) {
        let assign44800_e57962: f64 = (0.25 * locals.var_x_d);
        let assign44800_e57963: f64 = (1.0 - assign44800_e57962);
        let assign44800_e57964: f64 = (locals.var_x_d * assign44800_e57963);
        let assign44800_e57965: f64 = (0.3333333333333333 * assign44800_e57964);
        let assign44800_e57966: f64 = (1.0 - assign44800_e57965);
        let assign44800_e57967: f64 = (assign44800_e57966).sqrt();
        (assign44800_e57967, ((-(0.3333333333333333 * ((locals.var_x_d_dn4 * assign44800_e57963) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn4)))))) / (2.0 * assign44800_e57967)), ((-(0.3333333333333333 * ((locals.var_x_d_dn6 * assign44800_e57963) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn6)))))) / (2.0 * assign44800_e57967)), ((-(0.3333333333333333 * ((locals.var_x_d_dn7 * assign44800_e57963) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn7)))))) / (2.0 * assign44800_e57967)), ((-(0.3333333333333333 * ((locals.var_x_d_dn8 * assign44800_e57963) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn8)))))) / (2.0 * assign44800_e57967)), ((-(0.3333333333333333 * ((locals.var_x_d_dn9 * assign44800_e57963) + (locals.var_x_d * (-(0.25 * locals.var_x_d_dn9)))))) / (2.0 * assign44800_e57967)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign44800_e57969;
        locals.var_temp__blk949_dn4 = assign44800_e57969_d_n4;
        locals.var_temp__blk949_dn6 = assign44800_e57969_d_n6;
        locals.var_temp__blk949_dn7 = assign44800_e57969_d_n7;
        locals.var_temp__blk949_dn8 = assign44800_e57969_d_n8;
        locals.var_temp__blk949_dn9 = assign44800_e57969_d_n9;

        let (assign44810_e57981, assign44810_e57981_d_n4, assign44810_e57981_d_n6, assign44810_e57981_d_n7, assign44810_e57981_d_n8, assign44810_e57981_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1227 != 0.0)) {
        let assign44810_e57978: f64 = (locals.var_x_d * locals.var_temp__blk949);
        let assign44810_e57979: f64 = (0.7071067811865475 * assign44810_e57978);
        (assign44810_e57979, (0.7071067811865475 * ((locals.var_x_d_dn4 * locals.var_temp__blk949) + (locals.var_x_d * locals.var_temp__blk949_dn4))), (0.7071067811865475 * ((locals.var_x_d_dn6 * locals.var_temp__blk949) + (locals.var_x_d * locals.var_temp__blk949_dn6))), (0.7071067811865475 * ((locals.var_x_d_dn7 * locals.var_temp__blk949) + (locals.var_x_d * locals.var_temp__blk949_dn7))), (0.7071067811865475 * ((locals.var_x_d_dn8 * locals.var_temp__blk949) + (locals.var_x_d * locals.var_temp__blk949_dn8))), (0.7071067811865475 * ((locals.var_x_d_dn9 * locals.var_temp__blk949) + (locals.var_x_d * locals.var_temp__blk949_dn9))),)
    } else {
        (locals.var_sqd, locals.var_sqd_dn4, locals.var_sqd_dn6, locals.var_sqd_dn7, locals.var_sqd_dn8, locals.var_sqd_dn9,)
    }
};
        locals.var_sqd = assign44810_e57981;
        locals.var_sqd_dn4 = assign44810_e57981_d_n4;
        locals.var_sqd_dn6 = assign44810_e57981_d_n6;
        locals.var_sqd_dn7 = assign44810_e57981_d_n7;
        locals.var_sqd_dn8 = assign44810_e57981_d_n8;
        locals.var_sqd_dn9 = assign44810_e57981_d_n9;

        let (assign44820_e58003, assign44820_e58003_d_n4, assign44820_e58003_d_n6, assign44820_e58003_d_n7, assign44820_e58003_d_n8, assign44820_e58003_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1227 != 0.0)) {
        let assign44820_e57989: f64 = (0.16666666666666666 * locals.var_delta_nd);
        let assign44820_e57991: f64 = (assign44820_e57989 * locals.var_x_d);
        let assign44820_e57993: f64 = (assign44820_e57991 * locals.var_x_d);
        let assign44820_e57995: f64 = (assign44820_e57993 * locals.var_x_d);
        let assign44820_e57999: f64 = (1.75 * locals.var_x_d);
        let assign44820_e58000: f64 = (1.0 + assign44820_e57999);
        let assign44820_e58001: f64 = (assign44820_e57995 * assign44820_e58000);
        (assign44820_e58001, (((((((((0.16666666666666666 * locals.var_delta_nd_dn4) * locals.var_x_d) + (assign44820_e57989 * locals.var_x_d_dn4)) * locals.var_x_d) + (assign44820_e57991 * locals.var_x_d_dn4)) * locals.var_x_d) + (assign44820_e57993 * locals.var_x_d_dn4)) * assign44820_e58000) + (assign44820_e57995 * (1.75 * locals.var_x_d_dn4))), (((((((((0.16666666666666666 * locals.var_delta_nd_dn6) * locals.var_x_d) + (assign44820_e57989 * locals.var_x_d_dn6)) * locals.var_x_d) + (assign44820_e57991 * locals.var_x_d_dn6)) * locals.var_x_d) + (assign44820_e57993 * locals.var_x_d_dn6)) * assign44820_e58000) + (assign44820_e57995 * (1.75 * locals.var_x_d_dn6))), (((((((((0.16666666666666666 * locals.var_delta_nd_dn7) * locals.var_x_d) + (assign44820_e57989 * locals.var_x_d_dn7)) * locals.var_x_d) + (assign44820_e57991 * locals.var_x_d_dn7)) * locals.var_x_d) + (assign44820_e57993 * locals.var_x_d_dn7)) * assign44820_e58000) + (assign44820_e57995 * (1.75 * locals.var_x_d_dn7))), (((((((((0.16666666666666666 * locals.var_delta_nd_dn8) * locals.var_x_d) + (assign44820_e57989 * locals.var_x_d_dn8)) * locals.var_x_d) + (assign44820_e57991 * locals.var_x_d_dn8)) * locals.var_x_d) + (assign44820_e57993 * locals.var_x_d_dn8)) * assign44820_e58000) + (assign44820_e57995 * (1.75 * locals.var_x_d_dn8))), (((((((((0.16666666666666666 * locals.var_delta_nd_dn9) * locals.var_x_d) + (assign44820_e57989 * locals.var_x_d_dn9)) * locals.var_x_d) + (assign44820_e57991 * locals.var_x_d_dn9)) * locals.var_x_d) + (assign44820_e57993 * locals.var_x_d_dn9)) * assign44820_e58000) + (assign44820_e57995 * (1.75 * locals.var_x_d_dn9))),)
    } else {
        (locals.var_dd, locals.var_dd_dn4, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8, locals.var_dd_dn9,)
    }
};
        locals.var_dd = assign44820_e58003;
        locals.var_dd_dn4 = assign44820_e58003_d_n4;
        locals.var_dd_dn6 = assign44820_e58003_d_n6;
        locals.var_dd_dn7 = assign44820_e58003_d_n7;
        locals.var_dd_dn8 = assign44820_e58003_d_n8;
        locals.var_dd_dn9 = assign44820_e58003_d_n9;

        let (assign44830_e58016, assign44830_e58016_d_n4, assign44830_e58016_d_n6, assign44830_e58016_d_n7, assign44830_e58016_d_n8, assign44830_e58016_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1227 == 0.0)) {
        let assign44830_e58012: f64 = (locals.var_x_d - 1.0);
        let assign44830_e58014: f64 = (assign44830_e58012 + locals.var_ed);
        (assign44830_e58014, (locals.var_x_d_dn4 + locals.var_ed_dn4), (locals.var_x_d_dn6 + locals.var_ed_dn6), (locals.var_x_d_dn7 + locals.var_ed_dn7), (locals.var_x_d_dn8 + locals.var_ed_dn8), (locals.var_x_d_dn9 + locals.var_ed_dn9),)
    } else {
        (locals.var_pd, locals.var_pd_dn4, locals.var_pd_dn6, locals.var_pd_dn7, locals.var_pd_dn8, locals.var_pd_dn9,)
    }
};
        locals.var_pd = assign44830_e58016;
        locals.var_pd_dn4 = assign44830_e58016_d_n4;
        locals.var_pd_dn6 = assign44830_e58016_d_n6;
        locals.var_pd_dn7 = assign44830_e58016_d_n7;
        locals.var_pd_dn8 = assign44830_e58016_d_n8;
        locals.var_pd_dn9 = assign44830_e58016_d_n9;

        let (assign44840_e58026, assign44840_e58026_d_n4, assign44840_e58026_d_n6, assign44840_e58026_d_n7, assign44840_e58026_d_n8, assign44840_e58026_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1227 == 0.0)) {
        let assign44840_e58024: f64 = (locals.var_pd).sqrt();
        (assign44840_e58024, (locals.var_pd_dn4 / (2.0 * assign44840_e58024)), (locals.var_pd_dn6 / (2.0 * assign44840_e58024)), (locals.var_pd_dn7 / (2.0 * assign44840_e58024)), (locals.var_pd_dn8 / (2.0 * assign44840_e58024)), (locals.var_pd_dn9 / (2.0 * assign44840_e58024)),)
    } else {
        (locals.var_sqd, locals.var_sqd_dn4, locals.var_sqd_dn6, locals.var_sqd_dn7, locals.var_sqd_dn8, locals.var_sqd_dn9,)
    }
};
        locals.var_sqd = assign44840_e58026;
        locals.var_sqd_dn4 = assign44840_e58026_d_n4;
        locals.var_sqd_dn6 = assign44840_e58026_d_n6;
        locals.var_sqd_dn7 = assign44840_e58026_d_n7;
        locals.var_sqd_dn8 = assign44840_e58026_d_n8;
        locals.var_sqd_dn9 = assign44840_e58026_d_n9;

        let (assign44850_e58045, assign44850_e58045_d_n4, assign44850_e58045_d_n6, assign44850_e58045_d_n7, assign44850_e58045_d_n8, assign44850_e58045_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 != 0.0)) && (locals.var_guard1227 == 0.0)) {
        let assign44850_e58036: f64 = (1.0 / locals.var_ed);
        let assign44850_e58038: f64 = (assign44850_e58036 - locals.var_x_d);
        let assign44850_e58040: f64 = (assign44850_e58038 - 1.0);
        let assign44850_e58042: f64 = (assign44850_e58040 - locals.var_xi0d);
        let assign44850_e58043: f64 = (locals.var_delta_nd * assign44850_e58042);
        (assign44850_e58043, ((locals.var_delta_nd_dn4 * assign44850_e58042) + (locals.var_delta_nd * (((-(locals.var_ed_dn4 / (locals.var_ed * locals.var_ed))) - locals.var_x_d_dn4) - locals.var_xi0d_dn4))), ((locals.var_delta_nd_dn6 * assign44850_e58042) + (locals.var_delta_nd * (((-(locals.var_ed_dn6 / (locals.var_ed * locals.var_ed))) - locals.var_x_d_dn6) - locals.var_xi0d_dn6))), ((locals.var_delta_nd_dn7 * assign44850_e58042) + (locals.var_delta_nd * (((-(locals.var_ed_dn7 / (locals.var_ed * locals.var_ed))) - locals.var_x_d_dn7) - locals.var_xi0d_dn7))), ((locals.var_delta_nd_dn8 * assign44850_e58042) + (locals.var_delta_nd * (((-(locals.var_ed_dn8 / (locals.var_ed * locals.var_ed))) - locals.var_x_d_dn8) - locals.var_xi0d_dn8))), ((locals.var_delta_nd_dn9 * assign44850_e58042) + (locals.var_delta_nd * (((-(locals.var_ed_dn9 / (locals.var_ed * locals.var_ed))) - locals.var_x_d_dn9) - locals.var_xi0d_dn9))),)
    } else {
        (locals.var_dd, locals.var_dd_dn4, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8, locals.var_dd_dn9,)
    }
};
        locals.var_dd = assign44850_e58045;
        locals.var_dd_dn4 = assign44850_e58045_d_n4;
        locals.var_dd_dn6 = assign44850_e58045_d_n6;
        locals.var_dd_dn7 = assign44850_e58045_d_n7;
        locals.var_dd_dn8 = assign44850_e58045_d_n8;
        locals.var_dd_dn9 = assign44850_e58045_d_n9;

        let assign44860_e58049: f64 = (locals.var_xn_d - 230.25850929940458);
        let assign44860_e58050: f64 = if locals.var_x_d > assign44860_e58049 { 1.0 } else { 0.0 };
        locals.var_guard1228 = assign44860_e58050;

        let (assign44870_e58062, assign44870_e58062_d_n4, assign44870_e58062_d_n6, assign44870_e58062_d_n7, assign44870_e58062_d_n8, assign44870_e58062_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1228 != 0.0)) {
        let assign44870_e58059: f64 = (locals.var_x_d - locals.var_xn_d);
        let assign44870_e58060: f64 = (assign44870_e58059).exp();
        (assign44870_e58060, (assign44870_e58060 * (locals.var_x_d_dn4 - locals.var_xn_d_dn4)), (assign44870_e58060 * (locals.var_x_d_dn6 - locals.var_xn_d_dn6)), (assign44870_e58060 * (locals.var_x_d_dn7 - locals.var_xn_d_dn7)), (assign44870_e58060 * (locals.var_x_d_dn8 - locals.var_xn_d_dn8)), (assign44870_e58060 * (locals.var_x_d_dn9 - locals.var_xn_d_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign44870_e58062;
        locals.var_temp__blk949_dn4 = assign44870_e58062_d_n4;
        locals.var_temp__blk949_dn6 = assign44870_e58062_d_n6;
        locals.var_temp__blk949_dn7 = assign44870_e58062_d_n7;
        locals.var_temp__blk949_dn8 = assign44870_e58062_d_n8;
        locals.var_temp__blk949_dn9 = assign44870_e58062_d_n9;

        let (assign44880_e58073, assign44880_e58073_d_n4, assign44880_e58073_d_n6, assign44880_e58073_d_n7, assign44880_e58073_d_n8, assign44880_e58073_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1228 != 0.0)) {
        let assign44880_e58071: f64 = (locals.var_delta_nd / locals.var_temp__blk949);
        (assign44880_e58071, (((locals.var_delta_nd_dn4 * locals.var_temp__blk949) - (locals.var_delta_nd * locals.var_temp__blk949_dn4)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_delta_nd_dn6 * locals.var_temp__blk949) - (locals.var_delta_nd * locals.var_temp__blk949_dn6)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_delta_nd_dn7 * locals.var_temp__blk949) - (locals.var_delta_nd * locals.var_temp__blk949_dn7)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_delta_nd_dn8 * locals.var_temp__blk949) - (locals.var_delta_nd * locals.var_temp__blk949_dn8)) / (locals.var_temp__blk949 * locals.var_temp__blk949)), (((locals.var_delta_nd_dn9 * locals.var_temp__blk949) - (locals.var_delta_nd * locals.var_temp__blk949_dn9)) / (locals.var_temp__blk949 * locals.var_temp__blk949)),)
    } else {
        (locals.var_ed, locals.var_ed_dn4, locals.var_ed_dn6, locals.var_ed_dn7, locals.var_ed_dn8, locals.var_ed_dn9,)
    }
};
        locals.var_ed = assign44880_e58073;
        locals.var_ed_dn4 = assign44880_e58073_d_n4;
        locals.var_ed_dn6 = assign44880_e58073_d_n6;
        locals.var_ed_dn7 = assign44880_e58073_d_n7;
        locals.var_ed_dn8 = assign44880_e58073_d_n8;
        locals.var_ed_dn9 = assign44880_e58073_d_n9;

        let (assign44890_e58090, assign44890_e58090_d_n4, assign44890_e58090_d_n6, assign44890_e58090_d_n7, assign44890_e58090_d_n8, assign44890_e58090_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1228 != 0.0)) {
        let assign44890_e58084: f64 = (locals.var_x_d + 1.0);
        let assign44890_e58086: f64 = (assign44890_e58084 + locals.var_xi0d);
        let assign44890_e58087: f64 = (locals.var_delta_nd * assign44890_e58086);
        let assign44890_e58088: f64 = (locals.var_temp__blk949 - assign44890_e58087);
        (assign44890_e58088, (locals.var_temp__blk949_dn4 - ((locals.var_delta_nd_dn4 * assign44890_e58086) + (locals.var_delta_nd * (locals.var_x_d_dn4 + locals.var_xi0d_dn4)))), (locals.var_temp__blk949_dn6 - ((locals.var_delta_nd_dn6 * assign44890_e58086) + (locals.var_delta_nd * (locals.var_x_d_dn6 + locals.var_xi0d_dn6)))), (locals.var_temp__blk949_dn7 - ((locals.var_delta_nd_dn7 * assign44890_e58086) + (locals.var_delta_nd * (locals.var_x_d_dn7 + locals.var_xi0d_dn7)))), (locals.var_temp__blk949_dn8 - ((locals.var_delta_nd_dn8 * assign44890_e58086) + (locals.var_delta_nd * (locals.var_x_d_dn8 + locals.var_xi0d_dn8)))), (locals.var_temp__blk949_dn9 - ((locals.var_delta_nd_dn9 * assign44890_e58086) + (locals.var_delta_nd * (locals.var_x_d_dn9 + locals.var_xi0d_dn9)))),)
    } else {
        (locals.var_dd, locals.var_dd_dn4, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8, locals.var_dd_dn9,)
    }
};
        locals.var_dd = assign44890_e58090;
        locals.var_dd_dn4 = assign44890_e58090_d_n4;
        locals.var_dd_dn6 = assign44890_e58090_d_n6;
        locals.var_dd_dn7 = assign44890_e58090_d_n7;
        locals.var_dd_dn8 = assign44890_e58090_d_n8;
        locals.var_dd_dn9 = assign44890_e58090_d_n9;

        let (assign44900_e58122, assign44900_e58122_d_n4, assign44900_e58122_d_n6, assign44900_e58122_d_n7, assign44900_e58122_d_n8, assign44900_e58122_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1228 == 0.0)) {
        let assign44900_e58102: f64 = (locals.var_x_d - 230.25850929940458);
        let assign44900_e58107: f64 = (locals.var_x_d - 230.25850929940458);
        let assign44900_e58111: f64 = (locals.var_x_d - 230.25850929940458);
        let assign44900_e58113: f64 = (assign44900_e58111 * 0.3333333333333333);
        let assign44900_e58114: f64 = (1.0 + assign44900_e58113);
        let assign44900_e58115: f64 = (assign44900_e58107 * assign44900_e58114);
        let assign44900_e58116: f64 = (0.5 * assign44900_e58115);
        let assign44900_e58117: f64 = (1.0 + assign44900_e58116);
        let assign44900_e58118: f64 = (assign44900_e58102 * assign44900_e58117);
        let assign44900_e58119: f64 = (1.0 + assign44900_e58118);
        let assign44900_e58120: f64 = (1e-100 / assign44900_e58119);
        (assign44900_e58120, (-((1e-100 * ((locals.var_x_d_dn4 * assign44900_e58117) + (assign44900_e58102 * (0.5 * ((locals.var_x_d_dn4 * assign44900_e58114) + (assign44900_e58107 * (locals.var_x_d_dn4 * 0.3333333333333333))))))) / (assign44900_e58119 * assign44900_e58119))), (-((1e-100 * ((locals.var_x_d_dn6 * assign44900_e58117) + (assign44900_e58102 * (0.5 * ((locals.var_x_d_dn6 * assign44900_e58114) + (assign44900_e58107 * (locals.var_x_d_dn6 * 0.3333333333333333))))))) / (assign44900_e58119 * assign44900_e58119))), (-((1e-100 * ((locals.var_x_d_dn7 * assign44900_e58117) + (assign44900_e58102 * (0.5 * ((locals.var_x_d_dn7 * assign44900_e58114) + (assign44900_e58107 * (locals.var_x_d_dn7 * 0.3333333333333333))))))) / (assign44900_e58119 * assign44900_e58119))), (-((1e-100 * ((locals.var_x_d_dn8 * assign44900_e58117) + (assign44900_e58102 * (0.5 * ((locals.var_x_d_dn8 * assign44900_e58114) + (assign44900_e58107 * (locals.var_x_d_dn8 * 0.3333333333333333))))))) / (assign44900_e58119 * assign44900_e58119))), (-((1e-100 * ((locals.var_x_d_dn9 * assign44900_e58117) + (assign44900_e58102 * (0.5 * ((locals.var_x_d_dn9 * assign44900_e58114) + (assign44900_e58107 * (locals.var_x_d_dn9 * 0.3333333333333333))))))) / (assign44900_e58119 * assign44900_e58119))),)
    } else {
        (locals.var_ed, locals.var_ed_dn4, locals.var_ed_dn6, locals.var_ed_dn7, locals.var_ed_dn8, locals.var_ed_dn9,)
    }
};
        locals.var_ed = assign44900_e58122;
        locals.var_ed_dn4 = assign44900_e58122_d_n4;
        locals.var_ed_dn6 = assign44900_e58122_d_n6;
        locals.var_ed_dn7 = assign44900_e58122_d_n7;
        locals.var_ed_dn8 = assign44900_e58122_d_n8;
        locals.var_ed_dn9 = assign44900_e58122_d_n9;

        let (assign44910_e58160, assign44910_e58160_d_n4, assign44910_e58160_d_n6, assign44910_e58160_d_n7, assign44910_e58160_d_n8, assign44910_e58160_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1228 == 0.0)) {
        let assign44910_e58134: f64 = (locals.var_xn_d - locals.var_x_d);
        let assign44910_e58136: f64 = (assign44910_e58134 - 230.25850929940458);
        let assign44910_e58141: f64 = (locals.var_xn_d - locals.var_x_d);
        let assign44910_e58143: f64 = (assign44910_e58141 - 230.25850929940458);
        let assign44910_e58147: f64 = (locals.var_xn_d - locals.var_x_d);
        let assign44910_e58149: f64 = (assign44910_e58147 - 230.25850929940458);
        let assign44910_e58151: f64 = (assign44910_e58149 * 0.3333333333333333);
        let assign44910_e58152: f64 = (1.0 + assign44910_e58151);
        let assign44910_e58153: f64 = (assign44910_e58143 * assign44910_e58152);
        let assign44910_e58154: f64 = (0.5 * assign44910_e58153);
        let assign44910_e58155: f64 = (1.0 + assign44910_e58154);
        let assign44910_e58156: f64 = (assign44910_e58136 * assign44910_e58155);
        let assign44910_e58157: f64 = (1.0 + assign44910_e58156);
        let assign44910_e58158: f64 = (1e-100 / assign44910_e58157);
        (assign44910_e58158, (-((1e-100 * (((locals.var_xn_d_dn4 - locals.var_x_d_dn4) * assign44910_e58155) + (assign44910_e58136 * (0.5 * (((locals.var_xn_d_dn4 - locals.var_x_d_dn4) * assign44910_e58152) + (assign44910_e58143 * ((locals.var_xn_d_dn4 - locals.var_x_d_dn4) * 0.3333333333333333))))))) / (assign44910_e58157 * assign44910_e58157))), (-((1e-100 * (((locals.var_xn_d_dn6 - locals.var_x_d_dn6) * assign44910_e58155) + (assign44910_e58136 * (0.5 * (((locals.var_xn_d_dn6 - locals.var_x_d_dn6) * assign44910_e58152) + (assign44910_e58143 * ((locals.var_xn_d_dn6 - locals.var_x_d_dn6) * 0.3333333333333333))))))) / (assign44910_e58157 * assign44910_e58157))), (-((1e-100 * (((locals.var_xn_d_dn7 - locals.var_x_d_dn7) * assign44910_e58155) + (assign44910_e58136 * (0.5 * (((locals.var_xn_d_dn7 - locals.var_x_d_dn7) * assign44910_e58152) + (assign44910_e58143 * ((locals.var_xn_d_dn7 - locals.var_x_d_dn7) * 0.3333333333333333))))))) / (assign44910_e58157 * assign44910_e58157))), (-((1e-100 * (((locals.var_xn_d_dn8 - locals.var_x_d_dn8) * assign44910_e58155) + (assign44910_e58136 * (0.5 * (((locals.var_xn_d_dn8 - locals.var_x_d_dn8) * assign44910_e58152) + (assign44910_e58143 * ((locals.var_xn_d_dn8 - locals.var_x_d_dn8) * 0.3333333333333333))))))) / (assign44910_e58157 * assign44910_e58157))), (-((1e-100 * (((locals.var_xn_d_dn9 - locals.var_x_d_dn9) * assign44910_e58155) + (assign44910_e58136 * (0.5 * (((locals.var_xn_d_dn9 - locals.var_x_d_dn9) * assign44910_e58152) + (assign44910_e58143 * ((locals.var_xn_d_dn9 - locals.var_x_d_dn9) * 0.3333333333333333))))))) / (assign44910_e58157 * assign44910_e58157))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign44910_e58160;
        locals.var_temp__blk949_dn4 = assign44910_e58160_d_n4;
        locals.var_temp__blk949_dn6 = assign44910_e58160_d_n6;
        locals.var_temp__blk949_dn7 = assign44910_e58160_d_n7;
        locals.var_temp__blk949_dn8 = assign44910_e58160_d_n8;
        locals.var_temp__blk949_dn9 = assign44910_e58160_d_n9;

        let (assign44920_e58178, assign44920_e58178_d_n4, assign44920_e58178_d_n6, assign44920_e58178_d_n7, assign44920_e58178_d_n8, assign44920_e58178_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1228 == 0.0)) {
        let assign44920_e58172: f64 = (locals.var_x_d + 1.0);
        let assign44920_e58174: f64 = (assign44920_e58172 + locals.var_xi0d);
        let assign44920_e58175: f64 = (locals.var_delta_nd * assign44920_e58174);
        let assign44920_e58176: f64 = (locals.var_temp__blk949 - assign44920_e58175);
        (assign44920_e58176, (locals.var_temp__blk949_dn4 - ((locals.var_delta_nd_dn4 * assign44920_e58174) + (locals.var_delta_nd * (locals.var_x_d_dn4 + locals.var_xi0d_dn4)))), (locals.var_temp__blk949_dn6 - ((locals.var_delta_nd_dn6 * assign44920_e58174) + (locals.var_delta_nd * (locals.var_x_d_dn6 + locals.var_xi0d_dn6)))), (locals.var_temp__blk949_dn7 - ((locals.var_delta_nd_dn7 * assign44920_e58174) + (locals.var_delta_nd * (locals.var_x_d_dn7 + locals.var_xi0d_dn7)))), (locals.var_temp__blk949_dn8 - ((locals.var_delta_nd_dn8 * assign44920_e58174) + (locals.var_delta_nd * (locals.var_x_d_dn8 + locals.var_xi0d_dn8)))), (locals.var_temp__blk949_dn9 - ((locals.var_delta_nd_dn9 * assign44920_e58174) + (locals.var_delta_nd * (locals.var_x_d_dn9 + locals.var_xi0d_dn9)))),)
    } else {
        (locals.var_dd, locals.var_dd_dn4, locals.var_dd_dn6, locals.var_dd_dn7, locals.var_dd_dn8, locals.var_dd_dn9,)
    }
};
        locals.var_dd = assign44920_e58178;
        locals.var_dd_dn4 = assign44920_e58178_d_n4;
        locals.var_dd_dn6 = assign44920_e58178_d_n6;
        locals.var_dd_dn7 = assign44920_e58178_d_n7;
        locals.var_dd_dn8 = assign44920_e58178_d_n8;
        locals.var_dd_dn9 = assign44920_e58178_d_n9;

        let (assign44930_e58189, assign44930_e58189_d_n4, assign44930_e58189_d_n6, assign44930_e58189_d_n7, assign44930_e58189_d_n8, assign44930_e58189_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1226 == 0.0)) {
        let assign44930_e58185: f64 = (locals.var_x_d - 1.0);
        let assign44930_e58187: f64 = (assign44930_e58185 + locals.var_ed);
        (assign44930_e58187, (locals.var_x_d_dn4 + locals.var_ed_dn4), (locals.var_x_d_dn6 + locals.var_ed_dn6), (locals.var_x_d_dn7 + locals.var_ed_dn7), (locals.var_x_d_dn8 + locals.var_ed_dn8), (locals.var_x_d_dn9 + locals.var_ed_dn9),)
    } else {
        (locals.var_pd, locals.var_pd_dn4, locals.var_pd_dn6, locals.var_pd_dn7, locals.var_pd_dn8, locals.var_pd_dn9,)
    }
};
        locals.var_pd = assign44930_e58189;
        locals.var_pd_dn4 = assign44930_e58189_d_n4;
        locals.var_pd_dn6 = assign44930_e58189_d_n6;
        locals.var_pd_dn7 = assign44930_e58189_d_n7;
        locals.var_pd_dn8 = assign44930_e58189_d_n8;
        locals.var_pd_dn9 = assign44930_e58189_d_n9;

        let (assign44940_e58197, assign44940_e58197_d_n4, assign44940_e58197_d_n6, assign44940_e58197_d_n7, assign44940_e58197_d_n8, assign44940_e58197_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1226 == 0.0)) {
        let assign44940_e58195: f64 = (locals.var_pd).sqrt();
        (assign44940_e58195, (locals.var_pd_dn4 / (2.0 * assign44940_e58195)), (locals.var_pd_dn6 / (2.0 * assign44940_e58195)), (locals.var_pd_dn7 / (2.0 * assign44940_e58195)), (locals.var_pd_dn8 / (2.0 * assign44940_e58195)), (locals.var_pd_dn9 / (2.0 * assign44940_e58195)),)
    } else {
        (locals.var_sqd, locals.var_sqd_dn4, locals.var_sqd_dn6, locals.var_sqd_dn7, locals.var_sqd_dn8, locals.var_sqd_dn9,)
    }
};
        locals.var_sqd = assign44940_e58197;
        locals.var_sqd_dn4 = assign44940_e58197_d_n4;
        locals.var_sqd_dn6 = assign44940_e58197_d_n6;
        locals.var_sqd_dn7 = assign44940_e58197_d_n7;
        locals.var_sqd_dn8 = assign44940_e58197_d_n8;
        locals.var_sqd_dn9 = assign44940_e58197_d_n9;

        let (assign44950_e58205, assign44950_e58205_d_n4, assign44950_e58205_d_n6, assign44950_e58205_d_n7, assign44950_e58205_d_n8, assign44950_e58205_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44950_e58201: f64 = (locals.var_sqd * locals.var_gf);
        let assign44950_e58203: f64 = (assign44950_e58201 * locals.var_phit1);
        (assign44950_e58203, ((((locals.var_sqd_dn4 * locals.var_gf) + (locals.var_sqd * locals.var_gf_dn4)) * locals.var_phit1) + (assign44950_e58201 * locals.var_phit1_dn4)), ((((locals.var_sqd_dn6 * locals.var_gf) + (locals.var_sqd * locals.var_gf_dn6)) * locals.var_phit1) + (assign44950_e58201 * locals.var_phit1_dn6)), ((((locals.var_sqd_dn7 * locals.var_gf) + (locals.var_sqd * locals.var_gf_dn7)) * locals.var_phit1) + (assign44950_e58201 * locals.var_phit1_dn7)), ((((locals.var_sqd_dn8 * locals.var_gf) + (locals.var_sqd * locals.var_gf_dn8)) * locals.var_phit1) + (assign44950_e58201 * locals.var_phit1_dn8)), ((((locals.var_sqd_dn9 * locals.var_gf) + (locals.var_sqd * locals.var_gf_dn9)) * locals.var_phit1) + (assign44950_e58201 * locals.var_phit1_dn9)),)
    } else {
        (locals.var_qbd, locals.var_qbd_dn4, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn8, locals.var_qbd_dn9,)
    }
};
        locals.var_qbd = assign44950_e58205;
        locals.var_qbd_dn4 = assign44950_e58205_d_n4;
        locals.var_qbd_dn6 = assign44950_e58205_d_n6;
        locals.var_qbd_dn7 = assign44950_e58205_d_n7;
        locals.var_qbd_dn8 = assign44950_e58205_d_n8;
        locals.var_qbd_dn9 = assign44950_e58205_d_n9;

        let (assign44960_e58213, assign44960_e58213_d_n4, assign44960_e58213_d_n6, assign44960_e58213_d_n7, assign44960_e58213_d_n8, assign44960_e58213_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44960_e58210: f64 = (locals.var_x_s + locals.var_x_d);
        let assign44960_e58211: f64 = (0.5 * assign44960_e58210);
        (assign44960_e58211, (0.5 * (locals.var_x_s_dn4 + locals.var_x_d_dn4)), (0.5 * (locals.var_x_s_dn6 + locals.var_x_d_dn6)), (0.5 * (locals.var_x_s_dn7 + locals.var_x_d_dn7)), (0.5 * (locals.var_x_s_dn8 + locals.var_x_d_dn8)), (0.5 * (locals.var_x_s_dn9 + locals.var_x_d_dn9)),)
    } else {
        (locals.var_x_m, locals.var_x_m_dn4, locals.var_x_m_dn6, locals.var_x_m_dn7, locals.var_x_m_dn8, locals.var_x_m_dn9,)
    }
};
        locals.var_x_m = assign44960_e58213;
        locals.var_x_m_dn4 = assign44960_e58213_d_n4;
        locals.var_x_m_dn6 = assign44960_e58213_d_n6;
        locals.var_x_m_dn7 = assign44960_e58213_d_n7;
        locals.var_x_m_dn8 = assign44960_e58213_d_n8;
        locals.var_x_m_dn9 = assign44960_e58213_d_n9;

        let (assign44970_e58217, assign44970_e58217_d_n4, assign44970_e58217_d_n6, assign44970_e58217_d_n7, assign44970_e58217_d_n8, assign44970_e58217_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_em, locals.var_em_dn4, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9,)
    }
};
        locals.var_em = assign44970_e58217;
        locals.var_em_dn4 = assign44970_e58217_d_n4;
        locals.var_em_dn6 = assign44970_e58217_d_n6;
        locals.var_em_dn7 = assign44970_e58217_d_n7;
        locals.var_em_dn8 = assign44970_e58217_d_n8;
        locals.var_em_dn9 = assign44970_e58217_d_n9;

        let (assign44980_e58223, assign44980_e58223_d_n4, assign44980_e58223_d_n6, assign44980_e58223_d_n7, assign44980_e58223_d_n8, assign44980_e58223_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign44980_e58221: f64 = (locals.var_ed * locals.var_es);
        (assign44980_e58221, ((locals.var_ed_dn4 * locals.var_es) + (locals.var_ed * locals.var_es_dn4)), ((locals.var_ed_dn6 * locals.var_es) + (locals.var_ed * locals.var_es_dn6)), ((locals.var_ed_dn7 * locals.var_es) + (locals.var_ed * locals.var_es_dn7)), ((locals.var_ed_dn8 * locals.var_es) + (locals.var_ed * locals.var_es_dn8)), ((locals.var_ed_dn9 * locals.var_es) + (locals.var_ed * locals.var_es_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign44980_e58223;
        locals.var_temp__blk949_dn4 = assign44980_e58223_d_n4;
        locals.var_temp__blk949_dn6 = assign44980_e58223_d_n6;
        locals.var_temp__blk949_dn7 = assign44980_e58223_d_n7;
        locals.var_temp__blk949_dn8 = assign44980_e58223_d_n8;
        locals.var_temp__blk949_dn9 = assign44980_e58223_d_n9;

    }

    pub(super) fn stamp_transient_block_26(
        locals: &mut StampLocals,
    ) {
        let assign44990_e58226: f64 = if locals.var_temp__blk949 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1229 = assign44990_e58226;

        let (assign45000_e58233, assign45000_e58233_d_n4, assign45000_e58233_d_n6, assign45000_e58233_d_n7, assign45000_e58233_d_n8, assign45000_e58233_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1229 != 0.0)) {
        let assign45000_e58231: f64 = (locals.var_temp__blk949).sqrt();
        (assign45000_e58231, (locals.var_temp__blk949_dn4 / (2.0 * assign45000_e58231)), (locals.var_temp__blk949_dn6 / (2.0 * assign45000_e58231)), (locals.var_temp__blk949_dn7 / (2.0 * assign45000_e58231)), (locals.var_temp__blk949_dn8 / (2.0 * assign45000_e58231)), (locals.var_temp__blk949_dn9 / (2.0 * assign45000_e58231)),)
    } else {
        (locals.var_em, locals.var_em_dn4, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9,)
    }
};
        locals.var_em = assign45000_e58233;
        locals.var_em_dn4 = assign45000_e58233_d_n4;
        locals.var_em_dn6 = assign45000_e58233_d_n6;
        locals.var_em_dn7 = assign45000_e58233_d_n7;
        locals.var_em_dn8 = assign45000_e58233_d_n8;
        locals.var_em_dn9 = assign45000_e58233_d_n9;

        let (assign45010_e58241, assign45010_e58241_d_n4, assign45010_e58241_d_n6, assign45010_e58241_d_n7, assign45010_e58241_d_n8, assign45010_e58241_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45010_e58238: f64 = (locals.var_ds + locals.var_dd);
        let assign45010_e58239: f64 = (0.5 * assign45010_e58238);
        (assign45010_e58239, (0.5 * (locals.var_ds_dn4 + locals.var_dd_dn4)), (0.5 * (locals.var_ds_dn6 + locals.var_dd_dn6)), (0.5 * (locals.var_ds_dn7 + locals.var_dd_dn7)), (0.5 * (locals.var_ds_dn8 + locals.var_dd_dn8)), (0.5 * (locals.var_ds_dn9 + locals.var_dd_dn9)),)
    } else {
        (locals.var_d_bar, locals.var_d_bar_dn4, locals.var_d_bar_dn6, locals.var_d_bar_dn7, locals.var_d_bar_dn8, locals.var_d_bar_dn9,)
    }
};
        locals.var_d_bar = assign45010_e58241;
        locals.var_d_bar_dn4 = assign45010_e58241_d_n4;
        locals.var_d_bar_dn6 = assign45010_e58241_d_n6;
        locals.var_d_bar_dn7 = assign45010_e58241_d_n7;
        locals.var_d_bar_dn8 = assign45010_e58241_d_n8;
        locals.var_d_bar_dn9 = assign45010_e58241_d_n9;

        let (assign45020_e58257, assign45020_e58257_d_n4, assign45020_e58257_d_n6, assign45020_e58257_d_n7, assign45020_e58257_d_n8, assign45020_e58257_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45020_e58247: f64 = (locals.var_x_ds * locals.var_x_ds);
        let assign45020_e58251: f64 = (2.0 * locals.var_inv_gf2);
        let assign45020_e58252: f64 = (locals.var_em - assign45020_e58251);
        let assign45020_e58253: f64 = (assign45020_e58247 * assign45020_e58252);
        let assign45020_e58254: f64 = (0.125 * assign45020_e58253);
        let assign45020_e58255: f64 = (locals.var_d_bar + assign45020_e58254);
        (assign45020_e58255, (locals.var_d_bar_dn4 + (0.125 * ((((locals.var_x_ds_dn4 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn4)) * assign45020_e58252) + (assign45020_e58247 * (locals.var_em_dn4 - (2.0 * locals.var_inv_gf2_dn4)))))), (locals.var_d_bar_dn6 + (0.125 * ((((locals.var_x_ds_dn6 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn6)) * assign45020_e58252) + (assign45020_e58247 * (locals.var_em_dn6 - (2.0 * locals.var_inv_gf2_dn6)))))), (locals.var_d_bar_dn7 + (0.125 * ((((locals.var_x_ds_dn7 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn7)) * assign45020_e58252) + (assign45020_e58247 * (locals.var_em_dn7 - (2.0 * locals.var_inv_gf2_dn7)))))), (locals.var_d_bar_dn8 + (0.125 * ((((locals.var_x_ds_dn8 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn8)) * assign45020_e58252) + (assign45020_e58247 * (locals.var_em_dn8 - (2.0 * locals.var_inv_gf2_dn8)))))), (locals.var_d_bar_dn9 + (0.125 * ((((locals.var_x_ds_dn9 * locals.var_x_ds) + (locals.var_x_ds * locals.var_x_ds_dn9)) * assign45020_e58252) + (assign45020_e58247 * (locals.var_em_dn9 - (2.0 * locals.var_inv_gf2_dn9)))))),)
    } else {
        (locals.var_dm, locals.var_dm_dn4, locals.var_dm_dn6, locals.var_dm_dn7, locals.var_dm_dn8, locals.var_dm_dn9,)
    }
};
        locals.var_dm = assign45020_e58257;
        locals.var_dm_dn4 = assign45020_e58257_d_n4;
        locals.var_dm_dn6 = assign45020_e58257_d_n6;
        locals.var_dm_dn7 = assign45020_e58257_d_n7;
        locals.var_dm_dn8 = assign45020_e58257_d_n8;
        locals.var_dm_dn9 = assign45020_e58257_d_n9;

        let assign45030_e58260: f64 = if locals.var_x_m < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1230 = assign45030_e58260;

        let (assign45040_e58282, assign45040_e58282_d_n4, assign45040_e58282_d_n6, assign45040_e58282_d_n7, assign45040_e58282_d_n8, assign45040_e58282_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign45040_e58267: f64 = (locals.var_x_m * locals.var_x_m);
        let assign45040_e58274: f64 = (0.25 * locals.var_x_m);
        let assign45040_e58275: f64 = (1.0 - assign45040_e58274);
        let assign45040_e58276: f64 = (locals.var_x_m * assign45040_e58275);
        let assign45040_e58277: f64 = (0.3333333333333333 * assign45040_e58276);
        let assign45040_e58278: f64 = (1.0 - assign45040_e58277);
        let assign45040_e58279: f64 = (assign45040_e58267 * assign45040_e58278);
        let assign45040_e58280: f64 = (0.5 * assign45040_e58279);
        (assign45040_e58280, (0.5 * ((((locals.var_x_m_dn4 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn4)) * assign45040_e58278) + (assign45040_e58267 * (-(0.3333333333333333 * ((locals.var_x_m_dn4 * assign45040_e58275) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn4))))))))), (0.5 * ((((locals.var_x_m_dn6 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn6)) * assign45040_e58278) + (assign45040_e58267 * (-(0.3333333333333333 * ((locals.var_x_m_dn6 * assign45040_e58275) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn6))))))))), (0.5 * ((((locals.var_x_m_dn7 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn7)) * assign45040_e58278) + (assign45040_e58267 * (-(0.3333333333333333 * ((locals.var_x_m_dn7 * assign45040_e58275) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn7))))))))), (0.5 * ((((locals.var_x_m_dn8 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn8)) * assign45040_e58278) + (assign45040_e58267 * (-(0.3333333333333333 * ((locals.var_x_m_dn8 * assign45040_e58275) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn8))))))))), (0.5 * ((((locals.var_x_m_dn9 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn9)) * assign45040_e58278) + (assign45040_e58267 * (-(0.3333333333333333 * ((locals.var_x_m_dn9 * assign45040_e58275) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn9))))))))),)
    } else {
        (locals.var_pm, locals.var_pm_dn4, locals.var_pm_dn6, locals.var_pm_dn7, locals.var_pm_dn8, locals.var_pm_dn9,)
    }
};
        locals.var_pm = assign45040_e58282;
        locals.var_pm_dn4 = assign45040_e58282_d_n4;
        locals.var_pm_dn6 = assign45040_e58282_d_n6;
        locals.var_pm_dn7 = assign45040_e58282_d_n7;
        locals.var_pm_dn8 = assign45040_e58282_d_n8;
        locals.var_pm_dn9 = assign45040_e58282_d_n9;

        let (assign45050_e58293, assign45050_e58293_d_n4, assign45050_e58293_d_n6, assign45050_e58293_d_n7, assign45050_e58293_d_n8, assign45050_e58293_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign45050_e58289: f64 = (locals.var_dm + locals.var_pm);
        let assign45050_e58290: f64 = (assign45050_e58289).sqrt();
        let assign45050_e58291: f64 = (locals.var_gf * assign45050_e58290);
        (assign45050_e58291, ((locals.var_gf_dn4 * assign45050_e58290) + (locals.var_gf * ((locals.var_dm_dn4 + locals.var_pm_dn4) / (2.0 * assign45050_e58290)))), ((locals.var_gf_dn6 * assign45050_e58290) + (locals.var_gf * ((locals.var_dm_dn6 + locals.var_pm_dn6) / (2.0 * assign45050_e58290)))), ((locals.var_gf_dn7 * assign45050_e58290) + (locals.var_gf * ((locals.var_dm_dn7 + locals.var_pm_dn7) / (2.0 * assign45050_e58290)))), ((locals.var_gf_dn8 * assign45050_e58290) + (locals.var_gf * ((locals.var_dm_dn8 + locals.var_pm_dn8) / (2.0 * assign45050_e58290)))), ((locals.var_gf_dn9 * assign45050_e58290) + (locals.var_gf * ((locals.var_dm_dn9 + locals.var_pm_dn9) / (2.0 * assign45050_e58290)))),)
    } else {
        (locals.var_xgm, locals.var_xgm_dn4, locals.var_xgm_dn6, locals.var_xgm_dn7, locals.var_xgm_dn8, locals.var_xgm_dn9,)
    }
};
        locals.var_xgm = assign45050_e58293;
        locals.var_xgm_dn4 = assign45050_e58293_d_n4;
        locals.var_xgm_dn6 = assign45050_e58293_d_n6;
        locals.var_xgm_dn7 = assign45050_e58293_d_n7;
        locals.var_xgm_dn8 = assign45050_e58293_d_n8;
        locals.var_xgm_dn9 = assign45050_e58293_d_n9;

        let assign45060_e58296: f64 = if locals.var_kp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1231 = assign45060_e58296;

        let (assign45070_e58311, assign45070_e58311_d_n4, assign45070_e58311_d_n6, assign45070_e58311_d_n7, assign45070_e58311_d_n8, assign45070_e58311_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 != 0.0)) && (locals.var_guard1231 != 0.0)) {
        let assign45070_e58306: f64 = (locals.var_kp * locals.var_xgm);
        let assign45070_e58307: f64 = (1.0 + assign45070_e58306);
        let assign45070_e58308: f64 = (assign45070_e58307).sqrt();
        let assign45070_e58309: f64 = (1.0 / assign45070_e58308);
        (assign45070_e58309, (-((((locals.var_kp_dn4 * locals.var_xgm) + (locals.var_kp * locals.var_xgm_dn4)) / (2.0 * assign45070_e58308)) / (assign45070_e58308 * assign45070_e58308))), (-(((locals.var_kp * locals.var_xgm_dn6) / (2.0 * assign45070_e58308)) / (assign45070_e58308 * assign45070_e58308))), (-(((locals.var_kp * locals.var_xgm_dn7) / (2.0 * assign45070_e58308)) / (assign45070_e58308 * assign45070_e58308))), (-(((locals.var_kp * locals.var_xgm_dn8) / (2.0 * assign45070_e58308)) / (assign45070_e58308 * assign45070_e58308))), (-(((locals.var_kp * locals.var_xgm_dn9) / (2.0 * assign45070_e58308)) / (assign45070_e58308 * assign45070_e58308))),)
    } else {
        (locals.var_eta_p, locals.var_eta_p_dn4, locals.var_eta_p_dn6, locals.var_eta_p_dn7, locals.var_eta_p_dn8, locals.var_eta_p_dn9,)
    }
};
        locals.var_eta_p = assign45070_e58311;
        locals.var_eta_p_dn4 = assign45070_e58311_d_n4;
        locals.var_eta_p_dn6 = assign45070_e58311_d_n6;
        locals.var_eta_p_dn7 = assign45070_e58311_d_n7;
        locals.var_eta_p_dn8 = assign45070_e58311_d_n8;
        locals.var_eta_p_dn9 = assign45070_e58311_d_n9;

        let (assign45080_e58328, assign45080_e58328_d_n4, assign45080_e58328_d_n6, assign45080_e58328_d_n7, assign45080_e58328_d_n8, assign45080_e58328_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign45080_e58321: f64 = (0.25 * locals.var_x_m);
        let assign45080_e58322: f64 = (1.0 - assign45080_e58321);
        let assign45080_e58323: f64 = (locals.var_x_m * assign45080_e58322);
        let assign45080_e58324: f64 = (0.3333333333333333 * assign45080_e58323);
        let assign45080_e58325: f64 = (1.0 - assign45080_e58324);
        let assign45080_e58326: f64 = (assign45080_e58325).sqrt();
        (assign45080_e58326, ((-(0.3333333333333333 * ((locals.var_x_m_dn4 * assign45080_e58322) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn4)))))) / (2.0 * assign45080_e58326)), ((-(0.3333333333333333 * ((locals.var_x_m_dn6 * assign45080_e58322) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn6)))))) / (2.0 * assign45080_e58326)), ((-(0.3333333333333333 * ((locals.var_x_m_dn7 * assign45080_e58322) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn7)))))) / (2.0 * assign45080_e58326)), ((-(0.3333333333333333 * ((locals.var_x_m_dn8 * assign45080_e58322) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn8)))))) / (2.0 * assign45080_e58326)), ((-(0.3333333333333333 * ((locals.var_x_m_dn9 * assign45080_e58322) + (locals.var_x_m * (-(0.25 * locals.var_x_m_dn9)))))) / (2.0 * assign45080_e58326)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign45080_e58328;
        locals.var_temp__blk949_dn4 = assign45080_e58328_d_n4;
        locals.var_temp__blk949_dn6 = assign45080_e58328_d_n6;
        locals.var_temp__blk949_dn7 = assign45080_e58328_d_n7;
        locals.var_temp__blk949_dn8 = assign45080_e58328_d_n8;
        locals.var_temp__blk949_dn9 = assign45080_e58328_d_n9;

        let (assign45090_e58338, assign45090_e58338_d_n4, assign45090_e58338_d_n6, assign45090_e58338_d_n7, assign45090_e58338_d_n8, assign45090_e58338_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign45090_e58335: f64 = (locals.var_x_m * locals.var_temp__blk949);
        let assign45090_e58336: f64 = (0.7071067811865475 * assign45090_e58335);
        (assign45090_e58336, (0.7071067811865475 * ((locals.var_x_m_dn4 * locals.var_temp__blk949) + (locals.var_x_m * locals.var_temp__blk949_dn4))), (0.7071067811865475 * ((locals.var_x_m_dn6 * locals.var_temp__blk949) + (locals.var_x_m * locals.var_temp__blk949_dn6))), (0.7071067811865475 * ((locals.var_x_m_dn7 * locals.var_temp__blk949) + (locals.var_x_m * locals.var_temp__blk949_dn7))), (0.7071067811865475 * ((locals.var_x_m_dn8 * locals.var_temp__blk949) + (locals.var_x_m * locals.var_temp__blk949_dn8))), (0.7071067811865475 * ((locals.var_x_m_dn9 * locals.var_temp__blk949) + (locals.var_x_m * locals.var_temp__blk949_dn9))),)
    } else {
        (locals.var_sqm, locals.var_sqm_dn4, locals.var_sqm_dn6, locals.var_sqm_dn7, locals.var_sqm_dn8, locals.var_sqm_dn9,)
    }
};
        locals.var_sqm = assign45090_e58338;
        locals.var_sqm_dn4 = assign45090_e58338_d_n4;
        locals.var_sqm_dn6 = assign45090_e58338_d_n6;
        locals.var_sqm_dn7 = assign45090_e58338_d_n7;
        locals.var_sqm_dn8 = assign45090_e58338_d_n8;
        locals.var_sqm_dn9 = assign45090_e58338_d_n9;

        let (assign45100_e58362, assign45100_e58362_d_n4, assign45100_e58362_d_n6, assign45100_e58362_d_n7, assign45100_e58362_d_n8, assign45100_e58362_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign45100_e58348: f64 = (0.5 * locals.var_x_m);
        let assign45100_e58349: f64 = (1.0 - assign45100_e58348);
        let assign45100_e58353: f64 = (locals.var_x_m * locals.var_x_m);
        let assign45100_e58354: f64 = (0.16666666666666666 * assign45100_e58353);
        let assign45100_e58355: f64 = (assign45100_e58349 + assign45100_e58354);
        let assign45100_e58356: f64 = (locals.var_gf * assign45100_e58355);
        let assign45100_e58358: f64 = (assign45100_e58356 / locals.var_temp__blk949);
        let assign45100_e58359: f64 = (0.7071067811865475 * assign45100_e58358);
        let assign45100_e58360: f64 = (locals.var_eta_p + assign45100_e58359);
        (assign45100_e58360, (locals.var_eta_p_dn4 + (0.7071067811865475 * (((((locals.var_gf_dn4 * assign45100_e58355) + (locals.var_gf * ((-(0.5 * locals.var_x_m_dn4)) + (0.16666666666666666 * ((locals.var_x_m_dn4 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn4)))))) * locals.var_temp__blk949) - (assign45100_e58356 * locals.var_temp__blk949_dn4)) / (locals.var_temp__blk949 * locals.var_temp__blk949)))), (locals.var_eta_p_dn6 + (0.7071067811865475 * (((((locals.var_gf_dn6 * assign45100_e58355) + (locals.var_gf * ((-(0.5 * locals.var_x_m_dn6)) + (0.16666666666666666 * ((locals.var_x_m_dn6 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn6)))))) * locals.var_temp__blk949) - (assign45100_e58356 * locals.var_temp__blk949_dn6)) / (locals.var_temp__blk949 * locals.var_temp__blk949)))), (locals.var_eta_p_dn7 + (0.7071067811865475 * (((((locals.var_gf_dn7 * assign45100_e58355) + (locals.var_gf * ((-(0.5 * locals.var_x_m_dn7)) + (0.16666666666666666 * ((locals.var_x_m_dn7 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn7)))))) * locals.var_temp__blk949) - (assign45100_e58356 * locals.var_temp__blk949_dn7)) / (locals.var_temp__blk949 * locals.var_temp__blk949)))), (locals.var_eta_p_dn8 + (0.7071067811865475 * (((((locals.var_gf_dn8 * assign45100_e58355) + (locals.var_gf * ((-(0.5 * locals.var_x_m_dn8)) + (0.16666666666666666 * ((locals.var_x_m_dn8 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn8)))))) * locals.var_temp__blk949) - (assign45100_e58356 * locals.var_temp__blk949_dn8)) / (locals.var_temp__blk949 * locals.var_temp__blk949)))), (locals.var_eta_p_dn9 + (0.7071067811865475 * (((((locals.var_gf_dn9 * assign45100_e58355) + (locals.var_gf * ((-(0.5 * locals.var_x_m_dn9)) + (0.16666666666666666 * ((locals.var_x_m_dn9 * locals.var_x_m) + (locals.var_x_m * locals.var_x_m_dn9)))))) * locals.var_temp__blk949) - (assign45100_e58356 * locals.var_temp__blk949_dn9)) / (locals.var_temp__blk949 * locals.var_temp__blk949)))),)
    } else {
        (locals.var_alpha, locals.var_alpha_dn4, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn8, locals.var_alpha_dn9,)
    }
};
        locals.var_alpha = assign45100_e58362;
        locals.var_alpha_dn4 = assign45100_e58362_d_n4;
        locals.var_alpha_dn6 = assign45100_e58362_d_n6;
        locals.var_alpha_dn7 = assign45100_e58362_d_n7;
        locals.var_alpha_dn8 = assign45100_e58362_d_n8;
        locals.var_alpha_dn9 = assign45100_e58362_d_n9;

        let (assign45110_e58373, assign45110_e58373_d_n4, assign45110_e58373_d_n6, assign45110_e58373_d_n7, assign45110_e58373_d_n8, assign45110_e58373_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) {
        let assign45110_e58369: f64 = (locals.var_x_m - 1.0);
        let assign45110_e58371: f64 = (assign45110_e58369 + locals.var_em);
        (assign45110_e58371, (locals.var_x_m_dn4 + locals.var_em_dn4), (locals.var_x_m_dn6 + locals.var_em_dn6), (locals.var_x_m_dn7 + locals.var_em_dn7), (locals.var_x_m_dn8 + locals.var_em_dn8), (locals.var_x_m_dn9 + locals.var_em_dn9),)
    } else {
        (locals.var_pm, locals.var_pm_dn4, locals.var_pm_dn6, locals.var_pm_dn7, locals.var_pm_dn8, locals.var_pm_dn9,)
    }
};
        locals.var_pm = assign45110_e58373;
        locals.var_pm_dn4 = assign45110_e58373_d_n4;
        locals.var_pm_dn6 = assign45110_e58373_d_n6;
        locals.var_pm_dn7 = assign45110_e58373_d_n7;
        locals.var_pm_dn8 = assign45110_e58373_d_n8;
        locals.var_pm_dn9 = assign45110_e58373_d_n9;

        let (assign45120_e58385, assign45120_e58385_d_n4, assign45120_e58385_d_n6, assign45120_e58385_d_n7, assign45120_e58385_d_n8, assign45120_e58385_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) {
        let assign45120_e58381: f64 = (locals.var_dm + locals.var_pm);
        let assign45120_e58382: f64 = (assign45120_e58381).sqrt();
        let assign45120_e58383: f64 = (locals.var_gf * assign45120_e58382);
        (assign45120_e58383, ((locals.var_gf_dn4 * assign45120_e58382) + (locals.var_gf * ((locals.var_dm_dn4 + locals.var_pm_dn4) / (2.0 * assign45120_e58382)))), ((locals.var_gf_dn6 * assign45120_e58382) + (locals.var_gf * ((locals.var_dm_dn6 + locals.var_pm_dn6) / (2.0 * assign45120_e58382)))), ((locals.var_gf_dn7 * assign45120_e58382) + (locals.var_gf * ((locals.var_dm_dn7 + locals.var_pm_dn7) / (2.0 * assign45120_e58382)))), ((locals.var_gf_dn8 * assign45120_e58382) + (locals.var_gf * ((locals.var_dm_dn8 + locals.var_pm_dn8) / (2.0 * assign45120_e58382)))), ((locals.var_gf_dn9 * assign45120_e58382) + (locals.var_gf * ((locals.var_dm_dn9 + locals.var_pm_dn9) / (2.0 * assign45120_e58382)))),)
    } else {
        (locals.var_xgm, locals.var_xgm_dn4, locals.var_xgm_dn6, locals.var_xgm_dn7, locals.var_xgm_dn8, locals.var_xgm_dn9,)
    }
};
        locals.var_xgm = assign45120_e58385;
        locals.var_xgm_dn4 = assign45120_e58385_d_n4;
        locals.var_xgm_dn6 = assign45120_e58385_d_n6;
        locals.var_xgm_dn7 = assign45120_e58385_d_n7;
        locals.var_xgm_dn8 = assign45120_e58385_d_n8;
        locals.var_xgm_dn9 = assign45120_e58385_d_n9;

        let assign45130_e58388: f64 = if locals.var_kp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1232 = assign45130_e58388;

        let (assign45140_e58405, assign45140_e58405_d_n4, assign45140_e58405_d_n6, assign45140_e58405_d_n7, assign45140_e58405_d_n8, assign45140_e58405_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45140_e58397: f64 = (1.0 - locals.var_em);
        let assign45140_e58401: f64 = (locals.var_xgm * locals.var_inv_gf2);
        let assign45140_e58402: f64 = (2.0 * assign45140_e58401);
        let assign45140_e58403: f64 = (assign45140_e58397 + assign45140_e58402);
        (assign45140_e58403, ((-locals.var_em_dn4) + (2.0 * ((locals.var_xgm_dn4 * locals.var_inv_gf2) + (locals.var_xgm * locals.var_inv_gf2_dn4)))), ((-locals.var_em_dn6) + (2.0 * ((locals.var_xgm_dn6 * locals.var_inv_gf2) + (locals.var_xgm * locals.var_inv_gf2_dn6)))), ((-locals.var_em_dn7) + (2.0 * ((locals.var_xgm_dn7 * locals.var_inv_gf2) + (locals.var_xgm * locals.var_inv_gf2_dn7)))), ((-locals.var_em_dn8) + (2.0 * ((locals.var_xgm_dn8 * locals.var_inv_gf2) + (locals.var_xgm * locals.var_inv_gf2_dn8)))), ((-locals.var_em_dn9) + (2.0 * ((locals.var_xgm_dn9 * locals.var_inv_gf2) + (locals.var_xgm * locals.var_inv_gf2_dn9)))),)
    } else {
        (locals.var_d0, locals.var_d0_dn4, locals.var_d0_dn6, locals.var_d0_dn7, locals.var_d0_dn8, locals.var_d0_dn9,)
    }
};
        locals.var_d0 = assign45140_e58405;
        locals.var_d0_dn4 = assign45140_e58405_d_n4;
        locals.var_d0_dn6 = assign45140_e58405_d_n6;
        locals.var_d0_dn7 = assign45140_e58405_d_n7;
        locals.var_d0_dn8 = assign45140_e58405_d_n8;
        locals.var_d0_dn9 = assign45140_e58405_d_n9;

        let (assign45150_e58421, assign45150_e58421_d_n4, assign45150_e58421_d_n6, assign45150_e58421_d_n7, assign45150_e58421_d_n8, assign45150_e58421_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45150_e58416: f64 = (locals.var_kp * locals.var_xgm);
        let assign45150_e58417: f64 = (1.0 + assign45150_e58416);
        let assign45150_e58418: f64 = (assign45150_e58417).sqrt();
        let assign45150_e58419: f64 = (1.0 / assign45150_e58418);
        (assign45150_e58419, (-((((locals.var_kp_dn4 * locals.var_xgm) + (locals.var_kp * locals.var_xgm_dn4)) / (2.0 * assign45150_e58418)) / (assign45150_e58418 * assign45150_e58418))), (-(((locals.var_kp * locals.var_xgm_dn6) / (2.0 * assign45150_e58418)) / (assign45150_e58418 * assign45150_e58418))), (-(((locals.var_kp * locals.var_xgm_dn7) / (2.0 * assign45150_e58418)) / (assign45150_e58418 * assign45150_e58418))), (-(((locals.var_kp * locals.var_xgm_dn8) / (2.0 * assign45150_e58418)) / (assign45150_e58418 * assign45150_e58418))), (-(((locals.var_kp * locals.var_xgm_dn9) / (2.0 * assign45150_e58418)) / (assign45150_e58418 * assign45150_e58418))),)
    } else {
        (locals.var_eta_p, locals.var_eta_p_dn4, locals.var_eta_p_dn6, locals.var_eta_p_dn7, locals.var_eta_p_dn8, locals.var_eta_p_dn9,)
    }
};
        locals.var_eta_p = assign45150_e58421;
        locals.var_eta_p_dn4 = assign45150_e58421_d_n4;
        locals.var_eta_p_dn6 = assign45150_e58421_d_n6;
        locals.var_eta_p_dn7 = assign45150_e58421_d_n7;
        locals.var_eta_p_dn8 = assign45150_e58421_d_n8;
        locals.var_eta_p_dn9 = assign45150_e58421_d_n9;

        let (assign45160_e58434, assign45160_e58434_d_n4, assign45160_e58434_d_n6, assign45160_e58434_d_n7, assign45160_e58434_d_n8, assign45160_e58434_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45160_e58431: f64 = (locals.var_eta_p + 1.0);
        let assign45160_e58432: f64 = (locals.var_eta_p / assign45160_e58431);
        (assign45160_e58432, (((locals.var_eta_p_dn4 * assign45160_e58431) - (locals.var_eta_p * locals.var_eta_p_dn4)) / (assign45160_e58431 * assign45160_e58431)), (((locals.var_eta_p_dn6 * assign45160_e58431) - (locals.var_eta_p * locals.var_eta_p_dn6)) / (assign45160_e58431 * assign45160_e58431)), (((locals.var_eta_p_dn7 * assign45160_e58431) - (locals.var_eta_p * locals.var_eta_p_dn7)) / (assign45160_e58431 * assign45160_e58431)), (((locals.var_eta_p_dn8 * assign45160_e58431) - (locals.var_eta_p * locals.var_eta_p_dn8)) / (assign45160_e58431 * assign45160_e58431)), (((locals.var_eta_p_dn9 * assign45160_e58431) - (locals.var_eta_p * locals.var_eta_p_dn9)) / (assign45160_e58431 * assign45160_e58431)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign45160_e58434;
        locals.var_temp__blk949_dn4 = assign45160_e58434_d_n4;
        locals.var_temp__blk949_dn6 = assign45160_e58434_d_n6;
        locals.var_temp__blk949_dn7 = assign45160_e58434_d_n7;
        locals.var_temp__blk949_dn8 = assign45160_e58434_d_n8;
        locals.var_temp__blk949_dn9 = assign45160_e58434_d_n9;

        let (assign45170_e58451, assign45170_e58451_d_n4, assign45170_e58451_d_n6, assign45170_e58451_d_n7, assign45170_e58451_d_n8, assign45170_e58451_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45170_e58444: f64 = (locals.var_temp__blk949 * locals.var_temp__blk949);
        let assign45170_e58446: f64 = (assign45170_e58444 * locals.var_gf2);
        let assign45170_e58448: f64 = (assign45170_e58446 * locals.var_dm);
        let assign45170_e58449: f64 = (locals.var_kp * assign45170_e58448);
        (assign45170_e58449, ((locals.var_kp_dn4 * assign45170_e58448) + (locals.var_kp * ((((((locals.var_temp__blk949_dn4 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn4)) * locals.var_gf2) + (assign45170_e58444 * locals.var_gf2_dn4)) * locals.var_dm) + (assign45170_e58446 * locals.var_dm_dn4)))), (locals.var_kp * ((((((locals.var_temp__blk949_dn6 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn6)) * locals.var_gf2) + (assign45170_e58444 * locals.var_gf2_dn6)) * locals.var_dm) + (assign45170_e58446 * locals.var_dm_dn6))), (locals.var_kp * ((((((locals.var_temp__blk949_dn7 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn7)) * locals.var_gf2) + (assign45170_e58444 * locals.var_gf2_dn7)) * locals.var_dm) + (assign45170_e58446 * locals.var_dm_dn7))), (locals.var_kp * ((((((locals.var_temp__blk949_dn8 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn8)) * locals.var_gf2) + (assign45170_e58444 * locals.var_gf2_dn8)) * locals.var_dm) + (assign45170_e58446 * locals.var_dm_dn8))), (locals.var_kp * ((((((locals.var_temp__blk949_dn9 * locals.var_temp__blk949) + (locals.var_temp__blk949 * locals.var_temp__blk949_dn9)) * locals.var_gf2) + (assign45170_e58444 * locals.var_gf2_dn9)) * locals.var_dm) + (assign45170_e58446 * locals.var_dm_dn9))),)
    } else {
        (locals.var_x_pm, locals.var_x_pm_dn4, locals.var_x_pm_dn6, locals.var_x_pm_dn7, locals.var_x_pm_dn8, locals.var_x_pm_dn9,)
    }
};
        locals.var_x_pm = assign45170_e58451;
        locals.var_x_pm_dn4 = assign45170_e58451_d_n4;
        locals.var_x_pm_dn6 = assign45170_e58451_d_n6;
        locals.var_x_pm_dn7 = assign45170_e58451_d_n7;
        locals.var_x_pm_dn8 = assign45170_e58451_d_n8;
        locals.var_x_pm_dn9 = assign45170_e58451_d_n9;

        let (assign45180_e58472, assign45180_e58472_d_n4, assign45180_e58472_d_n6, assign45180_e58472_d_n7, assign45180_e58472_d_n8, assign45180_e58472_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45180_e58461: f64 = (locals.var_xgm - locals.var_x_pm);
        let assign45180_e58462: f64 = (2.0 * assign45180_e58461);
        let assign45180_e58466: f64 = (1.0 - locals.var_em);
        let assign45180_e58468: f64 = (assign45180_e58466 + locals.var_dm);
        let assign45180_e58469: f64 = (locals.var_gf2 * assign45180_e58468);
        let assign45180_e58470: f64 = (assign45180_e58462 + assign45180_e58469);
        (assign45180_e58470, ((2.0 * (locals.var_xgm_dn4 - locals.var_x_pm_dn4)) + ((locals.var_gf2_dn4 * assign45180_e58468) + (locals.var_gf2 * ((-locals.var_em_dn4) + locals.var_dm_dn4)))), ((2.0 * (locals.var_xgm_dn6 - locals.var_x_pm_dn6)) + ((locals.var_gf2_dn6 * assign45180_e58468) + (locals.var_gf2 * ((-locals.var_em_dn6) + locals.var_dm_dn6)))), ((2.0 * (locals.var_xgm_dn7 - locals.var_x_pm_dn7)) + ((locals.var_gf2_dn7 * assign45180_e58468) + (locals.var_gf2 * ((-locals.var_em_dn7) + locals.var_dm_dn7)))), ((2.0 * (locals.var_xgm_dn8 - locals.var_x_pm_dn8)) + ((locals.var_gf2_dn8 * assign45180_e58468) + (locals.var_gf2 * ((-locals.var_em_dn8) + locals.var_dm_dn8)))), ((2.0 * (locals.var_xgm_dn9 - locals.var_x_pm_dn9)) + ((locals.var_gf2_dn9 * assign45180_e58468) + (locals.var_gf2 * ((-locals.var_em_dn9) + locals.var_dm_dn9)))),)
    } else {
        (locals.var_p_pd, locals.var_p_pd_dn4, locals.var_p_pd_dn6, locals.var_p_pd_dn7, locals.var_p_pd_dn8, locals.var_p_pd_dn9,)
    }
};
        locals.var_p_pd = assign45180_e58472;
        locals.var_p_pd_dn4 = assign45180_e58472_d_n4;
        locals.var_p_pd_dn6 = assign45180_e58472_d_n6;
        locals.var_p_pd_dn7 = assign45180_e58472_d_n7;
        locals.var_p_pd_dn8 = assign45180_e58472_d_n8;
        locals.var_p_pd_dn9 = assign45180_e58472_d_n9;

        let (assign45190_e58487, assign45190_e58487_d_n4, assign45190_e58487_d_n6, assign45190_e58487_d_n7, assign45190_e58487_d_n8, assign45190_e58487_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45190_e58483: f64 = (2.0 * locals.var_xgm);
        let assign45190_e58484: f64 = (locals.var_x_pm - assign45190_e58483);
        let assign45190_e58485: f64 = (locals.var_x_pm * assign45190_e58484);
        (assign45190_e58485, ((locals.var_x_pm_dn4 * assign45190_e58484) + (locals.var_x_pm * (locals.var_x_pm_dn4 - (2.0 * locals.var_xgm_dn4)))), ((locals.var_x_pm_dn6 * assign45190_e58484) + (locals.var_x_pm * (locals.var_x_pm_dn6 - (2.0 * locals.var_xgm_dn6)))), ((locals.var_x_pm_dn7 * assign45190_e58484) + (locals.var_x_pm * (locals.var_x_pm_dn7 - (2.0 * locals.var_xgm_dn7)))), ((locals.var_x_pm_dn8 * assign45190_e58484) + (locals.var_x_pm * (locals.var_x_pm_dn8 - (2.0 * locals.var_xgm_dn8)))), ((locals.var_x_pm_dn9 * assign45190_e58484) + (locals.var_x_pm * (locals.var_x_pm_dn9 - (2.0 * locals.var_xgm_dn9)))),)
    } else {
        (locals.var_q_pd, locals.var_q_pd_dn4, locals.var_q_pd_dn6, locals.var_q_pd_dn7, locals.var_q_pd_dn8, locals.var_q_pd_dn9,)
    }
};
        locals.var_q_pd = assign45190_e58487;
        locals.var_q_pd_dn4 = assign45190_e58487_d_n4;
        locals.var_q_pd_dn6 = assign45190_e58487_d_n6;
        locals.var_q_pd_dn7 = assign45190_e58487_d_n7;
        locals.var_q_pd_dn8 = assign45190_e58487_d_n8;
        locals.var_q_pd_dn9 = assign45190_e58487_d_n9;

        let (assign45200_e58504, assign45200_e58504_d_n4, assign45200_e58504_d_n6, assign45200_e58504_d_n7, assign45200_e58504_d_n8, assign45200_e58504_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45200_e58499: f64 = (locals.var_em + locals.var_dm);
        let assign45200_e58500: f64 = (locals.var_gf2 * assign45200_e58499);
        let assign45200_e58501: f64 = (0.5 * assign45200_e58500);
        let assign45200_e58502: f64 = (1.0 - assign45200_e58501);
        (assign45200_e58502, (-(0.5 * ((locals.var_gf2_dn4 * assign45200_e58499) + (locals.var_gf2 * (locals.var_em_dn4 + locals.var_dm_dn4))))), (-(0.5 * ((locals.var_gf2_dn6 * assign45200_e58499) + (locals.var_gf2 * (locals.var_em_dn6 + locals.var_dm_dn6))))), (-(0.5 * ((locals.var_gf2_dn7 * assign45200_e58499) + (locals.var_gf2 * (locals.var_em_dn7 + locals.var_dm_dn7))))), (-(0.5 * ((locals.var_gf2_dn8 * assign45200_e58499) + (locals.var_gf2 * (locals.var_em_dn8 + locals.var_dm_dn8))))), (-(0.5 * ((locals.var_gf2_dn9 * assign45200_e58499) + (locals.var_gf2 * (locals.var_em_dn9 + locals.var_dm_dn9))))),)
    } else {
        (locals.var_xi_pd, locals.var_xi_pd_dn4, locals.var_xi_pd_dn6, locals.var_xi_pd_dn7, locals.var_xi_pd_dn8, locals.var_xi_pd_dn9,)
    }
};
        locals.var_xi_pd = assign45200_e58504;
        locals.var_xi_pd_dn4 = assign45200_e58504_d_n4;
        locals.var_xi_pd_dn6 = assign45200_e58504_d_n6;
        locals.var_xi_pd_dn7 = assign45200_e58504_d_n7;
        locals.var_xi_pd_dn8 = assign45200_e58504_d_n8;
        locals.var_xi_pd_dn9 = assign45200_e58504_d_n9;

        let (assign45210_e58523, assign45210_e58523_d_n4, assign45210_e58523_d_n6, assign45210_e58523_d_n7, assign45210_e58523_d_n8, assign45210_e58523_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45210_e58513: f64 = (locals.var_q_pd * locals.var_p_pd);
        let assign45210_e58516: f64 = (locals.var_p_pd * locals.var_p_pd);
        let assign45210_e58519: f64 = (locals.var_xi_pd * locals.var_q_pd);
        let assign45210_e58520: f64 = (assign45210_e58516 - assign45210_e58519);
        let assign45210_e58521: f64 = (assign45210_e58513 / assign45210_e58520);
        (assign45210_e58521, (((((locals.var_q_pd_dn4 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn4)) * assign45210_e58520) - (assign45210_e58513 * (((locals.var_p_pd_dn4 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn4)) - ((locals.var_xi_pd_dn4 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn4))))) / (assign45210_e58520 * assign45210_e58520)), (((((locals.var_q_pd_dn6 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn6)) * assign45210_e58520) - (assign45210_e58513 * (((locals.var_p_pd_dn6 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn6)) - ((locals.var_xi_pd_dn6 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn6))))) / (assign45210_e58520 * assign45210_e58520)), (((((locals.var_q_pd_dn7 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn7)) * assign45210_e58520) - (assign45210_e58513 * (((locals.var_p_pd_dn7 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn7)) - ((locals.var_xi_pd_dn7 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn7))))) / (assign45210_e58520 * assign45210_e58520)), (((((locals.var_q_pd_dn8 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn8)) * assign45210_e58520) - (assign45210_e58513 * (((locals.var_p_pd_dn8 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn8)) - ((locals.var_xi_pd_dn8 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn8))))) / (assign45210_e58520 * assign45210_e58520)), (((((locals.var_q_pd_dn9 * locals.var_p_pd) + (locals.var_q_pd * locals.var_p_pd_dn9)) * assign45210_e58520) - (assign45210_e58513 * (((locals.var_p_pd_dn9 * locals.var_p_pd) + (locals.var_p_pd * locals.var_p_pd_dn9)) - ((locals.var_xi_pd_dn9 * locals.var_q_pd) + (locals.var_xi_pd * locals.var_q_pd_dn9))))) / (assign45210_e58520 * assign45210_e58520)),)
    } else {
        (locals.var_u_pd, locals.var_u_pd_dn4, locals.var_u_pd_dn6, locals.var_u_pd_dn7, locals.var_u_pd_dn8, locals.var_u_pd_dn9,)
    }
};
        locals.var_u_pd = assign45210_e58523;
        locals.var_u_pd_dn4 = assign45210_e58523_d_n4;
        locals.var_u_pd_dn6 = assign45210_e58523_d_n6;
        locals.var_u_pd_dn7 = assign45210_e58523_d_n7;
        locals.var_u_pd_dn8 = assign45210_e58523_d_n8;
        locals.var_u_pd_dn9 = assign45210_e58523_d_n9;

        let (assign45220_e58534, assign45220_e58534_d_n4, assign45220_e58534_d_n6, assign45220_e58534_d_n7, assign45220_e58534_d_n8, assign45220_e58534_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45220_e58532: f64 = (locals.var_x_m + locals.var_u_pd);
        (assign45220_e58532, (locals.var_x_m_dn4 + locals.var_u_pd_dn4), (locals.var_x_m_dn6 + locals.var_u_pd_dn6), (locals.var_x_m_dn7 + locals.var_u_pd_dn7), (locals.var_x_m_dn8 + locals.var_u_pd_dn8), (locals.var_x_m_dn9 + locals.var_u_pd_dn9),)
    } else {
        (locals.var_x_m, locals.var_x_m_dn4, locals.var_x_m_dn6, locals.var_x_m_dn7, locals.var_x_m_dn8, locals.var_x_m_dn9,)
    }
};
        locals.var_x_m = assign45220_e58534;
        locals.var_x_m_dn4 = assign45220_e58534_d_n4;
        locals.var_x_m_dn6 = assign45220_e58534_d_n6;
        locals.var_x_m_dn7 = assign45220_e58534_d_n7;
        locals.var_x_m_dn8 = assign45220_e58534_d_n8;
        locals.var_x_m_dn9 = assign45220_e58534_d_n9;

        let (assign45230_e58544, assign45230_e58544_d_n4, assign45230_e58544_d_n6, assign45230_e58544_d_n7, assign45230_e58544_d_n8, assign45230_e58544_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45230_e58542: f64 = (locals.var_u_pd).exp();
        (assign45230_e58542, (assign45230_e58542 * locals.var_u_pd_dn4), (assign45230_e58542 * locals.var_u_pd_dn6), (assign45230_e58542 * locals.var_u_pd_dn7), (assign45230_e58542 * locals.var_u_pd_dn8), (assign45230_e58542 * locals.var_u_pd_dn9),)
    } else {
        (locals.var_km, locals.var_km_dn4, locals.var_km_dn6, locals.var_km_dn7, locals.var_km_dn8, locals.var_km_dn9,)
    }
};
        locals.var_km = assign45230_e58544;
        locals.var_km_dn4 = assign45230_e58544_d_n4;
        locals.var_km_dn6 = assign45230_e58544_d_n6;
        locals.var_km_dn7 = assign45230_e58544_d_n7;
        locals.var_km_dn8 = assign45230_e58544_d_n8;
        locals.var_km_dn9 = assign45230_e58544_d_n9;

        let (assign45240_e58555, assign45240_e58555_d_n4, assign45240_e58555_d_n6, assign45240_e58555_d_n7, assign45240_e58555_d_n8, assign45240_e58555_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45240_e58553: f64 = (locals.var_em / locals.var_km);
        (assign45240_e58553, (((locals.var_em_dn4 * locals.var_km) - (locals.var_em * locals.var_km_dn4)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn6 * locals.var_km) - (locals.var_em * locals.var_km_dn6)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn7 * locals.var_km) - (locals.var_em * locals.var_km_dn7)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn8 * locals.var_km) - (locals.var_em * locals.var_km_dn8)) / (locals.var_km * locals.var_km)), (((locals.var_em_dn9 * locals.var_km) - (locals.var_em * locals.var_km_dn9)) / (locals.var_km * locals.var_km)),)
    } else {
        (locals.var_em, locals.var_em_dn4, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9,)
    }
};
        locals.var_em = assign45240_e58555;
        locals.var_em_dn4 = assign45240_e58555_d_n4;
        locals.var_em_dn6 = assign45240_e58555_d_n6;
        locals.var_em_dn7 = assign45240_e58555_d_n7;
        locals.var_em_dn8 = assign45240_e58555_d_n8;
        locals.var_em_dn9 = assign45240_e58555_d_n9;

        let (assign45250_e58566, assign45250_e58566_d_n4, assign45250_e58566_d_n6, assign45250_e58566_d_n7, assign45250_e58566_d_n8, assign45250_e58566_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45250_e58564: f64 = (locals.var_dm * locals.var_km);
        (assign45250_e58564, ((locals.var_dm_dn4 * locals.var_km) + (locals.var_dm * locals.var_km_dn4)), ((locals.var_dm_dn6 * locals.var_km) + (locals.var_dm * locals.var_km_dn6)), ((locals.var_dm_dn7 * locals.var_km) + (locals.var_dm * locals.var_km_dn7)), ((locals.var_dm_dn8 * locals.var_km) + (locals.var_dm * locals.var_km_dn8)), ((locals.var_dm_dn9 * locals.var_km) + (locals.var_dm * locals.var_km_dn9)),)
    } else {
        (locals.var_dm, locals.var_dm_dn4, locals.var_dm_dn6, locals.var_dm_dn7, locals.var_dm_dn8, locals.var_dm_dn9,)
    }
};
        locals.var_dm = assign45250_e58566;
        locals.var_dm_dn4 = assign45250_e58566_d_n4;
        locals.var_dm_dn6 = assign45250_e58566_d_n6;
        locals.var_dm_dn7 = assign45250_e58566_d_n7;
        locals.var_dm_dn8 = assign45250_e58566_d_n8;
        locals.var_dm_dn9 = assign45250_e58566_d_n9;

        let (assign45260_e58579, assign45260_e58579_d_n4, assign45260_e58579_d_n6, assign45260_e58579_d_n7, assign45260_e58579_d_n8, assign45260_e58579_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45260_e58575: f64 = (locals.var_x_m - 1.0);
        let assign45260_e58577: f64 = (assign45260_e58575 + locals.var_em);
        (assign45260_e58577, (locals.var_x_m_dn4 + locals.var_em_dn4), (locals.var_x_m_dn6 + locals.var_em_dn6), (locals.var_x_m_dn7 + locals.var_em_dn7), (locals.var_x_m_dn8 + locals.var_em_dn8), (locals.var_x_m_dn9 + locals.var_em_dn9),)
    } else {
        (locals.var_pm, locals.var_pm_dn4, locals.var_pm_dn6, locals.var_pm_dn7, locals.var_pm_dn8, locals.var_pm_dn9,)
    }
};
        locals.var_pm = assign45260_e58579;
        locals.var_pm_dn4 = assign45260_e58579_d_n4;
        locals.var_pm_dn6 = assign45260_e58579_d_n6;
        locals.var_pm_dn7 = assign45260_e58579_d_n7;
        locals.var_pm_dn8 = assign45260_e58579_d_n8;
        locals.var_pm_dn9 = assign45260_e58579_d_n9;

        let (assign45270_e58593, assign45270_e58593_d_n4, assign45270_e58593_d_n6, assign45270_e58593_d_n7, assign45270_e58593_d_n8, assign45270_e58593_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45270_e58589: f64 = (locals.var_dm + locals.var_pm);
        let assign45270_e58590: f64 = (assign45270_e58589).sqrt();
        let assign45270_e58591: f64 = (locals.var_gf * assign45270_e58590);
        (assign45270_e58591, ((locals.var_gf_dn4 * assign45270_e58590) + (locals.var_gf * ((locals.var_dm_dn4 + locals.var_pm_dn4) / (2.0 * assign45270_e58590)))), ((locals.var_gf_dn6 * assign45270_e58590) + (locals.var_gf * ((locals.var_dm_dn6 + locals.var_pm_dn6) / (2.0 * assign45270_e58590)))), ((locals.var_gf_dn7 * assign45270_e58590) + (locals.var_gf * ((locals.var_dm_dn7 + locals.var_pm_dn7) / (2.0 * assign45270_e58590)))), ((locals.var_gf_dn8 * assign45270_e58590) + (locals.var_gf * ((locals.var_dm_dn8 + locals.var_pm_dn8) / (2.0 * assign45270_e58590)))), ((locals.var_gf_dn9 * assign45270_e58590) + (locals.var_gf * ((locals.var_dm_dn9 + locals.var_pm_dn9) / (2.0 * assign45270_e58590)))),)
    } else {
        (locals.var_xgm, locals.var_xgm_dn4, locals.var_xgm_dn6, locals.var_xgm_dn7, locals.var_xgm_dn8, locals.var_xgm_dn9,)
    }
};
        locals.var_xgm = assign45270_e58593;
        locals.var_xgm_dn4 = assign45270_e58593_d_n4;
        locals.var_xgm_dn6 = assign45270_e58593_d_n6;
        locals.var_xgm_dn7 = assign45270_e58593_d_n7;
        locals.var_xgm_dn8 = assign45270_e58593_d_n8;
        locals.var_xgm_dn9 = assign45270_e58593_d_n9;

        let (assign45280_e58612, assign45280_e58612_d_n4, assign45280_e58612_d_n6, assign45280_e58612_d_n7, assign45280_e58612_d_n8, assign45280_e58612_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45280_e58602: f64 = (1.0 - locals.var_em);
        let assign45280_e58606: f64 = (locals.var_xgm * locals.var_eta_p);
        let assign45280_e58608: f64 = (assign45280_e58606 * locals.var_inv_gf2);
        let assign45280_e58609: f64 = (2.0 * assign45280_e58608);
        let assign45280_e58610: f64 = (assign45280_e58602 + assign45280_e58609);
        (assign45280_e58610, ((-locals.var_em_dn4) + (2.0 * ((((locals.var_xgm_dn4 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn4)) * locals.var_inv_gf2) + (assign45280_e58606 * locals.var_inv_gf2_dn4)))), ((-locals.var_em_dn6) + (2.0 * ((((locals.var_xgm_dn6 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn6)) * locals.var_inv_gf2) + (assign45280_e58606 * locals.var_inv_gf2_dn6)))), ((-locals.var_em_dn7) + (2.0 * ((((locals.var_xgm_dn7 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn7)) * locals.var_inv_gf2) + (assign45280_e58606 * locals.var_inv_gf2_dn7)))), ((-locals.var_em_dn8) + (2.0 * ((((locals.var_xgm_dn8 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn8)) * locals.var_inv_gf2) + (assign45280_e58606 * locals.var_inv_gf2_dn8)))), ((-locals.var_em_dn9) + (2.0 * ((((locals.var_xgm_dn9 * locals.var_eta_p) + (locals.var_xgm * locals.var_eta_p_dn9)) * locals.var_inv_gf2) + (assign45280_e58606 * locals.var_inv_gf2_dn9)))),)
    } else {
        (locals.var_km0, locals.var_km0_dn4, locals.var_km0_dn6, locals.var_km0_dn7, locals.var_km0_dn8, locals.var_km0_dn9,)
    }
};
        locals.var_km0 = assign45280_e58612;
        locals.var_km0_dn4 = assign45280_e58612_d_n4;
        locals.var_km0_dn6 = assign45280_e58612_d_n6;
        locals.var_km0_dn7 = assign45280_e58612_d_n7;
        locals.var_km0_dn8 = assign45280_e58612_d_n8;
        locals.var_km0_dn9 = assign45280_e58612_d_n9;

        let (assign45290_e58633, assign45290_e58633_d_n4, assign45290_e58633_d_n6, assign45290_e58633_d_n7, assign45290_e58633_d_n8, assign45290_e58633_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45290_e58621: f64 = (locals.var_x_ds * locals.var_km);
        let assign45290_e58624: f64 = (locals.var_d0 + locals.var_d_bar);
        let assign45290_e58625: f64 = (assign45290_e58621 * assign45290_e58624);
        let assign45290_e58629: f64 = (locals.var_km * locals.var_d_bar);
        let assign45290_e58630: f64 = (locals.var_km0 + assign45290_e58629);
        let assign45290_e58631: f64 = (assign45290_e58625 / assign45290_e58630);
        (assign45290_e58631, (((((((locals.var_x_ds_dn4 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn4)) * assign45290_e58624) + (assign45290_e58621 * (locals.var_d0_dn4 + locals.var_d_bar_dn4))) * assign45290_e58630) - (assign45290_e58625 * (locals.var_km0_dn4 + ((locals.var_km_dn4 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn4))))) / (assign45290_e58630 * assign45290_e58630)), (((((((locals.var_x_ds_dn6 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn6)) * assign45290_e58624) + (assign45290_e58621 * (locals.var_d0_dn6 + locals.var_d_bar_dn6))) * assign45290_e58630) - (assign45290_e58625 * (locals.var_km0_dn6 + ((locals.var_km_dn6 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn6))))) / (assign45290_e58630 * assign45290_e58630)), (((((((locals.var_x_ds_dn7 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn7)) * assign45290_e58624) + (assign45290_e58621 * (locals.var_d0_dn7 + locals.var_d_bar_dn7))) * assign45290_e58630) - (assign45290_e58625 * (locals.var_km0_dn7 + ((locals.var_km_dn7 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn7))))) / (assign45290_e58630 * assign45290_e58630)), (((((((locals.var_x_ds_dn8 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn8)) * assign45290_e58624) + (assign45290_e58621 * (locals.var_d0_dn8 + locals.var_d_bar_dn8))) * assign45290_e58630) - (assign45290_e58625 * (locals.var_km0_dn8 + ((locals.var_km_dn8 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn8))))) / (assign45290_e58630 * assign45290_e58630)), (((((((locals.var_x_ds_dn9 * locals.var_km) + (locals.var_x_ds * locals.var_km_dn9)) * assign45290_e58624) + (assign45290_e58621 * (locals.var_d0_dn9 + locals.var_d_bar_dn9))) * assign45290_e58630) - (assign45290_e58625 * (locals.var_km0_dn9 + ((locals.var_km_dn9 * locals.var_d_bar) + (locals.var_km * locals.var_d_bar_dn9))))) / (assign45290_e58630 * assign45290_e58630)),)
    } else {
        (locals.var_x_ds, locals.var_x_ds_dn4, locals.var_x_ds_dn6, locals.var_x_ds_dn7, locals.var_x_ds_dn8, locals.var_x_ds_dn9,)
    }
};
        locals.var_x_ds = assign45290_e58633;
        locals.var_x_ds_dn4 = assign45290_e58633_d_n4;
        locals.var_x_ds_dn6 = assign45290_e58633_d_n6;
        locals.var_x_ds_dn7 = assign45290_e58633_d_n7;
        locals.var_x_ds_dn8 = assign45290_e58633_d_n8;
        locals.var_x_ds_dn9 = assign45290_e58633_d_n9;

        let (assign45300_e58644, assign45300_e58644_d_n4, assign45300_e58644_d_n6, assign45300_e58644_d_n7, assign45300_e58644_d_n8, assign45300_e58644_d_n9,) = {
    if (((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) && (locals.var_guard1232 != 0.0)) {
        let assign45300_e58642: f64 = (locals.var_x_ds * locals.var_phit1);
        (assign45300_e58642, ((locals.var_x_ds_dn4 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn4)), ((locals.var_x_ds_dn6 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn6)), ((locals.var_x_ds_dn7 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn7)), ((locals.var_x_ds_dn8 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn8)), ((locals.var_x_ds_dn9 * locals.var_phit1) + (locals.var_x_ds * locals.var_phit1_dn9)),)
    } else {
        (locals.var_dps, locals.var_dps_dn4, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8, locals.var_dps_dn9,)
    }
};
        locals.var_dps = assign45300_e58644;
        locals.var_dps_dn4 = assign45300_e58644_d_n4;
        locals.var_dps_dn6 = assign45300_e58644_d_n6;
        locals.var_dps_dn7 = assign45300_e58644_d_n7;
        locals.var_dps_dn8 = assign45300_e58644_d_n8;
        locals.var_dps_dn9 = assign45300_e58644_d_n9;

    }

    pub(super) fn stamp_transient_block_27(
        locals: &mut StampLocals,
    ) {
        let (assign45310_e58652, assign45310_e58652_d_n4, assign45310_e58652_d_n6, assign45310_e58652_d_n7, assign45310_e58652_d_n8, assign45310_e58652_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) {
        let assign45310_e58650: f64 = (locals.var_pm).sqrt();
        (assign45310_e58650, (locals.var_pm_dn4 / (2.0 * assign45310_e58650)), (locals.var_pm_dn6 / (2.0 * assign45310_e58650)), (locals.var_pm_dn7 / (2.0 * assign45310_e58650)), (locals.var_pm_dn8 / (2.0 * assign45310_e58650)), (locals.var_pm_dn9 / (2.0 * assign45310_e58650)),)
    } else {
        (locals.var_sqm, locals.var_sqm_dn4, locals.var_sqm_dn6, locals.var_sqm_dn7, locals.var_sqm_dn8, locals.var_sqm_dn9,)
    }
};
        locals.var_sqm = assign45310_e58652;
        locals.var_sqm_dn4 = assign45310_e58652_d_n4;
        locals.var_sqm_dn6 = assign45310_e58652_d_n6;
        locals.var_sqm_dn7 = assign45310_e58652_d_n7;
        locals.var_sqm_dn8 = assign45310_e58652_d_n8;
        locals.var_sqm_dn9 = assign45310_e58652_d_n9;

        let (assign45320_e58669, assign45320_e58669_d_n4, assign45320_e58669_d_n6, assign45320_e58669_d_n7, assign45320_e58669_d_n8, assign45320_e58669_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1230 == 0.0)) {
        let assign45320_e58662: f64 = (1.0 - locals.var_em);
        let assign45320_e58663: f64 = (locals.var_gf * assign45320_e58662);
        let assign45320_e58665: f64 = (assign45320_e58663 / locals.var_sqm);
        let assign45320_e58666: f64 = (0.5 * assign45320_e58665);
        let assign45320_e58667: f64 = (locals.var_eta_p + assign45320_e58666);
        (assign45320_e58667, (locals.var_eta_p_dn4 + (0.5 * (((((locals.var_gf_dn4 * assign45320_e58662) + (locals.var_gf * (-locals.var_em_dn4))) * locals.var_sqm) - (assign45320_e58663 * locals.var_sqm_dn4)) / (locals.var_sqm * locals.var_sqm)))), (locals.var_eta_p_dn6 + (0.5 * (((((locals.var_gf_dn6 * assign45320_e58662) + (locals.var_gf * (-locals.var_em_dn6))) * locals.var_sqm) - (assign45320_e58663 * locals.var_sqm_dn6)) / (locals.var_sqm * locals.var_sqm)))), (locals.var_eta_p_dn7 + (0.5 * (((((locals.var_gf_dn7 * assign45320_e58662) + (locals.var_gf * (-locals.var_em_dn7))) * locals.var_sqm) - (assign45320_e58663 * locals.var_sqm_dn7)) / (locals.var_sqm * locals.var_sqm)))), (locals.var_eta_p_dn8 + (0.5 * (((((locals.var_gf_dn8 * assign45320_e58662) + (locals.var_gf * (-locals.var_em_dn8))) * locals.var_sqm) - (assign45320_e58663 * locals.var_sqm_dn8)) / (locals.var_sqm * locals.var_sqm)))), (locals.var_eta_p_dn9 + (0.5 * (((((locals.var_gf_dn9 * assign45320_e58662) + (locals.var_gf * (-locals.var_em_dn9))) * locals.var_sqm) - (assign45320_e58663 * locals.var_sqm_dn9)) / (locals.var_sqm * locals.var_sqm)))),)
    } else {
        (locals.var_alpha, locals.var_alpha_dn4, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn8, locals.var_alpha_dn9,)
    }
};
        locals.var_alpha = assign45320_e58669;
        locals.var_alpha_dn4 = assign45320_e58669_d_n4;
        locals.var_alpha_dn6 = assign45320_e58669_d_n6;
        locals.var_alpha_dn7 = assign45320_e58669_d_n7;
        locals.var_alpha_dn8 = assign45320_e58669_d_n8;
        locals.var_alpha_dn9 = assign45320_e58669_d_n9;

        let (assign45330_e58683, assign45330_e58683_d_n4, assign45330_e58683_d_n6, assign45330_e58683_d_n7, assign45330_e58683_d_n8, assign45330_e58683_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45330_e58674: f64 = (locals.var_gf2 * locals.var_dm);
        let assign45330_e58678: f64 = (locals.var_gf * locals.var_sqm);
        let assign45330_e58679: f64 = (locals.var_xgm + assign45330_e58678);
        let assign45330_e58680: f64 = (assign45330_e58674 / assign45330_e58679);
        let assign45330_e58681: f64 = (locals.var_phit1 * assign45330_e58680);
        (assign45330_e58681, ((locals.var_phit1_dn4 * assign45330_e58680) + (locals.var_phit1 * (((((locals.var_gf2_dn4 * locals.var_dm) + (locals.var_gf2 * locals.var_dm_dn4)) * assign45330_e58679) - (assign45330_e58674 * (locals.var_xgm_dn4 + ((locals.var_gf_dn4 * locals.var_sqm) + (locals.var_gf * locals.var_sqm_dn4))))) / (assign45330_e58679 * assign45330_e58679)))), ((locals.var_phit1_dn6 * assign45330_e58680) + (locals.var_phit1 * (((((locals.var_gf2_dn6 * locals.var_dm) + (locals.var_gf2 * locals.var_dm_dn6)) * assign45330_e58679) - (assign45330_e58674 * (locals.var_xgm_dn6 + ((locals.var_gf_dn6 * locals.var_sqm) + (locals.var_gf * locals.var_sqm_dn6))))) / (assign45330_e58679 * assign45330_e58679)))), ((locals.var_phit1_dn7 * assign45330_e58680) + (locals.var_phit1 * (((((locals.var_gf2_dn7 * locals.var_dm) + (locals.var_gf2 * locals.var_dm_dn7)) * assign45330_e58679) - (assign45330_e58674 * (locals.var_xgm_dn7 + ((locals.var_gf_dn7 * locals.var_sqm) + (locals.var_gf * locals.var_sqm_dn7))))) / (assign45330_e58679 * assign45330_e58679)))), ((locals.var_phit1_dn8 * assign45330_e58680) + (locals.var_phit1 * (((((locals.var_gf2_dn8 * locals.var_dm) + (locals.var_gf2 * locals.var_dm_dn8)) * assign45330_e58679) - (assign45330_e58674 * (locals.var_xgm_dn8 + ((locals.var_gf_dn8 * locals.var_sqm) + (locals.var_gf * locals.var_sqm_dn8))))) / (assign45330_e58679 * assign45330_e58679)))), ((locals.var_phit1_dn9 * assign45330_e58680) + (locals.var_phit1 * (((((locals.var_gf2_dn9 * locals.var_dm) + (locals.var_gf2 * locals.var_dm_dn9)) * assign45330_e58679) - (assign45330_e58674 * (locals.var_xgm_dn9 + ((locals.var_gf_dn9 * locals.var_sqm) + (locals.var_gf * locals.var_sqm_dn9))))) / (assign45330_e58679 * assign45330_e58679)))),)
    } else {
        (locals.var_qim, locals.var_qim_dn4, locals.var_qim_dn6, locals.var_qim_dn7, locals.var_qim_dn8, locals.var_qim_dn9,)
    }
};
        locals.var_qim = assign45330_e58683;
        locals.var_qim_dn4 = assign45330_e58683_d_n4;
        locals.var_qim_dn6 = assign45330_e58683_d_n6;
        locals.var_qim_dn7 = assign45330_e58683_d_n7;
        locals.var_qim_dn8 = assign45330_e58683_d_n8;
        locals.var_qim_dn9 = assign45330_e58683_d_n9;

        let (assign45340_e58691, assign45340_e58691_d_n4, assign45340_e58691_d_n6, assign45340_e58691_d_n7, assign45340_e58691_d_n8, assign45340_e58691_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45340_e58688: f64 = (locals.var_phit1 * locals.var_alpha);
        let assign45340_e58689: f64 = (locals.var_qim + assign45340_e58688);
        (assign45340_e58689, (locals.var_qim_dn4 + ((locals.var_phit1_dn4 * locals.var_alpha) + (locals.var_phit1 * locals.var_alpha_dn4))), (locals.var_qim_dn6 + ((locals.var_phit1_dn6 * locals.var_alpha) + (locals.var_phit1 * locals.var_alpha_dn6))), (locals.var_qim_dn7 + ((locals.var_phit1_dn7 * locals.var_alpha) + (locals.var_phit1 * locals.var_alpha_dn7))), (locals.var_qim_dn8 + ((locals.var_phit1_dn8 * locals.var_alpha) + (locals.var_phit1 * locals.var_alpha_dn8))), (locals.var_qim_dn9 + ((locals.var_phit1_dn9 * locals.var_alpha) + (locals.var_phit1 * locals.var_alpha_dn9))),)
    } else {
        (locals.var_qim1, locals.var_qim1_dn4, locals.var_qim1_dn6, locals.var_qim1_dn7, locals.var_qim1_dn8, locals.var_qim1_dn9,)
    }
};
        locals.var_qim1 = assign45340_e58691;
        locals.var_qim1_dn4 = assign45340_e58691_d_n4;
        locals.var_qim1_dn6 = assign45340_e58691_d_n6;
        locals.var_qim1_dn7 = assign45340_e58691_d_n7;
        locals.var_qim1_dn8 = assign45340_e58691_d_n8;
        locals.var_qim1_dn9 = assign45340_e58691_d_n9;

        let (assign45350_e58699, assign45350_e58699_d_n4, assign45350_e58699_d_n6, assign45350_e58699_d_n7, assign45350_e58699_d_n8, assign45350_e58699_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45350_e58695: f64 = (locals.var_sqm * locals.var_gf);
        let assign45350_e58697: f64 = (assign45350_e58695 * locals.var_phit1);
        (assign45350_e58697, ((((locals.var_sqm_dn4 * locals.var_gf) + (locals.var_sqm * locals.var_gf_dn4)) * locals.var_phit1) + (assign45350_e58695 * locals.var_phit1_dn4)), ((((locals.var_sqm_dn6 * locals.var_gf) + (locals.var_sqm * locals.var_gf_dn6)) * locals.var_phit1) + (assign45350_e58695 * locals.var_phit1_dn6)), ((((locals.var_sqm_dn7 * locals.var_gf) + (locals.var_sqm * locals.var_gf_dn7)) * locals.var_phit1) + (assign45350_e58695 * locals.var_phit1_dn7)), ((((locals.var_sqm_dn8 * locals.var_gf) + (locals.var_sqm * locals.var_gf_dn8)) * locals.var_phit1) + (assign45350_e58695 * locals.var_phit1_dn8)), ((((locals.var_sqm_dn9 * locals.var_gf) + (locals.var_sqm * locals.var_gf_dn9)) * locals.var_phit1) + (assign45350_e58695 * locals.var_phit1_dn9)),)
    } else {
        (locals.var_qbm, locals.var_qbm_dn4, locals.var_qbm_dn6, locals.var_qbm_dn7, locals.var_qbm_dn8, locals.var_qbm_dn9,)
    }
};
        locals.var_qbm = assign45350_e58699;
        locals.var_qbm_dn4 = assign45350_e58699_d_n4;
        locals.var_qbm_dn6 = assign45350_e58699_d_n6;
        locals.var_qbm_dn7 = assign45350_e58699_d_n7;
        locals.var_qbm_dn8 = assign45350_e58699_d_n8;
        locals.var_qbm_dn9 = assign45350_e58699_d_n9;

        let assign45360_e58702: f64 = if locals.var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1233 = assign45360_e58702;

        let (assign45370_e58712, assign45370_e58712_d_n4, assign45370_e58712_d_n6, assign45370_e58712_d_n7, assign45370_e58712_d_n8, assign45370_e58712_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1233 != 0.0)) {
        let assign45370_e58709: f64 = (locals.var_rsg_i * locals.var_qim);
        let assign45370_e58710: f64 = (1.0 - assign45370_e58709);
        (assign45370_e58710, (-(locals.var_rsg_i * locals.var_qim_dn4)), (-(locals.var_rsg_i * locals.var_qim_dn6)), (-(locals.var_rsg_i * locals.var_qim_dn7)), (-(locals.var_rsg_i * locals.var_qim_dn8)), (-(locals.var_rsg_i * locals.var_qim_dn9)),)
    } else {
        (locals.var_rhog, locals.var_rhog_dn4, locals.var_rhog_dn6, locals.var_rhog_dn7, locals.var_rhog_dn8, locals.var_rhog_dn9,)
    }
};
        locals.var_rhog = assign45370_e58712;
        locals.var_rhog_dn4 = assign45370_e58712_d_n4;
        locals.var_rhog_dn6 = assign45370_e58712_d_n6;
        locals.var_rhog_dn7 = assign45370_e58712_d_n7;
        locals.var_rhog_dn8 = assign45370_e58712_d_n8;
        locals.var_rhog_dn9 = assign45370_e58712_d_n9;

        let (assign45380_e58725, assign45380_e58725_d_n4, assign45380_e58725_d_n6, assign45380_e58725_d_n7, assign45380_e58725_d_n8, assign45380_e58725_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1233 == 0.0)) {
        let assign45380_e58721: f64 = (locals.var_rsg_i * locals.var_qim);
        let assign45380_e58722: f64 = (1.0 + assign45380_e58721);
        let assign45380_e58723: f64 = (1.0 / assign45380_e58722);
        (assign45380_e58723, (-((locals.var_rsg_i * locals.var_qim_dn4) / (assign45380_e58722 * assign45380_e58722))), (-((locals.var_rsg_i * locals.var_qim_dn6) / (assign45380_e58722 * assign45380_e58722))), (-((locals.var_rsg_i * locals.var_qim_dn7) / (assign45380_e58722 * assign45380_e58722))), (-((locals.var_rsg_i * locals.var_qim_dn8) / (assign45380_e58722 * assign45380_e58722))), (-((locals.var_rsg_i * locals.var_qim_dn9) / (assign45380_e58722 * assign45380_e58722))),)
    } else {
        (locals.var_rhog, locals.var_rhog_dn4, locals.var_rhog_dn6, locals.var_rhog_dn7, locals.var_rhog_dn8, locals.var_rhog_dn9,)
    }
};
        locals.var_rhog = assign45380_e58725;
        locals.var_rhog_dn4 = assign45380_e58725_d_n4;
        locals.var_rhog_dn6 = assign45380_e58725_d_n6;
        locals.var_rhog_dn7 = assign45380_e58725_d_n7;
        locals.var_rhog_dn8 = assign45380_e58725_d_n8;
        locals.var_rhog_dn9 = assign45380_e58725_d_n9;

        let (assign45390_e58735, assign45390_e58735_d_n4, assign45390_e58735_d_n6, assign45390_e58735_d_n7, assign45390_e58735_d_n8, assign45390_e58735_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45390_e58729: f64 = (locals.var_ther_i * locals.var_rhob);
        let assign45390_e58731: f64 = (assign45390_e58729 * locals.var_rhog);
        let assign45390_e58733: f64 = (assign45390_e58731 * locals.var_qim);
        (assign45390_e58733, ((((((locals.var_ther_i_dn4 * locals.var_rhob) + (locals.var_ther_i * locals.var_rhob_dn4)) * locals.var_rhog) + (assign45390_e58729 * locals.var_rhog_dn4)) * locals.var_qim) + (assign45390_e58731 * locals.var_qim_dn4)), (((((locals.var_ther_i * locals.var_rhob_dn6) * locals.var_rhog) + (assign45390_e58729 * locals.var_rhog_dn6)) * locals.var_qim) + (assign45390_e58731 * locals.var_qim_dn6)), (((((locals.var_ther_i * locals.var_rhob_dn7) * locals.var_rhog) + (assign45390_e58729 * locals.var_rhog_dn7)) * locals.var_qim) + (assign45390_e58731 * locals.var_qim_dn7)), (((((locals.var_ther_i * locals.var_rhob_dn8) * locals.var_rhog) + (assign45390_e58729 * locals.var_rhog_dn8)) * locals.var_qim) + (assign45390_e58731 * locals.var_qim_dn8)), (((((locals.var_ther_i * locals.var_rhob_dn9) * locals.var_rhog) + (assign45390_e58729 * locals.var_rhog_dn9)) * locals.var_qim) + (assign45390_e58731 * locals.var_qim_dn9)),)
    } else {
        (locals.var_gr, locals.var_gr_dn4, locals.var_gr_dn6, locals.var_gr_dn7, locals.var_gr_dn8, locals.var_gr_dn9,)
    }
};
        locals.var_gr = assign45390_e58735;
        locals.var_gr_dn4 = assign45390_e58735_d_n4;
        locals.var_gr_dn6 = assign45390_e58735_d_n6;
        locals.var_gr_dn7 = assign45390_e58735_d_n7;
        locals.var_gr_dn8 = assign45390_e58735_d_n8;
        locals.var_gr_dn9 = assign45390_e58735_d_n9;

        let (assign45400_e58743, assign45400_e58743_d_n4, assign45400_e58743_d_n6, assign45400_e58743_d_n7, assign45400_e58743_d_n8, assign45400_e58743_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45400_e58740: f64 = (locals.var_eta_mu * locals.var_qim);
        let assign45400_e58741: f64 = (locals.var_qbm + assign45400_e58740);
        (assign45400_e58741, (locals.var_qbm_dn4 + (locals.var_eta_mu * locals.var_qim_dn4)), (locals.var_qbm_dn6 + (locals.var_eta_mu * locals.var_qim_dn6)), (locals.var_qbm_dn7 + (locals.var_eta_mu * locals.var_qim_dn7)), (locals.var_qbm_dn8 + (locals.var_eta_mu * locals.var_qim_dn8)), (locals.var_qbm_dn9 + (locals.var_eta_mu * locals.var_qim_dn9)),)
    } else {
        (locals.var_qeff, locals.var_qeff_dn4, locals.var_qeff_dn6, locals.var_qeff_dn7, locals.var_qeff_dn8, locals.var_qeff_dn9,)
    }
};
        locals.var_qeff = assign45400_e58743;
        locals.var_qeff_dn4 = assign45400_e58743_d_n4;
        locals.var_qeff_dn6 = assign45400_e58743_d_n6;
        locals.var_qeff_dn7 = assign45400_e58743_d_n7;
        locals.var_qeff_dn8 = assign45400_e58743_d_n8;
        locals.var_qeff_dn9 = assign45400_e58743_d_n9;

        let (assign45410_e58751, assign45410_e58751_d_n4, assign45410_e58751_d_n6, assign45410_e58751_d_n7, assign45410_e58751_d_n8, assign45410_e58751_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45410_e58748: f64 = (locals.var_eta_mu1 * locals.var_qim);
        let assign45410_e58749: f64 = (locals.var_qbm + assign45410_e58748);
        (assign45410_e58749, (locals.var_qbm_dn4 + (locals.var_eta_mu1 * locals.var_qim_dn4)), (locals.var_qbm_dn6 + (locals.var_eta_mu1 * locals.var_qim_dn6)), (locals.var_qbm_dn7 + (locals.var_eta_mu1 * locals.var_qim_dn7)), (locals.var_qbm_dn8 + (locals.var_eta_mu1 * locals.var_qim_dn8)), (locals.var_qbm_dn9 + (locals.var_eta_mu1 * locals.var_qim_dn9)),)
    } else {
        (locals.var_qeff1, locals.var_qeff1_dn4, locals.var_qeff1_dn6, locals.var_qeff1_dn7, locals.var_qeff1_dn8, locals.var_qeff1_dn9,)
    }
};
        locals.var_qeff1 = assign45410_e58751;
        locals.var_qeff1_dn4 = assign45410_e58751_d_n4;
        locals.var_qeff1_dn6 = assign45410_e58751_d_n6;
        locals.var_qeff1_dn7 = assign45410_e58751_d_n7;
        locals.var_qeff1_dn8 = assign45410_e58751_d_n8;
        locals.var_qeff1_dn9 = assign45410_e58751_d_n9;

        let (assign45420_e58757, assign45420_e58757_d_n4, assign45420_e58757_d_n6, assign45420_e58757_d_n7, assign45420_e58757_d_n8, assign45420_e58757_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45420_e58755: f64 = (locals.var_e_eff0 * locals.var_qeff);
        (assign45420_e58755, (locals.var_e_eff0 * locals.var_qeff_dn4), (locals.var_e_eff0 * locals.var_qeff_dn6), (locals.var_e_eff0 * locals.var_qeff_dn7), (locals.var_e_eff0 * locals.var_qeff_dn8), (locals.var_e_eff0 * locals.var_qeff_dn9),)
    } else {
        (locals.var_eeffm, locals.var_eeffm_dn4, locals.var_eeffm_dn6, locals.var_eeffm_dn7, locals.var_eeffm_dn8, locals.var_eeffm_dn9,)
    }
};
        locals.var_eeffm = assign45420_e58757;
        locals.var_eeffm_dn4 = assign45420_e58757_d_n4;
        locals.var_eeffm_dn6 = assign45420_e58757_d_n6;
        locals.var_eeffm_dn7 = assign45420_e58757_d_n7;
        locals.var_eeffm_dn8 = assign45420_e58757_d_n8;
        locals.var_eeffm_dn9 = assign45420_e58757_d_n9;

        let (assign45430_e58768, assign45430_e58768_d_n4, assign45430_e58768_d_n6, assign45430_e58768_d_n7, assign45430_e58768_d_n8, assign45430_e58768_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45430_e58762: f64 = (locals.var_pm + locals.var_dm);
        let assign45430_e58764: f64 = (assign45430_e58762 + 1e-14);
        let assign45430_e58765: f64 = (locals.var_pm / assign45430_e58764);
        let assign45430_e58766: f64 = (assign45430_e58765).ln();
        (assign45430_e58766, ((((locals.var_pm_dn4 * assign45430_e58764) - (locals.var_pm * (locals.var_pm_dn4 + locals.var_dm_dn4))) / (assign45430_e58764 * assign45430_e58764)) / assign45430_e58765), ((((locals.var_pm_dn6 * assign45430_e58764) - (locals.var_pm * (locals.var_pm_dn6 + locals.var_dm_dn6))) / (assign45430_e58764 * assign45430_e58764)) / assign45430_e58765), ((((locals.var_pm_dn7 * assign45430_e58764) - (locals.var_pm * (locals.var_pm_dn7 + locals.var_dm_dn7))) / (assign45430_e58764 * assign45430_e58764)) / assign45430_e58765), ((((locals.var_pm_dn8 * assign45430_e58764) - (locals.var_pm * (locals.var_pm_dn8 + locals.var_dm_dn8))) / (assign45430_e58764 * assign45430_e58764)) / assign45430_e58765), ((((locals.var_pm_dn9 * assign45430_e58764) - (locals.var_pm * (locals.var_pm_dn9 + locals.var_dm_dn9))) / (assign45430_e58764 * assign45430_e58764)) / assign45430_e58765),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign45430_e58768;
        locals.var_temp1_dn4 = assign45430_e58768_d_n4;
        locals.var_temp1_dn6 = assign45430_e58768_d_n6;
        locals.var_temp1_dn7 = assign45430_e58768_d_n7;
        locals.var_temp1_dn8 = assign45430_e58768_d_n8;
        locals.var_temp1_dn9 = assign45430_e58768_d_n9;

        let (assign45440_e58785, assign45440_e58785_d_n4, assign45440_e58785_d_n6, assign45440_e58785_d_n7, assign45440_e58785_d_n8, assign45440_e58785_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45440_e58772: f64 = (locals.var_eeffm * locals.var_mue_t);
        let assign45440_e58774: f64 = (assign45440_e58772).powf(locals.var_themu_t);
        let assign45440_e58778: f64 = (0.5 * locals.var_thecs_t);
        let assign45440_e58780: f64 = (assign45440_e58778 * locals.var_temp1);
        let assign45440_e58781: f64 = (assign45440_e58780).exp();
        let assign45440_e58782: f64 = (locals.var_cs_t * assign45440_e58781);
        let assign45440_e58783: f64 = (assign45440_e58774 + assign45440_e58782);
        (assign45440_e58783, (if locals.var_themu_t_dn4 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign45440_e58772).powf(locals.var_themu_t - 1.0) * ((locals.var_eeffm_dn4 * locals.var_mue_t) + (locals.var_eeffm * locals.var_mue_t_dn4)))) } } else { (assign45440_e58774 * ((locals.var_themu_t_dn4 * (assign45440_e58772).ln()) + (locals.var_themu_t * (((locals.var_eeffm_dn4 * locals.var_mue_t) + (locals.var_eeffm * locals.var_mue_t_dn4)) / assign45440_e58772)))) } + ((locals.var_cs_t_dn4 * assign45440_e58781) + (locals.var_cs_t * (assign45440_e58781 * (((0.5 * locals.var_thecs_t_dn4) * locals.var_temp1) + (assign45440_e58778 * locals.var_temp1_dn4)))))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign45440_e58772).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm_dn6 * locals.var_mue_t))) } } else { (assign45440_e58774 * (locals.var_themu_t * ((locals.var_eeffm_dn6 * locals.var_mue_t) / assign45440_e58772))) } + (locals.var_cs_t * (assign45440_e58781 * (assign45440_e58778 * locals.var_temp1_dn6)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign45440_e58772).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm_dn7 * locals.var_mue_t))) } } else { (assign45440_e58774 * (locals.var_themu_t * ((locals.var_eeffm_dn7 * locals.var_mue_t) / assign45440_e58772))) } + (locals.var_cs_t * (assign45440_e58781 * (assign45440_e58778 * locals.var_temp1_dn7)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign45440_e58772).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm_dn8 * locals.var_mue_t))) } } else { (assign45440_e58774 * (locals.var_themu_t * ((locals.var_eeffm_dn8 * locals.var_mue_t) / assign45440_e58772))) } + (locals.var_cs_t * (assign45440_e58781 * (assign45440_e58778 * locals.var_temp1_dn8)))), (if 0.0 == 0.0 && ((locals.var_themu_t) as f64).is_finite() && ((locals.var_themu_t) as f64).fract() == 0.0 { if locals.var_themu_t == 0.0 { 0.0 } else { (locals.var_themu_t * ((assign45440_e58772).powf(locals.var_themu_t - 1.0) * (locals.var_eeffm_dn9 * locals.var_mue_t))) } } else { (assign45440_e58774 * (locals.var_themu_t * ((locals.var_eeffm_dn9 * locals.var_mue_t) / assign45440_e58772))) } + (locals.var_cs_t * (assign45440_e58781 * (assign45440_e58778 * locals.var_temp1_dn9)))),)
    } else {
        (locals.var_mutmp, locals.var_mutmp_dn4, locals.var_mutmp_dn6, locals.var_mutmp_dn7, locals.var_mutmp_dn8, locals.var_mutmp_dn9,)
    }
};
        locals.var_mutmp = assign45440_e58785;
        locals.var_mutmp_dn4 = assign45440_e58785_d_n4;
        locals.var_mutmp_dn6 = assign45440_e58785_d_n6;
        locals.var_mutmp_dn7 = assign45440_e58785_d_n7;
        locals.var_mutmp_dn8 = assign45440_e58785_d_n8;
        locals.var_mutmp_dn9 = assign45440_e58785_d_n9;

        let (assign45450_e58795, assign45450_e58795_d_n4, assign45450_e58795_d_n6, assign45450_e58795_d_n7, assign45450_e58795_d_n8, assign45450_e58795_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45450_e58789: f64 = (1.0 + locals.var_mutmp);
        let assign45450_e58791: f64 = (assign45450_e58789 + locals.var_gr);
        let assign45450_e58793: f64 = (assign45450_e58791 * locals.var_rxcor);
        (assign45450_e58793, (((locals.var_mutmp_dn4 + locals.var_gr_dn4) * locals.var_rxcor) + (assign45450_e58791 * locals.var_rxcor_dn4)), (((locals.var_mutmp_dn6 + locals.var_gr_dn6) * locals.var_rxcor) + (assign45450_e58791 * locals.var_rxcor_dn6)), (((locals.var_mutmp_dn7 + locals.var_gr_dn7) * locals.var_rxcor) + (assign45450_e58791 * locals.var_rxcor_dn7)), (((locals.var_mutmp_dn8 + locals.var_gr_dn8) * locals.var_rxcor) + (assign45450_e58791 * locals.var_rxcor_dn8)), (((locals.var_mutmp_dn9 + locals.var_gr_dn9) * locals.var_rxcor) + (assign45450_e58791 * locals.var_rxcor_dn9)),)
    } else {
        (locals.var_gmob, locals.var_gmob_dn4, locals.var_gmob_dn6, locals.var_gmob_dn7, locals.var_gmob_dn8, locals.var_gmob_dn9,)
    }
};
        locals.var_gmob = assign45450_e58795;
        locals.var_gmob_dn4 = assign45450_e58795_d_n4;
        locals.var_gmob_dn6 = assign45450_e58795_d_n6;
        locals.var_gmob_dn7 = assign45450_e58795_d_n7;
        locals.var_gmob_dn8 = assign45450_e58795_d_n8;
        locals.var_gmob_dn9 = assign45450_e58795_d_n9;

        let (assign45460_e58814, assign45460_e58814_d_n4, assign45460_e58814_d_n6, assign45460_e58814_d_n7, assign45460_e58814_d_n8, assign45460_e58814_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45460_e58800: f64 = (locals.var_v_ds - locals.var_dps);
        let assign45460_e58802: f64 = (assign45460_e58800 * locals.var_inv_vp);
        let assign45460_e58803: f64 = (1.0 + assign45460_e58802);
        let assign45460_e58807: f64 = (locals.var_vdse - locals.var_dps);
        let assign45460_e58809: f64 = (assign45460_e58807 * locals.var_inv_vp);
        let assign45460_e58810: f64 = (1.0 + assign45460_e58809);
        let assign45460_e58811: f64 = (assign45460_e58803 / assign45460_e58810);
        let assign45460_e58812: f64 = (assign45460_e58811).ln();
        (assign45460_e58812, ((((((-locals.var_dps_dn4) * locals.var_inv_vp) * assign45460_e58810) - (assign45460_e58803 * ((locals.var_vdse_dn4 - locals.var_dps_dn4) * locals.var_inv_vp))) / (assign45460_e58810 * assign45460_e58810)) / assign45460_e58811), ((((((-locals.var_dps_dn6) * locals.var_inv_vp) * assign45460_e58810) - (assign45460_e58803 * ((locals.var_vdse_dn6 - locals.var_dps_dn6) * locals.var_inv_vp))) / (assign45460_e58810 * assign45460_e58810)) / assign45460_e58811), ((((((locals.var_v_ds_dn7 - locals.var_dps_dn7) * locals.var_inv_vp) * assign45460_e58810) - (assign45460_e58803 * ((locals.var_vdse_dn7 - locals.var_dps_dn7) * locals.var_inv_vp))) / (assign45460_e58810 * assign45460_e58810)) / assign45460_e58811), ((((((locals.var_v_ds_dn8 - locals.var_dps_dn8) * locals.var_inv_vp) * assign45460_e58810) - (assign45460_e58803 * ((locals.var_vdse_dn8 - locals.var_dps_dn8) * locals.var_inv_vp))) / (assign45460_e58810 * assign45460_e58810)) / assign45460_e58811), ((((((-locals.var_dps_dn9) * locals.var_inv_vp) * assign45460_e58810) - (assign45460_e58803 * ((locals.var_vdse_dn9 - locals.var_dps_dn9) * locals.var_inv_vp))) / (assign45460_e58810 * assign45460_e58810)) / assign45460_e58811),)
    } else {
        (locals.var_s1, locals.var_s1_dn4, locals.var_s1_dn6, locals.var_s1_dn7, locals.var_s1_dn8, locals.var_s1_dn9,)
    }
};
        locals.var_s1 = assign45460_e58814;
        locals.var_s1_dn4 = assign45460_e58814_d_n4;
        locals.var_s1_dn6 = assign45460_e58814_d_n6;
        locals.var_s1_dn7 = assign45460_e58814_d_n7;
        locals.var_s1_dn8 = assign45460_e58814_d_n8;
        locals.var_s1_dn9 = assign45460_e58814_d_n9;

        let (assign45470_e58820, assign45470_e58820_d_n4, assign45470_e58820_d_n6, assign45470_e58820_d_n7, assign45470_e58820_d_n8, assign45470_e58820_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45470_e58818: f64 = (locals.var_qim * locals.var_xitsb);
        (assign45470_e58818, ((locals.var_qim_dn4 * locals.var_xitsb) + (locals.var_qim * locals.var_xitsb_dn4)), ((locals.var_qim_dn6 * locals.var_xitsb) + (locals.var_qim * locals.var_xitsb_dn6)), ((locals.var_qim_dn7 * locals.var_xitsb) + (locals.var_qim * locals.var_xitsb_dn7)), ((locals.var_qim_dn8 * locals.var_xitsb) + (locals.var_qim * locals.var_xitsb_dn8)), ((locals.var_qim_dn9 * locals.var_xitsb) + (locals.var_qim * locals.var_xitsb_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign45470_e58820;
        locals.var_temp2_dn4 = assign45470_e58820_d_n4;
        locals.var_temp2_dn6 = assign45470_e58820_d_n6;
        locals.var_temp2_dn7 = assign45470_e58820_d_n7;
        locals.var_temp2_dn8 = assign45470_e58820_d_n8;
        locals.var_temp2_dn9 = assign45470_e58820_d_n9;

        let (assign45480_e58828, assign45480_e58828_d_n4, assign45480_e58828_d_n6, assign45480_e58828_d_n7, assign45480_e58828_d_n8, assign45480_e58828_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45480_e58825: f64 = (locals.var_thesatt_i + locals.var_temp2);
        let assign45480_e58826: f64 = (locals.var_temp2 / assign45480_e58825);
        (assign45480_e58826, (((locals.var_temp2_dn4 * assign45480_e58825) - (locals.var_temp2 * locals.var_temp2_dn4)) / (assign45480_e58825 * assign45480_e58825)), (((locals.var_temp2_dn6 * assign45480_e58825) - (locals.var_temp2 * locals.var_temp2_dn6)) / (assign45480_e58825 * assign45480_e58825)), (((locals.var_temp2_dn7 * assign45480_e58825) - (locals.var_temp2 * locals.var_temp2_dn7)) / (assign45480_e58825 * assign45480_e58825)), (((locals.var_temp2_dn8 * assign45480_e58825) - (locals.var_temp2 * locals.var_temp2_dn8)) / (assign45480_e58825 * assign45480_e58825)), (((locals.var_temp2_dn9 * assign45480_e58825) - (locals.var_temp2 * locals.var_temp2_dn9)) / (assign45480_e58825 * assign45480_e58825)),)
    } else {
        (locals.var_wsat, locals.var_wsat_dn4, locals.var_wsat_dn6, locals.var_wsat_dn7, locals.var_wsat_dn8, locals.var_wsat_dn9,)
    }
};
        locals.var_wsat = assign45480_e58828;
        locals.var_wsat_dn4 = assign45480_e58828_d_n4;
        locals.var_wsat_dn6 = assign45480_e58828_d_n6;
        locals.var_wsat_dn7 = assign45480_e58828_d_n7;
        locals.var_wsat_dn8 = assign45480_e58828_d_n8;
        locals.var_wsat_dn9 = assign45480_e58828_d_n9;

        let assign45490_e58831: f64 = if locals.var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1234 = assign45490_e58831;

        let (assign45500_e58843, assign45500_e58843_d_n4, assign45500_e58843_d_n6, assign45500_e58843_d_n7, assign45500_e58843_d_n8, assign45500_e58843_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1234 != 0.0)) {
        let assign45500_e58839: f64 = (locals.var_thesatg_i * locals.var_wsat);
        let assign45500_e58840: f64 = (1.0 - assign45500_e58839);
        let assign45500_e58841: f64 = (1.0 / assign45500_e58840);
        (assign45500_e58841, (-((-(locals.var_thesatg_i * locals.var_wsat_dn4)) / (assign45500_e58840 * assign45500_e58840))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn6)) / (assign45500_e58840 * assign45500_e58840))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn7)) / (assign45500_e58840 * assign45500_e58840))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn8)) / (assign45500_e58840 * assign45500_e58840))), (-((-(locals.var_thesatg_i * locals.var_wsat_dn9)) / (assign45500_e58840 * assign45500_e58840))),)
    } else {
        (locals.var_factheta, locals.var_factheta_dn4, locals.var_factheta_dn6, locals.var_factheta_dn7, locals.var_factheta_dn8, locals.var_factheta_dn9,)
    }
};
        locals.var_factheta = assign45500_e58843;
        locals.var_factheta_dn4 = assign45500_e58843_d_n4;
        locals.var_factheta_dn6 = assign45500_e58843_d_n6;
        locals.var_factheta_dn7 = assign45500_e58843_d_n7;
        locals.var_factheta_dn8 = assign45500_e58843_d_n8;
        locals.var_factheta_dn9 = assign45500_e58843_d_n9;

        let (assign45510_e58854, assign45510_e58854_d_n4, assign45510_e58854_d_n6, assign45510_e58854_d_n7, assign45510_e58854_d_n8, assign45510_e58854_d_n9,) = {
    if ((locals.var_guard1214 != 0.0) && (locals.var_guard1234 == 0.0)) {
        let assign45510_e58851: f64 = (locals.var_thesatg_i * locals.var_wsat);
        let assign45510_e58852: f64 = (1.0 + assign45510_e58851);
        (assign45510_e58852, (locals.var_thesatg_i * locals.var_wsat_dn4), (locals.var_thesatg_i * locals.var_wsat_dn6), (locals.var_thesatg_i * locals.var_wsat_dn7), (locals.var_thesatg_i * locals.var_wsat_dn8), (locals.var_thesatg_i * locals.var_wsat_dn9),)
    } else {
        (locals.var_factheta, locals.var_factheta_dn4, locals.var_factheta_dn6, locals.var_factheta_dn7, locals.var_factheta_dn8, locals.var_factheta_dn9,)
    }
};
        locals.var_factheta = assign45510_e58854;
        locals.var_factheta_dn4 = assign45510_e58854_d_n4;
        locals.var_factheta_dn6 = assign45510_e58854_d_n6;
        locals.var_factheta_dn7 = assign45510_e58854_d_n7;
        locals.var_factheta_dn8 = assign45510_e58854_d_n8;
        locals.var_factheta_dn9 = assign45510_e58854_d_n9;

        let (assign45520_e58860, assign45520_e58860_d_n4, assign45520_e58860_d_n6, assign45520_e58860_d_n7, assign45520_e58860_d_n8, assign45520_e58860_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45520_e58858: f64 = (locals.var_thesatloc * locals.var_factheta);
        (assign45520_e58858, ((locals.var_thesatloc_dn4 * locals.var_factheta) + (locals.var_thesatloc * locals.var_factheta_dn4)), (locals.var_thesatloc * locals.var_factheta_dn6), (locals.var_thesatloc * locals.var_factheta_dn7), (locals.var_thesatloc * locals.var_factheta_dn8), (locals.var_thesatloc * locals.var_factheta_dn9),)
    } else {
        (locals.var_thesateff, locals.var_thesateff_dn4, locals.var_thesateff_dn6, locals.var_thesateff_dn7, locals.var_thesateff_dn8, locals.var_thesateff_dn9,)
    }
};
        locals.var_thesateff = assign45520_e58860;
        locals.var_thesateff_dn4 = assign45520_e58860_d_n4;
        locals.var_thesateff_dn6 = assign45520_e58860_d_n6;
        locals.var_thesateff_dn7 = assign45520_e58860_d_n7;
        locals.var_thesateff_dn8 = assign45520_e58860_d_n8;
        locals.var_thesateff_dn9 = assign45520_e58860_d_n9;

        let (assign45530_e58866, assign45530_e58866_d_n4, assign45530_e58866_d_n6, assign45530_e58866_d_n7, assign45530_e58866_d_n8, assign45530_e58866_d_n9,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign45530_e58864: f64 = (locals.var_xgm * locals.var_phit1);
        (assign45530_e58864, ((locals.var_xgm_dn4 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn4)), ((locals.var_xgm_dn6 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn6)), ((locals.var_xgm_dn7 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn7)), ((locals.var_xgm_dn8 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn8)), ((locals.var_xgm_dn9 * locals.var_phit1) + (locals.var_xgm * locals.var_phit1_dn9)),)
    } else {
        (locals.var_voxm, locals.var_voxm_dn4, locals.var_voxm_dn6, locals.var_voxm_dn7, locals.var_voxm_dn8, locals.var_voxm_dn9,)
    }
};
        locals.var_voxm = assign45530_e58866;
        locals.var_voxm_dn4 = assign45530_e58866_d_n4;
        locals.var_voxm_dn6 = assign45530_e58866_d_n6;
        locals.var_voxm_dn7 = assign45530_e58866_d_n7;
        locals.var_voxm_dn8 = assign45530_e58866_d_n8;
        locals.var_voxm_dn9 = assign45530_e58866_d_n9;

        locals.var_vdsat_lim_dc = locals.var_vdsat_lim;
        locals.var_vdsat_lim_dc_dn4 = locals.var_vdsat_lim_dn4;
        locals.var_vdsat_lim_dc_dn6 = locals.var_vdsat_lim_dn6;
        locals.var_vdsat_lim_dc_dn7 = locals.var_vdsat_lim_dn7;
        locals.var_vdsat_lim_dc_dn8 = locals.var_vdsat_lim_dn8;
        locals.var_vdsat_lim_dc_dn9 = locals.var_vdsat_lim_dn9;

        locals.var_vdse_dc = locals.var_vdse;
        locals.var_vdse_dc_dn4 = locals.var_vdse_dn4;
        locals.var_vdse_dc_dn6 = locals.var_vdse_dn6;
        locals.var_vdse_dc_dn7 = locals.var_vdse_dn7;
        locals.var_vdse_dc_dn8 = locals.var_vdse_dn8;
        locals.var_vdse_dc_dn9 = locals.var_vdse_dn9;

        locals.var_udse_dc = locals.var_udse;
        locals.var_udse_dc_dn4 = locals.var_udse_dn4;
        locals.var_udse_dc_dn6 = locals.var_udse_dn6;
        locals.var_udse_dc_dn7 = locals.var_udse_dn7;
        locals.var_udse_dc_dn8 = locals.var_udse_dn8;
        locals.var_udse_dc_dn9 = locals.var_udse_dn9;

        locals.var_x_ds_dc = locals.var_x_ds;
        locals.var_x_ds_dc_dn4 = locals.var_x_ds_dn4;
        locals.var_x_ds_dc_dn6 = locals.var_x_ds_dn6;
        locals.var_x_ds_dc_dn7 = locals.var_x_ds_dn7;
        locals.var_x_ds_dc_dn8 = locals.var_x_ds_dn8;
        locals.var_x_ds_dc_dn9 = locals.var_x_ds_dn9;

        locals.var_dps_dc = locals.var_dps;
        locals.var_dps_dc_dn4 = locals.var_dps_dn4;
        locals.var_dps_dc_dn6 = locals.var_dps_dn6;
        locals.var_dps_dc_dn7 = locals.var_dps_dn7;
        locals.var_dps_dc_dn8 = locals.var_dps_dn8;
        locals.var_dps_dc_dn9 = locals.var_dps_dn9;

        locals.var_x_m_dc = locals.var_x_m;
        locals.var_x_m_dc_dn4 = locals.var_x_m_dn4;
        locals.var_x_m_dc_dn6 = locals.var_x_m_dn6;
        locals.var_x_m_dc_dn7 = locals.var_x_m_dn7;
        locals.var_x_m_dc_dn8 = locals.var_x_m_dn8;
        locals.var_x_m_dc_dn9 = locals.var_x_m_dn9;

        locals.var_qbd_dc = locals.var_qbd;
        locals.var_qbd_dc_dn4 = locals.var_qbd_dn4;
        locals.var_qbd_dc_dn6 = locals.var_qbd_dn6;
        locals.var_qbd_dc_dn7 = locals.var_qbd_dn7;
        locals.var_qbd_dc_dn8 = locals.var_qbd_dn8;
        locals.var_qbd_dc_dn9 = locals.var_qbd_dn9;

        locals.var_eta_p_dc = locals.var_eta_p;
        locals.var_eta_p_dc_dn4 = locals.var_eta_p_dn4;
        locals.var_eta_p_dc_dn6 = locals.var_eta_p_dn6;
        locals.var_eta_p_dc_dn7 = locals.var_eta_p_dn7;
        locals.var_eta_p_dc_dn8 = locals.var_eta_p_dn8;
        locals.var_eta_p_dc_dn9 = locals.var_eta_p_dn9;

        locals.var_alpha_dc = locals.var_alpha;
        locals.var_alpha_dc_dn4 = locals.var_alpha_dn4;
        locals.var_alpha_dc_dn6 = locals.var_alpha_dn6;
        locals.var_alpha_dc_dn7 = locals.var_alpha_dn7;
        locals.var_alpha_dc_dn8 = locals.var_alpha_dn8;
        locals.var_alpha_dc_dn9 = locals.var_alpha_dn9;

        locals.var_qim_dc = locals.var_qim;
        locals.var_qim_dc_dn4 = locals.var_qim_dn4;
        locals.var_qim_dc_dn6 = locals.var_qim_dn6;
        locals.var_qim_dc_dn7 = locals.var_qim_dn7;
        locals.var_qim_dc_dn8 = locals.var_qim_dn8;
        locals.var_qim_dc_dn9 = locals.var_qim_dn9;

        locals.var_qim1_dc = locals.var_qim1;
        locals.var_qim1_dc_dn4 = locals.var_qim1_dn4;
        locals.var_qim1_dc_dn6 = locals.var_qim1_dn6;
        locals.var_qim1_dc_dn7 = locals.var_qim1_dn7;
        locals.var_qim1_dc_dn8 = locals.var_qim1_dn8;
        locals.var_qim1_dc_dn9 = locals.var_qim1_dn9;

        locals.var_qbm_dc = locals.var_qbm;
        locals.var_qbm_dc_dn4 = locals.var_qbm_dn4;
        locals.var_qbm_dc_dn6 = locals.var_qbm_dn6;
        locals.var_qbm_dc_dn7 = locals.var_qbm_dn7;
        locals.var_qbm_dc_dn8 = locals.var_qbm_dn8;
        locals.var_qbm_dc_dn9 = locals.var_qbm_dn9;

        locals.var_qeff1_dc = locals.var_qeff1;
        locals.var_qeff1_dc_dn4 = locals.var_qeff1_dn4;
        locals.var_qeff1_dc_dn6 = locals.var_qeff1_dn6;
        locals.var_qeff1_dc_dn7 = locals.var_qeff1_dn7;
        locals.var_qeff1_dc_dn8 = locals.var_qeff1_dn8;
        locals.var_qeff1_dc_dn9 = locals.var_qeff1_dn9;

        locals.var_gmob_dc = locals.var_gmob;
        locals.var_gmob_dc_dn4 = locals.var_gmob_dn4;
        locals.var_gmob_dc_dn6 = locals.var_gmob_dn6;
        locals.var_gmob_dc_dn7 = locals.var_gmob_dn7;
        locals.var_gmob_dc_dn8 = locals.var_gmob_dn8;
        locals.var_gmob_dc_dn9 = locals.var_gmob_dn9;

        locals.var_s1_dc = locals.var_s1;
        locals.var_s1_dc_dn4 = locals.var_s1_dn4;
        locals.var_s1_dc_dn6 = locals.var_s1_dn6;
        locals.var_s1_dc_dn7 = locals.var_s1_dn7;
        locals.var_s1_dc_dn8 = locals.var_s1_dn8;
        locals.var_s1_dc_dn9 = locals.var_s1_dn9;

        locals.var_thesateff_dc = locals.var_thesateff;
        locals.var_thesateff_dc_dn4 = locals.var_thesateff_dn4;
        locals.var_thesateff_dc_dn6 = locals.var_thesateff_dn6;
        locals.var_thesateff_dc_dn7 = locals.var_thesateff_dn7;
        locals.var_thesateff_dc_dn8 = locals.var_thesateff_dn8;
        locals.var_thesateff_dc_dn9 = locals.var_thesateff_dn9;

        locals.var_voxm_dc = locals.var_voxm;
        locals.var_voxm_dc_dn4 = locals.var_voxm_dn4;
        locals.var_voxm_dc_dn6 = locals.var_voxm_dn6;
        locals.var_voxm_dc_dn7 = locals.var_voxm_dn7;
        locals.var_voxm_dc_dn8 = locals.var_voxm_dn8;
        locals.var_voxm_dc_dn9 = locals.var_voxm_dn9;

        locals.var_gdl_dc = 1.0;
        locals.var_gdl_dc_dn4 = 0.0;
        locals.var_gdl_dc_dn6 = 0.0;
        locals.var_gdl_dc_dn7 = 0.0;
        locals.var_gdl_dc_dn8 = 0.0;
        locals.var_gdl_dc_dn9 = 0.0;

        locals.var_gmob_dl_dc = 1.0;
        locals.var_gmob_dl_dc_dn4 = 0.0;
        locals.var_gmob_dl_dc_dn6 = 0.0;
        locals.var_gmob_dl_dc_dn7 = 0.0;
        locals.var_gmob_dl_dc_dn8 = 0.0;
        locals.var_gmob_dl_dc_dn9 = 0.0;

        locals.var_gvsatinv_dc = 1.0;
        locals.var_gvsatinv_dc_dn4 = 0.0;
        locals.var_gvsatinv_dc_dn6 = 0.0;
        locals.var_gvsatinv_dc_dn7 = 0.0;
        locals.var_gvsatinv_dc_dn8 = 0.0;
        locals.var_gvsatinv_dc_dn9 = 0.0;

        locals.var_h_dc = 1.0;
        locals.var_h_dc_dn4 = 0.0;
        locals.var_h_dc_dn6 = 0.0;
        locals.var_h_dc_dn7 = 0.0;
        locals.var_h_dc_dn8 = 0.0;
        locals.var_h_dc_dn9 = 0.0;

    }

    pub(super) fn stamp_transient_block_28(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_i_ds = 0.0;
        locals.var_i_ds_dn4 = 0.0;
        locals.var_i_ds_dn6 = 0.0;
        locals.var_i_ds_dn7 = 0.0;
        locals.var_i_ds_dn8 = 0.0;
        locals.var_i_ds_dn9 = 0.0;

        let assign45870_e58940: f64 = if locals.var_xg_dc > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1235 = assign45870_e58940;

        let (assign45880_e58949, assign45880_e58949_d_n7, assign45880_e58949_d_n8,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign45880_e58945: f64 = (locals.var_vdsx * locals.var_inv_vp);
        let assign45880_e58946: f64 = (1.0 + assign45880_e58945);
        let assign45880_e58947: f64 = (assign45880_e58946).ln();
        (assign45880_e58947, ((locals.var_vdsx_dn7 * locals.var_inv_vp) / assign45880_e58946), ((locals.var_vdsx_dn8 * locals.var_inv_vp) / assign45880_e58946),)
    } else {
        (locals.var_s2, locals.var_s2_dn7, locals.var_s2_dn8,)
    }
};
        locals.var_s2 = assign45880_e58949;
        locals.var_s2_dn7 = assign45880_e58949_d_n7;
        locals.var_s2_dn8 = assign45880_e58949_d_n8;

        let (assign45890_e58957, assign45890_e58957_d_n4, assign45890_e58957_d_n6, assign45890_e58957_d_n7, assign45890_e58957_d_n8, assign45890_e58957_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign45890_e58953: f64 = (locals.var_phit1_dc * locals.var_alpha_dc);
        let assign45890_e58955: f64 = (assign45890_e58953 / locals.var_qim1_dc);
        (assign45890_e58955, (((((locals.var_phit1_dc_dn4 * locals.var_alpha_dc) + (locals.var_phit1_dc * locals.var_alpha_dc_dn4)) * locals.var_qim1_dc) - (assign45890_e58953 * locals.var_qim1_dc_dn4)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((((locals.var_phit1_dc_dn6 * locals.var_alpha_dc) + (locals.var_phit1_dc * locals.var_alpha_dc_dn6)) * locals.var_qim1_dc) - (assign45890_e58953 * locals.var_qim1_dc_dn6)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((((locals.var_phit1_dc_dn7 * locals.var_alpha_dc) + (locals.var_phit1_dc * locals.var_alpha_dc_dn7)) * locals.var_qim1_dc) - (assign45890_e58953 * locals.var_qim1_dc_dn7)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((((locals.var_phit1_dc_dn8 * locals.var_alpha_dc) + (locals.var_phit1_dc * locals.var_alpha_dc_dn8)) * locals.var_qim1_dc) - (assign45890_e58953 * locals.var_qim1_dc_dn8)) / (locals.var_qim1_dc * locals.var_qim1_dc)), (((((locals.var_phit1_dc_dn9 * locals.var_alpha_dc) + (locals.var_phit1_dc * locals.var_alpha_dc_dn9)) * locals.var_qim1_dc) - (assign45890_e58953 * locals.var_qim1_dc_dn9)) / (locals.var_qim1_dc * locals.var_qim1_dc)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign45890_e58957;
        locals.var_temp__blk949_dn4 = assign45890_e58957_d_n4;
        locals.var_temp__blk949_dn6 = assign45890_e58957_d_n6;
        locals.var_temp__blk949_dn7 = assign45890_e58957_d_n7;
        locals.var_temp__blk949_dn8 = assign45890_e58957_d_n8;
        locals.var_temp__blk949_dn9 = assign45890_e58957_d_n9;

        let (assign45900_e58981, assign45900_e58981_d_n4, assign45900_e58981_d_n6, assign45900_e58981_d_n7, assign45900_e58981_d_n8, assign45900_e58981_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign45900_e58962: f64 = (locals.var_alp1_i / locals.var_qim1_dc);
        let assign45900_e58963: f64 = (locals.var_alp_i + assign45900_e58962);
        let assign45900_e58965: f64 = (assign45900_e58963 * locals.var_qim_dc);
        let assign45900_e58967: f64 = (assign45900_e58965 / locals.var_qim1_dc);
        let assign45900_e58969: f64 = (assign45900_e58967 * locals.var_s1_dc);
        let assign45900_e58972: f64 = (locals.var_alp2_i * locals.var_qbm_dc);
        let assign45900_e58974: f64 = (assign45900_e58972 * locals.var_temp__blk949);
        let assign45900_e58976: f64 = (assign45900_e58974 * locals.var_temp__blk949);
        let assign45900_e58978: f64 = (assign45900_e58976 * locals.var_s2);
        let assign45900_e58979: f64 = (assign45900_e58969 + assign45900_e58978);
        (assign45900_e58979, (((((((((-((locals.var_alp1_i * locals.var_qim1_dc_dn4) / (locals.var_qim1_dc * locals.var_qim1_dc))) * locals.var_qim_dc) + (assign45900_e58963 * locals.var_qim_dc_dn4)) * locals.var_qim1_dc) - (assign45900_e58965 * locals.var_qim1_dc_dn4)) / (locals.var_qim1_dc * locals.var_qim1_dc)) * locals.var_s1_dc) + (assign45900_e58967 * locals.var_s1_dc_dn4)) + ((((((locals.var_alp2_i * locals.var_qbm_dc_dn4) * locals.var_temp__blk949) + (assign45900_e58972 * locals.var_temp__blk949_dn4)) * locals.var_temp__blk949) + (assign45900_e58974 * locals.var_temp__blk949_dn4)) * locals.var_s2)), (((((((((-((locals.var_alp1_i * locals.var_qim1_dc_dn6) / (locals.var_qim1_dc * locals.var_qim1_dc))) * locals.var_qim_dc) + (assign45900_e58963 * locals.var_qim_dc_dn6)) * locals.var_qim1_dc) - (assign45900_e58965 * locals.var_qim1_dc_dn6)) / (locals.var_qim1_dc * locals.var_qim1_dc)) * locals.var_s1_dc) + (assign45900_e58967 * locals.var_s1_dc_dn6)) + ((((((locals.var_alp2_i * locals.var_qbm_dc_dn6) * locals.var_temp__blk949) + (assign45900_e58972 * locals.var_temp__blk949_dn6)) * locals.var_temp__blk949) + (assign45900_e58974 * locals.var_temp__blk949_dn6)) * locals.var_s2)), (((((((((-((locals.var_alp1_i * locals.var_qim1_dc_dn7) / (locals.var_qim1_dc * locals.var_qim1_dc))) * locals.var_qim_dc) + (assign45900_e58963 * locals.var_qim_dc_dn7)) * locals.var_qim1_dc) - (assign45900_e58965 * locals.var_qim1_dc_dn7)) / (locals.var_qim1_dc * locals.var_qim1_dc)) * locals.var_s1_dc) + (assign45900_e58967 * locals.var_s1_dc_dn7)) + (((((((locals.var_alp2_i * locals.var_qbm_dc_dn7) * locals.var_temp__blk949) + (assign45900_e58972 * locals.var_temp__blk949_dn7)) * locals.var_temp__blk949) + (assign45900_e58974 * locals.var_temp__blk949_dn7)) * locals.var_s2) + (assign45900_e58976 * locals.var_s2_dn7))), (((((((((-((locals.var_alp1_i * locals.var_qim1_dc_dn8) / (locals.var_qim1_dc * locals.var_qim1_dc))) * locals.var_qim_dc) + (assign45900_e58963 * locals.var_qim_dc_dn8)) * locals.var_qim1_dc) - (assign45900_e58965 * locals.var_qim1_dc_dn8)) / (locals.var_qim1_dc * locals.var_qim1_dc)) * locals.var_s1_dc) + (assign45900_e58967 * locals.var_s1_dc_dn8)) + (((((((locals.var_alp2_i * locals.var_qbm_dc_dn8) * locals.var_temp__blk949) + (assign45900_e58972 * locals.var_temp__blk949_dn8)) * locals.var_temp__blk949) + (assign45900_e58974 * locals.var_temp__blk949_dn8)) * locals.var_s2) + (assign45900_e58976 * locals.var_s2_dn8))), (((((((((-((locals.var_alp1_i * locals.var_qim1_dc_dn9) / (locals.var_qim1_dc * locals.var_qim1_dc))) * locals.var_qim_dc) + (assign45900_e58963 * locals.var_qim_dc_dn9)) * locals.var_qim1_dc) - (assign45900_e58965 * locals.var_qim1_dc_dn9)) / (locals.var_qim1_dc * locals.var_qim1_dc)) * locals.var_s1_dc) + (assign45900_e58967 * locals.var_s1_dc_dn9)) + ((((((locals.var_alp2_i * locals.var_qbm_dc_dn9) * locals.var_temp__blk949) + (assign45900_e58972 * locals.var_temp__blk949_dn9)) * locals.var_temp__blk949) + (assign45900_e58974 * locals.var_temp__blk949_dn9)) * locals.var_s2)),)
    } else {
        (locals.var_dl, locals.var_dl_dn4, locals.var_dl_dn6, locals.var_dl_dn7, locals.var_dl_dn8, locals.var_dl_dn9,)
    }
};
        locals.var_dl = assign45900_e58981;
        locals.var_dl_dn4 = assign45900_e58981_d_n4;
        locals.var_dl_dn6 = assign45900_e58981_d_n6;
        locals.var_dl_dn7 = assign45900_e58981_d_n7;
        locals.var_dl_dn8 = assign45900_e58981_d_n8;
        locals.var_dl_dn9 = assign45900_e58981_d_n9;

        let (assign45910_e58993, assign45910_e58993_d_n4, assign45910_e58993_d_n6, assign45910_e58993_d_n7, assign45910_e58993_d_n8, assign45910_e58993_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign45910_e58986: f64 = (1.0 + locals.var_dl);
        let assign45910_e58989: f64 = (locals.var_dl * locals.var_dl);
        let assign45910_e58990: f64 = (assign45910_e58986 + assign45910_e58989);
        let assign45910_e58991: f64 = (1.0 / assign45910_e58990);
        (assign45910_e58991, (-((locals.var_dl_dn4 + ((locals.var_dl_dn4 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn4))) / (assign45910_e58990 * assign45910_e58990))), (-((locals.var_dl_dn6 + ((locals.var_dl_dn6 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn6))) / (assign45910_e58990 * assign45910_e58990))), (-((locals.var_dl_dn7 + ((locals.var_dl_dn7 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn7))) / (assign45910_e58990 * assign45910_e58990))), (-((locals.var_dl_dn8 + ((locals.var_dl_dn8 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn8))) / (assign45910_e58990 * assign45910_e58990))), (-((locals.var_dl_dn9 + ((locals.var_dl_dn9 * locals.var_dl) + (locals.var_dl * locals.var_dl_dn9))) / (assign45910_e58990 * assign45910_e58990))),)
    } else {
        (locals.var_gdl_dc, locals.var_gdl_dc_dn4, locals.var_gdl_dc_dn6, locals.var_gdl_dc_dn7, locals.var_gdl_dc_dn8, locals.var_gdl_dc_dn9,)
    }
};
        locals.var_gdl_dc = assign45910_e58993;
        locals.var_gdl_dc_dn4 = assign45910_e58993_d_n4;
        locals.var_gdl_dc_dn6 = assign45910_e58993_d_n6;
        locals.var_gdl_dc_dn7 = assign45910_e58993_d_n7;
        locals.var_gdl_dc_dn8 = assign45910_e58993_d_n8;
        locals.var_gdl_dc_dn9 = assign45910_e58993_d_n9;

        let (assign45920_e58999, assign45920_e58999_d_n4, assign45920_e58999_d_n6, assign45920_e58999_d_n7, assign45920_e58999_d_n8, assign45920_e58999_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign45920_e58997: f64 = (locals.var_gmob_dc * locals.var_gdl_dc);
        (assign45920_e58997, ((locals.var_gmob_dc_dn4 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn4)), ((locals.var_gmob_dc_dn6 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn6)), ((locals.var_gmob_dc_dn7 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn7)), ((locals.var_gmob_dc_dn8 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn8)), ((locals.var_gmob_dc_dn9 * locals.var_gdl_dc) + (locals.var_gmob_dc * locals.var_gdl_dc_dn9)),)
    } else {
        (locals.var_gmob_dl_dc, locals.var_gmob_dl_dc_dn4, locals.var_gmob_dl_dc_dn6, locals.var_gmob_dl_dc_dn7, locals.var_gmob_dl_dc_dn8, locals.var_gmob_dl_dc_dn9,)
    }
};
        locals.var_gmob_dl_dc = assign45920_e58999;
        locals.var_gmob_dl_dc_dn4 = assign45920_e58999_d_n4;
        locals.var_gmob_dl_dc_dn6 = assign45920_e58999_d_n6;
        locals.var_gmob_dl_dc_dn7 = assign45920_e58999_d_n7;
        locals.var_gmob_dl_dc_dn8 = assign45920_e58999_d_n8;
        locals.var_gmob_dl_dc_dn9 = assign45920_e58999_d_n9;

        let (assign45930_e59005, assign45930_e59005_d_n4, assign45930_e59005_d_n6, assign45930_e59005_d_n7, assign45930_e59005_d_n8, assign45930_e59005_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign45930_e59003: f64 = (locals.var_thesateff_dc / locals.var_gmob_dl_dc);
        (assign45930_e59003, (((locals.var_thesateff_dc_dn4 * locals.var_gmob_dl_dc) - (locals.var_thesateff_dc * locals.var_gmob_dl_dc_dn4)) / (locals.var_gmob_dl_dc * locals.var_gmob_dl_dc)), (((locals.var_thesateff_dc_dn6 * locals.var_gmob_dl_dc) - (locals.var_thesateff_dc * locals.var_gmob_dl_dc_dn6)) / (locals.var_gmob_dl_dc * locals.var_gmob_dl_dc)), (((locals.var_thesateff_dc_dn7 * locals.var_gmob_dl_dc) - (locals.var_thesateff_dc * locals.var_gmob_dl_dc_dn7)) / (locals.var_gmob_dl_dc * locals.var_gmob_dl_dc)), (((locals.var_thesateff_dc_dn8 * locals.var_gmob_dl_dc) - (locals.var_thesateff_dc * locals.var_gmob_dl_dc_dn8)) / (locals.var_gmob_dl_dc * locals.var_gmob_dl_dc)), (((locals.var_thesateff_dc_dn9 * locals.var_gmob_dl_dc) - (locals.var_thesateff_dc * locals.var_gmob_dl_dc_dn9)) / (locals.var_gmob_dl_dc * locals.var_gmob_dl_dc)),)
    } else {
        (locals.var_thesat1_dc, locals.var_thesat1_dc_dn4, locals.var_thesat1_dc_dn6, locals.var_thesat1_dc_dn7, locals.var_thesat1_dc_dn8, locals.var_thesat1_dc_dn9,)
    }
};
        locals.var_thesat1_dc = assign45930_e59005;
        locals.var_thesat1_dc_dn4 = assign45930_e59005_d_n4;
        locals.var_thesat1_dc_dn6 = assign45930_e59005_d_n6;
        locals.var_thesat1_dc_dn7 = assign45930_e59005_d_n7;
        locals.var_thesat1_dc_dn8 = assign45930_e59005_d_n8;
        locals.var_thesat1_dc_dn9 = assign45930_e59005_d_n9;

        let (assign45940_e59015, assign45940_e59015_d_n4, assign45940_e59015_d_n6, assign45940_e59015_d_n7, assign45940_e59015_d_n8, assign45940_e59015_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign45940_e59009: f64 = (locals.var_thesat1_dc * locals.var_thesat1_dc);
        let assign45940_e59011: f64 = (assign45940_e59009 * locals.var_dps_dc);
        let assign45940_e59013: f64 = (assign45940_e59011 * locals.var_dps_dc);
        (assign45940_e59013, ((((((locals.var_thesat1_dc_dn4 * locals.var_thesat1_dc) + (locals.var_thesat1_dc * locals.var_thesat1_dc_dn4)) * locals.var_dps_dc) + (assign45940_e59009 * locals.var_dps_dc_dn4)) * locals.var_dps_dc) + (assign45940_e59011 * locals.var_dps_dc_dn4)), ((((((locals.var_thesat1_dc_dn6 * locals.var_thesat1_dc) + (locals.var_thesat1_dc * locals.var_thesat1_dc_dn6)) * locals.var_dps_dc) + (assign45940_e59009 * locals.var_dps_dc_dn6)) * locals.var_dps_dc) + (assign45940_e59011 * locals.var_dps_dc_dn6)), ((((((locals.var_thesat1_dc_dn7 * locals.var_thesat1_dc) + (locals.var_thesat1_dc * locals.var_thesat1_dc_dn7)) * locals.var_dps_dc) + (assign45940_e59009 * locals.var_dps_dc_dn7)) * locals.var_dps_dc) + (assign45940_e59011 * locals.var_dps_dc_dn7)), ((((((locals.var_thesat1_dc_dn8 * locals.var_thesat1_dc) + (locals.var_thesat1_dc * locals.var_thesat1_dc_dn8)) * locals.var_dps_dc) + (assign45940_e59009 * locals.var_dps_dc_dn8)) * locals.var_dps_dc) + (assign45940_e59011 * locals.var_dps_dc_dn8)), ((((((locals.var_thesat1_dc_dn9 * locals.var_thesat1_dc) + (locals.var_thesat1_dc * locals.var_thesat1_dc_dn9)) * locals.var_dps_dc) + (assign45940_e59009 * locals.var_dps_dc_dn9)) * locals.var_dps_dc) + (assign45940_e59011 * locals.var_dps_dc_dn9)),)
    } else {
        (locals.var_zsat, locals.var_zsat_dn4, locals.var_zsat_dn6, locals.var_zsat_dn7, locals.var_zsat_dn8, locals.var_zsat_dn9,)
    }
};
        locals.var_zsat = assign45940_e59015;
        locals.var_zsat_dn4 = assign45940_e59015_d_n4;
        locals.var_zsat_dn6 = assign45940_e59015_d_n6;
        locals.var_zsat_dn7 = assign45940_e59015_d_n7;
        locals.var_zsat_dn8 = assign45940_e59015_d_n8;
        locals.var_zsat_dn9 = assign45940_e59015_d_n9;

        let assign45950_e59018: f64 = (-1.0);
        let assign45950_e59019: f64 = if locals.var_chnl_type == assign45950_e59018 { 1.0 } else { 0.0 };
        locals.var_guard1236 = assign45950_e59019;

        let (assign45960_e59031, assign45960_e59031_d_n4, assign45960_e59031_d_n6, assign45960_e59031_d_n7, assign45960_e59031_d_n8, assign45960_e59031_d_n9,) = {
    if ((locals.var_guard1235 != 0.0) && (locals.var_guard1236 != 0.0)) {
        let assign45960_e59027: f64 = (locals.var_thesat1_dc * locals.var_dps_dc);
        let assign45960_e59028: f64 = (1.0 + assign45960_e59027);
        let assign45960_e59029: f64 = (locals.var_zsat / assign45960_e59028);
        (assign45960_e59029, (((locals.var_zsat_dn4 * assign45960_e59028) - (locals.var_zsat * ((locals.var_thesat1_dc_dn4 * locals.var_dps_dc) + (locals.var_thesat1_dc * locals.var_dps_dc_dn4)))) / (assign45960_e59028 * assign45960_e59028)), (((locals.var_zsat_dn6 * assign45960_e59028) - (locals.var_zsat * ((locals.var_thesat1_dc_dn6 * locals.var_dps_dc) + (locals.var_thesat1_dc * locals.var_dps_dc_dn6)))) / (assign45960_e59028 * assign45960_e59028)), (((locals.var_zsat_dn7 * assign45960_e59028) - (locals.var_zsat * ((locals.var_thesat1_dc_dn7 * locals.var_dps_dc) + (locals.var_thesat1_dc * locals.var_dps_dc_dn7)))) / (assign45960_e59028 * assign45960_e59028)), (((locals.var_zsat_dn8 * assign45960_e59028) - (locals.var_zsat * ((locals.var_thesat1_dc_dn8 * locals.var_dps_dc) + (locals.var_thesat1_dc * locals.var_dps_dc_dn8)))) / (assign45960_e59028 * assign45960_e59028)), (((locals.var_zsat_dn9 * assign45960_e59028) - (locals.var_zsat * ((locals.var_thesat1_dc_dn9 * locals.var_dps_dc) + (locals.var_thesat1_dc * locals.var_dps_dc_dn9)))) / (assign45960_e59028 * assign45960_e59028)),)
    } else {
        (locals.var_zsat, locals.var_zsat_dn4, locals.var_zsat_dn6, locals.var_zsat_dn7, locals.var_zsat_dn8, locals.var_zsat_dn9,)
    }
};
        locals.var_zsat = assign45960_e59031;
        locals.var_zsat_dn4 = assign45960_e59031_d_n4;
        locals.var_zsat_dn6 = assign45960_e59031_d_n6;
        locals.var_zsat_dn7 = assign45960_e59031_d_n7;
        locals.var_zsat_dn8 = assign45960_e59031_d_n8;
        locals.var_zsat_dn9 = assign45960_e59031_d_n9;

        let (assign45970_e59046, assign45970_e59046_d_n4, assign45970_e59046_d_n6, assign45970_e59046_d_n7, assign45970_e59046_d_n8, assign45970_e59046_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign45970_e59039: f64 = (2.0 * locals.var_zsat);
        let assign45970_e59040: f64 = (1.0 + assign45970_e59039);
        let assign45970_e59041: f64 = (assign45970_e59040).sqrt();
        let assign45970_e59042: f64 = (1.0 + assign45970_e59041);
        let assign45970_e59043: f64 = (locals.var_gmob_dl_dc * assign45970_e59042);
        let assign45970_e59044: f64 = (0.5 * assign45970_e59043);
        (assign45970_e59044, (0.5 * ((locals.var_gmob_dl_dc_dn4 * assign45970_e59042) + (locals.var_gmob_dl_dc * ((2.0 * locals.var_zsat_dn4) / (2.0 * assign45970_e59041))))), (0.5 * ((locals.var_gmob_dl_dc_dn6 * assign45970_e59042) + (locals.var_gmob_dl_dc * ((2.0 * locals.var_zsat_dn6) / (2.0 * assign45970_e59041))))), (0.5 * ((locals.var_gmob_dl_dc_dn7 * assign45970_e59042) + (locals.var_gmob_dl_dc * ((2.0 * locals.var_zsat_dn7) / (2.0 * assign45970_e59041))))), (0.5 * ((locals.var_gmob_dl_dc_dn8 * assign45970_e59042) + (locals.var_gmob_dl_dc * ((2.0 * locals.var_zsat_dn8) / (2.0 * assign45970_e59041))))), (0.5 * ((locals.var_gmob_dl_dc_dn9 * assign45970_e59042) + (locals.var_gmob_dl_dc * ((2.0 * locals.var_zsat_dn9) / (2.0 * assign45970_e59041))))),)
    } else {
        (locals.var_gvsat, locals.var_gvsat_dn4, locals.var_gvsat_dn6, locals.var_gvsat_dn7, locals.var_gvsat_dn8, locals.var_gvsat_dn9,)
    }
};
        locals.var_gvsat = assign45970_e59046;
        locals.var_gvsat_dn4 = assign45970_e59046_d_n4;
        locals.var_gvsat_dn6 = assign45970_e59046_d_n6;
        locals.var_gvsat_dn7 = assign45970_e59046_d_n7;
        locals.var_gvsat_dn8 = assign45970_e59046_d_n8;
        locals.var_gvsat_dn9 = assign45970_e59046_d_n9;

        let (assign45980_e59052, assign45980_e59052_d_n4, assign45980_e59052_d_n6, assign45980_e59052_d_n7, assign45980_e59052_d_n8, assign45980_e59052_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign45980_e59050: f64 = (1.0 / locals.var_gvsat);
        (assign45980_e59050, (-(locals.var_gvsat_dn4 / (locals.var_gvsat * locals.var_gvsat))), (-(locals.var_gvsat_dn6 / (locals.var_gvsat * locals.var_gvsat))), (-(locals.var_gvsat_dn7 / (locals.var_gvsat * locals.var_gvsat))), (-(locals.var_gvsat_dn8 / (locals.var_gvsat * locals.var_gvsat))), (-(locals.var_gvsat_dn9 / (locals.var_gvsat * locals.var_gvsat))),)
    } else {
        (locals.var_gvsatinv_dc, locals.var_gvsatinv_dc_dn4, locals.var_gvsatinv_dc_dn6, locals.var_gvsatinv_dc_dn7, locals.var_gvsatinv_dc_dn8, locals.var_gvsatinv_dc_dn9,)
    }
};
        locals.var_gvsatinv_dc = assign45980_e59052;
        locals.var_gvsatinv_dc_dn4 = assign45980_e59052_d_n4;
        locals.var_gvsatinv_dc_dn6 = assign45980_e59052_d_n6;
        locals.var_gvsatinv_dc_dn7 = assign45980_e59052_d_n7;
        locals.var_gvsatinv_dc_dn8 = assign45980_e59052_d_n8;
        locals.var_gvsatinv_dc_dn9 = assign45980_e59052_d_n9;

        let (assign45990_e59058, assign45990_e59058_d_n4, assign45990_e59058_d_n6, assign45990_e59058_d_n7, assign45990_e59058_d_n8, assign45990_e59058_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign45990_e59056: f64 = (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc);
        (assign45990_e59056, ((locals.var_gmob_dl_dc_dn4 * locals.var_gvsatinv_dc) + (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc_dn4)), ((locals.var_gmob_dl_dc_dn6 * locals.var_gvsatinv_dc) + (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc_dn6)), ((locals.var_gmob_dl_dc_dn7 * locals.var_gvsatinv_dc) + (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc_dn7)), ((locals.var_gmob_dl_dc_dn8 * locals.var_gvsatinv_dc) + (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc_dn8)), ((locals.var_gmob_dl_dc_dn9 * locals.var_gvsatinv_dc) + (locals.var_gmob_dl_dc * locals.var_gvsatinv_dc_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign45990_e59058;
        locals.var_temp__blk949_dn4 = assign45990_e59058_d_n4;
        locals.var_temp__blk949_dn6 = assign45990_e59058_d_n6;
        locals.var_temp__blk949_dn7 = assign45990_e59058_d_n7;
        locals.var_temp__blk949_dn8 = assign45990_e59058_d_n8;
        locals.var_temp__blk949_dn9 = assign45990_e59058_d_n9;

        let (assign46000_e59072, assign46000_e59072_d_n4, assign46000_e59072_d_n6, assign46000_e59072_d_n7, assign46000_e59072_d_n8, assign46000_e59072_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign46000_e59065: f64 = (locals.var_zsat * locals.var_temp__blk949);
        let assign46000_e59067: f64 = (assign46000_e59065 * locals.var_temp__blk949);
        let assign46000_e59068: f64 = (0.5 * assign46000_e59067);
        let assign46000_e59069: f64 = (1.0 + assign46000_e59068);
        let assign46000_e59070: f64 = (locals.var_alpha_dc * assign46000_e59069);
        (assign46000_e59070, ((locals.var_alpha_dc_dn4 * assign46000_e59069) + (locals.var_alpha_dc * (0.5 * ((((locals.var_zsat_dn4 * locals.var_temp__blk949) + (locals.var_zsat * locals.var_temp__blk949_dn4)) * locals.var_temp__blk949) + (assign46000_e59065 * locals.var_temp__blk949_dn4))))), ((locals.var_alpha_dc_dn6 * assign46000_e59069) + (locals.var_alpha_dc * (0.5 * ((((locals.var_zsat_dn6 * locals.var_temp__blk949) + (locals.var_zsat * locals.var_temp__blk949_dn6)) * locals.var_temp__blk949) + (assign46000_e59065 * locals.var_temp__blk949_dn6))))), ((locals.var_alpha_dc_dn7 * assign46000_e59069) + (locals.var_alpha_dc * (0.5 * ((((locals.var_zsat_dn7 * locals.var_temp__blk949) + (locals.var_zsat * locals.var_temp__blk949_dn7)) * locals.var_temp__blk949) + (assign46000_e59065 * locals.var_temp__blk949_dn7))))), ((locals.var_alpha_dc_dn8 * assign46000_e59069) + (locals.var_alpha_dc * (0.5 * ((((locals.var_zsat_dn8 * locals.var_temp__blk949) + (locals.var_zsat * locals.var_temp__blk949_dn8)) * locals.var_temp__blk949) + (assign46000_e59065 * locals.var_temp__blk949_dn8))))), ((locals.var_alpha_dc_dn9 * assign46000_e59069) + (locals.var_alpha_dc * (0.5 * ((((locals.var_zsat_dn9 * locals.var_temp__blk949) + (locals.var_zsat * locals.var_temp__blk949_dn9)) * locals.var_temp__blk949) + (assign46000_e59065 * locals.var_temp__blk949_dn9))))),)
    } else {
        (locals.var_alpha1, locals.var_alpha1_dn4, locals.var_alpha1_dn6, locals.var_alpha1_dn7, locals.var_alpha1_dn8, locals.var_alpha1_dn9,)
    }
};
        locals.var_alpha1 = assign46000_e59072;
        locals.var_alpha1_dn4 = assign46000_e59072_d_n4;
        locals.var_alpha1_dn6 = assign46000_e59072_d_n6;
        locals.var_alpha1_dn7 = assign46000_e59072_d_n7;
        locals.var_alpha1_dn8 = assign46000_e59072_d_n8;
        locals.var_alpha1_dn9 = assign46000_e59072_d_n9;

        let (assign46010_e59080, assign46010_e59080_d_n4, assign46010_e59080_d_n6, assign46010_e59080_d_n7, assign46010_e59080_d_n8, assign46010_e59080_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign46010_e59076: f64 = (locals.var_temp__blk949 * locals.var_qim1_dc);
        let assign46010_e59078: f64 = (assign46010_e59076 / locals.var_alpha1);
        (assign46010_e59078, (((((locals.var_temp__blk949_dn4 * locals.var_qim1_dc) + (locals.var_temp__blk949 * locals.var_qim1_dc_dn4)) * locals.var_alpha1) - (assign46010_e59076 * locals.var_alpha1_dn4)) / (locals.var_alpha1 * locals.var_alpha1)), (((((locals.var_temp__blk949_dn6 * locals.var_qim1_dc) + (locals.var_temp__blk949 * locals.var_qim1_dc_dn6)) * locals.var_alpha1) - (assign46010_e59076 * locals.var_alpha1_dn6)) / (locals.var_alpha1 * locals.var_alpha1)), (((((locals.var_temp__blk949_dn7 * locals.var_qim1_dc) + (locals.var_temp__blk949 * locals.var_qim1_dc_dn7)) * locals.var_alpha1) - (assign46010_e59076 * locals.var_alpha1_dn7)) / (locals.var_alpha1 * locals.var_alpha1)), (((((locals.var_temp__blk949_dn8 * locals.var_qim1_dc) + (locals.var_temp__blk949 * locals.var_qim1_dc_dn8)) * locals.var_alpha1) - (assign46010_e59076 * locals.var_alpha1_dn8)) / (locals.var_alpha1 * locals.var_alpha1)), (((((locals.var_temp__blk949_dn9 * locals.var_qim1_dc) + (locals.var_temp__blk949 * locals.var_qim1_dc_dn9)) * locals.var_alpha1) - (assign46010_e59076 * locals.var_alpha1_dn9)) / (locals.var_alpha1 * locals.var_alpha1)),)
    } else {
        (locals.var_h_dc, locals.var_h_dc_dn4, locals.var_h_dc_dn6, locals.var_h_dc_dn7, locals.var_h_dc_dn8, locals.var_h_dc_dn9,)
    }
};
        locals.var_h_dc = assign46010_e59080;
        locals.var_h_dc_dn4 = assign46010_e59080_d_n4;
        locals.var_h_dc_dn6 = assign46010_e59080_d_n6;
        locals.var_h_dc_dn7 = assign46010_e59080_d_n7;
        locals.var_h_dc_dn8 = assign46010_e59080_d_n8;
        locals.var_h_dc_dn9 = assign46010_e59080_d_n9;

        let (assign46020_e59090, assign46020_e59090_d_n4, assign46020_e59090_d_n6, assign46020_e59090_d_n7, assign46020_e59090_d_n8, assign46020_e59090_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign46020_e59084: f64 = (locals.var_bet_i * locals.var_qim1_dc);
        let assign46020_e59086: f64 = (assign46020_e59084 * locals.var_dps_dc);
        let assign46020_e59088: f64 = (assign46020_e59086 * locals.var_gvsatinv_dc);
        (assign46020_e59088, ((((((locals.var_bet_i_dn4 * locals.var_qim1_dc) + (locals.var_bet_i * locals.var_qim1_dc_dn4)) * locals.var_dps_dc) + (assign46020_e59084 * locals.var_dps_dc_dn4)) * locals.var_gvsatinv_dc) + (assign46020_e59086 * locals.var_gvsatinv_dc_dn4)), (((((locals.var_bet_i * locals.var_qim1_dc_dn6) * locals.var_dps_dc) + (assign46020_e59084 * locals.var_dps_dc_dn6)) * locals.var_gvsatinv_dc) + (assign46020_e59086 * locals.var_gvsatinv_dc_dn6)), (((((locals.var_bet_i * locals.var_qim1_dc_dn7) * locals.var_dps_dc) + (assign46020_e59084 * locals.var_dps_dc_dn7)) * locals.var_gvsatinv_dc) + (assign46020_e59086 * locals.var_gvsatinv_dc_dn7)), (((((locals.var_bet_i * locals.var_qim1_dc_dn8) * locals.var_dps_dc) + (assign46020_e59084 * locals.var_dps_dc_dn8)) * locals.var_gvsatinv_dc) + (assign46020_e59086 * locals.var_gvsatinv_dc_dn8)), (((((locals.var_bet_i * locals.var_qim1_dc_dn9) * locals.var_dps_dc) + (assign46020_e59084 * locals.var_dps_dc_dn9)) * locals.var_gvsatinv_dc) + (assign46020_e59086 * locals.var_gvsatinv_dc_dn9)),)
    } else {
        (locals.var_i_ds, locals.var_i_ds_dn4, locals.var_i_ds_dn6, locals.var_i_ds_dn7, locals.var_i_ds_dn8, locals.var_i_ds_dn9,)
    }
};
        locals.var_i_ds = assign46020_e59090;
        locals.var_i_ds_dn4 = assign46020_e59090_d_n4;
        locals.var_i_ds_dn6 = assign46020_e59090_d_n6;
        locals.var_i_ds_dn7 = assign46020_e59090_d_n7;
        locals.var_i_ds_dn8 = assign46020_e59090_d_n8;
        locals.var_i_ds_dn9 = assign46020_e59090_d_n9;

        locals.var_xs_ov = 0.0;
        locals.var_xs_ov_dn6 = 0.0;
        locals.var_xs_ov_dn7 = 0.0;
        locals.var_xs_ov_dn8 = 0.0;

        locals.var_xd_ov = 0.0;
        locals.var_xd_ov_dn6 = 0.0;
        locals.var_xd_ov_dn7 = 0.0;
        locals.var_xd_ov_dn8 = 0.0;

        locals.var_vovs = 0.0;
        locals.var_vovs_dn6 = 0.0;
        locals.var_vovs_dn7 = 0.0;
        locals.var_vovs_dn8 = 0.0;

        locals.var_vovd = 0.0;
        locals.var_vovd_dn6 = 0.0;
        locals.var_vovd_dn7 = 0.0;
        locals.var_vovd_dn8 = 0.0;

        let assign46070_e59125: f64 = if (((((p.p40 != 0.0) && ((locals.var_igov_i > 0.0) || (locals.var_igovd_i > 0.0))) || ((p.p42 != 0.0) && ((locals.var_agidl_i > 0.0) || (locals.var_agidld_i > 0.0)))) || (locals.var_cgov_i > 0.0)) || (locals.var_cgovd_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1237 = assign46070_e59125;

        let (assign46080_e59138, assign46080_e59138_d_n6, assign46080_e59138_d_n7, assign46080_e59138_d_n8,) = {
    if (locals.var_guard1237 != 0.0) {
        let assign46080_e59131: f64 = (locals.var_xgs_ov * locals.var_xgs_ov);
        let assign46080_e59133: f64 = (assign46080_e59131 + locals.var_sp_ov_eps2_s);
        let assign46080_e59134: f64 = (assign46080_e59133).sqrt();
        let assign46080_e59135: f64 = (locals.var_xgs_ov + assign46080_e59134);
        let assign46080_e59136: f64 = (0.5 * assign46080_e59135);
        (assign46080_e59136, (0.5 * (locals.var_xgs_ov_dn6 + (((locals.var_xgs_ov_dn6 * locals.var_xgs_ov) + (locals.var_xgs_ov * locals.var_xgs_ov_dn6)) / (2.0 * assign46080_e59134)))), (0.5 * (locals.var_xgs_ov_dn7 + (((locals.var_xgs_ov_dn7 * locals.var_xgs_ov) + (locals.var_xgs_ov * locals.var_xgs_ov_dn7)) / (2.0 * assign46080_e59134)))), (0.5 * (locals.var_xgs_ov_dn8 + (((locals.var_xgs_ov_dn8 * locals.var_xgs_ov) + (locals.var_xgs_ov * locals.var_xgs_ov_dn8)) / (2.0 * assign46080_e59134)))),)
    } else {
        (locals.var_sp_ov_xg, locals.var_sp_ov_xg_dn6, locals.var_sp_ov_xg_dn7, locals.var_sp_ov_xg_dn8,)
    }
};
        locals.var_sp_ov_xg = assign46080_e59138;
        locals.var_sp_ov_xg_dn6 = assign46080_e59138_d_n6;
        locals.var_sp_ov_xg_dn7 = assign46080_e59138_d_n7;
        locals.var_sp_ov_xg_dn8 = assign46080_e59138_d_n8;

        let (assign46090_e59160, assign46090_e59160_d_n6, assign46090_e59160_d_n7, assign46090_e59160_d_n8,) = {
    if (locals.var_guard1237 != 0.0) {
        let assign46090_e59141: f64 = (-locals.var_sp_ov_xg);
        let assign46090_e59144: f64 = (locals.var_gov2_s * 0.5);
        let assign46090_e59145: f64 = (assign46090_e59141 - assign46090_e59144);
        let assign46090_e59150: f64 = (locals.var_gov2_s * 0.25);
        let assign46090_e59151: f64 = (locals.var_sp_ov_xg + assign46090_e59150);
        let assign46090_e59153: f64 = (assign46090_e59151 + locals.var_sp_ov_a_s);
        let assign46090_e59154: f64 = (assign46090_e59153).sqrt();
        let assign46090_e59155: f64 = (locals.var_gov_s * assign46090_e59154);
        let assign46090_e59156: f64 = (assign46090_e59145 + assign46090_e59155);
        let assign46090_e59158: f64 = (assign46090_e59156 + locals.var_sp_ov_delta1_s);
        (assign46090_e59158, ((-locals.var_sp_ov_xg_dn6) + (locals.var_gov_s * (locals.var_sp_ov_xg_dn6 / (2.0 * assign46090_e59154)))), ((-locals.var_sp_ov_xg_dn7) + (locals.var_gov_s * (locals.var_sp_ov_xg_dn7 / (2.0 * assign46090_e59154)))), ((-locals.var_sp_ov_xg_dn8) + (locals.var_gov_s * (locals.var_sp_ov_xg_dn8 / (2.0 * assign46090_e59154)))),)
    } else {
        (locals.var_xs_ov, locals.var_xs_ov_dn6, locals.var_xs_ov_dn7, locals.var_xs_ov_dn8,)
    }
};
        locals.var_xs_ov = assign46090_e59160;
        locals.var_xs_ov_dn6 = assign46090_e59160_d_n6;
        locals.var_xs_ov_dn7 = assign46090_e59160_d_n7;
        locals.var_xs_ov_dn8 = assign46090_e59160_d_n8;

        let (assign46100_e59173, assign46100_e59173_d_n6, assign46100_e59173_d_n7, assign46100_e59173_d_n8,) = {
    if (locals.var_guard1237 != 0.0) {
        let assign46100_e59166: f64 = (locals.var_xgd_ov * locals.var_xgd_ov);
        let assign46100_e59168: f64 = (assign46100_e59166 + locals.var_sp_ov_eps2_d);
        let assign46100_e59169: f64 = (assign46100_e59168).sqrt();
        let assign46100_e59170: f64 = (locals.var_xgd_ov + assign46100_e59169);
        let assign46100_e59171: f64 = (0.5 * assign46100_e59170);
        (assign46100_e59171, (0.5 * (locals.var_xgd_ov_dn6 + (((locals.var_xgd_ov_dn6 * locals.var_xgd_ov) + (locals.var_xgd_ov * locals.var_xgd_ov_dn6)) / (2.0 * assign46100_e59169)))), (0.5 * (locals.var_xgd_ov_dn7 + (((locals.var_xgd_ov_dn7 * locals.var_xgd_ov) + (locals.var_xgd_ov * locals.var_xgd_ov_dn7)) / (2.0 * assign46100_e59169)))), (0.5 * (locals.var_xgd_ov_dn8 + (((locals.var_xgd_ov_dn8 * locals.var_xgd_ov) + (locals.var_xgd_ov * locals.var_xgd_ov_dn8)) / (2.0 * assign46100_e59169)))),)
    } else {
        (locals.var_sp_ov_xg, locals.var_sp_ov_xg_dn6, locals.var_sp_ov_xg_dn7, locals.var_sp_ov_xg_dn8,)
    }
};
        locals.var_sp_ov_xg = assign46100_e59173;
        locals.var_sp_ov_xg_dn6 = assign46100_e59173_d_n6;
        locals.var_sp_ov_xg_dn7 = assign46100_e59173_d_n7;
        locals.var_sp_ov_xg_dn8 = assign46100_e59173_d_n8;

        let (assign46110_e59195, assign46110_e59195_d_n6, assign46110_e59195_d_n7, assign46110_e59195_d_n8,) = {
    if (locals.var_guard1237 != 0.0) {
        let assign46110_e59176: f64 = (-locals.var_sp_ov_xg);
        let assign46110_e59179: f64 = (locals.var_gov2_d * 0.5);
        let assign46110_e59180: f64 = (assign46110_e59176 - assign46110_e59179);
        let assign46110_e59185: f64 = (locals.var_gov2_d * 0.25);
        let assign46110_e59186: f64 = (locals.var_sp_ov_xg + assign46110_e59185);
        let assign46110_e59188: f64 = (assign46110_e59186 + locals.var_sp_ov_a_d);
        let assign46110_e59189: f64 = (assign46110_e59188).sqrt();
        let assign46110_e59190: f64 = (locals.var_gov_d * assign46110_e59189);
        let assign46110_e59191: f64 = (assign46110_e59180 + assign46110_e59190);
        let assign46110_e59193: f64 = (assign46110_e59191 + locals.var_sp_ov_delta1_d);
        (assign46110_e59193, ((-locals.var_sp_ov_xg_dn6) + (locals.var_gov_d * (locals.var_sp_ov_xg_dn6 / (2.0 * assign46110_e59189)))), ((-locals.var_sp_ov_xg_dn7) + (locals.var_gov_d * (locals.var_sp_ov_xg_dn7 / (2.0 * assign46110_e59189)))), ((-locals.var_sp_ov_xg_dn8) + (locals.var_gov_d * (locals.var_sp_ov_xg_dn8 / (2.0 * assign46110_e59189)))),)
    } else {
        (locals.var_xd_ov, locals.var_xd_ov_dn6, locals.var_xd_ov_dn7, locals.var_xd_ov_dn8,)
    }
};
        locals.var_xd_ov = assign46110_e59195;
        locals.var_xd_ov_dn6 = assign46110_e59195_d_n6;
        locals.var_xd_ov_dn7 = assign46110_e59195_d_n7;
        locals.var_xd_ov_dn8 = assign46110_e59195_d_n8;

        let (assign46120_e59204, assign46120_e59204_d_n6, assign46120_e59204_d_n7, assign46120_e59204_d_n8,) = {
    if (locals.var_guard1237 != 0.0) {
        let assign46120_e59198: f64 = (-locals.var_phita);
        let assign46120_e59201: f64 = (locals.var_xgs_ov + locals.var_xs_ov);
        let assign46120_e59202: f64 = (assign46120_e59198 * assign46120_e59201);
        (assign46120_e59202, (assign46120_e59198 * (locals.var_xgs_ov_dn6 + locals.var_xs_ov_dn6)), (assign46120_e59198 * (locals.var_xgs_ov_dn7 + locals.var_xs_ov_dn7)), (assign46120_e59198 * (locals.var_xgs_ov_dn8 + locals.var_xs_ov_dn8)),)
    } else {
        (locals.var_vovs, locals.var_vovs_dn6, locals.var_vovs_dn7, locals.var_vovs_dn8,)
    }
};
        locals.var_vovs = assign46120_e59204;
        locals.var_vovs_dn6 = assign46120_e59204_d_n6;
        locals.var_vovs_dn7 = assign46120_e59204_d_n7;
        locals.var_vovs_dn8 = assign46120_e59204_d_n8;

        let (assign46130_e59213, assign46130_e59213_d_n6, assign46130_e59213_d_n7, assign46130_e59213_d_n8,) = {
    if (locals.var_guard1237 != 0.0) {
        let assign46130_e59207: f64 = (-locals.var_phita);
        let assign46130_e59210: f64 = (locals.var_xgd_ov + locals.var_xd_ov);
        let assign46130_e59211: f64 = (assign46130_e59207 * assign46130_e59210);
        (assign46130_e59211, (assign46130_e59207 * (locals.var_xgd_ov_dn6 + locals.var_xd_ov_dn6)), (assign46130_e59207 * (locals.var_xgd_ov_dn7 + locals.var_xd_ov_dn7)), (assign46130_e59207 * (locals.var_xgd_ov_dn8 + locals.var_xd_ov_dn8)),)
    } else {
        (locals.var_vovd, locals.var_vovd_dn6, locals.var_vovd_dn7, locals.var_vovd_dn8,)
    }
};
        locals.var_vovd = assign46130_e59213;
        locals.var_vovd_dn6 = assign46130_e59213_d_n6;
        locals.var_vovd_dn7 = assign46130_e59213_d_n7;
        locals.var_vovd_dn8 = assign46130_e59213_d_n8;

        locals.var_igsov = 0.0;
        locals.var_igsov_dn4 = 0.0;
        locals.var_igsov_dn6 = 0.0;
        locals.var_igsov_dn7 = 0.0;
        locals.var_igsov_dn8 = 0.0;
        locals.var_igsov_dn9 = 0.0;

        locals.var_igdov = 0.0;
        locals.var_igdov_dn4 = 0.0;
        locals.var_igdov_dn6 = 0.0;
        locals.var_igdov_dn7 = 0.0;
        locals.var_igdov_dn8 = 0.0;
        locals.var_igdov_dn9 = 0.0;

        locals.var_igc_1 = 0.0;
        locals.var_igc_1_dn4 = 0.0;
        locals.var_igc_1_dn6 = 0.0;
        locals.var_igc_1_dn7 = 0.0;
        locals.var_igc_1_dn8 = 0.0;
        locals.var_igc_1_dn9 = 0.0;

        locals.var_i_gb = 0.0;
        locals.var_i_gb_dn4 = 0.0;
        locals.var_i_gb_dn6 = 0.0;
        locals.var_i_gb_dn7 = 0.0;
        locals.var_i_gb_dn8 = 0.0;
        locals.var_i_gb_dn9 = 0.0;

        locals.var_i_gcs = 0.0;
        locals.var_i_gcs_dn4 = 0.0;
        locals.var_i_gcs_dn6 = 0.0;
        locals.var_i_gcs_dn7 = 0.0;
        locals.var_i_gcs_dn8 = 0.0;
        locals.var_i_gcs_dn9 = 0.0;

        locals.var_i_gcd = 0.0;
        locals.var_i_gcd_dn4 = 0.0;
        locals.var_i_gcd_dn6 = 0.0;
        locals.var_i_gcd_dn7 = 0.0;
        locals.var_i_gcd_dn8 = 0.0;
        locals.var_i_gcd_dn9 = 0.0;

        let assign46200_e59222: f64 = if p.p40 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1238 = assign46200_e59222;

        let assign46210_e59225: f64 = if locals.var_igov_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1239 = assign46210_e59225;

        let (assign46220_e59238, assign46220_e59238_d_n4, assign46220_e59238_d_n6, assign46220_e59238_d_n7, assign46220_e59238_d_n8, assign46220_e59238_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
        let assign46220_e59231: f64 = (locals.var_vovs * locals.var_vovs);
        let assign46220_e59233: f64 = (assign46220_e59231 + 1e-6);
        let assign46220_e59234: f64 = (assign46220_e59233).sqrt();
        let assign46220_e59236: f64 = (assign46220_e59234 * locals.var_inv_chib);
        (assign46220_e59236, 0.0, ((((locals.var_vovs_dn6 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn6)) / (2.0 * assign46220_e59234)) * locals.var_inv_chib), ((((locals.var_vovs_dn7 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn7)) / (2.0 * assign46220_e59234)) * locals.var_inv_chib), ((((locals.var_vovs_dn8 * locals.var_vovs) + (locals.var_vovs * locals.var_vovs_dn8)) / (2.0 * assign46220_e59234)) * locals.var_inv_chib), 0.0,)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign46220_e59238;
        locals.var_zg_dn4 = assign46220_e59238_d_n4;
        locals.var_zg_dn6 = assign46220_e59238_d_n6;
        locals.var_zg_dn7 = assign46220_e59238_d_n7;
        locals.var_zg_dn8 = assign46220_e59238_d_n8;
        locals.var_zg_dn9 = assign46220_e59238_d_n9;

        let assign46230_e59241: f64 = if locals.var_gc3ov_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1240 = assign46230_e59241;

        let (assign46240_e59264, assign46240_e59264_d_n4, assign46240_e59264_d_n6, assign46240_e59264_d_n7, assign46240_e59264_d_n8, assign46240_e59264_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) && (locals.var_guard1240 != 0.0)) {
        let assign46240_e59250: f64 = (locals.var_zg + locals.var_gcqov);
        let assign46240_e59253: f64 = (locals.var_zg - locals.var_gcqov);
        let assign46240_e59256: f64 = (locals.var_zg - locals.var_gcqov);
        let assign46240_e59257: f64 = (assign46240_e59253 * assign46240_e59256);
        let assign46240_e59259: f64 = (assign46240_e59257 + 1e-6);
        let assign46240_e59260: f64 = (assign46240_e59259).sqrt();
        let assign46240_e59261: f64 = (assign46240_e59250 - assign46240_e59260);
        let assign46240_e59262: f64 = (0.5 * assign46240_e59261);
        (assign46240_e59262, (0.5 * (locals.var_zg_dn4 - (((locals.var_zg_dn4 * assign46240_e59256) + (assign46240_e59253 * locals.var_zg_dn4)) / (2.0 * assign46240_e59260)))), (0.5 * (locals.var_zg_dn6 - (((locals.var_zg_dn6 * assign46240_e59256) + (assign46240_e59253 * locals.var_zg_dn6)) / (2.0 * assign46240_e59260)))), (0.5 * (locals.var_zg_dn7 - (((locals.var_zg_dn7 * assign46240_e59256) + (assign46240_e59253 * locals.var_zg_dn7)) / (2.0 * assign46240_e59260)))), (0.5 * (locals.var_zg_dn8 - (((locals.var_zg_dn8 * assign46240_e59256) + (assign46240_e59253 * locals.var_zg_dn8)) / (2.0 * assign46240_e59260)))), (0.5 * (locals.var_zg_dn9 - (((locals.var_zg_dn9 * assign46240_e59256) + (assign46240_e59253 * locals.var_zg_dn9)) / (2.0 * assign46240_e59260)))),)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign46240_e59264;
        locals.var_zg_dn4 = assign46240_e59264_d_n4;
        locals.var_zg_dn6 = assign46240_e59264_d_n6;
        locals.var_zg_dn7 = assign46240_e59264_d_n7;
        locals.var_zg_dn8 = assign46240_e59264_d_n8;
        locals.var_zg_dn9 = assign46240_e59264_d_n9;

        let (assign46250_e59281, assign46250_e59281_d_n4, assign46250_e59281_d_n6, assign46250_e59281_d_n7, assign46250_e59281_d_n8, assign46250_e59281_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
        let assign46250_e59270: f64 = (-1.5);
        let assign46250_e59275: f64 = (locals.var_gc3ov_i * locals.var_zg);
        let assign46250_e59276: f64 = (locals.var_gc2ov_i + assign46250_e59275);
        let assign46250_e59277: f64 = (locals.var_zg * assign46250_e59276);
        let assign46250_e59278: f64 = (assign46250_e59270 + assign46250_e59277);
        let assign46250_e59279: f64 = (locals.var_bov * assign46250_e59278);
        (assign46250_e59279, (locals.var_bov * ((locals.var_zg_dn4 * assign46250_e59276) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn4)))), (locals.var_bov * ((locals.var_zg_dn6 * assign46250_e59276) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn6)))), (locals.var_bov * ((locals.var_zg_dn7 * assign46250_e59276) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn7)))), (locals.var_bov * ((locals.var_zg_dn8 * assign46250_e59276) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn8)))), (locals.var_bov * ((locals.var_zg_dn9 * assign46250_e59276) + (locals.var_zg * (locals.var_gc3ov_i * locals.var_zg_dn9)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46250_e59281;
        locals.var_temp__blk949_dn4 = assign46250_e59281_d_n4;
        locals.var_temp__blk949_dn6 = assign46250_e59281_d_n6;
        locals.var_temp__blk949_dn7 = assign46250_e59281_d_n7;
        locals.var_temp__blk949_dn8 = assign46250_e59281_d_n8;
        locals.var_temp__blk949_dn9 = assign46250_e59281_d_n9;

        let assign46260_e59284: f64 = if locals.var_temp__blk949 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1241 = assign46260_e59284;

    }

    pub(super) fn stamp_transient_block_29(
        locals: &mut StampLocals,
    ) {
        let (assign46270_e59306, assign46270_e59306_d_n4, assign46270_e59306_d_n6, assign46270_e59306_d_n7, assign46270_e59306_d_n8, assign46270_e59306_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) && (locals.var_guard1241 != 0.0)) {
        let assign46270_e59298: f64 = (locals.var_temp__blk949 * 0.3333333333333333);
        let assign46270_e59299: f64 = (1.0 + assign46270_e59298);
        let assign46270_e59300: f64 = (locals.var_temp__blk949 * assign46270_e59299);
        let assign46270_e59301: f64 = (0.5 * assign46270_e59300);
        let assign46270_e59302: f64 = (1.0 + assign46270_e59301);
        let assign46270_e59303: f64 = (locals.var_temp__blk949 * assign46270_e59302);
        let assign46270_e59304: f64 = (1.0 + assign46270_e59303);
        (assign46270_e59304, ((locals.var_temp__blk949_dn4 * assign46270_e59302) + (locals.var_temp__blk949 * (0.5 * ((locals.var_temp__blk949_dn4 * assign46270_e59299) + (locals.var_temp__blk949 * (locals.var_temp__blk949_dn4 * 0.3333333333333333)))))), ((locals.var_temp__blk949_dn6 * assign46270_e59302) + (locals.var_temp__blk949 * (0.5 * ((locals.var_temp__blk949_dn6 * assign46270_e59299) + (locals.var_temp__blk949 * (locals.var_temp__blk949_dn6 * 0.3333333333333333)))))), ((locals.var_temp__blk949_dn7 * assign46270_e59302) + (locals.var_temp__blk949 * (0.5 * ((locals.var_temp__blk949_dn7 * assign46270_e59299) + (locals.var_temp__blk949 * (locals.var_temp__blk949_dn7 * 0.3333333333333333)))))), ((locals.var_temp__blk949_dn8 * assign46270_e59302) + (locals.var_temp__blk949 * (0.5 * ((locals.var_temp__blk949_dn8 * assign46270_e59299) + (locals.var_temp__blk949 * (locals.var_temp__blk949_dn8 * 0.3333333333333333)))))), ((locals.var_temp__blk949_dn9 * assign46270_e59302) + (locals.var_temp__blk949 * (0.5 * ((locals.var_temp__blk949_dn9 * assign46270_e59299) + (locals.var_temp__blk949 * (locals.var_temp__blk949_dn9 * 0.3333333333333333)))))),)
    } else {
        (locals.var_tp, locals.var_tp_dn4, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9,)
    }
};
        locals.var_tp = assign46270_e59306;
        locals.var_tp_dn4 = assign46270_e59306_d_n4;
        locals.var_tp_dn6 = assign46270_e59306_d_n6;
        locals.var_tp_dn7 = assign46270_e59306_d_n7;
        locals.var_tp_dn8 = assign46270_e59306_d_n8;
        locals.var_tp_dn9 = assign46270_e59306_d_n9;

        let assign46280_e59309: f64 = (-230.25850929940458);
        let assign46280_e59310: f64 = if locals.var_temp__blk949 > assign46280_e59309 { 1.0 } else { 0.0 };
        locals.var_guard1242 = assign46280_e59310;

        let (assign46290_e59322, assign46290_e59322_d_n4, assign46290_e59322_d_n6, assign46290_e59322_d_n7, assign46290_e59322_d_n8, assign46290_e59322_d_n9,) = {
    if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) && (locals.var_guard1241 == 0.0)) && (locals.var_guard1242 != 0.0)) {
        let assign46290_e59320: f64 = (locals.var_temp__blk949).exp();
        (assign46290_e59320, (assign46290_e59320 * locals.var_temp__blk949_dn4), (assign46290_e59320 * locals.var_temp__blk949_dn6), (assign46290_e59320 * locals.var_temp__blk949_dn7), (assign46290_e59320 * locals.var_temp__blk949_dn8), (assign46290_e59320 * locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_tp, locals.var_tp_dn4, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9,)
    }
};
        locals.var_tp = assign46290_e59322;
        locals.var_tp_dn4 = assign46290_e59322_d_n4;
        locals.var_tp_dn6 = assign46290_e59322_d_n6;
        locals.var_tp_dn7 = assign46290_e59322_d_n7;
        locals.var_tp_dn8 = assign46290_e59322_d_n8;
        locals.var_tp_dn9 = assign46290_e59322_d_n9;

        let (assign46300_e59359, assign46300_e59359_d_n4, assign46300_e59359_d_n6, assign46300_e59359_d_n7, assign46300_e59359_d_n8, assign46300_e59359_d_n9,) = {
    if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) && (locals.var_guard1241 == 0.0)) && (locals.var_guard1242 == 0.0)) {
        let assign46300_e59335: f64 = (-230.25850929940458);
        let assign46300_e59337: f64 = (assign46300_e59335 - locals.var_temp__blk949);
        let assign46300_e59341: f64 = (-230.25850929940458);
        let assign46300_e59343: f64 = (assign46300_e59341 - locals.var_temp__blk949);
        let assign46300_e59346: f64 = (-230.25850929940458);
        let assign46300_e59348: f64 = (assign46300_e59346 - locals.var_temp__blk949);
        let assign46300_e59350: f64 = (assign46300_e59348 * 0.3333333333333333);
        let assign46300_e59351: f64 = (1.0 + assign46300_e59350);
        let assign46300_e59352: f64 = (assign46300_e59343 * assign46300_e59351);
        let assign46300_e59353: f64 = (0.5 * assign46300_e59352);
        let assign46300_e59354: f64 = (1.0 + assign46300_e59353);
        let assign46300_e59355: f64 = (assign46300_e59337 * assign46300_e59354);
        let assign46300_e59356: f64 = (1.0 + assign46300_e59355);
        let assign46300_e59357: f64 = (1e-100 / assign46300_e59356);
        (assign46300_e59357, (-((1e-100 * (((-locals.var_temp__blk949_dn4) * assign46300_e59354) + (assign46300_e59337 * (0.5 * (((-locals.var_temp__blk949_dn4) * assign46300_e59351) + (assign46300_e59343 * ((-locals.var_temp__blk949_dn4) * 0.3333333333333333))))))) / (assign46300_e59356 * assign46300_e59356))), (-((1e-100 * (((-locals.var_temp__blk949_dn6) * assign46300_e59354) + (assign46300_e59337 * (0.5 * (((-locals.var_temp__blk949_dn6) * assign46300_e59351) + (assign46300_e59343 * ((-locals.var_temp__blk949_dn6) * 0.3333333333333333))))))) / (assign46300_e59356 * assign46300_e59356))), (-((1e-100 * (((-locals.var_temp__blk949_dn7) * assign46300_e59354) + (assign46300_e59337 * (0.5 * (((-locals.var_temp__blk949_dn7) * assign46300_e59351) + (assign46300_e59343 * ((-locals.var_temp__blk949_dn7) * 0.3333333333333333))))))) / (assign46300_e59356 * assign46300_e59356))), (-((1e-100 * (((-locals.var_temp__blk949_dn8) * assign46300_e59354) + (assign46300_e59337 * (0.5 * (((-locals.var_temp__blk949_dn8) * assign46300_e59351) + (assign46300_e59343 * ((-locals.var_temp__blk949_dn8) * 0.3333333333333333))))))) / (assign46300_e59356 * assign46300_e59356))), (-((1e-100 * (((-locals.var_temp__blk949_dn9) * assign46300_e59354) + (assign46300_e59337 * (0.5 * (((-locals.var_temp__blk949_dn9) * assign46300_e59351) + (assign46300_e59343 * ((-locals.var_temp__blk949_dn9) * 0.3333333333333333))))))) / (assign46300_e59356 * assign46300_e59356))),)
    } else {
        (locals.var_tp, locals.var_tp_dn4, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9,)
    }
};
        locals.var_tp = assign46300_e59359;
        locals.var_tp_dn4 = assign46300_e59359_d_n4;
        locals.var_tp_dn6 = assign46300_e59359_d_n6;
        locals.var_tp_dn7 = assign46300_e59359_d_n7;
        locals.var_tp_dn8 = assign46300_e59359_d_n8;
        locals.var_tp_dn9 = assign46300_e59359_d_n9;

        let (assign46310_e59367, assign46310_e59367_d_n6, assign46310_e59367_d_n7, assign46310_e59367_d_n8,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
        let assign46310_e59365: f64 = (3.0 + locals.var_xs_ov);
        (assign46310_e59365, locals.var_xs_ov_dn6, locals.var_xs_ov_dn7, locals.var_xs_ov_dn8,)
    } else {
        (locals.var_fs1, locals.var_fs1_dn6, locals.var_fs1_dn7, locals.var_fs1_dn8,)
    }
};
        locals.var_fs1 = assign46310_e59367;
        locals.var_fs1_dn6 = assign46310_e59367_d_n6;
        locals.var_fs1_dn7 = assign46310_e59367_d_n7;
        locals.var_fs1_dn8 = assign46310_e59367_d_n8;

        let (assign46320_e59376,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
        let assign46320_e59372: f64 = (-3.0);
        let assign46320_e59374: f64 = (assign46320_e59372 - locals.var_gco_i);
        (assign46320_e59374,)
    } else {
        (locals.var_fs2,)
    }
};
        locals.var_fs2 = assign46320_e59376;

        let (assign46330_e59384, assign46330_e59384_d_n6, assign46330_e59384_d_n7, assign46330_e59384_d_n8,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
        let assign46330_e59382: f64 = (30.0 * locals.var_vgsprime);
        (assign46330_e59382, (30.0 * locals.var_vgsprime_dn6), (30.0 * locals.var_vgsprime_dn7), (30.0 * locals.var_vgsprime_dn8),)
    } else {
        (locals.var_fs3, locals.var_fs3_dn6, locals.var_fs3_dn7, locals.var_fs3_dn8,)
    }
};
        locals.var_fs3 = assign46330_e59384;
        locals.var_fs3_dn6 = assign46330_e59384_d_n6;
        locals.var_fs3_dn7 = assign46330_e59384_d_n7;
        locals.var_fs3_dn8 = assign46330_e59384_d_n8;

        let (assign46340_e59392,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
        let assign46340_e59390: f64 = (4.0 - 0.9);
        (assign46340_e59390,)
    } else {
        (locals.var_tme1,)
    }
};
        locals.var_tme1 = assign46340_e59392;

        let (assign46350_e59400, assign46350_e59400_d_n4, assign46350_e59400_d_n6, assign46350_e59400_d_n7, assign46350_e59400_d_n8, assign46350_e59400_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
        let assign46350_e59398: f64 = (locals.var_fs1 + locals.var_fs3);
        (assign46350_e59398, 0.0, (locals.var_fs1_dn6 + locals.var_fs3_dn6), (locals.var_fs1_dn7 + locals.var_fs3_dn7), (locals.var_fs1_dn8 + locals.var_fs3_dn8), 0.0,)
    } else {
        (locals.var_tme2, locals.var_tme2_dn4, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8, locals.var_tme2_dn9,)
    }
};
        locals.var_tme2 = assign46350_e59400;
        locals.var_tme2_dn4 = assign46350_e59400_d_n4;
        locals.var_tme2_dn6 = assign46350_e59400_d_n6;
        locals.var_tme2_dn7 = assign46350_e59400_d_n7;
        locals.var_tme2_dn8 = assign46350_e59400_d_n8;
        locals.var_tme2_dn9 = assign46350_e59400_d_n9;

        let (assign46360_e59421, assign46360_e59421_d_n4, assign46360_e59421_d_n6, assign46360_e59421_d_n7, assign46360_e59421_d_n8, assign46360_e59421_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
        let assign46360_e59406: f64 = (2.0 / locals.var_tme1);
        let assign46360_e59410: f64 = (locals.var_tme2 * locals.var_tme2);
        let assign46360_e59413: f64 = (locals.var_tme1 * locals.var_fs1);
        let assign46360_e59415: f64 = (assign46360_e59413 * locals.var_fs3);
        let assign46360_e59416: f64 = (assign46360_e59410 - assign46360_e59415);
        let assign46360_e59417: f64 = (assign46360_e59416).sqrt();
        let assign46360_e59418: f64 = (locals.var_tme2 - assign46360_e59417);
        let assign46360_e59419: f64 = (assign46360_e59406 * assign46360_e59418);
        (assign46360_e59419, (assign46360_e59406 * (locals.var_tme2_dn4 - (((locals.var_tme2_dn4 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn4)) / (2.0 * assign46360_e59417)))), (assign46360_e59406 * (locals.var_tme2_dn6 - ((((locals.var_tme2_dn6 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn6)) - (((locals.var_tme1 * locals.var_fs1_dn6) * locals.var_fs3) + (assign46360_e59413 * locals.var_fs3_dn6))) / (2.0 * assign46360_e59417)))), (assign46360_e59406 * (locals.var_tme2_dn7 - ((((locals.var_tme2_dn7 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn7)) - (((locals.var_tme1 * locals.var_fs1_dn7) * locals.var_fs3) + (assign46360_e59413 * locals.var_fs3_dn7))) / (2.0 * assign46360_e59417)))), (assign46360_e59406 * (locals.var_tme2_dn8 - ((((locals.var_tme2_dn8 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn8)) - (((locals.var_tme1 * locals.var_fs1_dn8) * locals.var_fs3) + (assign46360_e59413 * locals.var_fs3_dn8))) / (2.0 * assign46360_e59417)))), (assign46360_e59406 * (locals.var_tme2_dn9 - (((locals.var_tme2_dn9 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn9)) / (2.0 * assign46360_e59417)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46360_e59421;
        locals.var_temp__blk949_dn4 = assign46360_e59421_d_n4;
        locals.var_temp__blk949_dn6 = assign46360_e59421_d_n6;
        locals.var_temp__blk949_dn7 = assign46360_e59421_d_n7;
        locals.var_temp__blk949_dn8 = assign46360_e59421_d_n8;
        locals.var_temp__blk949_dn9 = assign46360_e59421_d_n9;

        let (assign46370_e59429,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
        let assign46370_e59427: f64 = (4.0 - 0.3);
        (assign46370_e59427,)
    } else {
        (locals.var_tme1,)
    }
};
        locals.var_tme1 = assign46370_e59429;

        let (assign46380_e59437, assign46380_e59437_d_n4, assign46380_e59437_d_n6, assign46380_e59437_d_n7, assign46380_e59437_d_n8, assign46380_e59437_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
        let assign46380_e59435: f64 = (locals.var_fs2 + locals.var_temp__blk949);
        (assign46380_e59435, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    } else {
        (locals.var_tme2, locals.var_tme2_dn4, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8, locals.var_tme2_dn9,)
    }
};
        locals.var_tme2 = assign46380_e59437;
        locals.var_tme2_dn4 = assign46380_e59437_d_n4;
        locals.var_tme2_dn6 = assign46380_e59437_d_n6;
        locals.var_tme2_dn7 = assign46380_e59437_d_n7;
        locals.var_tme2_dn8 = assign46380_e59437_d_n8;
        locals.var_tme2_dn9 = assign46380_e59437_d_n9;

        let (assign46390_e59458, assign46390_e59458_d_n4, assign46390_e59458_d_n6, assign46390_e59458_d_n7, assign46390_e59458_d_n8, assign46390_e59458_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
        let assign46390_e59443: f64 = (2.0 / locals.var_tme1);
        let assign46390_e59447: f64 = (locals.var_tme2 * locals.var_tme2);
        let assign46390_e59450: f64 = (locals.var_tme1 * locals.var_fs2);
        let assign46390_e59452: f64 = (assign46390_e59450 * locals.var_temp__blk949);
        let assign46390_e59453: f64 = (assign46390_e59447 - assign46390_e59452);
        let assign46390_e59454: f64 = (assign46390_e59453).sqrt();
        let assign46390_e59455: f64 = (locals.var_tme2 + assign46390_e59454);
        let assign46390_e59456: f64 = (assign46390_e59443 * assign46390_e59455);
        (assign46390_e59456, (assign46390_e59443 * (locals.var_tme2_dn4 + ((((locals.var_tme2_dn4 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn4)) - (assign46390_e59450 * locals.var_temp__blk949_dn4)) / (2.0 * assign46390_e59454)))), (assign46390_e59443 * (locals.var_tme2_dn6 + ((((locals.var_tme2_dn6 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn6)) - (assign46390_e59450 * locals.var_temp__blk949_dn6)) / (2.0 * assign46390_e59454)))), (assign46390_e59443 * (locals.var_tme2_dn7 + ((((locals.var_tme2_dn7 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn7)) - (assign46390_e59450 * locals.var_temp__blk949_dn7)) / (2.0 * assign46390_e59454)))), (assign46390_e59443 * (locals.var_tme2_dn8 + ((((locals.var_tme2_dn8 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn8)) - (assign46390_e59450 * locals.var_temp__blk949_dn8)) / (2.0 * assign46390_e59454)))), (assign46390_e59443 * (locals.var_tme2_dn9 + ((((locals.var_tme2_dn9 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn9)) - (assign46390_e59450 * locals.var_temp__blk949_dn9)) / (2.0 * assign46390_e59454)))),)
    } else {
        (locals.var_fs, locals.var_fs_dn4, locals.var_fs_dn6, locals.var_fs_dn7, locals.var_fs_dn8, locals.var_fs_dn9,)
    }
};
        locals.var_fs = assign46390_e59458;
        locals.var_fs_dn4 = assign46390_e59458_d_n4;
        locals.var_fs_dn6 = assign46390_e59458_d_n6;
        locals.var_fs_dn7 = assign46390_e59458_d_n7;
        locals.var_fs_dn8 = assign46390_e59458_d_n8;
        locals.var_fs_dn9 = assign46390_e59458_d_n9;

        let (assign46400_e59468, assign46400_e59468_d_n4, assign46400_e59468_d_n6, assign46400_e59468_d_n7, assign46400_e59468_d_n8, assign46400_e59468_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1239 != 0.0)) {
        let assign46400_e59465: f64 = (locals.var_tp * locals.var_fs);
        let assign46400_e59466: f64 = (locals.var_igov_i * assign46400_e59465);
        (assign46400_e59466, (locals.var_igov_i * ((locals.var_tp_dn4 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn4))), (locals.var_igov_i * ((locals.var_tp_dn6 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn6))), (locals.var_igov_i * ((locals.var_tp_dn7 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn7))), (locals.var_igov_i * ((locals.var_tp_dn8 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn8))), (locals.var_igov_i * ((locals.var_tp_dn9 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn9))),)
    } else {
        (locals.var_igsov, locals.var_igsov_dn4, locals.var_igsov_dn6, locals.var_igsov_dn7, locals.var_igsov_dn8, locals.var_igsov_dn9,)
    }
};
        locals.var_igsov = assign46400_e59468;
        locals.var_igsov_dn4 = assign46400_e59468_d_n4;
        locals.var_igsov_dn6 = assign46400_e59468_d_n6;
        locals.var_igsov_dn7 = assign46400_e59468_d_n7;
        locals.var_igsov_dn8 = assign46400_e59468_d_n8;
        locals.var_igsov_dn9 = assign46400_e59468_d_n9;

        let assign46410_e59471: f64 = if locals.var_igovd_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1243 = assign46410_e59471;

        let (assign46420_e59484, assign46420_e59484_d_n4, assign46420_e59484_d_n6, assign46420_e59484_d_n7, assign46420_e59484_d_n8, assign46420_e59484_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
        let assign46420_e59477: f64 = (locals.var_vovd * locals.var_vovd);
        let assign46420_e59479: f64 = (assign46420_e59477 + 1e-6);
        let assign46420_e59480: f64 = (assign46420_e59479).sqrt();
        let assign46420_e59482: f64 = (assign46420_e59480 * locals.var_inv_chib);
        (assign46420_e59482, 0.0, ((((locals.var_vovd_dn6 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn6)) / (2.0 * assign46420_e59480)) * locals.var_inv_chib), ((((locals.var_vovd_dn7 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn7)) / (2.0 * assign46420_e59480)) * locals.var_inv_chib), ((((locals.var_vovd_dn8 * locals.var_vovd) + (locals.var_vovd * locals.var_vovd_dn8)) / (2.0 * assign46420_e59480)) * locals.var_inv_chib), 0.0,)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign46420_e59484;
        locals.var_zg_dn4 = assign46420_e59484_d_n4;
        locals.var_zg_dn6 = assign46420_e59484_d_n6;
        locals.var_zg_dn7 = assign46420_e59484_d_n7;
        locals.var_zg_dn8 = assign46420_e59484_d_n8;
        locals.var_zg_dn9 = assign46420_e59484_d_n9;

        let assign46430_e59487: f64 = if locals.var_gc3ovd_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1244 = assign46430_e59487;

        let (assign46440_e59510, assign46440_e59510_d_n4, assign46440_e59510_d_n6, assign46440_e59510_d_n7, assign46440_e59510_d_n8, assign46440_e59510_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) && (locals.var_guard1244 != 0.0)) {
        let assign46440_e59496: f64 = (locals.var_zg + locals.var_gcqovd);
        let assign46440_e59499: f64 = (locals.var_zg - locals.var_gcqovd);
        let assign46440_e59502: f64 = (locals.var_zg - locals.var_gcqovd);
        let assign46440_e59503: f64 = (assign46440_e59499 * assign46440_e59502);
        let assign46440_e59505: f64 = (assign46440_e59503 + 1e-6);
        let assign46440_e59506: f64 = (assign46440_e59505).sqrt();
        let assign46440_e59507: f64 = (assign46440_e59496 - assign46440_e59506);
        let assign46440_e59508: f64 = (0.5 * assign46440_e59507);
        (assign46440_e59508, (0.5 * (locals.var_zg_dn4 - (((locals.var_zg_dn4 * assign46440_e59502) + (assign46440_e59499 * locals.var_zg_dn4)) / (2.0 * assign46440_e59506)))), (0.5 * (locals.var_zg_dn6 - (((locals.var_zg_dn6 * assign46440_e59502) + (assign46440_e59499 * locals.var_zg_dn6)) / (2.0 * assign46440_e59506)))), (0.5 * (locals.var_zg_dn7 - (((locals.var_zg_dn7 * assign46440_e59502) + (assign46440_e59499 * locals.var_zg_dn7)) / (2.0 * assign46440_e59506)))), (0.5 * (locals.var_zg_dn8 - (((locals.var_zg_dn8 * assign46440_e59502) + (assign46440_e59499 * locals.var_zg_dn8)) / (2.0 * assign46440_e59506)))), (0.5 * (locals.var_zg_dn9 - (((locals.var_zg_dn9 * assign46440_e59502) + (assign46440_e59499 * locals.var_zg_dn9)) / (2.0 * assign46440_e59506)))),)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign46440_e59510;
        locals.var_zg_dn4 = assign46440_e59510_d_n4;
        locals.var_zg_dn6 = assign46440_e59510_d_n6;
        locals.var_zg_dn7 = assign46440_e59510_d_n7;
        locals.var_zg_dn8 = assign46440_e59510_d_n8;
        locals.var_zg_dn9 = assign46440_e59510_d_n9;

        let (assign46450_e59527, assign46450_e59527_d_n4, assign46450_e59527_d_n6, assign46450_e59527_d_n7, assign46450_e59527_d_n8, assign46450_e59527_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
        let assign46450_e59516: f64 = (-1.5);
        let assign46450_e59521: f64 = (locals.var_gc3ovd_i * locals.var_zg);
        let assign46450_e59522: f64 = (locals.var_gc2ovd_i + assign46450_e59521);
        let assign46450_e59523: f64 = (locals.var_zg * assign46450_e59522);
        let assign46450_e59524: f64 = (assign46450_e59516 + assign46450_e59523);
        let assign46450_e59525: f64 = (locals.var_bov_d * assign46450_e59524);
        (assign46450_e59525, (locals.var_bov_d * ((locals.var_zg_dn4 * assign46450_e59522) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn4)))), (locals.var_bov_d * ((locals.var_zg_dn6 * assign46450_e59522) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn6)))), (locals.var_bov_d * ((locals.var_zg_dn7 * assign46450_e59522) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn7)))), (locals.var_bov_d * ((locals.var_zg_dn8 * assign46450_e59522) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn8)))), (locals.var_bov_d * ((locals.var_zg_dn9 * assign46450_e59522) + (locals.var_zg * (locals.var_gc3ovd_i * locals.var_zg_dn9)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46450_e59527;
        locals.var_temp__blk949_dn4 = assign46450_e59527_d_n4;
        locals.var_temp__blk949_dn6 = assign46450_e59527_d_n6;
        locals.var_temp__blk949_dn7 = assign46450_e59527_d_n7;
        locals.var_temp__blk949_dn8 = assign46450_e59527_d_n8;
        locals.var_temp__blk949_dn9 = assign46450_e59527_d_n9;

        let assign46460_e59530: f64 = if locals.var_temp__blk949 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1245 = assign46460_e59530;

        let (assign46470_e59552, assign46470_e59552_d_n4, assign46470_e59552_d_n6, assign46470_e59552_d_n7, assign46470_e59552_d_n8, assign46470_e59552_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) && (locals.var_guard1245 != 0.0)) {
        let assign46470_e59544: f64 = (locals.var_temp__blk949 * 0.3333333333333333);
        let assign46470_e59545: f64 = (1.0 + assign46470_e59544);
        let assign46470_e59546: f64 = (locals.var_temp__blk949 * assign46470_e59545);
        let assign46470_e59547: f64 = (0.5 * assign46470_e59546);
        let assign46470_e59548: f64 = (1.0 + assign46470_e59547);
        let assign46470_e59549: f64 = (locals.var_temp__blk949 * assign46470_e59548);
        let assign46470_e59550: f64 = (1.0 + assign46470_e59549);
        (assign46470_e59550, ((locals.var_temp__blk949_dn4 * assign46470_e59548) + (locals.var_temp__blk949 * (0.5 * ((locals.var_temp__blk949_dn4 * assign46470_e59545) + (locals.var_temp__blk949 * (locals.var_temp__blk949_dn4 * 0.3333333333333333)))))), ((locals.var_temp__blk949_dn6 * assign46470_e59548) + (locals.var_temp__blk949 * (0.5 * ((locals.var_temp__blk949_dn6 * assign46470_e59545) + (locals.var_temp__blk949 * (locals.var_temp__blk949_dn6 * 0.3333333333333333)))))), ((locals.var_temp__blk949_dn7 * assign46470_e59548) + (locals.var_temp__blk949 * (0.5 * ((locals.var_temp__blk949_dn7 * assign46470_e59545) + (locals.var_temp__blk949 * (locals.var_temp__blk949_dn7 * 0.3333333333333333)))))), ((locals.var_temp__blk949_dn8 * assign46470_e59548) + (locals.var_temp__blk949 * (0.5 * ((locals.var_temp__blk949_dn8 * assign46470_e59545) + (locals.var_temp__blk949 * (locals.var_temp__blk949_dn8 * 0.3333333333333333)))))), ((locals.var_temp__blk949_dn9 * assign46470_e59548) + (locals.var_temp__blk949 * (0.5 * ((locals.var_temp__blk949_dn9 * assign46470_e59545) + (locals.var_temp__blk949 * (locals.var_temp__blk949_dn9 * 0.3333333333333333)))))),)
    } else {
        (locals.var_tp, locals.var_tp_dn4, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9,)
    }
};
        locals.var_tp = assign46470_e59552;
        locals.var_tp_dn4 = assign46470_e59552_d_n4;
        locals.var_tp_dn6 = assign46470_e59552_d_n6;
        locals.var_tp_dn7 = assign46470_e59552_d_n7;
        locals.var_tp_dn8 = assign46470_e59552_d_n8;
        locals.var_tp_dn9 = assign46470_e59552_d_n9;

        let assign46480_e59555: f64 = (-230.25850929940458);
        let assign46480_e59556: f64 = if locals.var_temp__blk949 > assign46480_e59555 { 1.0 } else { 0.0 };
        locals.var_guard1246 = assign46480_e59556;

        let (assign46490_e59568, assign46490_e59568_d_n4, assign46490_e59568_d_n6, assign46490_e59568_d_n7, assign46490_e59568_d_n8, assign46490_e59568_d_n9,) = {
    if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) && (locals.var_guard1245 == 0.0)) && (locals.var_guard1246 != 0.0)) {
        let assign46490_e59566: f64 = (locals.var_temp__blk949).exp();
        (assign46490_e59566, (assign46490_e59566 * locals.var_temp__blk949_dn4), (assign46490_e59566 * locals.var_temp__blk949_dn6), (assign46490_e59566 * locals.var_temp__blk949_dn7), (assign46490_e59566 * locals.var_temp__blk949_dn8), (assign46490_e59566 * locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_tp, locals.var_tp_dn4, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9,)
    }
};
        locals.var_tp = assign46490_e59568;
        locals.var_tp_dn4 = assign46490_e59568_d_n4;
        locals.var_tp_dn6 = assign46490_e59568_d_n6;
        locals.var_tp_dn7 = assign46490_e59568_d_n7;
        locals.var_tp_dn8 = assign46490_e59568_d_n8;
        locals.var_tp_dn9 = assign46490_e59568_d_n9;

        let (assign46500_e59605, assign46500_e59605_d_n4, assign46500_e59605_d_n6, assign46500_e59605_d_n7, assign46500_e59605_d_n8, assign46500_e59605_d_n9,) = {
    if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) && (locals.var_guard1245 == 0.0)) && (locals.var_guard1246 == 0.0)) {
        let assign46500_e59581: f64 = (-230.25850929940458);
        let assign46500_e59583: f64 = (assign46500_e59581 - locals.var_temp__blk949);
        let assign46500_e59587: f64 = (-230.25850929940458);
        let assign46500_e59589: f64 = (assign46500_e59587 - locals.var_temp__blk949);
        let assign46500_e59592: f64 = (-230.25850929940458);
        let assign46500_e59594: f64 = (assign46500_e59592 - locals.var_temp__blk949);
        let assign46500_e59596: f64 = (assign46500_e59594 * 0.3333333333333333);
        let assign46500_e59597: f64 = (1.0 + assign46500_e59596);
        let assign46500_e59598: f64 = (assign46500_e59589 * assign46500_e59597);
        let assign46500_e59599: f64 = (0.5 * assign46500_e59598);
        let assign46500_e59600: f64 = (1.0 + assign46500_e59599);
        let assign46500_e59601: f64 = (assign46500_e59583 * assign46500_e59600);
        let assign46500_e59602: f64 = (1.0 + assign46500_e59601);
        let assign46500_e59603: f64 = (1e-100 / assign46500_e59602);
        (assign46500_e59603, (-((1e-100 * (((-locals.var_temp__blk949_dn4) * assign46500_e59600) + (assign46500_e59583 * (0.5 * (((-locals.var_temp__blk949_dn4) * assign46500_e59597) + (assign46500_e59589 * ((-locals.var_temp__blk949_dn4) * 0.3333333333333333))))))) / (assign46500_e59602 * assign46500_e59602))), (-((1e-100 * (((-locals.var_temp__blk949_dn6) * assign46500_e59600) + (assign46500_e59583 * (0.5 * (((-locals.var_temp__blk949_dn6) * assign46500_e59597) + (assign46500_e59589 * ((-locals.var_temp__blk949_dn6) * 0.3333333333333333))))))) / (assign46500_e59602 * assign46500_e59602))), (-((1e-100 * (((-locals.var_temp__blk949_dn7) * assign46500_e59600) + (assign46500_e59583 * (0.5 * (((-locals.var_temp__blk949_dn7) * assign46500_e59597) + (assign46500_e59589 * ((-locals.var_temp__blk949_dn7) * 0.3333333333333333))))))) / (assign46500_e59602 * assign46500_e59602))), (-((1e-100 * (((-locals.var_temp__blk949_dn8) * assign46500_e59600) + (assign46500_e59583 * (0.5 * (((-locals.var_temp__blk949_dn8) * assign46500_e59597) + (assign46500_e59589 * ((-locals.var_temp__blk949_dn8) * 0.3333333333333333))))))) / (assign46500_e59602 * assign46500_e59602))), (-((1e-100 * (((-locals.var_temp__blk949_dn9) * assign46500_e59600) + (assign46500_e59583 * (0.5 * (((-locals.var_temp__blk949_dn9) * assign46500_e59597) + (assign46500_e59589 * ((-locals.var_temp__blk949_dn9) * 0.3333333333333333))))))) / (assign46500_e59602 * assign46500_e59602))),)
    } else {
        (locals.var_tp, locals.var_tp_dn4, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9,)
    }
};
        locals.var_tp = assign46500_e59605;
        locals.var_tp_dn4 = assign46500_e59605_d_n4;
        locals.var_tp_dn6 = assign46500_e59605_d_n6;
        locals.var_tp_dn7 = assign46500_e59605_d_n7;
        locals.var_tp_dn8 = assign46500_e59605_d_n8;
        locals.var_tp_dn9 = assign46500_e59605_d_n9;

        let (assign46510_e59613, assign46510_e59613_d_n6, assign46510_e59613_d_n7, assign46510_e59613_d_n8,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
        let assign46510_e59611: f64 = (3.0 + locals.var_xd_ov);
        (assign46510_e59611, locals.var_xd_ov_dn6, locals.var_xd_ov_dn7, locals.var_xd_ov_dn8,)
    } else {
        (locals.var_fs1, locals.var_fs1_dn6, locals.var_fs1_dn7, locals.var_fs1_dn8,)
    }
};
        locals.var_fs1 = assign46510_e59613;
        locals.var_fs1_dn6 = assign46510_e59613_d_n6;
        locals.var_fs1_dn7 = assign46510_e59613_d_n7;
        locals.var_fs1_dn8 = assign46510_e59613_d_n8;

        let (assign46520_e59622,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
        let assign46520_e59618: f64 = (-3.0);
        let assign46520_e59620: f64 = (assign46520_e59618 - locals.var_gco_i);
        (assign46520_e59620,)
    } else {
        (locals.var_fs2,)
    }
};
        locals.var_fs2 = assign46520_e59622;

        let (assign46530_e59630, assign46530_e59630_d_n6, assign46530_e59630_d_n7, assign46530_e59630_d_n8,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
        let assign46530_e59628: f64 = (30.0 * locals.var_vgdprime);
        (assign46530_e59628, (30.0 * locals.var_vgdprime_dn6), (30.0 * locals.var_vgdprime_dn7), (30.0 * locals.var_vgdprime_dn8),)
    } else {
        (locals.var_fs3, locals.var_fs3_dn6, locals.var_fs3_dn7, locals.var_fs3_dn8,)
    }
};
        locals.var_fs3 = assign46530_e59630;
        locals.var_fs3_dn6 = assign46530_e59630_d_n6;
        locals.var_fs3_dn7 = assign46530_e59630_d_n7;
        locals.var_fs3_dn8 = assign46530_e59630_d_n8;

        let (assign46540_e59638,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
        let assign46540_e59636: f64 = (4.0 - 0.9);
        (assign46540_e59636,)
    } else {
        (locals.var_tme1,)
    }
};
        locals.var_tme1 = assign46540_e59638;

        let (assign46550_e59646, assign46550_e59646_d_n4, assign46550_e59646_d_n6, assign46550_e59646_d_n7, assign46550_e59646_d_n8, assign46550_e59646_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
        let assign46550_e59644: f64 = (locals.var_fs1 + locals.var_fs3);
        (assign46550_e59644, 0.0, (locals.var_fs1_dn6 + locals.var_fs3_dn6), (locals.var_fs1_dn7 + locals.var_fs3_dn7), (locals.var_fs1_dn8 + locals.var_fs3_dn8), 0.0,)
    } else {
        (locals.var_tme2, locals.var_tme2_dn4, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8, locals.var_tme2_dn9,)
    }
};
        locals.var_tme2 = assign46550_e59646;
        locals.var_tme2_dn4 = assign46550_e59646_d_n4;
        locals.var_tme2_dn6 = assign46550_e59646_d_n6;
        locals.var_tme2_dn7 = assign46550_e59646_d_n7;
        locals.var_tme2_dn8 = assign46550_e59646_d_n8;
        locals.var_tme2_dn9 = assign46550_e59646_d_n9;

        let (assign46560_e59667, assign46560_e59667_d_n4, assign46560_e59667_d_n6, assign46560_e59667_d_n7, assign46560_e59667_d_n8, assign46560_e59667_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
        let assign46560_e59652: f64 = (2.0 / locals.var_tme1);
        let assign46560_e59656: f64 = (locals.var_tme2 * locals.var_tme2);
        let assign46560_e59659: f64 = (locals.var_tme1 * locals.var_fs1);
        let assign46560_e59661: f64 = (assign46560_e59659 * locals.var_fs3);
        let assign46560_e59662: f64 = (assign46560_e59656 - assign46560_e59661);
        let assign46560_e59663: f64 = (assign46560_e59662).sqrt();
        let assign46560_e59664: f64 = (locals.var_tme2 - assign46560_e59663);
        let assign46560_e59665: f64 = (assign46560_e59652 * assign46560_e59664);
        (assign46560_e59665, (assign46560_e59652 * (locals.var_tme2_dn4 - (((locals.var_tme2_dn4 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn4)) / (2.0 * assign46560_e59663)))), (assign46560_e59652 * (locals.var_tme2_dn6 - ((((locals.var_tme2_dn6 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn6)) - (((locals.var_tme1 * locals.var_fs1_dn6) * locals.var_fs3) + (assign46560_e59659 * locals.var_fs3_dn6))) / (2.0 * assign46560_e59663)))), (assign46560_e59652 * (locals.var_tme2_dn7 - ((((locals.var_tme2_dn7 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn7)) - (((locals.var_tme1 * locals.var_fs1_dn7) * locals.var_fs3) + (assign46560_e59659 * locals.var_fs3_dn7))) / (2.0 * assign46560_e59663)))), (assign46560_e59652 * (locals.var_tme2_dn8 - ((((locals.var_tme2_dn8 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn8)) - (((locals.var_tme1 * locals.var_fs1_dn8) * locals.var_fs3) + (assign46560_e59659 * locals.var_fs3_dn8))) / (2.0 * assign46560_e59663)))), (assign46560_e59652 * (locals.var_tme2_dn9 - (((locals.var_tme2_dn9 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn9)) / (2.0 * assign46560_e59663)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46560_e59667;
        locals.var_temp__blk949_dn4 = assign46560_e59667_d_n4;
        locals.var_temp__blk949_dn6 = assign46560_e59667_d_n6;
        locals.var_temp__blk949_dn7 = assign46560_e59667_d_n7;
        locals.var_temp__blk949_dn8 = assign46560_e59667_d_n8;
        locals.var_temp__blk949_dn9 = assign46560_e59667_d_n9;

        let (assign46570_e59675,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
        let assign46570_e59673: f64 = (4.0 - 0.3);
        (assign46570_e59673,)
    } else {
        (locals.var_tme1,)
    }
};
        locals.var_tme1 = assign46570_e59675;

        let (assign46580_e59683, assign46580_e59683_d_n4, assign46580_e59683_d_n6, assign46580_e59683_d_n7, assign46580_e59683_d_n8, assign46580_e59683_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
        let assign46580_e59681: f64 = (locals.var_fs2 + locals.var_temp__blk949);
        (assign46580_e59681, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    } else {
        (locals.var_tme2, locals.var_tme2_dn4, locals.var_tme2_dn6, locals.var_tme2_dn7, locals.var_tme2_dn8, locals.var_tme2_dn9,)
    }
};
        locals.var_tme2 = assign46580_e59683;
        locals.var_tme2_dn4 = assign46580_e59683_d_n4;
        locals.var_tme2_dn6 = assign46580_e59683_d_n6;
        locals.var_tme2_dn7 = assign46580_e59683_d_n7;
        locals.var_tme2_dn8 = assign46580_e59683_d_n8;
        locals.var_tme2_dn9 = assign46580_e59683_d_n9;

        let (assign46590_e59704, assign46590_e59704_d_n4, assign46590_e59704_d_n6, assign46590_e59704_d_n7, assign46590_e59704_d_n8, assign46590_e59704_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
        let assign46590_e59689: f64 = (2.0 / locals.var_tme1);
        let assign46590_e59693: f64 = (locals.var_tme2 * locals.var_tme2);
        let assign46590_e59696: f64 = (locals.var_tme1 * locals.var_fs2);
        let assign46590_e59698: f64 = (assign46590_e59696 * locals.var_temp__blk949);
        let assign46590_e59699: f64 = (assign46590_e59693 - assign46590_e59698);
        let assign46590_e59700: f64 = (assign46590_e59699).sqrt();
        let assign46590_e59701: f64 = (locals.var_tme2 + assign46590_e59700);
        let assign46590_e59702: f64 = (assign46590_e59689 * assign46590_e59701);
        (assign46590_e59702, (assign46590_e59689 * (locals.var_tme2_dn4 + ((((locals.var_tme2_dn4 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn4)) - (assign46590_e59696 * locals.var_temp__blk949_dn4)) / (2.0 * assign46590_e59700)))), (assign46590_e59689 * (locals.var_tme2_dn6 + ((((locals.var_tme2_dn6 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn6)) - (assign46590_e59696 * locals.var_temp__blk949_dn6)) / (2.0 * assign46590_e59700)))), (assign46590_e59689 * (locals.var_tme2_dn7 + ((((locals.var_tme2_dn7 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn7)) - (assign46590_e59696 * locals.var_temp__blk949_dn7)) / (2.0 * assign46590_e59700)))), (assign46590_e59689 * (locals.var_tme2_dn8 + ((((locals.var_tme2_dn8 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn8)) - (assign46590_e59696 * locals.var_temp__blk949_dn8)) / (2.0 * assign46590_e59700)))), (assign46590_e59689 * (locals.var_tme2_dn9 + ((((locals.var_tme2_dn9 * locals.var_tme2) + (locals.var_tme2 * locals.var_tme2_dn9)) - (assign46590_e59696 * locals.var_temp__blk949_dn9)) / (2.0 * assign46590_e59700)))),)
    } else {
        (locals.var_fs, locals.var_fs_dn4, locals.var_fs_dn6, locals.var_fs_dn7, locals.var_fs_dn8, locals.var_fs_dn9,)
    }
};
        locals.var_fs = assign46590_e59704;
        locals.var_fs_dn4 = assign46590_e59704_d_n4;
        locals.var_fs_dn6 = assign46590_e59704_d_n6;
        locals.var_fs_dn7 = assign46590_e59704_d_n7;
        locals.var_fs_dn8 = assign46590_e59704_d_n8;
        locals.var_fs_dn9 = assign46590_e59704_d_n9;

        let (assign46600_e59714, assign46600_e59714_d_n4, assign46600_e59714_d_n6, assign46600_e59714_d_n7, assign46600_e59714_d_n8, assign46600_e59714_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1243 != 0.0)) {
        let assign46600_e59711: f64 = (locals.var_tp * locals.var_fs);
        let assign46600_e59712: f64 = (locals.var_igovd_i * assign46600_e59711);
        (assign46600_e59712, (locals.var_igovd_i * ((locals.var_tp_dn4 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn4))), (locals.var_igovd_i * ((locals.var_tp_dn6 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn6))), (locals.var_igovd_i * ((locals.var_tp_dn7 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn7))), (locals.var_igovd_i * ((locals.var_tp_dn8 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn8))), (locals.var_igovd_i * ((locals.var_tp_dn9 * locals.var_fs) + (locals.var_tp * locals.var_fs_dn9))),)
    } else {
        (locals.var_igdov, locals.var_igdov_dn4, locals.var_igdov_dn6, locals.var_igdov_dn7, locals.var_igdov_dn8, locals.var_igdov_dn9,)
    }
};
        locals.var_igdov = assign46600_e59714;
        locals.var_igdov_dn4 = assign46600_e59714_d_n4;
        locals.var_igdov_dn6 = assign46600_e59714_d_n6;
        locals.var_igdov_dn7 = assign46600_e59714_d_n7;
        locals.var_igdov_dn8 = assign46600_e59714_d_n8;
        locals.var_igdov_dn9 = assign46600_e59714_d_n9;

        let assign46610_e59717: f64 = if locals.var_iginv_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1247 = assign46610_e59717;

        let assign46620_e59720: f64 = if locals.var_xg_dc <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1248 = assign46620_e59720;

    }

    pub(super) fn stamp_transient_block_30(
        locals: &mut StampLocals,
    ) {
        let (assign46630_e59730, assign46630_e59730_d_n4, assign46630_e59730_d_n6, assign46630_e59730_d_n7, assign46630_e59730_d_n8, assign46630_e59730_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1248 != 0.0)) {
        let assign46630_e59728: f64 = (1.0 + locals.var_ar);
        (assign46630_e59728, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46630_e59730;
        locals.var_temp__blk949_dn4 = assign46630_e59730_d_n4;
        locals.var_temp__blk949_dn6 = assign46630_e59730_d_n6;
        locals.var_temp__blk949_dn7 = assign46630_e59730_d_n7;
        locals.var_temp__blk949_dn8 = assign46630_e59730_d_n8;
        locals.var_temp__blk949_dn9 = assign46630_e59730_d_n9;

        let (assign46640_e59743, assign46640_e59743_d_n4, assign46640_e59743_d_n6, assign46640_e59743_d_n7, assign46640_e59743_d_n8, assign46640_e59743_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1248 != 0.0)) {
        let assign46640_e59737: f64 = (locals.var_temp__blk949).sqrt();
        let assign46640_e59739: f64 = (assign46640_e59737 * locals.var_v_ds);
        let assign46640_e59741: f64 = (assign46640_e59739 / locals.var_vdsat_lim_dc);
        (assign46640_e59741, (((((locals.var_temp__blk949_dn4 / (2.0 * assign46640_e59737)) * locals.var_v_ds) * locals.var_vdsat_lim_dc) - (assign46640_e59739 * locals.var_vdsat_lim_dc_dn4)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), (((((locals.var_temp__blk949_dn6 / (2.0 * assign46640_e59737)) * locals.var_v_ds) * locals.var_vdsat_lim_dc) - (assign46640_e59739 * locals.var_vdsat_lim_dc_dn6)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), ((((((locals.var_temp__blk949_dn7 / (2.0 * assign46640_e59737)) * locals.var_v_ds) + (assign46640_e59737 * locals.var_v_ds_dn7)) * locals.var_vdsat_lim_dc) - (assign46640_e59739 * locals.var_vdsat_lim_dc_dn7)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), ((((((locals.var_temp__blk949_dn8 / (2.0 * assign46640_e59737)) * locals.var_v_ds) + (assign46640_e59737 * locals.var_v_ds_dn8)) * locals.var_vdsat_lim_dc) - (assign46640_e59739 * locals.var_vdsat_lim_dc_dn8)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)), (((((locals.var_temp__blk949_dn9 / (2.0 * assign46640_e59737)) * locals.var_v_ds) * locals.var_vdsat_lim_dc) - (assign46640_e59739 * locals.var_vdsat_lim_dc_dn9)) / (locals.var_vdsat_lim_dc * locals.var_vdsat_lim_dc)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign46640_e59743;
        locals.var_temp1_dn4 = assign46640_e59743_d_n4;
        locals.var_temp1_dn6 = assign46640_e59743_d_n6;
        locals.var_temp1_dn7 = assign46640_e59743_d_n7;
        locals.var_temp1_dn8 = assign46640_e59743_d_n8;
        locals.var_temp1_dn9 = assign46640_e59743_d_n9;

        let (assign46650_e59755, assign46650_e59755_d_n4, assign46650_e59755_d_n6, assign46650_e59755_d_n7, assign46650_e59755_d_n8, assign46650_e59755_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1248 != 0.0)) {
        let assign46650_e59751: f64 = (locals.var_temp1 * locals.var_temp1);
        let assign46650_e59753: f64 = (assign46650_e59751 + locals.var_temp__blk949);
        (assign46650_e59753, (((locals.var_temp1_dn4 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn4)) + locals.var_temp__blk949_dn4), (((locals.var_temp1_dn6 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn6)) + locals.var_temp__blk949_dn6), (((locals.var_temp1_dn7 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn7)) + locals.var_temp__blk949_dn7), (((locals.var_temp1_dn8 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn8)) + locals.var_temp__blk949_dn8), (((locals.var_temp1_dn9 * locals.var_temp1) + (locals.var_temp1 * locals.var_temp1_dn9)) + locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign46650_e59755;
        locals.var_temp2_dn4 = assign46650_e59755_d_n4;
        locals.var_temp2_dn6 = assign46650_e59755_d_n6;
        locals.var_temp2_dn7 = assign46650_e59755_d_n7;
        locals.var_temp2_dn8 = assign46650_e59755_d_n8;
        locals.var_temp2_dn9 = assign46650_e59755_d_n9;

        let (assign46660_e59765, assign46660_e59765_d_n4, assign46660_e59765_d_n6, assign46660_e59765_d_n7, assign46660_e59765_d_n8, assign46660_e59765_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1248 != 0.0)) {
        let assign46660_e59763: f64 = (2.0 * locals.var_temp1);
        (assign46660_e59763, (2.0 * locals.var_temp1_dn4), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8), (2.0 * locals.var_temp1_dn9),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46660_e59765;
        locals.var_temp__blk949_dn4 = assign46660_e59765_d_n4;
        locals.var_temp__blk949_dn6 = assign46660_e59765_d_n6;
        locals.var_temp__blk949_dn7 = assign46660_e59765_d_n7;
        locals.var_temp__blk949_dn8 = assign46660_e59765_d_n8;
        locals.var_temp__blk949_dn9 = assign46660_e59765_d_n9;

        let (assign46670_e59787, assign46670_e59787_d_n4, assign46670_e59787_d_n6, assign46670_e59787_d_n7, assign46670_e59787_d_n8, assign46670_e59787_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1248 != 0.0)) {
        let assign46670_e59773: f64 = (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc);
        let assign46670_e59775: f64 = (assign46670_e59773 * locals.var_temp__blk949);
        let assign46670_e59778: f64 = (locals.var_temp2 - locals.var_temp__blk949);
        let assign46670_e59779: f64 = (assign46670_e59778).sqrt();
        let assign46670_e59782: f64 = (locals.var_temp2 + locals.var_temp__blk949);
        let assign46670_e59783: f64 = (assign46670_e59782).sqrt();
        let assign46670_e59784: f64 = (assign46670_e59779 + assign46670_e59783);
        let assign46670_e59785: f64 = (assign46670_e59775 / assign46670_e59784);
        (assign46670_e59785, (((((((locals.var_vdsat_lim_dc_dn4 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn4)) * locals.var_temp__blk949) + (assign46670_e59773 * locals.var_temp__blk949_dn4)) * assign46670_e59784) - (assign46670_e59775 * (((locals.var_temp2_dn4 - locals.var_temp__blk949_dn4) / (2.0 * assign46670_e59779)) + ((locals.var_temp2_dn4 + locals.var_temp__blk949_dn4) / (2.0 * assign46670_e59783))))) / (assign46670_e59784 * assign46670_e59784)), (((((((locals.var_vdsat_lim_dc_dn6 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn6)) * locals.var_temp__blk949) + (assign46670_e59773 * locals.var_temp__blk949_dn6)) * assign46670_e59784) - (assign46670_e59775 * (((locals.var_temp2_dn6 - locals.var_temp__blk949_dn6) / (2.0 * assign46670_e59779)) + ((locals.var_temp2_dn6 + locals.var_temp__blk949_dn6) / (2.0 * assign46670_e59783))))) / (assign46670_e59784 * assign46670_e59784)), (((((((locals.var_vdsat_lim_dc_dn7 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn7)) * locals.var_temp__blk949) + (assign46670_e59773 * locals.var_temp__blk949_dn7)) * assign46670_e59784) - (assign46670_e59775 * (((locals.var_temp2_dn7 - locals.var_temp__blk949_dn7) / (2.0 * assign46670_e59779)) + ((locals.var_temp2_dn7 + locals.var_temp__blk949_dn7) / (2.0 * assign46670_e59783))))) / (assign46670_e59784 * assign46670_e59784)), (((((((locals.var_vdsat_lim_dc_dn8 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn8)) * locals.var_temp__blk949) + (assign46670_e59773 * locals.var_temp__blk949_dn8)) * assign46670_e59784) - (assign46670_e59775 * (((locals.var_temp2_dn8 - locals.var_temp__blk949_dn8) / (2.0 * assign46670_e59779)) + ((locals.var_temp2_dn8 + locals.var_temp__blk949_dn8) / (2.0 * assign46670_e59783))))) / (assign46670_e59784 * assign46670_e59784)), (((((((locals.var_vdsat_lim_dc_dn9 * locals.var_inv_phit1_dc) + (locals.var_vdsat_lim_dc * locals.var_inv_phit1_dc_dn9)) * locals.var_temp__blk949) + (assign46670_e59773 * locals.var_temp__blk949_dn9)) * assign46670_e59784) - (assign46670_e59775 * (((locals.var_temp2_dn9 - locals.var_temp__blk949_dn9) / (2.0 * assign46670_e59779)) + ((locals.var_temp2_dn9 + locals.var_temp__blk949_dn9) / (2.0 * assign46670_e59783))))) / (assign46670_e59784 * assign46670_e59784)),)
    } else {
        (locals.var_udse_dc, locals.var_udse_dc_dn4, locals.var_udse_dc_dn6, locals.var_udse_dc_dn7, locals.var_udse_dc_dn8, locals.var_udse_dc_dn9,)
    }
};
        locals.var_udse_dc = assign46670_e59787;
        locals.var_udse_dc_dn4 = assign46670_e59787_d_n4;
        locals.var_udse_dc_dn6 = assign46670_e59787_d_n6;
        locals.var_udse_dc_dn7 = assign46670_e59787_d_n7;
        locals.var_udse_dc_dn8 = assign46670_e59787_d_n8;
        locals.var_udse_dc_dn9 = assign46670_e59787_d_n9;

        let assign46680_e59790: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
        let assign46680_e59792: f64 = (-230.25850929940458);
        let assign46680_e59793: f64 = if assign46680_e59790 > assign46680_e59792 { 1.0 } else { 0.0 };
        locals.var_guard1249 = assign46680_e59793;

        let (assign46690_e59804, assign46690_e59804_d_n4, assign46690_e59804_d_n6, assign46690_e59804_d_n7, assign46690_e59804_d_n8, assign46690_e59804_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1249 != 0.0)) {
        let assign46690_e59801: f64 = (locals.var_x_ds_dc - locals.var_udse_dc);
        let assign46690_e59802: f64 = (assign46690_e59801).exp();
        (assign46690_e59802, (assign46690_e59802 * (locals.var_x_ds_dc_dn4 - locals.var_udse_dc_dn4)), (assign46690_e59802 * (locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)), (assign46690_e59802 * (locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)), (assign46690_e59802 * (locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)), (assign46690_e59802 * (locals.var_x_ds_dc_dn9 - locals.var_udse_dc_dn9)),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46690_e59804;
        locals.var_temp__blk949_dn4 = assign46690_e59804_d_n4;
        locals.var_temp__blk949_dn6 = assign46690_e59804_d_n6;
        locals.var_temp__blk949_dn7 = assign46690_e59804_d_n7;
        locals.var_temp__blk949_dn8 = assign46690_e59804_d_n8;
        locals.var_temp__blk949_dn9 = assign46690_e59804_d_n9;

        let (assign46700_e59844, assign46700_e59844_d_n4, assign46700_e59844_d_n6, assign46700_e59844_d_n7, assign46700_e59844_d_n8, assign46700_e59844_d_n9,) = {
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
        (assign46700_e59842, (-((1e-100 * (((-(locals.var_x_ds_dc_dn4 - locals.var_udse_dc_dn4)) * assign46700_e59839) + (assign46700_e59818 * (0.5 * (((-(locals.var_x_ds_dc_dn4 - locals.var_udse_dc_dn4)) * assign46700_e59836) + (assign46700_e59826 * ((-(locals.var_x_ds_dc_dn4 - locals.var_udse_dc_dn4)) * 0.3333333333333333))))))) / (assign46700_e59841 * assign46700_e59841))), (-((1e-100 * (((-(locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)) * assign46700_e59839) + (assign46700_e59818 * (0.5 * (((-(locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)) * assign46700_e59836) + (assign46700_e59826 * ((-(locals.var_x_ds_dc_dn6 - locals.var_udse_dc_dn6)) * 0.3333333333333333))))))) / (assign46700_e59841 * assign46700_e59841))), (-((1e-100 * (((-(locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)) * assign46700_e59839) + (assign46700_e59818 * (0.5 * (((-(locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)) * assign46700_e59836) + (assign46700_e59826 * ((-(locals.var_x_ds_dc_dn7 - locals.var_udse_dc_dn7)) * 0.3333333333333333))))))) / (assign46700_e59841 * assign46700_e59841))), (-((1e-100 * (((-(locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)) * assign46700_e59839) + (assign46700_e59818 * (0.5 * (((-(locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)) * assign46700_e59836) + (assign46700_e59826 * ((-(locals.var_x_ds_dc_dn8 - locals.var_udse_dc_dn8)) * 0.3333333333333333))))))) / (assign46700_e59841 * assign46700_e59841))), (-((1e-100 * (((-(locals.var_x_ds_dc_dn9 - locals.var_udse_dc_dn9)) * assign46700_e59839) + (assign46700_e59818 * (0.5 * (((-(locals.var_x_ds_dc_dn9 - locals.var_udse_dc_dn9)) * assign46700_e59836) + (assign46700_e59826 * ((-(locals.var_x_ds_dc_dn9 - locals.var_udse_dc_dn9)) * 0.3333333333333333))))))) / (assign46700_e59841 * assign46700_e59841))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46700_e59844;
        locals.var_temp__blk949_dn4 = assign46700_e59844_d_n4;
        locals.var_temp__blk949_dn6 = assign46700_e59844_d_n6;
        locals.var_temp__blk949_dn7 = assign46700_e59844_d_n7;
        locals.var_temp__blk949_dn8 = assign46700_e59844_d_n8;
        locals.var_temp__blk949_dn9 = assign46700_e59844_d_n9;

        let (assign46710_e59863, assign46710_e59863_d_n4, assign46710_e59863_d_n6, assign46710_e59863_d_n7, assign46710_e59863_d_n8, assign46710_e59863_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign46710_e59852: f64 = (0.5 * locals.var_x_ds_dc);
        let assign46710_e59856: f64 = (1.0 + locals.var_temp__blk949);
        let assign46710_e59857: f64 = (0.5 * assign46710_e59856);
        let assign46710_e59858: f64 = (assign46710_e59857).ln();
        let assign46710_e59859: f64 = (assign46710_e59852 - assign46710_e59858);
        let assign46710_e59860: f64 = (locals.var_phit1_dc * assign46710_e59859);
        let assign46710_e59861: f64 = (locals.var_vsbstar_dc + assign46710_e59860);
        (assign46710_e59861, (locals.var_vsbstar_dc_dn4 + ((locals.var_phit1_dc_dn4 * assign46710_e59859) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn4) - ((0.5 * locals.var_temp__blk949_dn4) / assign46710_e59857))))), (locals.var_vsbstar_dc_dn6 + ((locals.var_phit1_dc_dn6 * assign46710_e59859) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn6) - ((0.5 * locals.var_temp__blk949_dn6) / assign46710_e59857))))), (locals.var_vsbstar_dc_dn7 + ((locals.var_phit1_dc_dn7 * assign46710_e59859) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn7) - ((0.5 * locals.var_temp__blk949_dn7) / assign46710_e59857))))), (locals.var_vsbstar_dc_dn8 + ((locals.var_phit1_dc_dn8 * assign46710_e59859) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn8) - ((0.5 * locals.var_temp__blk949_dn8) / assign46710_e59857))))), (locals.var_vsbstar_dc_dn9 + ((locals.var_phit1_dc_dn9 * assign46710_e59859) + (locals.var_phit1_dc * ((0.5 * locals.var_x_ds_dc_dn9) - ((0.5 * locals.var_temp__blk949_dn9) / assign46710_e59857))))),)
    } else {
        (locals.var_vm, locals.var_vm_dn4, locals.var_vm_dn6, locals.var_vm_dn7, locals.var_vm_dn8, locals.var_vm_dn9,)
    }
};
        locals.var_vm = assign46710_e59863;
        locals.var_vm_dn4 = assign46710_e59863_d_n4;
        locals.var_vm_dn6 = assign46710_e59863_d_n6;
        locals.var_vm_dn7 = assign46710_e59863_d_n7;
        locals.var_vm_dn8 = assign46710_e59863_d_n8;
        locals.var_vm_dn9 = assign46710_e59863_d_n9;

        let (assign46720_e59871, assign46720_e59871_d_n4, assign46720_e59871_d_n6, assign46720_e59871_d_n7, assign46720_e59871_d_n8, assign46720_e59871_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign46720_e59869: f64 = (locals.var_gco_i * locals.var_phit1_dc);
        (assign46720_e59869, (locals.var_gco_i * locals.var_phit1_dc_dn4), (locals.var_gco_i * locals.var_phit1_dc_dn6), (locals.var_gco_i * locals.var_phit1_dc_dn7), (locals.var_gco_i * locals.var_phit1_dc_dn8), (locals.var_gco_i * locals.var_phit1_dc_dn9),)
    } else {
        (locals.var_dch, locals.var_dch_dn4, locals.var_dch_dn6, locals.var_dch_dn7, locals.var_dch_dn8, locals.var_dch_dn9,)
    }
};
        locals.var_dch = assign46720_e59871;
        locals.var_dch_dn4 = assign46720_e59871_d_n4;
        locals.var_dch_dn6 = assign46720_e59871_d_n6;
        locals.var_dch_dn7 = assign46720_e59871_d_n7;
        locals.var_dch_dn8 = assign46720_e59871_d_n8;
        locals.var_dch_dn9 = assign46720_e59871_d_n9;

        let (assign46730_e59879, assign46730_e59879_d_n4, assign46730_e59879_d_n6, assign46730_e59879_d_n7, assign46730_e59879_d_n8, assign46730_e59879_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign46730_e59877: f64 = (locals.var_voxm_dc + locals.var_dch);
        (assign46730_e59877, (locals.var_voxm_dc_dn4 + locals.var_dch_dn4), (locals.var_voxm_dc_dn6 + locals.var_dch_dn6), (locals.var_voxm_dc_dn7 + locals.var_dch_dn7), (locals.var_voxm_dc_dn8 + locals.var_dch_dn8), (locals.var_voxm_dc_dn9 + locals.var_dch_dn9),)
    } else {
        (locals.var_arg2mina, locals.var_arg2mina_dn4, locals.var_arg2mina_dn6, locals.var_arg2mina_dn7, locals.var_arg2mina_dn8, locals.var_arg2mina_dn9,)
    }
};
        locals.var_arg2mina = assign46730_e59879;
        locals.var_arg2mina_dn4 = assign46730_e59879_d_n4;
        locals.var_arg2mina_dn6 = assign46730_e59879_d_n6;
        locals.var_arg2mina_dn7 = assign46730_e59879_d_n7;
        locals.var_arg2mina_dn8 = assign46730_e59879_d_n8;
        locals.var_arg2mina_dn9 = assign46730_e59879_d_n9;

        let (assign46740_e59900, assign46740_e59900_d_n4, assign46740_e59900_d_n6, assign46740_e59900_d_n7, assign46740_e59900_d_n8, assign46740_e59900_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign46740_e59886: f64 = locals.var_arg2mina;
        let assign46740_e59889: f64 = (-locals.var_arg2mina);
        let assign46740_e59892: f64 = (-locals.var_arg2mina);
        let assign46740_e59893: f64 = (assign46740_e59889 * assign46740_e59892);
        let assign46740_e59895: f64 = (assign46740_e59893 + 0.01);
        let assign46740_e59896: f64 = (assign46740_e59895).sqrt();
        let assign46740_e59897: f64 = (assign46740_e59886 - assign46740_e59896);
        let assign46740_e59898: f64 = (0.5 * assign46740_e59897);
        (assign46740_e59898, (0.5 * (locals.var_arg2mina_dn4 - ((((-locals.var_arg2mina_dn4) * assign46740_e59892) + (assign46740_e59889 * (-locals.var_arg2mina_dn4))) / (2.0 * assign46740_e59896)))), (0.5 * (locals.var_arg2mina_dn6 - ((((-locals.var_arg2mina_dn6) * assign46740_e59892) + (assign46740_e59889 * (-locals.var_arg2mina_dn6))) / (2.0 * assign46740_e59896)))), (0.5 * (locals.var_arg2mina_dn7 - ((((-locals.var_arg2mina_dn7) * assign46740_e59892) + (assign46740_e59889 * (-locals.var_arg2mina_dn7))) / (2.0 * assign46740_e59896)))), (0.5 * (locals.var_arg2mina_dn8 - ((((-locals.var_arg2mina_dn8) * assign46740_e59892) + (assign46740_e59889 * (-locals.var_arg2mina_dn8))) / (2.0 * assign46740_e59896)))), (0.5 * (locals.var_arg2mina_dn9 - ((((-locals.var_arg2mina_dn9) * assign46740_e59892) + (assign46740_e59889 * (-locals.var_arg2mina_dn9))) / (2.0 * assign46740_e59896)))),)
    } else {
        (locals.var_psi_t, locals.var_psi_t_dn4, locals.var_psi_t_dn6, locals.var_psi_t_dn7, locals.var_psi_t_dn8, locals.var_psi_t_dn9,)
    }
};
        locals.var_psi_t = assign46740_e59900;
        locals.var_psi_t_dn4 = assign46740_e59900_d_n4;
        locals.var_psi_t_dn6 = assign46740_e59900_d_n6;
        locals.var_psi_t_dn7 = assign46740_e59900_d_n7;
        locals.var_psi_t_dn8 = assign46740_e59900_d_n8;
        locals.var_psi_t_dn9 = assign46740_e59900_d_n9;

        let (assign46750_e59913, assign46750_e59913_d_n4, assign46750_e59913_d_n6, assign46750_e59913_d_n7, assign46750_e59913_d_n8, assign46750_e59913_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign46750_e59906: f64 = (locals.var_voxm_dc * locals.var_voxm_dc);
        let assign46750_e59908: f64 = (assign46750_e59906 + 1e-6);
        let assign46750_e59909: f64 = (assign46750_e59908).sqrt();
        let assign46750_e59911: f64 = (assign46750_e59909 * locals.var_inv_chib);
        (assign46750_e59911, ((((locals.var_voxm_dc_dn4 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn4)) / (2.0 * assign46750_e59909)) * locals.var_inv_chib), ((((locals.var_voxm_dc_dn6 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn6)) / (2.0 * assign46750_e59909)) * locals.var_inv_chib), ((((locals.var_voxm_dc_dn7 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn7)) / (2.0 * assign46750_e59909)) * locals.var_inv_chib), ((((locals.var_voxm_dc_dn8 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn8)) / (2.0 * assign46750_e59909)) * locals.var_inv_chib), ((((locals.var_voxm_dc_dn9 * locals.var_voxm_dc) + (locals.var_voxm_dc * locals.var_voxm_dc_dn9)) / (2.0 * assign46750_e59909)) * locals.var_inv_chib),)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign46750_e59913;
        locals.var_zg_dn4 = assign46750_e59913_d_n4;
        locals.var_zg_dn6 = assign46750_e59913_d_n6;
        locals.var_zg_dn7 = assign46750_e59913_d_n7;
        locals.var_zg_dn8 = assign46750_e59913_d_n8;
        locals.var_zg_dn9 = assign46750_e59913_d_n9;

        let assign46760_e59916: f64 = if locals.var_gc3_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1250 = assign46760_e59916;

        let (assign46770_e59939, assign46770_e59939_d_n4, assign46770_e59939_d_n6, assign46770_e59939_d_n7, assign46770_e59939_d_n8, assign46770_e59939_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1250 != 0.0)) {
        let assign46770_e59925: f64 = (locals.var_zg + locals.var_gcq);
        let assign46770_e59928: f64 = (locals.var_zg - locals.var_gcq);
        let assign46770_e59931: f64 = (locals.var_zg - locals.var_gcq);
        let assign46770_e59932: f64 = (assign46770_e59928 * assign46770_e59931);
        let assign46770_e59934: f64 = (assign46770_e59932 + 1e-6);
        let assign46770_e59935: f64 = (assign46770_e59934).sqrt();
        let assign46770_e59936: f64 = (assign46770_e59925 - assign46770_e59935);
        let assign46770_e59937: f64 = (0.5 * assign46770_e59936);
        (assign46770_e59937, (0.5 * (locals.var_zg_dn4 - (((locals.var_zg_dn4 * assign46770_e59931) + (assign46770_e59928 * locals.var_zg_dn4)) / (2.0 * assign46770_e59935)))), (0.5 * (locals.var_zg_dn6 - (((locals.var_zg_dn6 * assign46770_e59931) + (assign46770_e59928 * locals.var_zg_dn6)) / (2.0 * assign46770_e59935)))), (0.5 * (locals.var_zg_dn7 - (((locals.var_zg_dn7 * assign46770_e59931) + (assign46770_e59928 * locals.var_zg_dn7)) / (2.0 * assign46770_e59935)))), (0.5 * (locals.var_zg_dn8 - (((locals.var_zg_dn8 * assign46770_e59931) + (assign46770_e59928 * locals.var_zg_dn8)) / (2.0 * assign46770_e59935)))), (0.5 * (locals.var_zg_dn9 - (((locals.var_zg_dn9 * assign46770_e59931) + (assign46770_e59928 * locals.var_zg_dn9)) / (2.0 * assign46770_e59935)))),)
    } else {
        (locals.var_zg, locals.var_zg_dn4, locals.var_zg_dn6, locals.var_zg_dn7, locals.var_zg_dn8, locals.var_zg_dn9,)
    }
};
        locals.var_zg = assign46770_e59939;
        locals.var_zg_dn4 = assign46770_e59939_d_n4;
        locals.var_zg_dn6 = assign46770_e59939_d_n6;
        locals.var_zg_dn7 = assign46770_e59939_d_n7;
        locals.var_zg_dn8 = assign46770_e59939_d_n8;
        locals.var_zg_dn9 = assign46770_e59939_d_n9;

        let (assign46780_e59953, assign46780_e59953_d_n4, assign46780_e59953_d_n6, assign46780_e59953_d_n7, assign46780_e59953_d_n8, assign46780_e59953_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign46780_e59946: f64 = (locals.var_psi_t - locals.var_alpha_b);
        let assign46780_e59948: f64 = (assign46780_e59946 - locals.var_vm);
        let assign46780_e59950: f64 = (assign46780_e59948 * locals.var_inv_phit1_dc);
        let assign46780_e59951: f64 = (locals.var_x_m_dc + assign46780_e59950);
        (assign46780_e59951, (locals.var_x_m_dc_dn4 + ((((locals.var_psi_t_dn4 - locals.var_alpha_b_dn4) - locals.var_vm_dn4) * locals.var_inv_phit1_dc) + (assign46780_e59948 * locals.var_inv_phit1_dc_dn4))), (locals.var_x_m_dc_dn6 + (((locals.var_psi_t_dn6 - locals.var_vm_dn6) * locals.var_inv_phit1_dc) + (assign46780_e59948 * locals.var_inv_phit1_dc_dn6))), (locals.var_x_m_dc_dn7 + (((locals.var_psi_t_dn7 - locals.var_vm_dn7) * locals.var_inv_phit1_dc) + (assign46780_e59948 * locals.var_inv_phit1_dc_dn7))), (locals.var_x_m_dc_dn8 + (((locals.var_psi_t_dn8 - locals.var_vm_dn8) * locals.var_inv_phit1_dc) + (assign46780_e59948 * locals.var_inv_phit1_dc_dn8))), (locals.var_x_m_dc_dn9 + (((locals.var_psi_t_dn9 - locals.var_vm_dn9) * locals.var_inv_phit1_dc) + (assign46780_e59948 * locals.var_inv_phit1_dc_dn9))),)
    } else {
        (locals.var_arg1, locals.var_arg1_dn4, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8, locals.var_arg1_dn9,)
    }
};
        locals.var_arg1 = assign46780_e59953;
        locals.var_arg1_dn4 = assign46780_e59953_d_n4;
        locals.var_arg1_dn6 = assign46780_e59953_d_n6;
        locals.var_arg1_dn7 = assign46780_e59953_d_n7;
        locals.var_arg1_dn8 = assign46780_e59953_d_n8;
        locals.var_arg1_dn9 = assign46780_e59953_d_n9;

        let assign46790_e59955: f64 = (locals.var_arg1).abs();
        let assign46790_e59957: f64 = if assign46790_e59955 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1251 = assign46790_e59957;

        let (assign46800_e59966, assign46800_e59966_d_n4, assign46800_e59966_d_n6, assign46800_e59966_d_n7, assign46800_e59966_d_n8, assign46800_e59966_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1251 != 0.0)) {
        let assign46800_e59964: f64 = (locals.var_arg1).exp();
        (assign46800_e59964, (assign46800_e59964 * locals.var_arg1_dn4), (assign46800_e59964 * locals.var_arg1_dn6), (assign46800_e59964 * locals.var_arg1_dn7), (assign46800_e59964 * locals.var_arg1_dn8), (assign46800_e59964 * locals.var_arg1_dn9),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn4, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, locals.var_dsi_dn9,)
    }
};
        locals.var_dsi = assign46800_e59966;
        locals.var_dsi_dn4 = assign46800_e59966_d_n4;
        locals.var_dsi_dn6 = assign46800_e59966_d_n6;
        locals.var_dsi_dn7 = assign46800_e59966_d_n7;
        locals.var_dsi_dn8 = assign46800_e59966_d_n8;
        locals.var_dsi_dn9 = assign46800_e59966_d_n9;

        let assign46810_e59969: f64 = if locals.var_arg1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1252 = assign46810_e59969;

        let (assign46820_e60005, assign46820_e60005_d_n4, assign46820_e60005_d_n6, assign46820_e60005_d_n7, assign46820_e60005_d_n8, assign46820_e60005_d_n9,) = {
    if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1251 == 0.0)) && (locals.var_guard1252 != 0.0)) {
        let assign46820_e59981: f64 = (-230.25850929940458);
        let assign46820_e59983: f64 = (assign46820_e59981 - locals.var_arg1);
        let assign46820_e59987: f64 = (-230.25850929940458);
        let assign46820_e59989: f64 = (assign46820_e59987 - locals.var_arg1);
        let assign46820_e59992: f64 = (-230.25850929940458);
        let assign46820_e59994: f64 = (assign46820_e59992 - locals.var_arg1);
        let assign46820_e59996: f64 = (assign46820_e59994 * 0.3333333333333333);
        let assign46820_e59997: f64 = (1.0 + assign46820_e59996);
        let assign46820_e59998: f64 = (assign46820_e59989 * assign46820_e59997);
        let assign46820_e59999: f64 = (0.5 * assign46820_e59998);
        let assign46820_e60000: f64 = (1.0 + assign46820_e59999);
        let assign46820_e60001: f64 = (assign46820_e59983 * assign46820_e60000);
        let assign46820_e60002: f64 = (1.0 + assign46820_e60001);
        let assign46820_e60003: f64 = (1e-100 / assign46820_e60002);
        (assign46820_e60003, (-((1e-100 * (((-locals.var_arg1_dn4) * assign46820_e60000) + (assign46820_e59983 * (0.5 * (((-locals.var_arg1_dn4) * assign46820_e59997) + (assign46820_e59989 * ((-locals.var_arg1_dn4) * 0.3333333333333333))))))) / (assign46820_e60002 * assign46820_e60002))), (-((1e-100 * (((-locals.var_arg1_dn6) * assign46820_e60000) + (assign46820_e59983 * (0.5 * (((-locals.var_arg1_dn6) * assign46820_e59997) + (assign46820_e59989 * ((-locals.var_arg1_dn6) * 0.3333333333333333))))))) / (assign46820_e60002 * assign46820_e60002))), (-((1e-100 * (((-locals.var_arg1_dn7) * assign46820_e60000) + (assign46820_e59983 * (0.5 * (((-locals.var_arg1_dn7) * assign46820_e59997) + (assign46820_e59989 * ((-locals.var_arg1_dn7) * 0.3333333333333333))))))) / (assign46820_e60002 * assign46820_e60002))), (-((1e-100 * (((-locals.var_arg1_dn8) * assign46820_e60000) + (assign46820_e59983 * (0.5 * (((-locals.var_arg1_dn8) * assign46820_e59997) + (assign46820_e59989 * ((-locals.var_arg1_dn8) * 0.3333333333333333))))))) / (assign46820_e60002 * assign46820_e60002))), (-((1e-100 * (((-locals.var_arg1_dn9) * assign46820_e60000) + (assign46820_e59983 * (0.5 * (((-locals.var_arg1_dn9) * assign46820_e59997) + (assign46820_e59989 * ((-locals.var_arg1_dn9) * 0.3333333333333333))))))) / (assign46820_e60002 * assign46820_e60002))),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn4, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, locals.var_dsi_dn9,)
    }
};
        locals.var_dsi = assign46820_e60005;
        locals.var_dsi_dn4 = assign46820_e60005_d_n4;
        locals.var_dsi_dn6 = assign46820_e60005_d_n6;
        locals.var_dsi_dn7 = assign46820_e60005_d_n7;
        locals.var_dsi_dn8 = assign46820_e60005_d_n8;
        locals.var_dsi_dn9 = assign46820_e60005_d_n9;

        let (assign46830_e60039, assign46830_e60039_d_n4, assign46830_e60039_d_n6, assign46830_e60039_d_n7, assign46830_e60039_d_n8, assign46830_e60039_d_n9,) = {
    if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1251 == 0.0)) && (locals.var_guard1252 == 0.0)) {
        let assign46830_e60019: f64 = (locals.var_arg1 - 230.25850929940458);
        let assign46830_e60024: f64 = (locals.var_arg1 - 230.25850929940458);
        let assign46830_e60028: f64 = (locals.var_arg1 - 230.25850929940458);
        let assign46830_e60030: f64 = (assign46830_e60028 * 0.3333333333333333);
        let assign46830_e60031: f64 = (1.0 + assign46830_e60030);
        let assign46830_e60032: f64 = (assign46830_e60024 * assign46830_e60031);
        let assign46830_e60033: f64 = (0.5 * assign46830_e60032);
        let assign46830_e60034: f64 = (1.0 + assign46830_e60033);
        let assign46830_e60035: f64 = (assign46830_e60019 * assign46830_e60034);
        let assign46830_e60036: f64 = (1.0 + assign46830_e60035);
        let assign46830_e60037: f64 = (1e100 * assign46830_e60036);
        (assign46830_e60037, (1e100 * ((locals.var_arg1_dn4 * assign46830_e60034) + (assign46830_e60019 * (0.5 * ((locals.var_arg1_dn4 * assign46830_e60031) + (assign46830_e60024 * (locals.var_arg1_dn4 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn6 * assign46830_e60034) + (assign46830_e60019 * (0.5 * ((locals.var_arg1_dn6 * assign46830_e60031) + (assign46830_e60024 * (locals.var_arg1_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn7 * assign46830_e60034) + (assign46830_e60019 * (0.5 * ((locals.var_arg1_dn7 * assign46830_e60031) + (assign46830_e60024 * (locals.var_arg1_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn8 * assign46830_e60034) + (assign46830_e60019 * (0.5 * ((locals.var_arg1_dn8 * assign46830_e60031) + (assign46830_e60024 * (locals.var_arg1_dn8 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn9 * assign46830_e60034) + (assign46830_e60019 * (0.5 * ((locals.var_arg1_dn9 * assign46830_e60031) + (assign46830_e60024 * (locals.var_arg1_dn9 * 0.3333333333333333))))))),)
    } else {
        (locals.var_dsi, locals.var_dsi_dn4, locals.var_dsi_dn6, locals.var_dsi_dn7, locals.var_dsi_dn8, locals.var_dsi_dn9,)
    }
};
        locals.var_dsi = assign46830_e60039;
        locals.var_dsi_dn4 = assign46830_e60039_d_n4;
        locals.var_dsi_dn6 = assign46830_e60039_d_n6;
        locals.var_dsi_dn7 = assign46830_e60039_d_n7;
        locals.var_dsi_dn8 = assign46830_e60039_d_n8;
        locals.var_dsi_dn9 = assign46830_e60039_d_n9;

        let (assign46840_e60052, assign46840_e60052_d_n4, assign46840_e60052_d_n6, assign46840_e60052_d_n7, assign46840_e60052_d_n8, assign46840_e60052_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign46840_e60045: f64 = (locals.var_v_gs + locals.var_vsbstar_dc);
        let assign46840_e60047: f64 = (assign46840_e60045 - locals.var_vm);
        let assign46840_e60048: f64 = (-assign46840_e60047);
        let assign46840_e60050: f64 = (assign46840_e60048 * locals.var_inv_phit1_dc);
        (assign46840_e60050, (((-(locals.var_vsbstar_dc_dn4 - locals.var_vm_dn4)) * locals.var_inv_phit1_dc) + (assign46840_e60048 * locals.var_inv_phit1_dc_dn4)), (((-((locals.var_v_gs_dn6 + locals.var_vsbstar_dc_dn6) - locals.var_vm_dn6)) * locals.var_inv_phit1_dc) + (assign46840_e60048 * locals.var_inv_phit1_dc_dn6)), (((-((locals.var_v_gs_dn7 + locals.var_vsbstar_dc_dn7) - locals.var_vm_dn7)) * locals.var_inv_phit1_dc) + (assign46840_e60048 * locals.var_inv_phit1_dc_dn7)), (((-((locals.var_v_gs_dn8 + locals.var_vsbstar_dc_dn8) - locals.var_vm_dn8)) * locals.var_inv_phit1_dc) + (assign46840_e60048 * locals.var_inv_phit1_dc_dn8)), (((-(locals.var_vsbstar_dc_dn9 - locals.var_vm_dn9)) * locals.var_inv_phit1_dc) + (assign46840_e60048 * locals.var_inv_phit1_dc_dn9)),)
    } else {
        (locals.var_arg1, locals.var_arg1_dn4, locals.var_arg1_dn6, locals.var_arg1_dn7, locals.var_arg1_dn8, locals.var_arg1_dn9,)
    }
};
        locals.var_arg1 = assign46840_e60052;
        locals.var_arg1_dn4 = assign46840_e60052_d_n4;
        locals.var_arg1_dn6 = assign46840_e60052_d_n6;
        locals.var_arg1_dn7 = assign46840_e60052_d_n7;
        locals.var_arg1_dn8 = assign46840_e60052_d_n8;
        locals.var_arg1_dn9 = assign46840_e60052_d_n9;

        let assign46850_e60054: f64 = (locals.var_arg1).abs();
        let assign46850_e60056: f64 = if assign46850_e60054 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1253 = assign46850_e60056;

        let (assign46860_e60065, assign46860_e60065_d_n4, assign46860_e60065_d_n6, assign46860_e60065_d_n7, assign46860_e60065_d_n8, assign46860_e60065_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1253 != 0.0)) {
        let assign46860_e60063: f64 = (locals.var_arg1).exp();
        (assign46860_e60063, (assign46860_e60063 * locals.var_arg1_dn4), (assign46860_e60063 * locals.var_arg1_dn6), (assign46860_e60063 * locals.var_arg1_dn7), (assign46860_e60063 * locals.var_arg1_dn8), (assign46860_e60063 * locals.var_arg1_dn9),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46860_e60065;
        locals.var_temp__blk949_dn4 = assign46860_e60065_d_n4;
        locals.var_temp__blk949_dn6 = assign46860_e60065_d_n6;
        locals.var_temp__blk949_dn7 = assign46860_e60065_d_n7;
        locals.var_temp__blk949_dn8 = assign46860_e60065_d_n8;
        locals.var_temp__blk949_dn9 = assign46860_e60065_d_n9;

        let assign46870_e60068: f64 = if locals.var_arg1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1254 = assign46870_e60068;

        let (assign46880_e60104, assign46880_e60104_d_n4, assign46880_e60104_d_n6, assign46880_e60104_d_n7, assign46880_e60104_d_n8, assign46880_e60104_d_n9,) = {
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
        (assign46880_e60102, (-((1e-100 * (((-locals.var_arg1_dn4) * assign46880_e60099) + (assign46880_e60082 * (0.5 * (((-locals.var_arg1_dn4) * assign46880_e60096) + (assign46880_e60088 * ((-locals.var_arg1_dn4) * 0.3333333333333333))))))) / (assign46880_e60101 * assign46880_e60101))), (-((1e-100 * (((-locals.var_arg1_dn6) * assign46880_e60099) + (assign46880_e60082 * (0.5 * (((-locals.var_arg1_dn6) * assign46880_e60096) + (assign46880_e60088 * ((-locals.var_arg1_dn6) * 0.3333333333333333))))))) / (assign46880_e60101 * assign46880_e60101))), (-((1e-100 * (((-locals.var_arg1_dn7) * assign46880_e60099) + (assign46880_e60082 * (0.5 * (((-locals.var_arg1_dn7) * assign46880_e60096) + (assign46880_e60088 * ((-locals.var_arg1_dn7) * 0.3333333333333333))))))) / (assign46880_e60101 * assign46880_e60101))), (-((1e-100 * (((-locals.var_arg1_dn8) * assign46880_e60099) + (assign46880_e60082 * (0.5 * (((-locals.var_arg1_dn8) * assign46880_e60096) + (assign46880_e60088 * ((-locals.var_arg1_dn8) * 0.3333333333333333))))))) / (assign46880_e60101 * assign46880_e60101))), (-((1e-100 * (((-locals.var_arg1_dn9) * assign46880_e60099) + (assign46880_e60082 * (0.5 * (((-locals.var_arg1_dn9) * assign46880_e60096) + (assign46880_e60088 * ((-locals.var_arg1_dn9) * 0.3333333333333333))))))) / (assign46880_e60101 * assign46880_e60101))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46880_e60104;
        locals.var_temp__blk949_dn4 = assign46880_e60104_d_n4;
        locals.var_temp__blk949_dn6 = assign46880_e60104_d_n6;
        locals.var_temp__blk949_dn7 = assign46880_e60104_d_n7;
        locals.var_temp__blk949_dn8 = assign46880_e60104_d_n8;
        locals.var_temp__blk949_dn9 = assign46880_e60104_d_n9;

        let (assign46890_e60138, assign46890_e60138_d_n4, assign46890_e60138_d_n6, assign46890_e60138_d_n7, assign46890_e60138_d_n8, assign46890_e60138_d_n9,) = {
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
        (assign46890_e60136, (1e100 * ((locals.var_arg1_dn4 * assign46890_e60133) + (assign46890_e60118 * (0.5 * ((locals.var_arg1_dn4 * assign46890_e60130) + (assign46890_e60123 * (locals.var_arg1_dn4 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn6 * assign46890_e60133) + (assign46890_e60118 * (0.5 * ((locals.var_arg1_dn6 * assign46890_e60130) + (assign46890_e60123 * (locals.var_arg1_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn7 * assign46890_e60133) + (assign46890_e60118 * (0.5 * ((locals.var_arg1_dn7 * assign46890_e60130) + (assign46890_e60123 * (locals.var_arg1_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn8 * assign46890_e60133) + (assign46890_e60118 * (0.5 * ((locals.var_arg1_dn8 * assign46890_e60130) + (assign46890_e60123 * (locals.var_arg1_dn8 * 0.3333333333333333))))))), (1e100 * ((locals.var_arg1_dn9 * assign46890_e60133) + (assign46890_e60118 * (0.5 * ((locals.var_arg1_dn9 * assign46890_e60130) + (assign46890_e60123 * (locals.var_arg1_dn9 * 0.3333333333333333))))))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46890_e60138;
        locals.var_temp__blk949_dn4 = assign46890_e60138_d_n4;
        locals.var_temp__blk949_dn6 = assign46890_e60138_d_n6;
        locals.var_temp__blk949_dn7 = assign46890_e60138_d_n7;
        locals.var_temp__blk949_dn8 = assign46890_e60138_d_n8;
        locals.var_temp__blk949_dn9 = assign46890_e60138_d_n9;

        let (assign46900_e60146, assign46900_e60146_d_n4, assign46900_e60146_d_n6, assign46900_e60146_d_n7, assign46900_e60146_d_n8, assign46900_e60146_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign46900_e60144: f64 = (locals.var_dsi * locals.var_temp__blk949);
        (assign46900_e60144, ((locals.var_dsi_dn4 * locals.var_temp__blk949) + (locals.var_dsi * locals.var_temp__blk949_dn4)), ((locals.var_dsi_dn6 * locals.var_temp__blk949) + (locals.var_dsi * locals.var_temp__blk949_dn6)), ((locals.var_dsi_dn7 * locals.var_temp__blk949) + (locals.var_dsi * locals.var_temp__blk949_dn7)), ((locals.var_dsi_dn8 * locals.var_temp__blk949) + (locals.var_dsi * locals.var_temp__blk949_dn8)), ((locals.var_dsi_dn9 * locals.var_temp__blk949) + (locals.var_dsi * locals.var_temp__blk949_dn9)),)
    } else {
        (locals.var_dgate, locals.var_dgate_dn4, locals.var_dgate_dn6, locals.var_dgate_dn7, locals.var_dgate_dn8, locals.var_dgate_dn9,)
    }
};
        locals.var_dgate = assign46900_e60146;
        locals.var_dgate_dn4 = assign46900_e60146_d_n4;
        locals.var_dgate_dn6 = assign46900_e60146_d_n6;
        locals.var_dgate_dn7 = assign46900_e60146_d_n7;
        locals.var_dgate_dn8 = assign46900_e60146_d_n8;
        locals.var_dgate_dn9 = assign46900_e60146_d_n9;

        let (assign46910_e60163, assign46910_e60163_d_n4, assign46910_e60163_d_n6, assign46910_e60163_d_n7, assign46910_e60163_d_n8, assign46910_e60163_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign46910_e60152: f64 = (-1.5);
        let assign46910_e60157: f64 = (locals.var_gc3_i * locals.var_zg);
        let assign46910_e60158: f64 = (locals.var_gc2_i + assign46910_e60157);
        let assign46910_e60159: f64 = (locals.var_zg * assign46910_e60158);
        let assign46910_e60160: f64 = (assign46910_e60152 + assign46910_e60159);
        let assign46910_e60161: f64 = (locals.var_bch * assign46910_e60160);
        (assign46910_e60161, (locals.var_bch * ((locals.var_zg_dn4 * assign46910_e60158) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn4)))), (locals.var_bch * ((locals.var_zg_dn6 * assign46910_e60158) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn6)))), (locals.var_bch * ((locals.var_zg_dn7 * assign46910_e60158) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn7)))), (locals.var_bch * ((locals.var_zg_dn8 * assign46910_e60158) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn8)))), (locals.var_bch * ((locals.var_zg_dn9 * assign46910_e60158) + (locals.var_zg * (locals.var_gc3_i * locals.var_zg_dn9)))),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign46910_e60163;
        locals.var_temp__blk949_dn4 = assign46910_e60163_d_n4;
        locals.var_temp__blk949_dn6 = assign46910_e60163_d_n6;
        locals.var_temp__blk949_dn7 = assign46910_e60163_d_n7;
        locals.var_temp__blk949_dn8 = assign46910_e60163_d_n8;
        locals.var_temp__blk949_dn9 = assign46910_e60163_d_n9;

        let assign46920_e60166: f64 = if locals.var_temp__blk949 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1255 = assign46920_e60166;

        let (assign46930_e60188, assign46930_e60188_d_n4, assign46930_e60188_d_n6, assign46930_e60188_d_n7, assign46930_e60188_d_n8, assign46930_e60188_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1255 != 0.0)) {
        let assign46930_e60180: f64 = (locals.var_temp__blk949 * 0.3333333333333333);
        let assign46930_e60181: f64 = (1.0 + assign46930_e60180);
        let assign46930_e60182: f64 = (locals.var_temp__blk949 * assign46930_e60181);
        let assign46930_e60183: f64 = (0.5 * assign46930_e60182);
        let assign46930_e60184: f64 = (1.0 + assign46930_e60183);
        let assign46930_e60185: f64 = (locals.var_temp__blk949 * assign46930_e60184);
        let assign46930_e60186: f64 = (1.0 + assign46930_e60185);
        (assign46930_e60186, ((locals.var_temp__blk949_dn4 * assign46930_e60184) + (locals.var_temp__blk949 * (0.5 * ((locals.var_temp__blk949_dn4 * assign46930_e60181) + (locals.var_temp__blk949 * (locals.var_temp__blk949_dn4 * 0.3333333333333333)))))), ((locals.var_temp__blk949_dn6 * assign46930_e60184) + (locals.var_temp__blk949 * (0.5 * ((locals.var_temp__blk949_dn6 * assign46930_e60181) + (locals.var_temp__blk949 * (locals.var_temp__blk949_dn6 * 0.3333333333333333)))))), ((locals.var_temp__blk949_dn7 * assign46930_e60184) + (locals.var_temp__blk949 * (0.5 * ((locals.var_temp__blk949_dn7 * assign46930_e60181) + (locals.var_temp__blk949 * (locals.var_temp__blk949_dn7 * 0.3333333333333333)))))), ((locals.var_temp__blk949_dn8 * assign46930_e60184) + (locals.var_temp__blk949 * (0.5 * ((locals.var_temp__blk949_dn8 * assign46930_e60181) + (locals.var_temp__blk949 * (locals.var_temp__blk949_dn8 * 0.3333333333333333)))))), ((locals.var_temp__blk949_dn9 * assign46930_e60184) + (locals.var_temp__blk949 * (0.5 * ((locals.var_temp__blk949_dn9 * assign46930_e60181) + (locals.var_temp__blk949 * (locals.var_temp__blk949_dn9 * 0.3333333333333333)))))),)
    } else {
        (locals.var_tp, locals.var_tp_dn4, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9,)
    }
};
        locals.var_tp = assign46930_e60188;
        locals.var_tp_dn4 = assign46930_e60188_d_n4;
        locals.var_tp_dn6 = assign46930_e60188_d_n6;
        locals.var_tp_dn7 = assign46930_e60188_d_n7;
        locals.var_tp_dn8 = assign46930_e60188_d_n8;
        locals.var_tp_dn9 = assign46930_e60188_d_n9;

        let assign46940_e60191: f64 = (-230.25850929940458);
        let assign46940_e60192: f64 = if locals.var_temp__blk949 > assign46940_e60191 { 1.0 } else { 0.0 };
        locals.var_guard1256 = assign46940_e60192;

    }

    pub(super) fn stamp_transient_block_31(
        locals: &mut StampLocals,
    ) {
        let (assign46950_e60204, assign46950_e60204_d_n4, assign46950_e60204_d_n6, assign46950_e60204_d_n7, assign46950_e60204_d_n8, assign46950_e60204_d_n9,) = {
    if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1255 == 0.0)) && (locals.var_guard1256 != 0.0)) {
        let assign46950_e60202: f64 = (locals.var_temp__blk949).exp();
        (assign46950_e60202, (assign46950_e60202 * locals.var_temp__blk949_dn4), (assign46950_e60202 * locals.var_temp__blk949_dn6), (assign46950_e60202 * locals.var_temp__blk949_dn7), (assign46950_e60202 * locals.var_temp__blk949_dn8), (assign46950_e60202 * locals.var_temp__blk949_dn9),)
    } else {
        (locals.var_tp, locals.var_tp_dn4, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9,)
    }
};
        locals.var_tp = assign46950_e60204;
        locals.var_tp_dn4 = assign46950_e60204_d_n4;
        locals.var_tp_dn6 = assign46950_e60204_d_n6;
        locals.var_tp_dn7 = assign46950_e60204_d_n7;
        locals.var_tp_dn8 = assign46950_e60204_d_n8;
        locals.var_tp_dn9 = assign46950_e60204_d_n9;

        let (assign46960_e60241, assign46960_e60241_d_n4, assign46960_e60241_d_n6, assign46960_e60241_d_n7, assign46960_e60241_d_n8, assign46960_e60241_d_n9,) = {
    if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1255 == 0.0)) && (locals.var_guard1256 == 0.0)) {
        let assign46960_e60217: f64 = (-230.25850929940458);
        let assign46960_e60219: f64 = (assign46960_e60217 - locals.var_temp__blk949);
        let assign46960_e60223: f64 = (-230.25850929940458);
        let assign46960_e60225: f64 = (assign46960_e60223 - locals.var_temp__blk949);
        let assign46960_e60228: f64 = (-230.25850929940458);
        let assign46960_e60230: f64 = (assign46960_e60228 - locals.var_temp__blk949);
        let assign46960_e60232: f64 = (assign46960_e60230 * 0.3333333333333333);
        let assign46960_e60233: f64 = (1.0 + assign46960_e60232);
        let assign46960_e60234: f64 = (assign46960_e60225 * assign46960_e60233);
        let assign46960_e60235: f64 = (0.5 * assign46960_e60234);
        let assign46960_e60236: f64 = (1.0 + assign46960_e60235);
        let assign46960_e60237: f64 = (assign46960_e60219 * assign46960_e60236);
        let assign46960_e60238: f64 = (1.0 + assign46960_e60237);
        let assign46960_e60239: f64 = (1e-100 / assign46960_e60238);
        (assign46960_e60239, (-((1e-100 * (((-locals.var_temp__blk949_dn4) * assign46960_e60236) + (assign46960_e60219 * (0.5 * (((-locals.var_temp__blk949_dn4) * assign46960_e60233) + (assign46960_e60225 * ((-locals.var_temp__blk949_dn4) * 0.3333333333333333))))))) / (assign46960_e60238 * assign46960_e60238))), (-((1e-100 * (((-locals.var_temp__blk949_dn6) * assign46960_e60236) + (assign46960_e60219 * (0.5 * (((-locals.var_temp__blk949_dn6) * assign46960_e60233) + (assign46960_e60225 * ((-locals.var_temp__blk949_dn6) * 0.3333333333333333))))))) / (assign46960_e60238 * assign46960_e60238))), (-((1e-100 * (((-locals.var_temp__blk949_dn7) * assign46960_e60236) + (assign46960_e60219 * (0.5 * (((-locals.var_temp__blk949_dn7) * assign46960_e60233) + (assign46960_e60225 * ((-locals.var_temp__blk949_dn7) * 0.3333333333333333))))))) / (assign46960_e60238 * assign46960_e60238))), (-((1e-100 * (((-locals.var_temp__blk949_dn8) * assign46960_e60236) + (assign46960_e60219 * (0.5 * (((-locals.var_temp__blk949_dn8) * assign46960_e60233) + (assign46960_e60225 * ((-locals.var_temp__blk949_dn8) * 0.3333333333333333))))))) / (assign46960_e60238 * assign46960_e60238))), (-((1e-100 * (((-locals.var_temp__blk949_dn9) * assign46960_e60236) + (assign46960_e60219 * (0.5 * (((-locals.var_temp__blk949_dn9) * assign46960_e60233) + (assign46960_e60225 * ((-locals.var_temp__blk949_dn9) * 0.3333333333333333))))))) / (assign46960_e60238 * assign46960_e60238))),)
    } else {
        (locals.var_tp, locals.var_tp_dn4, locals.var_tp_dn6, locals.var_tp_dn7, locals.var_tp_dn8, locals.var_tp_dn9,)
    }
};
        locals.var_tp = assign46960_e60241;
        locals.var_tp_dn4 = assign46960_e60241_d_n4;
        locals.var_tp_dn6 = assign46960_e60241_d_n6;
        locals.var_tp_dn7 = assign46960_e60241_d_n7;
        locals.var_tp_dn8 = assign46960_e60241_d_n8;
        locals.var_tp_dn9 = assign46960_e60241_d_n9;

        let (assign46970_e60258, assign46970_e60258_d_n4, assign46970_e60258_d_n6, assign46970_e60258_d_n7, assign46970_e60258_d_n8, assign46970_e60258_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign46970_e60249: f64 = (1.0 + locals.var_dsi);
        let assign46970_e60252: f64 = (1.0 + locals.var_dgate);
        let assign46970_e60253: f64 = (assign46970_e60249 / assign46970_e60252);
        let assign46970_e60254: f64 = (assign46970_e60253).ln();
        let assign46970_e60255: f64 = (locals.var_tp * assign46970_e60254);
        let assign46970_e60256: f64 = (locals.var_iginv_i * assign46970_e60255);
        (assign46970_e60256, (locals.var_iginv_i * ((locals.var_tp_dn4 * assign46970_e60254) + (locals.var_tp * ((((locals.var_dsi_dn4 * assign46970_e60252) - (assign46970_e60249 * locals.var_dgate_dn4)) / (assign46970_e60252 * assign46970_e60252)) / assign46970_e60253)))), (locals.var_iginv_i * ((locals.var_tp_dn6 * assign46970_e60254) + (locals.var_tp * ((((locals.var_dsi_dn6 * assign46970_e60252) - (assign46970_e60249 * locals.var_dgate_dn6)) / (assign46970_e60252 * assign46970_e60252)) / assign46970_e60253)))), (locals.var_iginv_i * ((locals.var_tp_dn7 * assign46970_e60254) + (locals.var_tp * ((((locals.var_dsi_dn7 * assign46970_e60252) - (assign46970_e60249 * locals.var_dgate_dn7)) / (assign46970_e60252 * assign46970_e60252)) / assign46970_e60253)))), (locals.var_iginv_i * ((locals.var_tp_dn8 * assign46970_e60254) + (locals.var_tp * ((((locals.var_dsi_dn8 * assign46970_e60252) - (assign46970_e60249 * locals.var_dgate_dn8)) / (assign46970_e60252 * assign46970_e60252)) / assign46970_e60253)))), (locals.var_iginv_i * ((locals.var_tp_dn9 * assign46970_e60254) + (locals.var_tp * ((((locals.var_dsi_dn9 * assign46970_e60252) - (assign46970_e60249 * locals.var_dgate_dn9)) / (assign46970_e60252 * assign46970_e60252)) / assign46970_e60253)))),)
    } else {
        (locals.var_igc0, locals.var_igc0_dn4, locals.var_igc0_dn6, locals.var_igc0_dn7, locals.var_igc0_dn8, locals.var_igc0_dn9,)
    }
};
        locals.var_igc0 = assign46970_e60258;
        locals.var_igc0_dn4 = assign46970_e60258_d_n4;
        locals.var_igc0_dn6 = assign46970_e60258_d_n6;
        locals.var_igc0_dn7 = assign46970_e60258_d_n7;
        locals.var_igc0_dn8 = assign46970_e60258_d_n8;
        locals.var_igc0_dn9 = assign46970_e60258_d_n9;

        let assign46980_e60269: f64 = if ((locals.var_xg_dc <= 0.0) || ((locals.var_gc2_i == 0.0) && (locals.var_gc3_i == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard1257 = assign46980_e60269;

        let (assign46990_e60277, assign46990_e60277_d_n4, assign46990_e60277_d_n6, assign46990_e60277_d_n7, assign46990_e60277_d_n8, assign46990_e60277_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igc, locals.var_igc_dn4, locals.var_igc_dn6, locals.var_igc_dn7, locals.var_igc_dn8, locals.var_igc_dn9,)
    }
};
        locals.var_igc = assign46990_e60277;
        locals.var_igc_dn4 = assign46990_e60277_d_n4;
        locals.var_igc_dn6 = assign46990_e60277_d_n6;
        locals.var_igc_dn7 = assign46990_e60277_d_n7;
        locals.var_igc_dn8 = assign46990_e60277_d_n8;
        locals.var_igc_dn9 = assign46990_e60277_d_n9;

        let (assign47000_e60285, assign47000_e60285_d_n4, assign47000_e60285_d_n6, assign47000_e60285_d_n7, assign47000_e60285_d_n8, assign47000_e60285_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 != 0.0)) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igcd_h, locals.var_igcd_h_dn4, locals.var_igcd_h_dn6, locals.var_igcd_h_dn7, locals.var_igcd_h_dn8, locals.var_igcd_h_dn9,)
    }
};
        locals.var_igcd_h = assign47000_e60285;
        locals.var_igcd_h_dn4 = assign47000_e60285_d_n4;
        locals.var_igcd_h_dn6 = assign47000_e60285_d_n6;
        locals.var_igcd_h_dn7 = assign47000_e60285_d_n7;
        locals.var_igcd_h_dn8 = assign47000_e60285_d_n8;
        locals.var_igcd_h_dn9 = assign47000_e60285_d_n9;

        let (assign47010_e60300, assign47010_e60300_d_n4, assign47010_e60300_d_n6, assign47010_e60300_d_n7, assign47010_e60300_d_n8, assign47010_e60300_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) {
        let assign47010_e60295: f64 = (2.0 * locals.var_gc3_i);
        let assign47010_e60297: f64 = (assign47010_e60295 * locals.var_zg);
        let assign47010_e60298: f64 = (locals.var_gc2_i + assign47010_e60297);
        (assign47010_e60298, (assign47010_e60295 * locals.var_zg_dn4), (assign47010_e60295 * locals.var_zg_dn6), (assign47010_e60295 * locals.var_zg_dn7), (assign47010_e60295 * locals.var_zg_dn8), (assign47010_e60295 * locals.var_zg_dn9),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign47010_e60300;
        locals.var_temp__blk949_dn4 = assign47010_e60300_d_n4;
        locals.var_temp__blk949_dn6 = assign47010_e60300_d_n6;
        locals.var_temp__blk949_dn7 = assign47010_e60300_d_n7;
        locals.var_temp__blk949_dn8 = assign47010_e60300_d_n8;
        locals.var_temp__blk949_dn9 = assign47010_e60300_d_n9;

        let (assign47020_e60313, assign47020_e60313_d_n4, assign47020_e60313_d_n6, assign47020_e60313_d_n7, assign47020_e60313_d_n8, assign47020_e60313_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) {
        let assign47020_e60310: f64 = (locals.var_temp__blk949 * locals.var_bch);
        let assign47020_e60311: f64 = (locals.var_chib_i / assign47020_e60310);
        (assign47020_e60311, (-((locals.var_chib_i * (locals.var_temp__blk949_dn4 * locals.var_bch)) / (assign47020_e60310 * assign47020_e60310))), (-((locals.var_chib_i * (locals.var_temp__blk949_dn6 * locals.var_bch)) / (assign47020_e60310 * assign47020_e60310))), (-((locals.var_chib_i * (locals.var_temp__blk949_dn7 * locals.var_bch)) / (assign47020_e60310 * assign47020_e60310))), (-((locals.var_chib_i * (locals.var_temp__blk949_dn8 * locals.var_bch)) / (assign47020_e60310 * assign47020_e60310))), (-((locals.var_chib_i * (locals.var_temp__blk949_dn9 * locals.var_bch)) / (assign47020_e60310 * assign47020_e60310))),)
    } else {
        (locals.var_u0, locals.var_u0_dn4, locals.var_u0_dn6, locals.var_u0_dn7, locals.var_u0_dn8, locals.var_u0_dn9,)
    }
};
        locals.var_u0 = assign47020_e60313;
        locals.var_u0_dn4 = assign47020_e60313_d_n4;
        locals.var_u0_dn6 = assign47020_e60313_d_n6;
        locals.var_u0_dn7 = assign47020_e60313_d_n7;
        locals.var_u0_dn8 = assign47020_e60313_d_n8;
        locals.var_u0_dn9 = assign47020_e60313_d_n9;

        let (assign47030_e60326, assign47030_e60326_d_n4, assign47030_e60326_d_n6, assign47030_e60326_d_n7, assign47030_e60326_d_n8, assign47030_e60326_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) {
        let assign47030_e60323: f64 = (locals.var_dps_dc / locals.var_u0);
        let assign47030_e60324: f64 = (0.5 * assign47030_e60323);
        (assign47030_e60324, (0.5 * (((locals.var_dps_dc_dn4 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn4)) / (locals.var_u0 * locals.var_u0))), (0.5 * (((locals.var_dps_dc_dn6 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn6)) / (locals.var_u0 * locals.var_u0))), (0.5 * (((locals.var_dps_dc_dn7 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn7)) / (locals.var_u0 * locals.var_u0))), (0.5 * (((locals.var_dps_dc_dn8 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn8)) / (locals.var_u0 * locals.var_u0))), (0.5 * (((locals.var_dps_dc_dn9 * locals.var_u0) - (locals.var_dps_dc * locals.var_u0_dn9)) / (locals.var_u0 * locals.var_u0))),)
    } else {
        (locals.var_x, locals.var_x_dn4, locals.var_x_dn6, locals.var_x_dn7, locals.var_x_dn8, locals.var_x_dn9,)
    }
};
        locals.var_x = assign47030_e60326;
        locals.var_x_dn4 = assign47030_e60326_d_n4;
        locals.var_x_dn6 = assign47030_e60326_d_n6;
        locals.var_x_dn7 = assign47030_e60326_d_n7;
        locals.var_x_dn8 = assign47030_e60326_d_n8;
        locals.var_x_dn9 = assign47030_e60326_d_n9;

        let (assign47040_e60337, assign47040_e60337_d_n4, assign47040_e60337_d_n6, assign47040_e60337_d_n7, assign47040_e60337_d_n8, assign47040_e60337_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) {
        let assign47040_e60335: f64 = (locals.var_u0 / locals.var_h_dc);
        (assign47040_e60335, (((locals.var_u0_dn4 * locals.var_h_dc) - (locals.var_u0 * locals.var_h_dc_dn4)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_u0_dn6 * locals.var_h_dc) - (locals.var_u0 * locals.var_h_dc_dn6)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_u0_dn7 * locals.var_h_dc) - (locals.var_u0 * locals.var_h_dc_dn7)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_u0_dn8 * locals.var_h_dc) - (locals.var_u0 * locals.var_h_dc_dn8)) / (locals.var_h_dc * locals.var_h_dc)), (((locals.var_u0_dn9 * locals.var_h_dc) - (locals.var_u0 * locals.var_h_dc_dn9)) / (locals.var_h_dc * locals.var_h_dc)),)
    } else {
        (locals.var_u0_div_h, locals.var_u0_div_h_dn4, locals.var_u0_div_h_dn6, locals.var_u0_div_h_dn7, locals.var_u0_div_h_dn8, locals.var_u0_div_h_dn9,)
    }
};
        locals.var_u0_div_h = assign47040_e60337;
        locals.var_u0_div_h_dn4 = assign47040_e60337_d_n4;
        locals.var_u0_div_h_dn6 = assign47040_e60337_d_n6;
        locals.var_u0_div_h_dn7 = assign47040_e60337_d_n7;
        locals.var_u0_div_h_dn8 = assign47040_e60337_d_n8;
        locals.var_u0_div_h_dn9 = assign47040_e60337_d_n9;

        let (assign47050_e60352, assign47050_e60352_d_n4, assign47050_e60352_d_n6, assign47050_e60352_d_n7, assign47050_e60352_d_n8, assign47050_e60352_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) {
        let assign47050_e60347: f64 = (1.0 - locals.var_u0_div_h);
        let assign47050_e60348: f64 = (locals.var_u0_div_h * assign47050_e60347);
        let assign47050_e60350: f64 = (assign47050_e60348 * 0.5);
        (assign47050_e60350, (((locals.var_u0_div_h_dn4 * assign47050_e60347) + (locals.var_u0_div_h * (-locals.var_u0_div_h_dn4))) * 0.5), (((locals.var_u0_div_h_dn6 * assign47050_e60347) + (locals.var_u0_div_h * (-locals.var_u0_div_h_dn6))) * 0.5), (((locals.var_u0_div_h_dn7 * assign47050_e60347) + (locals.var_u0_div_h * (-locals.var_u0_div_h_dn7))) * 0.5), (((locals.var_u0_div_h_dn8 * assign47050_e60347) + (locals.var_u0_div_h * (-locals.var_u0_div_h_dn8))) * 0.5), (((locals.var_u0_div_h_dn9 * assign47050_e60347) + (locals.var_u0_div_h * (-locals.var_u0_div_h_dn9))) * 0.5),)
    } else {
        (locals.var_bg, locals.var_bg_dn4, locals.var_bg_dn6, locals.var_bg_dn7, locals.var_bg_dn8, locals.var_bg_dn9,)
    }
};
        locals.var_bg = assign47050_e60352;
        locals.var_bg_dn4 = assign47050_e60352_d_n4;
        locals.var_bg_dn6 = assign47050_e60352_d_n6;
        locals.var_bg_dn7 = assign47050_e60352_d_n7;
        locals.var_bg_dn8 = assign47050_e60352_d_n8;
        locals.var_bg_dn9 = assign47050_e60352_d_n9;

        let (assign47060_e60365, assign47060_e60365_d_n4, assign47060_e60365_d_n6, assign47060_e60365_d_n7, assign47060_e60365_d_n8, assign47060_e60365_d_n9,) = {
    if (((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) {
        let assign47060_e60362: f64 = (3.0 * locals.var_bg);
        let assign47060_e60363: f64 = (0.5 - assign47060_e60362);
        (assign47060_e60363, (-(3.0 * locals.var_bg_dn4)), (-(3.0 * locals.var_bg_dn6)), (-(3.0 * locals.var_bg_dn7)), (-(3.0 * locals.var_bg_dn8)), (-(3.0 * locals.var_bg_dn9)),)
    } else {
        (locals.var_ag, locals.var_ag_dn4, locals.var_ag_dn6, locals.var_ag_dn7, locals.var_ag_dn8, locals.var_ag_dn9,)
    }
};
        locals.var_ag = assign47060_e60365;
        locals.var_ag_dn4 = assign47060_e60365_d_n4;
        locals.var_ag_dn6 = assign47060_e60365_d_n6;
        locals.var_ag_dn7 = assign47060_e60365_d_n7;
        locals.var_ag_dn8 = assign47060_e60365_d_n8;
        locals.var_ag_dn9 = assign47060_e60365_d_n9;

        let assign47070_e60368: f64 = if locals.var_x < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard1258 = assign47070_e60368;

        let (assign47080_e60381, assign47080_e60381_d_n4, assign47080_e60381_d_n6, assign47080_e60381_d_n7, assign47080_e60381_d_n8, assign47080_e60381_d_n9,) = {
    if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) && (locals.var_guard1258 != 0.0)) {
        let assign47080_e60379: f64 = (locals.var_x * locals.var_x);
        (assign47080_e60379, ((locals.var_x_dn4 * locals.var_x) + (locals.var_x * locals.var_x_dn4)), ((locals.var_x_dn6 * locals.var_x) + (locals.var_x * locals.var_x_dn6)), ((locals.var_x_dn7 * locals.var_x) + (locals.var_x * locals.var_x_dn7)), ((locals.var_x_dn8 * locals.var_x) + (locals.var_x * locals.var_x_dn8)), ((locals.var_x_dn9 * locals.var_x) + (locals.var_x * locals.var_x_dn9)),)
    } else {
        (locals.var_xsq, locals.var_xsq_dn4, locals.var_xsq_dn6, locals.var_xsq_dn7, locals.var_xsq_dn8, locals.var_xsq_dn9,)
    }
};
        locals.var_xsq = assign47080_e60381;
        locals.var_xsq_dn4 = assign47080_e60381_d_n4;
        locals.var_xsq_dn6 = assign47080_e60381_d_n6;
        locals.var_xsq_dn7 = assign47080_e60381_d_n7;
        locals.var_xsq_dn8 = assign47080_e60381_d_n8;
        locals.var_xsq_dn9 = assign47080_e60381_d_n9;

        let (assign47090_e60410, assign47090_e60410_d_n4, assign47090_e60410_d_n6, assign47090_e60410_d_n7, assign47090_e60410_d_n8, assign47090_e60410_d_n9,) = {
    if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) && (locals.var_guard1258 != 0.0)) {
        let assign47090_e60395: f64 = (locals.var_u0_div_h * 0.3333333333333333);
        let assign47090_e60396: f64 = (0.16666666666666666 + assign47090_e60395);
        let assign47090_e60402: f64 = (0.2 * locals.var_u0_div_h);
        let assign47090_e60403: f64 = (0.05 + assign47090_e60402);
        let assign47090_e60404: f64 = (locals.var_xsq * assign47090_e60403);
        let assign47090_e60405: f64 = (0.16666666666666666 * assign47090_e60404);
        let assign47090_e60406: f64 = (assign47090_e60396 + assign47090_e60405);
        let assign47090_e60407: f64 = (locals.var_xsq * assign47090_e60406);
        let assign47090_e60408: f64 = (1.0 + assign47090_e60407);
        (assign47090_e60408, ((locals.var_xsq_dn4 * assign47090_e60406) + (locals.var_xsq * ((locals.var_u0_div_h_dn4 * 0.3333333333333333) + (0.16666666666666666 * ((locals.var_xsq_dn4 * assign47090_e60403) + (locals.var_xsq * (0.2 * locals.var_u0_div_h_dn4))))))), ((locals.var_xsq_dn6 * assign47090_e60406) + (locals.var_xsq * ((locals.var_u0_div_h_dn6 * 0.3333333333333333) + (0.16666666666666666 * ((locals.var_xsq_dn6 * assign47090_e60403) + (locals.var_xsq * (0.2 * locals.var_u0_div_h_dn6))))))), ((locals.var_xsq_dn7 * assign47090_e60406) + (locals.var_xsq * ((locals.var_u0_div_h_dn7 * 0.3333333333333333) + (0.16666666666666666 * ((locals.var_xsq_dn7 * assign47090_e60403) + (locals.var_xsq * (0.2 * locals.var_u0_div_h_dn7))))))), ((locals.var_xsq_dn8 * assign47090_e60406) + (locals.var_xsq * ((locals.var_u0_div_h_dn8 * 0.3333333333333333) + (0.16666666666666666 * ((locals.var_xsq_dn8 * assign47090_e60403) + (locals.var_xsq * (0.2 * locals.var_u0_div_h_dn8))))))), ((locals.var_xsq_dn9 * assign47090_e60406) + (locals.var_xsq * ((locals.var_u0_div_h_dn9 * 0.3333333333333333) + (0.16666666666666666 * ((locals.var_xsq_dn9 * assign47090_e60403) + (locals.var_xsq * (0.2 * locals.var_u0_div_h_dn9))))))),)
    } else {
        (locals.var_igc, locals.var_igc_dn4, locals.var_igc_dn6, locals.var_igc_dn7, locals.var_igc_dn8, locals.var_igc_dn9,)
    }
};
        locals.var_igc = assign47090_e60410;
        locals.var_igc_dn4 = assign47090_e60410_d_n4;
        locals.var_igc_dn6 = assign47090_e60410_d_n6;
        locals.var_igc_dn7 = assign47090_e60410_d_n7;
        locals.var_igc_dn8 = assign47090_e60410_d_n8;
        locals.var_igc_dn9 = assign47090_e60410_d_n9;

        let (assign47100_e60445, assign47100_e60445_d_n4, assign47100_e60445_d_n6, assign47100_e60445_d_n7, assign47100_e60445_d_n8, assign47100_e60445_d_n9,) = {
    if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) && (locals.var_guard1258 != 0.0)) {
        let assign47100_e60421: f64 = (0.5 * locals.var_igc);
        let assign47100_e60429: f64 = (locals.var_bg + 0.25);
        let assign47100_e60430: f64 = (0.4 * assign47100_e60429);
        let assign47100_e60435: f64 = (0.125 + locals.var_bg);
        let assign47100_e60436: f64 = (locals.var_xsq * assign47100_e60435);
        let assign47100_e60437: f64 = (0.0285714285714 * assign47100_e60436);
        let assign47100_e60438: f64 = (assign47100_e60430 + assign47100_e60437);
        let assign47100_e60439: f64 = (locals.var_xsq * assign47100_e60438);
        let assign47100_e60440: f64 = (1.0 + assign47100_e60439);
        let assign47100_e60441: f64 = (locals.var_x * assign47100_e60440);
        let assign47100_e60442: f64 = (0.16666666666666666 * assign47100_e60441);
        let assign47100_e60443: f64 = (assign47100_e60421 - assign47100_e60442);
        (assign47100_e60443, ((0.5 * locals.var_igc_dn4) - (0.16666666666666666 * ((locals.var_x_dn4 * assign47100_e60440) + (locals.var_x * ((locals.var_xsq_dn4 * assign47100_e60438) + (locals.var_xsq * ((0.4 * locals.var_bg_dn4) + (0.0285714285714 * ((locals.var_xsq_dn4 * assign47100_e60435) + (locals.var_xsq * locals.var_bg_dn4)))))))))), ((0.5 * locals.var_igc_dn6) - (0.16666666666666666 * ((locals.var_x_dn6 * assign47100_e60440) + (locals.var_x * ((locals.var_xsq_dn6 * assign47100_e60438) + (locals.var_xsq * ((0.4 * locals.var_bg_dn6) + (0.0285714285714 * ((locals.var_xsq_dn6 * assign47100_e60435) + (locals.var_xsq * locals.var_bg_dn6)))))))))), ((0.5 * locals.var_igc_dn7) - (0.16666666666666666 * ((locals.var_x_dn7 * assign47100_e60440) + (locals.var_x * ((locals.var_xsq_dn7 * assign47100_e60438) + (locals.var_xsq * ((0.4 * locals.var_bg_dn7) + (0.0285714285714 * ((locals.var_xsq_dn7 * assign47100_e60435) + (locals.var_xsq * locals.var_bg_dn7)))))))))), ((0.5 * locals.var_igc_dn8) - (0.16666666666666666 * ((locals.var_x_dn8 * assign47100_e60440) + (locals.var_x * ((locals.var_xsq_dn8 * assign47100_e60438) + (locals.var_xsq * ((0.4 * locals.var_bg_dn8) + (0.0285714285714 * ((locals.var_xsq_dn8 * assign47100_e60435) + (locals.var_xsq * locals.var_bg_dn8)))))))))), ((0.5 * locals.var_igc_dn9) - (0.16666666666666666 * ((locals.var_x_dn9 * assign47100_e60440) + (locals.var_x * ((locals.var_xsq_dn9 * assign47100_e60438) + (locals.var_xsq * ((0.4 * locals.var_bg_dn9) + (0.0285714285714 * ((locals.var_xsq_dn9 * assign47100_e60435) + (locals.var_xsq * locals.var_bg_dn9)))))))))),)
    } else {
        (locals.var_igcd_h, locals.var_igcd_h_dn4, locals.var_igcd_h_dn6, locals.var_igcd_h_dn7, locals.var_igcd_h_dn8, locals.var_igcd_h_dn9,)
    }
};
        locals.var_igcd_h = assign47100_e60445;
        locals.var_igcd_h_dn4 = assign47100_e60445_d_n4;
        locals.var_igcd_h_dn6 = assign47100_e60445_d_n6;
        locals.var_igcd_h_dn7 = assign47100_e60445_d_n7;
        locals.var_igcd_h_dn8 = assign47100_e60445_d_n8;
        locals.var_igcd_h_dn9 = assign47100_e60445_d_n9;

        let (assign47110_e60459, assign47110_e60459_d_n4, assign47110_e60459_d_n6, assign47110_e60459_d_n7, assign47110_e60459_d_n8, assign47110_e60459_d_n9,) = {
    if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) && (locals.var_guard1258 == 0.0)) {
        let assign47110_e60457: f64 = (1.0 / locals.var_x);
        (assign47110_e60457, (-(locals.var_x_dn4 / (locals.var_x * locals.var_x))), (-(locals.var_x_dn6 / (locals.var_x * locals.var_x))), (-(locals.var_x_dn7 / (locals.var_x * locals.var_x))), (-(locals.var_x_dn8 / (locals.var_x * locals.var_x))), (-(locals.var_x_dn9 / (locals.var_x * locals.var_x))),)
    } else {
        (locals.var_inv_x, locals.var_inv_x_dn4, locals.var_inv_x_dn6, locals.var_inv_x_dn7, locals.var_inv_x_dn8, locals.var_inv_x_dn9,)
    }
};
        locals.var_inv_x = assign47110_e60459;
        locals.var_inv_x_dn4 = assign47110_e60459_d_n4;
        locals.var_inv_x_dn6 = assign47110_e60459_d_n6;
        locals.var_inv_x_dn7 = assign47110_e60459_d_n7;
        locals.var_inv_x_dn8 = assign47110_e60459_d_n8;
        locals.var_inv_x_dn9 = assign47110_e60459_d_n9;

        let assign47120_e60461: f64 = (locals.var_x).abs();
        let assign47120_e60463: f64 = if assign47120_e60461 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1259 = assign47120_e60463;

        let (assign47130_e60478, assign47130_e60478_d_n4, assign47130_e60478_d_n6, assign47130_e60478_d_n7, assign47130_e60478_d_n8, assign47130_e60478_d_n9,) = {
    if (((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) && (locals.var_guard1258 == 0.0)) && (locals.var_guard1259 != 0.0)) {
        let assign47130_e60476: f64 = (locals.var_x).exp();
        (assign47130_e60476, (assign47130_e60476 * locals.var_x_dn4), (assign47130_e60476 * locals.var_x_dn6), (assign47130_e60476 * locals.var_x_dn7), (assign47130_e60476 * locals.var_x_dn8), (assign47130_e60476 * locals.var_x_dn9),)
    } else {
        (locals.var_ex, locals.var_ex_dn4, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8, locals.var_ex_dn9,)
    }
};
        locals.var_ex = assign47130_e60478;
        locals.var_ex_dn4 = assign47130_e60478_d_n4;
        locals.var_ex_dn6 = assign47130_e60478_d_n6;
        locals.var_ex_dn7 = assign47130_e60478_d_n7;
        locals.var_ex_dn8 = assign47130_e60478_d_n8;
        locals.var_ex_dn9 = assign47130_e60478_d_n9;

        let assign47140_e60481: f64 = if locals.var_x < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1260 = assign47140_e60481;

        let (assign47150_e60523, assign47150_e60523_d_n4, assign47150_e60523_d_n6, assign47150_e60523_d_n7, assign47150_e60523_d_n8, assign47150_e60523_d_n9,) = {
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
        (assign47150_e60521, (-((1e-100 * (((-locals.var_x_dn4) * assign47150_e60518) + (assign47150_e60501 * (0.5 * (((-locals.var_x_dn4) * assign47150_e60515) + (assign47150_e60507 * ((-locals.var_x_dn4) * 0.3333333333333333))))))) / (assign47150_e60520 * assign47150_e60520))), (-((1e-100 * (((-locals.var_x_dn6) * assign47150_e60518) + (assign47150_e60501 * (0.5 * (((-locals.var_x_dn6) * assign47150_e60515) + (assign47150_e60507 * ((-locals.var_x_dn6) * 0.3333333333333333))))))) / (assign47150_e60520 * assign47150_e60520))), (-((1e-100 * (((-locals.var_x_dn7) * assign47150_e60518) + (assign47150_e60501 * (0.5 * (((-locals.var_x_dn7) * assign47150_e60515) + (assign47150_e60507 * ((-locals.var_x_dn7) * 0.3333333333333333))))))) / (assign47150_e60520 * assign47150_e60520))), (-((1e-100 * (((-locals.var_x_dn8) * assign47150_e60518) + (assign47150_e60501 * (0.5 * (((-locals.var_x_dn8) * assign47150_e60515) + (assign47150_e60507 * ((-locals.var_x_dn8) * 0.3333333333333333))))))) / (assign47150_e60520 * assign47150_e60520))), (-((1e-100 * (((-locals.var_x_dn9) * assign47150_e60518) + (assign47150_e60501 * (0.5 * (((-locals.var_x_dn9) * assign47150_e60515) + (assign47150_e60507 * ((-locals.var_x_dn9) * 0.3333333333333333))))))) / (assign47150_e60520 * assign47150_e60520))),)
    } else {
        (locals.var_ex, locals.var_ex_dn4, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8, locals.var_ex_dn9,)
    }
};
        locals.var_ex = assign47150_e60523;
        locals.var_ex_dn4 = assign47150_e60523_d_n4;
        locals.var_ex_dn6 = assign47150_e60523_d_n6;
        locals.var_ex_dn7 = assign47150_e60523_d_n7;
        locals.var_ex_dn8 = assign47150_e60523_d_n8;
        locals.var_ex_dn9 = assign47150_e60523_d_n9;

        let (assign47160_e60563, assign47160_e60563_d_n4, assign47160_e60563_d_n6, assign47160_e60563_d_n7, assign47160_e60563_d_n8, assign47160_e60563_d_n9,) = {
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
        (assign47160_e60561, (1e100 * ((locals.var_x_dn4 * assign47160_e60558) + (assign47160_e60543 * (0.5 * ((locals.var_x_dn4 * assign47160_e60555) + (assign47160_e60548 * (locals.var_x_dn4 * 0.3333333333333333))))))), (1e100 * ((locals.var_x_dn6 * assign47160_e60558) + (assign47160_e60543 * (0.5 * ((locals.var_x_dn6 * assign47160_e60555) + (assign47160_e60548 * (locals.var_x_dn6 * 0.3333333333333333))))))), (1e100 * ((locals.var_x_dn7 * assign47160_e60558) + (assign47160_e60543 * (0.5 * ((locals.var_x_dn7 * assign47160_e60555) + (assign47160_e60548 * (locals.var_x_dn7 * 0.3333333333333333))))))), (1e100 * ((locals.var_x_dn8 * assign47160_e60558) + (assign47160_e60543 * (0.5 * ((locals.var_x_dn8 * assign47160_e60555) + (assign47160_e60548 * (locals.var_x_dn8 * 0.3333333333333333))))))), (1e100 * ((locals.var_x_dn9 * assign47160_e60558) + (assign47160_e60543 * (0.5 * ((locals.var_x_dn9 * assign47160_e60555) + (assign47160_e60548 * (locals.var_x_dn9 * 0.3333333333333333))))))),)
    } else {
        (locals.var_ex, locals.var_ex_dn4, locals.var_ex_dn6, locals.var_ex_dn7, locals.var_ex_dn8, locals.var_ex_dn9,)
    }
};
        locals.var_ex = assign47160_e60563;
        locals.var_ex_dn4 = assign47160_e60563_d_n4;
        locals.var_ex_dn6 = assign47160_e60563_d_n6;
        locals.var_ex_dn7 = assign47160_e60563_d_n7;
        locals.var_ex_dn8 = assign47160_e60563_d_n8;
        locals.var_ex_dn9 = assign47160_e60563_d_n9;

        let (assign47170_e60577, assign47170_e60577_d_n4, assign47170_e60577_d_n6, assign47170_e60577_d_n7, assign47170_e60577_d_n8, assign47170_e60577_d_n9,) = {
    if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) && (locals.var_guard1258 == 0.0)) {
        let assign47170_e60575: f64 = (1.0 / locals.var_ex);
        (assign47170_e60575, (-(locals.var_ex_dn4 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn6 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn7 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn8 / (locals.var_ex * locals.var_ex))), (-(locals.var_ex_dn9 / (locals.var_ex * locals.var_ex))),)
    } else {
        (locals.var_inv_ex, locals.var_inv_ex_dn4, locals.var_inv_ex_dn6, locals.var_inv_ex_dn7, locals.var_inv_ex_dn8, locals.var_inv_ex_dn9,)
    }
};
        locals.var_inv_ex = assign47170_e60577;
        locals.var_inv_ex_dn4 = assign47170_e60577_d_n4;
        locals.var_inv_ex_dn6 = assign47170_e60577_d_n6;
        locals.var_inv_ex_dn7 = assign47170_e60577_d_n7;
        locals.var_inv_ex_dn8 = assign47170_e60577_d_n8;
        locals.var_inv_ex_dn9 = assign47170_e60577_d_n9;

        let (assign47180_e60591, assign47180_e60591_d_n4, assign47180_e60591_d_n6, assign47180_e60591_d_n7, assign47180_e60591_d_n8, assign47180_e60591_d_n9,) = {
    if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) && (locals.var_guard1258 == 0.0)) {
        let assign47180_e60589: f64 = (locals.var_ex - locals.var_inv_ex);
        (assign47180_e60589, (locals.var_ex_dn4 - locals.var_inv_ex_dn4), (locals.var_ex_dn6 - locals.var_inv_ex_dn6), (locals.var_ex_dn7 - locals.var_inv_ex_dn7), (locals.var_ex_dn8 - locals.var_inv_ex_dn8), (locals.var_ex_dn9 - locals.var_inv_ex_dn9),)
    } else {
        (locals.var_temp__blk949, locals.var_temp__blk949_dn4, locals.var_temp__blk949_dn6, locals.var_temp__blk949_dn7, locals.var_temp__blk949_dn8, locals.var_temp__blk949_dn9,)
    }
};
        locals.var_temp__blk949 = assign47180_e60591;
        locals.var_temp__blk949_dn4 = assign47180_e60591_d_n4;
        locals.var_temp__blk949_dn6 = assign47180_e60591_d_n6;
        locals.var_temp__blk949_dn7 = assign47180_e60591_d_n7;
        locals.var_temp__blk949_dn8 = assign47180_e60591_d_n8;
        locals.var_temp__blk949_dn9 = assign47180_e60591_d_n9;

        let (assign47190_e60605, assign47190_e60605_d_n4, assign47190_e60605_d_n6, assign47190_e60605_d_n7, assign47190_e60605_d_n8, assign47190_e60605_d_n9,) = {
    if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) && (locals.var_guard1258 == 0.0)) {
        let assign47190_e60603: f64 = (locals.var_ex + locals.var_inv_ex);
        (assign47190_e60603, (locals.var_ex_dn4 + locals.var_inv_ex_dn4), (locals.var_ex_dn6 + locals.var_inv_ex_dn6), (locals.var_ex_dn7 + locals.var_inv_ex_dn7), (locals.var_ex_dn8 + locals.var_inv_ex_dn8), (locals.var_ex_dn9 + locals.var_inv_ex_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign47190_e60605;
        locals.var_temp2_dn4 = assign47190_e60605_d_n4;
        locals.var_temp2_dn6 = assign47190_e60605_d_n6;
        locals.var_temp2_dn7 = assign47190_e60605_d_n7;
        locals.var_temp2_dn8 = assign47190_e60605_d_n8;
        locals.var_temp2_dn9 = assign47190_e60605_d_n9;

        let (assign47200_e60629, assign47200_e60629_d_n4, assign47200_e60629_d_n6, assign47200_e60629_d_n7, assign47200_e60629_d_n8, assign47200_e60629_d_n9,) = {
    if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) && (locals.var_guard1258 == 0.0)) {
        let assign47200_e60618: f64 = (1.0 - locals.var_u0_div_h);
        let assign47200_e60620: f64 = (assign47200_e60618 * locals.var_temp__blk949);
        let assign47200_e60622: f64 = (assign47200_e60620 * locals.var_inv_x);
        let assign47200_e60625: f64 = (locals.var_u0_div_h * locals.var_temp2);
        let assign47200_e60626: f64 = (assign47200_e60622 + assign47200_e60625);
        let assign47200_e60627: f64 = (0.5 * assign47200_e60626);
        (assign47200_e60627, (0.5 * ((((((-locals.var_u0_div_h_dn4) * locals.var_temp__blk949) + (assign47200_e60618 * locals.var_temp__blk949_dn4)) * locals.var_inv_x) + (assign47200_e60620 * locals.var_inv_x_dn4)) + ((locals.var_u0_div_h_dn4 * locals.var_temp2) + (locals.var_u0_div_h * locals.var_temp2_dn4)))), (0.5 * ((((((-locals.var_u0_div_h_dn6) * locals.var_temp__blk949) + (assign47200_e60618 * locals.var_temp__blk949_dn6)) * locals.var_inv_x) + (assign47200_e60620 * locals.var_inv_x_dn6)) + ((locals.var_u0_div_h_dn6 * locals.var_temp2) + (locals.var_u0_div_h * locals.var_temp2_dn6)))), (0.5 * ((((((-locals.var_u0_div_h_dn7) * locals.var_temp__blk949) + (assign47200_e60618 * locals.var_temp__blk949_dn7)) * locals.var_inv_x) + (assign47200_e60620 * locals.var_inv_x_dn7)) + ((locals.var_u0_div_h_dn7 * locals.var_temp2) + (locals.var_u0_div_h * locals.var_temp2_dn7)))), (0.5 * ((((((-locals.var_u0_div_h_dn8) * locals.var_temp__blk949) + (assign47200_e60618 * locals.var_temp__blk949_dn8)) * locals.var_inv_x) + (assign47200_e60620 * locals.var_inv_x_dn8)) + ((locals.var_u0_div_h_dn8 * locals.var_temp2) + (locals.var_u0_div_h * locals.var_temp2_dn8)))), (0.5 * ((((((-locals.var_u0_div_h_dn9) * locals.var_temp__blk949) + (assign47200_e60618 * locals.var_temp__blk949_dn9)) * locals.var_inv_x) + (assign47200_e60620 * locals.var_inv_x_dn9)) + ((locals.var_u0_div_h_dn9 * locals.var_temp2) + (locals.var_u0_div_h * locals.var_temp2_dn9)))),)
    } else {
        (locals.var_igc, locals.var_igc_dn4, locals.var_igc_dn6, locals.var_igc_dn7, locals.var_igc_dn8, locals.var_igc_dn9,)
    }
};
        locals.var_igc = assign47200_e60629;
        locals.var_igc_dn4 = assign47200_e60629_d_n4;
        locals.var_igc_dn6 = assign47200_e60629_d_n6;
        locals.var_igc_dn7 = assign47200_e60629_d_n7;
        locals.var_igc_dn8 = assign47200_e60629_d_n8;
        locals.var_igc_dn9 = assign47200_e60629_d_n9;

        let (assign47210_e60659, assign47210_e60659_d_n4, assign47210_e60659_d_n6, assign47210_e60659_d_n7, assign47210_e60659_d_n8, assign47210_e60659_d_n9,) = {
    if ((((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) && (locals.var_guard1257 == 0.0)) && (locals.var_guard1258 == 0.0)) {
        let assign47210_e60645: f64 = (locals.var_ag * locals.var_inv_x);
        let assign47210_e60647: f64 = (assign47210_e60645 * locals.var_inv_x);
        let assign47210_e60648: f64 = (locals.var_bg - assign47210_e60647);
        let assign47210_e60649: f64 = (locals.var_temp__blk949 * assign47210_e60648);
        let assign47210_e60650: f64 = (locals.var_igc - assign47210_e60649);
        let assign47210_e60653: f64 = (locals.var_ag * locals.var_temp2);
        let assign47210_e60655: f64 = (assign47210_e60653 * locals.var_inv_x);
        let assign47210_e60656: f64 = (assign47210_e60650 - assign47210_e60655);
        let assign47210_e60657: f64 = (0.5 * assign47210_e60656);
        (assign47210_e60657, (0.5 * ((locals.var_igc_dn4 - ((locals.var_temp__blk949_dn4 * assign47210_e60648) + (locals.var_temp__blk949 * (locals.var_bg_dn4 - ((((locals.var_ag_dn4 * locals.var_inv_x) + (locals.var_ag * locals.var_inv_x_dn4)) * locals.var_inv_x) + (assign47210_e60645 * locals.var_inv_x_dn4)))))) - ((((locals.var_ag_dn4 * locals.var_temp2) + (locals.var_ag * locals.var_temp2_dn4)) * locals.var_inv_x) + (assign47210_e60653 * locals.var_inv_x_dn4)))), (0.5 * ((locals.var_igc_dn6 - ((locals.var_temp__blk949_dn6 * assign47210_e60648) + (locals.var_temp__blk949 * (locals.var_bg_dn6 - ((((locals.var_ag_dn6 * locals.var_inv_x) + (locals.var_ag * locals.var_inv_x_dn6)) * locals.var_inv_x) + (assign47210_e60645 * locals.var_inv_x_dn6)))))) - ((((locals.var_ag_dn6 * locals.var_temp2) + (locals.var_ag * locals.var_temp2_dn6)) * locals.var_inv_x) + (assign47210_e60653 * locals.var_inv_x_dn6)))), (0.5 * ((locals.var_igc_dn7 - ((locals.var_temp__blk949_dn7 * assign47210_e60648) + (locals.var_temp__blk949 * (locals.var_bg_dn7 - ((((locals.var_ag_dn7 * locals.var_inv_x) + (locals.var_ag * locals.var_inv_x_dn7)) * locals.var_inv_x) + (assign47210_e60645 * locals.var_inv_x_dn7)))))) - ((((locals.var_ag_dn7 * locals.var_temp2) + (locals.var_ag * locals.var_temp2_dn7)) * locals.var_inv_x) + (assign47210_e60653 * locals.var_inv_x_dn7)))), (0.5 * ((locals.var_igc_dn8 - ((locals.var_temp__blk949_dn8 * assign47210_e60648) + (locals.var_temp__blk949 * (locals.var_bg_dn8 - ((((locals.var_ag_dn8 * locals.var_inv_x) + (locals.var_ag * locals.var_inv_x_dn8)) * locals.var_inv_x) + (assign47210_e60645 * locals.var_inv_x_dn8)))))) - ((((locals.var_ag_dn8 * locals.var_temp2) + (locals.var_ag * locals.var_temp2_dn8)) * locals.var_inv_x) + (assign47210_e60653 * locals.var_inv_x_dn8)))), (0.5 * ((locals.var_igc_dn9 - ((locals.var_temp__blk949_dn9 * assign47210_e60648) + (locals.var_temp__blk949 * (locals.var_bg_dn9 - ((((locals.var_ag_dn9 * locals.var_inv_x) + (locals.var_ag * locals.var_inv_x_dn9)) * locals.var_inv_x) + (assign47210_e60645 * locals.var_inv_x_dn9)))))) - ((((locals.var_ag_dn9 * locals.var_temp2) + (locals.var_ag * locals.var_temp2_dn9)) * locals.var_inv_x) + (assign47210_e60653 * locals.var_inv_x_dn9)))),)
    } else {
        (locals.var_igcd_h, locals.var_igcd_h_dn4, locals.var_igcd_h_dn6, locals.var_igcd_h_dn7, locals.var_igcd_h_dn8, locals.var_igcd_h_dn9,)
    }
};
        locals.var_igcd_h = assign47210_e60659;
        locals.var_igcd_h_dn4 = assign47210_e60659_d_n4;
        locals.var_igcd_h_dn6 = assign47210_e60659_d_n6;
        locals.var_igcd_h_dn7 = assign47210_e60659_d_n7;
        locals.var_igcd_h_dn8 = assign47210_e60659_d_n8;
        locals.var_igcd_h_dn9 = assign47210_e60659_d_n9;

        let (assign47220_e60676, assign47220_e60676_d_n4, assign47220_e60676_d_n6, assign47220_e60676_d_n7, assign47220_e60676_d_n8, assign47220_e60676_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign47220_e60668: f64 = (locals.var_xg_dc * locals.var_xg_dc);
        let assign47220_e60670: f64 = (assign47220_e60668 + 1e-6);
        let assign47220_e60671: f64 = (assign47220_e60670).sqrt();
        let assign47220_e60672: f64 = (locals.var_xg_dc / assign47220_e60671);
        let assign47220_e60673: f64 = (1.0 + assign47220_e60672);
        let assign47220_e60674: f64 = (0.5 * assign47220_e60673);
        (assign47220_e60674, (0.5 * (((locals.var_xg_dc_dn4 * assign47220_e60671) - (locals.var_xg_dc * (((locals.var_xg_dc_dn4 * locals.var_xg_dc) + (locals.var_xg_dc * locals.var_xg_dc_dn4)) / (2.0 * assign47220_e60671)))) / (assign47220_e60671 * assign47220_e60671))), (0.5 * (((locals.var_xg_dc_dn6 * assign47220_e60671) - (locals.var_xg_dc * (((locals.var_xg_dc_dn6 * locals.var_xg_dc) + (locals.var_xg_dc * locals.var_xg_dc_dn6)) / (2.0 * assign47220_e60671)))) / (assign47220_e60671 * assign47220_e60671))), (0.5 * (((locals.var_xg_dc_dn7 * assign47220_e60671) - (locals.var_xg_dc * (((locals.var_xg_dc_dn7 * locals.var_xg_dc) + (locals.var_xg_dc * locals.var_xg_dc_dn7)) / (2.0 * assign47220_e60671)))) / (assign47220_e60671 * assign47220_e60671))), (0.5 * (((locals.var_xg_dc_dn8 * assign47220_e60671) - (locals.var_xg_dc * (((locals.var_xg_dc_dn8 * locals.var_xg_dc) + (locals.var_xg_dc * locals.var_xg_dc_dn8)) / (2.0 * assign47220_e60671)))) / (assign47220_e60671 * assign47220_e60671))), (0.5 * (((locals.var_xg_dc_dn9 * assign47220_e60671) - (locals.var_xg_dc * (((locals.var_xg_dc_dn9 * locals.var_xg_dc) + (locals.var_xg_dc * locals.var_xg_dc_dn9)) / (2.0 * assign47220_e60671)))) / (assign47220_e60671 * assign47220_e60671))),)
    } else {
        (locals.var_sg, locals.var_sg_dn4, locals.var_sg_dn6, locals.var_sg_dn7, locals.var_sg_dn8, locals.var_sg_dn9,)
    }
};
        locals.var_sg = assign47220_e60676;
        locals.var_sg_dn4 = assign47220_e60676_d_n4;
        locals.var_sg_dn6 = assign47220_e60676_d_n6;
        locals.var_sg_dn7 = assign47220_e60676_d_n7;
        locals.var_sg_dn8 = assign47220_e60676_d_n8;
        locals.var_sg_dn9 = assign47220_e60676_d_n9;

        let (assign47230_e60686, assign47230_e60686_d_n4, assign47230_e60686_d_n6, assign47230_e60686_d_n7, assign47230_e60686_d_n8, assign47230_e60686_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign47230_e60682: f64 = (locals.var_igc0 * locals.var_igc);
        let assign47230_e60684: f64 = (assign47230_e60682 * locals.var_sg);
        (assign47230_e60684, ((((locals.var_igc0_dn4 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn4)) * locals.var_sg) + (assign47230_e60682 * locals.var_sg_dn4)), ((((locals.var_igc0_dn6 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn6)) * locals.var_sg) + (assign47230_e60682 * locals.var_sg_dn6)), ((((locals.var_igc0_dn7 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn7)) * locals.var_sg) + (assign47230_e60682 * locals.var_sg_dn7)), ((((locals.var_igc0_dn8 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn8)) * locals.var_sg) + (assign47230_e60682 * locals.var_sg_dn8)), ((((locals.var_igc0_dn9 * locals.var_igc) + (locals.var_igc0 * locals.var_igc_dn9)) * locals.var_sg) + (assign47230_e60682 * locals.var_sg_dn9)),)
    } else {
        (locals.var_igc_1, locals.var_igc_1_dn4, locals.var_igc_1_dn6, locals.var_igc_1_dn7, locals.var_igc_1_dn8, locals.var_igc_1_dn9,)
    }
};
        locals.var_igc_1 = assign47230_e60686;
        locals.var_igc_1_dn4 = assign47230_e60686_d_n4;
        locals.var_igc_1_dn6 = assign47230_e60686_d_n6;
        locals.var_igc_1_dn7 = assign47230_e60686_d_n7;
        locals.var_igc_1_dn8 = assign47230_e60686_d_n8;
        locals.var_igc_1_dn9 = assign47230_e60686_d_n9;

        let (assign47240_e60696, assign47240_e60696_d_n4, assign47240_e60696_d_n6, assign47240_e60696_d_n7, assign47240_e60696_d_n8, assign47240_e60696_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign47240_e60692: f64 = (locals.var_igc0 * locals.var_igcd_h);
        let assign47240_e60694: f64 = (assign47240_e60692 * locals.var_sg);
        (assign47240_e60694, ((((locals.var_igc0_dn4 * locals.var_igcd_h) + (locals.var_igc0 * locals.var_igcd_h_dn4)) * locals.var_sg) + (assign47240_e60692 * locals.var_sg_dn4)), ((((locals.var_igc0_dn6 * locals.var_igcd_h) + (locals.var_igc0 * locals.var_igcd_h_dn6)) * locals.var_sg) + (assign47240_e60692 * locals.var_sg_dn6)), ((((locals.var_igc0_dn7 * locals.var_igcd_h) + (locals.var_igc0 * locals.var_igcd_h_dn7)) * locals.var_sg) + (assign47240_e60692 * locals.var_sg_dn7)), ((((locals.var_igc0_dn8 * locals.var_igcd_h) + (locals.var_igc0 * locals.var_igcd_h_dn8)) * locals.var_sg) + (assign47240_e60692 * locals.var_sg_dn8)), ((((locals.var_igc0_dn9 * locals.var_igcd_h) + (locals.var_igc0 * locals.var_igcd_h_dn9)) * locals.var_sg) + (assign47240_e60692 * locals.var_sg_dn9)),)
    } else {
        (locals.var_i_gcd, locals.var_i_gcd_dn4, locals.var_i_gcd_dn6, locals.var_i_gcd_dn7, locals.var_i_gcd_dn8, locals.var_i_gcd_dn9,)
    }
};
        locals.var_i_gcd = assign47240_e60696;
        locals.var_i_gcd_dn4 = assign47240_e60696_d_n4;
        locals.var_i_gcd_dn6 = assign47240_e60696_d_n6;
        locals.var_i_gcd_dn7 = assign47240_e60696_d_n7;
        locals.var_i_gcd_dn8 = assign47240_e60696_d_n8;
        locals.var_i_gcd_dn9 = assign47240_e60696_d_n9;

        let (assign47250_e60704, assign47250_e60704_d_n4, assign47250_e60704_d_n6, assign47250_e60704_d_n7, assign47250_e60704_d_n8, assign47250_e60704_d_n9,) = {
    if ((locals.var_guard1238 != 0.0) && (locals.var_guard1247 != 0.0)) {
        let assign47250_e60702: f64 = (locals.var_igc_1 - locals.var_i_gcd);
        (assign47250_e60702, (locals.var_igc_1_dn4 - locals.var_i_gcd_dn4), (locals.var_igc_1_dn6 - locals.var_i_gcd_dn6), (locals.var_igc_1_dn7 - locals.var_i_gcd_dn7), (locals.var_igc_1_dn8 - locals.var_i_gcd_dn8), (locals.var_igc_1_dn9 - locals.var_i_gcd_dn9),)
    } else {
        (locals.var_i_gcs, locals.var_i_gcs_dn4, locals.var_i_gcs_dn6, locals.var_i_gcs_dn7, locals.var_i_gcs_dn8, locals.var_i_gcs_dn9,)
    }
};
        locals.var_i_gcs = assign47250_e60704;
        locals.var_i_gcs_dn4 = assign47250_e60704_d_n4;
        locals.var_i_gcs_dn6 = assign47250_e60704_d_n6;
        locals.var_i_gcs_dn7 = assign47250_e60704_d_n7;
        locals.var_i_gcs_dn8 = assign47250_e60704_d_n8;
        locals.var_i_gcs_dn9 = assign47250_e60704_d_n9;

    }
}
