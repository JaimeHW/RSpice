#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        var_a0: f64,
        var_a0_dn3: f64,
        var_a0_dn4: f64,
        var_a0_dn5: f64,
        var_a0_dn6: f64,
        var_a0_dn7: f64,
        var_a0_dn8: f64,
        var_guard91: f64,
        var_k1: f64,
        var_k1_2: f64,
        var_lna0: f64,
        var_lna0_dn3: f64,
        var_lna0_dn4: f64,
        var_lna0_dn5: f64,
        var_lna0_dn6: f64,
        var_lna0_dn7: f64,
        var_lna0_dn8: f64,
        var_phi1_0: f64,
        var_phi1_0_dn3: f64,
        var_phi1_0_dn4: f64,
        var_phi1_0_dn5: f64,
        var_phi1_0_dn6: f64,
        var_phi1_0_dn7: f64,
        var_phi1_0_dn8: f64,
        var_phi2: f64,
        var_phi2_dn3: f64,
        var_phi2_dn4: f64,
        var_phi2_dn5: f64,
        var_phi2_dn6: f64,
        var_phi2_dn7: f64,
        var_phi2_dn8: f64,
        var_q2: f64,
        var_q2_dn3: f64,
        var_q2_dn4: f64,
        var_q2_dn5: f64,
        var_q2_dn6: f64,
        var_q2_dn7: f64,
        var_q2_dn8: f64,
        var_delta_slot: &mut f64,
        var_delta_dn3_slot: &mut f64,
        var_delta_dn4_slot: &mut f64,
        var_delta_dn5_slot: &mut f64,
        var_delta_dn6_slot: &mut f64,
        var_delta_dn7_slot: &mut f64,
        var_delta_dn8_slot: &mut f64,
        var_dg1_slot: &mut f64,
        var_dg1_dn3_slot: &mut f64,
        var_dg1_dn4_slot: &mut f64,
        var_dg1_dn5_slot: &mut f64,
        var_dg1_dn6_slot: &mut f64,
        var_dg1_dn7_slot: &mut f64,
        var_dg1_dn8_slot: &mut f64,
        var_dg2_slot: &mut f64,
        var_dg2_dn3_slot: &mut f64,
        var_dg2_dn4_slot: &mut f64,
        var_dg2_dn5_slot: &mut f64,
        var_dg2_dn6_slot: &mut f64,
        var_dg2_dn7_slot: &mut f64,
        var_dg2_dn8_slot: &mut f64,
        var_g_slot: &mut f64,
        var_g_dn3_slot: &mut f64,
        var_g_dn4_slot: &mut f64,
        var_g_dn5_slot: &mut f64,
        var_g_dn6_slot: &mut f64,
        var_g_dn7_slot: &mut f64,
        var_g_dn8_slot: &mut f64,
        var_phissat_slot: &mut f64,
        var_phissat_dn3_slot: &mut f64,
        var_phissat_dn4_slot: &mut f64,
        var_phissat_dn5_slot: &mut f64,
        var_phissat_dn6_slot: &mut f64,
        var_phissat_dn7_slot: &mut f64,
        var_phissat_dn8_slot: &mut f64,
        var_q1_slot: &mut f64,
        var_q1_dn3_slot: &mut f64,
        var_q1_dn4_slot: &mut f64,
        var_q1_dn5_slot: &mut f64,
        var_q1_dn6_slot: &mut f64,
        var_q1_dn7_slot: &mut f64,
        var_q1_dn8_slot: &mut f64,
        var_qsqrt_slot: &mut f64,
        var_qsqrt_dn3_slot: &mut f64,
        var_qsqrt_dn4_slot: &mut f64,
        var_qsqrt_dn5_slot: &mut f64,
        var_qsqrt_dn6_slot: &mut f64,
        var_qsqrt_dn7_slot: &mut f64,
        var_qsqrt_dn8_slot: &mut f64,
        var_qt_slot: &mut f64,
        var_qt_dn3_slot: &mut f64,
        var_qt_dn4_slot: &mut f64,
        var_qt_dn5_slot: &mut f64,
        var_qt_dn6_slot: &mut f64,
        var_qt_dn7_slot: &mut f64,
        var_qt_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_xg1_slot: &mut f64,
        var_xg1_dn3_slot: &mut f64,
        var_xg1_dn4_slot: &mut f64,
        var_xg1_dn5_slot: &mut f64,
        var_xg1_dn6_slot: &mut f64,
        var_xg1_dn7_slot: &mut f64,
        var_xg1_dn8_slot: &mut f64,
    ) {
        let mut var_delta: f64 = *var_delta_slot;
        let mut var_delta_dn3: f64 = *var_delta_dn3_slot;
        let mut var_delta_dn4: f64 = *var_delta_dn4_slot;
        let mut var_delta_dn5: f64 = *var_delta_dn5_slot;
        let mut var_delta_dn6: f64 = *var_delta_dn6_slot;
        let mut var_delta_dn7: f64 = *var_delta_dn7_slot;
        let mut var_delta_dn8: f64 = *var_delta_dn8_slot;
        let mut var_dg1: f64 = *var_dg1_slot;
        let mut var_dg1_dn3: f64 = *var_dg1_dn3_slot;
        let mut var_dg1_dn4: f64 = *var_dg1_dn4_slot;
        let mut var_dg1_dn5: f64 = *var_dg1_dn5_slot;
        let mut var_dg1_dn6: f64 = *var_dg1_dn6_slot;
        let mut var_dg1_dn7: f64 = *var_dg1_dn7_slot;
        let mut var_dg1_dn8: f64 = *var_dg1_dn8_slot;
        let mut var_dg2: f64 = *var_dg2_slot;
        let mut var_dg2_dn3: f64 = *var_dg2_dn3_slot;
        let mut var_dg2_dn4: f64 = *var_dg2_dn4_slot;
        let mut var_dg2_dn5: f64 = *var_dg2_dn5_slot;
        let mut var_dg2_dn6: f64 = *var_dg2_dn6_slot;
        let mut var_dg2_dn7: f64 = *var_dg2_dn7_slot;
        let mut var_dg2_dn8: f64 = *var_dg2_dn8_slot;
        let mut var_g: f64 = *var_g_slot;
        let mut var_g_dn3: f64 = *var_g_dn3_slot;
        let mut var_g_dn4: f64 = *var_g_dn4_slot;
        let mut var_g_dn5: f64 = *var_g_dn5_slot;
        let mut var_g_dn6: f64 = *var_g_dn6_slot;
        let mut var_g_dn7: f64 = *var_g_dn7_slot;
        let mut var_g_dn8: f64 = *var_g_dn8_slot;
        let mut var_phissat: f64 = *var_phissat_slot;
        let mut var_phissat_dn3: f64 = *var_phissat_dn3_slot;
        let mut var_phissat_dn4: f64 = *var_phissat_dn4_slot;
        let mut var_phissat_dn5: f64 = *var_phissat_dn5_slot;
        let mut var_phissat_dn6: f64 = *var_phissat_dn6_slot;
        let mut var_phissat_dn7: f64 = *var_phissat_dn7_slot;
        let mut var_phissat_dn8: f64 = *var_phissat_dn8_slot;
        let mut var_q1: f64 = *var_q1_slot;
        let mut var_q1_dn3: f64 = *var_q1_dn3_slot;
        let mut var_q1_dn4: f64 = *var_q1_dn4_slot;
        let mut var_q1_dn5: f64 = *var_q1_dn5_slot;
        let mut var_q1_dn6: f64 = *var_q1_dn6_slot;
        let mut var_q1_dn7: f64 = *var_q1_dn7_slot;
        let mut var_q1_dn8: f64 = *var_q1_dn8_slot;
        let mut var_qsqrt: f64 = *var_qsqrt_slot;
        let mut var_qsqrt_dn3: f64 = *var_qsqrt_dn3_slot;
        let mut var_qsqrt_dn4: f64 = *var_qsqrt_dn4_slot;
        let mut var_qsqrt_dn5: f64 = *var_qsqrt_dn5_slot;
        let mut var_qsqrt_dn6: f64 = *var_qsqrt_dn6_slot;
        let mut var_qsqrt_dn7: f64 = *var_qsqrt_dn7_slot;
        let mut var_qsqrt_dn8: f64 = *var_qsqrt_dn8_slot;
        let mut var_qt: f64 = *var_qt_slot;
        let mut var_qt_dn3: f64 = *var_qt_dn3_slot;
        let mut var_qt_dn4: f64 = *var_qt_dn4_slot;
        let mut var_qt_dn5: f64 = *var_qt_dn5_slot;
        let mut var_qt_dn6: f64 = *var_qt_dn6_slot;
        let mut var_qt_dn7: f64 = *var_qt_dn7_slot;
        let mut var_qt_dn8: f64 = *var_qt_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_xg1: f64 = *var_xg1_slot;
        let mut var_xg1_dn3: f64 = *var_xg1_dn3_slot;
        let mut var_xg1_dn4: f64 = *var_xg1_dn4_slot;
        let mut var_xg1_dn5: f64 = *var_xg1_dn5_slot;
        let mut var_xg1_dn6: f64 = *var_xg1_dn6_slot;
        let mut var_xg1_dn7: f64 = *var_xg1_dn7_slot;
        let mut var_xg1_dn8: f64 = *var_xg1_dn8_slot;

        let (assign8070_e8053, assign8070_e8053_d_n3, assign8070_e8053_d_n4, assign8070_e8053_d_n5, assign8070_e8053_d_n6, assign8070_e8053_d_n7, assign8070_e8053_d_n8,) = {
    if (var_guard91 != 0.0) {
        let assign8070_e8051: f64 = (40.0 * var_k1);
        (assign8070_e8051, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_q1, var_q1_dn3, var_q1_dn4, var_q1_dn5, var_q1_dn6, var_q1_dn7, var_q1_dn8,)
    }
};
        var_q1 = assign8070_e8053;
        var_q1_dn3 = assign8070_e8053_d_n3;
        var_q1_dn4 = assign8070_e8053_d_n4;
        var_q1_dn5 = assign8070_e8053_d_n5;
        var_q1_dn6 = assign8070_e8053_d_n6;
        var_q1_dn7 = assign8070_e8053_d_n7;
        var_q1_dn8 = assign8070_e8053_d_n8;

        let (assign8080_e8059, assign8080_e8059_d_n3, assign8080_e8059_d_n4, assign8080_e8059_d_n5, assign8080_e8059_d_n6, assign8080_e8059_d_n7, assign8080_e8059_d_n8,) = {
    if (var_guard91 != 0.0) {
        let assign8080_e8057: f64 = (var_q1 + var_q2);
        (assign8080_e8057, (var_q1_dn3 + var_q2_dn3), (var_q1_dn4 + var_q2_dn4), (var_q1_dn5 + var_q2_dn5), (var_q1_dn6 + var_q2_dn6), (var_q1_dn7 + var_q2_dn7), (var_q1_dn8 + var_q2_dn8),)
    } else {
        (var_qt, var_qt_dn3, var_qt_dn4, var_qt_dn5, var_qt_dn6, var_qt_dn7, var_qt_dn8,)
    }
};
        var_qt = assign8080_e8059;
        var_qt_dn3 = assign8080_e8059_d_n3;
        var_qt_dn4 = assign8080_e8059_d_n4;
        var_qt_dn5 = assign8080_e8059_d_n5;
        var_qt_dn6 = assign8080_e8059_d_n6;
        var_qt_dn7 = assign8080_e8059_d_n7;
        var_qt_dn8 = assign8080_e8059_d_n8;

        let (assign8090_e8065, assign8090_e8065_d_n3, assign8090_e8065_d_n4, assign8090_e8065_d_n5, assign8090_e8065_d_n6, assign8090_e8065_d_n7, assign8090_e8065_d_n8,) = {
    if (var_guard91 != 0.0) {
        let assign8090_e8063: f64 = (var_q1 * var_q2);
        (assign8090_e8063, ((var_q1_dn3 * var_q2) + (var_q1 * var_q2_dn3)), ((var_q1_dn4 * var_q2) + (var_q1 * var_q2_dn4)), ((var_q1_dn5 * var_q2) + (var_q1 * var_q2_dn5)), ((var_q1_dn6 * var_q2) + (var_q1 * var_q2_dn6)), ((var_q1_dn7 * var_q2) + (var_q1 * var_q2_dn7)), ((var_q1_dn8 * var_q2) + (var_q1 * var_q2_dn8)),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8,)
    }
};
        var_t3 = assign8090_e8065;
        var_t3_dn3 = assign8090_e8065_d_n3;
        var_t3_dn4 = assign8090_e8065_d_n4;
        var_t3_dn5 = assign8090_e8065_d_n5;
        var_t3_dn6 = assign8090_e8065_d_n6;
        var_t3_dn7 = assign8090_e8065_d_n7;
        var_t3_dn8 = assign8090_e8065_d_n8;

        let (assign8100_e8073, assign8100_e8073_d_n3, assign8100_e8073_d_n4, assign8100_e8073_d_n5, assign8100_e8073_d_n6, assign8100_e8073_d_n7, assign8100_e8073_d_n8,) = {
    if (var_guard91 != 0.0) {
        let assign8100_e8069: f64 = (0.06534 * var_qt);
        let assign8100_e8071: f64 = (assign8100_e8069 + 1.0);
        (assign8100_e8071, (0.06534 * var_qt_dn3), (0.06534 * var_qt_dn4), (0.06534 * var_qt_dn5), (0.06534 * var_qt_dn6), (0.06534 * var_qt_dn7), (0.06534 * var_qt_dn8),)
    } else {
        (var_t4, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8,)
    }
};
        var_t4 = assign8100_e8073;
        var_t4_dn3 = assign8100_e8073_d_n3;
        var_t4_dn4 = assign8100_e8073_d_n4;
        var_t4_dn5 = assign8100_e8073_d_n5;
        var_t4_dn6 = assign8100_e8073_d_n6;
        var_t4_dn7 = assign8100_e8073_d_n7;
        var_t4_dn8 = assign8100_e8073_d_n8;

        let (assign8110_e8083, assign8110_e8083_d_n3, assign8110_e8083_d_n4, assign8110_e8083_d_n5, assign8110_e8083_d_n6, assign8110_e8083_d_n7, assign8110_e8083_d_n8,) = {
    if (var_guard91 != 0.0) {
        let assign8110_e8077: f64 = (var_qt * 8.57973);
        let assign8110_e8079: f64 = (assign8110_e8077 + var_t3);
        let assign8110_e8081: f64 = (assign8110_e8079 + 39.47841);
        (assign8110_e8081, ((var_qt_dn3 * 8.57973) + var_t3_dn3), ((var_qt_dn4 * 8.57973) + var_t3_dn4), ((var_qt_dn5 * 8.57973) + var_t3_dn5), ((var_qt_dn6 * 8.57973) + var_t3_dn6), ((var_qt_dn7 * 8.57973) + var_t3_dn7), ((var_qt_dn8 * 8.57973) + var_t3_dn8),)
    } else {
        (var_t5, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8,)
    }
};
        var_t5 = assign8110_e8083;
        var_t5_dn3 = assign8110_e8083_d_n3;
        var_t5_dn4 = assign8110_e8083_d_n4;
        var_t5_dn5 = assign8110_e8083_d_n5;
        var_t5_dn6 = assign8110_e8083_d_n6;
        var_t5_dn7 = assign8110_e8083_d_n7;
        var_t5_dn8 = assign8110_e8083_d_n8;

        let (assign8120_e8093, assign8120_e8093_d_n3, assign8120_e8093_d_n4, assign8120_e8093_d_n5, assign8120_e8093_d_n6, assign8120_e8093_d_n7, assign8120_e8093_d_n8,) = {
    if (var_guard91 != 0.0) {
        let assign8120_e8087: f64 = (78.95683 * var_qt);
        let assign8120_e8090: f64 = (39.47841 * var_t3);
        let assign8120_e8091: f64 = (assign8120_e8087 + assign8120_e8090);
        (assign8120_e8091, ((78.95683 * var_qt_dn3) + (39.47841 * var_t3_dn3)), ((78.95683 * var_qt_dn4) + (39.47841 * var_t3_dn4)), ((78.95683 * var_qt_dn5) + (39.47841 * var_t3_dn5)), ((78.95683 * var_qt_dn6) + (39.47841 * var_t3_dn6)), ((78.95683 * var_qt_dn7) + (39.47841 * var_t3_dn7)), ((78.95683 * var_qt_dn8) + (39.47841 * var_t3_dn8)),)
    } else {
        (var_t6, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8,)
    }
};
        var_t6 = assign8120_e8093;
        var_t6_dn3 = assign8120_e8093_d_n3;
        var_t6_dn4 = assign8120_e8093_d_n4;
        var_t6_dn5 = assign8120_e8093_d_n5;
        var_t6_dn6 = assign8120_e8093_d_n6;
        var_t6_dn7 = assign8120_e8093_d_n7;
        var_t6_dn8 = assign8120_e8093_d_n8;

        let (assign8130_e8114, assign8130_e8114_d_n3, assign8130_e8114_d_n4, assign8130_e8114_d_n5, assign8130_e8114_d_n6, assign8130_e8114_d_n7, assign8130_e8114_d_n8,) = {
    if (var_guard91 != 0.0) {
        let assign8130_e8096: f64 = (-var_t5);
        let assign8130_e8098: f64 = (-4.0);
        let assign8130_e8100: f64 = (assign8130_e8098 * var_t4);
        let assign8130_e8102: f64 = (assign8130_e8100 * var_t6);
        let assign8130_e8105: f64 = (var_t5 * var_t5);
        let assign8130_e8106: f64 = (assign8130_e8102 + assign8130_e8105);
        let assign8130_e8107: f64 = (assign8130_e8106).sqrt();
        let assign8130_e8108: f64 = (assign8130_e8096 + assign8130_e8107);
        let assign8130_e8111: f64 = (2.0 * var_t4);
        let assign8130_e8112: f64 = (assign8130_e8108 / assign8130_e8111);
        (assign8130_e8112, (((((-var_t5_dn3) + (((((assign8130_e8098 * var_t4_dn3) * var_t6) + (assign8130_e8100 * var_t6_dn3)) + ((var_t5_dn3 * var_t5) + (var_t5 * var_t5_dn3))) / (2.0 * assign8130_e8107))) * assign8130_e8111) - (assign8130_e8108 * (2.0 * var_t4_dn3))) / (assign8130_e8111 * assign8130_e8111)), (((((-var_t5_dn4) + (((((assign8130_e8098 * var_t4_dn4) * var_t6) + (assign8130_e8100 * var_t6_dn4)) + ((var_t5_dn4 * var_t5) + (var_t5 * var_t5_dn4))) / (2.0 * assign8130_e8107))) * assign8130_e8111) - (assign8130_e8108 * (2.0 * var_t4_dn4))) / (assign8130_e8111 * assign8130_e8111)), (((((-var_t5_dn5) + (((((assign8130_e8098 * var_t4_dn5) * var_t6) + (assign8130_e8100 * var_t6_dn5)) + ((var_t5_dn5 * var_t5) + (var_t5 * var_t5_dn5))) / (2.0 * assign8130_e8107))) * assign8130_e8111) - (assign8130_e8108 * (2.0 * var_t4_dn5))) / (assign8130_e8111 * assign8130_e8111)), (((((-var_t5_dn6) + (((((assign8130_e8098 * var_t4_dn6) * var_t6) + (assign8130_e8100 * var_t6_dn6)) + ((var_t5_dn6 * var_t5) + (var_t5 * var_t5_dn6))) / (2.0 * assign8130_e8107))) * assign8130_e8111) - (assign8130_e8108 * (2.0 * var_t4_dn6))) / (assign8130_e8111 * assign8130_e8111)), (((((-var_t5_dn7) + (((((assign8130_e8098 * var_t4_dn7) * var_t6) + (assign8130_e8100 * var_t6_dn7)) + ((var_t5_dn7 * var_t5) + (var_t5 * var_t5_dn7))) / (2.0 * assign8130_e8107))) * assign8130_e8111) - (assign8130_e8108 * (2.0 * var_t4_dn7))) / (assign8130_e8111 * assign8130_e8111)), (((((-var_t5_dn8) + (((((assign8130_e8098 * var_t4_dn8) * var_t6) + (assign8130_e8100 * var_t6_dn8)) + ((var_t5_dn8 * var_t5) + (var_t5 * var_t5_dn8))) / (2.0 * assign8130_e8107))) * assign8130_e8111) - (assign8130_e8108 * (2.0 * var_t4_dn8))) / (assign8130_e8111 * assign8130_e8111)),)
    } else {
        (var_qsqrt, var_qsqrt_dn3, var_qsqrt_dn4, var_qsqrt_dn5, var_qsqrt_dn6, var_qsqrt_dn7, var_qsqrt_dn8,)
    }
};
        var_qsqrt = assign8130_e8114;
        var_qsqrt_dn3 = assign8130_e8114_d_n3;
        var_qsqrt_dn4 = assign8130_e8114_d_n4;
        var_qsqrt_dn5 = assign8130_e8114_d_n5;
        var_qsqrt_dn6 = assign8130_e8114_d_n6;
        var_qsqrt_dn7 = assign8130_e8114_d_n7;
        var_qsqrt_dn8 = assign8130_e8114_d_n8;

        let (assign8140_e8126, assign8140_e8126_d_n3, assign8140_e8126_d_n4, assign8140_e8126_d_n5, assign8140_e8126_d_n6, assign8140_e8126_d_n7, assign8140_e8126_d_n8,) = {
    if (var_guard91 != 0.0) {
        let assign8140_e8119: f64 = (1.0 + var_k1);
        let assign8140_e8120: f64 = (var_phi1_0 * assign8140_e8119);
        let assign8140_e8122: f64 = (assign8140_e8120 - var_phi2);
        let assign8140_e8124: f64 = (assign8140_e8122 / var_k1);
        (assign8140_e8124, (((var_phi1_0_dn3 * assign8140_e8119) - var_phi2_dn3) / var_k1), (((var_phi1_0_dn4 * assign8140_e8119) - var_phi2_dn4) / var_k1), (((var_phi1_0_dn5 * assign8140_e8119) - var_phi2_dn5) / var_k1), (((var_phi1_0_dn6 * assign8140_e8119) - var_phi2_dn6) / var_k1), (((var_phi1_0_dn7 * assign8140_e8119) - var_phi2_dn7) / var_k1), (((var_phi1_0_dn8 * assign8140_e8119) - var_phi2_dn8) / var_k1),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8,)
    }
};
        var_t3 = assign8140_e8126;
        var_t3_dn3 = assign8140_e8126_d_n3;
        var_t3_dn4 = assign8140_e8126_d_n4;
        var_t3_dn5 = assign8140_e8126_d_n5;
        var_t3_dn6 = assign8140_e8126_d_n6;
        var_t3_dn7 = assign8140_e8126_d_n7;
        var_t3_dn8 = assign8140_e8126_d_n8;

        let (assign8150_e8138, assign8150_e8138_d_n3, assign8150_e8138_d_n4, assign8150_e8138_d_n5, assign8150_e8138_d_n6, assign8150_e8138_d_n7, assign8150_e8138_d_n8,) = {
    if (var_guard91 != 0.0) {
        let assign8150_e8131: f64 = (var_xg1 - var_t3);
        let assign8150_e8133: f64 = (assign8150_e8131 + 2.0);
        let assign8150_e8134: f64 = (40.0 * assign8150_e8133);
        let assign8150_e8136: f64 = (assign8150_e8134 / 5.0);
        (assign8150_e8136, ((40.0 * (var_xg1_dn3 - var_t3_dn3)) / 5.0), ((40.0 * (var_xg1_dn4 - var_t3_dn4)) / 5.0), ((40.0 * (var_xg1_dn5 - var_t3_dn5)) / 5.0), ((40.0 * (var_xg1_dn6 - var_t3_dn6)) / 5.0), ((40.0 * (var_xg1_dn7 - var_t3_dn7)) / 5.0), ((40.0 * (var_xg1_dn8 - var_t3_dn8)) / 5.0),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign8150_e8138;
        var_t0_dn3 = assign8150_e8138_d_n3;
        var_t0_dn4 = assign8150_e8138_d_n4;
        var_t0_dn5 = assign8150_e8138_d_n5;
        var_t0_dn6 = assign8150_e8138_d_n6;
        var_t0_dn7 = assign8150_e8138_d_n7;
        var_t0_dn8 = assign8150_e8138_d_n8;

        let (assign8160_e8156, assign8160_e8156_d_n3, assign8160_e8156_d_n4, assign8160_e8156_d_n5, assign8160_e8156_d_n6, assign8160_e8156_d_n7, assign8160_e8156_d_n8,) = {
    if (var_guard91 != 0.0) {
        let assign8160_e8144: f64 = (var_xg1 - var_t3);
        let assign8160_e8146: f64 = (assign8160_e8144 + 2.0);
        let assign8160_e8147: f64 = (-assign8160_e8146);
        let assign8160_e8150: f64 = (2.0 / 0.69);
        let assign8160_e8151: f64 = (assign8160_e8147 / assign8160_e8150);
        let assign8160_e8152: f64 = (assign8160_e8151).exp();
        let assign8160_e8153: f64 = (1.0 - assign8160_e8152);
        let assign8160_e8154: f64 = (var_qsqrt * assign8160_e8153);
        (assign8160_e8154, ((var_qsqrt_dn3 * assign8160_e8153) + (var_qsqrt * (-(assign8160_e8152 * ((-(var_xg1_dn3 - var_t3_dn3)) / assign8160_e8150))))), ((var_qsqrt_dn4 * assign8160_e8153) + (var_qsqrt * (-(assign8160_e8152 * ((-(var_xg1_dn4 - var_t3_dn4)) / assign8160_e8150))))), ((var_qsqrt_dn5 * assign8160_e8153) + (var_qsqrt * (-(assign8160_e8152 * ((-(var_xg1_dn5 - var_t3_dn5)) / assign8160_e8150))))), ((var_qsqrt_dn6 * assign8160_e8153) + (var_qsqrt * (-(assign8160_e8152 * ((-(var_xg1_dn6 - var_t3_dn6)) / assign8160_e8150))))), ((var_qsqrt_dn7 * assign8160_e8153) + (var_qsqrt * (-(assign8160_e8152 * ((-(var_xg1_dn7 - var_t3_dn7)) / assign8160_e8150))))), ((var_qsqrt_dn8 * assign8160_e8153) + (var_qsqrt * (-(assign8160_e8152 * ((-(var_xg1_dn8 - var_t3_dn8)) / assign8160_e8150))))),)
    } else {
        (var_qsqrt, var_qsqrt_dn3, var_qsqrt_dn4, var_qsqrt_dn5, var_qsqrt_dn6, var_qsqrt_dn7, var_qsqrt_dn8,)
    }
};
        var_qsqrt = assign8160_e8156;
        var_qsqrt_dn3 = assign8160_e8156_d_n3;
        var_qsqrt_dn4 = assign8160_e8156_d_n4;
        var_qsqrt_dn5 = assign8160_e8156_d_n5;
        var_qsqrt_dn6 = assign8160_e8156_d_n6;
        var_qsqrt_dn7 = assign8160_e8156_d_n7;
        var_qsqrt_dn8 = assign8160_e8156_d_n8;

        let (assign8170_e8162, assign8170_e8162_d_n3, assign8170_e8162_d_n4, assign8170_e8162_d_n5, assign8170_e8162_d_n6, assign8170_e8162_d_n7, assign8170_e8162_d_n8,) = {
    if (var_guard91 != 0.0) {
        let assign8170_e8160: f64 = (var_qsqrt).min(50.0);
        (assign8170_e8160, if var_qsqrt <= 50.0 { var_qsqrt_dn3 } else { 0.0 }, if var_qsqrt <= 50.0 { var_qsqrt_dn4 } else { 0.0 }, if var_qsqrt <= 50.0 { var_qsqrt_dn5 } else { 0.0 }, if var_qsqrt <= 50.0 { var_qsqrt_dn6 } else { 0.0 }, if var_qsqrt <= 50.0 { var_qsqrt_dn7 } else { 0.0 }, if var_qsqrt <= 50.0 { var_qsqrt_dn8 } else { 0.0 },)
    } else {
        (var_qsqrt, var_qsqrt_dn3, var_qsqrt_dn4, var_qsqrt_dn5, var_qsqrt_dn6, var_qsqrt_dn7, var_qsqrt_dn8,)
    }
};
        var_qsqrt = assign8170_e8162;
        var_qsqrt_dn3 = assign8170_e8162_d_n3;
        var_qsqrt_dn4 = assign8170_e8162_d_n4;
        var_qsqrt_dn5 = assign8170_e8162_d_n5;
        var_qsqrt_dn6 = assign8170_e8162_d_n6;
        var_qsqrt_dn7 = assign8170_e8162_d_n7;
        var_qsqrt_dn8 = assign8170_e8162_d_n8;

        let assign8180_e8165: f64 = (var_xg1).max(var_phi1_0);
        var_xg1 = assign8180_e8165;
        var_xg1_dn3 = if var_xg1 >= var_phi1_0 { var_xg1_dn3 } else { var_phi1_0_dn3 };
        var_xg1_dn4 = if var_xg1 >= var_phi1_0 { var_xg1_dn4 } else { var_phi1_0_dn4 };
        var_xg1_dn5 = if var_xg1 >= var_phi1_0 { var_xg1_dn5 } else { var_phi1_0_dn5 };
        var_xg1_dn6 = if var_xg1 >= var_phi1_0 { var_xg1_dn6 } else { var_phi1_0_dn6 };
        var_xg1_dn7 = if var_xg1 >= var_phi1_0 { var_xg1_dn7 } else { var_phi1_0_dn7 };
        var_xg1_dn8 = if var_xg1 >= var_phi1_0 { var_xg1_dn8 } else { var_phi1_0_dn8 };

        let assign8190_e8169: f64 = (var_xg1 - var_phi1_0);
        let assign8190_e8170: f64 = (var_k1_2 * assign8190_e8169);
        let assign8190_e8173: f64 = (var_xg1 - var_phi1_0);
        let assign8190_e8174: f64 = (assign8190_e8170 * assign8190_e8173);
        let assign8190_e8176: f64 = (assign8190_e8174 + 39.47841);
        let assign8190_e8177: f64 = (assign8190_e8176).ln();
        let assign8190_e8179: f64 = (assign8190_e8177 - var_lna0);
        var_phissat = assign8190_e8179;
        var_phissat_dn3 = (((((var_k1_2 * (var_xg1_dn3 - var_phi1_0_dn3)) * assign8190_e8173) + (assign8190_e8170 * (var_xg1_dn3 - var_phi1_0_dn3))) / assign8190_e8176) - var_lna0_dn3);
        var_phissat_dn4 = (((((var_k1_2 * (var_xg1_dn4 - var_phi1_0_dn4)) * assign8190_e8173) + (assign8190_e8170 * (var_xg1_dn4 - var_phi1_0_dn4))) / assign8190_e8176) - var_lna0_dn4);
        var_phissat_dn5 = (((((var_k1_2 * (var_xg1_dn5 - var_phi1_0_dn5)) * assign8190_e8173) + (assign8190_e8170 * (var_xg1_dn5 - var_phi1_0_dn5))) / assign8190_e8176) - var_lna0_dn5);
        var_phissat_dn6 = (((((var_k1_2 * (var_xg1_dn6 - var_phi1_0_dn6)) * assign8190_e8173) + (assign8190_e8170 * (var_xg1_dn6 - var_phi1_0_dn6))) / assign8190_e8176) - var_lna0_dn6);
        var_phissat_dn7 = (((((var_k1_2 * (var_xg1_dn7 - var_phi1_0_dn7)) * assign8190_e8173) + (assign8190_e8170 * (var_xg1_dn7 - var_phi1_0_dn7))) / assign8190_e8176) - var_lna0_dn7);
        var_phissat_dn8 = (((((var_k1_2 * (var_xg1_dn8 - var_phi1_0_dn8)) * assign8190_e8173) + (assign8190_e8170 * (var_xg1_dn8 - var_phi1_0_dn8))) / assign8190_e8176) - var_lna0_dn8);

        let assign8200_e8183: f64 = (1.0 + var_k1);
        let assign8200_e8184: f64 = (var_phi1_0 * assign8200_e8183);
        let assign8200_e8186: f64 = (assign8200_e8184 - var_phi2);
        let assign8200_e8188: f64 = (assign8200_e8186 / var_k1);
        var_t3 = assign8200_e8188;
        var_t3_dn3 = (((var_phi1_0_dn3 * assign8200_e8183) - var_phi2_dn3) / var_k1);
        var_t3_dn4 = (((var_phi1_0_dn4 * assign8200_e8183) - var_phi2_dn4) / var_k1);
        var_t3_dn5 = (((var_phi1_0_dn5 * assign8200_e8183) - var_phi2_dn5) / var_k1);
        var_t3_dn6 = (((var_phi1_0_dn6 * assign8200_e8183) - var_phi2_dn6) / var_k1);
        var_t3_dn7 = (((var_phi1_0_dn7 * assign8200_e8183) - var_phi2_dn7) / var_k1);
        var_t3_dn8 = (((var_phi1_0_dn8 * assign8200_e8183) - var_phi2_dn8) / var_k1);

        let assign8210_e8192: f64 = (var_t3 - var_phi1_0);
        let assign8210_e8193: f64 = (var_k1_2 * assign8210_e8192);
        let assign8210_e8196: f64 = (var_t3 - var_phi1_0);
        let assign8210_e8197: f64 = (assign8210_e8193 * assign8210_e8196);
        let assign8210_e8199: f64 = (assign8210_e8197 + 39.47841);
        let assign8210_e8200: f64 = (assign8210_e8199).ln();
        let assign8210_e8202: f64 = (assign8210_e8200 - var_lna0);
        var_t4 = assign8210_e8202;
        var_t4_dn3 = (((((var_k1_2 * (var_t3_dn3 - var_phi1_0_dn3)) * assign8210_e8196) + (assign8210_e8193 * (var_t3_dn3 - var_phi1_0_dn3))) / assign8210_e8199) - var_lna0_dn3);
        var_t4_dn4 = (((((var_k1_2 * (var_t3_dn4 - var_phi1_0_dn4)) * assign8210_e8196) + (assign8210_e8193 * (var_t3_dn4 - var_phi1_0_dn4))) / assign8210_e8199) - var_lna0_dn4);
        var_t4_dn5 = (((((var_k1_2 * (var_t3_dn5 - var_phi1_0_dn5)) * assign8210_e8196) + (assign8210_e8193 * (var_t3_dn5 - var_phi1_0_dn5))) / assign8210_e8199) - var_lna0_dn5);
        var_t4_dn6 = (((((var_k1_2 * (var_t3_dn6 - var_phi1_0_dn6)) * assign8210_e8196) + (assign8210_e8193 * (var_t3_dn6 - var_phi1_0_dn6))) / assign8210_e8199) - var_lna0_dn6);
        var_t4_dn7 = (((((var_k1_2 * (var_t3_dn7 - var_phi1_0_dn7)) * assign8210_e8196) + (assign8210_e8193 * (var_t3_dn7 - var_phi1_0_dn7))) / assign8210_e8199) - var_lna0_dn7);
        var_t4_dn8 = (((((var_k1_2 * (var_t3_dn8 - var_phi1_0_dn8)) * assign8210_e8196) + (assign8210_e8193 * (var_t3_dn8 - var_phi1_0_dn8))) / assign8210_e8199) - var_lna0_dn8);

        let assign8220_e8205: f64 = (var_t4 - var_phi1_0);
        var_t5 = assign8220_e8205;
        var_t5_dn3 = (var_t4_dn3 - var_phi1_0_dn3);
        var_t5_dn4 = (var_t4_dn4 - var_phi1_0_dn4);
        var_t5_dn5 = (var_t4_dn5 - var_phi1_0_dn5);
        var_t5_dn6 = (var_t4_dn6 - var_phi1_0_dn6);
        var_t5_dn7 = (var_t4_dn7 - var_phi1_0_dn7);
        var_t5_dn8 = (var_t4_dn8 - var_phi1_0_dn8);

        let assign8230_e8208: f64 = (var_phissat - var_t5);
        var_phissat = assign8230_e8208;
        var_phissat_dn3 = (var_phissat_dn3 - var_t5_dn3);
        var_phissat_dn4 = (var_phissat_dn4 - var_t5_dn4);
        var_phissat_dn5 = (var_phissat_dn5 - var_t5_dn5);
        var_phissat_dn6 = (var_phissat_dn6 - var_t5_dn6);
        var_phissat_dn7 = (var_phissat_dn7 - var_t5_dn7);
        var_phissat_dn8 = (var_phissat_dn8 - var_t5_dn8);

        let assign8240_e8211: f64 = (var_xg1 - var_phissat);
        var_q1 = assign8240_e8211;
        var_q1_dn3 = (var_xg1_dn3 - var_phissat_dn3);
        var_q1_dn4 = (var_xg1_dn4 - var_phissat_dn4);
        var_q1_dn5 = (var_xg1_dn5 - var_phissat_dn5);
        var_q1_dn6 = (var_xg1_dn6 - var_phissat_dn6);
        var_q1_dn7 = (var_xg1_dn7 - var_phissat_dn7);
        var_q1_dn8 = (var_xg1_dn8 - var_phissat_dn8);

        let assign8250_e8213: f64 = (-var_a0);
        let assign8250_e8215: f64 = (var_phissat).exp();
        let assign8250_e8216: f64 = (assign8250_e8213 * assign8250_e8215);
        var_t0 = assign8250_e8216;
        var_t0_dn3 = (((-var_a0_dn3) * assign8250_e8215) + (assign8250_e8213 * (assign8250_e8215 * var_phissat_dn3)));
        var_t0_dn4 = (((-var_a0_dn4) * assign8250_e8215) + (assign8250_e8213 * (assign8250_e8215 * var_phissat_dn4)));
        var_t0_dn5 = (((-var_a0_dn5) * assign8250_e8215) + (assign8250_e8213 * (assign8250_e8215 * var_phissat_dn5)));
        var_t0_dn6 = (((-var_a0_dn6) * assign8250_e8215) + (assign8250_e8213 * (assign8250_e8215 * var_phissat_dn6)));
        var_t0_dn7 = (((-var_a0_dn7) * assign8250_e8215) + (assign8250_e8213 * (assign8250_e8215 * var_phissat_dn7)));
        var_t0_dn8 = (((-var_a0_dn8) * assign8250_e8215) + (assign8250_e8213 * (assign8250_e8215 * var_phissat_dn8)));

        let assign8260_e8219: f64 = (var_k1_2 * var_q1);
        var_t1 = assign8260_e8219;
        var_t1_dn3 = (var_k1_2 * var_q1_dn3);
        var_t1_dn4 = (var_k1_2 * var_q1_dn4);
        var_t1_dn5 = (var_k1_2 * var_q1_dn5);
        var_t1_dn6 = (var_k1_2 * var_q1_dn6);
        var_t1_dn7 = (var_k1_2 * var_q1_dn7);
        var_t1_dn8 = (var_k1_2 * var_q1_dn8);

        let assign8270_e8222: f64 = (var_t1 * var_q1);
        let assign8270_e8224: f64 = (assign8270_e8222 + var_t0);
        let assign8270_e8226: f64 = (assign8270_e8224 - var_qsqrt);
        let assign8270_e8227: f64 = (-assign8270_e8226);
        let assign8270_e8229: f64 = (-2.0);
        let assign8270_e8231: f64 = (assign8270_e8229 * var_t1);
        let assign8270_e8233: f64 = (assign8270_e8231 + var_t0);
        let assign8270_e8234: f64 = (assign8270_e8227 / assign8270_e8233);
        var_delta = assign8270_e8234;
        var_delta_dn3 = ((((-((((var_t1_dn3 * var_q1) + (var_t1 * var_q1_dn3)) + var_t0_dn3) - var_qsqrt_dn3)) * assign8270_e8233) - (assign8270_e8227 * ((assign8270_e8229 * var_t1_dn3) + var_t0_dn3))) / (assign8270_e8233 * assign8270_e8233));
        var_delta_dn4 = ((((-((((var_t1_dn4 * var_q1) + (var_t1 * var_q1_dn4)) + var_t0_dn4) - var_qsqrt_dn4)) * assign8270_e8233) - (assign8270_e8227 * ((assign8270_e8229 * var_t1_dn4) + var_t0_dn4))) / (assign8270_e8233 * assign8270_e8233));
        var_delta_dn5 = ((((-((((var_t1_dn5 * var_q1) + (var_t1 * var_q1_dn5)) + var_t0_dn5) - var_qsqrt_dn5)) * assign8270_e8233) - (assign8270_e8227 * ((assign8270_e8229 * var_t1_dn5) + var_t0_dn5))) / (assign8270_e8233 * assign8270_e8233));
        var_delta_dn6 = ((((-((((var_t1_dn6 * var_q1) + (var_t1 * var_q1_dn6)) + var_t0_dn6) - var_qsqrt_dn6)) * assign8270_e8233) - (assign8270_e8227 * ((assign8270_e8229 * var_t1_dn6) + var_t0_dn6))) / (assign8270_e8233 * assign8270_e8233));
        var_delta_dn7 = ((((-((((var_t1_dn7 * var_q1) + (var_t1 * var_q1_dn7)) + var_t0_dn7) - var_qsqrt_dn7)) * assign8270_e8233) - (assign8270_e8227 * ((assign8270_e8229 * var_t1_dn7) + var_t0_dn7))) / (assign8270_e8233 * assign8270_e8233));
        var_delta_dn8 = ((((-((((var_t1_dn8 * var_q1) + (var_t1 * var_q1_dn8)) + var_t0_dn8) - var_qsqrt_dn8)) * assign8270_e8233) - (assign8270_e8227 * ((assign8270_e8229 * var_t1_dn8) + var_t0_dn8))) / (assign8270_e8233 * assign8270_e8233));

        let assign8280_e8237: f64 = (var_phissat + var_delta);
        var_phissat = assign8280_e8237;
        var_phissat_dn3 = (var_phissat_dn3 + var_delta_dn3);
        var_phissat_dn4 = (var_phissat_dn4 + var_delta_dn4);
        var_phissat_dn5 = (var_phissat_dn5 + var_delta_dn5);
        var_phissat_dn6 = (var_phissat_dn6 + var_delta_dn6);
        var_phissat_dn7 = (var_phissat_dn7 + var_delta_dn7);
        var_phissat_dn8 = (var_phissat_dn8 + var_delta_dn8);

        let assign8290_e8240: f64 = (var_xg1 - var_phissat);
        var_q1 = assign8290_e8240;
        var_q1_dn3 = (var_xg1_dn3 - var_phissat_dn3);
        var_q1_dn4 = (var_xg1_dn4 - var_phissat_dn4);
        var_q1_dn5 = (var_xg1_dn5 - var_phissat_dn5);
        var_q1_dn6 = (var_xg1_dn6 - var_phissat_dn6);
        var_q1_dn7 = (var_xg1_dn7 - var_phissat_dn7);
        var_q1_dn8 = (var_xg1_dn8 - var_phissat_dn8);

        let assign8300_e8243: f64 = (var_k1_2 * var_q1);
        var_t2 = assign8300_e8243;
        var_t2_dn3 = (var_k1_2 * var_q1_dn3);
        var_t2_dn4 = (var_k1_2 * var_q1_dn4);
        var_t2_dn5 = (var_k1_2 * var_q1_dn5);
        var_t2_dn6 = (var_k1_2 * var_q1_dn6);
        var_t2_dn7 = (var_k1_2 * var_q1_dn7);
        var_t2_dn8 = (var_k1_2 * var_q1_dn8);

        let assign8310_e8247: f64 = (var_t2 * var_q1);
        let assign8310_e8249: f64 = (assign8310_e8247 - var_qsqrt);
        let assign8310_e8250: f64 = (1.0 / assign8310_e8249);
        var_t0 = assign8310_e8250;
        var_t0_dn3 = (-((((var_t2_dn3 * var_q1) + (var_t2 * var_q1_dn3)) - var_qsqrt_dn3) / (assign8310_e8249 * assign8310_e8249)));
        var_t0_dn4 = (-((((var_t2_dn4 * var_q1) + (var_t2 * var_q1_dn4)) - var_qsqrt_dn4) / (assign8310_e8249 * assign8310_e8249)));
        var_t0_dn5 = (-((((var_t2_dn5 * var_q1) + (var_t2 * var_q1_dn5)) - var_qsqrt_dn5) / (assign8310_e8249 * assign8310_e8249)));
        var_t0_dn6 = (-((((var_t2_dn6 * var_q1) + (var_t2 * var_q1_dn6)) - var_qsqrt_dn6) / (assign8310_e8249 * assign8310_e8249)));
        var_t0_dn7 = (-((((var_t2_dn7 * var_q1) + (var_t2 * var_q1_dn7)) - var_qsqrt_dn7) / (assign8310_e8249 * assign8310_e8249)));
        var_t0_dn8 = (-((((var_t2_dn8 * var_q1) + (var_t2 * var_q1_dn8)) - var_qsqrt_dn8) / (assign8310_e8249 * assign8310_e8249)));

        let assign8320_e8253: f64 = (var_t2 * var_q1);
        let assign8320_e8255: f64 = (assign8320_e8253 - var_qsqrt);
        let assign8320_e8256: f64 = (assign8320_e8255).abs();
        let assign8320_e8257: f64 = (assign8320_e8256).ln();
        let assign8320_e8259: f64 = (assign8320_e8257 - var_lna0);
        let assign8320_e8261: f64 = (assign8320_e8259 - var_phissat);
        var_g = assign8320_e8261;
        var_g_dn3 = (((if assign8320_e8255 >= 0.0 { (((var_t2_dn3 * var_q1) + (var_t2 * var_q1_dn3)) - var_qsqrt_dn3) } else { (-(((var_t2_dn3 * var_q1) + (var_t2 * var_q1_dn3)) - var_qsqrt_dn3)) } / assign8320_e8256) - var_lna0_dn3) - var_phissat_dn3);
        var_g_dn4 = (((if assign8320_e8255 >= 0.0 { (((var_t2_dn4 * var_q1) + (var_t2 * var_q1_dn4)) - var_qsqrt_dn4) } else { (-(((var_t2_dn4 * var_q1) + (var_t2 * var_q1_dn4)) - var_qsqrt_dn4)) } / assign8320_e8256) - var_lna0_dn4) - var_phissat_dn4);
        var_g_dn5 = (((if assign8320_e8255 >= 0.0 { (((var_t2_dn5 * var_q1) + (var_t2 * var_q1_dn5)) - var_qsqrt_dn5) } else { (-(((var_t2_dn5 * var_q1) + (var_t2 * var_q1_dn5)) - var_qsqrt_dn5)) } / assign8320_e8256) - var_lna0_dn5) - var_phissat_dn5);
        var_g_dn6 = (((if assign8320_e8255 >= 0.0 { (((var_t2_dn6 * var_q1) + (var_t2 * var_q1_dn6)) - var_qsqrt_dn6) } else { (-(((var_t2_dn6 * var_q1) + (var_t2 * var_q1_dn6)) - var_qsqrt_dn6)) } / assign8320_e8256) - var_lna0_dn6) - var_phissat_dn6);
        var_g_dn7 = (((if assign8320_e8255 >= 0.0 { (((var_t2_dn7 * var_q1) + (var_t2 * var_q1_dn7)) - var_qsqrt_dn7) } else { (-(((var_t2_dn7 * var_q1) + (var_t2 * var_q1_dn7)) - var_qsqrt_dn7)) } / assign8320_e8256) - var_lna0_dn7) - var_phissat_dn7);
        var_g_dn8 = (((if assign8320_e8255 >= 0.0 { (((var_t2_dn8 * var_q1) + (var_t2 * var_q1_dn8)) - var_qsqrt_dn8) } else { (-(((var_t2_dn8 * var_q1) + (var_t2 * var_q1_dn8)) - var_qsqrt_dn8)) } / assign8320_e8256) - var_lna0_dn8) - var_phissat_dn8);

        let assign8330_e8264: f64 = (-2.0);
        let assign8330_e8266: f64 = (assign8330_e8264 * var_t2);
        let assign8330_e8268: f64 = (assign8330_e8266 * var_t0);
        let assign8330_e8270: f64 = (assign8330_e8268 - 1.0);
        let assign8330_e8271: f64 = (1.0 / assign8330_e8270);
        var_dg1 = assign8330_e8271;
        var_dg1_dn3 = (-((((assign8330_e8264 * var_t2_dn3) * var_t0) + (assign8330_e8266 * var_t0_dn3)) / (assign8330_e8270 * assign8330_e8270)));
        var_dg1_dn4 = (-((((assign8330_e8264 * var_t2_dn4) * var_t0) + (assign8330_e8266 * var_t0_dn4)) / (assign8330_e8270 * assign8330_e8270)));
        var_dg1_dn5 = (-((((assign8330_e8264 * var_t2_dn5) * var_t0) + (assign8330_e8266 * var_t0_dn5)) / (assign8330_e8270 * assign8330_e8270)));
        var_dg1_dn6 = (-((((assign8330_e8264 * var_t2_dn6) * var_t0) + (assign8330_e8266 * var_t0_dn6)) / (assign8330_e8270 * assign8330_e8270)));
        var_dg1_dn7 = (-((((assign8330_e8264 * var_t2_dn7) * var_t0) + (assign8330_e8266 * var_t0_dn7)) / (assign8330_e8270 * assign8330_e8270)));
        var_dg1_dn8 = (-((((assign8330_e8264 * var_t2_dn8) * var_t0) + (assign8330_e8266 * var_t0_dn8)) / (assign8330_e8270 * assign8330_e8270)));

        let assign8340_e8273: f64 = (-4.0);
        let assign8340_e8275: f64 = (assign8340_e8273 * var_t2);
        let assign8340_e8277: f64 = (assign8340_e8275 * var_t2);
        let assign8340_e8279: f64 = (assign8340_e8277 * var_t0);
        let assign8340_e8281: f64 = (assign8340_e8279 * var_t0);
        let assign8340_e8284: f64 = (2.0 * var_k1_2);
        let assign8340_e8286: f64 = (assign8340_e8284 * var_t0);
        let assign8340_e8287: f64 = (assign8340_e8281 + assign8340_e8286);
        var_dg2 = assign8340_e8287;
        var_dg2_dn3 = ((((((((assign8340_e8273 * var_t2_dn3) * var_t2) + (assign8340_e8275 * var_t2_dn3)) * var_t0) + (assign8340_e8277 * var_t0_dn3)) * var_t0) + (assign8340_e8279 * var_t0_dn3)) + (assign8340_e8284 * var_t0_dn3));
        var_dg2_dn4 = ((((((((assign8340_e8273 * var_t2_dn4) * var_t2) + (assign8340_e8275 * var_t2_dn4)) * var_t0) + (assign8340_e8277 * var_t0_dn4)) * var_t0) + (assign8340_e8279 * var_t0_dn4)) + (assign8340_e8284 * var_t0_dn4));
        var_dg2_dn5 = ((((((((assign8340_e8273 * var_t2_dn5) * var_t2) + (assign8340_e8275 * var_t2_dn5)) * var_t0) + (assign8340_e8277 * var_t0_dn5)) * var_t0) + (assign8340_e8279 * var_t0_dn5)) + (assign8340_e8284 * var_t0_dn5));
        var_dg2_dn6 = ((((((((assign8340_e8273 * var_t2_dn6) * var_t2) + (assign8340_e8275 * var_t2_dn6)) * var_t0) + (assign8340_e8277 * var_t0_dn6)) * var_t0) + (assign8340_e8279 * var_t0_dn6)) + (assign8340_e8284 * var_t0_dn6));
        var_dg2_dn7 = ((((((((assign8340_e8273 * var_t2_dn7) * var_t2) + (assign8340_e8275 * var_t2_dn7)) * var_t0) + (assign8340_e8277 * var_t0_dn7)) * var_t0) + (assign8340_e8279 * var_t0_dn7)) + (assign8340_e8284 * var_t0_dn7));
        var_dg2_dn8 = ((((((((assign8340_e8273 * var_t2_dn8) * var_t2) + (assign8340_e8275 * var_t2_dn8)) * var_t0) + (assign8340_e8277 * var_t0_dn8)) * var_t0) + (assign8340_e8279 * var_t0_dn8)) + (assign8340_e8284 * var_t0_dn8));

        let assign8350_e8290: f64 = (var_g * var_dg1);
        var_t1 = assign8350_e8290;
        var_t1_dn3 = ((var_g_dn3 * var_dg1) + (var_g * var_dg1_dn3));
        var_t1_dn4 = ((var_g_dn4 * var_dg1) + (var_g * var_dg1_dn4));
        var_t1_dn5 = ((var_g_dn5 * var_dg1) + (var_g * var_dg1_dn5));
        var_t1_dn6 = ((var_g_dn6 * var_dg1) + (var_g * var_dg1_dn6));
        var_t1_dn7 = ((var_g_dn7 * var_dg1) + (var_g * var_dg1_dn7));
        var_t1_dn8 = ((var_g_dn8 * var_dg1) + (var_g * var_dg1_dn8));

        let assign8360_e8292: f64 = (-var_t1);
        let assign8360_e8295: f64 = (0.5 * var_t1);
        let assign8360_e8297: f64 = (assign8360_e8295 * var_t1);
        let assign8360_e8299: f64 = (assign8360_e8297 * var_dg2);
        let assign8360_e8301: f64 = (assign8360_e8299 * var_dg1);
        let assign8360_e8302: f64 = (assign8360_e8292 - assign8360_e8301);
        var_delta = assign8360_e8302;
        var_delta_dn3 = ((-var_t1_dn3) - (((((((0.5 * var_t1_dn3) * var_t1) + (assign8360_e8295 * var_t1_dn3)) * var_dg2) + (assign8360_e8297 * var_dg2_dn3)) * var_dg1) + (assign8360_e8299 * var_dg1_dn3)));
        var_delta_dn4 = ((-var_t1_dn4) - (((((((0.5 * var_t1_dn4) * var_t1) + (assign8360_e8295 * var_t1_dn4)) * var_dg2) + (assign8360_e8297 * var_dg2_dn4)) * var_dg1) + (assign8360_e8299 * var_dg1_dn4)));
        var_delta_dn5 = ((-var_t1_dn5) - (((((((0.5 * var_t1_dn5) * var_t1) + (assign8360_e8295 * var_t1_dn5)) * var_dg2) + (assign8360_e8297 * var_dg2_dn5)) * var_dg1) + (assign8360_e8299 * var_dg1_dn5)));
        var_delta_dn6 = ((-var_t1_dn6) - (((((((0.5 * var_t1_dn6) * var_t1) + (assign8360_e8295 * var_t1_dn6)) * var_dg2) + (assign8360_e8297 * var_dg2_dn6)) * var_dg1) + (assign8360_e8299 * var_dg1_dn6)));
        var_delta_dn7 = ((-var_t1_dn7) - (((((((0.5 * var_t1_dn7) * var_t1) + (assign8360_e8295 * var_t1_dn7)) * var_dg2) + (assign8360_e8297 * var_dg2_dn7)) * var_dg1) + (assign8360_e8299 * var_dg1_dn7)));
        var_delta_dn8 = ((-var_t1_dn8) - (((((((0.5 * var_t1_dn8) * var_t1) + (assign8360_e8295 * var_t1_dn8)) * var_dg2) + (assign8360_e8297 * var_dg2_dn8)) * var_dg1) + (assign8360_e8299 * var_dg1_dn8)));

        let assign8370_e8305: f64 = (-10.0);
        let assign8370_e8306: f64 = (var_delta).max(assign8370_e8305);
        var_delta = assign8370_e8306;
        var_delta_dn3 = if var_delta >= assign8370_e8305 { var_delta_dn3 } else { 0.0 };
        var_delta_dn4 = if var_delta >= assign8370_e8305 { var_delta_dn4 } else { 0.0 };
        var_delta_dn5 = if var_delta >= assign8370_e8305 { var_delta_dn5 } else { 0.0 };
        var_delta_dn6 = if var_delta >= assign8370_e8305 { var_delta_dn6 } else { 0.0 };
        var_delta_dn7 = if var_delta >= assign8370_e8305 { var_delta_dn7 } else { 0.0 };
        var_delta_dn8 = if var_delta >= assign8370_e8305 { var_delta_dn8 } else { 0.0 };

        let assign8380_e8309: f64 = (var_delta).min(10.0);
        var_delta = assign8380_e8309;
        var_delta_dn3 = if var_delta <= 10.0 { var_delta_dn3 } else { 0.0 };
        var_delta_dn4 = if var_delta <= 10.0 { var_delta_dn4 } else { 0.0 };
        var_delta_dn5 = if var_delta <= 10.0 { var_delta_dn5 } else { 0.0 };
        var_delta_dn6 = if var_delta <= 10.0 { var_delta_dn6 } else { 0.0 };
        var_delta_dn7 = if var_delta <= 10.0 { var_delta_dn7 } else { 0.0 };
        var_delta_dn8 = if var_delta <= 10.0 { var_delta_dn8 } else { 0.0 };

        let assign8390_e8312: f64 = (var_phissat + var_delta);
        var_phissat = assign8390_e8312;
        var_phissat_dn3 = (var_phissat_dn3 + var_delta_dn3);
        var_phissat_dn4 = (var_phissat_dn4 + var_delta_dn4);
        var_phissat_dn5 = (var_phissat_dn5 + var_delta_dn5);
        var_phissat_dn6 = (var_phissat_dn6 + var_delta_dn6);
        var_phissat_dn7 = (var_phissat_dn7 + var_delta_dn7);
        var_phissat_dn8 = (var_phissat_dn8 + var_delta_dn8);

        let assign8400_e8315: f64 = (var_xg1 - var_phissat);
        var_q1 = assign8400_e8315;
        var_q1_dn3 = (var_xg1_dn3 - var_phissat_dn3);
        var_q1_dn4 = (var_xg1_dn4 - var_phissat_dn4);
        var_q1_dn5 = (var_xg1_dn5 - var_phissat_dn5);
        var_q1_dn6 = (var_xg1_dn6 - var_phissat_dn6);
        var_q1_dn7 = (var_xg1_dn7 - var_phissat_dn7);
        var_q1_dn8 = (var_xg1_dn8 - var_phissat_dn8);

        let assign8410_e8318: f64 = (var_k1_2 * var_q1);
        var_t2 = assign8410_e8318;
        var_t2_dn3 = (var_k1_2 * var_q1_dn3);
        var_t2_dn4 = (var_k1_2 * var_q1_dn4);
        var_t2_dn5 = (var_k1_2 * var_q1_dn5);
        var_t2_dn6 = (var_k1_2 * var_q1_dn6);
        var_t2_dn7 = (var_k1_2 * var_q1_dn7);
        var_t2_dn8 = (var_k1_2 * var_q1_dn8);

        let assign8420_e8322: f64 = (var_t2 * var_q1);
        let assign8420_e8324: f64 = (assign8420_e8322 - var_qsqrt);
        let assign8420_e8325: f64 = (1.0 / assign8420_e8324);
        var_t0 = assign8420_e8325;
        var_t0_dn3 = (-((((var_t2_dn3 * var_q1) + (var_t2 * var_q1_dn3)) - var_qsqrt_dn3) / (assign8420_e8324 * assign8420_e8324)));
        var_t0_dn4 = (-((((var_t2_dn4 * var_q1) + (var_t2 * var_q1_dn4)) - var_qsqrt_dn4) / (assign8420_e8324 * assign8420_e8324)));
        var_t0_dn5 = (-((((var_t2_dn5 * var_q1) + (var_t2 * var_q1_dn5)) - var_qsqrt_dn5) / (assign8420_e8324 * assign8420_e8324)));
        var_t0_dn6 = (-((((var_t2_dn6 * var_q1) + (var_t2 * var_q1_dn6)) - var_qsqrt_dn6) / (assign8420_e8324 * assign8420_e8324)));
        var_t0_dn7 = (-((((var_t2_dn7 * var_q1) + (var_t2 * var_q1_dn7)) - var_qsqrt_dn7) / (assign8420_e8324 * assign8420_e8324)));
        var_t0_dn8 = (-((((var_t2_dn8 * var_q1) + (var_t2 * var_q1_dn8)) - var_qsqrt_dn8) / (assign8420_e8324 * assign8420_e8324)));

        let assign8430_e8328: f64 = (var_t2 * var_q1);
        let assign8430_e8330: f64 = (assign8430_e8328 - var_qsqrt);
        let assign8430_e8331: f64 = (assign8430_e8330).abs();
        let assign8430_e8332: f64 = (assign8430_e8331).ln();
        let assign8430_e8334: f64 = (assign8430_e8332 - var_lna0);
        let assign8430_e8336: f64 = (assign8430_e8334 - var_phissat);
        var_g = assign8430_e8336;
        var_g_dn3 = (((if assign8430_e8330 >= 0.0 { (((var_t2_dn3 * var_q1) + (var_t2 * var_q1_dn3)) - var_qsqrt_dn3) } else { (-(((var_t2_dn3 * var_q1) + (var_t2 * var_q1_dn3)) - var_qsqrt_dn3)) } / assign8430_e8331) - var_lna0_dn3) - var_phissat_dn3);
        var_g_dn4 = (((if assign8430_e8330 >= 0.0 { (((var_t2_dn4 * var_q1) + (var_t2 * var_q1_dn4)) - var_qsqrt_dn4) } else { (-(((var_t2_dn4 * var_q1) + (var_t2 * var_q1_dn4)) - var_qsqrt_dn4)) } / assign8430_e8331) - var_lna0_dn4) - var_phissat_dn4);
        var_g_dn5 = (((if assign8430_e8330 >= 0.0 { (((var_t2_dn5 * var_q1) + (var_t2 * var_q1_dn5)) - var_qsqrt_dn5) } else { (-(((var_t2_dn5 * var_q1) + (var_t2 * var_q1_dn5)) - var_qsqrt_dn5)) } / assign8430_e8331) - var_lna0_dn5) - var_phissat_dn5);
        var_g_dn6 = (((if assign8430_e8330 >= 0.0 { (((var_t2_dn6 * var_q1) + (var_t2 * var_q1_dn6)) - var_qsqrt_dn6) } else { (-(((var_t2_dn6 * var_q1) + (var_t2 * var_q1_dn6)) - var_qsqrt_dn6)) } / assign8430_e8331) - var_lna0_dn6) - var_phissat_dn6);
        var_g_dn7 = (((if assign8430_e8330 >= 0.0 { (((var_t2_dn7 * var_q1) + (var_t2 * var_q1_dn7)) - var_qsqrt_dn7) } else { (-(((var_t2_dn7 * var_q1) + (var_t2 * var_q1_dn7)) - var_qsqrt_dn7)) } / assign8430_e8331) - var_lna0_dn7) - var_phissat_dn7);
        var_g_dn8 = (((if assign8430_e8330 >= 0.0 { (((var_t2_dn8 * var_q1) + (var_t2 * var_q1_dn8)) - var_qsqrt_dn8) } else { (-(((var_t2_dn8 * var_q1) + (var_t2 * var_q1_dn8)) - var_qsqrt_dn8)) } / assign8430_e8331) - var_lna0_dn8) - var_phissat_dn8);

        let assign8440_e8339: f64 = (-2.0);
        let assign8440_e8341: f64 = (assign8440_e8339 * var_t2);
        let assign8440_e8343: f64 = (assign8440_e8341 * var_t0);
        let assign8440_e8345: f64 = (assign8440_e8343 - 1.0);
        let assign8440_e8346: f64 = (1.0 / assign8440_e8345);
        var_dg1 = assign8440_e8346;
        var_dg1_dn3 = (-((((assign8440_e8339 * var_t2_dn3) * var_t0) + (assign8440_e8341 * var_t0_dn3)) / (assign8440_e8345 * assign8440_e8345)));
        var_dg1_dn4 = (-((((assign8440_e8339 * var_t2_dn4) * var_t0) + (assign8440_e8341 * var_t0_dn4)) / (assign8440_e8345 * assign8440_e8345)));
        var_dg1_dn5 = (-((((assign8440_e8339 * var_t2_dn5) * var_t0) + (assign8440_e8341 * var_t0_dn5)) / (assign8440_e8345 * assign8440_e8345)));
        var_dg1_dn6 = (-((((assign8440_e8339 * var_t2_dn6) * var_t0) + (assign8440_e8341 * var_t0_dn6)) / (assign8440_e8345 * assign8440_e8345)));
        var_dg1_dn7 = (-((((assign8440_e8339 * var_t2_dn7) * var_t0) + (assign8440_e8341 * var_t0_dn7)) / (assign8440_e8345 * assign8440_e8345)));
        var_dg1_dn8 = (-((((assign8440_e8339 * var_t2_dn8) * var_t0) + (assign8440_e8341 * var_t0_dn8)) / (assign8440_e8345 * assign8440_e8345)));

        *var_delta_slot = var_delta;
        *var_delta_dn3_slot = var_delta_dn3;
        *var_delta_dn4_slot = var_delta_dn4;
        *var_delta_dn5_slot = var_delta_dn5;
        *var_delta_dn6_slot = var_delta_dn6;
        *var_delta_dn7_slot = var_delta_dn7;
        *var_delta_dn8_slot = var_delta_dn8;
        *var_dg1_slot = var_dg1;
        *var_dg1_dn3_slot = var_dg1_dn3;
        *var_dg1_dn4_slot = var_dg1_dn4;
        *var_dg1_dn5_slot = var_dg1_dn5;
        *var_dg1_dn6_slot = var_dg1_dn6;
        *var_dg1_dn7_slot = var_dg1_dn7;
        *var_dg1_dn8_slot = var_dg1_dn8;
        *var_dg2_slot = var_dg2;
        *var_dg2_dn3_slot = var_dg2_dn3;
        *var_dg2_dn4_slot = var_dg2_dn4;
        *var_dg2_dn5_slot = var_dg2_dn5;
        *var_dg2_dn6_slot = var_dg2_dn6;
        *var_dg2_dn7_slot = var_dg2_dn7;
        *var_dg2_dn8_slot = var_dg2_dn8;
        *var_g_slot = var_g;
        *var_g_dn3_slot = var_g_dn3;
        *var_g_dn4_slot = var_g_dn4;
        *var_g_dn5_slot = var_g_dn5;
        *var_g_dn6_slot = var_g_dn6;
        *var_g_dn7_slot = var_g_dn7;
        *var_g_dn8_slot = var_g_dn8;
        *var_phissat_slot = var_phissat;
        *var_phissat_dn3_slot = var_phissat_dn3;
        *var_phissat_dn4_slot = var_phissat_dn4;
        *var_phissat_dn5_slot = var_phissat_dn5;
        *var_phissat_dn6_slot = var_phissat_dn6;
        *var_phissat_dn7_slot = var_phissat_dn7;
        *var_phissat_dn8_slot = var_phissat_dn8;
        *var_q1_slot = var_q1;
        *var_q1_dn3_slot = var_q1_dn3;
        *var_q1_dn4_slot = var_q1_dn4;
        *var_q1_dn5_slot = var_q1_dn5;
        *var_q1_dn6_slot = var_q1_dn6;
        *var_q1_dn7_slot = var_q1_dn7;
        *var_q1_dn8_slot = var_q1_dn8;
        *var_qsqrt_slot = var_qsqrt;
        *var_qsqrt_dn3_slot = var_qsqrt_dn3;
        *var_qsqrt_dn4_slot = var_qsqrt_dn4;
        *var_qsqrt_dn5_slot = var_qsqrt_dn5;
        *var_qsqrt_dn6_slot = var_qsqrt_dn6;
        *var_qsqrt_dn7_slot = var_qsqrt_dn7;
        *var_qsqrt_dn8_slot = var_qsqrt_dn8;
        *var_qt_slot = var_qt;
        *var_qt_dn3_slot = var_qt_dn3;
        *var_qt_dn4_slot = var_qt_dn4;
        *var_qt_dn5_slot = var_qt_dn5;
        *var_qt_dn6_slot = var_qt_dn6;
        *var_qt_dn7_slot = var_qt_dn7;
        *var_qt_dn8_slot = var_qt_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t4_slot = var_t4;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t5_slot = var_t5;
        *var_t5_dn3_slot = var_t5_dn3;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t6_slot = var_t6;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_xg1_slot = var_xg1;
        *var_xg1_dn3_slot = var_xg1_dn3;
        *var_xg1_dn4_slot = var_xg1_dn4;
        *var_xg1_dn5_slot = var_xg1_dn5;
        *var_xg1_dn6_slot = var_xg1_dn6;
        *var_xg1_dn7_slot = var_xg1_dn7;
        *var_xg1_dn8_slot = var_xg1_dn8;
    }

    pub(super) fn stamp_transient_block_17(
        var_a0: f64,
        var_a0_dn3: f64,
        var_a0_dn4: f64,
        var_a0_dn5: f64,
        var_a0_dn6: f64,
        var_a0_dn7: f64,
        var_a0_dn8: f64,
        var_dg1: f64,
        var_dg1_dn3: f64,
        var_dg1_dn4: f64,
        var_dg1_dn5: f64,
        var_dg1_dn6: f64,
        var_dg1_dn7: f64,
        var_dg1_dn8: f64,
        var_g: f64,
        var_g_dn3: f64,
        var_g_dn4: f64,
        var_g_dn5: f64,
        var_g_dn6: f64,
        var_g_dn7: f64,
        var_g_dn8: f64,
        var_k1: f64,
        var_k1_2: f64,
        var_k2: f64,
        var_nvtm: f64,
        var_nvtm_dn3: f64,
        var_nvtm_dn4: f64,
        var_nvtm_dn5: f64,
        var_nvtm_dn6: f64,
        var_nvtm_dn7: f64,
        var_nvtm_dn8: f64,
        var_phi1_0: f64,
        var_phi1_0_dn3: f64,
        var_phi1_0_dn4: f64,
        var_phi1_0_dn5: f64,
        var_phi1_0_dn6: f64,
        var_phi1_0_dn7: f64,
        var_phi1_0_dn8: f64,
        var_vdseff: f64,
        var_vdseff_dn3: f64,
        var_vdseff_dn4: f64,
        var_vdseff_dn5: f64,
        var_vdseff_dn6: f64,
        var_vdseff_dn7: f64,
        var_vdseff_dn8: f64,
        var_vgfb1eff: f64,
        var_vgfb1eff_dn3: f64,
        var_vgfb1eff_dn4: f64,
        var_vgfb1eff_dn5: f64,
        var_vgfb1eff_dn6: f64,
        var_vgfb1eff_dn7: f64,
        var_vgfb1eff_dn8: f64,
        var_xg2: f64,
        var_xg2_dn3: f64,
        var_xg2_dn4: f64,
        var_xg2_dn5: f64,
        var_xg2_dn6: f64,
        var_xg2_dn7: f64,
        var_xg2_dn8: f64,
        var_aaux_slot: &mut f64,
        var_aaux_dn3_slot: &mut f64,
        var_aaux_dn4_slot: &mut f64,
        var_aaux_dn5_slot: &mut f64,
        var_aaux_dn6_slot: &mut f64,
        var_aaux_dn7_slot: &mut f64,
        var_aaux_dn8_slot: &mut f64,
        var_auxb1_slot: &mut f64,
        var_auxb1_dn3_slot: &mut f64,
        var_auxb1_dn4_slot: &mut f64,
        var_auxb1_dn5_slot: &mut f64,
        var_auxb1_dn6_slot: &mut f64,
        var_auxb1_dn7_slot: &mut f64,
        var_auxb1_dn8_slot: &mut f64,
        var_coth1_slot: &mut f64,
        var_coth1_dn3_slot: &mut f64,
        var_coth1_dn4_slot: &mut f64,
        var_coth1_dn5_slot: &mut f64,
        var_coth1_dn6_slot: &mut f64,
        var_coth1_dn7_slot: &mut f64,
        var_coth1_dn8_slot: &mut f64,
        var_csc1_slot: &mut f64,
        var_csc1_dn3_slot: &mut f64,
        var_csc1_dn4_slot: &mut f64,
        var_csc1_dn5_slot: &mut f64,
        var_csc1_dn6_slot: &mut f64,
        var_csc1_dn7_slot: &mut f64,
        var_csc1_dn8_slot: &mut f64,
        var_delta_slot: &mut f64,
        var_delta_dn3_slot: &mut f64,
        var_delta_dn4_slot: &mut f64,
        var_delta_dn5_slot: &mut f64,
        var_delta_dn6_slot: &mut f64,
        var_delta_dn7_slot: &mut f64,
        var_delta_dn8_slot: &mut f64,
        var_df_slot: &mut f64,
        var_df_dn3_slot: &mut f64,
        var_df_dn4_slot: &mut f64,
        var_df_dn5_slot: &mut f64,
        var_df_dn6_slot: &mut f64,
        var_df_dn7_slot: &mut f64,
        var_df_dn8_slot: &mut f64,
        var_dg2_slot: &mut f64,
        var_dg2_dn3_slot: &mut f64,
        var_dg2_dn4_slot: &mut f64,
        var_dg2_dn5_slot: &mut f64,
        var_dg2_dn6_slot: &mut f64,
        var_dg2_dn7_slot: &mut f64,
        var_dg2_dn8_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn3_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn4_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn5_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn6_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn7_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn8_slot: &mut f64,
        var_dq2_slot: &mut f64,
        var_dq2_dn3_slot: &mut f64,
        var_dq2_dn4_slot: &mut f64,
        var_dq2_dn5_slot: &mut f64,
        var_dq2_dn6_slot: &mut f64,
        var_dq2_dn7_slot: &mut f64,
        var_dq2_dn8_slot: &mut f64,
        var_dqcoth_slot: &mut f64,
        var_dqcoth_dn3_slot: &mut f64,
        var_dqcoth_dn4_slot: &mut f64,
        var_dqcoth_dn5_slot: &mut f64,
        var_dqcoth_dn6_slot: &mut f64,
        var_dqcoth_dn7_slot: &mut f64,
        var_dqcoth_dn8_slot: &mut f64,
        var_dqcothqdqsqrt_slot: &mut f64,
        var_dqcothqdqsqrt_dn3_slot: &mut f64,
        var_dqcothqdqsqrt_dn4_slot: &mut f64,
        var_dqcothqdqsqrt_dn5_slot: &mut f64,
        var_dqcothqdqsqrt_dn6_slot: &mut f64,
        var_dqcothqdqsqrt_dn7_slot: &mut f64,
        var_dqcothqdqsqrt_dn8_slot: &mut f64,
        var_dqsqrt_slot: &mut f64,
        var_dqsqrt_dn3_slot: &mut f64,
        var_dqsqrt_dn4_slot: &mut f64,
        var_dqsqrt_dn5_slot: &mut f64,
        var_dqsqrt_dn6_slot: &mut f64,
        var_dqsqrt_dn7_slot: &mut f64,
        var_dqsqrt_dn8_slot: &mut f64,
        var_f_slot: &mut f64,
        var_f_dn3_slot: &mut f64,
        var_f_dn4_slot: &mut f64,
        var_f_dn5_slot: &mut f64,
        var_f_dn6_slot: &mut f64,
        var_f_dn7_slot: &mut f64,
        var_f_dn8_slot: &mut f64,
        var_guard92_slot: &mut f64,
        var_phi1_slot: &mut f64,
        var_phi1_dn3_slot: &mut f64,
        var_phi1_dn4_slot: &mut f64,
        var_phi1_dn5_slot: &mut f64,
        var_phi1_dn6_slot: &mut f64,
        var_phi1_dn7_slot: &mut f64,
        var_phi1_dn8_slot: &mut f64,
        var_phissat_slot: &mut f64,
        var_phissat_dn3_slot: &mut f64,
        var_phissat_dn4_slot: &mut f64,
        var_phissat_dn5_slot: &mut f64,
        var_phissat_dn6_slot: &mut f64,
        var_phissat_dn7_slot: &mut f64,
        var_phissat_dn8_slot: &mut f64,
        var_q_slot: &mut f64,
        var_q1_slot: &mut f64,
        var_q1_dn3_slot: &mut f64,
        var_q1_dn4_slot: &mut f64,
        var_q1_dn5_slot: &mut f64,
        var_q1_dn6_slot: &mut f64,
        var_q1_dn7_slot: &mut f64,
        var_q1_dn8_slot: &mut f64,
        var_q2_slot: &mut f64,
        var_q2_dn3_slot: &mut f64,
        var_q2_dn4_slot: &mut f64,
        var_q2_dn5_slot: &mut f64,
        var_q2_dn6_slot: &mut f64,
        var_q2_dn7_slot: &mut f64,
        var_q2_dn8_slot: &mut f64,
        var_q_dn3_slot: &mut f64,
        var_q_dn4_slot: &mut f64,
        var_q_dn5_slot: &mut f64,
        var_q_dn6_slot: &mut f64,
        var_q_dn7_slot: &mut f64,
        var_q_dn8_slot: &mut f64,
        var_qcoth_slot: &mut f64,
        var_qcoth_dn3_slot: &mut f64,
        var_qcoth_dn4_slot: &mut f64,
        var_qcoth_dn5_slot: &mut f64,
        var_qcoth_dn6_slot: &mut f64,
        var_qcoth_dn7_slot: &mut f64,
        var_qcoth_dn8_slot: &mut f64,
        var_qsqrt_slot: &mut f64,
        var_qsqrt_dn3_slot: &mut f64,
        var_qsqrt_dn4_slot: &mut f64,
        var_qsqrt_dn5_slot: &mut f64,
        var_qsqrt_dn6_slot: &mut f64,
        var_qsqrt_dn7_slot: &mut f64,
        var_qsqrt_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_xg1_slot: &mut f64,
        var_xg1_dn3_slot: &mut f64,
        var_xg1_dn4_slot: &mut f64,
        var_xg1_dn5_slot: &mut f64,
        var_xg1_dn6_slot: &mut f64,
        var_xg1_dn7_slot: &mut f64,
        var_xg1_dn8_slot: &mut f64,
    ) {
        let mut var_aaux: f64 = *var_aaux_slot;
        let mut var_aaux_dn3: f64 = *var_aaux_dn3_slot;
        let mut var_aaux_dn4: f64 = *var_aaux_dn4_slot;
        let mut var_aaux_dn5: f64 = *var_aaux_dn5_slot;
        let mut var_aaux_dn6: f64 = *var_aaux_dn6_slot;
        let mut var_aaux_dn7: f64 = *var_aaux_dn7_slot;
        let mut var_aaux_dn8: f64 = *var_aaux_dn8_slot;
        let mut var_auxb1: f64 = *var_auxb1_slot;
        let mut var_auxb1_dn3: f64 = *var_auxb1_dn3_slot;
        let mut var_auxb1_dn4: f64 = *var_auxb1_dn4_slot;
        let mut var_auxb1_dn5: f64 = *var_auxb1_dn5_slot;
        let mut var_auxb1_dn6: f64 = *var_auxb1_dn6_slot;
        let mut var_auxb1_dn7: f64 = *var_auxb1_dn7_slot;
        let mut var_auxb1_dn8: f64 = *var_auxb1_dn8_slot;
        let mut var_coth1: f64 = *var_coth1_slot;
        let mut var_coth1_dn3: f64 = *var_coth1_dn3_slot;
        let mut var_coth1_dn4: f64 = *var_coth1_dn4_slot;
        let mut var_coth1_dn5: f64 = *var_coth1_dn5_slot;
        let mut var_coth1_dn6: f64 = *var_coth1_dn6_slot;
        let mut var_coth1_dn7: f64 = *var_coth1_dn7_slot;
        let mut var_coth1_dn8: f64 = *var_coth1_dn8_slot;
        let mut var_csc1: f64 = *var_csc1_slot;
        let mut var_csc1_dn3: f64 = *var_csc1_dn3_slot;
        let mut var_csc1_dn4: f64 = *var_csc1_dn4_slot;
        let mut var_csc1_dn5: f64 = *var_csc1_dn5_slot;
        let mut var_csc1_dn6: f64 = *var_csc1_dn6_slot;
        let mut var_csc1_dn7: f64 = *var_csc1_dn7_slot;
        let mut var_csc1_dn8: f64 = *var_csc1_dn8_slot;
        let mut var_delta: f64 = *var_delta_slot;
        let mut var_delta_dn3: f64 = *var_delta_dn3_slot;
        let mut var_delta_dn4: f64 = *var_delta_dn4_slot;
        let mut var_delta_dn5: f64 = *var_delta_dn5_slot;
        let mut var_delta_dn6: f64 = *var_delta_dn6_slot;
        let mut var_delta_dn7: f64 = *var_delta_dn7_slot;
        let mut var_delta_dn8: f64 = *var_delta_dn8_slot;
        let mut var_df: f64 = *var_df_slot;
        let mut var_df_dn3: f64 = *var_df_dn3_slot;
        let mut var_df_dn4: f64 = *var_df_dn4_slot;
        let mut var_df_dn5: f64 = *var_df_dn5_slot;
        let mut var_df_dn6: f64 = *var_df_dn6_slot;
        let mut var_df_dn7: f64 = *var_df_dn7_slot;
        let mut var_df_dn8: f64 = *var_df_dn8_slot;
        let mut var_dg2: f64 = *var_dg2_slot;
        let mut var_dg2_dn3: f64 = *var_dg2_dn3_slot;
        let mut var_dg2_dn4: f64 = *var_dg2_dn4_slot;
        let mut var_dg2_dn5: f64 = *var_dg2_dn5_slot;
        let mut var_dg2_dn6: f64 = *var_dg2_dn6_slot;
        let mut var_dg2_dn7: f64 = *var_dg2_dn7_slot;
        let mut var_dg2_dn8: f64 = *var_dg2_dn8_slot;
        let mut var_dlogsinhqsqdqsqrt: f64 = *var_dlogsinhqsqdqsqrt_slot;
        let mut var_dlogsinhqsqdqsqrt_dn3: f64 = *var_dlogsinhqsqdqsqrt_dn3_slot;
        let mut var_dlogsinhqsqdqsqrt_dn4: f64 = *var_dlogsinhqsqdqsqrt_dn4_slot;
        let mut var_dlogsinhqsqdqsqrt_dn5: f64 = *var_dlogsinhqsqdqsqrt_dn5_slot;
        let mut var_dlogsinhqsqdqsqrt_dn6: f64 = *var_dlogsinhqsqdqsqrt_dn6_slot;
        let mut var_dlogsinhqsqdqsqrt_dn7: f64 = *var_dlogsinhqsqdqsqrt_dn7_slot;
        let mut var_dlogsinhqsqdqsqrt_dn8: f64 = *var_dlogsinhqsqdqsqrt_dn8_slot;
        let mut var_dq2: f64 = *var_dq2_slot;
        let mut var_dq2_dn3: f64 = *var_dq2_dn3_slot;
        let mut var_dq2_dn4: f64 = *var_dq2_dn4_slot;
        let mut var_dq2_dn5: f64 = *var_dq2_dn5_slot;
        let mut var_dq2_dn6: f64 = *var_dq2_dn6_slot;
        let mut var_dq2_dn7: f64 = *var_dq2_dn7_slot;
        let mut var_dq2_dn8: f64 = *var_dq2_dn8_slot;
        let mut var_dqcoth: f64 = *var_dqcoth_slot;
        let mut var_dqcoth_dn3: f64 = *var_dqcoth_dn3_slot;
        let mut var_dqcoth_dn4: f64 = *var_dqcoth_dn4_slot;
        let mut var_dqcoth_dn5: f64 = *var_dqcoth_dn5_slot;
        let mut var_dqcoth_dn6: f64 = *var_dqcoth_dn6_slot;
        let mut var_dqcoth_dn7: f64 = *var_dqcoth_dn7_slot;
        let mut var_dqcoth_dn8: f64 = *var_dqcoth_dn8_slot;
        let mut var_dqcothqdqsqrt: f64 = *var_dqcothqdqsqrt_slot;
        let mut var_dqcothqdqsqrt_dn3: f64 = *var_dqcothqdqsqrt_dn3_slot;
        let mut var_dqcothqdqsqrt_dn4: f64 = *var_dqcothqdqsqrt_dn4_slot;
        let mut var_dqcothqdqsqrt_dn5: f64 = *var_dqcothqdqsqrt_dn5_slot;
        let mut var_dqcothqdqsqrt_dn6: f64 = *var_dqcothqdqsqrt_dn6_slot;
        let mut var_dqcothqdqsqrt_dn7: f64 = *var_dqcothqdqsqrt_dn7_slot;
        let mut var_dqcothqdqsqrt_dn8: f64 = *var_dqcothqdqsqrt_dn8_slot;
        let mut var_dqsqrt: f64 = *var_dqsqrt_slot;
        let mut var_dqsqrt_dn3: f64 = *var_dqsqrt_dn3_slot;
        let mut var_dqsqrt_dn4: f64 = *var_dqsqrt_dn4_slot;
        let mut var_dqsqrt_dn5: f64 = *var_dqsqrt_dn5_slot;
        let mut var_dqsqrt_dn6: f64 = *var_dqsqrt_dn6_slot;
        let mut var_dqsqrt_dn7: f64 = *var_dqsqrt_dn7_slot;
        let mut var_dqsqrt_dn8: f64 = *var_dqsqrt_dn8_slot;
        let mut var_f: f64 = *var_f_slot;
        let mut var_f_dn3: f64 = *var_f_dn3_slot;
        let mut var_f_dn4: f64 = *var_f_dn4_slot;
        let mut var_f_dn5: f64 = *var_f_dn5_slot;
        let mut var_f_dn6: f64 = *var_f_dn6_slot;
        let mut var_f_dn7: f64 = *var_f_dn7_slot;
        let mut var_f_dn8: f64 = *var_f_dn8_slot;
        let mut var_guard92: f64 = *var_guard92_slot;
        let mut var_phi1: f64 = *var_phi1_slot;
        let mut var_phi1_dn3: f64 = *var_phi1_dn3_slot;
        let mut var_phi1_dn4: f64 = *var_phi1_dn4_slot;
        let mut var_phi1_dn5: f64 = *var_phi1_dn5_slot;
        let mut var_phi1_dn6: f64 = *var_phi1_dn6_slot;
        let mut var_phi1_dn7: f64 = *var_phi1_dn7_slot;
        let mut var_phi1_dn8: f64 = *var_phi1_dn8_slot;
        let mut var_phissat: f64 = *var_phissat_slot;
        let mut var_phissat_dn3: f64 = *var_phissat_dn3_slot;
        let mut var_phissat_dn4: f64 = *var_phissat_dn4_slot;
        let mut var_phissat_dn5: f64 = *var_phissat_dn5_slot;
        let mut var_phissat_dn6: f64 = *var_phissat_dn6_slot;
        let mut var_phissat_dn7: f64 = *var_phissat_dn7_slot;
        let mut var_phissat_dn8: f64 = *var_phissat_dn8_slot;
        let mut var_q: f64 = *var_q_slot;
        let mut var_q1: f64 = *var_q1_slot;
        let mut var_q1_dn3: f64 = *var_q1_dn3_slot;
        let mut var_q1_dn4: f64 = *var_q1_dn4_slot;
        let mut var_q1_dn5: f64 = *var_q1_dn5_slot;
        let mut var_q1_dn6: f64 = *var_q1_dn6_slot;
        let mut var_q1_dn7: f64 = *var_q1_dn7_slot;
        let mut var_q1_dn8: f64 = *var_q1_dn8_slot;
        let mut var_q2: f64 = *var_q2_slot;
        let mut var_q2_dn3: f64 = *var_q2_dn3_slot;
        let mut var_q2_dn4: f64 = *var_q2_dn4_slot;
        let mut var_q2_dn5: f64 = *var_q2_dn5_slot;
        let mut var_q2_dn6: f64 = *var_q2_dn6_slot;
        let mut var_q2_dn7: f64 = *var_q2_dn7_slot;
        let mut var_q2_dn8: f64 = *var_q2_dn8_slot;
        let mut var_q_dn3: f64 = *var_q_dn3_slot;
        let mut var_q_dn4: f64 = *var_q_dn4_slot;
        let mut var_q_dn5: f64 = *var_q_dn5_slot;
        let mut var_q_dn6: f64 = *var_q_dn6_slot;
        let mut var_q_dn7: f64 = *var_q_dn7_slot;
        let mut var_q_dn8: f64 = *var_q_dn8_slot;
        let mut var_qcoth: f64 = *var_qcoth_slot;
        let mut var_qcoth_dn3: f64 = *var_qcoth_dn3_slot;
        let mut var_qcoth_dn4: f64 = *var_qcoth_dn4_slot;
        let mut var_qcoth_dn5: f64 = *var_qcoth_dn5_slot;
        let mut var_qcoth_dn6: f64 = *var_qcoth_dn6_slot;
        let mut var_qcoth_dn7: f64 = *var_qcoth_dn7_slot;
        let mut var_qcoth_dn8: f64 = *var_qcoth_dn8_slot;
        let mut var_qsqrt: f64 = *var_qsqrt_slot;
        let mut var_qsqrt_dn3: f64 = *var_qsqrt_dn3_slot;
        let mut var_qsqrt_dn4: f64 = *var_qsqrt_dn4_slot;
        let mut var_qsqrt_dn5: f64 = *var_qsqrt_dn5_slot;
        let mut var_qsqrt_dn6: f64 = *var_qsqrt_dn6_slot;
        let mut var_qsqrt_dn7: f64 = *var_qsqrt_dn7_slot;
        let mut var_qsqrt_dn8: f64 = *var_qsqrt_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_xg1: f64 = *var_xg1_slot;
        let mut var_xg1_dn3: f64 = *var_xg1_dn3_slot;
        let mut var_xg1_dn4: f64 = *var_xg1_dn4_slot;
        let mut var_xg1_dn5: f64 = *var_xg1_dn5_slot;
        let mut var_xg1_dn6: f64 = *var_xg1_dn6_slot;
        let mut var_xg1_dn7: f64 = *var_xg1_dn7_slot;
        let mut var_xg1_dn8: f64 = *var_xg1_dn8_slot;

        let assign8450_e8348: f64 = (-4.0);
        let assign8450_e8350: f64 = (assign8450_e8348 * var_t2);
        let assign8450_e8352: f64 = (assign8450_e8350 * var_t2);
        let assign8450_e8354: f64 = (assign8450_e8352 * var_t0);
        let assign8450_e8356: f64 = (assign8450_e8354 * var_t0);
        let assign8450_e8359: f64 = (2.0 * var_k1_2);
        let assign8450_e8361: f64 = (assign8450_e8359 * var_t0);
        let assign8450_e8362: f64 = (assign8450_e8356 + assign8450_e8361);
        var_dg2 = assign8450_e8362;
        var_dg2_dn3 = ((((((((assign8450_e8348 * var_t2_dn3) * var_t2) + (assign8450_e8350 * var_t2_dn3)) * var_t0) + (assign8450_e8352 * var_t0_dn3)) * var_t0) + (assign8450_e8354 * var_t0_dn3)) + (assign8450_e8359 * var_t0_dn3));
        var_dg2_dn4 = ((((((((assign8450_e8348 * var_t2_dn4) * var_t2) + (assign8450_e8350 * var_t2_dn4)) * var_t0) + (assign8450_e8352 * var_t0_dn4)) * var_t0) + (assign8450_e8354 * var_t0_dn4)) + (assign8450_e8359 * var_t0_dn4));
        var_dg2_dn5 = ((((((((assign8450_e8348 * var_t2_dn5) * var_t2) + (assign8450_e8350 * var_t2_dn5)) * var_t0) + (assign8450_e8352 * var_t0_dn5)) * var_t0) + (assign8450_e8354 * var_t0_dn5)) + (assign8450_e8359 * var_t0_dn5));
        var_dg2_dn6 = ((((((((assign8450_e8348 * var_t2_dn6) * var_t2) + (assign8450_e8350 * var_t2_dn6)) * var_t0) + (assign8450_e8352 * var_t0_dn6)) * var_t0) + (assign8450_e8354 * var_t0_dn6)) + (assign8450_e8359 * var_t0_dn6));
        var_dg2_dn7 = ((((((((assign8450_e8348 * var_t2_dn7) * var_t2) + (assign8450_e8350 * var_t2_dn7)) * var_t0) + (assign8450_e8352 * var_t0_dn7)) * var_t0) + (assign8450_e8354 * var_t0_dn7)) + (assign8450_e8359 * var_t0_dn7));
        var_dg2_dn8 = ((((((((assign8450_e8348 * var_t2_dn8) * var_t2) + (assign8450_e8350 * var_t2_dn8)) * var_t0) + (assign8450_e8352 * var_t0_dn8)) * var_t0) + (assign8450_e8354 * var_t0_dn8)) + (assign8450_e8359 * var_t0_dn8));

        let assign8460_e8365: f64 = (var_g * var_dg1);
        var_t1 = assign8460_e8365;
        var_t1_dn3 = ((var_g_dn3 * var_dg1) + (var_g * var_dg1_dn3));
        var_t1_dn4 = ((var_g_dn4 * var_dg1) + (var_g * var_dg1_dn4));
        var_t1_dn5 = ((var_g_dn5 * var_dg1) + (var_g * var_dg1_dn5));
        var_t1_dn6 = ((var_g_dn6 * var_dg1) + (var_g * var_dg1_dn6));
        var_t1_dn7 = ((var_g_dn7 * var_dg1) + (var_g * var_dg1_dn7));
        var_t1_dn8 = ((var_g_dn8 * var_dg1) + (var_g * var_dg1_dn8));

        let assign8470_e8367: f64 = (-var_t1);
        let assign8470_e8370: f64 = (0.5 * var_t1);
        let assign8470_e8372: f64 = (assign8470_e8370 * var_t1);
        let assign8470_e8374: f64 = (assign8470_e8372 * var_dg2);
        let assign8470_e8376: f64 = (assign8470_e8374 * var_dg1);
        let assign8470_e8377: f64 = (assign8470_e8367 - assign8470_e8376);
        var_delta = assign8470_e8377;
        var_delta_dn3 = ((-var_t1_dn3) - (((((((0.5 * var_t1_dn3) * var_t1) + (assign8470_e8370 * var_t1_dn3)) * var_dg2) + (assign8470_e8372 * var_dg2_dn3)) * var_dg1) + (assign8470_e8374 * var_dg1_dn3)));
        var_delta_dn4 = ((-var_t1_dn4) - (((((((0.5 * var_t1_dn4) * var_t1) + (assign8470_e8370 * var_t1_dn4)) * var_dg2) + (assign8470_e8372 * var_dg2_dn4)) * var_dg1) + (assign8470_e8374 * var_dg1_dn4)));
        var_delta_dn5 = ((-var_t1_dn5) - (((((((0.5 * var_t1_dn5) * var_t1) + (assign8470_e8370 * var_t1_dn5)) * var_dg2) + (assign8470_e8372 * var_dg2_dn5)) * var_dg1) + (assign8470_e8374 * var_dg1_dn5)));
        var_delta_dn6 = ((-var_t1_dn6) - (((((((0.5 * var_t1_dn6) * var_t1) + (assign8470_e8370 * var_t1_dn6)) * var_dg2) + (assign8470_e8372 * var_dg2_dn6)) * var_dg1) + (assign8470_e8374 * var_dg1_dn6)));
        var_delta_dn7 = ((-var_t1_dn7) - (((((((0.5 * var_t1_dn7) * var_t1) + (assign8470_e8370 * var_t1_dn7)) * var_dg2) + (assign8470_e8372 * var_dg2_dn7)) * var_dg1) + (assign8470_e8374 * var_dg1_dn7)));
        var_delta_dn8 = ((-var_t1_dn8) - (((((((0.5 * var_t1_dn8) * var_t1) + (assign8470_e8370 * var_t1_dn8)) * var_dg2) + (assign8470_e8372 * var_dg2_dn8)) * var_dg1) + (assign8470_e8374 * var_dg1_dn8)));

        let assign8480_e8380: f64 = (-10.0);
        let assign8480_e8381: f64 = (var_delta).max(assign8480_e8380);
        var_delta = assign8480_e8381;
        var_delta_dn3 = if var_delta >= assign8480_e8380 { var_delta_dn3 } else { 0.0 };
        var_delta_dn4 = if var_delta >= assign8480_e8380 { var_delta_dn4 } else { 0.0 };
        var_delta_dn5 = if var_delta >= assign8480_e8380 { var_delta_dn5 } else { 0.0 };
        var_delta_dn6 = if var_delta >= assign8480_e8380 { var_delta_dn6 } else { 0.0 };
        var_delta_dn7 = if var_delta >= assign8480_e8380 { var_delta_dn7 } else { 0.0 };
        var_delta_dn8 = if var_delta >= assign8480_e8380 { var_delta_dn8 } else { 0.0 };

        let assign8490_e8384: f64 = (var_delta).min(10.0);
        var_delta = assign8490_e8384;
        var_delta_dn3 = if var_delta <= 10.0 { var_delta_dn3 } else { 0.0 };
        var_delta_dn4 = if var_delta <= 10.0 { var_delta_dn4 } else { 0.0 };
        var_delta_dn5 = if var_delta <= 10.0 { var_delta_dn5 } else { 0.0 };
        var_delta_dn6 = if var_delta <= 10.0 { var_delta_dn6 } else { 0.0 };
        var_delta_dn7 = if var_delta <= 10.0 { var_delta_dn7 } else { 0.0 };
        var_delta_dn8 = if var_delta <= 10.0 { var_delta_dn8 } else { 0.0 };

        let assign8500_e8387: f64 = (var_phissat + var_delta);
        var_phissat = assign8500_e8387;
        var_phissat_dn3 = (var_phissat_dn3 + var_delta_dn3);
        var_phissat_dn4 = (var_phissat_dn4 + var_delta_dn4);
        var_phissat_dn5 = (var_phissat_dn5 + var_delta_dn5);
        var_phissat_dn6 = (var_phissat_dn6 + var_delta_dn6);
        var_phissat_dn7 = (var_phissat_dn7 + var_delta_dn7);
        var_phissat_dn8 = (var_phissat_dn8 + var_delta_dn8);

        let assign8510_e8391: f64 = (var_phi1_0 - 4.0);
        let assign8510_e8392: f64 = (var_phissat).max(assign8510_e8391);
        var_phissat = assign8510_e8392;
        var_phissat_dn3 = if var_phissat >= assign8510_e8391 { var_phissat_dn3 } else { var_phi1_0_dn3 };
        var_phissat_dn4 = if var_phissat >= assign8510_e8391 { var_phissat_dn4 } else { var_phi1_0_dn4 };
        var_phissat_dn5 = if var_phissat >= assign8510_e8391 { var_phissat_dn5 } else { var_phi1_0_dn5 };
        var_phissat_dn6 = if var_phissat >= assign8510_e8391 { var_phissat_dn6 } else { var_phi1_0_dn6 };
        var_phissat_dn7 = if var_phissat >= assign8510_e8391 { var_phissat_dn7 } else { var_phi1_0_dn7 };
        var_phissat_dn8 = if var_phissat >= assign8510_e8391 { var_phissat_dn8 } else { var_phi1_0_dn8 };

        let assign8520_e8395: f64 = (var_vgfb1eff - var_vdseff);
        let assign8520_e8397: f64 = (assign8520_e8395 / var_nvtm);
        var_xg1 = assign8520_e8397;
        var_xg1_dn3 = ((((var_vgfb1eff_dn3 - var_vdseff_dn3) * var_nvtm) - (assign8520_e8395 * var_nvtm_dn3)) / (var_nvtm * var_nvtm));
        var_xg1_dn4 = ((((var_vgfb1eff_dn4 - var_vdseff_dn4) * var_nvtm) - (assign8520_e8395 * var_nvtm_dn4)) / (var_nvtm * var_nvtm));
        var_xg1_dn5 = ((((var_vgfb1eff_dn5 - var_vdseff_dn5) * var_nvtm) - (assign8520_e8395 * var_nvtm_dn5)) / (var_nvtm * var_nvtm));
        var_xg1_dn6 = ((((var_vgfb1eff_dn6 - var_vdseff_dn6) * var_nvtm) - (assign8520_e8395 * var_nvtm_dn6)) / (var_nvtm * var_nvtm));
        var_xg1_dn7 = ((((var_vgfb1eff_dn7 - var_vdseff_dn7) * var_nvtm) - (assign8520_e8395 * var_nvtm_dn7)) / (var_nvtm * var_nvtm));
        var_xg1_dn8 = ((((var_vgfb1eff_dn8 - var_vdseff_dn8) * var_nvtm) - (assign8520_e8395 * var_nvtm_dn8)) / (var_nvtm * var_nvtm));

        let assign8530_e8404: f64 = (1.05 * var_phissat);
        let assign8530_e8405: f64 = (var_phi1 - assign8530_e8404);
        let assign8530_e8407: f64 = assign8530_e8405;
        let assign8530_e8408: f64 = (assign8530_e8407).exp();
        let assign8530_e8409: f64 = (1.0 + assign8530_e8408);
        let assign8530_e8410: f64 = (assign8530_e8409).ln();
        let assign8530_e8411: f64 = assign8530_e8410;
        let assign8530_e8412: f64 = (var_phi1 - assign8530_e8411);
        var_phi1 = assign8530_e8412;
        var_phi1_dn3 = (var_phi1_dn3 - ((assign8530_e8408 * (var_phi1_dn3 - (1.05 * var_phissat_dn3))) / assign8530_e8409));
        var_phi1_dn4 = (var_phi1_dn4 - ((assign8530_e8408 * (var_phi1_dn4 - (1.05 * var_phissat_dn4))) / assign8530_e8409));
        var_phi1_dn5 = (var_phi1_dn5 - ((assign8530_e8408 * (var_phi1_dn5 - (1.05 * var_phissat_dn5))) / assign8530_e8409));
        var_phi1_dn6 = (var_phi1_dn6 - ((assign8530_e8408 * (var_phi1_dn6 - (1.05 * var_phissat_dn6))) / assign8530_e8409));
        var_phi1_dn7 = (var_phi1_dn7 - ((assign8530_e8408 * (var_phi1_dn7 - (1.05 * var_phissat_dn7))) / assign8530_e8409));
        var_phi1_dn8 = (var_phi1_dn8 - ((assign8530_e8408 * (var_phi1_dn8 - (1.05 * var_phissat_dn8))) / assign8530_e8409));

        let assign8540_e8415: f64 = (var_phi1).min(var_phissat);
        var_phi1 = assign8540_e8415;
        var_phi1_dn3 = if var_phi1 <= var_phissat { var_phi1_dn3 } else { var_phissat_dn3 };
        var_phi1_dn4 = if var_phi1 <= var_phissat { var_phi1_dn4 } else { var_phissat_dn4 };
        var_phi1_dn5 = if var_phi1 <= var_phissat { var_phi1_dn5 } else { var_phissat_dn5 };
        var_phi1_dn6 = if var_phi1 <= var_phissat { var_phi1_dn6 } else { var_phissat_dn6 };
        var_phi1_dn7 = if var_phi1 <= var_phissat { var_phi1_dn7 } else { var_phissat_dn7 };
        var_phi1_dn8 = if var_phi1 <= var_phissat { var_phi1_dn8 } else { var_phissat_dn8 };

        let assign8550_e8418: f64 = (var_xg1 - var_phi1);
        var_q1 = assign8550_e8418;
        var_q1_dn3 = (var_xg1_dn3 - var_phi1_dn3);
        var_q1_dn4 = (var_xg1_dn4 - var_phi1_dn4);
        var_q1_dn5 = (var_xg1_dn5 - var_phi1_dn5);
        var_q1_dn6 = (var_xg1_dn6 - var_phi1_dn6);
        var_q1_dn7 = (var_xg1_dn7 - var_phi1_dn7);
        var_q1_dn8 = (var_xg1_dn8 - var_phi1_dn8);

        let assign8560_e8421: f64 = (var_k1 * var_q1);
        var_auxb1 = assign8560_e8421;
        var_auxb1_dn3 = (var_k1 * var_q1_dn3);
        var_auxb1_dn4 = (var_k1 * var_q1_dn4);
        var_auxb1_dn5 = (var_k1 * var_q1_dn5);
        var_auxb1_dn6 = (var_k1 * var_q1_dn6);
        var_auxb1_dn7 = (var_k1 * var_q1_dn7);
        var_auxb1_dn8 = (var_k1 * var_q1_dn8);

        let assign8570_e8423: f64 = (-var_a0);
        let assign8570_e8425: f64 = (var_phi1).exp();
        let assign8570_e8426: f64 = (assign8570_e8423 * assign8570_e8425);
        var_aaux = assign8570_e8426;
        var_aaux_dn3 = (((-var_a0_dn3) * assign8570_e8425) + (assign8570_e8423 * (assign8570_e8425 * var_phi1_dn3)));
        var_aaux_dn4 = (((-var_a0_dn4) * assign8570_e8425) + (assign8570_e8423 * (assign8570_e8425 * var_phi1_dn4)));
        var_aaux_dn5 = (((-var_a0_dn5) * assign8570_e8425) + (assign8570_e8423 * (assign8570_e8425 * var_phi1_dn5)));
        var_aaux_dn6 = (((-var_a0_dn6) * assign8570_e8425) + (assign8570_e8423 * (assign8570_e8425 * var_phi1_dn6)));
        var_aaux_dn7 = (((-var_a0_dn7) * assign8570_e8425) + (assign8570_e8423 * (assign8570_e8425 * var_phi1_dn7)));
        var_aaux_dn8 = (((-var_a0_dn8) * assign8570_e8425) + (assign8570_e8423 * (assign8570_e8425 * var_phi1_dn8)));

        let assign8580_e8429: f64 = (var_auxb1 * var_auxb1);
        let assign8580_e8431: f64 = (assign8580_e8429 + var_aaux);
        var_qsqrt = assign8580_e8431;
        var_qsqrt_dn3 = (((var_auxb1_dn3 * var_auxb1) + (var_auxb1 * var_auxb1_dn3)) + var_aaux_dn3);
        var_qsqrt_dn4 = (((var_auxb1_dn4 * var_auxb1) + (var_auxb1 * var_auxb1_dn4)) + var_aaux_dn4);
        var_qsqrt_dn5 = (((var_auxb1_dn5 * var_auxb1) + (var_auxb1 * var_auxb1_dn5)) + var_aaux_dn5);
        var_qsqrt_dn6 = (((var_auxb1_dn6 * var_auxb1) + (var_auxb1 * var_auxb1_dn6)) + var_aaux_dn6);
        var_qsqrt_dn7 = (((var_auxb1_dn7 * var_auxb1) + (var_auxb1 * var_auxb1_dn7)) + var_aaux_dn7);
        var_qsqrt_dn8 = (((var_auxb1_dn8 * var_auxb1) + (var_auxb1 * var_auxb1_dn8)) + var_aaux_dn8);

        let assign8590_e8434: f64 = if var_qsqrt < 0.0 { 1.0 } else { 0.0 };
        var_guard92 = assign8590_e8434;

        let (assign8600_e8440, assign8600_e8440_d_n3, assign8600_e8440_d_n4, assign8600_e8440_d_n5, assign8600_e8440_d_n6, assign8600_e8440_d_n7, assign8600_e8440_d_n8,) = {
    if (var_guard92 != 0.0) {
        let assign8600_e8437: f64 = (-var_qsqrt);
        let assign8600_e8438: f64 = (assign8600_e8437).sqrt();
        (assign8600_e8438, ((-var_qsqrt_dn3) / (2.0 * assign8600_e8438)), ((-var_qsqrt_dn4) / (2.0 * assign8600_e8438)), ((-var_qsqrt_dn5) / (2.0 * assign8600_e8438)), ((-var_qsqrt_dn6) / (2.0 * assign8600_e8438)), ((-var_qsqrt_dn7) / (2.0 * assign8600_e8438)), ((-var_qsqrt_dn8) / (2.0 * assign8600_e8438)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign8600_e8440;
        var_q_dn3 = assign8600_e8440_d_n3;
        var_q_dn4 = assign8600_e8440_d_n4;
        var_q_dn5 = assign8600_e8440_d_n5;
        var_q_dn6 = assign8600_e8440_d_n6;
        var_q_dn7 = assign8600_e8440_d_n7;
        var_q_dn8 = assign8600_e8440_d_n8;

        let (assign8610_e8449, assign8610_e8449_d_n3, assign8610_e8449_d_n4, assign8610_e8449_d_n5, assign8610_e8449_d_n6, assign8610_e8449_d_n7, assign8610_e8449_d_n8,) = {
    if (var_guard92 != 0.0) {
        let assign8610_e8445: f64 = (0.5 * var_q);
        let assign8610_e8446: f64 = (assign8610_e8445).sin();
        let assign8610_e8447: f64 = (1.0 / assign8610_e8446);
        (assign8610_e8447, (-(((assign8610_e8445).cos() * (0.5 * var_q_dn3)) / (assign8610_e8446 * assign8610_e8446))), (-(((assign8610_e8445).cos() * (0.5 * var_q_dn4)) / (assign8610_e8446 * assign8610_e8446))), (-(((assign8610_e8445).cos() * (0.5 * var_q_dn5)) / (assign8610_e8446 * assign8610_e8446))), (-(((assign8610_e8445).cos() * (0.5 * var_q_dn6)) / (assign8610_e8446 * assign8610_e8446))), (-(((assign8610_e8445).cos() * (0.5 * var_q_dn7)) / (assign8610_e8446 * assign8610_e8446))), (-(((assign8610_e8445).cos() * (0.5 * var_q_dn8)) / (assign8610_e8446 * assign8610_e8446))),)
    } else {
        (var_csc1, var_csc1_dn3, var_csc1_dn4, var_csc1_dn5, var_csc1_dn6, var_csc1_dn7, var_csc1_dn8,)
    }
};
        var_csc1 = assign8610_e8449;
        var_csc1_dn3 = assign8610_e8449_d_n3;
        var_csc1_dn4 = assign8610_e8449_d_n4;
        var_csc1_dn5 = assign8610_e8449_d_n5;
        var_csc1_dn6 = assign8610_e8449_d_n6;
        var_csc1_dn7 = assign8610_e8449_d_n7;
        var_csc1_dn8 = assign8610_e8449_d_n8;

        let (assign8620_e8455, assign8620_e8455_d_n3, assign8620_e8455_d_n4, assign8620_e8455_d_n5, assign8620_e8455_d_n6, assign8620_e8455_d_n7, assign8620_e8455_d_n8,) = {
    if (var_guard92 != 0.0) {
        let assign8620_e8453: f64 = (var_csc1 * var_csc1);
        (assign8620_e8453, ((var_csc1_dn3 * var_csc1) + (var_csc1 * var_csc1_dn3)), ((var_csc1_dn4 * var_csc1) + (var_csc1 * var_csc1_dn4)), ((var_csc1_dn5 * var_csc1) + (var_csc1 * var_csc1_dn5)), ((var_csc1_dn6 * var_csc1) + (var_csc1 * var_csc1_dn6)), ((var_csc1_dn7 * var_csc1) + (var_csc1 * var_csc1_dn7)), ((var_csc1_dn8 * var_csc1) + (var_csc1 * var_csc1_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign8620_e8455;
        var_t1_dn3 = assign8620_e8455_d_n3;
        var_t1_dn4 = assign8620_e8455_d_n4;
        var_t1_dn5 = assign8620_e8455_d_n5;
        var_t1_dn6 = assign8620_e8455_d_n6;
        var_t1_dn7 = assign8620_e8455_d_n7;
        var_t1_dn8 = assign8620_e8455_d_n8;

        let (assign8630_e8464, assign8630_e8464_d_n3, assign8630_e8464_d_n4, assign8630_e8464_d_n5, assign8630_e8464_d_n6, assign8630_e8464_d_n7, assign8630_e8464_d_n8,) = {
    if (var_guard92 != 0.0) {
        let assign8630_e8459: f64 = (0.5 * var_q);
        let assign8630_e8460: f64 = (assign8630_e8459).cos();
        let assign8630_e8462: f64 = (assign8630_e8460 * var_csc1);
        (assign8630_e8462, (((-(assign8630_e8459).sin() * (0.5 * var_q_dn3)) * var_csc1) + (assign8630_e8460 * var_csc1_dn3)), (((-(assign8630_e8459).sin() * (0.5 * var_q_dn4)) * var_csc1) + (assign8630_e8460 * var_csc1_dn4)), (((-(assign8630_e8459).sin() * (0.5 * var_q_dn5)) * var_csc1) + (assign8630_e8460 * var_csc1_dn5)), (((-(assign8630_e8459).sin() * (0.5 * var_q_dn6)) * var_csc1) + (assign8630_e8460 * var_csc1_dn6)), (((-(assign8630_e8459).sin() * (0.5 * var_q_dn7)) * var_csc1) + (assign8630_e8460 * var_csc1_dn7)), (((-(assign8630_e8459).sin() * (0.5 * var_q_dn8)) * var_csc1) + (assign8630_e8460 * var_csc1_dn8)),)
    } else {
        (var_coth1, var_coth1_dn3, var_coth1_dn4, var_coth1_dn5, var_coth1_dn6, var_coth1_dn7, var_coth1_dn8,)
    }
};
        var_coth1 = assign8630_e8464;
        var_coth1_dn3 = assign8630_e8464_d_n3;
        var_coth1_dn4 = assign8630_e8464_d_n4;
        var_coth1_dn5 = assign8630_e8464_d_n5;
        var_coth1_dn6 = assign8630_e8464_d_n6;
        var_coth1_dn7 = assign8630_e8464_d_n7;
        var_coth1_dn8 = assign8630_e8464_d_n8;

        let (assign8640_e8473, assign8640_e8473_d_n3, assign8640_e8473_d_n4, assign8640_e8473_d_n5, assign8640_e8473_d_n6, assign8640_e8473_d_n7, assign8640_e8473_d_n8,) = {
    if (var_guard92 != 0.0) {
        let assign8640_e8467: f64 = (-0.5);
        let assign8640_e8469: f64 = (assign8640_e8467 * var_coth1);
        let assign8640_e8471: f64 = (assign8640_e8469 / var_q);
        (assign8640_e8471, ((((assign8640_e8467 * var_coth1_dn3) * var_q) - (assign8640_e8469 * var_q_dn3)) / (var_q * var_q)), ((((assign8640_e8467 * var_coth1_dn4) * var_q) - (assign8640_e8469 * var_q_dn4)) / (var_q * var_q)), ((((assign8640_e8467 * var_coth1_dn5) * var_q) - (assign8640_e8469 * var_q_dn5)) / (var_q * var_q)), ((((assign8640_e8467 * var_coth1_dn6) * var_q) - (assign8640_e8469 * var_q_dn6)) / (var_q * var_q)), ((((assign8640_e8467 * var_coth1_dn7) * var_q) - (assign8640_e8469 * var_q_dn7)) / (var_q * var_q)), ((((assign8640_e8467 * var_coth1_dn8) * var_q) - (assign8640_e8469 * var_q_dn8)) / (var_q * var_q)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign8640_e8473;
        var_t0_dn3 = assign8640_e8473_d_n3;
        var_t0_dn4 = assign8640_e8473_d_n4;
        var_t0_dn5 = assign8640_e8473_d_n5;
        var_t0_dn6 = assign8640_e8473_d_n6;
        var_t0_dn7 = assign8640_e8473_d_n7;
        var_t0_dn8 = assign8640_e8473_d_n8;

        let (assign8650_e8481, assign8650_e8481_d_n3, assign8650_e8481_d_n4, assign8650_e8481_d_n5, assign8650_e8481_d_n6, assign8650_e8481_d_n7, assign8650_e8481_d_n8,) = {
    if (var_guard92 != 0.0) {
        let assign8650_e8477: f64 = (0.25 * var_t1);
        let assign8650_e8479: f64 = (assign8650_e8477 + var_t0);
        (assign8650_e8479, ((0.25 * var_t1_dn3) + var_t0_dn3), ((0.25 * var_t1_dn4) + var_t0_dn4), ((0.25 * var_t1_dn5) + var_t0_dn5), ((0.25 * var_t1_dn6) + var_t0_dn6), ((0.25 * var_t1_dn7) + var_t0_dn7), ((0.25 * var_t1_dn8) + var_t0_dn8),)
    } else {
        (var_dqcothqdqsqrt, var_dqcothqdqsqrt_dn3, var_dqcothqdqsqrt_dn4, var_dqcothqdqsqrt_dn5, var_dqcothqdqsqrt_dn6, var_dqcothqdqsqrt_dn7, var_dqcothqdqsqrt_dn8,)
    }
};
        var_dqcothqdqsqrt = assign8650_e8481;
        var_dqcothqdqsqrt_dn3 = assign8650_e8481_d_n3;
        var_dqcothqdqsqrt_dn4 = assign8650_e8481_d_n4;
        var_dqcothqdqsqrt_dn5 = assign8650_e8481_d_n5;
        var_dqcothqdqsqrt_dn6 = assign8650_e8481_d_n6;
        var_dqcothqdqsqrt_dn7 = assign8650_e8481_d_n7;
        var_dqcothqdqsqrt_dn8 = assign8650_e8481_d_n8;

        let (assign8660_e8487, assign8660_e8487_d_n3, assign8660_e8487_d_n4, assign8660_e8487_d_n5, assign8660_e8487_d_n6, assign8660_e8487_d_n7, assign8660_e8487_d_n8,) = {
    if (var_guard92 == 0.0) {
        let assign8660_e8485: f64 = (var_qsqrt).sqrt();
        (assign8660_e8485, (var_qsqrt_dn3 / (2.0 * assign8660_e8485)), (var_qsqrt_dn4 / (2.0 * assign8660_e8485)), (var_qsqrt_dn5 / (2.0 * assign8660_e8485)), (var_qsqrt_dn6 / (2.0 * assign8660_e8485)), (var_qsqrt_dn7 / (2.0 * assign8660_e8485)), (var_qsqrt_dn8 / (2.0 * assign8660_e8485)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign8660_e8487;
        var_q_dn3 = assign8660_e8487_d_n3;
        var_q_dn4 = assign8660_e8487_d_n4;
        var_q_dn5 = assign8660_e8487_d_n5;
        var_q_dn6 = assign8660_e8487_d_n6;
        var_q_dn7 = assign8660_e8487_d_n7;
        var_q_dn8 = assign8660_e8487_d_n8;

        let (assign8670_e8497, assign8670_e8497_d_n3, assign8670_e8497_d_n4, assign8670_e8497_d_n5, assign8670_e8497_d_n6, assign8670_e8497_d_n7, assign8670_e8497_d_n8,) = {
    if (var_guard92 == 0.0) {
        let assign8670_e8493: f64 = (0.5 * var_q);
        let assign8670_e8494: f64 = (assign8670_e8493).sinh();
        let assign8670_e8495: f64 = (1.0 / assign8670_e8494);
        (assign8670_e8495, (-(((assign8670_e8493).cosh() * (0.5 * var_q_dn3)) / (assign8670_e8494 * assign8670_e8494))), (-(((assign8670_e8493).cosh() * (0.5 * var_q_dn4)) / (assign8670_e8494 * assign8670_e8494))), (-(((assign8670_e8493).cosh() * (0.5 * var_q_dn5)) / (assign8670_e8494 * assign8670_e8494))), (-(((assign8670_e8493).cosh() * (0.5 * var_q_dn6)) / (assign8670_e8494 * assign8670_e8494))), (-(((assign8670_e8493).cosh() * (0.5 * var_q_dn7)) / (assign8670_e8494 * assign8670_e8494))), (-(((assign8670_e8493).cosh() * (0.5 * var_q_dn8)) / (assign8670_e8494 * assign8670_e8494))),)
    } else {
        (var_csc1, var_csc1_dn3, var_csc1_dn4, var_csc1_dn5, var_csc1_dn6, var_csc1_dn7, var_csc1_dn8,)
    }
};
        var_csc1 = assign8670_e8497;
        var_csc1_dn3 = assign8670_e8497_d_n3;
        var_csc1_dn4 = assign8670_e8497_d_n4;
        var_csc1_dn5 = assign8670_e8497_d_n5;
        var_csc1_dn6 = assign8670_e8497_d_n6;
        var_csc1_dn7 = assign8670_e8497_d_n7;
        var_csc1_dn8 = assign8670_e8497_d_n8;

        let (assign8680_e8504, assign8680_e8504_d_n3, assign8680_e8504_d_n4, assign8680_e8504_d_n5, assign8680_e8504_d_n6, assign8680_e8504_d_n7, assign8680_e8504_d_n8,) = {
    if (var_guard92 == 0.0) {
        let assign8680_e8502: f64 = (var_csc1 * var_csc1);
        (assign8680_e8502, ((var_csc1_dn3 * var_csc1) + (var_csc1 * var_csc1_dn3)), ((var_csc1_dn4 * var_csc1) + (var_csc1 * var_csc1_dn4)), ((var_csc1_dn5 * var_csc1) + (var_csc1 * var_csc1_dn5)), ((var_csc1_dn6 * var_csc1) + (var_csc1 * var_csc1_dn6)), ((var_csc1_dn7 * var_csc1) + (var_csc1 * var_csc1_dn7)), ((var_csc1_dn8 * var_csc1) + (var_csc1 * var_csc1_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign8680_e8504;
        var_t1_dn3 = assign8680_e8504_d_n3;
        var_t1_dn4 = assign8680_e8504_d_n4;
        var_t1_dn5 = assign8680_e8504_d_n5;
        var_t1_dn6 = assign8680_e8504_d_n6;
        var_t1_dn7 = assign8680_e8504_d_n7;
        var_t1_dn8 = assign8680_e8504_d_n8;

        let (assign8690_e8512, assign8690_e8512_d_n3, assign8690_e8512_d_n4, assign8690_e8512_d_n5, assign8690_e8512_d_n6, assign8690_e8512_d_n7, assign8690_e8512_d_n8,) = {
    if (var_guard92 == 0.0) {
        let assign8690_e8509: f64 = (1.0 + var_t1);
        let assign8690_e8510: f64 = (assign8690_e8509).sqrt();
        (assign8690_e8510, (var_t1_dn3 / (2.0 * assign8690_e8510)), (var_t1_dn4 / (2.0 * assign8690_e8510)), (var_t1_dn5 / (2.0 * assign8690_e8510)), (var_t1_dn6 / (2.0 * assign8690_e8510)), (var_t1_dn7 / (2.0 * assign8690_e8510)), (var_t1_dn8 / (2.0 * assign8690_e8510)),)
    } else {
        (var_coth1, var_coth1_dn3, var_coth1_dn4, var_coth1_dn5, var_coth1_dn6, var_coth1_dn7, var_coth1_dn8,)
    }
};
        var_coth1 = assign8690_e8512;
        var_coth1_dn3 = assign8690_e8512_d_n3;
        var_coth1_dn4 = assign8690_e8512_d_n4;
        var_coth1_dn5 = assign8690_e8512_d_n5;
        var_coth1_dn6 = assign8690_e8512_d_n6;
        var_coth1_dn7 = assign8690_e8512_d_n7;
        var_coth1_dn8 = assign8690_e8512_d_n8;

        let (assign8700_e8521, assign8700_e8521_d_n3, assign8700_e8521_d_n4, assign8700_e8521_d_n5, assign8700_e8521_d_n6, assign8700_e8521_d_n7, assign8700_e8521_d_n8,) = {
    if (var_guard92 == 0.0) {
        let assign8700_e8517: f64 = (0.5 * var_coth1);
        let assign8700_e8519: f64 = (assign8700_e8517 / var_q);
        (assign8700_e8519, ((((0.5 * var_coth1_dn3) * var_q) - (assign8700_e8517 * var_q_dn3)) / (var_q * var_q)), ((((0.5 * var_coth1_dn4) * var_q) - (assign8700_e8517 * var_q_dn4)) / (var_q * var_q)), ((((0.5 * var_coth1_dn5) * var_q) - (assign8700_e8517 * var_q_dn5)) / (var_q * var_q)), ((((0.5 * var_coth1_dn6) * var_q) - (assign8700_e8517 * var_q_dn6)) / (var_q * var_q)), ((((0.5 * var_coth1_dn7) * var_q) - (assign8700_e8517 * var_q_dn7)) / (var_q * var_q)), ((((0.5 * var_coth1_dn8) * var_q) - (assign8700_e8517 * var_q_dn8)) / (var_q * var_q)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign8700_e8521;
        var_t0_dn3 = assign8700_e8521_d_n3;
        var_t0_dn4 = assign8700_e8521_d_n4;
        var_t0_dn5 = assign8700_e8521_d_n5;
        var_t0_dn6 = assign8700_e8521_d_n6;
        var_t0_dn7 = assign8700_e8521_d_n7;
        var_t0_dn8 = assign8700_e8521_d_n8;

        let (assign8710_e8531, assign8710_e8531_d_n3, assign8710_e8531_d_n4, assign8710_e8531_d_n5, assign8710_e8531_d_n6, assign8710_e8531_d_n7, assign8710_e8531_d_n8,) = {
    if (var_guard92 == 0.0) {
        let assign8710_e8525: f64 = (-0.25);
        let assign8710_e8527: f64 = (assign8710_e8525 * var_t1);
        let assign8710_e8529: f64 = (assign8710_e8527 + var_t0);
        (assign8710_e8529, ((assign8710_e8525 * var_t1_dn3) + var_t0_dn3), ((assign8710_e8525 * var_t1_dn4) + var_t0_dn4), ((assign8710_e8525 * var_t1_dn5) + var_t0_dn5), ((assign8710_e8525 * var_t1_dn6) + var_t0_dn6), ((assign8710_e8525 * var_t1_dn7) + var_t0_dn7), ((assign8710_e8525 * var_t1_dn8) + var_t0_dn8),)
    } else {
        (var_dqcothqdqsqrt, var_dqcothqdqsqrt_dn3, var_dqcothqdqsqrt_dn4, var_dqcothqdqsqrt_dn5, var_dqcothqdqsqrt_dn6, var_dqcothqdqsqrt_dn7, var_dqcothqdqsqrt_dn8,)
    }
};
        var_dqcothqdqsqrt = assign8710_e8531;
        var_dqcothqdqsqrt_dn3 = assign8710_e8531_d_n3;
        var_dqcothqdqsqrt_dn4 = assign8710_e8531_d_n4;
        var_dqcothqdqsqrt_dn5 = assign8710_e8531_d_n5;
        var_dqcothqdqsqrt_dn6 = assign8710_e8531_d_n6;
        var_dqcothqdqsqrt_dn7 = assign8710_e8531_d_n7;
        var_dqcothqdqsqrt_dn8 = assign8710_e8531_d_n8;

        let assign8720_e8534: f64 = (var_q * var_coth1);
        var_qcoth = assign8720_e8534;
        var_qcoth_dn3 = ((var_q_dn3 * var_coth1) + (var_q * var_coth1_dn3));
        var_qcoth_dn4 = ((var_q_dn4 * var_coth1) + (var_q * var_coth1_dn4));
        var_qcoth_dn5 = ((var_q_dn5 * var_coth1) + (var_q * var_coth1_dn5));
        var_qcoth_dn6 = ((var_q_dn6 * var_coth1) + (var_q * var_coth1_dn6));
        var_qcoth_dn7 = ((var_q_dn7 * var_coth1) + (var_q * var_coth1_dn7));
        var_qcoth_dn8 = ((var_q_dn8 * var_coth1) + (var_q * var_coth1_dn8));

        let assign8730_e8537: f64 = (var_auxb1 + var_qcoth);
        var_t2 = assign8730_e8537;
        var_t2_dn3 = (var_auxb1_dn3 + var_qcoth_dn3);
        var_t2_dn4 = (var_auxb1_dn4 + var_qcoth_dn4);
        var_t2_dn5 = (var_auxb1_dn5 + var_qcoth_dn5);
        var_t2_dn6 = (var_auxb1_dn6 + var_qcoth_dn6);
        var_t2_dn7 = (var_auxb1_dn7 + var_qcoth_dn7);
        var_t2_dn8 = (var_auxb1_dn8 + var_qcoth_dn8);

        let assign8740_e8540: f64 = (1.0 / var_t2);
        var_t3 = assign8740_e8540;
        var_t3_dn3 = (-(var_t2_dn3 / (var_t2 * var_t2)));
        var_t3_dn4 = (-(var_t2_dn4 / (var_t2 * var_t2)));
        var_t3_dn5 = (-(var_t2_dn5 / (var_t2 * var_t2)));
        var_t3_dn6 = (-(var_t2_dn6 / (var_t2 * var_t2)));
        var_t3_dn7 = (-(var_t2_dn7 / (var_t2 * var_t2)));
        var_t3_dn8 = (-(var_t2_dn8 / (var_t2 * var_t2)));

        let assign8750_e8543: f64 = (var_xg2 - var_xg1);
        let assign8750_e8545: f64 = (assign8750_e8543 + var_q1);
        let assign8750_e8548: f64 = (var_qsqrt * var_t1);
        let assign8750_e8550: f64 = (assign8750_e8548 * var_t3);
        let assign8750_e8552: f64 = (assign8750_e8550 * var_t3);
        let assign8750_e8553: f64 = (assign8750_e8552).abs();
        let assign8750_e8554: f64 = (assign8750_e8553).ln();
        let assign8750_e8555: f64 = (assign8750_e8545 - assign8750_e8554);
        var_q2 = assign8750_e8555;
        var_q2_dn3 = (((var_xg2_dn3 - var_xg1_dn3) + var_q1_dn3) - (if assign8750_e8552 >= 0.0 { ((((((var_qsqrt_dn3 * var_t1) + (var_qsqrt * var_t1_dn3)) * var_t3) + (assign8750_e8548 * var_t3_dn3)) * var_t3) + (assign8750_e8550 * var_t3_dn3)) } else { (-((((((var_qsqrt_dn3 * var_t1) + (var_qsqrt * var_t1_dn3)) * var_t3) + (assign8750_e8548 * var_t3_dn3)) * var_t3) + (assign8750_e8550 * var_t3_dn3))) } / assign8750_e8553));
        var_q2_dn4 = (((var_xg2_dn4 - var_xg1_dn4) + var_q1_dn4) - (if assign8750_e8552 >= 0.0 { ((((((var_qsqrt_dn4 * var_t1) + (var_qsqrt * var_t1_dn4)) * var_t3) + (assign8750_e8548 * var_t3_dn4)) * var_t3) + (assign8750_e8550 * var_t3_dn4)) } else { (-((((((var_qsqrt_dn4 * var_t1) + (var_qsqrt * var_t1_dn4)) * var_t3) + (assign8750_e8548 * var_t3_dn4)) * var_t3) + (assign8750_e8550 * var_t3_dn4))) } / assign8750_e8553));
        var_q2_dn5 = (((var_xg2_dn5 - var_xg1_dn5) + var_q1_dn5) - (if assign8750_e8552 >= 0.0 { ((((((var_qsqrt_dn5 * var_t1) + (var_qsqrt * var_t1_dn5)) * var_t3) + (assign8750_e8548 * var_t3_dn5)) * var_t3) + (assign8750_e8550 * var_t3_dn5)) } else { (-((((((var_qsqrt_dn5 * var_t1) + (var_qsqrt * var_t1_dn5)) * var_t3) + (assign8750_e8548 * var_t3_dn5)) * var_t3) + (assign8750_e8550 * var_t3_dn5))) } / assign8750_e8553));
        var_q2_dn6 = (((var_xg2_dn6 - var_xg1_dn6) + var_q1_dn6) - (if assign8750_e8552 >= 0.0 { ((((((var_qsqrt_dn6 * var_t1) + (var_qsqrt * var_t1_dn6)) * var_t3) + (assign8750_e8548 * var_t3_dn6)) * var_t3) + (assign8750_e8550 * var_t3_dn6)) } else { (-((((((var_qsqrt_dn6 * var_t1) + (var_qsqrt * var_t1_dn6)) * var_t3) + (assign8750_e8548 * var_t3_dn6)) * var_t3) + (assign8750_e8550 * var_t3_dn6))) } / assign8750_e8553));
        var_q2_dn7 = (((var_xg2_dn7 - var_xg1_dn7) + var_q1_dn7) - (if assign8750_e8552 >= 0.0 { ((((((var_qsqrt_dn7 * var_t1) + (var_qsqrt * var_t1_dn7)) * var_t3) + (assign8750_e8548 * var_t3_dn7)) * var_t3) + (assign8750_e8550 * var_t3_dn7)) } else { (-((((((var_qsqrt_dn7 * var_t1) + (var_qsqrt * var_t1_dn7)) * var_t3) + (assign8750_e8548 * var_t3_dn7)) * var_t3) + (assign8750_e8550 * var_t3_dn7))) } / assign8750_e8553));
        var_q2_dn8 = (((var_xg2_dn8 - var_xg1_dn8) + var_q1_dn8) - (if assign8750_e8552 >= 0.0 { ((((((var_qsqrt_dn8 * var_t1) + (var_qsqrt * var_t1_dn8)) * var_t3) + (assign8750_e8548 * var_t3_dn8)) * var_t3) + (assign8750_e8550 * var_t3_dn8)) } else { (-((((((var_qsqrt_dn8 * var_t1) + (var_qsqrt * var_t1_dn8)) * var_t3) + (assign8750_e8548 * var_t3_dn8)) * var_t3) + (assign8750_e8550 * var_t3_dn8))) } / assign8750_e8553));

        let assign8760_e8559: f64 = (var_auxb1 + var_qcoth);
        let assign8760_e8562: f64 = (var_k2 * var_q2);
        let assign8760_e8564: f64 = (assign8760_e8562 + var_auxb1);
        let assign8760_e8565: f64 = (assign8760_e8559 * assign8760_e8564);
        let assign8760_e8566: f64 = (var_aaux + assign8760_e8565);
        var_f = assign8760_e8566;
        var_f_dn3 = (var_aaux_dn3 + (((var_auxb1_dn3 + var_qcoth_dn3) * assign8760_e8564) + (assign8760_e8559 * ((var_k2 * var_q2_dn3) + var_auxb1_dn3))));
        var_f_dn4 = (var_aaux_dn4 + (((var_auxb1_dn4 + var_qcoth_dn4) * assign8760_e8564) + (assign8760_e8559 * ((var_k2 * var_q2_dn4) + var_auxb1_dn4))));
        var_f_dn5 = (var_aaux_dn5 + (((var_auxb1_dn5 + var_qcoth_dn5) * assign8760_e8564) + (assign8760_e8559 * ((var_k2 * var_q2_dn5) + var_auxb1_dn5))));
        var_f_dn6 = (var_aaux_dn6 + (((var_auxb1_dn6 + var_qcoth_dn6) * assign8760_e8564) + (assign8760_e8559 * ((var_k2 * var_q2_dn6) + var_auxb1_dn6))));
        var_f_dn7 = (var_aaux_dn7 + (((var_auxb1_dn7 + var_qcoth_dn7) * assign8760_e8564) + (assign8760_e8559 * ((var_k2 * var_q2_dn7) + var_auxb1_dn7))));
        var_f_dn8 = (var_aaux_dn8 + (((var_auxb1_dn8 + var_qcoth_dn8) * assign8760_e8564) + (assign8760_e8559 * ((var_k2 * var_q2_dn8) + var_auxb1_dn8))));

        let assign8770_e8569: f64 = (1.0 / var_qsqrt);
        let assign8770_e8571: f64 = (assign8770_e8569 - var_t0);
        var_dlogsinhqsqdqsqrt = assign8770_e8571;
        var_dlogsinhqsqdqsqrt_dn3 = ((-(var_qsqrt_dn3 / (var_qsqrt * var_qsqrt))) - var_t0_dn3);
        var_dlogsinhqsqdqsqrt_dn4 = ((-(var_qsqrt_dn4 / (var_qsqrt * var_qsqrt))) - var_t0_dn4);
        var_dlogsinhqsqdqsqrt_dn5 = ((-(var_qsqrt_dn5 / (var_qsqrt * var_qsqrt))) - var_t0_dn5);
        var_dlogsinhqsqdqsqrt_dn6 = ((-(var_qsqrt_dn6 / (var_qsqrt * var_qsqrt))) - var_t0_dn6);
        var_dlogsinhqsqdqsqrt_dn7 = ((-(var_qsqrt_dn7 / (var_qsqrt * var_qsqrt))) - var_t0_dn7);
        var_dlogsinhqsqdqsqrt_dn8 = ((-(var_qsqrt_dn8 / (var_qsqrt * var_qsqrt))) - var_t0_dn8);

        let assign8780_e8573: f64 = (-2.0);
        let assign8780_e8575: f64 = (assign8780_e8573 * var_k1);
        let assign8780_e8577: f64 = (assign8780_e8575 * var_auxb1);
        let assign8780_e8579: f64 = (assign8780_e8577 + var_aaux);
        var_dqsqrt = assign8780_e8579;
        var_dqsqrt_dn3 = ((assign8780_e8575 * var_auxb1_dn3) + var_aaux_dn3);
        var_dqsqrt_dn4 = ((assign8780_e8575 * var_auxb1_dn4) + var_aaux_dn4);
        var_dqsqrt_dn5 = ((assign8780_e8575 * var_auxb1_dn5) + var_aaux_dn5);
        var_dqsqrt_dn6 = ((assign8780_e8575 * var_auxb1_dn6) + var_aaux_dn6);
        var_dqsqrt_dn7 = ((assign8780_e8575 * var_auxb1_dn7) + var_aaux_dn7);
        var_dqsqrt_dn8 = ((assign8780_e8575 * var_auxb1_dn8) + var_aaux_dn8);

        let assign8790_e8582: f64 = (var_dqcothqdqsqrt * var_dqsqrt);
        var_dqcoth = assign8790_e8582;
        var_dqcoth_dn3 = ((var_dqcothqdqsqrt_dn3 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn3));
        var_dqcoth_dn4 = ((var_dqcothqdqsqrt_dn4 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn4));
        var_dqcoth_dn5 = ((var_dqcothqdqsqrt_dn5 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn5));
        var_dqcoth_dn6 = ((var_dqcothqdqsqrt_dn6 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn6));
        var_dqcoth_dn7 = ((var_dqcothqdqsqrt_dn7 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn7));
        var_dqcoth_dn8 = ((var_dqcothqdqsqrt_dn8 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn8));

        let assign8800_e8584: f64 = (-1.0);
        let assign8800_e8587: f64 = (-var_k1);
        let assign8800_e8589: f64 = (assign8800_e8587 + var_dqcoth);
        let assign8800_e8591: f64 = (assign8800_e8589 * var_t3);
        let assign8800_e8592: f64 = (2.0 * assign8800_e8591);
        let assign8800_e8593: f64 = (assign8800_e8584 + assign8800_e8592);
        let assign8800_e8596: f64 = (var_dlogsinhqsqdqsqrt * var_dqsqrt);
        let assign8800_e8597: f64 = (assign8800_e8593 - assign8800_e8596);
        var_dq2 = assign8800_e8597;
        var_dq2_dn3 = ((2.0 * ((var_dqcoth_dn3 * var_t3) + (assign8800_e8589 * var_t3_dn3))) - ((var_dlogsinhqsqdqsqrt_dn3 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn3)));
        var_dq2_dn4 = ((2.0 * ((var_dqcoth_dn4 * var_t3) + (assign8800_e8589 * var_t3_dn4))) - ((var_dlogsinhqsqdqsqrt_dn4 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn4)));
        var_dq2_dn5 = ((2.0 * ((var_dqcoth_dn5 * var_t3) + (assign8800_e8589 * var_t3_dn5))) - ((var_dlogsinhqsqdqsqrt_dn5 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn5)));
        var_dq2_dn6 = ((2.0 * ((var_dqcoth_dn6 * var_t3) + (assign8800_e8589 * var_t3_dn6))) - ((var_dlogsinhqsqdqsqrt_dn6 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn6)));
        var_dq2_dn7 = ((2.0 * ((var_dqcoth_dn7 * var_t3) + (assign8800_e8589 * var_t3_dn7))) - ((var_dlogsinhqsqdqsqrt_dn7 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn7)));
        var_dq2_dn8 = ((2.0 * ((var_dqcoth_dn8 * var_t3) + (assign8800_e8589 * var_t3_dn8))) - ((var_dlogsinhqsqdqsqrt_dn8 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn8)));

        let assign8810_e8602: f64 = (var_auxb1 + var_t2);
        let assign8810_e8603: f64 = (var_k1 * assign8810_e8602);
        let assign8810_e8604: f64 = (var_aaux - assign8810_e8603);
        let assign8810_e8607: f64 = (var_auxb1 * var_dqcoth);
        let assign8810_e8608: f64 = (assign8810_e8604 + assign8810_e8607);
        let assign8810_e8612: f64 = (var_dq2 * var_t2);
        let assign8810_e8616: f64 = (var_dqcoth - var_k1);
        let assign8810_e8617: f64 = (var_q2 * assign8810_e8616);
        let assign8810_e8618: f64 = (assign8810_e8612 + assign8810_e8617);
        let assign8810_e8619: f64 = (var_k2 * assign8810_e8618);
        let assign8810_e8620: f64 = (assign8810_e8608 + assign8810_e8619);
        var_df = assign8810_e8620;
        var_df_dn3 = (((var_aaux_dn3 - (var_k1 * (var_auxb1_dn3 + var_t2_dn3))) + ((var_auxb1_dn3 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn3))) + (var_k2 * (((var_dq2_dn3 * var_t2) + (var_dq2 * var_t2_dn3)) + ((var_q2_dn3 * assign8810_e8616) + (var_q2 * var_dqcoth_dn3)))));
        var_df_dn4 = (((var_aaux_dn4 - (var_k1 * (var_auxb1_dn4 + var_t2_dn4))) + ((var_auxb1_dn4 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn4))) + (var_k2 * (((var_dq2_dn4 * var_t2) + (var_dq2 * var_t2_dn4)) + ((var_q2_dn4 * assign8810_e8616) + (var_q2 * var_dqcoth_dn4)))));
        var_df_dn5 = (((var_aaux_dn5 - (var_k1 * (var_auxb1_dn5 + var_t2_dn5))) + ((var_auxb1_dn5 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn5))) + (var_k2 * (((var_dq2_dn5 * var_t2) + (var_dq2 * var_t2_dn5)) + ((var_q2_dn5 * assign8810_e8616) + (var_q2 * var_dqcoth_dn5)))));
        var_df_dn6 = (((var_aaux_dn6 - (var_k1 * (var_auxb1_dn6 + var_t2_dn6))) + ((var_auxb1_dn6 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn6))) + (var_k2 * (((var_dq2_dn6 * var_t2) + (var_dq2 * var_t2_dn6)) + ((var_q2_dn6 * assign8810_e8616) + (var_q2 * var_dqcoth_dn6)))));
        var_df_dn7 = (((var_aaux_dn7 - (var_k1 * (var_auxb1_dn7 + var_t2_dn7))) + ((var_auxb1_dn7 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn7))) + (var_k2 * (((var_dq2_dn7 * var_t2) + (var_dq2 * var_t2_dn7)) + ((var_q2_dn7 * assign8810_e8616) + (var_q2 * var_dqcoth_dn7)))));
        var_df_dn8 = (((var_aaux_dn8 - (var_k1 * (var_auxb1_dn8 + var_t2_dn8))) + ((var_auxb1_dn8 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn8))) + (var_k2 * (((var_dq2_dn8 * var_t2) + (var_dq2 * var_t2_dn8)) + ((var_q2_dn8 * assign8810_e8616) + (var_q2 * var_dqcoth_dn8)))));

        let assign8820_e8622: f64 = (-var_f);
        let assign8820_e8624: f64 = (assign8820_e8622 / var_df);
        var_delta = assign8820_e8624;
        var_delta_dn3 = ((((-var_f_dn3) * var_df) - (assign8820_e8622 * var_df_dn3)) / (var_df * var_df));
        var_delta_dn4 = ((((-var_f_dn4) * var_df) - (assign8820_e8622 * var_df_dn4)) / (var_df * var_df));
        var_delta_dn5 = ((((-var_f_dn5) * var_df) - (assign8820_e8622 * var_df_dn5)) / (var_df * var_df));
        var_delta_dn6 = ((((-var_f_dn6) * var_df) - (assign8820_e8622 * var_df_dn6)) / (var_df * var_df));
        var_delta_dn7 = ((((-var_f_dn7) * var_df) - (assign8820_e8622 * var_df_dn7)) / (var_df * var_df));
        var_delta_dn8 = ((((-var_f_dn8) * var_df) - (assign8820_e8622 * var_df_dn8)) / (var_df * var_df));

        let assign8830_e8627: f64 = (var_phi1 + var_delta);
        var_phi1 = assign8830_e8627;
        var_phi1_dn3 = (var_phi1_dn3 + var_delta_dn3);
        var_phi1_dn4 = (var_phi1_dn4 + var_delta_dn4);
        var_phi1_dn5 = (var_phi1_dn5 + var_delta_dn5);
        var_phi1_dn6 = (var_phi1_dn6 + var_delta_dn6);
        var_phi1_dn7 = (var_phi1_dn7 + var_delta_dn7);
        var_phi1_dn8 = (var_phi1_dn8 + var_delta_dn8);

        let assign8840_e8630: f64 = (var_xg1 - var_phi1);
        var_q1 = assign8840_e8630;
        var_q1_dn3 = (var_xg1_dn3 - var_phi1_dn3);
        var_q1_dn4 = (var_xg1_dn4 - var_phi1_dn4);
        var_q1_dn5 = (var_xg1_dn5 - var_phi1_dn5);
        var_q1_dn6 = (var_xg1_dn6 - var_phi1_dn6);
        var_q1_dn7 = (var_xg1_dn7 - var_phi1_dn7);
        var_q1_dn8 = (var_xg1_dn8 - var_phi1_dn8);

        *var_aaux_slot = var_aaux;
        *var_aaux_dn3_slot = var_aaux_dn3;
        *var_aaux_dn4_slot = var_aaux_dn4;
        *var_aaux_dn5_slot = var_aaux_dn5;
        *var_aaux_dn6_slot = var_aaux_dn6;
        *var_aaux_dn7_slot = var_aaux_dn7;
        *var_aaux_dn8_slot = var_aaux_dn8;
        *var_auxb1_slot = var_auxb1;
        *var_auxb1_dn3_slot = var_auxb1_dn3;
        *var_auxb1_dn4_slot = var_auxb1_dn4;
        *var_auxb1_dn5_slot = var_auxb1_dn5;
        *var_auxb1_dn6_slot = var_auxb1_dn6;
        *var_auxb1_dn7_slot = var_auxb1_dn7;
        *var_auxb1_dn8_slot = var_auxb1_dn8;
        *var_coth1_slot = var_coth1;
        *var_coth1_dn3_slot = var_coth1_dn3;
        *var_coth1_dn4_slot = var_coth1_dn4;
        *var_coth1_dn5_slot = var_coth1_dn5;
        *var_coth1_dn6_slot = var_coth1_dn6;
        *var_coth1_dn7_slot = var_coth1_dn7;
        *var_coth1_dn8_slot = var_coth1_dn8;
        *var_csc1_slot = var_csc1;
        *var_csc1_dn3_slot = var_csc1_dn3;
        *var_csc1_dn4_slot = var_csc1_dn4;
        *var_csc1_dn5_slot = var_csc1_dn5;
        *var_csc1_dn6_slot = var_csc1_dn6;
        *var_csc1_dn7_slot = var_csc1_dn7;
        *var_csc1_dn8_slot = var_csc1_dn8;
        *var_delta_slot = var_delta;
        *var_delta_dn3_slot = var_delta_dn3;
        *var_delta_dn4_slot = var_delta_dn4;
        *var_delta_dn5_slot = var_delta_dn5;
        *var_delta_dn6_slot = var_delta_dn6;
        *var_delta_dn7_slot = var_delta_dn7;
        *var_delta_dn8_slot = var_delta_dn8;
        *var_df_slot = var_df;
        *var_df_dn3_slot = var_df_dn3;
        *var_df_dn4_slot = var_df_dn4;
        *var_df_dn5_slot = var_df_dn5;
        *var_df_dn6_slot = var_df_dn6;
        *var_df_dn7_slot = var_df_dn7;
        *var_df_dn8_slot = var_df_dn8;
        *var_dg2_slot = var_dg2;
        *var_dg2_dn3_slot = var_dg2_dn3;
        *var_dg2_dn4_slot = var_dg2_dn4;
        *var_dg2_dn5_slot = var_dg2_dn5;
        *var_dg2_dn6_slot = var_dg2_dn6;
        *var_dg2_dn7_slot = var_dg2_dn7;
        *var_dg2_dn8_slot = var_dg2_dn8;
        *var_dlogsinhqsqdqsqrt_slot = var_dlogsinhqsqdqsqrt;
        *var_dlogsinhqsqdqsqrt_dn3_slot = var_dlogsinhqsqdqsqrt_dn3;
        *var_dlogsinhqsqdqsqrt_dn4_slot = var_dlogsinhqsqdqsqrt_dn4;
        *var_dlogsinhqsqdqsqrt_dn5_slot = var_dlogsinhqsqdqsqrt_dn5;
        *var_dlogsinhqsqdqsqrt_dn6_slot = var_dlogsinhqsqdqsqrt_dn6;
        *var_dlogsinhqsqdqsqrt_dn7_slot = var_dlogsinhqsqdqsqrt_dn7;
        *var_dlogsinhqsqdqsqrt_dn8_slot = var_dlogsinhqsqdqsqrt_dn8;
        *var_dq2_slot = var_dq2;
        *var_dq2_dn3_slot = var_dq2_dn3;
        *var_dq2_dn4_slot = var_dq2_dn4;
        *var_dq2_dn5_slot = var_dq2_dn5;
        *var_dq2_dn6_slot = var_dq2_dn6;
        *var_dq2_dn7_slot = var_dq2_dn7;
        *var_dq2_dn8_slot = var_dq2_dn8;
        *var_dqcoth_slot = var_dqcoth;
        *var_dqcoth_dn3_slot = var_dqcoth_dn3;
        *var_dqcoth_dn4_slot = var_dqcoth_dn4;
        *var_dqcoth_dn5_slot = var_dqcoth_dn5;
        *var_dqcoth_dn6_slot = var_dqcoth_dn6;
        *var_dqcoth_dn7_slot = var_dqcoth_dn7;
        *var_dqcoth_dn8_slot = var_dqcoth_dn8;
        *var_dqcothqdqsqrt_slot = var_dqcothqdqsqrt;
        *var_dqcothqdqsqrt_dn3_slot = var_dqcothqdqsqrt_dn3;
        *var_dqcothqdqsqrt_dn4_slot = var_dqcothqdqsqrt_dn4;
        *var_dqcothqdqsqrt_dn5_slot = var_dqcothqdqsqrt_dn5;
        *var_dqcothqdqsqrt_dn6_slot = var_dqcothqdqsqrt_dn6;
        *var_dqcothqdqsqrt_dn7_slot = var_dqcothqdqsqrt_dn7;
        *var_dqcothqdqsqrt_dn8_slot = var_dqcothqdqsqrt_dn8;
        *var_dqsqrt_slot = var_dqsqrt;
        *var_dqsqrt_dn3_slot = var_dqsqrt_dn3;
        *var_dqsqrt_dn4_slot = var_dqsqrt_dn4;
        *var_dqsqrt_dn5_slot = var_dqsqrt_dn5;
        *var_dqsqrt_dn6_slot = var_dqsqrt_dn6;
        *var_dqsqrt_dn7_slot = var_dqsqrt_dn7;
        *var_dqsqrt_dn8_slot = var_dqsqrt_dn8;
        *var_f_slot = var_f;
        *var_f_dn3_slot = var_f_dn3;
        *var_f_dn4_slot = var_f_dn4;
        *var_f_dn5_slot = var_f_dn5;
        *var_f_dn6_slot = var_f_dn6;
        *var_f_dn7_slot = var_f_dn7;
        *var_f_dn8_slot = var_f_dn8;
        *var_guard92_slot = var_guard92;
        *var_phi1_slot = var_phi1;
        *var_phi1_dn3_slot = var_phi1_dn3;
        *var_phi1_dn4_slot = var_phi1_dn4;
        *var_phi1_dn5_slot = var_phi1_dn5;
        *var_phi1_dn6_slot = var_phi1_dn6;
        *var_phi1_dn7_slot = var_phi1_dn7;
        *var_phi1_dn8_slot = var_phi1_dn8;
        *var_phissat_slot = var_phissat;
        *var_phissat_dn3_slot = var_phissat_dn3;
        *var_phissat_dn4_slot = var_phissat_dn4;
        *var_phissat_dn5_slot = var_phissat_dn5;
        *var_phissat_dn6_slot = var_phissat_dn6;
        *var_phissat_dn7_slot = var_phissat_dn7;
        *var_phissat_dn8_slot = var_phissat_dn8;
        *var_q_slot = var_q;
        *var_q1_slot = var_q1;
        *var_q1_dn3_slot = var_q1_dn3;
        *var_q1_dn4_slot = var_q1_dn4;
        *var_q1_dn5_slot = var_q1_dn5;
        *var_q1_dn6_slot = var_q1_dn6;
        *var_q1_dn7_slot = var_q1_dn7;
        *var_q1_dn8_slot = var_q1_dn8;
        *var_q2_slot = var_q2;
        *var_q2_dn3_slot = var_q2_dn3;
        *var_q2_dn4_slot = var_q2_dn4;
        *var_q2_dn5_slot = var_q2_dn5;
        *var_q2_dn6_slot = var_q2_dn6;
        *var_q2_dn7_slot = var_q2_dn7;
        *var_q2_dn8_slot = var_q2_dn8;
        *var_q_dn3_slot = var_q_dn3;
        *var_q_dn4_slot = var_q_dn4;
        *var_q_dn5_slot = var_q_dn5;
        *var_q_dn6_slot = var_q_dn6;
        *var_q_dn7_slot = var_q_dn7;
        *var_q_dn8_slot = var_q_dn8;
        *var_qcoth_slot = var_qcoth;
        *var_qcoth_dn3_slot = var_qcoth_dn3;
        *var_qcoth_dn4_slot = var_qcoth_dn4;
        *var_qcoth_dn5_slot = var_qcoth_dn5;
        *var_qcoth_dn6_slot = var_qcoth_dn6;
        *var_qcoth_dn7_slot = var_qcoth_dn7;
        *var_qcoth_dn8_slot = var_qcoth_dn8;
        *var_qsqrt_slot = var_qsqrt;
        *var_qsqrt_dn3_slot = var_qsqrt_dn3;
        *var_qsqrt_dn4_slot = var_qsqrt_dn4;
        *var_qsqrt_dn5_slot = var_qsqrt_dn5;
        *var_qsqrt_dn6_slot = var_qsqrt_dn6;
        *var_qsqrt_dn7_slot = var_qsqrt_dn7;
        *var_qsqrt_dn8_slot = var_qsqrt_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_xg1_slot = var_xg1;
        *var_xg1_dn3_slot = var_xg1_dn3;
        *var_xg1_dn4_slot = var_xg1_dn4;
        *var_xg1_dn5_slot = var_xg1_dn5;
        *var_xg1_dn6_slot = var_xg1_dn6;
        *var_xg1_dn7_slot = var_xg1_dn7;
        *var_xg1_dn8_slot = var_xg1_dn8;
    }

    pub(super) fn stamp_transient_block_18(
        var_a0: f64,
        var_a0_dn3: f64,
        var_a0_dn4: f64,
        var_a0_dn5: f64,
        var_a0_dn6: f64,
        var_a0_dn7: f64,
        var_a0_dn8: f64,
        var_k1: f64,
        var_k2: f64,
        var_xg1: f64,
        var_xg1_dn3: f64,
        var_xg1_dn4: f64,
        var_xg1_dn5: f64,
        var_xg1_dn6: f64,
        var_xg1_dn7: f64,
        var_xg1_dn8: f64,
        var_xg2: f64,
        var_xg2_dn3: f64,
        var_xg2_dn4: f64,
        var_xg2_dn5: f64,
        var_xg2_dn6: f64,
        var_xg2_dn7: f64,
        var_xg2_dn8: f64,
        var_aaux_slot: &mut f64,
        var_aaux_dn3_slot: &mut f64,
        var_aaux_dn4_slot: &mut f64,
        var_aaux_dn5_slot: &mut f64,
        var_aaux_dn6_slot: &mut f64,
        var_aaux_dn7_slot: &mut f64,
        var_aaux_dn8_slot: &mut f64,
        var_auxb1_slot: &mut f64,
        var_auxb1_dn3_slot: &mut f64,
        var_auxb1_dn4_slot: &mut f64,
        var_auxb1_dn5_slot: &mut f64,
        var_auxb1_dn6_slot: &mut f64,
        var_auxb1_dn7_slot: &mut f64,
        var_auxb1_dn8_slot: &mut f64,
        var_coth1_slot: &mut f64,
        var_coth1_dn3_slot: &mut f64,
        var_coth1_dn4_slot: &mut f64,
        var_coth1_dn5_slot: &mut f64,
        var_coth1_dn6_slot: &mut f64,
        var_coth1_dn7_slot: &mut f64,
        var_coth1_dn8_slot: &mut f64,
        var_csc1_slot: &mut f64,
        var_csc1_dn3_slot: &mut f64,
        var_csc1_dn4_slot: &mut f64,
        var_csc1_dn5_slot: &mut f64,
        var_csc1_dn6_slot: &mut f64,
        var_csc1_dn7_slot: &mut f64,
        var_csc1_dn8_slot: &mut f64,
        var_delta_slot: &mut f64,
        var_delta_dn3_slot: &mut f64,
        var_delta_dn4_slot: &mut f64,
        var_delta_dn5_slot: &mut f64,
        var_delta_dn6_slot: &mut f64,
        var_delta_dn7_slot: &mut f64,
        var_delta_dn8_slot: &mut f64,
        var_df_slot: &mut f64,
        var_df_dn3_slot: &mut f64,
        var_df_dn4_slot: &mut f64,
        var_df_dn5_slot: &mut f64,
        var_df_dn6_slot: &mut f64,
        var_df_dn7_slot: &mut f64,
        var_df_dn8_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn3_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn4_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn5_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn6_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn7_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn8_slot: &mut f64,
        var_dq2_slot: &mut f64,
        var_dq2_dn3_slot: &mut f64,
        var_dq2_dn4_slot: &mut f64,
        var_dq2_dn5_slot: &mut f64,
        var_dq2_dn6_slot: &mut f64,
        var_dq2_dn7_slot: &mut f64,
        var_dq2_dn8_slot: &mut f64,
        var_dqcoth_slot: &mut f64,
        var_dqcoth_dn3_slot: &mut f64,
        var_dqcoth_dn4_slot: &mut f64,
        var_dqcoth_dn5_slot: &mut f64,
        var_dqcoth_dn6_slot: &mut f64,
        var_dqcoth_dn7_slot: &mut f64,
        var_dqcoth_dn8_slot: &mut f64,
        var_dqcothqdqsqrt_slot: &mut f64,
        var_dqcothqdqsqrt_dn3_slot: &mut f64,
        var_dqcothqdqsqrt_dn4_slot: &mut f64,
        var_dqcothqdqsqrt_dn5_slot: &mut f64,
        var_dqcothqdqsqrt_dn6_slot: &mut f64,
        var_dqcothqdqsqrt_dn7_slot: &mut f64,
        var_dqcothqdqsqrt_dn8_slot: &mut f64,
        var_dqsqrt_slot: &mut f64,
        var_dqsqrt_dn3_slot: &mut f64,
        var_dqsqrt_dn4_slot: &mut f64,
        var_dqsqrt_dn5_slot: &mut f64,
        var_dqsqrt_dn6_slot: &mut f64,
        var_dqsqrt_dn7_slot: &mut f64,
        var_dqsqrt_dn8_slot: &mut f64,
        var_f_slot: &mut f64,
        var_f_dn3_slot: &mut f64,
        var_f_dn4_slot: &mut f64,
        var_f_dn5_slot: &mut f64,
        var_f_dn6_slot: &mut f64,
        var_f_dn7_slot: &mut f64,
        var_f_dn8_slot: &mut f64,
        var_guard93_slot: &mut f64,
        var_guard94_slot: &mut f64,
        var_phi1_slot: &mut f64,
        var_phi1_dn3_slot: &mut f64,
        var_phi1_dn4_slot: &mut f64,
        var_phi1_dn5_slot: &mut f64,
        var_phi1_dn6_slot: &mut f64,
        var_phi1_dn7_slot: &mut f64,
        var_phi1_dn8_slot: &mut f64,
        var_q_slot: &mut f64,
        var_q1_slot: &mut f64,
        var_q1_dn3_slot: &mut f64,
        var_q1_dn4_slot: &mut f64,
        var_q1_dn5_slot: &mut f64,
        var_q1_dn6_slot: &mut f64,
        var_q1_dn7_slot: &mut f64,
        var_q1_dn8_slot: &mut f64,
        var_q2_slot: &mut f64,
        var_q2_dn3_slot: &mut f64,
        var_q2_dn4_slot: &mut f64,
        var_q2_dn5_slot: &mut f64,
        var_q2_dn6_slot: &mut f64,
        var_q2_dn7_slot: &mut f64,
        var_q2_dn8_slot: &mut f64,
        var_q_dn3_slot: &mut f64,
        var_q_dn4_slot: &mut f64,
        var_q_dn5_slot: &mut f64,
        var_q_dn6_slot: &mut f64,
        var_q_dn7_slot: &mut f64,
        var_q_dn8_slot: &mut f64,
        var_qcoth_slot: &mut f64,
        var_qcoth_dn3_slot: &mut f64,
        var_qcoth_dn4_slot: &mut f64,
        var_qcoth_dn5_slot: &mut f64,
        var_qcoth_dn6_slot: &mut f64,
        var_qcoth_dn7_slot: &mut f64,
        var_qcoth_dn8_slot: &mut f64,
        var_qsqrt_slot: &mut f64,
        var_qsqrt_dn3_slot: &mut f64,
        var_qsqrt_dn4_slot: &mut f64,
        var_qsqrt_dn5_slot: &mut f64,
        var_qsqrt_dn6_slot: &mut f64,
        var_qsqrt_dn7_slot: &mut f64,
        var_qsqrt_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
    ) {
        let mut var_aaux: f64 = *var_aaux_slot;
        let mut var_aaux_dn3: f64 = *var_aaux_dn3_slot;
        let mut var_aaux_dn4: f64 = *var_aaux_dn4_slot;
        let mut var_aaux_dn5: f64 = *var_aaux_dn5_slot;
        let mut var_aaux_dn6: f64 = *var_aaux_dn6_slot;
        let mut var_aaux_dn7: f64 = *var_aaux_dn7_slot;
        let mut var_aaux_dn8: f64 = *var_aaux_dn8_slot;
        let mut var_auxb1: f64 = *var_auxb1_slot;
        let mut var_auxb1_dn3: f64 = *var_auxb1_dn3_slot;
        let mut var_auxb1_dn4: f64 = *var_auxb1_dn4_slot;
        let mut var_auxb1_dn5: f64 = *var_auxb1_dn5_slot;
        let mut var_auxb1_dn6: f64 = *var_auxb1_dn6_slot;
        let mut var_auxb1_dn7: f64 = *var_auxb1_dn7_slot;
        let mut var_auxb1_dn8: f64 = *var_auxb1_dn8_slot;
        let mut var_coth1: f64 = *var_coth1_slot;
        let mut var_coth1_dn3: f64 = *var_coth1_dn3_slot;
        let mut var_coth1_dn4: f64 = *var_coth1_dn4_slot;
        let mut var_coth1_dn5: f64 = *var_coth1_dn5_slot;
        let mut var_coth1_dn6: f64 = *var_coth1_dn6_slot;
        let mut var_coth1_dn7: f64 = *var_coth1_dn7_slot;
        let mut var_coth1_dn8: f64 = *var_coth1_dn8_slot;
        let mut var_csc1: f64 = *var_csc1_slot;
        let mut var_csc1_dn3: f64 = *var_csc1_dn3_slot;
        let mut var_csc1_dn4: f64 = *var_csc1_dn4_slot;
        let mut var_csc1_dn5: f64 = *var_csc1_dn5_slot;
        let mut var_csc1_dn6: f64 = *var_csc1_dn6_slot;
        let mut var_csc1_dn7: f64 = *var_csc1_dn7_slot;
        let mut var_csc1_dn8: f64 = *var_csc1_dn8_slot;
        let mut var_delta: f64 = *var_delta_slot;
        let mut var_delta_dn3: f64 = *var_delta_dn3_slot;
        let mut var_delta_dn4: f64 = *var_delta_dn4_slot;
        let mut var_delta_dn5: f64 = *var_delta_dn5_slot;
        let mut var_delta_dn6: f64 = *var_delta_dn6_slot;
        let mut var_delta_dn7: f64 = *var_delta_dn7_slot;
        let mut var_delta_dn8: f64 = *var_delta_dn8_slot;
        let mut var_df: f64 = *var_df_slot;
        let mut var_df_dn3: f64 = *var_df_dn3_slot;
        let mut var_df_dn4: f64 = *var_df_dn4_slot;
        let mut var_df_dn5: f64 = *var_df_dn5_slot;
        let mut var_df_dn6: f64 = *var_df_dn6_slot;
        let mut var_df_dn7: f64 = *var_df_dn7_slot;
        let mut var_df_dn8: f64 = *var_df_dn8_slot;
        let mut var_dlogsinhqsqdqsqrt: f64 = *var_dlogsinhqsqdqsqrt_slot;
        let mut var_dlogsinhqsqdqsqrt_dn3: f64 = *var_dlogsinhqsqdqsqrt_dn3_slot;
        let mut var_dlogsinhqsqdqsqrt_dn4: f64 = *var_dlogsinhqsqdqsqrt_dn4_slot;
        let mut var_dlogsinhqsqdqsqrt_dn5: f64 = *var_dlogsinhqsqdqsqrt_dn5_slot;
        let mut var_dlogsinhqsqdqsqrt_dn6: f64 = *var_dlogsinhqsqdqsqrt_dn6_slot;
        let mut var_dlogsinhqsqdqsqrt_dn7: f64 = *var_dlogsinhqsqdqsqrt_dn7_slot;
        let mut var_dlogsinhqsqdqsqrt_dn8: f64 = *var_dlogsinhqsqdqsqrt_dn8_slot;
        let mut var_dq2: f64 = *var_dq2_slot;
        let mut var_dq2_dn3: f64 = *var_dq2_dn3_slot;
        let mut var_dq2_dn4: f64 = *var_dq2_dn4_slot;
        let mut var_dq2_dn5: f64 = *var_dq2_dn5_slot;
        let mut var_dq2_dn6: f64 = *var_dq2_dn6_slot;
        let mut var_dq2_dn7: f64 = *var_dq2_dn7_slot;
        let mut var_dq2_dn8: f64 = *var_dq2_dn8_slot;
        let mut var_dqcoth: f64 = *var_dqcoth_slot;
        let mut var_dqcoth_dn3: f64 = *var_dqcoth_dn3_slot;
        let mut var_dqcoth_dn4: f64 = *var_dqcoth_dn4_slot;
        let mut var_dqcoth_dn5: f64 = *var_dqcoth_dn5_slot;
        let mut var_dqcoth_dn6: f64 = *var_dqcoth_dn6_slot;
        let mut var_dqcoth_dn7: f64 = *var_dqcoth_dn7_slot;
        let mut var_dqcoth_dn8: f64 = *var_dqcoth_dn8_slot;
        let mut var_dqcothqdqsqrt: f64 = *var_dqcothqdqsqrt_slot;
        let mut var_dqcothqdqsqrt_dn3: f64 = *var_dqcothqdqsqrt_dn3_slot;
        let mut var_dqcothqdqsqrt_dn4: f64 = *var_dqcothqdqsqrt_dn4_slot;
        let mut var_dqcothqdqsqrt_dn5: f64 = *var_dqcothqdqsqrt_dn5_slot;
        let mut var_dqcothqdqsqrt_dn6: f64 = *var_dqcothqdqsqrt_dn6_slot;
        let mut var_dqcothqdqsqrt_dn7: f64 = *var_dqcothqdqsqrt_dn7_slot;
        let mut var_dqcothqdqsqrt_dn8: f64 = *var_dqcothqdqsqrt_dn8_slot;
        let mut var_dqsqrt: f64 = *var_dqsqrt_slot;
        let mut var_dqsqrt_dn3: f64 = *var_dqsqrt_dn3_slot;
        let mut var_dqsqrt_dn4: f64 = *var_dqsqrt_dn4_slot;
        let mut var_dqsqrt_dn5: f64 = *var_dqsqrt_dn5_slot;
        let mut var_dqsqrt_dn6: f64 = *var_dqsqrt_dn6_slot;
        let mut var_dqsqrt_dn7: f64 = *var_dqsqrt_dn7_slot;
        let mut var_dqsqrt_dn8: f64 = *var_dqsqrt_dn8_slot;
        let mut var_f: f64 = *var_f_slot;
        let mut var_f_dn3: f64 = *var_f_dn3_slot;
        let mut var_f_dn4: f64 = *var_f_dn4_slot;
        let mut var_f_dn5: f64 = *var_f_dn5_slot;
        let mut var_f_dn6: f64 = *var_f_dn6_slot;
        let mut var_f_dn7: f64 = *var_f_dn7_slot;
        let mut var_f_dn8: f64 = *var_f_dn8_slot;
        let mut var_guard93: f64 = *var_guard93_slot;
        let mut var_guard94: f64 = *var_guard94_slot;
        let mut var_phi1: f64 = *var_phi1_slot;
        let mut var_phi1_dn3: f64 = *var_phi1_dn3_slot;
        let mut var_phi1_dn4: f64 = *var_phi1_dn4_slot;
        let mut var_phi1_dn5: f64 = *var_phi1_dn5_slot;
        let mut var_phi1_dn6: f64 = *var_phi1_dn6_slot;
        let mut var_phi1_dn7: f64 = *var_phi1_dn7_slot;
        let mut var_phi1_dn8: f64 = *var_phi1_dn8_slot;
        let mut var_q: f64 = *var_q_slot;
        let mut var_q1: f64 = *var_q1_slot;
        let mut var_q1_dn3: f64 = *var_q1_dn3_slot;
        let mut var_q1_dn4: f64 = *var_q1_dn4_slot;
        let mut var_q1_dn5: f64 = *var_q1_dn5_slot;
        let mut var_q1_dn6: f64 = *var_q1_dn6_slot;
        let mut var_q1_dn7: f64 = *var_q1_dn7_slot;
        let mut var_q1_dn8: f64 = *var_q1_dn8_slot;
        let mut var_q2: f64 = *var_q2_slot;
        let mut var_q2_dn3: f64 = *var_q2_dn3_slot;
        let mut var_q2_dn4: f64 = *var_q2_dn4_slot;
        let mut var_q2_dn5: f64 = *var_q2_dn5_slot;
        let mut var_q2_dn6: f64 = *var_q2_dn6_slot;
        let mut var_q2_dn7: f64 = *var_q2_dn7_slot;
        let mut var_q2_dn8: f64 = *var_q2_dn8_slot;
        let mut var_q_dn3: f64 = *var_q_dn3_slot;
        let mut var_q_dn4: f64 = *var_q_dn4_slot;
        let mut var_q_dn5: f64 = *var_q_dn5_slot;
        let mut var_q_dn6: f64 = *var_q_dn6_slot;
        let mut var_q_dn7: f64 = *var_q_dn7_slot;
        let mut var_q_dn8: f64 = *var_q_dn8_slot;
        let mut var_qcoth: f64 = *var_qcoth_slot;
        let mut var_qcoth_dn3: f64 = *var_qcoth_dn3_slot;
        let mut var_qcoth_dn4: f64 = *var_qcoth_dn4_slot;
        let mut var_qcoth_dn5: f64 = *var_qcoth_dn5_slot;
        let mut var_qcoth_dn6: f64 = *var_qcoth_dn6_slot;
        let mut var_qcoth_dn7: f64 = *var_qcoth_dn7_slot;
        let mut var_qcoth_dn8: f64 = *var_qcoth_dn8_slot;
        let mut var_qsqrt: f64 = *var_qsqrt_slot;
        let mut var_qsqrt_dn3: f64 = *var_qsqrt_dn3_slot;
        let mut var_qsqrt_dn4: f64 = *var_qsqrt_dn4_slot;
        let mut var_qsqrt_dn5: f64 = *var_qsqrt_dn5_slot;
        let mut var_qsqrt_dn6: f64 = *var_qsqrt_dn6_slot;
        let mut var_qsqrt_dn7: f64 = *var_qsqrt_dn7_slot;
        let mut var_qsqrt_dn8: f64 = *var_qsqrt_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;

        let assign8850_e8633: f64 = (var_k1 * var_q1);
        var_auxb1 = assign8850_e8633;
        var_auxb1_dn3 = (var_k1 * var_q1_dn3);
        var_auxb1_dn4 = (var_k1 * var_q1_dn4);
        var_auxb1_dn5 = (var_k1 * var_q1_dn5);
        var_auxb1_dn6 = (var_k1 * var_q1_dn6);
        var_auxb1_dn7 = (var_k1 * var_q1_dn7);
        var_auxb1_dn8 = (var_k1 * var_q1_dn8);

        let assign8860_e8635: f64 = (-var_a0);
        let assign8860_e8637: f64 = (var_phi1).exp();
        let assign8860_e8638: f64 = (assign8860_e8635 * assign8860_e8637);
        var_aaux = assign8860_e8638;
        var_aaux_dn3 = (((-var_a0_dn3) * assign8860_e8637) + (assign8860_e8635 * (assign8860_e8637 * var_phi1_dn3)));
        var_aaux_dn4 = (((-var_a0_dn4) * assign8860_e8637) + (assign8860_e8635 * (assign8860_e8637 * var_phi1_dn4)));
        var_aaux_dn5 = (((-var_a0_dn5) * assign8860_e8637) + (assign8860_e8635 * (assign8860_e8637 * var_phi1_dn5)));
        var_aaux_dn6 = (((-var_a0_dn6) * assign8860_e8637) + (assign8860_e8635 * (assign8860_e8637 * var_phi1_dn6)));
        var_aaux_dn7 = (((-var_a0_dn7) * assign8860_e8637) + (assign8860_e8635 * (assign8860_e8637 * var_phi1_dn7)));
        var_aaux_dn8 = (((-var_a0_dn8) * assign8860_e8637) + (assign8860_e8635 * (assign8860_e8637 * var_phi1_dn8)));

        let assign8870_e8641: f64 = (var_auxb1 * var_auxb1);
        let assign8870_e8643: f64 = (assign8870_e8641 + var_aaux);
        var_qsqrt = assign8870_e8643;
        var_qsqrt_dn3 = (((var_auxb1_dn3 * var_auxb1) + (var_auxb1 * var_auxb1_dn3)) + var_aaux_dn3);
        var_qsqrt_dn4 = (((var_auxb1_dn4 * var_auxb1) + (var_auxb1 * var_auxb1_dn4)) + var_aaux_dn4);
        var_qsqrt_dn5 = (((var_auxb1_dn5 * var_auxb1) + (var_auxb1 * var_auxb1_dn5)) + var_aaux_dn5);
        var_qsqrt_dn6 = (((var_auxb1_dn6 * var_auxb1) + (var_auxb1 * var_auxb1_dn6)) + var_aaux_dn6);
        var_qsqrt_dn7 = (((var_auxb1_dn7 * var_auxb1) + (var_auxb1 * var_auxb1_dn7)) + var_aaux_dn7);
        var_qsqrt_dn8 = (((var_auxb1_dn8 * var_auxb1) + (var_auxb1 * var_auxb1_dn8)) + var_aaux_dn8);

        let assign8880_e8646: f64 = if var_qsqrt < 0.0 { 1.0 } else { 0.0 };
        var_guard93 = assign8880_e8646;

        let (assign8890_e8652, assign8890_e8652_d_n3, assign8890_e8652_d_n4, assign8890_e8652_d_n5, assign8890_e8652_d_n6, assign8890_e8652_d_n7, assign8890_e8652_d_n8,) = {
    if (var_guard93 != 0.0) {
        let assign8890_e8649: f64 = (-var_qsqrt);
        let assign8890_e8650: f64 = (assign8890_e8649).sqrt();
        (assign8890_e8650, ((-var_qsqrt_dn3) / (2.0 * assign8890_e8650)), ((-var_qsqrt_dn4) / (2.0 * assign8890_e8650)), ((-var_qsqrt_dn5) / (2.0 * assign8890_e8650)), ((-var_qsqrt_dn6) / (2.0 * assign8890_e8650)), ((-var_qsqrt_dn7) / (2.0 * assign8890_e8650)), ((-var_qsqrt_dn8) / (2.0 * assign8890_e8650)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign8890_e8652;
        var_q_dn3 = assign8890_e8652_d_n3;
        var_q_dn4 = assign8890_e8652_d_n4;
        var_q_dn5 = assign8890_e8652_d_n5;
        var_q_dn6 = assign8890_e8652_d_n6;
        var_q_dn7 = assign8890_e8652_d_n7;
        var_q_dn8 = assign8890_e8652_d_n8;

        let (assign8900_e8661, assign8900_e8661_d_n3, assign8900_e8661_d_n4, assign8900_e8661_d_n5, assign8900_e8661_d_n6, assign8900_e8661_d_n7, assign8900_e8661_d_n8,) = {
    if (var_guard93 != 0.0) {
        let assign8900_e8657: f64 = (0.5 * var_q);
        let assign8900_e8658: f64 = (assign8900_e8657).sin();
        let assign8900_e8659: f64 = (1.0 / assign8900_e8658);
        (assign8900_e8659, (-(((assign8900_e8657).cos() * (0.5 * var_q_dn3)) / (assign8900_e8658 * assign8900_e8658))), (-(((assign8900_e8657).cos() * (0.5 * var_q_dn4)) / (assign8900_e8658 * assign8900_e8658))), (-(((assign8900_e8657).cos() * (0.5 * var_q_dn5)) / (assign8900_e8658 * assign8900_e8658))), (-(((assign8900_e8657).cos() * (0.5 * var_q_dn6)) / (assign8900_e8658 * assign8900_e8658))), (-(((assign8900_e8657).cos() * (0.5 * var_q_dn7)) / (assign8900_e8658 * assign8900_e8658))), (-(((assign8900_e8657).cos() * (0.5 * var_q_dn8)) / (assign8900_e8658 * assign8900_e8658))),)
    } else {
        (var_csc1, var_csc1_dn3, var_csc1_dn4, var_csc1_dn5, var_csc1_dn6, var_csc1_dn7, var_csc1_dn8,)
    }
};
        var_csc1 = assign8900_e8661;
        var_csc1_dn3 = assign8900_e8661_d_n3;
        var_csc1_dn4 = assign8900_e8661_d_n4;
        var_csc1_dn5 = assign8900_e8661_d_n5;
        var_csc1_dn6 = assign8900_e8661_d_n6;
        var_csc1_dn7 = assign8900_e8661_d_n7;
        var_csc1_dn8 = assign8900_e8661_d_n8;

        let (assign8910_e8667, assign8910_e8667_d_n3, assign8910_e8667_d_n4, assign8910_e8667_d_n5, assign8910_e8667_d_n6, assign8910_e8667_d_n7, assign8910_e8667_d_n8,) = {
    if (var_guard93 != 0.0) {
        let assign8910_e8665: f64 = (var_csc1 * var_csc1);
        (assign8910_e8665, ((var_csc1_dn3 * var_csc1) + (var_csc1 * var_csc1_dn3)), ((var_csc1_dn4 * var_csc1) + (var_csc1 * var_csc1_dn4)), ((var_csc1_dn5 * var_csc1) + (var_csc1 * var_csc1_dn5)), ((var_csc1_dn6 * var_csc1) + (var_csc1 * var_csc1_dn6)), ((var_csc1_dn7 * var_csc1) + (var_csc1 * var_csc1_dn7)), ((var_csc1_dn8 * var_csc1) + (var_csc1 * var_csc1_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign8910_e8667;
        var_t1_dn3 = assign8910_e8667_d_n3;
        var_t1_dn4 = assign8910_e8667_d_n4;
        var_t1_dn5 = assign8910_e8667_d_n5;
        var_t1_dn6 = assign8910_e8667_d_n6;
        var_t1_dn7 = assign8910_e8667_d_n7;
        var_t1_dn8 = assign8910_e8667_d_n8;

        let (assign8920_e8676, assign8920_e8676_d_n3, assign8920_e8676_d_n4, assign8920_e8676_d_n5, assign8920_e8676_d_n6, assign8920_e8676_d_n7, assign8920_e8676_d_n8,) = {
    if (var_guard93 != 0.0) {
        let assign8920_e8671: f64 = (0.5 * var_q);
        let assign8920_e8672: f64 = (assign8920_e8671).cos();
        let assign8920_e8674: f64 = (assign8920_e8672 * var_csc1);
        (assign8920_e8674, (((-(assign8920_e8671).sin() * (0.5 * var_q_dn3)) * var_csc1) + (assign8920_e8672 * var_csc1_dn3)), (((-(assign8920_e8671).sin() * (0.5 * var_q_dn4)) * var_csc1) + (assign8920_e8672 * var_csc1_dn4)), (((-(assign8920_e8671).sin() * (0.5 * var_q_dn5)) * var_csc1) + (assign8920_e8672 * var_csc1_dn5)), (((-(assign8920_e8671).sin() * (0.5 * var_q_dn6)) * var_csc1) + (assign8920_e8672 * var_csc1_dn6)), (((-(assign8920_e8671).sin() * (0.5 * var_q_dn7)) * var_csc1) + (assign8920_e8672 * var_csc1_dn7)), (((-(assign8920_e8671).sin() * (0.5 * var_q_dn8)) * var_csc1) + (assign8920_e8672 * var_csc1_dn8)),)
    } else {
        (var_coth1, var_coth1_dn3, var_coth1_dn4, var_coth1_dn5, var_coth1_dn6, var_coth1_dn7, var_coth1_dn8,)
    }
};
        var_coth1 = assign8920_e8676;
        var_coth1_dn3 = assign8920_e8676_d_n3;
        var_coth1_dn4 = assign8920_e8676_d_n4;
        var_coth1_dn5 = assign8920_e8676_d_n5;
        var_coth1_dn6 = assign8920_e8676_d_n6;
        var_coth1_dn7 = assign8920_e8676_d_n7;
        var_coth1_dn8 = assign8920_e8676_d_n8;

        let (assign8930_e8685, assign8930_e8685_d_n3, assign8930_e8685_d_n4, assign8930_e8685_d_n5, assign8930_e8685_d_n6, assign8930_e8685_d_n7, assign8930_e8685_d_n8,) = {
    if (var_guard93 != 0.0) {
        let assign8930_e8679: f64 = (-0.5);
        let assign8930_e8681: f64 = (assign8930_e8679 * var_coth1);
        let assign8930_e8683: f64 = (assign8930_e8681 / var_q);
        (assign8930_e8683, ((((assign8930_e8679 * var_coth1_dn3) * var_q) - (assign8930_e8681 * var_q_dn3)) / (var_q * var_q)), ((((assign8930_e8679 * var_coth1_dn4) * var_q) - (assign8930_e8681 * var_q_dn4)) / (var_q * var_q)), ((((assign8930_e8679 * var_coth1_dn5) * var_q) - (assign8930_e8681 * var_q_dn5)) / (var_q * var_q)), ((((assign8930_e8679 * var_coth1_dn6) * var_q) - (assign8930_e8681 * var_q_dn6)) / (var_q * var_q)), ((((assign8930_e8679 * var_coth1_dn7) * var_q) - (assign8930_e8681 * var_q_dn7)) / (var_q * var_q)), ((((assign8930_e8679 * var_coth1_dn8) * var_q) - (assign8930_e8681 * var_q_dn8)) / (var_q * var_q)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign8930_e8685;
        var_t0_dn3 = assign8930_e8685_d_n3;
        var_t0_dn4 = assign8930_e8685_d_n4;
        var_t0_dn5 = assign8930_e8685_d_n5;
        var_t0_dn6 = assign8930_e8685_d_n6;
        var_t0_dn7 = assign8930_e8685_d_n7;
        var_t0_dn8 = assign8930_e8685_d_n8;

        let (assign8940_e8693, assign8940_e8693_d_n3, assign8940_e8693_d_n4, assign8940_e8693_d_n5, assign8940_e8693_d_n6, assign8940_e8693_d_n7, assign8940_e8693_d_n8,) = {
    if (var_guard93 != 0.0) {
        let assign8940_e8689: f64 = (0.25 * var_t1);
        let assign8940_e8691: f64 = (assign8940_e8689 + var_t0);
        (assign8940_e8691, ((0.25 * var_t1_dn3) + var_t0_dn3), ((0.25 * var_t1_dn4) + var_t0_dn4), ((0.25 * var_t1_dn5) + var_t0_dn5), ((0.25 * var_t1_dn6) + var_t0_dn6), ((0.25 * var_t1_dn7) + var_t0_dn7), ((0.25 * var_t1_dn8) + var_t0_dn8),)
    } else {
        (var_dqcothqdqsqrt, var_dqcothqdqsqrt_dn3, var_dqcothqdqsqrt_dn4, var_dqcothqdqsqrt_dn5, var_dqcothqdqsqrt_dn6, var_dqcothqdqsqrt_dn7, var_dqcothqdqsqrt_dn8,)
    }
};
        var_dqcothqdqsqrt = assign8940_e8693;
        var_dqcothqdqsqrt_dn3 = assign8940_e8693_d_n3;
        var_dqcothqdqsqrt_dn4 = assign8940_e8693_d_n4;
        var_dqcothqdqsqrt_dn5 = assign8940_e8693_d_n5;
        var_dqcothqdqsqrt_dn6 = assign8940_e8693_d_n6;
        var_dqcothqdqsqrt_dn7 = assign8940_e8693_d_n7;
        var_dqcothqdqsqrt_dn8 = assign8940_e8693_d_n8;

        let (assign8950_e8699, assign8950_e8699_d_n3, assign8950_e8699_d_n4, assign8950_e8699_d_n5, assign8950_e8699_d_n6, assign8950_e8699_d_n7, assign8950_e8699_d_n8,) = {
    if (var_guard93 == 0.0) {
        let assign8950_e8697: f64 = (var_qsqrt).sqrt();
        (assign8950_e8697, (var_qsqrt_dn3 / (2.0 * assign8950_e8697)), (var_qsqrt_dn4 / (2.0 * assign8950_e8697)), (var_qsqrt_dn5 / (2.0 * assign8950_e8697)), (var_qsqrt_dn6 / (2.0 * assign8950_e8697)), (var_qsqrt_dn7 / (2.0 * assign8950_e8697)), (var_qsqrt_dn8 / (2.0 * assign8950_e8697)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign8950_e8699;
        var_q_dn3 = assign8950_e8699_d_n3;
        var_q_dn4 = assign8950_e8699_d_n4;
        var_q_dn5 = assign8950_e8699_d_n5;
        var_q_dn6 = assign8950_e8699_d_n6;
        var_q_dn7 = assign8950_e8699_d_n7;
        var_q_dn8 = assign8950_e8699_d_n8;

        let (assign8960_e8709, assign8960_e8709_d_n3, assign8960_e8709_d_n4, assign8960_e8709_d_n5, assign8960_e8709_d_n6, assign8960_e8709_d_n7, assign8960_e8709_d_n8,) = {
    if (var_guard93 == 0.0) {
        let assign8960_e8705: f64 = (0.5 * var_q);
        let assign8960_e8706: f64 = (assign8960_e8705).sinh();
        let assign8960_e8707: f64 = (1.0 / assign8960_e8706);
        (assign8960_e8707, (-(((assign8960_e8705).cosh() * (0.5 * var_q_dn3)) / (assign8960_e8706 * assign8960_e8706))), (-(((assign8960_e8705).cosh() * (0.5 * var_q_dn4)) / (assign8960_e8706 * assign8960_e8706))), (-(((assign8960_e8705).cosh() * (0.5 * var_q_dn5)) / (assign8960_e8706 * assign8960_e8706))), (-(((assign8960_e8705).cosh() * (0.5 * var_q_dn6)) / (assign8960_e8706 * assign8960_e8706))), (-(((assign8960_e8705).cosh() * (0.5 * var_q_dn7)) / (assign8960_e8706 * assign8960_e8706))), (-(((assign8960_e8705).cosh() * (0.5 * var_q_dn8)) / (assign8960_e8706 * assign8960_e8706))),)
    } else {
        (var_csc1, var_csc1_dn3, var_csc1_dn4, var_csc1_dn5, var_csc1_dn6, var_csc1_dn7, var_csc1_dn8,)
    }
};
        var_csc1 = assign8960_e8709;
        var_csc1_dn3 = assign8960_e8709_d_n3;
        var_csc1_dn4 = assign8960_e8709_d_n4;
        var_csc1_dn5 = assign8960_e8709_d_n5;
        var_csc1_dn6 = assign8960_e8709_d_n6;
        var_csc1_dn7 = assign8960_e8709_d_n7;
        var_csc1_dn8 = assign8960_e8709_d_n8;

        let (assign8970_e8716, assign8970_e8716_d_n3, assign8970_e8716_d_n4, assign8970_e8716_d_n5, assign8970_e8716_d_n6, assign8970_e8716_d_n7, assign8970_e8716_d_n8,) = {
    if (var_guard93 == 0.0) {
        let assign8970_e8714: f64 = (var_csc1 * var_csc1);
        (assign8970_e8714, ((var_csc1_dn3 * var_csc1) + (var_csc1 * var_csc1_dn3)), ((var_csc1_dn4 * var_csc1) + (var_csc1 * var_csc1_dn4)), ((var_csc1_dn5 * var_csc1) + (var_csc1 * var_csc1_dn5)), ((var_csc1_dn6 * var_csc1) + (var_csc1 * var_csc1_dn6)), ((var_csc1_dn7 * var_csc1) + (var_csc1 * var_csc1_dn7)), ((var_csc1_dn8 * var_csc1) + (var_csc1 * var_csc1_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign8970_e8716;
        var_t1_dn3 = assign8970_e8716_d_n3;
        var_t1_dn4 = assign8970_e8716_d_n4;
        var_t1_dn5 = assign8970_e8716_d_n5;
        var_t1_dn6 = assign8970_e8716_d_n6;
        var_t1_dn7 = assign8970_e8716_d_n7;
        var_t1_dn8 = assign8970_e8716_d_n8;

        let (assign8980_e8724, assign8980_e8724_d_n3, assign8980_e8724_d_n4, assign8980_e8724_d_n5, assign8980_e8724_d_n6, assign8980_e8724_d_n7, assign8980_e8724_d_n8,) = {
    if (var_guard93 == 0.0) {
        let assign8980_e8721: f64 = (1.0 + var_t1);
        let assign8980_e8722: f64 = (assign8980_e8721).sqrt();
        (assign8980_e8722, (var_t1_dn3 / (2.0 * assign8980_e8722)), (var_t1_dn4 / (2.0 * assign8980_e8722)), (var_t1_dn5 / (2.0 * assign8980_e8722)), (var_t1_dn6 / (2.0 * assign8980_e8722)), (var_t1_dn7 / (2.0 * assign8980_e8722)), (var_t1_dn8 / (2.0 * assign8980_e8722)),)
    } else {
        (var_coth1, var_coth1_dn3, var_coth1_dn4, var_coth1_dn5, var_coth1_dn6, var_coth1_dn7, var_coth1_dn8,)
    }
};
        var_coth1 = assign8980_e8724;
        var_coth1_dn3 = assign8980_e8724_d_n3;
        var_coth1_dn4 = assign8980_e8724_d_n4;
        var_coth1_dn5 = assign8980_e8724_d_n5;
        var_coth1_dn6 = assign8980_e8724_d_n6;
        var_coth1_dn7 = assign8980_e8724_d_n7;
        var_coth1_dn8 = assign8980_e8724_d_n8;

        let (assign8990_e8733, assign8990_e8733_d_n3, assign8990_e8733_d_n4, assign8990_e8733_d_n5, assign8990_e8733_d_n6, assign8990_e8733_d_n7, assign8990_e8733_d_n8,) = {
    if (var_guard93 == 0.0) {
        let assign8990_e8729: f64 = (0.5 * var_coth1);
        let assign8990_e8731: f64 = (assign8990_e8729 / var_q);
        (assign8990_e8731, ((((0.5 * var_coth1_dn3) * var_q) - (assign8990_e8729 * var_q_dn3)) / (var_q * var_q)), ((((0.5 * var_coth1_dn4) * var_q) - (assign8990_e8729 * var_q_dn4)) / (var_q * var_q)), ((((0.5 * var_coth1_dn5) * var_q) - (assign8990_e8729 * var_q_dn5)) / (var_q * var_q)), ((((0.5 * var_coth1_dn6) * var_q) - (assign8990_e8729 * var_q_dn6)) / (var_q * var_q)), ((((0.5 * var_coth1_dn7) * var_q) - (assign8990_e8729 * var_q_dn7)) / (var_q * var_q)), ((((0.5 * var_coth1_dn8) * var_q) - (assign8990_e8729 * var_q_dn8)) / (var_q * var_q)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign8990_e8733;
        var_t0_dn3 = assign8990_e8733_d_n3;
        var_t0_dn4 = assign8990_e8733_d_n4;
        var_t0_dn5 = assign8990_e8733_d_n5;
        var_t0_dn6 = assign8990_e8733_d_n6;
        var_t0_dn7 = assign8990_e8733_d_n7;
        var_t0_dn8 = assign8990_e8733_d_n8;

        let (assign9000_e8743, assign9000_e8743_d_n3, assign9000_e8743_d_n4, assign9000_e8743_d_n5, assign9000_e8743_d_n6, assign9000_e8743_d_n7, assign9000_e8743_d_n8,) = {
    if (var_guard93 == 0.0) {
        let assign9000_e8737: f64 = (-0.25);
        let assign9000_e8739: f64 = (assign9000_e8737 * var_t1);
        let assign9000_e8741: f64 = (assign9000_e8739 + var_t0);
        (assign9000_e8741, ((assign9000_e8737 * var_t1_dn3) + var_t0_dn3), ((assign9000_e8737 * var_t1_dn4) + var_t0_dn4), ((assign9000_e8737 * var_t1_dn5) + var_t0_dn5), ((assign9000_e8737 * var_t1_dn6) + var_t0_dn6), ((assign9000_e8737 * var_t1_dn7) + var_t0_dn7), ((assign9000_e8737 * var_t1_dn8) + var_t0_dn8),)
    } else {
        (var_dqcothqdqsqrt, var_dqcothqdqsqrt_dn3, var_dqcothqdqsqrt_dn4, var_dqcothqdqsqrt_dn5, var_dqcothqdqsqrt_dn6, var_dqcothqdqsqrt_dn7, var_dqcothqdqsqrt_dn8,)
    }
};
        var_dqcothqdqsqrt = assign9000_e8743;
        var_dqcothqdqsqrt_dn3 = assign9000_e8743_d_n3;
        var_dqcothqdqsqrt_dn4 = assign9000_e8743_d_n4;
        var_dqcothqdqsqrt_dn5 = assign9000_e8743_d_n5;
        var_dqcothqdqsqrt_dn6 = assign9000_e8743_d_n6;
        var_dqcothqdqsqrt_dn7 = assign9000_e8743_d_n7;
        var_dqcothqdqsqrt_dn8 = assign9000_e8743_d_n8;

        let assign9010_e8746: f64 = (var_q * var_coth1);
        var_qcoth = assign9010_e8746;
        var_qcoth_dn3 = ((var_q_dn3 * var_coth1) + (var_q * var_coth1_dn3));
        var_qcoth_dn4 = ((var_q_dn4 * var_coth1) + (var_q * var_coth1_dn4));
        var_qcoth_dn5 = ((var_q_dn5 * var_coth1) + (var_q * var_coth1_dn5));
        var_qcoth_dn6 = ((var_q_dn6 * var_coth1) + (var_q * var_coth1_dn6));
        var_qcoth_dn7 = ((var_q_dn7 * var_coth1) + (var_q * var_coth1_dn7));
        var_qcoth_dn8 = ((var_q_dn8 * var_coth1) + (var_q * var_coth1_dn8));

        let assign9020_e8749: f64 = (var_auxb1 + var_qcoth);
        var_t2 = assign9020_e8749;
        var_t2_dn3 = (var_auxb1_dn3 + var_qcoth_dn3);
        var_t2_dn4 = (var_auxb1_dn4 + var_qcoth_dn4);
        var_t2_dn5 = (var_auxb1_dn5 + var_qcoth_dn5);
        var_t2_dn6 = (var_auxb1_dn6 + var_qcoth_dn6);
        var_t2_dn7 = (var_auxb1_dn7 + var_qcoth_dn7);
        var_t2_dn8 = (var_auxb1_dn8 + var_qcoth_dn8);

        let assign9030_e8752: f64 = (1.0 / var_t2);
        var_t3 = assign9030_e8752;
        var_t3_dn3 = (-(var_t2_dn3 / (var_t2 * var_t2)));
        var_t3_dn4 = (-(var_t2_dn4 / (var_t2 * var_t2)));
        var_t3_dn5 = (-(var_t2_dn5 / (var_t2 * var_t2)));
        var_t3_dn6 = (-(var_t2_dn6 / (var_t2 * var_t2)));
        var_t3_dn7 = (-(var_t2_dn7 / (var_t2 * var_t2)));
        var_t3_dn8 = (-(var_t2_dn8 / (var_t2 * var_t2)));

        let assign9040_e8755: f64 = (var_xg2 - var_xg1);
        let assign9040_e8757: f64 = (assign9040_e8755 + var_q1);
        let assign9040_e8760: f64 = (var_qsqrt * var_t1);
        let assign9040_e8762: f64 = (assign9040_e8760 * var_t3);
        let assign9040_e8764: f64 = (assign9040_e8762 * var_t3);
        let assign9040_e8765: f64 = (assign9040_e8764).abs();
        let assign9040_e8766: f64 = (assign9040_e8765).ln();
        let assign9040_e8767: f64 = (assign9040_e8757 - assign9040_e8766);
        var_q2 = assign9040_e8767;
        var_q2_dn3 = (((var_xg2_dn3 - var_xg1_dn3) + var_q1_dn3) - (if assign9040_e8764 >= 0.0 { ((((((var_qsqrt_dn3 * var_t1) + (var_qsqrt * var_t1_dn3)) * var_t3) + (assign9040_e8760 * var_t3_dn3)) * var_t3) + (assign9040_e8762 * var_t3_dn3)) } else { (-((((((var_qsqrt_dn3 * var_t1) + (var_qsqrt * var_t1_dn3)) * var_t3) + (assign9040_e8760 * var_t3_dn3)) * var_t3) + (assign9040_e8762 * var_t3_dn3))) } / assign9040_e8765));
        var_q2_dn4 = (((var_xg2_dn4 - var_xg1_dn4) + var_q1_dn4) - (if assign9040_e8764 >= 0.0 { ((((((var_qsqrt_dn4 * var_t1) + (var_qsqrt * var_t1_dn4)) * var_t3) + (assign9040_e8760 * var_t3_dn4)) * var_t3) + (assign9040_e8762 * var_t3_dn4)) } else { (-((((((var_qsqrt_dn4 * var_t1) + (var_qsqrt * var_t1_dn4)) * var_t3) + (assign9040_e8760 * var_t3_dn4)) * var_t3) + (assign9040_e8762 * var_t3_dn4))) } / assign9040_e8765));
        var_q2_dn5 = (((var_xg2_dn5 - var_xg1_dn5) + var_q1_dn5) - (if assign9040_e8764 >= 0.0 { ((((((var_qsqrt_dn5 * var_t1) + (var_qsqrt * var_t1_dn5)) * var_t3) + (assign9040_e8760 * var_t3_dn5)) * var_t3) + (assign9040_e8762 * var_t3_dn5)) } else { (-((((((var_qsqrt_dn5 * var_t1) + (var_qsqrt * var_t1_dn5)) * var_t3) + (assign9040_e8760 * var_t3_dn5)) * var_t3) + (assign9040_e8762 * var_t3_dn5))) } / assign9040_e8765));
        var_q2_dn6 = (((var_xg2_dn6 - var_xg1_dn6) + var_q1_dn6) - (if assign9040_e8764 >= 0.0 { ((((((var_qsqrt_dn6 * var_t1) + (var_qsqrt * var_t1_dn6)) * var_t3) + (assign9040_e8760 * var_t3_dn6)) * var_t3) + (assign9040_e8762 * var_t3_dn6)) } else { (-((((((var_qsqrt_dn6 * var_t1) + (var_qsqrt * var_t1_dn6)) * var_t3) + (assign9040_e8760 * var_t3_dn6)) * var_t3) + (assign9040_e8762 * var_t3_dn6))) } / assign9040_e8765));
        var_q2_dn7 = (((var_xg2_dn7 - var_xg1_dn7) + var_q1_dn7) - (if assign9040_e8764 >= 0.0 { ((((((var_qsqrt_dn7 * var_t1) + (var_qsqrt * var_t1_dn7)) * var_t3) + (assign9040_e8760 * var_t3_dn7)) * var_t3) + (assign9040_e8762 * var_t3_dn7)) } else { (-((((((var_qsqrt_dn7 * var_t1) + (var_qsqrt * var_t1_dn7)) * var_t3) + (assign9040_e8760 * var_t3_dn7)) * var_t3) + (assign9040_e8762 * var_t3_dn7))) } / assign9040_e8765));
        var_q2_dn8 = (((var_xg2_dn8 - var_xg1_dn8) + var_q1_dn8) - (if assign9040_e8764 >= 0.0 { ((((((var_qsqrt_dn8 * var_t1) + (var_qsqrt * var_t1_dn8)) * var_t3) + (assign9040_e8760 * var_t3_dn8)) * var_t3) + (assign9040_e8762 * var_t3_dn8)) } else { (-((((((var_qsqrt_dn8 * var_t1) + (var_qsqrt * var_t1_dn8)) * var_t3) + (assign9040_e8760 * var_t3_dn8)) * var_t3) + (assign9040_e8762 * var_t3_dn8))) } / assign9040_e8765));

        let assign9050_e8771: f64 = (var_auxb1 + var_qcoth);
        let assign9050_e8774: f64 = (var_k2 * var_q2);
        let assign9050_e8776: f64 = (assign9050_e8774 + var_auxb1);
        let assign9050_e8777: f64 = (assign9050_e8771 * assign9050_e8776);
        let assign9050_e8778: f64 = (var_aaux + assign9050_e8777);
        var_f = assign9050_e8778;
        var_f_dn3 = (var_aaux_dn3 + (((var_auxb1_dn3 + var_qcoth_dn3) * assign9050_e8776) + (assign9050_e8771 * ((var_k2 * var_q2_dn3) + var_auxb1_dn3))));
        var_f_dn4 = (var_aaux_dn4 + (((var_auxb1_dn4 + var_qcoth_dn4) * assign9050_e8776) + (assign9050_e8771 * ((var_k2 * var_q2_dn4) + var_auxb1_dn4))));
        var_f_dn5 = (var_aaux_dn5 + (((var_auxb1_dn5 + var_qcoth_dn5) * assign9050_e8776) + (assign9050_e8771 * ((var_k2 * var_q2_dn5) + var_auxb1_dn5))));
        var_f_dn6 = (var_aaux_dn6 + (((var_auxb1_dn6 + var_qcoth_dn6) * assign9050_e8776) + (assign9050_e8771 * ((var_k2 * var_q2_dn6) + var_auxb1_dn6))));
        var_f_dn7 = (var_aaux_dn7 + (((var_auxb1_dn7 + var_qcoth_dn7) * assign9050_e8776) + (assign9050_e8771 * ((var_k2 * var_q2_dn7) + var_auxb1_dn7))));
        var_f_dn8 = (var_aaux_dn8 + (((var_auxb1_dn8 + var_qcoth_dn8) * assign9050_e8776) + (assign9050_e8771 * ((var_k2 * var_q2_dn8) + var_auxb1_dn8))));

        let assign9060_e8781: f64 = (1.0 / var_qsqrt);
        let assign9060_e8783: f64 = (assign9060_e8781 - var_t0);
        var_dlogsinhqsqdqsqrt = assign9060_e8783;
        var_dlogsinhqsqdqsqrt_dn3 = ((-(var_qsqrt_dn3 / (var_qsqrt * var_qsqrt))) - var_t0_dn3);
        var_dlogsinhqsqdqsqrt_dn4 = ((-(var_qsqrt_dn4 / (var_qsqrt * var_qsqrt))) - var_t0_dn4);
        var_dlogsinhqsqdqsqrt_dn5 = ((-(var_qsqrt_dn5 / (var_qsqrt * var_qsqrt))) - var_t0_dn5);
        var_dlogsinhqsqdqsqrt_dn6 = ((-(var_qsqrt_dn6 / (var_qsqrt * var_qsqrt))) - var_t0_dn6);
        var_dlogsinhqsqdqsqrt_dn7 = ((-(var_qsqrt_dn7 / (var_qsqrt * var_qsqrt))) - var_t0_dn7);
        var_dlogsinhqsqdqsqrt_dn8 = ((-(var_qsqrt_dn8 / (var_qsqrt * var_qsqrt))) - var_t0_dn8);

        let assign9070_e8785: f64 = (-2.0);
        let assign9070_e8787: f64 = (assign9070_e8785 * var_k1);
        let assign9070_e8789: f64 = (assign9070_e8787 * var_auxb1);
        let assign9070_e8791: f64 = (assign9070_e8789 + var_aaux);
        var_dqsqrt = assign9070_e8791;
        var_dqsqrt_dn3 = ((assign9070_e8787 * var_auxb1_dn3) + var_aaux_dn3);
        var_dqsqrt_dn4 = ((assign9070_e8787 * var_auxb1_dn4) + var_aaux_dn4);
        var_dqsqrt_dn5 = ((assign9070_e8787 * var_auxb1_dn5) + var_aaux_dn5);
        var_dqsqrt_dn6 = ((assign9070_e8787 * var_auxb1_dn6) + var_aaux_dn6);
        var_dqsqrt_dn7 = ((assign9070_e8787 * var_auxb1_dn7) + var_aaux_dn7);
        var_dqsqrt_dn8 = ((assign9070_e8787 * var_auxb1_dn8) + var_aaux_dn8);

        let assign9080_e8794: f64 = (var_dqcothqdqsqrt * var_dqsqrt);
        var_dqcoth = assign9080_e8794;
        var_dqcoth_dn3 = ((var_dqcothqdqsqrt_dn3 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn3));
        var_dqcoth_dn4 = ((var_dqcothqdqsqrt_dn4 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn4));
        var_dqcoth_dn5 = ((var_dqcothqdqsqrt_dn5 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn5));
        var_dqcoth_dn6 = ((var_dqcothqdqsqrt_dn6 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn6));
        var_dqcoth_dn7 = ((var_dqcothqdqsqrt_dn7 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn7));
        var_dqcoth_dn8 = ((var_dqcothqdqsqrt_dn8 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn8));

        let assign9090_e8796: f64 = (-1.0);
        let assign9090_e8799: f64 = (-var_k1);
        let assign9090_e8801: f64 = (assign9090_e8799 + var_dqcoth);
        let assign9090_e8803: f64 = (assign9090_e8801 * var_t3);
        let assign9090_e8804: f64 = (2.0 * assign9090_e8803);
        let assign9090_e8805: f64 = (assign9090_e8796 + assign9090_e8804);
        let assign9090_e8808: f64 = (var_dlogsinhqsqdqsqrt * var_dqsqrt);
        let assign9090_e8809: f64 = (assign9090_e8805 - assign9090_e8808);
        var_dq2 = assign9090_e8809;
        var_dq2_dn3 = ((2.0 * ((var_dqcoth_dn3 * var_t3) + (assign9090_e8801 * var_t3_dn3))) - ((var_dlogsinhqsqdqsqrt_dn3 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn3)));
        var_dq2_dn4 = ((2.0 * ((var_dqcoth_dn4 * var_t3) + (assign9090_e8801 * var_t3_dn4))) - ((var_dlogsinhqsqdqsqrt_dn4 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn4)));
        var_dq2_dn5 = ((2.0 * ((var_dqcoth_dn5 * var_t3) + (assign9090_e8801 * var_t3_dn5))) - ((var_dlogsinhqsqdqsqrt_dn5 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn5)));
        var_dq2_dn6 = ((2.0 * ((var_dqcoth_dn6 * var_t3) + (assign9090_e8801 * var_t3_dn6))) - ((var_dlogsinhqsqdqsqrt_dn6 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn6)));
        var_dq2_dn7 = ((2.0 * ((var_dqcoth_dn7 * var_t3) + (assign9090_e8801 * var_t3_dn7))) - ((var_dlogsinhqsqdqsqrt_dn7 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn7)));
        var_dq2_dn8 = ((2.0 * ((var_dqcoth_dn8 * var_t3) + (assign9090_e8801 * var_t3_dn8))) - ((var_dlogsinhqsqdqsqrt_dn8 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn8)));

        let assign9100_e8814: f64 = (var_auxb1 + var_t2);
        let assign9100_e8815: f64 = (var_k1 * assign9100_e8814);
        let assign9100_e8816: f64 = (var_aaux - assign9100_e8815);
        let assign9100_e8819: f64 = (var_auxb1 * var_dqcoth);
        let assign9100_e8820: f64 = (assign9100_e8816 + assign9100_e8819);
        let assign9100_e8824: f64 = (var_dq2 * var_t2);
        let assign9100_e8828: f64 = (var_dqcoth - var_k1);
        let assign9100_e8829: f64 = (var_q2 * assign9100_e8828);
        let assign9100_e8830: f64 = (assign9100_e8824 + assign9100_e8829);
        let assign9100_e8831: f64 = (var_k2 * assign9100_e8830);
        let assign9100_e8832: f64 = (assign9100_e8820 + assign9100_e8831);
        var_df = assign9100_e8832;
        var_df_dn3 = (((var_aaux_dn3 - (var_k1 * (var_auxb1_dn3 + var_t2_dn3))) + ((var_auxb1_dn3 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn3))) + (var_k2 * (((var_dq2_dn3 * var_t2) + (var_dq2 * var_t2_dn3)) + ((var_q2_dn3 * assign9100_e8828) + (var_q2 * var_dqcoth_dn3)))));
        var_df_dn4 = (((var_aaux_dn4 - (var_k1 * (var_auxb1_dn4 + var_t2_dn4))) + ((var_auxb1_dn4 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn4))) + (var_k2 * (((var_dq2_dn4 * var_t2) + (var_dq2 * var_t2_dn4)) + ((var_q2_dn4 * assign9100_e8828) + (var_q2 * var_dqcoth_dn4)))));
        var_df_dn5 = (((var_aaux_dn5 - (var_k1 * (var_auxb1_dn5 + var_t2_dn5))) + ((var_auxb1_dn5 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn5))) + (var_k2 * (((var_dq2_dn5 * var_t2) + (var_dq2 * var_t2_dn5)) + ((var_q2_dn5 * assign9100_e8828) + (var_q2 * var_dqcoth_dn5)))));
        var_df_dn6 = (((var_aaux_dn6 - (var_k1 * (var_auxb1_dn6 + var_t2_dn6))) + ((var_auxb1_dn6 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn6))) + (var_k2 * (((var_dq2_dn6 * var_t2) + (var_dq2 * var_t2_dn6)) + ((var_q2_dn6 * assign9100_e8828) + (var_q2 * var_dqcoth_dn6)))));
        var_df_dn7 = (((var_aaux_dn7 - (var_k1 * (var_auxb1_dn7 + var_t2_dn7))) + ((var_auxb1_dn7 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn7))) + (var_k2 * (((var_dq2_dn7 * var_t2) + (var_dq2 * var_t2_dn7)) + ((var_q2_dn7 * assign9100_e8828) + (var_q2 * var_dqcoth_dn7)))));
        var_df_dn8 = (((var_aaux_dn8 - (var_k1 * (var_auxb1_dn8 + var_t2_dn8))) + ((var_auxb1_dn8 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn8))) + (var_k2 * (((var_dq2_dn8 * var_t2) + (var_dq2 * var_t2_dn8)) + ((var_q2_dn8 * assign9100_e8828) + (var_q2 * var_dqcoth_dn8)))));

        let assign9110_e8834: f64 = (-var_f);
        let assign9110_e8836: f64 = (assign9110_e8834 / var_df);
        var_delta = assign9110_e8836;
        var_delta_dn3 = ((((-var_f_dn3) * var_df) - (assign9110_e8834 * var_df_dn3)) / (var_df * var_df));
        var_delta_dn4 = ((((-var_f_dn4) * var_df) - (assign9110_e8834 * var_df_dn4)) / (var_df * var_df));
        var_delta_dn5 = ((((-var_f_dn5) * var_df) - (assign9110_e8834 * var_df_dn5)) / (var_df * var_df));
        var_delta_dn6 = ((((-var_f_dn6) * var_df) - (assign9110_e8834 * var_df_dn6)) / (var_df * var_df));
        var_delta_dn7 = ((((-var_f_dn7) * var_df) - (assign9110_e8834 * var_df_dn7)) / (var_df * var_df));
        var_delta_dn8 = ((((-var_f_dn8) * var_df) - (assign9110_e8834 * var_df_dn8)) / (var_df * var_df));

        let assign9120_e8839: f64 = (var_phi1 + var_delta);
        var_phi1 = assign9120_e8839;
        var_phi1_dn3 = (var_phi1_dn3 + var_delta_dn3);
        var_phi1_dn4 = (var_phi1_dn4 + var_delta_dn4);
        var_phi1_dn5 = (var_phi1_dn5 + var_delta_dn5);
        var_phi1_dn6 = (var_phi1_dn6 + var_delta_dn6);
        var_phi1_dn7 = (var_phi1_dn7 + var_delta_dn7);
        var_phi1_dn8 = (var_phi1_dn8 + var_delta_dn8);

        let assign9130_e8842: f64 = (var_xg1 - var_phi1);
        var_q1 = assign9130_e8842;
        var_q1_dn3 = (var_xg1_dn3 - var_phi1_dn3);
        var_q1_dn4 = (var_xg1_dn4 - var_phi1_dn4);
        var_q1_dn5 = (var_xg1_dn5 - var_phi1_dn5);
        var_q1_dn6 = (var_xg1_dn6 - var_phi1_dn6);
        var_q1_dn7 = (var_xg1_dn7 - var_phi1_dn7);
        var_q1_dn8 = (var_xg1_dn8 - var_phi1_dn8);

        let assign9140_e8845: f64 = (var_k1 * var_q1);
        var_auxb1 = assign9140_e8845;
        var_auxb1_dn3 = (var_k1 * var_q1_dn3);
        var_auxb1_dn4 = (var_k1 * var_q1_dn4);
        var_auxb1_dn5 = (var_k1 * var_q1_dn5);
        var_auxb1_dn6 = (var_k1 * var_q1_dn6);
        var_auxb1_dn7 = (var_k1 * var_q1_dn7);
        var_auxb1_dn8 = (var_k1 * var_q1_dn8);

        let assign9150_e8847: f64 = (-var_a0);
        let assign9150_e8849: f64 = (var_phi1).exp();
        let assign9150_e8850: f64 = (assign9150_e8847 * assign9150_e8849);
        var_aaux = assign9150_e8850;
        var_aaux_dn3 = (((-var_a0_dn3) * assign9150_e8849) + (assign9150_e8847 * (assign9150_e8849 * var_phi1_dn3)));
        var_aaux_dn4 = (((-var_a0_dn4) * assign9150_e8849) + (assign9150_e8847 * (assign9150_e8849 * var_phi1_dn4)));
        var_aaux_dn5 = (((-var_a0_dn5) * assign9150_e8849) + (assign9150_e8847 * (assign9150_e8849 * var_phi1_dn5)));
        var_aaux_dn6 = (((-var_a0_dn6) * assign9150_e8849) + (assign9150_e8847 * (assign9150_e8849 * var_phi1_dn6)));
        var_aaux_dn7 = (((-var_a0_dn7) * assign9150_e8849) + (assign9150_e8847 * (assign9150_e8849 * var_phi1_dn7)));
        var_aaux_dn8 = (((-var_a0_dn8) * assign9150_e8849) + (assign9150_e8847 * (assign9150_e8849 * var_phi1_dn8)));

        let assign9160_e8853: f64 = (var_auxb1 * var_auxb1);
        let assign9160_e8855: f64 = (assign9160_e8853 + var_aaux);
        var_qsqrt = assign9160_e8855;
        var_qsqrt_dn3 = (((var_auxb1_dn3 * var_auxb1) + (var_auxb1 * var_auxb1_dn3)) + var_aaux_dn3);
        var_qsqrt_dn4 = (((var_auxb1_dn4 * var_auxb1) + (var_auxb1 * var_auxb1_dn4)) + var_aaux_dn4);
        var_qsqrt_dn5 = (((var_auxb1_dn5 * var_auxb1) + (var_auxb1 * var_auxb1_dn5)) + var_aaux_dn5);
        var_qsqrt_dn6 = (((var_auxb1_dn6 * var_auxb1) + (var_auxb1 * var_auxb1_dn6)) + var_aaux_dn6);
        var_qsqrt_dn7 = (((var_auxb1_dn7 * var_auxb1) + (var_auxb1 * var_auxb1_dn7)) + var_aaux_dn7);
        var_qsqrt_dn8 = (((var_auxb1_dn8 * var_auxb1) + (var_auxb1 * var_auxb1_dn8)) + var_aaux_dn8);

        let assign9170_e8858: f64 = if var_qsqrt < 0.0 { 1.0 } else { 0.0 };
        var_guard94 = assign9170_e8858;

        let (assign9180_e8864, assign9180_e8864_d_n3, assign9180_e8864_d_n4, assign9180_e8864_d_n5, assign9180_e8864_d_n6, assign9180_e8864_d_n7, assign9180_e8864_d_n8,) = {
    if (var_guard94 != 0.0) {
        let assign9180_e8861: f64 = (-var_qsqrt);
        let assign9180_e8862: f64 = (assign9180_e8861).sqrt();
        (assign9180_e8862, ((-var_qsqrt_dn3) / (2.0 * assign9180_e8862)), ((-var_qsqrt_dn4) / (2.0 * assign9180_e8862)), ((-var_qsqrt_dn5) / (2.0 * assign9180_e8862)), ((-var_qsqrt_dn6) / (2.0 * assign9180_e8862)), ((-var_qsqrt_dn7) / (2.0 * assign9180_e8862)), ((-var_qsqrt_dn8) / (2.0 * assign9180_e8862)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign9180_e8864;
        var_q_dn3 = assign9180_e8864_d_n3;
        var_q_dn4 = assign9180_e8864_d_n4;
        var_q_dn5 = assign9180_e8864_d_n5;
        var_q_dn6 = assign9180_e8864_d_n6;
        var_q_dn7 = assign9180_e8864_d_n7;
        var_q_dn8 = assign9180_e8864_d_n8;

        let (assign9190_e8873, assign9190_e8873_d_n3, assign9190_e8873_d_n4, assign9190_e8873_d_n5, assign9190_e8873_d_n6, assign9190_e8873_d_n7, assign9190_e8873_d_n8,) = {
    if (var_guard94 != 0.0) {
        let assign9190_e8869: f64 = (0.5 * var_q);
        let assign9190_e8870: f64 = (assign9190_e8869).sin();
        let assign9190_e8871: f64 = (1.0 / assign9190_e8870);
        (assign9190_e8871, (-(((assign9190_e8869).cos() * (0.5 * var_q_dn3)) / (assign9190_e8870 * assign9190_e8870))), (-(((assign9190_e8869).cos() * (0.5 * var_q_dn4)) / (assign9190_e8870 * assign9190_e8870))), (-(((assign9190_e8869).cos() * (0.5 * var_q_dn5)) / (assign9190_e8870 * assign9190_e8870))), (-(((assign9190_e8869).cos() * (0.5 * var_q_dn6)) / (assign9190_e8870 * assign9190_e8870))), (-(((assign9190_e8869).cos() * (0.5 * var_q_dn7)) / (assign9190_e8870 * assign9190_e8870))), (-(((assign9190_e8869).cos() * (0.5 * var_q_dn8)) / (assign9190_e8870 * assign9190_e8870))),)
    } else {
        (var_csc1, var_csc1_dn3, var_csc1_dn4, var_csc1_dn5, var_csc1_dn6, var_csc1_dn7, var_csc1_dn8,)
    }
};
        var_csc1 = assign9190_e8873;
        var_csc1_dn3 = assign9190_e8873_d_n3;
        var_csc1_dn4 = assign9190_e8873_d_n4;
        var_csc1_dn5 = assign9190_e8873_d_n5;
        var_csc1_dn6 = assign9190_e8873_d_n6;
        var_csc1_dn7 = assign9190_e8873_d_n7;
        var_csc1_dn8 = assign9190_e8873_d_n8;

        let (assign9200_e8879, assign9200_e8879_d_n3, assign9200_e8879_d_n4, assign9200_e8879_d_n5, assign9200_e8879_d_n6, assign9200_e8879_d_n7, assign9200_e8879_d_n8,) = {
    if (var_guard94 != 0.0) {
        let assign9200_e8877: f64 = (var_csc1 * var_csc1);
        (assign9200_e8877, ((var_csc1_dn3 * var_csc1) + (var_csc1 * var_csc1_dn3)), ((var_csc1_dn4 * var_csc1) + (var_csc1 * var_csc1_dn4)), ((var_csc1_dn5 * var_csc1) + (var_csc1 * var_csc1_dn5)), ((var_csc1_dn6 * var_csc1) + (var_csc1 * var_csc1_dn6)), ((var_csc1_dn7 * var_csc1) + (var_csc1 * var_csc1_dn7)), ((var_csc1_dn8 * var_csc1) + (var_csc1 * var_csc1_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign9200_e8879;
        var_t1_dn3 = assign9200_e8879_d_n3;
        var_t1_dn4 = assign9200_e8879_d_n4;
        var_t1_dn5 = assign9200_e8879_d_n5;
        var_t1_dn6 = assign9200_e8879_d_n6;
        var_t1_dn7 = assign9200_e8879_d_n7;
        var_t1_dn8 = assign9200_e8879_d_n8;

        let (assign9210_e8888, assign9210_e8888_d_n3, assign9210_e8888_d_n4, assign9210_e8888_d_n5, assign9210_e8888_d_n6, assign9210_e8888_d_n7, assign9210_e8888_d_n8,) = {
    if (var_guard94 != 0.0) {
        let assign9210_e8883: f64 = (0.5 * var_q);
        let assign9210_e8884: f64 = (assign9210_e8883).cos();
        let assign9210_e8886: f64 = (assign9210_e8884 * var_csc1);
        (assign9210_e8886, (((-(assign9210_e8883).sin() * (0.5 * var_q_dn3)) * var_csc1) + (assign9210_e8884 * var_csc1_dn3)), (((-(assign9210_e8883).sin() * (0.5 * var_q_dn4)) * var_csc1) + (assign9210_e8884 * var_csc1_dn4)), (((-(assign9210_e8883).sin() * (0.5 * var_q_dn5)) * var_csc1) + (assign9210_e8884 * var_csc1_dn5)), (((-(assign9210_e8883).sin() * (0.5 * var_q_dn6)) * var_csc1) + (assign9210_e8884 * var_csc1_dn6)), (((-(assign9210_e8883).sin() * (0.5 * var_q_dn7)) * var_csc1) + (assign9210_e8884 * var_csc1_dn7)), (((-(assign9210_e8883).sin() * (0.5 * var_q_dn8)) * var_csc1) + (assign9210_e8884 * var_csc1_dn8)),)
    } else {
        (var_coth1, var_coth1_dn3, var_coth1_dn4, var_coth1_dn5, var_coth1_dn6, var_coth1_dn7, var_coth1_dn8,)
    }
};
        var_coth1 = assign9210_e8888;
        var_coth1_dn3 = assign9210_e8888_d_n3;
        var_coth1_dn4 = assign9210_e8888_d_n4;
        var_coth1_dn5 = assign9210_e8888_d_n5;
        var_coth1_dn6 = assign9210_e8888_d_n6;
        var_coth1_dn7 = assign9210_e8888_d_n7;
        var_coth1_dn8 = assign9210_e8888_d_n8;

        let (assign9220_e8897, assign9220_e8897_d_n3, assign9220_e8897_d_n4, assign9220_e8897_d_n5, assign9220_e8897_d_n6, assign9220_e8897_d_n7, assign9220_e8897_d_n8,) = {
    if (var_guard94 != 0.0) {
        let assign9220_e8891: f64 = (-0.5);
        let assign9220_e8893: f64 = (assign9220_e8891 * var_coth1);
        let assign9220_e8895: f64 = (assign9220_e8893 / var_q);
        (assign9220_e8895, ((((assign9220_e8891 * var_coth1_dn3) * var_q) - (assign9220_e8893 * var_q_dn3)) / (var_q * var_q)), ((((assign9220_e8891 * var_coth1_dn4) * var_q) - (assign9220_e8893 * var_q_dn4)) / (var_q * var_q)), ((((assign9220_e8891 * var_coth1_dn5) * var_q) - (assign9220_e8893 * var_q_dn5)) / (var_q * var_q)), ((((assign9220_e8891 * var_coth1_dn6) * var_q) - (assign9220_e8893 * var_q_dn6)) / (var_q * var_q)), ((((assign9220_e8891 * var_coth1_dn7) * var_q) - (assign9220_e8893 * var_q_dn7)) / (var_q * var_q)), ((((assign9220_e8891 * var_coth1_dn8) * var_q) - (assign9220_e8893 * var_q_dn8)) / (var_q * var_q)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign9220_e8897;
        var_t0_dn3 = assign9220_e8897_d_n3;
        var_t0_dn4 = assign9220_e8897_d_n4;
        var_t0_dn5 = assign9220_e8897_d_n5;
        var_t0_dn6 = assign9220_e8897_d_n6;
        var_t0_dn7 = assign9220_e8897_d_n7;
        var_t0_dn8 = assign9220_e8897_d_n8;

        *var_aaux_slot = var_aaux;
        *var_aaux_dn3_slot = var_aaux_dn3;
        *var_aaux_dn4_slot = var_aaux_dn4;
        *var_aaux_dn5_slot = var_aaux_dn5;
        *var_aaux_dn6_slot = var_aaux_dn6;
        *var_aaux_dn7_slot = var_aaux_dn7;
        *var_aaux_dn8_slot = var_aaux_dn8;
        *var_auxb1_slot = var_auxb1;
        *var_auxb1_dn3_slot = var_auxb1_dn3;
        *var_auxb1_dn4_slot = var_auxb1_dn4;
        *var_auxb1_dn5_slot = var_auxb1_dn5;
        *var_auxb1_dn6_slot = var_auxb1_dn6;
        *var_auxb1_dn7_slot = var_auxb1_dn7;
        *var_auxb1_dn8_slot = var_auxb1_dn8;
        *var_coth1_slot = var_coth1;
        *var_coth1_dn3_slot = var_coth1_dn3;
        *var_coth1_dn4_slot = var_coth1_dn4;
        *var_coth1_dn5_slot = var_coth1_dn5;
        *var_coth1_dn6_slot = var_coth1_dn6;
        *var_coth1_dn7_slot = var_coth1_dn7;
        *var_coth1_dn8_slot = var_coth1_dn8;
        *var_csc1_slot = var_csc1;
        *var_csc1_dn3_slot = var_csc1_dn3;
        *var_csc1_dn4_slot = var_csc1_dn4;
        *var_csc1_dn5_slot = var_csc1_dn5;
        *var_csc1_dn6_slot = var_csc1_dn6;
        *var_csc1_dn7_slot = var_csc1_dn7;
        *var_csc1_dn8_slot = var_csc1_dn8;
        *var_delta_slot = var_delta;
        *var_delta_dn3_slot = var_delta_dn3;
        *var_delta_dn4_slot = var_delta_dn4;
        *var_delta_dn5_slot = var_delta_dn5;
        *var_delta_dn6_slot = var_delta_dn6;
        *var_delta_dn7_slot = var_delta_dn7;
        *var_delta_dn8_slot = var_delta_dn8;
        *var_df_slot = var_df;
        *var_df_dn3_slot = var_df_dn3;
        *var_df_dn4_slot = var_df_dn4;
        *var_df_dn5_slot = var_df_dn5;
        *var_df_dn6_slot = var_df_dn6;
        *var_df_dn7_slot = var_df_dn7;
        *var_df_dn8_slot = var_df_dn8;
        *var_dlogsinhqsqdqsqrt_slot = var_dlogsinhqsqdqsqrt;
        *var_dlogsinhqsqdqsqrt_dn3_slot = var_dlogsinhqsqdqsqrt_dn3;
        *var_dlogsinhqsqdqsqrt_dn4_slot = var_dlogsinhqsqdqsqrt_dn4;
        *var_dlogsinhqsqdqsqrt_dn5_slot = var_dlogsinhqsqdqsqrt_dn5;
        *var_dlogsinhqsqdqsqrt_dn6_slot = var_dlogsinhqsqdqsqrt_dn6;
        *var_dlogsinhqsqdqsqrt_dn7_slot = var_dlogsinhqsqdqsqrt_dn7;
        *var_dlogsinhqsqdqsqrt_dn8_slot = var_dlogsinhqsqdqsqrt_dn8;
        *var_dq2_slot = var_dq2;
        *var_dq2_dn3_slot = var_dq2_dn3;
        *var_dq2_dn4_slot = var_dq2_dn4;
        *var_dq2_dn5_slot = var_dq2_dn5;
        *var_dq2_dn6_slot = var_dq2_dn6;
        *var_dq2_dn7_slot = var_dq2_dn7;
        *var_dq2_dn8_slot = var_dq2_dn8;
        *var_dqcoth_slot = var_dqcoth;
        *var_dqcoth_dn3_slot = var_dqcoth_dn3;
        *var_dqcoth_dn4_slot = var_dqcoth_dn4;
        *var_dqcoth_dn5_slot = var_dqcoth_dn5;
        *var_dqcoth_dn6_slot = var_dqcoth_dn6;
        *var_dqcoth_dn7_slot = var_dqcoth_dn7;
        *var_dqcoth_dn8_slot = var_dqcoth_dn8;
        *var_dqcothqdqsqrt_slot = var_dqcothqdqsqrt;
        *var_dqcothqdqsqrt_dn3_slot = var_dqcothqdqsqrt_dn3;
        *var_dqcothqdqsqrt_dn4_slot = var_dqcothqdqsqrt_dn4;
        *var_dqcothqdqsqrt_dn5_slot = var_dqcothqdqsqrt_dn5;
        *var_dqcothqdqsqrt_dn6_slot = var_dqcothqdqsqrt_dn6;
        *var_dqcothqdqsqrt_dn7_slot = var_dqcothqdqsqrt_dn7;
        *var_dqcothqdqsqrt_dn8_slot = var_dqcothqdqsqrt_dn8;
        *var_dqsqrt_slot = var_dqsqrt;
        *var_dqsqrt_dn3_slot = var_dqsqrt_dn3;
        *var_dqsqrt_dn4_slot = var_dqsqrt_dn4;
        *var_dqsqrt_dn5_slot = var_dqsqrt_dn5;
        *var_dqsqrt_dn6_slot = var_dqsqrt_dn6;
        *var_dqsqrt_dn7_slot = var_dqsqrt_dn7;
        *var_dqsqrt_dn8_slot = var_dqsqrt_dn8;
        *var_f_slot = var_f;
        *var_f_dn3_slot = var_f_dn3;
        *var_f_dn4_slot = var_f_dn4;
        *var_f_dn5_slot = var_f_dn5;
        *var_f_dn6_slot = var_f_dn6;
        *var_f_dn7_slot = var_f_dn7;
        *var_f_dn8_slot = var_f_dn8;
        *var_guard93_slot = var_guard93;
        *var_guard94_slot = var_guard94;
        *var_phi1_slot = var_phi1;
        *var_phi1_dn3_slot = var_phi1_dn3;
        *var_phi1_dn4_slot = var_phi1_dn4;
        *var_phi1_dn5_slot = var_phi1_dn5;
        *var_phi1_dn6_slot = var_phi1_dn6;
        *var_phi1_dn7_slot = var_phi1_dn7;
        *var_phi1_dn8_slot = var_phi1_dn8;
        *var_q_slot = var_q;
        *var_q1_slot = var_q1;
        *var_q1_dn3_slot = var_q1_dn3;
        *var_q1_dn4_slot = var_q1_dn4;
        *var_q1_dn5_slot = var_q1_dn5;
        *var_q1_dn6_slot = var_q1_dn6;
        *var_q1_dn7_slot = var_q1_dn7;
        *var_q1_dn8_slot = var_q1_dn8;
        *var_q2_slot = var_q2;
        *var_q2_dn3_slot = var_q2_dn3;
        *var_q2_dn4_slot = var_q2_dn4;
        *var_q2_dn5_slot = var_q2_dn5;
        *var_q2_dn6_slot = var_q2_dn6;
        *var_q2_dn7_slot = var_q2_dn7;
        *var_q2_dn8_slot = var_q2_dn8;
        *var_q_dn3_slot = var_q_dn3;
        *var_q_dn4_slot = var_q_dn4;
        *var_q_dn5_slot = var_q_dn5;
        *var_q_dn6_slot = var_q_dn6;
        *var_q_dn7_slot = var_q_dn7;
        *var_q_dn8_slot = var_q_dn8;
        *var_qcoth_slot = var_qcoth;
        *var_qcoth_dn3_slot = var_qcoth_dn3;
        *var_qcoth_dn4_slot = var_qcoth_dn4;
        *var_qcoth_dn5_slot = var_qcoth_dn5;
        *var_qcoth_dn6_slot = var_qcoth_dn6;
        *var_qcoth_dn7_slot = var_qcoth_dn7;
        *var_qcoth_dn8_slot = var_qcoth_dn8;
        *var_qsqrt_slot = var_qsqrt;
        *var_qsqrt_dn3_slot = var_qsqrt_dn3;
        *var_qsqrt_dn4_slot = var_qsqrt_dn4;
        *var_qsqrt_dn5_slot = var_qsqrt_dn5;
        *var_qsqrt_dn6_slot = var_qsqrt_dn6;
        *var_qsqrt_dn7_slot = var_qsqrt_dn7;
        *var_qsqrt_dn8_slot = var_qsqrt_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
    }

    pub(super) fn stamp_transient_block_19(
        var_a0: f64,
        var_a0_dn3: f64,
        var_a0_dn4: f64,
        var_a0_dn5: f64,
        var_a0_dn6: f64,
        var_a0_dn7: f64,
        var_a0_dn8: f64,
        var_guard94: f64,
        var_k1: f64,
        var_k2: f64,
        var_xg1: f64,
        var_xg1_dn3: f64,
        var_xg1_dn4: f64,
        var_xg1_dn5: f64,
        var_xg1_dn6: f64,
        var_xg1_dn7: f64,
        var_xg1_dn8: f64,
        var_xg2: f64,
        var_xg2_dn3: f64,
        var_xg2_dn4: f64,
        var_xg2_dn5: f64,
        var_xg2_dn6: f64,
        var_xg2_dn7: f64,
        var_xg2_dn8: f64,
        var_aaux_slot: &mut f64,
        var_aaux_dn3_slot: &mut f64,
        var_aaux_dn4_slot: &mut f64,
        var_aaux_dn5_slot: &mut f64,
        var_aaux_dn6_slot: &mut f64,
        var_aaux_dn7_slot: &mut f64,
        var_aaux_dn8_slot: &mut f64,
        var_auxb1_slot: &mut f64,
        var_auxb1_dn3_slot: &mut f64,
        var_auxb1_dn4_slot: &mut f64,
        var_auxb1_dn5_slot: &mut f64,
        var_auxb1_dn6_slot: &mut f64,
        var_auxb1_dn7_slot: &mut f64,
        var_auxb1_dn8_slot: &mut f64,
        var_coth1_slot: &mut f64,
        var_coth1_dn3_slot: &mut f64,
        var_coth1_dn4_slot: &mut f64,
        var_coth1_dn5_slot: &mut f64,
        var_coth1_dn6_slot: &mut f64,
        var_coth1_dn7_slot: &mut f64,
        var_coth1_dn8_slot: &mut f64,
        var_csc1_slot: &mut f64,
        var_csc1_dn3_slot: &mut f64,
        var_csc1_dn4_slot: &mut f64,
        var_csc1_dn5_slot: &mut f64,
        var_csc1_dn6_slot: &mut f64,
        var_csc1_dn7_slot: &mut f64,
        var_csc1_dn8_slot: &mut f64,
        var_delta_slot: &mut f64,
        var_delta_dn3_slot: &mut f64,
        var_delta_dn4_slot: &mut f64,
        var_delta_dn5_slot: &mut f64,
        var_delta_dn6_slot: &mut f64,
        var_delta_dn7_slot: &mut f64,
        var_delta_dn8_slot: &mut f64,
        var_df_slot: &mut f64,
        var_df_dn3_slot: &mut f64,
        var_df_dn4_slot: &mut f64,
        var_df_dn5_slot: &mut f64,
        var_df_dn6_slot: &mut f64,
        var_df_dn7_slot: &mut f64,
        var_df_dn8_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn3_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn4_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn5_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn6_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn7_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn8_slot: &mut f64,
        var_dq2_slot: &mut f64,
        var_dq2_dn3_slot: &mut f64,
        var_dq2_dn4_slot: &mut f64,
        var_dq2_dn5_slot: &mut f64,
        var_dq2_dn6_slot: &mut f64,
        var_dq2_dn7_slot: &mut f64,
        var_dq2_dn8_slot: &mut f64,
        var_dqcoth_slot: &mut f64,
        var_dqcoth_dn3_slot: &mut f64,
        var_dqcoth_dn4_slot: &mut f64,
        var_dqcoth_dn5_slot: &mut f64,
        var_dqcoth_dn6_slot: &mut f64,
        var_dqcoth_dn7_slot: &mut f64,
        var_dqcoth_dn8_slot: &mut f64,
        var_dqcothqdqsqrt_slot: &mut f64,
        var_dqcothqdqsqrt_dn3_slot: &mut f64,
        var_dqcothqdqsqrt_dn4_slot: &mut f64,
        var_dqcothqdqsqrt_dn5_slot: &mut f64,
        var_dqcothqdqsqrt_dn6_slot: &mut f64,
        var_dqcothqdqsqrt_dn7_slot: &mut f64,
        var_dqcothqdqsqrt_dn8_slot: &mut f64,
        var_dqsqrt_slot: &mut f64,
        var_dqsqrt_dn3_slot: &mut f64,
        var_dqsqrt_dn4_slot: &mut f64,
        var_dqsqrt_dn5_slot: &mut f64,
        var_dqsqrt_dn6_slot: &mut f64,
        var_dqsqrt_dn7_slot: &mut f64,
        var_dqsqrt_dn8_slot: &mut f64,
        var_f_slot: &mut f64,
        var_f_dn3_slot: &mut f64,
        var_f_dn4_slot: &mut f64,
        var_f_dn5_slot: &mut f64,
        var_f_dn6_slot: &mut f64,
        var_f_dn7_slot: &mut f64,
        var_f_dn8_slot: &mut f64,
        var_guard95_slot: &mut f64,
        var_phi1_slot: &mut f64,
        var_phi1_dn3_slot: &mut f64,
        var_phi1_dn4_slot: &mut f64,
        var_phi1_dn5_slot: &mut f64,
        var_phi1_dn6_slot: &mut f64,
        var_phi1_dn7_slot: &mut f64,
        var_phi1_dn8_slot: &mut f64,
        var_q_slot: &mut f64,
        var_q1_slot: &mut f64,
        var_q1_dn3_slot: &mut f64,
        var_q1_dn4_slot: &mut f64,
        var_q1_dn5_slot: &mut f64,
        var_q1_dn6_slot: &mut f64,
        var_q1_dn7_slot: &mut f64,
        var_q1_dn8_slot: &mut f64,
        var_q2_slot: &mut f64,
        var_q2_dn3_slot: &mut f64,
        var_q2_dn4_slot: &mut f64,
        var_q2_dn5_slot: &mut f64,
        var_q2_dn6_slot: &mut f64,
        var_q2_dn7_slot: &mut f64,
        var_q2_dn8_slot: &mut f64,
        var_q_dn3_slot: &mut f64,
        var_q_dn4_slot: &mut f64,
        var_q_dn5_slot: &mut f64,
        var_q_dn6_slot: &mut f64,
        var_q_dn7_slot: &mut f64,
        var_q_dn8_slot: &mut f64,
        var_qcoth_slot: &mut f64,
        var_qcoth_dn3_slot: &mut f64,
        var_qcoth_dn4_slot: &mut f64,
        var_qcoth_dn5_slot: &mut f64,
        var_qcoth_dn6_slot: &mut f64,
        var_qcoth_dn7_slot: &mut f64,
        var_qcoth_dn8_slot: &mut f64,
        var_qsqrt_slot: &mut f64,
        var_qsqrt_dn3_slot: &mut f64,
        var_qsqrt_dn4_slot: &mut f64,
        var_qsqrt_dn5_slot: &mut f64,
        var_qsqrt_dn6_slot: &mut f64,
        var_qsqrt_dn7_slot: &mut f64,
        var_qsqrt_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
    ) {
        let mut var_aaux: f64 = *var_aaux_slot;
        let mut var_aaux_dn3: f64 = *var_aaux_dn3_slot;
        let mut var_aaux_dn4: f64 = *var_aaux_dn4_slot;
        let mut var_aaux_dn5: f64 = *var_aaux_dn5_slot;
        let mut var_aaux_dn6: f64 = *var_aaux_dn6_slot;
        let mut var_aaux_dn7: f64 = *var_aaux_dn7_slot;
        let mut var_aaux_dn8: f64 = *var_aaux_dn8_slot;
        let mut var_auxb1: f64 = *var_auxb1_slot;
        let mut var_auxb1_dn3: f64 = *var_auxb1_dn3_slot;
        let mut var_auxb1_dn4: f64 = *var_auxb1_dn4_slot;
        let mut var_auxb1_dn5: f64 = *var_auxb1_dn5_slot;
        let mut var_auxb1_dn6: f64 = *var_auxb1_dn6_slot;
        let mut var_auxb1_dn7: f64 = *var_auxb1_dn7_slot;
        let mut var_auxb1_dn8: f64 = *var_auxb1_dn8_slot;
        let mut var_coth1: f64 = *var_coth1_slot;
        let mut var_coth1_dn3: f64 = *var_coth1_dn3_slot;
        let mut var_coth1_dn4: f64 = *var_coth1_dn4_slot;
        let mut var_coth1_dn5: f64 = *var_coth1_dn5_slot;
        let mut var_coth1_dn6: f64 = *var_coth1_dn6_slot;
        let mut var_coth1_dn7: f64 = *var_coth1_dn7_slot;
        let mut var_coth1_dn8: f64 = *var_coth1_dn8_slot;
        let mut var_csc1: f64 = *var_csc1_slot;
        let mut var_csc1_dn3: f64 = *var_csc1_dn3_slot;
        let mut var_csc1_dn4: f64 = *var_csc1_dn4_slot;
        let mut var_csc1_dn5: f64 = *var_csc1_dn5_slot;
        let mut var_csc1_dn6: f64 = *var_csc1_dn6_slot;
        let mut var_csc1_dn7: f64 = *var_csc1_dn7_slot;
        let mut var_csc1_dn8: f64 = *var_csc1_dn8_slot;
        let mut var_delta: f64 = *var_delta_slot;
        let mut var_delta_dn3: f64 = *var_delta_dn3_slot;
        let mut var_delta_dn4: f64 = *var_delta_dn4_slot;
        let mut var_delta_dn5: f64 = *var_delta_dn5_slot;
        let mut var_delta_dn6: f64 = *var_delta_dn6_slot;
        let mut var_delta_dn7: f64 = *var_delta_dn7_slot;
        let mut var_delta_dn8: f64 = *var_delta_dn8_slot;
        let mut var_df: f64 = *var_df_slot;
        let mut var_df_dn3: f64 = *var_df_dn3_slot;
        let mut var_df_dn4: f64 = *var_df_dn4_slot;
        let mut var_df_dn5: f64 = *var_df_dn5_slot;
        let mut var_df_dn6: f64 = *var_df_dn6_slot;
        let mut var_df_dn7: f64 = *var_df_dn7_slot;
        let mut var_df_dn8: f64 = *var_df_dn8_slot;
        let mut var_dlogsinhqsqdqsqrt: f64 = *var_dlogsinhqsqdqsqrt_slot;
        let mut var_dlogsinhqsqdqsqrt_dn3: f64 = *var_dlogsinhqsqdqsqrt_dn3_slot;
        let mut var_dlogsinhqsqdqsqrt_dn4: f64 = *var_dlogsinhqsqdqsqrt_dn4_slot;
        let mut var_dlogsinhqsqdqsqrt_dn5: f64 = *var_dlogsinhqsqdqsqrt_dn5_slot;
        let mut var_dlogsinhqsqdqsqrt_dn6: f64 = *var_dlogsinhqsqdqsqrt_dn6_slot;
        let mut var_dlogsinhqsqdqsqrt_dn7: f64 = *var_dlogsinhqsqdqsqrt_dn7_slot;
        let mut var_dlogsinhqsqdqsqrt_dn8: f64 = *var_dlogsinhqsqdqsqrt_dn8_slot;
        let mut var_dq2: f64 = *var_dq2_slot;
        let mut var_dq2_dn3: f64 = *var_dq2_dn3_slot;
        let mut var_dq2_dn4: f64 = *var_dq2_dn4_slot;
        let mut var_dq2_dn5: f64 = *var_dq2_dn5_slot;
        let mut var_dq2_dn6: f64 = *var_dq2_dn6_slot;
        let mut var_dq2_dn7: f64 = *var_dq2_dn7_slot;
        let mut var_dq2_dn8: f64 = *var_dq2_dn8_slot;
        let mut var_dqcoth: f64 = *var_dqcoth_slot;
        let mut var_dqcoth_dn3: f64 = *var_dqcoth_dn3_slot;
        let mut var_dqcoth_dn4: f64 = *var_dqcoth_dn4_slot;
        let mut var_dqcoth_dn5: f64 = *var_dqcoth_dn5_slot;
        let mut var_dqcoth_dn6: f64 = *var_dqcoth_dn6_slot;
        let mut var_dqcoth_dn7: f64 = *var_dqcoth_dn7_slot;
        let mut var_dqcoth_dn8: f64 = *var_dqcoth_dn8_slot;
        let mut var_dqcothqdqsqrt: f64 = *var_dqcothqdqsqrt_slot;
        let mut var_dqcothqdqsqrt_dn3: f64 = *var_dqcothqdqsqrt_dn3_slot;
        let mut var_dqcothqdqsqrt_dn4: f64 = *var_dqcothqdqsqrt_dn4_slot;
        let mut var_dqcothqdqsqrt_dn5: f64 = *var_dqcothqdqsqrt_dn5_slot;
        let mut var_dqcothqdqsqrt_dn6: f64 = *var_dqcothqdqsqrt_dn6_slot;
        let mut var_dqcothqdqsqrt_dn7: f64 = *var_dqcothqdqsqrt_dn7_slot;
        let mut var_dqcothqdqsqrt_dn8: f64 = *var_dqcothqdqsqrt_dn8_slot;
        let mut var_dqsqrt: f64 = *var_dqsqrt_slot;
        let mut var_dqsqrt_dn3: f64 = *var_dqsqrt_dn3_slot;
        let mut var_dqsqrt_dn4: f64 = *var_dqsqrt_dn4_slot;
        let mut var_dqsqrt_dn5: f64 = *var_dqsqrt_dn5_slot;
        let mut var_dqsqrt_dn6: f64 = *var_dqsqrt_dn6_slot;
        let mut var_dqsqrt_dn7: f64 = *var_dqsqrt_dn7_slot;
        let mut var_dqsqrt_dn8: f64 = *var_dqsqrt_dn8_slot;
        let mut var_f: f64 = *var_f_slot;
        let mut var_f_dn3: f64 = *var_f_dn3_slot;
        let mut var_f_dn4: f64 = *var_f_dn4_slot;
        let mut var_f_dn5: f64 = *var_f_dn5_slot;
        let mut var_f_dn6: f64 = *var_f_dn6_slot;
        let mut var_f_dn7: f64 = *var_f_dn7_slot;
        let mut var_f_dn8: f64 = *var_f_dn8_slot;
        let mut var_guard95: f64 = *var_guard95_slot;
        let mut var_phi1: f64 = *var_phi1_slot;
        let mut var_phi1_dn3: f64 = *var_phi1_dn3_slot;
        let mut var_phi1_dn4: f64 = *var_phi1_dn4_slot;
        let mut var_phi1_dn5: f64 = *var_phi1_dn5_slot;
        let mut var_phi1_dn6: f64 = *var_phi1_dn6_slot;
        let mut var_phi1_dn7: f64 = *var_phi1_dn7_slot;
        let mut var_phi1_dn8: f64 = *var_phi1_dn8_slot;
        let mut var_q: f64 = *var_q_slot;
        let mut var_q1: f64 = *var_q1_slot;
        let mut var_q1_dn3: f64 = *var_q1_dn3_slot;
        let mut var_q1_dn4: f64 = *var_q1_dn4_slot;
        let mut var_q1_dn5: f64 = *var_q1_dn5_slot;
        let mut var_q1_dn6: f64 = *var_q1_dn6_slot;
        let mut var_q1_dn7: f64 = *var_q1_dn7_slot;
        let mut var_q1_dn8: f64 = *var_q1_dn8_slot;
        let mut var_q2: f64 = *var_q2_slot;
        let mut var_q2_dn3: f64 = *var_q2_dn3_slot;
        let mut var_q2_dn4: f64 = *var_q2_dn4_slot;
        let mut var_q2_dn5: f64 = *var_q2_dn5_slot;
        let mut var_q2_dn6: f64 = *var_q2_dn6_slot;
        let mut var_q2_dn7: f64 = *var_q2_dn7_slot;
        let mut var_q2_dn8: f64 = *var_q2_dn8_slot;
        let mut var_q_dn3: f64 = *var_q_dn3_slot;
        let mut var_q_dn4: f64 = *var_q_dn4_slot;
        let mut var_q_dn5: f64 = *var_q_dn5_slot;
        let mut var_q_dn6: f64 = *var_q_dn6_slot;
        let mut var_q_dn7: f64 = *var_q_dn7_slot;
        let mut var_q_dn8: f64 = *var_q_dn8_slot;
        let mut var_qcoth: f64 = *var_qcoth_slot;
        let mut var_qcoth_dn3: f64 = *var_qcoth_dn3_slot;
        let mut var_qcoth_dn4: f64 = *var_qcoth_dn4_slot;
        let mut var_qcoth_dn5: f64 = *var_qcoth_dn5_slot;
        let mut var_qcoth_dn6: f64 = *var_qcoth_dn6_slot;
        let mut var_qcoth_dn7: f64 = *var_qcoth_dn7_slot;
        let mut var_qcoth_dn8: f64 = *var_qcoth_dn8_slot;
        let mut var_qsqrt: f64 = *var_qsqrt_slot;
        let mut var_qsqrt_dn3: f64 = *var_qsqrt_dn3_slot;
        let mut var_qsqrt_dn4: f64 = *var_qsqrt_dn4_slot;
        let mut var_qsqrt_dn5: f64 = *var_qsqrt_dn5_slot;
        let mut var_qsqrt_dn6: f64 = *var_qsqrt_dn6_slot;
        let mut var_qsqrt_dn7: f64 = *var_qsqrt_dn7_slot;
        let mut var_qsqrt_dn8: f64 = *var_qsqrt_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;

        let (assign9230_e8905, assign9230_e8905_d_n3, assign9230_e8905_d_n4, assign9230_e8905_d_n5, assign9230_e8905_d_n6, assign9230_e8905_d_n7, assign9230_e8905_d_n8,) = {
    if (var_guard94 != 0.0) {
        let assign9230_e8901: f64 = (0.25 * var_t1);
        let assign9230_e8903: f64 = (assign9230_e8901 + var_t0);
        (assign9230_e8903, ((0.25 * var_t1_dn3) + var_t0_dn3), ((0.25 * var_t1_dn4) + var_t0_dn4), ((0.25 * var_t1_dn5) + var_t0_dn5), ((0.25 * var_t1_dn6) + var_t0_dn6), ((0.25 * var_t1_dn7) + var_t0_dn7), ((0.25 * var_t1_dn8) + var_t0_dn8),)
    } else {
        (var_dqcothqdqsqrt, var_dqcothqdqsqrt_dn3, var_dqcothqdqsqrt_dn4, var_dqcothqdqsqrt_dn5, var_dqcothqdqsqrt_dn6, var_dqcothqdqsqrt_dn7, var_dqcothqdqsqrt_dn8,)
    }
};
        var_dqcothqdqsqrt = assign9230_e8905;
        var_dqcothqdqsqrt_dn3 = assign9230_e8905_d_n3;
        var_dqcothqdqsqrt_dn4 = assign9230_e8905_d_n4;
        var_dqcothqdqsqrt_dn5 = assign9230_e8905_d_n5;
        var_dqcothqdqsqrt_dn6 = assign9230_e8905_d_n6;
        var_dqcothqdqsqrt_dn7 = assign9230_e8905_d_n7;
        var_dqcothqdqsqrt_dn8 = assign9230_e8905_d_n8;

        let (assign9240_e8911, assign9240_e8911_d_n3, assign9240_e8911_d_n4, assign9240_e8911_d_n5, assign9240_e8911_d_n6, assign9240_e8911_d_n7, assign9240_e8911_d_n8,) = {
    if (var_guard94 == 0.0) {
        let assign9240_e8909: f64 = (var_qsqrt).sqrt();
        (assign9240_e8909, (var_qsqrt_dn3 / (2.0 * assign9240_e8909)), (var_qsqrt_dn4 / (2.0 * assign9240_e8909)), (var_qsqrt_dn5 / (2.0 * assign9240_e8909)), (var_qsqrt_dn6 / (2.0 * assign9240_e8909)), (var_qsqrt_dn7 / (2.0 * assign9240_e8909)), (var_qsqrt_dn8 / (2.0 * assign9240_e8909)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign9240_e8911;
        var_q_dn3 = assign9240_e8911_d_n3;
        var_q_dn4 = assign9240_e8911_d_n4;
        var_q_dn5 = assign9240_e8911_d_n5;
        var_q_dn6 = assign9240_e8911_d_n6;
        var_q_dn7 = assign9240_e8911_d_n7;
        var_q_dn8 = assign9240_e8911_d_n8;

        let (assign9250_e8921, assign9250_e8921_d_n3, assign9250_e8921_d_n4, assign9250_e8921_d_n5, assign9250_e8921_d_n6, assign9250_e8921_d_n7, assign9250_e8921_d_n8,) = {
    if (var_guard94 == 0.0) {
        let assign9250_e8917: f64 = (0.5 * var_q);
        let assign9250_e8918: f64 = (assign9250_e8917).sinh();
        let assign9250_e8919: f64 = (1.0 / assign9250_e8918);
        (assign9250_e8919, (-(((assign9250_e8917).cosh() * (0.5 * var_q_dn3)) / (assign9250_e8918 * assign9250_e8918))), (-(((assign9250_e8917).cosh() * (0.5 * var_q_dn4)) / (assign9250_e8918 * assign9250_e8918))), (-(((assign9250_e8917).cosh() * (0.5 * var_q_dn5)) / (assign9250_e8918 * assign9250_e8918))), (-(((assign9250_e8917).cosh() * (0.5 * var_q_dn6)) / (assign9250_e8918 * assign9250_e8918))), (-(((assign9250_e8917).cosh() * (0.5 * var_q_dn7)) / (assign9250_e8918 * assign9250_e8918))), (-(((assign9250_e8917).cosh() * (0.5 * var_q_dn8)) / (assign9250_e8918 * assign9250_e8918))),)
    } else {
        (var_csc1, var_csc1_dn3, var_csc1_dn4, var_csc1_dn5, var_csc1_dn6, var_csc1_dn7, var_csc1_dn8,)
    }
};
        var_csc1 = assign9250_e8921;
        var_csc1_dn3 = assign9250_e8921_d_n3;
        var_csc1_dn4 = assign9250_e8921_d_n4;
        var_csc1_dn5 = assign9250_e8921_d_n5;
        var_csc1_dn6 = assign9250_e8921_d_n6;
        var_csc1_dn7 = assign9250_e8921_d_n7;
        var_csc1_dn8 = assign9250_e8921_d_n8;

        let (assign9260_e8928, assign9260_e8928_d_n3, assign9260_e8928_d_n4, assign9260_e8928_d_n5, assign9260_e8928_d_n6, assign9260_e8928_d_n7, assign9260_e8928_d_n8,) = {
    if (var_guard94 == 0.0) {
        let assign9260_e8926: f64 = (var_csc1 * var_csc1);
        (assign9260_e8926, ((var_csc1_dn3 * var_csc1) + (var_csc1 * var_csc1_dn3)), ((var_csc1_dn4 * var_csc1) + (var_csc1 * var_csc1_dn4)), ((var_csc1_dn5 * var_csc1) + (var_csc1 * var_csc1_dn5)), ((var_csc1_dn6 * var_csc1) + (var_csc1 * var_csc1_dn6)), ((var_csc1_dn7 * var_csc1) + (var_csc1 * var_csc1_dn7)), ((var_csc1_dn8 * var_csc1) + (var_csc1 * var_csc1_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign9260_e8928;
        var_t1_dn3 = assign9260_e8928_d_n3;
        var_t1_dn4 = assign9260_e8928_d_n4;
        var_t1_dn5 = assign9260_e8928_d_n5;
        var_t1_dn6 = assign9260_e8928_d_n6;
        var_t1_dn7 = assign9260_e8928_d_n7;
        var_t1_dn8 = assign9260_e8928_d_n8;

        let (assign9270_e8936, assign9270_e8936_d_n3, assign9270_e8936_d_n4, assign9270_e8936_d_n5, assign9270_e8936_d_n6, assign9270_e8936_d_n7, assign9270_e8936_d_n8,) = {
    if (var_guard94 == 0.0) {
        let assign9270_e8933: f64 = (1.0 + var_t1);
        let assign9270_e8934: f64 = (assign9270_e8933).sqrt();
        (assign9270_e8934, (var_t1_dn3 / (2.0 * assign9270_e8934)), (var_t1_dn4 / (2.0 * assign9270_e8934)), (var_t1_dn5 / (2.0 * assign9270_e8934)), (var_t1_dn6 / (2.0 * assign9270_e8934)), (var_t1_dn7 / (2.0 * assign9270_e8934)), (var_t1_dn8 / (2.0 * assign9270_e8934)),)
    } else {
        (var_coth1, var_coth1_dn3, var_coth1_dn4, var_coth1_dn5, var_coth1_dn6, var_coth1_dn7, var_coth1_dn8,)
    }
};
        var_coth1 = assign9270_e8936;
        var_coth1_dn3 = assign9270_e8936_d_n3;
        var_coth1_dn4 = assign9270_e8936_d_n4;
        var_coth1_dn5 = assign9270_e8936_d_n5;
        var_coth1_dn6 = assign9270_e8936_d_n6;
        var_coth1_dn7 = assign9270_e8936_d_n7;
        var_coth1_dn8 = assign9270_e8936_d_n8;

        let (assign9280_e8945, assign9280_e8945_d_n3, assign9280_e8945_d_n4, assign9280_e8945_d_n5, assign9280_e8945_d_n6, assign9280_e8945_d_n7, assign9280_e8945_d_n8,) = {
    if (var_guard94 == 0.0) {
        let assign9280_e8941: f64 = (0.5 * var_coth1);
        let assign9280_e8943: f64 = (assign9280_e8941 / var_q);
        (assign9280_e8943, ((((0.5 * var_coth1_dn3) * var_q) - (assign9280_e8941 * var_q_dn3)) / (var_q * var_q)), ((((0.5 * var_coth1_dn4) * var_q) - (assign9280_e8941 * var_q_dn4)) / (var_q * var_q)), ((((0.5 * var_coth1_dn5) * var_q) - (assign9280_e8941 * var_q_dn5)) / (var_q * var_q)), ((((0.5 * var_coth1_dn6) * var_q) - (assign9280_e8941 * var_q_dn6)) / (var_q * var_q)), ((((0.5 * var_coth1_dn7) * var_q) - (assign9280_e8941 * var_q_dn7)) / (var_q * var_q)), ((((0.5 * var_coth1_dn8) * var_q) - (assign9280_e8941 * var_q_dn8)) / (var_q * var_q)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign9280_e8945;
        var_t0_dn3 = assign9280_e8945_d_n3;
        var_t0_dn4 = assign9280_e8945_d_n4;
        var_t0_dn5 = assign9280_e8945_d_n5;
        var_t0_dn6 = assign9280_e8945_d_n6;
        var_t0_dn7 = assign9280_e8945_d_n7;
        var_t0_dn8 = assign9280_e8945_d_n8;

        let (assign9290_e8955, assign9290_e8955_d_n3, assign9290_e8955_d_n4, assign9290_e8955_d_n5, assign9290_e8955_d_n6, assign9290_e8955_d_n7, assign9290_e8955_d_n8,) = {
    if (var_guard94 == 0.0) {
        let assign9290_e8949: f64 = (-0.25);
        let assign9290_e8951: f64 = (assign9290_e8949 * var_t1);
        let assign9290_e8953: f64 = (assign9290_e8951 + var_t0);
        (assign9290_e8953, ((assign9290_e8949 * var_t1_dn3) + var_t0_dn3), ((assign9290_e8949 * var_t1_dn4) + var_t0_dn4), ((assign9290_e8949 * var_t1_dn5) + var_t0_dn5), ((assign9290_e8949 * var_t1_dn6) + var_t0_dn6), ((assign9290_e8949 * var_t1_dn7) + var_t0_dn7), ((assign9290_e8949 * var_t1_dn8) + var_t0_dn8),)
    } else {
        (var_dqcothqdqsqrt, var_dqcothqdqsqrt_dn3, var_dqcothqdqsqrt_dn4, var_dqcothqdqsqrt_dn5, var_dqcothqdqsqrt_dn6, var_dqcothqdqsqrt_dn7, var_dqcothqdqsqrt_dn8,)
    }
};
        var_dqcothqdqsqrt = assign9290_e8955;
        var_dqcothqdqsqrt_dn3 = assign9290_e8955_d_n3;
        var_dqcothqdqsqrt_dn4 = assign9290_e8955_d_n4;
        var_dqcothqdqsqrt_dn5 = assign9290_e8955_d_n5;
        var_dqcothqdqsqrt_dn6 = assign9290_e8955_d_n6;
        var_dqcothqdqsqrt_dn7 = assign9290_e8955_d_n7;
        var_dqcothqdqsqrt_dn8 = assign9290_e8955_d_n8;

        let assign9300_e8958: f64 = (var_q * var_coth1);
        var_qcoth = assign9300_e8958;
        var_qcoth_dn3 = ((var_q_dn3 * var_coth1) + (var_q * var_coth1_dn3));
        var_qcoth_dn4 = ((var_q_dn4 * var_coth1) + (var_q * var_coth1_dn4));
        var_qcoth_dn5 = ((var_q_dn5 * var_coth1) + (var_q * var_coth1_dn5));
        var_qcoth_dn6 = ((var_q_dn6 * var_coth1) + (var_q * var_coth1_dn6));
        var_qcoth_dn7 = ((var_q_dn7 * var_coth1) + (var_q * var_coth1_dn7));
        var_qcoth_dn8 = ((var_q_dn8 * var_coth1) + (var_q * var_coth1_dn8));

        let assign9310_e8961: f64 = (var_auxb1 + var_qcoth);
        var_t2 = assign9310_e8961;
        var_t2_dn3 = (var_auxb1_dn3 + var_qcoth_dn3);
        var_t2_dn4 = (var_auxb1_dn4 + var_qcoth_dn4);
        var_t2_dn5 = (var_auxb1_dn5 + var_qcoth_dn5);
        var_t2_dn6 = (var_auxb1_dn6 + var_qcoth_dn6);
        var_t2_dn7 = (var_auxb1_dn7 + var_qcoth_dn7);
        var_t2_dn8 = (var_auxb1_dn8 + var_qcoth_dn8);

        let assign9320_e8964: f64 = (1.0 / var_t2);
        var_t3 = assign9320_e8964;
        var_t3_dn3 = (-(var_t2_dn3 / (var_t2 * var_t2)));
        var_t3_dn4 = (-(var_t2_dn4 / (var_t2 * var_t2)));
        var_t3_dn5 = (-(var_t2_dn5 / (var_t2 * var_t2)));
        var_t3_dn6 = (-(var_t2_dn6 / (var_t2 * var_t2)));
        var_t3_dn7 = (-(var_t2_dn7 / (var_t2 * var_t2)));
        var_t3_dn8 = (-(var_t2_dn8 / (var_t2 * var_t2)));

        let assign9330_e8967: f64 = (var_xg2 - var_xg1);
        let assign9330_e8969: f64 = (assign9330_e8967 + var_q1);
        let assign9330_e8972: f64 = (var_qsqrt * var_t1);
        let assign9330_e8974: f64 = (assign9330_e8972 * var_t3);
        let assign9330_e8976: f64 = (assign9330_e8974 * var_t3);
        let assign9330_e8977: f64 = (assign9330_e8976).abs();
        let assign9330_e8978: f64 = (assign9330_e8977).ln();
        let assign9330_e8979: f64 = (assign9330_e8969 - assign9330_e8978);
        var_q2 = assign9330_e8979;
        var_q2_dn3 = (((var_xg2_dn3 - var_xg1_dn3) + var_q1_dn3) - (if assign9330_e8976 >= 0.0 { ((((((var_qsqrt_dn3 * var_t1) + (var_qsqrt * var_t1_dn3)) * var_t3) + (assign9330_e8972 * var_t3_dn3)) * var_t3) + (assign9330_e8974 * var_t3_dn3)) } else { (-((((((var_qsqrt_dn3 * var_t1) + (var_qsqrt * var_t1_dn3)) * var_t3) + (assign9330_e8972 * var_t3_dn3)) * var_t3) + (assign9330_e8974 * var_t3_dn3))) } / assign9330_e8977));
        var_q2_dn4 = (((var_xg2_dn4 - var_xg1_dn4) + var_q1_dn4) - (if assign9330_e8976 >= 0.0 { ((((((var_qsqrt_dn4 * var_t1) + (var_qsqrt * var_t1_dn4)) * var_t3) + (assign9330_e8972 * var_t3_dn4)) * var_t3) + (assign9330_e8974 * var_t3_dn4)) } else { (-((((((var_qsqrt_dn4 * var_t1) + (var_qsqrt * var_t1_dn4)) * var_t3) + (assign9330_e8972 * var_t3_dn4)) * var_t3) + (assign9330_e8974 * var_t3_dn4))) } / assign9330_e8977));
        var_q2_dn5 = (((var_xg2_dn5 - var_xg1_dn5) + var_q1_dn5) - (if assign9330_e8976 >= 0.0 { ((((((var_qsqrt_dn5 * var_t1) + (var_qsqrt * var_t1_dn5)) * var_t3) + (assign9330_e8972 * var_t3_dn5)) * var_t3) + (assign9330_e8974 * var_t3_dn5)) } else { (-((((((var_qsqrt_dn5 * var_t1) + (var_qsqrt * var_t1_dn5)) * var_t3) + (assign9330_e8972 * var_t3_dn5)) * var_t3) + (assign9330_e8974 * var_t3_dn5))) } / assign9330_e8977));
        var_q2_dn6 = (((var_xg2_dn6 - var_xg1_dn6) + var_q1_dn6) - (if assign9330_e8976 >= 0.0 { ((((((var_qsqrt_dn6 * var_t1) + (var_qsqrt * var_t1_dn6)) * var_t3) + (assign9330_e8972 * var_t3_dn6)) * var_t3) + (assign9330_e8974 * var_t3_dn6)) } else { (-((((((var_qsqrt_dn6 * var_t1) + (var_qsqrt * var_t1_dn6)) * var_t3) + (assign9330_e8972 * var_t3_dn6)) * var_t3) + (assign9330_e8974 * var_t3_dn6))) } / assign9330_e8977));
        var_q2_dn7 = (((var_xg2_dn7 - var_xg1_dn7) + var_q1_dn7) - (if assign9330_e8976 >= 0.0 { ((((((var_qsqrt_dn7 * var_t1) + (var_qsqrt * var_t1_dn7)) * var_t3) + (assign9330_e8972 * var_t3_dn7)) * var_t3) + (assign9330_e8974 * var_t3_dn7)) } else { (-((((((var_qsqrt_dn7 * var_t1) + (var_qsqrt * var_t1_dn7)) * var_t3) + (assign9330_e8972 * var_t3_dn7)) * var_t3) + (assign9330_e8974 * var_t3_dn7))) } / assign9330_e8977));
        var_q2_dn8 = (((var_xg2_dn8 - var_xg1_dn8) + var_q1_dn8) - (if assign9330_e8976 >= 0.0 { ((((((var_qsqrt_dn8 * var_t1) + (var_qsqrt * var_t1_dn8)) * var_t3) + (assign9330_e8972 * var_t3_dn8)) * var_t3) + (assign9330_e8974 * var_t3_dn8)) } else { (-((((((var_qsqrt_dn8 * var_t1) + (var_qsqrt * var_t1_dn8)) * var_t3) + (assign9330_e8972 * var_t3_dn8)) * var_t3) + (assign9330_e8974 * var_t3_dn8))) } / assign9330_e8977));

        let assign9340_e8983: f64 = (var_auxb1 + var_qcoth);
        let assign9340_e8986: f64 = (var_k2 * var_q2);
        let assign9340_e8988: f64 = (assign9340_e8986 + var_auxb1);
        let assign9340_e8989: f64 = (assign9340_e8983 * assign9340_e8988);
        let assign9340_e8990: f64 = (var_aaux + assign9340_e8989);
        var_f = assign9340_e8990;
        var_f_dn3 = (var_aaux_dn3 + (((var_auxb1_dn3 + var_qcoth_dn3) * assign9340_e8988) + (assign9340_e8983 * ((var_k2 * var_q2_dn3) + var_auxb1_dn3))));
        var_f_dn4 = (var_aaux_dn4 + (((var_auxb1_dn4 + var_qcoth_dn4) * assign9340_e8988) + (assign9340_e8983 * ((var_k2 * var_q2_dn4) + var_auxb1_dn4))));
        var_f_dn5 = (var_aaux_dn5 + (((var_auxb1_dn5 + var_qcoth_dn5) * assign9340_e8988) + (assign9340_e8983 * ((var_k2 * var_q2_dn5) + var_auxb1_dn5))));
        var_f_dn6 = (var_aaux_dn6 + (((var_auxb1_dn6 + var_qcoth_dn6) * assign9340_e8988) + (assign9340_e8983 * ((var_k2 * var_q2_dn6) + var_auxb1_dn6))));
        var_f_dn7 = (var_aaux_dn7 + (((var_auxb1_dn7 + var_qcoth_dn7) * assign9340_e8988) + (assign9340_e8983 * ((var_k2 * var_q2_dn7) + var_auxb1_dn7))));
        var_f_dn8 = (var_aaux_dn8 + (((var_auxb1_dn8 + var_qcoth_dn8) * assign9340_e8988) + (assign9340_e8983 * ((var_k2 * var_q2_dn8) + var_auxb1_dn8))));

        let assign9350_e8993: f64 = (1.0 / var_qsqrt);
        let assign9350_e8995: f64 = (assign9350_e8993 - var_t0);
        var_dlogsinhqsqdqsqrt = assign9350_e8995;
        var_dlogsinhqsqdqsqrt_dn3 = ((-(var_qsqrt_dn3 / (var_qsqrt * var_qsqrt))) - var_t0_dn3);
        var_dlogsinhqsqdqsqrt_dn4 = ((-(var_qsqrt_dn4 / (var_qsqrt * var_qsqrt))) - var_t0_dn4);
        var_dlogsinhqsqdqsqrt_dn5 = ((-(var_qsqrt_dn5 / (var_qsqrt * var_qsqrt))) - var_t0_dn5);
        var_dlogsinhqsqdqsqrt_dn6 = ((-(var_qsqrt_dn6 / (var_qsqrt * var_qsqrt))) - var_t0_dn6);
        var_dlogsinhqsqdqsqrt_dn7 = ((-(var_qsqrt_dn7 / (var_qsqrt * var_qsqrt))) - var_t0_dn7);
        var_dlogsinhqsqdqsqrt_dn8 = ((-(var_qsqrt_dn8 / (var_qsqrt * var_qsqrt))) - var_t0_dn8);

        let assign9360_e8997: f64 = (-2.0);
        let assign9360_e8999: f64 = (assign9360_e8997 * var_k1);
        let assign9360_e9001: f64 = (assign9360_e8999 * var_auxb1);
        let assign9360_e9003: f64 = (assign9360_e9001 + var_aaux);
        var_dqsqrt = assign9360_e9003;
        var_dqsqrt_dn3 = ((assign9360_e8999 * var_auxb1_dn3) + var_aaux_dn3);
        var_dqsqrt_dn4 = ((assign9360_e8999 * var_auxb1_dn4) + var_aaux_dn4);
        var_dqsqrt_dn5 = ((assign9360_e8999 * var_auxb1_dn5) + var_aaux_dn5);
        var_dqsqrt_dn6 = ((assign9360_e8999 * var_auxb1_dn6) + var_aaux_dn6);
        var_dqsqrt_dn7 = ((assign9360_e8999 * var_auxb1_dn7) + var_aaux_dn7);
        var_dqsqrt_dn8 = ((assign9360_e8999 * var_auxb1_dn8) + var_aaux_dn8);

        let assign9370_e9006: f64 = (var_dqcothqdqsqrt * var_dqsqrt);
        var_dqcoth = assign9370_e9006;
        var_dqcoth_dn3 = ((var_dqcothqdqsqrt_dn3 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn3));
        var_dqcoth_dn4 = ((var_dqcothqdqsqrt_dn4 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn4));
        var_dqcoth_dn5 = ((var_dqcothqdqsqrt_dn5 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn5));
        var_dqcoth_dn6 = ((var_dqcothqdqsqrt_dn6 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn6));
        var_dqcoth_dn7 = ((var_dqcothqdqsqrt_dn7 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn7));
        var_dqcoth_dn8 = ((var_dqcothqdqsqrt_dn8 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn8));

        let assign9380_e9008: f64 = (-1.0);
        let assign9380_e9011: f64 = (-var_k1);
        let assign9380_e9013: f64 = (assign9380_e9011 + var_dqcoth);
        let assign9380_e9015: f64 = (assign9380_e9013 * var_t3);
        let assign9380_e9016: f64 = (2.0 * assign9380_e9015);
        let assign9380_e9017: f64 = (assign9380_e9008 + assign9380_e9016);
        let assign9380_e9020: f64 = (var_dlogsinhqsqdqsqrt * var_dqsqrt);
        let assign9380_e9021: f64 = (assign9380_e9017 - assign9380_e9020);
        var_dq2 = assign9380_e9021;
        var_dq2_dn3 = ((2.0 * ((var_dqcoth_dn3 * var_t3) + (assign9380_e9013 * var_t3_dn3))) - ((var_dlogsinhqsqdqsqrt_dn3 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn3)));
        var_dq2_dn4 = ((2.0 * ((var_dqcoth_dn4 * var_t3) + (assign9380_e9013 * var_t3_dn4))) - ((var_dlogsinhqsqdqsqrt_dn4 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn4)));
        var_dq2_dn5 = ((2.0 * ((var_dqcoth_dn5 * var_t3) + (assign9380_e9013 * var_t3_dn5))) - ((var_dlogsinhqsqdqsqrt_dn5 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn5)));
        var_dq2_dn6 = ((2.0 * ((var_dqcoth_dn6 * var_t3) + (assign9380_e9013 * var_t3_dn6))) - ((var_dlogsinhqsqdqsqrt_dn6 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn6)));
        var_dq2_dn7 = ((2.0 * ((var_dqcoth_dn7 * var_t3) + (assign9380_e9013 * var_t3_dn7))) - ((var_dlogsinhqsqdqsqrt_dn7 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn7)));
        var_dq2_dn8 = ((2.0 * ((var_dqcoth_dn8 * var_t3) + (assign9380_e9013 * var_t3_dn8))) - ((var_dlogsinhqsqdqsqrt_dn8 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn8)));

        let assign9390_e9026: f64 = (var_auxb1 + var_t2);
        let assign9390_e9027: f64 = (var_k1 * assign9390_e9026);
        let assign9390_e9028: f64 = (var_aaux - assign9390_e9027);
        let assign9390_e9031: f64 = (var_auxb1 * var_dqcoth);
        let assign9390_e9032: f64 = (assign9390_e9028 + assign9390_e9031);
        let assign9390_e9036: f64 = (var_dq2 * var_t2);
        let assign9390_e9040: f64 = (var_dqcoth - var_k1);
        let assign9390_e9041: f64 = (var_q2 * assign9390_e9040);
        let assign9390_e9042: f64 = (assign9390_e9036 + assign9390_e9041);
        let assign9390_e9043: f64 = (var_k2 * assign9390_e9042);
        let assign9390_e9044: f64 = (assign9390_e9032 + assign9390_e9043);
        var_df = assign9390_e9044;
        var_df_dn3 = (((var_aaux_dn3 - (var_k1 * (var_auxb1_dn3 + var_t2_dn3))) + ((var_auxb1_dn3 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn3))) + (var_k2 * (((var_dq2_dn3 * var_t2) + (var_dq2 * var_t2_dn3)) + ((var_q2_dn3 * assign9390_e9040) + (var_q2 * var_dqcoth_dn3)))));
        var_df_dn4 = (((var_aaux_dn4 - (var_k1 * (var_auxb1_dn4 + var_t2_dn4))) + ((var_auxb1_dn4 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn4))) + (var_k2 * (((var_dq2_dn4 * var_t2) + (var_dq2 * var_t2_dn4)) + ((var_q2_dn4 * assign9390_e9040) + (var_q2 * var_dqcoth_dn4)))));
        var_df_dn5 = (((var_aaux_dn5 - (var_k1 * (var_auxb1_dn5 + var_t2_dn5))) + ((var_auxb1_dn5 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn5))) + (var_k2 * (((var_dq2_dn5 * var_t2) + (var_dq2 * var_t2_dn5)) + ((var_q2_dn5 * assign9390_e9040) + (var_q2 * var_dqcoth_dn5)))));
        var_df_dn6 = (((var_aaux_dn6 - (var_k1 * (var_auxb1_dn6 + var_t2_dn6))) + ((var_auxb1_dn6 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn6))) + (var_k2 * (((var_dq2_dn6 * var_t2) + (var_dq2 * var_t2_dn6)) + ((var_q2_dn6 * assign9390_e9040) + (var_q2 * var_dqcoth_dn6)))));
        var_df_dn7 = (((var_aaux_dn7 - (var_k1 * (var_auxb1_dn7 + var_t2_dn7))) + ((var_auxb1_dn7 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn7))) + (var_k2 * (((var_dq2_dn7 * var_t2) + (var_dq2 * var_t2_dn7)) + ((var_q2_dn7 * assign9390_e9040) + (var_q2 * var_dqcoth_dn7)))));
        var_df_dn8 = (((var_aaux_dn8 - (var_k1 * (var_auxb1_dn8 + var_t2_dn8))) + ((var_auxb1_dn8 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn8))) + (var_k2 * (((var_dq2_dn8 * var_t2) + (var_dq2 * var_t2_dn8)) + ((var_q2_dn8 * assign9390_e9040) + (var_q2 * var_dqcoth_dn8)))));

        let assign9400_e9046: f64 = (-var_f);
        let assign9400_e9048: f64 = (assign9400_e9046 / var_df);
        var_delta = assign9400_e9048;
        var_delta_dn3 = ((((-var_f_dn3) * var_df) - (assign9400_e9046 * var_df_dn3)) / (var_df * var_df));
        var_delta_dn4 = ((((-var_f_dn4) * var_df) - (assign9400_e9046 * var_df_dn4)) / (var_df * var_df));
        var_delta_dn5 = ((((-var_f_dn5) * var_df) - (assign9400_e9046 * var_df_dn5)) / (var_df * var_df));
        var_delta_dn6 = ((((-var_f_dn6) * var_df) - (assign9400_e9046 * var_df_dn6)) / (var_df * var_df));
        var_delta_dn7 = ((((-var_f_dn7) * var_df) - (assign9400_e9046 * var_df_dn7)) / (var_df * var_df));
        var_delta_dn8 = ((((-var_f_dn8) * var_df) - (assign9400_e9046 * var_df_dn8)) / (var_df * var_df));

        let assign9410_e9051: f64 = (var_phi1 + var_delta);
        var_phi1 = assign9410_e9051;
        var_phi1_dn3 = (var_phi1_dn3 + var_delta_dn3);
        var_phi1_dn4 = (var_phi1_dn4 + var_delta_dn4);
        var_phi1_dn5 = (var_phi1_dn5 + var_delta_dn5);
        var_phi1_dn6 = (var_phi1_dn6 + var_delta_dn6);
        var_phi1_dn7 = (var_phi1_dn7 + var_delta_dn7);
        var_phi1_dn8 = (var_phi1_dn8 + var_delta_dn8);

        let assign9420_e9054: f64 = (var_xg1 - var_phi1);
        var_q1 = assign9420_e9054;
        var_q1_dn3 = (var_xg1_dn3 - var_phi1_dn3);
        var_q1_dn4 = (var_xg1_dn4 - var_phi1_dn4);
        var_q1_dn5 = (var_xg1_dn5 - var_phi1_dn5);
        var_q1_dn6 = (var_xg1_dn6 - var_phi1_dn6);
        var_q1_dn7 = (var_xg1_dn7 - var_phi1_dn7);
        var_q1_dn8 = (var_xg1_dn8 - var_phi1_dn8);

        let assign9430_e9057: f64 = (var_k1 * var_q1);
        var_auxb1 = assign9430_e9057;
        var_auxb1_dn3 = (var_k1 * var_q1_dn3);
        var_auxb1_dn4 = (var_k1 * var_q1_dn4);
        var_auxb1_dn5 = (var_k1 * var_q1_dn5);
        var_auxb1_dn6 = (var_k1 * var_q1_dn6);
        var_auxb1_dn7 = (var_k1 * var_q1_dn7);
        var_auxb1_dn8 = (var_k1 * var_q1_dn8);

        let assign9440_e9059: f64 = (-var_a0);
        let assign9440_e9061: f64 = (var_phi1).exp();
        let assign9440_e9062: f64 = (assign9440_e9059 * assign9440_e9061);
        var_aaux = assign9440_e9062;
        var_aaux_dn3 = (((-var_a0_dn3) * assign9440_e9061) + (assign9440_e9059 * (assign9440_e9061 * var_phi1_dn3)));
        var_aaux_dn4 = (((-var_a0_dn4) * assign9440_e9061) + (assign9440_e9059 * (assign9440_e9061 * var_phi1_dn4)));
        var_aaux_dn5 = (((-var_a0_dn5) * assign9440_e9061) + (assign9440_e9059 * (assign9440_e9061 * var_phi1_dn5)));
        var_aaux_dn6 = (((-var_a0_dn6) * assign9440_e9061) + (assign9440_e9059 * (assign9440_e9061 * var_phi1_dn6)));
        var_aaux_dn7 = (((-var_a0_dn7) * assign9440_e9061) + (assign9440_e9059 * (assign9440_e9061 * var_phi1_dn7)));
        var_aaux_dn8 = (((-var_a0_dn8) * assign9440_e9061) + (assign9440_e9059 * (assign9440_e9061 * var_phi1_dn8)));

        let assign9450_e9065: f64 = (var_auxb1 * var_auxb1);
        let assign9450_e9067: f64 = (assign9450_e9065 + var_aaux);
        var_qsqrt = assign9450_e9067;
        var_qsqrt_dn3 = (((var_auxb1_dn3 * var_auxb1) + (var_auxb1 * var_auxb1_dn3)) + var_aaux_dn3);
        var_qsqrt_dn4 = (((var_auxb1_dn4 * var_auxb1) + (var_auxb1 * var_auxb1_dn4)) + var_aaux_dn4);
        var_qsqrt_dn5 = (((var_auxb1_dn5 * var_auxb1) + (var_auxb1 * var_auxb1_dn5)) + var_aaux_dn5);
        var_qsqrt_dn6 = (((var_auxb1_dn6 * var_auxb1) + (var_auxb1 * var_auxb1_dn6)) + var_aaux_dn6);
        var_qsqrt_dn7 = (((var_auxb1_dn7 * var_auxb1) + (var_auxb1 * var_auxb1_dn7)) + var_aaux_dn7);
        var_qsqrt_dn8 = (((var_auxb1_dn8 * var_auxb1) + (var_auxb1 * var_auxb1_dn8)) + var_aaux_dn8);

        let assign9460_e9070: f64 = if var_qsqrt < 0.0 { 1.0 } else { 0.0 };
        var_guard95 = assign9460_e9070;

        let (assign9470_e9076, assign9470_e9076_d_n3, assign9470_e9076_d_n4, assign9470_e9076_d_n5, assign9470_e9076_d_n6, assign9470_e9076_d_n7, assign9470_e9076_d_n8,) = {
    if (var_guard95 != 0.0) {
        let assign9470_e9073: f64 = (-var_qsqrt);
        let assign9470_e9074: f64 = (assign9470_e9073).sqrt();
        (assign9470_e9074, ((-var_qsqrt_dn3) / (2.0 * assign9470_e9074)), ((-var_qsqrt_dn4) / (2.0 * assign9470_e9074)), ((-var_qsqrt_dn5) / (2.0 * assign9470_e9074)), ((-var_qsqrt_dn6) / (2.0 * assign9470_e9074)), ((-var_qsqrt_dn7) / (2.0 * assign9470_e9074)), ((-var_qsqrt_dn8) / (2.0 * assign9470_e9074)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign9470_e9076;
        var_q_dn3 = assign9470_e9076_d_n3;
        var_q_dn4 = assign9470_e9076_d_n4;
        var_q_dn5 = assign9470_e9076_d_n5;
        var_q_dn6 = assign9470_e9076_d_n6;
        var_q_dn7 = assign9470_e9076_d_n7;
        var_q_dn8 = assign9470_e9076_d_n8;

        let (assign9480_e9085, assign9480_e9085_d_n3, assign9480_e9085_d_n4, assign9480_e9085_d_n5, assign9480_e9085_d_n6, assign9480_e9085_d_n7, assign9480_e9085_d_n8,) = {
    if (var_guard95 != 0.0) {
        let assign9480_e9081: f64 = (0.5 * var_q);
        let assign9480_e9082: f64 = (assign9480_e9081).sin();
        let assign9480_e9083: f64 = (1.0 / assign9480_e9082);
        (assign9480_e9083, (-(((assign9480_e9081).cos() * (0.5 * var_q_dn3)) / (assign9480_e9082 * assign9480_e9082))), (-(((assign9480_e9081).cos() * (0.5 * var_q_dn4)) / (assign9480_e9082 * assign9480_e9082))), (-(((assign9480_e9081).cos() * (0.5 * var_q_dn5)) / (assign9480_e9082 * assign9480_e9082))), (-(((assign9480_e9081).cos() * (0.5 * var_q_dn6)) / (assign9480_e9082 * assign9480_e9082))), (-(((assign9480_e9081).cos() * (0.5 * var_q_dn7)) / (assign9480_e9082 * assign9480_e9082))), (-(((assign9480_e9081).cos() * (0.5 * var_q_dn8)) / (assign9480_e9082 * assign9480_e9082))),)
    } else {
        (var_csc1, var_csc1_dn3, var_csc1_dn4, var_csc1_dn5, var_csc1_dn6, var_csc1_dn7, var_csc1_dn8,)
    }
};
        var_csc1 = assign9480_e9085;
        var_csc1_dn3 = assign9480_e9085_d_n3;
        var_csc1_dn4 = assign9480_e9085_d_n4;
        var_csc1_dn5 = assign9480_e9085_d_n5;
        var_csc1_dn6 = assign9480_e9085_d_n6;
        var_csc1_dn7 = assign9480_e9085_d_n7;
        var_csc1_dn8 = assign9480_e9085_d_n8;

        let (assign9490_e9091, assign9490_e9091_d_n3, assign9490_e9091_d_n4, assign9490_e9091_d_n5, assign9490_e9091_d_n6, assign9490_e9091_d_n7, assign9490_e9091_d_n8,) = {
    if (var_guard95 != 0.0) {
        let assign9490_e9089: f64 = (var_csc1 * var_csc1);
        (assign9490_e9089, ((var_csc1_dn3 * var_csc1) + (var_csc1 * var_csc1_dn3)), ((var_csc1_dn4 * var_csc1) + (var_csc1 * var_csc1_dn4)), ((var_csc1_dn5 * var_csc1) + (var_csc1 * var_csc1_dn5)), ((var_csc1_dn6 * var_csc1) + (var_csc1 * var_csc1_dn6)), ((var_csc1_dn7 * var_csc1) + (var_csc1 * var_csc1_dn7)), ((var_csc1_dn8 * var_csc1) + (var_csc1 * var_csc1_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign9490_e9091;
        var_t1_dn3 = assign9490_e9091_d_n3;
        var_t1_dn4 = assign9490_e9091_d_n4;
        var_t1_dn5 = assign9490_e9091_d_n5;
        var_t1_dn6 = assign9490_e9091_d_n6;
        var_t1_dn7 = assign9490_e9091_d_n7;
        var_t1_dn8 = assign9490_e9091_d_n8;

        let (assign9500_e9100, assign9500_e9100_d_n3, assign9500_e9100_d_n4, assign9500_e9100_d_n5, assign9500_e9100_d_n6, assign9500_e9100_d_n7, assign9500_e9100_d_n8,) = {
    if (var_guard95 != 0.0) {
        let assign9500_e9095: f64 = (0.5 * var_q);
        let assign9500_e9096: f64 = (assign9500_e9095).cos();
        let assign9500_e9098: f64 = (assign9500_e9096 * var_csc1);
        (assign9500_e9098, (((-(assign9500_e9095).sin() * (0.5 * var_q_dn3)) * var_csc1) + (assign9500_e9096 * var_csc1_dn3)), (((-(assign9500_e9095).sin() * (0.5 * var_q_dn4)) * var_csc1) + (assign9500_e9096 * var_csc1_dn4)), (((-(assign9500_e9095).sin() * (0.5 * var_q_dn5)) * var_csc1) + (assign9500_e9096 * var_csc1_dn5)), (((-(assign9500_e9095).sin() * (0.5 * var_q_dn6)) * var_csc1) + (assign9500_e9096 * var_csc1_dn6)), (((-(assign9500_e9095).sin() * (0.5 * var_q_dn7)) * var_csc1) + (assign9500_e9096 * var_csc1_dn7)), (((-(assign9500_e9095).sin() * (0.5 * var_q_dn8)) * var_csc1) + (assign9500_e9096 * var_csc1_dn8)),)
    } else {
        (var_coth1, var_coth1_dn3, var_coth1_dn4, var_coth1_dn5, var_coth1_dn6, var_coth1_dn7, var_coth1_dn8,)
    }
};
        var_coth1 = assign9500_e9100;
        var_coth1_dn3 = assign9500_e9100_d_n3;
        var_coth1_dn4 = assign9500_e9100_d_n4;
        var_coth1_dn5 = assign9500_e9100_d_n5;
        var_coth1_dn6 = assign9500_e9100_d_n6;
        var_coth1_dn7 = assign9500_e9100_d_n7;
        var_coth1_dn8 = assign9500_e9100_d_n8;

        let (assign9510_e9109, assign9510_e9109_d_n3, assign9510_e9109_d_n4, assign9510_e9109_d_n5, assign9510_e9109_d_n6, assign9510_e9109_d_n7, assign9510_e9109_d_n8,) = {
    if (var_guard95 != 0.0) {
        let assign9510_e9103: f64 = (-0.5);
        let assign9510_e9105: f64 = (assign9510_e9103 * var_coth1);
        let assign9510_e9107: f64 = (assign9510_e9105 / var_q);
        (assign9510_e9107, ((((assign9510_e9103 * var_coth1_dn3) * var_q) - (assign9510_e9105 * var_q_dn3)) / (var_q * var_q)), ((((assign9510_e9103 * var_coth1_dn4) * var_q) - (assign9510_e9105 * var_q_dn4)) / (var_q * var_q)), ((((assign9510_e9103 * var_coth1_dn5) * var_q) - (assign9510_e9105 * var_q_dn5)) / (var_q * var_q)), ((((assign9510_e9103 * var_coth1_dn6) * var_q) - (assign9510_e9105 * var_q_dn6)) / (var_q * var_q)), ((((assign9510_e9103 * var_coth1_dn7) * var_q) - (assign9510_e9105 * var_q_dn7)) / (var_q * var_q)), ((((assign9510_e9103 * var_coth1_dn8) * var_q) - (assign9510_e9105 * var_q_dn8)) / (var_q * var_q)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign9510_e9109;
        var_t0_dn3 = assign9510_e9109_d_n3;
        var_t0_dn4 = assign9510_e9109_d_n4;
        var_t0_dn5 = assign9510_e9109_d_n5;
        var_t0_dn6 = assign9510_e9109_d_n6;
        var_t0_dn7 = assign9510_e9109_d_n7;
        var_t0_dn8 = assign9510_e9109_d_n8;

        let (assign9520_e9117, assign9520_e9117_d_n3, assign9520_e9117_d_n4, assign9520_e9117_d_n5, assign9520_e9117_d_n6, assign9520_e9117_d_n7, assign9520_e9117_d_n8,) = {
    if (var_guard95 != 0.0) {
        let assign9520_e9113: f64 = (0.25 * var_t1);
        let assign9520_e9115: f64 = (assign9520_e9113 + var_t0);
        (assign9520_e9115, ((0.25 * var_t1_dn3) + var_t0_dn3), ((0.25 * var_t1_dn4) + var_t0_dn4), ((0.25 * var_t1_dn5) + var_t0_dn5), ((0.25 * var_t1_dn6) + var_t0_dn6), ((0.25 * var_t1_dn7) + var_t0_dn7), ((0.25 * var_t1_dn8) + var_t0_dn8),)
    } else {
        (var_dqcothqdqsqrt, var_dqcothqdqsqrt_dn3, var_dqcothqdqsqrt_dn4, var_dqcothqdqsqrt_dn5, var_dqcothqdqsqrt_dn6, var_dqcothqdqsqrt_dn7, var_dqcothqdqsqrt_dn8,)
    }
};
        var_dqcothqdqsqrt = assign9520_e9117;
        var_dqcothqdqsqrt_dn3 = assign9520_e9117_d_n3;
        var_dqcothqdqsqrt_dn4 = assign9520_e9117_d_n4;
        var_dqcothqdqsqrt_dn5 = assign9520_e9117_d_n5;
        var_dqcothqdqsqrt_dn6 = assign9520_e9117_d_n6;
        var_dqcothqdqsqrt_dn7 = assign9520_e9117_d_n7;
        var_dqcothqdqsqrt_dn8 = assign9520_e9117_d_n8;

        let (assign9530_e9123, assign9530_e9123_d_n3, assign9530_e9123_d_n4, assign9530_e9123_d_n5, assign9530_e9123_d_n6, assign9530_e9123_d_n7, assign9530_e9123_d_n8,) = {
    if (var_guard95 == 0.0) {
        let assign9530_e9121: f64 = (var_qsqrt).sqrt();
        (assign9530_e9121, (var_qsqrt_dn3 / (2.0 * assign9530_e9121)), (var_qsqrt_dn4 / (2.0 * assign9530_e9121)), (var_qsqrt_dn5 / (2.0 * assign9530_e9121)), (var_qsqrt_dn6 / (2.0 * assign9530_e9121)), (var_qsqrt_dn7 / (2.0 * assign9530_e9121)), (var_qsqrt_dn8 / (2.0 * assign9530_e9121)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign9530_e9123;
        var_q_dn3 = assign9530_e9123_d_n3;
        var_q_dn4 = assign9530_e9123_d_n4;
        var_q_dn5 = assign9530_e9123_d_n5;
        var_q_dn6 = assign9530_e9123_d_n6;
        var_q_dn7 = assign9530_e9123_d_n7;
        var_q_dn8 = assign9530_e9123_d_n8;

        let (assign9540_e9133, assign9540_e9133_d_n3, assign9540_e9133_d_n4, assign9540_e9133_d_n5, assign9540_e9133_d_n6, assign9540_e9133_d_n7, assign9540_e9133_d_n8,) = {
    if (var_guard95 == 0.0) {
        let assign9540_e9129: f64 = (0.5 * var_q);
        let assign9540_e9130: f64 = (assign9540_e9129).sinh();
        let assign9540_e9131: f64 = (1.0 / assign9540_e9130);
        (assign9540_e9131, (-(((assign9540_e9129).cosh() * (0.5 * var_q_dn3)) / (assign9540_e9130 * assign9540_e9130))), (-(((assign9540_e9129).cosh() * (0.5 * var_q_dn4)) / (assign9540_e9130 * assign9540_e9130))), (-(((assign9540_e9129).cosh() * (0.5 * var_q_dn5)) / (assign9540_e9130 * assign9540_e9130))), (-(((assign9540_e9129).cosh() * (0.5 * var_q_dn6)) / (assign9540_e9130 * assign9540_e9130))), (-(((assign9540_e9129).cosh() * (0.5 * var_q_dn7)) / (assign9540_e9130 * assign9540_e9130))), (-(((assign9540_e9129).cosh() * (0.5 * var_q_dn8)) / (assign9540_e9130 * assign9540_e9130))),)
    } else {
        (var_csc1, var_csc1_dn3, var_csc1_dn4, var_csc1_dn5, var_csc1_dn6, var_csc1_dn7, var_csc1_dn8,)
    }
};
        var_csc1 = assign9540_e9133;
        var_csc1_dn3 = assign9540_e9133_d_n3;
        var_csc1_dn4 = assign9540_e9133_d_n4;
        var_csc1_dn5 = assign9540_e9133_d_n5;
        var_csc1_dn6 = assign9540_e9133_d_n6;
        var_csc1_dn7 = assign9540_e9133_d_n7;
        var_csc1_dn8 = assign9540_e9133_d_n8;

        let (assign9550_e9140, assign9550_e9140_d_n3, assign9550_e9140_d_n4, assign9550_e9140_d_n5, assign9550_e9140_d_n6, assign9550_e9140_d_n7, assign9550_e9140_d_n8,) = {
    if (var_guard95 == 0.0) {
        let assign9550_e9138: f64 = (var_csc1 * var_csc1);
        (assign9550_e9138, ((var_csc1_dn3 * var_csc1) + (var_csc1 * var_csc1_dn3)), ((var_csc1_dn4 * var_csc1) + (var_csc1 * var_csc1_dn4)), ((var_csc1_dn5 * var_csc1) + (var_csc1 * var_csc1_dn5)), ((var_csc1_dn6 * var_csc1) + (var_csc1 * var_csc1_dn6)), ((var_csc1_dn7 * var_csc1) + (var_csc1 * var_csc1_dn7)), ((var_csc1_dn8 * var_csc1) + (var_csc1 * var_csc1_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign9550_e9140;
        var_t1_dn3 = assign9550_e9140_d_n3;
        var_t1_dn4 = assign9550_e9140_d_n4;
        var_t1_dn5 = assign9550_e9140_d_n5;
        var_t1_dn6 = assign9550_e9140_d_n6;
        var_t1_dn7 = assign9550_e9140_d_n7;
        var_t1_dn8 = assign9550_e9140_d_n8;

        let (assign9560_e9148, assign9560_e9148_d_n3, assign9560_e9148_d_n4, assign9560_e9148_d_n5, assign9560_e9148_d_n6, assign9560_e9148_d_n7, assign9560_e9148_d_n8,) = {
    if (var_guard95 == 0.0) {
        let assign9560_e9145: f64 = (1.0 + var_t1);
        let assign9560_e9146: f64 = (assign9560_e9145).sqrt();
        (assign9560_e9146, (var_t1_dn3 / (2.0 * assign9560_e9146)), (var_t1_dn4 / (2.0 * assign9560_e9146)), (var_t1_dn5 / (2.0 * assign9560_e9146)), (var_t1_dn6 / (2.0 * assign9560_e9146)), (var_t1_dn7 / (2.0 * assign9560_e9146)), (var_t1_dn8 / (2.0 * assign9560_e9146)),)
    } else {
        (var_coth1, var_coth1_dn3, var_coth1_dn4, var_coth1_dn5, var_coth1_dn6, var_coth1_dn7, var_coth1_dn8,)
    }
};
        var_coth1 = assign9560_e9148;
        var_coth1_dn3 = assign9560_e9148_d_n3;
        var_coth1_dn4 = assign9560_e9148_d_n4;
        var_coth1_dn5 = assign9560_e9148_d_n5;
        var_coth1_dn6 = assign9560_e9148_d_n6;
        var_coth1_dn7 = assign9560_e9148_d_n7;
        var_coth1_dn8 = assign9560_e9148_d_n8;

        let (assign9570_e9157, assign9570_e9157_d_n3, assign9570_e9157_d_n4, assign9570_e9157_d_n5, assign9570_e9157_d_n6, assign9570_e9157_d_n7, assign9570_e9157_d_n8,) = {
    if (var_guard95 == 0.0) {
        let assign9570_e9153: f64 = (0.5 * var_coth1);
        let assign9570_e9155: f64 = (assign9570_e9153 / var_q);
        (assign9570_e9155, ((((0.5 * var_coth1_dn3) * var_q) - (assign9570_e9153 * var_q_dn3)) / (var_q * var_q)), ((((0.5 * var_coth1_dn4) * var_q) - (assign9570_e9153 * var_q_dn4)) / (var_q * var_q)), ((((0.5 * var_coth1_dn5) * var_q) - (assign9570_e9153 * var_q_dn5)) / (var_q * var_q)), ((((0.5 * var_coth1_dn6) * var_q) - (assign9570_e9153 * var_q_dn6)) / (var_q * var_q)), ((((0.5 * var_coth1_dn7) * var_q) - (assign9570_e9153 * var_q_dn7)) / (var_q * var_q)), ((((0.5 * var_coth1_dn8) * var_q) - (assign9570_e9153 * var_q_dn8)) / (var_q * var_q)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign9570_e9157;
        var_t0_dn3 = assign9570_e9157_d_n3;
        var_t0_dn4 = assign9570_e9157_d_n4;
        var_t0_dn5 = assign9570_e9157_d_n5;
        var_t0_dn6 = assign9570_e9157_d_n6;
        var_t0_dn7 = assign9570_e9157_d_n7;
        var_t0_dn8 = assign9570_e9157_d_n8;

        let (assign9580_e9167, assign9580_e9167_d_n3, assign9580_e9167_d_n4, assign9580_e9167_d_n5, assign9580_e9167_d_n6, assign9580_e9167_d_n7, assign9580_e9167_d_n8,) = {
    if (var_guard95 == 0.0) {
        let assign9580_e9161: f64 = (-0.25);
        let assign9580_e9163: f64 = (assign9580_e9161 * var_t1);
        let assign9580_e9165: f64 = (assign9580_e9163 + var_t0);
        (assign9580_e9165, ((assign9580_e9161 * var_t1_dn3) + var_t0_dn3), ((assign9580_e9161 * var_t1_dn4) + var_t0_dn4), ((assign9580_e9161 * var_t1_dn5) + var_t0_dn5), ((assign9580_e9161 * var_t1_dn6) + var_t0_dn6), ((assign9580_e9161 * var_t1_dn7) + var_t0_dn7), ((assign9580_e9161 * var_t1_dn8) + var_t0_dn8),)
    } else {
        (var_dqcothqdqsqrt, var_dqcothqdqsqrt_dn3, var_dqcothqdqsqrt_dn4, var_dqcothqdqsqrt_dn5, var_dqcothqdqsqrt_dn6, var_dqcothqdqsqrt_dn7, var_dqcothqdqsqrt_dn8,)
    }
};
        var_dqcothqdqsqrt = assign9580_e9167;
        var_dqcothqdqsqrt_dn3 = assign9580_e9167_d_n3;
        var_dqcothqdqsqrt_dn4 = assign9580_e9167_d_n4;
        var_dqcothqdqsqrt_dn5 = assign9580_e9167_d_n5;
        var_dqcothqdqsqrt_dn6 = assign9580_e9167_d_n6;
        var_dqcothqdqsqrt_dn7 = assign9580_e9167_d_n7;
        var_dqcothqdqsqrt_dn8 = assign9580_e9167_d_n8;

        *var_aaux_slot = var_aaux;
        *var_aaux_dn3_slot = var_aaux_dn3;
        *var_aaux_dn4_slot = var_aaux_dn4;
        *var_aaux_dn5_slot = var_aaux_dn5;
        *var_aaux_dn6_slot = var_aaux_dn6;
        *var_aaux_dn7_slot = var_aaux_dn7;
        *var_aaux_dn8_slot = var_aaux_dn8;
        *var_auxb1_slot = var_auxb1;
        *var_auxb1_dn3_slot = var_auxb1_dn3;
        *var_auxb1_dn4_slot = var_auxb1_dn4;
        *var_auxb1_dn5_slot = var_auxb1_dn5;
        *var_auxb1_dn6_slot = var_auxb1_dn6;
        *var_auxb1_dn7_slot = var_auxb1_dn7;
        *var_auxb1_dn8_slot = var_auxb1_dn8;
        *var_coth1_slot = var_coth1;
        *var_coth1_dn3_slot = var_coth1_dn3;
        *var_coth1_dn4_slot = var_coth1_dn4;
        *var_coth1_dn5_slot = var_coth1_dn5;
        *var_coth1_dn6_slot = var_coth1_dn6;
        *var_coth1_dn7_slot = var_coth1_dn7;
        *var_coth1_dn8_slot = var_coth1_dn8;
        *var_csc1_slot = var_csc1;
        *var_csc1_dn3_slot = var_csc1_dn3;
        *var_csc1_dn4_slot = var_csc1_dn4;
        *var_csc1_dn5_slot = var_csc1_dn5;
        *var_csc1_dn6_slot = var_csc1_dn6;
        *var_csc1_dn7_slot = var_csc1_dn7;
        *var_csc1_dn8_slot = var_csc1_dn8;
        *var_delta_slot = var_delta;
        *var_delta_dn3_slot = var_delta_dn3;
        *var_delta_dn4_slot = var_delta_dn4;
        *var_delta_dn5_slot = var_delta_dn5;
        *var_delta_dn6_slot = var_delta_dn6;
        *var_delta_dn7_slot = var_delta_dn7;
        *var_delta_dn8_slot = var_delta_dn8;
        *var_df_slot = var_df;
        *var_df_dn3_slot = var_df_dn3;
        *var_df_dn4_slot = var_df_dn4;
        *var_df_dn5_slot = var_df_dn5;
        *var_df_dn6_slot = var_df_dn6;
        *var_df_dn7_slot = var_df_dn7;
        *var_df_dn8_slot = var_df_dn8;
        *var_dlogsinhqsqdqsqrt_slot = var_dlogsinhqsqdqsqrt;
        *var_dlogsinhqsqdqsqrt_dn3_slot = var_dlogsinhqsqdqsqrt_dn3;
        *var_dlogsinhqsqdqsqrt_dn4_slot = var_dlogsinhqsqdqsqrt_dn4;
        *var_dlogsinhqsqdqsqrt_dn5_slot = var_dlogsinhqsqdqsqrt_dn5;
        *var_dlogsinhqsqdqsqrt_dn6_slot = var_dlogsinhqsqdqsqrt_dn6;
        *var_dlogsinhqsqdqsqrt_dn7_slot = var_dlogsinhqsqdqsqrt_dn7;
        *var_dlogsinhqsqdqsqrt_dn8_slot = var_dlogsinhqsqdqsqrt_dn8;
        *var_dq2_slot = var_dq2;
        *var_dq2_dn3_slot = var_dq2_dn3;
        *var_dq2_dn4_slot = var_dq2_dn4;
        *var_dq2_dn5_slot = var_dq2_dn5;
        *var_dq2_dn6_slot = var_dq2_dn6;
        *var_dq2_dn7_slot = var_dq2_dn7;
        *var_dq2_dn8_slot = var_dq2_dn8;
        *var_dqcoth_slot = var_dqcoth;
        *var_dqcoth_dn3_slot = var_dqcoth_dn3;
        *var_dqcoth_dn4_slot = var_dqcoth_dn4;
        *var_dqcoth_dn5_slot = var_dqcoth_dn5;
        *var_dqcoth_dn6_slot = var_dqcoth_dn6;
        *var_dqcoth_dn7_slot = var_dqcoth_dn7;
        *var_dqcoth_dn8_slot = var_dqcoth_dn8;
        *var_dqcothqdqsqrt_slot = var_dqcothqdqsqrt;
        *var_dqcothqdqsqrt_dn3_slot = var_dqcothqdqsqrt_dn3;
        *var_dqcothqdqsqrt_dn4_slot = var_dqcothqdqsqrt_dn4;
        *var_dqcothqdqsqrt_dn5_slot = var_dqcothqdqsqrt_dn5;
        *var_dqcothqdqsqrt_dn6_slot = var_dqcothqdqsqrt_dn6;
        *var_dqcothqdqsqrt_dn7_slot = var_dqcothqdqsqrt_dn7;
        *var_dqcothqdqsqrt_dn8_slot = var_dqcothqdqsqrt_dn8;
        *var_dqsqrt_slot = var_dqsqrt;
        *var_dqsqrt_dn3_slot = var_dqsqrt_dn3;
        *var_dqsqrt_dn4_slot = var_dqsqrt_dn4;
        *var_dqsqrt_dn5_slot = var_dqsqrt_dn5;
        *var_dqsqrt_dn6_slot = var_dqsqrt_dn6;
        *var_dqsqrt_dn7_slot = var_dqsqrt_dn7;
        *var_dqsqrt_dn8_slot = var_dqsqrt_dn8;
        *var_f_slot = var_f;
        *var_f_dn3_slot = var_f_dn3;
        *var_f_dn4_slot = var_f_dn4;
        *var_f_dn5_slot = var_f_dn5;
        *var_f_dn6_slot = var_f_dn6;
        *var_f_dn7_slot = var_f_dn7;
        *var_f_dn8_slot = var_f_dn8;
        *var_guard95_slot = var_guard95;
        *var_phi1_slot = var_phi1;
        *var_phi1_dn3_slot = var_phi1_dn3;
        *var_phi1_dn4_slot = var_phi1_dn4;
        *var_phi1_dn5_slot = var_phi1_dn5;
        *var_phi1_dn6_slot = var_phi1_dn6;
        *var_phi1_dn7_slot = var_phi1_dn7;
        *var_phi1_dn8_slot = var_phi1_dn8;
        *var_q_slot = var_q;
        *var_q1_slot = var_q1;
        *var_q1_dn3_slot = var_q1_dn3;
        *var_q1_dn4_slot = var_q1_dn4;
        *var_q1_dn5_slot = var_q1_dn5;
        *var_q1_dn6_slot = var_q1_dn6;
        *var_q1_dn7_slot = var_q1_dn7;
        *var_q1_dn8_slot = var_q1_dn8;
        *var_q2_slot = var_q2;
        *var_q2_dn3_slot = var_q2_dn3;
        *var_q2_dn4_slot = var_q2_dn4;
        *var_q2_dn5_slot = var_q2_dn5;
        *var_q2_dn6_slot = var_q2_dn6;
        *var_q2_dn7_slot = var_q2_dn7;
        *var_q2_dn8_slot = var_q2_dn8;
        *var_q_dn3_slot = var_q_dn3;
        *var_q_dn4_slot = var_q_dn4;
        *var_q_dn5_slot = var_q_dn5;
        *var_q_dn6_slot = var_q_dn6;
        *var_q_dn7_slot = var_q_dn7;
        *var_q_dn8_slot = var_q_dn8;
        *var_qcoth_slot = var_qcoth;
        *var_qcoth_dn3_slot = var_qcoth_dn3;
        *var_qcoth_dn4_slot = var_qcoth_dn4;
        *var_qcoth_dn5_slot = var_qcoth_dn5;
        *var_qcoth_dn6_slot = var_qcoth_dn6;
        *var_qcoth_dn7_slot = var_qcoth_dn7;
        *var_qcoth_dn8_slot = var_qcoth_dn8;
        *var_qsqrt_slot = var_qsqrt;
        *var_qsqrt_dn3_slot = var_qsqrt_dn3;
        *var_qsqrt_dn4_slot = var_qsqrt_dn4;
        *var_qsqrt_dn5_slot = var_qsqrt_dn5;
        *var_qsqrt_dn6_slot = var_qsqrt_dn6;
        *var_qsqrt_dn7_slot = var_qsqrt_dn7;
        *var_qsqrt_dn8_slot = var_qsqrt_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
    }

    pub(super) fn stamp_transient_block_20(
        var_a0: f64,
        var_a0_dn3: f64,
        var_a0_dn4: f64,
        var_a0_dn5: f64,
        var_a0_dn6: f64,
        var_a0_dn7: f64,
        var_a0_dn8: f64,
        var_k1: f64,
        var_k2: f64,
        var_xg1: f64,
        var_xg1_dn3: f64,
        var_xg1_dn4: f64,
        var_xg1_dn5: f64,
        var_xg1_dn6: f64,
        var_xg1_dn7: f64,
        var_xg1_dn8: f64,
        var_xg2: f64,
        var_xg2_dn3: f64,
        var_xg2_dn4: f64,
        var_xg2_dn5: f64,
        var_xg2_dn6: f64,
        var_xg2_dn7: f64,
        var_xg2_dn8: f64,
        var_aaux_slot: &mut f64,
        var_aaux_dn3_slot: &mut f64,
        var_aaux_dn4_slot: &mut f64,
        var_aaux_dn5_slot: &mut f64,
        var_aaux_dn6_slot: &mut f64,
        var_aaux_dn7_slot: &mut f64,
        var_aaux_dn8_slot: &mut f64,
        var_auxb1_slot: &mut f64,
        var_auxb1_dn3_slot: &mut f64,
        var_auxb1_dn4_slot: &mut f64,
        var_auxb1_dn5_slot: &mut f64,
        var_auxb1_dn6_slot: &mut f64,
        var_auxb1_dn7_slot: &mut f64,
        var_auxb1_dn8_slot: &mut f64,
        var_coth1_slot: &mut f64,
        var_coth1_dn3_slot: &mut f64,
        var_coth1_dn4_slot: &mut f64,
        var_coth1_dn5_slot: &mut f64,
        var_coth1_dn6_slot: &mut f64,
        var_coth1_dn7_slot: &mut f64,
        var_coth1_dn8_slot: &mut f64,
        var_csc1_slot: &mut f64,
        var_csc1_dn3_slot: &mut f64,
        var_csc1_dn4_slot: &mut f64,
        var_csc1_dn5_slot: &mut f64,
        var_csc1_dn6_slot: &mut f64,
        var_csc1_dn7_slot: &mut f64,
        var_csc1_dn8_slot: &mut f64,
        var_delta_slot: &mut f64,
        var_delta_dn3_slot: &mut f64,
        var_delta_dn4_slot: &mut f64,
        var_delta_dn5_slot: &mut f64,
        var_delta_dn6_slot: &mut f64,
        var_delta_dn7_slot: &mut f64,
        var_delta_dn8_slot: &mut f64,
        var_df_slot: &mut f64,
        var_df_dn3_slot: &mut f64,
        var_df_dn4_slot: &mut f64,
        var_df_dn5_slot: &mut f64,
        var_df_dn6_slot: &mut f64,
        var_df_dn7_slot: &mut f64,
        var_df_dn8_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn3_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn4_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn5_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn6_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn7_slot: &mut f64,
        var_dlogsinhqsqdqsqrt_dn8_slot: &mut f64,
        var_dq2_slot: &mut f64,
        var_dq2_dn3_slot: &mut f64,
        var_dq2_dn4_slot: &mut f64,
        var_dq2_dn5_slot: &mut f64,
        var_dq2_dn6_slot: &mut f64,
        var_dq2_dn7_slot: &mut f64,
        var_dq2_dn8_slot: &mut f64,
        var_dqcoth_slot: &mut f64,
        var_dqcoth_dn3_slot: &mut f64,
        var_dqcoth_dn4_slot: &mut f64,
        var_dqcoth_dn5_slot: &mut f64,
        var_dqcoth_dn6_slot: &mut f64,
        var_dqcoth_dn7_slot: &mut f64,
        var_dqcoth_dn8_slot: &mut f64,
        var_dqcothqdqsqrt_slot: &mut f64,
        var_dqcothqdqsqrt_dn3_slot: &mut f64,
        var_dqcothqdqsqrt_dn4_slot: &mut f64,
        var_dqcothqdqsqrt_dn5_slot: &mut f64,
        var_dqcothqdqsqrt_dn6_slot: &mut f64,
        var_dqcothqdqsqrt_dn7_slot: &mut f64,
        var_dqcothqdqsqrt_dn8_slot: &mut f64,
        var_dqsqrt_slot: &mut f64,
        var_dqsqrt_dn3_slot: &mut f64,
        var_dqsqrt_dn4_slot: &mut f64,
        var_dqsqrt_dn5_slot: &mut f64,
        var_dqsqrt_dn6_slot: &mut f64,
        var_dqsqrt_dn7_slot: &mut f64,
        var_dqsqrt_dn8_slot: &mut f64,
        var_f_slot: &mut f64,
        var_f_dn3_slot: &mut f64,
        var_f_dn4_slot: &mut f64,
        var_f_dn5_slot: &mut f64,
        var_f_dn6_slot: &mut f64,
        var_f_dn7_slot: &mut f64,
        var_f_dn8_slot: &mut f64,
        var_guard96_slot: &mut f64,
        var_phi1_slot: &mut f64,
        var_phi1_dn3_slot: &mut f64,
        var_phi1_dn4_slot: &mut f64,
        var_phi1_dn5_slot: &mut f64,
        var_phi1_dn6_slot: &mut f64,
        var_phi1_dn7_slot: &mut f64,
        var_phi1_dn8_slot: &mut f64,
        var_q_slot: &mut f64,
        var_q1_slot: &mut f64,
        var_q1_dn3_slot: &mut f64,
        var_q1_dn4_slot: &mut f64,
        var_q1_dn5_slot: &mut f64,
        var_q1_dn6_slot: &mut f64,
        var_q1_dn7_slot: &mut f64,
        var_q1_dn8_slot: &mut f64,
        var_q2_slot: &mut f64,
        var_q2_dn3_slot: &mut f64,
        var_q2_dn4_slot: &mut f64,
        var_q2_dn5_slot: &mut f64,
        var_q2_dn6_slot: &mut f64,
        var_q2_dn7_slot: &mut f64,
        var_q2_dn8_slot: &mut f64,
        var_q_dn3_slot: &mut f64,
        var_q_dn4_slot: &mut f64,
        var_q_dn5_slot: &mut f64,
        var_q_dn6_slot: &mut f64,
        var_q_dn7_slot: &mut f64,
        var_q_dn8_slot: &mut f64,
        var_qcoth_slot: &mut f64,
        var_qcoth_dn3_slot: &mut f64,
        var_qcoth_dn4_slot: &mut f64,
        var_qcoth_dn5_slot: &mut f64,
        var_qcoth_dn6_slot: &mut f64,
        var_qcoth_dn7_slot: &mut f64,
        var_qcoth_dn8_slot: &mut f64,
        var_qsqrt_slot: &mut f64,
        var_qsqrt_dn3_slot: &mut f64,
        var_qsqrt_dn4_slot: &mut f64,
        var_qsqrt_dn5_slot: &mut f64,
        var_qsqrt_dn6_slot: &mut f64,
        var_qsqrt_dn7_slot: &mut f64,
        var_qsqrt_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
    ) {
        let mut var_aaux: f64 = *var_aaux_slot;
        let mut var_aaux_dn3: f64 = *var_aaux_dn3_slot;
        let mut var_aaux_dn4: f64 = *var_aaux_dn4_slot;
        let mut var_aaux_dn5: f64 = *var_aaux_dn5_slot;
        let mut var_aaux_dn6: f64 = *var_aaux_dn6_slot;
        let mut var_aaux_dn7: f64 = *var_aaux_dn7_slot;
        let mut var_aaux_dn8: f64 = *var_aaux_dn8_slot;
        let mut var_auxb1: f64 = *var_auxb1_slot;
        let mut var_auxb1_dn3: f64 = *var_auxb1_dn3_slot;
        let mut var_auxb1_dn4: f64 = *var_auxb1_dn4_slot;
        let mut var_auxb1_dn5: f64 = *var_auxb1_dn5_slot;
        let mut var_auxb1_dn6: f64 = *var_auxb1_dn6_slot;
        let mut var_auxb1_dn7: f64 = *var_auxb1_dn7_slot;
        let mut var_auxb1_dn8: f64 = *var_auxb1_dn8_slot;
        let mut var_coth1: f64 = *var_coth1_slot;
        let mut var_coth1_dn3: f64 = *var_coth1_dn3_slot;
        let mut var_coth1_dn4: f64 = *var_coth1_dn4_slot;
        let mut var_coth1_dn5: f64 = *var_coth1_dn5_slot;
        let mut var_coth1_dn6: f64 = *var_coth1_dn6_slot;
        let mut var_coth1_dn7: f64 = *var_coth1_dn7_slot;
        let mut var_coth1_dn8: f64 = *var_coth1_dn8_slot;
        let mut var_csc1: f64 = *var_csc1_slot;
        let mut var_csc1_dn3: f64 = *var_csc1_dn3_slot;
        let mut var_csc1_dn4: f64 = *var_csc1_dn4_slot;
        let mut var_csc1_dn5: f64 = *var_csc1_dn5_slot;
        let mut var_csc1_dn6: f64 = *var_csc1_dn6_slot;
        let mut var_csc1_dn7: f64 = *var_csc1_dn7_slot;
        let mut var_csc1_dn8: f64 = *var_csc1_dn8_slot;
        let mut var_delta: f64 = *var_delta_slot;
        let mut var_delta_dn3: f64 = *var_delta_dn3_slot;
        let mut var_delta_dn4: f64 = *var_delta_dn4_slot;
        let mut var_delta_dn5: f64 = *var_delta_dn5_slot;
        let mut var_delta_dn6: f64 = *var_delta_dn6_slot;
        let mut var_delta_dn7: f64 = *var_delta_dn7_slot;
        let mut var_delta_dn8: f64 = *var_delta_dn8_slot;
        let mut var_df: f64 = *var_df_slot;
        let mut var_df_dn3: f64 = *var_df_dn3_slot;
        let mut var_df_dn4: f64 = *var_df_dn4_slot;
        let mut var_df_dn5: f64 = *var_df_dn5_slot;
        let mut var_df_dn6: f64 = *var_df_dn6_slot;
        let mut var_df_dn7: f64 = *var_df_dn7_slot;
        let mut var_df_dn8: f64 = *var_df_dn8_slot;
        let mut var_dlogsinhqsqdqsqrt: f64 = *var_dlogsinhqsqdqsqrt_slot;
        let mut var_dlogsinhqsqdqsqrt_dn3: f64 = *var_dlogsinhqsqdqsqrt_dn3_slot;
        let mut var_dlogsinhqsqdqsqrt_dn4: f64 = *var_dlogsinhqsqdqsqrt_dn4_slot;
        let mut var_dlogsinhqsqdqsqrt_dn5: f64 = *var_dlogsinhqsqdqsqrt_dn5_slot;
        let mut var_dlogsinhqsqdqsqrt_dn6: f64 = *var_dlogsinhqsqdqsqrt_dn6_slot;
        let mut var_dlogsinhqsqdqsqrt_dn7: f64 = *var_dlogsinhqsqdqsqrt_dn7_slot;
        let mut var_dlogsinhqsqdqsqrt_dn8: f64 = *var_dlogsinhqsqdqsqrt_dn8_slot;
        let mut var_dq2: f64 = *var_dq2_slot;
        let mut var_dq2_dn3: f64 = *var_dq2_dn3_slot;
        let mut var_dq2_dn4: f64 = *var_dq2_dn4_slot;
        let mut var_dq2_dn5: f64 = *var_dq2_dn5_slot;
        let mut var_dq2_dn6: f64 = *var_dq2_dn6_slot;
        let mut var_dq2_dn7: f64 = *var_dq2_dn7_slot;
        let mut var_dq2_dn8: f64 = *var_dq2_dn8_slot;
        let mut var_dqcoth: f64 = *var_dqcoth_slot;
        let mut var_dqcoth_dn3: f64 = *var_dqcoth_dn3_slot;
        let mut var_dqcoth_dn4: f64 = *var_dqcoth_dn4_slot;
        let mut var_dqcoth_dn5: f64 = *var_dqcoth_dn5_slot;
        let mut var_dqcoth_dn6: f64 = *var_dqcoth_dn6_slot;
        let mut var_dqcoth_dn7: f64 = *var_dqcoth_dn7_slot;
        let mut var_dqcoth_dn8: f64 = *var_dqcoth_dn8_slot;
        let mut var_dqcothqdqsqrt: f64 = *var_dqcothqdqsqrt_slot;
        let mut var_dqcothqdqsqrt_dn3: f64 = *var_dqcothqdqsqrt_dn3_slot;
        let mut var_dqcothqdqsqrt_dn4: f64 = *var_dqcothqdqsqrt_dn4_slot;
        let mut var_dqcothqdqsqrt_dn5: f64 = *var_dqcothqdqsqrt_dn5_slot;
        let mut var_dqcothqdqsqrt_dn6: f64 = *var_dqcothqdqsqrt_dn6_slot;
        let mut var_dqcothqdqsqrt_dn7: f64 = *var_dqcothqdqsqrt_dn7_slot;
        let mut var_dqcothqdqsqrt_dn8: f64 = *var_dqcothqdqsqrt_dn8_slot;
        let mut var_dqsqrt: f64 = *var_dqsqrt_slot;
        let mut var_dqsqrt_dn3: f64 = *var_dqsqrt_dn3_slot;
        let mut var_dqsqrt_dn4: f64 = *var_dqsqrt_dn4_slot;
        let mut var_dqsqrt_dn5: f64 = *var_dqsqrt_dn5_slot;
        let mut var_dqsqrt_dn6: f64 = *var_dqsqrt_dn6_slot;
        let mut var_dqsqrt_dn7: f64 = *var_dqsqrt_dn7_slot;
        let mut var_dqsqrt_dn8: f64 = *var_dqsqrt_dn8_slot;
        let mut var_f: f64 = *var_f_slot;
        let mut var_f_dn3: f64 = *var_f_dn3_slot;
        let mut var_f_dn4: f64 = *var_f_dn4_slot;
        let mut var_f_dn5: f64 = *var_f_dn5_slot;
        let mut var_f_dn6: f64 = *var_f_dn6_slot;
        let mut var_f_dn7: f64 = *var_f_dn7_slot;
        let mut var_f_dn8: f64 = *var_f_dn8_slot;
        let mut var_guard96: f64 = *var_guard96_slot;
        let mut var_phi1: f64 = *var_phi1_slot;
        let mut var_phi1_dn3: f64 = *var_phi1_dn3_slot;
        let mut var_phi1_dn4: f64 = *var_phi1_dn4_slot;
        let mut var_phi1_dn5: f64 = *var_phi1_dn5_slot;
        let mut var_phi1_dn6: f64 = *var_phi1_dn6_slot;
        let mut var_phi1_dn7: f64 = *var_phi1_dn7_slot;
        let mut var_phi1_dn8: f64 = *var_phi1_dn8_slot;
        let mut var_q: f64 = *var_q_slot;
        let mut var_q1: f64 = *var_q1_slot;
        let mut var_q1_dn3: f64 = *var_q1_dn3_slot;
        let mut var_q1_dn4: f64 = *var_q1_dn4_slot;
        let mut var_q1_dn5: f64 = *var_q1_dn5_slot;
        let mut var_q1_dn6: f64 = *var_q1_dn6_slot;
        let mut var_q1_dn7: f64 = *var_q1_dn7_slot;
        let mut var_q1_dn8: f64 = *var_q1_dn8_slot;
        let mut var_q2: f64 = *var_q2_slot;
        let mut var_q2_dn3: f64 = *var_q2_dn3_slot;
        let mut var_q2_dn4: f64 = *var_q2_dn4_slot;
        let mut var_q2_dn5: f64 = *var_q2_dn5_slot;
        let mut var_q2_dn6: f64 = *var_q2_dn6_slot;
        let mut var_q2_dn7: f64 = *var_q2_dn7_slot;
        let mut var_q2_dn8: f64 = *var_q2_dn8_slot;
        let mut var_q_dn3: f64 = *var_q_dn3_slot;
        let mut var_q_dn4: f64 = *var_q_dn4_slot;
        let mut var_q_dn5: f64 = *var_q_dn5_slot;
        let mut var_q_dn6: f64 = *var_q_dn6_slot;
        let mut var_q_dn7: f64 = *var_q_dn7_slot;
        let mut var_q_dn8: f64 = *var_q_dn8_slot;
        let mut var_qcoth: f64 = *var_qcoth_slot;
        let mut var_qcoth_dn3: f64 = *var_qcoth_dn3_slot;
        let mut var_qcoth_dn4: f64 = *var_qcoth_dn4_slot;
        let mut var_qcoth_dn5: f64 = *var_qcoth_dn5_slot;
        let mut var_qcoth_dn6: f64 = *var_qcoth_dn6_slot;
        let mut var_qcoth_dn7: f64 = *var_qcoth_dn7_slot;
        let mut var_qcoth_dn8: f64 = *var_qcoth_dn8_slot;
        let mut var_qsqrt: f64 = *var_qsqrt_slot;
        let mut var_qsqrt_dn3: f64 = *var_qsqrt_dn3_slot;
        let mut var_qsqrt_dn4: f64 = *var_qsqrt_dn4_slot;
        let mut var_qsqrt_dn5: f64 = *var_qsqrt_dn5_slot;
        let mut var_qsqrt_dn6: f64 = *var_qsqrt_dn6_slot;
        let mut var_qsqrt_dn7: f64 = *var_qsqrt_dn7_slot;
        let mut var_qsqrt_dn8: f64 = *var_qsqrt_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;

        let assign9590_e9170: f64 = (var_q * var_coth1);
        var_qcoth = assign9590_e9170;
        var_qcoth_dn3 = ((var_q_dn3 * var_coth1) + (var_q * var_coth1_dn3));
        var_qcoth_dn4 = ((var_q_dn4 * var_coth1) + (var_q * var_coth1_dn4));
        var_qcoth_dn5 = ((var_q_dn5 * var_coth1) + (var_q * var_coth1_dn5));
        var_qcoth_dn6 = ((var_q_dn6 * var_coth1) + (var_q * var_coth1_dn6));
        var_qcoth_dn7 = ((var_q_dn7 * var_coth1) + (var_q * var_coth1_dn7));
        var_qcoth_dn8 = ((var_q_dn8 * var_coth1) + (var_q * var_coth1_dn8));

        let assign9600_e9173: f64 = (var_auxb1 + var_qcoth);
        var_t2 = assign9600_e9173;
        var_t2_dn3 = (var_auxb1_dn3 + var_qcoth_dn3);
        var_t2_dn4 = (var_auxb1_dn4 + var_qcoth_dn4);
        var_t2_dn5 = (var_auxb1_dn5 + var_qcoth_dn5);
        var_t2_dn6 = (var_auxb1_dn6 + var_qcoth_dn6);
        var_t2_dn7 = (var_auxb1_dn7 + var_qcoth_dn7);
        var_t2_dn8 = (var_auxb1_dn8 + var_qcoth_dn8);

        let assign9610_e9176: f64 = (1.0 / var_t2);
        var_t3 = assign9610_e9176;
        var_t3_dn3 = (-(var_t2_dn3 / (var_t2 * var_t2)));
        var_t3_dn4 = (-(var_t2_dn4 / (var_t2 * var_t2)));
        var_t3_dn5 = (-(var_t2_dn5 / (var_t2 * var_t2)));
        var_t3_dn6 = (-(var_t2_dn6 / (var_t2 * var_t2)));
        var_t3_dn7 = (-(var_t2_dn7 / (var_t2 * var_t2)));
        var_t3_dn8 = (-(var_t2_dn8 / (var_t2 * var_t2)));

        let assign9620_e9179: f64 = (var_xg2 - var_xg1);
        let assign9620_e9181: f64 = (assign9620_e9179 + var_q1);
        let assign9620_e9184: f64 = (var_qsqrt * var_t1);
        let assign9620_e9186: f64 = (assign9620_e9184 * var_t3);
        let assign9620_e9188: f64 = (assign9620_e9186 * var_t3);
        let assign9620_e9189: f64 = (assign9620_e9188).abs();
        let assign9620_e9190: f64 = (assign9620_e9189).ln();
        let assign9620_e9191: f64 = (assign9620_e9181 - assign9620_e9190);
        var_q2 = assign9620_e9191;
        var_q2_dn3 = (((var_xg2_dn3 - var_xg1_dn3) + var_q1_dn3) - (if assign9620_e9188 >= 0.0 { ((((((var_qsqrt_dn3 * var_t1) + (var_qsqrt * var_t1_dn3)) * var_t3) + (assign9620_e9184 * var_t3_dn3)) * var_t3) + (assign9620_e9186 * var_t3_dn3)) } else { (-((((((var_qsqrt_dn3 * var_t1) + (var_qsqrt * var_t1_dn3)) * var_t3) + (assign9620_e9184 * var_t3_dn3)) * var_t3) + (assign9620_e9186 * var_t3_dn3))) } / assign9620_e9189));
        var_q2_dn4 = (((var_xg2_dn4 - var_xg1_dn4) + var_q1_dn4) - (if assign9620_e9188 >= 0.0 { ((((((var_qsqrt_dn4 * var_t1) + (var_qsqrt * var_t1_dn4)) * var_t3) + (assign9620_e9184 * var_t3_dn4)) * var_t3) + (assign9620_e9186 * var_t3_dn4)) } else { (-((((((var_qsqrt_dn4 * var_t1) + (var_qsqrt * var_t1_dn4)) * var_t3) + (assign9620_e9184 * var_t3_dn4)) * var_t3) + (assign9620_e9186 * var_t3_dn4))) } / assign9620_e9189));
        var_q2_dn5 = (((var_xg2_dn5 - var_xg1_dn5) + var_q1_dn5) - (if assign9620_e9188 >= 0.0 { ((((((var_qsqrt_dn5 * var_t1) + (var_qsqrt * var_t1_dn5)) * var_t3) + (assign9620_e9184 * var_t3_dn5)) * var_t3) + (assign9620_e9186 * var_t3_dn5)) } else { (-((((((var_qsqrt_dn5 * var_t1) + (var_qsqrt * var_t1_dn5)) * var_t3) + (assign9620_e9184 * var_t3_dn5)) * var_t3) + (assign9620_e9186 * var_t3_dn5))) } / assign9620_e9189));
        var_q2_dn6 = (((var_xg2_dn6 - var_xg1_dn6) + var_q1_dn6) - (if assign9620_e9188 >= 0.0 { ((((((var_qsqrt_dn6 * var_t1) + (var_qsqrt * var_t1_dn6)) * var_t3) + (assign9620_e9184 * var_t3_dn6)) * var_t3) + (assign9620_e9186 * var_t3_dn6)) } else { (-((((((var_qsqrt_dn6 * var_t1) + (var_qsqrt * var_t1_dn6)) * var_t3) + (assign9620_e9184 * var_t3_dn6)) * var_t3) + (assign9620_e9186 * var_t3_dn6))) } / assign9620_e9189));
        var_q2_dn7 = (((var_xg2_dn7 - var_xg1_dn7) + var_q1_dn7) - (if assign9620_e9188 >= 0.0 { ((((((var_qsqrt_dn7 * var_t1) + (var_qsqrt * var_t1_dn7)) * var_t3) + (assign9620_e9184 * var_t3_dn7)) * var_t3) + (assign9620_e9186 * var_t3_dn7)) } else { (-((((((var_qsqrt_dn7 * var_t1) + (var_qsqrt * var_t1_dn7)) * var_t3) + (assign9620_e9184 * var_t3_dn7)) * var_t3) + (assign9620_e9186 * var_t3_dn7))) } / assign9620_e9189));
        var_q2_dn8 = (((var_xg2_dn8 - var_xg1_dn8) + var_q1_dn8) - (if assign9620_e9188 >= 0.0 { ((((((var_qsqrt_dn8 * var_t1) + (var_qsqrt * var_t1_dn8)) * var_t3) + (assign9620_e9184 * var_t3_dn8)) * var_t3) + (assign9620_e9186 * var_t3_dn8)) } else { (-((((((var_qsqrt_dn8 * var_t1) + (var_qsqrt * var_t1_dn8)) * var_t3) + (assign9620_e9184 * var_t3_dn8)) * var_t3) + (assign9620_e9186 * var_t3_dn8))) } / assign9620_e9189));

        let assign9630_e9195: f64 = (var_auxb1 + var_qcoth);
        let assign9630_e9198: f64 = (var_k2 * var_q2);
        let assign9630_e9200: f64 = (assign9630_e9198 + var_auxb1);
        let assign9630_e9201: f64 = (assign9630_e9195 * assign9630_e9200);
        let assign9630_e9202: f64 = (var_aaux + assign9630_e9201);
        var_f = assign9630_e9202;
        var_f_dn3 = (var_aaux_dn3 + (((var_auxb1_dn3 + var_qcoth_dn3) * assign9630_e9200) + (assign9630_e9195 * ((var_k2 * var_q2_dn3) + var_auxb1_dn3))));
        var_f_dn4 = (var_aaux_dn4 + (((var_auxb1_dn4 + var_qcoth_dn4) * assign9630_e9200) + (assign9630_e9195 * ((var_k2 * var_q2_dn4) + var_auxb1_dn4))));
        var_f_dn5 = (var_aaux_dn5 + (((var_auxb1_dn5 + var_qcoth_dn5) * assign9630_e9200) + (assign9630_e9195 * ((var_k2 * var_q2_dn5) + var_auxb1_dn5))));
        var_f_dn6 = (var_aaux_dn6 + (((var_auxb1_dn6 + var_qcoth_dn6) * assign9630_e9200) + (assign9630_e9195 * ((var_k2 * var_q2_dn6) + var_auxb1_dn6))));
        var_f_dn7 = (var_aaux_dn7 + (((var_auxb1_dn7 + var_qcoth_dn7) * assign9630_e9200) + (assign9630_e9195 * ((var_k2 * var_q2_dn7) + var_auxb1_dn7))));
        var_f_dn8 = (var_aaux_dn8 + (((var_auxb1_dn8 + var_qcoth_dn8) * assign9630_e9200) + (assign9630_e9195 * ((var_k2 * var_q2_dn8) + var_auxb1_dn8))));

        let assign9640_e9205: f64 = (1.0 / var_qsqrt);
        let assign9640_e9207: f64 = (assign9640_e9205 - var_t0);
        var_dlogsinhqsqdqsqrt = assign9640_e9207;
        var_dlogsinhqsqdqsqrt_dn3 = ((-(var_qsqrt_dn3 / (var_qsqrt * var_qsqrt))) - var_t0_dn3);
        var_dlogsinhqsqdqsqrt_dn4 = ((-(var_qsqrt_dn4 / (var_qsqrt * var_qsqrt))) - var_t0_dn4);
        var_dlogsinhqsqdqsqrt_dn5 = ((-(var_qsqrt_dn5 / (var_qsqrt * var_qsqrt))) - var_t0_dn5);
        var_dlogsinhqsqdqsqrt_dn6 = ((-(var_qsqrt_dn6 / (var_qsqrt * var_qsqrt))) - var_t0_dn6);
        var_dlogsinhqsqdqsqrt_dn7 = ((-(var_qsqrt_dn7 / (var_qsqrt * var_qsqrt))) - var_t0_dn7);
        var_dlogsinhqsqdqsqrt_dn8 = ((-(var_qsqrt_dn8 / (var_qsqrt * var_qsqrt))) - var_t0_dn8);

        let assign9650_e9209: f64 = (-2.0);
        let assign9650_e9211: f64 = (assign9650_e9209 * var_k1);
        let assign9650_e9213: f64 = (assign9650_e9211 * var_auxb1);
        let assign9650_e9215: f64 = (assign9650_e9213 + var_aaux);
        var_dqsqrt = assign9650_e9215;
        var_dqsqrt_dn3 = ((assign9650_e9211 * var_auxb1_dn3) + var_aaux_dn3);
        var_dqsqrt_dn4 = ((assign9650_e9211 * var_auxb1_dn4) + var_aaux_dn4);
        var_dqsqrt_dn5 = ((assign9650_e9211 * var_auxb1_dn5) + var_aaux_dn5);
        var_dqsqrt_dn6 = ((assign9650_e9211 * var_auxb1_dn6) + var_aaux_dn6);
        var_dqsqrt_dn7 = ((assign9650_e9211 * var_auxb1_dn7) + var_aaux_dn7);
        var_dqsqrt_dn8 = ((assign9650_e9211 * var_auxb1_dn8) + var_aaux_dn8);

        let assign9660_e9218: f64 = (var_dqcothqdqsqrt * var_dqsqrt);
        var_dqcoth = assign9660_e9218;
        var_dqcoth_dn3 = ((var_dqcothqdqsqrt_dn3 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn3));
        var_dqcoth_dn4 = ((var_dqcothqdqsqrt_dn4 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn4));
        var_dqcoth_dn5 = ((var_dqcothqdqsqrt_dn5 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn5));
        var_dqcoth_dn6 = ((var_dqcothqdqsqrt_dn6 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn6));
        var_dqcoth_dn7 = ((var_dqcothqdqsqrt_dn7 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn7));
        var_dqcoth_dn8 = ((var_dqcothqdqsqrt_dn8 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn8));

        let assign9670_e9220: f64 = (-1.0);
        let assign9670_e9223: f64 = (-var_k1);
        let assign9670_e9225: f64 = (assign9670_e9223 + var_dqcoth);
        let assign9670_e9227: f64 = (assign9670_e9225 * var_t3);
        let assign9670_e9228: f64 = (2.0 * assign9670_e9227);
        let assign9670_e9229: f64 = (assign9670_e9220 + assign9670_e9228);
        let assign9670_e9232: f64 = (var_dlogsinhqsqdqsqrt * var_dqsqrt);
        let assign9670_e9233: f64 = (assign9670_e9229 - assign9670_e9232);
        var_dq2 = assign9670_e9233;
        var_dq2_dn3 = ((2.0 * ((var_dqcoth_dn3 * var_t3) + (assign9670_e9225 * var_t3_dn3))) - ((var_dlogsinhqsqdqsqrt_dn3 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn3)));
        var_dq2_dn4 = ((2.0 * ((var_dqcoth_dn4 * var_t3) + (assign9670_e9225 * var_t3_dn4))) - ((var_dlogsinhqsqdqsqrt_dn4 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn4)));
        var_dq2_dn5 = ((2.0 * ((var_dqcoth_dn5 * var_t3) + (assign9670_e9225 * var_t3_dn5))) - ((var_dlogsinhqsqdqsqrt_dn5 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn5)));
        var_dq2_dn6 = ((2.0 * ((var_dqcoth_dn6 * var_t3) + (assign9670_e9225 * var_t3_dn6))) - ((var_dlogsinhqsqdqsqrt_dn6 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn6)));
        var_dq2_dn7 = ((2.0 * ((var_dqcoth_dn7 * var_t3) + (assign9670_e9225 * var_t3_dn7))) - ((var_dlogsinhqsqdqsqrt_dn7 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn7)));
        var_dq2_dn8 = ((2.0 * ((var_dqcoth_dn8 * var_t3) + (assign9670_e9225 * var_t3_dn8))) - ((var_dlogsinhqsqdqsqrt_dn8 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn8)));

        let assign9680_e9238: f64 = (var_auxb1 + var_t2);
        let assign9680_e9239: f64 = (var_k1 * assign9680_e9238);
        let assign9680_e9240: f64 = (var_aaux - assign9680_e9239);
        let assign9680_e9243: f64 = (var_auxb1 * var_dqcoth);
        let assign9680_e9244: f64 = (assign9680_e9240 + assign9680_e9243);
        let assign9680_e9248: f64 = (var_dq2 * var_t2);
        let assign9680_e9252: f64 = (var_dqcoth - var_k1);
        let assign9680_e9253: f64 = (var_q2 * assign9680_e9252);
        let assign9680_e9254: f64 = (assign9680_e9248 + assign9680_e9253);
        let assign9680_e9255: f64 = (var_k2 * assign9680_e9254);
        let assign9680_e9256: f64 = (assign9680_e9244 + assign9680_e9255);
        var_df = assign9680_e9256;
        var_df_dn3 = (((var_aaux_dn3 - (var_k1 * (var_auxb1_dn3 + var_t2_dn3))) + ((var_auxb1_dn3 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn3))) + (var_k2 * (((var_dq2_dn3 * var_t2) + (var_dq2 * var_t2_dn3)) + ((var_q2_dn3 * assign9680_e9252) + (var_q2 * var_dqcoth_dn3)))));
        var_df_dn4 = (((var_aaux_dn4 - (var_k1 * (var_auxb1_dn4 + var_t2_dn4))) + ((var_auxb1_dn4 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn4))) + (var_k2 * (((var_dq2_dn4 * var_t2) + (var_dq2 * var_t2_dn4)) + ((var_q2_dn4 * assign9680_e9252) + (var_q2 * var_dqcoth_dn4)))));
        var_df_dn5 = (((var_aaux_dn5 - (var_k1 * (var_auxb1_dn5 + var_t2_dn5))) + ((var_auxb1_dn5 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn5))) + (var_k2 * (((var_dq2_dn5 * var_t2) + (var_dq2 * var_t2_dn5)) + ((var_q2_dn5 * assign9680_e9252) + (var_q2 * var_dqcoth_dn5)))));
        var_df_dn6 = (((var_aaux_dn6 - (var_k1 * (var_auxb1_dn6 + var_t2_dn6))) + ((var_auxb1_dn6 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn6))) + (var_k2 * (((var_dq2_dn6 * var_t2) + (var_dq2 * var_t2_dn6)) + ((var_q2_dn6 * assign9680_e9252) + (var_q2 * var_dqcoth_dn6)))));
        var_df_dn7 = (((var_aaux_dn7 - (var_k1 * (var_auxb1_dn7 + var_t2_dn7))) + ((var_auxb1_dn7 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn7))) + (var_k2 * (((var_dq2_dn7 * var_t2) + (var_dq2 * var_t2_dn7)) + ((var_q2_dn7 * assign9680_e9252) + (var_q2 * var_dqcoth_dn7)))));
        var_df_dn8 = (((var_aaux_dn8 - (var_k1 * (var_auxb1_dn8 + var_t2_dn8))) + ((var_auxb1_dn8 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn8))) + (var_k2 * (((var_dq2_dn8 * var_t2) + (var_dq2 * var_t2_dn8)) + ((var_q2_dn8 * assign9680_e9252) + (var_q2 * var_dqcoth_dn8)))));

        let assign9690_e9258: f64 = (-var_f);
        let assign9690_e9260: f64 = (assign9690_e9258 / var_df);
        var_delta = assign9690_e9260;
        var_delta_dn3 = ((((-var_f_dn3) * var_df) - (assign9690_e9258 * var_df_dn3)) / (var_df * var_df));
        var_delta_dn4 = ((((-var_f_dn4) * var_df) - (assign9690_e9258 * var_df_dn4)) / (var_df * var_df));
        var_delta_dn5 = ((((-var_f_dn5) * var_df) - (assign9690_e9258 * var_df_dn5)) / (var_df * var_df));
        var_delta_dn6 = ((((-var_f_dn6) * var_df) - (assign9690_e9258 * var_df_dn6)) / (var_df * var_df));
        var_delta_dn7 = ((((-var_f_dn7) * var_df) - (assign9690_e9258 * var_df_dn7)) / (var_df * var_df));
        var_delta_dn8 = ((((-var_f_dn8) * var_df) - (assign9690_e9258 * var_df_dn8)) / (var_df * var_df));

        let assign9700_e9263: f64 = (var_phi1 + var_delta);
        var_phi1 = assign9700_e9263;
        var_phi1_dn3 = (var_phi1_dn3 + var_delta_dn3);
        var_phi1_dn4 = (var_phi1_dn4 + var_delta_dn4);
        var_phi1_dn5 = (var_phi1_dn5 + var_delta_dn5);
        var_phi1_dn6 = (var_phi1_dn6 + var_delta_dn6);
        var_phi1_dn7 = (var_phi1_dn7 + var_delta_dn7);
        var_phi1_dn8 = (var_phi1_dn8 + var_delta_dn8);

        let assign9710_e9266: f64 = (var_xg1 - var_phi1);
        var_q1 = assign9710_e9266;
        var_q1_dn3 = (var_xg1_dn3 - var_phi1_dn3);
        var_q1_dn4 = (var_xg1_dn4 - var_phi1_dn4);
        var_q1_dn5 = (var_xg1_dn5 - var_phi1_dn5);
        var_q1_dn6 = (var_xg1_dn6 - var_phi1_dn6);
        var_q1_dn7 = (var_xg1_dn7 - var_phi1_dn7);
        var_q1_dn8 = (var_xg1_dn8 - var_phi1_dn8);

        let assign9720_e9269: f64 = (var_k1 * var_q1);
        var_auxb1 = assign9720_e9269;
        var_auxb1_dn3 = (var_k1 * var_q1_dn3);
        var_auxb1_dn4 = (var_k1 * var_q1_dn4);
        var_auxb1_dn5 = (var_k1 * var_q1_dn5);
        var_auxb1_dn6 = (var_k1 * var_q1_dn6);
        var_auxb1_dn7 = (var_k1 * var_q1_dn7);
        var_auxb1_dn8 = (var_k1 * var_q1_dn8);

        let assign9730_e9271: f64 = (-var_a0);
        let assign9730_e9273: f64 = (var_phi1).exp();
        let assign9730_e9274: f64 = (assign9730_e9271 * assign9730_e9273);
        var_aaux = assign9730_e9274;
        var_aaux_dn3 = (((-var_a0_dn3) * assign9730_e9273) + (assign9730_e9271 * (assign9730_e9273 * var_phi1_dn3)));
        var_aaux_dn4 = (((-var_a0_dn4) * assign9730_e9273) + (assign9730_e9271 * (assign9730_e9273 * var_phi1_dn4)));
        var_aaux_dn5 = (((-var_a0_dn5) * assign9730_e9273) + (assign9730_e9271 * (assign9730_e9273 * var_phi1_dn5)));
        var_aaux_dn6 = (((-var_a0_dn6) * assign9730_e9273) + (assign9730_e9271 * (assign9730_e9273 * var_phi1_dn6)));
        var_aaux_dn7 = (((-var_a0_dn7) * assign9730_e9273) + (assign9730_e9271 * (assign9730_e9273 * var_phi1_dn7)));
        var_aaux_dn8 = (((-var_a0_dn8) * assign9730_e9273) + (assign9730_e9271 * (assign9730_e9273 * var_phi1_dn8)));

        let assign9740_e9277: f64 = (var_auxb1 * var_auxb1);
        let assign9740_e9279: f64 = (assign9740_e9277 + var_aaux);
        var_qsqrt = assign9740_e9279;
        var_qsqrt_dn3 = (((var_auxb1_dn3 * var_auxb1) + (var_auxb1 * var_auxb1_dn3)) + var_aaux_dn3);
        var_qsqrt_dn4 = (((var_auxb1_dn4 * var_auxb1) + (var_auxb1 * var_auxb1_dn4)) + var_aaux_dn4);
        var_qsqrt_dn5 = (((var_auxb1_dn5 * var_auxb1) + (var_auxb1 * var_auxb1_dn5)) + var_aaux_dn5);
        var_qsqrt_dn6 = (((var_auxb1_dn6 * var_auxb1) + (var_auxb1 * var_auxb1_dn6)) + var_aaux_dn6);
        var_qsqrt_dn7 = (((var_auxb1_dn7 * var_auxb1) + (var_auxb1 * var_auxb1_dn7)) + var_aaux_dn7);
        var_qsqrt_dn8 = (((var_auxb1_dn8 * var_auxb1) + (var_auxb1 * var_auxb1_dn8)) + var_aaux_dn8);

        let assign9750_e9282: f64 = if var_qsqrt < 0.0 { 1.0 } else { 0.0 };
        var_guard96 = assign9750_e9282;

        let (assign9760_e9288, assign9760_e9288_d_n3, assign9760_e9288_d_n4, assign9760_e9288_d_n5, assign9760_e9288_d_n6, assign9760_e9288_d_n7, assign9760_e9288_d_n8,) = {
    if (var_guard96 != 0.0) {
        let assign9760_e9285: f64 = (-var_qsqrt);
        let assign9760_e9286: f64 = (assign9760_e9285).sqrt();
        (assign9760_e9286, ((-var_qsqrt_dn3) / (2.0 * assign9760_e9286)), ((-var_qsqrt_dn4) / (2.0 * assign9760_e9286)), ((-var_qsqrt_dn5) / (2.0 * assign9760_e9286)), ((-var_qsqrt_dn6) / (2.0 * assign9760_e9286)), ((-var_qsqrt_dn7) / (2.0 * assign9760_e9286)), ((-var_qsqrt_dn8) / (2.0 * assign9760_e9286)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign9760_e9288;
        var_q_dn3 = assign9760_e9288_d_n3;
        var_q_dn4 = assign9760_e9288_d_n4;
        var_q_dn5 = assign9760_e9288_d_n5;
        var_q_dn6 = assign9760_e9288_d_n6;
        var_q_dn7 = assign9760_e9288_d_n7;
        var_q_dn8 = assign9760_e9288_d_n8;

        let (assign9770_e9297, assign9770_e9297_d_n3, assign9770_e9297_d_n4, assign9770_e9297_d_n5, assign9770_e9297_d_n6, assign9770_e9297_d_n7, assign9770_e9297_d_n8,) = {
    if (var_guard96 != 0.0) {
        let assign9770_e9293: f64 = (0.5 * var_q);
        let assign9770_e9294: f64 = (assign9770_e9293).sin();
        let assign9770_e9295: f64 = (1.0 / assign9770_e9294);
        (assign9770_e9295, (-(((assign9770_e9293).cos() * (0.5 * var_q_dn3)) / (assign9770_e9294 * assign9770_e9294))), (-(((assign9770_e9293).cos() * (0.5 * var_q_dn4)) / (assign9770_e9294 * assign9770_e9294))), (-(((assign9770_e9293).cos() * (0.5 * var_q_dn5)) / (assign9770_e9294 * assign9770_e9294))), (-(((assign9770_e9293).cos() * (0.5 * var_q_dn6)) / (assign9770_e9294 * assign9770_e9294))), (-(((assign9770_e9293).cos() * (0.5 * var_q_dn7)) / (assign9770_e9294 * assign9770_e9294))), (-(((assign9770_e9293).cos() * (0.5 * var_q_dn8)) / (assign9770_e9294 * assign9770_e9294))),)
    } else {
        (var_csc1, var_csc1_dn3, var_csc1_dn4, var_csc1_dn5, var_csc1_dn6, var_csc1_dn7, var_csc1_dn8,)
    }
};
        var_csc1 = assign9770_e9297;
        var_csc1_dn3 = assign9770_e9297_d_n3;
        var_csc1_dn4 = assign9770_e9297_d_n4;
        var_csc1_dn5 = assign9770_e9297_d_n5;
        var_csc1_dn6 = assign9770_e9297_d_n6;
        var_csc1_dn7 = assign9770_e9297_d_n7;
        var_csc1_dn8 = assign9770_e9297_d_n8;

        let (assign9780_e9303, assign9780_e9303_d_n3, assign9780_e9303_d_n4, assign9780_e9303_d_n5, assign9780_e9303_d_n6, assign9780_e9303_d_n7, assign9780_e9303_d_n8,) = {
    if (var_guard96 != 0.0) {
        let assign9780_e9301: f64 = (var_csc1 * var_csc1);
        (assign9780_e9301, ((var_csc1_dn3 * var_csc1) + (var_csc1 * var_csc1_dn3)), ((var_csc1_dn4 * var_csc1) + (var_csc1 * var_csc1_dn4)), ((var_csc1_dn5 * var_csc1) + (var_csc1 * var_csc1_dn5)), ((var_csc1_dn6 * var_csc1) + (var_csc1 * var_csc1_dn6)), ((var_csc1_dn7 * var_csc1) + (var_csc1 * var_csc1_dn7)), ((var_csc1_dn8 * var_csc1) + (var_csc1 * var_csc1_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign9780_e9303;
        var_t1_dn3 = assign9780_e9303_d_n3;
        var_t1_dn4 = assign9780_e9303_d_n4;
        var_t1_dn5 = assign9780_e9303_d_n5;
        var_t1_dn6 = assign9780_e9303_d_n6;
        var_t1_dn7 = assign9780_e9303_d_n7;
        var_t1_dn8 = assign9780_e9303_d_n8;

        let (assign9790_e9312, assign9790_e9312_d_n3, assign9790_e9312_d_n4, assign9790_e9312_d_n5, assign9790_e9312_d_n6, assign9790_e9312_d_n7, assign9790_e9312_d_n8,) = {
    if (var_guard96 != 0.0) {
        let assign9790_e9307: f64 = (0.5 * var_q);
        let assign9790_e9308: f64 = (assign9790_e9307).cos();
        let assign9790_e9310: f64 = (assign9790_e9308 * var_csc1);
        (assign9790_e9310, (((-(assign9790_e9307).sin() * (0.5 * var_q_dn3)) * var_csc1) + (assign9790_e9308 * var_csc1_dn3)), (((-(assign9790_e9307).sin() * (0.5 * var_q_dn4)) * var_csc1) + (assign9790_e9308 * var_csc1_dn4)), (((-(assign9790_e9307).sin() * (0.5 * var_q_dn5)) * var_csc1) + (assign9790_e9308 * var_csc1_dn5)), (((-(assign9790_e9307).sin() * (0.5 * var_q_dn6)) * var_csc1) + (assign9790_e9308 * var_csc1_dn6)), (((-(assign9790_e9307).sin() * (0.5 * var_q_dn7)) * var_csc1) + (assign9790_e9308 * var_csc1_dn7)), (((-(assign9790_e9307).sin() * (0.5 * var_q_dn8)) * var_csc1) + (assign9790_e9308 * var_csc1_dn8)),)
    } else {
        (var_coth1, var_coth1_dn3, var_coth1_dn4, var_coth1_dn5, var_coth1_dn6, var_coth1_dn7, var_coth1_dn8,)
    }
};
        var_coth1 = assign9790_e9312;
        var_coth1_dn3 = assign9790_e9312_d_n3;
        var_coth1_dn4 = assign9790_e9312_d_n4;
        var_coth1_dn5 = assign9790_e9312_d_n5;
        var_coth1_dn6 = assign9790_e9312_d_n6;
        var_coth1_dn7 = assign9790_e9312_d_n7;
        var_coth1_dn8 = assign9790_e9312_d_n8;

        let (assign9800_e9321, assign9800_e9321_d_n3, assign9800_e9321_d_n4, assign9800_e9321_d_n5, assign9800_e9321_d_n6, assign9800_e9321_d_n7, assign9800_e9321_d_n8,) = {
    if (var_guard96 != 0.0) {
        let assign9800_e9315: f64 = (-0.5);
        let assign9800_e9317: f64 = (assign9800_e9315 * var_coth1);
        let assign9800_e9319: f64 = (assign9800_e9317 / var_q);
        (assign9800_e9319, ((((assign9800_e9315 * var_coth1_dn3) * var_q) - (assign9800_e9317 * var_q_dn3)) / (var_q * var_q)), ((((assign9800_e9315 * var_coth1_dn4) * var_q) - (assign9800_e9317 * var_q_dn4)) / (var_q * var_q)), ((((assign9800_e9315 * var_coth1_dn5) * var_q) - (assign9800_e9317 * var_q_dn5)) / (var_q * var_q)), ((((assign9800_e9315 * var_coth1_dn6) * var_q) - (assign9800_e9317 * var_q_dn6)) / (var_q * var_q)), ((((assign9800_e9315 * var_coth1_dn7) * var_q) - (assign9800_e9317 * var_q_dn7)) / (var_q * var_q)), ((((assign9800_e9315 * var_coth1_dn8) * var_q) - (assign9800_e9317 * var_q_dn8)) / (var_q * var_q)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign9800_e9321;
        var_t0_dn3 = assign9800_e9321_d_n3;
        var_t0_dn4 = assign9800_e9321_d_n4;
        var_t0_dn5 = assign9800_e9321_d_n5;
        var_t0_dn6 = assign9800_e9321_d_n6;
        var_t0_dn7 = assign9800_e9321_d_n7;
        var_t0_dn8 = assign9800_e9321_d_n8;

        let (assign9810_e9329, assign9810_e9329_d_n3, assign9810_e9329_d_n4, assign9810_e9329_d_n5, assign9810_e9329_d_n6, assign9810_e9329_d_n7, assign9810_e9329_d_n8,) = {
    if (var_guard96 != 0.0) {
        let assign9810_e9325: f64 = (0.25 * var_t1);
        let assign9810_e9327: f64 = (assign9810_e9325 + var_t0);
        (assign9810_e9327, ((0.25 * var_t1_dn3) + var_t0_dn3), ((0.25 * var_t1_dn4) + var_t0_dn4), ((0.25 * var_t1_dn5) + var_t0_dn5), ((0.25 * var_t1_dn6) + var_t0_dn6), ((0.25 * var_t1_dn7) + var_t0_dn7), ((0.25 * var_t1_dn8) + var_t0_dn8),)
    } else {
        (var_dqcothqdqsqrt, var_dqcothqdqsqrt_dn3, var_dqcothqdqsqrt_dn4, var_dqcothqdqsqrt_dn5, var_dqcothqdqsqrt_dn6, var_dqcothqdqsqrt_dn7, var_dqcothqdqsqrt_dn8,)
    }
};
        var_dqcothqdqsqrt = assign9810_e9329;
        var_dqcothqdqsqrt_dn3 = assign9810_e9329_d_n3;
        var_dqcothqdqsqrt_dn4 = assign9810_e9329_d_n4;
        var_dqcothqdqsqrt_dn5 = assign9810_e9329_d_n5;
        var_dqcothqdqsqrt_dn6 = assign9810_e9329_d_n6;
        var_dqcothqdqsqrt_dn7 = assign9810_e9329_d_n7;
        var_dqcothqdqsqrt_dn8 = assign9810_e9329_d_n8;

        let (assign9820_e9335, assign9820_e9335_d_n3, assign9820_e9335_d_n4, assign9820_e9335_d_n5, assign9820_e9335_d_n6, assign9820_e9335_d_n7, assign9820_e9335_d_n8,) = {
    if (var_guard96 == 0.0) {
        let assign9820_e9333: f64 = (var_qsqrt).sqrt();
        (assign9820_e9333, (var_qsqrt_dn3 / (2.0 * assign9820_e9333)), (var_qsqrt_dn4 / (2.0 * assign9820_e9333)), (var_qsqrt_dn5 / (2.0 * assign9820_e9333)), (var_qsqrt_dn6 / (2.0 * assign9820_e9333)), (var_qsqrt_dn7 / (2.0 * assign9820_e9333)), (var_qsqrt_dn8 / (2.0 * assign9820_e9333)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign9820_e9335;
        var_q_dn3 = assign9820_e9335_d_n3;
        var_q_dn4 = assign9820_e9335_d_n4;
        var_q_dn5 = assign9820_e9335_d_n5;
        var_q_dn6 = assign9820_e9335_d_n6;
        var_q_dn7 = assign9820_e9335_d_n7;
        var_q_dn8 = assign9820_e9335_d_n8;

        let (assign9830_e9345, assign9830_e9345_d_n3, assign9830_e9345_d_n4, assign9830_e9345_d_n5, assign9830_e9345_d_n6, assign9830_e9345_d_n7, assign9830_e9345_d_n8,) = {
    if (var_guard96 == 0.0) {
        let assign9830_e9341: f64 = (0.5 * var_q);
        let assign9830_e9342: f64 = (assign9830_e9341).sinh();
        let assign9830_e9343: f64 = (1.0 / assign9830_e9342);
        (assign9830_e9343, (-(((assign9830_e9341).cosh() * (0.5 * var_q_dn3)) / (assign9830_e9342 * assign9830_e9342))), (-(((assign9830_e9341).cosh() * (0.5 * var_q_dn4)) / (assign9830_e9342 * assign9830_e9342))), (-(((assign9830_e9341).cosh() * (0.5 * var_q_dn5)) / (assign9830_e9342 * assign9830_e9342))), (-(((assign9830_e9341).cosh() * (0.5 * var_q_dn6)) / (assign9830_e9342 * assign9830_e9342))), (-(((assign9830_e9341).cosh() * (0.5 * var_q_dn7)) / (assign9830_e9342 * assign9830_e9342))), (-(((assign9830_e9341).cosh() * (0.5 * var_q_dn8)) / (assign9830_e9342 * assign9830_e9342))),)
    } else {
        (var_csc1, var_csc1_dn3, var_csc1_dn4, var_csc1_dn5, var_csc1_dn6, var_csc1_dn7, var_csc1_dn8,)
    }
};
        var_csc1 = assign9830_e9345;
        var_csc1_dn3 = assign9830_e9345_d_n3;
        var_csc1_dn4 = assign9830_e9345_d_n4;
        var_csc1_dn5 = assign9830_e9345_d_n5;
        var_csc1_dn6 = assign9830_e9345_d_n6;
        var_csc1_dn7 = assign9830_e9345_d_n7;
        var_csc1_dn8 = assign9830_e9345_d_n8;

        let (assign9840_e9352, assign9840_e9352_d_n3, assign9840_e9352_d_n4, assign9840_e9352_d_n5, assign9840_e9352_d_n6, assign9840_e9352_d_n7, assign9840_e9352_d_n8,) = {
    if (var_guard96 == 0.0) {
        let assign9840_e9350: f64 = (var_csc1 * var_csc1);
        (assign9840_e9350, ((var_csc1_dn3 * var_csc1) + (var_csc1 * var_csc1_dn3)), ((var_csc1_dn4 * var_csc1) + (var_csc1 * var_csc1_dn4)), ((var_csc1_dn5 * var_csc1) + (var_csc1 * var_csc1_dn5)), ((var_csc1_dn6 * var_csc1) + (var_csc1 * var_csc1_dn6)), ((var_csc1_dn7 * var_csc1) + (var_csc1 * var_csc1_dn7)), ((var_csc1_dn8 * var_csc1) + (var_csc1 * var_csc1_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign9840_e9352;
        var_t1_dn3 = assign9840_e9352_d_n3;
        var_t1_dn4 = assign9840_e9352_d_n4;
        var_t1_dn5 = assign9840_e9352_d_n5;
        var_t1_dn6 = assign9840_e9352_d_n6;
        var_t1_dn7 = assign9840_e9352_d_n7;
        var_t1_dn8 = assign9840_e9352_d_n8;

        let (assign9850_e9360, assign9850_e9360_d_n3, assign9850_e9360_d_n4, assign9850_e9360_d_n5, assign9850_e9360_d_n6, assign9850_e9360_d_n7, assign9850_e9360_d_n8,) = {
    if (var_guard96 == 0.0) {
        let assign9850_e9357: f64 = (1.0 + var_t1);
        let assign9850_e9358: f64 = (assign9850_e9357).sqrt();
        (assign9850_e9358, (var_t1_dn3 / (2.0 * assign9850_e9358)), (var_t1_dn4 / (2.0 * assign9850_e9358)), (var_t1_dn5 / (2.0 * assign9850_e9358)), (var_t1_dn6 / (2.0 * assign9850_e9358)), (var_t1_dn7 / (2.0 * assign9850_e9358)), (var_t1_dn8 / (2.0 * assign9850_e9358)),)
    } else {
        (var_coth1, var_coth1_dn3, var_coth1_dn4, var_coth1_dn5, var_coth1_dn6, var_coth1_dn7, var_coth1_dn8,)
    }
};
        var_coth1 = assign9850_e9360;
        var_coth1_dn3 = assign9850_e9360_d_n3;
        var_coth1_dn4 = assign9850_e9360_d_n4;
        var_coth1_dn5 = assign9850_e9360_d_n5;
        var_coth1_dn6 = assign9850_e9360_d_n6;
        var_coth1_dn7 = assign9850_e9360_d_n7;
        var_coth1_dn8 = assign9850_e9360_d_n8;

        let (assign9860_e9369, assign9860_e9369_d_n3, assign9860_e9369_d_n4, assign9860_e9369_d_n5, assign9860_e9369_d_n6, assign9860_e9369_d_n7, assign9860_e9369_d_n8,) = {
    if (var_guard96 == 0.0) {
        let assign9860_e9365: f64 = (0.5 * var_coth1);
        let assign9860_e9367: f64 = (assign9860_e9365 / var_q);
        (assign9860_e9367, ((((0.5 * var_coth1_dn3) * var_q) - (assign9860_e9365 * var_q_dn3)) / (var_q * var_q)), ((((0.5 * var_coth1_dn4) * var_q) - (assign9860_e9365 * var_q_dn4)) / (var_q * var_q)), ((((0.5 * var_coth1_dn5) * var_q) - (assign9860_e9365 * var_q_dn5)) / (var_q * var_q)), ((((0.5 * var_coth1_dn6) * var_q) - (assign9860_e9365 * var_q_dn6)) / (var_q * var_q)), ((((0.5 * var_coth1_dn7) * var_q) - (assign9860_e9365 * var_q_dn7)) / (var_q * var_q)), ((((0.5 * var_coth1_dn8) * var_q) - (assign9860_e9365 * var_q_dn8)) / (var_q * var_q)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign9860_e9369;
        var_t0_dn3 = assign9860_e9369_d_n3;
        var_t0_dn4 = assign9860_e9369_d_n4;
        var_t0_dn5 = assign9860_e9369_d_n5;
        var_t0_dn6 = assign9860_e9369_d_n6;
        var_t0_dn7 = assign9860_e9369_d_n7;
        var_t0_dn8 = assign9860_e9369_d_n8;

        let (assign9870_e9379, assign9870_e9379_d_n3, assign9870_e9379_d_n4, assign9870_e9379_d_n5, assign9870_e9379_d_n6, assign9870_e9379_d_n7, assign9870_e9379_d_n8,) = {
    if (var_guard96 == 0.0) {
        let assign9870_e9373: f64 = (-0.25);
        let assign9870_e9375: f64 = (assign9870_e9373 * var_t1);
        let assign9870_e9377: f64 = (assign9870_e9375 + var_t0);
        (assign9870_e9377, ((assign9870_e9373 * var_t1_dn3) + var_t0_dn3), ((assign9870_e9373 * var_t1_dn4) + var_t0_dn4), ((assign9870_e9373 * var_t1_dn5) + var_t0_dn5), ((assign9870_e9373 * var_t1_dn6) + var_t0_dn6), ((assign9870_e9373 * var_t1_dn7) + var_t0_dn7), ((assign9870_e9373 * var_t1_dn8) + var_t0_dn8),)
    } else {
        (var_dqcothqdqsqrt, var_dqcothqdqsqrt_dn3, var_dqcothqdqsqrt_dn4, var_dqcothqdqsqrt_dn5, var_dqcothqdqsqrt_dn6, var_dqcothqdqsqrt_dn7, var_dqcothqdqsqrt_dn8,)
    }
};
        var_dqcothqdqsqrt = assign9870_e9379;
        var_dqcothqdqsqrt_dn3 = assign9870_e9379_d_n3;
        var_dqcothqdqsqrt_dn4 = assign9870_e9379_d_n4;
        var_dqcothqdqsqrt_dn5 = assign9870_e9379_d_n5;
        var_dqcothqdqsqrt_dn6 = assign9870_e9379_d_n6;
        var_dqcothqdqsqrt_dn7 = assign9870_e9379_d_n7;
        var_dqcothqdqsqrt_dn8 = assign9870_e9379_d_n8;

        let assign9880_e9382: f64 = (var_q * var_coth1);
        var_qcoth = assign9880_e9382;
        var_qcoth_dn3 = ((var_q_dn3 * var_coth1) + (var_q * var_coth1_dn3));
        var_qcoth_dn4 = ((var_q_dn4 * var_coth1) + (var_q * var_coth1_dn4));
        var_qcoth_dn5 = ((var_q_dn5 * var_coth1) + (var_q * var_coth1_dn5));
        var_qcoth_dn6 = ((var_q_dn6 * var_coth1) + (var_q * var_coth1_dn6));
        var_qcoth_dn7 = ((var_q_dn7 * var_coth1) + (var_q * var_coth1_dn7));
        var_qcoth_dn8 = ((var_q_dn8 * var_coth1) + (var_q * var_coth1_dn8));

        let assign9890_e9385: f64 = (var_auxb1 + var_qcoth);
        var_t2 = assign9890_e9385;
        var_t2_dn3 = (var_auxb1_dn3 + var_qcoth_dn3);
        var_t2_dn4 = (var_auxb1_dn4 + var_qcoth_dn4);
        var_t2_dn5 = (var_auxb1_dn5 + var_qcoth_dn5);
        var_t2_dn6 = (var_auxb1_dn6 + var_qcoth_dn6);
        var_t2_dn7 = (var_auxb1_dn7 + var_qcoth_dn7);
        var_t2_dn8 = (var_auxb1_dn8 + var_qcoth_dn8);

        let assign9900_e9388: f64 = (1.0 / var_t2);
        var_t3 = assign9900_e9388;
        var_t3_dn3 = (-(var_t2_dn3 / (var_t2 * var_t2)));
        var_t3_dn4 = (-(var_t2_dn4 / (var_t2 * var_t2)));
        var_t3_dn5 = (-(var_t2_dn5 / (var_t2 * var_t2)));
        var_t3_dn6 = (-(var_t2_dn6 / (var_t2 * var_t2)));
        var_t3_dn7 = (-(var_t2_dn7 / (var_t2 * var_t2)));
        var_t3_dn8 = (-(var_t2_dn8 / (var_t2 * var_t2)));

        let assign9910_e9391: f64 = (var_xg2 - var_xg1);
        let assign9910_e9393: f64 = (assign9910_e9391 + var_q1);
        let assign9910_e9396: f64 = (var_qsqrt * var_t1);
        let assign9910_e9398: f64 = (assign9910_e9396 * var_t3);
        let assign9910_e9400: f64 = (assign9910_e9398 * var_t3);
        let assign9910_e9401: f64 = (assign9910_e9400).abs();
        let assign9910_e9402: f64 = (assign9910_e9401).ln();
        let assign9910_e9403: f64 = (assign9910_e9393 - assign9910_e9402);
        var_q2 = assign9910_e9403;
        var_q2_dn3 = (((var_xg2_dn3 - var_xg1_dn3) + var_q1_dn3) - (if assign9910_e9400 >= 0.0 { ((((((var_qsqrt_dn3 * var_t1) + (var_qsqrt * var_t1_dn3)) * var_t3) + (assign9910_e9396 * var_t3_dn3)) * var_t3) + (assign9910_e9398 * var_t3_dn3)) } else { (-((((((var_qsqrt_dn3 * var_t1) + (var_qsqrt * var_t1_dn3)) * var_t3) + (assign9910_e9396 * var_t3_dn3)) * var_t3) + (assign9910_e9398 * var_t3_dn3))) } / assign9910_e9401));
        var_q2_dn4 = (((var_xg2_dn4 - var_xg1_dn4) + var_q1_dn4) - (if assign9910_e9400 >= 0.0 { ((((((var_qsqrt_dn4 * var_t1) + (var_qsqrt * var_t1_dn4)) * var_t3) + (assign9910_e9396 * var_t3_dn4)) * var_t3) + (assign9910_e9398 * var_t3_dn4)) } else { (-((((((var_qsqrt_dn4 * var_t1) + (var_qsqrt * var_t1_dn4)) * var_t3) + (assign9910_e9396 * var_t3_dn4)) * var_t3) + (assign9910_e9398 * var_t3_dn4))) } / assign9910_e9401));
        var_q2_dn5 = (((var_xg2_dn5 - var_xg1_dn5) + var_q1_dn5) - (if assign9910_e9400 >= 0.0 { ((((((var_qsqrt_dn5 * var_t1) + (var_qsqrt * var_t1_dn5)) * var_t3) + (assign9910_e9396 * var_t3_dn5)) * var_t3) + (assign9910_e9398 * var_t3_dn5)) } else { (-((((((var_qsqrt_dn5 * var_t1) + (var_qsqrt * var_t1_dn5)) * var_t3) + (assign9910_e9396 * var_t3_dn5)) * var_t3) + (assign9910_e9398 * var_t3_dn5))) } / assign9910_e9401));
        var_q2_dn6 = (((var_xg2_dn6 - var_xg1_dn6) + var_q1_dn6) - (if assign9910_e9400 >= 0.0 { ((((((var_qsqrt_dn6 * var_t1) + (var_qsqrt * var_t1_dn6)) * var_t3) + (assign9910_e9396 * var_t3_dn6)) * var_t3) + (assign9910_e9398 * var_t3_dn6)) } else { (-((((((var_qsqrt_dn6 * var_t1) + (var_qsqrt * var_t1_dn6)) * var_t3) + (assign9910_e9396 * var_t3_dn6)) * var_t3) + (assign9910_e9398 * var_t3_dn6))) } / assign9910_e9401));
        var_q2_dn7 = (((var_xg2_dn7 - var_xg1_dn7) + var_q1_dn7) - (if assign9910_e9400 >= 0.0 { ((((((var_qsqrt_dn7 * var_t1) + (var_qsqrt * var_t1_dn7)) * var_t3) + (assign9910_e9396 * var_t3_dn7)) * var_t3) + (assign9910_e9398 * var_t3_dn7)) } else { (-((((((var_qsqrt_dn7 * var_t1) + (var_qsqrt * var_t1_dn7)) * var_t3) + (assign9910_e9396 * var_t3_dn7)) * var_t3) + (assign9910_e9398 * var_t3_dn7))) } / assign9910_e9401));
        var_q2_dn8 = (((var_xg2_dn8 - var_xg1_dn8) + var_q1_dn8) - (if assign9910_e9400 >= 0.0 { ((((((var_qsqrt_dn8 * var_t1) + (var_qsqrt * var_t1_dn8)) * var_t3) + (assign9910_e9396 * var_t3_dn8)) * var_t3) + (assign9910_e9398 * var_t3_dn8)) } else { (-((((((var_qsqrt_dn8 * var_t1) + (var_qsqrt * var_t1_dn8)) * var_t3) + (assign9910_e9396 * var_t3_dn8)) * var_t3) + (assign9910_e9398 * var_t3_dn8))) } / assign9910_e9401));

        let assign9920_e9407: f64 = (var_auxb1 + var_qcoth);
        let assign9920_e9410: f64 = (var_k2 * var_q2);
        let assign9920_e9412: f64 = (assign9920_e9410 + var_auxb1);
        let assign9920_e9413: f64 = (assign9920_e9407 * assign9920_e9412);
        let assign9920_e9414: f64 = (var_aaux + assign9920_e9413);
        var_f = assign9920_e9414;
        var_f_dn3 = (var_aaux_dn3 + (((var_auxb1_dn3 + var_qcoth_dn3) * assign9920_e9412) + (assign9920_e9407 * ((var_k2 * var_q2_dn3) + var_auxb1_dn3))));
        var_f_dn4 = (var_aaux_dn4 + (((var_auxb1_dn4 + var_qcoth_dn4) * assign9920_e9412) + (assign9920_e9407 * ((var_k2 * var_q2_dn4) + var_auxb1_dn4))));
        var_f_dn5 = (var_aaux_dn5 + (((var_auxb1_dn5 + var_qcoth_dn5) * assign9920_e9412) + (assign9920_e9407 * ((var_k2 * var_q2_dn5) + var_auxb1_dn5))));
        var_f_dn6 = (var_aaux_dn6 + (((var_auxb1_dn6 + var_qcoth_dn6) * assign9920_e9412) + (assign9920_e9407 * ((var_k2 * var_q2_dn6) + var_auxb1_dn6))));
        var_f_dn7 = (var_aaux_dn7 + (((var_auxb1_dn7 + var_qcoth_dn7) * assign9920_e9412) + (assign9920_e9407 * ((var_k2 * var_q2_dn7) + var_auxb1_dn7))));
        var_f_dn8 = (var_aaux_dn8 + (((var_auxb1_dn8 + var_qcoth_dn8) * assign9920_e9412) + (assign9920_e9407 * ((var_k2 * var_q2_dn8) + var_auxb1_dn8))));

        let assign9930_e9417: f64 = (1.0 / var_qsqrt);
        let assign9930_e9419: f64 = (assign9930_e9417 - var_t0);
        var_dlogsinhqsqdqsqrt = assign9930_e9419;
        var_dlogsinhqsqdqsqrt_dn3 = ((-(var_qsqrt_dn3 / (var_qsqrt * var_qsqrt))) - var_t0_dn3);
        var_dlogsinhqsqdqsqrt_dn4 = ((-(var_qsqrt_dn4 / (var_qsqrt * var_qsqrt))) - var_t0_dn4);
        var_dlogsinhqsqdqsqrt_dn5 = ((-(var_qsqrt_dn5 / (var_qsqrt * var_qsqrt))) - var_t0_dn5);
        var_dlogsinhqsqdqsqrt_dn6 = ((-(var_qsqrt_dn6 / (var_qsqrt * var_qsqrt))) - var_t0_dn6);
        var_dlogsinhqsqdqsqrt_dn7 = ((-(var_qsqrt_dn7 / (var_qsqrt * var_qsqrt))) - var_t0_dn7);
        var_dlogsinhqsqdqsqrt_dn8 = ((-(var_qsqrt_dn8 / (var_qsqrt * var_qsqrt))) - var_t0_dn8);

        let assign9940_e9421: f64 = (-2.0);
        let assign9940_e9423: f64 = (assign9940_e9421 * var_k1);
        let assign9940_e9425: f64 = (assign9940_e9423 * var_auxb1);
        let assign9940_e9427: f64 = (assign9940_e9425 + var_aaux);
        var_dqsqrt = assign9940_e9427;
        var_dqsqrt_dn3 = ((assign9940_e9423 * var_auxb1_dn3) + var_aaux_dn3);
        var_dqsqrt_dn4 = ((assign9940_e9423 * var_auxb1_dn4) + var_aaux_dn4);
        var_dqsqrt_dn5 = ((assign9940_e9423 * var_auxb1_dn5) + var_aaux_dn5);
        var_dqsqrt_dn6 = ((assign9940_e9423 * var_auxb1_dn6) + var_aaux_dn6);
        var_dqsqrt_dn7 = ((assign9940_e9423 * var_auxb1_dn7) + var_aaux_dn7);
        var_dqsqrt_dn8 = ((assign9940_e9423 * var_auxb1_dn8) + var_aaux_dn8);

        let assign9950_e9430: f64 = (var_dqcothqdqsqrt * var_dqsqrt);
        var_dqcoth = assign9950_e9430;
        var_dqcoth_dn3 = ((var_dqcothqdqsqrt_dn3 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn3));
        var_dqcoth_dn4 = ((var_dqcothqdqsqrt_dn4 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn4));
        var_dqcoth_dn5 = ((var_dqcothqdqsqrt_dn5 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn5));
        var_dqcoth_dn6 = ((var_dqcothqdqsqrt_dn6 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn6));
        var_dqcoth_dn7 = ((var_dqcothqdqsqrt_dn7 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn7));
        var_dqcoth_dn8 = ((var_dqcothqdqsqrt_dn8 * var_dqsqrt) + (var_dqcothqdqsqrt * var_dqsqrt_dn8));

        let assign9960_e9432: f64 = (-1.0);
        let assign9960_e9435: f64 = (-var_k1);
        let assign9960_e9437: f64 = (assign9960_e9435 + var_dqcoth);
        let assign9960_e9439: f64 = (assign9960_e9437 * var_t3);
        let assign9960_e9440: f64 = (2.0 * assign9960_e9439);
        let assign9960_e9441: f64 = (assign9960_e9432 + assign9960_e9440);
        let assign9960_e9444: f64 = (var_dlogsinhqsqdqsqrt * var_dqsqrt);
        let assign9960_e9445: f64 = (assign9960_e9441 - assign9960_e9444);
        var_dq2 = assign9960_e9445;
        var_dq2_dn3 = ((2.0 * ((var_dqcoth_dn3 * var_t3) + (assign9960_e9437 * var_t3_dn3))) - ((var_dlogsinhqsqdqsqrt_dn3 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn3)));
        var_dq2_dn4 = ((2.0 * ((var_dqcoth_dn4 * var_t3) + (assign9960_e9437 * var_t3_dn4))) - ((var_dlogsinhqsqdqsqrt_dn4 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn4)));
        var_dq2_dn5 = ((2.0 * ((var_dqcoth_dn5 * var_t3) + (assign9960_e9437 * var_t3_dn5))) - ((var_dlogsinhqsqdqsqrt_dn5 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn5)));
        var_dq2_dn6 = ((2.0 * ((var_dqcoth_dn6 * var_t3) + (assign9960_e9437 * var_t3_dn6))) - ((var_dlogsinhqsqdqsqrt_dn6 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn6)));
        var_dq2_dn7 = ((2.0 * ((var_dqcoth_dn7 * var_t3) + (assign9960_e9437 * var_t3_dn7))) - ((var_dlogsinhqsqdqsqrt_dn7 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn7)));
        var_dq2_dn8 = ((2.0 * ((var_dqcoth_dn8 * var_t3) + (assign9960_e9437 * var_t3_dn8))) - ((var_dlogsinhqsqdqsqrt_dn8 * var_dqsqrt) + (var_dlogsinhqsqdqsqrt * var_dqsqrt_dn8)));

        let assign9970_e9450: f64 = (var_auxb1 + var_t2);
        let assign9970_e9451: f64 = (var_k1 * assign9970_e9450);
        let assign9970_e9452: f64 = (var_aaux - assign9970_e9451);
        let assign9970_e9455: f64 = (var_auxb1 * var_dqcoth);
        let assign9970_e9456: f64 = (assign9970_e9452 + assign9970_e9455);
        let assign9970_e9460: f64 = (var_dq2 * var_t2);
        let assign9970_e9464: f64 = (var_dqcoth - var_k1);
        let assign9970_e9465: f64 = (var_q2 * assign9970_e9464);
        let assign9970_e9466: f64 = (assign9970_e9460 + assign9970_e9465);
        let assign9970_e9467: f64 = (var_k2 * assign9970_e9466);
        let assign9970_e9468: f64 = (assign9970_e9456 + assign9970_e9467);
        var_df = assign9970_e9468;
        var_df_dn3 = (((var_aaux_dn3 - (var_k1 * (var_auxb1_dn3 + var_t2_dn3))) + ((var_auxb1_dn3 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn3))) + (var_k2 * (((var_dq2_dn3 * var_t2) + (var_dq2 * var_t2_dn3)) + ((var_q2_dn3 * assign9970_e9464) + (var_q2 * var_dqcoth_dn3)))));
        var_df_dn4 = (((var_aaux_dn4 - (var_k1 * (var_auxb1_dn4 + var_t2_dn4))) + ((var_auxb1_dn4 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn4))) + (var_k2 * (((var_dq2_dn4 * var_t2) + (var_dq2 * var_t2_dn4)) + ((var_q2_dn4 * assign9970_e9464) + (var_q2 * var_dqcoth_dn4)))));
        var_df_dn5 = (((var_aaux_dn5 - (var_k1 * (var_auxb1_dn5 + var_t2_dn5))) + ((var_auxb1_dn5 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn5))) + (var_k2 * (((var_dq2_dn5 * var_t2) + (var_dq2 * var_t2_dn5)) + ((var_q2_dn5 * assign9970_e9464) + (var_q2 * var_dqcoth_dn5)))));
        var_df_dn6 = (((var_aaux_dn6 - (var_k1 * (var_auxb1_dn6 + var_t2_dn6))) + ((var_auxb1_dn6 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn6))) + (var_k2 * (((var_dq2_dn6 * var_t2) + (var_dq2 * var_t2_dn6)) + ((var_q2_dn6 * assign9970_e9464) + (var_q2 * var_dqcoth_dn6)))));
        var_df_dn7 = (((var_aaux_dn7 - (var_k1 * (var_auxb1_dn7 + var_t2_dn7))) + ((var_auxb1_dn7 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn7))) + (var_k2 * (((var_dq2_dn7 * var_t2) + (var_dq2 * var_t2_dn7)) + ((var_q2_dn7 * assign9970_e9464) + (var_q2 * var_dqcoth_dn7)))));
        var_df_dn8 = (((var_aaux_dn8 - (var_k1 * (var_auxb1_dn8 + var_t2_dn8))) + ((var_auxb1_dn8 * var_dqcoth) + (var_auxb1 * var_dqcoth_dn8))) + (var_k2 * (((var_dq2_dn8 * var_t2) + (var_dq2 * var_t2_dn8)) + ((var_q2_dn8 * assign9970_e9464) + (var_q2 * var_dqcoth_dn8)))));

        *var_aaux_slot = var_aaux;
        *var_aaux_dn3_slot = var_aaux_dn3;
        *var_aaux_dn4_slot = var_aaux_dn4;
        *var_aaux_dn5_slot = var_aaux_dn5;
        *var_aaux_dn6_slot = var_aaux_dn6;
        *var_aaux_dn7_slot = var_aaux_dn7;
        *var_aaux_dn8_slot = var_aaux_dn8;
        *var_auxb1_slot = var_auxb1;
        *var_auxb1_dn3_slot = var_auxb1_dn3;
        *var_auxb1_dn4_slot = var_auxb1_dn4;
        *var_auxb1_dn5_slot = var_auxb1_dn5;
        *var_auxb1_dn6_slot = var_auxb1_dn6;
        *var_auxb1_dn7_slot = var_auxb1_dn7;
        *var_auxb1_dn8_slot = var_auxb1_dn8;
        *var_coth1_slot = var_coth1;
        *var_coth1_dn3_slot = var_coth1_dn3;
        *var_coth1_dn4_slot = var_coth1_dn4;
        *var_coth1_dn5_slot = var_coth1_dn5;
        *var_coth1_dn6_slot = var_coth1_dn6;
        *var_coth1_dn7_slot = var_coth1_dn7;
        *var_coth1_dn8_slot = var_coth1_dn8;
        *var_csc1_slot = var_csc1;
        *var_csc1_dn3_slot = var_csc1_dn3;
        *var_csc1_dn4_slot = var_csc1_dn4;
        *var_csc1_dn5_slot = var_csc1_dn5;
        *var_csc1_dn6_slot = var_csc1_dn6;
        *var_csc1_dn7_slot = var_csc1_dn7;
        *var_csc1_dn8_slot = var_csc1_dn8;
        *var_delta_slot = var_delta;
        *var_delta_dn3_slot = var_delta_dn3;
        *var_delta_dn4_slot = var_delta_dn4;
        *var_delta_dn5_slot = var_delta_dn5;
        *var_delta_dn6_slot = var_delta_dn6;
        *var_delta_dn7_slot = var_delta_dn7;
        *var_delta_dn8_slot = var_delta_dn8;
        *var_df_slot = var_df;
        *var_df_dn3_slot = var_df_dn3;
        *var_df_dn4_slot = var_df_dn4;
        *var_df_dn5_slot = var_df_dn5;
        *var_df_dn6_slot = var_df_dn6;
        *var_df_dn7_slot = var_df_dn7;
        *var_df_dn8_slot = var_df_dn8;
        *var_dlogsinhqsqdqsqrt_slot = var_dlogsinhqsqdqsqrt;
        *var_dlogsinhqsqdqsqrt_dn3_slot = var_dlogsinhqsqdqsqrt_dn3;
        *var_dlogsinhqsqdqsqrt_dn4_slot = var_dlogsinhqsqdqsqrt_dn4;
        *var_dlogsinhqsqdqsqrt_dn5_slot = var_dlogsinhqsqdqsqrt_dn5;
        *var_dlogsinhqsqdqsqrt_dn6_slot = var_dlogsinhqsqdqsqrt_dn6;
        *var_dlogsinhqsqdqsqrt_dn7_slot = var_dlogsinhqsqdqsqrt_dn7;
        *var_dlogsinhqsqdqsqrt_dn8_slot = var_dlogsinhqsqdqsqrt_dn8;
        *var_dq2_slot = var_dq2;
        *var_dq2_dn3_slot = var_dq2_dn3;
        *var_dq2_dn4_slot = var_dq2_dn4;
        *var_dq2_dn5_slot = var_dq2_dn5;
        *var_dq2_dn6_slot = var_dq2_dn6;
        *var_dq2_dn7_slot = var_dq2_dn7;
        *var_dq2_dn8_slot = var_dq2_dn8;
        *var_dqcoth_slot = var_dqcoth;
        *var_dqcoth_dn3_slot = var_dqcoth_dn3;
        *var_dqcoth_dn4_slot = var_dqcoth_dn4;
        *var_dqcoth_dn5_slot = var_dqcoth_dn5;
        *var_dqcoth_dn6_slot = var_dqcoth_dn6;
        *var_dqcoth_dn7_slot = var_dqcoth_dn7;
        *var_dqcoth_dn8_slot = var_dqcoth_dn8;
        *var_dqcothqdqsqrt_slot = var_dqcothqdqsqrt;
        *var_dqcothqdqsqrt_dn3_slot = var_dqcothqdqsqrt_dn3;
        *var_dqcothqdqsqrt_dn4_slot = var_dqcothqdqsqrt_dn4;
        *var_dqcothqdqsqrt_dn5_slot = var_dqcothqdqsqrt_dn5;
        *var_dqcothqdqsqrt_dn6_slot = var_dqcothqdqsqrt_dn6;
        *var_dqcothqdqsqrt_dn7_slot = var_dqcothqdqsqrt_dn7;
        *var_dqcothqdqsqrt_dn8_slot = var_dqcothqdqsqrt_dn8;
        *var_dqsqrt_slot = var_dqsqrt;
        *var_dqsqrt_dn3_slot = var_dqsqrt_dn3;
        *var_dqsqrt_dn4_slot = var_dqsqrt_dn4;
        *var_dqsqrt_dn5_slot = var_dqsqrt_dn5;
        *var_dqsqrt_dn6_slot = var_dqsqrt_dn6;
        *var_dqsqrt_dn7_slot = var_dqsqrt_dn7;
        *var_dqsqrt_dn8_slot = var_dqsqrt_dn8;
        *var_f_slot = var_f;
        *var_f_dn3_slot = var_f_dn3;
        *var_f_dn4_slot = var_f_dn4;
        *var_f_dn5_slot = var_f_dn5;
        *var_f_dn6_slot = var_f_dn6;
        *var_f_dn7_slot = var_f_dn7;
        *var_f_dn8_slot = var_f_dn8;
        *var_guard96_slot = var_guard96;
        *var_phi1_slot = var_phi1;
        *var_phi1_dn3_slot = var_phi1_dn3;
        *var_phi1_dn4_slot = var_phi1_dn4;
        *var_phi1_dn5_slot = var_phi1_dn5;
        *var_phi1_dn6_slot = var_phi1_dn6;
        *var_phi1_dn7_slot = var_phi1_dn7;
        *var_phi1_dn8_slot = var_phi1_dn8;
        *var_q_slot = var_q;
        *var_q1_slot = var_q1;
        *var_q1_dn3_slot = var_q1_dn3;
        *var_q1_dn4_slot = var_q1_dn4;
        *var_q1_dn5_slot = var_q1_dn5;
        *var_q1_dn6_slot = var_q1_dn6;
        *var_q1_dn7_slot = var_q1_dn7;
        *var_q1_dn8_slot = var_q1_dn8;
        *var_q2_slot = var_q2;
        *var_q2_dn3_slot = var_q2_dn3;
        *var_q2_dn4_slot = var_q2_dn4;
        *var_q2_dn5_slot = var_q2_dn5;
        *var_q2_dn6_slot = var_q2_dn6;
        *var_q2_dn7_slot = var_q2_dn7;
        *var_q2_dn8_slot = var_q2_dn8;
        *var_q_dn3_slot = var_q_dn3;
        *var_q_dn4_slot = var_q_dn4;
        *var_q_dn5_slot = var_q_dn5;
        *var_q_dn6_slot = var_q_dn6;
        *var_q_dn7_slot = var_q_dn7;
        *var_q_dn8_slot = var_q_dn8;
        *var_qcoth_slot = var_qcoth;
        *var_qcoth_dn3_slot = var_qcoth_dn3;
        *var_qcoth_dn4_slot = var_qcoth_dn4;
        *var_qcoth_dn5_slot = var_qcoth_dn5;
        *var_qcoth_dn6_slot = var_qcoth_dn6;
        *var_qcoth_dn7_slot = var_qcoth_dn7;
        *var_qcoth_dn8_slot = var_qcoth_dn8;
        *var_qsqrt_slot = var_qsqrt;
        *var_qsqrt_dn3_slot = var_qsqrt_dn3;
        *var_qsqrt_dn4_slot = var_qsqrt_dn4;
        *var_qsqrt_dn5_slot = var_qsqrt_dn5;
        *var_qsqrt_dn6_slot = var_qsqrt_dn6;
        *var_qsqrt_dn7_slot = var_qsqrt_dn7;
        *var_qsqrt_dn8_slot = var_qsqrt_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
    }

    pub(super) fn stamp_transient_block_21(
        p: &Parameters,
        var_a0: f64,
        var_a0_dn3: f64,
        var_a0_dn4: f64,
        var_a0_dn5: f64,
        var_a0_dn6: f64,
        var_a0_dn7: f64,
        var_a0_dn8: f64,
        var_cox1: f64,
        var_cox2: f64,
        var_csi: f64,
        var_df: f64,
        var_df_dn3: f64,
        var_df_dn4: f64,
        var_df_dn5: f64,
        var_df_dn6: f64,
        var_df_dn7: f64,
        var_df_dn8: f64,
        var_eefffactor: f64,
        var_eefffactor2: f64,
        var_eta_mu: f64,
        var_eta_mu2: f64,
        var_eu_i: f64,
        var_eub_i: f64,
        var_f: f64,
        var_f_dn3: f64,
        var_f_dn4: f64,
        var_f_dn5: f64,
        var_f_dn6: f64,
        var_f_dn7: f64,
        var_f_dn8: f64,
        var_k1: f64,
        var_k1_2: f64,
        var_nbody_i: f64,
        var_nvtm: f64,
        var_nvtm_dn3: f64,
        var_nvtm_dn4: f64,
        var_nvtm_dn5: f64,
        var_nvtm_dn6: f64,
        var_nvtm_dn7: f64,
        var_nvtm_dn8: f64,
        var_qb0: f64,
        var_qbacks: f64,
        var_qbacks_dn3: f64,
        var_qbacks_dn4: f64,
        var_qbacks_dn5: f64,
        var_qbacks_dn6: f64,
        var_qbacks_dn7: f64,
        var_qbacks_dn8: f64,
        var_qfronts: f64,
        var_qfronts_dn3: f64,
        var_qfronts_dn4: f64,
        var_qfronts_dn5: f64,
        var_qfronts_dn6: f64,
        var_qfronts_dn7: f64,
        var_qfronts_dn8: f64,
        var_qis: f64,
        var_qis_dn3: f64,
        var_qis_dn4: f64,
        var_qis_dn5: f64,
        var_qis_dn6: f64,
        var_qis_dn7: f64,
        var_qis_dn8: f64,
        var_ua_t: f64,
        var_ua_t_dn4: f64,
        var_uc_t: f64,
        var_uc_t_dn4: f64,
        var_ucs_t: f64,
        var_ucs_t_dn4: f64,
        var_ud_t: f64,
        var_ud_t_dn4: f64,
        var_udb_i: f64,
        var_vbgx: f64,
        var_vbgx_dn3: f64,
        var_vbgx_dn5: f64,
        var_vbgx_dn6: f64,
        var_vdseff: f64,
        var_vdseff_dn3: f64,
        var_vdseff_dn4: f64,
        var_vdseff_dn5: f64,
        var_vdseff_dn6: f64,
        var_vdseff_dn7: f64,
        var_vdseff_dn8: f64,
        var_xg1: f64,
        var_xg1_dn3: f64,
        var_xg1_dn4: f64,
        var_xg1_dn5: f64,
        var_xg1_dn6: f64,
        var_xg1_dn7: f64,
        var_xg1_dn8: f64,
        var_xg2: f64,
        var_xg2_dn3: f64,
        var_xg2_dn4: f64,
        var_xg2_dn5: f64,
        var_xg2_dn6: f64,
        var_xg2_dn7: f64,
        var_xg2_dn8: f64,
        var_delta_slot: &mut f64,
        var_delta_dn3_slot: &mut f64,
        var_delta_dn4_slot: &mut f64,
        var_delta_dn5_slot: &mut f64,
        var_delta_dn6_slot: &mut f64,
        var_delta_dn7_slot: &mut f64,
        var_delta_dn8_slot: &mut f64,
        var_dqi_slot: &mut f64,
        var_dqi_dn3_slot: &mut f64,
        var_dqi_dn4_slot: &mut f64,
        var_dqi_dn5_slot: &mut f64,
        var_dqi_dn6_slot: &mut f64,
        var_dqi_dn7_slot: &mut f64,
        var_dqi_dn8_slot: &mut f64,
        var_eeffm_slot: &mut f64,
        var_eeffm2_slot: &mut f64,
        var_eeffm2_dn3_slot: &mut f64,
        var_eeffm2_dn4_slot: &mut f64,
        var_eeffm2_dn5_slot: &mut f64,
        var_eeffm2_dn6_slot: &mut f64,
        var_eeffm2_dn7_slot: &mut f64,
        var_eeffm2_dn8_slot: &mut f64,
        var_eeffm_dn3_slot: &mut f64,
        var_eeffm_dn4_slot: &mut f64,
        var_eeffm_dn5_slot: &mut f64,
        var_eeffm_dn6_slot: &mut f64,
        var_eeffm_dn7_slot: &mut f64,
        var_eeffm_dn8_slot: &mut f64,
        var_guard97_slot: &mut f64,
        var_guard98_slot: &mut f64,
        var_guard99_slot: &mut f64,
        var_phi1_slot: &mut f64,
        var_phi1_dn3_slot: &mut f64,
        var_phi1_dn4_slot: &mut f64,
        var_phi1_dn5_slot: &mut f64,
        var_phi1_dn6_slot: &mut f64,
        var_phi1_dn7_slot: &mut f64,
        var_phi1_dn8_slot: &mut f64,
        var_phi2_slot: &mut f64,
        var_phi2_dn3_slot: &mut f64,
        var_phi2_dn4_slot: &mut f64,
        var_phi2_dn5_slot: &mut f64,
        var_phi2_dn6_slot: &mut f64,
        var_phi2_dn7_slot: &mut f64,
        var_phi2_dn8_slot: &mut f64,
        var_q_slot: &mut f64,
        var_q1_slot: &mut f64,
        var_q1_dn3_slot: &mut f64,
        var_q1_dn4_slot: &mut f64,
        var_q1_dn5_slot: &mut f64,
        var_q1_dn6_slot: &mut f64,
        var_q1_dn7_slot: &mut f64,
        var_q1_dn8_slot: &mut f64,
        var_q_dn3_slot: &mut f64,
        var_q_dn4_slot: &mut f64,
        var_q_dn5_slot: &mut f64,
        var_q_dn6_slot: &mut f64,
        var_q_dn7_slot: &mut f64,
        var_q_dn8_slot: &mut f64,
        var_qba_slot: &mut f64,
        var_qbackd_slot: &mut f64,
        var_qbackd_dn3_slot: &mut f64,
        var_qbackd_dn4_slot: &mut f64,
        var_qbackd_dn5_slot: &mut f64,
        var_qbackd_dn6_slot: &mut f64,
        var_qbackd_dn7_slot: &mut f64,
        var_qbackd_dn8_slot: &mut f64,
        var_qcoth_slot: &mut f64,
        var_qcoth_dn3_slot: &mut f64,
        var_qcoth_dn4_slot: &mut f64,
        var_qcoth_dn5_slot: &mut f64,
        var_qcoth_dn6_slot: &mut f64,
        var_qcoth_dn7_slot: &mut f64,
        var_qcoth_dn8_slot: &mut f64,
        var_qfrontd_slot: &mut f64,
        var_qfrontd_dn3_slot: &mut f64,
        var_qfrontd_dn4_slot: &mut f64,
        var_qfrontd_dn5_slot: &mut f64,
        var_qfrontd_dn6_slot: &mut f64,
        var_qfrontd_dn7_slot: &mut f64,
        var_qfrontd_dn8_slot: &mut f64,
        var_qia_slot: &mut f64,
        var_qia2_slot: &mut f64,
        var_qia2_dn3_slot: &mut f64,
        var_qia2_dn4_slot: &mut f64,
        var_qia2_dn5_slot: &mut f64,
        var_qia2_dn6_slot: &mut f64,
        var_qia2_dn7_slot: &mut f64,
        var_qia2_dn8_slot: &mut f64,
        var_qia_dn3_slot: &mut f64,
        var_qia_dn4_slot: &mut f64,
        var_qia_dn5_slot: &mut f64,
        var_qia_dn6_slot: &mut f64,
        var_qia_dn7_slot: &mut f64,
        var_qia_dn8_slot: &mut f64,
        var_qib2_slot: &mut f64,
        var_qib2_dn3_slot: &mut f64,
        var_qib2_dn4_slot: &mut f64,
        var_qib2_dn5_slot: &mut f64,
        var_qib2_dn6_slot: &mut f64,
        var_qib2_dn7_slot: &mut f64,
        var_qib2_dn8_slot: &mut f64,
        var_qicored_slot: &mut f64,
        var_qicored_dn3_slot: &mut f64,
        var_qicored_dn4_slot: &mut f64,
        var_qicored_dn5_slot: &mut f64,
        var_qicored_dn6_slot: &mut f64,
        var_qicored_dn7_slot: &mut f64,
        var_qicored_dn8_slot: &mut f64,
        var_qid_slot: &mut f64,
        var_qid_dn3_slot: &mut f64,
        var_qid_dn4_slot: &mut f64,
        var_qid_dn5_slot: &mut f64,
        var_qid_dn6_slot: &mut f64,
        var_qid_dn7_slot: &mut f64,
        var_qid_dn8_slot: &mut f64,
        var_qsqrt_slot: &mut f64,
        var_qsqrt_dn3_slot: &mut f64,
        var_qsqrt_dn4_slot: &mut f64,
        var_qsqrt_dn5_slot: &mut f64,
        var_qsqrt_dn6_slot: &mut f64,
        var_qsqrt_dn7_slot: &mut f64,
        var_qsqrt_dn8_slot: &mut f64,
        var_qtotd_slot: &mut f64,
        var_qtotd_dn3_slot: &mut f64,
        var_qtotd_dn4_slot: &mut f64,
        var_qtotd_dn5_slot: &mut f64,
        var_qtotd_dn6_slot: &mut f64,
        var_qtotd_dn7_slot: &mut f64,
        var_qtotd_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2__blk100_slot: &mut f64,
        var_t2__blk100_dn3_slot: &mut f64,
        var_t2__blk100_dn4_slot: &mut f64,
        var_t2__blk100_dn5_slot: &mut f64,
        var_t2__blk100_dn6_slot: &mut f64,
        var_t2__blk100_dn7_slot: &mut f64,
        var_t2__blk100_dn8_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3__blk101_slot: &mut f64,
        var_t3__blk101_dn3_slot: &mut f64,
        var_t3__blk101_dn4_slot: &mut f64,
        var_t3__blk101_dn5_slot: &mut f64,
        var_t3__blk101_dn6_slot: &mut f64,
        var_t3__blk101_dn7_slot: &mut f64,
        var_t3__blk101_dn8_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
    ) {
        let mut var_delta: f64 = *var_delta_slot;
        let mut var_delta_dn3: f64 = *var_delta_dn3_slot;
        let mut var_delta_dn4: f64 = *var_delta_dn4_slot;
        let mut var_delta_dn5: f64 = *var_delta_dn5_slot;
        let mut var_delta_dn6: f64 = *var_delta_dn6_slot;
        let mut var_delta_dn7: f64 = *var_delta_dn7_slot;
        let mut var_delta_dn8: f64 = *var_delta_dn8_slot;
        let mut var_dqi: f64 = *var_dqi_slot;
        let mut var_dqi_dn3: f64 = *var_dqi_dn3_slot;
        let mut var_dqi_dn4: f64 = *var_dqi_dn4_slot;
        let mut var_dqi_dn5: f64 = *var_dqi_dn5_slot;
        let mut var_dqi_dn6: f64 = *var_dqi_dn6_slot;
        let mut var_dqi_dn7: f64 = *var_dqi_dn7_slot;
        let mut var_dqi_dn8: f64 = *var_dqi_dn8_slot;
        let mut var_eeffm: f64 = *var_eeffm_slot;
        let mut var_eeffm2: f64 = *var_eeffm2_slot;
        let mut var_eeffm2_dn3: f64 = *var_eeffm2_dn3_slot;
        let mut var_eeffm2_dn4: f64 = *var_eeffm2_dn4_slot;
        let mut var_eeffm2_dn5: f64 = *var_eeffm2_dn5_slot;
        let mut var_eeffm2_dn6: f64 = *var_eeffm2_dn6_slot;
        let mut var_eeffm2_dn7: f64 = *var_eeffm2_dn7_slot;
        let mut var_eeffm2_dn8: f64 = *var_eeffm2_dn8_slot;
        let mut var_eeffm_dn3: f64 = *var_eeffm_dn3_slot;
        let mut var_eeffm_dn4: f64 = *var_eeffm_dn4_slot;
        let mut var_eeffm_dn5: f64 = *var_eeffm_dn5_slot;
        let mut var_eeffm_dn6: f64 = *var_eeffm_dn6_slot;
        let mut var_eeffm_dn7: f64 = *var_eeffm_dn7_slot;
        let mut var_eeffm_dn8: f64 = *var_eeffm_dn8_slot;
        let mut var_guard97: f64 = *var_guard97_slot;
        let mut var_guard98: f64 = *var_guard98_slot;
        let mut var_guard99: f64 = *var_guard99_slot;
        let mut var_phi1: f64 = *var_phi1_slot;
        let mut var_phi1_dn3: f64 = *var_phi1_dn3_slot;
        let mut var_phi1_dn4: f64 = *var_phi1_dn4_slot;
        let mut var_phi1_dn5: f64 = *var_phi1_dn5_slot;
        let mut var_phi1_dn6: f64 = *var_phi1_dn6_slot;
        let mut var_phi1_dn7: f64 = *var_phi1_dn7_slot;
        let mut var_phi1_dn8: f64 = *var_phi1_dn8_slot;
        let mut var_phi2: f64 = *var_phi2_slot;
        let mut var_phi2_dn3: f64 = *var_phi2_dn3_slot;
        let mut var_phi2_dn4: f64 = *var_phi2_dn4_slot;
        let mut var_phi2_dn5: f64 = *var_phi2_dn5_slot;
        let mut var_phi2_dn6: f64 = *var_phi2_dn6_slot;
        let mut var_phi2_dn7: f64 = *var_phi2_dn7_slot;
        let mut var_phi2_dn8: f64 = *var_phi2_dn8_slot;
        let mut var_q: f64 = *var_q_slot;
        let mut var_q1: f64 = *var_q1_slot;
        let mut var_q1_dn3: f64 = *var_q1_dn3_slot;
        let mut var_q1_dn4: f64 = *var_q1_dn4_slot;
        let mut var_q1_dn5: f64 = *var_q1_dn5_slot;
        let mut var_q1_dn6: f64 = *var_q1_dn6_slot;
        let mut var_q1_dn7: f64 = *var_q1_dn7_slot;
        let mut var_q1_dn8: f64 = *var_q1_dn8_slot;
        let mut var_q_dn3: f64 = *var_q_dn3_slot;
        let mut var_q_dn4: f64 = *var_q_dn4_slot;
        let mut var_q_dn5: f64 = *var_q_dn5_slot;
        let mut var_q_dn6: f64 = *var_q_dn6_slot;
        let mut var_q_dn7: f64 = *var_q_dn7_slot;
        let mut var_q_dn8: f64 = *var_q_dn8_slot;
        let mut var_qba: f64 = *var_qba_slot;
        let mut var_qbackd: f64 = *var_qbackd_slot;
        let mut var_qbackd_dn3: f64 = *var_qbackd_dn3_slot;
        let mut var_qbackd_dn4: f64 = *var_qbackd_dn4_slot;
        let mut var_qbackd_dn5: f64 = *var_qbackd_dn5_slot;
        let mut var_qbackd_dn6: f64 = *var_qbackd_dn6_slot;
        let mut var_qbackd_dn7: f64 = *var_qbackd_dn7_slot;
        let mut var_qbackd_dn8: f64 = *var_qbackd_dn8_slot;
        let mut var_qcoth: f64 = *var_qcoth_slot;
        let mut var_qcoth_dn3: f64 = *var_qcoth_dn3_slot;
        let mut var_qcoth_dn4: f64 = *var_qcoth_dn4_slot;
        let mut var_qcoth_dn5: f64 = *var_qcoth_dn5_slot;
        let mut var_qcoth_dn6: f64 = *var_qcoth_dn6_slot;
        let mut var_qcoth_dn7: f64 = *var_qcoth_dn7_slot;
        let mut var_qcoth_dn8: f64 = *var_qcoth_dn8_slot;
        let mut var_qfrontd: f64 = *var_qfrontd_slot;
        let mut var_qfrontd_dn3: f64 = *var_qfrontd_dn3_slot;
        let mut var_qfrontd_dn4: f64 = *var_qfrontd_dn4_slot;
        let mut var_qfrontd_dn5: f64 = *var_qfrontd_dn5_slot;
        let mut var_qfrontd_dn6: f64 = *var_qfrontd_dn6_slot;
        let mut var_qfrontd_dn7: f64 = *var_qfrontd_dn7_slot;
        let mut var_qfrontd_dn8: f64 = *var_qfrontd_dn8_slot;
        let mut var_qia: f64 = *var_qia_slot;
        let mut var_qia2: f64 = *var_qia2_slot;
        let mut var_qia2_dn3: f64 = *var_qia2_dn3_slot;
        let mut var_qia2_dn4: f64 = *var_qia2_dn4_slot;
        let mut var_qia2_dn5: f64 = *var_qia2_dn5_slot;
        let mut var_qia2_dn6: f64 = *var_qia2_dn6_slot;
        let mut var_qia2_dn7: f64 = *var_qia2_dn7_slot;
        let mut var_qia2_dn8: f64 = *var_qia2_dn8_slot;
        let mut var_qia_dn3: f64 = *var_qia_dn3_slot;
        let mut var_qia_dn4: f64 = *var_qia_dn4_slot;
        let mut var_qia_dn5: f64 = *var_qia_dn5_slot;
        let mut var_qia_dn6: f64 = *var_qia_dn6_slot;
        let mut var_qia_dn7: f64 = *var_qia_dn7_slot;
        let mut var_qia_dn8: f64 = *var_qia_dn8_slot;
        let mut var_qib2: f64 = *var_qib2_slot;
        let mut var_qib2_dn3: f64 = *var_qib2_dn3_slot;
        let mut var_qib2_dn4: f64 = *var_qib2_dn4_slot;
        let mut var_qib2_dn5: f64 = *var_qib2_dn5_slot;
        let mut var_qib2_dn6: f64 = *var_qib2_dn6_slot;
        let mut var_qib2_dn7: f64 = *var_qib2_dn7_slot;
        let mut var_qib2_dn8: f64 = *var_qib2_dn8_slot;
        let mut var_qicored: f64 = *var_qicored_slot;
        let mut var_qicored_dn3: f64 = *var_qicored_dn3_slot;
        let mut var_qicored_dn4: f64 = *var_qicored_dn4_slot;
        let mut var_qicored_dn5: f64 = *var_qicored_dn5_slot;
        let mut var_qicored_dn6: f64 = *var_qicored_dn6_slot;
        let mut var_qicored_dn7: f64 = *var_qicored_dn7_slot;
        let mut var_qicored_dn8: f64 = *var_qicored_dn8_slot;
        let mut var_qid: f64 = *var_qid_slot;
        let mut var_qid_dn3: f64 = *var_qid_dn3_slot;
        let mut var_qid_dn4: f64 = *var_qid_dn4_slot;
        let mut var_qid_dn5: f64 = *var_qid_dn5_slot;
        let mut var_qid_dn6: f64 = *var_qid_dn6_slot;
        let mut var_qid_dn7: f64 = *var_qid_dn7_slot;
        let mut var_qid_dn8: f64 = *var_qid_dn8_slot;
        let mut var_qsqrt: f64 = *var_qsqrt_slot;
        let mut var_qsqrt_dn3: f64 = *var_qsqrt_dn3_slot;
        let mut var_qsqrt_dn4: f64 = *var_qsqrt_dn4_slot;
        let mut var_qsqrt_dn5: f64 = *var_qsqrt_dn5_slot;
        let mut var_qsqrt_dn6: f64 = *var_qsqrt_dn6_slot;
        let mut var_qsqrt_dn7: f64 = *var_qsqrt_dn7_slot;
        let mut var_qsqrt_dn8: f64 = *var_qsqrt_dn8_slot;
        let mut var_qtotd: f64 = *var_qtotd_slot;
        let mut var_qtotd_dn3: f64 = *var_qtotd_dn3_slot;
        let mut var_qtotd_dn4: f64 = *var_qtotd_dn4_slot;
        let mut var_qtotd_dn5: f64 = *var_qtotd_dn5_slot;
        let mut var_qtotd_dn6: f64 = *var_qtotd_dn6_slot;
        let mut var_qtotd_dn7: f64 = *var_qtotd_dn7_slot;
        let mut var_qtotd_dn8: f64 = *var_qtotd_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2__blk100: f64 = *var_t2__blk100_slot;
        let mut var_t2__blk100_dn3: f64 = *var_t2__blk100_dn3_slot;
        let mut var_t2__blk100_dn4: f64 = *var_t2__blk100_dn4_slot;
        let mut var_t2__blk100_dn5: f64 = *var_t2__blk100_dn5_slot;
        let mut var_t2__blk100_dn6: f64 = *var_t2__blk100_dn6_slot;
        let mut var_t2__blk100_dn7: f64 = *var_t2__blk100_dn7_slot;
        let mut var_t2__blk100_dn8: f64 = *var_t2__blk100_dn8_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3__blk101: f64 = *var_t3__blk101_slot;
        let mut var_t3__blk101_dn3: f64 = *var_t3__blk101_dn3_slot;
        let mut var_t3__blk101_dn4: f64 = *var_t3__blk101_dn4_slot;
        let mut var_t3__blk101_dn5: f64 = *var_t3__blk101_dn5_slot;
        let mut var_t3__blk101_dn6: f64 = *var_t3__blk101_dn6_slot;
        let mut var_t3__blk101_dn7: f64 = *var_t3__blk101_dn7_slot;
        let mut var_t3__blk101_dn8: f64 = *var_t3__blk101_dn8_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;

        let assign9980_e9470: f64 = (-var_f);
        let assign9980_e9472: f64 = (assign9980_e9470 / var_df);
        var_delta = assign9980_e9472;
        var_delta_dn3 = ((((-var_f_dn3) * var_df) - (assign9980_e9470 * var_df_dn3)) / (var_df * var_df));
        var_delta_dn4 = ((((-var_f_dn4) * var_df) - (assign9980_e9470 * var_df_dn4)) / (var_df * var_df));
        var_delta_dn5 = ((((-var_f_dn5) * var_df) - (assign9980_e9470 * var_df_dn5)) / (var_df * var_df));
        var_delta_dn6 = ((((-var_f_dn6) * var_df) - (assign9980_e9470 * var_df_dn6)) / (var_df * var_df));
        var_delta_dn7 = ((((-var_f_dn7) * var_df) - (assign9980_e9470 * var_df_dn7)) / (var_df * var_df));
        var_delta_dn8 = ((((-var_f_dn8) * var_df) - (assign9980_e9470 * var_df_dn8)) / (var_df * var_df));

        let assign9990_e9475: f64 = (var_phi1 + var_delta);
        var_phi1 = assign9990_e9475;
        var_phi1_dn3 = (var_phi1_dn3 + var_delta_dn3);
        var_phi1_dn4 = (var_phi1_dn4 + var_delta_dn4);
        var_phi1_dn5 = (var_phi1_dn5 + var_delta_dn5);
        var_phi1_dn6 = (var_phi1_dn6 + var_delta_dn6);
        var_phi1_dn7 = (var_phi1_dn7 + var_delta_dn7);
        var_phi1_dn8 = (var_phi1_dn8 + var_delta_dn8);

        let assign10000_e9478: f64 = (var_xg1 - var_phi1);
        var_q1 = assign10000_e9478;
        var_q1_dn3 = (var_xg1_dn3 - var_phi1_dn3);
        var_q1_dn4 = (var_xg1_dn4 - var_phi1_dn4);
        var_q1_dn5 = (var_xg1_dn5 - var_phi1_dn5);
        var_q1_dn6 = (var_xg1_dn6 - var_phi1_dn6);
        var_q1_dn7 = (var_xg1_dn7 - var_phi1_dn7);
        var_q1_dn8 = (var_xg1_dn8 - var_phi1_dn8);

        let assign10010_e9481: f64 = (var_phi1).exp();
        let assign10010_e9482: f64 = (var_a0 * assign10010_e9481);
        var_t0 = assign10010_e9482;
        var_t0_dn3 = ((var_a0_dn3 * assign10010_e9481) + (var_a0 * (assign10010_e9481 * var_phi1_dn3)));
        var_t0_dn4 = ((var_a0_dn4 * assign10010_e9481) + (var_a0 * (assign10010_e9481 * var_phi1_dn4)));
        var_t0_dn5 = ((var_a0_dn5 * assign10010_e9481) + (var_a0 * (assign10010_e9481 * var_phi1_dn5)));
        var_t0_dn6 = ((var_a0_dn6 * assign10010_e9481) + (var_a0 * (assign10010_e9481 * var_phi1_dn6)));
        var_t0_dn7 = ((var_a0_dn7 * assign10010_e9481) + (var_a0 * (assign10010_e9481 * var_phi1_dn7)));
        var_t0_dn8 = ((var_a0_dn8 * assign10010_e9481) + (var_a0 * (assign10010_e9481 * var_phi1_dn8)));

        let assign10020_e9485: f64 = (var_k1_2 * var_q1);
        let assign10020_e9487: f64 = (assign10020_e9485 * var_q1);
        let assign10020_e9489: f64 = (assign10020_e9487 - var_t0);
        var_qsqrt = assign10020_e9489;
        var_qsqrt_dn3 = ((((var_k1_2 * var_q1_dn3) * var_q1) + (assign10020_e9485 * var_q1_dn3)) - var_t0_dn3);
        var_qsqrt_dn4 = ((((var_k1_2 * var_q1_dn4) * var_q1) + (assign10020_e9485 * var_q1_dn4)) - var_t0_dn4);
        var_qsqrt_dn5 = ((((var_k1_2 * var_q1_dn5) * var_q1) + (assign10020_e9485 * var_q1_dn5)) - var_t0_dn5);
        var_qsqrt_dn6 = ((((var_k1_2 * var_q1_dn6) * var_q1) + (assign10020_e9485 * var_q1_dn6)) - var_t0_dn6);
        var_qsqrt_dn7 = ((((var_k1_2 * var_q1_dn7) * var_q1) + (assign10020_e9485 * var_q1_dn7)) - var_t0_dn7);
        var_qsqrt_dn8 = ((((var_k1_2 * var_q1_dn8) * var_q1) + (assign10020_e9485 * var_q1_dn8)) - var_t0_dn8);

        let assign10030_e9492: f64 = if var_qsqrt < 0.0 { 1.0 } else { 0.0 };
        var_guard97 = assign10030_e9492;

        let (assign10040_e9498, assign10040_e9498_d_n3, assign10040_e9498_d_n4, assign10040_e9498_d_n5, assign10040_e9498_d_n6, assign10040_e9498_d_n7, assign10040_e9498_d_n8,) = {
    if (var_guard97 != 0.0) {
        let assign10040_e9495: f64 = (-var_qsqrt);
        let assign10040_e9496: f64 = (assign10040_e9495).sqrt();
        (assign10040_e9496, ((-var_qsqrt_dn3) / (2.0 * assign10040_e9496)), ((-var_qsqrt_dn4) / (2.0 * assign10040_e9496)), ((-var_qsqrt_dn5) / (2.0 * assign10040_e9496)), ((-var_qsqrt_dn6) / (2.0 * assign10040_e9496)), ((-var_qsqrt_dn7) / (2.0 * assign10040_e9496)), ((-var_qsqrt_dn8) / (2.0 * assign10040_e9496)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign10040_e9498;
        var_q_dn3 = assign10040_e9498_d_n3;
        var_q_dn4 = assign10040_e9498_d_n4;
        var_q_dn5 = assign10040_e9498_d_n5;
        var_q_dn6 = assign10040_e9498_d_n6;
        var_q_dn7 = assign10040_e9498_d_n7;
        var_q_dn8 = assign10040_e9498_d_n8;

        let (assign10050_e9504, assign10050_e9504_d_n3, assign10050_e9504_d_n4, assign10050_e9504_d_n5, assign10050_e9504_d_n6, assign10050_e9504_d_n7, assign10050_e9504_d_n8,) = {
    if (var_guard97 != 0.0) {
        let assign10050_e9502: f64 = (0.5 * var_q);
        (assign10050_e9502, (0.5 * var_q_dn3), (0.5 * var_q_dn4), (0.5 * var_q_dn5), (0.5 * var_q_dn6), (0.5 * var_q_dn7), (0.5 * var_q_dn8),)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8,)
    }
};
        var_t2 = assign10050_e9504;
        var_t2_dn3 = assign10050_e9504_d_n3;
        var_t2_dn4 = assign10050_e9504_d_n4;
        var_t2_dn5 = assign10050_e9504_d_n5;
        var_t2_dn6 = assign10050_e9504_d_n6;
        var_t2_dn7 = assign10050_e9504_d_n7;
        var_t2_dn8 = assign10050_e9504_d_n8;

        let (assign10060_e9511, assign10060_e9511_d_n3, assign10060_e9511_d_n4, assign10060_e9511_d_n5, assign10060_e9511_d_n6, assign10060_e9511_d_n7, assign10060_e9511_d_n8,) = {
    if (var_guard97 != 0.0) {
        let assign10060_e9508: f64 = (var_t2).tan();
        let assign10060_e9509: f64 = (var_q / assign10060_e9508);
        (assign10060_e9509, (((var_q_dn3 * assign10060_e9508) - (var_q * (var_t2_dn3 / ((var_t2).cos() * (var_t2).cos())))) / (assign10060_e9508 * assign10060_e9508)), (((var_q_dn4 * assign10060_e9508) - (var_q * (var_t2_dn4 / ((var_t2).cos() * (var_t2).cos())))) / (assign10060_e9508 * assign10060_e9508)), (((var_q_dn5 * assign10060_e9508) - (var_q * (var_t2_dn5 / ((var_t2).cos() * (var_t2).cos())))) / (assign10060_e9508 * assign10060_e9508)), (((var_q_dn6 * assign10060_e9508) - (var_q * (var_t2_dn6 / ((var_t2).cos() * (var_t2).cos())))) / (assign10060_e9508 * assign10060_e9508)), (((var_q_dn7 * assign10060_e9508) - (var_q * (var_t2_dn7 / ((var_t2).cos() * (var_t2).cos())))) / (assign10060_e9508 * assign10060_e9508)), (((var_q_dn8 * assign10060_e9508) - (var_q * (var_t2_dn8 / ((var_t2).cos() * (var_t2).cos())))) / (assign10060_e9508 * assign10060_e9508)),)
    } else {
        (var_qcoth, var_qcoth_dn3, var_qcoth_dn4, var_qcoth_dn5, var_qcoth_dn6, var_qcoth_dn7, var_qcoth_dn8,)
    }
};
        var_qcoth = assign10060_e9511;
        var_qcoth_dn3 = assign10060_e9511_d_n3;
        var_qcoth_dn4 = assign10060_e9511_d_n4;
        var_qcoth_dn5 = assign10060_e9511_d_n5;
        var_qcoth_dn6 = assign10060_e9511_d_n6;
        var_qcoth_dn7 = assign10060_e9511_d_n7;
        var_qcoth_dn8 = assign10060_e9511_d_n8;

        let (assign10070_e9516, assign10070_e9516_d_n3, assign10070_e9516_d_n4, assign10070_e9516_d_n5, assign10070_e9516_d_n6, assign10070_e9516_d_n7, assign10070_e9516_d_n8,) = {
    if (var_guard97 != 0.0) {
        let assign10070_e9514: f64 = (var_t2).sin();
        (assign10070_e9514, ((var_t2).cos() * var_t2_dn3), ((var_t2).cos() * var_t2_dn4), ((var_t2).cos() * var_t2_dn5), ((var_t2).cos() * var_t2_dn6), ((var_t2).cos() * var_t2_dn7), ((var_t2).cos() * var_t2_dn8),)
    } else {
        (var_t6, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8,)
    }
};
        var_t6 = assign10070_e9516;
        var_t6_dn3 = assign10070_e9516_d_n3;
        var_t6_dn4 = assign10070_e9516_d_n4;
        var_t6_dn5 = assign10070_e9516_d_n5;
        var_t6_dn6 = assign10070_e9516_d_n6;
        var_t6_dn7 = assign10070_e9516_d_n7;
        var_t6_dn8 = assign10070_e9516_d_n8;

        let (assign10080_e9523, assign10080_e9523_d_n3, assign10080_e9523_d_n4, assign10080_e9523_d_n5, assign10080_e9523_d_n6, assign10080_e9523_d_n7, assign10080_e9523_d_n8,) = {
    if (var_guard97 != 0.0) {
        let assign10080_e9519: f64 = (-var_t6);
        let assign10080_e9521: f64 = (assign10080_e9519 * var_t6);
        (assign10080_e9521, (((-var_t6_dn3) * var_t6) + (assign10080_e9519 * var_t6_dn3)), (((-var_t6_dn4) * var_t6) + (assign10080_e9519 * var_t6_dn4)), (((-var_t6_dn5) * var_t6) + (assign10080_e9519 * var_t6_dn5)), (((-var_t6_dn6) * var_t6) + (assign10080_e9519 * var_t6_dn6)), (((-var_t6_dn7) * var_t6) + (assign10080_e9519 * var_t6_dn7)), (((-var_t6_dn8) * var_t6) + (assign10080_e9519 * var_t6_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign10080_e9523;
        var_t1_dn3 = assign10080_e9523_d_n3;
        var_t1_dn4 = assign10080_e9523_d_n4;
        var_t1_dn5 = assign10080_e9523_d_n5;
        var_t1_dn6 = assign10080_e9523_d_n6;
        var_t1_dn7 = assign10080_e9523_d_n7;
        var_t1_dn8 = assign10080_e9523_d_n8;

        let (assign10090_e9529, assign10090_e9529_d_n3, assign10090_e9529_d_n4, assign10090_e9529_d_n5, assign10090_e9529_d_n6, assign10090_e9529_d_n7, assign10090_e9529_d_n8,) = {
    if (var_guard97 == 0.0) {
        let assign10090_e9527: f64 = (var_qsqrt).sqrt();
        (assign10090_e9527, (var_qsqrt_dn3 / (2.0 * assign10090_e9527)), (var_qsqrt_dn4 / (2.0 * assign10090_e9527)), (var_qsqrt_dn5 / (2.0 * assign10090_e9527)), (var_qsqrt_dn6 / (2.0 * assign10090_e9527)), (var_qsqrt_dn7 / (2.0 * assign10090_e9527)), (var_qsqrt_dn8 / (2.0 * assign10090_e9527)),)
    } else {
        (var_q, var_q_dn3, var_q_dn4, var_q_dn5, var_q_dn6, var_q_dn7, var_q_dn8,)
    }
};
        var_q = assign10090_e9529;
        var_q_dn3 = assign10090_e9529_d_n3;
        var_q_dn4 = assign10090_e9529_d_n4;
        var_q_dn5 = assign10090_e9529_d_n5;
        var_q_dn6 = assign10090_e9529_d_n6;
        var_q_dn7 = assign10090_e9529_d_n7;
        var_q_dn8 = assign10090_e9529_d_n8;

        let (assign10100_e9536, assign10100_e9536_d_n3, assign10100_e9536_d_n4, assign10100_e9536_d_n5, assign10100_e9536_d_n6, assign10100_e9536_d_n7, assign10100_e9536_d_n8,) = {
    if (var_guard97 == 0.0) {
        let assign10100_e9534: f64 = (0.5 * var_q);
        (assign10100_e9534, (0.5 * var_q_dn3), (0.5 * var_q_dn4), (0.5 * var_q_dn5), (0.5 * var_q_dn6), (0.5 * var_q_dn7), (0.5 * var_q_dn8),)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8,)
    }
};
        var_t2 = assign10100_e9536;
        var_t2_dn3 = assign10100_e9536_d_n3;
        var_t2_dn4 = assign10100_e9536_d_n4;
        var_t2_dn5 = assign10100_e9536_d_n5;
        var_t2_dn6 = assign10100_e9536_d_n6;
        var_t2_dn7 = assign10100_e9536_d_n7;
        var_t2_dn8 = assign10100_e9536_d_n8;

        let (assign10110_e9542, assign10110_e9542_d_n3, assign10110_e9542_d_n4, assign10110_e9542_d_n5, assign10110_e9542_d_n6, assign10110_e9542_d_n7, assign10110_e9542_d_n8,) = {
    if (var_guard97 == 0.0) {
        let assign10110_e9540: f64 = (var_t2).sinh();
        (assign10110_e9540, ((var_t2).cosh() * var_t2_dn3), ((var_t2).cosh() * var_t2_dn4), ((var_t2).cosh() * var_t2_dn5), ((var_t2).cosh() * var_t2_dn6), ((var_t2).cosh() * var_t2_dn7), ((var_t2).cosh() * var_t2_dn8),)
    } else {
        (var_t6, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8,)
    }
};
        var_t6 = assign10110_e9542;
        var_t6_dn3 = assign10110_e9542_d_n3;
        var_t6_dn4 = assign10110_e9542_d_n4;
        var_t6_dn5 = assign10110_e9542_d_n5;
        var_t6_dn6 = assign10110_e9542_d_n6;
        var_t6_dn7 = assign10110_e9542_d_n7;
        var_t6_dn8 = assign10110_e9542_d_n8;

        let (assign10120_e9549, assign10120_e9549_d_n3, assign10120_e9549_d_n4, assign10120_e9549_d_n5, assign10120_e9549_d_n6, assign10120_e9549_d_n7, assign10120_e9549_d_n8,) = {
    if (var_guard97 == 0.0) {
        let assign10120_e9547: f64 = (var_t6 * var_t6);
        (assign10120_e9547, ((var_t6_dn3 * var_t6) + (var_t6 * var_t6_dn3)), ((var_t6_dn4 * var_t6) + (var_t6 * var_t6_dn4)), ((var_t6_dn5 * var_t6) + (var_t6 * var_t6_dn5)), ((var_t6_dn6 * var_t6) + (var_t6 * var_t6_dn6)), ((var_t6_dn7 * var_t6) + (var_t6 * var_t6_dn7)), ((var_t6_dn8 * var_t6) + (var_t6 * var_t6_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign10120_e9549;
        var_t1_dn3 = assign10120_e9549_d_n3;
        var_t1_dn4 = assign10120_e9549_d_n4;
        var_t1_dn5 = assign10120_e9549_d_n5;
        var_t1_dn6 = assign10120_e9549_d_n6;
        var_t1_dn7 = assign10120_e9549_d_n7;
        var_t1_dn8 = assign10120_e9549_d_n8;

        let (assign10130_e9557, assign10130_e9557_d_n3, assign10130_e9557_d_n4, assign10130_e9557_d_n5, assign10130_e9557_d_n6, assign10130_e9557_d_n7, assign10130_e9557_d_n8,) = {
    if (var_guard97 == 0.0) {
        let assign10130_e9554: f64 = (var_t2).tanh();
        let assign10130_e9555: f64 = (var_q / assign10130_e9554);
        (assign10130_e9555, (((var_q_dn3 * assign10130_e9554) - (var_q * (var_t2_dn3 / ((var_t2).cosh() * (var_t2).cosh())))) / (assign10130_e9554 * assign10130_e9554)), (((var_q_dn4 * assign10130_e9554) - (var_q * (var_t2_dn4 / ((var_t2).cosh() * (var_t2).cosh())))) / (assign10130_e9554 * assign10130_e9554)), (((var_q_dn5 * assign10130_e9554) - (var_q * (var_t2_dn5 / ((var_t2).cosh() * (var_t2).cosh())))) / (assign10130_e9554 * assign10130_e9554)), (((var_q_dn6 * assign10130_e9554) - (var_q * (var_t2_dn6 / ((var_t2).cosh() * (var_t2).cosh())))) / (assign10130_e9554 * assign10130_e9554)), (((var_q_dn7 * assign10130_e9554) - (var_q * (var_t2_dn7 / ((var_t2).cosh() * (var_t2).cosh())))) / (assign10130_e9554 * assign10130_e9554)), (((var_q_dn8 * assign10130_e9554) - (var_q * (var_t2_dn8 / ((var_t2).cosh() * (var_t2).cosh())))) / (assign10130_e9554 * assign10130_e9554)),)
    } else {
        (var_qcoth, var_qcoth_dn3, var_qcoth_dn4, var_qcoth_dn5, var_qcoth_dn6, var_qcoth_dn7, var_qcoth_dn8,)
    }
};
        var_qcoth = assign10130_e9557;
        var_qcoth_dn3 = assign10130_e9557_d_n3;
        var_qcoth_dn4 = assign10130_e9557_d_n4;
        var_qcoth_dn5 = assign10130_e9557_d_n5;
        var_qcoth_dn6 = assign10130_e9557_d_n6;
        var_qcoth_dn7 = assign10130_e9557_d_n7;
        var_qcoth_dn8 = assign10130_e9557_d_n8;

        let assign10140_e9560: f64 = (var_k1 * var_q1);
        let assign10140_e9562: f64 = (assign10140_e9560 - var_qcoth);
        let assign10140_e9567: f64 = (var_t1 * var_t0);
        let assign10140_e9568: f64 = (var_qsqrt / assign10140_e9567);
        let assign10140_e9569: f64 = (1.0 - assign10140_e9568);
        let assign10140_e9570: f64 = (assign10140_e9562 / assign10140_e9569);
        var_qicored = assign10140_e9570;
        var_qicored_dn3 = (((((var_k1 * var_q1_dn3) - var_qcoth_dn3) * assign10140_e9569) - (assign10140_e9562 * (-(((var_qsqrt_dn3 * assign10140_e9567) - (var_qsqrt * ((var_t1_dn3 * var_t0) + (var_t1 * var_t0_dn3)))) / (assign10140_e9567 * assign10140_e9567))))) / (assign10140_e9569 * assign10140_e9569));
        var_qicored_dn4 = (((((var_k1 * var_q1_dn4) - var_qcoth_dn4) * assign10140_e9569) - (assign10140_e9562 * (-(((var_qsqrt_dn4 * assign10140_e9567) - (var_qsqrt * ((var_t1_dn4 * var_t0) + (var_t1 * var_t0_dn4)))) / (assign10140_e9567 * assign10140_e9567))))) / (assign10140_e9569 * assign10140_e9569));
        var_qicored_dn5 = (((((var_k1 * var_q1_dn5) - var_qcoth_dn5) * assign10140_e9569) - (assign10140_e9562 * (-(((var_qsqrt_dn5 * assign10140_e9567) - (var_qsqrt * ((var_t1_dn5 * var_t0) + (var_t1 * var_t0_dn5)))) / (assign10140_e9567 * assign10140_e9567))))) / (assign10140_e9569 * assign10140_e9569));
        var_qicored_dn6 = (((((var_k1 * var_q1_dn6) - var_qcoth_dn6) * assign10140_e9569) - (assign10140_e9562 * (-(((var_qsqrt_dn6 * assign10140_e9567) - (var_qsqrt * ((var_t1_dn6 * var_t0) + (var_t1 * var_t0_dn6)))) / (assign10140_e9567 * assign10140_e9567))))) / (assign10140_e9569 * assign10140_e9569));
        var_qicored_dn7 = (((((var_k1 * var_q1_dn7) - var_qcoth_dn7) * assign10140_e9569) - (assign10140_e9562 * (-(((var_qsqrt_dn7 * assign10140_e9567) - (var_qsqrt * ((var_t1_dn7 * var_t0) + (var_t1 * var_t0_dn7)))) / (assign10140_e9567 * assign10140_e9567))))) / (assign10140_e9569 * assign10140_e9569));
        var_qicored_dn8 = (((((var_k1 * var_q1_dn8) - var_qcoth_dn8) * assign10140_e9569) - (assign10140_e9562 * (-(((var_qsqrt_dn8 * assign10140_e9567) - (var_qsqrt * ((var_t1_dn8 * var_t0) + (var_t1 * var_t0_dn8)))) / (assign10140_e9567 * assign10140_e9567))))) / (assign10140_e9569 * assign10140_e9569));

        let assign10150_e9573: f64 = (var_q1 * var_cox1);
        let assign10150_e9575: f64 = (assign10150_e9573 * var_nvtm);
        var_qfrontd = assign10150_e9575;
        var_qfrontd_dn3 = (((var_q1_dn3 * var_cox1) * var_nvtm) + (assign10150_e9573 * var_nvtm_dn3));
        var_qfrontd_dn4 = (((var_q1_dn4 * var_cox1) * var_nvtm) + (assign10150_e9573 * var_nvtm_dn4));
        var_qfrontd_dn5 = (((var_q1_dn5 * var_cox1) * var_nvtm) + (assign10150_e9573 * var_nvtm_dn5));
        var_qfrontd_dn6 = (((var_q1_dn6 * var_cox1) * var_nvtm) + (assign10150_e9573 * var_nvtm_dn6));
        var_qfrontd_dn7 = (((var_q1_dn7 * var_cox1) * var_nvtm) + (assign10150_e9573 * var_nvtm_dn7));
        var_qfrontd_dn8 = (((var_q1_dn8 * var_cox1) * var_nvtm) + (assign10150_e9573 * var_nvtm_dn8));

        let assign10160_e9578: f64 = (var_qicored * var_csi);
        let assign10160_e9580: f64 = (assign10160_e9578 * var_nvtm);
        var_qtotd = assign10160_e9580;
        var_qtotd_dn3 = (((var_qicored_dn3 * var_csi) * var_nvtm) + (assign10160_e9578 * var_nvtm_dn3));
        var_qtotd_dn4 = (((var_qicored_dn4 * var_csi) * var_nvtm) + (assign10160_e9578 * var_nvtm_dn4));
        var_qtotd_dn5 = (((var_qicored_dn5 * var_csi) * var_nvtm) + (assign10160_e9578 * var_nvtm_dn5));
        var_qtotd_dn6 = (((var_qicored_dn6 * var_csi) * var_nvtm) + (assign10160_e9578 * var_nvtm_dn6));
        var_qtotd_dn7 = (((var_qicored_dn7 * var_csi) * var_nvtm) + (assign10160_e9578 * var_nvtm_dn7));
        var_qtotd_dn8 = (((var_qicored_dn8 * var_csi) * var_nvtm) + (assign10160_e9578 * var_nvtm_dn8));

        let assign10170_e9583: f64 = (var_qtotd - var_qfrontd);
        var_qbackd = assign10170_e9583;
        var_qbackd_dn3 = (var_qtotd_dn3 - var_qfrontd_dn3);
        var_qbackd_dn4 = (var_qtotd_dn4 - var_qfrontd_dn4);
        var_qbackd_dn5 = (var_qtotd_dn5 - var_qfrontd_dn5);
        var_qbackd_dn6 = (var_qtotd_dn6 - var_qfrontd_dn6);
        var_qbackd_dn7 = (var_qtotd_dn7 - var_qfrontd_dn7);
        var_qbackd_dn8 = (var_qtotd_dn8 - var_qfrontd_dn8);

        let assign10180_e9588: f64 = (var_cox2 * var_nvtm);
        let assign10180_e9589: f64 = (var_qbackd / assign10180_e9588);
        let assign10180_e9590: f64 = (var_xg2 - assign10180_e9589);
        var_phi2 = assign10180_e9590;
        var_phi2_dn3 = (var_xg2_dn3 - (((var_qbackd_dn3 * assign10180_e9588) - (var_qbackd * (var_cox2 * var_nvtm_dn3))) / (assign10180_e9588 * assign10180_e9588)));
        var_phi2_dn4 = (var_xg2_dn4 - (((var_qbackd_dn4 * assign10180_e9588) - (var_qbackd * (var_cox2 * var_nvtm_dn4))) / (assign10180_e9588 * assign10180_e9588)));
        var_phi2_dn5 = (var_xg2_dn5 - (((var_qbackd_dn5 * assign10180_e9588) - (var_qbackd * (var_cox2 * var_nvtm_dn5))) / (assign10180_e9588 * assign10180_e9588)));
        var_phi2_dn6 = (var_xg2_dn6 - (((var_qbackd_dn6 * assign10180_e9588) - (var_qbackd * (var_cox2 * var_nvtm_dn6))) / (assign10180_e9588 * assign10180_e9588)));
        var_phi2_dn7 = (var_xg2_dn7 - (((var_qbackd_dn7 * assign10180_e9588) - (var_qbackd * (var_cox2 * var_nvtm_dn7))) / (assign10180_e9588 * assign10180_e9588)));
        var_phi2_dn8 = (var_xg2_dn8 - (((var_qbackd_dn8 * assign10180_e9588) - (var_qbackd * (var_cox2 * var_nvtm_dn8))) / (assign10180_e9588 * assign10180_e9588)));

        let assign10190_e9593: f64 = (var_qtotd / var_cox1);
        var_qid = assign10190_e9593;
        var_qid_dn3 = (var_qtotd_dn3 / var_cox1);
        var_qid_dn4 = (var_qtotd_dn4 / var_cox1);
        var_qid_dn5 = (var_qtotd_dn5 / var_cox1);
        var_qid_dn6 = (var_qtotd_dn6 / var_cox1);
        var_qid_dn7 = (var_qtotd_dn7 / var_cox1);
        var_qid_dn8 = (var_qtotd_dn8 / var_cox1);

        let assign10200_e9597: f64 = (var_qis + var_qid);
        let assign10200_e9598: f64 = (0.5 * assign10200_e9597);
        var_qia = assign10200_e9598;
        var_qia_dn3 = (0.5 * (var_qis_dn3 + var_qid_dn3));
        var_qia_dn4 = (0.5 * (var_qis_dn4 + var_qid_dn4));
        var_qia_dn5 = (0.5 * (var_qis_dn5 + var_qid_dn5));
        var_qia_dn6 = (0.5 * (var_qis_dn6 + var_qid_dn6));
        var_qia_dn7 = (0.5 * (var_qis_dn7 + var_qid_dn7));
        var_qia_dn8 = (0.5 * (var_qis_dn8 + var_qid_dn8));

        let assign10210_e9601: f64 = (var_qis - var_qid);
        var_dqi = assign10210_e9601;
        var_dqi_dn3 = (var_qis_dn3 - var_qid_dn3);
        var_dqi_dn4 = (var_qis_dn4 - var_qid_dn4);
        var_dqi_dn5 = (var_qis_dn5 - var_qid_dn5);
        var_dqi_dn6 = (var_qis_dn6 - var_qid_dn6);
        var_dqi_dn7 = (var_qis_dn7 - var_qid_dn7);
        var_dqi_dn8 = (var_qis_dn8 - var_qid_dn8);

        let assign10220_e9604: f64 = (1.60219e-19 * var_nbody_i);
        let assign10220_e9606: f64 = (assign10220_e9604 * p.p49);
        let assign10220_e9608: f64 = (assign10220_e9606 / var_cox1);
        var_qba = assign10220_e9608;

        let assign10230_e9611: f64 = (var_vdseff).powf(2.0);
        let assign10230_e9613: f64 = (assign10230_e9611 / 0.000625);
        var_t0 = assign10230_e9613;
        var_t0_dn3 = (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((var_vdseff).powf(2.0 - 1.0) * var_vdseff_dn3)) } } else { (assign10230_e9611 * (2.0 * (var_vdseff_dn3 / var_vdseff))) } / 0.000625);
        var_t0_dn4 = (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((var_vdseff).powf(2.0 - 1.0) * var_vdseff_dn4)) } } else { (assign10230_e9611 * (2.0 * (var_vdseff_dn4 / var_vdseff))) } / 0.000625);
        var_t0_dn5 = (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((var_vdseff).powf(2.0 - 1.0) * var_vdseff_dn5)) } } else { (assign10230_e9611 * (2.0 * (var_vdseff_dn5 / var_vdseff))) } / 0.000625);
        var_t0_dn6 = (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((var_vdseff).powf(2.0 - 1.0) * var_vdseff_dn6)) } } else { (assign10230_e9611 * (2.0 * (var_vdseff_dn6 / var_vdseff))) } / 0.000625);
        var_t0_dn7 = (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((var_vdseff).powf(2.0 - 1.0) * var_vdseff_dn7)) } } else { (assign10230_e9611 * (2.0 * (var_vdseff_dn7 / var_vdseff))) } / 0.000625);
        var_t0_dn8 = (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((var_vdseff).powf(2.0 - 1.0) * var_vdseff_dn8)) } } else { (assign10230_e9611 * (2.0 * (var_vdseff_dn8 / var_vdseff))) } / 0.000625);

        let assign10240_e9616: f64 = if p.p162 != 0.0 { 1.0 } else { 0.0 };
        var_guard98 = assign10240_e9616;

        let (assign10250_e9642, assign10250_e9642_d_n3, assign10250_e9642_d_n4, assign10250_e9642_d_n5, assign10250_e9642_d_n6, assign10250_e9642_d_n7, assign10250_e9642_d_n8,) = {
    if (var_guard98 != 0.0) {
        let assign10250_e9620: f64 = (var_qfronts + var_qfrontd);
        let assign10250_e9623: f64 = (2.0 * var_cox1);
        let assign10250_e9624: f64 = (assign10250_e9620 / assign10250_e9623);
        let assign10250_e9628: f64 = (-var_t0);
        let assign10250_e9629: f64 = { let limited_exp_arg = assign10250_e9628; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign10250_e9630: f64 = (1.0 - assign10250_e9629);
        let assign10250_e9631: f64 = (p.p162 * assign10250_e9630);
        let assign10250_e9633: f64 = (assign10250_e9631 * 0.5);
        let assign10250_e9636: f64 = (var_qfronts - var_qfrontd);
        let assign10250_e9637: f64 = (assign10250_e9633 * assign10250_e9636);
        let assign10250_e9639: f64 = (assign10250_e9637 / var_cox1);
        let assign10250_e9640: f64 = (assign10250_e9624 + assign10250_e9639);
        (assign10250_e9640, (((var_qfronts_dn3 + var_qfrontd_dn3) / assign10250_e9623) + (((((p.p162 * (-({ let limited_exp_arg = assign10250_e9628; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t0_dn3)))) * 0.5) * assign10250_e9636) + (assign10250_e9633 * (var_qfronts_dn3 - var_qfrontd_dn3))) / var_cox1)), (((var_qfronts_dn4 + var_qfrontd_dn4) / assign10250_e9623) + (((((p.p162 * (-({ let limited_exp_arg = assign10250_e9628; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t0_dn4)))) * 0.5) * assign10250_e9636) + (assign10250_e9633 * (var_qfronts_dn4 - var_qfrontd_dn4))) / var_cox1)), (((var_qfronts_dn5 + var_qfrontd_dn5) / assign10250_e9623) + (((((p.p162 * (-({ let limited_exp_arg = assign10250_e9628; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t0_dn5)))) * 0.5) * assign10250_e9636) + (assign10250_e9633 * (var_qfronts_dn5 - var_qfrontd_dn5))) / var_cox1)), (((var_qfronts_dn6 + var_qfrontd_dn6) / assign10250_e9623) + (((((p.p162 * (-({ let limited_exp_arg = assign10250_e9628; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t0_dn6)))) * 0.5) * assign10250_e9636) + (assign10250_e9633 * (var_qfronts_dn6 - var_qfrontd_dn6))) / var_cox1)), (((var_qfronts_dn7 + var_qfrontd_dn7) / assign10250_e9623) + (((((p.p162 * (-({ let limited_exp_arg = assign10250_e9628; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t0_dn7)))) * 0.5) * assign10250_e9636) + (assign10250_e9633 * (var_qfronts_dn7 - var_qfrontd_dn7))) / var_cox1)), (((var_qfronts_dn8 + var_qfrontd_dn8) / assign10250_e9623) + (((((p.p162 * (-({ let limited_exp_arg = assign10250_e9628; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t0_dn8)))) * 0.5) * assign10250_e9636) + (assign10250_e9633 * (var_qfronts_dn8 - var_qfrontd_dn8))) / var_cox1)),)
    } else {
        (var_qia2, var_qia2_dn3, var_qia2_dn4, var_qia2_dn5, var_qia2_dn6, var_qia2_dn7, var_qia2_dn8,)
    }
};
        var_qia2 = assign10250_e9642;
        var_qia2_dn3 = assign10250_e9642_d_n3;
        var_qia2_dn4 = assign10250_e9642_d_n4;
        var_qia2_dn5 = assign10250_e9642_d_n5;
        var_qia2_dn6 = assign10250_e9642_d_n6;
        var_qia2_dn7 = assign10250_e9642_d_n7;
        var_qia2_dn8 = assign10250_e9642_d_n8;

        let (assign10260_e9653, assign10260_e9653_d_n3, assign10260_e9653_d_n4, assign10260_e9653_d_n5, assign10260_e9653_d_n6, assign10260_e9653_d_n7, assign10260_e9653_d_n8,) = {
    if (var_guard98 == 0.0) {
        let assign10260_e9647: f64 = (var_qfronts + var_qfrontd);
        let assign10260_e9650: f64 = (2.0 * var_cox1);
        let assign10260_e9651: f64 = (assign10260_e9647 / assign10260_e9650);
        (assign10260_e9651, ((var_qfronts_dn3 + var_qfrontd_dn3) / assign10260_e9650), ((var_qfronts_dn4 + var_qfrontd_dn4) / assign10260_e9650), ((var_qfronts_dn5 + var_qfrontd_dn5) / assign10260_e9650), ((var_qfronts_dn6 + var_qfrontd_dn6) / assign10260_e9650), ((var_qfronts_dn7 + var_qfrontd_dn7) / assign10260_e9650), ((var_qfronts_dn8 + var_qfrontd_dn8) / assign10260_e9650),)
    } else {
        (var_qia2, var_qia2_dn3, var_qia2_dn4, var_qia2_dn5, var_qia2_dn6, var_qia2_dn7, var_qia2_dn8,)
    }
};
        var_qia2 = assign10260_e9653;
        var_qia2_dn3 = assign10260_e9653_d_n3;
        var_qia2_dn4 = assign10260_e9653_d_n4;
        var_qia2_dn5 = assign10260_e9653_d_n5;
        var_qia2_dn6 = assign10260_e9653_d_n6;
        var_qia2_dn7 = assign10260_e9653_d_n7;
        var_qia2_dn8 = assign10260_e9653_d_n8;

        let assign10270_e9656: f64 = if p.p189 != 0.0 { 1.0 } else { 0.0 };
        var_guard99 = assign10270_e9656;

        let (assign10280_e9682, assign10280_e9682_d_n3, assign10280_e9682_d_n4, assign10280_e9682_d_n5, assign10280_e9682_d_n6, assign10280_e9682_d_n7, assign10280_e9682_d_n8,) = {
    if (var_guard99 != 0.0) {
        let assign10280_e9660: f64 = (var_qbacks + var_qbackd);
        let assign10280_e9663: f64 = (2.0 * var_cox2);
        let assign10280_e9664: f64 = (assign10280_e9660 / assign10280_e9663);
        let assign10280_e9668: f64 = (-var_t0);
        let assign10280_e9669: f64 = { let limited_exp_arg = assign10280_e9668; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign10280_e9670: f64 = (1.0 - assign10280_e9669);
        let assign10280_e9671: f64 = (p.p189 * assign10280_e9670);
        let assign10280_e9673: f64 = (assign10280_e9671 * 0.5);
        let assign10280_e9676: f64 = (var_qbacks - var_qbackd);
        let assign10280_e9677: f64 = (assign10280_e9673 * assign10280_e9676);
        let assign10280_e9679: f64 = (assign10280_e9677 / var_cox2);
        let assign10280_e9680: f64 = (assign10280_e9664 + assign10280_e9679);
        (assign10280_e9680, (((var_qbacks_dn3 + var_qbackd_dn3) / assign10280_e9663) + (((((p.p189 * (-({ let limited_exp_arg = assign10280_e9668; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t0_dn3)))) * 0.5) * assign10280_e9676) + (assign10280_e9673 * (var_qbacks_dn3 - var_qbackd_dn3))) / var_cox2)), (((var_qbacks_dn4 + var_qbackd_dn4) / assign10280_e9663) + (((((p.p189 * (-({ let limited_exp_arg = assign10280_e9668; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t0_dn4)))) * 0.5) * assign10280_e9676) + (assign10280_e9673 * (var_qbacks_dn4 - var_qbackd_dn4))) / var_cox2)), (((var_qbacks_dn5 + var_qbackd_dn5) / assign10280_e9663) + (((((p.p189 * (-({ let limited_exp_arg = assign10280_e9668; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t0_dn5)))) * 0.5) * assign10280_e9676) + (assign10280_e9673 * (var_qbacks_dn5 - var_qbackd_dn5))) / var_cox2)), (((var_qbacks_dn6 + var_qbackd_dn6) / assign10280_e9663) + (((((p.p189 * (-({ let limited_exp_arg = assign10280_e9668; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t0_dn6)))) * 0.5) * assign10280_e9676) + (assign10280_e9673 * (var_qbacks_dn6 - var_qbackd_dn6))) / var_cox2)), (((var_qbacks_dn7 + var_qbackd_dn7) / assign10280_e9663) + (((((p.p189 * (-({ let limited_exp_arg = assign10280_e9668; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t0_dn7)))) * 0.5) * assign10280_e9676) + (assign10280_e9673 * (var_qbacks_dn7 - var_qbackd_dn7))) / var_cox2)), (((var_qbacks_dn8 + var_qbackd_dn8) / assign10280_e9663) + (((((p.p189 * (-({ let limited_exp_arg = assign10280_e9668; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t0_dn8)))) * 0.5) * assign10280_e9676) + (assign10280_e9673 * (var_qbacks_dn8 - var_qbackd_dn8))) / var_cox2)),)
    } else {
        (var_qib2, var_qib2_dn3, var_qib2_dn4, var_qib2_dn5, var_qib2_dn6, var_qib2_dn7, var_qib2_dn8,)
    }
};
        var_qib2 = assign10280_e9682;
        var_qib2_dn3 = assign10280_e9682_d_n3;
        var_qib2_dn4 = assign10280_e9682_d_n4;
        var_qib2_dn5 = assign10280_e9682_d_n5;
        var_qib2_dn6 = assign10280_e9682_d_n6;
        var_qib2_dn7 = assign10280_e9682_d_n7;
        var_qib2_dn8 = assign10280_e9682_d_n8;

        let (assign10290_e9693, assign10290_e9693_d_n3, assign10290_e9693_d_n4, assign10290_e9693_d_n5, assign10290_e9693_d_n6, assign10290_e9693_d_n7, assign10290_e9693_d_n8,) = {
    if (var_guard99 == 0.0) {
        let assign10290_e9687: f64 = (var_qbacks + var_qbackd);
        let assign10290_e9690: f64 = (2.0 * var_cox2);
        let assign10290_e9691: f64 = (assign10290_e9687 / assign10290_e9690);
        (assign10290_e9691, ((var_qbacks_dn3 + var_qbackd_dn3) / assign10290_e9690), ((var_qbacks_dn4 + var_qbackd_dn4) / assign10290_e9690), ((var_qbacks_dn5 + var_qbackd_dn5) / assign10290_e9690), ((var_qbacks_dn6 + var_qbackd_dn6) / assign10290_e9690), ((var_qbacks_dn7 + var_qbackd_dn7) / assign10290_e9690), ((var_qbacks_dn8 + var_qbackd_dn8) / assign10290_e9690),)
    } else {
        (var_qib2, var_qib2_dn3, var_qib2_dn4, var_qib2_dn5, var_qib2_dn6, var_qib2_dn7, var_qib2_dn8,)
    }
};
        var_qib2 = assign10290_e9693;
        var_qib2_dn3 = assign10290_e9693_d_n3;
        var_qib2_dn4 = assign10290_e9693_d_n4;
        var_qib2_dn5 = assign10290_e9693_d_n5;
        var_qib2_dn6 = assign10290_e9693_d_n6;
        var_qib2_dn7 = assign10290_e9693_d_n7;
        var_qib2_dn8 = assign10290_e9693_d_n8;

        let assign10300_e9696: f64 = (var_eta_mu * var_qia2);
        let assign10300_e9698: f64 = (assign10300_e9696 + var_qba);
        var_t2 = assign10300_e9698;
        var_t2_dn3 = (var_eta_mu * var_qia2_dn3);
        var_t2_dn4 = (var_eta_mu * var_qia2_dn4);
        var_t2_dn5 = (var_eta_mu * var_qia2_dn5);
        var_t2_dn6 = (var_eta_mu * var_qia2_dn6);
        var_t2_dn7 = (var_eta_mu * var_qia2_dn7);
        var_t2_dn8 = (var_eta_mu * var_qia2_dn8);

        let assign10310_e9703: f64 = (var_t2 * var_t2);
        let assign10310_e9705: f64 = (assign10310_e9703 + 0.001);
        let assign10310_e9706: f64 = (assign10310_e9705).sqrt();
        let assign10310_e9707: f64 = (var_t2 + assign10310_e9706);
        let assign10310_e9708: f64 = (0.5 * assign10310_e9707);
        var_t3 = assign10310_e9708;
        var_t3_dn3 = (0.5 * (var_t2_dn3 + (((var_t2_dn3 * var_t2) + (var_t2 * var_t2_dn3)) / (2.0 * assign10310_e9706))));
        var_t3_dn4 = (0.5 * (var_t2_dn4 + (((var_t2_dn4 * var_t2) + (var_t2 * var_t2_dn4)) / (2.0 * assign10310_e9706))));
        var_t3_dn5 = (0.5 * (var_t2_dn5 + (((var_t2_dn5 * var_t2) + (var_t2 * var_t2_dn5)) / (2.0 * assign10310_e9706))));
        var_t3_dn6 = (0.5 * (var_t2_dn6 + (((var_t2_dn6 * var_t2) + (var_t2 * var_t2_dn6)) / (2.0 * assign10310_e9706))));
        var_t3_dn7 = (0.5 * (var_t2_dn7 + (((var_t2_dn7 * var_t2) + (var_t2 * var_t2_dn7)) / (2.0 * assign10310_e9706))));
        var_t3_dn8 = (0.5 * (var_t2_dn8 + (((var_t2_dn8 * var_t2) + (var_t2 * var_t2_dn8)) / (2.0 * assign10310_e9706))));

        let assign10320_e9711: f64 = (var_eefffactor * var_t3);
        var_eeffm = assign10320_e9711;
        var_eeffm_dn3 = (var_eefffactor * var_t3_dn3);
        var_eeffm_dn4 = (var_eefffactor * var_t3_dn4);
        var_eeffm_dn5 = (var_eefffactor * var_t3_dn5);
        var_eeffm_dn6 = (var_eefffactor * var_t3_dn6);
        var_eeffm_dn7 = (var_eefffactor * var_t3_dn7);
        var_eeffm_dn8 = (var_eefffactor * var_t3_dn8);

        let assign10330_e9714: f64 = (var_eta_mu2 * var_qib2);
        let assign10330_e9716: f64 = (assign10330_e9714 + var_qba);
        var_t2 = assign10330_e9716;
        var_t2_dn3 = (var_eta_mu2 * var_qib2_dn3);
        var_t2_dn4 = (var_eta_mu2 * var_qib2_dn4);
        var_t2_dn5 = (var_eta_mu2 * var_qib2_dn5);
        var_t2_dn6 = (var_eta_mu2 * var_qib2_dn6);
        var_t2_dn7 = (var_eta_mu2 * var_qib2_dn7);
        var_t2_dn8 = (var_eta_mu2 * var_qib2_dn8);

        let assign10340_e9721: f64 = (var_t2 * var_t2);
        let assign10340_e9723: f64 = (assign10340_e9721 + 0.001);
        let assign10340_e9724: f64 = (assign10340_e9723).sqrt();
        let assign10340_e9725: f64 = (var_t2 + assign10340_e9724);
        let assign10340_e9726: f64 = (0.5 * assign10340_e9725);
        var_t3 = assign10340_e9726;
        var_t3_dn3 = (0.5 * (var_t2_dn3 + (((var_t2_dn3 * var_t2) + (var_t2 * var_t2_dn3)) / (2.0 * assign10340_e9724))));
        var_t3_dn4 = (0.5 * (var_t2_dn4 + (((var_t2_dn4 * var_t2) + (var_t2 * var_t2_dn4)) / (2.0 * assign10340_e9724))));
        var_t3_dn5 = (0.5 * (var_t2_dn5 + (((var_t2_dn5 * var_t2) + (var_t2 * var_t2_dn5)) / (2.0 * assign10340_e9724))));
        var_t3_dn6 = (0.5 * (var_t2_dn6 + (((var_t2_dn6 * var_t2) + (var_t2 * var_t2_dn6)) / (2.0 * assign10340_e9724))));
        var_t3_dn7 = (0.5 * (var_t2_dn7 + (((var_t2_dn7 * var_t2) + (var_t2 * var_t2_dn7)) / (2.0 * assign10340_e9724))));
        var_t3_dn8 = (0.5 * (var_t2_dn8 + (((var_t2_dn8 * var_t2) + (var_t2 * var_t2_dn8)) / (2.0 * assign10340_e9724))));

        let assign10350_e9729: f64 = (var_eefffactor2 * var_t3);
        var_eeffm2 = assign10350_e9729;
        var_eeffm2_dn3 = (var_eefffactor2 * var_t3_dn3);
        var_eeffm2_dn4 = (var_eefffactor2 * var_t3_dn4);
        var_eeffm2_dn5 = (var_eefffactor2 * var_t3_dn5);
        var_eeffm2_dn6 = (var_eefffactor2 * var_t3_dn6);
        var_eeffm2_dn7 = (var_eefffactor2 * var_t3_dn7);
        var_eeffm2_dn8 = (var_eefffactor2 * var_t3_dn8);

        let assign10360_e9734: f64 = (var_qia / var_qb0);
        let assign10360_e9735: f64 = (assign10360_e9734).abs();
        let assign10360_e9736: f64 = (1.0 + assign10360_e9735);
        let assign10360_e9737: f64 = (0.5 * assign10360_e9736);
        let assign10360_e9739: f64 = (assign10360_e9737).powf(var_ucs_t);
        var_t2__blk100 = assign10360_e9739;
        var_t2__blk100_dn3 = if 0.0 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign10360_e9737).powf(var_ucs_t - 1.0) * (0.5 * if assign10360_e9734 >= 0.0 { (var_qia_dn3 / var_qb0) } else { (-(var_qia_dn3 / var_qb0)) }))) } } else { (assign10360_e9739 * (var_ucs_t * ((0.5 * if assign10360_e9734 >= 0.0 { (var_qia_dn3 / var_qb0) } else { (-(var_qia_dn3 / var_qb0)) }) / assign10360_e9737))) };
        var_t2__blk100_dn4 = if var_ucs_t_dn4 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign10360_e9737).powf(var_ucs_t - 1.0) * (0.5 * if assign10360_e9734 >= 0.0 { (var_qia_dn4 / var_qb0) } else { (-(var_qia_dn4 / var_qb0)) }))) } } else { (assign10360_e9739 * ((var_ucs_t_dn4 * (assign10360_e9737).ln()) + (var_ucs_t * ((0.5 * if assign10360_e9734 >= 0.0 { (var_qia_dn4 / var_qb0) } else { (-(var_qia_dn4 / var_qb0)) }) / assign10360_e9737)))) };
        var_t2__blk100_dn5 = if 0.0 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign10360_e9737).powf(var_ucs_t - 1.0) * (0.5 * if assign10360_e9734 >= 0.0 { (var_qia_dn5 / var_qb0) } else { (-(var_qia_dn5 / var_qb0)) }))) } } else { (assign10360_e9739 * (var_ucs_t * ((0.5 * if assign10360_e9734 >= 0.0 { (var_qia_dn5 / var_qb0) } else { (-(var_qia_dn5 / var_qb0)) }) / assign10360_e9737))) };
        var_t2__blk100_dn6 = if 0.0 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign10360_e9737).powf(var_ucs_t - 1.0) * (0.5 * if assign10360_e9734 >= 0.0 { (var_qia_dn6 / var_qb0) } else { (-(var_qia_dn6 / var_qb0)) }))) } } else { (assign10360_e9739 * (var_ucs_t * ((0.5 * if assign10360_e9734 >= 0.0 { (var_qia_dn6 / var_qb0) } else { (-(var_qia_dn6 / var_qb0)) }) / assign10360_e9737))) };
        var_t2__blk100_dn7 = if 0.0 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign10360_e9737).powf(var_ucs_t - 1.0) * (0.5 * if assign10360_e9734 >= 0.0 { (var_qia_dn7 / var_qb0) } else { (-(var_qia_dn7 / var_qb0)) }))) } } else { (assign10360_e9739 * (var_ucs_t * ((0.5 * if assign10360_e9734 >= 0.0 { (var_qia_dn7 / var_qb0) } else { (-(var_qia_dn7 / var_qb0)) }) / assign10360_e9737))) };
        var_t2__blk100_dn8 = if 0.0 == 0.0 && ((var_ucs_t) as f64).is_finite() && ((var_ucs_t) as f64).fract() == 0.0 { if var_ucs_t == 0.0 { 0.0 } else { (var_ucs_t * ((assign10360_e9737).powf(var_ucs_t - 1.0) * (0.5 * if assign10360_e9734 >= 0.0 { (var_qia_dn8 / var_qb0) } else { (-(var_qia_dn8 / var_qb0)) }))) } } else { (assign10360_e9739 * (var_ucs_t * ((0.5 * if assign10360_e9734 >= 0.0 { (var_qia_dn8 / var_qb0) } else { (-(var_qia_dn8 / var_qb0)) }) / assign10360_e9737))) };

        let assign10370_e9743: f64 = (var_vbgx * var_uc_t);
        let assign10370_e9744: f64 = (var_ua_t + assign10370_e9743);
        let assign10370_e9746: f64 = (var_eeffm).abs();
        let assign10370_e9750: f64 = (var_eub_i * var_vbgx);
        let assign10370_e9751: f64 = (var_eu_i + assign10370_e9750);
        let assign10370_e9752: f64 = (assign10370_e9746).powf(assign10370_e9751);
        let assign10370_e9753: f64 = (assign10370_e9744 * assign10370_e9752);
        let assign10370_e9757: f64 = (var_vbgx * var_udb_i);
        let assign10370_e9758: f64 = (var_ud_t + assign10370_e9757);
        let assign10370_e9760: f64 = (assign10370_e9758 / var_t2__blk100);
        let assign10370_e9761: f64 = (assign10370_e9753 + assign10370_e9760);
        var_t3__blk101 = assign10370_e9761;
        var_t3__blk101_dn3 = ((((var_vbgx_dn3 * var_uc_t) * assign10370_e9752) + (assign10370_e9744 * if (var_eub_i * var_vbgx_dn3) == 0.0 && ((assign10370_e9751) as f64).is_finite() && ((assign10370_e9751) as f64).fract() == 0.0 { if assign10370_e9751 == 0.0 { 0.0 } else { (assign10370_e9751 * ((assign10370_e9746).powf(assign10370_e9751 - 1.0) * if var_eeffm >= 0.0 { var_eeffm_dn3 } else { (-var_eeffm_dn3) })) } } else { (assign10370_e9752 * (((var_eub_i * var_vbgx_dn3) * (assign10370_e9746).ln()) + (assign10370_e9751 * (if var_eeffm >= 0.0 { var_eeffm_dn3 } else { (-var_eeffm_dn3) } / assign10370_e9746)))) })) + ((((var_vbgx_dn3 * var_udb_i) * var_t2__blk100) - (assign10370_e9758 * var_t2__blk100_dn3)) / (var_t2__blk100 * var_t2__blk100)));
        var_t3__blk101_dn4 = ((((var_ua_t_dn4 + (var_vbgx * var_uc_t_dn4)) * assign10370_e9752) + (assign10370_e9744 * if 0.0 == 0.0 && ((assign10370_e9751) as f64).is_finite() && ((assign10370_e9751) as f64).fract() == 0.0 { if assign10370_e9751 == 0.0 { 0.0 } else { (assign10370_e9751 * ((assign10370_e9746).powf(assign10370_e9751 - 1.0) * if var_eeffm >= 0.0 { var_eeffm_dn4 } else { (-var_eeffm_dn4) })) } } else { (assign10370_e9752 * (assign10370_e9751 * (if var_eeffm >= 0.0 { var_eeffm_dn4 } else { (-var_eeffm_dn4) } / assign10370_e9746))) })) + (((var_ud_t_dn4 * var_t2__blk100) - (assign10370_e9758 * var_t2__blk100_dn4)) / (var_t2__blk100 * var_t2__blk100)));
        var_t3__blk101_dn5 = ((((var_vbgx_dn5 * var_uc_t) * assign10370_e9752) + (assign10370_e9744 * if (var_eub_i * var_vbgx_dn5) == 0.0 && ((assign10370_e9751) as f64).is_finite() && ((assign10370_e9751) as f64).fract() == 0.0 { if assign10370_e9751 == 0.0 { 0.0 } else { (assign10370_e9751 * ((assign10370_e9746).powf(assign10370_e9751 - 1.0) * if var_eeffm >= 0.0 { var_eeffm_dn5 } else { (-var_eeffm_dn5) })) } } else { (assign10370_e9752 * (((var_eub_i * var_vbgx_dn5) * (assign10370_e9746).ln()) + (assign10370_e9751 * (if var_eeffm >= 0.0 { var_eeffm_dn5 } else { (-var_eeffm_dn5) } / assign10370_e9746)))) })) + ((((var_vbgx_dn5 * var_udb_i) * var_t2__blk100) - (assign10370_e9758 * var_t2__blk100_dn5)) / (var_t2__blk100 * var_t2__blk100)));
        var_t3__blk101_dn6 = ((((var_vbgx_dn6 * var_uc_t) * assign10370_e9752) + (assign10370_e9744 * if (var_eub_i * var_vbgx_dn6) == 0.0 && ((assign10370_e9751) as f64).is_finite() && ((assign10370_e9751) as f64).fract() == 0.0 { if assign10370_e9751 == 0.0 { 0.0 } else { (assign10370_e9751 * ((assign10370_e9746).powf(assign10370_e9751 - 1.0) * if var_eeffm >= 0.0 { var_eeffm_dn6 } else { (-var_eeffm_dn6) })) } } else { (assign10370_e9752 * (((var_eub_i * var_vbgx_dn6) * (assign10370_e9746).ln()) + (assign10370_e9751 * (if var_eeffm >= 0.0 { var_eeffm_dn6 } else { (-var_eeffm_dn6) } / assign10370_e9746)))) })) + ((((var_vbgx_dn6 * var_udb_i) * var_t2__blk100) - (assign10370_e9758 * var_t2__blk100_dn6)) / (var_t2__blk100 * var_t2__blk100)));
        var_t3__blk101_dn7 = ((assign10370_e9744 * if 0.0 == 0.0 && ((assign10370_e9751) as f64).is_finite() && ((assign10370_e9751) as f64).fract() == 0.0 { if assign10370_e9751 == 0.0 { 0.0 } else { (assign10370_e9751 * ((assign10370_e9746).powf(assign10370_e9751 - 1.0) * if var_eeffm >= 0.0 { var_eeffm_dn7 } else { (-var_eeffm_dn7) })) } } else { (assign10370_e9752 * (assign10370_e9751 * (if var_eeffm >= 0.0 { var_eeffm_dn7 } else { (-var_eeffm_dn7) } / assign10370_e9746))) }) + (-((assign10370_e9758 * var_t2__blk100_dn7) / (var_t2__blk100 * var_t2__blk100))));
        var_t3__blk101_dn8 = ((assign10370_e9744 * if 0.0 == 0.0 && ((assign10370_e9751) as f64).is_finite() && ((assign10370_e9751) as f64).fract() == 0.0 { if assign10370_e9751 == 0.0 { 0.0 } else { (assign10370_e9751 * ((assign10370_e9746).powf(assign10370_e9751 - 1.0) * if var_eeffm >= 0.0 { var_eeffm_dn8 } else { (-var_eeffm_dn8) })) } } else { (assign10370_e9752 * (assign10370_e9751 * (if var_eeffm >= 0.0 { var_eeffm_dn8 } else { (-var_eeffm_dn8) } / assign10370_e9746))) }) + (-((assign10370_e9758 * var_t2__blk100_dn8) / (var_t2__blk100 * var_t2__blk100))));

        *var_delta_slot = var_delta;
        *var_delta_dn3_slot = var_delta_dn3;
        *var_delta_dn4_slot = var_delta_dn4;
        *var_delta_dn5_slot = var_delta_dn5;
        *var_delta_dn6_slot = var_delta_dn6;
        *var_delta_dn7_slot = var_delta_dn7;
        *var_delta_dn8_slot = var_delta_dn8;
        *var_dqi_slot = var_dqi;
        *var_dqi_dn3_slot = var_dqi_dn3;
        *var_dqi_dn4_slot = var_dqi_dn4;
        *var_dqi_dn5_slot = var_dqi_dn5;
        *var_dqi_dn6_slot = var_dqi_dn6;
        *var_dqi_dn7_slot = var_dqi_dn7;
        *var_dqi_dn8_slot = var_dqi_dn8;
        *var_eeffm_slot = var_eeffm;
        *var_eeffm2_slot = var_eeffm2;
        *var_eeffm2_dn3_slot = var_eeffm2_dn3;
        *var_eeffm2_dn4_slot = var_eeffm2_dn4;
        *var_eeffm2_dn5_slot = var_eeffm2_dn5;
        *var_eeffm2_dn6_slot = var_eeffm2_dn6;
        *var_eeffm2_dn7_slot = var_eeffm2_dn7;
        *var_eeffm2_dn8_slot = var_eeffm2_dn8;
        *var_eeffm_dn3_slot = var_eeffm_dn3;
        *var_eeffm_dn4_slot = var_eeffm_dn4;
        *var_eeffm_dn5_slot = var_eeffm_dn5;
        *var_eeffm_dn6_slot = var_eeffm_dn6;
        *var_eeffm_dn7_slot = var_eeffm_dn7;
        *var_eeffm_dn8_slot = var_eeffm_dn8;
        *var_guard97_slot = var_guard97;
        *var_guard98_slot = var_guard98;
        *var_guard99_slot = var_guard99;
        *var_phi1_slot = var_phi1;
        *var_phi1_dn3_slot = var_phi1_dn3;
        *var_phi1_dn4_slot = var_phi1_dn4;
        *var_phi1_dn5_slot = var_phi1_dn5;
        *var_phi1_dn6_slot = var_phi1_dn6;
        *var_phi1_dn7_slot = var_phi1_dn7;
        *var_phi1_dn8_slot = var_phi1_dn8;
        *var_phi2_slot = var_phi2;
        *var_phi2_dn3_slot = var_phi2_dn3;
        *var_phi2_dn4_slot = var_phi2_dn4;
        *var_phi2_dn5_slot = var_phi2_dn5;
        *var_phi2_dn6_slot = var_phi2_dn6;
        *var_phi2_dn7_slot = var_phi2_dn7;
        *var_phi2_dn8_slot = var_phi2_dn8;
        *var_q_slot = var_q;
        *var_q1_slot = var_q1;
        *var_q1_dn3_slot = var_q1_dn3;
        *var_q1_dn4_slot = var_q1_dn4;
        *var_q1_dn5_slot = var_q1_dn5;
        *var_q1_dn6_slot = var_q1_dn6;
        *var_q1_dn7_slot = var_q1_dn7;
        *var_q1_dn8_slot = var_q1_dn8;
        *var_q_dn3_slot = var_q_dn3;
        *var_q_dn4_slot = var_q_dn4;
        *var_q_dn5_slot = var_q_dn5;
        *var_q_dn6_slot = var_q_dn6;
        *var_q_dn7_slot = var_q_dn7;
        *var_q_dn8_slot = var_q_dn8;
        *var_qba_slot = var_qba;
        *var_qbackd_slot = var_qbackd;
        *var_qbackd_dn3_slot = var_qbackd_dn3;
        *var_qbackd_dn4_slot = var_qbackd_dn4;
        *var_qbackd_dn5_slot = var_qbackd_dn5;
        *var_qbackd_dn6_slot = var_qbackd_dn6;
        *var_qbackd_dn7_slot = var_qbackd_dn7;
        *var_qbackd_dn8_slot = var_qbackd_dn8;
        *var_qcoth_slot = var_qcoth;
        *var_qcoth_dn3_slot = var_qcoth_dn3;
        *var_qcoth_dn4_slot = var_qcoth_dn4;
        *var_qcoth_dn5_slot = var_qcoth_dn5;
        *var_qcoth_dn6_slot = var_qcoth_dn6;
        *var_qcoth_dn7_slot = var_qcoth_dn7;
        *var_qcoth_dn8_slot = var_qcoth_dn8;
        *var_qfrontd_slot = var_qfrontd;
        *var_qfrontd_dn3_slot = var_qfrontd_dn3;
        *var_qfrontd_dn4_slot = var_qfrontd_dn4;
        *var_qfrontd_dn5_slot = var_qfrontd_dn5;
        *var_qfrontd_dn6_slot = var_qfrontd_dn6;
        *var_qfrontd_dn7_slot = var_qfrontd_dn7;
        *var_qfrontd_dn8_slot = var_qfrontd_dn8;
        *var_qia_slot = var_qia;
        *var_qia2_slot = var_qia2;
        *var_qia2_dn3_slot = var_qia2_dn3;
        *var_qia2_dn4_slot = var_qia2_dn4;
        *var_qia2_dn5_slot = var_qia2_dn5;
        *var_qia2_dn6_slot = var_qia2_dn6;
        *var_qia2_dn7_slot = var_qia2_dn7;
        *var_qia2_dn8_slot = var_qia2_dn8;
        *var_qia_dn3_slot = var_qia_dn3;
        *var_qia_dn4_slot = var_qia_dn4;
        *var_qia_dn5_slot = var_qia_dn5;
        *var_qia_dn6_slot = var_qia_dn6;
        *var_qia_dn7_slot = var_qia_dn7;
        *var_qia_dn8_slot = var_qia_dn8;
        *var_qib2_slot = var_qib2;
        *var_qib2_dn3_slot = var_qib2_dn3;
        *var_qib2_dn4_slot = var_qib2_dn4;
        *var_qib2_dn5_slot = var_qib2_dn5;
        *var_qib2_dn6_slot = var_qib2_dn6;
        *var_qib2_dn7_slot = var_qib2_dn7;
        *var_qib2_dn8_slot = var_qib2_dn8;
        *var_qicored_slot = var_qicored;
        *var_qicored_dn3_slot = var_qicored_dn3;
        *var_qicored_dn4_slot = var_qicored_dn4;
        *var_qicored_dn5_slot = var_qicored_dn5;
        *var_qicored_dn6_slot = var_qicored_dn6;
        *var_qicored_dn7_slot = var_qicored_dn7;
        *var_qicored_dn8_slot = var_qicored_dn8;
        *var_qid_slot = var_qid;
        *var_qid_dn3_slot = var_qid_dn3;
        *var_qid_dn4_slot = var_qid_dn4;
        *var_qid_dn5_slot = var_qid_dn5;
        *var_qid_dn6_slot = var_qid_dn6;
        *var_qid_dn7_slot = var_qid_dn7;
        *var_qid_dn8_slot = var_qid_dn8;
        *var_qsqrt_slot = var_qsqrt;
        *var_qsqrt_dn3_slot = var_qsqrt_dn3;
        *var_qsqrt_dn4_slot = var_qsqrt_dn4;
        *var_qsqrt_dn5_slot = var_qsqrt_dn5;
        *var_qsqrt_dn6_slot = var_qsqrt_dn6;
        *var_qsqrt_dn7_slot = var_qsqrt_dn7;
        *var_qsqrt_dn8_slot = var_qsqrt_dn8;
        *var_qtotd_slot = var_qtotd;
        *var_qtotd_dn3_slot = var_qtotd_dn3;
        *var_qtotd_dn4_slot = var_qtotd_dn4;
        *var_qtotd_dn5_slot = var_qtotd_dn5;
        *var_qtotd_dn6_slot = var_qtotd_dn6;
        *var_qtotd_dn7_slot = var_qtotd_dn7;
        *var_qtotd_dn8_slot = var_qtotd_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2__blk100_slot = var_t2__blk100;
        *var_t2__blk100_dn3_slot = var_t2__blk100_dn3;
        *var_t2__blk100_dn4_slot = var_t2__blk100_dn4;
        *var_t2__blk100_dn5_slot = var_t2__blk100_dn5;
        *var_t2__blk100_dn6_slot = var_t2__blk100_dn6;
        *var_t2__blk100_dn7_slot = var_t2__blk100_dn7;
        *var_t2__blk100_dn8_slot = var_t2__blk100_dn8;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3__blk101_slot = var_t3__blk101;
        *var_t3__blk101_dn3_slot = var_t3__blk101_dn3;
        *var_t3__blk101_dn4_slot = var_t3__blk101_dn4;
        *var_t3__blk101_dn5_slot = var_t3__blk101_dn5;
        *var_t3__blk101_dn6_slot = var_t3__blk101_dn6;
        *var_t3__blk101_dn7_slot = var_t3__blk101_dn7;
        *var_t3__blk101_dn8_slot = var_t3__blk101_dn8;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t6_slot = var_t6;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
    }

    pub(super) fn stamp_transient_block_22(
        p: &Parameters,
        var_cox1: f64,
        var_cox2: f64,
        var_diblfactor: f64,
        var_diblfactor_dn3: f64,
        var_diblfactor_dn4: f64,
        var_diblfactor_dn5: f64,
        var_diblfactor_dn6: f64,
        var_diblfactor_dn7: f64,
        var_diblfactor_dn8: f64,
        var_dqi: f64,
        var_dqi_dn3: f64,
        var_dqi_dn4: f64,
        var_dqi_dn5: f64,
        var_dqi_dn6: f64,
        var_dqi_dn7: f64,
        var_dqi_dn8: f64,
        var_dvth_all: f64,
        var_dvth_all_dn3: f64,
        var_dvth_all_dn4: f64,
        var_dvth_all_dn5: f64,
        var_dvth_all_dn6: f64,
        var_dvth_all_dn7: f64,
        var_dvth_all_dn8: f64,
        var_eefffactor: f64,
        var_eeffm2: f64,
        var_eeffm2_dn3: f64,
        var_eeffm2_dn4: f64,
        var_eeffm2_dn5: f64,
        var_eeffm2_dn6: f64,
        var_eeffm2_dn7: f64,
        var_eeffm2_dn8: f64,
        var_esatl: f64,
        var_esatl_dn3: f64,
        var_esatl_dn4: f64,
        var_esatl_dn5: f64,
        var_esatl_dn6: f64,
        var_esatl_dn7: f64,
        var_esatl_dn8: f64,
        var_eta_mu_cv: f64,
        var_eu2_i: f64,
        var_eu_i: f64,
        var_eub2_i: f64,
        var_leff: f64,
        var_leffcv: f64,
        var_nvtm: f64,
        var_nvtm_dn3: f64,
        var_nvtm_dn4: f64,
        var_nvtm_dn5: f64,
        var_nvtm_dn6: f64,
        var_nvtm_dn7: f64,
        var_nvtm_dn8: f64,
        var_ptwg_t: f64,
        var_ptwg_t_dn4: f64,
        var_ptwgb2_i: f64,
        var_ptwgb_i: f64,
        var_pvag_i: f64,
        var_qb0: f64,
        var_qba: f64,
        var_qbackd: f64,
        var_qbackd_dn3: f64,
        var_qbackd_dn4: f64,
        var_qbackd_dn5: f64,
        var_qbackd_dn6: f64,
        var_qbackd_dn7: f64,
        var_qbackd_dn8: f64,
        var_qbacks: f64,
        var_qbacks_dn3: f64,
        var_qbacks_dn4: f64,
        var_qbacks_dn5: f64,
        var_qbacks_dn6: f64,
        var_qbacks_dn7: f64,
        var_qbacks_dn8: f64,
        var_qfrontd: f64,
        var_qfrontd_dn3: f64,
        var_qfrontd_dn4: f64,
        var_qfrontd_dn5: f64,
        var_qfrontd_dn6: f64,
        var_qfrontd_dn7: f64,
        var_qfrontd_dn8: f64,
        var_qfronts: f64,
        var_qfronts_dn3: f64,
        var_qfronts_dn4: f64,
        var_qfronts_dn5: f64,
        var_qfronts_dn6: f64,
        var_qfronts_dn7: f64,
        var_qfronts_dn8: f64,
        var_qia: f64,
        var_qia_dn3: f64,
        var_qia_dn4: f64,
        var_qia_dn5: f64,
        var_qia_dn6: f64,
        var_qia_dn7: f64,
        var_qia_dn8: f64,
        var_t3__blk101: f64,
        var_t3__blk101_dn3: f64,
        var_t3__blk101_dn4: f64,
        var_t3__blk101_dn5: f64,
        var_t3__blk101_dn6: f64,
        var_t3__blk101_dn7: f64,
        var_t3__blk101_dn8: f64,
        var_u02_i: f64,
        var_u0_t: f64,
        var_u0_t_dn4: f64,
        var_ua2_i: f64,
        var_ua_t: f64,
        var_ua_t_dn4: f64,
        var_uc2_i: f64,
        var_ucs2_i: f64,
        var_ud2_i: f64,
        var_udb2_i: f64,
        var_vbgx: f64,
        var_vbgx_dn3: f64,
        var_vbgx_dn5: f64,
        var_vbgx_dn6: f64,
        var_vbgxpos: f64,
        var_vbgxpos_dn3: f64,
        var_vbgxpos_dn5: f64,
        var_vbgxpos_dn6: f64,
        var_vds: f64,
        var_vds_dn5: f64,
        var_vds_dn6: f64,
        var_vdsat: f64,
        var_vdsat_dn3: f64,
        var_vdsat_dn4: f64,
        var_vdsat_dn5: f64,
        var_vdsat_dn6: f64,
        var_vdsat_dn7: f64,
        var_vdsat_dn8: f64,
        var_vdseff: f64,
        var_vdseff_dn3: f64,
        var_vdseff_dn4: f64,
        var_vdseff_dn5: f64,
        var_vdseff_dn6: f64,
        var_vdseff_dn7: f64,
        var_vdseff_dn8: f64,
        var_vgfb1eff: f64,
        var_vgfb1eff_dn3: f64,
        var_vgfb1eff_dn4: f64,
        var_vgfb1eff_dn5: f64,
        var_vgfb1eff_dn6: f64,
        var_vgfb1eff_dn7: f64,
        var_vgfb1eff_dn8: f64,
        var_vgfb2: f64,
        var_vgfb2_dn3: f64,
        var_vgfb2_dn4: f64,
        var_vgfb2_dn5: f64,
        var_vgfb2_dn6: f64,
        var_vgfb2_dn7: f64,
        var_vgfb2_dn8: f64,
        var_vsat1_t: f64,
        var_vsat1_t_dn4: f64,
        var_vsatb_t: f64,
        var_vsatb_t_dn4: f64,
        var_vsatcv_t: f64,
        var_vsatcv_t_dn4: f64,
        var_vtm: f64,
        var_vtm_dn4: f64,
        var_weff: f64,
        var_beta_slot: &mut f64,
        var_beta_dn3_slot: &mut f64,
        var_beta_dn4_slot: &mut f64,
        var_beta_dn5_slot: &mut f64,
        var_beta_dn6_slot: &mut f64,
        var_beta_dn7_slot: &mut f64,
        var_beta_dn8_slot: &mut f64,
        var_diffvds_slot: &mut f64,
        var_diffvds_dn3_slot: &mut f64,
        var_diffvds_dn4_slot: &mut f64,
        var_diffvds_dn5_slot: &mut f64,
        var_diffvds_dn6_slot: &mut f64,
        var_diffvds_dn7_slot: &mut f64,
        var_diffvds_dn8_slot: &mut f64,
        var_dmob_slot: &mut f64,
        var_dmob_cv_slot: &mut f64,
        var_dmob_cv_dn3_slot: &mut f64,
        var_dmob_cv_dn4_slot: &mut f64,
        var_dmob_cv_dn5_slot: &mut f64,
        var_dmob_cv_dn6_slot: &mut f64,
        var_dmob_cv_dn7_slot: &mut f64,
        var_dmob_cv_dn8_slot: &mut f64,
        var_dmob_dn3_slot: &mut f64,
        var_dmob_dn4_slot: &mut f64,
        var_dmob_dn5_slot: &mut f64,
        var_dmob_dn6_slot: &mut f64,
        var_dmob_dn7_slot: &mut f64,
        var_dmob_dn8_slot: &mut f64,
        var_dvsat_slot: &mut f64,
        var_dvsat_dn3_slot: &mut f64,
        var_dvsat_dn4_slot: &mut f64,
        var_dvsat_dn5_slot: &mut f64,
        var_dvsat_dn6_slot: &mut f64,
        var_dvsat_dn7_slot: &mut f64,
        var_dvsat_dn8_slot: &mut f64,
        var_eeffm_cv_slot: &mut f64,
        var_eeffm_cv_dn3_slot: &mut f64,
        var_eeffm_cv_dn4_slot: &mut f64,
        var_eeffm_cv_dn5_slot: &mut f64,
        var_eeffm_cv_dn6_slot: &mut f64,
        var_eeffm_cv_dn7_slot: &mut f64,
        var_eeffm_cv_dn8_slot: &mut f64,
        var_esat1_slot: &mut f64,
        var_esat1_dn3_slot: &mut f64,
        var_esat1_dn4_slot: &mut f64,
        var_esat1_dn5_slot: &mut f64,
        var_esat1_dn6_slot: &mut f64,
        var_esat1_dn7_slot: &mut f64,
        var_esat1_dn8_slot: &mut f64,
        var_esat1l_slot: &mut f64,
        var_esat1l_dn3_slot: &mut f64,
        var_esat1l_dn4_slot: &mut f64,
        var_esat1l_dn5_slot: &mut f64,
        var_esat1l_dn6_slot: &mut f64,
        var_esat1l_dn7_slot: &mut f64,
        var_esat1l_dn8_slot: &mut f64,
        var_esatcv_slot: &mut f64,
        var_esatcv_dn3_slot: &mut f64,
        var_esatcv_dn4_slot: &mut f64,
        var_esatcv_dn5_slot: &mut f64,
        var_esatcv_dn6_slot: &mut f64,
        var_esatcv_dn7_slot: &mut f64,
        var_esatcv_dn8_slot: &mut f64,
        var_esatcvl_slot: &mut f64,
        var_esatcvl_dn3_slot: &mut f64,
        var_esatcvl_dn4_slot: &mut f64,
        var_esatcvl_dn5_slot: &mut f64,
        var_esatcvl_dn6_slot: &mut f64,
        var_esatcvl_dn7_slot: &mut f64,
        var_esatcvl_dn8_slot: &mut f64,
        var_guard104_slot: &mut f64,
        var_guard105_slot: &mut f64,
        var_moc_slot: &mut f64,
        var_moc_dn3_slot: &mut f64,
        var_moc_dn4_slot: &mut f64,
        var_moc_dn5_slot: &mut f64,
        var_moc_dn6_slot: &mut f64,
        var_moc_dn7_slot: &mut f64,
        var_moc_dn8_slot: &mut f64,
        var_pvagfactor_slot: &mut f64,
        var_pvagfactor_dn3_slot: &mut f64,
        var_pvagfactor_dn4_slot: &mut f64,
        var_pvagfactor_dn5_slot: &mut f64,
        var_pvagfactor_dn6_slot: &mut f64,
        var_pvagfactor_dn7_slot: &mut f64,
        var_pvagfactor_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2__blk102_slot: &mut f64,
        var_t2__blk102_dn3_slot: &mut f64,
        var_t2__blk102_dn4_slot: &mut f64,
        var_t2__blk102_dn5_slot: &mut f64,
        var_t2__blk102_dn6_slot: &mut f64,
        var_t2__blk102_dn7_slot: &mut f64,
        var_t2__blk102_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3__blk103_slot: &mut f64,
        var_t3__blk103_dn3_slot: &mut f64,
        var_t3__blk103_dn4_slot: &mut f64,
        var_t3__blk103_dn5_slot: &mut f64,
        var_t3__blk103_dn6_slot: &mut f64,
        var_t3__blk103_dn7_slot: &mut f64,
        var_t3__blk103_dn8_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_ueff1_slot: &mut f64,
        var_ueff1_dn3_slot: &mut f64,
        var_ueff1_dn4_slot: &mut f64,
        var_ueff1_dn5_slot: &mut f64,
        var_ueff1_dn6_slot: &mut f64,
        var_ueff1_dn7_slot: &mut f64,
        var_ueff1_dn8_slot: &mut f64,
        var_ueff2_slot: &mut f64,
        var_ueff2_dn3_slot: &mut f64,
        var_ueff2_dn4_slot: &mut f64,
        var_ueff2_dn5_slot: &mut f64,
        var_ueff2_dn6_slot: &mut f64,
        var_ueff2_dn7_slot: &mut f64,
        var_ueff2_dn8_slot: &mut f64,
        var_utotal_slot: &mut f64,
        var_utotal_dn3_slot: &mut f64,
        var_utotal_dn4_slot: &mut f64,
        var_utotal_dn5_slot: &mut f64,
        var_utotal_dn6_slot: &mut f64,
        var_utotal_dn7_slot: &mut f64,
        var_utotal_dn8_slot: &mut f64,
        var_vadibl_slot: &mut f64,
        var_vadibl_dn3_slot: &mut f64,
        var_vadibl_dn4_slot: &mut f64,
        var_vadibl_dn5_slot: &mut f64,
        var_vadibl_dn6_slot: &mut f64,
        var_vadibl_dn7_slot: &mut f64,
        var_vadibl_dn8_slot: &mut f64,
        var_vgst2vtm_slot: &mut f64,
        var_vgst2vtm_dn3_slot: &mut f64,
        var_vgst2vtm_dn4_slot: &mut f64,
        var_vgst2vtm_dn5_slot: &mut f64,
        var_vgst2vtm_dn6_slot: &mut f64,
        var_vgst2vtm_dn7_slot: &mut f64,
        var_vgst2vtm_dn8_slot: &mut f64,
        var_w1_slot: &mut f64,
        var_w1_dn3_slot: &mut f64,
        var_w1_dn4_slot: &mut f64,
        var_w1_dn5_slot: &mut f64,
        var_w1_dn6_slot: &mut f64,
        var_w1_dn7_slot: &mut f64,
        var_w1_dn8_slot: &mut f64,
        var_w2_slot: &mut f64,
        var_w2_dn3_slot: &mut f64,
        var_w2_dn4_slot: &mut f64,
        var_w2_dn5_slot: &mut f64,
        var_w2_dn6_slot: &mut f64,
        var_w2_dn7_slot: &mut f64,
        var_w2_dn8_slot: &mut f64,
        var_xsat_slot: &mut f64,
        var_xsat_dn3_slot: &mut f64,
        var_xsat_dn4_slot: &mut f64,
        var_xsat_dn5_slot: &mut f64,
        var_xsat_dn6_slot: &mut f64,
        var_xsat_dn7_slot: &mut f64,
        var_xsat_dn8_slot: &mut f64,
    ) {
        let mut var_beta: f64 = *var_beta_slot;
        let mut var_beta_dn3: f64 = *var_beta_dn3_slot;
        let mut var_beta_dn4: f64 = *var_beta_dn4_slot;
        let mut var_beta_dn5: f64 = *var_beta_dn5_slot;
        let mut var_beta_dn6: f64 = *var_beta_dn6_slot;
        let mut var_beta_dn7: f64 = *var_beta_dn7_slot;
        let mut var_beta_dn8: f64 = *var_beta_dn8_slot;
        let mut var_diffvds: f64 = *var_diffvds_slot;
        let mut var_diffvds_dn3: f64 = *var_diffvds_dn3_slot;
        let mut var_diffvds_dn4: f64 = *var_diffvds_dn4_slot;
        let mut var_diffvds_dn5: f64 = *var_diffvds_dn5_slot;
        let mut var_diffvds_dn6: f64 = *var_diffvds_dn6_slot;
        let mut var_diffvds_dn7: f64 = *var_diffvds_dn7_slot;
        let mut var_diffvds_dn8: f64 = *var_diffvds_dn8_slot;
        let mut var_dmob: f64 = *var_dmob_slot;
        let mut var_dmob_cv: f64 = *var_dmob_cv_slot;
        let mut var_dmob_cv_dn3: f64 = *var_dmob_cv_dn3_slot;
        let mut var_dmob_cv_dn4: f64 = *var_dmob_cv_dn4_slot;
        let mut var_dmob_cv_dn5: f64 = *var_dmob_cv_dn5_slot;
        let mut var_dmob_cv_dn6: f64 = *var_dmob_cv_dn6_slot;
        let mut var_dmob_cv_dn7: f64 = *var_dmob_cv_dn7_slot;
        let mut var_dmob_cv_dn8: f64 = *var_dmob_cv_dn8_slot;
        let mut var_dmob_dn3: f64 = *var_dmob_dn3_slot;
        let mut var_dmob_dn4: f64 = *var_dmob_dn4_slot;
        let mut var_dmob_dn5: f64 = *var_dmob_dn5_slot;
        let mut var_dmob_dn6: f64 = *var_dmob_dn6_slot;
        let mut var_dmob_dn7: f64 = *var_dmob_dn7_slot;
        let mut var_dmob_dn8: f64 = *var_dmob_dn8_slot;
        let mut var_dvsat: f64 = *var_dvsat_slot;
        let mut var_dvsat_dn3: f64 = *var_dvsat_dn3_slot;
        let mut var_dvsat_dn4: f64 = *var_dvsat_dn4_slot;
        let mut var_dvsat_dn5: f64 = *var_dvsat_dn5_slot;
        let mut var_dvsat_dn6: f64 = *var_dvsat_dn6_slot;
        let mut var_dvsat_dn7: f64 = *var_dvsat_dn7_slot;
        let mut var_dvsat_dn8: f64 = *var_dvsat_dn8_slot;
        let mut var_eeffm_cv: f64 = *var_eeffm_cv_slot;
        let mut var_eeffm_cv_dn3: f64 = *var_eeffm_cv_dn3_slot;
        let mut var_eeffm_cv_dn4: f64 = *var_eeffm_cv_dn4_slot;
        let mut var_eeffm_cv_dn5: f64 = *var_eeffm_cv_dn5_slot;
        let mut var_eeffm_cv_dn6: f64 = *var_eeffm_cv_dn6_slot;
        let mut var_eeffm_cv_dn7: f64 = *var_eeffm_cv_dn7_slot;
        let mut var_eeffm_cv_dn8: f64 = *var_eeffm_cv_dn8_slot;
        let mut var_esat1: f64 = *var_esat1_slot;
        let mut var_esat1_dn3: f64 = *var_esat1_dn3_slot;
        let mut var_esat1_dn4: f64 = *var_esat1_dn4_slot;
        let mut var_esat1_dn5: f64 = *var_esat1_dn5_slot;
        let mut var_esat1_dn6: f64 = *var_esat1_dn6_slot;
        let mut var_esat1_dn7: f64 = *var_esat1_dn7_slot;
        let mut var_esat1_dn8: f64 = *var_esat1_dn8_slot;
        let mut var_esat1l: f64 = *var_esat1l_slot;
        let mut var_esat1l_dn3: f64 = *var_esat1l_dn3_slot;
        let mut var_esat1l_dn4: f64 = *var_esat1l_dn4_slot;
        let mut var_esat1l_dn5: f64 = *var_esat1l_dn5_slot;
        let mut var_esat1l_dn6: f64 = *var_esat1l_dn6_slot;
        let mut var_esat1l_dn7: f64 = *var_esat1l_dn7_slot;
        let mut var_esat1l_dn8: f64 = *var_esat1l_dn8_slot;
        let mut var_esatcv: f64 = *var_esatcv_slot;
        let mut var_esatcv_dn3: f64 = *var_esatcv_dn3_slot;
        let mut var_esatcv_dn4: f64 = *var_esatcv_dn4_slot;
        let mut var_esatcv_dn5: f64 = *var_esatcv_dn5_slot;
        let mut var_esatcv_dn6: f64 = *var_esatcv_dn6_slot;
        let mut var_esatcv_dn7: f64 = *var_esatcv_dn7_slot;
        let mut var_esatcv_dn8: f64 = *var_esatcv_dn8_slot;
        let mut var_esatcvl: f64 = *var_esatcvl_slot;
        let mut var_esatcvl_dn3: f64 = *var_esatcvl_dn3_slot;
        let mut var_esatcvl_dn4: f64 = *var_esatcvl_dn4_slot;
        let mut var_esatcvl_dn5: f64 = *var_esatcvl_dn5_slot;
        let mut var_esatcvl_dn6: f64 = *var_esatcvl_dn6_slot;
        let mut var_esatcvl_dn7: f64 = *var_esatcvl_dn7_slot;
        let mut var_esatcvl_dn8: f64 = *var_esatcvl_dn8_slot;
        let mut var_guard104: f64 = *var_guard104_slot;
        let mut var_guard105: f64 = *var_guard105_slot;
        let mut var_moc: f64 = *var_moc_slot;
        let mut var_moc_dn3: f64 = *var_moc_dn3_slot;
        let mut var_moc_dn4: f64 = *var_moc_dn4_slot;
        let mut var_moc_dn5: f64 = *var_moc_dn5_slot;
        let mut var_moc_dn6: f64 = *var_moc_dn6_slot;
        let mut var_moc_dn7: f64 = *var_moc_dn7_slot;
        let mut var_moc_dn8: f64 = *var_moc_dn8_slot;
        let mut var_pvagfactor: f64 = *var_pvagfactor_slot;
        let mut var_pvagfactor_dn3: f64 = *var_pvagfactor_dn3_slot;
        let mut var_pvagfactor_dn4: f64 = *var_pvagfactor_dn4_slot;
        let mut var_pvagfactor_dn5: f64 = *var_pvagfactor_dn5_slot;
        let mut var_pvagfactor_dn6: f64 = *var_pvagfactor_dn6_slot;
        let mut var_pvagfactor_dn7: f64 = *var_pvagfactor_dn7_slot;
        let mut var_pvagfactor_dn8: f64 = *var_pvagfactor_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2__blk102: f64 = *var_t2__blk102_slot;
        let mut var_t2__blk102_dn3: f64 = *var_t2__blk102_dn3_slot;
        let mut var_t2__blk102_dn4: f64 = *var_t2__blk102_dn4_slot;
        let mut var_t2__blk102_dn5: f64 = *var_t2__blk102_dn5_slot;
        let mut var_t2__blk102_dn6: f64 = *var_t2__blk102_dn6_slot;
        let mut var_t2__blk102_dn7: f64 = *var_t2__blk102_dn7_slot;
        let mut var_t2__blk102_dn8: f64 = *var_t2__blk102_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3__blk103: f64 = *var_t3__blk103_slot;
        let mut var_t3__blk103_dn3: f64 = *var_t3__blk103_dn3_slot;
        let mut var_t3__blk103_dn4: f64 = *var_t3__blk103_dn4_slot;
        let mut var_t3__blk103_dn5: f64 = *var_t3__blk103_dn5_slot;
        let mut var_t3__blk103_dn6: f64 = *var_t3__blk103_dn6_slot;
        let mut var_t3__blk103_dn7: f64 = *var_t3__blk103_dn7_slot;
        let mut var_t3__blk103_dn8: f64 = *var_t3__blk103_dn8_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_ueff1: f64 = *var_ueff1_slot;
        let mut var_ueff1_dn3: f64 = *var_ueff1_dn3_slot;
        let mut var_ueff1_dn4: f64 = *var_ueff1_dn4_slot;
        let mut var_ueff1_dn5: f64 = *var_ueff1_dn5_slot;
        let mut var_ueff1_dn6: f64 = *var_ueff1_dn6_slot;
        let mut var_ueff1_dn7: f64 = *var_ueff1_dn7_slot;
        let mut var_ueff1_dn8: f64 = *var_ueff1_dn8_slot;
        let mut var_ueff2: f64 = *var_ueff2_slot;
        let mut var_ueff2_dn3: f64 = *var_ueff2_dn3_slot;
        let mut var_ueff2_dn4: f64 = *var_ueff2_dn4_slot;
        let mut var_ueff2_dn5: f64 = *var_ueff2_dn5_slot;
        let mut var_ueff2_dn6: f64 = *var_ueff2_dn6_slot;
        let mut var_ueff2_dn7: f64 = *var_ueff2_dn7_slot;
        let mut var_ueff2_dn8: f64 = *var_ueff2_dn8_slot;
        let mut var_utotal: f64 = *var_utotal_slot;
        let mut var_utotal_dn3: f64 = *var_utotal_dn3_slot;
        let mut var_utotal_dn4: f64 = *var_utotal_dn4_slot;
        let mut var_utotal_dn5: f64 = *var_utotal_dn5_slot;
        let mut var_utotal_dn6: f64 = *var_utotal_dn6_slot;
        let mut var_utotal_dn7: f64 = *var_utotal_dn7_slot;
        let mut var_utotal_dn8: f64 = *var_utotal_dn8_slot;
        let mut var_vadibl: f64 = *var_vadibl_slot;
        let mut var_vadibl_dn3: f64 = *var_vadibl_dn3_slot;
        let mut var_vadibl_dn4: f64 = *var_vadibl_dn4_slot;
        let mut var_vadibl_dn5: f64 = *var_vadibl_dn5_slot;
        let mut var_vadibl_dn6: f64 = *var_vadibl_dn6_slot;
        let mut var_vadibl_dn7: f64 = *var_vadibl_dn7_slot;
        let mut var_vadibl_dn8: f64 = *var_vadibl_dn8_slot;
        let mut var_vgst2vtm: f64 = *var_vgst2vtm_slot;
        let mut var_vgst2vtm_dn3: f64 = *var_vgst2vtm_dn3_slot;
        let mut var_vgst2vtm_dn4: f64 = *var_vgst2vtm_dn4_slot;
        let mut var_vgst2vtm_dn5: f64 = *var_vgst2vtm_dn5_slot;
        let mut var_vgst2vtm_dn6: f64 = *var_vgst2vtm_dn6_slot;
        let mut var_vgst2vtm_dn7: f64 = *var_vgst2vtm_dn7_slot;
        let mut var_vgst2vtm_dn8: f64 = *var_vgst2vtm_dn8_slot;
        let mut var_w1: f64 = *var_w1_slot;
        let mut var_w1_dn3: f64 = *var_w1_dn3_slot;
        let mut var_w1_dn4: f64 = *var_w1_dn4_slot;
        let mut var_w1_dn5: f64 = *var_w1_dn5_slot;
        let mut var_w1_dn6: f64 = *var_w1_dn6_slot;
        let mut var_w1_dn7: f64 = *var_w1_dn7_slot;
        let mut var_w1_dn8: f64 = *var_w1_dn8_slot;
        let mut var_w2: f64 = *var_w2_slot;
        let mut var_w2_dn3: f64 = *var_w2_dn3_slot;
        let mut var_w2_dn4: f64 = *var_w2_dn4_slot;
        let mut var_w2_dn5: f64 = *var_w2_dn5_slot;
        let mut var_w2_dn6: f64 = *var_w2_dn6_slot;
        let mut var_w2_dn7: f64 = *var_w2_dn7_slot;
        let mut var_w2_dn8: f64 = *var_w2_dn8_slot;
        let mut var_xsat: f64 = *var_xsat_slot;
        let mut var_xsat_dn3: f64 = *var_xsat_dn3_slot;
        let mut var_xsat_dn4: f64 = *var_xsat_dn4_slot;
        let mut var_xsat_dn5: f64 = *var_xsat_dn5_slot;
        let mut var_xsat_dn6: f64 = *var_xsat_dn6_slot;
        let mut var_xsat_dn7: f64 = *var_xsat_dn7_slot;
        let mut var_xsat_dn8: f64 = *var_xsat_dn8_slot;

        let assign10380_e9764: f64 = (1.0 + var_t3__blk101);
        var_dmob = assign10380_e9764;
        var_dmob_dn3 = var_t3__blk101_dn3;
        var_dmob_dn4 = var_t3__blk101_dn4;
        var_dmob_dn5 = var_t3__blk101_dn5;
        var_dmob_dn6 = var_t3__blk101_dn6;
        var_dmob_dn7 = var_t3__blk101_dn7;
        var_dmob_dn8 = var_t3__blk101_dn8;

        let assign10390_e9768: f64 = (var_dmob + 1.0);
        let assign10390_e9771: f64 = (var_dmob - 1.0);
        let assign10390_e9774: f64 = (var_dmob - 1.0);
        let assign10390_e9775: f64 = (assign10390_e9771 * assign10390_e9774);
        let assign10390_e9778: f64 = (0.25 * p.p154);
        let assign10390_e9780: f64 = (assign10390_e9778 * p.p154);
        let assign10390_e9781: f64 = (assign10390_e9775 + assign10390_e9780);
        let assign10390_e9782: f64 = (assign10390_e9781).sqrt();
        let assign10390_e9783: f64 = (assign10390_e9768 + assign10390_e9782);
        let assign10390_e9784: f64 = (0.5 * assign10390_e9783);
        var_dmob = assign10390_e9784;
        var_dmob_dn3 = (0.5 * (var_dmob_dn3 + (((var_dmob_dn3 * assign10390_e9774) + (assign10390_e9771 * var_dmob_dn3)) / (2.0 * assign10390_e9782))));
        var_dmob_dn4 = (0.5 * (var_dmob_dn4 + (((var_dmob_dn4 * assign10390_e9774) + (assign10390_e9771 * var_dmob_dn4)) / (2.0 * assign10390_e9782))));
        var_dmob_dn5 = (0.5 * (var_dmob_dn5 + (((var_dmob_dn5 * assign10390_e9774) + (assign10390_e9771 * var_dmob_dn5)) / (2.0 * assign10390_e9782))));
        var_dmob_dn6 = (0.5 * (var_dmob_dn6 + (((var_dmob_dn6 * assign10390_e9774) + (assign10390_e9771 * var_dmob_dn6)) / (2.0 * assign10390_e9782))));
        var_dmob_dn7 = (0.5 * (var_dmob_dn7 + (((var_dmob_dn7 * assign10390_e9774) + (assign10390_e9771 * var_dmob_dn7)) / (2.0 * assign10390_e9782))));
        var_dmob_dn8 = (0.5 * (var_dmob_dn8 + (((var_dmob_dn8 * assign10390_e9774) + (assign10390_e9771 * var_dmob_dn8)) / (2.0 * assign10390_e9782))));

        let assign10400_e9787: f64 = (var_dmob / p.p11);
        var_dmob = assign10400_e9787;
        var_dmob_dn3 = (var_dmob_dn3 / p.p11);
        var_dmob_dn4 = (var_dmob_dn4 / p.p11);
        var_dmob_dn5 = (var_dmob_dn5 / p.p11);
        var_dmob_dn6 = (var_dmob_dn6 / p.p11);
        var_dmob_dn7 = (var_dmob_dn7 / p.p11);
        var_dmob_dn8 = (var_dmob_dn8 / p.p11);

        let assign10410_e9790: f64 = (var_u0_t / var_dmob);
        var_ueff1 = assign10410_e9790;
        var_ueff1_dn3 = (-((var_u0_t * var_dmob_dn3) / (var_dmob * var_dmob)));
        var_ueff1_dn4 = (((var_u0_t_dn4 * var_dmob) - (var_u0_t * var_dmob_dn4)) / (var_dmob * var_dmob));
        var_ueff1_dn5 = (-((var_u0_t * var_dmob_dn5) / (var_dmob * var_dmob)));
        var_ueff1_dn6 = (-((var_u0_t * var_dmob_dn6) / (var_dmob * var_dmob)));
        var_ueff1_dn7 = (-((var_u0_t * var_dmob_dn7) / (var_dmob * var_dmob)));
        var_ueff1_dn8 = (-((var_u0_t * var_dmob_dn8) / (var_dmob * var_dmob)));

        let assign10420_e9795: f64 = (var_qia / var_qb0);
        let assign10420_e9796: f64 = (assign10420_e9795).abs();
        let assign10420_e9797: f64 = (1.0 + assign10420_e9796);
        let assign10420_e9798: f64 = (0.5 * assign10420_e9797);
        let assign10420_e9800: f64 = (assign10420_e9798).powf(var_ucs2_i);
        var_t2__blk102 = assign10420_e9800;
        var_t2__blk102_dn3 = if 0.0 == 0.0 && ((var_ucs2_i) as f64).is_finite() && ((var_ucs2_i) as f64).fract() == 0.0 { if var_ucs2_i == 0.0 { 0.0 } else { (var_ucs2_i * ((assign10420_e9798).powf(var_ucs2_i - 1.0) * (0.5 * if assign10420_e9795 >= 0.0 { (var_qia_dn3 / var_qb0) } else { (-(var_qia_dn3 / var_qb0)) }))) } } else { (assign10420_e9800 * (var_ucs2_i * ((0.5 * if assign10420_e9795 >= 0.0 { (var_qia_dn3 / var_qb0) } else { (-(var_qia_dn3 / var_qb0)) }) / assign10420_e9798))) };
        var_t2__blk102_dn4 = if 0.0 == 0.0 && ((var_ucs2_i) as f64).is_finite() && ((var_ucs2_i) as f64).fract() == 0.0 { if var_ucs2_i == 0.0 { 0.0 } else { (var_ucs2_i * ((assign10420_e9798).powf(var_ucs2_i - 1.0) * (0.5 * if assign10420_e9795 >= 0.0 { (var_qia_dn4 / var_qb0) } else { (-(var_qia_dn4 / var_qb0)) }))) } } else { (assign10420_e9800 * (var_ucs2_i * ((0.5 * if assign10420_e9795 >= 0.0 { (var_qia_dn4 / var_qb0) } else { (-(var_qia_dn4 / var_qb0)) }) / assign10420_e9798))) };
        var_t2__blk102_dn5 = if 0.0 == 0.0 && ((var_ucs2_i) as f64).is_finite() && ((var_ucs2_i) as f64).fract() == 0.0 { if var_ucs2_i == 0.0 { 0.0 } else { (var_ucs2_i * ((assign10420_e9798).powf(var_ucs2_i - 1.0) * (0.5 * if assign10420_e9795 >= 0.0 { (var_qia_dn5 / var_qb0) } else { (-(var_qia_dn5 / var_qb0)) }))) } } else { (assign10420_e9800 * (var_ucs2_i * ((0.5 * if assign10420_e9795 >= 0.0 { (var_qia_dn5 / var_qb0) } else { (-(var_qia_dn5 / var_qb0)) }) / assign10420_e9798))) };
        var_t2__blk102_dn6 = if 0.0 == 0.0 && ((var_ucs2_i) as f64).is_finite() && ((var_ucs2_i) as f64).fract() == 0.0 { if var_ucs2_i == 0.0 { 0.0 } else { (var_ucs2_i * ((assign10420_e9798).powf(var_ucs2_i - 1.0) * (0.5 * if assign10420_e9795 >= 0.0 { (var_qia_dn6 / var_qb0) } else { (-(var_qia_dn6 / var_qb0)) }))) } } else { (assign10420_e9800 * (var_ucs2_i * ((0.5 * if assign10420_e9795 >= 0.0 { (var_qia_dn6 / var_qb0) } else { (-(var_qia_dn6 / var_qb0)) }) / assign10420_e9798))) };
        var_t2__blk102_dn7 = if 0.0 == 0.0 && ((var_ucs2_i) as f64).is_finite() && ((var_ucs2_i) as f64).fract() == 0.0 { if var_ucs2_i == 0.0 { 0.0 } else { (var_ucs2_i * ((assign10420_e9798).powf(var_ucs2_i - 1.0) * (0.5 * if assign10420_e9795 >= 0.0 { (var_qia_dn7 / var_qb0) } else { (-(var_qia_dn7 / var_qb0)) }))) } } else { (assign10420_e9800 * (var_ucs2_i * ((0.5 * if assign10420_e9795 >= 0.0 { (var_qia_dn7 / var_qb0) } else { (-(var_qia_dn7 / var_qb0)) }) / assign10420_e9798))) };
        var_t2__blk102_dn8 = if 0.0 == 0.0 && ((var_ucs2_i) as f64).is_finite() && ((var_ucs2_i) as f64).fract() == 0.0 { if var_ucs2_i == 0.0 { 0.0 } else { (var_ucs2_i * ((assign10420_e9798).powf(var_ucs2_i - 1.0) * (0.5 * if assign10420_e9795 >= 0.0 { (var_qia_dn8 / var_qb0) } else { (-(var_qia_dn8 / var_qb0)) }))) } } else { (assign10420_e9800 * (var_ucs2_i * ((0.5 * if assign10420_e9795 >= 0.0 { (var_qia_dn8 / var_qb0) } else { (-(var_qia_dn8 / var_qb0)) }) / assign10420_e9798))) };

        let assign10430_e9804: f64 = (var_vbgx * var_uc2_i);
        let assign10430_e9805: f64 = (var_ua2_i + assign10430_e9804);
        let assign10430_e9807: f64 = (var_eeffm2).abs();
        let assign10430_e9811: f64 = (var_eub2_i * var_vbgx);
        let assign10430_e9812: f64 = (var_eu2_i + assign10430_e9811);
        let assign10430_e9813: f64 = (assign10430_e9807).powf(assign10430_e9812);
        let assign10430_e9814: f64 = (assign10430_e9805 * assign10430_e9813);
        let assign10430_e9818: f64 = (var_vbgx * var_udb2_i);
        let assign10430_e9819: f64 = (var_ud2_i + assign10430_e9818);
        let assign10430_e9821: f64 = (assign10430_e9819 / var_t2__blk102);
        let assign10430_e9822: f64 = (assign10430_e9814 + assign10430_e9821);
        var_t3__blk103 = assign10430_e9822;
        var_t3__blk103_dn3 = ((((var_vbgx_dn3 * var_uc2_i) * assign10430_e9813) + (assign10430_e9805 * if (var_eub2_i * var_vbgx_dn3) == 0.0 && ((assign10430_e9812) as f64).is_finite() && ((assign10430_e9812) as f64).fract() == 0.0 { if assign10430_e9812 == 0.0 { 0.0 } else { (assign10430_e9812 * ((assign10430_e9807).powf(assign10430_e9812 - 1.0) * if var_eeffm2 >= 0.0 { var_eeffm2_dn3 } else { (-var_eeffm2_dn3) })) } } else { (assign10430_e9813 * (((var_eub2_i * var_vbgx_dn3) * (assign10430_e9807).ln()) + (assign10430_e9812 * (if var_eeffm2 >= 0.0 { var_eeffm2_dn3 } else { (-var_eeffm2_dn3) } / assign10430_e9807)))) })) + ((((var_vbgx_dn3 * var_udb2_i) * var_t2__blk102) - (assign10430_e9819 * var_t2__blk102_dn3)) / (var_t2__blk102 * var_t2__blk102)));
        var_t3__blk103_dn4 = ((assign10430_e9805 * if 0.0 == 0.0 && ((assign10430_e9812) as f64).is_finite() && ((assign10430_e9812) as f64).fract() == 0.0 { if assign10430_e9812 == 0.0 { 0.0 } else { (assign10430_e9812 * ((assign10430_e9807).powf(assign10430_e9812 - 1.0) * if var_eeffm2 >= 0.0 { var_eeffm2_dn4 } else { (-var_eeffm2_dn4) })) } } else { (assign10430_e9813 * (assign10430_e9812 * (if var_eeffm2 >= 0.0 { var_eeffm2_dn4 } else { (-var_eeffm2_dn4) } / assign10430_e9807))) }) + (-((assign10430_e9819 * var_t2__blk102_dn4) / (var_t2__blk102 * var_t2__blk102))));
        var_t3__blk103_dn5 = ((((var_vbgx_dn5 * var_uc2_i) * assign10430_e9813) + (assign10430_e9805 * if (var_eub2_i * var_vbgx_dn5) == 0.0 && ((assign10430_e9812) as f64).is_finite() && ((assign10430_e9812) as f64).fract() == 0.0 { if assign10430_e9812 == 0.0 { 0.0 } else { (assign10430_e9812 * ((assign10430_e9807).powf(assign10430_e9812 - 1.0) * if var_eeffm2 >= 0.0 { var_eeffm2_dn5 } else { (-var_eeffm2_dn5) })) } } else { (assign10430_e9813 * (((var_eub2_i * var_vbgx_dn5) * (assign10430_e9807).ln()) + (assign10430_e9812 * (if var_eeffm2 >= 0.0 { var_eeffm2_dn5 } else { (-var_eeffm2_dn5) } / assign10430_e9807)))) })) + ((((var_vbgx_dn5 * var_udb2_i) * var_t2__blk102) - (assign10430_e9819 * var_t2__blk102_dn5)) / (var_t2__blk102 * var_t2__blk102)));
        var_t3__blk103_dn6 = ((((var_vbgx_dn6 * var_uc2_i) * assign10430_e9813) + (assign10430_e9805 * if (var_eub2_i * var_vbgx_dn6) == 0.0 && ((assign10430_e9812) as f64).is_finite() && ((assign10430_e9812) as f64).fract() == 0.0 { if assign10430_e9812 == 0.0 { 0.0 } else { (assign10430_e9812 * ((assign10430_e9807).powf(assign10430_e9812 - 1.0) * if var_eeffm2 >= 0.0 { var_eeffm2_dn6 } else { (-var_eeffm2_dn6) })) } } else { (assign10430_e9813 * (((var_eub2_i * var_vbgx_dn6) * (assign10430_e9807).ln()) + (assign10430_e9812 * (if var_eeffm2 >= 0.0 { var_eeffm2_dn6 } else { (-var_eeffm2_dn6) } / assign10430_e9807)))) })) + ((((var_vbgx_dn6 * var_udb2_i) * var_t2__blk102) - (assign10430_e9819 * var_t2__blk102_dn6)) / (var_t2__blk102 * var_t2__blk102)));
        var_t3__blk103_dn7 = ((assign10430_e9805 * if 0.0 == 0.0 && ((assign10430_e9812) as f64).is_finite() && ((assign10430_e9812) as f64).fract() == 0.0 { if assign10430_e9812 == 0.0 { 0.0 } else { (assign10430_e9812 * ((assign10430_e9807).powf(assign10430_e9812 - 1.0) * if var_eeffm2 >= 0.0 { var_eeffm2_dn7 } else { (-var_eeffm2_dn7) })) } } else { (assign10430_e9813 * (assign10430_e9812 * (if var_eeffm2 >= 0.0 { var_eeffm2_dn7 } else { (-var_eeffm2_dn7) } / assign10430_e9807))) }) + (-((assign10430_e9819 * var_t2__blk102_dn7) / (var_t2__blk102 * var_t2__blk102))));
        var_t3__blk103_dn8 = ((assign10430_e9805 * if 0.0 == 0.0 && ((assign10430_e9812) as f64).is_finite() && ((assign10430_e9812) as f64).fract() == 0.0 { if assign10430_e9812 == 0.0 { 0.0 } else { (assign10430_e9812 * ((assign10430_e9807).powf(assign10430_e9812 - 1.0) * if var_eeffm2 >= 0.0 { var_eeffm2_dn8 } else { (-var_eeffm2_dn8) })) } } else { (assign10430_e9813 * (assign10430_e9812 * (if var_eeffm2 >= 0.0 { var_eeffm2_dn8 } else { (-var_eeffm2_dn8) } / assign10430_e9807))) }) + (-((assign10430_e9819 * var_t2__blk102_dn8) / (var_t2__blk102 * var_t2__blk102))));

        let assign10440_e9825: f64 = (1.0 + var_t3__blk103);
        var_dmob = assign10440_e9825;
        var_dmob_dn3 = var_t3__blk103_dn3;
        var_dmob_dn4 = var_t3__blk103_dn4;
        var_dmob_dn5 = var_t3__blk103_dn5;
        var_dmob_dn6 = var_t3__blk103_dn6;
        var_dmob_dn7 = var_t3__blk103_dn7;
        var_dmob_dn8 = var_t3__blk103_dn8;

        let assign10450_e9829: f64 = (var_dmob + 1.0);
        let assign10450_e9832: f64 = (var_dmob - 1.0);
        let assign10450_e9835: f64 = (var_dmob - 1.0);
        let assign10450_e9836: f64 = (assign10450_e9832 * assign10450_e9835);
        let assign10450_e9839: f64 = (0.25 * p.p154);
        let assign10450_e9841: f64 = (assign10450_e9839 * p.p154);
        let assign10450_e9842: f64 = (assign10450_e9836 + assign10450_e9841);
        let assign10450_e9843: f64 = (assign10450_e9842).sqrt();
        let assign10450_e9844: f64 = (assign10450_e9829 + assign10450_e9843);
        let assign10450_e9845: f64 = (0.5 * assign10450_e9844);
        var_dmob = assign10450_e9845;
        var_dmob_dn3 = (0.5 * (var_dmob_dn3 + (((var_dmob_dn3 * assign10450_e9835) + (assign10450_e9832 * var_dmob_dn3)) / (2.0 * assign10450_e9843))));
        var_dmob_dn4 = (0.5 * (var_dmob_dn4 + (((var_dmob_dn4 * assign10450_e9835) + (assign10450_e9832 * var_dmob_dn4)) / (2.0 * assign10450_e9843))));
        var_dmob_dn5 = (0.5 * (var_dmob_dn5 + (((var_dmob_dn5 * assign10450_e9835) + (assign10450_e9832 * var_dmob_dn5)) / (2.0 * assign10450_e9843))));
        var_dmob_dn6 = (0.5 * (var_dmob_dn6 + (((var_dmob_dn6 * assign10450_e9835) + (assign10450_e9832 * var_dmob_dn6)) / (2.0 * assign10450_e9843))));
        var_dmob_dn7 = (0.5 * (var_dmob_dn7 + (((var_dmob_dn7 * assign10450_e9835) + (assign10450_e9832 * var_dmob_dn7)) / (2.0 * assign10450_e9843))));
        var_dmob_dn8 = (0.5 * (var_dmob_dn8 + (((var_dmob_dn8 * assign10450_e9835) + (assign10450_e9832 * var_dmob_dn8)) / (2.0 * assign10450_e9843))));

        let assign10460_e9848: f64 = (var_dmob / p.p11);
        var_dmob = assign10460_e9848;
        var_dmob_dn3 = (var_dmob_dn3 / p.p11);
        var_dmob_dn4 = (var_dmob_dn4 / p.p11);
        var_dmob_dn5 = (var_dmob_dn5 / p.p11);
        var_dmob_dn6 = (var_dmob_dn6 / p.p11);
        var_dmob_dn7 = (var_dmob_dn7 / p.p11);
        var_dmob_dn8 = (var_dmob_dn8 / p.p11);

        let assign10470_e9851: f64 = (var_u02_i / var_dmob);
        var_ueff2 = assign10470_e9851;
        var_ueff2_dn3 = (-((var_u02_i * var_dmob_dn3) / (var_dmob * var_dmob)));
        var_ueff2_dn4 = (-((var_u02_i * var_dmob_dn4) / (var_dmob * var_dmob)));
        var_ueff2_dn5 = (-((var_u02_i * var_dmob_dn5) / (var_dmob * var_dmob)));
        var_ueff2_dn6 = (-((var_u02_i * var_dmob_dn6) / (var_dmob * var_dmob)));
        var_ueff2_dn7 = (-((var_u02_i * var_dmob_dn7) / (var_dmob * var_dmob)));
        var_ueff2_dn8 = (-((var_u02_i * var_dmob_dn8) / (var_dmob * var_dmob)));

        let assign10480_e9855: f64 = (var_qfronts + var_qfrontd);
        let assign10480_e9858: f64 = (2.0 * var_cox1);
        let assign10480_e9859: f64 = (assign10480_e9855 / assign10480_e9858);
        let assign10480_e9860: f64 = (var_vgfb1eff - assign10480_e9859);
        var_t0 = assign10480_e9860;
        var_t0_dn3 = (var_vgfb1eff_dn3 - ((var_qfronts_dn3 + var_qfrontd_dn3) / assign10480_e9858));
        var_t0_dn4 = (var_vgfb1eff_dn4 - ((var_qfronts_dn4 + var_qfrontd_dn4) / assign10480_e9858));
        var_t0_dn5 = (var_vgfb1eff_dn5 - ((var_qfronts_dn5 + var_qfrontd_dn5) / assign10480_e9858));
        var_t0_dn6 = (var_vgfb1eff_dn6 - ((var_qfronts_dn6 + var_qfrontd_dn6) / assign10480_e9858));
        var_t0_dn7 = (var_vgfb1eff_dn7 - ((var_qfronts_dn7 + var_qfrontd_dn7) / assign10480_e9858));
        var_t0_dn8 = (var_vgfb1eff_dn8 - ((var_qfronts_dn8 + var_qfrontd_dn8) / assign10480_e9858));

        let assign10490_e9863: f64 = (var_vgfb2 - var_dvth_all);
        let assign10490_e9866: f64 = (var_qbacks + var_qbackd);
        let assign10490_e9869: f64 = (2.0 * var_cox2);
        let assign10490_e9870: f64 = (assign10490_e9866 / assign10490_e9869);
        let assign10490_e9871: f64 = (assign10490_e9863 - assign10490_e9870);
        var_t1 = assign10490_e9871;
        var_t1_dn3 = ((var_vgfb2_dn3 - var_dvth_all_dn3) - ((var_qbacks_dn3 + var_qbackd_dn3) / assign10490_e9869));
        var_t1_dn4 = ((var_vgfb2_dn4 - var_dvth_all_dn4) - ((var_qbacks_dn4 + var_qbackd_dn4) / assign10490_e9869));
        var_t1_dn5 = ((var_vgfb2_dn5 - var_dvth_all_dn5) - ((var_qbacks_dn5 + var_qbackd_dn5) / assign10490_e9869));
        var_t1_dn6 = ((var_vgfb2_dn6 - var_dvth_all_dn6) - ((var_qbacks_dn6 + var_qbackd_dn6) / assign10490_e9869));
        var_t1_dn7 = ((var_vgfb2_dn7 - var_dvth_all_dn7) - ((var_qbacks_dn7 + var_qbackd_dn7) / assign10490_e9869));
        var_t1_dn8 = ((var_vgfb2_dn8 - var_dvth_all_dn8) - ((var_qbacks_dn8 + var_qbackd_dn8) / assign10490_e9869));

        let assign10500_e9874: f64 = (var_t0 / var_nvtm);
        let assign10500_e9875: f64 = (assign10500_e9874).exp();
        let assign10500_e9878: f64 = (var_t0 / var_nvtm);
        let assign10500_e9879: f64 = (assign10500_e9878).exp();
        let assign10500_e9882: f64 = (var_t1 / var_nvtm);
        let assign10500_e9883: f64 = (assign10500_e9882).exp();
        let assign10500_e9884: f64 = (assign10500_e9879 + assign10500_e9883);
        let assign10500_e9885: f64 = (assign10500_e9875 / assign10500_e9884);
        var_w1 = assign10500_e9885;
        var_w1_dn3 = ((((assign10500_e9875 * (((var_t0_dn3 * var_nvtm) - (var_t0 * var_nvtm_dn3)) / (var_nvtm * var_nvtm))) * assign10500_e9884) - (assign10500_e9875 * ((assign10500_e9879 * (((var_t0_dn3 * var_nvtm) - (var_t0 * var_nvtm_dn3)) / (var_nvtm * var_nvtm))) + (assign10500_e9883 * (((var_t1_dn3 * var_nvtm) - (var_t1 * var_nvtm_dn3)) / (var_nvtm * var_nvtm)))))) / (assign10500_e9884 * assign10500_e9884));
        var_w1_dn4 = ((((assign10500_e9875 * (((var_t0_dn4 * var_nvtm) - (var_t0 * var_nvtm_dn4)) / (var_nvtm * var_nvtm))) * assign10500_e9884) - (assign10500_e9875 * ((assign10500_e9879 * (((var_t0_dn4 * var_nvtm) - (var_t0 * var_nvtm_dn4)) / (var_nvtm * var_nvtm))) + (assign10500_e9883 * (((var_t1_dn4 * var_nvtm) - (var_t1 * var_nvtm_dn4)) / (var_nvtm * var_nvtm)))))) / (assign10500_e9884 * assign10500_e9884));
        var_w1_dn5 = ((((assign10500_e9875 * (((var_t0_dn5 * var_nvtm) - (var_t0 * var_nvtm_dn5)) / (var_nvtm * var_nvtm))) * assign10500_e9884) - (assign10500_e9875 * ((assign10500_e9879 * (((var_t0_dn5 * var_nvtm) - (var_t0 * var_nvtm_dn5)) / (var_nvtm * var_nvtm))) + (assign10500_e9883 * (((var_t1_dn5 * var_nvtm) - (var_t1 * var_nvtm_dn5)) / (var_nvtm * var_nvtm)))))) / (assign10500_e9884 * assign10500_e9884));
        var_w1_dn6 = ((((assign10500_e9875 * (((var_t0_dn6 * var_nvtm) - (var_t0 * var_nvtm_dn6)) / (var_nvtm * var_nvtm))) * assign10500_e9884) - (assign10500_e9875 * ((assign10500_e9879 * (((var_t0_dn6 * var_nvtm) - (var_t0 * var_nvtm_dn6)) / (var_nvtm * var_nvtm))) + (assign10500_e9883 * (((var_t1_dn6 * var_nvtm) - (var_t1 * var_nvtm_dn6)) / (var_nvtm * var_nvtm)))))) / (assign10500_e9884 * assign10500_e9884));
        var_w1_dn7 = ((((assign10500_e9875 * (((var_t0_dn7 * var_nvtm) - (var_t0 * var_nvtm_dn7)) / (var_nvtm * var_nvtm))) * assign10500_e9884) - (assign10500_e9875 * ((assign10500_e9879 * (((var_t0_dn7 * var_nvtm) - (var_t0 * var_nvtm_dn7)) / (var_nvtm * var_nvtm))) + (assign10500_e9883 * (((var_t1_dn7 * var_nvtm) - (var_t1 * var_nvtm_dn7)) / (var_nvtm * var_nvtm)))))) / (assign10500_e9884 * assign10500_e9884));
        var_w1_dn8 = ((((assign10500_e9875 * (((var_t0_dn8 * var_nvtm) - (var_t0 * var_nvtm_dn8)) / (var_nvtm * var_nvtm))) * assign10500_e9884) - (assign10500_e9875 * ((assign10500_e9879 * (((var_t0_dn8 * var_nvtm) - (var_t0 * var_nvtm_dn8)) / (var_nvtm * var_nvtm))) + (assign10500_e9883 * (((var_t1_dn8 * var_nvtm) - (var_t1 * var_nvtm_dn8)) / (var_nvtm * var_nvtm)))))) / (assign10500_e9884 * assign10500_e9884));

        let assign10510_e9888: f64 = (var_t1 / var_nvtm);
        let assign10510_e9889: f64 = (assign10510_e9888).exp();
        let assign10510_e9892: f64 = (var_t0 / var_nvtm);
        let assign10510_e9893: f64 = (assign10510_e9892).exp();
        let assign10510_e9896: f64 = (var_t1 / var_nvtm);
        let assign10510_e9897: f64 = (assign10510_e9896).exp();
        let assign10510_e9898: f64 = (assign10510_e9893 + assign10510_e9897);
        let assign10510_e9899: f64 = (assign10510_e9889 / assign10510_e9898);
        var_w2 = assign10510_e9899;
        var_w2_dn3 = ((((assign10510_e9889 * (((var_t1_dn3 * var_nvtm) - (var_t1 * var_nvtm_dn3)) / (var_nvtm * var_nvtm))) * assign10510_e9898) - (assign10510_e9889 * ((assign10510_e9893 * (((var_t0_dn3 * var_nvtm) - (var_t0 * var_nvtm_dn3)) / (var_nvtm * var_nvtm))) + (assign10510_e9897 * (((var_t1_dn3 * var_nvtm) - (var_t1 * var_nvtm_dn3)) / (var_nvtm * var_nvtm)))))) / (assign10510_e9898 * assign10510_e9898));
        var_w2_dn4 = ((((assign10510_e9889 * (((var_t1_dn4 * var_nvtm) - (var_t1 * var_nvtm_dn4)) / (var_nvtm * var_nvtm))) * assign10510_e9898) - (assign10510_e9889 * ((assign10510_e9893 * (((var_t0_dn4 * var_nvtm) - (var_t0 * var_nvtm_dn4)) / (var_nvtm * var_nvtm))) + (assign10510_e9897 * (((var_t1_dn4 * var_nvtm) - (var_t1 * var_nvtm_dn4)) / (var_nvtm * var_nvtm)))))) / (assign10510_e9898 * assign10510_e9898));
        var_w2_dn5 = ((((assign10510_e9889 * (((var_t1_dn5 * var_nvtm) - (var_t1 * var_nvtm_dn5)) / (var_nvtm * var_nvtm))) * assign10510_e9898) - (assign10510_e9889 * ((assign10510_e9893 * (((var_t0_dn5 * var_nvtm) - (var_t0 * var_nvtm_dn5)) / (var_nvtm * var_nvtm))) + (assign10510_e9897 * (((var_t1_dn5 * var_nvtm) - (var_t1 * var_nvtm_dn5)) / (var_nvtm * var_nvtm)))))) / (assign10510_e9898 * assign10510_e9898));
        var_w2_dn6 = ((((assign10510_e9889 * (((var_t1_dn6 * var_nvtm) - (var_t1 * var_nvtm_dn6)) / (var_nvtm * var_nvtm))) * assign10510_e9898) - (assign10510_e9889 * ((assign10510_e9893 * (((var_t0_dn6 * var_nvtm) - (var_t0 * var_nvtm_dn6)) / (var_nvtm * var_nvtm))) + (assign10510_e9897 * (((var_t1_dn6 * var_nvtm) - (var_t1 * var_nvtm_dn6)) / (var_nvtm * var_nvtm)))))) / (assign10510_e9898 * assign10510_e9898));
        var_w2_dn7 = ((((assign10510_e9889 * (((var_t1_dn7 * var_nvtm) - (var_t1 * var_nvtm_dn7)) / (var_nvtm * var_nvtm))) * assign10510_e9898) - (assign10510_e9889 * ((assign10510_e9893 * (((var_t0_dn7 * var_nvtm) - (var_t0 * var_nvtm_dn7)) / (var_nvtm * var_nvtm))) + (assign10510_e9897 * (((var_t1_dn7 * var_nvtm) - (var_t1 * var_nvtm_dn7)) / (var_nvtm * var_nvtm)))))) / (assign10510_e9898 * assign10510_e9898));
        var_w2_dn8 = ((((assign10510_e9889 * (((var_t1_dn8 * var_nvtm) - (var_t1 * var_nvtm_dn8)) / (var_nvtm * var_nvtm))) * assign10510_e9898) - (assign10510_e9889 * ((assign10510_e9893 * (((var_t0_dn8 * var_nvtm) - (var_t0 * var_nvtm_dn8)) / (var_nvtm * var_nvtm))) + (assign10510_e9897 * (((var_t1_dn8 * var_nvtm) - (var_t1 * var_nvtm_dn8)) / (var_nvtm * var_nvtm)))))) / (assign10510_e9898 * assign10510_e9898));

        let assign10520_e9902: f64 = (var_w1 * var_ueff1);
        let assign10520_e9905: f64 = (var_w2 * var_ueff2);
        let assign10520_e9906: f64 = (assign10520_e9902 + assign10520_e9905);
        var_utotal = assign10520_e9906;
        var_utotal_dn3 = (((var_w1_dn3 * var_ueff1) + (var_w1 * var_ueff1_dn3)) + ((var_w2_dn3 * var_ueff2) + (var_w2 * var_ueff2_dn3)));
        var_utotal_dn4 = (((var_w1_dn4 * var_ueff1) + (var_w1 * var_ueff1_dn4)) + ((var_w2_dn4 * var_ueff2) + (var_w2 * var_ueff2_dn4)));
        var_utotal_dn5 = (((var_w1_dn5 * var_ueff1) + (var_w1 * var_ueff1_dn5)) + ((var_w2_dn5 * var_ueff2) + (var_w2 * var_ueff2_dn5)));
        var_utotal_dn6 = (((var_w1_dn6 * var_ueff1) + (var_w1 * var_ueff1_dn6)) + ((var_w2_dn6 * var_ueff2) + (var_w2 * var_ueff2_dn6)));
        var_utotal_dn7 = (((var_w1_dn7 * var_ueff1) + (var_w1 * var_ueff1_dn7)) + ((var_w2_dn7 * var_ueff2) + (var_w2 * var_ueff2_dn7)));
        var_utotal_dn8 = (((var_w1_dn8 * var_ueff1) + (var_w1 * var_ueff1_dn8)) + ((var_w2_dn8 * var_ueff2) + (var_w2 * var_ueff2_dn8)));

        let assign10530_e9909: f64 = (var_utotal * var_cox1);
        let assign10530_e9911: f64 = (assign10530_e9909 * var_weff);
        let assign10530_e9913: f64 = (assign10530_e9911 / var_leff);
        var_beta = assign10530_e9913;
        var_beta_dn3 = (((var_utotal_dn3 * var_cox1) * var_weff) / var_leff);
        var_beta_dn4 = (((var_utotal_dn4 * var_cox1) * var_weff) / var_leff);
        var_beta_dn5 = (((var_utotal_dn5 * var_cox1) * var_weff) / var_leff);
        var_beta_dn6 = (((var_utotal_dn6 * var_cox1) * var_weff) / var_leff);
        var_beta_dn7 = (((var_utotal_dn7 * var_cox1) * var_weff) / var_leff);
        var_beta_dn8 = (((var_utotal_dn8 * var_cox1) * var_weff) / var_leff);

        let assign10540_e9918: f64 = (var_eta_mu_cv * var_qia);
        let assign10540_e9919: f64 = (var_qba + assign10540_e9918);
        let assign10540_e9920: f64 = (var_eefffactor * assign10540_e9919);
        var_eeffm_cv = assign10540_e9920;
        var_eeffm_cv_dn3 = (var_eefffactor * (var_eta_mu_cv * var_qia_dn3));
        var_eeffm_cv_dn4 = (var_eefffactor * (var_eta_mu_cv * var_qia_dn4));
        var_eeffm_cv_dn5 = (var_eefffactor * (var_eta_mu_cv * var_qia_dn5));
        var_eeffm_cv_dn6 = (var_eefffactor * (var_eta_mu_cv * var_qia_dn6));
        var_eeffm_cv_dn7 = (var_eefffactor * (var_eta_mu_cv * var_qia_dn7));
        var_eeffm_cv_dn8 = (var_eefffactor * (var_eta_mu_cv * var_qia_dn8));

        let assign10550_e9923: f64 = (var_eeffm_cv).abs();
        let assign10550_e9925: f64 = (assign10550_e9923).powf(var_eu_i);
        let assign10550_e9926: f64 = (var_ua_t * assign10550_e9925);
        var_t3 = assign10550_e9926;
        var_t3_dn3 = (var_ua_t * if 0.0 == 0.0 && ((var_eu_i) as f64).is_finite() && ((var_eu_i) as f64).fract() == 0.0 { if var_eu_i == 0.0 { 0.0 } else { (var_eu_i * ((assign10550_e9923).powf(var_eu_i - 1.0) * if var_eeffm_cv >= 0.0 { var_eeffm_cv_dn3 } else { (-var_eeffm_cv_dn3) })) } } else { (assign10550_e9925 * (var_eu_i * (if var_eeffm_cv >= 0.0 { var_eeffm_cv_dn3 } else { (-var_eeffm_cv_dn3) } / assign10550_e9923))) });
        var_t3_dn4 = ((var_ua_t_dn4 * assign10550_e9925) + (var_ua_t * if 0.0 == 0.0 && ((var_eu_i) as f64).is_finite() && ((var_eu_i) as f64).fract() == 0.0 { if var_eu_i == 0.0 { 0.0 } else { (var_eu_i * ((assign10550_e9923).powf(var_eu_i - 1.0) * if var_eeffm_cv >= 0.0 { var_eeffm_cv_dn4 } else { (-var_eeffm_cv_dn4) })) } } else { (assign10550_e9925 * (var_eu_i * (if var_eeffm_cv >= 0.0 { var_eeffm_cv_dn4 } else { (-var_eeffm_cv_dn4) } / assign10550_e9923))) }));
        var_t3_dn5 = (var_ua_t * if 0.0 == 0.0 && ((var_eu_i) as f64).is_finite() && ((var_eu_i) as f64).fract() == 0.0 { if var_eu_i == 0.0 { 0.0 } else { (var_eu_i * ((assign10550_e9923).powf(var_eu_i - 1.0) * if var_eeffm_cv >= 0.0 { var_eeffm_cv_dn5 } else { (-var_eeffm_cv_dn5) })) } } else { (assign10550_e9925 * (var_eu_i * (if var_eeffm_cv >= 0.0 { var_eeffm_cv_dn5 } else { (-var_eeffm_cv_dn5) } / assign10550_e9923))) });
        var_t3_dn6 = (var_ua_t * if 0.0 == 0.0 && ((var_eu_i) as f64).is_finite() && ((var_eu_i) as f64).fract() == 0.0 { if var_eu_i == 0.0 { 0.0 } else { (var_eu_i * ((assign10550_e9923).powf(var_eu_i - 1.0) * if var_eeffm_cv >= 0.0 { var_eeffm_cv_dn6 } else { (-var_eeffm_cv_dn6) })) } } else { (assign10550_e9925 * (var_eu_i * (if var_eeffm_cv >= 0.0 { var_eeffm_cv_dn6 } else { (-var_eeffm_cv_dn6) } / assign10550_e9923))) });
        var_t3_dn7 = (var_ua_t * if 0.0 == 0.0 && ((var_eu_i) as f64).is_finite() && ((var_eu_i) as f64).fract() == 0.0 { if var_eu_i == 0.0 { 0.0 } else { (var_eu_i * ((assign10550_e9923).powf(var_eu_i - 1.0) * if var_eeffm_cv >= 0.0 { var_eeffm_cv_dn7 } else { (-var_eeffm_cv_dn7) })) } } else { (assign10550_e9925 * (var_eu_i * (if var_eeffm_cv >= 0.0 { var_eeffm_cv_dn7 } else { (-var_eeffm_cv_dn7) } / assign10550_e9923))) });
        var_t3_dn8 = (var_ua_t * if 0.0 == 0.0 && ((var_eu_i) as f64).is_finite() && ((var_eu_i) as f64).fract() == 0.0 { if var_eu_i == 0.0 { 0.0 } else { (var_eu_i * ((assign10550_e9923).powf(var_eu_i - 1.0) * if var_eeffm_cv >= 0.0 { var_eeffm_cv_dn8 } else { (-var_eeffm_cv_dn8) })) } } else { (assign10550_e9925 * (var_eu_i * (if var_eeffm_cv >= 0.0 { var_eeffm_cv_dn8 } else { (-var_eeffm_cv_dn8) } / assign10550_e9923))) });

        let assign10560_e9929: f64 = (1.0 + var_t3);
        var_dmob_cv = assign10560_e9929;
        var_dmob_cv_dn3 = var_t3_dn3;
        var_dmob_cv_dn4 = var_t3_dn4;
        var_dmob_cv_dn5 = var_t3_dn5;
        var_dmob_cv_dn6 = var_t3_dn6;
        var_dmob_cv_dn7 = var_t3_dn7;
        var_dmob_cv_dn8 = var_t3_dn8;

        let assign10570_e9933: f64 = (var_dmob_cv + 1.0);
        let assign10570_e9936: f64 = (var_dmob_cv - 1.0);
        let assign10570_e9939: f64 = (var_dmob_cv - 1.0);
        let assign10570_e9940: f64 = (assign10570_e9936 * assign10570_e9939);
        let assign10570_e9943: f64 = (0.25 * p.p154);
        let assign10570_e9945: f64 = (assign10570_e9943 * p.p154);
        let assign10570_e9946: f64 = (assign10570_e9940 + assign10570_e9945);
        let assign10570_e9947: f64 = (assign10570_e9946).sqrt();
        let assign10570_e9948: f64 = (assign10570_e9933 + assign10570_e9947);
        let assign10570_e9949: f64 = (0.5 * assign10570_e9948);
        var_dmob_cv = assign10570_e9949;
        var_dmob_cv_dn3 = (0.5 * (var_dmob_cv_dn3 + (((var_dmob_cv_dn3 * assign10570_e9939) + (assign10570_e9936 * var_dmob_cv_dn3)) / (2.0 * assign10570_e9947))));
        var_dmob_cv_dn4 = (0.5 * (var_dmob_cv_dn4 + (((var_dmob_cv_dn4 * assign10570_e9939) + (assign10570_e9936 * var_dmob_cv_dn4)) / (2.0 * assign10570_e9947))));
        var_dmob_cv_dn5 = (0.5 * (var_dmob_cv_dn5 + (((var_dmob_cv_dn5 * assign10570_e9939) + (assign10570_e9936 * var_dmob_cv_dn5)) / (2.0 * assign10570_e9947))));
        var_dmob_cv_dn6 = (0.5 * (var_dmob_cv_dn6 + (((var_dmob_cv_dn6 * assign10570_e9939) + (assign10570_e9936 * var_dmob_cv_dn6)) / (2.0 * assign10570_e9947))));
        var_dmob_cv_dn7 = (0.5 * (var_dmob_cv_dn7 + (((var_dmob_cv_dn7 * assign10570_e9939) + (assign10570_e9936 * var_dmob_cv_dn7)) / (2.0 * assign10570_e9947))));
        var_dmob_cv_dn8 = (0.5 * (var_dmob_cv_dn8 + (((var_dmob_cv_dn8 * assign10570_e9939) + (assign10570_e9936 * var_dmob_cv_dn8)) / (2.0 * assign10570_e9947))));

        let assign10580_e9952: f64 = (var_dmob_cv / p.p11);
        var_dmob_cv = assign10580_e9952;
        var_dmob_cv_dn3 = (var_dmob_cv_dn3 / p.p11);
        var_dmob_cv_dn4 = (var_dmob_cv_dn4 / p.p11);
        var_dmob_cv_dn5 = (var_dmob_cv_dn5 / p.p11);
        var_dmob_cv_dn6 = (var_dmob_cv_dn6 / p.p11);
        var_dmob_cv_dn7 = (var_dmob_cv_dn7 / p.p11);
        var_dmob_cv_dn8 = (var_dmob_cv_dn8 / p.p11);

        let assign10590_e9955: f64 = (2.0 * var_vsat1_t);
        let assign10590_e9957: f64 = (assign10590_e9955 / var_utotal);
        var_esat1 = assign10590_e9957;
        var_esat1_dn3 = (-((assign10590_e9955 * var_utotal_dn3) / (var_utotal * var_utotal)));
        var_esat1_dn4 = ((((2.0 * var_vsat1_t_dn4) * var_utotal) - (assign10590_e9955 * var_utotal_dn4)) / (var_utotal * var_utotal));
        var_esat1_dn5 = (-((assign10590_e9955 * var_utotal_dn5) / (var_utotal * var_utotal)));
        var_esat1_dn6 = (-((assign10590_e9955 * var_utotal_dn6) / (var_utotal * var_utotal)));
        var_esat1_dn7 = (-((assign10590_e9955 * var_utotal_dn7) / (var_utotal * var_utotal)));
        var_esat1_dn8 = (-((assign10590_e9955 * var_utotal_dn8) / (var_utotal * var_utotal)));

        let assign10600_e9960: f64 = (var_esat1 * var_leff);
        var_esat1l = assign10600_e9960;
        var_esat1l_dn3 = (var_esat1_dn3 * var_leff);
        var_esat1l_dn4 = (var_esat1_dn4 * var_leff);
        var_esat1l_dn5 = (var_esat1_dn5 * var_leff);
        var_esat1l_dn6 = (var_esat1_dn6 * var_leff);
        var_esat1l_dn7 = (var_esat1_dn7 * var_leff);
        var_esat1l_dn8 = (var_esat1_dn8 * var_leff);

        let assign10610_e9964: f64 = (var_vsatb_t * var_vbgx);
        let assign10610_e9965: f64 = (0.8 + assign10610_e9964);
        var_t0 = assign10610_e9965;
        var_t0_dn3 = (var_vsatb_t * var_vbgx_dn3);
        var_t0_dn4 = (var_vsatb_t_dn4 * var_vbgx);
        var_t0_dn5 = (var_vsatb_t * var_vbgx_dn5);
        var_t0_dn6 = (var_vsatb_t * var_vbgx_dn6);
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;

        let assign10620_e9971: f64 = (var_t0 * var_t0);
        let assign10620_e9973: f64 = (assign10620_e9971 + 0.01);
        let assign10620_e9974: f64 = (assign10620_e9973).sqrt();
        let assign10620_e9975: f64 = (var_t0 + assign10620_e9974);
        let assign10620_e9976: f64 = (0.5 * assign10620_e9975);
        let assign10620_e9977: f64 = (0.2 + assign10620_e9976);
        var_xsat = assign10620_e9977;
        var_xsat_dn3 = (0.5 * (var_t0_dn3 + (((var_t0_dn3 * var_t0) + (var_t0 * var_t0_dn3)) / (2.0 * assign10620_e9974))));
        var_xsat_dn4 = (0.5 * (var_t0_dn4 + (((var_t0_dn4 * var_t0) + (var_t0 * var_t0_dn4)) / (2.0 * assign10620_e9974))));
        var_xsat_dn5 = (0.5 * (var_t0_dn5 + (((var_t0_dn5 * var_t0) + (var_t0 * var_t0_dn5)) / (2.0 * assign10620_e9974))));
        var_xsat_dn6 = (0.5 * (var_t0_dn6 + (((var_t0_dn6 * var_t0) + (var_t0 * var_t0_dn6)) / (2.0 * assign10620_e9974))));
        var_xsat_dn7 = (0.5 * (var_t0_dn7 + (((var_t0_dn7 * var_t0) + (var_t0 * var_t0_dn7)) / (2.0 * assign10620_e9974))));
        var_xsat_dn8 = (0.5 * (var_t0_dn8 + (((var_t0_dn8 * var_t0) + (var_t0 * var_t0_dn8)) / (2.0 * assign10620_e9974))));

        let assign10630_e9980: f64 = (var_dqi / var_esat1l);
        let assign10630_e9982: f64 = (assign10630_e9980 * var_xsat);
        var_t0 = assign10630_e9982;
        var_t0_dn3 = (((((var_dqi_dn3 * var_esat1l) - (var_dqi * var_esat1l_dn3)) / (var_esat1l * var_esat1l)) * var_xsat) + (assign10630_e9980 * var_xsat_dn3));
        var_t0_dn4 = (((((var_dqi_dn4 * var_esat1l) - (var_dqi * var_esat1l_dn4)) / (var_esat1l * var_esat1l)) * var_xsat) + (assign10630_e9980 * var_xsat_dn4));
        var_t0_dn5 = (((((var_dqi_dn5 * var_esat1l) - (var_dqi * var_esat1l_dn5)) / (var_esat1l * var_esat1l)) * var_xsat) + (assign10630_e9980 * var_xsat_dn5));
        var_t0_dn6 = (((((var_dqi_dn6 * var_esat1l) - (var_dqi * var_esat1l_dn6)) / (var_esat1l * var_esat1l)) * var_xsat) + (assign10630_e9980 * var_xsat_dn6));
        var_t0_dn7 = (((((var_dqi_dn7 * var_esat1l) - (var_dqi * var_esat1l_dn7)) / (var_esat1l * var_esat1l)) * var_xsat) + (assign10630_e9980 * var_xsat_dn7));
        var_t0_dn8 = (((((var_dqi_dn8 * var_esat1l) - (var_dqi * var_esat1l_dn8)) / (var_esat1l * var_esat1l)) * var_xsat) + (assign10630_e9980 * var_xsat_dn8));

        let assign10640_e9987: f64 = (var_t0 * var_t0);
        let assign10640_e9988: f64 = (p.p109 + assign10640_e9987);
        let assign10640_e9989: f64 = (assign10640_e9988).sqrt();
        let assign10640_e9990: f64 = (1.0 + assign10640_e9989);
        let assign10640_e9993: f64 = (p.p109).sqrt();
        let assign10640_e9994: f64 = (1.0 + assign10640_e9993);
        let assign10640_e9995: f64 = (assign10640_e9990 / assign10640_e9994);
        var_dvsat = assign10640_e9995;
        var_dvsat_dn3 = ((((var_t0_dn3 * var_t0) + (var_t0 * var_t0_dn3)) / (2.0 * assign10640_e9989)) / assign10640_e9994);
        var_dvsat_dn4 = ((((var_t0_dn4 * var_t0) + (var_t0 * var_t0_dn4)) / (2.0 * assign10640_e9989)) / assign10640_e9994);
        var_dvsat_dn5 = ((((var_t0_dn5 * var_t0) + (var_t0 * var_t0_dn5)) / (2.0 * assign10640_e9989)) / assign10640_e9994);
        var_dvsat_dn6 = ((((var_t0_dn6 * var_t0) + (var_t0 * var_t0_dn6)) / (2.0 * assign10640_e9989)) / assign10640_e9994);
        var_dvsat_dn7 = ((((var_t0_dn7 * var_t0) + (var_t0 * var_t0_dn7)) / (2.0 * assign10640_e9989)) / assign10640_e9994);
        var_dvsat_dn8 = ((((var_t0_dn8 * var_t0) + (var_t0 * var_t0_dn8)) / (2.0 * assign10640_e9989)) / assign10640_e9994);

        let assign10650_e10001: f64 = (var_ptwgb_i * var_vbgxpos);
        let assign10650_e10002: f64 = (var_ptwg_t - assign10650_e10001);
        let assign10650_e10005: f64 = (var_ptwgb2_i * var_vbgx);
        let assign10650_e10006: f64 = (assign10650_e10002 - assign10650_e10005);
        let assign10650_e10007: f64 = (0.5 * assign10650_e10006);
        let assign10650_e10009: f64 = (assign10650_e10007 * var_qia);
        let assign10650_e10011: f64 = (assign10650_e10009 * var_dqi);
        let assign10650_e10013: f64 = (assign10650_e10011 * var_dqi);
        let assign10650_e10014: f64 = (var_dvsat + assign10650_e10013);
        var_dvsat = assign10650_e10014;
        var_dvsat_dn3 = (var_dvsat_dn3 + (((((((0.5 * ((-(var_ptwgb_i * var_vbgxpos_dn3)) - (var_ptwgb2_i * var_vbgx_dn3))) * var_qia) + (assign10650_e10007 * var_qia_dn3)) * var_dqi) + (assign10650_e10009 * var_dqi_dn3)) * var_dqi) + (assign10650_e10011 * var_dqi_dn3)));
        var_dvsat_dn4 = (var_dvsat_dn4 + (((((((0.5 * var_ptwg_t_dn4) * var_qia) + (assign10650_e10007 * var_qia_dn4)) * var_dqi) + (assign10650_e10009 * var_dqi_dn4)) * var_dqi) + (assign10650_e10011 * var_dqi_dn4)));
        var_dvsat_dn5 = (var_dvsat_dn5 + (((((((0.5 * ((-(var_ptwgb_i * var_vbgxpos_dn5)) - (var_ptwgb2_i * var_vbgx_dn5))) * var_qia) + (assign10650_e10007 * var_qia_dn5)) * var_dqi) + (assign10650_e10009 * var_dqi_dn5)) * var_dqi) + (assign10650_e10011 * var_dqi_dn5)));
        var_dvsat_dn6 = (var_dvsat_dn6 + (((((((0.5 * ((-(var_ptwgb_i * var_vbgxpos_dn6)) - (var_ptwgb2_i * var_vbgx_dn6))) * var_qia) + (assign10650_e10007 * var_qia_dn6)) * var_dqi) + (assign10650_e10009 * var_dqi_dn6)) * var_dqi) + (assign10650_e10011 * var_dqi_dn6)));
        var_dvsat_dn7 = (var_dvsat_dn7 + (((((assign10650_e10007 * var_qia_dn7) * var_dqi) + (assign10650_e10009 * var_dqi_dn7)) * var_dqi) + (assign10650_e10011 * var_dqi_dn7)));
        var_dvsat_dn8 = (var_dvsat_dn8 + (((((assign10650_e10007 * var_qia_dn8) * var_dqi) + (assign10650_e10009 * var_dqi_dn8)) * var_dqi) + (assign10650_e10011 * var_dqi_dn8)));

        let assign10660_e10018: f64 = (var_dvsat + 1.0);
        let assign10660_e10021: f64 = (var_dvsat - 1.0);
        let assign10660_e10024: f64 = (var_dvsat - 1.0);
        let assign10660_e10025: f64 = (assign10660_e10021 * assign10660_e10024);
        let assign10660_e10028: f64 = (0.25 * p.p134);
        let assign10660_e10030: f64 = (assign10660_e10028 * p.p134);
        let assign10660_e10031: f64 = (assign10660_e10025 + assign10660_e10030);
        let assign10660_e10032: f64 = (assign10660_e10031).sqrt();
        let assign10660_e10033: f64 = (assign10660_e10018 + assign10660_e10032);
        let assign10660_e10034: f64 = (0.5 * assign10660_e10033);
        var_dvsat = assign10660_e10034;
        var_dvsat_dn3 = (0.5 * (var_dvsat_dn3 + (((var_dvsat_dn3 * assign10660_e10024) + (assign10660_e10021 * var_dvsat_dn3)) / (2.0 * assign10660_e10032))));
        var_dvsat_dn4 = (0.5 * (var_dvsat_dn4 + (((var_dvsat_dn4 * assign10660_e10024) + (assign10660_e10021 * var_dvsat_dn4)) / (2.0 * assign10660_e10032))));
        var_dvsat_dn5 = (0.5 * (var_dvsat_dn5 + (((var_dvsat_dn5 * assign10660_e10024) + (assign10660_e10021 * var_dvsat_dn5)) / (2.0 * assign10660_e10032))));
        var_dvsat_dn6 = (0.5 * (var_dvsat_dn6 + (((var_dvsat_dn6 * assign10660_e10024) + (assign10660_e10021 * var_dvsat_dn6)) / (2.0 * assign10660_e10032))));
        var_dvsat_dn7 = (0.5 * (var_dvsat_dn7 + (((var_dvsat_dn7 * assign10660_e10024) + (assign10660_e10021 * var_dvsat_dn7)) / (2.0 * assign10660_e10032))));
        var_dvsat_dn8 = (0.5 * (var_dvsat_dn8 + (((var_dvsat_dn8 * assign10660_e10024) + (assign10660_e10021 * var_dvsat_dn8)) / (2.0 * assign10660_e10032))));

        let assign10670_e10037: f64 = (2.0 * var_vsatcv_t);
        let assign10670_e10039: f64 = (assign10670_e10037 * var_dmob_cv);
        let assign10670_e10041: f64 = (assign10670_e10039 / var_u0_t);
        var_esatcv = assign10670_e10041;
        var_esatcv_dn3 = ((assign10670_e10037 * var_dmob_cv_dn3) / var_u0_t);
        var_esatcv_dn4 = ((((((2.0 * var_vsatcv_t_dn4) * var_dmob_cv) + (assign10670_e10037 * var_dmob_cv_dn4)) * var_u0_t) - (assign10670_e10039 * var_u0_t_dn4)) / (var_u0_t * var_u0_t));
        var_esatcv_dn5 = ((assign10670_e10037 * var_dmob_cv_dn5) / var_u0_t);
        var_esatcv_dn6 = ((assign10670_e10037 * var_dmob_cv_dn6) / var_u0_t);
        var_esatcv_dn7 = ((assign10670_e10037 * var_dmob_cv_dn7) / var_u0_t);
        var_esatcv_dn8 = ((assign10670_e10037 * var_dmob_cv_dn8) / var_u0_t);

        let assign10680_e10044: f64 = (var_esatcv * var_leffcv);
        var_esatcvl = assign10680_e10044;
        var_esatcvl_dn3 = (var_esatcv_dn3 * var_leffcv);
        var_esatcvl_dn4 = (var_esatcv_dn4 * var_leffcv);
        var_esatcvl_dn5 = (var_esatcv_dn5 * var_leffcv);
        var_esatcvl_dn6 = (var_esatcv_dn6 * var_leffcv);
        var_esatcvl_dn7 = (var_esatcv_dn7 * var_leffcv);
        var_esatcvl_dn8 = (var_esatcv_dn8 * var_leffcv);

        let assign10690_e10047: f64 = if var_pvag_i > 0.0 { 1.0 } else { 0.0 };
        var_guard104 = assign10690_e10047;

        let (assign10700_e10057, assign10700_e10057_d_n3, assign10700_e10057_d_n4, assign10700_e10057_d_n5, assign10700_e10057_d_n6, assign10700_e10057_d_n7, assign10700_e10057_d_n8,) = {
    if (var_guard104 != 0.0) {
        let assign10700_e10052: f64 = (var_pvag_i * var_qia);
        let assign10700_e10054: f64 = (assign10700_e10052 / var_esatl);
        let assign10700_e10055: f64 = (1.0 + assign10700_e10054);
        (assign10700_e10055, ((((var_pvag_i * var_qia_dn3) * var_esatl) - (assign10700_e10052 * var_esatl_dn3)) / (var_esatl * var_esatl)), ((((var_pvag_i * var_qia_dn4) * var_esatl) - (assign10700_e10052 * var_esatl_dn4)) / (var_esatl * var_esatl)), ((((var_pvag_i * var_qia_dn5) * var_esatl) - (assign10700_e10052 * var_esatl_dn5)) / (var_esatl * var_esatl)), ((((var_pvag_i * var_qia_dn6) * var_esatl) - (assign10700_e10052 * var_esatl_dn6)) / (var_esatl * var_esatl)), ((((var_pvag_i * var_qia_dn7) * var_esatl) - (assign10700_e10052 * var_esatl_dn7)) / (var_esatl * var_esatl)), ((((var_pvag_i * var_qia_dn8) * var_esatl) - (assign10700_e10052 * var_esatl_dn8)) / (var_esatl * var_esatl)),)
    } else {
        (var_pvagfactor, var_pvagfactor_dn3, var_pvagfactor_dn4, var_pvagfactor_dn5, var_pvagfactor_dn6, var_pvagfactor_dn7, var_pvagfactor_dn8,)
    }
};
        var_pvagfactor = assign10700_e10057;
        var_pvagfactor_dn3 = assign10700_e10057_d_n3;
        var_pvagfactor_dn4 = assign10700_e10057_d_n4;
        var_pvagfactor_dn5 = assign10700_e10057_d_n5;
        var_pvagfactor_dn6 = assign10700_e10057_d_n6;
        var_pvagfactor_dn7 = assign10700_e10057_d_n7;
        var_pvagfactor_dn8 = assign10700_e10057_d_n8;

        let (assign10710_e10070, assign10710_e10070_d_n3, assign10710_e10070_d_n4, assign10710_e10070_d_n5, assign10710_e10070_d_n6, assign10710_e10070_d_n7, assign10710_e10070_d_n8,) = {
    if (var_guard104 == 0.0) {
        let assign10710_e10064: f64 = (var_pvag_i * var_qia);
        let assign10710_e10066: f64 = (assign10710_e10064 / var_esatl);
        let assign10710_e10067: f64 = (1.0 - assign10710_e10066);
        let assign10710_e10068: f64 = (1.0 / assign10710_e10067);
        (assign10710_e10068, (-((-((((var_pvag_i * var_qia_dn3) * var_esatl) - (assign10710_e10064 * var_esatl_dn3)) / (var_esatl * var_esatl))) / (assign10710_e10067 * assign10710_e10067))), (-((-((((var_pvag_i * var_qia_dn4) * var_esatl) - (assign10710_e10064 * var_esatl_dn4)) / (var_esatl * var_esatl))) / (assign10710_e10067 * assign10710_e10067))), (-((-((((var_pvag_i * var_qia_dn5) * var_esatl) - (assign10710_e10064 * var_esatl_dn5)) / (var_esatl * var_esatl))) / (assign10710_e10067 * assign10710_e10067))), (-((-((((var_pvag_i * var_qia_dn6) * var_esatl) - (assign10710_e10064 * var_esatl_dn6)) / (var_esatl * var_esatl))) / (assign10710_e10067 * assign10710_e10067))), (-((-((((var_pvag_i * var_qia_dn7) * var_esatl) - (assign10710_e10064 * var_esatl_dn7)) / (var_esatl * var_esatl))) / (assign10710_e10067 * assign10710_e10067))), (-((-((((var_pvag_i * var_qia_dn8) * var_esatl) - (assign10710_e10064 * var_esatl_dn8)) / (var_esatl * var_esatl))) / (assign10710_e10067 * assign10710_e10067))),)
    } else {
        (var_pvagfactor, var_pvagfactor_dn3, var_pvagfactor_dn4, var_pvagfactor_dn5, var_pvagfactor_dn6, var_pvagfactor_dn7, var_pvagfactor_dn8,)
    }
};
        var_pvagfactor = assign10710_e10070;
        var_pvagfactor_dn3 = assign10710_e10070_d_n3;
        var_pvagfactor_dn4 = assign10710_e10070_d_n4;
        var_pvagfactor_dn5 = assign10710_e10070_d_n5;
        var_pvagfactor_dn6 = assign10710_e10070_d_n6;
        var_pvagfactor_dn7 = assign10710_e10070_d_n7;
        var_pvagfactor_dn8 = assign10710_e10070_d_n8;

        let assign10720_e10073: f64 = (var_vds - var_vdseff);
        var_diffvds = assign10720_e10073;
        var_diffvds_dn3 = (-var_vdseff_dn3);
        var_diffvds_dn4 = (-var_vdseff_dn4);
        var_diffvds_dn5 = (var_vds_dn5 - var_vdseff_dn5);
        var_diffvds_dn6 = (var_vds_dn6 - var_vdseff_dn6);
        var_diffvds_dn7 = (-var_vdseff_dn7);
        var_diffvds_dn8 = (-var_vdseff_dn8);

        let assign10730_e10077: f64 = (2.0 * var_vtm);
        let assign10730_e10078: f64 = (var_qia + assign10730_e10077);
        var_vgst2vtm = assign10730_e10078;
        var_vgst2vtm_dn3 = var_qia_dn3;
        var_vgst2vtm_dn4 = (var_qia_dn4 + (2.0 * var_vtm_dn4));
        var_vgst2vtm_dn5 = var_qia_dn5;
        var_vgst2vtm_dn6 = var_qia_dn6;
        var_vgst2vtm_dn7 = var_qia_dn7;
        var_vgst2vtm_dn8 = var_qia_dn8;

        let assign10740_e10081: f64 = if var_diblfactor > 0.0 { 1.0 } else { 0.0 };
        var_guard105 = assign10740_e10081;

        let (assign10750_e10085, assign10750_e10085_d_n3, assign10750_e10085_d_n4, assign10750_e10085_d_n5, assign10750_e10085_d_n6, assign10750_e10085_d_n7, assign10750_e10085_d_n8,) = {
    if (var_guard105 != 0.0) {
        (var_vgst2vtm, var_vgst2vtm_dn3, var_vgst2vtm_dn4, var_vgst2vtm_dn5, var_vgst2vtm_dn6, var_vgst2vtm_dn7, var_vgst2vtm_dn8,)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign10750_e10085;
        var_t1_dn3 = assign10750_e10085_d_n3;
        var_t1_dn4 = assign10750_e10085_d_n4;
        var_t1_dn5 = assign10750_e10085_d_n5;
        var_t1_dn6 = assign10750_e10085_d_n6;
        var_t1_dn7 = assign10750_e10085_d_n7;
        var_t1_dn8 = assign10750_e10085_d_n8;

        let (assign10760_e10093, assign10760_e10093_d_n3, assign10760_e10093_d_n4, assign10760_e10093_d_n5, assign10760_e10093_d_n6, assign10760_e10093_d_n7, assign10760_e10093_d_n8,) = {
    if (var_guard105 != 0.0) {
        let assign10760_e10090: f64 = (var_vdsat + var_t1);
        let assign10760_e10091: f64 = (var_t1 / assign10760_e10090);
        (assign10760_e10091, (((var_t1_dn3 * assign10760_e10090) - (var_t1 * (var_vdsat_dn3 + var_t1_dn3))) / (assign10760_e10090 * assign10760_e10090)), (((var_t1_dn4 * assign10760_e10090) - (var_t1 * (var_vdsat_dn4 + var_t1_dn4))) / (assign10760_e10090 * assign10760_e10090)), (((var_t1_dn5 * assign10760_e10090) - (var_t1 * (var_vdsat_dn5 + var_t1_dn5))) / (assign10760_e10090 * assign10760_e10090)), (((var_t1_dn6 * assign10760_e10090) - (var_t1 * (var_vdsat_dn6 + var_t1_dn6))) / (assign10760_e10090 * assign10760_e10090)), (((var_t1_dn7 * assign10760_e10090) - (var_t1 * (var_vdsat_dn7 + var_t1_dn7))) / (assign10760_e10090 * assign10760_e10090)), (((var_t1_dn8 * assign10760_e10090) - (var_t1 * (var_vdsat_dn8 + var_t1_dn8))) / (assign10760_e10090 * assign10760_e10090)),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8,)
    }
};
        var_t3 = assign10760_e10093;
        var_t3_dn3 = assign10760_e10093_d_n3;
        var_t3_dn4 = assign10760_e10093_d_n4;
        var_t3_dn5 = assign10760_e10093_d_n5;
        var_t3_dn6 = assign10760_e10093_d_n6;
        var_t3_dn7 = assign10760_e10093_d_n7;
        var_t3_dn8 = assign10760_e10093_d_n8;

        let (assign10770_e10103, assign10770_e10103_d_n3, assign10770_e10103_d_n4, assign10770_e10103_d_n5, assign10770_e10103_d_n6, assign10770_e10103_d_n7, assign10770_e10103_d_n8,) = {
    if (var_guard105 != 0.0) {
        let assign10770_e10097: f64 = (var_t1 / var_diblfactor);
        let assign10770_e10099: f64 = (assign10770_e10097 * var_t3);
        let assign10770_e10101: f64 = (assign10770_e10099 * var_pvagfactor);
        (assign10770_e10101, (((((((var_t1_dn3 * var_diblfactor) - (var_t1 * var_diblfactor_dn3)) / (var_diblfactor * var_diblfactor)) * var_t3) + (assign10770_e10097 * var_t3_dn3)) * var_pvagfactor) + (assign10770_e10099 * var_pvagfactor_dn3)), (((((((var_t1_dn4 * var_diblfactor) - (var_t1 * var_diblfactor_dn4)) / (var_diblfactor * var_diblfactor)) * var_t3) + (assign10770_e10097 * var_t3_dn4)) * var_pvagfactor) + (assign10770_e10099 * var_pvagfactor_dn4)), (((((((var_t1_dn5 * var_diblfactor) - (var_t1 * var_diblfactor_dn5)) / (var_diblfactor * var_diblfactor)) * var_t3) + (assign10770_e10097 * var_t3_dn5)) * var_pvagfactor) + (assign10770_e10099 * var_pvagfactor_dn5)), (((((((var_t1_dn6 * var_diblfactor) - (var_t1 * var_diblfactor_dn6)) / (var_diblfactor * var_diblfactor)) * var_t3) + (assign10770_e10097 * var_t3_dn6)) * var_pvagfactor) + (assign10770_e10099 * var_pvagfactor_dn6)), (((((((var_t1_dn7 * var_diblfactor) - (var_t1 * var_diblfactor_dn7)) / (var_diblfactor * var_diblfactor)) * var_t3) + (assign10770_e10097 * var_t3_dn7)) * var_pvagfactor) + (assign10770_e10099 * var_pvagfactor_dn7)), (((((((var_t1_dn8 * var_diblfactor) - (var_t1 * var_diblfactor_dn8)) / (var_diblfactor * var_diblfactor)) * var_t3) + (assign10770_e10097 * var_t3_dn8)) * var_pvagfactor) + (assign10770_e10099 * var_pvagfactor_dn8)),)
    } else {
        (var_vadibl, var_vadibl_dn3, var_vadibl_dn4, var_vadibl_dn5, var_vadibl_dn6, var_vadibl_dn7, var_vadibl_dn8,)
    }
};
        var_vadibl = assign10770_e10103;
        var_vadibl_dn3 = assign10770_e10103_d_n3;
        var_vadibl_dn4 = assign10770_e10103_d_n4;
        var_vadibl_dn5 = assign10770_e10103_d_n5;
        var_vadibl_dn6 = assign10770_e10103_d_n6;
        var_vadibl_dn7 = assign10770_e10103_d_n7;
        var_vadibl_dn8 = assign10770_e10103_d_n8;

        let (assign10780_e10111, assign10780_e10111_d_n3, assign10780_e10111_d_n4, assign10780_e10111_d_n5, assign10780_e10111_d_n6, assign10780_e10111_d_n7, assign10780_e10111_d_n8,) = {
    if (var_guard105 != 0.0) {
        let assign10780_e10108: f64 = (var_diffvds / var_vadibl);
        let assign10780_e10109: f64 = (1.0 + assign10780_e10108);
        (assign10780_e10109, (((var_diffvds_dn3 * var_vadibl) - (var_diffvds * var_vadibl_dn3)) / (var_vadibl * var_vadibl)), (((var_diffvds_dn4 * var_vadibl) - (var_diffvds * var_vadibl_dn4)) / (var_vadibl * var_vadibl)), (((var_diffvds_dn5 * var_vadibl) - (var_diffvds * var_vadibl_dn5)) / (var_vadibl * var_vadibl)), (((var_diffvds_dn6 * var_vadibl) - (var_diffvds * var_vadibl_dn6)) / (var_vadibl * var_vadibl)), (((var_diffvds_dn7 * var_vadibl) - (var_diffvds * var_vadibl_dn7)) / (var_vadibl * var_vadibl)), (((var_diffvds_dn8 * var_vadibl) - (var_diffvds * var_vadibl_dn8)) / (var_vadibl * var_vadibl)),)
    } else {
        (var_moc, var_moc_dn3, var_moc_dn4, var_moc_dn5, var_moc_dn6, var_moc_dn7, var_moc_dn8,)
    }
};
        var_moc = assign10780_e10111;
        var_moc_dn3 = assign10780_e10111_d_n3;
        var_moc_dn4 = assign10780_e10111_d_n4;
        var_moc_dn5 = assign10780_e10111_d_n5;
        var_moc_dn6 = assign10780_e10111_d_n6;
        var_moc_dn7 = assign10780_e10111_d_n7;
        var_moc_dn8 = assign10780_e10111_d_n8;

        *var_beta_slot = var_beta;
        *var_beta_dn3_slot = var_beta_dn3;
        *var_beta_dn4_slot = var_beta_dn4;
        *var_beta_dn5_slot = var_beta_dn5;
        *var_beta_dn6_slot = var_beta_dn6;
        *var_beta_dn7_slot = var_beta_dn7;
        *var_beta_dn8_slot = var_beta_dn8;
        *var_diffvds_slot = var_diffvds;
        *var_diffvds_dn3_slot = var_diffvds_dn3;
        *var_diffvds_dn4_slot = var_diffvds_dn4;
        *var_diffvds_dn5_slot = var_diffvds_dn5;
        *var_diffvds_dn6_slot = var_diffvds_dn6;
        *var_diffvds_dn7_slot = var_diffvds_dn7;
        *var_diffvds_dn8_slot = var_diffvds_dn8;
        *var_dmob_slot = var_dmob;
        *var_dmob_cv_slot = var_dmob_cv;
        *var_dmob_cv_dn3_slot = var_dmob_cv_dn3;
        *var_dmob_cv_dn4_slot = var_dmob_cv_dn4;
        *var_dmob_cv_dn5_slot = var_dmob_cv_dn5;
        *var_dmob_cv_dn6_slot = var_dmob_cv_dn6;
        *var_dmob_cv_dn7_slot = var_dmob_cv_dn7;
        *var_dmob_cv_dn8_slot = var_dmob_cv_dn8;
        *var_dmob_dn3_slot = var_dmob_dn3;
        *var_dmob_dn4_slot = var_dmob_dn4;
        *var_dmob_dn5_slot = var_dmob_dn5;
        *var_dmob_dn6_slot = var_dmob_dn6;
        *var_dmob_dn7_slot = var_dmob_dn7;
        *var_dmob_dn8_slot = var_dmob_dn8;
        *var_dvsat_slot = var_dvsat;
        *var_dvsat_dn3_slot = var_dvsat_dn3;
        *var_dvsat_dn4_slot = var_dvsat_dn4;
        *var_dvsat_dn5_slot = var_dvsat_dn5;
        *var_dvsat_dn6_slot = var_dvsat_dn6;
        *var_dvsat_dn7_slot = var_dvsat_dn7;
        *var_dvsat_dn8_slot = var_dvsat_dn8;
        *var_eeffm_cv_slot = var_eeffm_cv;
        *var_eeffm_cv_dn3_slot = var_eeffm_cv_dn3;
        *var_eeffm_cv_dn4_slot = var_eeffm_cv_dn4;
        *var_eeffm_cv_dn5_slot = var_eeffm_cv_dn5;
        *var_eeffm_cv_dn6_slot = var_eeffm_cv_dn6;
        *var_eeffm_cv_dn7_slot = var_eeffm_cv_dn7;
        *var_eeffm_cv_dn8_slot = var_eeffm_cv_dn8;
        *var_esat1_slot = var_esat1;
        *var_esat1_dn3_slot = var_esat1_dn3;
        *var_esat1_dn4_slot = var_esat1_dn4;
        *var_esat1_dn5_slot = var_esat1_dn5;
        *var_esat1_dn6_slot = var_esat1_dn6;
        *var_esat1_dn7_slot = var_esat1_dn7;
        *var_esat1_dn8_slot = var_esat1_dn8;
        *var_esat1l_slot = var_esat1l;
        *var_esat1l_dn3_slot = var_esat1l_dn3;
        *var_esat1l_dn4_slot = var_esat1l_dn4;
        *var_esat1l_dn5_slot = var_esat1l_dn5;
        *var_esat1l_dn6_slot = var_esat1l_dn6;
        *var_esat1l_dn7_slot = var_esat1l_dn7;
        *var_esat1l_dn8_slot = var_esat1l_dn8;
        *var_esatcv_slot = var_esatcv;
        *var_esatcv_dn3_slot = var_esatcv_dn3;
        *var_esatcv_dn4_slot = var_esatcv_dn4;
        *var_esatcv_dn5_slot = var_esatcv_dn5;
        *var_esatcv_dn6_slot = var_esatcv_dn6;
        *var_esatcv_dn7_slot = var_esatcv_dn7;
        *var_esatcv_dn8_slot = var_esatcv_dn8;
        *var_esatcvl_slot = var_esatcvl;
        *var_esatcvl_dn3_slot = var_esatcvl_dn3;
        *var_esatcvl_dn4_slot = var_esatcvl_dn4;
        *var_esatcvl_dn5_slot = var_esatcvl_dn5;
        *var_esatcvl_dn6_slot = var_esatcvl_dn6;
        *var_esatcvl_dn7_slot = var_esatcvl_dn7;
        *var_esatcvl_dn8_slot = var_esatcvl_dn8;
        *var_guard104_slot = var_guard104;
        *var_guard105_slot = var_guard105;
        *var_moc_slot = var_moc;
        *var_moc_dn3_slot = var_moc_dn3;
        *var_moc_dn4_slot = var_moc_dn4;
        *var_moc_dn5_slot = var_moc_dn5;
        *var_moc_dn6_slot = var_moc_dn6;
        *var_moc_dn7_slot = var_moc_dn7;
        *var_moc_dn8_slot = var_moc_dn8;
        *var_pvagfactor_slot = var_pvagfactor;
        *var_pvagfactor_dn3_slot = var_pvagfactor_dn3;
        *var_pvagfactor_dn4_slot = var_pvagfactor_dn4;
        *var_pvagfactor_dn5_slot = var_pvagfactor_dn5;
        *var_pvagfactor_dn6_slot = var_pvagfactor_dn6;
        *var_pvagfactor_dn7_slot = var_pvagfactor_dn7;
        *var_pvagfactor_dn8_slot = var_pvagfactor_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2__blk102_slot = var_t2__blk102;
        *var_t2__blk102_dn3_slot = var_t2__blk102_dn3;
        *var_t2__blk102_dn4_slot = var_t2__blk102_dn4;
        *var_t2__blk102_dn5_slot = var_t2__blk102_dn5;
        *var_t2__blk102_dn6_slot = var_t2__blk102_dn6;
        *var_t2__blk102_dn7_slot = var_t2__blk102_dn7;
        *var_t2__blk102_dn8_slot = var_t2__blk102_dn8;
        *var_t3_slot = var_t3;
        *var_t3__blk103_slot = var_t3__blk103;
        *var_t3__blk103_dn3_slot = var_t3__blk103_dn3;
        *var_t3__blk103_dn4_slot = var_t3__blk103_dn4;
        *var_t3__blk103_dn5_slot = var_t3__blk103_dn5;
        *var_t3__blk103_dn6_slot = var_t3__blk103_dn6;
        *var_t3__blk103_dn7_slot = var_t3__blk103_dn7;
        *var_t3__blk103_dn8_slot = var_t3__blk103_dn8;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_ueff1_slot = var_ueff1;
        *var_ueff1_dn3_slot = var_ueff1_dn3;
        *var_ueff1_dn4_slot = var_ueff1_dn4;
        *var_ueff1_dn5_slot = var_ueff1_dn5;
        *var_ueff1_dn6_slot = var_ueff1_dn6;
        *var_ueff1_dn7_slot = var_ueff1_dn7;
        *var_ueff1_dn8_slot = var_ueff1_dn8;
        *var_ueff2_slot = var_ueff2;
        *var_ueff2_dn3_slot = var_ueff2_dn3;
        *var_ueff2_dn4_slot = var_ueff2_dn4;
        *var_ueff2_dn5_slot = var_ueff2_dn5;
        *var_ueff2_dn6_slot = var_ueff2_dn6;
        *var_ueff2_dn7_slot = var_ueff2_dn7;
        *var_ueff2_dn8_slot = var_ueff2_dn8;
        *var_utotal_slot = var_utotal;
        *var_utotal_dn3_slot = var_utotal_dn3;
        *var_utotal_dn4_slot = var_utotal_dn4;
        *var_utotal_dn5_slot = var_utotal_dn5;
        *var_utotal_dn6_slot = var_utotal_dn6;
        *var_utotal_dn7_slot = var_utotal_dn7;
        *var_utotal_dn8_slot = var_utotal_dn8;
        *var_vadibl_slot = var_vadibl;
        *var_vadibl_dn3_slot = var_vadibl_dn3;
        *var_vadibl_dn4_slot = var_vadibl_dn4;
        *var_vadibl_dn5_slot = var_vadibl_dn5;
        *var_vadibl_dn6_slot = var_vadibl_dn6;
        *var_vadibl_dn7_slot = var_vadibl_dn7;
        *var_vadibl_dn8_slot = var_vadibl_dn8;
        *var_vgst2vtm_slot = var_vgst2vtm;
        *var_vgst2vtm_dn3_slot = var_vgst2vtm_dn3;
        *var_vgst2vtm_dn4_slot = var_vgst2vtm_dn4;
        *var_vgst2vtm_dn5_slot = var_vgst2vtm_dn5;
        *var_vgst2vtm_dn6_slot = var_vgst2vtm_dn6;
        *var_vgst2vtm_dn7_slot = var_vgst2vtm_dn7;
        *var_vgst2vtm_dn8_slot = var_vgst2vtm_dn8;
        *var_w1_slot = var_w1;
        *var_w1_dn3_slot = var_w1_dn3;
        *var_w1_dn4_slot = var_w1_dn4;
        *var_w1_dn5_slot = var_w1_dn5;
        *var_w1_dn6_slot = var_w1_dn6;
        *var_w1_dn7_slot = var_w1_dn7;
        *var_w1_dn8_slot = var_w1_dn8;
        *var_w2_slot = var_w2;
        *var_w2_dn3_slot = var_w2_dn3;
        *var_w2_dn4_slot = var_w2_dn4;
        *var_w2_dn5_slot = var_w2_dn5;
        *var_w2_dn6_slot = var_w2_dn6;
        *var_w2_dn7_slot = var_w2_dn7;
        *var_w2_dn8_slot = var_w2_dn8;
        *var_xsat_slot = var_xsat;
        *var_xsat_dn3_slot = var_xsat_dn3;
        *var_xsat_dn4_slot = var_xsat_dn4;
        *var_xsat_dn5_slot = var_xsat_dn5;
        *var_xsat_dn6_slot = var_xsat_dn6;
        *var_xsat_dn7_slot = var_xsat_dn7;
        *var_xsat_dn8_slot = var_xsat_dn8;
    }

    pub(super) fn stamp_transient_block_23(
        p: &Parameters,
        var_cox1: f64,
        var_csi: f64,
        var_diffvds: f64,
        var_diffvds_dn3: f64,
        var_diffvds_dn4: f64,
        var_diffvds_dn5: f64,
        var_diffvds_dn6: f64,
        var_diffvds_dn7: f64,
        var_diffvds_dn8: f64,
        var_dqi: f64,
        var_dqi_dn3: f64,
        var_dqi_dn4: f64,
        var_dqi_dn5: f64,
        var_dqi_dn6: f64,
        var_dqi_dn7: f64,
        var_dqi_dn8: f64,
        var_esatcvl: f64,
        var_esatcvl_dn3: f64,
        var_esatcvl_dn4: f64,
        var_esatcvl_dn5: f64,
        var_esatcvl_dn6: f64,
        var_esatcvl_dn7: f64,
        var_esatcvl_dn8: f64,
        var_esatl: f64,
        var_esatl_dn3: f64,
        var_esatl_dn4: f64,
        var_esatl_dn5: f64,
        var_esatl_dn6: f64,
        var_esatl_dn7: f64,
        var_esatl_dn8: f64,
        var_guard105: f64,
        var_k0_t: f64,
        var_k0_t_dn4: f64,
        var_k0si_t: f64,
        var_k0si_t_dn4: f64,
        var_k0sisat_t: f64,
        var_k0sisat_t_dn4: f64,
        var_nvtm: f64,
        var_nvtm_dn3: f64,
        var_nvtm_dn4: f64,
        var_nvtm_dn5: f64,
        var_nvtm_dn6: f64,
        var_nvtm_dn7: f64,
        var_nvtm_dn8: f64,
        var_pclm_i: f64,
        var_pclmcv_i: f64,
        var_prwb_i: f64,
        var_prwg_i: f64,
        var_qia: f64,
        var_qia_dn3: f64,
        var_qia_dn4: f64,
        var_qia_dn5: f64,
        var_qia_dn6: f64,
        var_qia_dn7: f64,
        var_qia_dn8: f64,
        var_qicored: f64,
        var_qicored_dn3: f64,
        var_qicored_dn4: f64,
        var_qicored_dn5: f64,
        var_qicored_dn6: f64,
        var_qicored_dn7: f64,
        var_qicored_dn8: f64,
        var_qicores: f64,
        var_qicores_dn3: f64,
        var_qicores_dn4: f64,
        var_qicores_dn5: f64,
        var_qicores_dn6: f64,
        var_qicores_dn7: f64,
        var_qicores_dn8: f64,
        var_qid: f64,
        var_qid_dn3: f64,
        var_qid_dn4: f64,
        var_qid_dn5: f64,
        var_qid_dn6: f64,
        var_qid_dn7: f64,
        var_qid_dn8: f64,
        var_qis: f64,
        var_qis_dn3: f64,
        var_qis_dn4: f64,
        var_qis_dn5: f64,
        var_qis_dn6: f64,
        var_qis_dn7: f64,
        var_qis_dn8: f64,
        var_rdstemp: f64,
        var_rdstemp_dn4: f64,
        var_rsourcegeo: f64,
        var_rsw_i: f64,
        var_rswmin_i: f64,
        var_vbgs_noswap: f64,
        var_vbgs_noswap_dn3: f64,
        var_vbgs_noswap_dn6: f64,
        var_vds: f64,
        var_vds_dn5: f64,
        var_vds_dn6: f64,
        var_vdsat: f64,
        var_vdsat_dn3: f64,
        var_vdsat_dn4: f64,
        var_vdsat_dn5: f64,
        var_vdsat_dn6: f64,
        var_vdsat_dn7: f64,
        var_vdsat_dn8: f64,
        var_vdseff: f64,
        var_vdseff_dn3: f64,
        var_vdseff_dn4: f64,
        var_vdseff_dn5: f64,
        var_vdseff_dn6: f64,
        var_vdseff_dn7: f64,
        var_vdseff_dn8: f64,
        var_vfbsd: f64,
        var_vfbsd_dn3: f64,
        var_vfbsd_dn4: f64,
        var_vfbsd_dn5: f64,
        var_vfbsd_dn6: f64,
        var_vfbsd_dn7: f64,
        var_vfbsd_dn8: f64,
        var_vgd_noswap: f64,
        var_vgd_noswap_dn5: f64,
        var_vgd_noswap_dn8: f64,
        var_vgs_noswap: f64,
        var_vgs_noswap_dn6: f64,
        var_vgs_noswap_dn8: f64,
        var_vtm: f64,
        var_vtm_dn4: f64,
        var_weffwrfactor: f64,
        var_dr_slot: &mut f64,
        var_dr_dn3_slot: &mut f64,
        var_dr_dn4_slot: &mut f64,
        var_dr_dn5_slot: &mut f64,
        var_dr_dn6_slot: &mut f64,
        var_dr_dn7_slot: &mut f64,
        var_dr_dn8_slot: &mut f64,
        var_guard106_slot: &mut f64,
        var_guard107_slot: &mut f64,
        var_guard108_slot: &mut f64,
        var_guard109_slot: &mut f64,
        var_guard116_slot: &mut f64,
        var_ids0_slot: &mut f64,
        var_ids0_dn3_slot: &mut f64,
        var_ids0_dn4_slot: &mut f64,
        var_ids0_dn5_slot: &mut f64,
        var_ids0_dn6_slot: &mut f64,
        var_ids0_dn7_slot: &mut f64,
        var_ids0_dn8_slot: &mut f64,
        var_ids0_ov_dqi_slot: &mut f64,
        var_ids0_ov_dqi_dn3_slot: &mut f64,
        var_ids0_ov_dqi_dn4_slot: &mut f64,
        var_ids0_ov_dqi_dn5_slot: &mut f64,
        var_ids0_ov_dqi_dn6_slot: &mut f64,
        var_ids0_ov_dqi_dn7_slot: &mut f64,
        var_ids0_ov_dqi_dn8_slot: &mut f64,
        var_mclm_slot: &mut f64,
        var_mclm_dn3_slot: &mut f64,
        var_mclm_dn4_slot: &mut f64,
        var_mclm_dn5_slot: &mut f64,
        var_mclm_dn6_slot: &mut f64,
        var_mclm_dn7_slot: &mut f64,
        var_mclm_dn8_slot: &mut f64,
        var_mclmcv_slot: &mut f64,
        var_mclmcv_dn3_slot: &mut f64,
        var_mclmcv_dn4_slot: &mut f64,
        var_mclmcv_dn5_slot: &mut f64,
        var_mclmcv_dn6_slot: &mut f64,
        var_mclmcv_dn7_slot: &mut f64,
        var_mclmcv_dn8_slot: &mut f64,
        var_mnud_slot: &mut f64,
        var_mnud_dn3_slot: &mut f64,
        var_mnud_dn4_slot: &mut f64,
        var_mnud_dn5_slot: &mut f64,
        var_mnud_dn6_slot: &mut f64,
        var_mnud_dn7_slot: &mut f64,
        var_mnud_dn8_slot: &mut f64,
        var_moc_slot: &mut f64,
        var_moc_dn3_slot: &mut f64,
        var_moc_dn4_slot: &mut f64,
        var_moc_dn5_slot: &mut f64,
        var_moc_dn6_slot: &mut f64,
        var_moc_dn7_slot: &mut f64,
        var_moc_dn8_slot: &mut f64,
        var_rdsi_slot: &mut f64,
        var_rdsi_dn3_slot: &mut f64,
        var_rdsi_dn4_slot: &mut f64,
        var_rdsi_dn5_slot: &mut f64,
        var_rdsi_dn6_slot: &mut f64,
        var_rdsi_dn7_slot: &mut f64,
        var_rdsi_dn8_slot: &mut f64,
        var_rsource_slot: &mut f64,
        var_rsource_dn3_slot: &mut f64,
        var_rsource_dn4_slot: &mut f64,
        var_rsource_dn5_slot: &mut f64,
        var_rsource_dn6_slot: &mut f64,
        var_rsource_dn7_slot: &mut f64,
        var_rsource_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1__blk110_slot: &mut f64,
        var_t1__blk110_dn3_slot: &mut f64,
        var_t1__blk110_dn4_slot: &mut f64,
        var_t1__blk110_dn5_slot: &mut f64,
        var_t1__blk110_dn6_slot: &mut f64,
        var_t1__blk110_dn7_slot: &mut f64,
        var_t1__blk110_dn8_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2__blk114_slot: &mut f64,
        var_t2__blk114_dn3_slot: &mut f64,
        var_t2__blk114_dn4_slot: &mut f64,
        var_t2__blk114_dn5_slot: &mut f64,
        var_t2__blk114_dn6_slot: &mut f64,
        var_t2__blk114_dn7_slot: &mut f64,
        var_t2__blk114_dn8_slot: &mut f64,
        var_t3__blk115_slot: &mut f64,
        var_t3__blk115_dn3_slot: &mut f64,
        var_t3__blk115_dn4_slot: &mut f64,
        var_t3__blk115_dn5_slot: &mut f64,
        var_t3__blk115_dn6_slot: &mut f64,
        var_t3__blk115_dn7_slot: &mut f64,
        var_t3__blk115_dn8_slot: &mut f64,
        var_t4__blk111_slot: &mut f64,
        var_t4__blk111_dn3_slot: &mut f64,
        var_t4__blk111_dn4_slot: &mut f64,
        var_t4__blk111_dn5_slot: &mut f64,
        var_t4__blk111_dn6_slot: &mut f64,
        var_t4__blk111_dn7_slot: &mut f64,
        var_t4__blk111_dn8_slot: &mut f64,
        var_vgd_eff_slot: &mut f64,
        var_vgd_eff_dn3_slot: &mut f64,
        var_vgd_eff_dn4_slot: &mut f64,
        var_vgd_eff_dn5_slot: &mut f64,
        var_vgd_eff_dn6_slot: &mut f64,
        var_vgd_eff_dn7_slot: &mut f64,
        var_vgd_eff_dn8_slot: &mut f64,
        var_vgs_eff_slot: &mut f64,
        var_vgs_eff_dn3_slot: &mut f64,
        var_vgs_eff_dn4_slot: &mut f64,
        var_vgs_eff_dn5_slot: &mut f64,
        var_vgs_eff_dn6_slot: &mut f64,
        var_vgs_eff_dn7_slot: &mut f64,
        var_vgs_eff_dn8_slot: &mut f64,
    ) {
        let mut var_dr: f64 = *var_dr_slot;
        let mut var_dr_dn3: f64 = *var_dr_dn3_slot;
        let mut var_dr_dn4: f64 = *var_dr_dn4_slot;
        let mut var_dr_dn5: f64 = *var_dr_dn5_slot;
        let mut var_dr_dn6: f64 = *var_dr_dn6_slot;
        let mut var_dr_dn7: f64 = *var_dr_dn7_slot;
        let mut var_dr_dn8: f64 = *var_dr_dn8_slot;
        let mut var_guard106: f64 = *var_guard106_slot;
        let mut var_guard107: f64 = *var_guard107_slot;
        let mut var_guard108: f64 = *var_guard108_slot;
        let mut var_guard109: f64 = *var_guard109_slot;
        let mut var_guard116: f64 = *var_guard116_slot;
        let mut var_ids0: f64 = *var_ids0_slot;
        let mut var_ids0_dn3: f64 = *var_ids0_dn3_slot;
        let mut var_ids0_dn4: f64 = *var_ids0_dn4_slot;
        let mut var_ids0_dn5: f64 = *var_ids0_dn5_slot;
        let mut var_ids0_dn6: f64 = *var_ids0_dn6_slot;
        let mut var_ids0_dn7: f64 = *var_ids0_dn7_slot;
        let mut var_ids0_dn8: f64 = *var_ids0_dn8_slot;
        let mut var_ids0_ov_dqi: f64 = *var_ids0_ov_dqi_slot;
        let mut var_ids0_ov_dqi_dn3: f64 = *var_ids0_ov_dqi_dn3_slot;
        let mut var_ids0_ov_dqi_dn4: f64 = *var_ids0_ov_dqi_dn4_slot;
        let mut var_ids0_ov_dqi_dn5: f64 = *var_ids0_ov_dqi_dn5_slot;
        let mut var_ids0_ov_dqi_dn6: f64 = *var_ids0_ov_dqi_dn6_slot;
        let mut var_ids0_ov_dqi_dn7: f64 = *var_ids0_ov_dqi_dn7_slot;
        let mut var_ids0_ov_dqi_dn8: f64 = *var_ids0_ov_dqi_dn8_slot;
        let mut var_mclm: f64 = *var_mclm_slot;
        let mut var_mclm_dn3: f64 = *var_mclm_dn3_slot;
        let mut var_mclm_dn4: f64 = *var_mclm_dn4_slot;
        let mut var_mclm_dn5: f64 = *var_mclm_dn5_slot;
        let mut var_mclm_dn6: f64 = *var_mclm_dn6_slot;
        let mut var_mclm_dn7: f64 = *var_mclm_dn7_slot;
        let mut var_mclm_dn8: f64 = *var_mclm_dn8_slot;
        let mut var_mclmcv: f64 = *var_mclmcv_slot;
        let mut var_mclmcv_dn3: f64 = *var_mclmcv_dn3_slot;
        let mut var_mclmcv_dn4: f64 = *var_mclmcv_dn4_slot;
        let mut var_mclmcv_dn5: f64 = *var_mclmcv_dn5_slot;
        let mut var_mclmcv_dn6: f64 = *var_mclmcv_dn6_slot;
        let mut var_mclmcv_dn7: f64 = *var_mclmcv_dn7_slot;
        let mut var_mclmcv_dn8: f64 = *var_mclmcv_dn8_slot;
        let mut var_mnud: f64 = *var_mnud_slot;
        let mut var_mnud_dn3: f64 = *var_mnud_dn3_slot;
        let mut var_mnud_dn4: f64 = *var_mnud_dn4_slot;
        let mut var_mnud_dn5: f64 = *var_mnud_dn5_slot;
        let mut var_mnud_dn6: f64 = *var_mnud_dn6_slot;
        let mut var_mnud_dn7: f64 = *var_mnud_dn7_slot;
        let mut var_mnud_dn8: f64 = *var_mnud_dn8_slot;
        let mut var_moc: f64 = *var_moc_slot;
        let mut var_moc_dn3: f64 = *var_moc_dn3_slot;
        let mut var_moc_dn4: f64 = *var_moc_dn4_slot;
        let mut var_moc_dn5: f64 = *var_moc_dn5_slot;
        let mut var_moc_dn6: f64 = *var_moc_dn6_slot;
        let mut var_moc_dn7: f64 = *var_moc_dn7_slot;
        let mut var_moc_dn8: f64 = *var_moc_dn8_slot;
        let mut var_rdsi: f64 = *var_rdsi_slot;
        let mut var_rdsi_dn3: f64 = *var_rdsi_dn3_slot;
        let mut var_rdsi_dn4: f64 = *var_rdsi_dn4_slot;
        let mut var_rdsi_dn5: f64 = *var_rdsi_dn5_slot;
        let mut var_rdsi_dn6: f64 = *var_rdsi_dn6_slot;
        let mut var_rdsi_dn7: f64 = *var_rdsi_dn7_slot;
        let mut var_rdsi_dn8: f64 = *var_rdsi_dn8_slot;
        let mut var_rsource: f64 = *var_rsource_slot;
        let mut var_rsource_dn3: f64 = *var_rsource_dn3_slot;
        let mut var_rsource_dn4: f64 = *var_rsource_dn4_slot;
        let mut var_rsource_dn5: f64 = *var_rsource_dn5_slot;
        let mut var_rsource_dn6: f64 = *var_rsource_dn6_slot;
        let mut var_rsource_dn7: f64 = *var_rsource_dn7_slot;
        let mut var_rsource_dn8: f64 = *var_rsource_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1__blk110: f64 = *var_t1__blk110_slot;
        let mut var_t1__blk110_dn3: f64 = *var_t1__blk110_dn3_slot;
        let mut var_t1__blk110_dn4: f64 = *var_t1__blk110_dn4_slot;
        let mut var_t1__blk110_dn5: f64 = *var_t1__blk110_dn5_slot;
        let mut var_t1__blk110_dn6: f64 = *var_t1__blk110_dn6_slot;
        let mut var_t1__blk110_dn7: f64 = *var_t1__blk110_dn7_slot;
        let mut var_t1__blk110_dn8: f64 = *var_t1__blk110_dn8_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2__blk114: f64 = *var_t2__blk114_slot;
        let mut var_t2__blk114_dn3: f64 = *var_t2__blk114_dn3_slot;
        let mut var_t2__blk114_dn4: f64 = *var_t2__blk114_dn4_slot;
        let mut var_t2__blk114_dn5: f64 = *var_t2__blk114_dn5_slot;
        let mut var_t2__blk114_dn6: f64 = *var_t2__blk114_dn6_slot;
        let mut var_t2__blk114_dn7: f64 = *var_t2__blk114_dn7_slot;
        let mut var_t2__blk114_dn8: f64 = *var_t2__blk114_dn8_slot;
        let mut var_t3__blk115: f64 = *var_t3__blk115_slot;
        let mut var_t3__blk115_dn3: f64 = *var_t3__blk115_dn3_slot;
        let mut var_t3__blk115_dn4: f64 = *var_t3__blk115_dn4_slot;
        let mut var_t3__blk115_dn5: f64 = *var_t3__blk115_dn5_slot;
        let mut var_t3__blk115_dn6: f64 = *var_t3__blk115_dn6_slot;
        let mut var_t3__blk115_dn7: f64 = *var_t3__blk115_dn7_slot;
        let mut var_t3__blk115_dn8: f64 = *var_t3__blk115_dn8_slot;
        let mut var_t4__blk111: f64 = *var_t4__blk111_slot;
        let mut var_t4__blk111_dn3: f64 = *var_t4__blk111_dn3_slot;
        let mut var_t4__blk111_dn4: f64 = *var_t4__blk111_dn4_slot;
        let mut var_t4__blk111_dn5: f64 = *var_t4__blk111_dn5_slot;
        let mut var_t4__blk111_dn6: f64 = *var_t4__blk111_dn6_slot;
        let mut var_t4__blk111_dn7: f64 = *var_t4__blk111_dn7_slot;
        let mut var_t4__blk111_dn8: f64 = *var_t4__blk111_dn8_slot;
        let mut var_vgd_eff: f64 = *var_vgd_eff_slot;
        let mut var_vgd_eff_dn3: f64 = *var_vgd_eff_dn3_slot;
        let mut var_vgd_eff_dn4: f64 = *var_vgd_eff_dn4_slot;
        let mut var_vgd_eff_dn5: f64 = *var_vgd_eff_dn5_slot;
        let mut var_vgd_eff_dn6: f64 = *var_vgd_eff_dn6_slot;
        let mut var_vgd_eff_dn7: f64 = *var_vgd_eff_dn7_slot;
        let mut var_vgd_eff_dn8: f64 = *var_vgd_eff_dn8_slot;
        let mut var_vgs_eff: f64 = *var_vgs_eff_slot;
        let mut var_vgs_eff_dn3: f64 = *var_vgs_eff_dn3_slot;
        let mut var_vgs_eff_dn4: f64 = *var_vgs_eff_dn4_slot;
        let mut var_vgs_eff_dn5: f64 = *var_vgs_eff_dn5_slot;
        let mut var_vgs_eff_dn6: f64 = *var_vgs_eff_dn6_slot;
        let mut var_vgs_eff_dn7: f64 = *var_vgs_eff_dn7_slot;
        let mut var_vgs_eff_dn8: f64 = *var_vgs_eff_dn8_slot;

        let (assign10790_e10116, assign10790_e10116_d_n3, assign10790_e10116_d_n4, assign10790_e10116_d_n5, assign10790_e10116_d_n6, assign10790_e10116_d_n7, assign10790_e10116_d_n8,) = {
    if (var_guard105 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_moc, var_moc_dn3, var_moc_dn4, var_moc_dn5, var_moc_dn6, var_moc_dn7, var_moc_dn8,)
    }
};
        var_moc = assign10790_e10116;
        var_moc_dn3 = assign10790_e10116_d_n3;
        var_moc_dn4 = assign10790_e10116_d_n4;
        var_moc_dn5 = assign10790_e10116_d_n5;
        var_moc_dn6 = assign10790_e10116_d_n6;
        var_moc_dn7 = assign10790_e10116_d_n7;
        var_moc_dn8 = assign10790_e10116_d_n8;

        let assign10800_e10119: f64 = if var_pclm_i > 0.0 { 1.0 } else { 0.0 };
        var_guard106 = assign10800_e10119;

        let assign10810_e10122: f64 = if p.p213 < 0.0 { 1.0 } else { 0.0 };
        var_guard107 = assign10810_e10122;

        let (assign10820_e10136, assign10820_e10136_d_n3, assign10820_e10136_d_n4, assign10820_e10136_d_n5, assign10820_e10136_d_n6, assign10820_e10136_d_n7, assign10820_e10136_d_n8,) = {
    if ((var_guard106 != 0.0) && (var_guard107 != 0.0)) {
        let assign10820_e10129: f64 = (1.0 / var_pclm_i);
        let assign10820_e10132: f64 = (p.p213 * var_qia);
        let assign10820_e10133: f64 = (assign10820_e10129 - assign10820_e10132);
        let assign10820_e10134: f64 = (1.0 / assign10820_e10133);
        (assign10820_e10134, (-((-(p.p213 * var_qia_dn3)) / (assign10820_e10133 * assign10820_e10133))), (-((-(p.p213 * var_qia_dn4)) / (assign10820_e10133 * assign10820_e10133))), (-((-(p.p213 * var_qia_dn5)) / (assign10820_e10133 * assign10820_e10133))), (-((-(p.p213 * var_qia_dn6)) / (assign10820_e10133 * assign10820_e10133))), (-((-(p.p213 * var_qia_dn7)) / (assign10820_e10133 * assign10820_e10133))), (-((-(p.p213 * var_qia_dn8)) / (assign10820_e10133 * assign10820_e10133))),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign10820_e10136;
        var_t1_dn3 = assign10820_e10136_d_n3;
        var_t1_dn4 = assign10820_e10136_d_n4;
        var_t1_dn5 = assign10820_e10136_d_n5;
        var_t1_dn6 = assign10820_e10136_d_n6;
        var_t1_dn7 = assign10820_e10136_d_n7;
        var_t1_dn8 = assign10820_e10136_d_n8;

        let (assign10830_e10149, assign10830_e10149_d_n3, assign10830_e10149_d_n4, assign10830_e10149_d_n5, assign10830_e10149_d_n6, assign10830_e10149_d_n7, assign10830_e10149_d_n8,) = {
    if ((var_guard106 != 0.0) && (var_guard107 == 0.0)) {
        let assign10830_e10145: f64 = (p.p213 * var_qia);
        let assign10830_e10146: f64 = (1.0 + assign10830_e10145);
        let assign10830_e10147: f64 = (var_pclm_i * assign10830_e10146);
        (assign10830_e10147, (var_pclm_i * (p.p213 * var_qia_dn3)), (var_pclm_i * (p.p213 * var_qia_dn4)), (var_pclm_i * (p.p213 * var_qia_dn5)), (var_pclm_i * (p.p213 * var_qia_dn6)), (var_pclm_i * (p.p213 * var_qia_dn7)), (var_pclm_i * (p.p213 * var_qia_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign10830_e10149;
        var_t1_dn3 = assign10830_e10149_d_n3;
        var_t1_dn4 = assign10830_e10149_d_n4;
        var_t1_dn5 = assign10830_e10149_d_n5;
        var_t1_dn6 = assign10830_e10149_d_n6;
        var_t1_dn7 = assign10830_e10149_d_n7;
        var_t1_dn8 = assign10830_e10149_d_n8;

        let (assign10840_e10168, assign10840_e10168_d_n3, assign10840_e10168_d_n4, assign10840_e10168_d_n5, assign10840_e10168_d_n6, assign10840_e10168_d_n7, assign10840_e10168_d_n8,) = {
    if (var_guard106 != 0.0) {
        let assign10840_e10156: f64 = (var_diffvds / var_t1);
        let assign10840_e10159: f64 = (var_vdsat + var_esatl);
        let assign10840_e10160: f64 = (assign10840_e10156 / assign10840_e10159);
        let assign10840_e10161: f64 = (1.0 + assign10840_e10160);
        let assign10840_e10163: f64 = (assign10840_e10161).max(1e-38);
        let assign10840_e10164: f64 = (assign10840_e10163).ln();
        let assign10840_e10165: f64 = (var_t1 * assign10840_e10164);
        let assign10840_e10166: f64 = (1.0 + assign10840_e10165);
        (assign10840_e10166, ((var_t1_dn3 * assign10840_e10164) + (var_t1 * (if assign10840_e10161 >= 1e-38 { ((((((var_diffvds_dn3 * var_t1) - (var_diffvds * var_t1_dn3)) / (var_t1 * var_t1)) * assign10840_e10159) - (assign10840_e10156 * (var_vdsat_dn3 + var_esatl_dn3))) / (assign10840_e10159 * assign10840_e10159)) } else { 0.0 } / assign10840_e10163))), ((var_t1_dn4 * assign10840_e10164) + (var_t1 * (if assign10840_e10161 >= 1e-38 { ((((((var_diffvds_dn4 * var_t1) - (var_diffvds * var_t1_dn4)) / (var_t1 * var_t1)) * assign10840_e10159) - (assign10840_e10156 * (var_vdsat_dn4 + var_esatl_dn4))) / (assign10840_e10159 * assign10840_e10159)) } else { 0.0 } / assign10840_e10163))), ((var_t1_dn5 * assign10840_e10164) + (var_t1 * (if assign10840_e10161 >= 1e-38 { ((((((var_diffvds_dn5 * var_t1) - (var_diffvds * var_t1_dn5)) / (var_t1 * var_t1)) * assign10840_e10159) - (assign10840_e10156 * (var_vdsat_dn5 + var_esatl_dn5))) / (assign10840_e10159 * assign10840_e10159)) } else { 0.0 } / assign10840_e10163))), ((var_t1_dn6 * assign10840_e10164) + (var_t1 * (if assign10840_e10161 >= 1e-38 { ((((((var_diffvds_dn6 * var_t1) - (var_diffvds * var_t1_dn6)) / (var_t1 * var_t1)) * assign10840_e10159) - (assign10840_e10156 * (var_vdsat_dn6 + var_esatl_dn6))) / (assign10840_e10159 * assign10840_e10159)) } else { 0.0 } / assign10840_e10163))), ((var_t1_dn7 * assign10840_e10164) + (var_t1 * (if assign10840_e10161 >= 1e-38 { ((((((var_diffvds_dn7 * var_t1) - (var_diffvds * var_t1_dn7)) / (var_t1 * var_t1)) * assign10840_e10159) - (assign10840_e10156 * (var_vdsat_dn7 + var_esatl_dn7))) / (assign10840_e10159 * assign10840_e10159)) } else { 0.0 } / assign10840_e10163))), ((var_t1_dn8 * assign10840_e10164) + (var_t1 * (if assign10840_e10161 >= 1e-38 { ((((((var_diffvds_dn8 * var_t1) - (var_diffvds * var_t1_dn8)) / (var_t1 * var_t1)) * assign10840_e10159) - (assign10840_e10156 * (var_vdsat_dn8 + var_esatl_dn8))) / (assign10840_e10159 * assign10840_e10159)) } else { 0.0 } / assign10840_e10163))),)
    } else {
        (var_mclm, var_mclm_dn3, var_mclm_dn4, var_mclm_dn5, var_mclm_dn6, var_mclm_dn7, var_mclm_dn8,)
    }
};
        var_mclm = assign10840_e10168;
        var_mclm_dn3 = assign10840_e10168_d_n3;
        var_mclm_dn4 = assign10840_e10168_d_n4;
        var_mclm_dn5 = assign10840_e10168_d_n5;
        var_mclm_dn6 = assign10840_e10168_d_n6;
        var_mclm_dn7 = assign10840_e10168_d_n7;
        var_mclm_dn8 = assign10840_e10168_d_n8;

        let (assign10850_e10173, assign10850_e10173_d_n3, assign10850_e10173_d_n4, assign10850_e10173_d_n5, assign10850_e10173_d_n6, assign10850_e10173_d_n7, assign10850_e10173_d_n8,) = {
    if (var_guard106 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_mclm, var_mclm_dn3, var_mclm_dn4, var_mclm_dn5, var_mclm_dn6, var_mclm_dn7, var_mclm_dn8,)
    }
};
        var_mclm = assign10850_e10173;
        var_mclm_dn3 = assign10850_e10173_d_n3;
        var_mclm_dn4 = assign10850_e10173_d_n4;
        var_mclm_dn5 = assign10850_e10173_d_n5;
        var_mclm_dn6 = assign10850_e10173_d_n6;
        var_mclm_dn7 = assign10850_e10173_d_n7;
        var_mclm_dn8 = assign10850_e10173_d_n8;

        let assign10860_e10176: f64 = (var_moc * var_mclm);
        var_moc = assign10860_e10176;
        var_moc_dn3 = ((var_moc_dn3 * var_mclm) + (var_moc * var_mclm_dn3));
        var_moc_dn4 = ((var_moc_dn4 * var_mclm) + (var_moc * var_mclm_dn4));
        var_moc_dn5 = ((var_moc_dn5 * var_mclm) + (var_moc * var_mclm_dn5));
        var_moc_dn6 = ((var_moc_dn6 * var_mclm) + (var_moc * var_mclm_dn6));
        var_moc_dn7 = ((var_moc_dn7 * var_mclm) + (var_moc * var_mclm_dn7));
        var_moc_dn8 = ((var_moc_dn8 * var_mclm) + (var_moc * var_mclm_dn8));

        let assign10870_e10179: f64 = if var_pclmcv_i > 0.0 { 1.0 } else { 0.0 };
        var_guard108 = assign10870_e10179;

        let (assign10880_e10200, assign10880_e10200_d_n3, assign10880_e10200_d_n4, assign10880_e10200_d_n5, assign10880_e10200_d_n6, assign10880_e10200_d_n7, assign10880_e10200_d_n8,) = {
    if (var_guard108 != 0.0) {
        let assign10880_e10186: f64 = (var_vds - var_vdseff);
        let assign10880_e10188: f64 = (assign10880_e10186 / var_pclmcv_i);
        let assign10880_e10191: f64 = (var_vdsat + var_esatcvl);
        let assign10880_e10192: f64 = (assign10880_e10188 / assign10880_e10191);
        let assign10880_e10193: f64 = (1.0 + assign10880_e10192);
        let assign10880_e10195: f64 = (assign10880_e10193).max(1e-38);
        let assign10880_e10196: f64 = (assign10880_e10195).ln();
        let assign10880_e10197: f64 = (var_pclmcv_i * assign10880_e10196);
        let assign10880_e10198: f64 = (1.0 + assign10880_e10197);
        (assign10880_e10198, (var_pclmcv_i * (if assign10880_e10193 >= 1e-38 { (((((-var_vdseff_dn3) / var_pclmcv_i) * assign10880_e10191) - (assign10880_e10188 * (var_vdsat_dn3 + var_esatcvl_dn3))) / (assign10880_e10191 * assign10880_e10191)) } else { 0.0 } / assign10880_e10195)), (var_pclmcv_i * (if assign10880_e10193 >= 1e-38 { (((((-var_vdseff_dn4) / var_pclmcv_i) * assign10880_e10191) - (assign10880_e10188 * (var_vdsat_dn4 + var_esatcvl_dn4))) / (assign10880_e10191 * assign10880_e10191)) } else { 0.0 } / assign10880_e10195)), (var_pclmcv_i * (if assign10880_e10193 >= 1e-38 { (((((var_vds_dn5 - var_vdseff_dn5) / var_pclmcv_i) * assign10880_e10191) - (assign10880_e10188 * (var_vdsat_dn5 + var_esatcvl_dn5))) / (assign10880_e10191 * assign10880_e10191)) } else { 0.0 } / assign10880_e10195)), (var_pclmcv_i * (if assign10880_e10193 >= 1e-38 { (((((var_vds_dn6 - var_vdseff_dn6) / var_pclmcv_i) * assign10880_e10191) - (assign10880_e10188 * (var_vdsat_dn6 + var_esatcvl_dn6))) / (assign10880_e10191 * assign10880_e10191)) } else { 0.0 } / assign10880_e10195)), (var_pclmcv_i * (if assign10880_e10193 >= 1e-38 { (((((-var_vdseff_dn7) / var_pclmcv_i) * assign10880_e10191) - (assign10880_e10188 * (var_vdsat_dn7 + var_esatcvl_dn7))) / (assign10880_e10191 * assign10880_e10191)) } else { 0.0 } / assign10880_e10195)), (var_pclmcv_i * (if assign10880_e10193 >= 1e-38 { (((((-var_vdseff_dn8) / var_pclmcv_i) * assign10880_e10191) - (assign10880_e10188 * (var_vdsat_dn8 + var_esatcvl_dn8))) / (assign10880_e10191 * assign10880_e10191)) } else { 0.0 } / assign10880_e10195)),)
    } else {
        (var_mclmcv, var_mclmcv_dn3, var_mclmcv_dn4, var_mclmcv_dn5, var_mclmcv_dn6, var_mclmcv_dn7, var_mclmcv_dn8,)
    }
};
        var_mclmcv = assign10880_e10200;
        var_mclmcv_dn3 = assign10880_e10200_d_n3;
        var_mclmcv_dn4 = assign10880_e10200_d_n4;
        var_mclmcv_dn5 = assign10880_e10200_d_n5;
        var_mclmcv_dn6 = assign10880_e10200_d_n6;
        var_mclmcv_dn7 = assign10880_e10200_d_n7;
        var_mclmcv_dn8 = assign10880_e10200_d_n8;

        let (assign10890_e10205, assign10890_e10205_d_n3, assign10890_e10205_d_n4, assign10890_e10205_d_n5, assign10890_e10205_d_n6, assign10890_e10205_d_n7, assign10890_e10205_d_n8,) = {
    if (var_guard108 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_mclmcv, var_mclmcv_dn3, var_mclmcv_dn4, var_mclmcv_dn5, var_mclmcv_dn6, var_mclmcv_dn7, var_mclmcv_dn8,)
    }
};
        var_mclmcv = assign10890_e10205;
        var_mclmcv_dn3 = assign10890_e10205_d_n3;
        var_mclmcv_dn4 = assign10890_e10205_d_n4;
        var_mclmcv_dn5 = assign10890_e10205_d_n5;
        var_mclmcv_dn6 = assign10890_e10205_d_n6;
        var_mclmcv_dn7 = assign10890_e10205_d_n7;
        var_mclmcv_dn8 = assign10890_e10205_d_n8;

        let assign10900_e10208: f64 = if var_k0_t != 0.0 { 1.0 } else { 0.0 };
        var_guard109 = assign10900_e10208;

        let (assign10910_e10228, assign10910_e10228_d_n3, assign10910_e10228_d_n4, assign10910_e10228_d_n5, assign10910_e10228_d_n6, assign10910_e10228_d_n7, assign10910_e10228_d_n8,) = {
    if (var_guard109 != 0.0) {
        let assign10910_e10215: f64 = (var_k0sisat_t * var_dqi);
        let assign10910_e10217: f64 = (assign10910_e10215 * var_dqi);
        let assign10910_e10218: f64 = (var_k0si_t + assign10910_e10217);
        let assign10910_e10219: f64 = (0.0_f64).max(assign10910_e10218);
        let assign10910_e10221: f64 = (assign10910_e10219 * var_qia);
        let assign10910_e10224: f64 = (2.0 * var_nvtm);
        let assign10910_e10225: f64 = (assign10910_e10221 + assign10910_e10224);
        let assign10910_e10226: f64 = (var_k0_t / assign10910_e10225);
        (assign10910_e10226, (-((var_k0_t * (((if 0.0 >= assign10910_e10218 { 0.0 } else { (((var_k0sisat_t * var_dqi_dn3) * var_dqi) + (assign10910_e10215 * var_dqi_dn3)) } * var_qia) + (assign10910_e10219 * var_qia_dn3)) + (2.0 * var_nvtm_dn3))) / (assign10910_e10225 * assign10910_e10225))), (((var_k0_t_dn4 * assign10910_e10225) - (var_k0_t * (((if 0.0 >= assign10910_e10218 { 0.0 } else { (var_k0si_t_dn4 + ((((var_k0sisat_t_dn4 * var_dqi) + (var_k0sisat_t * var_dqi_dn4)) * var_dqi) + (assign10910_e10215 * var_dqi_dn4))) } * var_qia) + (assign10910_e10219 * var_qia_dn4)) + (2.0 * var_nvtm_dn4)))) / (assign10910_e10225 * assign10910_e10225)), (-((var_k0_t * (((if 0.0 >= assign10910_e10218 { 0.0 } else { (((var_k0sisat_t * var_dqi_dn5) * var_dqi) + (assign10910_e10215 * var_dqi_dn5)) } * var_qia) + (assign10910_e10219 * var_qia_dn5)) + (2.0 * var_nvtm_dn5))) / (assign10910_e10225 * assign10910_e10225))), (-((var_k0_t * (((if 0.0 >= assign10910_e10218 { 0.0 } else { (((var_k0sisat_t * var_dqi_dn6) * var_dqi) + (assign10910_e10215 * var_dqi_dn6)) } * var_qia) + (assign10910_e10219 * var_qia_dn6)) + (2.0 * var_nvtm_dn6))) / (assign10910_e10225 * assign10910_e10225))), (-((var_k0_t * (((if 0.0 >= assign10910_e10218 { 0.0 } else { (((var_k0sisat_t * var_dqi_dn7) * var_dqi) + (assign10910_e10215 * var_dqi_dn7)) } * var_qia) + (assign10910_e10219 * var_qia_dn7)) + (2.0 * var_nvtm_dn7))) / (assign10910_e10225 * assign10910_e10225))), (-((var_k0_t * (((if 0.0 >= assign10910_e10218 { 0.0 } else { (((var_k0sisat_t * var_dqi_dn8) * var_dqi) + (assign10910_e10215 * var_dqi_dn8)) } * var_qia) + (assign10910_e10219 * var_qia_dn8)) + (2.0 * var_nvtm_dn8))) / (assign10910_e10225 * assign10910_e10225))),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign10910_e10228;
        var_t1_dn3 = assign10910_e10228_d_n3;
        var_t1_dn4 = assign10910_e10228_d_n4;
        var_t1_dn5 = assign10910_e10228_d_n5;
        var_t1_dn6 = assign10910_e10228_d_n6;
        var_t1_dn7 = assign10910_e10228_d_n7;
        var_t1_dn8 = assign10910_e10228_d_n8;

        let (assign10920_e10234, assign10920_e10234_d_n3, assign10920_e10234_d_n4, assign10920_e10234_d_n5, assign10920_e10234_d_n6, assign10920_e10234_d_n7, assign10920_e10234_d_n8,) = {
    if (var_guard109 != 0.0) {
        let assign10920_e10231: f64 = (-var_t1);
        let assign10920_e10232: f64 = { let limited_exp_arg = assign10920_e10231; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign10920_e10232, ({ let limited_exp_arg = assign10920_e10231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_dn3)), ({ let limited_exp_arg = assign10920_e10231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_dn4)), ({ let limited_exp_arg = assign10920_e10231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_dn5)), ({ let limited_exp_arg = assign10920_e10231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_dn6)), ({ let limited_exp_arg = assign10920_e10231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_dn7)), ({ let limited_exp_arg = assign10920_e10231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_dn8)),)
    } else {
        (var_mnud, var_mnud_dn3, var_mnud_dn4, var_mnud_dn5, var_mnud_dn6, var_mnud_dn7, var_mnud_dn8,)
    }
};
        var_mnud = assign10920_e10234;
        var_mnud_dn3 = assign10920_e10234_d_n3;
        var_mnud_dn4 = assign10920_e10234_d_n4;
        var_mnud_dn5 = assign10920_e10234_d_n5;
        var_mnud_dn6 = assign10920_e10234_d_n6;
        var_mnud_dn7 = assign10920_e10234_d_n7;
        var_mnud_dn8 = assign10920_e10234_d_n8;

        let (assign10930_e10239, assign10930_e10239_d_n3, assign10930_e10239_d_n4, assign10930_e10239_d_n5, assign10930_e10239_d_n6, assign10930_e10239_d_n7, assign10930_e10239_d_n8,) = {
    if (var_guard109 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_mnud, var_mnud_dn3, var_mnud_dn4, var_mnud_dn5, var_mnud_dn6, var_mnud_dn7, var_mnud_dn8,)
    }
};
        var_mnud = assign10930_e10239;
        var_mnud_dn3 = assign10930_e10239_d_n3;
        var_mnud_dn4 = assign10930_e10239_d_n4;
        var_mnud_dn5 = assign10930_e10239_d_n5;
        var_mnud_dn6 = assign10930_e10239_d_n6;
        var_mnud_dn7 = assign10930_e10239_d_n7;
        var_mnud_dn8 = assign10930_e10239_d_n8;

        let assign10940_e10242: f64 = (var_qicores - var_qicored);
        var_t0 = assign10940_e10242;
        var_t0_dn3 = (var_qicores_dn3 - var_qicored_dn3);
        var_t0_dn4 = (var_qicores_dn4 - var_qicored_dn4);
        var_t0_dn5 = (var_qicores_dn5 - var_qicored_dn5);
        var_t0_dn6 = (var_qicores_dn6 - var_qicored_dn6);
        var_t0_dn7 = (var_qicores_dn7 - var_qicored_dn7);
        var_t0_dn8 = (var_qicores_dn8 - var_qicored_dn8);

        let assign10950_e10245: f64 = (var_qicores * var_qicores);
        let assign10950_e10248: f64 = (var_qicored * var_qicored);
        let assign10950_e10249: f64 = (assign10950_e10245 - assign10950_e10248);
        var_t1 = assign10950_e10249;
        var_t1_dn3 = (((var_qicores_dn3 * var_qicores) + (var_qicores * var_qicores_dn3)) - ((var_qicored_dn3 * var_qicored) + (var_qicored * var_qicored_dn3)));
        var_t1_dn4 = (((var_qicores_dn4 * var_qicores) + (var_qicores * var_qicores_dn4)) - ((var_qicored_dn4 * var_qicored) + (var_qicored * var_qicored_dn4)));
        var_t1_dn5 = (((var_qicores_dn5 * var_qicores) + (var_qicores * var_qicores_dn5)) - ((var_qicored_dn5 * var_qicored) + (var_qicored * var_qicored_dn5)));
        var_t1_dn6 = (((var_qicores_dn6 * var_qicores) + (var_qicores * var_qicores_dn6)) - ((var_qicored_dn6 * var_qicored) + (var_qicored * var_qicored_dn6)));
        var_t1_dn7 = (((var_qicores_dn7 * var_qicores) + (var_qicores * var_qicores_dn7)) - ((var_qicored_dn7 * var_qicored) + (var_qicored * var_qicored_dn7)));
        var_t1_dn8 = (((var_qicores_dn8 * var_qicores) + (var_qicores * var_qicores_dn8)) - ((var_qicored_dn8 * var_qicored) + (var_qicored * var_qicored_dn8)));

        let assign10960_e10252: f64 = (var_csi * var_nvtm);
        let assign10960_e10254: f64 = (assign10960_e10252 * 2.0);
        let assign10960_e10256: f64 = (assign10960_e10254 * var_vtm);
        let assign10960_e10258: f64 = (assign10960_e10256 * var_t0);
        let assign10960_e10261: f64 = (var_csi * var_nvtm);
        let assign10960_e10263: f64 = (assign10960_e10261 * var_csi);
        let assign10960_e10265: f64 = (assign10960_e10263 * var_nvtm);
        let assign10960_e10267: f64 = (assign10960_e10265 * 0.5);
        let assign10960_e10269: f64 = (assign10960_e10267 * var_t1);
        let assign10960_e10271: f64 = (assign10960_e10269 / var_cox1);
        let assign10960_e10272: f64 = (assign10960_e10258 + assign10960_e10271);
        var_ids0 = assign10960_e10272;
        var_ids0_dn3 = ((((((var_csi * var_nvtm_dn3) * 2.0) * var_vtm) * var_t0) + (assign10960_e10256 * var_t0_dn3)) + ((((((((var_csi * var_nvtm_dn3) * var_csi) * var_nvtm) + (assign10960_e10263 * var_nvtm_dn3)) * 0.5) * var_t1) + (assign10960_e10267 * var_t1_dn3)) / var_cox1));
        var_ids0_dn4 = (((((((var_csi * var_nvtm_dn4) * 2.0) * var_vtm) + (assign10960_e10254 * var_vtm_dn4)) * var_t0) + (assign10960_e10256 * var_t0_dn4)) + ((((((((var_csi * var_nvtm_dn4) * var_csi) * var_nvtm) + (assign10960_e10263 * var_nvtm_dn4)) * 0.5) * var_t1) + (assign10960_e10267 * var_t1_dn4)) / var_cox1));
        var_ids0_dn5 = ((((((var_csi * var_nvtm_dn5) * 2.0) * var_vtm) * var_t0) + (assign10960_e10256 * var_t0_dn5)) + ((((((((var_csi * var_nvtm_dn5) * var_csi) * var_nvtm) + (assign10960_e10263 * var_nvtm_dn5)) * 0.5) * var_t1) + (assign10960_e10267 * var_t1_dn5)) / var_cox1));
        var_ids0_dn6 = ((((((var_csi * var_nvtm_dn6) * 2.0) * var_vtm) * var_t0) + (assign10960_e10256 * var_t0_dn6)) + ((((((((var_csi * var_nvtm_dn6) * var_csi) * var_nvtm) + (assign10960_e10263 * var_nvtm_dn6)) * 0.5) * var_t1) + (assign10960_e10267 * var_t1_dn6)) / var_cox1));
        var_ids0_dn7 = ((((((var_csi * var_nvtm_dn7) * 2.0) * var_vtm) * var_t0) + (assign10960_e10256 * var_t0_dn7)) + ((((((((var_csi * var_nvtm_dn7) * var_csi) * var_nvtm) + (assign10960_e10263 * var_nvtm_dn7)) * 0.5) * var_t1) + (assign10960_e10267 * var_t1_dn7)) / var_cox1));
        var_ids0_dn8 = ((((((var_csi * var_nvtm_dn8) * 2.0) * var_vtm) * var_t0) + (assign10960_e10256 * var_t0_dn8)) + ((((((((var_csi * var_nvtm_dn8) * var_csi) * var_nvtm) + (assign10960_e10263 * var_nvtm_dn8)) * 0.5) * var_t1) + (assign10960_e10267 * var_t1_dn8)) / var_cox1));

        let assign10970_e10276: f64 = (var_qis + var_qid);
        let assign10970_e10277: f64 = (0.5 * assign10970_e10276);
        let assign10970_e10279: f64 = (assign10970_e10277 + var_vtm);
        var_ids0_ov_dqi = assign10970_e10279;
        var_ids0_ov_dqi_dn3 = (0.5 * (var_qis_dn3 + var_qid_dn3));
        var_ids0_ov_dqi_dn4 = ((0.5 * (var_qis_dn4 + var_qid_dn4)) + var_vtm_dn4);
        var_ids0_ov_dqi_dn5 = (0.5 * (var_qis_dn5 + var_qid_dn5));
        var_ids0_ov_dqi_dn6 = (0.5 * (var_qis_dn6 + var_qid_dn6));
        var_ids0_ov_dqi_dn7 = (0.5 * (var_qis_dn7 + var_qid_dn7));
        var_ids0_ov_dqi_dn8 = (0.5 * (var_qis_dn8 + var_qid_dn8));

        let assign10980_e10282: f64 = if p.p14 == 1.0 { 1.0 } else { 0.0 };
        var_guard116 = assign10980_e10282;

        let (assign10990_e10286, assign10990_e10286_d_n3, assign10990_e10286_d_n4, assign10990_e10286_d_n5, assign10990_e10286_d_n6, assign10990_e10286_d_n7, assign10990_e10286_d_n8,) = {
    if (var_guard116 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdsi, var_rdsi_dn3, var_rdsi_dn4, var_rdsi_dn5, var_rdsi_dn6, var_rdsi_dn7, var_rdsi_dn8,)
    }
};
        var_rdsi = assign10990_e10286;
        var_rdsi_dn3 = assign10990_e10286_d_n3;
        var_rdsi_dn4 = assign10990_e10286_d_n4;
        var_rdsi_dn5 = assign10990_e10286_d_n5;
        var_rdsi_dn6 = assign10990_e10286_d_n6;
        var_rdsi_dn7 = assign10990_e10286_d_n7;
        var_rdsi_dn8 = assign10990_e10286_d_n8;

        let (assign11000_e10290, assign11000_e10290_d_n3, assign11000_e10290_d_n4, assign11000_e10290_d_n5, assign11000_e10290_d_n6, assign11000_e10290_d_n7, assign11000_e10290_d_n8,) = {
    if (var_guard116 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dr, var_dr_dn3, var_dr_dn4, var_dr_dn5, var_dr_dn6, var_dr_dn7, var_dr_dn8,)
    }
};
        var_dr = assign11000_e10290;
        var_dr_dn3 = assign11000_e10290_d_n3;
        var_dr_dn4 = assign11000_e10290_d_n4;
        var_dr_dn5 = assign11000_e10290_d_n5;
        var_dr_dn6 = assign11000_e10290_d_n6;
        var_dr_dn7 = assign11000_e10290_d_n7;
        var_dr_dn8 = assign11000_e10290_d_n8;

        let (assign11010_e10296, assign11010_e10296_d_n3, assign11010_e10296_d_n4, assign11010_e10296_d_n5, assign11010_e10296_d_n6, assign11010_e10296_d_n7, assign11010_e10296_d_n8,) = {
    if (var_guard116 != 0.0) {
        let assign11010_e10294: f64 = (var_vgs_noswap - var_vfbsd);
        (assign11010_e10294, (-var_vfbsd_dn3), (-var_vfbsd_dn4), (-var_vfbsd_dn5), (var_vgs_noswap_dn6 - var_vfbsd_dn6), (-var_vfbsd_dn7), (var_vgs_noswap_dn8 - var_vfbsd_dn8),)
    } else {
        (var_t2__blk114, var_t2__blk114_dn3, var_t2__blk114_dn4, var_t2__blk114_dn5, var_t2__blk114_dn6, var_t2__blk114_dn7, var_t2__blk114_dn8,)
    }
};
        var_t2__blk114 = assign11010_e10296;
        var_t2__blk114_dn3 = assign11010_e10296_d_n3;
        var_t2__blk114_dn4 = assign11010_e10296_d_n4;
        var_t2__blk114_dn5 = assign11010_e10296_d_n5;
        var_t2__blk114_dn6 = assign11010_e10296_d_n6;
        var_t2__blk114_dn7 = assign11010_e10296_d_n7;
        var_t2__blk114_dn8 = assign11010_e10296_d_n8;

        let (assign11020_e10305, assign11020_e10305_d_n3, assign11020_e10305_d_n4, assign11020_e10305_d_n5, assign11020_e10305_d_n6, assign11020_e10305_d_n7, assign11020_e10305_d_n8,) = {
    if (var_guard116 != 0.0) {
        let assign11020_e10300: f64 = (var_t2__blk114 * var_t2__blk114);
        let assign11020_e10302: f64 = (assign11020_e10300 + 0.0001);
        let assign11020_e10303: f64 = (assign11020_e10302).sqrt();
        (assign11020_e10303, (((var_t2__blk114_dn3 * var_t2__blk114) + (var_t2__blk114 * var_t2__blk114_dn3)) / (2.0 * assign11020_e10303)), (((var_t2__blk114_dn4 * var_t2__blk114) + (var_t2__blk114 * var_t2__blk114_dn4)) / (2.0 * assign11020_e10303)), (((var_t2__blk114_dn5 * var_t2__blk114) + (var_t2__blk114 * var_t2__blk114_dn5)) / (2.0 * assign11020_e10303)), (((var_t2__blk114_dn6 * var_t2__blk114) + (var_t2__blk114 * var_t2__blk114_dn6)) / (2.0 * assign11020_e10303)), (((var_t2__blk114_dn7 * var_t2__blk114) + (var_t2__blk114 * var_t2__blk114_dn7)) / (2.0 * assign11020_e10303)), (((var_t2__blk114_dn8 * var_t2__blk114) + (var_t2__blk114 * var_t2__blk114_dn8)) / (2.0 * assign11020_e10303)),)
    } else {
        (var_t3__blk115, var_t3__blk115_dn3, var_t3__blk115_dn4, var_t3__blk115_dn5, var_t3__blk115_dn6, var_t3__blk115_dn7, var_t3__blk115_dn8,)
    }
};
        var_t3__blk115 = assign11020_e10305;
        var_t3__blk115_dn3 = assign11020_e10305_d_n3;
        var_t3__blk115_dn4 = assign11020_e10305_d_n4;
        var_t3__blk115_dn5 = assign11020_e10305_d_n5;
        var_t3__blk115_dn6 = assign11020_e10305_d_n6;
        var_t3__blk115_dn7 = assign11020_e10305_d_n7;
        var_t3__blk115_dn8 = assign11020_e10305_d_n8;

        let (assign11030_e10313, assign11030_e10313_d_n3, assign11030_e10313_d_n4, assign11030_e10313_d_n5, assign11030_e10313_d_n6, assign11030_e10313_d_n7, assign11030_e10313_d_n8,) = {
    if (var_guard116 != 0.0) {
        let assign11030_e10310: f64 = (var_t2__blk114 + var_t3__blk115);
        let assign11030_e10311: f64 = (0.5 * assign11030_e10310);
        (assign11030_e10311, (0.5 * (var_t2__blk114_dn3 + var_t3__blk115_dn3)), (0.5 * (var_t2__blk114_dn4 + var_t3__blk115_dn4)), (0.5 * (var_t2__blk114_dn5 + var_t3__blk115_dn5)), (0.5 * (var_t2__blk114_dn6 + var_t3__blk115_dn6)), (0.5 * (var_t2__blk114_dn7 + var_t3__blk115_dn7)), (0.5 * (var_t2__blk114_dn8 + var_t3__blk115_dn8)),)
    } else {
        (var_vgs_eff, var_vgs_eff_dn3, var_vgs_eff_dn4, var_vgs_eff_dn5, var_vgs_eff_dn6, var_vgs_eff_dn7, var_vgs_eff_dn8,)
    }
};
        var_vgs_eff = assign11030_e10313;
        var_vgs_eff_dn3 = assign11030_e10313_d_n3;
        var_vgs_eff_dn4 = assign11030_e10313_d_n4;
        var_vgs_eff_dn5 = assign11030_e10313_d_n5;
        var_vgs_eff_dn6 = assign11030_e10313_d_n6;
        var_vgs_eff_dn7 = assign11030_e10313_d_n7;
        var_vgs_eff_dn8 = assign11030_e10313_d_n8;

        let (assign11040_e10321, assign11040_e10321_d_n3, assign11040_e10321_d_n4, assign11040_e10321_d_n5, assign11040_e10321_d_n6, assign11040_e10321_d_n7, assign11040_e10321_d_n8,) = {
    if (var_guard116 != 0.0) {
        let assign11040_e10318: f64 = (var_prwg_i * var_vgs_eff);
        let assign11040_e10319: f64 = (1.0 + assign11040_e10318);
        (assign11040_e10319, (var_prwg_i * var_vgs_eff_dn3), (var_prwg_i * var_vgs_eff_dn4), (var_prwg_i * var_vgs_eff_dn5), (var_prwg_i * var_vgs_eff_dn6), (var_prwg_i * var_vgs_eff_dn7), (var_prwg_i * var_vgs_eff_dn8),)
    } else {
        (var_t4__blk111, var_t4__blk111_dn3, var_t4__blk111_dn4, var_t4__blk111_dn5, var_t4__blk111_dn6, var_t4__blk111_dn7, var_t4__blk111_dn8,)
    }
};
        var_t4__blk111 = assign11040_e10321;
        var_t4__blk111_dn3 = assign11040_e10321_d_n3;
        var_t4__blk111_dn4 = assign11040_e10321_d_n4;
        var_t4__blk111_dn5 = assign11040_e10321_d_n5;
        var_t4__blk111_dn6 = assign11040_e10321_d_n6;
        var_t4__blk111_dn7 = assign11040_e10321_d_n7;
        var_t4__blk111_dn8 = assign11040_e10321_d_n8;

        let (assign11050_e10327, assign11050_e10327_d_n3, assign11050_e10327_d_n4, assign11050_e10327_d_n5, assign11050_e10327_d_n6, assign11050_e10327_d_n7, assign11050_e10327_d_n8,) = {
    if (var_guard116 != 0.0) {
        let assign11050_e10325: f64 = (1.0 / var_t4__blk111);
        (assign11050_e10325, (-(var_t4__blk111_dn3 / (var_t4__blk111 * var_t4__blk111))), (-(var_t4__blk111_dn4 / (var_t4__blk111 * var_t4__blk111))), (-(var_t4__blk111_dn5 / (var_t4__blk111 * var_t4__blk111))), (-(var_t4__blk111_dn6 / (var_t4__blk111 * var_t4__blk111))), (-(var_t4__blk111_dn7 / (var_t4__blk111 * var_t4__blk111))), (-(var_t4__blk111_dn8 / (var_t4__blk111 * var_t4__blk111))),)
    } else {
        (var_t1__blk110, var_t1__blk110_dn3, var_t1__blk110_dn4, var_t1__blk110_dn5, var_t1__blk110_dn6, var_t1__blk110_dn7, var_t1__blk110_dn8,)
    }
};
        var_t1__blk110 = assign11050_e10327;
        var_t1__blk110_dn3 = assign11050_e10327_d_n3;
        var_t1__blk110_dn4 = assign11050_e10327_d_n4;
        var_t1__blk110_dn5 = assign11050_e10327_d_n5;
        var_t1__blk110_dn6 = assign11050_e10327_d_n6;
        var_t1__blk110_dn7 = assign11050_e10327_d_n7;
        var_t1__blk110_dn8 = assign11050_e10327_d_n8;

        let (assign11060_e10337, assign11060_e10337_d_n3, assign11060_e10337_d_n4, assign11060_e10337_d_n5, assign11060_e10337_d_n6, assign11060_e10337_d_n7, assign11060_e10337_d_n8,) = {
    if (var_guard116 != 0.0) {
        let assign11060_e10332: f64 = (0.5 * var_vbgs_noswap);
        let assign11060_e10334: f64 = (assign11060_e10332 * var_prwb_i);
        let assign11060_e10335: f64 = (var_t1__blk110 - assign11060_e10334);
        (assign11060_e10335, (var_t1__blk110_dn3 - ((0.5 * var_vbgs_noswap_dn3) * var_prwb_i)), var_t1__blk110_dn4, var_t1__blk110_dn5, (var_t1__blk110_dn6 - ((0.5 * var_vbgs_noswap_dn6) * var_prwb_i)), var_t1__blk110_dn7, var_t1__blk110_dn8,)
    } else {
        (var_t1__blk110, var_t1__blk110_dn3, var_t1__blk110_dn4, var_t1__blk110_dn5, var_t1__blk110_dn6, var_t1__blk110_dn7, var_t1__blk110_dn8,)
    }
};
        var_t1__blk110 = assign11060_e10337;
        var_t1__blk110_dn3 = assign11060_e10337_d_n3;
        var_t1__blk110_dn4 = assign11060_e10337_d_n4;
        var_t1__blk110_dn5 = assign11060_e10337_d_n5;
        var_t1__blk110_dn6 = assign11060_e10337_d_n6;
        var_t1__blk110_dn7 = assign11060_e10337_d_n7;
        var_t1__blk110_dn8 = assign11060_e10337_d_n8;

        let (assign11070_e10350, assign11070_e10350_d_n3, assign11070_e10350_d_n4, assign11070_e10350_d_n5, assign11070_e10350_d_n6, assign11070_e10350_d_n7, assign11070_e10350_d_n8,) = {
    if (var_guard116 != 0.0) {
        let assign11070_e10343: f64 = (var_t1__blk110 * var_t1__blk110);
        let assign11070_e10345: f64 = (assign11070_e10343 + 0.01);
        let assign11070_e10346: f64 = (assign11070_e10345).sqrt();
        let assign11070_e10347: f64 = (var_t1__blk110 + assign11070_e10346);
        let assign11070_e10348: f64 = (0.5 * assign11070_e10347);
        (assign11070_e10348, (0.5 * (var_t1__blk110_dn3 + (((var_t1__blk110_dn3 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn3)) / (2.0 * assign11070_e10346)))), (0.5 * (var_t1__blk110_dn4 + (((var_t1__blk110_dn4 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn4)) / (2.0 * assign11070_e10346)))), (0.5 * (var_t1__blk110_dn5 + (((var_t1__blk110_dn5 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn5)) / (2.0 * assign11070_e10346)))), (0.5 * (var_t1__blk110_dn6 + (((var_t1__blk110_dn6 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn6)) / (2.0 * assign11070_e10346)))), (0.5 * (var_t1__blk110_dn7 + (((var_t1__blk110_dn7 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn7)) / (2.0 * assign11070_e10346)))), (0.5 * (var_t1__blk110_dn8 + (((var_t1__blk110_dn8 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn8)) / (2.0 * assign11070_e10346)))),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign11070_e10350;
        var_t0_dn3 = assign11070_e10350_d_n3;
        var_t0_dn4 = assign11070_e10350_d_n4;
        var_t0_dn5 = assign11070_e10350_d_n5;
        var_t0_dn6 = assign11070_e10350_d_n6;
        var_t0_dn7 = assign11070_e10350_d_n7;
        var_t0_dn8 = assign11070_e10350_d_n8;

        let (assign11080_e10364, assign11080_e10364_d_n3, assign11080_e10364_d_n4, assign11080_e10364_d_n5, assign11080_e10364_d_n6, assign11080_e10364_d_n7, assign11080_e10364_d_n8,) = {
    if (var_guard116 != 0.0) {
        let assign11080_e10357: f64 = (var_rsw_i * var_t0);
        let assign11080_e10358: f64 = (var_rswmin_i + assign11080_e10357);
        let assign11080_e10360: f64 = (assign11080_e10358 * var_weffwrfactor);
        let assign11080_e10361: f64 = (var_rsourcegeo + assign11080_e10360);
        let assign11080_e10362: f64 = (var_rdstemp * assign11080_e10361);
        (assign11080_e10362, (var_rdstemp * ((var_rsw_i * var_t0_dn3) * var_weffwrfactor)), ((var_rdstemp_dn4 * assign11080_e10361) + (var_rdstemp * ((var_rsw_i * var_t0_dn4) * var_weffwrfactor))), (var_rdstemp * ((var_rsw_i * var_t0_dn5) * var_weffwrfactor)), (var_rdstemp * ((var_rsw_i * var_t0_dn6) * var_weffwrfactor)), (var_rdstemp * ((var_rsw_i * var_t0_dn7) * var_weffwrfactor)), (var_rdstemp * ((var_rsw_i * var_t0_dn8) * var_weffwrfactor)),)
    } else {
        (var_rsource, var_rsource_dn3, var_rsource_dn4, var_rsource_dn5, var_rsource_dn6, var_rsource_dn7, var_rsource_dn8,)
    }
};
        var_rsource = assign11080_e10364;
        var_rsource_dn3 = assign11080_e10364_d_n3;
        var_rsource_dn4 = assign11080_e10364_d_n4;
        var_rsource_dn5 = assign11080_e10364_d_n5;
        var_rsource_dn6 = assign11080_e10364_d_n6;
        var_rsource_dn7 = assign11080_e10364_d_n7;
        var_rsource_dn8 = assign11080_e10364_d_n8;

        let (assign11090_e10370, assign11090_e10370_d_n3, assign11090_e10370_d_n4, assign11090_e10370_d_n5, assign11090_e10370_d_n6, assign11090_e10370_d_n7, assign11090_e10370_d_n8,) = {
    if (var_guard116 != 0.0) {
        let assign11090_e10368: f64 = (var_vgd_noswap - var_vfbsd);
        (assign11090_e10368, (-var_vfbsd_dn3), (-var_vfbsd_dn4), (var_vgd_noswap_dn5 - var_vfbsd_dn5), (-var_vfbsd_dn6), (-var_vfbsd_dn7), (var_vgd_noswap_dn8 - var_vfbsd_dn8),)
    } else {
        (var_t2__blk114, var_t2__blk114_dn3, var_t2__blk114_dn4, var_t2__blk114_dn5, var_t2__blk114_dn6, var_t2__blk114_dn7, var_t2__blk114_dn8,)
    }
};
        var_t2__blk114 = assign11090_e10370;
        var_t2__blk114_dn3 = assign11090_e10370_d_n3;
        var_t2__blk114_dn4 = assign11090_e10370_d_n4;
        var_t2__blk114_dn5 = assign11090_e10370_d_n5;
        var_t2__blk114_dn6 = assign11090_e10370_d_n6;
        var_t2__blk114_dn7 = assign11090_e10370_d_n7;
        var_t2__blk114_dn8 = assign11090_e10370_d_n8;

        let (assign11100_e10379, assign11100_e10379_d_n3, assign11100_e10379_d_n4, assign11100_e10379_d_n5, assign11100_e10379_d_n6, assign11100_e10379_d_n7, assign11100_e10379_d_n8,) = {
    if (var_guard116 != 0.0) {
        let assign11100_e10374: f64 = (var_t2__blk114 * var_t2__blk114);
        let assign11100_e10376: f64 = (assign11100_e10374 + 0.0001);
        let assign11100_e10377: f64 = (assign11100_e10376).sqrt();
        (assign11100_e10377, (((var_t2__blk114_dn3 * var_t2__blk114) + (var_t2__blk114 * var_t2__blk114_dn3)) / (2.0 * assign11100_e10377)), (((var_t2__blk114_dn4 * var_t2__blk114) + (var_t2__blk114 * var_t2__blk114_dn4)) / (2.0 * assign11100_e10377)), (((var_t2__blk114_dn5 * var_t2__blk114) + (var_t2__blk114 * var_t2__blk114_dn5)) / (2.0 * assign11100_e10377)), (((var_t2__blk114_dn6 * var_t2__blk114) + (var_t2__blk114 * var_t2__blk114_dn6)) / (2.0 * assign11100_e10377)), (((var_t2__blk114_dn7 * var_t2__blk114) + (var_t2__blk114 * var_t2__blk114_dn7)) / (2.0 * assign11100_e10377)), (((var_t2__blk114_dn8 * var_t2__blk114) + (var_t2__blk114 * var_t2__blk114_dn8)) / (2.0 * assign11100_e10377)),)
    } else {
        (var_t3__blk115, var_t3__blk115_dn3, var_t3__blk115_dn4, var_t3__blk115_dn5, var_t3__blk115_dn6, var_t3__blk115_dn7, var_t3__blk115_dn8,)
    }
};
        var_t3__blk115 = assign11100_e10379;
        var_t3__blk115_dn3 = assign11100_e10379_d_n3;
        var_t3__blk115_dn4 = assign11100_e10379_d_n4;
        var_t3__blk115_dn5 = assign11100_e10379_d_n5;
        var_t3__blk115_dn6 = assign11100_e10379_d_n6;
        var_t3__blk115_dn7 = assign11100_e10379_d_n7;
        var_t3__blk115_dn8 = assign11100_e10379_d_n8;

        let (assign11110_e10387, assign11110_e10387_d_n3, assign11110_e10387_d_n4, assign11110_e10387_d_n5, assign11110_e10387_d_n6, assign11110_e10387_d_n7, assign11110_e10387_d_n8,) = {
    if (var_guard116 != 0.0) {
        let assign11110_e10384: f64 = (var_t2__blk114 + var_t3__blk115);
        let assign11110_e10385: f64 = (0.5 * assign11110_e10384);
        (assign11110_e10385, (0.5 * (var_t2__blk114_dn3 + var_t3__blk115_dn3)), (0.5 * (var_t2__blk114_dn4 + var_t3__blk115_dn4)), (0.5 * (var_t2__blk114_dn5 + var_t3__blk115_dn5)), (0.5 * (var_t2__blk114_dn6 + var_t3__blk115_dn6)), (0.5 * (var_t2__blk114_dn7 + var_t3__blk115_dn7)), (0.5 * (var_t2__blk114_dn8 + var_t3__blk115_dn8)),)
    } else {
        (var_vgd_eff, var_vgd_eff_dn3, var_vgd_eff_dn4, var_vgd_eff_dn5, var_vgd_eff_dn6, var_vgd_eff_dn7, var_vgd_eff_dn8,)
    }
};
        var_vgd_eff = assign11110_e10387;
        var_vgd_eff_dn3 = assign11110_e10387_d_n3;
        var_vgd_eff_dn4 = assign11110_e10387_d_n4;
        var_vgd_eff_dn5 = assign11110_e10387_d_n5;
        var_vgd_eff_dn6 = assign11110_e10387_d_n6;
        var_vgd_eff_dn7 = assign11110_e10387_d_n7;
        var_vgd_eff_dn8 = assign11110_e10387_d_n8;

        let (assign11120_e10395, assign11120_e10395_d_n3, assign11120_e10395_d_n4, assign11120_e10395_d_n5, assign11120_e10395_d_n6, assign11120_e10395_d_n7, assign11120_e10395_d_n8,) = {
    if (var_guard116 != 0.0) {
        let assign11120_e10392: f64 = (var_prwg_i * var_vgd_eff);
        let assign11120_e10393: f64 = (1.0 + assign11120_e10392);
        (assign11120_e10393, (var_prwg_i * var_vgd_eff_dn3), (var_prwg_i * var_vgd_eff_dn4), (var_prwg_i * var_vgd_eff_dn5), (var_prwg_i * var_vgd_eff_dn6), (var_prwg_i * var_vgd_eff_dn7), (var_prwg_i * var_vgd_eff_dn8),)
    } else {
        (var_t4__blk111, var_t4__blk111_dn3, var_t4__blk111_dn4, var_t4__blk111_dn5, var_t4__blk111_dn6, var_t4__blk111_dn7, var_t4__blk111_dn8,)
    }
};
        var_t4__blk111 = assign11120_e10395;
        var_t4__blk111_dn3 = assign11120_e10395_d_n3;
        var_t4__blk111_dn4 = assign11120_e10395_d_n4;
        var_t4__blk111_dn5 = assign11120_e10395_d_n5;
        var_t4__blk111_dn6 = assign11120_e10395_d_n6;
        var_t4__blk111_dn7 = assign11120_e10395_d_n7;
        var_t4__blk111_dn8 = assign11120_e10395_d_n8;

        *var_dr_slot = var_dr;
        *var_dr_dn3_slot = var_dr_dn3;
        *var_dr_dn4_slot = var_dr_dn4;
        *var_dr_dn5_slot = var_dr_dn5;
        *var_dr_dn6_slot = var_dr_dn6;
        *var_dr_dn7_slot = var_dr_dn7;
        *var_dr_dn8_slot = var_dr_dn8;
        *var_guard106_slot = var_guard106;
        *var_guard107_slot = var_guard107;
        *var_guard108_slot = var_guard108;
        *var_guard109_slot = var_guard109;
        *var_guard116_slot = var_guard116;
        *var_ids0_slot = var_ids0;
        *var_ids0_dn3_slot = var_ids0_dn3;
        *var_ids0_dn4_slot = var_ids0_dn4;
        *var_ids0_dn5_slot = var_ids0_dn5;
        *var_ids0_dn6_slot = var_ids0_dn6;
        *var_ids0_dn7_slot = var_ids0_dn7;
        *var_ids0_dn8_slot = var_ids0_dn8;
        *var_ids0_ov_dqi_slot = var_ids0_ov_dqi;
        *var_ids0_ov_dqi_dn3_slot = var_ids0_ov_dqi_dn3;
        *var_ids0_ov_dqi_dn4_slot = var_ids0_ov_dqi_dn4;
        *var_ids0_ov_dqi_dn5_slot = var_ids0_ov_dqi_dn5;
        *var_ids0_ov_dqi_dn6_slot = var_ids0_ov_dqi_dn6;
        *var_ids0_ov_dqi_dn7_slot = var_ids0_ov_dqi_dn7;
        *var_ids0_ov_dqi_dn8_slot = var_ids0_ov_dqi_dn8;
        *var_mclm_slot = var_mclm;
        *var_mclm_dn3_slot = var_mclm_dn3;
        *var_mclm_dn4_slot = var_mclm_dn4;
        *var_mclm_dn5_slot = var_mclm_dn5;
        *var_mclm_dn6_slot = var_mclm_dn6;
        *var_mclm_dn7_slot = var_mclm_dn7;
        *var_mclm_dn8_slot = var_mclm_dn8;
        *var_mclmcv_slot = var_mclmcv;
        *var_mclmcv_dn3_slot = var_mclmcv_dn3;
        *var_mclmcv_dn4_slot = var_mclmcv_dn4;
        *var_mclmcv_dn5_slot = var_mclmcv_dn5;
        *var_mclmcv_dn6_slot = var_mclmcv_dn6;
        *var_mclmcv_dn7_slot = var_mclmcv_dn7;
        *var_mclmcv_dn8_slot = var_mclmcv_dn8;
        *var_mnud_slot = var_mnud;
        *var_mnud_dn3_slot = var_mnud_dn3;
        *var_mnud_dn4_slot = var_mnud_dn4;
        *var_mnud_dn5_slot = var_mnud_dn5;
        *var_mnud_dn6_slot = var_mnud_dn6;
        *var_mnud_dn7_slot = var_mnud_dn7;
        *var_mnud_dn8_slot = var_mnud_dn8;
        *var_moc_slot = var_moc;
        *var_moc_dn3_slot = var_moc_dn3;
        *var_moc_dn4_slot = var_moc_dn4;
        *var_moc_dn5_slot = var_moc_dn5;
        *var_moc_dn6_slot = var_moc_dn6;
        *var_moc_dn7_slot = var_moc_dn7;
        *var_moc_dn8_slot = var_moc_dn8;
        *var_rdsi_slot = var_rdsi;
        *var_rdsi_dn3_slot = var_rdsi_dn3;
        *var_rdsi_dn4_slot = var_rdsi_dn4;
        *var_rdsi_dn5_slot = var_rdsi_dn5;
        *var_rdsi_dn6_slot = var_rdsi_dn6;
        *var_rdsi_dn7_slot = var_rdsi_dn7;
        *var_rdsi_dn8_slot = var_rdsi_dn8;
        *var_rsource_slot = var_rsource;
        *var_rsource_dn3_slot = var_rsource_dn3;
        *var_rsource_dn4_slot = var_rsource_dn4;
        *var_rsource_dn5_slot = var_rsource_dn5;
        *var_rsource_dn6_slot = var_rsource_dn6;
        *var_rsource_dn7_slot = var_rsource_dn7;
        *var_rsource_dn8_slot = var_rsource_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1__blk110_slot = var_t1__blk110;
        *var_t1__blk110_dn3_slot = var_t1__blk110_dn3;
        *var_t1__blk110_dn4_slot = var_t1__blk110_dn4;
        *var_t1__blk110_dn5_slot = var_t1__blk110_dn5;
        *var_t1__blk110_dn6_slot = var_t1__blk110_dn6;
        *var_t1__blk110_dn7_slot = var_t1__blk110_dn7;
        *var_t1__blk110_dn8_slot = var_t1__blk110_dn8;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2__blk114_slot = var_t2__blk114;
        *var_t2__blk114_dn3_slot = var_t2__blk114_dn3;
        *var_t2__blk114_dn4_slot = var_t2__blk114_dn4;
        *var_t2__blk114_dn5_slot = var_t2__blk114_dn5;
        *var_t2__blk114_dn6_slot = var_t2__blk114_dn6;
        *var_t2__blk114_dn7_slot = var_t2__blk114_dn7;
        *var_t2__blk114_dn8_slot = var_t2__blk114_dn8;
        *var_t3__blk115_slot = var_t3__blk115;
        *var_t3__blk115_dn3_slot = var_t3__blk115_dn3;
        *var_t3__blk115_dn4_slot = var_t3__blk115_dn4;
        *var_t3__blk115_dn5_slot = var_t3__blk115_dn5;
        *var_t3__blk115_dn6_slot = var_t3__blk115_dn6;
        *var_t3__blk115_dn7_slot = var_t3__blk115_dn7;
        *var_t3__blk115_dn8_slot = var_t3__blk115_dn8;
        *var_t4__blk111_slot = var_t4__blk111;
        *var_t4__blk111_dn3_slot = var_t4__blk111_dn3;
        *var_t4__blk111_dn4_slot = var_t4__blk111_dn4;
        *var_t4__blk111_dn5_slot = var_t4__blk111_dn5;
        *var_t4__blk111_dn6_slot = var_t4__blk111_dn6;
        *var_t4__blk111_dn7_slot = var_t4__blk111_dn7;
        *var_t4__blk111_dn8_slot = var_t4__blk111_dn8;
        *var_vgd_eff_slot = var_vgd_eff;
        *var_vgd_eff_dn3_slot = var_vgd_eff_dn3;
        *var_vgd_eff_dn4_slot = var_vgd_eff_dn4;
        *var_vgd_eff_dn5_slot = var_vgd_eff_dn5;
        *var_vgd_eff_dn6_slot = var_vgd_eff_dn6;
        *var_vgd_eff_dn7_slot = var_vgd_eff_dn7;
        *var_vgd_eff_dn8_slot = var_vgd_eff_dn8;
        *var_vgs_eff_slot = var_vgs_eff;
        *var_vgs_eff_dn3_slot = var_vgs_eff_dn3;
        *var_vgs_eff_dn4_slot = var_vgs_eff_dn4;
        *var_vgs_eff_dn5_slot = var_vgs_eff_dn5;
        *var_vgs_eff_dn6_slot = var_vgs_eff_dn6;
        *var_vgs_eff_dn7_slot = var_vgs_eff_dn7;
        *var_vgs_eff_dn8_slot = var_vgs_eff_dn8;
    }

    pub(super) fn stamp_transient_block_24(
        p: &Parameters,
        var_beta: f64,
        var_beta_dn3: f64,
        var_beta_dn4: f64,
        var_beta_dn5: f64,
        var_beta_dn6: f64,
        var_beta_dn7: f64,
        var_beta_dn8: f64,
        var_cox1: f64,
        var_dvsat: f64,
        var_dvsat_dn3: f64,
        var_dvsat_dn4: f64,
        var_dvsat_dn5: f64,
        var_dvsat_dn6: f64,
        var_dvsat_dn7: f64,
        var_dvsat_dn8: f64,
        var_etaqm_i: f64,
        var_guard116: f64,
        var_ids0: f64,
        var_ids0_dn3: f64,
        var_ids0_dn4: f64,
        var_ids0_dn5: f64,
        var_ids0_dn6: f64,
        var_ids0_dn7: f64,
        var_ids0_dn8: f64,
        var_ids0_ov_dqi: f64,
        var_ids0_ov_dqi_dn3: f64,
        var_ids0_ov_dqi_dn4: f64,
        var_ids0_ov_dqi_dn5: f64,
        var_ids0_ov_dqi_dn6: f64,
        var_ids0_ov_dqi_dn7: f64,
        var_ids0_ov_dqi_dn8: f64,
        var_mnud: f64,
        var_mnud_dn3: f64,
        var_mnud_dn4: f64,
        var_mnud_dn5: f64,
        var_mnud_dn6: f64,
        var_mnud_dn7: f64,
        var_mnud_dn8: f64,
        var_moc: f64,
        var_moc_dn3: f64,
        var_moc_dn4: f64,
        var_moc_dn5: f64,
        var_moc_dn6: f64,
        var_moc_dn7: f64,
        var_moc_dn8: f64,
        var_pqm_i: f64,
        var_prwb_i: f64,
        var_prwg_i: f64,
        var_qba: f64,
        var_qbackd: f64,
        var_qbackd_dn3: f64,
        var_qbackd_dn4: f64,
        var_qbackd_dn5: f64,
        var_qbackd_dn6: f64,
        var_qbackd_dn7: f64,
        var_qbackd_dn8: f64,
        var_qbacks: f64,
        var_qbacks_dn3: f64,
        var_qbacks_dn4: f64,
        var_qbacks_dn5: f64,
        var_qbacks_dn6: f64,
        var_qbacks_dn7: f64,
        var_qbacks_dn8: f64,
        var_qfrontd: f64,
        var_qfrontd_dn3: f64,
        var_qfrontd_dn4: f64,
        var_qfrontd_dn5: f64,
        var_qfrontd_dn6: f64,
        var_qfrontd_dn7: f64,
        var_qfrontd_dn8: f64,
        var_qfronts: f64,
        var_qfronts_dn3: f64,
        var_qfronts_dn4: f64,
        var_qfronts_dn5: f64,
        var_qfronts_dn6: f64,
        var_qfronts_dn7: f64,
        var_qfronts_dn8: f64,
        var_qia: f64,
        var_qia_dn3: f64,
        var_qia_dn4: f64,
        var_qia_dn5: f64,
        var_qia_dn6: f64,
        var_qia_dn7: f64,
        var_qia_dn8: f64,
        var_qm0_i: f64,
        var_qmtcencv_i: f64,
        var_qtotd: f64,
        var_qtotd_dn3: f64,
        var_qtotd_dn4: f64,
        var_qtotd_dn5: f64,
        var_qtotd_dn6: f64,
        var_qtotd_dn7: f64,
        var_qtotd_dn8: f64,
        var_qtots: f64,
        var_qtots_dn3: f64,
        var_qtots_dn4: f64,
        var_qtots_dn5: f64,
        var_qtots_dn6: f64,
        var_qtots_dn7: f64,
        var_qtots_dn8: f64,
        var_rdraingeo: f64,
        var_rdstemp: f64,
        var_rdstemp_dn4: f64,
        var_rdsw_i: f64,
        var_rdswmin_i: f64,
        var_rdw_i: f64,
        var_rdwmin_i: f64,
        var_rsourcegeo: f64,
        var_vbgd: f64,
        var_vbgd_dn3: f64,
        var_vbgd_dn5: f64,
        var_vbgd_dn6: f64,
        var_vbgd_noswap: f64,
        var_vbgd_noswap_dn3: f64,
        var_vbgd_noswap_dn5: f64,
        var_vbgs: f64,
        var_vbgs_dn3: f64,
        var_vbgs_dn5: f64,
        var_vbgs_dn6: f64,
        var_weffwrfactor: f64,
        var_dr_slot: &mut f64,
        var_dr_dn3_slot: &mut f64,
        var_dr_dn4_slot: &mut f64,
        var_dr_dn5_slot: &mut f64,
        var_dr_dn6_slot: &mut f64,
        var_dr_dn7_slot: &mut f64,
        var_dr_dn8_slot: &mut f64,
        var_guard117_slot: &mut f64,
        var_guard118_slot: &mut f64,
        var_ids_slot: &mut f64,
        var_ids_dn3_slot: &mut f64,
        var_ids_dn4_slot: &mut f64,
        var_ids_dn5_slot: &mut f64,
        var_ids_dn6_slot: &mut f64,
        var_ids_dn7_slot: &mut f64,
        var_ids_dn8_slot: &mut f64,
        var_qbg_slot: &mut f64,
        var_qbg_dn3_slot: &mut f64,
        var_qbg_dn4_slot: &mut f64,
        var_qbg_dn5_slot: &mut f64,
        var_qbg_dn6_slot: &mut f64,
        var_qbg_dn7_slot: &mut f64,
        var_qbg_dn8_slot: &mut f64,
        var_qd_slot: &mut f64,
        var_qd_dn3_slot: &mut f64,
        var_qd_dn4_slot: &mut f64,
        var_qd_dn5_slot: &mut f64,
        var_qd_dn6_slot: &mut f64,
        var_qd_dn7_slot: &mut f64,
        var_qd_dn8_slot: &mut f64,
        var_qfg_slot: &mut f64,
        var_qfg_dn3_slot: &mut f64,
        var_qfg_dn4_slot: &mut f64,
        var_qfg_dn5_slot: &mut f64,
        var_qfg_dn6_slot: &mut f64,
        var_qfg_dn7_slot: &mut f64,
        var_qfg_dn8_slot: &mut f64,
        var_qs_slot: &mut f64,
        var_qs_dn3_slot: &mut f64,
        var_qs_dn4_slot: &mut f64,
        var_qs_dn5_slot: &mut f64,
        var_qs_dn6_slot: &mut f64,
        var_qs_dn7_slot: &mut f64,
        var_qs_dn8_slot: &mut f64,
        var_rdrain_slot: &mut f64,
        var_rdrain_dn3_slot: &mut f64,
        var_rdrain_dn4_slot: &mut f64,
        var_rdrain_dn5_slot: &mut f64,
        var_rdrain_dn6_slot: &mut f64,
        var_rdrain_dn7_slot: &mut f64,
        var_rdrain_dn8_slot: &mut f64,
        var_rdsi_slot: &mut f64,
        var_rdsi_dn3_slot: &mut f64,
        var_rdsi_dn4_slot: &mut f64,
        var_rdsi_dn5_slot: &mut f64,
        var_rdsi_dn6_slot: &mut f64,
        var_rdsi_dn7_slot: &mut f64,
        var_rdsi_dn8_slot: &mut f64,
        var_rsource_slot: &mut f64,
        var_rsource_dn3_slot: &mut f64,
        var_rsource_dn4_slot: &mut f64,
        var_rsource_dn5_slot: &mut f64,
        var_rsource_dn6_slot: &mut f64,
        var_rsource_dn7_slot: &mut f64,
        var_rsource_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1__blk110_slot: &mut f64,
        var_t1__blk110_dn3_slot: &mut f64,
        var_t1__blk110_dn4_slot: &mut f64,
        var_t1__blk110_dn5_slot: &mut f64,
        var_t1__blk110_dn6_slot: &mut f64,
        var_t1__blk110_dn7_slot: &mut f64,
        var_t1__blk110_dn8_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4__blk111_slot: &mut f64,
        var_t4__blk111_dn3_slot: &mut f64,
        var_t4__blk111_dn4_slot: &mut f64,
        var_t4__blk111_dn5_slot: &mut f64,
        var_t4__blk111_dn6_slot: &mut f64,
        var_t4__blk111_dn7_slot: &mut f64,
        var_t4__blk111_dn8_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_tcen_slot: &mut f64,
        var_tcen0_slot: &mut f64,
        var_tcen_dn3_slot: &mut f64,
        var_tcen_dn4_slot: &mut f64,
        var_tcen_dn5_slot: &mut f64,
        var_tcen_dn6_slot: &mut f64,
        var_tcen_dn7_slot: &mut f64,
        var_tcen_dn8_slot: &mut f64,
    ) {
        let mut var_dr: f64 = *var_dr_slot;
        let mut var_dr_dn3: f64 = *var_dr_dn3_slot;
        let mut var_dr_dn4: f64 = *var_dr_dn4_slot;
        let mut var_dr_dn5: f64 = *var_dr_dn5_slot;
        let mut var_dr_dn6: f64 = *var_dr_dn6_slot;
        let mut var_dr_dn7: f64 = *var_dr_dn7_slot;
        let mut var_dr_dn8: f64 = *var_dr_dn8_slot;
        let mut var_guard117: f64 = *var_guard117_slot;
        let mut var_guard118: f64 = *var_guard118_slot;
        let mut var_ids: f64 = *var_ids_slot;
        let mut var_ids_dn3: f64 = *var_ids_dn3_slot;
        let mut var_ids_dn4: f64 = *var_ids_dn4_slot;
        let mut var_ids_dn5: f64 = *var_ids_dn5_slot;
        let mut var_ids_dn6: f64 = *var_ids_dn6_slot;
        let mut var_ids_dn7: f64 = *var_ids_dn7_slot;
        let mut var_ids_dn8: f64 = *var_ids_dn8_slot;
        let mut var_qbg: f64 = *var_qbg_slot;
        let mut var_qbg_dn3: f64 = *var_qbg_dn3_slot;
        let mut var_qbg_dn4: f64 = *var_qbg_dn4_slot;
        let mut var_qbg_dn5: f64 = *var_qbg_dn5_slot;
        let mut var_qbg_dn6: f64 = *var_qbg_dn6_slot;
        let mut var_qbg_dn7: f64 = *var_qbg_dn7_slot;
        let mut var_qbg_dn8: f64 = *var_qbg_dn8_slot;
        let mut var_qd: f64 = *var_qd_slot;
        let mut var_qd_dn3: f64 = *var_qd_dn3_slot;
        let mut var_qd_dn4: f64 = *var_qd_dn4_slot;
        let mut var_qd_dn5: f64 = *var_qd_dn5_slot;
        let mut var_qd_dn6: f64 = *var_qd_dn6_slot;
        let mut var_qd_dn7: f64 = *var_qd_dn7_slot;
        let mut var_qd_dn8: f64 = *var_qd_dn8_slot;
        let mut var_qfg: f64 = *var_qfg_slot;
        let mut var_qfg_dn3: f64 = *var_qfg_dn3_slot;
        let mut var_qfg_dn4: f64 = *var_qfg_dn4_slot;
        let mut var_qfg_dn5: f64 = *var_qfg_dn5_slot;
        let mut var_qfg_dn6: f64 = *var_qfg_dn6_slot;
        let mut var_qfg_dn7: f64 = *var_qfg_dn7_slot;
        let mut var_qfg_dn8: f64 = *var_qfg_dn8_slot;
        let mut var_qs: f64 = *var_qs_slot;
        let mut var_qs_dn3: f64 = *var_qs_dn3_slot;
        let mut var_qs_dn4: f64 = *var_qs_dn4_slot;
        let mut var_qs_dn5: f64 = *var_qs_dn5_slot;
        let mut var_qs_dn6: f64 = *var_qs_dn6_slot;
        let mut var_qs_dn7: f64 = *var_qs_dn7_slot;
        let mut var_qs_dn8: f64 = *var_qs_dn8_slot;
        let mut var_rdrain: f64 = *var_rdrain_slot;
        let mut var_rdrain_dn3: f64 = *var_rdrain_dn3_slot;
        let mut var_rdrain_dn4: f64 = *var_rdrain_dn4_slot;
        let mut var_rdrain_dn5: f64 = *var_rdrain_dn5_slot;
        let mut var_rdrain_dn6: f64 = *var_rdrain_dn6_slot;
        let mut var_rdrain_dn7: f64 = *var_rdrain_dn7_slot;
        let mut var_rdrain_dn8: f64 = *var_rdrain_dn8_slot;
        let mut var_rdsi: f64 = *var_rdsi_slot;
        let mut var_rdsi_dn3: f64 = *var_rdsi_dn3_slot;
        let mut var_rdsi_dn4: f64 = *var_rdsi_dn4_slot;
        let mut var_rdsi_dn5: f64 = *var_rdsi_dn5_slot;
        let mut var_rdsi_dn6: f64 = *var_rdsi_dn6_slot;
        let mut var_rdsi_dn7: f64 = *var_rdsi_dn7_slot;
        let mut var_rdsi_dn8: f64 = *var_rdsi_dn8_slot;
        let mut var_rsource: f64 = *var_rsource_slot;
        let mut var_rsource_dn3: f64 = *var_rsource_dn3_slot;
        let mut var_rsource_dn4: f64 = *var_rsource_dn4_slot;
        let mut var_rsource_dn5: f64 = *var_rsource_dn5_slot;
        let mut var_rsource_dn6: f64 = *var_rsource_dn6_slot;
        let mut var_rsource_dn7: f64 = *var_rsource_dn7_slot;
        let mut var_rsource_dn8: f64 = *var_rsource_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1__blk110: f64 = *var_t1__blk110_slot;
        let mut var_t1__blk110_dn3: f64 = *var_t1__blk110_dn3_slot;
        let mut var_t1__blk110_dn4: f64 = *var_t1__blk110_dn4_slot;
        let mut var_t1__blk110_dn5: f64 = *var_t1__blk110_dn5_slot;
        let mut var_t1__blk110_dn6: f64 = *var_t1__blk110_dn6_slot;
        let mut var_t1__blk110_dn7: f64 = *var_t1__blk110_dn7_slot;
        let mut var_t1__blk110_dn8: f64 = *var_t1__blk110_dn8_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4__blk111: f64 = *var_t4__blk111_slot;
        let mut var_t4__blk111_dn3: f64 = *var_t4__blk111_dn3_slot;
        let mut var_t4__blk111_dn4: f64 = *var_t4__blk111_dn4_slot;
        let mut var_t4__blk111_dn5: f64 = *var_t4__blk111_dn5_slot;
        let mut var_t4__blk111_dn6: f64 = *var_t4__blk111_dn6_slot;
        let mut var_t4__blk111_dn7: f64 = *var_t4__blk111_dn7_slot;
        let mut var_t4__blk111_dn8: f64 = *var_t4__blk111_dn8_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_tcen: f64 = *var_tcen_slot;
        let mut var_tcen0: f64 = *var_tcen0_slot;
        let mut var_tcen_dn3: f64 = *var_tcen_dn3_slot;
        let mut var_tcen_dn4: f64 = *var_tcen_dn4_slot;
        let mut var_tcen_dn5: f64 = *var_tcen_dn5_slot;
        let mut var_tcen_dn6: f64 = *var_tcen_dn6_slot;
        let mut var_tcen_dn7: f64 = *var_tcen_dn7_slot;
        let mut var_tcen_dn8: f64 = *var_tcen_dn8_slot;

        let (assign11130_e10401, assign11130_e10401_d_n3, assign11130_e10401_d_n4, assign11130_e10401_d_n5, assign11130_e10401_d_n6, assign11130_e10401_d_n7, assign11130_e10401_d_n8,) = {
    if (var_guard116 != 0.0) {
        let assign11130_e10399: f64 = (1.0 / var_t4__blk111);
        (assign11130_e10399, (-(var_t4__blk111_dn3 / (var_t4__blk111 * var_t4__blk111))), (-(var_t4__blk111_dn4 / (var_t4__blk111 * var_t4__blk111))), (-(var_t4__blk111_dn5 / (var_t4__blk111 * var_t4__blk111))), (-(var_t4__blk111_dn6 / (var_t4__blk111 * var_t4__blk111))), (-(var_t4__blk111_dn7 / (var_t4__blk111 * var_t4__blk111))), (-(var_t4__blk111_dn8 / (var_t4__blk111 * var_t4__blk111))),)
    } else {
        (var_t1__blk110, var_t1__blk110_dn3, var_t1__blk110_dn4, var_t1__blk110_dn5, var_t1__blk110_dn6, var_t1__blk110_dn7, var_t1__blk110_dn8,)
    }
};
        var_t1__blk110 = assign11130_e10401;
        var_t1__blk110_dn3 = assign11130_e10401_d_n3;
        var_t1__blk110_dn4 = assign11130_e10401_d_n4;
        var_t1__blk110_dn5 = assign11130_e10401_d_n5;
        var_t1__blk110_dn6 = assign11130_e10401_d_n6;
        var_t1__blk110_dn7 = assign11130_e10401_d_n7;
        var_t1__blk110_dn8 = assign11130_e10401_d_n8;

        let (assign11140_e10411, assign11140_e10411_d_n3, assign11140_e10411_d_n4, assign11140_e10411_d_n5, assign11140_e10411_d_n6, assign11140_e10411_d_n7, assign11140_e10411_d_n8,) = {
    if (var_guard116 != 0.0) {
        let assign11140_e10406: f64 = (0.5 * var_vbgd_noswap);
        let assign11140_e10408: f64 = (assign11140_e10406 * var_prwb_i);
        let assign11140_e10409: f64 = (var_t1__blk110 - assign11140_e10408);
        (assign11140_e10409, (var_t1__blk110_dn3 - ((0.5 * var_vbgd_noswap_dn3) * var_prwb_i)), var_t1__blk110_dn4, (var_t1__blk110_dn5 - ((0.5 * var_vbgd_noswap_dn5) * var_prwb_i)), var_t1__blk110_dn6, var_t1__blk110_dn7, var_t1__blk110_dn8,)
    } else {
        (var_t1__blk110, var_t1__blk110_dn3, var_t1__blk110_dn4, var_t1__blk110_dn5, var_t1__blk110_dn6, var_t1__blk110_dn7, var_t1__blk110_dn8,)
    }
};
        var_t1__blk110 = assign11140_e10411;
        var_t1__blk110_dn3 = assign11140_e10411_d_n3;
        var_t1__blk110_dn4 = assign11140_e10411_d_n4;
        var_t1__blk110_dn5 = assign11140_e10411_d_n5;
        var_t1__blk110_dn6 = assign11140_e10411_d_n6;
        var_t1__blk110_dn7 = assign11140_e10411_d_n7;
        var_t1__blk110_dn8 = assign11140_e10411_d_n8;

        let (assign11150_e10424, assign11150_e10424_d_n3, assign11150_e10424_d_n4, assign11150_e10424_d_n5, assign11150_e10424_d_n6, assign11150_e10424_d_n7, assign11150_e10424_d_n8,) = {
    if (var_guard116 != 0.0) {
        let assign11150_e10417: f64 = (var_t1__blk110 * var_t1__blk110);
        let assign11150_e10419: f64 = (assign11150_e10417 + 0.01);
        let assign11150_e10420: f64 = (assign11150_e10419).sqrt();
        let assign11150_e10421: f64 = (var_t1__blk110 + assign11150_e10420);
        let assign11150_e10422: f64 = (0.5 * assign11150_e10421);
        (assign11150_e10422, (0.5 * (var_t1__blk110_dn3 + (((var_t1__blk110_dn3 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn3)) / (2.0 * assign11150_e10420)))), (0.5 * (var_t1__blk110_dn4 + (((var_t1__blk110_dn4 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn4)) / (2.0 * assign11150_e10420)))), (0.5 * (var_t1__blk110_dn5 + (((var_t1__blk110_dn5 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn5)) / (2.0 * assign11150_e10420)))), (0.5 * (var_t1__blk110_dn6 + (((var_t1__blk110_dn6 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn6)) / (2.0 * assign11150_e10420)))), (0.5 * (var_t1__blk110_dn7 + (((var_t1__blk110_dn7 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn7)) / (2.0 * assign11150_e10420)))), (0.5 * (var_t1__blk110_dn8 + (((var_t1__blk110_dn8 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn8)) / (2.0 * assign11150_e10420)))),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign11150_e10424;
        var_t0_dn3 = assign11150_e10424_d_n3;
        var_t0_dn4 = assign11150_e10424_d_n4;
        var_t0_dn5 = assign11150_e10424_d_n5;
        var_t0_dn6 = assign11150_e10424_d_n6;
        var_t0_dn7 = assign11150_e10424_d_n7;
        var_t0_dn8 = assign11150_e10424_d_n8;

        let (assign11160_e10438, assign11160_e10438_d_n3, assign11160_e10438_d_n4, assign11160_e10438_d_n5, assign11160_e10438_d_n6, assign11160_e10438_d_n7, assign11160_e10438_d_n8,) = {
    if (var_guard116 != 0.0) {
        let assign11160_e10431: f64 = (var_rdw_i * var_t0);
        let assign11160_e10432: f64 = (var_rdwmin_i + assign11160_e10431);
        let assign11160_e10434: f64 = (assign11160_e10432 * var_weffwrfactor);
        let assign11160_e10435: f64 = (var_rdraingeo + assign11160_e10434);
        let assign11160_e10436: f64 = (var_rdstemp * assign11160_e10435);
        (assign11160_e10436, (var_rdstemp * ((var_rdw_i * var_t0_dn3) * var_weffwrfactor)), ((var_rdstemp_dn4 * assign11160_e10435) + (var_rdstemp * ((var_rdw_i * var_t0_dn4) * var_weffwrfactor))), (var_rdstemp * ((var_rdw_i * var_t0_dn5) * var_weffwrfactor)), (var_rdstemp * ((var_rdw_i * var_t0_dn6) * var_weffwrfactor)), (var_rdstemp * ((var_rdw_i * var_t0_dn7) * var_weffwrfactor)), (var_rdstemp * ((var_rdw_i * var_t0_dn8) * var_weffwrfactor)),)
    } else {
        (var_rdrain, var_rdrain_dn3, var_rdrain_dn4, var_rdrain_dn5, var_rdrain_dn6, var_rdrain_dn7, var_rdrain_dn8,)
    }
};
        var_rdrain = assign11160_e10438;
        var_rdrain_dn3 = assign11160_e10438_d_n3;
        var_rdrain_dn4 = assign11160_e10438_d_n4;
        var_rdrain_dn5 = assign11160_e10438_d_n5;
        var_rdrain_dn6 = assign11160_e10438_d_n6;
        var_rdrain_dn7 = assign11160_e10438_d_n7;
        var_rdrain_dn8 = assign11160_e10438_d_n8;

        let (assign11170_e10447, assign11170_e10447_d_n3, assign11170_e10447_d_n4, assign11170_e10447_d_n5, assign11170_e10447_d_n6, assign11170_e10447_d_n7, assign11170_e10447_d_n8,) = {
    if (var_guard116 == 0.0) {
        let assign11170_e10444: f64 = (var_prwg_i * var_qia);
        let assign11170_e10445: f64 = (1.0 + assign11170_e10444);
        (assign11170_e10445, (var_prwg_i * var_qia_dn3), (var_prwg_i * var_qia_dn4), (var_prwg_i * var_qia_dn5), (var_prwg_i * var_qia_dn6), (var_prwg_i * var_qia_dn7), (var_prwg_i * var_qia_dn8),)
    } else {
        (var_t4__blk111, var_t4__blk111_dn3, var_t4__blk111_dn4, var_t4__blk111_dn5, var_t4__blk111_dn6, var_t4__blk111_dn7, var_t4__blk111_dn8,)
    }
};
        var_t4__blk111 = assign11170_e10447;
        var_t4__blk111_dn3 = assign11170_e10447_d_n3;
        var_t4__blk111_dn4 = assign11170_e10447_d_n4;
        var_t4__blk111_dn5 = assign11170_e10447_d_n5;
        var_t4__blk111_dn6 = assign11170_e10447_d_n6;
        var_t4__blk111_dn7 = assign11170_e10447_d_n7;
        var_t4__blk111_dn8 = assign11170_e10447_d_n8;

        let (assign11180_e10454, assign11180_e10454_d_n3, assign11180_e10454_d_n4, assign11180_e10454_d_n5, assign11180_e10454_d_n6, assign11180_e10454_d_n7, assign11180_e10454_d_n8,) = {
    if (var_guard116 == 0.0) {
        let assign11180_e10452: f64 = (1.0 / var_t4__blk111);
        (assign11180_e10452, (-(var_t4__blk111_dn3 / (var_t4__blk111 * var_t4__blk111))), (-(var_t4__blk111_dn4 / (var_t4__blk111 * var_t4__blk111))), (-(var_t4__blk111_dn5 / (var_t4__blk111 * var_t4__blk111))), (-(var_t4__blk111_dn6 / (var_t4__blk111 * var_t4__blk111))), (-(var_t4__blk111_dn7 / (var_t4__blk111 * var_t4__blk111))), (-(var_t4__blk111_dn8 / (var_t4__blk111 * var_t4__blk111))),)
    } else {
        (var_t1__blk110, var_t1__blk110_dn3, var_t1__blk110_dn4, var_t1__blk110_dn5, var_t1__blk110_dn6, var_t1__blk110_dn7, var_t1__blk110_dn8,)
    }
};
        var_t1__blk110 = assign11180_e10454;
        var_t1__blk110_dn3 = assign11180_e10454_d_n3;
        var_t1__blk110_dn4 = assign11180_e10454_d_n4;
        var_t1__blk110_dn5 = assign11180_e10454_d_n5;
        var_t1__blk110_dn6 = assign11180_e10454_d_n6;
        var_t1__blk110_dn7 = assign11180_e10454_d_n7;
        var_t1__blk110_dn8 = assign11180_e10454_d_n8;

        let (assign11190_e10467, assign11190_e10467_d_n3, assign11190_e10467_d_n4, assign11190_e10467_d_n5, assign11190_e10467_d_n6, assign11190_e10467_d_n7, assign11190_e10467_d_n8,) = {
    if (var_guard116 == 0.0) {
        let assign11190_e10461: f64 = (var_vbgd + var_vbgs);
        let assign11190_e10462: f64 = (0.5 * assign11190_e10461);
        let assign11190_e10464: f64 = (assign11190_e10462 * var_prwb_i);
        let assign11190_e10465: f64 = (var_t1__blk110 - assign11190_e10464);
        (assign11190_e10465, (var_t1__blk110_dn3 - ((0.5 * (var_vbgd_dn3 + var_vbgs_dn3)) * var_prwb_i)), var_t1__blk110_dn4, (var_t1__blk110_dn5 - ((0.5 * (var_vbgd_dn5 + var_vbgs_dn5)) * var_prwb_i)), (var_t1__blk110_dn6 - ((0.5 * (var_vbgd_dn6 + var_vbgs_dn6)) * var_prwb_i)), var_t1__blk110_dn7, var_t1__blk110_dn8,)
    } else {
        (var_t1__blk110, var_t1__blk110_dn3, var_t1__blk110_dn4, var_t1__blk110_dn5, var_t1__blk110_dn6, var_t1__blk110_dn7, var_t1__blk110_dn8,)
    }
};
        var_t1__blk110 = assign11190_e10467;
        var_t1__blk110_dn3 = assign11190_e10467_d_n3;
        var_t1__blk110_dn4 = assign11190_e10467_d_n4;
        var_t1__blk110_dn5 = assign11190_e10467_d_n5;
        var_t1__blk110_dn6 = assign11190_e10467_d_n6;
        var_t1__blk110_dn7 = assign11190_e10467_d_n7;
        var_t1__blk110_dn8 = assign11190_e10467_d_n8;

        let (assign11200_e10481, assign11200_e10481_d_n3, assign11200_e10481_d_n4, assign11200_e10481_d_n5, assign11200_e10481_d_n6, assign11200_e10481_d_n7, assign11200_e10481_d_n8,) = {
    if (var_guard116 == 0.0) {
        let assign11200_e10474: f64 = (var_t1__blk110 * var_t1__blk110);
        let assign11200_e10476: f64 = (assign11200_e10474 + 0.01);
        let assign11200_e10477: f64 = (assign11200_e10476).sqrt();
        let assign11200_e10478: f64 = (var_t1__blk110 + assign11200_e10477);
        let assign11200_e10479: f64 = (0.5 * assign11200_e10478);
        (assign11200_e10479, (0.5 * (var_t1__blk110_dn3 + (((var_t1__blk110_dn3 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn3)) / (2.0 * assign11200_e10477)))), (0.5 * (var_t1__blk110_dn4 + (((var_t1__blk110_dn4 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn4)) / (2.0 * assign11200_e10477)))), (0.5 * (var_t1__blk110_dn5 + (((var_t1__blk110_dn5 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn5)) / (2.0 * assign11200_e10477)))), (0.5 * (var_t1__blk110_dn6 + (((var_t1__blk110_dn6 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn6)) / (2.0 * assign11200_e10477)))), (0.5 * (var_t1__blk110_dn7 + (((var_t1__blk110_dn7 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn7)) / (2.0 * assign11200_e10477)))), (0.5 * (var_t1__blk110_dn8 + (((var_t1__blk110_dn8 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn8)) / (2.0 * assign11200_e10477)))),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign11200_e10481;
        var_t0_dn3 = assign11200_e10481_d_n3;
        var_t0_dn4 = assign11200_e10481_d_n4;
        var_t0_dn5 = assign11200_e10481_d_n5;
        var_t0_dn6 = assign11200_e10481_d_n6;
        var_t0_dn7 = assign11200_e10481_d_n7;
        var_t0_dn8 = assign11200_e10481_d_n8;

        let (assign11210_e10494, assign11210_e10494_d_n3, assign11210_e10494_d_n4, assign11210_e10494_d_n5, assign11210_e10494_d_n6, assign11210_e10494_d_n7, assign11210_e10494_d_n8,) = {
    if (var_guard116 == 0.0) {
        let assign11210_e10488: f64 = (var_rdsw_i * var_t0);
        let assign11210_e10489: f64 = (var_rdswmin_i + assign11210_e10488);
        let assign11210_e10491: f64 = (assign11210_e10489 * var_weffwrfactor);
        let assign11210_e10492: f64 = (var_rdstemp * assign11210_e10491);
        (assign11210_e10492, (var_rdstemp * ((var_rdsw_i * var_t0_dn3) * var_weffwrfactor)), ((var_rdstemp_dn4 * assign11210_e10491) + (var_rdstemp * ((var_rdsw_i * var_t0_dn4) * var_weffwrfactor))), (var_rdstemp * ((var_rdsw_i * var_t0_dn5) * var_weffwrfactor)), (var_rdstemp * ((var_rdsw_i * var_t0_dn6) * var_weffwrfactor)), (var_rdstemp * ((var_rdsw_i * var_t0_dn7) * var_weffwrfactor)), (var_rdstemp * ((var_rdsw_i * var_t0_dn8) * var_weffwrfactor)),)
    } else {
        (var_rdsi, var_rdsi_dn3, var_rdsi_dn4, var_rdsi_dn5, var_rdsi_dn6, var_rdsi_dn7, var_rdsi_dn8,)
    }
};
        var_rdsi = assign11210_e10494;
        var_rdsi_dn3 = assign11210_e10494_d_n3;
        var_rdsi_dn4 = assign11210_e10494_d_n4;
        var_rdsi_dn5 = assign11210_e10494_d_n5;
        var_rdsi_dn6 = assign11210_e10494_d_n6;
        var_rdsi_dn7 = assign11210_e10494_d_n7;
        var_rdsi_dn8 = assign11210_e10494_d_n8;

        let (assign11220_e10509, assign11220_e10509_d_n3, assign11220_e10509_d_n4, assign11220_e10509_d_n5, assign11220_e10509_d_n6, assign11220_e10509_d_n7, assign11220_e10509_d_n8,) = {
    if (var_guard116 == 0.0) {
        let assign11220_e10500: f64 = (p.p2 * var_beta);
        let assign11220_e10502: f64 = (assign11220_e10500 * var_ids0_ov_dqi);
        let assign11220_e10504: f64 = (assign11220_e10502 / var_dvsat);
        let assign11220_e10506: f64 = (assign11220_e10504 * var_rdsi);
        let assign11220_e10507: f64 = (1.0 + assign11220_e10506);
        (assign11220_e10507, ((((((((p.p2 * var_beta_dn3) * var_ids0_ov_dqi) + (assign11220_e10500 * var_ids0_ov_dqi_dn3)) * var_dvsat) - (assign11220_e10502 * var_dvsat_dn3)) / (var_dvsat * var_dvsat)) * var_rdsi) + (assign11220_e10504 * var_rdsi_dn3)), ((((((((p.p2 * var_beta_dn4) * var_ids0_ov_dqi) + (assign11220_e10500 * var_ids0_ov_dqi_dn4)) * var_dvsat) - (assign11220_e10502 * var_dvsat_dn4)) / (var_dvsat * var_dvsat)) * var_rdsi) + (assign11220_e10504 * var_rdsi_dn4)), ((((((((p.p2 * var_beta_dn5) * var_ids0_ov_dqi) + (assign11220_e10500 * var_ids0_ov_dqi_dn5)) * var_dvsat) - (assign11220_e10502 * var_dvsat_dn5)) / (var_dvsat * var_dvsat)) * var_rdsi) + (assign11220_e10504 * var_rdsi_dn5)), ((((((((p.p2 * var_beta_dn6) * var_ids0_ov_dqi) + (assign11220_e10500 * var_ids0_ov_dqi_dn6)) * var_dvsat) - (assign11220_e10502 * var_dvsat_dn6)) / (var_dvsat * var_dvsat)) * var_rdsi) + (assign11220_e10504 * var_rdsi_dn6)), ((((((((p.p2 * var_beta_dn7) * var_ids0_ov_dqi) + (assign11220_e10500 * var_ids0_ov_dqi_dn7)) * var_dvsat) - (assign11220_e10502 * var_dvsat_dn7)) / (var_dvsat * var_dvsat)) * var_rdsi) + (assign11220_e10504 * var_rdsi_dn7)), ((((((((p.p2 * var_beta_dn8) * var_ids0_ov_dqi) + (assign11220_e10500 * var_ids0_ov_dqi_dn8)) * var_dvsat) - (assign11220_e10502 * var_dvsat_dn8)) / (var_dvsat * var_dvsat)) * var_rdsi) + (assign11220_e10504 * var_rdsi_dn8)),)
    } else {
        (var_dr, var_dr_dn3, var_dr_dn4, var_dr_dn5, var_dr_dn6, var_dr_dn7, var_dr_dn8,)
    }
};
        var_dr = assign11220_e10509;
        var_dr_dn3 = assign11220_e10509_d_n3;
        var_dr_dn4 = assign11220_e10509_d_n4;
        var_dr_dn5 = assign11220_e10509_d_n5;
        var_dr_dn6 = assign11220_e10509_d_n6;
        var_dr_dn7 = assign11220_e10509_d_n7;
        var_dr_dn8 = assign11220_e10509_d_n8;

        let (assign11230_e10514, assign11230_e10514_d_n3, assign11230_e10514_d_n4, assign11230_e10514_d_n5, assign11230_e10514_d_n6, assign11230_e10514_d_n7, assign11230_e10514_d_n8,) = {
    if (var_guard116 == 0.0) {
        (var_rdraingeo, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdrain, var_rdrain_dn3, var_rdrain_dn4, var_rdrain_dn5, var_rdrain_dn6, var_rdrain_dn7, var_rdrain_dn8,)
    }
};
        var_rdrain = assign11230_e10514;
        var_rdrain_dn3 = assign11230_e10514_d_n3;
        var_rdrain_dn4 = assign11230_e10514_d_n4;
        var_rdrain_dn5 = assign11230_e10514_d_n5;
        var_rdrain_dn6 = assign11230_e10514_d_n6;
        var_rdrain_dn7 = assign11230_e10514_d_n7;
        var_rdrain_dn8 = assign11230_e10514_d_n8;

        let (assign11240_e10519, assign11240_e10519_d_n3, assign11240_e10519_d_n4, assign11240_e10519_d_n5, assign11240_e10519_d_n6, assign11240_e10519_d_n7, assign11240_e10519_d_n8,) = {
    if (var_guard116 == 0.0) {
        (var_rsourcegeo, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rsource, var_rsource_dn3, var_rsource_dn4, var_rsource_dn5, var_rsource_dn6, var_rsource_dn7, var_rsource_dn8,)
    }
};
        var_rsource = assign11240_e10519;
        var_rsource_dn3 = assign11240_e10519_d_n3;
        var_rsource_dn4 = assign11240_e10519_d_n4;
        var_rsource_dn5 = assign11240_e10519_d_n5;
        var_rsource_dn6 = assign11240_e10519_d_n6;
        var_rsource_dn7 = assign11240_e10519_d_n7;
        var_rsource_dn8 = assign11240_e10519_d_n8;

        let assign11250_e10522: f64 = if p.p14 == 2.0 { 1.0 } else { 0.0 };
        var_guard117 = assign11250_e10522;

        let (assign11260_e10533, assign11260_e10533_d_n3, assign11260_e10533_d_n4, assign11260_e10533_d_n5, assign11260_e10533_d_n6, assign11260_e10533_d_n7, assign11260_e10533_d_n8,) = {
    if ((var_guard116 == 0.0) && (var_guard117 != 0.0)) {
        let assign11260_e10530: f64 = (var_prwg_i * var_qia);
        let assign11260_e10531: f64 = (1.0 + assign11260_e10530);
        (assign11260_e10531, (var_prwg_i * var_qia_dn3), (var_prwg_i * var_qia_dn4), (var_prwg_i * var_qia_dn5), (var_prwg_i * var_qia_dn6), (var_prwg_i * var_qia_dn7), (var_prwg_i * var_qia_dn8),)
    } else {
        (var_t4__blk111, var_t4__blk111_dn3, var_t4__blk111_dn4, var_t4__blk111_dn5, var_t4__blk111_dn6, var_t4__blk111_dn7, var_t4__blk111_dn8,)
    }
};
        var_t4__blk111 = assign11260_e10533;
        var_t4__blk111_dn3 = assign11260_e10533_d_n3;
        var_t4__blk111_dn4 = assign11260_e10533_d_n4;
        var_t4__blk111_dn5 = assign11260_e10533_d_n5;
        var_t4__blk111_dn6 = assign11260_e10533_d_n6;
        var_t4__blk111_dn7 = assign11260_e10533_d_n7;
        var_t4__blk111_dn8 = assign11260_e10533_d_n8;

        let (assign11270_e10542, assign11270_e10542_d_n3, assign11270_e10542_d_n4, assign11270_e10542_d_n5, assign11270_e10542_d_n6, assign11270_e10542_d_n7, assign11270_e10542_d_n8,) = {
    if ((var_guard116 == 0.0) && (var_guard117 != 0.0)) {
        let assign11270_e10540: f64 = (1.0 / var_t4__blk111);
        (assign11270_e10540, (-(var_t4__blk111_dn3 / (var_t4__blk111 * var_t4__blk111))), (-(var_t4__blk111_dn4 / (var_t4__blk111 * var_t4__blk111))), (-(var_t4__blk111_dn5 / (var_t4__blk111 * var_t4__blk111))), (-(var_t4__blk111_dn6 / (var_t4__blk111 * var_t4__blk111))), (-(var_t4__blk111_dn7 / (var_t4__blk111 * var_t4__blk111))), (-(var_t4__blk111_dn8 / (var_t4__blk111 * var_t4__blk111))),)
    } else {
        (var_t1__blk110, var_t1__blk110_dn3, var_t1__blk110_dn4, var_t1__blk110_dn5, var_t1__blk110_dn6, var_t1__blk110_dn7, var_t1__blk110_dn8,)
    }
};
        var_t1__blk110 = assign11270_e10542;
        var_t1__blk110_dn3 = assign11270_e10542_d_n3;
        var_t1__blk110_dn4 = assign11270_e10542_d_n4;
        var_t1__blk110_dn5 = assign11270_e10542_d_n5;
        var_t1__blk110_dn6 = assign11270_e10542_d_n6;
        var_t1__blk110_dn7 = assign11270_e10542_d_n7;
        var_t1__blk110_dn8 = assign11270_e10542_d_n8;

        let (assign11280_e10557, assign11280_e10557_d_n3, assign11280_e10557_d_n4, assign11280_e10557_d_n5, assign11280_e10557_d_n6, assign11280_e10557_d_n7, assign11280_e10557_d_n8,) = {
    if ((var_guard116 == 0.0) && (var_guard117 != 0.0)) {
        let assign11280_e10551: f64 = (var_vbgd + var_vbgs);
        let assign11280_e10552: f64 = (0.5 * assign11280_e10551);
        let assign11280_e10554: f64 = (assign11280_e10552 * var_prwb_i);
        let assign11280_e10555: f64 = (var_t1__blk110 - assign11280_e10554);
        (assign11280_e10555, (var_t1__blk110_dn3 - ((0.5 * (var_vbgd_dn3 + var_vbgs_dn3)) * var_prwb_i)), var_t1__blk110_dn4, (var_t1__blk110_dn5 - ((0.5 * (var_vbgd_dn5 + var_vbgs_dn5)) * var_prwb_i)), (var_t1__blk110_dn6 - ((0.5 * (var_vbgd_dn6 + var_vbgs_dn6)) * var_prwb_i)), var_t1__blk110_dn7, var_t1__blk110_dn8,)
    } else {
        (var_t1__blk110, var_t1__blk110_dn3, var_t1__blk110_dn4, var_t1__blk110_dn5, var_t1__blk110_dn6, var_t1__blk110_dn7, var_t1__blk110_dn8,)
    }
};
        var_t1__blk110 = assign11280_e10557;
        var_t1__blk110_dn3 = assign11280_e10557_d_n3;
        var_t1__blk110_dn4 = assign11280_e10557_d_n4;
        var_t1__blk110_dn5 = assign11280_e10557_d_n5;
        var_t1__blk110_dn6 = assign11280_e10557_d_n6;
        var_t1__blk110_dn7 = assign11280_e10557_d_n7;
        var_t1__blk110_dn8 = assign11280_e10557_d_n8;

        let (assign11290_e10573, assign11290_e10573_d_n3, assign11290_e10573_d_n4, assign11290_e10573_d_n5, assign11290_e10573_d_n6, assign11290_e10573_d_n7, assign11290_e10573_d_n8,) = {
    if ((var_guard116 == 0.0) && (var_guard117 != 0.0)) {
        let assign11290_e10566: f64 = (var_t1__blk110 * var_t1__blk110);
        let assign11290_e10568: f64 = (assign11290_e10566 + 0.01);
        let assign11290_e10569: f64 = (assign11290_e10568).sqrt();
        let assign11290_e10570: f64 = (var_t1__blk110 + assign11290_e10569);
        let assign11290_e10571: f64 = (0.5 * assign11290_e10570);
        (assign11290_e10571, (0.5 * (var_t1__blk110_dn3 + (((var_t1__blk110_dn3 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn3)) / (2.0 * assign11290_e10569)))), (0.5 * (var_t1__blk110_dn4 + (((var_t1__blk110_dn4 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn4)) / (2.0 * assign11290_e10569)))), (0.5 * (var_t1__blk110_dn5 + (((var_t1__blk110_dn5 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn5)) / (2.0 * assign11290_e10569)))), (0.5 * (var_t1__blk110_dn6 + (((var_t1__blk110_dn6 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn6)) / (2.0 * assign11290_e10569)))), (0.5 * (var_t1__blk110_dn7 + (((var_t1__blk110_dn7 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn7)) / (2.0 * assign11290_e10569)))), (0.5 * (var_t1__blk110_dn8 + (((var_t1__blk110_dn8 * var_t1__blk110) + (var_t1__blk110 * var_t1__blk110_dn8)) / (2.0 * assign11290_e10569)))),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign11290_e10573;
        var_t0_dn3 = assign11290_e10573_d_n3;
        var_t0_dn4 = assign11290_e10573_d_n4;
        var_t0_dn5 = assign11290_e10573_d_n5;
        var_t0_dn6 = assign11290_e10573_d_n6;
        var_t0_dn7 = assign11290_e10573_d_n7;
        var_t0_dn8 = assign11290_e10573_d_n8;

        let (assign11300_e10592, assign11300_e10592_d_n3, assign11300_e10592_d_n4, assign11300_e10592_d_n5, assign11300_e10592_d_n6, assign11300_e10592_d_n7, assign11300_e10592_d_n8,) = {
    if ((var_guard116 == 0.0) && (var_guard117 != 0.0)) {
        let assign11300_e10581: f64 = (var_rsourcegeo + var_rdraingeo);
        let assign11300_e10583: f64 = (assign11300_e10581 + var_rdswmin_i);
        let assign11300_e10586: f64 = (var_rdsw_i * var_t0);
        let assign11300_e10587: f64 = (assign11300_e10583 + assign11300_e10586);
        let assign11300_e10588: f64 = (var_rdstemp * assign11300_e10587);
        let assign11300_e10590: f64 = (assign11300_e10588 * var_weffwrfactor);
        (assign11300_e10590, ((var_rdstemp * (var_rdsw_i * var_t0_dn3)) * var_weffwrfactor), (((var_rdstemp_dn4 * assign11300_e10587) + (var_rdstemp * (var_rdsw_i * var_t0_dn4))) * var_weffwrfactor), ((var_rdstemp * (var_rdsw_i * var_t0_dn5)) * var_weffwrfactor), ((var_rdstemp * (var_rdsw_i * var_t0_dn6)) * var_weffwrfactor), ((var_rdstemp * (var_rdsw_i * var_t0_dn7)) * var_weffwrfactor), ((var_rdstemp * (var_rdsw_i * var_t0_dn8)) * var_weffwrfactor),)
    } else {
        (var_rdsi, var_rdsi_dn3, var_rdsi_dn4, var_rdsi_dn5, var_rdsi_dn6, var_rdsi_dn7, var_rdsi_dn8,)
    }
};
        var_rdsi = assign11300_e10592;
        var_rdsi_dn3 = assign11300_e10592_d_n3;
        var_rdsi_dn4 = assign11300_e10592_d_n4;
        var_rdsi_dn5 = assign11300_e10592_d_n5;
        var_rdsi_dn6 = assign11300_e10592_d_n6;
        var_rdsi_dn7 = assign11300_e10592_d_n7;
        var_rdsi_dn8 = assign11300_e10592_d_n8;

        let (assign11310_e10609, assign11310_e10609_d_n3, assign11310_e10609_d_n4, assign11310_e10609_d_n5, assign11310_e10609_d_n6, assign11310_e10609_d_n7, assign11310_e10609_d_n8,) = {
    if ((var_guard116 == 0.0) && (var_guard117 != 0.0)) {
        let assign11310_e10600: f64 = (p.p2 * var_beta);
        let assign11310_e10602: f64 = (assign11310_e10600 * var_ids0_ov_dqi);
        let assign11310_e10604: f64 = (assign11310_e10602 / var_dvsat);
        let assign11310_e10606: f64 = (assign11310_e10604 * var_rdsi);
        let assign11310_e10607: f64 = (1.0 + assign11310_e10606);
        (assign11310_e10607, ((((((((p.p2 * var_beta_dn3) * var_ids0_ov_dqi) + (assign11310_e10600 * var_ids0_ov_dqi_dn3)) * var_dvsat) - (assign11310_e10602 * var_dvsat_dn3)) / (var_dvsat * var_dvsat)) * var_rdsi) + (assign11310_e10604 * var_rdsi_dn3)), ((((((((p.p2 * var_beta_dn4) * var_ids0_ov_dqi) + (assign11310_e10600 * var_ids0_ov_dqi_dn4)) * var_dvsat) - (assign11310_e10602 * var_dvsat_dn4)) / (var_dvsat * var_dvsat)) * var_rdsi) + (assign11310_e10604 * var_rdsi_dn4)), ((((((((p.p2 * var_beta_dn5) * var_ids0_ov_dqi) + (assign11310_e10600 * var_ids0_ov_dqi_dn5)) * var_dvsat) - (assign11310_e10602 * var_dvsat_dn5)) / (var_dvsat * var_dvsat)) * var_rdsi) + (assign11310_e10604 * var_rdsi_dn5)), ((((((((p.p2 * var_beta_dn6) * var_ids0_ov_dqi) + (assign11310_e10600 * var_ids0_ov_dqi_dn6)) * var_dvsat) - (assign11310_e10602 * var_dvsat_dn6)) / (var_dvsat * var_dvsat)) * var_rdsi) + (assign11310_e10604 * var_rdsi_dn6)), ((((((((p.p2 * var_beta_dn7) * var_ids0_ov_dqi) + (assign11310_e10600 * var_ids0_ov_dqi_dn7)) * var_dvsat) - (assign11310_e10602 * var_dvsat_dn7)) / (var_dvsat * var_dvsat)) * var_rdsi) + (assign11310_e10604 * var_rdsi_dn7)), ((((((((p.p2 * var_beta_dn8) * var_ids0_ov_dqi) + (assign11310_e10600 * var_ids0_ov_dqi_dn8)) * var_dvsat) - (assign11310_e10602 * var_dvsat_dn8)) / (var_dvsat * var_dvsat)) * var_rdsi) + (assign11310_e10604 * var_rdsi_dn8)),)
    } else {
        (var_dr, var_dr_dn3, var_dr_dn4, var_dr_dn5, var_dr_dn6, var_dr_dn7, var_dr_dn8,)
    }
};
        var_dr = assign11310_e10609;
        var_dr_dn3 = assign11310_e10609_d_n3;
        var_dr_dn4 = assign11310_e10609_d_n4;
        var_dr_dn5 = assign11310_e10609_d_n5;
        var_dr_dn6 = assign11310_e10609_d_n6;
        var_dr_dn7 = assign11310_e10609_d_n7;
        var_dr_dn8 = assign11310_e10609_d_n8;

        let (assign11320_e10616, assign11320_e10616_d_n3, assign11320_e10616_d_n4, assign11320_e10616_d_n5, assign11320_e10616_d_n6, assign11320_e10616_d_n7, assign11320_e10616_d_n8,) = {
    if ((var_guard116 == 0.0) && (var_guard117 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdrain, var_rdrain_dn3, var_rdrain_dn4, var_rdrain_dn5, var_rdrain_dn6, var_rdrain_dn7, var_rdrain_dn8,)
    }
};
        var_rdrain = assign11320_e10616;
        var_rdrain_dn3 = assign11320_e10616_d_n3;
        var_rdrain_dn4 = assign11320_e10616_d_n4;
        var_rdrain_dn5 = assign11320_e10616_d_n5;
        var_rdrain_dn6 = assign11320_e10616_d_n6;
        var_rdrain_dn7 = assign11320_e10616_d_n7;
        var_rdrain_dn8 = assign11320_e10616_d_n8;

        let (assign11330_e10623, assign11330_e10623_d_n3, assign11330_e10623_d_n4, assign11330_e10623_d_n5, assign11330_e10623_d_n6, assign11330_e10623_d_n7, assign11330_e10623_d_n8,) = {
    if ((var_guard116 == 0.0) && (var_guard117 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rsource, var_rsource_dn3, var_rsource_dn4, var_rsource_dn5, var_rsource_dn6, var_rsource_dn7, var_rsource_dn8,)
    }
};
        var_rsource = assign11330_e10623;
        var_rsource_dn3 = assign11330_e10623_d_n3;
        var_rsource_dn4 = assign11330_e10623_d_n4;
        var_rsource_dn5 = assign11330_e10623_d_n5;
        var_rsource_dn6 = assign11330_e10623_d_n6;
        var_rsource_dn7 = assign11330_e10623_d_n7;
        var_rsource_dn8 = assign11330_e10623_d_n8;

        let assign11340_e10626: f64 = (var_beta / var_cox1);
        let assign11340_e10628: f64 = (assign11340_e10626 * var_ids0);
        let assign11340_e10630: f64 = (assign11340_e10628 * var_moc);
        let assign11340_e10632: f64 = (assign11340_e10630 * var_mnud);
        let assign11340_e10635: f64 = (var_dvsat * var_dr);
        let assign11340_e10636: f64 = (assign11340_e10632 / assign11340_e10635);
        var_ids = assign11340_e10636;
        var_ids_dn3 = ((((((((((var_beta_dn3 / var_cox1) * var_ids0) + (assign11340_e10626 * var_ids0_dn3)) * var_moc) + (assign11340_e10628 * var_moc_dn3)) * var_mnud) + (assign11340_e10630 * var_mnud_dn3)) * assign11340_e10635) - (assign11340_e10632 * ((var_dvsat_dn3 * var_dr) + (var_dvsat * var_dr_dn3)))) / (assign11340_e10635 * assign11340_e10635));
        var_ids_dn4 = ((((((((((var_beta_dn4 / var_cox1) * var_ids0) + (assign11340_e10626 * var_ids0_dn4)) * var_moc) + (assign11340_e10628 * var_moc_dn4)) * var_mnud) + (assign11340_e10630 * var_mnud_dn4)) * assign11340_e10635) - (assign11340_e10632 * ((var_dvsat_dn4 * var_dr) + (var_dvsat * var_dr_dn4)))) / (assign11340_e10635 * assign11340_e10635));
        var_ids_dn5 = ((((((((((var_beta_dn5 / var_cox1) * var_ids0) + (assign11340_e10626 * var_ids0_dn5)) * var_moc) + (assign11340_e10628 * var_moc_dn5)) * var_mnud) + (assign11340_e10630 * var_mnud_dn5)) * assign11340_e10635) - (assign11340_e10632 * ((var_dvsat_dn5 * var_dr) + (var_dvsat * var_dr_dn5)))) / (assign11340_e10635 * assign11340_e10635));
        var_ids_dn6 = ((((((((((var_beta_dn6 / var_cox1) * var_ids0) + (assign11340_e10626 * var_ids0_dn6)) * var_moc) + (assign11340_e10628 * var_moc_dn6)) * var_mnud) + (assign11340_e10630 * var_mnud_dn6)) * assign11340_e10635) - (assign11340_e10632 * ((var_dvsat_dn6 * var_dr) + (var_dvsat * var_dr_dn6)))) / (assign11340_e10635 * assign11340_e10635));
        var_ids_dn7 = ((((((((((var_beta_dn7 / var_cox1) * var_ids0) + (assign11340_e10626 * var_ids0_dn7)) * var_moc) + (assign11340_e10628 * var_moc_dn7)) * var_mnud) + (assign11340_e10630 * var_mnud_dn7)) * assign11340_e10635) - (assign11340_e10632 * ((var_dvsat_dn7 * var_dr) + (var_dvsat * var_dr_dn7)))) / (assign11340_e10635 * assign11340_e10635));
        var_ids_dn8 = ((((((((((var_beta_dn8 / var_cox1) * var_ids0) + (assign11340_e10626 * var_ids0_dn8)) * var_moc) + (assign11340_e10628 * var_moc_dn8)) * var_mnud) + (assign11340_e10630 * var_mnud_dn8)) * assign11340_e10635) - (assign11340_e10632 * ((var_dvsat_dn8 * var_dr) + (var_dvsat * var_dr_dn8)))) / (assign11340_e10635 * assign11340_e10635));

        let assign11350_e10639: f64 = (p.p2 * var_ids);
        var_ids = assign11350_e10639;
        var_ids_dn3 = (p.p2 * var_ids_dn3);
        var_ids_dn4 = (p.p2 * var_ids_dn4);
        var_ids_dn5 = (p.p2 * var_ids_dn5);
        var_ids_dn6 = (p.p2 * var_ids_dn6);
        var_ids_dn7 = (p.p2 * var_ids_dn7);
        var_ids_dn8 = (p.p2 * var_ids_dn8);

        let assign11360_e10642: f64 = (var_qfrontd + var_qfronts);
        let assign11360_e10644: f64 = (assign11360_e10642 / 2.0);
        var_qfg = assign11360_e10644;
        var_qfg_dn3 = ((var_qfrontd_dn3 + var_qfronts_dn3) / 2.0);
        var_qfg_dn4 = ((var_qfrontd_dn4 + var_qfronts_dn4) / 2.0);
        var_qfg_dn5 = ((var_qfrontd_dn5 + var_qfronts_dn5) / 2.0);
        var_qfg_dn6 = ((var_qfrontd_dn6 + var_qfronts_dn6) / 2.0);
        var_qfg_dn7 = ((var_qfrontd_dn7 + var_qfronts_dn7) / 2.0);
        var_qfg_dn8 = ((var_qfrontd_dn8 + var_qfronts_dn8) / 2.0);

        let assign11370_e10647: f64 = (1.0 / 6.0);
        let assign11370_e10651: f64 = (2.0 * var_qtotd);
        let assign11370_e10652: f64 = (var_qtots + assign11370_e10651);
        let assign11370_e10653: f64 = (assign11370_e10647 * assign11370_e10652);
        var_qd = assign11370_e10653;
        var_qd_dn3 = (assign11370_e10647 * (var_qtots_dn3 + (2.0 * var_qtotd_dn3)));
        var_qd_dn4 = (assign11370_e10647 * (var_qtots_dn4 + (2.0 * var_qtotd_dn4)));
        var_qd_dn5 = (assign11370_e10647 * (var_qtots_dn5 + (2.0 * var_qtotd_dn5)));
        var_qd_dn6 = (assign11370_e10647 * (var_qtots_dn6 + (2.0 * var_qtotd_dn6)));
        var_qd_dn7 = (assign11370_e10647 * (var_qtots_dn7 + (2.0 * var_qtotd_dn7)));
        var_qd_dn8 = (assign11370_e10647 * (var_qtots_dn8 + (2.0 * var_qtotd_dn8)));

        let assign11380_e10656: f64 = (1.0 / 6.0);
        let assign11380_e10659: f64 = (2.0 * var_qtots);
        let assign11380_e10661: f64 = (assign11380_e10659 + var_qtotd);
        let assign11380_e10662: f64 = (assign11380_e10656 * assign11380_e10661);
        var_qs = assign11380_e10662;
        var_qs_dn3 = (assign11380_e10656 * ((2.0 * var_qtots_dn3) + var_qtotd_dn3));
        var_qs_dn4 = (assign11380_e10656 * ((2.0 * var_qtots_dn4) + var_qtotd_dn4));
        var_qs_dn5 = (assign11380_e10656 * ((2.0 * var_qtots_dn5) + var_qtotd_dn5));
        var_qs_dn6 = (assign11380_e10656 * ((2.0 * var_qtots_dn6) + var_qtotd_dn6));
        var_qs_dn7 = (assign11380_e10656 * ((2.0 * var_qtots_dn7) + var_qtotd_dn7));
        var_qs_dn8 = (assign11380_e10656 * ((2.0 * var_qtots_dn8) + var_qtotd_dn8));

        let assign11390_e10665: f64 = (var_qbackd + var_qbacks);
        let assign11390_e10667: f64 = (assign11390_e10665 / 2.0);
        var_qbg = assign11390_e10667;
        var_qbg_dn3 = ((var_qbackd_dn3 + var_qbacks_dn3) / 2.0);
        var_qbg_dn4 = ((var_qbackd_dn4 + var_qbacks_dn4) / 2.0);
        var_qbg_dn5 = ((var_qbackd_dn5 + var_qbacks_dn5) / 2.0);
        var_qbg_dn6 = ((var_qbackd_dn6 + var_qbacks_dn6) / 2.0);
        var_qbg_dn7 = ((var_qbackd_dn7 + var_qbacks_dn7) / 2.0);
        var_qbg_dn8 = ((var_qbackd_dn8 + var_qbacks_dn8) / 2.0);

        let assign11400_e10670: f64 = if var_qmtcencv_i > 0.0 { 1.0 } else { 0.0 };
        var_guard118 = assign11400_e10670;

        let (assign11410_e10680, assign11410_e10680_d_n3, assign11410_e10680_d_n4, assign11410_e10680_d_n5, assign11410_e10680_d_n6, assign11410_e10680_d_n7, assign11410_e10680_d_n8,) = {
    if (var_guard118 != 0.0) {
        let assign11410_e10675: f64 = (var_etaqm_i * var_qba);
        let assign11410_e10676: f64 = (var_qia + assign11410_e10675);
        let assign11410_e10678: f64 = (assign11410_e10676 / var_qm0_i);
        (assign11410_e10678, (var_qia_dn3 / var_qm0_i), (var_qia_dn4 / var_qm0_i), (var_qia_dn5 / var_qm0_i), (var_qia_dn6 / var_qm0_i), (var_qia_dn7 / var_qm0_i), (var_qia_dn8 / var_qm0_i),)
    } else {
        (var_t4, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8,)
    }
};
        var_t4 = assign11410_e10680;
        var_t4_dn3 = assign11410_e10680_d_n3;
        var_t4_dn4 = assign11410_e10680_d_n4;
        var_t4_dn5 = assign11410_e10680_d_n5;
        var_t4_dn6 = assign11410_e10680_d_n6;
        var_t4_dn7 = assign11410_e10680_d_n7;
        var_t4_dn8 = assign11410_e10680_d_n8;

        let (assign11420_e10688, assign11420_e10688_d_n3, assign11420_e10688_d_n4, assign11420_e10688_d_n5, assign11420_e10688_d_n6, assign11420_e10688_d_n7, assign11420_e10688_d_n8,) = {
    if (var_guard118 != 0.0) {
        let assign11420_e10685: f64 = (var_t4).powf(var_pqm_i);
        let assign11420_e10686: f64 = (1.0 + assign11420_e10685);
        (assign11420_e10686, if 0.0 == 0.0 && ((var_pqm_i) as f64).is_finite() && ((var_pqm_i) as f64).fract() == 0.0 { if var_pqm_i == 0.0 { 0.0 } else { (var_pqm_i * ((var_t4).powf(var_pqm_i - 1.0) * var_t4_dn3)) } } else { (assign11420_e10685 * (var_pqm_i * (var_t4_dn3 / var_t4))) }, if 0.0 == 0.0 && ((var_pqm_i) as f64).is_finite() && ((var_pqm_i) as f64).fract() == 0.0 { if var_pqm_i == 0.0 { 0.0 } else { (var_pqm_i * ((var_t4).powf(var_pqm_i - 1.0) * var_t4_dn4)) } } else { (assign11420_e10685 * (var_pqm_i * (var_t4_dn4 / var_t4))) }, if 0.0 == 0.0 && ((var_pqm_i) as f64).is_finite() && ((var_pqm_i) as f64).fract() == 0.0 { if var_pqm_i == 0.0 { 0.0 } else { (var_pqm_i * ((var_t4).powf(var_pqm_i - 1.0) * var_t4_dn5)) } } else { (assign11420_e10685 * (var_pqm_i * (var_t4_dn5 / var_t4))) }, if 0.0 == 0.0 && ((var_pqm_i) as f64).is_finite() && ((var_pqm_i) as f64).fract() == 0.0 { if var_pqm_i == 0.0 { 0.0 } else { (var_pqm_i * ((var_t4).powf(var_pqm_i - 1.0) * var_t4_dn6)) } } else { (assign11420_e10685 * (var_pqm_i * (var_t4_dn6 / var_t4))) }, if 0.0 == 0.0 && ((var_pqm_i) as f64).is_finite() && ((var_pqm_i) as f64).fract() == 0.0 { if var_pqm_i == 0.0 { 0.0 } else { (var_pqm_i * ((var_t4).powf(var_pqm_i - 1.0) * var_t4_dn7)) } } else { (assign11420_e10685 * (var_pqm_i * (var_t4_dn7 / var_t4))) }, if 0.0 == 0.0 && ((var_pqm_i) as f64).is_finite() && ((var_pqm_i) as f64).fract() == 0.0 { if var_pqm_i == 0.0 { 0.0 } else { (var_pqm_i * ((var_t4).powf(var_pqm_i - 1.0) * var_t4_dn8)) } } else { (assign11420_e10685 * (var_pqm_i * (var_t4_dn8 / var_t4))) },)
    } else {
        (var_t5, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8,)
    }
};
        var_t5 = assign11420_e10688;
        var_t5_dn3 = assign11420_e10688_d_n3;
        var_t5_dn4 = assign11420_e10688_d_n4;
        var_t5_dn5 = assign11420_e10688_d_n5;
        var_t5_dn6 = assign11420_e10688_d_n6;
        var_t5_dn7 = assign11420_e10688_d_n7;
        var_t5_dn8 = assign11420_e10688_d_n8;

        let (assign11430_e10692,) = {
    if (var_guard118 != 0.0) {
        (p.p49,)
    } else {
        (var_tcen0,)
    }
};
        var_tcen0 = assign11430_e10692;

        let (assign11440_e10698, assign11440_e10698_d_n3, assign11440_e10698_d_n4, assign11440_e10698_d_n5, assign11440_e10698_d_n6, assign11440_e10698_d_n7, assign11440_e10698_d_n8,) = {
    if (var_guard118 != 0.0) {
        let assign11440_e10696: f64 = (var_tcen0 / var_t5);
        (assign11440_e10696, (-((var_tcen0 * var_t5_dn3) / (var_t5 * var_t5))), (-((var_tcen0 * var_t5_dn4) / (var_t5 * var_t5))), (-((var_tcen0 * var_t5_dn5) / (var_t5 * var_t5))), (-((var_tcen0 * var_t5_dn6) / (var_t5 * var_t5))), (-((var_tcen0 * var_t5_dn7) / (var_t5 * var_t5))), (-((var_tcen0 * var_t5_dn8) / (var_t5 * var_t5))),)
    } else {
        (var_tcen, var_tcen_dn3, var_tcen_dn4, var_tcen_dn5, var_tcen_dn6, var_tcen_dn7, var_tcen_dn8,)
    }
};
        var_tcen = assign11440_e10698;
        var_tcen_dn3 = assign11440_e10698_d_n3;
        var_tcen_dn4 = assign11440_e10698_d_n4;
        var_tcen_dn5 = assign11440_e10698_d_n5;
        var_tcen_dn6 = assign11440_e10698_d_n6;
        var_tcen_dn7 = assign11440_e10698_d_n7;
        var_tcen_dn8 = assign11440_e10698_d_n8;

        *var_dr_slot = var_dr;
        *var_dr_dn3_slot = var_dr_dn3;
        *var_dr_dn4_slot = var_dr_dn4;
        *var_dr_dn5_slot = var_dr_dn5;
        *var_dr_dn6_slot = var_dr_dn6;
        *var_dr_dn7_slot = var_dr_dn7;
        *var_dr_dn8_slot = var_dr_dn8;
        *var_guard117_slot = var_guard117;
        *var_guard118_slot = var_guard118;
        *var_ids_slot = var_ids;
        *var_ids_dn3_slot = var_ids_dn3;
        *var_ids_dn4_slot = var_ids_dn4;
        *var_ids_dn5_slot = var_ids_dn5;
        *var_ids_dn6_slot = var_ids_dn6;
        *var_ids_dn7_slot = var_ids_dn7;
        *var_ids_dn8_slot = var_ids_dn8;
        *var_qbg_slot = var_qbg;
        *var_qbg_dn3_slot = var_qbg_dn3;
        *var_qbg_dn4_slot = var_qbg_dn4;
        *var_qbg_dn5_slot = var_qbg_dn5;
        *var_qbg_dn6_slot = var_qbg_dn6;
        *var_qbg_dn7_slot = var_qbg_dn7;
        *var_qbg_dn8_slot = var_qbg_dn8;
        *var_qd_slot = var_qd;
        *var_qd_dn3_slot = var_qd_dn3;
        *var_qd_dn4_slot = var_qd_dn4;
        *var_qd_dn5_slot = var_qd_dn5;
        *var_qd_dn6_slot = var_qd_dn6;
        *var_qd_dn7_slot = var_qd_dn7;
        *var_qd_dn8_slot = var_qd_dn8;
        *var_qfg_slot = var_qfg;
        *var_qfg_dn3_slot = var_qfg_dn3;
        *var_qfg_dn4_slot = var_qfg_dn4;
        *var_qfg_dn5_slot = var_qfg_dn5;
        *var_qfg_dn6_slot = var_qfg_dn6;
        *var_qfg_dn7_slot = var_qfg_dn7;
        *var_qfg_dn8_slot = var_qfg_dn8;
        *var_qs_slot = var_qs;
        *var_qs_dn3_slot = var_qs_dn3;
        *var_qs_dn4_slot = var_qs_dn4;
        *var_qs_dn5_slot = var_qs_dn5;
        *var_qs_dn6_slot = var_qs_dn6;
        *var_qs_dn7_slot = var_qs_dn7;
        *var_qs_dn8_slot = var_qs_dn8;
        *var_rdrain_slot = var_rdrain;
        *var_rdrain_dn3_slot = var_rdrain_dn3;
        *var_rdrain_dn4_slot = var_rdrain_dn4;
        *var_rdrain_dn5_slot = var_rdrain_dn5;
        *var_rdrain_dn6_slot = var_rdrain_dn6;
        *var_rdrain_dn7_slot = var_rdrain_dn7;
        *var_rdrain_dn8_slot = var_rdrain_dn8;
        *var_rdsi_slot = var_rdsi;
        *var_rdsi_dn3_slot = var_rdsi_dn3;
        *var_rdsi_dn4_slot = var_rdsi_dn4;
        *var_rdsi_dn5_slot = var_rdsi_dn5;
        *var_rdsi_dn6_slot = var_rdsi_dn6;
        *var_rdsi_dn7_slot = var_rdsi_dn7;
        *var_rdsi_dn8_slot = var_rdsi_dn8;
        *var_rsource_slot = var_rsource;
        *var_rsource_dn3_slot = var_rsource_dn3;
        *var_rsource_dn4_slot = var_rsource_dn4;
        *var_rsource_dn5_slot = var_rsource_dn5;
        *var_rsource_dn6_slot = var_rsource_dn6;
        *var_rsource_dn7_slot = var_rsource_dn7;
        *var_rsource_dn8_slot = var_rsource_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1__blk110_slot = var_t1__blk110;
        *var_t1__blk110_dn3_slot = var_t1__blk110_dn3;
        *var_t1__blk110_dn4_slot = var_t1__blk110_dn4;
        *var_t1__blk110_dn5_slot = var_t1__blk110_dn5;
        *var_t1__blk110_dn6_slot = var_t1__blk110_dn6;
        *var_t1__blk110_dn7_slot = var_t1__blk110_dn7;
        *var_t1__blk110_dn8_slot = var_t1__blk110_dn8;
        *var_t4_slot = var_t4;
        *var_t4__blk111_slot = var_t4__blk111;
        *var_t4__blk111_dn3_slot = var_t4__blk111_dn3;
        *var_t4__blk111_dn4_slot = var_t4__blk111_dn4;
        *var_t4__blk111_dn5_slot = var_t4__blk111_dn5;
        *var_t4__blk111_dn6_slot = var_t4__blk111_dn6;
        *var_t4__blk111_dn7_slot = var_t4__blk111_dn7;
        *var_t4__blk111_dn8_slot = var_t4__blk111_dn8;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t5_slot = var_t5;
        *var_t5_dn3_slot = var_t5_dn3;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_tcen_slot = var_tcen;
        *var_tcen0_slot = var_tcen0;
        *var_tcen_dn3_slot = var_tcen_dn3;
        *var_tcen_dn4_slot = var_tcen_dn4;
        *var_tcen_dn5_slot = var_tcen_dn5;
        *var_tcen_dn6_slot = var_tcen_dn6;
        *var_tcen_dn7_slot = var_tcen_dn7;
        *var_tcen_dn8_slot = var_tcen_dn8;
    }

    pub(super) fn stamp_transient_block_25(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_aigbinv_i: f64,
        var_alpha0_i: f64,
        var_alpha1_i: f64,
        var_beta0_t: f64,
        var_beta0_t_dn4: f64,
        var_bigbinv_i: f64,
        var_cdbox: f64,
        var_cdbox_dn3: f64,
        var_cdbox_dn4: f64,
        var_cdbox_dn5: f64,
        var_cdbox_dn6: f64,
        var_cdbox_dn7: f64,
        var_cdbox_dn8: f64,
        var_cfd_i: f64,
        var_cfs_i: f64,
        var_cigbinv_i: f64,
        var_cox1: f64,
        var_cox1p: f64,
        var_csbox: f64,
        var_csbox_dn3: f64,
        var_csbox_dn4: f64,
        var_csbox_dn5: f64,
        var_csbox_dn6: f64,
        var_csbox_dn7: f64,
        var_csbox_dn8: f64,
        var_devsign: f64,
        var_diffvds: f64,
        var_diffvds_dn3: f64,
        var_diffvds_dn4: f64,
        var_diffvds_dn5: f64,
        var_diffvds_dn6: f64,
        var_diffvds_dn7: f64,
        var_diffvds_dn8: f64,
        var_eigbinv_i: f64,
        var_epsratio: f64,
        var_guard118: f64,
        var_ids: f64,
        var_ids_dn3: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_ids_dn8: f64,
        var_imgtoxp: f64,
        var_leff: f64,
        var_leffcv: f64,
        var_lovd_i: f64,
        var_lovs_i: f64,
        var_mclmcv: f64,
        var_mclmcv_dn3: f64,
        var_mclmcv_dn4: f64,
        var_mclmcv_dn5: f64,
        var_mclmcv_dn6: f64,
        var_mclmcv_dn7: f64,
        var_mclmcv_dn8: f64,
        var_nigbinv_i: f64,
        var_phig2_i: f64,
        var_phig2_i_dn3: f64,
        var_phig2_i_dn4: f64,
        var_phig2_i_dn5: f64,
        var_phig2_i_dn6: f64,
        var_phig2_i_dn7: f64,
        var_phig2_i_dn8: f64,
        var_phisd: f64,
        var_phisd_dn3: f64,
        var_phisd_dn4: f64,
        var_phisd_dn5: f64,
        var_phisd_dn6: f64,
        var_phisd_dn7: f64,
        var_phisd_dn8: f64,
        var_qia: f64,
        var_qia_dn3: f64,
        var_qia_dn4: f64,
        var_qia_dn5: f64,
        var_qia_dn6: f64,
        var_qia_dn7: f64,
        var_qia_dn8: f64,
        var_qmtcencv_i: f64,
        var_tcen: f64,
        var_tcen_dn3: f64,
        var_tcen_dn4: f64,
        var_tcen_dn5: f64,
        var_tcen_dn6: f64,
        var_tcen_dn7: f64,
        var_tcen_dn8: f64,
        var_vbgd_noswap: f64,
        var_vbgd_noswap_dn3: f64,
        var_vbgd_noswap_dn5: f64,
        var_vbgs_noswap: f64,
        var_vbgs_noswap_dn3: f64,
        var_vbgs_noswap_dn6: f64,
        var_vfbsd: f64,
        var_vfbsd_dn3: f64,
        var_vfbsd_dn4: f64,
        var_vfbsd_dn5: f64,
        var_vfbsd_dn6: f64,
        var_vfbsd_dn7: f64,
        var_vfbsd_dn8: f64,
        var_vgd_ov_noswap: f64,
        var_vgd_ov_noswap_dn5: f64,
        var_vgd_ov_noswap_dn7: f64,
        var_vgs_ov_noswap: f64,
        var_vgs_ov_noswap_dn6: f64,
        var_vgs_ov_noswap_dn7: f64,
        var_vtm: f64,
        var_vtm_dn4: f64,
        var_weffcv: f64,
        var_coxeff_slot: &mut f64,
        var_coxeff_dn3_slot: &mut f64,
        var_coxeff_dn4_slot: &mut f64,
        var_coxeff_dn5_slot: &mut f64,
        var_coxeff_dn6_slot: &mut f64,
        var_coxeff_dn7_slot: &mut f64,
        var_coxeff_dn8_slot: &mut f64,
        var_guard119_slot: &mut f64,
        var_guard120_slot: &mut f64,
        var_guard121_slot: &mut f64,
        var_igbacc_slot: &mut f64,
        var_igbacc_dn3_slot: &mut f64,
        var_igbacc_dn4_slot: &mut f64,
        var_igbacc_dn5_slot: &mut f64,
        var_igbacc_dn6_slot: &mut f64,
        var_igbacc_dn7_slot: &mut f64,
        var_igbacc_dn8_slot: &mut f64,
        var_igbinv_slot: &mut f64,
        var_igbinv_dn3_slot: &mut f64,
        var_igbinv_dn4_slot: &mut f64,
        var_igbinv_dn5_slot: &mut f64,
        var_igbinv_dn6_slot: &mut f64,
        var_igbinv_dn7_slot: &mut f64,
        var_igbinv_dn8_slot: &mut f64,
        var_igcd_slot: &mut f64,
        var_igcd_dn3_slot: &mut f64,
        var_igcd_dn4_slot: &mut f64,
        var_igcd_dn5_slot: &mut f64,
        var_igcd_dn6_slot: &mut f64,
        var_igcd_dn7_slot: &mut f64,
        var_igcd_dn8_slot: &mut f64,
        var_igcs_slot: &mut f64,
        var_igcs_dn3_slot: &mut f64,
        var_igcs_dn4_slot: &mut f64,
        var_igcs_dn5_slot: &mut f64,
        var_igcs_dn6_slot: &mut f64,
        var_igcs_dn7_slot: &mut f64,
        var_igcs_dn8_slot: &mut f64,
        var_igd_slot: &mut f64,
        var_igd_dn3_slot: &mut f64,
        var_igd_dn4_slot: &mut f64,
        var_igd_dn5_slot: &mut f64,
        var_igd_dn6_slot: &mut f64,
        var_igd_dn7_slot: &mut f64,
        var_igd_dn8_slot: &mut f64,
        var_igs_slot: &mut f64,
        var_igs_dn3_slot: &mut f64,
        var_igs_dn4_slot: &mut f64,
        var_igs_dn5_slot: &mut f64,
        var_igs_dn6_slot: &mut f64,
        var_igs_dn7_slot: &mut f64,
        var_igs_dn8_slot: &mut f64,
        var_iii_slot: &mut f64,
        var_iii_dn3_slot: &mut f64,
        var_iii_dn4_slot: &mut f64,
        var_iii_dn5_slot: &mut f64,
        var_iii_dn6_slot: &mut f64,
        var_iii_dn7_slot: &mut f64,
        var_iii_dn8_slot: &mut f64,
        var_qbg_slot: &mut f64,
        var_qbg_dn3_slot: &mut f64,
        var_qbg_dn4_slot: &mut f64,
        var_qbg_dn5_slot: &mut f64,
        var_qbg_dn6_slot: &mut f64,
        var_qbg_dn7_slot: &mut f64,
        var_qbg_dn8_slot: &mut f64,
        var_qd_slot: &mut f64,
        var_qd_dn3_slot: &mut f64,
        var_qd_dn4_slot: &mut f64,
        var_qd_dn5_slot: &mut f64,
        var_qd_dn6_slot: &mut f64,
        var_qd_dn7_slot: &mut f64,
        var_qd_dn8_slot: &mut f64,
        var_qdbg_slot: &mut f64,
        var_qdbg_dn3_slot: &mut f64,
        var_qdbg_dn4_slot: &mut f64,
        var_qdbg_dn5_slot: &mut f64,
        var_qdbg_dn6_slot: &mut f64,
        var_qdbg_dn7_slot: &mut f64,
        var_qdbg_dn8_slot: &mut f64,
        var_qfg_slot: &mut f64,
        var_qfg_dn3_slot: &mut f64,
        var_qfg_dn4_slot: &mut f64,
        var_qfg_dn5_slot: &mut f64,
        var_qfg_dn6_slot: &mut f64,
        var_qfg_dn7_slot: &mut f64,
        var_qfg_dn8_slot: &mut f64,
        var_qfgd_of_slot: &mut f64,
        var_qfgd_of_dn5_slot: &mut f64,
        var_qfgd_of_dn7_slot: &mut f64,
        var_qfgd_ov_slot: &mut f64,
        var_qfgd_ov_dn3_slot: &mut f64,
        var_qfgd_ov_dn4_slot: &mut f64,
        var_qfgd_ov_dn5_slot: &mut f64,
        var_qfgd_ov_dn6_slot: &mut f64,
        var_qfgd_ov_dn7_slot: &mut f64,
        var_qfgd_ov_dn8_slot: &mut f64,
        var_qfgd_parasitic_slot: &mut f64,
        var_qfgd_parasitic_dn3_slot: &mut f64,
        var_qfgd_parasitic_dn4_slot: &mut f64,
        var_qfgd_parasitic_dn5_slot: &mut f64,
        var_qfgd_parasitic_dn6_slot: &mut f64,
        var_qfgd_parasitic_dn7_slot: &mut f64,
        var_qfgd_parasitic_dn8_slot: &mut f64,
        var_qfgs_of_slot: &mut f64,
        var_qfgs_of_dn6_slot: &mut f64,
        var_qfgs_of_dn7_slot: &mut f64,
        var_qfgs_ov_slot: &mut f64,
        var_qfgs_ov_dn3_slot: &mut f64,
        var_qfgs_ov_dn4_slot: &mut f64,
        var_qfgs_ov_dn5_slot: &mut f64,
        var_qfgs_ov_dn6_slot: &mut f64,
        var_qfgs_ov_dn7_slot: &mut f64,
        var_qfgs_ov_dn8_slot: &mut f64,
        var_qfgs_parasitic_slot: &mut f64,
        var_qfgs_parasitic_dn3_slot: &mut f64,
        var_qfgs_parasitic_dn4_slot: &mut f64,
        var_qfgs_parasitic_dn5_slot: &mut f64,
        var_qfgs_parasitic_dn6_slot: &mut f64,
        var_qfgs_parasitic_dn7_slot: &mut f64,
        var_qfgs_parasitic_dn8_slot: &mut f64,
        var_qs_slot: &mut f64,
        var_qs_dn3_slot: &mut f64,
        var_qs_dn4_slot: &mut f64,
        var_qs_dn5_slot: &mut f64,
        var_qs_dn6_slot: &mut f64,
        var_qs_dn7_slot: &mut f64,
        var_qs_dn8_slot: &mut f64,
        var_qsbg_slot: &mut f64,
        var_qsbg_dn3_slot: &mut f64,
        var_qsbg_dn4_slot: &mut f64,
        var_qsbg_dn5_slot: &mut f64,
        var_qsbg_dn6_slot: &mut f64,
        var_qsbg_dn7_slot: &mut f64,
        var_qsbg_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_vaux_igbinv_slot: &mut f64,
        var_vaux_igbinv_dn3_slot: &mut f64,
        var_vaux_igbinv_dn4_slot: &mut f64,
        var_vaux_igbinv_dn5_slot: &mut f64,
        var_vaux_igbinv_dn6_slot: &mut f64,
        var_vaux_igbinv_dn7_slot: &mut f64,
        var_vaux_igbinv_dn8_slot: &mut f64,
        var_vfbsd_bg_slot: &mut f64,
        var_vfbsd_bg_dn3_slot: &mut f64,
        var_vfbsd_bg_dn4_slot: &mut f64,
        var_vfbsd_bg_dn5_slot: &mut f64,
        var_vfbsd_bg_dn6_slot: &mut f64,
        var_vfbsd_bg_dn7_slot: &mut f64,
        var_vfbsd_bg_dn8_slot: &mut f64,
        var_vfgd_ov_slot: &mut f64,
        var_vfgd_ov_dn3_slot: &mut f64,
        var_vfgd_ov_dn4_slot: &mut f64,
        var_vfgd_ov_dn5_slot: &mut f64,
        var_vfgd_ov_dn6_slot: &mut f64,
        var_vfgd_ov_dn7_slot: &mut f64,
        var_vfgd_ov_dn8_slot: &mut f64,
        var_vfgs_ov_slot: &mut f64,
        var_vfgs_ov_dn3_slot: &mut f64,
        var_vfgs_ov_dn4_slot: &mut f64,
        var_vfgs_ov_dn5_slot: &mut f64,
        var_vfgs_ov_dn6_slot: &mut f64,
        var_vfgs_ov_dn7_slot: &mut f64,
        var_vfgs_ov_dn8_slot: &mut f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let mut var_coxeff: f64 = *var_coxeff_slot;
        let mut var_coxeff_dn3: f64 = *var_coxeff_dn3_slot;
        let mut var_coxeff_dn4: f64 = *var_coxeff_dn4_slot;
        let mut var_coxeff_dn5: f64 = *var_coxeff_dn5_slot;
        let mut var_coxeff_dn6: f64 = *var_coxeff_dn6_slot;
        let mut var_coxeff_dn7: f64 = *var_coxeff_dn7_slot;
        let mut var_coxeff_dn8: f64 = *var_coxeff_dn8_slot;
        let mut var_guard119: f64 = *var_guard119_slot;
        let mut var_guard120: f64 = *var_guard120_slot;
        let mut var_guard121: f64 = *var_guard121_slot;
        let mut var_igbacc: f64 = *var_igbacc_slot;
        let mut var_igbacc_dn3: f64 = *var_igbacc_dn3_slot;
        let mut var_igbacc_dn4: f64 = *var_igbacc_dn4_slot;
        let mut var_igbacc_dn5: f64 = *var_igbacc_dn5_slot;
        let mut var_igbacc_dn6: f64 = *var_igbacc_dn6_slot;
        let mut var_igbacc_dn7: f64 = *var_igbacc_dn7_slot;
        let mut var_igbacc_dn8: f64 = *var_igbacc_dn8_slot;
        let mut var_igbinv: f64 = *var_igbinv_slot;
        let mut var_igbinv_dn3: f64 = *var_igbinv_dn3_slot;
        let mut var_igbinv_dn4: f64 = *var_igbinv_dn4_slot;
        let mut var_igbinv_dn5: f64 = *var_igbinv_dn5_slot;
        let mut var_igbinv_dn6: f64 = *var_igbinv_dn6_slot;
        let mut var_igbinv_dn7: f64 = *var_igbinv_dn7_slot;
        let mut var_igbinv_dn8: f64 = *var_igbinv_dn8_slot;
        let mut var_igcd: f64 = *var_igcd_slot;
        let mut var_igcd_dn3: f64 = *var_igcd_dn3_slot;
        let mut var_igcd_dn4: f64 = *var_igcd_dn4_slot;
        let mut var_igcd_dn5: f64 = *var_igcd_dn5_slot;
        let mut var_igcd_dn6: f64 = *var_igcd_dn6_slot;
        let mut var_igcd_dn7: f64 = *var_igcd_dn7_slot;
        let mut var_igcd_dn8: f64 = *var_igcd_dn8_slot;
        let mut var_igcs: f64 = *var_igcs_slot;
        let mut var_igcs_dn3: f64 = *var_igcs_dn3_slot;
        let mut var_igcs_dn4: f64 = *var_igcs_dn4_slot;
        let mut var_igcs_dn5: f64 = *var_igcs_dn5_slot;
        let mut var_igcs_dn6: f64 = *var_igcs_dn6_slot;
        let mut var_igcs_dn7: f64 = *var_igcs_dn7_slot;
        let mut var_igcs_dn8: f64 = *var_igcs_dn8_slot;
        let mut var_igd: f64 = *var_igd_slot;
        let mut var_igd_dn3: f64 = *var_igd_dn3_slot;
        let mut var_igd_dn4: f64 = *var_igd_dn4_slot;
        let mut var_igd_dn5: f64 = *var_igd_dn5_slot;
        let mut var_igd_dn6: f64 = *var_igd_dn6_slot;
        let mut var_igd_dn7: f64 = *var_igd_dn7_slot;
        let mut var_igd_dn8: f64 = *var_igd_dn8_slot;
        let mut var_igs: f64 = *var_igs_slot;
        let mut var_igs_dn3: f64 = *var_igs_dn3_slot;
        let mut var_igs_dn4: f64 = *var_igs_dn4_slot;
        let mut var_igs_dn5: f64 = *var_igs_dn5_slot;
        let mut var_igs_dn6: f64 = *var_igs_dn6_slot;
        let mut var_igs_dn7: f64 = *var_igs_dn7_slot;
        let mut var_igs_dn8: f64 = *var_igs_dn8_slot;
        let mut var_iii: f64 = *var_iii_slot;
        let mut var_iii_dn3: f64 = *var_iii_dn3_slot;
        let mut var_iii_dn4: f64 = *var_iii_dn4_slot;
        let mut var_iii_dn5: f64 = *var_iii_dn5_slot;
        let mut var_iii_dn6: f64 = *var_iii_dn6_slot;
        let mut var_iii_dn7: f64 = *var_iii_dn7_slot;
        let mut var_iii_dn8: f64 = *var_iii_dn8_slot;
        let mut var_qbg: f64 = *var_qbg_slot;
        let mut var_qbg_dn3: f64 = *var_qbg_dn3_slot;
        let mut var_qbg_dn4: f64 = *var_qbg_dn4_slot;
        let mut var_qbg_dn5: f64 = *var_qbg_dn5_slot;
        let mut var_qbg_dn6: f64 = *var_qbg_dn6_slot;
        let mut var_qbg_dn7: f64 = *var_qbg_dn7_slot;
        let mut var_qbg_dn8: f64 = *var_qbg_dn8_slot;
        let mut var_qd: f64 = *var_qd_slot;
        let mut var_qd_dn3: f64 = *var_qd_dn3_slot;
        let mut var_qd_dn4: f64 = *var_qd_dn4_slot;
        let mut var_qd_dn5: f64 = *var_qd_dn5_slot;
        let mut var_qd_dn6: f64 = *var_qd_dn6_slot;
        let mut var_qd_dn7: f64 = *var_qd_dn7_slot;
        let mut var_qd_dn8: f64 = *var_qd_dn8_slot;
        let mut var_qdbg: f64 = *var_qdbg_slot;
        let mut var_qdbg_dn3: f64 = *var_qdbg_dn3_slot;
        let mut var_qdbg_dn4: f64 = *var_qdbg_dn4_slot;
        let mut var_qdbg_dn5: f64 = *var_qdbg_dn5_slot;
        let mut var_qdbg_dn6: f64 = *var_qdbg_dn6_slot;
        let mut var_qdbg_dn7: f64 = *var_qdbg_dn7_slot;
        let mut var_qdbg_dn8: f64 = *var_qdbg_dn8_slot;
        let mut var_qfg: f64 = *var_qfg_slot;
        let mut var_qfg_dn3: f64 = *var_qfg_dn3_slot;
        let mut var_qfg_dn4: f64 = *var_qfg_dn4_slot;
        let mut var_qfg_dn5: f64 = *var_qfg_dn5_slot;
        let mut var_qfg_dn6: f64 = *var_qfg_dn6_slot;
        let mut var_qfg_dn7: f64 = *var_qfg_dn7_slot;
        let mut var_qfg_dn8: f64 = *var_qfg_dn8_slot;
        let mut var_qfgd_of: f64 = *var_qfgd_of_slot;
        let mut var_qfgd_of_dn5: f64 = *var_qfgd_of_dn5_slot;
        let mut var_qfgd_of_dn7: f64 = *var_qfgd_of_dn7_slot;
        let mut var_qfgd_ov: f64 = *var_qfgd_ov_slot;
        let mut var_qfgd_ov_dn3: f64 = *var_qfgd_ov_dn3_slot;
        let mut var_qfgd_ov_dn4: f64 = *var_qfgd_ov_dn4_slot;
        let mut var_qfgd_ov_dn5: f64 = *var_qfgd_ov_dn5_slot;
        let mut var_qfgd_ov_dn6: f64 = *var_qfgd_ov_dn6_slot;
        let mut var_qfgd_ov_dn7: f64 = *var_qfgd_ov_dn7_slot;
        let mut var_qfgd_ov_dn8: f64 = *var_qfgd_ov_dn8_slot;
        let mut var_qfgd_parasitic: f64 = *var_qfgd_parasitic_slot;
        let mut var_qfgd_parasitic_dn3: f64 = *var_qfgd_parasitic_dn3_slot;
        let mut var_qfgd_parasitic_dn4: f64 = *var_qfgd_parasitic_dn4_slot;
        let mut var_qfgd_parasitic_dn5: f64 = *var_qfgd_parasitic_dn5_slot;
        let mut var_qfgd_parasitic_dn6: f64 = *var_qfgd_parasitic_dn6_slot;
        let mut var_qfgd_parasitic_dn7: f64 = *var_qfgd_parasitic_dn7_slot;
        let mut var_qfgd_parasitic_dn8: f64 = *var_qfgd_parasitic_dn8_slot;
        let mut var_qfgs_of: f64 = *var_qfgs_of_slot;
        let mut var_qfgs_of_dn6: f64 = *var_qfgs_of_dn6_slot;
        let mut var_qfgs_of_dn7: f64 = *var_qfgs_of_dn7_slot;
        let mut var_qfgs_ov: f64 = *var_qfgs_ov_slot;
        let mut var_qfgs_ov_dn3: f64 = *var_qfgs_ov_dn3_slot;
        let mut var_qfgs_ov_dn4: f64 = *var_qfgs_ov_dn4_slot;
        let mut var_qfgs_ov_dn5: f64 = *var_qfgs_ov_dn5_slot;
        let mut var_qfgs_ov_dn6: f64 = *var_qfgs_ov_dn6_slot;
        let mut var_qfgs_ov_dn7: f64 = *var_qfgs_ov_dn7_slot;
        let mut var_qfgs_ov_dn8: f64 = *var_qfgs_ov_dn8_slot;
        let mut var_qfgs_parasitic: f64 = *var_qfgs_parasitic_slot;
        let mut var_qfgs_parasitic_dn3: f64 = *var_qfgs_parasitic_dn3_slot;
        let mut var_qfgs_parasitic_dn4: f64 = *var_qfgs_parasitic_dn4_slot;
        let mut var_qfgs_parasitic_dn5: f64 = *var_qfgs_parasitic_dn5_slot;
        let mut var_qfgs_parasitic_dn6: f64 = *var_qfgs_parasitic_dn6_slot;
        let mut var_qfgs_parasitic_dn7: f64 = *var_qfgs_parasitic_dn7_slot;
        let mut var_qfgs_parasitic_dn8: f64 = *var_qfgs_parasitic_dn8_slot;
        let mut var_qs: f64 = *var_qs_slot;
        let mut var_qs_dn3: f64 = *var_qs_dn3_slot;
        let mut var_qs_dn4: f64 = *var_qs_dn4_slot;
        let mut var_qs_dn5: f64 = *var_qs_dn5_slot;
        let mut var_qs_dn6: f64 = *var_qs_dn6_slot;
        let mut var_qs_dn7: f64 = *var_qs_dn7_slot;
        let mut var_qs_dn8: f64 = *var_qs_dn8_slot;
        let mut var_qsbg: f64 = *var_qsbg_slot;
        let mut var_qsbg_dn3: f64 = *var_qsbg_dn3_slot;
        let mut var_qsbg_dn4: f64 = *var_qsbg_dn4_slot;
        let mut var_qsbg_dn5: f64 = *var_qsbg_dn5_slot;
        let mut var_qsbg_dn6: f64 = *var_qsbg_dn6_slot;
        let mut var_qsbg_dn7: f64 = *var_qsbg_dn7_slot;
        let mut var_qsbg_dn8: f64 = *var_qsbg_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_vaux_igbinv: f64 = *var_vaux_igbinv_slot;
        let mut var_vaux_igbinv_dn3: f64 = *var_vaux_igbinv_dn3_slot;
        let mut var_vaux_igbinv_dn4: f64 = *var_vaux_igbinv_dn4_slot;
        let mut var_vaux_igbinv_dn5: f64 = *var_vaux_igbinv_dn5_slot;
        let mut var_vaux_igbinv_dn6: f64 = *var_vaux_igbinv_dn6_slot;
        let mut var_vaux_igbinv_dn7: f64 = *var_vaux_igbinv_dn7_slot;
        let mut var_vaux_igbinv_dn8: f64 = *var_vaux_igbinv_dn8_slot;
        let mut var_vfbsd_bg: f64 = *var_vfbsd_bg_slot;
        let mut var_vfbsd_bg_dn3: f64 = *var_vfbsd_bg_dn3_slot;
        let mut var_vfbsd_bg_dn4: f64 = *var_vfbsd_bg_dn4_slot;
        let mut var_vfbsd_bg_dn5: f64 = *var_vfbsd_bg_dn5_slot;
        let mut var_vfbsd_bg_dn6: f64 = *var_vfbsd_bg_dn6_slot;
        let mut var_vfbsd_bg_dn7: f64 = *var_vfbsd_bg_dn7_slot;
        let mut var_vfbsd_bg_dn8: f64 = *var_vfbsd_bg_dn8_slot;
        let mut var_vfgd_ov: f64 = *var_vfgd_ov_slot;
        let mut var_vfgd_ov_dn3: f64 = *var_vfgd_ov_dn3_slot;
        let mut var_vfgd_ov_dn4: f64 = *var_vfgd_ov_dn4_slot;
        let mut var_vfgd_ov_dn5: f64 = *var_vfgd_ov_dn5_slot;
        let mut var_vfgd_ov_dn6: f64 = *var_vfgd_ov_dn6_slot;
        let mut var_vfgd_ov_dn7: f64 = *var_vfgd_ov_dn7_slot;
        let mut var_vfgd_ov_dn8: f64 = *var_vfgd_ov_dn8_slot;
        let mut var_vfgs_ov: f64 = *var_vfgs_ov_slot;
        let mut var_vfgs_ov_dn3: f64 = *var_vfgs_ov_dn3_slot;
        let mut var_vfgs_ov_dn4: f64 = *var_vfgs_ov_dn4_slot;
        let mut var_vfgs_ov_dn5: f64 = *var_vfgs_ov_dn5_slot;
        let mut var_vfgs_ov_dn6: f64 = *var_vfgs_ov_dn6_slot;
        let mut var_vfgs_ov_dn7: f64 = *var_vfgs_ov_dn7_slot;
        let mut var_vfgs_ov_dn8: f64 = *var_vfgs_ov_dn8_slot;

        let (assign11450_e10716, assign11450_e10716_d_n3, assign11450_e10716_d_n4, assign11450_e10716_d_n5, assign11450_e10716_d_n6, assign11450_e10716_d_n7, assign11450_e10716_d_n8,) = {
    if (var_guard118 != 0.0) {
        let assign11450_e10702: f64 = (3.9 * 8.85418e-12);
        let assign11450_e10705: f64 = (var_imgtoxp * 3.9);
        let assign11450_e10707: f64 = (assign11450_e10705 / p.p60);
        let assign11450_e10710: f64 = (var_tcen * var_qmtcencv_i);
        let assign11450_e10712: f64 = (assign11450_e10710 / var_epsratio);
        let assign11450_e10713: f64 = (assign11450_e10707 + assign11450_e10712);
        let assign11450_e10714: f64 = (assign11450_e10702 / assign11450_e10713);
        (assign11450_e10714, (-((assign11450_e10702 * ((var_tcen_dn3 * var_qmtcencv_i) / var_epsratio)) / (assign11450_e10713 * assign11450_e10713))), (-((assign11450_e10702 * ((var_tcen_dn4 * var_qmtcencv_i) / var_epsratio)) / (assign11450_e10713 * assign11450_e10713))), (-((assign11450_e10702 * ((var_tcen_dn5 * var_qmtcencv_i) / var_epsratio)) / (assign11450_e10713 * assign11450_e10713))), (-((assign11450_e10702 * ((var_tcen_dn6 * var_qmtcencv_i) / var_epsratio)) / (assign11450_e10713 * assign11450_e10713))), (-((assign11450_e10702 * ((var_tcen_dn7 * var_qmtcencv_i) / var_epsratio)) / (assign11450_e10713 * assign11450_e10713))), (-((assign11450_e10702 * ((var_tcen_dn8 * var_qmtcencv_i) / var_epsratio)) / (assign11450_e10713 * assign11450_e10713))),)
    } else {
        (var_coxeff, var_coxeff_dn3, var_coxeff_dn4, var_coxeff_dn5, var_coxeff_dn6, var_coxeff_dn7, var_coxeff_dn8,)
    }
};
        var_coxeff = assign11450_e10716;
        var_coxeff_dn3 = assign11450_e10716_d_n3;
        var_coxeff_dn4 = assign11450_e10716_d_n4;
        var_coxeff_dn5 = assign11450_e10716_d_n5;
        var_coxeff_dn6 = assign11450_e10716_d_n6;
        var_coxeff_dn7 = assign11450_e10716_d_n7;
        var_coxeff_dn8 = assign11450_e10716_d_n8;

        let (assign11460_e10721, assign11460_e10721_d_n3, assign11460_e10721_d_n4, assign11460_e10721_d_n5, assign11460_e10721_d_n6, assign11460_e10721_d_n7, assign11460_e10721_d_n8,) = {
    if (var_guard118 == 0.0) {
        (var_cox1p, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_coxeff, var_coxeff_dn3, var_coxeff_dn4, var_coxeff_dn5, var_coxeff_dn6, var_coxeff_dn7, var_coxeff_dn8,)
    }
};
        var_coxeff = assign11460_e10721;
        var_coxeff_dn3 = assign11460_e10721_d_n3;
        var_coxeff_dn4 = assign11460_e10721_d_n4;
        var_coxeff_dn5 = assign11460_e10721_d_n5;
        var_coxeff_dn6 = assign11460_e10721_d_n6;
        var_coxeff_dn7 = assign11460_e10721_d_n7;
        var_coxeff_dn8 = assign11460_e10721_d_n8;

        let assign11470_e10724: f64 = (var_weffcv * var_leffcv);
        let assign11470_e10726: f64 = (assign11470_e10724 / var_mclmcv);
        var_t0 = assign11470_e10726;
        var_t0_dn3 = (-((assign11470_e10724 * var_mclmcv_dn3) / (var_mclmcv * var_mclmcv)));
        var_t0_dn4 = (-((assign11470_e10724 * var_mclmcv_dn4) / (var_mclmcv * var_mclmcv)));
        var_t0_dn5 = (-((assign11470_e10724 * var_mclmcv_dn5) / (var_mclmcv * var_mclmcv)));
        var_t0_dn6 = (-((assign11470_e10724 * var_mclmcv_dn6) / (var_mclmcv * var_mclmcv)));
        var_t0_dn7 = (-((assign11470_e10724 * var_mclmcv_dn7) / (var_mclmcv * var_mclmcv)));
        var_t0_dn8 = (-((assign11470_e10724 * var_mclmcv_dn8) / (var_mclmcv * var_mclmcv)));

        let assign11480_e10729: f64 = (var_qfg * var_t0);
        var_qfg = assign11480_e10729;
        var_qfg_dn3 = ((var_qfg_dn3 * var_t0) + (var_qfg * var_t0_dn3));
        var_qfg_dn4 = ((var_qfg_dn4 * var_t0) + (var_qfg * var_t0_dn4));
        var_qfg_dn5 = ((var_qfg_dn5 * var_t0) + (var_qfg * var_t0_dn5));
        var_qfg_dn6 = ((var_qfg_dn6 * var_t0) + (var_qfg * var_t0_dn6));
        var_qfg_dn7 = ((var_qfg_dn7 * var_t0) + (var_qfg * var_t0_dn7));
        var_qfg_dn8 = ((var_qfg_dn8 * var_t0) + (var_qfg * var_t0_dn8));

        let assign11490_e10731: f64 = (-var_qd);
        let assign11490_e10733: f64 = (assign11490_e10731 * var_t0);
        var_qd = assign11490_e10733;
        var_qd_dn3 = (((-var_qd_dn3) * var_t0) + (assign11490_e10731 * var_t0_dn3));
        var_qd_dn4 = (((-var_qd_dn4) * var_t0) + (assign11490_e10731 * var_t0_dn4));
        var_qd_dn5 = (((-var_qd_dn5) * var_t0) + (assign11490_e10731 * var_t0_dn5));
        var_qd_dn6 = (((-var_qd_dn6) * var_t0) + (assign11490_e10731 * var_t0_dn6));
        var_qd_dn7 = (((-var_qd_dn7) * var_t0) + (assign11490_e10731 * var_t0_dn7));
        var_qd_dn8 = (((-var_qd_dn8) * var_t0) + (assign11490_e10731 * var_t0_dn8));

        let assign11500_e10736: f64 = (var_qbg * var_t0);
        var_qbg = assign11500_e10736;
        var_qbg_dn3 = ((var_qbg_dn3 * var_t0) + (var_qbg * var_t0_dn3));
        var_qbg_dn4 = ((var_qbg_dn4 * var_t0) + (var_qbg * var_t0_dn4));
        var_qbg_dn5 = ((var_qbg_dn5 * var_t0) + (var_qbg * var_t0_dn5));
        var_qbg_dn6 = ((var_qbg_dn6 * var_t0) + (var_qbg * var_t0_dn6));
        var_qbg_dn7 = ((var_qbg_dn7 * var_t0) + (var_qbg * var_t0_dn7));
        var_qbg_dn8 = ((var_qbg_dn8 * var_t0) + (var_qbg * var_t0_dn8));

        let assign11510_e10738: f64 = (-var_qs);
        let assign11510_e10740: f64 = (assign11510_e10738 * var_t0);
        var_qs = assign11510_e10740;
        var_qs_dn3 = (((-var_qs_dn3) * var_t0) + (assign11510_e10738 * var_t0_dn3));
        var_qs_dn4 = (((-var_qs_dn4) * var_t0) + (assign11510_e10738 * var_t0_dn4));
        var_qs_dn5 = (((-var_qs_dn5) * var_t0) + (assign11510_e10738 * var_t0_dn5));
        var_qs_dn6 = (((-var_qs_dn6) * var_t0) + (assign11510_e10738 * var_t0_dn6));
        var_qs_dn7 = (((-var_qs_dn7) * var_t0) + (assign11510_e10738 * var_t0_dn7));
        var_qs_dn8 = (((-var_qs_dn8) * var_t0) + (assign11510_e10738 * var_t0_dn8));

        let assign11520_e10743: f64 = (var_weffcv * var_lovs_i);
        let assign11520_e10745: f64 = (assign11520_e10743 * var_cox1);
        let assign11520_e10747: f64 = (assign11520_e10745 * (nv7 - nv6));
        var_qfgs_ov = assign11520_e10747;
        var_qfgs_ov_dn3 = 0.0;
        var_qfgs_ov_dn4 = 0.0;
        var_qfgs_ov_dn5 = 0.0;
        var_qfgs_ov_dn6 = (-assign11520_e10745);
        var_qfgs_ov_dn7 = assign11520_e10745;
        var_qfgs_ov_dn8 = 0.0;

        let assign11530_e10750: f64 = (var_weffcv * var_lovd_i);
        let assign11530_e10752: f64 = (assign11530_e10750 * var_cox1);
        let assign11530_e10754: f64 = (assign11530_e10752 * (nv7 - nv5));
        var_qfgd_ov = assign11530_e10754;
        var_qfgd_ov_dn3 = 0.0;
        var_qfgd_ov_dn4 = 0.0;
        var_qfgd_ov_dn5 = (-assign11530_e10752);
        var_qfgd_ov_dn6 = 0.0;
        var_qfgd_ov_dn7 = assign11530_e10752;
        var_qfgd_ov_dn8 = 0.0;

        let assign11540_e10758: f64 = (var_phig2_i - var_phisd);
        let assign11540_e10759: f64 = (var_devsign * assign11540_e10758);
        var_vfbsd_bg = assign11540_e10759;
        var_vfbsd_bg_dn3 = (var_devsign * (var_phig2_i_dn3 - var_phisd_dn3));
        var_vfbsd_bg_dn4 = (var_devsign * (var_phig2_i_dn4 - var_phisd_dn4));
        var_vfbsd_bg_dn5 = (var_devsign * (var_phig2_i_dn5 - var_phisd_dn5));
        var_vfbsd_bg_dn6 = (var_devsign * (var_phig2_i_dn6 - var_phisd_dn6));
        var_vfbsd_bg_dn7 = (var_devsign * (var_phig2_i_dn7 - var_phisd_dn7));
        var_vfbsd_bg_dn8 = (var_devsign * (var_phig2_i_dn8 - var_phisd_dn8));

        let assign11550_e10762: f64 = (var_vgs_ov_noswap - var_vfbsd);
        let assign11550_e10764: f64 = (assign11550_e10762 + 0.02);
        let assign11550_e10767: f64 = (p.p45 / p.p46);
        let assign11550_e10770: f64 = (var_vbgs_noswap - var_vfbsd_bg);
        let assign11550_e10772: f64 = (assign11550_e10770 - p.p268);
        let assign11550_e10773: f64 = (assign11550_e10767 * assign11550_e10772);
        let assign11550_e10775: f64 = (assign11550_e10773 * p.p269);
        let assign11550_e10776: f64 = (assign11550_e10764 + assign11550_e10775);
        var_t0 = assign11550_e10776;
        var_t0_dn3 = ((-var_vfbsd_dn3) + ((assign11550_e10767 * (var_vbgs_noswap_dn3 - var_vfbsd_bg_dn3)) * p.p269));
        var_t0_dn4 = ((-var_vfbsd_dn4) + ((assign11550_e10767 * (-var_vfbsd_bg_dn4)) * p.p269));
        var_t0_dn5 = ((-var_vfbsd_dn5) + ((assign11550_e10767 * (-var_vfbsd_bg_dn5)) * p.p269));
        var_t0_dn6 = ((var_vgs_ov_noswap_dn6 - var_vfbsd_dn6) + ((assign11550_e10767 * (var_vbgs_noswap_dn6 - var_vfbsd_bg_dn6)) * p.p269));
        var_t0_dn7 = ((var_vgs_ov_noswap_dn7 - var_vfbsd_dn7) + ((assign11550_e10767 * (-var_vfbsd_bg_dn7)) * p.p269));
        var_t0_dn8 = ((-var_vfbsd_dn8) + ((assign11550_e10767 * (-var_vfbsd_bg_dn8)) * p.p269));

        let assign11560_e10781: f64 = (var_t0 * var_t0);
        let assign11560_e10784: f64 = (4.0 * 0.02);
        let assign11560_e10785: f64 = (assign11560_e10781 + assign11560_e10784);
        let assign11560_e10786: f64 = (assign11560_e10785).sqrt();
        let assign11560_e10787: f64 = (var_t0 - assign11560_e10786);
        let assign11560_e10788: f64 = (0.5 * assign11560_e10787);
        var_vfgs_ov = assign11560_e10788;
        var_vfgs_ov_dn3 = (0.5 * (var_t0_dn3 - (((var_t0_dn3 * var_t0) + (var_t0 * var_t0_dn3)) / (2.0 * assign11560_e10786))));
        var_vfgs_ov_dn4 = (0.5 * (var_t0_dn4 - (((var_t0_dn4 * var_t0) + (var_t0 * var_t0_dn4)) / (2.0 * assign11560_e10786))));
        var_vfgs_ov_dn5 = (0.5 * (var_t0_dn5 - (((var_t0_dn5 * var_t0) + (var_t0 * var_t0_dn5)) / (2.0 * assign11560_e10786))));
        var_vfgs_ov_dn6 = (0.5 * (var_t0_dn6 - (((var_t0_dn6 * var_t0) + (var_t0 * var_t0_dn6)) / (2.0 * assign11560_e10786))));
        var_vfgs_ov_dn7 = (0.5 * (var_t0_dn7 - (((var_t0_dn7 * var_t0) + (var_t0 * var_t0_dn7)) / (2.0 * assign11560_e10786))));
        var_vfgs_ov_dn8 = (0.5 * (var_t0_dn8 - (((var_t0_dn8 * var_t0) + (var_t0 * var_t0_dn8)) / (2.0 * assign11560_e10786))));

        let assign11570_e10791: f64 = (var_vgs_ov_noswap - var_vfbsd);
        let assign11570_e10793: f64 = (assign11570_e10791 - var_vfgs_ov);
        var_t1 = assign11570_e10793;
        var_t1_dn3 = ((-var_vfbsd_dn3) - var_vfgs_ov_dn3);
        var_t1_dn4 = ((-var_vfbsd_dn4) - var_vfgs_ov_dn4);
        var_t1_dn5 = ((-var_vfbsd_dn5) - var_vfgs_ov_dn5);
        var_t1_dn6 = ((var_vgs_ov_noswap_dn6 - var_vfbsd_dn6) - var_vfgs_ov_dn6);
        var_t1_dn7 = ((var_vgs_ov_noswap_dn7 - var_vfbsd_dn7) - var_vfgs_ov_dn7);
        var_t1_dn8 = ((-var_vfbsd_dn8) - var_vfgs_ov_dn8);

        let assign11580_e10797: f64 = (var_devsign * var_weffcv);
        let assign11580_e10799: f64 = (assign11580_e10797 * p.p263);
        let assign11580_e10803: f64 = (0.5 * p.p265);
        let assign11580_e10807: f64 = (4.0 * var_vfgs_ov);
        let assign11580_e10809: f64 = (assign11580_e10807 / p.p265);
        let assign11580_e10810: f64 = (1.0 - assign11580_e10809);
        let assign11580_e10811: f64 = (assign11580_e10810).sqrt();
        let assign11580_e10813: f64 = (assign11580_e10811 - 1.0);
        let assign11580_e10814: f64 = (assign11580_e10803 * assign11580_e10813);
        let assign11580_e10815: f64 = (var_t1 - assign11580_e10814);
        let assign11580_e10816: f64 = (assign11580_e10799 * assign11580_e10815);
        let assign11580_e10817: f64 = (var_qfgs_ov + assign11580_e10816);
        var_qfgs_ov = assign11580_e10817;
        var_qfgs_ov_dn3 = (var_qfgs_ov_dn3 + (assign11580_e10799 * (var_t1_dn3 - (assign11580_e10803 * ((-((4.0 * var_vfgs_ov_dn3) / p.p265)) / (2.0 * assign11580_e10811))))));
        var_qfgs_ov_dn4 = (var_qfgs_ov_dn4 + (assign11580_e10799 * (var_t1_dn4 - (assign11580_e10803 * ((-((4.0 * var_vfgs_ov_dn4) / p.p265)) / (2.0 * assign11580_e10811))))));
        var_qfgs_ov_dn5 = (var_qfgs_ov_dn5 + (assign11580_e10799 * (var_t1_dn5 - (assign11580_e10803 * ((-((4.0 * var_vfgs_ov_dn5) / p.p265)) / (2.0 * assign11580_e10811))))));
        var_qfgs_ov_dn6 = (var_qfgs_ov_dn6 + (assign11580_e10799 * (var_t1_dn6 - (assign11580_e10803 * ((-((4.0 * var_vfgs_ov_dn6) / p.p265)) / (2.0 * assign11580_e10811))))));
        var_qfgs_ov_dn7 = (var_qfgs_ov_dn7 + (assign11580_e10799 * (var_t1_dn7 - (assign11580_e10803 * ((-((4.0 * var_vfgs_ov_dn7) / p.p265)) / (2.0 * assign11580_e10811))))));
        var_qfgs_ov_dn8 = (var_qfgs_ov_dn8 + (assign11580_e10799 * (var_t1_dn8 - (assign11580_e10803 * ((-((4.0 * var_vfgs_ov_dn8) / p.p265)) / (2.0 * assign11580_e10811))))));

        let assign11590_e10820: f64 = (var_vgd_ov_noswap - var_vfbsd);
        let assign11590_e10822: f64 = (assign11590_e10820 + 0.02);
        let assign11590_e10825: f64 = (p.p45 / p.p46);
        let assign11590_e10828: f64 = (var_vbgd_noswap - var_vfbsd_bg);
        let assign11590_e10830: f64 = (assign11590_e10828 - p.p270);
        let assign11590_e10831: f64 = (assign11590_e10825 * assign11590_e10830);
        let assign11590_e10833: f64 = (assign11590_e10831 * p.p271);
        let assign11590_e10834: f64 = (assign11590_e10822 + assign11590_e10833);
        var_t0 = assign11590_e10834;
        var_t0_dn3 = ((-var_vfbsd_dn3) + ((assign11590_e10825 * (var_vbgd_noswap_dn3 - var_vfbsd_bg_dn3)) * p.p271));
        var_t0_dn4 = ((-var_vfbsd_dn4) + ((assign11590_e10825 * (-var_vfbsd_bg_dn4)) * p.p271));
        var_t0_dn5 = ((var_vgd_ov_noswap_dn5 - var_vfbsd_dn5) + ((assign11590_e10825 * (var_vbgd_noswap_dn5 - var_vfbsd_bg_dn5)) * p.p271));
        var_t0_dn6 = ((-var_vfbsd_dn6) + ((assign11590_e10825 * (-var_vfbsd_bg_dn6)) * p.p271));
        var_t0_dn7 = ((var_vgd_ov_noswap_dn7 - var_vfbsd_dn7) + ((assign11590_e10825 * (-var_vfbsd_bg_dn7)) * p.p271));
        var_t0_dn8 = ((-var_vfbsd_dn8) + ((assign11590_e10825 * (-var_vfbsd_bg_dn8)) * p.p271));

        let assign11600_e10839: f64 = (var_t0 * var_t0);
        let assign11600_e10842: f64 = (4.0 * 0.02);
        let assign11600_e10843: f64 = (assign11600_e10839 + assign11600_e10842);
        let assign11600_e10844: f64 = (assign11600_e10843).sqrt();
        let assign11600_e10845: f64 = (var_t0 - assign11600_e10844);
        let assign11600_e10846: f64 = (0.5 * assign11600_e10845);
        var_vfgd_ov = assign11600_e10846;
        var_vfgd_ov_dn3 = (0.5 * (var_t0_dn3 - (((var_t0_dn3 * var_t0) + (var_t0 * var_t0_dn3)) / (2.0 * assign11600_e10844))));
        var_vfgd_ov_dn4 = (0.5 * (var_t0_dn4 - (((var_t0_dn4 * var_t0) + (var_t0 * var_t0_dn4)) / (2.0 * assign11600_e10844))));
        var_vfgd_ov_dn5 = (0.5 * (var_t0_dn5 - (((var_t0_dn5 * var_t0) + (var_t0 * var_t0_dn5)) / (2.0 * assign11600_e10844))));
        var_vfgd_ov_dn6 = (0.5 * (var_t0_dn6 - (((var_t0_dn6 * var_t0) + (var_t0 * var_t0_dn6)) / (2.0 * assign11600_e10844))));
        var_vfgd_ov_dn7 = (0.5 * (var_t0_dn7 - (((var_t0_dn7 * var_t0) + (var_t0 * var_t0_dn7)) / (2.0 * assign11600_e10844))));
        var_vfgd_ov_dn8 = (0.5 * (var_t0_dn8 - (((var_t0_dn8 * var_t0) + (var_t0 * var_t0_dn8)) / (2.0 * assign11600_e10844))));

        let assign11610_e10849: f64 = (var_vgd_ov_noswap - var_vfbsd);
        let assign11610_e10851: f64 = (assign11610_e10849 - var_vfgd_ov);
        var_t1 = assign11610_e10851;
        var_t1_dn3 = ((-var_vfbsd_dn3) - var_vfgd_ov_dn3);
        var_t1_dn4 = ((-var_vfbsd_dn4) - var_vfgd_ov_dn4);
        var_t1_dn5 = ((var_vgd_ov_noswap_dn5 - var_vfbsd_dn5) - var_vfgd_ov_dn5);
        var_t1_dn6 = ((-var_vfbsd_dn6) - var_vfgd_ov_dn6);
        var_t1_dn7 = ((var_vgd_ov_noswap_dn7 - var_vfbsd_dn7) - var_vfgd_ov_dn7);
        var_t1_dn8 = ((-var_vfbsd_dn8) - var_vfgd_ov_dn8);

        let assign11620_e10855: f64 = (var_devsign * var_weffcv);
        let assign11620_e10857: f64 = (assign11620_e10855 * p.p264);
        let assign11620_e10861: f64 = (0.5 * p.p266);
        let assign11620_e10865: f64 = (4.0 * var_vfgd_ov);
        let assign11620_e10867: f64 = (assign11620_e10865 / p.p266);
        let assign11620_e10868: f64 = (1.0 - assign11620_e10867);
        let assign11620_e10869: f64 = (assign11620_e10868).sqrt();
        let assign11620_e10871: f64 = (assign11620_e10869 - 1.0);
        let assign11620_e10872: f64 = (assign11620_e10861 * assign11620_e10871);
        let assign11620_e10873: f64 = (var_t1 - assign11620_e10872);
        let assign11620_e10874: f64 = (assign11620_e10857 * assign11620_e10873);
        let assign11620_e10875: f64 = (var_qfgd_ov + assign11620_e10874);
        var_qfgd_ov = assign11620_e10875;
        var_qfgd_ov_dn3 = (var_qfgd_ov_dn3 + (assign11620_e10857 * (var_t1_dn3 - (assign11620_e10861 * ((-((4.0 * var_vfgd_ov_dn3) / p.p266)) / (2.0 * assign11620_e10869))))));
        var_qfgd_ov_dn4 = (var_qfgd_ov_dn4 + (assign11620_e10857 * (var_t1_dn4 - (assign11620_e10861 * ((-((4.0 * var_vfgd_ov_dn4) / p.p266)) / (2.0 * assign11620_e10869))))));
        var_qfgd_ov_dn5 = (var_qfgd_ov_dn5 + (assign11620_e10857 * (var_t1_dn5 - (assign11620_e10861 * ((-((4.0 * var_vfgd_ov_dn5) / p.p266)) / (2.0 * assign11620_e10869))))));
        var_qfgd_ov_dn6 = (var_qfgd_ov_dn6 + (assign11620_e10857 * (var_t1_dn6 - (assign11620_e10861 * ((-((4.0 * var_vfgd_ov_dn6) / p.p266)) / (2.0 * assign11620_e10869))))));
        var_qfgd_ov_dn7 = (var_qfgd_ov_dn7 + (assign11620_e10857 * (var_t1_dn7 - (assign11620_e10861 * ((-((4.0 * var_vfgd_ov_dn7) / p.p266)) / (2.0 * assign11620_e10869))))));
        var_qfgd_ov_dn8 = (var_qfgd_ov_dn8 + (assign11620_e10857 * (var_t1_dn8 - (assign11620_e10861 * ((-((4.0 * var_vfgd_ov_dn8) / p.p266)) / (2.0 * assign11620_e10869))))));

        let assign11630_e10878: f64 = (var_weffcv * var_cfs_i);
        let assign11630_e10880: f64 = (assign11630_e10878 * (nv7 - nv6));
        var_qfgs_of = assign11630_e10880;
        var_qfgs_of_dn6 = (-assign11630_e10878);
        var_qfgs_of_dn7 = assign11630_e10878;

        let assign11640_e10883: f64 = (var_weffcv * var_cfd_i);
        let assign11640_e10885: f64 = (assign11640_e10883 * (nv7 - nv5));
        var_qfgd_of = assign11640_e10885;
        var_qfgd_of_dn5 = (-assign11640_e10883);
        var_qfgd_of_dn7 = assign11640_e10883;

        let assign11650_e10888: f64 = (var_qfgs_ov + var_qfgs_of);
        var_qfgs_parasitic = assign11650_e10888;
        var_qfgs_parasitic_dn3 = var_qfgs_ov_dn3;
        var_qfgs_parasitic_dn4 = var_qfgs_ov_dn4;
        var_qfgs_parasitic_dn5 = var_qfgs_ov_dn5;
        var_qfgs_parasitic_dn6 = (var_qfgs_ov_dn6 + var_qfgs_of_dn6);
        var_qfgs_parasitic_dn7 = (var_qfgs_ov_dn7 + var_qfgs_of_dn7);
        var_qfgs_parasitic_dn8 = var_qfgs_ov_dn8;

        let assign11660_e10891: f64 = (var_qfgd_ov + var_qfgd_of);
        var_qfgd_parasitic = assign11660_e10891;
        var_qfgd_parasitic_dn3 = var_qfgd_ov_dn3;
        var_qfgd_parasitic_dn4 = var_qfgd_ov_dn4;
        var_qfgd_parasitic_dn5 = (var_qfgd_ov_dn5 + var_qfgd_of_dn5);
        var_qfgd_parasitic_dn6 = var_qfgd_ov_dn6;
        var_qfgd_parasitic_dn7 = (var_qfgd_ov_dn7 + var_qfgd_of_dn7);
        var_qfgd_parasitic_dn8 = var_qfgd_ov_dn8;

        let assign11670_e10894: f64 = (var_devsign * var_csbox);
        let assign11670_e10896: f64 = (assign11670_e10894 * (nv6 - nv3));
        var_qsbg = assign11670_e10896;
        var_qsbg_dn3 = (((var_devsign * var_csbox_dn3) * (nv6 - nv3)) + (-assign11670_e10894));
        var_qsbg_dn4 = ((var_devsign * var_csbox_dn4) * (nv6 - nv3));
        var_qsbg_dn5 = ((var_devsign * var_csbox_dn5) * (nv6 - nv3));
        var_qsbg_dn6 = (((var_devsign * var_csbox_dn6) * (nv6 - nv3)) + assign11670_e10894);
        var_qsbg_dn7 = ((var_devsign * var_csbox_dn7) * (nv6 - nv3));
        var_qsbg_dn8 = ((var_devsign * var_csbox_dn8) * (nv6 - nv3));

        let assign11680_e10899: f64 = (var_devsign * var_cdbox);
        let assign11680_e10901: f64 = (assign11680_e10899 * (nv5 - nv3));
        var_qdbg = assign11680_e10901;
        var_qdbg_dn3 = (((var_devsign * var_cdbox_dn3) * (nv5 - nv3)) + (-assign11680_e10899));
        var_qdbg_dn4 = ((var_devsign * var_cdbox_dn4) * (nv5 - nv3));
        var_qdbg_dn5 = (((var_devsign * var_cdbox_dn5) * (nv5 - nv3)) + assign11680_e10899);
        var_qdbg_dn6 = ((var_devsign * var_cdbox_dn6) * (nv5 - nv3));
        var_qdbg_dn7 = ((var_devsign * var_cdbox_dn7) * (nv5 - nv3));
        var_qdbg_dn8 = ((var_devsign * var_cdbox_dn8) * (nv5 - nv3));

        let assign11690_e10905: f64 = (var_alpha1_i * var_leff);
        let assign11690_e10906: f64 = (var_alpha0_i + assign11690_e10905);
        let assign11690_e10908: f64 = (assign11690_e10906 / var_leff);
        var_t0 = assign11690_e10908;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = 0.0;
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;

        let assign11700_e10915: f64 = if ((var_t0 <= 0.0) || (var_beta0_t <= 0.0)) { 1.0 } else { 0.0 };
        var_guard119 = assign11700_e10915;

        let (assign11710_e10919, assign11710_e10919_d_n3, assign11710_e10919_d_n4, assign11710_e10919_d_n5, assign11710_e10919_d_n6, assign11710_e10919_d_n7, assign11710_e10919_d_n8,) = {
    if (var_guard119 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_iii, var_iii_dn3, var_iii_dn4, var_iii_dn5, var_iii_dn6, var_iii_dn7, var_iii_dn8,)
    }
};
        var_iii = assign11710_e10919;
        var_iii_dn3 = assign11710_e10919_d_n3;
        var_iii_dn4 = assign11710_e10919_d_n4;
        var_iii_dn5 = assign11710_e10919_d_n5;
        var_iii_dn6 = assign11710_e10919_d_n6;
        var_iii_dn7 = assign11710_e10919_d_n7;
        var_iii_dn8 = assign11710_e10919_d_n8;

        let assign11720_e10923: f64 = (var_beta0_t / 80.0);
        let assign11720_e10924: f64 = if var_diffvds > assign11720_e10923 { 1.0 } else { 0.0 };
        var_guard120 = assign11720_e10924;

        let (assign11730_e10934, assign11730_e10934_d_n3, assign11730_e10934_d_n4, assign11730_e10934_d_n5, assign11730_e10934_d_n6, assign11730_e10934_d_n7, assign11730_e10934_d_n8,) = {
    if ((var_guard119 == 0.0) && (var_guard120 != 0.0)) {
        let assign11730_e10930: f64 = (-var_beta0_t);
        let assign11730_e10932: f64 = (assign11730_e10930 / var_diffvds);
        (assign11730_e10932, (-((assign11730_e10930 * var_diffvds_dn3) / (var_diffvds * var_diffvds))), ((((-var_beta0_t_dn4) * var_diffvds) - (assign11730_e10930 * var_diffvds_dn4)) / (var_diffvds * var_diffvds)), (-((assign11730_e10930 * var_diffvds_dn5) / (var_diffvds * var_diffvds))), (-((assign11730_e10930 * var_diffvds_dn6) / (var_diffvds * var_diffvds))), (-((assign11730_e10930 * var_diffvds_dn7) / (var_diffvds * var_diffvds))), (-((assign11730_e10930 * var_diffvds_dn8) / (var_diffvds * var_diffvds))),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign11730_e10934;
        var_t1_dn3 = assign11730_e10934_d_n3;
        var_t1_dn4 = assign11730_e10934_d_n4;
        var_t1_dn5 = assign11730_e10934_d_n5;
        var_t1_dn6 = assign11730_e10934_d_n6;
        var_t1_dn7 = assign11730_e10934_d_n7;
        var_t1_dn8 = assign11730_e10934_d_n8;

        let (assign11740_e10948, assign11740_e10948_d_n3, assign11740_e10948_d_n4, assign11740_e10948_d_n5, assign11740_e10948_d_n6, assign11740_e10948_d_n7, assign11740_e10948_d_n8,) = {
    if ((var_guard119 == 0.0) && (var_guard120 != 0.0)) {
        let assign11740_e10941: f64 = (var_t0 * var_diffvds);
        let assign11740_e10943: f64 = (assign11740_e10941 * var_ids);
        let assign11740_e10945: f64 = { let limited_exp_arg = var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign11740_e10946: f64 = (assign11740_e10943 * assign11740_e10945);
        (assign11740_e10946, ((((((var_t0_dn3 * var_diffvds) + (var_t0 * var_diffvds_dn3)) * var_ids) + (assign11740_e10941 * var_ids_dn3)) * assign11740_e10945) + (assign11740_e10943 * ({ let limited_exp_arg = var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t1_dn3))), ((((((var_t0_dn4 * var_diffvds) + (var_t0 * var_diffvds_dn4)) * var_ids) + (assign11740_e10941 * var_ids_dn4)) * assign11740_e10945) + (assign11740_e10943 * ({ let limited_exp_arg = var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t1_dn4))), ((((((var_t0_dn5 * var_diffvds) + (var_t0 * var_diffvds_dn5)) * var_ids) + (assign11740_e10941 * var_ids_dn5)) * assign11740_e10945) + (assign11740_e10943 * ({ let limited_exp_arg = var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t1_dn5))), ((((((var_t0_dn6 * var_diffvds) + (var_t0 * var_diffvds_dn6)) * var_ids) + (assign11740_e10941 * var_ids_dn6)) * assign11740_e10945) + (assign11740_e10943 * ({ let limited_exp_arg = var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t1_dn6))), ((((((var_t0_dn7 * var_diffvds) + (var_t0 * var_diffvds_dn7)) * var_ids) + (assign11740_e10941 * var_ids_dn7)) * assign11740_e10945) + (assign11740_e10943 * ({ let limited_exp_arg = var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t1_dn7))), ((((((var_t0_dn8 * var_diffvds) + (var_t0 * var_diffvds_dn8)) * var_ids) + (assign11740_e10941 * var_ids_dn8)) * assign11740_e10945) + (assign11740_e10943 * ({ let limited_exp_arg = var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t1_dn8))),)
    } else {
        (var_iii, var_iii_dn3, var_iii_dn4, var_iii_dn5, var_iii_dn6, var_iii_dn7, var_iii_dn8,)
    }
};
        var_iii = assign11740_e10948;
        var_iii_dn3 = assign11740_e10948_d_n3;
        var_iii_dn4 = assign11740_e10948_d_n4;
        var_iii_dn5 = assign11740_e10948_d_n5;
        var_iii_dn6 = assign11740_e10948_d_n6;
        var_iii_dn7 = assign11740_e10948_d_n7;
        var_iii_dn8 = assign11740_e10948_d_n8;

        let (assign11750_e10962, assign11750_e10962_d_n3, assign11750_e10962_d_n4, assign11750_e10962_d_n5, assign11750_e10962_d_n6, assign11750_e10962_d_n7, assign11750_e10962_d_n8,) = {
    if ((var_guard119 == 0.0) && (var_guard120 == 0.0)) {
        let assign11750_e10956: f64 = (var_t0 * var_diffvds);
        let assign11750_e10958: f64 = (assign11750_e10956 * var_ids);
        let assign11750_e10960: f64 = (assign11750_e10958 * 1.804851387e-35);
        (assign11750_e10960, (((((var_t0_dn3 * var_diffvds) + (var_t0 * var_diffvds_dn3)) * var_ids) + (assign11750_e10956 * var_ids_dn3)) * 1.804851387e-35), (((((var_t0_dn4 * var_diffvds) + (var_t0 * var_diffvds_dn4)) * var_ids) + (assign11750_e10956 * var_ids_dn4)) * 1.804851387e-35), (((((var_t0_dn5 * var_diffvds) + (var_t0 * var_diffvds_dn5)) * var_ids) + (assign11750_e10956 * var_ids_dn5)) * 1.804851387e-35), (((((var_t0_dn6 * var_diffvds) + (var_t0 * var_diffvds_dn6)) * var_ids) + (assign11750_e10956 * var_ids_dn6)) * 1.804851387e-35), (((((var_t0_dn7 * var_diffvds) + (var_t0 * var_diffvds_dn7)) * var_ids) + (assign11750_e10956 * var_ids_dn7)) * 1.804851387e-35), (((((var_t0_dn8 * var_diffvds) + (var_t0 * var_diffvds_dn8)) * var_ids) + (assign11750_e10956 * var_ids_dn8)) * 1.804851387e-35),)
    } else {
        (var_iii, var_iii_dn3, var_iii_dn4, var_iii_dn5, var_iii_dn6, var_iii_dn7, var_iii_dn8,)
    }
};
        var_iii = assign11750_e10962;
        var_iii_dn3 = assign11750_e10962_d_n3;
        var_iii_dn4 = assign11750_e10962_d_n4;
        var_iii_dn5 = assign11750_e10962_d_n5;
        var_iii_dn6 = assign11750_e10962_d_n6;
        var_iii_dn7 = assign11750_e10962_d_n7;
        var_iii_dn8 = assign11750_e10962_d_n8;

        var_igbinv = 0.0;
        var_igbinv_dn3 = 0.0;
        var_igbinv_dn4 = 0.0;
        var_igbinv_dn5 = 0.0;
        var_igbinv_dn6 = 0.0;
        var_igbinv_dn7 = 0.0;
        var_igbinv_dn8 = 0.0;

        var_igbacc = 0.0;
        var_igbacc_dn3 = 0.0;
        var_igbacc_dn4 = 0.0;
        var_igbacc_dn5 = 0.0;
        var_igbacc_dn6 = 0.0;
        var_igbacc_dn7 = 0.0;
        var_igbacc_dn8 = 0.0;

        var_igcs = 0.0;
        var_igcs_dn3 = 0.0;
        var_igcs_dn4 = 0.0;
        var_igcs_dn5 = 0.0;
        var_igcs_dn6 = 0.0;
        var_igcs_dn7 = 0.0;
        var_igcs_dn8 = 0.0;

        var_igcd = 0.0;
        var_igcd_dn3 = 0.0;
        var_igcd_dn4 = 0.0;
        var_igcd_dn5 = 0.0;
        var_igcd_dn6 = 0.0;
        var_igcd_dn7 = 0.0;
        var_igcd_dn8 = 0.0;

        var_igs = 0.0;
        var_igs_dn3 = 0.0;
        var_igs_dn4 = 0.0;
        var_igs_dn5 = 0.0;
        var_igs_dn6 = 0.0;
        var_igs_dn7 = 0.0;
        var_igs_dn8 = 0.0;

        var_igd = 0.0;
        var_igd_dn3 = 0.0;
        var_igd_dn4 = 0.0;
        var_igd_dn5 = 0.0;
        var_igd_dn6 = 0.0;
        var_igd_dn7 = 0.0;
        var_igd_dn8 = 0.0;

        let assign11820_e10971: f64 = if p.p17 != 0.0 { 1.0 } else { 0.0 };
        var_guard121 = assign11820_e10971;

        let (assign11830_e10981, assign11830_e10981_d_n3, assign11830_e10981_d_n4, assign11830_e10981_d_n5, assign11830_e10981_d_n6, assign11830_e10981_d_n7, assign11830_e10981_d_n8,) = {
    if (var_guard121 != 0.0) {
        let assign11830_e10975: f64 = (var_qia - var_eigbinv_i);
        let assign11830_e10977: f64 = (assign11830_e10975 / var_nigbinv_i);
        let assign11830_e10979: f64 = (assign11830_e10977 / var_vtm);
        (assign11830_e10979, ((var_qia_dn3 / var_nigbinv_i) / var_vtm), ((((var_qia_dn4 / var_nigbinv_i) * var_vtm) - (assign11830_e10977 * var_vtm_dn4)) / (var_vtm * var_vtm)), ((var_qia_dn5 / var_nigbinv_i) / var_vtm), ((var_qia_dn6 / var_nigbinv_i) / var_vtm), ((var_qia_dn7 / var_nigbinv_i) / var_vtm), ((var_qia_dn8 / var_nigbinv_i) / var_vtm),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign11830_e10981;
        var_t1_dn3 = assign11830_e10981_d_n3;
        var_t1_dn4 = assign11830_e10981_d_n4;
        var_t1_dn5 = assign11830_e10981_d_n5;
        var_t1_dn6 = assign11830_e10981_d_n6;
        var_t1_dn7 = assign11830_e10981_d_n7;
        var_t1_dn8 = assign11830_e10981_d_n8;

        let (assign11840_e10995, assign11840_e10995_d_n3, assign11840_e10995_d_n4, assign11840_e10995_d_n5, assign11840_e10995_d_n6, assign11840_e10995_d_n7, assign11840_e10995_d_n8,) = {
    if (var_guard121 != 0.0) {
        let assign11840_e10985: f64 = (var_nigbinv_i * var_vtm);
        let assign11840_e10988: f64 = { let limited_exp_arg = var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign11840_e10989: f64 = (1.0 + assign11840_e10988);
        let assign11840_e10991: f64 = (assign11840_e10989).max(1e-38);
        let assign11840_e10992: f64 = (assign11840_e10991).ln();
        let assign11840_e10993: f64 = (assign11840_e10985 * assign11840_e10992);
        (assign11840_e10993, (assign11840_e10985 * (if assign11840_e10989 >= 1e-38 { ({ let limited_exp_arg = var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t1_dn3) } else { 0.0 } / assign11840_e10991)), (((var_nigbinv_i * var_vtm_dn4) * assign11840_e10992) + (assign11840_e10985 * (if assign11840_e10989 >= 1e-38 { ({ let limited_exp_arg = var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t1_dn4) } else { 0.0 } / assign11840_e10991))), (assign11840_e10985 * (if assign11840_e10989 >= 1e-38 { ({ let limited_exp_arg = var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t1_dn5) } else { 0.0 } / assign11840_e10991)), (assign11840_e10985 * (if assign11840_e10989 >= 1e-38 { ({ let limited_exp_arg = var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t1_dn6) } else { 0.0 } / assign11840_e10991)), (assign11840_e10985 * (if assign11840_e10989 >= 1e-38 { ({ let limited_exp_arg = var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t1_dn7) } else { 0.0 } / assign11840_e10991)), (assign11840_e10985 * (if assign11840_e10989 >= 1e-38 { ({ let limited_exp_arg = var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t1_dn8) } else { 0.0 } / assign11840_e10991)),)
    } else {
        (var_vaux_igbinv, var_vaux_igbinv_dn3, var_vaux_igbinv_dn4, var_vaux_igbinv_dn5, var_vaux_igbinv_dn6, var_vaux_igbinv_dn7, var_vaux_igbinv_dn8,)
    }
};
        var_vaux_igbinv = assign11840_e10995;
        var_vaux_igbinv_dn3 = assign11840_e10995_d_n3;
        var_vaux_igbinv_dn4 = assign11840_e10995_d_n4;
        var_vaux_igbinv_dn5 = assign11840_e10995_d_n5;
        var_vaux_igbinv_dn6 = assign11840_e10995_d_n6;
        var_vaux_igbinv_dn7 = assign11840_e10995_d_n7;
        var_vaux_igbinv_dn8 = assign11840_e10995_d_n8;

        let (assign11850_e11003, assign11850_e11003_d_n3, assign11850_e11003_d_n4, assign11850_e11003_d_n5, assign11850_e11003_d_n6, assign11850_e11003_d_n7, assign11850_e11003_d_n8,) = {
    if (var_guard121 != 0.0) {
        let assign11850_e11000: f64 = (var_bigbinv_i * var_qia);
        let assign11850_e11001: f64 = (var_aigbinv_i - assign11850_e11000);
        (assign11850_e11001, (-(var_bigbinv_i * var_qia_dn3)), (-(var_bigbinv_i * var_qia_dn4)), (-(var_bigbinv_i * var_qia_dn5)), (-(var_bigbinv_i * var_qia_dn6)), (-(var_bigbinv_i * var_qia_dn7)), (-(var_bigbinv_i * var_qia_dn8)),)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8,)
    }
};
        var_t2 = assign11850_e11003;
        var_t2_dn3 = assign11850_e11003_d_n3;
        var_t2_dn4 = assign11850_e11003_d_n4;
        var_t2_dn5 = assign11850_e11003_d_n5;
        var_t2_dn6 = assign11850_e11003_d_n6;
        var_t2_dn7 = assign11850_e11003_d_n7;
        var_t2_dn8 = assign11850_e11003_d_n8;

        let (assign11860_e11011, assign11860_e11011_d_n3, assign11860_e11011_d_n4, assign11860_e11011_d_n5, assign11860_e11011_d_n6, assign11860_e11011_d_n7, assign11860_e11011_d_n8,) = {
    if (var_guard121 != 0.0) {
        let assign11860_e11008: f64 = (var_cigbinv_i * var_qia);
        let assign11860_e11009: f64 = (1.0 + assign11860_e11008);
        (assign11860_e11009, (var_cigbinv_i * var_qia_dn3), (var_cigbinv_i * var_qia_dn4), (var_cigbinv_i * var_qia_dn5), (var_cigbinv_i * var_qia_dn6), (var_cigbinv_i * var_qia_dn7), (var_cigbinv_i * var_qia_dn8),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8,)
    }
};
        var_t3 = assign11860_e11011;
        var_t3_dn3 = assign11860_e11011_d_n3;
        var_t3_dn4 = assign11860_e11011_d_n4;
        var_t3_dn5 = assign11860_e11011_d_n5;
        var_t3_dn6 = assign11860_e11011_d_n6;
        var_t3_dn7 = assign11860_e11011_d_n7;
        var_t3_dn8 = assign11860_e11011_d_n8;

        *var_coxeff_slot = var_coxeff;
        *var_coxeff_dn3_slot = var_coxeff_dn3;
        *var_coxeff_dn4_slot = var_coxeff_dn4;
        *var_coxeff_dn5_slot = var_coxeff_dn5;
        *var_coxeff_dn6_slot = var_coxeff_dn6;
        *var_coxeff_dn7_slot = var_coxeff_dn7;
        *var_coxeff_dn8_slot = var_coxeff_dn8;
        *var_guard119_slot = var_guard119;
        *var_guard120_slot = var_guard120;
        *var_guard121_slot = var_guard121;
        *var_igbacc_slot = var_igbacc;
        *var_igbacc_dn3_slot = var_igbacc_dn3;
        *var_igbacc_dn4_slot = var_igbacc_dn4;
        *var_igbacc_dn5_slot = var_igbacc_dn5;
        *var_igbacc_dn6_slot = var_igbacc_dn6;
        *var_igbacc_dn7_slot = var_igbacc_dn7;
        *var_igbacc_dn8_slot = var_igbacc_dn8;
        *var_igbinv_slot = var_igbinv;
        *var_igbinv_dn3_slot = var_igbinv_dn3;
        *var_igbinv_dn4_slot = var_igbinv_dn4;
        *var_igbinv_dn5_slot = var_igbinv_dn5;
        *var_igbinv_dn6_slot = var_igbinv_dn6;
        *var_igbinv_dn7_slot = var_igbinv_dn7;
        *var_igbinv_dn8_slot = var_igbinv_dn8;
        *var_igcd_slot = var_igcd;
        *var_igcd_dn3_slot = var_igcd_dn3;
        *var_igcd_dn4_slot = var_igcd_dn4;
        *var_igcd_dn5_slot = var_igcd_dn5;
        *var_igcd_dn6_slot = var_igcd_dn6;
        *var_igcd_dn7_slot = var_igcd_dn7;
        *var_igcd_dn8_slot = var_igcd_dn8;
        *var_igcs_slot = var_igcs;
        *var_igcs_dn3_slot = var_igcs_dn3;
        *var_igcs_dn4_slot = var_igcs_dn4;
        *var_igcs_dn5_slot = var_igcs_dn5;
        *var_igcs_dn6_slot = var_igcs_dn6;
        *var_igcs_dn7_slot = var_igcs_dn7;
        *var_igcs_dn8_slot = var_igcs_dn8;
        *var_igd_slot = var_igd;
        *var_igd_dn3_slot = var_igd_dn3;
        *var_igd_dn4_slot = var_igd_dn4;
        *var_igd_dn5_slot = var_igd_dn5;
        *var_igd_dn6_slot = var_igd_dn6;
        *var_igd_dn7_slot = var_igd_dn7;
        *var_igd_dn8_slot = var_igd_dn8;
        *var_igs_slot = var_igs;
        *var_igs_dn3_slot = var_igs_dn3;
        *var_igs_dn4_slot = var_igs_dn4;
        *var_igs_dn5_slot = var_igs_dn5;
        *var_igs_dn6_slot = var_igs_dn6;
        *var_igs_dn7_slot = var_igs_dn7;
        *var_igs_dn8_slot = var_igs_dn8;
        *var_iii_slot = var_iii;
        *var_iii_dn3_slot = var_iii_dn3;
        *var_iii_dn4_slot = var_iii_dn4;
        *var_iii_dn5_slot = var_iii_dn5;
        *var_iii_dn6_slot = var_iii_dn6;
        *var_iii_dn7_slot = var_iii_dn7;
        *var_iii_dn8_slot = var_iii_dn8;
        *var_qbg_slot = var_qbg;
        *var_qbg_dn3_slot = var_qbg_dn3;
        *var_qbg_dn4_slot = var_qbg_dn4;
        *var_qbg_dn5_slot = var_qbg_dn5;
        *var_qbg_dn6_slot = var_qbg_dn6;
        *var_qbg_dn7_slot = var_qbg_dn7;
        *var_qbg_dn8_slot = var_qbg_dn8;
        *var_qd_slot = var_qd;
        *var_qd_dn3_slot = var_qd_dn3;
        *var_qd_dn4_slot = var_qd_dn4;
        *var_qd_dn5_slot = var_qd_dn5;
        *var_qd_dn6_slot = var_qd_dn6;
        *var_qd_dn7_slot = var_qd_dn7;
        *var_qd_dn8_slot = var_qd_dn8;
        *var_qdbg_slot = var_qdbg;
        *var_qdbg_dn3_slot = var_qdbg_dn3;
        *var_qdbg_dn4_slot = var_qdbg_dn4;
        *var_qdbg_dn5_slot = var_qdbg_dn5;
        *var_qdbg_dn6_slot = var_qdbg_dn6;
        *var_qdbg_dn7_slot = var_qdbg_dn7;
        *var_qdbg_dn8_slot = var_qdbg_dn8;
        *var_qfg_slot = var_qfg;
        *var_qfg_dn3_slot = var_qfg_dn3;
        *var_qfg_dn4_slot = var_qfg_dn4;
        *var_qfg_dn5_slot = var_qfg_dn5;
        *var_qfg_dn6_slot = var_qfg_dn6;
        *var_qfg_dn7_slot = var_qfg_dn7;
        *var_qfg_dn8_slot = var_qfg_dn8;
        *var_qfgd_of_slot = var_qfgd_of;
        *var_qfgd_of_dn5_slot = var_qfgd_of_dn5;
        *var_qfgd_of_dn7_slot = var_qfgd_of_dn7;
        *var_qfgd_ov_slot = var_qfgd_ov;
        *var_qfgd_ov_dn3_slot = var_qfgd_ov_dn3;
        *var_qfgd_ov_dn4_slot = var_qfgd_ov_dn4;
        *var_qfgd_ov_dn5_slot = var_qfgd_ov_dn5;
        *var_qfgd_ov_dn6_slot = var_qfgd_ov_dn6;
        *var_qfgd_ov_dn7_slot = var_qfgd_ov_dn7;
        *var_qfgd_ov_dn8_slot = var_qfgd_ov_dn8;
        *var_qfgd_parasitic_slot = var_qfgd_parasitic;
        *var_qfgd_parasitic_dn3_slot = var_qfgd_parasitic_dn3;
        *var_qfgd_parasitic_dn4_slot = var_qfgd_parasitic_dn4;
        *var_qfgd_parasitic_dn5_slot = var_qfgd_parasitic_dn5;
        *var_qfgd_parasitic_dn6_slot = var_qfgd_parasitic_dn6;
        *var_qfgd_parasitic_dn7_slot = var_qfgd_parasitic_dn7;
        *var_qfgd_parasitic_dn8_slot = var_qfgd_parasitic_dn8;
        *var_qfgs_of_slot = var_qfgs_of;
        *var_qfgs_of_dn6_slot = var_qfgs_of_dn6;
        *var_qfgs_of_dn7_slot = var_qfgs_of_dn7;
        *var_qfgs_ov_slot = var_qfgs_ov;
        *var_qfgs_ov_dn3_slot = var_qfgs_ov_dn3;
        *var_qfgs_ov_dn4_slot = var_qfgs_ov_dn4;
        *var_qfgs_ov_dn5_slot = var_qfgs_ov_dn5;
        *var_qfgs_ov_dn6_slot = var_qfgs_ov_dn6;
        *var_qfgs_ov_dn7_slot = var_qfgs_ov_dn7;
        *var_qfgs_ov_dn8_slot = var_qfgs_ov_dn8;
        *var_qfgs_parasitic_slot = var_qfgs_parasitic;
        *var_qfgs_parasitic_dn3_slot = var_qfgs_parasitic_dn3;
        *var_qfgs_parasitic_dn4_slot = var_qfgs_parasitic_dn4;
        *var_qfgs_parasitic_dn5_slot = var_qfgs_parasitic_dn5;
        *var_qfgs_parasitic_dn6_slot = var_qfgs_parasitic_dn6;
        *var_qfgs_parasitic_dn7_slot = var_qfgs_parasitic_dn7;
        *var_qfgs_parasitic_dn8_slot = var_qfgs_parasitic_dn8;
        *var_qs_slot = var_qs;
        *var_qs_dn3_slot = var_qs_dn3;
        *var_qs_dn4_slot = var_qs_dn4;
        *var_qs_dn5_slot = var_qs_dn5;
        *var_qs_dn6_slot = var_qs_dn6;
        *var_qs_dn7_slot = var_qs_dn7;
        *var_qs_dn8_slot = var_qs_dn8;
        *var_qsbg_slot = var_qsbg;
        *var_qsbg_dn3_slot = var_qsbg_dn3;
        *var_qsbg_dn4_slot = var_qsbg_dn4;
        *var_qsbg_dn5_slot = var_qsbg_dn5;
        *var_qsbg_dn6_slot = var_qsbg_dn6;
        *var_qsbg_dn7_slot = var_qsbg_dn7;
        *var_qsbg_dn8_slot = var_qsbg_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_vaux_igbinv_slot = var_vaux_igbinv;
        *var_vaux_igbinv_dn3_slot = var_vaux_igbinv_dn3;
        *var_vaux_igbinv_dn4_slot = var_vaux_igbinv_dn4;
        *var_vaux_igbinv_dn5_slot = var_vaux_igbinv_dn5;
        *var_vaux_igbinv_dn6_slot = var_vaux_igbinv_dn6;
        *var_vaux_igbinv_dn7_slot = var_vaux_igbinv_dn7;
        *var_vaux_igbinv_dn8_slot = var_vaux_igbinv_dn8;
        *var_vfbsd_bg_slot = var_vfbsd_bg;
        *var_vfbsd_bg_dn3_slot = var_vfbsd_bg_dn3;
        *var_vfbsd_bg_dn4_slot = var_vfbsd_bg_dn4;
        *var_vfbsd_bg_dn5_slot = var_vfbsd_bg_dn5;
        *var_vfbsd_bg_dn6_slot = var_vfbsd_bg_dn6;
        *var_vfbsd_bg_dn7_slot = var_vfbsd_bg_dn7;
        *var_vfbsd_bg_dn8_slot = var_vfbsd_bg_dn8;
        *var_vfgd_ov_slot = var_vfgd_ov;
        *var_vfgd_ov_dn3_slot = var_vfgd_ov_dn3;
        *var_vfgd_ov_dn4_slot = var_vfgd_ov_dn4;
        *var_vfgd_ov_dn5_slot = var_vfgd_ov_dn5;
        *var_vfgd_ov_dn6_slot = var_vfgd_ov_dn6;
        *var_vfgd_ov_dn7_slot = var_vfgd_ov_dn7;
        *var_vfgd_ov_dn8_slot = var_vfgd_ov_dn8;
        *var_vfgs_ov_slot = var_vfgs_ov;
        *var_vfgs_ov_dn3_slot = var_vfgs_ov_dn3;
        *var_vfgs_ov_dn4_slot = var_vfgs_ov_dn4;
        *var_vfgs_ov_dn5_slot = var_vfgs_ov_dn5;
        *var_vfgs_ov_dn6_slot = var_vfgs_ov_dn6;
        *var_vfgs_ov_dn7_slot = var_vfgs_ov_dn7;
        *var_vfgs_ov_dn8_slot = var_vfgs_ov_dn8;
    }

    pub(super) fn stamp_transient_block_26(
        p: &Parameters,
        var_aechvb: f64,
        var_aigbacc_i: f64,
        var_aigc_i: f64,
        var_bechvb: f64,
        var_bigbacc_i: f64,
        var_bigc_i: f64,
        var_cigbacc_i: f64,
        var_cigc_i: f64,
        var_deltaphi1: f64,
        var_deltaphi1_dn4: f64,
        var_digc_i: f64,
        var_guard121: f64,
        var_igtemp: f64,
        var_igtemp_dn4: f64,
        var_leff: f64,
        var_nigbacc_i: f64,
        var_phib: f64,
        var_phib_dn3: f64,
        var_phib_dn4: f64,
        var_phib_dn5: f64,
        var_phib_dn6: f64,
        var_phib_dn7: f64,
        var_phib_dn8: f64,
        var_phifs: f64,
        var_phifs_dn3: f64,
        var_phifs_dn4: f64,
        var_phifs_dn5: f64,
        var_phifs_dn6: f64,
        var_phifs_dn7: f64,
        var_phifs_dn8: f64,
        var_qia: f64,
        var_qia_dn3: f64,
        var_qia_dn4: f64,
        var_qia_dn5: f64,
        var_qia_dn6: f64,
        var_qia_dn7: f64,
        var_qia_dn8: f64,
        var_toxratio: f64,
        var_toxratio_dn3: f64,
        var_toxratio_dn4: f64,
        var_toxratio_dn5: f64,
        var_toxratio_dn6: f64,
        var_toxratio_dn7: f64,
        var_toxratio_dn8: f64,
        var_vaux_igbinv: f64,
        var_vaux_igbinv_dn3: f64,
        var_vaux_igbinv_dn4: f64,
        var_vaux_igbinv_dn5: f64,
        var_vaux_igbinv_dn6: f64,
        var_vaux_igbinv_dn7: f64,
        var_vaux_igbinv_dn8: f64,
        var_vbgd_noswap: f64,
        var_vbgd_noswap_dn3: f64,
        var_vbgd_noswap_dn5: f64,
        var_vbgs_noswap: f64,
        var_vbgs_noswap_dn3: f64,
        var_vbgs_noswap_dn6: f64,
        var_vds_noswap: f64,
        var_vds_noswap_dn5: f64,
        var_vds_noswap_dn6: f64,
        var_vdsx: f64,
        var_vdsx_dn5: f64,
        var_vdsx_dn6: f64,
        var_vgbg: f64,
        var_vgbg_dn3: f64,
        var_vgbg_dn8: f64,
        var_vgfb1: f64,
        var_vgfb1_dn4: f64,
        var_vgfb1_dn5: f64,
        var_vgfb1_dn6: f64,
        var_vgfb1_dn8: f64,
        var_vtm: f64,
        var_vtm_dn4: f64,
        var_weff: f64,
        var_guard122_slot: &mut f64,
        var_guard123_slot: &mut f64,
        var_igbacc_slot: &mut f64,
        var_igbacc_dn3_slot: &mut f64,
        var_igbacc_dn4_slot: &mut f64,
        var_igbacc_dn5_slot: &mut f64,
        var_igbacc_dn6_slot: &mut f64,
        var_igbacc_dn7_slot: &mut f64,
        var_igbacc_dn8_slot: &mut f64,
        var_igbd_slot: &mut f64,
        var_igbd_dn3_slot: &mut f64,
        var_igbd_dn4_slot: &mut f64,
        var_igbd_dn5_slot: &mut f64,
        var_igbd_dn6_slot: &mut f64,
        var_igbd_dn7_slot: &mut f64,
        var_igbd_dn8_slot: &mut f64,
        var_igbinv_slot: &mut f64,
        var_igbinv_dn3_slot: &mut f64,
        var_igbinv_dn4_slot: &mut f64,
        var_igbinv_dn5_slot: &mut f64,
        var_igbinv_dn6_slot: &mut f64,
        var_igbinv_dn7_slot: &mut f64,
        var_igbinv_dn8_slot: &mut f64,
        var_igbs_slot: &mut f64,
        var_igbs_dn3_slot: &mut f64,
        var_igbs_dn4_slot: &mut f64,
        var_igbs_dn5_slot: &mut f64,
        var_igbs_dn6_slot: &mut f64,
        var_igbs_dn7_slot: &mut f64,
        var_igbs_dn8_slot: &mut f64,
        var_igc0_slot: &mut f64,
        var_igc0_dn3_slot: &mut f64,
        var_igc0_dn4_slot: &mut f64,
        var_igc0_dn5_slot: &mut f64,
        var_igc0_dn6_slot: &mut f64,
        var_igc0_dn7_slot: &mut f64,
        var_igc0_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_vaux_igbacc_slot: &mut f64,
        var_vaux_igbacc_dn3_slot: &mut f64,
        var_vaux_igbacc_dn4_slot: &mut f64,
        var_vaux_igbacc_dn5_slot: &mut f64,
        var_vaux_igbacc_dn6_slot: &mut f64,
        var_vaux_igbacc_dn7_slot: &mut f64,
        var_vaux_igbacc_dn8_slot: &mut f64,
        var_vfbzb_slot: &mut f64,
        var_vfbzb_dn3_slot: &mut f64,
        var_vfbzb_dn4_slot: &mut f64,
        var_vfbzb_dn5_slot: &mut f64,
        var_vfbzb_dn6_slot: &mut f64,
        var_vfbzb_dn7_slot: &mut f64,
        var_vfbzb_dn8_slot: &mut f64,
        var_voxacc_slot: &mut f64,
        var_voxacc_dn3_slot: &mut f64,
        var_voxacc_dn4_slot: &mut f64,
        var_voxacc_dn5_slot: &mut f64,
        var_voxacc_dn6_slot: &mut f64,
        var_voxacc_dn7_slot: &mut f64,
        var_voxacc_dn8_slot: &mut f64,
        var_wf_slot: &mut f64,
        var_wf_dn3_slot: &mut f64,
        var_wf_dn4_slot: &mut f64,
        var_wf_dn5_slot: &mut f64,
        var_wf_dn6_slot: &mut f64,
        var_wf_dn7_slot: &mut f64,
        var_wf_dn8_slot: &mut f64,
        var_wr_slot: &mut f64,
        var_wr_dn3_slot: &mut f64,
        var_wr_dn4_slot: &mut f64,
        var_wr_dn5_slot: &mut f64,
        var_wr_dn6_slot: &mut f64,
        var_wr_dn7_slot: &mut f64,
        var_wr_dn8_slot: &mut f64,
    ) {
        let mut var_guard122: f64 = *var_guard122_slot;
        let mut var_guard123: f64 = *var_guard123_slot;
        let mut var_igbacc: f64 = *var_igbacc_slot;
        let mut var_igbacc_dn3: f64 = *var_igbacc_dn3_slot;
        let mut var_igbacc_dn4: f64 = *var_igbacc_dn4_slot;
        let mut var_igbacc_dn5: f64 = *var_igbacc_dn5_slot;
        let mut var_igbacc_dn6: f64 = *var_igbacc_dn6_slot;
        let mut var_igbacc_dn7: f64 = *var_igbacc_dn7_slot;
        let mut var_igbacc_dn8: f64 = *var_igbacc_dn8_slot;
        let mut var_igbd: f64 = *var_igbd_slot;
        let mut var_igbd_dn3: f64 = *var_igbd_dn3_slot;
        let mut var_igbd_dn4: f64 = *var_igbd_dn4_slot;
        let mut var_igbd_dn5: f64 = *var_igbd_dn5_slot;
        let mut var_igbd_dn6: f64 = *var_igbd_dn6_slot;
        let mut var_igbd_dn7: f64 = *var_igbd_dn7_slot;
        let mut var_igbd_dn8: f64 = *var_igbd_dn8_slot;
        let mut var_igbinv: f64 = *var_igbinv_slot;
        let mut var_igbinv_dn3: f64 = *var_igbinv_dn3_slot;
        let mut var_igbinv_dn4: f64 = *var_igbinv_dn4_slot;
        let mut var_igbinv_dn5: f64 = *var_igbinv_dn5_slot;
        let mut var_igbinv_dn6: f64 = *var_igbinv_dn6_slot;
        let mut var_igbinv_dn7: f64 = *var_igbinv_dn7_slot;
        let mut var_igbinv_dn8: f64 = *var_igbinv_dn8_slot;
        let mut var_igbs: f64 = *var_igbs_slot;
        let mut var_igbs_dn3: f64 = *var_igbs_dn3_slot;
        let mut var_igbs_dn4: f64 = *var_igbs_dn4_slot;
        let mut var_igbs_dn5: f64 = *var_igbs_dn5_slot;
        let mut var_igbs_dn6: f64 = *var_igbs_dn6_slot;
        let mut var_igbs_dn7: f64 = *var_igbs_dn7_slot;
        let mut var_igbs_dn8: f64 = *var_igbs_dn8_slot;
        let mut var_igc0: f64 = *var_igc0_slot;
        let mut var_igc0_dn3: f64 = *var_igc0_dn3_slot;
        let mut var_igc0_dn4: f64 = *var_igc0_dn4_slot;
        let mut var_igc0_dn5: f64 = *var_igc0_dn5_slot;
        let mut var_igc0_dn6: f64 = *var_igc0_dn6_slot;
        let mut var_igc0_dn7: f64 = *var_igc0_dn7_slot;
        let mut var_igc0_dn8: f64 = *var_igc0_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_vaux_igbacc: f64 = *var_vaux_igbacc_slot;
        let mut var_vaux_igbacc_dn3: f64 = *var_vaux_igbacc_dn3_slot;
        let mut var_vaux_igbacc_dn4: f64 = *var_vaux_igbacc_dn4_slot;
        let mut var_vaux_igbacc_dn5: f64 = *var_vaux_igbacc_dn5_slot;
        let mut var_vaux_igbacc_dn6: f64 = *var_vaux_igbacc_dn6_slot;
        let mut var_vaux_igbacc_dn7: f64 = *var_vaux_igbacc_dn7_slot;
        let mut var_vaux_igbacc_dn8: f64 = *var_vaux_igbacc_dn8_slot;
        let mut var_vfbzb: f64 = *var_vfbzb_slot;
        let mut var_vfbzb_dn3: f64 = *var_vfbzb_dn3_slot;
        let mut var_vfbzb_dn4: f64 = *var_vfbzb_dn4_slot;
        let mut var_vfbzb_dn5: f64 = *var_vfbzb_dn5_slot;
        let mut var_vfbzb_dn6: f64 = *var_vfbzb_dn6_slot;
        let mut var_vfbzb_dn7: f64 = *var_vfbzb_dn7_slot;
        let mut var_vfbzb_dn8: f64 = *var_vfbzb_dn8_slot;
        let mut var_voxacc: f64 = *var_voxacc_slot;
        let mut var_voxacc_dn3: f64 = *var_voxacc_dn3_slot;
        let mut var_voxacc_dn4: f64 = *var_voxacc_dn4_slot;
        let mut var_voxacc_dn5: f64 = *var_voxacc_dn5_slot;
        let mut var_voxacc_dn6: f64 = *var_voxacc_dn6_slot;
        let mut var_voxacc_dn7: f64 = *var_voxacc_dn7_slot;
        let mut var_voxacc_dn8: f64 = *var_voxacc_dn8_slot;
        let mut var_wf: f64 = *var_wf_slot;
        let mut var_wf_dn3: f64 = *var_wf_dn3_slot;
        let mut var_wf_dn4: f64 = *var_wf_dn4_slot;
        let mut var_wf_dn5: f64 = *var_wf_dn5_slot;
        let mut var_wf_dn6: f64 = *var_wf_dn6_slot;
        let mut var_wf_dn7: f64 = *var_wf_dn7_slot;
        let mut var_wf_dn8: f64 = *var_wf_dn8_slot;
        let mut var_wr: f64 = *var_wr_slot;
        let mut var_wr_dn3: f64 = *var_wr_dn3_slot;
        let mut var_wr_dn4: f64 = *var_wr_dn4_slot;
        let mut var_wr_dn5: f64 = *var_wr_dn5_slot;
        let mut var_wr_dn6: f64 = *var_wr_dn6_slot;
        let mut var_wr_dn7: f64 = *var_wr_dn7_slot;
        let mut var_wr_dn8: f64 = *var_wr_dn8_slot;

        let (assign11870_e11022, assign11870_e11022_d_n3, assign11870_e11022_d_n4, assign11870_e11022_d_n5, assign11870_e11022_d_n6, assign11870_e11022_d_n7, assign11870_e11022_d_n8,) = {
    if (var_guard121 != 0.0) {
        let assign11870_e11014: f64 = (-982222000000.0);
        let assign11870_e11016: f64 = (assign11870_e11014 * p.p99);
        let assign11870_e11018: f64 = (assign11870_e11016 * var_t2);
        let assign11870_e11020: f64 = (assign11870_e11018 * var_t3);
        (assign11870_e11020, (((assign11870_e11016 * var_t2_dn3) * var_t3) + (assign11870_e11018 * var_t3_dn3)), (((assign11870_e11016 * var_t2_dn4) * var_t3) + (assign11870_e11018 * var_t3_dn4)), (((assign11870_e11016 * var_t2_dn5) * var_t3) + (assign11870_e11018 * var_t3_dn5)), (((assign11870_e11016 * var_t2_dn6) * var_t3) + (assign11870_e11018 * var_t3_dn6)), (((assign11870_e11016 * var_t2_dn7) * var_t3) + (assign11870_e11018 * var_t3_dn7)), (((assign11870_e11016 * var_t2_dn8) * var_t3) + (assign11870_e11018 * var_t3_dn8)),)
    } else {
        (var_t4, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8,)
    }
};
        var_t4 = assign11870_e11022;
        var_t4_dn3 = assign11870_e11022_d_n3;
        var_t4_dn4 = assign11870_e11022_d_n4;
        var_t4_dn5 = assign11870_e11022_d_n5;
        var_t4_dn6 = assign11870_e11022_d_n6;
        var_t4_dn7 = assign11870_e11022_d_n7;
        var_t4_dn8 = assign11870_e11022_d_n8;

        let (assign11880_e11027, assign11880_e11027_d_n3, assign11880_e11027_d_n4, assign11880_e11027_d_n5, assign11880_e11027_d_n6, assign11880_e11027_d_n7, assign11880_e11027_d_n8,) = {
    if (var_guard121 != 0.0) {
        let assign11880_e11025: f64 = { let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign11880_e11025, ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn3), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn4), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn5), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn6), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn7), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn8),)
    } else {
        (var_t5, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8,)
    }
};
        var_t5 = assign11880_e11027;
        var_t5_dn3 = assign11880_e11027_d_n3;
        var_t5_dn4 = assign11880_e11027_d_n4;
        var_t5_dn5 = assign11880_e11027_d_n5;
        var_t5_dn6 = assign11880_e11027_d_n6;
        var_t5_dn7 = assign11880_e11027_d_n7;
        var_t5_dn8 = assign11880_e11027_d_n8;

        let (assign11890_e11031, assign11890_e11031_d_n3, assign11890_e11031_d_n4, assign11890_e11031_d_n5, assign11890_e11031_d_n6, assign11890_e11031_d_n7, assign11890_e11031_d_n8,) = {
    if (var_guard121 != 0.0) {
        (3.75956e-7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t6, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8,)
    }
};
        var_t6 = assign11890_e11031;
        var_t6_dn3 = assign11890_e11031_d_n3;
        var_t6_dn4 = assign11890_e11031_d_n4;
        var_t6_dn5 = assign11890_e11031_d_n5;
        var_t6_dn6 = assign11890_e11031_d_n6;
        var_t6_dn7 = assign11890_e11031_d_n7;
        var_t6_dn8 = assign11890_e11031_d_n8;

        let (assign11900_e11047, assign11900_e11047_d_n3, assign11900_e11047_d_n4, assign11900_e11047_d_n5, assign11900_e11047_d_n6, assign11900_e11047_d_n7, assign11900_e11047_d_n8,) = {
    if (var_guard121 != 0.0) {
        let assign11900_e11035: f64 = (var_weff * var_leff);
        let assign11900_e11037: f64 = (assign11900_e11035 * var_t6);
        let assign11900_e11039: f64 = (assign11900_e11037 * var_toxratio);
        let assign11900_e11041: f64 = (assign11900_e11039 * var_vgbg);
        let assign11900_e11043: f64 = (assign11900_e11041 * var_vaux_igbinv);
        let assign11900_e11045: f64 = (assign11900_e11043 * var_t5);
        (assign11900_e11045, (((((((((assign11900_e11035 * var_t6_dn3) * var_toxratio) + (assign11900_e11037 * var_toxratio_dn3)) * var_vgbg) + (assign11900_e11039 * var_vgbg_dn3)) * var_vaux_igbinv) + (assign11900_e11041 * var_vaux_igbinv_dn3)) * var_t5) + (assign11900_e11043 * var_t5_dn3)), ((((((((assign11900_e11035 * var_t6_dn4) * var_toxratio) + (assign11900_e11037 * var_toxratio_dn4)) * var_vgbg) * var_vaux_igbinv) + (assign11900_e11041 * var_vaux_igbinv_dn4)) * var_t5) + (assign11900_e11043 * var_t5_dn4)), ((((((((assign11900_e11035 * var_t6_dn5) * var_toxratio) + (assign11900_e11037 * var_toxratio_dn5)) * var_vgbg) * var_vaux_igbinv) + (assign11900_e11041 * var_vaux_igbinv_dn5)) * var_t5) + (assign11900_e11043 * var_t5_dn5)), ((((((((assign11900_e11035 * var_t6_dn6) * var_toxratio) + (assign11900_e11037 * var_toxratio_dn6)) * var_vgbg) * var_vaux_igbinv) + (assign11900_e11041 * var_vaux_igbinv_dn6)) * var_t5) + (assign11900_e11043 * var_t5_dn6)), ((((((((assign11900_e11035 * var_t6_dn7) * var_toxratio) + (assign11900_e11037 * var_toxratio_dn7)) * var_vgbg) * var_vaux_igbinv) + (assign11900_e11041 * var_vaux_igbinv_dn7)) * var_t5) + (assign11900_e11043 * var_t5_dn7)), (((((((((assign11900_e11035 * var_t6_dn8) * var_toxratio) + (assign11900_e11037 * var_toxratio_dn8)) * var_vgbg) + (assign11900_e11039 * var_vgbg_dn8)) * var_vaux_igbinv) + (assign11900_e11041 * var_vaux_igbinv_dn8)) * var_t5) + (assign11900_e11043 * var_t5_dn8)),)
    } else {
        (var_igbinv, var_igbinv_dn3, var_igbinv_dn4, var_igbinv_dn5, var_igbinv_dn6, var_igbinv_dn7, var_igbinv_dn8,)
    }
};
        var_igbinv = assign11900_e11047;
        var_igbinv_dn3 = assign11900_e11047_d_n3;
        var_igbinv_dn4 = assign11900_e11047_d_n4;
        var_igbinv_dn5 = assign11900_e11047_d_n5;
        var_igbinv_dn6 = assign11900_e11047_d_n6;
        var_igbinv_dn7 = assign11900_e11047_d_n7;
        var_igbinv_dn8 = assign11900_e11047_d_n8;

        let (assign11910_e11053, assign11910_e11053_d_n3, assign11910_e11053_d_n4, assign11910_e11053_d_n5, assign11910_e11053_d_n6, assign11910_e11053_d_n7, assign11910_e11053_d_n8,) = {
    if (var_guard121 != 0.0) {
        let assign11910_e11051: f64 = (var_igbinv * var_igtemp);
        (assign11910_e11051, (var_igbinv_dn3 * var_igtemp), ((var_igbinv_dn4 * var_igtemp) + (var_igbinv * var_igtemp_dn4)), (var_igbinv_dn5 * var_igtemp), (var_igbinv_dn6 * var_igtemp), (var_igbinv_dn7 * var_igtemp), (var_igbinv_dn8 * var_igtemp),)
    } else {
        (var_igbinv, var_igbinv_dn3, var_igbinv_dn4, var_igbinv_dn5, var_igbinv_dn6, var_igbinv_dn7, var_igbinv_dn8,)
    }
};
        var_igbinv = assign11910_e11053;
        var_igbinv_dn3 = assign11910_e11053_d_n3;
        var_igbinv_dn4 = assign11910_e11053_d_n4;
        var_igbinv_dn5 = assign11910_e11053_d_n5;
        var_igbinv_dn6 = assign11910_e11053_d_n6;
        var_igbinv_dn7 = assign11910_e11053_d_n7;
        var_igbinv_dn8 = assign11910_e11053_d_n8;

        let (assign11920_e11059, assign11920_e11059_d_n3, assign11920_e11059_d_n4, assign11920_e11059_d_n5, assign11920_e11059_d_n6, assign11920_e11059_d_n7, assign11920_e11059_d_n8,) = {
    if (var_guard121 != 0.0) {
        let assign11920_e11057: f64 = (var_deltaphi1 - var_phib);
        (assign11920_e11057, (-var_phib_dn3), (var_deltaphi1_dn4 - var_phib_dn4), (-var_phib_dn5), (-var_phib_dn6), (-var_phib_dn7), (-var_phib_dn8),)
    } else {
        (var_vfbzb, var_vfbzb_dn3, var_vfbzb_dn4, var_vfbzb_dn5, var_vfbzb_dn6, var_vfbzb_dn7, var_vfbzb_dn8,)
    }
};
        var_vfbzb = assign11920_e11059;
        var_vfbzb_dn3 = assign11920_e11059_d_n3;
        var_vfbzb_dn4 = assign11920_e11059_d_n4;
        var_vfbzb_dn5 = assign11920_e11059_d_n5;
        var_vfbzb_dn6 = assign11920_e11059_d_n6;
        var_vfbzb_dn7 = assign11920_e11059_d_n7;
        var_vfbzb_dn8 = assign11920_e11059_d_n8;

        let (assign11930_e11065, assign11930_e11065_d_n3, assign11930_e11065_d_n4, assign11930_e11065_d_n5, assign11930_e11065_d_n6, assign11930_e11065_d_n7, assign11930_e11065_d_n8,) = {
    if (var_guard121 != 0.0) {
        let assign11930_e11063: f64 = (var_vfbzb - var_vgbg);
        (assign11930_e11063, (var_vfbzb_dn3 - var_vgbg_dn3), var_vfbzb_dn4, var_vfbzb_dn5, var_vfbzb_dn6, var_vfbzb_dn7, (var_vfbzb_dn8 - var_vgbg_dn8),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign11930_e11065;
        var_t0_dn3 = assign11930_e11065_d_n3;
        var_t0_dn4 = assign11930_e11065_d_n4;
        var_t0_dn5 = assign11930_e11065_d_n5;
        var_t0_dn6 = assign11930_e11065_d_n6;
        var_t0_dn7 = assign11930_e11065_d_n7;
        var_t0_dn8 = assign11930_e11065_d_n8;

        let (assign11940_e11073, assign11940_e11073_d_n3, assign11940_e11073_d_n4, assign11940_e11073_d_n5, assign11940_e11073_d_n6, assign11940_e11073_d_n7, assign11940_e11073_d_n8,) = {
    if (var_guard121 != 0.0) {
        let assign11940_e11069: f64 = (var_t0 / var_nigbacc_i);
        let assign11940_e11071: f64 = (assign11940_e11069 / var_vtm);
        (assign11940_e11071, ((var_t0_dn3 / var_nigbacc_i) / var_vtm), ((((var_t0_dn4 / var_nigbacc_i) * var_vtm) - (assign11940_e11069 * var_vtm_dn4)) / (var_vtm * var_vtm)), ((var_t0_dn5 / var_nigbacc_i) / var_vtm), ((var_t0_dn6 / var_nigbacc_i) / var_vtm), ((var_t0_dn7 / var_nigbacc_i) / var_vtm), ((var_t0_dn8 / var_nigbacc_i) / var_vtm),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign11940_e11073;
        var_t1_dn3 = assign11940_e11073_d_n3;
        var_t1_dn4 = assign11940_e11073_d_n4;
        var_t1_dn5 = assign11940_e11073_d_n5;
        var_t1_dn6 = assign11940_e11073_d_n6;
        var_t1_dn7 = assign11940_e11073_d_n7;
        var_t1_dn8 = assign11940_e11073_d_n8;

        let (assign11950_e11087, assign11950_e11087_d_n3, assign11950_e11087_d_n4, assign11950_e11087_d_n5, assign11950_e11087_d_n6, assign11950_e11087_d_n7, assign11950_e11087_d_n8,) = {
    if (var_guard121 != 0.0) {
        let assign11950_e11077: f64 = (var_nigbacc_i * var_vtm);
        let assign11950_e11080: f64 = { let limited_exp_arg = var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign11950_e11081: f64 = (1.0 + assign11950_e11080);
        let assign11950_e11083: f64 = (assign11950_e11081).max(1e-38);
        let assign11950_e11084: f64 = (assign11950_e11083).ln();
        let assign11950_e11085: f64 = (assign11950_e11077 * assign11950_e11084);
        (assign11950_e11085, (assign11950_e11077 * (if assign11950_e11081 >= 1e-38 { ({ let limited_exp_arg = var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t1_dn3) } else { 0.0 } / assign11950_e11083)), (((var_nigbacc_i * var_vtm_dn4) * assign11950_e11084) + (assign11950_e11077 * (if assign11950_e11081 >= 1e-38 { ({ let limited_exp_arg = var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t1_dn4) } else { 0.0 } / assign11950_e11083))), (assign11950_e11077 * (if assign11950_e11081 >= 1e-38 { ({ let limited_exp_arg = var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t1_dn5) } else { 0.0 } / assign11950_e11083)), (assign11950_e11077 * (if assign11950_e11081 >= 1e-38 { ({ let limited_exp_arg = var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t1_dn6) } else { 0.0 } / assign11950_e11083)), (assign11950_e11077 * (if assign11950_e11081 >= 1e-38 { ({ let limited_exp_arg = var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t1_dn7) } else { 0.0 } / assign11950_e11083)), (assign11950_e11077 * (if assign11950_e11081 >= 1e-38 { ({ let limited_exp_arg = var_t1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t1_dn8) } else { 0.0 } / assign11950_e11083)),)
    } else {
        (var_vaux_igbacc, var_vaux_igbacc_dn3, var_vaux_igbacc_dn4, var_vaux_igbacc_dn5, var_vaux_igbacc_dn6, var_vaux_igbacc_dn7, var_vaux_igbacc_dn8,)
    }
};
        var_vaux_igbacc = assign11950_e11087;
        var_vaux_igbacc_dn3 = assign11950_e11087_d_n3;
        var_vaux_igbacc_dn4 = assign11950_e11087_d_n4;
        var_vaux_igbacc_dn5 = assign11950_e11087_d_n5;
        var_vaux_igbacc_dn6 = assign11950_e11087_d_n6;
        var_vaux_igbacc_dn7 = assign11950_e11087_d_n7;
        var_vaux_igbacc_dn8 = assign11950_e11087_d_n8;

        let assign11960_e11090: f64 = if var_vfbzb <= 0.0 { 1.0 } else { 0.0 };
        var_guard122 = assign11960_e11090;

        let (assign11970_e11113, assign11970_e11113_d_n3, assign11970_e11113_d_n4, assign11970_e11113_d_n5, assign11970_e11113_d_n6, assign11970_e11113_d_n7, assign11970_e11113_d_n8,) = {
    if ((var_guard121 != 0.0) && (var_guard122 != 0.0)) {
        let assign11970_e11097: f64 = (var_t0 - 0.02);
        let assign11970_e11100: f64 = (var_t0 - 0.02);
        let assign11970_e11103: f64 = (var_t0 - 0.02);
        let assign11970_e11104: f64 = (assign11970_e11100 * assign11970_e11103);
        let assign11970_e11107: f64 = (0.08 * var_vfbzb);
        let assign11970_e11108: f64 = (assign11970_e11104 - assign11970_e11107);
        let assign11970_e11109: f64 = (assign11970_e11108).sqrt();
        let assign11970_e11110: f64 = (assign11970_e11097 + assign11970_e11109);
        let assign11970_e11111: f64 = (0.5 * assign11970_e11110);
        (assign11970_e11111, (0.5 * (var_t0_dn3 + ((((var_t0_dn3 * assign11970_e11103) + (assign11970_e11100 * var_t0_dn3)) - (0.08 * var_vfbzb_dn3)) / (2.0 * assign11970_e11109)))), (0.5 * (var_t0_dn4 + ((((var_t0_dn4 * assign11970_e11103) + (assign11970_e11100 * var_t0_dn4)) - (0.08 * var_vfbzb_dn4)) / (2.0 * assign11970_e11109)))), (0.5 * (var_t0_dn5 + ((((var_t0_dn5 * assign11970_e11103) + (assign11970_e11100 * var_t0_dn5)) - (0.08 * var_vfbzb_dn5)) / (2.0 * assign11970_e11109)))), (0.5 * (var_t0_dn6 + ((((var_t0_dn6 * assign11970_e11103) + (assign11970_e11100 * var_t0_dn6)) - (0.08 * var_vfbzb_dn6)) / (2.0 * assign11970_e11109)))), (0.5 * (var_t0_dn7 + ((((var_t0_dn7 * assign11970_e11103) + (assign11970_e11100 * var_t0_dn7)) - (0.08 * var_vfbzb_dn7)) / (2.0 * assign11970_e11109)))), (0.5 * (var_t0_dn8 + ((((var_t0_dn8 * assign11970_e11103) + (assign11970_e11100 * var_t0_dn8)) - (0.08 * var_vfbzb_dn8)) / (2.0 * assign11970_e11109)))),)
    } else {
        (var_voxacc, var_voxacc_dn3, var_voxacc_dn4, var_voxacc_dn5, var_voxacc_dn6, var_voxacc_dn7, var_voxacc_dn8,)
    }
};
        var_voxacc = assign11970_e11113;
        var_voxacc_dn3 = assign11970_e11113_d_n3;
        var_voxacc_dn4 = assign11970_e11113_d_n4;
        var_voxacc_dn5 = assign11970_e11113_d_n5;
        var_voxacc_dn6 = assign11970_e11113_d_n6;
        var_voxacc_dn7 = assign11970_e11113_d_n7;
        var_voxacc_dn8 = assign11970_e11113_d_n8;

        let (assign11980_e11137, assign11980_e11137_d_n3, assign11980_e11137_d_n4, assign11980_e11137_d_n5, assign11980_e11137_d_n6, assign11980_e11137_d_n7, assign11980_e11137_d_n8,) = {
    if ((var_guard121 != 0.0) && (var_guard122 == 0.0)) {
        let assign11980_e11121: f64 = (var_t0 - 0.02);
        let assign11980_e11124: f64 = (var_t0 - 0.02);
        let assign11980_e11127: f64 = (var_t0 - 0.02);
        let assign11980_e11128: f64 = (assign11980_e11124 * assign11980_e11127);
        let assign11980_e11131: f64 = (0.08 * var_vfbzb);
        let assign11980_e11132: f64 = (assign11980_e11128 + assign11980_e11131);
        let assign11980_e11133: f64 = (assign11980_e11132).sqrt();
        let assign11980_e11134: f64 = (assign11980_e11121 + assign11980_e11133);
        let assign11980_e11135: f64 = (0.5 * assign11980_e11134);
        (assign11980_e11135, (0.5 * (var_t0_dn3 + ((((var_t0_dn3 * assign11980_e11127) + (assign11980_e11124 * var_t0_dn3)) + (0.08 * var_vfbzb_dn3)) / (2.0 * assign11980_e11133)))), (0.5 * (var_t0_dn4 + ((((var_t0_dn4 * assign11980_e11127) + (assign11980_e11124 * var_t0_dn4)) + (0.08 * var_vfbzb_dn4)) / (2.0 * assign11980_e11133)))), (0.5 * (var_t0_dn5 + ((((var_t0_dn5 * assign11980_e11127) + (assign11980_e11124 * var_t0_dn5)) + (0.08 * var_vfbzb_dn5)) / (2.0 * assign11980_e11133)))), (0.5 * (var_t0_dn6 + ((((var_t0_dn6 * assign11980_e11127) + (assign11980_e11124 * var_t0_dn6)) + (0.08 * var_vfbzb_dn6)) / (2.0 * assign11980_e11133)))), (0.5 * (var_t0_dn7 + ((((var_t0_dn7 * assign11980_e11127) + (assign11980_e11124 * var_t0_dn7)) + (0.08 * var_vfbzb_dn7)) / (2.0 * assign11980_e11133)))), (0.5 * (var_t0_dn8 + ((((var_t0_dn8 * assign11980_e11127) + (assign11980_e11124 * var_t0_dn8)) + (0.08 * var_vfbzb_dn8)) / (2.0 * assign11980_e11133)))),)
    } else {
        (var_voxacc, var_voxacc_dn3, var_voxacc_dn4, var_voxacc_dn5, var_voxacc_dn6, var_voxacc_dn7, var_voxacc_dn8,)
    }
};
        var_voxacc = assign11980_e11137;
        var_voxacc_dn3 = assign11980_e11137_d_n3;
        var_voxacc_dn4 = assign11980_e11137_d_n4;
        var_voxacc_dn5 = assign11980_e11137_d_n5;
        var_voxacc_dn6 = assign11980_e11137_d_n6;
        var_voxacc_dn7 = assign11980_e11137_d_n7;
        var_voxacc_dn8 = assign11980_e11137_d_n8;

        let (assign11990_e11145, assign11990_e11145_d_n3, assign11990_e11145_d_n4, assign11990_e11145_d_n5, assign11990_e11145_d_n6, assign11990_e11145_d_n7, assign11990_e11145_d_n8,) = {
    if (var_guard121 != 0.0) {
        let assign11990_e11142: f64 = (var_bigbacc_i * var_voxacc);
        let assign11990_e11143: f64 = (var_aigbacc_i - assign11990_e11142);
        (assign11990_e11143, (-(var_bigbacc_i * var_voxacc_dn3)), (-(var_bigbacc_i * var_voxacc_dn4)), (-(var_bigbacc_i * var_voxacc_dn5)), (-(var_bigbacc_i * var_voxacc_dn6)), (-(var_bigbacc_i * var_voxacc_dn7)), (-(var_bigbacc_i * var_voxacc_dn8)),)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8,)
    }
};
        var_t2 = assign11990_e11145;
        var_t2_dn3 = assign11990_e11145_d_n3;
        var_t2_dn4 = assign11990_e11145_d_n4;
        var_t2_dn5 = assign11990_e11145_d_n5;
        var_t2_dn6 = assign11990_e11145_d_n6;
        var_t2_dn7 = assign11990_e11145_d_n7;
        var_t2_dn8 = assign11990_e11145_d_n8;

        let (assign12000_e11153, assign12000_e11153_d_n3, assign12000_e11153_d_n4, assign12000_e11153_d_n5, assign12000_e11153_d_n6, assign12000_e11153_d_n7, assign12000_e11153_d_n8,) = {
    if (var_guard121 != 0.0) {
        let assign12000_e11150: f64 = (var_cigbacc_i * var_voxacc);
        let assign12000_e11151: f64 = (1.0 + assign12000_e11150);
        (assign12000_e11151, (var_cigbacc_i * var_voxacc_dn3), (var_cigbacc_i * var_voxacc_dn4), (var_cigbacc_i * var_voxacc_dn5), (var_cigbacc_i * var_voxacc_dn6), (var_cigbacc_i * var_voxacc_dn7), (var_cigbacc_i * var_voxacc_dn8),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8,)
    }
};
        var_t3 = assign12000_e11153;
        var_t3_dn3 = assign12000_e11153_d_n3;
        var_t3_dn4 = assign12000_e11153_d_n4;
        var_t3_dn5 = assign12000_e11153_d_n5;
        var_t3_dn6 = assign12000_e11153_d_n6;
        var_t3_dn7 = assign12000_e11153_d_n7;
        var_t3_dn8 = assign12000_e11153_d_n8;

        let (assign12010_e11164, assign12010_e11164_d_n3, assign12010_e11164_d_n4, assign12010_e11164_d_n5, assign12010_e11164_d_n6, assign12010_e11164_d_n7, assign12010_e11164_d_n8,) = {
    if (var_guard121 != 0.0) {
        let assign12010_e11156: f64 = (-745669000000.0);
        let assign12010_e11158: f64 = (assign12010_e11156 * p.p99);
        let assign12010_e11160: f64 = (assign12010_e11158 * var_t2);
        let assign12010_e11162: f64 = (assign12010_e11160 * var_t3);
        (assign12010_e11162, (((assign12010_e11158 * var_t2_dn3) * var_t3) + (assign12010_e11160 * var_t3_dn3)), (((assign12010_e11158 * var_t2_dn4) * var_t3) + (assign12010_e11160 * var_t3_dn4)), (((assign12010_e11158 * var_t2_dn5) * var_t3) + (assign12010_e11160 * var_t3_dn5)), (((assign12010_e11158 * var_t2_dn6) * var_t3) + (assign12010_e11160 * var_t3_dn6)), (((assign12010_e11158 * var_t2_dn7) * var_t3) + (assign12010_e11160 * var_t3_dn7)), (((assign12010_e11158 * var_t2_dn8) * var_t3) + (assign12010_e11160 * var_t3_dn8)),)
    } else {
        (var_t4, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8,)
    }
};
        var_t4 = assign12010_e11164;
        var_t4_dn3 = assign12010_e11164_d_n3;
        var_t4_dn4 = assign12010_e11164_d_n4;
        var_t4_dn5 = assign12010_e11164_d_n5;
        var_t4_dn6 = assign12010_e11164_d_n6;
        var_t4_dn7 = assign12010_e11164_d_n7;
        var_t4_dn8 = assign12010_e11164_d_n8;

        let (assign12020_e11169, assign12020_e11169_d_n3, assign12020_e11169_d_n4, assign12020_e11169_d_n5, assign12020_e11169_d_n6, assign12020_e11169_d_n7, assign12020_e11169_d_n8,) = {
    if (var_guard121 != 0.0) {
        let assign12020_e11167: f64 = { let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign12020_e11167, ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn3), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn4), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn5), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn6), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn7), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn8),)
    } else {
        (var_t5, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8,)
    }
};
        var_t5 = assign12020_e11169;
        var_t5_dn3 = assign12020_e11169_d_n3;
        var_t5_dn4 = assign12020_e11169_d_n4;
        var_t5_dn5 = assign12020_e11169_d_n5;
        var_t5_dn6 = assign12020_e11169_d_n6;
        var_t5_dn7 = assign12020_e11169_d_n7;
        var_t5_dn8 = assign12020_e11169_d_n8;

        let (assign12030_e11173, assign12030_e11173_d_n3, assign12030_e11173_d_n4, assign12030_e11173_d_n5, assign12030_e11173_d_n6, assign12030_e11173_d_n7, assign12030_e11173_d_n8,) = {
    if (var_guard121 != 0.0) {
        (4.97232e-7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t6, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8,)
    }
};
        var_t6 = assign12030_e11173;
        var_t6_dn3 = assign12030_e11173_d_n3;
        var_t6_dn4 = assign12030_e11173_d_n4;
        var_t6_dn5 = assign12030_e11173_d_n5;
        var_t6_dn6 = assign12030_e11173_d_n6;
        var_t6_dn7 = assign12030_e11173_d_n7;
        var_t6_dn8 = assign12030_e11173_d_n8;

        let (assign12040_e11189, assign12040_e11189_d_n3, assign12040_e11189_d_n4, assign12040_e11189_d_n5, assign12040_e11189_d_n6, assign12040_e11189_d_n7, assign12040_e11189_d_n8,) = {
    if (var_guard121 != 0.0) {
        let assign12040_e11177: f64 = (var_weff * var_leff);
        let assign12040_e11179: f64 = (assign12040_e11177 * var_t6);
        let assign12040_e11181: f64 = (assign12040_e11179 * var_toxratio);
        let assign12040_e11183: f64 = (assign12040_e11181 * var_vgbg);
        let assign12040_e11185: f64 = (assign12040_e11183 * var_vaux_igbacc);
        let assign12040_e11187: f64 = (assign12040_e11185 * var_t5);
        (assign12040_e11187, (((((((((assign12040_e11177 * var_t6_dn3) * var_toxratio) + (assign12040_e11179 * var_toxratio_dn3)) * var_vgbg) + (assign12040_e11181 * var_vgbg_dn3)) * var_vaux_igbacc) + (assign12040_e11183 * var_vaux_igbacc_dn3)) * var_t5) + (assign12040_e11185 * var_t5_dn3)), ((((((((assign12040_e11177 * var_t6_dn4) * var_toxratio) + (assign12040_e11179 * var_toxratio_dn4)) * var_vgbg) * var_vaux_igbacc) + (assign12040_e11183 * var_vaux_igbacc_dn4)) * var_t5) + (assign12040_e11185 * var_t5_dn4)), ((((((((assign12040_e11177 * var_t6_dn5) * var_toxratio) + (assign12040_e11179 * var_toxratio_dn5)) * var_vgbg) * var_vaux_igbacc) + (assign12040_e11183 * var_vaux_igbacc_dn5)) * var_t5) + (assign12040_e11185 * var_t5_dn5)), ((((((((assign12040_e11177 * var_t6_dn6) * var_toxratio) + (assign12040_e11179 * var_toxratio_dn6)) * var_vgbg) * var_vaux_igbacc) + (assign12040_e11183 * var_vaux_igbacc_dn6)) * var_t5) + (assign12040_e11185 * var_t5_dn6)), ((((((((assign12040_e11177 * var_t6_dn7) * var_toxratio) + (assign12040_e11179 * var_toxratio_dn7)) * var_vgbg) * var_vaux_igbacc) + (assign12040_e11183 * var_vaux_igbacc_dn7)) * var_t5) + (assign12040_e11185 * var_t5_dn7)), (((((((((assign12040_e11177 * var_t6_dn8) * var_toxratio) + (assign12040_e11179 * var_toxratio_dn8)) * var_vgbg) + (assign12040_e11181 * var_vgbg_dn8)) * var_vaux_igbacc) + (assign12040_e11183 * var_vaux_igbacc_dn8)) * var_t5) + (assign12040_e11185 * var_t5_dn8)),)
    } else {
        (var_igbacc, var_igbacc_dn3, var_igbacc_dn4, var_igbacc_dn5, var_igbacc_dn6, var_igbacc_dn7, var_igbacc_dn8,)
    }
};
        var_igbacc = assign12040_e11189;
        var_igbacc_dn3 = assign12040_e11189_d_n3;
        var_igbacc_dn4 = assign12040_e11189_d_n4;
        var_igbacc_dn5 = assign12040_e11189_d_n5;
        var_igbacc_dn6 = assign12040_e11189_d_n6;
        var_igbacc_dn7 = assign12040_e11189_d_n7;
        var_igbacc_dn8 = assign12040_e11189_d_n8;

        let (assign12050_e11195, assign12050_e11195_d_n3, assign12050_e11195_d_n4, assign12050_e11195_d_n5, assign12050_e11195_d_n6, assign12050_e11195_d_n7, assign12050_e11195_d_n8,) = {
    if (var_guard121 != 0.0) {
        let assign12050_e11193: f64 = (var_igbacc * var_igtemp);
        (assign12050_e11193, (var_igbacc_dn3 * var_igtemp), ((var_igbacc_dn4 * var_igtemp) + (var_igbacc * var_igtemp_dn4)), (var_igbacc_dn5 * var_igtemp), (var_igbacc_dn6 * var_igtemp), (var_igbacc_dn7 * var_igtemp), (var_igbacc_dn8 * var_igtemp),)
    } else {
        (var_igbacc, var_igbacc_dn3, var_igbacc_dn4, var_igbacc_dn5, var_igbacc_dn6, var_igbacc_dn7, var_igbacc_dn8,)
    }
};
        var_igbacc = assign12050_e11195;
        var_igbacc_dn3 = assign12050_e11195_d_n3;
        var_igbacc_dn4 = assign12050_e11195_d_n4;
        var_igbacc_dn5 = assign12050_e11195_d_n5;
        var_igbacc_dn6 = assign12050_e11195_d_n6;
        var_igbacc_dn7 = assign12050_e11195_d_n7;
        var_igbacc_dn8 = assign12050_e11195_d_n8;

        let assign12060_e11198: f64 = (0.6 * var_vds_noswap);
        let assign12060_e11200: f64 = (assign12060_e11198 / var_vtm);
        let assign12060_e11201: f64 = (assign12060_e11200).tanh();
        var_t0 = assign12060_e11201;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = ((-((assign12060_e11198 * var_vtm_dn4) / (var_vtm * var_vtm))) / ((assign12060_e11200).cosh() * (assign12060_e11200).cosh()));
        var_t0_dn5 = (((0.6 * var_vds_noswap_dn5) / var_vtm) / ((assign12060_e11200).cosh() * (assign12060_e11200).cosh()));
        var_t0_dn6 = (((0.6 * var_vds_noswap_dn6) / var_vtm) / ((assign12060_e11200).cosh() * (assign12060_e11200).cosh()));
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;

        let assign12070_e11205: f64 = (0.5 * var_t0);
        let assign12070_e11206: f64 = (0.5 + assign12070_e11205);
        var_wf = assign12070_e11206;
        var_wf_dn3 = (0.5 * var_t0_dn3);
        var_wf_dn4 = (0.5 * var_t0_dn4);
        var_wf_dn5 = (0.5 * var_t0_dn5);
        var_wf_dn6 = (0.5 * var_t0_dn6);
        var_wf_dn7 = (0.5 * var_t0_dn7);
        var_wf_dn8 = (0.5 * var_t0_dn8);

        let assign12080_e11209: f64 = (1.0 - var_wf);
        var_wr = assign12080_e11209;
        var_wr_dn3 = (-var_wf_dn3);
        var_wr_dn4 = (-var_wf_dn4);
        var_wr_dn5 = (-var_wf_dn5);
        var_wr_dn6 = (-var_wf_dn6);
        var_wr_dn7 = (-var_wf_dn7);
        var_wr_dn8 = (-var_wf_dn8);

        let assign12090_e11213: f64 = (var_igbinv + var_igbacc);
        let assign12090_e11214: f64 = (var_wf * assign12090_e11213);
        var_igbs = assign12090_e11214;
        var_igbs_dn3 = ((var_wf_dn3 * assign12090_e11213) + (var_wf * (var_igbinv_dn3 + var_igbacc_dn3)));
        var_igbs_dn4 = ((var_wf_dn4 * assign12090_e11213) + (var_wf * (var_igbinv_dn4 + var_igbacc_dn4)));
        var_igbs_dn5 = ((var_wf_dn5 * assign12090_e11213) + (var_wf * (var_igbinv_dn5 + var_igbacc_dn5)));
        var_igbs_dn6 = ((var_wf_dn6 * assign12090_e11213) + (var_wf * (var_igbinv_dn6 + var_igbacc_dn6)));
        var_igbs_dn7 = ((var_wf_dn7 * assign12090_e11213) + (var_wf * (var_igbinv_dn7 + var_igbacc_dn7)));
        var_igbs_dn8 = ((var_wf_dn8 * assign12090_e11213) + (var_wf * (var_igbinv_dn8 + var_igbacc_dn8)));

        let assign12100_e11218: f64 = (var_igbinv + var_igbacc);
        let assign12100_e11219: f64 = (var_wr * assign12100_e11218);
        var_igbd = assign12100_e11219;
        var_igbd_dn3 = ((var_wr_dn3 * assign12100_e11218) + (var_wr * (var_igbinv_dn3 + var_igbacc_dn3)));
        var_igbd_dn4 = ((var_wr_dn4 * assign12100_e11218) + (var_wr * (var_igbinv_dn4 + var_igbacc_dn4)));
        var_igbd_dn5 = ((var_wr_dn5 * assign12100_e11218) + (var_wr * (var_igbinv_dn5 + var_igbacc_dn5)));
        var_igbd_dn6 = ((var_wr_dn6 * assign12100_e11218) + (var_wr * (var_igbinv_dn6 + var_igbacc_dn6)));
        var_igbd_dn7 = ((var_wr_dn7 * assign12100_e11218) + (var_wr * (var_igbinv_dn7 + var_igbacc_dn7)));
        var_igbd_dn8 = ((var_wr_dn8 * assign12100_e11218) + (var_wr * (var_igbinv_dn8 + var_igbacc_dn8)));

        let assign12110_e11222: f64 = if p.p16 != 0.0 { 1.0 } else { 0.0 };
        var_guard123 = assign12110_e11222;

        let (assign12120_e11234, assign12120_e11234_d_n3, assign12120_e11234_d_n4, assign12120_e11234_d_n5, assign12120_e11234_d_n6, assign12120_e11234_d_n7, assign12120_e11234_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12120_e11229: f64 = (var_digc_i * var_phifs);
        let assign12120_e11230: f64 = (var_vgfb1 - assign12120_e11229);
        let assign12120_e11231: f64 = (var_bigc_i * assign12120_e11230);
        let assign12120_e11232: f64 = (var_aigc_i - assign12120_e11231);
        (assign12120_e11232, (-(var_bigc_i * (-(var_digc_i * var_phifs_dn3)))), (-(var_bigc_i * (var_vgfb1_dn4 - (var_digc_i * var_phifs_dn4)))), (-(var_bigc_i * (var_vgfb1_dn5 - (var_digc_i * var_phifs_dn5)))), (-(var_bigc_i * (var_vgfb1_dn6 - (var_digc_i * var_phifs_dn6)))), (-(var_bigc_i * (-(var_digc_i * var_phifs_dn7)))), (-(var_bigc_i * (var_vgfb1_dn8 - (var_digc_i * var_phifs_dn8)))),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign12120_e11234;
        var_t1_dn3 = assign12120_e11234_d_n3;
        var_t1_dn4 = assign12120_e11234_d_n4;
        var_t1_dn5 = assign12120_e11234_d_n5;
        var_t1_dn6 = assign12120_e11234_d_n6;
        var_t1_dn7 = assign12120_e11234_d_n7;
        var_t1_dn8 = assign12120_e11234_d_n8;

        let (assign12130_e11246, assign12130_e11246_d_n3, assign12130_e11246_d_n4, assign12130_e11246_d_n5, assign12130_e11246_d_n6, assign12130_e11246_d_n7, assign12130_e11246_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12130_e11241: f64 = (var_digc_i * var_phifs);
        let assign12130_e11242: f64 = (var_vgfb1 - assign12130_e11241);
        let assign12130_e11243: f64 = (var_cigc_i * assign12130_e11242);
        let assign12130_e11244: f64 = (1.0 + assign12130_e11243);
        (assign12130_e11244, (var_cigc_i * (-(var_digc_i * var_phifs_dn3))), (var_cigc_i * (var_vgfb1_dn4 - (var_digc_i * var_phifs_dn4))), (var_cigc_i * (var_vgfb1_dn5 - (var_digc_i * var_phifs_dn5))), (var_cigc_i * (var_vgfb1_dn6 - (var_digc_i * var_phifs_dn6))), (var_cigc_i * (-(var_digc_i * var_phifs_dn7))), (var_cigc_i * (var_vgfb1_dn8 - (var_digc_i * var_phifs_dn8))),)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8,)
    }
};
        var_t2 = assign12130_e11246;
        var_t2_dn3 = assign12130_e11246_d_n3;
        var_t2_dn4 = assign12130_e11246_d_n4;
        var_t2_dn5 = assign12130_e11246_d_n5;
        var_t2_dn6 = assign12130_e11246_d_n6;
        var_t2_dn7 = assign12130_e11246_d_n7;
        var_t2_dn8 = assign12130_e11246_d_n8;

        let (assign12140_e11257, assign12140_e11257_d_n3, assign12140_e11257_d_n4, assign12140_e11257_d_n5, assign12140_e11257_d_n6, assign12140_e11257_d_n7, assign12140_e11257_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12140_e11249: f64 = (-var_bechvb);
        let assign12140_e11251: f64 = (assign12140_e11249 * p.p99);
        let assign12140_e11253: f64 = (assign12140_e11251 * var_t1);
        let assign12140_e11255: f64 = (assign12140_e11253 * var_t2);
        (assign12140_e11255, (((assign12140_e11251 * var_t1_dn3) * var_t2) + (assign12140_e11253 * var_t2_dn3)), (((assign12140_e11251 * var_t1_dn4) * var_t2) + (assign12140_e11253 * var_t2_dn4)), (((assign12140_e11251 * var_t1_dn5) * var_t2) + (assign12140_e11253 * var_t2_dn5)), (((assign12140_e11251 * var_t1_dn6) * var_t2) + (assign12140_e11253 * var_t2_dn6)), (((assign12140_e11251 * var_t1_dn7) * var_t2) + (assign12140_e11253 * var_t2_dn7)), (((assign12140_e11251 * var_t1_dn8) * var_t2) + (assign12140_e11253 * var_t2_dn8)),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8,)
    }
};
        var_t3 = assign12140_e11257;
        var_t3_dn3 = assign12140_e11257_d_n3;
        var_t3_dn4 = assign12140_e11257_d_n4;
        var_t3_dn5 = assign12140_e11257_d_n5;
        var_t3_dn6 = assign12140_e11257_d_n6;
        var_t3_dn7 = assign12140_e11257_d_n7;
        var_t3_dn8 = assign12140_e11257_d_n8;

        let (assign12150_e11264, assign12150_e11264_d_n3, assign12150_e11264_d_n4, assign12150_e11264_d_n5, assign12150_e11264_d_n6, assign12150_e11264_d_n7, assign12150_e11264_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12150_e11261: f64 = { let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign12150_e11262: f64 = (var_qia * assign12150_e11261);
        (assign12150_e11262, ((var_qia_dn3 * assign12150_e11261) + (var_qia * ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn3))), ((var_qia_dn4 * assign12150_e11261) + (var_qia * ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn4))), ((var_qia_dn5 * assign12150_e11261) + (var_qia * ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn5))), ((var_qia_dn6 * assign12150_e11261) + (var_qia * ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn6))), ((var_qia_dn7 * assign12150_e11261) + (var_qia * ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn7))), ((var_qia_dn8 * assign12150_e11261) + (var_qia * ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn8))),)
    } else {
        (var_t4, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8,)
    }
};
        var_t4 = assign12150_e11264;
        var_t4_dn3 = assign12150_e11264_d_n3;
        var_t4_dn4 = assign12150_e11264_d_n4;
        var_t4_dn5 = assign12150_e11264_d_n5;
        var_t4_dn6 = assign12150_e11264_d_n6;
        var_t4_dn7 = assign12150_e11264_d_n7;
        var_t4_dn8 = assign12150_e11264_d_n8;

        let (assign12160_e11278, assign12160_e11278_d_n3, assign12160_e11278_d_n4, assign12160_e11278_d_n5, assign12160_e11278_d_n6, assign12160_e11278_d_n7, assign12160_e11278_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12160_e11269: f64 = (0.5 * var_vdsx);
        let assign12160_e11270: f64 = (var_vgbg + assign12160_e11269);
        let assign12160_e11274: f64 = (var_vbgs_noswap + var_vbgd_noswap);
        let assign12160_e11275: f64 = (0.5 * assign12160_e11274);
        let assign12160_e11276: f64 = (assign12160_e11270 + assign12160_e11275);
        (assign12160_e11276, (var_vgbg_dn3 + (0.5 * (var_vbgs_noswap_dn3 + var_vbgd_noswap_dn3))), 0.0, ((0.5 * var_vdsx_dn5) + (0.5 * var_vbgd_noswap_dn5)), ((0.5 * var_vdsx_dn6) + (0.5 * var_vbgs_noswap_dn6)), 0.0, var_vgbg_dn8,)
    } else {
        (var_t5, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8,)
    }
};
        var_t5 = assign12160_e11278;
        var_t5_dn3 = assign12160_e11278_d_n3;
        var_t5_dn4 = assign12160_e11278_d_n4;
        var_t5_dn5 = assign12160_e11278_d_n5;
        var_t5_dn6 = assign12160_e11278_d_n6;
        var_t5_dn7 = assign12160_e11278_d_n7;
        var_t5_dn8 = assign12160_e11278_d_n8;

        let (assign12170_e11294, assign12170_e11294_d_n3, assign12170_e11294_d_n4, assign12170_e11294_d_n5, assign12170_e11294_d_n6, assign12170_e11294_d_n7, assign12170_e11294_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12170_e11282: f64 = (var_weff * var_leff);
        let assign12170_e11284: f64 = (assign12170_e11282 * var_aechvb);
        let assign12170_e11286: f64 = (assign12170_e11284 * var_toxratio);
        let assign12170_e11288: f64 = (assign12170_e11286 * var_t4);
        let assign12170_e11290: f64 = (assign12170_e11288 * var_t5);
        let assign12170_e11292: f64 = (assign12170_e11290 * var_igtemp);
        (assign12170_e11292, ((((((assign12170_e11284 * var_toxratio_dn3) * var_t4) + (assign12170_e11286 * var_t4_dn3)) * var_t5) + (assign12170_e11288 * var_t5_dn3)) * var_igtemp), (((((((assign12170_e11284 * var_toxratio_dn4) * var_t4) + (assign12170_e11286 * var_t4_dn4)) * var_t5) + (assign12170_e11288 * var_t5_dn4)) * var_igtemp) + (assign12170_e11290 * var_igtemp_dn4)), ((((((assign12170_e11284 * var_toxratio_dn5) * var_t4) + (assign12170_e11286 * var_t4_dn5)) * var_t5) + (assign12170_e11288 * var_t5_dn5)) * var_igtemp), ((((((assign12170_e11284 * var_toxratio_dn6) * var_t4) + (assign12170_e11286 * var_t4_dn6)) * var_t5) + (assign12170_e11288 * var_t5_dn6)) * var_igtemp), ((((((assign12170_e11284 * var_toxratio_dn7) * var_t4) + (assign12170_e11286 * var_t4_dn7)) * var_t5) + (assign12170_e11288 * var_t5_dn7)) * var_igtemp), ((((((assign12170_e11284 * var_toxratio_dn8) * var_t4) + (assign12170_e11286 * var_t4_dn8)) * var_t5) + (assign12170_e11288 * var_t5_dn8)) * var_igtemp),)
    } else {
        (var_igc0, var_igc0_dn3, var_igc0_dn4, var_igc0_dn5, var_igc0_dn6, var_igc0_dn7, var_igc0_dn8,)
    }
};
        var_igc0 = assign12170_e11294;
        var_igc0_dn3 = assign12170_e11294_d_n3;
        var_igc0_dn4 = assign12170_e11294_d_n4;
        var_igc0_dn5 = assign12170_e11294_d_n5;
        var_igc0_dn6 = assign12170_e11294_d_n6;
        var_igc0_dn7 = assign12170_e11294_d_n7;
        var_igc0_dn8 = assign12170_e11294_d_n8;

        *var_guard122_slot = var_guard122;
        *var_guard123_slot = var_guard123;
        *var_igbacc_slot = var_igbacc;
        *var_igbacc_dn3_slot = var_igbacc_dn3;
        *var_igbacc_dn4_slot = var_igbacc_dn4;
        *var_igbacc_dn5_slot = var_igbacc_dn5;
        *var_igbacc_dn6_slot = var_igbacc_dn6;
        *var_igbacc_dn7_slot = var_igbacc_dn7;
        *var_igbacc_dn8_slot = var_igbacc_dn8;
        *var_igbd_slot = var_igbd;
        *var_igbd_dn3_slot = var_igbd_dn3;
        *var_igbd_dn4_slot = var_igbd_dn4;
        *var_igbd_dn5_slot = var_igbd_dn5;
        *var_igbd_dn6_slot = var_igbd_dn6;
        *var_igbd_dn7_slot = var_igbd_dn7;
        *var_igbd_dn8_slot = var_igbd_dn8;
        *var_igbinv_slot = var_igbinv;
        *var_igbinv_dn3_slot = var_igbinv_dn3;
        *var_igbinv_dn4_slot = var_igbinv_dn4;
        *var_igbinv_dn5_slot = var_igbinv_dn5;
        *var_igbinv_dn6_slot = var_igbinv_dn6;
        *var_igbinv_dn7_slot = var_igbinv_dn7;
        *var_igbinv_dn8_slot = var_igbinv_dn8;
        *var_igbs_slot = var_igbs;
        *var_igbs_dn3_slot = var_igbs_dn3;
        *var_igbs_dn4_slot = var_igbs_dn4;
        *var_igbs_dn5_slot = var_igbs_dn5;
        *var_igbs_dn6_slot = var_igbs_dn6;
        *var_igbs_dn7_slot = var_igbs_dn7;
        *var_igbs_dn8_slot = var_igbs_dn8;
        *var_igc0_slot = var_igc0;
        *var_igc0_dn3_slot = var_igc0_dn3;
        *var_igc0_dn4_slot = var_igc0_dn4;
        *var_igc0_dn5_slot = var_igc0_dn5;
        *var_igc0_dn6_slot = var_igc0_dn6;
        *var_igc0_dn7_slot = var_igc0_dn7;
        *var_igc0_dn8_slot = var_igc0_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t4_slot = var_t4;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t5_slot = var_t5;
        *var_t5_dn3_slot = var_t5_dn3;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t6_slot = var_t6;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_vaux_igbacc_slot = var_vaux_igbacc;
        *var_vaux_igbacc_dn3_slot = var_vaux_igbacc_dn3;
        *var_vaux_igbacc_dn4_slot = var_vaux_igbacc_dn4;
        *var_vaux_igbacc_dn5_slot = var_vaux_igbacc_dn5;
        *var_vaux_igbacc_dn6_slot = var_vaux_igbacc_dn6;
        *var_vaux_igbacc_dn7_slot = var_vaux_igbacc_dn7;
        *var_vaux_igbacc_dn8_slot = var_vaux_igbacc_dn8;
        *var_vfbzb_slot = var_vfbzb;
        *var_vfbzb_dn3_slot = var_vfbzb_dn3;
        *var_vfbzb_dn4_slot = var_vfbzb_dn4;
        *var_vfbzb_dn5_slot = var_vfbzb_dn5;
        *var_vfbzb_dn6_slot = var_vfbzb_dn6;
        *var_vfbzb_dn7_slot = var_vfbzb_dn7;
        *var_vfbzb_dn8_slot = var_vfbzb_dn8;
        *var_voxacc_slot = var_voxacc;
        *var_voxacc_dn3_slot = var_voxacc_dn3;
        *var_voxacc_dn4_slot = var_voxacc_dn4;
        *var_voxacc_dn5_slot = var_voxacc_dn5;
        *var_voxacc_dn6_slot = var_voxacc_dn6;
        *var_voxacc_dn7_slot = var_voxacc_dn7;
        *var_voxacc_dn8_slot = var_voxacc_dn8;
        *var_wf_slot = var_wf;
        *var_wf_dn3_slot = var_wf_dn3;
        *var_wf_dn4_slot = var_wf_dn4;
        *var_wf_dn5_slot = var_wf_dn5;
        *var_wf_dn6_slot = var_wf_dn6;
        *var_wf_dn7_slot = var_wf_dn7;
        *var_wf_dn8_slot = var_wf_dn8;
        *var_wr_slot = var_wr;
        *var_wr_dn3_slot = var_wr_dn3;
        *var_wr_dn4_slot = var_wr_dn4;
        *var_wr_dn5_slot = var_wr_dn5;
        *var_wr_dn6_slot = var_wr_dn6;
        *var_wr_dn7_slot = var_wr_dn7;
        *var_wr_dn8_slot = var_wr_dn8;
    }

    pub(super) fn stamp_transient_block_27(
        p: &Parameters,
        var_agidl_i: f64,
        var_aigd_i: f64,
        var_aigs_i: f64,
        var_bechvb: f64,
        var_bgidl_t: f64,
        var_bigd_i: f64,
        var_bigs_i: f64,
        var_cigd_i: f64,
        var_cigs_i: f64,
        var_digd_i: f64,
        var_digs_i: f64,
        var_epsratio: f64,
        var_gamma0: f64,
        var_guard123: f64,
        var_igc0: f64,
        var_igc0_dn3: f64,
        var_igc0_dn4: f64,
        var_igc0_dn5: f64,
        var_igc0_dn6: f64,
        var_igc0_dn7: f64,
        var_igc0_dn8: f64,
        var_igsd_mult: f64,
        var_igsd_mult_dn3: f64,
        var_igsd_mult_dn4: f64,
        var_igsd_mult_dn5: f64,
        var_igsd_mult_dn6: f64,
        var_igsd_mult_dn7: f64,
        var_igsd_mult_dn8: f64,
        var_pigcd_i: f64,
        var_poxedge_i: f64,
        var_sigvds: f64,
        var_vbgs: f64,
        var_vbgs_dn3: f64,
        var_vbgs_dn5: f64,
        var_vbgs_dn6: f64,
        var_vdseff: f64,
        var_vdseff_dn3: f64,
        var_vdseff_dn4: f64,
        var_vdseff_dn5: f64,
        var_vdseff_dn6: f64,
        var_vdseff_dn7: f64,
        var_vdseff_dn8: f64,
        var_vfbsd: f64,
        var_vfbsd_bg: f64,
        var_vfbsd_bg_dn3: f64,
        var_vfbsd_bg_dn4: f64,
        var_vfbsd_bg_dn5: f64,
        var_vfbsd_bg_dn6: f64,
        var_vfbsd_bg_dn7: f64,
        var_vfbsd_bg_dn8: f64,
        var_vfbsd_dn3: f64,
        var_vfbsd_dn4: f64,
        var_vfbsd_dn5: f64,
        var_vfbsd_dn6: f64,
        var_vfbsd_dn7: f64,
        var_vfbsd_dn8: f64,
        var_vgd_noswap: f64,
        var_vgd_noswap_dn5: f64,
        var_vgd_noswap_dn8: f64,
        var_vgs_noswap: f64,
        var_vgs_noswap_dn6: f64,
        var_vgs_noswap_dn8: f64,
        var_guard124_slot: &mut f64,
        var_guard125_slot: &mut f64,
        var_guard126_slot: &mut f64,
        var_guard127_slot: &mut f64,
        var_igcd_slot: &mut f64,
        var_igcd_dn3_slot: &mut f64,
        var_igcd_dn4_slot: &mut f64,
        var_igcd_dn5_slot: &mut f64,
        var_igcd_dn6_slot: &mut f64,
        var_igcd_dn7_slot: &mut f64,
        var_igcd_dn8_slot: &mut f64,
        var_igcs_slot: &mut f64,
        var_igcs_dn3_slot: &mut f64,
        var_igcs_dn4_slot: &mut f64,
        var_igcs_dn5_slot: &mut f64,
        var_igcs_dn6_slot: &mut f64,
        var_igcs_dn7_slot: &mut f64,
        var_igcs_dn8_slot: &mut f64,
        var_igd_slot: &mut f64,
        var_igd_dn3_slot: &mut f64,
        var_igd_dn4_slot: &mut f64,
        var_igd_dn5_slot: &mut f64,
        var_igd_dn6_slot: &mut f64,
        var_igd_dn7_slot: &mut f64,
        var_igd_dn8_slot: &mut f64,
        var_igidl_slot: &mut f64,
        var_igidl_dn3_slot: &mut f64,
        var_igidl_dn4_slot: &mut f64,
        var_igidl_dn5_slot: &mut f64,
        var_igidl_dn6_slot: &mut f64,
        var_igidl_dn7_slot: &mut f64,
        var_igidl_dn8_slot: &mut f64,
        var_igisl_slot: &mut f64,
        var_igisl_dn3_slot: &mut f64,
        var_igisl_dn4_slot: &mut f64,
        var_igisl_dn5_slot: &mut f64,
        var_igisl_dn6_slot: &mut f64,
        var_igisl_dn7_slot: &mut f64,
        var_igisl_dn8_slot: &mut f64,
        var_igs_slot: &mut f64,
        var_igs_dn3_slot: &mut f64,
        var_igs_dn4_slot: &mut f64,
        var_igs_dn5_slot: &mut f64,
        var_igs_dn6_slot: &mut f64,
        var_igs_dn7_slot: &mut f64,
        var_igs_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_exp_slot: &mut f64,
        var_t1_exp_dn3_slot: &mut f64,
        var_t1_exp_dn4_slot: &mut f64,
        var_t1_exp_dn5_slot: &mut f64,
        var_t1_exp_dn6_slot: &mut f64,
        var_t1_exp_dn7_slot: &mut f64,
        var_t1_exp_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_vdseffx_slot: &mut f64,
        var_vdseffx_dn3_slot: &mut f64,
        var_vdseffx_dn4_slot: &mut f64,
        var_vdseffx_dn5_slot: &mut f64,
        var_vdseffx_dn6_slot: &mut f64,
        var_vdseffx_dn7_slot: &mut f64,
        var_vdseffx_dn8_slot: &mut f64,
        var_vfgd_eff_slot: &mut f64,
        var_vfgd_eff_dn3_slot: &mut f64,
        var_vfgd_eff_dn4_slot: &mut f64,
        var_vfgd_eff_dn5_slot: &mut f64,
        var_vfgd_eff_dn6_slot: &mut f64,
        var_vfgd_eff_dn7_slot: &mut f64,
        var_vfgd_eff_dn8_slot: &mut f64,
        var_vfgs_eff_slot: &mut f64,
        var_vfgs_eff_dn3_slot: &mut f64,
        var_vfgs_eff_dn4_slot: &mut f64,
        var_vfgs_eff_dn5_slot: &mut f64,
        var_vfgs_eff_dn6_slot: &mut f64,
        var_vfgs_eff_dn7_slot: &mut f64,
        var_vfgs_eff_dn8_slot: &mut f64,
    ) {
        let mut var_guard124: f64 = *var_guard124_slot;
        let mut var_guard125: f64 = *var_guard125_slot;
        let mut var_guard126: f64 = *var_guard126_slot;
        let mut var_guard127: f64 = *var_guard127_slot;
        let mut var_igcd: f64 = *var_igcd_slot;
        let mut var_igcd_dn3: f64 = *var_igcd_dn3_slot;
        let mut var_igcd_dn4: f64 = *var_igcd_dn4_slot;
        let mut var_igcd_dn5: f64 = *var_igcd_dn5_slot;
        let mut var_igcd_dn6: f64 = *var_igcd_dn6_slot;
        let mut var_igcd_dn7: f64 = *var_igcd_dn7_slot;
        let mut var_igcd_dn8: f64 = *var_igcd_dn8_slot;
        let mut var_igcs: f64 = *var_igcs_slot;
        let mut var_igcs_dn3: f64 = *var_igcs_dn3_slot;
        let mut var_igcs_dn4: f64 = *var_igcs_dn4_slot;
        let mut var_igcs_dn5: f64 = *var_igcs_dn5_slot;
        let mut var_igcs_dn6: f64 = *var_igcs_dn6_slot;
        let mut var_igcs_dn7: f64 = *var_igcs_dn7_slot;
        let mut var_igcs_dn8: f64 = *var_igcs_dn8_slot;
        let mut var_igd: f64 = *var_igd_slot;
        let mut var_igd_dn3: f64 = *var_igd_dn3_slot;
        let mut var_igd_dn4: f64 = *var_igd_dn4_slot;
        let mut var_igd_dn5: f64 = *var_igd_dn5_slot;
        let mut var_igd_dn6: f64 = *var_igd_dn6_slot;
        let mut var_igd_dn7: f64 = *var_igd_dn7_slot;
        let mut var_igd_dn8: f64 = *var_igd_dn8_slot;
        let mut var_igidl: f64 = *var_igidl_slot;
        let mut var_igidl_dn3: f64 = *var_igidl_dn3_slot;
        let mut var_igidl_dn4: f64 = *var_igidl_dn4_slot;
        let mut var_igidl_dn5: f64 = *var_igidl_dn5_slot;
        let mut var_igidl_dn6: f64 = *var_igidl_dn6_slot;
        let mut var_igidl_dn7: f64 = *var_igidl_dn7_slot;
        let mut var_igidl_dn8: f64 = *var_igidl_dn8_slot;
        let mut var_igisl: f64 = *var_igisl_slot;
        let mut var_igisl_dn3: f64 = *var_igisl_dn3_slot;
        let mut var_igisl_dn4: f64 = *var_igisl_dn4_slot;
        let mut var_igisl_dn5: f64 = *var_igisl_dn5_slot;
        let mut var_igisl_dn6: f64 = *var_igisl_dn6_slot;
        let mut var_igisl_dn7: f64 = *var_igisl_dn7_slot;
        let mut var_igisl_dn8: f64 = *var_igisl_dn8_slot;
        let mut var_igs: f64 = *var_igs_slot;
        let mut var_igs_dn3: f64 = *var_igs_dn3_slot;
        let mut var_igs_dn4: f64 = *var_igs_dn4_slot;
        let mut var_igs_dn5: f64 = *var_igs_dn5_slot;
        let mut var_igs_dn6: f64 = *var_igs_dn6_slot;
        let mut var_igs_dn7: f64 = *var_igs_dn7_slot;
        let mut var_igs_dn8: f64 = *var_igs_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_exp: f64 = *var_t1_exp_slot;
        let mut var_t1_exp_dn3: f64 = *var_t1_exp_dn3_slot;
        let mut var_t1_exp_dn4: f64 = *var_t1_exp_dn4_slot;
        let mut var_t1_exp_dn5: f64 = *var_t1_exp_dn5_slot;
        let mut var_t1_exp_dn6: f64 = *var_t1_exp_dn6_slot;
        let mut var_t1_exp_dn7: f64 = *var_t1_exp_dn7_slot;
        let mut var_t1_exp_dn8: f64 = *var_t1_exp_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_vdseffx: f64 = *var_vdseffx_slot;
        let mut var_vdseffx_dn3: f64 = *var_vdseffx_dn3_slot;
        let mut var_vdseffx_dn4: f64 = *var_vdseffx_dn4_slot;
        let mut var_vdseffx_dn5: f64 = *var_vdseffx_dn5_slot;
        let mut var_vdseffx_dn6: f64 = *var_vdseffx_dn6_slot;
        let mut var_vdseffx_dn7: f64 = *var_vdseffx_dn7_slot;
        let mut var_vdseffx_dn8: f64 = *var_vdseffx_dn8_slot;
        let mut var_vfgd_eff: f64 = *var_vfgd_eff_slot;
        let mut var_vfgd_eff_dn3: f64 = *var_vfgd_eff_dn3_slot;
        let mut var_vfgd_eff_dn4: f64 = *var_vfgd_eff_dn4_slot;
        let mut var_vfgd_eff_dn5: f64 = *var_vfgd_eff_dn5_slot;
        let mut var_vfgd_eff_dn6: f64 = *var_vfgd_eff_dn6_slot;
        let mut var_vfgd_eff_dn7: f64 = *var_vfgd_eff_dn7_slot;
        let mut var_vfgd_eff_dn8: f64 = *var_vfgd_eff_dn8_slot;
        let mut var_vfgs_eff: f64 = *var_vfgs_eff_slot;
        let mut var_vfgs_eff_dn3: f64 = *var_vfgs_eff_dn3_slot;
        let mut var_vfgs_eff_dn4: f64 = *var_vfgs_eff_dn4_slot;
        let mut var_vfgs_eff_dn5: f64 = *var_vfgs_eff_dn5_slot;
        let mut var_vfgs_eff_dn6: f64 = *var_vfgs_eff_dn6_slot;
        let mut var_vfgs_eff_dn7: f64 = *var_vfgs_eff_dn7_slot;
        let mut var_vfgs_eff_dn8: f64 = *var_vfgs_eff_dn8_slot;

        let (assign12180_e11305, assign12180_e11305_d_n3, assign12180_e11305_d_n4, assign12180_e11305_d_n5, assign12180_e11305_d_n6, assign12180_e11305_d_n7, assign12180_e11305_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12180_e11298: f64 = (var_vdseff * var_vdseff);
        let assign12180_e11300: f64 = (assign12180_e11298 + 0.01);
        let assign12180_e11301: f64 = (assign12180_e11300).sqrt();
        let assign12180_e11303: f64 = (assign12180_e11301 - 0.1);
        (assign12180_e11303, (((var_vdseff_dn3 * var_vdseff) + (var_vdseff * var_vdseff_dn3)) / (2.0 * assign12180_e11301)), (((var_vdseff_dn4 * var_vdseff) + (var_vdseff * var_vdseff_dn4)) / (2.0 * assign12180_e11301)), (((var_vdseff_dn5 * var_vdseff) + (var_vdseff * var_vdseff_dn5)) / (2.0 * assign12180_e11301)), (((var_vdseff_dn6 * var_vdseff) + (var_vdseff * var_vdseff_dn6)) / (2.0 * assign12180_e11301)), (((var_vdseff_dn7 * var_vdseff) + (var_vdseff * var_vdseff_dn7)) / (2.0 * assign12180_e11301)), (((var_vdseff_dn8 * var_vdseff) + (var_vdseff * var_vdseff_dn8)) / (2.0 * assign12180_e11301)),)
    } else {
        (var_vdseffx, var_vdseffx_dn3, var_vdseffx_dn4, var_vdseffx_dn5, var_vdseffx_dn6, var_vdseffx_dn7, var_vdseffx_dn8,)
    }
};
        var_vdseffx = assign12180_e11305;
        var_vdseffx_dn3 = assign12180_e11305_d_n3;
        var_vdseffx_dn4 = assign12180_e11305_d_n4;
        var_vdseffx_dn5 = assign12180_e11305_d_n5;
        var_vdseffx_dn6 = assign12180_e11305_d_n6;
        var_vdseffx_dn7 = assign12180_e11305_d_n7;
        var_vdseffx_dn8 = assign12180_e11305_d_n8;

        let (assign12190_e11311, assign12190_e11311_d_n3, assign12190_e11311_d_n4, assign12190_e11311_d_n5, assign12190_e11311_d_n6, assign12190_e11311_d_n7, assign12190_e11311_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12190_e11309: f64 = (var_pigcd_i * var_vdseffx);
        (assign12190_e11309, (var_pigcd_i * var_vdseffx_dn3), (var_pigcd_i * var_vdseffx_dn4), (var_pigcd_i * var_vdseffx_dn5), (var_pigcd_i * var_vdseffx_dn6), (var_pigcd_i * var_vdseffx_dn7), (var_pigcd_i * var_vdseffx_dn8),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign12190_e11311;
        var_t1_dn3 = assign12190_e11311_d_n3;
        var_t1_dn4 = assign12190_e11311_d_n4;
        var_t1_dn5 = assign12190_e11311_d_n5;
        var_t1_dn6 = assign12190_e11311_d_n6;
        var_t1_dn7 = assign12190_e11311_d_n7;
        var_t1_dn8 = assign12190_e11311_d_n8;

        let (assign12200_e11317, assign12200_e11317_d_n3, assign12200_e11317_d_n4, assign12200_e11317_d_n5, assign12200_e11317_d_n6, assign12200_e11317_d_n7, assign12200_e11317_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12200_e11314: f64 = (-var_t1);
        let assign12200_e11315: f64 = { let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign12200_e11315, ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_dn3)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_dn4)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_dn5)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_dn6)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_dn7)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t1_dn8)),)
    } else {
        (var_t1_exp, var_t1_exp_dn3, var_t1_exp_dn4, var_t1_exp_dn5, var_t1_exp_dn6, var_t1_exp_dn7, var_t1_exp_dn8,)
    }
};
        var_t1_exp = assign12200_e11317;
        var_t1_exp_dn3 = assign12200_e11317_d_n3;
        var_t1_exp_dn4 = assign12200_e11317_d_n4;
        var_t1_exp_dn5 = assign12200_e11317_d_n5;
        var_t1_exp_dn6 = assign12200_e11317_d_n6;
        var_t1_exp_dn7 = assign12200_e11317_d_n7;
        var_t1_exp_dn8 = assign12200_e11317_d_n8;

        let (assign12210_e11327, assign12210_e11327_d_n3, assign12210_e11327_d_n4, assign12210_e11327_d_n5, assign12210_e11327_d_n6, assign12210_e11327_d_n7, assign12210_e11327_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12210_e11321: f64 = (var_t1 + var_t1_exp);
        let assign12210_e11323: f64 = (assign12210_e11321 - 1.0);
        let assign12210_e11325: f64 = (assign12210_e11323 + 0.0001);
        (assign12210_e11325, (var_t1_dn3 + var_t1_exp_dn3), (var_t1_dn4 + var_t1_exp_dn4), (var_t1_dn5 + var_t1_exp_dn5), (var_t1_dn6 + var_t1_exp_dn6), (var_t1_dn7 + var_t1_exp_dn7), (var_t1_dn8 + var_t1_exp_dn8),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8,)
    }
};
        var_t3 = assign12210_e11327;
        var_t3_dn3 = assign12210_e11327_d_n3;
        var_t3_dn4 = assign12210_e11327_d_n4;
        var_t3_dn5 = assign12210_e11327_d_n5;
        var_t3_dn6 = assign12210_e11327_d_n6;
        var_t3_dn7 = assign12210_e11327_d_n7;
        var_t3_dn8 = assign12210_e11327_d_n8;

        let (assign12220_e11339, assign12220_e11339_d_n3, assign12220_e11339_d_n4, assign12220_e11339_d_n5, assign12220_e11339_d_n6, assign12220_e11339_d_n7, assign12220_e11339_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12220_e11332: f64 = (var_t1 + 1.0);
        let assign12220_e11334: f64 = (assign12220_e11332 * var_t1_exp);
        let assign12220_e11335: f64 = (1.0 - assign12220_e11334);
        let assign12220_e11337: f64 = (assign12220_e11335 + 0.0001);
        (assign12220_e11337, (-((var_t1_dn3 * var_t1_exp) + (assign12220_e11332 * var_t1_exp_dn3))), (-((var_t1_dn4 * var_t1_exp) + (assign12220_e11332 * var_t1_exp_dn4))), (-((var_t1_dn5 * var_t1_exp) + (assign12220_e11332 * var_t1_exp_dn5))), (-((var_t1_dn6 * var_t1_exp) + (assign12220_e11332 * var_t1_exp_dn6))), (-((var_t1_dn7 * var_t1_exp) + (assign12220_e11332 * var_t1_exp_dn7))), (-((var_t1_dn8 * var_t1_exp) + (assign12220_e11332 * var_t1_exp_dn8))),)
    } else {
        (var_t4, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8,)
    }
};
        var_t4 = assign12220_e11339;
        var_t4_dn3 = assign12220_e11339_d_n3;
        var_t4_dn4 = assign12220_e11339_d_n4;
        var_t4_dn5 = assign12220_e11339_d_n5;
        var_t4_dn6 = assign12220_e11339_d_n6;
        var_t4_dn7 = assign12220_e11339_d_n7;
        var_t4_dn8 = assign12220_e11339_d_n8;

        let (assign12230_e11347, assign12230_e11347_d_n3, assign12230_e11347_d_n4, assign12230_e11347_d_n5, assign12230_e11347_d_n6, assign12230_e11347_d_n7, assign12230_e11347_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12230_e11343: f64 = (var_t1 * var_t1);
        let assign12230_e11345: f64 = (assign12230_e11343 + 0.0002);
        (assign12230_e11345, ((var_t1_dn3 * var_t1) + (var_t1 * var_t1_dn3)), ((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)), ((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)), ((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)), ((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)), ((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)),)
    } else {
        (var_t5, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8,)
    }
};
        var_t5 = assign12230_e11347;
        var_t5_dn3 = assign12230_e11347_d_n3;
        var_t5_dn4 = assign12230_e11347_d_n4;
        var_t5_dn5 = assign12230_e11347_d_n5;
        var_t5_dn6 = assign12230_e11347_d_n6;
        var_t5_dn7 = assign12230_e11347_d_n7;
        var_t5_dn8 = assign12230_e11347_d_n8;

        let (assign12240_e11355, assign12240_e11355_d_n3, assign12240_e11355_d_n4, assign12240_e11355_d_n5, assign12240_e11355_d_n6, assign12240_e11355_d_n7, assign12240_e11355_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12240_e11351: f64 = (var_igc0 * var_t4);
        let assign12240_e11353: f64 = (assign12240_e11351 / var_t5);
        (assign12240_e11353, (((((var_igc0_dn3 * var_t4) + (var_igc0 * var_t4_dn3)) * var_t5) - (assign12240_e11351 * var_t5_dn3)) / (var_t5 * var_t5)), (((((var_igc0_dn4 * var_t4) + (var_igc0 * var_t4_dn4)) * var_t5) - (assign12240_e11351 * var_t5_dn4)) / (var_t5 * var_t5)), (((((var_igc0_dn5 * var_t4) + (var_igc0 * var_t4_dn5)) * var_t5) - (assign12240_e11351 * var_t5_dn5)) / (var_t5 * var_t5)), (((((var_igc0_dn6 * var_t4) + (var_igc0 * var_t4_dn6)) * var_t5) - (assign12240_e11351 * var_t5_dn6)) / (var_t5 * var_t5)), (((((var_igc0_dn7 * var_t4) + (var_igc0 * var_t4_dn7)) * var_t5) - (assign12240_e11351 * var_t5_dn7)) / (var_t5 * var_t5)), (((((var_igc0_dn8 * var_t4) + (var_igc0 * var_t4_dn8)) * var_t5) - (assign12240_e11351 * var_t5_dn8)) / (var_t5 * var_t5)),)
    } else {
        (var_igcd, var_igcd_dn3, var_igcd_dn4, var_igcd_dn5, var_igcd_dn6, var_igcd_dn7, var_igcd_dn8,)
    }
};
        var_igcd = assign12240_e11355;
        var_igcd_dn3 = assign12240_e11355_d_n3;
        var_igcd_dn4 = assign12240_e11355_d_n4;
        var_igcd_dn5 = assign12240_e11355_d_n5;
        var_igcd_dn6 = assign12240_e11355_d_n6;
        var_igcd_dn7 = assign12240_e11355_d_n7;
        var_igcd_dn8 = assign12240_e11355_d_n8;

        let (assign12250_e11363, assign12250_e11363_d_n3, assign12250_e11363_d_n4, assign12250_e11363_d_n5, assign12250_e11363_d_n6, assign12250_e11363_d_n7, assign12250_e11363_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12250_e11359: f64 = (var_igc0 * var_t3);
        let assign12250_e11361: f64 = (assign12250_e11359 / var_t5);
        (assign12250_e11361, (((((var_igc0_dn3 * var_t3) + (var_igc0 * var_t3_dn3)) * var_t5) - (assign12250_e11359 * var_t5_dn3)) / (var_t5 * var_t5)), (((((var_igc0_dn4 * var_t3) + (var_igc0 * var_t3_dn4)) * var_t5) - (assign12250_e11359 * var_t5_dn4)) / (var_t5 * var_t5)), (((((var_igc0_dn5 * var_t3) + (var_igc0 * var_t3_dn5)) * var_t5) - (assign12250_e11359 * var_t5_dn5)) / (var_t5 * var_t5)), (((((var_igc0_dn6 * var_t3) + (var_igc0 * var_t3_dn6)) * var_t5) - (assign12250_e11359 * var_t5_dn6)) / (var_t5 * var_t5)), (((((var_igc0_dn7 * var_t3) + (var_igc0 * var_t3_dn7)) * var_t5) - (assign12250_e11359 * var_t5_dn7)) / (var_t5 * var_t5)), (((((var_igc0_dn8 * var_t3) + (var_igc0 * var_t3_dn8)) * var_t5) - (assign12250_e11359 * var_t5_dn8)) / (var_t5 * var_t5)),)
    } else {
        (var_igcs, var_igcs_dn3, var_igcs_dn4, var_igcs_dn5, var_igcs_dn6, var_igcs_dn7, var_igcs_dn8,)
    }
};
        var_igcs = assign12250_e11363;
        var_igcs_dn3 = assign12250_e11363_d_n3;
        var_igcs_dn4 = assign12250_e11363_d_n4;
        var_igcs_dn5 = assign12250_e11363_d_n5;
        var_igcs_dn6 = assign12250_e11363_d_n6;
        var_igcs_dn7 = assign12250_e11363_d_n7;
        var_igcs_dn8 = assign12250_e11363_d_n8;

        let (assign12260_e11377, assign12260_e11377_d_n3, assign12260_e11377_d_n4, assign12260_e11377_d_n5, assign12260_e11377_d_n6, assign12260_e11377_d_n7, assign12260_e11377_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12260_e11367: f64 = (var_vgs_noswap - var_vfbsd);
        let assign12260_e11370: f64 = (var_digs_i * var_gamma0);
        let assign12260_e11373: f64 = (var_vbgs - var_vfbsd_bg);
        let assign12260_e11374: f64 = (assign12260_e11370 * assign12260_e11373);
        let assign12260_e11375: f64 = (assign12260_e11367 + assign12260_e11374);
        (assign12260_e11375, ((-var_vfbsd_dn3) + (assign12260_e11370 * (var_vbgs_dn3 - var_vfbsd_bg_dn3))), ((-var_vfbsd_dn4) + (assign12260_e11370 * (-var_vfbsd_bg_dn4))), ((-var_vfbsd_dn5) + (assign12260_e11370 * (var_vbgs_dn5 - var_vfbsd_bg_dn5))), ((var_vgs_noswap_dn6 - var_vfbsd_dn6) + (assign12260_e11370 * (var_vbgs_dn6 - var_vfbsd_bg_dn6))), ((-var_vfbsd_dn7) + (assign12260_e11370 * (-var_vfbsd_bg_dn7))), ((var_vgs_noswap_dn8 - var_vfbsd_dn8) + (assign12260_e11370 * (-var_vfbsd_bg_dn8))),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign12260_e11377;
        var_t0_dn3 = assign12260_e11377_d_n3;
        var_t0_dn4 = assign12260_e11377_d_n4;
        var_t0_dn5 = assign12260_e11377_d_n5;
        var_t0_dn6 = assign12260_e11377_d_n6;
        var_t0_dn7 = assign12260_e11377_d_n7;
        var_t0_dn8 = assign12260_e11377_d_n8;

        let (assign12270_e11386, assign12270_e11386_d_n3, assign12270_e11386_d_n4, assign12270_e11386_d_n5, assign12270_e11386_d_n6, assign12270_e11386_d_n7, assign12270_e11386_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12270_e11381: f64 = (var_t0 * var_t0);
        let assign12270_e11383: f64 = (assign12270_e11381 + 0.0001);
        let assign12270_e11384: f64 = (assign12270_e11383).sqrt();
        (assign12270_e11384, (((var_t0_dn3 * var_t0) + (var_t0 * var_t0_dn3)) / (2.0 * assign12270_e11384)), (((var_t0_dn4 * var_t0) + (var_t0 * var_t0_dn4)) / (2.0 * assign12270_e11384)), (((var_t0_dn5 * var_t0) + (var_t0 * var_t0_dn5)) / (2.0 * assign12270_e11384)), (((var_t0_dn6 * var_t0) + (var_t0 * var_t0_dn6)) / (2.0 * assign12270_e11384)), (((var_t0_dn7 * var_t0) + (var_t0 * var_t0_dn7)) / (2.0 * assign12270_e11384)), (((var_t0_dn8 * var_t0) + (var_t0 * var_t0_dn8)) / (2.0 * assign12270_e11384)),)
    } else {
        (var_vfgs_eff, var_vfgs_eff_dn3, var_vfgs_eff_dn4, var_vfgs_eff_dn5, var_vfgs_eff_dn6, var_vfgs_eff_dn7, var_vfgs_eff_dn8,)
    }
};
        var_vfgs_eff = assign12270_e11386;
        var_vfgs_eff_dn3 = assign12270_e11386_d_n3;
        var_vfgs_eff_dn4 = assign12270_e11386_d_n4;
        var_vfgs_eff_dn5 = assign12270_e11386_d_n5;
        var_vfgs_eff_dn6 = assign12270_e11386_d_n6;
        var_vfgs_eff_dn7 = assign12270_e11386_d_n7;
        var_vfgs_eff_dn8 = assign12270_e11386_d_n8;

        let (assign12280_e11394, assign12280_e11394_d_n3, assign12280_e11394_d_n4, assign12280_e11394_d_n5, assign12280_e11394_d_n6, assign12280_e11394_d_n7, assign12280_e11394_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12280_e11391: f64 = (var_bigs_i * var_vfgs_eff);
        let assign12280_e11392: f64 = (var_aigs_i - assign12280_e11391);
        (assign12280_e11392, (-(var_bigs_i * var_vfgs_eff_dn3)), (-(var_bigs_i * var_vfgs_eff_dn4)), (-(var_bigs_i * var_vfgs_eff_dn5)), (-(var_bigs_i * var_vfgs_eff_dn6)), (-(var_bigs_i * var_vfgs_eff_dn7)), (-(var_bigs_i * var_vfgs_eff_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign12280_e11394;
        var_t1_dn3 = assign12280_e11394_d_n3;
        var_t1_dn4 = assign12280_e11394_d_n4;
        var_t1_dn5 = assign12280_e11394_d_n5;
        var_t1_dn6 = assign12280_e11394_d_n6;
        var_t1_dn7 = assign12280_e11394_d_n7;
        var_t1_dn8 = assign12280_e11394_d_n8;

        let (assign12290_e11402, assign12290_e11402_d_n3, assign12290_e11402_d_n4, assign12290_e11402_d_n5, assign12290_e11402_d_n6, assign12290_e11402_d_n7, assign12290_e11402_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12290_e11399: f64 = (var_cigs_i * var_vfgs_eff);
        let assign12290_e11400: f64 = (1.0 + assign12290_e11399);
        (assign12290_e11400, (var_cigs_i * var_vfgs_eff_dn3), (var_cigs_i * var_vfgs_eff_dn4), (var_cigs_i * var_vfgs_eff_dn5), (var_cigs_i * var_vfgs_eff_dn6), (var_cigs_i * var_vfgs_eff_dn7), (var_cigs_i * var_vfgs_eff_dn8),)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8,)
    }
};
        var_t2 = assign12290_e11402;
        var_t2_dn3 = assign12290_e11402_d_n3;
        var_t2_dn4 = assign12290_e11402_d_n4;
        var_t2_dn5 = assign12290_e11402_d_n5;
        var_t2_dn6 = assign12290_e11402_d_n6;
        var_t2_dn7 = assign12290_e11402_d_n7;
        var_t2_dn8 = assign12290_e11402_d_n8;

        let (assign12300_e11415, assign12300_e11415_d_n3, assign12300_e11415_d_n4, assign12300_e11415_d_n5, assign12300_e11415_d_n6, assign12300_e11415_d_n7, assign12300_e11415_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12300_e11405: f64 = (-var_bechvb);
        let assign12300_e11407: f64 = (assign12300_e11405 * p.p99);
        let assign12300_e11409: f64 = (assign12300_e11407 * var_poxedge_i);
        let assign12300_e11411: f64 = (assign12300_e11409 * var_t1);
        let assign12300_e11413: f64 = (assign12300_e11411 * var_t2);
        (assign12300_e11413, (((assign12300_e11409 * var_t1_dn3) * var_t2) + (assign12300_e11411 * var_t2_dn3)), (((assign12300_e11409 * var_t1_dn4) * var_t2) + (assign12300_e11411 * var_t2_dn4)), (((assign12300_e11409 * var_t1_dn5) * var_t2) + (assign12300_e11411 * var_t2_dn5)), (((assign12300_e11409 * var_t1_dn6) * var_t2) + (assign12300_e11411 * var_t2_dn6)), (((assign12300_e11409 * var_t1_dn7) * var_t2) + (assign12300_e11411 * var_t2_dn7)), (((assign12300_e11409 * var_t1_dn8) * var_t2) + (assign12300_e11411 * var_t2_dn8)),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8,)
    }
};
        var_t3 = assign12300_e11415;
        var_t3_dn3 = assign12300_e11415_d_n3;
        var_t3_dn4 = assign12300_e11415_d_n4;
        var_t3_dn5 = assign12300_e11415_d_n5;
        var_t3_dn6 = assign12300_e11415_d_n6;
        var_t3_dn7 = assign12300_e11415_d_n7;
        var_t3_dn8 = assign12300_e11415_d_n8;

        let (assign12310_e11420, assign12310_e11420_d_n3, assign12310_e11420_d_n4, assign12310_e11420_d_n5, assign12310_e11420_d_n6, assign12310_e11420_d_n7, assign12310_e11420_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12310_e11418: f64 = { let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign12310_e11418, ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn3), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn4), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn5), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn6), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn7), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn8),)
    } else {
        (var_t4, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8,)
    }
};
        var_t4 = assign12310_e11420;
        var_t4_dn3 = assign12310_e11420_d_n3;
        var_t4_dn4 = assign12310_e11420_d_n4;
        var_t4_dn5 = assign12310_e11420_d_n5;
        var_t4_dn6 = assign12310_e11420_d_n6;
        var_t4_dn7 = assign12310_e11420_d_n7;
        var_t4_dn8 = assign12310_e11420_d_n8;

        let assign12320_e11423: f64 = if var_sigvds > 0.0 { 1.0 } else { 0.0 };
        var_guard124 = assign12320_e11423;

        let (assign12330_e11437, assign12330_e11437_d_n3, assign12330_e11437_d_n4, assign12330_e11437_d_n5, assign12330_e11437_d_n6, assign12330_e11437_d_n7, assign12330_e11437_d_n8,) = {
    if ((var_guard123 != 0.0) && (var_guard124 != 0.0)) {
        let assign12330_e11429: f64 = (var_igsd_mult * p.p234);
        let assign12330_e11431: f64 = (assign12330_e11429 * var_vgs_noswap);
        let assign12330_e11433: f64 = (assign12330_e11431 * var_vfgs_eff);
        let assign12330_e11435: f64 = (assign12330_e11433 * var_t4);
        (assign12330_e11435, ((((((var_igsd_mult_dn3 * p.p234) * var_vgs_noswap) * var_vfgs_eff) + (assign12330_e11431 * var_vfgs_eff_dn3)) * var_t4) + (assign12330_e11433 * var_t4_dn3)), ((((((var_igsd_mult_dn4 * p.p234) * var_vgs_noswap) * var_vfgs_eff) + (assign12330_e11431 * var_vfgs_eff_dn4)) * var_t4) + (assign12330_e11433 * var_t4_dn4)), ((((((var_igsd_mult_dn5 * p.p234) * var_vgs_noswap) * var_vfgs_eff) + (assign12330_e11431 * var_vfgs_eff_dn5)) * var_t4) + (assign12330_e11433 * var_t4_dn5)), (((((((var_igsd_mult_dn6 * p.p234) * var_vgs_noswap) + (assign12330_e11429 * var_vgs_noswap_dn6)) * var_vfgs_eff) + (assign12330_e11431 * var_vfgs_eff_dn6)) * var_t4) + (assign12330_e11433 * var_t4_dn6)), ((((((var_igsd_mult_dn7 * p.p234) * var_vgs_noswap) * var_vfgs_eff) + (assign12330_e11431 * var_vfgs_eff_dn7)) * var_t4) + (assign12330_e11433 * var_t4_dn7)), (((((((var_igsd_mult_dn8 * p.p234) * var_vgs_noswap) + (assign12330_e11429 * var_vgs_noswap_dn8)) * var_vfgs_eff) + (assign12330_e11431 * var_vfgs_eff_dn8)) * var_t4) + (assign12330_e11433 * var_t4_dn8)),)
    } else {
        (var_igs, var_igs_dn3, var_igs_dn4, var_igs_dn5, var_igs_dn6, var_igs_dn7, var_igs_dn8,)
    }
};
        var_igs = assign12330_e11437;
        var_igs_dn3 = assign12330_e11437_d_n3;
        var_igs_dn4 = assign12330_e11437_d_n4;
        var_igs_dn5 = assign12330_e11437_d_n5;
        var_igs_dn6 = assign12330_e11437_d_n6;
        var_igs_dn7 = assign12330_e11437_d_n7;
        var_igs_dn8 = assign12330_e11437_d_n8;

        let (assign12340_e11452, assign12340_e11452_d_n3, assign12340_e11452_d_n4, assign12340_e11452_d_n5, assign12340_e11452_d_n6, assign12340_e11452_d_n7, assign12340_e11452_d_n8,) = {
    if ((var_guard123 != 0.0) && (var_guard124 == 0.0)) {
        let assign12340_e11444: f64 = (var_igsd_mult * p.p234);
        let assign12340_e11446: f64 = (assign12340_e11444 * var_vgs_noswap);
        let assign12340_e11448: f64 = (assign12340_e11446 * var_vfgs_eff);
        let assign12340_e11450: f64 = (assign12340_e11448 * var_t4);
        (assign12340_e11450, ((((((var_igsd_mult_dn3 * p.p234) * var_vgs_noswap) * var_vfgs_eff) + (assign12340_e11446 * var_vfgs_eff_dn3)) * var_t4) + (assign12340_e11448 * var_t4_dn3)), ((((((var_igsd_mult_dn4 * p.p234) * var_vgs_noswap) * var_vfgs_eff) + (assign12340_e11446 * var_vfgs_eff_dn4)) * var_t4) + (assign12340_e11448 * var_t4_dn4)), ((((((var_igsd_mult_dn5 * p.p234) * var_vgs_noswap) * var_vfgs_eff) + (assign12340_e11446 * var_vfgs_eff_dn5)) * var_t4) + (assign12340_e11448 * var_t4_dn5)), (((((((var_igsd_mult_dn6 * p.p234) * var_vgs_noswap) + (assign12340_e11444 * var_vgs_noswap_dn6)) * var_vfgs_eff) + (assign12340_e11446 * var_vfgs_eff_dn6)) * var_t4) + (assign12340_e11448 * var_t4_dn6)), ((((((var_igsd_mult_dn7 * p.p234) * var_vgs_noswap) * var_vfgs_eff) + (assign12340_e11446 * var_vfgs_eff_dn7)) * var_t4) + (assign12340_e11448 * var_t4_dn7)), (((((((var_igsd_mult_dn8 * p.p234) * var_vgs_noswap) + (assign12340_e11444 * var_vgs_noswap_dn8)) * var_vfgs_eff) + (assign12340_e11446 * var_vfgs_eff_dn8)) * var_t4) + (assign12340_e11448 * var_t4_dn8)),)
    } else {
        (var_igd, var_igd_dn3, var_igd_dn4, var_igd_dn5, var_igd_dn6, var_igd_dn7, var_igd_dn8,)
    }
};
        var_igd = assign12340_e11452;
        var_igd_dn3 = assign12340_e11452_d_n3;
        var_igd_dn4 = assign12340_e11452_d_n4;
        var_igd_dn5 = assign12340_e11452_d_n5;
        var_igd_dn6 = assign12340_e11452_d_n6;
        var_igd_dn7 = assign12340_e11452_d_n7;
        var_igd_dn8 = assign12340_e11452_d_n8;

        let (assign12350_e11466, assign12350_e11466_d_n3, assign12350_e11466_d_n4, assign12350_e11466_d_n5, assign12350_e11466_d_n6, assign12350_e11466_d_n7, assign12350_e11466_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12350_e11456: f64 = (var_vgd_noswap - var_vfbsd);
        let assign12350_e11459: f64 = (var_digd_i * var_gamma0);
        let assign12350_e11462: f64 = (var_vbgs - var_vfbsd_bg);
        let assign12350_e11463: f64 = (assign12350_e11459 * assign12350_e11462);
        let assign12350_e11464: f64 = (assign12350_e11456 + assign12350_e11463);
        (assign12350_e11464, ((-var_vfbsd_dn3) + (assign12350_e11459 * (var_vbgs_dn3 - var_vfbsd_bg_dn3))), ((-var_vfbsd_dn4) + (assign12350_e11459 * (-var_vfbsd_bg_dn4))), ((var_vgd_noswap_dn5 - var_vfbsd_dn5) + (assign12350_e11459 * (var_vbgs_dn5 - var_vfbsd_bg_dn5))), ((-var_vfbsd_dn6) + (assign12350_e11459 * (var_vbgs_dn6 - var_vfbsd_bg_dn6))), ((-var_vfbsd_dn7) + (assign12350_e11459 * (-var_vfbsd_bg_dn7))), ((var_vgd_noswap_dn8 - var_vfbsd_dn8) + (assign12350_e11459 * (-var_vfbsd_bg_dn8))),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign12350_e11466;
        var_t0_dn3 = assign12350_e11466_d_n3;
        var_t0_dn4 = assign12350_e11466_d_n4;
        var_t0_dn5 = assign12350_e11466_d_n5;
        var_t0_dn6 = assign12350_e11466_d_n6;
        var_t0_dn7 = assign12350_e11466_d_n7;
        var_t0_dn8 = assign12350_e11466_d_n8;

        let (assign12360_e11475, assign12360_e11475_d_n3, assign12360_e11475_d_n4, assign12360_e11475_d_n5, assign12360_e11475_d_n6, assign12360_e11475_d_n7, assign12360_e11475_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12360_e11470: f64 = (var_t0 * var_t0);
        let assign12360_e11472: f64 = (assign12360_e11470 + 0.0001);
        let assign12360_e11473: f64 = (assign12360_e11472).sqrt();
        (assign12360_e11473, (((var_t0_dn3 * var_t0) + (var_t0 * var_t0_dn3)) / (2.0 * assign12360_e11473)), (((var_t0_dn4 * var_t0) + (var_t0 * var_t0_dn4)) / (2.0 * assign12360_e11473)), (((var_t0_dn5 * var_t0) + (var_t0 * var_t0_dn5)) / (2.0 * assign12360_e11473)), (((var_t0_dn6 * var_t0) + (var_t0 * var_t0_dn6)) / (2.0 * assign12360_e11473)), (((var_t0_dn7 * var_t0) + (var_t0 * var_t0_dn7)) / (2.0 * assign12360_e11473)), (((var_t0_dn8 * var_t0) + (var_t0 * var_t0_dn8)) / (2.0 * assign12360_e11473)),)
    } else {
        (var_vfgd_eff, var_vfgd_eff_dn3, var_vfgd_eff_dn4, var_vfgd_eff_dn5, var_vfgd_eff_dn6, var_vfgd_eff_dn7, var_vfgd_eff_dn8,)
    }
};
        var_vfgd_eff = assign12360_e11475;
        var_vfgd_eff_dn3 = assign12360_e11475_d_n3;
        var_vfgd_eff_dn4 = assign12360_e11475_d_n4;
        var_vfgd_eff_dn5 = assign12360_e11475_d_n5;
        var_vfgd_eff_dn6 = assign12360_e11475_d_n6;
        var_vfgd_eff_dn7 = assign12360_e11475_d_n7;
        var_vfgd_eff_dn8 = assign12360_e11475_d_n8;

        let (assign12370_e11483, assign12370_e11483_d_n3, assign12370_e11483_d_n4, assign12370_e11483_d_n5, assign12370_e11483_d_n6, assign12370_e11483_d_n7, assign12370_e11483_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12370_e11480: f64 = (var_bigd_i * var_vfgd_eff);
        let assign12370_e11481: f64 = (var_aigd_i - assign12370_e11480);
        (assign12370_e11481, (-(var_bigd_i * var_vfgd_eff_dn3)), (-(var_bigd_i * var_vfgd_eff_dn4)), (-(var_bigd_i * var_vfgd_eff_dn5)), (-(var_bigd_i * var_vfgd_eff_dn6)), (-(var_bigd_i * var_vfgd_eff_dn7)), (-(var_bigd_i * var_vfgd_eff_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign12370_e11483;
        var_t1_dn3 = assign12370_e11483_d_n3;
        var_t1_dn4 = assign12370_e11483_d_n4;
        var_t1_dn5 = assign12370_e11483_d_n5;
        var_t1_dn6 = assign12370_e11483_d_n6;
        var_t1_dn7 = assign12370_e11483_d_n7;
        var_t1_dn8 = assign12370_e11483_d_n8;

        let (assign12380_e11491, assign12380_e11491_d_n3, assign12380_e11491_d_n4, assign12380_e11491_d_n5, assign12380_e11491_d_n6, assign12380_e11491_d_n7, assign12380_e11491_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12380_e11488: f64 = (var_cigd_i * var_vfgd_eff);
        let assign12380_e11489: f64 = (1.0 + assign12380_e11488);
        (assign12380_e11489, (var_cigd_i * var_vfgd_eff_dn3), (var_cigd_i * var_vfgd_eff_dn4), (var_cigd_i * var_vfgd_eff_dn5), (var_cigd_i * var_vfgd_eff_dn6), (var_cigd_i * var_vfgd_eff_dn7), (var_cigd_i * var_vfgd_eff_dn8),)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8,)
    }
};
        var_t2 = assign12380_e11491;
        var_t2_dn3 = assign12380_e11491_d_n3;
        var_t2_dn4 = assign12380_e11491_d_n4;
        var_t2_dn5 = assign12380_e11491_d_n5;
        var_t2_dn6 = assign12380_e11491_d_n6;
        var_t2_dn7 = assign12380_e11491_d_n7;
        var_t2_dn8 = assign12380_e11491_d_n8;

        let (assign12390_e11504, assign12390_e11504_d_n3, assign12390_e11504_d_n4, assign12390_e11504_d_n5, assign12390_e11504_d_n6, assign12390_e11504_d_n7, assign12390_e11504_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12390_e11494: f64 = (-var_bechvb);
        let assign12390_e11496: f64 = (assign12390_e11494 * p.p99);
        let assign12390_e11498: f64 = (assign12390_e11496 * var_poxedge_i);
        let assign12390_e11500: f64 = (assign12390_e11498 * var_t1);
        let assign12390_e11502: f64 = (assign12390_e11500 * var_t2);
        (assign12390_e11502, (((assign12390_e11498 * var_t1_dn3) * var_t2) + (assign12390_e11500 * var_t2_dn3)), (((assign12390_e11498 * var_t1_dn4) * var_t2) + (assign12390_e11500 * var_t2_dn4)), (((assign12390_e11498 * var_t1_dn5) * var_t2) + (assign12390_e11500 * var_t2_dn5)), (((assign12390_e11498 * var_t1_dn6) * var_t2) + (assign12390_e11500 * var_t2_dn6)), (((assign12390_e11498 * var_t1_dn7) * var_t2) + (assign12390_e11500 * var_t2_dn7)), (((assign12390_e11498 * var_t1_dn8) * var_t2) + (assign12390_e11500 * var_t2_dn8)),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8,)
    }
};
        var_t3 = assign12390_e11504;
        var_t3_dn3 = assign12390_e11504_d_n3;
        var_t3_dn4 = assign12390_e11504_d_n4;
        var_t3_dn5 = assign12390_e11504_d_n5;
        var_t3_dn6 = assign12390_e11504_d_n6;
        var_t3_dn7 = assign12390_e11504_d_n7;
        var_t3_dn8 = assign12390_e11504_d_n8;

        let (assign12400_e11509, assign12400_e11509_d_n3, assign12400_e11509_d_n4, assign12400_e11509_d_n5, assign12400_e11509_d_n6, assign12400_e11509_d_n7, assign12400_e11509_d_n8,) = {
    if (var_guard123 != 0.0) {
        let assign12400_e11507: f64 = { let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign12400_e11507, ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn3), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn4), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn5), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn6), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn7), ({ let limited_exp_arg = var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t3_dn8),)
    } else {
        (var_t4, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8,)
    }
};
        var_t4 = assign12400_e11509;
        var_t4_dn3 = assign12400_e11509_d_n3;
        var_t4_dn4 = assign12400_e11509_d_n4;
        var_t4_dn5 = assign12400_e11509_d_n5;
        var_t4_dn6 = assign12400_e11509_d_n6;
        var_t4_dn7 = assign12400_e11509_d_n7;
        var_t4_dn8 = assign12400_e11509_d_n8;

        let assign12410_e11512: f64 = if var_sigvds > 0.0 { 1.0 } else { 0.0 };
        var_guard125 = assign12410_e11512;

        let (assign12420_e11526, assign12420_e11526_d_n3, assign12420_e11526_d_n4, assign12420_e11526_d_n5, assign12420_e11526_d_n6, assign12420_e11526_d_n7, assign12420_e11526_d_n8,) = {
    if ((var_guard123 != 0.0) && (var_guard125 != 0.0)) {
        let assign12420_e11518: f64 = (var_igsd_mult * p.p235);
        let assign12420_e11520: f64 = (assign12420_e11518 * var_vgd_noswap);
        let assign12420_e11522: f64 = (assign12420_e11520 * var_vfgd_eff);
        let assign12420_e11524: f64 = (assign12420_e11522 * var_t4);
        (assign12420_e11524, ((((((var_igsd_mult_dn3 * p.p235) * var_vgd_noswap) * var_vfgd_eff) + (assign12420_e11520 * var_vfgd_eff_dn3)) * var_t4) + (assign12420_e11522 * var_t4_dn3)), ((((((var_igsd_mult_dn4 * p.p235) * var_vgd_noswap) * var_vfgd_eff) + (assign12420_e11520 * var_vfgd_eff_dn4)) * var_t4) + (assign12420_e11522 * var_t4_dn4)), (((((((var_igsd_mult_dn5 * p.p235) * var_vgd_noswap) + (assign12420_e11518 * var_vgd_noswap_dn5)) * var_vfgd_eff) + (assign12420_e11520 * var_vfgd_eff_dn5)) * var_t4) + (assign12420_e11522 * var_t4_dn5)), ((((((var_igsd_mult_dn6 * p.p235) * var_vgd_noswap) * var_vfgd_eff) + (assign12420_e11520 * var_vfgd_eff_dn6)) * var_t4) + (assign12420_e11522 * var_t4_dn6)), ((((((var_igsd_mult_dn7 * p.p235) * var_vgd_noswap) * var_vfgd_eff) + (assign12420_e11520 * var_vfgd_eff_dn7)) * var_t4) + (assign12420_e11522 * var_t4_dn7)), (((((((var_igsd_mult_dn8 * p.p235) * var_vgd_noswap) + (assign12420_e11518 * var_vgd_noswap_dn8)) * var_vfgd_eff) + (assign12420_e11520 * var_vfgd_eff_dn8)) * var_t4) + (assign12420_e11522 * var_t4_dn8)),)
    } else {
        (var_igd, var_igd_dn3, var_igd_dn4, var_igd_dn5, var_igd_dn6, var_igd_dn7, var_igd_dn8,)
    }
};
        var_igd = assign12420_e11526;
        var_igd_dn3 = assign12420_e11526_d_n3;
        var_igd_dn4 = assign12420_e11526_d_n4;
        var_igd_dn5 = assign12420_e11526_d_n5;
        var_igd_dn6 = assign12420_e11526_d_n6;
        var_igd_dn7 = assign12420_e11526_d_n7;
        var_igd_dn8 = assign12420_e11526_d_n8;

        let (assign12430_e11541, assign12430_e11541_d_n3, assign12430_e11541_d_n4, assign12430_e11541_d_n5, assign12430_e11541_d_n6, assign12430_e11541_d_n7, assign12430_e11541_d_n8,) = {
    if ((var_guard123 != 0.0) && (var_guard125 == 0.0)) {
        let assign12430_e11533: f64 = (var_igsd_mult * p.p235);
        let assign12430_e11535: f64 = (assign12430_e11533 * var_vgd_noswap);
        let assign12430_e11537: f64 = (assign12430_e11535 * var_vfgd_eff);
        let assign12430_e11539: f64 = (assign12430_e11537 * var_t4);
        (assign12430_e11539, ((((((var_igsd_mult_dn3 * p.p235) * var_vgd_noswap) * var_vfgd_eff) + (assign12430_e11535 * var_vfgd_eff_dn3)) * var_t4) + (assign12430_e11537 * var_t4_dn3)), ((((((var_igsd_mult_dn4 * p.p235) * var_vgd_noswap) * var_vfgd_eff) + (assign12430_e11535 * var_vfgd_eff_dn4)) * var_t4) + (assign12430_e11537 * var_t4_dn4)), (((((((var_igsd_mult_dn5 * p.p235) * var_vgd_noswap) + (assign12430_e11533 * var_vgd_noswap_dn5)) * var_vfgd_eff) + (assign12430_e11535 * var_vfgd_eff_dn5)) * var_t4) + (assign12430_e11537 * var_t4_dn5)), ((((((var_igsd_mult_dn6 * p.p235) * var_vgd_noswap) * var_vfgd_eff) + (assign12430_e11535 * var_vfgd_eff_dn6)) * var_t4) + (assign12430_e11537 * var_t4_dn6)), ((((((var_igsd_mult_dn7 * p.p235) * var_vgd_noswap) * var_vfgd_eff) + (assign12430_e11535 * var_vfgd_eff_dn7)) * var_t4) + (assign12430_e11537 * var_t4_dn7)), (((((((var_igsd_mult_dn8 * p.p235) * var_vgd_noswap) + (assign12430_e11533 * var_vgd_noswap_dn8)) * var_vfgd_eff) + (assign12430_e11535 * var_vfgd_eff_dn8)) * var_t4) + (assign12430_e11537 * var_t4_dn8)),)
    } else {
        (var_igs, var_igs_dn3, var_igs_dn4, var_igs_dn5, var_igs_dn6, var_igs_dn7, var_igs_dn8,)
    }
};
        var_igs = assign12430_e11541;
        var_igs_dn3 = assign12430_e11541_d_n3;
        var_igs_dn4 = assign12430_e11541_d_n4;
        var_igs_dn5 = assign12430_e11541_d_n5;
        var_igs_dn6 = assign12430_e11541_d_n6;
        var_igs_dn7 = assign12430_e11541_d_n7;
        var_igs_dn8 = assign12430_e11541_d_n8;

        var_igisl = 0.0;
        var_igisl_dn3 = 0.0;
        var_igisl_dn4 = 0.0;
        var_igisl_dn5 = 0.0;
        var_igisl_dn6 = 0.0;
        var_igisl_dn7 = 0.0;
        var_igisl_dn8 = 0.0;

        var_igidl = 0.0;
        var_igidl_dn3 = 0.0;
        var_igidl_dn4 = 0.0;
        var_igidl_dn5 = 0.0;
        var_igidl_dn6 = 0.0;
        var_igidl_dn7 = 0.0;
        var_igidl_dn8 = 0.0;

        let assign12460_e11546: f64 = if p.p15 != 0.0 { 1.0 } else { 0.0 };
        var_guard126 = assign12460_e11546;

        let (assign12470_e11552, assign12470_e11552_d_n3, assign12470_e11552_d_n4, assign12470_e11552_d_n5, assign12470_e11552_d_n6, assign12470_e11552_d_n7, assign12470_e11552_d_n8,) = {
    if (var_guard126 != 0.0) {
        let assign12470_e11550: f64 = (var_epsratio * p.p45);
        (assign12470_e11550, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign12470_e11552;
        var_t0_dn3 = assign12470_e11552_d_n3;
        var_t0_dn4 = assign12470_e11552_d_n4;
        var_t0_dn5 = assign12470_e11552_d_n5;
        var_t0_dn6 = assign12470_e11552_d_n6;
        var_t0_dn7 = assign12470_e11552_d_n7;
        var_t0_dn8 = assign12470_e11552_d_n8;

        let assign12480_e11559: f64 = if ((var_agidl_i <= 0.0) || (var_bgidl_t <= 0.0)) { 1.0 } else { 0.0 };
        var_guard127 = assign12480_e11559;

        let (assign12490_e11565, assign12490_e11565_d_n3, assign12490_e11565_d_n4, assign12490_e11565_d_n5, assign12490_e11565_d_n6, assign12490_e11565_d_n7, assign12490_e11565_d_n8,) = {
    if ((var_guard126 != 0.0) && (var_guard127 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t6, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8,)
    }
};
        var_t6 = assign12490_e11565;
        var_t6_dn3 = assign12490_e11565_d_n3;
        var_t6_dn4 = assign12490_e11565_d_n4;
        var_t6_dn5 = assign12490_e11565_d_n5;
        var_t6_dn6 = assign12490_e11565_d_n6;
        var_t6_dn7 = assign12490_e11565_d_n7;
        var_t6_dn8 = assign12490_e11565_d_n8;

        *var_guard124_slot = var_guard124;
        *var_guard125_slot = var_guard125;
        *var_guard126_slot = var_guard126;
        *var_guard127_slot = var_guard127;
        *var_igcd_slot = var_igcd;
        *var_igcd_dn3_slot = var_igcd_dn3;
        *var_igcd_dn4_slot = var_igcd_dn4;
        *var_igcd_dn5_slot = var_igcd_dn5;
        *var_igcd_dn6_slot = var_igcd_dn6;
        *var_igcd_dn7_slot = var_igcd_dn7;
        *var_igcd_dn8_slot = var_igcd_dn8;
        *var_igcs_slot = var_igcs;
        *var_igcs_dn3_slot = var_igcs_dn3;
        *var_igcs_dn4_slot = var_igcs_dn4;
        *var_igcs_dn5_slot = var_igcs_dn5;
        *var_igcs_dn6_slot = var_igcs_dn6;
        *var_igcs_dn7_slot = var_igcs_dn7;
        *var_igcs_dn8_slot = var_igcs_dn8;
        *var_igd_slot = var_igd;
        *var_igd_dn3_slot = var_igd_dn3;
        *var_igd_dn4_slot = var_igd_dn4;
        *var_igd_dn5_slot = var_igd_dn5;
        *var_igd_dn6_slot = var_igd_dn6;
        *var_igd_dn7_slot = var_igd_dn7;
        *var_igd_dn8_slot = var_igd_dn8;
        *var_igidl_slot = var_igidl;
        *var_igidl_dn3_slot = var_igidl_dn3;
        *var_igidl_dn4_slot = var_igidl_dn4;
        *var_igidl_dn5_slot = var_igidl_dn5;
        *var_igidl_dn6_slot = var_igidl_dn6;
        *var_igidl_dn7_slot = var_igidl_dn7;
        *var_igidl_dn8_slot = var_igidl_dn8;
        *var_igisl_slot = var_igisl;
        *var_igisl_dn3_slot = var_igisl_dn3;
        *var_igisl_dn4_slot = var_igisl_dn4;
        *var_igisl_dn5_slot = var_igisl_dn5;
        *var_igisl_dn6_slot = var_igisl_dn6;
        *var_igisl_dn7_slot = var_igisl_dn7;
        *var_igisl_dn8_slot = var_igisl_dn8;
        *var_igs_slot = var_igs;
        *var_igs_dn3_slot = var_igs_dn3;
        *var_igs_dn4_slot = var_igs_dn4;
        *var_igs_dn5_slot = var_igs_dn5;
        *var_igs_dn6_slot = var_igs_dn6;
        *var_igs_dn7_slot = var_igs_dn7;
        *var_igs_dn8_slot = var_igs_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_exp_slot = var_t1_exp;
        *var_t1_exp_dn3_slot = var_t1_exp_dn3;
        *var_t1_exp_dn4_slot = var_t1_exp_dn4;
        *var_t1_exp_dn5_slot = var_t1_exp_dn5;
        *var_t1_exp_dn6_slot = var_t1_exp_dn6;
        *var_t1_exp_dn7_slot = var_t1_exp_dn7;
        *var_t1_exp_dn8_slot = var_t1_exp_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t4_slot = var_t4;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t5_slot = var_t5;
        *var_t5_dn3_slot = var_t5_dn3;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t6_slot = var_t6;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_vdseffx_slot = var_vdseffx;
        *var_vdseffx_dn3_slot = var_vdseffx_dn3;
        *var_vdseffx_dn4_slot = var_vdseffx_dn4;
        *var_vdseffx_dn5_slot = var_vdseffx_dn5;
        *var_vdseffx_dn6_slot = var_vdseffx_dn6;
        *var_vdseffx_dn7_slot = var_vdseffx_dn7;
        *var_vdseffx_dn8_slot = var_vdseffx_dn8;
        *var_vfgd_eff_slot = var_vfgd_eff;
        *var_vfgd_eff_dn3_slot = var_vfgd_eff_dn3;
        *var_vfgd_eff_dn4_slot = var_vfgd_eff_dn4;
        *var_vfgd_eff_dn5_slot = var_vfgd_eff_dn5;
        *var_vfgd_eff_dn6_slot = var_vfgd_eff_dn6;
        *var_vfgd_eff_dn7_slot = var_vfgd_eff_dn7;
        *var_vfgd_eff_dn8_slot = var_vfgd_eff_dn8;
        *var_vfgs_eff_slot = var_vfgs_eff;
        *var_vfgs_eff_dn3_slot = var_vfgs_eff_dn3;
        *var_vfgs_eff_dn4_slot = var_vfgs_eff_dn4;
        *var_vfgs_eff_dn5_slot = var_vfgs_eff_dn5;
        *var_vfgs_eff_dn6_slot = var_vfgs_eff_dn6;
        *var_vfgs_eff_dn7_slot = var_vfgs_eff_dn7;
        *var_vfgs_eff_dn8_slot = var_vfgs_eff_dn8;
    }

    pub(super) fn stamp_transient_block_28(
        p: &Parameters,
        var_agidl_i: f64,
        var_agisl_i: f64,
        var_bgidl_t: f64,
        var_bgidl_t_dn4: f64,
        var_bgisl_t: f64,
        var_bgisl_t_dn4: f64,
        var_diffvds: f64,
        var_diffvds_dn3: f64,
        var_diffvds_dn4: f64,
        var_diffvds_dn5: f64,
        var_diffvds_dn6: f64,
        var_diffvds_dn7: f64,
        var_diffvds_dn8: f64,
        var_egidl_i: f64,
        var_egisl_i: f64,
        var_gamma0: f64,
        var_guard126: f64,
        var_guard127: f64,
        var_leff: f64,
        var_lintnoi_i: f64,
        var_litl: f64,
        var_mpower_i: f64,
        var_noia2_i: f64,
        var_pgidl_i: f64,
        var_pgisl_i: f64,
        var_qia2: f64,
        var_qia2_dn3: f64,
        var_qia2_dn4: f64,
        var_qia2_dn5: f64,
        var_qia2_dn6: f64,
        var_qia2_dn7: f64,
        var_qia2_dn8: f64,
        var_qsref_i: f64,
        var_sigvds: f64,
        var_utotal: f64,
        var_utotal_dn3: f64,
        var_utotal_dn4: f64,
        var_utotal_dn5: f64,
        var_utotal_dn6: f64,
        var_utotal_dn7: f64,
        var_utotal_dn8: f64,
        var_vbegidl_i: f64,
        var_vbegisl_i: f64,
        var_vbgidl_i: f64,
        var_vbgisl_i: f64,
        var_vbgs: f64,
        var_vbgs_dn3: f64,
        var_vbgs_dn5: f64,
        var_vbgs_dn6: f64,
        var_vds_noswap: f64,
        var_vds_noswap_dn5: f64,
        var_vds_noswap_dn6: f64,
        var_vfbsd: f64,
        var_vfbsd_bg: f64,
        var_vfbsd_bg_dn3: f64,
        var_vfbsd_bg_dn4: f64,
        var_vfbsd_bg_dn5: f64,
        var_vfbsd_bg_dn6: f64,
        var_vfbsd_bg_dn7: f64,
        var_vfbsd_bg_dn8: f64,
        var_vfbsd_dn3: f64,
        var_vfbsd_dn4: f64,
        var_vfbsd_dn5: f64,
        var_vfbsd_dn6: f64,
        var_vfbsd_dn7: f64,
        var_vfbsd_dn8: f64,
        var_vgd_noswap: f64,
        var_vgd_noswap_dn5: f64,
        var_vgd_noswap_dn8: f64,
        var_vgs_noswap: f64,
        var_vgs_noswap_dn6: f64,
        var_vgs_noswap_dn8: f64,
        var_vsat_t: f64,
        var_vsat_t_dn4: f64,
        var_weff: f64,
        var_delclm_slot: &mut f64,
        var_delclm_dn3_slot: &mut f64,
        var_delclm_dn4_slot: &mut f64,
        var_delclm_dn5_slot: &mut f64,
        var_delclm_dn6_slot: &mut f64,
        var_delclm_dn7_slot: &mut f64,
        var_delclm_dn8_slot: &mut f64,
        var_esatnoi_slot: &mut f64,
        var_esatnoi_dn3_slot: &mut f64,
        var_esatnoi_dn4_slot: &mut f64,
        var_esatnoi_dn5_slot: &mut f64,
        var_esatnoi_dn6_slot: &mut f64,
        var_esatnoi_dn7_slot: &mut f64,
        var_esatnoi_dn8_slot: &mut f64,
        var_guard128_slot: &mut f64,
        var_guard129_slot: &mut f64,
        var_guard130_slot: &mut f64,
        var_guard131_slot: &mut f64,
        var_guard132_slot: &mut f64,
        var_guard133_slot: &mut f64,
        var_guard134_slot: &mut f64,
        var_igidl_slot: &mut f64,
        var_igidl_dn3_slot: &mut f64,
        var_igidl_dn4_slot: &mut f64,
        var_igidl_dn5_slot: &mut f64,
        var_igidl_dn6_slot: &mut f64,
        var_igidl_dn7_slot: &mut f64,
        var_igidl_dn8_slot: &mut f64,
        var_igisl_slot: &mut f64,
        var_igisl_dn3_slot: &mut f64,
        var_igisl_dn4_slot: &mut f64,
        var_igisl_dn5_slot: &mut f64,
        var_igisl_dn6_slot: &mut f64,
        var_igisl_dn7_slot: &mut f64,
        var_igisl_dn8_slot: &mut f64,
        var_leffnoi_slot: &mut f64,
        var_leffnoisq_slot: &mut f64,
        var_noiaeff_slot: &mut f64,
        var_noiaeff_dn3_slot: &mut f64,
        var_noiaeff_dn4_slot: &mut f64,
        var_noiaeff_dn5_slot: &mut f64,
        var_noiaeff_dn6_slot: &mut f64,
        var_noiaeff_dn7_slot: &mut f64,
        var_noiaeff_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
    ) {
        let mut var_delclm: f64 = *var_delclm_slot;
        let mut var_delclm_dn3: f64 = *var_delclm_dn3_slot;
        let mut var_delclm_dn4: f64 = *var_delclm_dn4_slot;
        let mut var_delclm_dn5: f64 = *var_delclm_dn5_slot;
        let mut var_delclm_dn6: f64 = *var_delclm_dn6_slot;
        let mut var_delclm_dn7: f64 = *var_delclm_dn7_slot;
        let mut var_delclm_dn8: f64 = *var_delclm_dn8_slot;
        let mut var_esatnoi: f64 = *var_esatnoi_slot;
        let mut var_esatnoi_dn3: f64 = *var_esatnoi_dn3_slot;
        let mut var_esatnoi_dn4: f64 = *var_esatnoi_dn4_slot;
        let mut var_esatnoi_dn5: f64 = *var_esatnoi_dn5_slot;
        let mut var_esatnoi_dn6: f64 = *var_esatnoi_dn6_slot;
        let mut var_esatnoi_dn7: f64 = *var_esatnoi_dn7_slot;
        let mut var_esatnoi_dn8: f64 = *var_esatnoi_dn8_slot;
        let mut var_guard128: f64 = *var_guard128_slot;
        let mut var_guard129: f64 = *var_guard129_slot;
        let mut var_guard130: f64 = *var_guard130_slot;
        let mut var_guard131: f64 = *var_guard131_slot;
        let mut var_guard132: f64 = *var_guard132_slot;
        let mut var_guard133: f64 = *var_guard133_slot;
        let mut var_guard134: f64 = *var_guard134_slot;
        let mut var_igidl: f64 = *var_igidl_slot;
        let mut var_igidl_dn3: f64 = *var_igidl_dn3_slot;
        let mut var_igidl_dn4: f64 = *var_igidl_dn4_slot;
        let mut var_igidl_dn5: f64 = *var_igidl_dn5_slot;
        let mut var_igidl_dn6: f64 = *var_igidl_dn6_slot;
        let mut var_igidl_dn7: f64 = *var_igidl_dn7_slot;
        let mut var_igidl_dn8: f64 = *var_igidl_dn8_slot;
        let mut var_igisl: f64 = *var_igisl_slot;
        let mut var_igisl_dn3: f64 = *var_igisl_dn3_slot;
        let mut var_igisl_dn4: f64 = *var_igisl_dn4_slot;
        let mut var_igisl_dn5: f64 = *var_igisl_dn5_slot;
        let mut var_igisl_dn6: f64 = *var_igisl_dn6_slot;
        let mut var_igisl_dn7: f64 = *var_igisl_dn7_slot;
        let mut var_igisl_dn8: f64 = *var_igisl_dn8_slot;
        let mut var_leffnoi: f64 = *var_leffnoi_slot;
        let mut var_leffnoisq: f64 = *var_leffnoisq_slot;
        let mut var_noiaeff: f64 = *var_noiaeff_slot;
        let mut var_noiaeff_dn3: f64 = *var_noiaeff_dn3_slot;
        let mut var_noiaeff_dn4: f64 = *var_noiaeff_dn4_slot;
        let mut var_noiaeff_dn5: f64 = *var_noiaeff_dn5_slot;
        let mut var_noiaeff_dn6: f64 = *var_noiaeff_dn6_slot;
        let mut var_noiaeff_dn7: f64 = *var_noiaeff_dn7_slot;
        let mut var_noiaeff_dn8: f64 = *var_noiaeff_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;

        let (assign12500_e11589, assign12500_e11589_d_n3, assign12500_e11589_d_n4, assign12500_e11589_d_n5, assign12500_e11589_d_n6, assign12500_e11589_d_n7, assign12500_e11589_d_n8,) = {
    if ((var_guard126 != 0.0) && (var_guard127 == 0.0)) {
        let assign12500_e11571: f64 = (-var_vgd_noswap);
        let assign12500_e11573: f64 = (assign12500_e11571 - var_egidl_i);
        let assign12500_e11575: f64 = (assign12500_e11573 + var_vfbsd);
        let assign12500_e11578: f64 = (var_vbgidl_i * var_gamma0);
        let assign12500_e11581: f64 = (var_vbgs - var_vfbsd_bg);
        let assign12500_e11583: f64 = (assign12500_e11581 - var_vbegidl_i);
        let assign12500_e11584: f64 = (assign12500_e11578 * assign12500_e11583);
        let assign12500_e11585: f64 = (assign12500_e11575 + assign12500_e11584);
        let assign12500_e11587: f64 = (assign12500_e11585 / var_t0);
        (assign12500_e11587, ((((var_vfbsd_dn3 + (assign12500_e11578 * (var_vbgs_dn3 - var_vfbsd_bg_dn3))) * var_t0) - (assign12500_e11585 * var_t0_dn3)) / (var_t0 * var_t0)), ((((var_vfbsd_dn4 + (assign12500_e11578 * (-var_vfbsd_bg_dn4))) * var_t0) - (assign12500_e11585 * var_t0_dn4)) / (var_t0 * var_t0)), ((((((-var_vgd_noswap_dn5) + var_vfbsd_dn5) + (assign12500_e11578 * (var_vbgs_dn5 - var_vfbsd_bg_dn5))) * var_t0) - (assign12500_e11585 * var_t0_dn5)) / (var_t0 * var_t0)), ((((var_vfbsd_dn6 + (assign12500_e11578 * (var_vbgs_dn6 - var_vfbsd_bg_dn6))) * var_t0) - (assign12500_e11585 * var_t0_dn6)) / (var_t0 * var_t0)), ((((var_vfbsd_dn7 + (assign12500_e11578 * (-var_vfbsd_bg_dn7))) * var_t0) - (assign12500_e11585 * var_t0_dn7)) / (var_t0 * var_t0)), ((((((-var_vgd_noswap_dn8) + var_vfbsd_dn8) + (assign12500_e11578 * (-var_vfbsd_bg_dn8))) * var_t0) - (assign12500_e11585 * var_t0_dn8)) / (var_t0 * var_t0)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign12500_e11589;
        var_t1_dn3 = assign12500_e11589_d_n3;
        var_t1_dn4 = assign12500_e11589_d_n4;
        var_t1_dn5 = assign12500_e11589_d_n5;
        var_t1_dn6 = assign12500_e11589_d_n6;
        var_t1_dn7 = assign12500_e11589_d_n7;
        var_t1_dn8 = assign12500_e11589_d_n8;

        let (assign12510_e11609, assign12510_e11609_d_n3, assign12510_e11609_d_n4, assign12510_e11609_d_n5, assign12510_e11609_d_n6, assign12510_e11609_d_n7, assign12510_e11609_d_n8,) = {
    if ((var_guard126 != 0.0) && (var_guard127 == 0.0)) {
        let assign12510_e11598: f64 = (var_t1 * var_t1);
        let assign12510_e11601: f64 = (4.0 * 0.01);
        let assign12510_e11603: f64 = (assign12510_e11601 * 0.01);
        let assign12510_e11604: f64 = (assign12510_e11598 + assign12510_e11603);
        let assign12510_e11605: f64 = (assign12510_e11604).sqrt();
        let assign12510_e11606: f64 = (var_t1 + assign12510_e11605);
        let assign12510_e11607: f64 = (0.5 * assign12510_e11606);
        (assign12510_e11607, (0.5 * (var_t1_dn3 + (((var_t1_dn3 * var_t1) + (var_t1 * var_t1_dn3)) / (2.0 * assign12510_e11605)))), (0.5 * (var_t1_dn4 + (((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) / (2.0 * assign12510_e11605)))), (0.5 * (var_t1_dn5 + (((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) / (2.0 * assign12510_e11605)))), (0.5 * (var_t1_dn6 + (((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) / (2.0 * assign12510_e11605)))), (0.5 * (var_t1_dn7 + (((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)) / (2.0 * assign12510_e11605)))), (0.5 * (var_t1_dn8 + (((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) / (2.0 * assign12510_e11605)))),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign12510_e11609;
        var_t1_dn3 = assign12510_e11609_d_n3;
        var_t1_dn4 = assign12510_e11609_d_n4;
        var_t1_dn5 = assign12510_e11609_d_n5;
        var_t1_dn6 = assign12510_e11609_d_n6;
        var_t1_dn7 = assign12510_e11609_d_n7;
        var_t1_dn8 = assign12510_e11609_d_n8;

        let (assign12520_e11620, assign12520_e11620_d_n3, assign12520_e11620_d_n4, assign12520_e11620_d_n5, assign12520_e11620_d_n6, assign12520_e11620_d_n7, assign12520_e11620_d_n8,) = {
    if ((var_guard126 != 0.0) && (var_guard127 == 0.0)) {
        let assign12520_e11617: f64 = (var_t1 + 0.001);
        let assign12520_e11618: f64 = (var_bgidl_t / assign12520_e11617);
        (assign12520_e11618, (-((var_bgidl_t * var_t1_dn3) / (assign12520_e11617 * assign12520_e11617))), (((var_bgidl_t_dn4 * assign12520_e11617) - (var_bgidl_t * var_t1_dn4)) / (assign12520_e11617 * assign12520_e11617)), (-((var_bgidl_t * var_t1_dn5) / (assign12520_e11617 * assign12520_e11617))), (-((var_bgidl_t * var_t1_dn6) / (assign12520_e11617 * assign12520_e11617))), (-((var_bgidl_t * var_t1_dn7) / (assign12520_e11617 * assign12520_e11617))), (-((var_bgidl_t * var_t1_dn8) / (assign12520_e11617 * assign12520_e11617))),)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8,)
    }
};
        var_t2 = assign12520_e11620;
        var_t2_dn3 = assign12520_e11620_d_n3;
        var_t2_dn4 = assign12520_e11620_d_n4;
        var_t2_dn5 = assign12520_e11620_d_n5;
        var_t2_dn6 = assign12520_e11620_d_n6;
        var_t2_dn7 = assign12520_e11620_d_n7;
        var_t2_dn8 = assign12520_e11620_d_n8;

        let (assign12530_e11633, assign12530_e11633_d_n3, assign12530_e11633_d_n4, assign12530_e11633_d_n5, assign12530_e11633_d_n6, assign12530_e11633_d_n7, assign12530_e11633_d_n8,) = {
    if ((var_guard126 != 0.0) && (var_guard127 == 0.0)) {
        let assign12530_e11628: f64 = (var_t1).max(1e-38);
        let assign12530_e11629: f64 = (assign12530_e11628).ln();
        let assign12530_e11630: f64 = (var_pgidl_i * assign12530_e11629);
        let assign12530_e11631: f64 = { let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign12530_e11631, ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_pgidl_i * (if var_t1 >= 1e-38 { var_t1_dn3 } else { 0.0 } / assign12530_e11628))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_pgidl_i * (if var_t1 >= 1e-38 { var_t1_dn4 } else { 0.0 } / assign12530_e11628))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_pgidl_i * (if var_t1 >= 1e-38 { var_t1_dn5 } else { 0.0 } / assign12530_e11628))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_pgidl_i * (if var_t1 >= 1e-38 { var_t1_dn6 } else { 0.0 } / assign12530_e11628))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_pgidl_i * (if var_t1 >= 1e-38 { var_t1_dn7 } else { 0.0 } / assign12530_e11628))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_pgidl_i * (if var_t1 >= 1e-38 { var_t1_dn8 } else { 0.0 } / assign12530_e11628))),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8,)
    }
};
        var_t3 = assign12530_e11633;
        var_t3_dn3 = assign12530_e11633_d_n3;
        var_t3_dn4 = assign12530_e11633_d_n4;
        var_t3_dn5 = assign12530_e11633_d_n5;
        var_t3_dn6 = assign12530_e11633_d_n6;
        var_t3_dn7 = assign12530_e11633_d_n7;
        var_t3_dn8 = assign12530_e11633_d_n8;

        let (assign12540_e11650, assign12540_e11650_d_n3, assign12540_e11650_d_n4, assign12540_e11650_d_n5, assign12540_e11650_d_n6, assign12540_e11650_d_n7, assign12540_e11650_d_n8,) = {
    if ((var_guard126 != 0.0) && (var_guard127 == 0.0)) {
        let assign12540_e11640: f64 = (var_agidl_i * var_weff);
        let assign12540_e11642: f64 = (assign12540_e11640 * var_t3);
        let assign12540_e11644: f64 = (-var_t2);
        let assign12540_e11645: f64 = { let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign12540_e11646: f64 = (assign12540_e11642 * assign12540_e11645);
        let assign12540_e11648: f64 = (assign12540_e11646 * var_vds_noswap);
        (assign12540_e11648, ((((assign12540_e11640 * var_t3_dn3) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn3)))) * var_vds_noswap), ((((assign12540_e11640 * var_t3_dn4) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn4)))) * var_vds_noswap), (((((assign12540_e11640 * var_t3_dn5) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn5)))) * var_vds_noswap) + (assign12540_e11646 * var_vds_noswap_dn5)), (((((assign12540_e11640 * var_t3_dn6) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn6)))) * var_vds_noswap) + (assign12540_e11646 * var_vds_noswap_dn6)), ((((assign12540_e11640 * var_t3_dn7) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn7)))) * var_vds_noswap), ((((assign12540_e11640 * var_t3_dn8) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn8)))) * var_vds_noswap),)
    } else {
        (var_t6, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8,)
    }
};
        var_t6 = assign12540_e11650;
        var_t6_dn3 = assign12540_e11650_d_n3;
        var_t6_dn4 = assign12540_e11650_d_n4;
        var_t6_dn5 = assign12540_e11650_d_n5;
        var_t6_dn6 = assign12540_e11650_d_n6;
        var_t6_dn7 = assign12540_e11650_d_n7;
        var_t6_dn8 = assign12540_e11650_d_n8;

        let assign12550_e11653: f64 = if var_sigvds > 0.0 { 1.0 } else { 0.0 };
        var_guard128 = assign12550_e11653;

        let (assign12560_e11659, assign12560_e11659_d_n3, assign12560_e11659_d_n4, assign12560_e11659_d_n5, assign12560_e11659_d_n6, assign12560_e11659_d_n7, assign12560_e11659_d_n8,) = {
    if ((var_guard126 != 0.0) && (var_guard128 != 0.0)) {
        (var_t6, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8,)
    } else {
        (var_igidl, var_igidl_dn3, var_igidl_dn4, var_igidl_dn5, var_igidl_dn6, var_igidl_dn7, var_igidl_dn8,)
    }
};
        var_igidl = assign12560_e11659;
        var_igidl_dn3 = assign12560_e11659_d_n3;
        var_igidl_dn4 = assign12560_e11659_d_n4;
        var_igidl_dn5 = assign12560_e11659_d_n5;
        var_igidl_dn6 = assign12560_e11659_d_n6;
        var_igidl_dn7 = assign12560_e11659_d_n7;
        var_igidl_dn8 = assign12560_e11659_d_n8;

        let (assign12570_e11666, assign12570_e11666_d_n3, assign12570_e11666_d_n4, assign12570_e11666_d_n5, assign12570_e11666_d_n6, assign12570_e11666_d_n7, assign12570_e11666_d_n8,) = {
    if ((var_guard126 != 0.0) && (var_guard128 == 0.0)) {
        (var_t6, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8,)
    } else {
        (var_igisl, var_igisl_dn3, var_igisl_dn4, var_igisl_dn5, var_igisl_dn6, var_igisl_dn7, var_igisl_dn8,)
    }
};
        var_igisl = assign12570_e11666;
        var_igisl_dn3 = assign12570_e11666_d_n3;
        var_igisl_dn4 = assign12570_e11666_d_n4;
        var_igisl_dn5 = assign12570_e11666_d_n5;
        var_igisl_dn6 = assign12570_e11666_d_n6;
        var_igisl_dn7 = assign12570_e11666_d_n7;
        var_igisl_dn8 = assign12570_e11666_d_n8;

        let assign12580_e11673: f64 = if ((var_agisl_i <= 0.0) || (var_bgisl_t <= 0.0)) { 1.0 } else { 0.0 };
        var_guard129 = assign12580_e11673;

        let (assign12590_e11679, assign12590_e11679_d_n3, assign12590_e11679_d_n4, assign12590_e11679_d_n5, assign12590_e11679_d_n6, assign12590_e11679_d_n7, assign12590_e11679_d_n8,) = {
    if ((var_guard126 != 0.0) && (var_guard129 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t6, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8,)
    }
};
        var_t6 = assign12590_e11679;
        var_t6_dn3 = assign12590_e11679_d_n3;
        var_t6_dn4 = assign12590_e11679_d_n4;
        var_t6_dn5 = assign12590_e11679_d_n5;
        var_t6_dn6 = assign12590_e11679_d_n6;
        var_t6_dn7 = assign12590_e11679_d_n7;
        var_t6_dn8 = assign12590_e11679_d_n8;

        let (assign12600_e11703, assign12600_e11703_d_n3, assign12600_e11703_d_n4, assign12600_e11703_d_n5, assign12600_e11703_d_n6, assign12600_e11703_d_n7, assign12600_e11703_d_n8,) = {
    if ((var_guard126 != 0.0) && (var_guard129 == 0.0)) {
        let assign12600_e11685: f64 = (-var_vgs_noswap);
        let assign12600_e11687: f64 = (assign12600_e11685 - var_egisl_i);
        let assign12600_e11689: f64 = (assign12600_e11687 + var_vfbsd);
        let assign12600_e11692: f64 = (var_vbgisl_i * var_gamma0);
        let assign12600_e11695: f64 = (var_vbgs - var_vfbsd_bg);
        let assign12600_e11697: f64 = (assign12600_e11695 - var_vbegisl_i);
        let assign12600_e11698: f64 = (assign12600_e11692 * assign12600_e11697);
        let assign12600_e11699: f64 = (assign12600_e11689 + assign12600_e11698);
        let assign12600_e11701: f64 = (assign12600_e11699 / var_t0);
        (assign12600_e11701, ((((var_vfbsd_dn3 + (assign12600_e11692 * (var_vbgs_dn3 - var_vfbsd_bg_dn3))) * var_t0) - (assign12600_e11699 * var_t0_dn3)) / (var_t0 * var_t0)), ((((var_vfbsd_dn4 + (assign12600_e11692 * (-var_vfbsd_bg_dn4))) * var_t0) - (assign12600_e11699 * var_t0_dn4)) / (var_t0 * var_t0)), ((((var_vfbsd_dn5 + (assign12600_e11692 * (var_vbgs_dn5 - var_vfbsd_bg_dn5))) * var_t0) - (assign12600_e11699 * var_t0_dn5)) / (var_t0 * var_t0)), ((((((-var_vgs_noswap_dn6) + var_vfbsd_dn6) + (assign12600_e11692 * (var_vbgs_dn6 - var_vfbsd_bg_dn6))) * var_t0) - (assign12600_e11699 * var_t0_dn6)) / (var_t0 * var_t0)), ((((var_vfbsd_dn7 + (assign12600_e11692 * (-var_vfbsd_bg_dn7))) * var_t0) - (assign12600_e11699 * var_t0_dn7)) / (var_t0 * var_t0)), ((((((-var_vgs_noswap_dn8) + var_vfbsd_dn8) + (assign12600_e11692 * (-var_vfbsd_bg_dn8))) * var_t0) - (assign12600_e11699 * var_t0_dn8)) / (var_t0 * var_t0)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign12600_e11703;
        var_t1_dn3 = assign12600_e11703_d_n3;
        var_t1_dn4 = assign12600_e11703_d_n4;
        var_t1_dn5 = assign12600_e11703_d_n5;
        var_t1_dn6 = assign12600_e11703_d_n6;
        var_t1_dn7 = assign12600_e11703_d_n7;
        var_t1_dn8 = assign12600_e11703_d_n8;

        let (assign12610_e11723, assign12610_e11723_d_n3, assign12610_e11723_d_n4, assign12610_e11723_d_n5, assign12610_e11723_d_n6, assign12610_e11723_d_n7, assign12610_e11723_d_n8,) = {
    if ((var_guard126 != 0.0) && (var_guard129 == 0.0)) {
        let assign12610_e11712: f64 = (var_t1 * var_t1);
        let assign12610_e11715: f64 = (4.0 * 0.01);
        let assign12610_e11717: f64 = (assign12610_e11715 * 0.01);
        let assign12610_e11718: f64 = (assign12610_e11712 + assign12610_e11717);
        let assign12610_e11719: f64 = (assign12610_e11718).sqrt();
        let assign12610_e11720: f64 = (var_t1 + assign12610_e11719);
        let assign12610_e11721: f64 = (0.5 * assign12610_e11720);
        (assign12610_e11721, (0.5 * (var_t1_dn3 + (((var_t1_dn3 * var_t1) + (var_t1 * var_t1_dn3)) / (2.0 * assign12610_e11719)))), (0.5 * (var_t1_dn4 + (((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) / (2.0 * assign12610_e11719)))), (0.5 * (var_t1_dn5 + (((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) / (2.0 * assign12610_e11719)))), (0.5 * (var_t1_dn6 + (((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) / (2.0 * assign12610_e11719)))), (0.5 * (var_t1_dn7 + (((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)) / (2.0 * assign12610_e11719)))), (0.5 * (var_t1_dn8 + (((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) / (2.0 * assign12610_e11719)))),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign12610_e11723;
        var_t1_dn3 = assign12610_e11723_d_n3;
        var_t1_dn4 = assign12610_e11723_d_n4;
        var_t1_dn5 = assign12610_e11723_d_n5;
        var_t1_dn6 = assign12610_e11723_d_n6;
        var_t1_dn7 = assign12610_e11723_d_n7;
        var_t1_dn8 = assign12610_e11723_d_n8;

        let (assign12620_e11734, assign12620_e11734_d_n3, assign12620_e11734_d_n4, assign12620_e11734_d_n5, assign12620_e11734_d_n6, assign12620_e11734_d_n7, assign12620_e11734_d_n8,) = {
    if ((var_guard126 != 0.0) && (var_guard129 == 0.0)) {
        let assign12620_e11731: f64 = (var_t1 + 0.001);
        let assign12620_e11732: f64 = (var_bgisl_t / assign12620_e11731);
        (assign12620_e11732, (-((var_bgisl_t * var_t1_dn3) / (assign12620_e11731 * assign12620_e11731))), (((var_bgisl_t_dn4 * assign12620_e11731) - (var_bgisl_t * var_t1_dn4)) / (assign12620_e11731 * assign12620_e11731)), (-((var_bgisl_t * var_t1_dn5) / (assign12620_e11731 * assign12620_e11731))), (-((var_bgisl_t * var_t1_dn6) / (assign12620_e11731 * assign12620_e11731))), (-((var_bgisl_t * var_t1_dn7) / (assign12620_e11731 * assign12620_e11731))), (-((var_bgisl_t * var_t1_dn8) / (assign12620_e11731 * assign12620_e11731))),)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8,)
    }
};
        var_t2 = assign12620_e11734;
        var_t2_dn3 = assign12620_e11734_d_n3;
        var_t2_dn4 = assign12620_e11734_d_n4;
        var_t2_dn5 = assign12620_e11734_d_n5;
        var_t2_dn6 = assign12620_e11734_d_n6;
        var_t2_dn7 = assign12620_e11734_d_n7;
        var_t2_dn8 = assign12620_e11734_d_n8;

        let (assign12630_e11747, assign12630_e11747_d_n3, assign12630_e11747_d_n4, assign12630_e11747_d_n5, assign12630_e11747_d_n6, assign12630_e11747_d_n7, assign12630_e11747_d_n8,) = {
    if ((var_guard126 != 0.0) && (var_guard129 == 0.0)) {
        let assign12630_e11742: f64 = (var_t1).max(1e-38);
        let assign12630_e11743: f64 = (assign12630_e11742).ln();
        let assign12630_e11744: f64 = (var_pgisl_i * assign12630_e11743);
        let assign12630_e11745: f64 = { let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign12630_e11745, ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_pgisl_i * (if var_t1 >= 1e-38 { var_t1_dn3 } else { 0.0 } / assign12630_e11742))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_pgisl_i * (if var_t1 >= 1e-38 { var_t1_dn4 } else { 0.0 } / assign12630_e11742))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_pgisl_i * (if var_t1 >= 1e-38 { var_t1_dn5 } else { 0.0 } / assign12630_e11742))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_pgisl_i * (if var_t1 >= 1e-38 { var_t1_dn6 } else { 0.0 } / assign12630_e11742))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_pgisl_i * (if var_t1 >= 1e-38 { var_t1_dn7 } else { 0.0 } / assign12630_e11742))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_pgisl_i * (if var_t1 >= 1e-38 { var_t1_dn8 } else { 0.0 } / assign12630_e11742))),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8,)
    }
};
        var_t3 = assign12630_e11747;
        var_t3_dn3 = assign12630_e11747_d_n3;
        var_t3_dn4 = assign12630_e11747_d_n4;
        var_t3_dn5 = assign12630_e11747_d_n5;
        var_t3_dn6 = assign12630_e11747_d_n6;
        var_t3_dn7 = assign12630_e11747_d_n7;
        var_t3_dn8 = assign12630_e11747_d_n8;

        let (assign12640_e11765, assign12640_e11765_d_n3, assign12640_e11765_d_n4, assign12640_e11765_d_n5, assign12640_e11765_d_n6, assign12640_e11765_d_n7, assign12640_e11765_d_n8,) = {
    if ((var_guard126 != 0.0) && (var_guard129 == 0.0)) {
        let assign12640_e11753: f64 = (-var_vds_noswap);
        let assign12640_e11755: f64 = (assign12640_e11753 * var_agisl_i);
        let assign12640_e11757: f64 = (assign12640_e11755 * var_weff);
        let assign12640_e11759: f64 = (assign12640_e11757 * var_t3);
        let assign12640_e11761: f64 = (-var_t2);
        let assign12640_e11762: f64 = { let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign12640_e11763: f64 = (assign12640_e11759 * assign12640_e11762);
        (assign12640_e11763, (((assign12640_e11757 * var_t3_dn3) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn3)))), (((assign12640_e11757 * var_t3_dn4) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn4)))), (((((((-var_vds_noswap_dn5) * var_agisl_i) * var_weff) * var_t3) + (assign12640_e11757 * var_t3_dn5)) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn5)))), (((((((-var_vds_noswap_dn6) * var_agisl_i) * var_weff) * var_t3) + (assign12640_e11757 * var_t3_dn6)) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn6)))), (((assign12640_e11757 * var_t3_dn7) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn7)))), (((assign12640_e11757 * var_t3_dn8) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn8)))),)
    } else {
        (var_t6, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8,)
    }
};
        var_t6 = assign12640_e11765;
        var_t6_dn3 = assign12640_e11765_d_n3;
        var_t6_dn4 = assign12640_e11765_d_n4;
        var_t6_dn5 = assign12640_e11765_d_n5;
        var_t6_dn6 = assign12640_e11765_d_n6;
        var_t6_dn7 = assign12640_e11765_d_n7;
        var_t6_dn8 = assign12640_e11765_d_n8;

        let assign12650_e11768: f64 = if var_sigvds > 0.0 { 1.0 } else { 0.0 };
        var_guard130 = assign12650_e11768;

        let (assign12660_e11774, assign12660_e11774_d_n3, assign12660_e11774_d_n4, assign12660_e11774_d_n5, assign12660_e11774_d_n6, assign12660_e11774_d_n7, assign12660_e11774_d_n8,) = {
    if ((var_guard126 != 0.0) && (var_guard130 != 0.0)) {
        (var_t6, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8,)
    } else {
        (var_igisl, var_igisl_dn3, var_igisl_dn4, var_igisl_dn5, var_igisl_dn6, var_igisl_dn7, var_igisl_dn8,)
    }
};
        var_igisl = assign12660_e11774;
        var_igisl_dn3 = assign12660_e11774_d_n3;
        var_igisl_dn4 = assign12660_e11774_d_n4;
        var_igisl_dn5 = assign12660_e11774_d_n5;
        var_igisl_dn6 = assign12660_e11774_d_n6;
        var_igisl_dn7 = assign12660_e11774_d_n7;
        var_igisl_dn8 = assign12660_e11774_d_n8;

        let (assign12670_e11781, assign12670_e11781_d_n3, assign12670_e11781_d_n4, assign12670_e11781_d_n5, assign12670_e11781_d_n6, assign12670_e11781_d_n7, assign12670_e11781_d_n8,) = {
    if ((var_guard126 != 0.0) && (var_guard130 == 0.0)) {
        (var_t6, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8,)
    } else {
        (var_igidl, var_igidl_dn3, var_igidl_dn4, var_igidl_dn5, var_igidl_dn6, var_igidl_dn7, var_igidl_dn8,)
    }
};
        var_igidl = assign12670_e11781;
        var_igidl_dn3 = assign12670_e11781_d_n3;
        var_igidl_dn4 = assign12670_e11781_d_n4;
        var_igidl_dn5 = assign12670_e11781_d_n5;
        var_igidl_dn6 = assign12670_e11781_d_n6;
        var_igidl_dn7 = assign12670_e11781_d_n7;
        var_igidl_dn8 = assign12670_e11781_d_n8;

        let assign12680_e11784: f64 = (2.0 * var_vsat_t);
        let assign12680_e11786: f64 = (assign12680_e11784 / var_utotal);
        var_esatnoi = assign12680_e11786;
        var_esatnoi_dn3 = (-((assign12680_e11784 * var_utotal_dn3) / (var_utotal * var_utotal)));
        var_esatnoi_dn4 = ((((2.0 * var_vsat_t_dn4) * var_utotal) - (assign12680_e11784 * var_utotal_dn4)) / (var_utotal * var_utotal));
        var_esatnoi_dn5 = (-((assign12680_e11784 * var_utotal_dn5) / (var_utotal * var_utotal)));
        var_esatnoi_dn6 = (-((assign12680_e11784 * var_utotal_dn6) / (var_utotal * var_utotal)));
        var_esatnoi_dn7 = (-((assign12680_e11784 * var_utotal_dn7) / (var_utotal * var_utotal)));
        var_esatnoi_dn8 = (-((assign12680_e11784 * var_utotal_dn8) / (var_utotal * var_utotal)));

        let assign12690_e11797: f64 = if (((p.p288 > 0.0) || (p.p289 > 0.0)) || (p.p290 > 0.0)) { 1.0 } else { 0.0 };
        var_guard131 = assign12690_e11797;

        let (assign12700_e11805,) = {
    if (var_guard131 != 0.0) {
        let assign12700_e11802: f64 = (2.0 * var_lintnoi_i);
        let assign12700_e11803: f64 = (var_leff - assign12700_e11802);
        (assign12700_e11803,)
    } else {
        (var_leffnoi,)
    }
};
        var_leffnoi = assign12700_e11805;

        let (assign12710_e11811,) = {
    if (var_guard131 != 0.0) {
        let assign12710_e11809: f64 = (var_leffnoi * var_leffnoi);
        (assign12710_e11809,)
    } else {
        (var_leffnoisq,)
    }
};
        var_leffnoisq = assign12710_e11811;

        let assign12720_e11814: f64 = if p.p287 <= 0.0 { 1.0 } else { 0.0 };
        var_guard132 = assign12720_e11814;

        let (assign12730_e11820, assign12730_e11820_d_n3, assign12730_e11820_d_n4, assign12730_e11820_d_n5, assign12730_e11820_d_n6, assign12730_e11820_d_n7, assign12730_e11820_d_n8,) = {
    if ((var_guard131 != 0.0) && (var_guard132 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_delclm, var_delclm_dn3, var_delclm_dn4, var_delclm_dn5, var_delclm_dn6, var_delclm_dn7, var_delclm_dn8,)
    }
};
        var_delclm = assign12730_e11820;
        var_delclm_dn3 = assign12730_e11820_d_n3;
        var_delclm_dn4 = assign12730_e11820_d_n4;
        var_delclm_dn5 = assign12730_e11820_d_n5;
        var_delclm_dn6 = assign12730_e11820_d_n6;
        var_delclm_dn7 = assign12730_e11820_d_n7;
        var_delclm_dn8 = assign12730_e11820_d_n8;

        let (assign12740_e11833, assign12740_e11833_d_n3, assign12740_e11833_d_n4, assign12740_e11833_d_n5, assign12740_e11833_d_n6, assign12740_e11833_d_n7, assign12740_e11833_d_n8,) = {
    if ((var_guard131 != 0.0) && (var_guard132 == 0.0)) {
        let assign12740_e11827: f64 = (var_diffvds / var_litl);
        let assign12740_e11829: f64 = (assign12740_e11827 + p.p287);
        let assign12740_e11831: f64 = (assign12740_e11829 / var_esatnoi);
        (assign12740_e11831, ((((var_diffvds_dn3 / var_litl) * var_esatnoi) - (assign12740_e11829 * var_esatnoi_dn3)) / (var_esatnoi * var_esatnoi)), ((((var_diffvds_dn4 / var_litl) * var_esatnoi) - (assign12740_e11829 * var_esatnoi_dn4)) / (var_esatnoi * var_esatnoi)), ((((var_diffvds_dn5 / var_litl) * var_esatnoi) - (assign12740_e11829 * var_esatnoi_dn5)) / (var_esatnoi * var_esatnoi)), ((((var_diffvds_dn6 / var_litl) * var_esatnoi) - (assign12740_e11829 * var_esatnoi_dn6)) / (var_esatnoi * var_esatnoi)), ((((var_diffvds_dn7 / var_litl) * var_esatnoi) - (assign12740_e11829 * var_esatnoi_dn7)) / (var_esatnoi * var_esatnoi)), ((((var_diffvds_dn8 / var_litl) * var_esatnoi) - (assign12740_e11829 * var_esatnoi_dn8)) / (var_esatnoi * var_esatnoi)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign12740_e11833;
        var_t0_dn3 = assign12740_e11833_d_n3;
        var_t0_dn4 = assign12740_e11833_d_n4;
        var_t0_dn5 = assign12740_e11833_d_n5;
        var_t0_dn6 = assign12740_e11833_d_n6;
        var_t0_dn7 = assign12740_e11833_d_n7;
        var_t0_dn8 = assign12740_e11833_d_n8;

        let (assign12750_e11845, assign12750_e11845_d_n3, assign12750_e11845_d_n4, assign12750_e11845_d_n5, assign12750_e11845_d_n6, assign12750_e11845_d_n7, assign12750_e11845_d_n8,) = {
    if ((var_guard131 != 0.0) && (var_guard132 == 0.0)) {
        let assign12750_e11841: f64 = (var_t0).max(1e-38);
        let assign12750_e11842: f64 = (assign12750_e11841).ln();
        let assign12750_e11843: f64 = (var_litl * assign12750_e11842);
        (assign12750_e11843, (var_litl * (if var_t0 >= 1e-38 { var_t0_dn3 } else { 0.0 } / assign12750_e11841)), (var_litl * (if var_t0 >= 1e-38 { var_t0_dn4 } else { 0.0 } / assign12750_e11841)), (var_litl * (if var_t0 >= 1e-38 { var_t0_dn5 } else { 0.0 } / assign12750_e11841)), (var_litl * (if var_t0 >= 1e-38 { var_t0_dn6 } else { 0.0 } / assign12750_e11841)), (var_litl * (if var_t0 >= 1e-38 { var_t0_dn7 } else { 0.0 } / assign12750_e11841)), (var_litl * (if var_t0 >= 1e-38 { var_t0_dn8 } else { 0.0 } / assign12750_e11841)),)
    } else {
        (var_delclm, var_delclm_dn3, var_delclm_dn4, var_delclm_dn5, var_delclm_dn6, var_delclm_dn7, var_delclm_dn8,)
    }
};
        var_delclm = assign12750_e11845;
        var_delclm_dn3 = assign12750_e11845_d_n3;
        var_delclm_dn4 = assign12750_e11845_d_n4;
        var_delclm_dn5 = assign12750_e11845_d_n5;
        var_delclm_dn6 = assign12750_e11845_d_n6;
        var_delclm_dn7 = assign12750_e11845_d_n7;
        var_delclm_dn8 = assign12750_e11845_d_n8;

        let assign12760_e11848: f64 = if var_delclm < 0.0 { 1.0 } else { 0.0 };
        var_guard133 = assign12760_e11848;

        let (assign12770_e11857, assign12770_e11857_d_n3, assign12770_e11857_d_n4, assign12770_e11857_d_n5, assign12770_e11857_d_n6, assign12770_e11857_d_n7, assign12770_e11857_d_n8,) = {
    if (((var_guard131 != 0.0) && (var_guard132 == 0.0)) && (var_guard133 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_delclm, var_delclm_dn3, var_delclm_dn4, var_delclm_dn5, var_delclm_dn6, var_delclm_dn7, var_delclm_dn8,)
    }
};
        var_delclm = assign12770_e11857;
        var_delclm_dn3 = assign12770_e11857_d_n3;
        var_delclm_dn4 = assign12770_e11857_d_n4;
        var_delclm_dn5 = assign12770_e11857_d_n5;
        var_delclm_dn6 = assign12770_e11857_d_n6;
        var_delclm_dn7 = assign12770_e11857_d_n7;
        var_delclm_dn8 = assign12770_e11857_d_n8;

        let assign12780_e11860: f64 = if p.p22 == 1.0 { 1.0 } else { 0.0 };
        var_guard134 = assign12780_e11860;

        let (assign12790_e11868, assign12790_e11868_d_n3, assign12790_e11868_d_n4, assign12790_e11868_d_n5, assign12790_e11868_d_n6, assign12790_e11868_d_n7, assign12790_e11868_d_n8,) = {
    if ((var_guard131 != 0.0) && (var_guard134 != 0.0)) {
        let assign12790_e11866: f64 = (var_qia2 / var_qsref_i);
        (assign12790_e11866, (var_qia2_dn3 / var_qsref_i), (var_qia2_dn4 / var_qsref_i), (var_qia2_dn5 / var_qsref_i), (var_qia2_dn6 / var_qsref_i), (var_qia2_dn7 / var_qsref_i), (var_qia2_dn8 / var_qsref_i),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign12790_e11868;
        var_t1_dn3 = assign12790_e11868_d_n3;
        var_t1_dn4 = assign12790_e11868_d_n4;
        var_t1_dn5 = assign12790_e11868_d_n5;
        var_t1_dn6 = assign12790_e11868_d_n6;
        var_t1_dn7 = assign12790_e11868_d_n7;
        var_t1_dn8 = assign12790_e11868_d_n8;

        let (assign12800_e11878, assign12800_e11878_d_n3, assign12800_e11878_d_n4, assign12800_e11878_d_n5, assign12800_e11878_d_n6, assign12800_e11878_d_n7, assign12800_e11878_d_n8,) = {
    if ((var_guard131 != 0.0) && (var_guard134 != 0.0)) {
        let assign12800_e11875: f64 = (var_t1).powf(var_mpower_i);
        let assign12800_e11876: f64 = (1.0 + assign12800_e11875);
        (assign12800_e11876, if 0.0 == 0.0 && ((var_mpower_i) as f64).is_finite() && ((var_mpower_i) as f64).fract() == 0.0 { if var_mpower_i == 0.0 { 0.0 } else { (var_mpower_i * ((var_t1).powf(var_mpower_i - 1.0) * var_t1_dn3)) } } else { (assign12800_e11875 * (var_mpower_i * (var_t1_dn3 / var_t1))) }, if 0.0 == 0.0 && ((var_mpower_i) as f64).is_finite() && ((var_mpower_i) as f64).fract() == 0.0 { if var_mpower_i == 0.0 { 0.0 } else { (var_mpower_i * ((var_t1).powf(var_mpower_i - 1.0) * var_t1_dn4)) } } else { (assign12800_e11875 * (var_mpower_i * (var_t1_dn4 / var_t1))) }, if 0.0 == 0.0 && ((var_mpower_i) as f64).is_finite() && ((var_mpower_i) as f64).fract() == 0.0 { if var_mpower_i == 0.0 { 0.0 } else { (var_mpower_i * ((var_t1).powf(var_mpower_i - 1.0) * var_t1_dn5)) } } else { (assign12800_e11875 * (var_mpower_i * (var_t1_dn5 / var_t1))) }, if 0.0 == 0.0 && ((var_mpower_i) as f64).is_finite() && ((var_mpower_i) as f64).fract() == 0.0 { if var_mpower_i == 0.0 { 0.0 } else { (var_mpower_i * ((var_t1).powf(var_mpower_i - 1.0) * var_t1_dn6)) } } else { (assign12800_e11875 * (var_mpower_i * (var_t1_dn6 / var_t1))) }, if 0.0 == 0.0 && ((var_mpower_i) as f64).is_finite() && ((var_mpower_i) as f64).fract() == 0.0 { if var_mpower_i == 0.0 { 0.0 } else { (var_mpower_i * ((var_t1).powf(var_mpower_i - 1.0) * var_t1_dn7)) } } else { (assign12800_e11875 * (var_mpower_i * (var_t1_dn7 / var_t1))) }, if 0.0 == 0.0 && ((var_mpower_i) as f64).is_finite() && ((var_mpower_i) as f64).fract() == 0.0 { if var_mpower_i == 0.0 { 0.0 } else { (var_mpower_i * ((var_t1).powf(var_mpower_i - 1.0) * var_t1_dn8)) } } else { (assign12800_e11875 * (var_mpower_i * (var_t1_dn8 / var_t1))) },)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8,)
    }
};
        var_t2 = assign12800_e11878;
        var_t2_dn3 = assign12800_e11878_d_n3;
        var_t2_dn4 = assign12800_e11878_d_n4;
        var_t2_dn5 = assign12800_e11878_d_n5;
        var_t2_dn6 = assign12800_e11878_d_n6;
        var_t2_dn7 = assign12800_e11878_d_n7;
        var_t2_dn8 = assign12800_e11878_d_n8;

        let (assign12810_e11886, assign12810_e11886_d_n3, assign12810_e11886_d_n4, assign12810_e11886_d_n5, assign12810_e11886_d_n6, assign12810_e11886_d_n7, assign12810_e11886_d_n8,) = {
    if ((var_guard131 != 0.0) && (var_guard134 != 0.0)) {
        let assign12810_e11884: f64 = (var_noia2_i / var_t2);
        (assign12810_e11884, (-((var_noia2_i * var_t2_dn3) / (var_t2 * var_t2))), (-((var_noia2_i * var_t2_dn4) / (var_t2 * var_t2))), (-((var_noia2_i * var_t2_dn5) / (var_t2 * var_t2))), (-((var_noia2_i * var_t2_dn6) / (var_t2 * var_t2))), (-((var_noia2_i * var_t2_dn7) / (var_t2 * var_t2))), (-((var_noia2_i * var_t2_dn8) / (var_t2 * var_t2))),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8,)
    }
};
        var_t3 = assign12810_e11886;
        var_t3_dn3 = assign12810_e11886_d_n3;
        var_t3_dn4 = assign12810_e11886_d_n4;
        var_t3_dn5 = assign12810_e11886_d_n5;
        var_t3_dn6 = assign12810_e11886_d_n6;
        var_t3_dn7 = assign12810_e11886_d_n7;
        var_t3_dn8 = assign12810_e11886_d_n8;

        let (assign12820_e11894, assign12820_e11894_d_n3, assign12820_e11894_d_n4, assign12820_e11894_d_n5, assign12820_e11894_d_n6, assign12820_e11894_d_n7, assign12820_e11894_d_n8,) = {
    if ((var_guard131 != 0.0) && (var_guard134 != 0.0)) {
        let assign12820_e11892: f64 = (var_t3 / p.p288);
        (assign12820_e11892, (var_t3_dn3 / p.p288), (var_t3_dn4 / p.p288), (var_t3_dn5 / p.p288), (var_t3_dn6 / p.p288), (var_t3_dn7 / p.p288), (var_t3_dn8 / p.p288),)
    } else {
        (var_t4, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8,)
    }
};
        var_t4 = assign12820_e11894;
        var_t4_dn3 = assign12820_e11894_d_n3;
        var_t4_dn4 = assign12820_e11894_d_n4;
        var_t4_dn5 = assign12820_e11894_d_n5;
        var_t4_dn6 = assign12820_e11894_d_n6;
        var_t4_dn7 = assign12820_e11894_d_n7;
        var_t4_dn8 = assign12820_e11894_d_n8;

        let (assign12830_e11919, assign12830_e11919_d_n3, assign12830_e11919_d_n4, assign12830_e11919_d_n5, assign12830_e11919_d_n6, assign12830_e11919_d_n7, assign12830_e11919_d_n8,) = {
    if ((var_guard131 != 0.0) && (var_guard134 != 0.0)) {
        let assign12830_e11901: f64 = (var_t4 + 1.0);
        let assign12830_e11904: f64 = (var_t4 - 1.0);
        let assign12830_e11907: f64 = (var_t4 - 1.0);
        let assign12830_e11908: f64 = (assign12830_e11904 * assign12830_e11907);
        let assign12830_e11911: f64 = (0.25 * p.p292);
        let assign12830_e11913: f64 = (assign12830_e11911 * p.p292);
        let assign12830_e11914: f64 = (assign12830_e11908 + assign12830_e11913);
        let assign12830_e11915: f64 = (assign12830_e11914).sqrt();
        let assign12830_e11916: f64 = (assign12830_e11901 + assign12830_e11915);
        let assign12830_e11917: f64 = (0.5 * assign12830_e11916);
        (assign12830_e11917, (0.5 * (var_t4_dn3 + (((var_t4_dn3 * assign12830_e11907) + (assign12830_e11904 * var_t4_dn3)) / (2.0 * assign12830_e11915)))), (0.5 * (var_t4_dn4 + (((var_t4_dn4 * assign12830_e11907) + (assign12830_e11904 * var_t4_dn4)) / (2.0 * assign12830_e11915)))), (0.5 * (var_t4_dn5 + (((var_t4_dn5 * assign12830_e11907) + (assign12830_e11904 * var_t4_dn5)) / (2.0 * assign12830_e11915)))), (0.5 * (var_t4_dn6 + (((var_t4_dn6 * assign12830_e11907) + (assign12830_e11904 * var_t4_dn6)) / (2.0 * assign12830_e11915)))), (0.5 * (var_t4_dn7 + (((var_t4_dn7 * assign12830_e11907) + (assign12830_e11904 * var_t4_dn7)) / (2.0 * assign12830_e11915)))), (0.5 * (var_t4_dn8 + (((var_t4_dn8 * assign12830_e11907) + (assign12830_e11904 * var_t4_dn8)) / (2.0 * assign12830_e11915)))),)
    } else {
        (var_t5, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8,)
    }
};
        var_t5 = assign12830_e11919;
        var_t5_dn3 = assign12830_e11919_d_n3;
        var_t5_dn4 = assign12830_e11919_d_n4;
        var_t5_dn5 = assign12830_e11919_d_n5;
        var_t5_dn6 = assign12830_e11919_d_n6;
        var_t5_dn7 = assign12830_e11919_d_n7;
        var_t5_dn8 = assign12830_e11919_d_n8;

        let (assign12840_e11927, assign12840_e11927_d_n3, assign12840_e11927_d_n4, assign12840_e11927_d_n5, assign12840_e11927_d_n6, assign12840_e11927_d_n7, assign12840_e11927_d_n8,) = {
    if ((var_guard131 != 0.0) && (var_guard134 != 0.0)) {
        let assign12840_e11925: f64 = (p.p288 * var_t5);
        (assign12840_e11925, (p.p288 * var_t5_dn3), (p.p288 * var_t5_dn4), (p.p288 * var_t5_dn5), (p.p288 * var_t5_dn6), (p.p288 * var_t5_dn7), (p.p288 * var_t5_dn8),)
    } else {
        (var_noiaeff, var_noiaeff_dn3, var_noiaeff_dn4, var_noiaeff_dn5, var_noiaeff_dn6, var_noiaeff_dn7, var_noiaeff_dn8,)
    }
};
        var_noiaeff = assign12840_e11927;
        var_noiaeff_dn3 = assign12840_e11927_d_n3;
        var_noiaeff_dn4 = assign12840_e11927_d_n4;
        var_noiaeff_dn5 = assign12840_e11927_d_n5;
        var_noiaeff_dn6 = assign12840_e11927_d_n6;
        var_noiaeff_dn7 = assign12840_e11927_d_n7;
        var_noiaeff_dn8 = assign12840_e11927_d_n8;

        *var_delclm_slot = var_delclm;
        *var_delclm_dn3_slot = var_delclm_dn3;
        *var_delclm_dn4_slot = var_delclm_dn4;
        *var_delclm_dn5_slot = var_delclm_dn5;
        *var_delclm_dn6_slot = var_delclm_dn6;
        *var_delclm_dn7_slot = var_delclm_dn7;
        *var_delclm_dn8_slot = var_delclm_dn8;
        *var_esatnoi_slot = var_esatnoi;
        *var_esatnoi_dn3_slot = var_esatnoi_dn3;
        *var_esatnoi_dn4_slot = var_esatnoi_dn4;
        *var_esatnoi_dn5_slot = var_esatnoi_dn5;
        *var_esatnoi_dn6_slot = var_esatnoi_dn6;
        *var_esatnoi_dn7_slot = var_esatnoi_dn7;
        *var_esatnoi_dn8_slot = var_esatnoi_dn8;
        *var_guard128_slot = var_guard128;
        *var_guard129_slot = var_guard129;
        *var_guard130_slot = var_guard130;
        *var_guard131_slot = var_guard131;
        *var_guard132_slot = var_guard132;
        *var_guard133_slot = var_guard133;
        *var_guard134_slot = var_guard134;
        *var_igidl_slot = var_igidl;
        *var_igidl_dn3_slot = var_igidl_dn3;
        *var_igidl_dn4_slot = var_igidl_dn4;
        *var_igidl_dn5_slot = var_igidl_dn5;
        *var_igidl_dn6_slot = var_igidl_dn6;
        *var_igidl_dn7_slot = var_igidl_dn7;
        *var_igidl_dn8_slot = var_igidl_dn8;
        *var_igisl_slot = var_igisl;
        *var_igisl_dn3_slot = var_igisl_dn3;
        *var_igisl_dn4_slot = var_igisl_dn4;
        *var_igisl_dn5_slot = var_igisl_dn5;
        *var_igisl_dn6_slot = var_igisl_dn6;
        *var_igisl_dn7_slot = var_igisl_dn7;
        *var_igisl_dn8_slot = var_igisl_dn8;
        *var_leffnoi_slot = var_leffnoi;
        *var_leffnoisq_slot = var_leffnoisq;
        *var_noiaeff_slot = var_noiaeff;
        *var_noiaeff_dn3_slot = var_noiaeff_dn3;
        *var_noiaeff_dn4_slot = var_noiaeff_dn4;
        *var_noiaeff_dn5_slot = var_noiaeff_dn5;
        *var_noiaeff_dn6_slot = var_noiaeff_dn6;
        *var_noiaeff_dn7_slot = var_noiaeff_dn7;
        *var_noiaeff_dn8_slot = var_noiaeff_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t4_slot = var_t4;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t5_slot = var_t5;
        *var_t5_dn3_slot = var_t5_dn3;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t6_slot = var_t6;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
    }

    pub(super) fn stamp_transient_block_29(
        p: &Parameters,
        var_cit_i: f64,
        var_coxeff: f64,
        var_coxeff_dn3: f64,
        var_coxeff_dn4: f64,
        var_coxeff_dn5: f64,
        var_coxeff_dn6: f64,
        var_coxeff_dn7: f64,
        var_coxeff_dn8: f64,
        var_delclm: f64,
        var_delclm_dn3: f64,
        var_delclm_dn4: f64,
        var_delclm_dn5: f64,
        var_delclm_dn6: f64,
        var_delclm_dn7: f64,
        var_delclm_dn8: f64,
        var_devsign: f64,
        var_guard131: f64,
        var_guard134: f64,
        var_ids: f64,
        var_ids_dn3: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_ids_dn8: f64,
        var_leffnoi: f64,
        var_leffnoisq: f64,
        var_qbg: f64,
        var_qbg_dn3: f64,
        var_qbg_dn4: f64,
        var_qbg_dn5: f64,
        var_qbg_dn6: f64,
        var_qbg_dn7: f64,
        var_qbg_dn8: f64,
        var_qdbg: f64,
        var_qdbg_dn3: f64,
        var_qdbg_dn4: f64,
        var_qdbg_dn5: f64,
        var_qdbg_dn6: f64,
        var_qdbg_dn7: f64,
        var_qdbg_dn8: f64,
        var_qfg: f64,
        var_qfg_dn3: f64,
        var_qfg_dn4: f64,
        var_qfg_dn5: f64,
        var_qfg_dn6: f64,
        var_qfg_dn7: f64,
        var_qfg_dn8: f64,
        var_qfgd_parasitic: f64,
        var_qfgd_parasitic_dn3: f64,
        var_qfgd_parasitic_dn4: f64,
        var_qfgd_parasitic_dn5: f64,
        var_qfgd_parasitic_dn6: f64,
        var_qfgd_parasitic_dn7: f64,
        var_qfgd_parasitic_dn8: f64,
        var_qfgs_parasitic: f64,
        var_qfgs_parasitic_dn3: f64,
        var_qfgs_parasitic_dn4: f64,
        var_qfgs_parasitic_dn5: f64,
        var_qfgs_parasitic_dn6: f64,
        var_qfgs_parasitic_dn7: f64,
        var_qfgs_parasitic_dn8: f64,
        var_qid: f64,
        var_qid_dn3: f64,
        var_qid_dn4: f64,
        var_qid_dn5: f64,
        var_qid_dn6: f64,
        var_qid_dn7: f64,
        var_qid_dn8: f64,
        var_qis: f64,
        var_qis_dn3: f64,
        var_qis_dn4: f64,
        var_qis_dn5: f64,
        var_qis_dn6: f64,
        var_qis_dn7: f64,
        var_qis_dn8: f64,
        var_qsbg: f64,
        var_qsbg_dn3: f64,
        var_qsbg_dn4: f64,
        var_qsbg_dn5: f64,
        var_qsbg_dn6: f64,
        var_qsbg_dn7: f64,
        var_qsbg_dn8: f64,
        var_sigvds: f64,
        var_utotal: f64,
        var_utotal_dn3: f64,
        var_utotal_dn4: f64,
        var_utotal_dn5: f64,
        var_utotal_dn6: f64,
        var_utotal_dn7: f64,
        var_utotal_dn8: f64,
        var_vtm: f64,
        var_vtm_dn4: f64,
        var_weff: f64,
        var_guard136_slot: &mut f64,
        var_n0_slot: &mut f64,
        var_n0_dn3_slot: &mut f64,
        var_n0_dn4_slot: &mut f64,
        var_n0_dn5_slot: &mut f64,
        var_n0_dn6_slot: &mut f64,
        var_n0_dn7_slot: &mut f64,
        var_n0_dn8_slot: &mut f64,
        var_nl_slot: &mut f64,
        var_nl_dn3_slot: &mut f64,
        var_nl_dn4_slot: &mut f64,
        var_nl_dn5_slot: &mut f64,
        var_nl_dn6_slot: &mut f64,
        var_nl_dn7_slot: &mut f64,
        var_nl_dn8_slot: &mut f64,
        var_noiaeff_slot: &mut f64,
        var_noiaeff_dn3_slot: &mut f64,
        var_noiaeff_dn4_slot: &mut f64,
        var_noiaeff_dn5_slot: &mut f64,
        var_noiaeff_dn6_slot: &mut f64,
        var_noiaeff_dn7_slot: &mut f64,
        var_noiaeff_dn8_slot: &mut f64,
        var_nstar_slot: &mut f64,
        var_nstar_dn3_slot: &mut f64,
        var_nstar_dn4_slot: &mut f64,
        var_nstar_dn5_slot: &mut f64,
        var_nstar_dn6_slot: &mut f64,
        var_nstar_dn7_slot: &mut f64,
        var_nstar_dn8_slot: &mut f64,
        var_qbgi_slot: &mut f64,
        var_qbgi_dn3_slot: &mut f64,
        var_qbgi_dn4_slot: &mut f64,
        var_qbgi_dn5_slot: &mut f64,
        var_qbgi_dn6_slot: &mut f64,
        var_qbgi_dn7_slot: &mut f64,
        var_qbgi_dn8_slot: &mut f64,
        var_qd_slot: &mut f64,
        var_qd_dn3_slot: &mut f64,
        var_qd_dn4_slot: &mut f64,
        var_qd_dn5_slot: &mut f64,
        var_qd_dn6_slot: &mut f64,
        var_qd_dn7_slot: &mut f64,
        var_qd_dn8_slot: &mut f64,
        var_qdi_slot: &mut f64,
        var_qdi_dn3_slot: &mut f64,
        var_qdi_dn4_slot: &mut f64,
        var_qdi_dn5_slot: &mut f64,
        var_qdi_dn6_slot: &mut f64,
        var_qdi_dn7_slot: &mut f64,
        var_qdi_dn8_slot: &mut f64,
        var_qfgi_slot: &mut f64,
        var_qfgi_dn3_slot: &mut f64,
        var_qfgi_dn4_slot: &mut f64,
        var_qfgi_dn5_slot: &mut f64,
        var_qfgi_dn6_slot: &mut f64,
        var_qfgi_dn7_slot: &mut f64,
        var_qfgi_dn8_slot: &mut f64,
        var_qs_slot: &mut f64,
        var_qs_dn3_slot: &mut f64,
        var_qs_dn4_slot: &mut f64,
        var_qs_dn5_slot: &mut f64,
        var_qs_dn6_slot: &mut f64,
        var_qs_dn7_slot: &mut f64,
        var_qs_dn8_slot: &mut f64,
        var_qsi_slot: &mut f64,
        var_qsi_dn3_slot: &mut f64,
        var_qsi_dn4_slot: &mut f64,
        var_qsi_dn5_slot: &mut f64,
        var_qsi_dn6_slot: &mut f64,
        var_qsi_dn7_slot: &mut f64,
        var_qsi_dn8_slot: &mut f64,
        var_ssi_slot: &mut f64,
        var_ssi_dn3_slot: &mut f64,
        var_ssi_dn4_slot: &mut f64,
        var_ssi_dn5_slot: &mut f64,
        var_ssi_dn6_slot: &mut f64,
        var_ssi_dn7_slot: &mut f64,
        var_ssi_dn8_slot: &mut f64,
        var_swi_slot: &mut f64,
        var_swi_dn3_slot: &mut f64,
        var_swi_dn4_slot: &mut f64,
        var_swi_dn5_slot: &mut f64,
        var_swi_dn6_slot: &mut f64,
        var_swi_dn7_slot: &mut f64,
        var_swi_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t10_slot: &mut f64,
        var_t10_dn3_slot: &mut f64,
        var_t10_dn4_slot: &mut f64,
        var_t10_dn5_slot: &mut f64,
        var_t10_dn6_slot: &mut f64,
        var_t10_dn7_slot: &mut f64,
        var_t10_dn8_slot: &mut f64,
        var_t11_slot: &mut f64,
        var_t11_dn3_slot: &mut f64,
        var_t11_dn4_slot: &mut f64,
        var_t11_dn5_slot: &mut f64,
        var_t11_dn6_slot: &mut f64,
        var_t11_dn7_slot: &mut f64,
        var_t11_dn8_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t7_slot: &mut f64,
        var_t7_dn3_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn7_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t8_slot: &mut f64,
        var_t8_dn3_slot: &mut f64,
        var_t8_dn4_slot: &mut f64,
        var_t8_dn5_slot: &mut f64,
        var_t8_dn6_slot: &mut f64,
        var_t8_dn7_slot: &mut f64,
        var_t8_dn8_slot: &mut f64,
        var_t9_slot: &mut f64,
        var_t9_dn3_slot: &mut f64,
        var_t9_dn4_slot: &mut f64,
        var_t9_dn5_slot: &mut f64,
        var_t9_dn6_slot: &mut f64,
        var_t9_dn7_slot: &mut f64,
        var_t9_dn8_slot: &mut f64,
    ) {
        let mut var_guard136: f64 = *var_guard136_slot;
        let mut var_n0: f64 = *var_n0_slot;
        let mut var_n0_dn3: f64 = *var_n0_dn3_slot;
        let mut var_n0_dn4: f64 = *var_n0_dn4_slot;
        let mut var_n0_dn5: f64 = *var_n0_dn5_slot;
        let mut var_n0_dn6: f64 = *var_n0_dn6_slot;
        let mut var_n0_dn7: f64 = *var_n0_dn7_slot;
        let mut var_n0_dn8: f64 = *var_n0_dn8_slot;
        let mut var_nl: f64 = *var_nl_slot;
        let mut var_nl_dn3: f64 = *var_nl_dn3_slot;
        let mut var_nl_dn4: f64 = *var_nl_dn4_slot;
        let mut var_nl_dn5: f64 = *var_nl_dn5_slot;
        let mut var_nl_dn6: f64 = *var_nl_dn6_slot;
        let mut var_nl_dn7: f64 = *var_nl_dn7_slot;
        let mut var_nl_dn8: f64 = *var_nl_dn8_slot;
        let mut var_noiaeff: f64 = *var_noiaeff_slot;
        let mut var_noiaeff_dn3: f64 = *var_noiaeff_dn3_slot;
        let mut var_noiaeff_dn4: f64 = *var_noiaeff_dn4_slot;
        let mut var_noiaeff_dn5: f64 = *var_noiaeff_dn5_slot;
        let mut var_noiaeff_dn6: f64 = *var_noiaeff_dn6_slot;
        let mut var_noiaeff_dn7: f64 = *var_noiaeff_dn7_slot;
        let mut var_noiaeff_dn8: f64 = *var_noiaeff_dn8_slot;
        let mut var_nstar: f64 = *var_nstar_slot;
        let mut var_nstar_dn3: f64 = *var_nstar_dn3_slot;
        let mut var_nstar_dn4: f64 = *var_nstar_dn4_slot;
        let mut var_nstar_dn5: f64 = *var_nstar_dn5_slot;
        let mut var_nstar_dn6: f64 = *var_nstar_dn6_slot;
        let mut var_nstar_dn7: f64 = *var_nstar_dn7_slot;
        let mut var_nstar_dn8: f64 = *var_nstar_dn8_slot;
        let mut var_qbgi: f64 = *var_qbgi_slot;
        let mut var_qbgi_dn3: f64 = *var_qbgi_dn3_slot;
        let mut var_qbgi_dn4: f64 = *var_qbgi_dn4_slot;
        let mut var_qbgi_dn5: f64 = *var_qbgi_dn5_slot;
        let mut var_qbgi_dn6: f64 = *var_qbgi_dn6_slot;
        let mut var_qbgi_dn7: f64 = *var_qbgi_dn7_slot;
        let mut var_qbgi_dn8: f64 = *var_qbgi_dn8_slot;
        let mut var_qd: f64 = *var_qd_slot;
        let mut var_qd_dn3: f64 = *var_qd_dn3_slot;
        let mut var_qd_dn4: f64 = *var_qd_dn4_slot;
        let mut var_qd_dn5: f64 = *var_qd_dn5_slot;
        let mut var_qd_dn6: f64 = *var_qd_dn6_slot;
        let mut var_qd_dn7: f64 = *var_qd_dn7_slot;
        let mut var_qd_dn8: f64 = *var_qd_dn8_slot;
        let mut var_qdi: f64 = *var_qdi_slot;
        let mut var_qdi_dn3: f64 = *var_qdi_dn3_slot;
        let mut var_qdi_dn4: f64 = *var_qdi_dn4_slot;
        let mut var_qdi_dn5: f64 = *var_qdi_dn5_slot;
        let mut var_qdi_dn6: f64 = *var_qdi_dn6_slot;
        let mut var_qdi_dn7: f64 = *var_qdi_dn7_slot;
        let mut var_qdi_dn8: f64 = *var_qdi_dn8_slot;
        let mut var_qfgi: f64 = *var_qfgi_slot;
        let mut var_qfgi_dn3: f64 = *var_qfgi_dn3_slot;
        let mut var_qfgi_dn4: f64 = *var_qfgi_dn4_slot;
        let mut var_qfgi_dn5: f64 = *var_qfgi_dn5_slot;
        let mut var_qfgi_dn6: f64 = *var_qfgi_dn6_slot;
        let mut var_qfgi_dn7: f64 = *var_qfgi_dn7_slot;
        let mut var_qfgi_dn8: f64 = *var_qfgi_dn8_slot;
        let mut var_qs: f64 = *var_qs_slot;
        let mut var_qs_dn3: f64 = *var_qs_dn3_slot;
        let mut var_qs_dn4: f64 = *var_qs_dn4_slot;
        let mut var_qs_dn5: f64 = *var_qs_dn5_slot;
        let mut var_qs_dn6: f64 = *var_qs_dn6_slot;
        let mut var_qs_dn7: f64 = *var_qs_dn7_slot;
        let mut var_qs_dn8: f64 = *var_qs_dn8_slot;
        let mut var_qsi: f64 = *var_qsi_slot;
        let mut var_qsi_dn3: f64 = *var_qsi_dn3_slot;
        let mut var_qsi_dn4: f64 = *var_qsi_dn4_slot;
        let mut var_qsi_dn5: f64 = *var_qsi_dn5_slot;
        let mut var_qsi_dn6: f64 = *var_qsi_dn6_slot;
        let mut var_qsi_dn7: f64 = *var_qsi_dn7_slot;
        let mut var_qsi_dn8: f64 = *var_qsi_dn8_slot;
        let mut var_ssi: f64 = *var_ssi_slot;
        let mut var_ssi_dn3: f64 = *var_ssi_dn3_slot;
        let mut var_ssi_dn4: f64 = *var_ssi_dn4_slot;
        let mut var_ssi_dn5: f64 = *var_ssi_dn5_slot;
        let mut var_ssi_dn6: f64 = *var_ssi_dn6_slot;
        let mut var_ssi_dn7: f64 = *var_ssi_dn7_slot;
        let mut var_ssi_dn8: f64 = *var_ssi_dn8_slot;
        let mut var_swi: f64 = *var_swi_slot;
        let mut var_swi_dn3: f64 = *var_swi_dn3_slot;
        let mut var_swi_dn4: f64 = *var_swi_dn4_slot;
        let mut var_swi_dn5: f64 = *var_swi_dn5_slot;
        let mut var_swi_dn6: f64 = *var_swi_dn6_slot;
        let mut var_swi_dn7: f64 = *var_swi_dn7_slot;
        let mut var_swi_dn8: f64 = *var_swi_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t10: f64 = *var_t10_slot;
        let mut var_t10_dn3: f64 = *var_t10_dn3_slot;
        let mut var_t10_dn4: f64 = *var_t10_dn4_slot;
        let mut var_t10_dn5: f64 = *var_t10_dn5_slot;
        let mut var_t10_dn6: f64 = *var_t10_dn6_slot;
        let mut var_t10_dn7: f64 = *var_t10_dn7_slot;
        let mut var_t10_dn8: f64 = *var_t10_dn8_slot;
        let mut var_t11: f64 = *var_t11_slot;
        let mut var_t11_dn3: f64 = *var_t11_dn3_slot;
        let mut var_t11_dn4: f64 = *var_t11_dn4_slot;
        let mut var_t11_dn5: f64 = *var_t11_dn5_slot;
        let mut var_t11_dn6: f64 = *var_t11_dn6_slot;
        let mut var_t11_dn7: f64 = *var_t11_dn7_slot;
        let mut var_t11_dn8: f64 = *var_t11_dn8_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t7: f64 = *var_t7_slot;
        let mut var_t7_dn3: f64 = *var_t7_dn3_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn7: f64 = *var_t7_dn7_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t8: f64 = *var_t8_slot;
        let mut var_t8_dn3: f64 = *var_t8_dn3_slot;
        let mut var_t8_dn4: f64 = *var_t8_dn4_slot;
        let mut var_t8_dn5: f64 = *var_t8_dn5_slot;
        let mut var_t8_dn6: f64 = *var_t8_dn6_slot;
        let mut var_t8_dn7: f64 = *var_t8_dn7_slot;
        let mut var_t8_dn8: f64 = *var_t8_dn8_slot;
        let mut var_t9: f64 = *var_t9_slot;
        let mut var_t9_dn3: f64 = *var_t9_dn3_slot;
        let mut var_t9_dn4: f64 = *var_t9_dn4_slot;
        let mut var_t9_dn5: f64 = *var_t9_dn5_slot;
        let mut var_t9_dn6: f64 = *var_t9_dn6_slot;
        let mut var_t9_dn7: f64 = *var_t9_dn7_slot;
        let mut var_t9_dn8: f64 = *var_t9_dn8_slot;

        let (assign12850_e11934, assign12850_e11934_d_n3, assign12850_e11934_d_n4, assign12850_e11934_d_n5, assign12850_e11934_d_n6, assign12850_e11934_d_n7, assign12850_e11934_d_n8,) = {
    if ((var_guard131 != 0.0) && (var_guard134 == 0.0)) {
        (p.p288, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_noiaeff, var_noiaeff_dn3, var_noiaeff_dn4, var_noiaeff_dn5, var_noiaeff_dn6, var_noiaeff_dn7, var_noiaeff_dn8,)
    }
};
        var_noiaeff = assign12850_e11934;
        var_noiaeff_dn3 = assign12850_e11934_d_n3;
        var_noiaeff_dn4 = assign12850_e11934_d_n4;
        var_noiaeff_dn5 = assign12850_e11934_d_n5;
        var_noiaeff_dn6 = assign12850_e11934_d_n6;
        var_noiaeff_dn7 = assign12850_e11934_d_n7;
        var_noiaeff_dn8 = assign12850_e11934_d_n8;

        let (assign12860_e11949, assign12860_e11949_d_n3, assign12860_e11949_d_n4, assign12860_e11949_d_n5, assign12860_e11949_d_n6, assign12860_e11949_d_n7, assign12860_e11949_d_n8,) = {
    if (var_guard131 != 0.0) {
        let assign12860_e11938: f64 = (1.60219e-19 * 1.60219e-19);
        let assign12860_e11940: f64 = (assign12860_e11938 * 1.60219e-19);
        let assign12860_e11942: f64 = (assign12860_e11940 * var_vtm);
        let assign12860_e11944: f64 = (var_ids).abs();
        let assign12860_e11945: f64 = (assign12860_e11942 * assign12860_e11944);
        let assign12860_e11947: f64 = (assign12860_e11945 * var_utotal);
        (assign12860_e11947, (((assign12860_e11942 * if var_ids >= 0.0 { var_ids_dn3 } else { (-var_ids_dn3) }) * var_utotal) + (assign12860_e11945 * var_utotal_dn3)), (((((assign12860_e11940 * var_vtm_dn4) * assign12860_e11944) + (assign12860_e11942 * if var_ids >= 0.0 { var_ids_dn4 } else { (-var_ids_dn4) })) * var_utotal) + (assign12860_e11945 * var_utotal_dn4)), (((assign12860_e11942 * if var_ids >= 0.0 { var_ids_dn5 } else { (-var_ids_dn5) }) * var_utotal) + (assign12860_e11945 * var_utotal_dn5)), (((assign12860_e11942 * if var_ids >= 0.0 { var_ids_dn6 } else { (-var_ids_dn6) }) * var_utotal) + (assign12860_e11945 * var_utotal_dn6)), (((assign12860_e11942 * if var_ids >= 0.0 { var_ids_dn7 } else { (-var_ids_dn7) }) * var_utotal) + (assign12860_e11945 * var_utotal_dn7)), (((assign12860_e11942 * if var_ids >= 0.0 { var_ids_dn8 } else { (-var_ids_dn8) }) * var_utotal) + (assign12860_e11945 * var_utotal_dn8)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign12860_e11949;
        var_t1_dn3 = assign12860_e11949_d_n3;
        var_t1_dn4 = assign12860_e11949_d_n4;
        var_t1_dn5 = assign12860_e11949_d_n5;
        var_t1_dn6 = assign12860_e11949_d_n6;
        var_t1_dn7 = assign12860_e11949_d_n7;
        var_t1_dn8 = assign12860_e11949_d_n8;

        let (assign12870_e11957, assign12870_e11957_d_n3, assign12870_e11957_d_n4, assign12870_e11957_d_n5, assign12870_e11957_d_n6, assign12870_e11957_d_n7, assign12870_e11957_d_n8,) = {
    if (var_guard131 != 0.0) {
        let assign12870_e11953: f64 = (10000000000.0 * var_coxeff);
        let assign12870_e11955: f64 = (assign12870_e11953 * var_leffnoisq);
        (assign12870_e11955, ((10000000000.0 * var_coxeff_dn3) * var_leffnoisq), ((10000000000.0 * var_coxeff_dn4) * var_leffnoisq), ((10000000000.0 * var_coxeff_dn5) * var_leffnoisq), ((10000000000.0 * var_coxeff_dn6) * var_leffnoisq), ((10000000000.0 * var_coxeff_dn7) * var_leffnoisq), ((10000000000.0 * var_coxeff_dn8) * var_leffnoisq),)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8,)
    }
};
        var_t2 = assign12870_e11957;
        var_t2_dn3 = assign12870_e11957_d_n3;
        var_t2_dn4 = assign12870_e11957_d_n4;
        var_t2_dn5 = assign12870_e11957_d_n5;
        var_t2_dn6 = assign12870_e11957_d_n6;
        var_t2_dn7 = assign12870_e11957_d_n7;
        var_t2_dn8 = assign12870_e11957_d_n8;

        let (assign12880_e11965, assign12880_e11965_d_n3, assign12880_e11965_d_n4, assign12880_e11965_d_n5, assign12880_e11965_d_n6, assign12880_e11965_d_n7, assign12880_e11965_d_n8,) = {
    if (var_guard131 != 0.0) {
        let assign12880_e11961: f64 = (var_coxeff * var_qis);
        let assign12880_e11963: f64 = (assign12880_e11961 / 1.60219e-19);
        (assign12880_e11963, (((var_coxeff_dn3 * var_qis) + (var_coxeff * var_qis_dn3)) / 1.60219e-19), (((var_coxeff_dn4 * var_qis) + (var_coxeff * var_qis_dn4)) / 1.60219e-19), (((var_coxeff_dn5 * var_qis) + (var_coxeff * var_qis_dn5)) / 1.60219e-19), (((var_coxeff_dn6 * var_qis) + (var_coxeff * var_qis_dn6)) / 1.60219e-19), (((var_coxeff_dn7 * var_qis) + (var_coxeff * var_qis_dn7)) / 1.60219e-19), (((var_coxeff_dn8 * var_qis) + (var_coxeff * var_qis_dn8)) / 1.60219e-19),)
    } else {
        (var_n0, var_n0_dn3, var_n0_dn4, var_n0_dn5, var_n0_dn6, var_n0_dn7, var_n0_dn8,)
    }
};
        var_n0 = assign12880_e11965;
        var_n0_dn3 = assign12880_e11965_d_n3;
        var_n0_dn4 = assign12880_e11965_d_n4;
        var_n0_dn5 = assign12880_e11965_d_n5;
        var_n0_dn6 = assign12880_e11965_d_n6;
        var_n0_dn7 = assign12880_e11965_d_n7;
        var_n0_dn8 = assign12880_e11965_d_n8;

        let (assign12890_e11973, assign12890_e11973_d_n3, assign12890_e11973_d_n4, assign12890_e11973_d_n5, assign12890_e11973_d_n6, assign12890_e11973_d_n7, assign12890_e11973_d_n8,) = {
    if (var_guard131 != 0.0) {
        let assign12890_e11969: f64 = (var_coxeff * var_qid);
        let assign12890_e11971: f64 = (assign12890_e11969 / 1.60219e-19);
        (assign12890_e11971, (((var_coxeff_dn3 * var_qid) + (var_coxeff * var_qid_dn3)) / 1.60219e-19), (((var_coxeff_dn4 * var_qid) + (var_coxeff * var_qid_dn4)) / 1.60219e-19), (((var_coxeff_dn5 * var_qid) + (var_coxeff * var_qid_dn5)) / 1.60219e-19), (((var_coxeff_dn6 * var_qid) + (var_coxeff * var_qid_dn6)) / 1.60219e-19), (((var_coxeff_dn7 * var_qid) + (var_coxeff * var_qid_dn7)) / 1.60219e-19), (((var_coxeff_dn8 * var_qid) + (var_coxeff * var_qid_dn8)) / 1.60219e-19),)
    } else {
        (var_nl, var_nl_dn3, var_nl_dn4, var_nl_dn5, var_nl_dn6, var_nl_dn7, var_nl_dn8,)
    }
};
        var_nl = assign12890_e11973;
        var_nl_dn3 = assign12890_e11973_d_n3;
        var_nl_dn4 = assign12890_e11973_d_n4;
        var_nl_dn5 = assign12890_e11973_d_n5;
        var_nl_dn6 = assign12890_e11973_d_n6;
        var_nl_dn7 = assign12890_e11973_d_n7;
        var_nl_dn8 = assign12890_e11973_d_n8;

        let (assign12900_e11983, assign12900_e11983_d_n3, assign12900_e11983_d_n4, assign12900_e11983_d_n5, assign12900_e11983_d_n6, assign12900_e11983_d_n7, assign12900_e11983_d_n8,) = {
    if (var_guard131 != 0.0) {
        let assign12900_e11977: f64 = (var_vtm / 1.60219e-19);
        let assign12900_e11980: f64 = (var_coxeff + var_cit_i);
        let assign12900_e11981: f64 = (assign12900_e11977 * assign12900_e11980);
        (assign12900_e11981, (assign12900_e11977 * var_coxeff_dn3), (((var_vtm_dn4 / 1.60219e-19) * assign12900_e11980) + (assign12900_e11977 * var_coxeff_dn4)), (assign12900_e11977 * var_coxeff_dn5), (assign12900_e11977 * var_coxeff_dn6), (assign12900_e11977 * var_coxeff_dn7), (assign12900_e11977 * var_coxeff_dn8),)
    } else {
        (var_nstar, var_nstar_dn3, var_nstar_dn4, var_nstar_dn5, var_nstar_dn6, var_nstar_dn7, var_nstar_dn8,)
    }
};
        var_nstar = assign12900_e11983;
        var_nstar_dn3 = assign12900_e11983_d_n3;
        var_nstar_dn4 = assign12900_e11983_d_n4;
        var_nstar_dn5 = assign12900_e11983_d_n5;
        var_nstar_dn6 = assign12900_e11983_d_n6;
        var_nstar_dn7 = assign12900_e11983_d_n7;
        var_nstar_dn8 = assign12900_e11983_d_n8;

        let (assign12910_e11998, assign12910_e11998_d_n3, assign12910_e11998_d_n4, assign12910_e11998_d_n5, assign12910_e11998_d_n6, assign12910_e11998_d_n7, assign12910_e11998_d_n8,) = {
    if (var_guard131 != 0.0) {
        let assign12910_e11988: f64 = (var_n0 + var_nstar);
        let assign12910_e11991: f64 = (var_nl + var_nstar);
        let assign12910_e11992: f64 = (assign12910_e11988 / assign12910_e11991);
        let assign12910_e11994: f64 = (assign12910_e11992).max(1e-38);
        let assign12910_e11995: f64 = (assign12910_e11994).ln();
        let assign12910_e11996: f64 = (var_noiaeff * assign12910_e11995);
        (assign12910_e11996, ((var_noiaeff_dn3 * assign12910_e11995) + (var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((var_n0_dn3 + var_nstar_dn3) * assign12910_e11991) - (assign12910_e11988 * (var_nl_dn3 + var_nstar_dn3))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((var_noiaeff_dn4 * assign12910_e11995) + (var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((var_n0_dn4 + var_nstar_dn4) * assign12910_e11991) - (assign12910_e11988 * (var_nl_dn4 + var_nstar_dn4))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((var_noiaeff_dn5 * assign12910_e11995) + (var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((var_n0_dn5 + var_nstar_dn5) * assign12910_e11991) - (assign12910_e11988 * (var_nl_dn5 + var_nstar_dn5))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((var_noiaeff_dn6 * assign12910_e11995) + (var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((var_n0_dn6 + var_nstar_dn6) * assign12910_e11991) - (assign12910_e11988 * (var_nl_dn6 + var_nstar_dn6))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((var_noiaeff_dn7 * assign12910_e11995) + (var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((var_n0_dn7 + var_nstar_dn7) * assign12910_e11991) - (assign12910_e11988 * (var_nl_dn7 + var_nstar_dn7))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((var_noiaeff_dn8 * assign12910_e11995) + (var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((var_n0_dn8 + var_nstar_dn8) * assign12910_e11991) - (assign12910_e11988 * (var_nl_dn8 + var_nstar_dn8))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8,)
    }
};
        var_t3 = assign12910_e11998;
        var_t3_dn3 = assign12910_e11998_d_n3;
        var_t3_dn4 = assign12910_e11998_d_n4;
        var_t3_dn5 = assign12910_e11998_d_n5;
        var_t3_dn6 = assign12910_e11998_d_n6;
        var_t3_dn7 = assign12910_e11998_d_n7;
        var_t3_dn8 = assign12910_e11998_d_n8;

        let (assign12920_e12006, assign12920_e12006_d_n3, assign12920_e12006_d_n4, assign12920_e12006_d_n5, assign12920_e12006_d_n6, assign12920_e12006_d_n7, assign12920_e12006_d_n8,) = {
    if (var_guard131 != 0.0) {
        let assign12920_e12003: f64 = (var_n0 - var_nl);
        let assign12920_e12004: f64 = (p.p289 * assign12920_e12003);
        (assign12920_e12004, (p.p289 * (var_n0_dn3 - var_nl_dn3)), (p.p289 * (var_n0_dn4 - var_nl_dn4)), (p.p289 * (var_n0_dn5 - var_nl_dn5)), (p.p289 * (var_n0_dn6 - var_nl_dn6)), (p.p289 * (var_n0_dn7 - var_nl_dn7)), (p.p289 * (var_n0_dn8 - var_nl_dn8)),)
    } else {
        (var_t4, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8,)
    }
};
        var_t4 = assign12920_e12006;
        var_t4_dn3 = assign12920_e12006_d_n3;
        var_t4_dn4 = assign12920_e12006_d_n4;
        var_t4_dn5 = assign12920_e12006_d_n5;
        var_t4_dn6 = assign12920_e12006_d_n6;
        var_t4_dn7 = assign12920_e12006_d_n7;
        var_t4_dn8 = assign12920_e12006_d_n8;

        let (assign12930_e12020, assign12930_e12020_d_n3, assign12930_e12020_d_n4, assign12930_e12020_d_n5, assign12930_e12020_d_n6, assign12930_e12020_d_n7, assign12930_e12020_d_n8,) = {
    if (var_guard131 != 0.0) {
        let assign12930_e12010: f64 = (0.5 * p.p290);
        let assign12930_e12013: f64 = (var_n0 * var_n0);
        let assign12930_e12016: f64 = (var_nl * var_nl);
        let assign12930_e12017: f64 = (assign12930_e12013 - assign12930_e12016);
        let assign12930_e12018: f64 = (assign12930_e12010 * assign12930_e12017);
        (assign12930_e12018, (assign12930_e12010 * (((var_n0_dn3 * var_n0) + (var_n0 * var_n0_dn3)) - ((var_nl_dn3 * var_nl) + (var_nl * var_nl_dn3)))), (assign12930_e12010 * (((var_n0_dn4 * var_n0) + (var_n0 * var_n0_dn4)) - ((var_nl_dn4 * var_nl) + (var_nl * var_nl_dn4)))), (assign12930_e12010 * (((var_n0_dn5 * var_n0) + (var_n0 * var_n0_dn5)) - ((var_nl_dn5 * var_nl) + (var_nl * var_nl_dn5)))), (assign12930_e12010 * (((var_n0_dn6 * var_n0) + (var_n0 * var_n0_dn6)) - ((var_nl_dn6 * var_nl) + (var_nl * var_nl_dn6)))), (assign12930_e12010 * (((var_n0_dn7 * var_n0) + (var_n0 * var_n0_dn7)) - ((var_nl_dn7 * var_nl) + (var_nl * var_nl_dn7)))), (assign12930_e12010 * (((var_n0_dn8 * var_n0) + (var_n0 * var_n0_dn8)) - ((var_nl_dn8 * var_nl) + (var_nl * var_nl_dn8)))),)
    } else {
        (var_t5, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8,)
    }
};
        var_t5 = assign12930_e12020;
        var_t5_dn3 = assign12930_e12020_d_n3;
        var_t5_dn4 = assign12930_e12020_d_n4;
        var_t5_dn5 = assign12930_e12020_d_n5;
        var_t5_dn6 = assign12930_e12020_d_n6;
        var_t5_dn7 = assign12930_e12020_d_n7;
        var_t5_dn8 = assign12930_e12020_d_n8;

        let (assign12940_e12030, assign12940_e12030_d_n3, assign12940_e12030_d_n4, assign12940_e12030_d_n5, assign12940_e12030_d_n6, assign12940_e12030_d_n7, assign12940_e12030_d_n8,) = {
    if (var_guard131 != 0.0) {
        let assign12940_e12024: f64 = (1.60219e-19 * var_vtm);
        let assign12940_e12026: f64 = (assign12940_e12024 * var_ids);
        let assign12940_e12028: f64 = (assign12940_e12026 * var_ids);
        (assign12940_e12028, (((assign12940_e12024 * var_ids_dn3) * var_ids) + (assign12940_e12026 * var_ids_dn3)), (((((1.60219e-19 * var_vtm_dn4) * var_ids) + (assign12940_e12024 * var_ids_dn4)) * var_ids) + (assign12940_e12026 * var_ids_dn4)), (((assign12940_e12024 * var_ids_dn5) * var_ids) + (assign12940_e12026 * var_ids_dn5)), (((assign12940_e12024 * var_ids_dn6) * var_ids) + (assign12940_e12026 * var_ids_dn6)), (((assign12940_e12024 * var_ids_dn7) * var_ids) + (assign12940_e12026 * var_ids_dn7)), (((assign12940_e12024 * var_ids_dn8) * var_ids) + (assign12940_e12026 * var_ids_dn8)),)
    } else {
        (var_t6, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8,)
    }
};
        var_t6 = assign12940_e12030;
        var_t6_dn3 = assign12940_e12030_d_n3;
        var_t6_dn4 = assign12940_e12030_d_n4;
        var_t6_dn5 = assign12940_e12030_d_n5;
        var_t6_dn6 = assign12940_e12030_d_n6;
        var_t6_dn7 = assign12940_e12030_d_n7;
        var_t6_dn8 = assign12940_e12030_d_n8;

        let (assign12950_e12040, assign12950_e12040_d_n3, assign12950_e12040_d_n4, assign12950_e12040_d_n5, assign12950_e12040_d_n6, assign12950_e12040_d_n7, assign12950_e12040_d_n8,) = {
    if (var_guard131 != 0.0) {
        let assign12950_e12034: f64 = (10000000000.0 * var_leffnoisq);
        let assign12950_e12036: f64 = (assign12950_e12034 * var_weff);
        let assign12950_e12038: f64 = (assign12950_e12036 * p.p2);
        (assign12950_e12038, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t7, var_t7_dn3, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8,)
    }
};
        var_t7 = assign12950_e12040;
        var_t7_dn3 = assign12950_e12040_d_n3;
        var_t7_dn4 = assign12950_e12040_d_n4;
        var_t7_dn5 = assign12950_e12040_d_n5;
        var_t7_dn6 = assign12950_e12040_d_n6;
        var_t7_dn7 = assign12950_e12040_d_n7;
        var_t7_dn8 = assign12950_e12040_d_n8;

        let (assign12960_e12054, assign12960_e12054_d_n3, assign12960_e12054_d_n4, assign12960_e12054_d_n5, assign12960_e12054_d_n6, assign12960_e12054_d_n7, assign12960_e12054_d_n8,) = {
    if (var_guard131 != 0.0) {
        let assign12960_e12045: f64 = (p.p289 * var_nl);
        let assign12960_e12046: f64 = (var_noiaeff + assign12960_e12045);
        let assign12960_e12049: f64 = (p.p290 * var_nl);
        let assign12960_e12051: f64 = (assign12960_e12049 * var_nl);
        let assign12960_e12052: f64 = (assign12960_e12046 + assign12960_e12051);
        (assign12960_e12052, ((var_noiaeff_dn3 + (p.p289 * var_nl_dn3)) + (((p.p290 * var_nl_dn3) * var_nl) + (assign12960_e12049 * var_nl_dn3))), ((var_noiaeff_dn4 + (p.p289 * var_nl_dn4)) + (((p.p290 * var_nl_dn4) * var_nl) + (assign12960_e12049 * var_nl_dn4))), ((var_noiaeff_dn5 + (p.p289 * var_nl_dn5)) + (((p.p290 * var_nl_dn5) * var_nl) + (assign12960_e12049 * var_nl_dn5))), ((var_noiaeff_dn6 + (p.p289 * var_nl_dn6)) + (((p.p290 * var_nl_dn6) * var_nl) + (assign12960_e12049 * var_nl_dn6))), ((var_noiaeff_dn7 + (p.p289 * var_nl_dn7)) + (((p.p290 * var_nl_dn7) * var_nl) + (assign12960_e12049 * var_nl_dn7))), ((var_noiaeff_dn8 + (p.p289 * var_nl_dn8)) + (((p.p290 * var_nl_dn8) * var_nl) + (assign12960_e12049 * var_nl_dn8))),)
    } else {
        (var_t8, var_t8_dn3, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn7, var_t8_dn8,)
    }
};
        var_t8 = assign12960_e12054;
        var_t8_dn3 = assign12960_e12054_d_n3;
        var_t8_dn4 = assign12960_e12054_d_n4;
        var_t8_dn5 = assign12960_e12054_d_n5;
        var_t8_dn6 = assign12960_e12054_d_n6;
        var_t8_dn7 = assign12960_e12054_d_n7;
        var_t8_dn8 = assign12960_e12054_d_n8;

        let (assign12970_e12064, assign12970_e12064_d_n3, assign12970_e12064_d_n4, assign12970_e12064_d_n5, assign12970_e12064_d_n6, assign12970_e12064_d_n7, assign12970_e12064_d_n8,) = {
    if (var_guard131 != 0.0) {
        let assign12970_e12058: f64 = (var_nl + var_nstar);
        let assign12970_e12061: f64 = (var_nl + var_nstar);
        let assign12970_e12062: f64 = (assign12970_e12058 * assign12970_e12061);
        (assign12970_e12062, (((var_nl_dn3 + var_nstar_dn3) * assign12970_e12061) + (assign12970_e12058 * (var_nl_dn3 + var_nstar_dn3))), (((var_nl_dn4 + var_nstar_dn4) * assign12970_e12061) + (assign12970_e12058 * (var_nl_dn4 + var_nstar_dn4))), (((var_nl_dn5 + var_nstar_dn5) * assign12970_e12061) + (assign12970_e12058 * (var_nl_dn5 + var_nstar_dn5))), (((var_nl_dn6 + var_nstar_dn6) * assign12970_e12061) + (assign12970_e12058 * (var_nl_dn6 + var_nstar_dn6))), (((var_nl_dn7 + var_nstar_dn7) * assign12970_e12061) + (assign12970_e12058 * (var_nl_dn7 + var_nstar_dn7))), (((var_nl_dn8 + var_nstar_dn8) * assign12970_e12061) + (assign12970_e12058 * (var_nl_dn8 + var_nstar_dn8))),)
    } else {
        (var_t9, var_t9_dn3, var_t9_dn4, var_t9_dn5, var_t9_dn6, var_t9_dn7, var_t9_dn8,)
    }
};
        var_t9 = assign12970_e12064;
        var_t9_dn3 = assign12970_e12064_d_n3;
        var_t9_dn4 = assign12970_e12064_d_n4;
        var_t9_dn5 = assign12970_e12064_d_n5;
        var_t9_dn6 = assign12970_e12064_d_n6;
        var_t9_dn7 = assign12970_e12064_d_n7;
        var_t9_dn8 = assign12970_e12064_d_n8;

        let (assign12980_e12086, assign12980_e12086_d_n3, assign12980_e12086_d_n4, assign12980_e12086_d_n5, assign12980_e12086_d_n6, assign12980_e12086_d_n7, assign12980_e12086_d_n8,) = {
    if (var_guard131 != 0.0) {
        let assign12980_e12068: f64 = (var_t1 / var_t2);
        let assign12980_e12071: f64 = (var_t3 + var_t4);
        let assign12980_e12073: f64 = (assign12980_e12071 + var_t5);
        let assign12980_e12074: f64 = (assign12980_e12068 * assign12980_e12073);
        let assign12980_e12077: f64 = (var_t6 / var_t7);
        let assign12980_e12079: f64 = (assign12980_e12077 * var_delclm);
        let assign12980_e12081: f64 = (assign12980_e12079 * var_t8);
        let assign12980_e12083: f64 = (assign12980_e12081 / var_t9);
        let assign12980_e12084: f64 = (assign12980_e12074 + assign12980_e12083);
        (assign12980_e12084, ((((((var_t1_dn3 * var_t2) - (var_t1 * var_t2_dn3)) / (var_t2 * var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((var_t3_dn3 + var_t4_dn3) + var_t5_dn3))) + ((((((((((var_t6_dn3 * var_t7) - (var_t6 * var_t7_dn3)) / (var_t7 * var_t7)) * var_delclm) + (assign12980_e12077 * var_delclm_dn3)) * var_t8) + (assign12980_e12079 * var_t8_dn3)) * var_t9) - (assign12980_e12081 * var_t9_dn3)) / (var_t9 * var_t9))), ((((((var_t1_dn4 * var_t2) - (var_t1 * var_t2_dn4)) / (var_t2 * var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((var_t3_dn4 + var_t4_dn4) + var_t5_dn4))) + ((((((((((var_t6_dn4 * var_t7) - (var_t6 * var_t7_dn4)) / (var_t7 * var_t7)) * var_delclm) + (assign12980_e12077 * var_delclm_dn4)) * var_t8) + (assign12980_e12079 * var_t8_dn4)) * var_t9) - (assign12980_e12081 * var_t9_dn4)) / (var_t9 * var_t9))), ((((((var_t1_dn5 * var_t2) - (var_t1 * var_t2_dn5)) / (var_t2 * var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((var_t3_dn5 + var_t4_dn5) + var_t5_dn5))) + ((((((((((var_t6_dn5 * var_t7) - (var_t6 * var_t7_dn5)) / (var_t7 * var_t7)) * var_delclm) + (assign12980_e12077 * var_delclm_dn5)) * var_t8) + (assign12980_e12079 * var_t8_dn5)) * var_t9) - (assign12980_e12081 * var_t9_dn5)) / (var_t9 * var_t9))), ((((((var_t1_dn6 * var_t2) - (var_t1 * var_t2_dn6)) / (var_t2 * var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((var_t3_dn6 + var_t4_dn6) + var_t5_dn6))) + ((((((((((var_t6_dn6 * var_t7) - (var_t6 * var_t7_dn6)) / (var_t7 * var_t7)) * var_delclm) + (assign12980_e12077 * var_delclm_dn6)) * var_t8) + (assign12980_e12079 * var_t8_dn6)) * var_t9) - (assign12980_e12081 * var_t9_dn6)) / (var_t9 * var_t9))), ((((((var_t1_dn7 * var_t2) - (var_t1 * var_t2_dn7)) / (var_t2 * var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((var_t3_dn7 + var_t4_dn7) + var_t5_dn7))) + ((((((((((var_t6_dn7 * var_t7) - (var_t6 * var_t7_dn7)) / (var_t7 * var_t7)) * var_delclm) + (assign12980_e12077 * var_delclm_dn7)) * var_t8) + (assign12980_e12079 * var_t8_dn7)) * var_t9) - (assign12980_e12081 * var_t9_dn7)) / (var_t9 * var_t9))), ((((((var_t1_dn8 * var_t2) - (var_t1 * var_t2_dn8)) / (var_t2 * var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((var_t3_dn8 + var_t4_dn8) + var_t5_dn8))) + ((((((((((var_t6_dn8 * var_t7) - (var_t6 * var_t7_dn8)) / (var_t7 * var_t7)) * var_delclm) + (assign12980_e12077 * var_delclm_dn8)) * var_t8) + (assign12980_e12079 * var_t8_dn8)) * var_t9) - (assign12980_e12081 * var_t9_dn8)) / (var_t9 * var_t9))),)
    } else {
        (var_ssi, var_ssi_dn3, var_ssi_dn4, var_ssi_dn5, var_ssi_dn6, var_ssi_dn7, var_ssi_dn8,)
    }
};
        var_ssi = assign12980_e12086;
        var_ssi_dn3 = assign12980_e12086_d_n3;
        var_ssi_dn4 = assign12980_e12086_d_n4;
        var_ssi_dn5 = assign12980_e12086_d_n5;
        var_ssi_dn6 = assign12980_e12086_d_n6;
        var_ssi_dn7 = assign12980_e12086_d_n7;
        var_ssi_dn8 = assign12980_e12086_d_n8;

        let (assign12990_e12094, assign12990_e12094_d_n3, assign12990_e12094_d_n4, assign12990_e12094_d_n5, assign12990_e12094_d_n6, assign12990_e12094_d_n7, assign12990_e12094_d_n8,) = {
    if (var_guard131 != 0.0) {
        let assign12990_e12090: f64 = (var_noiaeff * 1.60219e-19);
        let assign12990_e12092: f64 = (assign12990_e12090 * var_vtm);
        (assign12990_e12092, ((var_noiaeff_dn3 * 1.60219e-19) * var_vtm), (((var_noiaeff_dn4 * 1.60219e-19) * var_vtm) + (assign12990_e12090 * var_vtm_dn4)), ((var_noiaeff_dn5 * 1.60219e-19) * var_vtm), ((var_noiaeff_dn6 * 1.60219e-19) * var_vtm), ((var_noiaeff_dn7 * 1.60219e-19) * var_vtm), ((var_noiaeff_dn8 * 1.60219e-19) * var_vtm),)
    } else {
        (var_t10, var_t10_dn3, var_t10_dn4, var_t10_dn5, var_t10_dn6, var_t10_dn7, var_t10_dn8,)
    }
};
        var_t10 = assign12990_e12094;
        var_t10_dn3 = assign12990_e12094_d_n3;
        var_t10_dn4 = assign12990_e12094_d_n4;
        var_t10_dn5 = assign12990_e12094_d_n5;
        var_t10_dn6 = assign12990_e12094_d_n6;
        var_t10_dn7 = assign12990_e12094_d_n7;
        var_t10_dn8 = assign12990_e12094_d_n8;

        let (assign13000_e12108, assign13000_e12108_d_n3, assign13000_e12108_d_n4, assign13000_e12108_d_n5, assign13000_e12108_d_n6, assign13000_e12108_d_n7, assign13000_e12108_d_n8,) = {
    if (var_guard131 != 0.0) {
        let assign13000_e12098: f64 = (var_weff * p.p2);
        let assign13000_e12100: f64 = (assign13000_e12098 * var_leffnoi);
        let assign13000_e12102: f64 = (assign13000_e12100 * 10000000000.0);
        let assign13000_e12104: f64 = (assign13000_e12102 * var_nstar);
        let assign13000_e12106: f64 = (assign13000_e12104 * var_nstar);
        (assign13000_e12106, (((assign13000_e12102 * var_nstar_dn3) * var_nstar) + (assign13000_e12104 * var_nstar_dn3)), (((assign13000_e12102 * var_nstar_dn4) * var_nstar) + (assign13000_e12104 * var_nstar_dn4)), (((assign13000_e12102 * var_nstar_dn5) * var_nstar) + (assign13000_e12104 * var_nstar_dn5)), (((assign13000_e12102 * var_nstar_dn6) * var_nstar) + (assign13000_e12104 * var_nstar_dn6)), (((assign13000_e12102 * var_nstar_dn7) * var_nstar) + (assign13000_e12104 * var_nstar_dn7)), (((assign13000_e12102 * var_nstar_dn8) * var_nstar) + (assign13000_e12104 * var_nstar_dn8)),)
    } else {
        (var_t11, var_t11_dn3, var_t11_dn4, var_t11_dn5, var_t11_dn6, var_t11_dn7, var_t11_dn8,)
    }
};
        var_t11 = assign13000_e12108;
        var_t11_dn3 = assign13000_e12108_d_n3;
        var_t11_dn4 = assign13000_e12108_d_n4;
        var_t11_dn5 = assign13000_e12108_d_n5;
        var_t11_dn6 = assign13000_e12108_d_n6;
        var_t11_dn7 = assign13000_e12108_d_n7;
        var_t11_dn8 = assign13000_e12108_d_n8;

        let (assign13010_e12118, assign13010_e12118_d_n3, assign13010_e12118_d_n4, assign13010_e12118_d_n5, assign13010_e12118_d_n6, assign13010_e12118_d_n7, assign13010_e12118_d_n8,) = {
    if (var_guard131 != 0.0) {
        let assign13010_e12112: f64 = (var_t10 / var_t11);
        let assign13010_e12114: f64 = (assign13010_e12112 * var_ids);
        let assign13010_e12116: f64 = (assign13010_e12114 * var_ids);
        (assign13010_e12116, (((((((var_t10_dn3 * var_t11) - (var_t10 * var_t11_dn3)) / (var_t11 * var_t11)) * var_ids) + (assign13010_e12112 * var_ids_dn3)) * var_ids) + (assign13010_e12114 * var_ids_dn3)), (((((((var_t10_dn4 * var_t11) - (var_t10 * var_t11_dn4)) / (var_t11 * var_t11)) * var_ids) + (assign13010_e12112 * var_ids_dn4)) * var_ids) + (assign13010_e12114 * var_ids_dn4)), (((((((var_t10_dn5 * var_t11) - (var_t10 * var_t11_dn5)) / (var_t11 * var_t11)) * var_ids) + (assign13010_e12112 * var_ids_dn5)) * var_ids) + (assign13010_e12114 * var_ids_dn5)), (((((((var_t10_dn6 * var_t11) - (var_t10 * var_t11_dn6)) / (var_t11 * var_t11)) * var_ids) + (assign13010_e12112 * var_ids_dn6)) * var_ids) + (assign13010_e12114 * var_ids_dn6)), (((((((var_t10_dn7 * var_t11) - (var_t10 * var_t11_dn7)) / (var_t11 * var_t11)) * var_ids) + (assign13010_e12112 * var_ids_dn7)) * var_ids) + (assign13010_e12114 * var_ids_dn7)), (((((((var_t10_dn8 * var_t11) - (var_t10 * var_t11_dn8)) / (var_t11 * var_t11)) * var_ids) + (assign13010_e12112 * var_ids_dn8)) * var_ids) + (assign13010_e12114 * var_ids_dn8)),)
    } else {
        (var_swi, var_swi_dn3, var_swi_dn4, var_swi_dn5, var_swi_dn6, var_swi_dn7, var_swi_dn8,)
    }
};
        var_swi = assign13010_e12118;
        var_swi_dn3 = assign13010_e12118_d_n3;
        var_swi_dn4 = assign13010_e12118_d_n4;
        var_swi_dn5 = assign13010_e12118_d_n5;
        var_swi_dn6 = assign13010_e12118_d_n6;
        var_swi_dn7 = assign13010_e12118_d_n7;
        var_swi_dn8 = assign13010_e12118_d_n8;

        let (assign13020_e12124, assign13020_e12124_d_n3, assign13020_e12124_d_n4, assign13020_e12124_d_n5, assign13020_e12124_d_n6, assign13020_e12124_d_n7, assign13020_e12124_d_n8,) = {
    if (var_guard131 != 0.0) {
        let assign13020_e12122: f64 = (var_swi + var_ssi);
        (assign13020_e12122, (var_swi_dn3 + var_ssi_dn3), (var_swi_dn4 + var_ssi_dn4), (var_swi_dn5 + var_ssi_dn5), (var_swi_dn6 + var_ssi_dn6), (var_swi_dn7 + var_ssi_dn7), (var_swi_dn8 + var_ssi_dn8),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8,)
    }
};
        var_t1 = assign13020_e12124;
        var_t1_dn3 = assign13020_e12124_d_n3;
        var_t1_dn4 = assign13020_e12124_d_n4;
        var_t1_dn5 = assign13020_e12124_d_n5;
        var_t1_dn6 = assign13020_e12124_d_n6;
        var_t1_dn7 = assign13020_e12124_d_n7;
        var_t1_dn8 = assign13020_e12124_d_n8;

        let assign13070_e12152: f64 = (var_devsign * p.p2);
        let assign13070_e12154: f64 = (assign13070_e12152 * var_qfg);
        var_qfgi = assign13070_e12154;
        var_qfgi_dn3 = (assign13070_e12152 * var_qfg_dn3);
        var_qfgi_dn4 = (assign13070_e12152 * var_qfg_dn4);
        var_qfgi_dn5 = (assign13070_e12152 * var_qfg_dn5);
        var_qfgi_dn6 = (assign13070_e12152 * var_qfg_dn6);
        var_qfgi_dn7 = (assign13070_e12152 * var_qfg_dn7);
        var_qfgi_dn8 = (assign13070_e12152 * var_qfg_dn8);

        let assign13080_e12157: f64 = (p.p2 * var_qbg);
        var_qbgi = assign13080_e12157;
        var_qbgi_dn3 = (p.p2 * var_qbg_dn3);
        var_qbgi_dn4 = (p.p2 * var_qbg_dn4);
        var_qbgi_dn5 = (p.p2 * var_qbg_dn5);
        var_qbgi_dn6 = (p.p2 * var_qbg_dn6);
        var_qbgi_dn7 = (p.p2 * var_qbg_dn7);
        var_qbgi_dn8 = (p.p2 * var_qbg_dn8);

        let assign13090_e12160: f64 = if var_sigvds > 0.0 { 1.0 } else { 0.0 };
        var_guard136 = assign13090_e12160;

        let (assign13100_e12166, assign13100_e12166_d_n3, assign13100_e12166_d_n4, assign13100_e12166_d_n5, assign13100_e12166_d_n6, assign13100_e12166_d_n7, assign13100_e12166_d_n8,) = {
    if (var_guard136 != 0.0) {
        let assign13100_e12164: f64 = (p.p2 * var_qs);
        (assign13100_e12164, (p.p2 * var_qs_dn3), (p.p2 * var_qs_dn4), (p.p2 * var_qs_dn5), (p.p2 * var_qs_dn6), (p.p2 * var_qs_dn7), (p.p2 * var_qs_dn8),)
    } else {
        (var_qsi, var_qsi_dn3, var_qsi_dn4, var_qsi_dn5, var_qsi_dn6, var_qsi_dn7, var_qsi_dn8,)
    }
};
        var_qsi = assign13100_e12166;
        var_qsi_dn3 = assign13100_e12166_d_n3;
        var_qsi_dn4 = assign13100_e12166_d_n4;
        var_qsi_dn5 = assign13100_e12166_d_n5;
        var_qsi_dn6 = assign13100_e12166_d_n6;
        var_qsi_dn7 = assign13100_e12166_d_n7;
        var_qsi_dn8 = assign13100_e12166_d_n8;

        let (assign13110_e12172, assign13110_e12172_d_n3, assign13110_e12172_d_n4, assign13110_e12172_d_n5, assign13110_e12172_d_n6, assign13110_e12172_d_n7, assign13110_e12172_d_n8,) = {
    if (var_guard136 != 0.0) {
        let assign13110_e12170: f64 = (p.p2 * var_qd);
        (assign13110_e12170, (p.p2 * var_qd_dn3), (p.p2 * var_qd_dn4), (p.p2 * var_qd_dn5), (p.p2 * var_qd_dn6), (p.p2 * var_qd_dn7), (p.p2 * var_qd_dn8),)
    } else {
        (var_qdi, var_qdi_dn3, var_qdi_dn4, var_qdi_dn5, var_qdi_dn6, var_qdi_dn7, var_qdi_dn8,)
    }
};
        var_qdi = assign13110_e12172;
        var_qdi_dn3 = assign13110_e12172_d_n3;
        var_qdi_dn4 = assign13110_e12172_d_n4;
        var_qdi_dn5 = assign13110_e12172_d_n5;
        var_qdi_dn6 = assign13110_e12172_d_n6;
        var_qdi_dn7 = assign13110_e12172_d_n7;
        var_qdi_dn8 = assign13110_e12172_d_n8;

        let (assign13120_e12182, assign13120_e12182_d_n3, assign13120_e12182_d_n4, assign13120_e12182_d_n5, assign13120_e12182_d_n6, assign13120_e12182_d_n7, assign13120_e12182_d_n8,) = {
    if (var_guard136 != 0.0) {
        let assign13120_e12177: f64 = (var_qs - var_qfgs_parasitic);
        let assign13120_e12178: f64 = (p.p2 * assign13120_e12177);
        let assign13120_e12180: f64 = (assign13120_e12178 + var_qsbg);
        (assign13120_e12180, ((p.p2 * (var_qs_dn3 - var_qfgs_parasitic_dn3)) + var_qsbg_dn3), ((p.p2 * (var_qs_dn4 - var_qfgs_parasitic_dn4)) + var_qsbg_dn4), ((p.p2 * (var_qs_dn5 - var_qfgs_parasitic_dn5)) + var_qsbg_dn5), ((p.p2 * (var_qs_dn6 - var_qfgs_parasitic_dn6)) + var_qsbg_dn6), ((p.p2 * (var_qs_dn7 - var_qfgs_parasitic_dn7)) + var_qsbg_dn7), ((p.p2 * (var_qs_dn8 - var_qfgs_parasitic_dn8)) + var_qsbg_dn8),)
    } else {
        (var_qs, var_qs_dn3, var_qs_dn4, var_qs_dn5, var_qs_dn6, var_qs_dn7, var_qs_dn8,)
    }
};
        var_qs = assign13120_e12182;
        var_qs_dn3 = assign13120_e12182_d_n3;
        var_qs_dn4 = assign13120_e12182_d_n4;
        var_qs_dn5 = assign13120_e12182_d_n5;
        var_qs_dn6 = assign13120_e12182_d_n6;
        var_qs_dn7 = assign13120_e12182_d_n7;
        var_qs_dn8 = assign13120_e12182_d_n8;

        let (assign13130_e12192, assign13130_e12192_d_n3, assign13130_e12192_d_n4, assign13130_e12192_d_n5, assign13130_e12192_d_n6, assign13130_e12192_d_n7, assign13130_e12192_d_n8,) = {
    if (var_guard136 != 0.0) {
        let assign13130_e12187: f64 = (var_qd - var_qfgd_parasitic);
        let assign13130_e12188: f64 = (p.p2 * assign13130_e12187);
        let assign13130_e12190: f64 = (assign13130_e12188 + var_qdbg);
        (assign13130_e12190, ((p.p2 * (var_qd_dn3 - var_qfgd_parasitic_dn3)) + var_qdbg_dn3), ((p.p2 * (var_qd_dn4 - var_qfgd_parasitic_dn4)) + var_qdbg_dn4), ((p.p2 * (var_qd_dn5 - var_qfgd_parasitic_dn5)) + var_qdbg_dn5), ((p.p2 * (var_qd_dn6 - var_qfgd_parasitic_dn6)) + var_qdbg_dn6), ((p.p2 * (var_qd_dn7 - var_qfgd_parasitic_dn7)) + var_qdbg_dn7), ((p.p2 * (var_qd_dn8 - var_qfgd_parasitic_dn8)) + var_qdbg_dn8),)
    } else {
        (var_qd, var_qd_dn3, var_qd_dn4, var_qd_dn5, var_qd_dn6, var_qd_dn7, var_qd_dn8,)
    }
};
        var_qd = assign13130_e12192;
        var_qd_dn3 = assign13130_e12192_d_n3;
        var_qd_dn4 = assign13130_e12192_d_n4;
        var_qd_dn5 = assign13130_e12192_d_n5;
        var_qd_dn6 = assign13130_e12192_d_n6;
        var_qd_dn7 = assign13130_e12192_d_n7;
        var_qd_dn8 = assign13130_e12192_d_n8;

        let (assign13140_e12199, assign13140_e12199_d_n3, assign13140_e12199_d_n4, assign13140_e12199_d_n5, assign13140_e12199_d_n6, assign13140_e12199_d_n7, assign13140_e12199_d_n8,) = {
    if (var_guard136 == 0.0) {
        let assign13140_e12197: f64 = (p.p2 * var_qd);
        (assign13140_e12197, (p.p2 * var_qd_dn3), (p.p2 * var_qd_dn4), (p.p2 * var_qd_dn5), (p.p2 * var_qd_dn6), (p.p2 * var_qd_dn7), (p.p2 * var_qd_dn8),)
    } else {
        (var_qsi, var_qsi_dn3, var_qsi_dn4, var_qsi_dn5, var_qsi_dn6, var_qsi_dn7, var_qsi_dn8,)
    }
};
        var_qsi = assign13140_e12199;
        var_qsi_dn3 = assign13140_e12199_d_n3;
        var_qsi_dn4 = assign13140_e12199_d_n4;
        var_qsi_dn5 = assign13140_e12199_d_n5;
        var_qsi_dn6 = assign13140_e12199_d_n6;
        var_qsi_dn7 = assign13140_e12199_d_n7;
        var_qsi_dn8 = assign13140_e12199_d_n8;

        let (assign13150_e12206, assign13150_e12206_d_n3, assign13150_e12206_d_n4, assign13150_e12206_d_n5, assign13150_e12206_d_n6, assign13150_e12206_d_n7, assign13150_e12206_d_n8,) = {
    if (var_guard136 == 0.0) {
        let assign13150_e12204: f64 = (p.p2 * var_qs);
        (assign13150_e12204, (p.p2 * var_qs_dn3), (p.p2 * var_qs_dn4), (p.p2 * var_qs_dn5), (p.p2 * var_qs_dn6), (p.p2 * var_qs_dn7), (p.p2 * var_qs_dn8),)
    } else {
        (var_qdi, var_qdi_dn3, var_qdi_dn4, var_qdi_dn5, var_qdi_dn6, var_qdi_dn7, var_qdi_dn8,)
    }
};
        var_qdi = assign13150_e12206;
        var_qdi_dn3 = assign13150_e12206_d_n3;
        var_qdi_dn4 = assign13150_e12206_d_n4;
        var_qdi_dn5 = assign13150_e12206_d_n5;
        var_qdi_dn6 = assign13150_e12206_d_n6;
        var_qdi_dn7 = assign13150_e12206_d_n7;
        var_qdi_dn8 = assign13150_e12206_d_n8;

        let (assign13160_e12217, assign13160_e12217_d_n3, assign13160_e12217_d_n4, assign13160_e12217_d_n5, assign13160_e12217_d_n6, assign13160_e12217_d_n7, assign13160_e12217_d_n8,) = {
    if (var_guard136 == 0.0) {
        let assign13160_e12212: f64 = (var_qd - var_qfgs_parasitic);
        let assign13160_e12213: f64 = (p.p2 * assign13160_e12212);
        let assign13160_e12215: f64 = (assign13160_e12213 + var_qsbg);
        (assign13160_e12215, ((p.p2 * (var_qd_dn3 - var_qfgs_parasitic_dn3)) + var_qsbg_dn3), ((p.p2 * (var_qd_dn4 - var_qfgs_parasitic_dn4)) + var_qsbg_dn4), ((p.p2 * (var_qd_dn5 - var_qfgs_parasitic_dn5)) + var_qsbg_dn5), ((p.p2 * (var_qd_dn6 - var_qfgs_parasitic_dn6)) + var_qsbg_dn6), ((p.p2 * (var_qd_dn7 - var_qfgs_parasitic_dn7)) + var_qsbg_dn7), ((p.p2 * (var_qd_dn8 - var_qfgs_parasitic_dn8)) + var_qsbg_dn8),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign13160_e12217;
        var_t0_dn3 = assign13160_e12217_d_n3;
        var_t0_dn4 = assign13160_e12217_d_n4;
        var_t0_dn5 = assign13160_e12217_d_n5;
        var_t0_dn6 = assign13160_e12217_d_n6;
        var_t0_dn7 = assign13160_e12217_d_n7;
        var_t0_dn8 = assign13160_e12217_d_n8;

        let (assign13170_e12228, assign13170_e12228_d_n3, assign13170_e12228_d_n4, assign13170_e12228_d_n5, assign13170_e12228_d_n6, assign13170_e12228_d_n7, assign13170_e12228_d_n8,) = {
    if (var_guard136 == 0.0) {
        let assign13170_e12223: f64 = (var_qs - var_qfgd_parasitic);
        let assign13170_e12224: f64 = (p.p2 * assign13170_e12223);
        let assign13170_e12226: f64 = (assign13170_e12224 + var_qdbg);
        (assign13170_e12226, ((p.p2 * (var_qs_dn3 - var_qfgd_parasitic_dn3)) + var_qdbg_dn3), ((p.p2 * (var_qs_dn4 - var_qfgd_parasitic_dn4)) + var_qdbg_dn4), ((p.p2 * (var_qs_dn5 - var_qfgd_parasitic_dn5)) + var_qdbg_dn5), ((p.p2 * (var_qs_dn6 - var_qfgd_parasitic_dn6)) + var_qdbg_dn6), ((p.p2 * (var_qs_dn7 - var_qfgd_parasitic_dn7)) + var_qdbg_dn7), ((p.p2 * (var_qs_dn8 - var_qfgd_parasitic_dn8)) + var_qdbg_dn8),)
    } else {
        (var_qd, var_qd_dn3, var_qd_dn4, var_qd_dn5, var_qd_dn6, var_qd_dn7, var_qd_dn8,)
    }
};
        var_qd = assign13170_e12228;
        var_qd_dn3 = assign13170_e12228_d_n3;
        var_qd_dn4 = assign13170_e12228_d_n4;
        var_qd_dn5 = assign13170_e12228_d_n5;
        var_qd_dn6 = assign13170_e12228_d_n6;
        var_qd_dn7 = assign13170_e12228_d_n7;
        var_qd_dn8 = assign13170_e12228_d_n8;

        let (assign13180_e12233, assign13180_e12233_d_n3, assign13180_e12233_d_n4, assign13180_e12233_d_n5, assign13180_e12233_d_n6, assign13180_e12233_d_n7, assign13180_e12233_d_n8,) = {
    if (var_guard136 == 0.0) {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    } else {
        (var_qs, var_qs_dn3, var_qs_dn4, var_qs_dn5, var_qs_dn6, var_qs_dn7, var_qs_dn8,)
    }
};
        var_qs = assign13180_e12233;
        var_qs_dn3 = assign13180_e12233_d_n3;
        var_qs_dn4 = assign13180_e12233_d_n4;
        var_qs_dn5 = assign13180_e12233_d_n5;
        var_qs_dn6 = assign13180_e12233_d_n6;
        var_qs_dn7 = assign13180_e12233_d_n7;
        var_qs_dn8 = assign13180_e12233_d_n8;

        *var_guard136_slot = var_guard136;
        *var_n0_slot = var_n0;
        *var_n0_dn3_slot = var_n0_dn3;
        *var_n0_dn4_slot = var_n0_dn4;
        *var_n0_dn5_slot = var_n0_dn5;
        *var_n0_dn6_slot = var_n0_dn6;
        *var_n0_dn7_slot = var_n0_dn7;
        *var_n0_dn8_slot = var_n0_dn8;
        *var_nl_slot = var_nl;
        *var_nl_dn3_slot = var_nl_dn3;
        *var_nl_dn4_slot = var_nl_dn4;
        *var_nl_dn5_slot = var_nl_dn5;
        *var_nl_dn6_slot = var_nl_dn6;
        *var_nl_dn7_slot = var_nl_dn7;
        *var_nl_dn8_slot = var_nl_dn8;
        *var_noiaeff_slot = var_noiaeff;
        *var_noiaeff_dn3_slot = var_noiaeff_dn3;
        *var_noiaeff_dn4_slot = var_noiaeff_dn4;
        *var_noiaeff_dn5_slot = var_noiaeff_dn5;
        *var_noiaeff_dn6_slot = var_noiaeff_dn6;
        *var_noiaeff_dn7_slot = var_noiaeff_dn7;
        *var_noiaeff_dn8_slot = var_noiaeff_dn8;
        *var_nstar_slot = var_nstar;
        *var_nstar_dn3_slot = var_nstar_dn3;
        *var_nstar_dn4_slot = var_nstar_dn4;
        *var_nstar_dn5_slot = var_nstar_dn5;
        *var_nstar_dn6_slot = var_nstar_dn6;
        *var_nstar_dn7_slot = var_nstar_dn7;
        *var_nstar_dn8_slot = var_nstar_dn8;
        *var_qbgi_slot = var_qbgi;
        *var_qbgi_dn3_slot = var_qbgi_dn3;
        *var_qbgi_dn4_slot = var_qbgi_dn4;
        *var_qbgi_dn5_slot = var_qbgi_dn5;
        *var_qbgi_dn6_slot = var_qbgi_dn6;
        *var_qbgi_dn7_slot = var_qbgi_dn7;
        *var_qbgi_dn8_slot = var_qbgi_dn8;
        *var_qd_slot = var_qd;
        *var_qd_dn3_slot = var_qd_dn3;
        *var_qd_dn4_slot = var_qd_dn4;
        *var_qd_dn5_slot = var_qd_dn5;
        *var_qd_dn6_slot = var_qd_dn6;
        *var_qd_dn7_slot = var_qd_dn7;
        *var_qd_dn8_slot = var_qd_dn8;
        *var_qdi_slot = var_qdi;
        *var_qdi_dn3_slot = var_qdi_dn3;
        *var_qdi_dn4_slot = var_qdi_dn4;
        *var_qdi_dn5_slot = var_qdi_dn5;
        *var_qdi_dn6_slot = var_qdi_dn6;
        *var_qdi_dn7_slot = var_qdi_dn7;
        *var_qdi_dn8_slot = var_qdi_dn8;
        *var_qfgi_slot = var_qfgi;
        *var_qfgi_dn3_slot = var_qfgi_dn3;
        *var_qfgi_dn4_slot = var_qfgi_dn4;
        *var_qfgi_dn5_slot = var_qfgi_dn5;
        *var_qfgi_dn6_slot = var_qfgi_dn6;
        *var_qfgi_dn7_slot = var_qfgi_dn7;
        *var_qfgi_dn8_slot = var_qfgi_dn8;
        *var_qs_slot = var_qs;
        *var_qs_dn3_slot = var_qs_dn3;
        *var_qs_dn4_slot = var_qs_dn4;
        *var_qs_dn5_slot = var_qs_dn5;
        *var_qs_dn6_slot = var_qs_dn6;
        *var_qs_dn7_slot = var_qs_dn7;
        *var_qs_dn8_slot = var_qs_dn8;
        *var_qsi_slot = var_qsi;
        *var_qsi_dn3_slot = var_qsi_dn3;
        *var_qsi_dn4_slot = var_qsi_dn4;
        *var_qsi_dn5_slot = var_qsi_dn5;
        *var_qsi_dn6_slot = var_qsi_dn6;
        *var_qsi_dn7_slot = var_qsi_dn7;
        *var_qsi_dn8_slot = var_qsi_dn8;
        *var_ssi_slot = var_ssi;
        *var_ssi_dn3_slot = var_ssi_dn3;
        *var_ssi_dn4_slot = var_ssi_dn4;
        *var_ssi_dn5_slot = var_ssi_dn5;
        *var_ssi_dn6_slot = var_ssi_dn6;
        *var_ssi_dn7_slot = var_ssi_dn7;
        *var_ssi_dn8_slot = var_ssi_dn8;
        *var_swi_slot = var_swi;
        *var_swi_dn3_slot = var_swi_dn3;
        *var_swi_dn4_slot = var_swi_dn4;
        *var_swi_dn5_slot = var_swi_dn5;
        *var_swi_dn6_slot = var_swi_dn6;
        *var_swi_dn7_slot = var_swi_dn7;
        *var_swi_dn8_slot = var_swi_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t10_slot = var_t10;
        *var_t10_dn3_slot = var_t10_dn3;
        *var_t10_dn4_slot = var_t10_dn4;
        *var_t10_dn5_slot = var_t10_dn5;
        *var_t10_dn6_slot = var_t10_dn6;
        *var_t10_dn7_slot = var_t10_dn7;
        *var_t10_dn8_slot = var_t10_dn8;
        *var_t11_slot = var_t11;
        *var_t11_dn3_slot = var_t11_dn3;
        *var_t11_dn4_slot = var_t11_dn4;
        *var_t11_dn5_slot = var_t11_dn5;
        *var_t11_dn6_slot = var_t11_dn6;
        *var_t11_dn7_slot = var_t11_dn7;
        *var_t11_dn8_slot = var_t11_dn8;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t2_slot = var_t2;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t3_slot = var_t3;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t4_slot = var_t4;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t5_slot = var_t5;
        *var_t5_dn3_slot = var_t5_dn3;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t6_slot = var_t6;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t7_slot = var_t7;
        *var_t7_dn3_slot = var_t7_dn3;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn7_slot = var_t7_dn7;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t8_slot = var_t8;
        *var_t8_dn3_slot = var_t8_dn3;
        *var_t8_dn4_slot = var_t8_dn4;
        *var_t8_dn5_slot = var_t8_dn5;
        *var_t8_dn6_slot = var_t8_dn6;
        *var_t8_dn7_slot = var_t8_dn7;
        *var_t8_dn8_slot = var_t8_dn8;
        *var_t9_slot = var_t9;
        *var_t9_dn3_slot = var_t9_dn3;
        *var_t9_dn4_slot = var_t9_dn4;
        *var_t9_dn5_slot = var_t9_dn5;
        *var_t9_dn6_slot = var_t9_dn6;
        *var_t9_dn7_slot = var_t9_dn7;
        *var_t9_dn8_slot = var_t9_dn8;
    }

    pub(super) fn stamp_transient_block_30(
        p: &Parameters,
        var_cox1: f64,
        var_leff: f64,
        var_qdbg: f64,
        var_qdbg_dn3: f64,
        var_qdbg_dn4: f64,
        var_qdbg_dn5: f64,
        var_qdbg_dn6: f64,
        var_qdbg_dn7: f64,
        var_qdbg_dn8: f64,
        var_qdi: f64,
        var_qdi_dn3: f64,
        var_qdi_dn4: f64,
        var_qdi_dn5: f64,
        var_qdi_dn6: f64,
        var_qdi_dn7: f64,
        var_qdi_dn8: f64,
        var_qfgi: f64,
        var_qfgi_dn3: f64,
        var_qfgi_dn4: f64,
        var_qfgi_dn5: f64,
        var_qfgi_dn6: f64,
        var_qfgi_dn7: f64,
        var_qfgi_dn8: f64,
        var_qia: f64,
        var_qia_dn3: f64,
        var_qia_dn4: f64,
        var_qia_dn5: f64,
        var_qia_dn6: f64,
        var_qia_dn7: f64,
        var_qia_dn8: f64,
        var_qsbg: f64,
        var_qsbg_dn3: f64,
        var_qsbg_dn4: f64,
        var_qsbg_dn5: f64,
        var_qsbg_dn6: f64,
        var_qsbg_dn7: f64,
        var_qsbg_dn8: f64,
        var_qsi: f64,
        var_qsi_dn3: f64,
        var_qsi_dn4: f64,
        var_qsi_dn5: f64,
        var_qsi_dn6: f64,
        var_qsi_dn7: f64,
        var_qsi_dn8: f64,
        var_rdrain: f64,
        var_rdrain_dn3: f64,
        var_rdrain_dn4: f64,
        var_rdrain_dn5: f64,
        var_rdrain_dn6: f64,
        var_rdrain_dn7: f64,
        var_rdrain_dn8: f64,
        var_rdsi: f64,
        var_rdsi_dn3: f64,
        var_rdsi_dn4: f64,
        var_rdsi_dn5: f64,
        var_rdsi_dn6: f64,
        var_rdsi_dn7: f64,
        var_rdsi_dn8: f64,
        var_rsource: f64,
        var_rsource_dn3: f64,
        var_rsource_dn4: f64,
        var_rsource_dn5: f64,
        var_rsource_dn6: f64,
        var_rsource_dn7: f64,
        var_rsource_dn8: f64,
        var_sigvds: f64,
        var_utotal: f64,
        var_utotal_dn3: f64,
        var_utotal_dn4: f64,
        var_utotal_dn5: f64,
        var_utotal_dn6: f64,
        var_utotal_dn7: f64,
        var_utotal_dn8: f64,
        var_vtm: f64,
        var_vtm_dn4: f64,
        var_weff: f64,
        var_xrcrg1_i: f64,
        var_xrcrg2_i: f64,
        var_gcrg_slot: &mut f64,
        var_gcrg_dn3_slot: &mut f64,
        var_gcrg_dn4_slot: &mut f64,
        var_gcrg_dn5_slot: &mut f64,
        var_gcrg_dn6_slot: &mut f64,
        var_gcrg_dn7_slot: &mut f64,
        var_gcrg_dn8_slot: &mut f64,
        var_gdpr_slot: &mut f64,
        var_gdpr_dn3_slot: &mut f64,
        var_gdpr_dn4_slot: &mut f64,
        var_gdpr_dn5_slot: &mut f64,
        var_gdpr_dn6_slot: &mut f64,
        var_gdpr_dn7_slot: &mut f64,
        var_gdpr_dn8_slot: &mut f64,
        var_gspr_slot: &mut f64,
        var_gspr_dn3_slot: &mut f64,
        var_gspr_dn4_slot: &mut f64,
        var_gspr_dn5_slot: &mut f64,
        var_gspr_dn6_slot: &mut f64,
        var_gspr_dn7_slot: &mut f64,
        var_gspr_dn8_slot: &mut f64,
        var_guard137_slot: &mut f64,
        var_guard138_slot: &mut f64,
        var_guard139_slot: &mut f64,
        var_guard140_slot: &mut f64,
        var_guard147_slot: &mut f64,
        var_guard148_slot: &mut f64,
        var_idsovvds_slot: &mut f64,
        var_idsovvds_dn3_slot: &mut f64,
        var_idsovvds_dn4_slot: &mut f64,
        var_idsovvds_dn5_slot: &mut f64,
        var_idsovvds_dn6_slot: &mut f64,
        var_idsovvds_dn7_slot: &mut f64,
        var_idsovvds_dn8_slot: &mut f64,
        var_igcd_slot: &mut f64,
        var_igcd_dn3_slot: &mut f64,
        var_igcd_dn4_slot: &mut f64,
        var_igcd_dn5_slot: &mut f64,
        var_igcd_dn6_slot: &mut f64,
        var_igcd_dn7_slot: &mut f64,
        var_igcd_dn8_slot: &mut f64,
        var_igcs_slot: &mut f64,
        var_igcs_dn3_slot: &mut f64,
        var_igcs_dn4_slot: &mut f64,
        var_igcs_dn5_slot: &mut f64,
        var_igcs_dn6_slot: &mut f64,
        var_igcs_dn7_slot: &mut f64,
        var_igcs_dn8_slot: &mut f64,
        var_igd_slot: &mut f64,
        var_igd_dn3_slot: &mut f64,
        var_igd_dn4_slot: &mut f64,
        var_igd_dn5_slot: &mut f64,
        var_igd_dn6_slot: &mut f64,
        var_igd_dn7_slot: &mut f64,
        var_igd_dn8_slot: &mut f64,
        var_igidl_slot: &mut f64,
        var_igidl_dn3_slot: &mut f64,
        var_igidl_dn4_slot: &mut f64,
        var_igidl_dn5_slot: &mut f64,
        var_igidl_dn6_slot: &mut f64,
        var_igidl_dn7_slot: &mut f64,
        var_igidl_dn8_slot: &mut f64,
        var_igisl_slot: &mut f64,
        var_igisl_dn3_slot: &mut f64,
        var_igisl_dn4_slot: &mut f64,
        var_igisl_dn5_slot: &mut f64,
        var_igisl_dn6_slot: &mut f64,
        var_igisl_dn7_slot: &mut f64,
        var_igisl_dn8_slot: &mut f64,
        var_igs_slot: &mut f64,
        var_igs_dn3_slot: &mut f64,
        var_igs_dn4_slot: &mut f64,
        var_igs_dn5_slot: &mut f64,
        var_igs_dn6_slot: &mut f64,
        var_igs_dn7_slot: &mut f64,
        var_igs_dn8_slot: &mut f64,
        var_qbg_slot: &mut f64,
        var_qbg_dn3_slot: &mut f64,
        var_qbg_dn4_slot: &mut f64,
        var_qbg_dn5_slot: &mut f64,
        var_qbg_dn6_slot: &mut f64,
        var_qbg_dn7_slot: &mut f64,
        var_qbg_dn8_slot: &mut f64,
        var_qfg_slot: &mut f64,
        var_qfg_dn3_slot: &mut f64,
        var_qfg_dn4_slot: &mut f64,
        var_qfg_dn5_slot: &mut f64,
        var_qfg_dn6_slot: &mut f64,
        var_qfg_dn7_slot: &mut f64,
        var_qfg_dn8_slot: &mut f64,
        var_qfgd_parasitic_slot: &mut f64,
        var_qfgd_parasitic_dn3_slot: &mut f64,
        var_qfgd_parasitic_dn4_slot: &mut f64,
        var_qfgd_parasitic_dn5_slot: &mut f64,
        var_qfgd_parasitic_dn6_slot: &mut f64,
        var_qfgd_parasitic_dn7_slot: &mut f64,
        var_qfgd_parasitic_dn8_slot: &mut f64,
        var_qfgs_parasitic_slot: &mut f64,
        var_qfgs_parasitic_dn3_slot: &mut f64,
        var_qfgs_parasitic_dn4_slot: &mut f64,
        var_qfgs_parasitic_dn5_slot: &mut f64,
        var_qfgs_parasitic_dn6_slot: &mut f64,
        var_qfgs_parasitic_dn7_slot: &mut f64,
        var_qfgs_parasitic_dn8_slot: &mut f64,
        var_qinv_slot: &mut f64,
        var_qinv_dn3_slot: &mut f64,
        var_qinv_dn4_slot: &mut f64,
        var_qinv_dn5_slot: &mut f64,
        var_qinv_dn6_slot: &mut f64,
        var_qinv_dn7_slot: &mut f64,
        var_qinv_dn8_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
    ) {
        let mut var_gcrg: f64 = *var_gcrg_slot;
        let mut var_gcrg_dn3: f64 = *var_gcrg_dn3_slot;
        let mut var_gcrg_dn4: f64 = *var_gcrg_dn4_slot;
        let mut var_gcrg_dn5: f64 = *var_gcrg_dn5_slot;
        let mut var_gcrg_dn6: f64 = *var_gcrg_dn6_slot;
        let mut var_gcrg_dn7: f64 = *var_gcrg_dn7_slot;
        let mut var_gcrg_dn8: f64 = *var_gcrg_dn8_slot;
        let mut var_gdpr: f64 = *var_gdpr_slot;
        let mut var_gdpr_dn3: f64 = *var_gdpr_dn3_slot;
        let mut var_gdpr_dn4: f64 = *var_gdpr_dn4_slot;
        let mut var_gdpr_dn5: f64 = *var_gdpr_dn5_slot;
        let mut var_gdpr_dn6: f64 = *var_gdpr_dn6_slot;
        let mut var_gdpr_dn7: f64 = *var_gdpr_dn7_slot;
        let mut var_gdpr_dn8: f64 = *var_gdpr_dn8_slot;
        let mut var_gspr: f64 = *var_gspr_slot;
        let mut var_gspr_dn3: f64 = *var_gspr_dn3_slot;
        let mut var_gspr_dn4: f64 = *var_gspr_dn4_slot;
        let mut var_gspr_dn5: f64 = *var_gspr_dn5_slot;
        let mut var_gspr_dn6: f64 = *var_gspr_dn6_slot;
        let mut var_gspr_dn7: f64 = *var_gspr_dn7_slot;
        let mut var_gspr_dn8: f64 = *var_gspr_dn8_slot;
        let mut var_guard137: f64 = *var_guard137_slot;
        let mut var_guard138: f64 = *var_guard138_slot;
        let mut var_guard139: f64 = *var_guard139_slot;
        let mut var_guard140: f64 = *var_guard140_slot;
        let mut var_guard147: f64 = *var_guard147_slot;
        let mut var_guard148: f64 = *var_guard148_slot;
        let mut var_idsovvds: f64 = *var_idsovvds_slot;
        let mut var_idsovvds_dn3: f64 = *var_idsovvds_dn3_slot;
        let mut var_idsovvds_dn4: f64 = *var_idsovvds_dn4_slot;
        let mut var_idsovvds_dn5: f64 = *var_idsovvds_dn5_slot;
        let mut var_idsovvds_dn6: f64 = *var_idsovvds_dn6_slot;
        let mut var_idsovvds_dn7: f64 = *var_idsovvds_dn7_slot;
        let mut var_idsovvds_dn8: f64 = *var_idsovvds_dn8_slot;
        let mut var_igcd: f64 = *var_igcd_slot;
        let mut var_igcd_dn3: f64 = *var_igcd_dn3_slot;
        let mut var_igcd_dn4: f64 = *var_igcd_dn4_slot;
        let mut var_igcd_dn5: f64 = *var_igcd_dn5_slot;
        let mut var_igcd_dn6: f64 = *var_igcd_dn6_slot;
        let mut var_igcd_dn7: f64 = *var_igcd_dn7_slot;
        let mut var_igcd_dn8: f64 = *var_igcd_dn8_slot;
        let mut var_igcs: f64 = *var_igcs_slot;
        let mut var_igcs_dn3: f64 = *var_igcs_dn3_slot;
        let mut var_igcs_dn4: f64 = *var_igcs_dn4_slot;
        let mut var_igcs_dn5: f64 = *var_igcs_dn5_slot;
        let mut var_igcs_dn6: f64 = *var_igcs_dn6_slot;
        let mut var_igcs_dn7: f64 = *var_igcs_dn7_slot;
        let mut var_igcs_dn8: f64 = *var_igcs_dn8_slot;
        let mut var_igd: f64 = *var_igd_slot;
        let mut var_igd_dn3: f64 = *var_igd_dn3_slot;
        let mut var_igd_dn4: f64 = *var_igd_dn4_slot;
        let mut var_igd_dn5: f64 = *var_igd_dn5_slot;
        let mut var_igd_dn6: f64 = *var_igd_dn6_slot;
        let mut var_igd_dn7: f64 = *var_igd_dn7_slot;
        let mut var_igd_dn8: f64 = *var_igd_dn8_slot;
        let mut var_igidl: f64 = *var_igidl_slot;
        let mut var_igidl_dn3: f64 = *var_igidl_dn3_slot;
        let mut var_igidl_dn4: f64 = *var_igidl_dn4_slot;
        let mut var_igidl_dn5: f64 = *var_igidl_dn5_slot;
        let mut var_igidl_dn6: f64 = *var_igidl_dn6_slot;
        let mut var_igidl_dn7: f64 = *var_igidl_dn7_slot;
        let mut var_igidl_dn8: f64 = *var_igidl_dn8_slot;
        let mut var_igisl: f64 = *var_igisl_slot;
        let mut var_igisl_dn3: f64 = *var_igisl_dn3_slot;
        let mut var_igisl_dn4: f64 = *var_igisl_dn4_slot;
        let mut var_igisl_dn5: f64 = *var_igisl_dn5_slot;
        let mut var_igisl_dn6: f64 = *var_igisl_dn6_slot;
        let mut var_igisl_dn7: f64 = *var_igisl_dn7_slot;
        let mut var_igisl_dn8: f64 = *var_igisl_dn8_slot;
        let mut var_igs: f64 = *var_igs_slot;
        let mut var_igs_dn3: f64 = *var_igs_dn3_slot;
        let mut var_igs_dn4: f64 = *var_igs_dn4_slot;
        let mut var_igs_dn5: f64 = *var_igs_dn5_slot;
        let mut var_igs_dn6: f64 = *var_igs_dn6_slot;
        let mut var_igs_dn7: f64 = *var_igs_dn7_slot;
        let mut var_igs_dn8: f64 = *var_igs_dn8_slot;
        let mut var_qbg: f64 = *var_qbg_slot;
        let mut var_qbg_dn3: f64 = *var_qbg_dn3_slot;
        let mut var_qbg_dn4: f64 = *var_qbg_dn4_slot;
        let mut var_qbg_dn5: f64 = *var_qbg_dn5_slot;
        let mut var_qbg_dn6: f64 = *var_qbg_dn6_slot;
        let mut var_qbg_dn7: f64 = *var_qbg_dn7_slot;
        let mut var_qbg_dn8: f64 = *var_qbg_dn8_slot;
        let mut var_qfg: f64 = *var_qfg_slot;
        let mut var_qfg_dn3: f64 = *var_qfg_dn3_slot;
        let mut var_qfg_dn4: f64 = *var_qfg_dn4_slot;
        let mut var_qfg_dn5: f64 = *var_qfg_dn5_slot;
        let mut var_qfg_dn6: f64 = *var_qfg_dn6_slot;
        let mut var_qfg_dn7: f64 = *var_qfg_dn7_slot;
        let mut var_qfg_dn8: f64 = *var_qfg_dn8_slot;
        let mut var_qfgd_parasitic: f64 = *var_qfgd_parasitic_slot;
        let mut var_qfgd_parasitic_dn3: f64 = *var_qfgd_parasitic_dn3_slot;
        let mut var_qfgd_parasitic_dn4: f64 = *var_qfgd_parasitic_dn4_slot;
        let mut var_qfgd_parasitic_dn5: f64 = *var_qfgd_parasitic_dn5_slot;
        let mut var_qfgd_parasitic_dn6: f64 = *var_qfgd_parasitic_dn6_slot;
        let mut var_qfgd_parasitic_dn7: f64 = *var_qfgd_parasitic_dn7_slot;
        let mut var_qfgd_parasitic_dn8: f64 = *var_qfgd_parasitic_dn8_slot;
        let mut var_qfgs_parasitic: f64 = *var_qfgs_parasitic_slot;
        let mut var_qfgs_parasitic_dn3: f64 = *var_qfgs_parasitic_dn3_slot;
        let mut var_qfgs_parasitic_dn4: f64 = *var_qfgs_parasitic_dn4_slot;
        let mut var_qfgs_parasitic_dn5: f64 = *var_qfgs_parasitic_dn5_slot;
        let mut var_qfgs_parasitic_dn6: f64 = *var_qfgs_parasitic_dn6_slot;
        let mut var_qfgs_parasitic_dn7: f64 = *var_qfgs_parasitic_dn7_slot;
        let mut var_qfgs_parasitic_dn8: f64 = *var_qfgs_parasitic_dn8_slot;
        let mut var_qinv: f64 = *var_qinv_slot;
        let mut var_qinv_dn3: f64 = *var_qinv_dn3_slot;
        let mut var_qinv_dn4: f64 = *var_qinv_dn4_slot;
        let mut var_qinv_dn5: f64 = *var_qinv_dn5_slot;
        let mut var_qinv_dn6: f64 = *var_qinv_dn6_slot;
        let mut var_qinv_dn7: f64 = *var_qinv_dn7_slot;
        let mut var_qinv_dn8: f64 = *var_qinv_dn8_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;

        let assign13190_e12238: f64 = (var_qfgs_parasitic + var_qfgd_parasitic);
        let assign13190_e12239: f64 = (p.p2 * assign13190_e12238);
        let assign13190_e12240: f64 = (var_qfgi + assign13190_e12239);
        var_qfg = assign13190_e12240;
        var_qfg_dn3 = (var_qfgi_dn3 + (p.p2 * (var_qfgs_parasitic_dn3 + var_qfgd_parasitic_dn3)));
        var_qfg_dn4 = (var_qfgi_dn4 + (p.p2 * (var_qfgs_parasitic_dn4 + var_qfgd_parasitic_dn4)));
        var_qfg_dn5 = (var_qfgi_dn5 + (p.p2 * (var_qfgs_parasitic_dn5 + var_qfgd_parasitic_dn5)));
        var_qfg_dn6 = (var_qfgi_dn6 + (p.p2 * (var_qfgs_parasitic_dn6 + var_qfgd_parasitic_dn6)));
        var_qfg_dn7 = (var_qfgi_dn7 + (p.p2 * (var_qfgs_parasitic_dn7 + var_qfgd_parasitic_dn7)));
        var_qfg_dn8 = (var_qfgi_dn8 + (p.p2 * (var_qfgs_parasitic_dn8 + var_qfgd_parasitic_dn8)));

        let assign13200_e12243: f64 = (p.p2 * var_qbg);
        let assign13200_e12245: f64 = (assign13200_e12243 - var_qsbg);
        let assign13200_e12247: f64 = (assign13200_e12245 - var_qdbg);
        var_qbg = assign13200_e12247;
        var_qbg_dn3 = (((p.p2 * var_qbg_dn3) - var_qsbg_dn3) - var_qdbg_dn3);
        var_qbg_dn4 = (((p.p2 * var_qbg_dn4) - var_qsbg_dn4) - var_qdbg_dn4);
        var_qbg_dn5 = (((p.p2 * var_qbg_dn5) - var_qsbg_dn5) - var_qdbg_dn5);
        var_qbg_dn6 = (((p.p2 * var_qbg_dn6) - var_qsbg_dn6) - var_qdbg_dn6);
        var_qbg_dn7 = (((p.p2 * var_qbg_dn7) - var_qsbg_dn7) - var_qdbg_dn7);
        var_qbg_dn8 = (((p.p2 * var_qbg_dn8) - var_qsbg_dn8) - var_qdbg_dn8);

        let assign13210_e12250: f64 = (p.p2 * var_qfgs_parasitic);
        var_qfgs_parasitic = assign13210_e12250;
        var_qfgs_parasitic_dn3 = (p.p2 * var_qfgs_parasitic_dn3);
        var_qfgs_parasitic_dn4 = (p.p2 * var_qfgs_parasitic_dn4);
        var_qfgs_parasitic_dn5 = (p.p2 * var_qfgs_parasitic_dn5);
        var_qfgs_parasitic_dn6 = (p.p2 * var_qfgs_parasitic_dn6);
        var_qfgs_parasitic_dn7 = (p.p2 * var_qfgs_parasitic_dn7);
        var_qfgs_parasitic_dn8 = (p.p2 * var_qfgs_parasitic_dn8);

        let assign13220_e12253: f64 = (p.p2 * var_qfgd_parasitic);
        var_qfgd_parasitic = assign13220_e12253;
        var_qfgd_parasitic_dn3 = (p.p2 * var_qfgd_parasitic_dn3);
        var_qfgd_parasitic_dn4 = (p.p2 * var_qfgd_parasitic_dn4);
        var_qfgd_parasitic_dn5 = (p.p2 * var_qfgd_parasitic_dn5);
        var_qfgd_parasitic_dn6 = (p.p2 * var_qfgd_parasitic_dn6);
        var_qfgd_parasitic_dn7 = (p.p2 * var_qfgd_parasitic_dn7);
        var_qfgd_parasitic_dn8 = (p.p2 * var_qfgd_parasitic_dn8);

        let assign13230_e12256: f64 = (var_qsi + var_qdi);
        let assign13230_e12257: f64 = (-assign13230_e12256);
        var_qinv = assign13230_e12257;
        var_qinv_dn3 = (-(var_qsi_dn3 + var_qdi_dn3));
        var_qinv_dn4 = (-(var_qsi_dn4 + var_qdi_dn4));
        var_qinv_dn5 = (-(var_qsi_dn5 + var_qdi_dn5));
        var_qinv_dn6 = (-(var_qsi_dn6 + var_qdi_dn6));
        var_qinv_dn7 = (-(var_qsi_dn7 + var_qdi_dn7));
        var_qinv_dn8 = (-(var_qsi_dn8 + var_qdi_dn8));

        let assign13240_e12260: f64 = (var_utotal * var_qinv);
        var_t0 = assign13240_e12260;
        var_t0_dn3 = ((var_utotal_dn3 * var_qinv) + (var_utotal * var_qinv_dn3));
        var_t0_dn4 = ((var_utotal_dn4 * var_qinv) + (var_utotal * var_qinv_dn4));
        var_t0_dn5 = ((var_utotal_dn5 * var_qinv) + (var_utotal * var_qinv_dn5));
        var_t0_dn6 = ((var_utotal_dn6 * var_qinv) + (var_utotal * var_qinv_dn6));
        var_t0_dn7 = ((var_utotal_dn7 * var_qinv) + (var_utotal * var_qinv_dn7));
        var_t0_dn8 = ((var_utotal_dn8 * var_qinv) + (var_utotal * var_qinv_dn8));

        let assign13250_e12263: f64 = (var_t0 * var_rdsi);
        let assign13250_e12266: f64 = (var_leff * var_leff);
        let assign13250_e12267: f64 = (assign13250_e12263 + assign13250_e12266);
        var_t1 = assign13250_e12267;
        var_t1_dn3 = ((var_t0_dn3 * var_rdsi) + (var_t0 * var_rdsi_dn3));
        var_t1_dn4 = ((var_t0_dn4 * var_rdsi) + (var_t0 * var_rdsi_dn4));
        var_t1_dn5 = ((var_t0_dn5 * var_rdsi) + (var_t0 * var_rdsi_dn5));
        var_t1_dn6 = ((var_t0_dn6 * var_rdsi) + (var_t0 * var_rdsi_dn6));
        var_t1_dn7 = ((var_t0_dn7 * var_rdsi) + (var_t0 * var_rdsi_dn7));
        var_t1_dn8 = ((var_t0_dn8 * var_rdsi) + (var_t0 * var_rdsi_dn8));

        let assign13290_e12287: f64 = if ((p.p20 == 1.0) && (var_xrcrg1_i != 0.0)) { 1.0 } else { 0.0 };
        var_guard137 = assign13290_e12287;

        let (assign13300_e12297, assign13300_e12297_d_n3, assign13300_e12297_d_n4, assign13300_e12297_d_n5, assign13300_e12297_d_n6, assign13300_e12297_d_n7, assign13300_e12297_d_n8,) = {
    if (var_guard137 != 0.0) {
        let assign13300_e12291: f64 = (var_utotal * var_cox1);
        let assign13300_e12293: f64 = (assign13300_e12291 * var_weff);
        let assign13300_e12295: f64 = (assign13300_e12293 / var_leff);
        (assign13300_e12295, (((var_utotal_dn3 * var_cox1) * var_weff) / var_leff), (((var_utotal_dn4 * var_cox1) * var_weff) / var_leff), (((var_utotal_dn5 * var_cox1) * var_weff) / var_leff), (((var_utotal_dn6 * var_cox1) * var_weff) / var_leff), (((var_utotal_dn7 * var_cox1) * var_weff) / var_leff), (((var_utotal_dn8 * var_cox1) * var_weff) / var_leff),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8,)
    }
};
        var_t0 = assign13300_e12297;
        var_t0_dn3 = assign13300_e12297_d_n3;
        var_t0_dn4 = assign13300_e12297_d_n4;
        var_t0_dn5 = assign13300_e12297_d_n5;
        var_t0_dn6 = assign13300_e12297_d_n6;
        var_t0_dn7 = assign13300_e12297_d_n7;
        var_t0_dn8 = assign13300_e12297_d_n8;

        let (assign13310_e12303, assign13310_e12303_d_n3, assign13310_e12303_d_n4, assign13310_e12303_d_n5, assign13310_e12303_d_n6, assign13310_e12303_d_n7, assign13310_e12303_d_n8,) = {
    if (var_guard137 != 0.0) {
        let assign13310_e12301: f64 = (var_t0 * var_qia);
        (assign13310_e12301, ((var_t0_dn3 * var_qia) + (var_t0 * var_qia_dn3)), ((var_t0_dn4 * var_qia) + (var_t0 * var_qia_dn4)), ((var_t0_dn5 * var_qia) + (var_t0 * var_qia_dn5)), ((var_t0_dn6 * var_qia) + (var_t0 * var_qia_dn6)), ((var_t0_dn7 * var_qia) + (var_t0 * var_qia_dn7)), ((var_t0_dn8 * var_qia) + (var_t0 * var_qia_dn8)),)
    } else {
        (var_idsovvds, var_idsovvds_dn3, var_idsovvds_dn4, var_idsovvds_dn5, var_idsovvds_dn6, var_idsovvds_dn7, var_idsovvds_dn8,)
    }
};
        var_idsovvds = assign13310_e12303;
        var_idsovvds_dn3 = assign13310_e12303_d_n3;
        var_idsovvds_dn4 = assign13310_e12303_d_n4;
        var_idsovvds_dn5 = assign13310_e12303_d_n5;
        var_idsovvds_dn6 = assign13310_e12303_d_n6;
        var_idsovvds_dn7 = assign13310_e12303_d_n7;
        var_idsovvds_dn8 = assign13310_e12303_d_n8;

        let (assign13320_e12317, assign13320_e12317_d_n3, assign13320_e12317_d_n4, assign13320_e12317_d_n5, assign13320_e12317_d_n6, assign13320_e12317_d_n7, assign13320_e12317_d_n8,) = {
    if (var_guard137 != 0.0) {
        let assign13320_e12307: f64 = (p.p2 * var_xrcrg1_i);
        let assign13320_e12311: f64 = (var_xrcrg2_i * var_vtm);
        let assign13320_e12313: f64 = (assign13320_e12311 * var_t0);
        let assign13320_e12314: f64 = (var_idsovvds + assign13320_e12313);
        let assign13320_e12315: f64 = (assign13320_e12307 * assign13320_e12314);
        (assign13320_e12315, (assign13320_e12307 * (var_idsovvds_dn3 + (assign13320_e12311 * var_t0_dn3))), (assign13320_e12307 * (var_idsovvds_dn4 + (((var_xrcrg2_i * var_vtm_dn4) * var_t0) + (assign13320_e12311 * var_t0_dn4)))), (assign13320_e12307 * (var_idsovvds_dn5 + (assign13320_e12311 * var_t0_dn5))), (assign13320_e12307 * (var_idsovvds_dn6 + (assign13320_e12311 * var_t0_dn6))), (assign13320_e12307 * (var_idsovvds_dn7 + (assign13320_e12311 * var_t0_dn7))), (assign13320_e12307 * (var_idsovvds_dn8 + (assign13320_e12311 * var_t0_dn8))),)
    } else {
        (var_gcrg, var_gcrg_dn3, var_gcrg_dn4, var_gcrg_dn5, var_gcrg_dn6, var_gcrg_dn7, var_gcrg_dn8,)
    }
};
        var_gcrg = assign13320_e12317;
        var_gcrg_dn3 = assign13320_e12317_d_n3;
        var_gcrg_dn4 = assign13320_e12317_d_n4;
        var_gcrg_dn5 = assign13320_e12317_d_n5;
        var_gcrg_dn6 = assign13320_e12317_d_n6;
        var_gcrg_dn7 = assign13320_e12317_d_n7;
        var_gcrg_dn8 = assign13320_e12317_d_n8;

        let (assign13330_e12322, assign13330_e12322_d_n3, assign13330_e12322_d_n4, assign13330_e12322_d_n5, assign13330_e12322_d_n6, assign13330_e12322_d_n7, assign13330_e12322_d_n8,) = {
    if (var_guard137 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_gcrg, var_gcrg_dn3, var_gcrg_dn4, var_gcrg_dn5, var_gcrg_dn6, var_gcrg_dn7, var_gcrg_dn8,)
    }
};
        var_gcrg = assign13330_e12322;
        var_gcrg_dn3 = assign13330_e12322_d_n3;
        var_gcrg_dn4 = assign13330_e12322_d_n4;
        var_gcrg_dn5 = assign13330_e12322_d_n5;
        var_gcrg_dn6 = assign13330_e12322_d_n6;
        var_gcrg_dn7 = assign13330_e12322_d_n7;
        var_gcrg_dn8 = assign13330_e12322_d_n8;

        let assign13340_e12325: f64 = (p.p2 * var_igidl);
        var_igidl = assign13340_e12325;
        var_igidl_dn3 = (p.p2 * var_igidl_dn3);
        var_igidl_dn4 = (p.p2 * var_igidl_dn4);
        var_igidl_dn5 = (p.p2 * var_igidl_dn5);
        var_igidl_dn6 = (p.p2 * var_igidl_dn6);
        var_igidl_dn7 = (p.p2 * var_igidl_dn7);
        var_igidl_dn8 = (p.p2 * var_igidl_dn8);

        let assign13350_e12328: f64 = (p.p2 * var_igisl);
        var_igisl = assign13350_e12328;
        var_igisl_dn3 = (p.p2 * var_igisl_dn3);
        var_igisl_dn4 = (p.p2 * var_igisl_dn4);
        var_igisl_dn5 = (p.p2 * var_igisl_dn5);
        var_igisl_dn6 = (p.p2 * var_igisl_dn6);
        var_igisl_dn7 = (p.p2 * var_igisl_dn7);
        var_igisl_dn8 = (p.p2 * var_igisl_dn8);

        let assign13360_e12331: f64 = (p.p2 * var_igcd);
        var_igcd = assign13360_e12331;
        var_igcd_dn3 = (p.p2 * var_igcd_dn3);
        var_igcd_dn4 = (p.p2 * var_igcd_dn4);
        var_igcd_dn5 = (p.p2 * var_igcd_dn5);
        var_igcd_dn6 = (p.p2 * var_igcd_dn6);
        var_igcd_dn7 = (p.p2 * var_igcd_dn7);
        var_igcd_dn8 = (p.p2 * var_igcd_dn8);

        let assign13370_e12334: f64 = (p.p2 * var_igcs);
        var_igcs = assign13370_e12334;
        var_igcs_dn3 = (p.p2 * var_igcs_dn3);
        var_igcs_dn4 = (p.p2 * var_igcs_dn4);
        var_igcs_dn5 = (p.p2 * var_igcs_dn5);
        var_igcs_dn6 = (p.p2 * var_igcs_dn6);
        var_igcs_dn7 = (p.p2 * var_igcs_dn7);
        var_igcs_dn8 = (p.p2 * var_igcs_dn8);

        let assign13380_e12337: f64 = (p.p2 * var_igs);
        var_igs = assign13380_e12337;
        var_igs_dn3 = (p.p2 * var_igs_dn3);
        var_igs_dn4 = (p.p2 * var_igs_dn4);
        var_igs_dn5 = (p.p2 * var_igs_dn5);
        var_igs_dn6 = (p.p2 * var_igs_dn6);
        var_igs_dn7 = (p.p2 * var_igs_dn7);
        var_igs_dn8 = (p.p2 * var_igs_dn8);

        let assign13390_e12340: f64 = (p.p2 * var_igd);
        var_igd = assign13390_e12340;
        var_igd_dn3 = (p.p2 * var_igd_dn3);
        var_igd_dn4 = (p.p2 * var_igd_dn4);
        var_igd_dn5 = (p.p2 * var_igd_dn5);
        var_igd_dn6 = (p.p2 * var_igd_dn6);
        var_igd_dn7 = (p.p2 * var_igd_dn7);
        var_igd_dn8 = (p.p2 * var_igd_dn8);

        let assign13400_e12343: f64 = if var_sigvds > 0.0 { 1.0 } else { 0.0 };
        var_guard138 = assign13400_e12343;

        let assign13410_e12346: f64 = if p.p14 == 2.0 { 1.0 } else { 0.0 };
        var_guard139 = assign13410_e12346;

        let (assign13420_e12353, assign13420_e12353_d_n3, assign13420_e12353_d_n4, assign13420_e12353_d_n5, assign13420_e12353_d_n6, assign13420_e12353_d_n7, assign13420_e12353_d_n8,) = {
    if (var_guard139 == 0.0) {
        let assign13420_e12351: f64 = (1.0 / var_rdrain);
        (assign13420_e12351, (-(var_rdrain_dn3 / (var_rdrain * var_rdrain))), (-(var_rdrain_dn4 / (var_rdrain * var_rdrain))), (-(var_rdrain_dn5 / (var_rdrain * var_rdrain))), (-(var_rdrain_dn6 / (var_rdrain * var_rdrain))), (-(var_rdrain_dn7 / (var_rdrain * var_rdrain))), (-(var_rdrain_dn8 / (var_rdrain * var_rdrain))),)
    } else {
        (var_gdpr, var_gdpr_dn3, var_gdpr_dn4, var_gdpr_dn5, var_gdpr_dn6, var_gdpr_dn7, var_gdpr_dn8,)
    }
};
        var_gdpr = assign13420_e12353;
        var_gdpr_dn3 = assign13420_e12353_d_n3;
        var_gdpr_dn4 = assign13420_e12353_d_n4;
        var_gdpr_dn5 = assign13420_e12353_d_n5;
        var_gdpr_dn6 = assign13420_e12353_d_n6;
        var_gdpr_dn7 = assign13420_e12353_d_n7;
        var_gdpr_dn8 = assign13420_e12353_d_n8;

        let (assign13430_e12360, assign13430_e12360_d_n3, assign13430_e12360_d_n4, assign13430_e12360_d_n5, assign13430_e12360_d_n6, assign13430_e12360_d_n7, assign13430_e12360_d_n8,) = {
    if (var_guard139 == 0.0) {
        let assign13430_e12358: f64 = (1.0 / var_rsource);
        (assign13430_e12358, (-(var_rsource_dn3 / (var_rsource * var_rsource))), (-(var_rsource_dn4 / (var_rsource * var_rsource))), (-(var_rsource_dn5 / (var_rsource * var_rsource))), (-(var_rsource_dn6 / (var_rsource * var_rsource))), (-(var_rsource_dn7 / (var_rsource * var_rsource))), (-(var_rsource_dn8 / (var_rsource * var_rsource))),)
    } else {
        (var_gspr, var_gspr_dn3, var_gspr_dn4, var_gspr_dn5, var_gspr_dn6, var_gspr_dn7, var_gspr_dn8,)
    }
};
        var_gspr = assign13430_e12360;
        var_gspr_dn3 = assign13430_e12360_d_n3;
        var_gspr_dn4 = assign13430_e12360_d_n4;
        var_gspr_dn5 = assign13430_e12360_d_n5;
        var_gspr_dn6 = assign13430_e12360_d_n6;
        var_gspr_dn7 = assign13430_e12360_d_n7;
        var_gspr_dn8 = assign13430_e12360_d_n8;

        let assign13440_e12367: f64 = if ((p.p20 == 1.0) && (var_xrcrg1_i != 0.0)) { 1.0 } else { 0.0 };
        var_guard140 = assign13440_e12367;

        let assign13510_e12396: f64 = if ((p.p18 != 0.0) && (p.p310 > 0.0)) { 1.0 } else { 0.0 };
        var_guard147 = assign13510_e12396;

        let assign13520_e12399: f64 = if p.p14 != 2.0 { 1.0 } else { 0.0 };
        var_guard148 = assign13520_e12399;

        *var_gcrg_slot = var_gcrg;
        *var_gcrg_dn3_slot = var_gcrg_dn3;
        *var_gcrg_dn4_slot = var_gcrg_dn4;
        *var_gcrg_dn5_slot = var_gcrg_dn5;
        *var_gcrg_dn6_slot = var_gcrg_dn6;
        *var_gcrg_dn7_slot = var_gcrg_dn7;
        *var_gcrg_dn8_slot = var_gcrg_dn8;
        *var_gdpr_slot = var_gdpr;
        *var_gdpr_dn3_slot = var_gdpr_dn3;
        *var_gdpr_dn4_slot = var_gdpr_dn4;
        *var_gdpr_dn5_slot = var_gdpr_dn5;
        *var_gdpr_dn6_slot = var_gdpr_dn6;
        *var_gdpr_dn7_slot = var_gdpr_dn7;
        *var_gdpr_dn8_slot = var_gdpr_dn8;
        *var_gspr_slot = var_gspr;
        *var_gspr_dn3_slot = var_gspr_dn3;
        *var_gspr_dn4_slot = var_gspr_dn4;
        *var_gspr_dn5_slot = var_gspr_dn5;
        *var_gspr_dn6_slot = var_gspr_dn6;
        *var_gspr_dn7_slot = var_gspr_dn7;
        *var_gspr_dn8_slot = var_gspr_dn8;
        *var_guard137_slot = var_guard137;
        *var_guard138_slot = var_guard138;
        *var_guard139_slot = var_guard139;
        *var_guard140_slot = var_guard140;
        *var_guard147_slot = var_guard147;
        *var_guard148_slot = var_guard148;
        *var_idsovvds_slot = var_idsovvds;
        *var_idsovvds_dn3_slot = var_idsovvds_dn3;
        *var_idsovvds_dn4_slot = var_idsovvds_dn4;
        *var_idsovvds_dn5_slot = var_idsovvds_dn5;
        *var_idsovvds_dn6_slot = var_idsovvds_dn6;
        *var_idsovvds_dn7_slot = var_idsovvds_dn7;
        *var_idsovvds_dn8_slot = var_idsovvds_dn8;
        *var_igcd_slot = var_igcd;
        *var_igcd_dn3_slot = var_igcd_dn3;
        *var_igcd_dn4_slot = var_igcd_dn4;
        *var_igcd_dn5_slot = var_igcd_dn5;
        *var_igcd_dn6_slot = var_igcd_dn6;
        *var_igcd_dn7_slot = var_igcd_dn7;
        *var_igcd_dn8_slot = var_igcd_dn8;
        *var_igcs_slot = var_igcs;
        *var_igcs_dn3_slot = var_igcs_dn3;
        *var_igcs_dn4_slot = var_igcs_dn4;
        *var_igcs_dn5_slot = var_igcs_dn5;
        *var_igcs_dn6_slot = var_igcs_dn6;
        *var_igcs_dn7_slot = var_igcs_dn7;
        *var_igcs_dn8_slot = var_igcs_dn8;
        *var_igd_slot = var_igd;
        *var_igd_dn3_slot = var_igd_dn3;
        *var_igd_dn4_slot = var_igd_dn4;
        *var_igd_dn5_slot = var_igd_dn5;
        *var_igd_dn6_slot = var_igd_dn6;
        *var_igd_dn7_slot = var_igd_dn7;
        *var_igd_dn8_slot = var_igd_dn8;
        *var_igidl_slot = var_igidl;
        *var_igidl_dn3_slot = var_igidl_dn3;
        *var_igidl_dn4_slot = var_igidl_dn4;
        *var_igidl_dn5_slot = var_igidl_dn5;
        *var_igidl_dn6_slot = var_igidl_dn6;
        *var_igidl_dn7_slot = var_igidl_dn7;
        *var_igidl_dn8_slot = var_igidl_dn8;
        *var_igisl_slot = var_igisl;
        *var_igisl_dn3_slot = var_igisl_dn3;
        *var_igisl_dn4_slot = var_igisl_dn4;
        *var_igisl_dn5_slot = var_igisl_dn5;
        *var_igisl_dn6_slot = var_igisl_dn6;
        *var_igisl_dn7_slot = var_igisl_dn7;
        *var_igisl_dn8_slot = var_igisl_dn8;
        *var_igs_slot = var_igs;
        *var_igs_dn3_slot = var_igs_dn3;
        *var_igs_dn4_slot = var_igs_dn4;
        *var_igs_dn5_slot = var_igs_dn5;
        *var_igs_dn6_slot = var_igs_dn6;
        *var_igs_dn7_slot = var_igs_dn7;
        *var_igs_dn8_slot = var_igs_dn8;
        *var_qbg_slot = var_qbg;
        *var_qbg_dn3_slot = var_qbg_dn3;
        *var_qbg_dn4_slot = var_qbg_dn4;
        *var_qbg_dn5_slot = var_qbg_dn5;
        *var_qbg_dn6_slot = var_qbg_dn6;
        *var_qbg_dn7_slot = var_qbg_dn7;
        *var_qbg_dn8_slot = var_qbg_dn8;
        *var_qfg_slot = var_qfg;
        *var_qfg_dn3_slot = var_qfg_dn3;
        *var_qfg_dn4_slot = var_qfg_dn4;
        *var_qfg_dn5_slot = var_qfg_dn5;
        *var_qfg_dn6_slot = var_qfg_dn6;
        *var_qfg_dn7_slot = var_qfg_dn7;
        *var_qfg_dn8_slot = var_qfg_dn8;
        *var_qfgd_parasitic_slot = var_qfgd_parasitic;
        *var_qfgd_parasitic_dn3_slot = var_qfgd_parasitic_dn3;
        *var_qfgd_parasitic_dn4_slot = var_qfgd_parasitic_dn4;
        *var_qfgd_parasitic_dn5_slot = var_qfgd_parasitic_dn5;
        *var_qfgd_parasitic_dn6_slot = var_qfgd_parasitic_dn6;
        *var_qfgd_parasitic_dn7_slot = var_qfgd_parasitic_dn7;
        *var_qfgd_parasitic_dn8_slot = var_qfgd_parasitic_dn8;
        *var_qfgs_parasitic_slot = var_qfgs_parasitic;
        *var_qfgs_parasitic_dn3_slot = var_qfgs_parasitic_dn3;
        *var_qfgs_parasitic_dn4_slot = var_qfgs_parasitic_dn4;
        *var_qfgs_parasitic_dn5_slot = var_qfgs_parasitic_dn5;
        *var_qfgs_parasitic_dn6_slot = var_qfgs_parasitic_dn6;
        *var_qfgs_parasitic_dn7_slot = var_qfgs_parasitic_dn7;
        *var_qfgs_parasitic_dn8_slot = var_qfgs_parasitic_dn8;
        *var_qinv_slot = var_qinv;
        *var_qinv_dn3_slot = var_qinv_dn3;
        *var_qinv_dn4_slot = var_qinv_dn4;
        *var_qinv_dn5_slot = var_qinv_dn5;
        *var_qinv_dn6_slot = var_qinv_dn6;
        *var_qinv_dn7_slot = var_qinv_dn7;
        *var_qinv_dn8_slot = var_qinv_dn8;
        *var_t0_slot = var_t0;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t1_slot = var_t1;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
    }

    pub(super) fn stamp_reactive_block_0(
        p: &Parameters,
        var_devsign_slot: &mut f64,
        var_devsign_rv_slot: &mut f64,
        var_dliv_slot: &mut f64,
        var_dliv_db0_slot: &mut f64,
        var_dliv_db1_slot: &mut f64,
        var_dliv_db2_slot: &mut f64,
        var_dliv_db3_slot: &mut f64,
        var_dliv_db4_slot: &mut f64,
        var_dliv_dn0_slot: &mut f64,
        var_dliv_dn1_slot: &mut f64,
        var_dliv_dn2_slot: &mut f64,
        var_dliv_dn3_slot: &mut f64,
        var_dliv_dn4_slot: &mut f64,
        var_dliv_dn5_slot: &mut f64,
        var_dliv_dn6_slot: &mut f64,
        var_dliv_dn7_slot: &mut f64,
        var_dliv_dn8_slot: &mut f64,
        var_dliv_rdb0_slot: &mut f64,
        var_dliv_rdb1_slot: &mut f64,
        var_dliv_rdb2_slot: &mut f64,
        var_dliv_rdb3_slot: &mut f64,
        var_dliv_rdb4_slot: &mut f64,
        var_dliv_rdn0_slot: &mut f64,
        var_dliv_rdn1_slot: &mut f64,
        var_dliv_rdn2_slot: &mut f64,
        var_dliv_rdn3_slot: &mut f64,
        var_dliv_rdn4_slot: &mut f64,
        var_dliv_rdn5_slot: &mut f64,
        var_dliv_rdn6_slot: &mut f64,
        var_dliv_rdn7_slot: &mut f64,
        var_dliv_rdn8_slot: &mut f64,
        var_dliv_rv_slot: &mut f64,
        var_epssi_slot: &mut f64,
        var_epssi_db0_slot: &mut f64,
        var_epssi_db1_slot: &mut f64,
        var_epssi_db2_slot: &mut f64,
        var_epssi_db3_slot: &mut f64,
        var_epssi_db4_slot: &mut f64,
        var_epssi_dn0_slot: &mut f64,
        var_epssi_dn1_slot: &mut f64,
        var_epssi_dn2_slot: &mut f64,
        var_epssi_dn3_slot: &mut f64,
        var_epssi_dn4_slot: &mut f64,
        var_epssi_dn5_slot: &mut f64,
        var_epssi_dn6_slot: &mut f64,
        var_epssi_dn7_slot: &mut f64,
        var_epssi_dn8_slot: &mut f64,
        var_epssi_rdb0_slot: &mut f64,
        var_epssi_rdb1_slot: &mut f64,
        var_epssi_rdb2_slot: &mut f64,
        var_epssi_rdb3_slot: &mut f64,
        var_epssi_rdb4_slot: &mut f64,
        var_epssi_rdn0_slot: &mut f64,
        var_epssi_rdn1_slot: &mut f64,
        var_epssi_rdn2_slot: &mut f64,
        var_epssi_rdn3_slot: &mut f64,
        var_epssi_rdn4_slot: &mut f64,
        var_epssi_rdn5_slot: &mut f64,
        var_epssi_rdn6_slot: &mut f64,
        var_epssi_rdn7_slot: &mut f64,
        var_epssi_rdn8_slot: &mut f64,
        var_epssi_rv_slot: &mut f64,
        var_guard3_slot: &mut f64,
        var_guard3_db0_slot: &mut f64,
        var_guard3_db1_slot: &mut f64,
        var_guard3_db2_slot: &mut f64,
        var_guard3_db3_slot: &mut f64,
        var_guard3_db4_slot: &mut f64,
        var_guard3_dn0_slot: &mut f64,
        var_guard3_dn1_slot: &mut f64,
        var_guard3_dn2_slot: &mut f64,
        var_guard3_dn3_slot: &mut f64,
        var_guard3_dn4_slot: &mut f64,
        var_guard3_dn5_slot: &mut f64,
        var_guard3_dn6_slot: &mut f64,
        var_guard3_dn7_slot: &mut f64,
        var_guard3_dn8_slot: &mut f64,
        var_guard3_rdb0_slot: &mut f64,
        var_guard3_rdb1_slot: &mut f64,
        var_guard3_rdb2_slot: &mut f64,
        var_guard3_rdb3_slot: &mut f64,
        var_guard3_rdb4_slot: &mut f64,
        var_guard3_rdn0_slot: &mut f64,
        var_guard3_rdn1_slot: &mut f64,
        var_guard3_rdn2_slot: &mut f64,
        var_guard3_rdn3_slot: &mut f64,
        var_guard3_rdn4_slot: &mut f64,
        var_guard3_rdn5_slot: &mut f64,
        var_guard3_rdn6_slot: &mut f64,
        var_guard3_rdn7_slot: &mut f64,
        var_guard3_rdn8_slot: &mut f64,
        var_guard3_rv_slot: &mut f64,
        var_guard4_slot: &mut f64,
        var_guard4_db0_slot: &mut f64,
        var_guard4_db1_slot: &mut f64,
        var_guard4_db2_slot: &mut f64,
        var_guard4_db3_slot: &mut f64,
        var_guard4_db4_slot: &mut f64,
        var_guard4_dn0_slot: &mut f64,
        var_guard4_dn1_slot: &mut f64,
        var_guard4_dn2_slot: &mut f64,
        var_guard4_dn3_slot: &mut f64,
        var_guard4_dn4_slot: &mut f64,
        var_guard4_dn5_slot: &mut f64,
        var_guard4_dn6_slot: &mut f64,
        var_guard4_dn7_slot: &mut f64,
        var_guard4_dn8_slot: &mut f64,
        var_guard4_rdb0_slot: &mut f64,
        var_guard4_rdb1_slot: &mut f64,
        var_guard4_rdb2_slot: &mut f64,
        var_guard4_rdb3_slot: &mut f64,
        var_guard4_rdb4_slot: &mut f64,
        var_guard4_rdn0_slot: &mut f64,
        var_guard4_rdn1_slot: &mut f64,
        var_guard4_rdn2_slot: &mut f64,
        var_guard4_rdn3_slot: &mut f64,
        var_guard4_rdn4_slot: &mut f64,
        var_guard4_rdn5_slot: &mut f64,
        var_guard4_rdn6_slot: &mut f64,
        var_guard4_rdn7_slot: &mut f64,
        var_guard4_rdn8_slot: &mut f64,
        var_guard4_rv_slot: &mut f64,
        var_guard5_slot: &mut f64,
        var_guard5_db0_slot: &mut f64,
        var_guard5_db1_slot: &mut f64,
        var_guard5_db2_slot: &mut f64,
        var_guard5_db3_slot: &mut f64,
        var_guard5_db4_slot: &mut f64,
        var_guard5_dn0_slot: &mut f64,
        var_guard5_dn1_slot: &mut f64,
        var_guard5_dn2_slot: &mut f64,
        var_guard5_dn3_slot: &mut f64,
        var_guard5_dn4_slot: &mut f64,
        var_guard5_dn5_slot: &mut f64,
        var_guard5_dn6_slot: &mut f64,
        var_guard5_dn7_slot: &mut f64,
        var_guard5_dn8_slot: &mut f64,
        var_guard5_rdb0_slot: &mut f64,
        var_guard5_rdb1_slot: &mut f64,
        var_guard5_rdb2_slot: &mut f64,
        var_guard5_rdb3_slot: &mut f64,
        var_guard5_rdb4_slot: &mut f64,
        var_guard5_rdn0_slot: &mut f64,
        var_guard5_rdn1_slot: &mut f64,
        var_guard5_rdn2_slot: &mut f64,
        var_guard5_rdn3_slot: &mut f64,
        var_guard5_rdn4_slot: &mut f64,
        var_guard5_rdn5_slot: &mut f64,
        var_guard5_rdn6_slot: &mut f64,
        var_guard5_rdn7_slot: &mut f64,
        var_guard5_rdn8_slot: &mut f64,
        var_guard5_rv_slot: &mut f64,
        var_l_lln_slot: &mut f64,
        var_l_lln_db0_slot: &mut f64,
        var_l_lln_db1_slot: &mut f64,
        var_l_lln_db2_slot: &mut f64,
        var_l_lln_db3_slot: &mut f64,
        var_l_lln_db4_slot: &mut f64,
        var_l_lln_dn0_slot: &mut f64,
        var_l_lln_dn1_slot: &mut f64,
        var_l_lln_dn2_slot: &mut f64,
        var_l_lln_dn3_slot: &mut f64,
        var_l_lln_dn4_slot: &mut f64,
        var_l_lln_dn5_slot: &mut f64,
        var_l_lln_dn6_slot: &mut f64,
        var_l_lln_dn7_slot: &mut f64,
        var_l_lln_dn8_slot: &mut f64,
        var_l_lln_rdb0_slot: &mut f64,
        var_l_lln_rdb1_slot: &mut f64,
        var_l_lln_rdb2_slot: &mut f64,
        var_l_lln_rdb3_slot: &mut f64,
        var_l_lln_rdb4_slot: &mut f64,
        var_l_lln_rdn0_slot: &mut f64,
        var_l_lln_rdn1_slot: &mut f64,
        var_l_lln_rdn2_slot: &mut f64,
        var_l_lln_rdn3_slot: &mut f64,
        var_l_lln_rdn4_slot: &mut f64,
        var_l_lln_rdn5_slot: &mut f64,
        var_l_lln_rdn6_slot: &mut f64,
        var_l_lln_rdn7_slot: &mut f64,
        var_l_lln_rdn8_slot: &mut f64,
        var_l_lln_rv_slot: &mut f64,
        var_l_wln_slot: &mut f64,
        var_l_wln_db0_slot: &mut f64,
        var_l_wln_db1_slot: &mut f64,
        var_l_wln_db2_slot: &mut f64,
        var_l_wln_db3_slot: &mut f64,
        var_l_wln_db4_slot: &mut f64,
        var_l_wln_dn0_slot: &mut f64,
        var_l_wln_dn1_slot: &mut f64,
        var_l_wln_dn2_slot: &mut f64,
        var_l_wln_dn3_slot: &mut f64,
        var_l_wln_dn4_slot: &mut f64,
        var_l_wln_dn5_slot: &mut f64,
        var_l_wln_dn6_slot: &mut f64,
        var_l_wln_dn7_slot: &mut f64,
        var_l_wln_dn8_slot: &mut f64,
        var_l_wln_rdb0_slot: &mut f64,
        var_l_wln_rdb1_slot: &mut f64,
        var_l_wln_rdb2_slot: &mut f64,
        var_l_wln_rdb3_slot: &mut f64,
        var_l_wln_rdb4_slot: &mut f64,
        var_l_wln_rdn0_slot: &mut f64,
        var_l_wln_rdn1_slot: &mut f64,
        var_l_wln_rdn2_slot: &mut f64,
        var_l_wln_rdn3_slot: &mut f64,
        var_l_wln_rdn4_slot: &mut f64,
        var_l_wln_rdn5_slot: &mut f64,
        var_l_wln_rdn6_slot: &mut f64,
        var_l_wln_rdn7_slot: &mut f64,
        var_l_wln_rdn8_slot: &mut f64,
        var_l_wln_rv_slot: &mut f64,
        var_lnew_slot: &mut f64,
        var_lnew_db0_slot: &mut f64,
        var_lnew_db1_slot: &mut f64,
        var_lnew_db2_slot: &mut f64,
        var_lnew_db3_slot: &mut f64,
        var_lnew_db4_slot: &mut f64,
        var_lnew_dn0_slot: &mut f64,
        var_lnew_dn1_slot: &mut f64,
        var_lnew_dn2_slot: &mut f64,
        var_lnew_dn3_slot: &mut f64,
        var_lnew_dn4_slot: &mut f64,
        var_lnew_dn5_slot: &mut f64,
        var_lnew_dn6_slot: &mut f64,
        var_lnew_dn7_slot: &mut f64,
        var_lnew_dn8_slot: &mut f64,
        var_lnew_rdb0_slot: &mut f64,
        var_lnew_rdb1_slot: &mut f64,
        var_lnew_rdb2_slot: &mut f64,
        var_lnew_rdb3_slot: &mut f64,
        var_lnew_rdb4_slot: &mut f64,
        var_lnew_rdn0_slot: &mut f64,
        var_lnew_rdn1_slot: &mut f64,
        var_lnew_rdn2_slot: &mut f64,
        var_lnew_rdn3_slot: &mut f64,
        var_lnew_rdn4_slot: &mut f64,
        var_lnew_rdn5_slot: &mut f64,
        var_lnew_rdn6_slot: &mut f64,
        var_lnew_rdn7_slot: &mut f64,
        var_lnew_rdn8_slot: &mut f64,
        var_lnew_rv_slot: &mut f64,
        var_lw_lln_lwn_slot: &mut f64,
        var_lw_lln_lwn_db0_slot: &mut f64,
        var_lw_lln_lwn_db1_slot: &mut f64,
        var_lw_lln_lwn_db2_slot: &mut f64,
        var_lw_lln_lwn_db3_slot: &mut f64,
        var_lw_lln_lwn_db4_slot: &mut f64,
        var_lw_lln_lwn_dn0_slot: &mut f64,
        var_lw_lln_lwn_dn1_slot: &mut f64,
        var_lw_lln_lwn_dn2_slot: &mut f64,
        var_lw_lln_lwn_dn3_slot: &mut f64,
        var_lw_lln_lwn_dn4_slot: &mut f64,
        var_lw_lln_lwn_dn5_slot: &mut f64,
        var_lw_lln_lwn_dn6_slot: &mut f64,
        var_lw_lln_lwn_dn7_slot: &mut f64,
        var_lw_lln_lwn_dn8_slot: &mut f64,
        var_lw_lln_lwn_rdb0_slot: &mut f64,
        var_lw_lln_lwn_rdb1_slot: &mut f64,
        var_lw_lln_lwn_rdb2_slot: &mut f64,
        var_lw_lln_lwn_rdb3_slot: &mut f64,
        var_lw_lln_lwn_rdb4_slot: &mut f64,
        var_lw_lln_lwn_rdn0_slot: &mut f64,
        var_lw_lln_lwn_rdn1_slot: &mut f64,
        var_lw_lln_lwn_rdn2_slot: &mut f64,
        var_lw_lln_lwn_rdn3_slot: &mut f64,
        var_lw_lln_lwn_rdn4_slot: &mut f64,
        var_lw_lln_lwn_rdn5_slot: &mut f64,
        var_lw_lln_lwn_rdn6_slot: &mut f64,
        var_lw_lln_lwn_rdn7_slot: &mut f64,
        var_lw_lln_lwn_rdn8_slot: &mut f64,
        var_lw_lln_lwn_rv_slot: &mut f64,
        var_w_lwn_slot: &mut f64,
        var_w_lwn_db0_slot: &mut f64,
        var_w_lwn_db1_slot: &mut f64,
        var_w_lwn_db2_slot: &mut f64,
        var_w_lwn_db3_slot: &mut f64,
        var_w_lwn_db4_slot: &mut f64,
        var_w_lwn_dn0_slot: &mut f64,
        var_w_lwn_dn1_slot: &mut f64,
        var_w_lwn_dn2_slot: &mut f64,
        var_w_lwn_dn3_slot: &mut f64,
        var_w_lwn_dn4_slot: &mut f64,
        var_w_lwn_dn5_slot: &mut f64,
        var_w_lwn_dn6_slot: &mut f64,
        var_w_lwn_dn7_slot: &mut f64,
        var_w_lwn_dn8_slot: &mut f64,
        var_w_lwn_rdb0_slot: &mut f64,
        var_w_lwn_rdb1_slot: &mut f64,
        var_w_lwn_rdb2_slot: &mut f64,
        var_w_lwn_rdb3_slot: &mut f64,
        var_w_lwn_rdb4_slot: &mut f64,
        var_w_lwn_rdn0_slot: &mut f64,
        var_w_lwn_rdn1_slot: &mut f64,
        var_w_lwn_rdn2_slot: &mut f64,
        var_w_lwn_rdn3_slot: &mut f64,
        var_w_lwn_rdn4_slot: &mut f64,
        var_w_lwn_rdn5_slot: &mut f64,
        var_w_lwn_rdn6_slot: &mut f64,
        var_w_lwn_rdn7_slot: &mut f64,
        var_w_lwn_rdn8_slot: &mut f64,
        var_w_lwn_rv_slot: &mut f64,
        var_w_wwn_slot: &mut f64,
        var_w_wwn_db0_slot: &mut f64,
        var_w_wwn_db1_slot: &mut f64,
        var_w_wwn_db2_slot: &mut f64,
        var_w_wwn_db3_slot: &mut f64,
        var_w_wwn_db4_slot: &mut f64,
        var_w_wwn_dn0_slot: &mut f64,
        var_w_wwn_dn1_slot: &mut f64,
        var_w_wwn_dn2_slot: &mut f64,
        var_w_wwn_dn3_slot: &mut f64,
        var_w_wwn_dn4_slot: &mut f64,
        var_w_wwn_dn5_slot: &mut f64,
        var_w_wwn_dn6_slot: &mut f64,
        var_w_wwn_dn7_slot: &mut f64,
        var_w_wwn_dn8_slot: &mut f64,
        var_w_wwn_rdb0_slot: &mut f64,
        var_w_wwn_rdb1_slot: &mut f64,
        var_w_wwn_rdb2_slot: &mut f64,
        var_w_wwn_rdb3_slot: &mut f64,
        var_w_wwn_rdb4_slot: &mut f64,
        var_w_wwn_rdn0_slot: &mut f64,
        var_w_wwn_rdn1_slot: &mut f64,
        var_w_wwn_rdn2_slot: &mut f64,
        var_w_wwn_rdn3_slot: &mut f64,
        var_w_wwn_rdn4_slot: &mut f64,
        var_w_wwn_rdn5_slot: &mut f64,
        var_w_wwn_rdn6_slot: &mut f64,
        var_w_wwn_rdn7_slot: &mut f64,
        var_w_wwn_rdn8_slot: &mut f64,
        var_w_wwn_rv_slot: &mut f64,
        var_welsign_slot: &mut f64,
        var_welsign_rv_slot: &mut f64,
        var_wnew_slot: &mut f64,
        var_wnew_db0_slot: &mut f64,
        var_wnew_db1_slot: &mut f64,
        var_wnew_db2_slot: &mut f64,
        var_wnew_db3_slot: &mut f64,
        var_wnew_db4_slot: &mut f64,
        var_wnew_dn0_slot: &mut f64,
        var_wnew_dn1_slot: &mut f64,
        var_wnew_dn2_slot: &mut f64,
        var_wnew_dn3_slot: &mut f64,
        var_wnew_dn4_slot: &mut f64,
        var_wnew_dn5_slot: &mut f64,
        var_wnew_dn6_slot: &mut f64,
        var_wnew_dn7_slot: &mut f64,
        var_wnew_dn8_slot: &mut f64,
        var_wnew_rdb0_slot: &mut f64,
        var_wnew_rdb1_slot: &mut f64,
        var_wnew_rdb2_slot: &mut f64,
        var_wnew_rdb3_slot: &mut f64,
        var_wnew_rdb4_slot: &mut f64,
        var_wnew_rdn0_slot: &mut f64,
        var_wnew_rdn1_slot: &mut f64,
        var_wnew_rdn2_slot: &mut f64,
        var_wnew_rdn3_slot: &mut f64,
        var_wnew_rdn4_slot: &mut f64,
        var_wnew_rdn5_slot: &mut f64,
        var_wnew_rdn6_slot: &mut f64,
        var_wnew_rdn7_slot: &mut f64,
        var_wnew_rdn8_slot: &mut f64,
        var_wnew_rv_slot: &mut f64,
    ) {
        let mut var_devsign: f64 = *var_devsign_slot;
        let mut var_devsign_rv: f64 = *var_devsign_rv_slot;
        let mut var_dliv: f64 = *var_dliv_slot;
        let mut var_dliv_db0: f64 = *var_dliv_db0_slot;
        let mut var_dliv_db1: f64 = *var_dliv_db1_slot;
        let mut var_dliv_db2: f64 = *var_dliv_db2_slot;
        let mut var_dliv_db3: f64 = *var_dliv_db3_slot;
        let mut var_dliv_db4: f64 = *var_dliv_db4_slot;
        let mut var_dliv_dn0: f64 = *var_dliv_dn0_slot;
        let mut var_dliv_dn1: f64 = *var_dliv_dn1_slot;
        let mut var_dliv_dn2: f64 = *var_dliv_dn2_slot;
        let mut var_dliv_dn3: f64 = *var_dliv_dn3_slot;
        let mut var_dliv_dn4: f64 = *var_dliv_dn4_slot;
        let mut var_dliv_dn5: f64 = *var_dliv_dn5_slot;
        let mut var_dliv_dn6: f64 = *var_dliv_dn6_slot;
        let mut var_dliv_dn7: f64 = *var_dliv_dn7_slot;
        let mut var_dliv_dn8: f64 = *var_dliv_dn8_slot;
        let mut var_dliv_rdb0: f64 = *var_dliv_rdb0_slot;
        let mut var_dliv_rdb1: f64 = *var_dliv_rdb1_slot;
        let mut var_dliv_rdb2: f64 = *var_dliv_rdb2_slot;
        let mut var_dliv_rdb3: f64 = *var_dliv_rdb3_slot;
        let mut var_dliv_rdb4: f64 = *var_dliv_rdb4_slot;
        let mut var_dliv_rdn0: f64 = *var_dliv_rdn0_slot;
        let mut var_dliv_rdn1: f64 = *var_dliv_rdn1_slot;
        let mut var_dliv_rdn2: f64 = *var_dliv_rdn2_slot;
        let mut var_dliv_rdn3: f64 = *var_dliv_rdn3_slot;
        let mut var_dliv_rdn4: f64 = *var_dliv_rdn4_slot;
        let mut var_dliv_rdn5: f64 = *var_dliv_rdn5_slot;
        let mut var_dliv_rdn6: f64 = *var_dliv_rdn6_slot;
        let mut var_dliv_rdn7: f64 = *var_dliv_rdn7_slot;
        let mut var_dliv_rdn8: f64 = *var_dliv_rdn8_slot;
        let mut var_dliv_rv: f64 = *var_dliv_rv_slot;
        let mut var_epssi: f64 = *var_epssi_slot;
        let mut var_epssi_db0: f64 = *var_epssi_db0_slot;
        let mut var_epssi_db1: f64 = *var_epssi_db1_slot;
        let mut var_epssi_db2: f64 = *var_epssi_db2_slot;
        let mut var_epssi_db3: f64 = *var_epssi_db3_slot;
        let mut var_epssi_db4: f64 = *var_epssi_db4_slot;
        let mut var_epssi_dn0: f64 = *var_epssi_dn0_slot;
        let mut var_epssi_dn1: f64 = *var_epssi_dn1_slot;
        let mut var_epssi_dn2: f64 = *var_epssi_dn2_slot;
        let mut var_epssi_dn3: f64 = *var_epssi_dn3_slot;
        let mut var_epssi_dn4: f64 = *var_epssi_dn4_slot;
        let mut var_epssi_dn5: f64 = *var_epssi_dn5_slot;
        let mut var_epssi_dn6: f64 = *var_epssi_dn6_slot;
        let mut var_epssi_dn7: f64 = *var_epssi_dn7_slot;
        let mut var_epssi_dn8: f64 = *var_epssi_dn8_slot;
        let mut var_epssi_rdb0: f64 = *var_epssi_rdb0_slot;
        let mut var_epssi_rdb1: f64 = *var_epssi_rdb1_slot;
        let mut var_epssi_rdb2: f64 = *var_epssi_rdb2_slot;
        let mut var_epssi_rdb3: f64 = *var_epssi_rdb3_slot;
        let mut var_epssi_rdb4: f64 = *var_epssi_rdb4_slot;
        let mut var_epssi_rdn0: f64 = *var_epssi_rdn0_slot;
        let mut var_epssi_rdn1: f64 = *var_epssi_rdn1_slot;
        let mut var_epssi_rdn2: f64 = *var_epssi_rdn2_slot;
        let mut var_epssi_rdn3: f64 = *var_epssi_rdn3_slot;
        let mut var_epssi_rdn4: f64 = *var_epssi_rdn4_slot;
        let mut var_epssi_rdn5: f64 = *var_epssi_rdn5_slot;
        let mut var_epssi_rdn6: f64 = *var_epssi_rdn6_slot;
        let mut var_epssi_rdn7: f64 = *var_epssi_rdn7_slot;
        let mut var_epssi_rdn8: f64 = *var_epssi_rdn8_slot;
        let mut var_epssi_rv: f64 = *var_epssi_rv_slot;
        let mut var_guard3: f64 = *var_guard3_slot;
        let mut var_guard3_db0: f64 = *var_guard3_db0_slot;
        let mut var_guard3_db1: f64 = *var_guard3_db1_slot;
        let mut var_guard3_db2: f64 = *var_guard3_db2_slot;
        let mut var_guard3_db3: f64 = *var_guard3_db3_slot;
        let mut var_guard3_db4: f64 = *var_guard3_db4_slot;
        let mut var_guard3_dn0: f64 = *var_guard3_dn0_slot;
        let mut var_guard3_dn1: f64 = *var_guard3_dn1_slot;
        let mut var_guard3_dn2: f64 = *var_guard3_dn2_slot;
        let mut var_guard3_dn3: f64 = *var_guard3_dn3_slot;
        let mut var_guard3_dn4: f64 = *var_guard3_dn4_slot;
        let mut var_guard3_dn5: f64 = *var_guard3_dn5_slot;
        let mut var_guard3_dn6: f64 = *var_guard3_dn6_slot;
        let mut var_guard3_dn7: f64 = *var_guard3_dn7_slot;
        let mut var_guard3_dn8: f64 = *var_guard3_dn8_slot;
        let mut var_guard3_rdb0: f64 = *var_guard3_rdb0_slot;
        let mut var_guard3_rdb1: f64 = *var_guard3_rdb1_slot;
        let mut var_guard3_rdb2: f64 = *var_guard3_rdb2_slot;
        let mut var_guard3_rdb3: f64 = *var_guard3_rdb3_slot;
        let mut var_guard3_rdb4: f64 = *var_guard3_rdb4_slot;
        let mut var_guard3_rdn0: f64 = *var_guard3_rdn0_slot;
        let mut var_guard3_rdn1: f64 = *var_guard3_rdn1_slot;
        let mut var_guard3_rdn2: f64 = *var_guard3_rdn2_slot;
        let mut var_guard3_rdn3: f64 = *var_guard3_rdn3_slot;
        let mut var_guard3_rdn4: f64 = *var_guard3_rdn4_slot;
        let mut var_guard3_rdn5: f64 = *var_guard3_rdn5_slot;
        let mut var_guard3_rdn6: f64 = *var_guard3_rdn6_slot;
        let mut var_guard3_rdn7: f64 = *var_guard3_rdn7_slot;
        let mut var_guard3_rdn8: f64 = *var_guard3_rdn8_slot;
        let mut var_guard3_rv: f64 = *var_guard3_rv_slot;
        let mut var_guard4: f64 = *var_guard4_slot;
        let mut var_guard4_db0: f64 = *var_guard4_db0_slot;
        let mut var_guard4_db1: f64 = *var_guard4_db1_slot;
        let mut var_guard4_db2: f64 = *var_guard4_db2_slot;
        let mut var_guard4_db3: f64 = *var_guard4_db3_slot;
        let mut var_guard4_db4: f64 = *var_guard4_db4_slot;
        let mut var_guard4_dn0: f64 = *var_guard4_dn0_slot;
        let mut var_guard4_dn1: f64 = *var_guard4_dn1_slot;
        let mut var_guard4_dn2: f64 = *var_guard4_dn2_slot;
        let mut var_guard4_dn3: f64 = *var_guard4_dn3_slot;
        let mut var_guard4_dn4: f64 = *var_guard4_dn4_slot;
        let mut var_guard4_dn5: f64 = *var_guard4_dn5_slot;
        let mut var_guard4_dn6: f64 = *var_guard4_dn6_slot;
        let mut var_guard4_dn7: f64 = *var_guard4_dn7_slot;
        let mut var_guard4_dn8: f64 = *var_guard4_dn8_slot;
        let mut var_guard4_rdb0: f64 = *var_guard4_rdb0_slot;
        let mut var_guard4_rdb1: f64 = *var_guard4_rdb1_slot;
        let mut var_guard4_rdb2: f64 = *var_guard4_rdb2_slot;
        let mut var_guard4_rdb3: f64 = *var_guard4_rdb3_slot;
        let mut var_guard4_rdb4: f64 = *var_guard4_rdb4_slot;
        let mut var_guard4_rdn0: f64 = *var_guard4_rdn0_slot;
        let mut var_guard4_rdn1: f64 = *var_guard4_rdn1_slot;
        let mut var_guard4_rdn2: f64 = *var_guard4_rdn2_slot;
        let mut var_guard4_rdn3: f64 = *var_guard4_rdn3_slot;
        let mut var_guard4_rdn4: f64 = *var_guard4_rdn4_slot;
        let mut var_guard4_rdn5: f64 = *var_guard4_rdn5_slot;
        let mut var_guard4_rdn6: f64 = *var_guard4_rdn6_slot;
        let mut var_guard4_rdn7: f64 = *var_guard4_rdn7_slot;
        let mut var_guard4_rdn8: f64 = *var_guard4_rdn8_slot;
        let mut var_guard4_rv: f64 = *var_guard4_rv_slot;
        let mut var_guard5: f64 = *var_guard5_slot;
        let mut var_guard5_db0: f64 = *var_guard5_db0_slot;
        let mut var_guard5_db1: f64 = *var_guard5_db1_slot;
        let mut var_guard5_db2: f64 = *var_guard5_db2_slot;
        let mut var_guard5_db3: f64 = *var_guard5_db3_slot;
        let mut var_guard5_db4: f64 = *var_guard5_db4_slot;
        let mut var_guard5_dn0: f64 = *var_guard5_dn0_slot;
        let mut var_guard5_dn1: f64 = *var_guard5_dn1_slot;
        let mut var_guard5_dn2: f64 = *var_guard5_dn2_slot;
        let mut var_guard5_dn3: f64 = *var_guard5_dn3_slot;
        let mut var_guard5_dn4: f64 = *var_guard5_dn4_slot;
        let mut var_guard5_dn5: f64 = *var_guard5_dn5_slot;
        let mut var_guard5_dn6: f64 = *var_guard5_dn6_slot;
        let mut var_guard5_dn7: f64 = *var_guard5_dn7_slot;
        let mut var_guard5_dn8: f64 = *var_guard5_dn8_slot;
        let mut var_guard5_rdb0: f64 = *var_guard5_rdb0_slot;
        let mut var_guard5_rdb1: f64 = *var_guard5_rdb1_slot;
        let mut var_guard5_rdb2: f64 = *var_guard5_rdb2_slot;
        let mut var_guard5_rdb3: f64 = *var_guard5_rdb3_slot;
        let mut var_guard5_rdb4: f64 = *var_guard5_rdb4_slot;
        let mut var_guard5_rdn0: f64 = *var_guard5_rdn0_slot;
        let mut var_guard5_rdn1: f64 = *var_guard5_rdn1_slot;
        let mut var_guard5_rdn2: f64 = *var_guard5_rdn2_slot;
        let mut var_guard5_rdn3: f64 = *var_guard5_rdn3_slot;
        let mut var_guard5_rdn4: f64 = *var_guard5_rdn4_slot;
        let mut var_guard5_rdn5: f64 = *var_guard5_rdn5_slot;
        let mut var_guard5_rdn6: f64 = *var_guard5_rdn6_slot;
        let mut var_guard5_rdn7: f64 = *var_guard5_rdn7_slot;
        let mut var_guard5_rdn8: f64 = *var_guard5_rdn8_slot;
        let mut var_guard5_rv: f64 = *var_guard5_rv_slot;
        let mut var_l_lln: f64 = *var_l_lln_slot;
        let mut var_l_lln_db0: f64 = *var_l_lln_db0_slot;
        let mut var_l_lln_db1: f64 = *var_l_lln_db1_slot;
        let mut var_l_lln_db2: f64 = *var_l_lln_db2_slot;
        let mut var_l_lln_db3: f64 = *var_l_lln_db3_slot;
        let mut var_l_lln_db4: f64 = *var_l_lln_db4_slot;
        let mut var_l_lln_dn0: f64 = *var_l_lln_dn0_slot;
        let mut var_l_lln_dn1: f64 = *var_l_lln_dn1_slot;
        let mut var_l_lln_dn2: f64 = *var_l_lln_dn2_slot;
        let mut var_l_lln_dn3: f64 = *var_l_lln_dn3_slot;
        let mut var_l_lln_dn4: f64 = *var_l_lln_dn4_slot;
        let mut var_l_lln_dn5: f64 = *var_l_lln_dn5_slot;
        let mut var_l_lln_dn6: f64 = *var_l_lln_dn6_slot;
        let mut var_l_lln_dn7: f64 = *var_l_lln_dn7_slot;
        let mut var_l_lln_dn8: f64 = *var_l_lln_dn8_slot;
        let mut var_l_lln_rdb0: f64 = *var_l_lln_rdb0_slot;
        let mut var_l_lln_rdb1: f64 = *var_l_lln_rdb1_slot;
        let mut var_l_lln_rdb2: f64 = *var_l_lln_rdb2_slot;
        let mut var_l_lln_rdb3: f64 = *var_l_lln_rdb3_slot;
        let mut var_l_lln_rdb4: f64 = *var_l_lln_rdb4_slot;
        let mut var_l_lln_rdn0: f64 = *var_l_lln_rdn0_slot;
        let mut var_l_lln_rdn1: f64 = *var_l_lln_rdn1_slot;
        let mut var_l_lln_rdn2: f64 = *var_l_lln_rdn2_slot;
        let mut var_l_lln_rdn3: f64 = *var_l_lln_rdn3_slot;
        let mut var_l_lln_rdn4: f64 = *var_l_lln_rdn4_slot;
        let mut var_l_lln_rdn5: f64 = *var_l_lln_rdn5_slot;
        let mut var_l_lln_rdn6: f64 = *var_l_lln_rdn6_slot;
        let mut var_l_lln_rdn7: f64 = *var_l_lln_rdn7_slot;
        let mut var_l_lln_rdn8: f64 = *var_l_lln_rdn8_slot;
        let mut var_l_lln_rv: f64 = *var_l_lln_rv_slot;
        let mut var_l_wln: f64 = *var_l_wln_slot;
        let mut var_l_wln_db0: f64 = *var_l_wln_db0_slot;
        let mut var_l_wln_db1: f64 = *var_l_wln_db1_slot;
        let mut var_l_wln_db2: f64 = *var_l_wln_db2_slot;
        let mut var_l_wln_db3: f64 = *var_l_wln_db3_slot;
        let mut var_l_wln_db4: f64 = *var_l_wln_db4_slot;
        let mut var_l_wln_dn0: f64 = *var_l_wln_dn0_slot;
        let mut var_l_wln_dn1: f64 = *var_l_wln_dn1_slot;
        let mut var_l_wln_dn2: f64 = *var_l_wln_dn2_slot;
        let mut var_l_wln_dn3: f64 = *var_l_wln_dn3_slot;
        let mut var_l_wln_dn4: f64 = *var_l_wln_dn4_slot;
        let mut var_l_wln_dn5: f64 = *var_l_wln_dn5_slot;
        let mut var_l_wln_dn6: f64 = *var_l_wln_dn6_slot;
        let mut var_l_wln_dn7: f64 = *var_l_wln_dn7_slot;
        let mut var_l_wln_dn8: f64 = *var_l_wln_dn8_slot;
        let mut var_l_wln_rdb0: f64 = *var_l_wln_rdb0_slot;
        let mut var_l_wln_rdb1: f64 = *var_l_wln_rdb1_slot;
        let mut var_l_wln_rdb2: f64 = *var_l_wln_rdb2_slot;
        let mut var_l_wln_rdb3: f64 = *var_l_wln_rdb3_slot;
        let mut var_l_wln_rdb4: f64 = *var_l_wln_rdb4_slot;
        let mut var_l_wln_rdn0: f64 = *var_l_wln_rdn0_slot;
        let mut var_l_wln_rdn1: f64 = *var_l_wln_rdn1_slot;
        let mut var_l_wln_rdn2: f64 = *var_l_wln_rdn2_slot;
        let mut var_l_wln_rdn3: f64 = *var_l_wln_rdn3_slot;
        let mut var_l_wln_rdn4: f64 = *var_l_wln_rdn4_slot;
        let mut var_l_wln_rdn5: f64 = *var_l_wln_rdn5_slot;
        let mut var_l_wln_rdn6: f64 = *var_l_wln_rdn6_slot;
        let mut var_l_wln_rdn7: f64 = *var_l_wln_rdn7_slot;
        let mut var_l_wln_rdn8: f64 = *var_l_wln_rdn8_slot;
        let mut var_l_wln_rv: f64 = *var_l_wln_rv_slot;
        let mut var_lnew: f64 = *var_lnew_slot;
        let mut var_lnew_db0: f64 = *var_lnew_db0_slot;
        let mut var_lnew_db1: f64 = *var_lnew_db1_slot;
        let mut var_lnew_db2: f64 = *var_lnew_db2_slot;
        let mut var_lnew_db3: f64 = *var_lnew_db3_slot;
        let mut var_lnew_db4: f64 = *var_lnew_db4_slot;
        let mut var_lnew_dn0: f64 = *var_lnew_dn0_slot;
        let mut var_lnew_dn1: f64 = *var_lnew_dn1_slot;
        let mut var_lnew_dn2: f64 = *var_lnew_dn2_slot;
        let mut var_lnew_dn3: f64 = *var_lnew_dn3_slot;
        let mut var_lnew_dn4: f64 = *var_lnew_dn4_slot;
        let mut var_lnew_dn5: f64 = *var_lnew_dn5_slot;
        let mut var_lnew_dn6: f64 = *var_lnew_dn6_slot;
        let mut var_lnew_dn7: f64 = *var_lnew_dn7_slot;
        let mut var_lnew_dn8: f64 = *var_lnew_dn8_slot;
        let mut var_lnew_rdb0: f64 = *var_lnew_rdb0_slot;
        let mut var_lnew_rdb1: f64 = *var_lnew_rdb1_slot;
        let mut var_lnew_rdb2: f64 = *var_lnew_rdb2_slot;
        let mut var_lnew_rdb3: f64 = *var_lnew_rdb3_slot;
        let mut var_lnew_rdb4: f64 = *var_lnew_rdb4_slot;
        let mut var_lnew_rdn0: f64 = *var_lnew_rdn0_slot;
        let mut var_lnew_rdn1: f64 = *var_lnew_rdn1_slot;
        let mut var_lnew_rdn2: f64 = *var_lnew_rdn2_slot;
        let mut var_lnew_rdn3: f64 = *var_lnew_rdn3_slot;
        let mut var_lnew_rdn4: f64 = *var_lnew_rdn4_slot;
        let mut var_lnew_rdn5: f64 = *var_lnew_rdn5_slot;
        let mut var_lnew_rdn6: f64 = *var_lnew_rdn6_slot;
        let mut var_lnew_rdn7: f64 = *var_lnew_rdn7_slot;
        let mut var_lnew_rdn8: f64 = *var_lnew_rdn8_slot;
        let mut var_lnew_rv: f64 = *var_lnew_rv_slot;
        let mut var_lw_lln_lwn: f64 = *var_lw_lln_lwn_slot;
        let mut var_lw_lln_lwn_db0: f64 = *var_lw_lln_lwn_db0_slot;
        let mut var_lw_lln_lwn_db1: f64 = *var_lw_lln_lwn_db1_slot;
        let mut var_lw_lln_lwn_db2: f64 = *var_lw_lln_lwn_db2_slot;
        let mut var_lw_lln_lwn_db3: f64 = *var_lw_lln_lwn_db3_slot;
        let mut var_lw_lln_lwn_db4: f64 = *var_lw_lln_lwn_db4_slot;
        let mut var_lw_lln_lwn_dn0: f64 = *var_lw_lln_lwn_dn0_slot;
        let mut var_lw_lln_lwn_dn1: f64 = *var_lw_lln_lwn_dn1_slot;
        let mut var_lw_lln_lwn_dn2: f64 = *var_lw_lln_lwn_dn2_slot;
        let mut var_lw_lln_lwn_dn3: f64 = *var_lw_lln_lwn_dn3_slot;
        let mut var_lw_lln_lwn_dn4: f64 = *var_lw_lln_lwn_dn4_slot;
        let mut var_lw_lln_lwn_dn5: f64 = *var_lw_lln_lwn_dn5_slot;
        let mut var_lw_lln_lwn_dn6: f64 = *var_lw_lln_lwn_dn6_slot;
        let mut var_lw_lln_lwn_dn7: f64 = *var_lw_lln_lwn_dn7_slot;
        let mut var_lw_lln_lwn_dn8: f64 = *var_lw_lln_lwn_dn8_slot;
        let mut var_lw_lln_lwn_rdb0: f64 = *var_lw_lln_lwn_rdb0_slot;
        let mut var_lw_lln_lwn_rdb1: f64 = *var_lw_lln_lwn_rdb1_slot;
        let mut var_lw_lln_lwn_rdb2: f64 = *var_lw_lln_lwn_rdb2_slot;
        let mut var_lw_lln_lwn_rdb3: f64 = *var_lw_lln_lwn_rdb3_slot;
        let mut var_lw_lln_lwn_rdb4: f64 = *var_lw_lln_lwn_rdb4_slot;
        let mut var_lw_lln_lwn_rdn0: f64 = *var_lw_lln_lwn_rdn0_slot;
        let mut var_lw_lln_lwn_rdn1: f64 = *var_lw_lln_lwn_rdn1_slot;
        let mut var_lw_lln_lwn_rdn2: f64 = *var_lw_lln_lwn_rdn2_slot;
        let mut var_lw_lln_lwn_rdn3: f64 = *var_lw_lln_lwn_rdn3_slot;
        let mut var_lw_lln_lwn_rdn4: f64 = *var_lw_lln_lwn_rdn4_slot;
        let mut var_lw_lln_lwn_rdn5: f64 = *var_lw_lln_lwn_rdn5_slot;
        let mut var_lw_lln_lwn_rdn6: f64 = *var_lw_lln_lwn_rdn6_slot;
        let mut var_lw_lln_lwn_rdn7: f64 = *var_lw_lln_lwn_rdn7_slot;
        let mut var_lw_lln_lwn_rdn8: f64 = *var_lw_lln_lwn_rdn8_slot;
        let mut var_lw_lln_lwn_rv: f64 = *var_lw_lln_lwn_rv_slot;
        let mut var_w_lwn: f64 = *var_w_lwn_slot;
        let mut var_w_lwn_db0: f64 = *var_w_lwn_db0_slot;
        let mut var_w_lwn_db1: f64 = *var_w_lwn_db1_slot;
        let mut var_w_lwn_db2: f64 = *var_w_lwn_db2_slot;
        let mut var_w_lwn_db3: f64 = *var_w_lwn_db3_slot;
        let mut var_w_lwn_db4: f64 = *var_w_lwn_db4_slot;
        let mut var_w_lwn_dn0: f64 = *var_w_lwn_dn0_slot;
        let mut var_w_lwn_dn1: f64 = *var_w_lwn_dn1_slot;
        let mut var_w_lwn_dn2: f64 = *var_w_lwn_dn2_slot;
        let mut var_w_lwn_dn3: f64 = *var_w_lwn_dn3_slot;
        let mut var_w_lwn_dn4: f64 = *var_w_lwn_dn4_slot;
        let mut var_w_lwn_dn5: f64 = *var_w_lwn_dn5_slot;
        let mut var_w_lwn_dn6: f64 = *var_w_lwn_dn6_slot;
        let mut var_w_lwn_dn7: f64 = *var_w_lwn_dn7_slot;
        let mut var_w_lwn_dn8: f64 = *var_w_lwn_dn8_slot;
        let mut var_w_lwn_rdb0: f64 = *var_w_lwn_rdb0_slot;
        let mut var_w_lwn_rdb1: f64 = *var_w_lwn_rdb1_slot;
        let mut var_w_lwn_rdb2: f64 = *var_w_lwn_rdb2_slot;
        let mut var_w_lwn_rdb3: f64 = *var_w_lwn_rdb3_slot;
        let mut var_w_lwn_rdb4: f64 = *var_w_lwn_rdb4_slot;
        let mut var_w_lwn_rdn0: f64 = *var_w_lwn_rdn0_slot;
        let mut var_w_lwn_rdn1: f64 = *var_w_lwn_rdn1_slot;
        let mut var_w_lwn_rdn2: f64 = *var_w_lwn_rdn2_slot;
        let mut var_w_lwn_rdn3: f64 = *var_w_lwn_rdn3_slot;
        let mut var_w_lwn_rdn4: f64 = *var_w_lwn_rdn4_slot;
        let mut var_w_lwn_rdn5: f64 = *var_w_lwn_rdn5_slot;
        let mut var_w_lwn_rdn6: f64 = *var_w_lwn_rdn6_slot;
        let mut var_w_lwn_rdn7: f64 = *var_w_lwn_rdn7_slot;
        let mut var_w_lwn_rdn8: f64 = *var_w_lwn_rdn8_slot;
        let mut var_w_lwn_rv: f64 = *var_w_lwn_rv_slot;
        let mut var_w_wwn: f64 = *var_w_wwn_slot;
        let mut var_w_wwn_db0: f64 = *var_w_wwn_db0_slot;
        let mut var_w_wwn_db1: f64 = *var_w_wwn_db1_slot;
        let mut var_w_wwn_db2: f64 = *var_w_wwn_db2_slot;
        let mut var_w_wwn_db3: f64 = *var_w_wwn_db3_slot;
        let mut var_w_wwn_db4: f64 = *var_w_wwn_db4_slot;
        let mut var_w_wwn_dn0: f64 = *var_w_wwn_dn0_slot;
        let mut var_w_wwn_dn1: f64 = *var_w_wwn_dn1_slot;
        let mut var_w_wwn_dn2: f64 = *var_w_wwn_dn2_slot;
        let mut var_w_wwn_dn3: f64 = *var_w_wwn_dn3_slot;
        let mut var_w_wwn_dn4: f64 = *var_w_wwn_dn4_slot;
        let mut var_w_wwn_dn5: f64 = *var_w_wwn_dn5_slot;
        let mut var_w_wwn_dn6: f64 = *var_w_wwn_dn6_slot;
        let mut var_w_wwn_dn7: f64 = *var_w_wwn_dn7_slot;
        let mut var_w_wwn_dn8: f64 = *var_w_wwn_dn8_slot;
        let mut var_w_wwn_rdb0: f64 = *var_w_wwn_rdb0_slot;
        let mut var_w_wwn_rdb1: f64 = *var_w_wwn_rdb1_slot;
        let mut var_w_wwn_rdb2: f64 = *var_w_wwn_rdb2_slot;
        let mut var_w_wwn_rdb3: f64 = *var_w_wwn_rdb3_slot;
        let mut var_w_wwn_rdb4: f64 = *var_w_wwn_rdb4_slot;
        let mut var_w_wwn_rdn0: f64 = *var_w_wwn_rdn0_slot;
        let mut var_w_wwn_rdn1: f64 = *var_w_wwn_rdn1_slot;
        let mut var_w_wwn_rdn2: f64 = *var_w_wwn_rdn2_slot;
        let mut var_w_wwn_rdn3: f64 = *var_w_wwn_rdn3_slot;
        let mut var_w_wwn_rdn4: f64 = *var_w_wwn_rdn4_slot;
        let mut var_w_wwn_rdn5: f64 = *var_w_wwn_rdn5_slot;
        let mut var_w_wwn_rdn6: f64 = *var_w_wwn_rdn6_slot;
        let mut var_w_wwn_rdn7: f64 = *var_w_wwn_rdn7_slot;
        let mut var_w_wwn_rdn8: f64 = *var_w_wwn_rdn8_slot;
        let mut var_w_wwn_rv: f64 = *var_w_wwn_rv_slot;
        let mut var_welsign: f64 = *var_welsign_slot;
        let mut var_welsign_rv: f64 = *var_welsign_rv_slot;
        let mut var_wnew: f64 = *var_wnew_slot;
        let mut var_wnew_db0: f64 = *var_wnew_db0_slot;
        let mut var_wnew_db1: f64 = *var_wnew_db1_slot;
        let mut var_wnew_db2: f64 = *var_wnew_db2_slot;
        let mut var_wnew_db3: f64 = *var_wnew_db3_slot;
        let mut var_wnew_db4: f64 = *var_wnew_db4_slot;
        let mut var_wnew_dn0: f64 = *var_wnew_dn0_slot;
        let mut var_wnew_dn1: f64 = *var_wnew_dn1_slot;
        let mut var_wnew_dn2: f64 = *var_wnew_dn2_slot;
        let mut var_wnew_dn3: f64 = *var_wnew_dn3_slot;
        let mut var_wnew_dn4: f64 = *var_wnew_dn4_slot;
        let mut var_wnew_dn5: f64 = *var_wnew_dn5_slot;
        let mut var_wnew_dn6: f64 = *var_wnew_dn6_slot;
        let mut var_wnew_dn7: f64 = *var_wnew_dn7_slot;
        let mut var_wnew_dn8: f64 = *var_wnew_dn8_slot;
        let mut var_wnew_rdb0: f64 = *var_wnew_rdb0_slot;
        let mut var_wnew_rdb1: f64 = *var_wnew_rdb1_slot;
        let mut var_wnew_rdb2: f64 = *var_wnew_rdb2_slot;
        let mut var_wnew_rdb3: f64 = *var_wnew_rdb3_slot;
        let mut var_wnew_rdb4: f64 = *var_wnew_rdb4_slot;
        let mut var_wnew_rdn0: f64 = *var_wnew_rdn0_slot;
        let mut var_wnew_rdn1: f64 = *var_wnew_rdn1_slot;
        let mut var_wnew_rdn2: f64 = *var_wnew_rdn2_slot;
        let mut var_wnew_rdn3: f64 = *var_wnew_rdn3_slot;
        let mut var_wnew_rdn4: f64 = *var_wnew_rdn4_slot;
        let mut var_wnew_rdn5: f64 = *var_wnew_rdn5_slot;
        let mut var_wnew_rdn6: f64 = *var_wnew_rdn6_slot;
        let mut var_wnew_rdn7: f64 = *var_wnew_rdn7_slot;
        let mut var_wnew_rdn8: f64 = *var_wnew_rdn8_slot;
        let mut var_wnew_rv: f64 = *var_wnew_rv_slot;

        let assign70_e1130: f64 = if p.p12 == 1.0 { 1.0 } else { 0.0 };
        var_guard3 = assign70_e1130;
        var_guard3_dn0 = 0.0;
        var_guard3_dn1 = 0.0;
        var_guard3_dn2 = 0.0;
        var_guard3_dn3 = 0.0;
        var_guard3_dn4 = 0.0;
        var_guard3_dn5 = 0.0;
        var_guard3_dn6 = 0.0;
        var_guard3_dn7 = 0.0;
        var_guard3_dn8 = 0.0;
        var_guard3_db0 = 0.0;
        var_guard3_db1 = 0.0;
        var_guard3_db2 = 0.0;
        var_guard3_db3 = 0.0;
        var_guard3_db4 = 0.0;
        var_guard3_rv = 0.0;
        var_guard3_rdn0 = 0.0;
        var_guard3_rdn1 = 0.0;
        var_guard3_rdn2 = 0.0;
        var_guard3_rdn3 = 0.0;
        var_guard3_rdn4 = 0.0;
        var_guard3_rdn5 = 0.0;
        var_guard3_rdn6 = 0.0;
        var_guard3_rdn7 = 0.0;
        var_guard3_rdn8 = 0.0;
        var_guard3_rdb0 = 0.0;
        var_guard3_rdb1 = 0.0;
        var_guard3_rdb2 = 0.0;
        var_guard3_rdb3 = 0.0;
        var_guard3_rdb4 = 0.0;

        let (assign80_e1134,) = {
    if (var_guard3 != 0.0) {
        (1.0,)
    } else {
        (var_devsign,)
    }
};
        var_devsign = assign80_e1134;
        var_devsign_rv = 0.0;

        let (assign90_e1140,) = {
    if (var_guard3 == 0.0) {
        let assign90_e1138: f64 = (-1.0);
        (assign90_e1138,)
    } else {
        (var_devsign,)
    }
};
        var_devsign = assign90_e1140;
        var_devsign_rv = 0.0;

        let assign100_e1143: f64 = if p.p13 == 1.0 { 1.0 } else { 0.0 };
        var_guard4 = assign100_e1143;
        var_guard4_dn0 = 0.0;
        var_guard4_dn1 = 0.0;
        var_guard4_dn2 = 0.0;
        var_guard4_dn3 = 0.0;
        var_guard4_dn4 = 0.0;
        var_guard4_dn5 = 0.0;
        var_guard4_dn6 = 0.0;
        var_guard4_dn7 = 0.0;
        var_guard4_dn8 = 0.0;
        var_guard4_db0 = 0.0;
        var_guard4_db1 = 0.0;
        var_guard4_db2 = 0.0;
        var_guard4_db3 = 0.0;
        var_guard4_db4 = 0.0;
        var_guard4_rv = 0.0;
        var_guard4_rdn0 = 0.0;
        var_guard4_rdn1 = 0.0;
        var_guard4_rdn2 = 0.0;
        var_guard4_rdn3 = 0.0;
        var_guard4_rdn4 = 0.0;
        var_guard4_rdn5 = 0.0;
        var_guard4_rdn6 = 0.0;
        var_guard4_rdn7 = 0.0;
        var_guard4_rdn8 = 0.0;
        var_guard4_rdb0 = 0.0;
        var_guard4_rdb1 = 0.0;
        var_guard4_rdb2 = 0.0;
        var_guard4_rdb3 = 0.0;
        var_guard4_rdb4 = 0.0;

        let (assign110_e1147,) = {
    if (var_guard4 != 0.0) {
        (1.0,)
    } else {
        (var_welsign,)
    }
};
        var_welsign = assign110_e1147;
        var_welsign_rv = 0.0;

        let (assign120_e1153,) = {
    if (var_guard4 == 0.0) {
        let assign120_e1151: f64 = (-1.0);
        (assign120_e1151,)
    } else {
        (var_welsign,)
    }
};
        var_welsign = assign120_e1153;
        var_welsign_rv = 0.0;

        let assign130_e1156: f64 = (p.p59 * 8.85418e-12);
        var_epssi = assign130_e1156;
        var_epssi_dn0 = 0.0;
        var_epssi_dn1 = 0.0;
        var_epssi_dn2 = 0.0;
        var_epssi_dn3 = 0.0;
        var_epssi_dn4 = 0.0;
        var_epssi_dn5 = 0.0;
        var_epssi_dn6 = 0.0;
        var_epssi_dn7 = 0.0;
        var_epssi_dn8 = 0.0;
        var_epssi_db0 = 0.0;
        var_epssi_db1 = 0.0;
        var_epssi_db2 = 0.0;
        var_epssi_db3 = 0.0;
        var_epssi_db4 = 0.0;
        var_epssi_rv = 0.0;
        var_epssi_rdn0 = 0.0;
        var_epssi_rdn1 = 0.0;
        var_epssi_rdn2 = 0.0;
        var_epssi_rdn3 = 0.0;
        var_epssi_rdn4 = 0.0;
        var_epssi_rdn5 = 0.0;
        var_epssi_rdn6 = 0.0;
        var_epssi_rdn7 = 0.0;
        var_epssi_rdn8 = 0.0;
        var_epssi_rdb0 = 0.0;
        var_epssi_rdb1 = 0.0;
        var_epssi_rdb2 = 0.0;
        var_epssi_rdb3 = 0.0;
        var_epssi_rdb4 = 0.0;

        let assign140_e1159: f64 = if p.p21 == 0.0 { 1.0 } else { 0.0 };
        var_guard5 = assign140_e1159;
        var_guard5_dn0 = 0.0;
        var_guard5_dn1 = 0.0;
        var_guard5_dn2 = 0.0;
        var_guard5_dn3 = 0.0;
        var_guard5_dn4 = 0.0;
        var_guard5_dn5 = 0.0;
        var_guard5_dn6 = 0.0;
        var_guard5_dn7 = 0.0;
        var_guard5_dn8 = 0.0;
        var_guard5_db0 = 0.0;
        var_guard5_db1 = 0.0;
        var_guard5_db2 = 0.0;
        var_guard5_db3 = 0.0;
        var_guard5_db4 = 0.0;
        var_guard5_rv = 0.0;
        var_guard5_rdn0 = 0.0;
        var_guard5_rdn1 = 0.0;
        var_guard5_rdn2 = 0.0;
        var_guard5_rdn3 = 0.0;
        var_guard5_rdn4 = 0.0;
        var_guard5_rdn5 = 0.0;
        var_guard5_rdn6 = 0.0;
        var_guard5_rdn7 = 0.0;
        var_guard5_rdn8 = 0.0;
        var_guard5_rdb0 = 0.0;
        var_guard5_rdb1 = 0.0;
        var_guard5_rdb2 = 0.0;
        var_guard5_rdb3 = 0.0;
        var_guard5_rdb4 = 0.0;

        let (assign150_e1165, assign150_e1165_d_n0, assign150_e1165_d_n1, assign150_e1165_d_n2, assign150_e1165_d_n3, assign150_e1165_d_n4, assign150_e1165_d_n5, assign150_e1165_d_n6, assign150_e1165_d_n7, assign150_e1165_d_n8, assign150_e1165_d_b0, assign150_e1165_d_b1, assign150_e1165_d_b2, assign150_e1165_d_b3, assign150_e1165_d_b4,) = {
    if (var_guard5 != 0.0) {
        let assign150_e1163: f64 = (p.p1 / p.p2);
        (assign150_e1163, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_wnew, var_wnew_dn0, var_wnew_dn1, var_wnew_dn2, var_wnew_dn3, var_wnew_dn4, var_wnew_dn5, var_wnew_dn6, var_wnew_dn7, var_wnew_dn8, var_wnew_db0, var_wnew_db1, var_wnew_db2, var_wnew_db3, var_wnew_db4,)
    }
};
        var_wnew = assign150_e1165;
        var_wnew_dn0 = assign150_e1165_d_n0;
        var_wnew_dn1 = assign150_e1165_d_n1;
        var_wnew_dn2 = assign150_e1165_d_n2;
        var_wnew_dn3 = assign150_e1165_d_n3;
        var_wnew_dn4 = assign150_e1165_d_n4;
        var_wnew_dn5 = assign150_e1165_d_n5;
        var_wnew_dn6 = assign150_e1165_d_n6;
        var_wnew_dn7 = assign150_e1165_d_n7;
        var_wnew_dn8 = assign150_e1165_d_n8;
        var_wnew_db0 = assign150_e1165_d_b0;
        var_wnew_db1 = assign150_e1165_d_b1;
        var_wnew_db2 = assign150_e1165_d_b2;
        var_wnew_db3 = assign150_e1165_d_b3;
        var_wnew_db4 = assign150_e1165_d_b4;
        var_wnew_rv = 0.0;
        var_wnew_rdn0 = 0.0;
        var_wnew_rdn1 = 0.0;
        var_wnew_rdn2 = 0.0;
        var_wnew_rdn3 = 0.0;
        var_wnew_rdn4 = 0.0;
        var_wnew_rdn5 = 0.0;
        var_wnew_rdn6 = 0.0;
        var_wnew_rdn7 = 0.0;
        var_wnew_rdn8 = 0.0;
        var_wnew_rdb0 = 0.0;
        var_wnew_rdb1 = 0.0;
        var_wnew_rdb2 = 0.0;
        var_wnew_rdb3 = 0.0;
        var_wnew_rdb4 = 0.0;

        let (assign160_e1170, assign160_e1170_d_n0, assign160_e1170_d_n1, assign160_e1170_d_n2, assign160_e1170_d_n3, assign160_e1170_d_n4, assign160_e1170_d_n5, assign160_e1170_d_n6, assign160_e1170_d_n7, assign160_e1170_d_n8, assign160_e1170_d_b0, assign160_e1170_d_b1, assign160_e1170_d_b2, assign160_e1170_d_b3, assign160_e1170_d_b4,) = {
    if (var_guard5 == 0.0) {
        (p.p1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_wnew, var_wnew_dn0, var_wnew_dn1, var_wnew_dn2, var_wnew_dn3, var_wnew_dn4, var_wnew_dn5, var_wnew_dn6, var_wnew_dn7, var_wnew_dn8, var_wnew_db0, var_wnew_db1, var_wnew_db2, var_wnew_db3, var_wnew_db4,)
    }
};
        var_wnew = assign160_e1170;
        var_wnew_dn0 = assign160_e1170_d_n0;
        var_wnew_dn1 = assign160_e1170_d_n1;
        var_wnew_dn2 = assign160_e1170_d_n2;
        var_wnew_dn3 = assign160_e1170_d_n3;
        var_wnew_dn4 = assign160_e1170_d_n4;
        var_wnew_dn5 = assign160_e1170_d_n5;
        var_wnew_dn6 = assign160_e1170_d_n6;
        var_wnew_dn7 = assign160_e1170_d_n7;
        var_wnew_dn8 = assign160_e1170_d_n8;
        var_wnew_db0 = assign160_e1170_d_b0;
        var_wnew_db1 = assign160_e1170_d_b1;
        var_wnew_db2 = assign160_e1170_d_b2;
        var_wnew_db3 = assign160_e1170_d_b3;
        var_wnew_db4 = assign160_e1170_d_b4;
        var_wnew_rv = 0.0;
        var_wnew_rdn0 = 0.0;
        var_wnew_rdn1 = 0.0;
        var_wnew_rdn2 = 0.0;
        var_wnew_rdn3 = 0.0;
        var_wnew_rdn4 = 0.0;
        var_wnew_rdn5 = 0.0;
        var_wnew_rdn6 = 0.0;
        var_wnew_rdn7 = 0.0;
        var_wnew_rdn8 = 0.0;
        var_wnew_rdb0 = 0.0;
        var_wnew_rdb1 = 0.0;
        var_wnew_rdb2 = 0.0;
        var_wnew_rdb3 = 0.0;
        var_wnew_rdb4 = 0.0;

        let assign170_e1173: f64 = (p.p0 + p.p23);
        var_lnew = assign170_e1173;
        var_lnew_dn0 = 0.0;
        var_lnew_dn1 = 0.0;
        var_lnew_dn2 = 0.0;
        var_lnew_dn3 = 0.0;
        var_lnew_dn4 = 0.0;
        var_lnew_dn5 = 0.0;
        var_lnew_dn6 = 0.0;
        var_lnew_dn7 = 0.0;
        var_lnew_dn8 = 0.0;
        var_lnew_db0 = 0.0;
        var_lnew_db1 = 0.0;
        var_lnew_db2 = 0.0;
        var_lnew_db3 = 0.0;
        var_lnew_db4 = 0.0;
        var_lnew_rv = 0.0;
        var_lnew_rdn0 = 0.0;
        var_lnew_rdn1 = 0.0;
        var_lnew_rdn2 = 0.0;
        var_lnew_rdn3 = 0.0;
        var_lnew_rdn4 = 0.0;
        var_lnew_rdn5 = 0.0;
        var_lnew_rdn6 = 0.0;
        var_lnew_rdn7 = 0.0;
        var_lnew_rdn8 = 0.0;
        var_lnew_rdb0 = 0.0;
        var_lnew_rdb1 = 0.0;
        var_lnew_rdb2 = 0.0;
        var_lnew_rdb3 = 0.0;
        var_lnew_rdb4 = 0.0;

        let assign180_e1176: f64 = (var_wnew + p.p24);
        var_wnew = assign180_e1176;
        var_wnew_dn0 = var_wnew_dn0;
        var_wnew_dn1 = var_wnew_dn1;
        var_wnew_dn2 = var_wnew_dn2;
        var_wnew_dn3 = var_wnew_dn3;
        var_wnew_dn4 = var_wnew_dn4;
        var_wnew_dn5 = var_wnew_dn5;
        var_wnew_dn6 = var_wnew_dn6;
        var_wnew_dn7 = var_wnew_dn7;
        var_wnew_dn8 = var_wnew_dn8;
        var_wnew_db0 = var_wnew_db0;
        var_wnew_db1 = var_wnew_db1;
        var_wnew_db2 = var_wnew_db2;
        var_wnew_db3 = var_wnew_db3;
        var_wnew_db4 = var_wnew_db4;
        var_wnew_rv = 0.0;
        var_wnew_rdn0 = 0.0;
        var_wnew_rdn1 = 0.0;
        var_wnew_rdn2 = 0.0;
        var_wnew_rdn3 = 0.0;
        var_wnew_rdn4 = 0.0;
        var_wnew_rdn5 = 0.0;
        var_wnew_rdn6 = 0.0;
        var_wnew_rdn7 = 0.0;
        var_wnew_rdn8 = 0.0;
        var_wnew_rdb0 = 0.0;
        var_wnew_rdb1 = 0.0;
        var_wnew_rdb2 = 0.0;
        var_wnew_rdb3 = 0.0;
        var_wnew_rdb4 = 0.0;

        let assign190_e1179: f64 = (-p.p29);
        let assign190_e1180: f64 = (var_lnew).powf(assign190_e1179);
        var_l_lln = assign190_e1180;
        var_l_lln_dn0 = if 0.0 == 0.0 && ((assign190_e1179) as f64).is_finite() && ((assign190_e1179) as f64).fract() == 0.0 { if assign190_e1179 == 0.0 { 0.0 } else { (assign190_e1179 * ((var_lnew).powf(assign190_e1179 - 1.0) * var_lnew_dn0)) } } else { (assign190_e1180 * (assign190_e1179 * (var_lnew_dn0 / var_lnew))) };
        var_l_lln_dn1 = if 0.0 == 0.0 && ((assign190_e1179) as f64).is_finite() && ((assign190_e1179) as f64).fract() == 0.0 { if assign190_e1179 == 0.0 { 0.0 } else { (assign190_e1179 * ((var_lnew).powf(assign190_e1179 - 1.0) * var_lnew_dn1)) } } else { (assign190_e1180 * (assign190_e1179 * (var_lnew_dn1 / var_lnew))) };
        var_l_lln_dn2 = if 0.0 == 0.0 && ((assign190_e1179) as f64).is_finite() && ((assign190_e1179) as f64).fract() == 0.0 { if assign190_e1179 == 0.0 { 0.0 } else { (assign190_e1179 * ((var_lnew).powf(assign190_e1179 - 1.0) * var_lnew_dn2)) } } else { (assign190_e1180 * (assign190_e1179 * (var_lnew_dn2 / var_lnew))) };
        var_l_lln_dn3 = if 0.0 == 0.0 && ((assign190_e1179) as f64).is_finite() && ((assign190_e1179) as f64).fract() == 0.0 { if assign190_e1179 == 0.0 { 0.0 } else { (assign190_e1179 * ((var_lnew).powf(assign190_e1179 - 1.0) * var_lnew_dn3)) } } else { (assign190_e1180 * (assign190_e1179 * (var_lnew_dn3 / var_lnew))) };
        var_l_lln_dn4 = if 0.0 == 0.0 && ((assign190_e1179) as f64).is_finite() && ((assign190_e1179) as f64).fract() == 0.0 { if assign190_e1179 == 0.0 { 0.0 } else { (assign190_e1179 * ((var_lnew).powf(assign190_e1179 - 1.0) * var_lnew_dn4)) } } else { (assign190_e1180 * (assign190_e1179 * (var_lnew_dn4 / var_lnew))) };
        var_l_lln_dn5 = if 0.0 == 0.0 && ((assign190_e1179) as f64).is_finite() && ((assign190_e1179) as f64).fract() == 0.0 { if assign190_e1179 == 0.0 { 0.0 } else { (assign190_e1179 * ((var_lnew).powf(assign190_e1179 - 1.0) * var_lnew_dn5)) } } else { (assign190_e1180 * (assign190_e1179 * (var_lnew_dn5 / var_lnew))) };
        var_l_lln_dn6 = if 0.0 == 0.0 && ((assign190_e1179) as f64).is_finite() && ((assign190_e1179) as f64).fract() == 0.0 { if assign190_e1179 == 0.0 { 0.0 } else { (assign190_e1179 * ((var_lnew).powf(assign190_e1179 - 1.0) * var_lnew_dn6)) } } else { (assign190_e1180 * (assign190_e1179 * (var_lnew_dn6 / var_lnew))) };
        var_l_lln_dn7 = if 0.0 == 0.0 && ((assign190_e1179) as f64).is_finite() && ((assign190_e1179) as f64).fract() == 0.0 { if assign190_e1179 == 0.0 { 0.0 } else { (assign190_e1179 * ((var_lnew).powf(assign190_e1179 - 1.0) * var_lnew_dn7)) } } else { (assign190_e1180 * (assign190_e1179 * (var_lnew_dn7 / var_lnew))) };
        var_l_lln_dn8 = if 0.0 == 0.0 && ((assign190_e1179) as f64).is_finite() && ((assign190_e1179) as f64).fract() == 0.0 { if assign190_e1179 == 0.0 { 0.0 } else { (assign190_e1179 * ((var_lnew).powf(assign190_e1179 - 1.0) * var_lnew_dn8)) } } else { (assign190_e1180 * (assign190_e1179 * (var_lnew_dn8 / var_lnew))) };
        var_l_lln_db0 = if 0.0 == 0.0 && ((assign190_e1179) as f64).is_finite() && ((assign190_e1179) as f64).fract() == 0.0 { if assign190_e1179 == 0.0 { 0.0 } else { (assign190_e1179 * ((var_lnew).powf(assign190_e1179 - 1.0) * var_lnew_db0)) } } else { (assign190_e1180 * (assign190_e1179 * (var_lnew_db0 / var_lnew))) };
        var_l_lln_db1 = if 0.0 == 0.0 && ((assign190_e1179) as f64).is_finite() && ((assign190_e1179) as f64).fract() == 0.0 { if assign190_e1179 == 0.0 { 0.0 } else { (assign190_e1179 * ((var_lnew).powf(assign190_e1179 - 1.0) * var_lnew_db1)) } } else { (assign190_e1180 * (assign190_e1179 * (var_lnew_db1 / var_lnew))) };
        var_l_lln_db2 = if 0.0 == 0.0 && ((assign190_e1179) as f64).is_finite() && ((assign190_e1179) as f64).fract() == 0.0 { if assign190_e1179 == 0.0 { 0.0 } else { (assign190_e1179 * ((var_lnew).powf(assign190_e1179 - 1.0) * var_lnew_db2)) } } else { (assign190_e1180 * (assign190_e1179 * (var_lnew_db2 / var_lnew))) };
        var_l_lln_db3 = if 0.0 == 0.0 && ((assign190_e1179) as f64).is_finite() && ((assign190_e1179) as f64).fract() == 0.0 { if assign190_e1179 == 0.0 { 0.0 } else { (assign190_e1179 * ((var_lnew).powf(assign190_e1179 - 1.0) * var_lnew_db3)) } } else { (assign190_e1180 * (assign190_e1179 * (var_lnew_db3 / var_lnew))) };
        var_l_lln_db4 = if 0.0 == 0.0 && ((assign190_e1179) as f64).is_finite() && ((assign190_e1179) as f64).fract() == 0.0 { if assign190_e1179 == 0.0 { 0.0 } else { (assign190_e1179 * ((var_lnew).powf(assign190_e1179 - 1.0) * var_lnew_db4)) } } else { (assign190_e1180 * (assign190_e1179 * (var_lnew_db4 / var_lnew))) };
        var_l_lln_rv = 0.0;
        var_l_lln_rdn0 = 0.0;
        var_l_lln_rdn1 = 0.0;
        var_l_lln_rdn2 = 0.0;
        var_l_lln_rdn3 = 0.0;
        var_l_lln_rdn4 = 0.0;
        var_l_lln_rdn5 = 0.0;
        var_l_lln_rdn6 = 0.0;
        var_l_lln_rdn7 = 0.0;
        var_l_lln_rdn8 = 0.0;
        var_l_lln_rdb0 = 0.0;
        var_l_lln_rdb1 = 0.0;
        var_l_lln_rdb2 = 0.0;
        var_l_lln_rdb3 = 0.0;
        var_l_lln_rdb4 = 0.0;

        let assign200_e1183: f64 = (-p.p30);
        let assign200_e1184: f64 = (var_wnew).powf(assign200_e1183);
        var_w_lwn = assign200_e1184;
        var_w_lwn_dn0 = if 0.0 == 0.0 && ((assign200_e1183) as f64).is_finite() && ((assign200_e1183) as f64).fract() == 0.0 { if assign200_e1183 == 0.0 { 0.0 } else { (assign200_e1183 * ((var_wnew).powf(assign200_e1183 - 1.0) * var_wnew_dn0)) } } else { (assign200_e1184 * (assign200_e1183 * (var_wnew_dn0 / var_wnew))) };
        var_w_lwn_dn1 = if 0.0 == 0.0 && ((assign200_e1183) as f64).is_finite() && ((assign200_e1183) as f64).fract() == 0.0 { if assign200_e1183 == 0.0 { 0.0 } else { (assign200_e1183 * ((var_wnew).powf(assign200_e1183 - 1.0) * var_wnew_dn1)) } } else { (assign200_e1184 * (assign200_e1183 * (var_wnew_dn1 / var_wnew))) };
        var_w_lwn_dn2 = if 0.0 == 0.0 && ((assign200_e1183) as f64).is_finite() && ((assign200_e1183) as f64).fract() == 0.0 { if assign200_e1183 == 0.0 { 0.0 } else { (assign200_e1183 * ((var_wnew).powf(assign200_e1183 - 1.0) * var_wnew_dn2)) } } else { (assign200_e1184 * (assign200_e1183 * (var_wnew_dn2 / var_wnew))) };
        var_w_lwn_dn3 = if 0.0 == 0.0 && ((assign200_e1183) as f64).is_finite() && ((assign200_e1183) as f64).fract() == 0.0 { if assign200_e1183 == 0.0 { 0.0 } else { (assign200_e1183 * ((var_wnew).powf(assign200_e1183 - 1.0) * var_wnew_dn3)) } } else { (assign200_e1184 * (assign200_e1183 * (var_wnew_dn3 / var_wnew))) };
        var_w_lwn_dn4 = if 0.0 == 0.0 && ((assign200_e1183) as f64).is_finite() && ((assign200_e1183) as f64).fract() == 0.0 { if assign200_e1183 == 0.0 { 0.0 } else { (assign200_e1183 * ((var_wnew).powf(assign200_e1183 - 1.0) * var_wnew_dn4)) } } else { (assign200_e1184 * (assign200_e1183 * (var_wnew_dn4 / var_wnew))) };
        var_w_lwn_dn5 = if 0.0 == 0.0 && ((assign200_e1183) as f64).is_finite() && ((assign200_e1183) as f64).fract() == 0.0 { if assign200_e1183 == 0.0 { 0.0 } else { (assign200_e1183 * ((var_wnew).powf(assign200_e1183 - 1.0) * var_wnew_dn5)) } } else { (assign200_e1184 * (assign200_e1183 * (var_wnew_dn5 / var_wnew))) };
        var_w_lwn_dn6 = if 0.0 == 0.0 && ((assign200_e1183) as f64).is_finite() && ((assign200_e1183) as f64).fract() == 0.0 { if assign200_e1183 == 0.0 { 0.0 } else { (assign200_e1183 * ((var_wnew).powf(assign200_e1183 - 1.0) * var_wnew_dn6)) } } else { (assign200_e1184 * (assign200_e1183 * (var_wnew_dn6 / var_wnew))) };
        var_w_lwn_dn7 = if 0.0 == 0.0 && ((assign200_e1183) as f64).is_finite() && ((assign200_e1183) as f64).fract() == 0.0 { if assign200_e1183 == 0.0 { 0.0 } else { (assign200_e1183 * ((var_wnew).powf(assign200_e1183 - 1.0) * var_wnew_dn7)) } } else { (assign200_e1184 * (assign200_e1183 * (var_wnew_dn7 / var_wnew))) };
        var_w_lwn_dn8 = if 0.0 == 0.0 && ((assign200_e1183) as f64).is_finite() && ((assign200_e1183) as f64).fract() == 0.0 { if assign200_e1183 == 0.0 { 0.0 } else { (assign200_e1183 * ((var_wnew).powf(assign200_e1183 - 1.0) * var_wnew_dn8)) } } else { (assign200_e1184 * (assign200_e1183 * (var_wnew_dn8 / var_wnew))) };
        var_w_lwn_db0 = if 0.0 == 0.0 && ((assign200_e1183) as f64).is_finite() && ((assign200_e1183) as f64).fract() == 0.0 { if assign200_e1183 == 0.0 { 0.0 } else { (assign200_e1183 * ((var_wnew).powf(assign200_e1183 - 1.0) * var_wnew_db0)) } } else { (assign200_e1184 * (assign200_e1183 * (var_wnew_db0 / var_wnew))) };
        var_w_lwn_db1 = if 0.0 == 0.0 && ((assign200_e1183) as f64).is_finite() && ((assign200_e1183) as f64).fract() == 0.0 { if assign200_e1183 == 0.0 { 0.0 } else { (assign200_e1183 * ((var_wnew).powf(assign200_e1183 - 1.0) * var_wnew_db1)) } } else { (assign200_e1184 * (assign200_e1183 * (var_wnew_db1 / var_wnew))) };
        var_w_lwn_db2 = if 0.0 == 0.0 && ((assign200_e1183) as f64).is_finite() && ((assign200_e1183) as f64).fract() == 0.0 { if assign200_e1183 == 0.0 { 0.0 } else { (assign200_e1183 * ((var_wnew).powf(assign200_e1183 - 1.0) * var_wnew_db2)) } } else { (assign200_e1184 * (assign200_e1183 * (var_wnew_db2 / var_wnew))) };
        var_w_lwn_db3 = if 0.0 == 0.0 && ((assign200_e1183) as f64).is_finite() && ((assign200_e1183) as f64).fract() == 0.0 { if assign200_e1183 == 0.0 { 0.0 } else { (assign200_e1183 * ((var_wnew).powf(assign200_e1183 - 1.0) * var_wnew_db3)) } } else { (assign200_e1184 * (assign200_e1183 * (var_wnew_db3 / var_wnew))) };
        var_w_lwn_db4 = if 0.0 == 0.0 && ((assign200_e1183) as f64).is_finite() && ((assign200_e1183) as f64).fract() == 0.0 { if assign200_e1183 == 0.0 { 0.0 } else { (assign200_e1183 * ((var_wnew).powf(assign200_e1183 - 1.0) * var_wnew_db4)) } } else { (assign200_e1184 * (assign200_e1183 * (var_wnew_db4 / var_wnew))) };
        var_w_lwn_rv = 0.0;
        var_w_lwn_rdn0 = 0.0;
        var_w_lwn_rdn1 = 0.0;
        var_w_lwn_rdn2 = 0.0;
        var_w_lwn_rdn3 = 0.0;
        var_w_lwn_rdn4 = 0.0;
        var_w_lwn_rdn5 = 0.0;
        var_w_lwn_rdn6 = 0.0;
        var_w_lwn_rdn7 = 0.0;
        var_w_lwn_rdn8 = 0.0;
        var_w_lwn_rdb0 = 0.0;
        var_w_lwn_rdb1 = 0.0;
        var_w_lwn_rdb2 = 0.0;
        var_w_lwn_rdb3 = 0.0;
        var_w_lwn_rdb4 = 0.0;

        let assign210_e1187: f64 = (var_l_lln * var_w_lwn);
        var_lw_lln_lwn = assign210_e1187;
        var_lw_lln_lwn_dn0 = ((var_l_lln_dn0 * var_w_lwn) + (var_l_lln * var_w_lwn_dn0));
        var_lw_lln_lwn_dn1 = ((var_l_lln_dn1 * var_w_lwn) + (var_l_lln * var_w_lwn_dn1));
        var_lw_lln_lwn_dn2 = ((var_l_lln_dn2 * var_w_lwn) + (var_l_lln * var_w_lwn_dn2));
        var_lw_lln_lwn_dn3 = ((var_l_lln_dn3 * var_w_lwn) + (var_l_lln * var_w_lwn_dn3));
        var_lw_lln_lwn_dn4 = ((var_l_lln_dn4 * var_w_lwn) + (var_l_lln * var_w_lwn_dn4));
        var_lw_lln_lwn_dn5 = ((var_l_lln_dn5 * var_w_lwn) + (var_l_lln * var_w_lwn_dn5));
        var_lw_lln_lwn_dn6 = ((var_l_lln_dn6 * var_w_lwn) + (var_l_lln * var_w_lwn_dn6));
        var_lw_lln_lwn_dn7 = ((var_l_lln_dn7 * var_w_lwn) + (var_l_lln * var_w_lwn_dn7));
        var_lw_lln_lwn_dn8 = ((var_l_lln_dn8 * var_w_lwn) + (var_l_lln * var_w_lwn_dn8));
        var_lw_lln_lwn_db0 = ((var_l_lln_db0 * var_w_lwn) + (var_l_lln * var_w_lwn_db0));
        var_lw_lln_lwn_db1 = ((var_l_lln_db1 * var_w_lwn) + (var_l_lln * var_w_lwn_db1));
        var_lw_lln_lwn_db2 = ((var_l_lln_db2 * var_w_lwn) + (var_l_lln * var_w_lwn_db2));
        var_lw_lln_lwn_db3 = ((var_l_lln_db3 * var_w_lwn) + (var_l_lln * var_w_lwn_db3));
        var_lw_lln_lwn_db4 = ((var_l_lln_db4 * var_w_lwn) + (var_l_lln * var_w_lwn_db4));
        var_lw_lln_lwn_rv = 0.0;
        var_lw_lln_lwn_rdn0 = 0.0;
        var_lw_lln_lwn_rdn1 = 0.0;
        var_lw_lln_lwn_rdn2 = 0.0;
        var_lw_lln_lwn_rdn3 = 0.0;
        var_lw_lln_lwn_rdn4 = 0.0;
        var_lw_lln_lwn_rdn5 = 0.0;
        var_lw_lln_lwn_rdn6 = 0.0;
        var_lw_lln_lwn_rdn7 = 0.0;
        var_lw_lln_lwn_rdn8 = 0.0;
        var_lw_lln_lwn_rdb0 = 0.0;
        var_lw_lln_lwn_rdb1 = 0.0;
        var_lw_lln_lwn_rdb2 = 0.0;
        var_lw_lln_lwn_rdb3 = 0.0;
        var_lw_lln_lwn_rdb4 = 0.0;

        let assign220_e1191: f64 = (p.p26 * var_l_lln);
        let assign220_e1192: f64 = (p.p25 + assign220_e1191);
        let assign220_e1195: f64 = (p.p27 * var_w_lwn);
        let assign220_e1196: f64 = (assign220_e1192 + assign220_e1195);
        let assign220_e1199: f64 = (p.p28 * var_lw_lln_lwn);
        let assign220_e1200: f64 = (assign220_e1196 + assign220_e1199);
        var_dliv = assign220_e1200;
        var_dliv_dn0 = (((p.p26 * var_l_lln_dn0) + (p.p27 * var_w_lwn_dn0)) + (p.p28 * var_lw_lln_lwn_dn0));
        var_dliv_dn1 = (((p.p26 * var_l_lln_dn1) + (p.p27 * var_w_lwn_dn1)) + (p.p28 * var_lw_lln_lwn_dn1));
        var_dliv_dn2 = (((p.p26 * var_l_lln_dn2) + (p.p27 * var_w_lwn_dn2)) + (p.p28 * var_lw_lln_lwn_dn2));
        var_dliv_dn3 = (((p.p26 * var_l_lln_dn3) + (p.p27 * var_w_lwn_dn3)) + (p.p28 * var_lw_lln_lwn_dn3));
        var_dliv_dn4 = (((p.p26 * var_l_lln_dn4) + (p.p27 * var_w_lwn_dn4)) + (p.p28 * var_lw_lln_lwn_dn4));
        var_dliv_dn5 = (((p.p26 * var_l_lln_dn5) + (p.p27 * var_w_lwn_dn5)) + (p.p28 * var_lw_lln_lwn_dn5));
        var_dliv_dn6 = (((p.p26 * var_l_lln_dn6) + (p.p27 * var_w_lwn_dn6)) + (p.p28 * var_lw_lln_lwn_dn6));
        var_dliv_dn7 = (((p.p26 * var_l_lln_dn7) + (p.p27 * var_w_lwn_dn7)) + (p.p28 * var_lw_lln_lwn_dn7));
        var_dliv_dn8 = (((p.p26 * var_l_lln_dn8) + (p.p27 * var_w_lwn_dn8)) + (p.p28 * var_lw_lln_lwn_dn8));
        var_dliv_db0 = (((p.p26 * var_l_lln_db0) + (p.p27 * var_w_lwn_db0)) + (p.p28 * var_lw_lln_lwn_db0));
        var_dliv_db1 = (((p.p26 * var_l_lln_db1) + (p.p27 * var_w_lwn_db1)) + (p.p28 * var_lw_lln_lwn_db1));
        var_dliv_db2 = (((p.p26 * var_l_lln_db2) + (p.p27 * var_w_lwn_db2)) + (p.p28 * var_lw_lln_lwn_db2));
        var_dliv_db3 = (((p.p26 * var_l_lln_db3) + (p.p27 * var_w_lwn_db3)) + (p.p28 * var_lw_lln_lwn_db3));
        var_dliv_db4 = (((p.p26 * var_l_lln_db4) + (p.p27 * var_w_lwn_db4)) + (p.p28 * var_lw_lln_lwn_db4));
        var_dliv_rv = 0.0;
        var_dliv_rdn0 = 0.0;
        var_dliv_rdn1 = 0.0;
        var_dliv_rdn2 = 0.0;
        var_dliv_rdn3 = 0.0;
        var_dliv_rdn4 = 0.0;
        var_dliv_rdn5 = 0.0;
        var_dliv_rdn6 = 0.0;
        var_dliv_rdn7 = 0.0;
        var_dliv_rdn8 = 0.0;
        var_dliv_rdb0 = 0.0;
        var_dliv_rdb1 = 0.0;
        var_dliv_rdb2 = 0.0;
        var_dliv_rdb3 = 0.0;
        var_dliv_rdb4 = 0.0;

        let assign230_e1203: f64 = (-p.p35);
        let assign230_e1204: f64 = (var_lnew).powf(assign230_e1203);
        var_l_wln = assign230_e1204;
        var_l_wln_dn0 = if 0.0 == 0.0 && ((assign230_e1203) as f64).is_finite() && ((assign230_e1203) as f64).fract() == 0.0 { if assign230_e1203 == 0.0 { 0.0 } else { (assign230_e1203 * ((var_lnew).powf(assign230_e1203 - 1.0) * var_lnew_dn0)) } } else { (assign230_e1204 * (assign230_e1203 * (var_lnew_dn0 / var_lnew))) };
        var_l_wln_dn1 = if 0.0 == 0.0 && ((assign230_e1203) as f64).is_finite() && ((assign230_e1203) as f64).fract() == 0.0 { if assign230_e1203 == 0.0 { 0.0 } else { (assign230_e1203 * ((var_lnew).powf(assign230_e1203 - 1.0) * var_lnew_dn1)) } } else { (assign230_e1204 * (assign230_e1203 * (var_lnew_dn1 / var_lnew))) };
        var_l_wln_dn2 = if 0.0 == 0.0 && ((assign230_e1203) as f64).is_finite() && ((assign230_e1203) as f64).fract() == 0.0 { if assign230_e1203 == 0.0 { 0.0 } else { (assign230_e1203 * ((var_lnew).powf(assign230_e1203 - 1.0) * var_lnew_dn2)) } } else { (assign230_e1204 * (assign230_e1203 * (var_lnew_dn2 / var_lnew))) };
        var_l_wln_dn3 = if 0.0 == 0.0 && ((assign230_e1203) as f64).is_finite() && ((assign230_e1203) as f64).fract() == 0.0 { if assign230_e1203 == 0.0 { 0.0 } else { (assign230_e1203 * ((var_lnew).powf(assign230_e1203 - 1.0) * var_lnew_dn3)) } } else { (assign230_e1204 * (assign230_e1203 * (var_lnew_dn3 / var_lnew))) };
        var_l_wln_dn4 = if 0.0 == 0.0 && ((assign230_e1203) as f64).is_finite() && ((assign230_e1203) as f64).fract() == 0.0 { if assign230_e1203 == 0.0 { 0.0 } else { (assign230_e1203 * ((var_lnew).powf(assign230_e1203 - 1.0) * var_lnew_dn4)) } } else { (assign230_e1204 * (assign230_e1203 * (var_lnew_dn4 / var_lnew))) };
        var_l_wln_dn5 = if 0.0 == 0.0 && ((assign230_e1203) as f64).is_finite() && ((assign230_e1203) as f64).fract() == 0.0 { if assign230_e1203 == 0.0 { 0.0 } else { (assign230_e1203 * ((var_lnew).powf(assign230_e1203 - 1.0) * var_lnew_dn5)) } } else { (assign230_e1204 * (assign230_e1203 * (var_lnew_dn5 / var_lnew))) };
        var_l_wln_dn6 = if 0.0 == 0.0 && ((assign230_e1203) as f64).is_finite() && ((assign230_e1203) as f64).fract() == 0.0 { if assign230_e1203 == 0.0 { 0.0 } else { (assign230_e1203 * ((var_lnew).powf(assign230_e1203 - 1.0) * var_lnew_dn6)) } } else { (assign230_e1204 * (assign230_e1203 * (var_lnew_dn6 / var_lnew))) };
        var_l_wln_dn7 = if 0.0 == 0.0 && ((assign230_e1203) as f64).is_finite() && ((assign230_e1203) as f64).fract() == 0.0 { if assign230_e1203 == 0.0 { 0.0 } else { (assign230_e1203 * ((var_lnew).powf(assign230_e1203 - 1.0) * var_lnew_dn7)) } } else { (assign230_e1204 * (assign230_e1203 * (var_lnew_dn7 / var_lnew))) };
        var_l_wln_dn8 = if 0.0 == 0.0 && ((assign230_e1203) as f64).is_finite() && ((assign230_e1203) as f64).fract() == 0.0 { if assign230_e1203 == 0.0 { 0.0 } else { (assign230_e1203 * ((var_lnew).powf(assign230_e1203 - 1.0) * var_lnew_dn8)) } } else { (assign230_e1204 * (assign230_e1203 * (var_lnew_dn8 / var_lnew))) };
        var_l_wln_db0 = if 0.0 == 0.0 && ((assign230_e1203) as f64).is_finite() && ((assign230_e1203) as f64).fract() == 0.0 { if assign230_e1203 == 0.0 { 0.0 } else { (assign230_e1203 * ((var_lnew).powf(assign230_e1203 - 1.0) * var_lnew_db0)) } } else { (assign230_e1204 * (assign230_e1203 * (var_lnew_db0 / var_lnew))) };
        var_l_wln_db1 = if 0.0 == 0.0 && ((assign230_e1203) as f64).is_finite() && ((assign230_e1203) as f64).fract() == 0.0 { if assign230_e1203 == 0.0 { 0.0 } else { (assign230_e1203 * ((var_lnew).powf(assign230_e1203 - 1.0) * var_lnew_db1)) } } else { (assign230_e1204 * (assign230_e1203 * (var_lnew_db1 / var_lnew))) };
        var_l_wln_db2 = if 0.0 == 0.0 && ((assign230_e1203) as f64).is_finite() && ((assign230_e1203) as f64).fract() == 0.0 { if assign230_e1203 == 0.0 { 0.0 } else { (assign230_e1203 * ((var_lnew).powf(assign230_e1203 - 1.0) * var_lnew_db2)) } } else { (assign230_e1204 * (assign230_e1203 * (var_lnew_db2 / var_lnew))) };
        var_l_wln_db3 = if 0.0 == 0.0 && ((assign230_e1203) as f64).is_finite() && ((assign230_e1203) as f64).fract() == 0.0 { if assign230_e1203 == 0.0 { 0.0 } else { (assign230_e1203 * ((var_lnew).powf(assign230_e1203 - 1.0) * var_lnew_db3)) } } else { (assign230_e1204 * (assign230_e1203 * (var_lnew_db3 / var_lnew))) };
        var_l_wln_db4 = if 0.0 == 0.0 && ((assign230_e1203) as f64).is_finite() && ((assign230_e1203) as f64).fract() == 0.0 { if assign230_e1203 == 0.0 { 0.0 } else { (assign230_e1203 * ((var_lnew).powf(assign230_e1203 - 1.0) * var_lnew_db4)) } } else { (assign230_e1204 * (assign230_e1203 * (var_lnew_db4 / var_lnew))) };
        var_l_wln_rv = 0.0;
        var_l_wln_rdn0 = 0.0;
        var_l_wln_rdn1 = 0.0;
        var_l_wln_rdn2 = 0.0;
        var_l_wln_rdn3 = 0.0;
        var_l_wln_rdn4 = 0.0;
        var_l_wln_rdn5 = 0.0;
        var_l_wln_rdn6 = 0.0;
        var_l_wln_rdn7 = 0.0;
        var_l_wln_rdn8 = 0.0;
        var_l_wln_rdb0 = 0.0;
        var_l_wln_rdb1 = 0.0;
        var_l_wln_rdb2 = 0.0;
        var_l_wln_rdb3 = 0.0;
        var_l_wln_rdb4 = 0.0;

        let assign240_e1207: f64 = (-p.p36);
        let assign240_e1208: f64 = (var_wnew).powf(assign240_e1207);
        var_w_wwn = assign240_e1208;
        var_w_wwn_dn0 = if 0.0 == 0.0 && ((assign240_e1207) as f64).is_finite() && ((assign240_e1207) as f64).fract() == 0.0 { if assign240_e1207 == 0.0 { 0.0 } else { (assign240_e1207 * ((var_wnew).powf(assign240_e1207 - 1.0) * var_wnew_dn0)) } } else { (assign240_e1208 * (assign240_e1207 * (var_wnew_dn0 / var_wnew))) };
        var_w_wwn_dn1 = if 0.0 == 0.0 && ((assign240_e1207) as f64).is_finite() && ((assign240_e1207) as f64).fract() == 0.0 { if assign240_e1207 == 0.0 { 0.0 } else { (assign240_e1207 * ((var_wnew).powf(assign240_e1207 - 1.0) * var_wnew_dn1)) } } else { (assign240_e1208 * (assign240_e1207 * (var_wnew_dn1 / var_wnew))) };
        var_w_wwn_dn2 = if 0.0 == 0.0 && ((assign240_e1207) as f64).is_finite() && ((assign240_e1207) as f64).fract() == 0.0 { if assign240_e1207 == 0.0 { 0.0 } else { (assign240_e1207 * ((var_wnew).powf(assign240_e1207 - 1.0) * var_wnew_dn2)) } } else { (assign240_e1208 * (assign240_e1207 * (var_wnew_dn2 / var_wnew))) };
        var_w_wwn_dn3 = if 0.0 == 0.0 && ((assign240_e1207) as f64).is_finite() && ((assign240_e1207) as f64).fract() == 0.0 { if assign240_e1207 == 0.0 { 0.0 } else { (assign240_e1207 * ((var_wnew).powf(assign240_e1207 - 1.0) * var_wnew_dn3)) } } else { (assign240_e1208 * (assign240_e1207 * (var_wnew_dn3 / var_wnew))) };
        var_w_wwn_dn4 = if 0.0 == 0.0 && ((assign240_e1207) as f64).is_finite() && ((assign240_e1207) as f64).fract() == 0.0 { if assign240_e1207 == 0.0 { 0.0 } else { (assign240_e1207 * ((var_wnew).powf(assign240_e1207 - 1.0) * var_wnew_dn4)) } } else { (assign240_e1208 * (assign240_e1207 * (var_wnew_dn4 / var_wnew))) };
        var_w_wwn_dn5 = if 0.0 == 0.0 && ((assign240_e1207) as f64).is_finite() && ((assign240_e1207) as f64).fract() == 0.0 { if assign240_e1207 == 0.0 { 0.0 } else { (assign240_e1207 * ((var_wnew).powf(assign240_e1207 - 1.0) * var_wnew_dn5)) } } else { (assign240_e1208 * (assign240_e1207 * (var_wnew_dn5 / var_wnew))) };
        var_w_wwn_dn6 = if 0.0 == 0.0 && ((assign240_e1207) as f64).is_finite() && ((assign240_e1207) as f64).fract() == 0.0 { if assign240_e1207 == 0.0 { 0.0 } else { (assign240_e1207 * ((var_wnew).powf(assign240_e1207 - 1.0) * var_wnew_dn6)) } } else { (assign240_e1208 * (assign240_e1207 * (var_wnew_dn6 / var_wnew))) };
        var_w_wwn_dn7 = if 0.0 == 0.0 && ((assign240_e1207) as f64).is_finite() && ((assign240_e1207) as f64).fract() == 0.0 { if assign240_e1207 == 0.0 { 0.0 } else { (assign240_e1207 * ((var_wnew).powf(assign240_e1207 - 1.0) * var_wnew_dn7)) } } else { (assign240_e1208 * (assign240_e1207 * (var_wnew_dn7 / var_wnew))) };
        var_w_wwn_dn8 = if 0.0 == 0.0 && ((assign240_e1207) as f64).is_finite() && ((assign240_e1207) as f64).fract() == 0.0 { if assign240_e1207 == 0.0 { 0.0 } else { (assign240_e1207 * ((var_wnew).powf(assign240_e1207 - 1.0) * var_wnew_dn8)) } } else { (assign240_e1208 * (assign240_e1207 * (var_wnew_dn8 / var_wnew))) };
        var_w_wwn_db0 = if 0.0 == 0.0 && ((assign240_e1207) as f64).is_finite() && ((assign240_e1207) as f64).fract() == 0.0 { if assign240_e1207 == 0.0 { 0.0 } else { (assign240_e1207 * ((var_wnew).powf(assign240_e1207 - 1.0) * var_wnew_db0)) } } else { (assign240_e1208 * (assign240_e1207 * (var_wnew_db0 / var_wnew))) };
        var_w_wwn_db1 = if 0.0 == 0.0 && ((assign240_e1207) as f64).is_finite() && ((assign240_e1207) as f64).fract() == 0.0 { if assign240_e1207 == 0.0 { 0.0 } else { (assign240_e1207 * ((var_wnew).powf(assign240_e1207 - 1.0) * var_wnew_db1)) } } else { (assign240_e1208 * (assign240_e1207 * (var_wnew_db1 / var_wnew))) };
        var_w_wwn_db2 = if 0.0 == 0.0 && ((assign240_e1207) as f64).is_finite() && ((assign240_e1207) as f64).fract() == 0.0 { if assign240_e1207 == 0.0 { 0.0 } else { (assign240_e1207 * ((var_wnew).powf(assign240_e1207 - 1.0) * var_wnew_db2)) } } else { (assign240_e1208 * (assign240_e1207 * (var_wnew_db2 / var_wnew))) };
        var_w_wwn_db3 = if 0.0 == 0.0 && ((assign240_e1207) as f64).is_finite() && ((assign240_e1207) as f64).fract() == 0.0 { if assign240_e1207 == 0.0 { 0.0 } else { (assign240_e1207 * ((var_wnew).powf(assign240_e1207 - 1.0) * var_wnew_db3)) } } else { (assign240_e1208 * (assign240_e1207 * (var_wnew_db3 / var_wnew))) };
        var_w_wwn_db4 = if 0.0 == 0.0 && ((assign240_e1207) as f64).is_finite() && ((assign240_e1207) as f64).fract() == 0.0 { if assign240_e1207 == 0.0 { 0.0 } else { (assign240_e1207 * ((var_wnew).powf(assign240_e1207 - 1.0) * var_wnew_db4)) } } else { (assign240_e1208 * (assign240_e1207 * (var_wnew_db4 / var_wnew))) };
        var_w_wwn_rv = 0.0;
        var_w_wwn_rdn0 = 0.0;
        var_w_wwn_rdn1 = 0.0;
        var_w_wwn_rdn2 = 0.0;
        var_w_wwn_rdn3 = 0.0;
        var_w_wwn_rdn4 = 0.0;
        var_w_wwn_rdn5 = 0.0;
        var_w_wwn_rdn6 = 0.0;
        var_w_wwn_rdn7 = 0.0;
        var_w_wwn_rdn8 = 0.0;
        var_w_wwn_rdb0 = 0.0;
        var_w_wwn_rdb1 = 0.0;
        var_w_wwn_rdb2 = 0.0;
        var_w_wwn_rdb3 = 0.0;
        var_w_wwn_rdb4 = 0.0;

        *var_devsign_slot = var_devsign;
        *var_devsign_rv_slot = var_devsign_rv;
        *var_dliv_slot = var_dliv;
        *var_dliv_db0_slot = var_dliv_db0;
        *var_dliv_db1_slot = var_dliv_db1;
        *var_dliv_db2_slot = var_dliv_db2;
        *var_dliv_db3_slot = var_dliv_db3;
        *var_dliv_db4_slot = var_dliv_db4;
        *var_dliv_dn0_slot = var_dliv_dn0;
        *var_dliv_dn1_slot = var_dliv_dn1;
        *var_dliv_dn2_slot = var_dliv_dn2;
        *var_dliv_dn3_slot = var_dliv_dn3;
        *var_dliv_dn4_slot = var_dliv_dn4;
        *var_dliv_dn5_slot = var_dliv_dn5;
        *var_dliv_dn6_slot = var_dliv_dn6;
        *var_dliv_dn7_slot = var_dliv_dn7;
        *var_dliv_dn8_slot = var_dliv_dn8;
        *var_dliv_rdb0_slot = var_dliv_rdb0;
        *var_dliv_rdb1_slot = var_dliv_rdb1;
        *var_dliv_rdb2_slot = var_dliv_rdb2;
        *var_dliv_rdb3_slot = var_dliv_rdb3;
        *var_dliv_rdb4_slot = var_dliv_rdb4;
        *var_dliv_rdn0_slot = var_dliv_rdn0;
        *var_dliv_rdn1_slot = var_dliv_rdn1;
        *var_dliv_rdn2_slot = var_dliv_rdn2;
        *var_dliv_rdn3_slot = var_dliv_rdn3;
        *var_dliv_rdn4_slot = var_dliv_rdn4;
        *var_dliv_rdn5_slot = var_dliv_rdn5;
        *var_dliv_rdn6_slot = var_dliv_rdn6;
        *var_dliv_rdn7_slot = var_dliv_rdn7;
        *var_dliv_rdn8_slot = var_dliv_rdn8;
        *var_dliv_rv_slot = var_dliv_rv;
        *var_epssi_slot = var_epssi;
        *var_epssi_db0_slot = var_epssi_db0;
        *var_epssi_db1_slot = var_epssi_db1;
        *var_epssi_db2_slot = var_epssi_db2;
        *var_epssi_db3_slot = var_epssi_db3;
        *var_epssi_db4_slot = var_epssi_db4;
        *var_epssi_dn0_slot = var_epssi_dn0;
        *var_epssi_dn1_slot = var_epssi_dn1;
        *var_epssi_dn2_slot = var_epssi_dn2;
        *var_epssi_dn3_slot = var_epssi_dn3;
        *var_epssi_dn4_slot = var_epssi_dn4;
        *var_epssi_dn5_slot = var_epssi_dn5;
        *var_epssi_dn6_slot = var_epssi_dn6;
        *var_epssi_dn7_slot = var_epssi_dn7;
        *var_epssi_dn8_slot = var_epssi_dn8;
        *var_epssi_rdb0_slot = var_epssi_rdb0;
        *var_epssi_rdb1_slot = var_epssi_rdb1;
        *var_epssi_rdb2_slot = var_epssi_rdb2;
        *var_epssi_rdb3_slot = var_epssi_rdb3;
        *var_epssi_rdb4_slot = var_epssi_rdb4;
        *var_epssi_rdn0_slot = var_epssi_rdn0;
        *var_epssi_rdn1_slot = var_epssi_rdn1;
        *var_epssi_rdn2_slot = var_epssi_rdn2;
        *var_epssi_rdn3_slot = var_epssi_rdn3;
        *var_epssi_rdn4_slot = var_epssi_rdn4;
        *var_epssi_rdn5_slot = var_epssi_rdn5;
        *var_epssi_rdn6_slot = var_epssi_rdn6;
        *var_epssi_rdn7_slot = var_epssi_rdn7;
        *var_epssi_rdn8_slot = var_epssi_rdn8;
        *var_epssi_rv_slot = var_epssi_rv;
        *var_guard3_slot = var_guard3;
        *var_guard3_db0_slot = var_guard3_db0;
        *var_guard3_db1_slot = var_guard3_db1;
        *var_guard3_db2_slot = var_guard3_db2;
        *var_guard3_db3_slot = var_guard3_db3;
        *var_guard3_db4_slot = var_guard3_db4;
        *var_guard3_dn0_slot = var_guard3_dn0;
        *var_guard3_dn1_slot = var_guard3_dn1;
        *var_guard3_dn2_slot = var_guard3_dn2;
        *var_guard3_dn3_slot = var_guard3_dn3;
        *var_guard3_dn4_slot = var_guard3_dn4;
        *var_guard3_dn5_slot = var_guard3_dn5;
        *var_guard3_dn6_slot = var_guard3_dn6;
        *var_guard3_dn7_slot = var_guard3_dn7;
        *var_guard3_dn8_slot = var_guard3_dn8;
        *var_guard3_rdb0_slot = var_guard3_rdb0;
        *var_guard3_rdb1_slot = var_guard3_rdb1;
        *var_guard3_rdb2_slot = var_guard3_rdb2;
        *var_guard3_rdb3_slot = var_guard3_rdb3;
        *var_guard3_rdb4_slot = var_guard3_rdb4;
        *var_guard3_rdn0_slot = var_guard3_rdn0;
        *var_guard3_rdn1_slot = var_guard3_rdn1;
        *var_guard3_rdn2_slot = var_guard3_rdn2;
        *var_guard3_rdn3_slot = var_guard3_rdn3;
        *var_guard3_rdn4_slot = var_guard3_rdn4;
        *var_guard3_rdn5_slot = var_guard3_rdn5;
        *var_guard3_rdn6_slot = var_guard3_rdn6;
        *var_guard3_rdn7_slot = var_guard3_rdn7;
        *var_guard3_rdn8_slot = var_guard3_rdn8;
        *var_guard3_rv_slot = var_guard3_rv;
        *var_guard4_slot = var_guard4;
        *var_guard4_db0_slot = var_guard4_db0;
        *var_guard4_db1_slot = var_guard4_db1;
        *var_guard4_db2_slot = var_guard4_db2;
        *var_guard4_db3_slot = var_guard4_db3;
        *var_guard4_db4_slot = var_guard4_db4;
        *var_guard4_dn0_slot = var_guard4_dn0;
        *var_guard4_dn1_slot = var_guard4_dn1;
        *var_guard4_dn2_slot = var_guard4_dn2;
        *var_guard4_dn3_slot = var_guard4_dn3;
        *var_guard4_dn4_slot = var_guard4_dn4;
        *var_guard4_dn5_slot = var_guard4_dn5;
        *var_guard4_dn6_slot = var_guard4_dn6;
        *var_guard4_dn7_slot = var_guard4_dn7;
        *var_guard4_dn8_slot = var_guard4_dn8;
        *var_guard4_rdb0_slot = var_guard4_rdb0;
        *var_guard4_rdb1_slot = var_guard4_rdb1;
        *var_guard4_rdb2_slot = var_guard4_rdb2;
        *var_guard4_rdb3_slot = var_guard4_rdb3;
        *var_guard4_rdb4_slot = var_guard4_rdb4;
        *var_guard4_rdn0_slot = var_guard4_rdn0;
        *var_guard4_rdn1_slot = var_guard4_rdn1;
        *var_guard4_rdn2_slot = var_guard4_rdn2;
        *var_guard4_rdn3_slot = var_guard4_rdn3;
        *var_guard4_rdn4_slot = var_guard4_rdn4;
        *var_guard4_rdn5_slot = var_guard4_rdn5;
        *var_guard4_rdn6_slot = var_guard4_rdn6;
        *var_guard4_rdn7_slot = var_guard4_rdn7;
        *var_guard4_rdn8_slot = var_guard4_rdn8;
        *var_guard4_rv_slot = var_guard4_rv;
        *var_guard5_slot = var_guard5;
        *var_guard5_db0_slot = var_guard5_db0;
        *var_guard5_db1_slot = var_guard5_db1;
        *var_guard5_db2_slot = var_guard5_db2;
        *var_guard5_db3_slot = var_guard5_db3;
        *var_guard5_db4_slot = var_guard5_db4;
        *var_guard5_dn0_slot = var_guard5_dn0;
        *var_guard5_dn1_slot = var_guard5_dn1;
        *var_guard5_dn2_slot = var_guard5_dn2;
        *var_guard5_dn3_slot = var_guard5_dn3;
        *var_guard5_dn4_slot = var_guard5_dn4;
        *var_guard5_dn5_slot = var_guard5_dn5;
        *var_guard5_dn6_slot = var_guard5_dn6;
        *var_guard5_dn7_slot = var_guard5_dn7;
        *var_guard5_dn8_slot = var_guard5_dn8;
        *var_guard5_rdb0_slot = var_guard5_rdb0;
        *var_guard5_rdb1_slot = var_guard5_rdb1;
        *var_guard5_rdb2_slot = var_guard5_rdb2;
        *var_guard5_rdb3_slot = var_guard5_rdb3;
        *var_guard5_rdb4_slot = var_guard5_rdb4;
        *var_guard5_rdn0_slot = var_guard5_rdn0;
        *var_guard5_rdn1_slot = var_guard5_rdn1;
        *var_guard5_rdn2_slot = var_guard5_rdn2;
        *var_guard5_rdn3_slot = var_guard5_rdn3;
        *var_guard5_rdn4_slot = var_guard5_rdn4;
        *var_guard5_rdn5_slot = var_guard5_rdn5;
        *var_guard5_rdn6_slot = var_guard5_rdn6;
        *var_guard5_rdn7_slot = var_guard5_rdn7;
        *var_guard5_rdn8_slot = var_guard5_rdn8;
        *var_guard5_rv_slot = var_guard5_rv;
        *var_l_lln_slot = var_l_lln;
        *var_l_lln_db0_slot = var_l_lln_db0;
        *var_l_lln_db1_slot = var_l_lln_db1;
        *var_l_lln_db2_slot = var_l_lln_db2;
        *var_l_lln_db3_slot = var_l_lln_db3;
        *var_l_lln_db4_slot = var_l_lln_db4;
        *var_l_lln_dn0_slot = var_l_lln_dn0;
        *var_l_lln_dn1_slot = var_l_lln_dn1;
        *var_l_lln_dn2_slot = var_l_lln_dn2;
        *var_l_lln_dn3_slot = var_l_lln_dn3;
        *var_l_lln_dn4_slot = var_l_lln_dn4;
        *var_l_lln_dn5_slot = var_l_lln_dn5;
        *var_l_lln_dn6_slot = var_l_lln_dn6;
        *var_l_lln_dn7_slot = var_l_lln_dn7;
        *var_l_lln_dn8_slot = var_l_lln_dn8;
        *var_l_lln_rdb0_slot = var_l_lln_rdb0;
        *var_l_lln_rdb1_slot = var_l_lln_rdb1;
        *var_l_lln_rdb2_slot = var_l_lln_rdb2;
        *var_l_lln_rdb3_slot = var_l_lln_rdb3;
        *var_l_lln_rdb4_slot = var_l_lln_rdb4;
        *var_l_lln_rdn0_slot = var_l_lln_rdn0;
        *var_l_lln_rdn1_slot = var_l_lln_rdn1;
        *var_l_lln_rdn2_slot = var_l_lln_rdn2;
        *var_l_lln_rdn3_slot = var_l_lln_rdn3;
        *var_l_lln_rdn4_slot = var_l_lln_rdn4;
        *var_l_lln_rdn5_slot = var_l_lln_rdn5;
        *var_l_lln_rdn6_slot = var_l_lln_rdn6;
        *var_l_lln_rdn7_slot = var_l_lln_rdn7;
        *var_l_lln_rdn8_slot = var_l_lln_rdn8;
        *var_l_lln_rv_slot = var_l_lln_rv;
        *var_l_wln_slot = var_l_wln;
        *var_l_wln_db0_slot = var_l_wln_db0;
        *var_l_wln_db1_slot = var_l_wln_db1;
        *var_l_wln_db2_slot = var_l_wln_db2;
        *var_l_wln_db3_slot = var_l_wln_db3;
        *var_l_wln_db4_slot = var_l_wln_db4;
        *var_l_wln_dn0_slot = var_l_wln_dn0;
        *var_l_wln_dn1_slot = var_l_wln_dn1;
        *var_l_wln_dn2_slot = var_l_wln_dn2;
        *var_l_wln_dn3_slot = var_l_wln_dn3;
        *var_l_wln_dn4_slot = var_l_wln_dn4;
        *var_l_wln_dn5_slot = var_l_wln_dn5;
        *var_l_wln_dn6_slot = var_l_wln_dn6;
        *var_l_wln_dn7_slot = var_l_wln_dn7;
        *var_l_wln_dn8_slot = var_l_wln_dn8;
        *var_l_wln_rdb0_slot = var_l_wln_rdb0;
        *var_l_wln_rdb1_slot = var_l_wln_rdb1;
        *var_l_wln_rdb2_slot = var_l_wln_rdb2;
        *var_l_wln_rdb3_slot = var_l_wln_rdb3;
        *var_l_wln_rdb4_slot = var_l_wln_rdb4;
        *var_l_wln_rdn0_slot = var_l_wln_rdn0;
        *var_l_wln_rdn1_slot = var_l_wln_rdn1;
        *var_l_wln_rdn2_slot = var_l_wln_rdn2;
        *var_l_wln_rdn3_slot = var_l_wln_rdn3;
        *var_l_wln_rdn4_slot = var_l_wln_rdn4;
        *var_l_wln_rdn5_slot = var_l_wln_rdn5;
        *var_l_wln_rdn6_slot = var_l_wln_rdn6;
        *var_l_wln_rdn7_slot = var_l_wln_rdn7;
        *var_l_wln_rdn8_slot = var_l_wln_rdn8;
        *var_l_wln_rv_slot = var_l_wln_rv;
        *var_lnew_slot = var_lnew;
        *var_lnew_db0_slot = var_lnew_db0;
        *var_lnew_db1_slot = var_lnew_db1;
        *var_lnew_db2_slot = var_lnew_db2;
        *var_lnew_db3_slot = var_lnew_db3;
        *var_lnew_db4_slot = var_lnew_db4;
        *var_lnew_dn0_slot = var_lnew_dn0;
        *var_lnew_dn1_slot = var_lnew_dn1;
        *var_lnew_dn2_slot = var_lnew_dn2;
        *var_lnew_dn3_slot = var_lnew_dn3;
        *var_lnew_dn4_slot = var_lnew_dn4;
        *var_lnew_dn5_slot = var_lnew_dn5;
        *var_lnew_dn6_slot = var_lnew_dn6;
        *var_lnew_dn7_slot = var_lnew_dn7;
        *var_lnew_dn8_slot = var_lnew_dn8;
        *var_lnew_rdb0_slot = var_lnew_rdb0;
        *var_lnew_rdb1_slot = var_lnew_rdb1;
        *var_lnew_rdb2_slot = var_lnew_rdb2;
        *var_lnew_rdb3_slot = var_lnew_rdb3;
        *var_lnew_rdb4_slot = var_lnew_rdb4;
        *var_lnew_rdn0_slot = var_lnew_rdn0;
        *var_lnew_rdn1_slot = var_lnew_rdn1;
        *var_lnew_rdn2_slot = var_lnew_rdn2;
        *var_lnew_rdn3_slot = var_lnew_rdn3;
        *var_lnew_rdn4_slot = var_lnew_rdn4;
        *var_lnew_rdn5_slot = var_lnew_rdn5;
        *var_lnew_rdn6_slot = var_lnew_rdn6;
        *var_lnew_rdn7_slot = var_lnew_rdn7;
        *var_lnew_rdn8_slot = var_lnew_rdn8;
        *var_lnew_rv_slot = var_lnew_rv;
        *var_lw_lln_lwn_slot = var_lw_lln_lwn;
        *var_lw_lln_lwn_db0_slot = var_lw_lln_lwn_db0;
        *var_lw_lln_lwn_db1_slot = var_lw_lln_lwn_db1;
        *var_lw_lln_lwn_db2_slot = var_lw_lln_lwn_db2;
        *var_lw_lln_lwn_db3_slot = var_lw_lln_lwn_db3;
        *var_lw_lln_lwn_db4_slot = var_lw_lln_lwn_db4;
        *var_lw_lln_lwn_dn0_slot = var_lw_lln_lwn_dn0;
        *var_lw_lln_lwn_dn1_slot = var_lw_lln_lwn_dn1;
        *var_lw_lln_lwn_dn2_slot = var_lw_lln_lwn_dn2;
        *var_lw_lln_lwn_dn3_slot = var_lw_lln_lwn_dn3;
        *var_lw_lln_lwn_dn4_slot = var_lw_lln_lwn_dn4;
        *var_lw_lln_lwn_dn5_slot = var_lw_lln_lwn_dn5;
        *var_lw_lln_lwn_dn6_slot = var_lw_lln_lwn_dn6;
        *var_lw_lln_lwn_dn7_slot = var_lw_lln_lwn_dn7;
        *var_lw_lln_lwn_dn8_slot = var_lw_lln_lwn_dn8;
        *var_lw_lln_lwn_rdb0_slot = var_lw_lln_lwn_rdb0;
        *var_lw_lln_lwn_rdb1_slot = var_lw_lln_lwn_rdb1;
        *var_lw_lln_lwn_rdb2_slot = var_lw_lln_lwn_rdb2;
        *var_lw_lln_lwn_rdb3_slot = var_lw_lln_lwn_rdb3;
        *var_lw_lln_lwn_rdb4_slot = var_lw_lln_lwn_rdb4;
        *var_lw_lln_lwn_rdn0_slot = var_lw_lln_lwn_rdn0;
        *var_lw_lln_lwn_rdn1_slot = var_lw_lln_lwn_rdn1;
        *var_lw_lln_lwn_rdn2_slot = var_lw_lln_lwn_rdn2;
        *var_lw_lln_lwn_rdn3_slot = var_lw_lln_lwn_rdn3;
        *var_lw_lln_lwn_rdn4_slot = var_lw_lln_lwn_rdn4;
        *var_lw_lln_lwn_rdn5_slot = var_lw_lln_lwn_rdn5;
        *var_lw_lln_lwn_rdn6_slot = var_lw_lln_lwn_rdn6;
        *var_lw_lln_lwn_rdn7_slot = var_lw_lln_lwn_rdn7;
        *var_lw_lln_lwn_rdn8_slot = var_lw_lln_lwn_rdn8;
        *var_lw_lln_lwn_rv_slot = var_lw_lln_lwn_rv;
        *var_w_lwn_slot = var_w_lwn;
        *var_w_lwn_db0_slot = var_w_lwn_db0;
        *var_w_lwn_db1_slot = var_w_lwn_db1;
        *var_w_lwn_db2_slot = var_w_lwn_db2;
        *var_w_lwn_db3_slot = var_w_lwn_db3;
        *var_w_lwn_db4_slot = var_w_lwn_db4;
        *var_w_lwn_dn0_slot = var_w_lwn_dn0;
        *var_w_lwn_dn1_slot = var_w_lwn_dn1;
        *var_w_lwn_dn2_slot = var_w_lwn_dn2;
        *var_w_lwn_dn3_slot = var_w_lwn_dn3;
        *var_w_lwn_dn4_slot = var_w_lwn_dn4;
        *var_w_lwn_dn5_slot = var_w_lwn_dn5;
        *var_w_lwn_dn6_slot = var_w_lwn_dn6;
        *var_w_lwn_dn7_slot = var_w_lwn_dn7;
        *var_w_lwn_dn8_slot = var_w_lwn_dn8;
        *var_w_lwn_rdb0_slot = var_w_lwn_rdb0;
        *var_w_lwn_rdb1_slot = var_w_lwn_rdb1;
        *var_w_lwn_rdb2_slot = var_w_lwn_rdb2;
        *var_w_lwn_rdb3_slot = var_w_lwn_rdb3;
        *var_w_lwn_rdb4_slot = var_w_lwn_rdb4;
        *var_w_lwn_rdn0_slot = var_w_lwn_rdn0;
        *var_w_lwn_rdn1_slot = var_w_lwn_rdn1;
        *var_w_lwn_rdn2_slot = var_w_lwn_rdn2;
        *var_w_lwn_rdn3_slot = var_w_lwn_rdn3;
        *var_w_lwn_rdn4_slot = var_w_lwn_rdn4;
        *var_w_lwn_rdn5_slot = var_w_lwn_rdn5;
        *var_w_lwn_rdn6_slot = var_w_lwn_rdn6;
        *var_w_lwn_rdn7_slot = var_w_lwn_rdn7;
        *var_w_lwn_rdn8_slot = var_w_lwn_rdn8;
        *var_w_lwn_rv_slot = var_w_lwn_rv;
        *var_w_wwn_slot = var_w_wwn;
        *var_w_wwn_db0_slot = var_w_wwn_db0;
        *var_w_wwn_db1_slot = var_w_wwn_db1;
        *var_w_wwn_db2_slot = var_w_wwn_db2;
        *var_w_wwn_db3_slot = var_w_wwn_db3;
        *var_w_wwn_db4_slot = var_w_wwn_db4;
        *var_w_wwn_dn0_slot = var_w_wwn_dn0;
        *var_w_wwn_dn1_slot = var_w_wwn_dn1;
        *var_w_wwn_dn2_slot = var_w_wwn_dn2;
        *var_w_wwn_dn3_slot = var_w_wwn_dn3;
        *var_w_wwn_dn4_slot = var_w_wwn_dn4;
        *var_w_wwn_dn5_slot = var_w_wwn_dn5;
        *var_w_wwn_dn6_slot = var_w_wwn_dn6;
        *var_w_wwn_dn7_slot = var_w_wwn_dn7;
        *var_w_wwn_dn8_slot = var_w_wwn_dn8;
        *var_w_wwn_rdb0_slot = var_w_wwn_rdb0;
        *var_w_wwn_rdb1_slot = var_w_wwn_rdb1;
        *var_w_wwn_rdb2_slot = var_w_wwn_rdb2;
        *var_w_wwn_rdb3_slot = var_w_wwn_rdb3;
        *var_w_wwn_rdb4_slot = var_w_wwn_rdb4;
        *var_w_wwn_rdn0_slot = var_w_wwn_rdn0;
        *var_w_wwn_rdn1_slot = var_w_wwn_rdn1;
        *var_w_wwn_rdn2_slot = var_w_wwn_rdn2;
        *var_w_wwn_rdn3_slot = var_w_wwn_rdn3;
        *var_w_wwn_rdn4_slot = var_w_wwn_rdn4;
        *var_w_wwn_rdn5_slot = var_w_wwn_rdn5;
        *var_w_wwn_rdn6_slot = var_w_wwn_rdn6;
        *var_w_wwn_rdn7_slot = var_w_wwn_rdn7;
        *var_w_wwn_rdn8_slot = var_w_wwn_rdn8;
        *var_w_wwn_rv_slot = var_w_wwn_rv;
        *var_welsign_slot = var_welsign;
        *var_welsign_rv_slot = var_welsign_rv;
        *var_wnew_slot = var_wnew;
        *var_wnew_db0_slot = var_wnew_db0;
        *var_wnew_db1_slot = var_wnew_db1;
        *var_wnew_db2_slot = var_wnew_db2;
        *var_wnew_db3_slot = var_wnew_db3;
        *var_wnew_db4_slot = var_wnew_db4;
        *var_wnew_dn0_slot = var_wnew_dn0;
        *var_wnew_dn1_slot = var_wnew_dn1;
        *var_wnew_dn2_slot = var_wnew_dn2;
        *var_wnew_dn3_slot = var_wnew_dn3;
        *var_wnew_dn4_slot = var_wnew_dn4;
        *var_wnew_dn5_slot = var_wnew_dn5;
        *var_wnew_dn6_slot = var_wnew_dn6;
        *var_wnew_dn7_slot = var_wnew_dn7;
        *var_wnew_dn8_slot = var_wnew_dn8;
        *var_wnew_rdb0_slot = var_wnew_rdb0;
        *var_wnew_rdb1_slot = var_wnew_rdb1;
        *var_wnew_rdb2_slot = var_wnew_rdb2;
        *var_wnew_rdb3_slot = var_wnew_rdb3;
        *var_wnew_rdb4_slot = var_wnew_rdb4;
        *var_wnew_rdn0_slot = var_wnew_rdn0;
        *var_wnew_rdn1_slot = var_wnew_rdn1;
        *var_wnew_rdn2_slot = var_wnew_rdn2;
        *var_wnew_rdn3_slot = var_wnew_rdn3;
        *var_wnew_rdn4_slot = var_wnew_rdn4;
        *var_wnew_rdn5_slot = var_wnew_rdn5;
        *var_wnew_rdn6_slot = var_wnew_rdn6;
        *var_wnew_rdn7_slot = var_wnew_rdn7;
        *var_wnew_rdn8_slot = var_wnew_rdn8;
        *var_wnew_rv_slot = var_wnew_rv;
    }
}
