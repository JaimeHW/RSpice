#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_112(
        locals: &mut StampLocals,
    ) {
        let (assign41220_e47100, assign41220_e47100_d_n4, assign41220_e47100_d_n6, assign41220_e47100_d_n7, assign41220_e47100_d_n8, assign41220_e47100_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 != 0.0)) {
        let assign41220_e47095: f64 = (locals.var_aexp2s__blk944 / locals.var_qis__blk938);
        let assign41220_e47097: f64 = (assign41220_e47095 - locals.var_dqsqs_dxn_qi__blk950);
        let assign41220_e47098: f64 = (locals.var_a2s__blk948 / assign41220_e47097);
        (assign41220_e47098, (((locals.var_a2s__blk948_dn4 * assign41220_e47097) - (locals.var_a2s__blk948 * ((((locals.var_aexp2s__blk944_dn4 * locals.var_qis__blk938) - (locals.var_aexp2s__blk944 * locals.var_qis__blk938_dn4)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn4))) / (assign41220_e47097 * assign41220_e47097)), (((locals.var_a2s__blk948_dn6 * assign41220_e47097) - (locals.var_a2s__blk948 * ((((locals.var_aexp2s__blk944_dn6 * locals.var_qis__blk938) - (locals.var_aexp2s__blk944 * locals.var_qis__blk938_dn6)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn6))) / (assign41220_e47097 * assign41220_e47097)), (((locals.var_a2s__blk948_dn7 * assign41220_e47097) - (locals.var_a2s__blk948 * ((((locals.var_aexp2s__blk944_dn7 * locals.var_qis__blk938) - (locals.var_aexp2s__blk944 * locals.var_qis__blk938_dn7)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn7))) / (assign41220_e47097 * assign41220_e47097)), (((locals.var_a2s__blk948_dn8 * assign41220_e47097) - (locals.var_a2s__blk948 * ((((locals.var_aexp2s__blk944_dn8 * locals.var_qis__blk938) - (locals.var_aexp2s__blk944 * locals.var_qis__blk938_dn8)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn8))) / (assign41220_e47097 * assign41220_e47097)), (((locals.var_a2s__blk948_dn9 * assign41220_e47097) - (locals.var_a2s__blk948 * ((((locals.var_aexp2s__blk944_dn9 * locals.var_qis__blk938) - (locals.var_aexp2s__blk944 * locals.var_qis__blk938_dn9)) / (locals.var_qis__blk938 * locals.var_qis__blk938)) - locals.var_dqsqs_dxn_qi__blk950_dn9))) / (assign41220_e47097 * assign41220_e47097)),)
    } else {
        (locals.var_q2s_chap__blk1067, locals.var_q2s_chap__blk1067_dn4, locals.var_q2s_chap__blk1067_dn6, locals.var_q2s_chap__blk1067_dn7, locals.var_q2s_chap__blk1067_dn8, locals.var_q2s_chap__blk1067_dn9,)
    }
};
        locals.var_q2s_chap__blk1067 = assign41220_e47100;
        locals.var_q2s_chap__blk1067_dn4 = assign41220_e47100_d_n4;
        locals.var_q2s_chap__blk1067_dn6 = assign41220_e47100_d_n6;
        locals.var_q2s_chap__blk1067_dn7 = assign41220_e47100_d_n7;
        locals.var_q2s_chap__blk1067_dn8 = assign41220_e47100_d_n8;
        locals.var_q2s_chap__blk1067_dn9 = assign41220_e47100_d_n9;

        let (assign41230_e47114, assign41230_e47114_d_n4, assign41230_e47114_d_n6, assign41230_e47114_d_n7, assign41230_e47114_d_n8, assign41230_e47114_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 != 0.0)) {
        let assign41230_e47109: f64 = (locals.var_aexp2d__blk1008 / locals.var_qid__blk1003);
        let assign41230_e47111: f64 = (assign41230_e47109 - locals.var_dqsqd_dxn_qi__blk1014);
        let assign41230_e47112: f64 = (locals.var_a2d__blk1012 / assign41230_e47111);
        (assign41230_e47112, (((locals.var_a2d__blk1012_dn4 * assign41230_e47111) - (locals.var_a2d__blk1012 * ((((locals.var_aexp2d__blk1008_dn4 * locals.var_qid__blk1003) - (locals.var_aexp2d__blk1008 * locals.var_qid__blk1003_dn4)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn4))) / (assign41230_e47111 * assign41230_e47111)), (((locals.var_a2d__blk1012_dn6 * assign41230_e47111) - (locals.var_a2d__blk1012 * ((((locals.var_aexp2d__blk1008_dn6 * locals.var_qid__blk1003) - (locals.var_aexp2d__blk1008 * locals.var_qid__blk1003_dn6)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn6))) / (assign41230_e47111 * assign41230_e47111)), (((locals.var_a2d__blk1012_dn7 * assign41230_e47111) - (locals.var_a2d__blk1012 * ((((locals.var_aexp2d__blk1008_dn7 * locals.var_qid__blk1003) - (locals.var_aexp2d__blk1008 * locals.var_qid__blk1003_dn7)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn7))) / (assign41230_e47111 * assign41230_e47111)), (((locals.var_a2d__blk1012_dn8 * assign41230_e47111) - (locals.var_a2d__blk1012 * ((((locals.var_aexp2d__blk1008_dn8 * locals.var_qid__blk1003) - (locals.var_aexp2d__blk1008 * locals.var_qid__blk1003_dn8)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn8))) / (assign41230_e47111 * assign41230_e47111)), (((locals.var_a2d__blk1012_dn9 * assign41230_e47111) - (locals.var_a2d__blk1012 * ((((locals.var_aexp2d__blk1008_dn9 * locals.var_qid__blk1003) - (locals.var_aexp2d__blk1008 * locals.var_qid__blk1003_dn9)) / (locals.var_qid__blk1003 * locals.var_qid__blk1003)) - locals.var_dqsqd_dxn_qi__blk1014_dn9))) / (assign41230_e47111 * assign41230_e47111)),)
    } else {
        (locals.var_q2d_chap__blk1068, locals.var_q2d_chap__blk1068_dn4, locals.var_q2d_chap__blk1068_dn6, locals.var_q2d_chap__blk1068_dn7, locals.var_q2d_chap__blk1068_dn8, locals.var_q2d_chap__blk1068_dn9,)
    }
};
        locals.var_q2d_chap__blk1068 = assign41230_e47114;
        locals.var_q2d_chap__blk1068_dn4 = assign41230_e47114_d_n4;
        locals.var_q2d_chap__blk1068_dn6 = assign41230_e47114_d_n6;
        locals.var_q2d_chap__blk1068_dn7 = assign41230_e47114_d_n7;
        locals.var_q2d_chap__blk1068_dn8 = assign41230_e47114_d_n8;
        locals.var_q2d_chap__blk1068_dn9 = assign41230_e47114_d_n9;

        let (assign41240_e47126, assign41240_e47126_d_n4, assign41240_e47126_d_n6, assign41240_e47126_d_n7, assign41240_e47126_d_n8, assign41240_e47126_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 != 0.0)) {
        let assign41240_e47122: f64 = (locals.var_q2s_chap__blk1067 - locals.var_q2d_chap__blk1068);
        let assign41240_e47124: f64 = (assign41240_e47122 / locals.var_norm_ids__blk1063);
        (assign41240_e47124, ((((locals.var_q2s_chap__blk1067_dn4 - locals.var_q2d_chap__blk1068_dn4) * locals.var_norm_ids__blk1063) - (assign41240_e47122 * locals.var_norm_ids__blk1063_dn4)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q2s_chap__blk1067_dn6 - locals.var_q2d_chap__blk1068_dn6) * locals.var_norm_ids__blk1063) - (assign41240_e47122 * locals.var_norm_ids__blk1063_dn6)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q2s_chap__blk1067_dn7 - locals.var_q2d_chap__blk1068_dn7) * locals.var_norm_ids__blk1063) - (assign41240_e47122 * locals.var_norm_ids__blk1063_dn7)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q2s_chap__blk1067_dn8 - locals.var_q2d_chap__blk1068_dn8) * locals.var_norm_ids__blk1063) - (assign41240_e47122 * locals.var_norm_ids__blk1063_dn8)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)), ((((locals.var_q2s_chap__blk1067_dn9 - locals.var_q2d_chap__blk1068_dn9) * locals.var_norm_ids__blk1063) - (assign41240_e47122 * locals.var_norm_ids__blk1063_dn9)) / (locals.var_norm_ids__blk1063 * locals.var_norm_ids__blk1063)),)
    } else {
        (locals.var_inv_k2h2_0__blk1069, locals.var_inv_k2h2_0__blk1069_dn4, locals.var_inv_k2h2_0__blk1069_dn6, locals.var_inv_k2h2_0__blk1069_dn7, locals.var_inv_k2h2_0__blk1069_dn8, locals.var_inv_k2h2_0__blk1069_dn9,)
    }
};
        locals.var_inv_k2h2_0__blk1069 = assign41240_e47126;
        locals.var_inv_k2h2_0__blk1069_dn4 = assign41240_e47126_d_n4;
        locals.var_inv_k2h2_0__blk1069_dn6 = assign41240_e47126_d_n6;
        locals.var_inv_k2h2_0__blk1069_dn7 = assign41240_e47126_d_n7;
        locals.var_inv_k2h2_0__blk1069_dn8 = assign41240_e47126_d_n8;
        locals.var_inv_k2h2_0__blk1069_dn9 = assign41240_e47126_d_n9;

        let (assign41250_e47135, assign41250_e47135_d_n4, assign41250_e47135_d_n6, assign41250_e47135_d_n7, assign41250_e47135_d_n8, assign41250_e47135_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_inv_k1h1_0__blk1066, locals.var_inv_k1h1_0__blk1066_dn4, locals.var_inv_k1h1_0__blk1066_dn6, locals.var_inv_k1h1_0__blk1066_dn7, locals.var_inv_k1h1_0__blk1066_dn8, locals.var_inv_k1h1_0__blk1066_dn9,)
    }
};
        locals.var_inv_k1h1_0__blk1066 = assign41250_e47135;
        locals.var_inv_k1h1_0__blk1066_dn4 = assign41250_e47135_d_n4;
        locals.var_inv_k1h1_0__blk1066_dn6 = assign41250_e47135_d_n6;
        locals.var_inv_k1h1_0__blk1066_dn7 = assign41250_e47135_d_n7;
        locals.var_inv_k1h1_0__blk1066_dn8 = assign41250_e47135_d_n8;
        locals.var_inv_k1h1_0__blk1066_dn9 = assign41250_e47135_d_n9;

        let (assign41260_e47144, assign41260_e47144_d_n4, assign41260_e47144_d_n6, assign41260_e47144_d_n7, assign41260_e47144_d_n8, assign41260_e47144_d_n9,) = {
    if (((locals.var_guard1080 != 0.0) && (locals.var_guard1232 != 0.0)) && (locals.var_guard1233 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_inv_k2h2_0__blk1069, locals.var_inv_k2h2_0__blk1069_dn4, locals.var_inv_k2h2_0__blk1069_dn6, locals.var_inv_k2h2_0__blk1069_dn7, locals.var_inv_k2h2_0__blk1069_dn8, locals.var_inv_k2h2_0__blk1069_dn9,)
    }
};
        locals.var_inv_k2h2_0__blk1069 = assign41260_e47144;
        locals.var_inv_k2h2_0__blk1069_dn4 = assign41260_e47144_d_n4;
        locals.var_inv_k2h2_0__blk1069_dn6 = assign41260_e47144_d_n6;
        locals.var_inv_k2h2_0__blk1069_dn7 = assign41260_e47144_d_n7;
        locals.var_inv_k2h2_0__blk1069_dn8 = assign41260_e47144_d_n8;
        locals.var_inv_k2h2_0__blk1069_dn9 = assign41260_e47144_d_n9;

        let (assign41270_e47160, assign41270_e47160_d_n4, assign41270_e47160_d_n6, assign41270_e47160_d_n7, assign41270_e47160_d_n8, assign41270_e47160_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
        let assign41270_e47150: f64 = (-2.0);
        let assign41270_e47152: f64 = (assign41270_e47150 * locals.var_s1__blk969);
        let assign41270_e47155: f64 = (locals.var_inv_k1__blk906 / locals.var_q1chapinf__blk972);
        let assign41270_e47157: f64 = (assign41270_e47155 + locals.var_inv_dinf__blk975);
        let assign41270_e47158: f64 = (assign41270_e47152 * assign41270_e47157);
        (assign41270_e47158, (((assign41270_e47150 * locals.var_s1__blk969_dn4) * assign41270_e47157) + (assign41270_e47152 * ((((locals.var_inv_k1__blk906_dn4 * locals.var_q1chapinf__blk972) - (locals.var_inv_k1__blk906 * locals.var_q1chapinf__blk972_dn4)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972)) + locals.var_inv_dinf__blk975_dn4))), (((assign41270_e47150 * locals.var_s1__blk969_dn6) * assign41270_e47157) + (assign41270_e47152 * ((((locals.var_inv_k1__blk906_dn6 * locals.var_q1chapinf__blk972) - (locals.var_inv_k1__blk906 * locals.var_q1chapinf__blk972_dn6)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972)) + locals.var_inv_dinf__blk975_dn6))), (((assign41270_e47150 * locals.var_s1__blk969_dn7) * assign41270_e47157) + (assign41270_e47152 * ((((locals.var_inv_k1__blk906_dn7 * locals.var_q1chapinf__blk972) - (locals.var_inv_k1__blk906 * locals.var_q1chapinf__blk972_dn7)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972)) + locals.var_inv_dinf__blk975_dn7))), (((assign41270_e47150 * locals.var_s1__blk969_dn8) * assign41270_e47157) + (assign41270_e47152 * ((((locals.var_inv_k1__blk906_dn8 * locals.var_q1chapinf__blk972) - (locals.var_inv_k1__blk906 * locals.var_q1chapinf__blk972_dn8)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972)) + locals.var_inv_dinf__blk975_dn8))), (((assign41270_e47150 * locals.var_s1__blk969_dn9) * assign41270_e47157) + (assign41270_e47152 * ((((locals.var_inv_k1__blk906_dn9 * locals.var_q1chapinf__blk972) - (locals.var_inv_k1__blk906 * locals.var_q1chapinf__blk972_dn9)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972)) + locals.var_inv_dinf__blk975_dn9))),)
    } else {
        (locals.var_zeta1__blk1070, locals.var_zeta1__blk1070_dn4, locals.var_zeta1__blk1070_dn6, locals.var_zeta1__blk1070_dn7, locals.var_zeta1__blk1070_dn8, locals.var_zeta1__blk1070_dn9,)
    }
};
        locals.var_zeta1__blk1070 = assign41270_e47160;
        locals.var_zeta1__blk1070_dn4 = assign41270_e47160_d_n4;
        locals.var_zeta1__blk1070_dn6 = assign41270_e47160_d_n6;
        locals.var_zeta1__blk1070_dn7 = assign41270_e47160_d_n7;
        locals.var_zeta1__blk1070_dn8 = assign41270_e47160_d_n8;
        locals.var_zeta1__blk1070_dn9 = assign41270_e47160_d_n9;

        let (assign41280_e47176, assign41280_e47176_d_n4, assign41280_e47176_d_n6, assign41280_e47176_d_n7, assign41280_e47176_d_n8, assign41280_e47176_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
        let assign41280_e47166: f64 = (-2.0);
        let assign41280_e47168: f64 = (assign41280_e47166 * locals.var_s2__blk970);
        let assign41280_e47171: f64 = (locals.var_inv_k2__blk907 / locals.var_q2chapinf__blk973);
        let assign41280_e47173: f64 = (assign41280_e47171 + locals.var_inv_dinf__blk975);
        let assign41280_e47174: f64 = (assign41280_e47168 * assign41280_e47173);
        (assign41280_e47174, (((assign41280_e47166 * locals.var_s2__blk970_dn4) * assign41280_e47173) + (assign41280_e47168 * ((((locals.var_inv_k2__blk907_dn4 * locals.var_q2chapinf__blk973) - (locals.var_inv_k2__blk907 * locals.var_q2chapinf__blk973_dn4)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) + locals.var_inv_dinf__blk975_dn4))), (((assign41280_e47166 * locals.var_s2__blk970_dn6) * assign41280_e47173) + (assign41280_e47168 * ((((locals.var_inv_k2__blk907_dn6 * locals.var_q2chapinf__blk973) - (locals.var_inv_k2__blk907 * locals.var_q2chapinf__blk973_dn6)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) + locals.var_inv_dinf__blk975_dn6))), (((assign41280_e47166 * locals.var_s2__blk970_dn7) * assign41280_e47173) + (assign41280_e47168 * ((((locals.var_inv_k2__blk907_dn7 * locals.var_q2chapinf__blk973) - (locals.var_inv_k2__blk907 * locals.var_q2chapinf__blk973_dn7)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) + locals.var_inv_dinf__blk975_dn7))), (((assign41280_e47166 * locals.var_s2__blk970_dn8) * assign41280_e47173) + (assign41280_e47168 * ((((locals.var_inv_k2__blk907_dn8 * locals.var_q2chapinf__blk973) - (locals.var_inv_k2__blk907 * locals.var_q2chapinf__blk973_dn8)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) + locals.var_inv_dinf__blk975_dn8))), (((assign41280_e47166 * locals.var_s2__blk970_dn9) * assign41280_e47173) + (assign41280_e47168 * ((((locals.var_inv_k2__blk907_dn9 * locals.var_q2chapinf__blk973) - (locals.var_inv_k2__blk907 * locals.var_q2chapinf__blk973_dn9)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973)) + locals.var_inv_dinf__blk975_dn9))),)
    } else {
        (locals.var_zeta2__blk1071, locals.var_zeta2__blk1071_dn4, locals.var_zeta2__blk1071_dn6, locals.var_zeta2__blk1071_dn7, locals.var_zeta2__blk1071_dn8, locals.var_zeta2__blk1071_dn9,)
    }
};
        locals.var_zeta2__blk1071 = assign41280_e47176;
        locals.var_zeta2__blk1071_dn4 = assign41280_e47176_d_n4;
        locals.var_zeta2__blk1071_dn6 = assign41280_e47176_d_n6;
        locals.var_zeta2__blk1071_dn7 = assign41280_e47176_d_n7;
        locals.var_zeta2__blk1071_dn8 = assign41280_e47176_d_n8;
        locals.var_zeta2__blk1071_dn9 = assign41280_e47176_d_n9;

        let (assign41290_e47187, assign41290_e47187_d_n4, assign41290_e47187_d_n6, assign41290_e47187_d_n7, assign41290_e47187_d_n8, assign41290_e47187_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
        let assign41290_e47183: f64 = (locals.var_zeta2__blk1071 - locals.var_zeta1__blk1070);
        let assign41290_e47185: f64 = (assign41290_e47183 * locals.var_inv_dinf__blk975);
        (assign41290_e47185, (((locals.var_zeta2__blk1071_dn4 - locals.var_zeta1__blk1070_dn4) * locals.var_inv_dinf__blk975) + (assign41290_e47183 * locals.var_inv_dinf__blk975_dn4)), (((locals.var_zeta2__blk1071_dn6 - locals.var_zeta1__blk1070_dn6) * locals.var_inv_dinf__blk975) + (assign41290_e47183 * locals.var_inv_dinf__blk975_dn6)), (((locals.var_zeta2__blk1071_dn7 - locals.var_zeta1__blk1070_dn7) * locals.var_inv_dinf__blk975) + (assign41290_e47183 * locals.var_inv_dinf__blk975_dn7)), (((locals.var_zeta2__blk1071_dn8 - locals.var_zeta1__blk1070_dn8) * locals.var_inv_dinf__blk975) + (assign41290_e47183 * locals.var_inv_dinf__blk975_dn8)), (((locals.var_zeta2__blk1071_dn9 - locals.var_zeta1__blk1070_dn9) * locals.var_inv_dinf__blk975) + (assign41290_e47183 * locals.var_inv_dinf__blk975_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign41290_e47187;
        locals.var_temp_dn4 = assign41290_e47187_d_n4;
        locals.var_temp_dn6 = assign41290_e47187_d_n6;
        locals.var_temp_dn7 = assign41290_e47187_d_n7;
        locals.var_temp_dn8 = assign41290_e47187_d_n8;
        locals.var_temp_dn9 = assign41290_e47187_d_n9;

        let (assign41300_e47196, assign41300_e47196_d_n4, assign41300_e47196_d_n6, assign41300_e47196_d_n7, assign41300_e47196_d_n8, assign41300_e47196_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
        let assign41300_e47194: f64 = (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906);
        (assign41300_e47194, ((locals.var_zeta1__blk1070_dn4 * locals.var_inv_k1__blk906) + (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906_dn4)), ((locals.var_zeta1__blk1070_dn6 * locals.var_inv_k1__blk906) + (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906_dn6)), ((locals.var_zeta1__blk1070_dn7 * locals.var_inv_k1__blk906) + (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906_dn7)), ((locals.var_zeta1__blk1070_dn8 * locals.var_inv_k1__blk906) + (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906_dn8)), ((locals.var_zeta1__blk1070_dn9 * locals.var_inv_k1__blk906) + (locals.var_zeta1__blk1070 * locals.var_inv_k1__blk906_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign41300_e47196;
        locals.var_temp1_dn4 = assign41300_e47196_d_n4;
        locals.var_temp1_dn6 = assign41300_e47196_d_n6;
        locals.var_temp1_dn7 = assign41300_e47196_d_n7;
        locals.var_temp1_dn8 = assign41300_e47196_d_n8;
        locals.var_temp1_dn9 = assign41300_e47196_d_n9;

        let (assign41310_e47205, assign41310_e47205_d_n4, assign41310_e47205_d_n6, assign41310_e47205_d_n7, assign41310_e47205_d_n8, assign41310_e47205_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
        let assign41310_e47203: f64 = (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907);
        (assign41310_e47203, ((locals.var_zeta2__blk1071_dn4 * locals.var_inv_k2__blk907) + (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907_dn4)), ((locals.var_zeta2__blk1071_dn6 * locals.var_inv_k2__blk907) + (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907_dn6)), ((locals.var_zeta2__blk1071_dn7 * locals.var_inv_k2__blk907) + (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907_dn7)), ((locals.var_zeta2__blk1071_dn8 * locals.var_inv_k2__blk907) + (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907_dn8)), ((locals.var_zeta2__blk1071_dn9 * locals.var_inv_k2__blk907) + (locals.var_zeta2__blk1071 * locals.var_inv_k2__blk907_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign41310_e47205;
        locals.var_temp2_dn4 = assign41310_e47205_d_n4;
        locals.var_temp2_dn6 = assign41310_e47205_d_n6;
        locals.var_temp2_dn7 = assign41310_e47205_d_n7;
        locals.var_temp2_dn8 = assign41310_e47205_d_n8;
        locals.var_temp2_dn9 = assign41310_e47205_d_n9;

        let (assign41320_e47214, assign41320_e47214_d_n4, assign41320_e47214_d_n6, assign41320_e47214_d_n7, assign41320_e47214_d_n8, assign41320_e47214_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
        let assign41320_e47212: f64 = (locals.var_temp1 + locals.var_temp2);
        (assign41320_e47212, (locals.var_temp1_dn4 + locals.var_temp2_dn4), (locals.var_temp1_dn6 + locals.var_temp2_dn6), (locals.var_temp1_dn7 + locals.var_temp2_dn7), (locals.var_temp1_dn8 + locals.var_temp2_dn8), (locals.var_temp1_dn9 + locals.var_temp2_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign41320_e47214;
        locals.var_temp3_dn4 = assign41320_e47214_d_n4;
        locals.var_temp3_dn6 = assign41320_e47214_d_n6;
        locals.var_temp3_dn7 = assign41320_e47214_d_n7;
        locals.var_temp3_dn8 = assign41320_e47214_d_n8;
        locals.var_temp3_dn9 = assign41320_e47214_d_n9;

        let (assign41330_e47231, assign41330_e47231_d_n4, assign41330_e47231_d_n6, assign41330_e47231_d_n7, assign41330_e47231_d_n8, assign41330_e47231_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
        let assign41330_e47223: f64 = (locals.var_s1__blk969 * locals.var_inv_k1__blk906);
        let assign41330_e47226: f64 = (locals.var_s2__blk970 * locals.var_inv_k2__blk907);
        let assign41330_e47227: f64 = (assign41330_e47223 + assign41330_e47226);
        let assign41330_e47228: f64 = (2.0 * assign41330_e47227);
        let assign41330_e47229: f64 = (3.0 + assign41330_e47228);
        (assign41330_e47229, (2.0 * (((locals.var_s1__blk969_dn4 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn4)) + ((locals.var_s2__blk970_dn4 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn4)))), (2.0 * (((locals.var_s1__blk969_dn6 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn6)) + ((locals.var_s2__blk970_dn6 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn6)))), (2.0 * (((locals.var_s1__blk969_dn7 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn7)) + ((locals.var_s2__blk970_dn7 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn7)))), (2.0 * (((locals.var_s1__blk969_dn8 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn8)) + ((locals.var_s2__blk970_dn8 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn8)))), (2.0 * (((locals.var_s1__blk969_dn9 * locals.var_inv_k1__blk906) + (locals.var_s1__blk969 * locals.var_inv_k1__blk906_dn9)) + ((locals.var_s2__blk970_dn9 * locals.var_inv_k2__blk907) + (locals.var_s2__blk970 * locals.var_inv_k2__blk907_dn9)))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign41330_e47231;
        locals.var_temp4_dn4 = assign41330_e47231_d_n4;
        locals.var_temp4_dn6 = assign41330_e47231_d_n6;
        locals.var_temp4_dn7 = assign41330_e47231_d_n7;
        locals.var_temp4_dn8 = assign41330_e47231_d_n8;
        locals.var_temp4_dn9 = assign41330_e47231_d_n9;

        let (assign41340_e47246, assign41340_e47246_d_n4, assign41340_e47246_d_n6, assign41340_e47246_d_n7, assign41340_e47246_d_n8, assign41340_e47246_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
        let assign41340_e47238: f64 = (locals.var_temp2 + locals.var_temp);
        let assign41340_e47241: f64 = (locals.var_temp3 / locals.var_q1chapinf__blk972);
        let assign41340_e47242: f64 = (assign41340_e47238 - assign41340_e47241);
        let assign41340_e47244: f64 = (assign41340_e47242 / locals.var_temp4);
        (assign41340_e47244, (((((locals.var_temp2_dn4 + locals.var_temp_dn4) - (((locals.var_temp3_dn4 * locals.var_q1chapinf__blk972) - (locals.var_temp3 * locals.var_q1chapinf__blk972_dn4)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))) * locals.var_temp4) - (assign41340_e47242 * locals.var_temp4_dn4)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn6 + locals.var_temp_dn6) - (((locals.var_temp3_dn6 * locals.var_q1chapinf__blk972) - (locals.var_temp3 * locals.var_q1chapinf__blk972_dn6)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))) * locals.var_temp4) - (assign41340_e47242 * locals.var_temp4_dn6)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn7 + locals.var_temp_dn7) - (((locals.var_temp3_dn7 * locals.var_q1chapinf__blk972) - (locals.var_temp3 * locals.var_q1chapinf__blk972_dn7)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))) * locals.var_temp4) - (assign41340_e47242 * locals.var_temp4_dn7)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn8 + locals.var_temp_dn8) - (((locals.var_temp3_dn8 * locals.var_q1chapinf__blk972) - (locals.var_temp3 * locals.var_q1chapinf__blk972_dn8)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))) * locals.var_temp4) - (assign41340_e47242 * locals.var_temp4_dn8)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp2_dn9 + locals.var_temp_dn9) - (((locals.var_temp3_dn9 * locals.var_q1chapinf__blk972) - (locals.var_temp3 * locals.var_q1chapinf__blk972_dn9)) / (locals.var_q1chapinf__blk972 * locals.var_q1chapinf__blk972))) * locals.var_temp4) - (assign41340_e47242 * locals.var_temp4_dn9)) / (locals.var_temp4 * locals.var_temp4)),)
    } else {
        (locals.var_ksi1__blk1072, locals.var_ksi1__blk1072_dn4, locals.var_ksi1__blk1072_dn6, locals.var_ksi1__blk1072_dn7, locals.var_ksi1__blk1072_dn8, locals.var_ksi1__blk1072_dn9,)
    }
};
        locals.var_ksi1__blk1072 = assign41340_e47246;
        locals.var_ksi1__blk1072_dn4 = assign41340_e47246_d_n4;
        locals.var_ksi1__blk1072_dn6 = assign41340_e47246_d_n6;
        locals.var_ksi1__blk1072_dn7 = assign41340_e47246_d_n7;
        locals.var_ksi1__blk1072_dn8 = assign41340_e47246_d_n8;
        locals.var_ksi1__blk1072_dn9 = assign41340_e47246_d_n9;

        let (assign41350_e47261, assign41350_e47261_d_n4, assign41350_e47261_d_n6, assign41350_e47261_d_n7, assign41350_e47261_d_n8, assign41350_e47261_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
        let assign41350_e47253: f64 = (locals.var_temp1 - locals.var_temp);
        let assign41350_e47256: f64 = (locals.var_temp3 / locals.var_q2chapinf__blk973);
        let assign41350_e47257: f64 = (assign41350_e47253 - assign41350_e47256);
        let assign41350_e47259: f64 = (assign41350_e47257 / locals.var_temp4);
        (assign41350_e47259, (((((locals.var_temp1_dn4 - locals.var_temp_dn4) - (((locals.var_temp3_dn4 * locals.var_q2chapinf__blk973) - (locals.var_temp3 * locals.var_q2chapinf__blk973_dn4)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973))) * locals.var_temp4) - (assign41350_e47257 * locals.var_temp4_dn4)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn6 - locals.var_temp_dn6) - (((locals.var_temp3_dn6 * locals.var_q2chapinf__blk973) - (locals.var_temp3 * locals.var_q2chapinf__blk973_dn6)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973))) * locals.var_temp4) - (assign41350_e47257 * locals.var_temp4_dn6)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn7 - locals.var_temp_dn7) - (((locals.var_temp3_dn7 * locals.var_q2chapinf__blk973) - (locals.var_temp3 * locals.var_q2chapinf__blk973_dn7)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973))) * locals.var_temp4) - (assign41350_e47257 * locals.var_temp4_dn7)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn8 - locals.var_temp_dn8) - (((locals.var_temp3_dn8 * locals.var_q2chapinf__blk973) - (locals.var_temp3 * locals.var_q2chapinf__blk973_dn8)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973))) * locals.var_temp4) - (assign41350_e47257 * locals.var_temp4_dn8)) / (locals.var_temp4 * locals.var_temp4)), (((((locals.var_temp1_dn9 - locals.var_temp_dn9) - (((locals.var_temp3_dn9 * locals.var_q2chapinf__blk973) - (locals.var_temp3 * locals.var_q2chapinf__blk973_dn9)) / (locals.var_q2chapinf__blk973 * locals.var_q2chapinf__blk973))) * locals.var_temp4) - (assign41350_e47257 * locals.var_temp4_dn9)) / (locals.var_temp4 * locals.var_temp4)),)
    } else {
        (locals.var_ksi2__blk1073, locals.var_ksi2__blk1073_dn4, locals.var_ksi2__blk1073_dn6, locals.var_ksi2__blk1073_dn7, locals.var_ksi2__blk1073_dn8, locals.var_ksi2__blk1073_dn9,)
    }
};
        locals.var_ksi2__blk1073 = assign41350_e47261;
        locals.var_ksi2__blk1073_dn4 = assign41350_e47261_d_n4;
        locals.var_ksi2__blk1073_dn6 = assign41350_e47261_d_n6;
        locals.var_ksi2__blk1073_dn7 = assign41350_e47261_d_n7;
        locals.var_ksi2__blk1073_dn8 = assign41350_e47261_d_n8;
        locals.var_ksi2__blk1073_dn9 = assign41350_e47261_d_n9;

        let (assign41360_e47275, assign41360_e47275_d_n4, assign41360_e47275_d_n6, assign41360_e47275_d_n7, assign41360_e47275_d_n8, assign41360_e47275_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
        let assign41360_e47267: f64 = (-locals.var_q1chapinf__blk972);
        let assign41360_e47270: f64 = (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972);
        let assign41360_e47272: f64 = (assign41360_e47270 + locals.var_inv_dinf__blk975);
        let assign41360_e47273: f64 = (assign41360_e47267 * assign41360_e47272);
        (assign41360_e47273, (((-locals.var_q1chapinf__blk972_dn4) * assign41360_e47272) + (assign41360_e47267 * (((locals.var_ksi1__blk1072_dn4 * locals.var_q1chapinf__blk972) + (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972_dn4)) + locals.var_inv_dinf__blk975_dn4))), (((-locals.var_q1chapinf__blk972_dn6) * assign41360_e47272) + (assign41360_e47267 * (((locals.var_ksi1__blk1072_dn6 * locals.var_q1chapinf__blk972) + (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972_dn6)) + locals.var_inv_dinf__blk975_dn6))), (((-locals.var_q1chapinf__blk972_dn7) * assign41360_e47272) + (assign41360_e47267 * (((locals.var_ksi1__blk1072_dn7 * locals.var_q1chapinf__blk972) + (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972_dn7)) + locals.var_inv_dinf__blk975_dn7))), (((-locals.var_q1chapinf__blk972_dn8) * assign41360_e47272) + (assign41360_e47267 * (((locals.var_ksi1__blk1072_dn8 * locals.var_q1chapinf__blk972) + (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972_dn8)) + locals.var_inv_dinf__blk975_dn8))), (((-locals.var_q1chapinf__blk972_dn9) * assign41360_e47272) + (assign41360_e47267 * (((locals.var_ksi1__blk1072_dn9 * locals.var_q1chapinf__blk972) + (locals.var_ksi1__blk1072 * locals.var_q1chapinf__blk972_dn9)) + locals.var_inv_dinf__blk975_dn9))),)
    } else {
        (locals.var_inv_k1h1_0__blk1066, locals.var_inv_k1h1_0__blk1066_dn4, locals.var_inv_k1h1_0__blk1066_dn6, locals.var_inv_k1h1_0__blk1066_dn7, locals.var_inv_k1h1_0__blk1066_dn8, locals.var_inv_k1h1_0__blk1066_dn9,)
    }
};
        locals.var_inv_k1h1_0__blk1066 = assign41360_e47275;
        locals.var_inv_k1h1_0__blk1066_dn4 = assign41360_e47275_d_n4;
        locals.var_inv_k1h1_0__blk1066_dn6 = assign41360_e47275_d_n6;
        locals.var_inv_k1h1_0__blk1066_dn7 = assign41360_e47275_d_n7;
        locals.var_inv_k1h1_0__blk1066_dn8 = assign41360_e47275_d_n8;
        locals.var_inv_k1h1_0__blk1066_dn9 = assign41360_e47275_d_n9;

        let (assign41370_e47289, assign41370_e47289_d_n4, assign41370_e47289_d_n6, assign41370_e47289_d_n7, assign41370_e47289_d_n8, assign41370_e47289_d_n9,) = {
    if ((locals.var_guard1080 != 0.0) && (locals.var_guard1232 == 0.0)) {
        let assign41370_e47281: f64 = (-locals.var_q2chapinf__blk973);
        let assign41370_e47284: f64 = (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973);
        let assign41370_e47286: f64 = (assign41370_e47284 + locals.var_inv_dinf__blk975);
        let assign41370_e47287: f64 = (assign41370_e47281 * assign41370_e47286);
        (assign41370_e47287, (((-locals.var_q2chapinf__blk973_dn4) * assign41370_e47286) + (assign41370_e47281 * (((locals.var_ksi2__blk1073_dn4 * locals.var_q2chapinf__blk973) + (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973_dn4)) + locals.var_inv_dinf__blk975_dn4))), (((-locals.var_q2chapinf__blk973_dn6) * assign41370_e47286) + (assign41370_e47281 * (((locals.var_ksi2__blk1073_dn6 * locals.var_q2chapinf__blk973) + (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973_dn6)) + locals.var_inv_dinf__blk975_dn6))), (((-locals.var_q2chapinf__blk973_dn7) * assign41370_e47286) + (assign41370_e47281 * (((locals.var_ksi2__blk1073_dn7 * locals.var_q2chapinf__blk973) + (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973_dn7)) + locals.var_inv_dinf__blk975_dn7))), (((-locals.var_q2chapinf__blk973_dn8) * assign41370_e47286) + (assign41370_e47281 * (((locals.var_ksi2__blk1073_dn8 * locals.var_q2chapinf__blk973) + (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973_dn8)) + locals.var_inv_dinf__blk975_dn8))), (((-locals.var_q2chapinf__blk973_dn9) * assign41370_e47286) + (assign41370_e47281 * (((locals.var_ksi2__blk1073_dn9 * locals.var_q2chapinf__blk973) + (locals.var_ksi2__blk1073 * locals.var_q2chapinf__blk973_dn9)) + locals.var_inv_dinf__blk975_dn9))),)
    } else {
        (locals.var_inv_k2h2_0__blk1069, locals.var_inv_k2h2_0__blk1069_dn4, locals.var_inv_k2h2_0__blk1069_dn6, locals.var_inv_k2h2_0__blk1069_dn7, locals.var_inv_k2h2_0__blk1069_dn8, locals.var_inv_k2h2_0__blk1069_dn9,)
    }
};
        locals.var_inv_k2h2_0__blk1069 = assign41370_e47289;
        locals.var_inv_k2h2_0__blk1069_dn4 = assign41370_e47289_d_n4;
        locals.var_inv_k2h2_0__blk1069_dn6 = assign41370_e47289_d_n6;
        locals.var_inv_k2h2_0__blk1069_dn7 = assign41370_e47289_d_n7;
        locals.var_inv_k2h2_0__blk1069_dn8 = assign41370_e47289_d_n8;
        locals.var_inv_k2h2_0__blk1069_dn9 = assign41370_e47289_d_n9;

        let (assign41380_e47295, assign41380_e47295_d_n4, assign41380_e47295_d_n6, assign41380_e47295_d_n7, assign41380_e47295_d_n8, assign41380_e47295_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign41380_e47293: f64 = (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053);
        (assign41380_e47293, ((locals.var_inv_k1h1_0__blk1066_dn4 * locals.var_hsat__blk1053) + (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053_dn4)), ((locals.var_inv_k1h1_0__blk1066_dn6 * locals.var_hsat__blk1053) + (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053_dn6)), ((locals.var_inv_k1h1_0__blk1066_dn7 * locals.var_hsat__blk1053) + (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053_dn7)), ((locals.var_inv_k1h1_0__blk1066_dn8 * locals.var_hsat__blk1053) + (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053_dn8)), ((locals.var_inv_k1h1_0__blk1066_dn9 * locals.var_hsat__blk1053) + (locals.var_inv_k1h1_0__blk1066 * locals.var_hsat__blk1053_dn9)),)
    } else {
        (locals.var_inv_k1h1__blk1074, locals.var_inv_k1h1__blk1074_dn4, locals.var_inv_k1h1__blk1074_dn6, locals.var_inv_k1h1__blk1074_dn7, locals.var_inv_k1h1__blk1074_dn8, locals.var_inv_k1h1__blk1074_dn9,)
    }
};
        locals.var_inv_k1h1__blk1074 = assign41380_e47295;
        locals.var_inv_k1h1__blk1074_dn4 = assign41380_e47295_d_n4;
        locals.var_inv_k1h1__blk1074_dn6 = assign41380_e47295_d_n6;
        locals.var_inv_k1h1__blk1074_dn7 = assign41380_e47295_d_n7;
        locals.var_inv_k1h1__blk1074_dn8 = assign41380_e47295_d_n8;
        locals.var_inv_k1h1__blk1074_dn9 = assign41380_e47295_d_n9;

        let (assign41390_e47301, assign41390_e47301_d_n4, assign41390_e47301_d_n6, assign41390_e47301_d_n7, assign41390_e47301_d_n8, assign41390_e47301_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign41390_e47299: f64 = (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053);
        (assign41390_e47299, ((locals.var_inv_k2h2_0__blk1069_dn4 * locals.var_hsat__blk1053) + (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053_dn4)), ((locals.var_inv_k2h2_0__blk1069_dn6 * locals.var_hsat__blk1053) + (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053_dn6)), ((locals.var_inv_k2h2_0__blk1069_dn7 * locals.var_hsat__blk1053) + (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053_dn7)), ((locals.var_inv_k2h2_0__blk1069_dn8 * locals.var_hsat__blk1053) + (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053_dn8)), ((locals.var_inv_k2h2_0__blk1069_dn9 * locals.var_hsat__blk1053) + (locals.var_inv_k2h2_0__blk1069 * locals.var_hsat__blk1053_dn9)),)
    } else {
        (locals.var_inv_k2h2__blk1075, locals.var_inv_k2h2__blk1075_dn4, locals.var_inv_k2h2__blk1075_dn6, locals.var_inv_k2h2__blk1075_dn7, locals.var_inv_k2h2__blk1075_dn8, locals.var_inv_k2h2__blk1075_dn9,)
    }
};
        locals.var_inv_k2h2__blk1075 = assign41390_e47301;
        locals.var_inv_k2h2__blk1075_dn4 = assign41390_e47301_d_n4;
        locals.var_inv_k2h2__blk1075_dn6 = assign41390_e47301_d_n6;
        locals.var_inv_k2h2__blk1075_dn7 = assign41390_e47301_d_n7;
        locals.var_inv_k2h2__blk1075_dn8 = assign41390_e47301_d_n8;
        locals.var_inv_k2h2__blk1075_dn9 = assign41390_e47301_d_n9;

        let (assign41400_e47309, assign41400_e47309_d_n4, assign41400_e47309_d_n6, assign41400_e47309_d_n7, assign41400_e47309_d_n8, assign41400_e47309_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign41400_e47306: f64 = (locals.var_k1q1d__blk1004 - locals.var_k1q1s__blk939);
        let assign41400_e47307: f64 = (0.5 * assign41400_e47306);
        (assign41400_e47307, (0.5 * (locals.var_k1q1d__blk1004_dn4 - locals.var_k1q1s__blk939_dn4)), (0.5 * (locals.var_k1q1d__blk1004_dn6 - locals.var_k1q1s__blk939_dn6)), (0.5 * (locals.var_k1q1d__blk1004_dn7 - locals.var_k1q1s__blk939_dn7)), (0.5 * (locals.var_k1q1d__blk1004_dn8 - locals.var_k1q1s__blk939_dn8)), (0.5 * (locals.var_k1q1d__blk1004_dn9 - locals.var_k1q1s__blk939_dn9)),)
    } else {
        (locals.var_delta_k1q1__blk1076, locals.var_delta_k1q1__blk1076_dn4, locals.var_delta_k1q1__blk1076_dn6, locals.var_delta_k1q1__blk1076_dn7, locals.var_delta_k1q1__blk1076_dn8, locals.var_delta_k1q1__blk1076_dn9,)
    }
};
        locals.var_delta_k1q1__blk1076 = assign41400_e47309;
        locals.var_delta_k1q1__blk1076_dn4 = assign41400_e47309_d_n4;
        locals.var_delta_k1q1__blk1076_dn6 = assign41400_e47309_d_n6;
        locals.var_delta_k1q1__blk1076_dn7 = assign41400_e47309_d_n7;
        locals.var_delta_k1q1__blk1076_dn8 = assign41400_e47309_d_n8;
        locals.var_delta_k1q1__blk1076_dn9 = assign41400_e47309_d_n9;

        let (assign41410_e47317, assign41410_e47317_d_n4, assign41410_e47317_d_n6, assign41410_e47317_d_n7, assign41410_e47317_d_n8, assign41410_e47317_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign41410_e47314: f64 = (locals.var_k2q2d__blk1005 - locals.var_k2q2s__blk940);
        let assign41410_e47315: f64 = (0.5 * assign41410_e47314);
        (assign41410_e47315, (0.5 * (locals.var_k2q2d__blk1005_dn4 - locals.var_k2q2s__blk940_dn4)), (0.5 * (locals.var_k2q2d__blk1005_dn6 - locals.var_k2q2s__blk940_dn6)), (0.5 * (locals.var_k2q2d__blk1005_dn7 - locals.var_k2q2s__blk940_dn7)), (0.5 * (locals.var_k2q2d__blk1005_dn8 - locals.var_k2q2s__blk940_dn8)), (0.5 * (locals.var_k2q2d__blk1005_dn9 - locals.var_k2q2s__blk940_dn9)),)
    } else {
        (locals.var_delta_k2q2__blk1077, locals.var_delta_k2q2__blk1077_dn4, locals.var_delta_k2q2__blk1077_dn6, locals.var_delta_k2q2__blk1077_dn7, locals.var_delta_k2q2__blk1077_dn8, locals.var_delta_k2q2__blk1077_dn9,)
    }
};
        locals.var_delta_k2q2__blk1077 = assign41410_e47317;
        locals.var_delta_k2q2__blk1077_dn4 = assign41410_e47317_d_n4;
        locals.var_delta_k2q2__blk1077_dn6 = assign41410_e47317_d_n6;
        locals.var_delta_k2q2__blk1077_dn7 = assign41410_e47317_d_n7;
        locals.var_delta_k2q2__blk1077_dn8 = assign41410_e47317_d_n8;
        locals.var_delta_k2q2__blk1077_dn9 = assign41410_e47317_d_n9;

        let (assign41420_e47323, assign41420_e47323_d_n4, assign41420_e47323_d_n6, assign41420_e47323_d_n7, assign41420_e47323_d_n8, assign41420_e47323_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign41420_e47321: f64 = (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074);
        (assign41420_e47321, ((locals.var_delta_k1q1__blk1076_dn4 * locals.var_inv_k1h1__blk1074) + (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074_dn4)), ((locals.var_delta_k1q1__blk1076_dn6 * locals.var_inv_k1h1__blk1074) + (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074_dn6)), ((locals.var_delta_k1q1__blk1076_dn7 * locals.var_inv_k1h1__blk1074) + (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074_dn7)), ((locals.var_delta_k1q1__blk1076_dn8 * locals.var_inv_k1h1__blk1074) + (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074_dn8)), ((locals.var_delta_k1q1__blk1076_dn9 * locals.var_inv_k1h1__blk1074) + (locals.var_delta_k1q1__blk1076 * locals.var_inv_k1h1__blk1074_dn9)),)
    } else {
        (locals.var_prod1__blk1078, locals.var_prod1__blk1078_dn4, locals.var_prod1__blk1078_dn6, locals.var_prod1__blk1078_dn7, locals.var_prod1__blk1078_dn8, locals.var_prod1__blk1078_dn9,)
    }
};
        locals.var_prod1__blk1078 = assign41420_e47323;
        locals.var_prod1__blk1078_dn4 = assign41420_e47323_d_n4;
        locals.var_prod1__blk1078_dn6 = assign41420_e47323_d_n6;
        locals.var_prod1__blk1078_dn7 = assign41420_e47323_d_n7;
        locals.var_prod1__blk1078_dn8 = assign41420_e47323_d_n8;
        locals.var_prod1__blk1078_dn9 = assign41420_e47323_d_n9;

        let (assign41430_e47329, assign41430_e47329_d_n4, assign41430_e47329_d_n6, assign41430_e47329_d_n7, assign41430_e47329_d_n8, assign41430_e47329_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        let assign41430_e47327: f64 = (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075);
        (assign41430_e47327, ((locals.var_delta_k2q2__blk1077_dn4 * locals.var_inv_k2h2__blk1075) + (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075_dn4)), ((locals.var_delta_k2q2__blk1077_dn6 * locals.var_inv_k2h2__blk1075) + (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075_dn6)), ((locals.var_delta_k2q2__blk1077_dn7 * locals.var_inv_k2h2__blk1075) + (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075_dn7)), ((locals.var_delta_k2q2__blk1077_dn8 * locals.var_inv_k2h2__blk1075) + (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075_dn8)), ((locals.var_delta_k2q2__blk1077_dn9 * locals.var_inv_k2h2__blk1075) + (locals.var_delta_k2q2__blk1077 * locals.var_inv_k2h2__blk1075_dn9)),)
    } else {
        (locals.var_prod2__blk1079, locals.var_prod2__blk1079_dn4, locals.var_prod2__blk1079_dn6, locals.var_prod2__blk1079_dn7, locals.var_prod2__blk1079_dn8, locals.var_prod2__blk1079_dn9,)
    }
};
        locals.var_prod2__blk1079 = assign41430_e47329;
        locals.var_prod2__blk1079_dn4 = assign41430_e47329_d_n4;
        locals.var_prod2__blk1079_dn6 = assign41430_e47329_d_n6;
        locals.var_prod2__blk1079_dn7 = assign41430_e47329_d_n7;
        locals.var_prod2__blk1079_dn8 = assign41430_e47329_d_n8;
        locals.var_prod2__blk1079_dn9 = assign41430_e47329_d_n9;

        let (assign41440_e47333, assign41440_e47333_d_n4, assign41440_e47333_d_n6, assign41440_e47333_d_n7, assign41440_e47333_d_n8, assign41440_e47333_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_xg20shift__blk900, locals.var_xg20shift__blk900_dn4, locals.var_xg20shift__blk900_dn6, locals.var_xg20shift__blk900_dn7, locals.var_xg20shift__blk900_dn8, locals.var_xg20shift__blk900_dn9,)
    } else {
        (locals.var_xg20shift_ac, locals.var_xg20shift_ac_dn4, locals.var_xg20shift_ac_dn6, locals.var_xg20shift_ac_dn7, locals.var_xg20shift_ac_dn8, locals.var_xg20shift_ac_dn9,)
    }
};
        locals.var_xg20shift_ac = assign41440_e47333;
        locals.var_xg20shift_ac_dn4 = assign41440_e47333_d_n4;
        locals.var_xg20shift_ac_dn6 = assign41440_e47333_d_n6;
        locals.var_xg20shift_ac_dn7 = assign41440_e47333_d_n7;
        locals.var_xg20shift_ac_dn8 = assign41440_e47333_d_n8;
        locals.var_xg20shift_ac_dn9 = assign41440_e47333_d_n9;

        let (assign41450_e47337, assign41450_e47337_d_n4, assign41450_e47337_d_n6, assign41450_e47337_d_n7, assign41450_e47337_d_n8, assign41450_e47337_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_diff_min__blk904, locals.var_diff_min__blk904_dn4, locals.var_diff_min__blk904_dn6, locals.var_diff_min__blk904_dn7, locals.var_diff_min__blk904_dn8, locals.var_diff_min__blk904_dn9,)
    } else {
        (locals.var_diff_min_ac, locals.var_diff_min_ac_dn4, locals.var_diff_min_ac_dn6, locals.var_diff_min_ac_dn7, locals.var_diff_min_ac_dn8, locals.var_diff_min_ac_dn9,)
    }
};
        locals.var_diff_min_ac = assign41450_e47337;
        locals.var_diff_min_ac_dn4 = assign41450_e47337_d_n4;
        locals.var_diff_min_ac_dn6 = assign41450_e47337_d_n6;
        locals.var_diff_min_ac_dn7 = assign41450_e47337_d_n7;
        locals.var_diff_min_ac_dn8 = assign41450_e47337_d_n8;
        locals.var_diff_min_ac_dn9 = assign41450_e47337_d_n9;

        let (assign41460_e47341, assign41460_e47341_d_n4, assign41460_e47341_d_n6, assign41460_e47341_d_n7, assign41460_e47341_d_n8, assign41460_e47341_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_a0__blk905, locals.var_a0__blk905_dn4, locals.var_a0__blk905_dn6, locals.var_a0__blk905_dn7, locals.var_a0__blk905_dn8, locals.var_a0__blk905_dn9,)
    } else {
        (locals.var_a0_ac, locals.var_a0_ac_dn4, locals.var_a0_ac_dn6, locals.var_a0_ac_dn7, locals.var_a0_ac_dn8, locals.var_a0_ac_dn9,)
    }
};
        locals.var_a0_ac = assign41460_e47341;
        locals.var_a0_ac_dn4 = assign41460_e47341_d_n4;
        locals.var_a0_ac_dn6 = assign41460_e47341_d_n6;
        locals.var_a0_ac_dn7 = assign41460_e47341_d_n7;
        locals.var_a0_ac_dn8 = assign41460_e47341_d_n8;
        locals.var_a0_ac_dn9 = assign41460_e47341_d_n9;

        let (assign41470_e47345, assign41470_e47345_d_n4, assign41470_e47345_d_n6, assign41470_e47345_d_n7, assign41470_e47345_d_n8, assign41470_e47345_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_inv_k1__blk906, locals.var_inv_k1__blk906_dn4, locals.var_inv_k1__blk906_dn6, locals.var_inv_k1__blk906_dn7, locals.var_inv_k1__blk906_dn8, locals.var_inv_k1__blk906_dn9,)
    } else {
        (locals.var_inv_k1_ac, locals.var_inv_k1_ac_dn4, locals.var_inv_k1_ac_dn6, locals.var_inv_k1_ac_dn7, locals.var_inv_k1_ac_dn8, locals.var_inv_k1_ac_dn9,)
    }
};
        locals.var_inv_k1_ac = assign41470_e47345;
        locals.var_inv_k1_ac_dn4 = assign41470_e47345_d_n4;
        locals.var_inv_k1_ac_dn6 = assign41470_e47345_d_n6;
        locals.var_inv_k1_ac_dn7 = assign41470_e47345_d_n7;
        locals.var_inv_k1_ac_dn8 = assign41470_e47345_d_n8;
        locals.var_inv_k1_ac_dn9 = assign41470_e47345_d_n9;

        let (assign41480_e47349, assign41480_e47349_d_n4, assign41480_e47349_d_n6, assign41480_e47349_d_n7, assign41480_e47349_d_n8, assign41480_e47349_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_inv_k2__blk907, locals.var_inv_k2__blk907_dn4, locals.var_inv_k2__blk907_dn6, locals.var_inv_k2__blk907_dn7, locals.var_inv_k2__blk907_dn8, locals.var_inv_k2__blk907_dn9,)
    } else {
        (locals.var_inv_k2_ac, locals.var_inv_k2_ac_dn4, locals.var_inv_k2_ac_dn6, locals.var_inv_k2_ac_dn7, locals.var_inv_k2_ac_dn8, locals.var_inv_k2_ac_dn9,)
    }
};
        locals.var_inv_k2_ac = assign41480_e47349;
        locals.var_inv_k2_ac_dn4 = assign41480_e47349_d_n4;
        locals.var_inv_k2_ac_dn6 = assign41480_e47349_d_n6;
        locals.var_inv_k2_ac_dn7 = assign41480_e47349_d_n7;
        locals.var_inv_k2_ac_dn8 = assign41480_e47349_d_n8;
        locals.var_inv_k2_ac_dn9 = assign41480_e47349_d_n9;

        let (assign41490_e47353, assign41490_e47353_d_n4, assign41490_e47353_d_n6, assign41490_e47353_d_n7, assign41490_e47353_d_n8, assign41490_e47353_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_keq__blk934, locals.var_keq__blk934_dn4, locals.var_keq__blk934_dn6, locals.var_keq__blk934_dn7, locals.var_keq__blk934_dn8, locals.var_keq__blk934_dn9,)
    } else {
        (locals.var_keq_ac, locals.var_keq_ac_dn4, locals.var_keq_ac_dn6, locals.var_keq_ac_dn7, locals.var_keq_ac_dn8, locals.var_keq_ac_dn9,)
    }
};
        locals.var_keq_ac = assign41490_e47353;
        locals.var_keq_ac_dn4 = assign41490_e47353_d_n4;
        locals.var_keq_ac_dn6 = assign41490_e47353_d_n6;
        locals.var_keq_ac_dn7 = assign41490_e47353_d_n7;
        locals.var_keq_ac_dn8 = assign41490_e47353_d_n8;
        locals.var_keq_ac_dn9 = assign41490_e47353_d_n9;

        let (assign41500_e47357, assign41500_e47357_d_n4, assign41500_e47357_d_n6, assign41500_e47357_d_n7, assign41500_e47357_d_n8, assign41500_e47357_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_dx_wi__blk935, locals.var_dx_wi__blk935_dn4, locals.var_dx_wi__blk935_dn6, locals.var_dx_wi__blk935_dn7, locals.var_dx_wi__blk935_dn8, locals.var_dx_wi__blk935_dn9,)
    } else {
        (locals.var_dx_wi_ac, locals.var_dx_wi_ac_dn4, locals.var_dx_wi_ac_dn6, locals.var_dx_wi_ac_dn7, locals.var_dx_wi_ac_dn8, locals.var_dx_wi_ac_dn9,)
    }
};
        locals.var_dx_wi_ac = assign41500_e47357;
        locals.var_dx_wi_ac_dn4 = assign41500_e47357_d_n4;
        locals.var_dx_wi_ac_dn6 = assign41500_e47357_d_n6;
        locals.var_dx_wi_ac_dn7 = assign41500_e47357_d_n7;
        locals.var_dx_wi_ac_dn8 = assign41500_e47357_d_n8;
        locals.var_dx_wi_ac_dn9 = assign41500_e47357_d_n9;

        let (assign41510_e47361, assign41510_e47361_d_n4, assign41510_e47361_d_n6, assign41510_e47361_d_n7, assign41510_e47361_d_n8, assign41510_e47361_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_csiprime__blk919, locals.var_csiprime__blk919_dn4, locals.var_csiprime__blk919_dn6, locals.var_csiprime__blk919_dn7, locals.var_csiprime__blk919_dn8, locals.var_csiprime__blk919_dn9,)
    } else {
        (locals.var_csiprime_ac, locals.var_csiprime_ac_dn4, locals.var_csiprime_ac_dn6, locals.var_csiprime_ac_dn7, locals.var_csiprime_ac_dn8, locals.var_csiprime_ac_dn9,)
    }
};
        locals.var_csiprime_ac = assign41510_e47361;
        locals.var_csiprime_ac_dn4 = assign41510_e47361_d_n4;
        locals.var_csiprime_ac_dn6 = assign41510_e47361_d_n6;
        locals.var_csiprime_ac_dn7 = assign41510_e47361_d_n7;
        locals.var_csiprime_ac_dn8 = assign41510_e47361_d_n8;
        locals.var_csiprime_ac_dn9 = assign41510_e47361_d_n9;

        let (assign41520_e47365, assign41520_e47365_d_n4, assign41520_e47365_d_n6, assign41520_e47365_d_n7, assign41520_e47365_d_n8, assign41520_e47365_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_dx_wi_1d__blk918, locals.var_dx_wi_1d__blk918_dn4, locals.var_dx_wi_1d__blk918_dn6, locals.var_dx_wi_1d__blk918_dn7, locals.var_dx_wi_1d__blk918_dn8, locals.var_dx_wi_1d__blk918_dn9,)
    } else {
        (locals.var_dx_wi_1d_ac, locals.var_dx_wi_1d_ac_dn4, locals.var_dx_wi_1d_ac_dn6, locals.var_dx_wi_1d_ac_dn7, locals.var_dx_wi_1d_ac_dn8, locals.var_dx_wi_1d_ac_dn9,)
    }
};
        locals.var_dx_wi_1d_ac = assign41520_e47365;
        locals.var_dx_wi_1d_ac_dn4 = assign41520_e47365_d_n4;
        locals.var_dx_wi_1d_ac_dn6 = assign41520_e47365_d_n6;
        locals.var_dx_wi_1d_ac_dn7 = assign41520_e47365_d_n7;
        locals.var_dx_wi_1d_ac_dn8 = assign41520_e47365_d_n8;
        locals.var_dx_wi_1d_ac_dn9 = assign41520_e47365_d_n9;

        let (assign41530_e47369, assign41530_e47369_d_n4, assign41530_e47369_d_n6, assign41530_e47369_d_n7, assign41530_e47369_d_n8, assign41530_e47369_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_dleff__blk922, locals.var_dleff__blk922_dn4, locals.var_dleff__blk922_dn6, locals.var_dleff__blk922_dn7, locals.var_dleff__blk922_dn8, locals.var_dleff__blk922_dn9,)
    } else {
        (locals.var_dleff_ac, locals.var_dleff_ac_dn4, locals.var_dleff_ac_dn6, locals.var_dleff_ac_dn7, locals.var_dleff_ac_dn8, locals.var_dleff_ac_dn9,)
    }
};
        locals.var_dleff_ac = assign41530_e47369;
        locals.var_dleff_ac_dn4 = assign41530_e47369_d_n4;
        locals.var_dleff_ac_dn6 = assign41530_e47369_d_n6;
        locals.var_dleff_ac_dn7 = assign41530_e47369_d_n7;
        locals.var_dleff_ac_dn8 = assign41530_e47369_d_n8;
        locals.var_dleff_ac_dn9 = assign41530_e47369_d_n9;

    }

    pub(super) fn stamp_transient_block_113(
        locals: &mut StampLocals,
    ) {
        let (assign41540_e47373, assign41540_e47373_d_n4, assign41540_e47373_d_n6, assign41540_e47373_d_n7, assign41540_e47373_d_n8, assign41540_e47373_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_xedge__blk923, locals.var_xedge__blk923_dn4, locals.var_xedge__blk923_dn6, locals.var_xedge__blk923_dn7, locals.var_xedge__blk923_dn8, locals.var_xedge__blk923_dn9,)
    } else {
        (locals.var_xedge_ac, locals.var_xedge_ac_dn4, locals.var_xedge_ac_dn6, locals.var_xedge_ac_dn7, locals.var_xedge_ac_dn8, locals.var_xedge_ac_dn9,)
    }
};
        locals.var_xedge_ac = assign41540_e47373;
        locals.var_xedge_ac_dn4 = assign41540_e47373_d_n4;
        locals.var_xedge_ac_dn6 = assign41540_e47373_d_n6;
        locals.var_xedge_ac_dn7 = assign41540_e47373_d_n7;
        locals.var_xedge_ac_dn8 = assign41540_e47373_d_n8;
        locals.var_xedge_ac_dn9 = assign41540_e47373_d_n9;

        let (assign41550_e47377, assign41550_e47377_d_n4, assign41550_e47377_d_n6, assign41550_e47377_d_n7, assign41550_e47377_d_n8, assign41550_e47377_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_sce1__blk924, locals.var_sce1__blk924_dn4, locals.var_sce1__blk924_dn6, locals.var_sce1__blk924_dn7, locals.var_sce1__blk924_dn8, locals.var_sce1__blk924_dn9,)
    } else {
        (locals.var_sce1_ac, locals.var_sce1_ac_dn4, locals.var_sce1_ac_dn6, locals.var_sce1_ac_dn7, locals.var_sce1_ac_dn8, locals.var_sce1_ac_dn9,)
    }
};
        locals.var_sce1_ac = assign41550_e47377;
        locals.var_sce1_ac_dn4 = assign41550_e47377_d_n4;
        locals.var_sce1_ac_dn6 = assign41550_e47377_d_n6;
        locals.var_sce1_ac_dn7 = assign41550_e47377_d_n7;
        locals.var_sce1_ac_dn8 = assign41550_e47377_d_n8;
        locals.var_sce1_ac_dn9 = assign41550_e47377_d_n9;

        let (assign41560_e47381, assign41560_e47381_d_n4, assign41560_e47381_d_n6, assign41560_e47381_d_n7, assign41560_e47381_d_n8, assign41560_e47381_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_sce2__blk925, locals.var_sce2__blk925_dn4, locals.var_sce2__blk925_dn6, locals.var_sce2__blk925_dn7, locals.var_sce2__blk925_dn8, locals.var_sce2__blk925_dn9,)
    } else {
        (locals.var_sce2_ac, locals.var_sce2_ac_dn4, locals.var_sce2_ac_dn6, locals.var_sce2_ac_dn7, locals.var_sce2_ac_dn8, locals.var_sce2_ac_dn9,)
    }
};
        locals.var_sce2_ac = assign41560_e47381;
        locals.var_sce2_ac_dn4 = assign41560_e47381_d_n4;
        locals.var_sce2_ac_dn6 = assign41560_e47381_d_n6;
        locals.var_sce2_ac_dn7 = assign41560_e47381_d_n7;
        locals.var_sce2_ac_dn8 = assign41560_e47381_d_n8;
        locals.var_sce2_ac_dn9 = assign41560_e47381_d_n9;

        let (assign41570_e47385, assign41570_e47385_d_n4, assign41570_e47385_d_n6, assign41570_e47385_d_n7, assign41570_e47385_d_n8, assign41570_e47385_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_dxg1_dibl__blk926, locals.var_dxg1_dibl__blk926_dn4, locals.var_dxg1_dibl__blk926_dn6, locals.var_dxg1_dibl__blk926_dn7, locals.var_dxg1_dibl__blk926_dn8, locals.var_dxg1_dibl__blk926_dn9,)
    } else {
        (locals.var_dxg1_dibl_ac, locals.var_dxg1_dibl_ac_dn4, locals.var_dxg1_dibl_ac_dn6, locals.var_dxg1_dibl_ac_dn7, locals.var_dxg1_dibl_ac_dn8, locals.var_dxg1_dibl_ac_dn9,)
    }
};
        locals.var_dxg1_dibl_ac = assign41570_e47385;
        locals.var_dxg1_dibl_ac_dn4 = assign41570_e47385_d_n4;
        locals.var_dxg1_dibl_ac_dn6 = assign41570_e47385_d_n6;
        locals.var_dxg1_dibl_ac_dn7 = assign41570_e47385_d_n7;
        locals.var_dxg1_dibl_ac_dn8 = assign41570_e47385_d_n8;
        locals.var_dxg1_dibl_ac_dn9 = assign41570_e47385_d_n9;

        let (assign41580_e47389, assign41580_e47389_d_n4, assign41580_e47389_d_n6, assign41580_e47389_d_n7, assign41580_e47389_d_n8, assign41580_e47389_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_xg2__blk929, locals.var_xg2__blk929_dn4, locals.var_xg2__blk929_dn6, locals.var_xg2__blk929_dn7, locals.var_xg2__blk929_dn8, locals.var_xg2__blk929_dn9,)
    } else {
        (locals.var_xg2_ac, locals.var_xg2_ac_dn4, locals.var_xg2_ac_dn6, locals.var_xg2_ac_dn7, locals.var_xg2_ac_dn8, locals.var_xg2_ac_dn9,)
    }
};
        locals.var_xg2_ac = assign41580_e47389;
        locals.var_xg2_ac_dn4 = assign41580_e47389_d_n4;
        locals.var_xg2_ac_dn6 = assign41580_e47389_d_n6;
        locals.var_xg2_ac_dn7 = assign41580_e47389_d_n7;
        locals.var_xg2_ac_dn8 = assign41580_e47389_d_n8;
        locals.var_xg2_ac_dn9 = assign41580_e47389_d_n9;

        let (assign41590_e47393, assign41590_e47393_d_n4, assign41590_e47393_d_n6, assign41590_e47393_d_n7, assign41590_e47393_d_n8, assign41590_e47393_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_xg2x__blk931, locals.var_xg2x__blk931_dn4, locals.var_xg2x__blk931_dn6, locals.var_xg2x__blk931_dn7, locals.var_xg2x__blk931_dn8, locals.var_xg2x__blk931_dn9,)
    } else {
        (locals.var_xg2x_ac, locals.var_xg2x_ac_dn4, locals.var_xg2x_ac_dn6, locals.var_xg2x_ac_dn7, locals.var_xg2x_ac_dn8, locals.var_xg2x_ac_dn9,)
    }
};
        locals.var_xg2x_ac = assign41590_e47393;
        locals.var_xg2x_ac_dn4 = assign41590_e47393_d_n4;
        locals.var_xg2x_ac_dn6 = assign41590_e47393_d_n6;
        locals.var_xg2x_ac_dn7 = assign41590_e47393_d_n7;
        locals.var_xg2x_ac_dn8 = assign41590_e47393_d_n8;
        locals.var_xg2x_ac_dn9 = assign41590_e47393_d_n9;

        let (assign41600_e47397, assign41600_e47397_d_n4, assign41600_e47397_d_n6, assign41600_e47397_d_n7, assign41600_e47397_d_n8, assign41600_e47397_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_k1__blk932, locals.var_k1__blk932_dn4, locals.var_k1__blk932_dn6, locals.var_k1__blk932_dn7, locals.var_k1__blk932_dn8, locals.var_k1__blk932_dn9,)
    } else {
        (locals.var_k1_ac, locals.var_k1_ac_dn4, locals.var_k1_ac_dn6, locals.var_k1_ac_dn7, locals.var_k1_ac_dn8, locals.var_k1_ac_dn9,)
    }
};
        locals.var_k1_ac = assign41600_e47397;
        locals.var_k1_ac_dn4 = assign41600_e47397_d_n4;
        locals.var_k1_ac_dn6 = assign41600_e47397_d_n6;
        locals.var_k1_ac_dn7 = assign41600_e47397_d_n7;
        locals.var_k1_ac_dn8 = assign41600_e47397_d_n8;
        locals.var_k1_ac_dn9 = assign41600_e47397_d_n9;

        let (assign41610_e47401, assign41610_e47401_d_n4, assign41610_e47401_d_n6, assign41610_e47401_d_n7, assign41610_e47401_d_n8, assign41610_e47401_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_k2__blk933, locals.var_k2__blk933_dn4, locals.var_k2__blk933_dn6, locals.var_k2__blk933_dn7, locals.var_k2__blk933_dn8, locals.var_k2__blk933_dn9,)
    } else {
        (locals.var_k2_ac, locals.var_k2_ac_dn4, locals.var_k2_ac_dn6, locals.var_k2_ac_dn7, locals.var_k2_ac_dn8, locals.var_k2_ac_dn9,)
    }
};
        locals.var_k2_ac = assign41610_e47401;
        locals.var_k2_ac_dn4 = assign41610_e47401_d_n4;
        locals.var_k2_ac_dn6 = assign41610_e47401_d_n6;
        locals.var_k2_ac_dn7 = assign41610_e47401_d_n7;
        locals.var_k2_ac_dn8 = assign41610_e47401_d_n8;
        locals.var_k2_ac_dn9 = assign41610_e47401_d_n9;

        let (assign41620_e47405, assign41620_e47405_d_n4, assign41620_e47405_d_n6, assign41620_e47405_d_n7, assign41620_e47405_d_n8, assign41620_e47405_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_k1q1s__blk939, locals.var_k1q1s__blk939_dn4, locals.var_k1q1s__blk939_dn6, locals.var_k1q1s__blk939_dn7, locals.var_k1q1s__blk939_dn8, locals.var_k1q1s__blk939_dn9,)
    } else {
        (locals.var_k1q1s_ac, locals.var_k1q1s_ac_dn4, locals.var_k1q1s_ac_dn6, locals.var_k1q1s_ac_dn7, locals.var_k1q1s_ac_dn8, locals.var_k1q1s_ac_dn9,)
    }
};
        locals.var_k1q1s_ac = assign41620_e47405;
        locals.var_k1q1s_ac_dn4 = assign41620_e47405_d_n4;
        locals.var_k1q1s_ac_dn6 = assign41620_e47405_d_n6;
        locals.var_k1q1s_ac_dn7 = assign41620_e47405_d_n7;
        locals.var_k1q1s_ac_dn8 = assign41620_e47405_d_n8;
        locals.var_k1q1s_ac_dn9 = assign41620_e47405_d_n9;

        let (assign41630_e47409, assign41630_e47409_d_n4, assign41630_e47409_d_n6, assign41630_e47409_d_n7, assign41630_e47409_d_n8, assign41630_e47409_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_k2q2s__blk940, locals.var_k2q2s__blk940_dn4, locals.var_k2q2s__blk940_dn6, locals.var_k2q2s__blk940_dn7, locals.var_k2q2s__blk940_dn8, locals.var_k2q2s__blk940_dn9,)
    } else {
        (locals.var_k2q2s_ac, locals.var_k2q2s_ac_dn4, locals.var_k2q2s_ac_dn6, locals.var_k2q2s_ac_dn7, locals.var_k2q2s_ac_dn8, locals.var_k2q2s_ac_dn9,)
    }
};
        locals.var_k2q2s_ac = assign41630_e47409;
        locals.var_k2q2s_ac_dn4 = assign41630_e47409_d_n4;
        locals.var_k2q2s_ac_dn6 = assign41630_e47409_d_n6;
        locals.var_k2q2s_ac_dn7 = assign41630_e47409_d_n7;
        locals.var_k2q2s_ac_dn8 = assign41630_e47409_d_n8;
        locals.var_k2q2s_ac_dn9 = assign41630_e47409_d_n9;

        let (assign41640_e47413, assign41640_e47413_d_n4, assign41640_e47413_d_n6, assign41640_e47413_d_n7, assign41640_e47413_d_n8, assign41640_e47413_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_xdrifts__blk951, locals.var_xdrifts__blk951_dn4, locals.var_xdrifts__blk951_dn6, locals.var_xdrifts__blk951_dn7, locals.var_xdrifts__blk951_dn8, locals.var_xdrifts__blk951_dn9,)
    } else {
        (locals.var_xdrifts_ac, locals.var_xdrifts_ac_dn4, locals.var_xdrifts_ac_dn6, locals.var_xdrifts_ac_dn7, locals.var_xdrifts_ac_dn8, locals.var_xdrifts_ac_dn9,)
    }
};
        locals.var_xdrifts_ac = assign41640_e47413;
        locals.var_xdrifts_ac_dn4 = assign41640_e47413_d_n4;
        locals.var_xdrifts_ac_dn6 = assign41640_e47413_d_n6;
        locals.var_xdrifts_ac_dn7 = assign41640_e47413_d_n7;
        locals.var_xdrifts_ac_dn8 = assign41640_e47413_d_n8;
        locals.var_xdrifts_ac_dn9 = assign41640_e47413_d_n9;

        let (assign41650_e47417, assign41650_e47417_d_n4, assign41650_e47417_d_n6, assign41650_e47417_d_n7, assign41650_e47417_d_n8, assign41650_e47417_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_k1q1d__blk1004, locals.var_k1q1d__blk1004_dn4, locals.var_k1q1d__blk1004_dn6, locals.var_k1q1d__blk1004_dn7, locals.var_k1q1d__blk1004_dn8, locals.var_k1q1d__blk1004_dn9,)
    } else {
        (locals.var_k1q1d_ac, locals.var_k1q1d_ac_dn4, locals.var_k1q1d_ac_dn6, locals.var_k1q1d_ac_dn7, locals.var_k1q1d_ac_dn8, locals.var_k1q1d_ac_dn9,)
    }
};
        locals.var_k1q1d_ac = assign41650_e47417;
        locals.var_k1q1d_ac_dn4 = assign41650_e47417_d_n4;
        locals.var_k1q1d_ac_dn6 = assign41650_e47417_d_n6;
        locals.var_k1q1d_ac_dn7 = assign41650_e47417_d_n7;
        locals.var_k1q1d_ac_dn8 = assign41650_e47417_d_n8;
        locals.var_k1q1d_ac_dn9 = assign41650_e47417_d_n9;

        let (assign41660_e47421, assign41660_e47421_d_n4, assign41660_e47421_d_n6, assign41660_e47421_d_n7, assign41660_e47421_d_n8, assign41660_e47421_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_k2q2d__blk1005, locals.var_k2q2d__blk1005_dn4, locals.var_k2q2d__blk1005_dn6, locals.var_k2q2d__blk1005_dn7, locals.var_k2q2d__blk1005_dn8, locals.var_k2q2d__blk1005_dn9,)
    } else {
        (locals.var_k2q2d_ac, locals.var_k2q2d_ac_dn4, locals.var_k2q2d_ac_dn6, locals.var_k2q2d_ac_dn7, locals.var_k2q2d_ac_dn8, locals.var_k2q2d_ac_dn9,)
    }
};
        locals.var_k2q2d_ac = assign41660_e47421;
        locals.var_k2q2d_ac_dn4 = assign41660_e47421_d_n4;
        locals.var_k2q2d_ac_dn6 = assign41660_e47421_d_n6;
        locals.var_k2q2d_ac_dn7 = assign41660_e47421_d_n7;
        locals.var_k2q2d_ac_dn8 = assign41660_e47421_d_n8;
        locals.var_k2q2d_ac_dn9 = assign41660_e47421_d_n9;

        let (assign41670_e47425, assign41670_e47425_d_n4, assign41670_e47425_d_n6, assign41670_e47425_d_n7, assign41670_e47425_d_n8, assign41670_e47425_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_xdriftd__blk1015, locals.var_xdriftd__blk1015_dn4, locals.var_xdriftd__blk1015_dn6, locals.var_xdriftd__blk1015_dn7, locals.var_xdriftd__blk1015_dn8, locals.var_xdriftd__blk1015_dn9,)
    } else {
        (locals.var_xdriftd_ac, locals.var_xdriftd_ac_dn4, locals.var_xdriftd_ac_dn6, locals.var_xdriftd_ac_dn7, locals.var_xdriftd_ac_dn8, locals.var_xdriftd_ac_dn9,)
    }
};
        locals.var_xdriftd_ac = assign41670_e47425;
        locals.var_xdriftd_ac_dn4 = assign41670_e47425_d_n4;
        locals.var_xdriftd_ac_dn6 = assign41670_e47425_d_n6;
        locals.var_xdriftd_ac_dn7 = assign41670_e47425_d_n7;
        locals.var_xdriftd_ac_dn8 = assign41670_e47425_d_n8;
        locals.var_xdriftd_ac_dn9 = assign41670_e47425_d_n9;

        let (assign41680_e47429, assign41680_e47429_d_n4, assign41680_e47429_d_n6, assign41680_e47429_d_n7, assign41680_e47429_d_n8, assign41680_e47429_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_qim__blk1016, locals.var_qim__blk1016_dn4, locals.var_qim__blk1016_dn6, locals.var_qim__blk1016_dn7, locals.var_qim__blk1016_dn8, locals.var_qim__blk1016_dn9,)
    } else {
        (locals.var_qim_ac, locals.var_qim_ac_dn4, locals.var_qim_ac_dn6, locals.var_qim_ac_dn7, locals.var_qim_ac_dn8, locals.var_qim_ac_dn9,)
    }
};
        locals.var_qim_ac = assign41680_e47429;
        locals.var_qim_ac_dn4 = assign41680_e47429_d_n4;
        locals.var_qim_ac_dn6 = assign41680_e47429_d_n6;
        locals.var_qim_ac_dn7 = assign41680_e47429_d_n7;
        locals.var_qim_ac_dn8 = assign41680_e47429_d_n8;
        locals.var_qim_ac_dn9 = assign41680_e47429_d_n9;

        let (assign41690_e47433, assign41690_e47433_d_n4, assign41690_e47433_d_n6, assign41690_e47433_d_n7, assign41690_e47433_d_n8, assign41690_e47433_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_ratio_pd__blk1020, locals.var_ratio_pd__blk1020_dn4, locals.var_ratio_pd__blk1020_dn6, locals.var_ratio_pd__blk1020_dn7, locals.var_ratio_pd__blk1020_dn8, locals.var_ratio_pd__blk1020_dn9,)
    } else {
        (locals.var_ratio_pd_ac, locals.var_ratio_pd_ac_dn4, locals.var_ratio_pd_ac_dn6, locals.var_ratio_pd_ac_dn7, locals.var_ratio_pd_ac_dn8, locals.var_ratio_pd_ac_dn9,)
    }
};
        locals.var_ratio_pd_ac = assign41690_e47433;
        locals.var_ratio_pd_ac_dn4 = assign41690_e47433_d_n4;
        locals.var_ratio_pd_ac_dn6 = assign41690_e47433_d_n6;
        locals.var_ratio_pd_ac_dn7 = assign41690_e47433_d_n7;
        locals.var_ratio_pd_ac_dn8 = assign41690_e47433_d_n8;
        locals.var_ratio_pd_ac_dn9 = assign41690_e47433_d_n9;

        let (assign41700_e47437, assign41700_e47437_d_n4, assign41700_e47437_d_n6, assign41700_e47437_d_n7, assign41700_e47437_d_n8, assign41700_e47437_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_qi1m__blk1029, locals.var_qi1m__blk1029_dn4, locals.var_qi1m__blk1029_dn6, locals.var_qi1m__blk1029_dn7, locals.var_qi1m__blk1029_dn8, locals.var_qi1m__blk1029_dn9,)
    } else {
        (locals.var_qi1m_ac, locals.var_qi1m_ac_dn4, locals.var_qi1m_ac_dn6, locals.var_qi1m_ac_dn7, locals.var_qi1m_ac_dn8, locals.var_qi1m_ac_dn9,)
    }
};
        locals.var_qi1m_ac = assign41700_e47437;
        locals.var_qi1m_ac_dn4 = assign41700_e47437_d_n4;
        locals.var_qi1m_ac_dn6 = assign41700_e47437_d_n6;
        locals.var_qi1m_ac_dn7 = assign41700_e47437_d_n7;
        locals.var_qi1m_ac_dn8 = assign41700_e47437_d_n8;
        locals.var_qi1m_ac_dn9 = assign41700_e47437_d_n9;

        let (assign41710_e47441, assign41710_e47441_d_n4, assign41710_e47441_d_n6, assign41710_e47441_d_n7, assign41710_e47441_d_n8, assign41710_e47441_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_qi2m__blk1030, locals.var_qi2m__blk1030_dn4, locals.var_qi2m__blk1030_dn6, locals.var_qi2m__blk1030_dn7, locals.var_qi2m__blk1030_dn8, locals.var_qi2m__blk1030_dn9,)
    } else {
        (locals.var_qi2m_ac, locals.var_qi2m_ac_dn4, locals.var_qi2m_ac_dn6, locals.var_qi2m_ac_dn7, locals.var_qi2m_ac_dn8, locals.var_qi2m_ac_dn9,)
    }
};
        locals.var_qi2m_ac = assign41710_e47441;
        locals.var_qi2m_ac_dn4 = assign41710_e47441_d_n4;
        locals.var_qi2m_ac_dn6 = assign41710_e47441_d_n6;
        locals.var_qi2m_ac_dn7 = assign41710_e47441_d_n7;
        locals.var_qi2m_ac_dn8 = assign41710_e47441_d_n8;
        locals.var_qi2m_ac_dn9 = assign41710_e47441_d_n9;

        let (assign41720_e47445, assign41720_e47445_d_n4, assign41720_e47445_d_n6, assign41720_e47445_d_n7, assign41720_e47445_d_n8, assign41720_e47445_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_zsat__blk1051, locals.var_zsat__blk1051_dn4, locals.var_zsat__blk1051_dn6, locals.var_zsat__blk1051_dn7, locals.var_zsat__blk1051_dn8, locals.var_zsat__blk1051_dn9,)
    } else {
        (locals.var_zsat_ac, locals.var_zsat_ac_dn4, locals.var_zsat_ac_dn6, locals.var_zsat_ac_dn7, locals.var_zsat_ac_dn8, locals.var_zsat_ac_dn9,)
    }
};
        locals.var_zsat_ac = assign41720_e47445;
        locals.var_zsat_ac_dn4 = assign41720_e47445_d_n4;
        locals.var_zsat_ac_dn6 = assign41720_e47445_d_n6;
        locals.var_zsat_ac_dn7 = assign41720_e47445_d_n7;
        locals.var_zsat_ac_dn8 = assign41720_e47445_d_n8;
        locals.var_zsat_ac_dn9 = assign41720_e47445_d_n9;

        let (assign41730_e47449, assign41730_e47449_d_n4, assign41730_e47449_d_n6, assign41730_e47449_d_n7, assign41730_e47449_d_n8, assign41730_e47449_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_qmfact1__blk1054, locals.var_qmfact1__blk1054_dn4, locals.var_qmfact1__blk1054_dn6, locals.var_qmfact1__blk1054_dn7, locals.var_qmfact1__blk1054_dn8, locals.var_qmfact1__blk1054_dn9,)
    } else {
        (locals.var_qmfact1_ac, locals.var_qmfact1_ac_dn4, locals.var_qmfact1_ac_dn6, locals.var_qmfact1_ac_dn7, locals.var_qmfact1_ac_dn8, locals.var_qmfact1_ac_dn9,)
    }
};
        locals.var_qmfact1_ac = assign41730_e47449;
        locals.var_qmfact1_ac_dn4 = assign41730_e47449_d_n4;
        locals.var_qmfact1_ac_dn6 = assign41730_e47449_d_n6;
        locals.var_qmfact1_ac_dn7 = assign41730_e47449_d_n7;
        locals.var_qmfact1_ac_dn8 = assign41730_e47449_d_n8;
        locals.var_qmfact1_ac_dn9 = assign41730_e47449_d_n9;

        let (assign41740_e47453, assign41740_e47453_d_n4, assign41740_e47453_d_n6, assign41740_e47453_d_n7, assign41740_e47453_d_n8, assign41740_e47453_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_qmfact2__blk1055, locals.var_qmfact2__blk1055_dn4, locals.var_qmfact2__blk1055_dn6, locals.var_qmfact2__blk1055_dn7, locals.var_qmfact2__blk1055_dn8, locals.var_qmfact2__blk1055_dn9,)
    } else {
        (locals.var_qmfact2_ac, locals.var_qmfact2_ac_dn4, locals.var_qmfact2_ac_dn6, locals.var_qmfact2_ac_dn7, locals.var_qmfact2_ac_dn8, locals.var_qmfact2_ac_dn9,)
    }
};
        locals.var_qmfact2_ac = assign41740_e47453;
        locals.var_qmfact2_ac_dn4 = assign41740_e47453_d_n4;
        locals.var_qmfact2_ac_dn6 = assign41740_e47453_d_n6;
        locals.var_qmfact2_ac_dn7 = assign41740_e47453_d_n7;
        locals.var_qmfact2_ac_dn8 = assign41740_e47453_d_n8;
        locals.var_qmfact2_ac_dn9 = assign41740_e47453_d_n9;

        let (assign41750_e47457, assign41750_e47457_d_n4, assign41750_e47457_d_n6, assign41750_e47457_d_n7, assign41750_e47457_d_n8, assign41750_e47457_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_delta_k1q1__blk1076, locals.var_delta_k1q1__blk1076_dn4, locals.var_delta_k1q1__blk1076_dn6, locals.var_delta_k1q1__blk1076_dn7, locals.var_delta_k1q1__blk1076_dn8, locals.var_delta_k1q1__blk1076_dn9,)
    } else {
        (locals.var_delta_k1q1_ac, locals.var_delta_k1q1_ac_dn4, locals.var_delta_k1q1_ac_dn6, locals.var_delta_k1q1_ac_dn7, locals.var_delta_k1q1_ac_dn8, locals.var_delta_k1q1_ac_dn9,)
    }
};
        locals.var_delta_k1q1_ac = assign41750_e47457;
        locals.var_delta_k1q1_ac_dn4 = assign41750_e47457_d_n4;
        locals.var_delta_k1q1_ac_dn6 = assign41750_e47457_d_n6;
        locals.var_delta_k1q1_ac_dn7 = assign41750_e47457_d_n7;
        locals.var_delta_k1q1_ac_dn8 = assign41750_e47457_d_n8;
        locals.var_delta_k1q1_ac_dn9 = assign41750_e47457_d_n9;

        let (assign41760_e47461, assign41760_e47461_d_n4, assign41760_e47461_d_n6, assign41760_e47461_d_n7, assign41760_e47461_d_n8, assign41760_e47461_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_delta_k2q2__blk1077, locals.var_delta_k2q2__blk1077_dn4, locals.var_delta_k2q2__blk1077_dn6, locals.var_delta_k2q2__blk1077_dn7, locals.var_delta_k2q2__blk1077_dn8, locals.var_delta_k2q2__blk1077_dn9,)
    } else {
        (locals.var_delta_k2q2_ac, locals.var_delta_k2q2_ac_dn4, locals.var_delta_k2q2_ac_dn6, locals.var_delta_k2q2_ac_dn7, locals.var_delta_k2q2_ac_dn8, locals.var_delta_k2q2_ac_dn9,)
    }
};
        locals.var_delta_k2q2_ac = assign41760_e47461;
        locals.var_delta_k2q2_ac_dn4 = assign41760_e47461_d_n4;
        locals.var_delta_k2q2_ac_dn6 = assign41760_e47461_d_n6;
        locals.var_delta_k2q2_ac_dn7 = assign41760_e47461_d_n7;
        locals.var_delta_k2q2_ac_dn8 = assign41760_e47461_d_n8;
        locals.var_delta_k2q2_ac_dn9 = assign41760_e47461_d_n9;

        let (assign41770_e47465, assign41770_e47465_d_n4, assign41770_e47465_d_n6, assign41770_e47465_d_n7, assign41770_e47465_d_n8, assign41770_e47465_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_prod1__blk1078, locals.var_prod1__blk1078_dn4, locals.var_prod1__blk1078_dn6, locals.var_prod1__blk1078_dn7, locals.var_prod1__blk1078_dn8, locals.var_prod1__blk1078_dn9,)
    } else {
        (locals.var_prod1_ac, locals.var_prod1_ac_dn4, locals.var_prod1_ac_dn6, locals.var_prod1_ac_dn7, locals.var_prod1_ac_dn8, locals.var_prod1_ac_dn9,)
    }
};
        locals.var_prod1_ac = assign41770_e47465;
        locals.var_prod1_ac_dn4 = assign41770_e47465_d_n4;
        locals.var_prod1_ac_dn6 = assign41770_e47465_d_n6;
        locals.var_prod1_ac_dn7 = assign41770_e47465_d_n7;
        locals.var_prod1_ac_dn8 = assign41770_e47465_d_n8;
        locals.var_prod1_ac_dn9 = assign41770_e47465_d_n9;

        let (assign41780_e47469, assign41780_e47469_d_n4, assign41780_e47469_d_n6, assign41780_e47469_d_n7, assign41780_e47469_d_n8, assign41780_e47469_d_n9,) = {
    if (locals.var_guard1080 != 0.0) {
        (locals.var_prod2__blk1079, locals.var_prod2__blk1079_dn4, locals.var_prod2__blk1079_dn6, locals.var_prod2__blk1079_dn7, locals.var_prod2__blk1079_dn8, locals.var_prod2__blk1079_dn9,)
    } else {
        (locals.var_prod2_ac, locals.var_prod2_ac_dn4, locals.var_prod2_ac_dn6, locals.var_prod2_ac_dn7, locals.var_prod2_ac_dn8, locals.var_prod2_ac_dn9,)
    }
};
        locals.var_prod2_ac = assign41780_e47469;
        locals.var_prod2_ac_dn4 = assign41780_e47469_d_n4;
        locals.var_prod2_ac_dn6 = assign41780_e47469_d_n6;
        locals.var_prod2_ac_dn7 = assign41780_e47469_d_n7;
        locals.var_prod2_ac_dn8 = assign41780_e47469_d_n8;
        locals.var_prod2_ac_dn9 = assign41780_e47469_d_n9;

        let (assign41790_e47474, assign41790_e47474_d_n4, assign41790_e47474_d_n6, assign41790_e47474_d_n7, assign41790_e47474_d_n8, assign41790_e47474_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_xg20shift_dc, locals.var_xg20shift_dc_dn4, locals.var_xg20shift_dc_dn6, locals.var_xg20shift_dc_dn7, locals.var_xg20shift_dc_dn8, locals.var_xg20shift_dc_dn9,)
    } else {
        (locals.var_xg20shift_ac, locals.var_xg20shift_ac_dn4, locals.var_xg20shift_ac_dn6, locals.var_xg20shift_ac_dn7, locals.var_xg20shift_ac_dn8, locals.var_xg20shift_ac_dn9,)
    }
};
        locals.var_xg20shift_ac = assign41790_e47474;
        locals.var_xg20shift_ac_dn4 = assign41790_e47474_d_n4;
        locals.var_xg20shift_ac_dn6 = assign41790_e47474_d_n6;
        locals.var_xg20shift_ac_dn7 = assign41790_e47474_d_n7;
        locals.var_xg20shift_ac_dn8 = assign41790_e47474_d_n8;
        locals.var_xg20shift_ac_dn9 = assign41790_e47474_d_n9;

        let (assign41800_e47479, assign41800_e47479_d_n4, assign41800_e47479_d_n6, assign41800_e47479_d_n7, assign41800_e47479_d_n8, assign41800_e47479_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_diff_min_dc, locals.var_diff_min_dc_dn4, locals.var_diff_min_dc_dn6, locals.var_diff_min_dc_dn7, locals.var_diff_min_dc_dn8, locals.var_diff_min_dc_dn9,)
    } else {
        (locals.var_diff_min_ac, locals.var_diff_min_ac_dn4, locals.var_diff_min_ac_dn6, locals.var_diff_min_ac_dn7, locals.var_diff_min_ac_dn8, locals.var_diff_min_ac_dn9,)
    }
};
        locals.var_diff_min_ac = assign41800_e47479;
        locals.var_diff_min_ac_dn4 = assign41800_e47479_d_n4;
        locals.var_diff_min_ac_dn6 = assign41800_e47479_d_n6;
        locals.var_diff_min_ac_dn7 = assign41800_e47479_d_n7;
        locals.var_diff_min_ac_dn8 = assign41800_e47479_d_n8;
        locals.var_diff_min_ac_dn9 = assign41800_e47479_d_n9;

        let (assign41810_e47484, assign41810_e47484_d_n4, assign41810_e47484_d_n6, assign41810_e47484_d_n7, assign41810_e47484_d_n8, assign41810_e47484_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_a0_dc, locals.var_a0_dc_dn4, locals.var_a0_dc_dn6, locals.var_a0_dc_dn7, locals.var_a0_dc_dn8, locals.var_a0_dc_dn9,)
    } else {
        (locals.var_a0_ac, locals.var_a0_ac_dn4, locals.var_a0_ac_dn6, locals.var_a0_ac_dn7, locals.var_a0_ac_dn8, locals.var_a0_ac_dn9,)
    }
};
        locals.var_a0_ac = assign41810_e47484;
        locals.var_a0_ac_dn4 = assign41810_e47484_d_n4;
        locals.var_a0_ac_dn6 = assign41810_e47484_d_n6;
        locals.var_a0_ac_dn7 = assign41810_e47484_d_n7;
        locals.var_a0_ac_dn8 = assign41810_e47484_d_n8;
        locals.var_a0_ac_dn9 = assign41810_e47484_d_n9;

        let (assign41820_e47489, assign41820_e47489_d_n4, assign41820_e47489_d_n6, assign41820_e47489_d_n7, assign41820_e47489_d_n8, assign41820_e47489_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_inv_k1_dc, locals.var_inv_k1_dc_dn4, locals.var_inv_k1_dc_dn6, locals.var_inv_k1_dc_dn7, locals.var_inv_k1_dc_dn8, locals.var_inv_k1_dc_dn9,)
    } else {
        (locals.var_inv_k1_ac, locals.var_inv_k1_ac_dn4, locals.var_inv_k1_ac_dn6, locals.var_inv_k1_ac_dn7, locals.var_inv_k1_ac_dn8, locals.var_inv_k1_ac_dn9,)
    }
};
        locals.var_inv_k1_ac = assign41820_e47489;
        locals.var_inv_k1_ac_dn4 = assign41820_e47489_d_n4;
        locals.var_inv_k1_ac_dn6 = assign41820_e47489_d_n6;
        locals.var_inv_k1_ac_dn7 = assign41820_e47489_d_n7;
        locals.var_inv_k1_ac_dn8 = assign41820_e47489_d_n8;
        locals.var_inv_k1_ac_dn9 = assign41820_e47489_d_n9;

        let (assign41830_e47494, assign41830_e47494_d_n4, assign41830_e47494_d_n6, assign41830_e47494_d_n7, assign41830_e47494_d_n8, assign41830_e47494_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_inv_k2_dc, locals.var_inv_k2_dc_dn4, locals.var_inv_k2_dc_dn6, locals.var_inv_k2_dc_dn7, locals.var_inv_k2_dc_dn8, locals.var_inv_k2_dc_dn9,)
    } else {
        (locals.var_inv_k2_ac, locals.var_inv_k2_ac_dn4, locals.var_inv_k2_ac_dn6, locals.var_inv_k2_ac_dn7, locals.var_inv_k2_ac_dn8, locals.var_inv_k2_ac_dn9,)
    }
};
        locals.var_inv_k2_ac = assign41830_e47494;
        locals.var_inv_k2_ac_dn4 = assign41830_e47494_d_n4;
        locals.var_inv_k2_ac_dn6 = assign41830_e47494_d_n6;
        locals.var_inv_k2_ac_dn7 = assign41830_e47494_d_n7;
        locals.var_inv_k2_ac_dn8 = assign41830_e47494_d_n8;
        locals.var_inv_k2_ac_dn9 = assign41830_e47494_d_n9;

        let (assign41840_e47499, assign41840_e47499_d_n4, assign41840_e47499_d_n6, assign41840_e47499_d_n7, assign41840_e47499_d_n8, assign41840_e47499_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_keq_dc, locals.var_keq_dc_dn4, locals.var_keq_dc_dn6, locals.var_keq_dc_dn7, locals.var_keq_dc_dn8, locals.var_keq_dc_dn9,)
    } else {
        (locals.var_keq_ac, locals.var_keq_ac_dn4, locals.var_keq_ac_dn6, locals.var_keq_ac_dn7, locals.var_keq_ac_dn8, locals.var_keq_ac_dn9,)
    }
};
        locals.var_keq_ac = assign41840_e47499;
        locals.var_keq_ac_dn4 = assign41840_e47499_d_n4;
        locals.var_keq_ac_dn6 = assign41840_e47499_d_n6;
        locals.var_keq_ac_dn7 = assign41840_e47499_d_n7;
        locals.var_keq_ac_dn8 = assign41840_e47499_d_n8;
        locals.var_keq_ac_dn9 = assign41840_e47499_d_n9;

        let (assign41850_e47504, assign41850_e47504_d_n4, assign41850_e47504_d_n6, assign41850_e47504_d_n7, assign41850_e47504_d_n8, assign41850_e47504_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_dx_wi_dc, locals.var_dx_wi_dc_dn4, locals.var_dx_wi_dc_dn6, locals.var_dx_wi_dc_dn7, locals.var_dx_wi_dc_dn8, locals.var_dx_wi_dc_dn9,)
    } else {
        (locals.var_dx_wi_ac, locals.var_dx_wi_ac_dn4, locals.var_dx_wi_ac_dn6, locals.var_dx_wi_ac_dn7, locals.var_dx_wi_ac_dn8, locals.var_dx_wi_ac_dn9,)
    }
};
        locals.var_dx_wi_ac = assign41850_e47504;
        locals.var_dx_wi_ac_dn4 = assign41850_e47504_d_n4;
        locals.var_dx_wi_ac_dn6 = assign41850_e47504_d_n6;
        locals.var_dx_wi_ac_dn7 = assign41850_e47504_d_n7;
        locals.var_dx_wi_ac_dn8 = assign41850_e47504_d_n8;
        locals.var_dx_wi_ac_dn9 = assign41850_e47504_d_n9;

        let (assign41860_e47509, assign41860_e47509_d_n4, assign41860_e47509_d_n6, assign41860_e47509_d_n7, assign41860_e47509_d_n8, assign41860_e47509_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_csiprime_dc, locals.var_csiprime_dc_dn4, locals.var_csiprime_dc_dn6, locals.var_csiprime_dc_dn7, locals.var_csiprime_dc_dn8, locals.var_csiprime_dc_dn9,)
    } else {
        (locals.var_csiprime_ac, locals.var_csiprime_ac_dn4, locals.var_csiprime_ac_dn6, locals.var_csiprime_ac_dn7, locals.var_csiprime_ac_dn8, locals.var_csiprime_ac_dn9,)
    }
};
        locals.var_csiprime_ac = assign41860_e47509;
        locals.var_csiprime_ac_dn4 = assign41860_e47509_d_n4;
        locals.var_csiprime_ac_dn6 = assign41860_e47509_d_n6;
        locals.var_csiprime_ac_dn7 = assign41860_e47509_d_n7;
        locals.var_csiprime_ac_dn8 = assign41860_e47509_d_n8;
        locals.var_csiprime_ac_dn9 = assign41860_e47509_d_n9;

        let (assign41870_e47514, assign41870_e47514_d_n4, assign41870_e47514_d_n6, assign41870_e47514_d_n7, assign41870_e47514_d_n8, assign41870_e47514_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_dx_wi_1d_dc, locals.var_dx_wi_1d_dc_dn4, locals.var_dx_wi_1d_dc_dn6, locals.var_dx_wi_1d_dc_dn7, locals.var_dx_wi_1d_dc_dn8, locals.var_dx_wi_1d_dc_dn9,)
    } else {
        (locals.var_dx_wi_1d_ac, locals.var_dx_wi_1d_ac_dn4, locals.var_dx_wi_1d_ac_dn6, locals.var_dx_wi_1d_ac_dn7, locals.var_dx_wi_1d_ac_dn8, locals.var_dx_wi_1d_ac_dn9,)
    }
};
        locals.var_dx_wi_1d_ac = assign41870_e47514;
        locals.var_dx_wi_1d_ac_dn4 = assign41870_e47514_d_n4;
        locals.var_dx_wi_1d_ac_dn6 = assign41870_e47514_d_n6;
        locals.var_dx_wi_1d_ac_dn7 = assign41870_e47514_d_n7;
        locals.var_dx_wi_1d_ac_dn8 = assign41870_e47514_d_n8;
        locals.var_dx_wi_1d_ac_dn9 = assign41870_e47514_d_n9;

        let (assign41880_e47519, assign41880_e47519_d_n4, assign41880_e47519_d_n6, assign41880_e47519_d_n7, assign41880_e47519_d_n8, assign41880_e47519_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_dleff_dc, locals.var_dleff_dc_dn4, locals.var_dleff_dc_dn6, locals.var_dleff_dc_dn7, locals.var_dleff_dc_dn8, locals.var_dleff_dc_dn9,)
    } else {
        (locals.var_dleff_ac, locals.var_dleff_ac_dn4, locals.var_dleff_ac_dn6, locals.var_dleff_ac_dn7, locals.var_dleff_ac_dn8, locals.var_dleff_ac_dn9,)
    }
};
        locals.var_dleff_ac = assign41880_e47519;
        locals.var_dleff_ac_dn4 = assign41880_e47519_d_n4;
        locals.var_dleff_ac_dn6 = assign41880_e47519_d_n6;
        locals.var_dleff_ac_dn7 = assign41880_e47519_d_n7;
        locals.var_dleff_ac_dn8 = assign41880_e47519_d_n8;
        locals.var_dleff_ac_dn9 = assign41880_e47519_d_n9;

        let (assign41890_e47524, assign41890_e47524_d_n4, assign41890_e47524_d_n6, assign41890_e47524_d_n7, assign41890_e47524_d_n8, assign41890_e47524_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_xedge_dc, locals.var_xedge_dc_dn4, locals.var_xedge_dc_dn6, locals.var_xedge_dc_dn7, locals.var_xedge_dc_dn8, locals.var_xedge_dc_dn9,)
    } else {
        (locals.var_xedge_ac, locals.var_xedge_ac_dn4, locals.var_xedge_ac_dn6, locals.var_xedge_ac_dn7, locals.var_xedge_ac_dn8, locals.var_xedge_ac_dn9,)
    }
};
        locals.var_xedge_ac = assign41890_e47524;
        locals.var_xedge_ac_dn4 = assign41890_e47524_d_n4;
        locals.var_xedge_ac_dn6 = assign41890_e47524_d_n6;
        locals.var_xedge_ac_dn7 = assign41890_e47524_d_n7;
        locals.var_xedge_ac_dn8 = assign41890_e47524_d_n8;
        locals.var_xedge_ac_dn9 = assign41890_e47524_d_n9;

    }

    pub(super) fn stamp_transient_block_114(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign41900_e47529, assign41900_e47529_d_n4, assign41900_e47529_d_n6, assign41900_e47529_d_n7, assign41900_e47529_d_n8, assign41900_e47529_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_sce1_dc, locals.var_sce1_dc_dn4, locals.var_sce1_dc_dn6, locals.var_sce1_dc_dn7, locals.var_sce1_dc_dn8, locals.var_sce1_dc_dn9,)
    } else {
        (locals.var_sce1_ac, locals.var_sce1_ac_dn4, locals.var_sce1_ac_dn6, locals.var_sce1_ac_dn7, locals.var_sce1_ac_dn8, locals.var_sce1_ac_dn9,)
    }
};
        locals.var_sce1_ac = assign41900_e47529;
        locals.var_sce1_ac_dn4 = assign41900_e47529_d_n4;
        locals.var_sce1_ac_dn6 = assign41900_e47529_d_n6;
        locals.var_sce1_ac_dn7 = assign41900_e47529_d_n7;
        locals.var_sce1_ac_dn8 = assign41900_e47529_d_n8;
        locals.var_sce1_ac_dn9 = assign41900_e47529_d_n9;

        let (assign41910_e47534, assign41910_e47534_d_n4, assign41910_e47534_d_n6, assign41910_e47534_d_n7, assign41910_e47534_d_n8, assign41910_e47534_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_sce2_dc, locals.var_sce2_dc_dn4, locals.var_sce2_dc_dn6, locals.var_sce2_dc_dn7, locals.var_sce2_dc_dn8, locals.var_sce2_dc_dn9,)
    } else {
        (locals.var_sce2_ac, locals.var_sce2_ac_dn4, locals.var_sce2_ac_dn6, locals.var_sce2_ac_dn7, locals.var_sce2_ac_dn8, locals.var_sce2_ac_dn9,)
    }
};
        locals.var_sce2_ac = assign41910_e47534;
        locals.var_sce2_ac_dn4 = assign41910_e47534_d_n4;
        locals.var_sce2_ac_dn6 = assign41910_e47534_d_n6;
        locals.var_sce2_ac_dn7 = assign41910_e47534_d_n7;
        locals.var_sce2_ac_dn8 = assign41910_e47534_d_n8;
        locals.var_sce2_ac_dn9 = assign41910_e47534_d_n9;

        let (assign41920_e47539, assign41920_e47539_d_n4, assign41920_e47539_d_n6, assign41920_e47539_d_n7, assign41920_e47539_d_n8, assign41920_e47539_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_dxg1_dibl_dc, locals.var_dxg1_dibl_dc_dn4, locals.var_dxg1_dibl_dc_dn6, locals.var_dxg1_dibl_dc_dn7, locals.var_dxg1_dibl_dc_dn8, locals.var_dxg1_dibl_dc_dn9,)
    } else {
        (locals.var_dxg1_dibl_ac, locals.var_dxg1_dibl_ac_dn4, locals.var_dxg1_dibl_ac_dn6, locals.var_dxg1_dibl_ac_dn7, locals.var_dxg1_dibl_ac_dn8, locals.var_dxg1_dibl_ac_dn9,)
    }
};
        locals.var_dxg1_dibl_ac = assign41920_e47539;
        locals.var_dxg1_dibl_ac_dn4 = assign41920_e47539_d_n4;
        locals.var_dxg1_dibl_ac_dn6 = assign41920_e47539_d_n6;
        locals.var_dxg1_dibl_ac_dn7 = assign41920_e47539_d_n7;
        locals.var_dxg1_dibl_ac_dn8 = assign41920_e47539_d_n8;
        locals.var_dxg1_dibl_ac_dn9 = assign41920_e47539_d_n9;

        let (assign41930_e47544, assign41930_e47544_d_n4, assign41930_e47544_d_n6, assign41930_e47544_d_n7, assign41930_e47544_d_n8, assign41930_e47544_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_xg2_dc, locals.var_xg2_dc_dn4, locals.var_xg2_dc_dn6, locals.var_xg2_dc_dn7, locals.var_xg2_dc_dn8, locals.var_xg2_dc_dn9,)
    } else {
        (locals.var_xg2_ac, locals.var_xg2_ac_dn4, locals.var_xg2_ac_dn6, locals.var_xg2_ac_dn7, locals.var_xg2_ac_dn8, locals.var_xg2_ac_dn9,)
    }
};
        locals.var_xg2_ac = assign41930_e47544;
        locals.var_xg2_ac_dn4 = assign41930_e47544_d_n4;
        locals.var_xg2_ac_dn6 = assign41930_e47544_d_n6;
        locals.var_xg2_ac_dn7 = assign41930_e47544_d_n7;
        locals.var_xg2_ac_dn8 = assign41930_e47544_d_n8;
        locals.var_xg2_ac_dn9 = assign41930_e47544_d_n9;

        let (assign41940_e47549, assign41940_e47549_d_n4, assign41940_e47549_d_n6, assign41940_e47549_d_n7, assign41940_e47549_d_n8, assign41940_e47549_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_xg2x_dc, locals.var_xg2x_dc_dn4, locals.var_xg2x_dc_dn6, locals.var_xg2x_dc_dn7, locals.var_xg2x_dc_dn8, locals.var_xg2x_dc_dn9,)
    } else {
        (locals.var_xg2x_ac, locals.var_xg2x_ac_dn4, locals.var_xg2x_ac_dn6, locals.var_xg2x_ac_dn7, locals.var_xg2x_ac_dn8, locals.var_xg2x_ac_dn9,)
    }
};
        locals.var_xg2x_ac = assign41940_e47549;
        locals.var_xg2x_ac_dn4 = assign41940_e47549_d_n4;
        locals.var_xg2x_ac_dn6 = assign41940_e47549_d_n6;
        locals.var_xg2x_ac_dn7 = assign41940_e47549_d_n7;
        locals.var_xg2x_ac_dn8 = assign41940_e47549_d_n8;
        locals.var_xg2x_ac_dn9 = assign41940_e47549_d_n9;

        let (assign41950_e47554, assign41950_e47554_d_n4, assign41950_e47554_d_n6, assign41950_e47554_d_n7, assign41950_e47554_d_n8, assign41950_e47554_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_k1_dc, locals.var_k1_dc_dn4, locals.var_k1_dc_dn6, locals.var_k1_dc_dn7, locals.var_k1_dc_dn8, locals.var_k1_dc_dn9,)
    } else {
        (locals.var_k1_ac, locals.var_k1_ac_dn4, locals.var_k1_ac_dn6, locals.var_k1_ac_dn7, locals.var_k1_ac_dn8, locals.var_k1_ac_dn9,)
    }
};
        locals.var_k1_ac = assign41950_e47554;
        locals.var_k1_ac_dn4 = assign41950_e47554_d_n4;
        locals.var_k1_ac_dn6 = assign41950_e47554_d_n6;
        locals.var_k1_ac_dn7 = assign41950_e47554_d_n7;
        locals.var_k1_ac_dn8 = assign41950_e47554_d_n8;
        locals.var_k1_ac_dn9 = assign41950_e47554_d_n9;

        let (assign41960_e47559, assign41960_e47559_d_n4, assign41960_e47559_d_n6, assign41960_e47559_d_n7, assign41960_e47559_d_n8, assign41960_e47559_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_k2_dc, locals.var_k2_dc_dn4, locals.var_k2_dc_dn6, locals.var_k2_dc_dn7, locals.var_k2_dc_dn8, locals.var_k2_dc_dn9,)
    } else {
        (locals.var_k2_ac, locals.var_k2_ac_dn4, locals.var_k2_ac_dn6, locals.var_k2_ac_dn7, locals.var_k2_ac_dn8, locals.var_k2_ac_dn9,)
    }
};
        locals.var_k2_ac = assign41960_e47559;
        locals.var_k2_ac_dn4 = assign41960_e47559_d_n4;
        locals.var_k2_ac_dn6 = assign41960_e47559_d_n6;
        locals.var_k2_ac_dn7 = assign41960_e47559_d_n7;
        locals.var_k2_ac_dn8 = assign41960_e47559_d_n8;
        locals.var_k2_ac_dn9 = assign41960_e47559_d_n9;

        let (assign41970_e47564, assign41970_e47564_d_n4, assign41970_e47564_d_n6, assign41970_e47564_d_n7, assign41970_e47564_d_n8, assign41970_e47564_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_k1q1s_dc, locals.var_k1q1s_dc_dn4, locals.var_k1q1s_dc_dn6, locals.var_k1q1s_dc_dn7, locals.var_k1q1s_dc_dn8, locals.var_k1q1s_dc_dn9,)
    } else {
        (locals.var_k1q1s_ac, locals.var_k1q1s_ac_dn4, locals.var_k1q1s_ac_dn6, locals.var_k1q1s_ac_dn7, locals.var_k1q1s_ac_dn8, locals.var_k1q1s_ac_dn9,)
    }
};
        locals.var_k1q1s_ac = assign41970_e47564;
        locals.var_k1q1s_ac_dn4 = assign41970_e47564_d_n4;
        locals.var_k1q1s_ac_dn6 = assign41970_e47564_d_n6;
        locals.var_k1q1s_ac_dn7 = assign41970_e47564_d_n7;
        locals.var_k1q1s_ac_dn8 = assign41970_e47564_d_n8;
        locals.var_k1q1s_ac_dn9 = assign41970_e47564_d_n9;

        let (assign41980_e47569, assign41980_e47569_d_n4, assign41980_e47569_d_n6, assign41980_e47569_d_n7, assign41980_e47569_d_n8, assign41980_e47569_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_k2q2s_dc, locals.var_k2q2s_dc_dn4, locals.var_k2q2s_dc_dn6, locals.var_k2q2s_dc_dn7, locals.var_k2q2s_dc_dn8, locals.var_k2q2s_dc_dn9,)
    } else {
        (locals.var_k2q2s_ac, locals.var_k2q2s_ac_dn4, locals.var_k2q2s_ac_dn6, locals.var_k2q2s_ac_dn7, locals.var_k2q2s_ac_dn8, locals.var_k2q2s_ac_dn9,)
    }
};
        locals.var_k2q2s_ac = assign41980_e47569;
        locals.var_k2q2s_ac_dn4 = assign41980_e47569_d_n4;
        locals.var_k2q2s_ac_dn6 = assign41980_e47569_d_n6;
        locals.var_k2q2s_ac_dn7 = assign41980_e47569_d_n7;
        locals.var_k2q2s_ac_dn8 = assign41980_e47569_d_n8;
        locals.var_k2q2s_ac_dn9 = assign41980_e47569_d_n9;

        let (assign41990_e47574, assign41990_e47574_d_n4, assign41990_e47574_d_n6, assign41990_e47574_d_n7, assign41990_e47574_d_n8, assign41990_e47574_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_xdrifts_dc, locals.var_xdrifts_dc_dn4, locals.var_xdrifts_dc_dn6, locals.var_xdrifts_dc_dn7, locals.var_xdrifts_dc_dn8, locals.var_xdrifts_dc_dn9,)
    } else {
        (locals.var_xdrifts_ac, locals.var_xdrifts_ac_dn4, locals.var_xdrifts_ac_dn6, locals.var_xdrifts_ac_dn7, locals.var_xdrifts_ac_dn8, locals.var_xdrifts_ac_dn9,)
    }
};
        locals.var_xdrifts_ac = assign41990_e47574;
        locals.var_xdrifts_ac_dn4 = assign41990_e47574_d_n4;
        locals.var_xdrifts_ac_dn6 = assign41990_e47574_d_n6;
        locals.var_xdrifts_ac_dn7 = assign41990_e47574_d_n7;
        locals.var_xdrifts_ac_dn8 = assign41990_e47574_d_n8;
        locals.var_xdrifts_ac_dn9 = assign41990_e47574_d_n9;

        let (assign42000_e47579, assign42000_e47579_d_n4, assign42000_e47579_d_n6, assign42000_e47579_d_n7, assign42000_e47579_d_n8, assign42000_e47579_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_k1q1d_dc, locals.var_k1q1d_dc_dn4, locals.var_k1q1d_dc_dn6, locals.var_k1q1d_dc_dn7, locals.var_k1q1d_dc_dn8, locals.var_k1q1d_dc_dn9,)
    } else {
        (locals.var_k1q1d_ac, locals.var_k1q1d_ac_dn4, locals.var_k1q1d_ac_dn6, locals.var_k1q1d_ac_dn7, locals.var_k1q1d_ac_dn8, locals.var_k1q1d_ac_dn9,)
    }
};
        locals.var_k1q1d_ac = assign42000_e47579;
        locals.var_k1q1d_ac_dn4 = assign42000_e47579_d_n4;
        locals.var_k1q1d_ac_dn6 = assign42000_e47579_d_n6;
        locals.var_k1q1d_ac_dn7 = assign42000_e47579_d_n7;
        locals.var_k1q1d_ac_dn8 = assign42000_e47579_d_n8;
        locals.var_k1q1d_ac_dn9 = assign42000_e47579_d_n9;

        let (assign42010_e47584, assign42010_e47584_d_n4, assign42010_e47584_d_n6, assign42010_e47584_d_n7, assign42010_e47584_d_n8, assign42010_e47584_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_k2q2d_dc, locals.var_k2q2d_dc_dn4, locals.var_k2q2d_dc_dn6, locals.var_k2q2d_dc_dn7, locals.var_k2q2d_dc_dn8, locals.var_k2q2d_dc_dn9,)
    } else {
        (locals.var_k2q2d_ac, locals.var_k2q2d_ac_dn4, locals.var_k2q2d_ac_dn6, locals.var_k2q2d_ac_dn7, locals.var_k2q2d_ac_dn8, locals.var_k2q2d_ac_dn9,)
    }
};
        locals.var_k2q2d_ac = assign42010_e47584;
        locals.var_k2q2d_ac_dn4 = assign42010_e47584_d_n4;
        locals.var_k2q2d_ac_dn6 = assign42010_e47584_d_n6;
        locals.var_k2q2d_ac_dn7 = assign42010_e47584_d_n7;
        locals.var_k2q2d_ac_dn8 = assign42010_e47584_d_n8;
        locals.var_k2q2d_ac_dn9 = assign42010_e47584_d_n9;

        let (assign42020_e47589, assign42020_e47589_d_n4, assign42020_e47589_d_n6, assign42020_e47589_d_n7, assign42020_e47589_d_n8, assign42020_e47589_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_xdriftd_dc, locals.var_xdriftd_dc_dn4, locals.var_xdriftd_dc_dn6, locals.var_xdriftd_dc_dn7, locals.var_xdriftd_dc_dn8, locals.var_xdriftd_dc_dn9,)
    } else {
        (locals.var_xdriftd_ac, locals.var_xdriftd_ac_dn4, locals.var_xdriftd_ac_dn6, locals.var_xdriftd_ac_dn7, locals.var_xdriftd_ac_dn8, locals.var_xdriftd_ac_dn9,)
    }
};
        locals.var_xdriftd_ac = assign42020_e47589;
        locals.var_xdriftd_ac_dn4 = assign42020_e47589_d_n4;
        locals.var_xdriftd_ac_dn6 = assign42020_e47589_d_n6;
        locals.var_xdriftd_ac_dn7 = assign42020_e47589_d_n7;
        locals.var_xdriftd_ac_dn8 = assign42020_e47589_d_n8;
        locals.var_xdriftd_ac_dn9 = assign42020_e47589_d_n9;

        let (assign42030_e47594, assign42030_e47594_d_n4, assign42030_e47594_d_n6, assign42030_e47594_d_n7, assign42030_e47594_d_n8, assign42030_e47594_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_qim_dc, locals.var_qim_dc_dn4, locals.var_qim_dc_dn6, locals.var_qim_dc_dn7, locals.var_qim_dc_dn8, locals.var_qim_dc_dn9,)
    } else {
        (locals.var_qim_ac, locals.var_qim_ac_dn4, locals.var_qim_ac_dn6, locals.var_qim_ac_dn7, locals.var_qim_ac_dn8, locals.var_qim_ac_dn9,)
    }
};
        locals.var_qim_ac = assign42030_e47594;
        locals.var_qim_ac_dn4 = assign42030_e47594_d_n4;
        locals.var_qim_ac_dn6 = assign42030_e47594_d_n6;
        locals.var_qim_ac_dn7 = assign42030_e47594_d_n7;
        locals.var_qim_ac_dn8 = assign42030_e47594_d_n8;
        locals.var_qim_ac_dn9 = assign42030_e47594_d_n9;

        let (assign42040_e47599, assign42040_e47599_d_n4, assign42040_e47599_d_n6, assign42040_e47599_d_n7, assign42040_e47599_d_n8, assign42040_e47599_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_ratio_pd_dc, locals.var_ratio_pd_dc_dn4, locals.var_ratio_pd_dc_dn6, locals.var_ratio_pd_dc_dn7, locals.var_ratio_pd_dc_dn8, locals.var_ratio_pd_dc_dn9,)
    } else {
        (locals.var_ratio_pd_ac, locals.var_ratio_pd_ac_dn4, locals.var_ratio_pd_ac_dn6, locals.var_ratio_pd_ac_dn7, locals.var_ratio_pd_ac_dn8, locals.var_ratio_pd_ac_dn9,)
    }
};
        locals.var_ratio_pd_ac = assign42040_e47599;
        locals.var_ratio_pd_ac_dn4 = assign42040_e47599_d_n4;
        locals.var_ratio_pd_ac_dn6 = assign42040_e47599_d_n6;
        locals.var_ratio_pd_ac_dn7 = assign42040_e47599_d_n7;
        locals.var_ratio_pd_ac_dn8 = assign42040_e47599_d_n8;
        locals.var_ratio_pd_ac_dn9 = assign42040_e47599_d_n9;

        let (assign42050_e47604, assign42050_e47604_d_n4, assign42050_e47604_d_n6, assign42050_e47604_d_n7, assign42050_e47604_d_n8, assign42050_e47604_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_qi1m_dc, locals.var_qi1m_dc_dn4, locals.var_qi1m_dc_dn6, locals.var_qi1m_dc_dn7, locals.var_qi1m_dc_dn8, locals.var_qi1m_dc_dn9,)
    } else {
        (locals.var_qi1m_ac, locals.var_qi1m_ac_dn4, locals.var_qi1m_ac_dn6, locals.var_qi1m_ac_dn7, locals.var_qi1m_ac_dn8, locals.var_qi1m_ac_dn9,)
    }
};
        locals.var_qi1m_ac = assign42050_e47604;
        locals.var_qi1m_ac_dn4 = assign42050_e47604_d_n4;
        locals.var_qi1m_ac_dn6 = assign42050_e47604_d_n6;
        locals.var_qi1m_ac_dn7 = assign42050_e47604_d_n7;
        locals.var_qi1m_ac_dn8 = assign42050_e47604_d_n8;
        locals.var_qi1m_ac_dn9 = assign42050_e47604_d_n9;

        let (assign42060_e47609, assign42060_e47609_d_n4, assign42060_e47609_d_n6, assign42060_e47609_d_n7, assign42060_e47609_d_n8, assign42060_e47609_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_qi2m_dc, locals.var_qi2m_dc_dn4, locals.var_qi2m_dc_dn6, locals.var_qi2m_dc_dn7, locals.var_qi2m_dc_dn8, locals.var_qi2m_dc_dn9,)
    } else {
        (locals.var_qi2m_ac, locals.var_qi2m_ac_dn4, locals.var_qi2m_ac_dn6, locals.var_qi2m_ac_dn7, locals.var_qi2m_ac_dn8, locals.var_qi2m_ac_dn9,)
    }
};
        locals.var_qi2m_ac = assign42060_e47609;
        locals.var_qi2m_ac_dn4 = assign42060_e47609_d_n4;
        locals.var_qi2m_ac_dn6 = assign42060_e47609_d_n6;
        locals.var_qi2m_ac_dn7 = assign42060_e47609_d_n7;
        locals.var_qi2m_ac_dn8 = assign42060_e47609_d_n8;
        locals.var_qi2m_ac_dn9 = assign42060_e47609_d_n9;

        let (assign42070_e47614, assign42070_e47614_d_n4, assign42070_e47614_d_n6, assign42070_e47614_d_n7, assign42070_e47614_d_n8, assign42070_e47614_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_zsat_dc, locals.var_zsat_dc_dn4, locals.var_zsat_dc_dn6, locals.var_zsat_dc_dn7, locals.var_zsat_dc_dn8, locals.var_zsat_dc_dn9,)
    } else {
        (locals.var_zsat_ac, locals.var_zsat_ac_dn4, locals.var_zsat_ac_dn6, locals.var_zsat_ac_dn7, locals.var_zsat_ac_dn8, locals.var_zsat_ac_dn9,)
    }
};
        locals.var_zsat_ac = assign42070_e47614;
        locals.var_zsat_ac_dn4 = assign42070_e47614_d_n4;
        locals.var_zsat_ac_dn6 = assign42070_e47614_d_n6;
        locals.var_zsat_ac_dn7 = assign42070_e47614_d_n7;
        locals.var_zsat_ac_dn8 = assign42070_e47614_d_n8;
        locals.var_zsat_ac_dn9 = assign42070_e47614_d_n9;

        let (assign42080_e47619, assign42080_e47619_d_n4, assign42080_e47619_d_n6, assign42080_e47619_d_n7, assign42080_e47619_d_n8, assign42080_e47619_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_qmfact1_dc, locals.var_qmfact1_dc_dn4, locals.var_qmfact1_dc_dn6, locals.var_qmfact1_dc_dn7, locals.var_qmfact1_dc_dn8, locals.var_qmfact1_dc_dn9,)
    } else {
        (locals.var_qmfact1_ac, locals.var_qmfact1_ac_dn4, locals.var_qmfact1_ac_dn6, locals.var_qmfact1_ac_dn7, locals.var_qmfact1_ac_dn8, locals.var_qmfact1_ac_dn9,)
    }
};
        locals.var_qmfact1_ac = assign42080_e47619;
        locals.var_qmfact1_ac_dn4 = assign42080_e47619_d_n4;
        locals.var_qmfact1_ac_dn6 = assign42080_e47619_d_n6;
        locals.var_qmfact1_ac_dn7 = assign42080_e47619_d_n7;
        locals.var_qmfact1_ac_dn8 = assign42080_e47619_d_n8;
        locals.var_qmfact1_ac_dn9 = assign42080_e47619_d_n9;

        let (assign42090_e47624, assign42090_e47624_d_n4, assign42090_e47624_d_n6, assign42090_e47624_d_n7, assign42090_e47624_d_n8, assign42090_e47624_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_qmfact2_dc, locals.var_qmfact2_dc_dn4, locals.var_qmfact2_dc_dn6, locals.var_qmfact2_dc_dn7, locals.var_qmfact2_dc_dn8, locals.var_qmfact2_dc_dn9,)
    } else {
        (locals.var_qmfact2_ac, locals.var_qmfact2_ac_dn4, locals.var_qmfact2_ac_dn6, locals.var_qmfact2_ac_dn7, locals.var_qmfact2_ac_dn8, locals.var_qmfact2_ac_dn9,)
    }
};
        locals.var_qmfact2_ac = assign42090_e47624;
        locals.var_qmfact2_ac_dn4 = assign42090_e47624_d_n4;
        locals.var_qmfact2_ac_dn6 = assign42090_e47624_d_n6;
        locals.var_qmfact2_ac_dn7 = assign42090_e47624_d_n7;
        locals.var_qmfact2_ac_dn8 = assign42090_e47624_d_n8;
        locals.var_qmfact2_ac_dn9 = assign42090_e47624_d_n9;

        let (assign42100_e47629, assign42100_e47629_d_n4, assign42100_e47629_d_n6, assign42100_e47629_d_n7, assign42100_e47629_d_n8, assign42100_e47629_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_delta_k1q1_dc, locals.var_delta_k1q1_dc_dn4, locals.var_delta_k1q1_dc_dn6, locals.var_delta_k1q1_dc_dn7, locals.var_delta_k1q1_dc_dn8, locals.var_delta_k1q1_dc_dn9,)
    } else {
        (locals.var_delta_k1q1_ac, locals.var_delta_k1q1_ac_dn4, locals.var_delta_k1q1_ac_dn6, locals.var_delta_k1q1_ac_dn7, locals.var_delta_k1q1_ac_dn8, locals.var_delta_k1q1_ac_dn9,)
    }
};
        locals.var_delta_k1q1_ac = assign42100_e47629;
        locals.var_delta_k1q1_ac_dn4 = assign42100_e47629_d_n4;
        locals.var_delta_k1q1_ac_dn6 = assign42100_e47629_d_n6;
        locals.var_delta_k1q1_ac_dn7 = assign42100_e47629_d_n7;
        locals.var_delta_k1q1_ac_dn8 = assign42100_e47629_d_n8;
        locals.var_delta_k1q1_ac_dn9 = assign42100_e47629_d_n9;

        let (assign42110_e47634, assign42110_e47634_d_n4, assign42110_e47634_d_n6, assign42110_e47634_d_n7, assign42110_e47634_d_n8, assign42110_e47634_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_delta_k2q2_dc, locals.var_delta_k2q2_dc_dn4, locals.var_delta_k2q2_dc_dn6, locals.var_delta_k2q2_dc_dn7, locals.var_delta_k2q2_dc_dn8, locals.var_delta_k2q2_dc_dn9,)
    } else {
        (locals.var_delta_k2q2_ac, locals.var_delta_k2q2_ac_dn4, locals.var_delta_k2q2_ac_dn6, locals.var_delta_k2q2_ac_dn7, locals.var_delta_k2q2_ac_dn8, locals.var_delta_k2q2_ac_dn9,)
    }
};
        locals.var_delta_k2q2_ac = assign42110_e47634;
        locals.var_delta_k2q2_ac_dn4 = assign42110_e47634_d_n4;
        locals.var_delta_k2q2_ac_dn6 = assign42110_e47634_d_n6;
        locals.var_delta_k2q2_ac_dn7 = assign42110_e47634_d_n7;
        locals.var_delta_k2q2_ac_dn8 = assign42110_e47634_d_n8;
        locals.var_delta_k2q2_ac_dn9 = assign42110_e47634_d_n9;

        let (assign42120_e47639, assign42120_e47639_d_n4, assign42120_e47639_d_n6, assign42120_e47639_d_n7, assign42120_e47639_d_n8, assign42120_e47639_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_prod1_dc, locals.var_prod1_dc_dn4, locals.var_prod1_dc_dn6, locals.var_prod1_dc_dn7, locals.var_prod1_dc_dn8, locals.var_prod1_dc_dn9,)
    } else {
        (locals.var_prod1_ac, locals.var_prod1_ac_dn4, locals.var_prod1_ac_dn6, locals.var_prod1_ac_dn7, locals.var_prod1_ac_dn8, locals.var_prod1_ac_dn9,)
    }
};
        locals.var_prod1_ac = assign42120_e47639;
        locals.var_prod1_ac_dn4 = assign42120_e47639_d_n4;
        locals.var_prod1_ac_dn6 = assign42120_e47639_d_n6;
        locals.var_prod1_ac_dn7 = assign42120_e47639_d_n7;
        locals.var_prod1_ac_dn8 = assign42120_e47639_d_n8;
        locals.var_prod1_ac_dn9 = assign42120_e47639_d_n9;

        let (assign42130_e47644, assign42130_e47644_d_n4, assign42130_e47644_d_n6, assign42130_e47644_d_n7, assign42130_e47644_d_n8, assign42130_e47644_d_n9,) = {
    if (locals.var_guard1080 == 0.0) {
        (locals.var_prod2_dc, locals.var_prod2_dc_dn4, locals.var_prod2_dc_dn6, locals.var_prod2_dc_dn7, locals.var_prod2_dc_dn8, locals.var_prod2_dc_dn9,)
    } else {
        (locals.var_prod2_ac, locals.var_prod2_ac_dn4, locals.var_prod2_ac_dn6, locals.var_prod2_ac_dn7, locals.var_prod2_ac_dn8, locals.var_prod2_ac_dn9,)
    }
};
        locals.var_prod2_ac = assign42130_e47644;
        locals.var_prod2_ac_dn4 = assign42130_e47644_d_n4;
        locals.var_prod2_ac_dn6 = assign42130_e47644_d_n6;
        locals.var_prod2_ac_dn7 = assign42130_e47644_d_n7;
        locals.var_prod2_ac_dn8 = assign42130_e47644_d_n8;
        locals.var_prod2_ac_dn9 = assign42130_e47644_d_n9;

        let assign42140_e47648: f64 = (locals.var_dx_wi_1d_ac - locals.var_dx_wi_ac);
        let assign42140_e47649: f64 = (locals.var_fsceac_i * assign42140_e47648);
        let assign42140_e47653: f64 = (0.25 * locals.var_qim_ac);
        let assign42140_e47654: f64 = (1.0 + assign42140_e47653);
        let assign42140_e47655: f64 = (assign42140_e47649 / assign42140_e47654);
        locals.var_temp = assign42140_e47655;
        locals.var_temp_dn4 = ((((locals.var_fsceac_i * (locals.var_dx_wi_1d_ac_dn4 - locals.var_dx_wi_ac_dn4)) * assign42140_e47654) - (assign42140_e47649 * (0.25 * locals.var_qim_ac_dn4))) / (assign42140_e47654 * assign42140_e47654));
        locals.var_temp_dn6 = ((((locals.var_fsceac_i * (locals.var_dx_wi_1d_ac_dn6 - locals.var_dx_wi_ac_dn6)) * assign42140_e47654) - (assign42140_e47649 * (0.25 * locals.var_qim_ac_dn6))) / (assign42140_e47654 * assign42140_e47654));
        locals.var_temp_dn7 = ((((locals.var_fsceac_i * (locals.var_dx_wi_1d_ac_dn7 - locals.var_dx_wi_ac_dn7)) * assign42140_e47654) - (assign42140_e47649 * (0.25 * locals.var_qim_ac_dn7))) / (assign42140_e47654 * assign42140_e47654));
        locals.var_temp_dn8 = ((((locals.var_fsceac_i * (locals.var_dx_wi_1d_ac_dn8 - locals.var_dx_wi_ac_dn8)) * assign42140_e47654) - (assign42140_e47649 * (0.25 * locals.var_qim_ac_dn8))) / (assign42140_e47654 * assign42140_e47654));
        locals.var_temp_dn9 = ((((locals.var_fsceac_i * (locals.var_dx_wi_1d_ac_dn9 - locals.var_dx_wi_ac_dn9)) * assign42140_e47654) - (assign42140_e47649 * (0.25 * locals.var_qim_ac_dn9))) / (assign42140_e47654 * assign42140_e47654));

        let assign42150_e47659: f64 = (locals.var_k1q1s_ac + locals.var_k1q1d_ac);
        let assign42150_e47660: f64 = (0.5 * assign42150_e47659);
        let assign42150_e47662: f64 = (assign42150_e47660 + locals.var_temp);
        locals.var_k1q1m = assign42150_e47662;
        locals.var_k1q1m_dn4 = ((0.5 * (locals.var_k1q1s_ac_dn4 + locals.var_k1q1d_ac_dn4)) + locals.var_temp_dn4);
        locals.var_k1q1m_dn6 = ((0.5 * (locals.var_k1q1s_ac_dn6 + locals.var_k1q1d_ac_dn6)) + locals.var_temp_dn6);
        locals.var_k1q1m_dn7 = ((0.5 * (locals.var_k1q1s_ac_dn7 + locals.var_k1q1d_ac_dn7)) + locals.var_temp_dn7);
        locals.var_k1q1m_dn8 = ((0.5 * (locals.var_k1q1s_ac_dn8 + locals.var_k1q1d_ac_dn8)) + locals.var_temp_dn8);
        locals.var_k1q1m_dn9 = ((0.5 * (locals.var_k1q1s_ac_dn9 + locals.var_k1q1d_ac_dn9)) + locals.var_temp_dn9);

        let assign42160_e47666: f64 = (locals.var_k2q2s_ac + locals.var_k2q2d_ac);
        let assign42160_e47667: f64 = (0.5 * assign42160_e47666);
        let assign42160_e47669: f64 = (assign42160_e47667 - locals.var_temp);
        locals.var_k2q2m = assign42160_e47669;
        locals.var_k2q2m_dn4 = ((0.5 * (locals.var_k2q2s_ac_dn4 + locals.var_k2q2d_ac_dn4)) - locals.var_temp_dn4);
        locals.var_k2q2m_dn6 = ((0.5 * (locals.var_k2q2s_ac_dn6 + locals.var_k2q2d_ac_dn6)) - locals.var_temp_dn6);
        locals.var_k2q2m_dn7 = ((0.5 * (locals.var_k2q2s_ac_dn7 + locals.var_k2q2d_ac_dn7)) - locals.var_temp_dn7);
        locals.var_k2q2m_dn8 = ((0.5 * (locals.var_k2q2s_ac_dn8 + locals.var_k2q2d_ac_dn8)) - locals.var_temp_dn8);
        locals.var_k2q2m_dn9 = ((0.5 * (locals.var_k2q2s_ac_dn9 + locals.var_k2q2d_ac_dn9)) - locals.var_temp_dn9);

        let assign42170_e47672: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1234 = assign42170_e47672;

        let (assign42180_e47682, assign42180_e47682_d_n4, assign42180_e47682_d_n6, assign42180_e47682_d_n7, assign42180_e47682_d_n8, assign42180_e47682_d_n9,) = {
    if (locals.var_guard1234 != 0.0) {
        let assign42180_e47677: f64 = (locals.var_qi1m_ac / locals.var_qmfact1_ac);
        let assign42180_e47678: f64 = (locals.var_k1q1m + assign42180_e47677);
        let assign42180_e47680: f64 = (assign42180_e47678 - locals.var_qi1m_ac);
        (assign42180_e47680, ((locals.var_k1q1m_dn4 + (((locals.var_qi1m_ac_dn4 * locals.var_qmfact1_ac) - (locals.var_qi1m_ac * locals.var_qmfact1_ac_dn4)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac))) - locals.var_qi1m_ac_dn4), ((locals.var_k1q1m_dn6 + (((locals.var_qi1m_ac_dn6 * locals.var_qmfact1_ac) - (locals.var_qi1m_ac * locals.var_qmfact1_ac_dn6)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac))) - locals.var_qi1m_ac_dn6), ((locals.var_k1q1m_dn7 + (((locals.var_qi1m_ac_dn7 * locals.var_qmfact1_ac) - (locals.var_qi1m_ac * locals.var_qmfact1_ac_dn7)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac))) - locals.var_qi1m_ac_dn7), ((locals.var_k1q1m_dn8 + (((locals.var_qi1m_ac_dn8 * locals.var_qmfact1_ac) - (locals.var_qi1m_ac * locals.var_qmfact1_ac_dn8)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac))) - locals.var_qi1m_ac_dn8), ((locals.var_k1q1m_dn9 + (((locals.var_qi1m_ac_dn9 * locals.var_qmfact1_ac) - (locals.var_qi1m_ac * locals.var_qmfact1_ac_dn9)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac))) - locals.var_qi1m_ac_dn9),)
    } else {
        (locals.var_k1q1eff, locals.var_k1q1eff_dn4, locals.var_k1q1eff_dn6, locals.var_k1q1eff_dn7, locals.var_k1q1eff_dn8, locals.var_k1q1eff_dn9,)
    }
};
        locals.var_k1q1eff = assign42180_e47682;
        locals.var_k1q1eff_dn4 = assign42180_e47682_d_n4;
        locals.var_k1q1eff_dn6 = assign42180_e47682_d_n6;
        locals.var_k1q1eff_dn7 = assign42180_e47682_d_n7;
        locals.var_k1q1eff_dn8 = assign42180_e47682_d_n8;
        locals.var_k1q1eff_dn9 = assign42180_e47682_d_n9;

        let (assign42190_e47692, assign42190_e47692_d_n4, assign42190_e47692_d_n6, assign42190_e47692_d_n7, assign42190_e47692_d_n8, assign42190_e47692_d_n9,) = {
    if (locals.var_guard1234 != 0.0) {
        let assign42190_e47687: f64 = (locals.var_qi2m_ac / locals.var_qmfact2_ac);
        let assign42190_e47688: f64 = (locals.var_k2q2m + assign42190_e47687);
        let assign42190_e47690: f64 = (assign42190_e47688 - locals.var_qi2m_ac);
        (assign42190_e47690, ((locals.var_k2q2m_dn4 + (((locals.var_qi2m_ac_dn4 * locals.var_qmfact2_ac) - (locals.var_qi2m_ac * locals.var_qmfact2_ac_dn4)) / (locals.var_qmfact2_ac * locals.var_qmfact2_ac))) - locals.var_qi2m_ac_dn4), ((locals.var_k2q2m_dn6 + (((locals.var_qi2m_ac_dn6 * locals.var_qmfact2_ac) - (locals.var_qi2m_ac * locals.var_qmfact2_ac_dn6)) / (locals.var_qmfact2_ac * locals.var_qmfact2_ac))) - locals.var_qi2m_ac_dn6), ((locals.var_k2q2m_dn7 + (((locals.var_qi2m_ac_dn7 * locals.var_qmfact2_ac) - (locals.var_qi2m_ac * locals.var_qmfact2_ac_dn7)) / (locals.var_qmfact2_ac * locals.var_qmfact2_ac))) - locals.var_qi2m_ac_dn7), ((locals.var_k2q2m_dn8 + (((locals.var_qi2m_ac_dn8 * locals.var_qmfact2_ac) - (locals.var_qi2m_ac * locals.var_qmfact2_ac_dn8)) / (locals.var_qmfact2_ac * locals.var_qmfact2_ac))) - locals.var_qi2m_ac_dn8), ((locals.var_k2q2m_dn9 + (((locals.var_qi2m_ac_dn9 * locals.var_qmfact2_ac) - (locals.var_qi2m_ac * locals.var_qmfact2_ac_dn9)) / (locals.var_qmfact2_ac * locals.var_qmfact2_ac))) - locals.var_qi2m_ac_dn9),)
    } else {
        (locals.var_k2q2eff, locals.var_k2q2eff_dn4, locals.var_k2q2eff_dn6, locals.var_k2q2eff_dn7, locals.var_k2q2eff_dn8, locals.var_k2q2eff_dn9,)
    }
};
        locals.var_k2q2eff = assign42190_e47692;
        locals.var_k2q2eff_dn4 = assign42190_e47692_d_n4;
        locals.var_k2q2eff_dn6 = assign42190_e47692_d_n6;
        locals.var_k2q2eff_dn7 = assign42190_e47692_d_n7;
        locals.var_k2q2eff_dn8 = assign42190_e47692_d_n8;
        locals.var_k2q2eff_dn9 = assign42190_e47692_d_n9;

        let (assign42200_e47697, assign42200_e47697_d_n4, assign42200_e47697_d_n6, assign42200_e47697_d_n7, assign42200_e47697_d_n8, assign42200_e47697_d_n9,) = {
    if (locals.var_guard1234 == 0.0) {
        (locals.var_k1q1m, locals.var_k1q1m_dn4, locals.var_k1q1m_dn6, locals.var_k1q1m_dn7, locals.var_k1q1m_dn8, locals.var_k1q1m_dn9,)
    } else {
        (locals.var_k1q1eff, locals.var_k1q1eff_dn4, locals.var_k1q1eff_dn6, locals.var_k1q1eff_dn7, locals.var_k1q1eff_dn8, locals.var_k1q1eff_dn9,)
    }
};
        locals.var_k1q1eff = assign42200_e47697;
        locals.var_k1q1eff_dn4 = assign42200_e47697_d_n4;
        locals.var_k1q1eff_dn6 = assign42200_e47697_d_n6;
        locals.var_k1q1eff_dn7 = assign42200_e47697_d_n7;
        locals.var_k1q1eff_dn8 = assign42200_e47697_d_n8;
        locals.var_k1q1eff_dn9 = assign42200_e47697_d_n9;

        let (assign42210_e47702, assign42210_e47702_d_n4, assign42210_e47702_d_n6, assign42210_e47702_d_n7, assign42210_e47702_d_n8, assign42210_e47702_d_n9,) = {
    if (locals.var_guard1234 == 0.0) {
        (locals.var_k2q2m, locals.var_k2q2m_dn4, locals.var_k2q2m_dn6, locals.var_k2q2m_dn7, locals.var_k2q2m_dn8, locals.var_k2q2m_dn9,)
    } else {
        (locals.var_k2q2eff, locals.var_k2q2eff_dn4, locals.var_k2q2eff_dn6, locals.var_k2q2eff_dn7, locals.var_k2q2eff_dn8, locals.var_k2q2eff_dn9,)
    }
};
        locals.var_k2q2eff = assign42210_e47702;
        locals.var_k2q2eff_dn4 = assign42210_e47702_d_n4;
        locals.var_k2q2eff_dn6 = assign42210_e47702_d_n6;
        locals.var_k2q2eff_dn7 = assign42210_e47702_d_n7;
        locals.var_k2q2eff_dn8 = assign42210_e47702_d_n8;
        locals.var_k2q2eff_dn9 = assign42210_e47702_d_n9;

        let assign42220_e47705: f64 = (locals.var_delta_k1q1_ac * locals.var_prod1_ac);
        let assign42220_e47707: f64 = (assign42220_e47705 * 0.3333333333333);
        locals.var_temp1 = assign42220_e47707;
        locals.var_temp1_dn4 = (((locals.var_delta_k1q1_ac_dn4 * locals.var_prod1_ac) + (locals.var_delta_k1q1_ac * locals.var_prod1_ac_dn4)) * 0.3333333333333);
        locals.var_temp1_dn6 = (((locals.var_delta_k1q1_ac_dn6 * locals.var_prod1_ac) + (locals.var_delta_k1q1_ac * locals.var_prod1_ac_dn6)) * 0.3333333333333);
        locals.var_temp1_dn7 = (((locals.var_delta_k1q1_ac_dn7 * locals.var_prod1_ac) + (locals.var_delta_k1q1_ac * locals.var_prod1_ac_dn7)) * 0.3333333333333);
        locals.var_temp1_dn8 = (((locals.var_delta_k1q1_ac_dn8 * locals.var_prod1_ac) + (locals.var_delta_k1q1_ac * locals.var_prod1_ac_dn8)) * 0.3333333333333);
        locals.var_temp1_dn9 = (((locals.var_delta_k1q1_ac_dn9 * locals.var_prod1_ac) + (locals.var_delta_k1q1_ac * locals.var_prod1_ac_dn9)) * 0.3333333333333);

        let assign42230_e47710: f64 = (locals.var_delta_k1q1_ac * 0.1666666666667);
        let assign42230_e47716: f64 = (0.2 * locals.var_prod1_ac);
        let assign42230_e47717: f64 = (1.0 - assign42230_e47716);
        let assign42230_e47718: f64 = (locals.var_prod1_ac * assign42230_e47717);
        let assign42230_e47719: f64 = (1.0 + assign42230_e47718);
        let assign42230_e47720: f64 = (assign42230_e47710 * assign42230_e47719);
        locals.var_temp2 = assign42230_e47720;
        locals.var_temp2_dn4 = (((locals.var_delta_k1q1_ac_dn4 * 0.1666666666667) * assign42230_e47719) + (assign42230_e47710 * ((locals.var_prod1_ac_dn4 * assign42230_e47717) + (locals.var_prod1_ac * (-(0.2 * locals.var_prod1_ac_dn4))))));
        locals.var_temp2_dn6 = (((locals.var_delta_k1q1_ac_dn6 * 0.1666666666667) * assign42230_e47719) + (assign42230_e47710 * ((locals.var_prod1_ac_dn6 * assign42230_e47717) + (locals.var_prod1_ac * (-(0.2 * locals.var_prod1_ac_dn6))))));
        locals.var_temp2_dn7 = (((locals.var_delta_k1q1_ac_dn7 * 0.1666666666667) * assign42230_e47719) + (assign42230_e47710 * ((locals.var_prod1_ac_dn7 * assign42230_e47717) + (locals.var_prod1_ac * (-(0.2 * locals.var_prod1_ac_dn7))))));
        locals.var_temp2_dn8 = (((locals.var_delta_k1q1_ac_dn8 * 0.1666666666667) * assign42230_e47719) + (assign42230_e47710 * ((locals.var_prod1_ac_dn8 * assign42230_e47717) + (locals.var_prod1_ac * (-(0.2 * locals.var_prod1_ac_dn8))))));
        locals.var_temp2_dn9 = (((locals.var_delta_k1q1_ac_dn9 * 0.1666666666667) * assign42230_e47719) + (assign42230_e47710 * ((locals.var_prod1_ac_dn9 * assign42230_e47717) + (locals.var_prod1_ac * (-(0.2 * locals.var_prod1_ac_dn9))))));

        let assign42240_e47723: f64 = (0.5 * locals.var_k1q1eff);
        let assign42240_e47725: f64 = (assign42240_e47723 * locals.var_ratio_pd_ac);
        let assign42240_e47727: f64 = (assign42240_e47725 + locals.var_temp2);
        locals.var_k1q1deff = assign42240_e47727;
        locals.var_k1q1deff_dn4 = ((((0.5 * locals.var_k1q1eff_dn4) * locals.var_ratio_pd_ac) + (assign42240_e47723 * locals.var_ratio_pd_ac_dn4)) + locals.var_temp2_dn4);
        locals.var_k1q1deff_dn6 = ((((0.5 * locals.var_k1q1eff_dn6) * locals.var_ratio_pd_ac) + (assign42240_e47723 * locals.var_ratio_pd_ac_dn6)) + locals.var_temp2_dn6);
        locals.var_k1q1deff_dn7 = ((((0.5 * locals.var_k1q1eff_dn7) * locals.var_ratio_pd_ac) + (assign42240_e47723 * locals.var_ratio_pd_ac_dn7)) + locals.var_temp2_dn7);
        locals.var_k1q1deff_dn8 = ((((0.5 * locals.var_k1q1eff_dn8) * locals.var_ratio_pd_ac) + (assign42240_e47723 * locals.var_ratio_pd_ac_dn8)) + locals.var_temp2_dn8);
        locals.var_k1q1deff_dn9 = ((((0.5 * locals.var_k1q1eff_dn9) * locals.var_ratio_pd_ac) + (assign42240_e47723 * locals.var_ratio_pd_ac_dn9)) + locals.var_temp2_dn9);

        let assign42250_e47730: f64 = (locals.var_k1q1eff * locals.var_ratio_pd_ac);
        let assign42250_e47732: f64 = (assign42250_e47730 + locals.var_temp1);
        locals.var_k1q1eff = assign42250_e47732;
        locals.var_k1q1eff_dn4 = (((locals.var_k1q1eff_dn4 * locals.var_ratio_pd_ac) + (locals.var_k1q1eff * locals.var_ratio_pd_ac_dn4)) + locals.var_temp1_dn4);
        locals.var_k1q1eff_dn6 = (((locals.var_k1q1eff_dn6 * locals.var_ratio_pd_ac) + (locals.var_k1q1eff * locals.var_ratio_pd_ac_dn6)) + locals.var_temp1_dn6);
        locals.var_k1q1eff_dn7 = (((locals.var_k1q1eff_dn7 * locals.var_ratio_pd_ac) + (locals.var_k1q1eff * locals.var_ratio_pd_ac_dn7)) + locals.var_temp1_dn7);
        locals.var_k1q1eff_dn8 = (((locals.var_k1q1eff_dn8 * locals.var_ratio_pd_ac) + (locals.var_k1q1eff * locals.var_ratio_pd_ac_dn8)) + locals.var_temp1_dn8);
        locals.var_k1q1eff_dn9 = (((locals.var_k1q1eff_dn9 * locals.var_ratio_pd_ac) + (locals.var_k1q1eff * locals.var_ratio_pd_ac_dn9)) + locals.var_temp1_dn9);

        let assign42260_e47735: f64 = (locals.var_delta_k2q2_ac * locals.var_prod2_ac);
        let assign42260_e47737: f64 = (assign42260_e47735 * 0.3333333333333);
        locals.var_temp1 = assign42260_e47737;
        locals.var_temp1_dn4 = (((locals.var_delta_k2q2_ac_dn4 * locals.var_prod2_ac) + (locals.var_delta_k2q2_ac * locals.var_prod2_ac_dn4)) * 0.3333333333333);
        locals.var_temp1_dn6 = (((locals.var_delta_k2q2_ac_dn6 * locals.var_prod2_ac) + (locals.var_delta_k2q2_ac * locals.var_prod2_ac_dn6)) * 0.3333333333333);
        locals.var_temp1_dn7 = (((locals.var_delta_k2q2_ac_dn7 * locals.var_prod2_ac) + (locals.var_delta_k2q2_ac * locals.var_prod2_ac_dn7)) * 0.3333333333333);
        locals.var_temp1_dn8 = (((locals.var_delta_k2q2_ac_dn8 * locals.var_prod2_ac) + (locals.var_delta_k2q2_ac * locals.var_prod2_ac_dn8)) * 0.3333333333333);
        locals.var_temp1_dn9 = (((locals.var_delta_k2q2_ac_dn9 * locals.var_prod2_ac) + (locals.var_delta_k2q2_ac * locals.var_prod2_ac_dn9)) * 0.3333333333333);

        let assign42270_e47740: f64 = (locals.var_delta_k2q2_ac * 0.1666666666667);
        let assign42270_e47746: f64 = (0.2 * locals.var_prod2_ac);
        let assign42270_e47747: f64 = (1.0 - assign42270_e47746);
        let assign42270_e47748: f64 = (locals.var_prod2_ac * assign42270_e47747);
        let assign42270_e47749: f64 = (1.0 + assign42270_e47748);
        let assign42270_e47750: f64 = (assign42270_e47740 * assign42270_e47749);
        locals.var_temp2 = assign42270_e47750;
        locals.var_temp2_dn4 = (((locals.var_delta_k2q2_ac_dn4 * 0.1666666666667) * assign42270_e47749) + (assign42270_e47740 * ((locals.var_prod2_ac_dn4 * assign42270_e47747) + (locals.var_prod2_ac * (-(0.2 * locals.var_prod2_ac_dn4))))));
        locals.var_temp2_dn6 = (((locals.var_delta_k2q2_ac_dn6 * 0.1666666666667) * assign42270_e47749) + (assign42270_e47740 * ((locals.var_prod2_ac_dn6 * assign42270_e47747) + (locals.var_prod2_ac * (-(0.2 * locals.var_prod2_ac_dn6))))));
        locals.var_temp2_dn7 = (((locals.var_delta_k2q2_ac_dn7 * 0.1666666666667) * assign42270_e47749) + (assign42270_e47740 * ((locals.var_prod2_ac_dn7 * assign42270_e47747) + (locals.var_prod2_ac * (-(0.2 * locals.var_prod2_ac_dn7))))));
        locals.var_temp2_dn8 = (((locals.var_delta_k2q2_ac_dn8 * 0.1666666666667) * assign42270_e47749) + (assign42270_e47740 * ((locals.var_prod2_ac_dn8 * assign42270_e47747) + (locals.var_prod2_ac * (-(0.2 * locals.var_prod2_ac_dn8))))));
        locals.var_temp2_dn9 = (((locals.var_delta_k2q2_ac_dn9 * 0.1666666666667) * assign42270_e47749) + (assign42270_e47740 * ((locals.var_prod2_ac_dn9 * assign42270_e47747) + (locals.var_prod2_ac * (-(0.2 * locals.var_prod2_ac_dn9))))));

        let assign42280_e47753: f64 = (0.5 * locals.var_k2q2eff);
        let assign42280_e47755: f64 = (assign42280_e47753 + locals.var_temp2);
        locals.var_k2q2deff = assign42280_e47755;
        locals.var_k2q2deff_dn4 = ((0.5 * locals.var_k2q2eff_dn4) + locals.var_temp2_dn4);
        locals.var_k2q2deff_dn6 = ((0.5 * locals.var_k2q2eff_dn6) + locals.var_temp2_dn6);
        locals.var_k2q2deff_dn7 = ((0.5 * locals.var_k2q2eff_dn7) + locals.var_temp2_dn7);
        locals.var_k2q2deff_dn8 = ((0.5 * locals.var_k2q2eff_dn8) + locals.var_temp2_dn8);
        locals.var_k2q2deff_dn9 = ((0.5 * locals.var_k2q2eff_dn9) + locals.var_temp2_dn9);

    }

    pub(super) fn stamp_transient_block_115(
        locals: &mut StampLocals,
    ) {
        let assign42290_e47758: f64 = (locals.var_k2q2eff + locals.var_temp1);
        locals.var_k2q2eff = assign42290_e47758;
        locals.var_k2q2eff_dn4 = (locals.var_k2q2eff_dn4 + locals.var_temp1_dn4);
        locals.var_k2q2eff_dn6 = (locals.var_k2q2eff_dn6 + locals.var_temp1_dn6);
        locals.var_k2q2eff_dn7 = (locals.var_k2q2eff_dn7 + locals.var_temp1_dn7);
        locals.var_k2q2eff_dn8 = (locals.var_k2q2eff_dn8 + locals.var_temp1_dn8);
        locals.var_k2q2eff_dn9 = (locals.var_k2q2eff_dn9 + locals.var_temp1_dn9);

        let assign42300_e47761: f64 = (locals.var_csiprime_ac * locals.var_area_phit);
        locals.var_temp = assign42300_e47761;
        locals.var_temp_dn4 = ((locals.var_csiprime_ac_dn4 * locals.var_area_phit) + (locals.var_csiprime_ac * locals.var_area_phit_dn4));
        locals.var_temp_dn6 = ((locals.var_csiprime_ac_dn6 * locals.var_area_phit) + (locals.var_csiprime_ac * locals.var_area_phit_dn6));
        locals.var_temp_dn7 = ((locals.var_csiprime_ac_dn7 * locals.var_area_phit) + (locals.var_csiprime_ac * locals.var_area_phit_dn7));
        locals.var_temp_dn8 = ((locals.var_csiprime_ac_dn8 * locals.var_area_phit) + (locals.var_csiprime_ac * locals.var_area_phit_dn8));
        locals.var_temp_dn9 = ((locals.var_csiprime_ac_dn9 * locals.var_area_phit) + (locals.var_csiprime_ac * locals.var_area_phit_dn9));

        let assign42310_e47764: f64 = (locals.var_temp * locals.var_k1q1eff);
        locals.var_qg = assign42310_e47764;
        locals.var_qg_dn4 = ((locals.var_temp_dn4 * locals.var_k1q1eff) + (locals.var_temp * locals.var_k1q1eff_dn4));
        locals.var_qg_dn6 = ((locals.var_temp_dn6 * locals.var_k1q1eff) + (locals.var_temp * locals.var_k1q1eff_dn6));
        locals.var_qg_dn7 = ((locals.var_temp_dn7 * locals.var_k1q1eff) + (locals.var_temp * locals.var_k1q1eff_dn7));
        locals.var_qg_dn8 = ((locals.var_temp_dn8 * locals.var_k1q1eff) + (locals.var_temp * locals.var_k1q1eff_dn8));
        locals.var_qg_dn9 = ((locals.var_temp_dn9 * locals.var_k1q1eff) + (locals.var_temp * locals.var_k1q1eff_dn9));

        let assign42320_e47767: f64 = (locals.var_temp * locals.var_k2q2eff);
        locals.var_qb = assign42320_e47767;
        locals.var_qb_dn4 = ((locals.var_temp_dn4 * locals.var_k2q2eff) + (locals.var_temp * locals.var_k2q2eff_dn4));
        locals.var_qb_dn6 = ((locals.var_temp_dn6 * locals.var_k2q2eff) + (locals.var_temp * locals.var_k2q2eff_dn6));
        locals.var_qb_dn7 = ((locals.var_temp_dn7 * locals.var_k2q2eff) + (locals.var_temp * locals.var_k2q2eff_dn7));
        locals.var_qb_dn8 = ((locals.var_temp_dn8 * locals.var_k2q2eff) + (locals.var_temp * locals.var_k2q2eff_dn8));
        locals.var_qb_dn9 = ((locals.var_temp_dn9 * locals.var_k2q2eff) + (locals.var_temp * locals.var_k2q2eff_dn9));

        let assign42330_e47769: f64 = (-locals.var_temp);
        let assign42330_e47772: f64 = (locals.var_k1q1deff + locals.var_k2q2deff);
        let assign42330_e47773: f64 = (assign42330_e47769 * assign42330_e47772);
        locals.var_qd = assign42330_e47773;
        locals.var_qd_dn4 = (((-locals.var_temp_dn4) * assign42330_e47772) + (assign42330_e47769 * (locals.var_k1q1deff_dn4 + locals.var_k2q2deff_dn4)));
        locals.var_qd_dn6 = (((-locals.var_temp_dn6) * assign42330_e47772) + (assign42330_e47769 * (locals.var_k1q1deff_dn6 + locals.var_k2q2deff_dn6)));
        locals.var_qd_dn7 = (((-locals.var_temp_dn7) * assign42330_e47772) + (assign42330_e47769 * (locals.var_k1q1deff_dn7 + locals.var_k2q2deff_dn7)));
        locals.var_qd_dn8 = (((-locals.var_temp_dn8) * assign42330_e47772) + (assign42330_e47769 * (locals.var_k1q1deff_dn8 + locals.var_k2q2deff_dn8)));
        locals.var_qd_dn9 = (((-locals.var_temp_dn9) * assign42330_e47772) + (assign42330_e47769 * (locals.var_k1q1deff_dn9 + locals.var_k2q2deff_dn9)));

        let assign42340_e47776: f64 = if locals.var_fif_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1235 = assign42340_e47776;

        let (assign42350_e47784, assign42350_e47784_d_n4, assign42350_e47784_d_n6, assign42350_e47784_d_n7, assign42350_e47784_d_n8, assign42350_e47784_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42350_e47781: f64 = (2.0 * 0.6931471805599);
        let assign42350_e47782: f64 = (locals.var_xth_1d + assign42350_e47781);
        (assign42350_e47782, locals.var_xth_1d_dn4, locals.var_xth_1d_dn6, locals.var_xth_1d_dn7, locals.var_xth_1d_dn8, locals.var_xth_1d_dn9,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign42350_e47784;
        locals.var_temp_dn4 = assign42350_e47784_d_n4;
        locals.var_temp_dn6 = assign42350_e47784_d_n6;
        locals.var_temp_dn7 = assign42350_e47784_d_n7;
        locals.var_temp_dn8 = assign42350_e47784_d_n8;
        locals.var_temp_dn9 = assign42350_e47784_d_n9;

        let (assign42360_e47790, assign42360_e47790_d_n4, assign42360_e47790_d_n6, assign42360_e47790_d_n7, assign42360_e47790_d_n8, assign42360_e47790_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42360_e47788: f64 = (locals.var_xdrifts_ac + locals.var_temp);
        (assign42360_e47788, (locals.var_xdrifts_ac_dn4 + locals.var_temp_dn4), (locals.var_xdrifts_ac_dn6 + locals.var_temp_dn6), (locals.var_xdrifts_ac_dn7 + locals.var_temp_dn7), (locals.var_xdrifts_ac_dn8 + locals.var_temp_dn8), (locals.var_xdrifts_ac_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_xeffs, locals.var_xeffs_dn4, locals.var_xeffs_dn6, locals.var_xeffs_dn7, locals.var_xeffs_dn8, locals.var_xeffs_dn9,)
    }
};
        locals.var_xeffs = assign42360_e47790;
        locals.var_xeffs_dn4 = assign42360_e47790_d_n4;
        locals.var_xeffs_dn6 = assign42360_e47790_d_n6;
        locals.var_xeffs_dn7 = assign42360_e47790_d_n7;
        locals.var_xeffs_dn8 = assign42360_e47790_d_n8;
        locals.var_xeffs_dn9 = assign42360_e47790_d_n9;

        let (assign42370_e47796, assign42370_e47796_d_n4, assign42370_e47796_d_n6, assign42370_e47796_d_n7, assign42370_e47796_d_n8, assign42370_e47796_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42370_e47794: f64 = (locals.var_xdriftd_ac + locals.var_temp);
        (assign42370_e47794, (locals.var_xdriftd_ac_dn4 + locals.var_temp_dn4), (locals.var_xdriftd_ac_dn6 + locals.var_temp_dn6), (locals.var_xdriftd_ac_dn7 + locals.var_temp_dn7), (locals.var_xdriftd_ac_dn8 + locals.var_temp_dn8), (locals.var_xdriftd_ac_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_xeffd, locals.var_xeffd_dn4, locals.var_xeffd_dn6, locals.var_xeffd_dn7, locals.var_xeffd_dn8, locals.var_xeffd_dn9,)
    }
};
        locals.var_xeffd = assign42370_e47796;
        locals.var_xeffd_dn4 = assign42370_e47796_d_n4;
        locals.var_xeffd_dn6 = assign42370_e47796_d_n6;
        locals.var_xeffd_dn7 = assign42370_e47796_d_n7;
        locals.var_xeffd_dn8 = assign42370_e47796_d_n8;
        locals.var_xeffd_dn9 = assign42370_e47796_d_n9;

        let (assign42380_e47815, assign42380_e47815_d_n4, assign42380_e47815_d_n6, assign42380_e47815_d_n7, assign42380_e47815_d_n8, assign42380_e47815_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42380_e47801: f64 = (locals.var_xeffs + locals.var_xth_1d);
        let assign42380_e47804: f64 = (locals.var_xeffs - locals.var_xth_1d);
        let assign42380_e47807: f64 = (locals.var_xeffs - locals.var_xth_1d);
        let assign42380_e47808: f64 = (assign42380_e47804 * assign42380_e47807);
        let assign42380_e47810: f64 = (assign42380_e47808 + 9.0);
        let assign42380_e47811: f64 = (assign42380_e47810).sqrt();
        let assign42380_e47812: f64 = (assign42380_e47801 - assign42380_e47811);
        let assign42380_e47813: f64 = (0.5 * assign42380_e47812);
        (assign42380_e47813, (0.5 * ((locals.var_xeffs_dn4 + locals.var_xth_1d_dn4) - ((((locals.var_xeffs_dn4 - locals.var_xth_1d_dn4) * assign42380_e47807) + (assign42380_e47804 * (locals.var_xeffs_dn4 - locals.var_xth_1d_dn4))) / (2.0 * assign42380_e47811)))), (0.5 * ((locals.var_xeffs_dn6 + locals.var_xth_1d_dn6) - ((((locals.var_xeffs_dn6 - locals.var_xth_1d_dn6) * assign42380_e47807) + (assign42380_e47804 * (locals.var_xeffs_dn6 - locals.var_xth_1d_dn6))) / (2.0 * assign42380_e47811)))), (0.5 * ((locals.var_xeffs_dn7 + locals.var_xth_1d_dn7) - ((((locals.var_xeffs_dn7 - locals.var_xth_1d_dn7) * assign42380_e47807) + (assign42380_e47804 * (locals.var_xeffs_dn7 - locals.var_xth_1d_dn7))) / (2.0 * assign42380_e47811)))), (0.5 * ((locals.var_xeffs_dn8 + locals.var_xth_1d_dn8) - ((((locals.var_xeffs_dn8 - locals.var_xth_1d_dn8) * assign42380_e47807) + (assign42380_e47804 * (locals.var_xeffs_dn8 - locals.var_xth_1d_dn8))) / (2.0 * assign42380_e47811)))), (0.5 * ((locals.var_xeffs_dn9 + locals.var_xth_1d_dn9) - ((((locals.var_xeffs_dn9 - locals.var_xth_1d_dn9) * assign42380_e47807) + (assign42380_e47804 * (locals.var_xeffs_dn9 - locals.var_xth_1d_dn9))) / (2.0 * assign42380_e47811)))),)
    } else {
        (locals.var_xstars, locals.var_xstars_dn4, locals.var_xstars_dn6, locals.var_xstars_dn7, locals.var_xstars_dn8, locals.var_xstars_dn9,)
    }
};
        locals.var_xstars = assign42380_e47815;
        locals.var_xstars_dn4 = assign42380_e47815_d_n4;
        locals.var_xstars_dn6 = assign42380_e47815_d_n6;
        locals.var_xstars_dn7 = assign42380_e47815_d_n7;
        locals.var_xstars_dn8 = assign42380_e47815_d_n8;
        locals.var_xstars_dn9 = assign42380_e47815_d_n9;

        let (assign42390_e47840, assign42390_e47840_d_n4, assign42390_e47840_d_n6, assign42390_e47840_d_n7, assign42390_e47840_d_n8, assign42390_e47840_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42390_e47821: f64 = (locals.var_xth_1d + locals.var_xd);
        let assign42390_e47822: f64 = (locals.var_xeffd + assign42390_e47821);
        let assign42390_e47826: f64 = (locals.var_xth_1d + locals.var_xd);
        let assign42390_e47827: f64 = (locals.var_xeffd - assign42390_e47826);
        let assign42390_e47831: f64 = (locals.var_xth_1d + locals.var_xd);
        let assign42390_e47832: f64 = (locals.var_xeffd - assign42390_e47831);
        let assign42390_e47833: f64 = (assign42390_e47827 * assign42390_e47832);
        let assign42390_e47835: f64 = (assign42390_e47833 + 9.0);
        let assign42390_e47836: f64 = (assign42390_e47835).sqrt();
        let assign42390_e47837: f64 = (assign42390_e47822 - assign42390_e47836);
        let assign42390_e47838: f64 = (0.5 * assign42390_e47837);
        (assign42390_e47838, (0.5 * ((locals.var_xeffd_dn4 + (locals.var_xth_1d_dn4 + locals.var_xd_dn4)) - ((((locals.var_xeffd_dn4 - (locals.var_xth_1d_dn4 + locals.var_xd_dn4)) * assign42390_e47832) + (assign42390_e47827 * (locals.var_xeffd_dn4 - (locals.var_xth_1d_dn4 + locals.var_xd_dn4)))) / (2.0 * assign42390_e47836)))), (0.5 * ((locals.var_xeffd_dn6 + (locals.var_xth_1d_dn6 + locals.var_xd_dn6)) - ((((locals.var_xeffd_dn6 - (locals.var_xth_1d_dn6 + locals.var_xd_dn6)) * assign42390_e47832) + (assign42390_e47827 * (locals.var_xeffd_dn6 - (locals.var_xth_1d_dn6 + locals.var_xd_dn6)))) / (2.0 * assign42390_e47836)))), (0.5 * ((locals.var_xeffd_dn7 + (locals.var_xth_1d_dn7 + locals.var_xd_dn7)) - ((((locals.var_xeffd_dn7 - (locals.var_xth_1d_dn7 + locals.var_xd_dn7)) * assign42390_e47832) + (assign42390_e47827 * (locals.var_xeffd_dn7 - (locals.var_xth_1d_dn7 + locals.var_xd_dn7)))) / (2.0 * assign42390_e47836)))), (0.5 * ((locals.var_xeffd_dn8 + (locals.var_xth_1d_dn8 + locals.var_xd_dn8)) - ((((locals.var_xeffd_dn8 - (locals.var_xth_1d_dn8 + locals.var_xd_dn8)) * assign42390_e47832) + (assign42390_e47827 * (locals.var_xeffd_dn8 - (locals.var_xth_1d_dn8 + locals.var_xd_dn8)))) / (2.0 * assign42390_e47836)))), (0.5 * ((locals.var_xeffd_dn9 + (locals.var_xth_1d_dn9 + locals.var_xd_dn9)) - ((((locals.var_xeffd_dn9 - (locals.var_xth_1d_dn9 + locals.var_xd_dn9)) * assign42390_e47832) + (assign42390_e47827 * (locals.var_xeffd_dn9 - (locals.var_xth_1d_dn9 + locals.var_xd_dn9)))) / (2.0 * assign42390_e47836)))),)
    } else {
        (locals.var_xstard, locals.var_xstard_dn4, locals.var_xstard_dn6, locals.var_xstard_dn7, locals.var_xstard_dn8, locals.var_xstard_dn9,)
    }
};
        locals.var_xstard = assign42390_e47840;
        locals.var_xstard_dn4 = assign42390_e47840_d_n4;
        locals.var_xstard_dn6 = assign42390_e47840_d_n6;
        locals.var_xstard_dn7 = assign42390_e47840_d_n7;
        locals.var_xstard_dn8 = assign42390_e47840_d_n8;
        locals.var_xstard_dn9 = assign42390_e47840_d_n9;

        let (assign42400_e47851, assign42400_e47851_d_n4, assign42400_e47851_d_n6, assign42400_e47851_d_n7, assign42400_e47851_d_n8, assign42400_e47851_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42400_e47846: f64 = (0.5 + locals.var_inv_k2_ac);
        let assign42400_e47847: f64 = (locals.var_keq_ac * assign42400_e47846);
        let assign42400_e47848: f64 = (assign42400_e47847).sqrt();
        let assign42400_e47849: f64 = (locals.var_lambda2d * assign42400_e47848);
        (assign42400_e47849, (locals.var_lambda2d * (((locals.var_keq_ac_dn4 * assign42400_e47846) + (locals.var_keq_ac * locals.var_inv_k2_ac_dn4)) / (2.0 * assign42400_e47848))), (locals.var_lambda2d * (((locals.var_keq_ac_dn6 * assign42400_e47846) + (locals.var_keq_ac * locals.var_inv_k2_ac_dn6)) / (2.0 * assign42400_e47848))), (locals.var_lambda2d * (((locals.var_keq_ac_dn7 * assign42400_e47846) + (locals.var_keq_ac * locals.var_inv_k2_ac_dn7)) / (2.0 * assign42400_e47848))), (locals.var_lambda2d * (((locals.var_keq_ac_dn8 * assign42400_e47846) + (locals.var_keq_ac * locals.var_inv_k2_ac_dn8)) / (2.0 * assign42400_e47848))), (locals.var_lambda2d * (((locals.var_keq_ac_dn9 * assign42400_e47846) + (locals.var_keq_ac * locals.var_inv_k2_ac_dn9)) / (2.0 * assign42400_e47848))),)
    } else {
        (locals.var_lambdaf, locals.var_lambdaf_dn4, locals.var_lambdaf_dn6, locals.var_lambdaf_dn7, locals.var_lambdaf_dn8, locals.var_lambdaf_dn9,)
    }
};
        locals.var_lambdaf = assign42400_e47851;
        locals.var_lambdaf_dn4 = assign42400_e47851_d_n4;
        locals.var_lambdaf_dn6 = assign42400_e47851_d_n6;
        locals.var_lambdaf_dn7 = assign42400_e47851_d_n7;
        locals.var_lambdaf_dn8 = assign42400_e47851_d_n8;
        locals.var_lambdaf_dn9 = assign42400_e47851_d_n9;

        let (assign42410_e47866, assign42410_e47866_d_n4, assign42410_e47866_d_n6, assign42410_e47866_d_n7, assign42410_e47866_d_n8, assign42410_e47866_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42410_e47856: f64 = (locals.var_keq_ac * locals.var_k1_ac);
        let assign42410_e47858: f64 = (assign42410_e47856 * locals.var_inv_k2_ac);
        let assign42410_e47861: f64 = (0.5 + locals.var_inv_k1_ac);
        let assign42410_e47862: f64 = (assign42410_e47858 * assign42410_e47861);
        let assign42410_e47863: f64 = (assign42410_e47862).sqrt();
        let assign42410_e47864: f64 = (locals.var_lambda2d * assign42410_e47863);
        (assign42410_e47864, (locals.var_lambda2d * (((((((locals.var_keq_ac_dn4 * locals.var_k1_ac) + (locals.var_keq_ac * locals.var_k1_ac_dn4)) * locals.var_inv_k2_ac) + (assign42410_e47856 * locals.var_inv_k2_ac_dn4)) * assign42410_e47861) + (assign42410_e47858 * locals.var_inv_k1_ac_dn4)) / (2.0 * assign42410_e47863))), (locals.var_lambda2d * (((((((locals.var_keq_ac_dn6 * locals.var_k1_ac) + (locals.var_keq_ac * locals.var_k1_ac_dn6)) * locals.var_inv_k2_ac) + (assign42410_e47856 * locals.var_inv_k2_ac_dn6)) * assign42410_e47861) + (assign42410_e47858 * locals.var_inv_k1_ac_dn6)) / (2.0 * assign42410_e47863))), (locals.var_lambda2d * (((((((locals.var_keq_ac_dn7 * locals.var_k1_ac) + (locals.var_keq_ac * locals.var_k1_ac_dn7)) * locals.var_inv_k2_ac) + (assign42410_e47856 * locals.var_inv_k2_ac_dn7)) * assign42410_e47861) + (assign42410_e47858 * locals.var_inv_k1_ac_dn7)) / (2.0 * assign42410_e47863))), (locals.var_lambda2d * (((((((locals.var_keq_ac_dn8 * locals.var_k1_ac) + (locals.var_keq_ac * locals.var_k1_ac_dn8)) * locals.var_inv_k2_ac) + (assign42410_e47856 * locals.var_inv_k2_ac_dn8)) * assign42410_e47861) + (assign42410_e47858 * locals.var_inv_k1_ac_dn8)) / (2.0 * assign42410_e47863))), (locals.var_lambda2d * (((((((locals.var_keq_ac_dn9 * locals.var_k1_ac) + (locals.var_keq_ac * locals.var_k1_ac_dn9)) * locals.var_inv_k2_ac) + (assign42410_e47856 * locals.var_inv_k2_ac_dn9)) * assign42410_e47861) + (assign42410_e47858 * locals.var_inv_k1_ac_dn9)) / (2.0 * assign42410_e47863))),)
    } else {
        (locals.var_lambdab, locals.var_lambdab_dn4, locals.var_lambdab_dn6, locals.var_lambdab_dn7, locals.var_lambdab_dn8, locals.var_lambdab_dn9,)
    }
};
        locals.var_lambdab = assign42410_e47866;
        locals.var_lambdab_dn4 = assign42410_e47866_d_n4;
        locals.var_lambdab_dn6 = assign42410_e47866_d_n6;
        locals.var_lambdab_dn7 = assign42410_e47866_d_n7;
        locals.var_lambdab_dn8 = assign42410_e47866_d_n8;
        locals.var_lambdab_dn9 = assign42410_e47866_d_n9;

        let (assign42420_e47874, assign42420_e47874_d_n4, assign42420_e47874_d_n6, assign42420_e47874_d_n7, assign42420_e47874_d_n8, assign42420_e47874_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42420_e47870: f64 = (locals.var_lambdaf * locals.var_lambdaf);
        let assign42420_e47872: f64 = (assign42420_e47870 * locals.var_inner_sd);
        (assign42420_e47872, ((((locals.var_lambdaf_dn4 * locals.var_lambdaf) + (locals.var_lambdaf * locals.var_lambdaf_dn4)) * locals.var_inner_sd) + (assign42420_e47870 * locals.var_inner_sd_dn4)), ((((locals.var_lambdaf_dn6 * locals.var_lambdaf) + (locals.var_lambdaf * locals.var_lambdaf_dn6)) * locals.var_inner_sd) + (assign42420_e47870 * locals.var_inner_sd_dn6)), ((((locals.var_lambdaf_dn7 * locals.var_lambdaf) + (locals.var_lambdaf * locals.var_lambdaf_dn7)) * locals.var_inner_sd) + (assign42420_e47870 * locals.var_inner_sd_dn7)), ((((locals.var_lambdaf_dn8 * locals.var_lambdaf) + (locals.var_lambdaf * locals.var_lambdaf_dn8)) * locals.var_inner_sd) + (assign42420_e47870 * locals.var_inner_sd_dn8)), ((((locals.var_lambdaf_dn9 * locals.var_lambdaf) + (locals.var_lambdaf * locals.var_lambdaf_dn9)) * locals.var_inner_sd) + (assign42420_e47870 * locals.var_inner_sd_dn9)),)
    } else {
        (locals.var_xalphaf, locals.var_xalphaf_dn4, locals.var_xalphaf_dn6, locals.var_xalphaf_dn7, locals.var_xalphaf_dn8, locals.var_xalphaf_dn9,)
    }
};
        locals.var_xalphaf = assign42420_e47874;
        locals.var_xalphaf_dn4 = assign42420_e47874_d_n4;
        locals.var_xalphaf_dn6 = assign42420_e47874_d_n6;
        locals.var_xalphaf_dn7 = assign42420_e47874_d_n7;
        locals.var_xalphaf_dn8 = assign42420_e47874_d_n8;
        locals.var_xalphaf_dn9 = assign42420_e47874_d_n9;

        let (assign42430_e47882, assign42430_e47882_d_n4, assign42430_e47882_d_n6, assign42430_e47882_d_n7, assign42430_e47882_d_n8, assign42430_e47882_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42430_e47878: f64 = (locals.var_lambdab * locals.var_lambdab);
        let assign42430_e47880: f64 = (assign42430_e47878 * locals.var_inner_sd);
        (assign42430_e47880, ((((locals.var_lambdab_dn4 * locals.var_lambdab) + (locals.var_lambdab * locals.var_lambdab_dn4)) * locals.var_inner_sd) + (assign42430_e47878 * locals.var_inner_sd_dn4)), ((((locals.var_lambdab_dn6 * locals.var_lambdab) + (locals.var_lambdab * locals.var_lambdab_dn6)) * locals.var_inner_sd) + (assign42430_e47878 * locals.var_inner_sd_dn6)), ((((locals.var_lambdab_dn7 * locals.var_lambdab) + (locals.var_lambdab * locals.var_lambdab_dn7)) * locals.var_inner_sd) + (assign42430_e47878 * locals.var_inner_sd_dn7)), ((((locals.var_lambdab_dn8 * locals.var_lambdab) + (locals.var_lambdab * locals.var_lambdab_dn8)) * locals.var_inner_sd) + (assign42430_e47878 * locals.var_inner_sd_dn8)), ((((locals.var_lambdab_dn9 * locals.var_lambdab) + (locals.var_lambdab * locals.var_lambdab_dn9)) * locals.var_inner_sd) + (assign42430_e47878 * locals.var_inner_sd_dn9)),)
    } else {
        (locals.var_xalphab, locals.var_xalphab_dn4, locals.var_xalphab_dn6, locals.var_xalphab_dn7, locals.var_xalphab_dn8, locals.var_xalphab_dn9,)
    }
};
        locals.var_xalphab = assign42430_e47882;
        locals.var_xalphab_dn4 = assign42430_e47882_d_n4;
        locals.var_xalphab_dn6 = assign42430_e47882_d_n6;
        locals.var_xalphab_dn7 = assign42430_e47882_d_n7;
        locals.var_xalphab_dn8 = assign42430_e47882_d_n8;
        locals.var_xalphab_dn9 = assign42430_e47882_d_n9;

        let (assign42440_e47888, assign42440_e47888_d_n4, assign42440_e47888_d_n6, assign42440_e47888_d_n7, assign42440_e47888_d_n8, assign42440_e47888_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42440_e47886: f64 = (locals.var_xsd - locals.var_xstars);
        (assign42440_e47886, (locals.var_xsd_dn4 - locals.var_xstars_dn4), (locals.var_xsd_dn6 - locals.var_xstars_dn6), (locals.var_xsd_dn7 - locals.var_xstars_dn7), (locals.var_xsd_dn8 - locals.var_xstars_dn8), (locals.var_xsd_dn9 - locals.var_xstars_dn9),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign42440_e47888;
        locals.var_temp1_dn4 = assign42440_e47888_d_n4;
        locals.var_temp1_dn6 = assign42440_e47888_d_n6;
        locals.var_temp1_dn7 = assign42440_e47888_d_n7;
        locals.var_temp1_dn8 = assign42440_e47888_d_n8;
        locals.var_temp1_dn9 = assign42440_e47888_d_n9;

        let (assign42450_e47896, assign42450_e47896_d_n4, assign42450_e47896_d_n6, assign42450_e47896_d_n7, assign42450_e47896_d_n8, assign42450_e47896_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42450_e47892: f64 = (locals.var_xsd + locals.var_xd);
        let assign42450_e47894: f64 = (assign42450_e47892 - locals.var_xstard);
        (assign42450_e47894, ((locals.var_xsd_dn4 + locals.var_xd_dn4) - locals.var_xstard_dn4), ((locals.var_xsd_dn6 + locals.var_xd_dn6) - locals.var_xstard_dn6), ((locals.var_xsd_dn7 + locals.var_xd_dn7) - locals.var_xstard_dn7), ((locals.var_xsd_dn8 + locals.var_xd_dn8) - locals.var_xstard_dn8), ((locals.var_xsd_dn9 + locals.var_xd_dn9) - locals.var_xstard_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign42450_e47896;
        locals.var_temp2_dn4 = assign42450_e47896_d_n4;
        locals.var_temp2_dn6 = assign42450_e47896_d_n6;
        locals.var_temp2_dn7 = assign42450_e47896_d_n7;
        locals.var_temp2_dn8 = assign42450_e47896_d_n8;
        locals.var_temp2_dn9 = assign42450_e47896_d_n9;

        let (assign42460_e47902, assign42460_e47902_d_n4, assign42460_e47902_d_n6, assign42460_e47902_d_n7, assign42460_e47902_d_n8, assign42460_e47902_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42460_e47900: f64 = (2.0 * locals.var_xalphaf);
        (assign42460_e47900, (2.0 * locals.var_xalphaf_dn4), (2.0 * locals.var_xalphaf_dn6), (2.0 * locals.var_xalphaf_dn7), (2.0 * locals.var_xalphaf_dn8), (2.0 * locals.var_xalphaf_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign42460_e47902;
        locals.var_temp_dn4 = assign42460_e47902_d_n4;
        locals.var_temp_dn6 = assign42460_e47902_d_n6;
        locals.var_temp_dn7 = assign42460_e47902_d_n7;
        locals.var_temp_dn8 = assign42460_e47902_d_n8;
        locals.var_temp_dn9 = assign42460_e47902_d_n9;

        let (assign42470_e47917, assign42470_e47917_d_n4, assign42470_e47917_d_n6, assign42470_e47917_d_n7, assign42470_e47917_d_n8, assign42470_e47917_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42470_e47909: f64 = (locals.var_temp1 / locals.var_xalphaf);
        let assign42470_e47910: f64 = (1.0 + assign42470_e47909);
        let assign42470_e47911: f64 = (assign42470_e47910).sqrt();
        let assign42470_e47913: f64 = (assign42470_e47911 - 1.0);
        let assign42470_e47914: f64 = (locals.var_temp * assign42470_e47913);
        let assign42470_e47915: f64 = (locals.var_xstars + assign42470_e47914);
        (assign42470_e47915, (locals.var_xstars_dn4 + ((locals.var_temp_dn4 * assign42470_e47913) + (locals.var_temp * ((((locals.var_temp1_dn4 * locals.var_xalphaf) - (locals.var_temp1 * locals.var_xalphaf_dn4)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42470_e47911))))), (locals.var_xstars_dn6 + ((locals.var_temp_dn6 * assign42470_e47913) + (locals.var_temp * ((((locals.var_temp1_dn6 * locals.var_xalphaf) - (locals.var_temp1 * locals.var_xalphaf_dn6)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42470_e47911))))), (locals.var_xstars_dn7 + ((locals.var_temp_dn7 * assign42470_e47913) + (locals.var_temp * ((((locals.var_temp1_dn7 * locals.var_xalphaf) - (locals.var_temp1 * locals.var_xalphaf_dn7)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42470_e47911))))), (locals.var_xstars_dn8 + ((locals.var_temp_dn8 * assign42470_e47913) + (locals.var_temp * ((((locals.var_temp1_dn8 * locals.var_xalphaf) - (locals.var_temp1 * locals.var_xalphaf_dn8)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42470_e47911))))), (locals.var_xstars_dn9 + ((locals.var_temp_dn9 * assign42470_e47913) + (locals.var_temp * ((((locals.var_temp1_dn9 * locals.var_xalphaf) - (locals.var_temp1 * locals.var_xalphaf_dn9)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42470_e47911))))),)
    } else {
        (locals.var_xedgefs, locals.var_xedgefs_dn4, locals.var_xedgefs_dn6, locals.var_xedgefs_dn7, locals.var_xedgefs_dn8, locals.var_xedgefs_dn9,)
    }
};
        locals.var_xedgefs = assign42470_e47917;
        locals.var_xedgefs_dn4 = assign42470_e47917_d_n4;
        locals.var_xedgefs_dn6 = assign42470_e47917_d_n6;
        locals.var_xedgefs_dn7 = assign42470_e47917_d_n7;
        locals.var_xedgefs_dn8 = assign42470_e47917_d_n8;
        locals.var_xedgefs_dn9 = assign42470_e47917_d_n9;

        let (assign42480_e47932, assign42480_e47932_d_n4, assign42480_e47932_d_n6, assign42480_e47932_d_n7, assign42480_e47932_d_n8, assign42480_e47932_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42480_e47924: f64 = (locals.var_temp2 / locals.var_xalphaf);
        let assign42480_e47925: f64 = (1.0 + assign42480_e47924);
        let assign42480_e47926: f64 = (assign42480_e47925).sqrt();
        let assign42480_e47928: f64 = (assign42480_e47926 - 1.0);
        let assign42480_e47929: f64 = (locals.var_temp * assign42480_e47928);
        let assign42480_e47930: f64 = (locals.var_xstard + assign42480_e47929);
        (assign42480_e47930, (locals.var_xstard_dn4 + ((locals.var_temp_dn4 * assign42480_e47928) + (locals.var_temp * ((((locals.var_temp2_dn4 * locals.var_xalphaf) - (locals.var_temp2 * locals.var_xalphaf_dn4)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42480_e47926))))), (locals.var_xstard_dn6 + ((locals.var_temp_dn6 * assign42480_e47928) + (locals.var_temp * ((((locals.var_temp2_dn6 * locals.var_xalphaf) - (locals.var_temp2 * locals.var_xalphaf_dn6)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42480_e47926))))), (locals.var_xstard_dn7 + ((locals.var_temp_dn7 * assign42480_e47928) + (locals.var_temp * ((((locals.var_temp2_dn7 * locals.var_xalphaf) - (locals.var_temp2 * locals.var_xalphaf_dn7)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42480_e47926))))), (locals.var_xstard_dn8 + ((locals.var_temp_dn8 * assign42480_e47928) + (locals.var_temp * ((((locals.var_temp2_dn8 * locals.var_xalphaf) - (locals.var_temp2 * locals.var_xalphaf_dn8)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42480_e47926))))), (locals.var_xstard_dn9 + ((locals.var_temp_dn9 * assign42480_e47928) + (locals.var_temp * ((((locals.var_temp2_dn9 * locals.var_xalphaf) - (locals.var_temp2 * locals.var_xalphaf_dn9)) / (locals.var_xalphaf * locals.var_xalphaf)) / (2.0 * assign42480_e47926))))),)
    } else {
        (locals.var_xedgefd, locals.var_xedgefd_dn4, locals.var_xedgefd_dn6, locals.var_xedgefd_dn7, locals.var_xedgefd_dn8, locals.var_xedgefd_dn9,)
    }
};
        locals.var_xedgefd = assign42480_e47932;
        locals.var_xedgefd_dn4 = assign42480_e47932_d_n4;
        locals.var_xedgefd_dn6 = assign42480_e47932_d_n6;
        locals.var_xedgefd_dn7 = assign42480_e47932_d_n7;
        locals.var_xedgefd_dn8 = assign42480_e47932_d_n8;
        locals.var_xedgefd_dn9 = assign42480_e47932_d_n9;

        let (assign42490_e47938, assign42490_e47938_d_n4, assign42490_e47938_d_n6, assign42490_e47938_d_n7, assign42490_e47938_d_n8, assign42490_e47938_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42490_e47936: f64 = (2.0 * locals.var_xalphab);
        (assign42490_e47936, (2.0 * locals.var_xalphab_dn4), (2.0 * locals.var_xalphab_dn6), (2.0 * locals.var_xalphab_dn7), (2.0 * locals.var_xalphab_dn8), (2.0 * locals.var_xalphab_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign42490_e47938;
        locals.var_temp_dn4 = assign42490_e47938_d_n4;
        locals.var_temp_dn6 = assign42490_e47938_d_n6;
        locals.var_temp_dn7 = assign42490_e47938_d_n7;
        locals.var_temp_dn8 = assign42490_e47938_d_n8;
        locals.var_temp_dn9 = assign42490_e47938_d_n9;

        let (assign42500_e47953, assign42500_e47953_d_n4, assign42500_e47953_d_n6, assign42500_e47953_d_n7, assign42500_e47953_d_n8, assign42500_e47953_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42500_e47945: f64 = (locals.var_temp1 / locals.var_xalphab);
        let assign42500_e47946: f64 = (1.0 + assign42500_e47945);
        let assign42500_e47947: f64 = (assign42500_e47946).sqrt();
        let assign42500_e47949: f64 = (assign42500_e47947 - 1.0);
        let assign42500_e47950: f64 = (locals.var_temp * assign42500_e47949);
        let assign42500_e47951: f64 = (locals.var_xstars + assign42500_e47950);
        (assign42500_e47951, (locals.var_xstars_dn4 + ((locals.var_temp_dn4 * assign42500_e47949) + (locals.var_temp * ((((locals.var_temp1_dn4 * locals.var_xalphab) - (locals.var_temp1 * locals.var_xalphab_dn4)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42500_e47947))))), (locals.var_xstars_dn6 + ((locals.var_temp_dn6 * assign42500_e47949) + (locals.var_temp * ((((locals.var_temp1_dn6 * locals.var_xalphab) - (locals.var_temp1 * locals.var_xalphab_dn6)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42500_e47947))))), (locals.var_xstars_dn7 + ((locals.var_temp_dn7 * assign42500_e47949) + (locals.var_temp * ((((locals.var_temp1_dn7 * locals.var_xalphab) - (locals.var_temp1 * locals.var_xalphab_dn7)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42500_e47947))))), (locals.var_xstars_dn8 + ((locals.var_temp_dn8 * assign42500_e47949) + (locals.var_temp * ((((locals.var_temp1_dn8 * locals.var_xalphab) - (locals.var_temp1 * locals.var_xalphab_dn8)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42500_e47947))))), (locals.var_xstars_dn9 + ((locals.var_temp_dn9 * assign42500_e47949) + (locals.var_temp * ((((locals.var_temp1_dn9 * locals.var_xalphab) - (locals.var_temp1 * locals.var_xalphab_dn9)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42500_e47947))))),)
    } else {
        (locals.var_xedgebs, locals.var_xedgebs_dn4, locals.var_xedgebs_dn6, locals.var_xedgebs_dn7, locals.var_xedgebs_dn8, locals.var_xedgebs_dn9,)
    }
};
        locals.var_xedgebs = assign42500_e47953;
        locals.var_xedgebs_dn4 = assign42500_e47953_d_n4;
        locals.var_xedgebs_dn6 = assign42500_e47953_d_n6;
        locals.var_xedgebs_dn7 = assign42500_e47953_d_n7;
        locals.var_xedgebs_dn8 = assign42500_e47953_d_n8;
        locals.var_xedgebs_dn9 = assign42500_e47953_d_n9;

        let (assign42510_e47968, assign42510_e47968_d_n4, assign42510_e47968_d_n6, assign42510_e47968_d_n7, assign42510_e47968_d_n8, assign42510_e47968_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42510_e47960: f64 = (locals.var_temp2 / locals.var_xalphab);
        let assign42510_e47961: f64 = (1.0 + assign42510_e47960);
        let assign42510_e47962: f64 = (assign42510_e47961).sqrt();
        let assign42510_e47964: f64 = (assign42510_e47962 - 1.0);
        let assign42510_e47965: f64 = (locals.var_temp * assign42510_e47964);
        let assign42510_e47966: f64 = (locals.var_xstard + assign42510_e47965);
        (assign42510_e47966, (locals.var_xstard_dn4 + ((locals.var_temp_dn4 * assign42510_e47964) + (locals.var_temp * ((((locals.var_temp2_dn4 * locals.var_xalphab) - (locals.var_temp2 * locals.var_xalphab_dn4)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42510_e47962))))), (locals.var_xstard_dn6 + ((locals.var_temp_dn6 * assign42510_e47964) + (locals.var_temp * ((((locals.var_temp2_dn6 * locals.var_xalphab) - (locals.var_temp2 * locals.var_xalphab_dn6)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42510_e47962))))), (locals.var_xstard_dn7 + ((locals.var_temp_dn7 * assign42510_e47964) + (locals.var_temp * ((((locals.var_temp2_dn7 * locals.var_xalphab) - (locals.var_temp2 * locals.var_xalphab_dn7)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42510_e47962))))), (locals.var_xstard_dn8 + ((locals.var_temp_dn8 * assign42510_e47964) + (locals.var_temp * ((((locals.var_temp2_dn8 * locals.var_xalphab) - (locals.var_temp2 * locals.var_xalphab_dn8)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42510_e47962))))), (locals.var_xstard_dn9 + ((locals.var_temp_dn9 * assign42510_e47964) + (locals.var_temp * ((((locals.var_temp2_dn9 * locals.var_xalphab) - (locals.var_temp2 * locals.var_xalphab_dn9)) / (locals.var_xalphab * locals.var_xalphab)) / (2.0 * assign42510_e47962))))),)
    } else {
        (locals.var_xedgebd, locals.var_xedgebd_dn4, locals.var_xedgebd_dn6, locals.var_xedgebd_dn7, locals.var_xedgebd_dn8, locals.var_xedgebd_dn9,)
    }
};
        locals.var_xedgebd = assign42510_e47968;
        locals.var_xedgebd_dn4 = assign42510_e47968_d_n4;
        locals.var_xedgebd_dn6 = assign42510_e47968_d_n6;
        locals.var_xedgebd_dn7 = assign42510_e47968_d_n7;
        locals.var_xedgebd_dn8 = assign42510_e47968_d_n8;
        locals.var_xedgebd_dn9 = assign42510_e47968_d_n9;

        let (assign42520_e47974, assign42520_e47974_d_n4, assign42520_e47974_d_n6, assign42520_e47974_d_n7, assign42520_e47974_d_n8, assign42520_e47974_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42520_e47972: f64 = (locals.var_fif_phit * locals.var_csiprime_ac);
        (assign42520_e47972, ((locals.var_fif_phit_dn4 * locals.var_csiprime_ac) + (locals.var_fif_phit * locals.var_csiprime_ac_dn4)), ((locals.var_fif_phit_dn6 * locals.var_csiprime_ac) + (locals.var_fif_phit * locals.var_csiprime_ac_dn6)), ((locals.var_fif_phit_dn7 * locals.var_csiprime_ac) + (locals.var_fif_phit * locals.var_csiprime_ac_dn7)), ((locals.var_fif_phit_dn8 * locals.var_csiprime_ac) + (locals.var_fif_phit * locals.var_csiprime_ac_dn8)), ((locals.var_fif_phit_dn9 * locals.var_csiprime_ac) + (locals.var_fif_phit * locals.var_csiprime_ac_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign42520_e47974;
        locals.var_temp_dn4 = assign42520_e47974_d_n4;
        locals.var_temp_dn6 = assign42520_e47974_d_n6;
        locals.var_temp_dn7 = assign42520_e47974_d_n7;
        locals.var_temp_dn8 = assign42520_e47974_d_n8;
        locals.var_temp_dn9 = assign42520_e47974_d_n9;

        let (assign42530_e47985, assign42530_e47985_d_n4, assign42530_e47985_d_n6, assign42530_e47985_d_n7, assign42530_e47985_d_n8, assign42530_e47985_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42530_e47977: f64 = (-locals.var_temp);
        let assign42530_e47979: f64 = (assign42530_e47977 * locals.var_lambdaf);
        let assign42530_e47981: f64 = (assign42530_e47979 * locals.var_k1_ac);
        let assign42530_e47983: f64 = (assign42530_e47981 * locals.var_sce1_ac);
        (assign42530_e47983, (((((((-locals.var_temp_dn4) * locals.var_lambdaf) + (assign42530_e47977 * locals.var_lambdaf_dn4)) * locals.var_k1_ac) + (assign42530_e47979 * locals.var_k1_ac_dn4)) * locals.var_sce1_ac) + (assign42530_e47981 * locals.var_sce1_ac_dn4)), (((((((-locals.var_temp_dn6) * locals.var_lambdaf) + (assign42530_e47977 * locals.var_lambdaf_dn6)) * locals.var_k1_ac) + (assign42530_e47979 * locals.var_k1_ac_dn6)) * locals.var_sce1_ac) + (assign42530_e47981 * locals.var_sce1_ac_dn6)), (((((((-locals.var_temp_dn7) * locals.var_lambdaf) + (assign42530_e47977 * locals.var_lambdaf_dn7)) * locals.var_k1_ac) + (assign42530_e47979 * locals.var_k1_ac_dn7)) * locals.var_sce1_ac) + (assign42530_e47981 * locals.var_sce1_ac_dn7)), (((((((-locals.var_temp_dn8) * locals.var_lambdaf) + (assign42530_e47977 * locals.var_lambdaf_dn8)) * locals.var_k1_ac) + (assign42530_e47979 * locals.var_k1_ac_dn8)) * locals.var_sce1_ac) + (assign42530_e47981 * locals.var_sce1_ac_dn8)), (((((((-locals.var_temp_dn9) * locals.var_lambdaf) + (assign42530_e47977 * locals.var_lambdaf_dn9)) * locals.var_k1_ac) + (assign42530_e47979 * locals.var_k1_ac_dn9)) * locals.var_sce1_ac) + (assign42530_e47981 * locals.var_sce1_ac_dn9)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign42530_e47985;
        locals.var_temp1_dn4 = assign42530_e47985_d_n4;
        locals.var_temp1_dn6 = assign42530_e47985_d_n6;
        locals.var_temp1_dn7 = assign42530_e47985_d_n7;
        locals.var_temp1_dn8 = assign42530_e47985_d_n8;
        locals.var_temp1_dn9 = assign42530_e47985_d_n9;

        let (assign42540_e47996, assign42540_e47996_d_n4, assign42540_e47996_d_n6, assign42540_e47996_d_n7, assign42540_e47996_d_n8, assign42540_e47996_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42540_e47988: f64 = (-locals.var_temp);
        let assign42540_e47990: f64 = (assign42540_e47988 * locals.var_lambdab);
        let assign42540_e47992: f64 = (assign42540_e47990 * locals.var_k2_ac);
        let assign42540_e47994: f64 = (assign42540_e47992 * locals.var_sce2_ac);
        (assign42540_e47994, (((((((-locals.var_temp_dn4) * locals.var_lambdab) + (assign42540_e47988 * locals.var_lambdab_dn4)) * locals.var_k2_ac) + (assign42540_e47990 * locals.var_k2_ac_dn4)) * locals.var_sce2_ac) + (assign42540_e47992 * locals.var_sce2_ac_dn4)), (((((((-locals.var_temp_dn6) * locals.var_lambdab) + (assign42540_e47988 * locals.var_lambdab_dn6)) * locals.var_k2_ac) + (assign42540_e47990 * locals.var_k2_ac_dn6)) * locals.var_sce2_ac) + (assign42540_e47992 * locals.var_sce2_ac_dn6)), (((((((-locals.var_temp_dn7) * locals.var_lambdab) + (assign42540_e47988 * locals.var_lambdab_dn7)) * locals.var_k2_ac) + (assign42540_e47990 * locals.var_k2_ac_dn7)) * locals.var_sce2_ac) + (assign42540_e47992 * locals.var_sce2_ac_dn7)), (((((((-locals.var_temp_dn8) * locals.var_lambdab) + (assign42540_e47988 * locals.var_lambdab_dn8)) * locals.var_k2_ac) + (assign42540_e47990 * locals.var_k2_ac_dn8)) * locals.var_sce2_ac) + (assign42540_e47992 * locals.var_sce2_ac_dn8)), (((((((-locals.var_temp_dn9) * locals.var_lambdab) + (assign42540_e47988 * locals.var_lambdab_dn9)) * locals.var_k2_ac) + (assign42540_e47990 * locals.var_k2_ac_dn9)) * locals.var_sce2_ac) + (assign42540_e47992 * locals.var_sce2_ac_dn9)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign42540_e47996;
        locals.var_temp2_dn4 = assign42540_e47996_d_n4;
        locals.var_temp2_dn6 = assign42540_e47996_d_n6;
        locals.var_temp2_dn7 = assign42540_e47996_d_n7;
        locals.var_temp2_dn8 = assign42540_e47996_d_n8;
        locals.var_temp2_dn9 = assign42540_e47996_d_n9;

        let (assign42550_e48021, assign42550_e48021_d_n4, assign42550_e48021_d_n6, assign42550_e48021_d_n7, assign42550_e48021_d_n8, assign42550_e48021_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42550_e48001: f64 = (locals.var_xedgefs - locals.var_xeffs);
        let assign42550_e48003: f64 = assign42550_e48001;
        let assign42550_e48006: f64 = (locals.var_xedgefs - locals.var_xeffs);
        let assign42550_e48008: f64 = assign42550_e48006;
        let assign42550_e48011: f64 = (locals.var_xedgefs - locals.var_xeffs);
        let assign42550_e48013: f64 = assign42550_e48011;
        let assign42550_e48014: f64 = (assign42550_e48008 * assign42550_e48013);
        let assign42550_e48016: f64 = (assign42550_e48014 + 1.0);
        let assign42550_e48017: f64 = (assign42550_e48016).sqrt();
        let assign42550_e48018: f64 = (assign42550_e48003 + assign42550_e48017);
        let assign42550_e48019: f64 = (0.5 * assign42550_e48018);
        (assign42550_e48019, (0.5 * ((locals.var_xedgefs_dn4 - locals.var_xeffs_dn4) + ((((locals.var_xedgefs_dn4 - locals.var_xeffs_dn4) * assign42550_e48013) + (assign42550_e48008 * (locals.var_xedgefs_dn4 - locals.var_xeffs_dn4))) / (2.0 * assign42550_e48017)))), (0.5 * ((locals.var_xedgefs_dn6 - locals.var_xeffs_dn6) + ((((locals.var_xedgefs_dn6 - locals.var_xeffs_dn6) * assign42550_e48013) + (assign42550_e48008 * (locals.var_xedgefs_dn6 - locals.var_xeffs_dn6))) / (2.0 * assign42550_e48017)))), (0.5 * ((locals.var_xedgefs_dn7 - locals.var_xeffs_dn7) + ((((locals.var_xedgefs_dn7 - locals.var_xeffs_dn7) * assign42550_e48013) + (assign42550_e48008 * (locals.var_xedgefs_dn7 - locals.var_xeffs_dn7))) / (2.0 * assign42550_e48017)))), (0.5 * ((locals.var_xedgefs_dn8 - locals.var_xeffs_dn8) + ((((locals.var_xedgefs_dn8 - locals.var_xeffs_dn8) * assign42550_e48013) + (assign42550_e48008 * (locals.var_xedgefs_dn8 - locals.var_xeffs_dn8))) / (2.0 * assign42550_e48017)))), (0.5 * ((locals.var_xedgefs_dn9 - locals.var_xeffs_dn9) + ((((locals.var_xedgefs_dn9 - locals.var_xeffs_dn9) * assign42550_e48013) + (assign42550_e48008 * (locals.var_xedgefs_dn9 - locals.var_xeffs_dn9))) / (2.0 * assign42550_e48017)))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign42550_e48021;
        locals.var_temp_dn4 = assign42550_e48021_d_n4;
        locals.var_temp_dn6 = assign42550_e48021_d_n6;
        locals.var_temp_dn7 = assign42550_e48021_d_n7;
        locals.var_temp_dn8 = assign42550_e48021_d_n8;
        locals.var_temp_dn9 = assign42550_e48021_d_n9;

        let (assign42560_e48033, assign42560_e48033_d_n4, assign42560_e48033_d_n6, assign42560_e48033_d_n7, assign42560_e48033_d_n8, assign42560_e48033_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42560_e48025: f64 = (locals.var_temp1 * locals.var_temp);
        let assign42560_e48027: f64 = (assign42560_e48025 * locals.var_temp);
        let assign42560_e48030: f64 = (locals.var_xedgefs - locals.var_xstars);
        let assign42560_e48031: f64 = (assign42560_e48027 / assign42560_e48030);
        (assign42560_e48031, (((((((locals.var_temp1_dn4 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn4)) * locals.var_temp) + (assign42560_e48025 * locals.var_temp_dn4)) * assign42560_e48030) - (assign42560_e48027 * (locals.var_xedgefs_dn4 - locals.var_xstars_dn4))) / (assign42560_e48030 * assign42560_e48030)), (((((((locals.var_temp1_dn6 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn6)) * locals.var_temp) + (assign42560_e48025 * locals.var_temp_dn6)) * assign42560_e48030) - (assign42560_e48027 * (locals.var_xedgefs_dn6 - locals.var_xstars_dn6))) / (assign42560_e48030 * assign42560_e48030)), (((((((locals.var_temp1_dn7 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn7)) * locals.var_temp) + (assign42560_e48025 * locals.var_temp_dn7)) * assign42560_e48030) - (assign42560_e48027 * (locals.var_xedgefs_dn7 - locals.var_xstars_dn7))) / (assign42560_e48030 * assign42560_e48030)), (((((((locals.var_temp1_dn8 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn8)) * locals.var_temp) + (assign42560_e48025 * locals.var_temp_dn8)) * assign42560_e48030) - (assign42560_e48027 * (locals.var_xedgefs_dn8 - locals.var_xstars_dn8))) / (assign42560_e48030 * assign42560_e48030)), (((((((locals.var_temp1_dn9 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn9)) * locals.var_temp) + (assign42560_e48025 * locals.var_temp_dn9)) * assign42560_e48030) - (assign42560_e48027 * (locals.var_xedgefs_dn9 - locals.var_xstars_dn9))) / (assign42560_e48030 * assign42560_e48030)),)
    } else {
        (locals.var_qgsif, locals.var_qgsif_dn4, locals.var_qgsif_dn6, locals.var_qgsif_dn7, locals.var_qgsif_dn8, locals.var_qgsif_dn9,)
    }
};
        locals.var_qgsif = assign42560_e48033;
        locals.var_qgsif_dn4 = assign42560_e48033_d_n4;
        locals.var_qgsif_dn6 = assign42560_e48033_d_n6;
        locals.var_qgsif_dn7 = assign42560_e48033_d_n7;
        locals.var_qgsif_dn8 = assign42560_e48033_d_n8;
        locals.var_qgsif_dn9 = assign42560_e48033_d_n9;

        let (assign42570_e48058, assign42570_e48058_d_n4, assign42570_e48058_d_n6, assign42570_e48058_d_n7, assign42570_e48058_d_n8, assign42570_e48058_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42570_e48038: f64 = (locals.var_xedgefd - locals.var_xeffd);
        let assign42570_e48040: f64 = assign42570_e48038;
        let assign42570_e48043: f64 = (locals.var_xedgefd - locals.var_xeffd);
        let assign42570_e48045: f64 = assign42570_e48043;
        let assign42570_e48048: f64 = (locals.var_xedgefd - locals.var_xeffd);
        let assign42570_e48050: f64 = assign42570_e48048;
        let assign42570_e48051: f64 = (assign42570_e48045 * assign42570_e48050);
        let assign42570_e48053: f64 = (assign42570_e48051 + 1.0);
        let assign42570_e48054: f64 = (assign42570_e48053).sqrt();
        let assign42570_e48055: f64 = (assign42570_e48040 + assign42570_e48054);
        let assign42570_e48056: f64 = (0.5 * assign42570_e48055);
        (assign42570_e48056, (0.5 * ((locals.var_xedgefd_dn4 - locals.var_xeffd_dn4) + ((((locals.var_xedgefd_dn4 - locals.var_xeffd_dn4) * assign42570_e48050) + (assign42570_e48045 * (locals.var_xedgefd_dn4 - locals.var_xeffd_dn4))) / (2.0 * assign42570_e48054)))), (0.5 * ((locals.var_xedgefd_dn6 - locals.var_xeffd_dn6) + ((((locals.var_xedgefd_dn6 - locals.var_xeffd_dn6) * assign42570_e48050) + (assign42570_e48045 * (locals.var_xedgefd_dn6 - locals.var_xeffd_dn6))) / (2.0 * assign42570_e48054)))), (0.5 * ((locals.var_xedgefd_dn7 - locals.var_xeffd_dn7) + ((((locals.var_xedgefd_dn7 - locals.var_xeffd_dn7) * assign42570_e48050) + (assign42570_e48045 * (locals.var_xedgefd_dn7 - locals.var_xeffd_dn7))) / (2.0 * assign42570_e48054)))), (0.5 * ((locals.var_xedgefd_dn8 - locals.var_xeffd_dn8) + ((((locals.var_xedgefd_dn8 - locals.var_xeffd_dn8) * assign42570_e48050) + (assign42570_e48045 * (locals.var_xedgefd_dn8 - locals.var_xeffd_dn8))) / (2.0 * assign42570_e48054)))), (0.5 * ((locals.var_xedgefd_dn9 - locals.var_xeffd_dn9) + ((((locals.var_xedgefd_dn9 - locals.var_xeffd_dn9) * assign42570_e48050) + (assign42570_e48045 * (locals.var_xedgefd_dn9 - locals.var_xeffd_dn9))) / (2.0 * assign42570_e48054)))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign42570_e48058;
        locals.var_temp_dn4 = assign42570_e48058_d_n4;
        locals.var_temp_dn6 = assign42570_e48058_d_n6;
        locals.var_temp_dn7 = assign42570_e48058_d_n7;
        locals.var_temp_dn8 = assign42570_e48058_d_n8;
        locals.var_temp_dn9 = assign42570_e48058_d_n9;

        let (assign42580_e48070, assign42580_e48070_d_n4, assign42580_e48070_d_n6, assign42580_e48070_d_n7, assign42580_e48070_d_n8, assign42580_e48070_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42580_e48062: f64 = (locals.var_temp1 * locals.var_temp);
        let assign42580_e48064: f64 = (assign42580_e48062 * locals.var_temp);
        let assign42580_e48067: f64 = (locals.var_xedgefd - locals.var_xstard);
        let assign42580_e48068: f64 = (assign42580_e48064 / assign42580_e48067);
        (assign42580_e48068, (((((((locals.var_temp1_dn4 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn4)) * locals.var_temp) + (assign42580_e48062 * locals.var_temp_dn4)) * assign42580_e48067) - (assign42580_e48064 * (locals.var_xedgefd_dn4 - locals.var_xstard_dn4))) / (assign42580_e48067 * assign42580_e48067)), (((((((locals.var_temp1_dn6 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn6)) * locals.var_temp) + (assign42580_e48062 * locals.var_temp_dn6)) * assign42580_e48067) - (assign42580_e48064 * (locals.var_xedgefd_dn6 - locals.var_xstard_dn6))) / (assign42580_e48067 * assign42580_e48067)), (((((((locals.var_temp1_dn7 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn7)) * locals.var_temp) + (assign42580_e48062 * locals.var_temp_dn7)) * assign42580_e48067) - (assign42580_e48064 * (locals.var_xedgefd_dn7 - locals.var_xstard_dn7))) / (assign42580_e48067 * assign42580_e48067)), (((((((locals.var_temp1_dn8 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn8)) * locals.var_temp) + (assign42580_e48062 * locals.var_temp_dn8)) * assign42580_e48067) - (assign42580_e48064 * (locals.var_xedgefd_dn8 - locals.var_xstard_dn8))) / (assign42580_e48067 * assign42580_e48067)), (((((((locals.var_temp1_dn9 * locals.var_temp) + (locals.var_temp1 * locals.var_temp_dn9)) * locals.var_temp) + (assign42580_e48062 * locals.var_temp_dn9)) * assign42580_e48067) - (assign42580_e48064 * (locals.var_xedgefd_dn9 - locals.var_xstard_dn9))) / (assign42580_e48067 * assign42580_e48067)),)
    } else {
        (locals.var_qgdif, locals.var_qgdif_dn4, locals.var_qgdif_dn6, locals.var_qgdif_dn7, locals.var_qgdif_dn8, locals.var_qgdif_dn9,)
    }
};
        locals.var_qgdif = assign42580_e48070;
        locals.var_qgdif_dn4 = assign42580_e48070_d_n4;
        locals.var_qgdif_dn6 = assign42580_e48070_d_n6;
        locals.var_qgdif_dn7 = assign42580_e48070_d_n7;
        locals.var_qgdif_dn8 = assign42580_e48070_d_n8;
        locals.var_qgdif_dn9 = assign42580_e48070_d_n9;

        let (assign42590_e48095, assign42590_e48095_d_n4, assign42590_e48095_d_n6, assign42590_e48095_d_n7, assign42590_e48095_d_n8, assign42590_e48095_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42590_e48075: f64 = (locals.var_xedgebs - locals.var_xeffs);
        let assign42590_e48077: f64 = assign42590_e48075;
        let assign42590_e48080: f64 = (locals.var_xedgebs - locals.var_xeffs);
        let assign42590_e48082: f64 = assign42590_e48080;
        let assign42590_e48085: f64 = (locals.var_xedgebs - locals.var_xeffs);
        let assign42590_e48087: f64 = assign42590_e48085;
        let assign42590_e48088: f64 = (assign42590_e48082 * assign42590_e48087);
        let assign42590_e48090: f64 = (assign42590_e48088 + 1.0);
        let assign42590_e48091: f64 = (assign42590_e48090).sqrt();
        let assign42590_e48092: f64 = (assign42590_e48077 + assign42590_e48091);
        let assign42590_e48093: f64 = (0.5 * assign42590_e48092);
        (assign42590_e48093, (0.5 * ((locals.var_xedgebs_dn4 - locals.var_xeffs_dn4) + ((((locals.var_xedgebs_dn4 - locals.var_xeffs_dn4) * assign42590_e48087) + (assign42590_e48082 * (locals.var_xedgebs_dn4 - locals.var_xeffs_dn4))) / (2.0 * assign42590_e48091)))), (0.5 * ((locals.var_xedgebs_dn6 - locals.var_xeffs_dn6) + ((((locals.var_xedgebs_dn6 - locals.var_xeffs_dn6) * assign42590_e48087) + (assign42590_e48082 * (locals.var_xedgebs_dn6 - locals.var_xeffs_dn6))) / (2.0 * assign42590_e48091)))), (0.5 * ((locals.var_xedgebs_dn7 - locals.var_xeffs_dn7) + ((((locals.var_xedgebs_dn7 - locals.var_xeffs_dn7) * assign42590_e48087) + (assign42590_e48082 * (locals.var_xedgebs_dn7 - locals.var_xeffs_dn7))) / (2.0 * assign42590_e48091)))), (0.5 * ((locals.var_xedgebs_dn8 - locals.var_xeffs_dn8) + ((((locals.var_xedgebs_dn8 - locals.var_xeffs_dn8) * assign42590_e48087) + (assign42590_e48082 * (locals.var_xedgebs_dn8 - locals.var_xeffs_dn8))) / (2.0 * assign42590_e48091)))), (0.5 * ((locals.var_xedgebs_dn9 - locals.var_xeffs_dn9) + ((((locals.var_xedgebs_dn9 - locals.var_xeffs_dn9) * assign42590_e48087) + (assign42590_e48082 * (locals.var_xedgebs_dn9 - locals.var_xeffs_dn9))) / (2.0 * assign42590_e48091)))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign42590_e48095;
        locals.var_temp_dn4 = assign42590_e48095_d_n4;
        locals.var_temp_dn6 = assign42590_e48095_d_n6;
        locals.var_temp_dn7 = assign42590_e48095_d_n7;
        locals.var_temp_dn8 = assign42590_e48095_d_n8;
        locals.var_temp_dn9 = assign42590_e48095_d_n9;

    }

    pub(super) fn stamp_transient_block_116(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign42600_e48107, assign42600_e48107_d_n4, assign42600_e48107_d_n6, assign42600_e48107_d_n7, assign42600_e48107_d_n8, assign42600_e48107_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42600_e48099: f64 = (locals.var_temp2 * locals.var_temp);
        let assign42600_e48101: f64 = (assign42600_e48099 * locals.var_temp);
        let assign42600_e48104: f64 = (locals.var_xedgebs - locals.var_xstars);
        let assign42600_e48105: f64 = (assign42600_e48101 / assign42600_e48104);
        (assign42600_e48105, (((((((locals.var_temp2_dn4 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn4)) * locals.var_temp) + (assign42600_e48099 * locals.var_temp_dn4)) * assign42600_e48104) - (assign42600_e48101 * (locals.var_xedgebs_dn4 - locals.var_xstars_dn4))) / (assign42600_e48104 * assign42600_e48104)), (((((((locals.var_temp2_dn6 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn6)) * locals.var_temp) + (assign42600_e48099 * locals.var_temp_dn6)) * assign42600_e48104) - (assign42600_e48101 * (locals.var_xedgebs_dn6 - locals.var_xstars_dn6))) / (assign42600_e48104 * assign42600_e48104)), (((((((locals.var_temp2_dn7 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn7)) * locals.var_temp) + (assign42600_e48099 * locals.var_temp_dn7)) * assign42600_e48104) - (assign42600_e48101 * (locals.var_xedgebs_dn7 - locals.var_xstars_dn7))) / (assign42600_e48104 * assign42600_e48104)), (((((((locals.var_temp2_dn8 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn8)) * locals.var_temp) + (assign42600_e48099 * locals.var_temp_dn8)) * assign42600_e48104) - (assign42600_e48101 * (locals.var_xedgebs_dn8 - locals.var_xstars_dn8))) / (assign42600_e48104 * assign42600_e48104)), (((((((locals.var_temp2_dn9 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn9)) * locals.var_temp) + (assign42600_e48099 * locals.var_temp_dn9)) * assign42600_e48104) - (assign42600_e48101 * (locals.var_xedgebs_dn9 - locals.var_xstars_dn9))) / (assign42600_e48104 * assign42600_e48104)),)
    } else {
        (locals.var_qbsif, locals.var_qbsif_dn4, locals.var_qbsif_dn6, locals.var_qbsif_dn7, locals.var_qbsif_dn8, locals.var_qbsif_dn9,)
    }
};
        locals.var_qbsif = assign42600_e48107;
        locals.var_qbsif_dn4 = assign42600_e48107_d_n4;
        locals.var_qbsif_dn6 = assign42600_e48107_d_n6;
        locals.var_qbsif_dn7 = assign42600_e48107_d_n7;
        locals.var_qbsif_dn8 = assign42600_e48107_d_n8;
        locals.var_qbsif_dn9 = assign42600_e48107_d_n9;

        let (assign42610_e48132, assign42610_e48132_d_n4, assign42610_e48132_d_n6, assign42610_e48132_d_n7, assign42610_e48132_d_n8, assign42610_e48132_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42610_e48112: f64 = (locals.var_xedgebd - locals.var_xeffd);
        let assign42610_e48114: f64 = assign42610_e48112;
        let assign42610_e48117: f64 = (locals.var_xedgebd - locals.var_xeffd);
        let assign42610_e48119: f64 = assign42610_e48117;
        let assign42610_e48122: f64 = (locals.var_xedgebd - locals.var_xeffd);
        let assign42610_e48124: f64 = assign42610_e48122;
        let assign42610_e48125: f64 = (assign42610_e48119 * assign42610_e48124);
        let assign42610_e48127: f64 = (assign42610_e48125 + 1.0);
        let assign42610_e48128: f64 = (assign42610_e48127).sqrt();
        let assign42610_e48129: f64 = (assign42610_e48114 + assign42610_e48128);
        let assign42610_e48130: f64 = (0.5 * assign42610_e48129);
        (assign42610_e48130, (0.5 * ((locals.var_xedgebd_dn4 - locals.var_xeffd_dn4) + ((((locals.var_xedgebd_dn4 - locals.var_xeffd_dn4) * assign42610_e48124) + (assign42610_e48119 * (locals.var_xedgebd_dn4 - locals.var_xeffd_dn4))) / (2.0 * assign42610_e48128)))), (0.5 * ((locals.var_xedgebd_dn6 - locals.var_xeffd_dn6) + ((((locals.var_xedgebd_dn6 - locals.var_xeffd_dn6) * assign42610_e48124) + (assign42610_e48119 * (locals.var_xedgebd_dn6 - locals.var_xeffd_dn6))) / (2.0 * assign42610_e48128)))), (0.5 * ((locals.var_xedgebd_dn7 - locals.var_xeffd_dn7) + ((((locals.var_xedgebd_dn7 - locals.var_xeffd_dn7) * assign42610_e48124) + (assign42610_e48119 * (locals.var_xedgebd_dn7 - locals.var_xeffd_dn7))) / (2.0 * assign42610_e48128)))), (0.5 * ((locals.var_xedgebd_dn8 - locals.var_xeffd_dn8) + ((((locals.var_xedgebd_dn8 - locals.var_xeffd_dn8) * assign42610_e48124) + (assign42610_e48119 * (locals.var_xedgebd_dn8 - locals.var_xeffd_dn8))) / (2.0 * assign42610_e48128)))), (0.5 * ((locals.var_xedgebd_dn9 - locals.var_xeffd_dn9) + ((((locals.var_xedgebd_dn9 - locals.var_xeffd_dn9) * assign42610_e48124) + (assign42610_e48119 * (locals.var_xedgebd_dn9 - locals.var_xeffd_dn9))) / (2.0 * assign42610_e48128)))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign42610_e48132;
        locals.var_temp_dn4 = assign42610_e48132_d_n4;
        locals.var_temp_dn6 = assign42610_e48132_d_n6;
        locals.var_temp_dn7 = assign42610_e48132_d_n7;
        locals.var_temp_dn8 = assign42610_e48132_d_n8;
        locals.var_temp_dn9 = assign42610_e48132_d_n9;

        let (assign42620_e48144, assign42620_e48144_d_n4, assign42620_e48144_d_n6, assign42620_e48144_d_n7, assign42620_e48144_d_n8, assign42620_e48144_d_n9,) = {
    if (locals.var_guard1235 != 0.0) {
        let assign42620_e48136: f64 = (locals.var_temp2 * locals.var_temp);
        let assign42620_e48138: f64 = (assign42620_e48136 * locals.var_temp);
        let assign42620_e48141: f64 = (locals.var_xedgebd - locals.var_xstard);
        let assign42620_e48142: f64 = (assign42620_e48138 / assign42620_e48141);
        (assign42620_e48142, (((((((locals.var_temp2_dn4 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn4)) * locals.var_temp) + (assign42620_e48136 * locals.var_temp_dn4)) * assign42620_e48141) - (assign42620_e48138 * (locals.var_xedgebd_dn4 - locals.var_xstard_dn4))) / (assign42620_e48141 * assign42620_e48141)), (((((((locals.var_temp2_dn6 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn6)) * locals.var_temp) + (assign42620_e48136 * locals.var_temp_dn6)) * assign42620_e48141) - (assign42620_e48138 * (locals.var_xedgebd_dn6 - locals.var_xstard_dn6))) / (assign42620_e48141 * assign42620_e48141)), (((((((locals.var_temp2_dn7 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn7)) * locals.var_temp) + (assign42620_e48136 * locals.var_temp_dn7)) * assign42620_e48141) - (assign42620_e48138 * (locals.var_xedgebd_dn7 - locals.var_xstard_dn7))) / (assign42620_e48141 * assign42620_e48141)), (((((((locals.var_temp2_dn8 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn8)) * locals.var_temp) + (assign42620_e48136 * locals.var_temp_dn8)) * assign42620_e48141) - (assign42620_e48138 * (locals.var_xedgebd_dn8 - locals.var_xstard_dn8))) / (assign42620_e48141 * assign42620_e48141)), (((((((locals.var_temp2_dn9 * locals.var_temp) + (locals.var_temp2 * locals.var_temp_dn9)) * locals.var_temp) + (assign42620_e48136 * locals.var_temp_dn9)) * assign42620_e48141) - (assign42620_e48138 * (locals.var_xedgebd_dn9 - locals.var_xstard_dn9))) / (assign42620_e48141 * assign42620_e48141)),)
    } else {
        (locals.var_qbdif, locals.var_qbdif_dn4, locals.var_qbdif_dn6, locals.var_qbdif_dn7, locals.var_qbdif_dn8, locals.var_qbdif_dn9,)
    }
};
        locals.var_qbdif = assign42620_e48144;
        locals.var_qbdif_dn4 = assign42620_e48144_d_n4;
        locals.var_qbdif_dn6 = assign42620_e48144_d_n6;
        locals.var_qbdif_dn7 = assign42620_e48144_d_n7;
        locals.var_qbdif_dn8 = assign42620_e48144_d_n8;
        locals.var_qbdif_dn9 = assign42620_e48144_d_n9;

        let (assign42630_e48149, assign42630_e48149_d_n4, assign42630_e48149_d_n6, assign42630_e48149_d_n7, assign42630_e48149_d_n8, assign42630_e48149_d_n9,) = {
    if (locals.var_guard1235 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qgsif, locals.var_qgsif_dn4, locals.var_qgsif_dn6, locals.var_qgsif_dn7, locals.var_qgsif_dn8, locals.var_qgsif_dn9,)
    }
};
        locals.var_qgsif = assign42630_e48149;
        locals.var_qgsif_dn4 = assign42630_e48149_d_n4;
        locals.var_qgsif_dn6 = assign42630_e48149_d_n6;
        locals.var_qgsif_dn7 = assign42630_e48149_d_n7;
        locals.var_qgsif_dn8 = assign42630_e48149_d_n8;
        locals.var_qgsif_dn9 = assign42630_e48149_d_n9;

        let (assign42640_e48154, assign42640_e48154_d_n4, assign42640_e48154_d_n6, assign42640_e48154_d_n7, assign42640_e48154_d_n8, assign42640_e48154_d_n9,) = {
    if (locals.var_guard1235 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qgdif, locals.var_qgdif_dn4, locals.var_qgdif_dn6, locals.var_qgdif_dn7, locals.var_qgdif_dn8, locals.var_qgdif_dn9,)
    }
};
        locals.var_qgdif = assign42640_e48154;
        locals.var_qgdif_dn4 = assign42640_e48154_d_n4;
        locals.var_qgdif_dn6 = assign42640_e48154_d_n6;
        locals.var_qgdif_dn7 = assign42640_e48154_d_n7;
        locals.var_qgdif_dn8 = assign42640_e48154_d_n8;
        locals.var_qgdif_dn9 = assign42640_e48154_d_n9;

        let (assign42650_e48159, assign42650_e48159_d_n4, assign42650_e48159_d_n6, assign42650_e48159_d_n7, assign42650_e48159_d_n8, assign42650_e48159_d_n9,) = {
    if (locals.var_guard1235 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbsif, locals.var_qbsif_dn4, locals.var_qbsif_dn6, locals.var_qbsif_dn7, locals.var_qbsif_dn8, locals.var_qbsif_dn9,)
    }
};
        locals.var_qbsif = assign42650_e48159;
        locals.var_qbsif_dn4 = assign42650_e48159_d_n4;
        locals.var_qbsif_dn6 = assign42650_e48159_d_n6;
        locals.var_qbsif_dn7 = assign42650_e48159_d_n7;
        locals.var_qbsif_dn8 = assign42650_e48159_d_n8;
        locals.var_qbsif_dn9 = assign42650_e48159_d_n9;

        let (assign42660_e48164, assign42660_e48164_d_n4, assign42660_e48164_d_n6, assign42660_e48164_d_n7, assign42660_e48164_d_n8, assign42660_e48164_d_n9,) = {
    if (locals.var_guard1235 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbdif, locals.var_qbdif_dn4, locals.var_qbdif_dn6, locals.var_qbdif_dn7, locals.var_qbdif_dn8, locals.var_qbdif_dn9,)
    }
};
        locals.var_qbdif = assign42660_e48164;
        locals.var_qbdif_dn4 = assign42660_e48164_d_n4;
        locals.var_qbdif_dn6 = assign42660_e48164_d_n6;
        locals.var_qbdif_dn7 = assign42660_e48164_d_n7;
        locals.var_qbdif_dn8 = assign42660_e48164_d_n8;
        locals.var_qbdif_dn9 = assign42660_e48164_d_n9;

        let assign42670_e48167: f64 = (locals.var_cfr_i * locals.var_vgsu);
        locals.var_qgse = assign42670_e48167;
        locals.var_qgse_dn4 = (locals.var_cfr_i_dn4 * locals.var_vgsu);
        locals.var_qgse_dn6 = ((locals.var_cfr_i_dn6 * locals.var_vgsu) + (locals.var_cfr_i * locals.var_vgsu_dn6));
        locals.var_qgse_dn7 = (locals.var_cfr_i_dn7 * locals.var_vgsu);
        locals.var_qgse_dn8 = (locals.var_cfr_i_dn8 * locals.var_vgsu);
        locals.var_qgse_dn9 = ((locals.var_cfr_i_dn9 * locals.var_vgsu) + (locals.var_cfr_i * locals.var_vgsu_dn9));

        let assign42680_e48170: f64 = (locals.var_cfrd_i * locals.var_vgdu);
        locals.var_qgde = assign42680_e48170;
        locals.var_qgde_dn4 = (locals.var_cfrd_i_dn4 * locals.var_vgdu);
        locals.var_qgde_dn6 = ((locals.var_cfrd_i_dn6 * locals.var_vgdu) + (locals.var_cfrd_i * locals.var_vgdu_dn6));
        locals.var_qgde_dn7 = ((locals.var_cfrd_i_dn7 * locals.var_vgdu) + (locals.var_cfrd_i * locals.var_vgdu_dn7));
        locals.var_qgde_dn8 = (locals.var_cfrd_i_dn8 * locals.var_vgdu);
        locals.var_qgde_dn9 = ((locals.var_cfrd_i_dn9 * locals.var_vgdu) + (locals.var_cfrd_i * locals.var_vgdu_dn9));

        let assign42690_e48175: f64 = (locals.var_covdl_i * locals.var_dleff_ac);
        let assign42690_e48179: f64 = (locals.var_covdlb_i * locals.var_xg20shift_ac);
        let assign42690_e48180: f64 = (1.0 - assign42690_e48179);
        let assign42690_e48181: f64 = (assign42690_e48175 * assign42690_e48180);
        let assign42690_e48182: f64 = (1.0 - assign42690_e48181);
        let assign42690_e48184: f64 = assign42690_e48182;
        let assign42690_e48188: f64 = (locals.var_covdl_i * locals.var_dleff_ac);
        let assign42690_e48192: f64 = (locals.var_covdlb_i * locals.var_xg20shift_ac);
        let assign42690_e48193: f64 = (1.0 - assign42690_e48192);
        let assign42690_e48194: f64 = (assign42690_e48188 * assign42690_e48193);
        let assign42690_e48195: f64 = (1.0 - assign42690_e48194);
        let assign42690_e48197: f64 = assign42690_e48195;
        let assign42690_e48201: f64 = (locals.var_covdl_i * locals.var_dleff_ac);
        let assign42690_e48205: f64 = (locals.var_covdlb_i * locals.var_xg20shift_ac);
        let assign42690_e48206: f64 = (1.0 - assign42690_e48205);
        let assign42690_e48207: f64 = (assign42690_e48201 * assign42690_e48206);
        let assign42690_e48208: f64 = (1.0 - assign42690_e48207);
        let assign42690_e48210: f64 = assign42690_e48208;
        let assign42690_e48211: f64 = (assign42690_e48197 * assign42690_e48210);
        let assign42690_e48213: f64 = (assign42690_e48211 + 0.2);
        let assign42690_e48214: f64 = (assign42690_e48213).sqrt();
        let assign42690_e48215: f64 = (assign42690_e48184 + assign42690_e48214);
        let assign42690_e48216: f64 = (0.5 * assign42690_e48215);
        locals.var_temp = assign42690_e48216;
        locals.var_temp_dn4 = (0.5 * ((-(((locals.var_covdl_i * locals.var_dleff_ac_dn4) * assign42690_e48180) + (assign42690_e48175 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn4))))) + ((((-(((locals.var_covdl_i * locals.var_dleff_ac_dn4) * assign42690_e48193) + (assign42690_e48188 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn4))))) * assign42690_e48210) + (assign42690_e48197 * (-(((locals.var_covdl_i * locals.var_dleff_ac_dn4) * assign42690_e48206) + (assign42690_e48201 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn4))))))) / (2.0 * assign42690_e48214))));
        locals.var_temp_dn6 = (0.5 * ((-(((locals.var_covdl_i * locals.var_dleff_ac_dn6) * assign42690_e48180) + (assign42690_e48175 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn6))))) + ((((-(((locals.var_covdl_i * locals.var_dleff_ac_dn6) * assign42690_e48193) + (assign42690_e48188 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn6))))) * assign42690_e48210) + (assign42690_e48197 * (-(((locals.var_covdl_i * locals.var_dleff_ac_dn6) * assign42690_e48206) + (assign42690_e48201 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn6))))))) / (2.0 * assign42690_e48214))));
        locals.var_temp_dn7 = (0.5 * ((-(((locals.var_covdl_i * locals.var_dleff_ac_dn7) * assign42690_e48180) + (assign42690_e48175 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn7))))) + ((((-(((locals.var_covdl_i * locals.var_dleff_ac_dn7) * assign42690_e48193) + (assign42690_e48188 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn7))))) * assign42690_e48210) + (assign42690_e48197 * (-(((locals.var_covdl_i * locals.var_dleff_ac_dn7) * assign42690_e48206) + (assign42690_e48201 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn7))))))) / (2.0 * assign42690_e48214))));
        locals.var_temp_dn8 = (0.5 * ((-(((locals.var_covdl_i * locals.var_dleff_ac_dn8) * assign42690_e48180) + (assign42690_e48175 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn8))))) + ((((-(((locals.var_covdl_i * locals.var_dleff_ac_dn8) * assign42690_e48193) + (assign42690_e48188 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn8))))) * assign42690_e48210) + (assign42690_e48197 * (-(((locals.var_covdl_i * locals.var_dleff_ac_dn8) * assign42690_e48206) + (assign42690_e48201 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn8))))))) / (2.0 * assign42690_e48214))));
        locals.var_temp_dn9 = (0.5 * ((-(((locals.var_covdl_i * locals.var_dleff_ac_dn9) * assign42690_e48180) + (assign42690_e48175 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn9))))) + ((((-(((locals.var_covdl_i * locals.var_dleff_ac_dn9) * assign42690_e48193) + (assign42690_e48188 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn9))))) * assign42690_e48210) + (assign42690_e48197 * (-(((locals.var_covdl_i * locals.var_dleff_ac_dn9) * assign42690_e48206) + (assign42690_e48201 * (-(locals.var_covdlb_i * locals.var_xg20shift_ac_dn9))))))) / (2.0 * assign42690_e48214))));

        let assign42700_e48219: f64 = (locals.var_cov_i * locals.var_vovscv);
        let assign42700_e48221: f64 = (assign42700_e48219 * locals.var_temp);
        locals.var_qovs = assign42700_e48221;
        locals.var_qovs_dn4 = ((((locals.var_cov_i_dn4 * locals.var_vovscv) + (locals.var_cov_i * locals.var_vovscv_dn4)) * locals.var_temp) + (assign42700_e48219 * locals.var_temp_dn4));
        locals.var_qovs_dn6 = ((((locals.var_cov_i_dn6 * locals.var_vovscv) + (locals.var_cov_i * locals.var_vovscv_dn6)) * locals.var_temp) + (assign42700_e48219 * locals.var_temp_dn6));
        locals.var_qovs_dn7 = ((((locals.var_cov_i_dn7 * locals.var_vovscv) + (locals.var_cov_i * locals.var_vovscv_dn7)) * locals.var_temp) + (assign42700_e48219 * locals.var_temp_dn7));
        locals.var_qovs_dn8 = ((((locals.var_cov_i_dn8 * locals.var_vovscv) + (locals.var_cov_i * locals.var_vovscv_dn8)) * locals.var_temp) + (assign42700_e48219 * locals.var_temp_dn8));
        locals.var_qovs_dn9 = ((((locals.var_cov_i_dn9 * locals.var_vovscv) + (locals.var_cov_i * locals.var_vovscv_dn9)) * locals.var_temp) + (assign42700_e48219 * locals.var_temp_dn9));

        let assign42710_e48224: f64 = (locals.var_covd_i * locals.var_vovdcv);
        let assign42710_e48226: f64 = (assign42710_e48224 * locals.var_temp);
        locals.var_qovd = assign42710_e48226;
        locals.var_qovd_dn4 = ((((locals.var_covd_i_dn4 * locals.var_vovdcv) + (locals.var_covd_i * locals.var_vovdcv_dn4)) * locals.var_temp) + (assign42710_e48224 * locals.var_temp_dn4));
        locals.var_qovd_dn6 = ((((locals.var_covd_i_dn6 * locals.var_vovdcv) + (locals.var_covd_i * locals.var_vovdcv_dn6)) * locals.var_temp) + (assign42710_e48224 * locals.var_temp_dn6));
        locals.var_qovd_dn7 = ((((locals.var_covd_i_dn7 * locals.var_vovdcv) + (locals.var_covd_i * locals.var_vovdcv_dn7)) * locals.var_temp) + (assign42710_e48224 * locals.var_temp_dn7));
        locals.var_qovd_dn8 = ((((locals.var_covd_i_dn8 * locals.var_vovdcv) + (locals.var_covd_i * locals.var_vovdcv_dn8)) * locals.var_temp) + (assign42710_e48224 * locals.var_temp_dn8));
        locals.var_qovd_dn9 = ((((locals.var_covd_i_dn9 * locals.var_vovdcv) + (locals.var_covd_i * locals.var_vovdcv_dn9)) * locals.var_temp) + (assign42710_e48224 * locals.var_temp_dn9));

        let assign42720_e48229: f64 = (locals.var_cgbov_i * locals.var_vgb);
        locals.var_qgbe = assign42720_e48229;
        locals.var_qgbe_dn4 = (locals.var_cgbov_i_dn4 * locals.var_vgb);
        locals.var_qgbe_dn6 = ((locals.var_cgbov_i_dn6 * locals.var_vgb) + (locals.var_cgbov_i * locals.var_vgb_dn6));
        locals.var_qgbe_dn7 = ((locals.var_cgbov_i_dn7 * locals.var_vgb) + (locals.var_cgbov_i * locals.var_vgb_dn7));
        locals.var_qgbe_dn8 = ((locals.var_cgbov_i_dn8 * locals.var_vgb) + (locals.var_cgbov_i * locals.var_vgb_dn8));
        locals.var_qgbe_dn9 = ((locals.var_cgbov_i_dn9 * locals.var_vgb) + (locals.var_cgbov_i * locals.var_vgb_dn9));

        let assign42730_e48232: f64 = (locals.var_csd_i * locals.var_vds);
        locals.var_qdse = assign42730_e48232;
        locals.var_qdse_dn6 = (locals.var_csd_i * locals.var_vds_dn6);
        locals.var_qdse_dn7 = (locals.var_csd_i * locals.var_vds_dn7);

        let assign42740_e48235: f64 = (locals.var_cox2init * locals.var_asource_i);
        let assign42740_e48238: f64 = (locals.var_csdbp_i * locals.var_psource_i);
        let assign42740_e48239: f64 = (assign42740_e48235 + assign42740_e48238);
        let assign42740_e48240: f64 = (-assign42740_e48239);
        let assign42740_e48242: f64 = (assign42740_e48240 * locals.var_vsbu);
        locals.var_qssub = assign42740_e48242;
        locals.var_qssub_dn6 = (assign42740_e48240 * locals.var_vsbu_dn6);
        locals.var_qssub_dn8 = (assign42740_e48240 * locals.var_vsbu_dn8);

        let assign42750_e48245: f64 = (locals.var_cox2init * locals.var_adrain_i);
        let assign42750_e48248: f64 = (locals.var_csdbp_i * locals.var_pdrain_i);
        let assign42750_e48249: f64 = (assign42750_e48245 + assign42750_e48248);
        let assign42750_e48250: f64 = (-assign42750_e48249);
        let assign42750_e48252: f64 = (assign42750_e48250 * locals.var_vdbu);
        locals.var_qdsub = assign42750_e48252;
        locals.var_qdsub_dn6 = (assign42750_e48250 * locals.var_vdbu_dn6);
        locals.var_qdsub_dn7 = (assign42750_e48250 * locals.var_vdbu_dn7);
        locals.var_qdsub_dn8 = (assign42750_e48250 * locals.var_vdbu_dn8);

        let assign42790_e48269: f64 = (p.p31 * locals.var_mult_i_int);
        let assign42790_e48272: f64 = (locals.var_ids + locals.var_ids_edge);
        let assign42790_e48274: f64 = (assign42790_e48272 + locals.var_iimpact);
        let assign42790_e48275: f64 = (assign42790_e48269 * assign42790_e48274);
        locals.var_idse = assign42790_e48275;
        locals.var_idse_dn4 = (assign42790_e48269 * ((locals.var_ids_dn4 + locals.var_ids_edge_dn4) + locals.var_iimpact_dn4));
        locals.var_idse_dn6 = (assign42790_e48269 * ((locals.var_ids_dn6 + locals.var_ids_edge_dn6) + locals.var_iimpact_dn6));
        locals.var_idse_dn7 = (assign42790_e48269 * ((locals.var_ids_dn7 + locals.var_ids_edge_dn7) + locals.var_iimpact_dn7));
        locals.var_idse_dn8 = (assign42790_e48269 * ((locals.var_ids_dn8 + locals.var_ids_edge_dn8) + locals.var_iimpact_dn8));
        locals.var_idse_dn9 = (assign42790_e48269 * ((locals.var_ids_dn9 + locals.var_ids_edge_dn9) + locals.var_iimpact_dn9));

        let assign42840_e48298: f64 = (locals.var_mult_i_int * locals.var_ithpwr);
        locals.var_ithpwre = assign42840_e48298;
        locals.var_ithpwre_dn4 = (locals.var_mult_i_int * locals.var_ithpwr_dn4);
        locals.var_ithpwre_dn6 = (locals.var_mult_i_int * locals.var_ithpwr_dn6);
        locals.var_ithpwre_dn7 = (locals.var_mult_i_int * locals.var_ithpwr_dn7);
        locals.var_ithpwre_dn8 = (locals.var_mult_i_int * locals.var_ithpwr_dn8);
        locals.var_ithpwre_dn9 = (locals.var_mult_i_int * locals.var_ithpwr_dn9);

        let assign42860_e48304: f64 = if locals.var_sigvds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1239 = assign42860_e48304;

        let assign42910_e48319: f64 = (p.p32 * locals.var_mult_i_int);
        let assign42910_e48321: f64 = (assign42910_e48319 * locals.var_qg);
        locals.var_qg = assign42910_e48321;
        locals.var_qg_dn4 = (assign42910_e48319 * locals.var_qg_dn4);
        locals.var_qg_dn6 = (assign42910_e48319 * locals.var_qg_dn6);
        locals.var_qg_dn7 = (assign42910_e48319 * locals.var_qg_dn7);
        locals.var_qg_dn8 = (assign42910_e48319 * locals.var_qg_dn8);
        locals.var_qg_dn9 = (assign42910_e48319 * locals.var_qg_dn9);

        let assign42920_e48324: f64 = (p.p32 * locals.var_mult_i_int);
        let assign42920_e48326: f64 = (assign42920_e48324 * locals.var_qb);
        locals.var_qb = assign42920_e48326;
        locals.var_qb_dn4 = (assign42920_e48324 * locals.var_qb_dn4);
        locals.var_qb_dn6 = (assign42920_e48324 * locals.var_qb_dn6);
        locals.var_qb_dn7 = (assign42920_e48324 * locals.var_qb_dn7);
        locals.var_qb_dn8 = (assign42920_e48324 * locals.var_qb_dn8);
        locals.var_qb_dn9 = (assign42920_e48324 * locals.var_qb_dn9);

        let assign42930_e48329: f64 = (p.p32 * locals.var_mult_i_int);
        let assign42930_e48331: f64 = (assign42930_e48329 * locals.var_qd);
        locals.var_qd = assign42930_e48331;
        locals.var_qd_dn4 = (assign42930_e48329 * locals.var_qd_dn4);
        locals.var_qd_dn6 = (assign42930_e48329 * locals.var_qd_dn6);
        locals.var_qd_dn7 = (assign42930_e48329 * locals.var_qd_dn7);
        locals.var_qd_dn8 = (assign42930_e48329 * locals.var_qd_dn8);
        locals.var_qd_dn9 = (assign42930_e48329 * locals.var_qd_dn9);

        let assign42940_e48334: f64 = (locals.var_qg + locals.var_qb);
        let assign42940_e48336: f64 = (assign42940_e48334 + locals.var_qd);
        let assign42940_e48337: f64 = (-assign42940_e48336);
        locals.var_qs = assign42940_e48337;
        locals.var_qs_dn4 = (-((locals.var_qg_dn4 + locals.var_qb_dn4) + locals.var_qd_dn4));
        locals.var_qs_dn6 = (-((locals.var_qg_dn6 + locals.var_qb_dn6) + locals.var_qd_dn6));
        locals.var_qs_dn7 = (-((locals.var_qg_dn7 + locals.var_qb_dn7) + locals.var_qd_dn7));
        locals.var_qs_dn8 = (-((locals.var_qg_dn8 + locals.var_qb_dn8) + locals.var_qd_dn8));
        locals.var_qs_dn9 = (-((locals.var_qg_dn9 + locals.var_qb_dn9) + locals.var_qd_dn9));

        let assign42950_e48340: f64 = (p.p32 * locals.var_mult_i_int);
        let assign42950_e48342: f64 = (assign42950_e48340 * locals.var_qgsif);
        locals.var_qgsif = assign42950_e48342;
        locals.var_qgsif_dn4 = (assign42950_e48340 * locals.var_qgsif_dn4);
        locals.var_qgsif_dn6 = (assign42950_e48340 * locals.var_qgsif_dn6);
        locals.var_qgsif_dn7 = (assign42950_e48340 * locals.var_qgsif_dn7);
        locals.var_qgsif_dn8 = (assign42950_e48340 * locals.var_qgsif_dn8);
        locals.var_qgsif_dn9 = (assign42950_e48340 * locals.var_qgsif_dn9);

        let assign42960_e48345: f64 = (p.p32 * locals.var_mult_i_int);
        let assign42960_e48347: f64 = (assign42960_e48345 * locals.var_qgdif);
        locals.var_qgdif = assign42960_e48347;
        locals.var_qgdif_dn4 = (assign42960_e48345 * locals.var_qgdif_dn4);
        locals.var_qgdif_dn6 = (assign42960_e48345 * locals.var_qgdif_dn6);
        locals.var_qgdif_dn7 = (assign42960_e48345 * locals.var_qgdif_dn7);
        locals.var_qgdif_dn8 = (assign42960_e48345 * locals.var_qgdif_dn8);
        locals.var_qgdif_dn9 = (assign42960_e48345 * locals.var_qgdif_dn9);

        let assign42970_e48350: f64 = (p.p32 * locals.var_mult_i_int);
        let assign42970_e48352: f64 = (assign42970_e48350 * locals.var_qbsif);
        locals.var_qbsif = assign42970_e48352;
        locals.var_qbsif_dn4 = (assign42970_e48350 * locals.var_qbsif_dn4);
        locals.var_qbsif_dn6 = (assign42970_e48350 * locals.var_qbsif_dn6);
        locals.var_qbsif_dn7 = (assign42970_e48350 * locals.var_qbsif_dn7);
        locals.var_qbsif_dn8 = (assign42970_e48350 * locals.var_qbsif_dn8);
        locals.var_qbsif_dn9 = (assign42970_e48350 * locals.var_qbsif_dn9);

        let assign42980_e48355: f64 = (p.p32 * locals.var_mult_i_int);
        let assign42980_e48357: f64 = (assign42980_e48355 * locals.var_qbdif);
        locals.var_qbdif = assign42980_e48357;
        locals.var_qbdif_dn4 = (assign42980_e48355 * locals.var_qbdif_dn4);
        locals.var_qbdif_dn6 = (assign42980_e48355 * locals.var_qbdif_dn6);
        locals.var_qbdif_dn7 = (assign42980_e48355 * locals.var_qbdif_dn7);
        locals.var_qbdif_dn8 = (assign42980_e48355 * locals.var_qbdif_dn8);
        locals.var_qbdif_dn9 = (assign42980_e48355 * locals.var_qbdif_dn9);

        let assign42990_e48360: f64 = (p.p32 * locals.var_mult_i_int);
        let assign42990_e48362: f64 = (assign42990_e48360 * locals.var_qgse);
        locals.var_qgse = assign42990_e48362;
        locals.var_qgse_dn4 = (assign42990_e48360 * locals.var_qgse_dn4);
        locals.var_qgse_dn6 = (assign42990_e48360 * locals.var_qgse_dn6);
        locals.var_qgse_dn7 = (assign42990_e48360 * locals.var_qgse_dn7);
        locals.var_qgse_dn8 = (assign42990_e48360 * locals.var_qgse_dn8);
        locals.var_qgse_dn9 = (assign42990_e48360 * locals.var_qgse_dn9);

        let assign43000_e48365: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43000_e48367: f64 = (assign43000_e48365 * locals.var_qgde);
        locals.var_qgde = assign43000_e48367;
        locals.var_qgde_dn4 = (assign43000_e48365 * locals.var_qgde_dn4);
        locals.var_qgde_dn6 = (assign43000_e48365 * locals.var_qgde_dn6);
        locals.var_qgde_dn7 = (assign43000_e48365 * locals.var_qgde_dn7);
        locals.var_qgde_dn8 = (assign43000_e48365 * locals.var_qgde_dn8);
        locals.var_qgde_dn9 = (assign43000_e48365 * locals.var_qgde_dn9);

        let assign43010_e48370: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43010_e48372: f64 = (assign43010_e48370 * locals.var_qovs);
        locals.var_qovs = assign43010_e48372;
        locals.var_qovs_dn4 = (assign43010_e48370 * locals.var_qovs_dn4);
        locals.var_qovs_dn6 = (assign43010_e48370 * locals.var_qovs_dn6);
        locals.var_qovs_dn7 = (assign43010_e48370 * locals.var_qovs_dn7);
        locals.var_qovs_dn8 = (assign43010_e48370 * locals.var_qovs_dn8);
        locals.var_qovs_dn9 = (assign43010_e48370 * locals.var_qovs_dn9);

        let assign43020_e48375: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43020_e48377: f64 = (assign43020_e48375 * locals.var_qovd);
        locals.var_qovd = assign43020_e48377;
        locals.var_qovd_dn4 = (assign43020_e48375 * locals.var_qovd_dn4);
        locals.var_qovd_dn6 = (assign43020_e48375 * locals.var_qovd_dn6);
        locals.var_qovd_dn7 = (assign43020_e48375 * locals.var_qovd_dn7);
        locals.var_qovd_dn8 = (assign43020_e48375 * locals.var_qovd_dn8);
        locals.var_qovd_dn9 = (assign43020_e48375 * locals.var_qovd_dn9);

        let assign43030_e48380: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43030_e48382: f64 = (assign43030_e48380 * locals.var_qgbe);
        locals.var_qgbe = assign43030_e48382;
        locals.var_qgbe_dn4 = (assign43030_e48380 * locals.var_qgbe_dn4);
        locals.var_qgbe_dn6 = (assign43030_e48380 * locals.var_qgbe_dn6);
        locals.var_qgbe_dn7 = (assign43030_e48380 * locals.var_qgbe_dn7);
        locals.var_qgbe_dn8 = (assign43030_e48380 * locals.var_qgbe_dn8);
        locals.var_qgbe_dn9 = (assign43030_e48380 * locals.var_qgbe_dn9);

        let assign43040_e48385: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43040_e48387: f64 = (assign43040_e48385 * locals.var_qssub);
        locals.var_qssub = assign43040_e48387;
        locals.var_qssub_dn6 = (assign43040_e48385 * locals.var_qssub_dn6);
        locals.var_qssub_dn8 = (assign43040_e48385 * locals.var_qssub_dn8);

        let assign43050_e48390: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43050_e48392: f64 = (assign43050_e48390 * locals.var_qdsub);
        locals.var_qdsub = assign43050_e48392;
        locals.var_qdsub_dn6 = (assign43050_e48390 * locals.var_qdsub_dn6);
        locals.var_qdsub_dn7 = (assign43050_e48390 * locals.var_qdsub_dn7);
        locals.var_qdsub_dn8 = (assign43050_e48390 * locals.var_qdsub_dn8);

        let assign43060_e48395: f64 = (p.p32 * locals.var_mult_i_int);
        let assign43060_e48397: f64 = (assign43060_e48395 * locals.var_qdse);
        locals.var_qdse = assign43060_e48397;
        locals.var_qdse_dn6 = (assign43060_e48395 * locals.var_qdse_dn6);
        locals.var_qdse_dn7 = (assign43060_e48395 * locals.var_qdse_dn7);

        let assign43080_e48403: f64 = if locals.var_sigvds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1245 = assign43080_e48403;

        let (assign43090_e48407, assign43090_e48407_d_n4, assign43090_e48407_d_n6, assign43090_e48407_d_n7, assign43090_e48407_d_n8, assign43090_e48407_d_n9,) = {
    if (locals.var_guard1245 != 0.0) {
        (locals.var_qd, locals.var_qd_dn4, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9,)
    } else {
        (locals.var_temp_q, locals.var_temp_q_dn4, locals.var_temp_q_dn6, locals.var_temp_q_dn7, locals.var_temp_q_dn8, locals.var_temp_q_dn9,)
    }
};
        locals.var_temp_q = assign43090_e48407;
        locals.var_temp_q_dn4 = assign43090_e48407_d_n4;
        locals.var_temp_q_dn6 = assign43090_e48407_d_n6;
        locals.var_temp_q_dn7 = assign43090_e48407_d_n7;
        locals.var_temp_q_dn8 = assign43090_e48407_d_n8;
        locals.var_temp_q_dn9 = assign43090_e48407_d_n9;

        let (assign43100_e48411, assign43100_e48411_d_n4, assign43100_e48411_d_n6, assign43100_e48411_d_n7, assign43100_e48411_d_n8, assign43100_e48411_d_n9,) = {
    if (locals.var_guard1245 != 0.0) {
        (locals.var_qs, locals.var_qs_dn4, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9,)
    } else {
        (locals.var_qd, locals.var_qd_dn4, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9,)
    }
};
        locals.var_qd = assign43100_e48411;
        locals.var_qd_dn4 = assign43100_e48411_d_n4;
        locals.var_qd_dn6 = assign43100_e48411_d_n6;
        locals.var_qd_dn7 = assign43100_e48411_d_n7;
        locals.var_qd_dn8 = assign43100_e48411_d_n8;
        locals.var_qd_dn9 = assign43100_e48411_d_n9;

        let (assign43110_e48415, assign43110_e48415_d_n4, assign43110_e48415_d_n6, assign43110_e48415_d_n7, assign43110_e48415_d_n8, assign43110_e48415_d_n9,) = {
    if (locals.var_guard1245 != 0.0) {
        (locals.var_temp_q, locals.var_temp_q_dn4, locals.var_temp_q_dn6, locals.var_temp_q_dn7, locals.var_temp_q_dn8, locals.var_temp_q_dn9,)
    } else {
        (locals.var_qs, locals.var_qs_dn4, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9,)
    }
};
        locals.var_qs = assign43110_e48415;
        locals.var_qs_dn4 = assign43110_e48415_d_n4;
        locals.var_qs_dn6 = assign43110_e48415_d_n6;
        locals.var_qs_dn7 = assign43110_e48415_d_n7;
        locals.var_qs_dn8 = assign43110_e48415_d_n8;
        locals.var_qs_dn9 = assign43110_e48415_d_n9;

        let (assign43120_e48420, assign43120_e48420_d_n6, assign43120_e48420_d_n7,) = {
    if (locals.var_guard1245 != 0.0) {
        let assign43120_e48418: f64 = (-locals.var_qdse);
        (assign43120_e48418, (-locals.var_qdse_dn6), (-locals.var_qdse_dn7),)
    } else {
        (locals.var_qdse, locals.var_qdse_dn6, locals.var_qdse_dn7,)
    }
};
        locals.var_qdse = assign43120_e48420;
        locals.var_qdse_dn6 = assign43120_e48420_d_n6;
        locals.var_qdse_dn7 = assign43120_e48420_d_n7;

        let (assign43130_e48424, assign43130_e48424_d_n4, assign43130_e48424_d_n6, assign43130_e48424_d_n7, assign43130_e48424_d_n8, assign43130_e48424_d_n9,) = {
    if (locals.var_guard1245 != 0.0) {
        (locals.var_qgdif, locals.var_qgdif_dn4, locals.var_qgdif_dn6, locals.var_qgdif_dn7, locals.var_qgdif_dn8, locals.var_qgdif_dn9,)
    } else {
        (locals.var_temp_q, locals.var_temp_q_dn4, locals.var_temp_q_dn6, locals.var_temp_q_dn7, locals.var_temp_q_dn8, locals.var_temp_q_dn9,)
    }
};
        locals.var_temp_q = assign43130_e48424;
        locals.var_temp_q_dn4 = assign43130_e48424_d_n4;
        locals.var_temp_q_dn6 = assign43130_e48424_d_n6;
        locals.var_temp_q_dn7 = assign43130_e48424_d_n7;
        locals.var_temp_q_dn8 = assign43130_e48424_d_n8;
        locals.var_temp_q_dn9 = assign43130_e48424_d_n9;

        let (assign43140_e48428, assign43140_e48428_d_n4, assign43140_e48428_d_n6, assign43140_e48428_d_n7, assign43140_e48428_d_n8, assign43140_e48428_d_n9,) = {
    if (locals.var_guard1245 != 0.0) {
        (locals.var_qgsif, locals.var_qgsif_dn4, locals.var_qgsif_dn6, locals.var_qgsif_dn7, locals.var_qgsif_dn8, locals.var_qgsif_dn9,)
    } else {
        (locals.var_qgdif, locals.var_qgdif_dn4, locals.var_qgdif_dn6, locals.var_qgdif_dn7, locals.var_qgdif_dn8, locals.var_qgdif_dn9,)
    }
};
        locals.var_qgdif = assign43140_e48428;
        locals.var_qgdif_dn4 = assign43140_e48428_d_n4;
        locals.var_qgdif_dn6 = assign43140_e48428_d_n6;
        locals.var_qgdif_dn7 = assign43140_e48428_d_n7;
        locals.var_qgdif_dn8 = assign43140_e48428_d_n8;
        locals.var_qgdif_dn9 = assign43140_e48428_d_n9;

        let (assign43150_e48432, assign43150_e48432_d_n4, assign43150_e48432_d_n6, assign43150_e48432_d_n7, assign43150_e48432_d_n8, assign43150_e48432_d_n9,) = {
    if (locals.var_guard1245 != 0.0) {
        (locals.var_temp_q, locals.var_temp_q_dn4, locals.var_temp_q_dn6, locals.var_temp_q_dn7, locals.var_temp_q_dn8, locals.var_temp_q_dn9,)
    } else {
        (locals.var_qgsif, locals.var_qgsif_dn4, locals.var_qgsif_dn6, locals.var_qgsif_dn7, locals.var_qgsif_dn8, locals.var_qgsif_dn9,)
    }
};
        locals.var_qgsif = assign43150_e48432;
        locals.var_qgsif_dn4 = assign43150_e48432_d_n4;
        locals.var_qgsif_dn6 = assign43150_e48432_d_n6;
        locals.var_qgsif_dn7 = assign43150_e48432_d_n7;
        locals.var_qgsif_dn8 = assign43150_e48432_d_n8;
        locals.var_qgsif_dn9 = assign43150_e48432_d_n9;

        let (assign43160_e48436, assign43160_e48436_d_n4, assign43160_e48436_d_n6, assign43160_e48436_d_n7, assign43160_e48436_d_n8, assign43160_e48436_d_n9,) = {
    if (locals.var_guard1245 != 0.0) {
        (locals.var_qbdif, locals.var_qbdif_dn4, locals.var_qbdif_dn6, locals.var_qbdif_dn7, locals.var_qbdif_dn8, locals.var_qbdif_dn9,)
    } else {
        (locals.var_temp_q, locals.var_temp_q_dn4, locals.var_temp_q_dn6, locals.var_temp_q_dn7, locals.var_temp_q_dn8, locals.var_temp_q_dn9,)
    }
};
        locals.var_temp_q = assign43160_e48436;
        locals.var_temp_q_dn4 = assign43160_e48436_d_n4;
        locals.var_temp_q_dn6 = assign43160_e48436_d_n6;
        locals.var_temp_q_dn7 = assign43160_e48436_d_n7;
        locals.var_temp_q_dn8 = assign43160_e48436_d_n8;
        locals.var_temp_q_dn9 = assign43160_e48436_d_n9;

        let (assign43170_e48440, assign43170_e48440_d_n4, assign43170_e48440_d_n6, assign43170_e48440_d_n7, assign43170_e48440_d_n8, assign43170_e48440_d_n9,) = {
    if (locals.var_guard1245 != 0.0) {
        (locals.var_qbsif, locals.var_qbsif_dn4, locals.var_qbsif_dn6, locals.var_qbsif_dn7, locals.var_qbsif_dn8, locals.var_qbsif_dn9,)
    } else {
        (locals.var_qbdif, locals.var_qbdif_dn4, locals.var_qbdif_dn6, locals.var_qbdif_dn7, locals.var_qbdif_dn8, locals.var_qbdif_dn9,)
    }
};
        locals.var_qbdif = assign43170_e48440;
        locals.var_qbdif_dn4 = assign43170_e48440_d_n4;
        locals.var_qbdif_dn6 = assign43170_e48440_d_n6;
        locals.var_qbdif_dn7 = assign43170_e48440_d_n7;
        locals.var_qbdif_dn8 = assign43170_e48440_d_n8;
        locals.var_qbdif_dn9 = assign43170_e48440_d_n9;

    }

    pub(super) fn stamp_transient_block_117(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign43180_e48444, assign43180_e48444_d_n4, assign43180_e48444_d_n6, assign43180_e48444_d_n7, assign43180_e48444_d_n8, assign43180_e48444_d_n9,) = {
    if (locals.var_guard1245 != 0.0) {
        (locals.var_temp_q, locals.var_temp_q_dn4, locals.var_temp_q_dn6, locals.var_temp_q_dn7, locals.var_temp_q_dn8, locals.var_temp_q_dn9,)
    } else {
        (locals.var_qbsif, locals.var_qbsif_dn4, locals.var_qbsif_dn6, locals.var_qbsif_dn7, locals.var_qbsif_dn8, locals.var_qbsif_dn9,)
    }
};
        locals.var_qbsif = assign43180_e48444;
        locals.var_qbsif_dn4 = assign43180_e48444_d_n4;
        locals.var_qbsif_dn6 = assign43180_e48444_d_n6;
        locals.var_qbsif_dn7 = assign43180_e48444_d_n7;
        locals.var_qbsif_dn8 = assign43180_e48444_d_n8;
        locals.var_qbsif_dn9 = assign43180_e48444_d_n9;

        let assign43190_e48447: f64 = (locals.var_csiprime_dc / 1.602176565e-19);
        let assign43190_e48449: f64 = (assign43190_e48447 * locals.var_phit);
        locals.var_nunit = assign43190_e48449;
        locals.var_nunit_dn4 = (((locals.var_csiprime_dc_dn4 / 1.602176565e-19) * locals.var_phit) + (assign43190_e48447 * locals.var_phit_dn4));
        locals.var_nunit_dn6 = (((locals.var_csiprime_dc_dn6 / 1.602176565e-19) * locals.var_phit) + (assign43190_e48447 * locals.var_phit_dn6));
        locals.var_nunit_dn7 = (((locals.var_csiprime_dc_dn7 / 1.602176565e-19) * locals.var_phit) + (assign43190_e48447 * locals.var_phit_dn7));
        locals.var_nunit_dn8 = (((locals.var_csiprime_dc_dn8 / 1.602176565e-19) * locals.var_phit) + (assign43190_e48447 * locals.var_phit_dn8));
        locals.var_nunit_dn9 = (((locals.var_csiprime_dc_dn9 / 1.602176565e-19) * locals.var_phit) + (assign43190_e48447 * locals.var_phit_dn9));

        let assign43200_e48451: f64 = (-0.5);
        let assign43200_e48454: f64 = (locals.var_ds_dc + locals.var_dd_dc);
        let assign43200_e48455: f64 = (assign43200_e48451 * assign43200_e48454);
        locals.var_dm = assign43200_e48455;
        locals.var_dm_dn4 = (assign43200_e48451 * (locals.var_ds_dc_dn4 + locals.var_dd_dc_dn4));
        locals.var_dm_dn6 = (assign43200_e48451 * (locals.var_ds_dc_dn6 + locals.var_dd_dc_dn6));
        locals.var_dm_dn7 = (assign43200_e48451 * (locals.var_ds_dc_dn7 + locals.var_dd_dc_dn7));
        locals.var_dm_dn8 = (assign43200_e48451 * (locals.var_ds_dc_dn8 + locals.var_dd_dc_dn8));
        locals.var_dm_dn9 = (assign43200_e48451 * (locals.var_ds_dc_dn9 + locals.var_dd_dc_dn9));

        let assign43210_e48458: f64 = (locals.var_qim_dc + locals.var_dm);
        locals.var_qimstar = assign43210_e48458;
        locals.var_qimstar_dn4 = (locals.var_qim_dc_dn4 + locals.var_dm_dn4);
        locals.var_qimstar_dn6 = (locals.var_qim_dc_dn6 + locals.var_dm_dn6);
        locals.var_qimstar_dn7 = (locals.var_qim_dc_dn7 + locals.var_dm_dn7);
        locals.var_qimstar_dn8 = (locals.var_qim_dc_dn8 + locals.var_dm_dn8);
        locals.var_qimstar_dn9 = (locals.var_qim_dc_dn9 + locals.var_dm_dn9);

        let assign43220_e48461: f64 = (locals.var_qim_dc / locals.var_qimstar);
        locals.var_temp = assign43220_e48461;
        locals.var_temp_dn4 = (((locals.var_qim_dc_dn4 * locals.var_qimstar) - (locals.var_qim_dc * locals.var_qimstar_dn4)) / (locals.var_qimstar * locals.var_qimstar));
        locals.var_temp_dn6 = (((locals.var_qim_dc_dn6 * locals.var_qimstar) - (locals.var_qim_dc * locals.var_qimstar_dn6)) / (locals.var_qimstar * locals.var_qimstar));
        locals.var_temp_dn7 = (((locals.var_qim_dc_dn7 * locals.var_qimstar) - (locals.var_qim_dc * locals.var_qimstar_dn7)) / (locals.var_qimstar * locals.var_qimstar));
        locals.var_temp_dn8 = (((locals.var_qim_dc_dn8 * locals.var_qimstar) - (locals.var_qim_dc * locals.var_qimstar_dn8)) / (locals.var_qimstar * locals.var_qimstar));
        locals.var_temp_dn9 = (((locals.var_qim_dc_dn9 * locals.var_qimstar) - (locals.var_qim_dc * locals.var_qimstar_dn9)) / (locals.var_qimstar * locals.var_qimstar));

        let assign43230_e48465: f64 = locals.var_temp;
        let assign43230_e48468: f64 = locals.var_temp;
        let assign43230_e48471: f64 = locals.var_temp;
        let assign43230_e48472: f64 = (assign43230_e48468 * assign43230_e48471);
        let assign43230_e48474: f64 = (assign43230_e48472 + 1e-20);
        let assign43230_e48475: f64 = (assign43230_e48474).sqrt();
        let assign43230_e48476: f64 = (assign43230_e48465 + assign43230_e48475);
        let assign43230_e48477: f64 = (0.5 * assign43230_e48476);
        locals.var_t1 = assign43230_e48477;
        locals.var_t1_dn4 = (0.5 * (locals.var_temp_dn4 + (((locals.var_temp_dn4 * assign43230_e48471) + (assign43230_e48468 * locals.var_temp_dn4)) / (2.0 * assign43230_e48475))));
        locals.var_t1_dn6 = (0.5 * (locals.var_temp_dn6 + (((locals.var_temp_dn6 * assign43230_e48471) + (assign43230_e48468 * locals.var_temp_dn6)) / (2.0 * assign43230_e48475))));
        locals.var_t1_dn7 = (0.5 * (locals.var_temp_dn7 + (((locals.var_temp_dn7 * assign43230_e48471) + (assign43230_e48468 * locals.var_temp_dn7)) / (2.0 * assign43230_e48475))));
        locals.var_t1_dn8 = (0.5 * (locals.var_temp_dn8 + (((locals.var_temp_dn8 * assign43230_e48471) + (assign43230_e48468 * locals.var_temp_dn8)) / (2.0 * assign43230_e48475))));
        locals.var_t1_dn9 = (0.5 * (locals.var_temp_dn9 + (((locals.var_temp_dn9 * assign43230_e48471) + (assign43230_e48468 * locals.var_temp_dn9)) / (2.0 * assign43230_e48475))));

        let assign43240_e48479: f64 = (-0.1666666666667);
        let assign43240_e48481: f64 = (assign43240_e48479 * locals.var_delta_k1q1_dc);
        let assign43240_e48483: f64 = (assign43240_e48481 * locals.var_inv_k1h1_0_dc);
        locals.var_sqrt_t2 = assign43240_e48483;
        locals.var_sqrt_t2_dn4 = (((assign43240_e48479 * locals.var_delta_k1q1_dc_dn4) * locals.var_inv_k1h1_0_dc) + (assign43240_e48481 * locals.var_inv_k1h1_0_dc_dn4));
        locals.var_sqrt_t2_dn6 = (((assign43240_e48479 * locals.var_delta_k1q1_dc_dn6) * locals.var_inv_k1h1_0_dc) + (assign43240_e48481 * locals.var_inv_k1h1_0_dc_dn6));
        locals.var_sqrt_t2_dn7 = (((assign43240_e48479 * locals.var_delta_k1q1_dc_dn7) * locals.var_inv_k1h1_0_dc) + (assign43240_e48481 * locals.var_inv_k1h1_0_dc_dn7));
        locals.var_sqrt_t2_dn8 = (((assign43240_e48479 * locals.var_delta_k1q1_dc_dn8) * locals.var_inv_k1h1_0_dc) + (assign43240_e48481 * locals.var_inv_k1h1_0_dc_dn8));
        locals.var_sqrt_t2_dn9 = (((assign43240_e48479 * locals.var_delta_k1q1_dc_dn9) * locals.var_inv_k1h1_0_dc) + (assign43240_e48481 * locals.var_inv_k1h1_0_dc_dn9));

        let assign43250_e48486: f64 = (locals.var_sqrt_t2 * locals.var_sqrt_t2);
        locals.var_t2 = assign43250_e48486;
        locals.var_t2_dn4 = ((locals.var_sqrt_t2_dn4 * locals.var_sqrt_t2) + (locals.var_sqrt_t2 * locals.var_sqrt_t2_dn4));
        locals.var_t2_dn6 = ((locals.var_sqrt_t2_dn6 * locals.var_sqrt_t2) + (locals.var_sqrt_t2 * locals.var_sqrt_t2_dn6));
        locals.var_t2_dn7 = ((locals.var_sqrt_t2_dn7 * locals.var_sqrt_t2) + (locals.var_sqrt_t2 * locals.var_sqrt_t2_dn7));
        locals.var_t2_dn8 = ((locals.var_sqrt_t2_dn8 * locals.var_sqrt_t2) + (locals.var_sqrt_t2 * locals.var_sqrt_t2_dn8));
        locals.var_t2_dn9 = ((locals.var_sqrt_t2_dn9 * locals.var_sqrt_t2) + (locals.var_sqrt_t2 * locals.var_sqrt_t2_dn9));

        let assign43260_e48489: f64 = (locals.var_hsat_dc - 1.0);
        locals.var_r = assign43260_e48489;
        locals.var_r_dn4 = locals.var_hsat_dc_dn4;
        locals.var_r_dn6 = locals.var_hsat_dc_dn6;
        locals.var_r_dn7 = locals.var_hsat_dc_dn7;
        locals.var_r_dn8 = locals.var_hsat_dc_dn8;
        locals.var_r_dn9 = locals.var_hsat_dc_dn9;

        let assign43270_e48493: f64 = (12.0 * locals.var_r);
        let assign43270_e48495: f64 = (assign43270_e48493 * locals.var_t2);
        let assign43270_e48496: f64 = (1.0 - assign43270_e48495);
        let assign43270_e48498: f64 = (assign43270_e48496).max(1e-20);
        locals.var_lc = assign43270_e48498;
        locals.var_lc_dn4 = if assign43270_e48496 >= 1e-20 { (-(((12.0 * locals.var_r_dn4) * locals.var_t2) + (assign43270_e48493 * locals.var_t2_dn4))) } else { 0.0 };
        locals.var_lc_dn6 = if assign43270_e48496 >= 1e-20 { (-(((12.0 * locals.var_r_dn6) * locals.var_t2) + (assign43270_e48493 * locals.var_t2_dn6))) } else { 0.0 };
        locals.var_lc_dn7 = if assign43270_e48496 >= 1e-20 { (-(((12.0 * locals.var_r_dn7) * locals.var_t2) + (assign43270_e48493 * locals.var_t2_dn7))) } else { 0.0 };
        locals.var_lc_dn8 = if assign43270_e48496 >= 1e-20 { (-(((12.0 * locals.var_r_dn8) * locals.var_t2) + (assign43270_e48493 * locals.var_t2_dn8))) } else { 0.0 };
        locals.var_lc_dn9 = if assign43270_e48496 >= 1e-20 { (-(((12.0 * locals.var_r_dn9) * locals.var_t2) + (assign43270_e48493 * locals.var_t2_dn9))) } else { 0.0 };

        let assign43280_e48502: f64 = (locals.var_lc * locals.var_lc);
        let assign43280_e48503: f64 = (1.0 / assign43280_e48502);
        locals.var_lcinv2 = assign43280_e48503;
        locals.var_lcinv2_dn4 = (-(((locals.var_lc_dn4 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn4)) / (assign43280_e48502 * assign43280_e48502)));
        locals.var_lcinv2_dn6 = (-(((locals.var_lc_dn6 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn6)) / (assign43280_e48502 * assign43280_e48502)));
        locals.var_lcinv2_dn7 = (-(((locals.var_lc_dn7 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn7)) / (assign43280_e48502 * assign43280_e48502)));
        locals.var_lcinv2_dn8 = (-(((locals.var_lc_dn8 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn8)) / (assign43280_e48502 * assign43280_e48502)));
        locals.var_lcinv2_dn9 = (-(((locals.var_lc_dn9 * locals.var_lc) + (locals.var_lc * locals.var_lc_dn9)) / (assign43280_e48502 * assign43280_e48502)));

        let assign43290_e48506: f64 = (locals.var_betneff * locals.var_csiprime_dc);
        let assign43290_e48508: f64 = (assign43290_e48506 * locals.var_phit);
        let assign43290_e48510: f64 = (assign43290_e48508 * locals.var_qimstar);
        let assign43290_e48512: f64 = (assign43290_e48510 * locals.var_fdl);
        let assign43290_e48514: f64 = (assign43290_e48512 / locals.var_gvsat);
        let assign43290_e48516: f64 = (assign43290_e48514 / locals.var_qmfact);
        locals.var_g_ideal = assign43290_e48516;
        locals.var_g_ideal_dn4 = ((((((((((((((locals.var_betneff_dn4 * locals.var_csiprime_dc) + (locals.var_betneff * locals.var_csiprime_dc_dn4)) * locals.var_phit) + (assign43290_e48506 * locals.var_phit_dn4)) * locals.var_qimstar) + (assign43290_e48508 * locals.var_qimstar_dn4)) * locals.var_fdl) + (assign43290_e48510 * locals.var_fdl_dn4)) * locals.var_gvsat) - (assign43290_e48512 * locals.var_gvsat_dn4)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_qmfact) - (assign43290_e48514 * locals.var_qmfact_dn4)) / (locals.var_qmfact * locals.var_qmfact));
        locals.var_g_ideal_dn6 = ((((((((((((((locals.var_betneff_dn6 * locals.var_csiprime_dc) + (locals.var_betneff * locals.var_csiprime_dc_dn6)) * locals.var_phit) + (assign43290_e48506 * locals.var_phit_dn6)) * locals.var_qimstar) + (assign43290_e48508 * locals.var_qimstar_dn6)) * locals.var_fdl) + (assign43290_e48510 * locals.var_fdl_dn6)) * locals.var_gvsat) - (assign43290_e48512 * locals.var_gvsat_dn6)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_qmfact) - (assign43290_e48514 * locals.var_qmfact_dn6)) / (locals.var_qmfact * locals.var_qmfact));
        locals.var_g_ideal_dn7 = ((((((((((((((locals.var_betneff_dn7 * locals.var_csiprime_dc) + (locals.var_betneff * locals.var_csiprime_dc_dn7)) * locals.var_phit) + (assign43290_e48506 * locals.var_phit_dn7)) * locals.var_qimstar) + (assign43290_e48508 * locals.var_qimstar_dn7)) * locals.var_fdl) + (assign43290_e48510 * locals.var_fdl_dn7)) * locals.var_gvsat) - (assign43290_e48512 * locals.var_gvsat_dn7)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_qmfact) - (assign43290_e48514 * locals.var_qmfact_dn7)) / (locals.var_qmfact * locals.var_qmfact));
        locals.var_g_ideal_dn8 = ((((((((((((((locals.var_betneff_dn8 * locals.var_csiprime_dc) + (locals.var_betneff * locals.var_csiprime_dc_dn8)) * locals.var_phit) + (assign43290_e48506 * locals.var_phit_dn8)) * locals.var_qimstar) + (assign43290_e48508 * locals.var_qimstar_dn8)) * locals.var_fdl) + (assign43290_e48510 * locals.var_fdl_dn8)) * locals.var_gvsat) - (assign43290_e48512 * locals.var_gvsat_dn8)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_qmfact) - (assign43290_e48514 * locals.var_qmfact_dn8)) / (locals.var_qmfact * locals.var_qmfact));
        locals.var_g_ideal_dn9 = ((((((((((((((locals.var_betneff_dn9 * locals.var_csiprime_dc) + (locals.var_betneff * locals.var_csiprime_dc_dn9)) * locals.var_phit) + (assign43290_e48506 * locals.var_phit_dn9)) * locals.var_qimstar) + (assign43290_e48508 * locals.var_qimstar_dn9)) * locals.var_fdl) + (assign43290_e48510 * locals.var_fdl_dn9)) * locals.var_gvsat) - (assign43290_e48512 * locals.var_gvsat_dn9)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_qmfact) - (assign43290_e48514 * locals.var_qmfact_dn9)) / (locals.var_qmfact * locals.var_qmfact));

        let assign43300_e48519: f64 = (12.0 * locals.var_t2);
        locals.var_t2x12 = assign43300_e48519;
        locals.var_t2x12_dn4 = (12.0 * locals.var_t2_dn4);
        locals.var_t2x12_dn6 = (12.0 * locals.var_t2_dn6);
        locals.var_t2x12_dn7 = (12.0 * locals.var_t2_dn7);
        locals.var_t2x12_dn8 = (12.0 * locals.var_t2_dn8);
        locals.var_t2x12_dn9 = (12.0 * locals.var_t2_dn9);

        let assign43310_e48522: f64 = (locals.var_t1 + locals.var_t2x12);
        let assign43310_e48526: f64 = (1.0 + locals.var_t1);
        let assign43310_e48527: f64 = (2.0 * assign43310_e48526);
        let assign43310_e48529: f64 = (assign43310_e48527 * locals.var_t2x12);
        let assign43310_e48531: f64 = (assign43310_e48529 * locals.var_r);
        let assign43310_e48532: f64 = (assign43310_e48522 - assign43310_e48531);
        locals.var_temp1 = assign43310_e48532;
        locals.var_temp1_dn4 = ((locals.var_t1_dn4 + locals.var_t2x12_dn4) - (((((2.0 * locals.var_t1_dn4) * locals.var_t2x12) + (assign43310_e48527 * locals.var_t2x12_dn4)) * locals.var_r) + (assign43310_e48529 * locals.var_r_dn4)));
        locals.var_temp1_dn6 = ((locals.var_t1_dn6 + locals.var_t2x12_dn6) - (((((2.0 * locals.var_t1_dn6) * locals.var_t2x12) + (assign43310_e48527 * locals.var_t2x12_dn6)) * locals.var_r) + (assign43310_e48529 * locals.var_r_dn6)));
        locals.var_temp1_dn7 = ((locals.var_t1_dn7 + locals.var_t2x12_dn7) - (((((2.0 * locals.var_t1_dn7) * locals.var_t2x12) + (assign43310_e48527 * locals.var_t2x12_dn7)) * locals.var_r) + (assign43310_e48529 * locals.var_r_dn7)));
        locals.var_temp1_dn8 = ((locals.var_t1_dn8 + locals.var_t2x12_dn8) - (((((2.0 * locals.var_t1_dn8) * locals.var_t2x12) + (assign43310_e48527 * locals.var_t2x12_dn8)) * locals.var_r) + (assign43310_e48529 * locals.var_r_dn8)));
        locals.var_temp1_dn9 = ((locals.var_t1_dn9 + locals.var_t2x12_dn9) - (((((2.0 * locals.var_t1_dn9) * locals.var_t2x12) + (assign43310_e48527 * locals.var_t2x12_dn9)) * locals.var_r) + (assign43310_e48529 * locals.var_r_dn9)));

        let assign43320_e48535: f64 = (locals.var_temp1).max(1e-40);
        locals.var_temp2 = assign43320_e48535;
        locals.var_temp2_dn4 = if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn4 } else { 0.0 };
        locals.var_temp2_dn6 = if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn6 } else { 0.0 };
        locals.var_temp2_dn7 = if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn7 } else { 0.0 };
        locals.var_temp2_dn8 = if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn8 } else { 0.0 };
        locals.var_temp2_dn9 = if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn9 } else { 0.0 };

        let assign43330_e48538: f64 = (locals.var_g_ideal * locals.var_lcinv2);
        let assign43330_e48540: f64 = (assign43330_e48538 * locals.var_temp2);
        locals.var_gsid = assign43330_e48540;

        let assign43340_e48543: f64 = if locals.var_fntexc_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1278 = assign43340_e48543;

        let (assign43350_e48549,) = {
    if (locals.var_guard1278 != 0.0) {
        let assign43350_e48547: f64 = (locals.var_ggamma_dc / locals.var_gmob_dc);
        (assign43350_e48547,)
    } else {
        (locals.var_sqrt_zsatexc,)
    }
};
        locals.var_sqrt_zsatexc = assign43350_e48549;

        let (assign43360_e48569,) = {
    if (locals.var_guard1278 != 0.0) {
        let assign43360_e48553: f64 = (locals.var_fac_exc * locals.var_ids);
        let assign43360_e48555: f64 = (assign43360_e48553 * locals.var_xdeff_dc);
        let assign43360_e48557: f64 = (assign43360_e48555 * locals.var_phit0);
        let assign43360_e48561: f64 = (locals.var_sqrt_zsatexc * locals.var_sqrt_zsatexc);
        let assign43360_e48562: f64 = (1.0 + assign43360_e48561);
        let assign43360_e48564: f64 = (assign43360_e48562 * locals.var_lc);
        let assign43360_e48566: f64 = (assign43360_e48564 * locals.var_lc);
        let assign43360_e48567: f64 = (assign43360_e48557 / assign43360_e48566);
        (assign43360_e48567,)
    } else {
        (locals.var_sidexc,)
    }
};
        locals.var_sidexc = assign43360_e48569;

        let (assign43370_e48577,) = {
    if (locals.var_guard1278 != 0.0) {
        let assign43370_e48574: f64 = (locals.var_sidexc / locals.var_nt0);
        let assign43370_e48575: f64 = (locals.var_gsid + assign43370_e48574);
        (assign43370_e48575,)
    } else {
        (locals.var_gsid,)
    }
};
        locals.var_gsid = assign43370_e48577;

        let assign43390_e48587: f64 = (locals.var_k1_ac * locals.var_csiprime_ac);
        let assign43390_e48589: f64 = (assign43390_e48587 * locals.var_areaq_i);
        let assign43390_e48591: f64 = (assign43390_e48589 / locals.var_qmfact1_ac);
        locals.var_cox_qm = assign43390_e48591;
        locals.var_cox_qm_dn4 = ((((((locals.var_k1_ac_dn4 * locals.var_csiprime_ac) + (locals.var_k1_ac * locals.var_csiprime_ac_dn4)) * locals.var_areaq_i) * locals.var_qmfact1_ac) - (assign43390_e48589 * locals.var_qmfact1_ac_dn4)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac));
        locals.var_cox_qm_dn6 = ((((((locals.var_k1_ac_dn6 * locals.var_csiprime_ac) + (locals.var_k1_ac * locals.var_csiprime_ac_dn6)) * locals.var_areaq_i) * locals.var_qmfact1_ac) - (assign43390_e48589 * locals.var_qmfact1_ac_dn6)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac));
        locals.var_cox_qm_dn7 = ((((((locals.var_k1_ac_dn7 * locals.var_csiprime_ac) + (locals.var_k1_ac * locals.var_csiprime_ac_dn7)) * locals.var_areaq_i) * locals.var_qmfact1_ac) - (assign43390_e48589 * locals.var_qmfact1_ac_dn7)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac));
        locals.var_cox_qm_dn8 = ((((((locals.var_k1_ac_dn8 * locals.var_csiprime_ac) + (locals.var_k1_ac * locals.var_csiprime_ac_dn8)) * locals.var_areaq_i) * locals.var_qmfact1_ac) - (assign43390_e48589 * locals.var_qmfact1_ac_dn8)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac));
        locals.var_cox_qm_dn9 = ((((((locals.var_k1_ac_dn9 * locals.var_csiprime_ac) + (locals.var_k1_ac * locals.var_csiprime_ac_dn9)) * locals.var_areaq_i) * locals.var_qmfact1_ac) - (assign43390_e48589 * locals.var_qmfact1_ac_dn9)) / (locals.var_qmfact1_ac * locals.var_qmfact1_ac));

        let assign43400_e48594: f64 = (1.0 + locals.var_zsat_ac);
        let assign43400_e48596: f64 = (assign43400_e48594 * locals.var_cox_qm);
        locals.var_cgeff = assign43400_e48596;
        locals.var_cgeff_dn4 = ((locals.var_zsat_ac_dn4 * locals.var_cox_qm) + (assign43400_e48594 * locals.var_cox_qm_dn4));
        locals.var_cgeff_dn6 = ((locals.var_zsat_ac_dn6 * locals.var_cox_qm) + (assign43400_e48594 * locals.var_cox_qm_dn6));
        locals.var_cgeff_dn7 = ((locals.var_zsat_ac_dn7 * locals.var_cox_qm) + (assign43400_e48594 * locals.var_cox_qm_dn7));
        locals.var_cgeff_dn8 = ((locals.var_zsat_ac_dn8 * locals.var_cox_qm) + (assign43400_e48594 * locals.var_cox_qm_dn8));
        locals.var_cgeff_dn9 = ((locals.var_zsat_ac_dn9 * locals.var_cox_qm) + (assign43400_e48594 * locals.var_cox_qm_dn9));

        let assign43410_e48601: f64 = (0.25 * locals.var_sigvds);
        let assign43410_e48603: f64 = (assign43410_e48601 * locals.var_sqrt_t2);
        let assign43410_e48604: f64 = (0.5 - assign43410_e48603);
        let assign43410_e48605: f64 = (locals.var_cgeff * assign43410_e48604);
        locals.var_cdgeff = assign43410_e48605;
        locals.var_cdgeff_dn4 = ((locals.var_cgeff_dn4 * assign43410_e48604) + (locals.var_cgeff * (-(assign43410_e48601 * locals.var_sqrt_t2_dn4))));
        locals.var_cdgeff_dn6 = ((locals.var_cgeff_dn6 * assign43410_e48604) + (locals.var_cgeff * (-(assign43410_e48601 * locals.var_sqrt_t2_dn6))));
        locals.var_cdgeff_dn7 = ((locals.var_cgeff_dn7 * assign43410_e48604) + (locals.var_cgeff * (-(assign43410_e48601 * locals.var_sqrt_t2_dn7))));
        locals.var_cdgeff_dn8 = ((locals.var_cgeff_dn8 * assign43410_e48604) + (locals.var_cgeff * (-(assign43410_e48601 * locals.var_sqrt_t2_dn8))));
        locals.var_cdgeff_dn9 = ((locals.var_cgeff_dn9 * assign43410_e48604) + (locals.var_cgeff * (-(assign43410_e48601 * locals.var_sqrt_t2_dn9))));

        let assign43420_e48608: f64 = (locals.var_cgeff - locals.var_cdgeff);
        locals.var_csgeff = assign43420_e48608;
        locals.var_csgeff_dn4 = (locals.var_cgeff_dn4 - locals.var_cdgeff_dn4);
        locals.var_csgeff_dn6 = (locals.var_cgeff_dn6 - locals.var_cdgeff_dn6);
        locals.var_csgeff_dn7 = (locals.var_cgeff_dn7 - locals.var_cdgeff_dn7);
        locals.var_csgeff_dn8 = (locals.var_cgeff_dn8 - locals.var_cdgeff_dn8);
        locals.var_csgeff_dn9 = (locals.var_cgeff_dn9 - locals.var_cdgeff_dn9);

        locals.var_migid = 0.0;
        locals.var_migid_dn4 = 0.0;
        locals.var_migid_dn6 = 0.0;
        locals.var_migid_dn7 = 0.0;
        locals.var_migid_dn8 = 0.0;
        locals.var_migid_dn9 = 0.0;

        let assign43450_e48613: f64 = if p.p6 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1279 = assign43450_e48613;

        let (assign43460_e48639, assign43460_e48639_d_n4, assign43460_e48639_d_n6, assign43460_e48639_d_n7, assign43460_e48639_d_n8, assign43460_e48639_d_n9,) = {
    if (locals.var_guard1279 != 0.0) {
        let assign43460_e48617: f64 = (locals.var_t1 / 12.0);
        let assign43460_e48621: f64 = (locals.var_t1 + 0.2);
        let assign43460_e48623: f64 = (assign43460_e48621 - locals.var_t2x12);
        let assign43460_e48624: f64 = (locals.var_t2 * assign43460_e48623);
        let assign43460_e48625: f64 = (assign43460_e48617 - assign43460_e48624);
        let assign43460_e48628: f64 = (1.6 * locals.var_t2);
        let assign43460_e48631: f64 = (locals.var_t1 + 1.0);
        let assign43460_e48633: f64 = (assign43460_e48631 - locals.var_t2x12);
        let assign43460_e48634: f64 = (assign43460_e48628 * assign43460_e48633);
        let assign43460_e48636: f64 = (assign43460_e48634 * locals.var_r);
        let assign43460_e48637: f64 = (assign43460_e48625 - assign43460_e48636);
        (assign43460_e48637, (((locals.var_t1_dn4 / 12.0) - ((locals.var_t2_dn4 * assign43460_e48623) + (locals.var_t2 * (locals.var_t1_dn4 - locals.var_t2x12_dn4)))) - (((((1.6 * locals.var_t2_dn4) * assign43460_e48633) + (assign43460_e48628 * (locals.var_t1_dn4 - locals.var_t2x12_dn4))) * locals.var_r) + (assign43460_e48634 * locals.var_r_dn4))), (((locals.var_t1_dn6 / 12.0) - ((locals.var_t2_dn6 * assign43460_e48623) + (locals.var_t2 * (locals.var_t1_dn6 - locals.var_t2x12_dn6)))) - (((((1.6 * locals.var_t2_dn6) * assign43460_e48633) + (assign43460_e48628 * (locals.var_t1_dn6 - locals.var_t2x12_dn6))) * locals.var_r) + (assign43460_e48634 * locals.var_r_dn6))), (((locals.var_t1_dn7 / 12.0) - ((locals.var_t2_dn7 * assign43460_e48623) + (locals.var_t2 * (locals.var_t1_dn7 - locals.var_t2x12_dn7)))) - (((((1.6 * locals.var_t2_dn7) * assign43460_e48633) + (assign43460_e48628 * (locals.var_t1_dn7 - locals.var_t2x12_dn7))) * locals.var_r) + (assign43460_e48634 * locals.var_r_dn7))), (((locals.var_t1_dn8 / 12.0) - ((locals.var_t2_dn8 * assign43460_e48623) + (locals.var_t2 * (locals.var_t1_dn8 - locals.var_t2x12_dn8)))) - (((((1.6 * locals.var_t2_dn8) * assign43460_e48633) + (assign43460_e48628 * (locals.var_t1_dn8 - locals.var_t2x12_dn8))) * locals.var_r) + (assign43460_e48634 * locals.var_r_dn8))), (((locals.var_t1_dn9 / 12.0) - ((locals.var_t2_dn9 * assign43460_e48623) + (locals.var_t2 * (locals.var_t1_dn9 - locals.var_t2x12_dn9)))) - (((((1.6 * locals.var_t2_dn9) * assign43460_e48633) + (assign43460_e48628 * (locals.var_t1_dn9 - locals.var_t2x12_dn9))) * locals.var_r) + (assign43460_e48634 * locals.var_r_dn9))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign43460_e48639;
        locals.var_temp1_dn4 = assign43460_e48639_d_n4;
        locals.var_temp1_dn6 = assign43460_e48639_d_n6;
        locals.var_temp1_dn7 = assign43460_e48639_d_n7;
        locals.var_temp1_dn8 = assign43460_e48639_d_n8;
        locals.var_temp1_dn9 = assign43460_e48639_d_n9;

        let (assign43470_e48645, assign43470_e48645_d_n4, assign43470_e48645_d_n6, assign43470_e48645_d_n7, assign43470_e48645_d_n8, assign43470_e48645_d_n9,) = {
    if (locals.var_guard1279 != 0.0) {
        let assign43470_e48643: f64 = (locals.var_temp1).max(1e-40);
        (assign43470_e48643, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn4 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn6 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn7 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn8 } else { 0.0 }, if locals.var_temp1 >= 1e-40 { locals.var_temp1_dn9 } else { 0.0 },)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign43470_e48645;
        locals.var_temp2_dn4 = assign43470_e48645_d_n4;
        locals.var_temp2_dn6 = assign43470_e48645_d_n6;
        locals.var_temp2_dn7 = assign43470_e48645_d_n7;
        locals.var_temp2_dn8 = assign43470_e48645_d_n8;
        locals.var_temp2_dn9 = assign43470_e48645_d_n9;

        let (assign43480_e48655, assign43480_e48655_d_n4, assign43480_e48655_d_n6, assign43480_e48655_d_n7, assign43480_e48655_d_n8, assign43480_e48655_d_n9,) = {
    if (locals.var_guard1279 != 0.0) {
        let assign43480_e48649: f64 = (locals.var_g_ideal * locals.var_lc);
        let assign43480_e48651: f64 = (assign43480_e48649 * locals.var_lc);
        let assign43480_e48653: f64 = (assign43480_e48651 / locals.var_temp2);
        (assign43480_e48653, (((((((locals.var_g_ideal_dn4 * locals.var_lc) + (locals.var_g_ideal * locals.var_lc_dn4)) * locals.var_lc) + (assign43480_e48649 * locals.var_lc_dn4)) * locals.var_temp2) - (assign43480_e48651 * locals.var_temp2_dn4)) / (locals.var_temp2 * locals.var_temp2)), (((((((locals.var_g_ideal_dn6 * locals.var_lc) + (locals.var_g_ideal * locals.var_lc_dn6)) * locals.var_lc) + (assign43480_e48649 * locals.var_lc_dn6)) * locals.var_temp2) - (assign43480_e48651 * locals.var_temp2_dn6)) / (locals.var_temp2 * locals.var_temp2)), (((((((locals.var_g_ideal_dn7 * locals.var_lc) + (locals.var_g_ideal * locals.var_lc_dn7)) * locals.var_lc) + (assign43480_e48649 * locals.var_lc_dn7)) * locals.var_temp2) - (assign43480_e48651 * locals.var_temp2_dn7)) / (locals.var_temp2 * locals.var_temp2)), (((((((locals.var_g_ideal_dn8 * locals.var_lc) + (locals.var_g_ideal * locals.var_lc_dn8)) * locals.var_lc) + (assign43480_e48649 * locals.var_lc_dn8)) * locals.var_temp2) - (assign43480_e48651 * locals.var_temp2_dn8)) / (locals.var_temp2 * locals.var_temp2)), (((((((locals.var_g_ideal_dn9 * locals.var_lc) + (locals.var_g_ideal * locals.var_lc_dn9)) * locals.var_lc) + (assign43480_e48649 * locals.var_lc_dn9)) * locals.var_temp2) - (assign43480_e48651 * locals.var_temp2_dn9)) / (locals.var_temp2 * locals.var_temp2)),)
    } else {
        (locals.var_gsig, locals.var_gsig_dn4, locals.var_gsig_dn6, locals.var_gsig_dn7, locals.var_gsig_dn8, locals.var_gsig_dn9,)
    }
};
        locals.var_gsig = assign43480_e48655;
        locals.var_gsig_dn4 = assign43480_e48655_d_n4;
        locals.var_gsig_dn6 = assign43480_e48655_d_n6;
        locals.var_gsig_dn7 = assign43480_e48655_d_n7;
        locals.var_gsig_dn8 = assign43480_e48655_d_n8;
        locals.var_gsig_dn9 = assign43480_e48655_d_n9;

        let assign43500_e48668: f64 = if locals.var_gsid > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1280 = assign43500_e48668;

        let (assign43510_e48692, assign43510_e48692_d_n4, assign43510_e48692_d_n6, assign43510_e48692_d_n7, assign43510_e48692_d_n8, assign43510_e48692_d_n9,) = {
    if ((locals.var_guard1279 != 0.0) && (locals.var_guard1280 != 0.0)) {
        let assign43510_e48674: f64 = (locals.var_lcinv2 * locals.var_sqrt_t2);
        let assign43510_e48677: f64 = (1.0 - locals.var_t2x12);
        let assign43510_e48681: f64 = (19.2 * locals.var_t2);
        let assign43510_e48682: f64 = (locals.var_t1 + assign43510_e48681);
        let assign43510_e48685: f64 = (locals.var_t1 * locals.var_t2x12);
        let assign43510_e48686: f64 = (assign43510_e48682 - assign43510_e48685);
        let assign43510_e48688: f64 = (assign43510_e48686 * locals.var_r);
        let assign43510_e48689: f64 = (assign43510_e48677 - assign43510_e48688);
        let assign43510_e48690: f64 = (assign43510_e48674 * assign43510_e48689);
        (assign43510_e48690, ((((locals.var_lcinv2_dn4 * locals.var_sqrt_t2) + (locals.var_lcinv2 * locals.var_sqrt_t2_dn4)) * assign43510_e48689) + (assign43510_e48674 * ((-locals.var_t2x12_dn4) - ((((locals.var_t1_dn4 + (19.2 * locals.var_t2_dn4)) - ((locals.var_t1_dn4 * locals.var_t2x12) + (locals.var_t1 * locals.var_t2x12_dn4))) * locals.var_r) + (assign43510_e48686 * locals.var_r_dn4))))), ((((locals.var_lcinv2_dn6 * locals.var_sqrt_t2) + (locals.var_lcinv2 * locals.var_sqrt_t2_dn6)) * assign43510_e48689) + (assign43510_e48674 * ((-locals.var_t2x12_dn6) - ((((locals.var_t1_dn6 + (19.2 * locals.var_t2_dn6)) - ((locals.var_t1_dn6 * locals.var_t2x12) + (locals.var_t1 * locals.var_t2x12_dn6))) * locals.var_r) + (assign43510_e48686 * locals.var_r_dn6))))), ((((locals.var_lcinv2_dn7 * locals.var_sqrt_t2) + (locals.var_lcinv2 * locals.var_sqrt_t2_dn7)) * assign43510_e48689) + (assign43510_e48674 * ((-locals.var_t2x12_dn7) - ((((locals.var_t1_dn7 + (19.2 * locals.var_t2_dn7)) - ((locals.var_t1_dn7 * locals.var_t2x12) + (locals.var_t1 * locals.var_t2x12_dn7))) * locals.var_r) + (assign43510_e48686 * locals.var_r_dn7))))), ((((locals.var_lcinv2_dn8 * locals.var_sqrt_t2) + (locals.var_lcinv2 * locals.var_sqrt_t2_dn8)) * assign43510_e48689) + (assign43510_e48674 * ((-locals.var_t2x12_dn8) - ((((locals.var_t1_dn8 + (19.2 * locals.var_t2_dn8)) - ((locals.var_t1_dn8 * locals.var_t2x12) + (locals.var_t1 * locals.var_t2x12_dn8))) * locals.var_r) + (assign43510_e48686 * locals.var_r_dn8))))), ((((locals.var_lcinv2_dn9 * locals.var_sqrt_t2) + (locals.var_lcinv2 * locals.var_sqrt_t2_dn9)) * assign43510_e48689) + (assign43510_e48674 * ((-locals.var_t2x12_dn9) - ((((locals.var_t1_dn9 + (19.2 * locals.var_t2_dn9)) - ((locals.var_t1_dn9 * locals.var_t2x12) + (locals.var_t1 * locals.var_t2x12_dn9))) * locals.var_r) + (assign43510_e48686 * locals.var_r_dn9))))),)
    } else {
        (locals.var_migid, locals.var_migid_dn4, locals.var_migid_dn6, locals.var_migid_dn7, locals.var_migid_dn8, locals.var_migid_dn9,)
    }
};
        locals.var_migid = assign43510_e48692;
        locals.var_migid_dn4 = assign43510_e48692_d_n4;
        locals.var_migid_dn6 = assign43510_e48692_d_n6;
        locals.var_migid_dn7 = assign43510_e48692_d_n7;
        locals.var_migid_dn8 = assign43510_e48692_d_n8;
        locals.var_migid_dn9 = assign43510_e48692_d_n9;

        let (assign43540_e48775, assign43540_e48775_d_n4, assign43540_e48775_d_n6, assign43540_e48775_d_n7, assign43540_e48775_d_n8, assign43540_e48775_d_n9,) = {
    if (locals.var_guard1279 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gsig, locals.var_gsig_dn4, locals.var_gsig_dn6, locals.var_gsig_dn7, locals.var_gsig_dn8, locals.var_gsig_dn9,)
    }
};
        locals.var_gsig = assign43540_e48775;
        locals.var_gsig_dn4 = assign43540_e48775_d_n4;
        locals.var_gsig_dn6 = assign43540_e48775_d_n6;
        locals.var_gsig_dn7 = assign43540_e48775_d_n7;
        locals.var_gsig_dn8 = assign43540_e48775_d_n8;
        locals.var_gsig_dn9 = assign43540_e48775_d_n9;

        locals.var_nstar = locals.var_nunit;
        locals.var_nstar_dn4 = locals.var_nunit_dn4;
        locals.var_nstar_dn6 = locals.var_nunit_dn6;
        locals.var_nstar_dn7 = locals.var_nunit_dn7;
        locals.var_nstar_dn8 = locals.var_nunit_dn8;
        locals.var_nstar_dn9 = locals.var_nunit_dn9;

        let assign43580_e48790: f64 = (locals.var_qim_dc + 1.0);
        let assign43580_e48791: f64 = (locals.var_nunit * assign43580_e48790);
        locals.var_nmstar = assign43580_e48791;
        locals.var_nmstar_dn4 = ((locals.var_nunit_dn4 * assign43580_e48790) + (locals.var_nunit * locals.var_qim_dc_dn4));
        locals.var_nmstar_dn6 = ((locals.var_nunit_dn6 * assign43580_e48790) + (locals.var_nunit * locals.var_qim_dc_dn6));
        locals.var_nmstar_dn7 = ((locals.var_nunit_dn7 * assign43580_e48790) + (locals.var_nunit * locals.var_qim_dc_dn7));
        locals.var_nmstar_dn8 = ((locals.var_nunit_dn8 * assign43580_e48790) + (locals.var_nunit * locals.var_qim_dc_dn8));
        locals.var_nmstar_dn9 = ((locals.var_nunit_dn9 * assign43580_e48790) + (locals.var_nunit * locals.var_qim_dc_dn9));

        let assign43590_e48795: f64 = (locals.var_qis_dc - locals.var_qid_dc);
        let assign43590_e48796: f64 = (locals.var_nunit * assign43590_e48795);
        locals.var_deltan = assign43590_e48796;
        locals.var_deltan_dn4 = ((locals.var_nunit_dn4 * assign43590_e48795) + (locals.var_nunit * (locals.var_qis_dc_dn4 - locals.var_qid_dc_dn4)));
        locals.var_deltan_dn6 = ((locals.var_nunit_dn6 * assign43590_e48795) + (locals.var_nunit * (locals.var_qis_dc_dn6 - locals.var_qid_dc_dn6)));
        locals.var_deltan_dn7 = ((locals.var_nunit_dn7 * assign43590_e48795) + (locals.var_nunit * (locals.var_qis_dc_dn7 - locals.var_qid_dc_dn7)));
        locals.var_deltan_dn8 = ((locals.var_nunit_dn8 * assign43590_e48795) + (locals.var_nunit * (locals.var_qis_dc_dn8 - locals.var_qid_dc_dn8)));
        locals.var_deltan_dn9 = ((locals.var_nunit_dn9 * assign43590_e48795) + (locals.var_nunit * (locals.var_qis_dc_dn9 - locals.var_qid_dc_dn9)));

        let assign43600_e48800: f64 = (locals.var_nfb_i * locals.var_nstar);
        let assign43600_e48801: f64 = (locals.var_nfa_i - assign43600_e48800);
        let assign43600_e48804: f64 = (locals.var_nfc_i * locals.var_nstar);
        let assign43600_e48806: f64 = (assign43600_e48804 * locals.var_nstar);
        let assign43600_e48807: f64 = (assign43600_e48801 + assign43600_e48806);
        let assign43600_e48811: f64 = (0.5 * locals.var_deltan);
        let assign43600_e48812: f64 = (locals.var_nmstar + assign43600_e48811);
        let assign43600_e48816: f64 = (0.5 * locals.var_deltan);
        let assign43600_e48817: f64 = (locals.var_nmstar - assign43600_e48816);
        let assign43600_e48818: f64 = (assign43600_e48812 / assign43600_e48817);
        let assign43600_e48819: f64 = (assign43600_e48818).ln();
        let assign43600_e48820: f64 = (assign43600_e48807 * assign43600_e48819);
        locals.var_temp1 = assign43600_e48820;
        locals.var_temp1_dn4 = ((((-(locals.var_nfb_i * locals.var_nstar_dn4)) + (((locals.var_nfc_i * locals.var_nstar_dn4) * locals.var_nstar) + (assign43600_e48804 * locals.var_nstar_dn4))) * assign43600_e48819) + (assign43600_e48807 * (((((locals.var_nmstar_dn4 + (0.5 * locals.var_deltan_dn4)) * assign43600_e48817) - (assign43600_e48812 * (locals.var_nmstar_dn4 - (0.5 * locals.var_deltan_dn4)))) / (assign43600_e48817 * assign43600_e48817)) / assign43600_e48818)));
        locals.var_temp1_dn6 = ((((-(locals.var_nfb_i * locals.var_nstar_dn6)) + (((locals.var_nfc_i * locals.var_nstar_dn6) * locals.var_nstar) + (assign43600_e48804 * locals.var_nstar_dn6))) * assign43600_e48819) + (assign43600_e48807 * (((((locals.var_nmstar_dn6 + (0.5 * locals.var_deltan_dn6)) * assign43600_e48817) - (assign43600_e48812 * (locals.var_nmstar_dn6 - (0.5 * locals.var_deltan_dn6)))) / (assign43600_e48817 * assign43600_e48817)) / assign43600_e48818)));
        locals.var_temp1_dn7 = ((((-(locals.var_nfb_i * locals.var_nstar_dn7)) + (((locals.var_nfc_i * locals.var_nstar_dn7) * locals.var_nstar) + (assign43600_e48804 * locals.var_nstar_dn7))) * assign43600_e48819) + (assign43600_e48807 * (((((locals.var_nmstar_dn7 + (0.5 * locals.var_deltan_dn7)) * assign43600_e48817) - (assign43600_e48812 * (locals.var_nmstar_dn7 - (0.5 * locals.var_deltan_dn7)))) / (assign43600_e48817 * assign43600_e48817)) / assign43600_e48818)));
        locals.var_temp1_dn8 = ((((-(locals.var_nfb_i * locals.var_nstar_dn8)) + (((locals.var_nfc_i * locals.var_nstar_dn8) * locals.var_nstar) + (assign43600_e48804 * locals.var_nstar_dn8))) * assign43600_e48819) + (assign43600_e48807 * (((((locals.var_nmstar_dn8 + (0.5 * locals.var_deltan_dn8)) * assign43600_e48817) - (assign43600_e48812 * (locals.var_nmstar_dn8 - (0.5 * locals.var_deltan_dn8)))) / (assign43600_e48817 * assign43600_e48817)) / assign43600_e48818)));
        locals.var_temp1_dn9 = ((((-(locals.var_nfb_i * locals.var_nstar_dn9)) + (((locals.var_nfc_i * locals.var_nstar_dn9) * locals.var_nstar) + (assign43600_e48804 * locals.var_nstar_dn9))) * assign43600_e48819) + (assign43600_e48807 * (((((locals.var_nmstar_dn9 + (0.5 * locals.var_deltan_dn9)) * assign43600_e48817) - (assign43600_e48812 * (locals.var_nmstar_dn9 - (0.5 * locals.var_deltan_dn9)))) / (assign43600_e48817 * assign43600_e48817)) / assign43600_e48818)));

        let assign43610_e48827: f64 = (2.0 * locals.var_nstar);
        let assign43610_e48828: f64 = (locals.var_nmstar - assign43610_e48827);
        let assign43610_e48829: f64 = (locals.var_nfc_i * assign43610_e48828);
        let assign43610_e48830: f64 = (locals.var_nfb_i + assign43610_e48829);
        let assign43610_e48832: f64 = (assign43610_e48830 * locals.var_deltan);
        let assign43610_e48833: f64 = (locals.var_temp1 + assign43610_e48832);
        locals.var_temp2 = assign43610_e48833;
        locals.var_temp2_dn4 = (locals.var_temp1_dn4 + (((locals.var_nfc_i * (locals.var_nmstar_dn4 - (2.0 * locals.var_nstar_dn4))) * locals.var_deltan) + (assign43610_e48830 * locals.var_deltan_dn4)));
        locals.var_temp2_dn6 = (locals.var_temp1_dn6 + (((locals.var_nfc_i * (locals.var_nmstar_dn6 - (2.0 * locals.var_nstar_dn6))) * locals.var_deltan) + (assign43610_e48830 * locals.var_deltan_dn6)));
        locals.var_temp2_dn7 = (locals.var_temp1_dn7 + (((locals.var_nfc_i * (locals.var_nmstar_dn7 - (2.0 * locals.var_nstar_dn7))) * locals.var_deltan) + (assign43610_e48830 * locals.var_deltan_dn7)));
        locals.var_temp2_dn8 = (locals.var_temp1_dn8 + (((locals.var_nfc_i * (locals.var_nmstar_dn8 - (2.0 * locals.var_nstar_dn8))) * locals.var_deltan) + (assign43610_e48830 * locals.var_deltan_dn8)));
        locals.var_temp2_dn9 = (locals.var_temp1_dn9 + (((locals.var_nfc_i * (locals.var_nmstar_dn9 - (2.0 * locals.var_nstar_dn9))) * locals.var_deltan) + (assign43610_e48830 * locals.var_deltan_dn9)));

        let assign43620_e48837: f64 = (locals.var_nfe_i * locals.var_esurf1_dc);
        let assign43620_e48840: f64 = (locals.var_nfeb_i * locals.var_esurf2_dc);
        let assign43620_e48841: f64 = (assign43620_e48837 + assign43620_e48840);
        let assign43620_e48844: f64 = (locals.var_qim_dc + 1.0);
        let assign43620_e48845: f64 = (assign43620_e48841 / assign43620_e48844);
        let assign43620_e48846: f64 = (1.0 + assign43620_e48845);
        locals.var_temp = assign43620_e48846;
        locals.var_temp_dn4 = (((((locals.var_nfe_i * locals.var_esurf1_dc_dn4) + (locals.var_nfeb_i * locals.var_esurf2_dc_dn4)) * assign43620_e48844) - (assign43620_e48841 * locals.var_qim_dc_dn4)) / (assign43620_e48844 * assign43620_e48844));
        locals.var_temp_dn6 = (((((locals.var_nfe_i * locals.var_esurf1_dc_dn6) + (locals.var_nfeb_i * locals.var_esurf2_dc_dn6)) * assign43620_e48844) - (assign43620_e48841 * locals.var_qim_dc_dn6)) / (assign43620_e48844 * assign43620_e48844));
        locals.var_temp_dn7 = (((((locals.var_nfe_i * locals.var_esurf1_dc_dn7) + (locals.var_nfeb_i * locals.var_esurf2_dc_dn7)) * assign43620_e48844) - (assign43620_e48841 * locals.var_qim_dc_dn7)) / (assign43620_e48844 * assign43620_e48844));
        locals.var_temp_dn8 = (((((locals.var_nfe_i * locals.var_esurf1_dc_dn8) + (locals.var_nfeb_i * locals.var_esurf2_dc_dn8)) * assign43620_e48844) - (assign43620_e48841 * locals.var_qim_dc_dn8)) / (assign43620_e48844 * assign43620_e48844));
        locals.var_temp_dn9 = (((((locals.var_nfe_i * locals.var_esurf1_dc_dn9) + (locals.var_nfeb_i * locals.var_esurf2_dc_dn9)) * assign43620_e48844) - (assign43620_e48841 * locals.var_qim_dc_dn9)) / (assign43620_e48844 * assign43620_e48844));

        let assign43630_e48850: f64 = (locals.var_temp + 0.01);
        let assign43630_e48853: f64 = (locals.var_temp - 0.01);
        let assign43630_e48856: f64 = (locals.var_temp - 0.01);
        let assign43630_e48857: f64 = (assign43630_e48853 * assign43630_e48856);
        let assign43630_e48859: f64 = (assign43630_e48857 + 0.0001);
        let assign43630_e48860: f64 = (assign43630_e48859).sqrt();
        let assign43630_e48861: f64 = (assign43630_e48850 + assign43630_e48860);
        let assign43630_e48862: f64 = (0.5 * assign43630_e48861);
        locals.var_temp3 = assign43630_e48862;
        locals.var_temp3_dn4 = (0.5 * (locals.var_temp_dn4 + (((locals.var_temp_dn4 * assign43630_e48856) + (assign43630_e48853 * locals.var_temp_dn4)) / (2.0 * assign43630_e48860))));
        locals.var_temp3_dn6 = (0.5 * (locals.var_temp_dn6 + (((locals.var_temp_dn6 * assign43630_e48856) + (assign43630_e48853 * locals.var_temp_dn6)) / (2.0 * assign43630_e48860))));
        locals.var_temp3_dn7 = (0.5 * (locals.var_temp_dn7 + (((locals.var_temp_dn7 * assign43630_e48856) + (assign43630_e48853 * locals.var_temp_dn7)) / (2.0 * assign43630_e48860))));
        locals.var_temp3_dn8 = (0.5 * (locals.var_temp_dn8 + (((locals.var_temp_dn8 * assign43630_e48856) + (assign43630_e48853 * locals.var_temp_dn8)) / (2.0 * assign43630_e48860))));
        locals.var_temp3_dn9 = (0.5 * (locals.var_temp_dn9 + (((locals.var_temp_dn9 * assign43630_e48856) + (assign43630_e48853 * locals.var_temp_dn9)) / (2.0 * assign43630_e48860))));

        let assign43640_e48865: f64 = (1.602176565e-19 * locals.var_fact_ids);
        let assign43640_e48867: f64 = (assign43640_e48865 * locals.var_ids);
        let assign43640_e48869: f64 = (assign43640_e48867 / locals.var_gvsat);
        let assign43640_e48871: f64 = (assign43640_e48869 * locals.var_temp2);
        let assign43640_e48873: f64 = (assign43640_e48871 / locals.var_nstar);
        let assign43640_e48875: f64 = (assign43640_e48873 * locals.var_temp3);
        locals.var_temp = assign43640_e48875;
        locals.var_temp_dn4 = (((((((((((((1.602176565e-19 * locals.var_fact_ids_dn4) * locals.var_ids) + (assign43640_e48865 * locals.var_ids_dn4)) * locals.var_gvsat) - (assign43640_e48867 * locals.var_gvsat_dn4)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_temp2) + (assign43640_e48869 * locals.var_temp2_dn4)) * locals.var_nstar) - (assign43640_e48871 * locals.var_nstar_dn4)) / (locals.var_nstar * locals.var_nstar)) * locals.var_temp3) + (assign43640_e48873 * locals.var_temp3_dn4));
        locals.var_temp_dn6 = (((((((((((((1.602176565e-19 * locals.var_fact_ids_dn6) * locals.var_ids) + (assign43640_e48865 * locals.var_ids_dn6)) * locals.var_gvsat) - (assign43640_e48867 * locals.var_gvsat_dn6)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_temp2) + (assign43640_e48869 * locals.var_temp2_dn6)) * locals.var_nstar) - (assign43640_e48871 * locals.var_nstar_dn6)) / (locals.var_nstar * locals.var_nstar)) * locals.var_temp3) + (assign43640_e48873 * locals.var_temp3_dn6));
        locals.var_temp_dn7 = (((((((((((((1.602176565e-19 * locals.var_fact_ids_dn7) * locals.var_ids) + (assign43640_e48865 * locals.var_ids_dn7)) * locals.var_gvsat) - (assign43640_e48867 * locals.var_gvsat_dn7)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_temp2) + (assign43640_e48869 * locals.var_temp2_dn7)) * locals.var_nstar) - (assign43640_e48871 * locals.var_nstar_dn7)) / (locals.var_nstar * locals.var_nstar)) * locals.var_temp3) + (assign43640_e48873 * locals.var_temp3_dn7));
        locals.var_temp_dn8 = (((((((((((((1.602176565e-19 * locals.var_fact_ids_dn8) * locals.var_ids) + (assign43640_e48865 * locals.var_ids_dn8)) * locals.var_gvsat) - (assign43640_e48867 * locals.var_gvsat_dn8)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_temp2) + (assign43640_e48869 * locals.var_temp2_dn8)) * locals.var_nstar) - (assign43640_e48871 * locals.var_nstar_dn8)) / (locals.var_nstar * locals.var_nstar)) * locals.var_temp3) + (assign43640_e48873 * locals.var_temp3_dn8));
        locals.var_temp_dn9 = (((((((((((((1.602176565e-19 * locals.var_fact_ids_dn9) * locals.var_ids) + (assign43640_e48865 * locals.var_ids_dn9)) * locals.var_gvsat) - (assign43640_e48867 * locals.var_gvsat_dn9)) / (locals.var_gvsat * locals.var_gvsat)) * locals.var_temp2) + (assign43640_e48869 * locals.var_temp2_dn9)) * locals.var_nstar) - (assign43640_e48871 * locals.var_nstar_dn9)) / (locals.var_nstar * locals.var_nstar)) * locals.var_temp3) + (assign43640_e48873 * locals.var_temp3_dn9));

        let assign43710_e48935: f64 = (locals.var_tkd * 8.617332384961e-5);
        let assign43710_e48936: f64 = (1.0 / assign43710_e48935);
        locals.var_inv_phit0_op = assign43710_e48936;
        locals.var_inv_phit0_op_dn4 = (-((locals.var_tkd_dn4 * 8.617332384961e-5) / (assign43710_e48935 * assign43710_e48935)));
        locals.var_inv_phit0_op_dn6 = (-((locals.var_tkd_dn6 * 8.617332384961e-5) / (assign43710_e48935 * assign43710_e48935)));
        locals.var_inv_phit0_op_dn7 = (-((locals.var_tkd_dn7 * 8.617332384961e-5) / (assign43710_e48935 * assign43710_e48935)));
        locals.var_inv_phit0_op_dn8 = (-((locals.var_tkd_dn8 * 8.617332384961e-5) / (assign43710_e48935 * assign43710_e48935)));
        locals.var_inv_phit0_op_dn9 = (-((locals.var_tkd_dn9 * 8.617332384961e-5) / (assign43710_e48935 * assign43710_e48935)));

        let assign43720_e48940: f64 = (0.000473 * locals.var_tkd);
        let assign43720_e48942: f64 = (assign43720_e48940 * locals.var_tkd);
        let assign43720_e48945: f64 = (636.0 + locals.var_tkd);
        let assign43720_e48946: f64 = (assign43720_e48942 / assign43720_e48945);
        let assign43720_e48947: f64 = (1.17 - assign43720_e48946);
        locals.var_egsi_op = assign43720_e48947;
        locals.var_egsi_op_dn4 = (-((((((0.000473 * locals.var_tkd_dn4) * locals.var_tkd) + (assign43720_e48940 * locals.var_tkd_dn4)) * assign43720_e48945) - (assign43720_e48942 * locals.var_tkd_dn4)) / (assign43720_e48945 * assign43720_e48945)));
        locals.var_egsi_op_dn6 = (-((((((0.000473 * locals.var_tkd_dn6) * locals.var_tkd) + (assign43720_e48940 * locals.var_tkd_dn6)) * assign43720_e48945) - (assign43720_e48942 * locals.var_tkd_dn6)) / (assign43720_e48945 * assign43720_e48945)));
        locals.var_egsi_op_dn7 = (-((((((0.000473 * locals.var_tkd_dn7) * locals.var_tkd) + (assign43720_e48940 * locals.var_tkd_dn7)) * assign43720_e48945) - (assign43720_e48942 * locals.var_tkd_dn7)) / (assign43720_e48945 * assign43720_e48945)));
        locals.var_egsi_op_dn8 = (-((((((0.000473 * locals.var_tkd_dn8) * locals.var_tkd) + (assign43720_e48940 * locals.var_tkd_dn8)) * assign43720_e48945) - (assign43720_e48942 * locals.var_tkd_dn8)) / (assign43720_e48945 * assign43720_e48945)));
        locals.var_egsi_op_dn9 = (-((((((0.000473 * locals.var_tkd_dn9) * locals.var_tkd) + (assign43720_e48940 * locals.var_tkd_dn9)) * assign43720_e48945) - (assign43720_e48942 * locals.var_tkd_dn9)) / (assign43720_e48945 * assign43720_e48945)));

        let assign43730_e48951: f64 = (0.0004774 * locals.var_tkd);
        let assign43730_e48953: f64 = (assign43730_e48951 * locals.var_tkd);
        let assign43730_e48956: f64 = (235.0 + locals.var_tkd);
        let assign43730_e48957: f64 = (assign43730_e48953 / assign43730_e48956);
        let assign43730_e48958: f64 = (0.744 - assign43730_e48957);
        locals.var_egge_op = assign43730_e48958;
        locals.var_egge_op_dn4 = (-((((((0.0004774 * locals.var_tkd_dn4) * locals.var_tkd) + (assign43730_e48951 * locals.var_tkd_dn4)) * assign43730_e48956) - (assign43730_e48953 * locals.var_tkd_dn4)) / (assign43730_e48956 * assign43730_e48956)));
        locals.var_egge_op_dn6 = (-((((((0.0004774 * locals.var_tkd_dn6) * locals.var_tkd) + (assign43730_e48951 * locals.var_tkd_dn6)) * assign43730_e48956) - (assign43730_e48953 * locals.var_tkd_dn6)) / (assign43730_e48956 * assign43730_e48956)));
        locals.var_egge_op_dn7 = (-((((((0.0004774 * locals.var_tkd_dn7) * locals.var_tkd) + (assign43730_e48951 * locals.var_tkd_dn7)) * assign43730_e48956) - (assign43730_e48953 * locals.var_tkd_dn7)) / (assign43730_e48956 * assign43730_e48956)));
        locals.var_egge_op_dn8 = (-((((((0.0004774 * locals.var_tkd_dn8) * locals.var_tkd) + (assign43730_e48951 * locals.var_tkd_dn8)) * assign43730_e48956) - (assign43730_e48953 * locals.var_tkd_dn8)) / (assign43730_e48956 * assign43730_e48956)));
        locals.var_egge_op_dn9 = (-((((((0.0004774 * locals.var_tkd_dn9) * locals.var_tkd) + (assign43730_e48951 * locals.var_tkd_dn9)) * assign43730_e48956) - (assign43730_e48953 * locals.var_tkd_dn9)) / (assign43730_e48956 * assign43730_e48956)));

        let assign43740_e48961: f64 = (locals.var_egge_op - locals.var_egsi_op);
        let assign43740_e48963: f64 = (-0.4);
        let assign43740_e48965: f64 = (assign43740_e48963 * locals.var_one_m_xge);
        let assign43740_e48966: f64 = (assign43740_e48961 + assign43740_e48965);
        let assign43740_e48968: f64 = (assign43740_e48966 * locals.var_xge_i);
        locals.var_deg_op = assign43740_e48968;
        locals.var_deg_op_dn4 = ((locals.var_egge_op_dn4 - locals.var_egsi_op_dn4) * locals.var_xge_i);
        locals.var_deg_op_dn6 = ((locals.var_egge_op_dn6 - locals.var_egsi_op_dn6) * locals.var_xge_i);
        locals.var_deg_op_dn7 = ((locals.var_egge_op_dn7 - locals.var_egsi_op_dn7) * locals.var_xge_i);
        locals.var_deg_op_dn8 = ((locals.var_egge_op_dn8 - locals.var_egsi_op_dn8) * locals.var_xge_i);
        locals.var_deg_op_dn9 = ((locals.var_egge_op_dn9 - locals.var_egsi_op_dn9) * locals.var_xge_i);

        let assign43750_e48971: f64 = (locals.var_egsi_op + locals.var_deg_op);
        locals.var_eg_op = assign43750_e48971;
        locals.var_eg_op_dn4 = (locals.var_egsi_op_dn4 + locals.var_deg_op_dn4);
        locals.var_eg_op_dn6 = (locals.var_egsi_op_dn6 + locals.var_deg_op_dn6);
        locals.var_eg_op_dn7 = (locals.var_egsi_op_dn7 + locals.var_deg_op_dn7);
        locals.var_eg_op_dn8 = (locals.var_egsi_op_dn8 + locals.var_deg_op_dn8);
        locals.var_eg_op_dn9 = (locals.var_egsi_op_dn9 + locals.var_deg_op_dn9);

        let assign43760_e48974: f64 = (0.5 * locals.var_eg_op);
        let assign43760_e48976: f64 = (assign43760_e48974 * locals.var_inv_phit0_op);
        locals.var_eg_2phit0_op = assign43760_e48976;
        locals.var_eg_2phit0_op_dn4 = (((0.5 * locals.var_eg_op_dn4) * locals.var_inv_phit0_op) + (assign43760_e48974 * locals.var_inv_phit0_op_dn4));
        locals.var_eg_2phit0_op_dn6 = (((0.5 * locals.var_eg_op_dn6) * locals.var_inv_phit0_op) + (assign43760_e48974 * locals.var_inv_phit0_op_dn6));
        locals.var_eg_2phit0_op_dn7 = (((0.5 * locals.var_eg_op_dn7) * locals.var_inv_phit0_op) + (assign43760_e48974 * locals.var_inv_phit0_op_dn7));
        locals.var_eg_2phit0_op_dn8 = (((0.5 * locals.var_eg_op_dn8) * locals.var_inv_phit0_op) + (assign43760_e48974 * locals.var_inv_phit0_op_dn8));
        locals.var_eg_2phit0_op_dn9 = (((0.5 * locals.var_eg_op_dn9) * locals.var_inv_phit0_op) + (assign43760_e48974 * locals.var_inv_phit0_op_dn9));

        let assign43770_e48979: f64 = (0.05 * locals.var_xge_i);
        let assign43770_e48982: f64 = (0.5 * locals.var_deg_op);
        let assign43770_e48983: f64 = (assign43770_e48979 - assign43770_e48982);
        locals.var_dvfbch_op = assign43770_e48983;
        locals.var_dvfbch_op_dn4 = (-(0.5 * locals.var_deg_op_dn4));
        locals.var_dvfbch_op_dn6 = (-(0.5 * locals.var_deg_op_dn6));
        locals.var_dvfbch_op_dn7 = (-(0.5 * locals.var_deg_op_dn7));
        locals.var_dvfbch_op_dn8 = (-(0.5 * locals.var_deg_op_dn8));
        locals.var_dvfbch_op_dn9 = (-(0.5 * locals.var_deg_op_dn9));

    }

    pub(super) fn stamp_transient_block_118(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign43780_e48986: f64 = (locals.var_tkd * 0.0033333333333);
        let assign43780_e48987: f64 = (assign43780_e48986).sqrt();
        locals.var_temp = assign43780_e48987;
        locals.var_temp_dn4 = ((locals.var_tkd_dn4 * 0.0033333333333) / (2.0 * assign43780_e48987));
        locals.var_temp_dn6 = ((locals.var_tkd_dn6 * 0.0033333333333) / (2.0 * assign43780_e48987));
        locals.var_temp_dn7 = ((locals.var_tkd_dn7 * 0.0033333333333) / (2.0 * assign43780_e48987));
        locals.var_temp_dn8 = ((locals.var_tkd_dn8 * 0.0033333333333) / (2.0 * assign43780_e48987));
        locals.var_temp_dn9 = ((locals.var_tkd_dn9 * 0.0033333333333) / (2.0 * assign43780_e48987));

        let assign43790_e48990: f64 = (4.05e25 * locals.var_temp);
        let assign43790_e48992: f64 = (assign43790_e48990 * locals.var_temp);
        let assign43790_e48994: f64 = (assign43790_e48992 * locals.var_temp);
        locals.var_temp1 = assign43790_e48994;
        locals.var_temp1_dn4 = (((((4.05e25 * locals.var_temp_dn4) * locals.var_temp) + (assign43790_e48990 * locals.var_temp_dn4)) * locals.var_temp) + (assign43790_e48992 * locals.var_temp_dn4));
        locals.var_temp1_dn6 = (((((4.05e25 * locals.var_temp_dn6) * locals.var_temp) + (assign43790_e48990 * locals.var_temp_dn6)) * locals.var_temp) + (assign43790_e48992 * locals.var_temp_dn6));
        locals.var_temp1_dn7 = (((((4.05e25 * locals.var_temp_dn7) * locals.var_temp) + (assign43790_e48990 * locals.var_temp_dn7)) * locals.var_temp) + (assign43790_e48992 * locals.var_temp_dn7));
        locals.var_temp1_dn8 = (((((4.05e25 * locals.var_temp_dn8) * locals.var_temp) + (assign43790_e48990 * locals.var_temp_dn8)) * locals.var_temp) + (assign43790_e48992 * locals.var_temp_dn8));
        locals.var_temp1_dn9 = (((((4.05e25 * locals.var_temp_dn9) * locals.var_temp) + (assign43790_e48990 * locals.var_temp_dn9)) * locals.var_temp) + (assign43790_e48992 * locals.var_temp_dn9));

        let assign43800_e48997: f64 = (locals.var_temp1 * locals.var_niratio);
        locals.var_neff_op = assign43800_e48997;
        locals.var_neff_op_dn4 = (locals.var_temp1_dn4 * locals.var_niratio);
        locals.var_neff_op_dn6 = (locals.var_temp1_dn6 * locals.var_niratio);
        locals.var_neff_op_dn7 = (locals.var_temp1_dn7 * locals.var_niratio);
        locals.var_neff_op_dn8 = (locals.var_temp1_dn8 * locals.var_niratio);
        locals.var_neff_op_dn9 = (locals.var_temp1_dn9 * locals.var_niratio);

        let assign43810_e49002: f64 = (locals.var_ct_i * locals.var_tkr);
        let assign43810_e49004: f64 = (assign43810_e49002 / locals.var_tkd);
        let assign43810_e49005: f64 = (1.0 + assign43810_e49004);
        let assign43810_e49006: f64 = (locals.var_inv_phit0_op / assign43810_e49005);
        locals.var_inv_phit_op = assign43810_e49006;
        locals.var_inv_phit_op_dn4 = (((locals.var_inv_phit0_op_dn4 * assign43810_e49005) - (locals.var_inv_phit0_op * (-((assign43810_e49002 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd))))) / (assign43810_e49005 * assign43810_e49005));
        locals.var_inv_phit_op_dn6 = (((locals.var_inv_phit0_op_dn6 * assign43810_e49005) - (locals.var_inv_phit0_op * (-((assign43810_e49002 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd))))) / (assign43810_e49005 * assign43810_e49005));
        locals.var_inv_phit_op_dn7 = (((locals.var_inv_phit0_op_dn7 * assign43810_e49005) - (locals.var_inv_phit0_op * (-((assign43810_e49002 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd))))) / (assign43810_e49005 * assign43810_e49005));
        locals.var_inv_phit_op_dn8 = (((locals.var_inv_phit0_op_dn8 * assign43810_e49005) - (locals.var_inv_phit0_op * (-((assign43810_e49002 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd))))) / (assign43810_e49005 * assign43810_e49005));
        locals.var_inv_phit_op_dn9 = (((locals.var_inv_phit0_op_dn9 * assign43810_e49005) - (locals.var_inv_phit0_op * (-((assign43810_e49002 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd))))) / (assign43810_e49005 * assign43810_e49005));

        let assign43820_e49009: f64 = (2.0 * 1.602176565e-19);
        let assign43820_e49011: f64 = (assign43820_e49009 * locals.var_neff_op);
        let assign43820_e49013: f64 = (assign43820_e49011 * locals.var_epsch);
        let assign43820_e49015: f64 = (assign43820_e49013 * locals.var_inv_phit_op);
        locals.var_a0_csisq_op = assign43820_e49015;
        locals.var_a0_csisq_op_dn4 = ((((assign43820_e49009 * locals.var_neff_op_dn4) * locals.var_epsch) * locals.var_inv_phit_op) + (assign43820_e49013 * locals.var_inv_phit_op_dn4));
        locals.var_a0_csisq_op_dn6 = ((((assign43820_e49009 * locals.var_neff_op_dn6) * locals.var_epsch) * locals.var_inv_phit_op) + (assign43820_e49013 * locals.var_inv_phit_op_dn6));
        locals.var_a0_csisq_op_dn7 = ((((assign43820_e49009 * locals.var_neff_op_dn7) * locals.var_epsch) * locals.var_inv_phit_op) + (assign43820_e49013 * locals.var_inv_phit_op_dn7));
        locals.var_a0_csisq_op_dn8 = ((((assign43820_e49009 * locals.var_neff_op_dn8) * locals.var_epsch) * locals.var_inv_phit_op) + (assign43820_e49013 * locals.var_inv_phit_op_dn8));
        locals.var_a0_csisq_op_dn9 = ((((assign43820_e49009 * locals.var_neff_op_dn9) * locals.var_epsch) * locals.var_inv_phit_op) + (assign43820_e49013 * locals.var_inv_phit_op_dn9));

        let assign43830_e49018: f64 = (locals.var_csiprime_0 * locals.var_csiprime_0);
        let assign43830_e49020: f64 = (assign43830_e49018 / locals.var_a0_csisq_op);
        let assign43830_e49021: f64 = (assign43830_e49020).ln();
        let assign43830_e49023: f64 = (assign43830_e49021 - 0.6931471805599);
        let assign43830_e49025: f64 = (assign43830_e49023 + locals.var_eg_2phit0_op);
        locals.var_xth_1d_op = assign43830_e49025;
        locals.var_xth_1d_op_dn4 = (((-((assign43830_e49018 * locals.var_a0_csisq_op_dn4) / (locals.var_a0_csisq_op * locals.var_a0_csisq_op))) / assign43830_e49020) + locals.var_eg_2phit0_op_dn4);
        locals.var_xth_1d_op_dn6 = (((-((assign43830_e49018 * locals.var_a0_csisq_op_dn6) / (locals.var_a0_csisq_op * locals.var_a0_csisq_op))) / assign43830_e49020) + locals.var_eg_2phit0_op_dn6);
        locals.var_xth_1d_op_dn7 = (((-((assign43830_e49018 * locals.var_a0_csisq_op_dn7) / (locals.var_a0_csisq_op * locals.var_a0_csisq_op))) / assign43830_e49020) + locals.var_eg_2phit0_op_dn7);
        locals.var_xth_1d_op_dn8 = (((-((assign43830_e49018 * locals.var_a0_csisq_op_dn8) / (locals.var_a0_csisq_op * locals.var_a0_csisq_op))) / assign43830_e49020) + locals.var_eg_2phit0_op_dn8);
        locals.var_xth_1d_op_dn9 = (((-((assign43830_e49018 * locals.var_a0_csisq_op_dn9) / (locals.var_a0_csisq_op * locals.var_a0_csisq_op))) / assign43830_e49020) + locals.var_eg_2phit0_op_dn9);

        let assign43840_e49028: f64 = (0.5 * 1.602176565e-19);
        let assign43840_e49030: f64 = (assign43840_e49028 * locals.var_nsddc_i);
        let assign43840_e49032: f64 = (assign43840_e49030 * locals.var_tsi_i);
        let assign43840_e49035: f64 = (locals.var_cox1prime + locals.var_cox2prime);
        let assign43840_e49036: f64 = (assign43840_e49032 / assign43840_e49035);
        let assign43840_e49038: f64 = (assign43840_e49036 * locals.var_inv_phit_op);
        locals.var_xsddep_op = assign43840_e49038;
        locals.var_xsddep_op_dn4 = (assign43840_e49036 * locals.var_inv_phit_op_dn4);
        locals.var_xsddep_op_dn6 = (assign43840_e49036 * locals.var_inv_phit_op_dn6);
        locals.var_xsddep_op_dn7 = (assign43840_e49036 * locals.var_inv_phit_op_dn7);
        locals.var_xsddep_op_dn8 = (assign43840_e49036 * locals.var_inv_phit_op_dn8);
        locals.var_xsddep_op_dn9 = (assign43840_e49036 * locals.var_inv_phit_op_dn9);

        let assign43850_e49041: f64 = (locals.var_cfd_i * locals.var_inv_phit_op);
        locals.var_xd0_op = assign43850_e49041;
        locals.var_xd0_op_dn4 = (locals.var_cfd_i * locals.var_inv_phit_op_dn4);
        locals.var_xd0_op_dn6 = (locals.var_cfd_i * locals.var_inv_phit_op_dn6);
        locals.var_xd0_op_dn7 = (locals.var_cfd_i * locals.var_inv_phit_op_dn7);
        locals.var_xd0_op_dn8 = (locals.var_cfd_i * locals.var_inv_phit_op_dn8);
        locals.var_xd0_op_dn9 = (locals.var_cfd_i * locals.var_inv_phit_op_dn9);

        locals.var_qq_op = 0.0;
        locals.var_qq_op_dn4 = 0.0;
        locals.var_qq_op_dn6 = 0.0;
        locals.var_qq_op_dn7 = 0.0;
        locals.var_qq_op_dn8 = 0.0;
        locals.var_qq_op_dn9 = 0.0;

        locals.var_dvfbpdep_op = 0.0;
        locals.var_dvfbpdep_op_dn4 = 0.0;
        locals.var_dvfbpdep_op_dn6 = 0.0;
        locals.var_dvfbpdep_op_dn7 = 0.0;
        locals.var_dvfbpdep_op_dn8 = 0.0;
        locals.var_dvfbpdep_op_dn9 = 0.0;

        let assign43880_e49046: f64 = if p.p9 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1350 = assign43880_e49046;

        let (assign43890_e49057, assign43890_e49057_d_n4, assign43890_e49057_d_n6, assign43890_e49057_d_n7, assign43890_e49057_d_n8, assign43890_e49057_d_n9,) = {
    if (locals.var_guard1350 != 0.0) {
        let assign43890_e49050: f64 = (1.0 / locals.var_inv_phit0_op);
        let assign43890_e49053: f64 = (locals.var_np_i / locals.var_neff_poly);
        let assign43890_e49054: f64 = (assign43890_e49053).ln();
        let assign43890_e49055: f64 = (assign43890_e49050 * assign43890_e49054);
        (assign43890_e49055, (((-(locals.var_inv_phit0_op_dn4 / (locals.var_inv_phit0_op * locals.var_inv_phit0_op))) * assign43890_e49054) + (assign43890_e49050 * ((((locals.var_np_i_dn4 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn4)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign43890_e49053))), (((-(locals.var_inv_phit0_op_dn6 / (locals.var_inv_phit0_op * locals.var_inv_phit0_op))) * assign43890_e49054) + (assign43890_e49050 * ((((locals.var_np_i_dn6 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn6)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign43890_e49053))), (((-(locals.var_inv_phit0_op_dn7 / (locals.var_inv_phit0_op * locals.var_inv_phit0_op))) * assign43890_e49054) + (assign43890_e49050 * ((((locals.var_np_i_dn7 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn7)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign43890_e49053))), (((-(locals.var_inv_phit0_op_dn8 / (locals.var_inv_phit0_op * locals.var_inv_phit0_op))) * assign43890_e49054) + (assign43890_e49050 * ((((locals.var_np_i_dn8 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn8)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign43890_e49053))), (((-(locals.var_inv_phit0_op_dn9 / (locals.var_inv_phit0_op * locals.var_inv_phit0_op))) * assign43890_e49054) + (assign43890_e49050 * ((((locals.var_np_i_dn9 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn9)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign43890_e49053))),)
    } else {
        (locals.var_dvfbpdep_op, locals.var_dvfbpdep_op_dn4, locals.var_dvfbpdep_op_dn6, locals.var_dvfbpdep_op_dn7, locals.var_dvfbpdep_op_dn8, locals.var_dvfbpdep_op_dn9,)
    }
};
        locals.var_dvfbpdep_op = assign43890_e49057;
        locals.var_dvfbpdep_op_dn4 = assign43890_e49057_d_n4;
        locals.var_dvfbpdep_op_dn6 = assign43890_e49057_d_n6;
        locals.var_dvfbpdep_op_dn7 = assign43890_e49057_d_n7;
        locals.var_dvfbpdep_op_dn8 = assign43890_e49057_d_n8;
        locals.var_dvfbpdep_op_dn9 = assign43890_e49057_d_n9;

        let assign43900_e49060: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1351 = assign43900_e49060;

        let assign43910_e49063: f64 = 1.0;
        let assign43910_e49064: f64 = if p.p14 == assign43910_e49063 { 1.0 } else { 0.0 };
        locals.var_guard1352 = assign43910_e49064;

        let (assign43920_e49083, assign43920_e49083_d_n4, assign43920_e49083_d_n6, assign43920_e49083_d_n7, assign43920_e49083_d_n8, assign43920_e49083_d_n9,) = {
    if ((locals.var_guard1351 != 0.0) && (locals.var_guard1352 != 0.0)) {
        let assign43920_e49070: f64 = (0.4 * p.p13);
        let assign43920_e49072: f64 = (assign43920_e49070 * 1.27520989);
        let assign43920_e49074: f64 = (-0.3333333333333);
        let assign43920_e49077: f64 = (locals.var_tsisq / locals.var_inv_phit_op);
        let assign43920_e49078: f64 = (assign43920_e49077).ln();
        let assign43920_e49079: f64 = (assign43920_e49074 * assign43920_e49078);
        let assign43920_e49080: f64 = (assign43920_e49079).exp();
        let assign43920_e49081: f64 = (assign43920_e49072 * assign43920_e49080);
        (assign43920_e49081, (assign43920_e49072 * (assign43920_e49080 * (assign43920_e49074 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn4) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign43920_e49077)))), (assign43920_e49072 * (assign43920_e49080 * (assign43920_e49074 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn6) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign43920_e49077)))), (assign43920_e49072 * (assign43920_e49080 * (assign43920_e49074 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn7) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign43920_e49077)))), (assign43920_e49072 * (assign43920_e49080 * (assign43920_e49074 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn8) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign43920_e49077)))), (assign43920_e49072 * (assign43920_e49080 * (assign43920_e49074 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn9) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign43920_e49077)))),)
    } else {
        (locals.var_qq_op, locals.var_qq_op_dn4, locals.var_qq_op_dn6, locals.var_qq_op_dn7, locals.var_qq_op_dn8, locals.var_qq_op_dn9,)
    }
};
        locals.var_qq_op = assign43920_e49083;
        locals.var_qq_op_dn4 = assign43920_e49083_d_n4;
        locals.var_qq_op_dn6 = assign43920_e49083_d_n6;
        locals.var_qq_op_dn7 = assign43920_e49083_d_n7;
        locals.var_qq_op_dn8 = assign43920_e49083_d_n8;
        locals.var_qq_op_dn9 = assign43920_e49083_d_n9;

        let (assign43930_e49103, assign43930_e49103_d_n4, assign43930_e49103_d_n6, assign43930_e49103_d_n7, assign43930_e49103_d_n8, assign43930_e49103_d_n9,) = {
    if ((locals.var_guard1351 != 0.0) && (locals.var_guard1352 == 0.0)) {
        let assign43930_e49090: f64 = (0.4 * p.p13);
        let assign43930_e49092: f64 = (assign43930_e49090 * 1.5412087);
        let assign43930_e49094: f64 = (-0.3333333333333);
        let assign43930_e49097: f64 = (locals.var_tsisq / locals.var_inv_phit_op);
        let assign43930_e49098: f64 = (assign43930_e49097).ln();
        let assign43930_e49099: f64 = (assign43930_e49094 * assign43930_e49098);
        let assign43930_e49100: f64 = (assign43930_e49099).exp();
        let assign43930_e49101: f64 = (assign43930_e49092 * assign43930_e49100);
        (assign43930_e49101, (assign43930_e49092 * (assign43930_e49100 * (assign43930_e49094 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn4) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign43930_e49097)))), (assign43930_e49092 * (assign43930_e49100 * (assign43930_e49094 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn6) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign43930_e49097)))), (assign43930_e49092 * (assign43930_e49100 * (assign43930_e49094 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn7) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign43930_e49097)))), (assign43930_e49092 * (assign43930_e49100 * (assign43930_e49094 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn8) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign43930_e49097)))), (assign43930_e49092 * (assign43930_e49100 * (assign43930_e49094 * ((-((locals.var_tsisq * locals.var_inv_phit_op_dn9) / (locals.var_inv_phit_op * locals.var_inv_phit_op))) / assign43930_e49097)))),)
    } else {
        (locals.var_qq_op, locals.var_qq_op_dn4, locals.var_qq_op_dn6, locals.var_qq_op_dn7, locals.var_qq_op_dn8, locals.var_qq_op_dn9,)
    }
};
        locals.var_qq_op = assign43930_e49103;
        locals.var_qq_op_dn4 = assign43930_e49103_d_n4;
        locals.var_qq_op_dn6 = assign43930_e49103_d_n6;
        locals.var_qq_op_dn7 = assign43930_e49103_d_n7;
        locals.var_qq_op_dn8 = assign43930_e49103_d_n8;
        locals.var_qq_op_dn9 = assign43930_e49103_d_n9;

        let assign43940_e49106: f64 = (locals.var_vds * locals.var_inv_phit_op);
        locals.var_xd_op = assign43940_e49106;
        locals.var_xd_op_dn4 = (locals.var_vds * locals.var_inv_phit_op_dn4);
        locals.var_xd_op_dn6 = ((locals.var_vds_dn6 * locals.var_inv_phit_op) + (locals.var_vds * locals.var_inv_phit_op_dn6));
        locals.var_xd_op_dn7 = ((locals.var_vds_dn7 * locals.var_inv_phit_op) + (locals.var_vds * locals.var_inv_phit_op_dn7));
        locals.var_xd_op_dn8 = (locals.var_vds * locals.var_inv_phit_op_dn8);
        locals.var_xd_op_dn9 = (locals.var_vds * locals.var_inv_phit_op_dn9);

        let assign43950_e49109: f64 = (locals.var_vds * locals.var_vds);
        let assign43950_e49111: f64 = (assign43950_e49109 + 0.01);
        let assign43950_e49112: f64 = (assign43950_e49111).sqrt();
        let assign43950_e49114: f64 = (assign43950_e49112 - 0.1);
        let assign43950_e49116: f64 = (assign43950_e49114 * locals.var_inv_phit_op);
        locals.var_xdsx_op = assign43950_e49116;
        locals.var_xdsx_op_dn4 = (assign43950_e49114 * locals.var_inv_phit_op_dn4);
        locals.var_xdsx_op_dn6 = (((((locals.var_vds_dn6 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn6)) / (2.0 * assign43950_e49112)) * locals.var_inv_phit_op) + (assign43950_e49114 * locals.var_inv_phit_op_dn6));
        locals.var_xdsx_op_dn7 = (((((locals.var_vds_dn7 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn7)) / (2.0 * assign43950_e49112)) * locals.var_inv_phit_op) + (assign43950_e49114 * locals.var_inv_phit_op_dn7));
        locals.var_xdsx_op_dn8 = (assign43950_e49114 * locals.var_inv_phit_op_dn8);
        locals.var_xdsx_op_dn9 = (assign43950_e49114 * locals.var_inv_phit_op_dn9);

        let assign43960_e49120: f64 = (locals.var_xd_op - locals.var_xdsx_op);
        let assign43960_e49121: f64 = (0.5 * assign43960_e49120);
        locals.var_dxdsx_op = assign43960_e49121;
        locals.var_dxdsx_op_dn4 = (0.5 * (locals.var_xd_op_dn4 - locals.var_xdsx_op_dn4));
        locals.var_dxdsx_op_dn6 = (0.5 * (locals.var_xd_op_dn6 - locals.var_xdsx_op_dn6));
        locals.var_dxdsx_op_dn7 = (0.5 * (locals.var_xd_op_dn7 - locals.var_xdsx_op_dn7));
        locals.var_dxdsx_op_dn8 = (0.5 * (locals.var_xd_op_dn8 - locals.var_xdsx_op_dn8));
        locals.var_dxdsx_op_dn9 = (0.5 * (locals.var_xd_op_dn9 - locals.var_xdsx_op_dn9));

        let assign43970_e49124: f64 = (locals.var_k2_dc / locals.var_k1_dc);
        let assign43970_e49127: f64 = (1.0 + locals.var_k2_dc);
        let assign43970_e49128: f64 = (assign43970_e49124 / assign43970_e49127);
        locals.var_r1init_op = assign43970_e49128;
        locals.var_r1init_op_dn4 = ((((((locals.var_k2_dc_dn4 * locals.var_k1_dc) - (locals.var_k2_dc * locals.var_k1_dc_dn4)) / (locals.var_k1_dc * locals.var_k1_dc)) * assign43970_e49127) - (assign43970_e49124 * locals.var_k2_dc_dn4)) / (assign43970_e49127 * assign43970_e49127));
        locals.var_r1init_op_dn6 = ((((((locals.var_k2_dc_dn6 * locals.var_k1_dc) - (locals.var_k2_dc * locals.var_k1_dc_dn6)) / (locals.var_k1_dc * locals.var_k1_dc)) * assign43970_e49127) - (assign43970_e49124 * locals.var_k2_dc_dn6)) / (assign43970_e49127 * assign43970_e49127));
        locals.var_r1init_op_dn7 = ((((((locals.var_k2_dc_dn7 * locals.var_k1_dc) - (locals.var_k2_dc * locals.var_k1_dc_dn7)) / (locals.var_k1_dc * locals.var_k1_dc)) * assign43970_e49127) - (assign43970_e49124 * locals.var_k2_dc_dn7)) / (assign43970_e49127 * assign43970_e49127));
        locals.var_r1init_op_dn8 = ((((((locals.var_k2_dc_dn8 * locals.var_k1_dc) - (locals.var_k2_dc * locals.var_k1_dc_dn8)) / (locals.var_k1_dc * locals.var_k1_dc)) * assign43970_e49127) - (assign43970_e49124 * locals.var_k2_dc_dn8)) / (assign43970_e49127 * assign43970_e49127));
        locals.var_r1init_op_dn9 = ((((((locals.var_k2_dc_dn9 * locals.var_k1_dc) - (locals.var_k2_dc * locals.var_k1_dc_dn9)) / (locals.var_k1_dc * locals.var_k1_dc)) * assign43970_e49127) - (assign43970_e49124 * locals.var_k2_dc_dn9)) / (assign43970_e49127 * assign43970_e49127));

        let assign43980_e49131: f64 = (locals.var_k1_dc / locals.var_k2_dc);
        let assign43980_e49134: f64 = (1.0 + locals.var_k1_dc);
        let assign43980_e49135: f64 = (assign43980_e49131 / assign43980_e49134);
        locals.var_r2init_op = assign43980_e49135;
        locals.var_r2init_op_dn4 = ((((((locals.var_k1_dc_dn4 * locals.var_k2_dc) - (locals.var_k1_dc * locals.var_k2_dc_dn4)) / (locals.var_k2_dc * locals.var_k2_dc)) * assign43980_e49134) - (assign43980_e49131 * locals.var_k1_dc_dn4)) / (assign43980_e49134 * assign43980_e49134));
        locals.var_r2init_op_dn6 = ((((((locals.var_k1_dc_dn6 * locals.var_k2_dc) - (locals.var_k1_dc * locals.var_k2_dc_dn6)) / (locals.var_k2_dc * locals.var_k2_dc)) * assign43980_e49134) - (assign43980_e49131 * locals.var_k1_dc_dn6)) / (assign43980_e49134 * assign43980_e49134));
        locals.var_r2init_op_dn7 = ((((((locals.var_k1_dc_dn7 * locals.var_k2_dc) - (locals.var_k1_dc * locals.var_k2_dc_dn7)) / (locals.var_k2_dc * locals.var_k2_dc)) * assign43980_e49134) - (assign43980_e49131 * locals.var_k1_dc_dn7)) / (assign43980_e49134 * assign43980_e49134));
        locals.var_r2init_op_dn8 = ((((((locals.var_k1_dc_dn8 * locals.var_k2_dc) - (locals.var_k1_dc * locals.var_k2_dc_dn8)) / (locals.var_k2_dc * locals.var_k2_dc)) * assign43980_e49134) - (assign43980_e49131 * locals.var_k1_dc_dn8)) / (assign43980_e49134 * assign43980_e49134));
        locals.var_r2init_op_dn9 = ((((((locals.var_k1_dc_dn9 * locals.var_k2_dc) - (locals.var_k1_dc * locals.var_k2_dc_dn9)) / (locals.var_k2_dc * locals.var_k2_dc)) * assign43980_e49134) - (assign43980_e49131 * locals.var_k1_dc_dn9)) / (assign43980_e49134 * assign43980_e49134));

        let assign43990_e49139: f64 = (1.0 + locals.var_r1init_op);
        let assign43990_e49140: f64 = (locals.var_k1_dc * assign43990_e49139);
        let assign43990_e49142: f64 = (assign43990_e49140 * locals.var_diff_min_dc);
        let assign43990_e49144: f64 = (assign43990_e49142 / locals.var_a0_dc);
        let assign43990_e49145: f64 = (assign43990_e49144).ln();
        let assign43990_e49147: f64 = (assign43990_e49145 + 2.0);
        locals.var_x1init_op = assign43990_e49147;
        locals.var_x1init_op_dn4 = ((((((((locals.var_k1_dc_dn4 * assign43990_e49139) + (locals.var_k1_dc * locals.var_r1init_op_dn4)) * locals.var_diff_min_dc) + (assign43990_e49140 * locals.var_diff_min_dc_dn4)) * locals.var_a0_dc) - (assign43990_e49142 * locals.var_a0_dc_dn4)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign43990_e49144);
        locals.var_x1init_op_dn6 = ((((((((locals.var_k1_dc_dn6 * assign43990_e49139) + (locals.var_k1_dc * locals.var_r1init_op_dn6)) * locals.var_diff_min_dc) + (assign43990_e49140 * locals.var_diff_min_dc_dn6)) * locals.var_a0_dc) - (assign43990_e49142 * locals.var_a0_dc_dn6)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign43990_e49144);
        locals.var_x1init_op_dn7 = ((((((((locals.var_k1_dc_dn7 * assign43990_e49139) + (locals.var_k1_dc * locals.var_r1init_op_dn7)) * locals.var_diff_min_dc) + (assign43990_e49140 * locals.var_diff_min_dc_dn7)) * locals.var_a0_dc) - (assign43990_e49142 * locals.var_a0_dc_dn7)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign43990_e49144);
        locals.var_x1init_op_dn8 = ((((((((locals.var_k1_dc_dn8 * assign43990_e49139) + (locals.var_k1_dc * locals.var_r1init_op_dn8)) * locals.var_diff_min_dc) + (assign43990_e49140 * locals.var_diff_min_dc_dn8)) * locals.var_a0_dc) - (assign43990_e49142 * locals.var_a0_dc_dn8)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign43990_e49144);
        locals.var_x1init_op_dn9 = ((((((((locals.var_k1_dc_dn9 * assign43990_e49139) + (locals.var_k1_dc * locals.var_r1init_op_dn9)) * locals.var_diff_min_dc) + (assign43990_e49140 * locals.var_diff_min_dc_dn9)) * locals.var_a0_dc) - (assign43990_e49142 * locals.var_a0_dc_dn9)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign43990_e49144);

        let assign44000_e49151: f64 = (1.0 + locals.var_r2init_op);
        let assign44000_e49152: f64 = (locals.var_k2_dc * assign44000_e49151);
        let assign44000_e49154: f64 = (assign44000_e49152 * locals.var_diff_min_dc);
        let assign44000_e49156: f64 = (assign44000_e49154 / locals.var_a0_dc);
        let assign44000_e49157: f64 = (assign44000_e49156).ln();
        let assign44000_e49159: f64 = (assign44000_e49157 + 2.0);
        locals.var_x2init_op = assign44000_e49159;
        locals.var_x2init_op_dn4 = ((((((((locals.var_k2_dc_dn4 * assign44000_e49151) + (locals.var_k2_dc * locals.var_r2init_op_dn4)) * locals.var_diff_min_dc) + (assign44000_e49152 * locals.var_diff_min_dc_dn4)) * locals.var_a0_dc) - (assign44000_e49154 * locals.var_a0_dc_dn4)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44000_e49156);
        locals.var_x2init_op_dn6 = ((((((((locals.var_k2_dc_dn6 * assign44000_e49151) + (locals.var_k2_dc * locals.var_r2init_op_dn6)) * locals.var_diff_min_dc) + (assign44000_e49152 * locals.var_diff_min_dc_dn6)) * locals.var_a0_dc) - (assign44000_e49154 * locals.var_a0_dc_dn6)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44000_e49156);
        locals.var_x2init_op_dn7 = ((((((((locals.var_k2_dc_dn7 * assign44000_e49151) + (locals.var_k2_dc * locals.var_r2init_op_dn7)) * locals.var_diff_min_dc) + (assign44000_e49152 * locals.var_diff_min_dc_dn7)) * locals.var_a0_dc) - (assign44000_e49154 * locals.var_a0_dc_dn7)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44000_e49156);
        locals.var_x2init_op_dn8 = ((((((((locals.var_k2_dc_dn8 * assign44000_e49151) + (locals.var_k2_dc * locals.var_r2init_op_dn8)) * locals.var_diff_min_dc) + (assign44000_e49152 * locals.var_diff_min_dc_dn8)) * locals.var_a0_dc) - (assign44000_e49154 * locals.var_a0_dc_dn8)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44000_e49156);
        locals.var_x2init_op_dn9 = ((((((((locals.var_k2_dc_dn9 * assign44000_e49151) + (locals.var_k2_dc * locals.var_r2init_op_dn9)) * locals.var_diff_min_dc) + (assign44000_e49152 * locals.var_diff_min_dc_dn9)) * locals.var_a0_dc) - (assign44000_e49154 * locals.var_a0_dc_dn9)) / (locals.var_a0_dc * locals.var_a0_dc)) / assign44000_e49156);

        let assign44010_e49162: f64 = (1.0 + locals.var_r1init_op);
        let assign44010_e49164: f64 = (assign44010_e49162 * locals.var_x1init_op);
        let assign44010_e49167: f64 = (locals.var_xg2x_dc * locals.var_r1init_op);
        let assign44010_e49168: f64 = (assign44010_e49164 - assign44010_e49167);
        locals.var_xth1init_op = assign44010_e49168;
        locals.var_xth1init_op_dn4 = (((locals.var_r1init_op_dn4 * locals.var_x1init_op) + (assign44010_e49162 * locals.var_x1init_op_dn4)) - ((locals.var_xg2x_dc_dn4 * locals.var_r1init_op) + (locals.var_xg2x_dc * locals.var_r1init_op_dn4)));
        locals.var_xth1init_op_dn6 = (((locals.var_r1init_op_dn6 * locals.var_x1init_op) + (assign44010_e49162 * locals.var_x1init_op_dn6)) - ((locals.var_xg2x_dc_dn6 * locals.var_r1init_op) + (locals.var_xg2x_dc * locals.var_r1init_op_dn6)));
        locals.var_xth1init_op_dn7 = (((locals.var_r1init_op_dn7 * locals.var_x1init_op) + (assign44010_e49162 * locals.var_x1init_op_dn7)) - ((locals.var_xg2x_dc_dn7 * locals.var_r1init_op) + (locals.var_xg2x_dc * locals.var_r1init_op_dn7)));
        locals.var_xth1init_op_dn8 = (((locals.var_r1init_op_dn8 * locals.var_x1init_op) + (assign44010_e49162 * locals.var_x1init_op_dn8)) - ((locals.var_xg2x_dc_dn8 * locals.var_r1init_op) + (locals.var_xg2x_dc * locals.var_r1init_op_dn8)));
        locals.var_xth1init_op_dn9 = (((locals.var_r1init_op_dn9 * locals.var_x1init_op) + (assign44010_e49162 * locals.var_x1init_op_dn9)) - ((locals.var_xg2x_dc_dn9 * locals.var_r1init_op) + (locals.var_xg2x_dc * locals.var_r1init_op_dn9)));

        let assign44020_e49172: f64 = (1.0 / locals.var_r2init_op);
        let assign44020_e49173: f64 = (1.0 + assign44020_e49172);
        let assign44020_e49175: f64 = (assign44020_e49173 * locals.var_x2init_op);
        let assign44020_e49178: f64 = (locals.var_xg2x_dc / locals.var_r2init_op);
        let assign44020_e49179: f64 = (assign44020_e49175 - assign44020_e49178);
        locals.var_xth2init_op = assign44020_e49179;
        locals.var_xth2init_op_dn4 = ((((-(locals.var_r2init_op_dn4 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44020_e49173 * locals.var_x2init_op_dn4)) - (((locals.var_xg2x_dc_dn4 * locals.var_r2init_op) - (locals.var_xg2x_dc * locals.var_r2init_op_dn4)) / (locals.var_r2init_op * locals.var_r2init_op)));
        locals.var_xth2init_op_dn6 = ((((-(locals.var_r2init_op_dn6 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44020_e49173 * locals.var_x2init_op_dn6)) - (((locals.var_xg2x_dc_dn6 * locals.var_r2init_op) - (locals.var_xg2x_dc * locals.var_r2init_op_dn6)) / (locals.var_r2init_op * locals.var_r2init_op)));
        locals.var_xth2init_op_dn7 = ((((-(locals.var_r2init_op_dn7 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44020_e49173 * locals.var_x2init_op_dn7)) - (((locals.var_xg2x_dc_dn7 * locals.var_r2init_op) - (locals.var_xg2x_dc * locals.var_r2init_op_dn7)) / (locals.var_r2init_op * locals.var_r2init_op)));
        locals.var_xth2init_op_dn8 = ((((-(locals.var_r2init_op_dn8 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44020_e49173 * locals.var_x2init_op_dn8)) - (((locals.var_xg2x_dc_dn8 * locals.var_r2init_op) - (locals.var_xg2x_dc * locals.var_r2init_op_dn8)) / (locals.var_r2init_op * locals.var_r2init_op)));
        locals.var_xth2init_op_dn9 = ((((-(locals.var_r2init_op_dn9 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44020_e49173 * locals.var_x2init_op_dn9)) - (((locals.var_xg2x_dc_dn9 * locals.var_r2init_op) - (locals.var_xg2x_dc * locals.var_r2init_op_dn9)) / (locals.var_r2init_op * locals.var_r2init_op)));

        let assign44030_e49183: f64 = (locals.var_xth1init_op + locals.var_xth2init_op);
        let assign44030_e49186: f64 = (locals.var_xth1init_op - locals.var_xth2init_op);
        let assign44030_e49189: f64 = (locals.var_xth1init_op - locals.var_xth2init_op);
        let assign44030_e49190: f64 = (assign44030_e49186 * assign44030_e49189);
        let assign44030_e49192: f64 = (assign44030_e49190 + 38.0);
        let assign44030_e49193: f64 = (assign44030_e49192).sqrt();
        let assign44030_e49194: f64 = (assign44030_e49183 - assign44030_e49193);
        let assign44030_e49195: f64 = (0.5 * assign44030_e49194);
        let assign44030_e49197: f64 = (assign44030_e49195 - locals.var_xg2_dc);
        let assign44030_e49199: f64 = (assign44030_e49197 / locals.var_cic1_i);
        let assign44030_e49201: f64 = (assign44030_e49199 + locals.var_xg2_dc);
        locals.var_xg1thinit_op = assign44030_e49201;
        locals.var_xg1thinit_op_dn4 = ((((0.5 * ((locals.var_xth1init_op_dn4 + locals.var_xth2init_op_dn4) - ((((locals.var_xth1init_op_dn4 - locals.var_xth2init_op_dn4) * assign44030_e49189) + (assign44030_e49186 * (locals.var_xth1init_op_dn4 - locals.var_xth2init_op_dn4))) / (2.0 * assign44030_e49193)))) - locals.var_xg2_dc_dn4) / locals.var_cic1_i) + locals.var_xg2_dc_dn4);
        locals.var_xg1thinit_op_dn6 = ((((0.5 * ((locals.var_xth1init_op_dn6 + locals.var_xth2init_op_dn6) - ((((locals.var_xth1init_op_dn6 - locals.var_xth2init_op_dn6) * assign44030_e49189) + (assign44030_e49186 * (locals.var_xth1init_op_dn6 - locals.var_xth2init_op_dn6))) / (2.0 * assign44030_e49193)))) - locals.var_xg2_dc_dn6) / locals.var_cic1_i) + locals.var_xg2_dc_dn6);
        locals.var_xg1thinit_op_dn7 = ((((0.5 * ((locals.var_xth1init_op_dn7 + locals.var_xth2init_op_dn7) - ((((locals.var_xth1init_op_dn7 - locals.var_xth2init_op_dn7) * assign44030_e49189) + (assign44030_e49186 * (locals.var_xth1init_op_dn7 - locals.var_xth2init_op_dn7))) / (2.0 * assign44030_e49193)))) - locals.var_xg2_dc_dn7) / locals.var_cic1_i) + locals.var_xg2_dc_dn7);
        locals.var_xg1thinit_op_dn8 = ((((0.5 * ((locals.var_xth1init_op_dn8 + locals.var_xth2init_op_dn8) - ((((locals.var_xth1init_op_dn8 - locals.var_xth2init_op_dn8) * assign44030_e49189) + (assign44030_e49186 * (locals.var_xth1init_op_dn8 - locals.var_xth2init_op_dn8))) / (2.0 * assign44030_e49193)))) - locals.var_xg2_dc_dn8) / locals.var_cic1_i) + locals.var_xg2_dc_dn8);
        locals.var_xg1thinit_op_dn9 = ((((0.5 * ((locals.var_xth1init_op_dn9 + locals.var_xth2init_op_dn9) - ((((locals.var_xth1init_op_dn9 - locals.var_xth2init_op_dn9) * assign44030_e49189) + (assign44030_e49186 * (locals.var_xth1init_op_dn9 - locals.var_xth2init_op_dn9))) / (2.0 * assign44030_e49193)))) - locals.var_xg2_dc_dn9) / locals.var_cic1_i) + locals.var_xg2_dc_dn9);

        let assign44040_e49205: f64 = (locals.var_xg1thinit_op - locals.var_xedge_dc);
        let assign44040_e49207: f64 = (assign44040_e49205 / locals.var_sce1_dc);
        let assign44040_e49209: f64 = (assign44040_e49207 - locals.var_dxg1_dibl_dc);
        let assign44040_e49211: f64 = (assign44040_e49209 + locals.var_xedge_dc);
        let assign44040_e49212: f64 = (locals.var_phit * assign44040_e49211);
        let assign44040_e49214: f64 = (assign44040_e49212 + locals.var_vfb1_i);
        locals.var_vthinit_op = assign44040_e49214;
        locals.var_vthinit_op_dn4 = (((locals.var_phit_dn4 * assign44040_e49211) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn4 - locals.var_xedge_dc_dn4) * locals.var_sce1_dc) - (assign44040_e49205 * locals.var_sce1_dc_dn4)) / (locals.var_sce1_dc * locals.var_sce1_dc)) - locals.var_dxg1_dibl_dc_dn4) + locals.var_xedge_dc_dn4))) + locals.var_vfb1_i_dn4);
        locals.var_vthinit_op_dn6 = (((locals.var_phit_dn6 * assign44040_e49211) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn6 - locals.var_xedge_dc_dn6) * locals.var_sce1_dc) - (assign44040_e49205 * locals.var_sce1_dc_dn6)) / (locals.var_sce1_dc * locals.var_sce1_dc)) - locals.var_dxg1_dibl_dc_dn6) + locals.var_xedge_dc_dn6))) + locals.var_vfb1_i_dn6);
        locals.var_vthinit_op_dn7 = (((locals.var_phit_dn7 * assign44040_e49211) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn7 - locals.var_xedge_dc_dn7) * locals.var_sce1_dc) - (assign44040_e49205 * locals.var_sce1_dc_dn7)) / (locals.var_sce1_dc * locals.var_sce1_dc)) - locals.var_dxg1_dibl_dc_dn7) + locals.var_xedge_dc_dn7))) + locals.var_vfb1_i_dn7);
        locals.var_vthinit_op_dn8 = (((locals.var_phit_dn8 * assign44040_e49211) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn8 - locals.var_xedge_dc_dn8) * locals.var_sce1_dc) - (assign44040_e49205 * locals.var_sce1_dc_dn8)) / (locals.var_sce1_dc * locals.var_sce1_dc)) - locals.var_dxg1_dibl_dc_dn8) + locals.var_xedge_dc_dn8))) + locals.var_vfb1_i_dn8);
        locals.var_vthinit_op_dn9 = (((locals.var_phit_dn9 * assign44040_e49211) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn9 - locals.var_xedge_dc_dn9) * locals.var_sce1_dc) - (assign44040_e49205 * locals.var_sce1_dc_dn9)) / (locals.var_sce1_dc * locals.var_sce1_dc)) - locals.var_dxg1_dibl_dc_dn9) + locals.var_xedge_dc_dn9))) + locals.var_vfb1_i_dn9);

        let assign44050_e49218: f64 = (locals.var_tkd - locals.var_tkr);
        let assign44050_e49219: f64 = (locals.var_stcf_i * assign44050_e49218);
        locals.var_temp = assign44050_e49219;
        locals.var_temp_dn4 = ((locals.var_stcf_i_dn4 * assign44050_e49218) + (locals.var_stcf_i * locals.var_tkd_dn4));
        locals.var_temp_dn6 = ((locals.var_stcf_i_dn6 * assign44050_e49218) + (locals.var_stcf_i * locals.var_tkd_dn6));
        locals.var_temp_dn7 = ((locals.var_stcf_i_dn7 * assign44050_e49218) + (locals.var_stcf_i * locals.var_tkd_dn7));
        locals.var_temp_dn8 = ((locals.var_stcf_i_dn8 * assign44050_e49218) + (locals.var_stcf_i * locals.var_tkd_dn8));
        locals.var_temp_dn9 = ((locals.var_stcf_i_dn9 * assign44050_e49218) + (locals.var_stcf_i * locals.var_tkd_dn9));

        let assign44080_e49228: f64 = (p.p14 * locals.var_stvfb_i);
        let assign44080_e49231: f64 = (locals.var_tkd - locals.var_tkr);
        let assign44080_e49232: f64 = (assign44080_e49228 * assign44080_e49231);
        let assign44080_e49234: f64 = (assign44080_e49232 + locals.var_dvfbqm);
        locals.var_temp = assign44080_e49234;
        locals.var_temp_dn4 = (assign44080_e49228 * locals.var_tkd_dn4);
        locals.var_temp_dn6 = (assign44080_e49228 * locals.var_tkd_dn6);
        locals.var_temp_dn7 = (assign44080_e49228 * locals.var_tkd_dn7);
        locals.var_temp_dn8 = (assign44080_e49228 * locals.var_tkd_dn8);
        locals.var_temp_dn9 = (assign44080_e49228 * locals.var_tkd_dn9);

        let assign44090_e49238: f64 = (locals.var_vfb1_t + locals.var_dvfbch_op);
        let assign44090_e49240: f64 = (assign44090_e49238 + locals.var_dvfb1nch);
        let assign44090_e49241: f64 = (p.p14 * assign44090_e49240);
        let assign44090_e49243: f64 = (assign44090_e49241 + locals.var_temp);
        let assign44090_e49245: f64 = (assign44090_e49243 + p.p34);
        let assign44090_e49247: f64 = (assign44090_e49245 - locals.var_dvfbpdep_op);
        locals.var_vfb1_op = assign44090_e49247;
        locals.var_vfb1_op_dn4 = (((p.p14 * ((locals.var_vfb1_t_dn4 + locals.var_dvfbch_op_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp_dn4) - locals.var_dvfbpdep_op_dn4);
        locals.var_vfb1_op_dn6 = (((p.p14 * ((locals.var_vfb1_t_dn6 + locals.var_dvfbch_op_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp_dn6) - locals.var_dvfbpdep_op_dn6);
        locals.var_vfb1_op_dn7 = (((p.p14 * ((locals.var_vfb1_t_dn7 + locals.var_dvfbch_op_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp_dn7) - locals.var_dvfbpdep_op_dn7);
        locals.var_vfb1_op_dn8 = (((p.p14 * ((locals.var_vfb1_t_dn8 + locals.var_dvfbch_op_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp_dn8) - locals.var_dvfbpdep_op_dn8);
        locals.var_vfb1_op_dn9 = (((p.p14 * ((locals.var_vfb1_t_dn9 + locals.var_dvfbch_op_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp_dn9) - locals.var_dvfbpdep_op_dn9);

        let assign44100_e49251: f64 = (locals.var_vfb2_t + locals.var_dvfbch_op);
        let assign44100_e49253: f64 = (assign44100_e49251 + locals.var_dvfb2nch);
        let assign44100_e49254: f64 = (p.p14 * assign44100_e49253);
        let assign44100_e49256: f64 = (assign44100_e49254 + locals.var_temp);
        locals.var_vfb2_op = assign44100_e49256;
        locals.var_vfb2_op_dn4 = ((p.p14 * ((locals.var_vfb2_t_dn4 + locals.var_dvfbch_op_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4);
        locals.var_vfb2_op_dn6 = ((p.p14 * ((locals.var_vfb2_t_dn6 + locals.var_dvfbch_op_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6);
        locals.var_vfb2_op_dn7 = ((p.p14 * ((locals.var_vfb2_t_dn7 + locals.var_dvfbch_op_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7);
        locals.var_vfb2_op_dn8 = ((p.p14 * ((locals.var_vfb2_t_dn8 + locals.var_dvfbch_op_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8);
        locals.var_vfb2_op_dn9 = ((p.p14 * ((locals.var_vfb2_t_dn9 + locals.var_dvfbch_op_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9);

        let assign44110_e49259: f64 = (locals.var_vthinit_op - locals.var_vfb1_op);
        let assign44110_e49261: f64 = (assign44110_e49259 * locals.var_inv_phit_op);
        let assign44110_e49263: f64 = (assign44110_e49261 - locals.var_dxdsx_op);
        locals.var_xg10_op = assign44110_e49263;
        locals.var_xg10_op_dn4 = ((((locals.var_vthinit_op_dn4 - locals.var_vfb1_op_dn4) * locals.var_inv_phit_op) + (assign44110_e49259 * locals.var_inv_phit_op_dn4)) - locals.var_dxdsx_op_dn4);
        locals.var_xg10_op_dn6 = ((((locals.var_vthinit_op_dn6 - locals.var_vfb1_op_dn6) * locals.var_inv_phit_op) + (assign44110_e49259 * locals.var_inv_phit_op_dn6)) - locals.var_dxdsx_op_dn6);
        locals.var_xg10_op_dn7 = ((((locals.var_vthinit_op_dn7 - locals.var_vfb1_op_dn7) * locals.var_inv_phit_op) + (assign44110_e49259 * locals.var_inv_phit_op_dn7)) - locals.var_dxdsx_op_dn7);
        locals.var_xg10_op_dn8 = ((((locals.var_vthinit_op_dn8 - locals.var_vfb1_op_dn8) * locals.var_inv_phit_op) + (assign44110_e49259 * locals.var_inv_phit_op_dn8)) - locals.var_dxdsx_op_dn8);
        locals.var_xg10_op_dn9 = ((((locals.var_vthinit_op_dn9 - locals.var_vfb1_op_dn9) * locals.var_inv_phit_op) + (assign44110_e49259 * locals.var_inv_phit_op_dn9)) - locals.var_dxdsx_op_dn9);

        let assign44120_e49265: f64 = (-locals.var_vsb);
        let assign44120_e49267: f64 = (assign44120_e49265 - locals.var_vfb2_op);
        let assign44120_e49269: f64 = (assign44120_e49267 * locals.var_inv_phit_op);
        let assign44120_e49271: f64 = (assign44120_e49269 - locals.var_dxdsx_op);
        locals.var_xg20_op = assign44120_e49271;
        locals.var_xg20_op_dn4 = ((((-locals.var_vfb2_op_dn4) * locals.var_inv_phit_op) + (assign44120_e49267 * locals.var_inv_phit_op_dn4)) - locals.var_dxdsx_op_dn4);
        locals.var_xg20_op_dn6 = (((((-locals.var_vsb_dn6) - locals.var_vfb2_op_dn6) * locals.var_inv_phit_op) + (assign44120_e49267 * locals.var_inv_phit_op_dn6)) - locals.var_dxdsx_op_dn6);
        locals.var_xg20_op_dn7 = (((((-locals.var_vsb_dn7) - locals.var_vfb2_op_dn7) * locals.var_inv_phit_op) + (assign44120_e49267 * locals.var_inv_phit_op_dn7)) - locals.var_dxdsx_op_dn7);
        locals.var_xg20_op_dn8 = (((((-locals.var_vsb_dn8) - locals.var_vfb2_op_dn8) * locals.var_inv_phit_op) + (assign44120_e49267 * locals.var_inv_phit_op_dn8)) - locals.var_dxdsx_op_dn8);
        locals.var_xg20_op_dn9 = ((((-locals.var_vfb2_op_dn9) * locals.var_inv_phit_op) + (assign44120_e49267 * locals.var_inv_phit_op_dn9)) - locals.var_dxdsx_op_dn9);

        let assign44130_e49274: f64 = if p.p2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1353 = assign44130_e49274;

        let (assign44140_e49286, assign44140_e49286_d_n4, assign44140_e49286_d_n6, assign44140_e49286_d_n7, assign44140_e49286_d_n8, assign44140_e49286_d_n9,) = {
    if (locals.var_guard1353 != 0.0) {
        let assign44140_e49278: f64 = (p.p14 * locals.var_typesub_i);
        let assign44140_e49281: f64 = (locals.var_xg10_op - locals.var_xg20_op);
        let assign44140_e49282: f64 = (assign44140_e49278 * assign44140_e49281);
        let assign44140_e49284: f64 = (assign44140_e49282 / locals.var_gfsub);
        (assign44140_e49284, ((((assign44140_e49278 * (locals.var_xg10_op_dn4 - locals.var_xg20_op_dn4)) * locals.var_gfsub) - (assign44140_e49282 * locals.var_gfsub_dn4)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44140_e49278 * (locals.var_xg10_op_dn6 - locals.var_xg20_op_dn6)) * locals.var_gfsub) - (assign44140_e49282 * locals.var_gfsub_dn6)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44140_e49278 * (locals.var_xg10_op_dn7 - locals.var_xg20_op_dn7)) * locals.var_gfsub) - (assign44140_e49282 * locals.var_gfsub_dn7)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44140_e49278 * (locals.var_xg10_op_dn8 - locals.var_xg20_op_dn8)) * locals.var_gfsub) - (assign44140_e49282 * locals.var_gfsub_dn8)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44140_e49278 * (locals.var_xg10_op_dn9 - locals.var_xg20_op_dn9)) * locals.var_gfsub) - (assign44140_e49282 * locals.var_gfsub_dn9)) / (locals.var_gfsub * locals.var_gfsub)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign44140_e49286;
        locals.var_temp_dn4 = assign44140_e49286_d_n4;
        locals.var_temp_dn6 = assign44140_e49286_d_n6;
        locals.var_temp_dn7 = assign44140_e49286_d_n7;
        locals.var_temp_dn8 = assign44140_e49286_d_n8;
        locals.var_temp_dn9 = assign44140_e49286_d_n9;

        let assign44150_e49289: f64 = if locals.var_temp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1354 = assign44150_e49289;

        let (assign44160_e49301, assign44160_e49301_d_n4, assign44160_e49301_d_n6, assign44160_e49301_d_n7, assign44160_e49301_d_n8, assign44160_e49301_d_n9,) = {
    if ((locals.var_guard1353 != 0.0) && (locals.var_guard1354 != 0.0)) {
        let assign44160_e49294: f64 = (-2.0);
        let assign44160_e49297: f64 = (1.0 - locals.var_temp);
        let assign44160_e49298: f64 = (assign44160_e49297).ln();
        let assign44160_e49299: f64 = (assign44160_e49294 * assign44160_e49298);
        (assign44160_e49299, (assign44160_e49294 * ((-locals.var_temp_dn4) / assign44160_e49297)), (assign44160_e49294 * ((-locals.var_temp_dn6) / assign44160_e49297)), (assign44160_e49294 * ((-locals.var_temp_dn7) / assign44160_e49297)), (assign44160_e49294 * ((-locals.var_temp_dn8) / assign44160_e49297)), (assign44160_e49294 * ((-locals.var_temp_dn9) / assign44160_e49297)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign44160_e49301;
        locals.var_temp1_dn4 = assign44160_e49301_d_n4;
        locals.var_temp1_dn6 = assign44160_e49301_d_n6;
        locals.var_temp1_dn7 = assign44160_e49301_d_n7;
        locals.var_temp1_dn8 = assign44160_e49301_d_n8;
        locals.var_temp1_dn9 = assign44160_e49301_d_n9;

        let (assign44170_e49318, assign44170_e49318_d_n4, assign44170_e49318_d_n6, assign44170_e49318_d_n7, assign44170_e49318_d_n8, assign44170_e49318_d_n9,) = {
    if ((locals.var_guard1353 != 0.0) && (locals.var_guard1354 == 0.0)) {
        let assign44170_e49308: f64 = (locals.var_temp * locals.var_temp);
        let assign44170_e49312: f64 = (2.0 * locals.var_temp);
        let assign44170_e49314: f64 = (assign44170_e49312 / locals.var_gfsub);
        let assign44170_e49315: f64 = (1.0 + assign44170_e49314);
        let assign44170_e49316: f64 = (assign44170_e49308 / assign44170_e49315);
        (assign44170_e49316, (((((locals.var_temp_dn4 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn4)) * assign44170_e49315) - (assign44170_e49308 * ((((2.0 * locals.var_temp_dn4) * locals.var_gfsub) - (assign44170_e49312 * locals.var_gfsub_dn4)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44170_e49315 * assign44170_e49315)), (((((locals.var_temp_dn6 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn6)) * assign44170_e49315) - (assign44170_e49308 * ((((2.0 * locals.var_temp_dn6) * locals.var_gfsub) - (assign44170_e49312 * locals.var_gfsub_dn6)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44170_e49315 * assign44170_e49315)), (((((locals.var_temp_dn7 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn7)) * assign44170_e49315) - (assign44170_e49308 * ((((2.0 * locals.var_temp_dn7) * locals.var_gfsub) - (assign44170_e49312 * locals.var_gfsub_dn7)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44170_e49315 * assign44170_e49315)), (((((locals.var_temp_dn8 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn8)) * assign44170_e49315) - (assign44170_e49308 * ((((2.0 * locals.var_temp_dn8) * locals.var_gfsub) - (assign44170_e49312 * locals.var_gfsub_dn8)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44170_e49315 * assign44170_e49315)), (((((locals.var_temp_dn9 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn9)) * assign44170_e49315) - (assign44170_e49308 * ((((2.0 * locals.var_temp_dn9) * locals.var_gfsub) - (assign44170_e49312 * locals.var_gfsub_dn9)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44170_e49315 * assign44170_e49315)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign44170_e49318;
        locals.var_temp1_dn4 = assign44170_e49318_d_n4;
        locals.var_temp1_dn6 = assign44170_e49318_d_n6;
        locals.var_temp1_dn7 = assign44170_e49318_d_n7;
        locals.var_temp1_dn8 = assign44170_e49318_d_n8;
        locals.var_temp1_dn9 = assign44170_e49318_d_n9;

        let (assign44180_e49328, assign44180_e49328_d_n4, assign44180_e49328_d_n6, assign44180_e49328_d_n7, assign44180_e49328_d_n8, assign44180_e49328_d_n9,) = {
    if (locals.var_guard1353 != 0.0) {
        let assign44180_e49323: f64 = (p.p14 * locals.var_typesub_i);
        let assign44180_e49325: f64 = (assign44180_e49323 * locals.var_temp1);
        let assign44180_e49326: f64 = (locals.var_xg20_op + assign44180_e49325);
        (assign44180_e49326, (locals.var_xg20_op_dn4 + (assign44180_e49323 * locals.var_temp1_dn4)), (locals.var_xg20_op_dn6 + (assign44180_e49323 * locals.var_temp1_dn6)), (locals.var_xg20_op_dn7 + (assign44180_e49323 * locals.var_temp1_dn7)), (locals.var_xg20_op_dn8 + (assign44180_e49323 * locals.var_temp1_dn8)), (locals.var_xg20_op_dn9 + (assign44180_e49323 * locals.var_temp1_dn9)),)
    } else {
        (locals.var_xg2eff_op, locals.var_xg2eff_op_dn4, locals.var_xg2eff_op_dn6, locals.var_xg2eff_op_dn7, locals.var_xg2eff_op_dn8, locals.var_xg2eff_op_dn9,)
    }
};
        locals.var_xg2eff_op = assign44180_e49328;
        locals.var_xg2eff_op_dn4 = assign44180_e49328_d_n4;
        locals.var_xg2eff_op_dn6 = assign44180_e49328_d_n6;
        locals.var_xg2eff_op_dn7 = assign44180_e49328_d_n7;
        locals.var_xg2eff_op_dn8 = assign44180_e49328_d_n8;
        locals.var_xg2eff_op_dn9 = assign44180_e49328_d_n9;

        let (assign44190_e49333, assign44190_e49333_d_n4, assign44190_e49333_d_n6, assign44190_e49333_d_n7, assign44190_e49333_d_n8, assign44190_e49333_d_n9,) = {
    if (locals.var_guard1353 == 0.0) {
        (locals.var_xg20_op, locals.var_xg20_op_dn4, locals.var_xg20_op_dn6, locals.var_xg20_op_dn7, locals.var_xg20_op_dn8, locals.var_xg20_op_dn9,)
    } else {
        (locals.var_xg2eff_op, locals.var_xg2eff_op_dn4, locals.var_xg2eff_op_dn6, locals.var_xg2eff_op_dn7, locals.var_xg2eff_op_dn8, locals.var_xg2eff_op_dn9,)
    }
};
        locals.var_xg2eff_op = assign44190_e49333;
        locals.var_xg2eff_op_dn4 = assign44190_e49333_d_n4;
        locals.var_xg2eff_op_dn6 = assign44190_e49333_d_n6;
        locals.var_xg2eff_op_dn7 = assign44190_e49333_d_n7;
        locals.var_xg2eff_op_dn8 = assign44190_e49333_d_n8;
        locals.var_xg2eff_op_dn9 = assign44190_e49333_d_n9;

        let assign44200_e49337: f64 = (locals.var_xg10_op - locals.var_xg2eff_op);
        let assign44200_e49338: f64 = (locals.var_keq_1d * assign44200_e49337);
        locals.var_temp = assign44200_e49338;
        locals.var_temp_dn4 = (locals.var_keq_1d * (locals.var_xg10_op_dn4 - locals.var_xg2eff_op_dn4));
        locals.var_temp_dn6 = (locals.var_keq_1d * (locals.var_xg10_op_dn6 - locals.var_xg2eff_op_dn6));
        locals.var_temp_dn7 = (locals.var_keq_1d * (locals.var_xg10_op_dn7 - locals.var_xg2eff_op_dn7));
        locals.var_temp_dn8 = (locals.var_keq_1d * (locals.var_xg10_op_dn8 - locals.var_xg2eff_op_dn8));
        locals.var_temp_dn9 = (locals.var_keq_1d * (locals.var_xg10_op_dn9 - locals.var_xg2eff_op_dn9));

        let assign44210_e49341: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1355 = assign44210_e49341;

        let (assign44220_e49362, assign44220_e49362_d_n4, assign44220_e49362_d_n6, assign44220_e49362_d_n7, assign44220_e49362_d_n8, assign44220_e49362_d_n9,) = {
    if (locals.var_guard1355 != 0.0) {
        let assign44220_e49346: f64 = (locals.var_temp + locals.var_emin);
        let assign44220_e49349: f64 = (locals.var_temp - locals.var_emin);
        let assign44220_e49352: f64 = (locals.var_temp - locals.var_emin);
        let assign44220_e49353: f64 = (assign44220_e49349 * assign44220_e49352);
        let assign44220_e49356: f64 = (locals.var_emin * locals.var_emin);
        let assign44220_e49357: f64 = (assign44220_e49353 + assign44220_e49356);
        let assign44220_e49358: f64 = (assign44220_e49357).sqrt();
        let assign44220_e49359: f64 = (assign44220_e49346 + assign44220_e49358);
        let assign44220_e49360: f64 = (0.5 * assign44220_e49359);
        (assign44220_e49360, (0.5 * ((locals.var_temp_dn4 + locals.var_emin_dn4) + (((((locals.var_temp_dn4 - locals.var_emin_dn4) * assign44220_e49352) + (assign44220_e49349 * (locals.var_temp_dn4 - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign44220_e49358)))), (0.5 * ((locals.var_temp_dn6 + locals.var_emin_dn6) + (((((locals.var_temp_dn6 - locals.var_emin_dn6) * assign44220_e49352) + (assign44220_e49349 * (locals.var_temp_dn6 - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign44220_e49358)))), (0.5 * ((locals.var_temp_dn7 + locals.var_emin_dn7) + (((((locals.var_temp_dn7 - locals.var_emin_dn7) * assign44220_e49352) + (assign44220_e49349 * (locals.var_temp_dn7 - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign44220_e49358)))), (0.5 * ((locals.var_temp_dn8 + locals.var_emin_dn8) + (((((locals.var_temp_dn8 - locals.var_emin_dn8) * assign44220_e49352) + (assign44220_e49349 * (locals.var_temp_dn8 - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign44220_e49358)))), (0.5 * ((locals.var_temp_dn9 + locals.var_emin_dn9) + (((((locals.var_temp_dn9 - locals.var_emin_dn9) * assign44220_e49352) + (assign44220_e49349 * (locals.var_temp_dn9 - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign44220_e49358)))),)
    } else {
        (locals.var_e1_op, locals.var_e1_op_dn4, locals.var_e1_op_dn6, locals.var_e1_op_dn7, locals.var_e1_op_dn8, locals.var_e1_op_dn9,)
    }
};
        locals.var_e1_op = assign44220_e49362;
        locals.var_e1_op_dn4 = assign44220_e49362_d_n4;
        locals.var_e1_op_dn6 = assign44220_e49362_d_n6;
        locals.var_e1_op_dn7 = assign44220_e49362_d_n7;
        locals.var_e1_op_dn8 = assign44220_e49362_d_n8;
        locals.var_e1_op_dn9 = assign44220_e49362_d_n9;

    }

    pub(super) fn stamp_transient_block_119(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign44230_e49386, assign44230_e49386_d_n4, assign44230_e49386_d_n6, assign44230_e49386_d_n7, assign44230_e49386_d_n8, assign44230_e49386_d_n9,) = {
    if (locals.var_guard1355 != 0.0) {
        let assign44230_e49366: f64 = (-locals.var_temp);
        let assign44230_e49368: f64 = (assign44230_e49366 + locals.var_emin);
        let assign44230_e49370: f64 = (-locals.var_temp);
        let assign44230_e49372: f64 = (assign44230_e49370 - locals.var_emin);
        let assign44230_e49374: f64 = (-locals.var_temp);
        let assign44230_e49376: f64 = (assign44230_e49374 - locals.var_emin);
        let assign44230_e49377: f64 = (assign44230_e49372 * assign44230_e49376);
        let assign44230_e49380: f64 = (locals.var_emin * locals.var_emin);
        let assign44230_e49381: f64 = (assign44230_e49377 + assign44230_e49380);
        let assign44230_e49382: f64 = (assign44230_e49381).sqrt();
        let assign44230_e49383: f64 = (assign44230_e49368 + assign44230_e49382);
        let assign44230_e49384: f64 = (0.5 * assign44230_e49383);
        (assign44230_e49384, (0.5 * (((-locals.var_temp_dn4) + locals.var_emin_dn4) + ((((((-locals.var_temp_dn4) - locals.var_emin_dn4) * assign44230_e49376) + (assign44230_e49372 * ((-locals.var_temp_dn4) - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign44230_e49382)))), (0.5 * (((-locals.var_temp_dn6) + locals.var_emin_dn6) + ((((((-locals.var_temp_dn6) - locals.var_emin_dn6) * assign44230_e49376) + (assign44230_e49372 * ((-locals.var_temp_dn6) - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign44230_e49382)))), (0.5 * (((-locals.var_temp_dn7) + locals.var_emin_dn7) + ((((((-locals.var_temp_dn7) - locals.var_emin_dn7) * assign44230_e49376) + (assign44230_e49372 * ((-locals.var_temp_dn7) - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign44230_e49382)))), (0.5 * (((-locals.var_temp_dn8) + locals.var_emin_dn8) + ((((((-locals.var_temp_dn8) - locals.var_emin_dn8) * assign44230_e49376) + (assign44230_e49372 * ((-locals.var_temp_dn8) - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign44230_e49382)))), (0.5 * (((-locals.var_temp_dn9) + locals.var_emin_dn9) + ((((((-locals.var_temp_dn9) - locals.var_emin_dn9) * assign44230_e49376) + (assign44230_e49372 * ((-locals.var_temp_dn9) - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign44230_e49382)))),)
    } else {
        (locals.var_e2_op, locals.var_e2_op_dn4, locals.var_e2_op_dn6, locals.var_e2_op_dn7, locals.var_e2_op_dn8, locals.var_e2_op_dn9,)
    }
};
        locals.var_e2_op = assign44230_e49386;
        locals.var_e2_op_dn4 = assign44230_e49386_d_n4;
        locals.var_e2_op_dn6 = assign44230_e49386_d_n6;
        locals.var_e2_op_dn7 = assign44230_e49386_d_n7;
        locals.var_e2_op_dn8 = assign44230_e49386_d_n8;
        locals.var_e2_op_dn9 = assign44230_e49386_d_n9;

        let (assign44240_e49397, assign44240_e49397_d_n4, assign44240_e49397_d_n6, assign44240_e49397_d_n7, assign44240_e49397_d_n8, assign44240_e49397_d_n9,) = {
    if (locals.var_guard1355 != 0.0) {
        let assign44240_e49390: f64 = (-0.3333333333333);
        let assign44240_e49392: f64 = (locals.var_e1_op).ln();
        let assign44240_e49393: f64 = (assign44240_e49390 * assign44240_e49392);
        let assign44240_e49394: f64 = (assign44240_e49393).exp();
        let assign44240_e49395: f64 = (locals.var_qq_op * assign44240_e49394);
        (assign44240_e49395, ((locals.var_qq_op_dn4 * assign44240_e49394) + (locals.var_qq_op * (assign44240_e49394 * (assign44240_e49390 * (locals.var_e1_op_dn4 / locals.var_e1_op))))), ((locals.var_qq_op_dn6 * assign44240_e49394) + (locals.var_qq_op * (assign44240_e49394 * (assign44240_e49390 * (locals.var_e1_op_dn6 / locals.var_e1_op))))), ((locals.var_qq_op_dn7 * assign44240_e49394) + (locals.var_qq_op * (assign44240_e49394 * (assign44240_e49390 * (locals.var_e1_op_dn7 / locals.var_e1_op))))), ((locals.var_qq_op_dn8 * assign44240_e49394) + (locals.var_qq_op * (assign44240_e49394 * (assign44240_e49390 * (locals.var_e1_op_dn8 / locals.var_e1_op))))), ((locals.var_qq_op_dn9 * assign44240_e49394) + (locals.var_qq_op * (assign44240_e49394 * (assign44240_e49390 * (locals.var_e1_op_dn9 / locals.var_e1_op))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign44240_e49397;
        locals.var_temp1_dn4 = assign44240_e49397_d_n4;
        locals.var_temp1_dn6 = assign44240_e49397_d_n6;
        locals.var_temp1_dn7 = assign44240_e49397_d_n7;
        locals.var_temp1_dn8 = assign44240_e49397_d_n8;
        locals.var_temp1_dn9 = assign44240_e49397_d_n9;

        let (assign44250_e49408, assign44250_e49408_d_n4, assign44250_e49408_d_n6, assign44250_e49408_d_n7, assign44250_e49408_d_n8, assign44250_e49408_d_n9,) = {
    if (locals.var_guard1355 != 0.0) {
        let assign44250_e49401: f64 = (-0.3333333333333);
        let assign44250_e49403: f64 = (locals.var_e2_op).ln();
        let assign44250_e49404: f64 = (assign44250_e49401 * assign44250_e49403);
        let assign44250_e49405: f64 = (assign44250_e49404).exp();
        let assign44250_e49406: f64 = (locals.var_qq_op * assign44250_e49405);
        (assign44250_e49406, ((locals.var_qq_op_dn4 * assign44250_e49405) + (locals.var_qq_op * (assign44250_e49405 * (assign44250_e49401 * (locals.var_e2_op_dn4 / locals.var_e2_op))))), ((locals.var_qq_op_dn6 * assign44250_e49405) + (locals.var_qq_op * (assign44250_e49405 * (assign44250_e49401 * (locals.var_e2_op_dn6 / locals.var_e2_op))))), ((locals.var_qq_op_dn7 * assign44250_e49405) + (locals.var_qq_op * (assign44250_e49405 * (assign44250_e49401 * (locals.var_e2_op_dn7 / locals.var_e2_op))))), ((locals.var_qq_op_dn8 * assign44250_e49405) + (locals.var_qq_op * (assign44250_e49405 * (assign44250_e49401 * (locals.var_e2_op_dn8 / locals.var_e2_op))))), ((locals.var_qq_op_dn9 * assign44250_e49405) + (locals.var_qq_op * (assign44250_e49405 * (assign44250_e49401 * (locals.var_e2_op_dn9 / locals.var_e2_op))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign44250_e49408;
        locals.var_temp2_dn4 = assign44250_e49408_d_n4;
        locals.var_temp2_dn6 = assign44250_e49408_d_n6;
        locals.var_temp2_dn7 = assign44250_e49408_d_n7;
        locals.var_temp2_dn8 = assign44250_e49408_d_n8;
        locals.var_temp2_dn9 = assign44250_e49408_d_n9;

        let (assign44260_e49416, assign44260_e49416_d_n4, assign44260_e49416_d_n6, assign44260_e49416_d_n7, assign44260_e49416_d_n8, assign44260_e49416_d_n9,) = {
    if (locals.var_guard1355 != 0.0) {
        let assign44260_e49412: f64 = (1.0 - locals.var_temp1);
        let assign44260_e49414: f64 = (assign44260_e49412 - locals.var_temp2);
        (assign44260_e49414, ((-locals.var_temp1_dn4) - locals.var_temp2_dn4), ((-locals.var_temp1_dn6) - locals.var_temp2_dn6), ((-locals.var_temp1_dn7) - locals.var_temp2_dn7), ((-locals.var_temp1_dn8) - locals.var_temp2_dn8), ((-locals.var_temp1_dn9) - locals.var_temp2_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign44260_e49416;
        locals.var_temp3_dn4 = assign44260_e49416_d_n4;
        locals.var_temp3_dn6 = assign44260_e49416_d_n6;
        locals.var_temp3_dn7 = assign44260_e49416_d_n7;
        locals.var_temp3_dn8 = assign44260_e49416_d_n8;
        locals.var_temp3_dn9 = assign44260_e49416_d_n9;

        let (assign44280_e49434, assign44280_e49434_d_n4, assign44280_e49434_d_n6, assign44280_e49434_d_n7, assign44280_e49434_d_n8, assign44280_e49434_d_n9,) = {
    if (locals.var_guard1355 != 0.0) {
        let assign44280_e49426: f64 = (locals.var_k1_1d * locals.var_temp3);
        let assign44280_e49430: f64 = (locals.var_k1_1d * locals.var_temp1);
        let assign44280_e49431: f64 = (1.0 + assign44280_e49430);
        let assign44280_e49432: f64 = (assign44280_e49426 / assign44280_e49431);
        (assign44280_e49432, ((((locals.var_k1_1d * locals.var_temp3_dn4) * assign44280_e49431) - (assign44280_e49426 * (locals.var_k1_1d * locals.var_temp1_dn4))) / (assign44280_e49431 * assign44280_e49431)), ((((locals.var_k1_1d * locals.var_temp3_dn6) * assign44280_e49431) - (assign44280_e49426 * (locals.var_k1_1d * locals.var_temp1_dn6))) / (assign44280_e49431 * assign44280_e49431)), ((((locals.var_k1_1d * locals.var_temp3_dn7) * assign44280_e49431) - (assign44280_e49426 * (locals.var_k1_1d * locals.var_temp1_dn7))) / (assign44280_e49431 * assign44280_e49431)), ((((locals.var_k1_1d * locals.var_temp3_dn8) * assign44280_e49431) - (assign44280_e49426 * (locals.var_k1_1d * locals.var_temp1_dn8))) / (assign44280_e49431 * assign44280_e49431)), ((((locals.var_k1_1d * locals.var_temp3_dn9) * assign44280_e49431) - (assign44280_e49426 * (locals.var_k1_1d * locals.var_temp1_dn9))) / (assign44280_e49431 * assign44280_e49431)),)
    } else {
        (locals.var_k1_1d_qm_op, locals.var_k1_1d_qm_op_dn4, locals.var_k1_1d_qm_op_dn6, locals.var_k1_1d_qm_op_dn7, locals.var_k1_1d_qm_op_dn8, locals.var_k1_1d_qm_op_dn9,)
    }
};
        locals.var_k1_1d_qm_op = assign44280_e49434;
        locals.var_k1_1d_qm_op_dn4 = assign44280_e49434_d_n4;
        locals.var_k1_1d_qm_op_dn6 = assign44280_e49434_d_n6;
        locals.var_k1_1d_qm_op_dn7 = assign44280_e49434_d_n7;
        locals.var_k1_1d_qm_op_dn8 = assign44280_e49434_d_n8;
        locals.var_k1_1d_qm_op_dn9 = assign44280_e49434_d_n9;

        let (assign44290_e49446, assign44290_e49446_d_n4, assign44290_e49446_d_n6, assign44290_e49446_d_n7, assign44290_e49446_d_n8, assign44290_e49446_d_n9,) = {
    if (locals.var_guard1355 != 0.0) {
        let assign44290_e49438: f64 = (locals.var_k2_1d * locals.var_temp3);
        let assign44290_e49442: f64 = (locals.var_k2_1d * locals.var_temp2);
        let assign44290_e49443: f64 = (1.0 + assign44290_e49442);
        let assign44290_e49444: f64 = (assign44290_e49438 / assign44290_e49443);
        (assign44290_e49444, ((((locals.var_k2_1d * locals.var_temp3_dn4) * assign44290_e49443) - (assign44290_e49438 * (locals.var_k2_1d * locals.var_temp2_dn4))) / (assign44290_e49443 * assign44290_e49443)), ((((locals.var_k2_1d * locals.var_temp3_dn6) * assign44290_e49443) - (assign44290_e49438 * (locals.var_k2_1d * locals.var_temp2_dn6))) / (assign44290_e49443 * assign44290_e49443)), ((((locals.var_k2_1d * locals.var_temp3_dn7) * assign44290_e49443) - (assign44290_e49438 * (locals.var_k2_1d * locals.var_temp2_dn7))) / (assign44290_e49443 * assign44290_e49443)), ((((locals.var_k2_1d * locals.var_temp3_dn8) * assign44290_e49443) - (assign44290_e49438 * (locals.var_k2_1d * locals.var_temp2_dn8))) / (assign44290_e49443 * assign44290_e49443)), ((((locals.var_k2_1d * locals.var_temp3_dn9) * assign44290_e49443) - (assign44290_e49438 * (locals.var_k2_1d * locals.var_temp2_dn9))) / (assign44290_e49443 * assign44290_e49443)),)
    } else {
        (locals.var_k2_1d_qm_op, locals.var_k2_1d_qm_op_dn4, locals.var_k2_1d_qm_op_dn6, locals.var_k2_1d_qm_op_dn7, locals.var_k2_1d_qm_op_dn8, locals.var_k2_1d_qm_op_dn9,)
    }
};
        locals.var_k2_1d_qm_op = assign44290_e49446;
        locals.var_k2_1d_qm_op_dn4 = assign44290_e49446_d_n4;
        locals.var_k2_1d_qm_op_dn6 = assign44290_e49446_d_n6;
        locals.var_k2_1d_qm_op_dn7 = assign44290_e49446_d_n7;
        locals.var_k2_1d_qm_op_dn8 = assign44290_e49446_d_n8;
        locals.var_k2_1d_qm_op_dn9 = assign44290_e49446_d_n9;

        let (assign44300_e49460, assign44300_e49460_d_n4, assign44300_e49460_d_n6, assign44300_e49460_d_n7, assign44300_e49460_d_n8, assign44300_e49460_d_n9,) = {
    if (locals.var_guard1355 != 0.0) {
        let assign44300_e49452: f64 = (1.0 / locals.var_k1_1d_qm_op);
        let assign44300_e49453: f64 = (1.0 + assign44300_e49452);
        let assign44300_e49456: f64 = (1.0 / locals.var_k2_1d_qm_op);
        let assign44300_e49457: f64 = (assign44300_e49453 + assign44300_e49456);
        let assign44300_e49458: f64 = (1.0 / assign44300_e49457);
        (assign44300_e49458, (-(((-(locals.var_k1_1d_qm_op_dn4 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn4 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign44300_e49457 * assign44300_e49457))), (-(((-(locals.var_k1_1d_qm_op_dn6 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn6 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign44300_e49457 * assign44300_e49457))), (-(((-(locals.var_k1_1d_qm_op_dn7 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn7 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign44300_e49457 * assign44300_e49457))), (-(((-(locals.var_k1_1d_qm_op_dn8 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn8 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign44300_e49457 * assign44300_e49457))), (-(((-(locals.var_k1_1d_qm_op_dn9 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn9 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign44300_e49457 * assign44300_e49457))),)
    } else {
        (locals.var_keq_1d_qm_op, locals.var_keq_1d_qm_op_dn4, locals.var_keq_1d_qm_op_dn6, locals.var_keq_1d_qm_op_dn7, locals.var_keq_1d_qm_op_dn8, locals.var_keq_1d_qm_op_dn9,)
    }
};
        locals.var_keq_1d_qm_op = assign44300_e49460;
        locals.var_keq_1d_qm_op_dn4 = assign44300_e49460_d_n4;
        locals.var_keq_1d_qm_op_dn6 = assign44300_e49460_d_n6;
        locals.var_keq_1d_qm_op_dn7 = assign44300_e49460_d_n7;
        locals.var_keq_1d_qm_op_dn8 = assign44300_e49460_d_n8;
        locals.var_keq_1d_qm_op_dn9 = assign44300_e49460_d_n9;

        let (assign44320_e49470, assign44320_e49470_d_n4, assign44320_e49470_d_n6, assign44320_e49470_d_n7, assign44320_e49470_d_n8, assign44320_e49470_d_n9,) = {
    if (locals.var_guard1355 == 0.0) {
        (locals.var_k1_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_k1_1d_qm_op, locals.var_k1_1d_qm_op_dn4, locals.var_k1_1d_qm_op_dn6, locals.var_k1_1d_qm_op_dn7, locals.var_k1_1d_qm_op_dn8, locals.var_k1_1d_qm_op_dn9,)
    }
};
        locals.var_k1_1d_qm_op = assign44320_e49470;
        locals.var_k1_1d_qm_op_dn4 = assign44320_e49470_d_n4;
        locals.var_k1_1d_qm_op_dn6 = assign44320_e49470_d_n6;
        locals.var_k1_1d_qm_op_dn7 = assign44320_e49470_d_n7;
        locals.var_k1_1d_qm_op_dn8 = assign44320_e49470_d_n8;
        locals.var_k1_1d_qm_op_dn9 = assign44320_e49470_d_n9;

        let (assign44330_e49475, assign44330_e49475_d_n4, assign44330_e49475_d_n6, assign44330_e49475_d_n7, assign44330_e49475_d_n8, assign44330_e49475_d_n9,) = {
    if (locals.var_guard1355 == 0.0) {
        (locals.var_k2_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_k2_1d_qm_op, locals.var_k2_1d_qm_op_dn4, locals.var_k2_1d_qm_op_dn6, locals.var_k2_1d_qm_op_dn7, locals.var_k2_1d_qm_op_dn8, locals.var_k2_1d_qm_op_dn9,)
    }
};
        locals.var_k2_1d_qm_op = assign44330_e49475;
        locals.var_k2_1d_qm_op_dn4 = assign44330_e49475_d_n4;
        locals.var_k2_1d_qm_op_dn6 = assign44330_e49475_d_n6;
        locals.var_k2_1d_qm_op_dn7 = assign44330_e49475_d_n7;
        locals.var_k2_1d_qm_op_dn8 = assign44330_e49475_d_n8;
        locals.var_k2_1d_qm_op_dn9 = assign44330_e49475_d_n9;

        let (assign44340_e49480, assign44340_e49480_d_n4, assign44340_e49480_d_n6, assign44340_e49480_d_n7, assign44340_e49480_d_n8, assign44340_e49480_d_n9,) = {
    if (locals.var_guard1355 == 0.0) {
        (locals.var_keq_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_keq_1d_qm_op, locals.var_keq_1d_qm_op_dn4, locals.var_keq_1d_qm_op_dn6, locals.var_keq_1d_qm_op_dn7, locals.var_keq_1d_qm_op_dn8, locals.var_keq_1d_qm_op_dn9,)
    }
};
        locals.var_keq_1d_qm_op = assign44340_e49480;
        locals.var_keq_1d_qm_op_dn4 = assign44340_e49480_d_n4;
        locals.var_keq_1d_qm_op_dn6 = assign44340_e49480_d_n6;
        locals.var_keq_1d_qm_op_dn7 = assign44340_e49480_d_n7;
        locals.var_keq_1d_qm_op_dn8 = assign44340_e49480_d_n8;
        locals.var_keq_1d_qm_op_dn9 = assign44340_e49480_d_n9;

        let assign44350_e49484: f64 = (locals.var_xg10_op - locals.var_xg2eff_op);
        let assign44350_e49485: f64 = (locals.var_keq_1d_qm_op * assign44350_e49484);
        locals.var_dx_wi_1d_op = assign44350_e49485;
        locals.var_dx_wi_1d_op_dn4 = ((locals.var_keq_1d_qm_op_dn4 * assign44350_e49484) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn4 - locals.var_xg2eff_op_dn4)));
        locals.var_dx_wi_1d_op_dn6 = ((locals.var_keq_1d_qm_op_dn6 * assign44350_e49484) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn6 - locals.var_xg2eff_op_dn6)));
        locals.var_dx_wi_1d_op_dn7 = ((locals.var_keq_1d_qm_op_dn7 * assign44350_e49484) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn7 - locals.var_xg2eff_op_dn7)));
        locals.var_dx_wi_1d_op_dn8 = ((locals.var_keq_1d_qm_op_dn8 * assign44350_e49484) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn8 - locals.var_xg2eff_op_dn8)));
        locals.var_dx_wi_1d_op_dn9 = ((locals.var_keq_1d_qm_op_dn9 * assign44350_e49484) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn9 - locals.var_xg2eff_op_dn9)));

        let assign44360_e49488: f64 = if locals.var_dx_wi_1d_op > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1356 = assign44360_e49488;

        let assign44370_e49490: f64 = (-locals.var_dx_wi_1d_op);
        let assign44370_e49492: f64 = if assign44370_e49490 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1357 = assign44370_e49492;

        let (assign44380_e49503, assign44380_e49503_d_n4, assign44380_e49503_d_n6, assign44380_e49503_d_n7, assign44380_e49503_d_n8, assign44380_e49503_d_n9,) = {
    if ((locals.var_guard1356 != 0.0) && (locals.var_guard1357 != 0.0)) {
        let assign44380_e49498: f64 = (-locals.var_dx_wi_1d_op);
        let assign44380_e49499: f64 = (assign44380_e49498).exp();
        let assign44380_e49500: f64 = (1.0 + assign44380_e49499);
        let assign44380_e49501: f64 = (assign44380_e49500).ln();
        (assign44380_e49501, ((assign44380_e49499 * (-locals.var_dx_wi_1d_op_dn4)) / assign44380_e49500), ((assign44380_e49499 * (-locals.var_dx_wi_1d_op_dn6)) / assign44380_e49500), ((assign44380_e49499 * (-locals.var_dx_wi_1d_op_dn7)) / assign44380_e49500), ((assign44380_e49499 * (-locals.var_dx_wi_1d_op_dn8)) / assign44380_e49500), ((assign44380_e49499 * (-locals.var_dx_wi_1d_op_dn9)) / assign44380_e49500),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign44380_e49503;
        locals.var_temp_dn4 = assign44380_e49503_d_n4;
        locals.var_temp_dn6 = assign44380_e49503_d_n6;
        locals.var_temp_dn7 = assign44380_e49503_d_n7;
        locals.var_temp_dn8 = assign44380_e49503_d_n8;
        locals.var_temp_dn9 = assign44380_e49503_d_n9;

        let (assign44390_e49511, assign44390_e49511_d_n4, assign44390_e49511_d_n6, assign44390_e49511_d_n7, assign44390_e49511_d_n8, assign44390_e49511_d_n9,) = {
    if ((locals.var_guard1356 != 0.0) && (locals.var_guard1357 == 0.0)) {
        let assign44390_e49509: f64 = (-locals.var_dx_wi_1d_op);
        (assign44390_e49509, (-locals.var_dx_wi_1d_op_dn4), (-locals.var_dx_wi_1d_op_dn6), (-locals.var_dx_wi_1d_op_dn7), (-locals.var_dx_wi_1d_op_dn8), (-locals.var_dx_wi_1d_op_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign44390_e49511;
        locals.var_temp_dn4 = assign44390_e49511_d_n4;
        locals.var_temp_dn6 = assign44390_e49511_d_n6;
        locals.var_temp_dn7 = assign44390_e49511_d_n7;
        locals.var_temp_dn8 = assign44390_e49511_d_n8;
        locals.var_temp_dn9 = assign44390_e49511_d_n9;

        let (assign44400_e49523, assign44400_e49523_d_n4, assign44400_e49523_d_n6, assign44400_e49523_d_n7, assign44400_e49523_d_n8, assign44400_e49523_d_n9,) = {
    if (locals.var_guard1356 != 0.0) {
        let assign44400_e49516: f64 = (locals.var_dx_wi_1d_op / locals.var_k1_1d_qm_op);
        let assign44400_e49517: f64 = (locals.var_xg10_op - assign44400_e49516);
        let assign44400_e49519: f64 = (assign44400_e49517 + locals.var_temp);
        let assign44400_e49521: f64 = (assign44400_e49519 - 0.6931471805599);
        (assign44400_e49521, ((locals.var_xg10_op_dn4 - (((locals.var_dx_wi_1d_op_dn4 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn4)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn4), ((locals.var_xg10_op_dn6 - (((locals.var_dx_wi_1d_op_dn6 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn6)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn6), ((locals.var_xg10_op_dn7 - (((locals.var_dx_wi_1d_op_dn7 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn7)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn7), ((locals.var_xg10_op_dn8 - (((locals.var_dx_wi_1d_op_dn8 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn8)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn8), ((locals.var_xg10_op_dn9 - (((locals.var_dx_wi_1d_op_dn9 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn9)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn9),)
    } else {
        (locals.var_x_wi_1d_op, locals.var_x_wi_1d_op_dn4, locals.var_x_wi_1d_op_dn6, locals.var_x_wi_1d_op_dn7, locals.var_x_wi_1d_op_dn8, locals.var_x_wi_1d_op_dn9,)
    }
};
        locals.var_x_wi_1d_op = assign44400_e49523;
        locals.var_x_wi_1d_op_dn4 = assign44400_e49523_d_n4;
        locals.var_x_wi_1d_op_dn6 = assign44400_e49523_d_n6;
        locals.var_x_wi_1d_op_dn7 = assign44400_e49523_d_n7;
        locals.var_x_wi_1d_op_dn8 = assign44400_e49523_d_n8;
        locals.var_x_wi_1d_op_dn9 = assign44400_e49523_d_n9;

        let assign44410_e49526: f64 = if locals.var_dx_wi_1d_op < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1358 = assign44410_e49526;

        let (assign44420_e49537, assign44420_e49537_d_n4, assign44420_e49537_d_n6, assign44420_e49537_d_n7, assign44420_e49537_d_n8, assign44420_e49537_d_n9,) = {
    if ((locals.var_guard1356 == 0.0) && (locals.var_guard1358 != 0.0)) {
        let assign44420_e49533: f64 = (locals.var_dx_wi_1d_op).exp();
        let assign44420_e49534: f64 = (1.0 + assign44420_e49533);
        let assign44420_e49535: f64 = (assign44420_e49534).ln();
        (assign44420_e49535, ((assign44420_e49533 * locals.var_dx_wi_1d_op_dn4) / assign44420_e49534), ((assign44420_e49533 * locals.var_dx_wi_1d_op_dn6) / assign44420_e49534), ((assign44420_e49533 * locals.var_dx_wi_1d_op_dn7) / assign44420_e49534), ((assign44420_e49533 * locals.var_dx_wi_1d_op_dn8) / assign44420_e49534), ((assign44420_e49533 * locals.var_dx_wi_1d_op_dn9) / assign44420_e49534),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign44420_e49537;
        locals.var_temp_dn4 = assign44420_e49537_d_n4;
        locals.var_temp_dn6 = assign44420_e49537_d_n6;
        locals.var_temp_dn7 = assign44420_e49537_d_n7;
        locals.var_temp_dn8 = assign44420_e49537_d_n8;
        locals.var_temp_dn9 = assign44420_e49537_d_n9;

        let (assign44430_e49545, assign44430_e49545_d_n4, assign44430_e49545_d_n6, assign44430_e49545_d_n7, assign44430_e49545_d_n8, assign44430_e49545_d_n9,) = {
    if ((locals.var_guard1356 == 0.0) && (locals.var_guard1358 == 0.0)) {
        (locals.var_dx_wi_1d_op, locals.var_dx_wi_1d_op_dn4, locals.var_dx_wi_1d_op_dn6, locals.var_dx_wi_1d_op_dn7, locals.var_dx_wi_1d_op_dn8, locals.var_dx_wi_1d_op_dn9,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign44430_e49545;
        locals.var_temp_dn4 = assign44430_e49545_d_n4;
        locals.var_temp_dn6 = assign44430_e49545_d_n6;
        locals.var_temp_dn7 = assign44430_e49545_d_n7;
        locals.var_temp_dn8 = assign44430_e49545_d_n8;
        locals.var_temp_dn9 = assign44430_e49545_d_n9;

        let (assign44440_e49558, assign44440_e49558_d_n4, assign44440_e49558_d_n6, assign44440_e49558_d_n7, assign44440_e49558_d_n8, assign44440_e49558_d_n9,) = {
    if (locals.var_guard1356 == 0.0) {
        let assign44440_e49551: f64 = (locals.var_dx_wi_1d_op / locals.var_k2_1d_qm_op);
        let assign44440_e49552: f64 = (locals.var_xg2eff_op + assign44440_e49551);
        let assign44440_e49554: f64 = (assign44440_e49552 + locals.var_temp);
        let assign44440_e49556: f64 = (assign44440_e49554 - 0.6931471805599);
        (assign44440_e49556, ((locals.var_xg2eff_op_dn4 + (((locals.var_dx_wi_1d_op_dn4 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn4)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn4), ((locals.var_xg2eff_op_dn6 + (((locals.var_dx_wi_1d_op_dn6 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn6)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn6), ((locals.var_xg2eff_op_dn7 + (((locals.var_dx_wi_1d_op_dn7 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn7)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn7), ((locals.var_xg2eff_op_dn8 + (((locals.var_dx_wi_1d_op_dn8 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn8)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn8), ((locals.var_xg2eff_op_dn9 + (((locals.var_dx_wi_1d_op_dn9 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn9)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn9),)
    } else {
        (locals.var_x_wi_1d_op, locals.var_x_wi_1d_op_dn4, locals.var_x_wi_1d_op_dn6, locals.var_x_wi_1d_op_dn7, locals.var_x_wi_1d_op_dn8, locals.var_x_wi_1d_op_dn9,)
    }
};
        locals.var_x_wi_1d_op = assign44440_e49558;
        locals.var_x_wi_1d_op_dn4 = assign44440_e49558_d_n4;
        locals.var_x_wi_1d_op_dn6 = assign44440_e49558_d_n6;
        locals.var_x_wi_1d_op_dn7 = assign44440_e49558_d_n7;
        locals.var_x_wi_1d_op_dn8 = assign44440_e49558_d_n8;
        locals.var_x_wi_1d_op_dn9 = assign44440_e49558_d_n9;

        let assign44450_e49562: f64 = (locals.var_x_wi_1d_op + locals.var_xth_1d_op);
        let assign44450_e49565: f64 = (locals.var_x_wi_1d_op - locals.var_xth_1d_op);
        let assign44450_e49568: f64 = (locals.var_x_wi_1d_op - locals.var_xth_1d_op);
        let assign44450_e49569: f64 = (assign44450_e49565 * assign44450_e49568);
        let assign44450_e49571: f64 = (assign44450_e49569 + 4.0);
        let assign44450_e49572: f64 = (assign44450_e49571).sqrt();
        let assign44450_e49573: f64 = (assign44450_e49562 - assign44450_e49572);
        let assign44450_e49574: f64 = (0.5 * assign44450_e49573);
        locals.var_x_1d_op = assign44450_e49574;
        locals.var_x_1d_op_dn4 = (0.5 * ((locals.var_x_wi_1d_op_dn4 + locals.var_xth_1d_op_dn4) - ((((locals.var_x_wi_1d_op_dn4 - locals.var_xth_1d_op_dn4) * assign44450_e49568) + (assign44450_e49565 * (locals.var_x_wi_1d_op_dn4 - locals.var_xth_1d_op_dn4))) / (2.0 * assign44450_e49572))));
        locals.var_x_1d_op_dn6 = (0.5 * ((locals.var_x_wi_1d_op_dn6 + locals.var_xth_1d_op_dn6) - ((((locals.var_x_wi_1d_op_dn6 - locals.var_xth_1d_op_dn6) * assign44450_e49568) + (assign44450_e49565 * (locals.var_x_wi_1d_op_dn6 - locals.var_xth_1d_op_dn6))) / (2.0 * assign44450_e49572))));
        locals.var_x_1d_op_dn7 = (0.5 * ((locals.var_x_wi_1d_op_dn7 + locals.var_xth_1d_op_dn7) - ((((locals.var_x_wi_1d_op_dn7 - locals.var_xth_1d_op_dn7) * assign44450_e49568) + (assign44450_e49565 * (locals.var_x_wi_1d_op_dn7 - locals.var_xth_1d_op_dn7))) / (2.0 * assign44450_e49572))));
        locals.var_x_1d_op_dn8 = (0.5 * ((locals.var_x_wi_1d_op_dn8 + locals.var_xth_1d_op_dn8) - ((((locals.var_x_wi_1d_op_dn8 - locals.var_xth_1d_op_dn8) * assign44450_e49568) + (assign44450_e49565 * (locals.var_x_wi_1d_op_dn8 - locals.var_xth_1d_op_dn8))) / (2.0 * assign44450_e49572))));
        locals.var_x_1d_op_dn9 = (0.5 * ((locals.var_x_wi_1d_op_dn9 + locals.var_xth_1d_op_dn9) - ((((locals.var_x_wi_1d_op_dn9 - locals.var_xth_1d_op_dn9) * assign44450_e49568) + (assign44450_e49565 * (locals.var_x_wi_1d_op_dn9 - locals.var_xth_1d_op_dn9))) / (2.0 * assign44450_e49572))));

        let assign44460_e49579: f64 = (locals.var_xth_1d_op - locals.var_x_1d_op);
        let assign44460_e49580: f64 = (2.0 * assign44460_e49579);
        let assign44460_e49582: f64 = (assign44460_e49580 / locals.var_xsddep_op);
        let assign44460_e49583: f64 = (1.0 + assign44460_e49582);
        let assign44460_e49584: f64 = (assign44460_e49583).sqrt();
        let assign44460_e49586: f64 = (assign44460_e49584 - 1.0);
        locals.var_dleff_op = assign44460_e49586;
        locals.var_dleff_op_dn4 = (((((2.0 * (locals.var_xth_1d_op_dn4 - locals.var_x_1d_op_dn4)) * locals.var_xsddep_op) - (assign44460_e49580 * locals.var_xsddep_op_dn4)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign44460_e49584));
        locals.var_dleff_op_dn6 = (((((2.0 * (locals.var_xth_1d_op_dn6 - locals.var_x_1d_op_dn6)) * locals.var_xsddep_op) - (assign44460_e49580 * locals.var_xsddep_op_dn6)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign44460_e49584));
        locals.var_dleff_op_dn7 = (((((2.0 * (locals.var_xth_1d_op_dn7 - locals.var_x_1d_op_dn7)) * locals.var_xsddep_op) - (assign44460_e49580 * locals.var_xsddep_op_dn7)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign44460_e49584));
        locals.var_dleff_op_dn8 = (((((2.0 * (locals.var_xth_1d_op_dn8 - locals.var_x_1d_op_dn8)) * locals.var_xsddep_op) - (assign44460_e49580 * locals.var_xsddep_op_dn8)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign44460_e49584));
        locals.var_dleff_op_dn9 = (((((2.0 * (locals.var_xth_1d_op_dn9 - locals.var_x_1d_op_dn9)) * locals.var_xsddep_op) - (assign44460_e49580 * locals.var_xsddep_op_dn9)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign44460_e49584));

        let assign44480_e49596: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
        let assign44480_e49597: f64 = (1.0 + assign44480_e49596);
        let assign44480_e49599: f64 = (assign44480_e49597 + 0.5);
        let assign44480_e49603: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
        let assign44480_e49604: f64 = (1.0 + assign44480_e49603);
        let assign44480_e49606: f64 = (assign44480_e49604 - 0.5);
        let assign44480_e49610: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
        let assign44480_e49611: f64 = (1.0 + assign44480_e49610);
        let assign44480_e49613: f64 = (assign44480_e49611 - 0.5);
        let assign44480_e49614: f64 = (assign44480_e49606 * assign44480_e49613);
        let assign44480_e49616: f64 = (assign44480_e49614 + 0.01);
        let assign44480_e49617: f64 = (assign44480_e49616).sqrt();
        let assign44480_e49618: f64 = (assign44480_e49599 + assign44480_e49617);
        let assign44480_e49619: f64 = (0.5 * assign44480_e49618);
        locals.var_temp = assign44480_e49619;
        locals.var_temp_dn4 = (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn4) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn4) * assign44480_e49613) + (assign44480_e49606 * (locals.var_pscedlb_i * locals.var_xg20_op_dn4))) / (2.0 * assign44480_e49617))));
        locals.var_temp_dn6 = (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn6) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn6) * assign44480_e49613) + (assign44480_e49606 * (locals.var_pscedlb_i * locals.var_xg20_op_dn6))) / (2.0 * assign44480_e49617))));
        locals.var_temp_dn7 = (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn7) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn7) * assign44480_e49613) + (assign44480_e49606 * (locals.var_pscedlb_i * locals.var_xg20_op_dn7))) / (2.0 * assign44480_e49617))));
        locals.var_temp_dn8 = (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn8) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn8) * assign44480_e49613) + (assign44480_e49606 * (locals.var_pscedlb_i * locals.var_xg20_op_dn8))) / (2.0 * assign44480_e49617))));
        locals.var_temp_dn9 = (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn9) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn9) * assign44480_e49613) + (assign44480_e49606 * (locals.var_pscedlb_i * locals.var_xg20_op_dn9))) / (2.0 * assign44480_e49617))));

        let assign44510_e49636: f64 = (2.0 * locals.var_xd0_op);
        let assign44510_e49640: f64 = (locals.var_xdsx_op / locals.var_xd0_op);
        let assign44510_e49641: f64 = (1.0 + assign44510_e49640);
        let assign44510_e49642: f64 = (assign44510_e49641).sqrt();
        let assign44510_e49644: f64 = (assign44510_e49642 - 1.0);
        let assign44510_e49645: f64 = (assign44510_e49636 * assign44510_e49644);
        let assign44510_e49649: f64 = (locals.var_cfdl_i * locals.var_dleff_op);
        let assign44510_e49650: f64 = (1.0 + assign44510_e49649);
        let assign44510_e49651: f64 = (assign44510_e49645 * assign44510_e49650);
        let assign44510_e49655: f64 = (locals.var_cfdlb_i * locals.var_xg20_op);
        let assign44510_e49656: f64 = (1.0 + assign44510_e49655);
        let assign44510_e49657: f64 = (assign44510_e49651 * assign44510_e49656);
        locals.var_temp = assign44510_e49657;
        locals.var_temp_dn4 = (((((((2.0 * locals.var_xd0_op_dn4) * assign44510_e49644) + (assign44510_e49636 * ((((locals.var_xdsx_op_dn4 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn4)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign44510_e49642)))) * assign44510_e49650) + (assign44510_e49645 * (locals.var_cfdl_i * locals.var_dleff_op_dn4))) * assign44510_e49656) + (assign44510_e49651 * (locals.var_cfdlb_i * locals.var_xg20_op_dn4)));
        locals.var_temp_dn6 = (((((((2.0 * locals.var_xd0_op_dn6) * assign44510_e49644) + (assign44510_e49636 * ((((locals.var_xdsx_op_dn6 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn6)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign44510_e49642)))) * assign44510_e49650) + (assign44510_e49645 * (locals.var_cfdl_i * locals.var_dleff_op_dn6))) * assign44510_e49656) + (assign44510_e49651 * (locals.var_cfdlb_i * locals.var_xg20_op_dn6)));
        locals.var_temp_dn7 = (((((((2.0 * locals.var_xd0_op_dn7) * assign44510_e49644) + (assign44510_e49636 * ((((locals.var_xdsx_op_dn7 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn7)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign44510_e49642)))) * assign44510_e49650) + (assign44510_e49645 * (locals.var_cfdl_i * locals.var_dleff_op_dn7))) * assign44510_e49656) + (assign44510_e49651 * (locals.var_cfdlb_i * locals.var_xg20_op_dn7)));
        locals.var_temp_dn8 = (((((((2.0 * locals.var_xd0_op_dn8) * assign44510_e49644) + (assign44510_e49636 * ((((locals.var_xdsx_op_dn8 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn8)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign44510_e49642)))) * assign44510_e49650) + (assign44510_e49645 * (locals.var_cfdl_i * locals.var_dleff_op_dn8))) * assign44510_e49656) + (assign44510_e49651 * (locals.var_cfdlb_i * locals.var_xg20_op_dn8)));
        locals.var_temp_dn9 = (((((((2.0 * locals.var_xd0_op_dn9) * assign44510_e49644) + (assign44510_e49636 * ((((locals.var_xdsx_op_dn9 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn9)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign44510_e49642)))) * assign44510_e49650) + (assign44510_e49645 * (locals.var_cfdl_i * locals.var_dleff_op_dn9))) * assign44510_e49656) + (assign44510_e49651 * (locals.var_cfdlb_i * locals.var_xg20_op_dn9)));

        let assign44750_e49840: f64 = if p.p11 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1360 = assign44750_e49840;

        let (assign44760_e49850, assign44760_e49850_d_n4, assign44760_e49850_d_n6, assign44760_e49850_d_n7, assign44760_e49850_d_n8, assign44760_e49850_d_n9,) = {
    if (locals.var_guard1360 != 0.0) {
        let assign44760_e49844: f64 = (locals.var_k2_ac / locals.var_k1_ac);
        let assign44760_e49847: f64 = (1.0 + locals.var_k2_ac);
        let assign44760_e49848: f64 = (assign44760_e49844 / assign44760_e49847);
        (assign44760_e49848, ((((((locals.var_k2_ac_dn4 * locals.var_k1_ac) - (locals.var_k2_ac * locals.var_k1_ac_dn4)) / (locals.var_k1_ac * locals.var_k1_ac)) * assign44760_e49847) - (assign44760_e49844 * locals.var_k2_ac_dn4)) / (assign44760_e49847 * assign44760_e49847)), ((((((locals.var_k2_ac_dn6 * locals.var_k1_ac) - (locals.var_k2_ac * locals.var_k1_ac_dn6)) / (locals.var_k1_ac * locals.var_k1_ac)) * assign44760_e49847) - (assign44760_e49844 * locals.var_k2_ac_dn6)) / (assign44760_e49847 * assign44760_e49847)), ((((((locals.var_k2_ac_dn7 * locals.var_k1_ac) - (locals.var_k2_ac * locals.var_k1_ac_dn7)) / (locals.var_k1_ac * locals.var_k1_ac)) * assign44760_e49847) - (assign44760_e49844 * locals.var_k2_ac_dn7)) / (assign44760_e49847 * assign44760_e49847)), ((((((locals.var_k2_ac_dn8 * locals.var_k1_ac) - (locals.var_k2_ac * locals.var_k1_ac_dn8)) / (locals.var_k1_ac * locals.var_k1_ac)) * assign44760_e49847) - (assign44760_e49844 * locals.var_k2_ac_dn8)) / (assign44760_e49847 * assign44760_e49847)), ((((((locals.var_k2_ac_dn9 * locals.var_k1_ac) - (locals.var_k2_ac * locals.var_k1_ac_dn9)) / (locals.var_k1_ac * locals.var_k1_ac)) * assign44760_e49847) - (assign44760_e49844 * locals.var_k2_ac_dn9)) / (assign44760_e49847 * assign44760_e49847)),)
    } else {
        (locals.var_r1init_op, locals.var_r1init_op_dn4, locals.var_r1init_op_dn6, locals.var_r1init_op_dn7, locals.var_r1init_op_dn8, locals.var_r1init_op_dn9,)
    }
};
        locals.var_r1init_op = assign44760_e49850;
        locals.var_r1init_op_dn4 = assign44760_e49850_d_n4;
        locals.var_r1init_op_dn6 = assign44760_e49850_d_n6;
        locals.var_r1init_op_dn7 = assign44760_e49850_d_n7;
        locals.var_r1init_op_dn8 = assign44760_e49850_d_n8;
        locals.var_r1init_op_dn9 = assign44760_e49850_d_n9;

        let (assign44770_e49860, assign44770_e49860_d_n4, assign44770_e49860_d_n6, assign44770_e49860_d_n7, assign44770_e49860_d_n8, assign44770_e49860_d_n9,) = {
    if (locals.var_guard1360 != 0.0) {
        let assign44770_e49854: f64 = (locals.var_k1_ac / locals.var_k2_ac);
        let assign44770_e49857: f64 = (1.0 + locals.var_k1_ac);
        let assign44770_e49858: f64 = (assign44770_e49854 / assign44770_e49857);
        (assign44770_e49858, ((((((locals.var_k1_ac_dn4 * locals.var_k2_ac) - (locals.var_k1_ac * locals.var_k2_ac_dn4)) / (locals.var_k2_ac * locals.var_k2_ac)) * assign44770_e49857) - (assign44770_e49854 * locals.var_k1_ac_dn4)) / (assign44770_e49857 * assign44770_e49857)), ((((((locals.var_k1_ac_dn6 * locals.var_k2_ac) - (locals.var_k1_ac * locals.var_k2_ac_dn6)) / (locals.var_k2_ac * locals.var_k2_ac)) * assign44770_e49857) - (assign44770_e49854 * locals.var_k1_ac_dn6)) / (assign44770_e49857 * assign44770_e49857)), ((((((locals.var_k1_ac_dn7 * locals.var_k2_ac) - (locals.var_k1_ac * locals.var_k2_ac_dn7)) / (locals.var_k2_ac * locals.var_k2_ac)) * assign44770_e49857) - (assign44770_e49854 * locals.var_k1_ac_dn7)) / (assign44770_e49857 * assign44770_e49857)), ((((((locals.var_k1_ac_dn8 * locals.var_k2_ac) - (locals.var_k1_ac * locals.var_k2_ac_dn8)) / (locals.var_k2_ac * locals.var_k2_ac)) * assign44770_e49857) - (assign44770_e49854 * locals.var_k1_ac_dn8)) / (assign44770_e49857 * assign44770_e49857)), ((((((locals.var_k1_ac_dn9 * locals.var_k2_ac) - (locals.var_k1_ac * locals.var_k2_ac_dn9)) / (locals.var_k2_ac * locals.var_k2_ac)) * assign44770_e49857) - (assign44770_e49854 * locals.var_k1_ac_dn9)) / (assign44770_e49857 * assign44770_e49857)),)
    } else {
        (locals.var_r2init_op, locals.var_r2init_op_dn4, locals.var_r2init_op_dn6, locals.var_r2init_op_dn7, locals.var_r2init_op_dn8, locals.var_r2init_op_dn9,)
    }
};
        locals.var_r2init_op = assign44770_e49860;
        locals.var_r2init_op_dn4 = assign44770_e49860_d_n4;
        locals.var_r2init_op_dn6 = assign44770_e49860_d_n6;
        locals.var_r2init_op_dn7 = assign44770_e49860_d_n7;
        locals.var_r2init_op_dn8 = assign44770_e49860_d_n8;
        locals.var_r2init_op_dn9 = assign44770_e49860_d_n9;

        let (assign44780_e49875, assign44780_e49875_d_n4, assign44780_e49875_d_n6, assign44780_e49875_d_n7, assign44780_e49875_d_n8, assign44780_e49875_d_n9,) = {
    if (locals.var_guard1360 != 0.0) {
        let assign44780_e49865: f64 = (1.0 + locals.var_r1init_op);
        let assign44780_e49866: f64 = (locals.var_k1_ac * assign44780_e49865);
        let assign44780_e49868: f64 = (assign44780_e49866 * locals.var_diff_min_ac);
        let assign44780_e49870: f64 = (assign44780_e49868 / locals.var_a0_ac);
        let assign44780_e49871: f64 = (assign44780_e49870).ln();
        let assign44780_e49873: f64 = (assign44780_e49871 + 2.0);
        (assign44780_e49873, ((((((((locals.var_k1_ac_dn4 * assign44780_e49865) + (locals.var_k1_ac * locals.var_r1init_op_dn4)) * locals.var_diff_min_ac) + (assign44780_e49866 * locals.var_diff_min_ac_dn4)) * locals.var_a0_ac) - (assign44780_e49868 * locals.var_a0_ac_dn4)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44780_e49870), ((((((((locals.var_k1_ac_dn6 * assign44780_e49865) + (locals.var_k1_ac * locals.var_r1init_op_dn6)) * locals.var_diff_min_ac) + (assign44780_e49866 * locals.var_diff_min_ac_dn6)) * locals.var_a0_ac) - (assign44780_e49868 * locals.var_a0_ac_dn6)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44780_e49870), ((((((((locals.var_k1_ac_dn7 * assign44780_e49865) + (locals.var_k1_ac * locals.var_r1init_op_dn7)) * locals.var_diff_min_ac) + (assign44780_e49866 * locals.var_diff_min_ac_dn7)) * locals.var_a0_ac) - (assign44780_e49868 * locals.var_a0_ac_dn7)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44780_e49870), ((((((((locals.var_k1_ac_dn8 * assign44780_e49865) + (locals.var_k1_ac * locals.var_r1init_op_dn8)) * locals.var_diff_min_ac) + (assign44780_e49866 * locals.var_diff_min_ac_dn8)) * locals.var_a0_ac) - (assign44780_e49868 * locals.var_a0_ac_dn8)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44780_e49870), ((((((((locals.var_k1_ac_dn9 * assign44780_e49865) + (locals.var_k1_ac * locals.var_r1init_op_dn9)) * locals.var_diff_min_ac) + (assign44780_e49866 * locals.var_diff_min_ac_dn9)) * locals.var_a0_ac) - (assign44780_e49868 * locals.var_a0_ac_dn9)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44780_e49870),)
    } else {
        (locals.var_x1init_op, locals.var_x1init_op_dn4, locals.var_x1init_op_dn6, locals.var_x1init_op_dn7, locals.var_x1init_op_dn8, locals.var_x1init_op_dn9,)
    }
};
        locals.var_x1init_op = assign44780_e49875;
        locals.var_x1init_op_dn4 = assign44780_e49875_d_n4;
        locals.var_x1init_op_dn6 = assign44780_e49875_d_n6;
        locals.var_x1init_op_dn7 = assign44780_e49875_d_n7;
        locals.var_x1init_op_dn8 = assign44780_e49875_d_n8;
        locals.var_x1init_op_dn9 = assign44780_e49875_d_n9;

        let (assign44790_e49890, assign44790_e49890_d_n4, assign44790_e49890_d_n6, assign44790_e49890_d_n7, assign44790_e49890_d_n8, assign44790_e49890_d_n9,) = {
    if (locals.var_guard1360 != 0.0) {
        let assign44790_e49880: f64 = (1.0 + locals.var_r2init_op);
        let assign44790_e49881: f64 = (locals.var_k2_ac * assign44790_e49880);
        let assign44790_e49883: f64 = (assign44790_e49881 * locals.var_diff_min_ac);
        let assign44790_e49885: f64 = (assign44790_e49883 / locals.var_a0_ac);
        let assign44790_e49886: f64 = (assign44790_e49885).ln();
        let assign44790_e49888: f64 = (assign44790_e49886 + 2.0);
        (assign44790_e49888, ((((((((locals.var_k2_ac_dn4 * assign44790_e49880) + (locals.var_k2_ac * locals.var_r2init_op_dn4)) * locals.var_diff_min_ac) + (assign44790_e49881 * locals.var_diff_min_ac_dn4)) * locals.var_a0_ac) - (assign44790_e49883 * locals.var_a0_ac_dn4)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44790_e49885), ((((((((locals.var_k2_ac_dn6 * assign44790_e49880) + (locals.var_k2_ac * locals.var_r2init_op_dn6)) * locals.var_diff_min_ac) + (assign44790_e49881 * locals.var_diff_min_ac_dn6)) * locals.var_a0_ac) - (assign44790_e49883 * locals.var_a0_ac_dn6)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44790_e49885), ((((((((locals.var_k2_ac_dn7 * assign44790_e49880) + (locals.var_k2_ac * locals.var_r2init_op_dn7)) * locals.var_diff_min_ac) + (assign44790_e49881 * locals.var_diff_min_ac_dn7)) * locals.var_a0_ac) - (assign44790_e49883 * locals.var_a0_ac_dn7)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44790_e49885), ((((((((locals.var_k2_ac_dn8 * assign44790_e49880) + (locals.var_k2_ac * locals.var_r2init_op_dn8)) * locals.var_diff_min_ac) + (assign44790_e49881 * locals.var_diff_min_ac_dn8)) * locals.var_a0_ac) - (assign44790_e49883 * locals.var_a0_ac_dn8)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44790_e49885), ((((((((locals.var_k2_ac_dn9 * assign44790_e49880) + (locals.var_k2_ac * locals.var_r2init_op_dn9)) * locals.var_diff_min_ac) + (assign44790_e49881 * locals.var_diff_min_ac_dn9)) * locals.var_a0_ac) - (assign44790_e49883 * locals.var_a0_ac_dn9)) / (locals.var_a0_ac * locals.var_a0_ac)) / assign44790_e49885),)
    } else {
        (locals.var_x2init_op, locals.var_x2init_op_dn4, locals.var_x2init_op_dn6, locals.var_x2init_op_dn7, locals.var_x2init_op_dn8, locals.var_x2init_op_dn9,)
    }
};
        locals.var_x2init_op = assign44790_e49890;
        locals.var_x2init_op_dn4 = assign44790_e49890_d_n4;
        locals.var_x2init_op_dn6 = assign44790_e49890_d_n6;
        locals.var_x2init_op_dn7 = assign44790_e49890_d_n7;
        locals.var_x2init_op_dn8 = assign44790_e49890_d_n8;
        locals.var_x2init_op_dn9 = assign44790_e49890_d_n9;

        let (assign44800_e49902, assign44800_e49902_d_n4, assign44800_e49902_d_n6, assign44800_e49902_d_n7, assign44800_e49902_d_n8, assign44800_e49902_d_n9,) = {
    if (locals.var_guard1360 != 0.0) {
        let assign44800_e49894: f64 = (1.0 + locals.var_r1init_op);
        let assign44800_e49896: f64 = (assign44800_e49894 * locals.var_x1init_op);
        let assign44800_e49899: f64 = (locals.var_xg2x_ac * locals.var_r1init_op);
        let assign44800_e49900: f64 = (assign44800_e49896 - assign44800_e49899);
        (assign44800_e49900, (((locals.var_r1init_op_dn4 * locals.var_x1init_op) + (assign44800_e49894 * locals.var_x1init_op_dn4)) - ((locals.var_xg2x_ac_dn4 * locals.var_r1init_op) + (locals.var_xg2x_ac * locals.var_r1init_op_dn4))), (((locals.var_r1init_op_dn6 * locals.var_x1init_op) + (assign44800_e49894 * locals.var_x1init_op_dn6)) - ((locals.var_xg2x_ac_dn6 * locals.var_r1init_op) + (locals.var_xg2x_ac * locals.var_r1init_op_dn6))), (((locals.var_r1init_op_dn7 * locals.var_x1init_op) + (assign44800_e49894 * locals.var_x1init_op_dn7)) - ((locals.var_xg2x_ac_dn7 * locals.var_r1init_op) + (locals.var_xg2x_ac * locals.var_r1init_op_dn7))), (((locals.var_r1init_op_dn8 * locals.var_x1init_op) + (assign44800_e49894 * locals.var_x1init_op_dn8)) - ((locals.var_xg2x_ac_dn8 * locals.var_r1init_op) + (locals.var_xg2x_ac * locals.var_r1init_op_dn8))), (((locals.var_r1init_op_dn9 * locals.var_x1init_op) + (assign44800_e49894 * locals.var_x1init_op_dn9)) - ((locals.var_xg2x_ac_dn9 * locals.var_r1init_op) + (locals.var_xg2x_ac * locals.var_r1init_op_dn9))),)
    } else {
        (locals.var_xth1init_op, locals.var_xth1init_op_dn4, locals.var_xth1init_op_dn6, locals.var_xth1init_op_dn7, locals.var_xth1init_op_dn8, locals.var_xth1init_op_dn9,)
    }
};
        locals.var_xth1init_op = assign44800_e49902;
        locals.var_xth1init_op_dn4 = assign44800_e49902_d_n4;
        locals.var_xth1init_op_dn6 = assign44800_e49902_d_n6;
        locals.var_xth1init_op_dn7 = assign44800_e49902_d_n7;
        locals.var_xth1init_op_dn8 = assign44800_e49902_d_n8;
        locals.var_xth1init_op_dn9 = assign44800_e49902_d_n9;

        let (assign44810_e49916, assign44810_e49916_d_n4, assign44810_e49916_d_n6, assign44810_e49916_d_n7, assign44810_e49916_d_n8, assign44810_e49916_d_n9,) = {
    if (locals.var_guard1360 != 0.0) {
        let assign44810_e49907: f64 = (1.0 / locals.var_r2init_op);
        let assign44810_e49908: f64 = (1.0 + assign44810_e49907);
        let assign44810_e49910: f64 = (assign44810_e49908 * locals.var_x2init_op);
        let assign44810_e49913: f64 = (locals.var_xg2x_ac / locals.var_r2init_op);
        let assign44810_e49914: f64 = (assign44810_e49910 - assign44810_e49913);
        (assign44810_e49914, ((((-(locals.var_r2init_op_dn4 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44810_e49908 * locals.var_x2init_op_dn4)) - (((locals.var_xg2x_ac_dn4 * locals.var_r2init_op) - (locals.var_xg2x_ac * locals.var_r2init_op_dn4)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn6 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44810_e49908 * locals.var_x2init_op_dn6)) - (((locals.var_xg2x_ac_dn6 * locals.var_r2init_op) - (locals.var_xg2x_ac * locals.var_r2init_op_dn6)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn7 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44810_e49908 * locals.var_x2init_op_dn7)) - (((locals.var_xg2x_ac_dn7 * locals.var_r2init_op) - (locals.var_xg2x_ac * locals.var_r2init_op_dn7)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn8 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44810_e49908 * locals.var_x2init_op_dn8)) - (((locals.var_xg2x_ac_dn8 * locals.var_r2init_op) - (locals.var_xg2x_ac * locals.var_r2init_op_dn8)) / (locals.var_r2init_op * locals.var_r2init_op))), ((((-(locals.var_r2init_op_dn9 / (locals.var_r2init_op * locals.var_r2init_op))) * locals.var_x2init_op) + (assign44810_e49908 * locals.var_x2init_op_dn9)) - (((locals.var_xg2x_ac_dn9 * locals.var_r2init_op) - (locals.var_xg2x_ac * locals.var_r2init_op_dn9)) / (locals.var_r2init_op * locals.var_r2init_op))),)
    } else {
        (locals.var_xth2init_op, locals.var_xth2init_op_dn4, locals.var_xth2init_op_dn6, locals.var_xth2init_op_dn7, locals.var_xth2init_op_dn8, locals.var_xth2init_op_dn9,)
    }
};
        locals.var_xth2init_op = assign44810_e49916;
        locals.var_xth2init_op_dn4 = assign44810_e49916_d_n4;
        locals.var_xth2init_op_dn6 = assign44810_e49916_d_n6;
        locals.var_xth2init_op_dn7 = assign44810_e49916_d_n7;
        locals.var_xth2init_op_dn8 = assign44810_e49916_d_n8;
        locals.var_xth2init_op_dn9 = assign44810_e49916_d_n9;

        let (assign44820_e49941, assign44820_e49941_d_n4, assign44820_e49941_d_n6, assign44820_e49941_d_n7, assign44820_e49941_d_n8, assign44820_e49941_d_n9,) = {
    if (locals.var_guard1360 != 0.0) {
        let assign44820_e49921: f64 = (locals.var_xth1init_op + locals.var_xth2init_op);
        let assign44820_e49924: f64 = (locals.var_xth1init_op - locals.var_xth2init_op);
        let assign44820_e49927: f64 = (locals.var_xth1init_op - locals.var_xth2init_op);
        let assign44820_e49928: f64 = (assign44820_e49924 * assign44820_e49927);
        let assign44820_e49930: f64 = (assign44820_e49928 + 38.0);
        let assign44820_e49931: f64 = (assign44820_e49930).sqrt();
        let assign44820_e49932: f64 = (assign44820_e49921 - assign44820_e49931);
        let assign44820_e49933: f64 = (0.5 * assign44820_e49932);
        let assign44820_e49935: f64 = (assign44820_e49933 - locals.var_xg2_ac);
        let assign44820_e49937: f64 = (assign44820_e49935 / locals.var_cic1_i);
        let assign44820_e49939: f64 = (assign44820_e49937 + locals.var_xg2_ac);
        (assign44820_e49939, ((((0.5 * ((locals.var_xth1init_op_dn4 + locals.var_xth2init_op_dn4) - ((((locals.var_xth1init_op_dn4 - locals.var_xth2init_op_dn4) * assign44820_e49927) + (assign44820_e49924 * (locals.var_xth1init_op_dn4 - locals.var_xth2init_op_dn4))) / (2.0 * assign44820_e49931)))) - locals.var_xg2_ac_dn4) / locals.var_cic1_i) + locals.var_xg2_ac_dn4), ((((0.5 * ((locals.var_xth1init_op_dn6 + locals.var_xth2init_op_dn6) - ((((locals.var_xth1init_op_dn6 - locals.var_xth2init_op_dn6) * assign44820_e49927) + (assign44820_e49924 * (locals.var_xth1init_op_dn6 - locals.var_xth2init_op_dn6))) / (2.0 * assign44820_e49931)))) - locals.var_xg2_ac_dn6) / locals.var_cic1_i) + locals.var_xg2_ac_dn6), ((((0.5 * ((locals.var_xth1init_op_dn7 + locals.var_xth2init_op_dn7) - ((((locals.var_xth1init_op_dn7 - locals.var_xth2init_op_dn7) * assign44820_e49927) + (assign44820_e49924 * (locals.var_xth1init_op_dn7 - locals.var_xth2init_op_dn7))) / (2.0 * assign44820_e49931)))) - locals.var_xg2_ac_dn7) / locals.var_cic1_i) + locals.var_xg2_ac_dn7), ((((0.5 * ((locals.var_xth1init_op_dn8 + locals.var_xth2init_op_dn8) - ((((locals.var_xth1init_op_dn8 - locals.var_xth2init_op_dn8) * assign44820_e49927) + (assign44820_e49924 * (locals.var_xth1init_op_dn8 - locals.var_xth2init_op_dn8))) / (2.0 * assign44820_e49931)))) - locals.var_xg2_ac_dn8) / locals.var_cic1_i) + locals.var_xg2_ac_dn8), ((((0.5 * ((locals.var_xth1init_op_dn9 + locals.var_xth2init_op_dn9) - ((((locals.var_xth1init_op_dn9 - locals.var_xth2init_op_dn9) * assign44820_e49927) + (assign44820_e49924 * (locals.var_xth1init_op_dn9 - locals.var_xth2init_op_dn9))) / (2.0 * assign44820_e49931)))) - locals.var_xg2_ac_dn9) / locals.var_cic1_i) + locals.var_xg2_ac_dn9),)
    } else {
        (locals.var_xg1thinit_op, locals.var_xg1thinit_op_dn4, locals.var_xg1thinit_op_dn6, locals.var_xg1thinit_op_dn7, locals.var_xg1thinit_op_dn8, locals.var_xg1thinit_op_dn9,)
    }
};
        locals.var_xg1thinit_op = assign44820_e49941;
        locals.var_xg1thinit_op_dn4 = assign44820_e49941_d_n4;
        locals.var_xg1thinit_op_dn6 = assign44820_e49941_d_n6;
        locals.var_xg1thinit_op_dn7 = assign44820_e49941_d_n7;
        locals.var_xg1thinit_op_dn8 = assign44820_e49941_d_n8;
        locals.var_xg1thinit_op_dn9 = assign44820_e49941_d_n9;

    }

    pub(super) fn stamp_transient_block_120(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign44830_e49957, assign44830_e49957_d_n4, assign44830_e49957_d_n6, assign44830_e49957_d_n7, assign44830_e49957_d_n8, assign44830_e49957_d_n9,) = {
    if (locals.var_guard1360 != 0.0) {
        let assign44830_e49946: f64 = (locals.var_xg1thinit_op - locals.var_xedge_ac);
        let assign44830_e49948: f64 = (assign44830_e49946 / locals.var_sce1_ac);
        let assign44830_e49950: f64 = (assign44830_e49948 - locals.var_dxg1_dibl_ac);
        let assign44830_e49952: f64 = (assign44830_e49950 + locals.var_xedge_ac);
        let assign44830_e49953: f64 = (locals.var_phit * assign44830_e49952);
        let assign44830_e49955: f64 = (assign44830_e49953 + locals.var_vfbac1_i);
        (assign44830_e49955, (((locals.var_phit_dn4 * assign44830_e49952) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn4 - locals.var_xedge_ac_dn4) * locals.var_sce1_ac) - (assign44830_e49946 * locals.var_sce1_ac_dn4)) / (locals.var_sce1_ac * locals.var_sce1_ac)) - locals.var_dxg1_dibl_ac_dn4) + locals.var_xedge_ac_dn4))) + locals.var_vfbac1_i_dn4), (((locals.var_phit_dn6 * assign44830_e49952) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn6 - locals.var_xedge_ac_dn6) * locals.var_sce1_ac) - (assign44830_e49946 * locals.var_sce1_ac_dn6)) / (locals.var_sce1_ac * locals.var_sce1_ac)) - locals.var_dxg1_dibl_ac_dn6) + locals.var_xedge_ac_dn6))) + locals.var_vfbac1_i_dn6), (((locals.var_phit_dn7 * assign44830_e49952) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn7 - locals.var_xedge_ac_dn7) * locals.var_sce1_ac) - (assign44830_e49946 * locals.var_sce1_ac_dn7)) / (locals.var_sce1_ac * locals.var_sce1_ac)) - locals.var_dxg1_dibl_ac_dn7) + locals.var_xedge_ac_dn7))) + locals.var_vfbac1_i_dn7), (((locals.var_phit_dn8 * assign44830_e49952) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn8 - locals.var_xedge_ac_dn8) * locals.var_sce1_ac) - (assign44830_e49946 * locals.var_sce1_ac_dn8)) / (locals.var_sce1_ac * locals.var_sce1_ac)) - locals.var_dxg1_dibl_ac_dn8) + locals.var_xedge_ac_dn8))) + locals.var_vfbac1_i_dn8), (((locals.var_phit_dn9 * assign44830_e49952) + (locals.var_phit * ((((((locals.var_xg1thinit_op_dn9 - locals.var_xedge_ac_dn9) * locals.var_sce1_ac) - (assign44830_e49946 * locals.var_sce1_ac_dn9)) / (locals.var_sce1_ac * locals.var_sce1_ac)) - locals.var_dxg1_dibl_ac_dn9) + locals.var_xedge_ac_dn9))) + locals.var_vfbac1_i_dn9),)
    } else {
        (locals.var_vthinit_op, locals.var_vthinit_op_dn4, locals.var_vthinit_op_dn6, locals.var_vthinit_op_dn7, locals.var_vthinit_op_dn8, locals.var_vthinit_op_dn9,)
    }
};
        locals.var_vthinit_op = assign44830_e49957;
        locals.var_vthinit_op_dn4 = assign44830_e49957_d_n4;
        locals.var_vthinit_op_dn6 = assign44830_e49957_d_n6;
        locals.var_vthinit_op_dn7 = assign44830_e49957_d_n7;
        locals.var_vthinit_op_dn8 = assign44830_e49957_d_n8;
        locals.var_vthinit_op_dn9 = assign44830_e49957_d_n9;

        let (assign44840_e49965, assign44840_e49965_d_n4, assign44840_e49965_d_n6, assign44840_e49965_d_n7, assign44840_e49965_d_n8, assign44840_e49965_d_n9,) = {
    if (locals.var_guard1360 != 0.0) {
        let assign44840_e49962: f64 = (locals.var_tkd - locals.var_tkr);
        let assign44840_e49963: f64 = (locals.var_stcf_i * assign44840_e49962);
        (assign44840_e49963, ((locals.var_stcf_i_dn4 * assign44840_e49962) + (locals.var_stcf_i * locals.var_tkd_dn4)), ((locals.var_stcf_i_dn6 * assign44840_e49962) + (locals.var_stcf_i * locals.var_tkd_dn6)), ((locals.var_stcf_i_dn7 * assign44840_e49962) + (locals.var_stcf_i * locals.var_tkd_dn7)), ((locals.var_stcf_i_dn8 * assign44840_e49962) + (locals.var_stcf_i * locals.var_tkd_dn8)), ((locals.var_stcf_i_dn9 * assign44840_e49962) + (locals.var_stcf_i * locals.var_tkd_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign44840_e49965;
        locals.var_temp_dn4 = assign44840_e49965_d_n4;
        locals.var_temp_dn6 = assign44840_e49965_d_n6;
        locals.var_temp_dn7 = assign44840_e49965_d_n7;
        locals.var_temp_dn8 = assign44840_e49965_d_n8;
        locals.var_temp_dn9 = assign44840_e49965_d_n9;

        let (assign44870_e49989, assign44870_e49989_d_n4, assign44870_e49989_d_n6, assign44870_e49989_d_n7, assign44870_e49989_d_n8, assign44870_e49989_d_n9,) = {
    if (locals.var_guard1360 != 0.0) {
        let assign44870_e49981: f64 = (p.p14 * locals.var_stvfb_i);
        let assign44870_e49984: f64 = (locals.var_tkd - locals.var_tkr);
        let assign44870_e49985: f64 = (assign44870_e49981 * assign44870_e49984);
        let assign44870_e49987: f64 = (assign44870_e49985 + locals.var_dvfbqm);
        (assign44870_e49987, (assign44870_e49981 * locals.var_tkd_dn4), (assign44870_e49981 * locals.var_tkd_dn6), (assign44870_e49981 * locals.var_tkd_dn7), (assign44870_e49981 * locals.var_tkd_dn8), (assign44870_e49981 * locals.var_tkd_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign44870_e49989;
        locals.var_temp_dn4 = assign44870_e49989_d_n4;
        locals.var_temp_dn6 = assign44870_e49989_d_n6;
        locals.var_temp_dn7 = assign44870_e49989_d_n7;
        locals.var_temp_dn8 = assign44870_e49989_d_n8;
        locals.var_temp_dn9 = assign44870_e49989_d_n9;

        let (assign44880_e50005, assign44880_e50005_d_n4, assign44880_e50005_d_n6, assign44880_e50005_d_n7, assign44880_e50005_d_n8, assign44880_e50005_d_n9,) = {
    if (locals.var_guard1360 != 0.0) {
        let assign44880_e49994: f64 = (locals.var_vfbac1_t + locals.var_dvfbch_op);
        let assign44880_e49996: f64 = (assign44880_e49994 + locals.var_dvfb1nch);
        let assign44880_e49997: f64 = (p.p14 * assign44880_e49996);
        let assign44880_e49999: f64 = (assign44880_e49997 + locals.var_temp);
        let assign44880_e50001: f64 = (assign44880_e49999 + p.p34);
        let assign44880_e50003: f64 = (assign44880_e50001 - locals.var_dvfbpdep_op);
        (assign44880_e50003, (((p.p14 * ((locals.var_vfbac1_t_dn4 + locals.var_dvfbch_op_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp_dn4) - locals.var_dvfbpdep_op_dn4), (((p.p14 * ((locals.var_vfbac1_t_dn6 + locals.var_dvfbch_op_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp_dn6) - locals.var_dvfbpdep_op_dn6), (((p.p14 * ((locals.var_vfbac1_t_dn7 + locals.var_dvfbch_op_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp_dn7) - locals.var_dvfbpdep_op_dn7), (((p.p14 * ((locals.var_vfbac1_t_dn8 + locals.var_dvfbch_op_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp_dn8) - locals.var_dvfbpdep_op_dn8), (((p.p14 * ((locals.var_vfbac1_t_dn9 + locals.var_dvfbch_op_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp_dn9) - locals.var_dvfbpdep_op_dn9),)
    } else {
        (locals.var_vfb1_op, locals.var_vfb1_op_dn4, locals.var_vfb1_op_dn6, locals.var_vfb1_op_dn7, locals.var_vfb1_op_dn8, locals.var_vfb1_op_dn9,)
    }
};
        locals.var_vfb1_op = assign44880_e50005;
        locals.var_vfb1_op_dn4 = assign44880_e50005_d_n4;
        locals.var_vfb1_op_dn6 = assign44880_e50005_d_n6;
        locals.var_vfb1_op_dn7 = assign44880_e50005_d_n7;
        locals.var_vfb1_op_dn8 = assign44880_e50005_d_n8;
        locals.var_vfb1_op_dn9 = assign44880_e50005_d_n9;

        let (assign44890_e50017, assign44890_e50017_d_n4, assign44890_e50017_d_n6, assign44890_e50017_d_n7, assign44890_e50017_d_n8, assign44890_e50017_d_n9,) = {
    if (locals.var_guard1360 != 0.0) {
        let assign44890_e50010: f64 = (locals.var_vfbac2_t + locals.var_dvfbch_op);
        let assign44890_e50012: f64 = (assign44890_e50010 + locals.var_dvfb2nch);
        let assign44890_e50013: f64 = (p.p14 * assign44890_e50012);
        let assign44890_e50015: f64 = (assign44890_e50013 + locals.var_temp);
        (assign44890_e50015, ((p.p14 * ((locals.var_vfbac2_t_dn4 + locals.var_dvfbch_op_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4), ((p.p14 * ((locals.var_vfbac2_t_dn6 + locals.var_dvfbch_op_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6), ((p.p14 * ((locals.var_vfbac2_t_dn7 + locals.var_dvfbch_op_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7), ((p.p14 * ((locals.var_vfbac2_t_dn8 + locals.var_dvfbch_op_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8), ((p.p14 * ((locals.var_vfbac2_t_dn9 + locals.var_dvfbch_op_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9),)
    } else {
        (locals.var_vfb2_op, locals.var_vfb2_op_dn4, locals.var_vfb2_op_dn6, locals.var_vfb2_op_dn7, locals.var_vfb2_op_dn8, locals.var_vfb2_op_dn9,)
    }
};
        locals.var_vfb2_op = assign44890_e50017;
        locals.var_vfb2_op_dn4 = assign44890_e50017_d_n4;
        locals.var_vfb2_op_dn6 = assign44890_e50017_d_n6;
        locals.var_vfb2_op_dn7 = assign44890_e50017_d_n7;
        locals.var_vfb2_op_dn8 = assign44890_e50017_d_n8;
        locals.var_vfb2_op_dn9 = assign44890_e50017_d_n9;

        let (assign44900_e50027, assign44900_e50027_d_n4, assign44900_e50027_d_n6, assign44900_e50027_d_n7, assign44900_e50027_d_n8, assign44900_e50027_d_n9,) = {
    if (locals.var_guard1360 != 0.0) {
        let assign44900_e50021: f64 = (locals.var_vthinit_op - locals.var_vfb1_op);
        let assign44900_e50023: f64 = (assign44900_e50021 * locals.var_inv_phit_op);
        let assign44900_e50025: f64 = (assign44900_e50023 - locals.var_dxdsx_op);
        (assign44900_e50025, ((((locals.var_vthinit_op_dn4 - locals.var_vfb1_op_dn4) * locals.var_inv_phit_op) + (assign44900_e50021 * locals.var_inv_phit_op_dn4)) - locals.var_dxdsx_op_dn4), ((((locals.var_vthinit_op_dn6 - locals.var_vfb1_op_dn6) * locals.var_inv_phit_op) + (assign44900_e50021 * locals.var_inv_phit_op_dn6)) - locals.var_dxdsx_op_dn6), ((((locals.var_vthinit_op_dn7 - locals.var_vfb1_op_dn7) * locals.var_inv_phit_op) + (assign44900_e50021 * locals.var_inv_phit_op_dn7)) - locals.var_dxdsx_op_dn7), ((((locals.var_vthinit_op_dn8 - locals.var_vfb1_op_dn8) * locals.var_inv_phit_op) + (assign44900_e50021 * locals.var_inv_phit_op_dn8)) - locals.var_dxdsx_op_dn8), ((((locals.var_vthinit_op_dn9 - locals.var_vfb1_op_dn9) * locals.var_inv_phit_op) + (assign44900_e50021 * locals.var_inv_phit_op_dn9)) - locals.var_dxdsx_op_dn9),)
    } else {
        (locals.var_xg10_op, locals.var_xg10_op_dn4, locals.var_xg10_op_dn6, locals.var_xg10_op_dn7, locals.var_xg10_op_dn8, locals.var_xg10_op_dn9,)
    }
};
        locals.var_xg10_op = assign44900_e50027;
        locals.var_xg10_op_dn4 = assign44900_e50027_d_n4;
        locals.var_xg10_op_dn6 = assign44900_e50027_d_n6;
        locals.var_xg10_op_dn7 = assign44900_e50027_d_n7;
        locals.var_xg10_op_dn8 = assign44900_e50027_d_n8;
        locals.var_xg10_op_dn9 = assign44900_e50027_d_n9;

        let (assign44910_e50038, assign44910_e50038_d_n4, assign44910_e50038_d_n6, assign44910_e50038_d_n7, assign44910_e50038_d_n8, assign44910_e50038_d_n9,) = {
    if (locals.var_guard1360 != 0.0) {
        let assign44910_e50030: f64 = (-locals.var_vsb);
        let assign44910_e50032: f64 = (assign44910_e50030 - locals.var_vfb2_op);
        let assign44910_e50034: f64 = (assign44910_e50032 * locals.var_inv_phit_op);
        let assign44910_e50036: f64 = (assign44910_e50034 - locals.var_dxdsx_op);
        (assign44910_e50036, ((((-locals.var_vfb2_op_dn4) * locals.var_inv_phit_op) + (assign44910_e50032 * locals.var_inv_phit_op_dn4)) - locals.var_dxdsx_op_dn4), (((((-locals.var_vsb_dn6) - locals.var_vfb2_op_dn6) * locals.var_inv_phit_op) + (assign44910_e50032 * locals.var_inv_phit_op_dn6)) - locals.var_dxdsx_op_dn6), (((((-locals.var_vsb_dn7) - locals.var_vfb2_op_dn7) * locals.var_inv_phit_op) + (assign44910_e50032 * locals.var_inv_phit_op_dn7)) - locals.var_dxdsx_op_dn7), (((((-locals.var_vsb_dn8) - locals.var_vfb2_op_dn8) * locals.var_inv_phit_op) + (assign44910_e50032 * locals.var_inv_phit_op_dn8)) - locals.var_dxdsx_op_dn8), ((((-locals.var_vfb2_op_dn9) * locals.var_inv_phit_op) + (assign44910_e50032 * locals.var_inv_phit_op_dn9)) - locals.var_dxdsx_op_dn9),)
    } else {
        (locals.var_xg20_op, locals.var_xg20_op_dn4, locals.var_xg20_op_dn6, locals.var_xg20_op_dn7, locals.var_xg20_op_dn8, locals.var_xg20_op_dn9,)
    }
};
        locals.var_xg20_op = assign44910_e50038;
        locals.var_xg20_op_dn4 = assign44910_e50038_d_n4;
        locals.var_xg20_op_dn6 = assign44910_e50038_d_n6;
        locals.var_xg20_op_dn7 = assign44910_e50038_d_n7;
        locals.var_xg20_op_dn8 = assign44910_e50038_d_n8;
        locals.var_xg20_op_dn9 = assign44910_e50038_d_n9;

        let assign44920_e50041: f64 = if p.p2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1361 = assign44920_e50041;

        let (assign44930_e50055, assign44930_e50055_d_n4, assign44930_e50055_d_n6, assign44930_e50055_d_n7, assign44930_e50055_d_n8, assign44930_e50055_d_n9,) = {
    if ((locals.var_guard1360 != 0.0) && (locals.var_guard1361 != 0.0)) {
        let assign44930_e50047: f64 = (p.p14 * locals.var_typesub_i);
        let assign44930_e50050: f64 = (locals.var_xg10_op - locals.var_xg20_op);
        let assign44930_e50051: f64 = (assign44930_e50047 * assign44930_e50050);
        let assign44930_e50053: f64 = (assign44930_e50051 / locals.var_gfsub);
        (assign44930_e50053, ((((assign44930_e50047 * (locals.var_xg10_op_dn4 - locals.var_xg20_op_dn4)) * locals.var_gfsub) - (assign44930_e50051 * locals.var_gfsub_dn4)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44930_e50047 * (locals.var_xg10_op_dn6 - locals.var_xg20_op_dn6)) * locals.var_gfsub) - (assign44930_e50051 * locals.var_gfsub_dn6)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44930_e50047 * (locals.var_xg10_op_dn7 - locals.var_xg20_op_dn7)) * locals.var_gfsub) - (assign44930_e50051 * locals.var_gfsub_dn7)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44930_e50047 * (locals.var_xg10_op_dn8 - locals.var_xg20_op_dn8)) * locals.var_gfsub) - (assign44930_e50051 * locals.var_gfsub_dn8)) / (locals.var_gfsub * locals.var_gfsub)), ((((assign44930_e50047 * (locals.var_xg10_op_dn9 - locals.var_xg20_op_dn9)) * locals.var_gfsub) - (assign44930_e50051 * locals.var_gfsub_dn9)) / (locals.var_gfsub * locals.var_gfsub)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign44930_e50055;
        locals.var_temp_dn4 = assign44930_e50055_d_n4;
        locals.var_temp_dn6 = assign44930_e50055_d_n6;
        locals.var_temp_dn7 = assign44930_e50055_d_n7;
        locals.var_temp_dn8 = assign44930_e50055_d_n8;
        locals.var_temp_dn9 = assign44930_e50055_d_n9;

        let assign44940_e50058: f64 = if locals.var_temp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1362 = assign44940_e50058;

        let (assign44950_e50072, assign44950_e50072_d_n4, assign44950_e50072_d_n6, assign44950_e50072_d_n7, assign44950_e50072_d_n8, assign44950_e50072_d_n9,) = {
    if (((locals.var_guard1360 != 0.0) && (locals.var_guard1361 != 0.0)) && (locals.var_guard1362 != 0.0)) {
        let assign44950_e50065: f64 = (-2.0);
        let assign44950_e50068: f64 = (1.0 - locals.var_temp);
        let assign44950_e50069: f64 = (assign44950_e50068).ln();
        let assign44950_e50070: f64 = (assign44950_e50065 * assign44950_e50069);
        (assign44950_e50070, (assign44950_e50065 * ((-locals.var_temp_dn4) / assign44950_e50068)), (assign44950_e50065 * ((-locals.var_temp_dn6) / assign44950_e50068)), (assign44950_e50065 * ((-locals.var_temp_dn7) / assign44950_e50068)), (assign44950_e50065 * ((-locals.var_temp_dn8) / assign44950_e50068)), (assign44950_e50065 * ((-locals.var_temp_dn9) / assign44950_e50068)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign44950_e50072;
        locals.var_temp1_dn4 = assign44950_e50072_d_n4;
        locals.var_temp1_dn6 = assign44950_e50072_d_n6;
        locals.var_temp1_dn7 = assign44950_e50072_d_n7;
        locals.var_temp1_dn8 = assign44950_e50072_d_n8;
        locals.var_temp1_dn9 = assign44950_e50072_d_n9;

        let (assign44960_e50091, assign44960_e50091_d_n4, assign44960_e50091_d_n6, assign44960_e50091_d_n7, assign44960_e50091_d_n8, assign44960_e50091_d_n9,) = {
    if (((locals.var_guard1360 != 0.0) && (locals.var_guard1361 != 0.0)) && (locals.var_guard1362 == 0.0)) {
        let assign44960_e50081: f64 = (locals.var_temp * locals.var_temp);
        let assign44960_e50085: f64 = (2.0 * locals.var_temp);
        let assign44960_e50087: f64 = (assign44960_e50085 / locals.var_gfsub);
        let assign44960_e50088: f64 = (1.0 + assign44960_e50087);
        let assign44960_e50089: f64 = (assign44960_e50081 / assign44960_e50088);
        (assign44960_e50089, (((((locals.var_temp_dn4 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn4)) * assign44960_e50088) - (assign44960_e50081 * ((((2.0 * locals.var_temp_dn4) * locals.var_gfsub) - (assign44960_e50085 * locals.var_gfsub_dn4)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44960_e50088 * assign44960_e50088)), (((((locals.var_temp_dn6 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn6)) * assign44960_e50088) - (assign44960_e50081 * ((((2.0 * locals.var_temp_dn6) * locals.var_gfsub) - (assign44960_e50085 * locals.var_gfsub_dn6)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44960_e50088 * assign44960_e50088)), (((((locals.var_temp_dn7 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn7)) * assign44960_e50088) - (assign44960_e50081 * ((((2.0 * locals.var_temp_dn7) * locals.var_gfsub) - (assign44960_e50085 * locals.var_gfsub_dn7)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44960_e50088 * assign44960_e50088)), (((((locals.var_temp_dn8 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn8)) * assign44960_e50088) - (assign44960_e50081 * ((((2.0 * locals.var_temp_dn8) * locals.var_gfsub) - (assign44960_e50085 * locals.var_gfsub_dn8)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44960_e50088 * assign44960_e50088)), (((((locals.var_temp_dn9 * locals.var_temp) + (locals.var_temp * locals.var_temp_dn9)) * assign44960_e50088) - (assign44960_e50081 * ((((2.0 * locals.var_temp_dn9) * locals.var_gfsub) - (assign44960_e50085 * locals.var_gfsub_dn9)) / (locals.var_gfsub * locals.var_gfsub)))) / (assign44960_e50088 * assign44960_e50088)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign44960_e50091;
        locals.var_temp1_dn4 = assign44960_e50091_d_n4;
        locals.var_temp1_dn6 = assign44960_e50091_d_n6;
        locals.var_temp1_dn7 = assign44960_e50091_d_n7;
        locals.var_temp1_dn8 = assign44960_e50091_d_n8;
        locals.var_temp1_dn9 = assign44960_e50091_d_n9;

        let (assign44970_e50103, assign44970_e50103_d_n4, assign44970_e50103_d_n6, assign44970_e50103_d_n7, assign44970_e50103_d_n8, assign44970_e50103_d_n9,) = {
    if ((locals.var_guard1360 != 0.0) && (locals.var_guard1361 != 0.0)) {
        let assign44970_e50098: f64 = (p.p14 * locals.var_typesub_i);
        let assign44970_e50100: f64 = (assign44970_e50098 * locals.var_temp1);
        let assign44970_e50101: f64 = (locals.var_xg20_op + assign44970_e50100);
        (assign44970_e50101, (locals.var_xg20_op_dn4 + (assign44970_e50098 * locals.var_temp1_dn4)), (locals.var_xg20_op_dn6 + (assign44970_e50098 * locals.var_temp1_dn6)), (locals.var_xg20_op_dn7 + (assign44970_e50098 * locals.var_temp1_dn7)), (locals.var_xg20_op_dn8 + (assign44970_e50098 * locals.var_temp1_dn8)), (locals.var_xg20_op_dn9 + (assign44970_e50098 * locals.var_temp1_dn9)),)
    } else {
        (locals.var_xg2eff_op, locals.var_xg2eff_op_dn4, locals.var_xg2eff_op_dn6, locals.var_xg2eff_op_dn7, locals.var_xg2eff_op_dn8, locals.var_xg2eff_op_dn9,)
    }
};
        locals.var_xg2eff_op = assign44970_e50103;
        locals.var_xg2eff_op_dn4 = assign44970_e50103_d_n4;
        locals.var_xg2eff_op_dn6 = assign44970_e50103_d_n6;
        locals.var_xg2eff_op_dn7 = assign44970_e50103_d_n7;
        locals.var_xg2eff_op_dn8 = assign44970_e50103_d_n8;
        locals.var_xg2eff_op_dn9 = assign44970_e50103_d_n9;

        let (assign44980_e50110, assign44980_e50110_d_n4, assign44980_e50110_d_n6, assign44980_e50110_d_n7, assign44980_e50110_d_n8, assign44980_e50110_d_n9,) = {
    if ((locals.var_guard1360 != 0.0) && (locals.var_guard1361 == 0.0)) {
        (locals.var_xg20_op, locals.var_xg20_op_dn4, locals.var_xg20_op_dn6, locals.var_xg20_op_dn7, locals.var_xg20_op_dn8, locals.var_xg20_op_dn9,)
    } else {
        (locals.var_xg2eff_op, locals.var_xg2eff_op_dn4, locals.var_xg2eff_op_dn6, locals.var_xg2eff_op_dn7, locals.var_xg2eff_op_dn8, locals.var_xg2eff_op_dn9,)
    }
};
        locals.var_xg2eff_op = assign44980_e50110;
        locals.var_xg2eff_op_dn4 = assign44980_e50110_d_n4;
        locals.var_xg2eff_op_dn6 = assign44980_e50110_d_n6;
        locals.var_xg2eff_op_dn7 = assign44980_e50110_d_n7;
        locals.var_xg2eff_op_dn8 = assign44980_e50110_d_n8;
        locals.var_xg2eff_op_dn9 = assign44980_e50110_d_n9;

        let (assign44990_e50118, assign44990_e50118_d_n4, assign44990_e50118_d_n6, assign44990_e50118_d_n7, assign44990_e50118_d_n8, assign44990_e50118_d_n9,) = {
    if (locals.var_guard1360 != 0.0) {
        let assign44990_e50115: f64 = (locals.var_xg10_op - locals.var_xg2eff_op);
        let assign44990_e50116: f64 = (locals.var_keq_1d * assign44990_e50115);
        (assign44990_e50116, (locals.var_keq_1d * (locals.var_xg10_op_dn4 - locals.var_xg2eff_op_dn4)), (locals.var_keq_1d * (locals.var_xg10_op_dn6 - locals.var_xg2eff_op_dn6)), (locals.var_keq_1d * (locals.var_xg10_op_dn7 - locals.var_xg2eff_op_dn7)), (locals.var_keq_1d * (locals.var_xg10_op_dn8 - locals.var_xg2eff_op_dn8)), (locals.var_keq_1d * (locals.var_xg10_op_dn9 - locals.var_xg2eff_op_dn9)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign44990_e50118;
        locals.var_temp_dn4 = assign44990_e50118_d_n4;
        locals.var_temp_dn6 = assign44990_e50118_d_n6;
        locals.var_temp_dn7 = assign44990_e50118_d_n7;
        locals.var_temp_dn8 = assign44990_e50118_d_n8;
        locals.var_temp_dn9 = assign44990_e50118_d_n9;

        let assign45000_e50121: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1363 = assign45000_e50121;

        let (assign45010_e50144, assign45010_e50144_d_n4, assign45010_e50144_d_n6, assign45010_e50144_d_n7, assign45010_e50144_d_n8, assign45010_e50144_d_n9,) = {
    if ((locals.var_guard1360 != 0.0) && (locals.var_guard1363 != 0.0)) {
        let assign45010_e50128: f64 = (locals.var_temp + locals.var_emin);
        let assign45010_e50131: f64 = (locals.var_temp - locals.var_emin);
        let assign45010_e50134: f64 = (locals.var_temp - locals.var_emin);
        let assign45010_e50135: f64 = (assign45010_e50131 * assign45010_e50134);
        let assign45010_e50138: f64 = (locals.var_emin * locals.var_emin);
        let assign45010_e50139: f64 = (assign45010_e50135 + assign45010_e50138);
        let assign45010_e50140: f64 = (assign45010_e50139).sqrt();
        let assign45010_e50141: f64 = (assign45010_e50128 + assign45010_e50140);
        let assign45010_e50142: f64 = (0.5 * assign45010_e50141);
        (assign45010_e50142, (0.5 * ((locals.var_temp_dn4 + locals.var_emin_dn4) + (((((locals.var_temp_dn4 - locals.var_emin_dn4) * assign45010_e50134) + (assign45010_e50131 * (locals.var_temp_dn4 - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign45010_e50140)))), (0.5 * ((locals.var_temp_dn6 + locals.var_emin_dn6) + (((((locals.var_temp_dn6 - locals.var_emin_dn6) * assign45010_e50134) + (assign45010_e50131 * (locals.var_temp_dn6 - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign45010_e50140)))), (0.5 * ((locals.var_temp_dn7 + locals.var_emin_dn7) + (((((locals.var_temp_dn7 - locals.var_emin_dn7) * assign45010_e50134) + (assign45010_e50131 * (locals.var_temp_dn7 - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign45010_e50140)))), (0.5 * ((locals.var_temp_dn8 + locals.var_emin_dn8) + (((((locals.var_temp_dn8 - locals.var_emin_dn8) * assign45010_e50134) + (assign45010_e50131 * (locals.var_temp_dn8 - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign45010_e50140)))), (0.5 * ((locals.var_temp_dn9 + locals.var_emin_dn9) + (((((locals.var_temp_dn9 - locals.var_emin_dn9) * assign45010_e50134) + (assign45010_e50131 * (locals.var_temp_dn9 - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign45010_e50140)))),)
    } else {
        (locals.var_e1_op, locals.var_e1_op_dn4, locals.var_e1_op_dn6, locals.var_e1_op_dn7, locals.var_e1_op_dn8, locals.var_e1_op_dn9,)
    }
};
        locals.var_e1_op = assign45010_e50144;
        locals.var_e1_op_dn4 = assign45010_e50144_d_n4;
        locals.var_e1_op_dn6 = assign45010_e50144_d_n6;
        locals.var_e1_op_dn7 = assign45010_e50144_d_n7;
        locals.var_e1_op_dn8 = assign45010_e50144_d_n8;
        locals.var_e1_op_dn9 = assign45010_e50144_d_n9;

        let (assign45020_e50170, assign45020_e50170_d_n4, assign45020_e50170_d_n6, assign45020_e50170_d_n7, assign45020_e50170_d_n8, assign45020_e50170_d_n9,) = {
    if ((locals.var_guard1360 != 0.0) && (locals.var_guard1363 != 0.0)) {
        let assign45020_e50150: f64 = (-locals.var_temp);
        let assign45020_e50152: f64 = (assign45020_e50150 + locals.var_emin);
        let assign45020_e50154: f64 = (-locals.var_temp);
        let assign45020_e50156: f64 = (assign45020_e50154 - locals.var_emin);
        let assign45020_e50158: f64 = (-locals.var_temp);
        let assign45020_e50160: f64 = (assign45020_e50158 - locals.var_emin);
        let assign45020_e50161: f64 = (assign45020_e50156 * assign45020_e50160);
        let assign45020_e50164: f64 = (locals.var_emin * locals.var_emin);
        let assign45020_e50165: f64 = (assign45020_e50161 + assign45020_e50164);
        let assign45020_e50166: f64 = (assign45020_e50165).sqrt();
        let assign45020_e50167: f64 = (assign45020_e50152 + assign45020_e50166);
        let assign45020_e50168: f64 = (0.5 * assign45020_e50167);
        (assign45020_e50168, (0.5 * (((-locals.var_temp_dn4) + locals.var_emin_dn4) + ((((((-locals.var_temp_dn4) - locals.var_emin_dn4) * assign45020_e50160) + (assign45020_e50156 * ((-locals.var_temp_dn4) - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign45020_e50166)))), (0.5 * (((-locals.var_temp_dn6) + locals.var_emin_dn6) + ((((((-locals.var_temp_dn6) - locals.var_emin_dn6) * assign45020_e50160) + (assign45020_e50156 * ((-locals.var_temp_dn6) - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign45020_e50166)))), (0.5 * (((-locals.var_temp_dn7) + locals.var_emin_dn7) + ((((((-locals.var_temp_dn7) - locals.var_emin_dn7) * assign45020_e50160) + (assign45020_e50156 * ((-locals.var_temp_dn7) - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign45020_e50166)))), (0.5 * (((-locals.var_temp_dn8) + locals.var_emin_dn8) + ((((((-locals.var_temp_dn8) - locals.var_emin_dn8) * assign45020_e50160) + (assign45020_e50156 * ((-locals.var_temp_dn8) - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign45020_e50166)))), (0.5 * (((-locals.var_temp_dn9) + locals.var_emin_dn9) + ((((((-locals.var_temp_dn9) - locals.var_emin_dn9) * assign45020_e50160) + (assign45020_e50156 * ((-locals.var_temp_dn9) - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign45020_e50166)))),)
    } else {
        (locals.var_e2_op, locals.var_e2_op_dn4, locals.var_e2_op_dn6, locals.var_e2_op_dn7, locals.var_e2_op_dn8, locals.var_e2_op_dn9,)
    }
};
        locals.var_e2_op = assign45020_e50170;
        locals.var_e2_op_dn4 = assign45020_e50170_d_n4;
        locals.var_e2_op_dn6 = assign45020_e50170_d_n6;
        locals.var_e2_op_dn7 = assign45020_e50170_d_n7;
        locals.var_e2_op_dn8 = assign45020_e50170_d_n8;
        locals.var_e2_op_dn9 = assign45020_e50170_d_n9;

        let (assign45030_e50183, assign45030_e50183_d_n4, assign45030_e50183_d_n6, assign45030_e50183_d_n7, assign45030_e50183_d_n8, assign45030_e50183_d_n9,) = {
    if ((locals.var_guard1360 != 0.0) && (locals.var_guard1363 != 0.0)) {
        let assign45030_e50176: f64 = (-0.3333333333333);
        let assign45030_e50178: f64 = (locals.var_e1_op).ln();
        let assign45030_e50179: f64 = (assign45030_e50176 * assign45030_e50178);
        let assign45030_e50180: f64 = (assign45030_e50179).exp();
        let assign45030_e50181: f64 = (locals.var_qq_op * assign45030_e50180);
        (assign45030_e50181, ((locals.var_qq_op_dn4 * assign45030_e50180) + (locals.var_qq_op * (assign45030_e50180 * (assign45030_e50176 * (locals.var_e1_op_dn4 / locals.var_e1_op))))), ((locals.var_qq_op_dn6 * assign45030_e50180) + (locals.var_qq_op * (assign45030_e50180 * (assign45030_e50176 * (locals.var_e1_op_dn6 / locals.var_e1_op))))), ((locals.var_qq_op_dn7 * assign45030_e50180) + (locals.var_qq_op * (assign45030_e50180 * (assign45030_e50176 * (locals.var_e1_op_dn7 / locals.var_e1_op))))), ((locals.var_qq_op_dn8 * assign45030_e50180) + (locals.var_qq_op * (assign45030_e50180 * (assign45030_e50176 * (locals.var_e1_op_dn8 / locals.var_e1_op))))), ((locals.var_qq_op_dn9 * assign45030_e50180) + (locals.var_qq_op * (assign45030_e50180 * (assign45030_e50176 * (locals.var_e1_op_dn9 / locals.var_e1_op))))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign45030_e50183;
        locals.var_temp1_dn4 = assign45030_e50183_d_n4;
        locals.var_temp1_dn6 = assign45030_e50183_d_n6;
        locals.var_temp1_dn7 = assign45030_e50183_d_n7;
        locals.var_temp1_dn8 = assign45030_e50183_d_n8;
        locals.var_temp1_dn9 = assign45030_e50183_d_n9;

        let (assign45040_e50196, assign45040_e50196_d_n4, assign45040_e50196_d_n6, assign45040_e50196_d_n7, assign45040_e50196_d_n8, assign45040_e50196_d_n9,) = {
    if ((locals.var_guard1360 != 0.0) && (locals.var_guard1363 != 0.0)) {
        let assign45040_e50189: f64 = (-0.3333333333333);
        let assign45040_e50191: f64 = (locals.var_e2_op).ln();
        let assign45040_e50192: f64 = (assign45040_e50189 * assign45040_e50191);
        let assign45040_e50193: f64 = (assign45040_e50192).exp();
        let assign45040_e50194: f64 = (locals.var_qq_op * assign45040_e50193);
        (assign45040_e50194, ((locals.var_qq_op_dn4 * assign45040_e50193) + (locals.var_qq_op * (assign45040_e50193 * (assign45040_e50189 * (locals.var_e2_op_dn4 / locals.var_e2_op))))), ((locals.var_qq_op_dn6 * assign45040_e50193) + (locals.var_qq_op * (assign45040_e50193 * (assign45040_e50189 * (locals.var_e2_op_dn6 / locals.var_e2_op))))), ((locals.var_qq_op_dn7 * assign45040_e50193) + (locals.var_qq_op * (assign45040_e50193 * (assign45040_e50189 * (locals.var_e2_op_dn7 / locals.var_e2_op))))), ((locals.var_qq_op_dn8 * assign45040_e50193) + (locals.var_qq_op * (assign45040_e50193 * (assign45040_e50189 * (locals.var_e2_op_dn8 / locals.var_e2_op))))), ((locals.var_qq_op_dn9 * assign45040_e50193) + (locals.var_qq_op * (assign45040_e50193 * (assign45040_e50189 * (locals.var_e2_op_dn9 / locals.var_e2_op))))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign45040_e50196;
        locals.var_temp2_dn4 = assign45040_e50196_d_n4;
        locals.var_temp2_dn6 = assign45040_e50196_d_n6;
        locals.var_temp2_dn7 = assign45040_e50196_d_n7;
        locals.var_temp2_dn8 = assign45040_e50196_d_n8;
        locals.var_temp2_dn9 = assign45040_e50196_d_n9;

        let (assign45050_e50206, assign45050_e50206_d_n4, assign45050_e50206_d_n6, assign45050_e50206_d_n7, assign45050_e50206_d_n8, assign45050_e50206_d_n9,) = {
    if ((locals.var_guard1360 != 0.0) && (locals.var_guard1363 != 0.0)) {
        let assign45050_e50202: f64 = (1.0 - locals.var_temp1);
        let assign45050_e50204: f64 = (assign45050_e50202 - locals.var_temp2);
        (assign45050_e50204, ((-locals.var_temp1_dn4) - locals.var_temp2_dn4), ((-locals.var_temp1_dn6) - locals.var_temp2_dn6), ((-locals.var_temp1_dn7) - locals.var_temp2_dn7), ((-locals.var_temp1_dn8) - locals.var_temp2_dn8), ((-locals.var_temp1_dn9) - locals.var_temp2_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign45050_e50206;
        locals.var_temp3_dn4 = assign45050_e50206_d_n4;
        locals.var_temp3_dn6 = assign45050_e50206_d_n6;
        locals.var_temp3_dn7 = assign45050_e50206_d_n7;
        locals.var_temp3_dn8 = assign45050_e50206_d_n8;
        locals.var_temp3_dn9 = assign45050_e50206_d_n9;

        let (assign45070_e50228, assign45070_e50228_d_n4, assign45070_e50228_d_n6, assign45070_e50228_d_n7, assign45070_e50228_d_n8, assign45070_e50228_d_n9,) = {
    if ((locals.var_guard1360 != 0.0) && (locals.var_guard1363 != 0.0)) {
        let assign45070_e50220: f64 = (locals.var_k1_1d * locals.var_temp3);
        let assign45070_e50224: f64 = (locals.var_k1_1d * locals.var_temp1);
        let assign45070_e50225: f64 = (1.0 + assign45070_e50224);
        let assign45070_e50226: f64 = (assign45070_e50220 / assign45070_e50225);
        (assign45070_e50226, ((((locals.var_k1_1d * locals.var_temp3_dn4) * assign45070_e50225) - (assign45070_e50220 * (locals.var_k1_1d * locals.var_temp1_dn4))) / (assign45070_e50225 * assign45070_e50225)), ((((locals.var_k1_1d * locals.var_temp3_dn6) * assign45070_e50225) - (assign45070_e50220 * (locals.var_k1_1d * locals.var_temp1_dn6))) / (assign45070_e50225 * assign45070_e50225)), ((((locals.var_k1_1d * locals.var_temp3_dn7) * assign45070_e50225) - (assign45070_e50220 * (locals.var_k1_1d * locals.var_temp1_dn7))) / (assign45070_e50225 * assign45070_e50225)), ((((locals.var_k1_1d * locals.var_temp3_dn8) * assign45070_e50225) - (assign45070_e50220 * (locals.var_k1_1d * locals.var_temp1_dn8))) / (assign45070_e50225 * assign45070_e50225)), ((((locals.var_k1_1d * locals.var_temp3_dn9) * assign45070_e50225) - (assign45070_e50220 * (locals.var_k1_1d * locals.var_temp1_dn9))) / (assign45070_e50225 * assign45070_e50225)),)
    } else {
        (locals.var_k1_1d_qm_op, locals.var_k1_1d_qm_op_dn4, locals.var_k1_1d_qm_op_dn6, locals.var_k1_1d_qm_op_dn7, locals.var_k1_1d_qm_op_dn8, locals.var_k1_1d_qm_op_dn9,)
    }
};
        locals.var_k1_1d_qm_op = assign45070_e50228;
        locals.var_k1_1d_qm_op_dn4 = assign45070_e50228_d_n4;
        locals.var_k1_1d_qm_op_dn6 = assign45070_e50228_d_n6;
        locals.var_k1_1d_qm_op_dn7 = assign45070_e50228_d_n7;
        locals.var_k1_1d_qm_op_dn8 = assign45070_e50228_d_n8;
        locals.var_k1_1d_qm_op_dn9 = assign45070_e50228_d_n9;

        let (assign45080_e50242, assign45080_e50242_d_n4, assign45080_e50242_d_n6, assign45080_e50242_d_n7, assign45080_e50242_d_n8, assign45080_e50242_d_n9,) = {
    if ((locals.var_guard1360 != 0.0) && (locals.var_guard1363 != 0.0)) {
        let assign45080_e50234: f64 = (locals.var_k2_1d * locals.var_temp3);
        let assign45080_e50238: f64 = (locals.var_k2_1d * locals.var_temp2);
        let assign45080_e50239: f64 = (1.0 + assign45080_e50238);
        let assign45080_e50240: f64 = (assign45080_e50234 / assign45080_e50239);
        (assign45080_e50240, ((((locals.var_k2_1d * locals.var_temp3_dn4) * assign45080_e50239) - (assign45080_e50234 * (locals.var_k2_1d * locals.var_temp2_dn4))) / (assign45080_e50239 * assign45080_e50239)), ((((locals.var_k2_1d * locals.var_temp3_dn6) * assign45080_e50239) - (assign45080_e50234 * (locals.var_k2_1d * locals.var_temp2_dn6))) / (assign45080_e50239 * assign45080_e50239)), ((((locals.var_k2_1d * locals.var_temp3_dn7) * assign45080_e50239) - (assign45080_e50234 * (locals.var_k2_1d * locals.var_temp2_dn7))) / (assign45080_e50239 * assign45080_e50239)), ((((locals.var_k2_1d * locals.var_temp3_dn8) * assign45080_e50239) - (assign45080_e50234 * (locals.var_k2_1d * locals.var_temp2_dn8))) / (assign45080_e50239 * assign45080_e50239)), ((((locals.var_k2_1d * locals.var_temp3_dn9) * assign45080_e50239) - (assign45080_e50234 * (locals.var_k2_1d * locals.var_temp2_dn9))) / (assign45080_e50239 * assign45080_e50239)),)
    } else {
        (locals.var_k2_1d_qm_op, locals.var_k2_1d_qm_op_dn4, locals.var_k2_1d_qm_op_dn6, locals.var_k2_1d_qm_op_dn7, locals.var_k2_1d_qm_op_dn8, locals.var_k2_1d_qm_op_dn9,)
    }
};
        locals.var_k2_1d_qm_op = assign45080_e50242;
        locals.var_k2_1d_qm_op_dn4 = assign45080_e50242_d_n4;
        locals.var_k2_1d_qm_op_dn6 = assign45080_e50242_d_n6;
        locals.var_k2_1d_qm_op_dn7 = assign45080_e50242_d_n7;
        locals.var_k2_1d_qm_op_dn8 = assign45080_e50242_d_n8;
        locals.var_k2_1d_qm_op_dn9 = assign45080_e50242_d_n9;

        let (assign45090_e50258, assign45090_e50258_d_n4, assign45090_e50258_d_n6, assign45090_e50258_d_n7, assign45090_e50258_d_n8, assign45090_e50258_d_n9,) = {
    if ((locals.var_guard1360 != 0.0) && (locals.var_guard1363 != 0.0)) {
        let assign45090_e50250: f64 = (1.0 / locals.var_k1_1d_qm_op);
        let assign45090_e50251: f64 = (1.0 + assign45090_e50250);
        let assign45090_e50254: f64 = (1.0 / locals.var_k2_1d_qm_op);
        let assign45090_e50255: f64 = (assign45090_e50251 + assign45090_e50254);
        let assign45090_e50256: f64 = (1.0 / assign45090_e50255);
        (assign45090_e50256, (-(((-(locals.var_k1_1d_qm_op_dn4 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn4 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign45090_e50255 * assign45090_e50255))), (-(((-(locals.var_k1_1d_qm_op_dn6 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn6 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign45090_e50255 * assign45090_e50255))), (-(((-(locals.var_k1_1d_qm_op_dn7 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn7 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign45090_e50255 * assign45090_e50255))), (-(((-(locals.var_k1_1d_qm_op_dn8 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn8 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign45090_e50255 * assign45090_e50255))), (-(((-(locals.var_k1_1d_qm_op_dn9 / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + (-(locals.var_k2_1d_qm_op_dn9 / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op)))) / (assign45090_e50255 * assign45090_e50255))),)
    } else {
        (locals.var_keq_1d_qm_op, locals.var_keq_1d_qm_op_dn4, locals.var_keq_1d_qm_op_dn6, locals.var_keq_1d_qm_op_dn7, locals.var_keq_1d_qm_op_dn8, locals.var_keq_1d_qm_op_dn9,)
    }
};
        locals.var_keq_1d_qm_op = assign45090_e50258;
        locals.var_keq_1d_qm_op_dn4 = assign45090_e50258_d_n4;
        locals.var_keq_1d_qm_op_dn6 = assign45090_e50258_d_n6;
        locals.var_keq_1d_qm_op_dn7 = assign45090_e50258_d_n7;
        locals.var_keq_1d_qm_op_dn8 = assign45090_e50258_d_n8;
        locals.var_keq_1d_qm_op_dn9 = assign45090_e50258_d_n9;

        let (assign45110_e50272, assign45110_e50272_d_n4, assign45110_e50272_d_n6, assign45110_e50272_d_n7, assign45110_e50272_d_n8, assign45110_e50272_d_n9,) = {
    if ((locals.var_guard1360 != 0.0) && (locals.var_guard1363 == 0.0)) {
        (locals.var_k1_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_k1_1d_qm_op, locals.var_k1_1d_qm_op_dn4, locals.var_k1_1d_qm_op_dn6, locals.var_k1_1d_qm_op_dn7, locals.var_k1_1d_qm_op_dn8, locals.var_k1_1d_qm_op_dn9,)
    }
};
        locals.var_k1_1d_qm_op = assign45110_e50272;
        locals.var_k1_1d_qm_op_dn4 = assign45110_e50272_d_n4;
        locals.var_k1_1d_qm_op_dn6 = assign45110_e50272_d_n6;
        locals.var_k1_1d_qm_op_dn7 = assign45110_e50272_d_n7;
        locals.var_k1_1d_qm_op_dn8 = assign45110_e50272_d_n8;
        locals.var_k1_1d_qm_op_dn9 = assign45110_e50272_d_n9;

        let (assign45120_e50279, assign45120_e50279_d_n4, assign45120_e50279_d_n6, assign45120_e50279_d_n7, assign45120_e50279_d_n8, assign45120_e50279_d_n9,) = {
    if ((locals.var_guard1360 != 0.0) && (locals.var_guard1363 == 0.0)) {
        (locals.var_k2_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_k2_1d_qm_op, locals.var_k2_1d_qm_op_dn4, locals.var_k2_1d_qm_op_dn6, locals.var_k2_1d_qm_op_dn7, locals.var_k2_1d_qm_op_dn8, locals.var_k2_1d_qm_op_dn9,)
    }
};
        locals.var_k2_1d_qm_op = assign45120_e50279;
        locals.var_k2_1d_qm_op_dn4 = assign45120_e50279_d_n4;
        locals.var_k2_1d_qm_op_dn6 = assign45120_e50279_d_n6;
        locals.var_k2_1d_qm_op_dn7 = assign45120_e50279_d_n7;
        locals.var_k2_1d_qm_op_dn8 = assign45120_e50279_d_n8;
        locals.var_k2_1d_qm_op_dn9 = assign45120_e50279_d_n9;

        let (assign45130_e50286, assign45130_e50286_d_n4, assign45130_e50286_d_n6, assign45130_e50286_d_n7, assign45130_e50286_d_n8, assign45130_e50286_d_n9,) = {
    if ((locals.var_guard1360 != 0.0) && (locals.var_guard1363 == 0.0)) {
        (locals.var_keq_1d, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_keq_1d_qm_op, locals.var_keq_1d_qm_op_dn4, locals.var_keq_1d_qm_op_dn6, locals.var_keq_1d_qm_op_dn7, locals.var_keq_1d_qm_op_dn8, locals.var_keq_1d_qm_op_dn9,)
    }
};
        locals.var_keq_1d_qm_op = assign45130_e50286;
        locals.var_keq_1d_qm_op_dn4 = assign45130_e50286_d_n4;
        locals.var_keq_1d_qm_op_dn6 = assign45130_e50286_d_n6;
        locals.var_keq_1d_qm_op_dn7 = assign45130_e50286_d_n7;
        locals.var_keq_1d_qm_op_dn8 = assign45130_e50286_d_n8;
        locals.var_keq_1d_qm_op_dn9 = assign45130_e50286_d_n9;

        let (assign45140_e50294, assign45140_e50294_d_n4, assign45140_e50294_d_n6, assign45140_e50294_d_n7, assign45140_e50294_d_n8, assign45140_e50294_d_n9,) = {
    if (locals.var_guard1360 != 0.0) {
        let assign45140_e50291: f64 = (locals.var_xg10_op - locals.var_xg2eff_op);
        let assign45140_e50292: f64 = (locals.var_keq_1d_qm_op * assign45140_e50291);
        (assign45140_e50292, ((locals.var_keq_1d_qm_op_dn4 * assign45140_e50291) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn4 - locals.var_xg2eff_op_dn4))), ((locals.var_keq_1d_qm_op_dn6 * assign45140_e50291) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn6 - locals.var_xg2eff_op_dn6))), ((locals.var_keq_1d_qm_op_dn7 * assign45140_e50291) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn7 - locals.var_xg2eff_op_dn7))), ((locals.var_keq_1d_qm_op_dn8 * assign45140_e50291) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn8 - locals.var_xg2eff_op_dn8))), ((locals.var_keq_1d_qm_op_dn9 * assign45140_e50291) + (locals.var_keq_1d_qm_op * (locals.var_xg10_op_dn9 - locals.var_xg2eff_op_dn9))),)
    } else {
        (locals.var_dx_wi_1d_op, locals.var_dx_wi_1d_op_dn4, locals.var_dx_wi_1d_op_dn6, locals.var_dx_wi_1d_op_dn7, locals.var_dx_wi_1d_op_dn8, locals.var_dx_wi_1d_op_dn9,)
    }
};
        locals.var_dx_wi_1d_op = assign45140_e50294;
        locals.var_dx_wi_1d_op_dn4 = assign45140_e50294_d_n4;
        locals.var_dx_wi_1d_op_dn6 = assign45140_e50294_d_n6;
        locals.var_dx_wi_1d_op_dn7 = assign45140_e50294_d_n7;
        locals.var_dx_wi_1d_op_dn8 = assign45140_e50294_d_n8;
        locals.var_dx_wi_1d_op_dn9 = assign45140_e50294_d_n9;

        let assign45150_e50297: f64 = if locals.var_dx_wi_1d_op > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1364 = assign45150_e50297;

        let assign45160_e50299: f64 = (-locals.var_dx_wi_1d_op);
        let assign45160_e50301: f64 = if assign45160_e50299 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1365 = assign45160_e50301;

        let (assign45170_e50314, assign45170_e50314_d_n4, assign45170_e50314_d_n6, assign45170_e50314_d_n7, assign45170_e50314_d_n8, assign45170_e50314_d_n9,) = {
    if (((locals.var_guard1360 != 0.0) && (locals.var_guard1364 != 0.0)) && (locals.var_guard1365 != 0.0)) {
        let assign45170_e50309: f64 = (-locals.var_dx_wi_1d_op);
        let assign45170_e50310: f64 = (assign45170_e50309).exp();
        let assign45170_e50311: f64 = (1.0 + assign45170_e50310);
        let assign45170_e50312: f64 = (assign45170_e50311).ln();
        (assign45170_e50312, ((assign45170_e50310 * (-locals.var_dx_wi_1d_op_dn4)) / assign45170_e50311), ((assign45170_e50310 * (-locals.var_dx_wi_1d_op_dn6)) / assign45170_e50311), ((assign45170_e50310 * (-locals.var_dx_wi_1d_op_dn7)) / assign45170_e50311), ((assign45170_e50310 * (-locals.var_dx_wi_1d_op_dn8)) / assign45170_e50311), ((assign45170_e50310 * (-locals.var_dx_wi_1d_op_dn9)) / assign45170_e50311),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign45170_e50314;
        locals.var_temp_dn4 = assign45170_e50314_d_n4;
        locals.var_temp_dn6 = assign45170_e50314_d_n6;
        locals.var_temp_dn7 = assign45170_e50314_d_n7;
        locals.var_temp_dn8 = assign45170_e50314_d_n8;
        locals.var_temp_dn9 = assign45170_e50314_d_n9;

        let (assign45180_e50324, assign45180_e50324_d_n4, assign45180_e50324_d_n6, assign45180_e50324_d_n7, assign45180_e50324_d_n8, assign45180_e50324_d_n9,) = {
    if (((locals.var_guard1360 != 0.0) && (locals.var_guard1364 != 0.0)) && (locals.var_guard1365 == 0.0)) {
        let assign45180_e50322: f64 = (-locals.var_dx_wi_1d_op);
        (assign45180_e50322, (-locals.var_dx_wi_1d_op_dn4), (-locals.var_dx_wi_1d_op_dn6), (-locals.var_dx_wi_1d_op_dn7), (-locals.var_dx_wi_1d_op_dn8), (-locals.var_dx_wi_1d_op_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign45180_e50324;
        locals.var_temp_dn4 = assign45180_e50324_d_n4;
        locals.var_temp_dn6 = assign45180_e50324_d_n6;
        locals.var_temp_dn7 = assign45180_e50324_d_n7;
        locals.var_temp_dn8 = assign45180_e50324_d_n8;
        locals.var_temp_dn9 = assign45180_e50324_d_n9;

        let (assign45190_e50338, assign45190_e50338_d_n4, assign45190_e50338_d_n6, assign45190_e50338_d_n7, assign45190_e50338_d_n8, assign45190_e50338_d_n9,) = {
    if ((locals.var_guard1360 != 0.0) && (locals.var_guard1364 != 0.0)) {
        let assign45190_e50331: f64 = (locals.var_dx_wi_1d_op / locals.var_k1_1d_qm_op);
        let assign45190_e50332: f64 = (locals.var_xg10_op - assign45190_e50331);
        let assign45190_e50334: f64 = (assign45190_e50332 + locals.var_temp);
        let assign45190_e50336: f64 = (assign45190_e50334 - 0.6931471805599);
        (assign45190_e50336, ((locals.var_xg10_op_dn4 - (((locals.var_dx_wi_1d_op_dn4 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn4)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn4), ((locals.var_xg10_op_dn6 - (((locals.var_dx_wi_1d_op_dn6 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn6)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn6), ((locals.var_xg10_op_dn7 - (((locals.var_dx_wi_1d_op_dn7 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn7)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn7), ((locals.var_xg10_op_dn8 - (((locals.var_dx_wi_1d_op_dn8 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn8)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn8), ((locals.var_xg10_op_dn9 - (((locals.var_dx_wi_1d_op_dn9 * locals.var_k1_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k1_1d_qm_op_dn9)) / (locals.var_k1_1d_qm_op * locals.var_k1_1d_qm_op))) + locals.var_temp_dn9),)
    } else {
        (locals.var_x_wi_1d_op, locals.var_x_wi_1d_op_dn4, locals.var_x_wi_1d_op_dn6, locals.var_x_wi_1d_op_dn7, locals.var_x_wi_1d_op_dn8, locals.var_x_wi_1d_op_dn9,)
    }
};
        locals.var_x_wi_1d_op = assign45190_e50338;
        locals.var_x_wi_1d_op_dn4 = assign45190_e50338_d_n4;
        locals.var_x_wi_1d_op_dn6 = assign45190_e50338_d_n6;
        locals.var_x_wi_1d_op_dn7 = assign45190_e50338_d_n7;
        locals.var_x_wi_1d_op_dn8 = assign45190_e50338_d_n8;
        locals.var_x_wi_1d_op_dn9 = assign45190_e50338_d_n9;

    }

    pub(super) fn stamp_transient_block_121(
        locals: &mut StampLocals,
    ) {
        let assign45200_e50341: f64 = if locals.var_dx_wi_1d_op < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard1366 = assign45200_e50341;

        let (assign45210_e50354, assign45210_e50354_d_n4, assign45210_e50354_d_n6, assign45210_e50354_d_n7, assign45210_e50354_d_n8, assign45210_e50354_d_n9,) = {
    if (((locals.var_guard1360 != 0.0) && (locals.var_guard1364 == 0.0)) && (locals.var_guard1366 != 0.0)) {
        let assign45210_e50350: f64 = (locals.var_dx_wi_1d_op).exp();
        let assign45210_e50351: f64 = (1.0 + assign45210_e50350);
        let assign45210_e50352: f64 = (assign45210_e50351).ln();
        (assign45210_e50352, ((assign45210_e50350 * locals.var_dx_wi_1d_op_dn4) / assign45210_e50351), ((assign45210_e50350 * locals.var_dx_wi_1d_op_dn6) / assign45210_e50351), ((assign45210_e50350 * locals.var_dx_wi_1d_op_dn7) / assign45210_e50351), ((assign45210_e50350 * locals.var_dx_wi_1d_op_dn8) / assign45210_e50351), ((assign45210_e50350 * locals.var_dx_wi_1d_op_dn9) / assign45210_e50351),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign45210_e50354;
        locals.var_temp_dn4 = assign45210_e50354_d_n4;
        locals.var_temp_dn6 = assign45210_e50354_d_n6;
        locals.var_temp_dn7 = assign45210_e50354_d_n7;
        locals.var_temp_dn8 = assign45210_e50354_d_n8;
        locals.var_temp_dn9 = assign45210_e50354_d_n9;

        let (assign45220_e50364, assign45220_e50364_d_n4, assign45220_e50364_d_n6, assign45220_e50364_d_n7, assign45220_e50364_d_n8, assign45220_e50364_d_n9,) = {
    if (((locals.var_guard1360 != 0.0) && (locals.var_guard1364 == 0.0)) && (locals.var_guard1366 == 0.0)) {
        (locals.var_dx_wi_1d_op, locals.var_dx_wi_1d_op_dn4, locals.var_dx_wi_1d_op_dn6, locals.var_dx_wi_1d_op_dn7, locals.var_dx_wi_1d_op_dn8, locals.var_dx_wi_1d_op_dn9,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign45220_e50364;
        locals.var_temp_dn4 = assign45220_e50364_d_n4;
        locals.var_temp_dn6 = assign45220_e50364_d_n6;
        locals.var_temp_dn7 = assign45220_e50364_d_n7;
        locals.var_temp_dn8 = assign45220_e50364_d_n8;
        locals.var_temp_dn9 = assign45220_e50364_d_n9;

        let (assign45230_e50379, assign45230_e50379_d_n4, assign45230_e50379_d_n6, assign45230_e50379_d_n7, assign45230_e50379_d_n8, assign45230_e50379_d_n9,) = {
    if ((locals.var_guard1360 != 0.0) && (locals.var_guard1364 == 0.0)) {
        let assign45230_e50372: f64 = (locals.var_dx_wi_1d_op / locals.var_k2_1d_qm_op);
        let assign45230_e50373: f64 = (locals.var_xg2eff_op + assign45230_e50372);
        let assign45230_e50375: f64 = (assign45230_e50373 + locals.var_temp);
        let assign45230_e50377: f64 = (assign45230_e50375 - 0.6931471805599);
        (assign45230_e50377, ((locals.var_xg2eff_op_dn4 + (((locals.var_dx_wi_1d_op_dn4 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn4)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn4), ((locals.var_xg2eff_op_dn6 + (((locals.var_dx_wi_1d_op_dn6 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn6)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn6), ((locals.var_xg2eff_op_dn7 + (((locals.var_dx_wi_1d_op_dn7 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn7)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn7), ((locals.var_xg2eff_op_dn8 + (((locals.var_dx_wi_1d_op_dn8 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn8)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn8), ((locals.var_xg2eff_op_dn9 + (((locals.var_dx_wi_1d_op_dn9 * locals.var_k2_1d_qm_op) - (locals.var_dx_wi_1d_op * locals.var_k2_1d_qm_op_dn9)) / (locals.var_k2_1d_qm_op * locals.var_k2_1d_qm_op))) + locals.var_temp_dn9),)
    } else {
        (locals.var_x_wi_1d_op, locals.var_x_wi_1d_op_dn4, locals.var_x_wi_1d_op_dn6, locals.var_x_wi_1d_op_dn7, locals.var_x_wi_1d_op_dn8, locals.var_x_wi_1d_op_dn9,)
    }
};
        locals.var_x_wi_1d_op = assign45230_e50379;
        locals.var_x_wi_1d_op_dn4 = assign45230_e50379_d_n4;
        locals.var_x_wi_1d_op_dn6 = assign45230_e50379_d_n6;
        locals.var_x_wi_1d_op_dn7 = assign45230_e50379_d_n7;
        locals.var_x_wi_1d_op_dn8 = assign45230_e50379_d_n8;
        locals.var_x_wi_1d_op_dn9 = assign45230_e50379_d_n9;

        let (assign45240_e50398, assign45240_e50398_d_n4, assign45240_e50398_d_n6, assign45240_e50398_d_n7, assign45240_e50398_d_n8, assign45240_e50398_d_n9,) = {
    if (locals.var_guard1360 != 0.0) {
        let assign45240_e50384: f64 = (locals.var_x_wi_1d_op + locals.var_xth_1d_op);
        let assign45240_e50387: f64 = (locals.var_x_wi_1d_op - locals.var_xth_1d_op);
        let assign45240_e50390: f64 = (locals.var_x_wi_1d_op - locals.var_xth_1d_op);
        let assign45240_e50391: f64 = (assign45240_e50387 * assign45240_e50390);
        let assign45240_e50393: f64 = (assign45240_e50391 + 4.0);
        let assign45240_e50394: f64 = (assign45240_e50393).sqrt();
        let assign45240_e50395: f64 = (assign45240_e50384 - assign45240_e50394);
        let assign45240_e50396: f64 = (0.5 * assign45240_e50395);
        (assign45240_e50396, (0.5 * ((locals.var_x_wi_1d_op_dn4 + locals.var_xth_1d_op_dn4) - ((((locals.var_x_wi_1d_op_dn4 - locals.var_xth_1d_op_dn4) * assign45240_e50390) + (assign45240_e50387 * (locals.var_x_wi_1d_op_dn4 - locals.var_xth_1d_op_dn4))) / (2.0 * assign45240_e50394)))), (0.5 * ((locals.var_x_wi_1d_op_dn6 + locals.var_xth_1d_op_dn6) - ((((locals.var_x_wi_1d_op_dn6 - locals.var_xth_1d_op_dn6) * assign45240_e50390) + (assign45240_e50387 * (locals.var_x_wi_1d_op_dn6 - locals.var_xth_1d_op_dn6))) / (2.0 * assign45240_e50394)))), (0.5 * ((locals.var_x_wi_1d_op_dn7 + locals.var_xth_1d_op_dn7) - ((((locals.var_x_wi_1d_op_dn7 - locals.var_xth_1d_op_dn7) * assign45240_e50390) + (assign45240_e50387 * (locals.var_x_wi_1d_op_dn7 - locals.var_xth_1d_op_dn7))) / (2.0 * assign45240_e50394)))), (0.5 * ((locals.var_x_wi_1d_op_dn8 + locals.var_xth_1d_op_dn8) - ((((locals.var_x_wi_1d_op_dn8 - locals.var_xth_1d_op_dn8) * assign45240_e50390) + (assign45240_e50387 * (locals.var_x_wi_1d_op_dn8 - locals.var_xth_1d_op_dn8))) / (2.0 * assign45240_e50394)))), (0.5 * ((locals.var_x_wi_1d_op_dn9 + locals.var_xth_1d_op_dn9) - ((((locals.var_x_wi_1d_op_dn9 - locals.var_xth_1d_op_dn9) * assign45240_e50390) + (assign45240_e50387 * (locals.var_x_wi_1d_op_dn9 - locals.var_xth_1d_op_dn9))) / (2.0 * assign45240_e50394)))),)
    } else {
        (locals.var_x_1d_op, locals.var_x_1d_op_dn4, locals.var_x_1d_op_dn6, locals.var_x_1d_op_dn7, locals.var_x_1d_op_dn8, locals.var_x_1d_op_dn9,)
    }
};
        locals.var_x_1d_op = assign45240_e50398;
        locals.var_x_1d_op_dn4 = assign45240_e50398_d_n4;
        locals.var_x_1d_op_dn6 = assign45240_e50398_d_n6;
        locals.var_x_1d_op_dn7 = assign45240_e50398_d_n7;
        locals.var_x_1d_op_dn8 = assign45240_e50398_d_n8;
        locals.var_x_1d_op_dn9 = assign45240_e50398_d_n9;

        let (assign45250_e50413, assign45250_e50413_d_n4, assign45250_e50413_d_n6, assign45250_e50413_d_n7, assign45250_e50413_d_n8, assign45250_e50413_d_n9,) = {
    if (locals.var_guard1360 != 0.0) {
        let assign45250_e50404: f64 = (locals.var_xth_1d_op - locals.var_x_1d_op);
        let assign45250_e50405: f64 = (2.0 * assign45250_e50404);
        let assign45250_e50407: f64 = (assign45250_e50405 / locals.var_xsddep_op);
        let assign45250_e50408: f64 = (1.0 + assign45250_e50407);
        let assign45250_e50409: f64 = (assign45250_e50408).sqrt();
        let assign45250_e50411: f64 = (assign45250_e50409 - 1.0);
        (assign45250_e50411, (((((2.0 * (locals.var_xth_1d_op_dn4 - locals.var_x_1d_op_dn4)) * locals.var_xsddep_op) - (assign45250_e50405 * locals.var_xsddep_op_dn4)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign45250_e50409)), (((((2.0 * (locals.var_xth_1d_op_dn6 - locals.var_x_1d_op_dn6)) * locals.var_xsddep_op) - (assign45250_e50405 * locals.var_xsddep_op_dn6)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign45250_e50409)), (((((2.0 * (locals.var_xth_1d_op_dn7 - locals.var_x_1d_op_dn7)) * locals.var_xsddep_op) - (assign45250_e50405 * locals.var_xsddep_op_dn7)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign45250_e50409)), (((((2.0 * (locals.var_xth_1d_op_dn8 - locals.var_x_1d_op_dn8)) * locals.var_xsddep_op) - (assign45250_e50405 * locals.var_xsddep_op_dn8)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign45250_e50409)), (((((2.0 * (locals.var_xth_1d_op_dn9 - locals.var_x_1d_op_dn9)) * locals.var_xsddep_op) - (assign45250_e50405 * locals.var_xsddep_op_dn9)) / (locals.var_xsddep_op * locals.var_xsddep_op)) / (2.0 * assign45250_e50409)),)
    } else {
        (locals.var_dleff_op, locals.var_dleff_op_dn4, locals.var_dleff_op_dn6, locals.var_dleff_op_dn7, locals.var_dleff_op_dn8, locals.var_dleff_op_dn9,)
    }
};
        locals.var_dleff_op = assign45250_e50413;
        locals.var_dleff_op_dn4 = assign45250_e50413_d_n4;
        locals.var_dleff_op_dn6 = assign45250_e50413_d_n6;
        locals.var_dleff_op_dn7 = assign45250_e50413_d_n7;
        locals.var_dleff_op_dn8 = assign45250_e50413_d_n8;
        locals.var_dleff_op_dn9 = assign45250_e50413_d_n9;

        let (assign45270_e50452, assign45270_e50452_d_n4, assign45270_e50452_d_n6, assign45270_e50452_d_n7, assign45270_e50452_d_n8, assign45270_e50452_d_n9,) = {
    if (locals.var_guard1360 != 0.0) {
        let assign45270_e50427: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
        let assign45270_e50428: f64 = (1.0 + assign45270_e50427);
        let assign45270_e50430: f64 = (assign45270_e50428 + 0.5);
        let assign45270_e50434: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
        let assign45270_e50435: f64 = (1.0 + assign45270_e50434);
        let assign45270_e50437: f64 = (assign45270_e50435 - 0.5);
        let assign45270_e50441: f64 = (locals.var_pscedlb_i * locals.var_xg20_op);
        let assign45270_e50442: f64 = (1.0 + assign45270_e50441);
        let assign45270_e50444: f64 = (assign45270_e50442 - 0.5);
        let assign45270_e50445: f64 = (assign45270_e50437 * assign45270_e50444);
        let assign45270_e50447: f64 = (assign45270_e50445 + 0.01);
        let assign45270_e50448: f64 = (assign45270_e50447).sqrt();
        let assign45270_e50449: f64 = (assign45270_e50430 + assign45270_e50448);
        let assign45270_e50450: f64 = (0.5 * assign45270_e50449);
        (assign45270_e50450, (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn4) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn4) * assign45270_e50444) + (assign45270_e50437 * (locals.var_pscedlb_i * locals.var_xg20_op_dn4))) / (2.0 * assign45270_e50448)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn6) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn6) * assign45270_e50444) + (assign45270_e50437 * (locals.var_pscedlb_i * locals.var_xg20_op_dn6))) / (2.0 * assign45270_e50448)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn7) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn7) * assign45270_e50444) + (assign45270_e50437 * (locals.var_pscedlb_i * locals.var_xg20_op_dn7))) / (2.0 * assign45270_e50448)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn8) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn8) * assign45270_e50444) + (assign45270_e50437 * (locals.var_pscedlb_i * locals.var_xg20_op_dn8))) / (2.0 * assign45270_e50448)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20_op_dn9) + ((((locals.var_pscedlb_i * locals.var_xg20_op_dn9) * assign45270_e50444) + (assign45270_e50437 * (locals.var_pscedlb_i * locals.var_xg20_op_dn9))) / (2.0 * assign45270_e50448)))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign45270_e50452;
        locals.var_temp_dn4 = assign45270_e50452_d_n4;
        locals.var_temp_dn6 = assign45270_e50452_d_n6;
        locals.var_temp_dn7 = assign45270_e50452_d_n7;
        locals.var_temp_dn8 = assign45270_e50452_d_n8;
        locals.var_temp_dn9 = assign45270_e50452_d_n9;

        let (assign45300_e50499, assign45300_e50499_d_n4, assign45300_e50499_d_n6, assign45300_e50499_d_n7, assign45300_e50499_d_n8, assign45300_e50499_d_n9,) = {
    if (locals.var_guard1360 != 0.0) {
        let assign45300_e50476: f64 = (2.0 * locals.var_xd0_op);
        let assign45300_e50480: f64 = (locals.var_xdsx_op / locals.var_xd0_op);
        let assign45300_e50481: f64 = (1.0 + assign45300_e50480);
        let assign45300_e50482: f64 = (assign45300_e50481).sqrt();
        let assign45300_e50484: f64 = (assign45300_e50482 - 1.0);
        let assign45300_e50485: f64 = (assign45300_e50476 * assign45300_e50484);
        let assign45300_e50489: f64 = (locals.var_cfdl_i * locals.var_dleff_op);
        let assign45300_e50490: f64 = (1.0 + assign45300_e50489);
        let assign45300_e50491: f64 = (assign45300_e50485 * assign45300_e50490);
        let assign45300_e50495: f64 = (locals.var_cfdlb_i * locals.var_xg20_op);
        let assign45300_e50496: f64 = (1.0 + assign45300_e50495);
        let assign45300_e50497: f64 = (assign45300_e50491 * assign45300_e50496);
        (assign45300_e50497, (((((((2.0 * locals.var_xd0_op_dn4) * assign45300_e50484) + (assign45300_e50476 * ((((locals.var_xdsx_op_dn4 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn4)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign45300_e50482)))) * assign45300_e50490) + (assign45300_e50485 * (locals.var_cfdl_i * locals.var_dleff_op_dn4))) * assign45300_e50496) + (assign45300_e50491 * (locals.var_cfdlb_i * locals.var_xg20_op_dn4))), (((((((2.0 * locals.var_xd0_op_dn6) * assign45300_e50484) + (assign45300_e50476 * ((((locals.var_xdsx_op_dn6 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn6)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign45300_e50482)))) * assign45300_e50490) + (assign45300_e50485 * (locals.var_cfdl_i * locals.var_dleff_op_dn6))) * assign45300_e50496) + (assign45300_e50491 * (locals.var_cfdlb_i * locals.var_xg20_op_dn6))), (((((((2.0 * locals.var_xd0_op_dn7) * assign45300_e50484) + (assign45300_e50476 * ((((locals.var_xdsx_op_dn7 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn7)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign45300_e50482)))) * assign45300_e50490) + (assign45300_e50485 * (locals.var_cfdl_i * locals.var_dleff_op_dn7))) * assign45300_e50496) + (assign45300_e50491 * (locals.var_cfdlb_i * locals.var_xg20_op_dn7))), (((((((2.0 * locals.var_xd0_op_dn8) * assign45300_e50484) + (assign45300_e50476 * ((((locals.var_xdsx_op_dn8 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn8)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign45300_e50482)))) * assign45300_e50490) + (assign45300_e50485 * (locals.var_cfdl_i * locals.var_dleff_op_dn8))) * assign45300_e50496) + (assign45300_e50491 * (locals.var_cfdlb_i * locals.var_xg20_op_dn8))), (((((((2.0 * locals.var_xd0_op_dn9) * assign45300_e50484) + (assign45300_e50476 * ((((locals.var_xdsx_op_dn9 * locals.var_xd0_op) - (locals.var_xdsx_op * locals.var_xd0_op_dn9)) / (locals.var_xd0_op * locals.var_xd0_op)) / (2.0 * assign45300_e50482)))) * assign45300_e50490) + (assign45300_e50485 * (locals.var_cfdl_i * locals.var_dleff_op_dn9))) * assign45300_e50496) + (assign45300_e50491 * (locals.var_cfdlb_i * locals.var_xg20_op_dn9))),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign45300_e50499;
        locals.var_temp_dn4 = assign45300_e50499_d_n4;
        locals.var_temp_dn6 = assign45300_e50499_d_n6;
        locals.var_temp_dn7 = assign45300_e50499_d_n7;
        locals.var_temp_dn8 = assign45300_e50499_d_n8;
        locals.var_temp_dn9 = assign45300_e50499_d_n9;

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let assign00_e727: f64 = (273.15 + p.p15);
        locals.var_tkr = assign00_e727;
        locals.var_tkr_rv = 0.0;

        let assign10_e728: f64 = ctx_temp;
        let assign10_e730: f64 = (assign10_e728 + p.p36);
        let assign10_e732: f64 = (assign10_e730).min(1000.0);
        locals.var_temp = assign10_e732;
        locals.var_temp_dn4 = 0.0;
        locals.var_temp_dn6 = 0.0;
        locals.var_temp_dn7 = 0.0;
        locals.var_temp_dn8 = 0.0;
        locals.var_temp_dn9 = 0.0;
        locals.var_temp_rv = 0.0;

        let assign20_e735: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign20_e735;
        locals.var_guard1_rv = 0.0;

        let (assign30_e766, assign30_e766_d_n4, assign30_e766_d_n6, assign30_e766_d_n7, assign30_e766_d_n8, assign30_e766_d_n9,) = {
    if (locals.var_guard1 != 0.0) {
        let assign30_e742: f64 = (p.p18 * locals.var_temp);
        let assign30_e743: f64 = (p.p17 + assign30_e742);
        let assign30_e744: f64 = (locals.var_temp + assign30_e743);
        let assign30_e749: f64 = (p.p18 * locals.var_temp);
        let assign30_e750: f64 = (p.p17 + assign30_e749);
        let assign30_e751: f64 = (locals.var_temp - assign30_e750);
        let assign30_e756: f64 = (p.p18 * locals.var_temp);
        let assign30_e757: f64 = (p.p17 + assign30_e756);
        let assign30_e758: f64 = (locals.var_temp - assign30_e757);
        let assign30_e759: f64 = (assign30_e751 * assign30_e758);
        let assign30_e761: f64 = (assign30_e759 + p.p19);
        let assign30_e762: f64 = (assign30_e761).sqrt();
        let assign30_e763: f64 = (assign30_e744 + assign30_e762);
        let assign30_e764: f64 = (0.5 * assign30_e763);
        (assign30_e764, (0.5 * ((locals.var_temp_dn4 + (p.p18 * locals.var_temp_dn4)) + ((((locals.var_temp_dn4 - (p.p18 * locals.var_temp_dn4)) * assign30_e758) + (assign30_e751 * (locals.var_temp_dn4 - (p.p18 * locals.var_temp_dn4)))) / (2.0 * assign30_e762)))), (0.5 * ((locals.var_temp_dn6 + (p.p18 * locals.var_temp_dn6)) + ((((locals.var_temp_dn6 - (p.p18 * locals.var_temp_dn6)) * assign30_e758) + (assign30_e751 * (locals.var_temp_dn6 - (p.p18 * locals.var_temp_dn6)))) / (2.0 * assign30_e762)))), (0.5 * ((locals.var_temp_dn7 + (p.p18 * locals.var_temp_dn7)) + ((((locals.var_temp_dn7 - (p.p18 * locals.var_temp_dn7)) * assign30_e758) + (assign30_e751 * (locals.var_temp_dn7 - (p.p18 * locals.var_temp_dn7)))) / (2.0 * assign30_e762)))), (0.5 * ((locals.var_temp_dn8 + (p.p18 * locals.var_temp_dn8)) + ((((locals.var_temp_dn8 - (p.p18 * locals.var_temp_dn8)) * assign30_e758) + (assign30_e751 * (locals.var_temp_dn8 - (p.p18 * locals.var_temp_dn8)))) / (2.0 * assign30_e762)))), (0.5 * ((locals.var_temp_dn9 + (p.p18 * locals.var_temp_dn9)) + ((((locals.var_temp_dn9 - (p.p18 * locals.var_temp_dn9)) * assign30_e758) + (assign30_e751 * (locals.var_temp_dn9 - (p.p18 * locals.var_temp_dn9)))) / (2.0 * assign30_e762)))),)
    } else {
        (locals.var_tkd, locals.var_tkd_dn4, locals.var_tkd_dn6, locals.var_tkd_dn7, locals.var_tkd_dn8, locals.var_tkd_dn9,)
    }
};
        locals.var_tkd = assign30_e766;
        locals.var_tkd_dn4 = assign30_e766_d_n4;
        locals.var_tkd_dn6 = assign30_e766_d_n6;
        locals.var_tkd_dn7 = assign30_e766_d_n7;
        locals.var_tkd_dn8 = assign30_e766_d_n8;
        locals.var_tkd_dn9 = assign30_e766_d_n9;
        locals.var_tkd_rv = 0.0;

        let (assign40_e797, assign40_e797_d_n4, assign40_e797_d_n6, assign40_e797_d_n7, assign40_e797_d_n8, assign40_e797_d_n9,) = {
    if (locals.var_guard1 != 0.0) {
        let assign40_e772: f64 = (locals.var_tkd * 8.617332384961e-5);
        let assign40_e773: f64 = (10.0 / assign40_e772);
        let assign40_e775: f64 = (assign40_e773 + 600.0);
        let assign40_e779: f64 = (locals.var_tkd * 8.617332384961e-5);
        let assign40_e780: f64 = (10.0 / assign40_e779);
        let assign40_e782: f64 = (assign40_e780 - 600.0);
        let assign40_e786: f64 = (locals.var_tkd * 8.617332384961e-5);
        let assign40_e787: f64 = (10.0 / assign40_e786);
        let assign40_e789: f64 = (assign40_e787 - 600.0);
        let assign40_e790: f64 = (assign40_e782 * assign40_e789);
        let assign40_e792: f64 = (assign40_e790 + 0.01);
        let assign40_e793: f64 = (assign40_e792).sqrt();
        let assign40_e794: f64 = (assign40_e775 + assign40_e793);
        let assign40_e795: f64 = (0.5 * assign40_e794);
        (assign40_e795, (0.5 * ((-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))),)
    } else {
        (locals.var_xsatmax, locals.var_xsatmax_dn4, locals.var_xsatmax_dn6, locals.var_xsatmax_dn7, locals.var_xsatmax_dn8, locals.var_xsatmax_dn9,)
    }
};
        locals.var_xsatmax = assign40_e797;
        locals.var_xsatmax_dn4 = assign40_e797_d_n4;
        locals.var_xsatmax_dn6 = assign40_e797_d_n6;
        locals.var_xsatmax_dn7 = assign40_e797_d_n7;
        locals.var_xsatmax_dn8 = assign40_e797_d_n8;
        locals.var_xsatmax_dn9 = assign40_e797_d_n9;
        locals.var_xsatmax_rv = 0.0;

        let (assign50_e817, assign50_e817_d_n4, assign50_e817_d_n6, assign50_e817_d_n7, assign50_e817_d_n8, assign50_e817_d_n9,) = {
    if (locals.var_guard1 == 0.0) {
        let assign50_e803: f64 = (locals.var_temp + 1.0);
        let assign50_e806: f64 = (locals.var_temp - 1.0);
        let assign50_e809: f64 = (locals.var_temp - 1.0);
        let assign50_e810: f64 = (assign50_e806 * assign50_e809);
        let assign50_e812: f64 = (assign50_e810 + 0.001);
        let assign50_e813: f64 = (assign50_e812).sqrt();
        let assign50_e814: f64 = (assign50_e803 + assign50_e813);
        let assign50_e815: f64 = (0.5 * assign50_e814);
        (assign50_e815, (0.5 * (locals.var_temp_dn4 + (((locals.var_temp_dn4 * assign50_e809) + (assign50_e806 * locals.var_temp_dn4)) / (2.0 * assign50_e813)))), (0.5 * (locals.var_temp_dn6 + (((locals.var_temp_dn6 * assign50_e809) + (assign50_e806 * locals.var_temp_dn6)) / (2.0 * assign50_e813)))), (0.5 * (locals.var_temp_dn7 + (((locals.var_temp_dn7 * assign50_e809) + (assign50_e806 * locals.var_temp_dn7)) / (2.0 * assign50_e813)))), (0.5 * (locals.var_temp_dn8 + (((locals.var_temp_dn8 * assign50_e809) + (assign50_e806 * locals.var_temp_dn8)) / (2.0 * assign50_e813)))), (0.5 * (locals.var_temp_dn9 + (((locals.var_temp_dn9 * assign50_e809) + (assign50_e806 * locals.var_temp_dn9)) / (2.0 * assign50_e813)))),)
    } else {
        (locals.var_tkd, locals.var_tkd_dn4, locals.var_tkd_dn6, locals.var_tkd_dn7, locals.var_tkd_dn8, locals.var_tkd_dn9,)
    }
};
        locals.var_tkd = assign50_e817;
        locals.var_tkd_dn4 = assign50_e817_d_n4;
        locals.var_tkd_dn6 = assign50_e817_d_n6;
        locals.var_tkd_dn7 = assign50_e817_d_n7;
        locals.var_tkd_dn8 = assign50_e817_d_n8;
        locals.var_tkd_dn9 = assign50_e817_d_n9;
        locals.var_tkd_rv = 0.0;

        let (assign60_e822, assign60_e822_d_n4, assign60_e822_d_n6, assign60_e822_d_n7, assign60_e822_d_n8, assign60_e822_d_n9,) = {
    if (locals.var_guard1 == 0.0) {
        (600.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xsatmax, locals.var_xsatmax_dn4, locals.var_xsatmax_dn6, locals.var_xsatmax_dn7, locals.var_xsatmax_dn8, locals.var_xsatmax_dn9,)
    }
};
        locals.var_xsatmax = assign60_e822;
        locals.var_xsatmax_dn4 = assign60_e822_d_n4;
        locals.var_xsatmax_dn6 = assign60_e822_d_n6;
        locals.var_xsatmax_dn7 = assign60_e822_d_n7;
        locals.var_xsatmax_dn8 = assign60_e822_d_n8;
        locals.var_xsatmax_dn9 = assign60_e822_d_n9;
        locals.var_xsatmax_rv = 0.0;

        let assign70_e837: f64 = if (((p.p0 == 0.0) && (p.p172 > 0.0)) || ((p.p0 > 0.0) && (p.p439 > 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard2 = assign70_e837;
        locals.var_guard2_rv = 0.0;

        let (assign80_e841,) = {
    if (locals.var_guard2 != 0.0) {
        (p.p5,)
    } else {
        (locals.var_swshe_i,)
    }
};
        locals.var_swshe_i = assign80_e841;
        locals.var_swshe_i_rv = 0.0;

        let (assign90_e846,) = {
    if (locals.var_guard2 == 0.0) {
        (0.0,)
    } else {
        (locals.var_swshe_i,)
    }
};
        locals.var_swshe_i = assign90_e846;
        locals.var_swshe_i_rv = 0.0;

        locals.var_dtc = 0.0;
        locals.var_dtc_dn4 = 0.0;
        locals.var_dtc_rv = 0.0;

        locals.var_tkc = locals.var_tkd;
        locals.var_tkc_dn4 = locals.var_tkd_dn4;
        locals.var_tkc_dn6 = locals.var_tkd_dn6;
        locals.var_tkc_dn7 = locals.var_tkd_dn7;
        locals.var_tkc_dn8 = locals.var_tkd_dn8;
        locals.var_tkc_dn9 = locals.var_tkd_dn9;
        locals.var_tkc_rv = 0.0;

        let assign140_e856: f64 = (locals.var_tkc * locals.var_tkc);
        locals.var_tkc_sq = assign140_e856;
        locals.var_tkc_sq_dn4 = ((locals.var_tkc_dn4 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn4));
        locals.var_tkc_sq_dn6 = ((locals.var_tkc_dn6 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn6));
        locals.var_tkc_sq_dn7 = ((locals.var_tkc_dn7 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn7));
        locals.var_tkc_sq_dn8 = ((locals.var_tkc_dn8 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn8));
        locals.var_tkc_sq_dn9 = ((locals.var_tkc_dn9 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn9));
        locals.var_tkc_sq_rv = 0.0;

        let assign150_e859: f64 = (locals.var_tkc - locals.var_tkr);
        locals.var_dt = assign150_e859;
        locals.var_dt_dn4 = locals.var_tkc_dn4;
        locals.var_dt_dn6 = locals.var_tkc_dn6;
        locals.var_dt_dn7 = locals.var_tkc_dn7;
        locals.var_dt_dn8 = locals.var_tkc_dn8;
        locals.var_dt_dn9 = locals.var_tkc_dn9;
        locals.var_dt_rv = 0.0;

        let assign160_e862: f64 = (locals.var_tkc / locals.var_tkr);
        locals.var_rt = assign160_e862;
        locals.var_rt_dn4 = (locals.var_tkc_dn4 / locals.var_tkr);
        locals.var_rt_dn6 = (locals.var_tkc_dn6 / locals.var_tkr);
        locals.var_rt_dn7 = (locals.var_tkc_dn7 / locals.var_tkr);
        locals.var_rt_dn8 = (locals.var_tkc_dn8 / locals.var_tkr);
        locals.var_rt_dn9 = (locals.var_tkc_dn9 / locals.var_tkr);
        locals.var_rt_rv = 0.0;

        let assign170_e865: f64 = (locals.var_tkr / locals.var_tkc);
        locals.var_rtn = assign170_e865;
        locals.var_rtn_dn4 = (-((locals.var_tkr * locals.var_tkc_dn4) / (locals.var_tkc * locals.var_tkc)));
        locals.var_rtn_dn6 = (-((locals.var_tkr * locals.var_tkc_dn6) / (locals.var_tkc * locals.var_tkc)));
        locals.var_rtn_dn7 = (-((locals.var_tkr * locals.var_tkc_dn7) / (locals.var_tkc * locals.var_tkc)));
        locals.var_rtn_dn8 = (-((locals.var_tkr * locals.var_tkc_dn8) / (locals.var_tkc * locals.var_tkc)));
        locals.var_rtn_dn9 = (-((locals.var_tkr * locals.var_tkc_dn9) / (locals.var_tkc * locals.var_tkc)));
        locals.var_rtn_rv = 0.0;

        let assign180_e868: f64 = (locals.var_tkc * 8.617332384961e-5);
        locals.var_phit0 = assign180_e868;
        locals.var_phit0_dn4 = (locals.var_tkc_dn4 * 8.617332384961e-5);
        locals.var_phit0_dn6 = (locals.var_tkc_dn6 * 8.617332384961e-5);
        locals.var_phit0_dn7 = (locals.var_tkc_dn7 * 8.617332384961e-5);
        locals.var_phit0_dn8 = (locals.var_tkc_dn8 * 8.617332384961e-5);
        locals.var_phit0_dn9 = (locals.var_tkc_dn9 * 8.617332384961e-5);
        locals.var_phit0_rv = 0.0;

        let assign190_e871: f64 = (1.0 / locals.var_phit0);
        locals.var_inv_phit0 = assign190_e871;
        locals.var_inv_phit0_dn4 = (-(locals.var_phit0_dn4 / (locals.var_phit0 * locals.var_phit0)));
        locals.var_inv_phit0_dn6 = (-(locals.var_phit0_dn6 / (locals.var_phit0 * locals.var_phit0)));
        locals.var_inv_phit0_dn7 = (-(locals.var_phit0_dn7 / (locals.var_phit0 * locals.var_phit0)));
        locals.var_inv_phit0_dn8 = (-(locals.var_phit0_dn8 / (locals.var_phit0 * locals.var_phit0)));
        locals.var_inv_phit0_dn9 = (-(locals.var_phit0_dn9 / (locals.var_phit0 * locals.var_phit0)));
        locals.var_inv_phit0_rv = 0.0;

        let assign200_e874: f64 = if p.p0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard83 = assign200_e874;
        locals.var_guard83_rv = 0.0;

        let (assign210_e878,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p23,)
    } else {
        (locals.var_adrain_i,)
    }
};
        locals.var_adrain_i = assign210_e878;
        locals.var_adrain_i_rv = 0.0;

        let (assign220_e882,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p22,)
    } else {
        (locals.var_asource_i,)
    }
};
        locals.var_asource_i = assign220_e882;
        locals.var_asource_i_rv = 0.0;

        let (assign230_e886,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p25,)
    } else {
        (locals.var_pdrain_i,)
    }
};
        locals.var_pdrain_i = assign230_e886;
        locals.var_pdrain_i_rv = 0.0;

        let (assign240_e890,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p24,)
    } else {
        (locals.var_psource_i,)
    }
};
        locals.var_psource_i = assign240_e890;
        locals.var_psource_i_rv = 0.0;

        let (assign250_e894,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p30,)
    } else {
        (locals.var_mult_i_int,)
    }
};
        locals.var_mult_i_int = assign250_e894;
        locals.var_mult_i_int_rv = 0.0;

        let (assign260_e898,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p41,)
    } else {
        (locals.var_tox1_i,)
    }
};
        locals.var_tox1_i = assign260_e898;
        locals.var_tox1_i_rv = 0.0;

        let (assign270_e902,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p42,)
    } else {
        (locals.var_tsi_i,)
    }
};
        locals.var_tsi_i = assign270_e902;
        locals.var_tsi_i_rv = 0.0;

        let (assign280_e906,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p43,)
    } else {
        (locals.var_xge_i,)
    }
};
        locals.var_xge_i = assign280_e906;
        locals.var_xge_i_rv = 0.0;

        let (assign290_e910,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p44,)
    } else {
        (locals.var_tox2_i,)
    }
};
        locals.var_tox2_i = assign290_e910;
        locals.var_tox2_i_rv = 0.0;

        let (assign300_e914,) = {
    if (locals.var_guard83 != 0.0) {
        (1.0,)
    } else {
        (locals.var_typech_i,)
    }
};
        locals.var_typech_i = assign300_e914;
        locals.var_typech_i_rv = 0.0;

        let assign310_e917: f64 = if p.p45 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard84 = assign310_e917;
        locals.var_guard84_rv = 0.0;

        let (assign320_e924,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard84 != 0.0)) {
        let assign320_e922: f64 = (-1.0);
        (assign320_e922,)
    } else {
        (locals.var_typech_i,)
    }
};
        locals.var_typech_i = assign320_e924;
        locals.var_typech_i_rv = 0.0;

        let (assign330_e933,) = {
    if (locals.var_guard83 != 0.0) {
        let assign330_e927: f64 = (p.p45).abs();
        let assign330_e929: f64 = (assign330_e927).min(1e19);
        let assign330_e931: f64 = (assign330_e929 * 1000000.0);
        (assign330_e931,)
    } else {
        (locals.var_nch_i,)
    }
};
        locals.var_nch_i = assign330_e933;
        locals.var_nch_i_rv = 0.0;

        let (assign340_e937,) = {
    if (locals.var_guard83 != 0.0) {
        (1.0,)
    } else {
        (locals.var_typesub_i,)
    }
};
        locals.var_typesub_i = assign340_e937;
        locals.var_typesub_i_rv = 0.0;

        let assign350_e940: f64 = if p.p46 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard85 = assign350_e940;
        locals.var_guard85_rv = 0.0;

        let (assign360_e947,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign360_e945: f64 = (-1.0);
        (assign360_e945,)
    } else {
        (locals.var_typesub_i,)
    }
};
        locals.var_typesub_i = assign360_e947;
        locals.var_typesub_i_rv = 0.0;

        let (assign370_e958,) = {
    if (locals.var_guard83 != 0.0) {
        let assign370_e950: f64 = (p.p46).abs();
        let assign370_e952: f64 = (assign370_e950).max(1e16);
        let assign370_e954: f64 = (assign370_e952).min(1e21);
        let assign370_e956: f64 = (assign370_e954 * 1000000.0);
        (assign370_e956,)
    } else {
        (locals.var_nsub_i,)
    }
};
        locals.var_nsub_i = assign370_e958;
        locals.var_nsub_i_rv = 0.0;

        let (assign380_e962,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p47,)
    } else {
        (locals.var_ct_i,)
    }
};
        locals.var_ct_i = assign380_e962;
        locals.var_ct_i_rv = 0.0;

        let (assign390_e966,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p48,)
    } else {
        (locals.var_toxp_i,)
    }
};
        locals.var_toxp_i = assign390_e966;
        locals.var_toxp_i_rv = 0.0;

        let (assign400_e972,) = {
    if (locals.var_guard83 != 0.0) {
        let assign400_e970: f64 = (p.p49 * 1000000.0);
        (assign400_e970,)
    } else {
        (locals.var_nov_i,)
    }
};
        locals.var_nov_i = assign400_e972;
        locals.var_nov_i_rv = 0.0;

        let (assign410_e978,) = {
    if (locals.var_guard83 != 0.0) {
        let assign410_e976: f64 = (p.p50 * 1000000.0);
        (assign410_e976,)
    } else {
        (locals.var_novd_i,)
    }
};
        locals.var_novd_i = assign410_e978;
        locals.var_novd_i_rv = 0.0;

        let (assign420_e982, assign420_e982_d_n4, assign420_e982_d_n6, assign420_e982_d_n7, assign420_e982_d_n8, assign420_e982_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p51, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9,)
    }
};
        locals.var_vfb1_t = assign420_e982;
        locals.var_vfb1_t_dn4 = assign420_e982_d_n4;
        locals.var_vfb1_t_dn6 = assign420_e982_d_n6;
        locals.var_vfb1_t_dn7 = assign420_e982_d_n7;
        locals.var_vfb1_t_dn8 = assign420_e982_d_n8;
        locals.var_vfb1_t_dn9 = assign420_e982_d_n9;
        locals.var_vfb1_t_rv = 0.0;

        let (assign430_e986, assign430_e986_d_n4, assign430_e986_d_n6, assign430_e986_d_n7, assign430_e986_d_n8, assign430_e986_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p52, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9,)
    }
};
        locals.var_vfb2_t = assign430_e986;
        locals.var_vfb2_t_dn4 = assign430_e986_d_n4;
        locals.var_vfb2_t_dn6 = assign430_e986_d_n6;
        locals.var_vfb2_t_dn7 = assign430_e986_d_n7;
        locals.var_vfb2_t_dn8 = assign430_e986_d_n8;
        locals.var_vfb2_t_dn9 = assign430_e986_d_n9;
        locals.var_vfb2_t_rv = 0.0;

        let (assign440_e990,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p53,)
    } else {
        (locals.var_stvfb_i,)
    }
};
        locals.var_stvfb_i = assign440_e990;
        locals.var_stvfb_i_rv = 0.0;

        let (assign450_e996, assign450_e996_d_n4, assign450_e996_d_n6, assign450_e996_d_n7, assign450_e996_d_n8, assign450_e996_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        let assign450_e994: f64 = (p.p54 * 1000000.0);
        (assign450_e994, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_np_i, locals.var_np_i_dn4, locals.var_np_i_dn6, locals.var_np_i_dn7, locals.var_np_i_dn8, locals.var_np_i_dn9,)
    }
};
        locals.var_np_i = assign450_e996;
        locals.var_np_i_dn4 = assign450_e996_d_n4;
        locals.var_np_i_dn6 = assign450_e996_d_n6;
        locals.var_np_i_dn7 = assign450_e996_d_n7;
        locals.var_np_i_dn8 = assign450_e996_d_n8;
        locals.var_np_i_dn9 = assign450_e996_d_n9;
        locals.var_np_i_rv = 0.0;

        let (assign460_e1000,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p55,)
    } else {
        (locals.var_cic1_i,)
    }
};
        locals.var_cic1_i = assign460_e1000;
        locals.var_cic1_i_rv = 0.0;

        let (assign470_e1004,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p56,)
    } else {
        (locals.var_cic2_i,)
    }
};
        locals.var_cic2_i = assign470_e1004;
        locals.var_cic2_i_rv = 0.0;

        let (assign480_e1008,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p57,)
    } else {
        (locals.var_psce1_i,)
    }
};
        locals.var_psce1_i = assign480_e1008;
        locals.var_psce1_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign490_e1018,) = {
    if (locals.var_guard83 != 0.0) {
        let assign490_e1012: f64 = (p.p58 * locals.var_psce1_i);
        let assign490_e1014: f64 = (assign490_e1012 * locals.var_tox2_i);
        let assign490_e1016: f64 = (assign490_e1014 / locals.var_tox1_i);
        (assign490_e1016,)
    } else {
        (locals.var_psce2_i,)
    }
};
        locals.var_psce2_i = assign490_e1018;
        locals.var_psce2_i_rv = 0.0;

        let (assign500_e1024,) = {
    if (locals.var_guard83 != 0.0) {
        let assign500_e1022: f64 = (p.p59 * 1000000.0);
        (assign500_e1022,)
    } else {
        (locals.var_nsddc_i,)
    }
};
        locals.var_nsddc_i = assign500_e1024;
        locals.var_nsddc_i_rv = 0.0;

        let (assign510_e1028,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p60,)
    } else {
        (locals.var_pscedlb_i,)
    }
};
        locals.var_pscedlb_i = assign510_e1028;
        locals.var_pscedlb_i_rv = 0.0;

        let (assign520_e1032,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p61,)
    } else {
        (locals.var_pnce_i,)
    }
};
        locals.var_pnce_i = assign520_e1032;
        locals.var_pnce_i_rv = 0.0;

        let (assign530_e1036, assign530_e1036_d_n4, assign530_e1036_d_n6, assign530_e1036_d_n7, assign530_e1036_d_n8, assign530_e1036_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p62, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9,)
    }
};
        locals.var_cf1_t = assign530_e1036;
        locals.var_cf1_t_dn4 = assign530_e1036_d_n4;
        locals.var_cf1_t_dn6 = assign530_e1036_d_n6;
        locals.var_cf1_t_dn7 = assign530_e1036_d_n7;
        locals.var_cf1_t_dn8 = assign530_e1036_d_n8;
        locals.var_cf1_t_dn9 = assign530_e1036_d_n9;
        locals.var_cf1_t_rv = 0.0;

        let (assign540_e1046, assign540_e1046_d_n4, assign540_e1046_d_n6, assign540_e1046_d_n7, assign540_e1046_d_n8, assign540_e1046_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        let assign540_e1040: f64 = (p.p63 * locals.var_cf1_t);
        let assign540_e1042: f64 = (assign540_e1040 * locals.var_tox2_i);
        let assign540_e1044: f64 = (assign540_e1042 / locals.var_tox1_i);
        (assign540_e1044, (((p.p63 * locals.var_cf1_t_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cf1_t_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cf1_t_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cf1_t_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cf1_t_dn9) * locals.var_tox2_i) / locals.var_tox1_i),)
    } else {
        (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9,)
    }
};
        locals.var_cf2_t = assign540_e1046;
        locals.var_cf2_t_dn4 = assign540_e1046_d_n4;
        locals.var_cf2_t_dn6 = assign540_e1046_d_n6;
        locals.var_cf2_t_dn7 = assign540_e1046_d_n7;
        locals.var_cf2_t_dn8 = assign540_e1046_d_n8;
        locals.var_cf2_t_dn9 = assign540_e1046_d_n9;
        locals.var_cf2_t_rv = 0.0;

        let (assign550_e1050, assign550_e1050_d_n4, assign550_e1050_d_n6, assign550_e1050_d_n7, assign550_e1050_d_n8, assign550_e1050_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p64, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_stcf_i, locals.var_stcf_i_dn4, locals.var_stcf_i_dn6, locals.var_stcf_i_dn7, locals.var_stcf_i_dn8, locals.var_stcf_i_dn9,)
    }
};
        locals.var_stcf_i = assign550_e1050;
        locals.var_stcf_i_dn4 = assign550_e1050_d_n4;
        locals.var_stcf_i_dn6 = assign550_e1050_d_n6;
        locals.var_stcf_i_dn7 = assign550_e1050_d_n7;
        locals.var_stcf_i_dn8 = assign550_e1050_d_n8;
        locals.var_stcf_i_dn9 = assign550_e1050_d_n9;
        locals.var_stcf_i_rv = 0.0;

        let (assign560_e1054,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p65,)
    } else {
        (locals.var_cfd_i,)
    }
};
        locals.var_cfd_i = assign560_e1054;
        locals.var_cfd_i_rv = 0.0;

        let (assign570_e1058,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p66,)
    } else {
        (locals.var_cfdl_i,)
    }
};
        locals.var_cfdl_i = assign570_e1058;
        locals.var_cfdl_i_rv = 0.0;

        let (assign580_e1062,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p67,)
    } else {
        (locals.var_cfdlb_i,)
    }
};
        locals.var_cfdlb_i = assign580_e1062;
        locals.var_cfdlb_i_rv = 0.0;

        let (assign590_e1066, assign590_e1066_d_n4, assign590_e1066_d_n6, assign590_e1066_d_n7, assign590_e1066_d_n8, assign590_e1066_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p68, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_betn1_t, locals.var_betn1_t_dn4, locals.var_betn1_t_dn6, locals.var_betn1_t_dn7, locals.var_betn1_t_dn8, locals.var_betn1_t_dn9,)
    }
};
        locals.var_betn1_t = assign590_e1066;
        locals.var_betn1_t_dn4 = assign590_e1066_d_n4;
        locals.var_betn1_t_dn6 = assign590_e1066_d_n6;
        locals.var_betn1_t_dn7 = assign590_e1066_d_n7;
        locals.var_betn1_t_dn8 = assign590_e1066_d_n8;
        locals.var_betn1_t_dn9 = assign590_e1066_d_n9;
        locals.var_betn1_t_rv = 0.0;

        let (assign600_e1072, assign600_e1072_d_n4, assign600_e1072_d_n6, assign600_e1072_d_n7, assign600_e1072_d_n8, assign600_e1072_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        let assign600_e1070: f64 = (p.p69 * locals.var_betn1_t);
        (assign600_e1070, (p.p69 * locals.var_betn1_t_dn4), (p.p69 * locals.var_betn1_t_dn6), (p.p69 * locals.var_betn1_t_dn7), (p.p69 * locals.var_betn1_t_dn8), (p.p69 * locals.var_betn1_t_dn9),)
    } else {
        (locals.var_betn2_t, locals.var_betn2_t_dn4, locals.var_betn2_t_dn6, locals.var_betn2_t_dn7, locals.var_betn2_t_dn8, locals.var_betn2_t_dn9,)
    }
};
        locals.var_betn2_t = assign600_e1072;
        locals.var_betn2_t_dn4 = assign600_e1072_d_n4;
        locals.var_betn2_t_dn6 = assign600_e1072_d_n6;
        locals.var_betn2_t_dn7 = assign600_e1072_d_n7;
        locals.var_betn2_t_dn8 = assign600_e1072_d_n8;
        locals.var_betn2_t_dn9 = assign600_e1072_d_n9;
        locals.var_betn2_t_rv = 0.0;

        let (assign610_e1076,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p70,)
    } else {
        (locals.var_stbet_i,)
    }
};
        locals.var_stbet_i = assign610_e1076;
        locals.var_stbet_i_rv = 0.0;

        let (assign620_e1080,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p71,)
    } else {
        (locals.var_cs_t,)
    }
};
        locals.var_cs_t = assign620_e1080;
        locals.var_cs_t_rv = 0.0;

        let (assign630_e1084,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p72,)
    } else {
        (locals.var_csfi_i,)
    }
};
        locals.var_csfi_i = assign630_e1084;
        locals.var_csfi_i_rv = 0.0;

        let (assign640_e1088,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p73,)
    } else {
        (locals.var_csbi_i,)
    }
};
        locals.var_csbi_i = assign640_e1088;
        locals.var_csbi_i_rv = 0.0;

        let (assign650_e1092,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p74,)
    } else {
        (locals.var_stcs_i,)
    }
};
        locals.var_stcs_i = assign650_e1092;
        locals.var_stcs_i_rv = 0.0;

        let (assign660_e1096,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p75,)
    } else {
        (locals.var_thecs_t,)
    }
};
        locals.var_thecs_t = assign660_e1096;
        locals.var_thecs_t_rv = 0.0;

        let (assign670_e1100,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p76,)
    } else {
        (locals.var_stthecs_i,)
    }
};
        locals.var_stthecs_i = assign670_e1100;
        locals.var_stthecs_i_rv = 0.0;

        let (assign680_e1104,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p77,)
    } else {
        (locals.var_csthr_i,)
    }
};
        locals.var_csthr_i = assign680_e1104;
        locals.var_csthr_i_rv = 0.0;

        let (assign690_e1108,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p78,)
    } else {
        (locals.var_csthrb_i,)
    }
};
        locals.var_csthrb_i = assign690_e1108;
        locals.var_csthrb_i_rv = 0.0;

        let (assign700_e1112,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p79,)
    } else {
        (locals.var_mue_t,)
    }
};
        locals.var_mue_t = assign700_e1112;
        locals.var_mue_t_rv = 0.0;

        let (assign710_e1116,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p80,)
    } else {
        (locals.var_stmue_i,)
    }
};
        locals.var_stmue_i = assign710_e1116;
        locals.var_stmue_i_rv = 0.0;

        let (assign720_e1120,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p81,)
    } else {
        (locals.var_themu_t,)
    }
};
        locals.var_themu_t = assign720_e1120;
        locals.var_themu_t_rv = 0.0;

        let (assign730_e1124,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p82,)
    } else {
        (locals.var_stthemu_i,)
    }
};
        locals.var_stthemu_i = assign730_e1124;
        locals.var_stthemu_i_rv = 0.0;

        let (assign740_e1128,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p83,)
    } else {
        (locals.var_xcor_t,)
    }
};
        locals.var_xcor_t = assign740_e1128;
        locals.var_xcor_t_rv = 0.0;

        let (assign750_e1132,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p84,)
    } else {
        (locals.var_xcorb_i,)
    }
};
        locals.var_xcorb_i = assign750_e1132;
        locals.var_xcorb_i_rv = 0.0;

        let (assign760_e1136,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p85,)
    } else {
        (locals.var_stxcor_i,)
    }
};
        locals.var_stxcor_i = assign760_e1136;
        locals.var_stxcor_i_rv = 0.0;

        let (assign770_e1140,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p86,)
    } else {
        (locals.var_feta_i,)
    }
};
        locals.var_feta_i = assign770_e1140;
        locals.var_feta_i_rv = 0.0;

        let (assign780_e1144,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p87,)
    } else {
        (locals.var_rs_t,)
    }
};
        locals.var_rs_t = assign780_e1144;
        locals.var_rs_t_rv = 0.0;

        let (assign790_e1148,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p88,)
    } else {
        (locals.var_rsig_i,)
    }
};
        locals.var_rsig_i = assign790_e1148;
        locals.var_rsig_i_rv = 0.0;

        let (assign800_e1152,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p89,)
    } else {
        (locals.var_strs_i,)
    }
};
        locals.var_strs_i = assign800_e1152;
        locals.var_strs_i_rv = 0.0;

        let (assign810_e1156,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p90,)
    } else {
        (locals.var_rsg_i,)
    }
};
        locals.var_rsg_i = assign810_e1156;
        locals.var_rsg_i_rv = 0.0;

        let (assign820_e1160,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p91,)
    } else {
        (locals.var_thersg_i,)
    }
};
        locals.var_thersg_i = assign820_e1160;
        locals.var_thersg_i_rv = 0.0;

        let (assign830_e1164,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p92,)
    } else {
        (locals.var_rsb_i,)
    }
};
        locals.var_rsb_i = assign830_e1164;
        locals.var_rsb_i_rv = 0.0;

        let (assign840_e1168, assign840_e1168_d_n4, assign840_e1168_d_n6, assign840_e1168_d_n7, assign840_e1168_d_n8, assign840_e1168_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p93, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_thesat_t, locals.var_thesat_t_dn4, locals.var_thesat_t_dn6, locals.var_thesat_t_dn7, locals.var_thesat_t_dn8, locals.var_thesat_t_dn9,)
    }
};
        locals.var_thesat_t = assign840_e1168;
        locals.var_thesat_t_dn4 = assign840_e1168_d_n4;
        locals.var_thesat_t_dn6 = assign840_e1168_d_n6;
        locals.var_thesat_t_dn7 = assign840_e1168_d_n7;
        locals.var_thesat_t_dn8 = assign840_e1168_d_n8;
        locals.var_thesat_t_dn9 = assign840_e1168_d_n9;
        locals.var_thesat_t_rv = 0.0;

        let (assign850_e1172,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p94,)
    } else {
        (locals.var_stthesat_i,)
    }
};
        locals.var_stthesat_i = assign850_e1172;
        locals.var_stthesat_i_rv = 0.0;

        let (assign860_e1176,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p95,)
    } else {
        (locals.var_thesat1_i,)
    }
};
        locals.var_thesat1_i = assign860_e1176;
        locals.var_thesat1_i_rv = 0.0;

        let (assign870_e1180,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p96,)
    } else {
        (locals.var_thesat2_i,)
    }
};
        locals.var_thesat2_i = assign870_e1180;
        locals.var_thesat2_i_rv = 0.0;

        let (assign880_e1184,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p97,)
    } else {
        (locals.var_ax_i,)
    }
};
        locals.var_ax_i = assign880_e1184;
        locals.var_ax_i_rv = 0.0;

        let (assign890_e1188,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p98,)
    } else {
        (locals.var_alp_i,)
    }
};
        locals.var_alp_i = assign890_e1188;
        locals.var_alp_i_rv = 0.0;

        let (assign900_e1192,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p99,)
    } else {
        (locals.var_alp1_i,)
    }
};
        locals.var_alp1_i = assign900_e1192;
        locals.var_alp1_i_rv = 0.0;

        let (assign910_e1196,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p100,)
    } else {
        (locals.var_alpb_i,)
    }
};
        locals.var_alpb_i = assign910_e1196;
        locals.var_alpb_i_rv = 0.0;

        let (assign920_e1200,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p101,)
    } else {
        (locals.var_vp_i,)
    }
};
        locals.var_vp_i = assign920_e1200;
        locals.var_vp_i_rv = 0.0;

        let (assign930_e1204,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p102,)
    } else {
        (locals.var_vpg_i,)
    }
};
        locals.var_vpg_i = assign930_e1204;
        locals.var_vpg_i_rv = 0.0;

        let (assign940_e1208,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p103,)
    } else {
        (locals.var_gco_i,)
    }
};
        locals.var_gco_i = assign940_e1208;
        locals.var_gco_i_rv = 0.0;

        let (assign950_e1212,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p104,)
    } else {
        (locals.var_iginv_t,)
    }
};
        locals.var_iginv_t = assign950_e1212;
        locals.var_iginv_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign960_e1216,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p105,)
    } else {
        (locals.var_igovinv_t,)
    }
};
        locals.var_igovinv_t = assign960_e1216;
        locals.var_igovinv_t_rv = 0.0;

        let (assign970_e1220,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p106,)
    } else {
        (locals.var_igovinvd_t,)
    }
};
        locals.var_igovinvd_t = assign970_e1220;
        locals.var_igovinvd_t_rv = 0.0;

        let (assign1000_e1232,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p107,)
    } else {
        (locals.var_igovacc_t,)
    }
};
        locals.var_igovacc_t = assign1000_e1232;
        locals.var_igovacc_t_rv = 0.0;

        let (assign1010_e1236,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p108,)
    } else {
        (locals.var_igovaccd_t,)
    }
};
        locals.var_igovaccd_t = assign1010_e1236;
        locals.var_igovaccd_t_rv = 0.0;

        let (assign1020_e1240,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p109,)
    } else {
        (locals.var_stig_i,)
    }
};
        locals.var_stig_i = assign1020_e1240;
        locals.var_stig_i_rv = 0.0;

        let (assign1030_e1244,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p123,)
    } else {
        (locals.var_stigfn_i,)
    }
};
        locals.var_stigfn_i = assign1030_e1244;
        locals.var_stigfn_i_rv = 0.0;

        let (assign1040_e1248,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p110,)
    } else {
        (locals.var_gc2ch_i,)
    }
};
        locals.var_gc2ch_i = assign1040_e1248;
        locals.var_gc2ch_i_rv = 0.0;

        let (assign1050_e1252,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p111,)
    } else {
        (locals.var_gc3ch_i,)
    }
};
        locals.var_gc3ch_i = assign1050_e1252;
        locals.var_gc3ch_i_rv = 0.0;

        let (assign1060_e1256,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p112,)
    } else {
        (locals.var_gc2ovinv_i,)
    }
};
        locals.var_gc2ovinv_i = assign1060_e1256;
        locals.var_gc2ovinv_i_rv = 0.0;

        let (assign1070_e1260,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p122,)
    } else {
        (locals.var_gcovinvfn_i,)
    }
};
        locals.var_gcovinvfn_i = assign1070_e1260;
        locals.var_gcovinvfn_i_rv = 0.0;

        let (assign1080_e1264,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p113,)
    } else {
        (locals.var_gc3ovinv_i,)
    }
};
        locals.var_gc3ovinv_i = assign1080_e1264;
        locals.var_gc3ovinv_i_rv = 0.0;

        let (assign1090_e1268,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p114,)
    } else {
        (locals.var_gc2ovacc_i,)
    }
};
        locals.var_gc2ovacc_i = assign1090_e1268;
        locals.var_gc2ovacc_i_rv = 0.0;

        let (assign1100_e1272,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p115,)
    } else {
        (locals.var_gc3ovacc_i,)
    }
};
        locals.var_gc3ovacc_i = assign1100_e1272;
        locals.var_gc3ovacc_i_rv = 0.0;

        let (assign1110_e1276,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p116,)
    } else {
        (locals.var_gcdov_i,)
    }
};
        locals.var_gcdov_i = assign1110_e1276;
        locals.var_gcdov_i_rv = 0.0;

        let (assign1120_e1280,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p117,)
    } else {
        (locals.var_gcvdov_i,)
    }
};
        locals.var_gcvdov_i = assign1120_e1280;
        locals.var_gcvdov_i_rv = 0.0;

        let (assign1130_e1284,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p118,)
    } else {
        (locals.var_chib_i,)
    }
};
        locals.var_chib_i = assign1130_e1284;
        locals.var_chib_i_rv = 0.0;

        let (assign1140_e1288,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p119,)
    } else {
        (locals.var_niginv_i,)
    }
};
        locals.var_niginv_i = assign1140_e1288;
        locals.var_niginv_i_rv = 0.0;

        let (assign1150_e1292, assign1150_e1292_d_n4, assign1150_e1292_d_n6, assign1150_e1292_d_n7, assign1150_e1292_d_n8, assign1150_e1292_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p124, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_agidl_i, locals.var_agidl_i_dn4, locals.var_agidl_i_dn6, locals.var_agidl_i_dn7, locals.var_agidl_i_dn8, locals.var_agidl_i_dn9,)
    }
};
        locals.var_agidl_i = assign1150_e1292;
        locals.var_agidl_i_dn4 = assign1150_e1292_d_n4;
        locals.var_agidl_i_dn6 = assign1150_e1292_d_n6;
        locals.var_agidl_i_dn7 = assign1150_e1292_d_n7;
        locals.var_agidl_i_dn8 = assign1150_e1292_d_n8;
        locals.var_agidl_i_dn9 = assign1150_e1292_d_n9;
        locals.var_agidl_i_rv = 0.0;

        let (assign1160_e1296, assign1160_e1296_d_n4, assign1160_e1296_d_n6, assign1160_e1296_d_n7, assign1160_e1296_d_n8, assign1160_e1296_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p125, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_agidld_i, locals.var_agidld_i_dn4, locals.var_agidld_i_dn6, locals.var_agidld_i_dn7, locals.var_agidld_i_dn8, locals.var_agidld_i_dn9,)
    }
};
        locals.var_agidld_i = assign1160_e1296;
        locals.var_agidld_i_dn4 = assign1160_e1296_d_n4;
        locals.var_agidld_i_dn6 = assign1160_e1296_d_n6;
        locals.var_agidld_i_dn7 = assign1160_e1296_d_n7;
        locals.var_agidld_i_dn8 = assign1160_e1296_d_n8;
        locals.var_agidld_i_dn9 = assign1160_e1296_d_n9;
        locals.var_agidld_i_rv = 0.0;

        let (assign1170_e1300,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p126,)
    } else {
        (locals.var_bgidl_t,)
    }
};
        locals.var_bgidl_t = assign1170_e1300;
        locals.var_bgidl_t_rv = 0.0;

        let (assign1180_e1304,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p127,)
    } else {
        (locals.var_bgidld_t,)
    }
};
        locals.var_bgidld_t = assign1180_e1304;
        locals.var_bgidld_t_rv = 0.0;

        let (assign1190_e1308,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p128,)
    } else {
        (locals.var_stbgidl_i,)
    }
};
        locals.var_stbgidl_i = assign1190_e1308;
        locals.var_stbgidl_i_rv = 0.0;

        let (assign1200_e1312,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p129,)
    } else {
        (locals.var_stbgidld_i,)
    }
};
        locals.var_stbgidld_i = assign1200_e1312;
        locals.var_stbgidld_i_rv = 0.0;

        let (assign1210_e1316,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p130,)
    } else {
        (locals.var_cgidl_i,)
    }
};
        locals.var_cgidl_i = assign1210_e1316;
        locals.var_cgidl_i_rv = 0.0;

        let (assign1220_e1320,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p131,)
    } else {
        (locals.var_cgidld_i,)
    }
};
        locals.var_cgidld_i = assign1220_e1320;
        locals.var_cgidld_i_rv = 0.0;

        let (assign1230_e1324,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p132,)
    } else {
        (locals.var_dgidl_i,)
    }
};
        locals.var_dgidl_i = assign1230_e1324;
        locals.var_dgidl_i_rv = 0.0;

        let (assign1240_e1328,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p133,)
    } else {
        (locals.var_dgidld_i,)
    }
};
        locals.var_dgidld_i = assign1240_e1328;
        locals.var_dgidld_i_rv = 0.0;

        let (assign1260_e1336,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p148,)
    } else {
        (locals.var_a2_t,)
    }
};
        locals.var_a2_t = assign1260_e1336;
        locals.var_a2_t_rv = 0.0;

        let (assign1270_e1340,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p149,)
    } else {
        (locals.var_sta2_i,)
    }
};
        locals.var_sta2_i = assign1270_e1340;
        locals.var_sta2_i_rv = 0.0;

        let (assign1280_e1344,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p150,)
    } else {
        (locals.var_a3_i,)
    }
};
        locals.var_a3_i = assign1280_e1344;
        locals.var_a3_i_rv = 0.0;

        let (assign1290_e1348,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p134,)
    } else {
        (locals.var_ctedge_i,)
    }
};
        locals.var_ctedge_i = assign1290_e1348;
        locals.var_ctedge_i_rv = 0.0;

        let (assign1300_e1352, assign1300_e1352_d_n4, assign1300_e1352_d_n6, assign1300_e1352_d_n7, assign1300_e1352_d_n8, assign1300_e1352_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p135, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfb1edge_t, locals.var_vfb1edge_t_dn4, locals.var_vfb1edge_t_dn6, locals.var_vfb1edge_t_dn7, locals.var_vfb1edge_t_dn8, locals.var_vfb1edge_t_dn9,)
    }
};
        locals.var_vfb1edge_t = assign1300_e1352;
        locals.var_vfb1edge_t_dn4 = assign1300_e1352_d_n4;
        locals.var_vfb1edge_t_dn6 = assign1300_e1352_d_n6;
        locals.var_vfb1edge_t_dn7 = assign1300_e1352_d_n7;
        locals.var_vfb1edge_t_dn8 = assign1300_e1352_d_n8;
        locals.var_vfb1edge_t_dn9 = assign1300_e1352_d_n9;
        locals.var_vfb1edge_t_rv = 0.0;

        let (assign1310_e1356,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p136,)
    } else {
        (locals.var_vfb2edge_t,)
    }
};
        locals.var_vfb2edge_t = assign1310_e1356;
        locals.var_vfb2edge_t_rv = 0.0;

        let (assign1320_e1360,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p137,)
    } else {
        (locals.var_stvfbedge_i,)
    }
};
        locals.var_stvfbedge_i = assign1320_e1360;
        locals.var_stvfbedge_i_rv = 0.0;

        let (assign1330_e1364,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p138,)
    } else {
        (locals.var_cic1edge_i,)
    }
};
        locals.var_cic1edge_i = assign1330_e1364;
        locals.var_cic1edge_i_rv = 0.0;

        let (assign1340_e1368,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p139,)
    } else {
        (locals.var_cic2edge_i,)
    }
};
        locals.var_cic2edge_i = assign1340_e1368;
        locals.var_cic2edge_i_rv = 0.0;

        let (assign1350_e1372, assign1350_e1372_d_n4, assign1350_e1372_d_n6, assign1350_e1372_d_n7, assign1350_e1372_d_n8, assign1350_e1372_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p140, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psce1edge_i, locals.var_psce1edge_i_dn4, locals.var_psce1edge_i_dn6, locals.var_psce1edge_i_dn7, locals.var_psce1edge_i_dn8, locals.var_psce1edge_i_dn9,)
    }
};
        locals.var_psce1edge_i = assign1350_e1372;
        locals.var_psce1edge_i_dn4 = assign1350_e1372_d_n4;
        locals.var_psce1edge_i_dn6 = assign1350_e1372_d_n6;
        locals.var_psce1edge_i_dn7 = assign1350_e1372_d_n7;
        locals.var_psce1edge_i_dn8 = assign1350_e1372_d_n8;
        locals.var_psce1edge_i_dn9 = assign1350_e1372_d_n9;
        locals.var_psce1edge_i_rv = 0.0;

        let (assign1360_e1382, assign1360_e1382_d_n4, assign1360_e1382_d_n6, assign1360_e1382_d_n7, assign1360_e1382_d_n8, assign1360_e1382_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        let assign1360_e1376: f64 = (p.p141 * locals.var_psce1edge_i);
        let assign1360_e1378: f64 = (assign1360_e1376 * locals.var_tox2_i);
        let assign1360_e1380: f64 = (assign1360_e1378 / locals.var_tox1_i);
        (assign1360_e1380, (((p.p141 * locals.var_psce1edge_i_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p141 * locals.var_psce1edge_i_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p141 * locals.var_psce1edge_i_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p141 * locals.var_psce1edge_i_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p141 * locals.var_psce1edge_i_dn9) * locals.var_tox2_i) / locals.var_tox1_i),)
    } else {
        (locals.var_psce2edge_i, locals.var_psce2edge_i_dn4, locals.var_psce2edge_i_dn6, locals.var_psce2edge_i_dn7, locals.var_psce2edge_i_dn8, locals.var_psce2edge_i_dn9,)
    }
};
        locals.var_psce2edge_i = assign1360_e1382;
        locals.var_psce2edge_i_dn4 = assign1360_e1382_d_n4;
        locals.var_psce2edge_i_dn6 = assign1360_e1382_d_n6;
        locals.var_psce2edge_i_dn7 = assign1360_e1382_d_n7;
        locals.var_psce2edge_i_dn8 = assign1360_e1382_d_n8;
        locals.var_psce2edge_i_dn9 = assign1360_e1382_d_n9;
        locals.var_psce2edge_i_rv = 0.0;

        let (assign1370_e1386, assign1370_e1386_d_n4, assign1370_e1386_d_n6, assign1370_e1386_d_n7, assign1370_e1386_d_n8, assign1370_e1386_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p142, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cf1edge_i, locals.var_cf1edge_i_dn4, locals.var_cf1edge_i_dn6, locals.var_cf1edge_i_dn7, locals.var_cf1edge_i_dn8, locals.var_cf1edge_i_dn9,)
    }
};
        locals.var_cf1edge_i = assign1370_e1386;
        locals.var_cf1edge_i_dn4 = assign1370_e1386_d_n4;
        locals.var_cf1edge_i_dn6 = assign1370_e1386_d_n6;
        locals.var_cf1edge_i_dn7 = assign1370_e1386_d_n7;
        locals.var_cf1edge_i_dn8 = assign1370_e1386_d_n8;
        locals.var_cf1edge_i_dn9 = assign1370_e1386_d_n9;
        locals.var_cf1edge_i_rv = 0.0;

        let (assign1380_e1396, assign1380_e1396_d_n4, assign1380_e1396_d_n6, assign1380_e1396_d_n7, assign1380_e1396_d_n8, assign1380_e1396_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        let assign1380_e1390: f64 = (p.p143 * locals.var_cf1edge_i);
        let assign1380_e1392: f64 = (assign1380_e1390 * locals.var_tox2_i);
        let assign1380_e1394: f64 = (assign1380_e1392 / locals.var_tox1_i);
        (assign1380_e1394, (((p.p143 * locals.var_cf1edge_i_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p143 * locals.var_cf1edge_i_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p143 * locals.var_cf1edge_i_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p143 * locals.var_cf1edge_i_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p143 * locals.var_cf1edge_i_dn9) * locals.var_tox2_i) / locals.var_tox1_i),)
    } else {
        (locals.var_cf2edge_i, locals.var_cf2edge_i_dn4, locals.var_cf2edge_i_dn6, locals.var_cf2edge_i_dn7, locals.var_cf2edge_i_dn8, locals.var_cf2edge_i_dn9,)
    }
};
        locals.var_cf2edge_i = assign1380_e1396;
        locals.var_cf2edge_i_dn4 = assign1380_e1396_d_n4;
        locals.var_cf2edge_i_dn6 = assign1380_e1396_d_n6;
        locals.var_cf2edge_i_dn7 = assign1380_e1396_d_n7;
        locals.var_cf2edge_i_dn8 = assign1380_e1396_d_n8;
        locals.var_cf2edge_i_dn9 = assign1380_e1396_d_n9;
        locals.var_cf2edge_i_rv = 0.0;

        let (assign1390_e1400,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p144,)
    } else {
        (locals.var_cfdedge_i,)
    }
};
        locals.var_cfdedge_i = assign1390_e1400;
        locals.var_cfdedge_i_rv = 0.0;

        let (assign1400_e1404, assign1400_e1404_d_n4, assign1400_e1404_d_n6, assign1400_e1404_d_n7, assign1400_e1404_d_n8, assign1400_e1404_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p145, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_betnedge_t, locals.var_betnedge_t_dn4, locals.var_betnedge_t_dn6, locals.var_betnedge_t_dn7, locals.var_betnedge_t_dn8, locals.var_betnedge_t_dn9,)
    }
};
        locals.var_betnedge_t = assign1400_e1404;
        locals.var_betnedge_t_dn4 = assign1400_e1404_d_n4;
        locals.var_betnedge_t_dn6 = assign1400_e1404_d_n6;
        locals.var_betnedge_t_dn7 = assign1400_e1404_d_n7;
        locals.var_betnedge_t_dn8 = assign1400_e1404_d_n8;
        locals.var_betnedge_t_dn9 = assign1400_e1404_d_n9;
        locals.var_betnedge_t_rv = 0.0;

        let (assign1410_e1408,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p146,)
    } else {
        (locals.var_stbetedge_i,)
    }
};
        locals.var_stbetedge_i = assign1410_e1408;
        locals.var_stbetedge_i_rv = 0.0;

        let (assign1420_e1412,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p151,)
    } else {
        (locals.var_areaq_i,)
    }
};
        locals.var_areaq_i = assign1420_e1412;
        locals.var_areaq_i_rv = 0.0;

        let (assign1430_e1416, assign1430_e1416_d_n4, assign1430_e1416_d_n6, assign1430_e1416_d_n7, assign1430_e1416_d_n8, assign1430_e1416_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p152, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgbov_i, locals.var_cgbov_i_dn4, locals.var_cgbov_i_dn6, locals.var_cgbov_i_dn7, locals.var_cgbov_i_dn8, locals.var_cgbov_i_dn9,)
    }
};
        locals.var_cgbov_i = assign1430_e1416;
        locals.var_cgbov_i_dn4 = assign1430_e1416_d_n4;
        locals.var_cgbov_i_dn6 = assign1430_e1416_d_n6;
        locals.var_cgbov_i_dn7 = assign1430_e1416_d_n7;
        locals.var_cgbov_i_dn8 = assign1430_e1416_d_n8;
        locals.var_cgbov_i_dn9 = assign1430_e1416_d_n9;
        locals.var_cgbov_i_rv = 0.0;

        let (assign1440_e1422,) = {
    if (locals.var_guard83 != 0.0) {
        let assign1440_e1420: f64 = (p.p153 * 1000000.0);
        (assign1440_e1420,)
    } else {
        (locals.var_nsdac_i,)
    }
};
        locals.var_nsdac_i = assign1440_e1422;
        locals.var_nsdac_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign1450_e1426,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p154,)
    } else {
        (locals.var_fif_i,)
    }
};
        locals.var_fif_i = assign1450_e1426;
        locals.var_fif_i_rv = 0.0;

        let (assign1460_e1430,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p155,)
    } else {
        (locals.var_fsceac_i,)
    }
};
        locals.var_fsceac_i = assign1460_e1430;
        locals.var_fsceac_i_rv = 0.0;

        let (assign1470_e1434, assign1470_e1434_d_n4, assign1470_e1434_d_n6, assign1470_e1434_d_n7, assign1470_e1434_d_n8, assign1470_e1434_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9,)
    } else {
        (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9,)
    }
};
        locals.var_vfbac1_t = assign1470_e1434;
        locals.var_vfbac1_t_dn4 = assign1470_e1434_d_n4;
        locals.var_vfbac1_t_dn6 = assign1470_e1434_d_n6;
        locals.var_vfbac1_t_dn7 = assign1470_e1434_d_n7;
        locals.var_vfbac1_t_dn8 = assign1470_e1434_d_n8;
        locals.var_vfbac1_t_dn9 = assign1470_e1434_d_n9;
        locals.var_vfbac1_t_rv = 0.0;

        let (assign1480_e1438, assign1480_e1438_d_n4, assign1480_e1438_d_n6, assign1480_e1438_d_n7, assign1480_e1438_d_n8, assign1480_e1438_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9,)
    } else {
        (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9,)
    }
};
        locals.var_vfbac2_t = assign1480_e1438;
        locals.var_vfbac2_t_dn4 = assign1480_e1438_d_n4;
        locals.var_vfbac2_t_dn6 = assign1480_e1438_d_n6;
        locals.var_vfbac2_t_dn7 = assign1480_e1438_d_n7;
        locals.var_vfbac2_t_dn8 = assign1480_e1438_d_n8;
        locals.var_vfbac2_t_dn9 = assign1480_e1438_d_n9;
        locals.var_vfbac2_t_rv = 0.0;

        let (assign1490_e1442,) = {
    if (locals.var_guard83 != 0.0) {
        (locals.var_psce1_i,)
    } else {
        (locals.var_psceac1_i,)
    }
};
        locals.var_psceac1_i = assign1490_e1442;
        locals.var_psceac1_i_rv = 0.0;

        let (assign1500_e1446,) = {
    if (locals.var_guard83 != 0.0) {
        (locals.var_psce2_i,)
    } else {
        (locals.var_psceac2_i,)
    }
};
        locals.var_psceac2_i = assign1500_e1446;
        locals.var_psceac2_i_rv = 0.0;

        let (assign1510_e1450, assign1510_e1450_d_n4, assign1510_e1450_d_n6, assign1510_e1450_d_n7, assign1510_e1450_d_n8, assign1510_e1450_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9,)
    } else {
        (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9,)
    }
};
        locals.var_cfac1_t = assign1510_e1450;
        locals.var_cfac1_t_dn4 = assign1510_e1450_d_n4;
        locals.var_cfac1_t_dn6 = assign1510_e1450_d_n6;
        locals.var_cfac1_t_dn7 = assign1510_e1450_d_n7;
        locals.var_cfac1_t_dn8 = assign1510_e1450_d_n8;
        locals.var_cfac1_t_dn9 = assign1510_e1450_d_n9;
        locals.var_cfac1_t_rv = 0.0;

        let (assign1520_e1454, assign1520_e1454_d_n4, assign1520_e1454_d_n6, assign1520_e1454_d_n7, assign1520_e1454_d_n8, assign1520_e1454_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9,)
    } else {
        (locals.var_cfac2_t, locals.var_cfac2_t_dn4, locals.var_cfac2_t_dn6, locals.var_cfac2_t_dn7, locals.var_cfac2_t_dn8, locals.var_cfac2_t_dn9,)
    }
};
        locals.var_cfac2_t = assign1520_e1454;
        locals.var_cfac2_t_dn4 = assign1520_e1454_d_n4;
        locals.var_cfac2_t_dn6 = assign1520_e1454_d_n6;
        locals.var_cfac2_t_dn7 = assign1520_e1454_d_n7;
        locals.var_cfac2_t_dn8 = assign1520_e1454_d_n8;
        locals.var_cfac2_t_dn9 = assign1520_e1454_d_n9;
        locals.var_cfac2_t_rv = 0.0;

        let (assign1530_e1458, assign1530_e1458_d_n4, assign1530_e1458_d_n6, assign1530_e1458_d_n7, assign1530_e1458_d_n8, assign1530_e1458_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (locals.var_thesat_t, locals.var_thesat_t_dn4, locals.var_thesat_t_dn6, locals.var_thesat_t_dn7, locals.var_thesat_t_dn8, locals.var_thesat_t_dn9,)
    } else {
        (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9,)
    }
};
        locals.var_thesatac_t = assign1530_e1458;
        locals.var_thesatac_t_dn4 = assign1530_e1458_d_n4;
        locals.var_thesatac_t_dn6 = assign1530_e1458_d_n6;
        locals.var_thesatac_t_dn7 = assign1530_e1458_d_n7;
        locals.var_thesatac_t_dn8 = assign1530_e1458_d_n8;
        locals.var_thesatac_t_dn9 = assign1530_e1458_d_n9;
        locals.var_thesatac_t_rv = 0.0;

        let (assign1540_e1462,) = {
    if (locals.var_guard83 != 0.0) {
        (locals.var_ax_i,)
    } else {
        (locals.var_axac_i,)
    }
};
        locals.var_axac_i = assign1540_e1462;
        locals.var_axac_i_rv = 0.0;

        let (assign1550_e1466,) = {
    if (locals.var_guard83 != 0.0) {
        (locals.var_alp_i,)
    } else {
        (locals.var_alpac_i,)
    }
};
        locals.var_alpac_i = assign1550_e1466;
        locals.var_alpac_i_rv = 0.0;

        let assign1560_e1469: f64 = if p.p11 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard86 = assign1560_e1469;
        locals.var_guard86_rv = 0.0;

        let (assign1570_e1475, assign1570_e1475_d_n4, assign1570_e1475_d_n6, assign1570_e1475_d_n7, assign1570_e1475_d_n8, assign1570_e1475_d_n9,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
        (p.p51, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9,)
    }
};
        locals.var_vfbac1_t = assign1570_e1475;
        locals.var_vfbac1_t_dn4 = assign1570_e1475_d_n4;
        locals.var_vfbac1_t_dn6 = assign1570_e1475_d_n6;
        locals.var_vfbac1_t_dn7 = assign1570_e1475_d_n7;
        locals.var_vfbac1_t_dn8 = assign1570_e1475_d_n8;
        locals.var_vfbac1_t_dn9 = assign1570_e1475_d_n9;
        locals.var_vfbac1_t_rv = 0.0;

        let assign1580_e1477: f64 = if param_given[156] { 1.0 } else { 0.0 };
        let assign1580_e1479: f64 = if assign1580_e1477 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard87 = assign1580_e1479;
        locals.var_guard87_rv = 0.0;

        let (assign1590_e1487, assign1590_e1487_d_n4, assign1590_e1487_d_n6, assign1590_e1487_d_n7, assign1590_e1487_d_n8, assign1590_e1487_d_n9,) = {
    if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard87 != 0.0)) {
        (p.p156, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9,)
    }
};
        locals.var_vfbac1_t = assign1590_e1487;
        locals.var_vfbac1_t_dn4 = assign1590_e1487_d_n4;
        locals.var_vfbac1_t_dn6 = assign1590_e1487_d_n6;
        locals.var_vfbac1_t_dn7 = assign1590_e1487_d_n7;
        locals.var_vfbac1_t_dn8 = assign1590_e1487_d_n8;
        locals.var_vfbac1_t_dn9 = assign1590_e1487_d_n9;
        locals.var_vfbac1_t_rv = 0.0;

        let (assign1600_e1493, assign1600_e1493_d_n4, assign1600_e1493_d_n6, assign1600_e1493_d_n7, assign1600_e1493_d_n8, assign1600_e1493_d_n9,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
        (p.p52, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9,)
    }
};
        locals.var_vfbac2_t = assign1600_e1493;
        locals.var_vfbac2_t_dn4 = assign1600_e1493_d_n4;
        locals.var_vfbac2_t_dn6 = assign1600_e1493_d_n6;
        locals.var_vfbac2_t_dn7 = assign1600_e1493_d_n7;
        locals.var_vfbac2_t_dn8 = assign1600_e1493_d_n8;
        locals.var_vfbac2_t_dn9 = assign1600_e1493_d_n9;
        locals.var_vfbac2_t_rv = 0.0;

        let assign1610_e1495: f64 = if param_given[157] { 1.0 } else { 0.0 };
        let assign1610_e1497: f64 = if assign1610_e1495 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard88 = assign1610_e1497;
        locals.var_guard88_rv = 0.0;

        let (assign1620_e1505, assign1620_e1505_d_n4, assign1620_e1505_d_n6, assign1620_e1505_d_n7, assign1620_e1505_d_n8, assign1620_e1505_d_n9,) = {
    if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard88 != 0.0)) {
        (p.p157, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9,)
    }
};
        locals.var_vfbac2_t = assign1620_e1505;
        locals.var_vfbac2_t_dn4 = assign1620_e1505_d_n4;
        locals.var_vfbac2_t_dn6 = assign1620_e1505_d_n6;
        locals.var_vfbac2_t_dn7 = assign1620_e1505_d_n7;
        locals.var_vfbac2_t_dn8 = assign1620_e1505_d_n8;
        locals.var_vfbac2_t_dn9 = assign1620_e1505_d_n9;
        locals.var_vfbac2_t_rv = 0.0;

        let (assign1630_e1511,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
        (p.p57,)
    } else {
        (locals.var_psceac1_i,)
    }
};
        locals.var_psceac1_i = assign1630_e1511;
        locals.var_psceac1_i_rv = 0.0;

        let assign1640_e1513: f64 = if param_given[158] { 1.0 } else { 0.0 };
        let assign1640_e1515: f64 = if assign1640_e1513 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard89 = assign1640_e1515;
        locals.var_guard89_rv = 0.0;

        let (assign1650_e1523,) = {
    if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard89 != 0.0)) {
        (p.p158,)
    } else {
        (locals.var_psceac1_i,)
    }
};
        locals.var_psceac1_i = assign1650_e1523;
        locals.var_psceac1_i_rv = 0.0;

        let (assign1660_e1535,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
        let assign1660_e1529: f64 = (p.p58 * locals.var_psceac1_i);
        let assign1660_e1531: f64 = (assign1660_e1529 * locals.var_tox2_i);
        let assign1660_e1533: f64 = (assign1660_e1531 / locals.var_tox1_i);
        (assign1660_e1533,)
    } else {
        (locals.var_psceac2_i,)
    }
};
        locals.var_psceac2_i = assign1660_e1535;
        locals.var_psceac2_i_rv = 0.0;

        let (assign1670_e1541, assign1670_e1541_d_n4, assign1670_e1541_d_n6, assign1670_e1541_d_n7, assign1670_e1541_d_n8, assign1670_e1541_d_n9,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
        (p.p62, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9,)
    }
};
        locals.var_cfac1_t = assign1670_e1541;
        locals.var_cfac1_t_dn4 = assign1670_e1541_d_n4;
        locals.var_cfac1_t_dn6 = assign1670_e1541_d_n6;
        locals.var_cfac1_t_dn7 = assign1670_e1541_d_n7;
        locals.var_cfac1_t_dn8 = assign1670_e1541_d_n8;
        locals.var_cfac1_t_dn9 = assign1670_e1541_d_n9;
        locals.var_cfac1_t_rv = 0.0;

        let assign1680_e1543: f64 = if param_given[159] { 1.0 } else { 0.0 };
        let assign1680_e1545: f64 = if assign1680_e1543 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard90 = assign1680_e1545;
        locals.var_guard90_rv = 0.0;

        let (assign1690_e1553, assign1690_e1553_d_n4, assign1690_e1553_d_n6, assign1690_e1553_d_n7, assign1690_e1553_d_n8, assign1690_e1553_d_n9,) = {
    if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard90 != 0.0)) {
        (p.p159, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9,)
    }
};
        locals.var_cfac1_t = assign1690_e1553;
        locals.var_cfac1_t_dn4 = assign1690_e1553_d_n4;
        locals.var_cfac1_t_dn6 = assign1690_e1553_d_n6;
        locals.var_cfac1_t_dn7 = assign1690_e1553_d_n7;
        locals.var_cfac1_t_dn8 = assign1690_e1553_d_n8;
        locals.var_cfac1_t_dn9 = assign1690_e1553_d_n9;
        locals.var_cfac1_t_rv = 0.0;

        let (assign1700_e1565, assign1700_e1565_d_n4, assign1700_e1565_d_n6, assign1700_e1565_d_n7, assign1700_e1565_d_n8, assign1700_e1565_d_n9,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
        let assign1700_e1559: f64 = (p.p63 * locals.var_cfac1_t);
        let assign1700_e1561: f64 = (assign1700_e1559 * locals.var_tox2_i);
        let assign1700_e1563: f64 = (assign1700_e1561 / locals.var_tox1_i);
        (assign1700_e1563, (((p.p63 * locals.var_cfac1_t_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cfac1_t_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cfac1_t_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cfac1_t_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cfac1_t_dn9) * locals.var_tox2_i) / locals.var_tox1_i),)
    } else {
        (locals.var_cfac2_t, locals.var_cfac2_t_dn4, locals.var_cfac2_t_dn6, locals.var_cfac2_t_dn7, locals.var_cfac2_t_dn8, locals.var_cfac2_t_dn9,)
    }
};
        locals.var_cfac2_t = assign1700_e1565;
        locals.var_cfac2_t_dn4 = assign1700_e1565_d_n4;
        locals.var_cfac2_t_dn6 = assign1700_e1565_d_n6;
        locals.var_cfac2_t_dn7 = assign1700_e1565_d_n7;
        locals.var_cfac2_t_dn8 = assign1700_e1565_d_n8;
        locals.var_cfac2_t_dn9 = assign1700_e1565_d_n9;
        locals.var_cfac2_t_rv = 0.0;

        let (assign1710_e1571, assign1710_e1571_d_n4, assign1710_e1571_d_n6, assign1710_e1571_d_n7, assign1710_e1571_d_n8, assign1710_e1571_d_n9,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
        (p.p93, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9,)
    }
};
        locals.var_thesatac_t = assign1710_e1571;
        locals.var_thesatac_t_dn4 = assign1710_e1571_d_n4;
        locals.var_thesatac_t_dn6 = assign1710_e1571_d_n6;
        locals.var_thesatac_t_dn7 = assign1710_e1571_d_n7;
        locals.var_thesatac_t_dn8 = assign1710_e1571_d_n8;
        locals.var_thesatac_t_dn9 = assign1710_e1571_d_n9;
        locals.var_thesatac_t_rv = 0.0;

        let assign1720_e1573: f64 = if param_given[160] { 1.0 } else { 0.0 };
        let assign1720_e1575: f64 = if assign1720_e1573 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard91 = assign1720_e1575;
        locals.var_guard91_rv = 0.0;

        let (assign1730_e1583, assign1730_e1583_d_n4, assign1730_e1583_d_n6, assign1730_e1583_d_n7, assign1730_e1583_d_n8, assign1730_e1583_d_n9,) = {
    if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard91 != 0.0)) {
        (p.p160, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9,)
    }
};
        locals.var_thesatac_t = assign1730_e1583;
        locals.var_thesatac_t_dn4 = assign1730_e1583_d_n4;
        locals.var_thesatac_t_dn6 = assign1730_e1583_d_n6;
        locals.var_thesatac_t_dn7 = assign1730_e1583_d_n7;
        locals.var_thesatac_t_dn8 = assign1730_e1583_d_n8;
        locals.var_thesatac_t_dn9 = assign1730_e1583_d_n9;
        locals.var_thesatac_t_rv = 0.0;

        let (assign1740_e1589,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
        (p.p97,)
    } else {
        (locals.var_axac_i,)
    }
};
        locals.var_axac_i = assign1740_e1589;
        locals.var_axac_i_rv = 0.0;

        let assign1750_e1591: f64 = if param_given[161] { 1.0 } else { 0.0 };
        let assign1750_e1593: f64 = if assign1750_e1591 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard92 = assign1750_e1593;
        locals.var_guard92_rv = 0.0;

        let (assign1760_e1601,) = {
    if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard92 != 0.0)) {
        (p.p161,)
    } else {
        (locals.var_axac_i,)
    }
};
        locals.var_axac_i = assign1760_e1601;
        locals.var_axac_i_rv = 0.0;

        let (assign1770_e1607,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
        (p.p98,)
    } else {
        (locals.var_alpac_i,)
    }
};
        locals.var_alpac_i = assign1770_e1607;
        locals.var_alpac_i_rv = 0.0;

        let assign1780_e1609: f64 = if param_given[162] { 1.0 } else { 0.0 };
        let assign1780_e1611: f64 = if assign1780_e1609 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard93 = assign1780_e1611;
        locals.var_guard93_rv = 0.0;

        let (assign1790_e1619,) = {
    if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard93 != 0.0)) {
        (p.p162,)
    } else {
        (locals.var_alpac_i,)
    }
};
        locals.var_alpac_i = assign1790_e1619;
        locals.var_alpac_i_rv = 0.0;

        let (assign1800_e1623, assign1800_e1623_d_n4, assign1800_e1623_d_n6, assign1800_e1623_d_n7, assign1800_e1623_d_n8, assign1800_e1623_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p163, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cov_i, locals.var_cov_i_dn4, locals.var_cov_i_dn6, locals.var_cov_i_dn7, locals.var_cov_i_dn8, locals.var_cov_i_dn9,)
    }
};
        locals.var_cov_i = assign1800_e1623;
        locals.var_cov_i_dn4 = assign1800_e1623_d_n4;
        locals.var_cov_i_dn6 = assign1800_e1623_d_n6;
        locals.var_cov_i_dn7 = assign1800_e1623_d_n7;
        locals.var_cov_i_dn8 = assign1800_e1623_d_n8;
        locals.var_cov_i_dn9 = assign1800_e1623_d_n9;
        locals.var_cov_i_rv = 0.0;

        let (assign1810_e1627, assign1810_e1627_d_n4, assign1810_e1627_d_n6, assign1810_e1627_d_n7, assign1810_e1627_d_n8, assign1810_e1627_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p164, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_covd_i, locals.var_covd_i_dn4, locals.var_covd_i_dn6, locals.var_covd_i_dn7, locals.var_covd_i_dn8, locals.var_covd_i_dn9,)
    }
};
        locals.var_covd_i = assign1810_e1627;
        locals.var_covd_i_dn4 = assign1810_e1627_d_n4;
        locals.var_covd_i_dn6 = assign1810_e1627_d_n6;
        locals.var_covd_i_dn7 = assign1810_e1627_d_n7;
        locals.var_covd_i_dn8 = assign1810_e1627_d_n8;
        locals.var_covd_i_dn9 = assign1810_e1627_d_n9;
        locals.var_covd_i_rv = 0.0;

        let (assign1820_e1631,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p165,)
    } else {
        (locals.var_covdl_i,)
    }
};
        locals.var_covdl_i = assign1820_e1631;
        locals.var_covdl_i_rv = 0.0;

        let (assign1830_e1635,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p166,)
    } else {
        (locals.var_covdlb_i,)
    }
};
        locals.var_covdlb_i = assign1830_e1635;
        locals.var_covdlb_i_rv = 0.0;

        let (assign1840_e1639,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p167,)
    } else {
        (locals.var_dvfbov_i,)
    }
};
        locals.var_dvfbov_i = assign1840_e1639;
        locals.var_dvfbov_i_rv = 0.0;

        let (assign1850_e1643, assign1850_e1643_d_n4, assign1850_e1643_d_n6, assign1850_e1643_d_n7, assign1850_e1643_d_n8, assign1850_e1643_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p168, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cfr_i, locals.var_cfr_i_dn4, locals.var_cfr_i_dn6, locals.var_cfr_i_dn7, locals.var_cfr_i_dn8, locals.var_cfr_i_dn9,)
    }
};
        locals.var_cfr_i = assign1850_e1643;
        locals.var_cfr_i_dn4 = assign1850_e1643_d_n4;
        locals.var_cfr_i_dn6 = assign1850_e1643_d_n6;
        locals.var_cfr_i_dn7 = assign1850_e1643_d_n7;
        locals.var_cfr_i_dn8 = assign1850_e1643_d_n8;
        locals.var_cfr_i_dn9 = assign1850_e1643_d_n9;
        locals.var_cfr_i_rv = 0.0;

        let (assign1860_e1647, assign1860_e1647_d_n4, assign1860_e1647_d_n6, assign1860_e1647_d_n7, assign1860_e1647_d_n8, assign1860_e1647_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p169, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cfrd_i, locals.var_cfrd_i_dn4, locals.var_cfrd_i_dn6, locals.var_cfrd_i_dn7, locals.var_cfrd_i_dn8, locals.var_cfrd_i_dn9,)
    }
};
        locals.var_cfrd_i = assign1860_e1647;
        locals.var_cfrd_i_dn4 = assign1860_e1647_d_n4;
        locals.var_cfrd_i_dn6 = assign1860_e1647_d_n6;
        locals.var_cfrd_i_dn7 = assign1860_e1647_d_n7;
        locals.var_cfrd_i_dn8 = assign1860_e1647_d_n8;
        locals.var_cfrd_i_dn9 = assign1860_e1647_d_n9;
        locals.var_cfrd_i_rv = 0.0;

        let (assign1870_e1651,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p170,)
    } else {
        (locals.var_csd_i,)
    }
};
        locals.var_csd_i = assign1870_e1651;
        locals.var_csd_i_rv = 0.0;

        let (assign1880_e1655,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p171,)
    } else {
        (locals.var_csdbp_i,)
    }
};
        locals.var_csdbp_i = assign1880_e1655;
        locals.var_csdbp_i_rv = 0.0;

        let (assign1890_e1659, assign1890_e1659_d_n4, assign1890_e1659_d_n6, assign1890_e1659_d_n7, assign1890_e1659_d_n8, assign1890_e1659_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p172, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rth_t, locals.var_rth_t_dn4, locals.var_rth_t_dn6, locals.var_rth_t_dn7, locals.var_rth_t_dn8, locals.var_rth_t_dn9,)
    }
};
        locals.var_rth_t = assign1890_e1659;
        locals.var_rth_t_dn4 = assign1890_e1659_d_n4;
        locals.var_rth_t_dn6 = assign1890_e1659_d_n6;
        locals.var_rth_t_dn7 = assign1890_e1659_d_n7;
        locals.var_rth_t_dn8 = assign1890_e1659_d_n8;
        locals.var_rth_t_dn9 = assign1890_e1659_d_n9;
        locals.var_rth_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign1900_e1663,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p173,)
    } else {
        (locals.var_strth_i,)
    }
};
        locals.var_strth_i = assign1900_e1663;
        locals.var_strth_i_rv = 0.0;

        let (assign1940_e1679,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p177,)
    } else {
        (locals.var_nfa_i,)
    }
};
        locals.var_nfa_i = assign1940_e1679;
        locals.var_nfa_i_rv = 0.0;

        let (assign1950_e1683,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p178,)
    } else {
        (locals.var_nfb_i,)
    }
};
        locals.var_nfb_i = assign1950_e1683;
        locals.var_nfb_i_rv = 0.0;

        let (assign1960_e1687,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p179,)
    } else {
        (locals.var_nfc_i,)
    }
};
        locals.var_nfc_i = assign1960_e1687;
        locals.var_nfc_i_rv = 0.0;

        let (assign1970_e1691,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p180,)
    } else {
        (locals.var_nfe_i,)
    }
};
        locals.var_nfe_i = assign1970_e1691;
        locals.var_nfe_i_rv = 0.0;

        let (assign1980_e1695,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p181,)
    } else {
        (locals.var_nfeb_i,)
    }
};
        locals.var_nfeb_i = assign1980_e1695;
        locals.var_nfeb_i_rv = 0.0;

        let (assign2040_e1722,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2040_e1720: f64 = (1.0 / p.p29);
        (assign2040_e1720,)
    } else {
        (locals.var_invnf,)
    }
};
        locals.var_invnf = assign2040_e1722;
        locals.var_invnf_rv = 0.0;

        let (assign2050_e1731,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2050_e1727: f64 = (p.p21 * locals.var_invnf);
        let assign2050_e1729: f64 = (assign2050_e1727).max(1e-9);
        (assign2050_e1729,)
    } else {
        (locals.var_w_i,)
    }
};
        locals.var_w_i = assign2050_e1731;
        locals.var_w_i_rv = 0.0;

        let (assign2060_e1738,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2060_e1736: f64 = (p.p23 * locals.var_invnf);
        (assign2060_e1736,)
    } else {
        (locals.var_adrain_i,)
    }
};
        locals.var_adrain_i = assign2060_e1738;
        locals.var_adrain_i_rv = 0.0;

        let (assign2070_e1745,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2070_e1743: f64 = (p.p22 * locals.var_invnf);
        (assign2070_e1743,)
    } else {
        (locals.var_asource_i,)
    }
};
        locals.var_asource_i = assign2070_e1745;
        locals.var_asource_i_rv = 0.0;

        let (assign2080_e1752,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2080_e1750: f64 = (p.p25 * locals.var_invnf);
        (assign2080_e1750,)
    } else {
        (locals.var_pdrain_i,)
    }
};
        locals.var_pdrain_i = assign2080_e1752;
        locals.var_pdrain_i_rv = 0.0;

        let (assign2090_e1759,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2090_e1757: f64 = (p.p24 * locals.var_invnf);
        (assign2090_e1757,)
    } else {
        (locals.var_psource_i,)
    }
};
        locals.var_psource_i = assign2090_e1759;
        locals.var_psource_i_rv = 0.0;

        let (assign2100_e1766,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2100_e1764: f64 = (p.p30 * p.p29);
        (assign2100_e1764,)
    } else {
        (locals.var_mult_i_int,)
    }
};
        locals.var_mult_i_int = assign2100_e1766;
        locals.var_mult_i_int_rv = 0.0;

        let (assign2110_e1771,) = {
    if (locals.var_guard83 == 0.0) {
        (1e-6,)
    } else {
        (locals.var_len,)
    }
};
        locals.var_len = assign2110_e1771;
        locals.var_len_rv = 0.0;

        let (assign2120_e1776,) = {
    if (locals.var_guard83 == 0.0) {
        (1e-6,)
    } else {
        (locals.var_wen,)
    }
};
        locals.var_wen = assign2120_e1776;
        locals.var_wen_rv = 0.0;

        let (assign2130_e1783,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2130_e1781: f64 = (locals.var_len / p.p20);
        (assign2130_e1781,)
    } else {
        (locals.var_il,)
    }
};
        locals.var_il = assign2130_e1783;
        locals.var_il_rv = 0.0;

        let (assign2140_e1790,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2140_e1788: f64 = (locals.var_wen / locals.var_w_i);
        (assign2140_e1788,)
    } else {
        (locals.var_iw,)
    }
};
        locals.var_iw = assign2140_e1790;
        locals.var_iw_rv = 0.0;

        let (assign2150_e1807,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2150_e1797: f64 = (p.p188 * locals.var_il);
        let assign2150_e1798: f64 = (1.0 + assign2150_e1797);
        let assign2150_e1799: f64 = (p.p187 * assign2150_e1798);
        let assign2150_e1803: f64 = (p.p189 * locals.var_iw);
        let assign2150_e1804: f64 = (1.0 + assign2150_e1803);
        let assign2150_e1805: f64 = (assign2150_e1799 * assign2150_e1804);
        (assign2150_e1805,)
    } else {
        (locals.var_dellps,)
    }
};
        locals.var_dellps = assign2150_e1807;
        locals.var_dellps_rv = 0.0;

        let (assign2160_e1824,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2160_e1814: f64 = (p.p193 * locals.var_iw);
        let assign2160_e1815: f64 = (1.0 + assign2160_e1814);
        let assign2160_e1816: f64 = (p.p191 * assign2160_e1815);
        let assign2160_e1820: f64 = (p.p192 * locals.var_il);
        let assign2160_e1821: f64 = (1.0 + assign2160_e1820);
        let assign2160_e1822: f64 = (assign2160_e1816 * assign2160_e1821);
        (assign2160_e1822,)
    } else {
        (locals.var_delwod,)
    }
};
        locals.var_delwod = assign2160_e1824;
        locals.var_delwod_rv = 0.0;

        let (assign2170_e1837,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2170_e1829: f64 = (p.p20 + locals.var_dellps);
        let assign2170_e1832: f64 = (2.0 * p.p190);
        let assign2170_e1833: f64 = (assign2170_e1829 - assign2170_e1832);
        let assign2170_e1835: f64 = (assign2170_e1833).max(1e-9);
        (assign2170_e1835,)
    } else {
        (locals.var_le,)
    }
};
        locals.var_le = assign2170_e1837;
        locals.var_le_rv = 0.0;

        let (assign2180_e1850,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2180_e1842: f64 = (locals.var_w_i + locals.var_delwod);
        let assign2180_e1845: f64 = (2.0 * p.p194);
        let assign2180_e1846: f64 = (assign2180_e1842 - assign2180_e1845);
        let assign2180_e1848: f64 = (assign2180_e1846).max(1e-9);
        (assign2180_e1848,)
    } else {
        (locals.var_we,)
    }
};
        locals.var_we = assign2180_e1850;
        locals.var_we_rv = 0.0;

        let (assign2190_e1865,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2190_e1855: f64 = (p.p20 + locals.var_dellps);
        let assign2190_e1858: f64 = (2.0 * p.p190);
        let assign2190_e1859: f64 = (assign2190_e1855 - assign2190_e1858);
        let assign2190_e1861: f64 = (assign2190_e1859 + p.p195);
        let assign2190_e1863: f64 = (assign2190_e1861).max(1e-9);
        (assign2190_e1863,)
    } else {
        (locals.var_lecv,)
    }
};
        locals.var_lecv = assign2190_e1865;
        locals.var_lecv_rv = 0.0;

        let (assign2200_e1880,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2200_e1870: f64 = (locals.var_w_i + locals.var_delwod);
        let assign2200_e1873: f64 = (2.0 * p.p194);
        let assign2200_e1874: f64 = (assign2200_e1870 - assign2200_e1873);
        let assign2200_e1876: f64 = (assign2200_e1874 + p.p196);
        let assign2200_e1878: f64 = (assign2200_e1876).max(1e-9);
        (assign2200_e1878,)
    } else {
        (locals.var_wecv,)
    }
};
        locals.var_wecv = assign2200_e1880;
        locals.var_wecv_rv = 0.0;

        let (assign2210_e1887,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2210_e1885: f64 = (locals.var_len / locals.var_le);
        (assign2210_e1885,)
    } else {
        (locals.var_ile,)
    }
};
        locals.var_ile = assign2210_e1887;
        locals.var_ile_rv = 0.0;

        let (assign2220_e1894,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2220_e1892: f64 = (locals.var_wen / locals.var_we);
        (assign2220_e1892,)
    } else {
        (locals.var_iwe,)
    }
};
        locals.var_iwe = assign2220_e1894;
        locals.var_iwe_rv = 0.0;

        let (assign2230_e1901,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2230_e1899: f64 = (locals.var_ile * locals.var_iwe);
        (assign2230_e1899,)
    } else {
        (locals.var_iae,)
    }
};
        locals.var_iae = assign2230_e1901;
        locals.var_iae_rv = 0.0;

        let (assign2240_e1910, assign2240_e1910_d_n4, assign2240_e1910_d_n6, assign2240_e1910_d_n7, assign2240_e1910_d_n8, assign2240_e1910_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2240_e1906: f64 = (p.p20 + locals.var_dellps);
        let assign2240_e1908: f64 = (assign2240_e1906).max(1e-9);
        (assign2240_e1908, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign2240_e1910;
        locals.var_temp_dn4 = assign2240_e1910_d_n4;
        locals.var_temp_dn6 = assign2240_e1910_d_n6;
        locals.var_temp_dn7 = assign2240_e1910_d_n7;
        locals.var_temp_dn8 = assign2240_e1910_d_n8;
        locals.var_temp_dn9 = assign2240_e1910_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign2250_e1917, assign2250_e1917_d_n4, assign2250_e1917_d_n6, assign2250_e1917_d_n7, assign2250_e1917_d_n8, assign2250_e1917_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2250_e1915: f64 = (locals.var_temp / locals.var_len);
        (assign2250_e1915, (locals.var_temp_dn4 / locals.var_len), (locals.var_temp_dn6 / locals.var_len), (locals.var_temp_dn7 / locals.var_len), (locals.var_temp_dn8 / locals.var_len), (locals.var_temp_dn9 / locals.var_len),)
    } else {
        (locals.var_lphy, locals.var_lphy_dn4, locals.var_lphy_dn6, locals.var_lphy_dn7, locals.var_lphy_dn8, locals.var_lphy_dn9,)
    }
};
        locals.var_lphy = assign2250_e1917;
        locals.var_lphy_dn4 = assign2250_e1917_d_n4;
        locals.var_lphy_dn6 = assign2250_e1917_d_n6;
        locals.var_lphy_dn7 = assign2250_e1917_d_n7;
        locals.var_lphy_dn8 = assign2250_e1917_d_n8;
        locals.var_lphy_dn9 = assign2250_e1917_d_n9;
        locals.var_lphy_rv = 0.0;

        let (assign2260_e1926, assign2260_e1926_d_n4, assign2260_e1926_d_n6, assign2260_e1926_d_n7, assign2260_e1926_d_n8, assign2260_e1926_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2260_e1922: f64 = (locals.var_w_i + locals.var_delwod);
        let assign2260_e1924: f64 = (assign2260_e1922).max(1e-9);
        (assign2260_e1924, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign2260_e1926;
        locals.var_temp_dn4 = assign2260_e1926_d_n4;
        locals.var_temp_dn6 = assign2260_e1926_d_n6;
        locals.var_temp_dn7 = assign2260_e1926_d_n7;
        locals.var_temp_dn8 = assign2260_e1926_d_n8;
        locals.var_temp_dn9 = assign2260_e1926_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign2270_e1933, assign2270_e1933_d_n4, assign2270_e1933_d_n6, assign2270_e1933_d_n7, assign2270_e1933_d_n8, assign2270_e1933_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2270_e1931: f64 = (locals.var_temp / locals.var_wen);
        (assign2270_e1931, (locals.var_temp_dn4 / locals.var_wen), (locals.var_temp_dn6 / locals.var_wen), (locals.var_temp_dn7 / locals.var_wen), (locals.var_temp_dn8 / locals.var_wen), (locals.var_temp_dn9 / locals.var_wen),)
    } else {
        (locals.var_wphy, locals.var_wphy_dn4, locals.var_wphy_dn6, locals.var_wphy_dn7, locals.var_wphy_dn8, locals.var_wphy_dn9,)
    }
};
        locals.var_wphy = assign2270_e1933;
        locals.var_wphy_dn4 = assign2270_e1933_d_n4;
        locals.var_wphy_dn6 = assign2270_e1933_d_n6;
        locals.var_wphy_dn7 = assign2270_e1933_d_n7;
        locals.var_wphy_dn8 = assign2270_e1933_d_n8;
        locals.var_wphy_dn9 = assign2270_e1933_d_n9;
        locals.var_wphy_rv = 0.0;

        let (assign2320_e1976,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p197,)
    } else {
        (locals.var_tox1_i,)
    }
};
        locals.var_tox1_i = assign2320_e1976;
        locals.var_tox1_i_rv = 0.0;

        let (assign2330_e1981,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p198,)
    } else {
        (locals.var_tsi_i,)
    }
};
        locals.var_tsi_i = assign2330_e1981;
        locals.var_tsi_i_rv = 0.0;

        let (assign2340_e1986,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p199,)
    } else {
        (locals.var_xge_i,)
    }
};
        locals.var_xge_i = assign2340_e1986;
        locals.var_xge_i_rv = 0.0;

        let (assign2350_e1991,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p200,)
    } else {
        (locals.var_tox2_i,)
    }
};
        locals.var_tox2_i = assign2350_e1991;
        locals.var_tox2_i_rv = 0.0;

        let (assign2360_e1996,) = {
    if (locals.var_guard83 == 0.0) {
        (1.0,)
    } else {
        (locals.var_typech_i,)
    }
};
        locals.var_typech_i = assign2360_e1996;
        locals.var_typech_i_rv = 0.0;

        let assign2370_e1999: f64 = if p.p201 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard94 = assign2370_e1999;
        locals.var_guard94_rv = 0.0;

        let (assign2380_e2007,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard94 != 0.0)) {
        let assign2380_e2005: f64 = (-1.0);
        (assign2380_e2005,)
    } else {
        (locals.var_typech_i,)
    }
};
        locals.var_typech_i = assign2380_e2007;
        locals.var_typech_i_rv = 0.0;

        let (assign2390_e2017,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2390_e2011: f64 = (p.p201).abs();
        let assign2390_e2013: f64 = (assign2390_e2011).min(1e19);
        let assign2390_e2015: f64 = (assign2390_e2013 * 1000000.0);
        (assign2390_e2015,)
    } else {
        (locals.var_nch_i,)
    }
};
        locals.var_nch_i = assign2390_e2017;
        locals.var_nch_i_rv = 0.0;

        let (assign2400_e2022,) = {
    if (locals.var_guard83 == 0.0) {
        (1.0,)
    } else {
        (locals.var_typesub_i,)
    }
};
        locals.var_typesub_i = assign2400_e2022;
        locals.var_typesub_i_rv = 0.0;

        let assign2410_e2025: f64 = if p.p202 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard95 = assign2410_e2025;
        locals.var_guard95_rv = 0.0;

        let (assign2420_e2033,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard95 != 0.0)) {
        let assign2420_e2031: f64 = (-1.0);
        (assign2420_e2031,)
    } else {
        (locals.var_typesub_i,)
    }
};
        locals.var_typesub_i = assign2420_e2033;
        locals.var_typesub_i_rv = 0.0;

        let (assign2430_e2045,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2430_e2037: f64 = (p.p202).abs();
        let assign2430_e2039: f64 = (assign2430_e2037).max(1e16);
        let assign2430_e2041: f64 = (assign2430_e2039).min(1e21);
        let assign2430_e2043: f64 = (assign2430_e2041 * 1000000.0);
        (assign2430_e2043,)
    } else {
        (locals.var_nsub_i,)
    }
};
        locals.var_nsub_i = assign2430_e2045;
        locals.var_nsub_i_rv = 0.0;

        let (assign2440_e2050,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p203,)
    } else {
        (locals.var_ct_i,)
    }
};
        locals.var_ct_i = assign2440_e2050;
        locals.var_ct_i_rv = 0.0;

        let (assign2450_e2055,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p204,)
    } else {
        (locals.var_toxp_i,)
    }
};
        locals.var_toxp_i = assign2450_e2055;
        locals.var_toxp_i_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2460_e2062,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2460_e2060: f64 = (p.p205 * 1000000.0);
        (assign2460_e2060,)
    } else {
        (locals.var_nov_i,)
    }
};
        locals.var_nov_i = assign2460_e2062;
        locals.var_nov_i_rv = 0.0;

        let (assign2470_e2069,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2470_e2067: f64 = (p.p206 * 1000000.0);
        (assign2470_e2067,)
    } else {
        (locals.var_novd_i,)
    }
};
        locals.var_novd_i = assign2470_e2069;
        locals.var_novd_i_rv = 0.0;

        let (assign2480_e2086, assign2480_e2086_d_n4, assign2480_e2086_d_n6, assign2480_e2086_d_n7, assign2480_e2086_d_n8, assign2480_e2086_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2480_e2075: f64 = (locals.var_ile).powf(p.p209);
        let assign2480_e2076: f64 = (p.p208 * assign2480_e2075);
        let assign2480_e2081: f64 = (locals.var_ile).powf(p.p211);
        let assign2480_e2082: f64 = (p.p210 * assign2480_e2081);
        let assign2480_e2083: f64 = (1.0 + assign2480_e2082);
        let assign2480_e2084: f64 = (assign2480_e2076 / assign2480_e2083);
        (assign2480_e2084, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign2480_e2086;
        locals.var_temp_dn4 = assign2480_e2086_d_n4;
        locals.var_temp_dn6 = assign2480_e2086_d_n6;
        locals.var_temp_dn7 = assign2480_e2086_d_n7;
        locals.var_temp_dn8 = assign2480_e2086_d_n8;
        locals.var_temp_dn9 = assign2480_e2086_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign2490_e2101, assign2490_e2101_d_n4, assign2490_e2101_d_n6, assign2490_e2101_d_n7, assign2490_e2101_d_n8, assign2490_e2101_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2490_e2091: f64 = (p.p207 + locals.var_temp);
        let assign2490_e2094: f64 = (p.p212 * locals.var_iwe);
        let assign2490_e2095: f64 = (assign2490_e2091 + assign2490_e2094);
        let assign2490_e2098: f64 = (p.p213 * locals.var_iae);
        let assign2490_e2099: f64 = (assign2490_e2095 + assign2490_e2098);
        (assign2490_e2099, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    } else {
        (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9,)
    }
};
        locals.var_vfb1_t = assign2490_e2101;
        locals.var_vfb1_t_dn4 = assign2490_e2101_d_n4;
        locals.var_vfb1_t_dn6 = assign2490_e2101_d_n6;
        locals.var_vfb1_t_dn7 = assign2490_e2101_d_n7;
        locals.var_vfb1_t_dn8 = assign2490_e2101_d_n8;
        locals.var_vfb1_t_dn9 = assign2490_e2101_d_n9;
        locals.var_vfb1_t_rv = 0.0;

        let (assign2500_e2114, assign2500_e2114_d_n4, assign2500_e2114_d_n6, assign2500_e2114_d_n7, assign2500_e2114_d_n8, assign2500_e2114_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2500_e2107: f64 = (p.p215 * locals.var_tox2_i);
        let assign2500_e2109: f64 = (assign2500_e2107 / locals.var_tox1_i);
        let assign2500_e2111: f64 = (assign2500_e2109 * locals.var_temp);
        let assign2500_e2112: f64 = (p.p214 + assign2500_e2111);
        (assign2500_e2112, (assign2500_e2109 * locals.var_temp_dn4), (assign2500_e2109 * locals.var_temp_dn6), (assign2500_e2109 * locals.var_temp_dn7), (assign2500_e2109 * locals.var_temp_dn8), (assign2500_e2109 * locals.var_temp_dn9),)
    } else {
        (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9,)
    }
};
        locals.var_vfb2_t = assign2500_e2114;
        locals.var_vfb2_t_dn4 = assign2500_e2114_d_n4;
        locals.var_vfb2_t_dn6 = assign2500_e2114_d_n6;
        locals.var_vfb2_t_dn7 = assign2500_e2114_d_n7;
        locals.var_vfb2_t_dn8 = assign2500_e2114_d_n8;
        locals.var_vfb2_t_dn9 = assign2500_e2114_d_n9;
        locals.var_vfb2_t_rv = 0.0;

        let (assign2510_e2137,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2510_e2121: f64 = (p.p217 * locals.var_ile);
        let assign2510_e2122: f64 = (1.0 + assign2510_e2121);
        let assign2510_e2123: f64 = (p.p216 * assign2510_e2122);
        let assign2510_e2127: f64 = (p.p218 * locals.var_iwe);
        let assign2510_e2128: f64 = (1.0 + assign2510_e2127);
        let assign2510_e2129: f64 = (assign2510_e2123 * assign2510_e2128);
        let assign2510_e2133: f64 = (p.p219 * locals.var_iae);
        let assign2510_e2134: f64 = (1.0 + assign2510_e2133);
        let assign2510_e2135: f64 = (assign2510_e2129 * assign2510_e2134);
        (assign2510_e2135,)
    } else {
        (locals.var_stvfb_i,)
    }
};
        locals.var_stvfb_i = assign2510_e2137;
        locals.var_stvfb_i_rv = 0.0;

        let (assign2520_e2150, assign2520_e2150_d_n4, assign2520_e2150_d_n6, assign2520_e2150_d_n7, assign2520_e2150_d_n8, assign2520_e2150_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2520_e2144: f64 = (p.p221 * locals.var_ile);
        let assign2520_e2145: f64 = (1.0 + assign2520_e2144);
        let assign2520_e2146: f64 = (p.p220 * assign2520_e2145);
        let assign2520_e2148: f64 = (assign2520_e2146 * 1000000.0);
        (assign2520_e2148, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp0__blk79, locals.var_temp0__blk79_dn4, locals.var_temp0__blk79_dn6, locals.var_temp0__blk79_dn7, locals.var_temp0__blk79_dn8, locals.var_temp0__blk79_dn9,)
    }
};
        locals.var_temp0__blk79 = assign2520_e2150;
        locals.var_temp0__blk79_dn4 = assign2520_e2150_d_n4;
        locals.var_temp0__blk79_dn6 = assign2520_e2150_d_n6;
        locals.var_temp0__blk79_dn7 = assign2520_e2150_d_n7;
        locals.var_temp0__blk79_dn8 = assign2520_e2150_d_n8;
        locals.var_temp0__blk79_dn9 = assign2520_e2150_d_n9;
        locals.var_temp0__blk79_rv = 0.0;

        let (assign2530_e2159, assign2530_e2159_d_n4, assign2530_e2159_d_n6, assign2530_e2159_d_n7, assign2530_e2159_d_n8, assign2530_e2159_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2530_e2155: f64 = (locals.var_temp0__blk79).max(1e25);
        let assign2530_e2157: f64 = (assign2530_e2155).min(1e28);
        (assign2530_e2157, if assign2530_e2155 <= 1e28 { if locals.var_temp0__blk79 >= 1e25 { locals.var_temp0__blk79_dn4 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if locals.var_temp0__blk79 >= 1e25 { locals.var_temp0__blk79_dn6 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if locals.var_temp0__blk79 >= 1e25 { locals.var_temp0__blk79_dn7 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if locals.var_temp0__blk79 >= 1e25 { locals.var_temp0__blk79_dn8 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if locals.var_temp0__blk79 >= 1e25 { locals.var_temp0__blk79_dn9 } else { 0.0 } } else { 0.0 },)
    } else {
        (locals.var_np_i, locals.var_np_i_dn4, locals.var_np_i_dn6, locals.var_np_i_dn7, locals.var_np_i_dn8, locals.var_np_i_dn9,)
    }
};
        locals.var_np_i = assign2530_e2159;
        locals.var_np_i_dn4 = assign2530_e2159_d_n4;
        locals.var_np_i_dn6 = assign2530_e2159_d_n6;
        locals.var_np_i_dn7 = assign2530_e2159_d_n7;
        locals.var_np_i_dn8 = assign2530_e2159_d_n8;
        locals.var_np_i_dn9 = assign2530_e2159_d_n9;
        locals.var_np_i_rv = 0.0;

        let (assign2540_e2164,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p222,)
    } else {
        (locals.var_cic1_i,)
    }
};
        locals.var_cic1_i = assign2540_e2164;
        locals.var_cic1_i_rv = 0.0;

        let (assign2550_e2169,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p223,)
    } else {
        (locals.var_cic2_i,)
    }
};
        locals.var_cic2_i = assign2550_e2169;
        locals.var_cic2_i_rv = 0.0;

        let (assign2560_e2176,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2560_e2174: f64 = (1.0 - locals.var_xge_i);
        (assign2560_e2174,)
    } else {
        (locals.var_one_m_xge,)
    }
};
        locals.var_one_m_xge = assign2560_e2176;
        locals.var_one_m_xge_rv = 0.0;

        let (assign2570_e2187,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2570_e2181: f64 = (1.04479e-10 * locals.var_one_m_xge);
        let assign2570_e2184: f64 = (1.43438e-10 * locals.var_xge_i);
        let assign2570_e2185: f64 = (assign2570_e2181 + assign2570_e2184);
        (assign2570_e2185,)
    } else {
        (locals.var_epsch,)
    }
};
        locals.var_epsch = assign2570_e2187;
        locals.var_epsch_rv = 0.0;

        let (assign2580_e2203,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2580_e2192: f64 = (locals.var_epsch / 3.45313e-11);
        let assign2580_e2194: f64 = (assign2580_e2192 * locals.var_tsi_i);
        let assign2580_e2197: f64 = (locals.var_tox1_i + 4e-10);
        let assign2580_e2198: f64 = (assign2580_e2194 * assign2580_e2197);
        let assign2580_e2199: f64 = (assign2580_e2198).sqrt();
        let assign2580_e2201: f64 = (assign2580_e2199 / locals.var_le);
        (assign2580_e2201,)
    } else {
        (locals.var_lambda_le,)
    }
};
        locals.var_lambda_le = assign2580_e2203;
        locals.var_lambda_le_rv = 0.0;

        let (assign2590_e2220,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2590_e2208: f64 = (p.p224 * 2.0);
        let assign2590_e2211: f64 = (locals.var_lambda_le).powf(p.p225);
        let assign2590_e2212: f64 = (assign2590_e2208 * assign2590_e2211);
        let assign2590_e2216: f64 = (p.p226 * locals.var_iwe);
        let assign2590_e2217: f64 = (1.0 + assign2590_e2216);
        let assign2590_e2218: f64 = (assign2590_e2212 * assign2590_e2217);
        (assign2590_e2218,)
    } else {
        (locals.var_psce_p,)
    }
};
        locals.var_psce_p = assign2590_e2220;
        locals.var_psce_p_rv = 0.0;

        let (assign2600_e2229,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2600_e2225: f64 = (locals.var_psce_p).max(0.0);
        let assign2600_e2227: f64 = (assign2600_e2225).min(5.0);
        (assign2600_e2227,)
    } else {
        (locals.var_psce1_i,)
    }
};
        locals.var_psce1_i = assign2600_e2229;
        locals.var_psce1_i_rv = 0.0;

        let (assign2610_e2240,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2610_e2234: f64 = (p.p227 * locals.var_psce1_i);
        let assign2610_e2236: f64 = (assign2610_e2234 * locals.var_tox2_i);
        let assign2610_e2238: f64 = (assign2610_e2236 / locals.var_tox1_i);
        (assign2610_e2238,)
    } else {
        (locals.var_psce2_i,)
    }
};
        locals.var_psce2_i = assign2610_e2240;
        locals.var_psce2_i_rv = 0.0;

        let (assign2620_e2247,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2620_e2245: f64 = (p.p228 * 1000000.0);
        (assign2620_e2245,)
    } else {
        (locals.var_nsddc_i,)
    }
};
        locals.var_nsddc_i = assign2620_e2247;
        locals.var_nsddc_i_rv = 0.0;

        let (assign2630_e2252,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p229,)
    } else {
        (locals.var_pscedlb_i,)
    }
};
        locals.var_pscedlb_i = assign2630_e2252;
        locals.var_pscedlb_i_rv = 0.0;

        let (assign2640_e2259,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2640_e2257: f64 = (p.p230 * locals.var_iwe);
        (assign2640_e2257,)
    } else {
        (locals.var_pnce_p,)
    }
};
        locals.var_pnce_p = assign2640_e2259;
        locals.var_pnce_p_rv = 0.0;

        let (assign2650_e2269,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2650_e2264: f64 = (-1.0);
        let assign2650_e2265: f64 = (locals.var_pnce_p).max(assign2650_e2264);
        let assign2650_e2267: f64 = (assign2650_e2265).min(1.0);
        (assign2650_e2267,)
    } else {
        (locals.var_pnce_i,)
    }
};
        locals.var_pnce_i = assign2650_e2269;
        locals.var_pnce_i_rv = 0.0;

        let (assign2660_e2282, assign2660_e2282_d_n4, assign2660_e2282_d_n6, assign2660_e2282_d_n7, assign2660_e2282_d_n8, assign2660_e2282_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2660_e2274: f64 = (locals.var_lambda_le).powf(p.p232);
        let assign2660_e2278: f64 = (p.p233 * locals.var_iwe);
        let assign2660_e2279: f64 = (1.0 + assign2660_e2278);
        let assign2660_e2280: f64 = (assign2660_e2274 * assign2660_e2279);
        (assign2660_e2280, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign2660_e2282;
        locals.var_temp_dn4 = assign2660_e2282_d_n4;
        locals.var_temp_dn6 = assign2660_e2282_d_n6;
        locals.var_temp_dn7 = assign2660_e2282_d_n7;
        locals.var_temp_dn8 = assign2660_e2282_d_n8;
        locals.var_temp_dn9 = assign2660_e2282_d_n9;
        locals.var_temp_rv = 0.0;

        let (assign2670_e2289, assign2670_e2289_d_n4, assign2670_e2289_d_n6, assign2670_e2289_d_n7, assign2670_e2289_d_n8, assign2670_e2289_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2670_e2287: f64 = (p.p231 * locals.var_temp);
        (assign2670_e2287, (p.p231 * locals.var_temp_dn4), (p.p231 * locals.var_temp_dn6), (p.p231 * locals.var_temp_dn7), (p.p231 * locals.var_temp_dn8), (p.p231 * locals.var_temp_dn9),)
    } else {
        (locals.var_cf_p, locals.var_cf_p_dn4, locals.var_cf_p_dn6, locals.var_cf_p_dn7, locals.var_cf_p_dn8, locals.var_cf_p_dn9,)
    }
};
        locals.var_cf_p = assign2670_e2289;
        locals.var_cf_p_dn4 = assign2670_e2289_d_n4;
        locals.var_cf_p_dn6 = assign2670_e2289_d_n6;
        locals.var_cf_p_dn7 = assign2670_e2289_d_n7;
        locals.var_cf_p_dn8 = assign2670_e2289_d_n8;
        locals.var_cf_p_dn9 = assign2670_e2289_d_n9;
        locals.var_cf_p_rv = 0.0;

        let (assign2680_e2296, assign2680_e2296_d_n4, assign2680_e2296_d_n6, assign2680_e2296_d_n7, assign2680_e2296_d_n8, assign2680_e2296_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2680_e2294: f64 = (locals.var_cf_p).max(0.0);
        (assign2680_e2294, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn4 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn6 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn7 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn8 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9,)
    }
};
        locals.var_cf1_t = assign2680_e2296;
        locals.var_cf1_t_dn4 = assign2680_e2296_d_n4;
        locals.var_cf1_t_dn6 = assign2680_e2296_d_n6;
        locals.var_cf1_t_dn7 = assign2680_e2296_d_n7;
        locals.var_cf1_t_dn8 = assign2680_e2296_d_n8;
        locals.var_cf1_t_dn9 = assign2680_e2296_d_n9;
        locals.var_cf1_t_rv = 0.0;

        let (assign2690_e2307, assign2690_e2307_d_n4, assign2690_e2307_d_n6, assign2690_e2307_d_n7, assign2690_e2307_d_n8, assign2690_e2307_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2690_e2301: f64 = (p.p234 * locals.var_cf1_t);
        let assign2690_e2303: f64 = (assign2690_e2301 * locals.var_tox2_i);
        let assign2690_e2305: f64 = (assign2690_e2303 / locals.var_tox1_i);
        (assign2690_e2305, (((p.p234 * locals.var_cf1_t_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cf1_t_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cf1_t_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cf1_t_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cf1_t_dn9) * locals.var_tox2_i) / locals.var_tox1_i),)
    } else {
        (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9,)
    }
};
        locals.var_cf2_t = assign2690_e2307;
        locals.var_cf2_t_dn4 = assign2690_e2307_d_n4;
        locals.var_cf2_t_dn6 = assign2690_e2307_d_n6;
        locals.var_cf2_t_dn7 = assign2690_e2307_d_n7;
        locals.var_cf2_t_dn8 = assign2690_e2307_d_n8;
        locals.var_cf2_t_dn9 = assign2690_e2307_d_n9;
        locals.var_cf2_t_rv = 0.0;

        let (assign2700_e2314, assign2700_e2314_d_n4, assign2700_e2314_d_n6, assign2700_e2314_d_n7, assign2700_e2314_d_n8, assign2700_e2314_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2700_e2312: f64 = (p.p235 * locals.var_temp);
        (assign2700_e2312, (p.p235 * locals.var_temp_dn4), (p.p235 * locals.var_temp_dn6), (p.p235 * locals.var_temp_dn7), (p.p235 * locals.var_temp_dn8), (p.p235 * locals.var_temp_dn9),)
    } else {
        (locals.var_stcf_i, locals.var_stcf_i_dn4, locals.var_stcf_i_dn6, locals.var_stcf_i_dn7, locals.var_stcf_i_dn8, locals.var_stcf_i_dn9,)
    }
};
        locals.var_stcf_i = assign2700_e2314;
        locals.var_stcf_i_dn4 = assign2700_e2314_d_n4;
        locals.var_stcf_i_dn6 = assign2700_e2314_d_n6;
        locals.var_stcf_i_dn7 = assign2700_e2314_d_n7;
        locals.var_stcf_i_dn8 = assign2700_e2314_d_n8;
        locals.var_stcf_i_dn9 = assign2700_e2314_d_n9;
        locals.var_stcf_i_rv = 0.0;

        let (assign2710_e2319,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p236,)
    } else {
        (locals.var_cfd_i,)
    }
};
        locals.var_cfd_i = assign2710_e2319;
        locals.var_cfd_i_rv = 0.0;

        let (assign2720_e2334,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2720_e2324: f64 = (p.p237 * locals.var_ile);
        let assign2720_e2328: f64 = (p.p238 * locals.var_iwe);
        let assign2720_e2329: f64 = (1.0 + assign2720_e2328);
        let assign2720_e2331: f64 = (assign2720_e2329).max(0.001);
        let assign2720_e2332: f64 = (assign2720_e2324 / assign2720_e2331);
        (assign2720_e2332,)
    } else {
        (locals.var_cfdl_i,)
    }
};
        locals.var_cfdl_i = assign2720_e2334;
        locals.var_cfdl_i_rv = 0.0;

        let (assign2730_e2339,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p239,)
    } else {
        (locals.var_cfdlb_i,)
    }
};
        locals.var_cfdlb_i = assign2730_e2339;
        locals.var_cfdlb_i_rv = 0.0;

        let (assign2740_e2355, assign2740_e2355_d_n4, assign2740_e2355_d_n6, assign2740_e2355_d_n7, assign2740_e2355_d_n8, assign2740_e2355_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2740_e2343: f64 = (-locals.var_le);
        let assign2740_e2348: f64 = (p.p244 * locals.var_iwe);
        let assign2740_e2349: f64 = (1.0 + assign2740_e2348);
        let assign2740_e2351: f64 = (assign2740_e2349).max(0.001);
        let assign2740_e2352: f64 = (p.p243 * assign2740_e2351);
        let assign2740_e2353: f64 = (assign2740_e2343 / assign2740_e2352);
        (assign2740_e2353, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign2740_e2355;
        locals.var_temp1_dn4 = assign2740_e2355_d_n4;
        locals.var_temp1_dn6 = assign2740_e2355_d_n6;
        locals.var_temp1_dn7 = assign2740_e2355_d_n7;
        locals.var_temp1_dn8 = assign2740_e2355_d_n8;
        locals.var_temp1_dn9 = assign2740_e2355_d_n9;
        locals.var_temp1_rv = 0.0;

        let assign2750_e2358: f64 = (-80.0);
        let assign2750_e2359: f64 = if locals.var_temp1 > assign2750_e2358 { 1.0 } else { 0.0 };
        locals.var_guard96 = assign2750_e2359;
        locals.var_guard96_rv = 0.0;

        let (assign2760_e2367, assign2760_e2367_d_n4, assign2760_e2367_d_n6, assign2760_e2367_d_n7, assign2760_e2367_d_n8, assign2760_e2367_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard96 != 0.0)) {
        let assign2760_e2365: f64 = (locals.var_temp1).exp();
        (assign2760_e2365, (assign2760_e2365 * locals.var_temp1_dn4), (assign2760_e2365 * locals.var_temp1_dn6), (assign2760_e2365 * locals.var_temp1_dn7), (assign2760_e2365 * locals.var_temp1_dn8), (assign2760_e2365 * locals.var_temp1_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign2760_e2367;
        locals.var_temp2_dn4 = assign2760_e2367_d_n4;
        locals.var_temp2_dn6 = assign2760_e2367_d_n6;
        locals.var_temp2_dn7 = assign2760_e2367_d_n7;
        locals.var_temp2_dn8 = assign2760_e2367_d_n8;
        locals.var_temp2_dn9 = assign2760_e2367_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign2770_e2400, assign2770_e2400_d_n4, assign2770_e2400_d_n6, assign2770_e2400_d_n7, assign2770_e2400_d_n8, assign2770_e2400_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard96 == 0.0)) {
        let assign2770_e2376: f64 = (-locals.var_temp1);
        let assign2770_e2378: f64 = (assign2770_e2376 - 80.0);
        let assign2770_e2382: f64 = (-locals.var_temp1);
        let assign2770_e2384: f64 = (assign2770_e2382 - 80.0);
        let assign2770_e2385: f64 = (0.5 * assign2770_e2384);
        let assign2770_e2388: f64 = (-locals.var_temp1);
        let assign2770_e2390: f64 = (assign2770_e2388 - 80.0);
        let assign2770_e2392: f64 = (assign2770_e2390 * 0.3333333333333);
        let assign2770_e2393: f64 = (1.0 + assign2770_e2392);
        let assign2770_e2394: f64 = (assign2770_e2385 * assign2770_e2393);
        let assign2770_e2395: f64 = (1.0 + assign2770_e2394);
        let assign2770_e2396: f64 = (assign2770_e2378 * assign2770_e2395);
        let assign2770_e2397: f64 = (1.0 + assign2770_e2396);
        let assign2770_e2398: f64 = (1.80485e-35 / assign2770_e2397);
        (assign2770_e2398, (-((1.80485e-35 * (((-locals.var_temp1_dn4) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-locals.var_temp1_dn4)) * assign2770_e2393) + (assign2770_e2385 * ((-locals.var_temp1_dn4) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-locals.var_temp1_dn6) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-locals.var_temp1_dn6)) * assign2770_e2393) + (assign2770_e2385 * ((-locals.var_temp1_dn6) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-locals.var_temp1_dn7) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-locals.var_temp1_dn7)) * assign2770_e2393) + (assign2770_e2385 * ((-locals.var_temp1_dn7) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-locals.var_temp1_dn8) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-locals.var_temp1_dn8)) * assign2770_e2393) + (assign2770_e2385 * ((-locals.var_temp1_dn8) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-locals.var_temp1_dn9) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-locals.var_temp1_dn9)) * assign2770_e2393) + (assign2770_e2385 * ((-locals.var_temp1_dn9) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign2770_e2400;
        locals.var_temp2_dn4 = assign2770_e2400_d_n4;
        locals.var_temp2_dn6 = assign2770_e2400_d_n6;
        locals.var_temp2_dn7 = assign2770_e2400_d_n7;
        locals.var_temp2_dn8 = assign2770_e2400_d_n8;
        locals.var_temp2_dn9 = assign2770_e2400_d_n9;
        locals.var_temp2_rv = 0.0;

        let (assign2780_e2408, assign2780_e2408_d_n4, assign2780_e2408_d_n6, assign2780_e2408_d_n7, assign2780_e2408_d_n8, assign2780_e2408_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2780_e2404: f64 = (-locals.var_le);
        let assign2780_e2406: f64 = (assign2780_e2404 / p.p246);
        (assign2780_e2406, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign2780_e2408;
        locals.var_temp3_dn4 = assign2780_e2408_d_n4;
        locals.var_temp3_dn6 = assign2780_e2408_d_n6;
        locals.var_temp3_dn7 = assign2780_e2408_d_n7;
        locals.var_temp3_dn8 = assign2780_e2408_d_n8;
        locals.var_temp3_dn9 = assign2780_e2408_d_n9;
        locals.var_temp3_rv = 0.0;

        let assign2790_e2411: f64 = (-80.0);
        let assign2790_e2412: f64 = if locals.var_temp3 > assign2790_e2411 { 1.0 } else { 0.0 };
        locals.var_guard97 = assign2790_e2412;
        locals.var_guard97_rv = 0.0;

        let (assign2800_e2420, assign2800_e2420_d_n4, assign2800_e2420_d_n6, assign2800_e2420_d_n7, assign2800_e2420_d_n8, assign2800_e2420_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard97 != 0.0)) {
        let assign2800_e2418: f64 = (locals.var_temp3).exp();
        (assign2800_e2418, (assign2800_e2418 * locals.var_temp3_dn4), (assign2800_e2418 * locals.var_temp3_dn6), (assign2800_e2418 * locals.var_temp3_dn7), (assign2800_e2418 * locals.var_temp3_dn8), (assign2800_e2418 * locals.var_temp3_dn9),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign2800_e2420;
        locals.var_temp4_dn4 = assign2800_e2420_d_n4;
        locals.var_temp4_dn6 = assign2800_e2420_d_n6;
        locals.var_temp4_dn7 = assign2800_e2420_d_n7;
        locals.var_temp4_dn8 = assign2800_e2420_d_n8;
        locals.var_temp4_dn9 = assign2800_e2420_d_n9;
        locals.var_temp4_rv = 0.0;

    }
}
