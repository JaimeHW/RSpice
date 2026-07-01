#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_48(
        locals: &mut StampLocals,
    ) {
        let assign16380_e15046: f64 = (locals.var_t1__blk809 + locals.var_t2__blk810);
        let assign16380_e15047: f64 = (0.5 * assign16380_e15046);
        let assign16380_e15048: f64 = (locals.var_t0__blk808 - assign16380_e15047);
        locals.var_vbseff_cv = assign16380_e15048;
        locals.var_vbseff_cv_dn3 = (locals.var_t0__blk808_dn3 - (0.5 * (locals.var_t1__blk809_dn3 + locals.var_t2__blk810_dn3)));
        locals.var_vbseff_cv_dn4 = (locals.var_t0__blk808_dn4 - (0.5 * (locals.var_t1__blk809_dn4 + locals.var_t2__blk810_dn4)));
        locals.var_vbseff_cv_dn5 = (locals.var_t0__blk808_dn5 - (0.5 * (locals.var_t1__blk809_dn5 + locals.var_t2__blk810_dn5)));
        locals.var_vbseff_cv_dn6 = (locals.var_t0__blk808_dn6 - (0.5 * (locals.var_t1__blk809_dn6 + locals.var_t2__blk810_dn6)));
        locals.var_vbseff_cv_dn7 = (locals.var_t0__blk808_dn7 - (0.5 * (locals.var_t1__blk809_dn7 + locals.var_t2__blk810_dn7)));
        locals.var_vbseff_cv_dn8 = (locals.var_t0__blk808_dn8 - (0.5 * (locals.var_t1__blk809_dn8 + locals.var_t2__blk810_dn8)));
        locals.var_vbseff_cv_dn9 = (locals.var_t0__blk808_dn9 - (0.5 * (locals.var_t1__blk809_dn9 + locals.var_t2__blk810_dn9)));
        locals.var_vbseff_cv_dn10 = (locals.var_t0__blk808_dn10 - (0.5 * (locals.var_t1__blk809_dn10 + locals.var_t2__blk810_dn10)));
        locals.var_vbseff_cv_dn11 = (locals.var_t0__blk808_dn11 - (0.5 * (locals.var_t1__blk809_dn11 + locals.var_t2__blk810_dn11)));
        locals.var_vbseff_cv_dn12 = (locals.var_t0__blk808_dn12 - (0.5 * (locals.var_t1__blk809_dn12 + locals.var_t2__blk810_dn12)));
        locals.var_vbseff_cv_rv = 0.0;

        let assign16390_e15051: f64 = (locals.var_phi - locals.var_vbseff);
        locals.var_phis = assign16390_e15051;
        locals.var_phis_dn3 = (locals.var_phi_dn3 - locals.var_vbseff_dn3);
        locals.var_phis_dn4 = (locals.var_phi_dn4 - locals.var_vbseff_dn4);
        locals.var_phis_dn5 = (locals.var_phi_dn5 - locals.var_vbseff_dn5);
        locals.var_phis_dn6 = (locals.var_phi_dn6 - locals.var_vbseff_dn6);
        locals.var_phis_dn7 = (locals.var_phi_dn7 - locals.var_vbseff_dn7);
        locals.var_phis_dn8 = (locals.var_phi_dn8 - locals.var_vbseff_dn8);
        locals.var_phis_dn9 = (locals.var_phi_dn9 - locals.var_vbseff_dn9);
        locals.var_phis_dn10 = (locals.var_phi_dn10 - locals.var_vbseff_dn10);
        locals.var_phis_dn11 = (locals.var_phi_dn11 - locals.var_vbseff_dn11);
        locals.var_phis_dn12 = (locals.var_phi_dn12 - locals.var_vbseff_dn12);
        locals.var_phis_rv = 0.0;

        let assign16400_e15053: f64 = (locals.var_phis).sqrt();
        locals.var_sqrtphis = assign16400_e15053;
        locals.var_sqrtphis_dn3 = (locals.var_phis_dn3 / (2.0 * assign16400_e15053));
        locals.var_sqrtphis_dn4 = (locals.var_phis_dn4 / (2.0 * assign16400_e15053));
        locals.var_sqrtphis_dn5 = (locals.var_phis_dn5 / (2.0 * assign16400_e15053));
        locals.var_sqrtphis_dn6 = (locals.var_phis_dn6 / (2.0 * assign16400_e15053));
        locals.var_sqrtphis_dn7 = (locals.var_phis_dn7 / (2.0 * assign16400_e15053));
        locals.var_sqrtphis_dn8 = (locals.var_phis_dn8 / (2.0 * assign16400_e15053));
        locals.var_sqrtphis_dn9 = (locals.var_phis_dn9 / (2.0 * assign16400_e15053));
        locals.var_sqrtphis_dn10 = (locals.var_phis_dn10 / (2.0 * assign16400_e15053));
        locals.var_sqrtphis_dn11 = (locals.var_phis_dn11 / (2.0 * assign16400_e15053));
        locals.var_sqrtphis_dn12 = (locals.var_phis_dn12 / (2.0 * assign16400_e15053));
        locals.var_sqrtphis_rv = 0.0;

        let assign16410_e15056: f64 = (locals.var_xdep0 * locals.var_sqrtphis);
        let assign16410_e15058: f64 = (assign16410_e15056 / locals.var_sqrtphi);
        locals.var_xdep = assign16410_e15058;
        locals.var_xdep_dn3 = (((((locals.var_xdep0_dn3 * locals.var_sqrtphis) + (locals.var_xdep0 * locals.var_sqrtphis_dn3)) * locals.var_sqrtphi) - (assign16410_e15056 * locals.var_sqrtphi_dn3)) / (locals.var_sqrtphi * locals.var_sqrtphi));
        locals.var_xdep_dn4 = (((((locals.var_xdep0_dn4 * locals.var_sqrtphis) + (locals.var_xdep0 * locals.var_sqrtphis_dn4)) * locals.var_sqrtphi) - (assign16410_e15056 * locals.var_sqrtphi_dn4)) / (locals.var_sqrtphi * locals.var_sqrtphi));
        locals.var_xdep_dn5 = (((((locals.var_xdep0_dn5 * locals.var_sqrtphis) + (locals.var_xdep0 * locals.var_sqrtphis_dn5)) * locals.var_sqrtphi) - (assign16410_e15056 * locals.var_sqrtphi_dn5)) / (locals.var_sqrtphi * locals.var_sqrtphi));
        locals.var_xdep_dn6 = (((((locals.var_xdep0_dn6 * locals.var_sqrtphis) + (locals.var_xdep0 * locals.var_sqrtphis_dn6)) * locals.var_sqrtphi) - (assign16410_e15056 * locals.var_sqrtphi_dn6)) / (locals.var_sqrtphi * locals.var_sqrtphi));
        locals.var_xdep_dn7 = (((((locals.var_xdep0_dn7 * locals.var_sqrtphis) + (locals.var_xdep0 * locals.var_sqrtphis_dn7)) * locals.var_sqrtphi) - (assign16410_e15056 * locals.var_sqrtphi_dn7)) / (locals.var_sqrtphi * locals.var_sqrtphi));
        locals.var_xdep_dn8 = (((((locals.var_xdep0_dn8 * locals.var_sqrtphis) + (locals.var_xdep0 * locals.var_sqrtphis_dn8)) * locals.var_sqrtphi) - (assign16410_e15056 * locals.var_sqrtphi_dn8)) / (locals.var_sqrtphi * locals.var_sqrtphi));
        locals.var_xdep_dn9 = (((((locals.var_xdep0_dn9 * locals.var_sqrtphis) + (locals.var_xdep0 * locals.var_sqrtphis_dn9)) * locals.var_sqrtphi) - (assign16410_e15056 * locals.var_sqrtphi_dn9)) / (locals.var_sqrtphi * locals.var_sqrtphi));
        locals.var_xdep_dn10 = (((((locals.var_xdep0_dn10 * locals.var_sqrtphis) + (locals.var_xdep0 * locals.var_sqrtphis_dn10)) * locals.var_sqrtphi) - (assign16410_e15056 * locals.var_sqrtphi_dn10)) / (locals.var_sqrtphi * locals.var_sqrtphi));
        locals.var_xdep_dn11 = (((((locals.var_xdep0_dn11 * locals.var_sqrtphis) + (locals.var_xdep0 * locals.var_sqrtphis_dn11)) * locals.var_sqrtphi) - (assign16410_e15056 * locals.var_sqrtphi_dn11)) / (locals.var_sqrtphi * locals.var_sqrtphi));
        locals.var_xdep_dn12 = (((((locals.var_xdep0_dn12 * locals.var_sqrtphis) + (locals.var_xdep0 * locals.var_sqrtphis_dn12)) * locals.var_sqrtphi) - (assign16410_e15056 * locals.var_sqrtphi_dn12)) / (locals.var_sqrtphi * locals.var_sqrtphi));
        locals.var_xdep_rv = 0.0;

        let assign16430_e15071: f64 = (locals.var_xdep).sqrt();
        locals.var_t3__blk811 = assign16430_e15071;
        locals.var_t3__blk811_dn3 = (locals.var_xdep_dn3 / (2.0 * assign16430_e15071));
        locals.var_t3__blk811_dn4 = (locals.var_xdep_dn4 / (2.0 * assign16430_e15071));
        locals.var_t3__blk811_dn5 = (locals.var_xdep_dn5 / (2.0 * assign16430_e15071));
        locals.var_t3__blk811_dn6 = (locals.var_xdep_dn6 / (2.0 * assign16430_e15071));
        locals.var_t3__blk811_dn7 = (locals.var_xdep_dn7 / (2.0 * assign16430_e15071));
        locals.var_t3__blk811_dn8 = (locals.var_xdep_dn8 / (2.0 * assign16430_e15071));
        locals.var_t3__blk811_dn9 = (locals.var_xdep_dn9 / (2.0 * assign16430_e15071));
        locals.var_t3__blk811_dn10 = (locals.var_xdep_dn10 / (2.0 * assign16430_e15071));
        locals.var_t3__blk811_dn11 = (locals.var_xdep_dn11 / (2.0 * assign16430_e15071));
        locals.var_t3__blk811_dn12 = (locals.var_xdep_dn12 / (2.0 * assign16430_e15071));
        locals.var_t3__blk811_rv = 0.0;

        let assign16440_e15074: f64 = (locals.var_pparam_b4soidvt2 * locals.var_vbseff);
        locals.var_t0__blk808 = assign16440_e15074;
        locals.var_t0__blk808_dn3 = ((locals.var_pparam_b4soidvt2_dn3 * locals.var_vbseff) + (locals.var_pparam_b4soidvt2 * locals.var_vbseff_dn3));
        locals.var_t0__blk808_dn4 = ((locals.var_pparam_b4soidvt2_dn4 * locals.var_vbseff) + (locals.var_pparam_b4soidvt2 * locals.var_vbseff_dn4));
        locals.var_t0__blk808_dn5 = ((locals.var_pparam_b4soidvt2_dn5 * locals.var_vbseff) + (locals.var_pparam_b4soidvt2 * locals.var_vbseff_dn5));
        locals.var_t0__blk808_dn6 = ((locals.var_pparam_b4soidvt2_dn6 * locals.var_vbseff) + (locals.var_pparam_b4soidvt2 * locals.var_vbseff_dn6));
        locals.var_t0__blk808_dn7 = ((locals.var_pparam_b4soidvt2_dn7 * locals.var_vbseff) + (locals.var_pparam_b4soidvt2 * locals.var_vbseff_dn7));
        locals.var_t0__blk808_dn8 = ((locals.var_pparam_b4soidvt2_dn8 * locals.var_vbseff) + (locals.var_pparam_b4soidvt2 * locals.var_vbseff_dn8));
        locals.var_t0__blk808_dn9 = ((locals.var_pparam_b4soidvt2_dn9 * locals.var_vbseff) + (locals.var_pparam_b4soidvt2 * locals.var_vbseff_dn9));
        locals.var_t0__blk808_dn10 = ((locals.var_pparam_b4soidvt2_dn10 * locals.var_vbseff) + (locals.var_pparam_b4soidvt2 * locals.var_vbseff_dn10));
        locals.var_t0__blk808_dn11 = ((locals.var_pparam_b4soidvt2_dn11 * locals.var_vbseff) + (locals.var_pparam_b4soidvt2 * locals.var_vbseff_dn11));
        locals.var_t0__blk808_dn12 = ((locals.var_pparam_b4soidvt2_dn12 * locals.var_vbseff) + (locals.var_pparam_b4soidvt2 * locals.var_vbseff_dn12));
        locals.var_t0__blk808_rv = 0.0;

        let assign16450_e15077: f64 = (-0.5);
        let assign16450_e15078: f64 = if locals.var_t0__blk808 >= assign16450_e15077 { 1.0 } else { 0.0 };
        locals.var_guard1190 = assign16450_e15078;
        locals.var_guard1190_rv = 0.0;

        let (assign16460_e15084, assign16460_e15084_d_n3, assign16460_e15084_d_n4, assign16460_e15084_d_n5, assign16460_e15084_d_n6, assign16460_e15084_d_n7, assign16460_e15084_d_n8, assign16460_e15084_d_n9, assign16460_e15084_d_n10, assign16460_e15084_d_n11, assign16460_e15084_d_n12,) = {
    if (locals.var_guard1190 != 0.0) {
        let assign16460_e15082: f64 = (1.0 + locals.var_t0__blk808);
        (assign16460_e15082, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign16460_e15084;
        locals.var_t1__blk809_dn3 = assign16460_e15084_d_n3;
        locals.var_t1__blk809_dn4 = assign16460_e15084_d_n4;
        locals.var_t1__blk809_dn5 = assign16460_e15084_d_n5;
        locals.var_t1__blk809_dn6 = assign16460_e15084_d_n6;
        locals.var_t1__blk809_dn7 = assign16460_e15084_d_n7;
        locals.var_t1__blk809_dn8 = assign16460_e15084_d_n8;
        locals.var_t1__blk809_dn9 = assign16460_e15084_d_n9;
        locals.var_t1__blk809_dn10 = assign16460_e15084_d_n10;
        locals.var_t1__blk809_dn11 = assign16460_e15084_d_n11;
        locals.var_t1__blk809_dn12 = assign16460_e15084_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign16470_e15095, assign16470_e15095_d_n3, assign16470_e15095_d_n4, assign16470_e15095_d_n5, assign16470_e15095_d_n6, assign16470_e15095_d_n7, assign16470_e15095_d_n8, assign16470_e15095_d_n9, assign16470_e15095_d_n10, assign16470_e15095_d_n11, assign16470_e15095_d_n12,) = {
    if (locals.var_guard1190 == 0.0) {
        let assign16470_e15091: f64 = (8.0 * locals.var_t0__blk808);
        let assign16470_e15092: f64 = (3.0 + assign16470_e15091);
        let assign16470_e15093: f64 = (1.0 / assign16470_e15092);
        (assign16470_e15093, (-((8.0 * locals.var_t0__blk808_dn3) / (assign16470_e15092 * assign16470_e15092))), (-((8.0 * locals.var_t0__blk808_dn4) / (assign16470_e15092 * assign16470_e15092))), (-((8.0 * locals.var_t0__blk808_dn5) / (assign16470_e15092 * assign16470_e15092))), (-((8.0 * locals.var_t0__blk808_dn6) / (assign16470_e15092 * assign16470_e15092))), (-((8.0 * locals.var_t0__blk808_dn7) / (assign16470_e15092 * assign16470_e15092))), (-((8.0 * locals.var_t0__blk808_dn8) / (assign16470_e15092 * assign16470_e15092))), (-((8.0 * locals.var_t0__blk808_dn9) / (assign16470_e15092 * assign16470_e15092))), (-((8.0 * locals.var_t0__blk808_dn10) / (assign16470_e15092 * assign16470_e15092))), (-((8.0 * locals.var_t0__blk808_dn11) / (assign16470_e15092 * assign16470_e15092))), (-((8.0 * locals.var_t0__blk808_dn12) / (assign16470_e15092 * assign16470_e15092))),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign16470_e15095;
        locals.var_t4__blk812_dn3 = assign16470_e15095_d_n3;
        locals.var_t4__blk812_dn4 = assign16470_e15095_d_n4;
        locals.var_t4__blk812_dn5 = assign16470_e15095_d_n5;
        locals.var_t4__blk812_dn6 = assign16470_e15095_d_n6;
        locals.var_t4__blk812_dn7 = assign16470_e15095_d_n7;
        locals.var_t4__blk812_dn8 = assign16470_e15095_d_n8;
        locals.var_t4__blk812_dn9 = assign16470_e15095_d_n9;
        locals.var_t4__blk812_dn10 = assign16470_e15095_d_n10;
        locals.var_t4__blk812_dn11 = assign16470_e15095_d_n11;
        locals.var_t4__blk812_dn12 = assign16470_e15095_d_n12;
        locals.var_t4__blk812_rv = 0.0;

        let (assign16480_e15106, assign16480_e15106_d_n3, assign16480_e15106_d_n4, assign16480_e15106_d_n5, assign16480_e15106_d_n6, assign16480_e15106_d_n7, assign16480_e15106_d_n8, assign16480_e15106_d_n9, assign16480_e15106_d_n10, assign16480_e15106_d_n11, assign16480_e15106_d_n12,) = {
    if (locals.var_guard1190 == 0.0) {
        let assign16480_e15101: f64 = (3.0 * locals.var_t0__blk808);
        let assign16480_e15102: f64 = (1.0 + assign16480_e15101);
        let assign16480_e15104: f64 = (assign16480_e15102 * locals.var_t4__blk812);
        (assign16480_e15104, (((3.0 * locals.var_t0__blk808_dn3) * locals.var_t4__blk812) + (assign16480_e15102 * locals.var_t4__blk812_dn3)), (((3.0 * locals.var_t0__blk808_dn4) * locals.var_t4__blk812) + (assign16480_e15102 * locals.var_t4__blk812_dn4)), (((3.0 * locals.var_t0__blk808_dn5) * locals.var_t4__blk812) + (assign16480_e15102 * locals.var_t4__blk812_dn5)), (((3.0 * locals.var_t0__blk808_dn6) * locals.var_t4__blk812) + (assign16480_e15102 * locals.var_t4__blk812_dn6)), (((3.0 * locals.var_t0__blk808_dn7) * locals.var_t4__blk812) + (assign16480_e15102 * locals.var_t4__blk812_dn7)), (((3.0 * locals.var_t0__blk808_dn8) * locals.var_t4__blk812) + (assign16480_e15102 * locals.var_t4__blk812_dn8)), (((3.0 * locals.var_t0__blk808_dn9) * locals.var_t4__blk812) + (assign16480_e15102 * locals.var_t4__blk812_dn9)), (((3.0 * locals.var_t0__blk808_dn10) * locals.var_t4__blk812) + (assign16480_e15102 * locals.var_t4__blk812_dn10)), (((3.0 * locals.var_t0__blk808_dn11) * locals.var_t4__blk812) + (assign16480_e15102 * locals.var_t4__blk812_dn11)), (((3.0 * locals.var_t0__blk808_dn12) * locals.var_t4__blk812) + (assign16480_e15102 * locals.var_t4__blk812_dn12)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign16480_e15106;
        locals.var_t1__blk809_dn3 = assign16480_e15106_d_n3;
        locals.var_t1__blk809_dn4 = assign16480_e15106_d_n4;
        locals.var_t1__blk809_dn5 = assign16480_e15106_d_n5;
        locals.var_t1__blk809_dn6 = assign16480_e15106_d_n6;
        locals.var_t1__blk809_dn7 = assign16480_e15106_d_n7;
        locals.var_t1__blk809_dn8 = assign16480_e15106_d_n8;
        locals.var_t1__blk809_dn9 = assign16480_e15106_d_n9;
        locals.var_t1__blk809_dn10 = assign16480_e15106_d_n10;
        locals.var_t1__blk809_dn11 = assign16480_e15106_d_n11;
        locals.var_t1__blk809_dn12 = assign16480_e15106_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let assign16490_e15109: f64 = (locals.var_b4soifactor1 * locals.var_t3__blk811);
        let assign16490_e15111: f64 = (assign16490_e15109 * locals.var_t1__blk809);
        locals.var_lt1__blk830 = assign16490_e15111;
        locals.var_lt1__blk830_dn3 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn3) * locals.var_t1__blk809) + (assign16490_e15109 * locals.var_t1__blk809_dn3));
        locals.var_lt1__blk830_dn4 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn4) * locals.var_t1__blk809) + (assign16490_e15109 * locals.var_t1__blk809_dn4));
        locals.var_lt1__blk830_dn5 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn5) * locals.var_t1__blk809) + (assign16490_e15109 * locals.var_t1__blk809_dn5));
        locals.var_lt1__blk830_dn6 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn6) * locals.var_t1__blk809) + (assign16490_e15109 * locals.var_t1__blk809_dn6));
        locals.var_lt1__blk830_dn7 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn7) * locals.var_t1__blk809) + (assign16490_e15109 * locals.var_t1__blk809_dn7));
        locals.var_lt1__blk830_dn8 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn8) * locals.var_t1__blk809) + (assign16490_e15109 * locals.var_t1__blk809_dn8));
        locals.var_lt1__blk830_dn9 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn9) * locals.var_t1__blk809) + (assign16490_e15109 * locals.var_t1__blk809_dn9));
        locals.var_lt1__blk830_dn10 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn10) * locals.var_t1__blk809) + (assign16490_e15109 * locals.var_t1__blk809_dn10));
        locals.var_lt1__blk830_dn11 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn11) * locals.var_t1__blk809) + (assign16490_e15109 * locals.var_t1__blk809_dn11));
        locals.var_lt1__blk830_dn12 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn12) * locals.var_t1__blk809) + (assign16490_e15109 * locals.var_t1__blk809_dn12));
        locals.var_lt1__blk830_rv = 0.0;

        let assign16500_e15114: f64 = (locals.var_pparam_b4soidvt2w * locals.var_vbseff);
        locals.var_t0__blk808 = assign16500_e15114;
        locals.var_t0__blk808_dn3 = ((locals.var_pparam_b4soidvt2w_dn3 * locals.var_vbseff) + (locals.var_pparam_b4soidvt2w * locals.var_vbseff_dn3));
        locals.var_t0__blk808_dn4 = ((locals.var_pparam_b4soidvt2w_dn4 * locals.var_vbseff) + (locals.var_pparam_b4soidvt2w * locals.var_vbseff_dn4));
        locals.var_t0__blk808_dn5 = ((locals.var_pparam_b4soidvt2w_dn5 * locals.var_vbseff) + (locals.var_pparam_b4soidvt2w * locals.var_vbseff_dn5));
        locals.var_t0__blk808_dn6 = ((locals.var_pparam_b4soidvt2w_dn6 * locals.var_vbseff) + (locals.var_pparam_b4soidvt2w * locals.var_vbseff_dn6));
        locals.var_t0__blk808_dn7 = ((locals.var_pparam_b4soidvt2w_dn7 * locals.var_vbseff) + (locals.var_pparam_b4soidvt2w * locals.var_vbseff_dn7));
        locals.var_t0__blk808_dn8 = ((locals.var_pparam_b4soidvt2w_dn8 * locals.var_vbseff) + (locals.var_pparam_b4soidvt2w * locals.var_vbseff_dn8));
        locals.var_t0__blk808_dn9 = ((locals.var_pparam_b4soidvt2w_dn9 * locals.var_vbseff) + (locals.var_pparam_b4soidvt2w * locals.var_vbseff_dn9));
        locals.var_t0__blk808_dn10 = ((locals.var_pparam_b4soidvt2w_dn10 * locals.var_vbseff) + (locals.var_pparam_b4soidvt2w * locals.var_vbseff_dn10));
        locals.var_t0__blk808_dn11 = ((locals.var_pparam_b4soidvt2w_dn11 * locals.var_vbseff) + (locals.var_pparam_b4soidvt2w * locals.var_vbseff_dn11));
        locals.var_t0__blk808_dn12 = ((locals.var_pparam_b4soidvt2w_dn12 * locals.var_vbseff) + (locals.var_pparam_b4soidvt2w * locals.var_vbseff_dn12));
        locals.var_t0__blk808_rv = 0.0;

        let assign16510_e15117: f64 = (-0.5);
        let assign16510_e15118: f64 = if locals.var_t0__blk808 >= assign16510_e15117 { 1.0 } else { 0.0 };
        locals.var_guard1191 = assign16510_e15118;
        locals.var_guard1191_rv = 0.0;

        let (assign16520_e15124, assign16520_e15124_d_n3, assign16520_e15124_d_n4, assign16520_e15124_d_n5, assign16520_e15124_d_n6, assign16520_e15124_d_n7, assign16520_e15124_d_n8, assign16520_e15124_d_n9, assign16520_e15124_d_n10, assign16520_e15124_d_n11, assign16520_e15124_d_n12,) = {
    if (locals.var_guard1191 != 0.0) {
        let assign16520_e15122: f64 = (1.0 + locals.var_t0__blk808);
        (assign16520_e15122, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign16520_e15124;
        locals.var_t1__blk809_dn3 = assign16520_e15124_d_n3;
        locals.var_t1__blk809_dn4 = assign16520_e15124_d_n4;
        locals.var_t1__blk809_dn5 = assign16520_e15124_d_n5;
        locals.var_t1__blk809_dn6 = assign16520_e15124_d_n6;
        locals.var_t1__blk809_dn7 = assign16520_e15124_d_n7;
        locals.var_t1__blk809_dn8 = assign16520_e15124_d_n8;
        locals.var_t1__blk809_dn9 = assign16520_e15124_d_n9;
        locals.var_t1__blk809_dn10 = assign16520_e15124_d_n10;
        locals.var_t1__blk809_dn11 = assign16520_e15124_d_n11;
        locals.var_t1__blk809_dn12 = assign16520_e15124_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign16530_e15135, assign16530_e15135_d_n3, assign16530_e15135_d_n4, assign16530_e15135_d_n5, assign16530_e15135_d_n6, assign16530_e15135_d_n7, assign16530_e15135_d_n8, assign16530_e15135_d_n9, assign16530_e15135_d_n10, assign16530_e15135_d_n11, assign16530_e15135_d_n12,) = {
    if (locals.var_guard1191 == 0.0) {
        let assign16530_e15131: f64 = (8.0 * locals.var_t0__blk808);
        let assign16530_e15132: f64 = (3.0 + assign16530_e15131);
        let assign16530_e15133: f64 = (1.0 / assign16530_e15132);
        (assign16530_e15133, (-((8.0 * locals.var_t0__blk808_dn3) / (assign16530_e15132 * assign16530_e15132))), (-((8.0 * locals.var_t0__blk808_dn4) / (assign16530_e15132 * assign16530_e15132))), (-((8.0 * locals.var_t0__blk808_dn5) / (assign16530_e15132 * assign16530_e15132))), (-((8.0 * locals.var_t0__blk808_dn6) / (assign16530_e15132 * assign16530_e15132))), (-((8.0 * locals.var_t0__blk808_dn7) / (assign16530_e15132 * assign16530_e15132))), (-((8.0 * locals.var_t0__blk808_dn8) / (assign16530_e15132 * assign16530_e15132))), (-((8.0 * locals.var_t0__blk808_dn9) / (assign16530_e15132 * assign16530_e15132))), (-((8.0 * locals.var_t0__blk808_dn10) / (assign16530_e15132 * assign16530_e15132))), (-((8.0 * locals.var_t0__blk808_dn11) / (assign16530_e15132 * assign16530_e15132))), (-((8.0 * locals.var_t0__blk808_dn12) / (assign16530_e15132 * assign16530_e15132))),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign16530_e15135;
        locals.var_t4__blk812_dn3 = assign16530_e15135_d_n3;
        locals.var_t4__blk812_dn4 = assign16530_e15135_d_n4;
        locals.var_t4__blk812_dn5 = assign16530_e15135_d_n5;
        locals.var_t4__blk812_dn6 = assign16530_e15135_d_n6;
        locals.var_t4__blk812_dn7 = assign16530_e15135_d_n7;
        locals.var_t4__blk812_dn8 = assign16530_e15135_d_n8;
        locals.var_t4__blk812_dn9 = assign16530_e15135_d_n9;
        locals.var_t4__blk812_dn10 = assign16530_e15135_d_n10;
        locals.var_t4__blk812_dn11 = assign16530_e15135_d_n11;
        locals.var_t4__blk812_dn12 = assign16530_e15135_d_n12;
        locals.var_t4__blk812_rv = 0.0;

        let (assign16540_e15146, assign16540_e15146_d_n3, assign16540_e15146_d_n4, assign16540_e15146_d_n5, assign16540_e15146_d_n6, assign16540_e15146_d_n7, assign16540_e15146_d_n8, assign16540_e15146_d_n9, assign16540_e15146_d_n10, assign16540_e15146_d_n11, assign16540_e15146_d_n12,) = {
    if (locals.var_guard1191 == 0.0) {
        let assign16540_e15141: f64 = (3.0 * locals.var_t0__blk808);
        let assign16540_e15142: f64 = (1.0 + assign16540_e15141);
        let assign16540_e15144: f64 = (assign16540_e15142 * locals.var_t4__blk812);
        (assign16540_e15144, (((3.0 * locals.var_t0__blk808_dn3) * locals.var_t4__blk812) + (assign16540_e15142 * locals.var_t4__blk812_dn3)), (((3.0 * locals.var_t0__blk808_dn4) * locals.var_t4__blk812) + (assign16540_e15142 * locals.var_t4__blk812_dn4)), (((3.0 * locals.var_t0__blk808_dn5) * locals.var_t4__blk812) + (assign16540_e15142 * locals.var_t4__blk812_dn5)), (((3.0 * locals.var_t0__blk808_dn6) * locals.var_t4__blk812) + (assign16540_e15142 * locals.var_t4__blk812_dn6)), (((3.0 * locals.var_t0__blk808_dn7) * locals.var_t4__blk812) + (assign16540_e15142 * locals.var_t4__blk812_dn7)), (((3.0 * locals.var_t0__blk808_dn8) * locals.var_t4__blk812) + (assign16540_e15142 * locals.var_t4__blk812_dn8)), (((3.0 * locals.var_t0__blk808_dn9) * locals.var_t4__blk812) + (assign16540_e15142 * locals.var_t4__blk812_dn9)), (((3.0 * locals.var_t0__blk808_dn10) * locals.var_t4__blk812) + (assign16540_e15142 * locals.var_t4__blk812_dn10)), (((3.0 * locals.var_t0__blk808_dn11) * locals.var_t4__blk812) + (assign16540_e15142 * locals.var_t4__blk812_dn11)), (((3.0 * locals.var_t0__blk808_dn12) * locals.var_t4__blk812) + (assign16540_e15142 * locals.var_t4__blk812_dn12)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign16540_e15146;
        locals.var_t1__blk809_dn3 = assign16540_e15146_d_n3;
        locals.var_t1__blk809_dn4 = assign16540_e15146_d_n4;
        locals.var_t1__blk809_dn5 = assign16540_e15146_d_n5;
        locals.var_t1__blk809_dn6 = assign16540_e15146_d_n6;
        locals.var_t1__blk809_dn7 = assign16540_e15146_d_n7;
        locals.var_t1__blk809_dn8 = assign16540_e15146_d_n8;
        locals.var_t1__blk809_dn9 = assign16540_e15146_d_n9;
        locals.var_t1__blk809_dn10 = assign16540_e15146_d_n10;
        locals.var_t1__blk809_dn11 = assign16540_e15146_d_n11;
        locals.var_t1__blk809_dn12 = assign16540_e15146_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let assign16550_e15149: f64 = (locals.var_b4soifactor1 * locals.var_t3__blk811);
        let assign16550_e15151: f64 = (assign16550_e15149 * locals.var_t1__blk809);
        locals.var_ltw__blk831 = assign16550_e15151;
        locals.var_ltw__blk831_dn3 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn3) * locals.var_t1__blk809) + (assign16550_e15149 * locals.var_t1__blk809_dn3));
        locals.var_ltw__blk831_dn4 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn4) * locals.var_t1__blk809) + (assign16550_e15149 * locals.var_t1__blk809_dn4));
        locals.var_ltw__blk831_dn5 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn5) * locals.var_t1__blk809) + (assign16550_e15149 * locals.var_t1__blk809_dn5));
        locals.var_ltw__blk831_dn6 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn6) * locals.var_t1__blk809) + (assign16550_e15149 * locals.var_t1__blk809_dn6));
        locals.var_ltw__blk831_dn7 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn7) * locals.var_t1__blk809) + (assign16550_e15149 * locals.var_t1__blk809_dn7));
        locals.var_ltw__blk831_dn8 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn8) * locals.var_t1__blk809) + (assign16550_e15149 * locals.var_t1__blk809_dn8));
        locals.var_ltw__blk831_dn9 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn9) * locals.var_t1__blk809) + (assign16550_e15149 * locals.var_t1__blk809_dn9));
        locals.var_ltw__blk831_dn10 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn10) * locals.var_t1__blk809) + (assign16550_e15149 * locals.var_t1__blk809_dn10));
        locals.var_ltw__blk831_dn11 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn11) * locals.var_t1__blk809) + (assign16550_e15149 * locals.var_t1__blk809_dn11));
        locals.var_ltw__blk831_dn12 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn12) * locals.var_t1__blk809) + (assign16550_e15149 * locals.var_t1__blk809_dn12));
        locals.var_ltw__blk831_rv = 0.0;

        let assign16560_e15153: f64 = (-0.5);
        let assign16560_e15155: f64 = (assign16560_e15153 * locals.var_pparam_b4soidvt1);
        let assign16560_e15157: f64 = (assign16560_e15155 * locals.var_leff);
        let assign16560_e15159: f64 = (assign16560_e15157 / locals.var_lt1__blk830);
        locals.var_t0__blk808 = assign16560_e15159;
        locals.var_t0__blk808_dn3 = ((((((assign16560_e15153 * locals.var_pparam_b4soidvt1_dn3) * locals.var_leff) + (assign16560_e15155 * locals.var_leff_dn3)) * locals.var_lt1__blk830) - (assign16560_e15157 * locals.var_lt1__blk830_dn3)) / (locals.var_lt1__blk830 * locals.var_lt1__blk830));
        locals.var_t0__blk808_dn4 = ((((((assign16560_e15153 * locals.var_pparam_b4soidvt1_dn4) * locals.var_leff) + (assign16560_e15155 * locals.var_leff_dn4)) * locals.var_lt1__blk830) - (assign16560_e15157 * locals.var_lt1__blk830_dn4)) / (locals.var_lt1__blk830 * locals.var_lt1__blk830));
        locals.var_t0__blk808_dn5 = ((((((assign16560_e15153 * locals.var_pparam_b4soidvt1_dn5) * locals.var_leff) + (assign16560_e15155 * locals.var_leff_dn5)) * locals.var_lt1__blk830) - (assign16560_e15157 * locals.var_lt1__blk830_dn5)) / (locals.var_lt1__blk830 * locals.var_lt1__blk830));
        locals.var_t0__blk808_dn6 = ((((((assign16560_e15153 * locals.var_pparam_b4soidvt1_dn6) * locals.var_leff) + (assign16560_e15155 * locals.var_leff_dn6)) * locals.var_lt1__blk830) - (assign16560_e15157 * locals.var_lt1__blk830_dn6)) / (locals.var_lt1__blk830 * locals.var_lt1__blk830));
        locals.var_t0__blk808_dn7 = ((((((assign16560_e15153 * locals.var_pparam_b4soidvt1_dn7) * locals.var_leff) + (assign16560_e15155 * locals.var_leff_dn7)) * locals.var_lt1__blk830) - (assign16560_e15157 * locals.var_lt1__blk830_dn7)) / (locals.var_lt1__blk830 * locals.var_lt1__blk830));
        locals.var_t0__blk808_dn8 = ((((((assign16560_e15153 * locals.var_pparam_b4soidvt1_dn8) * locals.var_leff) + (assign16560_e15155 * locals.var_leff_dn8)) * locals.var_lt1__blk830) - (assign16560_e15157 * locals.var_lt1__blk830_dn8)) / (locals.var_lt1__blk830 * locals.var_lt1__blk830));
        locals.var_t0__blk808_dn9 = ((((((assign16560_e15153 * locals.var_pparam_b4soidvt1_dn9) * locals.var_leff) + (assign16560_e15155 * locals.var_leff_dn9)) * locals.var_lt1__blk830) - (assign16560_e15157 * locals.var_lt1__blk830_dn9)) / (locals.var_lt1__blk830 * locals.var_lt1__blk830));
        locals.var_t0__blk808_dn10 = ((((((assign16560_e15153 * locals.var_pparam_b4soidvt1_dn10) * locals.var_leff) + (assign16560_e15155 * locals.var_leff_dn10)) * locals.var_lt1__blk830) - (assign16560_e15157 * locals.var_lt1__blk830_dn10)) / (locals.var_lt1__blk830 * locals.var_lt1__blk830));
        locals.var_t0__blk808_dn11 = ((((((assign16560_e15153 * locals.var_pparam_b4soidvt1_dn11) * locals.var_leff) + (assign16560_e15155 * locals.var_leff_dn11)) * locals.var_lt1__blk830) - (assign16560_e15157 * locals.var_lt1__blk830_dn11)) / (locals.var_lt1__blk830 * locals.var_lt1__blk830));
        locals.var_t0__blk808_dn12 = ((((((assign16560_e15153 * locals.var_pparam_b4soidvt1_dn12) * locals.var_leff) + (assign16560_e15155 * locals.var_leff_dn12)) * locals.var_lt1__blk830) - (assign16560_e15157 * locals.var_lt1__blk830_dn12)) / (locals.var_lt1__blk830 * locals.var_lt1__blk830));
        locals.var_t0__blk808_rv = 0.0;

        let assign16570_e15162: f64 = (-100.0);
        let assign16570_e15163: f64 = if locals.var_t0__blk808 > assign16570_e15162 { 1.0 } else { 0.0 };
        locals.var_guard1192 = assign16570_e15163;
        locals.var_guard1192_rv = 0.0;

        let (assign16580_e15168, assign16580_e15168_d_n3, assign16580_e15168_d_n4, assign16580_e15168_d_n5, assign16580_e15168_d_n6, assign16580_e15168_d_n7, assign16580_e15168_d_n8, assign16580_e15168_d_n9, assign16580_e15168_d_n10, assign16580_e15168_d_n11, assign16580_e15168_d_n12,) = {
    if (locals.var_guard1192 != 0.0) {
        let assign16580_e15166: f64 = (locals.var_t0__blk808).exp();
        (assign16580_e15166, (assign16580_e15166 * locals.var_t0__blk808_dn3), (assign16580_e15166 * locals.var_t0__blk808_dn4), (assign16580_e15166 * locals.var_t0__blk808_dn5), (assign16580_e15166 * locals.var_t0__blk808_dn6), (assign16580_e15166 * locals.var_t0__blk808_dn7), (assign16580_e15166 * locals.var_t0__blk808_dn8), (assign16580_e15166 * locals.var_t0__blk808_dn9), (assign16580_e15166 * locals.var_t0__blk808_dn10), (assign16580_e15166 * locals.var_t0__blk808_dn11), (assign16580_e15166 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign16580_e15168;
        locals.var_t1__blk809_dn3 = assign16580_e15168_d_n3;
        locals.var_t1__blk809_dn4 = assign16580_e15168_d_n4;
        locals.var_t1__blk809_dn5 = assign16580_e15168_d_n5;
        locals.var_t1__blk809_dn6 = assign16580_e15168_d_n6;
        locals.var_t1__blk809_dn7 = assign16580_e15168_d_n7;
        locals.var_t1__blk809_dn8 = assign16580_e15168_d_n8;
        locals.var_t1__blk809_dn9 = assign16580_e15168_d_n9;
        locals.var_t1__blk809_dn10 = assign16580_e15168_d_n10;
        locals.var_t1__blk809_dn11 = assign16580_e15168_d_n11;
        locals.var_t1__blk809_dn12 = assign16580_e15168_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign16590_e15178, assign16590_e15178_d_n3, assign16590_e15178_d_n4, assign16590_e15178_d_n5, assign16590_e15178_d_n6, assign16590_e15178_d_n7, assign16590_e15178_d_n8, assign16590_e15178_d_n9, assign16590_e15178_d_n10, assign16590_e15178_d_n11, assign16590_e15178_d_n12,) = {
    if (locals.var_guard1192 != 0.0) {
        let assign16590_e15174: f64 = (2.0 * locals.var_t1__blk809);
        let assign16590_e15175: f64 = (1.0 + assign16590_e15174);
        let assign16590_e15176: f64 = (locals.var_t1__blk809 * assign16590_e15175);
        (assign16590_e15176, ((locals.var_t1__blk809_dn3 * assign16590_e15175) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn3))), ((locals.var_t1__blk809_dn4 * assign16590_e15175) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn4))), ((locals.var_t1__blk809_dn5 * assign16590_e15175) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn5))), ((locals.var_t1__blk809_dn6 * assign16590_e15175) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn6))), ((locals.var_t1__blk809_dn7 * assign16590_e15175) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn7))), ((locals.var_t1__blk809_dn8 * assign16590_e15175) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn8))), ((locals.var_t1__blk809_dn9 * assign16590_e15175) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn9))), ((locals.var_t1__blk809_dn10 * assign16590_e15175) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn10))), ((locals.var_t1__blk809_dn11 * assign16590_e15175) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn11))), ((locals.var_t1__blk809_dn12 * assign16590_e15175) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn12))),)
    } else {
        (locals.var_theta0__blk833, locals.var_theta0__blk833_dn3, locals.var_theta0__blk833_dn4, locals.var_theta0__blk833_dn5, locals.var_theta0__blk833_dn6, locals.var_theta0__blk833_dn7, locals.var_theta0__blk833_dn8, locals.var_theta0__blk833_dn9, locals.var_theta0__blk833_dn10, locals.var_theta0__blk833_dn11, locals.var_theta0__blk833_dn12,)
    }
};
        locals.var_theta0__blk833 = assign16590_e15178;
        locals.var_theta0__blk833_dn3 = assign16590_e15178_d_n3;
        locals.var_theta0__blk833_dn4 = assign16590_e15178_d_n4;
        locals.var_theta0__blk833_dn5 = assign16590_e15178_d_n5;
        locals.var_theta0__blk833_dn6 = assign16590_e15178_d_n6;
        locals.var_theta0__blk833_dn7 = assign16590_e15178_d_n7;
        locals.var_theta0__blk833_dn8 = assign16590_e15178_d_n8;
        locals.var_theta0__blk833_dn9 = assign16590_e15178_d_n9;
        locals.var_theta0__blk833_dn10 = assign16590_e15178_d_n10;
        locals.var_theta0__blk833_dn11 = assign16590_e15178_d_n11;
        locals.var_theta0__blk833_dn12 = assign16590_e15178_d_n12;
        locals.var_theta0__blk833_rv = 0.0;

        let (assign16600_e15183, assign16600_e15183_d_n3, assign16600_e15183_d_n4, assign16600_e15183_d_n5, assign16600_e15183_d_n6, assign16600_e15183_d_n7, assign16600_e15183_d_n8, assign16600_e15183_d_n9, assign16600_e15183_d_n10, assign16600_e15183_d_n11, assign16600_e15183_d_n12,) = {
    if (locals.var_guard1192 == 0.0) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign16600_e15183;
        locals.var_t1__blk809_dn3 = assign16600_e15183_d_n3;
        locals.var_t1__blk809_dn4 = assign16600_e15183_d_n4;
        locals.var_t1__blk809_dn5 = assign16600_e15183_d_n5;
        locals.var_t1__blk809_dn6 = assign16600_e15183_d_n6;
        locals.var_t1__blk809_dn7 = assign16600_e15183_d_n7;
        locals.var_t1__blk809_dn8 = assign16600_e15183_d_n8;
        locals.var_t1__blk809_dn9 = assign16600_e15183_d_n9;
        locals.var_t1__blk809_dn10 = assign16600_e15183_d_n10;
        locals.var_t1__blk809_dn11 = assign16600_e15183_d_n11;
        locals.var_t1__blk809_dn12 = assign16600_e15183_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign16610_e15194, assign16610_e15194_d_n3, assign16610_e15194_d_n4, assign16610_e15194_d_n5, assign16610_e15194_d_n6, assign16610_e15194_d_n7, assign16610_e15194_d_n8, assign16610_e15194_d_n9, assign16610_e15194_d_n10, assign16610_e15194_d_n11, assign16610_e15194_d_n12,) = {
    if (locals.var_guard1192 == 0.0) {
        let assign16610_e15190: f64 = (2.0 * locals.var_t1__blk809);
        let assign16610_e15191: f64 = (1.0 + assign16610_e15190);
        let assign16610_e15192: f64 = (locals.var_t1__blk809 * assign16610_e15191);
        (assign16610_e15192, ((locals.var_t1__blk809_dn3 * assign16610_e15191) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn3))), ((locals.var_t1__blk809_dn4 * assign16610_e15191) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn4))), ((locals.var_t1__blk809_dn5 * assign16610_e15191) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn5))), ((locals.var_t1__blk809_dn6 * assign16610_e15191) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn6))), ((locals.var_t1__blk809_dn7 * assign16610_e15191) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn7))), ((locals.var_t1__blk809_dn8 * assign16610_e15191) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn8))), ((locals.var_t1__blk809_dn9 * assign16610_e15191) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn9))), ((locals.var_t1__blk809_dn10 * assign16610_e15191) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn10))), ((locals.var_t1__blk809_dn11 * assign16610_e15191) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn11))), ((locals.var_t1__blk809_dn12 * assign16610_e15191) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn12))),)
    } else {
        (locals.var_theta0__blk833, locals.var_theta0__blk833_dn3, locals.var_theta0__blk833_dn4, locals.var_theta0__blk833_dn5, locals.var_theta0__blk833_dn6, locals.var_theta0__blk833_dn7, locals.var_theta0__blk833_dn8, locals.var_theta0__blk833_dn9, locals.var_theta0__blk833_dn10, locals.var_theta0__blk833_dn11, locals.var_theta0__blk833_dn12,)
    }
};
        locals.var_theta0__blk833 = assign16610_e15194;
        locals.var_theta0__blk833_dn3 = assign16610_e15194_d_n3;
        locals.var_theta0__blk833_dn4 = assign16610_e15194_d_n4;
        locals.var_theta0__blk833_dn5 = assign16610_e15194_d_n5;
        locals.var_theta0__blk833_dn6 = assign16610_e15194_d_n6;
        locals.var_theta0__blk833_dn7 = assign16610_e15194_d_n7;
        locals.var_theta0__blk833_dn8 = assign16610_e15194_d_n8;
        locals.var_theta0__blk833_dn9 = assign16610_e15194_d_n9;
        locals.var_theta0__blk833_dn10 = assign16610_e15194_d_n10;
        locals.var_theta0__blk833_dn11 = assign16610_e15194_d_n11;
        locals.var_theta0__blk833_dn12 = assign16610_e15194_d_n12;
        locals.var_theta0__blk833_rv = 0.0;

        let assign16620_e15197: f64 = (locals.var_pparam_b4soinfactor * locals.var_epssub);
        let assign16620_e15199: f64 = (assign16620_e15197 / locals.var_xdep);
        locals.var_t2__blk810 = assign16620_e15199;
        locals.var_t2__blk810_dn3 = ((((locals.var_pparam_b4soinfactor_dn3 * locals.var_epssub) * locals.var_xdep) - (assign16620_e15197 * locals.var_xdep_dn3)) / (locals.var_xdep * locals.var_xdep));
        locals.var_t2__blk810_dn4 = ((((locals.var_pparam_b4soinfactor_dn4 * locals.var_epssub) * locals.var_xdep) - (assign16620_e15197 * locals.var_xdep_dn4)) / (locals.var_xdep * locals.var_xdep));
        locals.var_t2__blk810_dn5 = ((((locals.var_pparam_b4soinfactor_dn5 * locals.var_epssub) * locals.var_xdep) - (assign16620_e15197 * locals.var_xdep_dn5)) / (locals.var_xdep * locals.var_xdep));
        locals.var_t2__blk810_dn6 = ((((locals.var_pparam_b4soinfactor_dn6 * locals.var_epssub) * locals.var_xdep) - (assign16620_e15197 * locals.var_xdep_dn6)) / (locals.var_xdep * locals.var_xdep));
        locals.var_t2__blk810_dn7 = ((((locals.var_pparam_b4soinfactor_dn7 * locals.var_epssub) * locals.var_xdep) - (assign16620_e15197 * locals.var_xdep_dn7)) / (locals.var_xdep * locals.var_xdep));
        locals.var_t2__blk810_dn8 = ((((locals.var_pparam_b4soinfactor_dn8 * locals.var_epssub) * locals.var_xdep) - (assign16620_e15197 * locals.var_xdep_dn8)) / (locals.var_xdep * locals.var_xdep));
        locals.var_t2__blk810_dn9 = ((((locals.var_pparam_b4soinfactor_dn9 * locals.var_epssub) * locals.var_xdep) - (assign16620_e15197 * locals.var_xdep_dn9)) / (locals.var_xdep * locals.var_xdep));
        locals.var_t2__blk810_dn10 = ((((locals.var_pparam_b4soinfactor_dn10 * locals.var_epssub) * locals.var_xdep) - (assign16620_e15197 * locals.var_xdep_dn10)) / (locals.var_xdep * locals.var_xdep));
        locals.var_t2__blk810_dn11 = ((((locals.var_pparam_b4soinfactor_dn11 * locals.var_epssub) * locals.var_xdep) - (assign16620_e15197 * locals.var_xdep_dn11)) / (locals.var_xdep * locals.var_xdep));
        locals.var_t2__blk810_dn12 = ((((locals.var_pparam_b4soinfactor_dn12 * locals.var_epssub) * locals.var_xdep) - (assign16620_e15197 * locals.var_xdep_dn12)) / (locals.var_xdep * locals.var_xdep));
        locals.var_t2__blk810_rv = 0.0;

        let assign16630_e15203: f64 = (locals.var_pparam_b4soicdscb * locals.var_vbseff);
        let assign16630_e15204: f64 = (locals.var_pparam_b4soicdsc + assign16630_e15203);
        let assign16630_e15207: f64 = (locals.var_pparam_b4soicdscd * locals.var_vds_1);
        let assign16630_e15208: f64 = (assign16630_e15204 + assign16630_e15207);
        locals.var_t3__blk811 = assign16630_e15208;
        locals.var_t3__blk811_dn3 = ((locals.var_pparam_b4soicdsc_dn3 + ((locals.var_pparam_b4soicdscb_dn3 * locals.var_vbseff) + (locals.var_pparam_b4soicdscb * locals.var_vbseff_dn3))) + (locals.var_pparam_b4soicdscd_dn3 * locals.var_vds_1));
        locals.var_t3__blk811_dn4 = ((locals.var_pparam_b4soicdsc_dn4 + ((locals.var_pparam_b4soicdscb_dn4 * locals.var_vbseff) + (locals.var_pparam_b4soicdscb * locals.var_vbseff_dn4))) + (locals.var_pparam_b4soicdscd_dn4 * locals.var_vds_1));
        locals.var_t3__blk811_dn5 = ((locals.var_pparam_b4soicdsc_dn5 + ((locals.var_pparam_b4soicdscb_dn5 * locals.var_vbseff) + (locals.var_pparam_b4soicdscb * locals.var_vbseff_dn5))) + (locals.var_pparam_b4soicdscd_dn5 * locals.var_vds_1));
        locals.var_t3__blk811_dn6 = ((locals.var_pparam_b4soicdsc_dn6 + ((locals.var_pparam_b4soicdscb_dn6 * locals.var_vbseff) + (locals.var_pparam_b4soicdscb * locals.var_vbseff_dn6))) + (locals.var_pparam_b4soicdscd_dn6 * locals.var_vds_1));
        locals.var_t3__blk811_dn7 = ((locals.var_pparam_b4soicdsc_dn7 + ((locals.var_pparam_b4soicdscb_dn7 * locals.var_vbseff) + (locals.var_pparam_b4soicdscb * locals.var_vbseff_dn7))) + ((locals.var_pparam_b4soicdscd_dn7 * locals.var_vds_1) + (locals.var_pparam_b4soicdscd * locals.var_vds_1_dn7)));
        locals.var_t3__blk811_dn8 = ((locals.var_pparam_b4soicdsc_dn8 + ((locals.var_pparam_b4soicdscb_dn8 * locals.var_vbseff) + (locals.var_pparam_b4soicdscb * locals.var_vbseff_dn8))) + ((locals.var_pparam_b4soicdscd_dn8 * locals.var_vds_1) + (locals.var_pparam_b4soicdscd * locals.var_vds_1_dn8)));
        locals.var_t3__blk811_dn9 = ((locals.var_pparam_b4soicdsc_dn9 + ((locals.var_pparam_b4soicdscb_dn9 * locals.var_vbseff) + (locals.var_pparam_b4soicdscb * locals.var_vbseff_dn9))) + (locals.var_pparam_b4soicdscd_dn9 * locals.var_vds_1));
        locals.var_t3__blk811_dn10 = ((locals.var_pparam_b4soicdsc_dn10 + ((locals.var_pparam_b4soicdscb_dn10 * locals.var_vbseff) + (locals.var_pparam_b4soicdscb * locals.var_vbseff_dn10))) + (locals.var_pparam_b4soicdscd_dn10 * locals.var_vds_1));
        locals.var_t3__blk811_dn11 = ((locals.var_pparam_b4soicdsc_dn11 + ((locals.var_pparam_b4soicdscb_dn11 * locals.var_vbseff) + (locals.var_pparam_b4soicdscb * locals.var_vbseff_dn11))) + (locals.var_pparam_b4soicdscd_dn11 * locals.var_vds_1));
        locals.var_t3__blk811_dn12 = ((locals.var_pparam_b4soicdsc_dn12 + ((locals.var_pparam_b4soicdscb_dn12 * locals.var_vbseff) + (locals.var_pparam_b4soicdscb * locals.var_vbseff_dn12))) + (locals.var_pparam_b4soicdscd_dn12 * locals.var_vds_1));
        locals.var_t3__blk811_rv = 0.0;

        let assign16640_e15212: f64 = (locals.var_t3__blk811 * locals.var_theta0__blk833);
        let assign16640_e15213: f64 = (locals.var_t2__blk810 + assign16640_e15212);
        let assign16640_e15215: f64 = (assign16640_e15213 + locals.var_pparam_b4soicit);
        let assign16640_e15217: f64 = (assign16640_e15215 / locals.var_b4soicox);
        locals.var_t4__blk812 = assign16640_e15217;
        locals.var_t4__blk812_dn3 = (((locals.var_t2__blk810_dn3 + ((locals.var_t3__blk811_dn3 * locals.var_theta0__blk833) + (locals.var_t3__blk811 * locals.var_theta0__blk833_dn3))) + locals.var_pparam_b4soicit_dn3) / locals.var_b4soicox);
        locals.var_t4__blk812_dn4 = (((locals.var_t2__blk810_dn4 + ((locals.var_t3__blk811_dn4 * locals.var_theta0__blk833) + (locals.var_t3__blk811 * locals.var_theta0__blk833_dn4))) + locals.var_pparam_b4soicit_dn4) / locals.var_b4soicox);
        locals.var_t4__blk812_dn5 = (((locals.var_t2__blk810_dn5 + ((locals.var_t3__blk811_dn5 * locals.var_theta0__blk833) + (locals.var_t3__blk811 * locals.var_theta0__blk833_dn5))) + locals.var_pparam_b4soicit_dn5) / locals.var_b4soicox);
        locals.var_t4__blk812_dn6 = (((locals.var_t2__blk810_dn6 + ((locals.var_t3__blk811_dn6 * locals.var_theta0__blk833) + (locals.var_t3__blk811 * locals.var_theta0__blk833_dn6))) + locals.var_pparam_b4soicit_dn6) / locals.var_b4soicox);
        locals.var_t4__blk812_dn7 = (((locals.var_t2__blk810_dn7 + ((locals.var_t3__blk811_dn7 * locals.var_theta0__blk833) + (locals.var_t3__blk811 * locals.var_theta0__blk833_dn7))) + locals.var_pparam_b4soicit_dn7) / locals.var_b4soicox);
        locals.var_t4__blk812_dn8 = (((locals.var_t2__blk810_dn8 + ((locals.var_t3__blk811_dn8 * locals.var_theta0__blk833) + (locals.var_t3__blk811 * locals.var_theta0__blk833_dn8))) + locals.var_pparam_b4soicit_dn8) / locals.var_b4soicox);
        locals.var_t4__blk812_dn9 = (((locals.var_t2__blk810_dn9 + ((locals.var_t3__blk811_dn9 * locals.var_theta0__blk833) + (locals.var_t3__blk811 * locals.var_theta0__blk833_dn9))) + locals.var_pparam_b4soicit_dn9) / locals.var_b4soicox);
        locals.var_t4__blk812_dn10 = (((locals.var_t2__blk810_dn10 + ((locals.var_t3__blk811_dn10 * locals.var_theta0__blk833) + (locals.var_t3__blk811 * locals.var_theta0__blk833_dn10))) + locals.var_pparam_b4soicit_dn10) / locals.var_b4soicox);
        locals.var_t4__blk812_dn11 = (((locals.var_t2__blk810_dn11 + ((locals.var_t3__blk811_dn11 * locals.var_theta0__blk833) + (locals.var_t3__blk811 * locals.var_theta0__blk833_dn11))) + locals.var_pparam_b4soicit_dn11) / locals.var_b4soicox);
        locals.var_t4__blk812_dn12 = (((locals.var_t2__blk810_dn12 + ((locals.var_t3__blk811_dn12 * locals.var_theta0__blk833) + (locals.var_t3__blk811 * locals.var_theta0__blk833_dn12))) + locals.var_pparam_b4soicit_dn12) / locals.var_b4soicox);
        locals.var_t4__blk812_rv = 0.0;

        let assign16650_e15220: f64 = (-0.5);
        let assign16650_e15221: f64 = if locals.var_t4__blk812 >= assign16650_e15220 { 1.0 } else { 0.0 };
        locals.var_guard1193 = assign16650_e15221;
        locals.var_guard1193_rv = 0.0;

        let (assign16660_e15227, assign16660_e15227_d_n3, assign16660_e15227_d_n4, assign16660_e15227_d_n5, assign16660_e15227_d_n6, assign16660_e15227_d_n7, assign16660_e15227_d_n8, assign16660_e15227_d_n9, assign16660_e15227_d_n10, assign16660_e15227_d_n11, assign16660_e15227_d_n12,) = {
    if (locals.var_guard1193 != 0.0) {
        let assign16660_e15225: f64 = (1.0 + locals.var_t4__blk812);
        (assign16660_e15225, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    } else {
        (locals.var_n__blk796, locals.var_n__blk796_dn3, locals.var_n__blk796_dn4, locals.var_n__blk796_dn5, locals.var_n__blk796_dn6, locals.var_n__blk796_dn7, locals.var_n__blk796_dn8, locals.var_n__blk796_dn9, locals.var_n__blk796_dn10, locals.var_n__blk796_dn11, locals.var_n__blk796_dn12,)
    }
};
        locals.var_n__blk796 = assign16660_e15227;
        locals.var_n__blk796_dn3 = assign16660_e15227_d_n3;
        locals.var_n__blk796_dn4 = assign16660_e15227_d_n4;
        locals.var_n__blk796_dn5 = assign16660_e15227_d_n5;
        locals.var_n__blk796_dn6 = assign16660_e15227_d_n6;
        locals.var_n__blk796_dn7 = assign16660_e15227_d_n7;
        locals.var_n__blk796_dn8 = assign16660_e15227_d_n8;
        locals.var_n__blk796_dn9 = assign16660_e15227_d_n9;
        locals.var_n__blk796_dn10 = assign16660_e15227_d_n10;
        locals.var_n__blk796_dn11 = assign16660_e15227_d_n11;
        locals.var_n__blk796_dn12 = assign16660_e15227_d_n12;
        locals.var_n__blk796_rv = 0.0;

        let (assign16670_e15238, assign16670_e15238_d_n3, assign16670_e15238_d_n4, assign16670_e15238_d_n5, assign16670_e15238_d_n6, assign16670_e15238_d_n7, assign16670_e15238_d_n8, assign16670_e15238_d_n9, assign16670_e15238_d_n10, assign16670_e15238_d_n11, assign16670_e15238_d_n12,) = {
    if (locals.var_guard1193 == 0.0) {
        let assign16670_e15234: f64 = (8.0 * locals.var_t4__blk812);
        let assign16670_e15235: f64 = (3.0 + assign16670_e15234);
        let assign16670_e15236: f64 = (1.0 / assign16670_e15235);
        (assign16670_e15236, (-((8.0 * locals.var_t4__blk812_dn3) / (assign16670_e15235 * assign16670_e15235))), (-((8.0 * locals.var_t4__blk812_dn4) / (assign16670_e15235 * assign16670_e15235))), (-((8.0 * locals.var_t4__blk812_dn5) / (assign16670_e15235 * assign16670_e15235))), (-((8.0 * locals.var_t4__blk812_dn6) / (assign16670_e15235 * assign16670_e15235))), (-((8.0 * locals.var_t4__blk812_dn7) / (assign16670_e15235 * assign16670_e15235))), (-((8.0 * locals.var_t4__blk812_dn8) / (assign16670_e15235 * assign16670_e15235))), (-((8.0 * locals.var_t4__blk812_dn9) / (assign16670_e15235 * assign16670_e15235))), (-((8.0 * locals.var_t4__blk812_dn10) / (assign16670_e15235 * assign16670_e15235))), (-((8.0 * locals.var_t4__blk812_dn11) / (assign16670_e15235 * assign16670_e15235))), (-((8.0 * locals.var_t4__blk812_dn12) / (assign16670_e15235 * assign16670_e15235))),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign16670_e15238;
        locals.var_t0__blk808_dn3 = assign16670_e15238_d_n3;
        locals.var_t0__blk808_dn4 = assign16670_e15238_d_n4;
        locals.var_t0__blk808_dn5 = assign16670_e15238_d_n5;
        locals.var_t0__blk808_dn6 = assign16670_e15238_d_n6;
        locals.var_t0__blk808_dn7 = assign16670_e15238_d_n7;
        locals.var_t0__blk808_dn8 = assign16670_e15238_d_n8;
        locals.var_t0__blk808_dn9 = assign16670_e15238_d_n9;
        locals.var_t0__blk808_dn10 = assign16670_e15238_d_n10;
        locals.var_t0__blk808_dn11 = assign16670_e15238_d_n11;
        locals.var_t0__blk808_dn12 = assign16670_e15238_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign16680_e15249, assign16680_e15249_d_n3, assign16680_e15249_d_n4, assign16680_e15249_d_n5, assign16680_e15249_d_n6, assign16680_e15249_d_n7, assign16680_e15249_d_n8, assign16680_e15249_d_n9, assign16680_e15249_d_n10, assign16680_e15249_d_n11, assign16680_e15249_d_n12,) = {
    if (locals.var_guard1193 == 0.0) {
        let assign16680_e15244: f64 = (3.0 * locals.var_t4__blk812);
        let assign16680_e15245: f64 = (1.0 + assign16680_e15244);
        let assign16680_e15247: f64 = (assign16680_e15245 * locals.var_t0__blk808);
        (assign16680_e15247, (((3.0 * locals.var_t4__blk812_dn3) * locals.var_t0__blk808) + (assign16680_e15245 * locals.var_t0__blk808_dn3)), (((3.0 * locals.var_t4__blk812_dn4) * locals.var_t0__blk808) + (assign16680_e15245 * locals.var_t0__blk808_dn4)), (((3.0 * locals.var_t4__blk812_dn5) * locals.var_t0__blk808) + (assign16680_e15245 * locals.var_t0__blk808_dn5)), (((3.0 * locals.var_t4__blk812_dn6) * locals.var_t0__blk808) + (assign16680_e15245 * locals.var_t0__blk808_dn6)), (((3.0 * locals.var_t4__blk812_dn7) * locals.var_t0__blk808) + (assign16680_e15245 * locals.var_t0__blk808_dn7)), (((3.0 * locals.var_t4__blk812_dn8) * locals.var_t0__blk808) + (assign16680_e15245 * locals.var_t0__blk808_dn8)), (((3.0 * locals.var_t4__blk812_dn9) * locals.var_t0__blk808) + (assign16680_e15245 * locals.var_t0__blk808_dn9)), (((3.0 * locals.var_t4__blk812_dn10) * locals.var_t0__blk808) + (assign16680_e15245 * locals.var_t0__blk808_dn10)), (((3.0 * locals.var_t4__blk812_dn11) * locals.var_t0__blk808) + (assign16680_e15245 * locals.var_t0__blk808_dn11)), (((3.0 * locals.var_t4__blk812_dn12) * locals.var_t0__blk808) + (assign16680_e15245 * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_n__blk796, locals.var_n__blk796_dn3, locals.var_n__blk796_dn4, locals.var_n__blk796_dn5, locals.var_n__blk796_dn6, locals.var_n__blk796_dn7, locals.var_n__blk796_dn8, locals.var_n__blk796_dn9, locals.var_n__blk796_dn10, locals.var_n__blk796_dn11, locals.var_n__blk796_dn12,)
    }
};
        locals.var_n__blk796 = assign16680_e15249;
        locals.var_n__blk796_dn3 = assign16680_e15249_d_n3;
        locals.var_n__blk796_dn4 = assign16680_e15249_d_n4;
        locals.var_n__blk796_dn5 = assign16680_e15249_d_n5;
        locals.var_n__blk796_dn6 = assign16680_e15249_d_n6;
        locals.var_n__blk796_dn7 = assign16680_e15249_d_n7;
        locals.var_n__blk796_dn8 = assign16680_e15249_d_n8;
        locals.var_n__blk796_dn9 = assign16680_e15249_d_n9;
        locals.var_n__blk796_dn10 = assign16680_e15249_d_n10;
        locals.var_n__blk796_dn11 = assign16680_e15249_d_n11;
        locals.var_n__blk796_dn12 = assign16680_e15249_d_n12;
        locals.var_n__blk796_rv = 0.0;

        let assign16690_e15252: f64 = if locals.var_pparam_b4soidvtp0 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1194 = assign16690_e15252;
        locals.var_guard1194_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_49(
        locals: &mut StampLocals,
    ) {
        let (assign16700_e15259, assign16700_e15259_d_n3, assign16700_e15259_d_n4, assign16700_e15259_d_n5, assign16700_e15259_d_n6, assign16700_e15259_d_n7, assign16700_e15259_d_n8, assign16700_e15259_d_n9, assign16700_e15259_d_n10, assign16700_e15259_d_n11, assign16700_e15259_d_n12,) = {
    if (locals.var_guard1194 != 0.0) {
        let assign16700_e15255: f64 = (-locals.var_pparam_b4soidvtp1);
        let assign16700_e15257: f64 = (assign16700_e15255 * locals.var_vds_1);
        (assign16700_e15257, ((-locals.var_pparam_b4soidvtp1_dn3) * locals.var_vds_1), ((-locals.var_pparam_b4soidvtp1_dn4) * locals.var_vds_1), ((-locals.var_pparam_b4soidvtp1_dn5) * locals.var_vds_1), ((-locals.var_pparam_b4soidvtp1_dn6) * locals.var_vds_1), (((-locals.var_pparam_b4soidvtp1_dn7) * locals.var_vds_1) + (assign16700_e15255 * locals.var_vds_1_dn7)), (((-locals.var_pparam_b4soidvtp1_dn8) * locals.var_vds_1) + (assign16700_e15255 * locals.var_vds_1_dn8)), ((-locals.var_pparam_b4soidvtp1_dn9) * locals.var_vds_1), ((-locals.var_pparam_b4soidvtp1_dn10) * locals.var_vds_1), ((-locals.var_pparam_b4soidvtp1_dn11) * locals.var_vds_1), ((-locals.var_pparam_b4soidvtp1_dn12) * locals.var_vds_1),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign16700_e15259;
        locals.var_t0__blk808_dn3 = assign16700_e15259_d_n3;
        locals.var_t0__blk808_dn4 = assign16700_e15259_d_n4;
        locals.var_t0__blk808_dn5 = assign16700_e15259_d_n5;
        locals.var_t0__blk808_dn6 = assign16700_e15259_d_n6;
        locals.var_t0__blk808_dn7 = assign16700_e15259_d_n7;
        locals.var_t0__blk808_dn8 = assign16700_e15259_d_n8;
        locals.var_t0__blk808_dn9 = assign16700_e15259_d_n9;
        locals.var_t0__blk808_dn10 = assign16700_e15259_d_n10;
        locals.var_t0__blk808_dn11 = assign16700_e15259_d_n11;
        locals.var_t0__blk808_dn12 = assign16700_e15259_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let assign16710_e15262: f64 = (-100.0);
        let assign16710_e15263: f64 = if locals.var_t0__blk808 < assign16710_e15262 { 1.0 } else { 0.0 };
        locals.var_guard1195 = assign16710_e15263;
        locals.var_guard1195_rv = 0.0;

        let (assign16720_e15269, assign16720_e15269_d_n3, assign16720_e15269_d_n4, assign16720_e15269_d_n5, assign16720_e15269_d_n6, assign16720_e15269_d_n7, assign16720_e15269_d_n8, assign16720_e15269_d_n9, assign16720_e15269_d_n10, assign16720_e15269_d_n11, assign16720_e15269_d_n12,) = {
    if ((locals.var_guard1194 != 0.0) && (locals.var_guard1195 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign16720_e15269;
        locals.var_t2__blk810_dn3 = assign16720_e15269_d_n3;
        locals.var_t2__blk810_dn4 = assign16720_e15269_d_n4;
        locals.var_t2__blk810_dn5 = assign16720_e15269_d_n5;
        locals.var_t2__blk810_dn6 = assign16720_e15269_d_n6;
        locals.var_t2__blk810_dn7 = assign16720_e15269_d_n7;
        locals.var_t2__blk810_dn8 = assign16720_e15269_d_n8;
        locals.var_t2__blk810_dn9 = assign16720_e15269_d_n9;
        locals.var_t2__blk810_dn10 = assign16720_e15269_d_n10;
        locals.var_t2__blk810_dn11 = assign16720_e15269_d_n11;
        locals.var_t2__blk810_dn12 = assign16720_e15269_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign16730_e15277, assign16730_e15277_d_n3, assign16730_e15277_d_n4, assign16730_e15277_d_n5, assign16730_e15277_d_n6, assign16730_e15277_d_n7, assign16730_e15277_d_n8, assign16730_e15277_d_n9, assign16730_e15277_d_n10, assign16730_e15277_d_n11, assign16730_e15277_d_n12,) = {
    if ((locals.var_guard1194 != 0.0) && (locals.var_guard1195 == 0.0)) {
        let assign16730_e15275: f64 = (locals.var_t0__blk808).exp();
        (assign16730_e15275, (assign16730_e15275 * locals.var_t0__blk808_dn3), (assign16730_e15275 * locals.var_t0__blk808_dn4), (assign16730_e15275 * locals.var_t0__blk808_dn5), (assign16730_e15275 * locals.var_t0__blk808_dn6), (assign16730_e15275 * locals.var_t0__blk808_dn7), (assign16730_e15275 * locals.var_t0__blk808_dn8), (assign16730_e15275 * locals.var_t0__blk808_dn9), (assign16730_e15275 * locals.var_t0__blk808_dn10), (assign16730_e15275 * locals.var_t0__blk808_dn11), (assign16730_e15275 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign16730_e15277;
        locals.var_t2__blk810_dn3 = assign16730_e15277_d_n3;
        locals.var_t2__blk810_dn4 = assign16730_e15277_d_n4;
        locals.var_t2__blk810_dn5 = assign16730_e15277_d_n5;
        locals.var_t2__blk810_dn6 = assign16730_e15277_d_n6;
        locals.var_t2__blk810_dn7 = assign16730_e15277_d_n7;
        locals.var_t2__blk810_dn8 = assign16730_e15277_d_n8;
        locals.var_t2__blk810_dn9 = assign16730_e15277_d_n9;
        locals.var_t2__blk810_dn10 = assign16730_e15277_d_n10;
        locals.var_t2__blk810_dn11 = assign16730_e15277_d_n11;
        locals.var_t2__blk810_dn12 = assign16730_e15277_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign16740_e15287, assign16740_e15287_d_n3, assign16740_e15287_d_n4, assign16740_e15287_d_n5, assign16740_e15287_d_n6, assign16740_e15287_d_n7, assign16740_e15287_d_n8, assign16740_e15287_d_n9, assign16740_e15287_d_n10, assign16740_e15287_d_n11, assign16740_e15287_d_n12,) = {
    if (locals.var_guard1194 != 0.0) {
        let assign16740_e15283: f64 = (1.0 + locals.var_t2__blk810);
        let assign16740_e15284: f64 = (locals.var_pparam_b4soidvtp0 * assign16740_e15283);
        let assign16740_e15285: f64 = (locals.var_leff + assign16740_e15284);
        (assign16740_e15285, (locals.var_leff_dn3 + ((locals.var_pparam_b4soidvtp0_dn3 * assign16740_e15283) + (locals.var_pparam_b4soidvtp0 * locals.var_t2__blk810_dn3))), (locals.var_leff_dn4 + ((locals.var_pparam_b4soidvtp0_dn4 * assign16740_e15283) + (locals.var_pparam_b4soidvtp0 * locals.var_t2__blk810_dn4))), (locals.var_leff_dn5 + ((locals.var_pparam_b4soidvtp0_dn5 * assign16740_e15283) + (locals.var_pparam_b4soidvtp0 * locals.var_t2__blk810_dn5))), (locals.var_leff_dn6 + ((locals.var_pparam_b4soidvtp0_dn6 * assign16740_e15283) + (locals.var_pparam_b4soidvtp0 * locals.var_t2__blk810_dn6))), (locals.var_leff_dn7 + ((locals.var_pparam_b4soidvtp0_dn7 * assign16740_e15283) + (locals.var_pparam_b4soidvtp0 * locals.var_t2__blk810_dn7))), (locals.var_leff_dn8 + ((locals.var_pparam_b4soidvtp0_dn8 * assign16740_e15283) + (locals.var_pparam_b4soidvtp0 * locals.var_t2__blk810_dn8))), (locals.var_leff_dn9 + ((locals.var_pparam_b4soidvtp0_dn9 * assign16740_e15283) + (locals.var_pparam_b4soidvtp0 * locals.var_t2__blk810_dn9))), (locals.var_leff_dn10 + ((locals.var_pparam_b4soidvtp0_dn10 * assign16740_e15283) + (locals.var_pparam_b4soidvtp0 * locals.var_t2__blk810_dn10))), (locals.var_leff_dn11 + ((locals.var_pparam_b4soidvtp0_dn11 * assign16740_e15283) + (locals.var_pparam_b4soidvtp0 * locals.var_t2__blk810_dn11))), (locals.var_leff_dn12 + ((locals.var_pparam_b4soidvtp0_dn12 * assign16740_e15283) + (locals.var_pparam_b4soidvtp0 * locals.var_t2__blk810_dn12))),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign16740_e15287;
        locals.var_t3__blk811_dn3 = assign16740_e15287_d_n3;
        locals.var_t3__blk811_dn4 = assign16740_e15287_d_n4;
        locals.var_t3__blk811_dn5 = assign16740_e15287_d_n5;
        locals.var_t3__blk811_dn6 = assign16740_e15287_d_n6;
        locals.var_t3__blk811_dn7 = assign16740_e15287_d_n7;
        locals.var_t3__blk811_dn8 = assign16740_e15287_d_n8;
        locals.var_t3__blk811_dn9 = assign16740_e15287_d_n9;
        locals.var_t3__blk811_dn10 = assign16740_e15287_d_n10;
        locals.var_t3__blk811_dn11 = assign16740_e15287_d_n11;
        locals.var_t3__blk811_dn12 = assign16740_e15287_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign16750_e15304, assign16750_e15304_d_n3, assign16750_e15304_d_n4, assign16750_e15304_d_n5, assign16750_e15304_d_n6, assign16750_e15304_d_n7, assign16750_e15304_d_n8, assign16750_e15304_d_n9, assign16750_e15304_d_n10, assign16750_e15304_d_n11, assign16750_e15304_d_n12,) = {
    if (locals.var_guard1194 != 0.0) {
        let assign16750_e15292: f64 = (locals.var_leff / locals.var_t3__blk811);
        let (assign16750_e15301, assign16750_e15301_d_n3, assign16750_e15301_d_n4, assign16750_e15301_d_n5, assign16750_e15301_d_n6, assign16750_e15301_d_n7, assign16750_e15301_d_n8, assign16750_e15301_d_n9, assign16750_e15301_d_n10, assign16750_e15301_d_n11, assign16750_e15301_d_n12,) = {
            if (assign16750_e15292 > 1e-38) {
                let assign16750_e15297: f64 = (locals.var_leff / locals.var_t3__blk811);
                let assign16750_e15298: f64 = (assign16750_e15297).ln();
                (assign16750_e15298, ((((locals.var_leff_dn3 * locals.var_t3__blk811) - (locals.var_leff * locals.var_t3__blk811_dn3)) / (locals.var_t3__blk811 * locals.var_t3__blk811)) / assign16750_e15297), ((((locals.var_leff_dn4 * locals.var_t3__blk811) - (locals.var_leff * locals.var_t3__blk811_dn4)) / (locals.var_t3__blk811 * locals.var_t3__blk811)) / assign16750_e15297), ((((locals.var_leff_dn5 * locals.var_t3__blk811) - (locals.var_leff * locals.var_t3__blk811_dn5)) / (locals.var_t3__blk811 * locals.var_t3__blk811)) / assign16750_e15297), ((((locals.var_leff_dn6 * locals.var_t3__blk811) - (locals.var_leff * locals.var_t3__blk811_dn6)) / (locals.var_t3__blk811 * locals.var_t3__blk811)) / assign16750_e15297), ((((locals.var_leff_dn7 * locals.var_t3__blk811) - (locals.var_leff * locals.var_t3__blk811_dn7)) / (locals.var_t3__blk811 * locals.var_t3__blk811)) / assign16750_e15297), ((((locals.var_leff_dn8 * locals.var_t3__blk811) - (locals.var_leff * locals.var_t3__blk811_dn8)) / (locals.var_t3__blk811 * locals.var_t3__blk811)) / assign16750_e15297), ((((locals.var_leff_dn9 * locals.var_t3__blk811) - (locals.var_leff * locals.var_t3__blk811_dn9)) / (locals.var_t3__blk811 * locals.var_t3__blk811)) / assign16750_e15297), ((((locals.var_leff_dn10 * locals.var_t3__blk811) - (locals.var_leff * locals.var_t3__blk811_dn10)) / (locals.var_t3__blk811 * locals.var_t3__blk811)) / assign16750_e15297), ((((locals.var_leff_dn11 * locals.var_t3__blk811) - (locals.var_leff * locals.var_t3__blk811_dn11)) / (locals.var_t3__blk811 * locals.var_t3__blk811)) / assign16750_e15297), ((((locals.var_leff_dn12 * locals.var_t3__blk811) - (locals.var_leff * locals.var_t3__blk811_dn12)) / (locals.var_t3__blk811 * locals.var_t3__blk811)) / assign16750_e15297),)
            } else {
                let assign16750_e15300: f64 = (-87.49823353377374);
                (assign16750_e15300, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign16750_e15302: f64 = (locals.var_vtm * assign16750_e15301);
        (assign16750_e15302, (locals.var_vtm * assign16750_e15301_d_n3), ((locals.var_vtm_dn4 * assign16750_e15301) + (locals.var_vtm * assign16750_e15301_d_n4)), ((locals.var_vtm_dn5 * assign16750_e15301) + (locals.var_vtm * assign16750_e15301_d_n5)), ((locals.var_vtm_dn6 * assign16750_e15301) + (locals.var_vtm * assign16750_e15301_d_n6)), (locals.var_vtm * assign16750_e15301_d_n7), (locals.var_vtm * assign16750_e15301_d_n8), (locals.var_vtm * assign16750_e15301_d_n9), (locals.var_vtm * assign16750_e15301_d_n10), (locals.var_vtm * assign16750_e15301_d_n11), (locals.var_vtm * assign16750_e15301_d_n12),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign16750_e15304;
        locals.var_t4__blk812_dn3 = assign16750_e15304_d_n3;
        locals.var_t4__blk812_dn4 = assign16750_e15304_d_n4;
        locals.var_t4__blk812_dn5 = assign16750_e15304_d_n5;
        locals.var_t4__blk812_dn6 = assign16750_e15304_d_n6;
        locals.var_t4__blk812_dn7 = assign16750_e15304_d_n7;
        locals.var_t4__blk812_dn8 = assign16750_e15304_d_n8;
        locals.var_t4__blk812_dn9 = assign16750_e15304_d_n9;
        locals.var_t4__blk812_dn10 = assign16750_e15304_d_n10;
        locals.var_t4__blk812_dn11 = assign16750_e15304_d_n11;
        locals.var_t4__blk812_dn12 = assign16750_e15304_d_n12;
        locals.var_t4__blk812_rv = 0.0;

        let (assign16760_e15310, assign16760_e15310_d_n3, assign16760_e15310_d_n4, assign16760_e15310_d_n5, assign16760_e15310_d_n6, assign16760_e15310_d_n7, assign16760_e15310_d_n8, assign16760_e15310_d_n9, assign16760_e15310_d_n10, assign16760_e15310_d_n11, assign16760_e15310_d_n12,) = {
    if (locals.var_guard1194 != 0.0) {
        let assign16760_e15308: f64 = (locals.var_n__blk796 * locals.var_t4__blk812);
        (assign16760_e15308, ((locals.var_n__blk796_dn3 * locals.var_t4__blk812) + (locals.var_n__blk796 * locals.var_t4__blk812_dn3)), ((locals.var_n__blk796_dn4 * locals.var_t4__blk812) + (locals.var_n__blk796 * locals.var_t4__blk812_dn4)), ((locals.var_n__blk796_dn5 * locals.var_t4__blk812) + (locals.var_n__blk796 * locals.var_t4__blk812_dn5)), ((locals.var_n__blk796_dn6 * locals.var_t4__blk812) + (locals.var_n__blk796 * locals.var_t4__blk812_dn6)), ((locals.var_n__blk796_dn7 * locals.var_t4__blk812) + (locals.var_n__blk796 * locals.var_t4__blk812_dn7)), ((locals.var_n__blk796_dn8 * locals.var_t4__blk812) + (locals.var_n__blk796 * locals.var_t4__blk812_dn8)), ((locals.var_n__blk796_dn9 * locals.var_t4__blk812) + (locals.var_n__blk796 * locals.var_t4__blk812_dn9)), ((locals.var_n__blk796_dn10 * locals.var_t4__blk812) + (locals.var_n__blk796 * locals.var_t4__blk812_dn10)), ((locals.var_n__blk796_dn11 * locals.var_t4__blk812) + (locals.var_n__blk796 * locals.var_t4__blk812_dn11)), ((locals.var_n__blk796_dn12 * locals.var_t4__blk812) + (locals.var_n__blk796 * locals.var_t4__blk812_dn12)),)
    } else {
        (locals.var_dits_sft__blk1055, locals.var_dits_sft__blk1055_dn3, locals.var_dits_sft__blk1055_dn4, locals.var_dits_sft__blk1055_dn5, locals.var_dits_sft__blk1055_dn6, locals.var_dits_sft__blk1055_dn7, locals.var_dits_sft__blk1055_dn8, locals.var_dits_sft__blk1055_dn9, locals.var_dits_sft__blk1055_dn10, locals.var_dits_sft__blk1055_dn11, locals.var_dits_sft__blk1055_dn12,)
    }
};
        locals.var_dits_sft__blk1055 = assign16760_e15310;
        locals.var_dits_sft__blk1055_dn3 = assign16760_e15310_d_n3;
        locals.var_dits_sft__blk1055_dn4 = assign16760_e15310_d_n4;
        locals.var_dits_sft__blk1055_dn5 = assign16760_e15310_d_n5;
        locals.var_dits_sft__blk1055_dn6 = assign16760_e15310_d_n6;
        locals.var_dits_sft__blk1055_dn7 = assign16760_e15310_d_n7;
        locals.var_dits_sft__blk1055_dn8 = assign16760_e15310_d_n8;
        locals.var_dits_sft__blk1055_dn9 = assign16760_e15310_d_n9;
        locals.var_dits_sft__blk1055_dn10 = assign16760_e15310_d_n10;
        locals.var_dits_sft__blk1055_dn11 = assign16760_e15310_d_n11;
        locals.var_dits_sft__blk1055_dn12 = assign16760_e15310_d_n12;
        locals.var_dits_sft__blk1055_rv = 0.0;

        let (assign16770_e15315, assign16770_e15315_d_n3, assign16770_e15315_d_n4, assign16770_e15315_d_n5, assign16770_e15315_d_n6, assign16770_e15315_d_n7, assign16770_e15315_d_n8, assign16770_e15315_d_n9, assign16770_e15315_d_n10, assign16770_e15315_d_n11, assign16770_e15315_d_n12,) = {
    if (locals.var_guard1194 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dits_sft__blk1055, locals.var_dits_sft__blk1055_dn3, locals.var_dits_sft__blk1055_dn4, locals.var_dits_sft__blk1055_dn5, locals.var_dits_sft__blk1055_dn6, locals.var_dits_sft__blk1055_dn7, locals.var_dits_sft__blk1055_dn8, locals.var_dits_sft__blk1055_dn9, locals.var_dits_sft__blk1055_dn10, locals.var_dits_sft__blk1055_dn11, locals.var_dits_sft__blk1055_dn12,)
    }
};
        locals.var_dits_sft__blk1055 = assign16770_e15315;
        locals.var_dits_sft__blk1055_dn3 = assign16770_e15315_d_n3;
        locals.var_dits_sft__blk1055_dn4 = assign16770_e15315_d_n4;
        locals.var_dits_sft__blk1055_dn5 = assign16770_e15315_d_n5;
        locals.var_dits_sft__blk1055_dn6 = assign16770_e15315_d_n6;
        locals.var_dits_sft__blk1055_dn7 = assign16770_e15315_d_n7;
        locals.var_dits_sft__blk1055_dn8 = assign16770_e15315_d_n8;
        locals.var_dits_sft__blk1055_dn9 = assign16770_e15315_d_n9;
        locals.var_dits_sft__blk1055_dn10 = assign16770_e15315_d_n10;
        locals.var_dits_sft__blk1055_dn11 = assign16770_e15315_d_n11;
        locals.var_dits_sft__blk1055_dn12 = assign16770_e15315_d_n12;
        locals.var_dits_sft__blk1055_rv = 0.0;

        let assign16780_e15318: f64 = (locals.var_pparam_b4soidvt0 * locals.var_theta0__blk833);
        locals.var_b4soithetavth = assign16780_e15318;
        locals.var_b4soithetavth_dn3 = ((locals.var_pparam_b4soidvt0_dn3 * locals.var_theta0__blk833) + (locals.var_pparam_b4soidvt0 * locals.var_theta0__blk833_dn3));
        locals.var_b4soithetavth_dn4 = ((locals.var_pparam_b4soidvt0_dn4 * locals.var_theta0__blk833) + (locals.var_pparam_b4soidvt0 * locals.var_theta0__blk833_dn4));
        locals.var_b4soithetavth_dn5 = ((locals.var_pparam_b4soidvt0_dn5 * locals.var_theta0__blk833) + (locals.var_pparam_b4soidvt0 * locals.var_theta0__blk833_dn5));
        locals.var_b4soithetavth_dn6 = ((locals.var_pparam_b4soidvt0_dn6 * locals.var_theta0__blk833) + (locals.var_pparam_b4soidvt0 * locals.var_theta0__blk833_dn6));
        locals.var_b4soithetavth_dn7 = ((locals.var_pparam_b4soidvt0_dn7 * locals.var_theta0__blk833) + (locals.var_pparam_b4soidvt0 * locals.var_theta0__blk833_dn7));
        locals.var_b4soithetavth_dn8 = ((locals.var_pparam_b4soidvt0_dn8 * locals.var_theta0__blk833) + (locals.var_pparam_b4soidvt0 * locals.var_theta0__blk833_dn8));
        locals.var_b4soithetavth_dn9 = ((locals.var_pparam_b4soidvt0_dn9 * locals.var_theta0__blk833) + (locals.var_pparam_b4soidvt0 * locals.var_theta0__blk833_dn9));
        locals.var_b4soithetavth_dn10 = ((locals.var_pparam_b4soidvt0_dn10 * locals.var_theta0__blk833) + (locals.var_pparam_b4soidvt0 * locals.var_theta0__blk833_dn10));
        locals.var_b4soithetavth_dn11 = ((locals.var_pparam_b4soidvt0_dn11 * locals.var_theta0__blk833) + (locals.var_pparam_b4soidvt0 * locals.var_theta0__blk833_dn11));
        locals.var_b4soithetavth_dn12 = ((locals.var_pparam_b4soidvt0_dn12 * locals.var_theta0__blk833) + (locals.var_pparam_b4soidvt0 * locals.var_theta0__blk833_dn12));
        locals.var_b4soithetavth_rv = 0.0;

        let assign16790_e15321: f64 = (locals.var_b4soithetavth * locals.var_v0__blk799);
        locals.var_delt_vth__blk832 = assign16790_e15321;
        locals.var_delt_vth__blk832_dn3 = ((locals.var_b4soithetavth_dn3 * locals.var_v0__blk799) + (locals.var_b4soithetavth * locals.var_v0__blk799_dn3));
        locals.var_delt_vth__blk832_dn4 = ((locals.var_b4soithetavth_dn4 * locals.var_v0__blk799) + (locals.var_b4soithetavth * locals.var_v0__blk799_dn4));
        locals.var_delt_vth__blk832_dn5 = ((locals.var_b4soithetavth_dn5 * locals.var_v0__blk799) + (locals.var_b4soithetavth * locals.var_v0__blk799_dn5));
        locals.var_delt_vth__blk832_dn6 = ((locals.var_b4soithetavth_dn6 * locals.var_v0__blk799) + (locals.var_b4soithetavth * locals.var_v0__blk799_dn6));
        locals.var_delt_vth__blk832_dn7 = ((locals.var_b4soithetavth_dn7 * locals.var_v0__blk799) + (locals.var_b4soithetavth * locals.var_v0__blk799_dn7));
        locals.var_delt_vth__blk832_dn8 = ((locals.var_b4soithetavth_dn8 * locals.var_v0__blk799) + (locals.var_b4soithetavth * locals.var_v0__blk799_dn8));
        locals.var_delt_vth__blk832_dn9 = ((locals.var_b4soithetavth_dn9 * locals.var_v0__blk799) + (locals.var_b4soithetavth * locals.var_v0__blk799_dn9));
        locals.var_delt_vth__blk832_dn10 = ((locals.var_b4soithetavth_dn10 * locals.var_v0__blk799) + (locals.var_b4soithetavth * locals.var_v0__blk799_dn10));
        locals.var_delt_vth__blk832_dn11 = ((locals.var_b4soithetavth_dn11 * locals.var_v0__blk799) + (locals.var_b4soithetavth * locals.var_v0__blk799_dn11));
        locals.var_delt_vth__blk832_dn12 = ((locals.var_b4soithetavth_dn12 * locals.var_v0__blk799) + (locals.var_b4soithetavth * locals.var_v0__blk799_dn12));
        locals.var_delt_vth__blk832_rv = 0.0;

        let assign16800_e15323: f64 = (-0.5);
        let assign16800_e15325: f64 = (assign16800_e15323 * locals.var_pparam_b4soidvt1w);
        let assign16800_e15327: f64 = (assign16800_e15325 * locals.var_pparam_b4soiweff);
        let assign16800_e15329: f64 = (assign16800_e15327 * locals.var_leff);
        let assign16800_e15331: f64 = (assign16800_e15329 / locals.var_ltw__blk831);
        locals.var_t0__blk808 = assign16800_e15331;
        locals.var_t0__blk808_dn3 = ((((((((assign16800_e15323 * locals.var_pparam_b4soidvt1w_dn3) * locals.var_pparam_b4soiweff) + (assign16800_e15325 * locals.var_pparam_b4soiweff_dn3)) * locals.var_leff) + (assign16800_e15327 * locals.var_leff_dn3)) * locals.var_ltw__blk831) - (assign16800_e15329 * locals.var_ltw__blk831_dn3)) / (locals.var_ltw__blk831 * locals.var_ltw__blk831));
        locals.var_t0__blk808_dn4 = ((((((((assign16800_e15323 * locals.var_pparam_b4soidvt1w_dn4) * locals.var_pparam_b4soiweff) + (assign16800_e15325 * locals.var_pparam_b4soiweff_dn4)) * locals.var_leff) + (assign16800_e15327 * locals.var_leff_dn4)) * locals.var_ltw__blk831) - (assign16800_e15329 * locals.var_ltw__blk831_dn4)) / (locals.var_ltw__blk831 * locals.var_ltw__blk831));
        locals.var_t0__blk808_dn5 = ((((((((assign16800_e15323 * locals.var_pparam_b4soidvt1w_dn5) * locals.var_pparam_b4soiweff) + (assign16800_e15325 * locals.var_pparam_b4soiweff_dn5)) * locals.var_leff) + (assign16800_e15327 * locals.var_leff_dn5)) * locals.var_ltw__blk831) - (assign16800_e15329 * locals.var_ltw__blk831_dn5)) / (locals.var_ltw__blk831 * locals.var_ltw__blk831));
        locals.var_t0__blk808_dn6 = ((((((((assign16800_e15323 * locals.var_pparam_b4soidvt1w_dn6) * locals.var_pparam_b4soiweff) + (assign16800_e15325 * locals.var_pparam_b4soiweff_dn6)) * locals.var_leff) + (assign16800_e15327 * locals.var_leff_dn6)) * locals.var_ltw__blk831) - (assign16800_e15329 * locals.var_ltw__blk831_dn6)) / (locals.var_ltw__blk831 * locals.var_ltw__blk831));
        locals.var_t0__blk808_dn7 = ((((((((assign16800_e15323 * locals.var_pparam_b4soidvt1w_dn7) * locals.var_pparam_b4soiweff) + (assign16800_e15325 * locals.var_pparam_b4soiweff_dn7)) * locals.var_leff) + (assign16800_e15327 * locals.var_leff_dn7)) * locals.var_ltw__blk831) - (assign16800_e15329 * locals.var_ltw__blk831_dn7)) / (locals.var_ltw__blk831 * locals.var_ltw__blk831));
        locals.var_t0__blk808_dn8 = ((((((((assign16800_e15323 * locals.var_pparam_b4soidvt1w_dn8) * locals.var_pparam_b4soiweff) + (assign16800_e15325 * locals.var_pparam_b4soiweff_dn8)) * locals.var_leff) + (assign16800_e15327 * locals.var_leff_dn8)) * locals.var_ltw__blk831) - (assign16800_e15329 * locals.var_ltw__blk831_dn8)) / (locals.var_ltw__blk831 * locals.var_ltw__blk831));
        locals.var_t0__blk808_dn9 = ((((((((assign16800_e15323 * locals.var_pparam_b4soidvt1w_dn9) * locals.var_pparam_b4soiweff) + (assign16800_e15325 * locals.var_pparam_b4soiweff_dn9)) * locals.var_leff) + (assign16800_e15327 * locals.var_leff_dn9)) * locals.var_ltw__blk831) - (assign16800_e15329 * locals.var_ltw__blk831_dn9)) / (locals.var_ltw__blk831 * locals.var_ltw__blk831));
        locals.var_t0__blk808_dn10 = ((((((((assign16800_e15323 * locals.var_pparam_b4soidvt1w_dn10) * locals.var_pparam_b4soiweff) + (assign16800_e15325 * locals.var_pparam_b4soiweff_dn10)) * locals.var_leff) + (assign16800_e15327 * locals.var_leff_dn10)) * locals.var_ltw__blk831) - (assign16800_e15329 * locals.var_ltw__blk831_dn10)) / (locals.var_ltw__blk831 * locals.var_ltw__blk831));
        locals.var_t0__blk808_dn11 = ((((((((assign16800_e15323 * locals.var_pparam_b4soidvt1w_dn11) * locals.var_pparam_b4soiweff) + (assign16800_e15325 * locals.var_pparam_b4soiweff_dn11)) * locals.var_leff) + (assign16800_e15327 * locals.var_leff_dn11)) * locals.var_ltw__blk831) - (assign16800_e15329 * locals.var_ltw__blk831_dn11)) / (locals.var_ltw__blk831 * locals.var_ltw__blk831));
        locals.var_t0__blk808_dn12 = ((((((((assign16800_e15323 * locals.var_pparam_b4soidvt1w_dn12) * locals.var_pparam_b4soiweff) + (assign16800_e15325 * locals.var_pparam_b4soiweff_dn12)) * locals.var_leff) + (assign16800_e15327 * locals.var_leff_dn12)) * locals.var_ltw__blk831) - (assign16800_e15329 * locals.var_ltw__blk831_dn12)) / (locals.var_ltw__blk831 * locals.var_ltw__blk831));
        locals.var_t0__blk808_rv = 0.0;

        let assign16810_e15334: f64 = (-100.0);
        let assign16810_e15335: f64 = if locals.var_t0__blk808 > assign16810_e15334 { 1.0 } else { 0.0 };
        locals.var_guard1196 = assign16810_e15335;
        locals.var_guard1196_rv = 0.0;

        let (assign16820_e15340, assign16820_e15340_d_n3, assign16820_e15340_d_n4, assign16820_e15340_d_n5, assign16820_e15340_d_n6, assign16820_e15340_d_n7, assign16820_e15340_d_n8, assign16820_e15340_d_n9, assign16820_e15340_d_n10, assign16820_e15340_d_n11, assign16820_e15340_d_n12,) = {
    if (locals.var_guard1196 != 0.0) {
        let assign16820_e15338: f64 = (locals.var_t0__blk808).exp();
        (assign16820_e15338, (assign16820_e15338 * locals.var_t0__blk808_dn3), (assign16820_e15338 * locals.var_t0__blk808_dn4), (assign16820_e15338 * locals.var_t0__blk808_dn5), (assign16820_e15338 * locals.var_t0__blk808_dn6), (assign16820_e15338 * locals.var_t0__blk808_dn7), (assign16820_e15338 * locals.var_t0__blk808_dn8), (assign16820_e15338 * locals.var_t0__blk808_dn9), (assign16820_e15338 * locals.var_t0__blk808_dn10), (assign16820_e15338 * locals.var_t0__blk808_dn11), (assign16820_e15338 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign16820_e15340;
        locals.var_t1__blk809_dn3 = assign16820_e15340_d_n3;
        locals.var_t1__blk809_dn4 = assign16820_e15340_d_n4;
        locals.var_t1__blk809_dn5 = assign16820_e15340_d_n5;
        locals.var_t1__blk809_dn6 = assign16820_e15340_d_n6;
        locals.var_t1__blk809_dn7 = assign16820_e15340_d_n7;
        locals.var_t1__blk809_dn8 = assign16820_e15340_d_n8;
        locals.var_t1__blk809_dn9 = assign16820_e15340_d_n9;
        locals.var_t1__blk809_dn10 = assign16820_e15340_d_n10;
        locals.var_t1__blk809_dn11 = assign16820_e15340_d_n11;
        locals.var_t1__blk809_dn12 = assign16820_e15340_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign16830_e15350, assign16830_e15350_d_n3, assign16830_e15350_d_n4, assign16830_e15350_d_n5, assign16830_e15350_d_n6, assign16830_e15350_d_n7, assign16830_e15350_d_n8, assign16830_e15350_d_n9, assign16830_e15350_d_n10, assign16830_e15350_d_n11, assign16830_e15350_d_n12,) = {
    if (locals.var_guard1196 != 0.0) {
        let assign16830_e15346: f64 = (2.0 * locals.var_t1__blk809);
        let assign16830_e15347: f64 = (1.0 + assign16830_e15346);
        let assign16830_e15348: f64 = (locals.var_t1__blk809 * assign16830_e15347);
        (assign16830_e15348, ((locals.var_t1__blk809_dn3 * assign16830_e15347) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn3))), ((locals.var_t1__blk809_dn4 * assign16830_e15347) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn4))), ((locals.var_t1__blk809_dn5 * assign16830_e15347) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn5))), ((locals.var_t1__blk809_dn6 * assign16830_e15347) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn6))), ((locals.var_t1__blk809_dn7 * assign16830_e15347) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn7))), ((locals.var_t1__blk809_dn8 * assign16830_e15347) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn8))), ((locals.var_t1__blk809_dn9 * assign16830_e15347) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn9))), ((locals.var_t1__blk809_dn10 * assign16830_e15347) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn10))), ((locals.var_t1__blk809_dn11 * assign16830_e15347) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn11))), ((locals.var_t1__blk809_dn12 * assign16830_e15347) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn12))),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign16830_e15350;
        locals.var_t2__blk810_dn3 = assign16830_e15350_d_n3;
        locals.var_t2__blk810_dn4 = assign16830_e15350_d_n4;
        locals.var_t2__blk810_dn5 = assign16830_e15350_d_n5;
        locals.var_t2__blk810_dn6 = assign16830_e15350_d_n6;
        locals.var_t2__blk810_dn7 = assign16830_e15350_d_n7;
        locals.var_t2__blk810_dn8 = assign16830_e15350_d_n8;
        locals.var_t2__blk810_dn9 = assign16830_e15350_d_n9;
        locals.var_t2__blk810_dn10 = assign16830_e15350_d_n10;
        locals.var_t2__blk810_dn11 = assign16830_e15350_d_n11;
        locals.var_t2__blk810_dn12 = assign16830_e15350_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign16840_e15355, assign16840_e15355_d_n3, assign16840_e15355_d_n4, assign16840_e15355_d_n5, assign16840_e15355_d_n6, assign16840_e15355_d_n7, assign16840_e15355_d_n8, assign16840_e15355_d_n9, assign16840_e15355_d_n10, assign16840_e15355_d_n11, assign16840_e15355_d_n12,) = {
    if (locals.var_guard1196 == 0.0) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign16840_e15355;
        locals.var_t1__blk809_dn3 = assign16840_e15355_d_n3;
        locals.var_t1__blk809_dn4 = assign16840_e15355_d_n4;
        locals.var_t1__blk809_dn5 = assign16840_e15355_d_n5;
        locals.var_t1__blk809_dn6 = assign16840_e15355_d_n6;
        locals.var_t1__blk809_dn7 = assign16840_e15355_d_n7;
        locals.var_t1__blk809_dn8 = assign16840_e15355_d_n8;
        locals.var_t1__blk809_dn9 = assign16840_e15355_d_n9;
        locals.var_t1__blk809_dn10 = assign16840_e15355_d_n10;
        locals.var_t1__blk809_dn11 = assign16840_e15355_d_n11;
        locals.var_t1__blk809_dn12 = assign16840_e15355_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign16850_e15366, assign16850_e15366_d_n3, assign16850_e15366_d_n4, assign16850_e15366_d_n5, assign16850_e15366_d_n6, assign16850_e15366_d_n7, assign16850_e15366_d_n8, assign16850_e15366_d_n9, assign16850_e15366_d_n10, assign16850_e15366_d_n11, assign16850_e15366_d_n12,) = {
    if (locals.var_guard1196 == 0.0) {
        let assign16850_e15362: f64 = (2.0 * locals.var_t1__blk809);
        let assign16850_e15363: f64 = (1.0 + assign16850_e15362);
        let assign16850_e15364: f64 = (locals.var_t1__blk809 * assign16850_e15363);
        (assign16850_e15364, ((locals.var_t1__blk809_dn3 * assign16850_e15363) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn3))), ((locals.var_t1__blk809_dn4 * assign16850_e15363) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn4))), ((locals.var_t1__blk809_dn5 * assign16850_e15363) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn5))), ((locals.var_t1__blk809_dn6 * assign16850_e15363) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn6))), ((locals.var_t1__blk809_dn7 * assign16850_e15363) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn7))), ((locals.var_t1__blk809_dn8 * assign16850_e15363) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn8))), ((locals.var_t1__blk809_dn9 * assign16850_e15363) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn9))), ((locals.var_t1__blk809_dn10 * assign16850_e15363) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn10))), ((locals.var_t1__blk809_dn11 * assign16850_e15363) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn11))), ((locals.var_t1__blk809_dn12 * assign16850_e15363) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn12))),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign16850_e15366;
        locals.var_t2__blk810_dn3 = assign16850_e15366_d_n3;
        locals.var_t2__blk810_dn4 = assign16850_e15366_d_n4;
        locals.var_t2__blk810_dn5 = assign16850_e15366_d_n5;
        locals.var_t2__blk810_dn6 = assign16850_e15366_d_n6;
        locals.var_t2__blk810_dn7 = assign16850_e15366_d_n7;
        locals.var_t2__blk810_dn8 = assign16850_e15366_d_n8;
        locals.var_t2__blk810_dn9 = assign16850_e15366_d_n9;
        locals.var_t2__blk810_dn10 = assign16850_e15366_d_n10;
        locals.var_t2__blk810_dn11 = assign16850_e15366_d_n11;
        locals.var_t2__blk810_dn12 = assign16850_e15366_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let assign16860_e15369: f64 = (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810);
        locals.var_t0__blk808 = assign16860_e15369;
        locals.var_t0__blk808_dn3 = ((locals.var_pparam_b4soidvt0w_dn3 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn3));
        locals.var_t0__blk808_dn4 = ((locals.var_pparam_b4soidvt0w_dn4 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn4));
        locals.var_t0__blk808_dn5 = ((locals.var_pparam_b4soidvt0w_dn5 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn5));
        locals.var_t0__blk808_dn6 = ((locals.var_pparam_b4soidvt0w_dn6 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn6));
        locals.var_t0__blk808_dn7 = ((locals.var_pparam_b4soidvt0w_dn7 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn7));
        locals.var_t0__blk808_dn8 = ((locals.var_pparam_b4soidvt0w_dn8 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn8));
        locals.var_t0__blk808_dn9 = ((locals.var_pparam_b4soidvt0w_dn9 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn9));
        locals.var_t0__blk808_dn10 = ((locals.var_pparam_b4soidvt0w_dn10 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn10));
        locals.var_t0__blk808_dn11 = ((locals.var_pparam_b4soidvt0w_dn11 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn11));
        locals.var_t0__blk808_dn12 = ((locals.var_pparam_b4soidvt0w_dn12 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn12));
        locals.var_t0__blk808_rv = 0.0;

        let assign16870_e15372: f64 = (locals.var_t0__blk808 * locals.var_v0__blk799);
        locals.var_deltvthw__blk869 = assign16870_e15372;
        locals.var_deltvthw__blk869_dn3 = ((locals.var_t0__blk808_dn3 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn3));
        locals.var_deltvthw__blk869_dn4 = ((locals.var_t0__blk808_dn4 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn4));
        locals.var_deltvthw__blk869_dn5 = ((locals.var_t0__blk808_dn5 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn5));
        locals.var_deltvthw__blk869_dn6 = ((locals.var_t0__blk808_dn6 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn6));
        locals.var_deltvthw__blk869_dn7 = ((locals.var_t0__blk808_dn7 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn7));
        locals.var_deltvthw__blk869_dn8 = ((locals.var_t0__blk808_dn8 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn8));
        locals.var_deltvthw__blk869_dn9 = ((locals.var_t0__blk808_dn9 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn9));
        locals.var_deltvthw__blk869_dn10 = ((locals.var_t0__blk808_dn10 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn10));
        locals.var_deltvthw__blk869_dn11 = ((locals.var_t0__blk808_dn11 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn11));
        locals.var_deltvthw__blk869_dn12 = ((locals.var_t0__blk808_dn12 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn12));
        locals.var_deltvthw__blk869_rv = 0.0;

        let assign16880_e15376: f64 = (locals.var_pparam_b4soilpe0 / locals.var_leff);
        let assign16880_e15377: f64 = (1.0 + assign16880_e15376);
        let assign16880_e15378: f64 = (assign16880_e15377).sqrt();
        locals.var_t0__blk808 = assign16880_e15378;
        locals.var_t0__blk808_dn3 = ((((locals.var_pparam_b4soilpe0_dn3 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn3)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign16880_e15378));
        locals.var_t0__blk808_dn4 = ((((locals.var_pparam_b4soilpe0_dn4 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn4)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign16880_e15378));
        locals.var_t0__blk808_dn5 = ((((locals.var_pparam_b4soilpe0_dn5 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn5)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign16880_e15378));
        locals.var_t0__blk808_dn6 = ((((locals.var_pparam_b4soilpe0_dn6 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn6)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign16880_e15378));
        locals.var_t0__blk808_dn7 = ((((locals.var_pparam_b4soilpe0_dn7 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn7)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign16880_e15378));
        locals.var_t0__blk808_dn8 = ((((locals.var_pparam_b4soilpe0_dn8 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn8)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign16880_e15378));
        locals.var_t0__blk808_dn9 = ((((locals.var_pparam_b4soilpe0_dn9 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn9)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign16880_e15378));
        locals.var_t0__blk808_dn10 = ((((locals.var_pparam_b4soilpe0_dn10 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn10)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign16880_e15378));
        locals.var_t0__blk808_dn11 = ((((locals.var_pparam_b4soilpe0_dn11 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn11)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign16880_e15378));
        locals.var_t0__blk808_dn12 = ((((locals.var_pparam_b4soilpe0_dn12 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn12)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign16880_e15378));
        locals.var_t0__blk808_rv = 0.0;

        let assign16890_e15382: f64 = (locals.var_pparam_b4soikt1l / locals.var_leff);
        let assign16890_e15383: f64 = (locals.var_pparam_b4soikt1 + assign16890_e15382);
        let assign16890_e15386: f64 = (locals.var_pparam_b4soikt2 * locals.var_vbseff);
        let assign16890_e15387: f64 = (assign16890_e15383 + assign16890_e15386);
        locals.var_t1__blk809 = assign16890_e15387;
        locals.var_t1__blk809_dn3 = ((locals.var_pparam_b4soikt1_dn3 + (((locals.var_pparam_b4soikt1l_dn3 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn3)) / (locals.var_leff * locals.var_leff))) + ((locals.var_pparam_b4soikt2_dn3 * locals.var_vbseff) + (locals.var_pparam_b4soikt2 * locals.var_vbseff_dn3)));
        locals.var_t1__blk809_dn4 = ((locals.var_pparam_b4soikt1_dn4 + (((locals.var_pparam_b4soikt1l_dn4 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn4)) / (locals.var_leff * locals.var_leff))) + ((locals.var_pparam_b4soikt2_dn4 * locals.var_vbseff) + (locals.var_pparam_b4soikt2 * locals.var_vbseff_dn4)));
        locals.var_t1__blk809_dn5 = ((locals.var_pparam_b4soikt1_dn5 + (((locals.var_pparam_b4soikt1l_dn5 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn5)) / (locals.var_leff * locals.var_leff))) + ((locals.var_pparam_b4soikt2_dn5 * locals.var_vbseff) + (locals.var_pparam_b4soikt2 * locals.var_vbseff_dn5)));
        locals.var_t1__blk809_dn6 = ((locals.var_pparam_b4soikt1_dn6 + (((locals.var_pparam_b4soikt1l_dn6 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn6)) / (locals.var_leff * locals.var_leff))) + ((locals.var_pparam_b4soikt2_dn6 * locals.var_vbseff) + (locals.var_pparam_b4soikt2 * locals.var_vbseff_dn6)));
        locals.var_t1__blk809_dn7 = ((locals.var_pparam_b4soikt1_dn7 + (((locals.var_pparam_b4soikt1l_dn7 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn7)) / (locals.var_leff * locals.var_leff))) + ((locals.var_pparam_b4soikt2_dn7 * locals.var_vbseff) + (locals.var_pparam_b4soikt2 * locals.var_vbseff_dn7)));
        locals.var_t1__blk809_dn8 = ((locals.var_pparam_b4soikt1_dn8 + (((locals.var_pparam_b4soikt1l_dn8 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn8)) / (locals.var_leff * locals.var_leff))) + ((locals.var_pparam_b4soikt2_dn8 * locals.var_vbseff) + (locals.var_pparam_b4soikt2 * locals.var_vbseff_dn8)));
        locals.var_t1__blk809_dn9 = ((locals.var_pparam_b4soikt1_dn9 + (((locals.var_pparam_b4soikt1l_dn9 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn9)) / (locals.var_leff * locals.var_leff))) + ((locals.var_pparam_b4soikt2_dn9 * locals.var_vbseff) + (locals.var_pparam_b4soikt2 * locals.var_vbseff_dn9)));
        locals.var_t1__blk809_dn10 = ((locals.var_pparam_b4soikt1_dn10 + (((locals.var_pparam_b4soikt1l_dn10 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn10)) / (locals.var_leff * locals.var_leff))) + ((locals.var_pparam_b4soikt2_dn10 * locals.var_vbseff) + (locals.var_pparam_b4soikt2 * locals.var_vbseff_dn10)));
        locals.var_t1__blk809_dn11 = ((locals.var_pparam_b4soikt1_dn11 + (((locals.var_pparam_b4soikt1l_dn11 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn11)) / (locals.var_leff * locals.var_leff))) + ((locals.var_pparam_b4soikt2_dn11 * locals.var_vbseff) + (locals.var_pparam_b4soikt2 * locals.var_vbseff_dn11)));
        locals.var_t1__blk809_dn12 = ((locals.var_pparam_b4soikt1_dn12 + (((locals.var_pparam_b4soikt1l_dn12 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn12)) / (locals.var_leff * locals.var_leff))) + ((locals.var_pparam_b4soikt2_dn12 * locals.var_vbseff) + (locals.var_pparam_b4soikt2 * locals.var_vbseff_dn12)));
        locals.var_t1__blk809_rv = 0.0;

        let assign16900_e15391: f64 = (locals.var_t0__blk808 - 1.0);
        let assign16900_e15392: f64 = (locals.var_here_b4soik1ox * assign16900_e15391);
        let assign16900_e15394: f64 = (assign16900_e15392 * locals.var_sqrtphi);
        let assign16900_e15397: f64 = (locals.var_t1__blk809 * locals.var_trm1);
        let assign16900_e15398: f64 = (assign16900_e15394 + assign16900_e15397);
        locals.var_deltvthtemp__blk868 = assign16900_e15398;
        locals.var_deltvthtemp__blk868_dn3 = (((((locals.var_here_b4soik1ox_dn3 * assign16900_e15391) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn3)) * locals.var_sqrtphi) + (assign16900_e15392 * locals.var_sqrtphi_dn3)) + (locals.var_t1__blk809_dn3 * locals.var_trm1));
        locals.var_deltvthtemp__blk868_dn4 = (((((locals.var_here_b4soik1ox_dn4 * assign16900_e15391) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn4)) * locals.var_sqrtphi) + (assign16900_e15392 * locals.var_sqrtphi_dn4)) + ((locals.var_t1__blk809_dn4 * locals.var_trm1) + (locals.var_t1__blk809 * locals.var_trm1_dn4)));
        locals.var_deltvthtemp__blk868_dn5 = (((((locals.var_here_b4soik1ox_dn5 * assign16900_e15391) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn5)) * locals.var_sqrtphi) + (assign16900_e15392 * locals.var_sqrtphi_dn5)) + ((locals.var_t1__blk809_dn5 * locals.var_trm1) + (locals.var_t1__blk809 * locals.var_trm1_dn5)));
        locals.var_deltvthtemp__blk868_dn6 = (((((locals.var_here_b4soik1ox_dn6 * assign16900_e15391) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn6)) * locals.var_sqrtphi) + (assign16900_e15392 * locals.var_sqrtphi_dn6)) + ((locals.var_t1__blk809_dn6 * locals.var_trm1) + (locals.var_t1__blk809 * locals.var_trm1_dn6)));
        locals.var_deltvthtemp__blk868_dn7 = (((((locals.var_here_b4soik1ox_dn7 * assign16900_e15391) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn7)) * locals.var_sqrtphi) + (assign16900_e15392 * locals.var_sqrtphi_dn7)) + (locals.var_t1__blk809_dn7 * locals.var_trm1));
        locals.var_deltvthtemp__blk868_dn8 = (((((locals.var_here_b4soik1ox_dn8 * assign16900_e15391) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn8)) * locals.var_sqrtphi) + (assign16900_e15392 * locals.var_sqrtphi_dn8)) + (locals.var_t1__blk809_dn8 * locals.var_trm1));
        locals.var_deltvthtemp__blk868_dn9 = (((((locals.var_here_b4soik1ox_dn9 * assign16900_e15391) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn9)) * locals.var_sqrtphi) + (assign16900_e15392 * locals.var_sqrtphi_dn9)) + (locals.var_t1__blk809_dn9 * locals.var_trm1));
        locals.var_deltvthtemp__blk868_dn10 = (((((locals.var_here_b4soik1ox_dn10 * assign16900_e15391) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn10)) * locals.var_sqrtphi) + (assign16900_e15392 * locals.var_sqrtphi_dn10)) + (locals.var_t1__blk809_dn10 * locals.var_trm1));
        locals.var_deltvthtemp__blk868_dn11 = (((((locals.var_here_b4soik1ox_dn11 * assign16900_e15391) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn11)) * locals.var_sqrtphi) + (assign16900_e15392 * locals.var_sqrtphi_dn11)) + (locals.var_t1__blk809_dn11 * locals.var_trm1));
        locals.var_deltvthtemp__blk868_dn12 = (((((locals.var_here_b4soik1ox_dn12 * assign16900_e15391) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn12)) * locals.var_sqrtphi) + (assign16900_e15392 * locals.var_sqrtphi_dn12)) + (locals.var_t1__blk809_dn12 * locals.var_trm1));
        locals.var_deltvthtemp__blk868_rv = 0.0;

        let assign16910_e15401: f64 = (locals.var_toxe * locals.var_phi);
        let assign16910_e15404: f64 = (locals.var_pparam_b4soiweff + locals.var_pparam_b4soiw0);
        let assign16910_e15405: f64 = (assign16910_e15401 / assign16910_e15404);
        locals.var_tmp2__blk835 = assign16910_e15405;
        locals.var_tmp2__blk835_dn3 = ((((locals.var_toxe * locals.var_phi_dn3) * assign16910_e15404) - (assign16910_e15401 * (locals.var_pparam_b4soiweff_dn3 + locals.var_pparam_b4soiw0_dn3))) / (assign16910_e15404 * assign16910_e15404));
        locals.var_tmp2__blk835_dn4 = ((((locals.var_toxe * locals.var_phi_dn4) * assign16910_e15404) - (assign16910_e15401 * (locals.var_pparam_b4soiweff_dn4 + locals.var_pparam_b4soiw0_dn4))) / (assign16910_e15404 * assign16910_e15404));
        locals.var_tmp2__blk835_dn5 = ((((locals.var_toxe * locals.var_phi_dn5) * assign16910_e15404) - (assign16910_e15401 * (locals.var_pparam_b4soiweff_dn5 + locals.var_pparam_b4soiw0_dn5))) / (assign16910_e15404 * assign16910_e15404));
        locals.var_tmp2__blk835_dn6 = ((((locals.var_toxe * locals.var_phi_dn6) * assign16910_e15404) - (assign16910_e15401 * (locals.var_pparam_b4soiweff_dn6 + locals.var_pparam_b4soiw0_dn6))) / (assign16910_e15404 * assign16910_e15404));
        locals.var_tmp2__blk835_dn7 = ((((locals.var_toxe * locals.var_phi_dn7) * assign16910_e15404) - (assign16910_e15401 * (locals.var_pparam_b4soiweff_dn7 + locals.var_pparam_b4soiw0_dn7))) / (assign16910_e15404 * assign16910_e15404));
        locals.var_tmp2__blk835_dn8 = ((((locals.var_toxe * locals.var_phi_dn8) * assign16910_e15404) - (assign16910_e15401 * (locals.var_pparam_b4soiweff_dn8 + locals.var_pparam_b4soiw0_dn8))) / (assign16910_e15404 * assign16910_e15404));
        locals.var_tmp2__blk835_dn9 = ((((locals.var_toxe * locals.var_phi_dn9) * assign16910_e15404) - (assign16910_e15401 * (locals.var_pparam_b4soiweff_dn9 + locals.var_pparam_b4soiw0_dn9))) / (assign16910_e15404 * assign16910_e15404));
        locals.var_tmp2__blk835_dn10 = ((((locals.var_toxe * locals.var_phi_dn10) * assign16910_e15404) - (assign16910_e15401 * (locals.var_pparam_b4soiweff_dn10 + locals.var_pparam_b4soiw0_dn10))) / (assign16910_e15404 * assign16910_e15404));
        locals.var_tmp2__blk835_dn11 = ((((locals.var_toxe * locals.var_phi_dn11) * assign16910_e15404) - (assign16910_e15401 * (locals.var_pparam_b4soiweff_dn11 + locals.var_pparam_b4soiw0_dn11))) / (assign16910_e15404 * assign16910_e15404));
        locals.var_tmp2__blk835_dn12 = ((((locals.var_toxe * locals.var_phi_dn12) * assign16910_e15404) - (assign16910_e15401 * (locals.var_pparam_b4soiweff_dn12 + locals.var_pparam_b4soiw0_dn12))) / (assign16910_e15404 * assign16910_e15404));
        locals.var_tmp2__blk835_rv = 0.0;

        let assign16920_e15409: f64 = (locals.var_pparam_b4soietab * locals.var_vbseff);
        let assign16920_e15410: f64 = (locals.var_here_b4soieta0 + assign16920_e15409);
        locals.var_t3__blk811 = assign16920_e15410;
        locals.var_t3__blk811_dn3 = (locals.var_here_b4soieta0_dn3 + ((locals.var_pparam_b4soietab_dn3 * locals.var_vbseff) + (locals.var_pparam_b4soietab * locals.var_vbseff_dn3)));
        locals.var_t3__blk811_dn4 = (locals.var_here_b4soieta0_dn4 + ((locals.var_pparam_b4soietab_dn4 * locals.var_vbseff) + (locals.var_pparam_b4soietab * locals.var_vbseff_dn4)));
        locals.var_t3__blk811_dn5 = (locals.var_here_b4soieta0_dn5 + ((locals.var_pparam_b4soietab_dn5 * locals.var_vbseff) + (locals.var_pparam_b4soietab * locals.var_vbseff_dn5)));
        locals.var_t3__blk811_dn6 = (locals.var_here_b4soieta0_dn6 + ((locals.var_pparam_b4soietab_dn6 * locals.var_vbseff) + (locals.var_pparam_b4soietab * locals.var_vbseff_dn6)));
        locals.var_t3__blk811_dn7 = (locals.var_here_b4soieta0_dn7 + ((locals.var_pparam_b4soietab_dn7 * locals.var_vbseff) + (locals.var_pparam_b4soietab * locals.var_vbseff_dn7)));
        locals.var_t3__blk811_dn8 = (locals.var_here_b4soieta0_dn8 + ((locals.var_pparam_b4soietab_dn8 * locals.var_vbseff) + (locals.var_pparam_b4soietab * locals.var_vbseff_dn8)));
        locals.var_t3__blk811_dn9 = (locals.var_here_b4soieta0_dn9 + ((locals.var_pparam_b4soietab_dn9 * locals.var_vbseff) + (locals.var_pparam_b4soietab * locals.var_vbseff_dn9)));
        locals.var_t3__blk811_dn10 = (locals.var_here_b4soieta0_dn10 + ((locals.var_pparam_b4soietab_dn10 * locals.var_vbseff) + (locals.var_pparam_b4soietab * locals.var_vbseff_dn10)));
        locals.var_t3__blk811_dn11 = (locals.var_here_b4soieta0_dn11 + ((locals.var_pparam_b4soietab_dn11 * locals.var_vbseff) + (locals.var_pparam_b4soietab * locals.var_vbseff_dn11)));
        locals.var_t3__blk811_dn12 = (locals.var_here_b4soieta0_dn12 + ((locals.var_pparam_b4soietab_dn12 * locals.var_vbseff) + (locals.var_pparam_b4soietab * locals.var_vbseff_dn12)));
        locals.var_t3__blk811_rv = 0.0;

        let assign16930_e15413: f64 = if locals.var_t3__blk811 < 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard1197 = assign16930_e15413;
        locals.var_guard1197_rv = 0.0;

        let (assign16940_e15423, assign16940_e15423_d_n3, assign16940_e15423_d_n4, assign16940_e15423_d_n5, assign16940_e15423_d_n6, assign16940_e15423_d_n7, assign16940_e15423_d_n8, assign16940_e15423_d_n9, assign16940_e15423_d_n10, assign16940_e15423_d_n11, assign16940_e15423_d_n12,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign16940_e15419: f64 = (20000.0 * locals.var_t3__blk811);
        let assign16940_e15420: f64 = (3.0 - assign16940_e15419);
        let assign16940_e15421: f64 = (1.0 / assign16940_e15420);
        (assign16940_e15421, (-((-(20000.0 * locals.var_t3__blk811_dn3)) / (assign16940_e15420 * assign16940_e15420))), (-((-(20000.0 * locals.var_t3__blk811_dn4)) / (assign16940_e15420 * assign16940_e15420))), (-((-(20000.0 * locals.var_t3__blk811_dn5)) / (assign16940_e15420 * assign16940_e15420))), (-((-(20000.0 * locals.var_t3__blk811_dn6)) / (assign16940_e15420 * assign16940_e15420))), (-((-(20000.0 * locals.var_t3__blk811_dn7)) / (assign16940_e15420 * assign16940_e15420))), (-((-(20000.0 * locals.var_t3__blk811_dn8)) / (assign16940_e15420 * assign16940_e15420))), (-((-(20000.0 * locals.var_t3__blk811_dn9)) / (assign16940_e15420 * assign16940_e15420))), (-((-(20000.0 * locals.var_t3__blk811_dn10)) / (assign16940_e15420 * assign16940_e15420))), (-((-(20000.0 * locals.var_t3__blk811_dn11)) / (assign16940_e15420 * assign16940_e15420))), (-((-(20000.0 * locals.var_t3__blk811_dn12)) / (assign16940_e15420 * assign16940_e15420))),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign16940_e15423;
        locals.var_t9_dn3 = assign16940_e15423_d_n3;
        locals.var_t9_dn4 = assign16940_e15423_d_n4;
        locals.var_t9_dn5 = assign16940_e15423_d_n5;
        locals.var_t9_dn6 = assign16940_e15423_d_n6;
        locals.var_t9_dn7 = assign16940_e15423_d_n7;
        locals.var_t9_dn8 = assign16940_e15423_d_n8;
        locals.var_t9_dn9 = assign16940_e15423_d_n9;
        locals.var_t9_dn10 = assign16940_e15423_d_n10;
        locals.var_t9_dn11 = assign16940_e15423_d_n11;
        locals.var_t9_dn12 = assign16940_e15423_d_n12;
        locals.var_t9_rv = 0.0;

        let (assign16950_e15431, assign16950_e15431_d_n3, assign16950_e15431_d_n4, assign16950_e15431_d_n5, assign16950_e15431_d_n6, assign16950_e15431_d_n7, assign16950_e15431_d_n8, assign16950_e15431_d_n9, assign16950_e15431_d_n10, assign16950_e15431_d_n11, assign16950_e15431_d_n12,) = {
    if (locals.var_guard1197 != 0.0) {
        let assign16950_e15427: f64 = (0.0002 - locals.var_t3__blk811);
        let assign16950_e15429: f64 = (assign16950_e15427 * locals.var_t9);
        (assign16950_e15429, (((-locals.var_t3__blk811_dn3) * locals.var_t9) + (assign16950_e15427 * locals.var_t9_dn3)), (((-locals.var_t3__blk811_dn4) * locals.var_t9) + (assign16950_e15427 * locals.var_t9_dn4)), (((-locals.var_t3__blk811_dn5) * locals.var_t9) + (assign16950_e15427 * locals.var_t9_dn5)), (((-locals.var_t3__blk811_dn6) * locals.var_t9) + (assign16950_e15427 * locals.var_t9_dn6)), (((-locals.var_t3__blk811_dn7) * locals.var_t9) + (assign16950_e15427 * locals.var_t9_dn7)), (((-locals.var_t3__blk811_dn8) * locals.var_t9) + (assign16950_e15427 * locals.var_t9_dn8)), (((-locals.var_t3__blk811_dn9) * locals.var_t9) + (assign16950_e15427 * locals.var_t9_dn9)), (((-locals.var_t3__blk811_dn10) * locals.var_t9) + (assign16950_e15427 * locals.var_t9_dn10)), (((-locals.var_t3__blk811_dn11) * locals.var_t9) + (assign16950_e15427 * locals.var_t9_dn11)), (((-locals.var_t3__blk811_dn12) * locals.var_t9) + (assign16950_e15427 * locals.var_t9_dn12)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign16950_e15431;
        locals.var_t3__blk811_dn3 = assign16950_e15431_d_n3;
        locals.var_t3__blk811_dn4 = assign16950_e15431_d_n4;
        locals.var_t3__blk811_dn5 = assign16950_e15431_d_n5;
        locals.var_t3__blk811_dn6 = assign16950_e15431_d_n6;
        locals.var_t3__blk811_dn7 = assign16950_e15431_d_n7;
        locals.var_t3__blk811_dn8 = assign16950_e15431_d_n8;
        locals.var_t3__blk811_dn9 = assign16950_e15431_d_n9;
        locals.var_t3__blk811_dn10 = assign16950_e15431_d_n10;
        locals.var_t3__blk811_dn11 = assign16950_e15431_d_n11;
        locals.var_t3__blk811_dn12 = assign16950_e15431_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let assign16960_e15434: f64 = (locals.var_t3__blk811 * locals.var_theta0vb0);
        let assign16960_e15436: f64 = (assign16960_e15434 * locals.var_vds_1);
        locals.var_dibl_sft__blk838 = assign16960_e15436;
        locals.var_dibl_sft__blk838_dn3 = (((locals.var_t3__blk811_dn3 * locals.var_theta0vb0) + (locals.var_t3__blk811 * locals.var_theta0vb0_dn3)) * locals.var_vds_1);
        locals.var_dibl_sft__blk838_dn4 = (((locals.var_t3__blk811_dn4 * locals.var_theta0vb0) + (locals.var_t3__blk811 * locals.var_theta0vb0_dn4)) * locals.var_vds_1);
        locals.var_dibl_sft__blk838_dn5 = (((locals.var_t3__blk811_dn5 * locals.var_theta0vb0) + (locals.var_t3__blk811 * locals.var_theta0vb0_dn5)) * locals.var_vds_1);
        locals.var_dibl_sft__blk838_dn6 = (((locals.var_t3__blk811_dn6 * locals.var_theta0vb0) + (locals.var_t3__blk811 * locals.var_theta0vb0_dn6)) * locals.var_vds_1);
        locals.var_dibl_sft__blk838_dn7 = ((((locals.var_t3__blk811_dn7 * locals.var_theta0vb0) + (locals.var_t3__blk811 * locals.var_theta0vb0_dn7)) * locals.var_vds_1) + (assign16960_e15434 * locals.var_vds_1_dn7));
        locals.var_dibl_sft__blk838_dn8 = ((((locals.var_t3__blk811_dn8 * locals.var_theta0vb0) + (locals.var_t3__blk811 * locals.var_theta0vb0_dn8)) * locals.var_vds_1) + (assign16960_e15434 * locals.var_vds_1_dn8));
        locals.var_dibl_sft__blk838_dn9 = (((locals.var_t3__blk811_dn9 * locals.var_theta0vb0) + (locals.var_t3__blk811 * locals.var_theta0vb0_dn9)) * locals.var_vds_1);
        locals.var_dibl_sft__blk838_dn10 = (((locals.var_t3__blk811_dn10 * locals.var_theta0vb0) + (locals.var_t3__blk811 * locals.var_theta0vb0_dn10)) * locals.var_vds_1);
        locals.var_dibl_sft__blk838_dn11 = (((locals.var_t3__blk811_dn11 * locals.var_theta0vb0) + (locals.var_t3__blk811 * locals.var_theta0vb0_dn11)) * locals.var_vds_1);
        locals.var_dibl_sft__blk838_dn12 = (((locals.var_t3__blk811_dn12 * locals.var_theta0vb0) + (locals.var_t3__blk811 * locals.var_theta0vb0_dn12)) * locals.var_vds_1);
        locals.var_dibl_sft__blk838_rv = 0.0;

        let assign16970_e15440: f64 = (locals.var_pparam_b4soilpeb / locals.var_leff);
        let assign16970_e15441: f64 = (1.0 + assign16970_e15440);
        let assign16970_e15442: f64 = (assign16970_e15441).sqrt();
        locals.var_lpe_vb__blk1054 = assign16970_e15442;
        locals.var_lpe_vb__blk1054_dn3 = ((((locals.var_pparam_b4soilpeb_dn3 * locals.var_leff) - (locals.var_pparam_b4soilpeb * locals.var_leff_dn3)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign16970_e15442));
        locals.var_lpe_vb__blk1054_dn4 = ((((locals.var_pparam_b4soilpeb_dn4 * locals.var_leff) - (locals.var_pparam_b4soilpeb * locals.var_leff_dn4)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign16970_e15442));
        locals.var_lpe_vb__blk1054_dn5 = ((((locals.var_pparam_b4soilpeb_dn5 * locals.var_leff) - (locals.var_pparam_b4soilpeb * locals.var_leff_dn5)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign16970_e15442));
        locals.var_lpe_vb__blk1054_dn6 = ((((locals.var_pparam_b4soilpeb_dn6 * locals.var_leff) - (locals.var_pparam_b4soilpeb * locals.var_leff_dn6)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign16970_e15442));
        locals.var_lpe_vb__blk1054_dn7 = ((((locals.var_pparam_b4soilpeb_dn7 * locals.var_leff) - (locals.var_pparam_b4soilpeb * locals.var_leff_dn7)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign16970_e15442));
        locals.var_lpe_vb__blk1054_dn8 = ((((locals.var_pparam_b4soilpeb_dn8 * locals.var_leff) - (locals.var_pparam_b4soilpeb * locals.var_leff_dn8)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign16970_e15442));
        locals.var_lpe_vb__blk1054_dn9 = ((((locals.var_pparam_b4soilpeb_dn9 * locals.var_leff) - (locals.var_pparam_b4soilpeb * locals.var_leff_dn9)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign16970_e15442));
        locals.var_lpe_vb__blk1054_dn10 = ((((locals.var_pparam_b4soilpeb_dn10 * locals.var_leff) - (locals.var_pparam_b4soilpeb * locals.var_leff_dn10)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign16970_e15442));
        locals.var_lpe_vb__blk1054_dn11 = ((((locals.var_pparam_b4soilpeb_dn11 * locals.var_leff) - (locals.var_pparam_b4soilpeb * locals.var_leff_dn11)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign16970_e15442));
        locals.var_lpe_vb__blk1054_dn12 = ((((locals.var_pparam_b4soilpeb_dn12 * locals.var_leff) - (locals.var_pparam_b4soilpeb * locals.var_leff_dn12)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign16970_e15442));
        locals.var_lpe_vb__blk1054_rv = 0.0;

        let assign16980_e15445: f64 = (2.2361 / locals.var_sqrtphi);
        locals.var_t9 = assign16980_e15445;
        locals.var_t9_dn3 = (-((2.2361 * locals.var_sqrtphi_dn3) / (locals.var_sqrtphi * locals.var_sqrtphi)));
        locals.var_t9_dn4 = (-((2.2361 * locals.var_sqrtphi_dn4) / (locals.var_sqrtphi * locals.var_sqrtphi)));
        locals.var_t9_dn5 = (-((2.2361 * locals.var_sqrtphi_dn5) / (locals.var_sqrtphi * locals.var_sqrtphi)));
        locals.var_t9_dn6 = (-((2.2361 * locals.var_sqrtphi_dn6) / (locals.var_sqrtphi * locals.var_sqrtphi)));
        locals.var_t9_dn7 = (-((2.2361 * locals.var_sqrtphi_dn7) / (locals.var_sqrtphi * locals.var_sqrtphi)));
        locals.var_t9_dn8 = (-((2.2361 * locals.var_sqrtphi_dn8) / (locals.var_sqrtphi * locals.var_sqrtphi)));
        locals.var_t9_dn9 = (-((2.2361 * locals.var_sqrtphi_dn9) / (locals.var_sqrtphi * locals.var_sqrtphi)));
        locals.var_t9_dn10 = (-((2.2361 * locals.var_sqrtphi_dn10) / (locals.var_sqrtphi * locals.var_sqrtphi)));
        locals.var_t9_dn11 = (-((2.2361 * locals.var_sqrtphi_dn11) / (locals.var_sqrtphi * locals.var_sqrtphi)));
        locals.var_t9_dn12 = (-((2.2361 * locals.var_sqrtphi_dn12) / (locals.var_sqrtphi * locals.var_sqrtphi)));
        locals.var_t9_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_50(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign16990_e15450: f64 = (locals.var_vbsh - locals.var_vbseff);
        let assign16990_e15451: f64 = (locals.var_t9 * assign16990_e15450);
        let assign16990_e15452: f64 = (locals.var_sqrtphis - assign16990_e15451);
        locals.var_sqrtphisext__blk928 = assign16990_e15452;
        locals.var_sqrtphisext__blk928_dn3 = (locals.var_sqrtphis_dn3 - ((locals.var_t9_dn3 * assign16990_e15450) + (locals.var_t9 * (locals.var_vbsh_dn3 - locals.var_vbseff_dn3))));
        locals.var_sqrtphisext__blk928_dn4 = (locals.var_sqrtphis_dn4 - ((locals.var_t9_dn4 * assign16990_e15450) + (locals.var_t9 * (locals.var_vbsh_dn4 - locals.var_vbseff_dn4))));
        locals.var_sqrtphisext__blk928_dn5 = (locals.var_sqrtphis_dn5 - ((locals.var_t9_dn5 * assign16990_e15450) + (locals.var_t9 * (locals.var_vbsh_dn5 - locals.var_vbseff_dn5))));
        locals.var_sqrtphisext__blk928_dn6 = (locals.var_sqrtphis_dn6 - ((locals.var_t9_dn6 * assign16990_e15450) + (locals.var_t9 * (locals.var_vbsh_dn6 - locals.var_vbseff_dn6))));
        locals.var_sqrtphisext__blk928_dn7 = (locals.var_sqrtphis_dn7 - ((locals.var_t9_dn7 * assign16990_e15450) + (locals.var_t9 * (locals.var_vbsh_dn7 - locals.var_vbseff_dn7))));
        locals.var_sqrtphisext__blk928_dn8 = (locals.var_sqrtphis_dn8 - ((locals.var_t9_dn8 * assign16990_e15450) + (locals.var_t9 * (locals.var_vbsh_dn8 - locals.var_vbseff_dn8))));
        locals.var_sqrtphisext__blk928_dn9 = (locals.var_sqrtphis_dn9 - ((locals.var_t9_dn9 * assign16990_e15450) + (locals.var_t9 * (locals.var_vbsh_dn9 - locals.var_vbseff_dn9))));
        locals.var_sqrtphisext__blk928_dn10 = (locals.var_sqrtphis_dn10 - ((locals.var_t9_dn10 * assign16990_e15450) + (locals.var_t9 * (locals.var_vbsh_dn10 - locals.var_vbseff_dn10))));
        locals.var_sqrtphisext__blk928_dn11 = (locals.var_sqrtphis_dn11 - ((locals.var_t9_dn11 * assign16990_e15450) + (locals.var_t9 * (locals.var_vbsh_dn11 - locals.var_vbseff_dn11))));
        locals.var_sqrtphisext__blk928_dn12 = (locals.var_sqrtphis_dn12 - ((locals.var_t9_dn12 * assign16990_e15450) + (locals.var_t9 * (locals.var_vbsh_dn12 - locals.var_vbseff_dn12))));
        locals.var_sqrtphisext__blk928_rv = 0.0;

        let assign17000_e15455: f64 = (2.0 * locals.var_pparam_b4soidvtp4);
        let assign17000_e15457: f64 = (assign17000_e15455 * locals.var_vds_1);
        let assign17000_e15458: f64 = (assign17000_e15457).exp();
        locals.var_t0__blk808 = assign17000_e15458;
        locals.var_t0__blk808_dn3 = (assign17000_e15458 * ((2.0 * locals.var_pparam_b4soidvtp4_dn3) * locals.var_vds_1));
        locals.var_t0__blk808_dn4 = (assign17000_e15458 * ((2.0 * locals.var_pparam_b4soidvtp4_dn4) * locals.var_vds_1));
        locals.var_t0__blk808_dn5 = (assign17000_e15458 * ((2.0 * locals.var_pparam_b4soidvtp4_dn5) * locals.var_vds_1));
        locals.var_t0__blk808_dn6 = (assign17000_e15458 * ((2.0 * locals.var_pparam_b4soidvtp4_dn6) * locals.var_vds_1));
        locals.var_t0__blk808_dn7 = (assign17000_e15458 * (((2.0 * locals.var_pparam_b4soidvtp4_dn7) * locals.var_vds_1) + (assign17000_e15455 * locals.var_vds_1_dn7)));
        locals.var_t0__blk808_dn8 = (assign17000_e15458 * (((2.0 * locals.var_pparam_b4soidvtp4_dn8) * locals.var_vds_1) + (assign17000_e15455 * locals.var_vds_1_dn8)));
        locals.var_t0__blk808_dn9 = (assign17000_e15458 * ((2.0 * locals.var_pparam_b4soidvtp4_dn9) * locals.var_vds_1));
        locals.var_t0__blk808_dn10 = (assign17000_e15458 * ((2.0 * locals.var_pparam_b4soidvtp4_dn10) * locals.var_vds_1));
        locals.var_t0__blk808_dn11 = (assign17000_e15458 * ((2.0 * locals.var_pparam_b4soidvtp4_dn11) * locals.var_vds_1));
        locals.var_t0__blk808_dn12 = (assign17000_e15458 * ((2.0 * locals.var_pparam_b4soidvtp4_dn12) * locals.var_vds_1));
        locals.var_t0__blk808_rv = 0.0;

        let assign17010_e15462: f64 = (locals.var_t0__blk808 - 1.0);
        let assign17010_e15463: f64 = (locals.var_pparam_b4soidvtp2factor * assign17010_e15462);
        let assign17010_e15466: f64 = (locals.var_t0__blk808 + 1.0);
        let assign17010_e15467: f64 = (assign17010_e15463 / assign17010_e15466);
        locals.var_dits_sft2__blk1056 = assign17010_e15467;
        locals.var_dits_sft2__blk1056_dn3 = (((((locals.var_pparam_b4soidvtp2factor_dn3 * assign17010_e15462) + (locals.var_pparam_b4soidvtp2factor * locals.var_t0__blk808_dn3)) * assign17010_e15466) - (assign17010_e15463 * locals.var_t0__blk808_dn3)) / (assign17010_e15466 * assign17010_e15466));
        locals.var_dits_sft2__blk1056_dn4 = (((((locals.var_pparam_b4soidvtp2factor_dn4 * assign17010_e15462) + (locals.var_pparam_b4soidvtp2factor * locals.var_t0__blk808_dn4)) * assign17010_e15466) - (assign17010_e15463 * locals.var_t0__blk808_dn4)) / (assign17010_e15466 * assign17010_e15466));
        locals.var_dits_sft2__blk1056_dn5 = (((((locals.var_pparam_b4soidvtp2factor_dn5 * assign17010_e15462) + (locals.var_pparam_b4soidvtp2factor * locals.var_t0__blk808_dn5)) * assign17010_e15466) - (assign17010_e15463 * locals.var_t0__blk808_dn5)) / (assign17010_e15466 * assign17010_e15466));
        locals.var_dits_sft2__blk1056_dn6 = (((((locals.var_pparam_b4soidvtp2factor_dn6 * assign17010_e15462) + (locals.var_pparam_b4soidvtp2factor * locals.var_t0__blk808_dn6)) * assign17010_e15466) - (assign17010_e15463 * locals.var_t0__blk808_dn6)) / (assign17010_e15466 * assign17010_e15466));
        locals.var_dits_sft2__blk1056_dn7 = (((((locals.var_pparam_b4soidvtp2factor_dn7 * assign17010_e15462) + (locals.var_pparam_b4soidvtp2factor * locals.var_t0__blk808_dn7)) * assign17010_e15466) - (assign17010_e15463 * locals.var_t0__blk808_dn7)) / (assign17010_e15466 * assign17010_e15466));
        locals.var_dits_sft2__blk1056_dn8 = (((((locals.var_pparam_b4soidvtp2factor_dn8 * assign17010_e15462) + (locals.var_pparam_b4soidvtp2factor * locals.var_t0__blk808_dn8)) * assign17010_e15466) - (assign17010_e15463 * locals.var_t0__blk808_dn8)) / (assign17010_e15466 * assign17010_e15466));
        locals.var_dits_sft2__blk1056_dn9 = (((((locals.var_pparam_b4soidvtp2factor_dn9 * assign17010_e15462) + (locals.var_pparam_b4soidvtp2factor * locals.var_t0__blk808_dn9)) * assign17010_e15466) - (assign17010_e15463 * locals.var_t0__blk808_dn9)) / (assign17010_e15466 * assign17010_e15466));
        locals.var_dits_sft2__blk1056_dn10 = (((((locals.var_pparam_b4soidvtp2factor_dn10 * assign17010_e15462) + (locals.var_pparam_b4soidvtp2factor * locals.var_t0__blk808_dn10)) * assign17010_e15466) - (assign17010_e15463 * locals.var_t0__blk808_dn10)) / (assign17010_e15466 * assign17010_e15466));
        locals.var_dits_sft2__blk1056_dn11 = (((((locals.var_pparam_b4soidvtp2factor_dn11 * assign17010_e15462) + (locals.var_pparam_b4soidvtp2factor * locals.var_t0__blk808_dn11)) * assign17010_e15466) - (assign17010_e15463 * locals.var_t0__blk808_dn11)) / (assign17010_e15466 * assign17010_e15466));
        locals.var_dits_sft2__blk1056_dn12 = (((((locals.var_pparam_b4soidvtp2factor_dn12 * assign17010_e15462) + (locals.var_pparam_b4soidvtp2factor * locals.var_t0__blk808_dn12)) * assign17010_e15466) - (assign17010_e15463 * locals.var_t0__blk808_dn12)) / (assign17010_e15466 * assign17010_e15466));
        locals.var_dits_sft2__blk1056_rv = 0.0;

        let assign17020_e15470: f64 = (p.p37 * locals.var_here_b4soivth0);
        let assign17020_e15473: f64 = (locals.var_here_b4soik1ox * locals.var_sqrtphisext__blk928);
        let assign17020_e15476: f64 = (locals.var_here_b4soik1eff * locals.var_sqrtphi);
        let assign17020_e15477: f64 = (assign17020_e15473 - assign17020_e15476);
        let assign17020_e15479: f64 = (assign17020_e15477 * locals.var_lpe_vb__blk1054);
        let assign17020_e15480: f64 = (assign17020_e15470 + assign17020_e15479);
        let assign17020_e15483: f64 = (locals.var_here_b4soik2ox * locals.var_vbseff);
        let assign17020_e15484: f64 = (assign17020_e15480 - assign17020_e15483);
        let assign17020_e15486: f64 = (assign17020_e15484 - locals.var_delt_vth__blk832);
        let assign17020_e15488: f64 = (assign17020_e15486 - locals.var_deltvthw__blk869);
        let assign17020_e15492: f64 = (locals.var_pparam_b4soik3b * locals.var_vbseff);
        let assign17020_e15493: f64 = (locals.var_pparam_b4soik3 + assign17020_e15492);
        let assign17020_e15495: f64 = (assign17020_e15493 * locals.var_tmp2__blk835);
        let assign17020_e15496: f64 = (assign17020_e15488 + assign17020_e15495);
        let assign17020_e15498: f64 = (assign17020_e15496 + locals.var_deltvthtemp__blk868);
        let assign17020_e15500: f64 = (assign17020_e15498 - locals.var_dibl_sft__blk838);
        let assign17020_e15502: f64 = (assign17020_e15500 - locals.var_dits_sft__blk1055);
        let assign17020_e15504: f64 = (assign17020_e15502 - locals.var_dits_sft2__blk1056);
        locals.var_vth__blk794 = assign17020_e15504;
        locals.var_vth__blk794_dn3 = ((((((((((p.p37 * locals.var_here_b4soivth0_dn3) + (((((locals.var_here_b4soik1ox_dn3 * locals.var_sqrtphisext__blk928) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext__blk928_dn3)) - ((locals.var_here_b4soik1eff_dn3 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn3))) * locals.var_lpe_vb__blk1054) + (assign17020_e15477 * locals.var_lpe_vb__blk1054_dn3))) - ((locals.var_here_b4soik2ox_dn3 * locals.var_vbseff) + (locals.var_here_b4soik2ox * locals.var_vbseff_dn3))) - locals.var_delt_vth__blk832_dn3) - locals.var_deltvthw__blk869_dn3) + (((locals.var_pparam_b4soik3_dn3 + ((locals.var_pparam_b4soik3b_dn3 * locals.var_vbseff) + (locals.var_pparam_b4soik3b * locals.var_vbseff_dn3))) * locals.var_tmp2__blk835) + (assign17020_e15493 * locals.var_tmp2__blk835_dn3))) + locals.var_deltvthtemp__blk868_dn3) - locals.var_dibl_sft__blk838_dn3) - locals.var_dits_sft__blk1055_dn3) - locals.var_dits_sft2__blk1056_dn3);
        locals.var_vth__blk794_dn4 = ((((((((((p.p37 * locals.var_here_b4soivth0_dn4) + (((((locals.var_here_b4soik1ox_dn4 * locals.var_sqrtphisext__blk928) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext__blk928_dn4)) - ((locals.var_here_b4soik1eff_dn4 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn4))) * locals.var_lpe_vb__blk1054) + (assign17020_e15477 * locals.var_lpe_vb__blk1054_dn4))) - ((locals.var_here_b4soik2ox_dn4 * locals.var_vbseff) + (locals.var_here_b4soik2ox * locals.var_vbseff_dn4))) - locals.var_delt_vth__blk832_dn4) - locals.var_deltvthw__blk869_dn4) + (((locals.var_pparam_b4soik3_dn4 + ((locals.var_pparam_b4soik3b_dn4 * locals.var_vbseff) + (locals.var_pparam_b4soik3b * locals.var_vbseff_dn4))) * locals.var_tmp2__blk835) + (assign17020_e15493 * locals.var_tmp2__blk835_dn4))) + locals.var_deltvthtemp__blk868_dn4) - locals.var_dibl_sft__blk838_dn4) - locals.var_dits_sft__blk1055_dn4) - locals.var_dits_sft2__blk1056_dn4);
        locals.var_vth__blk794_dn5 = ((((((((((p.p37 * locals.var_here_b4soivth0_dn5) + (((((locals.var_here_b4soik1ox_dn5 * locals.var_sqrtphisext__blk928) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext__blk928_dn5)) - ((locals.var_here_b4soik1eff_dn5 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn5))) * locals.var_lpe_vb__blk1054) + (assign17020_e15477 * locals.var_lpe_vb__blk1054_dn5))) - ((locals.var_here_b4soik2ox_dn5 * locals.var_vbseff) + (locals.var_here_b4soik2ox * locals.var_vbseff_dn5))) - locals.var_delt_vth__blk832_dn5) - locals.var_deltvthw__blk869_dn5) + (((locals.var_pparam_b4soik3_dn5 + ((locals.var_pparam_b4soik3b_dn5 * locals.var_vbseff) + (locals.var_pparam_b4soik3b * locals.var_vbseff_dn5))) * locals.var_tmp2__blk835) + (assign17020_e15493 * locals.var_tmp2__blk835_dn5))) + locals.var_deltvthtemp__blk868_dn5) - locals.var_dibl_sft__blk838_dn5) - locals.var_dits_sft__blk1055_dn5) - locals.var_dits_sft2__blk1056_dn5);
        locals.var_vth__blk794_dn6 = ((((((((((p.p37 * locals.var_here_b4soivth0_dn6) + (((((locals.var_here_b4soik1ox_dn6 * locals.var_sqrtphisext__blk928) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext__blk928_dn6)) - ((locals.var_here_b4soik1eff_dn6 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn6))) * locals.var_lpe_vb__blk1054) + (assign17020_e15477 * locals.var_lpe_vb__blk1054_dn6))) - ((locals.var_here_b4soik2ox_dn6 * locals.var_vbseff) + (locals.var_here_b4soik2ox * locals.var_vbseff_dn6))) - locals.var_delt_vth__blk832_dn6) - locals.var_deltvthw__blk869_dn6) + (((locals.var_pparam_b4soik3_dn6 + ((locals.var_pparam_b4soik3b_dn6 * locals.var_vbseff) + (locals.var_pparam_b4soik3b * locals.var_vbseff_dn6))) * locals.var_tmp2__blk835) + (assign17020_e15493 * locals.var_tmp2__blk835_dn6))) + locals.var_deltvthtemp__blk868_dn6) - locals.var_dibl_sft__blk838_dn6) - locals.var_dits_sft__blk1055_dn6) - locals.var_dits_sft2__blk1056_dn6);
        locals.var_vth__blk794_dn7 = ((((((((((p.p37 * locals.var_here_b4soivth0_dn7) + (((((locals.var_here_b4soik1ox_dn7 * locals.var_sqrtphisext__blk928) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext__blk928_dn7)) - ((locals.var_here_b4soik1eff_dn7 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn7))) * locals.var_lpe_vb__blk1054) + (assign17020_e15477 * locals.var_lpe_vb__blk1054_dn7))) - ((locals.var_here_b4soik2ox_dn7 * locals.var_vbseff) + (locals.var_here_b4soik2ox * locals.var_vbseff_dn7))) - locals.var_delt_vth__blk832_dn7) - locals.var_deltvthw__blk869_dn7) + (((locals.var_pparam_b4soik3_dn7 + ((locals.var_pparam_b4soik3b_dn7 * locals.var_vbseff) + (locals.var_pparam_b4soik3b * locals.var_vbseff_dn7))) * locals.var_tmp2__blk835) + (assign17020_e15493 * locals.var_tmp2__blk835_dn7))) + locals.var_deltvthtemp__blk868_dn7) - locals.var_dibl_sft__blk838_dn7) - locals.var_dits_sft__blk1055_dn7) - locals.var_dits_sft2__blk1056_dn7);
        locals.var_vth__blk794_dn8 = ((((((((((p.p37 * locals.var_here_b4soivth0_dn8) + (((((locals.var_here_b4soik1ox_dn8 * locals.var_sqrtphisext__blk928) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext__blk928_dn8)) - ((locals.var_here_b4soik1eff_dn8 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn8))) * locals.var_lpe_vb__blk1054) + (assign17020_e15477 * locals.var_lpe_vb__blk1054_dn8))) - ((locals.var_here_b4soik2ox_dn8 * locals.var_vbseff) + (locals.var_here_b4soik2ox * locals.var_vbseff_dn8))) - locals.var_delt_vth__blk832_dn8) - locals.var_deltvthw__blk869_dn8) + (((locals.var_pparam_b4soik3_dn8 + ((locals.var_pparam_b4soik3b_dn8 * locals.var_vbseff) + (locals.var_pparam_b4soik3b * locals.var_vbseff_dn8))) * locals.var_tmp2__blk835) + (assign17020_e15493 * locals.var_tmp2__blk835_dn8))) + locals.var_deltvthtemp__blk868_dn8) - locals.var_dibl_sft__blk838_dn8) - locals.var_dits_sft__blk1055_dn8) - locals.var_dits_sft2__blk1056_dn8);
        locals.var_vth__blk794_dn9 = ((((((((((p.p37 * locals.var_here_b4soivth0_dn9) + (((((locals.var_here_b4soik1ox_dn9 * locals.var_sqrtphisext__blk928) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext__blk928_dn9)) - ((locals.var_here_b4soik1eff_dn9 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn9))) * locals.var_lpe_vb__blk1054) + (assign17020_e15477 * locals.var_lpe_vb__blk1054_dn9))) - ((locals.var_here_b4soik2ox_dn9 * locals.var_vbseff) + (locals.var_here_b4soik2ox * locals.var_vbseff_dn9))) - locals.var_delt_vth__blk832_dn9) - locals.var_deltvthw__blk869_dn9) + (((locals.var_pparam_b4soik3_dn9 + ((locals.var_pparam_b4soik3b_dn9 * locals.var_vbseff) + (locals.var_pparam_b4soik3b * locals.var_vbseff_dn9))) * locals.var_tmp2__blk835) + (assign17020_e15493 * locals.var_tmp2__blk835_dn9))) + locals.var_deltvthtemp__blk868_dn9) - locals.var_dibl_sft__blk838_dn9) - locals.var_dits_sft__blk1055_dn9) - locals.var_dits_sft2__blk1056_dn9);
        locals.var_vth__blk794_dn10 = ((((((((((p.p37 * locals.var_here_b4soivth0_dn10) + (((((locals.var_here_b4soik1ox_dn10 * locals.var_sqrtphisext__blk928) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext__blk928_dn10)) - ((locals.var_here_b4soik1eff_dn10 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn10))) * locals.var_lpe_vb__blk1054) + (assign17020_e15477 * locals.var_lpe_vb__blk1054_dn10))) - ((locals.var_here_b4soik2ox_dn10 * locals.var_vbseff) + (locals.var_here_b4soik2ox * locals.var_vbseff_dn10))) - locals.var_delt_vth__blk832_dn10) - locals.var_deltvthw__blk869_dn10) + (((locals.var_pparam_b4soik3_dn10 + ((locals.var_pparam_b4soik3b_dn10 * locals.var_vbseff) + (locals.var_pparam_b4soik3b * locals.var_vbseff_dn10))) * locals.var_tmp2__blk835) + (assign17020_e15493 * locals.var_tmp2__blk835_dn10))) + locals.var_deltvthtemp__blk868_dn10) - locals.var_dibl_sft__blk838_dn10) - locals.var_dits_sft__blk1055_dn10) - locals.var_dits_sft2__blk1056_dn10);
        locals.var_vth__blk794_dn11 = ((((((((((p.p37 * locals.var_here_b4soivth0_dn11) + (((((locals.var_here_b4soik1ox_dn11 * locals.var_sqrtphisext__blk928) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext__blk928_dn11)) - ((locals.var_here_b4soik1eff_dn11 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn11))) * locals.var_lpe_vb__blk1054) + (assign17020_e15477 * locals.var_lpe_vb__blk1054_dn11))) - ((locals.var_here_b4soik2ox_dn11 * locals.var_vbseff) + (locals.var_here_b4soik2ox * locals.var_vbseff_dn11))) - locals.var_delt_vth__blk832_dn11) - locals.var_deltvthw__blk869_dn11) + (((locals.var_pparam_b4soik3_dn11 + ((locals.var_pparam_b4soik3b_dn11 * locals.var_vbseff) + (locals.var_pparam_b4soik3b * locals.var_vbseff_dn11))) * locals.var_tmp2__blk835) + (assign17020_e15493 * locals.var_tmp2__blk835_dn11))) + locals.var_deltvthtemp__blk868_dn11) - locals.var_dibl_sft__blk838_dn11) - locals.var_dits_sft__blk1055_dn11) - locals.var_dits_sft2__blk1056_dn11);
        locals.var_vth__blk794_dn12 = ((((((((((p.p37 * locals.var_here_b4soivth0_dn12) + (((((locals.var_here_b4soik1ox_dn12 * locals.var_sqrtphisext__blk928) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext__blk928_dn12)) - ((locals.var_here_b4soik1eff_dn12 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn12))) * locals.var_lpe_vb__blk1054) + (assign17020_e15477 * locals.var_lpe_vb__blk1054_dn12))) - ((locals.var_here_b4soik2ox_dn12 * locals.var_vbseff) + (locals.var_here_b4soik2ox * locals.var_vbseff_dn12))) - locals.var_delt_vth__blk832_dn12) - locals.var_deltvthw__blk869_dn12) + (((locals.var_pparam_b4soik3_dn12 + ((locals.var_pparam_b4soik3b_dn12 * locals.var_vbseff) + (locals.var_pparam_b4soik3b * locals.var_vbseff_dn12))) * locals.var_tmp2__blk835) + (assign17020_e15493 * locals.var_tmp2__blk835_dn12))) + locals.var_deltvthtemp__blk868_dn12) - locals.var_dibl_sft__blk838_dn12) - locals.var_dits_sft__blk1055_dn12) - locals.var_dits_sft2__blk1056_dn12);
        locals.var_vth__blk794_rv = 0.0;

        let assign17040_e15508: f64 = (locals.var_phi - locals.var_vbseff_cv);
        locals.var_phis_cv = assign17040_e15508;
        locals.var_phis_cv_dn3 = (locals.var_phi_dn3 - locals.var_vbseff_cv_dn3);
        locals.var_phis_cv_dn4 = (locals.var_phi_dn4 - locals.var_vbseff_cv_dn4);
        locals.var_phis_cv_dn5 = (locals.var_phi_dn5 - locals.var_vbseff_cv_dn5);
        locals.var_phis_cv_dn6 = (locals.var_phi_dn6 - locals.var_vbseff_cv_dn6);
        locals.var_phis_cv_dn7 = (locals.var_phi_dn7 - locals.var_vbseff_cv_dn7);
        locals.var_phis_cv_dn8 = (locals.var_phi_dn8 - locals.var_vbseff_cv_dn8);
        locals.var_phis_cv_dn9 = (locals.var_phi_dn9 - locals.var_vbseff_cv_dn9);
        locals.var_phis_cv_dn10 = (locals.var_phi_dn10 - locals.var_vbseff_cv_dn10);
        locals.var_phis_cv_dn11 = (locals.var_phi_dn11 - locals.var_vbseff_cv_dn11);
        locals.var_phis_cv_dn12 = (locals.var_phi_dn12 - locals.var_vbseff_cv_dn12);
        locals.var_phis_cv_rv = 0.0;

        let assign17050_e15510: f64 = (locals.var_phis_cv).sqrt();
        locals.var_sqrtphis_cv = assign17050_e15510;
        locals.var_sqrtphis_cv_dn3 = (locals.var_phis_cv_dn3 / (2.0 * assign17050_e15510));
        locals.var_sqrtphis_cv_dn4 = (locals.var_phis_cv_dn4 / (2.0 * assign17050_e15510));
        locals.var_sqrtphis_cv_dn5 = (locals.var_phis_cv_dn5 / (2.0 * assign17050_e15510));
        locals.var_sqrtphis_cv_dn6 = (locals.var_phis_cv_dn6 / (2.0 * assign17050_e15510));
        locals.var_sqrtphis_cv_dn7 = (locals.var_phis_cv_dn7 / (2.0 * assign17050_e15510));
        locals.var_sqrtphis_cv_dn8 = (locals.var_phis_cv_dn8 / (2.0 * assign17050_e15510));
        locals.var_sqrtphis_cv_dn9 = (locals.var_phis_cv_dn9 / (2.0 * assign17050_e15510));
        locals.var_sqrtphis_cv_dn10 = (locals.var_phis_cv_dn10 / (2.0 * assign17050_e15510));
        locals.var_sqrtphis_cv_dn11 = (locals.var_phis_cv_dn11 / (2.0 * assign17050_e15510));
        locals.var_sqrtphis_cv_dn12 = (locals.var_phis_cv_dn12 / (2.0 * assign17050_e15510));
        locals.var_sqrtphis_cv_rv = 0.0;

        let assign17060_e15513: f64 = (locals.var_xdep0 * locals.var_sqrtphis_cv);
        let assign17060_e15515: f64 = (assign17060_e15513 / locals.var_sqrtphi);
        locals.var_xdep_cv = assign17060_e15515;
        locals.var_xdep_cv_dn3 = (((((locals.var_xdep0_dn3 * locals.var_sqrtphis_cv) + (locals.var_xdep0 * locals.var_sqrtphis_cv_dn3)) * locals.var_sqrtphi) - (assign17060_e15513 * locals.var_sqrtphi_dn3)) / (locals.var_sqrtphi * locals.var_sqrtphi));
        locals.var_xdep_cv_dn4 = (((((locals.var_xdep0_dn4 * locals.var_sqrtphis_cv) + (locals.var_xdep0 * locals.var_sqrtphis_cv_dn4)) * locals.var_sqrtphi) - (assign17060_e15513 * locals.var_sqrtphi_dn4)) / (locals.var_sqrtphi * locals.var_sqrtphi));
        locals.var_xdep_cv_dn5 = (((((locals.var_xdep0_dn5 * locals.var_sqrtphis_cv) + (locals.var_xdep0 * locals.var_sqrtphis_cv_dn5)) * locals.var_sqrtphi) - (assign17060_e15513 * locals.var_sqrtphi_dn5)) / (locals.var_sqrtphi * locals.var_sqrtphi));
        locals.var_xdep_cv_dn6 = (((((locals.var_xdep0_dn6 * locals.var_sqrtphis_cv) + (locals.var_xdep0 * locals.var_sqrtphis_cv_dn6)) * locals.var_sqrtphi) - (assign17060_e15513 * locals.var_sqrtphi_dn6)) / (locals.var_sqrtphi * locals.var_sqrtphi));
        locals.var_xdep_cv_dn7 = (((((locals.var_xdep0_dn7 * locals.var_sqrtphis_cv) + (locals.var_xdep0 * locals.var_sqrtphis_cv_dn7)) * locals.var_sqrtphi) - (assign17060_e15513 * locals.var_sqrtphi_dn7)) / (locals.var_sqrtphi * locals.var_sqrtphi));
        locals.var_xdep_cv_dn8 = (((((locals.var_xdep0_dn8 * locals.var_sqrtphis_cv) + (locals.var_xdep0 * locals.var_sqrtphis_cv_dn8)) * locals.var_sqrtphi) - (assign17060_e15513 * locals.var_sqrtphi_dn8)) / (locals.var_sqrtphi * locals.var_sqrtphi));
        locals.var_xdep_cv_dn9 = (((((locals.var_xdep0_dn9 * locals.var_sqrtphis_cv) + (locals.var_xdep0 * locals.var_sqrtphis_cv_dn9)) * locals.var_sqrtphi) - (assign17060_e15513 * locals.var_sqrtphi_dn9)) / (locals.var_sqrtphi * locals.var_sqrtphi));
        locals.var_xdep_cv_dn10 = (((((locals.var_xdep0_dn10 * locals.var_sqrtphis_cv) + (locals.var_xdep0 * locals.var_sqrtphis_cv_dn10)) * locals.var_sqrtphi) - (assign17060_e15513 * locals.var_sqrtphi_dn10)) / (locals.var_sqrtphi * locals.var_sqrtphi));
        locals.var_xdep_cv_dn11 = (((((locals.var_xdep0_dn11 * locals.var_sqrtphis_cv) + (locals.var_xdep0 * locals.var_sqrtphis_cv_dn11)) * locals.var_sqrtphi) - (assign17060_e15513 * locals.var_sqrtphi_dn11)) / (locals.var_sqrtphi * locals.var_sqrtphi));
        locals.var_xdep_cv_dn12 = (((((locals.var_xdep0_dn12 * locals.var_sqrtphis_cv) + (locals.var_xdep0 * locals.var_sqrtphis_cv_dn12)) * locals.var_sqrtphi) - (assign17060_e15513 * locals.var_sqrtphi_dn12)) / (locals.var_sqrtphi * locals.var_sqrtphi));
        locals.var_xdep_cv_rv = 0.0;

        let assign17080_e15528: f64 = (locals.var_xdep_cv).sqrt();
        locals.var_t3__blk811 = assign17080_e15528;
        locals.var_t3__blk811_dn3 = (locals.var_xdep_cv_dn3 / (2.0 * assign17080_e15528));
        locals.var_t3__blk811_dn4 = (locals.var_xdep_cv_dn4 / (2.0 * assign17080_e15528));
        locals.var_t3__blk811_dn5 = (locals.var_xdep_cv_dn5 / (2.0 * assign17080_e15528));
        locals.var_t3__blk811_dn6 = (locals.var_xdep_cv_dn6 / (2.0 * assign17080_e15528));
        locals.var_t3__blk811_dn7 = (locals.var_xdep_cv_dn7 / (2.0 * assign17080_e15528));
        locals.var_t3__blk811_dn8 = (locals.var_xdep_cv_dn8 / (2.0 * assign17080_e15528));
        locals.var_t3__blk811_dn9 = (locals.var_xdep_cv_dn9 / (2.0 * assign17080_e15528));
        locals.var_t3__blk811_dn10 = (locals.var_xdep_cv_dn10 / (2.0 * assign17080_e15528));
        locals.var_t3__blk811_dn11 = (locals.var_xdep_cv_dn11 / (2.0 * assign17080_e15528));
        locals.var_t3__blk811_dn12 = (locals.var_xdep_cv_dn12 / (2.0 * assign17080_e15528));
        locals.var_t3__blk811_rv = 0.0;

        let assign17090_e15531: f64 = (locals.var_pparam_b4soidvt2 * locals.var_vbseff_cv);
        locals.var_t0__blk808 = assign17090_e15531;
        locals.var_t0__blk808_dn3 = ((locals.var_pparam_b4soidvt2_dn3 * locals.var_vbseff_cv) + (locals.var_pparam_b4soidvt2 * locals.var_vbseff_cv_dn3));
        locals.var_t0__blk808_dn4 = ((locals.var_pparam_b4soidvt2_dn4 * locals.var_vbseff_cv) + (locals.var_pparam_b4soidvt2 * locals.var_vbseff_cv_dn4));
        locals.var_t0__blk808_dn5 = ((locals.var_pparam_b4soidvt2_dn5 * locals.var_vbseff_cv) + (locals.var_pparam_b4soidvt2 * locals.var_vbseff_cv_dn5));
        locals.var_t0__blk808_dn6 = ((locals.var_pparam_b4soidvt2_dn6 * locals.var_vbseff_cv) + (locals.var_pparam_b4soidvt2 * locals.var_vbseff_cv_dn6));
        locals.var_t0__blk808_dn7 = ((locals.var_pparam_b4soidvt2_dn7 * locals.var_vbseff_cv) + (locals.var_pparam_b4soidvt2 * locals.var_vbseff_cv_dn7));
        locals.var_t0__blk808_dn8 = ((locals.var_pparam_b4soidvt2_dn8 * locals.var_vbseff_cv) + (locals.var_pparam_b4soidvt2 * locals.var_vbseff_cv_dn8));
        locals.var_t0__blk808_dn9 = ((locals.var_pparam_b4soidvt2_dn9 * locals.var_vbseff_cv) + (locals.var_pparam_b4soidvt2 * locals.var_vbseff_cv_dn9));
        locals.var_t0__blk808_dn10 = ((locals.var_pparam_b4soidvt2_dn10 * locals.var_vbseff_cv) + (locals.var_pparam_b4soidvt2 * locals.var_vbseff_cv_dn10));
        locals.var_t0__blk808_dn11 = ((locals.var_pparam_b4soidvt2_dn11 * locals.var_vbseff_cv) + (locals.var_pparam_b4soidvt2 * locals.var_vbseff_cv_dn11));
        locals.var_t0__blk808_dn12 = ((locals.var_pparam_b4soidvt2_dn12 * locals.var_vbseff_cv) + (locals.var_pparam_b4soidvt2 * locals.var_vbseff_cv_dn12));
        locals.var_t0__blk808_rv = 0.0;

        let assign17100_e15534: f64 = (-0.5);
        let assign17100_e15535: f64 = if locals.var_t0__blk808 >= assign17100_e15534 { 1.0 } else { 0.0 };
        locals.var_guard1198 = assign17100_e15535;
        locals.var_guard1198_rv = 0.0;

        let (assign17110_e15541, assign17110_e15541_d_n3, assign17110_e15541_d_n4, assign17110_e15541_d_n5, assign17110_e15541_d_n6, assign17110_e15541_d_n7, assign17110_e15541_d_n8, assign17110_e15541_d_n9, assign17110_e15541_d_n10, assign17110_e15541_d_n11, assign17110_e15541_d_n12,) = {
    if (locals.var_guard1198 != 0.0) {
        let assign17110_e15539: f64 = (1.0 + locals.var_t0__blk808);
        (assign17110_e15539, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign17110_e15541;
        locals.var_t1__blk809_dn3 = assign17110_e15541_d_n3;
        locals.var_t1__blk809_dn4 = assign17110_e15541_d_n4;
        locals.var_t1__blk809_dn5 = assign17110_e15541_d_n5;
        locals.var_t1__blk809_dn6 = assign17110_e15541_d_n6;
        locals.var_t1__blk809_dn7 = assign17110_e15541_d_n7;
        locals.var_t1__blk809_dn8 = assign17110_e15541_d_n8;
        locals.var_t1__blk809_dn9 = assign17110_e15541_d_n9;
        locals.var_t1__blk809_dn10 = assign17110_e15541_d_n10;
        locals.var_t1__blk809_dn11 = assign17110_e15541_d_n11;
        locals.var_t1__blk809_dn12 = assign17110_e15541_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign17120_e15552, assign17120_e15552_d_n3, assign17120_e15552_d_n4, assign17120_e15552_d_n5, assign17120_e15552_d_n6, assign17120_e15552_d_n7, assign17120_e15552_d_n8, assign17120_e15552_d_n9, assign17120_e15552_d_n10, assign17120_e15552_d_n11, assign17120_e15552_d_n12,) = {
    if (locals.var_guard1198 == 0.0) {
        let assign17120_e15548: f64 = (8.0 * locals.var_t0__blk808);
        let assign17120_e15549: f64 = (3.0 + assign17120_e15548);
        let assign17120_e15550: f64 = (1.0 / assign17120_e15549);
        (assign17120_e15550, (-((8.0 * locals.var_t0__blk808_dn3) / (assign17120_e15549 * assign17120_e15549))), (-((8.0 * locals.var_t0__blk808_dn4) / (assign17120_e15549 * assign17120_e15549))), (-((8.0 * locals.var_t0__blk808_dn5) / (assign17120_e15549 * assign17120_e15549))), (-((8.0 * locals.var_t0__blk808_dn6) / (assign17120_e15549 * assign17120_e15549))), (-((8.0 * locals.var_t0__blk808_dn7) / (assign17120_e15549 * assign17120_e15549))), (-((8.0 * locals.var_t0__blk808_dn8) / (assign17120_e15549 * assign17120_e15549))), (-((8.0 * locals.var_t0__blk808_dn9) / (assign17120_e15549 * assign17120_e15549))), (-((8.0 * locals.var_t0__blk808_dn10) / (assign17120_e15549 * assign17120_e15549))), (-((8.0 * locals.var_t0__blk808_dn11) / (assign17120_e15549 * assign17120_e15549))), (-((8.0 * locals.var_t0__blk808_dn12) / (assign17120_e15549 * assign17120_e15549))),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign17120_e15552;
        locals.var_t4__blk812_dn3 = assign17120_e15552_d_n3;
        locals.var_t4__blk812_dn4 = assign17120_e15552_d_n4;
        locals.var_t4__blk812_dn5 = assign17120_e15552_d_n5;
        locals.var_t4__blk812_dn6 = assign17120_e15552_d_n6;
        locals.var_t4__blk812_dn7 = assign17120_e15552_d_n7;
        locals.var_t4__blk812_dn8 = assign17120_e15552_d_n8;
        locals.var_t4__blk812_dn9 = assign17120_e15552_d_n9;
        locals.var_t4__blk812_dn10 = assign17120_e15552_d_n10;
        locals.var_t4__blk812_dn11 = assign17120_e15552_d_n11;
        locals.var_t4__blk812_dn12 = assign17120_e15552_d_n12;
        locals.var_t4__blk812_rv = 0.0;

        let (assign17130_e15563, assign17130_e15563_d_n3, assign17130_e15563_d_n4, assign17130_e15563_d_n5, assign17130_e15563_d_n6, assign17130_e15563_d_n7, assign17130_e15563_d_n8, assign17130_e15563_d_n9, assign17130_e15563_d_n10, assign17130_e15563_d_n11, assign17130_e15563_d_n12,) = {
    if (locals.var_guard1198 == 0.0) {
        let assign17130_e15558: f64 = (3.0 * locals.var_t0__blk808);
        let assign17130_e15559: f64 = (1.0 + assign17130_e15558);
        let assign17130_e15561: f64 = (assign17130_e15559 * locals.var_t4__blk812);
        (assign17130_e15561, (((3.0 * locals.var_t0__blk808_dn3) * locals.var_t4__blk812) + (assign17130_e15559 * locals.var_t4__blk812_dn3)), (((3.0 * locals.var_t0__blk808_dn4) * locals.var_t4__blk812) + (assign17130_e15559 * locals.var_t4__blk812_dn4)), (((3.0 * locals.var_t0__blk808_dn5) * locals.var_t4__blk812) + (assign17130_e15559 * locals.var_t4__blk812_dn5)), (((3.0 * locals.var_t0__blk808_dn6) * locals.var_t4__blk812) + (assign17130_e15559 * locals.var_t4__blk812_dn6)), (((3.0 * locals.var_t0__blk808_dn7) * locals.var_t4__blk812) + (assign17130_e15559 * locals.var_t4__blk812_dn7)), (((3.0 * locals.var_t0__blk808_dn8) * locals.var_t4__blk812) + (assign17130_e15559 * locals.var_t4__blk812_dn8)), (((3.0 * locals.var_t0__blk808_dn9) * locals.var_t4__blk812) + (assign17130_e15559 * locals.var_t4__blk812_dn9)), (((3.0 * locals.var_t0__blk808_dn10) * locals.var_t4__blk812) + (assign17130_e15559 * locals.var_t4__blk812_dn10)), (((3.0 * locals.var_t0__blk808_dn11) * locals.var_t4__blk812) + (assign17130_e15559 * locals.var_t4__blk812_dn11)), (((3.0 * locals.var_t0__blk808_dn12) * locals.var_t4__blk812) + (assign17130_e15559 * locals.var_t4__blk812_dn12)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign17130_e15563;
        locals.var_t1__blk809_dn3 = assign17130_e15563_d_n3;
        locals.var_t1__blk809_dn4 = assign17130_e15563_d_n4;
        locals.var_t1__blk809_dn5 = assign17130_e15563_d_n5;
        locals.var_t1__blk809_dn6 = assign17130_e15563_d_n6;
        locals.var_t1__blk809_dn7 = assign17130_e15563_d_n7;
        locals.var_t1__blk809_dn8 = assign17130_e15563_d_n8;
        locals.var_t1__blk809_dn9 = assign17130_e15563_d_n9;
        locals.var_t1__blk809_dn10 = assign17130_e15563_d_n10;
        locals.var_t1__blk809_dn11 = assign17130_e15563_d_n11;
        locals.var_t1__blk809_dn12 = assign17130_e15563_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let assign17140_e15566: f64 = (locals.var_b4soifactor1 * locals.var_t3__blk811);
        let assign17140_e15568: f64 = (assign17140_e15566 * locals.var_t1__blk809);
        locals.var_lt1_cv = assign17140_e15568;
        locals.var_lt1_cv_dn3 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn3) * locals.var_t1__blk809) + (assign17140_e15566 * locals.var_t1__blk809_dn3));
        locals.var_lt1_cv_dn4 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn4) * locals.var_t1__blk809) + (assign17140_e15566 * locals.var_t1__blk809_dn4));
        locals.var_lt1_cv_dn5 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn5) * locals.var_t1__blk809) + (assign17140_e15566 * locals.var_t1__blk809_dn5));
        locals.var_lt1_cv_dn6 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn6) * locals.var_t1__blk809) + (assign17140_e15566 * locals.var_t1__blk809_dn6));
        locals.var_lt1_cv_dn7 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn7) * locals.var_t1__blk809) + (assign17140_e15566 * locals.var_t1__blk809_dn7));
        locals.var_lt1_cv_dn8 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn8) * locals.var_t1__blk809) + (assign17140_e15566 * locals.var_t1__blk809_dn8));
        locals.var_lt1_cv_dn9 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn9) * locals.var_t1__blk809) + (assign17140_e15566 * locals.var_t1__blk809_dn9));
        locals.var_lt1_cv_dn10 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn10) * locals.var_t1__blk809) + (assign17140_e15566 * locals.var_t1__blk809_dn10));
        locals.var_lt1_cv_dn11 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn11) * locals.var_t1__blk809) + (assign17140_e15566 * locals.var_t1__blk809_dn11));
        locals.var_lt1_cv_dn12 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn12) * locals.var_t1__blk809) + (assign17140_e15566 * locals.var_t1__blk809_dn12));
        locals.var_lt1_cv_rv = 0.0;

        let assign17150_e15571: f64 = (locals.var_pparam_b4soidvt2w * locals.var_vbseff_cv);
        locals.var_t0__blk808 = assign17150_e15571;
        locals.var_t0__blk808_dn3 = ((locals.var_pparam_b4soidvt2w_dn3 * locals.var_vbseff_cv) + (locals.var_pparam_b4soidvt2w * locals.var_vbseff_cv_dn3));
        locals.var_t0__blk808_dn4 = ((locals.var_pparam_b4soidvt2w_dn4 * locals.var_vbseff_cv) + (locals.var_pparam_b4soidvt2w * locals.var_vbseff_cv_dn4));
        locals.var_t0__blk808_dn5 = ((locals.var_pparam_b4soidvt2w_dn5 * locals.var_vbseff_cv) + (locals.var_pparam_b4soidvt2w * locals.var_vbseff_cv_dn5));
        locals.var_t0__blk808_dn6 = ((locals.var_pparam_b4soidvt2w_dn6 * locals.var_vbseff_cv) + (locals.var_pparam_b4soidvt2w * locals.var_vbseff_cv_dn6));
        locals.var_t0__blk808_dn7 = ((locals.var_pparam_b4soidvt2w_dn7 * locals.var_vbseff_cv) + (locals.var_pparam_b4soidvt2w * locals.var_vbseff_cv_dn7));
        locals.var_t0__blk808_dn8 = ((locals.var_pparam_b4soidvt2w_dn8 * locals.var_vbseff_cv) + (locals.var_pparam_b4soidvt2w * locals.var_vbseff_cv_dn8));
        locals.var_t0__blk808_dn9 = ((locals.var_pparam_b4soidvt2w_dn9 * locals.var_vbseff_cv) + (locals.var_pparam_b4soidvt2w * locals.var_vbseff_cv_dn9));
        locals.var_t0__blk808_dn10 = ((locals.var_pparam_b4soidvt2w_dn10 * locals.var_vbseff_cv) + (locals.var_pparam_b4soidvt2w * locals.var_vbseff_cv_dn10));
        locals.var_t0__blk808_dn11 = ((locals.var_pparam_b4soidvt2w_dn11 * locals.var_vbseff_cv) + (locals.var_pparam_b4soidvt2w * locals.var_vbseff_cv_dn11));
        locals.var_t0__blk808_dn12 = ((locals.var_pparam_b4soidvt2w_dn12 * locals.var_vbseff_cv) + (locals.var_pparam_b4soidvt2w * locals.var_vbseff_cv_dn12));
        locals.var_t0__blk808_rv = 0.0;

        let assign17160_e15574: f64 = (-0.5);
        let assign17160_e15575: f64 = if locals.var_t0__blk808 >= assign17160_e15574 { 1.0 } else { 0.0 };
        locals.var_guard1199 = assign17160_e15575;
        locals.var_guard1199_rv = 0.0;

        let (assign17170_e15581, assign17170_e15581_d_n3, assign17170_e15581_d_n4, assign17170_e15581_d_n5, assign17170_e15581_d_n6, assign17170_e15581_d_n7, assign17170_e15581_d_n8, assign17170_e15581_d_n9, assign17170_e15581_d_n10, assign17170_e15581_d_n11, assign17170_e15581_d_n12,) = {
    if (locals.var_guard1199 != 0.0) {
        let assign17170_e15579: f64 = (1.0 + locals.var_t0__blk808);
        (assign17170_e15579, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign17170_e15581;
        locals.var_t1__blk809_dn3 = assign17170_e15581_d_n3;
        locals.var_t1__blk809_dn4 = assign17170_e15581_d_n4;
        locals.var_t1__blk809_dn5 = assign17170_e15581_d_n5;
        locals.var_t1__blk809_dn6 = assign17170_e15581_d_n6;
        locals.var_t1__blk809_dn7 = assign17170_e15581_d_n7;
        locals.var_t1__blk809_dn8 = assign17170_e15581_d_n8;
        locals.var_t1__blk809_dn9 = assign17170_e15581_d_n9;
        locals.var_t1__blk809_dn10 = assign17170_e15581_d_n10;
        locals.var_t1__blk809_dn11 = assign17170_e15581_d_n11;
        locals.var_t1__blk809_dn12 = assign17170_e15581_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign17180_e15592, assign17180_e15592_d_n3, assign17180_e15592_d_n4, assign17180_e15592_d_n5, assign17180_e15592_d_n6, assign17180_e15592_d_n7, assign17180_e15592_d_n8, assign17180_e15592_d_n9, assign17180_e15592_d_n10, assign17180_e15592_d_n11, assign17180_e15592_d_n12,) = {
    if (locals.var_guard1199 == 0.0) {
        let assign17180_e15588: f64 = (8.0 * locals.var_t0__blk808);
        let assign17180_e15589: f64 = (3.0 + assign17180_e15588);
        let assign17180_e15590: f64 = (1.0 / assign17180_e15589);
        (assign17180_e15590, (-((8.0 * locals.var_t0__blk808_dn3) / (assign17180_e15589 * assign17180_e15589))), (-((8.0 * locals.var_t0__blk808_dn4) / (assign17180_e15589 * assign17180_e15589))), (-((8.0 * locals.var_t0__blk808_dn5) / (assign17180_e15589 * assign17180_e15589))), (-((8.0 * locals.var_t0__blk808_dn6) / (assign17180_e15589 * assign17180_e15589))), (-((8.0 * locals.var_t0__blk808_dn7) / (assign17180_e15589 * assign17180_e15589))), (-((8.0 * locals.var_t0__blk808_dn8) / (assign17180_e15589 * assign17180_e15589))), (-((8.0 * locals.var_t0__blk808_dn9) / (assign17180_e15589 * assign17180_e15589))), (-((8.0 * locals.var_t0__blk808_dn10) / (assign17180_e15589 * assign17180_e15589))), (-((8.0 * locals.var_t0__blk808_dn11) / (assign17180_e15589 * assign17180_e15589))), (-((8.0 * locals.var_t0__blk808_dn12) / (assign17180_e15589 * assign17180_e15589))),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign17180_e15592;
        locals.var_t4__blk812_dn3 = assign17180_e15592_d_n3;
        locals.var_t4__blk812_dn4 = assign17180_e15592_d_n4;
        locals.var_t4__blk812_dn5 = assign17180_e15592_d_n5;
        locals.var_t4__blk812_dn6 = assign17180_e15592_d_n6;
        locals.var_t4__blk812_dn7 = assign17180_e15592_d_n7;
        locals.var_t4__blk812_dn8 = assign17180_e15592_d_n8;
        locals.var_t4__blk812_dn9 = assign17180_e15592_d_n9;
        locals.var_t4__blk812_dn10 = assign17180_e15592_d_n10;
        locals.var_t4__blk812_dn11 = assign17180_e15592_d_n11;
        locals.var_t4__blk812_dn12 = assign17180_e15592_d_n12;
        locals.var_t4__blk812_rv = 0.0;

        let (assign17190_e15603, assign17190_e15603_d_n3, assign17190_e15603_d_n4, assign17190_e15603_d_n5, assign17190_e15603_d_n6, assign17190_e15603_d_n7, assign17190_e15603_d_n8, assign17190_e15603_d_n9, assign17190_e15603_d_n10, assign17190_e15603_d_n11, assign17190_e15603_d_n12,) = {
    if (locals.var_guard1199 == 0.0) {
        let assign17190_e15598: f64 = (3.0 * locals.var_t0__blk808);
        let assign17190_e15599: f64 = (1.0 + assign17190_e15598);
        let assign17190_e15601: f64 = (assign17190_e15599 * locals.var_t4__blk812);
        (assign17190_e15601, (((3.0 * locals.var_t0__blk808_dn3) * locals.var_t4__blk812) + (assign17190_e15599 * locals.var_t4__blk812_dn3)), (((3.0 * locals.var_t0__blk808_dn4) * locals.var_t4__blk812) + (assign17190_e15599 * locals.var_t4__blk812_dn4)), (((3.0 * locals.var_t0__blk808_dn5) * locals.var_t4__blk812) + (assign17190_e15599 * locals.var_t4__blk812_dn5)), (((3.0 * locals.var_t0__blk808_dn6) * locals.var_t4__blk812) + (assign17190_e15599 * locals.var_t4__blk812_dn6)), (((3.0 * locals.var_t0__blk808_dn7) * locals.var_t4__blk812) + (assign17190_e15599 * locals.var_t4__blk812_dn7)), (((3.0 * locals.var_t0__blk808_dn8) * locals.var_t4__blk812) + (assign17190_e15599 * locals.var_t4__blk812_dn8)), (((3.0 * locals.var_t0__blk808_dn9) * locals.var_t4__blk812) + (assign17190_e15599 * locals.var_t4__blk812_dn9)), (((3.0 * locals.var_t0__blk808_dn10) * locals.var_t4__blk812) + (assign17190_e15599 * locals.var_t4__blk812_dn10)), (((3.0 * locals.var_t0__blk808_dn11) * locals.var_t4__blk812) + (assign17190_e15599 * locals.var_t4__blk812_dn11)), (((3.0 * locals.var_t0__blk808_dn12) * locals.var_t4__blk812) + (assign17190_e15599 * locals.var_t4__blk812_dn12)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign17190_e15603;
        locals.var_t1__blk809_dn3 = assign17190_e15603_d_n3;
        locals.var_t1__blk809_dn4 = assign17190_e15603_d_n4;
        locals.var_t1__blk809_dn5 = assign17190_e15603_d_n5;
        locals.var_t1__blk809_dn6 = assign17190_e15603_d_n6;
        locals.var_t1__blk809_dn7 = assign17190_e15603_d_n7;
        locals.var_t1__blk809_dn8 = assign17190_e15603_d_n8;
        locals.var_t1__blk809_dn9 = assign17190_e15603_d_n9;
        locals.var_t1__blk809_dn10 = assign17190_e15603_d_n10;
        locals.var_t1__blk809_dn11 = assign17190_e15603_d_n11;
        locals.var_t1__blk809_dn12 = assign17190_e15603_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let assign17200_e15606: f64 = (locals.var_b4soifactor1 * locals.var_t3__blk811);
        let assign17200_e15608: f64 = (assign17200_e15606 * locals.var_t1__blk809);
        locals.var_ltw_cv = assign17200_e15608;
        locals.var_ltw_cv_dn3 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn3) * locals.var_t1__blk809) + (assign17200_e15606 * locals.var_t1__blk809_dn3));
        locals.var_ltw_cv_dn4 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn4) * locals.var_t1__blk809) + (assign17200_e15606 * locals.var_t1__blk809_dn4));
        locals.var_ltw_cv_dn5 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn5) * locals.var_t1__blk809) + (assign17200_e15606 * locals.var_t1__blk809_dn5));
        locals.var_ltw_cv_dn6 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn6) * locals.var_t1__blk809) + (assign17200_e15606 * locals.var_t1__blk809_dn6));
        locals.var_ltw_cv_dn7 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn7) * locals.var_t1__blk809) + (assign17200_e15606 * locals.var_t1__blk809_dn7));
        locals.var_ltw_cv_dn8 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn8) * locals.var_t1__blk809) + (assign17200_e15606 * locals.var_t1__blk809_dn8));
        locals.var_ltw_cv_dn9 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn9) * locals.var_t1__blk809) + (assign17200_e15606 * locals.var_t1__blk809_dn9));
        locals.var_ltw_cv_dn10 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn10) * locals.var_t1__blk809) + (assign17200_e15606 * locals.var_t1__blk809_dn10));
        locals.var_ltw_cv_dn11 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn11) * locals.var_t1__blk809) + (assign17200_e15606 * locals.var_t1__blk809_dn11));
        locals.var_ltw_cv_dn12 = (((locals.var_b4soifactor1 * locals.var_t3__blk811_dn12) * locals.var_t1__blk809) + (assign17200_e15606 * locals.var_t1__blk809_dn12));
        locals.var_ltw_cv_rv = 0.0;

        let assign17210_e15610: f64 = (-0.5);
        let assign17210_e15612: f64 = (assign17210_e15610 * locals.var_pparam_b4soidvt1);
        let assign17210_e15614: f64 = (assign17210_e15612 * locals.var_leff);
        let assign17210_e15616: f64 = (assign17210_e15614 / locals.var_lt1_cv);
        locals.var_t0__blk808 = assign17210_e15616;
        locals.var_t0__blk808_dn3 = ((((((assign17210_e15610 * locals.var_pparam_b4soidvt1_dn3) * locals.var_leff) + (assign17210_e15612 * locals.var_leff_dn3)) * locals.var_lt1_cv) - (assign17210_e15614 * locals.var_lt1_cv_dn3)) / (locals.var_lt1_cv * locals.var_lt1_cv));
        locals.var_t0__blk808_dn4 = ((((((assign17210_e15610 * locals.var_pparam_b4soidvt1_dn4) * locals.var_leff) + (assign17210_e15612 * locals.var_leff_dn4)) * locals.var_lt1_cv) - (assign17210_e15614 * locals.var_lt1_cv_dn4)) / (locals.var_lt1_cv * locals.var_lt1_cv));
        locals.var_t0__blk808_dn5 = ((((((assign17210_e15610 * locals.var_pparam_b4soidvt1_dn5) * locals.var_leff) + (assign17210_e15612 * locals.var_leff_dn5)) * locals.var_lt1_cv) - (assign17210_e15614 * locals.var_lt1_cv_dn5)) / (locals.var_lt1_cv * locals.var_lt1_cv));
        locals.var_t0__blk808_dn6 = ((((((assign17210_e15610 * locals.var_pparam_b4soidvt1_dn6) * locals.var_leff) + (assign17210_e15612 * locals.var_leff_dn6)) * locals.var_lt1_cv) - (assign17210_e15614 * locals.var_lt1_cv_dn6)) / (locals.var_lt1_cv * locals.var_lt1_cv));
        locals.var_t0__blk808_dn7 = ((((((assign17210_e15610 * locals.var_pparam_b4soidvt1_dn7) * locals.var_leff) + (assign17210_e15612 * locals.var_leff_dn7)) * locals.var_lt1_cv) - (assign17210_e15614 * locals.var_lt1_cv_dn7)) / (locals.var_lt1_cv * locals.var_lt1_cv));
        locals.var_t0__blk808_dn8 = ((((((assign17210_e15610 * locals.var_pparam_b4soidvt1_dn8) * locals.var_leff) + (assign17210_e15612 * locals.var_leff_dn8)) * locals.var_lt1_cv) - (assign17210_e15614 * locals.var_lt1_cv_dn8)) / (locals.var_lt1_cv * locals.var_lt1_cv));
        locals.var_t0__blk808_dn9 = ((((((assign17210_e15610 * locals.var_pparam_b4soidvt1_dn9) * locals.var_leff) + (assign17210_e15612 * locals.var_leff_dn9)) * locals.var_lt1_cv) - (assign17210_e15614 * locals.var_lt1_cv_dn9)) / (locals.var_lt1_cv * locals.var_lt1_cv));
        locals.var_t0__blk808_dn10 = ((((((assign17210_e15610 * locals.var_pparam_b4soidvt1_dn10) * locals.var_leff) + (assign17210_e15612 * locals.var_leff_dn10)) * locals.var_lt1_cv) - (assign17210_e15614 * locals.var_lt1_cv_dn10)) / (locals.var_lt1_cv * locals.var_lt1_cv));
        locals.var_t0__blk808_dn11 = ((((((assign17210_e15610 * locals.var_pparam_b4soidvt1_dn11) * locals.var_leff) + (assign17210_e15612 * locals.var_leff_dn11)) * locals.var_lt1_cv) - (assign17210_e15614 * locals.var_lt1_cv_dn11)) / (locals.var_lt1_cv * locals.var_lt1_cv));
        locals.var_t0__blk808_dn12 = ((((((assign17210_e15610 * locals.var_pparam_b4soidvt1_dn12) * locals.var_leff) + (assign17210_e15612 * locals.var_leff_dn12)) * locals.var_lt1_cv) - (assign17210_e15614 * locals.var_lt1_cv_dn12)) / (locals.var_lt1_cv * locals.var_lt1_cv));
        locals.var_t0__blk808_rv = 0.0;

        let assign17220_e15619: f64 = (-100.0);
        let assign17220_e15620: f64 = if locals.var_t0__blk808 > assign17220_e15619 { 1.0 } else { 0.0 };
        locals.var_guard1200 = assign17220_e15620;
        locals.var_guard1200_rv = 0.0;

        let (assign17230_e15625, assign17230_e15625_d_n3, assign17230_e15625_d_n4, assign17230_e15625_d_n5, assign17230_e15625_d_n6, assign17230_e15625_d_n7, assign17230_e15625_d_n8, assign17230_e15625_d_n9, assign17230_e15625_d_n10, assign17230_e15625_d_n11, assign17230_e15625_d_n12,) = {
    if (locals.var_guard1200 != 0.0) {
        let assign17230_e15623: f64 = (locals.var_t0__blk808).exp();
        (assign17230_e15623, (assign17230_e15623 * locals.var_t0__blk808_dn3), (assign17230_e15623 * locals.var_t0__blk808_dn4), (assign17230_e15623 * locals.var_t0__blk808_dn5), (assign17230_e15623 * locals.var_t0__blk808_dn6), (assign17230_e15623 * locals.var_t0__blk808_dn7), (assign17230_e15623 * locals.var_t0__blk808_dn8), (assign17230_e15623 * locals.var_t0__blk808_dn9), (assign17230_e15623 * locals.var_t0__blk808_dn10), (assign17230_e15623 * locals.var_t0__blk808_dn11), (assign17230_e15623 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign17230_e15625;
        locals.var_t1__blk809_dn3 = assign17230_e15625_d_n3;
        locals.var_t1__blk809_dn4 = assign17230_e15625_d_n4;
        locals.var_t1__blk809_dn5 = assign17230_e15625_d_n5;
        locals.var_t1__blk809_dn6 = assign17230_e15625_d_n6;
        locals.var_t1__blk809_dn7 = assign17230_e15625_d_n7;
        locals.var_t1__blk809_dn8 = assign17230_e15625_d_n8;
        locals.var_t1__blk809_dn9 = assign17230_e15625_d_n9;
        locals.var_t1__blk809_dn10 = assign17230_e15625_d_n10;
        locals.var_t1__blk809_dn11 = assign17230_e15625_d_n11;
        locals.var_t1__blk809_dn12 = assign17230_e15625_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign17240_e15635, assign17240_e15635_d_n3, assign17240_e15635_d_n4, assign17240_e15635_d_n5, assign17240_e15635_d_n6, assign17240_e15635_d_n7, assign17240_e15635_d_n8, assign17240_e15635_d_n9, assign17240_e15635_d_n10, assign17240_e15635_d_n11, assign17240_e15635_d_n12,) = {
    if (locals.var_guard1200 != 0.0) {
        let assign17240_e15631: f64 = (2.0 * locals.var_t1__blk809);
        let assign17240_e15632: f64 = (1.0 + assign17240_e15631);
        let assign17240_e15633: f64 = (locals.var_t1__blk809 * assign17240_e15632);
        (assign17240_e15633, ((locals.var_t1__blk809_dn3 * assign17240_e15632) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn3))), ((locals.var_t1__blk809_dn4 * assign17240_e15632) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn4))), ((locals.var_t1__blk809_dn5 * assign17240_e15632) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn5))), ((locals.var_t1__blk809_dn6 * assign17240_e15632) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn6))), ((locals.var_t1__blk809_dn7 * assign17240_e15632) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn7))), ((locals.var_t1__blk809_dn8 * assign17240_e15632) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn8))), ((locals.var_t1__blk809_dn9 * assign17240_e15632) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn9))), ((locals.var_t1__blk809_dn10 * assign17240_e15632) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn10))), ((locals.var_t1__blk809_dn11 * assign17240_e15632) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn11))), ((locals.var_t1__blk809_dn12 * assign17240_e15632) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn12))),)
    } else {
        (locals.var_theta0_cv, locals.var_theta0_cv_dn3, locals.var_theta0_cv_dn4, locals.var_theta0_cv_dn5, locals.var_theta0_cv_dn6, locals.var_theta0_cv_dn7, locals.var_theta0_cv_dn8, locals.var_theta0_cv_dn9, locals.var_theta0_cv_dn10, locals.var_theta0_cv_dn11, locals.var_theta0_cv_dn12,)
    }
};
        locals.var_theta0_cv = assign17240_e15635;
        locals.var_theta0_cv_dn3 = assign17240_e15635_d_n3;
        locals.var_theta0_cv_dn4 = assign17240_e15635_d_n4;
        locals.var_theta0_cv_dn5 = assign17240_e15635_d_n5;
        locals.var_theta0_cv_dn6 = assign17240_e15635_d_n6;
        locals.var_theta0_cv_dn7 = assign17240_e15635_d_n7;
        locals.var_theta0_cv_dn8 = assign17240_e15635_d_n8;
        locals.var_theta0_cv_dn9 = assign17240_e15635_d_n9;
        locals.var_theta0_cv_dn10 = assign17240_e15635_d_n10;
        locals.var_theta0_cv_dn11 = assign17240_e15635_d_n11;
        locals.var_theta0_cv_dn12 = assign17240_e15635_d_n12;
        locals.var_theta0_cv_rv = 0.0;

        let (assign17250_e15640, assign17250_e15640_d_n3, assign17250_e15640_d_n4, assign17250_e15640_d_n5, assign17250_e15640_d_n6, assign17250_e15640_d_n7, assign17250_e15640_d_n8, assign17250_e15640_d_n9, assign17250_e15640_d_n10, assign17250_e15640_d_n11, assign17250_e15640_d_n12,) = {
    if (locals.var_guard1200 == 0.0) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign17250_e15640;
        locals.var_t1__blk809_dn3 = assign17250_e15640_d_n3;
        locals.var_t1__blk809_dn4 = assign17250_e15640_d_n4;
        locals.var_t1__blk809_dn5 = assign17250_e15640_d_n5;
        locals.var_t1__blk809_dn6 = assign17250_e15640_d_n6;
        locals.var_t1__blk809_dn7 = assign17250_e15640_d_n7;
        locals.var_t1__blk809_dn8 = assign17250_e15640_d_n8;
        locals.var_t1__blk809_dn9 = assign17250_e15640_d_n9;
        locals.var_t1__blk809_dn10 = assign17250_e15640_d_n10;
        locals.var_t1__blk809_dn11 = assign17250_e15640_d_n11;
        locals.var_t1__blk809_dn12 = assign17250_e15640_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign17260_e15651, assign17260_e15651_d_n3, assign17260_e15651_d_n4, assign17260_e15651_d_n5, assign17260_e15651_d_n6, assign17260_e15651_d_n7, assign17260_e15651_d_n8, assign17260_e15651_d_n9, assign17260_e15651_d_n10, assign17260_e15651_d_n11, assign17260_e15651_d_n12,) = {
    if (locals.var_guard1200 == 0.0) {
        let assign17260_e15647: f64 = (2.0 * locals.var_t1__blk809);
        let assign17260_e15648: f64 = (1.0 + assign17260_e15647);
        let assign17260_e15649: f64 = (locals.var_t1__blk809 * assign17260_e15648);
        (assign17260_e15649, ((locals.var_t1__blk809_dn3 * assign17260_e15648) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn3))), ((locals.var_t1__blk809_dn4 * assign17260_e15648) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn4))), ((locals.var_t1__blk809_dn5 * assign17260_e15648) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn5))), ((locals.var_t1__blk809_dn6 * assign17260_e15648) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn6))), ((locals.var_t1__blk809_dn7 * assign17260_e15648) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn7))), ((locals.var_t1__blk809_dn8 * assign17260_e15648) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn8))), ((locals.var_t1__blk809_dn9 * assign17260_e15648) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn9))), ((locals.var_t1__blk809_dn10 * assign17260_e15648) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn10))), ((locals.var_t1__blk809_dn11 * assign17260_e15648) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn11))), ((locals.var_t1__blk809_dn12 * assign17260_e15648) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn12))),)
    } else {
        (locals.var_theta0_cv, locals.var_theta0_cv_dn3, locals.var_theta0_cv_dn4, locals.var_theta0_cv_dn5, locals.var_theta0_cv_dn6, locals.var_theta0_cv_dn7, locals.var_theta0_cv_dn8, locals.var_theta0_cv_dn9, locals.var_theta0_cv_dn10, locals.var_theta0_cv_dn11, locals.var_theta0_cv_dn12,)
    }
};
        locals.var_theta0_cv = assign17260_e15651;
        locals.var_theta0_cv_dn3 = assign17260_e15651_d_n3;
        locals.var_theta0_cv_dn4 = assign17260_e15651_d_n4;
        locals.var_theta0_cv_dn5 = assign17260_e15651_d_n5;
        locals.var_theta0_cv_dn6 = assign17260_e15651_d_n6;
        locals.var_theta0_cv_dn7 = assign17260_e15651_d_n7;
        locals.var_theta0_cv_dn8 = assign17260_e15651_d_n8;
        locals.var_theta0_cv_dn9 = assign17260_e15651_d_n9;
        locals.var_theta0_cv_dn10 = assign17260_e15651_d_n10;
        locals.var_theta0_cv_dn11 = assign17260_e15651_d_n11;
        locals.var_theta0_cv_dn12 = assign17260_e15651_d_n12;
        locals.var_theta0_cv_rv = 0.0;

        let assign17270_e15654: f64 = (locals.var_pparam_b4soinfactor * locals.var_epssub);
        let assign17270_e15656: f64 = (assign17270_e15654 / locals.var_xdep_cv);
        locals.var_t2__blk810 = assign17270_e15656;
        locals.var_t2__blk810_dn3 = ((((locals.var_pparam_b4soinfactor_dn3 * locals.var_epssub) * locals.var_xdep_cv) - (assign17270_e15654 * locals.var_xdep_cv_dn3)) / (locals.var_xdep_cv * locals.var_xdep_cv));
        locals.var_t2__blk810_dn4 = ((((locals.var_pparam_b4soinfactor_dn4 * locals.var_epssub) * locals.var_xdep_cv) - (assign17270_e15654 * locals.var_xdep_cv_dn4)) / (locals.var_xdep_cv * locals.var_xdep_cv));
        locals.var_t2__blk810_dn5 = ((((locals.var_pparam_b4soinfactor_dn5 * locals.var_epssub) * locals.var_xdep_cv) - (assign17270_e15654 * locals.var_xdep_cv_dn5)) / (locals.var_xdep_cv * locals.var_xdep_cv));
        locals.var_t2__blk810_dn6 = ((((locals.var_pparam_b4soinfactor_dn6 * locals.var_epssub) * locals.var_xdep_cv) - (assign17270_e15654 * locals.var_xdep_cv_dn6)) / (locals.var_xdep_cv * locals.var_xdep_cv));
        locals.var_t2__blk810_dn7 = ((((locals.var_pparam_b4soinfactor_dn7 * locals.var_epssub) * locals.var_xdep_cv) - (assign17270_e15654 * locals.var_xdep_cv_dn7)) / (locals.var_xdep_cv * locals.var_xdep_cv));
        locals.var_t2__blk810_dn8 = ((((locals.var_pparam_b4soinfactor_dn8 * locals.var_epssub) * locals.var_xdep_cv) - (assign17270_e15654 * locals.var_xdep_cv_dn8)) / (locals.var_xdep_cv * locals.var_xdep_cv));
        locals.var_t2__blk810_dn9 = ((((locals.var_pparam_b4soinfactor_dn9 * locals.var_epssub) * locals.var_xdep_cv) - (assign17270_e15654 * locals.var_xdep_cv_dn9)) / (locals.var_xdep_cv * locals.var_xdep_cv));
        locals.var_t2__blk810_dn10 = ((((locals.var_pparam_b4soinfactor_dn10 * locals.var_epssub) * locals.var_xdep_cv) - (assign17270_e15654 * locals.var_xdep_cv_dn10)) / (locals.var_xdep_cv * locals.var_xdep_cv));
        locals.var_t2__blk810_dn11 = ((((locals.var_pparam_b4soinfactor_dn11 * locals.var_epssub) * locals.var_xdep_cv) - (assign17270_e15654 * locals.var_xdep_cv_dn11)) / (locals.var_xdep_cv * locals.var_xdep_cv));
        locals.var_t2__blk810_dn12 = ((((locals.var_pparam_b4soinfactor_dn12 * locals.var_epssub) * locals.var_xdep_cv) - (assign17270_e15654 * locals.var_xdep_cv_dn12)) / (locals.var_xdep_cv * locals.var_xdep_cv));
        locals.var_t2__blk810_rv = 0.0;

        let assign17280_e15660: f64 = (locals.var_pparam_b4soicdscb * locals.var_vbseff_cv);
        let assign17280_e15661: f64 = (locals.var_pparam_b4soicdsc + assign17280_e15660);
        let assign17280_e15664: f64 = (locals.var_pparam_b4soicdscd * locals.var_vds_1);
        let assign17280_e15665: f64 = (assign17280_e15661 + assign17280_e15664);
        locals.var_t3__blk811 = assign17280_e15665;
        locals.var_t3__blk811_dn3 = ((locals.var_pparam_b4soicdsc_dn3 + ((locals.var_pparam_b4soicdscb_dn3 * locals.var_vbseff_cv) + (locals.var_pparam_b4soicdscb * locals.var_vbseff_cv_dn3))) + (locals.var_pparam_b4soicdscd_dn3 * locals.var_vds_1));
        locals.var_t3__blk811_dn4 = ((locals.var_pparam_b4soicdsc_dn4 + ((locals.var_pparam_b4soicdscb_dn4 * locals.var_vbseff_cv) + (locals.var_pparam_b4soicdscb * locals.var_vbseff_cv_dn4))) + (locals.var_pparam_b4soicdscd_dn4 * locals.var_vds_1));
        locals.var_t3__blk811_dn5 = ((locals.var_pparam_b4soicdsc_dn5 + ((locals.var_pparam_b4soicdscb_dn5 * locals.var_vbseff_cv) + (locals.var_pparam_b4soicdscb * locals.var_vbseff_cv_dn5))) + (locals.var_pparam_b4soicdscd_dn5 * locals.var_vds_1));
        locals.var_t3__blk811_dn6 = ((locals.var_pparam_b4soicdsc_dn6 + ((locals.var_pparam_b4soicdscb_dn6 * locals.var_vbseff_cv) + (locals.var_pparam_b4soicdscb * locals.var_vbseff_cv_dn6))) + (locals.var_pparam_b4soicdscd_dn6 * locals.var_vds_1));
        locals.var_t3__blk811_dn7 = ((locals.var_pparam_b4soicdsc_dn7 + ((locals.var_pparam_b4soicdscb_dn7 * locals.var_vbseff_cv) + (locals.var_pparam_b4soicdscb * locals.var_vbseff_cv_dn7))) + ((locals.var_pparam_b4soicdscd_dn7 * locals.var_vds_1) + (locals.var_pparam_b4soicdscd * locals.var_vds_1_dn7)));
        locals.var_t3__blk811_dn8 = ((locals.var_pparam_b4soicdsc_dn8 + ((locals.var_pparam_b4soicdscb_dn8 * locals.var_vbseff_cv) + (locals.var_pparam_b4soicdscb * locals.var_vbseff_cv_dn8))) + ((locals.var_pparam_b4soicdscd_dn8 * locals.var_vds_1) + (locals.var_pparam_b4soicdscd * locals.var_vds_1_dn8)));
        locals.var_t3__blk811_dn9 = ((locals.var_pparam_b4soicdsc_dn9 + ((locals.var_pparam_b4soicdscb_dn9 * locals.var_vbseff_cv) + (locals.var_pparam_b4soicdscb * locals.var_vbseff_cv_dn9))) + (locals.var_pparam_b4soicdscd_dn9 * locals.var_vds_1));
        locals.var_t3__blk811_dn10 = ((locals.var_pparam_b4soicdsc_dn10 + ((locals.var_pparam_b4soicdscb_dn10 * locals.var_vbseff_cv) + (locals.var_pparam_b4soicdscb * locals.var_vbseff_cv_dn10))) + (locals.var_pparam_b4soicdscd_dn10 * locals.var_vds_1));
        locals.var_t3__blk811_dn11 = ((locals.var_pparam_b4soicdsc_dn11 + ((locals.var_pparam_b4soicdscb_dn11 * locals.var_vbseff_cv) + (locals.var_pparam_b4soicdscb * locals.var_vbseff_cv_dn11))) + (locals.var_pparam_b4soicdscd_dn11 * locals.var_vds_1));
        locals.var_t3__blk811_dn12 = ((locals.var_pparam_b4soicdsc_dn12 + ((locals.var_pparam_b4soicdscb_dn12 * locals.var_vbseff_cv) + (locals.var_pparam_b4soicdscb * locals.var_vbseff_cv_dn12))) + (locals.var_pparam_b4soicdscd_dn12 * locals.var_vds_1));
        locals.var_t3__blk811_rv = 0.0;

        let assign17290_e15669: f64 = (locals.var_t3__blk811 * locals.var_theta0_cv);
        let assign17290_e15670: f64 = (locals.var_t2__blk810 + assign17290_e15669);
        let assign17290_e15672: f64 = (assign17290_e15670 + locals.var_pparam_b4soicit);
        let assign17290_e15674: f64 = (assign17290_e15672 / locals.var_b4soicox);
        locals.var_t4__blk812 = assign17290_e15674;
        locals.var_t4__blk812_dn3 = (((locals.var_t2__blk810_dn3 + ((locals.var_t3__blk811_dn3 * locals.var_theta0_cv) + (locals.var_t3__blk811 * locals.var_theta0_cv_dn3))) + locals.var_pparam_b4soicit_dn3) / locals.var_b4soicox);
        locals.var_t4__blk812_dn4 = (((locals.var_t2__blk810_dn4 + ((locals.var_t3__blk811_dn4 * locals.var_theta0_cv) + (locals.var_t3__blk811 * locals.var_theta0_cv_dn4))) + locals.var_pparam_b4soicit_dn4) / locals.var_b4soicox);
        locals.var_t4__blk812_dn5 = (((locals.var_t2__blk810_dn5 + ((locals.var_t3__blk811_dn5 * locals.var_theta0_cv) + (locals.var_t3__blk811 * locals.var_theta0_cv_dn5))) + locals.var_pparam_b4soicit_dn5) / locals.var_b4soicox);
        locals.var_t4__blk812_dn6 = (((locals.var_t2__blk810_dn6 + ((locals.var_t3__blk811_dn6 * locals.var_theta0_cv) + (locals.var_t3__blk811 * locals.var_theta0_cv_dn6))) + locals.var_pparam_b4soicit_dn6) / locals.var_b4soicox);
        locals.var_t4__blk812_dn7 = (((locals.var_t2__blk810_dn7 + ((locals.var_t3__blk811_dn7 * locals.var_theta0_cv) + (locals.var_t3__blk811 * locals.var_theta0_cv_dn7))) + locals.var_pparam_b4soicit_dn7) / locals.var_b4soicox);
        locals.var_t4__blk812_dn8 = (((locals.var_t2__blk810_dn8 + ((locals.var_t3__blk811_dn8 * locals.var_theta0_cv) + (locals.var_t3__blk811 * locals.var_theta0_cv_dn8))) + locals.var_pparam_b4soicit_dn8) / locals.var_b4soicox);
        locals.var_t4__blk812_dn9 = (((locals.var_t2__blk810_dn9 + ((locals.var_t3__blk811_dn9 * locals.var_theta0_cv) + (locals.var_t3__blk811 * locals.var_theta0_cv_dn9))) + locals.var_pparam_b4soicit_dn9) / locals.var_b4soicox);
        locals.var_t4__blk812_dn10 = (((locals.var_t2__blk810_dn10 + ((locals.var_t3__blk811_dn10 * locals.var_theta0_cv) + (locals.var_t3__blk811 * locals.var_theta0_cv_dn10))) + locals.var_pparam_b4soicit_dn10) / locals.var_b4soicox);
        locals.var_t4__blk812_dn11 = (((locals.var_t2__blk810_dn11 + ((locals.var_t3__blk811_dn11 * locals.var_theta0_cv) + (locals.var_t3__blk811 * locals.var_theta0_cv_dn11))) + locals.var_pparam_b4soicit_dn11) / locals.var_b4soicox);
        locals.var_t4__blk812_dn12 = (((locals.var_t2__blk810_dn12 + ((locals.var_t3__blk811_dn12 * locals.var_theta0_cv) + (locals.var_t3__blk811 * locals.var_theta0_cv_dn12))) + locals.var_pparam_b4soicit_dn12) / locals.var_b4soicox);
        locals.var_t4__blk812_rv = 0.0;

        let assign17300_e15677: f64 = (-0.5);
        let assign17300_e15678: f64 = if locals.var_t4__blk812 >= assign17300_e15677 { 1.0 } else { 0.0 };
        locals.var_guard1201 = assign17300_e15678;
        locals.var_guard1201_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_51(
        locals: &mut StampLocals,
    ) {
        let (assign17310_e15684, assign17310_e15684_d_n3, assign17310_e15684_d_n4, assign17310_e15684_d_n5, assign17310_e15684_d_n6, assign17310_e15684_d_n7, assign17310_e15684_d_n8, assign17310_e15684_d_n9, assign17310_e15684_d_n10, assign17310_e15684_d_n11, assign17310_e15684_d_n12,) = {
    if (locals.var_guard1201 != 0.0) {
        let assign17310_e15682: f64 = (1.0 + locals.var_t4__blk812);
        (assign17310_e15682, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    } else {
        (locals.var_n_cv, locals.var_n_cv_dn3, locals.var_n_cv_dn4, locals.var_n_cv_dn5, locals.var_n_cv_dn6, locals.var_n_cv_dn7, locals.var_n_cv_dn8, locals.var_n_cv_dn9, locals.var_n_cv_dn10, locals.var_n_cv_dn11, locals.var_n_cv_dn12,)
    }
};
        locals.var_n_cv = assign17310_e15684;
        locals.var_n_cv_dn3 = assign17310_e15684_d_n3;
        locals.var_n_cv_dn4 = assign17310_e15684_d_n4;
        locals.var_n_cv_dn5 = assign17310_e15684_d_n5;
        locals.var_n_cv_dn6 = assign17310_e15684_d_n6;
        locals.var_n_cv_dn7 = assign17310_e15684_d_n7;
        locals.var_n_cv_dn8 = assign17310_e15684_d_n8;
        locals.var_n_cv_dn9 = assign17310_e15684_d_n9;
        locals.var_n_cv_dn10 = assign17310_e15684_d_n10;
        locals.var_n_cv_dn11 = assign17310_e15684_d_n11;
        locals.var_n_cv_dn12 = assign17310_e15684_d_n12;
        locals.var_n_cv_rv = 0.0;

        let (assign17320_e15695, assign17320_e15695_d_n3, assign17320_e15695_d_n4, assign17320_e15695_d_n5, assign17320_e15695_d_n6, assign17320_e15695_d_n7, assign17320_e15695_d_n8, assign17320_e15695_d_n9, assign17320_e15695_d_n10, assign17320_e15695_d_n11, assign17320_e15695_d_n12,) = {
    if (locals.var_guard1201 == 0.0) {
        let assign17320_e15691: f64 = (8.0 * locals.var_t4__blk812);
        let assign17320_e15692: f64 = (3.0 + assign17320_e15691);
        let assign17320_e15693: f64 = (1.0 / assign17320_e15692);
        (assign17320_e15693, (-((8.0 * locals.var_t4__blk812_dn3) / (assign17320_e15692 * assign17320_e15692))), (-((8.0 * locals.var_t4__blk812_dn4) / (assign17320_e15692 * assign17320_e15692))), (-((8.0 * locals.var_t4__blk812_dn5) / (assign17320_e15692 * assign17320_e15692))), (-((8.0 * locals.var_t4__blk812_dn6) / (assign17320_e15692 * assign17320_e15692))), (-((8.0 * locals.var_t4__blk812_dn7) / (assign17320_e15692 * assign17320_e15692))), (-((8.0 * locals.var_t4__blk812_dn8) / (assign17320_e15692 * assign17320_e15692))), (-((8.0 * locals.var_t4__blk812_dn9) / (assign17320_e15692 * assign17320_e15692))), (-((8.0 * locals.var_t4__blk812_dn10) / (assign17320_e15692 * assign17320_e15692))), (-((8.0 * locals.var_t4__blk812_dn11) / (assign17320_e15692 * assign17320_e15692))), (-((8.0 * locals.var_t4__blk812_dn12) / (assign17320_e15692 * assign17320_e15692))),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign17320_e15695;
        locals.var_t0__blk808_dn3 = assign17320_e15695_d_n3;
        locals.var_t0__blk808_dn4 = assign17320_e15695_d_n4;
        locals.var_t0__blk808_dn5 = assign17320_e15695_d_n5;
        locals.var_t0__blk808_dn6 = assign17320_e15695_d_n6;
        locals.var_t0__blk808_dn7 = assign17320_e15695_d_n7;
        locals.var_t0__blk808_dn8 = assign17320_e15695_d_n8;
        locals.var_t0__blk808_dn9 = assign17320_e15695_d_n9;
        locals.var_t0__blk808_dn10 = assign17320_e15695_d_n10;
        locals.var_t0__blk808_dn11 = assign17320_e15695_d_n11;
        locals.var_t0__blk808_dn12 = assign17320_e15695_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign17330_e15706, assign17330_e15706_d_n3, assign17330_e15706_d_n4, assign17330_e15706_d_n5, assign17330_e15706_d_n6, assign17330_e15706_d_n7, assign17330_e15706_d_n8, assign17330_e15706_d_n9, assign17330_e15706_d_n10, assign17330_e15706_d_n11, assign17330_e15706_d_n12,) = {
    if (locals.var_guard1201 == 0.0) {
        let assign17330_e15701: f64 = (3.0 * locals.var_t4__blk812);
        let assign17330_e15702: f64 = (1.0 + assign17330_e15701);
        let assign17330_e15704: f64 = (assign17330_e15702 * locals.var_t0__blk808);
        (assign17330_e15704, (((3.0 * locals.var_t4__blk812_dn3) * locals.var_t0__blk808) + (assign17330_e15702 * locals.var_t0__blk808_dn3)), (((3.0 * locals.var_t4__blk812_dn4) * locals.var_t0__blk808) + (assign17330_e15702 * locals.var_t0__blk808_dn4)), (((3.0 * locals.var_t4__blk812_dn5) * locals.var_t0__blk808) + (assign17330_e15702 * locals.var_t0__blk808_dn5)), (((3.0 * locals.var_t4__blk812_dn6) * locals.var_t0__blk808) + (assign17330_e15702 * locals.var_t0__blk808_dn6)), (((3.0 * locals.var_t4__blk812_dn7) * locals.var_t0__blk808) + (assign17330_e15702 * locals.var_t0__blk808_dn7)), (((3.0 * locals.var_t4__blk812_dn8) * locals.var_t0__blk808) + (assign17330_e15702 * locals.var_t0__blk808_dn8)), (((3.0 * locals.var_t4__blk812_dn9) * locals.var_t0__blk808) + (assign17330_e15702 * locals.var_t0__blk808_dn9)), (((3.0 * locals.var_t4__blk812_dn10) * locals.var_t0__blk808) + (assign17330_e15702 * locals.var_t0__blk808_dn10)), (((3.0 * locals.var_t4__blk812_dn11) * locals.var_t0__blk808) + (assign17330_e15702 * locals.var_t0__blk808_dn11)), (((3.0 * locals.var_t4__blk812_dn12) * locals.var_t0__blk808) + (assign17330_e15702 * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_n_cv, locals.var_n_cv_dn3, locals.var_n_cv_dn4, locals.var_n_cv_dn5, locals.var_n_cv_dn6, locals.var_n_cv_dn7, locals.var_n_cv_dn8, locals.var_n_cv_dn9, locals.var_n_cv_dn10, locals.var_n_cv_dn11, locals.var_n_cv_dn12,)
    }
};
        locals.var_n_cv = assign17330_e15706;
        locals.var_n_cv_dn3 = assign17330_e15706_d_n3;
        locals.var_n_cv_dn4 = assign17330_e15706_d_n4;
        locals.var_n_cv_dn5 = assign17330_e15706_d_n5;
        locals.var_n_cv_dn6 = assign17330_e15706_d_n6;
        locals.var_n_cv_dn7 = assign17330_e15706_d_n7;
        locals.var_n_cv_dn8 = assign17330_e15706_d_n8;
        locals.var_n_cv_dn9 = assign17330_e15706_d_n9;
        locals.var_n_cv_dn10 = assign17330_e15706_d_n10;
        locals.var_n_cv_dn11 = assign17330_e15706_d_n11;
        locals.var_n_cv_dn12 = assign17330_e15706_d_n12;
        locals.var_n_cv_rv = 0.0;

        let assign17340_e15709: f64 = if locals.var_pparam_b4soidvtp0 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1202 = assign17340_e15709;
        locals.var_guard1202_rv = 0.0;

        let (assign17350_e15716, assign17350_e15716_d_n3, assign17350_e15716_d_n4, assign17350_e15716_d_n5, assign17350_e15716_d_n6, assign17350_e15716_d_n7, assign17350_e15716_d_n8, assign17350_e15716_d_n9, assign17350_e15716_d_n10, assign17350_e15716_d_n11, assign17350_e15716_d_n12,) = {
    if (locals.var_guard1202 != 0.0) {
        let assign17350_e15712: f64 = (-locals.var_pparam_b4soidvtp1);
        let assign17350_e15714: f64 = (assign17350_e15712 * locals.var_vds_1);
        (assign17350_e15714, ((-locals.var_pparam_b4soidvtp1_dn3) * locals.var_vds_1), ((-locals.var_pparam_b4soidvtp1_dn4) * locals.var_vds_1), ((-locals.var_pparam_b4soidvtp1_dn5) * locals.var_vds_1), ((-locals.var_pparam_b4soidvtp1_dn6) * locals.var_vds_1), (((-locals.var_pparam_b4soidvtp1_dn7) * locals.var_vds_1) + (assign17350_e15712 * locals.var_vds_1_dn7)), (((-locals.var_pparam_b4soidvtp1_dn8) * locals.var_vds_1) + (assign17350_e15712 * locals.var_vds_1_dn8)), ((-locals.var_pparam_b4soidvtp1_dn9) * locals.var_vds_1), ((-locals.var_pparam_b4soidvtp1_dn10) * locals.var_vds_1), ((-locals.var_pparam_b4soidvtp1_dn11) * locals.var_vds_1), ((-locals.var_pparam_b4soidvtp1_dn12) * locals.var_vds_1),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign17350_e15716;
        locals.var_t0__blk808_dn3 = assign17350_e15716_d_n3;
        locals.var_t0__blk808_dn4 = assign17350_e15716_d_n4;
        locals.var_t0__blk808_dn5 = assign17350_e15716_d_n5;
        locals.var_t0__blk808_dn6 = assign17350_e15716_d_n6;
        locals.var_t0__blk808_dn7 = assign17350_e15716_d_n7;
        locals.var_t0__blk808_dn8 = assign17350_e15716_d_n8;
        locals.var_t0__blk808_dn9 = assign17350_e15716_d_n9;
        locals.var_t0__blk808_dn10 = assign17350_e15716_d_n10;
        locals.var_t0__blk808_dn11 = assign17350_e15716_d_n11;
        locals.var_t0__blk808_dn12 = assign17350_e15716_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let assign17360_e15719: f64 = (-100.0);
        let assign17360_e15720: f64 = if locals.var_t0__blk808 < assign17360_e15719 { 1.0 } else { 0.0 };
        locals.var_guard1203 = assign17360_e15720;
        locals.var_guard1203_rv = 0.0;

        let (assign17370_e15726, assign17370_e15726_d_n3, assign17370_e15726_d_n4, assign17370_e15726_d_n5, assign17370_e15726_d_n6, assign17370_e15726_d_n7, assign17370_e15726_d_n8, assign17370_e15726_d_n9, assign17370_e15726_d_n10, assign17370_e15726_d_n11, assign17370_e15726_d_n12,) = {
    if ((locals.var_guard1202 != 0.0) && (locals.var_guard1203 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign17370_e15726;
        locals.var_t2__blk810_dn3 = assign17370_e15726_d_n3;
        locals.var_t2__blk810_dn4 = assign17370_e15726_d_n4;
        locals.var_t2__blk810_dn5 = assign17370_e15726_d_n5;
        locals.var_t2__blk810_dn6 = assign17370_e15726_d_n6;
        locals.var_t2__blk810_dn7 = assign17370_e15726_d_n7;
        locals.var_t2__blk810_dn8 = assign17370_e15726_d_n8;
        locals.var_t2__blk810_dn9 = assign17370_e15726_d_n9;
        locals.var_t2__blk810_dn10 = assign17370_e15726_d_n10;
        locals.var_t2__blk810_dn11 = assign17370_e15726_d_n11;
        locals.var_t2__blk810_dn12 = assign17370_e15726_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign17380_e15734, assign17380_e15734_d_n3, assign17380_e15734_d_n4, assign17380_e15734_d_n5, assign17380_e15734_d_n6, assign17380_e15734_d_n7, assign17380_e15734_d_n8, assign17380_e15734_d_n9, assign17380_e15734_d_n10, assign17380_e15734_d_n11, assign17380_e15734_d_n12,) = {
    if ((locals.var_guard1202 != 0.0) && (locals.var_guard1203 == 0.0)) {
        let assign17380_e15732: f64 = (locals.var_t0__blk808).exp();
        (assign17380_e15732, (assign17380_e15732 * locals.var_t0__blk808_dn3), (assign17380_e15732 * locals.var_t0__blk808_dn4), (assign17380_e15732 * locals.var_t0__blk808_dn5), (assign17380_e15732 * locals.var_t0__blk808_dn6), (assign17380_e15732 * locals.var_t0__blk808_dn7), (assign17380_e15732 * locals.var_t0__blk808_dn8), (assign17380_e15732 * locals.var_t0__blk808_dn9), (assign17380_e15732 * locals.var_t0__blk808_dn10), (assign17380_e15732 * locals.var_t0__blk808_dn11), (assign17380_e15732 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign17380_e15734;
        locals.var_t2__blk810_dn3 = assign17380_e15734_d_n3;
        locals.var_t2__blk810_dn4 = assign17380_e15734_d_n4;
        locals.var_t2__blk810_dn5 = assign17380_e15734_d_n5;
        locals.var_t2__blk810_dn6 = assign17380_e15734_d_n6;
        locals.var_t2__blk810_dn7 = assign17380_e15734_d_n7;
        locals.var_t2__blk810_dn8 = assign17380_e15734_d_n8;
        locals.var_t2__blk810_dn9 = assign17380_e15734_d_n9;
        locals.var_t2__blk810_dn10 = assign17380_e15734_d_n10;
        locals.var_t2__blk810_dn11 = assign17380_e15734_d_n11;
        locals.var_t2__blk810_dn12 = assign17380_e15734_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign17390_e15744, assign17390_e15744_d_n3, assign17390_e15744_d_n4, assign17390_e15744_d_n5, assign17390_e15744_d_n6, assign17390_e15744_d_n7, assign17390_e15744_d_n8, assign17390_e15744_d_n9, assign17390_e15744_d_n10, assign17390_e15744_d_n11, assign17390_e15744_d_n12,) = {
    if (locals.var_guard1202 != 0.0) {
        let assign17390_e15740: f64 = (1.0 + locals.var_t2__blk810);
        let assign17390_e15741: f64 = (locals.var_pparam_b4soidvtp0 * assign17390_e15740);
        let assign17390_e15742: f64 = (locals.var_leff + assign17390_e15741);
        (assign17390_e15742, (locals.var_leff_dn3 + ((locals.var_pparam_b4soidvtp0_dn3 * assign17390_e15740) + (locals.var_pparam_b4soidvtp0 * locals.var_t2__blk810_dn3))), (locals.var_leff_dn4 + ((locals.var_pparam_b4soidvtp0_dn4 * assign17390_e15740) + (locals.var_pparam_b4soidvtp0 * locals.var_t2__blk810_dn4))), (locals.var_leff_dn5 + ((locals.var_pparam_b4soidvtp0_dn5 * assign17390_e15740) + (locals.var_pparam_b4soidvtp0 * locals.var_t2__blk810_dn5))), (locals.var_leff_dn6 + ((locals.var_pparam_b4soidvtp0_dn6 * assign17390_e15740) + (locals.var_pparam_b4soidvtp0 * locals.var_t2__blk810_dn6))), (locals.var_leff_dn7 + ((locals.var_pparam_b4soidvtp0_dn7 * assign17390_e15740) + (locals.var_pparam_b4soidvtp0 * locals.var_t2__blk810_dn7))), (locals.var_leff_dn8 + ((locals.var_pparam_b4soidvtp0_dn8 * assign17390_e15740) + (locals.var_pparam_b4soidvtp0 * locals.var_t2__blk810_dn8))), (locals.var_leff_dn9 + ((locals.var_pparam_b4soidvtp0_dn9 * assign17390_e15740) + (locals.var_pparam_b4soidvtp0 * locals.var_t2__blk810_dn9))), (locals.var_leff_dn10 + ((locals.var_pparam_b4soidvtp0_dn10 * assign17390_e15740) + (locals.var_pparam_b4soidvtp0 * locals.var_t2__blk810_dn10))), (locals.var_leff_dn11 + ((locals.var_pparam_b4soidvtp0_dn11 * assign17390_e15740) + (locals.var_pparam_b4soidvtp0 * locals.var_t2__blk810_dn11))), (locals.var_leff_dn12 + ((locals.var_pparam_b4soidvtp0_dn12 * assign17390_e15740) + (locals.var_pparam_b4soidvtp0 * locals.var_t2__blk810_dn12))),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign17390_e15744;
        locals.var_t3__blk811_dn3 = assign17390_e15744_d_n3;
        locals.var_t3__blk811_dn4 = assign17390_e15744_d_n4;
        locals.var_t3__blk811_dn5 = assign17390_e15744_d_n5;
        locals.var_t3__blk811_dn6 = assign17390_e15744_d_n6;
        locals.var_t3__blk811_dn7 = assign17390_e15744_d_n7;
        locals.var_t3__blk811_dn8 = assign17390_e15744_d_n8;
        locals.var_t3__blk811_dn9 = assign17390_e15744_d_n9;
        locals.var_t3__blk811_dn10 = assign17390_e15744_d_n10;
        locals.var_t3__blk811_dn11 = assign17390_e15744_d_n11;
        locals.var_t3__blk811_dn12 = assign17390_e15744_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign17400_e15761, assign17400_e15761_d_n3, assign17400_e15761_d_n4, assign17400_e15761_d_n5, assign17400_e15761_d_n6, assign17400_e15761_d_n7, assign17400_e15761_d_n8, assign17400_e15761_d_n9, assign17400_e15761_d_n10, assign17400_e15761_d_n11, assign17400_e15761_d_n12,) = {
    if (locals.var_guard1202 != 0.0) {
        let assign17400_e15749: f64 = (locals.var_leff / locals.var_t3__blk811);
        let (assign17400_e15758, assign17400_e15758_d_n3, assign17400_e15758_d_n4, assign17400_e15758_d_n5, assign17400_e15758_d_n6, assign17400_e15758_d_n7, assign17400_e15758_d_n8, assign17400_e15758_d_n9, assign17400_e15758_d_n10, assign17400_e15758_d_n11, assign17400_e15758_d_n12,) = {
            if (assign17400_e15749 > 1e-38) {
                let assign17400_e15754: f64 = (locals.var_leff / locals.var_t3__blk811);
                let assign17400_e15755: f64 = (assign17400_e15754).ln();
                (assign17400_e15755, ((((locals.var_leff_dn3 * locals.var_t3__blk811) - (locals.var_leff * locals.var_t3__blk811_dn3)) / (locals.var_t3__blk811 * locals.var_t3__blk811)) / assign17400_e15754), ((((locals.var_leff_dn4 * locals.var_t3__blk811) - (locals.var_leff * locals.var_t3__blk811_dn4)) / (locals.var_t3__blk811 * locals.var_t3__blk811)) / assign17400_e15754), ((((locals.var_leff_dn5 * locals.var_t3__blk811) - (locals.var_leff * locals.var_t3__blk811_dn5)) / (locals.var_t3__blk811 * locals.var_t3__blk811)) / assign17400_e15754), ((((locals.var_leff_dn6 * locals.var_t3__blk811) - (locals.var_leff * locals.var_t3__blk811_dn6)) / (locals.var_t3__blk811 * locals.var_t3__blk811)) / assign17400_e15754), ((((locals.var_leff_dn7 * locals.var_t3__blk811) - (locals.var_leff * locals.var_t3__blk811_dn7)) / (locals.var_t3__blk811 * locals.var_t3__blk811)) / assign17400_e15754), ((((locals.var_leff_dn8 * locals.var_t3__blk811) - (locals.var_leff * locals.var_t3__blk811_dn8)) / (locals.var_t3__blk811 * locals.var_t3__blk811)) / assign17400_e15754), ((((locals.var_leff_dn9 * locals.var_t3__blk811) - (locals.var_leff * locals.var_t3__blk811_dn9)) / (locals.var_t3__blk811 * locals.var_t3__blk811)) / assign17400_e15754), ((((locals.var_leff_dn10 * locals.var_t3__blk811) - (locals.var_leff * locals.var_t3__blk811_dn10)) / (locals.var_t3__blk811 * locals.var_t3__blk811)) / assign17400_e15754), ((((locals.var_leff_dn11 * locals.var_t3__blk811) - (locals.var_leff * locals.var_t3__blk811_dn11)) / (locals.var_t3__blk811 * locals.var_t3__blk811)) / assign17400_e15754), ((((locals.var_leff_dn12 * locals.var_t3__blk811) - (locals.var_leff * locals.var_t3__blk811_dn12)) / (locals.var_t3__blk811 * locals.var_t3__blk811)) / assign17400_e15754),)
            } else {
                let assign17400_e15757: f64 = (-87.49823353377374);
                (assign17400_e15757, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign17400_e15759: f64 = (locals.var_vtm * assign17400_e15758);
        (assign17400_e15759, (locals.var_vtm * assign17400_e15758_d_n3), ((locals.var_vtm_dn4 * assign17400_e15758) + (locals.var_vtm * assign17400_e15758_d_n4)), ((locals.var_vtm_dn5 * assign17400_e15758) + (locals.var_vtm * assign17400_e15758_d_n5)), ((locals.var_vtm_dn6 * assign17400_e15758) + (locals.var_vtm * assign17400_e15758_d_n6)), (locals.var_vtm * assign17400_e15758_d_n7), (locals.var_vtm * assign17400_e15758_d_n8), (locals.var_vtm * assign17400_e15758_d_n9), (locals.var_vtm * assign17400_e15758_d_n10), (locals.var_vtm * assign17400_e15758_d_n11), (locals.var_vtm * assign17400_e15758_d_n12),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign17400_e15761;
        locals.var_t4__blk812_dn3 = assign17400_e15761_d_n3;
        locals.var_t4__blk812_dn4 = assign17400_e15761_d_n4;
        locals.var_t4__blk812_dn5 = assign17400_e15761_d_n5;
        locals.var_t4__blk812_dn6 = assign17400_e15761_d_n6;
        locals.var_t4__blk812_dn7 = assign17400_e15761_d_n7;
        locals.var_t4__blk812_dn8 = assign17400_e15761_d_n8;
        locals.var_t4__blk812_dn9 = assign17400_e15761_d_n9;
        locals.var_t4__blk812_dn10 = assign17400_e15761_d_n10;
        locals.var_t4__blk812_dn11 = assign17400_e15761_d_n11;
        locals.var_t4__blk812_dn12 = assign17400_e15761_d_n12;
        locals.var_t4__blk812_rv = 0.0;

        let (assign17410_e15767, assign17410_e15767_d_n3, assign17410_e15767_d_n4, assign17410_e15767_d_n5, assign17410_e15767_d_n6, assign17410_e15767_d_n7, assign17410_e15767_d_n8, assign17410_e15767_d_n9, assign17410_e15767_d_n10, assign17410_e15767_d_n11, assign17410_e15767_d_n12,) = {
    if (locals.var_guard1202 != 0.0) {
        let assign17410_e15765: f64 = (locals.var_n_cv * locals.var_t4__blk812);
        (assign17410_e15765, ((locals.var_n_cv_dn3 * locals.var_t4__blk812) + (locals.var_n_cv * locals.var_t4__blk812_dn3)), ((locals.var_n_cv_dn4 * locals.var_t4__blk812) + (locals.var_n_cv * locals.var_t4__blk812_dn4)), ((locals.var_n_cv_dn5 * locals.var_t4__blk812) + (locals.var_n_cv * locals.var_t4__blk812_dn5)), ((locals.var_n_cv_dn6 * locals.var_t4__blk812) + (locals.var_n_cv * locals.var_t4__blk812_dn6)), ((locals.var_n_cv_dn7 * locals.var_t4__blk812) + (locals.var_n_cv * locals.var_t4__blk812_dn7)), ((locals.var_n_cv_dn8 * locals.var_t4__blk812) + (locals.var_n_cv * locals.var_t4__blk812_dn8)), ((locals.var_n_cv_dn9 * locals.var_t4__blk812) + (locals.var_n_cv * locals.var_t4__blk812_dn9)), ((locals.var_n_cv_dn10 * locals.var_t4__blk812) + (locals.var_n_cv * locals.var_t4__blk812_dn10)), ((locals.var_n_cv_dn11 * locals.var_t4__blk812) + (locals.var_n_cv * locals.var_t4__blk812_dn11)), ((locals.var_n_cv_dn12 * locals.var_t4__blk812) + (locals.var_n_cv * locals.var_t4__blk812_dn12)),)
    } else {
        (locals.var_dits_sft_cv, locals.var_dits_sft_cv_dn3, locals.var_dits_sft_cv_dn4, locals.var_dits_sft_cv_dn5, locals.var_dits_sft_cv_dn6, locals.var_dits_sft_cv_dn7, locals.var_dits_sft_cv_dn8, locals.var_dits_sft_cv_dn9, locals.var_dits_sft_cv_dn10, locals.var_dits_sft_cv_dn11, locals.var_dits_sft_cv_dn12,)
    }
};
        locals.var_dits_sft_cv = assign17410_e15767;
        locals.var_dits_sft_cv_dn3 = assign17410_e15767_d_n3;
        locals.var_dits_sft_cv_dn4 = assign17410_e15767_d_n4;
        locals.var_dits_sft_cv_dn5 = assign17410_e15767_d_n5;
        locals.var_dits_sft_cv_dn6 = assign17410_e15767_d_n6;
        locals.var_dits_sft_cv_dn7 = assign17410_e15767_d_n7;
        locals.var_dits_sft_cv_dn8 = assign17410_e15767_d_n8;
        locals.var_dits_sft_cv_dn9 = assign17410_e15767_d_n9;
        locals.var_dits_sft_cv_dn10 = assign17410_e15767_d_n10;
        locals.var_dits_sft_cv_dn11 = assign17410_e15767_d_n11;
        locals.var_dits_sft_cv_dn12 = assign17410_e15767_d_n12;
        locals.var_dits_sft_cv_rv = 0.0;

        let (assign17420_e15772, assign17420_e15772_d_n3, assign17420_e15772_d_n4, assign17420_e15772_d_n5, assign17420_e15772_d_n6, assign17420_e15772_d_n7, assign17420_e15772_d_n8, assign17420_e15772_d_n9, assign17420_e15772_d_n10, assign17420_e15772_d_n11, assign17420_e15772_d_n12,) = {
    if (locals.var_guard1202 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dits_sft_cv, locals.var_dits_sft_cv_dn3, locals.var_dits_sft_cv_dn4, locals.var_dits_sft_cv_dn5, locals.var_dits_sft_cv_dn6, locals.var_dits_sft_cv_dn7, locals.var_dits_sft_cv_dn8, locals.var_dits_sft_cv_dn9, locals.var_dits_sft_cv_dn10, locals.var_dits_sft_cv_dn11, locals.var_dits_sft_cv_dn12,)
    }
};
        locals.var_dits_sft_cv = assign17420_e15772;
        locals.var_dits_sft_cv_dn3 = assign17420_e15772_d_n3;
        locals.var_dits_sft_cv_dn4 = assign17420_e15772_d_n4;
        locals.var_dits_sft_cv_dn5 = assign17420_e15772_d_n5;
        locals.var_dits_sft_cv_dn6 = assign17420_e15772_d_n6;
        locals.var_dits_sft_cv_dn7 = assign17420_e15772_d_n7;
        locals.var_dits_sft_cv_dn8 = assign17420_e15772_d_n8;
        locals.var_dits_sft_cv_dn9 = assign17420_e15772_d_n9;
        locals.var_dits_sft_cv_dn10 = assign17420_e15772_d_n10;
        locals.var_dits_sft_cv_dn11 = assign17420_e15772_d_n11;
        locals.var_dits_sft_cv_dn12 = assign17420_e15772_d_n12;
        locals.var_dits_sft_cv_rv = 0.0;

        let assign17430_e15775: f64 = (locals.var_pparam_b4soidvt0 * locals.var_theta0_cv);
        locals.var_b4soithetavth = assign17430_e15775;
        locals.var_b4soithetavth_dn3 = ((locals.var_pparam_b4soidvt0_dn3 * locals.var_theta0_cv) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_cv_dn3));
        locals.var_b4soithetavth_dn4 = ((locals.var_pparam_b4soidvt0_dn4 * locals.var_theta0_cv) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_cv_dn4));
        locals.var_b4soithetavth_dn5 = ((locals.var_pparam_b4soidvt0_dn5 * locals.var_theta0_cv) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_cv_dn5));
        locals.var_b4soithetavth_dn6 = ((locals.var_pparam_b4soidvt0_dn6 * locals.var_theta0_cv) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_cv_dn6));
        locals.var_b4soithetavth_dn7 = ((locals.var_pparam_b4soidvt0_dn7 * locals.var_theta0_cv) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_cv_dn7));
        locals.var_b4soithetavth_dn8 = ((locals.var_pparam_b4soidvt0_dn8 * locals.var_theta0_cv) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_cv_dn8));
        locals.var_b4soithetavth_dn9 = ((locals.var_pparam_b4soidvt0_dn9 * locals.var_theta0_cv) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_cv_dn9));
        locals.var_b4soithetavth_dn10 = ((locals.var_pparam_b4soidvt0_dn10 * locals.var_theta0_cv) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_cv_dn10));
        locals.var_b4soithetavth_dn11 = ((locals.var_pparam_b4soidvt0_dn11 * locals.var_theta0_cv) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_cv_dn11));
        locals.var_b4soithetavth_dn12 = ((locals.var_pparam_b4soidvt0_dn12 * locals.var_theta0_cv) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_cv_dn12));
        locals.var_b4soithetavth_rv = 0.0;

        let assign17440_e15778: f64 = (locals.var_b4soithetavth * locals.var_v0__blk799);
        locals.var_delt_vth_cv = assign17440_e15778;
        locals.var_delt_vth_cv_dn3 = ((locals.var_b4soithetavth_dn3 * locals.var_v0__blk799) + (locals.var_b4soithetavth * locals.var_v0__blk799_dn3));
        locals.var_delt_vth_cv_dn4 = ((locals.var_b4soithetavth_dn4 * locals.var_v0__blk799) + (locals.var_b4soithetavth * locals.var_v0__blk799_dn4));
        locals.var_delt_vth_cv_dn5 = ((locals.var_b4soithetavth_dn5 * locals.var_v0__blk799) + (locals.var_b4soithetavth * locals.var_v0__blk799_dn5));
        locals.var_delt_vth_cv_dn6 = ((locals.var_b4soithetavth_dn6 * locals.var_v0__blk799) + (locals.var_b4soithetavth * locals.var_v0__blk799_dn6));
        locals.var_delt_vth_cv_dn7 = ((locals.var_b4soithetavth_dn7 * locals.var_v0__blk799) + (locals.var_b4soithetavth * locals.var_v0__blk799_dn7));
        locals.var_delt_vth_cv_dn8 = ((locals.var_b4soithetavth_dn8 * locals.var_v0__blk799) + (locals.var_b4soithetavth * locals.var_v0__blk799_dn8));
        locals.var_delt_vth_cv_dn9 = ((locals.var_b4soithetavth_dn9 * locals.var_v0__blk799) + (locals.var_b4soithetavth * locals.var_v0__blk799_dn9));
        locals.var_delt_vth_cv_dn10 = ((locals.var_b4soithetavth_dn10 * locals.var_v0__blk799) + (locals.var_b4soithetavth * locals.var_v0__blk799_dn10));
        locals.var_delt_vth_cv_dn11 = ((locals.var_b4soithetavth_dn11 * locals.var_v0__blk799) + (locals.var_b4soithetavth * locals.var_v0__blk799_dn11));
        locals.var_delt_vth_cv_dn12 = ((locals.var_b4soithetavth_dn12 * locals.var_v0__blk799) + (locals.var_b4soithetavth * locals.var_v0__blk799_dn12));
        locals.var_delt_vth_cv_rv = 0.0;

        let assign17450_e15780: f64 = (-0.5);
        let assign17450_e15782: f64 = (assign17450_e15780 * locals.var_pparam_b4soidvt1w);
        let assign17450_e15784: f64 = (assign17450_e15782 * locals.var_pparam_b4soiweff);
        let assign17450_e15786: f64 = (assign17450_e15784 * locals.var_leff);
        let assign17450_e15788: f64 = (assign17450_e15786 / locals.var_ltw_cv);
        locals.var_t0__blk808 = assign17450_e15788;
        locals.var_t0__blk808_dn3 = ((((((((assign17450_e15780 * locals.var_pparam_b4soidvt1w_dn3) * locals.var_pparam_b4soiweff) + (assign17450_e15782 * locals.var_pparam_b4soiweff_dn3)) * locals.var_leff) + (assign17450_e15784 * locals.var_leff_dn3)) * locals.var_ltw_cv) - (assign17450_e15786 * locals.var_ltw_cv_dn3)) / (locals.var_ltw_cv * locals.var_ltw_cv));
        locals.var_t0__blk808_dn4 = ((((((((assign17450_e15780 * locals.var_pparam_b4soidvt1w_dn4) * locals.var_pparam_b4soiweff) + (assign17450_e15782 * locals.var_pparam_b4soiweff_dn4)) * locals.var_leff) + (assign17450_e15784 * locals.var_leff_dn4)) * locals.var_ltw_cv) - (assign17450_e15786 * locals.var_ltw_cv_dn4)) / (locals.var_ltw_cv * locals.var_ltw_cv));
        locals.var_t0__blk808_dn5 = ((((((((assign17450_e15780 * locals.var_pparam_b4soidvt1w_dn5) * locals.var_pparam_b4soiweff) + (assign17450_e15782 * locals.var_pparam_b4soiweff_dn5)) * locals.var_leff) + (assign17450_e15784 * locals.var_leff_dn5)) * locals.var_ltw_cv) - (assign17450_e15786 * locals.var_ltw_cv_dn5)) / (locals.var_ltw_cv * locals.var_ltw_cv));
        locals.var_t0__blk808_dn6 = ((((((((assign17450_e15780 * locals.var_pparam_b4soidvt1w_dn6) * locals.var_pparam_b4soiweff) + (assign17450_e15782 * locals.var_pparam_b4soiweff_dn6)) * locals.var_leff) + (assign17450_e15784 * locals.var_leff_dn6)) * locals.var_ltw_cv) - (assign17450_e15786 * locals.var_ltw_cv_dn6)) / (locals.var_ltw_cv * locals.var_ltw_cv));
        locals.var_t0__blk808_dn7 = ((((((((assign17450_e15780 * locals.var_pparam_b4soidvt1w_dn7) * locals.var_pparam_b4soiweff) + (assign17450_e15782 * locals.var_pparam_b4soiweff_dn7)) * locals.var_leff) + (assign17450_e15784 * locals.var_leff_dn7)) * locals.var_ltw_cv) - (assign17450_e15786 * locals.var_ltw_cv_dn7)) / (locals.var_ltw_cv * locals.var_ltw_cv));
        locals.var_t0__blk808_dn8 = ((((((((assign17450_e15780 * locals.var_pparam_b4soidvt1w_dn8) * locals.var_pparam_b4soiweff) + (assign17450_e15782 * locals.var_pparam_b4soiweff_dn8)) * locals.var_leff) + (assign17450_e15784 * locals.var_leff_dn8)) * locals.var_ltw_cv) - (assign17450_e15786 * locals.var_ltw_cv_dn8)) / (locals.var_ltw_cv * locals.var_ltw_cv));
        locals.var_t0__blk808_dn9 = ((((((((assign17450_e15780 * locals.var_pparam_b4soidvt1w_dn9) * locals.var_pparam_b4soiweff) + (assign17450_e15782 * locals.var_pparam_b4soiweff_dn9)) * locals.var_leff) + (assign17450_e15784 * locals.var_leff_dn9)) * locals.var_ltw_cv) - (assign17450_e15786 * locals.var_ltw_cv_dn9)) / (locals.var_ltw_cv * locals.var_ltw_cv));
        locals.var_t0__blk808_dn10 = ((((((((assign17450_e15780 * locals.var_pparam_b4soidvt1w_dn10) * locals.var_pparam_b4soiweff) + (assign17450_e15782 * locals.var_pparam_b4soiweff_dn10)) * locals.var_leff) + (assign17450_e15784 * locals.var_leff_dn10)) * locals.var_ltw_cv) - (assign17450_e15786 * locals.var_ltw_cv_dn10)) / (locals.var_ltw_cv * locals.var_ltw_cv));
        locals.var_t0__blk808_dn11 = ((((((((assign17450_e15780 * locals.var_pparam_b4soidvt1w_dn11) * locals.var_pparam_b4soiweff) + (assign17450_e15782 * locals.var_pparam_b4soiweff_dn11)) * locals.var_leff) + (assign17450_e15784 * locals.var_leff_dn11)) * locals.var_ltw_cv) - (assign17450_e15786 * locals.var_ltw_cv_dn11)) / (locals.var_ltw_cv * locals.var_ltw_cv));
        locals.var_t0__blk808_dn12 = ((((((((assign17450_e15780 * locals.var_pparam_b4soidvt1w_dn12) * locals.var_pparam_b4soiweff) + (assign17450_e15782 * locals.var_pparam_b4soiweff_dn12)) * locals.var_leff) + (assign17450_e15784 * locals.var_leff_dn12)) * locals.var_ltw_cv) - (assign17450_e15786 * locals.var_ltw_cv_dn12)) / (locals.var_ltw_cv * locals.var_ltw_cv));
        locals.var_t0__blk808_rv = 0.0;

        let assign17460_e15791: f64 = (-100.0);
        let assign17460_e15792: f64 = if locals.var_t0__blk808 > assign17460_e15791 { 1.0 } else { 0.0 };
        locals.var_guard1204 = assign17460_e15792;
        locals.var_guard1204_rv = 0.0;

        let (assign17470_e15797, assign17470_e15797_d_n3, assign17470_e15797_d_n4, assign17470_e15797_d_n5, assign17470_e15797_d_n6, assign17470_e15797_d_n7, assign17470_e15797_d_n8, assign17470_e15797_d_n9, assign17470_e15797_d_n10, assign17470_e15797_d_n11, assign17470_e15797_d_n12,) = {
    if (locals.var_guard1204 != 0.0) {
        let assign17470_e15795: f64 = (locals.var_t0__blk808).exp();
        (assign17470_e15795, (assign17470_e15795 * locals.var_t0__blk808_dn3), (assign17470_e15795 * locals.var_t0__blk808_dn4), (assign17470_e15795 * locals.var_t0__blk808_dn5), (assign17470_e15795 * locals.var_t0__blk808_dn6), (assign17470_e15795 * locals.var_t0__blk808_dn7), (assign17470_e15795 * locals.var_t0__blk808_dn8), (assign17470_e15795 * locals.var_t0__blk808_dn9), (assign17470_e15795 * locals.var_t0__blk808_dn10), (assign17470_e15795 * locals.var_t0__blk808_dn11), (assign17470_e15795 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign17470_e15797;
        locals.var_t1__blk809_dn3 = assign17470_e15797_d_n3;
        locals.var_t1__blk809_dn4 = assign17470_e15797_d_n4;
        locals.var_t1__blk809_dn5 = assign17470_e15797_d_n5;
        locals.var_t1__blk809_dn6 = assign17470_e15797_d_n6;
        locals.var_t1__blk809_dn7 = assign17470_e15797_d_n7;
        locals.var_t1__blk809_dn8 = assign17470_e15797_d_n8;
        locals.var_t1__blk809_dn9 = assign17470_e15797_d_n9;
        locals.var_t1__blk809_dn10 = assign17470_e15797_d_n10;
        locals.var_t1__blk809_dn11 = assign17470_e15797_d_n11;
        locals.var_t1__blk809_dn12 = assign17470_e15797_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign17480_e15807, assign17480_e15807_d_n3, assign17480_e15807_d_n4, assign17480_e15807_d_n5, assign17480_e15807_d_n6, assign17480_e15807_d_n7, assign17480_e15807_d_n8, assign17480_e15807_d_n9, assign17480_e15807_d_n10, assign17480_e15807_d_n11, assign17480_e15807_d_n12,) = {
    if (locals.var_guard1204 != 0.0) {
        let assign17480_e15803: f64 = (2.0 * locals.var_t1__blk809);
        let assign17480_e15804: f64 = (1.0 + assign17480_e15803);
        let assign17480_e15805: f64 = (locals.var_t1__blk809 * assign17480_e15804);
        (assign17480_e15805, ((locals.var_t1__blk809_dn3 * assign17480_e15804) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn3))), ((locals.var_t1__blk809_dn4 * assign17480_e15804) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn4))), ((locals.var_t1__blk809_dn5 * assign17480_e15804) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn5))), ((locals.var_t1__blk809_dn6 * assign17480_e15804) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn6))), ((locals.var_t1__blk809_dn7 * assign17480_e15804) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn7))), ((locals.var_t1__blk809_dn8 * assign17480_e15804) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn8))), ((locals.var_t1__blk809_dn9 * assign17480_e15804) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn9))), ((locals.var_t1__blk809_dn10 * assign17480_e15804) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn10))), ((locals.var_t1__blk809_dn11 * assign17480_e15804) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn11))), ((locals.var_t1__blk809_dn12 * assign17480_e15804) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn12))),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign17480_e15807;
        locals.var_t2__blk810_dn3 = assign17480_e15807_d_n3;
        locals.var_t2__blk810_dn4 = assign17480_e15807_d_n4;
        locals.var_t2__blk810_dn5 = assign17480_e15807_d_n5;
        locals.var_t2__blk810_dn6 = assign17480_e15807_d_n6;
        locals.var_t2__blk810_dn7 = assign17480_e15807_d_n7;
        locals.var_t2__blk810_dn8 = assign17480_e15807_d_n8;
        locals.var_t2__blk810_dn9 = assign17480_e15807_d_n9;
        locals.var_t2__blk810_dn10 = assign17480_e15807_d_n10;
        locals.var_t2__blk810_dn11 = assign17480_e15807_d_n11;
        locals.var_t2__blk810_dn12 = assign17480_e15807_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign17490_e15812, assign17490_e15812_d_n3, assign17490_e15812_d_n4, assign17490_e15812_d_n5, assign17490_e15812_d_n6, assign17490_e15812_d_n7, assign17490_e15812_d_n8, assign17490_e15812_d_n9, assign17490_e15812_d_n10, assign17490_e15812_d_n11, assign17490_e15812_d_n12,) = {
    if (locals.var_guard1204 == 0.0) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign17490_e15812;
        locals.var_t1__blk809_dn3 = assign17490_e15812_d_n3;
        locals.var_t1__blk809_dn4 = assign17490_e15812_d_n4;
        locals.var_t1__blk809_dn5 = assign17490_e15812_d_n5;
        locals.var_t1__blk809_dn6 = assign17490_e15812_d_n6;
        locals.var_t1__blk809_dn7 = assign17490_e15812_d_n7;
        locals.var_t1__blk809_dn8 = assign17490_e15812_d_n8;
        locals.var_t1__blk809_dn9 = assign17490_e15812_d_n9;
        locals.var_t1__blk809_dn10 = assign17490_e15812_d_n10;
        locals.var_t1__blk809_dn11 = assign17490_e15812_d_n11;
        locals.var_t1__blk809_dn12 = assign17490_e15812_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign17500_e15823, assign17500_e15823_d_n3, assign17500_e15823_d_n4, assign17500_e15823_d_n5, assign17500_e15823_d_n6, assign17500_e15823_d_n7, assign17500_e15823_d_n8, assign17500_e15823_d_n9, assign17500_e15823_d_n10, assign17500_e15823_d_n11, assign17500_e15823_d_n12,) = {
    if (locals.var_guard1204 == 0.0) {
        let assign17500_e15819: f64 = (2.0 * locals.var_t1__blk809);
        let assign17500_e15820: f64 = (1.0 + assign17500_e15819);
        let assign17500_e15821: f64 = (locals.var_t1__blk809 * assign17500_e15820);
        (assign17500_e15821, ((locals.var_t1__blk809_dn3 * assign17500_e15820) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn3))), ((locals.var_t1__blk809_dn4 * assign17500_e15820) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn4))), ((locals.var_t1__blk809_dn5 * assign17500_e15820) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn5))), ((locals.var_t1__blk809_dn6 * assign17500_e15820) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn6))), ((locals.var_t1__blk809_dn7 * assign17500_e15820) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn7))), ((locals.var_t1__blk809_dn8 * assign17500_e15820) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn8))), ((locals.var_t1__blk809_dn9 * assign17500_e15820) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn9))), ((locals.var_t1__blk809_dn10 * assign17500_e15820) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn10))), ((locals.var_t1__blk809_dn11 * assign17500_e15820) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn11))), ((locals.var_t1__blk809_dn12 * assign17500_e15820) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn12))),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign17500_e15823;
        locals.var_t2__blk810_dn3 = assign17500_e15823_d_n3;
        locals.var_t2__blk810_dn4 = assign17500_e15823_d_n4;
        locals.var_t2__blk810_dn5 = assign17500_e15823_d_n5;
        locals.var_t2__blk810_dn6 = assign17500_e15823_d_n6;
        locals.var_t2__blk810_dn7 = assign17500_e15823_d_n7;
        locals.var_t2__blk810_dn8 = assign17500_e15823_d_n8;
        locals.var_t2__blk810_dn9 = assign17500_e15823_d_n9;
        locals.var_t2__blk810_dn10 = assign17500_e15823_d_n10;
        locals.var_t2__blk810_dn11 = assign17500_e15823_d_n11;
        locals.var_t2__blk810_dn12 = assign17500_e15823_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let assign17510_e15826: f64 = (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810);
        locals.var_t0__blk808 = assign17510_e15826;
        locals.var_t0__blk808_dn3 = ((locals.var_pparam_b4soidvt0w_dn3 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn3));
        locals.var_t0__blk808_dn4 = ((locals.var_pparam_b4soidvt0w_dn4 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn4));
        locals.var_t0__blk808_dn5 = ((locals.var_pparam_b4soidvt0w_dn5 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn5));
        locals.var_t0__blk808_dn6 = ((locals.var_pparam_b4soidvt0w_dn6 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn6));
        locals.var_t0__blk808_dn7 = ((locals.var_pparam_b4soidvt0w_dn7 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn7));
        locals.var_t0__blk808_dn8 = ((locals.var_pparam_b4soidvt0w_dn8 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn8));
        locals.var_t0__blk808_dn9 = ((locals.var_pparam_b4soidvt0w_dn9 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn9));
        locals.var_t0__blk808_dn10 = ((locals.var_pparam_b4soidvt0w_dn10 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn10));
        locals.var_t0__blk808_dn11 = ((locals.var_pparam_b4soidvt0w_dn11 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn11));
        locals.var_t0__blk808_dn12 = ((locals.var_pparam_b4soidvt0w_dn12 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn12));
        locals.var_t0__blk808_rv = 0.0;

        let assign17520_e15829: f64 = (locals.var_t0__blk808 * locals.var_v0__blk799);
        locals.var_deltvthw_cv = assign17520_e15829;
        locals.var_deltvthw_cv_dn3 = ((locals.var_t0__blk808_dn3 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn3));
        locals.var_deltvthw_cv_dn4 = ((locals.var_t0__blk808_dn4 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn4));
        locals.var_deltvthw_cv_dn5 = ((locals.var_t0__blk808_dn5 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn5));
        locals.var_deltvthw_cv_dn6 = ((locals.var_t0__blk808_dn6 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn6));
        locals.var_deltvthw_cv_dn7 = ((locals.var_t0__blk808_dn7 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn7));
        locals.var_deltvthw_cv_dn8 = ((locals.var_t0__blk808_dn8 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn8));
        locals.var_deltvthw_cv_dn9 = ((locals.var_t0__blk808_dn9 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn9));
        locals.var_deltvthw_cv_dn10 = ((locals.var_t0__blk808_dn10 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn10));
        locals.var_deltvthw_cv_dn11 = ((locals.var_t0__blk808_dn11 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn11));
        locals.var_deltvthw_cv_dn12 = ((locals.var_t0__blk808_dn12 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn12));
        locals.var_deltvthw_cv_rv = 0.0;

        let assign17530_e15833: f64 = (locals.var_pparam_b4soilpe0 / locals.var_leff);
        let assign17530_e15834: f64 = (1.0 + assign17530_e15833);
        let assign17530_e15835: f64 = (assign17530_e15834).sqrt();
        locals.var_t0__blk808 = assign17530_e15835;
        locals.var_t0__blk808_dn3 = ((((locals.var_pparam_b4soilpe0_dn3 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn3)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17530_e15835));
        locals.var_t0__blk808_dn4 = ((((locals.var_pparam_b4soilpe0_dn4 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn4)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17530_e15835));
        locals.var_t0__blk808_dn5 = ((((locals.var_pparam_b4soilpe0_dn5 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn5)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17530_e15835));
        locals.var_t0__blk808_dn6 = ((((locals.var_pparam_b4soilpe0_dn6 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn6)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17530_e15835));
        locals.var_t0__blk808_dn7 = ((((locals.var_pparam_b4soilpe0_dn7 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn7)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17530_e15835));
        locals.var_t0__blk808_dn8 = ((((locals.var_pparam_b4soilpe0_dn8 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn8)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17530_e15835));
        locals.var_t0__blk808_dn9 = ((((locals.var_pparam_b4soilpe0_dn9 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn9)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17530_e15835));
        locals.var_t0__blk808_dn10 = ((((locals.var_pparam_b4soilpe0_dn10 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn10)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17530_e15835));
        locals.var_t0__blk808_dn11 = ((((locals.var_pparam_b4soilpe0_dn11 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn11)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17530_e15835));
        locals.var_t0__blk808_dn12 = ((((locals.var_pparam_b4soilpe0_dn12 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn12)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17530_e15835));
        locals.var_t0__blk808_rv = 0.0;

        let assign17540_e15839: f64 = (locals.var_pparam_b4soikt1l / locals.var_leff);
        let assign17540_e15840: f64 = (locals.var_pparam_b4soikt1 + assign17540_e15839);
        let assign17540_e15843: f64 = (locals.var_pparam_b4soikt2 * locals.var_vbseff_cv);
        let assign17540_e15844: f64 = (assign17540_e15840 + assign17540_e15843);
        locals.var_t1__blk809 = assign17540_e15844;
        locals.var_t1__blk809_dn3 = ((locals.var_pparam_b4soikt1_dn3 + (((locals.var_pparam_b4soikt1l_dn3 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn3)) / (locals.var_leff * locals.var_leff))) + ((locals.var_pparam_b4soikt2_dn3 * locals.var_vbseff_cv) + (locals.var_pparam_b4soikt2 * locals.var_vbseff_cv_dn3)));
        locals.var_t1__blk809_dn4 = ((locals.var_pparam_b4soikt1_dn4 + (((locals.var_pparam_b4soikt1l_dn4 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn4)) / (locals.var_leff * locals.var_leff))) + ((locals.var_pparam_b4soikt2_dn4 * locals.var_vbseff_cv) + (locals.var_pparam_b4soikt2 * locals.var_vbseff_cv_dn4)));
        locals.var_t1__blk809_dn5 = ((locals.var_pparam_b4soikt1_dn5 + (((locals.var_pparam_b4soikt1l_dn5 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn5)) / (locals.var_leff * locals.var_leff))) + ((locals.var_pparam_b4soikt2_dn5 * locals.var_vbseff_cv) + (locals.var_pparam_b4soikt2 * locals.var_vbseff_cv_dn5)));
        locals.var_t1__blk809_dn6 = ((locals.var_pparam_b4soikt1_dn6 + (((locals.var_pparam_b4soikt1l_dn6 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn6)) / (locals.var_leff * locals.var_leff))) + ((locals.var_pparam_b4soikt2_dn6 * locals.var_vbseff_cv) + (locals.var_pparam_b4soikt2 * locals.var_vbseff_cv_dn6)));
        locals.var_t1__blk809_dn7 = ((locals.var_pparam_b4soikt1_dn7 + (((locals.var_pparam_b4soikt1l_dn7 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn7)) / (locals.var_leff * locals.var_leff))) + ((locals.var_pparam_b4soikt2_dn7 * locals.var_vbseff_cv) + (locals.var_pparam_b4soikt2 * locals.var_vbseff_cv_dn7)));
        locals.var_t1__blk809_dn8 = ((locals.var_pparam_b4soikt1_dn8 + (((locals.var_pparam_b4soikt1l_dn8 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn8)) / (locals.var_leff * locals.var_leff))) + ((locals.var_pparam_b4soikt2_dn8 * locals.var_vbseff_cv) + (locals.var_pparam_b4soikt2 * locals.var_vbseff_cv_dn8)));
        locals.var_t1__blk809_dn9 = ((locals.var_pparam_b4soikt1_dn9 + (((locals.var_pparam_b4soikt1l_dn9 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn9)) / (locals.var_leff * locals.var_leff))) + ((locals.var_pparam_b4soikt2_dn9 * locals.var_vbseff_cv) + (locals.var_pparam_b4soikt2 * locals.var_vbseff_cv_dn9)));
        locals.var_t1__blk809_dn10 = ((locals.var_pparam_b4soikt1_dn10 + (((locals.var_pparam_b4soikt1l_dn10 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn10)) / (locals.var_leff * locals.var_leff))) + ((locals.var_pparam_b4soikt2_dn10 * locals.var_vbseff_cv) + (locals.var_pparam_b4soikt2 * locals.var_vbseff_cv_dn10)));
        locals.var_t1__blk809_dn11 = ((locals.var_pparam_b4soikt1_dn11 + (((locals.var_pparam_b4soikt1l_dn11 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn11)) / (locals.var_leff * locals.var_leff))) + ((locals.var_pparam_b4soikt2_dn11 * locals.var_vbseff_cv) + (locals.var_pparam_b4soikt2 * locals.var_vbseff_cv_dn11)));
        locals.var_t1__blk809_dn12 = ((locals.var_pparam_b4soikt1_dn12 + (((locals.var_pparam_b4soikt1l_dn12 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn12)) / (locals.var_leff * locals.var_leff))) + ((locals.var_pparam_b4soikt2_dn12 * locals.var_vbseff_cv) + (locals.var_pparam_b4soikt2 * locals.var_vbseff_cv_dn12)));
        locals.var_t1__blk809_rv = 0.0;

        let assign17550_e15848: f64 = (locals.var_t0__blk808 - 1.0);
        let assign17550_e15849: f64 = (locals.var_here_b4soik1ox * assign17550_e15848);
        let assign17550_e15851: f64 = (assign17550_e15849 * locals.var_sqrtphi);
        let assign17550_e15854: f64 = (locals.var_t1__blk809 * locals.var_trm1);
        let assign17550_e15855: f64 = (assign17550_e15851 + assign17550_e15854);
        locals.var_deltvthtemp_cv = assign17550_e15855;
        locals.var_deltvthtemp_cv_dn3 = (((((locals.var_here_b4soik1ox_dn3 * assign17550_e15848) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn3)) * locals.var_sqrtphi) + (assign17550_e15849 * locals.var_sqrtphi_dn3)) + (locals.var_t1__blk809_dn3 * locals.var_trm1));
        locals.var_deltvthtemp_cv_dn4 = (((((locals.var_here_b4soik1ox_dn4 * assign17550_e15848) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn4)) * locals.var_sqrtphi) + (assign17550_e15849 * locals.var_sqrtphi_dn4)) + ((locals.var_t1__blk809_dn4 * locals.var_trm1) + (locals.var_t1__blk809 * locals.var_trm1_dn4)));
        locals.var_deltvthtemp_cv_dn5 = (((((locals.var_here_b4soik1ox_dn5 * assign17550_e15848) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn5)) * locals.var_sqrtphi) + (assign17550_e15849 * locals.var_sqrtphi_dn5)) + ((locals.var_t1__blk809_dn5 * locals.var_trm1) + (locals.var_t1__blk809 * locals.var_trm1_dn5)));
        locals.var_deltvthtemp_cv_dn6 = (((((locals.var_here_b4soik1ox_dn6 * assign17550_e15848) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn6)) * locals.var_sqrtphi) + (assign17550_e15849 * locals.var_sqrtphi_dn6)) + ((locals.var_t1__blk809_dn6 * locals.var_trm1) + (locals.var_t1__blk809 * locals.var_trm1_dn6)));
        locals.var_deltvthtemp_cv_dn7 = (((((locals.var_here_b4soik1ox_dn7 * assign17550_e15848) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn7)) * locals.var_sqrtphi) + (assign17550_e15849 * locals.var_sqrtphi_dn7)) + (locals.var_t1__blk809_dn7 * locals.var_trm1));
        locals.var_deltvthtemp_cv_dn8 = (((((locals.var_here_b4soik1ox_dn8 * assign17550_e15848) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn8)) * locals.var_sqrtphi) + (assign17550_e15849 * locals.var_sqrtphi_dn8)) + (locals.var_t1__blk809_dn8 * locals.var_trm1));
        locals.var_deltvthtemp_cv_dn9 = (((((locals.var_here_b4soik1ox_dn9 * assign17550_e15848) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn9)) * locals.var_sqrtphi) + (assign17550_e15849 * locals.var_sqrtphi_dn9)) + (locals.var_t1__blk809_dn9 * locals.var_trm1));
        locals.var_deltvthtemp_cv_dn10 = (((((locals.var_here_b4soik1ox_dn10 * assign17550_e15848) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn10)) * locals.var_sqrtphi) + (assign17550_e15849 * locals.var_sqrtphi_dn10)) + (locals.var_t1__blk809_dn10 * locals.var_trm1));
        locals.var_deltvthtemp_cv_dn11 = (((((locals.var_here_b4soik1ox_dn11 * assign17550_e15848) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn11)) * locals.var_sqrtphi) + (assign17550_e15849 * locals.var_sqrtphi_dn11)) + (locals.var_t1__blk809_dn11 * locals.var_trm1));
        locals.var_deltvthtemp_cv_dn12 = (((((locals.var_here_b4soik1ox_dn12 * assign17550_e15848) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn12)) * locals.var_sqrtphi) + (assign17550_e15849 * locals.var_sqrtphi_dn12)) + (locals.var_t1__blk809_dn12 * locals.var_trm1));
        locals.var_deltvthtemp_cv_rv = 0.0;

        let assign17560_e15858: f64 = (locals.var_toxe * locals.var_phi);
        let assign17560_e15861: f64 = (locals.var_pparam_b4soiweff + locals.var_pparam_b4soiw0);
        let assign17560_e15862: f64 = (assign17560_e15858 / assign17560_e15861);
        locals.var_tmp2_cv = assign17560_e15862;
        locals.var_tmp2_cv_dn3 = ((((locals.var_toxe * locals.var_phi_dn3) * assign17560_e15861) - (assign17560_e15858 * (locals.var_pparam_b4soiweff_dn3 + locals.var_pparam_b4soiw0_dn3))) / (assign17560_e15861 * assign17560_e15861));
        locals.var_tmp2_cv_dn4 = ((((locals.var_toxe * locals.var_phi_dn4) * assign17560_e15861) - (assign17560_e15858 * (locals.var_pparam_b4soiweff_dn4 + locals.var_pparam_b4soiw0_dn4))) / (assign17560_e15861 * assign17560_e15861));
        locals.var_tmp2_cv_dn5 = ((((locals.var_toxe * locals.var_phi_dn5) * assign17560_e15861) - (assign17560_e15858 * (locals.var_pparam_b4soiweff_dn5 + locals.var_pparam_b4soiw0_dn5))) / (assign17560_e15861 * assign17560_e15861));
        locals.var_tmp2_cv_dn6 = ((((locals.var_toxe * locals.var_phi_dn6) * assign17560_e15861) - (assign17560_e15858 * (locals.var_pparam_b4soiweff_dn6 + locals.var_pparam_b4soiw0_dn6))) / (assign17560_e15861 * assign17560_e15861));
        locals.var_tmp2_cv_dn7 = ((((locals.var_toxe * locals.var_phi_dn7) * assign17560_e15861) - (assign17560_e15858 * (locals.var_pparam_b4soiweff_dn7 + locals.var_pparam_b4soiw0_dn7))) / (assign17560_e15861 * assign17560_e15861));
        locals.var_tmp2_cv_dn8 = ((((locals.var_toxe * locals.var_phi_dn8) * assign17560_e15861) - (assign17560_e15858 * (locals.var_pparam_b4soiweff_dn8 + locals.var_pparam_b4soiw0_dn8))) / (assign17560_e15861 * assign17560_e15861));
        locals.var_tmp2_cv_dn9 = ((((locals.var_toxe * locals.var_phi_dn9) * assign17560_e15861) - (assign17560_e15858 * (locals.var_pparam_b4soiweff_dn9 + locals.var_pparam_b4soiw0_dn9))) / (assign17560_e15861 * assign17560_e15861));
        locals.var_tmp2_cv_dn10 = ((((locals.var_toxe * locals.var_phi_dn10) * assign17560_e15861) - (assign17560_e15858 * (locals.var_pparam_b4soiweff_dn10 + locals.var_pparam_b4soiw0_dn10))) / (assign17560_e15861 * assign17560_e15861));
        locals.var_tmp2_cv_dn11 = ((((locals.var_toxe * locals.var_phi_dn11) * assign17560_e15861) - (assign17560_e15858 * (locals.var_pparam_b4soiweff_dn11 + locals.var_pparam_b4soiw0_dn11))) / (assign17560_e15861 * assign17560_e15861));
        locals.var_tmp2_cv_dn12 = ((((locals.var_toxe * locals.var_phi_dn12) * assign17560_e15861) - (assign17560_e15858 * (locals.var_pparam_b4soiweff_dn12 + locals.var_pparam_b4soiw0_dn12))) / (assign17560_e15861 * assign17560_e15861));
        locals.var_tmp2_cv_rv = 0.0;

        let assign17570_e15866: f64 = (locals.var_pparam_b4soietabcv * locals.var_vbseff_cv);
        let assign17570_e15867: f64 = (locals.var_here_b4soieta0cv + assign17570_e15866);
        locals.var_t3__blk811 = assign17570_e15867;
        locals.var_t3__blk811_dn3 = (locals.var_here_b4soieta0cv_dn3 + ((locals.var_pparam_b4soietabcv_dn3 * locals.var_vbseff_cv) + (locals.var_pparam_b4soietabcv * locals.var_vbseff_cv_dn3)));
        locals.var_t3__blk811_dn4 = (locals.var_here_b4soieta0cv_dn4 + ((locals.var_pparam_b4soietabcv_dn4 * locals.var_vbseff_cv) + (locals.var_pparam_b4soietabcv * locals.var_vbseff_cv_dn4)));
        locals.var_t3__blk811_dn5 = (locals.var_here_b4soieta0cv_dn5 + ((locals.var_pparam_b4soietabcv_dn5 * locals.var_vbseff_cv) + (locals.var_pparam_b4soietabcv * locals.var_vbseff_cv_dn5)));
        locals.var_t3__blk811_dn6 = (locals.var_here_b4soieta0cv_dn6 + ((locals.var_pparam_b4soietabcv_dn6 * locals.var_vbseff_cv) + (locals.var_pparam_b4soietabcv * locals.var_vbseff_cv_dn6)));
        locals.var_t3__blk811_dn7 = (locals.var_here_b4soieta0cv_dn7 + ((locals.var_pparam_b4soietabcv_dn7 * locals.var_vbseff_cv) + (locals.var_pparam_b4soietabcv * locals.var_vbseff_cv_dn7)));
        locals.var_t3__blk811_dn8 = (locals.var_here_b4soieta0cv_dn8 + ((locals.var_pparam_b4soietabcv_dn8 * locals.var_vbseff_cv) + (locals.var_pparam_b4soietabcv * locals.var_vbseff_cv_dn8)));
        locals.var_t3__blk811_dn9 = (locals.var_here_b4soieta0cv_dn9 + ((locals.var_pparam_b4soietabcv_dn9 * locals.var_vbseff_cv) + (locals.var_pparam_b4soietabcv * locals.var_vbseff_cv_dn9)));
        locals.var_t3__blk811_dn10 = (locals.var_here_b4soieta0cv_dn10 + ((locals.var_pparam_b4soietabcv_dn10 * locals.var_vbseff_cv) + (locals.var_pparam_b4soietabcv * locals.var_vbseff_cv_dn10)));
        locals.var_t3__blk811_dn11 = (locals.var_here_b4soieta0cv_dn11 + ((locals.var_pparam_b4soietabcv_dn11 * locals.var_vbseff_cv) + (locals.var_pparam_b4soietabcv * locals.var_vbseff_cv_dn11)));
        locals.var_t3__blk811_dn12 = (locals.var_here_b4soieta0cv_dn12 + ((locals.var_pparam_b4soietabcv_dn12 * locals.var_vbseff_cv) + (locals.var_pparam_b4soietabcv * locals.var_vbseff_cv_dn12)));
        locals.var_t3__blk811_rv = 0.0;

        let assign17580_e15870: f64 = if locals.var_t3__blk811 < 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard1205 = assign17580_e15870;
        locals.var_guard1205_rv = 0.0;

        let (assign17590_e15880, assign17590_e15880_d_n3, assign17590_e15880_d_n4, assign17590_e15880_d_n5, assign17590_e15880_d_n6, assign17590_e15880_d_n7, assign17590_e15880_d_n8, assign17590_e15880_d_n9, assign17590_e15880_d_n10, assign17590_e15880_d_n11, assign17590_e15880_d_n12,) = {
    if (locals.var_guard1205 != 0.0) {
        let assign17590_e15876: f64 = (20000.0 * locals.var_t3__blk811);
        let assign17590_e15877: f64 = (3.0 - assign17590_e15876);
        let assign17590_e15878: f64 = (1.0 / assign17590_e15877);
        (assign17590_e15878, (-((-(20000.0 * locals.var_t3__blk811_dn3)) / (assign17590_e15877 * assign17590_e15877))), (-((-(20000.0 * locals.var_t3__blk811_dn4)) / (assign17590_e15877 * assign17590_e15877))), (-((-(20000.0 * locals.var_t3__blk811_dn5)) / (assign17590_e15877 * assign17590_e15877))), (-((-(20000.0 * locals.var_t3__blk811_dn6)) / (assign17590_e15877 * assign17590_e15877))), (-((-(20000.0 * locals.var_t3__blk811_dn7)) / (assign17590_e15877 * assign17590_e15877))), (-((-(20000.0 * locals.var_t3__blk811_dn8)) / (assign17590_e15877 * assign17590_e15877))), (-((-(20000.0 * locals.var_t3__blk811_dn9)) / (assign17590_e15877 * assign17590_e15877))), (-((-(20000.0 * locals.var_t3__blk811_dn10)) / (assign17590_e15877 * assign17590_e15877))), (-((-(20000.0 * locals.var_t3__blk811_dn11)) / (assign17590_e15877 * assign17590_e15877))), (-((-(20000.0 * locals.var_t3__blk811_dn12)) / (assign17590_e15877 * assign17590_e15877))),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign17590_e15880;
        locals.var_t9_dn3 = assign17590_e15880_d_n3;
        locals.var_t9_dn4 = assign17590_e15880_d_n4;
        locals.var_t9_dn5 = assign17590_e15880_d_n5;
        locals.var_t9_dn6 = assign17590_e15880_d_n6;
        locals.var_t9_dn7 = assign17590_e15880_d_n7;
        locals.var_t9_dn8 = assign17590_e15880_d_n8;
        locals.var_t9_dn9 = assign17590_e15880_d_n9;
        locals.var_t9_dn10 = assign17590_e15880_d_n10;
        locals.var_t9_dn11 = assign17590_e15880_d_n11;
        locals.var_t9_dn12 = assign17590_e15880_d_n12;
        locals.var_t9_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_52(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17600_e15888, assign17600_e15888_d_n3, assign17600_e15888_d_n4, assign17600_e15888_d_n5, assign17600_e15888_d_n6, assign17600_e15888_d_n7, assign17600_e15888_d_n8, assign17600_e15888_d_n9, assign17600_e15888_d_n10, assign17600_e15888_d_n11, assign17600_e15888_d_n12,) = {
    if (locals.var_guard1205 != 0.0) {
        let assign17600_e15884: f64 = (0.0002 - locals.var_t3__blk811);
        let assign17600_e15886: f64 = (assign17600_e15884 * locals.var_t9);
        (assign17600_e15886, (((-locals.var_t3__blk811_dn3) * locals.var_t9) + (assign17600_e15884 * locals.var_t9_dn3)), (((-locals.var_t3__blk811_dn4) * locals.var_t9) + (assign17600_e15884 * locals.var_t9_dn4)), (((-locals.var_t3__blk811_dn5) * locals.var_t9) + (assign17600_e15884 * locals.var_t9_dn5)), (((-locals.var_t3__blk811_dn6) * locals.var_t9) + (assign17600_e15884 * locals.var_t9_dn6)), (((-locals.var_t3__blk811_dn7) * locals.var_t9) + (assign17600_e15884 * locals.var_t9_dn7)), (((-locals.var_t3__blk811_dn8) * locals.var_t9) + (assign17600_e15884 * locals.var_t9_dn8)), (((-locals.var_t3__blk811_dn9) * locals.var_t9) + (assign17600_e15884 * locals.var_t9_dn9)), (((-locals.var_t3__blk811_dn10) * locals.var_t9) + (assign17600_e15884 * locals.var_t9_dn10)), (((-locals.var_t3__blk811_dn11) * locals.var_t9) + (assign17600_e15884 * locals.var_t9_dn11)), (((-locals.var_t3__blk811_dn12) * locals.var_t9) + (assign17600_e15884 * locals.var_t9_dn12)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign17600_e15888;
        locals.var_t3__blk811_dn3 = assign17600_e15888_d_n3;
        locals.var_t3__blk811_dn4 = assign17600_e15888_d_n4;
        locals.var_t3__blk811_dn5 = assign17600_e15888_d_n5;
        locals.var_t3__blk811_dn6 = assign17600_e15888_d_n6;
        locals.var_t3__blk811_dn7 = assign17600_e15888_d_n7;
        locals.var_t3__blk811_dn8 = assign17600_e15888_d_n8;
        locals.var_t3__blk811_dn9 = assign17600_e15888_d_n9;
        locals.var_t3__blk811_dn10 = assign17600_e15888_d_n10;
        locals.var_t3__blk811_dn11 = assign17600_e15888_d_n11;
        locals.var_t3__blk811_dn12 = assign17600_e15888_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let assign17610_e15891: f64 = (locals.var_t3__blk811 * locals.var_theta0vb0);
        let assign17610_e15893: f64 = (assign17610_e15891 * locals.var_vds_1);
        locals.var_dibl_sft_cv = assign17610_e15893;
        locals.var_dibl_sft_cv_dn3 = (((locals.var_t3__blk811_dn3 * locals.var_theta0vb0) + (locals.var_t3__blk811 * locals.var_theta0vb0_dn3)) * locals.var_vds_1);
        locals.var_dibl_sft_cv_dn4 = (((locals.var_t3__blk811_dn4 * locals.var_theta0vb0) + (locals.var_t3__blk811 * locals.var_theta0vb0_dn4)) * locals.var_vds_1);
        locals.var_dibl_sft_cv_dn5 = (((locals.var_t3__blk811_dn5 * locals.var_theta0vb0) + (locals.var_t3__blk811 * locals.var_theta0vb0_dn5)) * locals.var_vds_1);
        locals.var_dibl_sft_cv_dn6 = (((locals.var_t3__blk811_dn6 * locals.var_theta0vb0) + (locals.var_t3__blk811 * locals.var_theta0vb0_dn6)) * locals.var_vds_1);
        locals.var_dibl_sft_cv_dn7 = ((((locals.var_t3__blk811_dn7 * locals.var_theta0vb0) + (locals.var_t3__blk811 * locals.var_theta0vb0_dn7)) * locals.var_vds_1) + (assign17610_e15891 * locals.var_vds_1_dn7));
        locals.var_dibl_sft_cv_dn8 = ((((locals.var_t3__blk811_dn8 * locals.var_theta0vb0) + (locals.var_t3__blk811 * locals.var_theta0vb0_dn8)) * locals.var_vds_1) + (assign17610_e15891 * locals.var_vds_1_dn8));
        locals.var_dibl_sft_cv_dn9 = (((locals.var_t3__blk811_dn9 * locals.var_theta0vb0) + (locals.var_t3__blk811 * locals.var_theta0vb0_dn9)) * locals.var_vds_1);
        locals.var_dibl_sft_cv_dn10 = (((locals.var_t3__blk811_dn10 * locals.var_theta0vb0) + (locals.var_t3__blk811 * locals.var_theta0vb0_dn10)) * locals.var_vds_1);
        locals.var_dibl_sft_cv_dn11 = (((locals.var_t3__blk811_dn11 * locals.var_theta0vb0) + (locals.var_t3__blk811 * locals.var_theta0vb0_dn11)) * locals.var_vds_1);
        locals.var_dibl_sft_cv_dn12 = (((locals.var_t3__blk811_dn12 * locals.var_theta0vb0) + (locals.var_t3__blk811 * locals.var_theta0vb0_dn12)) * locals.var_vds_1);
        locals.var_dibl_sft_cv_rv = 0.0;

        let assign17620_e15897: f64 = (locals.var_pparam_b4soilpeb / locals.var_leff);
        let assign17620_e15898: f64 = (1.0 + assign17620_e15897);
        let assign17620_e15899: f64 = (assign17620_e15898).sqrt();
        locals.var_lpe_vb__blk1054 = assign17620_e15899;
        locals.var_lpe_vb__blk1054_dn3 = ((((locals.var_pparam_b4soilpeb_dn3 * locals.var_leff) - (locals.var_pparam_b4soilpeb * locals.var_leff_dn3)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17620_e15899));
        locals.var_lpe_vb__blk1054_dn4 = ((((locals.var_pparam_b4soilpeb_dn4 * locals.var_leff) - (locals.var_pparam_b4soilpeb * locals.var_leff_dn4)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17620_e15899));
        locals.var_lpe_vb__blk1054_dn5 = ((((locals.var_pparam_b4soilpeb_dn5 * locals.var_leff) - (locals.var_pparam_b4soilpeb * locals.var_leff_dn5)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17620_e15899));
        locals.var_lpe_vb__blk1054_dn6 = ((((locals.var_pparam_b4soilpeb_dn6 * locals.var_leff) - (locals.var_pparam_b4soilpeb * locals.var_leff_dn6)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17620_e15899));
        locals.var_lpe_vb__blk1054_dn7 = ((((locals.var_pparam_b4soilpeb_dn7 * locals.var_leff) - (locals.var_pparam_b4soilpeb * locals.var_leff_dn7)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17620_e15899));
        locals.var_lpe_vb__blk1054_dn8 = ((((locals.var_pparam_b4soilpeb_dn8 * locals.var_leff) - (locals.var_pparam_b4soilpeb * locals.var_leff_dn8)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17620_e15899));
        locals.var_lpe_vb__blk1054_dn9 = ((((locals.var_pparam_b4soilpeb_dn9 * locals.var_leff) - (locals.var_pparam_b4soilpeb * locals.var_leff_dn9)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17620_e15899));
        locals.var_lpe_vb__blk1054_dn10 = ((((locals.var_pparam_b4soilpeb_dn10 * locals.var_leff) - (locals.var_pparam_b4soilpeb * locals.var_leff_dn10)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17620_e15899));
        locals.var_lpe_vb__blk1054_dn11 = ((((locals.var_pparam_b4soilpeb_dn11 * locals.var_leff) - (locals.var_pparam_b4soilpeb * locals.var_leff_dn11)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17620_e15899));
        locals.var_lpe_vb__blk1054_dn12 = ((((locals.var_pparam_b4soilpeb_dn12 * locals.var_leff) - (locals.var_pparam_b4soilpeb * locals.var_leff_dn12)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17620_e15899));
        locals.var_lpe_vb__blk1054_rv = 0.0;

        let assign17630_e15902: f64 = (2.2361 / locals.var_sqrtphi);
        locals.var_t9 = assign17630_e15902;
        locals.var_t9_dn3 = (-((2.2361 * locals.var_sqrtphi_dn3) / (locals.var_sqrtphi * locals.var_sqrtphi)));
        locals.var_t9_dn4 = (-((2.2361 * locals.var_sqrtphi_dn4) / (locals.var_sqrtphi * locals.var_sqrtphi)));
        locals.var_t9_dn5 = (-((2.2361 * locals.var_sqrtphi_dn5) / (locals.var_sqrtphi * locals.var_sqrtphi)));
        locals.var_t9_dn6 = (-((2.2361 * locals.var_sqrtphi_dn6) / (locals.var_sqrtphi * locals.var_sqrtphi)));
        locals.var_t9_dn7 = (-((2.2361 * locals.var_sqrtphi_dn7) / (locals.var_sqrtphi * locals.var_sqrtphi)));
        locals.var_t9_dn8 = (-((2.2361 * locals.var_sqrtphi_dn8) / (locals.var_sqrtphi * locals.var_sqrtphi)));
        locals.var_t9_dn9 = (-((2.2361 * locals.var_sqrtphi_dn9) / (locals.var_sqrtphi * locals.var_sqrtphi)));
        locals.var_t9_dn10 = (-((2.2361 * locals.var_sqrtphi_dn10) / (locals.var_sqrtphi * locals.var_sqrtphi)));
        locals.var_t9_dn11 = (-((2.2361 * locals.var_sqrtphi_dn11) / (locals.var_sqrtphi * locals.var_sqrtphi)));
        locals.var_t9_dn12 = (-((2.2361 * locals.var_sqrtphi_dn12) / (locals.var_sqrtphi * locals.var_sqrtphi)));
        locals.var_t9_rv = 0.0;

        let assign17640_e15907: f64 = (locals.var_vbsh_cv - locals.var_vbseff_cv);
        let assign17640_e15908: f64 = (locals.var_t9 * assign17640_e15907);
        let assign17640_e15909: f64 = (locals.var_sqrtphis_cv - assign17640_e15908);
        locals.var_sqrtphisext_cv = assign17640_e15909;
        locals.var_sqrtphisext_cv_dn3 = (locals.var_sqrtphis_cv_dn3 - ((locals.var_t9_dn3 * assign17640_e15907) + (locals.var_t9 * (locals.var_vbsh_cv_dn3 - locals.var_vbseff_cv_dn3))));
        locals.var_sqrtphisext_cv_dn4 = (locals.var_sqrtphis_cv_dn4 - ((locals.var_t9_dn4 * assign17640_e15907) + (locals.var_t9 * (locals.var_vbsh_cv_dn4 - locals.var_vbseff_cv_dn4))));
        locals.var_sqrtphisext_cv_dn5 = (locals.var_sqrtphis_cv_dn5 - ((locals.var_t9_dn5 * assign17640_e15907) + (locals.var_t9 * (locals.var_vbsh_cv_dn5 - locals.var_vbseff_cv_dn5))));
        locals.var_sqrtphisext_cv_dn6 = (locals.var_sqrtphis_cv_dn6 - ((locals.var_t9_dn6 * assign17640_e15907) + (locals.var_t9 * (locals.var_vbsh_cv_dn6 - locals.var_vbseff_cv_dn6))));
        locals.var_sqrtphisext_cv_dn7 = (locals.var_sqrtphis_cv_dn7 - ((locals.var_t9_dn7 * assign17640_e15907) + (locals.var_t9 * (locals.var_vbsh_cv_dn7 - locals.var_vbseff_cv_dn7))));
        locals.var_sqrtphisext_cv_dn8 = (locals.var_sqrtphis_cv_dn8 - ((locals.var_t9_dn8 * assign17640_e15907) + (locals.var_t9 * (locals.var_vbsh_cv_dn8 - locals.var_vbseff_cv_dn8))));
        locals.var_sqrtphisext_cv_dn9 = (locals.var_sqrtphis_cv_dn9 - ((locals.var_t9_dn9 * assign17640_e15907) + (locals.var_t9 * (locals.var_vbsh_cv_dn9 - locals.var_vbseff_cv_dn9))));
        locals.var_sqrtphisext_cv_dn10 = (locals.var_sqrtphis_cv_dn10 - ((locals.var_t9_dn10 * assign17640_e15907) + (locals.var_t9 * (locals.var_vbsh_cv_dn10 - locals.var_vbseff_cv_dn10))));
        locals.var_sqrtphisext_cv_dn11 = (locals.var_sqrtphis_cv_dn11 - ((locals.var_t9_dn11 * assign17640_e15907) + (locals.var_t9 * (locals.var_vbsh_cv_dn11 - locals.var_vbseff_cv_dn11))));
        locals.var_sqrtphisext_cv_dn12 = (locals.var_sqrtphis_cv_dn12 - ((locals.var_t9_dn12 * assign17640_e15907) + (locals.var_t9 * (locals.var_vbsh_cv_dn12 - locals.var_vbseff_cv_dn12))));
        locals.var_sqrtphisext_cv_rv = 0.0;

        let assign17650_e15912: f64 = (2.0 * locals.var_pparam_b4soidvtp4);
        let assign17650_e15914: f64 = (assign17650_e15912 * locals.var_vds_1);
        let assign17650_e15915: f64 = (assign17650_e15914).exp();
        locals.var_t0__blk808 = assign17650_e15915;
        locals.var_t0__blk808_dn3 = (assign17650_e15915 * ((2.0 * locals.var_pparam_b4soidvtp4_dn3) * locals.var_vds_1));
        locals.var_t0__blk808_dn4 = (assign17650_e15915 * ((2.0 * locals.var_pparam_b4soidvtp4_dn4) * locals.var_vds_1));
        locals.var_t0__blk808_dn5 = (assign17650_e15915 * ((2.0 * locals.var_pparam_b4soidvtp4_dn5) * locals.var_vds_1));
        locals.var_t0__blk808_dn6 = (assign17650_e15915 * ((2.0 * locals.var_pparam_b4soidvtp4_dn6) * locals.var_vds_1));
        locals.var_t0__blk808_dn7 = (assign17650_e15915 * (((2.0 * locals.var_pparam_b4soidvtp4_dn7) * locals.var_vds_1) + (assign17650_e15912 * locals.var_vds_1_dn7)));
        locals.var_t0__blk808_dn8 = (assign17650_e15915 * (((2.0 * locals.var_pparam_b4soidvtp4_dn8) * locals.var_vds_1) + (assign17650_e15912 * locals.var_vds_1_dn8)));
        locals.var_t0__blk808_dn9 = (assign17650_e15915 * ((2.0 * locals.var_pparam_b4soidvtp4_dn9) * locals.var_vds_1));
        locals.var_t0__blk808_dn10 = (assign17650_e15915 * ((2.0 * locals.var_pparam_b4soidvtp4_dn10) * locals.var_vds_1));
        locals.var_t0__blk808_dn11 = (assign17650_e15915 * ((2.0 * locals.var_pparam_b4soidvtp4_dn11) * locals.var_vds_1));
        locals.var_t0__blk808_dn12 = (assign17650_e15915 * ((2.0 * locals.var_pparam_b4soidvtp4_dn12) * locals.var_vds_1));
        locals.var_t0__blk808_rv = 0.0;

        let assign17660_e15919: f64 = (locals.var_t0__blk808 - 1.0);
        let assign17660_e15920: f64 = (locals.var_pparam_b4soidvtp2factor * assign17660_e15919);
        let assign17660_e15923: f64 = (locals.var_t0__blk808 + 1.0);
        let assign17660_e15924: f64 = (assign17660_e15920 / assign17660_e15923);
        locals.var_dits_sft2__blk1056 = assign17660_e15924;
        locals.var_dits_sft2__blk1056_dn3 = (((((locals.var_pparam_b4soidvtp2factor_dn3 * assign17660_e15919) + (locals.var_pparam_b4soidvtp2factor * locals.var_t0__blk808_dn3)) * assign17660_e15923) - (assign17660_e15920 * locals.var_t0__blk808_dn3)) / (assign17660_e15923 * assign17660_e15923));
        locals.var_dits_sft2__blk1056_dn4 = (((((locals.var_pparam_b4soidvtp2factor_dn4 * assign17660_e15919) + (locals.var_pparam_b4soidvtp2factor * locals.var_t0__blk808_dn4)) * assign17660_e15923) - (assign17660_e15920 * locals.var_t0__blk808_dn4)) / (assign17660_e15923 * assign17660_e15923));
        locals.var_dits_sft2__blk1056_dn5 = (((((locals.var_pparam_b4soidvtp2factor_dn5 * assign17660_e15919) + (locals.var_pparam_b4soidvtp2factor * locals.var_t0__blk808_dn5)) * assign17660_e15923) - (assign17660_e15920 * locals.var_t0__blk808_dn5)) / (assign17660_e15923 * assign17660_e15923));
        locals.var_dits_sft2__blk1056_dn6 = (((((locals.var_pparam_b4soidvtp2factor_dn6 * assign17660_e15919) + (locals.var_pparam_b4soidvtp2factor * locals.var_t0__blk808_dn6)) * assign17660_e15923) - (assign17660_e15920 * locals.var_t0__blk808_dn6)) / (assign17660_e15923 * assign17660_e15923));
        locals.var_dits_sft2__blk1056_dn7 = (((((locals.var_pparam_b4soidvtp2factor_dn7 * assign17660_e15919) + (locals.var_pparam_b4soidvtp2factor * locals.var_t0__blk808_dn7)) * assign17660_e15923) - (assign17660_e15920 * locals.var_t0__blk808_dn7)) / (assign17660_e15923 * assign17660_e15923));
        locals.var_dits_sft2__blk1056_dn8 = (((((locals.var_pparam_b4soidvtp2factor_dn8 * assign17660_e15919) + (locals.var_pparam_b4soidvtp2factor * locals.var_t0__blk808_dn8)) * assign17660_e15923) - (assign17660_e15920 * locals.var_t0__blk808_dn8)) / (assign17660_e15923 * assign17660_e15923));
        locals.var_dits_sft2__blk1056_dn9 = (((((locals.var_pparam_b4soidvtp2factor_dn9 * assign17660_e15919) + (locals.var_pparam_b4soidvtp2factor * locals.var_t0__blk808_dn9)) * assign17660_e15923) - (assign17660_e15920 * locals.var_t0__blk808_dn9)) / (assign17660_e15923 * assign17660_e15923));
        locals.var_dits_sft2__blk1056_dn10 = (((((locals.var_pparam_b4soidvtp2factor_dn10 * assign17660_e15919) + (locals.var_pparam_b4soidvtp2factor * locals.var_t0__blk808_dn10)) * assign17660_e15923) - (assign17660_e15920 * locals.var_t0__blk808_dn10)) / (assign17660_e15923 * assign17660_e15923));
        locals.var_dits_sft2__blk1056_dn11 = (((((locals.var_pparam_b4soidvtp2factor_dn11 * assign17660_e15919) + (locals.var_pparam_b4soidvtp2factor * locals.var_t0__blk808_dn11)) * assign17660_e15923) - (assign17660_e15920 * locals.var_t0__blk808_dn11)) / (assign17660_e15923 * assign17660_e15923));
        locals.var_dits_sft2__blk1056_dn12 = (((((locals.var_pparam_b4soidvtp2factor_dn12 * assign17660_e15919) + (locals.var_pparam_b4soidvtp2factor * locals.var_t0__blk808_dn12)) * assign17660_e15923) - (assign17660_e15920 * locals.var_t0__blk808_dn12)) / (assign17660_e15923 * assign17660_e15923));
        locals.var_dits_sft2__blk1056_rv = 0.0;

        let assign17670_e15927: f64 = (p.p37 * locals.var_here_b4soivth0);
        let assign17670_e15930: f64 = (locals.var_here_b4soik1ox * locals.var_sqrtphisext_cv);
        let assign17670_e15933: f64 = (locals.var_here_b4soik1eff * locals.var_sqrtphi);
        let assign17670_e15934: f64 = (assign17670_e15930 - assign17670_e15933);
        let assign17670_e15936: f64 = (assign17670_e15934 * locals.var_lpe_vb__blk1054);
        let assign17670_e15937: f64 = (assign17670_e15927 + assign17670_e15936);
        let assign17670_e15940: f64 = (locals.var_here_b4soik2ox * locals.var_vbseff_cv);
        let assign17670_e15941: f64 = (assign17670_e15937 - assign17670_e15940);
        let assign17670_e15943: f64 = (assign17670_e15941 - locals.var_delt_vth_cv);
        let assign17670_e15945: f64 = (assign17670_e15943 - locals.var_deltvthw_cv);
        let assign17670_e15949: f64 = (locals.var_pparam_b4soik3b * locals.var_vbseff_cv);
        let assign17670_e15950: f64 = (locals.var_pparam_b4soik3 + assign17670_e15949);
        let assign17670_e15952: f64 = (assign17670_e15950 * locals.var_tmp2_cv);
        let assign17670_e15953: f64 = (assign17670_e15945 + assign17670_e15952);
        let assign17670_e15955: f64 = (assign17670_e15953 + locals.var_deltvthtemp_cv);
        let assign17670_e15957: f64 = (assign17670_e15955 - locals.var_dibl_sft_cv);
        let assign17670_e15959: f64 = (assign17670_e15957 - locals.var_dits_sft_cv);
        let assign17670_e15961: f64 = (assign17670_e15959 - locals.var_dits_sft2__blk1056);
        locals.var_vth_cv = assign17670_e15961;
        locals.var_vth_cv_dn3 = ((((((((((p.p37 * locals.var_here_b4soivth0_dn3) + (((((locals.var_here_b4soik1ox_dn3 * locals.var_sqrtphisext_cv) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext_cv_dn3)) - ((locals.var_here_b4soik1eff_dn3 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn3))) * locals.var_lpe_vb__blk1054) + (assign17670_e15934 * locals.var_lpe_vb__blk1054_dn3))) - ((locals.var_here_b4soik2ox_dn3 * locals.var_vbseff_cv) + (locals.var_here_b4soik2ox * locals.var_vbseff_cv_dn3))) - locals.var_delt_vth_cv_dn3) - locals.var_deltvthw_cv_dn3) + (((locals.var_pparam_b4soik3_dn3 + ((locals.var_pparam_b4soik3b_dn3 * locals.var_vbseff_cv) + (locals.var_pparam_b4soik3b * locals.var_vbseff_cv_dn3))) * locals.var_tmp2_cv) + (assign17670_e15950 * locals.var_tmp2_cv_dn3))) + locals.var_deltvthtemp_cv_dn3) - locals.var_dibl_sft_cv_dn3) - locals.var_dits_sft_cv_dn3) - locals.var_dits_sft2__blk1056_dn3);
        locals.var_vth_cv_dn4 = ((((((((((p.p37 * locals.var_here_b4soivth0_dn4) + (((((locals.var_here_b4soik1ox_dn4 * locals.var_sqrtphisext_cv) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext_cv_dn4)) - ((locals.var_here_b4soik1eff_dn4 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn4))) * locals.var_lpe_vb__blk1054) + (assign17670_e15934 * locals.var_lpe_vb__blk1054_dn4))) - ((locals.var_here_b4soik2ox_dn4 * locals.var_vbseff_cv) + (locals.var_here_b4soik2ox * locals.var_vbseff_cv_dn4))) - locals.var_delt_vth_cv_dn4) - locals.var_deltvthw_cv_dn4) + (((locals.var_pparam_b4soik3_dn4 + ((locals.var_pparam_b4soik3b_dn4 * locals.var_vbseff_cv) + (locals.var_pparam_b4soik3b * locals.var_vbseff_cv_dn4))) * locals.var_tmp2_cv) + (assign17670_e15950 * locals.var_tmp2_cv_dn4))) + locals.var_deltvthtemp_cv_dn4) - locals.var_dibl_sft_cv_dn4) - locals.var_dits_sft_cv_dn4) - locals.var_dits_sft2__blk1056_dn4);
        locals.var_vth_cv_dn5 = ((((((((((p.p37 * locals.var_here_b4soivth0_dn5) + (((((locals.var_here_b4soik1ox_dn5 * locals.var_sqrtphisext_cv) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext_cv_dn5)) - ((locals.var_here_b4soik1eff_dn5 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn5))) * locals.var_lpe_vb__blk1054) + (assign17670_e15934 * locals.var_lpe_vb__blk1054_dn5))) - ((locals.var_here_b4soik2ox_dn5 * locals.var_vbseff_cv) + (locals.var_here_b4soik2ox * locals.var_vbseff_cv_dn5))) - locals.var_delt_vth_cv_dn5) - locals.var_deltvthw_cv_dn5) + (((locals.var_pparam_b4soik3_dn5 + ((locals.var_pparam_b4soik3b_dn5 * locals.var_vbseff_cv) + (locals.var_pparam_b4soik3b * locals.var_vbseff_cv_dn5))) * locals.var_tmp2_cv) + (assign17670_e15950 * locals.var_tmp2_cv_dn5))) + locals.var_deltvthtemp_cv_dn5) - locals.var_dibl_sft_cv_dn5) - locals.var_dits_sft_cv_dn5) - locals.var_dits_sft2__blk1056_dn5);
        locals.var_vth_cv_dn6 = ((((((((((p.p37 * locals.var_here_b4soivth0_dn6) + (((((locals.var_here_b4soik1ox_dn6 * locals.var_sqrtphisext_cv) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext_cv_dn6)) - ((locals.var_here_b4soik1eff_dn6 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn6))) * locals.var_lpe_vb__blk1054) + (assign17670_e15934 * locals.var_lpe_vb__blk1054_dn6))) - ((locals.var_here_b4soik2ox_dn6 * locals.var_vbseff_cv) + (locals.var_here_b4soik2ox * locals.var_vbseff_cv_dn6))) - locals.var_delt_vth_cv_dn6) - locals.var_deltvthw_cv_dn6) + (((locals.var_pparam_b4soik3_dn6 + ((locals.var_pparam_b4soik3b_dn6 * locals.var_vbseff_cv) + (locals.var_pparam_b4soik3b * locals.var_vbseff_cv_dn6))) * locals.var_tmp2_cv) + (assign17670_e15950 * locals.var_tmp2_cv_dn6))) + locals.var_deltvthtemp_cv_dn6) - locals.var_dibl_sft_cv_dn6) - locals.var_dits_sft_cv_dn6) - locals.var_dits_sft2__blk1056_dn6);
        locals.var_vth_cv_dn7 = ((((((((((p.p37 * locals.var_here_b4soivth0_dn7) + (((((locals.var_here_b4soik1ox_dn7 * locals.var_sqrtphisext_cv) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext_cv_dn7)) - ((locals.var_here_b4soik1eff_dn7 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn7))) * locals.var_lpe_vb__blk1054) + (assign17670_e15934 * locals.var_lpe_vb__blk1054_dn7))) - ((locals.var_here_b4soik2ox_dn7 * locals.var_vbseff_cv) + (locals.var_here_b4soik2ox * locals.var_vbseff_cv_dn7))) - locals.var_delt_vth_cv_dn7) - locals.var_deltvthw_cv_dn7) + (((locals.var_pparam_b4soik3_dn7 + ((locals.var_pparam_b4soik3b_dn7 * locals.var_vbseff_cv) + (locals.var_pparam_b4soik3b * locals.var_vbseff_cv_dn7))) * locals.var_tmp2_cv) + (assign17670_e15950 * locals.var_tmp2_cv_dn7))) + locals.var_deltvthtemp_cv_dn7) - locals.var_dibl_sft_cv_dn7) - locals.var_dits_sft_cv_dn7) - locals.var_dits_sft2__blk1056_dn7);
        locals.var_vth_cv_dn8 = ((((((((((p.p37 * locals.var_here_b4soivth0_dn8) + (((((locals.var_here_b4soik1ox_dn8 * locals.var_sqrtphisext_cv) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext_cv_dn8)) - ((locals.var_here_b4soik1eff_dn8 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn8))) * locals.var_lpe_vb__blk1054) + (assign17670_e15934 * locals.var_lpe_vb__blk1054_dn8))) - ((locals.var_here_b4soik2ox_dn8 * locals.var_vbseff_cv) + (locals.var_here_b4soik2ox * locals.var_vbseff_cv_dn8))) - locals.var_delt_vth_cv_dn8) - locals.var_deltvthw_cv_dn8) + (((locals.var_pparam_b4soik3_dn8 + ((locals.var_pparam_b4soik3b_dn8 * locals.var_vbseff_cv) + (locals.var_pparam_b4soik3b * locals.var_vbseff_cv_dn8))) * locals.var_tmp2_cv) + (assign17670_e15950 * locals.var_tmp2_cv_dn8))) + locals.var_deltvthtemp_cv_dn8) - locals.var_dibl_sft_cv_dn8) - locals.var_dits_sft_cv_dn8) - locals.var_dits_sft2__blk1056_dn8);
        locals.var_vth_cv_dn9 = ((((((((((p.p37 * locals.var_here_b4soivth0_dn9) + (((((locals.var_here_b4soik1ox_dn9 * locals.var_sqrtphisext_cv) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext_cv_dn9)) - ((locals.var_here_b4soik1eff_dn9 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn9))) * locals.var_lpe_vb__blk1054) + (assign17670_e15934 * locals.var_lpe_vb__blk1054_dn9))) - ((locals.var_here_b4soik2ox_dn9 * locals.var_vbseff_cv) + (locals.var_here_b4soik2ox * locals.var_vbseff_cv_dn9))) - locals.var_delt_vth_cv_dn9) - locals.var_deltvthw_cv_dn9) + (((locals.var_pparam_b4soik3_dn9 + ((locals.var_pparam_b4soik3b_dn9 * locals.var_vbseff_cv) + (locals.var_pparam_b4soik3b * locals.var_vbseff_cv_dn9))) * locals.var_tmp2_cv) + (assign17670_e15950 * locals.var_tmp2_cv_dn9))) + locals.var_deltvthtemp_cv_dn9) - locals.var_dibl_sft_cv_dn9) - locals.var_dits_sft_cv_dn9) - locals.var_dits_sft2__blk1056_dn9);
        locals.var_vth_cv_dn10 = ((((((((((p.p37 * locals.var_here_b4soivth0_dn10) + (((((locals.var_here_b4soik1ox_dn10 * locals.var_sqrtphisext_cv) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext_cv_dn10)) - ((locals.var_here_b4soik1eff_dn10 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn10))) * locals.var_lpe_vb__blk1054) + (assign17670_e15934 * locals.var_lpe_vb__blk1054_dn10))) - ((locals.var_here_b4soik2ox_dn10 * locals.var_vbseff_cv) + (locals.var_here_b4soik2ox * locals.var_vbseff_cv_dn10))) - locals.var_delt_vth_cv_dn10) - locals.var_deltvthw_cv_dn10) + (((locals.var_pparam_b4soik3_dn10 + ((locals.var_pparam_b4soik3b_dn10 * locals.var_vbseff_cv) + (locals.var_pparam_b4soik3b * locals.var_vbseff_cv_dn10))) * locals.var_tmp2_cv) + (assign17670_e15950 * locals.var_tmp2_cv_dn10))) + locals.var_deltvthtemp_cv_dn10) - locals.var_dibl_sft_cv_dn10) - locals.var_dits_sft_cv_dn10) - locals.var_dits_sft2__blk1056_dn10);
        locals.var_vth_cv_dn11 = ((((((((((p.p37 * locals.var_here_b4soivth0_dn11) + (((((locals.var_here_b4soik1ox_dn11 * locals.var_sqrtphisext_cv) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext_cv_dn11)) - ((locals.var_here_b4soik1eff_dn11 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn11))) * locals.var_lpe_vb__blk1054) + (assign17670_e15934 * locals.var_lpe_vb__blk1054_dn11))) - ((locals.var_here_b4soik2ox_dn11 * locals.var_vbseff_cv) + (locals.var_here_b4soik2ox * locals.var_vbseff_cv_dn11))) - locals.var_delt_vth_cv_dn11) - locals.var_deltvthw_cv_dn11) + (((locals.var_pparam_b4soik3_dn11 + ((locals.var_pparam_b4soik3b_dn11 * locals.var_vbseff_cv) + (locals.var_pparam_b4soik3b * locals.var_vbseff_cv_dn11))) * locals.var_tmp2_cv) + (assign17670_e15950 * locals.var_tmp2_cv_dn11))) + locals.var_deltvthtemp_cv_dn11) - locals.var_dibl_sft_cv_dn11) - locals.var_dits_sft_cv_dn11) - locals.var_dits_sft2__blk1056_dn11);
        locals.var_vth_cv_dn12 = ((((((((((p.p37 * locals.var_here_b4soivth0_dn12) + (((((locals.var_here_b4soik1ox_dn12 * locals.var_sqrtphisext_cv) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext_cv_dn12)) - ((locals.var_here_b4soik1eff_dn12 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn12))) * locals.var_lpe_vb__blk1054) + (assign17670_e15934 * locals.var_lpe_vb__blk1054_dn12))) - ((locals.var_here_b4soik2ox_dn12 * locals.var_vbseff_cv) + (locals.var_here_b4soik2ox * locals.var_vbseff_cv_dn12))) - locals.var_delt_vth_cv_dn12) - locals.var_deltvthw_cv_dn12) + (((locals.var_pparam_b4soik3_dn12 + ((locals.var_pparam_b4soik3b_dn12 * locals.var_vbseff_cv) + (locals.var_pparam_b4soik3b * locals.var_vbseff_cv_dn12))) * locals.var_tmp2_cv) + (assign17670_e15950 * locals.var_tmp2_cv_dn12))) + locals.var_deltvthtemp_cv_dn12) - locals.var_dibl_sft_cv_dn12) - locals.var_dits_sft_cv_dn12) - locals.var_dits_sft2__blk1056_dn12);
        locals.var_vth_cv_rv = 0.0;

        let assign17680_e15972: f64 = if (((p.p61 == 3.0) && (p.p36 == 1.0)) && (p.p14 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1206 = assign17680_e15972;
        locals.var_guard1206_rv = 0.0;

        let (assign17690_e15977, assign17690_e15977_d_n3, assign17690_e15977_d_n4, assign17690_e15977_d_n5, assign17690_e15977_d_n6, assign17690_e15977_d_n7, assign17690_e15977_d_n8, assign17690_e15977_d_n9, assign17690_e15977_d_n10, assign17690_e15977_d_n11, assign17690_e15977_d_n12,) = {
    if (locals.var_guard1206 != 0.0) {
        let assign17690_e15975: f64 = (locals.var_xdep0).sqrt();
        (assign17690_e15975, (locals.var_xdep0_dn3 / (2.0 * assign17690_e15975)), (locals.var_xdep0_dn4 / (2.0 * assign17690_e15975)), (locals.var_xdep0_dn5 / (2.0 * assign17690_e15975)), (locals.var_xdep0_dn6 / (2.0 * assign17690_e15975)), (locals.var_xdep0_dn7 / (2.0 * assign17690_e15975)), (locals.var_xdep0_dn8 / (2.0 * assign17690_e15975)), (locals.var_xdep0_dn9 / (2.0 * assign17690_e15975)), (locals.var_xdep0_dn10 / (2.0 * assign17690_e15975)), (locals.var_xdep0_dn11 / (2.0 * assign17690_e15975)), (locals.var_xdep0_dn12 / (2.0 * assign17690_e15975)),)
    } else {
        (locals.var_t3zb, locals.var_t3zb_dn3, locals.var_t3zb_dn4, locals.var_t3zb_dn5, locals.var_t3zb_dn6, locals.var_t3zb_dn7, locals.var_t3zb_dn8, locals.var_t3zb_dn9, locals.var_t3zb_dn10, locals.var_t3zb_dn11, locals.var_t3zb_dn12,)
    }
};
        locals.var_t3zb = assign17690_e15977;
        locals.var_t3zb_dn3 = assign17690_e15977_d_n3;
        locals.var_t3zb_dn4 = assign17690_e15977_d_n4;
        locals.var_t3zb_dn5 = assign17690_e15977_d_n5;
        locals.var_t3zb_dn6 = assign17690_e15977_d_n6;
        locals.var_t3zb_dn7 = assign17690_e15977_d_n7;
        locals.var_t3zb_dn8 = assign17690_e15977_d_n8;
        locals.var_t3zb_dn9 = assign17690_e15977_d_n9;
        locals.var_t3zb_dn10 = assign17690_e15977_d_n10;
        locals.var_t3zb_dn11 = assign17690_e15977_d_n11;
        locals.var_t3zb_dn12 = assign17690_e15977_d_n12;
        locals.var_t3zb_rv = 0.0;

        let (assign17700_e15983, assign17700_e15983_d_n3, assign17700_e15983_d_n4, assign17700_e15983_d_n5, assign17700_e15983_d_n6, assign17700_e15983_d_n7, assign17700_e15983_d_n8, assign17700_e15983_d_n9, assign17700_e15983_d_n10, assign17700_e15983_d_n11, assign17700_e15983_d_n12,) = {
    if (locals.var_guard1206 != 0.0) {
        let assign17700_e15981: f64 = (locals.var_b4soifactor1 * locals.var_t3zb);
        (assign17700_e15981, (locals.var_b4soifactor1 * locals.var_t3zb_dn3), (locals.var_b4soifactor1 * locals.var_t3zb_dn4), (locals.var_b4soifactor1 * locals.var_t3zb_dn5), (locals.var_b4soifactor1 * locals.var_t3zb_dn6), (locals.var_b4soifactor1 * locals.var_t3zb_dn7), (locals.var_b4soifactor1 * locals.var_t3zb_dn8), (locals.var_b4soifactor1 * locals.var_t3zb_dn9), (locals.var_b4soifactor1 * locals.var_t3zb_dn10), (locals.var_b4soifactor1 * locals.var_t3zb_dn11), (locals.var_b4soifactor1 * locals.var_t3zb_dn12),)
    } else {
        (locals.var_lt1zb, locals.var_lt1zb_dn3, locals.var_lt1zb_dn4, locals.var_lt1zb_dn5, locals.var_lt1zb_dn6, locals.var_lt1zb_dn7, locals.var_lt1zb_dn8, locals.var_lt1zb_dn9, locals.var_lt1zb_dn10, locals.var_lt1zb_dn11, locals.var_lt1zb_dn12,)
    }
};
        locals.var_lt1zb = assign17700_e15983;
        locals.var_lt1zb_dn3 = assign17700_e15983_d_n3;
        locals.var_lt1zb_dn4 = assign17700_e15983_d_n4;
        locals.var_lt1zb_dn5 = assign17700_e15983_d_n5;
        locals.var_lt1zb_dn6 = assign17700_e15983_d_n6;
        locals.var_lt1zb_dn7 = assign17700_e15983_d_n7;
        locals.var_lt1zb_dn8 = assign17700_e15983_d_n8;
        locals.var_lt1zb_dn9 = assign17700_e15983_d_n9;
        locals.var_lt1zb_dn10 = assign17700_e15983_d_n10;
        locals.var_lt1zb_dn11 = assign17700_e15983_d_n11;
        locals.var_lt1zb_dn12 = assign17700_e15983_d_n12;
        locals.var_lt1zb_rv = 0.0;

        let (assign17710_e15989, assign17710_e15989_d_n3, assign17710_e15989_d_n4, assign17710_e15989_d_n5, assign17710_e15989_d_n6, assign17710_e15989_d_n7, assign17710_e15989_d_n8, assign17710_e15989_d_n9, assign17710_e15989_d_n10, assign17710_e15989_d_n11, assign17710_e15989_d_n12,) = {
    if (locals.var_guard1206 != 0.0) {
        let assign17710_e15987: f64 = (locals.var_b4soifactor1 * locals.var_t3zb);
        (assign17710_e15987, (locals.var_b4soifactor1 * locals.var_t3zb_dn3), (locals.var_b4soifactor1 * locals.var_t3zb_dn4), (locals.var_b4soifactor1 * locals.var_t3zb_dn5), (locals.var_b4soifactor1 * locals.var_t3zb_dn6), (locals.var_b4soifactor1 * locals.var_t3zb_dn7), (locals.var_b4soifactor1 * locals.var_t3zb_dn8), (locals.var_b4soifactor1 * locals.var_t3zb_dn9), (locals.var_b4soifactor1 * locals.var_t3zb_dn10), (locals.var_b4soifactor1 * locals.var_t3zb_dn11), (locals.var_b4soifactor1 * locals.var_t3zb_dn12),)
    } else {
        (locals.var_ltwzb, locals.var_ltwzb_dn3, locals.var_ltwzb_dn4, locals.var_ltwzb_dn5, locals.var_ltwzb_dn6, locals.var_ltwzb_dn7, locals.var_ltwzb_dn8, locals.var_ltwzb_dn9, locals.var_ltwzb_dn10, locals.var_ltwzb_dn11, locals.var_ltwzb_dn12,)
    }
};
        locals.var_ltwzb = assign17710_e15989;
        locals.var_ltwzb_dn3 = assign17710_e15989_d_n3;
        locals.var_ltwzb_dn4 = assign17710_e15989_d_n4;
        locals.var_ltwzb_dn5 = assign17710_e15989_d_n5;
        locals.var_ltwzb_dn6 = assign17710_e15989_d_n6;
        locals.var_ltwzb_dn7 = assign17710_e15989_d_n7;
        locals.var_ltwzb_dn8 = assign17710_e15989_d_n8;
        locals.var_ltwzb_dn9 = assign17710_e15989_d_n9;
        locals.var_ltwzb_dn10 = assign17710_e15989_d_n10;
        locals.var_ltwzb_dn11 = assign17710_e15989_d_n11;
        locals.var_ltwzb_dn12 = assign17710_e15989_d_n12;
        locals.var_ltwzb_rv = 0.0;

        let (assign17720_e16000, assign17720_e16000_d_n3, assign17720_e16000_d_n4, assign17720_e16000_d_n5, assign17720_e16000_d_n6, assign17720_e16000_d_n7, assign17720_e16000_d_n8, assign17720_e16000_d_n9, assign17720_e16000_d_n10, assign17720_e16000_d_n11, assign17720_e16000_d_n12,) = {
    if (locals.var_guard1206 != 0.0) {
        let assign17720_e15992: f64 = (-0.5);
        let assign17720_e15994: f64 = (assign17720_e15992 * locals.var_pparam_b4soidvt1);
        let assign17720_e15996: f64 = (assign17720_e15994 * locals.var_leff);
        let assign17720_e15998: f64 = (assign17720_e15996 / locals.var_lt1zb);
        (assign17720_e15998, ((((((assign17720_e15992 * locals.var_pparam_b4soidvt1_dn3) * locals.var_leff) + (assign17720_e15994 * locals.var_leff_dn3)) * locals.var_lt1zb) - (assign17720_e15996 * locals.var_lt1zb_dn3)) / (locals.var_lt1zb * locals.var_lt1zb)), ((((((assign17720_e15992 * locals.var_pparam_b4soidvt1_dn4) * locals.var_leff) + (assign17720_e15994 * locals.var_leff_dn4)) * locals.var_lt1zb) - (assign17720_e15996 * locals.var_lt1zb_dn4)) / (locals.var_lt1zb * locals.var_lt1zb)), ((((((assign17720_e15992 * locals.var_pparam_b4soidvt1_dn5) * locals.var_leff) + (assign17720_e15994 * locals.var_leff_dn5)) * locals.var_lt1zb) - (assign17720_e15996 * locals.var_lt1zb_dn5)) / (locals.var_lt1zb * locals.var_lt1zb)), ((((((assign17720_e15992 * locals.var_pparam_b4soidvt1_dn6) * locals.var_leff) + (assign17720_e15994 * locals.var_leff_dn6)) * locals.var_lt1zb) - (assign17720_e15996 * locals.var_lt1zb_dn6)) / (locals.var_lt1zb * locals.var_lt1zb)), ((((((assign17720_e15992 * locals.var_pparam_b4soidvt1_dn7) * locals.var_leff) + (assign17720_e15994 * locals.var_leff_dn7)) * locals.var_lt1zb) - (assign17720_e15996 * locals.var_lt1zb_dn7)) / (locals.var_lt1zb * locals.var_lt1zb)), ((((((assign17720_e15992 * locals.var_pparam_b4soidvt1_dn8) * locals.var_leff) + (assign17720_e15994 * locals.var_leff_dn8)) * locals.var_lt1zb) - (assign17720_e15996 * locals.var_lt1zb_dn8)) / (locals.var_lt1zb * locals.var_lt1zb)), ((((((assign17720_e15992 * locals.var_pparam_b4soidvt1_dn9) * locals.var_leff) + (assign17720_e15994 * locals.var_leff_dn9)) * locals.var_lt1zb) - (assign17720_e15996 * locals.var_lt1zb_dn9)) / (locals.var_lt1zb * locals.var_lt1zb)), ((((((assign17720_e15992 * locals.var_pparam_b4soidvt1_dn10) * locals.var_leff) + (assign17720_e15994 * locals.var_leff_dn10)) * locals.var_lt1zb) - (assign17720_e15996 * locals.var_lt1zb_dn10)) / (locals.var_lt1zb * locals.var_lt1zb)), ((((((assign17720_e15992 * locals.var_pparam_b4soidvt1_dn11) * locals.var_leff) + (assign17720_e15994 * locals.var_leff_dn11)) * locals.var_lt1zb) - (assign17720_e15996 * locals.var_lt1zb_dn11)) / (locals.var_lt1zb * locals.var_lt1zb)), ((((((assign17720_e15992 * locals.var_pparam_b4soidvt1_dn12) * locals.var_leff) + (assign17720_e15994 * locals.var_leff_dn12)) * locals.var_lt1zb) - (assign17720_e15996 * locals.var_lt1zb_dn12)) / (locals.var_lt1zb * locals.var_lt1zb)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign17720_e16000;
        locals.var_t0__blk808_dn3 = assign17720_e16000_d_n3;
        locals.var_t0__blk808_dn4 = assign17720_e16000_d_n4;
        locals.var_t0__blk808_dn5 = assign17720_e16000_d_n5;
        locals.var_t0__blk808_dn6 = assign17720_e16000_d_n6;
        locals.var_t0__blk808_dn7 = assign17720_e16000_d_n7;
        locals.var_t0__blk808_dn8 = assign17720_e16000_d_n8;
        locals.var_t0__blk808_dn9 = assign17720_e16000_d_n9;
        locals.var_t0__blk808_dn10 = assign17720_e16000_d_n10;
        locals.var_t0__blk808_dn11 = assign17720_e16000_d_n11;
        locals.var_t0__blk808_dn12 = assign17720_e16000_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let assign17730_e16003: f64 = (-100.0);
        let assign17730_e16004: f64 = if locals.var_t0__blk808 > assign17730_e16003 { 1.0 } else { 0.0 };
        locals.var_guard1207 = assign17730_e16004;
        locals.var_guard1207_rv = 0.0;

        let (assign17740_e16011, assign17740_e16011_d_n3, assign17740_e16011_d_n4, assign17740_e16011_d_n5, assign17740_e16011_d_n6, assign17740_e16011_d_n7, assign17740_e16011_d_n8, assign17740_e16011_d_n9, assign17740_e16011_d_n10, assign17740_e16011_d_n11, assign17740_e16011_d_n12,) = {
    if ((locals.var_guard1206 != 0.0) && (locals.var_guard1207 != 0.0)) {
        let assign17740_e16009: f64 = (locals.var_t0__blk808).exp();
        (assign17740_e16009, (assign17740_e16009 * locals.var_t0__blk808_dn3), (assign17740_e16009 * locals.var_t0__blk808_dn4), (assign17740_e16009 * locals.var_t0__blk808_dn5), (assign17740_e16009 * locals.var_t0__blk808_dn6), (assign17740_e16009 * locals.var_t0__blk808_dn7), (assign17740_e16009 * locals.var_t0__blk808_dn8), (assign17740_e16009 * locals.var_t0__blk808_dn9), (assign17740_e16009 * locals.var_t0__blk808_dn10), (assign17740_e16009 * locals.var_t0__blk808_dn11), (assign17740_e16009 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign17740_e16011;
        locals.var_t1__blk809_dn3 = assign17740_e16011_d_n3;
        locals.var_t1__blk809_dn4 = assign17740_e16011_d_n4;
        locals.var_t1__blk809_dn5 = assign17740_e16011_d_n5;
        locals.var_t1__blk809_dn6 = assign17740_e16011_d_n6;
        locals.var_t1__blk809_dn7 = assign17740_e16011_d_n7;
        locals.var_t1__blk809_dn8 = assign17740_e16011_d_n8;
        locals.var_t1__blk809_dn9 = assign17740_e16011_d_n9;
        locals.var_t1__blk809_dn10 = assign17740_e16011_d_n10;
        locals.var_t1__blk809_dn11 = assign17740_e16011_d_n11;
        locals.var_t1__blk809_dn12 = assign17740_e16011_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign17750_e16023, assign17750_e16023_d_n3, assign17750_e16023_d_n4, assign17750_e16023_d_n5, assign17750_e16023_d_n6, assign17750_e16023_d_n7, assign17750_e16023_d_n8, assign17750_e16023_d_n9, assign17750_e16023_d_n10, assign17750_e16023_d_n11, assign17750_e16023_d_n12,) = {
    if ((locals.var_guard1206 != 0.0) && (locals.var_guard1207 != 0.0)) {
        let assign17750_e16019: f64 = (2.0 * locals.var_t1__blk809);
        let assign17750_e16020: f64 = (1.0 + assign17750_e16019);
        let assign17750_e16021: f64 = (locals.var_t1__blk809 * assign17750_e16020);
        (assign17750_e16021, ((locals.var_t1__blk809_dn3 * assign17750_e16020) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn3))), ((locals.var_t1__blk809_dn4 * assign17750_e16020) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn4))), ((locals.var_t1__blk809_dn5 * assign17750_e16020) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn5))), ((locals.var_t1__blk809_dn6 * assign17750_e16020) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn6))), ((locals.var_t1__blk809_dn7 * assign17750_e16020) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn7))), ((locals.var_t1__blk809_dn8 * assign17750_e16020) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn8))), ((locals.var_t1__blk809_dn9 * assign17750_e16020) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn9))), ((locals.var_t1__blk809_dn10 * assign17750_e16020) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn10))), ((locals.var_t1__blk809_dn11 * assign17750_e16020) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn11))), ((locals.var_t1__blk809_dn12 * assign17750_e16020) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn12))),)
    } else {
        (locals.var_theta0zb, locals.var_theta0zb_dn3, locals.var_theta0zb_dn4, locals.var_theta0zb_dn5, locals.var_theta0zb_dn6, locals.var_theta0zb_dn7, locals.var_theta0zb_dn8, locals.var_theta0zb_dn9, locals.var_theta0zb_dn10, locals.var_theta0zb_dn11, locals.var_theta0zb_dn12,)
    }
};
        locals.var_theta0zb = assign17750_e16023;
        locals.var_theta0zb_dn3 = assign17750_e16023_d_n3;
        locals.var_theta0zb_dn4 = assign17750_e16023_d_n4;
        locals.var_theta0zb_dn5 = assign17750_e16023_d_n5;
        locals.var_theta0zb_dn6 = assign17750_e16023_d_n6;
        locals.var_theta0zb_dn7 = assign17750_e16023_d_n7;
        locals.var_theta0zb_dn8 = assign17750_e16023_d_n8;
        locals.var_theta0zb_dn9 = assign17750_e16023_d_n9;
        locals.var_theta0zb_dn10 = assign17750_e16023_d_n10;
        locals.var_theta0zb_dn11 = assign17750_e16023_d_n11;
        locals.var_theta0zb_dn12 = assign17750_e16023_d_n12;
        locals.var_theta0zb_rv = 0.0;

        let (assign17760_e16030, assign17760_e16030_d_n3, assign17760_e16030_d_n4, assign17760_e16030_d_n5, assign17760_e16030_d_n6, assign17760_e16030_d_n7, assign17760_e16030_d_n8, assign17760_e16030_d_n9, assign17760_e16030_d_n10, assign17760_e16030_d_n11, assign17760_e16030_d_n12,) = {
    if ((locals.var_guard1206 != 0.0) && (locals.var_guard1207 == 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign17760_e16030;
        locals.var_t1__blk809_dn3 = assign17760_e16030_d_n3;
        locals.var_t1__blk809_dn4 = assign17760_e16030_d_n4;
        locals.var_t1__blk809_dn5 = assign17760_e16030_d_n5;
        locals.var_t1__blk809_dn6 = assign17760_e16030_d_n6;
        locals.var_t1__blk809_dn7 = assign17760_e16030_d_n7;
        locals.var_t1__blk809_dn8 = assign17760_e16030_d_n8;
        locals.var_t1__blk809_dn9 = assign17760_e16030_d_n9;
        locals.var_t1__blk809_dn10 = assign17760_e16030_d_n10;
        locals.var_t1__blk809_dn11 = assign17760_e16030_d_n11;
        locals.var_t1__blk809_dn12 = assign17760_e16030_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign17770_e16043, assign17770_e16043_d_n3, assign17770_e16043_d_n4, assign17770_e16043_d_n5, assign17770_e16043_d_n6, assign17770_e16043_d_n7, assign17770_e16043_d_n8, assign17770_e16043_d_n9, assign17770_e16043_d_n10, assign17770_e16043_d_n11, assign17770_e16043_d_n12,) = {
    if ((locals.var_guard1206 != 0.0) && (locals.var_guard1207 == 0.0)) {
        let assign17770_e16039: f64 = (2.0 * locals.var_t1__blk809);
        let assign17770_e16040: f64 = (1.0 + assign17770_e16039);
        let assign17770_e16041: f64 = (locals.var_t1__blk809 * assign17770_e16040);
        (assign17770_e16041, ((locals.var_t1__blk809_dn3 * assign17770_e16040) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn3))), ((locals.var_t1__blk809_dn4 * assign17770_e16040) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn4))), ((locals.var_t1__blk809_dn5 * assign17770_e16040) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn5))), ((locals.var_t1__blk809_dn6 * assign17770_e16040) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn6))), ((locals.var_t1__blk809_dn7 * assign17770_e16040) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn7))), ((locals.var_t1__blk809_dn8 * assign17770_e16040) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn8))), ((locals.var_t1__blk809_dn9 * assign17770_e16040) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn9))), ((locals.var_t1__blk809_dn10 * assign17770_e16040) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn10))), ((locals.var_t1__blk809_dn11 * assign17770_e16040) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn11))), ((locals.var_t1__blk809_dn12 * assign17770_e16040) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn12))),)
    } else {
        (locals.var_theta0zb, locals.var_theta0zb_dn3, locals.var_theta0zb_dn4, locals.var_theta0zb_dn5, locals.var_theta0zb_dn6, locals.var_theta0zb_dn7, locals.var_theta0zb_dn8, locals.var_theta0zb_dn9, locals.var_theta0zb_dn10, locals.var_theta0zb_dn11, locals.var_theta0zb_dn12,)
    }
};
        locals.var_theta0zb = assign17770_e16043;
        locals.var_theta0zb_dn3 = assign17770_e16043_d_n3;
        locals.var_theta0zb_dn4 = assign17770_e16043_d_n4;
        locals.var_theta0zb_dn5 = assign17770_e16043_d_n5;
        locals.var_theta0zb_dn6 = assign17770_e16043_d_n6;
        locals.var_theta0zb_dn7 = assign17770_e16043_d_n7;
        locals.var_theta0zb_dn8 = assign17770_e16043_d_n8;
        locals.var_theta0zb_dn9 = assign17770_e16043_d_n9;
        locals.var_theta0zb_dn10 = assign17770_e16043_d_n10;
        locals.var_theta0zb_dn11 = assign17770_e16043_d_n11;
        locals.var_theta0zb_dn12 = assign17770_e16043_d_n12;
        locals.var_theta0zb_rv = 0.0;

        let (assign17780_e16051, assign17780_e16051_d_n3, assign17780_e16051_d_n4, assign17780_e16051_d_n5, assign17780_e16051_d_n6, assign17780_e16051_d_n7, assign17780_e16051_d_n8, assign17780_e16051_d_n9, assign17780_e16051_d_n10, assign17780_e16051_d_n11, assign17780_e16051_d_n12,) = {
    if (locals.var_guard1206 != 0.0) {
        let assign17780_e16047: f64 = (locals.var_pparam_b4soidvt0 * locals.var_theta0zb);
        let assign17780_e16049: f64 = (assign17780_e16047 * locals.var_v0__blk799);
        (assign17780_e16049, ((((locals.var_pparam_b4soidvt0_dn3 * locals.var_theta0zb) + (locals.var_pparam_b4soidvt0 * locals.var_theta0zb_dn3)) * locals.var_v0__blk799) + (assign17780_e16047 * locals.var_v0__blk799_dn3)), ((((locals.var_pparam_b4soidvt0_dn4 * locals.var_theta0zb) + (locals.var_pparam_b4soidvt0 * locals.var_theta0zb_dn4)) * locals.var_v0__blk799) + (assign17780_e16047 * locals.var_v0__blk799_dn4)), ((((locals.var_pparam_b4soidvt0_dn5 * locals.var_theta0zb) + (locals.var_pparam_b4soidvt0 * locals.var_theta0zb_dn5)) * locals.var_v0__blk799) + (assign17780_e16047 * locals.var_v0__blk799_dn5)), ((((locals.var_pparam_b4soidvt0_dn6 * locals.var_theta0zb) + (locals.var_pparam_b4soidvt0 * locals.var_theta0zb_dn6)) * locals.var_v0__blk799) + (assign17780_e16047 * locals.var_v0__blk799_dn6)), ((((locals.var_pparam_b4soidvt0_dn7 * locals.var_theta0zb) + (locals.var_pparam_b4soidvt0 * locals.var_theta0zb_dn7)) * locals.var_v0__blk799) + (assign17780_e16047 * locals.var_v0__blk799_dn7)), ((((locals.var_pparam_b4soidvt0_dn8 * locals.var_theta0zb) + (locals.var_pparam_b4soidvt0 * locals.var_theta0zb_dn8)) * locals.var_v0__blk799) + (assign17780_e16047 * locals.var_v0__blk799_dn8)), ((((locals.var_pparam_b4soidvt0_dn9 * locals.var_theta0zb) + (locals.var_pparam_b4soidvt0 * locals.var_theta0zb_dn9)) * locals.var_v0__blk799) + (assign17780_e16047 * locals.var_v0__blk799_dn9)), ((((locals.var_pparam_b4soidvt0_dn10 * locals.var_theta0zb) + (locals.var_pparam_b4soidvt0 * locals.var_theta0zb_dn10)) * locals.var_v0__blk799) + (assign17780_e16047 * locals.var_v0__blk799_dn10)), ((((locals.var_pparam_b4soidvt0_dn11 * locals.var_theta0zb) + (locals.var_pparam_b4soidvt0 * locals.var_theta0zb_dn11)) * locals.var_v0__blk799) + (assign17780_e16047 * locals.var_v0__blk799_dn11)), ((((locals.var_pparam_b4soidvt0_dn12 * locals.var_theta0zb) + (locals.var_pparam_b4soidvt0 * locals.var_theta0zb_dn12)) * locals.var_v0__blk799) + (assign17780_e16047 * locals.var_v0__blk799_dn12)),)
    } else {
        (locals.var_delt_vthzb, locals.var_delt_vthzb_dn3, locals.var_delt_vthzb_dn4, locals.var_delt_vthzb_dn5, locals.var_delt_vthzb_dn6, locals.var_delt_vthzb_dn7, locals.var_delt_vthzb_dn8, locals.var_delt_vthzb_dn9, locals.var_delt_vthzb_dn10, locals.var_delt_vthzb_dn11, locals.var_delt_vthzb_dn12,)
    }
};
        locals.var_delt_vthzb = assign17780_e16051;
        locals.var_delt_vthzb_dn3 = assign17780_e16051_d_n3;
        locals.var_delt_vthzb_dn4 = assign17780_e16051_d_n4;
        locals.var_delt_vthzb_dn5 = assign17780_e16051_d_n5;
        locals.var_delt_vthzb_dn6 = assign17780_e16051_d_n6;
        locals.var_delt_vthzb_dn7 = assign17780_e16051_d_n7;
        locals.var_delt_vthzb_dn8 = assign17780_e16051_d_n8;
        locals.var_delt_vthzb_dn9 = assign17780_e16051_d_n9;
        locals.var_delt_vthzb_dn10 = assign17780_e16051_d_n10;
        locals.var_delt_vthzb_dn11 = assign17780_e16051_d_n11;
        locals.var_delt_vthzb_dn12 = assign17780_e16051_d_n12;
        locals.var_delt_vthzb_rv = 0.0;

        let (assign17790_e16064, assign17790_e16064_d_n3, assign17790_e16064_d_n4, assign17790_e16064_d_n5, assign17790_e16064_d_n6, assign17790_e16064_d_n7, assign17790_e16064_d_n8, assign17790_e16064_d_n9, assign17790_e16064_d_n10, assign17790_e16064_d_n11, assign17790_e16064_d_n12,) = {
    if (locals.var_guard1206 != 0.0) {
        let assign17790_e16054: f64 = (-0.5);
        let assign17790_e16056: f64 = (assign17790_e16054 * locals.var_pparam_b4soidvt1w);
        let assign17790_e16058: f64 = (assign17790_e16056 * locals.var_pparam_b4soiweff);
        let assign17790_e16060: f64 = (assign17790_e16058 * locals.var_leff);
        let assign17790_e16062: f64 = (assign17790_e16060 / locals.var_ltwzb);
        (assign17790_e16062, ((((((((assign17790_e16054 * locals.var_pparam_b4soidvt1w_dn3) * locals.var_pparam_b4soiweff) + (assign17790_e16056 * locals.var_pparam_b4soiweff_dn3)) * locals.var_leff) + (assign17790_e16058 * locals.var_leff_dn3)) * locals.var_ltwzb) - (assign17790_e16060 * locals.var_ltwzb_dn3)) / (locals.var_ltwzb * locals.var_ltwzb)), ((((((((assign17790_e16054 * locals.var_pparam_b4soidvt1w_dn4) * locals.var_pparam_b4soiweff) + (assign17790_e16056 * locals.var_pparam_b4soiweff_dn4)) * locals.var_leff) + (assign17790_e16058 * locals.var_leff_dn4)) * locals.var_ltwzb) - (assign17790_e16060 * locals.var_ltwzb_dn4)) / (locals.var_ltwzb * locals.var_ltwzb)), ((((((((assign17790_e16054 * locals.var_pparam_b4soidvt1w_dn5) * locals.var_pparam_b4soiweff) + (assign17790_e16056 * locals.var_pparam_b4soiweff_dn5)) * locals.var_leff) + (assign17790_e16058 * locals.var_leff_dn5)) * locals.var_ltwzb) - (assign17790_e16060 * locals.var_ltwzb_dn5)) / (locals.var_ltwzb * locals.var_ltwzb)), ((((((((assign17790_e16054 * locals.var_pparam_b4soidvt1w_dn6) * locals.var_pparam_b4soiweff) + (assign17790_e16056 * locals.var_pparam_b4soiweff_dn6)) * locals.var_leff) + (assign17790_e16058 * locals.var_leff_dn6)) * locals.var_ltwzb) - (assign17790_e16060 * locals.var_ltwzb_dn6)) / (locals.var_ltwzb * locals.var_ltwzb)), ((((((((assign17790_e16054 * locals.var_pparam_b4soidvt1w_dn7) * locals.var_pparam_b4soiweff) + (assign17790_e16056 * locals.var_pparam_b4soiweff_dn7)) * locals.var_leff) + (assign17790_e16058 * locals.var_leff_dn7)) * locals.var_ltwzb) - (assign17790_e16060 * locals.var_ltwzb_dn7)) / (locals.var_ltwzb * locals.var_ltwzb)), ((((((((assign17790_e16054 * locals.var_pparam_b4soidvt1w_dn8) * locals.var_pparam_b4soiweff) + (assign17790_e16056 * locals.var_pparam_b4soiweff_dn8)) * locals.var_leff) + (assign17790_e16058 * locals.var_leff_dn8)) * locals.var_ltwzb) - (assign17790_e16060 * locals.var_ltwzb_dn8)) / (locals.var_ltwzb * locals.var_ltwzb)), ((((((((assign17790_e16054 * locals.var_pparam_b4soidvt1w_dn9) * locals.var_pparam_b4soiweff) + (assign17790_e16056 * locals.var_pparam_b4soiweff_dn9)) * locals.var_leff) + (assign17790_e16058 * locals.var_leff_dn9)) * locals.var_ltwzb) - (assign17790_e16060 * locals.var_ltwzb_dn9)) / (locals.var_ltwzb * locals.var_ltwzb)), ((((((((assign17790_e16054 * locals.var_pparam_b4soidvt1w_dn10) * locals.var_pparam_b4soiweff) + (assign17790_e16056 * locals.var_pparam_b4soiweff_dn10)) * locals.var_leff) + (assign17790_e16058 * locals.var_leff_dn10)) * locals.var_ltwzb) - (assign17790_e16060 * locals.var_ltwzb_dn10)) / (locals.var_ltwzb * locals.var_ltwzb)), ((((((((assign17790_e16054 * locals.var_pparam_b4soidvt1w_dn11) * locals.var_pparam_b4soiweff) + (assign17790_e16056 * locals.var_pparam_b4soiweff_dn11)) * locals.var_leff) + (assign17790_e16058 * locals.var_leff_dn11)) * locals.var_ltwzb) - (assign17790_e16060 * locals.var_ltwzb_dn11)) / (locals.var_ltwzb * locals.var_ltwzb)), ((((((((assign17790_e16054 * locals.var_pparam_b4soidvt1w_dn12) * locals.var_pparam_b4soiweff) + (assign17790_e16056 * locals.var_pparam_b4soiweff_dn12)) * locals.var_leff) + (assign17790_e16058 * locals.var_leff_dn12)) * locals.var_ltwzb) - (assign17790_e16060 * locals.var_ltwzb_dn12)) / (locals.var_ltwzb * locals.var_ltwzb)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign17790_e16064;
        locals.var_t0__blk808_dn3 = assign17790_e16064_d_n3;
        locals.var_t0__blk808_dn4 = assign17790_e16064_d_n4;
        locals.var_t0__blk808_dn5 = assign17790_e16064_d_n5;
        locals.var_t0__blk808_dn6 = assign17790_e16064_d_n6;
        locals.var_t0__blk808_dn7 = assign17790_e16064_d_n7;
        locals.var_t0__blk808_dn8 = assign17790_e16064_d_n8;
        locals.var_t0__blk808_dn9 = assign17790_e16064_d_n9;
        locals.var_t0__blk808_dn10 = assign17790_e16064_d_n10;
        locals.var_t0__blk808_dn11 = assign17790_e16064_d_n11;
        locals.var_t0__blk808_dn12 = assign17790_e16064_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let assign17800_e16067: f64 = (-100.0);
        let assign17800_e16068: f64 = if locals.var_t0__blk808 > assign17800_e16067 { 1.0 } else { 0.0 };
        locals.var_guard1208 = assign17800_e16068;
        locals.var_guard1208_rv = 0.0;

        let (assign17810_e16075, assign17810_e16075_d_n3, assign17810_e16075_d_n4, assign17810_e16075_d_n5, assign17810_e16075_d_n6, assign17810_e16075_d_n7, assign17810_e16075_d_n8, assign17810_e16075_d_n9, assign17810_e16075_d_n10, assign17810_e16075_d_n11, assign17810_e16075_d_n12,) = {
    if ((locals.var_guard1206 != 0.0) && (locals.var_guard1208 != 0.0)) {
        let assign17810_e16073: f64 = (locals.var_t0__blk808).exp();
        (assign17810_e16073, (assign17810_e16073 * locals.var_t0__blk808_dn3), (assign17810_e16073 * locals.var_t0__blk808_dn4), (assign17810_e16073 * locals.var_t0__blk808_dn5), (assign17810_e16073 * locals.var_t0__blk808_dn6), (assign17810_e16073 * locals.var_t0__blk808_dn7), (assign17810_e16073 * locals.var_t0__blk808_dn8), (assign17810_e16073 * locals.var_t0__blk808_dn9), (assign17810_e16073 * locals.var_t0__blk808_dn10), (assign17810_e16073 * locals.var_t0__blk808_dn11), (assign17810_e16073 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign17810_e16075;
        locals.var_t1__blk809_dn3 = assign17810_e16075_d_n3;
        locals.var_t1__blk809_dn4 = assign17810_e16075_d_n4;
        locals.var_t1__blk809_dn5 = assign17810_e16075_d_n5;
        locals.var_t1__blk809_dn6 = assign17810_e16075_d_n6;
        locals.var_t1__blk809_dn7 = assign17810_e16075_d_n7;
        locals.var_t1__blk809_dn8 = assign17810_e16075_d_n8;
        locals.var_t1__blk809_dn9 = assign17810_e16075_d_n9;
        locals.var_t1__blk809_dn10 = assign17810_e16075_d_n10;
        locals.var_t1__blk809_dn11 = assign17810_e16075_d_n11;
        locals.var_t1__blk809_dn12 = assign17810_e16075_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign17820_e16087, assign17820_e16087_d_n3, assign17820_e16087_d_n4, assign17820_e16087_d_n5, assign17820_e16087_d_n6, assign17820_e16087_d_n7, assign17820_e16087_d_n8, assign17820_e16087_d_n9, assign17820_e16087_d_n10, assign17820_e16087_d_n11, assign17820_e16087_d_n12,) = {
    if ((locals.var_guard1206 != 0.0) && (locals.var_guard1208 != 0.0)) {
        let assign17820_e16083: f64 = (2.0 * locals.var_t1__blk809);
        let assign17820_e16084: f64 = (1.0 + assign17820_e16083);
        let assign17820_e16085: f64 = (locals.var_t1__blk809 * assign17820_e16084);
        (assign17820_e16085, ((locals.var_t1__blk809_dn3 * assign17820_e16084) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn3))), ((locals.var_t1__blk809_dn4 * assign17820_e16084) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn4))), ((locals.var_t1__blk809_dn5 * assign17820_e16084) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn5))), ((locals.var_t1__blk809_dn6 * assign17820_e16084) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn6))), ((locals.var_t1__blk809_dn7 * assign17820_e16084) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn7))), ((locals.var_t1__blk809_dn8 * assign17820_e16084) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn8))), ((locals.var_t1__blk809_dn9 * assign17820_e16084) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn9))), ((locals.var_t1__blk809_dn10 * assign17820_e16084) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn10))), ((locals.var_t1__blk809_dn11 * assign17820_e16084) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn11))), ((locals.var_t1__blk809_dn12 * assign17820_e16084) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn12))),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign17820_e16087;
        locals.var_t2__blk810_dn3 = assign17820_e16087_d_n3;
        locals.var_t2__blk810_dn4 = assign17820_e16087_d_n4;
        locals.var_t2__blk810_dn5 = assign17820_e16087_d_n5;
        locals.var_t2__blk810_dn6 = assign17820_e16087_d_n6;
        locals.var_t2__blk810_dn7 = assign17820_e16087_d_n7;
        locals.var_t2__blk810_dn8 = assign17820_e16087_d_n8;
        locals.var_t2__blk810_dn9 = assign17820_e16087_d_n9;
        locals.var_t2__blk810_dn10 = assign17820_e16087_d_n10;
        locals.var_t2__blk810_dn11 = assign17820_e16087_d_n11;
        locals.var_t2__blk810_dn12 = assign17820_e16087_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign17830_e16094, assign17830_e16094_d_n3, assign17830_e16094_d_n4, assign17830_e16094_d_n5, assign17830_e16094_d_n6, assign17830_e16094_d_n7, assign17830_e16094_d_n8, assign17830_e16094_d_n9, assign17830_e16094_d_n10, assign17830_e16094_d_n11, assign17830_e16094_d_n12,) = {
    if ((locals.var_guard1206 != 0.0) && (locals.var_guard1208 == 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign17830_e16094;
        locals.var_t1__blk809_dn3 = assign17830_e16094_d_n3;
        locals.var_t1__blk809_dn4 = assign17830_e16094_d_n4;
        locals.var_t1__blk809_dn5 = assign17830_e16094_d_n5;
        locals.var_t1__blk809_dn6 = assign17830_e16094_d_n6;
        locals.var_t1__blk809_dn7 = assign17830_e16094_d_n7;
        locals.var_t1__blk809_dn8 = assign17830_e16094_d_n8;
        locals.var_t1__blk809_dn9 = assign17830_e16094_d_n9;
        locals.var_t1__blk809_dn10 = assign17830_e16094_d_n10;
        locals.var_t1__blk809_dn11 = assign17830_e16094_d_n11;
        locals.var_t1__blk809_dn12 = assign17830_e16094_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign17840_e16107, assign17840_e16107_d_n3, assign17840_e16107_d_n4, assign17840_e16107_d_n5, assign17840_e16107_d_n6, assign17840_e16107_d_n7, assign17840_e16107_d_n8, assign17840_e16107_d_n9, assign17840_e16107_d_n10, assign17840_e16107_d_n11, assign17840_e16107_d_n12,) = {
    if ((locals.var_guard1206 != 0.0) && (locals.var_guard1208 == 0.0)) {
        let assign17840_e16103: f64 = (2.0 * locals.var_t1__blk809);
        let assign17840_e16104: f64 = (1.0 + assign17840_e16103);
        let assign17840_e16105: f64 = (locals.var_t1__blk809 * assign17840_e16104);
        (assign17840_e16105, ((locals.var_t1__blk809_dn3 * assign17840_e16104) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn3))), ((locals.var_t1__blk809_dn4 * assign17840_e16104) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn4))), ((locals.var_t1__blk809_dn5 * assign17840_e16104) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn5))), ((locals.var_t1__blk809_dn6 * assign17840_e16104) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn6))), ((locals.var_t1__blk809_dn7 * assign17840_e16104) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn7))), ((locals.var_t1__blk809_dn8 * assign17840_e16104) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn8))), ((locals.var_t1__blk809_dn9 * assign17840_e16104) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn9))), ((locals.var_t1__blk809_dn10 * assign17840_e16104) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn10))), ((locals.var_t1__blk809_dn11 * assign17840_e16104) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn11))), ((locals.var_t1__blk809_dn12 * assign17840_e16104) + (locals.var_t1__blk809 * (2.0 * locals.var_t1__blk809_dn12))),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign17840_e16107;
        locals.var_t2__blk810_dn3 = assign17840_e16107_d_n3;
        locals.var_t2__blk810_dn4 = assign17840_e16107_d_n4;
        locals.var_t2__blk810_dn5 = assign17840_e16107_d_n5;
        locals.var_t2__blk810_dn6 = assign17840_e16107_d_n6;
        locals.var_t2__blk810_dn7 = assign17840_e16107_d_n7;
        locals.var_t2__blk810_dn8 = assign17840_e16107_d_n8;
        locals.var_t2__blk810_dn9 = assign17840_e16107_d_n9;
        locals.var_t2__blk810_dn10 = assign17840_e16107_d_n10;
        locals.var_t2__blk810_dn11 = assign17840_e16107_d_n11;
        locals.var_t2__blk810_dn12 = assign17840_e16107_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign17850_e16113, assign17850_e16113_d_n3, assign17850_e16113_d_n4, assign17850_e16113_d_n5, assign17850_e16113_d_n6, assign17850_e16113_d_n7, assign17850_e16113_d_n8, assign17850_e16113_d_n9, assign17850_e16113_d_n10, assign17850_e16113_d_n11, assign17850_e16113_d_n12,) = {
    if (locals.var_guard1206 != 0.0) {
        let assign17850_e16111: f64 = (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810);
        (assign17850_e16111, ((locals.var_pparam_b4soidvt0w_dn3 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn3)), ((locals.var_pparam_b4soidvt0w_dn4 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn4)), ((locals.var_pparam_b4soidvt0w_dn5 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn5)), ((locals.var_pparam_b4soidvt0w_dn6 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn6)), ((locals.var_pparam_b4soidvt0w_dn7 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn7)), ((locals.var_pparam_b4soidvt0w_dn8 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn8)), ((locals.var_pparam_b4soidvt0w_dn9 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn9)), ((locals.var_pparam_b4soidvt0w_dn10 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn10)), ((locals.var_pparam_b4soidvt0w_dn11 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn11)), ((locals.var_pparam_b4soidvt0w_dn12 * locals.var_t2__blk810) + (locals.var_pparam_b4soidvt0w * locals.var_t2__blk810_dn12)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign17850_e16113;
        locals.var_t0__blk808_dn3 = assign17850_e16113_d_n3;
        locals.var_t0__blk808_dn4 = assign17850_e16113_d_n4;
        locals.var_t0__blk808_dn5 = assign17850_e16113_d_n5;
        locals.var_t0__blk808_dn6 = assign17850_e16113_d_n6;
        locals.var_t0__blk808_dn7 = assign17850_e16113_d_n7;
        locals.var_t0__blk808_dn8 = assign17850_e16113_d_n8;
        locals.var_t0__blk808_dn9 = assign17850_e16113_d_n9;
        locals.var_t0__blk808_dn10 = assign17850_e16113_d_n10;
        locals.var_t0__blk808_dn11 = assign17850_e16113_d_n11;
        locals.var_t0__blk808_dn12 = assign17850_e16113_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign17860_e16119, assign17860_e16119_d_n3, assign17860_e16119_d_n4, assign17860_e16119_d_n5, assign17860_e16119_d_n6, assign17860_e16119_d_n7, assign17860_e16119_d_n8, assign17860_e16119_d_n9, assign17860_e16119_d_n10, assign17860_e16119_d_n11, assign17860_e16119_d_n12,) = {
    if (locals.var_guard1206 != 0.0) {
        let assign17860_e16117: f64 = (locals.var_t0__blk808 * locals.var_v0__blk799);
        (assign17860_e16117, ((locals.var_t0__blk808_dn3 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn3)), ((locals.var_t0__blk808_dn4 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn4)), ((locals.var_t0__blk808_dn5 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn5)), ((locals.var_t0__blk808_dn6 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn6)), ((locals.var_t0__blk808_dn7 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn7)), ((locals.var_t0__blk808_dn8 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn8)), ((locals.var_t0__blk808_dn9 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn9)), ((locals.var_t0__blk808_dn10 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn10)), ((locals.var_t0__blk808_dn11 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn11)), ((locals.var_t0__blk808_dn12 * locals.var_v0__blk799) + (locals.var_t0__blk808 * locals.var_v0__blk799_dn12)),)
    } else {
        (locals.var_deltvthwzb, locals.var_deltvthwzb_dn3, locals.var_deltvthwzb_dn4, locals.var_deltvthwzb_dn5, locals.var_deltvthwzb_dn6, locals.var_deltvthwzb_dn7, locals.var_deltvthwzb_dn8, locals.var_deltvthwzb_dn9, locals.var_deltvthwzb_dn10, locals.var_deltvthwzb_dn11, locals.var_deltvthwzb_dn12,)
    }
};
        locals.var_deltvthwzb = assign17860_e16119;
        locals.var_deltvthwzb_dn3 = assign17860_e16119_d_n3;
        locals.var_deltvthwzb_dn4 = assign17860_e16119_d_n4;
        locals.var_deltvthwzb_dn5 = assign17860_e16119_d_n5;
        locals.var_deltvthwzb_dn6 = assign17860_e16119_d_n6;
        locals.var_deltvthwzb_dn7 = assign17860_e16119_d_n7;
        locals.var_deltvthwzb_dn8 = assign17860_e16119_d_n8;
        locals.var_deltvthwzb_dn9 = assign17860_e16119_d_n9;
        locals.var_deltvthwzb_dn10 = assign17860_e16119_d_n10;
        locals.var_deltvthwzb_dn11 = assign17860_e16119_d_n11;
        locals.var_deltvthwzb_dn12 = assign17860_e16119_d_n12;
        locals.var_deltvthwzb_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_53(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17870_e16128, assign17870_e16128_d_n3, assign17870_e16128_d_n4, assign17870_e16128_d_n5, assign17870_e16128_d_n6, assign17870_e16128_d_n7, assign17870_e16128_d_n8, assign17870_e16128_d_n9, assign17870_e16128_d_n10, assign17870_e16128_d_n11, assign17870_e16128_d_n12,) = {
    if (locals.var_guard1206 != 0.0) {
        let assign17870_e16124: f64 = (locals.var_pparam_b4soilpe0 / locals.var_leff);
        let assign17870_e16125: f64 = (1.0 + assign17870_e16124);
        let assign17870_e16126: f64 = (assign17870_e16125).sqrt();
        (assign17870_e16126, ((((locals.var_pparam_b4soilpe0_dn3 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn3)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17870_e16126)), ((((locals.var_pparam_b4soilpe0_dn4 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn4)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17870_e16126)), ((((locals.var_pparam_b4soilpe0_dn5 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn5)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17870_e16126)), ((((locals.var_pparam_b4soilpe0_dn6 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn6)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17870_e16126)), ((((locals.var_pparam_b4soilpe0_dn7 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn7)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17870_e16126)), ((((locals.var_pparam_b4soilpe0_dn8 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn8)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17870_e16126)), ((((locals.var_pparam_b4soilpe0_dn9 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn9)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17870_e16126)), ((((locals.var_pparam_b4soilpe0_dn10 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn10)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17870_e16126)), ((((locals.var_pparam_b4soilpe0_dn11 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn11)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17870_e16126)), ((((locals.var_pparam_b4soilpe0_dn12 * locals.var_leff) - (locals.var_pparam_b4soilpe0 * locals.var_leff_dn12)) / (locals.var_leff * locals.var_leff)) / (2.0 * assign17870_e16126)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign17870_e16128;
        locals.var_t0__blk808_dn3 = assign17870_e16128_d_n3;
        locals.var_t0__blk808_dn4 = assign17870_e16128_d_n4;
        locals.var_t0__blk808_dn5 = assign17870_e16128_d_n5;
        locals.var_t0__blk808_dn6 = assign17870_e16128_d_n6;
        locals.var_t0__blk808_dn7 = assign17870_e16128_d_n7;
        locals.var_t0__blk808_dn8 = assign17870_e16128_d_n8;
        locals.var_t0__blk808_dn9 = assign17870_e16128_d_n9;
        locals.var_t0__blk808_dn10 = assign17870_e16128_d_n10;
        locals.var_t0__blk808_dn11 = assign17870_e16128_d_n11;
        locals.var_t0__blk808_dn12 = assign17870_e16128_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign17880_e16136, assign17880_e16136_d_n3, assign17880_e16136_d_n4, assign17880_e16136_d_n5, assign17880_e16136_d_n6, assign17880_e16136_d_n7, assign17880_e16136_d_n8, assign17880_e16136_d_n9, assign17880_e16136_d_n10, assign17880_e16136_d_n11, assign17880_e16136_d_n12,) = {
    if (locals.var_guard1206 != 0.0) {
        let assign17880_e16133: f64 = (locals.var_pparam_b4soikt1l / locals.var_leff);
        let assign17880_e16134: f64 = (locals.var_pparam_b4soikt1 + assign17880_e16133);
        (assign17880_e16134, (locals.var_pparam_b4soikt1_dn3 + (((locals.var_pparam_b4soikt1l_dn3 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn3)) / (locals.var_leff * locals.var_leff))), (locals.var_pparam_b4soikt1_dn4 + (((locals.var_pparam_b4soikt1l_dn4 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn4)) / (locals.var_leff * locals.var_leff))), (locals.var_pparam_b4soikt1_dn5 + (((locals.var_pparam_b4soikt1l_dn5 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn5)) / (locals.var_leff * locals.var_leff))), (locals.var_pparam_b4soikt1_dn6 + (((locals.var_pparam_b4soikt1l_dn6 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn6)) / (locals.var_leff * locals.var_leff))), (locals.var_pparam_b4soikt1_dn7 + (((locals.var_pparam_b4soikt1l_dn7 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn7)) / (locals.var_leff * locals.var_leff))), (locals.var_pparam_b4soikt1_dn8 + (((locals.var_pparam_b4soikt1l_dn8 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn8)) / (locals.var_leff * locals.var_leff))), (locals.var_pparam_b4soikt1_dn9 + (((locals.var_pparam_b4soikt1l_dn9 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn9)) / (locals.var_leff * locals.var_leff))), (locals.var_pparam_b4soikt1_dn10 + (((locals.var_pparam_b4soikt1l_dn10 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn10)) / (locals.var_leff * locals.var_leff))), (locals.var_pparam_b4soikt1_dn11 + (((locals.var_pparam_b4soikt1l_dn11 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn11)) / (locals.var_leff * locals.var_leff))), (locals.var_pparam_b4soikt1_dn12 + (((locals.var_pparam_b4soikt1l_dn12 * locals.var_leff) - (locals.var_pparam_b4soikt1l * locals.var_leff_dn12)) / (locals.var_leff * locals.var_leff))),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign17880_e16136;
        locals.var_t1__blk809_dn3 = assign17880_e16136_d_n3;
        locals.var_t1__blk809_dn4 = assign17880_e16136_d_n4;
        locals.var_t1__blk809_dn5 = assign17880_e16136_d_n5;
        locals.var_t1__blk809_dn6 = assign17880_e16136_d_n6;
        locals.var_t1__blk809_dn7 = assign17880_e16136_d_n7;
        locals.var_t1__blk809_dn8 = assign17880_e16136_d_n8;
        locals.var_t1__blk809_dn9 = assign17880_e16136_d_n9;
        locals.var_t1__blk809_dn10 = assign17880_e16136_d_n10;
        locals.var_t1__blk809_dn11 = assign17880_e16136_d_n11;
        locals.var_t1__blk809_dn12 = assign17880_e16136_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign17890_e16150, assign17890_e16150_d_n3, assign17890_e16150_d_n4, assign17890_e16150_d_n5, assign17890_e16150_d_n6, assign17890_e16150_d_n7, assign17890_e16150_d_n8, assign17890_e16150_d_n9, assign17890_e16150_d_n10, assign17890_e16150_d_n11, assign17890_e16150_d_n12,) = {
    if (locals.var_guard1206 != 0.0) {
        let assign17890_e16141: f64 = (locals.var_t0__blk808 - 1.0);
        let assign17890_e16142: f64 = (locals.var_here_b4soik1ox * assign17890_e16141);
        let assign17890_e16144: f64 = (assign17890_e16142 * locals.var_sqrtphi);
        let assign17890_e16147: f64 = (locals.var_t1__blk809 * locals.var_trm1);
        let assign17890_e16148: f64 = (assign17890_e16144 + assign17890_e16147);
        (assign17890_e16148, (((((locals.var_here_b4soik1ox_dn3 * assign17890_e16141) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn3)) * locals.var_sqrtphi) + (assign17890_e16142 * locals.var_sqrtphi_dn3)) + (locals.var_t1__blk809_dn3 * locals.var_trm1)), (((((locals.var_here_b4soik1ox_dn4 * assign17890_e16141) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn4)) * locals.var_sqrtphi) + (assign17890_e16142 * locals.var_sqrtphi_dn4)) + ((locals.var_t1__blk809_dn4 * locals.var_trm1) + (locals.var_t1__blk809 * locals.var_trm1_dn4))), (((((locals.var_here_b4soik1ox_dn5 * assign17890_e16141) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn5)) * locals.var_sqrtphi) + (assign17890_e16142 * locals.var_sqrtphi_dn5)) + ((locals.var_t1__blk809_dn5 * locals.var_trm1) + (locals.var_t1__blk809 * locals.var_trm1_dn5))), (((((locals.var_here_b4soik1ox_dn6 * assign17890_e16141) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn6)) * locals.var_sqrtphi) + (assign17890_e16142 * locals.var_sqrtphi_dn6)) + ((locals.var_t1__blk809_dn6 * locals.var_trm1) + (locals.var_t1__blk809 * locals.var_trm1_dn6))), (((((locals.var_here_b4soik1ox_dn7 * assign17890_e16141) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn7)) * locals.var_sqrtphi) + (assign17890_e16142 * locals.var_sqrtphi_dn7)) + (locals.var_t1__blk809_dn7 * locals.var_trm1)), (((((locals.var_here_b4soik1ox_dn8 * assign17890_e16141) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn8)) * locals.var_sqrtphi) + (assign17890_e16142 * locals.var_sqrtphi_dn8)) + (locals.var_t1__blk809_dn8 * locals.var_trm1)), (((((locals.var_here_b4soik1ox_dn9 * assign17890_e16141) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn9)) * locals.var_sqrtphi) + (assign17890_e16142 * locals.var_sqrtphi_dn9)) + (locals.var_t1__blk809_dn9 * locals.var_trm1)), (((((locals.var_here_b4soik1ox_dn10 * assign17890_e16141) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn10)) * locals.var_sqrtphi) + (assign17890_e16142 * locals.var_sqrtphi_dn10)) + (locals.var_t1__blk809_dn10 * locals.var_trm1)), (((((locals.var_here_b4soik1ox_dn11 * assign17890_e16141) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn11)) * locals.var_sqrtphi) + (assign17890_e16142 * locals.var_sqrtphi_dn11)) + (locals.var_t1__blk809_dn11 * locals.var_trm1)), (((((locals.var_here_b4soik1ox_dn12 * assign17890_e16141) + (locals.var_here_b4soik1ox * locals.var_t0__blk808_dn12)) * locals.var_sqrtphi) + (assign17890_e16142 * locals.var_sqrtphi_dn12)) + (locals.var_t1__blk809_dn12 * locals.var_trm1)),)
    } else {
        (locals.var_deltvthtempzb, locals.var_deltvthtempzb_dn3, locals.var_deltvthtempzb_dn4, locals.var_deltvthtempzb_dn5, locals.var_deltvthtempzb_dn6, locals.var_deltvthtempzb_dn7, locals.var_deltvthtempzb_dn8, locals.var_deltvthtempzb_dn9, locals.var_deltvthtempzb_dn10, locals.var_deltvthtempzb_dn11, locals.var_deltvthtempzb_dn12,)
    }
};
        locals.var_deltvthtempzb = assign17890_e16150;
        locals.var_deltvthtempzb_dn3 = assign17890_e16150_d_n3;
        locals.var_deltvthtempzb_dn4 = assign17890_e16150_d_n4;
        locals.var_deltvthtempzb_dn5 = assign17890_e16150_d_n5;
        locals.var_deltvthtempzb_dn6 = assign17890_e16150_d_n6;
        locals.var_deltvthtempzb_dn7 = assign17890_e16150_d_n7;
        locals.var_deltvthtempzb_dn8 = assign17890_e16150_d_n8;
        locals.var_deltvthtempzb_dn9 = assign17890_e16150_d_n9;
        locals.var_deltvthtempzb_dn10 = assign17890_e16150_d_n10;
        locals.var_deltvthtempzb_dn11 = assign17890_e16150_d_n11;
        locals.var_deltvthtempzb_dn12 = assign17890_e16150_d_n12;
        locals.var_deltvthtempzb_rv = 0.0;

        let (assign17900_e16166, assign17900_e16166_d_n3, assign17900_e16166_d_n4, assign17900_e16166_d_n5, assign17900_e16166_d_n6, assign17900_e16166_d_n7, assign17900_e16166_d_n8, assign17900_e16166_d_n9, assign17900_e16166_d_n10, assign17900_e16166_d_n11, assign17900_e16166_d_n12,) = {
    if (locals.var_guard1206 != 0.0) {
        let assign17900_e16154: f64 = (p.p37 * locals.var_here_b4soivth0);
        let assign17900_e16156: f64 = (assign17900_e16154 - locals.var_delt_vthzb);
        let assign17900_e16158: f64 = (assign17900_e16156 - locals.var_deltvthwzb);
        let assign17900_e16161: f64 = (locals.var_pparam_b4soik3 * locals.var_tmp2_cv);
        let assign17900_e16162: f64 = (assign17900_e16158 + assign17900_e16161);
        let assign17900_e16164: f64 = (assign17900_e16162 + locals.var_deltvthtempzb);
        (assign17900_e16164, (((((p.p37 * locals.var_here_b4soivth0_dn3) - locals.var_delt_vthzb_dn3) - locals.var_deltvthwzb_dn3) + ((locals.var_pparam_b4soik3_dn3 * locals.var_tmp2_cv) + (locals.var_pparam_b4soik3 * locals.var_tmp2_cv_dn3))) + locals.var_deltvthtempzb_dn3), (((((p.p37 * locals.var_here_b4soivth0_dn4) - locals.var_delt_vthzb_dn4) - locals.var_deltvthwzb_dn4) + ((locals.var_pparam_b4soik3_dn4 * locals.var_tmp2_cv) + (locals.var_pparam_b4soik3 * locals.var_tmp2_cv_dn4))) + locals.var_deltvthtempzb_dn4), (((((p.p37 * locals.var_here_b4soivth0_dn5) - locals.var_delt_vthzb_dn5) - locals.var_deltvthwzb_dn5) + ((locals.var_pparam_b4soik3_dn5 * locals.var_tmp2_cv) + (locals.var_pparam_b4soik3 * locals.var_tmp2_cv_dn5))) + locals.var_deltvthtempzb_dn5), (((((p.p37 * locals.var_here_b4soivth0_dn6) - locals.var_delt_vthzb_dn6) - locals.var_deltvthwzb_dn6) + ((locals.var_pparam_b4soik3_dn6 * locals.var_tmp2_cv) + (locals.var_pparam_b4soik3 * locals.var_tmp2_cv_dn6))) + locals.var_deltvthtempzb_dn6), (((((p.p37 * locals.var_here_b4soivth0_dn7) - locals.var_delt_vthzb_dn7) - locals.var_deltvthwzb_dn7) + ((locals.var_pparam_b4soik3_dn7 * locals.var_tmp2_cv) + (locals.var_pparam_b4soik3 * locals.var_tmp2_cv_dn7))) + locals.var_deltvthtempzb_dn7), (((((p.p37 * locals.var_here_b4soivth0_dn8) - locals.var_delt_vthzb_dn8) - locals.var_deltvthwzb_dn8) + ((locals.var_pparam_b4soik3_dn8 * locals.var_tmp2_cv) + (locals.var_pparam_b4soik3 * locals.var_tmp2_cv_dn8))) + locals.var_deltvthtempzb_dn8), (((((p.p37 * locals.var_here_b4soivth0_dn9) - locals.var_delt_vthzb_dn9) - locals.var_deltvthwzb_dn9) + ((locals.var_pparam_b4soik3_dn9 * locals.var_tmp2_cv) + (locals.var_pparam_b4soik3 * locals.var_tmp2_cv_dn9))) + locals.var_deltvthtempzb_dn9), (((((p.p37 * locals.var_here_b4soivth0_dn10) - locals.var_delt_vthzb_dn10) - locals.var_deltvthwzb_dn10) + ((locals.var_pparam_b4soik3_dn10 * locals.var_tmp2_cv) + (locals.var_pparam_b4soik3 * locals.var_tmp2_cv_dn10))) + locals.var_deltvthtempzb_dn10), (((((p.p37 * locals.var_here_b4soivth0_dn11) - locals.var_delt_vthzb_dn11) - locals.var_deltvthwzb_dn11) + ((locals.var_pparam_b4soik3_dn11 * locals.var_tmp2_cv) + (locals.var_pparam_b4soik3 * locals.var_tmp2_cv_dn11))) + locals.var_deltvthtempzb_dn11), (((((p.p37 * locals.var_here_b4soivth0_dn12) - locals.var_delt_vthzb_dn12) - locals.var_deltvthwzb_dn12) + ((locals.var_pparam_b4soik3_dn12 * locals.var_tmp2_cv) + (locals.var_pparam_b4soik3 * locals.var_tmp2_cv_dn12))) + locals.var_deltvthtempzb_dn12),)
    } else {
        (locals.var_vthzb, locals.var_vthzb_dn3, locals.var_vthzb_dn4, locals.var_vthzb_dn5, locals.var_vthzb_dn6, locals.var_vthzb_dn7, locals.var_vthzb_dn8, locals.var_vthzb_dn9, locals.var_vthzb_dn10, locals.var_vthzb_dn11, locals.var_vthzb_dn12,)
    }
};
        locals.var_vthzb = assign17900_e16166;
        locals.var_vthzb_dn3 = assign17900_e16166_d_n3;
        locals.var_vthzb_dn4 = assign17900_e16166_d_n4;
        locals.var_vthzb_dn5 = assign17900_e16166_d_n5;
        locals.var_vthzb_dn6 = assign17900_e16166_d_n6;
        locals.var_vthzb_dn7 = assign17900_e16166_d_n7;
        locals.var_vthzb_dn8 = assign17900_e16166_d_n8;
        locals.var_vthzb_dn9 = assign17900_e16166_d_n9;
        locals.var_vthzb_dn10 = assign17900_e16166_d_n10;
        locals.var_vthzb_dn11 = assign17900_e16166_d_n11;
        locals.var_vthzb_dn12 = assign17900_e16166_d_n12;
        locals.var_vthzb_rv = 0.0;

        let (assign17910_e16171, assign17910_e16171_d_n3, assign17910_e16171_d_n4, assign17910_e16171_d_n5, assign17910_e16171_d_n6, assign17910_e16171_d_n7, assign17910_e16171_d_n8, assign17910_e16171_d_n9, assign17910_e16171_d_n10, assign17910_e16171_d_n11, assign17910_e16171_d_n12,) = {
    if (locals.var_guard1206 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vthzb, locals.var_vthzb_dn3, locals.var_vthzb_dn4, locals.var_vthzb_dn5, locals.var_vthzb_dn6, locals.var_vthzb_dn7, locals.var_vthzb_dn8, locals.var_vthzb_dn9, locals.var_vthzb_dn10, locals.var_vthzb_dn11, locals.var_vthzb_dn12,)
    }
};
        locals.var_vthzb = assign17910_e16171;
        locals.var_vthzb_dn3 = assign17910_e16171_d_n3;
        locals.var_vthzb_dn4 = assign17910_e16171_d_n4;
        locals.var_vthzb_dn5 = assign17910_e16171_d_n5;
        locals.var_vthzb_dn6 = assign17910_e16171_d_n6;
        locals.var_vthzb_dn7 = assign17910_e16171_d_n7;
        locals.var_vthzb_dn8 = assign17910_e16171_d_n8;
        locals.var_vthzb_dn9 = assign17910_e16171_d_n9;
        locals.var_vthzb_dn10 = assign17910_e16171_d_n10;
        locals.var_vthzb_dn11 = assign17910_e16171_d_n11;
        locals.var_vthzb_dn12 = assign17910_e16171_d_n12;
        locals.var_vthzb_rv = 0.0;

        let assign17920_e16174: f64 = (locals.var_vgs_eff__blk790 - locals.var_vth__blk794);
        locals.var_vgst__blk795 = assign17920_e16174;
        locals.var_vgst__blk795_dn3 = (locals.var_vgs_eff__blk790_dn3 - locals.var_vth__blk794_dn3);
        locals.var_vgst__blk795_dn4 = (locals.var_vgs_eff__blk790_dn4 - locals.var_vth__blk794_dn4);
        locals.var_vgst__blk795_dn5 = (locals.var_vgs_eff__blk790_dn5 - locals.var_vth__blk794_dn5);
        locals.var_vgst__blk795_dn6 = (locals.var_vgs_eff__blk790_dn6 - locals.var_vth__blk794_dn6);
        locals.var_vgst__blk795_dn7 = (locals.var_vgs_eff__blk790_dn7 - locals.var_vth__blk794_dn7);
        locals.var_vgst__blk795_dn8 = (locals.var_vgs_eff__blk790_dn8 - locals.var_vth__blk794_dn8);
        locals.var_vgst__blk795_dn9 = (locals.var_vgs_eff__blk790_dn9 - locals.var_vth__blk794_dn9);
        locals.var_vgst__blk795_dn10 = (locals.var_vgs_eff__blk790_dn10 - locals.var_vth__blk794_dn10);
        locals.var_vgst__blk795_dn11 = (locals.var_vgs_eff__blk790_dn11 - locals.var_vth__blk794_dn11);
        locals.var_vgst__blk795_dn12 = (locals.var_vgs_eff__blk790_dn12 - locals.var_vth__blk794_dn12);
        locals.var_vgst__blk795_rv = 0.0;

        let assign17930_e16177: f64 = (locals.var_n__blk796 * locals.var_vtm);
        locals.var_t10__blk818 = assign17930_e16177;
        locals.var_t10__blk818_dn3 = (locals.var_n__blk796_dn3 * locals.var_vtm);
        locals.var_t10__blk818_dn4 = ((locals.var_n__blk796_dn4 * locals.var_vtm) + (locals.var_n__blk796 * locals.var_vtm_dn4));
        locals.var_t10__blk818_dn5 = ((locals.var_n__blk796_dn5 * locals.var_vtm) + (locals.var_n__blk796 * locals.var_vtm_dn5));
        locals.var_t10__blk818_dn6 = ((locals.var_n__blk796_dn6 * locals.var_vtm) + (locals.var_n__blk796 * locals.var_vtm_dn6));
        locals.var_t10__blk818_dn7 = (locals.var_n__blk796_dn7 * locals.var_vtm);
        locals.var_t10__blk818_dn8 = (locals.var_n__blk796_dn8 * locals.var_vtm);
        locals.var_t10__blk818_dn9 = (locals.var_n__blk796_dn9 * locals.var_vtm);
        locals.var_t10__blk818_dn10 = (locals.var_n__blk796_dn10 * locals.var_vtm);
        locals.var_t10__blk818_dn11 = (locals.var_n__blk796_dn11 * locals.var_vtm);
        locals.var_t10__blk818_dn12 = (locals.var_n__blk796_dn12 * locals.var_vtm);
        locals.var_t10__blk818_rv = 0.0;

        let assign17940_e16180: f64 = (locals.var_pparam_b4soimstar * locals.var_vgst__blk795);
        let assign17940_e16182: f64 = (assign17940_e16180 / locals.var_t10__blk818);
        locals.var_vgstnvt__blk774 = assign17940_e16182;
        locals.var_vgstnvt__blk774_dn3 = (((((locals.var_pparam_b4soimstar_dn3 * locals.var_vgst__blk795) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk795_dn3)) * locals.var_t10__blk818) - (assign17940_e16180 * locals.var_t10__blk818_dn3)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_vgstnvt__blk774_dn4 = (((((locals.var_pparam_b4soimstar_dn4 * locals.var_vgst__blk795) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk795_dn4)) * locals.var_t10__blk818) - (assign17940_e16180 * locals.var_t10__blk818_dn4)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_vgstnvt__blk774_dn5 = (((((locals.var_pparam_b4soimstar_dn5 * locals.var_vgst__blk795) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk795_dn5)) * locals.var_t10__blk818) - (assign17940_e16180 * locals.var_t10__blk818_dn5)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_vgstnvt__blk774_dn6 = (((((locals.var_pparam_b4soimstar_dn6 * locals.var_vgst__blk795) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk795_dn6)) * locals.var_t10__blk818) - (assign17940_e16180 * locals.var_t10__blk818_dn6)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_vgstnvt__blk774_dn7 = (((((locals.var_pparam_b4soimstar_dn7 * locals.var_vgst__blk795) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk795_dn7)) * locals.var_t10__blk818) - (assign17940_e16180 * locals.var_t10__blk818_dn7)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_vgstnvt__blk774_dn8 = (((((locals.var_pparam_b4soimstar_dn8 * locals.var_vgst__blk795) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk795_dn8)) * locals.var_t10__blk818) - (assign17940_e16180 * locals.var_t10__blk818_dn8)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_vgstnvt__blk774_dn9 = (((((locals.var_pparam_b4soimstar_dn9 * locals.var_vgst__blk795) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk795_dn9)) * locals.var_t10__blk818) - (assign17940_e16180 * locals.var_t10__blk818_dn9)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_vgstnvt__blk774_dn10 = (((((locals.var_pparam_b4soimstar_dn10 * locals.var_vgst__blk795) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk795_dn10)) * locals.var_t10__blk818) - (assign17940_e16180 * locals.var_t10__blk818_dn10)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_vgstnvt__blk774_dn11 = (((((locals.var_pparam_b4soimstar_dn11 * locals.var_vgst__blk795) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk795_dn11)) * locals.var_t10__blk818) - (assign17940_e16180 * locals.var_t10__blk818_dn11)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_vgstnvt__blk774_dn12 = (((((locals.var_pparam_b4soimstar_dn12 * locals.var_vgst__blk795) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk795_dn12)) * locals.var_t10__blk818) - (assign17940_e16180 * locals.var_t10__blk818_dn12)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_vgstnvt__blk774_rv = 0.0;

        let assign17950_e16186: f64 = (1.0 - locals.var_pparam_b4soimstar);
        let assign17950_e16188: f64 = (assign17950_e16186 * locals.var_vgst__blk795);
        let assign17950_e16189: f64 = (locals.var_pparam_b4soivoff - assign17950_e16188);
        let assign17950_e16191: f64 = (assign17950_e16189 / locals.var_t10__blk818);
        locals.var_exparg__blk798 = assign17950_e16191;
        locals.var_exparg__blk798_dn3 = ((((locals.var_pparam_b4soivoff_dn3 - (((-locals.var_pparam_b4soimstar_dn3) * locals.var_vgst__blk795) + (assign17950_e16186 * locals.var_vgst__blk795_dn3))) * locals.var_t10__blk818) - (assign17950_e16189 * locals.var_t10__blk818_dn3)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_exparg__blk798_dn4 = ((((locals.var_pparam_b4soivoff_dn4 - (((-locals.var_pparam_b4soimstar_dn4) * locals.var_vgst__blk795) + (assign17950_e16186 * locals.var_vgst__blk795_dn4))) * locals.var_t10__blk818) - (assign17950_e16189 * locals.var_t10__blk818_dn4)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_exparg__blk798_dn5 = ((((locals.var_pparam_b4soivoff_dn5 - (((-locals.var_pparam_b4soimstar_dn5) * locals.var_vgst__blk795) + (assign17950_e16186 * locals.var_vgst__blk795_dn5))) * locals.var_t10__blk818) - (assign17950_e16189 * locals.var_t10__blk818_dn5)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_exparg__blk798_dn6 = ((((locals.var_pparam_b4soivoff_dn6 - (((-locals.var_pparam_b4soimstar_dn6) * locals.var_vgst__blk795) + (assign17950_e16186 * locals.var_vgst__blk795_dn6))) * locals.var_t10__blk818) - (assign17950_e16189 * locals.var_t10__blk818_dn6)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_exparg__blk798_dn7 = ((((locals.var_pparam_b4soivoff_dn7 - (((-locals.var_pparam_b4soimstar_dn7) * locals.var_vgst__blk795) + (assign17950_e16186 * locals.var_vgst__blk795_dn7))) * locals.var_t10__blk818) - (assign17950_e16189 * locals.var_t10__blk818_dn7)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_exparg__blk798_dn8 = ((((locals.var_pparam_b4soivoff_dn8 - (((-locals.var_pparam_b4soimstar_dn8) * locals.var_vgst__blk795) + (assign17950_e16186 * locals.var_vgst__blk795_dn8))) * locals.var_t10__blk818) - (assign17950_e16189 * locals.var_t10__blk818_dn8)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_exparg__blk798_dn9 = ((((locals.var_pparam_b4soivoff_dn9 - (((-locals.var_pparam_b4soimstar_dn9) * locals.var_vgst__blk795) + (assign17950_e16186 * locals.var_vgst__blk795_dn9))) * locals.var_t10__blk818) - (assign17950_e16189 * locals.var_t10__blk818_dn9)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_exparg__blk798_dn10 = ((((locals.var_pparam_b4soivoff_dn10 - (((-locals.var_pparam_b4soimstar_dn10) * locals.var_vgst__blk795) + (assign17950_e16186 * locals.var_vgst__blk795_dn10))) * locals.var_t10__blk818) - (assign17950_e16189 * locals.var_t10__blk818_dn10)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_exparg__blk798_dn11 = ((((locals.var_pparam_b4soivoff_dn11 - (((-locals.var_pparam_b4soimstar_dn11) * locals.var_vgst__blk795) + (assign17950_e16186 * locals.var_vgst__blk795_dn11))) * locals.var_t10__blk818) - (assign17950_e16189 * locals.var_t10__blk818_dn11)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_exparg__blk798_dn12 = ((((locals.var_pparam_b4soivoff_dn12 - (((-locals.var_pparam_b4soimstar_dn12) * locals.var_vgst__blk795) + (assign17950_e16186 * locals.var_vgst__blk795_dn12))) * locals.var_t10__blk818) - (assign17950_e16189 * locals.var_t10__blk818_dn12)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_exparg__blk798_rv = 0.0;

        let assign17960_e16194: f64 = if locals.var_vgstnvt__blk774 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1209 = assign17960_e16194;
        locals.var_guard1209_rv = 0.0;

        let (assign17970_e16198, assign17970_e16198_d_n3, assign17970_e16198_d_n4, assign17970_e16198_d_n5, assign17970_e16198_d_n6, assign17970_e16198_d_n7, assign17970_e16198_d_n8, assign17970_e16198_d_n9, assign17970_e16198_d_n10, assign17970_e16198_d_n11, assign17970_e16198_d_n12,) = {
    if (locals.var_guard1209 != 0.0) {
        (locals.var_vgst__blk795, locals.var_vgst__blk795_dn3, locals.var_vgst__blk795_dn4, locals.var_vgst__blk795_dn5, locals.var_vgst__blk795_dn6, locals.var_vgst__blk795_dn7, locals.var_vgst__blk795_dn8, locals.var_vgst__blk795_dn9, locals.var_vgst__blk795_dn10, locals.var_vgst__blk795_dn11, locals.var_vgst__blk795_dn12,)
    } else {
        (locals.var_vgsteff__blk840, locals.var_vgsteff__blk840_dn3, locals.var_vgsteff__blk840_dn4, locals.var_vgsteff__blk840_dn5, locals.var_vgsteff__blk840_dn6, locals.var_vgsteff__blk840_dn7, locals.var_vgsteff__blk840_dn8, locals.var_vgsteff__blk840_dn9, locals.var_vgsteff__blk840_dn10, locals.var_vgsteff__blk840_dn11, locals.var_vgsteff__blk840_dn12,)
    }
};
        locals.var_vgsteff__blk840 = assign17970_e16198;
        locals.var_vgsteff__blk840_dn3 = assign17970_e16198_d_n3;
        locals.var_vgsteff__blk840_dn4 = assign17970_e16198_d_n4;
        locals.var_vgsteff__blk840_dn5 = assign17970_e16198_d_n5;
        locals.var_vgsteff__blk840_dn6 = assign17970_e16198_d_n6;
        locals.var_vgsteff__blk840_dn7 = assign17970_e16198_d_n7;
        locals.var_vgsteff__blk840_dn8 = assign17970_e16198_d_n8;
        locals.var_vgsteff__blk840_dn9 = assign17970_e16198_d_n9;
        locals.var_vgsteff__blk840_dn10 = assign17970_e16198_d_n10;
        locals.var_vgsteff__blk840_dn11 = assign17970_e16198_d_n11;
        locals.var_vgsteff__blk840_dn12 = assign17970_e16198_d_n12;
        locals.var_vgsteff__blk840_rv = 0.0;

        let (assign17980_e16202, assign17980_e16202_d_n3, assign17980_e16202_d_n4, assign17980_e16202_d_n5, assign17980_e16202_d_n6, assign17980_e16202_d_n7, assign17980_e16202_d_n8, assign17980_e16202_d_n9, assign17980_e16202_d_n10, assign17980_e16202_d_n11, assign17980_e16202_d_n12,) = {
    if (locals.var_guard1209 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_expvgst__blk775, locals.var_expvgst__blk775_dn3, locals.var_expvgst__blk775_dn4, locals.var_expvgst__blk775_dn5, locals.var_expvgst__blk775_dn6, locals.var_expvgst__blk775_dn7, locals.var_expvgst__blk775_dn8, locals.var_expvgst__blk775_dn9, locals.var_expvgst__blk775_dn10, locals.var_expvgst__blk775_dn11, locals.var_expvgst__blk775_dn12,)
    }
};
        locals.var_expvgst__blk775 = assign17980_e16202;
        locals.var_expvgst__blk775_dn3 = assign17980_e16202_d_n3;
        locals.var_expvgst__blk775_dn4 = assign17980_e16202_d_n4;
        locals.var_expvgst__blk775_dn5 = assign17980_e16202_d_n5;
        locals.var_expvgst__blk775_dn6 = assign17980_e16202_d_n6;
        locals.var_expvgst__blk775_dn7 = assign17980_e16202_d_n7;
        locals.var_expvgst__blk775_dn8 = assign17980_e16202_d_n8;
        locals.var_expvgst__blk775_dn9 = assign17980_e16202_d_n9;
        locals.var_expvgst__blk775_dn10 = assign17980_e16202_d_n10;
        locals.var_expvgst__blk775_dn11 = assign17980_e16202_d_n11;
        locals.var_expvgst__blk775_dn12 = assign17980_e16202_d_n12;
        locals.var_expvgst__blk775_rv = 0.0;

        let assign17990_e16205: f64 = if locals.var_exparg__blk798 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1210 = assign17990_e16205;
        locals.var_guard1210_rv = 0.0;

        let (assign18000_e16218, assign18000_e16218_d_n3, assign18000_e16218_d_n4, assign18000_e16218_d_n5, assign18000_e16218_d_n6, assign18000_e16218_d_n7, assign18000_e16218_d_n8, assign18000_e16218_d_n9, assign18000_e16218_d_n10, assign18000_e16218_d_n11, assign18000_e16218_d_n12,) = {
    if ((locals.var_guard1209 == 0.0) && (locals.var_guard1210 != 0.0)) {
        let assign18000_e16212: f64 = (locals.var_vgst__blk795 - locals.var_pparam_b4soivoff);
        let assign18000_e16215: f64 = (locals.var_n__blk796 * locals.var_vtm);
        let assign18000_e16216: f64 = (assign18000_e16212 / assign18000_e16215);
        (assign18000_e16216, ((((locals.var_vgst__blk795_dn3 - locals.var_pparam_b4soivoff_dn3) * assign18000_e16215) - (assign18000_e16212 * (locals.var_n__blk796_dn3 * locals.var_vtm))) / (assign18000_e16215 * assign18000_e16215)), ((((locals.var_vgst__blk795_dn4 - locals.var_pparam_b4soivoff_dn4) * assign18000_e16215) - (assign18000_e16212 * ((locals.var_n__blk796_dn4 * locals.var_vtm) + (locals.var_n__blk796 * locals.var_vtm_dn4)))) / (assign18000_e16215 * assign18000_e16215)), ((((locals.var_vgst__blk795_dn5 - locals.var_pparam_b4soivoff_dn5) * assign18000_e16215) - (assign18000_e16212 * ((locals.var_n__blk796_dn5 * locals.var_vtm) + (locals.var_n__blk796 * locals.var_vtm_dn5)))) / (assign18000_e16215 * assign18000_e16215)), ((((locals.var_vgst__blk795_dn6 - locals.var_pparam_b4soivoff_dn6) * assign18000_e16215) - (assign18000_e16212 * ((locals.var_n__blk796_dn6 * locals.var_vtm) + (locals.var_n__blk796 * locals.var_vtm_dn6)))) / (assign18000_e16215 * assign18000_e16215)), ((((locals.var_vgst__blk795_dn7 - locals.var_pparam_b4soivoff_dn7) * assign18000_e16215) - (assign18000_e16212 * (locals.var_n__blk796_dn7 * locals.var_vtm))) / (assign18000_e16215 * assign18000_e16215)), ((((locals.var_vgst__blk795_dn8 - locals.var_pparam_b4soivoff_dn8) * assign18000_e16215) - (assign18000_e16212 * (locals.var_n__blk796_dn8 * locals.var_vtm))) / (assign18000_e16215 * assign18000_e16215)), ((((locals.var_vgst__blk795_dn9 - locals.var_pparam_b4soivoff_dn9) * assign18000_e16215) - (assign18000_e16212 * (locals.var_n__blk796_dn9 * locals.var_vtm))) / (assign18000_e16215 * assign18000_e16215)), ((((locals.var_vgst__blk795_dn10 - locals.var_pparam_b4soivoff_dn10) * assign18000_e16215) - (assign18000_e16212 * (locals.var_n__blk796_dn10 * locals.var_vtm))) / (assign18000_e16215 * assign18000_e16215)), ((((locals.var_vgst__blk795_dn11 - locals.var_pparam_b4soivoff_dn11) * assign18000_e16215) - (assign18000_e16212 * (locals.var_n__blk796_dn11 * locals.var_vtm))) / (assign18000_e16215 * assign18000_e16215)), ((((locals.var_vgst__blk795_dn12 - locals.var_pparam_b4soivoff_dn12) * assign18000_e16215) - (assign18000_e16212 * (locals.var_n__blk796_dn12 * locals.var_vtm))) / (assign18000_e16215 * assign18000_e16215)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign18000_e16218;
        locals.var_t0__blk808_dn3 = assign18000_e16218_d_n3;
        locals.var_t0__blk808_dn4 = assign18000_e16218_d_n4;
        locals.var_t0__blk808_dn5 = assign18000_e16218_d_n5;
        locals.var_t0__blk808_dn6 = assign18000_e16218_d_n6;
        locals.var_t0__blk808_dn7 = assign18000_e16218_d_n7;
        locals.var_t0__blk808_dn8 = assign18000_e16218_d_n8;
        locals.var_t0__blk808_dn9 = assign18000_e16218_d_n9;
        locals.var_t0__blk808_dn10 = assign18000_e16218_d_n10;
        locals.var_t0__blk808_dn11 = assign18000_e16218_d_n11;
        locals.var_t0__blk808_dn12 = assign18000_e16218_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign18010_e16226, assign18010_e16226_d_n3, assign18010_e16226_d_n4, assign18010_e16226_d_n5, assign18010_e16226_d_n6, assign18010_e16226_d_n7, assign18010_e16226_d_n8, assign18010_e16226_d_n9, assign18010_e16226_d_n10, assign18010_e16226_d_n11, assign18010_e16226_d_n12,) = {
    if ((locals.var_guard1209 == 0.0) && (locals.var_guard1210 != 0.0)) {
        let assign18010_e16224: f64 = (locals.var_t0__blk808).exp();
        (assign18010_e16224, (assign18010_e16224 * locals.var_t0__blk808_dn3), (assign18010_e16224 * locals.var_t0__blk808_dn4), (assign18010_e16224 * locals.var_t0__blk808_dn5), (assign18010_e16224 * locals.var_t0__blk808_dn6), (assign18010_e16224 * locals.var_t0__blk808_dn7), (assign18010_e16224 * locals.var_t0__blk808_dn8), (assign18010_e16224 * locals.var_t0__blk808_dn9), (assign18010_e16224 * locals.var_t0__blk808_dn10), (assign18010_e16224 * locals.var_t0__blk808_dn11), (assign18010_e16224 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_expvgst__blk775, locals.var_expvgst__blk775_dn3, locals.var_expvgst__blk775_dn4, locals.var_expvgst__blk775_dn5, locals.var_expvgst__blk775_dn6, locals.var_expvgst__blk775_dn7, locals.var_expvgst__blk775_dn8, locals.var_expvgst__blk775_dn9, locals.var_expvgst__blk775_dn10, locals.var_expvgst__blk775_dn11, locals.var_expvgst__blk775_dn12,)
    }
};
        locals.var_expvgst__blk775 = assign18010_e16226;
        locals.var_expvgst__blk775_dn3 = assign18010_e16226_d_n3;
        locals.var_expvgst__blk775_dn4 = assign18010_e16226_d_n4;
        locals.var_expvgst__blk775_dn5 = assign18010_e16226_d_n5;
        locals.var_expvgst__blk775_dn6 = assign18010_e16226_d_n6;
        locals.var_expvgst__blk775_dn7 = assign18010_e16226_d_n7;
        locals.var_expvgst__blk775_dn8 = assign18010_e16226_d_n8;
        locals.var_expvgst__blk775_dn9 = assign18010_e16226_d_n9;
        locals.var_expvgst__blk775_dn10 = assign18010_e16226_d_n10;
        locals.var_expvgst__blk775_dn11 = assign18010_e16226_d_n11;
        locals.var_expvgst__blk775_dn12 = assign18010_e16226_d_n12;
        locals.var_expvgst__blk775_rv = 0.0;

        let (assign18020_e16239, assign18020_e16239_d_n3, assign18020_e16239_d_n4, assign18020_e16239_d_n5, assign18020_e16239_d_n6, assign18020_e16239_d_n7, assign18020_e16239_d_n8, assign18020_e16239_d_n9, assign18020_e16239_d_n10, assign18020_e16239_d_n11, assign18020_e16239_d_n12,) = {
    if ((locals.var_guard1209 == 0.0) && (locals.var_guard1210 != 0.0)) {
        let assign18020_e16233: f64 = (locals.var_vtm * locals.var_cdep0);
        let assign18020_e16235: f64 = (assign18020_e16233 / locals.var_b4soicox);
        let assign18020_e16237: f64 = (assign18020_e16235 * locals.var_expvgst__blk775);
        (assign18020_e16237, ((((locals.var_vtm * locals.var_cdep0_dn3) / locals.var_b4soicox) * locals.var_expvgst__blk775) + (assign18020_e16235 * locals.var_expvgst__blk775_dn3)), (((((locals.var_vtm_dn4 * locals.var_cdep0) + (locals.var_vtm * locals.var_cdep0_dn4)) / locals.var_b4soicox) * locals.var_expvgst__blk775) + (assign18020_e16235 * locals.var_expvgst__blk775_dn4)), (((((locals.var_vtm_dn5 * locals.var_cdep0) + (locals.var_vtm * locals.var_cdep0_dn5)) / locals.var_b4soicox) * locals.var_expvgst__blk775) + (assign18020_e16235 * locals.var_expvgst__blk775_dn5)), (((((locals.var_vtm_dn6 * locals.var_cdep0) + (locals.var_vtm * locals.var_cdep0_dn6)) / locals.var_b4soicox) * locals.var_expvgst__blk775) + (assign18020_e16235 * locals.var_expvgst__blk775_dn6)), ((((locals.var_vtm * locals.var_cdep0_dn7) / locals.var_b4soicox) * locals.var_expvgst__blk775) + (assign18020_e16235 * locals.var_expvgst__blk775_dn7)), ((((locals.var_vtm * locals.var_cdep0_dn8) / locals.var_b4soicox) * locals.var_expvgst__blk775) + (assign18020_e16235 * locals.var_expvgst__blk775_dn8)), ((((locals.var_vtm * locals.var_cdep0_dn9) / locals.var_b4soicox) * locals.var_expvgst__blk775) + (assign18020_e16235 * locals.var_expvgst__blk775_dn9)), ((((locals.var_vtm * locals.var_cdep0_dn10) / locals.var_b4soicox) * locals.var_expvgst__blk775) + (assign18020_e16235 * locals.var_expvgst__blk775_dn10)), ((((locals.var_vtm * locals.var_cdep0_dn11) / locals.var_b4soicox) * locals.var_expvgst__blk775) + (assign18020_e16235 * locals.var_expvgst__blk775_dn11)), ((((locals.var_vtm * locals.var_cdep0_dn12) / locals.var_b4soicox) * locals.var_expvgst__blk775) + (assign18020_e16235 * locals.var_expvgst__blk775_dn12)),)
    } else {
        (locals.var_vgsteff__blk840, locals.var_vgsteff__blk840_dn3, locals.var_vgsteff__blk840_dn4, locals.var_vgsteff__blk840_dn5, locals.var_vgsteff__blk840_dn6, locals.var_vgsteff__blk840_dn7, locals.var_vgsteff__blk840_dn8, locals.var_vgsteff__blk840_dn9, locals.var_vgsteff__blk840_dn10, locals.var_vgsteff__blk840_dn11, locals.var_vgsteff__blk840_dn12,)
    }
};
        locals.var_vgsteff__blk840 = assign18020_e16239;
        locals.var_vgsteff__blk840_dn3 = assign18020_e16239_d_n3;
        locals.var_vgsteff__blk840_dn4 = assign18020_e16239_d_n4;
        locals.var_vgsteff__blk840_dn5 = assign18020_e16239_d_n5;
        locals.var_vgsteff__blk840_dn6 = assign18020_e16239_d_n6;
        locals.var_vgsteff__blk840_dn7 = assign18020_e16239_d_n7;
        locals.var_vgsteff__blk840_dn8 = assign18020_e16239_d_n8;
        locals.var_vgsteff__blk840_dn9 = assign18020_e16239_d_n9;
        locals.var_vgsteff__blk840_dn10 = assign18020_e16239_d_n10;
        locals.var_vgsteff__blk840_dn11 = assign18020_e16239_d_n11;
        locals.var_vgsteff__blk840_dn12 = assign18020_e16239_d_n12;
        locals.var_vgsteff__blk840_rv = 0.0;

        let (assign18030_e16248, assign18030_e16248_d_n3, assign18030_e16248_d_n4, assign18030_e16248_d_n5, assign18030_e16248_d_n6, assign18030_e16248_d_n7, assign18030_e16248_d_n8, assign18030_e16248_d_n9, assign18030_e16248_d_n10, assign18030_e16248_d_n11, assign18030_e16248_d_n12,) = {
    if ((locals.var_guard1209 == 0.0) && (locals.var_guard1210 == 0.0)) {
        let assign18030_e16246: f64 = (locals.var_vgstnvt__blk774).exp();
        (assign18030_e16246, (assign18030_e16246 * locals.var_vgstnvt__blk774_dn3), (assign18030_e16246 * locals.var_vgstnvt__blk774_dn4), (assign18030_e16246 * locals.var_vgstnvt__blk774_dn5), (assign18030_e16246 * locals.var_vgstnvt__blk774_dn6), (assign18030_e16246 * locals.var_vgstnvt__blk774_dn7), (assign18030_e16246 * locals.var_vgstnvt__blk774_dn8), (assign18030_e16246 * locals.var_vgstnvt__blk774_dn9), (assign18030_e16246 * locals.var_vgstnvt__blk774_dn10), (assign18030_e16246 * locals.var_vgstnvt__blk774_dn11), (assign18030_e16246 * locals.var_vgstnvt__blk774_dn12),)
    } else {
        (locals.var_expvgst__blk775, locals.var_expvgst__blk775_dn3, locals.var_expvgst__blk775_dn4, locals.var_expvgst__blk775_dn5, locals.var_expvgst__blk775_dn6, locals.var_expvgst__blk775_dn7, locals.var_expvgst__blk775_dn8, locals.var_expvgst__blk775_dn9, locals.var_expvgst__blk775_dn10, locals.var_expvgst__blk775_dn11, locals.var_expvgst__blk775_dn12,)
    }
};
        locals.var_expvgst__blk775 = assign18030_e16248;
        locals.var_expvgst__blk775_dn3 = assign18030_e16248_d_n3;
        locals.var_expvgst__blk775_dn4 = assign18030_e16248_d_n4;
        locals.var_expvgst__blk775_dn5 = assign18030_e16248_d_n5;
        locals.var_expvgst__blk775_dn6 = assign18030_e16248_d_n6;
        locals.var_expvgst__blk775_dn7 = assign18030_e16248_d_n7;
        locals.var_expvgst__blk775_dn8 = assign18030_e16248_d_n8;
        locals.var_expvgst__blk775_dn9 = assign18030_e16248_d_n9;
        locals.var_expvgst__blk775_dn10 = assign18030_e16248_d_n10;
        locals.var_expvgst__blk775_dn11 = assign18030_e16248_d_n11;
        locals.var_expvgst__blk775_dn12 = assign18030_e16248_d_n12;
        locals.var_expvgst__blk775_rv = 0.0;

        let (assign18040_e16261, assign18040_e16261_d_n3, assign18040_e16261_d_n4, assign18040_e16261_d_n5, assign18040_e16261_d_n6, assign18040_e16261_d_n7, assign18040_e16261_d_n8, assign18040_e16261_d_n9, assign18040_e16261_d_n10, assign18040_e16261_d_n11, assign18040_e16261_d_n12,) = {
    if ((locals.var_guard1209 == 0.0) && (locals.var_guard1210 == 0.0)) {
        let assign18040_e16257: f64 = (1.0 + locals.var_expvgst__blk775);
        let assign18040_e16258: f64 = (assign18040_e16257).ln();
        let assign18040_e16259: f64 = (locals.var_t10__blk818 * assign18040_e16258);
        (assign18040_e16259, ((locals.var_t10__blk818_dn3 * assign18040_e16258) + (locals.var_t10__blk818 * (locals.var_expvgst__blk775_dn3 / assign18040_e16257))), ((locals.var_t10__blk818_dn4 * assign18040_e16258) + (locals.var_t10__blk818 * (locals.var_expvgst__blk775_dn4 / assign18040_e16257))), ((locals.var_t10__blk818_dn5 * assign18040_e16258) + (locals.var_t10__blk818 * (locals.var_expvgst__blk775_dn5 / assign18040_e16257))), ((locals.var_t10__blk818_dn6 * assign18040_e16258) + (locals.var_t10__blk818 * (locals.var_expvgst__blk775_dn6 / assign18040_e16257))), ((locals.var_t10__blk818_dn7 * assign18040_e16258) + (locals.var_t10__blk818 * (locals.var_expvgst__blk775_dn7 / assign18040_e16257))), ((locals.var_t10__blk818_dn8 * assign18040_e16258) + (locals.var_t10__blk818 * (locals.var_expvgst__blk775_dn8 / assign18040_e16257))), ((locals.var_t10__blk818_dn9 * assign18040_e16258) + (locals.var_t10__blk818 * (locals.var_expvgst__blk775_dn9 / assign18040_e16257))), ((locals.var_t10__blk818_dn10 * assign18040_e16258) + (locals.var_t10__blk818 * (locals.var_expvgst__blk775_dn10 / assign18040_e16257))), ((locals.var_t10__blk818_dn11 * assign18040_e16258) + (locals.var_t10__blk818 * (locals.var_expvgst__blk775_dn11 / assign18040_e16257))), ((locals.var_t10__blk818_dn12 * assign18040_e16258) + (locals.var_t10__blk818 * (locals.var_expvgst__blk775_dn12 / assign18040_e16257))),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign18040_e16261;
        locals.var_t1__blk809_dn3 = assign18040_e16261_d_n3;
        locals.var_t1__blk809_dn4 = assign18040_e16261_d_n4;
        locals.var_t1__blk809_dn5 = assign18040_e16261_d_n5;
        locals.var_t1__blk809_dn6 = assign18040_e16261_d_n6;
        locals.var_t1__blk809_dn7 = assign18040_e16261_d_n7;
        locals.var_t1__blk809_dn8 = assign18040_e16261_d_n8;
        locals.var_t1__blk809_dn9 = assign18040_e16261_d_n9;
        locals.var_t1__blk809_dn10 = assign18040_e16261_d_n10;
        locals.var_t1__blk809_dn11 = assign18040_e16261_d_n11;
        locals.var_t1__blk809_dn12 = assign18040_e16261_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign18050_e16281, assign18050_e16281_d_n3, assign18050_e16281_d_n4, assign18050_e16281_d_n5, assign18050_e16281_d_n6, assign18050_e16281_d_n7, assign18050_e16281_d_n8, assign18050_e16281_d_n9, assign18050_e16281_d_n10, assign18050_e16281_d_n11, assign18050_e16281_d_n12,) = {
    if ((locals.var_guard1209 == 0.0) && (locals.var_guard1210 == 0.0)) {
        let assign18050_e16268: f64 = (-locals.var_b4soicox);
        let assign18050_e16271: f64 = (locals.var_vtm * locals.var_cdep0);
        let assign18050_e16272: f64 = (assign18050_e16268 / assign18050_e16271);
        let assign18050_e16274: f64 = (locals.var_exparg__blk798).exp();
        let assign18050_e16275: f64 = (assign18050_e16272 * assign18050_e16274);
        let assign18050_e16278: f64 = (1.0 - locals.var_pparam_b4soimstar);
        let assign18050_e16279: f64 = (assign18050_e16275 * assign18050_e16278);
        (assign18050_e16279, (((((-((assign18050_e16268 * (locals.var_vtm * locals.var_cdep0_dn3)) / (assign18050_e16271 * assign18050_e16271))) * assign18050_e16274) + (assign18050_e16272 * (assign18050_e16274 * locals.var_exparg__blk798_dn3))) * assign18050_e16278) + (assign18050_e16275 * (-locals.var_pparam_b4soimstar_dn3))), (((((-((assign18050_e16268 * ((locals.var_vtm_dn4 * locals.var_cdep0) + (locals.var_vtm * locals.var_cdep0_dn4))) / (assign18050_e16271 * assign18050_e16271))) * assign18050_e16274) + (assign18050_e16272 * (assign18050_e16274 * locals.var_exparg__blk798_dn4))) * assign18050_e16278) + (assign18050_e16275 * (-locals.var_pparam_b4soimstar_dn4))), (((((-((assign18050_e16268 * ((locals.var_vtm_dn5 * locals.var_cdep0) + (locals.var_vtm * locals.var_cdep0_dn5))) / (assign18050_e16271 * assign18050_e16271))) * assign18050_e16274) + (assign18050_e16272 * (assign18050_e16274 * locals.var_exparg__blk798_dn5))) * assign18050_e16278) + (assign18050_e16275 * (-locals.var_pparam_b4soimstar_dn5))), (((((-((assign18050_e16268 * ((locals.var_vtm_dn6 * locals.var_cdep0) + (locals.var_vtm * locals.var_cdep0_dn6))) / (assign18050_e16271 * assign18050_e16271))) * assign18050_e16274) + (assign18050_e16272 * (assign18050_e16274 * locals.var_exparg__blk798_dn6))) * assign18050_e16278) + (assign18050_e16275 * (-locals.var_pparam_b4soimstar_dn6))), (((((-((assign18050_e16268 * (locals.var_vtm * locals.var_cdep0_dn7)) / (assign18050_e16271 * assign18050_e16271))) * assign18050_e16274) + (assign18050_e16272 * (assign18050_e16274 * locals.var_exparg__blk798_dn7))) * assign18050_e16278) + (assign18050_e16275 * (-locals.var_pparam_b4soimstar_dn7))), (((((-((assign18050_e16268 * (locals.var_vtm * locals.var_cdep0_dn8)) / (assign18050_e16271 * assign18050_e16271))) * assign18050_e16274) + (assign18050_e16272 * (assign18050_e16274 * locals.var_exparg__blk798_dn8))) * assign18050_e16278) + (assign18050_e16275 * (-locals.var_pparam_b4soimstar_dn8))), (((((-((assign18050_e16268 * (locals.var_vtm * locals.var_cdep0_dn9)) / (assign18050_e16271 * assign18050_e16271))) * assign18050_e16274) + (assign18050_e16272 * (assign18050_e16274 * locals.var_exparg__blk798_dn9))) * assign18050_e16278) + (assign18050_e16275 * (-locals.var_pparam_b4soimstar_dn9))), (((((-((assign18050_e16268 * (locals.var_vtm * locals.var_cdep0_dn10)) / (assign18050_e16271 * assign18050_e16271))) * assign18050_e16274) + (assign18050_e16272 * (assign18050_e16274 * locals.var_exparg__blk798_dn10))) * assign18050_e16278) + (assign18050_e16275 * (-locals.var_pparam_b4soimstar_dn10))), (((((-((assign18050_e16268 * (locals.var_vtm * locals.var_cdep0_dn11)) / (assign18050_e16271 * assign18050_e16271))) * assign18050_e16274) + (assign18050_e16272 * (assign18050_e16274 * locals.var_exparg__blk798_dn11))) * assign18050_e16278) + (assign18050_e16275 * (-locals.var_pparam_b4soimstar_dn11))), (((((-((assign18050_e16268 * (locals.var_vtm * locals.var_cdep0_dn12)) / (assign18050_e16271 * assign18050_e16271))) * assign18050_e16274) + (assign18050_e16272 * (assign18050_e16274 * locals.var_exparg__blk798_dn12))) * assign18050_e16278) + (assign18050_e16275 * (-locals.var_pparam_b4soimstar_dn12))),)
    } else {
        (locals.var_dt2_dvg, locals.var_dt2_dvg_dn3, locals.var_dt2_dvg_dn4, locals.var_dt2_dvg_dn5, locals.var_dt2_dvg_dn6, locals.var_dt2_dvg_dn7, locals.var_dt2_dvg_dn8, locals.var_dt2_dvg_dn9, locals.var_dt2_dvg_dn10, locals.var_dt2_dvg_dn11, locals.var_dt2_dvg_dn12,)
    }
};
        locals.var_dt2_dvg = assign18050_e16281;
        locals.var_dt2_dvg_dn3 = assign18050_e16281_d_n3;
        locals.var_dt2_dvg_dn4 = assign18050_e16281_d_n4;
        locals.var_dt2_dvg_dn5 = assign18050_e16281_d_n5;
        locals.var_dt2_dvg_dn6 = assign18050_e16281_d_n6;
        locals.var_dt2_dvg_dn7 = assign18050_e16281_d_n7;
        locals.var_dt2_dvg_dn8 = assign18050_e16281_d_n8;
        locals.var_dt2_dvg_dn9 = assign18050_e16281_d_n9;
        locals.var_dt2_dvg_dn10 = assign18050_e16281_d_n10;
        locals.var_dt2_dvg_dn11 = assign18050_e16281_d_n11;
        locals.var_dt2_dvg_dn12 = assign18050_e16281_d_n12;
        locals.var_dt2_dvg_rv = 0.0;

        let (assign18060_e16297, assign18060_e16297_d_n3, assign18060_e16297_d_n4, assign18060_e16297_d_n5, assign18060_e16297_d_n6, assign18060_e16297_d_n7, assign18060_e16297_d_n8, assign18060_e16297_d_n9, assign18060_e16297_d_n10, assign18060_e16297_d_n11, assign18060_e16297_d_n12,) = {
    if ((locals.var_guard1209 == 0.0) && (locals.var_guard1210 == 0.0)) {
        let assign18060_e16290: f64 = (locals.var_t10__blk818 * locals.var_dt2_dvg);
        let assign18060_e16293: f64 = (1.0 - locals.var_pparam_b4soimstar);
        let assign18060_e16294: f64 = (assign18060_e16290 / assign18060_e16293);
        let assign18060_e16295: f64 = (locals.var_pparam_b4soimstar - assign18060_e16294);
        (assign18060_e16295, (locals.var_pparam_b4soimstar_dn3 - (((((locals.var_t10__blk818_dn3 * locals.var_dt2_dvg) + (locals.var_t10__blk818 * locals.var_dt2_dvg_dn3)) * assign18060_e16293) - (assign18060_e16290 * (-locals.var_pparam_b4soimstar_dn3))) / (assign18060_e16293 * assign18060_e16293))), (locals.var_pparam_b4soimstar_dn4 - (((((locals.var_t10__blk818_dn4 * locals.var_dt2_dvg) + (locals.var_t10__blk818 * locals.var_dt2_dvg_dn4)) * assign18060_e16293) - (assign18060_e16290 * (-locals.var_pparam_b4soimstar_dn4))) / (assign18060_e16293 * assign18060_e16293))), (locals.var_pparam_b4soimstar_dn5 - (((((locals.var_t10__blk818_dn5 * locals.var_dt2_dvg) + (locals.var_t10__blk818 * locals.var_dt2_dvg_dn5)) * assign18060_e16293) - (assign18060_e16290 * (-locals.var_pparam_b4soimstar_dn5))) / (assign18060_e16293 * assign18060_e16293))), (locals.var_pparam_b4soimstar_dn6 - (((((locals.var_t10__blk818_dn6 * locals.var_dt2_dvg) + (locals.var_t10__blk818 * locals.var_dt2_dvg_dn6)) * assign18060_e16293) - (assign18060_e16290 * (-locals.var_pparam_b4soimstar_dn6))) / (assign18060_e16293 * assign18060_e16293))), (locals.var_pparam_b4soimstar_dn7 - (((((locals.var_t10__blk818_dn7 * locals.var_dt2_dvg) + (locals.var_t10__blk818 * locals.var_dt2_dvg_dn7)) * assign18060_e16293) - (assign18060_e16290 * (-locals.var_pparam_b4soimstar_dn7))) / (assign18060_e16293 * assign18060_e16293))), (locals.var_pparam_b4soimstar_dn8 - (((((locals.var_t10__blk818_dn8 * locals.var_dt2_dvg) + (locals.var_t10__blk818 * locals.var_dt2_dvg_dn8)) * assign18060_e16293) - (assign18060_e16290 * (-locals.var_pparam_b4soimstar_dn8))) / (assign18060_e16293 * assign18060_e16293))), (locals.var_pparam_b4soimstar_dn9 - (((((locals.var_t10__blk818_dn9 * locals.var_dt2_dvg) + (locals.var_t10__blk818 * locals.var_dt2_dvg_dn9)) * assign18060_e16293) - (assign18060_e16290 * (-locals.var_pparam_b4soimstar_dn9))) / (assign18060_e16293 * assign18060_e16293))), (locals.var_pparam_b4soimstar_dn10 - (((((locals.var_t10__blk818_dn10 * locals.var_dt2_dvg) + (locals.var_t10__blk818 * locals.var_dt2_dvg_dn10)) * assign18060_e16293) - (assign18060_e16290 * (-locals.var_pparam_b4soimstar_dn10))) / (assign18060_e16293 * assign18060_e16293))), (locals.var_pparam_b4soimstar_dn11 - (((((locals.var_t10__blk818_dn11 * locals.var_dt2_dvg) + (locals.var_t10__blk818 * locals.var_dt2_dvg_dn11)) * assign18060_e16293) - (assign18060_e16290 * (-locals.var_pparam_b4soimstar_dn11))) / (assign18060_e16293 * assign18060_e16293))), (locals.var_pparam_b4soimstar_dn12 - (((((locals.var_t10__blk818_dn12 * locals.var_dt2_dvg) + (locals.var_t10__blk818 * locals.var_dt2_dvg_dn12)) * assign18060_e16293) - (assign18060_e16290 * (-locals.var_pparam_b4soimstar_dn12))) / (assign18060_e16293 * assign18060_e16293))),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign18060_e16297;
        locals.var_t2__blk810_dn3 = assign18060_e16297_d_n3;
        locals.var_t2__blk810_dn4 = assign18060_e16297_d_n4;
        locals.var_t2__blk810_dn5 = assign18060_e16297_d_n5;
        locals.var_t2__blk810_dn6 = assign18060_e16297_d_n6;
        locals.var_t2__blk810_dn7 = assign18060_e16297_d_n7;
        locals.var_t2__blk810_dn8 = assign18060_e16297_d_n8;
        locals.var_t2__blk810_dn9 = assign18060_e16297_d_n9;
        locals.var_t2__blk810_dn10 = assign18060_e16297_d_n10;
        locals.var_t2__blk810_dn11 = assign18060_e16297_d_n11;
        locals.var_t2__blk810_dn12 = assign18060_e16297_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign18070_e16307, assign18070_e16307_d_n3, assign18070_e16307_d_n4, assign18070_e16307_d_n5, assign18070_e16307_d_n6, assign18070_e16307_d_n7, assign18070_e16307_d_n8, assign18070_e16307_d_n9, assign18070_e16307_d_n10, assign18070_e16307_d_n11, assign18070_e16307_d_n12,) = {
    if ((locals.var_guard1209 == 0.0) && (locals.var_guard1210 == 0.0)) {
        let assign18070_e16305: f64 = (locals.var_t1__blk809 / locals.var_t2__blk810);
        (assign18070_e16305, (((locals.var_t1__blk809_dn3 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn3)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn4 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn4)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn5 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn5)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn6 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn6)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn7 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn7)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn8 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn8)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn9 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn9)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn10 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn10)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn11 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn11)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn12 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn12)) / (locals.var_t2__blk810 * locals.var_t2__blk810)),)
    } else {
        (locals.var_vgsteff__blk840, locals.var_vgsteff__blk840_dn3, locals.var_vgsteff__blk840_dn4, locals.var_vgsteff__blk840_dn5, locals.var_vgsteff__blk840_dn6, locals.var_vgsteff__blk840_dn7, locals.var_vgsteff__blk840_dn8, locals.var_vgsteff__blk840_dn9, locals.var_vgsteff__blk840_dn10, locals.var_vgsteff__blk840_dn11, locals.var_vgsteff__blk840_dn12,)
    }
};
        locals.var_vgsteff__blk840 = assign18070_e16307;
        locals.var_vgsteff__blk840_dn3 = assign18070_e16307_d_n3;
        locals.var_vgsteff__blk840_dn4 = assign18070_e16307_d_n4;
        locals.var_vgsteff__blk840_dn5 = assign18070_e16307_d_n5;
        locals.var_vgsteff__blk840_dn6 = assign18070_e16307_d_n6;
        locals.var_vgsteff__blk840_dn7 = assign18070_e16307_d_n7;
        locals.var_vgsteff__blk840_dn8 = assign18070_e16307_d_n8;
        locals.var_vgsteff__blk840_dn9 = assign18070_e16307_d_n9;
        locals.var_vgsteff__blk840_dn10 = assign18070_e16307_d_n10;
        locals.var_vgsteff__blk840_dn11 = assign18070_e16307_d_n11;
        locals.var_vgsteff__blk840_dn12 = assign18070_e16307_d_n12;
        locals.var_vgsteff__blk840_rv = 0.0;

        let assign18080_e16311: f64 = (2.0 * locals.var_vtm);
        let assign18080_e16312: f64 = (locals.var_vgsteff__blk840 + assign18080_e16311);
        locals.var_vgst2vtm = assign18080_e16312;
        locals.var_vgst2vtm_dn3 = locals.var_vgsteff__blk840_dn3;
        locals.var_vgst2vtm_dn4 = (locals.var_vgsteff__blk840_dn4 + (2.0 * locals.var_vtm_dn4));
        locals.var_vgst2vtm_dn5 = (locals.var_vgsteff__blk840_dn5 + (2.0 * locals.var_vtm_dn5));
        locals.var_vgst2vtm_dn6 = (locals.var_vgsteff__blk840_dn6 + (2.0 * locals.var_vtm_dn6));
        locals.var_vgst2vtm_dn7 = locals.var_vgsteff__blk840_dn7;
        locals.var_vgst2vtm_dn8 = locals.var_vgsteff__blk840_dn8;
        locals.var_vgst2vtm_dn9 = locals.var_vgsteff__blk840_dn9;
        locals.var_vgst2vtm_dn10 = locals.var_vgsteff__blk840_dn10;
        locals.var_vgst2vtm_dn11 = locals.var_vgsteff__blk840_dn11;
        locals.var_vgst2vtm_dn12 = locals.var_vgsteff__blk840_dn12;
        locals.var_vgst2vtm_rv = 0.0;

        locals.var_b4soivgsteff = locals.var_vgsteff__blk840;
        locals.var_b4soivgsteff_dn3 = locals.var_vgsteff__blk840_dn3;
        locals.var_b4soivgsteff_dn4 = locals.var_vgsteff__blk840_dn4;
        locals.var_b4soivgsteff_dn5 = locals.var_vgsteff__blk840_dn5;
        locals.var_b4soivgsteff_dn6 = locals.var_vgsteff__blk840_dn6;
        locals.var_b4soivgsteff_dn7 = locals.var_vgsteff__blk840_dn7;
        locals.var_b4soivgsteff_dn8 = locals.var_vgsteff__blk840_dn8;
        locals.var_b4soivgsteff_dn9 = locals.var_vgsteff__blk840_dn9;
        locals.var_b4soivgsteff_dn10 = locals.var_vgsteff__blk840_dn10;
        locals.var_b4soivgsteff_dn11 = locals.var_vgsteff__blk840_dn11;
        locals.var_b4soivgsteff_dn12 = locals.var_vgsteff__blk840_dn12;
        locals.var_b4soivgsteff_rv = 0.0;

        let assign18100_e16316: f64 = if locals.var_pparam_b4soifprout <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1211 = assign18100_e16316;
        locals.var_guard1211_rv = 0.0;

        let (assign18110_e16320, assign18110_e16320_d_n3, assign18110_e16320_d_n4, assign18110_e16320_d_n5, assign18110_e16320_d_n6, assign18110_e16320_d_n7, assign18110_e16320_d_n8, assign18110_e16320_d_n9, assign18110_e16320_d_n10, assign18110_e16320_d_n11, assign18110_e16320_d_n12,) = {
    if (locals.var_guard1211 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fp, locals.var_fp_dn3, locals.var_fp_dn4, locals.var_fp_dn5, locals.var_fp_dn6, locals.var_fp_dn7, locals.var_fp_dn8, locals.var_fp_dn9, locals.var_fp_dn10, locals.var_fp_dn11, locals.var_fp_dn12,)
    }
};
        locals.var_fp = assign18110_e16320;
        locals.var_fp_dn3 = assign18110_e16320_d_n3;
        locals.var_fp_dn4 = assign18110_e16320_d_n4;
        locals.var_fp_dn5 = assign18110_e16320_d_n5;
        locals.var_fp_dn6 = assign18110_e16320_d_n6;
        locals.var_fp_dn7 = assign18110_e16320_d_n7;
        locals.var_fp_dn8 = assign18110_e16320_d_n8;
        locals.var_fp_dn9 = assign18110_e16320_d_n9;
        locals.var_fp_dn10 = assign18110_e16320_d_n10;
        locals.var_fp_dn11 = assign18110_e16320_d_n11;
        locals.var_fp_dn12 = assign18110_e16320_d_n12;
        locals.var_fp_rv = 0.0;

        let (assign18120_e16330, assign18120_e16330_d_n3, assign18120_e16330_d_n4, assign18120_e16330_d_n5, assign18120_e16330_d_n6, assign18120_e16330_d_n7, assign18120_e16330_d_n8, assign18120_e16330_d_n9, assign18120_e16330_d_n10, assign18120_e16330_d_n11, assign18120_e16330_d_n12,) = {
    if (locals.var_guard1211 == 0.0) {
        let assign18120_e16325: f64 = (locals.var_leff).sqrt();
        let assign18120_e16326: f64 = (locals.var_pparam_b4soifprout * assign18120_e16325);
        let assign18120_e16328: f64 = (assign18120_e16326 / locals.var_vgst2vtm);
        (assign18120_e16328, (((((locals.var_pparam_b4soifprout_dn3 * assign18120_e16325) + (locals.var_pparam_b4soifprout * (locals.var_leff_dn3 / (2.0 * assign18120_e16325)))) * locals.var_vgst2vtm) - (assign18120_e16326 * locals.var_vgst2vtm_dn3)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)), (((((locals.var_pparam_b4soifprout_dn4 * assign18120_e16325) + (locals.var_pparam_b4soifprout * (locals.var_leff_dn4 / (2.0 * assign18120_e16325)))) * locals.var_vgst2vtm) - (assign18120_e16326 * locals.var_vgst2vtm_dn4)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)), (((((locals.var_pparam_b4soifprout_dn5 * assign18120_e16325) + (locals.var_pparam_b4soifprout * (locals.var_leff_dn5 / (2.0 * assign18120_e16325)))) * locals.var_vgst2vtm) - (assign18120_e16326 * locals.var_vgst2vtm_dn5)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)), (((((locals.var_pparam_b4soifprout_dn6 * assign18120_e16325) + (locals.var_pparam_b4soifprout * (locals.var_leff_dn6 / (2.0 * assign18120_e16325)))) * locals.var_vgst2vtm) - (assign18120_e16326 * locals.var_vgst2vtm_dn6)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)), (((((locals.var_pparam_b4soifprout_dn7 * assign18120_e16325) + (locals.var_pparam_b4soifprout * (locals.var_leff_dn7 / (2.0 * assign18120_e16325)))) * locals.var_vgst2vtm) - (assign18120_e16326 * locals.var_vgst2vtm_dn7)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)), (((((locals.var_pparam_b4soifprout_dn8 * assign18120_e16325) + (locals.var_pparam_b4soifprout * (locals.var_leff_dn8 / (2.0 * assign18120_e16325)))) * locals.var_vgst2vtm) - (assign18120_e16326 * locals.var_vgst2vtm_dn8)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)), (((((locals.var_pparam_b4soifprout_dn9 * assign18120_e16325) + (locals.var_pparam_b4soifprout * (locals.var_leff_dn9 / (2.0 * assign18120_e16325)))) * locals.var_vgst2vtm) - (assign18120_e16326 * locals.var_vgst2vtm_dn9)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)), (((((locals.var_pparam_b4soifprout_dn10 * assign18120_e16325) + (locals.var_pparam_b4soifprout * (locals.var_leff_dn10 / (2.0 * assign18120_e16325)))) * locals.var_vgst2vtm) - (assign18120_e16326 * locals.var_vgst2vtm_dn10)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)), (((((locals.var_pparam_b4soifprout_dn11 * assign18120_e16325) + (locals.var_pparam_b4soifprout * (locals.var_leff_dn11 / (2.0 * assign18120_e16325)))) * locals.var_vgst2vtm) - (assign18120_e16326 * locals.var_vgst2vtm_dn11)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)), (((((locals.var_pparam_b4soifprout_dn12 * assign18120_e16325) + (locals.var_pparam_b4soifprout * (locals.var_leff_dn12 / (2.0 * assign18120_e16325)))) * locals.var_vgst2vtm) - (assign18120_e16326 * locals.var_vgst2vtm_dn12)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign18120_e16330;
        locals.var_t9_dn3 = assign18120_e16330_d_n3;
        locals.var_t9_dn4 = assign18120_e16330_d_n4;
        locals.var_t9_dn5 = assign18120_e16330_d_n5;
        locals.var_t9_dn6 = assign18120_e16330_d_n6;
        locals.var_t9_dn7 = assign18120_e16330_d_n7;
        locals.var_t9_dn8 = assign18120_e16330_d_n8;
        locals.var_t9_dn9 = assign18120_e16330_d_n9;
        locals.var_t9_dn10 = assign18120_e16330_d_n10;
        locals.var_t9_dn11 = assign18120_e16330_d_n11;
        locals.var_t9_dn12 = assign18120_e16330_d_n12;
        locals.var_t9_rv = 0.0;

        let (assign18130_e16339, assign18130_e16339_d_n3, assign18130_e16339_d_n4, assign18130_e16339_d_n5, assign18130_e16339_d_n6, assign18130_e16339_d_n7, assign18130_e16339_d_n8, assign18130_e16339_d_n9, assign18130_e16339_d_n10, assign18130_e16339_d_n11, assign18130_e16339_d_n12,) = {
    if (locals.var_guard1211 == 0.0) {
        let assign18130_e16336: f64 = (1.0 + locals.var_t9);
        let assign18130_e16337: f64 = (1.0 / assign18130_e16336);
        (assign18130_e16337, (-(locals.var_t9_dn3 / (assign18130_e16336 * assign18130_e16336))), (-(locals.var_t9_dn4 / (assign18130_e16336 * assign18130_e16336))), (-(locals.var_t9_dn5 / (assign18130_e16336 * assign18130_e16336))), (-(locals.var_t9_dn6 / (assign18130_e16336 * assign18130_e16336))), (-(locals.var_t9_dn7 / (assign18130_e16336 * assign18130_e16336))), (-(locals.var_t9_dn8 / (assign18130_e16336 * assign18130_e16336))), (-(locals.var_t9_dn9 / (assign18130_e16336 * assign18130_e16336))), (-(locals.var_t9_dn10 / (assign18130_e16336 * assign18130_e16336))), (-(locals.var_t9_dn11 / (assign18130_e16336 * assign18130_e16336))), (-(locals.var_t9_dn12 / (assign18130_e16336 * assign18130_e16336))),)
    } else {
        (locals.var_fp, locals.var_fp_dn3, locals.var_fp_dn4, locals.var_fp_dn5, locals.var_fp_dn6, locals.var_fp_dn7, locals.var_fp_dn8, locals.var_fp_dn9, locals.var_fp_dn10, locals.var_fp_dn11, locals.var_fp_dn12,)
    }
};
        locals.var_fp = assign18130_e16339;
        locals.var_fp_dn3 = assign18130_e16339_d_n3;
        locals.var_fp_dn4 = assign18130_e16339_d_n4;
        locals.var_fp_dn5 = assign18130_e16339_d_n5;
        locals.var_fp_dn6 = assign18130_e16339_d_n6;
        locals.var_fp_dn7 = assign18130_e16339_d_n7;
        locals.var_fp_dn8 = assign18130_e16339_d_n8;
        locals.var_fp_dn9 = assign18130_e16339_d_n9;
        locals.var_fp_dn10 = assign18130_e16339_d_n10;
        locals.var_fp_dn11 = assign18130_e16339_d_n11;
        locals.var_fp_dn12 = assign18130_e16339_d_n12;
        locals.var_fp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_54(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign18140_e16342: f64 = (locals.var_sqrtphis - locals.var_sqrtphi);
        locals.var_t9 = assign18140_e16342;
        locals.var_t9_dn3 = (locals.var_sqrtphis_dn3 - locals.var_sqrtphi_dn3);
        locals.var_t9_dn4 = (locals.var_sqrtphis_dn4 - locals.var_sqrtphi_dn4);
        locals.var_t9_dn5 = (locals.var_sqrtphis_dn5 - locals.var_sqrtphi_dn5);
        locals.var_t9_dn6 = (locals.var_sqrtphis_dn6 - locals.var_sqrtphi_dn6);
        locals.var_t9_dn7 = (locals.var_sqrtphis_dn7 - locals.var_sqrtphi_dn7);
        locals.var_t9_dn8 = (locals.var_sqrtphis_dn8 - locals.var_sqrtphi_dn8);
        locals.var_t9_dn9 = (locals.var_sqrtphis_dn9 - locals.var_sqrtphi_dn9);
        locals.var_t9_dn10 = (locals.var_sqrtphis_dn10 - locals.var_sqrtphi_dn10);
        locals.var_t9_dn11 = (locals.var_sqrtphis_dn11 - locals.var_sqrtphi_dn11);
        locals.var_t9_dn12 = (locals.var_sqrtphis_dn12 - locals.var_sqrtphi_dn12);
        locals.var_t9_rv = 0.0;

        let assign18150_e16346: f64 = (2.0 - p.p22);
        let assign18150_e16349: f64 = (locals.var_pparam_b4soidwg * locals.var_vgsteff__blk840);
        let assign18150_e16352: f64 = (locals.var_pparam_b4soidwb * locals.var_t9);
        let assign18150_e16353: f64 = (assign18150_e16349 + assign18150_e16352);
        let assign18150_e16354: f64 = (assign18150_e16346 * assign18150_e16353);
        let assign18150_e16355: f64 = (locals.var_pparam_b4soiweff - assign18150_e16354);
        locals.var_weff = assign18150_e16355;
        locals.var_weff_dn3 = (locals.var_pparam_b4soiweff_dn3 - (assign18150_e16346 * (((locals.var_pparam_b4soidwg_dn3 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soidwg * locals.var_vgsteff__blk840_dn3)) + ((locals.var_pparam_b4soidwb_dn3 * locals.var_t9) + (locals.var_pparam_b4soidwb * locals.var_t9_dn3)))));
        locals.var_weff_dn4 = (locals.var_pparam_b4soiweff_dn4 - (assign18150_e16346 * (((locals.var_pparam_b4soidwg_dn4 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soidwg * locals.var_vgsteff__blk840_dn4)) + ((locals.var_pparam_b4soidwb_dn4 * locals.var_t9) + (locals.var_pparam_b4soidwb * locals.var_t9_dn4)))));
        locals.var_weff_dn5 = (locals.var_pparam_b4soiweff_dn5 - (assign18150_e16346 * (((locals.var_pparam_b4soidwg_dn5 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soidwg * locals.var_vgsteff__blk840_dn5)) + ((locals.var_pparam_b4soidwb_dn5 * locals.var_t9) + (locals.var_pparam_b4soidwb * locals.var_t9_dn5)))));
        locals.var_weff_dn6 = (locals.var_pparam_b4soiweff_dn6 - (assign18150_e16346 * (((locals.var_pparam_b4soidwg_dn6 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soidwg * locals.var_vgsteff__blk840_dn6)) + ((locals.var_pparam_b4soidwb_dn6 * locals.var_t9) + (locals.var_pparam_b4soidwb * locals.var_t9_dn6)))));
        locals.var_weff_dn7 = (locals.var_pparam_b4soiweff_dn7 - (assign18150_e16346 * (((locals.var_pparam_b4soidwg_dn7 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soidwg * locals.var_vgsteff__blk840_dn7)) + ((locals.var_pparam_b4soidwb_dn7 * locals.var_t9) + (locals.var_pparam_b4soidwb * locals.var_t9_dn7)))));
        locals.var_weff_dn8 = (locals.var_pparam_b4soiweff_dn8 - (assign18150_e16346 * (((locals.var_pparam_b4soidwg_dn8 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soidwg * locals.var_vgsteff__blk840_dn8)) + ((locals.var_pparam_b4soidwb_dn8 * locals.var_t9) + (locals.var_pparam_b4soidwb * locals.var_t9_dn8)))));
        locals.var_weff_dn9 = (locals.var_pparam_b4soiweff_dn9 - (assign18150_e16346 * (((locals.var_pparam_b4soidwg_dn9 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soidwg * locals.var_vgsteff__blk840_dn9)) + ((locals.var_pparam_b4soidwb_dn9 * locals.var_t9) + (locals.var_pparam_b4soidwb * locals.var_t9_dn9)))));
        locals.var_weff_dn10 = (locals.var_pparam_b4soiweff_dn10 - (assign18150_e16346 * (((locals.var_pparam_b4soidwg_dn10 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soidwg * locals.var_vgsteff__blk840_dn10)) + ((locals.var_pparam_b4soidwb_dn10 * locals.var_t9) + (locals.var_pparam_b4soidwb * locals.var_t9_dn10)))));
        locals.var_weff_dn11 = (locals.var_pparam_b4soiweff_dn11 - (assign18150_e16346 * (((locals.var_pparam_b4soidwg_dn11 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soidwg * locals.var_vgsteff__blk840_dn11)) + ((locals.var_pparam_b4soidwb_dn11 * locals.var_t9) + (locals.var_pparam_b4soidwb * locals.var_t9_dn11)))));
        locals.var_weff_dn12 = (locals.var_pparam_b4soiweff_dn12 - (assign18150_e16346 * (((locals.var_pparam_b4soidwg_dn12 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soidwg * locals.var_vgsteff__blk840_dn12)) + ((locals.var_pparam_b4soidwb_dn12 * locals.var_t9) + (locals.var_pparam_b4soidwb * locals.var_t9_dn12)))));
        locals.var_weff_rv = 0.0;

        let assign18160_e16358: f64 = if locals.var_weff < 2e-8 { 1.0 } else { 0.0 };
        locals.var_guard1212 = assign18160_e16358;
        locals.var_guard1212_rv = 0.0;

        let (assign18170_e16368, assign18170_e16368_d_n3, assign18170_e16368_d_n4, assign18170_e16368_d_n5, assign18170_e16368_d_n6, assign18170_e16368_d_n7, assign18170_e16368_d_n8, assign18170_e16368_d_n9, assign18170_e16368_d_n10, assign18170_e16368_d_n11, assign18170_e16368_d_n12,) = {
    if (locals.var_guard1212 != 0.0) {
        let assign18170_e16364: f64 = (2.0 * locals.var_weff);
        let assign18170_e16365: f64 = (6e-8 - assign18170_e16364);
        let assign18170_e16366: f64 = (1.0 / assign18170_e16365);
        (assign18170_e16366, (-((-(2.0 * locals.var_weff_dn3)) / (assign18170_e16365 * assign18170_e16365))), (-((-(2.0 * locals.var_weff_dn4)) / (assign18170_e16365 * assign18170_e16365))), (-((-(2.0 * locals.var_weff_dn5)) / (assign18170_e16365 * assign18170_e16365))), (-((-(2.0 * locals.var_weff_dn6)) / (assign18170_e16365 * assign18170_e16365))), (-((-(2.0 * locals.var_weff_dn7)) / (assign18170_e16365 * assign18170_e16365))), (-((-(2.0 * locals.var_weff_dn8)) / (assign18170_e16365 * assign18170_e16365))), (-((-(2.0 * locals.var_weff_dn9)) / (assign18170_e16365 * assign18170_e16365))), (-((-(2.0 * locals.var_weff_dn10)) / (assign18170_e16365 * assign18170_e16365))), (-((-(2.0 * locals.var_weff_dn11)) / (assign18170_e16365 * assign18170_e16365))), (-((-(2.0 * locals.var_weff_dn12)) / (assign18170_e16365 * assign18170_e16365))),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign18170_e16368;
        locals.var_t0__blk808_dn3 = assign18170_e16368_d_n3;
        locals.var_t0__blk808_dn4 = assign18170_e16368_d_n4;
        locals.var_t0__blk808_dn5 = assign18170_e16368_d_n5;
        locals.var_t0__blk808_dn6 = assign18170_e16368_d_n6;
        locals.var_t0__blk808_dn7 = assign18170_e16368_d_n7;
        locals.var_t0__blk808_dn8 = assign18170_e16368_d_n8;
        locals.var_t0__blk808_dn9 = assign18170_e16368_d_n9;
        locals.var_t0__blk808_dn10 = assign18170_e16368_d_n10;
        locals.var_t0__blk808_dn11 = assign18170_e16368_d_n11;
        locals.var_t0__blk808_dn12 = assign18170_e16368_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign18180_e16378, assign18180_e16378_d_n3, assign18180_e16378_d_n4, assign18180_e16378_d_n5, assign18180_e16378_d_n6, assign18180_e16378_d_n7, assign18180_e16378_d_n8, assign18180_e16378_d_n9, assign18180_e16378_d_n10, assign18180_e16378_d_n11, assign18180_e16378_d_n12,) = {
    if (locals.var_guard1212 != 0.0) {
        let assign18180_e16373: f64 = (4e-8 - locals.var_weff);
        let assign18180_e16374: f64 = (2e-8 * assign18180_e16373);
        let assign18180_e16376: f64 = (assign18180_e16374 * locals.var_t0__blk808);
        (assign18180_e16376, (((2e-8 * (-locals.var_weff_dn3)) * locals.var_t0__blk808) + (assign18180_e16374 * locals.var_t0__blk808_dn3)), (((2e-8 * (-locals.var_weff_dn4)) * locals.var_t0__blk808) + (assign18180_e16374 * locals.var_t0__blk808_dn4)), (((2e-8 * (-locals.var_weff_dn5)) * locals.var_t0__blk808) + (assign18180_e16374 * locals.var_t0__blk808_dn5)), (((2e-8 * (-locals.var_weff_dn6)) * locals.var_t0__blk808) + (assign18180_e16374 * locals.var_t0__blk808_dn6)), (((2e-8 * (-locals.var_weff_dn7)) * locals.var_t0__blk808) + (assign18180_e16374 * locals.var_t0__blk808_dn7)), (((2e-8 * (-locals.var_weff_dn8)) * locals.var_t0__blk808) + (assign18180_e16374 * locals.var_t0__blk808_dn8)), (((2e-8 * (-locals.var_weff_dn9)) * locals.var_t0__blk808) + (assign18180_e16374 * locals.var_t0__blk808_dn9)), (((2e-8 * (-locals.var_weff_dn10)) * locals.var_t0__blk808) + (assign18180_e16374 * locals.var_t0__blk808_dn10)), (((2e-8 * (-locals.var_weff_dn11)) * locals.var_t0__blk808) + (assign18180_e16374 * locals.var_t0__blk808_dn11)), (((2e-8 * (-locals.var_weff_dn12)) * locals.var_t0__blk808) + (assign18180_e16374 * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_weff, locals.var_weff_dn3, locals.var_weff_dn4, locals.var_weff_dn5, locals.var_weff_dn6, locals.var_weff_dn7, locals.var_weff_dn8, locals.var_weff_dn9, locals.var_weff_dn10, locals.var_weff_dn11, locals.var_weff_dn12,)
    }
};
        locals.var_weff = assign18180_e16378;
        locals.var_weff_dn3 = assign18180_e16378_d_n3;
        locals.var_weff_dn4 = assign18180_e16378_d_n4;
        locals.var_weff_dn5 = assign18180_e16378_d_n5;
        locals.var_weff_dn6 = assign18180_e16378_d_n6;
        locals.var_weff_dn7 = assign18180_e16378_d_n7;
        locals.var_weff_dn8 = assign18180_e16378_d_n8;
        locals.var_weff_dn9 = assign18180_e16378_d_n9;
        locals.var_weff_dn10 = assign18180_e16378_d_n10;
        locals.var_weff_dn11 = assign18180_e16378_d_n11;
        locals.var_weff_dn12 = assign18180_e16378_d_n12;
        locals.var_weff_rv = 0.0;

        let assign18190_e16381: f64 = if p.p429 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1213 = assign18190_e16381;
        locals.var_guard1213_rv = 0.0;

        let (assign18200_e16385, assign18200_e16385_d_n3, assign18200_e16385_d_n4, assign18200_e16385_d_n5, assign18200_e16385_d_n6, assign18200_e16385_d_n7, assign18200_e16385_d_n8, assign18200_e16385_d_n9, assign18200_e16385_d_n10, assign18200_e16385_d_n11, assign18200_e16385_d_n12,) = {
    if (locals.var_guard1213 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rds, locals.var_rds_dn3, locals.var_rds_dn4, locals.var_rds_dn5, locals.var_rds_dn6, locals.var_rds_dn7, locals.var_rds_dn8, locals.var_rds_dn9, locals.var_rds_dn10, locals.var_rds_dn11, locals.var_rds_dn12,)
    }
};
        locals.var_rds = assign18200_e16385;
        locals.var_rds_dn3 = assign18200_e16385_d_n3;
        locals.var_rds_dn4 = assign18200_e16385_d_n4;
        locals.var_rds_dn5 = assign18200_e16385_d_n5;
        locals.var_rds_dn6 = assign18200_e16385_d_n6;
        locals.var_rds_dn7 = assign18200_e16385_d_n7;
        locals.var_rds_dn8 = assign18200_e16385_d_n8;
        locals.var_rds_dn9 = assign18200_e16385_d_n9;
        locals.var_rds_dn10 = assign18200_e16385_d_n10;
        locals.var_rds_dn11 = assign18200_e16385_d_n11;
        locals.var_rds_dn12 = assign18200_e16385_d_n12;
        locals.var_rds_rv = 0.0;

        let (assign18210_e16396, assign18210_e16396_d_n3, assign18210_e16396_d_n4, assign18210_e16396_d_n5, assign18210_e16396_d_n6, assign18210_e16396_d_n7, assign18210_e16396_d_n8, assign18210_e16396_d_n9, assign18210_e16396_d_n10, assign18210_e16396_d_n11, assign18210_e16396_d_n12,) = {
    if (locals.var_guard1213 == 0.0) {
        let assign18210_e16390: f64 = (locals.var_pparam_b4soiprwg * locals.var_vgsteff__blk840);
        let assign18210_e16393: f64 = (locals.var_pparam_b4soiprwb * locals.var_t9);
        let assign18210_e16394: f64 = (assign18210_e16390 + assign18210_e16393);
        (assign18210_e16394, (((locals.var_pparam_b4soiprwg_dn3 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soiprwg * locals.var_vgsteff__blk840_dn3)) + ((locals.var_pparam_b4soiprwb_dn3 * locals.var_t9) + (locals.var_pparam_b4soiprwb * locals.var_t9_dn3))), (((locals.var_pparam_b4soiprwg_dn4 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soiprwg * locals.var_vgsteff__blk840_dn4)) + ((locals.var_pparam_b4soiprwb_dn4 * locals.var_t9) + (locals.var_pparam_b4soiprwb * locals.var_t9_dn4))), (((locals.var_pparam_b4soiprwg_dn5 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soiprwg * locals.var_vgsteff__blk840_dn5)) + ((locals.var_pparam_b4soiprwb_dn5 * locals.var_t9) + (locals.var_pparam_b4soiprwb * locals.var_t9_dn5))), (((locals.var_pparam_b4soiprwg_dn6 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soiprwg * locals.var_vgsteff__blk840_dn6)) + ((locals.var_pparam_b4soiprwb_dn6 * locals.var_t9) + (locals.var_pparam_b4soiprwb * locals.var_t9_dn6))), (((locals.var_pparam_b4soiprwg_dn7 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soiprwg * locals.var_vgsteff__blk840_dn7)) + ((locals.var_pparam_b4soiprwb_dn7 * locals.var_t9) + (locals.var_pparam_b4soiprwb * locals.var_t9_dn7))), (((locals.var_pparam_b4soiprwg_dn8 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soiprwg * locals.var_vgsteff__blk840_dn8)) + ((locals.var_pparam_b4soiprwb_dn8 * locals.var_t9) + (locals.var_pparam_b4soiprwb * locals.var_t9_dn8))), (((locals.var_pparam_b4soiprwg_dn9 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soiprwg * locals.var_vgsteff__blk840_dn9)) + ((locals.var_pparam_b4soiprwb_dn9 * locals.var_t9) + (locals.var_pparam_b4soiprwb * locals.var_t9_dn9))), (((locals.var_pparam_b4soiprwg_dn10 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soiprwg * locals.var_vgsteff__blk840_dn10)) + ((locals.var_pparam_b4soiprwb_dn10 * locals.var_t9) + (locals.var_pparam_b4soiprwb * locals.var_t9_dn10))), (((locals.var_pparam_b4soiprwg_dn11 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soiprwg * locals.var_vgsteff__blk840_dn11)) + ((locals.var_pparam_b4soiprwb_dn11 * locals.var_t9) + (locals.var_pparam_b4soiprwb * locals.var_t9_dn11))), (((locals.var_pparam_b4soiprwg_dn12 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soiprwg * locals.var_vgsteff__blk840_dn12)) + ((locals.var_pparam_b4soiprwb_dn12 * locals.var_t9) + (locals.var_pparam_b4soiprwb * locals.var_t9_dn12))),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign18210_e16396;
        locals.var_t0__blk808_dn3 = assign18210_e16396_d_n3;
        locals.var_t0__blk808_dn4 = assign18210_e16396_d_n4;
        locals.var_t0__blk808_dn5 = assign18210_e16396_d_n5;
        locals.var_t0__blk808_dn6 = assign18210_e16396_d_n6;
        locals.var_t0__blk808_dn7 = assign18210_e16396_d_n7;
        locals.var_t0__blk808_dn8 = assign18210_e16396_d_n8;
        locals.var_t0__blk808_dn9 = assign18210_e16396_d_n9;
        locals.var_t0__blk808_dn10 = assign18210_e16396_d_n10;
        locals.var_t0__blk808_dn11 = assign18210_e16396_d_n11;
        locals.var_t0__blk808_dn12 = assign18210_e16396_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let assign18220_e16399: f64 = (-0.9);
        let assign18220_e16400: f64 = if locals.var_t0__blk808 >= assign18220_e16399 { 1.0 } else { 0.0 };
        locals.var_guard1214 = assign18220_e16400;
        locals.var_guard1214_rv = 0.0;

        let (assign18230_e16411, assign18230_e16411_d_n3, assign18230_e16411_d_n4, assign18230_e16411_d_n5, assign18230_e16411_d_n6, assign18230_e16411_d_n7, assign18230_e16411_d_n8, assign18230_e16411_d_n9, assign18230_e16411_d_n10, assign18230_e16411_d_n11, assign18230_e16411_d_n12,) = {
    if ((locals.var_guard1213 == 0.0) && (locals.var_guard1214 != 0.0)) {
        let assign18230_e16408: f64 = (1.0 + locals.var_t0__blk808);
        let assign18230_e16409: f64 = (locals.var_rds0 * assign18230_e16408);
        (assign18230_e16409, ((locals.var_rds0_dn3 * assign18230_e16408) + (locals.var_rds0 * locals.var_t0__blk808_dn3)), ((locals.var_rds0_dn4 * assign18230_e16408) + (locals.var_rds0 * locals.var_t0__blk808_dn4)), ((locals.var_rds0_dn5 * assign18230_e16408) + (locals.var_rds0 * locals.var_t0__blk808_dn5)), ((locals.var_rds0_dn6 * assign18230_e16408) + (locals.var_rds0 * locals.var_t0__blk808_dn6)), ((locals.var_rds0_dn7 * assign18230_e16408) + (locals.var_rds0 * locals.var_t0__blk808_dn7)), ((locals.var_rds0_dn8 * assign18230_e16408) + (locals.var_rds0 * locals.var_t0__blk808_dn8)), ((locals.var_rds0_dn9 * assign18230_e16408) + (locals.var_rds0 * locals.var_t0__blk808_dn9)), ((locals.var_rds0_dn10 * assign18230_e16408) + (locals.var_rds0 * locals.var_t0__blk808_dn10)), ((locals.var_rds0_dn11 * assign18230_e16408) + (locals.var_rds0 * locals.var_t0__blk808_dn11)), ((locals.var_rds0_dn12 * assign18230_e16408) + (locals.var_rds0 * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_rds, locals.var_rds_dn3, locals.var_rds_dn4, locals.var_rds_dn5, locals.var_rds_dn6, locals.var_rds_dn7, locals.var_rds_dn8, locals.var_rds_dn9, locals.var_rds_dn10, locals.var_rds_dn11, locals.var_rds_dn12,)
    }
};
        locals.var_rds = assign18230_e16411;
        locals.var_rds_dn3 = assign18230_e16411_d_n3;
        locals.var_rds_dn4 = assign18230_e16411_d_n4;
        locals.var_rds_dn5 = assign18230_e16411_d_n5;
        locals.var_rds_dn6 = assign18230_e16411_d_n6;
        locals.var_rds_dn7 = assign18230_e16411_d_n7;
        locals.var_rds_dn8 = assign18230_e16411_d_n8;
        locals.var_rds_dn9 = assign18230_e16411_d_n9;
        locals.var_rds_dn10 = assign18230_e16411_d_n10;
        locals.var_rds_dn11 = assign18230_e16411_d_n11;
        locals.var_rds_dn12 = assign18230_e16411_d_n12;
        locals.var_rds_rv = 0.0;

        let (assign18240_e16425, assign18240_e16425_d_n3, assign18240_e16425_d_n4, assign18240_e16425_d_n5, assign18240_e16425_d_n6, assign18240_e16425_d_n7, assign18240_e16425_d_n8, assign18240_e16425_d_n9, assign18240_e16425_d_n10, assign18240_e16425_d_n11, assign18240_e16425_d_n12,) = {
    if ((locals.var_guard1213 == 0.0) && (locals.var_guard1214 == 0.0)) {
        let assign18240_e16421: f64 = (20.0 * locals.var_t0__blk808);
        let assign18240_e16422: f64 = (17.0 + assign18240_e16421);
        let assign18240_e16423: f64 = (1.0 / assign18240_e16422);
        (assign18240_e16423, (-((20.0 * locals.var_t0__blk808_dn3) / (assign18240_e16422 * assign18240_e16422))), (-((20.0 * locals.var_t0__blk808_dn4) / (assign18240_e16422 * assign18240_e16422))), (-((20.0 * locals.var_t0__blk808_dn5) / (assign18240_e16422 * assign18240_e16422))), (-((20.0 * locals.var_t0__blk808_dn6) / (assign18240_e16422 * assign18240_e16422))), (-((20.0 * locals.var_t0__blk808_dn7) / (assign18240_e16422 * assign18240_e16422))), (-((20.0 * locals.var_t0__blk808_dn8) / (assign18240_e16422 * assign18240_e16422))), (-((20.0 * locals.var_t0__blk808_dn9) / (assign18240_e16422 * assign18240_e16422))), (-((20.0 * locals.var_t0__blk808_dn10) / (assign18240_e16422 * assign18240_e16422))), (-((20.0 * locals.var_t0__blk808_dn11) / (assign18240_e16422 * assign18240_e16422))), (-((20.0 * locals.var_t0__blk808_dn12) / (assign18240_e16422 * assign18240_e16422))),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign18240_e16425;
        locals.var_t1__blk809_dn3 = assign18240_e16425_d_n3;
        locals.var_t1__blk809_dn4 = assign18240_e16425_d_n4;
        locals.var_t1__blk809_dn5 = assign18240_e16425_d_n5;
        locals.var_t1__blk809_dn6 = assign18240_e16425_d_n6;
        locals.var_t1__blk809_dn7 = assign18240_e16425_d_n7;
        locals.var_t1__blk809_dn8 = assign18240_e16425_d_n8;
        locals.var_t1__blk809_dn9 = assign18240_e16425_d_n9;
        locals.var_t1__blk809_dn10 = assign18240_e16425_d_n10;
        locals.var_t1__blk809_dn11 = assign18240_e16425_d_n11;
        locals.var_t1__blk809_dn12 = assign18240_e16425_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign18250_e16439, assign18250_e16439_d_n3, assign18250_e16439_d_n4, assign18250_e16439_d_n5, assign18250_e16439_d_n6, assign18250_e16439_d_n7, assign18250_e16439_d_n8, assign18250_e16439_d_n9, assign18250_e16439_d_n10, assign18250_e16439_d_n11, assign18250_e16439_d_n12,) = {
    if ((locals.var_guard1213 == 0.0) && (locals.var_guard1214 == 0.0)) {
        let assign18250_e16434: f64 = (0.8 + locals.var_t0__blk808);
        let assign18250_e16435: f64 = (locals.var_rds0 * assign18250_e16434);
        let assign18250_e16437: f64 = (assign18250_e16435 * locals.var_t1__blk809);
        (assign18250_e16437, ((((locals.var_rds0_dn3 * assign18250_e16434) + (locals.var_rds0 * locals.var_t0__blk808_dn3)) * locals.var_t1__blk809) + (assign18250_e16435 * locals.var_t1__blk809_dn3)), ((((locals.var_rds0_dn4 * assign18250_e16434) + (locals.var_rds0 * locals.var_t0__blk808_dn4)) * locals.var_t1__blk809) + (assign18250_e16435 * locals.var_t1__blk809_dn4)), ((((locals.var_rds0_dn5 * assign18250_e16434) + (locals.var_rds0 * locals.var_t0__blk808_dn5)) * locals.var_t1__blk809) + (assign18250_e16435 * locals.var_t1__blk809_dn5)), ((((locals.var_rds0_dn6 * assign18250_e16434) + (locals.var_rds0 * locals.var_t0__blk808_dn6)) * locals.var_t1__blk809) + (assign18250_e16435 * locals.var_t1__blk809_dn6)), ((((locals.var_rds0_dn7 * assign18250_e16434) + (locals.var_rds0 * locals.var_t0__blk808_dn7)) * locals.var_t1__blk809) + (assign18250_e16435 * locals.var_t1__blk809_dn7)), ((((locals.var_rds0_dn8 * assign18250_e16434) + (locals.var_rds0 * locals.var_t0__blk808_dn8)) * locals.var_t1__blk809) + (assign18250_e16435 * locals.var_t1__blk809_dn8)), ((((locals.var_rds0_dn9 * assign18250_e16434) + (locals.var_rds0 * locals.var_t0__blk808_dn9)) * locals.var_t1__blk809) + (assign18250_e16435 * locals.var_t1__blk809_dn9)), ((((locals.var_rds0_dn10 * assign18250_e16434) + (locals.var_rds0 * locals.var_t0__blk808_dn10)) * locals.var_t1__blk809) + (assign18250_e16435 * locals.var_t1__blk809_dn10)), ((((locals.var_rds0_dn11 * assign18250_e16434) + (locals.var_rds0 * locals.var_t0__blk808_dn11)) * locals.var_t1__blk809) + (assign18250_e16435 * locals.var_t1__blk809_dn11)), ((((locals.var_rds0_dn12 * assign18250_e16434) + (locals.var_rds0 * locals.var_t0__blk808_dn12)) * locals.var_t1__blk809) + (assign18250_e16435 * locals.var_t1__blk809_dn12)),)
    } else {
        (locals.var_rds, locals.var_rds_dn3, locals.var_rds_dn4, locals.var_rds_dn5, locals.var_rds_dn6, locals.var_rds_dn7, locals.var_rds_dn8, locals.var_rds_dn9, locals.var_rds_dn10, locals.var_rds_dn11, locals.var_rds_dn12,)
    }
};
        locals.var_rds = assign18250_e16439;
        locals.var_rds_dn3 = assign18250_e16439_d_n3;
        locals.var_rds_dn4 = assign18250_e16439_d_n4;
        locals.var_rds_dn5 = assign18250_e16439_d_n5;
        locals.var_rds_dn6 = assign18250_e16439_d_n6;
        locals.var_rds_dn7 = assign18250_e16439_d_n7;
        locals.var_rds_dn8 = assign18250_e16439_d_n8;
        locals.var_rds_dn9 = assign18250_e16439_d_n9;
        locals.var_rds_dn10 = assign18250_e16439_d_n10;
        locals.var_rds_dn11 = assign18250_e16439_d_n11;
        locals.var_rds_dn12 = assign18250_e16439_d_n12;
        locals.var_rds_rv = 0.0;

        let assign18260_e16443: f64 = (p.p137 * locals.var_trm1);
        let assign18260_e16444: f64 = (p.p135 + assign18260_e16443);
        locals.var_rsc_t = assign18260_e16444;
        locals.var_rsc_t_dn4 = (p.p137 * locals.var_trm1_dn4);
        locals.var_rsc_t_dn5 = (p.p137 * locals.var_trm1_dn5);
        locals.var_rsc_t_dn6 = (p.p137 * locals.var_trm1_dn6);
        locals.var_rsc_t_rv = 0.0;

        let assign18270_e16448: f64 = (p.p138 * locals.var_trm1);
        let assign18270_e16449: f64 = (p.p136 + assign18270_e16448);
        locals.var_rdc_t = assign18270_e16449;
        locals.var_rdc_t_dn4 = (p.p138 * locals.var_trm1_dn4);
        locals.var_rdc_t_dn5 = (p.p138 * locals.var_trm1_dn5);
        locals.var_rdc_t_dn6 = (p.p138 * locals.var_trm1_dn6);
        locals.var_rdc_t_rv = 0.0;

        let assign18280_e16452: f64 = if p.p429 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1215 = assign18280_e16452;
        locals.var_guard1215_rv = 0.0;

        let (assign18290_e16464, assign18290_e16464_d_n3, assign18290_e16464_d_n4, assign18290_e16464_d_n5, assign18290_e16464_d_n6, assign18290_e16464_d_n7, assign18290_e16464_d_n8, assign18290_e16464_d_n9, assign18290_e16464_d_n10, assign18290_e16464_d_n11, assign18290_e16464_d_n12,) = {
    if (locals.var_guard1215 != 0.0) {
        let assign18290_e16456: f64 = (locals.var_b4soidrainresistance + locals.var_rds);
        let assign18290_e16458: f64 = (assign18290_e16456 + locals.var_b4soisourceresistance);
        let assign18290_e16460: f64 = (assign18290_e16458 + locals.var_rdc_t);
        let assign18290_e16462: f64 = (assign18290_e16460 + locals.var_rsc_t);
        (assign18290_e16462, locals.var_rds_dn3, ((locals.var_rds_dn4 + locals.var_rdc_t_dn4) + locals.var_rsc_t_dn4), ((locals.var_rds_dn5 + locals.var_rdc_t_dn5) + locals.var_rsc_t_dn5), ((locals.var_rds_dn6 + locals.var_rdc_t_dn6) + locals.var_rsc_t_dn6), locals.var_rds_dn7, locals.var_rds_dn8, locals.var_rds_dn9, locals.var_rds_dn10, locals.var_rds_dn11, locals.var_rds_dn12,)
    } else {
        (locals.var_rds, locals.var_rds_dn3, locals.var_rds_dn4, locals.var_rds_dn5, locals.var_rds_dn6, locals.var_rds_dn7, locals.var_rds_dn8, locals.var_rds_dn9, locals.var_rds_dn10, locals.var_rds_dn11, locals.var_rds_dn12,)
    }
};
        locals.var_rds = assign18290_e16464;
        locals.var_rds_dn3 = assign18290_e16464_d_n3;
        locals.var_rds_dn4 = assign18290_e16464_d_n4;
        locals.var_rds_dn5 = assign18290_e16464_d_n5;
        locals.var_rds_dn6 = assign18290_e16464_d_n6;
        locals.var_rds_dn7 = assign18290_e16464_d_n7;
        locals.var_rds_dn8 = assign18290_e16464_d_n8;
        locals.var_rds_dn9 = assign18290_e16464_d_n9;
        locals.var_rds_dn10 = assign18290_e16464_d_n10;
        locals.var_rds_dn11 = assign18290_e16464_d_n11;
        locals.var_rds_dn12 = assign18290_e16464_d_n12;
        locals.var_rds_rv = 0.0;

        let assign18310_e16470: f64 = if locals.var_pparam_b4soia0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1216 = assign18310_e16470;
        locals.var_guard1216_rv = 0.0;

        let (assign18320_e16474, assign18320_e16474_d_n3, assign18320_e16474_d_n4, assign18320_e16474_d_n5, assign18320_e16474_d_n6, assign18320_e16474_d_n7, assign18320_e16474_d_n8, assign18320_e16474_d_n9, assign18320_e16474_d_n10, assign18320_e16474_d_n11, assign18320_e16474_d_n12,) = {
    if (locals.var_guard1216 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_abulk, locals.var_abulk_dn3, locals.var_abulk_dn4, locals.var_abulk_dn5, locals.var_abulk_dn6, locals.var_abulk_dn7, locals.var_abulk_dn8, locals.var_abulk_dn9, locals.var_abulk_dn10, locals.var_abulk_dn11, locals.var_abulk_dn12,)
    }
};
        locals.var_abulk = assign18320_e16474;
        locals.var_abulk_dn3 = assign18320_e16474_d_n3;
        locals.var_abulk_dn4 = assign18320_e16474_d_n4;
        locals.var_abulk_dn5 = assign18320_e16474_d_n5;
        locals.var_abulk_dn6 = assign18320_e16474_d_n6;
        locals.var_abulk_dn7 = assign18320_e16474_d_n7;
        locals.var_abulk_dn8 = assign18320_e16474_d_n8;
        locals.var_abulk_dn9 = assign18320_e16474_d_n9;
        locals.var_abulk_dn10 = assign18320_e16474_d_n10;
        locals.var_abulk_dn11 = assign18320_e16474_d_n11;
        locals.var_abulk_dn12 = assign18320_e16474_d_n12;
        locals.var_abulk_rv = 0.0;

        let (assign18330_e16478, assign18330_e16478_d_n3, assign18330_e16478_d_n4, assign18330_e16478_d_n5, assign18330_e16478_d_n6, assign18330_e16478_d_n7, assign18330_e16478_d_n8, assign18330_e16478_d_n9, assign18330_e16478_d_n10, assign18330_e16478_d_n11, assign18330_e16478_d_n12,) = {
    if (locals.var_guard1216 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_abulk0, locals.var_abulk0_dn3, locals.var_abulk0_dn4, locals.var_abulk0_dn5, locals.var_abulk0_dn6, locals.var_abulk0_dn7, locals.var_abulk0_dn8, locals.var_abulk0_dn9, locals.var_abulk0_dn10, locals.var_abulk0_dn11, locals.var_abulk0_dn12,)
    }
};
        locals.var_abulk0 = assign18330_e16478;
        locals.var_abulk0_dn3 = assign18330_e16478_d_n3;
        locals.var_abulk0_dn4 = assign18330_e16478_d_n4;
        locals.var_abulk0_dn5 = assign18330_e16478_d_n5;
        locals.var_abulk0_dn6 = assign18330_e16478_d_n6;
        locals.var_abulk0_dn7 = assign18330_e16478_d_n7;
        locals.var_abulk0_dn8 = assign18330_e16478_d_n8;
        locals.var_abulk0_dn9 = assign18330_e16478_d_n9;
        locals.var_abulk0_dn10 = assign18330_e16478_d_n10;
        locals.var_abulk0_dn11 = assign18330_e16478_d_n11;
        locals.var_abulk0_dn12 = assign18330_e16478_d_n12;
        locals.var_abulk0_rv = 0.0;

        let (assign18340_e16485, assign18340_e16485_d_n3, assign18340_e16485_d_n4, assign18340_e16485_d_n5, assign18340_e16485_d_n6, assign18340_e16485_d_n7, assign18340_e16485_d_n8, assign18340_e16485_d_n9, assign18340_e16485_d_n10, assign18340_e16485_d_n11, assign18340_e16485_d_n12,) = {
    if (locals.var_guard1216 == 0.0) {
        let assign18340_e16483: f64 = (locals.var_pparam_b4soiketa * locals.var_vbsh);
        (assign18340_e16483, ((locals.var_pparam_b4soiketa_dn3 * locals.var_vbsh) + (locals.var_pparam_b4soiketa * locals.var_vbsh_dn3)), ((locals.var_pparam_b4soiketa_dn4 * locals.var_vbsh) + (locals.var_pparam_b4soiketa * locals.var_vbsh_dn4)), ((locals.var_pparam_b4soiketa_dn5 * locals.var_vbsh) + (locals.var_pparam_b4soiketa * locals.var_vbsh_dn5)), ((locals.var_pparam_b4soiketa_dn6 * locals.var_vbsh) + (locals.var_pparam_b4soiketa * locals.var_vbsh_dn6)), ((locals.var_pparam_b4soiketa_dn7 * locals.var_vbsh) + (locals.var_pparam_b4soiketa * locals.var_vbsh_dn7)), ((locals.var_pparam_b4soiketa_dn8 * locals.var_vbsh) + (locals.var_pparam_b4soiketa * locals.var_vbsh_dn8)), ((locals.var_pparam_b4soiketa_dn9 * locals.var_vbsh) + (locals.var_pparam_b4soiketa * locals.var_vbsh_dn9)), ((locals.var_pparam_b4soiketa_dn10 * locals.var_vbsh) + (locals.var_pparam_b4soiketa * locals.var_vbsh_dn10)), ((locals.var_pparam_b4soiketa_dn11 * locals.var_vbsh) + (locals.var_pparam_b4soiketa * locals.var_vbsh_dn11)), ((locals.var_pparam_b4soiketa_dn12 * locals.var_vbsh) + (locals.var_pparam_b4soiketa * locals.var_vbsh_dn12)),)
    } else {
        (locals.var_t10__blk818, locals.var_t10__blk818_dn3, locals.var_t10__blk818_dn4, locals.var_t10__blk818_dn5, locals.var_t10__blk818_dn6, locals.var_t10__blk818_dn7, locals.var_t10__blk818_dn8, locals.var_t10__blk818_dn9, locals.var_t10__blk818_dn10, locals.var_t10__blk818_dn11, locals.var_t10__blk818_dn12,)
    }
};
        locals.var_t10__blk818 = assign18340_e16485;
        locals.var_t10__blk818_dn3 = assign18340_e16485_d_n3;
        locals.var_t10__blk818_dn4 = assign18340_e16485_d_n4;
        locals.var_t10__blk818_dn5 = assign18340_e16485_d_n5;
        locals.var_t10__blk818_dn6 = assign18340_e16485_d_n6;
        locals.var_t10__blk818_dn7 = assign18340_e16485_d_n7;
        locals.var_t10__blk818_dn8 = assign18340_e16485_d_n8;
        locals.var_t10__blk818_dn9 = assign18340_e16485_d_n9;
        locals.var_t10__blk818_dn10 = assign18340_e16485_d_n10;
        locals.var_t10__blk818_dn11 = assign18340_e16485_d_n11;
        locals.var_t10__blk818_dn12 = assign18340_e16485_d_n12;
        locals.var_t10__blk818_rv = 0.0;

        let assign18350_e16488: f64 = (-0.5);
        let assign18350_e16489: f64 = if locals.var_t10__blk818 >= assign18350_e16488 { 1.0 } else { 0.0 };
        locals.var_guard1217 = assign18350_e16489;
        locals.var_guard1217_rv = 0.0;

        let (assign18360_e16500, assign18360_e16500_d_n3, assign18360_e16500_d_n4, assign18360_e16500_d_n5, assign18360_e16500_d_n6, assign18360_e16500_d_n7, assign18360_e16500_d_n8, assign18360_e16500_d_n9, assign18360_e16500_d_n10, assign18360_e16500_d_n11, assign18360_e16500_d_n12,) = {
    if ((locals.var_guard1216 == 0.0) && (locals.var_guard1217 != 0.0)) {
        let assign18360_e16497: f64 = (1.0 + locals.var_t10__blk818);
        let assign18360_e16498: f64 = (1.0 / assign18360_e16497);
        (assign18360_e16498, (-(locals.var_t10__blk818_dn3 / (assign18360_e16497 * assign18360_e16497))), (-(locals.var_t10__blk818_dn4 / (assign18360_e16497 * assign18360_e16497))), (-(locals.var_t10__blk818_dn5 / (assign18360_e16497 * assign18360_e16497))), (-(locals.var_t10__blk818_dn6 / (assign18360_e16497 * assign18360_e16497))), (-(locals.var_t10__blk818_dn7 / (assign18360_e16497 * assign18360_e16497))), (-(locals.var_t10__blk818_dn8 / (assign18360_e16497 * assign18360_e16497))), (-(locals.var_t10__blk818_dn9 / (assign18360_e16497 * assign18360_e16497))), (-(locals.var_t10__blk818_dn10 / (assign18360_e16497 * assign18360_e16497))), (-(locals.var_t10__blk818_dn11 / (assign18360_e16497 * assign18360_e16497))), (-(locals.var_t10__blk818_dn12 / (assign18360_e16497 * assign18360_e16497))),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12,)
    }
};
        locals.var_t11 = assign18360_e16500;
        locals.var_t11_dn3 = assign18360_e16500_d_n3;
        locals.var_t11_dn4 = assign18360_e16500_d_n4;
        locals.var_t11_dn5 = assign18360_e16500_d_n5;
        locals.var_t11_dn6 = assign18360_e16500_d_n6;
        locals.var_t11_dn7 = assign18360_e16500_d_n7;
        locals.var_t11_dn8 = assign18360_e16500_d_n8;
        locals.var_t11_dn9 = assign18360_e16500_d_n9;
        locals.var_t11_dn10 = assign18360_e16500_d_n10;
        locals.var_t11_dn11 = assign18360_e16500_d_n11;
        locals.var_t11_dn12 = assign18360_e16500_d_n12;
        locals.var_t11_rv = 0.0;

        let (assign18370_e16517, assign18370_e16517_d_n3, assign18370_e16517_d_n4, assign18370_e16517_d_n5, assign18370_e16517_d_n6, assign18370_e16517_d_n7, assign18370_e16517_d_n8, assign18370_e16517_d_n9, assign18370_e16517_d_n10, assign18370_e16517_d_n11, assign18370_e16517_d_n12,) = {
    if ((locals.var_guard1216 == 0.0) && (locals.var_guard1217 == 0.0)) {
        let assign18370_e16507: f64 = (-1.0);
        let assign18370_e16510: f64 = (1.0 - 0.5);
        let assign18370_e16513: f64 = (1.0 - 0.5);
        let assign18370_e16514: f64 = (assign18370_e16510 * assign18370_e16513);
        let assign18370_e16515: f64 = (assign18370_e16507 / assign18370_e16514);
        (assign18370_e16515, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12,)
    }
};
        locals.var_t12 = assign18370_e16517;
        locals.var_t12_dn3 = assign18370_e16517_d_n3;
        locals.var_t12_dn4 = assign18370_e16517_d_n4;
        locals.var_t12_dn5 = assign18370_e16517_d_n5;
        locals.var_t12_dn6 = assign18370_e16517_d_n6;
        locals.var_t12_dn7 = assign18370_e16517_d_n7;
        locals.var_t12_dn8 = assign18370_e16517_d_n8;
        locals.var_t12_dn9 = assign18370_e16517_d_n9;
        locals.var_t12_dn10 = assign18370_e16517_d_n10;
        locals.var_t12_dn11 = assign18370_e16517_d_n11;
        locals.var_t12_dn12 = assign18370_e16517_d_n12;
        locals.var_t12_rv = 0.0;

        let (assign18380_e16533, assign18380_e16533_d_n3, assign18380_e16533_d_n4, assign18380_e16533_d_n5, assign18380_e16533_d_n6, assign18380_e16533_d_n7, assign18380_e16533_d_n8, assign18380_e16533_d_n9, assign18380_e16533_d_n10, assign18380_e16533_d_n11, assign18380_e16533_d_n12,) = {
    if ((locals.var_guard1216 == 0.0) && (locals.var_guard1217 == 0.0)) {
        let assign18380_e16526: f64 = (1.0 - 0.5);
        let assign18380_e16527: f64 = (1.0 / assign18380_e16526);
        let assign18380_e16530: f64 = (locals.var_t12 * 0.5);
        let assign18380_e16531: f64 = (assign18380_e16527 + assign18380_e16530);
        (assign18380_e16531, (locals.var_t12_dn3 * 0.5), (locals.var_t12_dn4 * 0.5), (locals.var_t12_dn5 * 0.5), (locals.var_t12_dn6 * 0.5), (locals.var_t12_dn7 * 0.5), (locals.var_t12_dn8 * 0.5), (locals.var_t12_dn9 * 0.5), (locals.var_t12_dn10 * 0.5), (locals.var_t12_dn11 * 0.5), (locals.var_t12_dn12 * 0.5),)
    } else {
        (locals.var_t13, locals.var_t13_dn3, locals.var_t13_dn4, locals.var_t13_dn5, locals.var_t13_dn6, locals.var_t13_dn7, locals.var_t13_dn8, locals.var_t13_dn9, locals.var_t13_dn10, locals.var_t13_dn11, locals.var_t13_dn12,)
    }
};
        locals.var_t13 = assign18380_e16533;
        locals.var_t13_dn3 = assign18380_e16533_d_n3;
        locals.var_t13_dn4 = assign18380_e16533_d_n4;
        locals.var_t13_dn5 = assign18380_e16533_d_n5;
        locals.var_t13_dn6 = assign18380_e16533_d_n6;
        locals.var_t13_dn7 = assign18380_e16533_d_n7;
        locals.var_t13_dn8 = assign18380_e16533_d_n8;
        locals.var_t13_dn9 = assign18380_e16533_d_n9;
        locals.var_t13_dn10 = assign18380_e16533_d_n10;
        locals.var_t13_dn11 = assign18380_e16533_d_n11;
        locals.var_t13_dn12 = assign18380_e16533_d_n12;
        locals.var_t13_rv = 0.0;

        let (assign18390_e16545, assign18390_e16545_d_n3, assign18390_e16545_d_n4, assign18390_e16545_d_n5, assign18390_e16545_d_n6, assign18390_e16545_d_n7, assign18390_e16545_d_n8, assign18390_e16545_d_n9, assign18390_e16545_d_n10, assign18390_e16545_d_n11, assign18390_e16545_d_n12,) = {
    if ((locals.var_guard1216 == 0.0) && (locals.var_guard1217 == 0.0)) {
        let assign18390_e16541: f64 = (locals.var_t12 * locals.var_t10__blk818);
        let assign18390_e16543: f64 = (assign18390_e16541 + locals.var_t13);
        (assign18390_e16543, (((locals.var_t12_dn3 * locals.var_t10__blk818) + (locals.var_t12 * locals.var_t10__blk818_dn3)) + locals.var_t13_dn3), (((locals.var_t12_dn4 * locals.var_t10__blk818) + (locals.var_t12 * locals.var_t10__blk818_dn4)) + locals.var_t13_dn4), (((locals.var_t12_dn5 * locals.var_t10__blk818) + (locals.var_t12 * locals.var_t10__blk818_dn5)) + locals.var_t13_dn5), (((locals.var_t12_dn6 * locals.var_t10__blk818) + (locals.var_t12 * locals.var_t10__blk818_dn6)) + locals.var_t13_dn6), (((locals.var_t12_dn7 * locals.var_t10__blk818) + (locals.var_t12 * locals.var_t10__blk818_dn7)) + locals.var_t13_dn7), (((locals.var_t12_dn8 * locals.var_t10__blk818) + (locals.var_t12 * locals.var_t10__blk818_dn8)) + locals.var_t13_dn8), (((locals.var_t12_dn9 * locals.var_t10__blk818) + (locals.var_t12 * locals.var_t10__blk818_dn9)) + locals.var_t13_dn9), (((locals.var_t12_dn10 * locals.var_t10__blk818) + (locals.var_t12 * locals.var_t10__blk818_dn10)) + locals.var_t13_dn10), (((locals.var_t12_dn11 * locals.var_t10__blk818) + (locals.var_t12 * locals.var_t10__blk818_dn11)) + locals.var_t13_dn11), (((locals.var_t12_dn12 * locals.var_t10__blk818) + (locals.var_t12 * locals.var_t10__blk818_dn12)) + locals.var_t13_dn12),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12,)
    }
};
        locals.var_t11 = assign18390_e16545;
        locals.var_t11_dn3 = assign18390_e16545_d_n3;
        locals.var_t11_dn4 = assign18390_e16545_d_n4;
        locals.var_t11_dn5 = assign18390_e16545_d_n5;
        locals.var_t11_dn6 = assign18390_e16545_d_n6;
        locals.var_t11_dn7 = assign18390_e16545_d_n7;
        locals.var_t11_dn8 = assign18390_e16545_d_n8;
        locals.var_t11_dn9 = assign18390_e16545_d_n9;
        locals.var_t11_dn10 = assign18390_e16545_d_n10;
        locals.var_t11_dn11 = assign18390_e16545_d_n11;
        locals.var_t11_dn12 = assign18390_e16545_d_n12;
        locals.var_t11_rv = 0.0;

        let (assign18400_e16552, assign18400_e16552_d_n3, assign18400_e16552_d_n4, assign18400_e16552_d_n5, assign18400_e16552_d_n6, assign18400_e16552_d_n7, assign18400_e16552_d_n8, assign18400_e16552_d_n9, assign18400_e16552_d_n10, assign18400_e16552_d_n11, assign18400_e16552_d_n12,) = {
    if (locals.var_guard1216 == 0.0) {
        let assign18400_e16550: f64 = (locals.var_phi + locals.var_pparam_b4soiketas);
        (assign18400_e16550, (locals.var_phi_dn3 + locals.var_pparam_b4soiketas_dn3), (locals.var_phi_dn4 + locals.var_pparam_b4soiketas_dn4), (locals.var_phi_dn5 + locals.var_pparam_b4soiketas_dn5), (locals.var_phi_dn6 + locals.var_pparam_b4soiketas_dn6), (locals.var_phi_dn7 + locals.var_pparam_b4soiketas_dn7), (locals.var_phi_dn8 + locals.var_pparam_b4soiketas_dn8), (locals.var_phi_dn9 + locals.var_pparam_b4soiketas_dn9), (locals.var_phi_dn10 + locals.var_pparam_b4soiketas_dn10), (locals.var_phi_dn11 + locals.var_pparam_b4soiketas_dn11), (locals.var_phi_dn12 + locals.var_pparam_b4soiketas_dn12),)
    } else {
        (locals.var_t10__blk818, locals.var_t10__blk818_dn3, locals.var_t10__blk818_dn4, locals.var_t10__blk818_dn5, locals.var_t10__blk818_dn6, locals.var_t10__blk818_dn7, locals.var_t10__blk818_dn8, locals.var_t10__blk818_dn9, locals.var_t10__blk818_dn10, locals.var_t10__blk818_dn11, locals.var_t10__blk818_dn12,)
    }
};
        locals.var_t10__blk818 = assign18400_e16552;
        locals.var_t10__blk818_dn3 = assign18400_e16552_d_n3;
        locals.var_t10__blk818_dn4 = assign18400_e16552_d_n4;
        locals.var_t10__blk818_dn5 = assign18400_e16552_d_n5;
        locals.var_t10__blk818_dn6 = assign18400_e16552_d_n6;
        locals.var_t10__blk818_dn7 = assign18400_e16552_d_n7;
        locals.var_t10__blk818_dn8 = assign18400_e16552_d_n8;
        locals.var_t10__blk818_dn9 = assign18400_e16552_d_n9;
        locals.var_t10__blk818_dn10 = assign18400_e16552_d_n10;
        locals.var_t10__blk818_dn11 = assign18400_e16552_d_n11;
        locals.var_t10__blk818_dn12 = assign18400_e16552_d_n12;
        locals.var_t10__blk818_rv = 0.0;

        let (assign18410_e16561, assign18410_e16561_d_n3, assign18410_e16561_d_n4, assign18410_e16561_d_n5, assign18410_e16561_d_n6, assign18410_e16561_d_n7, assign18410_e16561_d_n8, assign18410_e16561_d_n9, assign18410_e16561_d_n10, assign18410_e16561_d_n11, assign18410_e16561_d_n12,) = {
    if (locals.var_guard1216 == 0.0) {
        let assign18410_e16557: f64 = (locals.var_vbsh * locals.var_t11);
        let assign18410_e16559: f64 = (assign18410_e16557 / locals.var_t10__blk818);
        (assign18410_e16559, (((((locals.var_vbsh_dn3 * locals.var_t11) + (locals.var_vbsh * locals.var_t11_dn3)) * locals.var_t10__blk818) - (assign18410_e16557 * locals.var_t10__blk818_dn3)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((((locals.var_vbsh_dn4 * locals.var_t11) + (locals.var_vbsh * locals.var_t11_dn4)) * locals.var_t10__blk818) - (assign18410_e16557 * locals.var_t10__blk818_dn4)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((((locals.var_vbsh_dn5 * locals.var_t11) + (locals.var_vbsh * locals.var_t11_dn5)) * locals.var_t10__blk818) - (assign18410_e16557 * locals.var_t10__blk818_dn5)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((((locals.var_vbsh_dn6 * locals.var_t11) + (locals.var_vbsh * locals.var_t11_dn6)) * locals.var_t10__blk818) - (assign18410_e16557 * locals.var_t10__blk818_dn6)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((((locals.var_vbsh_dn7 * locals.var_t11) + (locals.var_vbsh * locals.var_t11_dn7)) * locals.var_t10__blk818) - (assign18410_e16557 * locals.var_t10__blk818_dn7)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((((locals.var_vbsh_dn8 * locals.var_t11) + (locals.var_vbsh * locals.var_t11_dn8)) * locals.var_t10__blk818) - (assign18410_e16557 * locals.var_t10__blk818_dn8)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((((locals.var_vbsh_dn9 * locals.var_t11) + (locals.var_vbsh * locals.var_t11_dn9)) * locals.var_t10__blk818) - (assign18410_e16557 * locals.var_t10__blk818_dn9)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((((locals.var_vbsh_dn10 * locals.var_t11) + (locals.var_vbsh * locals.var_t11_dn10)) * locals.var_t10__blk818) - (assign18410_e16557 * locals.var_t10__blk818_dn10)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((((locals.var_vbsh_dn11 * locals.var_t11) + (locals.var_vbsh * locals.var_t11_dn11)) * locals.var_t10__blk818) - (assign18410_e16557 * locals.var_t10__blk818_dn11)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((((locals.var_vbsh_dn12 * locals.var_t11) + (locals.var_vbsh * locals.var_t11_dn12)) * locals.var_t10__blk818) - (assign18410_e16557 * locals.var_t10__blk818_dn12)) / (locals.var_t10__blk818 * locals.var_t10__blk818)),)
    } else {
        (locals.var_t13, locals.var_t13_dn3, locals.var_t13_dn4, locals.var_t13_dn5, locals.var_t13_dn6, locals.var_t13_dn7, locals.var_t13_dn8, locals.var_t13_dn9, locals.var_t13_dn10, locals.var_t13_dn11, locals.var_t13_dn12,)
    }
};
        locals.var_t13 = assign18410_e16561;
        locals.var_t13_dn3 = assign18410_e16561_d_n3;
        locals.var_t13_dn4 = assign18410_e16561_d_n4;
        locals.var_t13_dn5 = assign18410_e16561_d_n5;
        locals.var_t13_dn6 = assign18410_e16561_d_n6;
        locals.var_t13_dn7 = assign18410_e16561_d_n7;
        locals.var_t13_dn8 = assign18410_e16561_d_n8;
        locals.var_t13_dn9 = assign18410_e16561_d_n9;
        locals.var_t13_dn10 = assign18410_e16561_d_n10;
        locals.var_t13_dn11 = assign18410_e16561_d_n11;
        locals.var_t13_dn12 = assign18410_e16561_d_n12;
        locals.var_t13_rv = 0.0;

        let assign18420_e16564: f64 = if locals.var_t13 < 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1218 = assign18420_e16564;
        locals.var_guard1218_rv = 0.0;

        let (assign18430_e16576, assign18430_e16576_d_n3, assign18430_e16576_d_n4, assign18430_e16576_d_n5, assign18430_e16576_d_n6, assign18430_e16576_d_n7, assign18430_e16576_d_n8, assign18430_e16576_d_n9, assign18430_e16576_d_n10, assign18430_e16576_d_n11, assign18430_e16576_d_n12,) = {
    if ((locals.var_guard1216 == 0.0) && (locals.var_guard1218 != 0.0)) {
        let assign18430_e16572: f64 = (1.0 - locals.var_t13);
        let assign18430_e16573: f64 = (assign18430_e16572).sqrt();
        let assign18430_e16574: f64 = (1.0 / assign18430_e16573);
        (assign18430_e16574, (-(((-locals.var_t13_dn3) / (2.0 * assign18430_e16573)) / (assign18430_e16573 * assign18430_e16573))), (-(((-locals.var_t13_dn4) / (2.0 * assign18430_e16573)) / (assign18430_e16573 * assign18430_e16573))), (-(((-locals.var_t13_dn5) / (2.0 * assign18430_e16573)) / (assign18430_e16573 * assign18430_e16573))), (-(((-locals.var_t13_dn6) / (2.0 * assign18430_e16573)) / (assign18430_e16573 * assign18430_e16573))), (-(((-locals.var_t13_dn7) / (2.0 * assign18430_e16573)) / (assign18430_e16573 * assign18430_e16573))), (-(((-locals.var_t13_dn8) / (2.0 * assign18430_e16573)) / (assign18430_e16573 * assign18430_e16573))), (-(((-locals.var_t13_dn9) / (2.0 * assign18430_e16573)) / (assign18430_e16573 * assign18430_e16573))), (-(((-locals.var_t13_dn10) / (2.0 * assign18430_e16573)) / (assign18430_e16573 * assign18430_e16573))), (-(((-locals.var_t13_dn11) / (2.0 * assign18430_e16573)) / (assign18430_e16573 * assign18430_e16573))), (-(((-locals.var_t13_dn12) / (2.0 * assign18430_e16573)) / (assign18430_e16573 * assign18430_e16573))),)
    } else {
        (locals.var_t14, locals.var_t14_dn3, locals.var_t14_dn4, locals.var_t14_dn5, locals.var_t14_dn6, locals.var_t14_dn7, locals.var_t14_dn8, locals.var_t14_dn9, locals.var_t14_dn10, locals.var_t14_dn11, locals.var_t14_dn12,)
    }
};
        locals.var_t14 = assign18430_e16576;
        locals.var_t14_dn3 = assign18430_e16576_d_n3;
        locals.var_t14_dn4 = assign18430_e16576_d_n4;
        locals.var_t14_dn5 = assign18430_e16576_d_n5;
        locals.var_t14_dn6 = assign18430_e16576_d_n6;
        locals.var_t14_dn7 = assign18430_e16576_d_n7;
        locals.var_t14_dn8 = assign18430_e16576_d_n8;
        locals.var_t14_dn9 = assign18430_e16576_d_n9;
        locals.var_t14_dn10 = assign18430_e16576_d_n10;
        locals.var_t14_dn11 = assign18430_e16576_d_n11;
        locals.var_t14_dn12 = assign18430_e16576_d_n12;
        locals.var_t14_rv = 0.0;

        let (assign18440_e16595, assign18440_e16595_d_n3, assign18440_e16595_d_n4, assign18440_e16595_d_n5, assign18440_e16595_d_n6, assign18440_e16595_d_n7, assign18440_e16595_d_n8, assign18440_e16595_d_n9, assign18440_e16595_d_n10, assign18440_e16595_d_n11, assign18440_e16595_d_n12,) = {
    if ((locals.var_guard1216 == 0.0) && (locals.var_guard1218 == 0.0)) {
        let assign18440_e16586: f64 = (1.0 - 0.5);
        let assign18440_e16587: f64 = (2.0 * assign18440_e16586);
        let assign18440_e16590: f64 = (1.0 - 0.5);
        let assign18440_e16591: f64 = (assign18440_e16590).sqrt();
        let assign18440_e16592: f64 = (assign18440_e16587 * assign18440_e16591);
        let assign18440_e16593: f64 = (1.0 / assign18440_e16592);
        (assign18440_e16593, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12,)
    }
};
        locals.var_t11 = assign18440_e16595;
        locals.var_t11_dn3 = assign18440_e16595_d_n3;
        locals.var_t11_dn4 = assign18440_e16595_d_n4;
        locals.var_t11_dn5 = assign18440_e16595_d_n5;
        locals.var_t11_dn6 = assign18440_e16595_d_n6;
        locals.var_t11_dn7 = assign18440_e16595_d_n7;
        locals.var_t11_dn8 = assign18440_e16595_d_n8;
        locals.var_t11_dn9 = assign18440_e16595_d_n9;
        locals.var_t11_dn10 = assign18440_e16595_d_n10;
        locals.var_t11_dn11 = assign18440_e16595_d_n11;
        locals.var_t11_dn12 = assign18440_e16595_d_n12;
        locals.var_t11_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_55(
        locals: &mut StampLocals,
    ) {
        let (assign18450_e16612, assign18450_e16612_d_n3, assign18450_e16612_d_n4, assign18450_e16612_d_n5, assign18450_e16612_d_n6, assign18450_e16612_d_n7, assign18450_e16612_d_n8, assign18450_e16612_d_n9, assign18450_e16612_d_n10, assign18450_e16612_d_n11, assign18450_e16612_d_n12,) = {
    if ((locals.var_guard1216 == 0.0) && (locals.var_guard1218 == 0.0)) {
        let assign18450_e16604: f64 = (1.0 - 0.5);
        let assign18450_e16605: f64 = (assign18450_e16604).sqrt();
        let assign18450_e16606: f64 = (1.0 / assign18450_e16605);
        let assign18450_e16609: f64 = (locals.var_t11 * 0.5);
        let assign18450_e16610: f64 = (assign18450_e16606 - assign18450_e16609);
        (assign18450_e16610, (-(locals.var_t11_dn3 * 0.5)), (-(locals.var_t11_dn4 * 0.5)), (-(locals.var_t11_dn5 * 0.5)), (-(locals.var_t11_dn6 * 0.5)), (-(locals.var_t11_dn7 * 0.5)), (-(locals.var_t11_dn8 * 0.5)), (-(locals.var_t11_dn9 * 0.5)), (-(locals.var_t11_dn10 * 0.5)), (-(locals.var_t11_dn11 * 0.5)), (-(locals.var_t11_dn12 * 0.5)),)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12,)
    }
};
        locals.var_t12 = assign18450_e16612;
        locals.var_t12_dn3 = assign18450_e16612_d_n3;
        locals.var_t12_dn4 = assign18450_e16612_d_n4;
        locals.var_t12_dn5 = assign18450_e16612_d_n5;
        locals.var_t12_dn6 = assign18450_e16612_d_n6;
        locals.var_t12_dn7 = assign18450_e16612_d_n7;
        locals.var_t12_dn8 = assign18450_e16612_d_n8;
        locals.var_t12_dn9 = assign18450_e16612_d_n9;
        locals.var_t12_dn10 = assign18450_e16612_d_n10;
        locals.var_t12_dn11 = assign18450_e16612_d_n11;
        locals.var_t12_dn12 = assign18450_e16612_d_n12;
        locals.var_t12_rv = 0.0;

        let (assign18460_e16624, assign18460_e16624_d_n3, assign18460_e16624_d_n4, assign18460_e16624_d_n5, assign18460_e16624_d_n6, assign18460_e16624_d_n7, assign18460_e16624_d_n8, assign18460_e16624_d_n9, assign18460_e16624_d_n10, assign18460_e16624_d_n11, assign18460_e16624_d_n12,) = {
    if ((locals.var_guard1216 == 0.0) && (locals.var_guard1218 == 0.0)) {
        let assign18460_e16620: f64 = (locals.var_t11 * locals.var_t13);
        let assign18460_e16622: f64 = (assign18460_e16620 + locals.var_t12);
        (assign18460_e16622, (((locals.var_t11_dn3 * locals.var_t13) + (locals.var_t11 * locals.var_t13_dn3)) + locals.var_t12_dn3), (((locals.var_t11_dn4 * locals.var_t13) + (locals.var_t11 * locals.var_t13_dn4)) + locals.var_t12_dn4), (((locals.var_t11_dn5 * locals.var_t13) + (locals.var_t11 * locals.var_t13_dn5)) + locals.var_t12_dn5), (((locals.var_t11_dn6 * locals.var_t13) + (locals.var_t11 * locals.var_t13_dn6)) + locals.var_t12_dn6), (((locals.var_t11_dn7 * locals.var_t13) + (locals.var_t11 * locals.var_t13_dn7)) + locals.var_t12_dn7), (((locals.var_t11_dn8 * locals.var_t13) + (locals.var_t11 * locals.var_t13_dn8)) + locals.var_t12_dn8), (((locals.var_t11_dn9 * locals.var_t13) + (locals.var_t11 * locals.var_t13_dn9)) + locals.var_t12_dn9), (((locals.var_t11_dn10 * locals.var_t13) + (locals.var_t11 * locals.var_t13_dn10)) + locals.var_t12_dn10), (((locals.var_t11_dn11 * locals.var_t13) + (locals.var_t11 * locals.var_t13_dn11)) + locals.var_t12_dn11), (((locals.var_t11_dn12 * locals.var_t13) + (locals.var_t11 * locals.var_t13_dn12)) + locals.var_t12_dn12),)
    } else {
        (locals.var_t14, locals.var_t14_dn3, locals.var_t14_dn4, locals.var_t14_dn5, locals.var_t14_dn6, locals.var_t14_dn7, locals.var_t14_dn8, locals.var_t14_dn9, locals.var_t14_dn10, locals.var_t14_dn11, locals.var_t14_dn12,)
    }
};
        locals.var_t14 = assign18460_e16624;
        locals.var_t14_dn3 = assign18460_e16624_d_n3;
        locals.var_t14_dn4 = assign18460_e16624_d_n4;
        locals.var_t14_dn5 = assign18460_e16624_d_n5;
        locals.var_t14_dn6 = assign18460_e16624_d_n6;
        locals.var_t14_dn7 = assign18460_e16624_d_n7;
        locals.var_t14_dn8 = assign18460_e16624_d_n8;
        locals.var_t14_dn9 = assign18460_e16624_d_n9;
        locals.var_t14_dn10 = assign18460_e16624_d_n10;
        locals.var_t14_dn11 = assign18460_e16624_d_n11;
        locals.var_t14_dn12 = assign18460_e16624_d_n12;
        locals.var_t14_rv = 0.0;

        let (assign18470_e16638, assign18470_e16638_d_n3, assign18470_e16638_d_n4, assign18470_e16638_d_n5, assign18470_e16638_d_n6, assign18470_e16638_d_n7, assign18470_e16638_d_n8, assign18470_e16638_d_n9, assign18470_e16638_d_n10, assign18470_e16638_d_n11, assign18470_e16638_d_n12,) = {
    if (locals.var_guard1216 == 0.0) {
        let assign18470_e16629: f64 = (0.5 * locals.var_here_b4soik1ox);
        let assign18470_e16631: f64 = (assign18470_e16629 * locals.var_lpe_vb__blk1054);
        let assign18470_e16634: f64 = (locals.var_phi + locals.var_pparam_b4soiketas);
        let assign18470_e16635: f64 = (assign18470_e16634).sqrt();
        let assign18470_e16636: f64 = (assign18470_e16631 / assign18470_e16635);
        (assign18470_e16636, ((((((0.5 * locals.var_here_b4soik1ox_dn3) * locals.var_lpe_vb__blk1054) + (assign18470_e16629 * locals.var_lpe_vb__blk1054_dn3)) * assign18470_e16635) - (assign18470_e16631 * ((locals.var_phi_dn3 + locals.var_pparam_b4soiketas_dn3) / (2.0 * assign18470_e16635)))) / (assign18470_e16635 * assign18470_e16635)), ((((((0.5 * locals.var_here_b4soik1ox_dn4) * locals.var_lpe_vb__blk1054) + (assign18470_e16629 * locals.var_lpe_vb__blk1054_dn4)) * assign18470_e16635) - (assign18470_e16631 * ((locals.var_phi_dn4 + locals.var_pparam_b4soiketas_dn4) / (2.0 * assign18470_e16635)))) / (assign18470_e16635 * assign18470_e16635)), ((((((0.5 * locals.var_here_b4soik1ox_dn5) * locals.var_lpe_vb__blk1054) + (assign18470_e16629 * locals.var_lpe_vb__blk1054_dn5)) * assign18470_e16635) - (assign18470_e16631 * ((locals.var_phi_dn5 + locals.var_pparam_b4soiketas_dn5) / (2.0 * assign18470_e16635)))) / (assign18470_e16635 * assign18470_e16635)), ((((((0.5 * locals.var_here_b4soik1ox_dn6) * locals.var_lpe_vb__blk1054) + (assign18470_e16629 * locals.var_lpe_vb__blk1054_dn6)) * assign18470_e16635) - (assign18470_e16631 * ((locals.var_phi_dn6 + locals.var_pparam_b4soiketas_dn6) / (2.0 * assign18470_e16635)))) / (assign18470_e16635 * assign18470_e16635)), ((((((0.5 * locals.var_here_b4soik1ox_dn7) * locals.var_lpe_vb__blk1054) + (assign18470_e16629 * locals.var_lpe_vb__blk1054_dn7)) * assign18470_e16635) - (assign18470_e16631 * ((locals.var_phi_dn7 + locals.var_pparam_b4soiketas_dn7) / (2.0 * assign18470_e16635)))) / (assign18470_e16635 * assign18470_e16635)), ((((((0.5 * locals.var_here_b4soik1ox_dn8) * locals.var_lpe_vb__blk1054) + (assign18470_e16629 * locals.var_lpe_vb__blk1054_dn8)) * assign18470_e16635) - (assign18470_e16631 * ((locals.var_phi_dn8 + locals.var_pparam_b4soiketas_dn8) / (2.0 * assign18470_e16635)))) / (assign18470_e16635 * assign18470_e16635)), ((((((0.5 * locals.var_here_b4soik1ox_dn9) * locals.var_lpe_vb__blk1054) + (assign18470_e16629 * locals.var_lpe_vb__blk1054_dn9)) * assign18470_e16635) - (assign18470_e16631 * ((locals.var_phi_dn9 + locals.var_pparam_b4soiketas_dn9) / (2.0 * assign18470_e16635)))) / (assign18470_e16635 * assign18470_e16635)), ((((((0.5 * locals.var_here_b4soik1ox_dn10) * locals.var_lpe_vb__blk1054) + (assign18470_e16629 * locals.var_lpe_vb__blk1054_dn10)) * assign18470_e16635) - (assign18470_e16631 * ((locals.var_phi_dn10 + locals.var_pparam_b4soiketas_dn10) / (2.0 * assign18470_e16635)))) / (assign18470_e16635 * assign18470_e16635)), ((((((0.5 * locals.var_here_b4soik1ox_dn11) * locals.var_lpe_vb__blk1054) + (assign18470_e16629 * locals.var_lpe_vb__blk1054_dn11)) * assign18470_e16635) - (assign18470_e16631 * ((locals.var_phi_dn11 + locals.var_pparam_b4soiketas_dn11) / (2.0 * assign18470_e16635)))) / (assign18470_e16635 * assign18470_e16635)), ((((((0.5 * locals.var_here_b4soik1ox_dn12) * locals.var_lpe_vb__blk1054) + (assign18470_e16629 * locals.var_lpe_vb__blk1054_dn12)) * assign18470_e16635) - (assign18470_e16631 * ((locals.var_phi_dn12 + locals.var_pparam_b4soiketas_dn12) / (2.0 * assign18470_e16635)))) / (assign18470_e16635 * assign18470_e16635)),)
    } else {
        (locals.var_t10__blk818, locals.var_t10__blk818_dn3, locals.var_t10__blk818_dn4, locals.var_t10__blk818_dn5, locals.var_t10__blk818_dn6, locals.var_t10__blk818_dn7, locals.var_t10__blk818_dn8, locals.var_t10__blk818_dn9, locals.var_t10__blk818_dn10, locals.var_t10__blk818_dn11, locals.var_t10__blk818_dn12,)
    }
};
        locals.var_t10__blk818 = assign18470_e16638;
        locals.var_t10__blk818_dn3 = assign18470_e16638_d_n3;
        locals.var_t10__blk818_dn4 = assign18470_e16638_d_n4;
        locals.var_t10__blk818_dn5 = assign18470_e16638_d_n5;
        locals.var_t10__blk818_dn6 = assign18470_e16638_d_n6;
        locals.var_t10__blk818_dn7 = assign18470_e16638_d_n7;
        locals.var_t10__blk818_dn8 = assign18470_e16638_d_n8;
        locals.var_t10__blk818_dn9 = assign18470_e16638_d_n9;
        locals.var_t10__blk818_dn10 = assign18470_e16638_d_n10;
        locals.var_t10__blk818_dn11 = assign18470_e16638_d_n11;
        locals.var_t10__blk818_dn12 = assign18470_e16638_d_n12;
        locals.var_t10__blk818_rv = 0.0;

        let (assign18480_e16645, assign18480_e16645_d_n3, assign18480_e16645_d_n4, assign18480_e16645_d_n5, assign18480_e16645_d_n6, assign18480_e16645_d_n7, assign18480_e16645_d_n8, assign18480_e16645_d_n9, assign18480_e16645_d_n10, assign18480_e16645_d_n11, assign18480_e16645_d_n12,) = {
    if (locals.var_guard1216 == 0.0) {
        let assign18480_e16643: f64 = (locals.var_t10__blk818 * locals.var_t14);
        (assign18480_e16643, ((locals.var_t10__blk818_dn3 * locals.var_t14) + (locals.var_t10__blk818 * locals.var_t14_dn3)), ((locals.var_t10__blk818_dn4 * locals.var_t14) + (locals.var_t10__blk818 * locals.var_t14_dn4)), ((locals.var_t10__blk818_dn5 * locals.var_t14) + (locals.var_t10__blk818 * locals.var_t14_dn5)), ((locals.var_t10__blk818_dn6 * locals.var_t14) + (locals.var_t10__blk818 * locals.var_t14_dn6)), ((locals.var_t10__blk818_dn7 * locals.var_t14) + (locals.var_t10__blk818 * locals.var_t14_dn7)), ((locals.var_t10__blk818_dn8 * locals.var_t14) + (locals.var_t10__blk818 * locals.var_t14_dn8)), ((locals.var_t10__blk818_dn9 * locals.var_t14) + (locals.var_t10__blk818 * locals.var_t14_dn9)), ((locals.var_t10__blk818_dn10 * locals.var_t14) + (locals.var_t10__blk818 * locals.var_t14_dn10)), ((locals.var_t10__blk818_dn11 * locals.var_t14) + (locals.var_t10__blk818 * locals.var_t14_dn11)), ((locals.var_t10__blk818_dn12 * locals.var_t14) + (locals.var_t10__blk818 * locals.var_t14_dn12)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign18480_e16645;
        locals.var_t1__blk809_dn3 = assign18480_e16645_d_n3;
        locals.var_t1__blk809_dn4 = assign18480_e16645_d_n4;
        locals.var_t1__blk809_dn5 = assign18480_e16645_d_n5;
        locals.var_t1__blk809_dn6 = assign18480_e16645_d_n6;
        locals.var_t1__blk809_dn7 = assign18480_e16645_d_n7;
        locals.var_t1__blk809_dn8 = assign18480_e16645_d_n8;
        locals.var_t1__blk809_dn9 = assign18480_e16645_d_n9;
        locals.var_t1__blk809_dn10 = assign18480_e16645_d_n10;
        locals.var_t1__blk809_dn11 = assign18480_e16645_d_n11;
        locals.var_t1__blk809_dn12 = assign18480_e16645_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign18490_e16653, assign18490_e16653_d_n3, assign18490_e16653_d_n4, assign18490_e16653_d_n5, assign18490_e16653_d_n6, assign18490_e16653_d_n7, assign18490_e16653_d_n8, assign18490_e16653_d_n9, assign18490_e16653_d_n10, assign18490_e16653_d_n11, assign18490_e16653_d_n12,) = {
    if (locals.var_guard1216 == 0.0) {
        let assign18490_e16650: f64 = (locals.var_pparam_b4soixj * locals.var_xdep);
        let assign18490_e16651: f64 = (assign18490_e16650).sqrt();
        (assign18490_e16651, (((locals.var_pparam_b4soixj_dn3 * locals.var_xdep) + (locals.var_pparam_b4soixj * locals.var_xdep_dn3)) / (2.0 * assign18490_e16651)), (((locals.var_pparam_b4soixj_dn4 * locals.var_xdep) + (locals.var_pparam_b4soixj * locals.var_xdep_dn4)) / (2.0 * assign18490_e16651)), (((locals.var_pparam_b4soixj_dn5 * locals.var_xdep) + (locals.var_pparam_b4soixj * locals.var_xdep_dn5)) / (2.0 * assign18490_e16651)), (((locals.var_pparam_b4soixj_dn6 * locals.var_xdep) + (locals.var_pparam_b4soixj * locals.var_xdep_dn6)) / (2.0 * assign18490_e16651)), (((locals.var_pparam_b4soixj_dn7 * locals.var_xdep) + (locals.var_pparam_b4soixj * locals.var_xdep_dn7)) / (2.0 * assign18490_e16651)), (((locals.var_pparam_b4soixj_dn8 * locals.var_xdep) + (locals.var_pparam_b4soixj * locals.var_xdep_dn8)) / (2.0 * assign18490_e16651)), (((locals.var_pparam_b4soixj_dn9 * locals.var_xdep) + (locals.var_pparam_b4soixj * locals.var_xdep_dn9)) / (2.0 * assign18490_e16651)), (((locals.var_pparam_b4soixj_dn10 * locals.var_xdep) + (locals.var_pparam_b4soixj * locals.var_xdep_dn10)) / (2.0 * assign18490_e16651)), (((locals.var_pparam_b4soixj_dn11 * locals.var_xdep) + (locals.var_pparam_b4soixj * locals.var_xdep_dn11)) / (2.0 * assign18490_e16651)), (((locals.var_pparam_b4soixj_dn12 * locals.var_xdep) + (locals.var_pparam_b4soixj * locals.var_xdep_dn12)) / (2.0 * assign18490_e16651)),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign18490_e16653;
        locals.var_t9_dn3 = assign18490_e16653_d_n3;
        locals.var_t9_dn4 = assign18490_e16653_d_n4;
        locals.var_t9_dn5 = assign18490_e16653_d_n5;
        locals.var_t9_dn6 = assign18490_e16653_d_n6;
        locals.var_t9_dn7 = assign18490_e16653_d_n7;
        locals.var_t9_dn8 = assign18490_e16653_d_n8;
        locals.var_t9_dn9 = assign18490_e16653_d_n9;
        locals.var_t9_dn10 = assign18490_e16653_d_n10;
        locals.var_t9_dn11 = assign18490_e16653_d_n11;
        locals.var_t9_dn12 = assign18490_e16653_d_n12;
        locals.var_t9_rv = 0.0;

        let (assign18500_e16662, assign18500_e16662_d_n3, assign18500_e16662_d_n4, assign18500_e16662_d_n5, assign18500_e16662_d_n6, assign18500_e16662_d_n7, assign18500_e16662_d_n8, assign18500_e16662_d_n9, assign18500_e16662_d_n10, assign18500_e16662_d_n11, assign18500_e16662_d_n12,) = {
    if (locals.var_guard1216 == 0.0) {
        let assign18500_e16659: f64 = (2.0 * locals.var_t9);
        let assign18500_e16660: f64 = (locals.var_leff + assign18500_e16659);
        (assign18500_e16660, (locals.var_leff_dn3 + (2.0 * locals.var_t9_dn3)), (locals.var_leff_dn4 + (2.0 * locals.var_t9_dn4)), (locals.var_leff_dn5 + (2.0 * locals.var_t9_dn5)), (locals.var_leff_dn6 + (2.0 * locals.var_t9_dn6)), (locals.var_leff_dn7 + (2.0 * locals.var_t9_dn7)), (locals.var_leff_dn8 + (2.0 * locals.var_t9_dn8)), (locals.var_leff_dn9 + (2.0 * locals.var_t9_dn9)), (locals.var_leff_dn10 + (2.0 * locals.var_t9_dn10)), (locals.var_leff_dn11 + (2.0 * locals.var_t9_dn11)), (locals.var_leff_dn12 + (2.0 * locals.var_t9_dn12)),)
    } else {
        (locals.var_tmp1__blk834, locals.var_tmp1__blk834_dn3, locals.var_tmp1__blk834_dn4, locals.var_tmp1__blk834_dn5, locals.var_tmp1__blk834_dn6, locals.var_tmp1__blk834_dn7, locals.var_tmp1__blk834_dn8, locals.var_tmp1__blk834_dn9, locals.var_tmp1__blk834_dn10, locals.var_tmp1__blk834_dn11, locals.var_tmp1__blk834_dn12,)
    }
};
        locals.var_tmp1__blk834 = assign18500_e16662;
        locals.var_tmp1__blk834_dn3 = assign18500_e16662_d_n3;
        locals.var_tmp1__blk834_dn4 = assign18500_e16662_d_n4;
        locals.var_tmp1__blk834_dn5 = assign18500_e16662_d_n5;
        locals.var_tmp1__blk834_dn6 = assign18500_e16662_d_n6;
        locals.var_tmp1__blk834_dn7 = assign18500_e16662_d_n7;
        locals.var_tmp1__blk834_dn8 = assign18500_e16662_d_n8;
        locals.var_tmp1__blk834_dn9 = assign18500_e16662_d_n9;
        locals.var_tmp1__blk834_dn10 = assign18500_e16662_d_n10;
        locals.var_tmp1__blk834_dn11 = assign18500_e16662_d_n11;
        locals.var_tmp1__blk834_dn12 = assign18500_e16662_d_n12;
        locals.var_tmp1__blk834_rv = 0.0;

        let (assign18510_e16669, assign18510_e16669_d_n3, assign18510_e16669_d_n4, assign18510_e16669_d_n5, assign18510_e16669_d_n6, assign18510_e16669_d_n7, assign18510_e16669_d_n8, assign18510_e16669_d_n9, assign18510_e16669_d_n10, assign18510_e16669_d_n11, assign18510_e16669_d_n12,) = {
    if (locals.var_guard1216 == 0.0) {
        let assign18510_e16667: f64 = (locals.var_leff / locals.var_tmp1__blk834);
        (assign18510_e16667, (((locals.var_leff_dn3 * locals.var_tmp1__blk834) - (locals.var_leff * locals.var_tmp1__blk834_dn3)) / (locals.var_tmp1__blk834 * locals.var_tmp1__blk834)), (((locals.var_leff_dn4 * locals.var_tmp1__blk834) - (locals.var_leff * locals.var_tmp1__blk834_dn4)) / (locals.var_tmp1__blk834 * locals.var_tmp1__blk834)), (((locals.var_leff_dn5 * locals.var_tmp1__blk834) - (locals.var_leff * locals.var_tmp1__blk834_dn5)) / (locals.var_tmp1__blk834 * locals.var_tmp1__blk834)), (((locals.var_leff_dn6 * locals.var_tmp1__blk834) - (locals.var_leff * locals.var_tmp1__blk834_dn6)) / (locals.var_tmp1__blk834 * locals.var_tmp1__blk834)), (((locals.var_leff_dn7 * locals.var_tmp1__blk834) - (locals.var_leff * locals.var_tmp1__blk834_dn7)) / (locals.var_tmp1__blk834 * locals.var_tmp1__blk834)), (((locals.var_leff_dn8 * locals.var_tmp1__blk834) - (locals.var_leff * locals.var_tmp1__blk834_dn8)) / (locals.var_tmp1__blk834 * locals.var_tmp1__blk834)), (((locals.var_leff_dn9 * locals.var_tmp1__blk834) - (locals.var_leff * locals.var_tmp1__blk834_dn9)) / (locals.var_tmp1__blk834 * locals.var_tmp1__blk834)), (((locals.var_leff_dn10 * locals.var_tmp1__blk834) - (locals.var_leff * locals.var_tmp1__blk834_dn10)) / (locals.var_tmp1__blk834 * locals.var_tmp1__blk834)), (((locals.var_leff_dn11 * locals.var_tmp1__blk834) - (locals.var_leff * locals.var_tmp1__blk834_dn11)) / (locals.var_tmp1__blk834 * locals.var_tmp1__blk834)), (((locals.var_leff_dn12 * locals.var_tmp1__blk834) - (locals.var_leff * locals.var_tmp1__blk834_dn12)) / (locals.var_tmp1__blk834 * locals.var_tmp1__blk834)),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign18510_e16669;
        locals.var_t5__blk813_dn3 = assign18510_e16669_d_n3;
        locals.var_t5__blk813_dn4 = assign18510_e16669_d_n4;
        locals.var_t5__blk813_dn5 = assign18510_e16669_d_n5;
        locals.var_t5__blk813_dn6 = assign18510_e16669_d_n6;
        locals.var_t5__blk813_dn7 = assign18510_e16669_d_n7;
        locals.var_t5__blk813_dn8 = assign18510_e16669_d_n8;
        locals.var_t5__blk813_dn9 = assign18510_e16669_d_n9;
        locals.var_t5__blk813_dn10 = assign18510_e16669_d_n10;
        locals.var_t5__blk813_dn11 = assign18510_e16669_d_n11;
        locals.var_t5__blk813_dn12 = assign18510_e16669_d_n12;
        locals.var_t5__blk813_rv = 0.0;

        let (assign18520_e16676, assign18520_e16676_d_n3, assign18520_e16676_d_n4, assign18520_e16676_d_n5, assign18520_e16676_d_n6, assign18520_e16676_d_n7, assign18520_e16676_d_n8, assign18520_e16676_d_n9, assign18520_e16676_d_n10, assign18520_e16676_d_n11, assign18520_e16676_d_n12,) = {
    if (locals.var_guard1216 == 0.0) {
        let assign18520_e16674: f64 = (locals.var_pparam_b4soia0 * locals.var_t5__blk813);
        (assign18520_e16674, ((locals.var_pparam_b4soia0_dn3 * locals.var_t5__blk813) + (locals.var_pparam_b4soia0 * locals.var_t5__blk813_dn3)), ((locals.var_pparam_b4soia0_dn4 * locals.var_t5__blk813) + (locals.var_pparam_b4soia0 * locals.var_t5__blk813_dn4)), ((locals.var_pparam_b4soia0_dn5 * locals.var_t5__blk813) + (locals.var_pparam_b4soia0 * locals.var_t5__blk813_dn5)), ((locals.var_pparam_b4soia0_dn6 * locals.var_t5__blk813) + (locals.var_pparam_b4soia0 * locals.var_t5__blk813_dn6)), ((locals.var_pparam_b4soia0_dn7 * locals.var_t5__blk813) + (locals.var_pparam_b4soia0 * locals.var_t5__blk813_dn7)), ((locals.var_pparam_b4soia0_dn8 * locals.var_t5__blk813) + (locals.var_pparam_b4soia0 * locals.var_t5__blk813_dn8)), ((locals.var_pparam_b4soia0_dn9 * locals.var_t5__blk813) + (locals.var_pparam_b4soia0 * locals.var_t5__blk813_dn9)), ((locals.var_pparam_b4soia0_dn10 * locals.var_t5__blk813) + (locals.var_pparam_b4soia0 * locals.var_t5__blk813_dn10)), ((locals.var_pparam_b4soia0_dn11 * locals.var_t5__blk813) + (locals.var_pparam_b4soia0 * locals.var_t5__blk813_dn11)), ((locals.var_pparam_b4soia0_dn12 * locals.var_t5__blk813) + (locals.var_pparam_b4soia0 * locals.var_t5__blk813_dn12)),)
    } else {
        (locals.var_tmp2__blk835, locals.var_tmp2__blk835_dn3, locals.var_tmp2__blk835_dn4, locals.var_tmp2__blk835_dn5, locals.var_tmp2__blk835_dn6, locals.var_tmp2__blk835_dn7, locals.var_tmp2__blk835_dn8, locals.var_tmp2__blk835_dn9, locals.var_tmp2__blk835_dn10, locals.var_tmp2__blk835_dn11, locals.var_tmp2__blk835_dn12,)
    }
};
        locals.var_tmp2__blk835 = assign18520_e16676;
        locals.var_tmp2__blk835_dn3 = assign18520_e16676_d_n3;
        locals.var_tmp2__blk835_dn4 = assign18520_e16676_d_n4;
        locals.var_tmp2__blk835_dn5 = assign18520_e16676_d_n5;
        locals.var_tmp2__blk835_dn6 = assign18520_e16676_d_n6;
        locals.var_tmp2__blk835_dn7 = assign18520_e16676_d_n7;
        locals.var_tmp2__blk835_dn8 = assign18520_e16676_d_n8;
        locals.var_tmp2__blk835_dn9 = assign18520_e16676_d_n9;
        locals.var_tmp2__blk835_dn10 = assign18520_e16676_d_n10;
        locals.var_tmp2__blk835_dn11 = assign18520_e16676_d_n11;
        locals.var_tmp2__blk835_dn12 = assign18520_e16676_d_n12;
        locals.var_tmp2__blk835_rv = 0.0;

        let (assign18530_e16683, assign18530_e16683_d_n3, assign18530_e16683_d_n4, assign18530_e16683_d_n5, assign18530_e16683_d_n6, assign18530_e16683_d_n7, assign18530_e16683_d_n8, assign18530_e16683_d_n9, assign18530_e16683_d_n10, assign18530_e16683_d_n11, assign18530_e16683_d_n12,) = {
    if (locals.var_guard1216 == 0.0) {
        let assign18530_e16681: f64 = (locals.var_pparam_b4soiweff + locals.var_pparam_b4soib1);
        (assign18530_e16681, (locals.var_pparam_b4soiweff_dn3 + locals.var_pparam_b4soib1_dn3), (locals.var_pparam_b4soiweff_dn4 + locals.var_pparam_b4soib1_dn4), (locals.var_pparam_b4soiweff_dn5 + locals.var_pparam_b4soib1_dn5), (locals.var_pparam_b4soiweff_dn6 + locals.var_pparam_b4soib1_dn6), (locals.var_pparam_b4soiweff_dn7 + locals.var_pparam_b4soib1_dn7), (locals.var_pparam_b4soiweff_dn8 + locals.var_pparam_b4soib1_dn8), (locals.var_pparam_b4soiweff_dn9 + locals.var_pparam_b4soib1_dn9), (locals.var_pparam_b4soiweff_dn10 + locals.var_pparam_b4soib1_dn10), (locals.var_pparam_b4soiweff_dn11 + locals.var_pparam_b4soib1_dn11), (locals.var_pparam_b4soiweff_dn12 + locals.var_pparam_b4soib1_dn12),)
    } else {
        (locals.var_tmp3__blk836, locals.var_tmp3__blk836_dn3, locals.var_tmp3__blk836_dn4, locals.var_tmp3__blk836_dn5, locals.var_tmp3__blk836_dn6, locals.var_tmp3__blk836_dn7, locals.var_tmp3__blk836_dn8, locals.var_tmp3__blk836_dn9, locals.var_tmp3__blk836_dn10, locals.var_tmp3__blk836_dn11, locals.var_tmp3__blk836_dn12,)
    }
};
        locals.var_tmp3__blk836 = assign18530_e16683;
        locals.var_tmp3__blk836_dn3 = assign18530_e16683_d_n3;
        locals.var_tmp3__blk836_dn4 = assign18530_e16683_d_n4;
        locals.var_tmp3__blk836_dn5 = assign18530_e16683_d_n5;
        locals.var_tmp3__blk836_dn6 = assign18530_e16683_d_n6;
        locals.var_tmp3__blk836_dn7 = assign18530_e16683_d_n7;
        locals.var_tmp3__blk836_dn8 = assign18530_e16683_d_n8;
        locals.var_tmp3__blk836_dn9 = assign18530_e16683_d_n9;
        locals.var_tmp3__blk836_dn10 = assign18530_e16683_d_n10;
        locals.var_tmp3__blk836_dn11 = assign18530_e16683_d_n11;
        locals.var_tmp3__blk836_dn12 = assign18530_e16683_d_n12;
        locals.var_tmp3__blk836_rv = 0.0;

        let (assign18540_e16690, assign18540_e16690_d_n3, assign18540_e16690_d_n4, assign18540_e16690_d_n5, assign18540_e16690_d_n6, assign18540_e16690_d_n7, assign18540_e16690_d_n8, assign18540_e16690_d_n9, assign18540_e16690_d_n10, assign18540_e16690_d_n11, assign18540_e16690_d_n12,) = {
    if (locals.var_guard1216 == 0.0) {
        let assign18540_e16688: f64 = (locals.var_pparam_b4soib0 / locals.var_tmp3__blk836);
        (assign18540_e16688, (((locals.var_pparam_b4soib0_dn3 * locals.var_tmp3__blk836) - (locals.var_pparam_b4soib0 * locals.var_tmp3__blk836_dn3)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836)), (((locals.var_pparam_b4soib0_dn4 * locals.var_tmp3__blk836) - (locals.var_pparam_b4soib0 * locals.var_tmp3__blk836_dn4)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836)), (((locals.var_pparam_b4soib0_dn5 * locals.var_tmp3__blk836) - (locals.var_pparam_b4soib0 * locals.var_tmp3__blk836_dn5)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836)), (((locals.var_pparam_b4soib0_dn6 * locals.var_tmp3__blk836) - (locals.var_pparam_b4soib0 * locals.var_tmp3__blk836_dn6)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836)), (((locals.var_pparam_b4soib0_dn7 * locals.var_tmp3__blk836) - (locals.var_pparam_b4soib0 * locals.var_tmp3__blk836_dn7)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836)), (((locals.var_pparam_b4soib0_dn8 * locals.var_tmp3__blk836) - (locals.var_pparam_b4soib0 * locals.var_tmp3__blk836_dn8)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836)), (((locals.var_pparam_b4soib0_dn9 * locals.var_tmp3__blk836) - (locals.var_pparam_b4soib0 * locals.var_tmp3__blk836_dn9)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836)), (((locals.var_pparam_b4soib0_dn10 * locals.var_tmp3__blk836) - (locals.var_pparam_b4soib0 * locals.var_tmp3__blk836_dn10)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836)), (((locals.var_pparam_b4soib0_dn11 * locals.var_tmp3__blk836) - (locals.var_pparam_b4soib0 * locals.var_tmp3__blk836_dn11)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836)), (((locals.var_pparam_b4soib0_dn12 * locals.var_tmp3__blk836) - (locals.var_pparam_b4soib0 * locals.var_tmp3__blk836_dn12)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836)),)
    } else {
        (locals.var_tmp4, locals.var_tmp4_dn3, locals.var_tmp4_dn4, locals.var_tmp4_dn5, locals.var_tmp4_dn6, locals.var_tmp4_dn7, locals.var_tmp4_dn8, locals.var_tmp4_dn9, locals.var_tmp4_dn10, locals.var_tmp4_dn11, locals.var_tmp4_dn12,)
    }
};
        locals.var_tmp4 = assign18540_e16690;
        locals.var_tmp4_dn3 = assign18540_e16690_d_n3;
        locals.var_tmp4_dn4 = assign18540_e16690_d_n4;
        locals.var_tmp4_dn5 = assign18540_e16690_d_n5;
        locals.var_tmp4_dn6 = assign18540_e16690_d_n6;
        locals.var_tmp4_dn7 = assign18540_e16690_d_n7;
        locals.var_tmp4_dn8 = assign18540_e16690_d_n8;
        locals.var_tmp4_dn9 = assign18540_e16690_d_n9;
        locals.var_tmp4_dn10 = assign18540_e16690_d_n10;
        locals.var_tmp4_dn11 = assign18540_e16690_d_n11;
        locals.var_tmp4_dn12 = assign18540_e16690_d_n12;
        locals.var_tmp4_rv = 0.0;

        let (assign18550_e16697, assign18550_e16697_d_n3, assign18550_e16697_d_n4, assign18550_e16697_d_n5, assign18550_e16697_d_n6, assign18550_e16697_d_n7, assign18550_e16697_d_n8, assign18550_e16697_d_n9, assign18550_e16697_d_n10, assign18550_e16697_d_n11, assign18550_e16697_d_n12,) = {
    if (locals.var_guard1216 == 0.0) {
        let assign18550_e16695: f64 = (locals.var_tmp2__blk835 + locals.var_tmp4);
        (assign18550_e16695, (locals.var_tmp2__blk835_dn3 + locals.var_tmp4_dn3), (locals.var_tmp2__blk835_dn4 + locals.var_tmp4_dn4), (locals.var_tmp2__blk835_dn5 + locals.var_tmp4_dn5), (locals.var_tmp2__blk835_dn6 + locals.var_tmp4_dn6), (locals.var_tmp2__blk835_dn7 + locals.var_tmp4_dn7), (locals.var_tmp2__blk835_dn8 + locals.var_tmp4_dn8), (locals.var_tmp2__blk835_dn9 + locals.var_tmp4_dn9), (locals.var_tmp2__blk835_dn10 + locals.var_tmp4_dn10), (locals.var_tmp2__blk835_dn11 + locals.var_tmp4_dn11), (locals.var_tmp2__blk835_dn12 + locals.var_tmp4_dn12),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign18550_e16697;
        locals.var_t2__blk810_dn3 = assign18550_e16697_d_n3;
        locals.var_t2__blk810_dn4 = assign18550_e16697_d_n4;
        locals.var_t2__blk810_dn5 = assign18550_e16697_d_n5;
        locals.var_t2__blk810_dn6 = assign18550_e16697_d_n6;
        locals.var_t2__blk810_dn7 = assign18550_e16697_d_n7;
        locals.var_t2__blk810_dn8 = assign18550_e16697_d_n8;
        locals.var_t2__blk810_dn9 = assign18550_e16697_d_n9;
        locals.var_t2__blk810_dn10 = assign18550_e16697_d_n10;
        locals.var_t2__blk810_dn11 = assign18550_e16697_d_n11;
        locals.var_t2__blk810_dn12 = assign18550_e16697_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign18560_e16704, assign18560_e16704_d_n3, assign18560_e16704_d_n4, assign18560_e16704_d_n5, assign18560_e16704_d_n6, assign18560_e16704_d_n7, assign18560_e16704_d_n8, assign18560_e16704_d_n9, assign18560_e16704_d_n10, assign18560_e16704_d_n11, assign18560_e16704_d_n12,) = {
    if (locals.var_guard1216 == 0.0) {
        let assign18560_e16702: f64 = (locals.var_t5__blk813 * locals.var_t5__blk813);
        (assign18560_e16702, ((locals.var_t5__blk813_dn3 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn3)), ((locals.var_t5__blk813_dn4 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn4)), ((locals.var_t5__blk813_dn5 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn5)), ((locals.var_t5__blk813_dn6 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn6)), ((locals.var_t5__blk813_dn7 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn7)), ((locals.var_t5__blk813_dn8 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn8)), ((locals.var_t5__blk813_dn9 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn9)), ((locals.var_t5__blk813_dn10 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn10)), ((locals.var_t5__blk813_dn11 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn11)), ((locals.var_t5__blk813_dn12 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn12)),)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign18560_e16704;
        locals.var_t6__blk814_dn3 = assign18560_e16704_d_n3;
        locals.var_t6__blk814_dn4 = assign18560_e16704_d_n4;
        locals.var_t6__blk814_dn5 = assign18560_e16704_d_n5;
        locals.var_t6__blk814_dn6 = assign18560_e16704_d_n6;
        locals.var_t6__blk814_dn7 = assign18560_e16704_d_n7;
        locals.var_t6__blk814_dn8 = assign18560_e16704_d_n8;
        locals.var_t6__blk814_dn9 = assign18560_e16704_d_n9;
        locals.var_t6__blk814_dn10 = assign18560_e16704_d_n10;
        locals.var_t6__blk814_dn11 = assign18560_e16704_d_n11;
        locals.var_t6__blk814_dn12 = assign18560_e16704_d_n12;
        locals.var_t6__blk814_rv = 0.0;

        let (assign18570_e16711, assign18570_e16711_d_n3, assign18570_e16711_d_n4, assign18570_e16711_d_n5, assign18570_e16711_d_n6, assign18570_e16711_d_n7, assign18570_e16711_d_n8, assign18570_e16711_d_n9, assign18570_e16711_d_n10, assign18570_e16711_d_n11, assign18570_e16711_d_n12,) = {
    if (locals.var_guard1216 == 0.0) {
        let assign18570_e16709: f64 = (locals.var_t5__blk813 * locals.var_t6__blk814);
        (assign18570_e16709, ((locals.var_t5__blk813_dn3 * locals.var_t6__blk814) + (locals.var_t5__blk813 * locals.var_t6__blk814_dn3)), ((locals.var_t5__blk813_dn4 * locals.var_t6__blk814) + (locals.var_t5__blk813 * locals.var_t6__blk814_dn4)), ((locals.var_t5__blk813_dn5 * locals.var_t6__blk814) + (locals.var_t5__blk813 * locals.var_t6__blk814_dn5)), ((locals.var_t5__blk813_dn6 * locals.var_t6__blk814) + (locals.var_t5__blk813 * locals.var_t6__blk814_dn6)), ((locals.var_t5__blk813_dn7 * locals.var_t6__blk814) + (locals.var_t5__blk813 * locals.var_t6__blk814_dn7)), ((locals.var_t5__blk813_dn8 * locals.var_t6__blk814) + (locals.var_t5__blk813 * locals.var_t6__blk814_dn8)), ((locals.var_t5__blk813_dn9 * locals.var_t6__blk814) + (locals.var_t5__blk813 * locals.var_t6__blk814_dn9)), ((locals.var_t5__blk813_dn10 * locals.var_t6__blk814) + (locals.var_t5__blk813 * locals.var_t6__blk814_dn10)), ((locals.var_t5__blk813_dn11 * locals.var_t6__blk814) + (locals.var_t5__blk813 * locals.var_t6__blk814_dn11)), ((locals.var_t5__blk813_dn12 * locals.var_t6__blk814) + (locals.var_t5__blk813 * locals.var_t6__blk814_dn12)),)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign18570_e16711;
        locals.var_t7__blk815_dn3 = assign18570_e16711_d_n3;
        locals.var_t7__blk815_dn4 = assign18570_e16711_d_n4;
        locals.var_t7__blk815_dn5 = assign18570_e16711_d_n5;
        locals.var_t7__blk815_dn6 = assign18570_e16711_d_n6;
        locals.var_t7__blk815_dn7 = assign18570_e16711_d_n7;
        locals.var_t7__blk815_dn8 = assign18570_e16711_d_n8;
        locals.var_t7__blk815_dn9 = assign18570_e16711_d_n9;
        locals.var_t7__blk815_dn10 = assign18570_e16711_d_n10;
        locals.var_t7__blk815_dn11 = assign18570_e16711_d_n11;
        locals.var_t7__blk815_dn12 = assign18570_e16711_d_n12;
        locals.var_t7__blk815_rv = 0.0;

        let (assign18580_e16720, assign18580_e16720_d_n3, assign18580_e16720_d_n4, assign18580_e16720_d_n5, assign18580_e16720_d_n6, assign18580_e16720_d_n7, assign18580_e16720_d_n8, assign18580_e16720_d_n9, assign18580_e16720_d_n10, assign18580_e16720_d_n11, assign18580_e16720_d_n12,) = {
    if (locals.var_guard1216 == 0.0) {
        let assign18580_e16717: f64 = (locals.var_t1__blk809 * locals.var_t2__blk810);
        let assign18580_e16718: f64 = (1.0 + assign18580_e16717);
        (assign18580_e16718, ((locals.var_t1__blk809_dn3 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn3)), ((locals.var_t1__blk809_dn4 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn4)), ((locals.var_t1__blk809_dn5 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn5)), ((locals.var_t1__blk809_dn6 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn6)), ((locals.var_t1__blk809_dn7 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn7)), ((locals.var_t1__blk809_dn8 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn8)), ((locals.var_t1__blk809_dn9 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn9)), ((locals.var_t1__blk809_dn10 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn10)), ((locals.var_t1__blk809_dn11 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn11)), ((locals.var_t1__blk809_dn12 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn12)),)
    } else {
        (locals.var_abulk0, locals.var_abulk0_dn3, locals.var_abulk0_dn4, locals.var_abulk0_dn5, locals.var_abulk0_dn6, locals.var_abulk0_dn7, locals.var_abulk0_dn8, locals.var_abulk0_dn9, locals.var_abulk0_dn10, locals.var_abulk0_dn11, locals.var_abulk0_dn12,)
    }
};
        locals.var_abulk0 = assign18580_e16720;
        locals.var_abulk0_dn3 = assign18580_e16720_d_n3;
        locals.var_abulk0_dn4 = assign18580_e16720_d_n4;
        locals.var_abulk0_dn5 = assign18580_e16720_d_n5;
        locals.var_abulk0_dn6 = assign18580_e16720_d_n6;
        locals.var_abulk0_dn7 = assign18580_e16720_d_n7;
        locals.var_abulk0_dn8 = assign18580_e16720_d_n8;
        locals.var_abulk0_dn9 = assign18580_e16720_d_n9;
        locals.var_abulk0_dn10 = assign18580_e16720_d_n10;
        locals.var_abulk0_dn11 = assign18580_e16720_d_n11;
        locals.var_abulk0_dn12 = assign18580_e16720_d_n12;
        locals.var_abulk0_rv = 0.0;

        let (assign18590_e16729, assign18590_e16729_d_n3, assign18590_e16729_d_n4, assign18590_e16729_d_n5, assign18590_e16729_d_n6, assign18590_e16729_d_n7, assign18590_e16729_d_n8, assign18590_e16729_d_n9, assign18590_e16729_d_n10, assign18590_e16729_d_n11, assign18590_e16729_d_n12,) = {
    if (locals.var_guard1216 == 0.0) {
        let assign18590_e16725: f64 = (locals.var_pparam_b4soiags * locals.var_pparam_b4soia0);
        let assign18590_e16727: f64 = (assign18590_e16725 * locals.var_t7__blk815);
        (assign18590_e16727, ((((locals.var_pparam_b4soiags_dn3 * locals.var_pparam_b4soia0) + (locals.var_pparam_b4soiags * locals.var_pparam_b4soia0_dn3)) * locals.var_t7__blk815) + (assign18590_e16725 * locals.var_t7__blk815_dn3)), ((((locals.var_pparam_b4soiags_dn4 * locals.var_pparam_b4soia0) + (locals.var_pparam_b4soiags * locals.var_pparam_b4soia0_dn4)) * locals.var_t7__blk815) + (assign18590_e16725 * locals.var_t7__blk815_dn4)), ((((locals.var_pparam_b4soiags_dn5 * locals.var_pparam_b4soia0) + (locals.var_pparam_b4soiags * locals.var_pparam_b4soia0_dn5)) * locals.var_t7__blk815) + (assign18590_e16725 * locals.var_t7__blk815_dn5)), ((((locals.var_pparam_b4soiags_dn6 * locals.var_pparam_b4soia0) + (locals.var_pparam_b4soiags * locals.var_pparam_b4soia0_dn6)) * locals.var_t7__blk815) + (assign18590_e16725 * locals.var_t7__blk815_dn6)), ((((locals.var_pparam_b4soiags_dn7 * locals.var_pparam_b4soia0) + (locals.var_pparam_b4soiags * locals.var_pparam_b4soia0_dn7)) * locals.var_t7__blk815) + (assign18590_e16725 * locals.var_t7__blk815_dn7)), ((((locals.var_pparam_b4soiags_dn8 * locals.var_pparam_b4soia0) + (locals.var_pparam_b4soiags * locals.var_pparam_b4soia0_dn8)) * locals.var_t7__blk815) + (assign18590_e16725 * locals.var_t7__blk815_dn8)), ((((locals.var_pparam_b4soiags_dn9 * locals.var_pparam_b4soia0) + (locals.var_pparam_b4soiags * locals.var_pparam_b4soia0_dn9)) * locals.var_t7__blk815) + (assign18590_e16725 * locals.var_t7__blk815_dn9)), ((((locals.var_pparam_b4soiags_dn10 * locals.var_pparam_b4soia0) + (locals.var_pparam_b4soiags * locals.var_pparam_b4soia0_dn10)) * locals.var_t7__blk815) + (assign18590_e16725 * locals.var_t7__blk815_dn10)), ((((locals.var_pparam_b4soiags_dn11 * locals.var_pparam_b4soia0) + (locals.var_pparam_b4soiags * locals.var_pparam_b4soia0_dn11)) * locals.var_t7__blk815) + (assign18590_e16725 * locals.var_t7__blk815_dn11)), ((((locals.var_pparam_b4soiags_dn12 * locals.var_pparam_b4soia0) + (locals.var_pparam_b4soiags * locals.var_pparam_b4soia0_dn12)) * locals.var_t7__blk815) + (assign18590_e16725 * locals.var_t7__blk815_dn12)),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12,)
    }
};
        locals.var_t8 = assign18590_e16729;
        locals.var_t8_dn3 = assign18590_e16729_d_n3;
        locals.var_t8_dn4 = assign18590_e16729_d_n4;
        locals.var_t8_dn5 = assign18590_e16729_d_n5;
        locals.var_t8_dn6 = assign18590_e16729_d_n6;
        locals.var_t8_dn7 = assign18590_e16729_d_n7;
        locals.var_t8_dn8 = assign18590_e16729_d_n8;
        locals.var_t8_dn9 = assign18590_e16729_d_n9;
        locals.var_t8_dn10 = assign18590_e16729_d_n10;
        locals.var_t8_dn11 = assign18590_e16729_d_n11;
        locals.var_t8_dn12 = assign18590_e16729_d_n12;
        locals.var_t8_rv = 0.0;

        let (assign18600_e16737, assign18600_e16737_d_n3, assign18600_e16737_d_n4, assign18600_e16737_d_n5, assign18600_e16737_d_n6, assign18600_e16737_d_n7, assign18600_e16737_d_n8, assign18600_e16737_d_n9, assign18600_e16737_d_n10, assign18600_e16737_d_n11, assign18600_e16737_d_n12,) = {
    if (locals.var_guard1216 == 0.0) {
        let assign18600_e16733: f64 = (-locals.var_t1__blk809);
        let assign18600_e16735: f64 = (assign18600_e16733 * locals.var_t8);
        (assign18600_e16735, (((-locals.var_t1__blk809_dn3) * locals.var_t8) + (assign18600_e16733 * locals.var_t8_dn3)), (((-locals.var_t1__blk809_dn4) * locals.var_t8) + (assign18600_e16733 * locals.var_t8_dn4)), (((-locals.var_t1__blk809_dn5) * locals.var_t8) + (assign18600_e16733 * locals.var_t8_dn5)), (((-locals.var_t1__blk809_dn6) * locals.var_t8) + (assign18600_e16733 * locals.var_t8_dn6)), (((-locals.var_t1__blk809_dn7) * locals.var_t8) + (assign18600_e16733 * locals.var_t8_dn7)), (((-locals.var_t1__blk809_dn8) * locals.var_t8) + (assign18600_e16733 * locals.var_t8_dn8)), (((-locals.var_t1__blk809_dn9) * locals.var_t8) + (assign18600_e16733 * locals.var_t8_dn9)), (((-locals.var_t1__blk809_dn10) * locals.var_t8) + (assign18600_e16733 * locals.var_t8_dn10)), (((-locals.var_t1__blk809_dn11) * locals.var_t8) + (assign18600_e16733 * locals.var_t8_dn11)), (((-locals.var_t1__blk809_dn12) * locals.var_t8) + (assign18600_e16733 * locals.var_t8_dn12)),)
    } else {
        (locals.var_dabulk_dvg, locals.var_dabulk_dvg_dn3, locals.var_dabulk_dvg_dn4, locals.var_dabulk_dvg_dn5, locals.var_dabulk_dvg_dn6, locals.var_dabulk_dvg_dn7, locals.var_dabulk_dvg_dn8, locals.var_dabulk_dvg_dn9, locals.var_dabulk_dvg_dn10, locals.var_dabulk_dvg_dn11, locals.var_dabulk_dvg_dn12,)
    }
};
        locals.var_dabulk_dvg = assign18600_e16737;
        locals.var_dabulk_dvg_dn3 = assign18600_e16737_d_n3;
        locals.var_dabulk_dvg_dn4 = assign18600_e16737_d_n4;
        locals.var_dabulk_dvg_dn5 = assign18600_e16737_d_n5;
        locals.var_dabulk_dvg_dn6 = assign18600_e16737_d_n6;
        locals.var_dabulk_dvg_dn7 = assign18600_e16737_d_n7;
        locals.var_dabulk_dvg_dn8 = assign18600_e16737_d_n8;
        locals.var_dabulk_dvg_dn9 = assign18600_e16737_d_n9;
        locals.var_dabulk_dvg_dn10 = assign18600_e16737_d_n10;
        locals.var_dabulk_dvg_dn11 = assign18600_e16737_d_n11;
        locals.var_dabulk_dvg_dn12 = assign18600_e16737_d_n12;
        locals.var_dabulk_dvg_rv = 0.0;

        let (assign18610_e16746, assign18610_e16746_d_n3, assign18610_e16746_d_n4, assign18610_e16746_d_n5, assign18610_e16746_d_n6, assign18610_e16746_d_n7, assign18610_e16746_d_n8, assign18610_e16746_d_n9, assign18610_e16746_d_n10, assign18610_e16746_d_n11, assign18610_e16746_d_n12,) = {
    if (locals.var_guard1216 == 0.0) {
        let assign18610_e16743: f64 = (locals.var_dabulk_dvg * locals.var_vgsteff__blk840);
        let assign18610_e16744: f64 = (locals.var_abulk0 + assign18610_e16743);
        (assign18610_e16744, (locals.var_abulk0_dn3 + ((locals.var_dabulk_dvg_dn3 * locals.var_vgsteff__blk840) + (locals.var_dabulk_dvg * locals.var_vgsteff__blk840_dn3))), (locals.var_abulk0_dn4 + ((locals.var_dabulk_dvg_dn4 * locals.var_vgsteff__blk840) + (locals.var_dabulk_dvg * locals.var_vgsteff__blk840_dn4))), (locals.var_abulk0_dn5 + ((locals.var_dabulk_dvg_dn5 * locals.var_vgsteff__blk840) + (locals.var_dabulk_dvg * locals.var_vgsteff__blk840_dn5))), (locals.var_abulk0_dn6 + ((locals.var_dabulk_dvg_dn6 * locals.var_vgsteff__blk840) + (locals.var_dabulk_dvg * locals.var_vgsteff__blk840_dn6))), (locals.var_abulk0_dn7 + ((locals.var_dabulk_dvg_dn7 * locals.var_vgsteff__blk840) + (locals.var_dabulk_dvg * locals.var_vgsteff__blk840_dn7))), (locals.var_abulk0_dn8 + ((locals.var_dabulk_dvg_dn8 * locals.var_vgsteff__blk840) + (locals.var_dabulk_dvg * locals.var_vgsteff__blk840_dn8))), (locals.var_abulk0_dn9 + ((locals.var_dabulk_dvg_dn9 * locals.var_vgsteff__blk840) + (locals.var_dabulk_dvg * locals.var_vgsteff__blk840_dn9))), (locals.var_abulk0_dn10 + ((locals.var_dabulk_dvg_dn10 * locals.var_vgsteff__blk840) + (locals.var_dabulk_dvg * locals.var_vgsteff__blk840_dn10))), (locals.var_abulk0_dn11 + ((locals.var_dabulk_dvg_dn11 * locals.var_vgsteff__blk840) + (locals.var_dabulk_dvg * locals.var_vgsteff__blk840_dn11))), (locals.var_abulk0_dn12 + ((locals.var_dabulk_dvg_dn12 * locals.var_vgsteff__blk840) + (locals.var_dabulk_dvg * locals.var_vgsteff__blk840_dn12))),)
    } else {
        (locals.var_abulk, locals.var_abulk_dn3, locals.var_abulk_dn4, locals.var_abulk_dn5, locals.var_abulk_dn6, locals.var_abulk_dn7, locals.var_abulk_dn8, locals.var_abulk_dn9, locals.var_abulk_dn10, locals.var_abulk_dn11, locals.var_abulk_dn12,)
    }
};
        locals.var_abulk = assign18610_e16746;
        locals.var_abulk_dn3 = assign18610_e16746_d_n3;
        locals.var_abulk_dn4 = assign18610_e16746_d_n4;
        locals.var_abulk_dn5 = assign18610_e16746_d_n5;
        locals.var_abulk_dn6 = assign18610_e16746_d_n6;
        locals.var_abulk_dn7 = assign18610_e16746_d_n7;
        locals.var_abulk_dn8 = assign18610_e16746_d_n8;
        locals.var_abulk_dn9 = assign18610_e16746_d_n9;
        locals.var_abulk_dn10 = assign18610_e16746_d_n10;
        locals.var_abulk_dn11 = assign18610_e16746_d_n11;
        locals.var_abulk_dn12 = assign18610_e16746_d_n12;
        locals.var_abulk_rv = 0.0;

        let assign18620_e16749: f64 = if locals.var_abulk0 < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard1219 = assign18620_e16749;
        locals.var_guard1219_rv = 0.0;

        let (assign18630_e16759, assign18630_e16759_d_n3, assign18630_e16759_d_n4, assign18630_e16759_d_n5, assign18630_e16759_d_n6, assign18630_e16759_d_n7, assign18630_e16759_d_n8, assign18630_e16759_d_n9, assign18630_e16759_d_n10, assign18630_e16759_d_n11, assign18630_e16759_d_n12,) = {
    if (locals.var_guard1219 != 0.0) {
        let assign18630_e16755: f64 = (200.0 * locals.var_abulk0);
        let assign18630_e16756: f64 = (3.0 - assign18630_e16755);
        let assign18630_e16757: f64 = (1.0 / assign18630_e16756);
        (assign18630_e16757, (-((-(200.0 * locals.var_abulk0_dn3)) / (assign18630_e16756 * assign18630_e16756))), (-((-(200.0 * locals.var_abulk0_dn4)) / (assign18630_e16756 * assign18630_e16756))), (-((-(200.0 * locals.var_abulk0_dn5)) / (assign18630_e16756 * assign18630_e16756))), (-((-(200.0 * locals.var_abulk0_dn6)) / (assign18630_e16756 * assign18630_e16756))), (-((-(200.0 * locals.var_abulk0_dn7)) / (assign18630_e16756 * assign18630_e16756))), (-((-(200.0 * locals.var_abulk0_dn8)) / (assign18630_e16756 * assign18630_e16756))), (-((-(200.0 * locals.var_abulk0_dn9)) / (assign18630_e16756 * assign18630_e16756))), (-((-(200.0 * locals.var_abulk0_dn10)) / (assign18630_e16756 * assign18630_e16756))), (-((-(200.0 * locals.var_abulk0_dn11)) / (assign18630_e16756 * assign18630_e16756))), (-((-(200.0 * locals.var_abulk0_dn12)) / (assign18630_e16756 * assign18630_e16756))),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign18630_e16759;
        locals.var_t9_dn3 = assign18630_e16759_d_n3;
        locals.var_t9_dn4 = assign18630_e16759_d_n4;
        locals.var_t9_dn5 = assign18630_e16759_d_n5;
        locals.var_t9_dn6 = assign18630_e16759_d_n6;
        locals.var_t9_dn7 = assign18630_e16759_d_n7;
        locals.var_t9_dn8 = assign18630_e16759_d_n8;
        locals.var_t9_dn9 = assign18630_e16759_d_n9;
        locals.var_t9_dn10 = assign18630_e16759_d_n10;
        locals.var_t9_dn11 = assign18630_e16759_d_n11;
        locals.var_t9_dn12 = assign18630_e16759_d_n12;
        locals.var_t9_rv = 0.0;

        let (assign18640_e16767, assign18640_e16767_d_n3, assign18640_e16767_d_n4, assign18640_e16767_d_n5, assign18640_e16767_d_n6, assign18640_e16767_d_n7, assign18640_e16767_d_n8, assign18640_e16767_d_n9, assign18640_e16767_d_n10, assign18640_e16767_d_n11, assign18640_e16767_d_n12,) = {
    if (locals.var_guard1219 != 0.0) {
        let assign18640_e16763: f64 = (0.02 - locals.var_abulk0);
        let assign18640_e16765: f64 = (assign18640_e16763 * locals.var_t9);
        (assign18640_e16765, (((-locals.var_abulk0_dn3) * locals.var_t9) + (assign18640_e16763 * locals.var_t9_dn3)), (((-locals.var_abulk0_dn4) * locals.var_t9) + (assign18640_e16763 * locals.var_t9_dn4)), (((-locals.var_abulk0_dn5) * locals.var_t9) + (assign18640_e16763 * locals.var_t9_dn5)), (((-locals.var_abulk0_dn6) * locals.var_t9) + (assign18640_e16763 * locals.var_t9_dn6)), (((-locals.var_abulk0_dn7) * locals.var_t9) + (assign18640_e16763 * locals.var_t9_dn7)), (((-locals.var_abulk0_dn8) * locals.var_t9) + (assign18640_e16763 * locals.var_t9_dn8)), (((-locals.var_abulk0_dn9) * locals.var_t9) + (assign18640_e16763 * locals.var_t9_dn9)), (((-locals.var_abulk0_dn10) * locals.var_t9) + (assign18640_e16763 * locals.var_t9_dn10)), (((-locals.var_abulk0_dn11) * locals.var_t9) + (assign18640_e16763 * locals.var_t9_dn11)), (((-locals.var_abulk0_dn12) * locals.var_t9) + (assign18640_e16763 * locals.var_t9_dn12)),)
    } else {
        (locals.var_abulk0, locals.var_abulk0_dn3, locals.var_abulk0_dn4, locals.var_abulk0_dn5, locals.var_abulk0_dn6, locals.var_abulk0_dn7, locals.var_abulk0_dn8, locals.var_abulk0_dn9, locals.var_abulk0_dn10, locals.var_abulk0_dn11, locals.var_abulk0_dn12,)
    }
};
        locals.var_abulk0 = assign18640_e16767;
        locals.var_abulk0_dn3 = assign18640_e16767_d_n3;
        locals.var_abulk0_dn4 = assign18640_e16767_d_n4;
        locals.var_abulk0_dn5 = assign18640_e16767_d_n5;
        locals.var_abulk0_dn6 = assign18640_e16767_d_n6;
        locals.var_abulk0_dn7 = assign18640_e16767_d_n7;
        locals.var_abulk0_dn8 = assign18640_e16767_d_n8;
        locals.var_abulk0_dn9 = assign18640_e16767_d_n9;
        locals.var_abulk0_dn10 = assign18640_e16767_d_n10;
        locals.var_abulk0_dn11 = assign18640_e16767_d_n11;
        locals.var_abulk0_dn12 = assign18640_e16767_d_n12;
        locals.var_abulk0_rv = 0.0;

        let assign18650_e16770: f64 = if locals.var_abulk < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard1220 = assign18650_e16770;
        locals.var_guard1220_rv = 0.0;

        let (assign18660_e16780, assign18660_e16780_d_n3, assign18660_e16780_d_n4, assign18660_e16780_d_n5, assign18660_e16780_d_n6, assign18660_e16780_d_n7, assign18660_e16780_d_n8, assign18660_e16780_d_n9, assign18660_e16780_d_n10, assign18660_e16780_d_n11, assign18660_e16780_d_n12,) = {
    if (locals.var_guard1220 != 0.0) {
        let assign18660_e16776: f64 = (200.0 * locals.var_abulk);
        let assign18660_e16777: f64 = (3.0 - assign18660_e16776);
        let assign18660_e16778: f64 = (1.0 / assign18660_e16777);
        (assign18660_e16778, (-((-(200.0 * locals.var_abulk_dn3)) / (assign18660_e16777 * assign18660_e16777))), (-((-(200.0 * locals.var_abulk_dn4)) / (assign18660_e16777 * assign18660_e16777))), (-((-(200.0 * locals.var_abulk_dn5)) / (assign18660_e16777 * assign18660_e16777))), (-((-(200.0 * locals.var_abulk_dn6)) / (assign18660_e16777 * assign18660_e16777))), (-((-(200.0 * locals.var_abulk_dn7)) / (assign18660_e16777 * assign18660_e16777))), (-((-(200.0 * locals.var_abulk_dn8)) / (assign18660_e16777 * assign18660_e16777))), (-((-(200.0 * locals.var_abulk_dn9)) / (assign18660_e16777 * assign18660_e16777))), (-((-(200.0 * locals.var_abulk_dn10)) / (assign18660_e16777 * assign18660_e16777))), (-((-(200.0 * locals.var_abulk_dn11)) / (assign18660_e16777 * assign18660_e16777))), (-((-(200.0 * locals.var_abulk_dn12)) / (assign18660_e16777 * assign18660_e16777))),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign18660_e16780;
        locals.var_t9_dn3 = assign18660_e16780_d_n3;
        locals.var_t9_dn4 = assign18660_e16780_d_n4;
        locals.var_t9_dn5 = assign18660_e16780_d_n5;
        locals.var_t9_dn6 = assign18660_e16780_d_n6;
        locals.var_t9_dn7 = assign18660_e16780_d_n7;
        locals.var_t9_dn8 = assign18660_e16780_d_n8;
        locals.var_t9_dn9 = assign18660_e16780_d_n9;
        locals.var_t9_dn10 = assign18660_e16780_d_n10;
        locals.var_t9_dn11 = assign18660_e16780_d_n11;
        locals.var_t9_dn12 = assign18660_e16780_d_n12;
        locals.var_t9_rv = 0.0;

        let (assign18670_e16788, assign18670_e16788_d_n3, assign18670_e16788_d_n4, assign18670_e16788_d_n5, assign18670_e16788_d_n6, assign18670_e16788_d_n7, assign18670_e16788_d_n8, assign18670_e16788_d_n9, assign18670_e16788_d_n10, assign18670_e16788_d_n11, assign18670_e16788_d_n12,) = {
    if (locals.var_guard1220 != 0.0) {
        let assign18670_e16784: f64 = (0.02 - locals.var_abulk);
        let assign18670_e16786: f64 = (assign18670_e16784 * locals.var_t9);
        (assign18670_e16786, (((-locals.var_abulk_dn3) * locals.var_t9) + (assign18670_e16784 * locals.var_t9_dn3)), (((-locals.var_abulk_dn4) * locals.var_t9) + (assign18670_e16784 * locals.var_t9_dn4)), (((-locals.var_abulk_dn5) * locals.var_t9) + (assign18670_e16784 * locals.var_t9_dn5)), (((-locals.var_abulk_dn6) * locals.var_t9) + (assign18670_e16784 * locals.var_t9_dn6)), (((-locals.var_abulk_dn7) * locals.var_t9) + (assign18670_e16784 * locals.var_t9_dn7)), (((-locals.var_abulk_dn8) * locals.var_t9) + (assign18670_e16784 * locals.var_t9_dn8)), (((-locals.var_abulk_dn9) * locals.var_t9) + (assign18670_e16784 * locals.var_t9_dn9)), (((-locals.var_abulk_dn10) * locals.var_t9) + (assign18670_e16784 * locals.var_t9_dn10)), (((-locals.var_abulk_dn11) * locals.var_t9) + (assign18670_e16784 * locals.var_t9_dn11)), (((-locals.var_abulk_dn12) * locals.var_t9) + (assign18670_e16784 * locals.var_t9_dn12)),)
    } else {
        (locals.var_abulk, locals.var_abulk_dn3, locals.var_abulk_dn4, locals.var_abulk_dn5, locals.var_abulk_dn6, locals.var_abulk_dn7, locals.var_abulk_dn8, locals.var_abulk_dn9, locals.var_abulk_dn10, locals.var_abulk_dn11, locals.var_abulk_dn12,)
    }
};
        locals.var_abulk = assign18670_e16788;
        locals.var_abulk_dn3 = assign18670_e16788_d_n3;
        locals.var_abulk_dn4 = assign18670_e16788_d_n4;
        locals.var_abulk_dn5 = assign18670_e16788_d_n5;
        locals.var_abulk_dn6 = assign18670_e16788_d_n6;
        locals.var_abulk_dn7 = assign18670_e16788_d_n7;
        locals.var_abulk_dn8 = assign18670_e16788_d_n8;
        locals.var_abulk_dn9 = assign18670_e16788_d_n9;
        locals.var_abulk_dn10 = assign18670_e16788_d_n10;
        locals.var_abulk_dn11 = assign18670_e16788_d_n11;
        locals.var_abulk_dn12 = assign18670_e16788_d_n12;
        locals.var_abulk_rv = 0.0;

        locals.var_b4soiabulk = locals.var_abulk;
        locals.var_b4soiabulk_dn3 = locals.var_abulk_dn3;
        locals.var_b4soiabulk_dn4 = locals.var_abulk_dn4;
        locals.var_b4soiabulk_dn5 = locals.var_abulk_dn5;
        locals.var_b4soiabulk_dn6 = locals.var_abulk_dn6;
        locals.var_b4soiabulk_dn7 = locals.var_abulk_dn7;
        locals.var_b4soiabulk_dn8 = locals.var_abulk_dn8;
        locals.var_b4soiabulk_dn9 = locals.var_abulk_dn9;
        locals.var_b4soiabulk_dn10 = locals.var_abulk_dn10;
        locals.var_b4soiabulk_dn11 = locals.var_abulk_dn11;
        locals.var_b4soiabulk_dn12 = locals.var_abulk_dn12;
        locals.var_b4soiabulk_rv = 0.0;

        let assign18690_e16792: f64 = if locals.var_pparam_b4soia0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1221 = assign18690_e16792;
        locals.var_guard1221_rv = 0.0;

        let (assign18700_e16796, assign18700_e16796_d_n3, assign18700_e16796_d_n4, assign18700_e16796_d_n5, assign18700_e16796_d_n6, assign18700_e16796_d_n7, assign18700_e16796_d_n8, assign18700_e16796_d_n9, assign18700_e16796_d_n10, assign18700_e16796_d_n11, assign18700_e16796_d_n12,) = {
    if (locals.var_guard1221 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_abulk0_cv, locals.var_abulk0_cv_dn3, locals.var_abulk0_cv_dn4, locals.var_abulk0_cv_dn5, locals.var_abulk0_cv_dn6, locals.var_abulk0_cv_dn7, locals.var_abulk0_cv_dn8, locals.var_abulk0_cv_dn9, locals.var_abulk0_cv_dn10, locals.var_abulk0_cv_dn11, locals.var_abulk0_cv_dn12,)
    }
};
        locals.var_abulk0_cv = assign18700_e16796;
        locals.var_abulk0_cv_dn3 = assign18700_e16796_d_n3;
        locals.var_abulk0_cv_dn4 = assign18700_e16796_d_n4;
        locals.var_abulk0_cv_dn5 = assign18700_e16796_d_n5;
        locals.var_abulk0_cv_dn6 = assign18700_e16796_d_n6;
        locals.var_abulk0_cv_dn7 = assign18700_e16796_d_n7;
        locals.var_abulk0_cv_dn8 = assign18700_e16796_d_n8;
        locals.var_abulk0_cv_dn9 = assign18700_e16796_d_n9;
        locals.var_abulk0_cv_dn10 = assign18700_e16796_d_n10;
        locals.var_abulk0_cv_dn11 = assign18700_e16796_d_n11;
        locals.var_abulk0_cv_dn12 = assign18700_e16796_d_n12;
        locals.var_abulk0_cv_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_56(
        locals: &mut StampLocals,
    ) {
        let (assign18710_e16803, assign18710_e16803_d_n3, assign18710_e16803_d_n4, assign18710_e16803_d_n5, assign18710_e16803_d_n6, assign18710_e16803_d_n7, assign18710_e16803_d_n8, assign18710_e16803_d_n9, assign18710_e16803_d_n10, assign18710_e16803_d_n11, assign18710_e16803_d_n12,) = {
    if (locals.var_guard1221 == 0.0) {
        let assign18710_e16801: f64 = (locals.var_pparam_b4soiketa * locals.var_vbsh_cv);
        (assign18710_e16801, ((locals.var_pparam_b4soiketa_dn3 * locals.var_vbsh_cv) + (locals.var_pparam_b4soiketa * locals.var_vbsh_cv_dn3)), ((locals.var_pparam_b4soiketa_dn4 * locals.var_vbsh_cv) + (locals.var_pparam_b4soiketa * locals.var_vbsh_cv_dn4)), ((locals.var_pparam_b4soiketa_dn5 * locals.var_vbsh_cv) + (locals.var_pparam_b4soiketa * locals.var_vbsh_cv_dn5)), ((locals.var_pparam_b4soiketa_dn6 * locals.var_vbsh_cv) + (locals.var_pparam_b4soiketa * locals.var_vbsh_cv_dn6)), ((locals.var_pparam_b4soiketa_dn7 * locals.var_vbsh_cv) + (locals.var_pparam_b4soiketa * locals.var_vbsh_cv_dn7)), ((locals.var_pparam_b4soiketa_dn8 * locals.var_vbsh_cv) + (locals.var_pparam_b4soiketa * locals.var_vbsh_cv_dn8)), ((locals.var_pparam_b4soiketa_dn9 * locals.var_vbsh_cv) + (locals.var_pparam_b4soiketa * locals.var_vbsh_cv_dn9)), ((locals.var_pparam_b4soiketa_dn10 * locals.var_vbsh_cv) + (locals.var_pparam_b4soiketa * locals.var_vbsh_cv_dn10)), ((locals.var_pparam_b4soiketa_dn11 * locals.var_vbsh_cv) + (locals.var_pparam_b4soiketa * locals.var_vbsh_cv_dn11)), ((locals.var_pparam_b4soiketa_dn12 * locals.var_vbsh_cv) + (locals.var_pparam_b4soiketa * locals.var_vbsh_cv_dn12)),)
    } else {
        (locals.var_t10__blk818, locals.var_t10__blk818_dn3, locals.var_t10__blk818_dn4, locals.var_t10__blk818_dn5, locals.var_t10__blk818_dn6, locals.var_t10__blk818_dn7, locals.var_t10__blk818_dn8, locals.var_t10__blk818_dn9, locals.var_t10__blk818_dn10, locals.var_t10__blk818_dn11, locals.var_t10__blk818_dn12,)
    }
};
        locals.var_t10__blk818 = assign18710_e16803;
        locals.var_t10__blk818_dn3 = assign18710_e16803_d_n3;
        locals.var_t10__blk818_dn4 = assign18710_e16803_d_n4;
        locals.var_t10__blk818_dn5 = assign18710_e16803_d_n5;
        locals.var_t10__blk818_dn6 = assign18710_e16803_d_n6;
        locals.var_t10__blk818_dn7 = assign18710_e16803_d_n7;
        locals.var_t10__blk818_dn8 = assign18710_e16803_d_n8;
        locals.var_t10__blk818_dn9 = assign18710_e16803_d_n9;
        locals.var_t10__blk818_dn10 = assign18710_e16803_d_n10;
        locals.var_t10__blk818_dn11 = assign18710_e16803_d_n11;
        locals.var_t10__blk818_dn12 = assign18710_e16803_d_n12;
        locals.var_t10__blk818_rv = 0.0;

        let assign18720_e16806: f64 = (-0.5);
        let assign18720_e16807: f64 = if locals.var_t10__blk818 >= assign18720_e16806 { 1.0 } else { 0.0 };
        locals.var_guard1222 = assign18720_e16807;
        locals.var_guard1222_rv = 0.0;

        let (assign18730_e16818, assign18730_e16818_d_n3, assign18730_e16818_d_n4, assign18730_e16818_d_n5, assign18730_e16818_d_n6, assign18730_e16818_d_n7, assign18730_e16818_d_n8, assign18730_e16818_d_n9, assign18730_e16818_d_n10, assign18730_e16818_d_n11, assign18730_e16818_d_n12,) = {
    if ((locals.var_guard1221 == 0.0) && (locals.var_guard1222 != 0.0)) {
        let assign18730_e16815: f64 = (1.0 + locals.var_t10__blk818);
        let assign18730_e16816: f64 = (1.0 / assign18730_e16815);
        (assign18730_e16816, (-(locals.var_t10__blk818_dn3 / (assign18730_e16815 * assign18730_e16815))), (-(locals.var_t10__blk818_dn4 / (assign18730_e16815 * assign18730_e16815))), (-(locals.var_t10__blk818_dn5 / (assign18730_e16815 * assign18730_e16815))), (-(locals.var_t10__blk818_dn6 / (assign18730_e16815 * assign18730_e16815))), (-(locals.var_t10__blk818_dn7 / (assign18730_e16815 * assign18730_e16815))), (-(locals.var_t10__blk818_dn8 / (assign18730_e16815 * assign18730_e16815))), (-(locals.var_t10__blk818_dn9 / (assign18730_e16815 * assign18730_e16815))), (-(locals.var_t10__blk818_dn10 / (assign18730_e16815 * assign18730_e16815))), (-(locals.var_t10__blk818_dn11 / (assign18730_e16815 * assign18730_e16815))), (-(locals.var_t10__blk818_dn12 / (assign18730_e16815 * assign18730_e16815))),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12,)
    }
};
        locals.var_t11 = assign18730_e16818;
        locals.var_t11_dn3 = assign18730_e16818_d_n3;
        locals.var_t11_dn4 = assign18730_e16818_d_n4;
        locals.var_t11_dn5 = assign18730_e16818_d_n5;
        locals.var_t11_dn6 = assign18730_e16818_d_n6;
        locals.var_t11_dn7 = assign18730_e16818_d_n7;
        locals.var_t11_dn8 = assign18730_e16818_d_n8;
        locals.var_t11_dn9 = assign18730_e16818_d_n9;
        locals.var_t11_dn10 = assign18730_e16818_d_n10;
        locals.var_t11_dn11 = assign18730_e16818_d_n11;
        locals.var_t11_dn12 = assign18730_e16818_d_n12;
        locals.var_t11_rv = 0.0;

        let (assign18740_e16835, assign18740_e16835_d_n3, assign18740_e16835_d_n4, assign18740_e16835_d_n5, assign18740_e16835_d_n6, assign18740_e16835_d_n7, assign18740_e16835_d_n8, assign18740_e16835_d_n9, assign18740_e16835_d_n10, assign18740_e16835_d_n11, assign18740_e16835_d_n12,) = {
    if ((locals.var_guard1221 == 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign18740_e16825: f64 = (-1.0);
        let assign18740_e16828: f64 = (1.0 - 0.5);
        let assign18740_e16831: f64 = (1.0 - 0.5);
        let assign18740_e16832: f64 = (assign18740_e16828 * assign18740_e16831);
        let assign18740_e16833: f64 = (assign18740_e16825 / assign18740_e16832);
        (assign18740_e16833, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12,)
    }
};
        locals.var_t12 = assign18740_e16835;
        locals.var_t12_dn3 = assign18740_e16835_d_n3;
        locals.var_t12_dn4 = assign18740_e16835_d_n4;
        locals.var_t12_dn5 = assign18740_e16835_d_n5;
        locals.var_t12_dn6 = assign18740_e16835_d_n6;
        locals.var_t12_dn7 = assign18740_e16835_d_n7;
        locals.var_t12_dn8 = assign18740_e16835_d_n8;
        locals.var_t12_dn9 = assign18740_e16835_d_n9;
        locals.var_t12_dn10 = assign18740_e16835_d_n10;
        locals.var_t12_dn11 = assign18740_e16835_d_n11;
        locals.var_t12_dn12 = assign18740_e16835_d_n12;
        locals.var_t12_rv = 0.0;

        let (assign18750_e16851, assign18750_e16851_d_n3, assign18750_e16851_d_n4, assign18750_e16851_d_n5, assign18750_e16851_d_n6, assign18750_e16851_d_n7, assign18750_e16851_d_n8, assign18750_e16851_d_n9, assign18750_e16851_d_n10, assign18750_e16851_d_n11, assign18750_e16851_d_n12,) = {
    if ((locals.var_guard1221 == 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign18750_e16844: f64 = (1.0 - 0.5);
        let assign18750_e16845: f64 = (1.0 / assign18750_e16844);
        let assign18750_e16848: f64 = (locals.var_t12 * 0.5);
        let assign18750_e16849: f64 = (assign18750_e16845 + assign18750_e16848);
        (assign18750_e16849, (locals.var_t12_dn3 * 0.5), (locals.var_t12_dn4 * 0.5), (locals.var_t12_dn5 * 0.5), (locals.var_t12_dn6 * 0.5), (locals.var_t12_dn7 * 0.5), (locals.var_t12_dn8 * 0.5), (locals.var_t12_dn9 * 0.5), (locals.var_t12_dn10 * 0.5), (locals.var_t12_dn11 * 0.5), (locals.var_t12_dn12 * 0.5),)
    } else {
        (locals.var_t13, locals.var_t13_dn3, locals.var_t13_dn4, locals.var_t13_dn5, locals.var_t13_dn6, locals.var_t13_dn7, locals.var_t13_dn8, locals.var_t13_dn9, locals.var_t13_dn10, locals.var_t13_dn11, locals.var_t13_dn12,)
    }
};
        locals.var_t13 = assign18750_e16851;
        locals.var_t13_dn3 = assign18750_e16851_d_n3;
        locals.var_t13_dn4 = assign18750_e16851_d_n4;
        locals.var_t13_dn5 = assign18750_e16851_d_n5;
        locals.var_t13_dn6 = assign18750_e16851_d_n6;
        locals.var_t13_dn7 = assign18750_e16851_d_n7;
        locals.var_t13_dn8 = assign18750_e16851_d_n8;
        locals.var_t13_dn9 = assign18750_e16851_d_n9;
        locals.var_t13_dn10 = assign18750_e16851_d_n10;
        locals.var_t13_dn11 = assign18750_e16851_d_n11;
        locals.var_t13_dn12 = assign18750_e16851_d_n12;
        locals.var_t13_rv = 0.0;

        let (assign18760_e16863, assign18760_e16863_d_n3, assign18760_e16863_d_n4, assign18760_e16863_d_n5, assign18760_e16863_d_n6, assign18760_e16863_d_n7, assign18760_e16863_d_n8, assign18760_e16863_d_n9, assign18760_e16863_d_n10, assign18760_e16863_d_n11, assign18760_e16863_d_n12,) = {
    if ((locals.var_guard1221 == 0.0) && (locals.var_guard1222 == 0.0)) {
        let assign18760_e16859: f64 = (locals.var_t12 * locals.var_t10__blk818);
        let assign18760_e16861: f64 = (assign18760_e16859 + locals.var_t13);
        (assign18760_e16861, (((locals.var_t12_dn3 * locals.var_t10__blk818) + (locals.var_t12 * locals.var_t10__blk818_dn3)) + locals.var_t13_dn3), (((locals.var_t12_dn4 * locals.var_t10__blk818) + (locals.var_t12 * locals.var_t10__blk818_dn4)) + locals.var_t13_dn4), (((locals.var_t12_dn5 * locals.var_t10__blk818) + (locals.var_t12 * locals.var_t10__blk818_dn5)) + locals.var_t13_dn5), (((locals.var_t12_dn6 * locals.var_t10__blk818) + (locals.var_t12 * locals.var_t10__blk818_dn6)) + locals.var_t13_dn6), (((locals.var_t12_dn7 * locals.var_t10__blk818) + (locals.var_t12 * locals.var_t10__blk818_dn7)) + locals.var_t13_dn7), (((locals.var_t12_dn8 * locals.var_t10__blk818) + (locals.var_t12 * locals.var_t10__blk818_dn8)) + locals.var_t13_dn8), (((locals.var_t12_dn9 * locals.var_t10__blk818) + (locals.var_t12 * locals.var_t10__blk818_dn9)) + locals.var_t13_dn9), (((locals.var_t12_dn10 * locals.var_t10__blk818) + (locals.var_t12 * locals.var_t10__blk818_dn10)) + locals.var_t13_dn10), (((locals.var_t12_dn11 * locals.var_t10__blk818) + (locals.var_t12 * locals.var_t10__blk818_dn11)) + locals.var_t13_dn11), (((locals.var_t12_dn12 * locals.var_t10__blk818) + (locals.var_t12 * locals.var_t10__blk818_dn12)) + locals.var_t13_dn12),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12,)
    }
};
        locals.var_t11 = assign18760_e16863;
        locals.var_t11_dn3 = assign18760_e16863_d_n3;
        locals.var_t11_dn4 = assign18760_e16863_d_n4;
        locals.var_t11_dn5 = assign18760_e16863_d_n5;
        locals.var_t11_dn6 = assign18760_e16863_d_n6;
        locals.var_t11_dn7 = assign18760_e16863_d_n7;
        locals.var_t11_dn8 = assign18760_e16863_d_n8;
        locals.var_t11_dn9 = assign18760_e16863_d_n9;
        locals.var_t11_dn10 = assign18760_e16863_d_n10;
        locals.var_t11_dn11 = assign18760_e16863_d_n11;
        locals.var_t11_dn12 = assign18760_e16863_d_n12;
        locals.var_t11_rv = 0.0;

        let (assign18770_e16870, assign18770_e16870_d_n3, assign18770_e16870_d_n4, assign18770_e16870_d_n5, assign18770_e16870_d_n6, assign18770_e16870_d_n7, assign18770_e16870_d_n8, assign18770_e16870_d_n9, assign18770_e16870_d_n10, assign18770_e16870_d_n11, assign18770_e16870_d_n12,) = {
    if (locals.var_guard1221 == 0.0) {
        let assign18770_e16868: f64 = (locals.var_phi + locals.var_pparam_b4soiketas);
        (assign18770_e16868, (locals.var_phi_dn3 + locals.var_pparam_b4soiketas_dn3), (locals.var_phi_dn4 + locals.var_pparam_b4soiketas_dn4), (locals.var_phi_dn5 + locals.var_pparam_b4soiketas_dn5), (locals.var_phi_dn6 + locals.var_pparam_b4soiketas_dn6), (locals.var_phi_dn7 + locals.var_pparam_b4soiketas_dn7), (locals.var_phi_dn8 + locals.var_pparam_b4soiketas_dn8), (locals.var_phi_dn9 + locals.var_pparam_b4soiketas_dn9), (locals.var_phi_dn10 + locals.var_pparam_b4soiketas_dn10), (locals.var_phi_dn11 + locals.var_pparam_b4soiketas_dn11), (locals.var_phi_dn12 + locals.var_pparam_b4soiketas_dn12),)
    } else {
        (locals.var_t10__blk818, locals.var_t10__blk818_dn3, locals.var_t10__blk818_dn4, locals.var_t10__blk818_dn5, locals.var_t10__blk818_dn6, locals.var_t10__blk818_dn7, locals.var_t10__blk818_dn8, locals.var_t10__blk818_dn9, locals.var_t10__blk818_dn10, locals.var_t10__blk818_dn11, locals.var_t10__blk818_dn12,)
    }
};
        locals.var_t10__blk818 = assign18770_e16870;
        locals.var_t10__blk818_dn3 = assign18770_e16870_d_n3;
        locals.var_t10__blk818_dn4 = assign18770_e16870_d_n4;
        locals.var_t10__blk818_dn5 = assign18770_e16870_d_n5;
        locals.var_t10__blk818_dn6 = assign18770_e16870_d_n6;
        locals.var_t10__blk818_dn7 = assign18770_e16870_d_n7;
        locals.var_t10__blk818_dn8 = assign18770_e16870_d_n8;
        locals.var_t10__blk818_dn9 = assign18770_e16870_d_n9;
        locals.var_t10__blk818_dn10 = assign18770_e16870_d_n10;
        locals.var_t10__blk818_dn11 = assign18770_e16870_d_n11;
        locals.var_t10__blk818_dn12 = assign18770_e16870_d_n12;
        locals.var_t10__blk818_rv = 0.0;

        let (assign18780_e16879, assign18780_e16879_d_n3, assign18780_e16879_d_n4, assign18780_e16879_d_n5, assign18780_e16879_d_n6, assign18780_e16879_d_n7, assign18780_e16879_d_n8, assign18780_e16879_d_n9, assign18780_e16879_d_n10, assign18780_e16879_d_n11, assign18780_e16879_d_n12,) = {
    if (locals.var_guard1221 == 0.0) {
        let assign18780_e16875: f64 = (locals.var_vbsh_cv * locals.var_t11);
        let assign18780_e16877: f64 = (assign18780_e16875 / locals.var_t10__blk818);
        (assign18780_e16877, (((((locals.var_vbsh_cv_dn3 * locals.var_t11) + (locals.var_vbsh_cv * locals.var_t11_dn3)) * locals.var_t10__blk818) - (assign18780_e16875 * locals.var_t10__blk818_dn3)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((((locals.var_vbsh_cv_dn4 * locals.var_t11) + (locals.var_vbsh_cv * locals.var_t11_dn4)) * locals.var_t10__blk818) - (assign18780_e16875 * locals.var_t10__blk818_dn4)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((((locals.var_vbsh_cv_dn5 * locals.var_t11) + (locals.var_vbsh_cv * locals.var_t11_dn5)) * locals.var_t10__blk818) - (assign18780_e16875 * locals.var_t10__blk818_dn5)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((((locals.var_vbsh_cv_dn6 * locals.var_t11) + (locals.var_vbsh_cv * locals.var_t11_dn6)) * locals.var_t10__blk818) - (assign18780_e16875 * locals.var_t10__blk818_dn6)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((((locals.var_vbsh_cv_dn7 * locals.var_t11) + (locals.var_vbsh_cv * locals.var_t11_dn7)) * locals.var_t10__blk818) - (assign18780_e16875 * locals.var_t10__blk818_dn7)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((((locals.var_vbsh_cv_dn8 * locals.var_t11) + (locals.var_vbsh_cv * locals.var_t11_dn8)) * locals.var_t10__blk818) - (assign18780_e16875 * locals.var_t10__blk818_dn8)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((((locals.var_vbsh_cv_dn9 * locals.var_t11) + (locals.var_vbsh_cv * locals.var_t11_dn9)) * locals.var_t10__blk818) - (assign18780_e16875 * locals.var_t10__blk818_dn9)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((((locals.var_vbsh_cv_dn10 * locals.var_t11) + (locals.var_vbsh_cv * locals.var_t11_dn10)) * locals.var_t10__blk818) - (assign18780_e16875 * locals.var_t10__blk818_dn10)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((((locals.var_vbsh_cv_dn11 * locals.var_t11) + (locals.var_vbsh_cv * locals.var_t11_dn11)) * locals.var_t10__blk818) - (assign18780_e16875 * locals.var_t10__blk818_dn11)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((((locals.var_vbsh_cv_dn12 * locals.var_t11) + (locals.var_vbsh_cv * locals.var_t11_dn12)) * locals.var_t10__blk818) - (assign18780_e16875 * locals.var_t10__blk818_dn12)) / (locals.var_t10__blk818 * locals.var_t10__blk818)),)
    } else {
        (locals.var_t13, locals.var_t13_dn3, locals.var_t13_dn4, locals.var_t13_dn5, locals.var_t13_dn6, locals.var_t13_dn7, locals.var_t13_dn8, locals.var_t13_dn9, locals.var_t13_dn10, locals.var_t13_dn11, locals.var_t13_dn12,)
    }
};
        locals.var_t13 = assign18780_e16879;
        locals.var_t13_dn3 = assign18780_e16879_d_n3;
        locals.var_t13_dn4 = assign18780_e16879_d_n4;
        locals.var_t13_dn5 = assign18780_e16879_d_n5;
        locals.var_t13_dn6 = assign18780_e16879_d_n6;
        locals.var_t13_dn7 = assign18780_e16879_d_n7;
        locals.var_t13_dn8 = assign18780_e16879_d_n8;
        locals.var_t13_dn9 = assign18780_e16879_d_n9;
        locals.var_t13_dn10 = assign18780_e16879_d_n10;
        locals.var_t13_dn11 = assign18780_e16879_d_n11;
        locals.var_t13_dn12 = assign18780_e16879_d_n12;
        locals.var_t13_rv = 0.0;

        let assign18790_e16882: f64 = if locals.var_t13 < 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1223 = assign18790_e16882;
        locals.var_guard1223_rv = 0.0;

        let (assign18800_e16894, assign18800_e16894_d_n3, assign18800_e16894_d_n4, assign18800_e16894_d_n5, assign18800_e16894_d_n6, assign18800_e16894_d_n7, assign18800_e16894_d_n8, assign18800_e16894_d_n9, assign18800_e16894_d_n10, assign18800_e16894_d_n11, assign18800_e16894_d_n12,) = {
    if ((locals.var_guard1221 == 0.0) && (locals.var_guard1223 != 0.0)) {
        let assign18800_e16890: f64 = (1.0 - locals.var_t13);
        let assign18800_e16891: f64 = (assign18800_e16890).sqrt();
        let assign18800_e16892: f64 = (1.0 / assign18800_e16891);
        (assign18800_e16892, (-(((-locals.var_t13_dn3) / (2.0 * assign18800_e16891)) / (assign18800_e16891 * assign18800_e16891))), (-(((-locals.var_t13_dn4) / (2.0 * assign18800_e16891)) / (assign18800_e16891 * assign18800_e16891))), (-(((-locals.var_t13_dn5) / (2.0 * assign18800_e16891)) / (assign18800_e16891 * assign18800_e16891))), (-(((-locals.var_t13_dn6) / (2.0 * assign18800_e16891)) / (assign18800_e16891 * assign18800_e16891))), (-(((-locals.var_t13_dn7) / (2.0 * assign18800_e16891)) / (assign18800_e16891 * assign18800_e16891))), (-(((-locals.var_t13_dn8) / (2.0 * assign18800_e16891)) / (assign18800_e16891 * assign18800_e16891))), (-(((-locals.var_t13_dn9) / (2.0 * assign18800_e16891)) / (assign18800_e16891 * assign18800_e16891))), (-(((-locals.var_t13_dn10) / (2.0 * assign18800_e16891)) / (assign18800_e16891 * assign18800_e16891))), (-(((-locals.var_t13_dn11) / (2.0 * assign18800_e16891)) / (assign18800_e16891 * assign18800_e16891))), (-(((-locals.var_t13_dn12) / (2.0 * assign18800_e16891)) / (assign18800_e16891 * assign18800_e16891))),)
    } else {
        (locals.var_t14, locals.var_t14_dn3, locals.var_t14_dn4, locals.var_t14_dn5, locals.var_t14_dn6, locals.var_t14_dn7, locals.var_t14_dn8, locals.var_t14_dn9, locals.var_t14_dn10, locals.var_t14_dn11, locals.var_t14_dn12,)
    }
};
        locals.var_t14 = assign18800_e16894;
        locals.var_t14_dn3 = assign18800_e16894_d_n3;
        locals.var_t14_dn4 = assign18800_e16894_d_n4;
        locals.var_t14_dn5 = assign18800_e16894_d_n5;
        locals.var_t14_dn6 = assign18800_e16894_d_n6;
        locals.var_t14_dn7 = assign18800_e16894_d_n7;
        locals.var_t14_dn8 = assign18800_e16894_d_n8;
        locals.var_t14_dn9 = assign18800_e16894_d_n9;
        locals.var_t14_dn10 = assign18800_e16894_d_n10;
        locals.var_t14_dn11 = assign18800_e16894_d_n11;
        locals.var_t14_dn12 = assign18800_e16894_d_n12;
        locals.var_t14_rv = 0.0;

        let (assign18810_e16913, assign18810_e16913_d_n3, assign18810_e16913_d_n4, assign18810_e16913_d_n5, assign18810_e16913_d_n6, assign18810_e16913_d_n7, assign18810_e16913_d_n8, assign18810_e16913_d_n9, assign18810_e16913_d_n10, assign18810_e16913_d_n11, assign18810_e16913_d_n12,) = {
    if ((locals.var_guard1221 == 0.0) && (locals.var_guard1223 == 0.0)) {
        let assign18810_e16904: f64 = (1.0 - 0.5);
        let assign18810_e16905: f64 = (2.0 * assign18810_e16904);
        let assign18810_e16908: f64 = (1.0 - 0.5);
        let assign18810_e16909: f64 = (assign18810_e16908).sqrt();
        let assign18810_e16910: f64 = (assign18810_e16905 * assign18810_e16909);
        let assign18810_e16911: f64 = (1.0 / assign18810_e16910);
        (assign18810_e16911, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12,)
    }
};
        locals.var_t11 = assign18810_e16913;
        locals.var_t11_dn3 = assign18810_e16913_d_n3;
        locals.var_t11_dn4 = assign18810_e16913_d_n4;
        locals.var_t11_dn5 = assign18810_e16913_d_n5;
        locals.var_t11_dn6 = assign18810_e16913_d_n6;
        locals.var_t11_dn7 = assign18810_e16913_d_n7;
        locals.var_t11_dn8 = assign18810_e16913_d_n8;
        locals.var_t11_dn9 = assign18810_e16913_d_n9;
        locals.var_t11_dn10 = assign18810_e16913_d_n10;
        locals.var_t11_dn11 = assign18810_e16913_d_n11;
        locals.var_t11_dn12 = assign18810_e16913_d_n12;
        locals.var_t11_rv = 0.0;

        let (assign18820_e16930, assign18820_e16930_d_n3, assign18820_e16930_d_n4, assign18820_e16930_d_n5, assign18820_e16930_d_n6, assign18820_e16930_d_n7, assign18820_e16930_d_n8, assign18820_e16930_d_n9, assign18820_e16930_d_n10, assign18820_e16930_d_n11, assign18820_e16930_d_n12,) = {
    if ((locals.var_guard1221 == 0.0) && (locals.var_guard1223 == 0.0)) {
        let assign18820_e16922: f64 = (1.0 - 0.5);
        let assign18820_e16923: f64 = (assign18820_e16922).sqrt();
        let assign18820_e16924: f64 = (1.0 / assign18820_e16923);
        let assign18820_e16927: f64 = (locals.var_t11 * 0.5);
        let assign18820_e16928: f64 = (assign18820_e16924 - assign18820_e16927);
        (assign18820_e16928, (-(locals.var_t11_dn3 * 0.5)), (-(locals.var_t11_dn4 * 0.5)), (-(locals.var_t11_dn5 * 0.5)), (-(locals.var_t11_dn6 * 0.5)), (-(locals.var_t11_dn7 * 0.5)), (-(locals.var_t11_dn8 * 0.5)), (-(locals.var_t11_dn9 * 0.5)), (-(locals.var_t11_dn10 * 0.5)), (-(locals.var_t11_dn11 * 0.5)), (-(locals.var_t11_dn12 * 0.5)),)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12,)
    }
};
        locals.var_t12 = assign18820_e16930;
        locals.var_t12_dn3 = assign18820_e16930_d_n3;
        locals.var_t12_dn4 = assign18820_e16930_d_n4;
        locals.var_t12_dn5 = assign18820_e16930_d_n5;
        locals.var_t12_dn6 = assign18820_e16930_d_n6;
        locals.var_t12_dn7 = assign18820_e16930_d_n7;
        locals.var_t12_dn8 = assign18820_e16930_d_n8;
        locals.var_t12_dn9 = assign18820_e16930_d_n9;
        locals.var_t12_dn10 = assign18820_e16930_d_n10;
        locals.var_t12_dn11 = assign18820_e16930_d_n11;
        locals.var_t12_dn12 = assign18820_e16930_d_n12;
        locals.var_t12_rv = 0.0;

        let (assign18830_e16942, assign18830_e16942_d_n3, assign18830_e16942_d_n4, assign18830_e16942_d_n5, assign18830_e16942_d_n6, assign18830_e16942_d_n7, assign18830_e16942_d_n8, assign18830_e16942_d_n9, assign18830_e16942_d_n10, assign18830_e16942_d_n11, assign18830_e16942_d_n12,) = {
    if ((locals.var_guard1221 == 0.0) && (locals.var_guard1223 == 0.0)) {
        let assign18830_e16938: f64 = (locals.var_t11 * locals.var_t13);
        let assign18830_e16940: f64 = (assign18830_e16938 + locals.var_t12);
        (assign18830_e16940, (((locals.var_t11_dn3 * locals.var_t13) + (locals.var_t11 * locals.var_t13_dn3)) + locals.var_t12_dn3), (((locals.var_t11_dn4 * locals.var_t13) + (locals.var_t11 * locals.var_t13_dn4)) + locals.var_t12_dn4), (((locals.var_t11_dn5 * locals.var_t13) + (locals.var_t11 * locals.var_t13_dn5)) + locals.var_t12_dn5), (((locals.var_t11_dn6 * locals.var_t13) + (locals.var_t11 * locals.var_t13_dn6)) + locals.var_t12_dn6), (((locals.var_t11_dn7 * locals.var_t13) + (locals.var_t11 * locals.var_t13_dn7)) + locals.var_t12_dn7), (((locals.var_t11_dn8 * locals.var_t13) + (locals.var_t11 * locals.var_t13_dn8)) + locals.var_t12_dn8), (((locals.var_t11_dn9 * locals.var_t13) + (locals.var_t11 * locals.var_t13_dn9)) + locals.var_t12_dn9), (((locals.var_t11_dn10 * locals.var_t13) + (locals.var_t11 * locals.var_t13_dn10)) + locals.var_t12_dn10), (((locals.var_t11_dn11 * locals.var_t13) + (locals.var_t11 * locals.var_t13_dn11)) + locals.var_t12_dn11), (((locals.var_t11_dn12 * locals.var_t13) + (locals.var_t11 * locals.var_t13_dn12)) + locals.var_t12_dn12),)
    } else {
        (locals.var_t14, locals.var_t14_dn3, locals.var_t14_dn4, locals.var_t14_dn5, locals.var_t14_dn6, locals.var_t14_dn7, locals.var_t14_dn8, locals.var_t14_dn9, locals.var_t14_dn10, locals.var_t14_dn11, locals.var_t14_dn12,)
    }
};
        locals.var_t14 = assign18830_e16942;
        locals.var_t14_dn3 = assign18830_e16942_d_n3;
        locals.var_t14_dn4 = assign18830_e16942_d_n4;
        locals.var_t14_dn5 = assign18830_e16942_d_n5;
        locals.var_t14_dn6 = assign18830_e16942_d_n6;
        locals.var_t14_dn7 = assign18830_e16942_d_n7;
        locals.var_t14_dn8 = assign18830_e16942_d_n8;
        locals.var_t14_dn9 = assign18830_e16942_d_n9;
        locals.var_t14_dn10 = assign18830_e16942_d_n10;
        locals.var_t14_dn11 = assign18830_e16942_d_n11;
        locals.var_t14_dn12 = assign18830_e16942_d_n12;
        locals.var_t14_rv = 0.0;

        let (assign18840_e16956, assign18840_e16956_d_n3, assign18840_e16956_d_n4, assign18840_e16956_d_n5, assign18840_e16956_d_n6, assign18840_e16956_d_n7, assign18840_e16956_d_n8, assign18840_e16956_d_n9, assign18840_e16956_d_n10, assign18840_e16956_d_n11, assign18840_e16956_d_n12,) = {
    if (locals.var_guard1221 == 0.0) {
        let assign18840_e16947: f64 = (0.5 * locals.var_here_b4soik1ox);
        let assign18840_e16949: f64 = (assign18840_e16947 * locals.var_lpe_vb__blk1054);
        let assign18840_e16952: f64 = (locals.var_phi + locals.var_pparam_b4soiketas);
        let assign18840_e16953: f64 = (assign18840_e16952).sqrt();
        let assign18840_e16954: f64 = (assign18840_e16949 / assign18840_e16953);
        (assign18840_e16954, ((((((0.5 * locals.var_here_b4soik1ox_dn3) * locals.var_lpe_vb__blk1054) + (assign18840_e16947 * locals.var_lpe_vb__blk1054_dn3)) * assign18840_e16953) - (assign18840_e16949 * ((locals.var_phi_dn3 + locals.var_pparam_b4soiketas_dn3) / (2.0 * assign18840_e16953)))) / (assign18840_e16953 * assign18840_e16953)), ((((((0.5 * locals.var_here_b4soik1ox_dn4) * locals.var_lpe_vb__blk1054) + (assign18840_e16947 * locals.var_lpe_vb__blk1054_dn4)) * assign18840_e16953) - (assign18840_e16949 * ((locals.var_phi_dn4 + locals.var_pparam_b4soiketas_dn4) / (2.0 * assign18840_e16953)))) / (assign18840_e16953 * assign18840_e16953)), ((((((0.5 * locals.var_here_b4soik1ox_dn5) * locals.var_lpe_vb__blk1054) + (assign18840_e16947 * locals.var_lpe_vb__blk1054_dn5)) * assign18840_e16953) - (assign18840_e16949 * ((locals.var_phi_dn5 + locals.var_pparam_b4soiketas_dn5) / (2.0 * assign18840_e16953)))) / (assign18840_e16953 * assign18840_e16953)), ((((((0.5 * locals.var_here_b4soik1ox_dn6) * locals.var_lpe_vb__blk1054) + (assign18840_e16947 * locals.var_lpe_vb__blk1054_dn6)) * assign18840_e16953) - (assign18840_e16949 * ((locals.var_phi_dn6 + locals.var_pparam_b4soiketas_dn6) / (2.0 * assign18840_e16953)))) / (assign18840_e16953 * assign18840_e16953)), ((((((0.5 * locals.var_here_b4soik1ox_dn7) * locals.var_lpe_vb__blk1054) + (assign18840_e16947 * locals.var_lpe_vb__blk1054_dn7)) * assign18840_e16953) - (assign18840_e16949 * ((locals.var_phi_dn7 + locals.var_pparam_b4soiketas_dn7) / (2.0 * assign18840_e16953)))) / (assign18840_e16953 * assign18840_e16953)), ((((((0.5 * locals.var_here_b4soik1ox_dn8) * locals.var_lpe_vb__blk1054) + (assign18840_e16947 * locals.var_lpe_vb__blk1054_dn8)) * assign18840_e16953) - (assign18840_e16949 * ((locals.var_phi_dn8 + locals.var_pparam_b4soiketas_dn8) / (2.0 * assign18840_e16953)))) / (assign18840_e16953 * assign18840_e16953)), ((((((0.5 * locals.var_here_b4soik1ox_dn9) * locals.var_lpe_vb__blk1054) + (assign18840_e16947 * locals.var_lpe_vb__blk1054_dn9)) * assign18840_e16953) - (assign18840_e16949 * ((locals.var_phi_dn9 + locals.var_pparam_b4soiketas_dn9) / (2.0 * assign18840_e16953)))) / (assign18840_e16953 * assign18840_e16953)), ((((((0.5 * locals.var_here_b4soik1ox_dn10) * locals.var_lpe_vb__blk1054) + (assign18840_e16947 * locals.var_lpe_vb__blk1054_dn10)) * assign18840_e16953) - (assign18840_e16949 * ((locals.var_phi_dn10 + locals.var_pparam_b4soiketas_dn10) / (2.0 * assign18840_e16953)))) / (assign18840_e16953 * assign18840_e16953)), ((((((0.5 * locals.var_here_b4soik1ox_dn11) * locals.var_lpe_vb__blk1054) + (assign18840_e16947 * locals.var_lpe_vb__blk1054_dn11)) * assign18840_e16953) - (assign18840_e16949 * ((locals.var_phi_dn11 + locals.var_pparam_b4soiketas_dn11) / (2.0 * assign18840_e16953)))) / (assign18840_e16953 * assign18840_e16953)), ((((((0.5 * locals.var_here_b4soik1ox_dn12) * locals.var_lpe_vb__blk1054) + (assign18840_e16947 * locals.var_lpe_vb__blk1054_dn12)) * assign18840_e16953) - (assign18840_e16949 * ((locals.var_phi_dn12 + locals.var_pparam_b4soiketas_dn12) / (2.0 * assign18840_e16953)))) / (assign18840_e16953 * assign18840_e16953)),)
    } else {
        (locals.var_t10__blk818, locals.var_t10__blk818_dn3, locals.var_t10__blk818_dn4, locals.var_t10__blk818_dn5, locals.var_t10__blk818_dn6, locals.var_t10__blk818_dn7, locals.var_t10__blk818_dn8, locals.var_t10__blk818_dn9, locals.var_t10__blk818_dn10, locals.var_t10__blk818_dn11, locals.var_t10__blk818_dn12,)
    }
};
        locals.var_t10__blk818 = assign18840_e16956;
        locals.var_t10__blk818_dn3 = assign18840_e16956_d_n3;
        locals.var_t10__blk818_dn4 = assign18840_e16956_d_n4;
        locals.var_t10__blk818_dn5 = assign18840_e16956_d_n5;
        locals.var_t10__blk818_dn6 = assign18840_e16956_d_n6;
        locals.var_t10__blk818_dn7 = assign18840_e16956_d_n7;
        locals.var_t10__blk818_dn8 = assign18840_e16956_d_n8;
        locals.var_t10__blk818_dn9 = assign18840_e16956_d_n9;
        locals.var_t10__blk818_dn10 = assign18840_e16956_d_n10;
        locals.var_t10__blk818_dn11 = assign18840_e16956_d_n11;
        locals.var_t10__blk818_dn12 = assign18840_e16956_d_n12;
        locals.var_t10__blk818_rv = 0.0;

        let (assign18850_e16963, assign18850_e16963_d_n3, assign18850_e16963_d_n4, assign18850_e16963_d_n5, assign18850_e16963_d_n6, assign18850_e16963_d_n7, assign18850_e16963_d_n8, assign18850_e16963_d_n9, assign18850_e16963_d_n10, assign18850_e16963_d_n11, assign18850_e16963_d_n12,) = {
    if (locals.var_guard1221 == 0.0) {
        let assign18850_e16961: f64 = (locals.var_t10__blk818 * locals.var_t14);
        (assign18850_e16961, ((locals.var_t10__blk818_dn3 * locals.var_t14) + (locals.var_t10__blk818 * locals.var_t14_dn3)), ((locals.var_t10__blk818_dn4 * locals.var_t14) + (locals.var_t10__blk818 * locals.var_t14_dn4)), ((locals.var_t10__blk818_dn5 * locals.var_t14) + (locals.var_t10__blk818 * locals.var_t14_dn5)), ((locals.var_t10__blk818_dn6 * locals.var_t14) + (locals.var_t10__blk818 * locals.var_t14_dn6)), ((locals.var_t10__blk818_dn7 * locals.var_t14) + (locals.var_t10__blk818 * locals.var_t14_dn7)), ((locals.var_t10__blk818_dn8 * locals.var_t14) + (locals.var_t10__blk818 * locals.var_t14_dn8)), ((locals.var_t10__blk818_dn9 * locals.var_t14) + (locals.var_t10__blk818 * locals.var_t14_dn9)), ((locals.var_t10__blk818_dn10 * locals.var_t14) + (locals.var_t10__blk818 * locals.var_t14_dn10)), ((locals.var_t10__blk818_dn11 * locals.var_t14) + (locals.var_t10__blk818 * locals.var_t14_dn11)), ((locals.var_t10__blk818_dn12 * locals.var_t14) + (locals.var_t10__blk818 * locals.var_t14_dn12)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign18850_e16963;
        locals.var_t1__blk809_dn3 = assign18850_e16963_d_n3;
        locals.var_t1__blk809_dn4 = assign18850_e16963_d_n4;
        locals.var_t1__blk809_dn5 = assign18850_e16963_d_n5;
        locals.var_t1__blk809_dn6 = assign18850_e16963_d_n6;
        locals.var_t1__blk809_dn7 = assign18850_e16963_d_n7;
        locals.var_t1__blk809_dn8 = assign18850_e16963_d_n8;
        locals.var_t1__blk809_dn9 = assign18850_e16963_d_n9;
        locals.var_t1__blk809_dn10 = assign18850_e16963_d_n10;
        locals.var_t1__blk809_dn11 = assign18850_e16963_d_n11;
        locals.var_t1__blk809_dn12 = assign18850_e16963_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign18860_e16971, assign18860_e16971_d_n3, assign18860_e16971_d_n4, assign18860_e16971_d_n5, assign18860_e16971_d_n6, assign18860_e16971_d_n7, assign18860_e16971_d_n8, assign18860_e16971_d_n9, assign18860_e16971_d_n10, assign18860_e16971_d_n11, assign18860_e16971_d_n12,) = {
    if (locals.var_guard1221 == 0.0) {
        let assign18860_e16968: f64 = (locals.var_pparam_b4soixj * locals.var_xdep_cv);
        let assign18860_e16969: f64 = (assign18860_e16968).sqrt();
        (assign18860_e16969, (((locals.var_pparam_b4soixj_dn3 * locals.var_xdep_cv) + (locals.var_pparam_b4soixj * locals.var_xdep_cv_dn3)) / (2.0 * assign18860_e16969)), (((locals.var_pparam_b4soixj_dn4 * locals.var_xdep_cv) + (locals.var_pparam_b4soixj * locals.var_xdep_cv_dn4)) / (2.0 * assign18860_e16969)), (((locals.var_pparam_b4soixj_dn5 * locals.var_xdep_cv) + (locals.var_pparam_b4soixj * locals.var_xdep_cv_dn5)) / (2.0 * assign18860_e16969)), (((locals.var_pparam_b4soixj_dn6 * locals.var_xdep_cv) + (locals.var_pparam_b4soixj * locals.var_xdep_cv_dn6)) / (2.0 * assign18860_e16969)), (((locals.var_pparam_b4soixj_dn7 * locals.var_xdep_cv) + (locals.var_pparam_b4soixj * locals.var_xdep_cv_dn7)) / (2.0 * assign18860_e16969)), (((locals.var_pparam_b4soixj_dn8 * locals.var_xdep_cv) + (locals.var_pparam_b4soixj * locals.var_xdep_cv_dn8)) / (2.0 * assign18860_e16969)), (((locals.var_pparam_b4soixj_dn9 * locals.var_xdep_cv) + (locals.var_pparam_b4soixj * locals.var_xdep_cv_dn9)) / (2.0 * assign18860_e16969)), (((locals.var_pparam_b4soixj_dn10 * locals.var_xdep_cv) + (locals.var_pparam_b4soixj * locals.var_xdep_cv_dn10)) / (2.0 * assign18860_e16969)), (((locals.var_pparam_b4soixj_dn11 * locals.var_xdep_cv) + (locals.var_pparam_b4soixj * locals.var_xdep_cv_dn11)) / (2.0 * assign18860_e16969)), (((locals.var_pparam_b4soixj_dn12 * locals.var_xdep_cv) + (locals.var_pparam_b4soixj * locals.var_xdep_cv_dn12)) / (2.0 * assign18860_e16969)),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign18860_e16971;
        locals.var_t9_dn3 = assign18860_e16971_d_n3;
        locals.var_t9_dn4 = assign18860_e16971_d_n4;
        locals.var_t9_dn5 = assign18860_e16971_d_n5;
        locals.var_t9_dn6 = assign18860_e16971_d_n6;
        locals.var_t9_dn7 = assign18860_e16971_d_n7;
        locals.var_t9_dn8 = assign18860_e16971_d_n8;
        locals.var_t9_dn9 = assign18860_e16971_d_n9;
        locals.var_t9_dn10 = assign18860_e16971_d_n10;
        locals.var_t9_dn11 = assign18860_e16971_d_n11;
        locals.var_t9_dn12 = assign18860_e16971_d_n12;
        locals.var_t9_rv = 0.0;

        let (assign18870_e16980, assign18870_e16980_d_n3, assign18870_e16980_d_n4, assign18870_e16980_d_n5, assign18870_e16980_d_n6, assign18870_e16980_d_n7, assign18870_e16980_d_n8, assign18870_e16980_d_n9, assign18870_e16980_d_n10, assign18870_e16980_d_n11, assign18870_e16980_d_n12,) = {
    if (locals.var_guard1221 == 0.0) {
        let assign18870_e16977: f64 = (2.0 * locals.var_t9);
        let assign18870_e16978: f64 = (locals.var_leff + assign18870_e16977);
        (assign18870_e16978, (locals.var_leff_dn3 + (2.0 * locals.var_t9_dn3)), (locals.var_leff_dn4 + (2.0 * locals.var_t9_dn4)), (locals.var_leff_dn5 + (2.0 * locals.var_t9_dn5)), (locals.var_leff_dn6 + (2.0 * locals.var_t9_dn6)), (locals.var_leff_dn7 + (2.0 * locals.var_t9_dn7)), (locals.var_leff_dn8 + (2.0 * locals.var_t9_dn8)), (locals.var_leff_dn9 + (2.0 * locals.var_t9_dn9)), (locals.var_leff_dn10 + (2.0 * locals.var_t9_dn10)), (locals.var_leff_dn11 + (2.0 * locals.var_t9_dn11)), (locals.var_leff_dn12 + (2.0 * locals.var_t9_dn12)),)
    } else {
        (locals.var_tmp1__blk834, locals.var_tmp1__blk834_dn3, locals.var_tmp1__blk834_dn4, locals.var_tmp1__blk834_dn5, locals.var_tmp1__blk834_dn6, locals.var_tmp1__blk834_dn7, locals.var_tmp1__blk834_dn8, locals.var_tmp1__blk834_dn9, locals.var_tmp1__blk834_dn10, locals.var_tmp1__blk834_dn11, locals.var_tmp1__blk834_dn12,)
    }
};
        locals.var_tmp1__blk834 = assign18870_e16980;
        locals.var_tmp1__blk834_dn3 = assign18870_e16980_d_n3;
        locals.var_tmp1__blk834_dn4 = assign18870_e16980_d_n4;
        locals.var_tmp1__blk834_dn5 = assign18870_e16980_d_n5;
        locals.var_tmp1__blk834_dn6 = assign18870_e16980_d_n6;
        locals.var_tmp1__blk834_dn7 = assign18870_e16980_d_n7;
        locals.var_tmp1__blk834_dn8 = assign18870_e16980_d_n8;
        locals.var_tmp1__blk834_dn9 = assign18870_e16980_d_n9;
        locals.var_tmp1__blk834_dn10 = assign18870_e16980_d_n10;
        locals.var_tmp1__blk834_dn11 = assign18870_e16980_d_n11;
        locals.var_tmp1__blk834_dn12 = assign18870_e16980_d_n12;
        locals.var_tmp1__blk834_rv = 0.0;

        let (assign18880_e16987, assign18880_e16987_d_n3, assign18880_e16987_d_n4, assign18880_e16987_d_n5, assign18880_e16987_d_n6, assign18880_e16987_d_n7, assign18880_e16987_d_n8, assign18880_e16987_d_n9, assign18880_e16987_d_n10, assign18880_e16987_d_n11, assign18880_e16987_d_n12,) = {
    if (locals.var_guard1221 == 0.0) {
        let assign18880_e16985: f64 = (locals.var_leff / locals.var_tmp1__blk834);
        (assign18880_e16985, (((locals.var_leff_dn3 * locals.var_tmp1__blk834) - (locals.var_leff * locals.var_tmp1__blk834_dn3)) / (locals.var_tmp1__blk834 * locals.var_tmp1__blk834)), (((locals.var_leff_dn4 * locals.var_tmp1__blk834) - (locals.var_leff * locals.var_tmp1__blk834_dn4)) / (locals.var_tmp1__blk834 * locals.var_tmp1__blk834)), (((locals.var_leff_dn5 * locals.var_tmp1__blk834) - (locals.var_leff * locals.var_tmp1__blk834_dn5)) / (locals.var_tmp1__blk834 * locals.var_tmp1__blk834)), (((locals.var_leff_dn6 * locals.var_tmp1__blk834) - (locals.var_leff * locals.var_tmp1__blk834_dn6)) / (locals.var_tmp1__blk834 * locals.var_tmp1__blk834)), (((locals.var_leff_dn7 * locals.var_tmp1__blk834) - (locals.var_leff * locals.var_tmp1__blk834_dn7)) / (locals.var_tmp1__blk834 * locals.var_tmp1__blk834)), (((locals.var_leff_dn8 * locals.var_tmp1__blk834) - (locals.var_leff * locals.var_tmp1__blk834_dn8)) / (locals.var_tmp1__blk834 * locals.var_tmp1__blk834)), (((locals.var_leff_dn9 * locals.var_tmp1__blk834) - (locals.var_leff * locals.var_tmp1__blk834_dn9)) / (locals.var_tmp1__blk834 * locals.var_tmp1__blk834)), (((locals.var_leff_dn10 * locals.var_tmp1__blk834) - (locals.var_leff * locals.var_tmp1__blk834_dn10)) / (locals.var_tmp1__blk834 * locals.var_tmp1__blk834)), (((locals.var_leff_dn11 * locals.var_tmp1__blk834) - (locals.var_leff * locals.var_tmp1__blk834_dn11)) / (locals.var_tmp1__blk834 * locals.var_tmp1__blk834)), (((locals.var_leff_dn12 * locals.var_tmp1__blk834) - (locals.var_leff * locals.var_tmp1__blk834_dn12)) / (locals.var_tmp1__blk834 * locals.var_tmp1__blk834)),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign18880_e16987;
        locals.var_t5__blk813_dn3 = assign18880_e16987_d_n3;
        locals.var_t5__blk813_dn4 = assign18880_e16987_d_n4;
        locals.var_t5__blk813_dn5 = assign18880_e16987_d_n5;
        locals.var_t5__blk813_dn6 = assign18880_e16987_d_n6;
        locals.var_t5__blk813_dn7 = assign18880_e16987_d_n7;
        locals.var_t5__blk813_dn8 = assign18880_e16987_d_n8;
        locals.var_t5__blk813_dn9 = assign18880_e16987_d_n9;
        locals.var_t5__blk813_dn10 = assign18880_e16987_d_n10;
        locals.var_t5__blk813_dn11 = assign18880_e16987_d_n11;
        locals.var_t5__blk813_dn12 = assign18880_e16987_d_n12;
        locals.var_t5__blk813_rv = 0.0;

        let (assign18890_e16994, assign18890_e16994_d_n3, assign18890_e16994_d_n4, assign18890_e16994_d_n5, assign18890_e16994_d_n6, assign18890_e16994_d_n7, assign18890_e16994_d_n8, assign18890_e16994_d_n9, assign18890_e16994_d_n10, assign18890_e16994_d_n11, assign18890_e16994_d_n12,) = {
    if (locals.var_guard1221 == 0.0) {
        let assign18890_e16992: f64 = (locals.var_pparam_b4soia0 * locals.var_t5__blk813);
        (assign18890_e16992, ((locals.var_pparam_b4soia0_dn3 * locals.var_t5__blk813) + (locals.var_pparam_b4soia0 * locals.var_t5__blk813_dn3)), ((locals.var_pparam_b4soia0_dn4 * locals.var_t5__blk813) + (locals.var_pparam_b4soia0 * locals.var_t5__blk813_dn4)), ((locals.var_pparam_b4soia0_dn5 * locals.var_t5__blk813) + (locals.var_pparam_b4soia0 * locals.var_t5__blk813_dn5)), ((locals.var_pparam_b4soia0_dn6 * locals.var_t5__blk813) + (locals.var_pparam_b4soia0 * locals.var_t5__blk813_dn6)), ((locals.var_pparam_b4soia0_dn7 * locals.var_t5__blk813) + (locals.var_pparam_b4soia0 * locals.var_t5__blk813_dn7)), ((locals.var_pparam_b4soia0_dn8 * locals.var_t5__blk813) + (locals.var_pparam_b4soia0 * locals.var_t5__blk813_dn8)), ((locals.var_pparam_b4soia0_dn9 * locals.var_t5__blk813) + (locals.var_pparam_b4soia0 * locals.var_t5__blk813_dn9)), ((locals.var_pparam_b4soia0_dn10 * locals.var_t5__blk813) + (locals.var_pparam_b4soia0 * locals.var_t5__blk813_dn10)), ((locals.var_pparam_b4soia0_dn11 * locals.var_t5__blk813) + (locals.var_pparam_b4soia0 * locals.var_t5__blk813_dn11)), ((locals.var_pparam_b4soia0_dn12 * locals.var_t5__blk813) + (locals.var_pparam_b4soia0 * locals.var_t5__blk813_dn12)),)
    } else {
        (locals.var_tmp2__blk835, locals.var_tmp2__blk835_dn3, locals.var_tmp2__blk835_dn4, locals.var_tmp2__blk835_dn5, locals.var_tmp2__blk835_dn6, locals.var_tmp2__blk835_dn7, locals.var_tmp2__blk835_dn8, locals.var_tmp2__blk835_dn9, locals.var_tmp2__blk835_dn10, locals.var_tmp2__blk835_dn11, locals.var_tmp2__blk835_dn12,)
    }
};
        locals.var_tmp2__blk835 = assign18890_e16994;
        locals.var_tmp2__blk835_dn3 = assign18890_e16994_d_n3;
        locals.var_tmp2__blk835_dn4 = assign18890_e16994_d_n4;
        locals.var_tmp2__blk835_dn5 = assign18890_e16994_d_n5;
        locals.var_tmp2__blk835_dn6 = assign18890_e16994_d_n6;
        locals.var_tmp2__blk835_dn7 = assign18890_e16994_d_n7;
        locals.var_tmp2__blk835_dn8 = assign18890_e16994_d_n8;
        locals.var_tmp2__blk835_dn9 = assign18890_e16994_d_n9;
        locals.var_tmp2__blk835_dn10 = assign18890_e16994_d_n10;
        locals.var_tmp2__blk835_dn11 = assign18890_e16994_d_n11;
        locals.var_tmp2__blk835_dn12 = assign18890_e16994_d_n12;
        locals.var_tmp2__blk835_rv = 0.0;

        let (assign18900_e17001, assign18900_e17001_d_n3, assign18900_e17001_d_n4, assign18900_e17001_d_n5, assign18900_e17001_d_n6, assign18900_e17001_d_n7, assign18900_e17001_d_n8, assign18900_e17001_d_n9, assign18900_e17001_d_n10, assign18900_e17001_d_n11, assign18900_e17001_d_n12,) = {
    if (locals.var_guard1221 == 0.0) {
        let assign18900_e16999: f64 = (locals.var_pparam_b4soiweff + locals.var_pparam_b4soib1);
        (assign18900_e16999, (locals.var_pparam_b4soiweff_dn3 + locals.var_pparam_b4soib1_dn3), (locals.var_pparam_b4soiweff_dn4 + locals.var_pparam_b4soib1_dn4), (locals.var_pparam_b4soiweff_dn5 + locals.var_pparam_b4soib1_dn5), (locals.var_pparam_b4soiweff_dn6 + locals.var_pparam_b4soib1_dn6), (locals.var_pparam_b4soiweff_dn7 + locals.var_pparam_b4soib1_dn7), (locals.var_pparam_b4soiweff_dn8 + locals.var_pparam_b4soib1_dn8), (locals.var_pparam_b4soiweff_dn9 + locals.var_pparam_b4soib1_dn9), (locals.var_pparam_b4soiweff_dn10 + locals.var_pparam_b4soib1_dn10), (locals.var_pparam_b4soiweff_dn11 + locals.var_pparam_b4soib1_dn11), (locals.var_pparam_b4soiweff_dn12 + locals.var_pparam_b4soib1_dn12),)
    } else {
        (locals.var_tmp3__blk836, locals.var_tmp3__blk836_dn3, locals.var_tmp3__blk836_dn4, locals.var_tmp3__blk836_dn5, locals.var_tmp3__blk836_dn6, locals.var_tmp3__blk836_dn7, locals.var_tmp3__blk836_dn8, locals.var_tmp3__blk836_dn9, locals.var_tmp3__blk836_dn10, locals.var_tmp3__blk836_dn11, locals.var_tmp3__blk836_dn12,)
    }
};
        locals.var_tmp3__blk836 = assign18900_e17001;
        locals.var_tmp3__blk836_dn3 = assign18900_e17001_d_n3;
        locals.var_tmp3__blk836_dn4 = assign18900_e17001_d_n4;
        locals.var_tmp3__blk836_dn5 = assign18900_e17001_d_n5;
        locals.var_tmp3__blk836_dn6 = assign18900_e17001_d_n6;
        locals.var_tmp3__blk836_dn7 = assign18900_e17001_d_n7;
        locals.var_tmp3__blk836_dn8 = assign18900_e17001_d_n8;
        locals.var_tmp3__blk836_dn9 = assign18900_e17001_d_n9;
        locals.var_tmp3__blk836_dn10 = assign18900_e17001_d_n10;
        locals.var_tmp3__blk836_dn11 = assign18900_e17001_d_n11;
        locals.var_tmp3__blk836_dn12 = assign18900_e17001_d_n12;
        locals.var_tmp3__blk836_rv = 0.0;

        let (assign18910_e17008, assign18910_e17008_d_n3, assign18910_e17008_d_n4, assign18910_e17008_d_n5, assign18910_e17008_d_n6, assign18910_e17008_d_n7, assign18910_e17008_d_n8, assign18910_e17008_d_n9, assign18910_e17008_d_n10, assign18910_e17008_d_n11, assign18910_e17008_d_n12,) = {
    if (locals.var_guard1221 == 0.0) {
        let assign18910_e17006: f64 = (locals.var_pparam_b4soib0 / locals.var_tmp3__blk836);
        (assign18910_e17006, (((locals.var_pparam_b4soib0_dn3 * locals.var_tmp3__blk836) - (locals.var_pparam_b4soib0 * locals.var_tmp3__blk836_dn3)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836)), (((locals.var_pparam_b4soib0_dn4 * locals.var_tmp3__blk836) - (locals.var_pparam_b4soib0 * locals.var_tmp3__blk836_dn4)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836)), (((locals.var_pparam_b4soib0_dn5 * locals.var_tmp3__blk836) - (locals.var_pparam_b4soib0 * locals.var_tmp3__blk836_dn5)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836)), (((locals.var_pparam_b4soib0_dn6 * locals.var_tmp3__blk836) - (locals.var_pparam_b4soib0 * locals.var_tmp3__blk836_dn6)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836)), (((locals.var_pparam_b4soib0_dn7 * locals.var_tmp3__blk836) - (locals.var_pparam_b4soib0 * locals.var_tmp3__blk836_dn7)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836)), (((locals.var_pparam_b4soib0_dn8 * locals.var_tmp3__blk836) - (locals.var_pparam_b4soib0 * locals.var_tmp3__blk836_dn8)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836)), (((locals.var_pparam_b4soib0_dn9 * locals.var_tmp3__blk836) - (locals.var_pparam_b4soib0 * locals.var_tmp3__blk836_dn9)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836)), (((locals.var_pparam_b4soib0_dn10 * locals.var_tmp3__blk836) - (locals.var_pparam_b4soib0 * locals.var_tmp3__blk836_dn10)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836)), (((locals.var_pparam_b4soib0_dn11 * locals.var_tmp3__blk836) - (locals.var_pparam_b4soib0 * locals.var_tmp3__blk836_dn11)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836)), (((locals.var_pparam_b4soib0_dn12 * locals.var_tmp3__blk836) - (locals.var_pparam_b4soib0 * locals.var_tmp3__blk836_dn12)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836)),)
    } else {
        (locals.var_tmp4, locals.var_tmp4_dn3, locals.var_tmp4_dn4, locals.var_tmp4_dn5, locals.var_tmp4_dn6, locals.var_tmp4_dn7, locals.var_tmp4_dn8, locals.var_tmp4_dn9, locals.var_tmp4_dn10, locals.var_tmp4_dn11, locals.var_tmp4_dn12,)
    }
};
        locals.var_tmp4 = assign18910_e17008;
        locals.var_tmp4_dn3 = assign18910_e17008_d_n3;
        locals.var_tmp4_dn4 = assign18910_e17008_d_n4;
        locals.var_tmp4_dn5 = assign18910_e17008_d_n5;
        locals.var_tmp4_dn6 = assign18910_e17008_d_n6;
        locals.var_tmp4_dn7 = assign18910_e17008_d_n7;
        locals.var_tmp4_dn8 = assign18910_e17008_d_n8;
        locals.var_tmp4_dn9 = assign18910_e17008_d_n9;
        locals.var_tmp4_dn10 = assign18910_e17008_d_n10;
        locals.var_tmp4_dn11 = assign18910_e17008_d_n11;
        locals.var_tmp4_dn12 = assign18910_e17008_d_n12;
        locals.var_tmp4_rv = 0.0;

        let (assign18920_e17015, assign18920_e17015_d_n3, assign18920_e17015_d_n4, assign18920_e17015_d_n5, assign18920_e17015_d_n6, assign18920_e17015_d_n7, assign18920_e17015_d_n8, assign18920_e17015_d_n9, assign18920_e17015_d_n10, assign18920_e17015_d_n11, assign18920_e17015_d_n12,) = {
    if (locals.var_guard1221 == 0.0) {
        let assign18920_e17013: f64 = (locals.var_tmp2__blk835 + locals.var_tmp4);
        (assign18920_e17013, (locals.var_tmp2__blk835_dn3 + locals.var_tmp4_dn3), (locals.var_tmp2__blk835_dn4 + locals.var_tmp4_dn4), (locals.var_tmp2__blk835_dn5 + locals.var_tmp4_dn5), (locals.var_tmp2__blk835_dn6 + locals.var_tmp4_dn6), (locals.var_tmp2__blk835_dn7 + locals.var_tmp4_dn7), (locals.var_tmp2__blk835_dn8 + locals.var_tmp4_dn8), (locals.var_tmp2__blk835_dn9 + locals.var_tmp4_dn9), (locals.var_tmp2__blk835_dn10 + locals.var_tmp4_dn10), (locals.var_tmp2__blk835_dn11 + locals.var_tmp4_dn11), (locals.var_tmp2__blk835_dn12 + locals.var_tmp4_dn12),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign18920_e17015;
        locals.var_t2__blk810_dn3 = assign18920_e17015_d_n3;
        locals.var_t2__blk810_dn4 = assign18920_e17015_d_n4;
        locals.var_t2__blk810_dn5 = assign18920_e17015_d_n5;
        locals.var_t2__blk810_dn6 = assign18920_e17015_d_n6;
        locals.var_t2__blk810_dn7 = assign18920_e17015_d_n7;
        locals.var_t2__blk810_dn8 = assign18920_e17015_d_n8;
        locals.var_t2__blk810_dn9 = assign18920_e17015_d_n9;
        locals.var_t2__blk810_dn10 = assign18920_e17015_d_n10;
        locals.var_t2__blk810_dn11 = assign18920_e17015_d_n11;
        locals.var_t2__blk810_dn12 = assign18920_e17015_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign18930_e17022, assign18930_e17022_d_n3, assign18930_e17022_d_n4, assign18930_e17022_d_n5, assign18930_e17022_d_n6, assign18930_e17022_d_n7, assign18930_e17022_d_n8, assign18930_e17022_d_n9, assign18930_e17022_d_n10, assign18930_e17022_d_n11, assign18930_e17022_d_n12,) = {
    if (locals.var_guard1221 == 0.0) {
        let assign18930_e17020: f64 = (locals.var_t5__blk813 * locals.var_t5__blk813);
        (assign18930_e17020, ((locals.var_t5__blk813_dn3 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn3)), ((locals.var_t5__blk813_dn4 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn4)), ((locals.var_t5__blk813_dn5 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn5)), ((locals.var_t5__blk813_dn6 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn6)), ((locals.var_t5__blk813_dn7 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn7)), ((locals.var_t5__blk813_dn8 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn8)), ((locals.var_t5__blk813_dn9 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn9)), ((locals.var_t5__blk813_dn10 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn10)), ((locals.var_t5__blk813_dn11 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn11)), ((locals.var_t5__blk813_dn12 * locals.var_t5__blk813) + (locals.var_t5__blk813 * locals.var_t5__blk813_dn12)),)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign18930_e17022;
        locals.var_t6__blk814_dn3 = assign18930_e17022_d_n3;
        locals.var_t6__blk814_dn4 = assign18930_e17022_d_n4;
        locals.var_t6__blk814_dn5 = assign18930_e17022_d_n5;
        locals.var_t6__blk814_dn6 = assign18930_e17022_d_n6;
        locals.var_t6__blk814_dn7 = assign18930_e17022_d_n7;
        locals.var_t6__blk814_dn8 = assign18930_e17022_d_n8;
        locals.var_t6__blk814_dn9 = assign18930_e17022_d_n9;
        locals.var_t6__blk814_dn10 = assign18930_e17022_d_n10;
        locals.var_t6__blk814_dn11 = assign18930_e17022_d_n11;
        locals.var_t6__blk814_dn12 = assign18930_e17022_d_n12;
        locals.var_t6__blk814_rv = 0.0;

        let (assign18940_e17029, assign18940_e17029_d_n3, assign18940_e17029_d_n4, assign18940_e17029_d_n5, assign18940_e17029_d_n6, assign18940_e17029_d_n7, assign18940_e17029_d_n8, assign18940_e17029_d_n9, assign18940_e17029_d_n10, assign18940_e17029_d_n11, assign18940_e17029_d_n12,) = {
    if (locals.var_guard1221 == 0.0) {
        let assign18940_e17027: f64 = (locals.var_t5__blk813 * locals.var_t6__blk814);
        (assign18940_e17027, ((locals.var_t5__blk813_dn3 * locals.var_t6__blk814) + (locals.var_t5__blk813 * locals.var_t6__blk814_dn3)), ((locals.var_t5__blk813_dn4 * locals.var_t6__blk814) + (locals.var_t5__blk813 * locals.var_t6__blk814_dn4)), ((locals.var_t5__blk813_dn5 * locals.var_t6__blk814) + (locals.var_t5__blk813 * locals.var_t6__blk814_dn5)), ((locals.var_t5__blk813_dn6 * locals.var_t6__blk814) + (locals.var_t5__blk813 * locals.var_t6__blk814_dn6)), ((locals.var_t5__blk813_dn7 * locals.var_t6__blk814) + (locals.var_t5__blk813 * locals.var_t6__blk814_dn7)), ((locals.var_t5__blk813_dn8 * locals.var_t6__blk814) + (locals.var_t5__blk813 * locals.var_t6__blk814_dn8)), ((locals.var_t5__blk813_dn9 * locals.var_t6__blk814) + (locals.var_t5__blk813 * locals.var_t6__blk814_dn9)), ((locals.var_t5__blk813_dn10 * locals.var_t6__blk814) + (locals.var_t5__blk813 * locals.var_t6__blk814_dn10)), ((locals.var_t5__blk813_dn11 * locals.var_t6__blk814) + (locals.var_t5__blk813 * locals.var_t6__blk814_dn11)), ((locals.var_t5__blk813_dn12 * locals.var_t6__blk814) + (locals.var_t5__blk813 * locals.var_t6__blk814_dn12)),)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign18940_e17029;
        locals.var_t7__blk815_dn3 = assign18940_e17029_d_n3;
        locals.var_t7__blk815_dn4 = assign18940_e17029_d_n4;
        locals.var_t7__blk815_dn5 = assign18940_e17029_d_n5;
        locals.var_t7__blk815_dn6 = assign18940_e17029_d_n6;
        locals.var_t7__blk815_dn7 = assign18940_e17029_d_n7;
        locals.var_t7__blk815_dn8 = assign18940_e17029_d_n8;
        locals.var_t7__blk815_dn9 = assign18940_e17029_d_n9;
        locals.var_t7__blk815_dn10 = assign18940_e17029_d_n10;
        locals.var_t7__blk815_dn11 = assign18940_e17029_d_n11;
        locals.var_t7__blk815_dn12 = assign18940_e17029_d_n12;
        locals.var_t7__blk815_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_57(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18950_e17038, assign18950_e17038_d_n3, assign18950_e17038_d_n4, assign18950_e17038_d_n5, assign18950_e17038_d_n6, assign18950_e17038_d_n7, assign18950_e17038_d_n8, assign18950_e17038_d_n9, assign18950_e17038_d_n10, assign18950_e17038_d_n11, assign18950_e17038_d_n12,) = {
    if (locals.var_guard1221 == 0.0) {
        let assign18950_e17035: f64 = (locals.var_t1__blk809 * locals.var_t2__blk810);
        let assign18950_e17036: f64 = (1.0 + assign18950_e17035);
        (assign18950_e17036, ((locals.var_t1__blk809_dn3 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn3)), ((locals.var_t1__blk809_dn4 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn4)), ((locals.var_t1__blk809_dn5 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn5)), ((locals.var_t1__blk809_dn6 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn6)), ((locals.var_t1__blk809_dn7 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn7)), ((locals.var_t1__blk809_dn8 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn8)), ((locals.var_t1__blk809_dn9 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn9)), ((locals.var_t1__blk809_dn10 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn10)), ((locals.var_t1__blk809_dn11 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn11)), ((locals.var_t1__blk809_dn12 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn12)),)
    } else {
        (locals.var_abulk0_cv, locals.var_abulk0_cv_dn3, locals.var_abulk0_cv_dn4, locals.var_abulk0_cv_dn5, locals.var_abulk0_cv_dn6, locals.var_abulk0_cv_dn7, locals.var_abulk0_cv_dn8, locals.var_abulk0_cv_dn9, locals.var_abulk0_cv_dn10, locals.var_abulk0_cv_dn11, locals.var_abulk0_cv_dn12,)
    }
};
        locals.var_abulk0_cv = assign18950_e17038;
        locals.var_abulk0_cv_dn3 = assign18950_e17038_d_n3;
        locals.var_abulk0_cv_dn4 = assign18950_e17038_d_n4;
        locals.var_abulk0_cv_dn5 = assign18950_e17038_d_n5;
        locals.var_abulk0_cv_dn6 = assign18950_e17038_d_n6;
        locals.var_abulk0_cv_dn7 = assign18950_e17038_d_n7;
        locals.var_abulk0_cv_dn8 = assign18950_e17038_d_n8;
        locals.var_abulk0_cv_dn9 = assign18950_e17038_d_n9;
        locals.var_abulk0_cv_dn10 = assign18950_e17038_d_n10;
        locals.var_abulk0_cv_dn11 = assign18950_e17038_d_n11;
        locals.var_abulk0_cv_dn12 = assign18950_e17038_d_n12;
        locals.var_abulk0_cv_rv = 0.0;

        let assign18960_e17041: f64 = if locals.var_abulk0_cv < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard1224 = assign18960_e17041;
        locals.var_guard1224_rv = 0.0;

        let (assign18970_e17051, assign18970_e17051_d_n3, assign18970_e17051_d_n4, assign18970_e17051_d_n5, assign18970_e17051_d_n6, assign18970_e17051_d_n7, assign18970_e17051_d_n8, assign18970_e17051_d_n9, assign18970_e17051_d_n10, assign18970_e17051_d_n11, assign18970_e17051_d_n12,) = {
    if (locals.var_guard1224 != 0.0) {
        let assign18970_e17047: f64 = (200.0 * locals.var_abulk0_cv);
        let assign18970_e17048: f64 = (3.0 - assign18970_e17047);
        let assign18970_e17049: f64 = (1.0 / assign18970_e17048);
        (assign18970_e17049, (-((-(200.0 * locals.var_abulk0_cv_dn3)) / (assign18970_e17048 * assign18970_e17048))), (-((-(200.0 * locals.var_abulk0_cv_dn4)) / (assign18970_e17048 * assign18970_e17048))), (-((-(200.0 * locals.var_abulk0_cv_dn5)) / (assign18970_e17048 * assign18970_e17048))), (-((-(200.0 * locals.var_abulk0_cv_dn6)) / (assign18970_e17048 * assign18970_e17048))), (-((-(200.0 * locals.var_abulk0_cv_dn7)) / (assign18970_e17048 * assign18970_e17048))), (-((-(200.0 * locals.var_abulk0_cv_dn8)) / (assign18970_e17048 * assign18970_e17048))), (-((-(200.0 * locals.var_abulk0_cv_dn9)) / (assign18970_e17048 * assign18970_e17048))), (-((-(200.0 * locals.var_abulk0_cv_dn10)) / (assign18970_e17048 * assign18970_e17048))), (-((-(200.0 * locals.var_abulk0_cv_dn11)) / (assign18970_e17048 * assign18970_e17048))), (-((-(200.0 * locals.var_abulk0_cv_dn12)) / (assign18970_e17048 * assign18970_e17048))),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign18970_e17051;
        locals.var_t9_dn3 = assign18970_e17051_d_n3;
        locals.var_t9_dn4 = assign18970_e17051_d_n4;
        locals.var_t9_dn5 = assign18970_e17051_d_n5;
        locals.var_t9_dn6 = assign18970_e17051_d_n6;
        locals.var_t9_dn7 = assign18970_e17051_d_n7;
        locals.var_t9_dn8 = assign18970_e17051_d_n8;
        locals.var_t9_dn9 = assign18970_e17051_d_n9;
        locals.var_t9_dn10 = assign18970_e17051_d_n10;
        locals.var_t9_dn11 = assign18970_e17051_d_n11;
        locals.var_t9_dn12 = assign18970_e17051_d_n12;
        locals.var_t9_rv = 0.0;

        let (assign18980_e17059, assign18980_e17059_d_n3, assign18980_e17059_d_n4, assign18980_e17059_d_n5, assign18980_e17059_d_n6, assign18980_e17059_d_n7, assign18980_e17059_d_n8, assign18980_e17059_d_n9, assign18980_e17059_d_n10, assign18980_e17059_d_n11, assign18980_e17059_d_n12,) = {
    if (locals.var_guard1224 != 0.0) {
        let assign18980_e17055: f64 = (0.02 - locals.var_abulk0_cv);
        let assign18980_e17057: f64 = (assign18980_e17055 * locals.var_t9);
        (assign18980_e17057, (((-locals.var_abulk0_cv_dn3) * locals.var_t9) + (assign18980_e17055 * locals.var_t9_dn3)), (((-locals.var_abulk0_cv_dn4) * locals.var_t9) + (assign18980_e17055 * locals.var_t9_dn4)), (((-locals.var_abulk0_cv_dn5) * locals.var_t9) + (assign18980_e17055 * locals.var_t9_dn5)), (((-locals.var_abulk0_cv_dn6) * locals.var_t9) + (assign18980_e17055 * locals.var_t9_dn6)), (((-locals.var_abulk0_cv_dn7) * locals.var_t9) + (assign18980_e17055 * locals.var_t9_dn7)), (((-locals.var_abulk0_cv_dn8) * locals.var_t9) + (assign18980_e17055 * locals.var_t9_dn8)), (((-locals.var_abulk0_cv_dn9) * locals.var_t9) + (assign18980_e17055 * locals.var_t9_dn9)), (((-locals.var_abulk0_cv_dn10) * locals.var_t9) + (assign18980_e17055 * locals.var_t9_dn10)), (((-locals.var_abulk0_cv_dn11) * locals.var_t9) + (assign18980_e17055 * locals.var_t9_dn11)), (((-locals.var_abulk0_cv_dn12) * locals.var_t9) + (assign18980_e17055 * locals.var_t9_dn12)),)
    } else {
        (locals.var_abulk0_cv, locals.var_abulk0_cv_dn3, locals.var_abulk0_cv_dn4, locals.var_abulk0_cv_dn5, locals.var_abulk0_cv_dn6, locals.var_abulk0_cv_dn7, locals.var_abulk0_cv_dn8, locals.var_abulk0_cv_dn9, locals.var_abulk0_cv_dn10, locals.var_abulk0_cv_dn11, locals.var_abulk0_cv_dn12,)
    }
};
        locals.var_abulk0_cv = assign18980_e17059;
        locals.var_abulk0_cv_dn3 = assign18980_e17059_d_n3;
        locals.var_abulk0_cv_dn4 = assign18980_e17059_d_n4;
        locals.var_abulk0_cv_dn5 = assign18980_e17059_d_n5;
        locals.var_abulk0_cv_dn6 = assign18980_e17059_d_n6;
        locals.var_abulk0_cv_dn7 = assign18980_e17059_d_n7;
        locals.var_abulk0_cv_dn8 = assign18980_e17059_d_n8;
        locals.var_abulk0_cv_dn9 = assign18980_e17059_d_n9;
        locals.var_abulk0_cv_dn10 = assign18980_e17059_d_n10;
        locals.var_abulk0_cv_dn11 = assign18980_e17059_d_n11;
        locals.var_abulk0_cv_dn12 = assign18980_e17059_d_n12;
        locals.var_abulk0_cv_rv = 0.0;

        let (assign18990_e17075, assign18990_e17075_d_n3, assign18990_e17075_d_n4, assign18990_e17075_d_n5, assign18990_e17075_d_n6, assign18990_e17075_d_n7, assign18990_e17075_d_n8, assign18990_e17075_d_n9, assign18990_e17075_d_n10, assign18990_e17075_d_n11, assign18990_e17075_d_n12,) = {
    if (p.p41 != 0.0) {
        let assign18990_e17063: f64 = (2.0 * p.p37);
        let assign18990_e17066: f64 = (p.p52 - p.p53);
        let assign18990_e17069: f64 = (0.5 * locals.var_eg__blk877);
        let assign18990_e17070: f64 = (assign18990_e17066 - assign18990_e17069);
        let assign18990_e17072: f64 = (assign18990_e17070 + 0.45);
        let assign18990_e17073: f64 = (assign18990_e17063 * assign18990_e17072);
        (assign18990_e17073, (assign18990_e17063 * (-(0.5 * locals.var_eg__blk877_dn3))), (assign18990_e17063 * (-(0.5 * locals.var_eg__blk877_dn4))), (assign18990_e17063 * (-(0.5 * locals.var_eg__blk877_dn5))), (assign18990_e17063 * (-(0.5 * locals.var_eg__blk877_dn6))), (assign18990_e17063 * (-(0.5 * locals.var_eg__blk877_dn7))), (assign18990_e17063 * (-(0.5 * locals.var_eg__blk877_dn8))), (assign18990_e17063 * (-(0.5 * locals.var_eg__blk877_dn9))), (assign18990_e17063 * (-(0.5 * locals.var_eg__blk877_dn10))), (assign18990_e17063 * (-(0.5 * locals.var_eg__blk877_dn11))), (assign18990_e17063 * (-(0.5 * locals.var_eg__blk877_dn12))),)
    } else {
        (locals.var_t14, locals.var_t14_dn3, locals.var_t14_dn4, locals.var_t14_dn5, locals.var_t14_dn6, locals.var_t14_dn7, locals.var_t14_dn8, locals.var_t14_dn9, locals.var_t14_dn10, locals.var_t14_dn11, locals.var_t14_dn12,)
    }
};
        locals.var_t14 = assign18990_e17075;
        locals.var_t14_dn3 = assign18990_e17075_d_n3;
        locals.var_t14_dn4 = assign18990_e17075_d_n4;
        locals.var_t14_dn5 = assign18990_e17075_d_n5;
        locals.var_t14_dn6 = assign18990_e17075_d_n6;
        locals.var_t14_dn7 = assign18990_e17075_d_n7;
        locals.var_t14_dn8 = assign18990_e17075_d_n8;
        locals.var_t14_dn9 = assign18990_e17075_d_n9;
        locals.var_t14_dn10 = assign18990_e17075_d_n10;
        locals.var_t14_dn11 = assign18990_e17075_d_n11;
        locals.var_t14_dn12 = assign18990_e17075_d_n12;
        locals.var_t14_rv = 0.0;

        let (assign19000_e17083,) = {
    if (p.p41 != 0.0) {
        let assign19000_e17079: f64 = (p.p45 * p.p47);
        let assign19000_e17081: f64 = (assign19000_e17079 / 3.9);
        (assign19000_e17081,)
    } else {
        (locals.var_toxe_mob,)
    }
};
        locals.var_toxe_mob = assign19000_e17083;
        locals.var_toxe_mob_rv = 0.0;

        let (assign19010_e17091, assign19010_e17091_d_n3, assign19010_e17091_d_n4, assign19010_e17091_d_n5, assign19010_e17091_d_n6, assign19010_e17091_d_n7, assign19010_e17091_d_n8, assign19010_e17091_d_n9, assign19010_e17091_d_n10, assign19010_e17091_d_n11, assign19010_e17091_d_n12,) = {
    if (p.p41 != 0.0) {
        let assign19010_e17088: f64 = (locals.var_ves - locals.var_vfbb);
        let assign19010_e17089: f64 = (p.p123 * assign19010_e17088);
        (assign19010_e17089, (p.p123 * (locals.var_ves_dn3 - locals.var_vfbb_dn3)), (p.p123 * (-locals.var_vfbb_dn4)), (p.p123 * (-locals.var_vfbb_dn5)), (p.p123 * (-locals.var_vfbb_dn6)), (p.p123 * (-locals.var_vfbb_dn7)), (p.p123 * (locals.var_ves_dn8 - locals.var_vfbb_dn8)), (p.p123 * (-locals.var_vfbb_dn9)), (p.p123 * (-locals.var_vfbb_dn10)), (p.p123 * (-locals.var_vfbb_dn11)), (p.p123 * (-locals.var_vfbb_dn12)),)
    } else {
        (locals.var_t15, locals.var_t15_dn3, locals.var_t15_dn4, locals.var_t15_dn5, locals.var_t15_dn6, locals.var_t15_dn7, locals.var_t15_dn8, locals.var_t15_dn9, locals.var_t15_dn10, locals.var_t15_dn11, locals.var_t15_dn12,)
    }
};
        locals.var_t15 = assign19010_e17091;
        locals.var_t15_dn3 = assign19010_e17091_d_n3;
        locals.var_t15_dn4 = assign19010_e17091_d_n4;
        locals.var_t15_dn5 = assign19010_e17091_d_n5;
        locals.var_t15_dn6 = assign19010_e17091_d_n6;
        locals.var_t15_dn7 = assign19010_e17091_d_n7;
        locals.var_t15_dn8 = assign19010_e17091_d_n8;
        locals.var_t15_dn9 = assign19010_e17091_d_n9;
        locals.var_t15_dn10 = assign19010_e17091_d_n10;
        locals.var_t15_dn11 = assign19010_e17091_d_n11;
        locals.var_t15_dn12 = assign19010_e17091_d_n12;
        locals.var_t15_rv = 0.0;

        let (assign19020_e17096, assign19020_e17096_d_n3, assign19020_e17096_d_n4, assign19020_e17096_d_n5, assign19020_e17096_d_n6, assign19020_e17096_d_n7, assign19020_e17096_d_n8, assign19020_e17096_d_n9, assign19020_e17096_d_n10, assign19020_e17096_d_n11, assign19020_e17096_d_n12,) = {
    if (p.p41 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t14, locals.var_t14_dn3, locals.var_t14_dn4, locals.var_t14_dn5, locals.var_t14_dn6, locals.var_t14_dn7, locals.var_t14_dn8, locals.var_t14_dn9, locals.var_t14_dn10, locals.var_t14_dn11, locals.var_t14_dn12,)
    }
};
        locals.var_t14 = assign19020_e17096;
        locals.var_t14_dn3 = assign19020_e17096_d_n3;
        locals.var_t14_dn4 = assign19020_e17096_d_n4;
        locals.var_t14_dn5 = assign19020_e17096_d_n5;
        locals.var_t14_dn6 = assign19020_e17096_d_n6;
        locals.var_t14_dn7 = assign19020_e17096_d_n7;
        locals.var_t14_dn8 = assign19020_e17096_d_n8;
        locals.var_t14_dn9 = assign19020_e17096_d_n9;
        locals.var_t14_dn10 = assign19020_e17096_d_n10;
        locals.var_t14_dn11 = assign19020_e17096_d_n11;
        locals.var_t14_dn12 = assign19020_e17096_d_n12;
        locals.var_t14_rv = 0.0;

        let (assign19030_e17101,) = {
    if (p.p41 == 0.0) {
        (p.p66,)
    } else {
        (locals.var_toxe_mob,)
    }
};
        locals.var_toxe_mob = assign19030_e17101;
        locals.var_toxe_mob_rv = 0.0;

        let (assign19040_e17110, assign19040_e17110_d_n3, assign19040_e17110_d_n4, assign19040_e17110_d_n5, assign19040_e17110_d_n6, assign19040_e17110_d_n7, assign19040_e17110_d_n8, assign19040_e17110_d_n9, assign19040_e17110_d_n10, assign19040_e17110_d_n11, assign19040_e17110_d_n12,) = {
    if (p.p41 == 0.0) {
        let assign19040_e17107: f64 = (locals.var_ves - locals.var_vfbb);
        let assign19040_e17108: f64 = (p.p123 * assign19040_e17107);
        (assign19040_e17108, (p.p123 * (locals.var_ves_dn3 - locals.var_vfbb_dn3)), (p.p123 * (-locals.var_vfbb_dn4)), (p.p123 * (-locals.var_vfbb_dn5)), (p.p123 * (-locals.var_vfbb_dn6)), (p.p123 * (-locals.var_vfbb_dn7)), (p.p123 * (locals.var_ves_dn8 - locals.var_vfbb_dn8)), (p.p123 * (-locals.var_vfbb_dn9)), (p.p123 * (-locals.var_vfbb_dn10)), (p.p123 * (-locals.var_vfbb_dn11)), (p.p123 * (-locals.var_vfbb_dn12)),)
    } else {
        (locals.var_t15, locals.var_t15_dn3, locals.var_t15_dn4, locals.var_t15_dn5, locals.var_t15_dn6, locals.var_t15_dn7, locals.var_t15_dn8, locals.var_t15_dn9, locals.var_t15_dn10, locals.var_t15_dn11, locals.var_t15_dn12,)
    }
};
        locals.var_t15 = assign19040_e17110;
        locals.var_t15_dn3 = assign19040_e17110_d_n3;
        locals.var_t15_dn4 = assign19040_e17110_d_n4;
        locals.var_t15_dn5 = assign19040_e17110_d_n5;
        locals.var_t15_dn6 = assign19040_e17110_d_n6;
        locals.var_t15_dn7 = assign19040_e17110_d_n7;
        locals.var_t15_dn8 = assign19040_e17110_d_n8;
        locals.var_t15_dn9 = assign19040_e17110_d_n9;
        locals.var_t15_dn10 = assign19040_e17110_d_n10;
        locals.var_t15_dn11 = assign19040_e17110_d_n11;
        locals.var_t15_dn12 = assign19040_e17110_d_n12;
        locals.var_t15_rv = 0.0;

        let assign19050_e17113: f64 = if p.p62 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1225 = assign19050_e17113;
        locals.var_guard1225_rv = 0.0;

        let (assign19060_e17123, assign19060_e17123_d_n3, assign19060_e17123_d_n4, assign19060_e17123_d_n5, assign19060_e17123_d_n6, assign19060_e17123_d_n7, assign19060_e17123_d_n8, assign19060_e17123_d_n9, assign19060_e17123_d_n10, assign19060_e17123_d_n11, assign19060_e17123_d_n12,) = {
    if (locals.var_guard1225 != 0.0) {
        let assign19060_e17117: f64 = (locals.var_vgsteff__blk840 + locals.var_vth__blk794);
        let assign19060_e17119: f64 = (assign19060_e17117 + locals.var_vth__blk794);
        let assign19060_e17121: f64 = (assign19060_e17119 - locals.var_t14);
        (assign19060_e17121, (((locals.var_vgsteff__blk840_dn3 + locals.var_vth__blk794_dn3) + locals.var_vth__blk794_dn3) - locals.var_t14_dn3), (((locals.var_vgsteff__blk840_dn4 + locals.var_vth__blk794_dn4) + locals.var_vth__blk794_dn4) - locals.var_t14_dn4), (((locals.var_vgsteff__blk840_dn5 + locals.var_vth__blk794_dn5) + locals.var_vth__blk794_dn5) - locals.var_t14_dn5), (((locals.var_vgsteff__blk840_dn6 + locals.var_vth__blk794_dn6) + locals.var_vth__blk794_dn6) - locals.var_t14_dn6), (((locals.var_vgsteff__blk840_dn7 + locals.var_vth__blk794_dn7) + locals.var_vth__blk794_dn7) - locals.var_t14_dn7), (((locals.var_vgsteff__blk840_dn8 + locals.var_vth__blk794_dn8) + locals.var_vth__blk794_dn8) - locals.var_t14_dn8), (((locals.var_vgsteff__blk840_dn9 + locals.var_vth__blk794_dn9) + locals.var_vth__blk794_dn9) - locals.var_t14_dn9), (((locals.var_vgsteff__blk840_dn10 + locals.var_vth__blk794_dn10) + locals.var_vth__blk794_dn10) - locals.var_t14_dn10), (((locals.var_vgsteff__blk840_dn11 + locals.var_vth__blk794_dn11) + locals.var_vth__blk794_dn11) - locals.var_t14_dn11), (((locals.var_vgsteff__blk840_dn12 + locals.var_vth__blk794_dn12) + locals.var_vth__blk794_dn12) - locals.var_t14_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign19060_e17123;
        locals.var_t0__blk808_dn3 = assign19060_e17123_d_n3;
        locals.var_t0__blk808_dn4 = assign19060_e17123_d_n4;
        locals.var_t0__blk808_dn5 = assign19060_e17123_d_n5;
        locals.var_t0__blk808_dn6 = assign19060_e17123_d_n6;
        locals.var_t0__blk808_dn7 = assign19060_e17123_d_n7;
        locals.var_t0__blk808_dn8 = assign19060_e17123_d_n8;
        locals.var_t0__blk808_dn9 = assign19060_e17123_d_n9;
        locals.var_t0__blk808_dn10 = assign19060_e17123_d_n10;
        locals.var_t0__blk808_dn11 = assign19060_e17123_d_n11;
        locals.var_t0__blk808_dn12 = assign19060_e17123_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign19070_e17131, assign19070_e17131_d_n3, assign19070_e17131_d_n4, assign19070_e17131_d_n5, assign19070_e17131_d_n6, assign19070_e17131_d_n7, assign19070_e17131_d_n8, assign19070_e17131_d_n9, assign19070_e17131_d_n10, assign19070_e17131_d_n11, assign19070_e17131_d_n12,) = {
    if (locals.var_guard1225 != 0.0) {
        let assign19070_e17128: f64 = (locals.var_uc * locals.var_vbseff);
        let assign19070_e17129: f64 = (locals.var_ua + assign19070_e17128);
        (assign19070_e17129, (locals.var_ua_dn3 + ((locals.var_uc_dn3 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn3))), (locals.var_ua_dn4 + ((locals.var_uc_dn4 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn4))), (locals.var_ua_dn5 + ((locals.var_uc_dn5 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn5))), (locals.var_ua_dn6 + ((locals.var_uc_dn6 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn6))), (locals.var_ua_dn7 + ((locals.var_uc_dn7 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn7))), (locals.var_ua_dn8 + ((locals.var_uc_dn8 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn8))), (locals.var_ua_dn9 + ((locals.var_uc_dn9 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn9))), (locals.var_ua_dn10 + ((locals.var_uc_dn10 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn10))), (locals.var_ua_dn11 + ((locals.var_uc_dn11 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn11))), (locals.var_ua_dn12 + ((locals.var_uc_dn12 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn12))),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign19070_e17131;
        locals.var_t2__blk810_dn3 = assign19070_e17131_d_n3;
        locals.var_t2__blk810_dn4 = assign19070_e17131_d_n4;
        locals.var_t2__blk810_dn5 = assign19070_e17131_d_n5;
        locals.var_t2__blk810_dn6 = assign19070_e17131_d_n6;
        locals.var_t2__blk810_dn7 = assign19070_e17131_d_n7;
        locals.var_t2__blk810_dn8 = assign19070_e17131_d_n8;
        locals.var_t2__blk810_dn9 = assign19070_e17131_d_n9;
        locals.var_t2__blk810_dn10 = assign19070_e17131_d_n10;
        locals.var_t2__blk810_dn11 = assign19070_e17131_d_n11;
        locals.var_t2__blk810_dn12 = assign19070_e17131_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign19080_e17137, assign19080_e17137_d_n3, assign19080_e17137_d_n4, assign19080_e17137_d_n5, assign19080_e17137_d_n6, assign19080_e17137_d_n7, assign19080_e17137_d_n8, assign19080_e17137_d_n9, assign19080_e17137_d_n10, assign19080_e17137_d_n11, assign19080_e17137_d_n12,) = {
    if (locals.var_guard1225 != 0.0) {
        let assign19080_e17135: f64 = (locals.var_t0__blk808 / locals.var_toxe_mob);
        (assign19080_e17135, (locals.var_t0__blk808_dn3 / locals.var_toxe_mob), (locals.var_t0__blk808_dn4 / locals.var_toxe_mob), (locals.var_t0__blk808_dn5 / locals.var_toxe_mob), (locals.var_t0__blk808_dn6 / locals.var_toxe_mob), (locals.var_t0__blk808_dn7 / locals.var_toxe_mob), (locals.var_t0__blk808_dn8 / locals.var_toxe_mob), (locals.var_t0__blk808_dn9 / locals.var_toxe_mob), (locals.var_t0__blk808_dn10 / locals.var_toxe_mob), (locals.var_t0__blk808_dn11 / locals.var_toxe_mob), (locals.var_t0__blk808_dn12 / locals.var_toxe_mob),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign19080_e17137;
        locals.var_t3__blk811_dn3 = assign19080_e17137_d_n3;
        locals.var_t3__blk811_dn4 = assign19080_e17137_d_n4;
        locals.var_t3__blk811_dn5 = assign19080_e17137_d_n5;
        locals.var_t3__blk811_dn6 = assign19080_e17137_d_n6;
        locals.var_t3__blk811_dn7 = assign19080_e17137_d_n7;
        locals.var_t3__blk811_dn8 = assign19080_e17137_d_n8;
        locals.var_t3__blk811_dn9 = assign19080_e17137_d_n9;
        locals.var_t3__blk811_dn10 = assign19080_e17137_d_n10;
        locals.var_t3__blk811_dn11 = assign19080_e17137_d_n11;
        locals.var_t3__blk811_dn12 = assign19080_e17137_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign19090_e17149, assign19090_e17149_d_n3, assign19090_e17149_d_n4, assign19090_e17149_d_n5, assign19090_e17149_d_n6, assign19090_e17149_d_n7, assign19090_e17149_d_n8, assign19090_e17149_d_n9, assign19090_e17149_d_n10, assign19090_e17149_d_n11, assign19090_e17149_d_n12,) = {
    if (locals.var_guard1225 != 0.0) {
        let assign19090_e17142: f64 = (locals.var_t2__blk810 + locals.var_t15);
        let assign19090_e17145: f64 = (locals.var_ub * locals.var_t3__blk811);
        let assign19090_e17146: f64 = (assign19090_e17142 + assign19090_e17145);
        let assign19090_e17147: f64 = (locals.var_t3__blk811 * assign19090_e17146);
        (assign19090_e17147, ((locals.var_t3__blk811_dn3 * assign19090_e17146) + (locals.var_t3__blk811 * ((locals.var_t2__blk810_dn3 + locals.var_t15_dn3) + ((locals.var_ub_dn3 * locals.var_t3__blk811) + (locals.var_ub * locals.var_t3__blk811_dn3))))), ((locals.var_t3__blk811_dn4 * assign19090_e17146) + (locals.var_t3__blk811 * ((locals.var_t2__blk810_dn4 + locals.var_t15_dn4) + ((locals.var_ub_dn4 * locals.var_t3__blk811) + (locals.var_ub * locals.var_t3__blk811_dn4))))), ((locals.var_t3__blk811_dn5 * assign19090_e17146) + (locals.var_t3__blk811 * ((locals.var_t2__blk810_dn5 + locals.var_t15_dn5) + ((locals.var_ub_dn5 * locals.var_t3__blk811) + (locals.var_ub * locals.var_t3__blk811_dn5))))), ((locals.var_t3__blk811_dn6 * assign19090_e17146) + (locals.var_t3__blk811 * ((locals.var_t2__blk810_dn6 + locals.var_t15_dn6) + ((locals.var_ub_dn6 * locals.var_t3__blk811) + (locals.var_ub * locals.var_t3__blk811_dn6))))), ((locals.var_t3__blk811_dn7 * assign19090_e17146) + (locals.var_t3__blk811 * ((locals.var_t2__blk810_dn7 + locals.var_t15_dn7) + ((locals.var_ub_dn7 * locals.var_t3__blk811) + (locals.var_ub * locals.var_t3__blk811_dn7))))), ((locals.var_t3__blk811_dn8 * assign19090_e17146) + (locals.var_t3__blk811 * ((locals.var_t2__blk810_dn8 + locals.var_t15_dn8) + ((locals.var_ub_dn8 * locals.var_t3__blk811) + (locals.var_ub * locals.var_t3__blk811_dn8))))), ((locals.var_t3__blk811_dn9 * assign19090_e17146) + (locals.var_t3__blk811 * ((locals.var_t2__blk810_dn9 + locals.var_t15_dn9) + ((locals.var_ub_dn9 * locals.var_t3__blk811) + (locals.var_ub * locals.var_t3__blk811_dn9))))), ((locals.var_t3__blk811_dn10 * assign19090_e17146) + (locals.var_t3__blk811 * ((locals.var_t2__blk810_dn10 + locals.var_t15_dn10) + ((locals.var_ub_dn10 * locals.var_t3__blk811) + (locals.var_ub * locals.var_t3__blk811_dn10))))), ((locals.var_t3__blk811_dn11 * assign19090_e17146) + (locals.var_t3__blk811 * ((locals.var_t2__blk810_dn11 + locals.var_t15_dn11) + ((locals.var_ub_dn11 * locals.var_t3__blk811) + (locals.var_ub * locals.var_t3__blk811_dn11))))), ((locals.var_t3__blk811_dn12 * assign19090_e17146) + (locals.var_t3__blk811 * ((locals.var_t2__blk810_dn12 + locals.var_t15_dn12) + ((locals.var_ub_dn12 * locals.var_t3__blk811) + (locals.var_ub * locals.var_t3__blk811_dn12))))),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign19090_e17149;
        locals.var_t5__blk813_dn3 = assign19090_e17149_d_n3;
        locals.var_t5__blk813_dn4 = assign19090_e17149_d_n4;
        locals.var_t5__blk813_dn5 = assign19090_e17149_d_n5;
        locals.var_t5__blk813_dn6 = assign19090_e17149_d_n6;
        locals.var_t5__blk813_dn7 = assign19090_e17149_d_n7;
        locals.var_t5__blk813_dn8 = assign19090_e17149_d_n8;
        locals.var_t5__blk813_dn9 = assign19090_e17149_d_n9;
        locals.var_t5__blk813_dn10 = assign19090_e17149_d_n10;
        locals.var_t5__blk813_dn11 = assign19090_e17149_d_n11;
        locals.var_t5__blk813_dn12 = assign19090_e17149_d_n12;
        locals.var_t5__blk813_rv = 0.0;

        let assign19100_e17152: f64 = if p.p62 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1226 = assign19100_e17152;
        locals.var_guard1226_rv = 0.0;

        let (assign19110_e17179, assign19110_e17179_d_n3, assign19110_e17179_d_n4, assign19110_e17179_d_n5, assign19110_e17179_d_n6, assign19110_e17179_d_n7, assign19110_e17179_d_n8, assign19110_e17179_d_n9, assign19110_e17179_d_n10, assign19110_e17179_d_n11, assign19110_e17179_d_n12,) = {
    if ((locals.var_guard1225 == 0.0) && (locals.var_guard1226 != 0.0)) {
        let assign19110_e17159: f64 = (locals.var_vgsteff__blk840 - locals.var_t14);
        let assign19110_e17161: f64 = (assign19110_e17159 / locals.var_toxe);
        let assign19110_e17165: f64 = (locals.var_uc * locals.var_vbseff);
        let assign19110_e17166: f64 = (locals.var_ua + assign19110_e17165);
        let assign19110_e17168: f64 = (assign19110_e17166 + locals.var_t15);
        let assign19110_e17172: f64 = (locals.var_vgsteff__blk840 - locals.var_t14);
        let assign19110_e17173: f64 = (locals.var_ub * assign19110_e17172);
        let assign19110_e17175: f64 = (assign19110_e17173 / locals.var_toxe);
        let assign19110_e17176: f64 = (assign19110_e17168 + assign19110_e17175);
        let assign19110_e17177: f64 = (assign19110_e17161 * assign19110_e17176);
        (assign19110_e17177, ((((locals.var_vgsteff__blk840_dn3 - locals.var_t14_dn3) / locals.var_toxe) * assign19110_e17176) + (assign19110_e17161 * (((locals.var_ua_dn3 + ((locals.var_uc_dn3 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn3))) + locals.var_t15_dn3) + (((locals.var_ub_dn3 * assign19110_e17172) + (locals.var_ub * (locals.var_vgsteff__blk840_dn3 - locals.var_t14_dn3))) / locals.var_toxe)))), ((((locals.var_vgsteff__blk840_dn4 - locals.var_t14_dn4) / locals.var_toxe) * assign19110_e17176) + (assign19110_e17161 * (((locals.var_ua_dn4 + ((locals.var_uc_dn4 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn4))) + locals.var_t15_dn4) + (((locals.var_ub_dn4 * assign19110_e17172) + (locals.var_ub * (locals.var_vgsteff__blk840_dn4 - locals.var_t14_dn4))) / locals.var_toxe)))), ((((locals.var_vgsteff__blk840_dn5 - locals.var_t14_dn5) / locals.var_toxe) * assign19110_e17176) + (assign19110_e17161 * (((locals.var_ua_dn5 + ((locals.var_uc_dn5 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn5))) + locals.var_t15_dn5) + (((locals.var_ub_dn5 * assign19110_e17172) + (locals.var_ub * (locals.var_vgsteff__blk840_dn5 - locals.var_t14_dn5))) / locals.var_toxe)))), ((((locals.var_vgsteff__blk840_dn6 - locals.var_t14_dn6) / locals.var_toxe) * assign19110_e17176) + (assign19110_e17161 * (((locals.var_ua_dn6 + ((locals.var_uc_dn6 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn6))) + locals.var_t15_dn6) + (((locals.var_ub_dn6 * assign19110_e17172) + (locals.var_ub * (locals.var_vgsteff__blk840_dn6 - locals.var_t14_dn6))) / locals.var_toxe)))), ((((locals.var_vgsteff__blk840_dn7 - locals.var_t14_dn7) / locals.var_toxe) * assign19110_e17176) + (assign19110_e17161 * (((locals.var_ua_dn7 + ((locals.var_uc_dn7 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn7))) + locals.var_t15_dn7) + (((locals.var_ub_dn7 * assign19110_e17172) + (locals.var_ub * (locals.var_vgsteff__blk840_dn7 - locals.var_t14_dn7))) / locals.var_toxe)))), ((((locals.var_vgsteff__blk840_dn8 - locals.var_t14_dn8) / locals.var_toxe) * assign19110_e17176) + (assign19110_e17161 * (((locals.var_ua_dn8 + ((locals.var_uc_dn8 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn8))) + locals.var_t15_dn8) + (((locals.var_ub_dn8 * assign19110_e17172) + (locals.var_ub * (locals.var_vgsteff__blk840_dn8 - locals.var_t14_dn8))) / locals.var_toxe)))), ((((locals.var_vgsteff__blk840_dn9 - locals.var_t14_dn9) / locals.var_toxe) * assign19110_e17176) + (assign19110_e17161 * (((locals.var_ua_dn9 + ((locals.var_uc_dn9 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn9))) + locals.var_t15_dn9) + (((locals.var_ub_dn9 * assign19110_e17172) + (locals.var_ub * (locals.var_vgsteff__blk840_dn9 - locals.var_t14_dn9))) / locals.var_toxe)))), ((((locals.var_vgsteff__blk840_dn10 - locals.var_t14_dn10) / locals.var_toxe) * assign19110_e17176) + (assign19110_e17161 * (((locals.var_ua_dn10 + ((locals.var_uc_dn10 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn10))) + locals.var_t15_dn10) + (((locals.var_ub_dn10 * assign19110_e17172) + (locals.var_ub * (locals.var_vgsteff__blk840_dn10 - locals.var_t14_dn10))) / locals.var_toxe)))), ((((locals.var_vgsteff__blk840_dn11 - locals.var_t14_dn11) / locals.var_toxe) * assign19110_e17176) + (assign19110_e17161 * (((locals.var_ua_dn11 + ((locals.var_uc_dn11 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn11))) + locals.var_t15_dn11) + (((locals.var_ub_dn11 * assign19110_e17172) + (locals.var_ub * (locals.var_vgsteff__blk840_dn11 - locals.var_t14_dn11))) / locals.var_toxe)))), ((((locals.var_vgsteff__blk840_dn12 - locals.var_t14_dn12) / locals.var_toxe) * assign19110_e17176) + (assign19110_e17161 * (((locals.var_ua_dn12 + ((locals.var_uc_dn12 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn12))) + locals.var_t15_dn12) + (((locals.var_ub_dn12 * assign19110_e17172) + (locals.var_ub * (locals.var_vgsteff__blk840_dn12 - locals.var_t14_dn12))) / locals.var_toxe)))),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign19110_e17179;
        locals.var_t5__blk813_dn3 = assign19110_e17179_d_n3;
        locals.var_t5__blk813_dn4 = assign19110_e17179_d_n4;
        locals.var_t5__blk813_dn5 = assign19110_e17179_d_n5;
        locals.var_t5__blk813_dn6 = assign19110_e17179_d_n6;
        locals.var_t5__blk813_dn7 = assign19110_e17179_d_n7;
        locals.var_t5__blk813_dn8 = assign19110_e17179_d_n8;
        locals.var_t5__blk813_dn9 = assign19110_e17179_d_n9;
        locals.var_t5__blk813_dn10 = assign19110_e17179_d_n10;
        locals.var_t5__blk813_dn11 = assign19110_e17179_d_n11;
        locals.var_t5__blk813_dn12 = assign19110_e17179_d_n12;
        locals.var_t5__blk813_rv = 0.0;

        let assign19120_e17182: f64 = if p.p62 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard1227 = assign19120_e17182;
        locals.var_guard1227_rv = 0.0;

        let (assign19130_e17198, assign19130_e17198_d_n3, assign19130_e17198_d_n4, assign19130_e17198_d_n5, assign19130_e17198_d_n6, assign19130_e17198_d_n7, assign19130_e17198_d_n8, assign19130_e17198_d_n9, assign19130_e17198_d_n10, assign19130_e17198_d_n11, assign19130_e17198_d_n12,) = {
    if (((locals.var_guard1225 == 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1227 != 0.0)) {
        let assign19130_e17192: f64 = (locals.var_vgsteff__blk840 + locals.var_vth__blk794);
        let assign19130_e17194: f64 = (assign19130_e17192 + locals.var_vth__blk794);
        let assign19130_e17196: f64 = (assign19130_e17194 - locals.var_t14);
        (assign19130_e17196, (((locals.var_vgsteff__blk840_dn3 + locals.var_vth__blk794_dn3) + locals.var_vth__blk794_dn3) - locals.var_t14_dn3), (((locals.var_vgsteff__blk840_dn4 + locals.var_vth__blk794_dn4) + locals.var_vth__blk794_dn4) - locals.var_t14_dn4), (((locals.var_vgsteff__blk840_dn5 + locals.var_vth__blk794_dn5) + locals.var_vth__blk794_dn5) - locals.var_t14_dn5), (((locals.var_vgsteff__blk840_dn6 + locals.var_vth__blk794_dn6) + locals.var_vth__blk794_dn6) - locals.var_t14_dn6), (((locals.var_vgsteff__blk840_dn7 + locals.var_vth__blk794_dn7) + locals.var_vth__blk794_dn7) - locals.var_t14_dn7), (((locals.var_vgsteff__blk840_dn8 + locals.var_vth__blk794_dn8) + locals.var_vth__blk794_dn8) - locals.var_t14_dn8), (((locals.var_vgsteff__blk840_dn9 + locals.var_vth__blk794_dn9) + locals.var_vth__blk794_dn9) - locals.var_t14_dn9), (((locals.var_vgsteff__blk840_dn10 + locals.var_vth__blk794_dn10) + locals.var_vth__blk794_dn10) - locals.var_t14_dn10), (((locals.var_vgsteff__blk840_dn11 + locals.var_vth__blk794_dn11) + locals.var_vth__blk794_dn11) - locals.var_t14_dn11), (((locals.var_vgsteff__blk840_dn12 + locals.var_vth__blk794_dn12) + locals.var_vth__blk794_dn12) - locals.var_t14_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign19130_e17198;
        locals.var_t0__blk808_dn3 = assign19130_e17198_d_n3;
        locals.var_t0__blk808_dn4 = assign19130_e17198_d_n4;
        locals.var_t0__blk808_dn5 = assign19130_e17198_d_n5;
        locals.var_t0__blk808_dn6 = assign19130_e17198_d_n6;
        locals.var_t0__blk808_dn7 = assign19130_e17198_d_n7;
        locals.var_t0__blk808_dn8 = assign19130_e17198_d_n8;
        locals.var_t0__blk808_dn9 = assign19130_e17198_d_n9;
        locals.var_t0__blk808_dn10 = assign19130_e17198_d_n10;
        locals.var_t0__blk808_dn11 = assign19130_e17198_d_n11;
        locals.var_t0__blk808_dn12 = assign19130_e17198_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign19140_e17212, assign19140_e17212_d_n3, assign19140_e17212_d_n4, assign19140_e17212_d_n5, assign19140_e17212_d_n6, assign19140_e17212_d_n7, assign19140_e17212_d_n8, assign19140_e17212_d_n9, assign19140_e17212_d_n10, assign19140_e17212_d_n11, assign19140_e17212_d_n12,) = {
    if (((locals.var_guard1225 == 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1227 != 0.0)) {
        let assign19140_e17209: f64 = (locals.var_uc * locals.var_vbseff);
        let assign19140_e17210: f64 = (1.0 + assign19140_e17209);
        (assign19140_e17210, ((locals.var_uc_dn3 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn3)), ((locals.var_uc_dn4 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn4)), ((locals.var_uc_dn5 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn5)), ((locals.var_uc_dn6 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn6)), ((locals.var_uc_dn7 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn7)), ((locals.var_uc_dn8 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn8)), ((locals.var_uc_dn9 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn9)), ((locals.var_uc_dn10 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn10)), ((locals.var_uc_dn11 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn11)), ((locals.var_uc_dn12 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn12)),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign19140_e17212;
        locals.var_t2__blk810_dn3 = assign19140_e17212_d_n3;
        locals.var_t2__blk810_dn4 = assign19140_e17212_d_n4;
        locals.var_t2__blk810_dn5 = assign19140_e17212_d_n5;
        locals.var_t2__blk810_dn6 = assign19140_e17212_d_n6;
        locals.var_t2__blk810_dn7 = assign19140_e17212_d_n7;
        locals.var_t2__blk810_dn8 = assign19140_e17212_d_n8;
        locals.var_t2__blk810_dn9 = assign19140_e17212_d_n9;
        locals.var_t2__blk810_dn10 = assign19140_e17212_d_n10;
        locals.var_t2__blk810_dn11 = assign19140_e17212_d_n11;
        locals.var_t2__blk810_dn12 = assign19140_e17212_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign19150_e17224, assign19150_e17224_d_n3, assign19150_e17224_d_n4, assign19150_e17224_d_n5, assign19150_e17224_d_n6, assign19150_e17224_d_n7, assign19150_e17224_d_n8, assign19150_e17224_d_n9, assign19150_e17224_d_n10, assign19150_e17224_d_n11, assign19150_e17224_d_n12,) = {
    if (((locals.var_guard1225 == 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1227 != 0.0)) {
        let assign19150_e17222: f64 = (locals.var_t0__blk808 / locals.var_toxe_mob);
        (assign19150_e17222, (locals.var_t0__blk808_dn3 / locals.var_toxe_mob), (locals.var_t0__blk808_dn4 / locals.var_toxe_mob), (locals.var_t0__blk808_dn5 / locals.var_toxe_mob), (locals.var_t0__blk808_dn6 / locals.var_toxe_mob), (locals.var_t0__blk808_dn7 / locals.var_toxe_mob), (locals.var_t0__blk808_dn8 / locals.var_toxe_mob), (locals.var_t0__blk808_dn9 / locals.var_toxe_mob), (locals.var_t0__blk808_dn10 / locals.var_toxe_mob), (locals.var_t0__blk808_dn11 / locals.var_toxe_mob), (locals.var_t0__blk808_dn12 / locals.var_toxe_mob),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign19150_e17224;
        locals.var_t3__blk811_dn3 = assign19150_e17224_d_n3;
        locals.var_t3__blk811_dn4 = assign19150_e17224_d_n4;
        locals.var_t3__blk811_dn5 = assign19150_e17224_d_n5;
        locals.var_t3__blk811_dn6 = assign19150_e17224_d_n6;
        locals.var_t3__blk811_dn7 = assign19150_e17224_d_n7;
        locals.var_t3__blk811_dn8 = assign19150_e17224_d_n8;
        locals.var_t3__blk811_dn9 = assign19150_e17224_d_n9;
        locals.var_t3__blk811_dn10 = assign19150_e17224_d_n10;
        locals.var_t3__blk811_dn11 = assign19150_e17224_d_n11;
        locals.var_t3__blk811_dn12 = assign19150_e17224_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign19160_e17240, assign19160_e17240_d_n3, assign19160_e17240_d_n4, assign19160_e17240_d_n5, assign19160_e17240_d_n6, assign19160_e17240_d_n7, assign19160_e17240_d_n8, assign19160_e17240_d_n9, assign19160_e17240_d_n10, assign19160_e17240_d_n11, assign19160_e17240_d_n12,) = {
    if (((locals.var_guard1225 == 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1227 != 0.0)) {
        let assign19160_e17236: f64 = (locals.var_ub * locals.var_t3__blk811);
        let assign19160_e17237: f64 = (locals.var_ua + assign19160_e17236);
        let assign19160_e17238: f64 = (locals.var_t3__blk811 * assign19160_e17237);
        (assign19160_e17238, ((locals.var_t3__blk811_dn3 * assign19160_e17237) + (locals.var_t3__blk811 * (locals.var_ua_dn3 + ((locals.var_ub_dn3 * locals.var_t3__blk811) + (locals.var_ub * locals.var_t3__blk811_dn3))))), ((locals.var_t3__blk811_dn4 * assign19160_e17237) + (locals.var_t3__blk811 * (locals.var_ua_dn4 + ((locals.var_ub_dn4 * locals.var_t3__blk811) + (locals.var_ub * locals.var_t3__blk811_dn4))))), ((locals.var_t3__blk811_dn5 * assign19160_e17237) + (locals.var_t3__blk811 * (locals.var_ua_dn5 + ((locals.var_ub_dn5 * locals.var_t3__blk811) + (locals.var_ub * locals.var_t3__blk811_dn5))))), ((locals.var_t3__blk811_dn6 * assign19160_e17237) + (locals.var_t3__blk811 * (locals.var_ua_dn6 + ((locals.var_ub_dn6 * locals.var_t3__blk811) + (locals.var_ub * locals.var_t3__blk811_dn6))))), ((locals.var_t3__blk811_dn7 * assign19160_e17237) + (locals.var_t3__blk811 * (locals.var_ua_dn7 + ((locals.var_ub_dn7 * locals.var_t3__blk811) + (locals.var_ub * locals.var_t3__blk811_dn7))))), ((locals.var_t3__blk811_dn8 * assign19160_e17237) + (locals.var_t3__blk811 * (locals.var_ua_dn8 + ((locals.var_ub_dn8 * locals.var_t3__blk811) + (locals.var_ub * locals.var_t3__blk811_dn8))))), ((locals.var_t3__blk811_dn9 * assign19160_e17237) + (locals.var_t3__blk811 * (locals.var_ua_dn9 + ((locals.var_ub_dn9 * locals.var_t3__blk811) + (locals.var_ub * locals.var_t3__blk811_dn9))))), ((locals.var_t3__blk811_dn10 * assign19160_e17237) + (locals.var_t3__blk811 * (locals.var_ua_dn10 + ((locals.var_ub_dn10 * locals.var_t3__blk811) + (locals.var_ub * locals.var_t3__blk811_dn10))))), ((locals.var_t3__blk811_dn11 * assign19160_e17237) + (locals.var_t3__blk811 * (locals.var_ua_dn11 + ((locals.var_ub_dn11 * locals.var_t3__blk811) + (locals.var_ub * locals.var_t3__blk811_dn11))))), ((locals.var_t3__blk811_dn12 * assign19160_e17237) + (locals.var_t3__blk811 * (locals.var_ua_dn12 + ((locals.var_ub_dn12 * locals.var_t3__blk811) + (locals.var_ub * locals.var_t3__blk811_dn12))))),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign19160_e17240;
        locals.var_t4__blk812_dn3 = assign19160_e17240_d_n3;
        locals.var_t4__blk812_dn4 = assign19160_e17240_d_n4;
        locals.var_t4__blk812_dn5 = assign19160_e17240_d_n5;
        locals.var_t4__blk812_dn6 = assign19160_e17240_d_n6;
        locals.var_t4__blk812_dn7 = assign19160_e17240_d_n7;
        locals.var_t4__blk812_dn8 = assign19160_e17240_d_n8;
        locals.var_t4__blk812_dn9 = assign19160_e17240_d_n9;
        locals.var_t4__blk812_dn10 = assign19160_e17240_d_n10;
        locals.var_t4__blk812_dn11 = assign19160_e17240_d_n11;
        locals.var_t4__blk812_dn12 = assign19160_e17240_d_n12;
        locals.var_t4__blk812_rv = 0.0;

        let (assign19170_e17252, assign19170_e17252_d_n3, assign19170_e17252_d_n4, assign19170_e17252_d_n5, assign19170_e17252_d_n6, assign19170_e17252_d_n7, assign19170_e17252_d_n8, assign19170_e17252_d_n9, assign19170_e17252_d_n10, assign19170_e17252_d_n11, assign19170_e17252_d_n12,) = {
    if (((locals.var_guard1225 == 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1227 != 0.0)) {
        let assign19170_e17250: f64 = (locals.var_t4__blk812 * locals.var_t2__blk810);
        (assign19170_e17250, ((locals.var_t4__blk812_dn3 * locals.var_t2__blk810) + (locals.var_t4__blk812 * locals.var_t2__blk810_dn3)), ((locals.var_t4__blk812_dn4 * locals.var_t2__blk810) + (locals.var_t4__blk812 * locals.var_t2__blk810_dn4)), ((locals.var_t4__blk812_dn5 * locals.var_t2__blk810) + (locals.var_t4__blk812 * locals.var_t2__blk810_dn5)), ((locals.var_t4__blk812_dn6 * locals.var_t2__blk810) + (locals.var_t4__blk812 * locals.var_t2__blk810_dn6)), ((locals.var_t4__blk812_dn7 * locals.var_t2__blk810) + (locals.var_t4__blk812 * locals.var_t2__blk810_dn7)), ((locals.var_t4__blk812_dn8 * locals.var_t2__blk810) + (locals.var_t4__blk812 * locals.var_t2__blk810_dn8)), ((locals.var_t4__blk812_dn9 * locals.var_t2__blk810) + (locals.var_t4__blk812 * locals.var_t2__blk810_dn9)), ((locals.var_t4__blk812_dn10 * locals.var_t2__blk810) + (locals.var_t4__blk812 * locals.var_t2__blk810_dn10)), ((locals.var_t4__blk812_dn11 * locals.var_t2__blk810) + (locals.var_t4__blk812 * locals.var_t2__blk810_dn11)), ((locals.var_t4__blk812_dn12 * locals.var_t2__blk810) + (locals.var_t4__blk812 * locals.var_t2__blk810_dn12)),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign19170_e17252;
        locals.var_t5__blk813_dn3 = assign19170_e17252_d_n3;
        locals.var_t5__blk813_dn4 = assign19170_e17252_d_n4;
        locals.var_t5__blk813_dn5 = assign19170_e17252_d_n5;
        locals.var_t5__blk813_dn6 = assign19170_e17252_d_n6;
        locals.var_t5__blk813_dn7 = assign19170_e17252_d_n7;
        locals.var_t5__blk813_dn8 = assign19170_e17252_d_n8;
        locals.var_t5__blk813_dn9 = assign19170_e17252_d_n9;
        locals.var_t5__blk813_dn10 = assign19170_e17252_d_n10;
        locals.var_t5__blk813_dn11 = assign19170_e17252_d_n11;
        locals.var_t5__blk813_dn12 = assign19170_e17252_d_n12;
        locals.var_t5__blk813_rv = 0.0;

        let (assign19180_e17271, assign19180_e17271_d_n3, assign19180_e17271_d_n4, assign19180_e17271_d_n5, assign19180_e17271_d_n6, assign19180_e17271_d_n7, assign19180_e17271_d_n8, assign19180_e17271_d_n9, assign19180_e17271_d_n10, assign19180_e17271_d_n11, assign19180_e17271_d_n12,) = {
    if (((locals.var_guard1225 == 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1227 == 0.0)) {
        let assign19180_e17263: f64 = (locals.var_vgsteff__blk840 + locals.var_b4soivtfbphi1);
        let assign19180_e17265: f64 = (assign19180_e17263 * 1e-8);
        let assign19180_e17267: f64 = (assign19180_e17265 / locals.var_toxe);
        let assign19180_e17269: f64 = (assign19180_e17267 / 6.0);
        (assign19180_e17269, ((((locals.var_vgsteff__blk840_dn3 + locals.var_b4soivtfbphi1_dn3) * 1e-8) / locals.var_toxe) / 6.0), ((((locals.var_vgsteff__blk840_dn4 + locals.var_b4soivtfbphi1_dn4) * 1e-8) / locals.var_toxe) / 6.0), ((((locals.var_vgsteff__blk840_dn5 + locals.var_b4soivtfbphi1_dn5) * 1e-8) / locals.var_toxe) / 6.0), ((((locals.var_vgsteff__blk840_dn6 + locals.var_b4soivtfbphi1_dn6) * 1e-8) / locals.var_toxe) / 6.0), ((((locals.var_vgsteff__blk840_dn7 + locals.var_b4soivtfbphi1_dn7) * 1e-8) / locals.var_toxe) / 6.0), ((((locals.var_vgsteff__blk840_dn8 + locals.var_b4soivtfbphi1_dn8) * 1e-8) / locals.var_toxe) / 6.0), ((((locals.var_vgsteff__blk840_dn9 + locals.var_b4soivtfbphi1_dn9) * 1e-8) / locals.var_toxe) / 6.0), ((((locals.var_vgsteff__blk840_dn10 + locals.var_b4soivtfbphi1_dn10) * 1e-8) / locals.var_toxe) / 6.0), ((((locals.var_vgsteff__blk840_dn11 + locals.var_b4soivtfbphi1_dn11) * 1e-8) / locals.var_toxe) / 6.0), ((((locals.var_vgsteff__blk840_dn12 + locals.var_b4soivtfbphi1_dn12) * 1e-8) / locals.var_toxe) / 6.0),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign19180_e17271;
        locals.var_t0__blk808_dn3 = assign19180_e17271_d_n3;
        locals.var_t0__blk808_dn4 = assign19180_e17271_d_n4;
        locals.var_t0__blk808_dn5 = assign19180_e17271_d_n5;
        locals.var_t0__blk808_dn6 = assign19180_e17271_d_n6;
        locals.var_t0__blk808_dn7 = assign19180_e17271_d_n7;
        locals.var_t0__blk808_dn8 = assign19180_e17271_d_n8;
        locals.var_t0__blk808_dn9 = assign19180_e17271_d_n9;
        locals.var_t0__blk808_dn10 = assign19180_e17271_d_n10;
        locals.var_t0__blk808_dn11 = assign19180_e17271_d_n11;
        locals.var_t0__blk808_dn12 = assign19180_e17271_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign19190_e17292, assign19190_e17292_d_n3, assign19190_e17292_d_n4, assign19190_e17292_d_n5, assign19190_e17292_d_n6, assign19190_e17292_d_n7, assign19190_e17292_d_n8, assign19190_e17292_d_n9, assign19190_e17292_d_n10, assign19190_e17292_d_n11, assign19190_e17292_d_n12,) = {
    if (((locals.var_guard1225 == 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1227 == 0.0)) {
        let (assign19190_e17288, assign19190_e17288_d_n3, assign19190_e17288_d_n4, assign19190_e17288_d_n5, assign19190_e17288_d_n6, assign19190_e17288_d_n7, assign19190_e17288_d_n8, assign19190_e17288_d_n9, assign19190_e17288_d_n10, assign19190_e17288_d_n11, assign19190_e17288_d_n12,) = {
            if (locals.var_t0__blk808 > 1e-38) {
                let assign19190_e17285: f64 = (locals.var_t0__blk808).ln();
                (assign19190_e17285, (locals.var_t0__blk808_dn3 / locals.var_t0__blk808), (locals.var_t0__blk808_dn4 / locals.var_t0__blk808), (locals.var_t0__blk808_dn5 / locals.var_t0__blk808), (locals.var_t0__blk808_dn6 / locals.var_t0__blk808), (locals.var_t0__blk808_dn7 / locals.var_t0__blk808), (locals.var_t0__blk808_dn8 / locals.var_t0__blk808), (locals.var_t0__blk808_dn9 / locals.var_t0__blk808), (locals.var_t0__blk808_dn10 / locals.var_t0__blk808), (locals.var_t0__blk808_dn11 / locals.var_t0__blk808), (locals.var_t0__blk808_dn12 / locals.var_t0__blk808),)
            } else {
                let assign19190_e17287: f64 = (-87.49823353377374);
                (assign19190_e17287, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign19190_e17289: f64 = (locals.var_pparam_b4soieu * assign19190_e17288);
        let assign19190_e17290: f64 = (assign19190_e17289).exp();
        (assign19190_e17290, (assign19190_e17290 * ((locals.var_pparam_b4soieu_dn3 * assign19190_e17288) + (locals.var_pparam_b4soieu * assign19190_e17288_d_n3))), (assign19190_e17290 * ((locals.var_pparam_b4soieu_dn4 * assign19190_e17288) + (locals.var_pparam_b4soieu * assign19190_e17288_d_n4))), (assign19190_e17290 * ((locals.var_pparam_b4soieu_dn5 * assign19190_e17288) + (locals.var_pparam_b4soieu * assign19190_e17288_d_n5))), (assign19190_e17290 * ((locals.var_pparam_b4soieu_dn6 * assign19190_e17288) + (locals.var_pparam_b4soieu * assign19190_e17288_d_n6))), (assign19190_e17290 * ((locals.var_pparam_b4soieu_dn7 * assign19190_e17288) + (locals.var_pparam_b4soieu * assign19190_e17288_d_n7))), (assign19190_e17290 * ((locals.var_pparam_b4soieu_dn8 * assign19190_e17288) + (locals.var_pparam_b4soieu * assign19190_e17288_d_n8))), (assign19190_e17290 * ((locals.var_pparam_b4soieu_dn9 * assign19190_e17288) + (locals.var_pparam_b4soieu * assign19190_e17288_d_n9))), (assign19190_e17290 * ((locals.var_pparam_b4soieu_dn10 * assign19190_e17288) + (locals.var_pparam_b4soieu * assign19190_e17288_d_n10))), (assign19190_e17290 * ((locals.var_pparam_b4soieu_dn11 * assign19190_e17288) + (locals.var_pparam_b4soieu * assign19190_e17288_d_n11))), (assign19190_e17290 * ((locals.var_pparam_b4soieu_dn12 * assign19190_e17288) + (locals.var_pparam_b4soieu * assign19190_e17288_d_n12))),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign19190_e17292;
        locals.var_t1__blk809_dn3 = assign19190_e17292_d_n3;
        locals.var_t1__blk809_dn4 = assign19190_e17292_d_n4;
        locals.var_t1__blk809_dn5 = assign19190_e17292_d_n5;
        locals.var_t1__blk809_dn6 = assign19190_e17292_d_n6;
        locals.var_t1__blk809_dn7 = assign19190_e17292_d_n7;
        locals.var_t1__blk809_dn8 = assign19190_e17292_d_n8;
        locals.var_t1__blk809_dn9 = assign19190_e17292_d_n9;
        locals.var_t1__blk809_dn10 = assign19190_e17292_d_n10;
        locals.var_t1__blk809_dn11 = assign19190_e17292_d_n11;
        locals.var_t1__blk809_dn12 = assign19190_e17292_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign19200_e17307, assign19200_e17307_d_n3, assign19200_e17307_d_n4, assign19200_e17307_d_n5, assign19200_e17307_d_n6, assign19200_e17307_d_n7, assign19200_e17307_d_n8, assign19200_e17307_d_n9, assign19200_e17307_d_n10, assign19200_e17307_d_n11, assign19200_e17307_d_n12,) = {
    if (((locals.var_guard1225 == 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1227 == 0.0)) {
        let assign19200_e17304: f64 = (locals.var_uc * locals.var_vbseff);
        let assign19200_e17305: f64 = (locals.var_ua + assign19200_e17304);
        (assign19200_e17305, (locals.var_ua_dn3 + ((locals.var_uc_dn3 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn3))), (locals.var_ua_dn4 + ((locals.var_uc_dn4 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn4))), (locals.var_ua_dn5 + ((locals.var_uc_dn5 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn5))), (locals.var_ua_dn6 + ((locals.var_uc_dn6 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn6))), (locals.var_ua_dn7 + ((locals.var_uc_dn7 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn7))), (locals.var_ua_dn8 + ((locals.var_uc_dn8 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn8))), (locals.var_ua_dn9 + ((locals.var_uc_dn9 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn9))), (locals.var_ua_dn10 + ((locals.var_uc_dn10 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn10))), (locals.var_ua_dn11 + ((locals.var_uc_dn11 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn11))), (locals.var_ua_dn12 + ((locals.var_uc_dn12 * locals.var_vbseff) + (locals.var_uc * locals.var_vbseff_dn12))),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign19200_e17307;
        locals.var_t2__blk810_dn3 = assign19200_e17307_d_n3;
        locals.var_t2__blk810_dn4 = assign19200_e17307_d_n4;
        locals.var_t2__blk810_dn5 = assign19200_e17307_d_n5;
        locals.var_t2__blk810_dn6 = assign19200_e17307_d_n6;
        locals.var_t2__blk810_dn7 = assign19200_e17307_d_n7;
        locals.var_t2__blk810_dn8 = assign19200_e17307_d_n8;
        locals.var_t2__blk810_dn9 = assign19200_e17307_d_n9;
        locals.var_t2__blk810_dn10 = assign19200_e17307_d_n10;
        locals.var_t2__blk810_dn11 = assign19200_e17307_d_n11;
        locals.var_t2__blk810_dn12 = assign19200_e17307_d_n12;
        locals.var_t2__blk810_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_58(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19210_e17322, assign19210_e17322_d_n3, assign19210_e17322_d_n4, assign19210_e17322_d_n5, assign19210_e17322_d_n6, assign19210_e17322_d_n7, assign19210_e17322_d_n8, assign19210_e17322_d_n9, assign19210_e17322_d_n10, assign19210_e17322_d_n11, assign19210_e17322_d_n12,) = {
    if (((locals.var_guard1225 == 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1227 == 0.0)) {
        let assign19210_e17319: f64 = (locals.var_tempratio).powf(locals.var_pparam_b4soiucste);
        let assign19210_e17320: f64 = (locals.var_pparam_b4soiucs * assign19210_e17319);
        (assign19210_e17320, ((locals.var_pparam_b4soiucs_dn3 * assign19210_e17319) + (locals.var_pparam_b4soiucs * if locals.var_pparam_b4soiucste_dn3 == 0.0 && ((locals.var_pparam_b4soiucste) as f64).is_finite() && ((locals.var_pparam_b4soiucste) as f64).fract() == 0.0 { 0.0 } else { (assign19210_e17319 * (locals.var_pparam_b4soiucste_dn3 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiucs_dn4 * assign19210_e17319) + (locals.var_pparam_b4soiucs * if locals.var_pparam_b4soiucste_dn4 == 0.0 && ((locals.var_pparam_b4soiucste) as f64).is_finite() && ((locals.var_pparam_b4soiucste) as f64).fract() == 0.0 { if locals.var_pparam_b4soiucste == 0.0 { 0.0 } else { (locals.var_pparam_b4soiucste * ((locals.var_tempratio).powf(locals.var_pparam_b4soiucste - 1.0) * locals.var_tempratio_dn4)) } } else { (assign19210_e17319 * ((locals.var_pparam_b4soiucste_dn4 * (locals.var_tempratio).ln()) + (locals.var_pparam_b4soiucste * (locals.var_tempratio_dn4 / locals.var_tempratio)))) })), ((locals.var_pparam_b4soiucs_dn5 * assign19210_e17319) + (locals.var_pparam_b4soiucs * if locals.var_pparam_b4soiucste_dn5 == 0.0 && ((locals.var_pparam_b4soiucste) as f64).is_finite() && ((locals.var_pparam_b4soiucste) as f64).fract() == 0.0 { if locals.var_pparam_b4soiucste == 0.0 { 0.0 } else { (locals.var_pparam_b4soiucste * ((locals.var_tempratio).powf(locals.var_pparam_b4soiucste - 1.0) * locals.var_tempratio_dn5)) } } else { (assign19210_e17319 * ((locals.var_pparam_b4soiucste_dn5 * (locals.var_tempratio).ln()) + (locals.var_pparam_b4soiucste * (locals.var_tempratio_dn5 / locals.var_tempratio)))) })), ((locals.var_pparam_b4soiucs_dn6 * assign19210_e17319) + (locals.var_pparam_b4soiucs * if locals.var_pparam_b4soiucste_dn6 == 0.0 && ((locals.var_pparam_b4soiucste) as f64).is_finite() && ((locals.var_pparam_b4soiucste) as f64).fract() == 0.0 { if locals.var_pparam_b4soiucste == 0.0 { 0.0 } else { (locals.var_pparam_b4soiucste * ((locals.var_tempratio).powf(locals.var_pparam_b4soiucste - 1.0) * locals.var_tempratio_dn6)) } } else { (assign19210_e17319 * ((locals.var_pparam_b4soiucste_dn6 * (locals.var_tempratio).ln()) + (locals.var_pparam_b4soiucste * (locals.var_tempratio_dn6 / locals.var_tempratio)))) })), ((locals.var_pparam_b4soiucs_dn7 * assign19210_e17319) + (locals.var_pparam_b4soiucs * if locals.var_pparam_b4soiucste_dn7 == 0.0 && ((locals.var_pparam_b4soiucste) as f64).is_finite() && ((locals.var_pparam_b4soiucste) as f64).fract() == 0.0 { 0.0 } else { (assign19210_e17319 * (locals.var_pparam_b4soiucste_dn7 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiucs_dn8 * assign19210_e17319) + (locals.var_pparam_b4soiucs * if locals.var_pparam_b4soiucste_dn8 == 0.0 && ((locals.var_pparam_b4soiucste) as f64).is_finite() && ((locals.var_pparam_b4soiucste) as f64).fract() == 0.0 { 0.0 } else { (assign19210_e17319 * (locals.var_pparam_b4soiucste_dn8 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiucs_dn9 * assign19210_e17319) + (locals.var_pparam_b4soiucs * if locals.var_pparam_b4soiucste_dn9 == 0.0 && ((locals.var_pparam_b4soiucste) as f64).is_finite() && ((locals.var_pparam_b4soiucste) as f64).fract() == 0.0 { 0.0 } else { (assign19210_e17319 * (locals.var_pparam_b4soiucste_dn9 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiucs_dn10 * assign19210_e17319) + (locals.var_pparam_b4soiucs * if locals.var_pparam_b4soiucste_dn10 == 0.0 && ((locals.var_pparam_b4soiucste) as f64).is_finite() && ((locals.var_pparam_b4soiucste) as f64).fract() == 0.0 { 0.0 } else { (assign19210_e17319 * (locals.var_pparam_b4soiucste_dn10 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiucs_dn11 * assign19210_e17319) + (locals.var_pparam_b4soiucs * if locals.var_pparam_b4soiucste_dn11 == 0.0 && ((locals.var_pparam_b4soiucste) as f64).is_finite() && ((locals.var_pparam_b4soiucste) as f64).fract() == 0.0 { 0.0 } else { (assign19210_e17319 * (locals.var_pparam_b4soiucste_dn11 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiucs_dn12 * assign19210_e17319) + (locals.var_pparam_b4soiucs * if locals.var_pparam_b4soiucste_dn12 == 0.0 && ((locals.var_pparam_b4soiucste) as f64).is_finite() && ((locals.var_pparam_b4soiucste) as f64).fract() == 0.0 { 0.0 } else { (assign19210_e17319 * (locals.var_pparam_b4soiucste_dn12 * (locals.var_tempratio).ln())) })),)
    } else {
        (locals.var_ucs, locals.var_ucs_dn3, locals.var_ucs_dn4, locals.var_ucs_dn5, locals.var_ucs_dn6, locals.var_ucs_dn7, locals.var_ucs_dn8, locals.var_ucs_dn9, locals.var_ucs_dn10, locals.var_ucs_dn11, locals.var_ucs_dn12,)
    }
};
        locals.var_ucs = assign19210_e17322;
        locals.var_ucs_dn3 = assign19210_e17322_d_n3;
        locals.var_ucs_dn4 = assign19210_e17322_d_n4;
        locals.var_ucs_dn5 = assign19210_e17322_d_n5;
        locals.var_ucs_dn6 = assign19210_e17322_d_n6;
        locals.var_ucs_dn7 = assign19210_e17322_d_n7;
        locals.var_ucs_dn8 = assign19210_e17322_d_n8;
        locals.var_ucs_dn9 = assign19210_e17322_d_n9;
        locals.var_ucs_dn10 = assign19210_e17322_d_n10;
        locals.var_ucs_dn11 = assign19210_e17322_d_n11;
        locals.var_ucs_dn12 = assign19210_e17322_d_n12;
        locals.var_ucs_rv = 0.0;

        let (assign19220_e17337, assign19220_e17337_d_n3, assign19220_e17337_d_n4, assign19220_e17337_d_n5, assign19220_e17337_d_n6, assign19220_e17337_d_n7, assign19220_e17337_d_n8, assign19220_e17337_d_n9, assign19220_e17337_d_n10, assign19220_e17337_d_n11, assign19220_e17337_d_n12,) = {
    if (((locals.var_guard1225 == 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1227 == 0.0)) {
        let assign19220_e17334: f64 = (locals.var_tempratio).powf(locals.var_pparam_b4soiud1);
        let assign19220_e17335: f64 = (locals.var_pparam_b4soiud * assign19220_e17334);
        (assign19220_e17335, ((locals.var_pparam_b4soiud_dn3 * assign19220_e17334) + (locals.var_pparam_b4soiud * if locals.var_pparam_b4soiud1_dn3 == 0.0 && ((locals.var_pparam_b4soiud1) as f64).is_finite() && ((locals.var_pparam_b4soiud1) as f64).fract() == 0.0 { 0.0 } else { (assign19220_e17334 * (locals.var_pparam_b4soiud1_dn3 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiud_dn4 * assign19220_e17334) + (locals.var_pparam_b4soiud * if locals.var_pparam_b4soiud1_dn4 == 0.0 && ((locals.var_pparam_b4soiud1) as f64).is_finite() && ((locals.var_pparam_b4soiud1) as f64).fract() == 0.0 { if locals.var_pparam_b4soiud1 == 0.0 { 0.0 } else { (locals.var_pparam_b4soiud1 * ((locals.var_tempratio).powf(locals.var_pparam_b4soiud1 - 1.0) * locals.var_tempratio_dn4)) } } else { (assign19220_e17334 * ((locals.var_pparam_b4soiud1_dn4 * (locals.var_tempratio).ln()) + (locals.var_pparam_b4soiud1 * (locals.var_tempratio_dn4 / locals.var_tempratio)))) })), ((locals.var_pparam_b4soiud_dn5 * assign19220_e17334) + (locals.var_pparam_b4soiud * if locals.var_pparam_b4soiud1_dn5 == 0.0 && ((locals.var_pparam_b4soiud1) as f64).is_finite() && ((locals.var_pparam_b4soiud1) as f64).fract() == 0.0 { if locals.var_pparam_b4soiud1 == 0.0 { 0.0 } else { (locals.var_pparam_b4soiud1 * ((locals.var_tempratio).powf(locals.var_pparam_b4soiud1 - 1.0) * locals.var_tempratio_dn5)) } } else { (assign19220_e17334 * ((locals.var_pparam_b4soiud1_dn5 * (locals.var_tempratio).ln()) + (locals.var_pparam_b4soiud1 * (locals.var_tempratio_dn5 / locals.var_tempratio)))) })), ((locals.var_pparam_b4soiud_dn6 * assign19220_e17334) + (locals.var_pparam_b4soiud * if locals.var_pparam_b4soiud1_dn6 == 0.0 && ((locals.var_pparam_b4soiud1) as f64).is_finite() && ((locals.var_pparam_b4soiud1) as f64).fract() == 0.0 { if locals.var_pparam_b4soiud1 == 0.0 { 0.0 } else { (locals.var_pparam_b4soiud1 * ((locals.var_tempratio).powf(locals.var_pparam_b4soiud1 - 1.0) * locals.var_tempratio_dn6)) } } else { (assign19220_e17334 * ((locals.var_pparam_b4soiud1_dn6 * (locals.var_tempratio).ln()) + (locals.var_pparam_b4soiud1 * (locals.var_tempratio_dn6 / locals.var_tempratio)))) })), ((locals.var_pparam_b4soiud_dn7 * assign19220_e17334) + (locals.var_pparam_b4soiud * if locals.var_pparam_b4soiud1_dn7 == 0.0 && ((locals.var_pparam_b4soiud1) as f64).is_finite() && ((locals.var_pparam_b4soiud1) as f64).fract() == 0.0 { 0.0 } else { (assign19220_e17334 * (locals.var_pparam_b4soiud1_dn7 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiud_dn8 * assign19220_e17334) + (locals.var_pparam_b4soiud * if locals.var_pparam_b4soiud1_dn8 == 0.0 && ((locals.var_pparam_b4soiud1) as f64).is_finite() && ((locals.var_pparam_b4soiud1) as f64).fract() == 0.0 { 0.0 } else { (assign19220_e17334 * (locals.var_pparam_b4soiud1_dn8 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiud_dn9 * assign19220_e17334) + (locals.var_pparam_b4soiud * if locals.var_pparam_b4soiud1_dn9 == 0.0 && ((locals.var_pparam_b4soiud1) as f64).is_finite() && ((locals.var_pparam_b4soiud1) as f64).fract() == 0.0 { 0.0 } else { (assign19220_e17334 * (locals.var_pparam_b4soiud1_dn9 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiud_dn10 * assign19220_e17334) + (locals.var_pparam_b4soiud * if locals.var_pparam_b4soiud1_dn10 == 0.0 && ((locals.var_pparam_b4soiud1) as f64).is_finite() && ((locals.var_pparam_b4soiud1) as f64).fract() == 0.0 { 0.0 } else { (assign19220_e17334 * (locals.var_pparam_b4soiud1_dn10 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiud_dn11 * assign19220_e17334) + (locals.var_pparam_b4soiud * if locals.var_pparam_b4soiud1_dn11 == 0.0 && ((locals.var_pparam_b4soiud1) as f64).is_finite() && ((locals.var_pparam_b4soiud1) as f64).fract() == 0.0 { 0.0 } else { (assign19220_e17334 * (locals.var_pparam_b4soiud1_dn11 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiud_dn12 * assign19220_e17334) + (locals.var_pparam_b4soiud * if locals.var_pparam_b4soiud1_dn12 == 0.0 && ((locals.var_pparam_b4soiud1) as f64).is_finite() && ((locals.var_pparam_b4soiud1) as f64).fract() == 0.0 { 0.0 } else { (assign19220_e17334 * (locals.var_pparam_b4soiud1_dn12 * (locals.var_tempratio).ln())) })),)
    } else {
        (locals.var_ud, locals.var_ud_dn3, locals.var_ud_dn4, locals.var_ud_dn5, locals.var_ud_dn6, locals.var_ud_dn7, locals.var_ud_dn8, locals.var_ud_dn9, locals.var_ud_dn10, locals.var_ud_dn11, locals.var_ud_dn12,)
    }
};
        locals.var_ud = assign19220_e17337;
        locals.var_ud_dn3 = assign19220_e17337_d_n3;
        locals.var_ud_dn4 = assign19220_e17337_d_n4;
        locals.var_ud_dn5 = assign19220_e17337_d_n5;
        locals.var_ud_dn6 = assign19220_e17337_d_n6;
        locals.var_ud_dn7 = assign19220_e17337_d_n7;
        locals.var_ud_dn8 = assign19220_e17337_d_n8;
        locals.var_ud_dn9 = assign19220_e17337_d_n9;
        locals.var_ud_dn10 = assign19220_e17337_d_n10;
        locals.var_ud_dn11 = assign19220_e17337_d_n11;
        locals.var_ud_dn12 = assign19220_e17337_d_n12;
        locals.var_ud_rv = 0.0;

        let (assign19230_e17348, assign19230_e17348_d_n3, assign19230_e17348_d_n4, assign19230_e17348_d_n5, assign19230_e17348_d_n6, assign19230_e17348_d_n7, assign19230_e17348_d_n8, assign19230_e17348_d_n9, assign19230_e17348_d_n10, assign19230_e17348_d_n11, assign19230_e17348_d_n12,) = {
    if (((locals.var_guard1225 == 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1227 == 0.0)) {
        (locals.var_b4soivgsteffvth, locals.var_b4soivgsteffvth_dn3, locals.var_b4soivgsteffvth_dn4, locals.var_b4soivgsteffvth_dn5, locals.var_b4soivgsteffvth_dn6, locals.var_b4soivgsteffvth_dn7, locals.var_b4soivgsteffvth_dn8, locals.var_b4soivgsteffvth_dn9, locals.var_b4soivgsteffvth_dn10, locals.var_b4soivgsteffvth_dn11, locals.var_b4soivgsteffvth_dn12,)
    } else {
        (locals.var_vgsteffvth, locals.var_vgsteffvth_dn3, locals.var_vgsteffvth_dn4, locals.var_vgsteffvth_dn5, locals.var_vgsteffvth_dn6, locals.var_vgsteffvth_dn7, locals.var_vgsteffvth_dn8, locals.var_vgsteffvth_dn9, locals.var_vgsteffvth_dn10, locals.var_vgsteffvth_dn11, locals.var_vgsteffvth_dn12,)
    }
};
        locals.var_vgsteffvth = assign19230_e17348;
        locals.var_vgsteffvth_dn3 = assign19230_e17348_d_n3;
        locals.var_vgsteffvth_dn4 = assign19230_e17348_d_n4;
        locals.var_vgsteffvth_dn5 = assign19230_e17348_d_n5;
        locals.var_vgsteffvth_dn6 = assign19230_e17348_d_n6;
        locals.var_vgsteffvth_dn7 = assign19230_e17348_d_n7;
        locals.var_vgsteffvth_dn8 = assign19230_e17348_d_n8;
        locals.var_vgsteffvth_dn9 = assign19230_e17348_d_n9;
        locals.var_vgsteffvth_dn10 = assign19230_e17348_d_n10;
        locals.var_vgsteffvth_dn11 = assign19230_e17348_d_n11;
        locals.var_vgsteffvth_dn12 = assign19230_e17348_d_n12;
        locals.var_vgsteffvth_rv = 0.0;

        let (assign19240_e17377, assign19240_e17377_d_n3, assign19240_e17377_d_n4, assign19240_e17377_d_n5, assign19240_e17377_d_n6, assign19240_e17377_d_n7, assign19240_e17377_d_n8, assign19240_e17377_d_n9, assign19240_e17377_d_n10, assign19240_e17377_d_n11, assign19240_e17377_d_n12,) = {
    if (((locals.var_guard1225 == 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1227 == 0.0)) {
        let assign19240_e17361: f64 = (locals.var_vgsteff__blk840 / locals.var_vgsteffvth);
        let assign19240_e17362: f64 = (1.0 + assign19240_e17361);
        let (assign19240_e17373, assign19240_e17373_d_n3, assign19240_e17373_d_n4, assign19240_e17373_d_n5, assign19240_e17373_d_n6, assign19240_e17373_d_n7, assign19240_e17373_d_n8, assign19240_e17373_d_n9, assign19240_e17373_d_n10, assign19240_e17373_d_n11, assign19240_e17373_d_n12,) = {
            if (assign19240_e17362 > 1e-38) {
                let assign19240_e17368: f64 = (locals.var_vgsteff__blk840 / locals.var_vgsteffvth);
                let assign19240_e17369: f64 = (1.0 + assign19240_e17368);
                let assign19240_e17370: f64 = (assign19240_e17369).ln();
                (assign19240_e17370, ((((locals.var_vgsteff__blk840_dn3 * locals.var_vgsteffvth) - (locals.var_vgsteff__blk840 * locals.var_vgsteffvth_dn3)) / (locals.var_vgsteffvth * locals.var_vgsteffvth)) / assign19240_e17369), ((((locals.var_vgsteff__blk840_dn4 * locals.var_vgsteffvth) - (locals.var_vgsteff__blk840 * locals.var_vgsteffvth_dn4)) / (locals.var_vgsteffvth * locals.var_vgsteffvth)) / assign19240_e17369), ((((locals.var_vgsteff__blk840_dn5 * locals.var_vgsteffvth) - (locals.var_vgsteff__blk840 * locals.var_vgsteffvth_dn5)) / (locals.var_vgsteffvth * locals.var_vgsteffvth)) / assign19240_e17369), ((((locals.var_vgsteff__blk840_dn6 * locals.var_vgsteffvth) - (locals.var_vgsteff__blk840 * locals.var_vgsteffvth_dn6)) / (locals.var_vgsteffvth * locals.var_vgsteffvth)) / assign19240_e17369), ((((locals.var_vgsteff__blk840_dn7 * locals.var_vgsteffvth) - (locals.var_vgsteff__blk840 * locals.var_vgsteffvth_dn7)) / (locals.var_vgsteffvth * locals.var_vgsteffvth)) / assign19240_e17369), ((((locals.var_vgsteff__blk840_dn8 * locals.var_vgsteffvth) - (locals.var_vgsteff__blk840 * locals.var_vgsteffvth_dn8)) / (locals.var_vgsteffvth * locals.var_vgsteffvth)) / assign19240_e17369), ((((locals.var_vgsteff__blk840_dn9 * locals.var_vgsteffvth) - (locals.var_vgsteff__blk840 * locals.var_vgsteffvth_dn9)) / (locals.var_vgsteffvth * locals.var_vgsteffvth)) / assign19240_e17369), ((((locals.var_vgsteff__blk840_dn10 * locals.var_vgsteffvth) - (locals.var_vgsteff__blk840 * locals.var_vgsteffvth_dn10)) / (locals.var_vgsteffvth * locals.var_vgsteffvth)) / assign19240_e17369), ((((locals.var_vgsteff__blk840_dn11 * locals.var_vgsteffvth) - (locals.var_vgsteff__blk840 * locals.var_vgsteffvth_dn11)) / (locals.var_vgsteffvth * locals.var_vgsteffvth)) / assign19240_e17369), ((((locals.var_vgsteff__blk840_dn12 * locals.var_vgsteffvth) - (locals.var_vgsteff__blk840 * locals.var_vgsteffvth_dn12)) / (locals.var_vgsteffvth * locals.var_vgsteffvth)) / assign19240_e17369),)
            } else {
                let assign19240_e17372: f64 = (-87.49823353377374);
                (assign19240_e17372, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign19240_e17374: f64 = (locals.var_ucs * assign19240_e17373);
        let assign19240_e17375: f64 = (assign19240_e17374).exp();
        (assign19240_e17375, (assign19240_e17375 * ((locals.var_ucs_dn3 * assign19240_e17373) + (locals.var_ucs * assign19240_e17373_d_n3))), (assign19240_e17375 * ((locals.var_ucs_dn4 * assign19240_e17373) + (locals.var_ucs * assign19240_e17373_d_n4))), (assign19240_e17375 * ((locals.var_ucs_dn5 * assign19240_e17373) + (locals.var_ucs * assign19240_e17373_d_n5))), (assign19240_e17375 * ((locals.var_ucs_dn6 * assign19240_e17373) + (locals.var_ucs * assign19240_e17373_d_n6))), (assign19240_e17375 * ((locals.var_ucs_dn7 * assign19240_e17373) + (locals.var_ucs * assign19240_e17373_d_n7))), (assign19240_e17375 * ((locals.var_ucs_dn8 * assign19240_e17373) + (locals.var_ucs * assign19240_e17373_d_n8))), (assign19240_e17375 * ((locals.var_ucs_dn9 * assign19240_e17373) + (locals.var_ucs * assign19240_e17373_d_n9))), (assign19240_e17375 * ((locals.var_ucs_dn10 * assign19240_e17373) + (locals.var_ucs * assign19240_e17373_d_n10))), (assign19240_e17375 * ((locals.var_ucs_dn11 * assign19240_e17373) + (locals.var_ucs * assign19240_e17373_d_n11))), (assign19240_e17375 * ((locals.var_ucs_dn12 * assign19240_e17373) + (locals.var_ucs * assign19240_e17373_d_n12))),)
    } else {
        (locals.var_t10__blk818, locals.var_t10__blk818_dn3, locals.var_t10__blk818_dn4, locals.var_t10__blk818_dn5, locals.var_t10__blk818_dn6, locals.var_t10__blk818_dn7, locals.var_t10__blk818_dn8, locals.var_t10__blk818_dn9, locals.var_t10__blk818_dn10, locals.var_t10__blk818_dn11, locals.var_t10__blk818_dn12,)
    }
};
        locals.var_t10__blk818 = assign19240_e17377;
        locals.var_t10__blk818_dn3 = assign19240_e17377_d_n3;
        locals.var_t10__blk818_dn4 = assign19240_e17377_d_n4;
        locals.var_t10__blk818_dn5 = assign19240_e17377_d_n5;
        locals.var_t10__blk818_dn6 = assign19240_e17377_d_n6;
        locals.var_t10__blk818_dn7 = assign19240_e17377_d_n7;
        locals.var_t10__blk818_dn8 = assign19240_e17377_d_n8;
        locals.var_t10__blk818_dn9 = assign19240_e17377_d_n9;
        locals.var_t10__blk818_dn10 = assign19240_e17377_d_n10;
        locals.var_t10__blk818_dn11 = assign19240_e17377_d_n11;
        locals.var_t10__blk818_dn12 = assign19240_e17377_d_n12;
        locals.var_t10__blk818_rv = 0.0;

        let (assign19250_e17390, assign19250_e17390_d_n3, assign19250_e17390_d_n4, assign19250_e17390_d_n5, assign19250_e17390_d_n6, assign19250_e17390_d_n7, assign19250_e17390_d_n8, assign19250_e17390_d_n9, assign19250_e17390_d_n10, assign19250_e17390_d_n11, assign19250_e17390_d_n12,) = {
    if (((locals.var_guard1225 == 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1227 == 0.0)) {
        let assign19250_e17388: f64 = (locals.var_ud / locals.var_t10__blk818);
        (assign19250_e17388, (((locals.var_ud_dn3 * locals.var_t10__blk818) - (locals.var_ud * locals.var_t10__blk818_dn3)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((locals.var_ud_dn4 * locals.var_t10__blk818) - (locals.var_ud * locals.var_t10__blk818_dn4)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((locals.var_ud_dn5 * locals.var_t10__blk818) - (locals.var_ud * locals.var_t10__blk818_dn5)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((locals.var_ud_dn6 * locals.var_t10__blk818) - (locals.var_ud * locals.var_t10__blk818_dn6)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((locals.var_ud_dn7 * locals.var_t10__blk818) - (locals.var_ud * locals.var_t10__blk818_dn7)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((locals.var_ud_dn8 * locals.var_t10__blk818) - (locals.var_ud * locals.var_t10__blk818_dn8)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((locals.var_ud_dn9 * locals.var_t10__blk818) - (locals.var_ud * locals.var_t10__blk818_dn9)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((locals.var_ud_dn10 * locals.var_t10__blk818) - (locals.var_ud * locals.var_t10__blk818_dn10)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((locals.var_ud_dn11 * locals.var_t10__blk818) - (locals.var_ud * locals.var_t10__blk818_dn11)) / (locals.var_t10__blk818 * locals.var_t10__blk818)), (((locals.var_ud_dn12 * locals.var_t10__blk818) - (locals.var_ud * locals.var_t10__blk818_dn12)) / (locals.var_t10__blk818 * locals.var_t10__blk818)),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12,)
    }
};
        locals.var_t11 = assign19250_e17390;
        locals.var_t11_dn3 = assign19250_e17390_d_n3;
        locals.var_t11_dn4 = assign19250_e17390_d_n4;
        locals.var_t11_dn5 = assign19250_e17390_d_n5;
        locals.var_t11_dn6 = assign19250_e17390_d_n6;
        locals.var_t11_dn7 = assign19250_e17390_d_n7;
        locals.var_t11_dn8 = assign19250_e17390_d_n8;
        locals.var_t11_dn9 = assign19250_e17390_d_n9;
        locals.var_t11_dn10 = assign19250_e17390_d_n10;
        locals.var_t11_dn11 = assign19250_e17390_d_n11;
        locals.var_t11_dn12 = assign19250_e17390_d_n12;
        locals.var_t11_rv = 0.0;

        let (assign19260_e17405, assign19260_e17405_d_n3, assign19260_e17405_d_n4, assign19260_e17405_d_n5, assign19260_e17405_d_n6, assign19260_e17405_d_n7, assign19260_e17405_d_n8, assign19260_e17405_d_n9, assign19260_e17405_d_n10, assign19260_e17405_d_n11, assign19260_e17405_d_n12,) = {
    if (((locals.var_guard1225 == 0.0) && (locals.var_guard1226 == 0.0)) && (locals.var_guard1227 == 0.0)) {
        let assign19260_e17401: f64 = (locals.var_t1__blk809 * locals.var_t2__blk810);
        let assign19260_e17403: f64 = (assign19260_e17401 + locals.var_t11);
        (assign19260_e17403, (((locals.var_t1__blk809_dn3 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn3)) + locals.var_t11_dn3), (((locals.var_t1__blk809_dn4 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn4)) + locals.var_t11_dn4), (((locals.var_t1__blk809_dn5 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn5)) + locals.var_t11_dn5), (((locals.var_t1__blk809_dn6 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn6)) + locals.var_t11_dn6), (((locals.var_t1__blk809_dn7 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn7)) + locals.var_t11_dn7), (((locals.var_t1__blk809_dn8 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn8)) + locals.var_t11_dn8), (((locals.var_t1__blk809_dn9 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn9)) + locals.var_t11_dn9), (((locals.var_t1__blk809_dn10 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn10)) + locals.var_t11_dn10), (((locals.var_t1__blk809_dn11 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn11)) + locals.var_t11_dn11), (((locals.var_t1__blk809_dn12 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn12)) + locals.var_t11_dn12),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign19260_e17405;
        locals.var_t5__blk813_dn3 = assign19260_e17405_d_n3;
        locals.var_t5__blk813_dn4 = assign19260_e17405_d_n4;
        locals.var_t5__blk813_dn5 = assign19260_e17405_d_n5;
        locals.var_t5__blk813_dn6 = assign19260_e17405_d_n6;
        locals.var_t5__blk813_dn7 = assign19260_e17405_d_n7;
        locals.var_t5__blk813_dn8 = assign19260_e17405_d_n8;
        locals.var_t5__blk813_dn9 = assign19260_e17405_d_n9;
        locals.var_t5__blk813_dn10 = assign19260_e17405_d_n10;
        locals.var_t5__blk813_dn11 = assign19260_e17405_d_n11;
        locals.var_t5__blk813_dn12 = assign19260_e17405_d_n12;
        locals.var_t5__blk813_rv = 0.0;

        let assign19270_e17408: f64 = (-0.8);
        let assign19270_e17409: f64 = if locals.var_t5__blk813 >= assign19270_e17408 { 1.0 } else { 0.0 };
        locals.var_guard1228 = assign19270_e17409;
        locals.var_guard1228_rv = 0.0;

        let (assign19280_e17415, assign19280_e17415_d_n3, assign19280_e17415_d_n4, assign19280_e17415_d_n5, assign19280_e17415_d_n6, assign19280_e17415_d_n7, assign19280_e17415_d_n8, assign19280_e17415_d_n9, assign19280_e17415_d_n10, assign19280_e17415_d_n11, assign19280_e17415_d_n12,) = {
    if (locals.var_guard1228 != 0.0) {
        let assign19280_e17413: f64 = (1.0 + locals.var_t5__blk813);
        (assign19280_e17413, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    } else {
        (locals.var_denomi, locals.var_denomi_dn3, locals.var_denomi_dn4, locals.var_denomi_dn5, locals.var_denomi_dn6, locals.var_denomi_dn7, locals.var_denomi_dn8, locals.var_denomi_dn9, locals.var_denomi_dn10, locals.var_denomi_dn11, locals.var_denomi_dn12,)
    }
};
        locals.var_denomi = assign19280_e17415;
        locals.var_denomi_dn3 = assign19280_e17415_d_n3;
        locals.var_denomi_dn4 = assign19280_e17415_d_n4;
        locals.var_denomi_dn5 = assign19280_e17415_d_n5;
        locals.var_denomi_dn6 = assign19280_e17415_d_n6;
        locals.var_denomi_dn7 = assign19280_e17415_d_n7;
        locals.var_denomi_dn8 = assign19280_e17415_d_n8;
        locals.var_denomi_dn9 = assign19280_e17415_d_n9;
        locals.var_denomi_dn10 = assign19280_e17415_d_n10;
        locals.var_denomi_dn11 = assign19280_e17415_d_n11;
        locals.var_denomi_dn12 = assign19280_e17415_d_n12;
        locals.var_denomi_rv = 0.0;

        let (assign19290_e17426, assign19290_e17426_d_n3, assign19290_e17426_d_n4, assign19290_e17426_d_n5, assign19290_e17426_d_n6, assign19290_e17426_d_n7, assign19290_e17426_d_n8, assign19290_e17426_d_n9, assign19290_e17426_d_n10, assign19290_e17426_d_n11, assign19290_e17426_d_n12,) = {
    if (locals.var_guard1228 == 0.0) {
        let assign19290_e17422: f64 = (10.0 * locals.var_t5__blk813);
        let assign19290_e17423: f64 = (7.0 + assign19290_e17422);
        let assign19290_e17424: f64 = (1.0 / assign19290_e17423);
        (assign19290_e17424, (-((10.0 * locals.var_t5__blk813_dn3) / (assign19290_e17423 * assign19290_e17423))), (-((10.0 * locals.var_t5__blk813_dn4) / (assign19290_e17423 * assign19290_e17423))), (-((10.0 * locals.var_t5__blk813_dn5) / (assign19290_e17423 * assign19290_e17423))), (-((10.0 * locals.var_t5__blk813_dn6) / (assign19290_e17423 * assign19290_e17423))), (-((10.0 * locals.var_t5__blk813_dn7) / (assign19290_e17423 * assign19290_e17423))), (-((10.0 * locals.var_t5__blk813_dn8) / (assign19290_e17423 * assign19290_e17423))), (-((10.0 * locals.var_t5__blk813_dn9) / (assign19290_e17423 * assign19290_e17423))), (-((10.0 * locals.var_t5__blk813_dn10) / (assign19290_e17423 * assign19290_e17423))), (-((10.0 * locals.var_t5__blk813_dn11) / (assign19290_e17423 * assign19290_e17423))), (-((10.0 * locals.var_t5__blk813_dn12) / (assign19290_e17423 * assign19290_e17423))),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign19290_e17426;
        locals.var_t9_dn3 = assign19290_e17426_d_n3;
        locals.var_t9_dn4 = assign19290_e17426_d_n4;
        locals.var_t9_dn5 = assign19290_e17426_d_n5;
        locals.var_t9_dn6 = assign19290_e17426_d_n6;
        locals.var_t9_dn7 = assign19290_e17426_d_n7;
        locals.var_t9_dn8 = assign19290_e17426_d_n8;
        locals.var_t9_dn9 = assign19290_e17426_d_n9;
        locals.var_t9_dn10 = assign19290_e17426_d_n10;
        locals.var_t9_dn11 = assign19290_e17426_d_n11;
        locals.var_t9_dn12 = assign19290_e17426_d_n12;
        locals.var_t9_rv = 0.0;

        let (assign19300_e17435, assign19300_e17435_d_n3, assign19300_e17435_d_n4, assign19300_e17435_d_n5, assign19300_e17435_d_n6, assign19300_e17435_d_n7, assign19300_e17435_d_n8, assign19300_e17435_d_n9, assign19300_e17435_d_n10, assign19300_e17435_d_n11, assign19300_e17435_d_n12,) = {
    if (locals.var_guard1228 == 0.0) {
        let assign19300_e17431: f64 = (0.6 + locals.var_t5__blk813);
        let assign19300_e17433: f64 = (assign19300_e17431 * locals.var_t9);
        (assign19300_e17433, ((locals.var_t5__blk813_dn3 * locals.var_t9) + (assign19300_e17431 * locals.var_t9_dn3)), ((locals.var_t5__blk813_dn4 * locals.var_t9) + (assign19300_e17431 * locals.var_t9_dn4)), ((locals.var_t5__blk813_dn5 * locals.var_t9) + (assign19300_e17431 * locals.var_t9_dn5)), ((locals.var_t5__blk813_dn6 * locals.var_t9) + (assign19300_e17431 * locals.var_t9_dn6)), ((locals.var_t5__blk813_dn7 * locals.var_t9) + (assign19300_e17431 * locals.var_t9_dn7)), ((locals.var_t5__blk813_dn8 * locals.var_t9) + (assign19300_e17431 * locals.var_t9_dn8)), ((locals.var_t5__blk813_dn9 * locals.var_t9) + (assign19300_e17431 * locals.var_t9_dn9)), ((locals.var_t5__blk813_dn10 * locals.var_t9) + (assign19300_e17431 * locals.var_t9_dn10)), ((locals.var_t5__blk813_dn11 * locals.var_t9) + (assign19300_e17431 * locals.var_t9_dn11)), ((locals.var_t5__blk813_dn12 * locals.var_t9) + (assign19300_e17431 * locals.var_t9_dn12)),)
    } else {
        (locals.var_denomi, locals.var_denomi_dn3, locals.var_denomi_dn4, locals.var_denomi_dn5, locals.var_denomi_dn6, locals.var_denomi_dn7, locals.var_denomi_dn8, locals.var_denomi_dn9, locals.var_denomi_dn10, locals.var_denomi_dn11, locals.var_denomi_dn12,)
    }
};
        locals.var_denomi = assign19300_e17435;
        locals.var_denomi_dn3 = assign19300_e17435_d_n3;
        locals.var_denomi_dn4 = assign19300_e17435_d_n4;
        locals.var_denomi_dn5 = assign19300_e17435_d_n5;
        locals.var_denomi_dn6 = assign19300_e17435_d_n6;
        locals.var_denomi_dn7 = assign19300_e17435_d_n7;
        locals.var_denomi_dn8 = assign19300_e17435_d_n8;
        locals.var_denomi_dn9 = assign19300_e17435_d_n9;
        locals.var_denomi_dn10 = assign19300_e17435_d_n10;
        locals.var_denomi_dn11 = assign19300_e17435_d_n11;
        locals.var_denomi_dn12 = assign19300_e17435_d_n12;
        locals.var_denomi_rv = 0.0;

        let assign19310_e17440: f64 = (locals.var_ves - locals.var_vfbb);
        let assign19310_e17441: f64 = (p.p124 * assign19310_e17440);
        let assign19310_e17442: f64 = (locals.var_u0temp + assign19310_e17441);
        let assign19310_e17444: f64 = (assign19310_e17442 / locals.var_denomi);
        locals.var_ueff = assign19310_e17444;
        locals.var_ueff_dn3 = ((((locals.var_u0temp_dn3 + (p.p124 * (locals.var_ves_dn3 - locals.var_vfbb_dn3))) * locals.var_denomi) - (assign19310_e17442 * locals.var_denomi_dn3)) / (locals.var_denomi * locals.var_denomi));
        locals.var_ueff_dn4 = ((((locals.var_u0temp_dn4 + (p.p124 * (-locals.var_vfbb_dn4))) * locals.var_denomi) - (assign19310_e17442 * locals.var_denomi_dn4)) / (locals.var_denomi * locals.var_denomi));
        locals.var_ueff_dn5 = ((((locals.var_u0temp_dn5 + (p.p124 * (-locals.var_vfbb_dn5))) * locals.var_denomi) - (assign19310_e17442 * locals.var_denomi_dn5)) / (locals.var_denomi * locals.var_denomi));
        locals.var_ueff_dn6 = ((((locals.var_u0temp_dn6 + (p.p124 * (-locals.var_vfbb_dn6))) * locals.var_denomi) - (assign19310_e17442 * locals.var_denomi_dn6)) / (locals.var_denomi * locals.var_denomi));
        locals.var_ueff_dn7 = ((((locals.var_u0temp_dn7 + (p.p124 * (-locals.var_vfbb_dn7))) * locals.var_denomi) - (assign19310_e17442 * locals.var_denomi_dn7)) / (locals.var_denomi * locals.var_denomi));
        locals.var_ueff_dn8 = ((((locals.var_u0temp_dn8 + (p.p124 * (locals.var_ves_dn8 - locals.var_vfbb_dn8))) * locals.var_denomi) - (assign19310_e17442 * locals.var_denomi_dn8)) / (locals.var_denomi * locals.var_denomi));
        locals.var_ueff_dn9 = ((((locals.var_u0temp_dn9 + (p.p124 * (-locals.var_vfbb_dn9))) * locals.var_denomi) - (assign19310_e17442 * locals.var_denomi_dn9)) / (locals.var_denomi * locals.var_denomi));
        locals.var_ueff_dn10 = ((((locals.var_u0temp_dn10 + (p.p124 * (-locals.var_vfbb_dn10))) * locals.var_denomi) - (assign19310_e17442 * locals.var_denomi_dn10)) / (locals.var_denomi * locals.var_denomi));
        locals.var_ueff_dn11 = ((((locals.var_u0temp_dn11 + (p.p124 * (-locals.var_vfbb_dn11))) * locals.var_denomi) - (assign19310_e17442 * locals.var_denomi_dn11)) / (locals.var_denomi * locals.var_denomi));
        locals.var_ueff_dn12 = ((((locals.var_u0temp_dn12 + (p.p124 * (-locals.var_vfbb_dn12))) * locals.var_denomi) - (assign19310_e17442 * locals.var_denomi_dn12)) / (locals.var_denomi * locals.var_denomi));
        locals.var_ueff_rv = 0.0;

        let assign19320_e17447: f64 = (locals.var_ueff * p.p31);
        locals.var_ueff = assign19320_e17447;
        locals.var_ueff_dn3 = (locals.var_ueff_dn3 * p.p31);
        locals.var_ueff_dn4 = (locals.var_ueff_dn4 * p.p31);
        locals.var_ueff_dn5 = (locals.var_ueff_dn5 * p.p31);
        locals.var_ueff_dn6 = (locals.var_ueff_dn6 * p.p31);
        locals.var_ueff_dn7 = (locals.var_ueff_dn7 * p.p31);
        locals.var_ueff_dn8 = (locals.var_ueff_dn8 * p.p31);
        locals.var_ueff_dn9 = (locals.var_ueff_dn9 * p.p31);
        locals.var_ueff_dn10 = (locals.var_ueff_dn10 * p.p31);
        locals.var_ueff_dn11 = (locals.var_ueff_dn11 * p.p31);
        locals.var_ueff_dn12 = (locals.var_ueff_dn12 * p.p31);
        locals.var_ueff_rv = 0.0;

        locals.var_b4soiueff = locals.var_ueff;
        locals.var_b4soiueff_dn3 = locals.var_ueff_dn3;
        locals.var_b4soiueff_dn4 = locals.var_ueff_dn4;
        locals.var_b4soiueff_dn5 = locals.var_ueff_dn5;
        locals.var_b4soiueff_dn6 = locals.var_ueff_dn6;
        locals.var_b4soiueff_dn7 = locals.var_ueff_dn7;
        locals.var_b4soiueff_dn8 = locals.var_ueff_dn8;
        locals.var_b4soiueff_dn9 = locals.var_ueff_dn9;
        locals.var_b4soiueff_dn10 = locals.var_ueff_dn10;
        locals.var_b4soiueff_dn11 = locals.var_ueff_dn11;
        locals.var_b4soiueff_dn12 = locals.var_ueff_dn12;
        locals.var_b4soiueff_rv = 0.0;

        let assign19340_e17451: f64 = (locals.var_weff * locals.var_vsattemp);
        let assign19340_e17453: f64 = (assign19340_e17451 * locals.var_b4soicox);
        locals.var_wvcox = assign19340_e17453;
        locals.var_wvcox_dn3 = (((locals.var_weff_dn3 * locals.var_vsattemp) + (locals.var_weff * locals.var_vsattemp_dn3)) * locals.var_b4soicox);
        locals.var_wvcox_dn4 = (((locals.var_weff_dn4 * locals.var_vsattemp) + (locals.var_weff * locals.var_vsattemp_dn4)) * locals.var_b4soicox);
        locals.var_wvcox_dn5 = (((locals.var_weff_dn5 * locals.var_vsattemp) + (locals.var_weff * locals.var_vsattemp_dn5)) * locals.var_b4soicox);
        locals.var_wvcox_dn6 = (((locals.var_weff_dn6 * locals.var_vsattemp) + (locals.var_weff * locals.var_vsattemp_dn6)) * locals.var_b4soicox);
        locals.var_wvcox_dn7 = (((locals.var_weff_dn7 * locals.var_vsattemp) + (locals.var_weff * locals.var_vsattemp_dn7)) * locals.var_b4soicox);
        locals.var_wvcox_dn8 = (((locals.var_weff_dn8 * locals.var_vsattemp) + (locals.var_weff * locals.var_vsattemp_dn8)) * locals.var_b4soicox);
        locals.var_wvcox_dn9 = (((locals.var_weff_dn9 * locals.var_vsattemp) + (locals.var_weff * locals.var_vsattemp_dn9)) * locals.var_b4soicox);
        locals.var_wvcox_dn10 = (((locals.var_weff_dn10 * locals.var_vsattemp) + (locals.var_weff * locals.var_vsattemp_dn10)) * locals.var_b4soicox);
        locals.var_wvcox_dn11 = (((locals.var_weff_dn11 * locals.var_vsattemp) + (locals.var_weff * locals.var_vsattemp_dn11)) * locals.var_b4soicox);
        locals.var_wvcox_dn12 = (((locals.var_weff_dn12 * locals.var_vsattemp) + (locals.var_weff * locals.var_vsattemp_dn12)) * locals.var_b4soicox);
        locals.var_wvcox_rv = 0.0;

        let assign19350_e17456: f64 = (locals.var_wvcox * locals.var_rds);
        locals.var_wvcoxrds = assign19350_e17456;
        locals.var_wvcoxrds_dn3 = ((locals.var_wvcox_dn3 * locals.var_rds) + (locals.var_wvcox * locals.var_rds_dn3));
        locals.var_wvcoxrds_dn4 = ((locals.var_wvcox_dn4 * locals.var_rds) + (locals.var_wvcox * locals.var_rds_dn4));
        locals.var_wvcoxrds_dn5 = ((locals.var_wvcox_dn5 * locals.var_rds) + (locals.var_wvcox * locals.var_rds_dn5));
        locals.var_wvcoxrds_dn6 = ((locals.var_wvcox_dn6 * locals.var_rds) + (locals.var_wvcox * locals.var_rds_dn6));
        locals.var_wvcoxrds_dn7 = ((locals.var_wvcox_dn7 * locals.var_rds) + (locals.var_wvcox * locals.var_rds_dn7));
        locals.var_wvcoxrds_dn8 = ((locals.var_wvcox_dn8 * locals.var_rds) + (locals.var_wvcox * locals.var_rds_dn8));
        locals.var_wvcoxrds_dn9 = ((locals.var_wvcox_dn9 * locals.var_rds) + (locals.var_wvcox * locals.var_rds_dn9));
        locals.var_wvcoxrds_dn10 = ((locals.var_wvcox_dn10 * locals.var_rds) + (locals.var_wvcox * locals.var_rds_dn10));
        locals.var_wvcoxrds_dn11 = ((locals.var_wvcox_dn11 * locals.var_rds) + (locals.var_wvcox * locals.var_rds_dn11));
        locals.var_wvcoxrds_dn12 = ((locals.var_wvcox_dn12 * locals.var_rds) + (locals.var_wvcox * locals.var_rds_dn12));
        locals.var_wvcoxrds_rv = 0.0;

        let assign19360_e17459: f64 = (2.0 * locals.var_vsattemp);
        let assign19360_e17461: f64 = (assign19360_e17459 / locals.var_ueff);
        locals.var_esat = assign19360_e17461;
        locals.var_esat_dn3 = ((((2.0 * locals.var_vsattemp_dn3) * locals.var_ueff) - (assign19360_e17459 * locals.var_ueff_dn3)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esat_dn4 = ((((2.0 * locals.var_vsattemp_dn4) * locals.var_ueff) - (assign19360_e17459 * locals.var_ueff_dn4)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esat_dn5 = ((((2.0 * locals.var_vsattemp_dn5) * locals.var_ueff) - (assign19360_e17459 * locals.var_ueff_dn5)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esat_dn6 = ((((2.0 * locals.var_vsattemp_dn6) * locals.var_ueff) - (assign19360_e17459 * locals.var_ueff_dn6)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esat_dn7 = ((((2.0 * locals.var_vsattemp_dn7) * locals.var_ueff) - (assign19360_e17459 * locals.var_ueff_dn7)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esat_dn8 = ((((2.0 * locals.var_vsattemp_dn8) * locals.var_ueff) - (assign19360_e17459 * locals.var_ueff_dn8)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esat_dn9 = ((((2.0 * locals.var_vsattemp_dn9) * locals.var_ueff) - (assign19360_e17459 * locals.var_ueff_dn9)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esat_dn10 = ((((2.0 * locals.var_vsattemp_dn10) * locals.var_ueff) - (assign19360_e17459 * locals.var_ueff_dn10)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esat_dn11 = ((((2.0 * locals.var_vsattemp_dn11) * locals.var_ueff) - (assign19360_e17459 * locals.var_ueff_dn11)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esat_dn12 = ((((2.0 * locals.var_vsattemp_dn12) * locals.var_ueff) - (assign19360_e17459 * locals.var_ueff_dn12)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esat_rv = 0.0;

        let assign19370_e17464: f64 = (locals.var_esat * locals.var_leff);
        locals.var_esatl = assign19370_e17464;
        locals.var_esatl_dn3 = ((locals.var_esat_dn3 * locals.var_leff) + (locals.var_esat * locals.var_leff_dn3));
        locals.var_esatl_dn4 = ((locals.var_esat_dn4 * locals.var_leff) + (locals.var_esat * locals.var_leff_dn4));
        locals.var_esatl_dn5 = ((locals.var_esat_dn5 * locals.var_leff) + (locals.var_esat * locals.var_leff_dn5));
        locals.var_esatl_dn6 = ((locals.var_esat_dn6 * locals.var_leff) + (locals.var_esat * locals.var_leff_dn6));
        locals.var_esatl_dn7 = ((locals.var_esat_dn7 * locals.var_leff) + (locals.var_esat * locals.var_leff_dn7));
        locals.var_esatl_dn8 = ((locals.var_esat_dn8 * locals.var_leff) + (locals.var_esat * locals.var_leff_dn8));
        locals.var_esatl_dn9 = ((locals.var_esat_dn9 * locals.var_leff) + (locals.var_esat * locals.var_leff_dn9));
        locals.var_esatl_dn10 = ((locals.var_esat_dn10 * locals.var_leff) + (locals.var_esat * locals.var_leff_dn10));
        locals.var_esatl_dn11 = ((locals.var_esat_dn11 * locals.var_leff) + (locals.var_esat * locals.var_leff_dn11));
        locals.var_esatl_dn12 = ((locals.var_esat_dn12 * locals.var_leff) + (locals.var_esat * locals.var_leff_dn12));
        locals.var_esatl_rv = 0.0;

        let assign19380_e17467: f64 = if locals.var_pparam_b4soia1 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1229 = assign19380_e17467;
        locals.var_guard1229_rv = 0.0;

        let (assign19390_e17471, assign19390_e17471_d_n3, assign19390_e17471_d_n4, assign19390_e17471_d_n5, assign19390_e17471_d_n6, assign19390_e17471_d_n7, assign19390_e17471_d_n8, assign19390_e17471_d_n9, assign19390_e17471_d_n10, assign19390_e17471_d_n11, assign19390_e17471_d_n12,) = {
    if (locals.var_guard1229 != 0.0) {
        (locals.var_pparam_b4soia2, locals.var_pparam_b4soia2_dn3, locals.var_pparam_b4soia2_dn4, locals.var_pparam_b4soia2_dn5, locals.var_pparam_b4soia2_dn6, locals.var_pparam_b4soia2_dn7, locals.var_pparam_b4soia2_dn8, locals.var_pparam_b4soia2_dn9, locals.var_pparam_b4soia2_dn10, locals.var_pparam_b4soia2_dn11, locals.var_pparam_b4soia2_dn12,)
    } else {
        (locals.var_lambda, locals.var_lambda_dn3, locals.var_lambda_dn4, locals.var_lambda_dn5, locals.var_lambda_dn6, locals.var_lambda_dn7, locals.var_lambda_dn8, locals.var_lambda_dn9, locals.var_lambda_dn10, locals.var_lambda_dn11, locals.var_lambda_dn12,)
    }
};
        locals.var_lambda = assign19390_e17471;
        locals.var_lambda_dn3 = assign19390_e17471_d_n3;
        locals.var_lambda_dn4 = assign19390_e17471_d_n4;
        locals.var_lambda_dn5 = assign19390_e17471_d_n5;
        locals.var_lambda_dn6 = assign19390_e17471_d_n6;
        locals.var_lambda_dn7 = assign19390_e17471_d_n7;
        locals.var_lambda_dn8 = assign19390_e17471_d_n8;
        locals.var_lambda_dn9 = assign19390_e17471_d_n9;
        locals.var_lambda_dn10 = assign19390_e17471_d_n10;
        locals.var_lambda_dn11 = assign19390_e17471_d_n11;
        locals.var_lambda_dn12 = assign19390_e17471_d_n12;
        locals.var_lambda_rv = 0.0;

        let assign19400_e17474: f64 = if locals.var_pparam_b4soia1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1230 = assign19400_e17474;
        locals.var_guard1230_rv = 0.0;

        let (assign19410_e17483, assign19410_e17483_d_n3, assign19410_e17483_d_n4, assign19410_e17483_d_n5, assign19410_e17483_d_n6, assign19410_e17483_d_n7, assign19410_e17483_d_n8, assign19410_e17483_d_n9, assign19410_e17483_d_n10, assign19410_e17483_d_n11, assign19410_e17483_d_n12,) = {
    if ((locals.var_guard1229 == 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign19410_e17481: f64 = (1.0 - locals.var_pparam_b4soia2);
        (assign19410_e17481, (-locals.var_pparam_b4soia2_dn3), (-locals.var_pparam_b4soia2_dn4), (-locals.var_pparam_b4soia2_dn5), (-locals.var_pparam_b4soia2_dn6), (-locals.var_pparam_b4soia2_dn7), (-locals.var_pparam_b4soia2_dn8), (-locals.var_pparam_b4soia2_dn9), (-locals.var_pparam_b4soia2_dn10), (-locals.var_pparam_b4soia2_dn11), (-locals.var_pparam_b4soia2_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign19410_e17483;
        locals.var_t0__blk808_dn3 = assign19410_e17483_d_n3;
        locals.var_t0__blk808_dn4 = assign19410_e17483_d_n4;
        locals.var_t0__blk808_dn5 = assign19410_e17483_d_n5;
        locals.var_t0__blk808_dn6 = assign19410_e17483_d_n6;
        locals.var_t0__blk808_dn7 = assign19410_e17483_d_n7;
        locals.var_t0__blk808_dn8 = assign19410_e17483_d_n8;
        locals.var_t0__blk808_dn9 = assign19410_e17483_d_n9;
        locals.var_t0__blk808_dn10 = assign19410_e17483_d_n10;
        locals.var_t0__blk808_dn11 = assign19410_e17483_d_n11;
        locals.var_t0__blk808_dn12 = assign19410_e17483_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign19420_e17496, assign19420_e17496_d_n3, assign19420_e17496_d_n4, assign19420_e17496_d_n5, assign19420_e17496_d_n6, assign19420_e17496_d_n7, assign19420_e17496_d_n8, assign19420_e17496_d_n9, assign19420_e17496_d_n10, assign19420_e17496_d_n11, assign19420_e17496_d_n12,) = {
    if ((locals.var_guard1229 == 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign19420_e17491: f64 = (locals.var_pparam_b4soia1 * locals.var_vgsteff__blk840);
        let assign19420_e17492: f64 = (locals.var_t0__blk808 - assign19420_e17491);
        let assign19420_e17494: f64 = (assign19420_e17492 - 0.0001);
        (assign19420_e17494, (locals.var_t0__blk808_dn3 - ((locals.var_pparam_b4soia1_dn3 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soia1 * locals.var_vgsteff__blk840_dn3))), (locals.var_t0__blk808_dn4 - ((locals.var_pparam_b4soia1_dn4 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soia1 * locals.var_vgsteff__blk840_dn4))), (locals.var_t0__blk808_dn5 - ((locals.var_pparam_b4soia1_dn5 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soia1 * locals.var_vgsteff__blk840_dn5))), (locals.var_t0__blk808_dn6 - ((locals.var_pparam_b4soia1_dn6 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soia1 * locals.var_vgsteff__blk840_dn6))), (locals.var_t0__blk808_dn7 - ((locals.var_pparam_b4soia1_dn7 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soia1 * locals.var_vgsteff__blk840_dn7))), (locals.var_t0__blk808_dn8 - ((locals.var_pparam_b4soia1_dn8 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soia1 * locals.var_vgsteff__blk840_dn8))), (locals.var_t0__blk808_dn9 - ((locals.var_pparam_b4soia1_dn9 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soia1 * locals.var_vgsteff__blk840_dn9))), (locals.var_t0__blk808_dn10 - ((locals.var_pparam_b4soia1_dn10 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soia1 * locals.var_vgsteff__blk840_dn10))), (locals.var_t0__blk808_dn11 - ((locals.var_pparam_b4soia1_dn11 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soia1 * locals.var_vgsteff__blk840_dn11))), (locals.var_t0__blk808_dn12 - ((locals.var_pparam_b4soia1_dn12 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soia1 * locals.var_vgsteff__blk840_dn12))),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign19420_e17496;
        locals.var_t1__blk809_dn3 = assign19420_e17496_d_n3;
        locals.var_t1__blk809_dn4 = assign19420_e17496_d_n4;
        locals.var_t1__blk809_dn5 = assign19420_e17496_d_n5;
        locals.var_t1__blk809_dn6 = assign19420_e17496_d_n6;
        locals.var_t1__blk809_dn7 = assign19420_e17496_d_n7;
        locals.var_t1__blk809_dn8 = assign19420_e17496_d_n8;
        locals.var_t1__blk809_dn9 = assign19420_e17496_d_n9;
        locals.var_t1__blk809_dn10 = assign19420_e17496_d_n10;
        locals.var_t1__blk809_dn11 = assign19420_e17496_d_n11;
        locals.var_t1__blk809_dn12 = assign19420_e17496_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign19430_e17510, assign19430_e17510_d_n3, assign19430_e17510_d_n4, assign19430_e17510_d_n5, assign19430_e17510_d_n6, assign19430_e17510_d_n7, assign19430_e17510_d_n8, assign19430_e17510_d_n9, assign19430_e17510_d_n10, assign19430_e17510_d_n11, assign19430_e17510_d_n12,) = {
    if ((locals.var_guard1229 == 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign19430_e17503: f64 = (locals.var_t1__blk809 * locals.var_t1__blk809);
        let assign19430_e17506: f64 = (0.0004 * locals.var_t0__blk808);
        let assign19430_e17507: f64 = (assign19430_e17503 + assign19430_e17506);
        let assign19430_e17508: f64 = (assign19430_e17507).sqrt();
        (assign19430_e17508, ((((locals.var_t1__blk809_dn3 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn3)) + (0.0004 * locals.var_t0__blk808_dn3)) / (2.0 * assign19430_e17508)), ((((locals.var_t1__blk809_dn4 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn4)) + (0.0004 * locals.var_t0__blk808_dn4)) / (2.0 * assign19430_e17508)), ((((locals.var_t1__blk809_dn5 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn5)) + (0.0004 * locals.var_t0__blk808_dn5)) / (2.0 * assign19430_e17508)), ((((locals.var_t1__blk809_dn6 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn6)) + (0.0004 * locals.var_t0__blk808_dn6)) / (2.0 * assign19430_e17508)), ((((locals.var_t1__blk809_dn7 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn7)) + (0.0004 * locals.var_t0__blk808_dn7)) / (2.0 * assign19430_e17508)), ((((locals.var_t1__blk809_dn8 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn8)) + (0.0004 * locals.var_t0__blk808_dn8)) / (2.0 * assign19430_e17508)), ((((locals.var_t1__blk809_dn9 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn9)) + (0.0004 * locals.var_t0__blk808_dn9)) / (2.0 * assign19430_e17508)), ((((locals.var_t1__blk809_dn10 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn10)) + (0.0004 * locals.var_t0__blk808_dn10)) / (2.0 * assign19430_e17508)), ((((locals.var_t1__blk809_dn11 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn11)) + (0.0004 * locals.var_t0__blk808_dn11)) / (2.0 * assign19430_e17508)), ((((locals.var_t1__blk809_dn12 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn12)) + (0.0004 * locals.var_t0__blk808_dn12)) / (2.0 * assign19430_e17508)),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign19430_e17510;
        locals.var_t2__blk810_dn3 = assign19430_e17510_d_n3;
        locals.var_t2__blk810_dn4 = assign19430_e17510_d_n4;
        locals.var_t2__blk810_dn5 = assign19430_e17510_d_n5;
        locals.var_t2__blk810_dn6 = assign19430_e17510_d_n6;
        locals.var_t2__blk810_dn7 = assign19430_e17510_d_n7;
        locals.var_t2__blk810_dn8 = assign19430_e17510_d_n8;
        locals.var_t2__blk810_dn9 = assign19430_e17510_d_n9;
        locals.var_t2__blk810_dn10 = assign19430_e17510_d_n10;
        locals.var_t2__blk810_dn11 = assign19430_e17510_d_n11;
        locals.var_t2__blk810_dn12 = assign19430_e17510_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign19440_e17525, assign19440_e17525_d_n3, assign19440_e17525_d_n4, assign19440_e17525_d_n5, assign19440_e17525_d_n6, assign19440_e17525_d_n7, assign19440_e17525_d_n8, assign19440_e17525_d_n9, assign19440_e17525_d_n10, assign19440_e17525_d_n11, assign19440_e17525_d_n12,) = {
    if ((locals.var_guard1229 == 0.0) && (locals.var_guard1230 != 0.0)) {
        let assign19440_e17517: f64 = (locals.var_pparam_b4soia2 + locals.var_t0__blk808);
        let assign19440_e17521: f64 = (locals.var_t1__blk809 + locals.var_t2__blk810);
        let assign19440_e17522: f64 = (0.5 * assign19440_e17521);
        let assign19440_e17523: f64 = (assign19440_e17517 - assign19440_e17522);
        (assign19440_e17523, ((locals.var_pparam_b4soia2_dn3 + locals.var_t0__blk808_dn3) - (0.5 * (locals.var_t1__blk809_dn3 + locals.var_t2__blk810_dn3))), ((locals.var_pparam_b4soia2_dn4 + locals.var_t0__blk808_dn4) - (0.5 * (locals.var_t1__blk809_dn4 + locals.var_t2__blk810_dn4))), ((locals.var_pparam_b4soia2_dn5 + locals.var_t0__blk808_dn5) - (0.5 * (locals.var_t1__blk809_dn5 + locals.var_t2__blk810_dn5))), ((locals.var_pparam_b4soia2_dn6 + locals.var_t0__blk808_dn6) - (0.5 * (locals.var_t1__blk809_dn6 + locals.var_t2__blk810_dn6))), ((locals.var_pparam_b4soia2_dn7 + locals.var_t0__blk808_dn7) - (0.5 * (locals.var_t1__blk809_dn7 + locals.var_t2__blk810_dn7))), ((locals.var_pparam_b4soia2_dn8 + locals.var_t0__blk808_dn8) - (0.5 * (locals.var_t1__blk809_dn8 + locals.var_t2__blk810_dn8))), ((locals.var_pparam_b4soia2_dn9 + locals.var_t0__blk808_dn9) - (0.5 * (locals.var_t1__blk809_dn9 + locals.var_t2__blk810_dn9))), ((locals.var_pparam_b4soia2_dn10 + locals.var_t0__blk808_dn10) - (0.5 * (locals.var_t1__blk809_dn10 + locals.var_t2__blk810_dn10))), ((locals.var_pparam_b4soia2_dn11 + locals.var_t0__blk808_dn11) - (0.5 * (locals.var_t1__blk809_dn11 + locals.var_t2__blk810_dn11))), ((locals.var_pparam_b4soia2_dn12 + locals.var_t0__blk808_dn12) - (0.5 * (locals.var_t1__blk809_dn12 + locals.var_t2__blk810_dn12))),)
    } else {
        (locals.var_lambda, locals.var_lambda_dn3, locals.var_lambda_dn4, locals.var_lambda_dn5, locals.var_lambda_dn6, locals.var_lambda_dn7, locals.var_lambda_dn8, locals.var_lambda_dn9, locals.var_lambda_dn10, locals.var_lambda_dn11, locals.var_lambda_dn12,)
    }
};
        locals.var_lambda = assign19440_e17525;
        locals.var_lambda_dn3 = assign19440_e17525_d_n3;
        locals.var_lambda_dn4 = assign19440_e17525_d_n4;
        locals.var_lambda_dn5 = assign19440_e17525_d_n5;
        locals.var_lambda_dn6 = assign19440_e17525_d_n6;
        locals.var_lambda_dn7 = assign19440_e17525_d_n7;
        locals.var_lambda_dn8 = assign19440_e17525_d_n8;
        locals.var_lambda_dn9 = assign19440_e17525_d_n9;
        locals.var_lambda_dn10 = assign19440_e17525_d_n10;
        locals.var_lambda_dn11 = assign19440_e17525_d_n11;
        locals.var_lambda_dn12 = assign19440_e17525_d_n12;
        locals.var_lambda_rv = 0.0;

        let (assign19450_e17539, assign19450_e17539_d_n3, assign19450_e17539_d_n4, assign19450_e17539_d_n5, assign19450_e17539_d_n6, assign19450_e17539_d_n7, assign19450_e17539_d_n8, assign19450_e17539_d_n9, assign19450_e17539_d_n10, assign19450_e17539_d_n11, assign19450_e17539_d_n12,) = {
    if ((locals.var_guard1229 == 0.0) && (locals.var_guard1230 == 0.0)) {
        let assign19450_e17534: f64 = (locals.var_pparam_b4soia1 * locals.var_vgsteff__blk840);
        let assign19450_e17535: f64 = (locals.var_pparam_b4soia2 + assign19450_e17534);
        let assign19450_e17537: f64 = (assign19450_e17535 - 0.0001);
        (assign19450_e17537, (locals.var_pparam_b4soia2_dn3 + ((locals.var_pparam_b4soia1_dn3 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soia1 * locals.var_vgsteff__blk840_dn3))), (locals.var_pparam_b4soia2_dn4 + ((locals.var_pparam_b4soia1_dn4 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soia1 * locals.var_vgsteff__blk840_dn4))), (locals.var_pparam_b4soia2_dn5 + ((locals.var_pparam_b4soia1_dn5 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soia1 * locals.var_vgsteff__blk840_dn5))), (locals.var_pparam_b4soia2_dn6 + ((locals.var_pparam_b4soia1_dn6 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soia1 * locals.var_vgsteff__blk840_dn6))), (locals.var_pparam_b4soia2_dn7 + ((locals.var_pparam_b4soia1_dn7 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soia1 * locals.var_vgsteff__blk840_dn7))), (locals.var_pparam_b4soia2_dn8 + ((locals.var_pparam_b4soia1_dn8 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soia1 * locals.var_vgsteff__blk840_dn8))), (locals.var_pparam_b4soia2_dn9 + ((locals.var_pparam_b4soia1_dn9 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soia1 * locals.var_vgsteff__blk840_dn9))), (locals.var_pparam_b4soia2_dn10 + ((locals.var_pparam_b4soia1_dn10 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soia1 * locals.var_vgsteff__blk840_dn10))), (locals.var_pparam_b4soia2_dn11 + ((locals.var_pparam_b4soia1_dn11 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soia1 * locals.var_vgsteff__blk840_dn11))), (locals.var_pparam_b4soia2_dn12 + ((locals.var_pparam_b4soia1_dn12 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soia1 * locals.var_vgsteff__blk840_dn12))),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign19450_e17539;
        locals.var_t1__blk809_dn3 = assign19450_e17539_d_n3;
        locals.var_t1__blk809_dn4 = assign19450_e17539_d_n4;
        locals.var_t1__blk809_dn5 = assign19450_e17539_d_n5;
        locals.var_t1__blk809_dn6 = assign19450_e17539_d_n6;
        locals.var_t1__blk809_dn7 = assign19450_e17539_d_n7;
        locals.var_t1__blk809_dn8 = assign19450_e17539_d_n8;
        locals.var_t1__blk809_dn9 = assign19450_e17539_d_n9;
        locals.var_t1__blk809_dn10 = assign19450_e17539_d_n10;
        locals.var_t1__blk809_dn11 = assign19450_e17539_d_n11;
        locals.var_t1__blk809_dn12 = assign19450_e17539_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign19460_e17554, assign19460_e17554_d_n3, assign19460_e17554_d_n4, assign19460_e17554_d_n5, assign19460_e17554_d_n6, assign19460_e17554_d_n7, assign19460_e17554_d_n8, assign19460_e17554_d_n9, assign19460_e17554_d_n10, assign19460_e17554_d_n11, assign19460_e17554_d_n12,) = {
    if ((locals.var_guard1229 == 0.0) && (locals.var_guard1230 == 0.0)) {
        let assign19460_e17547: f64 = (locals.var_t1__blk809 * locals.var_t1__blk809);
        let assign19460_e17550: f64 = (0.0004 * locals.var_pparam_b4soia2);
        let assign19460_e17551: f64 = (assign19460_e17547 + assign19460_e17550);
        let assign19460_e17552: f64 = (assign19460_e17551).sqrt();
        (assign19460_e17552, ((((locals.var_t1__blk809_dn3 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn3)) + (0.0004 * locals.var_pparam_b4soia2_dn3)) / (2.0 * assign19460_e17552)), ((((locals.var_t1__blk809_dn4 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn4)) + (0.0004 * locals.var_pparam_b4soia2_dn4)) / (2.0 * assign19460_e17552)), ((((locals.var_t1__blk809_dn5 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn5)) + (0.0004 * locals.var_pparam_b4soia2_dn5)) / (2.0 * assign19460_e17552)), ((((locals.var_t1__blk809_dn6 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn6)) + (0.0004 * locals.var_pparam_b4soia2_dn6)) / (2.0 * assign19460_e17552)), ((((locals.var_t1__blk809_dn7 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn7)) + (0.0004 * locals.var_pparam_b4soia2_dn7)) / (2.0 * assign19460_e17552)), ((((locals.var_t1__blk809_dn8 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn8)) + (0.0004 * locals.var_pparam_b4soia2_dn8)) / (2.0 * assign19460_e17552)), ((((locals.var_t1__blk809_dn9 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn9)) + (0.0004 * locals.var_pparam_b4soia2_dn9)) / (2.0 * assign19460_e17552)), ((((locals.var_t1__blk809_dn10 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn10)) + (0.0004 * locals.var_pparam_b4soia2_dn10)) / (2.0 * assign19460_e17552)), ((((locals.var_t1__blk809_dn11 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn11)) + (0.0004 * locals.var_pparam_b4soia2_dn11)) / (2.0 * assign19460_e17552)), ((((locals.var_t1__blk809_dn12 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn12)) + (0.0004 * locals.var_pparam_b4soia2_dn12)) / (2.0 * assign19460_e17552)),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign19460_e17554;
        locals.var_t2__blk810_dn3 = assign19460_e17554_d_n3;
        locals.var_t2__blk810_dn4 = assign19460_e17554_d_n4;
        locals.var_t2__blk810_dn5 = assign19460_e17554_d_n5;
        locals.var_t2__blk810_dn6 = assign19460_e17554_d_n6;
        locals.var_t2__blk810_dn7 = assign19460_e17554_d_n7;
        locals.var_t2__blk810_dn8 = assign19460_e17554_d_n8;
        locals.var_t2__blk810_dn9 = assign19460_e17554_d_n9;
        locals.var_t2__blk810_dn10 = assign19460_e17554_d_n10;
        locals.var_t2__blk810_dn11 = assign19460_e17554_d_n11;
        locals.var_t2__blk810_dn12 = assign19460_e17554_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign19470_e17566, assign19470_e17566_d_n3, assign19470_e17566_d_n4, assign19470_e17566_d_n5, assign19470_e17566_d_n6, assign19470_e17566_d_n7, assign19470_e17566_d_n8, assign19470_e17566_d_n9, assign19470_e17566_d_n10, assign19470_e17566_d_n11, assign19470_e17566_d_n12,) = {
    if ((locals.var_guard1229 == 0.0) && (locals.var_guard1230 == 0.0)) {
        let assign19470_e17563: f64 = (locals.var_t1__blk809 + locals.var_t2__blk810);
        let assign19470_e17564: f64 = (0.5 * assign19470_e17563);
        (assign19470_e17564, (0.5 * (locals.var_t1__blk809_dn3 + locals.var_t2__blk810_dn3)), (0.5 * (locals.var_t1__blk809_dn4 + locals.var_t2__blk810_dn4)), (0.5 * (locals.var_t1__blk809_dn5 + locals.var_t2__blk810_dn5)), (0.5 * (locals.var_t1__blk809_dn6 + locals.var_t2__blk810_dn6)), (0.5 * (locals.var_t1__blk809_dn7 + locals.var_t2__blk810_dn7)), (0.5 * (locals.var_t1__blk809_dn8 + locals.var_t2__blk810_dn8)), (0.5 * (locals.var_t1__blk809_dn9 + locals.var_t2__blk810_dn9)), (0.5 * (locals.var_t1__blk809_dn10 + locals.var_t2__blk810_dn10)), (0.5 * (locals.var_t1__blk809_dn11 + locals.var_t2__blk810_dn11)), (0.5 * (locals.var_t1__blk809_dn12 + locals.var_t2__blk810_dn12)),)
    } else {
        (locals.var_lambda, locals.var_lambda_dn3, locals.var_lambda_dn4, locals.var_lambda_dn5, locals.var_lambda_dn6, locals.var_lambda_dn7, locals.var_lambda_dn8, locals.var_lambda_dn9, locals.var_lambda_dn10, locals.var_lambda_dn11, locals.var_lambda_dn12,)
    }
};
        locals.var_lambda = assign19470_e17566;
        locals.var_lambda_dn3 = assign19470_e17566_d_n3;
        locals.var_lambda_dn4 = assign19470_e17566_d_n4;
        locals.var_lambda_dn5 = assign19470_e17566_d_n5;
        locals.var_lambda_dn6 = assign19470_e17566_d_n6;
        locals.var_lambda_dn7 = assign19470_e17566_d_n7;
        locals.var_lambda_dn8 = assign19470_e17566_d_n8;
        locals.var_lambda_dn9 = assign19470_e17566_d_n9;
        locals.var_lambda_dn10 = assign19470_e17566_d_n10;
        locals.var_lambda_dn11 = assign19470_e17566_d_n11;
        locals.var_lambda_dn12 = assign19470_e17566_d_n12;
        locals.var_lambda_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_59(
        locals: &mut StampLocals,
    ) {
        let assign19480_e17569: f64 = (locals.var_abulk / locals.var_vgst2vtm);
        locals.var_b4soiabovvgst2vtm = assign19480_e17569;
        locals.var_b4soiabovvgst2vtm_dn3 = (((locals.var_abulk_dn3 * locals.var_vgst2vtm) - (locals.var_abulk * locals.var_vgst2vtm_dn3)) / (locals.var_vgst2vtm * locals.var_vgst2vtm));
        locals.var_b4soiabovvgst2vtm_dn4 = (((locals.var_abulk_dn4 * locals.var_vgst2vtm) - (locals.var_abulk * locals.var_vgst2vtm_dn4)) / (locals.var_vgst2vtm * locals.var_vgst2vtm));
        locals.var_b4soiabovvgst2vtm_dn5 = (((locals.var_abulk_dn5 * locals.var_vgst2vtm) - (locals.var_abulk * locals.var_vgst2vtm_dn5)) / (locals.var_vgst2vtm * locals.var_vgst2vtm));
        locals.var_b4soiabovvgst2vtm_dn6 = (((locals.var_abulk_dn6 * locals.var_vgst2vtm) - (locals.var_abulk * locals.var_vgst2vtm_dn6)) / (locals.var_vgst2vtm * locals.var_vgst2vtm));
        locals.var_b4soiabovvgst2vtm_dn7 = (((locals.var_abulk_dn7 * locals.var_vgst2vtm) - (locals.var_abulk * locals.var_vgst2vtm_dn7)) / (locals.var_vgst2vtm * locals.var_vgst2vtm));
        locals.var_b4soiabovvgst2vtm_dn8 = (((locals.var_abulk_dn8 * locals.var_vgst2vtm) - (locals.var_abulk * locals.var_vgst2vtm_dn8)) / (locals.var_vgst2vtm * locals.var_vgst2vtm));
        locals.var_b4soiabovvgst2vtm_dn9 = (((locals.var_abulk_dn9 * locals.var_vgst2vtm) - (locals.var_abulk * locals.var_vgst2vtm_dn9)) / (locals.var_vgst2vtm * locals.var_vgst2vtm));
        locals.var_b4soiabovvgst2vtm_dn10 = (((locals.var_abulk_dn10 * locals.var_vgst2vtm) - (locals.var_abulk * locals.var_vgst2vtm_dn10)) / (locals.var_vgst2vtm * locals.var_vgst2vtm));
        locals.var_b4soiabovvgst2vtm_dn11 = (((locals.var_abulk_dn11 * locals.var_vgst2vtm) - (locals.var_abulk * locals.var_vgst2vtm_dn11)) / (locals.var_vgst2vtm * locals.var_vgst2vtm));
        locals.var_b4soiabovvgst2vtm_dn12 = (((locals.var_abulk_dn12 * locals.var_vgst2vtm) - (locals.var_abulk * locals.var_vgst2vtm_dn12)) / (locals.var_vgst2vtm * locals.var_vgst2vtm));
        locals.var_b4soiabovvgst2vtm_rv = 0.0;

        let assign19490_e17576: f64 = if ((locals.var_rds == 0.0) && (locals.var_lambda == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1231 = assign19490_e17576;
        locals.var_guard1231_rv = 0.0;

        let (assign19500_e17586, assign19500_e17586_d_n3, assign19500_e17586_d_n4, assign19500_e17586_d_n5, assign19500_e17586_d_n6, assign19500_e17586_d_n7, assign19500_e17586_d_n8, assign19500_e17586_d_n9, assign19500_e17586_d_n10, assign19500_e17586_d_n11, assign19500_e17586_d_n12,) = {
    if (locals.var_guard1231 != 0.0) {
        let assign19500_e17581: f64 = (locals.var_abulk * locals.var_esatl);
        let assign19500_e17583: f64 = (assign19500_e17581 + locals.var_vgst2vtm);
        let assign19500_e17584: f64 = (1.0 / assign19500_e17583);
        (assign19500_e17584, (-((((locals.var_abulk_dn3 * locals.var_esatl) + (locals.var_abulk * locals.var_esatl_dn3)) + locals.var_vgst2vtm_dn3) / (assign19500_e17583 * assign19500_e17583))), (-((((locals.var_abulk_dn4 * locals.var_esatl) + (locals.var_abulk * locals.var_esatl_dn4)) + locals.var_vgst2vtm_dn4) / (assign19500_e17583 * assign19500_e17583))), (-((((locals.var_abulk_dn5 * locals.var_esatl) + (locals.var_abulk * locals.var_esatl_dn5)) + locals.var_vgst2vtm_dn5) / (assign19500_e17583 * assign19500_e17583))), (-((((locals.var_abulk_dn6 * locals.var_esatl) + (locals.var_abulk * locals.var_esatl_dn6)) + locals.var_vgst2vtm_dn6) / (assign19500_e17583 * assign19500_e17583))), (-((((locals.var_abulk_dn7 * locals.var_esatl) + (locals.var_abulk * locals.var_esatl_dn7)) + locals.var_vgst2vtm_dn7) / (assign19500_e17583 * assign19500_e17583))), (-((((locals.var_abulk_dn8 * locals.var_esatl) + (locals.var_abulk * locals.var_esatl_dn8)) + locals.var_vgst2vtm_dn8) / (assign19500_e17583 * assign19500_e17583))), (-((((locals.var_abulk_dn9 * locals.var_esatl) + (locals.var_abulk * locals.var_esatl_dn9)) + locals.var_vgst2vtm_dn9) / (assign19500_e17583 * assign19500_e17583))), (-((((locals.var_abulk_dn10 * locals.var_esatl) + (locals.var_abulk * locals.var_esatl_dn10)) + locals.var_vgst2vtm_dn10) / (assign19500_e17583 * assign19500_e17583))), (-((((locals.var_abulk_dn11 * locals.var_esatl) + (locals.var_abulk * locals.var_esatl_dn11)) + locals.var_vgst2vtm_dn11) / (assign19500_e17583 * assign19500_e17583))), (-((((locals.var_abulk_dn12 * locals.var_esatl) + (locals.var_abulk * locals.var_esatl_dn12)) + locals.var_vgst2vtm_dn12) / (assign19500_e17583 * assign19500_e17583))),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign19500_e17586;
        locals.var_t0__blk808_dn3 = assign19500_e17586_d_n3;
        locals.var_t0__blk808_dn4 = assign19500_e17586_d_n4;
        locals.var_t0__blk808_dn5 = assign19500_e17586_d_n5;
        locals.var_t0__blk808_dn6 = assign19500_e17586_d_n6;
        locals.var_t0__blk808_dn7 = assign19500_e17586_d_n7;
        locals.var_t0__blk808_dn8 = assign19500_e17586_d_n8;
        locals.var_t0__blk808_dn9 = assign19500_e17586_d_n9;
        locals.var_t0__blk808_dn10 = assign19500_e17586_d_n10;
        locals.var_t0__blk808_dn11 = assign19500_e17586_d_n11;
        locals.var_t0__blk808_dn12 = assign19500_e17586_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign19510_e17592, assign19510_e17592_d_n3, assign19510_e17592_d_n4, assign19510_e17592_d_n5, assign19510_e17592_d_n6, assign19510_e17592_d_n7, assign19510_e17592_d_n8, assign19510_e17592_d_n9, assign19510_e17592_d_n10, assign19510_e17592_d_n11, assign19510_e17592_d_n12,) = {
    if (locals.var_guard1231 != 0.0) {
        let assign19510_e17590: f64 = (locals.var_esatl * locals.var_vgst2vtm);
        (assign19510_e17590, ((locals.var_esatl_dn3 * locals.var_vgst2vtm) + (locals.var_esatl * locals.var_vgst2vtm_dn3)), ((locals.var_esatl_dn4 * locals.var_vgst2vtm) + (locals.var_esatl * locals.var_vgst2vtm_dn4)), ((locals.var_esatl_dn5 * locals.var_vgst2vtm) + (locals.var_esatl * locals.var_vgst2vtm_dn5)), ((locals.var_esatl_dn6 * locals.var_vgst2vtm) + (locals.var_esatl * locals.var_vgst2vtm_dn6)), ((locals.var_esatl_dn7 * locals.var_vgst2vtm) + (locals.var_esatl * locals.var_vgst2vtm_dn7)), ((locals.var_esatl_dn8 * locals.var_vgst2vtm) + (locals.var_esatl * locals.var_vgst2vtm_dn8)), ((locals.var_esatl_dn9 * locals.var_vgst2vtm) + (locals.var_esatl * locals.var_vgst2vtm_dn9)), ((locals.var_esatl_dn10 * locals.var_vgst2vtm) + (locals.var_esatl * locals.var_vgst2vtm_dn10)), ((locals.var_esatl_dn11 * locals.var_vgst2vtm) + (locals.var_esatl * locals.var_vgst2vtm_dn11)), ((locals.var_esatl_dn12 * locals.var_vgst2vtm) + (locals.var_esatl * locals.var_vgst2vtm_dn12)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign19510_e17592;
        locals.var_t3__blk811_dn3 = assign19510_e17592_d_n3;
        locals.var_t3__blk811_dn4 = assign19510_e17592_d_n4;
        locals.var_t3__blk811_dn5 = assign19510_e17592_d_n5;
        locals.var_t3__blk811_dn6 = assign19510_e17592_d_n6;
        locals.var_t3__blk811_dn7 = assign19510_e17592_d_n7;
        locals.var_t3__blk811_dn8 = assign19510_e17592_d_n8;
        locals.var_t3__blk811_dn9 = assign19510_e17592_d_n9;
        locals.var_t3__blk811_dn10 = assign19510_e17592_d_n10;
        locals.var_t3__blk811_dn11 = assign19510_e17592_d_n11;
        locals.var_t3__blk811_dn12 = assign19510_e17592_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign19520_e17598, assign19520_e17598_d_n3, assign19520_e17598_d_n4, assign19520_e17598_d_n5, assign19520_e17598_d_n6, assign19520_e17598_d_n7, assign19520_e17598_d_n8, assign19520_e17598_d_n9, assign19520_e17598_d_n10, assign19520_e17598_d_n11, assign19520_e17598_d_n12,) = {
    if (locals.var_guard1231 != 0.0) {
        let assign19520_e17596: f64 = (locals.var_t3__blk811 * locals.var_t0__blk808);
        (assign19520_e17596, ((locals.var_t3__blk811_dn3 * locals.var_t0__blk808) + (locals.var_t3__blk811 * locals.var_t0__blk808_dn3)), ((locals.var_t3__blk811_dn4 * locals.var_t0__blk808) + (locals.var_t3__blk811 * locals.var_t0__blk808_dn4)), ((locals.var_t3__blk811_dn5 * locals.var_t0__blk808) + (locals.var_t3__blk811 * locals.var_t0__blk808_dn5)), ((locals.var_t3__blk811_dn6 * locals.var_t0__blk808) + (locals.var_t3__blk811 * locals.var_t0__blk808_dn6)), ((locals.var_t3__blk811_dn7 * locals.var_t0__blk808) + (locals.var_t3__blk811 * locals.var_t0__blk808_dn7)), ((locals.var_t3__blk811_dn8 * locals.var_t0__blk808) + (locals.var_t3__blk811 * locals.var_t0__blk808_dn8)), ((locals.var_t3__blk811_dn9 * locals.var_t0__blk808) + (locals.var_t3__blk811 * locals.var_t0__blk808_dn9)), ((locals.var_t3__blk811_dn10 * locals.var_t0__blk808) + (locals.var_t3__blk811 * locals.var_t0__blk808_dn10)), ((locals.var_t3__blk811_dn11 * locals.var_t0__blk808) + (locals.var_t3__blk811 * locals.var_t0__blk808_dn11)), ((locals.var_t3__blk811_dn12 * locals.var_t0__blk808) + (locals.var_t3__blk811 * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_vdsat_1, locals.var_vdsat_1_dn3, locals.var_vdsat_1_dn4, locals.var_vdsat_1_dn5, locals.var_vdsat_1_dn6, locals.var_vdsat_1_dn7, locals.var_vdsat_1_dn8, locals.var_vdsat_1_dn9, locals.var_vdsat_1_dn10, locals.var_vdsat_1_dn11, locals.var_vdsat_1_dn12,)
    }
};
        locals.var_vdsat_1 = assign19520_e17598;
        locals.var_vdsat_1_dn3 = assign19520_e17598_d_n3;
        locals.var_vdsat_1_dn4 = assign19520_e17598_d_n4;
        locals.var_vdsat_1_dn5 = assign19520_e17598_d_n5;
        locals.var_vdsat_1_dn6 = assign19520_e17598_d_n6;
        locals.var_vdsat_1_dn7 = assign19520_e17598_d_n7;
        locals.var_vdsat_1_dn8 = assign19520_e17598_d_n8;
        locals.var_vdsat_1_dn9 = assign19520_e17598_d_n9;
        locals.var_vdsat_1_dn10 = assign19520_e17598_d_n10;
        locals.var_vdsat_1_dn11 = assign19520_e17598_d_n11;
        locals.var_vdsat_1_dn12 = assign19520_e17598_d_n12;
        locals.var_vdsat_1_rv = 0.0;

        let (assign19530_e17605, assign19530_e17605_d_n3, assign19530_e17605_d_n4, assign19530_e17605_d_n5, assign19530_e17605_d_n6, assign19530_e17605_d_n7, assign19530_e17605_d_n8, assign19530_e17605_d_n9, assign19530_e17605_d_n10, assign19530_e17605_d_n11, assign19530_e17605_d_n12,) = {
    if (locals.var_guard1231 == 0.0) {
        let assign19530_e17603: f64 = (locals.var_abulk * locals.var_wvcoxrds);
        (assign19530_e17603, ((locals.var_abulk_dn3 * locals.var_wvcoxrds) + (locals.var_abulk * locals.var_wvcoxrds_dn3)), ((locals.var_abulk_dn4 * locals.var_wvcoxrds) + (locals.var_abulk * locals.var_wvcoxrds_dn4)), ((locals.var_abulk_dn5 * locals.var_wvcoxrds) + (locals.var_abulk * locals.var_wvcoxrds_dn5)), ((locals.var_abulk_dn6 * locals.var_wvcoxrds) + (locals.var_abulk * locals.var_wvcoxrds_dn6)), ((locals.var_abulk_dn7 * locals.var_wvcoxrds) + (locals.var_abulk * locals.var_wvcoxrds_dn7)), ((locals.var_abulk_dn8 * locals.var_wvcoxrds) + (locals.var_abulk * locals.var_wvcoxrds_dn8)), ((locals.var_abulk_dn9 * locals.var_wvcoxrds) + (locals.var_abulk * locals.var_wvcoxrds_dn9)), ((locals.var_abulk_dn10 * locals.var_wvcoxrds) + (locals.var_abulk * locals.var_wvcoxrds_dn10)), ((locals.var_abulk_dn11 * locals.var_wvcoxrds) + (locals.var_abulk * locals.var_wvcoxrds_dn11)), ((locals.var_abulk_dn12 * locals.var_wvcoxrds) + (locals.var_abulk * locals.var_wvcoxrds_dn12)),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign19530_e17605;
        locals.var_t9_dn3 = assign19530_e17605_d_n3;
        locals.var_t9_dn4 = assign19530_e17605_d_n4;
        locals.var_t9_dn5 = assign19530_e17605_d_n5;
        locals.var_t9_dn6 = assign19530_e17605_d_n6;
        locals.var_t9_dn7 = assign19530_e17605_d_n7;
        locals.var_t9_dn8 = assign19530_e17605_d_n8;
        locals.var_t9_dn9 = assign19530_e17605_d_n9;
        locals.var_t9_dn10 = assign19530_e17605_d_n10;
        locals.var_t9_dn11 = assign19530_e17605_d_n11;
        locals.var_t9_dn12 = assign19530_e17605_d_n12;
        locals.var_t9_rv = 0.0;

        let (assign19540_e17612, assign19540_e17612_d_n3, assign19540_e17612_d_n4, assign19540_e17612_d_n5, assign19540_e17612_d_n6, assign19540_e17612_d_n7, assign19540_e17612_d_n8, assign19540_e17612_d_n9, assign19540_e17612_d_n10, assign19540_e17612_d_n11, assign19540_e17612_d_n12,) = {
    if (locals.var_guard1231 == 0.0) {
        let assign19540_e17610: f64 = (locals.var_vgst2vtm * locals.var_t9);
        (assign19540_e17610, ((locals.var_vgst2vtm_dn3 * locals.var_t9) + (locals.var_vgst2vtm * locals.var_t9_dn3)), ((locals.var_vgst2vtm_dn4 * locals.var_t9) + (locals.var_vgst2vtm * locals.var_t9_dn4)), ((locals.var_vgst2vtm_dn5 * locals.var_t9) + (locals.var_vgst2vtm * locals.var_t9_dn5)), ((locals.var_vgst2vtm_dn6 * locals.var_t9) + (locals.var_vgst2vtm * locals.var_t9_dn6)), ((locals.var_vgst2vtm_dn7 * locals.var_t9) + (locals.var_vgst2vtm * locals.var_t9_dn7)), ((locals.var_vgst2vtm_dn8 * locals.var_t9) + (locals.var_vgst2vtm * locals.var_t9_dn8)), ((locals.var_vgst2vtm_dn9 * locals.var_t9) + (locals.var_vgst2vtm * locals.var_t9_dn9)), ((locals.var_vgst2vtm_dn10 * locals.var_t9) + (locals.var_vgst2vtm * locals.var_t9_dn10)), ((locals.var_vgst2vtm_dn11 * locals.var_t9) + (locals.var_vgst2vtm * locals.var_t9_dn11)), ((locals.var_vgst2vtm_dn12 * locals.var_t9) + (locals.var_vgst2vtm * locals.var_t9_dn12)),)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign19540_e17612;
        locals.var_t7__blk815_dn3 = assign19540_e17612_d_n3;
        locals.var_t7__blk815_dn4 = assign19540_e17612_d_n4;
        locals.var_t7__blk815_dn5 = assign19540_e17612_d_n5;
        locals.var_t7__blk815_dn6 = assign19540_e17612_d_n6;
        locals.var_t7__blk815_dn7 = assign19540_e17612_d_n7;
        locals.var_t7__blk815_dn8 = assign19540_e17612_d_n8;
        locals.var_t7__blk815_dn9 = assign19540_e17612_d_n9;
        locals.var_t7__blk815_dn10 = assign19540_e17612_d_n10;
        locals.var_t7__blk815_dn11 = assign19540_e17612_d_n11;
        locals.var_t7__blk815_dn12 = assign19540_e17612_d_n12;
        locals.var_t7__blk815_rv = 0.0;

        let (assign19550_e17619, assign19550_e17619_d_n3, assign19550_e17619_d_n4, assign19550_e17619_d_n5, assign19550_e17619_d_n6, assign19550_e17619_d_n7, assign19550_e17619_d_n8, assign19550_e17619_d_n9, assign19550_e17619_d_n10, assign19550_e17619_d_n11, assign19550_e17619_d_n12,) = {
    if (locals.var_guard1231 == 0.0) {
        let assign19550_e17617: f64 = (locals.var_vgst2vtm * locals.var_wvcoxrds);
        (assign19550_e17617, ((locals.var_vgst2vtm_dn3 * locals.var_wvcoxrds) + (locals.var_vgst2vtm * locals.var_wvcoxrds_dn3)), ((locals.var_vgst2vtm_dn4 * locals.var_wvcoxrds) + (locals.var_vgst2vtm * locals.var_wvcoxrds_dn4)), ((locals.var_vgst2vtm_dn5 * locals.var_wvcoxrds) + (locals.var_vgst2vtm * locals.var_wvcoxrds_dn5)), ((locals.var_vgst2vtm_dn6 * locals.var_wvcoxrds) + (locals.var_vgst2vtm * locals.var_wvcoxrds_dn6)), ((locals.var_vgst2vtm_dn7 * locals.var_wvcoxrds) + (locals.var_vgst2vtm * locals.var_wvcoxrds_dn7)), ((locals.var_vgst2vtm_dn8 * locals.var_wvcoxrds) + (locals.var_vgst2vtm * locals.var_wvcoxrds_dn8)), ((locals.var_vgst2vtm_dn9 * locals.var_wvcoxrds) + (locals.var_vgst2vtm * locals.var_wvcoxrds_dn9)), ((locals.var_vgst2vtm_dn10 * locals.var_wvcoxrds) + (locals.var_vgst2vtm * locals.var_wvcoxrds_dn10)), ((locals.var_vgst2vtm_dn11 * locals.var_wvcoxrds) + (locals.var_vgst2vtm * locals.var_wvcoxrds_dn11)), ((locals.var_vgst2vtm_dn12 * locals.var_wvcoxrds) + (locals.var_vgst2vtm * locals.var_wvcoxrds_dn12)),)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign19550_e17619;
        locals.var_t6__blk814_dn3 = assign19550_e17619_d_n3;
        locals.var_t6__blk814_dn4 = assign19550_e17619_d_n4;
        locals.var_t6__blk814_dn5 = assign19550_e17619_d_n5;
        locals.var_t6__blk814_dn6 = assign19550_e17619_d_n6;
        locals.var_t6__blk814_dn7 = assign19550_e17619_d_n7;
        locals.var_t6__blk814_dn8 = assign19550_e17619_d_n8;
        locals.var_t6__blk814_dn9 = assign19550_e17619_d_n9;
        locals.var_t6__blk814_dn10 = assign19550_e17619_d_n10;
        locals.var_t6__blk814_dn11 = assign19550_e17619_d_n11;
        locals.var_t6__blk814_dn12 = assign19550_e17619_d_n12;
        locals.var_t6__blk814_rv = 0.0;

        let (assign19560_e17634, assign19560_e17634_d_n3, assign19560_e17634_d_n4, assign19560_e17634_d_n5, assign19560_e17634_d_n6, assign19560_e17634_d_n7, assign19560_e17634_d_n8, assign19560_e17634_d_n9, assign19560_e17634_d_n10, assign19560_e17634_d_n11, assign19560_e17634_d_n12,) = {
    if (locals.var_guard1231 == 0.0) {
        let assign19560_e17624: f64 = (2.0 * locals.var_abulk);
        let assign19560_e17627: f64 = (locals.var_t9 - 1.0);
        let assign19560_e17630: f64 = (1.0 / locals.var_lambda);
        let assign19560_e17631: f64 = (assign19560_e17627 + assign19560_e17630);
        let assign19560_e17632: f64 = (assign19560_e17624 * assign19560_e17631);
        (assign19560_e17632, (((2.0 * locals.var_abulk_dn3) * assign19560_e17631) + (assign19560_e17624 * (locals.var_t9_dn3 + (-(locals.var_lambda_dn3 / (locals.var_lambda * locals.var_lambda)))))), (((2.0 * locals.var_abulk_dn4) * assign19560_e17631) + (assign19560_e17624 * (locals.var_t9_dn4 + (-(locals.var_lambda_dn4 / (locals.var_lambda * locals.var_lambda)))))), (((2.0 * locals.var_abulk_dn5) * assign19560_e17631) + (assign19560_e17624 * (locals.var_t9_dn5 + (-(locals.var_lambda_dn5 / (locals.var_lambda * locals.var_lambda)))))), (((2.0 * locals.var_abulk_dn6) * assign19560_e17631) + (assign19560_e17624 * (locals.var_t9_dn6 + (-(locals.var_lambda_dn6 / (locals.var_lambda * locals.var_lambda)))))), (((2.0 * locals.var_abulk_dn7) * assign19560_e17631) + (assign19560_e17624 * (locals.var_t9_dn7 + (-(locals.var_lambda_dn7 / (locals.var_lambda * locals.var_lambda)))))), (((2.0 * locals.var_abulk_dn8) * assign19560_e17631) + (assign19560_e17624 * (locals.var_t9_dn8 + (-(locals.var_lambda_dn8 / (locals.var_lambda * locals.var_lambda)))))), (((2.0 * locals.var_abulk_dn9) * assign19560_e17631) + (assign19560_e17624 * (locals.var_t9_dn9 + (-(locals.var_lambda_dn9 / (locals.var_lambda * locals.var_lambda)))))), (((2.0 * locals.var_abulk_dn10) * assign19560_e17631) + (assign19560_e17624 * (locals.var_t9_dn10 + (-(locals.var_lambda_dn10 / (locals.var_lambda * locals.var_lambda)))))), (((2.0 * locals.var_abulk_dn11) * assign19560_e17631) + (assign19560_e17624 * (locals.var_t9_dn11 + (-(locals.var_lambda_dn11 / (locals.var_lambda * locals.var_lambda)))))), (((2.0 * locals.var_abulk_dn12) * assign19560_e17631) + (assign19560_e17624 * (locals.var_t9_dn12 + (-(locals.var_lambda_dn12 / (locals.var_lambda * locals.var_lambda)))))),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign19560_e17634;
        locals.var_t0__blk808_dn3 = assign19560_e17634_d_n3;
        locals.var_t0__blk808_dn4 = assign19560_e17634_d_n4;
        locals.var_t0__blk808_dn5 = assign19560_e17634_d_n5;
        locals.var_t0__blk808_dn6 = assign19560_e17634_d_n6;
        locals.var_t0__blk808_dn7 = assign19560_e17634_d_n7;
        locals.var_t0__blk808_dn8 = assign19560_e17634_d_n8;
        locals.var_t0__blk808_dn9 = assign19560_e17634_d_n9;
        locals.var_t0__blk808_dn10 = assign19560_e17634_d_n10;
        locals.var_t0__blk808_dn11 = assign19560_e17634_d_n11;
        locals.var_t0__blk808_dn12 = assign19560_e17634_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign19570_e17653, assign19570_e17653_d_n3, assign19570_e17653_d_n4, assign19570_e17653_d_n5, assign19570_e17653_d_n6, assign19570_e17653_d_n7, assign19570_e17653_d_n8, assign19570_e17653_d_n9, assign19570_e17653_d_n10, assign19570_e17653_d_n11, assign19570_e17653_d_n12,) = {
    if (locals.var_guard1231 == 0.0) {
        let assign19570_e17640: f64 = (2.0 / locals.var_lambda);
        let assign19570_e17642: f64 = (assign19570_e17640 - 1.0);
        let assign19570_e17643: f64 = (locals.var_vgst2vtm * assign19570_e17642);
        let assign19570_e17646: f64 = (locals.var_abulk * locals.var_esatl);
        let assign19570_e17647: f64 = (assign19570_e17643 + assign19570_e17646);
        let assign19570_e17650: f64 = (3.0 * locals.var_t7__blk815);
        let assign19570_e17651: f64 = (assign19570_e17647 + assign19570_e17650);
        (assign19570_e17651, ((((locals.var_vgst2vtm_dn3 * assign19570_e17642) + (locals.var_vgst2vtm * (-((2.0 * locals.var_lambda_dn3) / (locals.var_lambda * locals.var_lambda))))) + ((locals.var_abulk_dn3 * locals.var_esatl) + (locals.var_abulk * locals.var_esatl_dn3))) + (3.0 * locals.var_t7__blk815_dn3)), ((((locals.var_vgst2vtm_dn4 * assign19570_e17642) + (locals.var_vgst2vtm * (-((2.0 * locals.var_lambda_dn4) / (locals.var_lambda * locals.var_lambda))))) + ((locals.var_abulk_dn4 * locals.var_esatl) + (locals.var_abulk * locals.var_esatl_dn4))) + (3.0 * locals.var_t7__blk815_dn4)), ((((locals.var_vgst2vtm_dn5 * assign19570_e17642) + (locals.var_vgst2vtm * (-((2.0 * locals.var_lambda_dn5) / (locals.var_lambda * locals.var_lambda))))) + ((locals.var_abulk_dn5 * locals.var_esatl) + (locals.var_abulk * locals.var_esatl_dn5))) + (3.0 * locals.var_t7__blk815_dn5)), ((((locals.var_vgst2vtm_dn6 * assign19570_e17642) + (locals.var_vgst2vtm * (-((2.0 * locals.var_lambda_dn6) / (locals.var_lambda * locals.var_lambda))))) + ((locals.var_abulk_dn6 * locals.var_esatl) + (locals.var_abulk * locals.var_esatl_dn6))) + (3.0 * locals.var_t7__blk815_dn6)), ((((locals.var_vgst2vtm_dn7 * assign19570_e17642) + (locals.var_vgst2vtm * (-((2.0 * locals.var_lambda_dn7) / (locals.var_lambda * locals.var_lambda))))) + ((locals.var_abulk_dn7 * locals.var_esatl) + (locals.var_abulk * locals.var_esatl_dn7))) + (3.0 * locals.var_t7__blk815_dn7)), ((((locals.var_vgst2vtm_dn8 * assign19570_e17642) + (locals.var_vgst2vtm * (-((2.0 * locals.var_lambda_dn8) / (locals.var_lambda * locals.var_lambda))))) + ((locals.var_abulk_dn8 * locals.var_esatl) + (locals.var_abulk * locals.var_esatl_dn8))) + (3.0 * locals.var_t7__blk815_dn8)), ((((locals.var_vgst2vtm_dn9 * assign19570_e17642) + (locals.var_vgst2vtm * (-((2.0 * locals.var_lambda_dn9) / (locals.var_lambda * locals.var_lambda))))) + ((locals.var_abulk_dn9 * locals.var_esatl) + (locals.var_abulk * locals.var_esatl_dn9))) + (3.0 * locals.var_t7__blk815_dn9)), ((((locals.var_vgst2vtm_dn10 * assign19570_e17642) + (locals.var_vgst2vtm * (-((2.0 * locals.var_lambda_dn10) / (locals.var_lambda * locals.var_lambda))))) + ((locals.var_abulk_dn10 * locals.var_esatl) + (locals.var_abulk * locals.var_esatl_dn10))) + (3.0 * locals.var_t7__blk815_dn10)), ((((locals.var_vgst2vtm_dn11 * assign19570_e17642) + (locals.var_vgst2vtm * (-((2.0 * locals.var_lambda_dn11) / (locals.var_lambda * locals.var_lambda))))) + ((locals.var_abulk_dn11 * locals.var_esatl) + (locals.var_abulk * locals.var_esatl_dn11))) + (3.0 * locals.var_t7__blk815_dn11)), ((((locals.var_vgst2vtm_dn12 * assign19570_e17642) + (locals.var_vgst2vtm * (-((2.0 * locals.var_lambda_dn12) / (locals.var_lambda * locals.var_lambda))))) + ((locals.var_abulk_dn12 * locals.var_esatl) + (locals.var_abulk * locals.var_esatl_dn12))) + (3.0 * locals.var_t7__blk815_dn12)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign19570_e17653;
        locals.var_t1__blk809_dn3 = assign19570_e17653_d_n3;
        locals.var_t1__blk809_dn4 = assign19570_e17653_d_n4;
        locals.var_t1__blk809_dn5 = assign19570_e17653_d_n5;
        locals.var_t1__blk809_dn6 = assign19570_e17653_d_n6;
        locals.var_t1__blk809_dn7 = assign19570_e17653_d_n7;
        locals.var_t1__blk809_dn8 = assign19570_e17653_d_n8;
        locals.var_t1__blk809_dn9 = assign19570_e17653_d_n9;
        locals.var_t1__blk809_dn10 = assign19570_e17653_d_n10;
        locals.var_t1__blk809_dn11 = assign19570_e17653_d_n11;
        locals.var_t1__blk809_dn12 = assign19570_e17653_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign19580_e17664, assign19580_e17664_d_n3, assign19580_e17664_d_n4, assign19580_e17664_d_n5, assign19580_e17664_d_n6, assign19580_e17664_d_n7, assign19580_e17664_d_n8, assign19580_e17664_d_n9, assign19580_e17664_d_n10, assign19580_e17664_d_n11, assign19580_e17664_d_n12,) = {
    if (locals.var_guard1231 == 0.0) {
        let assign19580_e17660: f64 = (2.0 * locals.var_t6__blk814);
        let assign19580_e17661: f64 = (locals.var_esatl + assign19580_e17660);
        let assign19580_e17662: f64 = (locals.var_vgst2vtm * assign19580_e17661);
        (assign19580_e17662, ((locals.var_vgst2vtm_dn3 * assign19580_e17661) + (locals.var_vgst2vtm * (locals.var_esatl_dn3 + (2.0 * locals.var_t6__blk814_dn3)))), ((locals.var_vgst2vtm_dn4 * assign19580_e17661) + (locals.var_vgst2vtm * (locals.var_esatl_dn4 + (2.0 * locals.var_t6__blk814_dn4)))), ((locals.var_vgst2vtm_dn5 * assign19580_e17661) + (locals.var_vgst2vtm * (locals.var_esatl_dn5 + (2.0 * locals.var_t6__blk814_dn5)))), ((locals.var_vgst2vtm_dn6 * assign19580_e17661) + (locals.var_vgst2vtm * (locals.var_esatl_dn6 + (2.0 * locals.var_t6__blk814_dn6)))), ((locals.var_vgst2vtm_dn7 * assign19580_e17661) + (locals.var_vgst2vtm * (locals.var_esatl_dn7 + (2.0 * locals.var_t6__blk814_dn7)))), ((locals.var_vgst2vtm_dn8 * assign19580_e17661) + (locals.var_vgst2vtm * (locals.var_esatl_dn8 + (2.0 * locals.var_t6__blk814_dn8)))), ((locals.var_vgst2vtm_dn9 * assign19580_e17661) + (locals.var_vgst2vtm * (locals.var_esatl_dn9 + (2.0 * locals.var_t6__blk814_dn9)))), ((locals.var_vgst2vtm_dn10 * assign19580_e17661) + (locals.var_vgst2vtm * (locals.var_esatl_dn10 + (2.0 * locals.var_t6__blk814_dn10)))), ((locals.var_vgst2vtm_dn11 * assign19580_e17661) + (locals.var_vgst2vtm * (locals.var_esatl_dn11 + (2.0 * locals.var_t6__blk814_dn11)))), ((locals.var_vgst2vtm_dn12 * assign19580_e17661) + (locals.var_vgst2vtm * (locals.var_esatl_dn12 + (2.0 * locals.var_t6__blk814_dn12)))),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign19580_e17664;
        locals.var_t2__blk810_dn3 = assign19580_e17664_d_n3;
        locals.var_t2__blk810_dn4 = assign19580_e17664_d_n4;
        locals.var_t2__blk810_dn5 = assign19580_e17664_d_n5;
        locals.var_t2__blk810_dn6 = assign19580_e17664_d_n6;
        locals.var_t2__blk810_dn7 = assign19580_e17664_d_n7;
        locals.var_t2__blk810_dn8 = assign19580_e17664_d_n8;
        locals.var_t2__blk810_dn9 = assign19580_e17664_d_n9;
        locals.var_t2__blk810_dn10 = assign19580_e17664_d_n10;
        locals.var_t2__blk810_dn11 = assign19580_e17664_d_n11;
        locals.var_t2__blk810_dn12 = assign19580_e17664_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign19590_e17678, assign19590_e17678_d_n3, assign19590_e17678_d_n4, assign19590_e17678_d_n5, assign19590_e17678_d_n6, assign19590_e17678_d_n7, assign19590_e17678_d_n8, assign19590_e17678_d_n9, assign19590_e17678_d_n10, assign19590_e17678_d_n11, assign19590_e17678_d_n12,) = {
    if (locals.var_guard1231 == 0.0) {
        let assign19590_e17669: f64 = (locals.var_t1__blk809 * locals.var_t1__blk809);
        let assign19590_e17672: f64 = (2.0 * locals.var_t0__blk808);
        let assign19590_e17674: f64 = (assign19590_e17672 * locals.var_t2__blk810);
        let assign19590_e17675: f64 = (assign19590_e17669 - assign19590_e17674);
        let assign19590_e17676: f64 = (assign19590_e17675).sqrt();
        (assign19590_e17676, ((((locals.var_t1__blk809_dn3 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn3)) - (((2.0 * locals.var_t0__blk808_dn3) * locals.var_t2__blk810) + (assign19590_e17672 * locals.var_t2__blk810_dn3))) / (2.0 * assign19590_e17676)), ((((locals.var_t1__blk809_dn4 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn4)) - (((2.0 * locals.var_t0__blk808_dn4) * locals.var_t2__blk810) + (assign19590_e17672 * locals.var_t2__blk810_dn4))) / (2.0 * assign19590_e17676)), ((((locals.var_t1__blk809_dn5 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn5)) - (((2.0 * locals.var_t0__blk808_dn5) * locals.var_t2__blk810) + (assign19590_e17672 * locals.var_t2__blk810_dn5))) / (2.0 * assign19590_e17676)), ((((locals.var_t1__blk809_dn6 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn6)) - (((2.0 * locals.var_t0__blk808_dn6) * locals.var_t2__blk810) + (assign19590_e17672 * locals.var_t2__blk810_dn6))) / (2.0 * assign19590_e17676)), ((((locals.var_t1__blk809_dn7 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn7)) - (((2.0 * locals.var_t0__blk808_dn7) * locals.var_t2__blk810) + (assign19590_e17672 * locals.var_t2__blk810_dn7))) / (2.0 * assign19590_e17676)), ((((locals.var_t1__blk809_dn8 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn8)) - (((2.0 * locals.var_t0__blk808_dn8) * locals.var_t2__blk810) + (assign19590_e17672 * locals.var_t2__blk810_dn8))) / (2.0 * assign19590_e17676)), ((((locals.var_t1__blk809_dn9 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn9)) - (((2.0 * locals.var_t0__blk808_dn9) * locals.var_t2__blk810) + (assign19590_e17672 * locals.var_t2__blk810_dn9))) / (2.0 * assign19590_e17676)), ((((locals.var_t1__blk809_dn10 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn10)) - (((2.0 * locals.var_t0__blk808_dn10) * locals.var_t2__blk810) + (assign19590_e17672 * locals.var_t2__blk810_dn10))) / (2.0 * assign19590_e17676)), ((((locals.var_t1__blk809_dn11 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn11)) - (((2.0 * locals.var_t0__blk808_dn11) * locals.var_t2__blk810) + (assign19590_e17672 * locals.var_t2__blk810_dn11))) / (2.0 * assign19590_e17676)), ((((locals.var_t1__blk809_dn12 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn12)) - (((2.0 * locals.var_t0__blk808_dn12) * locals.var_t2__blk810) + (assign19590_e17672 * locals.var_t2__blk810_dn12))) / (2.0 * assign19590_e17676)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign19590_e17678;
        locals.var_t3__blk811_dn3 = assign19590_e17678_d_n3;
        locals.var_t3__blk811_dn4 = assign19590_e17678_d_n4;
        locals.var_t3__blk811_dn5 = assign19590_e17678_d_n5;
        locals.var_t3__blk811_dn6 = assign19590_e17678_d_n6;
        locals.var_t3__blk811_dn7 = assign19590_e17678_d_n7;
        locals.var_t3__blk811_dn8 = assign19590_e17678_d_n8;
        locals.var_t3__blk811_dn9 = assign19590_e17678_d_n9;
        locals.var_t3__blk811_dn10 = assign19590_e17678_d_n10;
        locals.var_t3__blk811_dn11 = assign19590_e17678_d_n11;
        locals.var_t3__blk811_dn12 = assign19590_e17678_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign19600_e17687, assign19600_e17687_d_n3, assign19600_e17687_d_n4, assign19600_e17687_d_n5, assign19600_e17687_d_n6, assign19600_e17687_d_n7, assign19600_e17687_d_n8, assign19600_e17687_d_n9, assign19600_e17687_d_n10, assign19600_e17687_d_n11, assign19600_e17687_d_n12,) = {
    if (locals.var_guard1231 == 0.0) {
        let assign19600_e17683: f64 = (locals.var_t1__blk809 - locals.var_t3__blk811);
        let assign19600_e17685: f64 = (assign19600_e17683 / locals.var_t0__blk808);
        (assign19600_e17685, ((((locals.var_t1__blk809_dn3 - locals.var_t3__blk811_dn3) * locals.var_t0__blk808) - (assign19600_e17683 * locals.var_t0__blk808_dn3)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((locals.var_t1__blk809_dn4 - locals.var_t3__blk811_dn4) * locals.var_t0__blk808) - (assign19600_e17683 * locals.var_t0__blk808_dn4)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((locals.var_t1__blk809_dn5 - locals.var_t3__blk811_dn5) * locals.var_t0__blk808) - (assign19600_e17683 * locals.var_t0__blk808_dn5)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((locals.var_t1__blk809_dn6 - locals.var_t3__blk811_dn6) * locals.var_t0__blk808) - (assign19600_e17683 * locals.var_t0__blk808_dn6)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((locals.var_t1__blk809_dn7 - locals.var_t3__blk811_dn7) * locals.var_t0__blk808) - (assign19600_e17683 * locals.var_t0__blk808_dn7)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((locals.var_t1__blk809_dn8 - locals.var_t3__blk811_dn8) * locals.var_t0__blk808) - (assign19600_e17683 * locals.var_t0__blk808_dn8)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((locals.var_t1__blk809_dn9 - locals.var_t3__blk811_dn9) * locals.var_t0__blk808) - (assign19600_e17683 * locals.var_t0__blk808_dn9)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((locals.var_t1__blk809_dn10 - locals.var_t3__blk811_dn10) * locals.var_t0__blk808) - (assign19600_e17683 * locals.var_t0__blk808_dn10)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((locals.var_t1__blk809_dn11 - locals.var_t3__blk811_dn11) * locals.var_t0__blk808) - (assign19600_e17683 * locals.var_t0__blk808_dn11)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((locals.var_t1__blk809_dn12 - locals.var_t3__blk811_dn12) * locals.var_t0__blk808) - (assign19600_e17683 * locals.var_t0__blk808_dn12)) / (locals.var_t0__blk808 * locals.var_t0__blk808)),)
    } else {
        (locals.var_vdsat_1, locals.var_vdsat_1_dn3, locals.var_vdsat_1_dn4, locals.var_vdsat_1_dn5, locals.var_vdsat_1_dn6, locals.var_vdsat_1_dn7, locals.var_vdsat_1_dn8, locals.var_vdsat_1_dn9, locals.var_vdsat_1_dn10, locals.var_vdsat_1_dn11, locals.var_vdsat_1_dn12,)
    }
};
        locals.var_vdsat_1 = assign19600_e17687;
        locals.var_vdsat_1_dn3 = assign19600_e17687_d_n3;
        locals.var_vdsat_1_dn4 = assign19600_e17687_d_n4;
        locals.var_vdsat_1_dn5 = assign19600_e17687_d_n5;
        locals.var_vdsat_1_dn6 = assign19600_e17687_d_n6;
        locals.var_vdsat_1_dn7 = assign19600_e17687_d_n7;
        locals.var_vdsat_1_dn8 = assign19600_e17687_d_n8;
        locals.var_vdsat_1_dn9 = assign19600_e17687_d_n9;
        locals.var_vdsat_1_dn10 = assign19600_e17687_d_n10;
        locals.var_vdsat_1_dn11 = assign19600_e17687_d_n11;
        locals.var_vdsat_1_dn12 = assign19600_e17687_d_n12;
        locals.var_vdsat_1_rv = 0.0;

        let assign19620_e17691: f64 = (locals.var_vdsat_1 - locals.var_vds_1);
        let assign19620_e17693: f64 = (assign19620_e17691 - locals.var_pparam_b4soidelta);
        locals.var_t1__blk809 = assign19620_e17693;
        locals.var_t1__blk809_dn3 = (locals.var_vdsat_1_dn3 - locals.var_pparam_b4soidelta_dn3);
        locals.var_t1__blk809_dn4 = (locals.var_vdsat_1_dn4 - locals.var_pparam_b4soidelta_dn4);
        locals.var_t1__blk809_dn5 = (locals.var_vdsat_1_dn5 - locals.var_pparam_b4soidelta_dn5);
        locals.var_t1__blk809_dn6 = (locals.var_vdsat_1_dn6 - locals.var_pparam_b4soidelta_dn6);
        locals.var_t1__blk809_dn7 = ((locals.var_vdsat_1_dn7 - locals.var_vds_1_dn7) - locals.var_pparam_b4soidelta_dn7);
        locals.var_t1__blk809_dn8 = ((locals.var_vdsat_1_dn8 - locals.var_vds_1_dn8) - locals.var_pparam_b4soidelta_dn8);
        locals.var_t1__blk809_dn9 = (locals.var_vdsat_1_dn9 - locals.var_pparam_b4soidelta_dn9);
        locals.var_t1__blk809_dn10 = (locals.var_vdsat_1_dn10 - locals.var_pparam_b4soidelta_dn10);
        locals.var_t1__blk809_dn11 = (locals.var_vdsat_1_dn11 - locals.var_pparam_b4soidelta_dn11);
        locals.var_t1__blk809_dn12 = (locals.var_vdsat_1_dn12 - locals.var_pparam_b4soidelta_dn12);
        locals.var_t1__blk809_rv = 0.0;

        let assign19630_e17696: f64 = (locals.var_t1__blk809 * locals.var_t1__blk809);
        let assign19630_e17699: f64 = (4.0 * locals.var_pparam_b4soidelta);
        let assign19630_e17701: f64 = (assign19630_e17699 * locals.var_vdsat_1);
        let assign19630_e17702: f64 = (assign19630_e17696 + assign19630_e17701);
        let assign19630_e17703: f64 = (assign19630_e17702).sqrt();
        locals.var_t2__blk810 = assign19630_e17703;
        locals.var_t2__blk810_dn3 = ((((locals.var_t1__blk809_dn3 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn3)) + (((4.0 * locals.var_pparam_b4soidelta_dn3) * locals.var_vdsat_1) + (assign19630_e17699 * locals.var_vdsat_1_dn3))) / (2.0 * assign19630_e17703));
        locals.var_t2__blk810_dn4 = ((((locals.var_t1__blk809_dn4 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn4)) + (((4.0 * locals.var_pparam_b4soidelta_dn4) * locals.var_vdsat_1) + (assign19630_e17699 * locals.var_vdsat_1_dn4))) / (2.0 * assign19630_e17703));
        locals.var_t2__blk810_dn5 = ((((locals.var_t1__blk809_dn5 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn5)) + (((4.0 * locals.var_pparam_b4soidelta_dn5) * locals.var_vdsat_1) + (assign19630_e17699 * locals.var_vdsat_1_dn5))) / (2.0 * assign19630_e17703));
        locals.var_t2__blk810_dn6 = ((((locals.var_t1__blk809_dn6 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn6)) + (((4.0 * locals.var_pparam_b4soidelta_dn6) * locals.var_vdsat_1) + (assign19630_e17699 * locals.var_vdsat_1_dn6))) / (2.0 * assign19630_e17703));
        locals.var_t2__blk810_dn7 = ((((locals.var_t1__blk809_dn7 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn7)) + (((4.0 * locals.var_pparam_b4soidelta_dn7) * locals.var_vdsat_1) + (assign19630_e17699 * locals.var_vdsat_1_dn7))) / (2.0 * assign19630_e17703));
        locals.var_t2__blk810_dn8 = ((((locals.var_t1__blk809_dn8 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn8)) + (((4.0 * locals.var_pparam_b4soidelta_dn8) * locals.var_vdsat_1) + (assign19630_e17699 * locals.var_vdsat_1_dn8))) / (2.0 * assign19630_e17703));
        locals.var_t2__blk810_dn9 = ((((locals.var_t1__blk809_dn9 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn9)) + (((4.0 * locals.var_pparam_b4soidelta_dn9) * locals.var_vdsat_1) + (assign19630_e17699 * locals.var_vdsat_1_dn9))) / (2.0 * assign19630_e17703));
        locals.var_t2__blk810_dn10 = ((((locals.var_t1__blk809_dn10 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn10)) + (((4.0 * locals.var_pparam_b4soidelta_dn10) * locals.var_vdsat_1) + (assign19630_e17699 * locals.var_vdsat_1_dn10))) / (2.0 * assign19630_e17703));
        locals.var_t2__blk810_dn11 = ((((locals.var_t1__blk809_dn11 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn11)) + (((4.0 * locals.var_pparam_b4soidelta_dn11) * locals.var_vdsat_1) + (assign19630_e17699 * locals.var_vdsat_1_dn11))) / (2.0 * assign19630_e17703));
        locals.var_t2__blk810_dn12 = ((((locals.var_t1__blk809_dn12 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn12)) + (((4.0 * locals.var_pparam_b4soidelta_dn12) * locals.var_vdsat_1) + (assign19630_e17699 * locals.var_vdsat_1_dn12))) / (2.0 * assign19630_e17703));
        locals.var_t2__blk810_rv = 0.0;

        let assign19640_e17708: f64 = (locals.var_t1__blk809 + locals.var_t2__blk810);
        let assign19640_e17709: f64 = (0.5 * assign19640_e17708);
        let assign19640_e17710: f64 = (locals.var_vdsat_1 - assign19640_e17709);
        locals.var_vdseff = assign19640_e17710;
        locals.var_vdseff_dn3 = (locals.var_vdsat_1_dn3 - (0.5 * (locals.var_t1__blk809_dn3 + locals.var_t2__blk810_dn3)));
        locals.var_vdseff_dn4 = (locals.var_vdsat_1_dn4 - (0.5 * (locals.var_t1__blk809_dn4 + locals.var_t2__blk810_dn4)));
        locals.var_vdseff_dn5 = (locals.var_vdsat_1_dn5 - (0.5 * (locals.var_t1__blk809_dn5 + locals.var_t2__blk810_dn5)));
        locals.var_vdseff_dn6 = (locals.var_vdsat_1_dn6 - (0.5 * (locals.var_t1__blk809_dn6 + locals.var_t2__blk810_dn6)));
        locals.var_vdseff_dn7 = (locals.var_vdsat_1_dn7 - (0.5 * (locals.var_t1__blk809_dn7 + locals.var_t2__blk810_dn7)));
        locals.var_vdseff_dn8 = (locals.var_vdsat_1_dn8 - (0.5 * (locals.var_t1__blk809_dn8 + locals.var_t2__blk810_dn8)));
        locals.var_vdseff_dn9 = (locals.var_vdsat_1_dn9 - (0.5 * (locals.var_t1__blk809_dn9 + locals.var_t2__blk810_dn9)));
        locals.var_vdseff_dn10 = (locals.var_vdsat_1_dn10 - (0.5 * (locals.var_t1__blk809_dn10 + locals.var_t2__blk810_dn10)));
        locals.var_vdseff_dn11 = (locals.var_vdsat_1_dn11 - (0.5 * (locals.var_t1__blk809_dn11 + locals.var_t2__blk810_dn11)));
        locals.var_vdseff_dn12 = (locals.var_vdsat_1_dn12 - (0.5 * (locals.var_t1__blk809_dn12 + locals.var_t2__blk810_dn12)));
        locals.var_vdseff_rv = 0.0;

        let assign19650_e17713: f64 = if locals.var_vdseff > locals.var_vds_1 { 1.0 } else { 0.0 };
        locals.var_guard1232 = assign19650_e17713;
        locals.var_guard1232_rv = 0.0;

        let (assign19660_e17717, assign19660_e17717_d_n3, assign19660_e17717_d_n4, assign19660_e17717_d_n5, assign19660_e17717_d_n6, assign19660_e17717_d_n7, assign19660_e17717_d_n8, assign19660_e17717_d_n9, assign19660_e17717_d_n10, assign19660_e17717_d_n11, assign19660_e17717_d_n12,) = {
    if (locals.var_guard1232 != 0.0) {
        (locals.var_vds_1, 0.0, 0.0, 0.0, 0.0, locals.var_vds_1_dn7, locals.var_vds_1_dn8, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn3, locals.var_vdseff_dn4, locals.var_vdseff_dn5, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn8, locals.var_vdseff_dn9, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn12,)
    }
};
        locals.var_vdseff = assign19660_e17717;
        locals.var_vdseff_dn3 = assign19660_e17717_d_n3;
        locals.var_vdseff_dn4 = assign19660_e17717_d_n4;
        locals.var_vdseff_dn5 = assign19660_e17717_d_n5;
        locals.var_vdseff_dn6 = assign19660_e17717_d_n6;
        locals.var_vdseff_dn7 = assign19660_e17717_d_n7;
        locals.var_vdseff_dn8 = assign19660_e17717_d_n8;
        locals.var_vdseff_dn9 = assign19660_e17717_d_n9;
        locals.var_vdseff_dn10 = assign19660_e17717_d_n10;
        locals.var_vdseff_dn11 = assign19660_e17717_d_n11;
        locals.var_vdseff_dn12 = assign19660_e17717_d_n12;
        locals.var_vdseff_rv = 0.0;

        let assign19670_e17720: f64 = (locals.var_vds_1 - locals.var_vdseff);
        locals.var_diffvds = assign19670_e17720;
        locals.var_diffvds_dn3 = (-locals.var_vdseff_dn3);
        locals.var_diffvds_dn4 = (-locals.var_vdseff_dn4);
        locals.var_diffvds_dn5 = (-locals.var_vdseff_dn5);
        locals.var_diffvds_dn6 = (-locals.var_vdseff_dn6);
        locals.var_diffvds_dn7 = (locals.var_vds_1_dn7 - locals.var_vdseff_dn7);
        locals.var_diffvds_dn8 = (locals.var_vds_1_dn8 - locals.var_vdseff_dn8);
        locals.var_diffvds_dn9 = (-locals.var_vdseff_dn9);
        locals.var_diffvds_dn10 = (-locals.var_vdseff_dn10);
        locals.var_diffvds_dn11 = (-locals.var_vdseff_dn11);
        locals.var_diffvds_dn12 = (-locals.var_vdseff_dn12);
        locals.var_diffvds_rv = 0.0;

        locals.var_b4soivdseff = locals.var_vdseff;
        locals.var_b4soivdseff_dn3 = locals.var_vdseff_dn3;
        locals.var_b4soivdseff_dn4 = locals.var_vdseff_dn4;
        locals.var_b4soivdseff_dn5 = locals.var_vdseff_dn5;
        locals.var_b4soivdseff_dn6 = locals.var_vdseff_dn6;
        locals.var_b4soivdseff_dn7 = locals.var_vdseff_dn7;
        locals.var_b4soivdseff_dn8 = locals.var_vdseff_dn8;
        locals.var_b4soivdseff_dn9 = locals.var_vdseff_dn9;
        locals.var_b4soivdseff_dn10 = locals.var_vdseff_dn10;
        locals.var_b4soivdseff_dn11 = locals.var_vdseff_dn11;
        locals.var_b4soivdseff_dn12 = locals.var_vdseff_dn12;
        locals.var_b4soivdseff_rv = 0.0;

        let assign19690_e17725: f64 = (0.5 * locals.var_abulk);
        let assign19690_e17727: f64 = (assign19690_e17725 * locals.var_vdsat_1);
        let assign19690_e17729: f64 = (assign19690_e17727 / locals.var_vgst2vtm);
        let assign19690_e17730: f64 = (1.0 - assign19690_e17729);
        locals.var_tmp4 = assign19690_e17730;
        locals.var_tmp4_dn3 = (-((((((0.5 * locals.var_abulk_dn3) * locals.var_vdsat_1) + (assign19690_e17725 * locals.var_vdsat_1_dn3)) * locals.var_vgst2vtm) - (assign19690_e17727 * locals.var_vgst2vtm_dn3)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)));
        locals.var_tmp4_dn4 = (-((((((0.5 * locals.var_abulk_dn4) * locals.var_vdsat_1) + (assign19690_e17725 * locals.var_vdsat_1_dn4)) * locals.var_vgst2vtm) - (assign19690_e17727 * locals.var_vgst2vtm_dn4)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)));
        locals.var_tmp4_dn5 = (-((((((0.5 * locals.var_abulk_dn5) * locals.var_vdsat_1) + (assign19690_e17725 * locals.var_vdsat_1_dn5)) * locals.var_vgst2vtm) - (assign19690_e17727 * locals.var_vgst2vtm_dn5)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)));
        locals.var_tmp4_dn6 = (-((((((0.5 * locals.var_abulk_dn6) * locals.var_vdsat_1) + (assign19690_e17725 * locals.var_vdsat_1_dn6)) * locals.var_vgst2vtm) - (assign19690_e17727 * locals.var_vgst2vtm_dn6)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)));
        locals.var_tmp4_dn7 = (-((((((0.5 * locals.var_abulk_dn7) * locals.var_vdsat_1) + (assign19690_e17725 * locals.var_vdsat_1_dn7)) * locals.var_vgst2vtm) - (assign19690_e17727 * locals.var_vgst2vtm_dn7)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)));
        locals.var_tmp4_dn8 = (-((((((0.5 * locals.var_abulk_dn8) * locals.var_vdsat_1) + (assign19690_e17725 * locals.var_vdsat_1_dn8)) * locals.var_vgst2vtm) - (assign19690_e17727 * locals.var_vgst2vtm_dn8)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)));
        locals.var_tmp4_dn9 = (-((((((0.5 * locals.var_abulk_dn9) * locals.var_vdsat_1) + (assign19690_e17725 * locals.var_vdsat_1_dn9)) * locals.var_vgst2vtm) - (assign19690_e17727 * locals.var_vgst2vtm_dn9)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)));
        locals.var_tmp4_dn10 = (-((((((0.5 * locals.var_abulk_dn10) * locals.var_vdsat_1) + (assign19690_e17725 * locals.var_vdsat_1_dn10)) * locals.var_vgst2vtm) - (assign19690_e17727 * locals.var_vgst2vtm_dn10)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)));
        locals.var_tmp4_dn11 = (-((((((0.5 * locals.var_abulk_dn11) * locals.var_vdsat_1) + (assign19690_e17725 * locals.var_vdsat_1_dn11)) * locals.var_vgst2vtm) - (assign19690_e17727 * locals.var_vgst2vtm_dn11)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)));
        locals.var_tmp4_dn12 = (-((((((0.5 * locals.var_abulk_dn12) * locals.var_vdsat_1) + (assign19690_e17725 * locals.var_vdsat_1_dn12)) * locals.var_vgst2vtm) - (assign19690_e17727 * locals.var_vgst2vtm_dn12)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)));
        locals.var_tmp4_rv = 0.0;

        let assign19700_e17733: f64 = (locals.var_wvcoxrds * locals.var_vgsteff__blk840);
        locals.var_t9 = assign19700_e17733;
        locals.var_t9_dn3 = ((locals.var_wvcoxrds_dn3 * locals.var_vgsteff__blk840) + (locals.var_wvcoxrds * locals.var_vgsteff__blk840_dn3));
        locals.var_t9_dn4 = ((locals.var_wvcoxrds_dn4 * locals.var_vgsteff__blk840) + (locals.var_wvcoxrds * locals.var_vgsteff__blk840_dn4));
        locals.var_t9_dn5 = ((locals.var_wvcoxrds_dn5 * locals.var_vgsteff__blk840) + (locals.var_wvcoxrds * locals.var_vgsteff__blk840_dn5));
        locals.var_t9_dn6 = ((locals.var_wvcoxrds_dn6 * locals.var_vgsteff__blk840) + (locals.var_wvcoxrds * locals.var_vgsteff__blk840_dn6));
        locals.var_t9_dn7 = ((locals.var_wvcoxrds_dn7 * locals.var_vgsteff__blk840) + (locals.var_wvcoxrds * locals.var_vgsteff__blk840_dn7));
        locals.var_t9_dn8 = ((locals.var_wvcoxrds_dn8 * locals.var_vgsteff__blk840) + (locals.var_wvcoxrds * locals.var_vgsteff__blk840_dn8));
        locals.var_t9_dn9 = ((locals.var_wvcoxrds_dn9 * locals.var_vgsteff__blk840) + (locals.var_wvcoxrds * locals.var_vgsteff__blk840_dn9));
        locals.var_t9_dn10 = ((locals.var_wvcoxrds_dn10 * locals.var_vgsteff__blk840) + (locals.var_wvcoxrds * locals.var_vgsteff__blk840_dn10));
        locals.var_t9_dn11 = ((locals.var_wvcoxrds_dn11 * locals.var_vgsteff__blk840) + (locals.var_wvcoxrds * locals.var_vgsteff__blk840_dn11));
        locals.var_t9_dn12 = ((locals.var_wvcoxrds_dn12 * locals.var_vgsteff__blk840) + (locals.var_wvcoxrds * locals.var_vgsteff__blk840_dn12));
        locals.var_t9_rv = 0.0;

        let assign19710_e17736: f64 = (locals.var_esatl + locals.var_vdsat_1);
        let assign19710_e17739: f64 = (2.0 * locals.var_t9);
        let assign19710_e17741: f64 = (assign19710_e17739 * locals.var_tmp4);
        let assign19710_e17742: f64 = (assign19710_e17736 + assign19710_e17741);
        locals.var_t0__blk808 = assign19710_e17742;
        locals.var_t0__blk808_dn3 = ((locals.var_esatl_dn3 + locals.var_vdsat_1_dn3) + (((2.0 * locals.var_t9_dn3) * locals.var_tmp4) + (assign19710_e17739 * locals.var_tmp4_dn3)));
        locals.var_t0__blk808_dn4 = ((locals.var_esatl_dn4 + locals.var_vdsat_1_dn4) + (((2.0 * locals.var_t9_dn4) * locals.var_tmp4) + (assign19710_e17739 * locals.var_tmp4_dn4)));
        locals.var_t0__blk808_dn5 = ((locals.var_esatl_dn5 + locals.var_vdsat_1_dn5) + (((2.0 * locals.var_t9_dn5) * locals.var_tmp4) + (assign19710_e17739 * locals.var_tmp4_dn5)));
        locals.var_t0__blk808_dn6 = ((locals.var_esatl_dn6 + locals.var_vdsat_1_dn6) + (((2.0 * locals.var_t9_dn6) * locals.var_tmp4) + (assign19710_e17739 * locals.var_tmp4_dn6)));
        locals.var_t0__blk808_dn7 = ((locals.var_esatl_dn7 + locals.var_vdsat_1_dn7) + (((2.0 * locals.var_t9_dn7) * locals.var_tmp4) + (assign19710_e17739 * locals.var_tmp4_dn7)));
        locals.var_t0__blk808_dn8 = ((locals.var_esatl_dn8 + locals.var_vdsat_1_dn8) + (((2.0 * locals.var_t9_dn8) * locals.var_tmp4) + (assign19710_e17739 * locals.var_tmp4_dn8)));
        locals.var_t0__blk808_dn9 = ((locals.var_esatl_dn9 + locals.var_vdsat_1_dn9) + (((2.0 * locals.var_t9_dn9) * locals.var_tmp4) + (assign19710_e17739 * locals.var_tmp4_dn9)));
        locals.var_t0__blk808_dn10 = ((locals.var_esatl_dn10 + locals.var_vdsat_1_dn10) + (((2.0 * locals.var_t9_dn10) * locals.var_tmp4) + (assign19710_e17739 * locals.var_tmp4_dn10)));
        locals.var_t0__blk808_dn11 = ((locals.var_esatl_dn11 + locals.var_vdsat_1_dn11) + (((2.0 * locals.var_t9_dn11) * locals.var_tmp4) + (assign19710_e17739 * locals.var_tmp4_dn11)));
        locals.var_t0__blk808_dn12 = ((locals.var_esatl_dn12 + locals.var_vdsat_1_dn12) + (((2.0 * locals.var_t9_dn12) * locals.var_tmp4) + (assign19710_e17739 * locals.var_tmp4_dn12)));
        locals.var_t0__blk808_rv = 0.0;

        let assign19720_e17745: f64 = (locals.var_wvcoxrds * locals.var_abulk);
        locals.var_t9 = assign19720_e17745;
        locals.var_t9_dn3 = ((locals.var_wvcoxrds_dn3 * locals.var_abulk) + (locals.var_wvcoxrds * locals.var_abulk_dn3));
        locals.var_t9_dn4 = ((locals.var_wvcoxrds_dn4 * locals.var_abulk) + (locals.var_wvcoxrds * locals.var_abulk_dn4));
        locals.var_t9_dn5 = ((locals.var_wvcoxrds_dn5 * locals.var_abulk) + (locals.var_wvcoxrds * locals.var_abulk_dn5));
        locals.var_t9_dn6 = ((locals.var_wvcoxrds_dn6 * locals.var_abulk) + (locals.var_wvcoxrds * locals.var_abulk_dn6));
        locals.var_t9_dn7 = ((locals.var_wvcoxrds_dn7 * locals.var_abulk) + (locals.var_wvcoxrds * locals.var_abulk_dn7));
        locals.var_t9_dn8 = ((locals.var_wvcoxrds_dn8 * locals.var_abulk) + (locals.var_wvcoxrds * locals.var_abulk_dn8));
        locals.var_t9_dn9 = ((locals.var_wvcoxrds_dn9 * locals.var_abulk) + (locals.var_wvcoxrds * locals.var_abulk_dn9));
        locals.var_t9_dn10 = ((locals.var_wvcoxrds_dn10 * locals.var_abulk) + (locals.var_wvcoxrds * locals.var_abulk_dn10));
        locals.var_t9_dn11 = ((locals.var_wvcoxrds_dn11 * locals.var_abulk) + (locals.var_wvcoxrds * locals.var_abulk_dn11));
        locals.var_t9_dn12 = ((locals.var_wvcoxrds_dn12 * locals.var_abulk) + (locals.var_wvcoxrds * locals.var_abulk_dn12));
        locals.var_t9_rv = 0.0;

        let assign19730_e17748: f64 = (2.0 / locals.var_lambda);
        let assign19730_e17750: f64 = (assign19730_e17748 - 1.0);
        let assign19730_e17752: f64 = (assign19730_e17750 + locals.var_t9);
        locals.var_t1__blk809 = assign19730_e17752;
        locals.var_t1__blk809_dn3 = ((-((2.0 * locals.var_lambda_dn3) / (locals.var_lambda * locals.var_lambda))) + locals.var_t9_dn3);
        locals.var_t1__blk809_dn4 = ((-((2.0 * locals.var_lambda_dn4) / (locals.var_lambda * locals.var_lambda))) + locals.var_t9_dn4);
        locals.var_t1__blk809_dn5 = ((-((2.0 * locals.var_lambda_dn5) / (locals.var_lambda * locals.var_lambda))) + locals.var_t9_dn5);
        locals.var_t1__blk809_dn6 = ((-((2.0 * locals.var_lambda_dn6) / (locals.var_lambda * locals.var_lambda))) + locals.var_t9_dn6);
        locals.var_t1__blk809_dn7 = ((-((2.0 * locals.var_lambda_dn7) / (locals.var_lambda * locals.var_lambda))) + locals.var_t9_dn7);
        locals.var_t1__blk809_dn8 = ((-((2.0 * locals.var_lambda_dn8) / (locals.var_lambda * locals.var_lambda))) + locals.var_t9_dn8);
        locals.var_t1__blk809_dn9 = ((-((2.0 * locals.var_lambda_dn9) / (locals.var_lambda * locals.var_lambda))) + locals.var_t9_dn9);
        locals.var_t1__blk809_dn10 = ((-((2.0 * locals.var_lambda_dn10) / (locals.var_lambda * locals.var_lambda))) + locals.var_t9_dn10);
        locals.var_t1__blk809_dn11 = ((-((2.0 * locals.var_lambda_dn11) / (locals.var_lambda * locals.var_lambda))) + locals.var_t9_dn11);
        locals.var_t1__blk809_dn12 = ((-((2.0 * locals.var_lambda_dn12) / (locals.var_lambda * locals.var_lambda))) + locals.var_t9_dn12);
        locals.var_t1__blk809_rv = 0.0;

        let assign19740_e17755: f64 = (locals.var_t0__blk808 / locals.var_t1__blk809);
        locals.var_vasat = assign19740_e17755;
        locals.var_vasat_dn3 = (((locals.var_t0__blk808_dn3 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn3)) / (locals.var_t1__blk809 * locals.var_t1__blk809));
        locals.var_vasat_dn4 = (((locals.var_t0__blk808_dn4 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn4)) / (locals.var_t1__blk809 * locals.var_t1__blk809));
        locals.var_vasat_dn5 = (((locals.var_t0__blk808_dn5 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn5)) / (locals.var_t1__blk809 * locals.var_t1__blk809));
        locals.var_vasat_dn6 = (((locals.var_t0__blk808_dn6 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn6)) / (locals.var_t1__blk809 * locals.var_t1__blk809));
        locals.var_vasat_dn7 = (((locals.var_t0__blk808_dn7 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn7)) / (locals.var_t1__blk809 * locals.var_t1__blk809));
        locals.var_vasat_dn8 = (((locals.var_t0__blk808_dn8 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn8)) / (locals.var_t1__blk809 * locals.var_t1__blk809));
        locals.var_vasat_dn9 = (((locals.var_t0__blk808_dn9 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn9)) / (locals.var_t1__blk809 * locals.var_t1__blk809));
        locals.var_vasat_dn10 = (((locals.var_t0__blk808_dn10 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn10)) / (locals.var_t1__blk809 * locals.var_t1__blk809));
        locals.var_vasat_dn11 = (((locals.var_t0__blk808_dn11 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn11)) / (locals.var_t1__blk809 * locals.var_t1__blk809));
        locals.var_vasat_dn12 = (((locals.var_t0__blk808_dn12 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn12)) / (locals.var_t1__blk809 * locals.var_t1__blk809));
        locals.var_vasat_rv = 0.0;

        let assign19750_e17762: f64 = if ((locals.var_pparam_b4soipclm > 0.0) && (locals.var_diffvds > 1e-10)) { 1.0 } else { 0.0 };
        locals.var_guard1233 = assign19750_e17762;
        locals.var_guard1233_rv = 0.0;

        let (assign19760_e17772, assign19760_e17772_d_n3, assign19760_e17772_d_n4, assign19760_e17772_d_n5, assign19760_e17772_d_n6, assign19760_e17772_d_n7, assign19760_e17772_d_n8, assign19760_e17772_d_n9, assign19760_e17772_d_n10, assign19760_e17772_d_n11, assign19760_e17772_d_n12,) = {
    if (locals.var_guard1233 != 0.0) {
        let assign19760_e17767: f64 = (locals.var_pparam_b4soipclm * locals.var_abulk);
        let assign19760_e17769: f64 = (assign19760_e17767 * locals.var_pparam_b4soilitl);
        let assign19760_e17770: f64 = (1.0 / assign19760_e17769);
        (assign19760_e17770, (-(((((locals.var_pparam_b4soipclm_dn3 * locals.var_abulk) + (locals.var_pparam_b4soipclm * locals.var_abulk_dn3)) * locals.var_pparam_b4soilitl) + (assign19760_e17767 * locals.var_pparam_b4soilitl_dn3)) / (assign19760_e17769 * assign19760_e17769))), (-(((((locals.var_pparam_b4soipclm_dn4 * locals.var_abulk) + (locals.var_pparam_b4soipclm * locals.var_abulk_dn4)) * locals.var_pparam_b4soilitl) + (assign19760_e17767 * locals.var_pparam_b4soilitl_dn4)) / (assign19760_e17769 * assign19760_e17769))), (-(((((locals.var_pparam_b4soipclm_dn5 * locals.var_abulk) + (locals.var_pparam_b4soipclm * locals.var_abulk_dn5)) * locals.var_pparam_b4soilitl) + (assign19760_e17767 * locals.var_pparam_b4soilitl_dn5)) / (assign19760_e17769 * assign19760_e17769))), (-(((((locals.var_pparam_b4soipclm_dn6 * locals.var_abulk) + (locals.var_pparam_b4soipclm * locals.var_abulk_dn6)) * locals.var_pparam_b4soilitl) + (assign19760_e17767 * locals.var_pparam_b4soilitl_dn6)) / (assign19760_e17769 * assign19760_e17769))), (-(((((locals.var_pparam_b4soipclm_dn7 * locals.var_abulk) + (locals.var_pparam_b4soipclm * locals.var_abulk_dn7)) * locals.var_pparam_b4soilitl) + (assign19760_e17767 * locals.var_pparam_b4soilitl_dn7)) / (assign19760_e17769 * assign19760_e17769))), (-(((((locals.var_pparam_b4soipclm_dn8 * locals.var_abulk) + (locals.var_pparam_b4soipclm * locals.var_abulk_dn8)) * locals.var_pparam_b4soilitl) + (assign19760_e17767 * locals.var_pparam_b4soilitl_dn8)) / (assign19760_e17769 * assign19760_e17769))), (-(((((locals.var_pparam_b4soipclm_dn9 * locals.var_abulk) + (locals.var_pparam_b4soipclm * locals.var_abulk_dn9)) * locals.var_pparam_b4soilitl) + (assign19760_e17767 * locals.var_pparam_b4soilitl_dn9)) / (assign19760_e17769 * assign19760_e17769))), (-(((((locals.var_pparam_b4soipclm_dn10 * locals.var_abulk) + (locals.var_pparam_b4soipclm * locals.var_abulk_dn10)) * locals.var_pparam_b4soilitl) + (assign19760_e17767 * locals.var_pparam_b4soilitl_dn10)) / (assign19760_e17769 * assign19760_e17769))), (-(((((locals.var_pparam_b4soipclm_dn11 * locals.var_abulk) + (locals.var_pparam_b4soipclm * locals.var_abulk_dn11)) * locals.var_pparam_b4soilitl) + (assign19760_e17767 * locals.var_pparam_b4soilitl_dn11)) / (assign19760_e17769 * assign19760_e17769))), (-(((((locals.var_pparam_b4soipclm_dn12 * locals.var_abulk) + (locals.var_pparam_b4soipclm * locals.var_abulk_dn12)) * locals.var_pparam_b4soilitl) + (assign19760_e17767 * locals.var_pparam_b4soilitl_dn12)) / (assign19760_e17769 * assign19760_e17769))),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign19760_e17772;
        locals.var_t0__blk808_dn3 = assign19760_e17772_d_n3;
        locals.var_t0__blk808_dn4 = assign19760_e17772_d_n4;
        locals.var_t0__blk808_dn5 = assign19760_e17772_d_n5;
        locals.var_t0__blk808_dn6 = assign19760_e17772_d_n6;
        locals.var_t0__blk808_dn7 = assign19760_e17772_d_n7;
        locals.var_t0__blk808_dn8 = assign19760_e17772_d_n8;
        locals.var_t0__blk808_dn9 = assign19760_e17772_d_n9;
        locals.var_t0__blk808_dn10 = assign19760_e17772_d_n10;
        locals.var_t0__blk808_dn11 = assign19760_e17772_d_n11;
        locals.var_t0__blk808_dn12 = assign19760_e17772_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign19770_e17778, assign19770_e17778_d_n3, assign19770_e17778_d_n4, assign19770_e17778_d_n5, assign19770_e17778_d_n6, assign19770_e17778_d_n7, assign19770_e17778_d_n8, assign19770_e17778_d_n9, assign19770_e17778_d_n10, assign19770_e17778_d_n11, assign19770_e17778_d_n12,) = {
    if (locals.var_guard1233 != 0.0) {
        let assign19770_e17776: f64 = (locals.var_vgsteff__blk840 / locals.var_esatl);
        (assign19770_e17776, (((locals.var_vgsteff__blk840_dn3 * locals.var_esatl) - (locals.var_vgsteff__blk840 * locals.var_esatl_dn3)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_vgsteff__blk840_dn4 * locals.var_esatl) - (locals.var_vgsteff__blk840 * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_vgsteff__blk840_dn5 * locals.var_esatl) - (locals.var_vgsteff__blk840 * locals.var_esatl_dn5)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_vgsteff__blk840_dn6 * locals.var_esatl) - (locals.var_vgsteff__blk840 * locals.var_esatl_dn6)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_vgsteff__blk840_dn7 * locals.var_esatl) - (locals.var_vgsteff__blk840 * locals.var_esatl_dn7)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_vgsteff__blk840_dn8 * locals.var_esatl) - (locals.var_vgsteff__blk840 * locals.var_esatl_dn8)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_vgsteff__blk840_dn9 * locals.var_esatl) - (locals.var_vgsteff__blk840 * locals.var_esatl_dn9)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_vgsteff__blk840_dn10 * locals.var_esatl) - (locals.var_vgsteff__blk840 * locals.var_esatl_dn10)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_vgsteff__blk840_dn11 * locals.var_esatl) - (locals.var_vgsteff__blk840 * locals.var_esatl_dn11)) / (locals.var_esatl * locals.var_esatl)), (((locals.var_vgsteff__blk840_dn12 * locals.var_esatl) - (locals.var_vgsteff__blk840 * locals.var_esatl_dn12)) / (locals.var_esatl * locals.var_esatl)),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign19770_e17778;
        locals.var_t2__blk810_dn3 = assign19770_e17778_d_n3;
        locals.var_t2__blk810_dn4 = assign19770_e17778_d_n4;
        locals.var_t2__blk810_dn5 = assign19770_e17778_d_n5;
        locals.var_t2__blk810_dn6 = assign19770_e17778_d_n6;
        locals.var_t2__blk810_dn7 = assign19770_e17778_d_n7;
        locals.var_t2__blk810_dn8 = assign19770_e17778_d_n8;
        locals.var_t2__blk810_dn9 = assign19770_e17778_d_n9;
        locals.var_t2__blk810_dn10 = assign19770_e17778_d_n10;
        locals.var_t2__blk810_dn11 = assign19770_e17778_d_n11;
        locals.var_t2__blk810_dn12 = assign19770_e17778_d_n12;
        locals.var_t2__blk810_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_60(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19780_e17786, assign19780_e17786_d_n3, assign19780_e17786_d_n4, assign19780_e17786_d_n5, assign19780_e17786_d_n6, assign19780_e17786_d_n7, assign19780_e17786_d_n8, assign19780_e17786_d_n9, assign19780_e17786_d_n10, assign19780_e17786_d_n11, assign19780_e17786_d_n12,) = {
    if (locals.var_guard1233 != 0.0) {
        let assign19780_e17783: f64 = (locals.var_abulk + locals.var_t2__blk810);
        let assign19780_e17784: f64 = (locals.var_leff * assign19780_e17783);
        (assign19780_e17784, ((locals.var_leff_dn3 * assign19780_e17783) + (locals.var_leff * (locals.var_abulk_dn3 + locals.var_t2__blk810_dn3))), ((locals.var_leff_dn4 * assign19780_e17783) + (locals.var_leff * (locals.var_abulk_dn4 + locals.var_t2__blk810_dn4))), ((locals.var_leff_dn5 * assign19780_e17783) + (locals.var_leff * (locals.var_abulk_dn5 + locals.var_t2__blk810_dn5))), ((locals.var_leff_dn6 * assign19780_e17783) + (locals.var_leff * (locals.var_abulk_dn6 + locals.var_t2__blk810_dn6))), ((locals.var_leff_dn7 * assign19780_e17783) + (locals.var_leff * (locals.var_abulk_dn7 + locals.var_t2__blk810_dn7))), ((locals.var_leff_dn8 * assign19780_e17783) + (locals.var_leff * (locals.var_abulk_dn8 + locals.var_t2__blk810_dn8))), ((locals.var_leff_dn9 * assign19780_e17783) + (locals.var_leff * (locals.var_abulk_dn9 + locals.var_t2__blk810_dn9))), ((locals.var_leff_dn10 * assign19780_e17783) + (locals.var_leff * (locals.var_abulk_dn10 + locals.var_t2__blk810_dn10))), ((locals.var_leff_dn11 * assign19780_e17783) + (locals.var_leff * (locals.var_abulk_dn11 + locals.var_t2__blk810_dn11))), ((locals.var_leff_dn12 * assign19780_e17783) + (locals.var_leff * (locals.var_abulk_dn12 + locals.var_t2__blk810_dn12))),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign19780_e17786;
        locals.var_t1__blk809_dn3 = assign19780_e17786_d_n3;
        locals.var_t1__blk809_dn4 = assign19780_e17786_d_n4;
        locals.var_t1__blk809_dn5 = assign19780_e17786_d_n5;
        locals.var_t1__blk809_dn6 = assign19780_e17786_d_n6;
        locals.var_t1__blk809_dn7 = assign19780_e17786_d_n7;
        locals.var_t1__blk809_dn8 = assign19780_e17786_d_n8;
        locals.var_t1__blk809_dn9 = assign19780_e17786_d_n9;
        locals.var_t1__blk809_dn10 = assign19780_e17786_d_n10;
        locals.var_t1__blk809_dn11 = assign19780_e17786_d_n11;
        locals.var_t1__blk809_dn12 = assign19780_e17786_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign19790_e17792, assign19790_e17792_d_n3, assign19790_e17792_d_n4, assign19790_e17792_d_n5, assign19790_e17792_d_n6, assign19790_e17792_d_n7, assign19790_e17792_d_n8, assign19790_e17792_d_n9, assign19790_e17792_d_n10, assign19790_e17792_d_n11, assign19790_e17792_d_n12,) = {
    if (locals.var_guard1233 != 0.0) {
        let assign19790_e17790: f64 = (locals.var_t0__blk808 * locals.var_t1__blk809);
        (assign19790_e17790, ((locals.var_t0__blk808_dn3 * locals.var_t1__blk809) + (locals.var_t0__blk808 * locals.var_t1__blk809_dn3)), ((locals.var_t0__blk808_dn4 * locals.var_t1__blk809) + (locals.var_t0__blk808 * locals.var_t1__blk809_dn4)), ((locals.var_t0__blk808_dn5 * locals.var_t1__blk809) + (locals.var_t0__blk808 * locals.var_t1__blk809_dn5)), ((locals.var_t0__blk808_dn6 * locals.var_t1__blk809) + (locals.var_t0__blk808 * locals.var_t1__blk809_dn6)), ((locals.var_t0__blk808_dn7 * locals.var_t1__blk809) + (locals.var_t0__blk808 * locals.var_t1__blk809_dn7)), ((locals.var_t0__blk808_dn8 * locals.var_t1__blk809) + (locals.var_t0__blk808 * locals.var_t1__blk809_dn8)), ((locals.var_t0__blk808_dn9 * locals.var_t1__blk809) + (locals.var_t0__blk808 * locals.var_t1__blk809_dn9)), ((locals.var_t0__blk808_dn10 * locals.var_t1__blk809) + (locals.var_t0__blk808 * locals.var_t1__blk809_dn10)), ((locals.var_t0__blk808_dn11 * locals.var_t1__blk809) + (locals.var_t0__blk808 * locals.var_t1__blk809_dn11)), ((locals.var_t0__blk808_dn12 * locals.var_t1__blk809) + (locals.var_t0__blk808 * locals.var_t1__blk809_dn12)),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign19790_e17792;
        locals.var_t9_dn3 = assign19790_e17792_d_n3;
        locals.var_t9_dn4 = assign19790_e17792_d_n4;
        locals.var_t9_dn5 = assign19790_e17792_d_n5;
        locals.var_t9_dn6 = assign19790_e17792_d_n6;
        locals.var_t9_dn7 = assign19790_e17792_d_n7;
        locals.var_t9_dn8 = assign19790_e17792_d_n8;
        locals.var_t9_dn9 = assign19790_e17792_d_n9;
        locals.var_t9_dn10 = assign19790_e17792_d_n10;
        locals.var_t9_dn11 = assign19790_e17792_d_n11;
        locals.var_t9_dn12 = assign19790_e17792_d_n12;
        locals.var_t9_rv = 0.0;

        let (assign19800_e17798, assign19800_e17798_d_n3, assign19800_e17798_d_n4, assign19800_e17798_d_n5, assign19800_e17798_d_n6, assign19800_e17798_d_n7, assign19800_e17798_d_n8, assign19800_e17798_d_n9, assign19800_e17798_d_n10, assign19800_e17798_d_n11, assign19800_e17798_d_n12,) = {
    if (locals.var_guard1233 != 0.0) {
        let assign19800_e17796: f64 = (locals.var_t9 * locals.var_diffvds);
        (assign19800_e17796, ((locals.var_t9_dn3 * locals.var_diffvds) + (locals.var_t9 * locals.var_diffvds_dn3)), ((locals.var_t9_dn4 * locals.var_diffvds) + (locals.var_t9 * locals.var_diffvds_dn4)), ((locals.var_t9_dn5 * locals.var_diffvds) + (locals.var_t9 * locals.var_diffvds_dn5)), ((locals.var_t9_dn6 * locals.var_diffvds) + (locals.var_t9 * locals.var_diffvds_dn6)), ((locals.var_t9_dn7 * locals.var_diffvds) + (locals.var_t9 * locals.var_diffvds_dn7)), ((locals.var_t9_dn8 * locals.var_diffvds) + (locals.var_t9 * locals.var_diffvds_dn8)), ((locals.var_t9_dn9 * locals.var_diffvds) + (locals.var_t9 * locals.var_diffvds_dn9)), ((locals.var_t9_dn10 * locals.var_diffvds) + (locals.var_t9 * locals.var_diffvds_dn10)), ((locals.var_t9_dn11 * locals.var_diffvds) + (locals.var_t9 * locals.var_diffvds_dn11)), ((locals.var_t9_dn12 * locals.var_diffvds) + (locals.var_t9 * locals.var_diffvds_dn12)),)
    } else {
        (locals.var_vaclm, locals.var_vaclm_dn3, locals.var_vaclm_dn4, locals.var_vaclm_dn5, locals.var_vaclm_dn6, locals.var_vaclm_dn7, locals.var_vaclm_dn8, locals.var_vaclm_dn9, locals.var_vaclm_dn10, locals.var_vaclm_dn11, locals.var_vaclm_dn12,)
    }
};
        locals.var_vaclm = assign19800_e17798;
        locals.var_vaclm_dn3 = assign19800_e17798_d_n3;
        locals.var_vaclm_dn4 = assign19800_e17798_d_n4;
        locals.var_vaclm_dn5 = assign19800_e17798_d_n5;
        locals.var_vaclm_dn6 = assign19800_e17798_d_n6;
        locals.var_vaclm_dn7 = assign19800_e17798_d_n7;
        locals.var_vaclm_dn8 = assign19800_e17798_d_n8;
        locals.var_vaclm_dn9 = assign19800_e17798_d_n9;
        locals.var_vaclm_dn10 = assign19800_e17798_d_n10;
        locals.var_vaclm_dn11 = assign19800_e17798_d_n11;
        locals.var_vaclm_dn12 = assign19800_e17798_d_n12;
        locals.var_vaclm_rv = 0.0;

        let (assign19810_e17803, assign19810_e17803_d_n3, assign19810_e17803_d_n4, assign19810_e17803_d_n5, assign19810_e17803_d_n6, assign19810_e17803_d_n7, assign19810_e17803_d_n8, assign19810_e17803_d_n9, assign19810_e17803_d_n10, assign19810_e17803_d_n11, assign19810_e17803_d_n12,) = {
    if (locals.var_guard1233 == 0.0) {
        (2.688117142e43, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vaclm, locals.var_vaclm_dn3, locals.var_vaclm_dn4, locals.var_vaclm_dn5, locals.var_vaclm_dn6, locals.var_vaclm_dn7, locals.var_vaclm_dn8, locals.var_vaclm_dn9, locals.var_vaclm_dn10, locals.var_vaclm_dn11, locals.var_vaclm_dn12,)
    }
};
        locals.var_vaclm = assign19810_e17803;
        locals.var_vaclm_dn3 = assign19810_e17803_d_n3;
        locals.var_vaclm_dn4 = assign19810_e17803_d_n4;
        locals.var_vaclm_dn5 = assign19810_e17803_d_n5;
        locals.var_vaclm_dn6 = assign19810_e17803_d_n6;
        locals.var_vaclm_dn7 = assign19810_e17803_d_n7;
        locals.var_vaclm_dn8 = assign19810_e17803_d_n8;
        locals.var_vaclm_dn9 = assign19810_e17803_d_n9;
        locals.var_vaclm_dn10 = assign19810_e17803_d_n10;
        locals.var_vaclm_dn11 = assign19810_e17803_d_n11;
        locals.var_vaclm_dn12 = assign19810_e17803_d_n12;
        locals.var_vaclm_rv = 0.0;

        let assign19820_e17806: f64 = if locals.var_thetarout > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1234 = assign19820_e17806;
        locals.var_guard1234_rv = 0.0;

        let (assign19830_e17812, assign19830_e17812_d_n3, assign19830_e17812_d_n4, assign19830_e17812_d_n5, assign19830_e17812_d_n6, assign19830_e17812_d_n7, assign19830_e17812_d_n8, assign19830_e17812_d_n9, assign19830_e17812_d_n10, assign19830_e17812_d_n11, assign19830_e17812_d_n12,) = {
    if (locals.var_guard1234 != 0.0) {
        let assign19830_e17810: f64 = (locals.var_abulk * locals.var_vdsat_1);
        (assign19830_e17810, ((locals.var_abulk_dn3 * locals.var_vdsat_1) + (locals.var_abulk * locals.var_vdsat_1_dn3)), ((locals.var_abulk_dn4 * locals.var_vdsat_1) + (locals.var_abulk * locals.var_vdsat_1_dn4)), ((locals.var_abulk_dn5 * locals.var_vdsat_1) + (locals.var_abulk * locals.var_vdsat_1_dn5)), ((locals.var_abulk_dn6 * locals.var_vdsat_1) + (locals.var_abulk * locals.var_vdsat_1_dn6)), ((locals.var_abulk_dn7 * locals.var_vdsat_1) + (locals.var_abulk * locals.var_vdsat_1_dn7)), ((locals.var_abulk_dn8 * locals.var_vdsat_1) + (locals.var_abulk * locals.var_vdsat_1_dn8)), ((locals.var_abulk_dn9 * locals.var_vdsat_1) + (locals.var_abulk * locals.var_vdsat_1_dn9)), ((locals.var_abulk_dn10 * locals.var_vdsat_1) + (locals.var_abulk * locals.var_vdsat_1_dn10)), ((locals.var_abulk_dn11 * locals.var_vdsat_1) + (locals.var_abulk * locals.var_vdsat_1_dn11)), ((locals.var_abulk_dn12 * locals.var_vdsat_1) + (locals.var_abulk * locals.var_vdsat_1_dn12)),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12,)
    }
};
        locals.var_t8 = assign19830_e17812;
        locals.var_t8_dn3 = assign19830_e17812_d_n3;
        locals.var_t8_dn4 = assign19830_e17812_d_n4;
        locals.var_t8_dn5 = assign19830_e17812_d_n5;
        locals.var_t8_dn6 = assign19830_e17812_d_n6;
        locals.var_t8_dn7 = assign19830_e17812_d_n7;
        locals.var_t8_dn8 = assign19830_e17812_d_n8;
        locals.var_t8_dn9 = assign19830_e17812_d_n9;
        locals.var_t8_dn10 = assign19830_e17812_d_n10;
        locals.var_t8_dn11 = assign19830_e17812_d_n11;
        locals.var_t8_dn12 = assign19830_e17812_d_n12;
        locals.var_t8_rv = 0.0;

        let (assign19840_e17818, assign19840_e17818_d_n3, assign19840_e17818_d_n4, assign19840_e17818_d_n5, assign19840_e17818_d_n6, assign19840_e17818_d_n7, assign19840_e17818_d_n8, assign19840_e17818_d_n9, assign19840_e17818_d_n10, assign19840_e17818_d_n11, assign19840_e17818_d_n12,) = {
    if (locals.var_guard1234 != 0.0) {
        let assign19840_e17816: f64 = (locals.var_vgst2vtm * locals.var_t8);
        (assign19840_e17816, ((locals.var_vgst2vtm_dn3 * locals.var_t8) + (locals.var_vgst2vtm * locals.var_t8_dn3)), ((locals.var_vgst2vtm_dn4 * locals.var_t8) + (locals.var_vgst2vtm * locals.var_t8_dn4)), ((locals.var_vgst2vtm_dn5 * locals.var_t8) + (locals.var_vgst2vtm * locals.var_t8_dn5)), ((locals.var_vgst2vtm_dn6 * locals.var_t8) + (locals.var_vgst2vtm * locals.var_t8_dn6)), ((locals.var_vgst2vtm_dn7 * locals.var_t8) + (locals.var_vgst2vtm * locals.var_t8_dn7)), ((locals.var_vgst2vtm_dn8 * locals.var_t8) + (locals.var_vgst2vtm * locals.var_t8_dn8)), ((locals.var_vgst2vtm_dn9 * locals.var_t8) + (locals.var_vgst2vtm * locals.var_t8_dn9)), ((locals.var_vgst2vtm_dn10 * locals.var_t8) + (locals.var_vgst2vtm * locals.var_t8_dn10)), ((locals.var_vgst2vtm_dn11 * locals.var_t8) + (locals.var_vgst2vtm * locals.var_t8_dn11)), ((locals.var_vgst2vtm_dn12 * locals.var_t8) + (locals.var_vgst2vtm * locals.var_t8_dn12)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign19840_e17818;
        locals.var_t0__blk808_dn3 = assign19840_e17818_d_n3;
        locals.var_t0__blk808_dn4 = assign19840_e17818_d_n4;
        locals.var_t0__blk808_dn5 = assign19840_e17818_d_n5;
        locals.var_t0__blk808_dn6 = assign19840_e17818_d_n6;
        locals.var_t0__blk808_dn7 = assign19840_e17818_d_n7;
        locals.var_t0__blk808_dn8 = assign19840_e17818_d_n8;
        locals.var_t0__blk808_dn9 = assign19840_e17818_d_n9;
        locals.var_t0__blk808_dn10 = assign19840_e17818_d_n10;
        locals.var_t0__blk808_dn11 = assign19840_e17818_d_n11;
        locals.var_t0__blk808_dn12 = assign19840_e17818_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign19850_e17824, assign19850_e17824_d_n3, assign19850_e17824_d_n4, assign19850_e17824_d_n5, assign19850_e17824_d_n6, assign19850_e17824_d_n7, assign19850_e17824_d_n8, assign19850_e17824_d_n9, assign19850_e17824_d_n10, assign19850_e17824_d_n11, assign19850_e17824_d_n12,) = {
    if (locals.var_guard1234 != 0.0) {
        let assign19850_e17822: f64 = (locals.var_vgst2vtm + locals.var_t8);
        (assign19850_e17822, (locals.var_vgst2vtm_dn3 + locals.var_t8_dn3), (locals.var_vgst2vtm_dn4 + locals.var_t8_dn4), (locals.var_vgst2vtm_dn5 + locals.var_t8_dn5), (locals.var_vgst2vtm_dn6 + locals.var_t8_dn6), (locals.var_vgst2vtm_dn7 + locals.var_t8_dn7), (locals.var_vgst2vtm_dn8 + locals.var_t8_dn8), (locals.var_vgst2vtm_dn9 + locals.var_t8_dn9), (locals.var_vgst2vtm_dn10 + locals.var_t8_dn10), (locals.var_vgst2vtm_dn11 + locals.var_t8_dn11), (locals.var_vgst2vtm_dn12 + locals.var_t8_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign19850_e17824;
        locals.var_t1__blk809_dn3 = assign19850_e17824_d_n3;
        locals.var_t1__blk809_dn4 = assign19850_e17824_d_n4;
        locals.var_t1__blk809_dn5 = assign19850_e17824_d_n5;
        locals.var_t1__blk809_dn6 = assign19850_e17824_d_n6;
        locals.var_t1__blk809_dn7 = assign19850_e17824_d_n7;
        locals.var_t1__blk809_dn8 = assign19850_e17824_d_n8;
        locals.var_t1__blk809_dn9 = assign19850_e17824_d_n9;
        locals.var_t1__blk809_dn10 = assign19850_e17824_d_n10;
        locals.var_t1__blk809_dn11 = assign19850_e17824_d_n11;
        locals.var_t1__blk809_dn12 = assign19850_e17824_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign19860_e17828, assign19860_e17828_d_n3, assign19860_e17828_d_n4, assign19860_e17828_d_n5, assign19860_e17828_d_n6, assign19860_e17828_d_n7, assign19860_e17828_d_n8, assign19860_e17828_d_n9, assign19860_e17828_d_n10, assign19860_e17828_d_n11, assign19860_e17828_d_n12,) = {
    if (locals.var_guard1234 != 0.0) {
        (locals.var_thetarout, locals.var_thetarout_dn3, locals.var_thetarout_dn4, locals.var_thetarout_dn5, locals.var_thetarout_dn6, locals.var_thetarout_dn7, locals.var_thetarout_dn8, locals.var_thetarout_dn9, locals.var_thetarout_dn10, locals.var_thetarout_dn11, locals.var_thetarout_dn12,)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign19860_e17828;
        locals.var_t2__blk810_dn3 = assign19860_e17828_d_n3;
        locals.var_t2__blk810_dn4 = assign19860_e17828_d_n4;
        locals.var_t2__blk810_dn5 = assign19860_e17828_d_n5;
        locals.var_t2__blk810_dn6 = assign19860_e17828_d_n6;
        locals.var_t2__blk810_dn7 = assign19860_e17828_d_n7;
        locals.var_t2__blk810_dn8 = assign19860_e17828_d_n8;
        locals.var_t2__blk810_dn9 = assign19860_e17828_d_n9;
        locals.var_t2__blk810_dn10 = assign19860_e17828_d_n10;
        locals.var_t2__blk810_dn11 = assign19860_e17828_d_n11;
        locals.var_t2__blk810_dn12 = assign19860_e17828_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign19870_e17838, assign19870_e17838_d_n3, assign19870_e17838_d_n4, assign19870_e17838_d_n5, assign19870_e17838_d_n6, assign19870_e17838_d_n7, assign19870_e17838_d_n8, assign19870_e17838_d_n9, assign19870_e17838_d_n10, assign19870_e17838_d_n11, assign19870_e17838_d_n12,) = {
    if (locals.var_guard1234 != 0.0) {
        let assign19870_e17833: f64 = (locals.var_t0__blk808 / locals.var_t1__blk809);
        let assign19870_e17834: f64 = (locals.var_vgst2vtm - assign19870_e17833);
        let assign19870_e17836: f64 = (assign19870_e17834 / locals.var_t2__blk810);
        (assign19870_e17836, ((((locals.var_vgst2vtm_dn3 - (((locals.var_t0__blk808_dn3 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn3)) / (locals.var_t1__blk809 * locals.var_t1__blk809))) * locals.var_t2__blk810) - (assign19870_e17834 * locals.var_t2__blk810_dn3)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), ((((locals.var_vgst2vtm_dn4 - (((locals.var_t0__blk808_dn4 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn4)) / (locals.var_t1__blk809 * locals.var_t1__blk809))) * locals.var_t2__blk810) - (assign19870_e17834 * locals.var_t2__blk810_dn4)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), ((((locals.var_vgst2vtm_dn5 - (((locals.var_t0__blk808_dn5 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn5)) / (locals.var_t1__blk809 * locals.var_t1__blk809))) * locals.var_t2__blk810) - (assign19870_e17834 * locals.var_t2__blk810_dn5)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), ((((locals.var_vgst2vtm_dn6 - (((locals.var_t0__blk808_dn6 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn6)) / (locals.var_t1__blk809 * locals.var_t1__blk809))) * locals.var_t2__blk810) - (assign19870_e17834 * locals.var_t2__blk810_dn6)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), ((((locals.var_vgst2vtm_dn7 - (((locals.var_t0__blk808_dn7 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn7)) / (locals.var_t1__blk809 * locals.var_t1__blk809))) * locals.var_t2__blk810) - (assign19870_e17834 * locals.var_t2__blk810_dn7)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), ((((locals.var_vgst2vtm_dn8 - (((locals.var_t0__blk808_dn8 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn8)) / (locals.var_t1__blk809 * locals.var_t1__blk809))) * locals.var_t2__blk810) - (assign19870_e17834 * locals.var_t2__blk810_dn8)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), ((((locals.var_vgst2vtm_dn9 - (((locals.var_t0__blk808_dn9 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn9)) / (locals.var_t1__blk809 * locals.var_t1__blk809))) * locals.var_t2__blk810) - (assign19870_e17834 * locals.var_t2__blk810_dn9)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), ((((locals.var_vgst2vtm_dn10 - (((locals.var_t0__blk808_dn10 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn10)) / (locals.var_t1__blk809 * locals.var_t1__blk809))) * locals.var_t2__blk810) - (assign19870_e17834 * locals.var_t2__blk810_dn10)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), ((((locals.var_vgst2vtm_dn11 - (((locals.var_t0__blk808_dn11 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn11)) / (locals.var_t1__blk809 * locals.var_t1__blk809))) * locals.var_t2__blk810) - (assign19870_e17834 * locals.var_t2__blk810_dn11)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), ((((locals.var_vgst2vtm_dn12 - (((locals.var_t0__blk808_dn12 * locals.var_t1__blk809) - (locals.var_t0__blk808 * locals.var_t1__blk809_dn12)) / (locals.var_t1__blk809 * locals.var_t1__blk809))) * locals.var_t2__blk810) - (assign19870_e17834 * locals.var_t2__blk810_dn12)) / (locals.var_t2__blk810 * locals.var_t2__blk810)),)
    } else {
        (locals.var_vadibl, locals.var_vadibl_dn3, locals.var_vadibl_dn4, locals.var_vadibl_dn5, locals.var_vadibl_dn6, locals.var_vadibl_dn7, locals.var_vadibl_dn8, locals.var_vadibl_dn9, locals.var_vadibl_dn10, locals.var_vadibl_dn11, locals.var_vadibl_dn12,)
    }
};
        locals.var_vadibl = assign19870_e17838;
        locals.var_vadibl_dn3 = assign19870_e17838_d_n3;
        locals.var_vadibl_dn4 = assign19870_e17838_d_n4;
        locals.var_vadibl_dn5 = assign19870_e17838_d_n5;
        locals.var_vadibl_dn6 = assign19870_e17838_d_n6;
        locals.var_vadibl_dn7 = assign19870_e17838_d_n7;
        locals.var_vadibl_dn8 = assign19870_e17838_d_n8;
        locals.var_vadibl_dn9 = assign19870_e17838_d_n9;
        locals.var_vadibl_dn10 = assign19870_e17838_d_n10;
        locals.var_vadibl_dn11 = assign19870_e17838_d_n11;
        locals.var_vadibl_dn12 = assign19870_e17838_d_n12;
        locals.var_vadibl_rv = 0.0;

        let (assign19880_e17844, assign19880_e17844_d_n3, assign19880_e17844_d_n4, assign19880_e17844_d_n5, assign19880_e17844_d_n6, assign19880_e17844_d_n7, assign19880_e17844_d_n8, assign19880_e17844_d_n9, assign19880_e17844_d_n10, assign19880_e17844_d_n11, assign19880_e17844_d_n12,) = {
    if (locals.var_guard1234 != 0.0) {
        let assign19880_e17842: f64 = (locals.var_pparam_b4soipdiblb * locals.var_vbseff);
        (assign19880_e17842, ((locals.var_pparam_b4soipdiblb_dn3 * locals.var_vbseff) + (locals.var_pparam_b4soipdiblb * locals.var_vbseff_dn3)), ((locals.var_pparam_b4soipdiblb_dn4 * locals.var_vbseff) + (locals.var_pparam_b4soipdiblb * locals.var_vbseff_dn4)), ((locals.var_pparam_b4soipdiblb_dn5 * locals.var_vbseff) + (locals.var_pparam_b4soipdiblb * locals.var_vbseff_dn5)), ((locals.var_pparam_b4soipdiblb_dn6 * locals.var_vbseff) + (locals.var_pparam_b4soipdiblb * locals.var_vbseff_dn6)), ((locals.var_pparam_b4soipdiblb_dn7 * locals.var_vbseff) + (locals.var_pparam_b4soipdiblb * locals.var_vbseff_dn7)), ((locals.var_pparam_b4soipdiblb_dn8 * locals.var_vbseff) + (locals.var_pparam_b4soipdiblb * locals.var_vbseff_dn8)), ((locals.var_pparam_b4soipdiblb_dn9 * locals.var_vbseff) + (locals.var_pparam_b4soipdiblb * locals.var_vbseff_dn9)), ((locals.var_pparam_b4soipdiblb_dn10 * locals.var_vbseff) + (locals.var_pparam_b4soipdiblb * locals.var_vbseff_dn10)), ((locals.var_pparam_b4soipdiblb_dn11 * locals.var_vbseff) + (locals.var_pparam_b4soipdiblb * locals.var_vbseff_dn11)), ((locals.var_pparam_b4soipdiblb_dn12 * locals.var_vbseff) + (locals.var_pparam_b4soipdiblb * locals.var_vbseff_dn12)),)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign19880_e17844;
        locals.var_t7__blk815_dn3 = assign19880_e17844_d_n3;
        locals.var_t7__blk815_dn4 = assign19880_e17844_d_n4;
        locals.var_t7__blk815_dn5 = assign19880_e17844_d_n5;
        locals.var_t7__blk815_dn6 = assign19880_e17844_d_n6;
        locals.var_t7__blk815_dn7 = assign19880_e17844_d_n7;
        locals.var_t7__blk815_dn8 = assign19880_e17844_d_n8;
        locals.var_t7__blk815_dn9 = assign19880_e17844_d_n9;
        locals.var_t7__blk815_dn10 = assign19880_e17844_d_n10;
        locals.var_t7__blk815_dn11 = assign19880_e17844_d_n11;
        locals.var_t7__blk815_dn12 = assign19880_e17844_d_n12;
        locals.var_t7__blk815_rv = 0.0;

        let assign19890_e17847: f64 = (-0.9);
        let assign19890_e17848: f64 = if locals.var_t7__blk815 >= assign19890_e17847 { 1.0 } else { 0.0 };
        locals.var_guard1235 = assign19890_e17848;
        locals.var_guard1235_rv = 0.0;

        let (assign19900_e17858, assign19900_e17858_d_n3, assign19900_e17858_d_n4, assign19900_e17858_d_n5, assign19900_e17858_d_n6, assign19900_e17858_d_n7, assign19900_e17858_d_n8, assign19900_e17858_d_n9, assign19900_e17858_d_n10, assign19900_e17858_d_n11, assign19900_e17858_d_n12,) = {
    if ((locals.var_guard1234 != 0.0) && (locals.var_guard1235 != 0.0)) {
        let assign19900_e17855: f64 = (1.0 + locals.var_t7__blk815);
        let assign19900_e17856: f64 = (1.0 / assign19900_e17855);
        (assign19900_e17856, (-(locals.var_t7__blk815_dn3 / (assign19900_e17855 * assign19900_e17855))), (-(locals.var_t7__blk815_dn4 / (assign19900_e17855 * assign19900_e17855))), (-(locals.var_t7__blk815_dn5 / (assign19900_e17855 * assign19900_e17855))), (-(locals.var_t7__blk815_dn6 / (assign19900_e17855 * assign19900_e17855))), (-(locals.var_t7__blk815_dn7 / (assign19900_e17855 * assign19900_e17855))), (-(locals.var_t7__blk815_dn8 / (assign19900_e17855 * assign19900_e17855))), (-(locals.var_t7__blk815_dn9 / (assign19900_e17855 * assign19900_e17855))), (-(locals.var_t7__blk815_dn10 / (assign19900_e17855 * assign19900_e17855))), (-(locals.var_t7__blk815_dn11 / (assign19900_e17855 * assign19900_e17855))), (-(locals.var_t7__blk815_dn12 / (assign19900_e17855 * assign19900_e17855))),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign19900_e17858;
        locals.var_t3__blk811_dn3 = assign19900_e17858_d_n3;
        locals.var_t3__blk811_dn4 = assign19900_e17858_d_n4;
        locals.var_t3__blk811_dn5 = assign19900_e17858_d_n5;
        locals.var_t3__blk811_dn6 = assign19900_e17858_d_n6;
        locals.var_t3__blk811_dn7 = assign19900_e17858_d_n7;
        locals.var_t3__blk811_dn8 = assign19900_e17858_d_n8;
        locals.var_t3__blk811_dn9 = assign19900_e17858_d_n9;
        locals.var_t3__blk811_dn10 = assign19900_e17858_d_n10;
        locals.var_t3__blk811_dn11 = assign19900_e17858_d_n11;
        locals.var_t3__blk811_dn12 = assign19900_e17858_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign19910_e17866, assign19910_e17866_d_n3, assign19910_e17866_d_n4, assign19910_e17866_d_n5, assign19910_e17866_d_n6, assign19910_e17866_d_n7, assign19910_e17866_d_n8, assign19910_e17866_d_n9, assign19910_e17866_d_n10, assign19910_e17866_d_n11, assign19910_e17866_d_n12,) = {
    if ((locals.var_guard1234 != 0.0) && (locals.var_guard1235 != 0.0)) {
        let assign19910_e17864: f64 = (locals.var_vadibl * locals.var_t3__blk811);
        (assign19910_e17864, ((locals.var_vadibl_dn3 * locals.var_t3__blk811) + (locals.var_vadibl * locals.var_t3__blk811_dn3)), ((locals.var_vadibl_dn4 * locals.var_t3__blk811) + (locals.var_vadibl * locals.var_t3__blk811_dn4)), ((locals.var_vadibl_dn5 * locals.var_t3__blk811) + (locals.var_vadibl * locals.var_t3__blk811_dn5)), ((locals.var_vadibl_dn6 * locals.var_t3__blk811) + (locals.var_vadibl * locals.var_t3__blk811_dn6)), ((locals.var_vadibl_dn7 * locals.var_t3__blk811) + (locals.var_vadibl * locals.var_t3__blk811_dn7)), ((locals.var_vadibl_dn8 * locals.var_t3__blk811) + (locals.var_vadibl * locals.var_t3__blk811_dn8)), ((locals.var_vadibl_dn9 * locals.var_t3__blk811) + (locals.var_vadibl * locals.var_t3__blk811_dn9)), ((locals.var_vadibl_dn10 * locals.var_t3__blk811) + (locals.var_vadibl * locals.var_t3__blk811_dn10)), ((locals.var_vadibl_dn11 * locals.var_t3__blk811) + (locals.var_vadibl * locals.var_t3__blk811_dn11)), ((locals.var_vadibl_dn12 * locals.var_t3__blk811) + (locals.var_vadibl * locals.var_t3__blk811_dn12)),)
    } else {
        (locals.var_vadibl, locals.var_vadibl_dn3, locals.var_vadibl_dn4, locals.var_vadibl_dn5, locals.var_vadibl_dn6, locals.var_vadibl_dn7, locals.var_vadibl_dn8, locals.var_vadibl_dn9, locals.var_vadibl_dn10, locals.var_vadibl_dn11, locals.var_vadibl_dn12,)
    }
};
        locals.var_vadibl = assign19910_e17866;
        locals.var_vadibl_dn3 = assign19910_e17866_d_n3;
        locals.var_vadibl_dn4 = assign19910_e17866_d_n4;
        locals.var_vadibl_dn5 = assign19910_e17866_d_n5;
        locals.var_vadibl_dn6 = assign19910_e17866_d_n6;
        locals.var_vadibl_dn7 = assign19910_e17866_d_n7;
        locals.var_vadibl_dn8 = assign19910_e17866_d_n8;
        locals.var_vadibl_dn9 = assign19910_e17866_d_n9;
        locals.var_vadibl_dn10 = assign19910_e17866_d_n10;
        locals.var_vadibl_dn11 = assign19910_e17866_d_n11;
        locals.var_vadibl_dn12 = assign19910_e17866_d_n12;
        locals.var_vadibl_rv = 0.0;

        let (assign19920_e17877, assign19920_e17877_d_n3, assign19920_e17877_d_n4, assign19920_e17877_d_n5, assign19920_e17877_d_n6, assign19920_e17877_d_n7, assign19920_e17877_d_n8, assign19920_e17877_d_n9, assign19920_e17877_d_n10, assign19920_e17877_d_n11, assign19920_e17877_d_n12,) = {
    if ((locals.var_guard1234 != 0.0) && (locals.var_guard1235 == 0.0)) {
        let assign19920_e17874: f64 = (0.8 + locals.var_t7__blk815);
        let assign19920_e17875: f64 = (1.0 / assign19920_e17874);
        (assign19920_e17875, (-(locals.var_t7__blk815_dn3 / (assign19920_e17874 * assign19920_e17874))), (-(locals.var_t7__blk815_dn4 / (assign19920_e17874 * assign19920_e17874))), (-(locals.var_t7__blk815_dn5 / (assign19920_e17874 * assign19920_e17874))), (-(locals.var_t7__blk815_dn6 / (assign19920_e17874 * assign19920_e17874))), (-(locals.var_t7__blk815_dn7 / (assign19920_e17874 * assign19920_e17874))), (-(locals.var_t7__blk815_dn8 / (assign19920_e17874 * assign19920_e17874))), (-(locals.var_t7__blk815_dn9 / (assign19920_e17874 * assign19920_e17874))), (-(locals.var_t7__blk815_dn10 / (assign19920_e17874 * assign19920_e17874))), (-(locals.var_t7__blk815_dn11 / (assign19920_e17874 * assign19920_e17874))), (-(locals.var_t7__blk815_dn12 / (assign19920_e17874 * assign19920_e17874))),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign19920_e17877;
        locals.var_t4__blk812_dn3 = assign19920_e17877_d_n3;
        locals.var_t4__blk812_dn4 = assign19920_e17877_d_n4;
        locals.var_t4__blk812_dn5 = assign19920_e17877_d_n5;
        locals.var_t4__blk812_dn6 = assign19920_e17877_d_n6;
        locals.var_t4__blk812_dn7 = assign19920_e17877_d_n7;
        locals.var_t4__blk812_dn8 = assign19920_e17877_d_n8;
        locals.var_t4__blk812_dn9 = assign19920_e17877_d_n9;
        locals.var_t4__blk812_dn10 = assign19920_e17877_d_n10;
        locals.var_t4__blk812_dn11 = assign19920_e17877_d_n11;
        locals.var_t4__blk812_dn12 = assign19920_e17877_d_n12;
        locals.var_t4__blk812_rv = 0.0;

        let (assign19930_e17890, assign19930_e17890_d_n3, assign19930_e17890_d_n4, assign19930_e17890_d_n5, assign19930_e17890_d_n6, assign19930_e17890_d_n7, assign19930_e17890_d_n8, assign19930_e17890_d_n9, assign19930_e17890_d_n10, assign19930_e17890_d_n11, assign19930_e17890_d_n12,) = {
    if ((locals.var_guard1234 != 0.0) && (locals.var_guard1235 == 0.0)) {
        let assign19930_e17885: f64 = (20.0 * locals.var_t7__blk815);
        let assign19930_e17886: f64 = (17.0 + assign19930_e17885);
        let assign19930_e17888: f64 = (assign19930_e17886 * locals.var_t4__blk812);
        (assign19930_e17888, (((20.0 * locals.var_t7__blk815_dn3) * locals.var_t4__blk812) + (assign19930_e17886 * locals.var_t4__blk812_dn3)), (((20.0 * locals.var_t7__blk815_dn4) * locals.var_t4__blk812) + (assign19930_e17886 * locals.var_t4__blk812_dn4)), (((20.0 * locals.var_t7__blk815_dn5) * locals.var_t4__blk812) + (assign19930_e17886 * locals.var_t4__blk812_dn5)), (((20.0 * locals.var_t7__blk815_dn6) * locals.var_t4__blk812) + (assign19930_e17886 * locals.var_t4__blk812_dn6)), (((20.0 * locals.var_t7__blk815_dn7) * locals.var_t4__blk812) + (assign19930_e17886 * locals.var_t4__blk812_dn7)), (((20.0 * locals.var_t7__blk815_dn8) * locals.var_t4__blk812) + (assign19930_e17886 * locals.var_t4__blk812_dn8)), (((20.0 * locals.var_t7__blk815_dn9) * locals.var_t4__blk812) + (assign19930_e17886 * locals.var_t4__blk812_dn9)), (((20.0 * locals.var_t7__blk815_dn10) * locals.var_t4__blk812) + (assign19930_e17886 * locals.var_t4__blk812_dn10)), (((20.0 * locals.var_t7__blk815_dn11) * locals.var_t4__blk812) + (assign19930_e17886 * locals.var_t4__blk812_dn11)), (((20.0 * locals.var_t7__blk815_dn12) * locals.var_t4__blk812) + (assign19930_e17886 * locals.var_t4__blk812_dn12)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign19930_e17890;
        locals.var_t3__blk811_dn3 = assign19930_e17890_d_n3;
        locals.var_t3__blk811_dn4 = assign19930_e17890_d_n4;
        locals.var_t3__blk811_dn5 = assign19930_e17890_d_n5;
        locals.var_t3__blk811_dn6 = assign19930_e17890_d_n6;
        locals.var_t3__blk811_dn7 = assign19930_e17890_d_n7;
        locals.var_t3__blk811_dn8 = assign19930_e17890_d_n8;
        locals.var_t3__blk811_dn9 = assign19930_e17890_d_n9;
        locals.var_t3__blk811_dn10 = assign19930_e17890_d_n10;
        locals.var_t3__blk811_dn11 = assign19930_e17890_d_n11;
        locals.var_t3__blk811_dn12 = assign19930_e17890_d_n12;
        locals.var_t3__blk811_rv = 0.0;

        let (assign19940_e17899, assign19940_e17899_d_n3, assign19940_e17899_d_n4, assign19940_e17899_d_n5, assign19940_e17899_d_n6, assign19940_e17899_d_n7, assign19940_e17899_d_n8, assign19940_e17899_d_n9, assign19940_e17899_d_n10, assign19940_e17899_d_n11, assign19940_e17899_d_n12,) = {
    if ((locals.var_guard1234 != 0.0) && (locals.var_guard1235 == 0.0)) {
        let assign19940_e17897: f64 = (locals.var_vadibl * locals.var_t3__blk811);
        (assign19940_e17897, ((locals.var_vadibl_dn3 * locals.var_t3__blk811) + (locals.var_vadibl * locals.var_t3__blk811_dn3)), ((locals.var_vadibl_dn4 * locals.var_t3__blk811) + (locals.var_vadibl * locals.var_t3__blk811_dn4)), ((locals.var_vadibl_dn5 * locals.var_t3__blk811) + (locals.var_vadibl * locals.var_t3__blk811_dn5)), ((locals.var_vadibl_dn6 * locals.var_t3__blk811) + (locals.var_vadibl * locals.var_t3__blk811_dn6)), ((locals.var_vadibl_dn7 * locals.var_t3__blk811) + (locals.var_vadibl * locals.var_t3__blk811_dn7)), ((locals.var_vadibl_dn8 * locals.var_t3__blk811) + (locals.var_vadibl * locals.var_t3__blk811_dn8)), ((locals.var_vadibl_dn9 * locals.var_t3__blk811) + (locals.var_vadibl * locals.var_t3__blk811_dn9)), ((locals.var_vadibl_dn10 * locals.var_t3__blk811) + (locals.var_vadibl * locals.var_t3__blk811_dn10)), ((locals.var_vadibl_dn11 * locals.var_t3__blk811) + (locals.var_vadibl * locals.var_t3__blk811_dn11)), ((locals.var_vadibl_dn12 * locals.var_t3__blk811) + (locals.var_vadibl * locals.var_t3__blk811_dn12)),)
    } else {
        (locals.var_vadibl, locals.var_vadibl_dn3, locals.var_vadibl_dn4, locals.var_vadibl_dn5, locals.var_vadibl_dn6, locals.var_vadibl_dn7, locals.var_vadibl_dn8, locals.var_vadibl_dn9, locals.var_vadibl_dn10, locals.var_vadibl_dn11, locals.var_vadibl_dn12,)
    }
};
        locals.var_vadibl = assign19940_e17899;
        locals.var_vadibl_dn3 = assign19940_e17899_d_n3;
        locals.var_vadibl_dn4 = assign19940_e17899_d_n4;
        locals.var_vadibl_dn5 = assign19940_e17899_d_n5;
        locals.var_vadibl_dn6 = assign19940_e17899_d_n6;
        locals.var_vadibl_dn7 = assign19940_e17899_d_n7;
        locals.var_vadibl_dn8 = assign19940_e17899_d_n8;
        locals.var_vadibl_dn9 = assign19940_e17899_d_n9;
        locals.var_vadibl_dn10 = assign19940_e17899_d_n10;
        locals.var_vadibl_dn11 = assign19940_e17899_d_n11;
        locals.var_vadibl_dn12 = assign19940_e17899_d_n12;
        locals.var_vadibl_rv = 0.0;

        let (assign19950_e17904, assign19950_e17904_d_n3, assign19950_e17904_d_n4, assign19950_e17904_d_n5, assign19950_e17904_d_n6, assign19950_e17904_d_n7, assign19950_e17904_d_n8, assign19950_e17904_d_n9, assign19950_e17904_d_n10, assign19950_e17904_d_n11, assign19950_e17904_d_n12,) = {
    if (locals.var_guard1234 == 0.0) {
        (2.688117142e43, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vadibl, locals.var_vadibl_dn3, locals.var_vadibl_dn4, locals.var_vadibl_dn5, locals.var_vadibl_dn6, locals.var_vadibl_dn7, locals.var_vadibl_dn8, locals.var_vadibl_dn9, locals.var_vadibl_dn10, locals.var_vadibl_dn11, locals.var_vadibl_dn12,)
    }
};
        locals.var_vadibl = assign19950_e17904;
        locals.var_vadibl_dn3 = assign19950_e17904_d_n3;
        locals.var_vadibl_dn4 = assign19950_e17904_d_n4;
        locals.var_vadibl_dn5 = assign19950_e17904_d_n5;
        locals.var_vadibl_dn6 = assign19950_e17904_d_n6;
        locals.var_vadibl_dn7 = assign19950_e17904_d_n7;
        locals.var_vadibl_dn8 = assign19950_e17904_d_n8;
        locals.var_vadibl_dn9 = assign19950_e17904_d_n9;
        locals.var_vadibl_dn10 = assign19950_e17904_d_n10;
        locals.var_vadibl_dn11 = assign19950_e17904_d_n11;
        locals.var_vadibl_dn12 = assign19950_e17904_d_n12;
        locals.var_vadibl_rv = 0.0;

        let assign19960_e17907: f64 = (locals.var_pparam_b4soipditsd * locals.var_vds_1);
        locals.var_t0__blk808 = assign19960_e17907;
        locals.var_t0__blk808_dn3 = (locals.var_pparam_b4soipditsd_dn3 * locals.var_vds_1);
        locals.var_t0__blk808_dn4 = (locals.var_pparam_b4soipditsd_dn4 * locals.var_vds_1);
        locals.var_t0__blk808_dn5 = (locals.var_pparam_b4soipditsd_dn5 * locals.var_vds_1);
        locals.var_t0__blk808_dn6 = (locals.var_pparam_b4soipditsd_dn6 * locals.var_vds_1);
        locals.var_t0__blk808_dn7 = ((locals.var_pparam_b4soipditsd_dn7 * locals.var_vds_1) + (locals.var_pparam_b4soipditsd * locals.var_vds_1_dn7));
        locals.var_t0__blk808_dn8 = ((locals.var_pparam_b4soipditsd_dn8 * locals.var_vds_1) + (locals.var_pparam_b4soipditsd * locals.var_vds_1_dn8));
        locals.var_t0__blk808_dn9 = (locals.var_pparam_b4soipditsd_dn9 * locals.var_vds_1);
        locals.var_t0__blk808_dn10 = (locals.var_pparam_b4soipditsd_dn10 * locals.var_vds_1);
        locals.var_t0__blk808_dn11 = (locals.var_pparam_b4soipditsd_dn11 * locals.var_vds_1);
        locals.var_t0__blk808_dn12 = (locals.var_pparam_b4soipditsd_dn12 * locals.var_vds_1);
        locals.var_t0__blk808_rv = 0.0;

        let assign19970_e17910: f64 = if locals.var_t0__blk808 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1236 = assign19970_e17910;
        locals.var_guard1236_rv = 0.0;

        let (assign19980_e17914, assign19980_e17914_d_n3, assign19980_e17914_d_n4, assign19980_e17914_d_n5, assign19980_e17914_d_n6, assign19980_e17914_d_n7, assign19980_e17914_d_n8, assign19980_e17914_d_n9, assign19980_e17914_d_n10, assign19980_e17914_d_n11, assign19980_e17914_d_n12,) = {
    if (locals.var_guard1236 != 0.0) {
        (2.688117142e43, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign19980_e17914;
        locals.var_t1__blk809_dn3 = assign19980_e17914_d_n3;
        locals.var_t1__blk809_dn4 = assign19980_e17914_d_n4;
        locals.var_t1__blk809_dn5 = assign19980_e17914_d_n5;
        locals.var_t1__blk809_dn6 = assign19980_e17914_d_n6;
        locals.var_t1__blk809_dn7 = assign19980_e17914_d_n7;
        locals.var_t1__blk809_dn8 = assign19980_e17914_d_n8;
        locals.var_t1__blk809_dn9 = assign19980_e17914_d_n9;
        locals.var_t1__blk809_dn10 = assign19980_e17914_d_n10;
        locals.var_t1__blk809_dn11 = assign19980_e17914_d_n11;
        locals.var_t1__blk809_dn12 = assign19980_e17914_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign19990_e17920, assign19990_e17920_d_n3, assign19990_e17920_d_n4, assign19990_e17920_d_n5, assign19990_e17920_d_n6, assign19990_e17920_d_n7, assign19990_e17920_d_n8, assign19990_e17920_d_n9, assign19990_e17920_d_n10, assign19990_e17920_d_n11, assign19990_e17920_d_n12,) = {
    if (locals.var_guard1236 == 0.0) {
        let assign19990_e17918: f64 = (locals.var_t0__blk808).exp();
        (assign19990_e17918, (assign19990_e17918 * locals.var_t0__blk808_dn3), (assign19990_e17918 * locals.var_t0__blk808_dn4), (assign19990_e17918 * locals.var_t0__blk808_dn5), (assign19990_e17918 * locals.var_t0__blk808_dn6), (assign19990_e17918 * locals.var_t0__blk808_dn7), (assign19990_e17918 * locals.var_t0__blk808_dn8), (assign19990_e17918 * locals.var_t0__blk808_dn9), (assign19990_e17918 * locals.var_t0__blk808_dn10), (assign19990_e17918 * locals.var_t0__blk808_dn11), (assign19990_e17918 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign19990_e17920;
        locals.var_t1__blk809_dn3 = assign19990_e17920_d_n3;
        locals.var_t1__blk809_dn4 = assign19990_e17920_d_n4;
        locals.var_t1__blk809_dn5 = assign19990_e17920_d_n5;
        locals.var_t1__blk809_dn6 = assign19990_e17920_d_n6;
        locals.var_t1__blk809_dn7 = assign19990_e17920_d_n7;
        locals.var_t1__blk809_dn8 = assign19990_e17920_d_n8;
        locals.var_t1__blk809_dn9 = assign19990_e17920_d_n9;
        locals.var_t1__blk809_dn10 = assign19990_e17920_d_n10;
        locals.var_t1__blk809_dn11 = assign19990_e17920_d_n11;
        locals.var_t1__blk809_dn12 = assign19990_e17920_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let assign20000_e17923: f64 = if locals.var_pparam_b4soipdits > 3.720075976e-44 { 1.0 } else { 0.0 };
        locals.var_guard1237 = assign20000_e17923;
        locals.var_guard1237_rv = 0.0;

        let (assign20010_e17931, assign20010_e17931_d_n3, assign20010_e17931_d_n4, assign20010_e17931_d_n5, assign20010_e17931_d_n6, assign20010_e17931_d_n7, assign20010_e17931_d_n8, assign20010_e17931_d_n9, assign20010_e17931_d_n10, assign20010_e17931_d_n11, assign20010_e17931_d_n12,) = {
    if (locals.var_guard1237 != 0.0) {
        let assign20010_e17928: f64 = (p.p283 * locals.var_leff);
        let assign20010_e17929: f64 = (1.0 + assign20010_e17928);
        (assign20010_e17929, (p.p283 * locals.var_leff_dn3), (p.p283 * locals.var_leff_dn4), (p.p283 * locals.var_leff_dn5), (p.p283 * locals.var_leff_dn6), (p.p283 * locals.var_leff_dn7), (p.p283 * locals.var_leff_dn8), (p.p283 * locals.var_leff_dn9), (p.p283 * locals.var_leff_dn10), (p.p283 * locals.var_leff_dn11), (p.p283 * locals.var_leff_dn12),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign20010_e17931;
        locals.var_t2__blk810_dn3 = assign20010_e17931_d_n3;
        locals.var_t2__blk810_dn4 = assign20010_e17931_d_n4;
        locals.var_t2__blk810_dn5 = assign20010_e17931_d_n5;
        locals.var_t2__blk810_dn6 = assign20010_e17931_d_n6;
        locals.var_t2__blk810_dn7 = assign20010_e17931_d_n7;
        locals.var_t2__blk810_dn8 = assign20010_e17931_d_n8;
        locals.var_t2__blk810_dn9 = assign20010_e17931_d_n9;
        locals.var_t2__blk810_dn10 = assign20010_e17931_d_n10;
        locals.var_t2__blk810_dn11 = assign20010_e17931_d_n11;
        locals.var_t2__blk810_dn12 = assign20010_e17931_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign20020_e17941, assign20020_e17941_d_n3, assign20020_e17941_d_n4, assign20020_e17941_d_n5, assign20020_e17941_d_n6, assign20020_e17941_d_n7, assign20020_e17941_d_n8, assign20020_e17941_d_n9, assign20020_e17941_d_n10, assign20020_e17941_d_n11, assign20020_e17941_d_n12,) = {
    if (locals.var_guard1237 != 0.0) {
        let assign20020_e17936: f64 = (locals.var_t2__blk810 * locals.var_t1__blk809);
        let assign20020_e17937: f64 = (1.0 + assign20020_e17936);
        let assign20020_e17939: f64 = (assign20020_e17937 / locals.var_pparam_b4soipdits);
        (assign20020_e17939, (((((locals.var_t2__blk810_dn3 * locals.var_t1__blk809) + (locals.var_t2__blk810 * locals.var_t1__blk809_dn3)) * locals.var_pparam_b4soipdits) - (assign20020_e17937 * locals.var_pparam_b4soipdits_dn3)) / (locals.var_pparam_b4soipdits * locals.var_pparam_b4soipdits)), (((((locals.var_t2__blk810_dn4 * locals.var_t1__blk809) + (locals.var_t2__blk810 * locals.var_t1__blk809_dn4)) * locals.var_pparam_b4soipdits) - (assign20020_e17937 * locals.var_pparam_b4soipdits_dn4)) / (locals.var_pparam_b4soipdits * locals.var_pparam_b4soipdits)), (((((locals.var_t2__blk810_dn5 * locals.var_t1__blk809) + (locals.var_t2__blk810 * locals.var_t1__blk809_dn5)) * locals.var_pparam_b4soipdits) - (assign20020_e17937 * locals.var_pparam_b4soipdits_dn5)) / (locals.var_pparam_b4soipdits * locals.var_pparam_b4soipdits)), (((((locals.var_t2__blk810_dn6 * locals.var_t1__blk809) + (locals.var_t2__blk810 * locals.var_t1__blk809_dn6)) * locals.var_pparam_b4soipdits) - (assign20020_e17937 * locals.var_pparam_b4soipdits_dn6)) / (locals.var_pparam_b4soipdits * locals.var_pparam_b4soipdits)), (((((locals.var_t2__blk810_dn7 * locals.var_t1__blk809) + (locals.var_t2__blk810 * locals.var_t1__blk809_dn7)) * locals.var_pparam_b4soipdits) - (assign20020_e17937 * locals.var_pparam_b4soipdits_dn7)) / (locals.var_pparam_b4soipdits * locals.var_pparam_b4soipdits)), (((((locals.var_t2__blk810_dn8 * locals.var_t1__blk809) + (locals.var_t2__blk810 * locals.var_t1__blk809_dn8)) * locals.var_pparam_b4soipdits) - (assign20020_e17937 * locals.var_pparam_b4soipdits_dn8)) / (locals.var_pparam_b4soipdits * locals.var_pparam_b4soipdits)), (((((locals.var_t2__blk810_dn9 * locals.var_t1__blk809) + (locals.var_t2__blk810 * locals.var_t1__blk809_dn9)) * locals.var_pparam_b4soipdits) - (assign20020_e17937 * locals.var_pparam_b4soipdits_dn9)) / (locals.var_pparam_b4soipdits * locals.var_pparam_b4soipdits)), (((((locals.var_t2__blk810_dn10 * locals.var_t1__blk809) + (locals.var_t2__blk810 * locals.var_t1__blk809_dn10)) * locals.var_pparam_b4soipdits) - (assign20020_e17937 * locals.var_pparam_b4soipdits_dn10)) / (locals.var_pparam_b4soipdits * locals.var_pparam_b4soipdits)), (((((locals.var_t2__blk810_dn11 * locals.var_t1__blk809) + (locals.var_t2__blk810 * locals.var_t1__blk809_dn11)) * locals.var_pparam_b4soipdits) - (assign20020_e17937 * locals.var_pparam_b4soipdits_dn11)) / (locals.var_pparam_b4soipdits * locals.var_pparam_b4soipdits)), (((((locals.var_t2__blk810_dn12 * locals.var_t1__blk809) + (locals.var_t2__blk810 * locals.var_t1__blk809_dn12)) * locals.var_pparam_b4soipdits) - (assign20020_e17937 * locals.var_pparam_b4soipdits_dn12)) / (locals.var_pparam_b4soipdits * locals.var_pparam_b4soipdits)),)
    } else {
        (locals.var_vadits, locals.var_vadits_dn3, locals.var_vadits_dn4, locals.var_vadits_dn5, locals.var_vadits_dn6, locals.var_vadits_dn7, locals.var_vadits_dn8, locals.var_vadits_dn9, locals.var_vadits_dn10, locals.var_vadits_dn11, locals.var_vadits_dn12,)
    }
};
        locals.var_vadits = assign20020_e17941;
        locals.var_vadits_dn3 = assign20020_e17941_d_n3;
        locals.var_vadits_dn4 = assign20020_e17941_d_n4;
        locals.var_vadits_dn5 = assign20020_e17941_d_n5;
        locals.var_vadits_dn6 = assign20020_e17941_d_n6;
        locals.var_vadits_dn7 = assign20020_e17941_d_n7;
        locals.var_vadits_dn8 = assign20020_e17941_d_n8;
        locals.var_vadits_dn9 = assign20020_e17941_d_n9;
        locals.var_vadits_dn10 = assign20020_e17941_d_n10;
        locals.var_vadits_dn11 = assign20020_e17941_d_n11;
        locals.var_vadits_dn12 = assign20020_e17941_d_n12;
        locals.var_vadits_rv = 0.0;

        let (assign20030_e17947, assign20030_e17947_d_n3, assign20030_e17947_d_n4, assign20030_e17947_d_n5, assign20030_e17947_d_n6, assign20030_e17947_d_n7, assign20030_e17947_d_n8, assign20030_e17947_d_n9, assign20030_e17947_d_n10, assign20030_e17947_d_n11, assign20030_e17947_d_n12,) = {
    if (locals.var_guard1237 != 0.0) {
        let assign20030_e17945: f64 = (locals.var_vadits * locals.var_fp);
        (assign20030_e17945, ((locals.var_vadits_dn3 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn3)), ((locals.var_vadits_dn4 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn4)), ((locals.var_vadits_dn5 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn5)), ((locals.var_vadits_dn6 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn6)), ((locals.var_vadits_dn7 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn7)), ((locals.var_vadits_dn8 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn8)), ((locals.var_vadits_dn9 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn9)), ((locals.var_vadits_dn10 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn10)), ((locals.var_vadits_dn11 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn11)), ((locals.var_vadits_dn12 * locals.var_fp) + (locals.var_vadits * locals.var_fp_dn12)),)
    } else {
        (locals.var_vadits, locals.var_vadits_dn3, locals.var_vadits_dn4, locals.var_vadits_dn5, locals.var_vadits_dn6, locals.var_vadits_dn7, locals.var_vadits_dn8, locals.var_vadits_dn9, locals.var_vadits_dn10, locals.var_vadits_dn11, locals.var_vadits_dn12,)
    }
};
        locals.var_vadits = assign20030_e17947;
        locals.var_vadits_dn3 = assign20030_e17947_d_n3;
        locals.var_vadits_dn4 = assign20030_e17947_d_n4;
        locals.var_vadits_dn5 = assign20030_e17947_d_n5;
        locals.var_vadits_dn6 = assign20030_e17947_d_n6;
        locals.var_vadits_dn7 = assign20030_e17947_d_n7;
        locals.var_vadits_dn8 = assign20030_e17947_d_n8;
        locals.var_vadits_dn9 = assign20030_e17947_d_n9;
        locals.var_vadits_dn10 = assign20030_e17947_d_n10;
        locals.var_vadits_dn11 = assign20030_e17947_d_n11;
        locals.var_vadits_dn12 = assign20030_e17947_d_n12;
        locals.var_vadits_rv = 0.0;

        let (assign20040_e17952, assign20040_e17952_d_n3, assign20040_e17952_d_n4, assign20040_e17952_d_n5, assign20040_e17952_d_n6, assign20040_e17952_d_n7, assign20040_e17952_d_n8, assign20040_e17952_d_n9, assign20040_e17952_d_n10, assign20040_e17952_d_n11, assign20040_e17952_d_n12,) = {
    if (locals.var_guard1237 == 0.0) {
        (2.688117142e43, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vadits, locals.var_vadits_dn3, locals.var_vadits_dn4, locals.var_vadits_dn5, locals.var_vadits_dn6, locals.var_vadits_dn7, locals.var_vadits_dn8, locals.var_vadits_dn9, locals.var_vadits_dn10, locals.var_vadits_dn11, locals.var_vadits_dn12,)
    }
};
        locals.var_vadits = assign20040_e17952;
        locals.var_vadits_dn3 = assign20040_e17952_d_n3;
        locals.var_vadits_dn4 = assign20040_e17952_d_n4;
        locals.var_vadits_dn5 = assign20040_e17952_d_n5;
        locals.var_vadits_dn6 = assign20040_e17952_d_n6;
        locals.var_vadits_dn7 = assign20040_e17952_d_n7;
        locals.var_vadits_dn8 = assign20040_e17952_d_n8;
        locals.var_vadits_dn9 = assign20040_e17952_d_n9;
        locals.var_vadits_dn10 = assign20040_e17952_d_n10;
        locals.var_vadits_dn11 = assign20040_e17952_d_n11;
        locals.var_vadits_dn12 = assign20040_e17952_d_n12;
        locals.var_vadits_rv = 0.0;

        let assign20050_e17955: f64 = (locals.var_pparam_b4soipvag / locals.var_esatl);
        locals.var_t8 = assign20050_e17955;
        locals.var_t8_dn3 = (((locals.var_pparam_b4soipvag_dn3 * locals.var_esatl) - (locals.var_pparam_b4soipvag * locals.var_esatl_dn3)) / (locals.var_esatl * locals.var_esatl));
        locals.var_t8_dn4 = (((locals.var_pparam_b4soipvag_dn4 * locals.var_esatl) - (locals.var_pparam_b4soipvag * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl));
        locals.var_t8_dn5 = (((locals.var_pparam_b4soipvag_dn5 * locals.var_esatl) - (locals.var_pparam_b4soipvag * locals.var_esatl_dn5)) / (locals.var_esatl * locals.var_esatl));
        locals.var_t8_dn6 = (((locals.var_pparam_b4soipvag_dn6 * locals.var_esatl) - (locals.var_pparam_b4soipvag * locals.var_esatl_dn6)) / (locals.var_esatl * locals.var_esatl));
        locals.var_t8_dn7 = (((locals.var_pparam_b4soipvag_dn7 * locals.var_esatl) - (locals.var_pparam_b4soipvag * locals.var_esatl_dn7)) / (locals.var_esatl * locals.var_esatl));
        locals.var_t8_dn8 = (((locals.var_pparam_b4soipvag_dn8 * locals.var_esatl) - (locals.var_pparam_b4soipvag * locals.var_esatl_dn8)) / (locals.var_esatl * locals.var_esatl));
        locals.var_t8_dn9 = (((locals.var_pparam_b4soipvag_dn9 * locals.var_esatl) - (locals.var_pparam_b4soipvag * locals.var_esatl_dn9)) / (locals.var_esatl * locals.var_esatl));
        locals.var_t8_dn10 = (((locals.var_pparam_b4soipvag_dn10 * locals.var_esatl) - (locals.var_pparam_b4soipvag * locals.var_esatl_dn10)) / (locals.var_esatl * locals.var_esatl));
        locals.var_t8_dn11 = (((locals.var_pparam_b4soipvag_dn11 * locals.var_esatl) - (locals.var_pparam_b4soipvag * locals.var_esatl_dn11)) / (locals.var_esatl * locals.var_esatl));
        locals.var_t8_dn12 = (((locals.var_pparam_b4soipvag_dn12 * locals.var_esatl) - (locals.var_pparam_b4soipvag * locals.var_esatl_dn12)) / (locals.var_esatl * locals.var_esatl));
        locals.var_t8_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_61(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign20060_e17958: f64 = (locals.var_t8 * locals.var_vgsteff__blk840);
        locals.var_t9 = assign20060_e17958;
        locals.var_t9_dn3 = ((locals.var_t8_dn3 * locals.var_vgsteff__blk840) + (locals.var_t8 * locals.var_vgsteff__blk840_dn3));
        locals.var_t9_dn4 = ((locals.var_t8_dn4 * locals.var_vgsteff__blk840) + (locals.var_t8 * locals.var_vgsteff__blk840_dn4));
        locals.var_t9_dn5 = ((locals.var_t8_dn5 * locals.var_vgsteff__blk840) + (locals.var_t8 * locals.var_vgsteff__blk840_dn5));
        locals.var_t9_dn6 = ((locals.var_t8_dn6 * locals.var_vgsteff__blk840) + (locals.var_t8 * locals.var_vgsteff__blk840_dn6));
        locals.var_t9_dn7 = ((locals.var_t8_dn7 * locals.var_vgsteff__blk840) + (locals.var_t8 * locals.var_vgsteff__blk840_dn7));
        locals.var_t9_dn8 = ((locals.var_t8_dn8 * locals.var_vgsteff__blk840) + (locals.var_t8 * locals.var_vgsteff__blk840_dn8));
        locals.var_t9_dn9 = ((locals.var_t8_dn9 * locals.var_vgsteff__blk840) + (locals.var_t8 * locals.var_vgsteff__blk840_dn9));
        locals.var_t9_dn10 = ((locals.var_t8_dn10 * locals.var_vgsteff__blk840) + (locals.var_t8 * locals.var_vgsteff__blk840_dn10));
        locals.var_t9_dn11 = ((locals.var_t8_dn11 * locals.var_vgsteff__blk840) + (locals.var_t8 * locals.var_vgsteff__blk840_dn11));
        locals.var_t9_dn12 = ((locals.var_t8_dn12 * locals.var_vgsteff__blk840) + (locals.var_t8 * locals.var_vgsteff__blk840_dn12));
        locals.var_t9_rv = 0.0;

        let assign20070_e17961: f64 = (-0.9);
        let assign20070_e17962: f64 = if locals.var_t9 > assign20070_e17961 { 1.0 } else { 0.0 };
        locals.var_guard1238 = assign20070_e17962;
        locals.var_guard1238_rv = 0.0;

        let (assign20080_e17968, assign20080_e17968_d_n3, assign20080_e17968_d_n4, assign20080_e17968_d_n5, assign20080_e17968_d_n6, assign20080_e17968_d_n7, assign20080_e17968_d_n8, assign20080_e17968_d_n9, assign20080_e17968_d_n10, assign20080_e17968_d_n11, assign20080_e17968_d_n12,) = {
    if (locals.var_guard1238 != 0.0) {
        let assign20080_e17966: f64 = (1.0 + locals.var_t9);
        (assign20080_e17966, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign20080_e17968;
        locals.var_t0__blk808_dn3 = assign20080_e17968_d_n3;
        locals.var_t0__blk808_dn4 = assign20080_e17968_d_n4;
        locals.var_t0__blk808_dn5 = assign20080_e17968_d_n5;
        locals.var_t0__blk808_dn6 = assign20080_e17968_d_n6;
        locals.var_t0__blk808_dn7 = assign20080_e17968_d_n7;
        locals.var_t0__blk808_dn8 = assign20080_e17968_d_n8;
        locals.var_t0__blk808_dn9 = assign20080_e17968_d_n9;
        locals.var_t0__blk808_dn10 = assign20080_e17968_d_n10;
        locals.var_t0__blk808_dn11 = assign20080_e17968_d_n11;
        locals.var_t0__blk808_dn12 = assign20080_e17968_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign20090_e17979, assign20090_e17979_d_n3, assign20090_e17979_d_n4, assign20090_e17979_d_n5, assign20090_e17979_d_n6, assign20090_e17979_d_n7, assign20090_e17979_d_n8, assign20090_e17979_d_n9, assign20090_e17979_d_n10, assign20090_e17979_d_n11, assign20090_e17979_d_n12,) = {
    if (locals.var_guard1238 == 0.0) {
        let assign20090_e17975: f64 = (20.0 * locals.var_t9);
        let assign20090_e17976: f64 = (17.0 + assign20090_e17975);
        let assign20090_e17977: f64 = (1.0 / assign20090_e17976);
        (assign20090_e17977, (-((20.0 * locals.var_t9_dn3) / (assign20090_e17976 * assign20090_e17976))), (-((20.0 * locals.var_t9_dn4) / (assign20090_e17976 * assign20090_e17976))), (-((20.0 * locals.var_t9_dn5) / (assign20090_e17976 * assign20090_e17976))), (-((20.0 * locals.var_t9_dn6) / (assign20090_e17976 * assign20090_e17976))), (-((20.0 * locals.var_t9_dn7) / (assign20090_e17976 * assign20090_e17976))), (-((20.0 * locals.var_t9_dn8) / (assign20090_e17976 * assign20090_e17976))), (-((20.0 * locals.var_t9_dn9) / (assign20090_e17976 * assign20090_e17976))), (-((20.0 * locals.var_t9_dn10) / (assign20090_e17976 * assign20090_e17976))), (-((20.0 * locals.var_t9_dn11) / (assign20090_e17976 * assign20090_e17976))), (-((20.0 * locals.var_t9_dn12) / (assign20090_e17976 * assign20090_e17976))),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign20090_e17979;
        locals.var_t1__blk809_dn3 = assign20090_e17979_d_n3;
        locals.var_t1__blk809_dn4 = assign20090_e17979_d_n4;
        locals.var_t1__blk809_dn5 = assign20090_e17979_d_n5;
        locals.var_t1__blk809_dn6 = assign20090_e17979_d_n6;
        locals.var_t1__blk809_dn7 = assign20090_e17979_d_n7;
        locals.var_t1__blk809_dn8 = assign20090_e17979_d_n8;
        locals.var_t1__blk809_dn9 = assign20090_e17979_d_n9;
        locals.var_t1__blk809_dn10 = assign20090_e17979_d_n10;
        locals.var_t1__blk809_dn11 = assign20090_e17979_d_n11;
        locals.var_t1__blk809_dn12 = assign20090_e17979_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign20100_e17988, assign20100_e17988_d_n3, assign20100_e17988_d_n4, assign20100_e17988_d_n5, assign20100_e17988_d_n6, assign20100_e17988_d_n7, assign20100_e17988_d_n8, assign20100_e17988_d_n9, assign20100_e17988_d_n10, assign20100_e17988_d_n11, assign20100_e17988_d_n12,) = {
    if (locals.var_guard1238 == 0.0) {
        let assign20100_e17984: f64 = (0.8 + locals.var_t9);
        let assign20100_e17986: f64 = (assign20100_e17984 * locals.var_t1__blk809);
        (assign20100_e17986, ((locals.var_t9_dn3 * locals.var_t1__blk809) + (assign20100_e17984 * locals.var_t1__blk809_dn3)), ((locals.var_t9_dn4 * locals.var_t1__blk809) + (assign20100_e17984 * locals.var_t1__blk809_dn4)), ((locals.var_t9_dn5 * locals.var_t1__blk809) + (assign20100_e17984 * locals.var_t1__blk809_dn5)), ((locals.var_t9_dn6 * locals.var_t1__blk809) + (assign20100_e17984 * locals.var_t1__blk809_dn6)), ((locals.var_t9_dn7 * locals.var_t1__blk809) + (assign20100_e17984 * locals.var_t1__blk809_dn7)), ((locals.var_t9_dn8 * locals.var_t1__blk809) + (assign20100_e17984 * locals.var_t1__blk809_dn8)), ((locals.var_t9_dn9 * locals.var_t1__blk809) + (assign20100_e17984 * locals.var_t1__blk809_dn9)), ((locals.var_t9_dn10 * locals.var_t1__blk809) + (assign20100_e17984 * locals.var_t1__blk809_dn10)), ((locals.var_t9_dn11 * locals.var_t1__blk809) + (assign20100_e17984 * locals.var_t1__blk809_dn11)), ((locals.var_t9_dn12 * locals.var_t1__blk809) + (assign20100_e17984 * locals.var_t1__blk809_dn12)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign20100_e17988;
        locals.var_t0__blk808_dn3 = assign20100_e17988_d_n3;
        locals.var_t0__blk808_dn4 = assign20100_e17988_d_n4;
        locals.var_t0__blk808_dn5 = assign20100_e17988_d_n5;
        locals.var_t0__blk808_dn6 = assign20100_e17988_d_n6;
        locals.var_t0__blk808_dn7 = assign20100_e17988_d_n7;
        locals.var_t0__blk808_dn8 = assign20100_e17988_d_n8;
        locals.var_t0__blk808_dn9 = assign20100_e17988_d_n9;
        locals.var_t0__blk808_dn10 = assign20100_e17988_d_n10;
        locals.var_t0__blk808_dn11 = assign20100_e17988_d_n11;
        locals.var_t0__blk808_dn12 = assign20100_e17988_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let assign20110_e17991: f64 = (locals.var_vaclm + locals.var_vadibl);
        locals.var_tmp3__blk836 = assign20110_e17991;
        locals.var_tmp3__blk836_dn3 = (locals.var_vaclm_dn3 + locals.var_vadibl_dn3);
        locals.var_tmp3__blk836_dn4 = (locals.var_vaclm_dn4 + locals.var_vadibl_dn4);
        locals.var_tmp3__blk836_dn5 = (locals.var_vaclm_dn5 + locals.var_vadibl_dn5);
        locals.var_tmp3__blk836_dn6 = (locals.var_vaclm_dn6 + locals.var_vadibl_dn6);
        locals.var_tmp3__blk836_dn7 = (locals.var_vaclm_dn7 + locals.var_vadibl_dn7);
        locals.var_tmp3__blk836_dn8 = (locals.var_vaclm_dn8 + locals.var_vadibl_dn8);
        locals.var_tmp3__blk836_dn9 = (locals.var_vaclm_dn9 + locals.var_vadibl_dn9);
        locals.var_tmp3__blk836_dn10 = (locals.var_vaclm_dn10 + locals.var_vadibl_dn10);
        locals.var_tmp3__blk836_dn11 = (locals.var_vaclm_dn11 + locals.var_vadibl_dn11);
        locals.var_tmp3__blk836_dn12 = (locals.var_vaclm_dn12 + locals.var_vadibl_dn12);
        locals.var_tmp3__blk836_rv = 0.0;

        let assign20120_e17994: f64 = (locals.var_vaclm * locals.var_vadibl);
        let assign20120_e17996: f64 = (assign20120_e17994 / locals.var_tmp3__blk836);
        locals.var_t1__blk809 = assign20120_e17996;
        locals.var_t1__blk809_dn3 = (((((locals.var_vaclm_dn3 * locals.var_vadibl) + (locals.var_vaclm * locals.var_vadibl_dn3)) * locals.var_tmp3__blk836) - (assign20120_e17994 * locals.var_tmp3__blk836_dn3)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836));
        locals.var_t1__blk809_dn4 = (((((locals.var_vaclm_dn4 * locals.var_vadibl) + (locals.var_vaclm * locals.var_vadibl_dn4)) * locals.var_tmp3__blk836) - (assign20120_e17994 * locals.var_tmp3__blk836_dn4)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836));
        locals.var_t1__blk809_dn5 = (((((locals.var_vaclm_dn5 * locals.var_vadibl) + (locals.var_vaclm * locals.var_vadibl_dn5)) * locals.var_tmp3__blk836) - (assign20120_e17994 * locals.var_tmp3__blk836_dn5)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836));
        locals.var_t1__blk809_dn6 = (((((locals.var_vaclm_dn6 * locals.var_vadibl) + (locals.var_vaclm * locals.var_vadibl_dn6)) * locals.var_tmp3__blk836) - (assign20120_e17994 * locals.var_tmp3__blk836_dn6)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836));
        locals.var_t1__blk809_dn7 = (((((locals.var_vaclm_dn7 * locals.var_vadibl) + (locals.var_vaclm * locals.var_vadibl_dn7)) * locals.var_tmp3__blk836) - (assign20120_e17994 * locals.var_tmp3__blk836_dn7)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836));
        locals.var_t1__blk809_dn8 = (((((locals.var_vaclm_dn8 * locals.var_vadibl) + (locals.var_vaclm * locals.var_vadibl_dn8)) * locals.var_tmp3__blk836) - (assign20120_e17994 * locals.var_tmp3__blk836_dn8)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836));
        locals.var_t1__blk809_dn9 = (((((locals.var_vaclm_dn9 * locals.var_vadibl) + (locals.var_vaclm * locals.var_vadibl_dn9)) * locals.var_tmp3__blk836) - (assign20120_e17994 * locals.var_tmp3__blk836_dn9)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836));
        locals.var_t1__blk809_dn10 = (((((locals.var_vaclm_dn10 * locals.var_vadibl) + (locals.var_vaclm * locals.var_vadibl_dn10)) * locals.var_tmp3__blk836) - (assign20120_e17994 * locals.var_tmp3__blk836_dn10)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836));
        locals.var_t1__blk809_dn11 = (((((locals.var_vaclm_dn11 * locals.var_vadibl) + (locals.var_vaclm * locals.var_vadibl_dn11)) * locals.var_tmp3__blk836) - (assign20120_e17994 * locals.var_tmp3__blk836_dn11)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836));
        locals.var_t1__blk809_dn12 = (((((locals.var_vaclm_dn12 * locals.var_vadibl) + (locals.var_vaclm * locals.var_vadibl_dn12)) * locals.var_tmp3__blk836) - (assign20120_e17994 * locals.var_tmp3__blk836_dn12)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836));
        locals.var_t1__blk809_rv = 0.0;

        let assign20130_e17999: f64 = (locals.var_t1__blk809 + locals.var_vadits);
        locals.var_tmp3__blk836 = assign20130_e17999;
        locals.var_tmp3__blk836_dn3 = (locals.var_t1__blk809_dn3 + locals.var_vadits_dn3);
        locals.var_tmp3__blk836_dn4 = (locals.var_t1__blk809_dn4 + locals.var_vadits_dn4);
        locals.var_tmp3__blk836_dn5 = (locals.var_t1__blk809_dn5 + locals.var_vadits_dn5);
        locals.var_tmp3__blk836_dn6 = (locals.var_t1__blk809_dn6 + locals.var_vadits_dn6);
        locals.var_tmp3__blk836_dn7 = (locals.var_t1__blk809_dn7 + locals.var_vadits_dn7);
        locals.var_tmp3__blk836_dn8 = (locals.var_t1__blk809_dn8 + locals.var_vadits_dn8);
        locals.var_tmp3__blk836_dn9 = (locals.var_t1__blk809_dn9 + locals.var_vadits_dn9);
        locals.var_tmp3__blk836_dn10 = (locals.var_t1__blk809_dn10 + locals.var_vadits_dn10);
        locals.var_tmp3__blk836_dn11 = (locals.var_t1__blk809_dn11 + locals.var_vadits_dn11);
        locals.var_tmp3__blk836_dn12 = (locals.var_t1__blk809_dn12 + locals.var_vadits_dn12);
        locals.var_tmp3__blk836_rv = 0.0;

        let assign20140_e18002: f64 = (locals.var_t1__blk809 * locals.var_vadits);
        let assign20140_e18004: f64 = (assign20140_e18002 / locals.var_tmp3__blk836);
        locals.var_t2__blk810 = assign20140_e18004;
        locals.var_t2__blk810_dn3 = (((((locals.var_t1__blk809_dn3 * locals.var_vadits) + (locals.var_t1__blk809 * locals.var_vadits_dn3)) * locals.var_tmp3__blk836) - (assign20140_e18002 * locals.var_tmp3__blk836_dn3)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836));
        locals.var_t2__blk810_dn4 = (((((locals.var_t1__blk809_dn4 * locals.var_vadits) + (locals.var_t1__blk809 * locals.var_vadits_dn4)) * locals.var_tmp3__blk836) - (assign20140_e18002 * locals.var_tmp3__blk836_dn4)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836));
        locals.var_t2__blk810_dn5 = (((((locals.var_t1__blk809_dn5 * locals.var_vadits) + (locals.var_t1__blk809 * locals.var_vadits_dn5)) * locals.var_tmp3__blk836) - (assign20140_e18002 * locals.var_tmp3__blk836_dn5)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836));
        locals.var_t2__blk810_dn6 = (((((locals.var_t1__blk809_dn6 * locals.var_vadits) + (locals.var_t1__blk809 * locals.var_vadits_dn6)) * locals.var_tmp3__blk836) - (assign20140_e18002 * locals.var_tmp3__blk836_dn6)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836));
        locals.var_t2__blk810_dn7 = (((((locals.var_t1__blk809_dn7 * locals.var_vadits) + (locals.var_t1__blk809 * locals.var_vadits_dn7)) * locals.var_tmp3__blk836) - (assign20140_e18002 * locals.var_tmp3__blk836_dn7)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836));
        locals.var_t2__blk810_dn8 = (((((locals.var_t1__blk809_dn8 * locals.var_vadits) + (locals.var_t1__blk809 * locals.var_vadits_dn8)) * locals.var_tmp3__blk836) - (assign20140_e18002 * locals.var_tmp3__blk836_dn8)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836));
        locals.var_t2__blk810_dn9 = (((((locals.var_t1__blk809_dn9 * locals.var_vadits) + (locals.var_t1__blk809 * locals.var_vadits_dn9)) * locals.var_tmp3__blk836) - (assign20140_e18002 * locals.var_tmp3__blk836_dn9)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836));
        locals.var_t2__blk810_dn10 = (((((locals.var_t1__blk809_dn10 * locals.var_vadits) + (locals.var_t1__blk809 * locals.var_vadits_dn10)) * locals.var_tmp3__blk836) - (assign20140_e18002 * locals.var_tmp3__blk836_dn10)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836));
        locals.var_t2__blk810_dn11 = (((((locals.var_t1__blk809_dn11 * locals.var_vadits) + (locals.var_t1__blk809 * locals.var_vadits_dn11)) * locals.var_tmp3__blk836) - (assign20140_e18002 * locals.var_tmp3__blk836_dn11)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836));
        locals.var_t2__blk810_dn12 = (((((locals.var_t1__blk809_dn12 * locals.var_vadits) + (locals.var_t1__blk809 * locals.var_vadits_dn12)) * locals.var_tmp3__blk836) - (assign20140_e18002 * locals.var_tmp3__blk836_dn12)) / (locals.var_tmp3__blk836 * locals.var_tmp3__blk836));
        locals.var_t2__blk810_rv = 0.0;

        let assign20150_e18008: f64 = (locals.var_t0__blk808 * locals.var_t2__blk810);
        let assign20150_e18009: f64 = (locals.var_vasat + assign20150_e18008);
        locals.var_va = assign20150_e18009;
        locals.var_va_dn3 = (locals.var_vasat_dn3 + ((locals.var_t0__blk808_dn3 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn3)));
        locals.var_va_dn4 = (locals.var_vasat_dn4 + ((locals.var_t0__blk808_dn4 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn4)));
        locals.var_va_dn5 = (locals.var_vasat_dn5 + ((locals.var_t0__blk808_dn5 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn5)));
        locals.var_va_dn6 = (locals.var_vasat_dn6 + ((locals.var_t0__blk808_dn6 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn6)));
        locals.var_va_dn7 = (locals.var_vasat_dn7 + ((locals.var_t0__blk808_dn7 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn7)));
        locals.var_va_dn8 = (locals.var_vasat_dn8 + ((locals.var_t0__blk808_dn8 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn8)));
        locals.var_va_dn9 = (locals.var_vasat_dn9 + ((locals.var_t0__blk808_dn9 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn9)));
        locals.var_va_dn10 = (locals.var_vasat_dn10 + ((locals.var_t0__blk808_dn10 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn10)));
        locals.var_va_dn11 = (locals.var_vasat_dn11 + ((locals.var_t0__blk808_dn11 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn11)));
        locals.var_va_dn12 = (locals.var_vasat_dn12 + ((locals.var_t0__blk808_dn12 * locals.var_t2__blk810) + (locals.var_t0__blk808 * locals.var_t2__blk810_dn12)));
        locals.var_va_rv = 0.0;

        let assign20160_e18012: f64 = (locals.var_b4soicox * locals.var_weff);
        let assign20160_e18014: f64 = (assign20160_e18012 / locals.var_leff);
        locals.var_coxwovl = assign20160_e18014;
        locals.var_coxwovl_dn3 = ((((locals.var_b4soicox * locals.var_weff_dn3) * locals.var_leff) - (assign20160_e18012 * locals.var_leff_dn3)) / (locals.var_leff * locals.var_leff));
        locals.var_coxwovl_dn4 = ((((locals.var_b4soicox * locals.var_weff_dn4) * locals.var_leff) - (assign20160_e18012 * locals.var_leff_dn4)) / (locals.var_leff * locals.var_leff));
        locals.var_coxwovl_dn5 = ((((locals.var_b4soicox * locals.var_weff_dn5) * locals.var_leff) - (assign20160_e18012 * locals.var_leff_dn5)) / (locals.var_leff * locals.var_leff));
        locals.var_coxwovl_dn6 = ((((locals.var_b4soicox * locals.var_weff_dn6) * locals.var_leff) - (assign20160_e18012 * locals.var_leff_dn6)) / (locals.var_leff * locals.var_leff));
        locals.var_coxwovl_dn7 = ((((locals.var_b4soicox * locals.var_weff_dn7) * locals.var_leff) - (assign20160_e18012 * locals.var_leff_dn7)) / (locals.var_leff * locals.var_leff));
        locals.var_coxwovl_dn8 = ((((locals.var_b4soicox * locals.var_weff_dn8) * locals.var_leff) - (assign20160_e18012 * locals.var_leff_dn8)) / (locals.var_leff * locals.var_leff));
        locals.var_coxwovl_dn9 = ((((locals.var_b4soicox * locals.var_weff_dn9) * locals.var_leff) - (assign20160_e18012 * locals.var_leff_dn9)) / (locals.var_leff * locals.var_leff));
        locals.var_coxwovl_dn10 = ((((locals.var_b4soicox * locals.var_weff_dn10) * locals.var_leff) - (assign20160_e18012 * locals.var_leff_dn10)) / (locals.var_leff * locals.var_leff));
        locals.var_coxwovl_dn11 = ((((locals.var_b4soicox * locals.var_weff_dn11) * locals.var_leff) - (assign20160_e18012 * locals.var_leff_dn11)) / (locals.var_leff * locals.var_leff));
        locals.var_coxwovl_dn12 = ((((locals.var_b4soicox * locals.var_weff_dn12) * locals.var_leff) - (assign20160_e18012 * locals.var_leff_dn12)) / (locals.var_leff * locals.var_leff));
        locals.var_coxwovl_rv = 0.0;

        let assign20170_e18017: f64 = (locals.var_ueff * locals.var_coxwovl);
        locals.var_beta = assign20170_e18017;
        locals.var_beta_dn3 = ((locals.var_ueff_dn3 * locals.var_coxwovl) + (locals.var_ueff * locals.var_coxwovl_dn3));
        locals.var_beta_dn4 = ((locals.var_ueff_dn4 * locals.var_coxwovl) + (locals.var_ueff * locals.var_coxwovl_dn4));
        locals.var_beta_dn5 = ((locals.var_ueff_dn5 * locals.var_coxwovl) + (locals.var_ueff * locals.var_coxwovl_dn5));
        locals.var_beta_dn6 = ((locals.var_ueff_dn6 * locals.var_coxwovl) + (locals.var_ueff * locals.var_coxwovl_dn6));
        locals.var_beta_dn7 = ((locals.var_ueff_dn7 * locals.var_coxwovl) + (locals.var_ueff * locals.var_coxwovl_dn7));
        locals.var_beta_dn8 = ((locals.var_ueff_dn8 * locals.var_coxwovl) + (locals.var_ueff * locals.var_coxwovl_dn8));
        locals.var_beta_dn9 = ((locals.var_ueff_dn9 * locals.var_coxwovl) + (locals.var_ueff * locals.var_coxwovl_dn9));
        locals.var_beta_dn10 = ((locals.var_ueff_dn10 * locals.var_coxwovl) + (locals.var_ueff * locals.var_coxwovl_dn10));
        locals.var_beta_dn11 = ((locals.var_ueff_dn11 * locals.var_coxwovl) + (locals.var_ueff * locals.var_coxwovl_dn11));
        locals.var_beta_dn12 = ((locals.var_ueff_dn12 * locals.var_coxwovl) + (locals.var_ueff * locals.var_coxwovl_dn12));
        locals.var_beta_rv = 0.0;

        let assign20180_e18021: f64 = (0.5 * locals.var_abulk);
        let assign20180_e18023: f64 = (assign20180_e18021 * locals.var_vdseff);
        let assign20180_e18025: f64 = (assign20180_e18023 / locals.var_vgst2vtm);
        let assign20180_e18026: f64 = (1.0 - assign20180_e18025);
        locals.var_t0__blk808 = assign20180_e18026;
        locals.var_t0__blk808_dn3 = (-((((((0.5 * locals.var_abulk_dn3) * locals.var_vdseff) + (assign20180_e18021 * locals.var_vdseff_dn3)) * locals.var_vgst2vtm) - (assign20180_e18023 * locals.var_vgst2vtm_dn3)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)));
        locals.var_t0__blk808_dn4 = (-((((((0.5 * locals.var_abulk_dn4) * locals.var_vdseff) + (assign20180_e18021 * locals.var_vdseff_dn4)) * locals.var_vgst2vtm) - (assign20180_e18023 * locals.var_vgst2vtm_dn4)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)));
        locals.var_t0__blk808_dn5 = (-((((((0.5 * locals.var_abulk_dn5) * locals.var_vdseff) + (assign20180_e18021 * locals.var_vdseff_dn5)) * locals.var_vgst2vtm) - (assign20180_e18023 * locals.var_vgst2vtm_dn5)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)));
        locals.var_t0__blk808_dn6 = (-((((((0.5 * locals.var_abulk_dn6) * locals.var_vdseff) + (assign20180_e18021 * locals.var_vdseff_dn6)) * locals.var_vgst2vtm) - (assign20180_e18023 * locals.var_vgst2vtm_dn6)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)));
        locals.var_t0__blk808_dn7 = (-((((((0.5 * locals.var_abulk_dn7) * locals.var_vdseff) + (assign20180_e18021 * locals.var_vdseff_dn7)) * locals.var_vgst2vtm) - (assign20180_e18023 * locals.var_vgst2vtm_dn7)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)));
        locals.var_t0__blk808_dn8 = (-((((((0.5 * locals.var_abulk_dn8) * locals.var_vdseff) + (assign20180_e18021 * locals.var_vdseff_dn8)) * locals.var_vgst2vtm) - (assign20180_e18023 * locals.var_vgst2vtm_dn8)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)));
        locals.var_t0__blk808_dn9 = (-((((((0.5 * locals.var_abulk_dn9) * locals.var_vdseff) + (assign20180_e18021 * locals.var_vdseff_dn9)) * locals.var_vgst2vtm) - (assign20180_e18023 * locals.var_vgst2vtm_dn9)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)));
        locals.var_t0__blk808_dn10 = (-((((((0.5 * locals.var_abulk_dn10) * locals.var_vdseff) + (assign20180_e18021 * locals.var_vdseff_dn10)) * locals.var_vgst2vtm) - (assign20180_e18023 * locals.var_vgst2vtm_dn10)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)));
        locals.var_t0__blk808_dn11 = (-((((((0.5 * locals.var_abulk_dn11) * locals.var_vdseff) + (assign20180_e18021 * locals.var_vdseff_dn11)) * locals.var_vgst2vtm) - (assign20180_e18023 * locals.var_vgst2vtm_dn11)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)));
        locals.var_t0__blk808_dn12 = (-((((((0.5 * locals.var_abulk_dn12) * locals.var_vdseff) + (assign20180_e18021 * locals.var_vdseff_dn12)) * locals.var_vgst2vtm) - (assign20180_e18023 * locals.var_vgst2vtm_dn12)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)));
        locals.var_t0__blk808_rv = 0.0;

        let assign20190_e18029: f64 = (locals.var_vgsteff__blk840 * locals.var_t0__blk808);
        locals.var_fgche1 = assign20190_e18029;
        locals.var_fgche1_dn3 = ((locals.var_vgsteff__blk840_dn3 * locals.var_t0__blk808) + (locals.var_vgsteff__blk840 * locals.var_t0__blk808_dn3));
        locals.var_fgche1_dn4 = ((locals.var_vgsteff__blk840_dn4 * locals.var_t0__blk808) + (locals.var_vgsteff__blk840 * locals.var_t0__blk808_dn4));
        locals.var_fgche1_dn5 = ((locals.var_vgsteff__blk840_dn5 * locals.var_t0__blk808) + (locals.var_vgsteff__blk840 * locals.var_t0__blk808_dn5));
        locals.var_fgche1_dn6 = ((locals.var_vgsteff__blk840_dn6 * locals.var_t0__blk808) + (locals.var_vgsteff__blk840 * locals.var_t0__blk808_dn6));
        locals.var_fgche1_dn7 = ((locals.var_vgsteff__blk840_dn7 * locals.var_t0__blk808) + (locals.var_vgsteff__blk840 * locals.var_t0__blk808_dn7));
        locals.var_fgche1_dn8 = ((locals.var_vgsteff__blk840_dn8 * locals.var_t0__blk808) + (locals.var_vgsteff__blk840 * locals.var_t0__blk808_dn8));
        locals.var_fgche1_dn9 = ((locals.var_vgsteff__blk840_dn9 * locals.var_t0__blk808) + (locals.var_vgsteff__blk840 * locals.var_t0__blk808_dn9));
        locals.var_fgche1_dn10 = ((locals.var_vgsteff__blk840_dn10 * locals.var_t0__blk808) + (locals.var_vgsteff__blk840 * locals.var_t0__blk808_dn10));
        locals.var_fgche1_dn11 = ((locals.var_vgsteff__blk840_dn11 * locals.var_t0__blk808) + (locals.var_vgsteff__blk840 * locals.var_t0__blk808_dn11));
        locals.var_fgche1_dn12 = ((locals.var_vgsteff__blk840_dn12 * locals.var_t0__blk808) + (locals.var_vgsteff__blk840 * locals.var_t0__blk808_dn12));
        locals.var_fgche1_rv = 0.0;

        let assign20200_e18032: f64 = (locals.var_vdseff / locals.var_esatl);
        locals.var_t9 = assign20200_e18032;
        locals.var_t9_dn3 = (((locals.var_vdseff_dn3 * locals.var_esatl) - (locals.var_vdseff * locals.var_esatl_dn3)) / (locals.var_esatl * locals.var_esatl));
        locals.var_t9_dn4 = (((locals.var_vdseff_dn4 * locals.var_esatl) - (locals.var_vdseff * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl));
        locals.var_t9_dn5 = (((locals.var_vdseff_dn5 * locals.var_esatl) - (locals.var_vdseff * locals.var_esatl_dn5)) / (locals.var_esatl * locals.var_esatl));
        locals.var_t9_dn6 = (((locals.var_vdseff_dn6 * locals.var_esatl) - (locals.var_vdseff * locals.var_esatl_dn6)) / (locals.var_esatl * locals.var_esatl));
        locals.var_t9_dn7 = (((locals.var_vdseff_dn7 * locals.var_esatl) - (locals.var_vdseff * locals.var_esatl_dn7)) / (locals.var_esatl * locals.var_esatl));
        locals.var_t9_dn8 = (((locals.var_vdseff_dn8 * locals.var_esatl) - (locals.var_vdseff * locals.var_esatl_dn8)) / (locals.var_esatl * locals.var_esatl));
        locals.var_t9_dn9 = (((locals.var_vdseff_dn9 * locals.var_esatl) - (locals.var_vdseff * locals.var_esatl_dn9)) / (locals.var_esatl * locals.var_esatl));
        locals.var_t9_dn10 = (((locals.var_vdseff_dn10 * locals.var_esatl) - (locals.var_vdseff * locals.var_esatl_dn10)) / (locals.var_esatl * locals.var_esatl));
        locals.var_t9_dn11 = (((locals.var_vdseff_dn11 * locals.var_esatl) - (locals.var_vdseff * locals.var_esatl_dn11)) / (locals.var_esatl * locals.var_esatl));
        locals.var_t9_dn12 = (((locals.var_vdseff_dn12 * locals.var_esatl) - (locals.var_vdseff * locals.var_esatl_dn12)) / (locals.var_esatl * locals.var_esatl));
        locals.var_t9_rv = 0.0;

        let assign20210_e18035: f64 = (1.0 + locals.var_t9);
        locals.var_fgche2 = assign20210_e18035;
        locals.var_fgche2_dn3 = locals.var_t9_dn3;
        locals.var_fgche2_dn4 = locals.var_t9_dn4;
        locals.var_fgche2_dn5 = locals.var_t9_dn5;
        locals.var_fgche2_dn6 = locals.var_t9_dn6;
        locals.var_fgche2_dn7 = locals.var_t9_dn7;
        locals.var_fgche2_dn8 = locals.var_t9_dn8;
        locals.var_fgche2_dn9 = locals.var_t9_dn9;
        locals.var_fgche2_dn10 = locals.var_t9_dn10;
        locals.var_fgche2_dn11 = locals.var_t9_dn11;
        locals.var_fgche2_dn12 = locals.var_t9_dn12;
        locals.var_fgche2_rv = 0.0;

        let assign20220_e18038: f64 = (locals.var_beta * locals.var_fgche1);
        let assign20220_e18040: f64 = (assign20220_e18038 / locals.var_fgche2);
        locals.var_gche = assign20220_e18040;
        locals.var_gche_dn3 = (((((locals.var_beta_dn3 * locals.var_fgche1) + (locals.var_beta * locals.var_fgche1_dn3)) * locals.var_fgche2) - (assign20220_e18038 * locals.var_fgche2_dn3)) / (locals.var_fgche2 * locals.var_fgche2));
        locals.var_gche_dn4 = (((((locals.var_beta_dn4 * locals.var_fgche1) + (locals.var_beta * locals.var_fgche1_dn4)) * locals.var_fgche2) - (assign20220_e18038 * locals.var_fgche2_dn4)) / (locals.var_fgche2 * locals.var_fgche2));
        locals.var_gche_dn5 = (((((locals.var_beta_dn5 * locals.var_fgche1) + (locals.var_beta * locals.var_fgche1_dn5)) * locals.var_fgche2) - (assign20220_e18038 * locals.var_fgche2_dn5)) / (locals.var_fgche2 * locals.var_fgche2));
        locals.var_gche_dn6 = (((((locals.var_beta_dn6 * locals.var_fgche1) + (locals.var_beta * locals.var_fgche1_dn6)) * locals.var_fgche2) - (assign20220_e18038 * locals.var_fgche2_dn6)) / (locals.var_fgche2 * locals.var_fgche2));
        locals.var_gche_dn7 = (((((locals.var_beta_dn7 * locals.var_fgche1) + (locals.var_beta * locals.var_fgche1_dn7)) * locals.var_fgche2) - (assign20220_e18038 * locals.var_fgche2_dn7)) / (locals.var_fgche2 * locals.var_fgche2));
        locals.var_gche_dn8 = (((((locals.var_beta_dn8 * locals.var_fgche1) + (locals.var_beta * locals.var_fgche1_dn8)) * locals.var_fgche2) - (assign20220_e18038 * locals.var_fgche2_dn8)) / (locals.var_fgche2 * locals.var_fgche2));
        locals.var_gche_dn9 = (((((locals.var_beta_dn9 * locals.var_fgche1) + (locals.var_beta * locals.var_fgche1_dn9)) * locals.var_fgche2) - (assign20220_e18038 * locals.var_fgche2_dn9)) / (locals.var_fgche2 * locals.var_fgche2));
        locals.var_gche_dn10 = (((((locals.var_beta_dn10 * locals.var_fgche1) + (locals.var_beta * locals.var_fgche1_dn10)) * locals.var_fgche2) - (assign20220_e18038 * locals.var_fgche2_dn10)) / (locals.var_fgche2 * locals.var_fgche2));
        locals.var_gche_dn11 = (((((locals.var_beta_dn11 * locals.var_fgche1) + (locals.var_beta * locals.var_fgche1_dn11)) * locals.var_fgche2) - (assign20220_e18038 * locals.var_fgche2_dn11)) / (locals.var_fgche2 * locals.var_fgche2));
        locals.var_gche_dn12 = (((((locals.var_beta_dn12 * locals.var_fgche1) + (locals.var_beta * locals.var_fgche1_dn12)) * locals.var_fgche2) - (assign20220_e18038 * locals.var_fgche2_dn12)) / (locals.var_fgche2 * locals.var_fgche2));
        locals.var_gche_rv = 0.0;

        let assign20230_e18044: f64 = (locals.var_gche * locals.var_rds);
        let assign20230_e18045: f64 = (1.0 + assign20230_e18044);
        locals.var_t0__blk808 = assign20230_e18045;
        locals.var_t0__blk808_dn3 = ((locals.var_gche_dn3 * locals.var_rds) + (locals.var_gche * locals.var_rds_dn3));
        locals.var_t0__blk808_dn4 = ((locals.var_gche_dn4 * locals.var_rds) + (locals.var_gche * locals.var_rds_dn4));
        locals.var_t0__blk808_dn5 = ((locals.var_gche_dn5 * locals.var_rds) + (locals.var_gche * locals.var_rds_dn5));
        locals.var_t0__blk808_dn6 = ((locals.var_gche_dn6 * locals.var_rds) + (locals.var_gche * locals.var_rds_dn6));
        locals.var_t0__blk808_dn7 = ((locals.var_gche_dn7 * locals.var_rds) + (locals.var_gche * locals.var_rds_dn7));
        locals.var_t0__blk808_dn8 = ((locals.var_gche_dn8 * locals.var_rds) + (locals.var_gche * locals.var_rds_dn8));
        locals.var_t0__blk808_dn9 = ((locals.var_gche_dn9 * locals.var_rds) + (locals.var_gche * locals.var_rds_dn9));
        locals.var_t0__blk808_dn10 = ((locals.var_gche_dn10 * locals.var_rds) + (locals.var_gche * locals.var_rds_dn10));
        locals.var_t0__blk808_dn11 = ((locals.var_gche_dn11 * locals.var_rds) + (locals.var_gche * locals.var_rds_dn11));
        locals.var_t0__blk808_dn12 = ((locals.var_gche_dn12 * locals.var_rds) + (locals.var_gche * locals.var_rds_dn12));
        locals.var_t0__blk808_rv = 0.0;

        let assign20240_e18048: f64 = (locals.var_vdseff / locals.var_t0__blk808);
        locals.var_t9 = assign20240_e18048;
        locals.var_t9_dn3 = (((locals.var_vdseff_dn3 * locals.var_t0__blk808) - (locals.var_vdseff * locals.var_t0__blk808_dn3)) / (locals.var_t0__blk808 * locals.var_t0__blk808));
        locals.var_t9_dn4 = (((locals.var_vdseff_dn4 * locals.var_t0__blk808) - (locals.var_vdseff * locals.var_t0__blk808_dn4)) / (locals.var_t0__blk808 * locals.var_t0__blk808));
        locals.var_t9_dn5 = (((locals.var_vdseff_dn5 * locals.var_t0__blk808) - (locals.var_vdseff * locals.var_t0__blk808_dn5)) / (locals.var_t0__blk808 * locals.var_t0__blk808));
        locals.var_t9_dn6 = (((locals.var_vdseff_dn6 * locals.var_t0__blk808) - (locals.var_vdseff * locals.var_t0__blk808_dn6)) / (locals.var_t0__blk808 * locals.var_t0__blk808));
        locals.var_t9_dn7 = (((locals.var_vdseff_dn7 * locals.var_t0__blk808) - (locals.var_vdseff * locals.var_t0__blk808_dn7)) / (locals.var_t0__blk808 * locals.var_t0__blk808));
        locals.var_t9_dn8 = (((locals.var_vdseff_dn8 * locals.var_t0__blk808) - (locals.var_vdseff * locals.var_t0__blk808_dn8)) / (locals.var_t0__blk808 * locals.var_t0__blk808));
        locals.var_t9_dn9 = (((locals.var_vdseff_dn9 * locals.var_t0__blk808) - (locals.var_vdseff * locals.var_t0__blk808_dn9)) / (locals.var_t0__blk808 * locals.var_t0__blk808));
        locals.var_t9_dn10 = (((locals.var_vdseff_dn10 * locals.var_t0__blk808) - (locals.var_vdseff * locals.var_t0__blk808_dn10)) / (locals.var_t0__blk808 * locals.var_t0__blk808));
        locals.var_t9_dn11 = (((locals.var_vdseff_dn11 * locals.var_t0__blk808) - (locals.var_vdseff * locals.var_t0__blk808_dn11)) / (locals.var_t0__blk808 * locals.var_t0__blk808));
        locals.var_t9_dn12 = (((locals.var_vdseff_dn12 * locals.var_t0__blk808) - (locals.var_vdseff * locals.var_t0__blk808_dn12)) / (locals.var_t0__blk808 * locals.var_t0__blk808));
        locals.var_t9_rv = 0.0;

        let assign20250_e18051: f64 = (locals.var_gche * locals.var_t9);
        locals.var_idl = assign20250_e18051;
        locals.var_idl_dn3 = ((locals.var_gche_dn3 * locals.var_t9) + (locals.var_gche * locals.var_t9_dn3));
        locals.var_idl_dn4 = ((locals.var_gche_dn4 * locals.var_t9) + (locals.var_gche * locals.var_t9_dn4));
        locals.var_idl_dn5 = ((locals.var_gche_dn5 * locals.var_t9) + (locals.var_gche * locals.var_t9_dn5));
        locals.var_idl_dn6 = ((locals.var_gche_dn6 * locals.var_t9) + (locals.var_gche * locals.var_t9_dn6));
        locals.var_idl_dn7 = ((locals.var_gche_dn7 * locals.var_t9) + (locals.var_gche * locals.var_t9_dn7));
        locals.var_idl_dn8 = ((locals.var_gche_dn8 * locals.var_t9) + (locals.var_gche * locals.var_t9_dn8));
        locals.var_idl_dn9 = ((locals.var_gche_dn9 * locals.var_t9) + (locals.var_gche * locals.var_t9_dn9));
        locals.var_idl_dn10 = ((locals.var_gche_dn10 * locals.var_t9) + (locals.var_gche * locals.var_t9_dn10));
        locals.var_idl_dn11 = ((locals.var_gche_dn11 * locals.var_t9) + (locals.var_gche * locals.var_t9_dn11));
        locals.var_idl_dn12 = ((locals.var_gche_dn12 * locals.var_t9) + (locals.var_gche * locals.var_t9_dn12));
        locals.var_idl_rv = 0.0;

        let assign20260_e18054: f64 = (locals.var_gche / locals.var_t0__blk808);
        locals.var_idlovvdseff = assign20260_e18054;
        locals.var_idlovvdseff_dn3 = (((locals.var_gche_dn3 * locals.var_t0__blk808) - (locals.var_gche * locals.var_t0__blk808_dn3)) / (locals.var_t0__blk808 * locals.var_t0__blk808));
        locals.var_idlovvdseff_dn4 = (((locals.var_gche_dn4 * locals.var_t0__blk808) - (locals.var_gche * locals.var_t0__blk808_dn4)) / (locals.var_t0__blk808 * locals.var_t0__blk808));
        locals.var_idlovvdseff_dn5 = (((locals.var_gche_dn5 * locals.var_t0__blk808) - (locals.var_gche * locals.var_t0__blk808_dn5)) / (locals.var_t0__blk808 * locals.var_t0__blk808));
        locals.var_idlovvdseff_dn6 = (((locals.var_gche_dn6 * locals.var_t0__blk808) - (locals.var_gche * locals.var_t0__blk808_dn6)) / (locals.var_t0__blk808 * locals.var_t0__blk808));
        locals.var_idlovvdseff_dn7 = (((locals.var_gche_dn7 * locals.var_t0__blk808) - (locals.var_gche * locals.var_t0__blk808_dn7)) / (locals.var_t0__blk808 * locals.var_t0__blk808));
        locals.var_idlovvdseff_dn8 = (((locals.var_gche_dn8 * locals.var_t0__blk808) - (locals.var_gche * locals.var_t0__blk808_dn8)) / (locals.var_t0__blk808 * locals.var_t0__blk808));
        locals.var_idlovvdseff_dn9 = (((locals.var_gche_dn9 * locals.var_t0__blk808) - (locals.var_gche * locals.var_t0__blk808_dn9)) / (locals.var_t0__blk808 * locals.var_t0__blk808));
        locals.var_idlovvdseff_dn10 = (((locals.var_gche_dn10 * locals.var_t0__blk808) - (locals.var_gche * locals.var_t0__blk808_dn10)) / (locals.var_t0__blk808 * locals.var_t0__blk808));
        locals.var_idlovvdseff_dn11 = (((locals.var_gche_dn11 * locals.var_t0__blk808) - (locals.var_gche * locals.var_t0__blk808_dn11)) / (locals.var_t0__blk808 * locals.var_t0__blk808));
        locals.var_idlovvdseff_dn12 = (((locals.var_gche_dn12 * locals.var_t0__blk808) - (locals.var_gche * locals.var_t0__blk808_dn12)) / (locals.var_t0__blk808 * locals.var_t0__blk808));
        locals.var_idlovvdseff_rv = 0.0;

        let assign20270_e18057: f64 = (locals.var_diffvds / locals.var_va);
        locals.var_t9 = assign20270_e18057;
        locals.var_t9_dn3 = (((locals.var_diffvds_dn3 * locals.var_va) - (locals.var_diffvds * locals.var_va_dn3)) / (locals.var_va * locals.var_va));
        locals.var_t9_dn4 = (((locals.var_diffvds_dn4 * locals.var_va) - (locals.var_diffvds * locals.var_va_dn4)) / (locals.var_va * locals.var_va));
        locals.var_t9_dn5 = (((locals.var_diffvds_dn5 * locals.var_va) - (locals.var_diffvds * locals.var_va_dn5)) / (locals.var_va * locals.var_va));
        locals.var_t9_dn6 = (((locals.var_diffvds_dn6 * locals.var_va) - (locals.var_diffvds * locals.var_va_dn6)) / (locals.var_va * locals.var_va));
        locals.var_t9_dn7 = (((locals.var_diffvds_dn7 * locals.var_va) - (locals.var_diffvds * locals.var_va_dn7)) / (locals.var_va * locals.var_va));
        locals.var_t9_dn8 = (((locals.var_diffvds_dn8 * locals.var_va) - (locals.var_diffvds * locals.var_va_dn8)) / (locals.var_va * locals.var_va));
        locals.var_t9_dn9 = (((locals.var_diffvds_dn9 * locals.var_va) - (locals.var_diffvds * locals.var_va_dn9)) / (locals.var_va * locals.var_va));
        locals.var_t9_dn10 = (((locals.var_diffvds_dn10 * locals.var_va) - (locals.var_diffvds * locals.var_va_dn10)) / (locals.var_va * locals.var_va));
        locals.var_t9_dn11 = (((locals.var_diffvds_dn11 * locals.var_va) - (locals.var_diffvds * locals.var_va_dn11)) / (locals.var_va * locals.var_va));
        locals.var_t9_dn12 = (((locals.var_diffvds_dn12 * locals.var_va) - (locals.var_diffvds * locals.var_va_dn12)) / (locals.var_va * locals.var_va));
        locals.var_t9_rv = 0.0;

        let assign20280_e18060: f64 = (1.0 + locals.var_t9);
        locals.var_t0__blk808 = assign20280_e18060;
        locals.var_t0__blk808_dn3 = locals.var_t9_dn3;
        locals.var_t0__blk808_dn4 = locals.var_t9_dn4;
        locals.var_t0__blk808_dn5 = locals.var_t9_dn5;
        locals.var_t0__blk808_dn6 = locals.var_t9_dn6;
        locals.var_t0__blk808_dn7 = locals.var_t9_dn7;
        locals.var_t0__blk808_dn8 = locals.var_t9_dn8;
        locals.var_t0__blk808_dn9 = locals.var_t9_dn9;
        locals.var_t0__blk808_dn10 = locals.var_t9_dn10;
        locals.var_t0__blk808_dn11 = locals.var_t9_dn11;
        locals.var_t0__blk808_dn12 = locals.var_t9_dn12;
        locals.var_t0__blk808_rv = 0.0;

        let assign20290_e18063: f64 = (locals.var_idl * locals.var_t0__blk808);
        let assign20290_e18065: f64 = (assign20290_e18063 / p.p23);
        locals.var_ids_1 = assign20290_e18065;
        locals.var_ids_1_dn3 = (((locals.var_idl_dn3 * locals.var_t0__blk808) + (locals.var_idl * locals.var_t0__blk808_dn3)) / p.p23);
        locals.var_ids_1_dn4 = (((locals.var_idl_dn4 * locals.var_t0__blk808) + (locals.var_idl * locals.var_t0__blk808_dn4)) / p.p23);
        locals.var_ids_1_dn5 = (((locals.var_idl_dn5 * locals.var_t0__blk808) + (locals.var_idl * locals.var_t0__blk808_dn5)) / p.p23);
        locals.var_ids_1_dn6 = (((locals.var_idl_dn6 * locals.var_t0__blk808) + (locals.var_idl * locals.var_t0__blk808_dn6)) / p.p23);
        locals.var_ids_1_dn7 = (((locals.var_idl_dn7 * locals.var_t0__blk808) + (locals.var_idl * locals.var_t0__blk808_dn7)) / p.p23);
        locals.var_ids_1_dn8 = (((locals.var_idl_dn8 * locals.var_t0__blk808) + (locals.var_idl * locals.var_t0__blk808_dn8)) / p.p23);
        locals.var_ids_1_dn9 = (((locals.var_idl_dn9 * locals.var_t0__blk808) + (locals.var_idl * locals.var_t0__blk808_dn9)) / p.p23);
        locals.var_ids_1_dn10 = (((locals.var_idl_dn10 * locals.var_t0__blk808) + (locals.var_idl * locals.var_t0__blk808_dn10)) / p.p23);
        locals.var_ids_1_dn11 = (((locals.var_idl_dn11 * locals.var_t0__blk808) + (locals.var_idl * locals.var_t0__blk808_dn11)) / p.p23);
        locals.var_ids_1_dn12 = (((locals.var_idl_dn12 * locals.var_t0__blk808) + (locals.var_idl * locals.var_t0__blk808_dn12)) / p.p23);
        locals.var_ids_1_rv = 0.0;

        let assign20300_e18068: f64 = (locals.var_ids_1 * p.p30);
        locals.var_ids_1 = assign20300_e18068;
        locals.var_ids_1_dn3 = (locals.var_ids_1_dn3 * p.p30);
        locals.var_ids_1_dn4 = (locals.var_ids_1_dn4 * p.p30);
        locals.var_ids_1_dn5 = (locals.var_ids_1_dn5 * p.p30);
        locals.var_ids_1_dn6 = (locals.var_ids_1_dn6 * p.p30);
        locals.var_ids_1_dn7 = (locals.var_ids_1_dn7 * p.p30);
        locals.var_ids_1_dn8 = (locals.var_ids_1_dn8 * p.p30);
        locals.var_ids_1_dn9 = (locals.var_ids_1_dn9 * p.p30);
        locals.var_ids_1_dn10 = (locals.var_ids_1_dn10 * p.p30);
        locals.var_ids_1_dn11 = (locals.var_ids_1_dn11 * p.p30);
        locals.var_ids_1_dn12 = (locals.var_ids_1_dn12 * p.p30);
        locals.var_ids_1_rv = 0.0;

        let assign20310_e18071: f64 = (locals.var_idlovvdseff * locals.var_t0__blk808);
        let assign20310_e18073: f64 = (assign20310_e18071 / p.p23);
        locals.var_b4soiidovvds = assign20310_e18073;
        locals.var_b4soiidovvds_dn3 = (((locals.var_idlovvdseff_dn3 * locals.var_t0__blk808) + (locals.var_idlovvdseff * locals.var_t0__blk808_dn3)) / p.p23);
        locals.var_b4soiidovvds_dn4 = (((locals.var_idlovvdseff_dn4 * locals.var_t0__blk808) + (locals.var_idlovvdseff * locals.var_t0__blk808_dn4)) / p.p23);
        locals.var_b4soiidovvds_dn5 = (((locals.var_idlovvdseff_dn5 * locals.var_t0__blk808) + (locals.var_idlovvdseff * locals.var_t0__blk808_dn5)) / p.p23);
        locals.var_b4soiidovvds_dn6 = (((locals.var_idlovvdseff_dn6 * locals.var_t0__blk808) + (locals.var_idlovvdseff * locals.var_t0__blk808_dn6)) / p.p23);
        locals.var_b4soiidovvds_dn7 = (((locals.var_idlovvdseff_dn7 * locals.var_t0__blk808) + (locals.var_idlovvdseff * locals.var_t0__blk808_dn7)) / p.p23);
        locals.var_b4soiidovvds_dn8 = (((locals.var_idlovvdseff_dn8 * locals.var_t0__blk808) + (locals.var_idlovvdseff * locals.var_t0__blk808_dn8)) / p.p23);
        locals.var_b4soiidovvds_dn9 = (((locals.var_idlovvdseff_dn9 * locals.var_t0__blk808) + (locals.var_idlovvdseff * locals.var_t0__blk808_dn9)) / p.p23);
        locals.var_b4soiidovvds_dn10 = (((locals.var_idlovvdseff_dn10 * locals.var_t0__blk808) + (locals.var_idlovvdseff * locals.var_t0__blk808_dn10)) / p.p23);
        locals.var_b4soiidovvds_dn11 = (((locals.var_idlovvdseff_dn11 * locals.var_t0__blk808) + (locals.var_idlovvdseff * locals.var_t0__blk808_dn11)) / p.p23);
        locals.var_b4soiidovvds_dn12 = (((locals.var_idlovvdseff_dn12 * locals.var_t0__blk808) + (locals.var_idlovvdseff * locals.var_t0__blk808_dn12)) / p.p23);
        locals.var_b4soiidovvds_rv = 0.0;

        let assign20320_e18076: f64 = if locals.var_b4soiidovvds < 1e-9 { 1.0 } else { 0.0 };
        locals.var_guard1239 = assign20320_e18076;
        locals.var_guard1239_rv = 0.0;

        let (assign20330_e18080, assign20330_e18080_d_n3, assign20330_e18080_d_n4, assign20330_e18080_d_n5, assign20330_e18080_d_n6, assign20330_e18080_d_n7, assign20330_e18080_d_n8, assign20330_e18080_d_n9, assign20330_e18080_d_n10, assign20330_e18080_d_n11, assign20330_e18080_d_n12,) = {
    if (locals.var_guard1239 != 0.0) {
        (1e-9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soiidovvds, locals.var_b4soiidovvds_dn3, locals.var_b4soiidovvds_dn4, locals.var_b4soiidovvds_dn5, locals.var_b4soiidovvds_dn6, locals.var_b4soiidovvds_dn7, locals.var_b4soiidovvds_dn8, locals.var_b4soiidovvds_dn9, locals.var_b4soiidovvds_dn10, locals.var_b4soiidovvds_dn11, locals.var_b4soiidovvds_dn12,)
    }
};
        locals.var_b4soiidovvds = assign20330_e18080;
        locals.var_b4soiidovvds_dn3 = assign20330_e18080_d_n3;
        locals.var_b4soiidovvds_dn4 = assign20330_e18080_d_n4;
        locals.var_b4soiidovvds_dn5 = assign20330_e18080_d_n5;
        locals.var_b4soiidovvds_dn6 = assign20330_e18080_d_n6;
        locals.var_b4soiidovvds_dn7 = assign20330_e18080_d_n7;
        locals.var_b4soiidovvds_dn8 = assign20330_e18080_d_n8;
        locals.var_b4soiidovvds_dn9 = assign20330_e18080_d_n9;
        locals.var_b4soiidovvds_dn10 = assign20330_e18080_d_n10;
        locals.var_b4soiidovvds_dn11 = assign20330_e18080_d_n11;
        locals.var_b4soiidovvds_dn12 = assign20330_e18080_d_n12;
        locals.var_b4soiidovvds_rv = 0.0;

        let assign20340_e18083: f64 = (locals.var_idlovvdseff * locals.var_t0__blk808);
        let assign20340_e18085: f64 = (assign20340_e18083 / p.p23);
        locals.var_idovvds = assign20340_e18085;
        locals.var_idovvds_dn3 = (((locals.var_idlovvdseff_dn3 * locals.var_t0__blk808) + (locals.var_idlovvdseff * locals.var_t0__blk808_dn3)) / p.p23);
        locals.var_idovvds_dn4 = (((locals.var_idlovvdseff_dn4 * locals.var_t0__blk808) + (locals.var_idlovvdseff * locals.var_t0__blk808_dn4)) / p.p23);
        locals.var_idovvds_dn5 = (((locals.var_idlovvdseff_dn5 * locals.var_t0__blk808) + (locals.var_idlovvdseff * locals.var_t0__blk808_dn5)) / p.p23);
        locals.var_idovvds_dn6 = (((locals.var_idlovvdseff_dn6 * locals.var_t0__blk808) + (locals.var_idlovvdseff * locals.var_t0__blk808_dn6)) / p.p23);
        locals.var_idovvds_dn7 = (((locals.var_idlovvdseff_dn7 * locals.var_t0__blk808) + (locals.var_idlovvdseff * locals.var_t0__blk808_dn7)) / p.p23);
        locals.var_idovvds_dn8 = (((locals.var_idlovvdseff_dn8 * locals.var_t0__blk808) + (locals.var_idlovvdseff * locals.var_t0__blk808_dn8)) / p.p23);
        locals.var_idovvds_dn9 = (((locals.var_idlovvdseff_dn9 * locals.var_t0__blk808) + (locals.var_idlovvdseff * locals.var_t0__blk808_dn9)) / p.p23);
        locals.var_idovvds_dn10 = (((locals.var_idlovvdseff_dn10 * locals.var_t0__blk808) + (locals.var_idlovvdseff * locals.var_t0__blk808_dn10)) / p.p23);
        locals.var_idovvds_dn11 = (((locals.var_idlovvdseff_dn11 * locals.var_t0__blk808) + (locals.var_idlovvdseff * locals.var_t0__blk808_dn11)) / p.p23);
        locals.var_idovvds_dn12 = (((locals.var_idlovvdseff_dn12 * locals.var_t0__blk808) + (locals.var_idlovvdseff * locals.var_t0__blk808_dn12)) / p.p23);
        locals.var_idovvds_rv = 0.0;

        let assign20350_e18088: f64 = if locals.var_b4soisoimod != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1240 = assign20350_e18088;
        locals.var_guard1240_rv = 0.0;

        let assign20360_e18091: f64 = if p.p41 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1241 = assign20360_e18091;
        locals.var_guard1241_rv = 0.0;

        let (assign20370_e18103, assign20370_e18103_d_n3, assign20370_e18103_d_n4, assign20370_e18103_d_n5, assign20370_e18103_d_n6, assign20370_e18103_d_n7, assign20370_e18103_d_n8, assign20370_e18103_d_n9, assign20370_e18103_d_n10, assign20370_e18103_d_n11, assign20370_e18103_d_n12,) = {
    if ((locals.var_guard1240 != 0.0) && (locals.var_guard1241 != 0.0)) {
        let assign20370_e18097: f64 = (3.0 * 3.9);
        let assign20370_e18099: f64 = (assign20370_e18097 / locals.var_epsrox);
        let assign20370_e18101: f64 = (assign20370_e18099 * locals.var_toxe);
        (assign20370_e18101, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign20370_e18103;
        locals.var_t0__blk808_dn3 = assign20370_e18103_d_n3;
        locals.var_t0__blk808_dn4 = assign20370_e18103_d_n4;
        locals.var_t0__blk808_dn5 = assign20370_e18103_d_n5;
        locals.var_t0__blk808_dn6 = assign20370_e18103_d_n6;
        locals.var_t0__blk808_dn7 = assign20370_e18103_d_n7;
        locals.var_t0__blk808_dn8 = assign20370_e18103_d_n8;
        locals.var_t0__blk808_dn9 = assign20370_e18103_d_n9;
        locals.var_t0__blk808_dn10 = assign20370_e18103_d_n10;
        locals.var_t0__blk808_dn11 = assign20370_e18103_d_n11;
        locals.var_t0__blk808_dn12 = assign20370_e18103_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let (assign20380_e18114, assign20380_e18114_d_n3, assign20380_e18114_d_n4, assign20380_e18114_d_n5, assign20380_e18114_d_n6, assign20380_e18114_d_n7, assign20380_e18114_d_n8, assign20380_e18114_d_n9, assign20380_e18114_d_n10, assign20380_e18114_d_n11, assign20380_e18114_d_n12,) = {
    if ((locals.var_guard1240 != 0.0) && (locals.var_guard1241 == 0.0)) {
        let assign20380_e18110: f64 = (p.p47 * locals.var_toxe);
        let assign20380_e18112: f64 = (assign20380_e18110 / locals.var_epsrox);
        (assign20380_e18112, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign20380_e18114;
        locals.var_t0__blk808_dn3 = assign20380_e18114_d_n3;
        locals.var_t0__blk808_dn4 = assign20380_e18114_d_n4;
        locals.var_t0__blk808_dn5 = assign20380_e18114_d_n5;
        locals.var_t0__blk808_dn6 = assign20380_e18114_d_n6;
        locals.var_t0__blk808_dn7 = assign20380_e18114_d_n7;
        locals.var_t0__blk808_dn8 = assign20380_e18114_d_n8;
        locals.var_t0__blk808_dn9 = assign20380_e18114_d_n9;
        locals.var_t0__blk808_dn10 = assign20380_e18114_d_n10;
        locals.var_t0__blk808_dn11 = assign20380_e18114_d_n11;
        locals.var_t0__blk808_dn12 = assign20380_e18114_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let assign20390_e18117: f64 = if p.p43 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1242 = assign20390_e18117;
        locals.var_guard1242_rv = 0.0;

        let assign20400_e18120: f64 = if p.p41 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1243 = assign20400_e18120;
        locals.var_guard1243_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_62(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20410_e18135, assign20410_e18135_d_n3, assign20410_e18135_d_n4, assign20410_e18135_d_n5, assign20410_e18135_d_n6, assign20410_e18135_d_n7, assign20410_e18135_d_n8, assign20410_e18135_d_n9, assign20410_e18135_d_n10, assign20410_e18135_d_n11, assign20410_e18135_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 != 0.0)) && (locals.var_guard1243 != 0.0)) {
        let assign20410_e18127: f64 = (-locals.var_vds_1);
        let assign20410_e18129: f64 = (assign20410_e18127 - locals.var_vgd_eff_1);
        let assign20410_e18131: f64 = (assign20410_e18129 - locals.var_egisl);
        let assign20410_e18133: f64 = (assign20410_e18131 / locals.var_t0__blk808);
        (assign20410_e18133, (((((-locals.var_vgd_eff_1_dn3) - locals.var_egisl_dn3) * locals.var_t0__blk808) - (assign20410_e18131 * locals.var_t0__blk808_dn3)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-locals.var_vgd_eff_1_dn4) - locals.var_egisl_dn4) * locals.var_t0__blk808) - (assign20410_e18131 * locals.var_t0__blk808_dn4)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-locals.var_vgd_eff_1_dn5) - locals.var_egisl_dn5) * locals.var_t0__blk808) - (assign20410_e18131 * locals.var_t0__blk808_dn5)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-locals.var_vgd_eff_1_dn6) - locals.var_egisl_dn6) * locals.var_t0__blk808) - (assign20410_e18131 * locals.var_t0__blk808_dn6)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-locals.var_vds_1_dn7) - locals.var_vgd_eff_1_dn7) - locals.var_egisl_dn7) * locals.var_t0__blk808) - (assign20410_e18131 * locals.var_t0__blk808_dn7)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-locals.var_vds_1_dn8) - locals.var_vgd_eff_1_dn8) - locals.var_egisl_dn8) * locals.var_t0__blk808) - (assign20410_e18131 * locals.var_t0__blk808_dn8)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-locals.var_vgd_eff_1_dn9) - locals.var_egisl_dn9) * locals.var_t0__blk808) - (assign20410_e18131 * locals.var_t0__blk808_dn9)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-locals.var_vgd_eff_1_dn10) - locals.var_egisl_dn10) * locals.var_t0__blk808) - (assign20410_e18131 * locals.var_t0__blk808_dn10)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-locals.var_vgd_eff_1_dn11) - locals.var_egisl_dn11) * locals.var_t0__blk808) - (assign20410_e18131 * locals.var_t0__blk808_dn11)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-locals.var_vgd_eff_1_dn12) - locals.var_egisl_dn12) * locals.var_t0__blk808) - (assign20410_e18131 * locals.var_t0__blk808_dn12)) / (locals.var_t0__blk808 * locals.var_t0__blk808)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign20410_e18135;
        locals.var_t1__blk809_dn3 = assign20410_e18135_d_n3;
        locals.var_t1__blk809_dn4 = assign20410_e18135_d_n4;
        locals.var_t1__blk809_dn5 = assign20410_e18135_d_n5;
        locals.var_t1__blk809_dn6 = assign20410_e18135_d_n6;
        locals.var_t1__blk809_dn7 = assign20410_e18135_d_n7;
        locals.var_t1__blk809_dn8 = assign20410_e18135_d_n8;
        locals.var_t1__blk809_dn9 = assign20410_e18135_d_n9;
        locals.var_t1__blk809_dn10 = assign20410_e18135_d_n10;
        locals.var_t1__blk809_dn11 = assign20410_e18135_d_n11;
        locals.var_t1__blk809_dn12 = assign20410_e18135_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign20420_e18153, assign20420_e18153_d_n3, assign20420_e18153_d_n4, assign20420_e18153_d_n5, assign20420_e18153_d_n6, assign20420_e18153_d_n7, assign20420_e18153_d_n8, assign20420_e18153_d_n9, assign20420_e18153_d_n10, assign20420_e18153_d_n11, assign20420_e18153_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 != 0.0)) && (locals.var_guard1243 == 0.0)) {
        let assign20420_e18143: f64 = (-locals.var_vds_1);
        let assign20420_e18145: f64 = (assign20420_e18143 - locals.var_vgd_eff_1);
        let assign20420_e18147: f64 = (assign20420_e18145 - locals.var_egisl);
        let assign20420_e18149: f64 = (assign20420_e18147 + locals.var_pparam_b4soivfbsd);
        let assign20420_e18151: f64 = (assign20420_e18149 / locals.var_t0__blk808);
        (assign20420_e18151, ((((((-locals.var_vgd_eff_1_dn3) - locals.var_egisl_dn3) + locals.var_pparam_b4soivfbsd_dn3) * locals.var_t0__blk808) - (assign20420_e18149 * locals.var_t0__blk808_dn3)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-locals.var_vgd_eff_1_dn4) - locals.var_egisl_dn4) + locals.var_pparam_b4soivfbsd_dn4) * locals.var_t0__blk808) - (assign20420_e18149 * locals.var_t0__blk808_dn4)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-locals.var_vgd_eff_1_dn5) - locals.var_egisl_dn5) + locals.var_pparam_b4soivfbsd_dn5) * locals.var_t0__blk808) - (assign20420_e18149 * locals.var_t0__blk808_dn5)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-locals.var_vgd_eff_1_dn6) - locals.var_egisl_dn6) + locals.var_pparam_b4soivfbsd_dn6) * locals.var_t0__blk808) - (assign20420_e18149 * locals.var_t0__blk808_dn6)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((((-locals.var_vds_1_dn7) - locals.var_vgd_eff_1_dn7) - locals.var_egisl_dn7) + locals.var_pparam_b4soivfbsd_dn7) * locals.var_t0__blk808) - (assign20420_e18149 * locals.var_t0__blk808_dn7)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((((-locals.var_vds_1_dn8) - locals.var_vgd_eff_1_dn8) - locals.var_egisl_dn8) + locals.var_pparam_b4soivfbsd_dn8) * locals.var_t0__blk808) - (assign20420_e18149 * locals.var_t0__blk808_dn8)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-locals.var_vgd_eff_1_dn9) - locals.var_egisl_dn9) + locals.var_pparam_b4soivfbsd_dn9) * locals.var_t0__blk808) - (assign20420_e18149 * locals.var_t0__blk808_dn9)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-locals.var_vgd_eff_1_dn10) - locals.var_egisl_dn10) + locals.var_pparam_b4soivfbsd_dn10) * locals.var_t0__blk808) - (assign20420_e18149 * locals.var_t0__blk808_dn10)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-locals.var_vgd_eff_1_dn11) - locals.var_egisl_dn11) + locals.var_pparam_b4soivfbsd_dn11) * locals.var_t0__blk808) - (assign20420_e18149 * locals.var_t0__blk808_dn11)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-locals.var_vgd_eff_1_dn12) - locals.var_egisl_dn12) + locals.var_pparam_b4soivfbsd_dn12) * locals.var_t0__blk808) - (assign20420_e18149 * locals.var_t0__blk808_dn12)) / (locals.var_t0__blk808 * locals.var_t0__blk808)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign20420_e18153;
        locals.var_t1__blk809_dn3 = assign20420_e18153_d_n3;
        locals.var_t1__blk809_dn4 = assign20420_e18153_d_n4;
        locals.var_t1__blk809_dn5 = assign20420_e18153_d_n5;
        locals.var_t1__blk809_dn6 = assign20420_e18153_d_n6;
        locals.var_t1__blk809_dn7 = assign20420_e18153_d_n7;
        locals.var_t1__blk809_dn8 = assign20420_e18153_d_n8;
        locals.var_t1__blk809_dn9 = assign20420_e18153_d_n9;
        locals.var_t1__blk809_dn10 = assign20420_e18153_d_n10;
        locals.var_t1__blk809_dn11 = assign20420_e18153_d_n11;
        locals.var_t1__blk809_dn12 = assign20420_e18153_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let assign20430_e18164: f64 = if (((locals.var_agisl <= 0.0) || (locals.var_bgisl <= 0.0)) || (locals.var_cgisl < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1244 = assign20430_e18164;
        locals.var_guard1244_rv = 0.0;

        let (assign20450_e18194, assign20450_e18194_d_n3, assign20450_e18194_d_n4, assign20450_e18194_d_n5, assign20450_e18194_d_n6, assign20450_e18194_d_n7, assign20450_e18194_d_n8, assign20450_e18194_d_n9, assign20450_e18194_d_n10, assign20450_e18194_d_n11, assign20450_e18194_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 != 0.0)) && (locals.var_guard1244 == 0.0)) {
        let assign20450_e18183: f64 = (locals.var_t1__blk809 * locals.var_t1__blk809);
        let assign20450_e18186: f64 = (4.0 * 0.01);
        let assign20450_e18188: f64 = (assign20450_e18186 * 0.01);
        let assign20450_e18189: f64 = (assign20450_e18183 + assign20450_e18188);
        let assign20450_e18190: f64 = (assign20450_e18189).sqrt();
        let assign20450_e18191: f64 = (locals.var_t1__blk809 + assign20450_e18190);
        let assign20450_e18192: f64 = (0.5 * assign20450_e18191);
        (assign20450_e18192, (0.5 * (locals.var_t1__blk809_dn3 + (((locals.var_t1__blk809_dn3 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn3)) / (2.0 * assign20450_e18190)))), (0.5 * (locals.var_t1__blk809_dn4 + (((locals.var_t1__blk809_dn4 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn4)) / (2.0 * assign20450_e18190)))), (0.5 * (locals.var_t1__blk809_dn5 + (((locals.var_t1__blk809_dn5 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn5)) / (2.0 * assign20450_e18190)))), (0.5 * (locals.var_t1__blk809_dn6 + (((locals.var_t1__blk809_dn6 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn6)) / (2.0 * assign20450_e18190)))), (0.5 * (locals.var_t1__blk809_dn7 + (((locals.var_t1__blk809_dn7 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn7)) / (2.0 * assign20450_e18190)))), (0.5 * (locals.var_t1__blk809_dn8 + (((locals.var_t1__blk809_dn8 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn8)) / (2.0 * assign20450_e18190)))), (0.5 * (locals.var_t1__blk809_dn9 + (((locals.var_t1__blk809_dn9 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn9)) / (2.0 * assign20450_e18190)))), (0.5 * (locals.var_t1__blk809_dn10 + (((locals.var_t1__blk809_dn10 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn10)) / (2.0 * assign20450_e18190)))), (0.5 * (locals.var_t1__blk809_dn11 + (((locals.var_t1__blk809_dn11 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn11)) / (2.0 * assign20450_e18190)))), (0.5 * (locals.var_t1__blk809_dn12 + (((locals.var_t1__blk809_dn12 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn12)) / (2.0 * assign20450_e18190)))),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign20450_e18194;
        locals.var_t1__blk809_dn3 = assign20450_e18194_d_n3;
        locals.var_t1__blk809_dn4 = assign20450_e18194_d_n4;
        locals.var_t1__blk809_dn5 = assign20450_e18194_d_n5;
        locals.var_t1__blk809_dn6 = assign20450_e18194_d_n6;
        locals.var_t1__blk809_dn7 = assign20450_e18194_d_n7;
        locals.var_t1__blk809_dn8 = assign20450_e18194_d_n8;
        locals.var_t1__blk809_dn9 = assign20450_e18194_d_n9;
        locals.var_t1__blk809_dn10 = assign20450_e18194_d_n10;
        locals.var_t1__blk809_dn11 = assign20450_e18194_d_n11;
        locals.var_t1__blk809_dn12 = assign20450_e18194_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign20460_e18207, assign20460_e18207_d_n3, assign20460_e18207_d_n4, assign20460_e18207_d_n5, assign20460_e18207_d_n6, assign20460_e18207_d_n7, assign20460_e18207_d_n8, assign20460_e18207_d_n9, assign20460_e18207_d_n10, assign20460_e18207_d_n11, assign20460_e18207_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 != 0.0)) && (locals.var_guard1244 == 0.0)) {
        let assign20460_e18204: f64 = (locals.var_t1__blk809 + 0.001);
        let assign20460_e18205: f64 = (locals.var_bgisl / assign20460_e18204);
        (assign20460_e18205, (((locals.var_bgisl_dn3 * assign20460_e18204) - (locals.var_bgisl * locals.var_t1__blk809_dn3)) / (assign20460_e18204 * assign20460_e18204)), (((locals.var_bgisl_dn4 * assign20460_e18204) - (locals.var_bgisl * locals.var_t1__blk809_dn4)) / (assign20460_e18204 * assign20460_e18204)), (((locals.var_bgisl_dn5 * assign20460_e18204) - (locals.var_bgisl * locals.var_t1__blk809_dn5)) / (assign20460_e18204 * assign20460_e18204)), (((locals.var_bgisl_dn6 * assign20460_e18204) - (locals.var_bgisl * locals.var_t1__blk809_dn6)) / (assign20460_e18204 * assign20460_e18204)), (((locals.var_bgisl_dn7 * assign20460_e18204) - (locals.var_bgisl * locals.var_t1__blk809_dn7)) / (assign20460_e18204 * assign20460_e18204)), (((locals.var_bgisl_dn8 * assign20460_e18204) - (locals.var_bgisl * locals.var_t1__blk809_dn8)) / (assign20460_e18204 * assign20460_e18204)), (((locals.var_bgisl_dn9 * assign20460_e18204) - (locals.var_bgisl * locals.var_t1__blk809_dn9)) / (assign20460_e18204 * assign20460_e18204)), (((locals.var_bgisl_dn10 * assign20460_e18204) - (locals.var_bgisl * locals.var_t1__blk809_dn10)) / (assign20460_e18204 * assign20460_e18204)), (((locals.var_bgisl_dn11 * assign20460_e18204) - (locals.var_bgisl * locals.var_t1__blk809_dn11)) / (assign20460_e18204 * assign20460_e18204)), (((locals.var_bgisl_dn12 * assign20460_e18204) - (locals.var_bgisl * locals.var_t1__blk809_dn12)) / (assign20460_e18204 * assign20460_e18204)),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign20460_e18207;
        locals.var_t2__blk810_dn3 = assign20460_e18207_d_n3;
        locals.var_t2__blk810_dn4 = assign20460_e18207_d_n4;
        locals.var_t2__blk810_dn5 = assign20460_e18207_d_n5;
        locals.var_t2__blk810_dn6 = assign20460_e18207_d_n6;
        locals.var_t2__blk810_dn7 = assign20460_e18207_d_n7;
        locals.var_t2__blk810_dn8 = assign20460_e18207_d_n8;
        locals.var_t2__blk810_dn9 = assign20460_e18207_d_n9;
        locals.var_t2__blk810_dn10 = assign20460_e18207_d_n10;
        locals.var_t2__blk810_dn11 = assign20460_e18207_d_n11;
        locals.var_t2__blk810_dn12 = assign20460_e18207_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign20480_e18235, assign20480_e18235_d_n3, assign20480_e18235_d_n4, assign20480_e18235_d_n5, assign20480_e18235_d_n6, assign20480_e18235_d_n7, assign20480_e18235_d_n8, assign20480_e18235_d_n9, assign20480_e18235_d_n10, assign20480_e18235_d_n11, assign20480_e18235_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 != 0.0)) && (locals.var_guard1244 == 0.0)) {
        let assign20480_e18233: f64 = (locals.var_vbs_1 * locals.var_vbs_1);
        (assign20480_e18233, ((locals.var_vbs_1_dn3 * locals.var_vbs_1) + (locals.var_vbs_1 * locals.var_vbs_1_dn3)), ((locals.var_vbs_1_dn4 * locals.var_vbs_1) + (locals.var_vbs_1 * locals.var_vbs_1_dn4)), ((locals.var_vbs_1_dn5 * locals.var_vbs_1) + (locals.var_vbs_1 * locals.var_vbs_1_dn5)), ((locals.var_vbs_1_dn6 * locals.var_vbs_1) + (locals.var_vbs_1 * locals.var_vbs_1_dn6)), ((locals.var_vbs_1_dn7 * locals.var_vbs_1) + (locals.var_vbs_1 * locals.var_vbs_1_dn7)), ((locals.var_vbs_1_dn8 * locals.var_vbs_1) + (locals.var_vbs_1 * locals.var_vbs_1_dn8)), ((locals.var_vbs_1_dn9 * locals.var_vbs_1) + (locals.var_vbs_1 * locals.var_vbs_1_dn9)), ((locals.var_vbs_1_dn10 * locals.var_vbs_1) + (locals.var_vbs_1 * locals.var_vbs_1_dn10)), ((locals.var_vbs_1_dn11 * locals.var_vbs_1) + (locals.var_vbs_1 * locals.var_vbs_1_dn11)), ((locals.var_vbs_1_dn12 * locals.var_vbs_1) + (locals.var_vbs_1 * locals.var_vbs_1_dn12)),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign20480_e18235;
        locals.var_t4__blk812_dn3 = assign20480_e18235_d_n3;
        locals.var_t4__blk812_dn4 = assign20480_e18235_d_n4;
        locals.var_t4__blk812_dn5 = assign20480_e18235_d_n5;
        locals.var_t4__blk812_dn6 = assign20480_e18235_d_n6;
        locals.var_t4__blk812_dn7 = assign20480_e18235_d_n7;
        locals.var_t4__blk812_dn8 = assign20480_e18235_d_n8;
        locals.var_t4__blk812_dn9 = assign20480_e18235_d_n9;
        locals.var_t4__blk812_dn10 = assign20480_e18235_d_n10;
        locals.var_t4__blk812_dn11 = assign20480_e18235_d_n11;
        locals.var_t4__blk812_dn12 = assign20480_e18235_d_n12;
        locals.var_t4__blk812_rv = 0.0;

        let (assign20490_e18247, assign20490_e18247_d_n3, assign20490_e18247_d_n4, assign20490_e18247_d_n5, assign20490_e18247_d_n6, assign20490_e18247_d_n7, assign20490_e18247_d_n8, assign20490_e18247_d_n9, assign20490_e18247_d_n10, assign20490_e18247_d_n11, assign20490_e18247_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 != 0.0)) && (locals.var_guard1244 == 0.0)) {
        let assign20490_e18243: f64 = (-locals.var_vbs_1);
        let assign20490_e18245: f64 = (assign20490_e18243 * locals.var_t4__blk812);
        (assign20490_e18245, (((-locals.var_vbs_1_dn3) * locals.var_t4__blk812) + (assign20490_e18243 * locals.var_t4__blk812_dn3)), (((-locals.var_vbs_1_dn4) * locals.var_t4__blk812) + (assign20490_e18243 * locals.var_t4__blk812_dn4)), (((-locals.var_vbs_1_dn5) * locals.var_t4__blk812) + (assign20490_e18243 * locals.var_t4__blk812_dn5)), (((-locals.var_vbs_1_dn6) * locals.var_t4__blk812) + (assign20490_e18243 * locals.var_t4__blk812_dn6)), (((-locals.var_vbs_1_dn7) * locals.var_t4__blk812) + (assign20490_e18243 * locals.var_t4__blk812_dn7)), (((-locals.var_vbs_1_dn8) * locals.var_t4__blk812) + (assign20490_e18243 * locals.var_t4__blk812_dn8)), (((-locals.var_vbs_1_dn9) * locals.var_t4__blk812) + (assign20490_e18243 * locals.var_t4__blk812_dn9)), (((-locals.var_vbs_1_dn10) * locals.var_t4__blk812) + (assign20490_e18243 * locals.var_t4__blk812_dn10)), (((-locals.var_vbs_1_dn11) * locals.var_t4__blk812) + (assign20490_e18243 * locals.var_t4__blk812_dn11)), (((-locals.var_vbs_1_dn12) * locals.var_t4__blk812) + (assign20490_e18243 * locals.var_t4__blk812_dn12)),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign20490_e18247;
        locals.var_t5__blk813_dn3 = assign20490_e18247_d_n3;
        locals.var_t5__blk813_dn4 = assign20490_e18247_d_n4;
        locals.var_t5__blk813_dn5 = assign20490_e18247_d_n5;
        locals.var_t5__blk813_dn6 = assign20490_e18247_d_n6;
        locals.var_t5__blk813_dn7 = assign20490_e18247_d_n7;
        locals.var_t5__blk813_dn8 = assign20490_e18247_d_n8;
        locals.var_t5__blk813_dn9 = assign20490_e18247_d_n9;
        locals.var_t5__blk813_dn10 = assign20490_e18247_d_n10;
        locals.var_t5__blk813_dn11 = assign20490_e18247_d_n11;
        locals.var_t5__blk813_dn12 = assign20490_e18247_d_n12;
        locals.var_t5__blk813_rv = 0.0;

        let (assign20500_e18261, assign20500_e18261_d_n3, assign20500_e18261_d_n4, assign20500_e18261_d_n5, assign20500_e18261_d_n6, assign20500_e18261_d_n7, assign20500_e18261_d_n8, assign20500_e18261_d_n9, assign20500_e18261_d_n10, assign20500_e18261_d_n11, assign20500_e18261_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 != 0.0)) && (locals.var_guard1244 == 0.0)) {
        let assign20500_e18256: f64 = (locals.var_t5__blk813).abs();
        let assign20500_e18257: f64 = (locals.var_cgisl + assign20500_e18256);
        let assign20500_e18259: f64 = (assign20500_e18257 + 1e-9);
        (assign20500_e18259, (locals.var_cgisl_dn3 + if locals.var_t5__blk813 >= 0.0 { locals.var_t5__blk813_dn3 } else { (-locals.var_t5__blk813_dn3) }), (locals.var_cgisl_dn4 + if locals.var_t5__blk813 >= 0.0 { locals.var_t5__blk813_dn4 } else { (-locals.var_t5__blk813_dn4) }), (locals.var_cgisl_dn5 + if locals.var_t5__blk813 >= 0.0 { locals.var_t5__blk813_dn5 } else { (-locals.var_t5__blk813_dn5) }), (locals.var_cgisl_dn6 + if locals.var_t5__blk813 >= 0.0 { locals.var_t5__blk813_dn6 } else { (-locals.var_t5__blk813_dn6) }), (locals.var_cgisl_dn7 + if locals.var_t5__blk813 >= 0.0 { locals.var_t5__blk813_dn7 } else { (-locals.var_t5__blk813_dn7) }), (locals.var_cgisl_dn8 + if locals.var_t5__blk813 >= 0.0 { locals.var_t5__blk813_dn8 } else { (-locals.var_t5__blk813_dn8) }), (locals.var_cgisl_dn9 + if locals.var_t5__blk813 >= 0.0 { locals.var_t5__blk813_dn9 } else { (-locals.var_t5__blk813_dn9) }), (locals.var_cgisl_dn10 + if locals.var_t5__blk813 >= 0.0 { locals.var_t5__blk813_dn10 } else { (-locals.var_t5__blk813_dn10) }), (locals.var_cgisl_dn11 + if locals.var_t5__blk813 >= 0.0 { locals.var_t5__blk813_dn11 } else { (-locals.var_t5__blk813_dn11) }), (locals.var_cgisl_dn12 + if locals.var_t5__blk813 >= 0.0 { locals.var_t5__blk813_dn12 } else { (-locals.var_t5__blk813_dn12) }),)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign20500_e18261;
        locals.var_t6__blk814_dn3 = assign20500_e18261_d_n3;
        locals.var_t6__blk814_dn4 = assign20500_e18261_d_n4;
        locals.var_t6__blk814_dn5 = assign20500_e18261_d_n5;
        locals.var_t6__blk814_dn6 = assign20500_e18261_d_n6;
        locals.var_t6__blk814_dn7 = assign20500_e18261_d_n7;
        locals.var_t6__blk814_dn8 = assign20500_e18261_d_n8;
        locals.var_t6__blk814_dn9 = assign20500_e18261_d_n9;
        locals.var_t6__blk814_dn10 = assign20500_e18261_d_n10;
        locals.var_t6__blk814_dn11 = assign20500_e18261_d_n11;
        locals.var_t6__blk814_dn12 = assign20500_e18261_d_n12;
        locals.var_t6__blk814_rv = 0.0;

        let (assign20510_e18291, assign20510_e18291_d_n3, assign20510_e18291_d_n4, assign20510_e18291_d_n5, assign20510_e18291_d_n6, assign20510_e18291_d_n7, assign20510_e18291_d_n8, assign20510_e18291_d_n9, assign20510_e18291_d_n10, assign20510_e18291_d_n11, assign20510_e18291_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 != 0.0)) && (locals.var_guard1244 == 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t6__blk814;
        let assign20510_e18271: f64 = (locals.var_t5__blk813 * __rspice_inv_cse_0);
        let assign20510_e18274: f64 = (locals.var_t5__blk813 * __rspice_inv_cse_0);
        let assign20510_e18277: f64 = (locals.var_t5__blk813 * __rspice_inv_cse_0);
        let assign20510_e18278: f64 = (assign20510_e18274 * assign20510_e18277);
        let assign20510_e18281: f64 = (4.0 * 1e-6);
        let assign20510_e18283: f64 = (assign20510_e18281 * 1e-6);
        let assign20510_e18284: f64 = (assign20510_e18278 + assign20510_e18283);
        let assign20510_e18285: f64 = (assign20510_e18284).sqrt();
        let assign20510_e18286: f64 = (assign20510_e18271 + assign20510_e18285);
        let assign20510_e18287: f64 = (0.5 * assign20510_e18286);
        let assign20510_e18289: f64 = (assign20510_e18287 - 1e-6);
        (assign20510_e18289, (0.5 * ((((locals.var_t5__blk813_dn3 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn3)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) + ((((((locals.var_t5__blk813_dn3 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn3)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) * assign20510_e18277) + (assign20510_e18274 * (((locals.var_t5__blk813_dn3 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn3)) / (locals.var_t6__blk814 * locals.var_t6__blk814)))) / (2.0 * assign20510_e18285)))), (0.5 * ((((locals.var_t5__blk813_dn4 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn4)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) + ((((((locals.var_t5__blk813_dn4 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn4)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) * assign20510_e18277) + (assign20510_e18274 * (((locals.var_t5__blk813_dn4 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn4)) / (locals.var_t6__blk814 * locals.var_t6__blk814)))) / (2.0 * assign20510_e18285)))), (0.5 * ((((locals.var_t5__blk813_dn5 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn5)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) + ((((((locals.var_t5__blk813_dn5 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn5)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) * assign20510_e18277) + (assign20510_e18274 * (((locals.var_t5__blk813_dn5 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn5)) / (locals.var_t6__blk814 * locals.var_t6__blk814)))) / (2.0 * assign20510_e18285)))), (0.5 * ((((locals.var_t5__blk813_dn6 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn6)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) + ((((((locals.var_t5__blk813_dn6 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn6)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) * assign20510_e18277) + (assign20510_e18274 * (((locals.var_t5__blk813_dn6 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn6)) / (locals.var_t6__blk814 * locals.var_t6__blk814)))) / (2.0 * assign20510_e18285)))), (0.5 * ((((locals.var_t5__blk813_dn7 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn7)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) + ((((((locals.var_t5__blk813_dn7 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn7)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) * assign20510_e18277) + (assign20510_e18274 * (((locals.var_t5__blk813_dn7 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn7)) / (locals.var_t6__blk814 * locals.var_t6__blk814)))) / (2.0 * assign20510_e18285)))), (0.5 * ((((locals.var_t5__blk813_dn8 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn8)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) + ((((((locals.var_t5__blk813_dn8 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn8)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) * assign20510_e18277) + (assign20510_e18274 * (((locals.var_t5__blk813_dn8 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn8)) / (locals.var_t6__blk814 * locals.var_t6__blk814)))) / (2.0 * assign20510_e18285)))), (0.5 * ((((locals.var_t5__blk813_dn9 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn9)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) + ((((((locals.var_t5__blk813_dn9 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn9)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) * assign20510_e18277) + (assign20510_e18274 * (((locals.var_t5__blk813_dn9 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn9)) / (locals.var_t6__blk814 * locals.var_t6__blk814)))) / (2.0 * assign20510_e18285)))), (0.5 * ((((locals.var_t5__blk813_dn10 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn10)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) + ((((((locals.var_t5__blk813_dn10 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn10)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) * assign20510_e18277) + (assign20510_e18274 * (((locals.var_t5__blk813_dn10 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn10)) / (locals.var_t6__blk814 * locals.var_t6__blk814)))) / (2.0 * assign20510_e18285)))), (0.5 * ((((locals.var_t5__blk813_dn11 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn11)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) + ((((((locals.var_t5__blk813_dn11 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn11)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) * assign20510_e18277) + (assign20510_e18274 * (((locals.var_t5__blk813_dn11 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn11)) / (locals.var_t6__blk814 * locals.var_t6__blk814)))) / (2.0 * assign20510_e18285)))), (0.5 * ((((locals.var_t5__blk813_dn12 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn12)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) + ((((((locals.var_t5__blk813_dn12 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn12)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) * assign20510_e18277) + (assign20510_e18274 * (((locals.var_t5__blk813_dn12 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn12)) / (locals.var_t6__blk814 * locals.var_t6__blk814)))) / (2.0 * assign20510_e18285)))),)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign20510_e18291;
        locals.var_t7__blk815_dn3 = assign20510_e18291_d_n3;
        locals.var_t7__blk815_dn4 = assign20510_e18291_d_n4;
        locals.var_t7__blk815_dn5 = assign20510_e18291_d_n5;
        locals.var_t7__blk815_dn6 = assign20510_e18291_d_n6;
        locals.var_t7__blk815_dn7 = assign20510_e18291_d_n7;
        locals.var_t7__blk815_dn8 = assign20510_e18291_d_n8;
        locals.var_t7__blk815_dn9 = assign20510_e18291_d_n9;
        locals.var_t7__blk815_dn10 = assign20510_e18291_d_n10;
        locals.var_t7__blk815_dn11 = assign20510_e18291_d_n11;
        locals.var_t7__blk815_dn12 = assign20510_e18291_d_n12;
        locals.var_t7__blk815_rv = 0.0;

        let assign20530_e18305: f64 = if p.p41 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1245 = assign20530_e18305;
        locals.var_guard1245_rv = 0.0;

        let (assign20540_e18319, assign20540_e18319_d_n3, assign20540_e18319_d_n4, assign20540_e18319_d_n5, assign20540_e18319_d_n6, assign20540_e18319_d_n7, assign20540_e18319_d_n8, assign20540_e18319_d_n9, assign20540_e18319_d_n10, assign20540_e18319_d_n11, assign20540_e18319_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 != 0.0)) && (locals.var_guard1245 != 0.0)) {
        let assign20540_e18313: f64 = (locals.var_vds_1 - locals.var_vgs_eff__blk790);
        let assign20540_e18315: f64 = (assign20540_e18313 - locals.var_egidl);
        let assign20540_e18317: f64 = (assign20540_e18315 / locals.var_t0__blk808);
        (assign20540_e18317, (((((-locals.var_vgs_eff__blk790_dn3) - locals.var_egidl_dn3) * locals.var_t0__blk808) - (assign20540_e18315 * locals.var_t0__blk808_dn3)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-locals.var_vgs_eff__blk790_dn4) - locals.var_egidl_dn4) * locals.var_t0__blk808) - (assign20540_e18315 * locals.var_t0__blk808_dn4)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-locals.var_vgs_eff__blk790_dn5) - locals.var_egidl_dn5) * locals.var_t0__blk808) - (assign20540_e18315 * locals.var_t0__blk808_dn5)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-locals.var_vgs_eff__blk790_dn6) - locals.var_egidl_dn6) * locals.var_t0__blk808) - (assign20540_e18315 * locals.var_t0__blk808_dn6)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((locals.var_vds_1_dn7 - locals.var_vgs_eff__blk790_dn7) - locals.var_egidl_dn7) * locals.var_t0__blk808) - (assign20540_e18315 * locals.var_t0__blk808_dn7)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((locals.var_vds_1_dn8 - locals.var_vgs_eff__blk790_dn8) - locals.var_egidl_dn8) * locals.var_t0__blk808) - (assign20540_e18315 * locals.var_t0__blk808_dn8)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-locals.var_vgs_eff__blk790_dn9) - locals.var_egidl_dn9) * locals.var_t0__blk808) - (assign20540_e18315 * locals.var_t0__blk808_dn9)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-locals.var_vgs_eff__blk790_dn10) - locals.var_egidl_dn10) * locals.var_t0__blk808) - (assign20540_e18315 * locals.var_t0__blk808_dn10)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-locals.var_vgs_eff__blk790_dn11) - locals.var_egidl_dn11) * locals.var_t0__blk808) - (assign20540_e18315 * locals.var_t0__blk808_dn11)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-locals.var_vgs_eff__blk790_dn12) - locals.var_egidl_dn12) * locals.var_t0__blk808) - (assign20540_e18315 * locals.var_t0__blk808_dn12)) / (locals.var_t0__blk808 * locals.var_t0__blk808)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign20540_e18319;
        locals.var_t1__blk809_dn3 = assign20540_e18319_d_n3;
        locals.var_t1__blk809_dn4 = assign20540_e18319_d_n4;
        locals.var_t1__blk809_dn5 = assign20540_e18319_d_n5;
        locals.var_t1__blk809_dn6 = assign20540_e18319_d_n6;
        locals.var_t1__blk809_dn7 = assign20540_e18319_d_n7;
        locals.var_t1__blk809_dn8 = assign20540_e18319_d_n8;
        locals.var_t1__blk809_dn9 = assign20540_e18319_d_n9;
        locals.var_t1__blk809_dn10 = assign20540_e18319_d_n10;
        locals.var_t1__blk809_dn11 = assign20540_e18319_d_n11;
        locals.var_t1__blk809_dn12 = assign20540_e18319_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign20550_e18336, assign20550_e18336_d_n3, assign20550_e18336_d_n4, assign20550_e18336_d_n5, assign20550_e18336_d_n6, assign20550_e18336_d_n7, assign20550_e18336_d_n8, assign20550_e18336_d_n9, assign20550_e18336_d_n10, assign20550_e18336_d_n11, assign20550_e18336_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 != 0.0)) && (locals.var_guard1245 == 0.0)) {
        let assign20550_e18328: f64 = (locals.var_vds_1 - locals.var_vgs_eff__blk790);
        let assign20550_e18330: f64 = (assign20550_e18328 - locals.var_egidl);
        let assign20550_e18332: f64 = (assign20550_e18330 + locals.var_pparam_b4soivfbsd);
        let assign20550_e18334: f64 = (assign20550_e18332 / locals.var_t0__blk808);
        (assign20550_e18334, ((((((-locals.var_vgs_eff__blk790_dn3) - locals.var_egidl_dn3) + locals.var_pparam_b4soivfbsd_dn3) * locals.var_t0__blk808) - (assign20550_e18332 * locals.var_t0__blk808_dn3)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-locals.var_vgs_eff__blk790_dn4) - locals.var_egidl_dn4) + locals.var_pparam_b4soivfbsd_dn4) * locals.var_t0__blk808) - (assign20550_e18332 * locals.var_t0__blk808_dn4)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-locals.var_vgs_eff__blk790_dn5) - locals.var_egidl_dn5) + locals.var_pparam_b4soivfbsd_dn5) * locals.var_t0__blk808) - (assign20550_e18332 * locals.var_t0__blk808_dn5)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-locals.var_vgs_eff__blk790_dn6) - locals.var_egidl_dn6) + locals.var_pparam_b4soivfbsd_dn6) * locals.var_t0__blk808) - (assign20550_e18332 * locals.var_t0__blk808_dn6)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((locals.var_vds_1_dn7 - locals.var_vgs_eff__blk790_dn7) - locals.var_egidl_dn7) + locals.var_pparam_b4soivfbsd_dn7) * locals.var_t0__blk808) - (assign20550_e18332 * locals.var_t0__blk808_dn7)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((locals.var_vds_1_dn8 - locals.var_vgs_eff__blk790_dn8) - locals.var_egidl_dn8) + locals.var_pparam_b4soivfbsd_dn8) * locals.var_t0__blk808) - (assign20550_e18332 * locals.var_t0__blk808_dn8)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-locals.var_vgs_eff__blk790_dn9) - locals.var_egidl_dn9) + locals.var_pparam_b4soivfbsd_dn9) * locals.var_t0__blk808) - (assign20550_e18332 * locals.var_t0__blk808_dn9)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-locals.var_vgs_eff__blk790_dn10) - locals.var_egidl_dn10) + locals.var_pparam_b4soivfbsd_dn10) * locals.var_t0__blk808) - (assign20550_e18332 * locals.var_t0__blk808_dn10)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-locals.var_vgs_eff__blk790_dn11) - locals.var_egidl_dn11) + locals.var_pparam_b4soivfbsd_dn11) * locals.var_t0__blk808) - (assign20550_e18332 * locals.var_t0__blk808_dn11)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-locals.var_vgs_eff__blk790_dn12) - locals.var_egidl_dn12) + locals.var_pparam_b4soivfbsd_dn12) * locals.var_t0__blk808) - (assign20550_e18332 * locals.var_t0__blk808_dn12)) / (locals.var_t0__blk808 * locals.var_t0__blk808)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign20550_e18336;
        locals.var_t1__blk809_dn3 = assign20550_e18336_d_n3;
        locals.var_t1__blk809_dn4 = assign20550_e18336_d_n4;
        locals.var_t1__blk809_dn5 = assign20550_e18336_d_n5;
        locals.var_t1__blk809_dn6 = assign20550_e18336_d_n6;
        locals.var_t1__blk809_dn7 = assign20550_e18336_d_n7;
        locals.var_t1__blk809_dn8 = assign20550_e18336_d_n8;
        locals.var_t1__blk809_dn9 = assign20550_e18336_d_n9;
        locals.var_t1__blk809_dn10 = assign20550_e18336_d_n10;
        locals.var_t1__blk809_dn11 = assign20550_e18336_d_n11;
        locals.var_t1__blk809_dn12 = assign20550_e18336_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let assign20560_e18347: f64 = if (((locals.var_agidl <= 0.0) || (locals.var_bgidl <= 0.0)) || (locals.var_cgidl < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1246 = assign20560_e18347;
        locals.var_guard1246_rv = 0.0;

        let (assign20580_e18377, assign20580_e18377_d_n3, assign20580_e18377_d_n4, assign20580_e18377_d_n5, assign20580_e18377_d_n6, assign20580_e18377_d_n7, assign20580_e18377_d_n8, assign20580_e18377_d_n9, assign20580_e18377_d_n10, assign20580_e18377_d_n11, assign20580_e18377_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 != 0.0)) && (locals.var_guard1246 == 0.0)) {
        let assign20580_e18366: f64 = (locals.var_t1__blk809 * locals.var_t1__blk809);
        let assign20580_e18369: f64 = (4.0 * 0.01);
        let assign20580_e18371: f64 = (assign20580_e18369 * 0.01);
        let assign20580_e18372: f64 = (assign20580_e18366 + assign20580_e18371);
        let assign20580_e18373: f64 = (assign20580_e18372).sqrt();
        let assign20580_e18374: f64 = (locals.var_t1__blk809 + assign20580_e18373);
        let assign20580_e18375: f64 = (0.5 * assign20580_e18374);
        (assign20580_e18375, (0.5 * (locals.var_t1__blk809_dn3 + (((locals.var_t1__blk809_dn3 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn3)) / (2.0 * assign20580_e18373)))), (0.5 * (locals.var_t1__blk809_dn4 + (((locals.var_t1__blk809_dn4 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn4)) / (2.0 * assign20580_e18373)))), (0.5 * (locals.var_t1__blk809_dn5 + (((locals.var_t1__blk809_dn5 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn5)) / (2.0 * assign20580_e18373)))), (0.5 * (locals.var_t1__blk809_dn6 + (((locals.var_t1__blk809_dn6 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn6)) / (2.0 * assign20580_e18373)))), (0.5 * (locals.var_t1__blk809_dn7 + (((locals.var_t1__blk809_dn7 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn7)) / (2.0 * assign20580_e18373)))), (0.5 * (locals.var_t1__blk809_dn8 + (((locals.var_t1__blk809_dn8 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn8)) / (2.0 * assign20580_e18373)))), (0.5 * (locals.var_t1__blk809_dn9 + (((locals.var_t1__blk809_dn9 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn9)) / (2.0 * assign20580_e18373)))), (0.5 * (locals.var_t1__blk809_dn10 + (((locals.var_t1__blk809_dn10 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn10)) / (2.0 * assign20580_e18373)))), (0.5 * (locals.var_t1__blk809_dn11 + (((locals.var_t1__blk809_dn11 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn11)) / (2.0 * assign20580_e18373)))), (0.5 * (locals.var_t1__blk809_dn12 + (((locals.var_t1__blk809_dn12 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn12)) / (2.0 * assign20580_e18373)))),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign20580_e18377;
        locals.var_t1__blk809_dn3 = assign20580_e18377_d_n3;
        locals.var_t1__blk809_dn4 = assign20580_e18377_d_n4;
        locals.var_t1__blk809_dn5 = assign20580_e18377_d_n5;
        locals.var_t1__blk809_dn6 = assign20580_e18377_d_n6;
        locals.var_t1__blk809_dn7 = assign20580_e18377_d_n7;
        locals.var_t1__blk809_dn8 = assign20580_e18377_d_n8;
        locals.var_t1__blk809_dn9 = assign20580_e18377_d_n9;
        locals.var_t1__blk809_dn10 = assign20580_e18377_d_n10;
        locals.var_t1__blk809_dn11 = assign20580_e18377_d_n11;
        locals.var_t1__blk809_dn12 = assign20580_e18377_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign20590_e18390, assign20590_e18390_d_n3, assign20590_e18390_d_n4, assign20590_e18390_d_n5, assign20590_e18390_d_n6, assign20590_e18390_d_n7, assign20590_e18390_d_n8, assign20590_e18390_d_n9, assign20590_e18390_d_n10, assign20590_e18390_d_n11, assign20590_e18390_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 != 0.0)) && (locals.var_guard1246 == 0.0)) {
        let assign20590_e18387: f64 = (locals.var_t1__blk809 + 0.001);
        let assign20590_e18388: f64 = (locals.var_bgidl / assign20590_e18387);
        (assign20590_e18388, (((locals.var_bgidl_dn3 * assign20590_e18387) - (locals.var_bgidl * locals.var_t1__blk809_dn3)) / (assign20590_e18387 * assign20590_e18387)), (((locals.var_bgidl_dn4 * assign20590_e18387) - (locals.var_bgidl * locals.var_t1__blk809_dn4)) / (assign20590_e18387 * assign20590_e18387)), (((locals.var_bgidl_dn5 * assign20590_e18387) - (locals.var_bgidl * locals.var_t1__blk809_dn5)) / (assign20590_e18387 * assign20590_e18387)), (((locals.var_bgidl_dn6 * assign20590_e18387) - (locals.var_bgidl * locals.var_t1__blk809_dn6)) / (assign20590_e18387 * assign20590_e18387)), (((locals.var_bgidl_dn7 * assign20590_e18387) - (locals.var_bgidl * locals.var_t1__blk809_dn7)) / (assign20590_e18387 * assign20590_e18387)), (((locals.var_bgidl_dn8 * assign20590_e18387) - (locals.var_bgidl * locals.var_t1__blk809_dn8)) / (assign20590_e18387 * assign20590_e18387)), (((locals.var_bgidl_dn9 * assign20590_e18387) - (locals.var_bgidl * locals.var_t1__blk809_dn9)) / (assign20590_e18387 * assign20590_e18387)), (((locals.var_bgidl_dn10 * assign20590_e18387) - (locals.var_bgidl * locals.var_t1__blk809_dn10)) / (assign20590_e18387 * assign20590_e18387)), (((locals.var_bgidl_dn11 * assign20590_e18387) - (locals.var_bgidl * locals.var_t1__blk809_dn11)) / (assign20590_e18387 * assign20590_e18387)), (((locals.var_bgidl_dn12 * assign20590_e18387) - (locals.var_bgidl * locals.var_t1__blk809_dn12)) / (assign20590_e18387 * assign20590_e18387)),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign20590_e18390;
        locals.var_t2__blk810_dn3 = assign20590_e18390_d_n3;
        locals.var_t2__blk810_dn4 = assign20590_e18390_d_n4;
        locals.var_t2__blk810_dn5 = assign20590_e18390_d_n5;
        locals.var_t2__blk810_dn6 = assign20590_e18390_d_n6;
        locals.var_t2__blk810_dn7 = assign20590_e18390_d_n7;
        locals.var_t2__blk810_dn8 = assign20590_e18390_d_n8;
        locals.var_t2__blk810_dn9 = assign20590_e18390_d_n9;
        locals.var_t2__blk810_dn10 = assign20590_e18390_d_n10;
        locals.var_t2__blk810_dn11 = assign20590_e18390_d_n11;
        locals.var_t2__blk810_dn12 = assign20590_e18390_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign20610_e18418, assign20610_e18418_d_n3, assign20610_e18418_d_n4, assign20610_e18418_d_n5, assign20610_e18418_d_n6, assign20610_e18418_d_n7, assign20610_e18418_d_n8, assign20610_e18418_d_n9, assign20610_e18418_d_n10, assign20610_e18418_d_n11, assign20610_e18418_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 != 0.0)) && (locals.var_guard1246 == 0.0)) {
        let assign20610_e18416: f64 = (locals.var_vbd_1 * locals.var_vbd_1);
        (assign20610_e18416, 0.0, 0.0, ((locals.var_vbd_1_dn5 * locals.var_vbd_1) + (locals.var_vbd_1 * locals.var_vbd_1_dn5)), 0.0, ((locals.var_vbd_1_dn7 * locals.var_vbd_1) + (locals.var_vbd_1 * locals.var_vbd_1_dn7)), ((locals.var_vbd_1_dn8 * locals.var_vbd_1) + (locals.var_vbd_1 * locals.var_vbd_1_dn8)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign20610_e18418;
        locals.var_t4__blk812_dn3 = assign20610_e18418_d_n3;
        locals.var_t4__blk812_dn4 = assign20610_e18418_d_n4;
        locals.var_t4__blk812_dn5 = assign20610_e18418_d_n5;
        locals.var_t4__blk812_dn6 = assign20610_e18418_d_n6;
        locals.var_t4__blk812_dn7 = assign20610_e18418_d_n7;
        locals.var_t4__blk812_dn8 = assign20610_e18418_d_n8;
        locals.var_t4__blk812_dn9 = assign20610_e18418_d_n9;
        locals.var_t4__blk812_dn10 = assign20610_e18418_d_n10;
        locals.var_t4__blk812_dn11 = assign20610_e18418_d_n11;
        locals.var_t4__blk812_dn12 = assign20610_e18418_d_n12;
        locals.var_t4__blk812_rv = 0.0;

        let (assign20620_e18430, assign20620_e18430_d_n3, assign20620_e18430_d_n4, assign20620_e18430_d_n5, assign20620_e18430_d_n6, assign20620_e18430_d_n7, assign20620_e18430_d_n8, assign20620_e18430_d_n9, assign20620_e18430_d_n10, assign20620_e18430_d_n11, assign20620_e18430_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 != 0.0)) && (locals.var_guard1246 == 0.0)) {
        let assign20620_e18426: f64 = (-locals.var_vbd_1);
        let assign20620_e18428: f64 = (assign20620_e18426 * locals.var_t4__blk812);
        (assign20620_e18428, (assign20620_e18426 * locals.var_t4__blk812_dn3), (assign20620_e18426 * locals.var_t4__blk812_dn4), (((-locals.var_vbd_1_dn5) * locals.var_t4__blk812) + (assign20620_e18426 * locals.var_t4__blk812_dn5)), (assign20620_e18426 * locals.var_t4__blk812_dn6), (((-locals.var_vbd_1_dn7) * locals.var_t4__blk812) + (assign20620_e18426 * locals.var_t4__blk812_dn7)), (((-locals.var_vbd_1_dn8) * locals.var_t4__blk812) + (assign20620_e18426 * locals.var_t4__blk812_dn8)), (assign20620_e18426 * locals.var_t4__blk812_dn9), (assign20620_e18426 * locals.var_t4__blk812_dn10), (assign20620_e18426 * locals.var_t4__blk812_dn11), (assign20620_e18426 * locals.var_t4__blk812_dn12),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign20620_e18430;
        locals.var_t5__blk813_dn3 = assign20620_e18430_d_n3;
        locals.var_t5__blk813_dn4 = assign20620_e18430_d_n4;
        locals.var_t5__blk813_dn5 = assign20620_e18430_d_n5;
        locals.var_t5__blk813_dn6 = assign20620_e18430_d_n6;
        locals.var_t5__blk813_dn7 = assign20620_e18430_d_n7;
        locals.var_t5__blk813_dn8 = assign20620_e18430_d_n8;
        locals.var_t5__blk813_dn9 = assign20620_e18430_d_n9;
        locals.var_t5__blk813_dn10 = assign20620_e18430_d_n10;
        locals.var_t5__blk813_dn11 = assign20620_e18430_d_n11;
        locals.var_t5__blk813_dn12 = assign20620_e18430_d_n12;
        locals.var_t5__blk813_rv = 0.0;

        let (assign20630_e18444, assign20630_e18444_d_n3, assign20630_e18444_d_n4, assign20630_e18444_d_n5, assign20630_e18444_d_n6, assign20630_e18444_d_n7, assign20630_e18444_d_n8, assign20630_e18444_d_n9, assign20630_e18444_d_n10, assign20630_e18444_d_n11, assign20630_e18444_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 != 0.0)) && (locals.var_guard1246 == 0.0)) {
        let assign20630_e18439: f64 = (locals.var_t5__blk813).abs();
        let assign20630_e18440: f64 = (locals.var_cgidl + assign20630_e18439);
        let assign20630_e18442: f64 = (assign20630_e18440 + 1e-9);
        (assign20630_e18442, (locals.var_cgidl_dn3 + if locals.var_t5__blk813 >= 0.0 { locals.var_t5__blk813_dn3 } else { (-locals.var_t5__blk813_dn3) }), (locals.var_cgidl_dn4 + if locals.var_t5__blk813 >= 0.0 { locals.var_t5__blk813_dn4 } else { (-locals.var_t5__blk813_dn4) }), (locals.var_cgidl_dn5 + if locals.var_t5__blk813 >= 0.0 { locals.var_t5__blk813_dn5 } else { (-locals.var_t5__blk813_dn5) }), (locals.var_cgidl_dn6 + if locals.var_t5__blk813 >= 0.0 { locals.var_t5__blk813_dn6 } else { (-locals.var_t5__blk813_dn6) }), (locals.var_cgidl_dn7 + if locals.var_t5__blk813 >= 0.0 { locals.var_t5__blk813_dn7 } else { (-locals.var_t5__blk813_dn7) }), (locals.var_cgidl_dn8 + if locals.var_t5__blk813 >= 0.0 { locals.var_t5__blk813_dn8 } else { (-locals.var_t5__blk813_dn8) }), (locals.var_cgidl_dn9 + if locals.var_t5__blk813 >= 0.0 { locals.var_t5__blk813_dn9 } else { (-locals.var_t5__blk813_dn9) }), (locals.var_cgidl_dn10 + if locals.var_t5__blk813 >= 0.0 { locals.var_t5__blk813_dn10 } else { (-locals.var_t5__blk813_dn10) }), (locals.var_cgidl_dn11 + if locals.var_t5__blk813 >= 0.0 { locals.var_t5__blk813_dn11 } else { (-locals.var_t5__blk813_dn11) }), (locals.var_cgidl_dn12 + if locals.var_t5__blk813 >= 0.0 { locals.var_t5__blk813_dn12 } else { (-locals.var_t5__blk813_dn12) }),)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign20630_e18444;
        locals.var_t6__blk814_dn3 = assign20630_e18444_d_n3;
        locals.var_t6__blk814_dn4 = assign20630_e18444_d_n4;
        locals.var_t6__blk814_dn5 = assign20630_e18444_d_n5;
        locals.var_t6__blk814_dn6 = assign20630_e18444_d_n6;
        locals.var_t6__blk814_dn7 = assign20630_e18444_d_n7;
        locals.var_t6__blk814_dn8 = assign20630_e18444_d_n8;
        locals.var_t6__blk814_dn9 = assign20630_e18444_d_n9;
        locals.var_t6__blk814_dn10 = assign20630_e18444_d_n10;
        locals.var_t6__blk814_dn11 = assign20630_e18444_d_n11;
        locals.var_t6__blk814_dn12 = assign20630_e18444_d_n12;
        locals.var_t6__blk814_rv = 0.0;

        let (assign20640_e18474, assign20640_e18474_d_n3, assign20640_e18474_d_n4, assign20640_e18474_d_n5, assign20640_e18474_d_n6, assign20640_e18474_d_n7, assign20640_e18474_d_n8, assign20640_e18474_d_n9, assign20640_e18474_d_n10, assign20640_e18474_d_n11, assign20640_e18474_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 != 0.0)) && (locals.var_guard1246 == 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_t6__blk814;
        let assign20640_e18454: f64 = (locals.var_t5__blk813 * __rspice_inv_cse_1);
        let assign20640_e18457: f64 = (locals.var_t5__blk813 * __rspice_inv_cse_1);
        let assign20640_e18460: f64 = (locals.var_t5__blk813 * __rspice_inv_cse_1);
        let assign20640_e18461: f64 = (assign20640_e18457 * assign20640_e18460);
        let assign20640_e18464: f64 = (4.0 * 1e-6);
        let assign20640_e18466: f64 = (assign20640_e18464 * 1e-6);
        let assign20640_e18467: f64 = (assign20640_e18461 + assign20640_e18466);
        let assign20640_e18468: f64 = (assign20640_e18467).sqrt();
        let assign20640_e18469: f64 = (assign20640_e18454 + assign20640_e18468);
        let assign20640_e18470: f64 = (0.5 * assign20640_e18469);
        let assign20640_e18472: f64 = (assign20640_e18470 - 1e-6);
        (assign20640_e18472, (0.5 * ((((locals.var_t5__blk813_dn3 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn3)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) + ((((((locals.var_t5__blk813_dn3 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn3)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) * assign20640_e18460) + (assign20640_e18457 * (((locals.var_t5__blk813_dn3 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn3)) / (locals.var_t6__blk814 * locals.var_t6__blk814)))) / (2.0 * assign20640_e18468)))), (0.5 * ((((locals.var_t5__blk813_dn4 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn4)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) + ((((((locals.var_t5__blk813_dn4 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn4)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) * assign20640_e18460) + (assign20640_e18457 * (((locals.var_t5__blk813_dn4 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn4)) / (locals.var_t6__blk814 * locals.var_t6__blk814)))) / (2.0 * assign20640_e18468)))), (0.5 * ((((locals.var_t5__blk813_dn5 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn5)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) + ((((((locals.var_t5__blk813_dn5 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn5)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) * assign20640_e18460) + (assign20640_e18457 * (((locals.var_t5__blk813_dn5 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn5)) / (locals.var_t6__blk814 * locals.var_t6__blk814)))) / (2.0 * assign20640_e18468)))), (0.5 * ((((locals.var_t5__blk813_dn6 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn6)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) + ((((((locals.var_t5__blk813_dn6 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn6)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) * assign20640_e18460) + (assign20640_e18457 * (((locals.var_t5__blk813_dn6 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn6)) / (locals.var_t6__blk814 * locals.var_t6__blk814)))) / (2.0 * assign20640_e18468)))), (0.5 * ((((locals.var_t5__blk813_dn7 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn7)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) + ((((((locals.var_t5__blk813_dn7 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn7)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) * assign20640_e18460) + (assign20640_e18457 * (((locals.var_t5__blk813_dn7 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn7)) / (locals.var_t6__blk814 * locals.var_t6__blk814)))) / (2.0 * assign20640_e18468)))), (0.5 * ((((locals.var_t5__blk813_dn8 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn8)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) + ((((((locals.var_t5__blk813_dn8 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn8)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) * assign20640_e18460) + (assign20640_e18457 * (((locals.var_t5__blk813_dn8 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn8)) / (locals.var_t6__blk814 * locals.var_t6__blk814)))) / (2.0 * assign20640_e18468)))), (0.5 * ((((locals.var_t5__blk813_dn9 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn9)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) + ((((((locals.var_t5__blk813_dn9 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn9)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) * assign20640_e18460) + (assign20640_e18457 * (((locals.var_t5__blk813_dn9 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn9)) / (locals.var_t6__blk814 * locals.var_t6__blk814)))) / (2.0 * assign20640_e18468)))), (0.5 * ((((locals.var_t5__blk813_dn10 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn10)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) + ((((((locals.var_t5__blk813_dn10 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn10)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) * assign20640_e18460) + (assign20640_e18457 * (((locals.var_t5__blk813_dn10 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn10)) / (locals.var_t6__blk814 * locals.var_t6__blk814)))) / (2.0 * assign20640_e18468)))), (0.5 * ((((locals.var_t5__blk813_dn11 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn11)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) + ((((((locals.var_t5__blk813_dn11 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn11)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) * assign20640_e18460) + (assign20640_e18457 * (((locals.var_t5__blk813_dn11 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn11)) / (locals.var_t6__blk814 * locals.var_t6__blk814)))) / (2.0 * assign20640_e18468)))), (0.5 * ((((locals.var_t5__blk813_dn12 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn12)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) + ((((((locals.var_t5__blk813_dn12 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn12)) / (locals.var_t6__blk814 * locals.var_t6__blk814)) * assign20640_e18460) + (assign20640_e18457 * (((locals.var_t5__blk813_dn12 * locals.var_t6__blk814) - (locals.var_t5__blk813 * locals.var_t6__blk814_dn12)) / (locals.var_t6__blk814 * locals.var_t6__blk814)))) / (2.0 * assign20640_e18468)))),)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign20640_e18474;
        locals.var_t7__blk815_dn3 = assign20640_e18474_d_n3;
        locals.var_t7__blk815_dn4 = assign20640_e18474_d_n4;
        locals.var_t7__blk815_dn5 = assign20640_e18474_d_n5;
        locals.var_t7__blk815_dn6 = assign20640_e18474_d_n6;
        locals.var_t7__blk815_dn7 = assign20640_e18474_d_n7;
        locals.var_t7__blk815_dn8 = assign20640_e18474_d_n8;
        locals.var_t7__blk815_dn9 = assign20640_e18474_d_n9;
        locals.var_t7__blk815_dn10 = assign20640_e18474_d_n10;
        locals.var_t7__blk815_dn11 = assign20640_e18474_d_n11;
        locals.var_t7__blk815_dn12 = assign20640_e18474_d_n12;
        locals.var_t7__blk815_rv = 0.0;

        let assign20660_e18488: f64 = if p.p41 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1247 = assign20660_e18488;
        locals.var_guard1247_rv = 0.0;

        let (assign20670_e18506, assign20670_e18506_d_n3, assign20670_e18506_d_n4, assign20670_e18506_d_n5, assign20670_e18506_d_n6, assign20670_e18506_d_n7, assign20670_e18506_d_n8, assign20670_e18506_d_n9, assign20670_e18506_d_n10, assign20670_e18506_d_n11, assign20670_e18506_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 == 0.0)) && (locals.var_guard1247 != 0.0)) {
        let assign20670_e18496: f64 = (-locals.var_vds_1);
        let assign20670_e18499: f64 = (locals.var_rgisl * locals.var_vgd_eff_1);
        let assign20670_e18500: f64 = (assign20670_e18496 - assign20670_e18499);
        let assign20670_e18502: f64 = (assign20670_e18500 - locals.var_egisl);
        let assign20670_e18504: f64 = (assign20670_e18502 / locals.var_t0__blk808);
        (assign20670_e18504, (((((-((locals.var_rgisl_dn3 * locals.var_vgd_eff_1) + (locals.var_rgisl * locals.var_vgd_eff_1_dn3))) - locals.var_egisl_dn3) * locals.var_t0__blk808) - (assign20670_e18502 * locals.var_t0__blk808_dn3)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-((locals.var_rgisl_dn4 * locals.var_vgd_eff_1) + (locals.var_rgisl * locals.var_vgd_eff_1_dn4))) - locals.var_egisl_dn4) * locals.var_t0__blk808) - (assign20670_e18502 * locals.var_t0__blk808_dn4)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-((locals.var_rgisl_dn5 * locals.var_vgd_eff_1) + (locals.var_rgisl * locals.var_vgd_eff_1_dn5))) - locals.var_egisl_dn5) * locals.var_t0__blk808) - (assign20670_e18502 * locals.var_t0__blk808_dn5)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-((locals.var_rgisl_dn6 * locals.var_vgd_eff_1) + (locals.var_rgisl * locals.var_vgd_eff_1_dn6))) - locals.var_egisl_dn6) * locals.var_t0__blk808) - (assign20670_e18502 * locals.var_t0__blk808_dn6)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-locals.var_vds_1_dn7) - ((locals.var_rgisl_dn7 * locals.var_vgd_eff_1) + (locals.var_rgisl * locals.var_vgd_eff_1_dn7))) - locals.var_egisl_dn7) * locals.var_t0__blk808) - (assign20670_e18502 * locals.var_t0__blk808_dn7)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-locals.var_vds_1_dn8) - ((locals.var_rgisl_dn8 * locals.var_vgd_eff_1) + (locals.var_rgisl * locals.var_vgd_eff_1_dn8))) - locals.var_egisl_dn8) * locals.var_t0__blk808) - (assign20670_e18502 * locals.var_t0__blk808_dn8)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-((locals.var_rgisl_dn9 * locals.var_vgd_eff_1) + (locals.var_rgisl * locals.var_vgd_eff_1_dn9))) - locals.var_egisl_dn9) * locals.var_t0__blk808) - (assign20670_e18502 * locals.var_t0__blk808_dn9)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-((locals.var_rgisl_dn10 * locals.var_vgd_eff_1) + (locals.var_rgisl * locals.var_vgd_eff_1_dn10))) - locals.var_egisl_dn10) * locals.var_t0__blk808) - (assign20670_e18502 * locals.var_t0__blk808_dn10)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-((locals.var_rgisl_dn11 * locals.var_vgd_eff_1) + (locals.var_rgisl * locals.var_vgd_eff_1_dn11))) - locals.var_egisl_dn11) * locals.var_t0__blk808) - (assign20670_e18502 * locals.var_t0__blk808_dn11)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-((locals.var_rgisl_dn12 * locals.var_vgd_eff_1) + (locals.var_rgisl * locals.var_vgd_eff_1_dn12))) - locals.var_egisl_dn12) * locals.var_t0__blk808) - (assign20670_e18502 * locals.var_t0__blk808_dn12)) / (locals.var_t0__blk808 * locals.var_t0__blk808)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign20670_e18506;
        locals.var_t1__blk809_dn3 = assign20670_e18506_d_n3;
        locals.var_t1__blk809_dn4 = assign20670_e18506_d_n4;
        locals.var_t1__blk809_dn5 = assign20670_e18506_d_n5;
        locals.var_t1__blk809_dn6 = assign20670_e18506_d_n6;
        locals.var_t1__blk809_dn7 = assign20670_e18506_d_n7;
        locals.var_t1__blk809_dn8 = assign20670_e18506_d_n8;
        locals.var_t1__blk809_dn9 = assign20670_e18506_d_n9;
        locals.var_t1__blk809_dn10 = assign20670_e18506_d_n10;
        locals.var_t1__blk809_dn11 = assign20670_e18506_d_n11;
        locals.var_t1__blk809_dn12 = assign20670_e18506_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign20680_e18527, assign20680_e18527_d_n3, assign20680_e18527_d_n4, assign20680_e18527_d_n5, assign20680_e18527_d_n6, assign20680_e18527_d_n7, assign20680_e18527_d_n8, assign20680_e18527_d_n9, assign20680_e18527_d_n10, assign20680_e18527_d_n11, assign20680_e18527_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 == 0.0)) && (locals.var_guard1247 == 0.0)) {
        let assign20680_e18515: f64 = (-locals.var_vds_1);
        let assign20680_e18518: f64 = (locals.var_rgisl * locals.var_vgd_eff_1);
        let assign20680_e18519: f64 = (assign20680_e18515 - assign20680_e18518);
        let assign20680_e18521: f64 = (assign20680_e18519 - locals.var_egisl);
        let assign20680_e18523: f64 = (assign20680_e18521 + locals.var_pparam_b4soivfbsd);
        let assign20680_e18525: f64 = (assign20680_e18523 / locals.var_t0__blk808);
        (assign20680_e18525, ((((((-((locals.var_rgisl_dn3 * locals.var_vgd_eff_1) + (locals.var_rgisl * locals.var_vgd_eff_1_dn3))) - locals.var_egisl_dn3) + locals.var_pparam_b4soivfbsd_dn3) * locals.var_t0__blk808) - (assign20680_e18523 * locals.var_t0__blk808_dn3)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-((locals.var_rgisl_dn4 * locals.var_vgd_eff_1) + (locals.var_rgisl * locals.var_vgd_eff_1_dn4))) - locals.var_egisl_dn4) + locals.var_pparam_b4soivfbsd_dn4) * locals.var_t0__blk808) - (assign20680_e18523 * locals.var_t0__blk808_dn4)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-((locals.var_rgisl_dn5 * locals.var_vgd_eff_1) + (locals.var_rgisl * locals.var_vgd_eff_1_dn5))) - locals.var_egisl_dn5) + locals.var_pparam_b4soivfbsd_dn5) * locals.var_t0__blk808) - (assign20680_e18523 * locals.var_t0__blk808_dn5)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-((locals.var_rgisl_dn6 * locals.var_vgd_eff_1) + (locals.var_rgisl * locals.var_vgd_eff_1_dn6))) - locals.var_egisl_dn6) + locals.var_pparam_b4soivfbsd_dn6) * locals.var_t0__blk808) - (assign20680_e18523 * locals.var_t0__blk808_dn6)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((((-locals.var_vds_1_dn7) - ((locals.var_rgisl_dn7 * locals.var_vgd_eff_1) + (locals.var_rgisl * locals.var_vgd_eff_1_dn7))) - locals.var_egisl_dn7) + locals.var_pparam_b4soivfbsd_dn7) * locals.var_t0__blk808) - (assign20680_e18523 * locals.var_t0__blk808_dn7)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((((-locals.var_vds_1_dn8) - ((locals.var_rgisl_dn8 * locals.var_vgd_eff_1) + (locals.var_rgisl * locals.var_vgd_eff_1_dn8))) - locals.var_egisl_dn8) + locals.var_pparam_b4soivfbsd_dn8) * locals.var_t0__blk808) - (assign20680_e18523 * locals.var_t0__blk808_dn8)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-((locals.var_rgisl_dn9 * locals.var_vgd_eff_1) + (locals.var_rgisl * locals.var_vgd_eff_1_dn9))) - locals.var_egisl_dn9) + locals.var_pparam_b4soivfbsd_dn9) * locals.var_t0__blk808) - (assign20680_e18523 * locals.var_t0__blk808_dn9)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-((locals.var_rgisl_dn10 * locals.var_vgd_eff_1) + (locals.var_rgisl * locals.var_vgd_eff_1_dn10))) - locals.var_egisl_dn10) + locals.var_pparam_b4soivfbsd_dn10) * locals.var_t0__blk808) - (assign20680_e18523 * locals.var_t0__blk808_dn10)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-((locals.var_rgisl_dn11 * locals.var_vgd_eff_1) + (locals.var_rgisl * locals.var_vgd_eff_1_dn11))) - locals.var_egisl_dn11) + locals.var_pparam_b4soivfbsd_dn11) * locals.var_t0__blk808) - (assign20680_e18523 * locals.var_t0__blk808_dn11)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-((locals.var_rgisl_dn12 * locals.var_vgd_eff_1) + (locals.var_rgisl * locals.var_vgd_eff_1_dn12))) - locals.var_egisl_dn12) + locals.var_pparam_b4soivfbsd_dn12) * locals.var_t0__blk808) - (assign20680_e18523 * locals.var_t0__blk808_dn12)) / (locals.var_t0__blk808 * locals.var_t0__blk808)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign20680_e18527;
        locals.var_t1__blk809_dn3 = assign20680_e18527_d_n3;
        locals.var_t1__blk809_dn4 = assign20680_e18527_d_n4;
        locals.var_t1__blk809_dn5 = assign20680_e18527_d_n5;
        locals.var_t1__blk809_dn6 = assign20680_e18527_d_n6;
        locals.var_t1__blk809_dn7 = assign20680_e18527_d_n7;
        locals.var_t1__blk809_dn8 = assign20680_e18527_d_n8;
        locals.var_t1__blk809_dn9 = assign20680_e18527_d_n9;
        locals.var_t1__blk809_dn10 = assign20680_e18527_d_n10;
        locals.var_t1__blk809_dn11 = assign20680_e18527_d_n11;
        locals.var_t1__blk809_dn12 = assign20680_e18527_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let assign20690_e18538: f64 = if (((locals.var_agisl <= 0.0) || (locals.var_bgisl <= 0.0)) || (locals.var_cgisl < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1248 = assign20690_e18538;
        locals.var_guard1248_rv = 0.0;

        let (assign20710_e18570, assign20710_e18570_d_n3, assign20710_e18570_d_n4, assign20710_e18570_d_n5, assign20710_e18570_d_n6, assign20710_e18570_d_n7, assign20710_e18570_d_n8, assign20710_e18570_d_n9, assign20710_e18570_d_n10, assign20710_e18570_d_n11, assign20710_e18570_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 == 0.0)) && (locals.var_guard1248 == 0.0)) {
        let assign20710_e18559: f64 = (locals.var_t1__blk809 * locals.var_t1__blk809);
        let assign20710_e18562: f64 = (4.0 * 0.01);
        let assign20710_e18564: f64 = (assign20710_e18562 * 0.01);
        let assign20710_e18565: f64 = (assign20710_e18559 + assign20710_e18564);
        let assign20710_e18566: f64 = (assign20710_e18565).sqrt();
        let assign20710_e18567: f64 = (locals.var_t1__blk809 + assign20710_e18566);
        let assign20710_e18568: f64 = (0.5 * assign20710_e18567);
        (assign20710_e18568, (0.5 * (locals.var_t1__blk809_dn3 + (((locals.var_t1__blk809_dn3 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn3)) / (2.0 * assign20710_e18566)))), (0.5 * (locals.var_t1__blk809_dn4 + (((locals.var_t1__blk809_dn4 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn4)) / (2.0 * assign20710_e18566)))), (0.5 * (locals.var_t1__blk809_dn5 + (((locals.var_t1__blk809_dn5 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn5)) / (2.0 * assign20710_e18566)))), (0.5 * (locals.var_t1__blk809_dn6 + (((locals.var_t1__blk809_dn6 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn6)) / (2.0 * assign20710_e18566)))), (0.5 * (locals.var_t1__blk809_dn7 + (((locals.var_t1__blk809_dn7 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn7)) / (2.0 * assign20710_e18566)))), (0.5 * (locals.var_t1__blk809_dn8 + (((locals.var_t1__blk809_dn8 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn8)) / (2.0 * assign20710_e18566)))), (0.5 * (locals.var_t1__blk809_dn9 + (((locals.var_t1__blk809_dn9 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn9)) / (2.0 * assign20710_e18566)))), (0.5 * (locals.var_t1__blk809_dn10 + (((locals.var_t1__blk809_dn10 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn10)) / (2.0 * assign20710_e18566)))), (0.5 * (locals.var_t1__blk809_dn11 + (((locals.var_t1__blk809_dn11 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn11)) / (2.0 * assign20710_e18566)))), (0.5 * (locals.var_t1__blk809_dn12 + (((locals.var_t1__blk809_dn12 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn12)) / (2.0 * assign20710_e18566)))),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign20710_e18570;
        locals.var_t1__blk809_dn3 = assign20710_e18570_d_n3;
        locals.var_t1__blk809_dn4 = assign20710_e18570_d_n4;
        locals.var_t1__blk809_dn5 = assign20710_e18570_d_n5;
        locals.var_t1__blk809_dn6 = assign20710_e18570_d_n6;
        locals.var_t1__blk809_dn7 = assign20710_e18570_d_n7;
        locals.var_t1__blk809_dn8 = assign20710_e18570_d_n8;
        locals.var_t1__blk809_dn9 = assign20710_e18570_d_n9;
        locals.var_t1__blk809_dn10 = assign20710_e18570_d_n10;
        locals.var_t1__blk809_dn11 = assign20710_e18570_d_n11;
        locals.var_t1__blk809_dn12 = assign20710_e18570_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign20720_e18584, assign20720_e18584_d_n3, assign20720_e18584_d_n4, assign20720_e18584_d_n5, assign20720_e18584_d_n6, assign20720_e18584_d_n7, assign20720_e18584_d_n8, assign20720_e18584_d_n9, assign20720_e18584_d_n10, assign20720_e18584_d_n11, assign20720_e18584_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 == 0.0)) && (locals.var_guard1248 == 0.0)) {
        let assign20720_e18581: f64 = (locals.var_t1__blk809 + 0.001);
        let assign20720_e18582: f64 = (locals.var_bgisl / assign20720_e18581);
        (assign20720_e18582, (((locals.var_bgisl_dn3 * assign20720_e18581) - (locals.var_bgisl * locals.var_t1__blk809_dn3)) / (assign20720_e18581 * assign20720_e18581)), (((locals.var_bgisl_dn4 * assign20720_e18581) - (locals.var_bgisl * locals.var_t1__blk809_dn4)) / (assign20720_e18581 * assign20720_e18581)), (((locals.var_bgisl_dn5 * assign20720_e18581) - (locals.var_bgisl * locals.var_t1__blk809_dn5)) / (assign20720_e18581 * assign20720_e18581)), (((locals.var_bgisl_dn6 * assign20720_e18581) - (locals.var_bgisl * locals.var_t1__blk809_dn6)) / (assign20720_e18581 * assign20720_e18581)), (((locals.var_bgisl_dn7 * assign20720_e18581) - (locals.var_bgisl * locals.var_t1__blk809_dn7)) / (assign20720_e18581 * assign20720_e18581)), (((locals.var_bgisl_dn8 * assign20720_e18581) - (locals.var_bgisl * locals.var_t1__blk809_dn8)) / (assign20720_e18581 * assign20720_e18581)), (((locals.var_bgisl_dn9 * assign20720_e18581) - (locals.var_bgisl * locals.var_t1__blk809_dn9)) / (assign20720_e18581 * assign20720_e18581)), (((locals.var_bgisl_dn10 * assign20720_e18581) - (locals.var_bgisl * locals.var_t1__blk809_dn10)) / (assign20720_e18581 * assign20720_e18581)), (((locals.var_bgisl_dn11 * assign20720_e18581) - (locals.var_bgisl * locals.var_t1__blk809_dn11)) / (assign20720_e18581 * assign20720_e18581)), (((locals.var_bgisl_dn12 * assign20720_e18581) - (locals.var_bgisl * locals.var_t1__blk809_dn12)) / (assign20720_e18581 * assign20720_e18581)),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign20720_e18584;
        locals.var_t2__blk810_dn3 = assign20720_e18584_d_n3;
        locals.var_t2__blk810_dn4 = assign20720_e18584_d_n4;
        locals.var_t2__blk810_dn5 = assign20720_e18584_d_n5;
        locals.var_t2__blk810_dn6 = assign20720_e18584_d_n6;
        locals.var_t2__blk810_dn7 = assign20720_e18584_d_n7;
        locals.var_t2__blk810_dn8 = assign20720_e18584_d_n8;
        locals.var_t2__blk810_dn9 = assign20720_e18584_d_n9;
        locals.var_t2__blk810_dn10 = assign20720_e18584_d_n10;
        locals.var_t2__blk810_dn11 = assign20720_e18584_d_n11;
        locals.var_t2__blk810_dn12 = assign20720_e18584_d_n12;
        locals.var_t2__blk810_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_63(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20740_e18614, assign20740_e18614_d_n3, assign20740_e18614_d_n4, assign20740_e18614_d_n5, assign20740_e18614_d_n6, assign20740_e18614_d_n7, assign20740_e18614_d_n8, assign20740_e18614_d_n9, assign20740_e18614_d_n10, assign20740_e18614_d_n11, assign20740_e18614_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 == 0.0)) && (locals.var_guard1248 == 0.0)) {
        let assign20740_e18612: f64 = (locals.var_vbs_1 - locals.var_fgisl);
        (assign20740_e18612, (locals.var_vbs_1_dn3 - locals.var_fgisl_dn3), (locals.var_vbs_1_dn4 - locals.var_fgisl_dn4), (locals.var_vbs_1_dn5 - locals.var_fgisl_dn5), (locals.var_vbs_1_dn6 - locals.var_fgisl_dn6), (locals.var_vbs_1_dn7 - locals.var_fgisl_dn7), (locals.var_vbs_1_dn8 - locals.var_fgisl_dn8), (locals.var_vbs_1_dn9 - locals.var_fgisl_dn9), (locals.var_vbs_1_dn10 - locals.var_fgisl_dn10), (locals.var_vbs_1_dn11 - locals.var_fgisl_dn11), (locals.var_vbs_1_dn12 - locals.var_fgisl_dn12),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign20740_e18614;
        locals.var_t4__blk812_dn3 = assign20740_e18614_d_n3;
        locals.var_t4__blk812_dn4 = assign20740_e18614_d_n4;
        locals.var_t4__blk812_dn5 = assign20740_e18614_d_n5;
        locals.var_t4__blk812_dn6 = assign20740_e18614_d_n6;
        locals.var_t4__blk812_dn7 = assign20740_e18614_d_n7;
        locals.var_t4__blk812_dn8 = assign20740_e18614_d_n8;
        locals.var_t4__blk812_dn9 = assign20740_e18614_d_n9;
        locals.var_t4__blk812_dn10 = assign20740_e18614_d_n10;
        locals.var_t4__blk812_dn11 = assign20740_e18614_d_n11;
        locals.var_t4__blk812_dn12 = assign20740_e18614_d_n12;
        locals.var_t4__blk812_rv = 0.0;

        let assign20750_e18617: f64 = (-1.0);
        let assign20750_e18619: f64 = (assign20750_e18617 / 100.0);
        let assign20750_e18620: f64 = if locals.var_t4__blk812 >= assign20750_e18619 { 1.0 } else { 0.0 };
        locals.var_guard1249 = assign20750_e18620;
        locals.var_guard1249_rv = 0.0;

        let (assign20760_e18635, assign20760_e18635_d_n3, assign20760_e18635_d_n4, assign20760_e18635_d_n5, assign20760_e18635_d_n6, assign20760_e18635_d_n7, assign20760_e18635_d_n8, assign20760_e18635_d_n9, assign20760_e18635_d_n10, assign20760_e18635_d_n11, assign20760_e18635_d_n12,) = {
    if ((((locals.var_guard1240 != 0.0) && (locals.var_guard1242 == 0.0)) && (locals.var_guard1248 == 0.0)) && (locals.var_guard1249 != 0.0)) {
        let assign20760_e18631: f64 = (-locals.var_kgisl);
        let assign20760_e18633: f64 = (assign20760_e18631 * 100.0);
        (assign20760_e18633, ((-locals.var_kgisl_dn3) * 100.0), ((-locals.var_kgisl_dn4) * 100.0), ((-locals.var_kgisl_dn5) * 100.0), ((-locals.var_kgisl_dn6) * 100.0), ((-locals.var_kgisl_dn7) * 100.0), ((-locals.var_kgisl_dn8) * 100.0), ((-locals.var_kgisl_dn9) * 100.0), ((-locals.var_kgisl_dn10) * 100.0), ((-locals.var_kgisl_dn11) * 100.0), ((-locals.var_kgisl_dn12) * 100.0),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign20760_e18635;
        locals.var_t5__blk813_dn3 = assign20760_e18635_d_n3;
        locals.var_t5__blk813_dn4 = assign20760_e18635_d_n4;
        locals.var_t5__blk813_dn5 = assign20760_e18635_d_n5;
        locals.var_t5__blk813_dn6 = assign20760_e18635_d_n6;
        locals.var_t5__blk813_dn7 = assign20760_e18635_d_n7;
        locals.var_t5__blk813_dn8 = assign20760_e18635_d_n8;
        locals.var_t5__blk813_dn9 = assign20760_e18635_d_n9;
        locals.var_t5__blk813_dn10 = assign20760_e18635_d_n10;
        locals.var_t5__blk813_dn11 = assign20760_e18635_d_n11;
        locals.var_t5__blk813_dn12 = assign20760_e18635_d_n12;
        locals.var_t5__blk813_rv = 0.0;

        let (assign20770_e18650, assign20770_e18650_d_n3, assign20770_e18650_d_n4, assign20770_e18650_d_n5, assign20770_e18650_d_n6, assign20770_e18650_d_n7, assign20770_e18650_d_n8, assign20770_e18650_d_n9, assign20770_e18650_d_n10, assign20770_e18650_d_n11, assign20770_e18650_d_n12,) = {
    if ((((locals.var_guard1240 != 0.0) && (locals.var_guard1242 == 0.0)) && (locals.var_guard1248 == 0.0)) && (locals.var_guard1249 == 0.0)) {
        let assign20770_e18648: f64 = (locals.var_kgisl / locals.var_t4__blk812);
        (assign20770_e18648, (((locals.var_kgisl_dn3 * locals.var_t4__blk812) - (locals.var_kgisl * locals.var_t4__blk812_dn3)) / (locals.var_t4__blk812 * locals.var_t4__blk812)), (((locals.var_kgisl_dn4 * locals.var_t4__blk812) - (locals.var_kgisl * locals.var_t4__blk812_dn4)) / (locals.var_t4__blk812 * locals.var_t4__blk812)), (((locals.var_kgisl_dn5 * locals.var_t4__blk812) - (locals.var_kgisl * locals.var_t4__blk812_dn5)) / (locals.var_t4__blk812 * locals.var_t4__blk812)), (((locals.var_kgisl_dn6 * locals.var_t4__blk812) - (locals.var_kgisl * locals.var_t4__blk812_dn6)) / (locals.var_t4__blk812 * locals.var_t4__blk812)), (((locals.var_kgisl_dn7 * locals.var_t4__blk812) - (locals.var_kgisl * locals.var_t4__blk812_dn7)) / (locals.var_t4__blk812 * locals.var_t4__blk812)), (((locals.var_kgisl_dn8 * locals.var_t4__blk812) - (locals.var_kgisl * locals.var_t4__blk812_dn8)) / (locals.var_t4__blk812 * locals.var_t4__blk812)), (((locals.var_kgisl_dn9 * locals.var_t4__blk812) - (locals.var_kgisl * locals.var_t4__blk812_dn9)) / (locals.var_t4__blk812 * locals.var_t4__blk812)), (((locals.var_kgisl_dn10 * locals.var_t4__blk812) - (locals.var_kgisl * locals.var_t4__blk812_dn10)) / (locals.var_t4__blk812 * locals.var_t4__blk812)), (((locals.var_kgisl_dn11 * locals.var_t4__blk812) - (locals.var_kgisl * locals.var_t4__blk812_dn11)) / (locals.var_t4__blk812 * locals.var_t4__blk812)), (((locals.var_kgisl_dn12 * locals.var_t4__blk812) - (locals.var_kgisl * locals.var_t4__blk812_dn12)) / (locals.var_t4__blk812 * locals.var_t4__blk812)),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign20770_e18650;
        locals.var_t5__blk813_dn3 = assign20770_e18650_d_n3;
        locals.var_t5__blk813_dn4 = assign20770_e18650_d_n4;
        locals.var_t5__blk813_dn5 = assign20770_e18650_d_n5;
        locals.var_t5__blk813_dn6 = assign20770_e18650_d_n6;
        locals.var_t5__blk813_dn7 = assign20770_e18650_d_n7;
        locals.var_t5__blk813_dn8 = assign20770_e18650_d_n8;
        locals.var_t5__blk813_dn9 = assign20770_e18650_d_n9;
        locals.var_t5__blk813_dn10 = assign20770_e18650_d_n10;
        locals.var_t5__blk813_dn11 = assign20770_e18650_d_n11;
        locals.var_t5__blk813_dn12 = assign20770_e18650_d_n12;
        locals.var_t5__blk813_rv = 0.0;

        let (assign20780_e18661, assign20780_e18661_d_n3, assign20780_e18661_d_n4, assign20780_e18661_d_n5, assign20780_e18661_d_n6, assign20780_e18661_d_n7, assign20780_e18661_d_n8, assign20780_e18661_d_n9, assign20780_e18661_d_n10, assign20780_e18661_d_n11, assign20780_e18661_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 == 0.0)) && (locals.var_guard1248 == 0.0)) {
        let assign20780_e18659: f64 = (locals.var_t5__blk813).exp();
        (assign20780_e18659, (assign20780_e18659 * locals.var_t5__blk813_dn3), (assign20780_e18659 * locals.var_t5__blk813_dn4), (assign20780_e18659 * locals.var_t5__blk813_dn5), (assign20780_e18659 * locals.var_t5__blk813_dn6), (assign20780_e18659 * locals.var_t5__blk813_dn7), (assign20780_e18659 * locals.var_t5__blk813_dn8), (assign20780_e18659 * locals.var_t5__blk813_dn9), (assign20780_e18659 * locals.var_t5__blk813_dn10), (assign20780_e18659 * locals.var_t5__blk813_dn11), (assign20780_e18659 * locals.var_t5__blk813_dn12),)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign20780_e18661;
        locals.var_t6__blk814_dn3 = assign20780_e18661_d_n3;
        locals.var_t6__blk814_dn4 = assign20780_e18661_d_n4;
        locals.var_t6__blk814_dn5 = assign20780_e18661_d_n5;
        locals.var_t6__blk814_dn6 = assign20780_e18661_d_n6;
        locals.var_t6__blk814_dn7 = assign20780_e18661_d_n7;
        locals.var_t6__blk814_dn8 = assign20780_e18661_d_n8;
        locals.var_t6__blk814_dn9 = assign20780_e18661_d_n9;
        locals.var_t6__blk814_dn10 = assign20780_e18661_d_n10;
        locals.var_t6__blk814_dn11 = assign20780_e18661_d_n11;
        locals.var_t6__blk814_dn12 = assign20780_e18661_d_n12;
        locals.var_t6__blk814_rv = 0.0;

        let assign20800_e18676: f64 = if p.p41 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1250 = assign20800_e18676;
        locals.var_guard1250_rv = 0.0;

        let (assign20810_e18693, assign20810_e18693_d_n3, assign20810_e18693_d_n4, assign20810_e18693_d_n5, assign20810_e18693_d_n6, assign20810_e18693_d_n7, assign20810_e18693_d_n8, assign20810_e18693_d_n9, assign20810_e18693_d_n10, assign20810_e18693_d_n11, assign20810_e18693_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 == 0.0)) && (locals.var_guard1250 != 0.0)) {
        let assign20810_e18686: f64 = (locals.var_rgidl * locals.var_vgs_eff__blk790);
        let assign20810_e18687: f64 = (locals.var_vds_1 - assign20810_e18686);
        let assign20810_e18689: f64 = (assign20810_e18687 - locals.var_egidl);
        let assign20810_e18691: f64 = (assign20810_e18689 / locals.var_t0__blk808);
        (assign20810_e18691, (((((-((locals.var_rgidl_dn3 * locals.var_vgs_eff__blk790) + (locals.var_rgidl * locals.var_vgs_eff__blk790_dn3))) - locals.var_egidl_dn3) * locals.var_t0__blk808) - (assign20810_e18689 * locals.var_t0__blk808_dn3)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-((locals.var_rgidl_dn4 * locals.var_vgs_eff__blk790) + (locals.var_rgidl * locals.var_vgs_eff__blk790_dn4))) - locals.var_egidl_dn4) * locals.var_t0__blk808) - (assign20810_e18689 * locals.var_t0__blk808_dn4)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-((locals.var_rgidl_dn5 * locals.var_vgs_eff__blk790) + (locals.var_rgidl * locals.var_vgs_eff__blk790_dn5))) - locals.var_egidl_dn5) * locals.var_t0__blk808) - (assign20810_e18689 * locals.var_t0__blk808_dn5)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-((locals.var_rgidl_dn6 * locals.var_vgs_eff__blk790) + (locals.var_rgidl * locals.var_vgs_eff__blk790_dn6))) - locals.var_egidl_dn6) * locals.var_t0__blk808) - (assign20810_e18689 * locals.var_t0__blk808_dn6)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((locals.var_vds_1_dn7 - ((locals.var_rgidl_dn7 * locals.var_vgs_eff__blk790) + (locals.var_rgidl * locals.var_vgs_eff__blk790_dn7))) - locals.var_egidl_dn7) * locals.var_t0__blk808) - (assign20810_e18689 * locals.var_t0__blk808_dn7)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((locals.var_vds_1_dn8 - ((locals.var_rgidl_dn8 * locals.var_vgs_eff__blk790) + (locals.var_rgidl * locals.var_vgs_eff__blk790_dn8))) - locals.var_egidl_dn8) * locals.var_t0__blk808) - (assign20810_e18689 * locals.var_t0__blk808_dn8)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-((locals.var_rgidl_dn9 * locals.var_vgs_eff__blk790) + (locals.var_rgidl * locals.var_vgs_eff__blk790_dn9))) - locals.var_egidl_dn9) * locals.var_t0__blk808) - (assign20810_e18689 * locals.var_t0__blk808_dn9)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-((locals.var_rgidl_dn10 * locals.var_vgs_eff__blk790) + (locals.var_rgidl * locals.var_vgs_eff__blk790_dn10))) - locals.var_egidl_dn10) * locals.var_t0__blk808) - (assign20810_e18689 * locals.var_t0__blk808_dn10)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-((locals.var_rgidl_dn11 * locals.var_vgs_eff__blk790) + (locals.var_rgidl * locals.var_vgs_eff__blk790_dn11))) - locals.var_egidl_dn11) * locals.var_t0__blk808) - (assign20810_e18689 * locals.var_t0__blk808_dn11)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((-((locals.var_rgidl_dn12 * locals.var_vgs_eff__blk790) + (locals.var_rgidl * locals.var_vgs_eff__blk790_dn12))) - locals.var_egidl_dn12) * locals.var_t0__blk808) - (assign20810_e18689 * locals.var_t0__blk808_dn12)) / (locals.var_t0__blk808 * locals.var_t0__blk808)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign20810_e18693;
        locals.var_t1__blk809_dn3 = assign20810_e18693_d_n3;
        locals.var_t1__blk809_dn4 = assign20810_e18693_d_n4;
        locals.var_t1__blk809_dn5 = assign20810_e18693_d_n5;
        locals.var_t1__blk809_dn6 = assign20810_e18693_d_n6;
        locals.var_t1__blk809_dn7 = assign20810_e18693_d_n7;
        locals.var_t1__blk809_dn8 = assign20810_e18693_d_n8;
        locals.var_t1__blk809_dn9 = assign20810_e18693_d_n9;
        locals.var_t1__blk809_dn10 = assign20810_e18693_d_n10;
        locals.var_t1__blk809_dn11 = assign20810_e18693_d_n11;
        locals.var_t1__blk809_dn12 = assign20810_e18693_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign20820_e18713, assign20820_e18713_d_n3, assign20820_e18713_d_n4, assign20820_e18713_d_n5, assign20820_e18713_d_n6, assign20820_e18713_d_n7, assign20820_e18713_d_n8, assign20820_e18713_d_n9, assign20820_e18713_d_n10, assign20820_e18713_d_n11, assign20820_e18713_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 == 0.0)) && (locals.var_guard1250 == 0.0)) {
        let assign20820_e18704: f64 = (locals.var_rgidl * locals.var_vgs_eff__blk790);
        let assign20820_e18705: f64 = (locals.var_vds_1 - assign20820_e18704);
        let assign20820_e18707: f64 = (assign20820_e18705 - locals.var_egidl);
        let assign20820_e18709: f64 = (assign20820_e18707 + locals.var_pparam_b4soivfbsd);
        let assign20820_e18711: f64 = (assign20820_e18709 / locals.var_t0__blk808);
        (assign20820_e18711, ((((((-((locals.var_rgidl_dn3 * locals.var_vgs_eff__blk790) + (locals.var_rgidl * locals.var_vgs_eff__blk790_dn3))) - locals.var_egidl_dn3) + locals.var_pparam_b4soivfbsd_dn3) * locals.var_t0__blk808) - (assign20820_e18709 * locals.var_t0__blk808_dn3)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-((locals.var_rgidl_dn4 * locals.var_vgs_eff__blk790) + (locals.var_rgidl * locals.var_vgs_eff__blk790_dn4))) - locals.var_egidl_dn4) + locals.var_pparam_b4soivfbsd_dn4) * locals.var_t0__blk808) - (assign20820_e18709 * locals.var_t0__blk808_dn4)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-((locals.var_rgidl_dn5 * locals.var_vgs_eff__blk790) + (locals.var_rgidl * locals.var_vgs_eff__blk790_dn5))) - locals.var_egidl_dn5) + locals.var_pparam_b4soivfbsd_dn5) * locals.var_t0__blk808) - (assign20820_e18709 * locals.var_t0__blk808_dn5)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-((locals.var_rgidl_dn6 * locals.var_vgs_eff__blk790) + (locals.var_rgidl * locals.var_vgs_eff__blk790_dn6))) - locals.var_egidl_dn6) + locals.var_pparam_b4soivfbsd_dn6) * locals.var_t0__blk808) - (assign20820_e18709 * locals.var_t0__blk808_dn6)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((locals.var_vds_1_dn7 - ((locals.var_rgidl_dn7 * locals.var_vgs_eff__blk790) + (locals.var_rgidl * locals.var_vgs_eff__blk790_dn7))) - locals.var_egidl_dn7) + locals.var_pparam_b4soivfbsd_dn7) * locals.var_t0__blk808) - (assign20820_e18709 * locals.var_t0__blk808_dn7)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((locals.var_vds_1_dn8 - ((locals.var_rgidl_dn8 * locals.var_vgs_eff__blk790) + (locals.var_rgidl * locals.var_vgs_eff__blk790_dn8))) - locals.var_egidl_dn8) + locals.var_pparam_b4soivfbsd_dn8) * locals.var_t0__blk808) - (assign20820_e18709 * locals.var_t0__blk808_dn8)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-((locals.var_rgidl_dn9 * locals.var_vgs_eff__blk790) + (locals.var_rgidl * locals.var_vgs_eff__blk790_dn9))) - locals.var_egidl_dn9) + locals.var_pparam_b4soivfbsd_dn9) * locals.var_t0__blk808) - (assign20820_e18709 * locals.var_t0__blk808_dn9)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-((locals.var_rgidl_dn10 * locals.var_vgs_eff__blk790) + (locals.var_rgidl * locals.var_vgs_eff__blk790_dn10))) - locals.var_egidl_dn10) + locals.var_pparam_b4soivfbsd_dn10) * locals.var_t0__blk808) - (assign20820_e18709 * locals.var_t0__blk808_dn10)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-((locals.var_rgidl_dn11 * locals.var_vgs_eff__blk790) + (locals.var_rgidl * locals.var_vgs_eff__blk790_dn11))) - locals.var_egidl_dn11) + locals.var_pparam_b4soivfbsd_dn11) * locals.var_t0__blk808) - (assign20820_e18709 * locals.var_t0__blk808_dn11)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((((-((locals.var_rgidl_dn12 * locals.var_vgs_eff__blk790) + (locals.var_rgidl * locals.var_vgs_eff__blk790_dn12))) - locals.var_egidl_dn12) + locals.var_pparam_b4soivfbsd_dn12) * locals.var_t0__blk808) - (assign20820_e18709 * locals.var_t0__blk808_dn12)) / (locals.var_t0__blk808 * locals.var_t0__blk808)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign20820_e18713;
        locals.var_t1__blk809_dn3 = assign20820_e18713_d_n3;
        locals.var_t1__blk809_dn4 = assign20820_e18713_d_n4;
        locals.var_t1__blk809_dn5 = assign20820_e18713_d_n5;
        locals.var_t1__blk809_dn6 = assign20820_e18713_d_n6;
        locals.var_t1__blk809_dn7 = assign20820_e18713_d_n7;
        locals.var_t1__blk809_dn8 = assign20820_e18713_d_n8;
        locals.var_t1__blk809_dn9 = assign20820_e18713_d_n9;
        locals.var_t1__blk809_dn10 = assign20820_e18713_d_n10;
        locals.var_t1__blk809_dn11 = assign20820_e18713_d_n11;
        locals.var_t1__blk809_dn12 = assign20820_e18713_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let assign20830_e18724: f64 = if (((locals.var_agidl <= 0.0) || (locals.var_bgidl <= 0.0)) || (locals.var_cgidl < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1251 = assign20830_e18724;
        locals.var_guard1251_rv = 0.0;

        let (assign20850_e18756, assign20850_e18756_d_n3, assign20850_e18756_d_n4, assign20850_e18756_d_n5, assign20850_e18756_d_n6, assign20850_e18756_d_n7, assign20850_e18756_d_n8, assign20850_e18756_d_n9, assign20850_e18756_d_n10, assign20850_e18756_d_n11, assign20850_e18756_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 == 0.0)) && (locals.var_guard1251 == 0.0)) {
        let assign20850_e18745: f64 = (locals.var_t1__blk809 * locals.var_t1__blk809);
        let assign20850_e18748: f64 = (4.0 * 0.01);
        let assign20850_e18750: f64 = (assign20850_e18748 * 0.01);
        let assign20850_e18751: f64 = (assign20850_e18745 + assign20850_e18750);
        let assign20850_e18752: f64 = (assign20850_e18751).sqrt();
        let assign20850_e18753: f64 = (locals.var_t1__blk809 + assign20850_e18752);
        let assign20850_e18754: f64 = (0.5 * assign20850_e18753);
        (assign20850_e18754, (0.5 * (locals.var_t1__blk809_dn3 + (((locals.var_t1__blk809_dn3 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn3)) / (2.0 * assign20850_e18752)))), (0.5 * (locals.var_t1__blk809_dn4 + (((locals.var_t1__blk809_dn4 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn4)) / (2.0 * assign20850_e18752)))), (0.5 * (locals.var_t1__blk809_dn5 + (((locals.var_t1__blk809_dn5 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn5)) / (2.0 * assign20850_e18752)))), (0.5 * (locals.var_t1__blk809_dn6 + (((locals.var_t1__blk809_dn6 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn6)) / (2.0 * assign20850_e18752)))), (0.5 * (locals.var_t1__blk809_dn7 + (((locals.var_t1__blk809_dn7 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn7)) / (2.0 * assign20850_e18752)))), (0.5 * (locals.var_t1__blk809_dn8 + (((locals.var_t1__blk809_dn8 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn8)) / (2.0 * assign20850_e18752)))), (0.5 * (locals.var_t1__blk809_dn9 + (((locals.var_t1__blk809_dn9 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn9)) / (2.0 * assign20850_e18752)))), (0.5 * (locals.var_t1__blk809_dn10 + (((locals.var_t1__blk809_dn10 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn10)) / (2.0 * assign20850_e18752)))), (0.5 * (locals.var_t1__blk809_dn11 + (((locals.var_t1__blk809_dn11 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn11)) / (2.0 * assign20850_e18752)))), (0.5 * (locals.var_t1__blk809_dn12 + (((locals.var_t1__blk809_dn12 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn12)) / (2.0 * assign20850_e18752)))),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign20850_e18756;
        locals.var_t1__blk809_dn3 = assign20850_e18756_d_n3;
        locals.var_t1__blk809_dn4 = assign20850_e18756_d_n4;
        locals.var_t1__blk809_dn5 = assign20850_e18756_d_n5;
        locals.var_t1__blk809_dn6 = assign20850_e18756_d_n6;
        locals.var_t1__blk809_dn7 = assign20850_e18756_d_n7;
        locals.var_t1__blk809_dn8 = assign20850_e18756_d_n8;
        locals.var_t1__blk809_dn9 = assign20850_e18756_d_n9;
        locals.var_t1__blk809_dn10 = assign20850_e18756_d_n10;
        locals.var_t1__blk809_dn11 = assign20850_e18756_d_n11;
        locals.var_t1__blk809_dn12 = assign20850_e18756_d_n12;
        locals.var_t1__blk809_rv = 0.0;

        let (assign20860_e18770, assign20860_e18770_d_n3, assign20860_e18770_d_n4, assign20860_e18770_d_n5, assign20860_e18770_d_n6, assign20860_e18770_d_n7, assign20860_e18770_d_n8, assign20860_e18770_d_n9, assign20860_e18770_d_n10, assign20860_e18770_d_n11, assign20860_e18770_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 == 0.0)) && (locals.var_guard1251 == 0.0)) {
        let assign20860_e18767: f64 = (locals.var_t1__blk809 + 0.001);
        let assign20860_e18768: f64 = (locals.var_bgidl / assign20860_e18767);
        (assign20860_e18768, (((locals.var_bgidl_dn3 * assign20860_e18767) - (locals.var_bgidl * locals.var_t1__blk809_dn3)) / (assign20860_e18767 * assign20860_e18767)), (((locals.var_bgidl_dn4 * assign20860_e18767) - (locals.var_bgidl * locals.var_t1__blk809_dn4)) / (assign20860_e18767 * assign20860_e18767)), (((locals.var_bgidl_dn5 * assign20860_e18767) - (locals.var_bgidl * locals.var_t1__blk809_dn5)) / (assign20860_e18767 * assign20860_e18767)), (((locals.var_bgidl_dn6 * assign20860_e18767) - (locals.var_bgidl * locals.var_t1__blk809_dn6)) / (assign20860_e18767 * assign20860_e18767)), (((locals.var_bgidl_dn7 * assign20860_e18767) - (locals.var_bgidl * locals.var_t1__blk809_dn7)) / (assign20860_e18767 * assign20860_e18767)), (((locals.var_bgidl_dn8 * assign20860_e18767) - (locals.var_bgidl * locals.var_t1__blk809_dn8)) / (assign20860_e18767 * assign20860_e18767)), (((locals.var_bgidl_dn9 * assign20860_e18767) - (locals.var_bgidl * locals.var_t1__blk809_dn9)) / (assign20860_e18767 * assign20860_e18767)), (((locals.var_bgidl_dn10 * assign20860_e18767) - (locals.var_bgidl * locals.var_t1__blk809_dn10)) / (assign20860_e18767 * assign20860_e18767)), (((locals.var_bgidl_dn11 * assign20860_e18767) - (locals.var_bgidl * locals.var_t1__blk809_dn11)) / (assign20860_e18767 * assign20860_e18767)), (((locals.var_bgidl_dn12 * assign20860_e18767) - (locals.var_bgidl * locals.var_t1__blk809_dn12)) / (assign20860_e18767 * assign20860_e18767)),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign20860_e18770;
        locals.var_t2__blk810_dn3 = assign20860_e18770_d_n3;
        locals.var_t2__blk810_dn4 = assign20860_e18770_d_n4;
        locals.var_t2__blk810_dn5 = assign20860_e18770_d_n5;
        locals.var_t2__blk810_dn6 = assign20860_e18770_d_n6;
        locals.var_t2__blk810_dn7 = assign20860_e18770_d_n7;
        locals.var_t2__blk810_dn8 = assign20860_e18770_d_n8;
        locals.var_t2__blk810_dn9 = assign20860_e18770_d_n9;
        locals.var_t2__blk810_dn10 = assign20860_e18770_d_n10;
        locals.var_t2__blk810_dn11 = assign20860_e18770_d_n11;
        locals.var_t2__blk810_dn12 = assign20860_e18770_d_n12;
        locals.var_t2__blk810_rv = 0.0;

        let (assign20880_e18800, assign20880_e18800_d_n3, assign20880_e18800_d_n4, assign20880_e18800_d_n5, assign20880_e18800_d_n6, assign20880_e18800_d_n7, assign20880_e18800_d_n8, assign20880_e18800_d_n9, assign20880_e18800_d_n10, assign20880_e18800_d_n11, assign20880_e18800_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 == 0.0)) && (locals.var_guard1251 == 0.0)) {
        let assign20880_e18798: f64 = (locals.var_vbd_1 - locals.var_fgidl);
        (assign20880_e18798, (-locals.var_fgidl_dn3), (-locals.var_fgidl_dn4), (locals.var_vbd_1_dn5 - locals.var_fgidl_dn5), (-locals.var_fgidl_dn6), (locals.var_vbd_1_dn7 - locals.var_fgidl_dn7), (locals.var_vbd_1_dn8 - locals.var_fgidl_dn8), (-locals.var_fgidl_dn9), (-locals.var_fgidl_dn10), (-locals.var_fgidl_dn11), (-locals.var_fgidl_dn12),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign20880_e18800;
        locals.var_t4__blk812_dn3 = assign20880_e18800_d_n3;
        locals.var_t4__blk812_dn4 = assign20880_e18800_d_n4;
        locals.var_t4__blk812_dn5 = assign20880_e18800_d_n5;
        locals.var_t4__blk812_dn6 = assign20880_e18800_d_n6;
        locals.var_t4__blk812_dn7 = assign20880_e18800_d_n7;
        locals.var_t4__blk812_dn8 = assign20880_e18800_d_n8;
        locals.var_t4__blk812_dn9 = assign20880_e18800_d_n9;
        locals.var_t4__blk812_dn10 = assign20880_e18800_d_n10;
        locals.var_t4__blk812_dn11 = assign20880_e18800_d_n11;
        locals.var_t4__blk812_dn12 = assign20880_e18800_d_n12;
        locals.var_t4__blk812_rv = 0.0;

        let assign20890_e18803: f64 = (-1.0);
        let assign20890_e18805: f64 = (assign20890_e18803 / 100.0);
        let assign20890_e18806: f64 = if locals.var_t4__blk812 >= assign20890_e18805 { 1.0 } else { 0.0 };
        locals.var_guard1252 = assign20890_e18806;
        locals.var_guard1252_rv = 0.0;

        let (assign20900_e18821, assign20900_e18821_d_n3, assign20900_e18821_d_n4, assign20900_e18821_d_n5, assign20900_e18821_d_n6, assign20900_e18821_d_n7, assign20900_e18821_d_n8, assign20900_e18821_d_n9, assign20900_e18821_d_n10, assign20900_e18821_d_n11, assign20900_e18821_d_n12,) = {
    if ((((locals.var_guard1240 != 0.0) && (locals.var_guard1242 == 0.0)) && (locals.var_guard1251 == 0.0)) && (locals.var_guard1252 != 0.0)) {
        let assign20900_e18817: f64 = (-locals.var_kgidl);
        let assign20900_e18819: f64 = (assign20900_e18817 * 100.0);
        (assign20900_e18819, ((-locals.var_kgidl_dn3) * 100.0), ((-locals.var_kgidl_dn4) * 100.0), ((-locals.var_kgidl_dn5) * 100.0), ((-locals.var_kgidl_dn6) * 100.0), ((-locals.var_kgidl_dn7) * 100.0), ((-locals.var_kgidl_dn8) * 100.0), ((-locals.var_kgidl_dn9) * 100.0), ((-locals.var_kgidl_dn10) * 100.0), ((-locals.var_kgidl_dn11) * 100.0), ((-locals.var_kgidl_dn12) * 100.0),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign20900_e18821;
        locals.var_t5__blk813_dn3 = assign20900_e18821_d_n3;
        locals.var_t5__blk813_dn4 = assign20900_e18821_d_n4;
        locals.var_t5__blk813_dn5 = assign20900_e18821_d_n5;
        locals.var_t5__blk813_dn6 = assign20900_e18821_d_n6;
        locals.var_t5__blk813_dn7 = assign20900_e18821_d_n7;
        locals.var_t5__blk813_dn8 = assign20900_e18821_d_n8;
        locals.var_t5__blk813_dn9 = assign20900_e18821_d_n9;
        locals.var_t5__blk813_dn10 = assign20900_e18821_d_n10;
        locals.var_t5__blk813_dn11 = assign20900_e18821_d_n11;
        locals.var_t5__blk813_dn12 = assign20900_e18821_d_n12;
        locals.var_t5__blk813_rv = 0.0;

        let (assign20910_e18836, assign20910_e18836_d_n3, assign20910_e18836_d_n4, assign20910_e18836_d_n5, assign20910_e18836_d_n6, assign20910_e18836_d_n7, assign20910_e18836_d_n8, assign20910_e18836_d_n9, assign20910_e18836_d_n10, assign20910_e18836_d_n11, assign20910_e18836_d_n12,) = {
    if ((((locals.var_guard1240 != 0.0) && (locals.var_guard1242 == 0.0)) && (locals.var_guard1251 == 0.0)) && (locals.var_guard1252 == 0.0)) {
        let assign20910_e18834: f64 = (locals.var_kgidl / locals.var_t4__blk812);
        (assign20910_e18834, (((locals.var_kgidl_dn3 * locals.var_t4__blk812) - (locals.var_kgidl * locals.var_t4__blk812_dn3)) / (locals.var_t4__blk812 * locals.var_t4__blk812)), (((locals.var_kgidl_dn4 * locals.var_t4__blk812) - (locals.var_kgidl * locals.var_t4__blk812_dn4)) / (locals.var_t4__blk812 * locals.var_t4__blk812)), (((locals.var_kgidl_dn5 * locals.var_t4__blk812) - (locals.var_kgidl * locals.var_t4__blk812_dn5)) / (locals.var_t4__blk812 * locals.var_t4__blk812)), (((locals.var_kgidl_dn6 * locals.var_t4__blk812) - (locals.var_kgidl * locals.var_t4__blk812_dn6)) / (locals.var_t4__blk812 * locals.var_t4__blk812)), (((locals.var_kgidl_dn7 * locals.var_t4__blk812) - (locals.var_kgidl * locals.var_t4__blk812_dn7)) / (locals.var_t4__blk812 * locals.var_t4__blk812)), (((locals.var_kgidl_dn8 * locals.var_t4__blk812) - (locals.var_kgidl * locals.var_t4__blk812_dn8)) / (locals.var_t4__blk812 * locals.var_t4__blk812)), (((locals.var_kgidl_dn9 * locals.var_t4__blk812) - (locals.var_kgidl * locals.var_t4__blk812_dn9)) / (locals.var_t4__blk812 * locals.var_t4__blk812)), (((locals.var_kgidl_dn10 * locals.var_t4__blk812) - (locals.var_kgidl * locals.var_t4__blk812_dn10)) / (locals.var_t4__blk812 * locals.var_t4__blk812)), (((locals.var_kgidl_dn11 * locals.var_t4__blk812) - (locals.var_kgidl * locals.var_t4__blk812_dn11)) / (locals.var_t4__blk812 * locals.var_t4__blk812)), (((locals.var_kgidl_dn12 * locals.var_t4__blk812) - (locals.var_kgidl * locals.var_t4__blk812_dn12)) / (locals.var_t4__blk812 * locals.var_t4__blk812)),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign20910_e18836;
        locals.var_t5__blk813_dn3 = assign20910_e18836_d_n3;
        locals.var_t5__blk813_dn4 = assign20910_e18836_d_n4;
        locals.var_t5__blk813_dn5 = assign20910_e18836_d_n5;
        locals.var_t5__blk813_dn6 = assign20910_e18836_d_n6;
        locals.var_t5__blk813_dn7 = assign20910_e18836_d_n7;
        locals.var_t5__blk813_dn8 = assign20910_e18836_d_n8;
        locals.var_t5__blk813_dn9 = assign20910_e18836_d_n9;
        locals.var_t5__blk813_dn10 = assign20910_e18836_d_n10;
        locals.var_t5__blk813_dn11 = assign20910_e18836_d_n11;
        locals.var_t5__blk813_dn12 = assign20910_e18836_d_n12;
        locals.var_t5__blk813_rv = 0.0;

        let (assign20920_e18847, assign20920_e18847_d_n3, assign20920_e18847_d_n4, assign20920_e18847_d_n5, assign20920_e18847_d_n6, assign20920_e18847_d_n7, assign20920_e18847_d_n8, assign20920_e18847_d_n9, assign20920_e18847_d_n10, assign20920_e18847_d_n11, assign20920_e18847_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1242 == 0.0)) && (locals.var_guard1251 == 0.0)) {
        let assign20920_e18845: f64 = (locals.var_t5__blk813).exp();
        (assign20920_e18845, (assign20920_e18845 * locals.var_t5__blk813_dn3), (assign20920_e18845 * locals.var_t5__blk813_dn4), (assign20920_e18845 * locals.var_t5__blk813_dn5), (assign20920_e18845 * locals.var_t5__blk813_dn6), (assign20920_e18845 * locals.var_t5__blk813_dn7), (assign20920_e18845 * locals.var_t5__blk813_dn8), (assign20920_e18845 * locals.var_t5__blk813_dn9), (assign20920_e18845 * locals.var_t5__blk813_dn10), (assign20920_e18845 * locals.var_t5__blk813_dn11), (assign20920_e18845 * locals.var_t5__blk813_dn12),)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign20920_e18847;
        locals.var_t6__blk814_dn3 = assign20920_e18847_d_n3;
        locals.var_t6__blk814_dn4 = assign20920_e18847_d_n4;
        locals.var_t6__blk814_dn5 = assign20920_e18847_d_n5;
        locals.var_t6__blk814_dn6 = assign20920_e18847_d_n6;
        locals.var_t6__blk814_dn7 = assign20920_e18847_d_n7;
        locals.var_t6__blk814_dn8 = assign20920_e18847_d_n8;
        locals.var_t6__blk814_dn9 = assign20920_e18847_d_n9;
        locals.var_t6__blk814_dn10 = assign20920_e18847_d_n10;
        locals.var_t6__blk814_dn11 = assign20920_e18847_d_n11;
        locals.var_t6__blk814_dn12 = assign20920_e18847_d_n12;
        locals.var_t6__blk814_rv = 0.0;

        let (assign20940_e18865, assign20940_e18865_d_n3, assign20940_e18865_d_n4, assign20940_e18865_d_n5, assign20940_e18865_d_n6, assign20940_e18865_d_n7, assign20940_e18865_d_n8, assign20940_e18865_d_n9, assign20940_e18865_d_n10, assign20940_e18865_d_n11, assign20940_e18865_d_n12,) = {
    if (locals.var_guard1240 != 0.0) {
        let assign20940_e18863: f64 = (locals.var_pparam_b4soiwdios * p.p155);
        (assign20940_e18863, (locals.var_pparam_b4soiwdios_dn3 * p.p155), (locals.var_pparam_b4soiwdios_dn4 * p.p155), (locals.var_pparam_b4soiwdios_dn5 * p.p155), (locals.var_pparam_b4soiwdios_dn6 * p.p155), (locals.var_pparam_b4soiwdios_dn7 * p.p155), (locals.var_pparam_b4soiwdios_dn8 * p.p155), (locals.var_pparam_b4soiwdios_dn9 * p.p155), (locals.var_pparam_b4soiwdios_dn10 * p.p155), (locals.var_pparam_b4soiwdios_dn11 * p.p155), (locals.var_pparam_b4soiwdios_dn12 * p.p155),)
    } else {
        (locals.var_wstsi, locals.var_wstsi_dn3, locals.var_wstsi_dn4, locals.var_wstsi_dn5, locals.var_wstsi_dn6, locals.var_wstsi_dn7, locals.var_wstsi_dn8, locals.var_wstsi_dn9, locals.var_wstsi_dn10, locals.var_wstsi_dn11, locals.var_wstsi_dn12,)
    }
};
        locals.var_wstsi = assign20940_e18865;
        locals.var_wstsi_dn3 = assign20940_e18865_d_n3;
        locals.var_wstsi_dn4 = assign20940_e18865_d_n4;
        locals.var_wstsi_dn5 = assign20940_e18865_d_n5;
        locals.var_wstsi_dn6 = assign20940_e18865_d_n6;
        locals.var_wstsi_dn7 = assign20940_e18865_d_n7;
        locals.var_wstsi_dn8 = assign20940_e18865_d_n8;
        locals.var_wstsi_dn9 = assign20940_e18865_d_n9;
        locals.var_wstsi_dn10 = assign20940_e18865_d_n10;
        locals.var_wstsi_dn11 = assign20940_e18865_d_n11;
        locals.var_wstsi_dn12 = assign20940_e18865_d_n12;
        locals.var_wstsi_rv = 0.0;

        let (assign20950_e18871, assign20950_e18871_d_n3, assign20950_e18871_d_n4, assign20950_e18871_d_n5, assign20950_e18871_d_n6, assign20950_e18871_d_n7, assign20950_e18871_d_n8, assign20950_e18871_d_n9, assign20950_e18871_d_n10, assign20950_e18871_d_n11, assign20950_e18871_d_n12,) = {
    if (locals.var_guard1240 != 0.0) {
        let assign20950_e18869: f64 = (locals.var_pparam_b4soiwdiod * p.p155);
        (assign20950_e18869, (locals.var_pparam_b4soiwdiod_dn3 * p.p155), (locals.var_pparam_b4soiwdiod_dn4 * p.p155), (locals.var_pparam_b4soiwdiod_dn5 * p.p155), (locals.var_pparam_b4soiwdiod_dn6 * p.p155), (locals.var_pparam_b4soiwdiod_dn7 * p.p155), (locals.var_pparam_b4soiwdiod_dn8 * p.p155), (locals.var_pparam_b4soiwdiod_dn9 * p.p155), (locals.var_pparam_b4soiwdiod_dn10 * p.p155), (locals.var_pparam_b4soiwdiod_dn11 * p.p155), (locals.var_pparam_b4soiwdiod_dn12 * p.p155),)
    } else {
        (locals.var_wdtsi, locals.var_wdtsi_dn3, locals.var_wdtsi_dn4, locals.var_wdtsi_dn5, locals.var_wdtsi_dn6, locals.var_wdtsi_dn7, locals.var_wdtsi_dn8, locals.var_wdtsi_dn9, locals.var_wdtsi_dn10, locals.var_wdtsi_dn11, locals.var_wdtsi_dn12,)
    }
};
        locals.var_wdtsi = assign20950_e18871;
        locals.var_wdtsi_dn3 = assign20950_e18871_d_n3;
        locals.var_wdtsi_dn4 = assign20950_e18871_d_n4;
        locals.var_wdtsi_dn5 = assign20950_e18871_d_n5;
        locals.var_wdtsi_dn6 = assign20950_e18871_d_n6;
        locals.var_wdtsi_dn7 = assign20950_e18871_d_n7;
        locals.var_wdtsi_dn8 = assign20950_e18871_d_n8;
        locals.var_wdtsi_dn9 = assign20950_e18871_d_n9;
        locals.var_wdtsi_dn10 = assign20950_e18871_d_n10;
        locals.var_wdtsi_dn11 = assign20950_e18871_d_n11;
        locals.var_wdtsi_dn12 = assign20950_e18871_d_n12;
        locals.var_wdtsi_rv = 0.0;

        let (assign20960_e18877, assign20960_e18877_d_n3, assign20960_e18877_d_n4, assign20960_e18877_d_n5, assign20960_e18877_d_n6, assign20960_e18877_d_n7, assign20960_e18877_d_n8, assign20960_e18877_d_n9, assign20960_e18877_d_n10, assign20960_e18877_d_n11, assign20960_e18877_d_n12,) = {
    if (locals.var_guard1240 != 0.0) {
        let assign20960_e18875: f64 = (locals.var_vtm * locals.var_pparam_b4soindiode);
        (assign20960_e18875, (locals.var_vtm * locals.var_pparam_b4soindiode_dn3), ((locals.var_vtm_dn4 * locals.var_pparam_b4soindiode) + (locals.var_vtm * locals.var_pparam_b4soindiode_dn4)), ((locals.var_vtm_dn5 * locals.var_pparam_b4soindiode) + (locals.var_vtm * locals.var_pparam_b4soindiode_dn5)), ((locals.var_vtm_dn6 * locals.var_pparam_b4soindiode) + (locals.var_vtm * locals.var_pparam_b4soindiode_dn6)), (locals.var_vtm * locals.var_pparam_b4soindiode_dn7), (locals.var_vtm * locals.var_pparam_b4soindiode_dn8), (locals.var_vtm * locals.var_pparam_b4soindiode_dn9), (locals.var_vtm * locals.var_pparam_b4soindiode_dn10), (locals.var_vtm * locals.var_pparam_b4soindiode_dn11), (locals.var_vtm * locals.var_pparam_b4soindiode_dn12),)
    } else {
        (locals.var_nvtm1, locals.var_nvtm1_dn3, locals.var_nvtm1_dn4, locals.var_nvtm1_dn5, locals.var_nvtm1_dn6, locals.var_nvtm1_dn7, locals.var_nvtm1_dn8, locals.var_nvtm1_dn9, locals.var_nvtm1_dn10, locals.var_nvtm1_dn11, locals.var_nvtm1_dn12,)
    }
};
        locals.var_nvtm1 = assign20960_e18877;
        locals.var_nvtm1_dn3 = assign20960_e18877_d_n3;
        locals.var_nvtm1_dn4 = assign20960_e18877_d_n4;
        locals.var_nvtm1_dn5 = assign20960_e18877_d_n5;
        locals.var_nvtm1_dn6 = assign20960_e18877_d_n6;
        locals.var_nvtm1_dn7 = assign20960_e18877_d_n7;
        locals.var_nvtm1_dn8 = assign20960_e18877_d_n8;
        locals.var_nvtm1_dn9 = assign20960_e18877_d_n9;
        locals.var_nvtm1_dn10 = assign20960_e18877_d_n10;
        locals.var_nvtm1_dn11 = assign20960_e18877_d_n11;
        locals.var_nvtm1_dn12 = assign20960_e18877_d_n12;
        locals.var_nvtm1_rv = 0.0;

        let (assign20970_e18883, assign20970_e18883_d_n3, assign20970_e18883_d_n4, assign20970_e18883_d_n5, assign20970_e18883_d_n6, assign20970_e18883_d_n7, assign20970_e18883_d_n8, assign20970_e18883_d_n9, assign20970_e18883_d_n10, assign20970_e18883_d_n11, assign20970_e18883_d_n12,) = {
    if (locals.var_guard1240 != 0.0) {
        let assign20970_e18881: f64 = (locals.var_vsbs / locals.var_nvtm1);
        (assign20970_e18881, (-((locals.var_vsbs * locals.var_nvtm1_dn3) / (locals.var_nvtm1 * locals.var_nvtm1))), (-((locals.var_vsbs * locals.var_nvtm1_dn4) / (locals.var_nvtm1 * locals.var_nvtm1))), (-((locals.var_vsbs * locals.var_nvtm1_dn5) / (locals.var_nvtm1 * locals.var_nvtm1))), (-((locals.var_vsbs * locals.var_nvtm1_dn6) / (locals.var_nvtm1 * locals.var_nvtm1))), (-((locals.var_vsbs * locals.var_nvtm1_dn7) / (locals.var_nvtm1 * locals.var_nvtm1))), (((locals.var_vsbs_dn8 * locals.var_nvtm1) - (locals.var_vsbs * locals.var_nvtm1_dn8)) / (locals.var_nvtm1 * locals.var_nvtm1)), (-((locals.var_vsbs * locals.var_nvtm1_dn9) / (locals.var_nvtm1 * locals.var_nvtm1))), (-((locals.var_vsbs * locals.var_nvtm1_dn10) / (locals.var_nvtm1 * locals.var_nvtm1))), (((locals.var_vsbs_dn11 * locals.var_nvtm1) - (locals.var_vsbs * locals.var_nvtm1_dn11)) / (locals.var_nvtm1 * locals.var_nvtm1)), (-((locals.var_vsbs * locals.var_nvtm1_dn12) / (locals.var_nvtm1 * locals.var_nvtm1))),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign20970_e18883;
        locals.var_t0__blk808_dn3 = assign20970_e18883_d_n3;
        locals.var_t0__blk808_dn4 = assign20970_e18883_d_n4;
        locals.var_t0__blk808_dn5 = assign20970_e18883_d_n5;
        locals.var_t0__blk808_dn6 = assign20970_e18883_d_n6;
        locals.var_t0__blk808_dn7 = assign20970_e18883_d_n7;
        locals.var_t0__blk808_dn8 = assign20970_e18883_d_n8;
        locals.var_t0__blk808_dn9 = assign20970_e18883_d_n9;
        locals.var_t0__blk808_dn10 = assign20970_e18883_d_n10;
        locals.var_t0__blk808_dn11 = assign20970_e18883_d_n11;
        locals.var_t0__blk808_dn12 = assign20970_e18883_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let assign20980_e18886: f64 = if locals.var_t0__blk808 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1253 = assign20980_e18886;
        locals.var_guard1253_rv = 0.0;

        let (assign20990_e18898, assign20990_e18898_d_n3, assign20990_e18898_d_n4, assign20990_e18898_d_n5, assign20990_e18898_d_n6, assign20990_e18898_d_n7, assign20990_e18898_d_n8, assign20990_e18898_d_n9, assign20990_e18898_d_n10, assign20990_e18898_d_n11, assign20990_e18898_d_n12,) = {
    if ((locals.var_guard1240 != 0.0) && (locals.var_guard1253 != 0.0)) {
        let assign20990_e18893: f64 = (1.0 + locals.var_t0__blk808);
        let assign20990_e18895: f64 = (assign20990_e18893 - 100.0);
        let assign20990_e18896: f64 = (2.688117142e43 * assign20990_e18895);
        (assign20990_e18896, (2.688117142e43 * locals.var_t0__blk808_dn3), (2.688117142e43 * locals.var_t0__blk808_dn4), (2.688117142e43 * locals.var_t0__blk808_dn5), (2.688117142e43 * locals.var_t0__blk808_dn6), (2.688117142e43 * locals.var_t0__blk808_dn7), (2.688117142e43 * locals.var_t0__blk808_dn8), (2.688117142e43 * locals.var_t0__blk808_dn9), (2.688117142e43 * locals.var_t0__blk808_dn10), (2.688117142e43 * locals.var_t0__blk808_dn11), (2.688117142e43 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_expvbsnvtm, locals.var_expvbsnvtm_dn3, locals.var_expvbsnvtm_dn4, locals.var_expvbsnvtm_dn5, locals.var_expvbsnvtm_dn6, locals.var_expvbsnvtm_dn7, locals.var_expvbsnvtm_dn8, locals.var_expvbsnvtm_dn9, locals.var_expvbsnvtm_dn10, locals.var_expvbsnvtm_dn11, locals.var_expvbsnvtm_dn12,)
    }
};
        locals.var_expvbsnvtm = assign20990_e18898;
        locals.var_expvbsnvtm_dn3 = assign20990_e18898_d_n3;
        locals.var_expvbsnvtm_dn4 = assign20990_e18898_d_n4;
        locals.var_expvbsnvtm_dn5 = assign20990_e18898_d_n5;
        locals.var_expvbsnvtm_dn6 = assign20990_e18898_d_n6;
        locals.var_expvbsnvtm_dn7 = assign20990_e18898_d_n7;
        locals.var_expvbsnvtm_dn8 = assign20990_e18898_d_n8;
        locals.var_expvbsnvtm_dn9 = assign20990_e18898_d_n9;
        locals.var_expvbsnvtm_dn10 = assign20990_e18898_d_n10;
        locals.var_expvbsnvtm_dn11 = assign20990_e18898_d_n11;
        locals.var_expvbsnvtm_dn12 = assign20990_e18898_d_n12;
        locals.var_expvbsnvtm_rv = 0.0;

        let assign21000_e18901: f64 = (-100.0);
        let assign21000_e18902: f64 = if locals.var_t0__blk808 < assign21000_e18901 { 1.0 } else { 0.0 };
        locals.var_guard1254 = assign21000_e18902;
        locals.var_guard1254_rv = 0.0;

        let (assign21010_e18911, assign21010_e18911_d_n3, assign21010_e18911_d_n4, assign21010_e18911_d_n5, assign21010_e18911_d_n6, assign21010_e18911_d_n7, assign21010_e18911_d_n8, assign21010_e18911_d_n9, assign21010_e18911_d_n10, assign21010_e18911_d_n11, assign21010_e18911_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1254 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_expvbsnvtm, locals.var_expvbsnvtm_dn3, locals.var_expvbsnvtm_dn4, locals.var_expvbsnvtm_dn5, locals.var_expvbsnvtm_dn6, locals.var_expvbsnvtm_dn7, locals.var_expvbsnvtm_dn8, locals.var_expvbsnvtm_dn9, locals.var_expvbsnvtm_dn10, locals.var_expvbsnvtm_dn11, locals.var_expvbsnvtm_dn12,)
    }
};
        locals.var_expvbsnvtm = assign21010_e18911;
        locals.var_expvbsnvtm_dn3 = assign21010_e18911_d_n3;
        locals.var_expvbsnvtm_dn4 = assign21010_e18911_d_n4;
        locals.var_expvbsnvtm_dn5 = assign21010_e18911_d_n5;
        locals.var_expvbsnvtm_dn6 = assign21010_e18911_d_n6;
        locals.var_expvbsnvtm_dn7 = assign21010_e18911_d_n7;
        locals.var_expvbsnvtm_dn8 = assign21010_e18911_d_n8;
        locals.var_expvbsnvtm_dn9 = assign21010_e18911_d_n9;
        locals.var_expvbsnvtm_dn10 = assign21010_e18911_d_n10;
        locals.var_expvbsnvtm_dn11 = assign21010_e18911_d_n11;
        locals.var_expvbsnvtm_dn12 = assign21010_e18911_d_n12;
        locals.var_expvbsnvtm_rv = 0.0;

        let (assign21020_e18922, assign21020_e18922_d_n3, assign21020_e18922_d_n4, assign21020_e18922_d_n5, assign21020_e18922_d_n6, assign21020_e18922_d_n7, assign21020_e18922_d_n8, assign21020_e18922_d_n9, assign21020_e18922_d_n10, assign21020_e18922_d_n11, assign21020_e18922_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1253 == 0.0)) && (locals.var_guard1254 == 0.0)) {
        let assign21020_e18920: f64 = (locals.var_t0__blk808).exp();
        (assign21020_e18920, (assign21020_e18920 * locals.var_t0__blk808_dn3), (assign21020_e18920 * locals.var_t0__blk808_dn4), (assign21020_e18920 * locals.var_t0__blk808_dn5), (assign21020_e18920 * locals.var_t0__blk808_dn6), (assign21020_e18920 * locals.var_t0__blk808_dn7), (assign21020_e18920 * locals.var_t0__blk808_dn8), (assign21020_e18920 * locals.var_t0__blk808_dn9), (assign21020_e18920 * locals.var_t0__blk808_dn10), (assign21020_e18920 * locals.var_t0__blk808_dn11), (assign21020_e18920 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_expvbsnvtm, locals.var_expvbsnvtm_dn3, locals.var_expvbsnvtm_dn4, locals.var_expvbsnvtm_dn5, locals.var_expvbsnvtm_dn6, locals.var_expvbsnvtm_dn7, locals.var_expvbsnvtm_dn8, locals.var_expvbsnvtm_dn9, locals.var_expvbsnvtm_dn10, locals.var_expvbsnvtm_dn11, locals.var_expvbsnvtm_dn12,)
    }
};
        locals.var_expvbsnvtm = assign21020_e18922;
        locals.var_expvbsnvtm_dn3 = assign21020_e18922_d_n3;
        locals.var_expvbsnvtm_dn4 = assign21020_e18922_d_n4;
        locals.var_expvbsnvtm_dn5 = assign21020_e18922_d_n5;
        locals.var_expvbsnvtm_dn6 = assign21020_e18922_d_n6;
        locals.var_expvbsnvtm_dn7 = assign21020_e18922_d_n7;
        locals.var_expvbsnvtm_dn8 = assign21020_e18922_d_n8;
        locals.var_expvbsnvtm_dn9 = assign21020_e18922_d_n9;
        locals.var_expvbsnvtm_dn10 = assign21020_e18922_d_n10;
        locals.var_expvbsnvtm_dn11 = assign21020_e18922_d_n11;
        locals.var_expvbsnvtm_dn12 = assign21020_e18922_d_n12;
        locals.var_expvbsnvtm_rv = 0.0;

        let (assign21030_e18928, assign21030_e18928_d_n3, assign21030_e18928_d_n4, assign21030_e18928_d_n5, assign21030_e18928_d_n6, assign21030_e18928_d_n7, assign21030_e18928_d_n8, assign21030_e18928_d_n9, assign21030_e18928_d_n10, assign21030_e18928_d_n11, assign21030_e18928_d_n12,) = {
    if (locals.var_guard1240 != 0.0) {
        let assign21030_e18926: f64 = (locals.var_vtm * locals.var_pparam_b4soindioded);
        (assign21030_e18926, (locals.var_vtm * locals.var_pparam_b4soindioded_dn3), ((locals.var_vtm_dn4 * locals.var_pparam_b4soindioded) + (locals.var_vtm * locals.var_pparam_b4soindioded_dn4)), ((locals.var_vtm_dn5 * locals.var_pparam_b4soindioded) + (locals.var_vtm * locals.var_pparam_b4soindioded_dn5)), ((locals.var_vtm_dn6 * locals.var_pparam_b4soindioded) + (locals.var_vtm * locals.var_pparam_b4soindioded_dn6)), (locals.var_vtm * locals.var_pparam_b4soindioded_dn7), (locals.var_vtm * locals.var_pparam_b4soindioded_dn8), (locals.var_vtm * locals.var_pparam_b4soindioded_dn9), (locals.var_vtm * locals.var_pparam_b4soindioded_dn10), (locals.var_vtm * locals.var_pparam_b4soindioded_dn11), (locals.var_vtm * locals.var_pparam_b4soindioded_dn12),)
    } else {
        (locals.var_nvtm1, locals.var_nvtm1_dn3, locals.var_nvtm1_dn4, locals.var_nvtm1_dn5, locals.var_nvtm1_dn6, locals.var_nvtm1_dn7, locals.var_nvtm1_dn8, locals.var_nvtm1_dn9, locals.var_nvtm1_dn10, locals.var_nvtm1_dn11, locals.var_nvtm1_dn12,)
    }
};
        locals.var_nvtm1 = assign21030_e18928;
        locals.var_nvtm1_dn3 = assign21030_e18928_d_n3;
        locals.var_nvtm1_dn4 = assign21030_e18928_d_n4;
        locals.var_nvtm1_dn5 = assign21030_e18928_d_n5;
        locals.var_nvtm1_dn6 = assign21030_e18928_d_n6;
        locals.var_nvtm1_dn7 = assign21030_e18928_d_n7;
        locals.var_nvtm1_dn8 = assign21030_e18928_d_n8;
        locals.var_nvtm1_dn9 = assign21030_e18928_d_n9;
        locals.var_nvtm1_dn10 = assign21030_e18928_d_n10;
        locals.var_nvtm1_dn11 = assign21030_e18928_d_n11;
        locals.var_nvtm1_dn12 = assign21030_e18928_d_n12;
        locals.var_nvtm1_rv = 0.0;

        let (assign21040_e18934, assign21040_e18934_d_n3, assign21040_e18934_d_n4, assign21040_e18934_d_n5, assign21040_e18934_d_n6, assign21040_e18934_d_n7, assign21040_e18934_d_n8, assign21040_e18934_d_n9, assign21040_e18934_d_n10, assign21040_e18934_d_n11, assign21040_e18934_d_n12,) = {
    if (locals.var_guard1240 != 0.0) {
        let assign21040_e18932: f64 = (locals.var_vdbd / locals.var_nvtm1);
        (assign21040_e18932, (-((locals.var_vdbd * locals.var_nvtm1_dn3) / (locals.var_nvtm1 * locals.var_nvtm1))), (-((locals.var_vdbd * locals.var_nvtm1_dn4) / (locals.var_nvtm1 * locals.var_nvtm1))), (-((locals.var_vdbd * locals.var_nvtm1_dn5) / (locals.var_nvtm1 * locals.var_nvtm1))), (-((locals.var_vdbd * locals.var_nvtm1_dn6) / (locals.var_nvtm1 * locals.var_nvtm1))), (((locals.var_vdbd_dn7 * locals.var_nvtm1) - (locals.var_vdbd * locals.var_nvtm1_dn7)) / (locals.var_nvtm1 * locals.var_nvtm1)), (-((locals.var_vdbd * locals.var_nvtm1_dn8) / (locals.var_nvtm1 * locals.var_nvtm1))), (-((locals.var_vdbd * locals.var_nvtm1_dn9) / (locals.var_nvtm1 * locals.var_nvtm1))), (-((locals.var_vdbd * locals.var_nvtm1_dn10) / (locals.var_nvtm1 * locals.var_nvtm1))), (-((locals.var_vdbd * locals.var_nvtm1_dn11) / (locals.var_nvtm1 * locals.var_nvtm1))), (((locals.var_vdbd_dn12 * locals.var_nvtm1) - (locals.var_vdbd * locals.var_nvtm1_dn12)) / (locals.var_nvtm1 * locals.var_nvtm1)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign21040_e18934;
        locals.var_t0__blk808_dn3 = assign21040_e18934_d_n3;
        locals.var_t0__blk808_dn4 = assign21040_e18934_d_n4;
        locals.var_t0__blk808_dn5 = assign21040_e18934_d_n5;
        locals.var_t0__blk808_dn6 = assign21040_e18934_d_n6;
        locals.var_t0__blk808_dn7 = assign21040_e18934_d_n7;
        locals.var_t0__blk808_dn8 = assign21040_e18934_d_n8;
        locals.var_t0__blk808_dn9 = assign21040_e18934_d_n9;
        locals.var_t0__blk808_dn10 = assign21040_e18934_d_n10;
        locals.var_t0__blk808_dn11 = assign21040_e18934_d_n11;
        locals.var_t0__blk808_dn12 = assign21040_e18934_d_n12;
        locals.var_t0__blk808_rv = 0.0;

        let assign21050_e18937: f64 = if locals.var_t0__blk808 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1255 = assign21050_e18937;
        locals.var_guard1255_rv = 0.0;

    }
}
