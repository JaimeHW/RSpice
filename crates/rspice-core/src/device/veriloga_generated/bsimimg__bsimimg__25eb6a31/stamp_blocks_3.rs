#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_17(
        locals: &mut StampLocals,
    ) {
        let (assign8130_e8114, assign8130_e8114_d_n3, assign8130_e8114_d_n4, assign8130_e8114_d_n5, assign8130_e8114_d_n6, assign8130_e8114_d_n7, assign8130_e8114_d_n8,) = {
    if (locals.var_guard91 != 0.0) {
        let assign8130_e8096: f64 = (-locals.var_t5);
        let assign8130_e8098: f64 = (-4.0);
        let assign8130_e8100: f64 = (assign8130_e8098 * locals.var_t4);
        let assign8130_e8102: f64 = (assign8130_e8100 * locals.var_t6);
        let assign8130_e8105: f64 = (locals.var_t5 * locals.var_t5);
        let assign8130_e8106: f64 = (assign8130_e8102 + assign8130_e8105);
        let assign8130_e8107: f64 = (assign8130_e8106).sqrt();
        let assign8130_e8108: f64 = (assign8130_e8096 + assign8130_e8107);
        let assign8130_e8111: f64 = (2.0 * locals.var_t4);
        let assign8130_e8112: f64 = (assign8130_e8108 / assign8130_e8111);
        (assign8130_e8112, (((((-locals.var_t5_dn3) + (((((assign8130_e8098 * locals.var_t4_dn3) * locals.var_t6) + (assign8130_e8100 * locals.var_t6_dn3)) + ((locals.var_t5_dn3 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn3))) / (2.0 * assign8130_e8107))) * assign8130_e8111) - (assign8130_e8108 * (2.0 * locals.var_t4_dn3))) / (assign8130_e8111 * assign8130_e8111)), (((((-locals.var_t5_dn4) + (((((assign8130_e8098 * locals.var_t4_dn4) * locals.var_t6) + (assign8130_e8100 * locals.var_t6_dn4)) + ((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4))) / (2.0 * assign8130_e8107))) * assign8130_e8111) - (assign8130_e8108 * (2.0 * locals.var_t4_dn4))) / (assign8130_e8111 * assign8130_e8111)), (((((-locals.var_t5_dn5) + (((((assign8130_e8098 * locals.var_t4_dn5) * locals.var_t6) + (assign8130_e8100 * locals.var_t6_dn5)) + ((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5))) / (2.0 * assign8130_e8107))) * assign8130_e8111) - (assign8130_e8108 * (2.0 * locals.var_t4_dn5))) / (assign8130_e8111 * assign8130_e8111)), (((((-locals.var_t5_dn6) + (((((assign8130_e8098 * locals.var_t4_dn6) * locals.var_t6) + (assign8130_e8100 * locals.var_t6_dn6)) + ((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6))) / (2.0 * assign8130_e8107))) * assign8130_e8111) - (assign8130_e8108 * (2.0 * locals.var_t4_dn6))) / (assign8130_e8111 * assign8130_e8111)), (((((-locals.var_t5_dn7) + (((((assign8130_e8098 * locals.var_t4_dn7) * locals.var_t6) + (assign8130_e8100 * locals.var_t6_dn7)) + ((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7))) / (2.0 * assign8130_e8107))) * assign8130_e8111) - (assign8130_e8108 * (2.0 * locals.var_t4_dn7))) / (assign8130_e8111 * assign8130_e8111)), (((((-locals.var_t5_dn8) + (((((assign8130_e8098 * locals.var_t4_dn8) * locals.var_t6) + (assign8130_e8100 * locals.var_t6_dn8)) + ((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8))) / (2.0 * assign8130_e8107))) * assign8130_e8111) - (assign8130_e8108 * (2.0 * locals.var_t4_dn8))) / (assign8130_e8111 * assign8130_e8111)),)
    } else {
        (locals.var_qsqrt, locals.var_qsqrt_dn3, locals.var_qsqrt_dn4, locals.var_qsqrt_dn5, locals.var_qsqrt_dn6, locals.var_qsqrt_dn7, locals.var_qsqrt_dn8,)
    }
};
        locals.var_qsqrt = assign8130_e8114;
        locals.var_qsqrt_dn3 = assign8130_e8114_d_n3;
        locals.var_qsqrt_dn4 = assign8130_e8114_d_n4;
        locals.var_qsqrt_dn5 = assign8130_e8114_d_n5;
        locals.var_qsqrt_dn6 = assign8130_e8114_d_n6;
        locals.var_qsqrt_dn7 = assign8130_e8114_d_n7;
        locals.var_qsqrt_dn8 = assign8130_e8114_d_n8;
        locals.var_qsqrt_rv = 0.0;

        let (assign8140_e8126, assign8140_e8126_d_n3, assign8140_e8126_d_n4, assign8140_e8126_d_n5, assign8140_e8126_d_n6, assign8140_e8126_d_n7, assign8140_e8126_d_n8,) = {
    if (locals.var_guard91 != 0.0) {
        let assign8140_e8119: f64 = (1.0 + locals.var_k1);
        let assign8140_e8120: f64 = (locals.var_phi1_0 * assign8140_e8119);
        let assign8140_e8122: f64 = (assign8140_e8120 - locals.var_phi2);
        let assign8140_e8124: f64 = (assign8140_e8122 / locals.var_k1);
        (assign8140_e8124, (((locals.var_phi1_0_dn3 * assign8140_e8119) - locals.var_phi2_dn3) / locals.var_k1), (((locals.var_phi1_0_dn4 * assign8140_e8119) - locals.var_phi2_dn4) / locals.var_k1), (((locals.var_phi1_0_dn5 * assign8140_e8119) - locals.var_phi2_dn5) / locals.var_k1), (((locals.var_phi1_0_dn6 * assign8140_e8119) - locals.var_phi2_dn6) / locals.var_k1), (((locals.var_phi1_0_dn7 * assign8140_e8119) - locals.var_phi2_dn7) / locals.var_k1), (((locals.var_phi1_0_dn8 * assign8140_e8119) - locals.var_phi2_dn8) / locals.var_k1),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8,)
    }
};
        locals.var_t3 = assign8140_e8126;
        locals.var_t3_dn3 = assign8140_e8126_d_n3;
        locals.var_t3_dn4 = assign8140_e8126_d_n4;
        locals.var_t3_dn5 = assign8140_e8126_d_n5;
        locals.var_t3_dn6 = assign8140_e8126_d_n6;
        locals.var_t3_dn7 = assign8140_e8126_d_n7;
        locals.var_t3_dn8 = assign8140_e8126_d_n8;
        locals.var_t3_rv = 0.0;

        let (assign8150_e8138, assign8150_e8138_d_n3, assign8150_e8138_d_n4, assign8150_e8138_d_n5, assign8150_e8138_d_n6, assign8150_e8138_d_n7, assign8150_e8138_d_n8,) = {
    if (locals.var_guard91 != 0.0) {
        let assign8150_e8131: f64 = (locals.var_xg1 - locals.var_t3);
        let assign8150_e8133: f64 = (assign8150_e8131 + 2.0);
        let assign8150_e8134: f64 = (40.0 * assign8150_e8133);
        let assign8150_e8136: f64 = (assign8150_e8134 / 5.0);
        (assign8150_e8136, ((40.0 * (locals.var_xg1_dn3 - locals.var_t3_dn3)) / 5.0), ((40.0 * (locals.var_xg1_dn4 - locals.var_t3_dn4)) / 5.0), ((40.0 * (locals.var_xg1_dn5 - locals.var_t3_dn5)) / 5.0), ((40.0 * (locals.var_xg1_dn6 - locals.var_t3_dn6)) / 5.0), ((40.0 * (locals.var_xg1_dn7 - locals.var_t3_dn7)) / 5.0), ((40.0 * (locals.var_xg1_dn8 - locals.var_t3_dn8)) / 5.0),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    }
};
        locals.var_t0 = assign8150_e8138;
        locals.var_t0_dn3 = assign8150_e8138_d_n3;
        locals.var_t0_dn4 = assign8150_e8138_d_n4;
        locals.var_t0_dn5 = assign8150_e8138_d_n5;
        locals.var_t0_dn6 = assign8150_e8138_d_n6;
        locals.var_t0_dn7 = assign8150_e8138_d_n7;
        locals.var_t0_dn8 = assign8150_e8138_d_n8;
        locals.var_t0_rv = 0.0;

        let (assign8160_e8156, assign8160_e8156_d_n3, assign8160_e8156_d_n4, assign8160_e8156_d_n5, assign8160_e8156_d_n6, assign8160_e8156_d_n7, assign8160_e8156_d_n8,) = {
    if (locals.var_guard91 != 0.0) {
        let assign8160_e8144: f64 = (locals.var_xg1 - locals.var_t3);
        let assign8160_e8146: f64 = (assign8160_e8144 + 2.0);
        let assign8160_e8147: f64 = (-assign8160_e8146);
        let assign8160_e8150: f64 = (2.0 / 0.69);
        let assign8160_e8151: f64 = (assign8160_e8147 / assign8160_e8150);
        let assign8160_e8152: f64 = (assign8160_e8151).exp();
        let assign8160_e8153: f64 = (1.0 - assign8160_e8152);
        let assign8160_e8154: f64 = (locals.var_qsqrt * assign8160_e8153);
        (assign8160_e8154, ((locals.var_qsqrt_dn3 * assign8160_e8153) + (locals.var_qsqrt * (-(assign8160_e8152 * ((-(locals.var_xg1_dn3 - locals.var_t3_dn3)) / assign8160_e8150))))), ((locals.var_qsqrt_dn4 * assign8160_e8153) + (locals.var_qsqrt * (-(assign8160_e8152 * ((-(locals.var_xg1_dn4 - locals.var_t3_dn4)) / assign8160_e8150))))), ((locals.var_qsqrt_dn5 * assign8160_e8153) + (locals.var_qsqrt * (-(assign8160_e8152 * ((-(locals.var_xg1_dn5 - locals.var_t3_dn5)) / assign8160_e8150))))), ((locals.var_qsqrt_dn6 * assign8160_e8153) + (locals.var_qsqrt * (-(assign8160_e8152 * ((-(locals.var_xg1_dn6 - locals.var_t3_dn6)) / assign8160_e8150))))), ((locals.var_qsqrt_dn7 * assign8160_e8153) + (locals.var_qsqrt * (-(assign8160_e8152 * ((-(locals.var_xg1_dn7 - locals.var_t3_dn7)) / assign8160_e8150))))), ((locals.var_qsqrt_dn8 * assign8160_e8153) + (locals.var_qsqrt * (-(assign8160_e8152 * ((-(locals.var_xg1_dn8 - locals.var_t3_dn8)) / assign8160_e8150))))),)
    } else {
        (locals.var_qsqrt, locals.var_qsqrt_dn3, locals.var_qsqrt_dn4, locals.var_qsqrt_dn5, locals.var_qsqrt_dn6, locals.var_qsqrt_dn7, locals.var_qsqrt_dn8,)
    }
};
        locals.var_qsqrt = assign8160_e8156;
        locals.var_qsqrt_dn3 = assign8160_e8156_d_n3;
        locals.var_qsqrt_dn4 = assign8160_e8156_d_n4;
        locals.var_qsqrt_dn5 = assign8160_e8156_d_n5;
        locals.var_qsqrt_dn6 = assign8160_e8156_d_n6;
        locals.var_qsqrt_dn7 = assign8160_e8156_d_n7;
        locals.var_qsqrt_dn8 = assign8160_e8156_d_n8;
        locals.var_qsqrt_rv = 0.0;

        let (assign8170_e8162, assign8170_e8162_d_n3, assign8170_e8162_d_n4, assign8170_e8162_d_n5, assign8170_e8162_d_n6, assign8170_e8162_d_n7, assign8170_e8162_d_n8,) = {
    if (locals.var_guard91 != 0.0) {
        let assign8170_e8160: f64 = (locals.var_qsqrt).min(50.0);
        (assign8170_e8160, if locals.var_qsqrt <= 50.0 { locals.var_qsqrt_dn3 } else { 0.0 }, if locals.var_qsqrt <= 50.0 { locals.var_qsqrt_dn4 } else { 0.0 }, if locals.var_qsqrt <= 50.0 { locals.var_qsqrt_dn5 } else { 0.0 }, if locals.var_qsqrt <= 50.0 { locals.var_qsqrt_dn6 } else { 0.0 }, if locals.var_qsqrt <= 50.0 { locals.var_qsqrt_dn7 } else { 0.0 }, if locals.var_qsqrt <= 50.0 { locals.var_qsqrt_dn8 } else { 0.0 },)
    } else {
        (locals.var_qsqrt, locals.var_qsqrt_dn3, locals.var_qsqrt_dn4, locals.var_qsqrt_dn5, locals.var_qsqrt_dn6, locals.var_qsqrt_dn7, locals.var_qsqrt_dn8,)
    }
};
        locals.var_qsqrt = assign8170_e8162;
        locals.var_qsqrt_dn3 = assign8170_e8162_d_n3;
        locals.var_qsqrt_dn4 = assign8170_e8162_d_n4;
        locals.var_qsqrt_dn5 = assign8170_e8162_d_n5;
        locals.var_qsqrt_dn6 = assign8170_e8162_d_n6;
        locals.var_qsqrt_dn7 = assign8170_e8162_d_n7;
        locals.var_qsqrt_dn8 = assign8170_e8162_d_n8;
        locals.var_qsqrt_rv = 0.0;

        let assign8180_e8165: f64 = (locals.var_xg1).max(locals.var_phi1_0);
        locals.var_xg1 = assign8180_e8165;
        locals.var_xg1_dn3 = if locals.var_xg1 >= locals.var_phi1_0 { locals.var_xg1_dn3 } else { locals.var_phi1_0_dn3 };
        locals.var_xg1_dn4 = if locals.var_xg1 >= locals.var_phi1_0 { locals.var_xg1_dn4 } else { locals.var_phi1_0_dn4 };
        locals.var_xg1_dn5 = if locals.var_xg1 >= locals.var_phi1_0 { locals.var_xg1_dn5 } else { locals.var_phi1_0_dn5 };
        locals.var_xg1_dn6 = if locals.var_xg1 >= locals.var_phi1_0 { locals.var_xg1_dn6 } else { locals.var_phi1_0_dn6 };
        locals.var_xg1_dn7 = if locals.var_xg1 >= locals.var_phi1_0 { locals.var_xg1_dn7 } else { locals.var_phi1_0_dn7 };
        locals.var_xg1_dn8 = if locals.var_xg1 >= locals.var_phi1_0 { locals.var_xg1_dn8 } else { locals.var_phi1_0_dn8 };
        locals.var_xg1_rv = 0.0;

        let assign8190_e8169: f64 = (locals.var_xg1 - locals.var_phi1_0);
        let assign8190_e8170: f64 = (locals.var_k1_2 * assign8190_e8169);
        let assign8190_e8173: f64 = (locals.var_xg1 - locals.var_phi1_0);
        let assign8190_e8174: f64 = (assign8190_e8170 * assign8190_e8173);
        let assign8190_e8176: f64 = (assign8190_e8174 + 39.47841);
        let assign8190_e8177: f64 = (assign8190_e8176).ln();
        let assign8190_e8179: f64 = (assign8190_e8177 - locals.var_lna0);
        locals.var_phissat = assign8190_e8179;
        locals.var_phissat_dn3 = (((((locals.var_k1_2 * (locals.var_xg1_dn3 - locals.var_phi1_0_dn3)) * assign8190_e8173) + (assign8190_e8170 * (locals.var_xg1_dn3 - locals.var_phi1_0_dn3))) / assign8190_e8176) - locals.var_lna0_dn3);
        locals.var_phissat_dn4 = (((((locals.var_k1_2 * (locals.var_xg1_dn4 - locals.var_phi1_0_dn4)) * assign8190_e8173) + (assign8190_e8170 * (locals.var_xg1_dn4 - locals.var_phi1_0_dn4))) / assign8190_e8176) - locals.var_lna0_dn4);
        locals.var_phissat_dn5 = (((((locals.var_k1_2 * (locals.var_xg1_dn5 - locals.var_phi1_0_dn5)) * assign8190_e8173) + (assign8190_e8170 * (locals.var_xg1_dn5 - locals.var_phi1_0_dn5))) / assign8190_e8176) - locals.var_lna0_dn5);
        locals.var_phissat_dn6 = (((((locals.var_k1_2 * (locals.var_xg1_dn6 - locals.var_phi1_0_dn6)) * assign8190_e8173) + (assign8190_e8170 * (locals.var_xg1_dn6 - locals.var_phi1_0_dn6))) / assign8190_e8176) - locals.var_lna0_dn6);
        locals.var_phissat_dn7 = (((((locals.var_k1_2 * (locals.var_xg1_dn7 - locals.var_phi1_0_dn7)) * assign8190_e8173) + (assign8190_e8170 * (locals.var_xg1_dn7 - locals.var_phi1_0_dn7))) / assign8190_e8176) - locals.var_lna0_dn7);
        locals.var_phissat_dn8 = (((((locals.var_k1_2 * (locals.var_xg1_dn8 - locals.var_phi1_0_dn8)) * assign8190_e8173) + (assign8190_e8170 * (locals.var_xg1_dn8 - locals.var_phi1_0_dn8))) / assign8190_e8176) - locals.var_lna0_dn8);
        locals.var_phissat_rv = 0.0;

        let assign8200_e8183: f64 = (1.0 + locals.var_k1);
        let assign8200_e8184: f64 = (locals.var_phi1_0 * assign8200_e8183);
        let assign8200_e8186: f64 = (assign8200_e8184 - locals.var_phi2);
        let assign8200_e8188: f64 = (assign8200_e8186 / locals.var_k1);
        locals.var_t3 = assign8200_e8188;
        locals.var_t3_dn3 = (((locals.var_phi1_0_dn3 * assign8200_e8183) - locals.var_phi2_dn3) / locals.var_k1);
        locals.var_t3_dn4 = (((locals.var_phi1_0_dn4 * assign8200_e8183) - locals.var_phi2_dn4) / locals.var_k1);
        locals.var_t3_dn5 = (((locals.var_phi1_0_dn5 * assign8200_e8183) - locals.var_phi2_dn5) / locals.var_k1);
        locals.var_t3_dn6 = (((locals.var_phi1_0_dn6 * assign8200_e8183) - locals.var_phi2_dn6) / locals.var_k1);
        locals.var_t3_dn7 = (((locals.var_phi1_0_dn7 * assign8200_e8183) - locals.var_phi2_dn7) / locals.var_k1);
        locals.var_t3_dn8 = (((locals.var_phi1_0_dn8 * assign8200_e8183) - locals.var_phi2_dn8) / locals.var_k1);
        locals.var_t3_rv = 0.0;

        let assign8210_e8192: f64 = (locals.var_t3 - locals.var_phi1_0);
        let assign8210_e8193: f64 = (locals.var_k1_2 * assign8210_e8192);
        let assign8210_e8196: f64 = (locals.var_t3 - locals.var_phi1_0);
        let assign8210_e8197: f64 = (assign8210_e8193 * assign8210_e8196);
        let assign8210_e8199: f64 = (assign8210_e8197 + 39.47841);
        let assign8210_e8200: f64 = (assign8210_e8199).ln();
        let assign8210_e8202: f64 = (assign8210_e8200 - locals.var_lna0);
        locals.var_t4 = assign8210_e8202;
        locals.var_t4_dn3 = (((((locals.var_k1_2 * (locals.var_t3_dn3 - locals.var_phi1_0_dn3)) * assign8210_e8196) + (assign8210_e8193 * (locals.var_t3_dn3 - locals.var_phi1_0_dn3))) / assign8210_e8199) - locals.var_lna0_dn3);
        locals.var_t4_dn4 = (((((locals.var_k1_2 * (locals.var_t3_dn4 - locals.var_phi1_0_dn4)) * assign8210_e8196) + (assign8210_e8193 * (locals.var_t3_dn4 - locals.var_phi1_0_dn4))) / assign8210_e8199) - locals.var_lna0_dn4);
        locals.var_t4_dn5 = (((((locals.var_k1_2 * (locals.var_t3_dn5 - locals.var_phi1_0_dn5)) * assign8210_e8196) + (assign8210_e8193 * (locals.var_t3_dn5 - locals.var_phi1_0_dn5))) / assign8210_e8199) - locals.var_lna0_dn5);
        locals.var_t4_dn6 = (((((locals.var_k1_2 * (locals.var_t3_dn6 - locals.var_phi1_0_dn6)) * assign8210_e8196) + (assign8210_e8193 * (locals.var_t3_dn6 - locals.var_phi1_0_dn6))) / assign8210_e8199) - locals.var_lna0_dn6);
        locals.var_t4_dn7 = (((((locals.var_k1_2 * (locals.var_t3_dn7 - locals.var_phi1_0_dn7)) * assign8210_e8196) + (assign8210_e8193 * (locals.var_t3_dn7 - locals.var_phi1_0_dn7))) / assign8210_e8199) - locals.var_lna0_dn7);
        locals.var_t4_dn8 = (((((locals.var_k1_2 * (locals.var_t3_dn8 - locals.var_phi1_0_dn8)) * assign8210_e8196) + (assign8210_e8193 * (locals.var_t3_dn8 - locals.var_phi1_0_dn8))) / assign8210_e8199) - locals.var_lna0_dn8);
        locals.var_t4_rv = 0.0;

        let assign8220_e8205: f64 = (locals.var_t4 - locals.var_phi1_0);
        locals.var_t5 = assign8220_e8205;
        locals.var_t5_dn3 = (locals.var_t4_dn3 - locals.var_phi1_0_dn3);
        locals.var_t5_dn4 = (locals.var_t4_dn4 - locals.var_phi1_0_dn4);
        locals.var_t5_dn5 = (locals.var_t4_dn5 - locals.var_phi1_0_dn5);
        locals.var_t5_dn6 = (locals.var_t4_dn6 - locals.var_phi1_0_dn6);
        locals.var_t5_dn7 = (locals.var_t4_dn7 - locals.var_phi1_0_dn7);
        locals.var_t5_dn8 = (locals.var_t4_dn8 - locals.var_phi1_0_dn8);
        locals.var_t5_rv = 0.0;

        let assign8230_e8208: f64 = (locals.var_phissat - locals.var_t5);
        locals.var_phissat = assign8230_e8208;
        locals.var_phissat_dn3 = (locals.var_phissat_dn3 - locals.var_t5_dn3);
        locals.var_phissat_dn4 = (locals.var_phissat_dn4 - locals.var_t5_dn4);
        locals.var_phissat_dn5 = (locals.var_phissat_dn5 - locals.var_t5_dn5);
        locals.var_phissat_dn6 = (locals.var_phissat_dn6 - locals.var_t5_dn6);
        locals.var_phissat_dn7 = (locals.var_phissat_dn7 - locals.var_t5_dn7);
        locals.var_phissat_dn8 = (locals.var_phissat_dn8 - locals.var_t5_dn8);
        locals.var_phissat_rv = 0.0;

        let assign8240_e8211: f64 = (locals.var_xg1 - locals.var_phissat);
        locals.var_q1 = assign8240_e8211;
        locals.var_q1_dn3 = (locals.var_xg1_dn3 - locals.var_phissat_dn3);
        locals.var_q1_dn4 = (locals.var_xg1_dn4 - locals.var_phissat_dn4);
        locals.var_q1_dn5 = (locals.var_xg1_dn5 - locals.var_phissat_dn5);
        locals.var_q1_dn6 = (locals.var_xg1_dn6 - locals.var_phissat_dn6);
        locals.var_q1_dn7 = (locals.var_xg1_dn7 - locals.var_phissat_dn7);
        locals.var_q1_dn8 = (locals.var_xg1_dn8 - locals.var_phissat_dn8);
        locals.var_q1_rv = 0.0;

        let assign8250_e8213: f64 = (-locals.var_a0);
        let assign8250_e8215: f64 = (locals.var_phissat).exp();
        let assign8250_e8216: f64 = (assign8250_e8213 * assign8250_e8215);
        locals.var_t0 = assign8250_e8216;
        locals.var_t0_dn3 = (((-locals.var_a0_dn3) * assign8250_e8215) + (assign8250_e8213 * (assign8250_e8215 * locals.var_phissat_dn3)));
        locals.var_t0_dn4 = (((-locals.var_a0_dn4) * assign8250_e8215) + (assign8250_e8213 * (assign8250_e8215 * locals.var_phissat_dn4)));
        locals.var_t0_dn5 = (((-locals.var_a0_dn5) * assign8250_e8215) + (assign8250_e8213 * (assign8250_e8215 * locals.var_phissat_dn5)));
        locals.var_t0_dn6 = (((-locals.var_a0_dn6) * assign8250_e8215) + (assign8250_e8213 * (assign8250_e8215 * locals.var_phissat_dn6)));
        locals.var_t0_dn7 = (((-locals.var_a0_dn7) * assign8250_e8215) + (assign8250_e8213 * (assign8250_e8215 * locals.var_phissat_dn7)));
        locals.var_t0_dn8 = (((-locals.var_a0_dn8) * assign8250_e8215) + (assign8250_e8213 * (assign8250_e8215 * locals.var_phissat_dn8)));
        locals.var_t0_rv = 0.0;

        let assign8260_e8219: f64 = (locals.var_k1_2 * locals.var_q1);
        locals.var_t1 = assign8260_e8219;
        locals.var_t1_dn3 = (locals.var_k1_2 * locals.var_q1_dn3);
        locals.var_t1_dn4 = (locals.var_k1_2 * locals.var_q1_dn4);
        locals.var_t1_dn5 = (locals.var_k1_2 * locals.var_q1_dn5);
        locals.var_t1_dn6 = (locals.var_k1_2 * locals.var_q1_dn6);
        locals.var_t1_dn7 = (locals.var_k1_2 * locals.var_q1_dn7);
        locals.var_t1_dn8 = (locals.var_k1_2 * locals.var_q1_dn8);
        locals.var_t1_rv = 0.0;

        let assign8270_e8222: f64 = (locals.var_t1 * locals.var_q1);
        let assign8270_e8224: f64 = (assign8270_e8222 + locals.var_t0);
        let assign8270_e8226: f64 = (assign8270_e8224 - locals.var_qsqrt);
        let assign8270_e8227: f64 = (-assign8270_e8226);
        let assign8270_e8229: f64 = (-2.0);
        let assign8270_e8231: f64 = (assign8270_e8229 * locals.var_t1);
        let assign8270_e8233: f64 = (assign8270_e8231 + locals.var_t0);
        let assign8270_e8234: f64 = (assign8270_e8227 / assign8270_e8233);
        locals.var_delta = assign8270_e8234;
        locals.var_delta_dn3 = ((((-((((locals.var_t1_dn3 * locals.var_q1) + (locals.var_t1 * locals.var_q1_dn3)) + locals.var_t0_dn3) - locals.var_qsqrt_dn3)) * assign8270_e8233) - (assign8270_e8227 * ((assign8270_e8229 * locals.var_t1_dn3) + locals.var_t0_dn3))) / (assign8270_e8233 * assign8270_e8233));
        locals.var_delta_dn4 = ((((-((((locals.var_t1_dn4 * locals.var_q1) + (locals.var_t1 * locals.var_q1_dn4)) + locals.var_t0_dn4) - locals.var_qsqrt_dn4)) * assign8270_e8233) - (assign8270_e8227 * ((assign8270_e8229 * locals.var_t1_dn4) + locals.var_t0_dn4))) / (assign8270_e8233 * assign8270_e8233));
        locals.var_delta_dn5 = ((((-((((locals.var_t1_dn5 * locals.var_q1) + (locals.var_t1 * locals.var_q1_dn5)) + locals.var_t0_dn5) - locals.var_qsqrt_dn5)) * assign8270_e8233) - (assign8270_e8227 * ((assign8270_e8229 * locals.var_t1_dn5) + locals.var_t0_dn5))) / (assign8270_e8233 * assign8270_e8233));
        locals.var_delta_dn6 = ((((-((((locals.var_t1_dn6 * locals.var_q1) + (locals.var_t1 * locals.var_q1_dn6)) + locals.var_t0_dn6) - locals.var_qsqrt_dn6)) * assign8270_e8233) - (assign8270_e8227 * ((assign8270_e8229 * locals.var_t1_dn6) + locals.var_t0_dn6))) / (assign8270_e8233 * assign8270_e8233));
        locals.var_delta_dn7 = ((((-((((locals.var_t1_dn7 * locals.var_q1) + (locals.var_t1 * locals.var_q1_dn7)) + locals.var_t0_dn7) - locals.var_qsqrt_dn7)) * assign8270_e8233) - (assign8270_e8227 * ((assign8270_e8229 * locals.var_t1_dn7) + locals.var_t0_dn7))) / (assign8270_e8233 * assign8270_e8233));
        locals.var_delta_dn8 = ((((-((((locals.var_t1_dn8 * locals.var_q1) + (locals.var_t1 * locals.var_q1_dn8)) + locals.var_t0_dn8) - locals.var_qsqrt_dn8)) * assign8270_e8233) - (assign8270_e8227 * ((assign8270_e8229 * locals.var_t1_dn8) + locals.var_t0_dn8))) / (assign8270_e8233 * assign8270_e8233));
        locals.var_delta_rv = 0.0;

        let assign8280_e8237: f64 = (locals.var_phissat + locals.var_delta);
        locals.var_phissat = assign8280_e8237;
        locals.var_phissat_dn3 = (locals.var_phissat_dn3 + locals.var_delta_dn3);
        locals.var_phissat_dn4 = (locals.var_phissat_dn4 + locals.var_delta_dn4);
        locals.var_phissat_dn5 = (locals.var_phissat_dn5 + locals.var_delta_dn5);
        locals.var_phissat_dn6 = (locals.var_phissat_dn6 + locals.var_delta_dn6);
        locals.var_phissat_dn7 = (locals.var_phissat_dn7 + locals.var_delta_dn7);
        locals.var_phissat_dn8 = (locals.var_phissat_dn8 + locals.var_delta_dn8);
        locals.var_phissat_rv = 0.0;

        let assign8290_e8240: f64 = (locals.var_xg1 - locals.var_phissat);
        locals.var_q1 = assign8290_e8240;
        locals.var_q1_dn3 = (locals.var_xg1_dn3 - locals.var_phissat_dn3);
        locals.var_q1_dn4 = (locals.var_xg1_dn4 - locals.var_phissat_dn4);
        locals.var_q1_dn5 = (locals.var_xg1_dn5 - locals.var_phissat_dn5);
        locals.var_q1_dn6 = (locals.var_xg1_dn6 - locals.var_phissat_dn6);
        locals.var_q1_dn7 = (locals.var_xg1_dn7 - locals.var_phissat_dn7);
        locals.var_q1_dn8 = (locals.var_xg1_dn8 - locals.var_phissat_dn8);
        locals.var_q1_rv = 0.0;

        let assign8300_e8243: f64 = (locals.var_k1_2 * locals.var_q1);
        locals.var_t2 = assign8300_e8243;
        locals.var_t2_dn3 = (locals.var_k1_2 * locals.var_q1_dn3);
        locals.var_t2_dn4 = (locals.var_k1_2 * locals.var_q1_dn4);
        locals.var_t2_dn5 = (locals.var_k1_2 * locals.var_q1_dn5);
        locals.var_t2_dn6 = (locals.var_k1_2 * locals.var_q1_dn6);
        locals.var_t2_dn7 = (locals.var_k1_2 * locals.var_q1_dn7);
        locals.var_t2_dn8 = (locals.var_k1_2 * locals.var_q1_dn8);
        locals.var_t2_rv = 0.0;

        let assign8310_e8247: f64 = (locals.var_t2 * locals.var_q1);
        let assign8310_e8249: f64 = (assign8310_e8247 - locals.var_qsqrt);
        let assign8310_e8250: f64 = (1.0 / assign8310_e8249);
        locals.var_t0 = assign8310_e8250;
        locals.var_t0_dn3 = (-((((locals.var_t2_dn3 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn3)) - locals.var_qsqrt_dn3) / (assign8310_e8249 * assign8310_e8249)));
        locals.var_t0_dn4 = (-((((locals.var_t2_dn4 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn4)) - locals.var_qsqrt_dn4) / (assign8310_e8249 * assign8310_e8249)));
        locals.var_t0_dn5 = (-((((locals.var_t2_dn5 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn5)) - locals.var_qsqrt_dn5) / (assign8310_e8249 * assign8310_e8249)));
        locals.var_t0_dn6 = (-((((locals.var_t2_dn6 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn6)) - locals.var_qsqrt_dn6) / (assign8310_e8249 * assign8310_e8249)));
        locals.var_t0_dn7 = (-((((locals.var_t2_dn7 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn7)) - locals.var_qsqrt_dn7) / (assign8310_e8249 * assign8310_e8249)));
        locals.var_t0_dn8 = (-((((locals.var_t2_dn8 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn8)) - locals.var_qsqrt_dn8) / (assign8310_e8249 * assign8310_e8249)));
        locals.var_t0_rv = 0.0;

        let assign8320_e8253: f64 = (locals.var_t2 * locals.var_q1);
        let assign8320_e8255: f64 = (assign8320_e8253 - locals.var_qsqrt);
        let assign8320_e8256: f64 = (assign8320_e8255).abs();
        let assign8320_e8257: f64 = (assign8320_e8256).ln();
        let assign8320_e8259: f64 = (assign8320_e8257 - locals.var_lna0);
        let assign8320_e8261: f64 = (assign8320_e8259 - locals.var_phissat);
        locals.var_g = assign8320_e8261;
        locals.var_g_dn3 = (((if assign8320_e8255 >= 0.0 { (((locals.var_t2_dn3 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn3)) - locals.var_qsqrt_dn3) } else { (-(((locals.var_t2_dn3 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn3)) - locals.var_qsqrt_dn3)) } / assign8320_e8256) - locals.var_lna0_dn3) - locals.var_phissat_dn3);
        locals.var_g_dn4 = (((if assign8320_e8255 >= 0.0 { (((locals.var_t2_dn4 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn4)) - locals.var_qsqrt_dn4) } else { (-(((locals.var_t2_dn4 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn4)) - locals.var_qsqrt_dn4)) } / assign8320_e8256) - locals.var_lna0_dn4) - locals.var_phissat_dn4);
        locals.var_g_dn5 = (((if assign8320_e8255 >= 0.0 { (((locals.var_t2_dn5 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn5)) - locals.var_qsqrt_dn5) } else { (-(((locals.var_t2_dn5 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn5)) - locals.var_qsqrt_dn5)) } / assign8320_e8256) - locals.var_lna0_dn5) - locals.var_phissat_dn5);
        locals.var_g_dn6 = (((if assign8320_e8255 >= 0.0 { (((locals.var_t2_dn6 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn6)) - locals.var_qsqrt_dn6) } else { (-(((locals.var_t2_dn6 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn6)) - locals.var_qsqrt_dn6)) } / assign8320_e8256) - locals.var_lna0_dn6) - locals.var_phissat_dn6);
        locals.var_g_dn7 = (((if assign8320_e8255 >= 0.0 { (((locals.var_t2_dn7 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn7)) - locals.var_qsqrt_dn7) } else { (-(((locals.var_t2_dn7 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn7)) - locals.var_qsqrt_dn7)) } / assign8320_e8256) - locals.var_lna0_dn7) - locals.var_phissat_dn7);
        locals.var_g_dn8 = (((if assign8320_e8255 >= 0.0 { (((locals.var_t2_dn8 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn8)) - locals.var_qsqrt_dn8) } else { (-(((locals.var_t2_dn8 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn8)) - locals.var_qsqrt_dn8)) } / assign8320_e8256) - locals.var_lna0_dn8) - locals.var_phissat_dn8);
        locals.var_g_rv = 0.0;

        let assign8330_e8264: f64 = (-2.0);
        let assign8330_e8266: f64 = (assign8330_e8264 * locals.var_t2);
        let assign8330_e8268: f64 = (assign8330_e8266 * locals.var_t0);
        let assign8330_e8270: f64 = (assign8330_e8268 - 1.0);
        let assign8330_e8271: f64 = (1.0 / assign8330_e8270);
        locals.var_dg1 = assign8330_e8271;
        locals.var_dg1_dn3 = (-((((assign8330_e8264 * locals.var_t2_dn3) * locals.var_t0) + (assign8330_e8266 * locals.var_t0_dn3)) / (assign8330_e8270 * assign8330_e8270)));
        locals.var_dg1_dn4 = (-((((assign8330_e8264 * locals.var_t2_dn4) * locals.var_t0) + (assign8330_e8266 * locals.var_t0_dn4)) / (assign8330_e8270 * assign8330_e8270)));
        locals.var_dg1_dn5 = (-((((assign8330_e8264 * locals.var_t2_dn5) * locals.var_t0) + (assign8330_e8266 * locals.var_t0_dn5)) / (assign8330_e8270 * assign8330_e8270)));
        locals.var_dg1_dn6 = (-((((assign8330_e8264 * locals.var_t2_dn6) * locals.var_t0) + (assign8330_e8266 * locals.var_t0_dn6)) / (assign8330_e8270 * assign8330_e8270)));
        locals.var_dg1_dn7 = (-((((assign8330_e8264 * locals.var_t2_dn7) * locals.var_t0) + (assign8330_e8266 * locals.var_t0_dn7)) / (assign8330_e8270 * assign8330_e8270)));
        locals.var_dg1_dn8 = (-((((assign8330_e8264 * locals.var_t2_dn8) * locals.var_t0) + (assign8330_e8266 * locals.var_t0_dn8)) / (assign8330_e8270 * assign8330_e8270)));
        locals.var_dg1_rv = 0.0;

        let assign8340_e8273: f64 = (-4.0);
        let assign8340_e8275: f64 = (assign8340_e8273 * locals.var_t2);
        let assign8340_e8277: f64 = (assign8340_e8275 * locals.var_t2);
        let assign8340_e8279: f64 = (assign8340_e8277 * locals.var_t0);
        let assign8340_e8281: f64 = (assign8340_e8279 * locals.var_t0);
        let assign8340_e8284: f64 = (2.0 * locals.var_k1_2);
        let assign8340_e8286: f64 = (assign8340_e8284 * locals.var_t0);
        let assign8340_e8287: f64 = (assign8340_e8281 + assign8340_e8286);
        locals.var_dg2 = assign8340_e8287;
        locals.var_dg2_dn3 = ((((((((assign8340_e8273 * locals.var_t2_dn3) * locals.var_t2) + (assign8340_e8275 * locals.var_t2_dn3)) * locals.var_t0) + (assign8340_e8277 * locals.var_t0_dn3)) * locals.var_t0) + (assign8340_e8279 * locals.var_t0_dn3)) + (assign8340_e8284 * locals.var_t0_dn3));
        locals.var_dg2_dn4 = ((((((((assign8340_e8273 * locals.var_t2_dn4) * locals.var_t2) + (assign8340_e8275 * locals.var_t2_dn4)) * locals.var_t0) + (assign8340_e8277 * locals.var_t0_dn4)) * locals.var_t0) + (assign8340_e8279 * locals.var_t0_dn4)) + (assign8340_e8284 * locals.var_t0_dn4));
        locals.var_dg2_dn5 = ((((((((assign8340_e8273 * locals.var_t2_dn5) * locals.var_t2) + (assign8340_e8275 * locals.var_t2_dn5)) * locals.var_t0) + (assign8340_e8277 * locals.var_t0_dn5)) * locals.var_t0) + (assign8340_e8279 * locals.var_t0_dn5)) + (assign8340_e8284 * locals.var_t0_dn5));
        locals.var_dg2_dn6 = ((((((((assign8340_e8273 * locals.var_t2_dn6) * locals.var_t2) + (assign8340_e8275 * locals.var_t2_dn6)) * locals.var_t0) + (assign8340_e8277 * locals.var_t0_dn6)) * locals.var_t0) + (assign8340_e8279 * locals.var_t0_dn6)) + (assign8340_e8284 * locals.var_t0_dn6));
        locals.var_dg2_dn7 = ((((((((assign8340_e8273 * locals.var_t2_dn7) * locals.var_t2) + (assign8340_e8275 * locals.var_t2_dn7)) * locals.var_t0) + (assign8340_e8277 * locals.var_t0_dn7)) * locals.var_t0) + (assign8340_e8279 * locals.var_t0_dn7)) + (assign8340_e8284 * locals.var_t0_dn7));
        locals.var_dg2_dn8 = ((((((((assign8340_e8273 * locals.var_t2_dn8) * locals.var_t2) + (assign8340_e8275 * locals.var_t2_dn8)) * locals.var_t0) + (assign8340_e8277 * locals.var_t0_dn8)) * locals.var_t0) + (assign8340_e8279 * locals.var_t0_dn8)) + (assign8340_e8284 * locals.var_t0_dn8));
        locals.var_dg2_rv = 0.0;

        let assign8350_e8290: f64 = (locals.var_g * locals.var_dg1);
        locals.var_t1 = assign8350_e8290;
        locals.var_t1_dn3 = ((locals.var_g_dn3 * locals.var_dg1) + (locals.var_g * locals.var_dg1_dn3));
        locals.var_t1_dn4 = ((locals.var_g_dn4 * locals.var_dg1) + (locals.var_g * locals.var_dg1_dn4));
        locals.var_t1_dn5 = ((locals.var_g_dn5 * locals.var_dg1) + (locals.var_g * locals.var_dg1_dn5));
        locals.var_t1_dn6 = ((locals.var_g_dn6 * locals.var_dg1) + (locals.var_g * locals.var_dg1_dn6));
        locals.var_t1_dn7 = ((locals.var_g_dn7 * locals.var_dg1) + (locals.var_g * locals.var_dg1_dn7));
        locals.var_t1_dn8 = ((locals.var_g_dn8 * locals.var_dg1) + (locals.var_g * locals.var_dg1_dn8));
        locals.var_t1_rv = 0.0;

        let assign8360_e8292: f64 = (-locals.var_t1);
        let assign8360_e8295: f64 = (0.5 * locals.var_t1);
        let assign8360_e8297: f64 = (assign8360_e8295 * locals.var_t1);
        let assign8360_e8299: f64 = (assign8360_e8297 * locals.var_dg2);
        let assign8360_e8301: f64 = (assign8360_e8299 * locals.var_dg1);
        let assign8360_e8302: f64 = (assign8360_e8292 - assign8360_e8301);
        locals.var_delta = assign8360_e8302;
        locals.var_delta_dn3 = ((-locals.var_t1_dn3) - (((((((0.5 * locals.var_t1_dn3) * locals.var_t1) + (assign8360_e8295 * locals.var_t1_dn3)) * locals.var_dg2) + (assign8360_e8297 * locals.var_dg2_dn3)) * locals.var_dg1) + (assign8360_e8299 * locals.var_dg1_dn3)));
        locals.var_delta_dn4 = ((-locals.var_t1_dn4) - (((((((0.5 * locals.var_t1_dn4) * locals.var_t1) + (assign8360_e8295 * locals.var_t1_dn4)) * locals.var_dg2) + (assign8360_e8297 * locals.var_dg2_dn4)) * locals.var_dg1) + (assign8360_e8299 * locals.var_dg1_dn4)));
        locals.var_delta_dn5 = ((-locals.var_t1_dn5) - (((((((0.5 * locals.var_t1_dn5) * locals.var_t1) + (assign8360_e8295 * locals.var_t1_dn5)) * locals.var_dg2) + (assign8360_e8297 * locals.var_dg2_dn5)) * locals.var_dg1) + (assign8360_e8299 * locals.var_dg1_dn5)));
        locals.var_delta_dn6 = ((-locals.var_t1_dn6) - (((((((0.5 * locals.var_t1_dn6) * locals.var_t1) + (assign8360_e8295 * locals.var_t1_dn6)) * locals.var_dg2) + (assign8360_e8297 * locals.var_dg2_dn6)) * locals.var_dg1) + (assign8360_e8299 * locals.var_dg1_dn6)));
        locals.var_delta_dn7 = ((-locals.var_t1_dn7) - (((((((0.5 * locals.var_t1_dn7) * locals.var_t1) + (assign8360_e8295 * locals.var_t1_dn7)) * locals.var_dg2) + (assign8360_e8297 * locals.var_dg2_dn7)) * locals.var_dg1) + (assign8360_e8299 * locals.var_dg1_dn7)));
        locals.var_delta_dn8 = ((-locals.var_t1_dn8) - (((((((0.5 * locals.var_t1_dn8) * locals.var_t1) + (assign8360_e8295 * locals.var_t1_dn8)) * locals.var_dg2) + (assign8360_e8297 * locals.var_dg2_dn8)) * locals.var_dg1) + (assign8360_e8299 * locals.var_dg1_dn8)));
        locals.var_delta_rv = 0.0;

        let assign8370_e8305: f64 = (-10.0);
        let assign8370_e8306: f64 = (locals.var_delta).max(assign8370_e8305);
        locals.var_delta = assign8370_e8306;
        locals.var_delta_dn3 = if locals.var_delta >= assign8370_e8305 { locals.var_delta_dn3 } else { 0.0 };
        locals.var_delta_dn4 = if locals.var_delta >= assign8370_e8305 { locals.var_delta_dn4 } else { 0.0 };
        locals.var_delta_dn5 = if locals.var_delta >= assign8370_e8305 { locals.var_delta_dn5 } else { 0.0 };
        locals.var_delta_dn6 = if locals.var_delta >= assign8370_e8305 { locals.var_delta_dn6 } else { 0.0 };
        locals.var_delta_dn7 = if locals.var_delta >= assign8370_e8305 { locals.var_delta_dn7 } else { 0.0 };
        locals.var_delta_dn8 = if locals.var_delta >= assign8370_e8305 { locals.var_delta_dn8 } else { 0.0 };
        locals.var_delta_rv = 0.0;

        let assign8380_e8309: f64 = (locals.var_delta).min(10.0);
        locals.var_delta = assign8380_e8309;
        locals.var_delta_dn3 = if locals.var_delta <= 10.0 { locals.var_delta_dn3 } else { 0.0 };
        locals.var_delta_dn4 = if locals.var_delta <= 10.0 { locals.var_delta_dn4 } else { 0.0 };
        locals.var_delta_dn5 = if locals.var_delta <= 10.0 { locals.var_delta_dn5 } else { 0.0 };
        locals.var_delta_dn6 = if locals.var_delta <= 10.0 { locals.var_delta_dn6 } else { 0.0 };
        locals.var_delta_dn7 = if locals.var_delta <= 10.0 { locals.var_delta_dn7 } else { 0.0 };
        locals.var_delta_dn8 = if locals.var_delta <= 10.0 { locals.var_delta_dn8 } else { 0.0 };
        locals.var_delta_rv = 0.0;

        let assign8390_e8312: f64 = (locals.var_phissat + locals.var_delta);
        locals.var_phissat = assign8390_e8312;
        locals.var_phissat_dn3 = (locals.var_phissat_dn3 + locals.var_delta_dn3);
        locals.var_phissat_dn4 = (locals.var_phissat_dn4 + locals.var_delta_dn4);
        locals.var_phissat_dn5 = (locals.var_phissat_dn5 + locals.var_delta_dn5);
        locals.var_phissat_dn6 = (locals.var_phissat_dn6 + locals.var_delta_dn6);
        locals.var_phissat_dn7 = (locals.var_phissat_dn7 + locals.var_delta_dn7);
        locals.var_phissat_dn8 = (locals.var_phissat_dn8 + locals.var_delta_dn8);
        locals.var_phissat_rv = 0.0;

        let assign8400_e8315: f64 = (locals.var_xg1 - locals.var_phissat);
        locals.var_q1 = assign8400_e8315;
        locals.var_q1_dn3 = (locals.var_xg1_dn3 - locals.var_phissat_dn3);
        locals.var_q1_dn4 = (locals.var_xg1_dn4 - locals.var_phissat_dn4);
        locals.var_q1_dn5 = (locals.var_xg1_dn5 - locals.var_phissat_dn5);
        locals.var_q1_dn6 = (locals.var_xg1_dn6 - locals.var_phissat_dn6);
        locals.var_q1_dn7 = (locals.var_xg1_dn7 - locals.var_phissat_dn7);
        locals.var_q1_dn8 = (locals.var_xg1_dn8 - locals.var_phissat_dn8);
        locals.var_q1_rv = 0.0;

        let assign8410_e8318: f64 = (locals.var_k1_2 * locals.var_q1);
        locals.var_t2 = assign8410_e8318;
        locals.var_t2_dn3 = (locals.var_k1_2 * locals.var_q1_dn3);
        locals.var_t2_dn4 = (locals.var_k1_2 * locals.var_q1_dn4);
        locals.var_t2_dn5 = (locals.var_k1_2 * locals.var_q1_dn5);
        locals.var_t2_dn6 = (locals.var_k1_2 * locals.var_q1_dn6);
        locals.var_t2_dn7 = (locals.var_k1_2 * locals.var_q1_dn7);
        locals.var_t2_dn8 = (locals.var_k1_2 * locals.var_q1_dn8);
        locals.var_t2_rv = 0.0;

        let assign8420_e8322: f64 = (locals.var_t2 * locals.var_q1);
        let assign8420_e8324: f64 = (assign8420_e8322 - locals.var_qsqrt);
        let assign8420_e8325: f64 = (1.0 / assign8420_e8324);
        locals.var_t0 = assign8420_e8325;
        locals.var_t0_dn3 = (-((((locals.var_t2_dn3 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn3)) - locals.var_qsqrt_dn3) / (assign8420_e8324 * assign8420_e8324)));
        locals.var_t0_dn4 = (-((((locals.var_t2_dn4 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn4)) - locals.var_qsqrt_dn4) / (assign8420_e8324 * assign8420_e8324)));
        locals.var_t0_dn5 = (-((((locals.var_t2_dn5 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn5)) - locals.var_qsqrt_dn5) / (assign8420_e8324 * assign8420_e8324)));
        locals.var_t0_dn6 = (-((((locals.var_t2_dn6 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn6)) - locals.var_qsqrt_dn6) / (assign8420_e8324 * assign8420_e8324)));
        locals.var_t0_dn7 = (-((((locals.var_t2_dn7 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn7)) - locals.var_qsqrt_dn7) / (assign8420_e8324 * assign8420_e8324)));
        locals.var_t0_dn8 = (-((((locals.var_t2_dn8 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn8)) - locals.var_qsqrt_dn8) / (assign8420_e8324 * assign8420_e8324)));
        locals.var_t0_rv = 0.0;

        let assign8430_e8328: f64 = (locals.var_t2 * locals.var_q1);
        let assign8430_e8330: f64 = (assign8430_e8328 - locals.var_qsqrt);
        let assign8430_e8331: f64 = (assign8430_e8330).abs();
        let assign8430_e8332: f64 = (assign8430_e8331).ln();
        let assign8430_e8334: f64 = (assign8430_e8332 - locals.var_lna0);
        let assign8430_e8336: f64 = (assign8430_e8334 - locals.var_phissat);
        locals.var_g = assign8430_e8336;
        locals.var_g_dn3 = (((if assign8430_e8330 >= 0.0 { (((locals.var_t2_dn3 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn3)) - locals.var_qsqrt_dn3) } else { (-(((locals.var_t2_dn3 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn3)) - locals.var_qsqrt_dn3)) } / assign8430_e8331) - locals.var_lna0_dn3) - locals.var_phissat_dn3);
        locals.var_g_dn4 = (((if assign8430_e8330 >= 0.0 { (((locals.var_t2_dn4 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn4)) - locals.var_qsqrt_dn4) } else { (-(((locals.var_t2_dn4 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn4)) - locals.var_qsqrt_dn4)) } / assign8430_e8331) - locals.var_lna0_dn4) - locals.var_phissat_dn4);
        locals.var_g_dn5 = (((if assign8430_e8330 >= 0.0 { (((locals.var_t2_dn5 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn5)) - locals.var_qsqrt_dn5) } else { (-(((locals.var_t2_dn5 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn5)) - locals.var_qsqrt_dn5)) } / assign8430_e8331) - locals.var_lna0_dn5) - locals.var_phissat_dn5);
        locals.var_g_dn6 = (((if assign8430_e8330 >= 0.0 { (((locals.var_t2_dn6 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn6)) - locals.var_qsqrt_dn6) } else { (-(((locals.var_t2_dn6 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn6)) - locals.var_qsqrt_dn6)) } / assign8430_e8331) - locals.var_lna0_dn6) - locals.var_phissat_dn6);
        locals.var_g_dn7 = (((if assign8430_e8330 >= 0.0 { (((locals.var_t2_dn7 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn7)) - locals.var_qsqrt_dn7) } else { (-(((locals.var_t2_dn7 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn7)) - locals.var_qsqrt_dn7)) } / assign8430_e8331) - locals.var_lna0_dn7) - locals.var_phissat_dn7);
        locals.var_g_dn8 = (((if assign8430_e8330 >= 0.0 { (((locals.var_t2_dn8 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn8)) - locals.var_qsqrt_dn8) } else { (-(((locals.var_t2_dn8 * locals.var_q1) + (locals.var_t2 * locals.var_q1_dn8)) - locals.var_qsqrt_dn8)) } / assign8430_e8331) - locals.var_lna0_dn8) - locals.var_phissat_dn8);
        locals.var_g_rv = 0.0;

        let assign8440_e8339: f64 = (-2.0);
        let assign8440_e8341: f64 = (assign8440_e8339 * locals.var_t2);
        let assign8440_e8343: f64 = (assign8440_e8341 * locals.var_t0);
        let assign8440_e8345: f64 = (assign8440_e8343 - 1.0);
        let assign8440_e8346: f64 = (1.0 / assign8440_e8345);
        locals.var_dg1 = assign8440_e8346;
        locals.var_dg1_dn3 = (-((((assign8440_e8339 * locals.var_t2_dn3) * locals.var_t0) + (assign8440_e8341 * locals.var_t0_dn3)) / (assign8440_e8345 * assign8440_e8345)));
        locals.var_dg1_dn4 = (-((((assign8440_e8339 * locals.var_t2_dn4) * locals.var_t0) + (assign8440_e8341 * locals.var_t0_dn4)) / (assign8440_e8345 * assign8440_e8345)));
        locals.var_dg1_dn5 = (-((((assign8440_e8339 * locals.var_t2_dn5) * locals.var_t0) + (assign8440_e8341 * locals.var_t0_dn5)) / (assign8440_e8345 * assign8440_e8345)));
        locals.var_dg1_dn6 = (-((((assign8440_e8339 * locals.var_t2_dn6) * locals.var_t0) + (assign8440_e8341 * locals.var_t0_dn6)) / (assign8440_e8345 * assign8440_e8345)));
        locals.var_dg1_dn7 = (-((((assign8440_e8339 * locals.var_t2_dn7) * locals.var_t0) + (assign8440_e8341 * locals.var_t0_dn7)) / (assign8440_e8345 * assign8440_e8345)));
        locals.var_dg1_dn8 = (-((((assign8440_e8339 * locals.var_t2_dn8) * locals.var_t0) + (assign8440_e8341 * locals.var_t0_dn8)) / (assign8440_e8345 * assign8440_e8345)));
        locals.var_dg1_rv = 0.0;

        let assign8450_e8348: f64 = (-4.0);
        let assign8450_e8350: f64 = (assign8450_e8348 * locals.var_t2);
        let assign8450_e8352: f64 = (assign8450_e8350 * locals.var_t2);
        let assign8450_e8354: f64 = (assign8450_e8352 * locals.var_t0);
        let assign8450_e8356: f64 = (assign8450_e8354 * locals.var_t0);
        let assign8450_e8359: f64 = (2.0 * locals.var_k1_2);
        let assign8450_e8361: f64 = (assign8450_e8359 * locals.var_t0);
        let assign8450_e8362: f64 = (assign8450_e8356 + assign8450_e8361);
        locals.var_dg2 = assign8450_e8362;
        locals.var_dg2_dn3 = ((((((((assign8450_e8348 * locals.var_t2_dn3) * locals.var_t2) + (assign8450_e8350 * locals.var_t2_dn3)) * locals.var_t0) + (assign8450_e8352 * locals.var_t0_dn3)) * locals.var_t0) + (assign8450_e8354 * locals.var_t0_dn3)) + (assign8450_e8359 * locals.var_t0_dn3));
        locals.var_dg2_dn4 = ((((((((assign8450_e8348 * locals.var_t2_dn4) * locals.var_t2) + (assign8450_e8350 * locals.var_t2_dn4)) * locals.var_t0) + (assign8450_e8352 * locals.var_t0_dn4)) * locals.var_t0) + (assign8450_e8354 * locals.var_t0_dn4)) + (assign8450_e8359 * locals.var_t0_dn4));
        locals.var_dg2_dn5 = ((((((((assign8450_e8348 * locals.var_t2_dn5) * locals.var_t2) + (assign8450_e8350 * locals.var_t2_dn5)) * locals.var_t0) + (assign8450_e8352 * locals.var_t0_dn5)) * locals.var_t0) + (assign8450_e8354 * locals.var_t0_dn5)) + (assign8450_e8359 * locals.var_t0_dn5));
        locals.var_dg2_dn6 = ((((((((assign8450_e8348 * locals.var_t2_dn6) * locals.var_t2) + (assign8450_e8350 * locals.var_t2_dn6)) * locals.var_t0) + (assign8450_e8352 * locals.var_t0_dn6)) * locals.var_t0) + (assign8450_e8354 * locals.var_t0_dn6)) + (assign8450_e8359 * locals.var_t0_dn6));
        locals.var_dg2_dn7 = ((((((((assign8450_e8348 * locals.var_t2_dn7) * locals.var_t2) + (assign8450_e8350 * locals.var_t2_dn7)) * locals.var_t0) + (assign8450_e8352 * locals.var_t0_dn7)) * locals.var_t0) + (assign8450_e8354 * locals.var_t0_dn7)) + (assign8450_e8359 * locals.var_t0_dn7));
        locals.var_dg2_dn8 = ((((((((assign8450_e8348 * locals.var_t2_dn8) * locals.var_t2) + (assign8450_e8350 * locals.var_t2_dn8)) * locals.var_t0) + (assign8450_e8352 * locals.var_t0_dn8)) * locals.var_t0) + (assign8450_e8354 * locals.var_t0_dn8)) + (assign8450_e8359 * locals.var_t0_dn8));
        locals.var_dg2_rv = 0.0;

        let assign8460_e8365: f64 = (locals.var_g * locals.var_dg1);
        locals.var_t1 = assign8460_e8365;
        locals.var_t1_dn3 = ((locals.var_g_dn3 * locals.var_dg1) + (locals.var_g * locals.var_dg1_dn3));
        locals.var_t1_dn4 = ((locals.var_g_dn4 * locals.var_dg1) + (locals.var_g * locals.var_dg1_dn4));
        locals.var_t1_dn5 = ((locals.var_g_dn5 * locals.var_dg1) + (locals.var_g * locals.var_dg1_dn5));
        locals.var_t1_dn6 = ((locals.var_g_dn6 * locals.var_dg1) + (locals.var_g * locals.var_dg1_dn6));
        locals.var_t1_dn7 = ((locals.var_g_dn7 * locals.var_dg1) + (locals.var_g * locals.var_dg1_dn7));
        locals.var_t1_dn8 = ((locals.var_g_dn8 * locals.var_dg1) + (locals.var_g * locals.var_dg1_dn8));
        locals.var_t1_rv = 0.0;

        let assign8470_e8367: f64 = (-locals.var_t1);
        let assign8470_e8370: f64 = (0.5 * locals.var_t1);
        let assign8470_e8372: f64 = (assign8470_e8370 * locals.var_t1);
        let assign8470_e8374: f64 = (assign8470_e8372 * locals.var_dg2);
        let assign8470_e8376: f64 = (assign8470_e8374 * locals.var_dg1);
        let assign8470_e8377: f64 = (assign8470_e8367 - assign8470_e8376);
        locals.var_delta = assign8470_e8377;
        locals.var_delta_dn3 = ((-locals.var_t1_dn3) - (((((((0.5 * locals.var_t1_dn3) * locals.var_t1) + (assign8470_e8370 * locals.var_t1_dn3)) * locals.var_dg2) + (assign8470_e8372 * locals.var_dg2_dn3)) * locals.var_dg1) + (assign8470_e8374 * locals.var_dg1_dn3)));
        locals.var_delta_dn4 = ((-locals.var_t1_dn4) - (((((((0.5 * locals.var_t1_dn4) * locals.var_t1) + (assign8470_e8370 * locals.var_t1_dn4)) * locals.var_dg2) + (assign8470_e8372 * locals.var_dg2_dn4)) * locals.var_dg1) + (assign8470_e8374 * locals.var_dg1_dn4)));
        locals.var_delta_dn5 = ((-locals.var_t1_dn5) - (((((((0.5 * locals.var_t1_dn5) * locals.var_t1) + (assign8470_e8370 * locals.var_t1_dn5)) * locals.var_dg2) + (assign8470_e8372 * locals.var_dg2_dn5)) * locals.var_dg1) + (assign8470_e8374 * locals.var_dg1_dn5)));
        locals.var_delta_dn6 = ((-locals.var_t1_dn6) - (((((((0.5 * locals.var_t1_dn6) * locals.var_t1) + (assign8470_e8370 * locals.var_t1_dn6)) * locals.var_dg2) + (assign8470_e8372 * locals.var_dg2_dn6)) * locals.var_dg1) + (assign8470_e8374 * locals.var_dg1_dn6)));
        locals.var_delta_dn7 = ((-locals.var_t1_dn7) - (((((((0.5 * locals.var_t1_dn7) * locals.var_t1) + (assign8470_e8370 * locals.var_t1_dn7)) * locals.var_dg2) + (assign8470_e8372 * locals.var_dg2_dn7)) * locals.var_dg1) + (assign8470_e8374 * locals.var_dg1_dn7)));
        locals.var_delta_dn8 = ((-locals.var_t1_dn8) - (((((((0.5 * locals.var_t1_dn8) * locals.var_t1) + (assign8470_e8370 * locals.var_t1_dn8)) * locals.var_dg2) + (assign8470_e8372 * locals.var_dg2_dn8)) * locals.var_dg1) + (assign8470_e8374 * locals.var_dg1_dn8)));
        locals.var_delta_rv = 0.0;

        let assign8480_e8380: f64 = (-10.0);
        let assign8480_e8381: f64 = (locals.var_delta).max(assign8480_e8380);
        locals.var_delta = assign8480_e8381;
        locals.var_delta_dn3 = if locals.var_delta >= assign8480_e8380 { locals.var_delta_dn3 } else { 0.0 };
        locals.var_delta_dn4 = if locals.var_delta >= assign8480_e8380 { locals.var_delta_dn4 } else { 0.0 };
        locals.var_delta_dn5 = if locals.var_delta >= assign8480_e8380 { locals.var_delta_dn5 } else { 0.0 };
        locals.var_delta_dn6 = if locals.var_delta >= assign8480_e8380 { locals.var_delta_dn6 } else { 0.0 };
        locals.var_delta_dn7 = if locals.var_delta >= assign8480_e8380 { locals.var_delta_dn7 } else { 0.0 };
        locals.var_delta_dn8 = if locals.var_delta >= assign8480_e8380 { locals.var_delta_dn8 } else { 0.0 };
        locals.var_delta_rv = 0.0;

        let assign8490_e8384: f64 = (locals.var_delta).min(10.0);
        locals.var_delta = assign8490_e8384;
        locals.var_delta_dn3 = if locals.var_delta <= 10.0 { locals.var_delta_dn3 } else { 0.0 };
        locals.var_delta_dn4 = if locals.var_delta <= 10.0 { locals.var_delta_dn4 } else { 0.0 };
        locals.var_delta_dn5 = if locals.var_delta <= 10.0 { locals.var_delta_dn5 } else { 0.0 };
        locals.var_delta_dn6 = if locals.var_delta <= 10.0 { locals.var_delta_dn6 } else { 0.0 };
        locals.var_delta_dn7 = if locals.var_delta <= 10.0 { locals.var_delta_dn7 } else { 0.0 };
        locals.var_delta_dn8 = if locals.var_delta <= 10.0 { locals.var_delta_dn8 } else { 0.0 };
        locals.var_delta_rv = 0.0;

        let assign8500_e8387: f64 = (locals.var_phissat + locals.var_delta);
        locals.var_phissat = assign8500_e8387;
        locals.var_phissat_dn3 = (locals.var_phissat_dn3 + locals.var_delta_dn3);
        locals.var_phissat_dn4 = (locals.var_phissat_dn4 + locals.var_delta_dn4);
        locals.var_phissat_dn5 = (locals.var_phissat_dn5 + locals.var_delta_dn5);
        locals.var_phissat_dn6 = (locals.var_phissat_dn6 + locals.var_delta_dn6);
        locals.var_phissat_dn7 = (locals.var_phissat_dn7 + locals.var_delta_dn7);
        locals.var_phissat_dn8 = (locals.var_phissat_dn8 + locals.var_delta_dn8);
        locals.var_phissat_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_18(
        locals: &mut StampLocals,
    ) {
        let assign8510_e8391: f64 = (locals.var_phi1_0 - 4.0);
        let assign8510_e8392: f64 = (locals.var_phissat).max(assign8510_e8391);
        locals.var_phissat = assign8510_e8392;
        locals.var_phissat_dn3 = if locals.var_phissat >= assign8510_e8391 { locals.var_phissat_dn3 } else { locals.var_phi1_0_dn3 };
        locals.var_phissat_dn4 = if locals.var_phissat >= assign8510_e8391 { locals.var_phissat_dn4 } else { locals.var_phi1_0_dn4 };
        locals.var_phissat_dn5 = if locals.var_phissat >= assign8510_e8391 { locals.var_phissat_dn5 } else { locals.var_phi1_0_dn5 };
        locals.var_phissat_dn6 = if locals.var_phissat >= assign8510_e8391 { locals.var_phissat_dn6 } else { locals.var_phi1_0_dn6 };
        locals.var_phissat_dn7 = if locals.var_phissat >= assign8510_e8391 { locals.var_phissat_dn7 } else { locals.var_phi1_0_dn7 };
        locals.var_phissat_dn8 = if locals.var_phissat >= assign8510_e8391 { locals.var_phissat_dn8 } else { locals.var_phi1_0_dn8 };
        locals.var_phissat_rv = 0.0;

        let assign8520_e8395: f64 = (locals.var_vgfb1eff - locals.var_vdseff);
        let assign8520_e8397: f64 = (assign8520_e8395 / locals.var_nvtm);
        locals.var_xg1 = assign8520_e8397;
        locals.var_xg1_dn3 = ((((locals.var_vgfb1eff_dn3 - locals.var_vdseff_dn3) * locals.var_nvtm) - (assign8520_e8395 * locals.var_nvtm_dn3)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_xg1_dn4 = ((((locals.var_vgfb1eff_dn4 - locals.var_vdseff_dn4) * locals.var_nvtm) - (assign8520_e8395 * locals.var_nvtm_dn4)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_xg1_dn5 = ((((locals.var_vgfb1eff_dn5 - locals.var_vdseff_dn5) * locals.var_nvtm) - (assign8520_e8395 * locals.var_nvtm_dn5)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_xg1_dn6 = ((((locals.var_vgfb1eff_dn6 - locals.var_vdseff_dn6) * locals.var_nvtm) - (assign8520_e8395 * locals.var_nvtm_dn6)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_xg1_dn7 = ((((locals.var_vgfb1eff_dn7 - locals.var_vdseff_dn7) * locals.var_nvtm) - (assign8520_e8395 * locals.var_nvtm_dn7)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_xg1_dn8 = ((((locals.var_vgfb1eff_dn8 - locals.var_vdseff_dn8) * locals.var_nvtm) - (assign8520_e8395 * locals.var_nvtm_dn8)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_xg1_rv = 0.0;

        let assign8530_e8404: f64 = (1.05 * locals.var_phissat);
        let assign8530_e8405: f64 = (locals.var_phi1 - assign8530_e8404);
        let assign8530_e8407: f64 = assign8530_e8405;
        let assign8530_e8408: f64 = (assign8530_e8407).exp();
        let assign8530_e8409: f64 = (1.0 + assign8530_e8408);
        let assign8530_e8410: f64 = (assign8530_e8409).ln();
        let assign8530_e8411: f64 = assign8530_e8410;
        let assign8530_e8412: f64 = (locals.var_phi1 - assign8530_e8411);
        locals.var_phi1 = assign8530_e8412;
        locals.var_phi1_dn3 = (locals.var_phi1_dn3 - ((assign8530_e8408 * (locals.var_phi1_dn3 - (1.05 * locals.var_phissat_dn3))) / assign8530_e8409));
        locals.var_phi1_dn4 = (locals.var_phi1_dn4 - ((assign8530_e8408 * (locals.var_phi1_dn4 - (1.05 * locals.var_phissat_dn4))) / assign8530_e8409));
        locals.var_phi1_dn5 = (locals.var_phi1_dn5 - ((assign8530_e8408 * (locals.var_phi1_dn5 - (1.05 * locals.var_phissat_dn5))) / assign8530_e8409));
        locals.var_phi1_dn6 = (locals.var_phi1_dn6 - ((assign8530_e8408 * (locals.var_phi1_dn6 - (1.05 * locals.var_phissat_dn6))) / assign8530_e8409));
        locals.var_phi1_dn7 = (locals.var_phi1_dn7 - ((assign8530_e8408 * (locals.var_phi1_dn7 - (1.05 * locals.var_phissat_dn7))) / assign8530_e8409));
        locals.var_phi1_dn8 = (locals.var_phi1_dn8 - ((assign8530_e8408 * (locals.var_phi1_dn8 - (1.05 * locals.var_phissat_dn8))) / assign8530_e8409));
        locals.var_phi1_rv = 0.0;

        let assign8540_e8415: f64 = (locals.var_phi1).min(locals.var_phissat);
        locals.var_phi1 = assign8540_e8415;
        locals.var_phi1_dn3 = if locals.var_phi1 <= locals.var_phissat { locals.var_phi1_dn3 } else { locals.var_phissat_dn3 };
        locals.var_phi1_dn4 = if locals.var_phi1 <= locals.var_phissat { locals.var_phi1_dn4 } else { locals.var_phissat_dn4 };
        locals.var_phi1_dn5 = if locals.var_phi1 <= locals.var_phissat { locals.var_phi1_dn5 } else { locals.var_phissat_dn5 };
        locals.var_phi1_dn6 = if locals.var_phi1 <= locals.var_phissat { locals.var_phi1_dn6 } else { locals.var_phissat_dn6 };
        locals.var_phi1_dn7 = if locals.var_phi1 <= locals.var_phissat { locals.var_phi1_dn7 } else { locals.var_phissat_dn7 };
        locals.var_phi1_dn8 = if locals.var_phi1 <= locals.var_phissat { locals.var_phi1_dn8 } else { locals.var_phissat_dn8 };
        locals.var_phi1_rv = 0.0;

        let assign8550_e8418: f64 = (locals.var_xg1 - locals.var_phi1);
        locals.var_q1 = assign8550_e8418;
        locals.var_q1_dn3 = (locals.var_xg1_dn3 - locals.var_phi1_dn3);
        locals.var_q1_dn4 = (locals.var_xg1_dn4 - locals.var_phi1_dn4);
        locals.var_q1_dn5 = (locals.var_xg1_dn5 - locals.var_phi1_dn5);
        locals.var_q1_dn6 = (locals.var_xg1_dn6 - locals.var_phi1_dn6);
        locals.var_q1_dn7 = (locals.var_xg1_dn7 - locals.var_phi1_dn7);
        locals.var_q1_dn8 = (locals.var_xg1_dn8 - locals.var_phi1_dn8);
        locals.var_q1_rv = 0.0;

        let assign8560_e8421: f64 = (locals.var_k1 * locals.var_q1);
        locals.var_auxb1 = assign8560_e8421;
        locals.var_auxb1_dn3 = (locals.var_k1 * locals.var_q1_dn3);
        locals.var_auxb1_dn4 = (locals.var_k1 * locals.var_q1_dn4);
        locals.var_auxb1_dn5 = (locals.var_k1 * locals.var_q1_dn5);
        locals.var_auxb1_dn6 = (locals.var_k1 * locals.var_q1_dn6);
        locals.var_auxb1_dn7 = (locals.var_k1 * locals.var_q1_dn7);
        locals.var_auxb1_dn8 = (locals.var_k1 * locals.var_q1_dn8);
        locals.var_auxb1_rv = 0.0;

        let assign8570_e8423: f64 = (-locals.var_a0);
        let assign8570_e8425: f64 = (locals.var_phi1).exp();
        let assign8570_e8426: f64 = (assign8570_e8423 * assign8570_e8425);
        locals.var_aaux = assign8570_e8426;
        locals.var_aaux_dn3 = (((-locals.var_a0_dn3) * assign8570_e8425) + (assign8570_e8423 * (assign8570_e8425 * locals.var_phi1_dn3)));
        locals.var_aaux_dn4 = (((-locals.var_a0_dn4) * assign8570_e8425) + (assign8570_e8423 * (assign8570_e8425 * locals.var_phi1_dn4)));
        locals.var_aaux_dn5 = (((-locals.var_a0_dn5) * assign8570_e8425) + (assign8570_e8423 * (assign8570_e8425 * locals.var_phi1_dn5)));
        locals.var_aaux_dn6 = (((-locals.var_a0_dn6) * assign8570_e8425) + (assign8570_e8423 * (assign8570_e8425 * locals.var_phi1_dn6)));
        locals.var_aaux_dn7 = (((-locals.var_a0_dn7) * assign8570_e8425) + (assign8570_e8423 * (assign8570_e8425 * locals.var_phi1_dn7)));
        locals.var_aaux_dn8 = (((-locals.var_a0_dn8) * assign8570_e8425) + (assign8570_e8423 * (assign8570_e8425 * locals.var_phi1_dn8)));
        locals.var_aaux_rv = 0.0;

        let assign8580_e8429: f64 = (locals.var_auxb1 * locals.var_auxb1);
        let assign8580_e8431: f64 = (assign8580_e8429 + locals.var_aaux);
        locals.var_qsqrt = assign8580_e8431;
        locals.var_qsqrt_dn3 = (((locals.var_auxb1_dn3 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn3)) + locals.var_aaux_dn3);
        locals.var_qsqrt_dn4 = (((locals.var_auxb1_dn4 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn4)) + locals.var_aaux_dn4);
        locals.var_qsqrt_dn5 = (((locals.var_auxb1_dn5 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn5)) + locals.var_aaux_dn5);
        locals.var_qsqrt_dn6 = (((locals.var_auxb1_dn6 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn6)) + locals.var_aaux_dn6);
        locals.var_qsqrt_dn7 = (((locals.var_auxb1_dn7 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn7)) + locals.var_aaux_dn7);
        locals.var_qsqrt_dn8 = (((locals.var_auxb1_dn8 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn8)) + locals.var_aaux_dn8);
        locals.var_qsqrt_rv = 0.0;

        let assign8590_e8434: f64 = if locals.var_qsqrt < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard92 = assign8590_e8434;
        locals.var_guard92_rv = 0.0;

        let (assign8600_e8440, assign8600_e8440_d_n3, assign8600_e8440_d_n4, assign8600_e8440_d_n5, assign8600_e8440_d_n6, assign8600_e8440_d_n7, assign8600_e8440_d_n8,) = {
    if (locals.var_guard92 != 0.0) {
        let assign8600_e8437: f64 = (-locals.var_qsqrt);
        let assign8600_e8438: f64 = (assign8600_e8437).sqrt();
        (assign8600_e8438, ((-locals.var_qsqrt_dn3) / (2.0 * assign8600_e8438)), ((-locals.var_qsqrt_dn4) / (2.0 * assign8600_e8438)), ((-locals.var_qsqrt_dn5) / (2.0 * assign8600_e8438)), ((-locals.var_qsqrt_dn6) / (2.0 * assign8600_e8438)), ((-locals.var_qsqrt_dn7) / (2.0 * assign8600_e8438)), ((-locals.var_qsqrt_dn8) / (2.0 * assign8600_e8438)),)
    } else {
        (locals.var_q, locals.var_q_dn3, locals.var_q_dn4, locals.var_q_dn5, locals.var_q_dn6, locals.var_q_dn7, locals.var_q_dn8,)
    }
};
        locals.var_q = assign8600_e8440;
        locals.var_q_dn3 = assign8600_e8440_d_n3;
        locals.var_q_dn4 = assign8600_e8440_d_n4;
        locals.var_q_dn5 = assign8600_e8440_d_n5;
        locals.var_q_dn6 = assign8600_e8440_d_n6;
        locals.var_q_dn7 = assign8600_e8440_d_n7;
        locals.var_q_dn8 = assign8600_e8440_d_n8;
        locals.var_q_rv = 0.0;

        let (assign8610_e8449, assign8610_e8449_d_n3, assign8610_e8449_d_n4, assign8610_e8449_d_n5, assign8610_e8449_d_n6, assign8610_e8449_d_n7, assign8610_e8449_d_n8,) = {
    if (locals.var_guard92 != 0.0) {
        let assign8610_e8445: f64 = (0.5 * locals.var_q);
        let assign8610_e8446: f64 = (assign8610_e8445).sin();
        let assign8610_e8447: f64 = (1.0 / assign8610_e8446);
        (assign8610_e8447, (-(((assign8610_e8445).cos() * (0.5 * locals.var_q_dn3)) / (assign8610_e8446 * assign8610_e8446))), (-(((assign8610_e8445).cos() * (0.5 * locals.var_q_dn4)) / (assign8610_e8446 * assign8610_e8446))), (-(((assign8610_e8445).cos() * (0.5 * locals.var_q_dn5)) / (assign8610_e8446 * assign8610_e8446))), (-(((assign8610_e8445).cos() * (0.5 * locals.var_q_dn6)) / (assign8610_e8446 * assign8610_e8446))), (-(((assign8610_e8445).cos() * (0.5 * locals.var_q_dn7)) / (assign8610_e8446 * assign8610_e8446))), (-(((assign8610_e8445).cos() * (0.5 * locals.var_q_dn8)) / (assign8610_e8446 * assign8610_e8446))),)
    } else {
        (locals.var_csc1, locals.var_csc1_dn3, locals.var_csc1_dn4, locals.var_csc1_dn5, locals.var_csc1_dn6, locals.var_csc1_dn7, locals.var_csc1_dn8,)
    }
};
        locals.var_csc1 = assign8610_e8449;
        locals.var_csc1_dn3 = assign8610_e8449_d_n3;
        locals.var_csc1_dn4 = assign8610_e8449_d_n4;
        locals.var_csc1_dn5 = assign8610_e8449_d_n5;
        locals.var_csc1_dn6 = assign8610_e8449_d_n6;
        locals.var_csc1_dn7 = assign8610_e8449_d_n7;
        locals.var_csc1_dn8 = assign8610_e8449_d_n8;
        locals.var_csc1_rv = 0.0;

        let (assign8620_e8455, assign8620_e8455_d_n3, assign8620_e8455_d_n4, assign8620_e8455_d_n5, assign8620_e8455_d_n6, assign8620_e8455_d_n7, assign8620_e8455_d_n8,) = {
    if (locals.var_guard92 != 0.0) {
        let assign8620_e8453: f64 = (locals.var_csc1 * locals.var_csc1);
        (assign8620_e8453, ((locals.var_csc1_dn3 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn3)), ((locals.var_csc1_dn4 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn4)), ((locals.var_csc1_dn5 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn5)), ((locals.var_csc1_dn6 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn6)), ((locals.var_csc1_dn7 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn7)), ((locals.var_csc1_dn8 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn8)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign8620_e8455;
        locals.var_t1_dn3 = assign8620_e8455_d_n3;
        locals.var_t1_dn4 = assign8620_e8455_d_n4;
        locals.var_t1_dn5 = assign8620_e8455_d_n5;
        locals.var_t1_dn6 = assign8620_e8455_d_n6;
        locals.var_t1_dn7 = assign8620_e8455_d_n7;
        locals.var_t1_dn8 = assign8620_e8455_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign8630_e8464, assign8630_e8464_d_n3, assign8630_e8464_d_n4, assign8630_e8464_d_n5, assign8630_e8464_d_n6, assign8630_e8464_d_n7, assign8630_e8464_d_n8,) = {
    if (locals.var_guard92 != 0.0) {
        let assign8630_e8459: f64 = (0.5 * locals.var_q);
        let assign8630_e8460: f64 = (assign8630_e8459).cos();
        let assign8630_e8462: f64 = (assign8630_e8460 * locals.var_csc1);
        (assign8630_e8462, (((-(assign8630_e8459).sin() * (0.5 * locals.var_q_dn3)) * locals.var_csc1) + (assign8630_e8460 * locals.var_csc1_dn3)), (((-(assign8630_e8459).sin() * (0.5 * locals.var_q_dn4)) * locals.var_csc1) + (assign8630_e8460 * locals.var_csc1_dn4)), (((-(assign8630_e8459).sin() * (0.5 * locals.var_q_dn5)) * locals.var_csc1) + (assign8630_e8460 * locals.var_csc1_dn5)), (((-(assign8630_e8459).sin() * (0.5 * locals.var_q_dn6)) * locals.var_csc1) + (assign8630_e8460 * locals.var_csc1_dn6)), (((-(assign8630_e8459).sin() * (0.5 * locals.var_q_dn7)) * locals.var_csc1) + (assign8630_e8460 * locals.var_csc1_dn7)), (((-(assign8630_e8459).sin() * (0.5 * locals.var_q_dn8)) * locals.var_csc1) + (assign8630_e8460 * locals.var_csc1_dn8)),)
    } else {
        (locals.var_coth1, locals.var_coth1_dn3, locals.var_coth1_dn4, locals.var_coth1_dn5, locals.var_coth1_dn6, locals.var_coth1_dn7, locals.var_coth1_dn8,)
    }
};
        locals.var_coth1 = assign8630_e8464;
        locals.var_coth1_dn3 = assign8630_e8464_d_n3;
        locals.var_coth1_dn4 = assign8630_e8464_d_n4;
        locals.var_coth1_dn5 = assign8630_e8464_d_n5;
        locals.var_coth1_dn6 = assign8630_e8464_d_n6;
        locals.var_coth1_dn7 = assign8630_e8464_d_n7;
        locals.var_coth1_dn8 = assign8630_e8464_d_n8;
        locals.var_coth1_rv = 0.0;

        let (assign8640_e8473, assign8640_e8473_d_n3, assign8640_e8473_d_n4, assign8640_e8473_d_n5, assign8640_e8473_d_n6, assign8640_e8473_d_n7, assign8640_e8473_d_n8,) = {
    if (locals.var_guard92 != 0.0) {
        let assign8640_e8467: f64 = (-0.5);
        let assign8640_e8469: f64 = (assign8640_e8467 * locals.var_coth1);
        let assign8640_e8471: f64 = (assign8640_e8469 / locals.var_q);
        (assign8640_e8471, ((((assign8640_e8467 * locals.var_coth1_dn3) * locals.var_q) - (assign8640_e8469 * locals.var_q_dn3)) / (locals.var_q * locals.var_q)), ((((assign8640_e8467 * locals.var_coth1_dn4) * locals.var_q) - (assign8640_e8469 * locals.var_q_dn4)) / (locals.var_q * locals.var_q)), ((((assign8640_e8467 * locals.var_coth1_dn5) * locals.var_q) - (assign8640_e8469 * locals.var_q_dn5)) / (locals.var_q * locals.var_q)), ((((assign8640_e8467 * locals.var_coth1_dn6) * locals.var_q) - (assign8640_e8469 * locals.var_q_dn6)) / (locals.var_q * locals.var_q)), ((((assign8640_e8467 * locals.var_coth1_dn7) * locals.var_q) - (assign8640_e8469 * locals.var_q_dn7)) / (locals.var_q * locals.var_q)), ((((assign8640_e8467 * locals.var_coth1_dn8) * locals.var_q) - (assign8640_e8469 * locals.var_q_dn8)) / (locals.var_q * locals.var_q)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    }
};
        locals.var_t0 = assign8640_e8473;
        locals.var_t0_dn3 = assign8640_e8473_d_n3;
        locals.var_t0_dn4 = assign8640_e8473_d_n4;
        locals.var_t0_dn5 = assign8640_e8473_d_n5;
        locals.var_t0_dn6 = assign8640_e8473_d_n6;
        locals.var_t0_dn7 = assign8640_e8473_d_n7;
        locals.var_t0_dn8 = assign8640_e8473_d_n8;
        locals.var_t0_rv = 0.0;

        let (assign8650_e8481, assign8650_e8481_d_n3, assign8650_e8481_d_n4, assign8650_e8481_d_n5, assign8650_e8481_d_n6, assign8650_e8481_d_n7, assign8650_e8481_d_n8,) = {
    if (locals.var_guard92 != 0.0) {
        let assign8650_e8477: f64 = (0.25 * locals.var_t1);
        let assign8650_e8479: f64 = (assign8650_e8477 + locals.var_t0);
        (assign8650_e8479, ((0.25 * locals.var_t1_dn3) + locals.var_t0_dn3), ((0.25 * locals.var_t1_dn4) + locals.var_t0_dn4), ((0.25 * locals.var_t1_dn5) + locals.var_t0_dn5), ((0.25 * locals.var_t1_dn6) + locals.var_t0_dn6), ((0.25 * locals.var_t1_dn7) + locals.var_t0_dn7), ((0.25 * locals.var_t1_dn8) + locals.var_t0_dn8),)
    } else {
        (locals.var_dqcothqdqsqrt, locals.var_dqcothqdqsqrt_dn3, locals.var_dqcothqdqsqrt_dn4, locals.var_dqcothqdqsqrt_dn5, locals.var_dqcothqdqsqrt_dn6, locals.var_dqcothqdqsqrt_dn7, locals.var_dqcothqdqsqrt_dn8,)
    }
};
        locals.var_dqcothqdqsqrt = assign8650_e8481;
        locals.var_dqcothqdqsqrt_dn3 = assign8650_e8481_d_n3;
        locals.var_dqcothqdqsqrt_dn4 = assign8650_e8481_d_n4;
        locals.var_dqcothqdqsqrt_dn5 = assign8650_e8481_d_n5;
        locals.var_dqcothqdqsqrt_dn6 = assign8650_e8481_d_n6;
        locals.var_dqcothqdqsqrt_dn7 = assign8650_e8481_d_n7;
        locals.var_dqcothqdqsqrt_dn8 = assign8650_e8481_d_n8;
        locals.var_dqcothqdqsqrt_rv = 0.0;

        let (assign8660_e8487, assign8660_e8487_d_n3, assign8660_e8487_d_n4, assign8660_e8487_d_n5, assign8660_e8487_d_n6, assign8660_e8487_d_n7, assign8660_e8487_d_n8,) = {
    if (locals.var_guard92 == 0.0) {
        let assign8660_e8485: f64 = (locals.var_qsqrt).sqrt();
        (assign8660_e8485, (locals.var_qsqrt_dn3 / (2.0 * assign8660_e8485)), (locals.var_qsqrt_dn4 / (2.0 * assign8660_e8485)), (locals.var_qsqrt_dn5 / (2.0 * assign8660_e8485)), (locals.var_qsqrt_dn6 / (2.0 * assign8660_e8485)), (locals.var_qsqrt_dn7 / (2.0 * assign8660_e8485)), (locals.var_qsqrt_dn8 / (2.0 * assign8660_e8485)),)
    } else {
        (locals.var_q, locals.var_q_dn3, locals.var_q_dn4, locals.var_q_dn5, locals.var_q_dn6, locals.var_q_dn7, locals.var_q_dn8,)
    }
};
        locals.var_q = assign8660_e8487;
        locals.var_q_dn3 = assign8660_e8487_d_n3;
        locals.var_q_dn4 = assign8660_e8487_d_n4;
        locals.var_q_dn5 = assign8660_e8487_d_n5;
        locals.var_q_dn6 = assign8660_e8487_d_n6;
        locals.var_q_dn7 = assign8660_e8487_d_n7;
        locals.var_q_dn8 = assign8660_e8487_d_n8;
        locals.var_q_rv = 0.0;

        let (assign8670_e8497, assign8670_e8497_d_n3, assign8670_e8497_d_n4, assign8670_e8497_d_n5, assign8670_e8497_d_n6, assign8670_e8497_d_n7, assign8670_e8497_d_n8,) = {
    if (locals.var_guard92 == 0.0) {
        let assign8670_e8493: f64 = (0.5 * locals.var_q);
        let assign8670_e8494: f64 = (assign8670_e8493).sinh();
        let assign8670_e8495: f64 = (1.0 / assign8670_e8494);
        (assign8670_e8495, (-(((assign8670_e8493).cosh() * (0.5 * locals.var_q_dn3)) / (assign8670_e8494 * assign8670_e8494))), (-(((assign8670_e8493).cosh() * (0.5 * locals.var_q_dn4)) / (assign8670_e8494 * assign8670_e8494))), (-(((assign8670_e8493).cosh() * (0.5 * locals.var_q_dn5)) / (assign8670_e8494 * assign8670_e8494))), (-(((assign8670_e8493).cosh() * (0.5 * locals.var_q_dn6)) / (assign8670_e8494 * assign8670_e8494))), (-(((assign8670_e8493).cosh() * (0.5 * locals.var_q_dn7)) / (assign8670_e8494 * assign8670_e8494))), (-(((assign8670_e8493).cosh() * (0.5 * locals.var_q_dn8)) / (assign8670_e8494 * assign8670_e8494))),)
    } else {
        (locals.var_csc1, locals.var_csc1_dn3, locals.var_csc1_dn4, locals.var_csc1_dn5, locals.var_csc1_dn6, locals.var_csc1_dn7, locals.var_csc1_dn8,)
    }
};
        locals.var_csc1 = assign8670_e8497;
        locals.var_csc1_dn3 = assign8670_e8497_d_n3;
        locals.var_csc1_dn4 = assign8670_e8497_d_n4;
        locals.var_csc1_dn5 = assign8670_e8497_d_n5;
        locals.var_csc1_dn6 = assign8670_e8497_d_n6;
        locals.var_csc1_dn7 = assign8670_e8497_d_n7;
        locals.var_csc1_dn8 = assign8670_e8497_d_n8;
        locals.var_csc1_rv = 0.0;

        let (assign8680_e8504, assign8680_e8504_d_n3, assign8680_e8504_d_n4, assign8680_e8504_d_n5, assign8680_e8504_d_n6, assign8680_e8504_d_n7, assign8680_e8504_d_n8,) = {
    if (locals.var_guard92 == 0.0) {
        let assign8680_e8502: f64 = (locals.var_csc1 * locals.var_csc1);
        (assign8680_e8502, ((locals.var_csc1_dn3 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn3)), ((locals.var_csc1_dn4 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn4)), ((locals.var_csc1_dn5 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn5)), ((locals.var_csc1_dn6 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn6)), ((locals.var_csc1_dn7 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn7)), ((locals.var_csc1_dn8 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn8)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign8680_e8504;
        locals.var_t1_dn3 = assign8680_e8504_d_n3;
        locals.var_t1_dn4 = assign8680_e8504_d_n4;
        locals.var_t1_dn5 = assign8680_e8504_d_n5;
        locals.var_t1_dn6 = assign8680_e8504_d_n6;
        locals.var_t1_dn7 = assign8680_e8504_d_n7;
        locals.var_t1_dn8 = assign8680_e8504_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign8690_e8512, assign8690_e8512_d_n3, assign8690_e8512_d_n4, assign8690_e8512_d_n5, assign8690_e8512_d_n6, assign8690_e8512_d_n7, assign8690_e8512_d_n8,) = {
    if (locals.var_guard92 == 0.0) {
        let assign8690_e8509: f64 = (1.0 + locals.var_t1);
        let assign8690_e8510: f64 = (assign8690_e8509).sqrt();
        (assign8690_e8510, (locals.var_t1_dn3 / (2.0 * assign8690_e8510)), (locals.var_t1_dn4 / (2.0 * assign8690_e8510)), (locals.var_t1_dn5 / (2.0 * assign8690_e8510)), (locals.var_t1_dn6 / (2.0 * assign8690_e8510)), (locals.var_t1_dn7 / (2.0 * assign8690_e8510)), (locals.var_t1_dn8 / (2.0 * assign8690_e8510)),)
    } else {
        (locals.var_coth1, locals.var_coth1_dn3, locals.var_coth1_dn4, locals.var_coth1_dn5, locals.var_coth1_dn6, locals.var_coth1_dn7, locals.var_coth1_dn8,)
    }
};
        locals.var_coth1 = assign8690_e8512;
        locals.var_coth1_dn3 = assign8690_e8512_d_n3;
        locals.var_coth1_dn4 = assign8690_e8512_d_n4;
        locals.var_coth1_dn5 = assign8690_e8512_d_n5;
        locals.var_coth1_dn6 = assign8690_e8512_d_n6;
        locals.var_coth1_dn7 = assign8690_e8512_d_n7;
        locals.var_coth1_dn8 = assign8690_e8512_d_n8;
        locals.var_coth1_rv = 0.0;

        let (assign8700_e8521, assign8700_e8521_d_n3, assign8700_e8521_d_n4, assign8700_e8521_d_n5, assign8700_e8521_d_n6, assign8700_e8521_d_n7, assign8700_e8521_d_n8,) = {
    if (locals.var_guard92 == 0.0) {
        let assign8700_e8517: f64 = (0.5 * locals.var_coth1);
        let assign8700_e8519: f64 = (assign8700_e8517 / locals.var_q);
        (assign8700_e8519, ((((0.5 * locals.var_coth1_dn3) * locals.var_q) - (assign8700_e8517 * locals.var_q_dn3)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn4) * locals.var_q) - (assign8700_e8517 * locals.var_q_dn4)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn5) * locals.var_q) - (assign8700_e8517 * locals.var_q_dn5)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn6) * locals.var_q) - (assign8700_e8517 * locals.var_q_dn6)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn7) * locals.var_q) - (assign8700_e8517 * locals.var_q_dn7)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn8) * locals.var_q) - (assign8700_e8517 * locals.var_q_dn8)) / (locals.var_q * locals.var_q)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    }
};
        locals.var_t0 = assign8700_e8521;
        locals.var_t0_dn3 = assign8700_e8521_d_n3;
        locals.var_t0_dn4 = assign8700_e8521_d_n4;
        locals.var_t0_dn5 = assign8700_e8521_d_n5;
        locals.var_t0_dn6 = assign8700_e8521_d_n6;
        locals.var_t0_dn7 = assign8700_e8521_d_n7;
        locals.var_t0_dn8 = assign8700_e8521_d_n8;
        locals.var_t0_rv = 0.0;

        let (assign8710_e8531, assign8710_e8531_d_n3, assign8710_e8531_d_n4, assign8710_e8531_d_n5, assign8710_e8531_d_n6, assign8710_e8531_d_n7, assign8710_e8531_d_n8,) = {
    if (locals.var_guard92 == 0.0) {
        let assign8710_e8525: f64 = (-0.25);
        let assign8710_e8527: f64 = (assign8710_e8525 * locals.var_t1);
        let assign8710_e8529: f64 = (assign8710_e8527 + locals.var_t0);
        (assign8710_e8529, ((assign8710_e8525 * locals.var_t1_dn3) + locals.var_t0_dn3), ((assign8710_e8525 * locals.var_t1_dn4) + locals.var_t0_dn4), ((assign8710_e8525 * locals.var_t1_dn5) + locals.var_t0_dn5), ((assign8710_e8525 * locals.var_t1_dn6) + locals.var_t0_dn6), ((assign8710_e8525 * locals.var_t1_dn7) + locals.var_t0_dn7), ((assign8710_e8525 * locals.var_t1_dn8) + locals.var_t0_dn8),)
    } else {
        (locals.var_dqcothqdqsqrt, locals.var_dqcothqdqsqrt_dn3, locals.var_dqcothqdqsqrt_dn4, locals.var_dqcothqdqsqrt_dn5, locals.var_dqcothqdqsqrt_dn6, locals.var_dqcothqdqsqrt_dn7, locals.var_dqcothqdqsqrt_dn8,)
    }
};
        locals.var_dqcothqdqsqrt = assign8710_e8531;
        locals.var_dqcothqdqsqrt_dn3 = assign8710_e8531_d_n3;
        locals.var_dqcothqdqsqrt_dn4 = assign8710_e8531_d_n4;
        locals.var_dqcothqdqsqrt_dn5 = assign8710_e8531_d_n5;
        locals.var_dqcothqdqsqrt_dn6 = assign8710_e8531_d_n6;
        locals.var_dqcothqdqsqrt_dn7 = assign8710_e8531_d_n7;
        locals.var_dqcothqdqsqrt_dn8 = assign8710_e8531_d_n8;
        locals.var_dqcothqdqsqrt_rv = 0.0;

        let assign8720_e8534: f64 = (locals.var_q * locals.var_coth1);
        locals.var_qcoth = assign8720_e8534;
        locals.var_qcoth_dn3 = ((locals.var_q_dn3 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn3));
        locals.var_qcoth_dn4 = ((locals.var_q_dn4 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn4));
        locals.var_qcoth_dn5 = ((locals.var_q_dn5 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn5));
        locals.var_qcoth_dn6 = ((locals.var_q_dn6 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn6));
        locals.var_qcoth_dn7 = ((locals.var_q_dn7 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn7));
        locals.var_qcoth_dn8 = ((locals.var_q_dn8 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn8));
        locals.var_qcoth_rv = 0.0;

        let assign8730_e8537: f64 = (locals.var_auxb1 + locals.var_qcoth);
        locals.var_t2 = assign8730_e8537;
        locals.var_t2_dn3 = (locals.var_auxb1_dn3 + locals.var_qcoth_dn3);
        locals.var_t2_dn4 = (locals.var_auxb1_dn4 + locals.var_qcoth_dn4);
        locals.var_t2_dn5 = (locals.var_auxb1_dn5 + locals.var_qcoth_dn5);
        locals.var_t2_dn6 = (locals.var_auxb1_dn6 + locals.var_qcoth_dn6);
        locals.var_t2_dn7 = (locals.var_auxb1_dn7 + locals.var_qcoth_dn7);
        locals.var_t2_dn8 = (locals.var_auxb1_dn8 + locals.var_qcoth_dn8);
        locals.var_t2_rv = 0.0;

        let assign8740_e8540: f64 = (1.0 / locals.var_t2);
        locals.var_t3 = assign8740_e8540;
        locals.var_t3_dn3 = (-(locals.var_t2_dn3 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn4 = (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn5 = (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn6 = (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn7 = (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn8 = (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_rv = 0.0;

        let assign8750_e8543: f64 = (locals.var_xg2 - locals.var_xg1);
        let assign8750_e8545: f64 = (assign8750_e8543 + locals.var_q1);
        let assign8750_e8548: f64 = (locals.var_qsqrt * locals.var_t1);
        let assign8750_e8550: f64 = (assign8750_e8548 * locals.var_t3);
        let assign8750_e8552: f64 = (assign8750_e8550 * locals.var_t3);
        let assign8750_e8553: f64 = (assign8750_e8552).abs();
        let assign8750_e8554: f64 = (assign8750_e8553).ln();
        let assign8750_e8555: f64 = (assign8750_e8545 - assign8750_e8554);
        locals.var_q2 = assign8750_e8555;
        locals.var_q2_dn3 = (((locals.var_xg2_dn3 - locals.var_xg1_dn3) + locals.var_q1_dn3) - (if assign8750_e8552 >= 0.0 { ((((((locals.var_qsqrt_dn3 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn3)) * locals.var_t3) + (assign8750_e8548 * locals.var_t3_dn3)) * locals.var_t3) + (assign8750_e8550 * locals.var_t3_dn3)) } else { (-((((((locals.var_qsqrt_dn3 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn3)) * locals.var_t3) + (assign8750_e8548 * locals.var_t3_dn3)) * locals.var_t3) + (assign8750_e8550 * locals.var_t3_dn3))) } / assign8750_e8553));
        locals.var_q2_dn4 = (((locals.var_xg2_dn4 - locals.var_xg1_dn4) + locals.var_q1_dn4) - (if assign8750_e8552 >= 0.0 { ((((((locals.var_qsqrt_dn4 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn4)) * locals.var_t3) + (assign8750_e8548 * locals.var_t3_dn4)) * locals.var_t3) + (assign8750_e8550 * locals.var_t3_dn4)) } else { (-((((((locals.var_qsqrt_dn4 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn4)) * locals.var_t3) + (assign8750_e8548 * locals.var_t3_dn4)) * locals.var_t3) + (assign8750_e8550 * locals.var_t3_dn4))) } / assign8750_e8553));
        locals.var_q2_dn5 = (((locals.var_xg2_dn5 - locals.var_xg1_dn5) + locals.var_q1_dn5) - (if assign8750_e8552 >= 0.0 { ((((((locals.var_qsqrt_dn5 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn5)) * locals.var_t3) + (assign8750_e8548 * locals.var_t3_dn5)) * locals.var_t3) + (assign8750_e8550 * locals.var_t3_dn5)) } else { (-((((((locals.var_qsqrt_dn5 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn5)) * locals.var_t3) + (assign8750_e8548 * locals.var_t3_dn5)) * locals.var_t3) + (assign8750_e8550 * locals.var_t3_dn5))) } / assign8750_e8553));
        locals.var_q2_dn6 = (((locals.var_xg2_dn6 - locals.var_xg1_dn6) + locals.var_q1_dn6) - (if assign8750_e8552 >= 0.0 { ((((((locals.var_qsqrt_dn6 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn6)) * locals.var_t3) + (assign8750_e8548 * locals.var_t3_dn6)) * locals.var_t3) + (assign8750_e8550 * locals.var_t3_dn6)) } else { (-((((((locals.var_qsqrt_dn6 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn6)) * locals.var_t3) + (assign8750_e8548 * locals.var_t3_dn6)) * locals.var_t3) + (assign8750_e8550 * locals.var_t3_dn6))) } / assign8750_e8553));
        locals.var_q2_dn7 = (((locals.var_xg2_dn7 - locals.var_xg1_dn7) + locals.var_q1_dn7) - (if assign8750_e8552 >= 0.0 { ((((((locals.var_qsqrt_dn7 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn7)) * locals.var_t3) + (assign8750_e8548 * locals.var_t3_dn7)) * locals.var_t3) + (assign8750_e8550 * locals.var_t3_dn7)) } else { (-((((((locals.var_qsqrt_dn7 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn7)) * locals.var_t3) + (assign8750_e8548 * locals.var_t3_dn7)) * locals.var_t3) + (assign8750_e8550 * locals.var_t3_dn7))) } / assign8750_e8553));
        locals.var_q2_dn8 = (((locals.var_xg2_dn8 - locals.var_xg1_dn8) + locals.var_q1_dn8) - (if assign8750_e8552 >= 0.0 { ((((((locals.var_qsqrt_dn8 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn8)) * locals.var_t3) + (assign8750_e8548 * locals.var_t3_dn8)) * locals.var_t3) + (assign8750_e8550 * locals.var_t3_dn8)) } else { (-((((((locals.var_qsqrt_dn8 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn8)) * locals.var_t3) + (assign8750_e8548 * locals.var_t3_dn8)) * locals.var_t3) + (assign8750_e8550 * locals.var_t3_dn8))) } / assign8750_e8553));
        locals.var_q2_rv = 0.0;

        let assign8760_e8559: f64 = (locals.var_auxb1 + locals.var_qcoth);
        let assign8760_e8562: f64 = (locals.var_k2 * locals.var_q2);
        let assign8760_e8564: f64 = (assign8760_e8562 + locals.var_auxb1);
        let assign8760_e8565: f64 = (assign8760_e8559 * assign8760_e8564);
        let assign8760_e8566: f64 = (locals.var_aaux + assign8760_e8565);
        locals.var_f = assign8760_e8566;
        locals.var_f_dn3 = (locals.var_aaux_dn3 + (((locals.var_auxb1_dn3 + locals.var_qcoth_dn3) * assign8760_e8564) + (assign8760_e8559 * ((locals.var_k2 * locals.var_q2_dn3) + locals.var_auxb1_dn3))));
        locals.var_f_dn4 = (locals.var_aaux_dn4 + (((locals.var_auxb1_dn4 + locals.var_qcoth_dn4) * assign8760_e8564) + (assign8760_e8559 * ((locals.var_k2 * locals.var_q2_dn4) + locals.var_auxb1_dn4))));
        locals.var_f_dn5 = (locals.var_aaux_dn5 + (((locals.var_auxb1_dn5 + locals.var_qcoth_dn5) * assign8760_e8564) + (assign8760_e8559 * ((locals.var_k2 * locals.var_q2_dn5) + locals.var_auxb1_dn5))));
        locals.var_f_dn6 = (locals.var_aaux_dn6 + (((locals.var_auxb1_dn6 + locals.var_qcoth_dn6) * assign8760_e8564) + (assign8760_e8559 * ((locals.var_k2 * locals.var_q2_dn6) + locals.var_auxb1_dn6))));
        locals.var_f_dn7 = (locals.var_aaux_dn7 + (((locals.var_auxb1_dn7 + locals.var_qcoth_dn7) * assign8760_e8564) + (assign8760_e8559 * ((locals.var_k2 * locals.var_q2_dn7) + locals.var_auxb1_dn7))));
        locals.var_f_dn8 = (locals.var_aaux_dn8 + (((locals.var_auxb1_dn8 + locals.var_qcoth_dn8) * assign8760_e8564) + (assign8760_e8559 * ((locals.var_k2 * locals.var_q2_dn8) + locals.var_auxb1_dn8))));
        locals.var_f_rv = 0.0;

        let assign8770_e8569: f64 = (1.0 / locals.var_qsqrt);
        let assign8770_e8571: f64 = (assign8770_e8569 - locals.var_t0);
        locals.var_dlogsinhqsqdqsqrt = assign8770_e8571;
        locals.var_dlogsinhqsqdqsqrt_dn3 = ((-(locals.var_qsqrt_dn3 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn3);
        locals.var_dlogsinhqsqdqsqrt_dn4 = ((-(locals.var_qsqrt_dn4 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn4);
        locals.var_dlogsinhqsqdqsqrt_dn5 = ((-(locals.var_qsqrt_dn5 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn5);
        locals.var_dlogsinhqsqdqsqrt_dn6 = ((-(locals.var_qsqrt_dn6 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn6);
        locals.var_dlogsinhqsqdqsqrt_dn7 = ((-(locals.var_qsqrt_dn7 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn7);
        locals.var_dlogsinhqsqdqsqrt_dn8 = ((-(locals.var_qsqrt_dn8 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn8);
        locals.var_dlogsinhqsqdqsqrt_rv = 0.0;

        let assign8780_e8573: f64 = (-2.0);
        let assign8780_e8575: f64 = (assign8780_e8573 * locals.var_k1);
        let assign8780_e8577: f64 = (assign8780_e8575 * locals.var_auxb1);
        let assign8780_e8579: f64 = (assign8780_e8577 + locals.var_aaux);
        locals.var_dqsqrt = assign8780_e8579;
        locals.var_dqsqrt_dn3 = ((assign8780_e8575 * locals.var_auxb1_dn3) + locals.var_aaux_dn3);
        locals.var_dqsqrt_dn4 = ((assign8780_e8575 * locals.var_auxb1_dn4) + locals.var_aaux_dn4);
        locals.var_dqsqrt_dn5 = ((assign8780_e8575 * locals.var_auxb1_dn5) + locals.var_aaux_dn5);
        locals.var_dqsqrt_dn6 = ((assign8780_e8575 * locals.var_auxb1_dn6) + locals.var_aaux_dn6);
        locals.var_dqsqrt_dn7 = ((assign8780_e8575 * locals.var_auxb1_dn7) + locals.var_aaux_dn7);
        locals.var_dqsqrt_dn8 = ((assign8780_e8575 * locals.var_auxb1_dn8) + locals.var_aaux_dn8);
        locals.var_dqsqrt_rv = 0.0;

        let assign8790_e8582: f64 = (locals.var_dqcothqdqsqrt * locals.var_dqsqrt);
        locals.var_dqcoth = assign8790_e8582;
        locals.var_dqcoth_dn3 = ((locals.var_dqcothqdqsqrt_dn3 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn3));
        locals.var_dqcoth_dn4 = ((locals.var_dqcothqdqsqrt_dn4 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn4));
        locals.var_dqcoth_dn5 = ((locals.var_dqcothqdqsqrt_dn5 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn5));
        locals.var_dqcoth_dn6 = ((locals.var_dqcothqdqsqrt_dn6 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn6));
        locals.var_dqcoth_dn7 = ((locals.var_dqcothqdqsqrt_dn7 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn7));
        locals.var_dqcoth_dn8 = ((locals.var_dqcothqdqsqrt_dn8 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn8));
        locals.var_dqcoth_rv = 0.0;

        let assign8800_e8584: f64 = (-1.0);
        let assign8800_e8587: f64 = (-locals.var_k1);
        let assign8800_e8589: f64 = (assign8800_e8587 + locals.var_dqcoth);
        let assign8800_e8591: f64 = (assign8800_e8589 * locals.var_t3);
        let assign8800_e8592: f64 = (2.0 * assign8800_e8591);
        let assign8800_e8593: f64 = (assign8800_e8584 + assign8800_e8592);
        let assign8800_e8596: f64 = (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt);
        let assign8800_e8597: f64 = (assign8800_e8593 - assign8800_e8596);
        locals.var_dq2 = assign8800_e8597;
        locals.var_dq2_dn3 = ((2.0 * ((locals.var_dqcoth_dn3 * locals.var_t3) + (assign8800_e8589 * locals.var_t3_dn3))) - ((locals.var_dlogsinhqsqdqsqrt_dn3 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn3)));
        locals.var_dq2_dn4 = ((2.0 * ((locals.var_dqcoth_dn4 * locals.var_t3) + (assign8800_e8589 * locals.var_t3_dn4))) - ((locals.var_dlogsinhqsqdqsqrt_dn4 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn4)));
        locals.var_dq2_dn5 = ((2.0 * ((locals.var_dqcoth_dn5 * locals.var_t3) + (assign8800_e8589 * locals.var_t3_dn5))) - ((locals.var_dlogsinhqsqdqsqrt_dn5 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn5)));
        locals.var_dq2_dn6 = ((2.0 * ((locals.var_dqcoth_dn6 * locals.var_t3) + (assign8800_e8589 * locals.var_t3_dn6))) - ((locals.var_dlogsinhqsqdqsqrt_dn6 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn6)));
        locals.var_dq2_dn7 = ((2.0 * ((locals.var_dqcoth_dn7 * locals.var_t3) + (assign8800_e8589 * locals.var_t3_dn7))) - ((locals.var_dlogsinhqsqdqsqrt_dn7 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn7)));
        locals.var_dq2_dn8 = ((2.0 * ((locals.var_dqcoth_dn8 * locals.var_t3) + (assign8800_e8589 * locals.var_t3_dn8))) - ((locals.var_dlogsinhqsqdqsqrt_dn8 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn8)));
        locals.var_dq2_rv = 0.0;

        let assign8810_e8602: f64 = (locals.var_auxb1 + locals.var_t2);
        let assign8810_e8603: f64 = (locals.var_k1 * assign8810_e8602);
        let assign8810_e8604: f64 = (locals.var_aaux - assign8810_e8603);
        let assign8810_e8607: f64 = (locals.var_auxb1 * locals.var_dqcoth);
        let assign8810_e8608: f64 = (assign8810_e8604 + assign8810_e8607);
        let assign8810_e8612: f64 = (locals.var_dq2 * locals.var_t2);
        let assign8810_e8616: f64 = (locals.var_dqcoth - locals.var_k1);
        let assign8810_e8617: f64 = (locals.var_q2 * assign8810_e8616);
        let assign8810_e8618: f64 = (assign8810_e8612 + assign8810_e8617);
        let assign8810_e8619: f64 = (locals.var_k2 * assign8810_e8618);
        let assign8810_e8620: f64 = (assign8810_e8608 + assign8810_e8619);
        locals.var_df = assign8810_e8620;
        locals.var_df_dn3 = (((locals.var_aaux_dn3 - (locals.var_k1 * (locals.var_auxb1_dn3 + locals.var_t2_dn3))) + ((locals.var_auxb1_dn3 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn3))) + (locals.var_k2 * (((locals.var_dq2_dn3 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn3)) + ((locals.var_q2_dn3 * assign8810_e8616) + (locals.var_q2 * locals.var_dqcoth_dn3)))));
        locals.var_df_dn4 = (((locals.var_aaux_dn4 - (locals.var_k1 * (locals.var_auxb1_dn4 + locals.var_t2_dn4))) + ((locals.var_auxb1_dn4 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn4))) + (locals.var_k2 * (((locals.var_dq2_dn4 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn4)) + ((locals.var_q2_dn4 * assign8810_e8616) + (locals.var_q2 * locals.var_dqcoth_dn4)))));
        locals.var_df_dn5 = (((locals.var_aaux_dn5 - (locals.var_k1 * (locals.var_auxb1_dn5 + locals.var_t2_dn5))) + ((locals.var_auxb1_dn5 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn5))) + (locals.var_k2 * (((locals.var_dq2_dn5 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn5)) + ((locals.var_q2_dn5 * assign8810_e8616) + (locals.var_q2 * locals.var_dqcoth_dn5)))));
        locals.var_df_dn6 = (((locals.var_aaux_dn6 - (locals.var_k1 * (locals.var_auxb1_dn6 + locals.var_t2_dn6))) + ((locals.var_auxb1_dn6 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn6))) + (locals.var_k2 * (((locals.var_dq2_dn6 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn6)) + ((locals.var_q2_dn6 * assign8810_e8616) + (locals.var_q2 * locals.var_dqcoth_dn6)))));
        locals.var_df_dn7 = (((locals.var_aaux_dn7 - (locals.var_k1 * (locals.var_auxb1_dn7 + locals.var_t2_dn7))) + ((locals.var_auxb1_dn7 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn7))) + (locals.var_k2 * (((locals.var_dq2_dn7 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn7)) + ((locals.var_q2_dn7 * assign8810_e8616) + (locals.var_q2 * locals.var_dqcoth_dn7)))));
        locals.var_df_dn8 = (((locals.var_aaux_dn8 - (locals.var_k1 * (locals.var_auxb1_dn8 + locals.var_t2_dn8))) + ((locals.var_auxb1_dn8 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn8))) + (locals.var_k2 * (((locals.var_dq2_dn8 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn8)) + ((locals.var_q2_dn8 * assign8810_e8616) + (locals.var_q2 * locals.var_dqcoth_dn8)))));
        locals.var_df_rv = 0.0;

        let assign8820_e8622: f64 = (-locals.var_f);
        let assign8820_e8624: f64 = (assign8820_e8622 / locals.var_df);
        locals.var_delta = assign8820_e8624;
        locals.var_delta_dn3 = ((((-locals.var_f_dn3) * locals.var_df) - (assign8820_e8622 * locals.var_df_dn3)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn4 = ((((-locals.var_f_dn4) * locals.var_df) - (assign8820_e8622 * locals.var_df_dn4)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn5 = ((((-locals.var_f_dn5) * locals.var_df) - (assign8820_e8622 * locals.var_df_dn5)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn6 = ((((-locals.var_f_dn6) * locals.var_df) - (assign8820_e8622 * locals.var_df_dn6)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn7 = ((((-locals.var_f_dn7) * locals.var_df) - (assign8820_e8622 * locals.var_df_dn7)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn8 = ((((-locals.var_f_dn8) * locals.var_df) - (assign8820_e8622 * locals.var_df_dn8)) / (locals.var_df * locals.var_df));
        locals.var_delta_rv = 0.0;

        let assign8830_e8627: f64 = (locals.var_phi1 + locals.var_delta);
        locals.var_phi1 = assign8830_e8627;
        locals.var_phi1_dn3 = (locals.var_phi1_dn3 + locals.var_delta_dn3);
        locals.var_phi1_dn4 = (locals.var_phi1_dn4 + locals.var_delta_dn4);
        locals.var_phi1_dn5 = (locals.var_phi1_dn5 + locals.var_delta_dn5);
        locals.var_phi1_dn6 = (locals.var_phi1_dn6 + locals.var_delta_dn6);
        locals.var_phi1_dn7 = (locals.var_phi1_dn7 + locals.var_delta_dn7);
        locals.var_phi1_dn8 = (locals.var_phi1_dn8 + locals.var_delta_dn8);
        locals.var_phi1_rv = 0.0;

        let assign8840_e8630: f64 = (locals.var_xg1 - locals.var_phi1);
        locals.var_q1 = assign8840_e8630;
        locals.var_q1_dn3 = (locals.var_xg1_dn3 - locals.var_phi1_dn3);
        locals.var_q1_dn4 = (locals.var_xg1_dn4 - locals.var_phi1_dn4);
        locals.var_q1_dn5 = (locals.var_xg1_dn5 - locals.var_phi1_dn5);
        locals.var_q1_dn6 = (locals.var_xg1_dn6 - locals.var_phi1_dn6);
        locals.var_q1_dn7 = (locals.var_xg1_dn7 - locals.var_phi1_dn7);
        locals.var_q1_dn8 = (locals.var_xg1_dn8 - locals.var_phi1_dn8);
        locals.var_q1_rv = 0.0;

        let assign8850_e8633: f64 = (locals.var_k1 * locals.var_q1);
        locals.var_auxb1 = assign8850_e8633;
        locals.var_auxb1_dn3 = (locals.var_k1 * locals.var_q1_dn3);
        locals.var_auxb1_dn4 = (locals.var_k1 * locals.var_q1_dn4);
        locals.var_auxb1_dn5 = (locals.var_k1 * locals.var_q1_dn5);
        locals.var_auxb1_dn6 = (locals.var_k1 * locals.var_q1_dn6);
        locals.var_auxb1_dn7 = (locals.var_k1 * locals.var_q1_dn7);
        locals.var_auxb1_dn8 = (locals.var_k1 * locals.var_q1_dn8);
        locals.var_auxb1_rv = 0.0;

        let assign8860_e8635: f64 = (-locals.var_a0);
        let assign8860_e8637: f64 = (locals.var_phi1).exp();
        let assign8860_e8638: f64 = (assign8860_e8635 * assign8860_e8637);
        locals.var_aaux = assign8860_e8638;
        locals.var_aaux_dn3 = (((-locals.var_a0_dn3) * assign8860_e8637) + (assign8860_e8635 * (assign8860_e8637 * locals.var_phi1_dn3)));
        locals.var_aaux_dn4 = (((-locals.var_a0_dn4) * assign8860_e8637) + (assign8860_e8635 * (assign8860_e8637 * locals.var_phi1_dn4)));
        locals.var_aaux_dn5 = (((-locals.var_a0_dn5) * assign8860_e8637) + (assign8860_e8635 * (assign8860_e8637 * locals.var_phi1_dn5)));
        locals.var_aaux_dn6 = (((-locals.var_a0_dn6) * assign8860_e8637) + (assign8860_e8635 * (assign8860_e8637 * locals.var_phi1_dn6)));
        locals.var_aaux_dn7 = (((-locals.var_a0_dn7) * assign8860_e8637) + (assign8860_e8635 * (assign8860_e8637 * locals.var_phi1_dn7)));
        locals.var_aaux_dn8 = (((-locals.var_a0_dn8) * assign8860_e8637) + (assign8860_e8635 * (assign8860_e8637 * locals.var_phi1_dn8)));
        locals.var_aaux_rv = 0.0;

        let assign8870_e8641: f64 = (locals.var_auxb1 * locals.var_auxb1);
        let assign8870_e8643: f64 = (assign8870_e8641 + locals.var_aaux);
        locals.var_qsqrt = assign8870_e8643;
        locals.var_qsqrt_dn3 = (((locals.var_auxb1_dn3 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn3)) + locals.var_aaux_dn3);
        locals.var_qsqrt_dn4 = (((locals.var_auxb1_dn4 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn4)) + locals.var_aaux_dn4);
        locals.var_qsqrt_dn5 = (((locals.var_auxb1_dn5 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn5)) + locals.var_aaux_dn5);
        locals.var_qsqrt_dn6 = (((locals.var_auxb1_dn6 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn6)) + locals.var_aaux_dn6);
        locals.var_qsqrt_dn7 = (((locals.var_auxb1_dn7 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn7)) + locals.var_aaux_dn7);
        locals.var_qsqrt_dn8 = (((locals.var_auxb1_dn8 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn8)) + locals.var_aaux_dn8);
        locals.var_qsqrt_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_19(
        locals: &mut StampLocals,
    ) {
        let assign8880_e8646: f64 = if locals.var_qsqrt < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard93 = assign8880_e8646;
        locals.var_guard93_rv = 0.0;

        let (assign8890_e8652, assign8890_e8652_d_n3, assign8890_e8652_d_n4, assign8890_e8652_d_n5, assign8890_e8652_d_n6, assign8890_e8652_d_n7, assign8890_e8652_d_n8,) = {
    if (locals.var_guard93 != 0.0) {
        let assign8890_e8649: f64 = (-locals.var_qsqrt);
        let assign8890_e8650: f64 = (assign8890_e8649).sqrt();
        (assign8890_e8650, ((-locals.var_qsqrt_dn3) / (2.0 * assign8890_e8650)), ((-locals.var_qsqrt_dn4) / (2.0 * assign8890_e8650)), ((-locals.var_qsqrt_dn5) / (2.0 * assign8890_e8650)), ((-locals.var_qsqrt_dn6) / (2.0 * assign8890_e8650)), ((-locals.var_qsqrt_dn7) / (2.0 * assign8890_e8650)), ((-locals.var_qsqrt_dn8) / (2.0 * assign8890_e8650)),)
    } else {
        (locals.var_q, locals.var_q_dn3, locals.var_q_dn4, locals.var_q_dn5, locals.var_q_dn6, locals.var_q_dn7, locals.var_q_dn8,)
    }
};
        locals.var_q = assign8890_e8652;
        locals.var_q_dn3 = assign8890_e8652_d_n3;
        locals.var_q_dn4 = assign8890_e8652_d_n4;
        locals.var_q_dn5 = assign8890_e8652_d_n5;
        locals.var_q_dn6 = assign8890_e8652_d_n6;
        locals.var_q_dn7 = assign8890_e8652_d_n7;
        locals.var_q_dn8 = assign8890_e8652_d_n8;
        locals.var_q_rv = 0.0;

        let (assign8900_e8661, assign8900_e8661_d_n3, assign8900_e8661_d_n4, assign8900_e8661_d_n5, assign8900_e8661_d_n6, assign8900_e8661_d_n7, assign8900_e8661_d_n8,) = {
    if (locals.var_guard93 != 0.0) {
        let assign8900_e8657: f64 = (0.5 * locals.var_q);
        let assign8900_e8658: f64 = (assign8900_e8657).sin();
        let assign8900_e8659: f64 = (1.0 / assign8900_e8658);
        (assign8900_e8659, (-(((assign8900_e8657).cos() * (0.5 * locals.var_q_dn3)) / (assign8900_e8658 * assign8900_e8658))), (-(((assign8900_e8657).cos() * (0.5 * locals.var_q_dn4)) / (assign8900_e8658 * assign8900_e8658))), (-(((assign8900_e8657).cos() * (0.5 * locals.var_q_dn5)) / (assign8900_e8658 * assign8900_e8658))), (-(((assign8900_e8657).cos() * (0.5 * locals.var_q_dn6)) / (assign8900_e8658 * assign8900_e8658))), (-(((assign8900_e8657).cos() * (0.5 * locals.var_q_dn7)) / (assign8900_e8658 * assign8900_e8658))), (-(((assign8900_e8657).cos() * (0.5 * locals.var_q_dn8)) / (assign8900_e8658 * assign8900_e8658))),)
    } else {
        (locals.var_csc1, locals.var_csc1_dn3, locals.var_csc1_dn4, locals.var_csc1_dn5, locals.var_csc1_dn6, locals.var_csc1_dn7, locals.var_csc1_dn8,)
    }
};
        locals.var_csc1 = assign8900_e8661;
        locals.var_csc1_dn3 = assign8900_e8661_d_n3;
        locals.var_csc1_dn4 = assign8900_e8661_d_n4;
        locals.var_csc1_dn5 = assign8900_e8661_d_n5;
        locals.var_csc1_dn6 = assign8900_e8661_d_n6;
        locals.var_csc1_dn7 = assign8900_e8661_d_n7;
        locals.var_csc1_dn8 = assign8900_e8661_d_n8;
        locals.var_csc1_rv = 0.0;

        let (assign8910_e8667, assign8910_e8667_d_n3, assign8910_e8667_d_n4, assign8910_e8667_d_n5, assign8910_e8667_d_n6, assign8910_e8667_d_n7, assign8910_e8667_d_n8,) = {
    if (locals.var_guard93 != 0.0) {
        let assign8910_e8665: f64 = (locals.var_csc1 * locals.var_csc1);
        (assign8910_e8665, ((locals.var_csc1_dn3 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn3)), ((locals.var_csc1_dn4 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn4)), ((locals.var_csc1_dn5 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn5)), ((locals.var_csc1_dn6 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn6)), ((locals.var_csc1_dn7 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn7)), ((locals.var_csc1_dn8 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn8)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign8910_e8667;
        locals.var_t1_dn3 = assign8910_e8667_d_n3;
        locals.var_t1_dn4 = assign8910_e8667_d_n4;
        locals.var_t1_dn5 = assign8910_e8667_d_n5;
        locals.var_t1_dn6 = assign8910_e8667_d_n6;
        locals.var_t1_dn7 = assign8910_e8667_d_n7;
        locals.var_t1_dn8 = assign8910_e8667_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign8920_e8676, assign8920_e8676_d_n3, assign8920_e8676_d_n4, assign8920_e8676_d_n5, assign8920_e8676_d_n6, assign8920_e8676_d_n7, assign8920_e8676_d_n8,) = {
    if (locals.var_guard93 != 0.0) {
        let assign8920_e8671: f64 = (0.5 * locals.var_q);
        let assign8920_e8672: f64 = (assign8920_e8671).cos();
        let assign8920_e8674: f64 = (assign8920_e8672 * locals.var_csc1);
        (assign8920_e8674, (((-(assign8920_e8671).sin() * (0.5 * locals.var_q_dn3)) * locals.var_csc1) + (assign8920_e8672 * locals.var_csc1_dn3)), (((-(assign8920_e8671).sin() * (0.5 * locals.var_q_dn4)) * locals.var_csc1) + (assign8920_e8672 * locals.var_csc1_dn4)), (((-(assign8920_e8671).sin() * (0.5 * locals.var_q_dn5)) * locals.var_csc1) + (assign8920_e8672 * locals.var_csc1_dn5)), (((-(assign8920_e8671).sin() * (0.5 * locals.var_q_dn6)) * locals.var_csc1) + (assign8920_e8672 * locals.var_csc1_dn6)), (((-(assign8920_e8671).sin() * (0.5 * locals.var_q_dn7)) * locals.var_csc1) + (assign8920_e8672 * locals.var_csc1_dn7)), (((-(assign8920_e8671).sin() * (0.5 * locals.var_q_dn8)) * locals.var_csc1) + (assign8920_e8672 * locals.var_csc1_dn8)),)
    } else {
        (locals.var_coth1, locals.var_coth1_dn3, locals.var_coth1_dn4, locals.var_coth1_dn5, locals.var_coth1_dn6, locals.var_coth1_dn7, locals.var_coth1_dn8,)
    }
};
        locals.var_coth1 = assign8920_e8676;
        locals.var_coth1_dn3 = assign8920_e8676_d_n3;
        locals.var_coth1_dn4 = assign8920_e8676_d_n4;
        locals.var_coth1_dn5 = assign8920_e8676_d_n5;
        locals.var_coth1_dn6 = assign8920_e8676_d_n6;
        locals.var_coth1_dn7 = assign8920_e8676_d_n7;
        locals.var_coth1_dn8 = assign8920_e8676_d_n8;
        locals.var_coth1_rv = 0.0;

        let (assign8930_e8685, assign8930_e8685_d_n3, assign8930_e8685_d_n4, assign8930_e8685_d_n5, assign8930_e8685_d_n6, assign8930_e8685_d_n7, assign8930_e8685_d_n8,) = {
    if (locals.var_guard93 != 0.0) {
        let assign8930_e8679: f64 = (-0.5);
        let assign8930_e8681: f64 = (assign8930_e8679 * locals.var_coth1);
        let assign8930_e8683: f64 = (assign8930_e8681 / locals.var_q);
        (assign8930_e8683, ((((assign8930_e8679 * locals.var_coth1_dn3) * locals.var_q) - (assign8930_e8681 * locals.var_q_dn3)) / (locals.var_q * locals.var_q)), ((((assign8930_e8679 * locals.var_coth1_dn4) * locals.var_q) - (assign8930_e8681 * locals.var_q_dn4)) / (locals.var_q * locals.var_q)), ((((assign8930_e8679 * locals.var_coth1_dn5) * locals.var_q) - (assign8930_e8681 * locals.var_q_dn5)) / (locals.var_q * locals.var_q)), ((((assign8930_e8679 * locals.var_coth1_dn6) * locals.var_q) - (assign8930_e8681 * locals.var_q_dn6)) / (locals.var_q * locals.var_q)), ((((assign8930_e8679 * locals.var_coth1_dn7) * locals.var_q) - (assign8930_e8681 * locals.var_q_dn7)) / (locals.var_q * locals.var_q)), ((((assign8930_e8679 * locals.var_coth1_dn8) * locals.var_q) - (assign8930_e8681 * locals.var_q_dn8)) / (locals.var_q * locals.var_q)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    }
};
        locals.var_t0 = assign8930_e8685;
        locals.var_t0_dn3 = assign8930_e8685_d_n3;
        locals.var_t0_dn4 = assign8930_e8685_d_n4;
        locals.var_t0_dn5 = assign8930_e8685_d_n5;
        locals.var_t0_dn6 = assign8930_e8685_d_n6;
        locals.var_t0_dn7 = assign8930_e8685_d_n7;
        locals.var_t0_dn8 = assign8930_e8685_d_n8;
        locals.var_t0_rv = 0.0;

        let (assign8940_e8693, assign8940_e8693_d_n3, assign8940_e8693_d_n4, assign8940_e8693_d_n5, assign8940_e8693_d_n6, assign8940_e8693_d_n7, assign8940_e8693_d_n8,) = {
    if (locals.var_guard93 != 0.0) {
        let assign8940_e8689: f64 = (0.25 * locals.var_t1);
        let assign8940_e8691: f64 = (assign8940_e8689 + locals.var_t0);
        (assign8940_e8691, ((0.25 * locals.var_t1_dn3) + locals.var_t0_dn3), ((0.25 * locals.var_t1_dn4) + locals.var_t0_dn4), ((0.25 * locals.var_t1_dn5) + locals.var_t0_dn5), ((0.25 * locals.var_t1_dn6) + locals.var_t0_dn6), ((0.25 * locals.var_t1_dn7) + locals.var_t0_dn7), ((0.25 * locals.var_t1_dn8) + locals.var_t0_dn8),)
    } else {
        (locals.var_dqcothqdqsqrt, locals.var_dqcothqdqsqrt_dn3, locals.var_dqcothqdqsqrt_dn4, locals.var_dqcothqdqsqrt_dn5, locals.var_dqcothqdqsqrt_dn6, locals.var_dqcothqdqsqrt_dn7, locals.var_dqcothqdqsqrt_dn8,)
    }
};
        locals.var_dqcothqdqsqrt = assign8940_e8693;
        locals.var_dqcothqdqsqrt_dn3 = assign8940_e8693_d_n3;
        locals.var_dqcothqdqsqrt_dn4 = assign8940_e8693_d_n4;
        locals.var_dqcothqdqsqrt_dn5 = assign8940_e8693_d_n5;
        locals.var_dqcothqdqsqrt_dn6 = assign8940_e8693_d_n6;
        locals.var_dqcothqdqsqrt_dn7 = assign8940_e8693_d_n7;
        locals.var_dqcothqdqsqrt_dn8 = assign8940_e8693_d_n8;
        locals.var_dqcothqdqsqrt_rv = 0.0;

        let (assign8950_e8699, assign8950_e8699_d_n3, assign8950_e8699_d_n4, assign8950_e8699_d_n5, assign8950_e8699_d_n6, assign8950_e8699_d_n7, assign8950_e8699_d_n8,) = {
    if (locals.var_guard93 == 0.0) {
        let assign8950_e8697: f64 = (locals.var_qsqrt).sqrt();
        (assign8950_e8697, (locals.var_qsqrt_dn3 / (2.0 * assign8950_e8697)), (locals.var_qsqrt_dn4 / (2.0 * assign8950_e8697)), (locals.var_qsqrt_dn5 / (2.0 * assign8950_e8697)), (locals.var_qsqrt_dn6 / (2.0 * assign8950_e8697)), (locals.var_qsqrt_dn7 / (2.0 * assign8950_e8697)), (locals.var_qsqrt_dn8 / (2.0 * assign8950_e8697)),)
    } else {
        (locals.var_q, locals.var_q_dn3, locals.var_q_dn4, locals.var_q_dn5, locals.var_q_dn6, locals.var_q_dn7, locals.var_q_dn8,)
    }
};
        locals.var_q = assign8950_e8699;
        locals.var_q_dn3 = assign8950_e8699_d_n3;
        locals.var_q_dn4 = assign8950_e8699_d_n4;
        locals.var_q_dn5 = assign8950_e8699_d_n5;
        locals.var_q_dn6 = assign8950_e8699_d_n6;
        locals.var_q_dn7 = assign8950_e8699_d_n7;
        locals.var_q_dn8 = assign8950_e8699_d_n8;
        locals.var_q_rv = 0.0;

        let (assign8960_e8709, assign8960_e8709_d_n3, assign8960_e8709_d_n4, assign8960_e8709_d_n5, assign8960_e8709_d_n6, assign8960_e8709_d_n7, assign8960_e8709_d_n8,) = {
    if (locals.var_guard93 == 0.0) {
        let assign8960_e8705: f64 = (0.5 * locals.var_q);
        let assign8960_e8706: f64 = (assign8960_e8705).sinh();
        let assign8960_e8707: f64 = (1.0 / assign8960_e8706);
        (assign8960_e8707, (-(((assign8960_e8705).cosh() * (0.5 * locals.var_q_dn3)) / (assign8960_e8706 * assign8960_e8706))), (-(((assign8960_e8705).cosh() * (0.5 * locals.var_q_dn4)) / (assign8960_e8706 * assign8960_e8706))), (-(((assign8960_e8705).cosh() * (0.5 * locals.var_q_dn5)) / (assign8960_e8706 * assign8960_e8706))), (-(((assign8960_e8705).cosh() * (0.5 * locals.var_q_dn6)) / (assign8960_e8706 * assign8960_e8706))), (-(((assign8960_e8705).cosh() * (0.5 * locals.var_q_dn7)) / (assign8960_e8706 * assign8960_e8706))), (-(((assign8960_e8705).cosh() * (0.5 * locals.var_q_dn8)) / (assign8960_e8706 * assign8960_e8706))),)
    } else {
        (locals.var_csc1, locals.var_csc1_dn3, locals.var_csc1_dn4, locals.var_csc1_dn5, locals.var_csc1_dn6, locals.var_csc1_dn7, locals.var_csc1_dn8,)
    }
};
        locals.var_csc1 = assign8960_e8709;
        locals.var_csc1_dn3 = assign8960_e8709_d_n3;
        locals.var_csc1_dn4 = assign8960_e8709_d_n4;
        locals.var_csc1_dn5 = assign8960_e8709_d_n5;
        locals.var_csc1_dn6 = assign8960_e8709_d_n6;
        locals.var_csc1_dn7 = assign8960_e8709_d_n7;
        locals.var_csc1_dn8 = assign8960_e8709_d_n8;
        locals.var_csc1_rv = 0.0;

        let (assign8970_e8716, assign8970_e8716_d_n3, assign8970_e8716_d_n4, assign8970_e8716_d_n5, assign8970_e8716_d_n6, assign8970_e8716_d_n7, assign8970_e8716_d_n8,) = {
    if (locals.var_guard93 == 0.0) {
        let assign8970_e8714: f64 = (locals.var_csc1 * locals.var_csc1);
        (assign8970_e8714, ((locals.var_csc1_dn3 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn3)), ((locals.var_csc1_dn4 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn4)), ((locals.var_csc1_dn5 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn5)), ((locals.var_csc1_dn6 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn6)), ((locals.var_csc1_dn7 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn7)), ((locals.var_csc1_dn8 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn8)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign8970_e8716;
        locals.var_t1_dn3 = assign8970_e8716_d_n3;
        locals.var_t1_dn4 = assign8970_e8716_d_n4;
        locals.var_t1_dn5 = assign8970_e8716_d_n5;
        locals.var_t1_dn6 = assign8970_e8716_d_n6;
        locals.var_t1_dn7 = assign8970_e8716_d_n7;
        locals.var_t1_dn8 = assign8970_e8716_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign8980_e8724, assign8980_e8724_d_n3, assign8980_e8724_d_n4, assign8980_e8724_d_n5, assign8980_e8724_d_n6, assign8980_e8724_d_n7, assign8980_e8724_d_n8,) = {
    if (locals.var_guard93 == 0.0) {
        let assign8980_e8721: f64 = (1.0 + locals.var_t1);
        let assign8980_e8722: f64 = (assign8980_e8721).sqrt();
        (assign8980_e8722, (locals.var_t1_dn3 / (2.0 * assign8980_e8722)), (locals.var_t1_dn4 / (2.0 * assign8980_e8722)), (locals.var_t1_dn5 / (2.0 * assign8980_e8722)), (locals.var_t1_dn6 / (2.0 * assign8980_e8722)), (locals.var_t1_dn7 / (2.0 * assign8980_e8722)), (locals.var_t1_dn8 / (2.0 * assign8980_e8722)),)
    } else {
        (locals.var_coth1, locals.var_coth1_dn3, locals.var_coth1_dn4, locals.var_coth1_dn5, locals.var_coth1_dn6, locals.var_coth1_dn7, locals.var_coth1_dn8,)
    }
};
        locals.var_coth1 = assign8980_e8724;
        locals.var_coth1_dn3 = assign8980_e8724_d_n3;
        locals.var_coth1_dn4 = assign8980_e8724_d_n4;
        locals.var_coth1_dn5 = assign8980_e8724_d_n5;
        locals.var_coth1_dn6 = assign8980_e8724_d_n6;
        locals.var_coth1_dn7 = assign8980_e8724_d_n7;
        locals.var_coth1_dn8 = assign8980_e8724_d_n8;
        locals.var_coth1_rv = 0.0;

        let (assign8990_e8733, assign8990_e8733_d_n3, assign8990_e8733_d_n4, assign8990_e8733_d_n5, assign8990_e8733_d_n6, assign8990_e8733_d_n7, assign8990_e8733_d_n8,) = {
    if (locals.var_guard93 == 0.0) {
        let assign8990_e8729: f64 = (0.5 * locals.var_coth1);
        let assign8990_e8731: f64 = (assign8990_e8729 / locals.var_q);
        (assign8990_e8731, ((((0.5 * locals.var_coth1_dn3) * locals.var_q) - (assign8990_e8729 * locals.var_q_dn3)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn4) * locals.var_q) - (assign8990_e8729 * locals.var_q_dn4)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn5) * locals.var_q) - (assign8990_e8729 * locals.var_q_dn5)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn6) * locals.var_q) - (assign8990_e8729 * locals.var_q_dn6)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn7) * locals.var_q) - (assign8990_e8729 * locals.var_q_dn7)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn8) * locals.var_q) - (assign8990_e8729 * locals.var_q_dn8)) / (locals.var_q * locals.var_q)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    }
};
        locals.var_t0 = assign8990_e8733;
        locals.var_t0_dn3 = assign8990_e8733_d_n3;
        locals.var_t0_dn4 = assign8990_e8733_d_n4;
        locals.var_t0_dn5 = assign8990_e8733_d_n5;
        locals.var_t0_dn6 = assign8990_e8733_d_n6;
        locals.var_t0_dn7 = assign8990_e8733_d_n7;
        locals.var_t0_dn8 = assign8990_e8733_d_n8;
        locals.var_t0_rv = 0.0;

        let (assign9000_e8743, assign9000_e8743_d_n3, assign9000_e8743_d_n4, assign9000_e8743_d_n5, assign9000_e8743_d_n6, assign9000_e8743_d_n7, assign9000_e8743_d_n8,) = {
    if (locals.var_guard93 == 0.0) {
        let assign9000_e8737: f64 = (-0.25);
        let assign9000_e8739: f64 = (assign9000_e8737 * locals.var_t1);
        let assign9000_e8741: f64 = (assign9000_e8739 + locals.var_t0);
        (assign9000_e8741, ((assign9000_e8737 * locals.var_t1_dn3) + locals.var_t0_dn3), ((assign9000_e8737 * locals.var_t1_dn4) + locals.var_t0_dn4), ((assign9000_e8737 * locals.var_t1_dn5) + locals.var_t0_dn5), ((assign9000_e8737 * locals.var_t1_dn6) + locals.var_t0_dn6), ((assign9000_e8737 * locals.var_t1_dn7) + locals.var_t0_dn7), ((assign9000_e8737 * locals.var_t1_dn8) + locals.var_t0_dn8),)
    } else {
        (locals.var_dqcothqdqsqrt, locals.var_dqcothqdqsqrt_dn3, locals.var_dqcothqdqsqrt_dn4, locals.var_dqcothqdqsqrt_dn5, locals.var_dqcothqdqsqrt_dn6, locals.var_dqcothqdqsqrt_dn7, locals.var_dqcothqdqsqrt_dn8,)
    }
};
        locals.var_dqcothqdqsqrt = assign9000_e8743;
        locals.var_dqcothqdqsqrt_dn3 = assign9000_e8743_d_n3;
        locals.var_dqcothqdqsqrt_dn4 = assign9000_e8743_d_n4;
        locals.var_dqcothqdqsqrt_dn5 = assign9000_e8743_d_n5;
        locals.var_dqcothqdqsqrt_dn6 = assign9000_e8743_d_n6;
        locals.var_dqcothqdqsqrt_dn7 = assign9000_e8743_d_n7;
        locals.var_dqcothqdqsqrt_dn8 = assign9000_e8743_d_n8;
        locals.var_dqcothqdqsqrt_rv = 0.0;

        let assign9010_e8746: f64 = (locals.var_q * locals.var_coth1);
        locals.var_qcoth = assign9010_e8746;
        locals.var_qcoth_dn3 = ((locals.var_q_dn3 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn3));
        locals.var_qcoth_dn4 = ((locals.var_q_dn4 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn4));
        locals.var_qcoth_dn5 = ((locals.var_q_dn5 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn5));
        locals.var_qcoth_dn6 = ((locals.var_q_dn6 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn6));
        locals.var_qcoth_dn7 = ((locals.var_q_dn7 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn7));
        locals.var_qcoth_dn8 = ((locals.var_q_dn8 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn8));
        locals.var_qcoth_rv = 0.0;

        let assign9020_e8749: f64 = (locals.var_auxb1 + locals.var_qcoth);
        locals.var_t2 = assign9020_e8749;
        locals.var_t2_dn3 = (locals.var_auxb1_dn3 + locals.var_qcoth_dn3);
        locals.var_t2_dn4 = (locals.var_auxb1_dn4 + locals.var_qcoth_dn4);
        locals.var_t2_dn5 = (locals.var_auxb1_dn5 + locals.var_qcoth_dn5);
        locals.var_t2_dn6 = (locals.var_auxb1_dn6 + locals.var_qcoth_dn6);
        locals.var_t2_dn7 = (locals.var_auxb1_dn7 + locals.var_qcoth_dn7);
        locals.var_t2_dn8 = (locals.var_auxb1_dn8 + locals.var_qcoth_dn8);
        locals.var_t2_rv = 0.0;

        let assign9030_e8752: f64 = (1.0 / locals.var_t2);
        locals.var_t3 = assign9030_e8752;
        locals.var_t3_dn3 = (-(locals.var_t2_dn3 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn4 = (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn5 = (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn6 = (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn7 = (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn8 = (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_rv = 0.0;

        let assign9040_e8755: f64 = (locals.var_xg2 - locals.var_xg1);
        let assign9040_e8757: f64 = (assign9040_e8755 + locals.var_q1);
        let assign9040_e8760: f64 = (locals.var_qsqrt * locals.var_t1);
        let assign9040_e8762: f64 = (assign9040_e8760 * locals.var_t3);
        let assign9040_e8764: f64 = (assign9040_e8762 * locals.var_t3);
        let assign9040_e8765: f64 = (assign9040_e8764).abs();
        let assign9040_e8766: f64 = (assign9040_e8765).ln();
        let assign9040_e8767: f64 = (assign9040_e8757 - assign9040_e8766);
        locals.var_q2 = assign9040_e8767;
        locals.var_q2_dn3 = (((locals.var_xg2_dn3 - locals.var_xg1_dn3) + locals.var_q1_dn3) - (if assign9040_e8764 >= 0.0 { ((((((locals.var_qsqrt_dn3 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn3)) * locals.var_t3) + (assign9040_e8760 * locals.var_t3_dn3)) * locals.var_t3) + (assign9040_e8762 * locals.var_t3_dn3)) } else { (-((((((locals.var_qsqrt_dn3 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn3)) * locals.var_t3) + (assign9040_e8760 * locals.var_t3_dn3)) * locals.var_t3) + (assign9040_e8762 * locals.var_t3_dn3))) } / assign9040_e8765));
        locals.var_q2_dn4 = (((locals.var_xg2_dn4 - locals.var_xg1_dn4) + locals.var_q1_dn4) - (if assign9040_e8764 >= 0.0 { ((((((locals.var_qsqrt_dn4 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn4)) * locals.var_t3) + (assign9040_e8760 * locals.var_t3_dn4)) * locals.var_t3) + (assign9040_e8762 * locals.var_t3_dn4)) } else { (-((((((locals.var_qsqrt_dn4 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn4)) * locals.var_t3) + (assign9040_e8760 * locals.var_t3_dn4)) * locals.var_t3) + (assign9040_e8762 * locals.var_t3_dn4))) } / assign9040_e8765));
        locals.var_q2_dn5 = (((locals.var_xg2_dn5 - locals.var_xg1_dn5) + locals.var_q1_dn5) - (if assign9040_e8764 >= 0.0 { ((((((locals.var_qsqrt_dn5 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn5)) * locals.var_t3) + (assign9040_e8760 * locals.var_t3_dn5)) * locals.var_t3) + (assign9040_e8762 * locals.var_t3_dn5)) } else { (-((((((locals.var_qsqrt_dn5 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn5)) * locals.var_t3) + (assign9040_e8760 * locals.var_t3_dn5)) * locals.var_t3) + (assign9040_e8762 * locals.var_t3_dn5))) } / assign9040_e8765));
        locals.var_q2_dn6 = (((locals.var_xg2_dn6 - locals.var_xg1_dn6) + locals.var_q1_dn6) - (if assign9040_e8764 >= 0.0 { ((((((locals.var_qsqrt_dn6 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn6)) * locals.var_t3) + (assign9040_e8760 * locals.var_t3_dn6)) * locals.var_t3) + (assign9040_e8762 * locals.var_t3_dn6)) } else { (-((((((locals.var_qsqrt_dn6 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn6)) * locals.var_t3) + (assign9040_e8760 * locals.var_t3_dn6)) * locals.var_t3) + (assign9040_e8762 * locals.var_t3_dn6))) } / assign9040_e8765));
        locals.var_q2_dn7 = (((locals.var_xg2_dn7 - locals.var_xg1_dn7) + locals.var_q1_dn7) - (if assign9040_e8764 >= 0.0 { ((((((locals.var_qsqrt_dn7 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn7)) * locals.var_t3) + (assign9040_e8760 * locals.var_t3_dn7)) * locals.var_t3) + (assign9040_e8762 * locals.var_t3_dn7)) } else { (-((((((locals.var_qsqrt_dn7 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn7)) * locals.var_t3) + (assign9040_e8760 * locals.var_t3_dn7)) * locals.var_t3) + (assign9040_e8762 * locals.var_t3_dn7))) } / assign9040_e8765));
        locals.var_q2_dn8 = (((locals.var_xg2_dn8 - locals.var_xg1_dn8) + locals.var_q1_dn8) - (if assign9040_e8764 >= 0.0 { ((((((locals.var_qsqrt_dn8 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn8)) * locals.var_t3) + (assign9040_e8760 * locals.var_t3_dn8)) * locals.var_t3) + (assign9040_e8762 * locals.var_t3_dn8)) } else { (-((((((locals.var_qsqrt_dn8 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn8)) * locals.var_t3) + (assign9040_e8760 * locals.var_t3_dn8)) * locals.var_t3) + (assign9040_e8762 * locals.var_t3_dn8))) } / assign9040_e8765));
        locals.var_q2_rv = 0.0;

        let assign9050_e8771: f64 = (locals.var_auxb1 + locals.var_qcoth);
        let assign9050_e8774: f64 = (locals.var_k2 * locals.var_q2);
        let assign9050_e8776: f64 = (assign9050_e8774 + locals.var_auxb1);
        let assign9050_e8777: f64 = (assign9050_e8771 * assign9050_e8776);
        let assign9050_e8778: f64 = (locals.var_aaux + assign9050_e8777);
        locals.var_f = assign9050_e8778;
        locals.var_f_dn3 = (locals.var_aaux_dn3 + (((locals.var_auxb1_dn3 + locals.var_qcoth_dn3) * assign9050_e8776) + (assign9050_e8771 * ((locals.var_k2 * locals.var_q2_dn3) + locals.var_auxb1_dn3))));
        locals.var_f_dn4 = (locals.var_aaux_dn4 + (((locals.var_auxb1_dn4 + locals.var_qcoth_dn4) * assign9050_e8776) + (assign9050_e8771 * ((locals.var_k2 * locals.var_q2_dn4) + locals.var_auxb1_dn4))));
        locals.var_f_dn5 = (locals.var_aaux_dn5 + (((locals.var_auxb1_dn5 + locals.var_qcoth_dn5) * assign9050_e8776) + (assign9050_e8771 * ((locals.var_k2 * locals.var_q2_dn5) + locals.var_auxb1_dn5))));
        locals.var_f_dn6 = (locals.var_aaux_dn6 + (((locals.var_auxb1_dn6 + locals.var_qcoth_dn6) * assign9050_e8776) + (assign9050_e8771 * ((locals.var_k2 * locals.var_q2_dn6) + locals.var_auxb1_dn6))));
        locals.var_f_dn7 = (locals.var_aaux_dn7 + (((locals.var_auxb1_dn7 + locals.var_qcoth_dn7) * assign9050_e8776) + (assign9050_e8771 * ((locals.var_k2 * locals.var_q2_dn7) + locals.var_auxb1_dn7))));
        locals.var_f_dn8 = (locals.var_aaux_dn8 + (((locals.var_auxb1_dn8 + locals.var_qcoth_dn8) * assign9050_e8776) + (assign9050_e8771 * ((locals.var_k2 * locals.var_q2_dn8) + locals.var_auxb1_dn8))));
        locals.var_f_rv = 0.0;

        let assign9060_e8781: f64 = (1.0 / locals.var_qsqrt);
        let assign9060_e8783: f64 = (assign9060_e8781 - locals.var_t0);
        locals.var_dlogsinhqsqdqsqrt = assign9060_e8783;
        locals.var_dlogsinhqsqdqsqrt_dn3 = ((-(locals.var_qsqrt_dn3 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn3);
        locals.var_dlogsinhqsqdqsqrt_dn4 = ((-(locals.var_qsqrt_dn4 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn4);
        locals.var_dlogsinhqsqdqsqrt_dn5 = ((-(locals.var_qsqrt_dn5 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn5);
        locals.var_dlogsinhqsqdqsqrt_dn6 = ((-(locals.var_qsqrt_dn6 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn6);
        locals.var_dlogsinhqsqdqsqrt_dn7 = ((-(locals.var_qsqrt_dn7 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn7);
        locals.var_dlogsinhqsqdqsqrt_dn8 = ((-(locals.var_qsqrt_dn8 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn8);
        locals.var_dlogsinhqsqdqsqrt_rv = 0.0;

        let assign9070_e8785: f64 = (-2.0);
        let assign9070_e8787: f64 = (assign9070_e8785 * locals.var_k1);
        let assign9070_e8789: f64 = (assign9070_e8787 * locals.var_auxb1);
        let assign9070_e8791: f64 = (assign9070_e8789 + locals.var_aaux);
        locals.var_dqsqrt = assign9070_e8791;
        locals.var_dqsqrt_dn3 = ((assign9070_e8787 * locals.var_auxb1_dn3) + locals.var_aaux_dn3);
        locals.var_dqsqrt_dn4 = ((assign9070_e8787 * locals.var_auxb1_dn4) + locals.var_aaux_dn4);
        locals.var_dqsqrt_dn5 = ((assign9070_e8787 * locals.var_auxb1_dn5) + locals.var_aaux_dn5);
        locals.var_dqsqrt_dn6 = ((assign9070_e8787 * locals.var_auxb1_dn6) + locals.var_aaux_dn6);
        locals.var_dqsqrt_dn7 = ((assign9070_e8787 * locals.var_auxb1_dn7) + locals.var_aaux_dn7);
        locals.var_dqsqrt_dn8 = ((assign9070_e8787 * locals.var_auxb1_dn8) + locals.var_aaux_dn8);
        locals.var_dqsqrt_rv = 0.0;

        let assign9080_e8794: f64 = (locals.var_dqcothqdqsqrt * locals.var_dqsqrt);
        locals.var_dqcoth = assign9080_e8794;
        locals.var_dqcoth_dn3 = ((locals.var_dqcothqdqsqrt_dn3 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn3));
        locals.var_dqcoth_dn4 = ((locals.var_dqcothqdqsqrt_dn4 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn4));
        locals.var_dqcoth_dn5 = ((locals.var_dqcothqdqsqrt_dn5 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn5));
        locals.var_dqcoth_dn6 = ((locals.var_dqcothqdqsqrt_dn6 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn6));
        locals.var_dqcoth_dn7 = ((locals.var_dqcothqdqsqrt_dn7 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn7));
        locals.var_dqcoth_dn8 = ((locals.var_dqcothqdqsqrt_dn8 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn8));
        locals.var_dqcoth_rv = 0.0;

        let assign9090_e8796: f64 = (-1.0);
        let assign9090_e8799: f64 = (-locals.var_k1);
        let assign9090_e8801: f64 = (assign9090_e8799 + locals.var_dqcoth);
        let assign9090_e8803: f64 = (assign9090_e8801 * locals.var_t3);
        let assign9090_e8804: f64 = (2.0 * assign9090_e8803);
        let assign9090_e8805: f64 = (assign9090_e8796 + assign9090_e8804);
        let assign9090_e8808: f64 = (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt);
        let assign9090_e8809: f64 = (assign9090_e8805 - assign9090_e8808);
        locals.var_dq2 = assign9090_e8809;
        locals.var_dq2_dn3 = ((2.0 * ((locals.var_dqcoth_dn3 * locals.var_t3) + (assign9090_e8801 * locals.var_t3_dn3))) - ((locals.var_dlogsinhqsqdqsqrt_dn3 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn3)));
        locals.var_dq2_dn4 = ((2.0 * ((locals.var_dqcoth_dn4 * locals.var_t3) + (assign9090_e8801 * locals.var_t3_dn4))) - ((locals.var_dlogsinhqsqdqsqrt_dn4 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn4)));
        locals.var_dq2_dn5 = ((2.0 * ((locals.var_dqcoth_dn5 * locals.var_t3) + (assign9090_e8801 * locals.var_t3_dn5))) - ((locals.var_dlogsinhqsqdqsqrt_dn5 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn5)));
        locals.var_dq2_dn6 = ((2.0 * ((locals.var_dqcoth_dn6 * locals.var_t3) + (assign9090_e8801 * locals.var_t3_dn6))) - ((locals.var_dlogsinhqsqdqsqrt_dn6 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn6)));
        locals.var_dq2_dn7 = ((2.0 * ((locals.var_dqcoth_dn7 * locals.var_t3) + (assign9090_e8801 * locals.var_t3_dn7))) - ((locals.var_dlogsinhqsqdqsqrt_dn7 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn7)));
        locals.var_dq2_dn8 = ((2.0 * ((locals.var_dqcoth_dn8 * locals.var_t3) + (assign9090_e8801 * locals.var_t3_dn8))) - ((locals.var_dlogsinhqsqdqsqrt_dn8 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn8)));
        locals.var_dq2_rv = 0.0;

        let assign9100_e8814: f64 = (locals.var_auxb1 + locals.var_t2);
        let assign9100_e8815: f64 = (locals.var_k1 * assign9100_e8814);
        let assign9100_e8816: f64 = (locals.var_aaux - assign9100_e8815);
        let assign9100_e8819: f64 = (locals.var_auxb1 * locals.var_dqcoth);
        let assign9100_e8820: f64 = (assign9100_e8816 + assign9100_e8819);
        let assign9100_e8824: f64 = (locals.var_dq2 * locals.var_t2);
        let assign9100_e8828: f64 = (locals.var_dqcoth - locals.var_k1);
        let assign9100_e8829: f64 = (locals.var_q2 * assign9100_e8828);
        let assign9100_e8830: f64 = (assign9100_e8824 + assign9100_e8829);
        let assign9100_e8831: f64 = (locals.var_k2 * assign9100_e8830);
        let assign9100_e8832: f64 = (assign9100_e8820 + assign9100_e8831);
        locals.var_df = assign9100_e8832;
        locals.var_df_dn3 = (((locals.var_aaux_dn3 - (locals.var_k1 * (locals.var_auxb1_dn3 + locals.var_t2_dn3))) + ((locals.var_auxb1_dn3 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn3))) + (locals.var_k2 * (((locals.var_dq2_dn3 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn3)) + ((locals.var_q2_dn3 * assign9100_e8828) + (locals.var_q2 * locals.var_dqcoth_dn3)))));
        locals.var_df_dn4 = (((locals.var_aaux_dn4 - (locals.var_k1 * (locals.var_auxb1_dn4 + locals.var_t2_dn4))) + ((locals.var_auxb1_dn4 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn4))) + (locals.var_k2 * (((locals.var_dq2_dn4 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn4)) + ((locals.var_q2_dn4 * assign9100_e8828) + (locals.var_q2 * locals.var_dqcoth_dn4)))));
        locals.var_df_dn5 = (((locals.var_aaux_dn5 - (locals.var_k1 * (locals.var_auxb1_dn5 + locals.var_t2_dn5))) + ((locals.var_auxb1_dn5 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn5))) + (locals.var_k2 * (((locals.var_dq2_dn5 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn5)) + ((locals.var_q2_dn5 * assign9100_e8828) + (locals.var_q2 * locals.var_dqcoth_dn5)))));
        locals.var_df_dn6 = (((locals.var_aaux_dn6 - (locals.var_k1 * (locals.var_auxb1_dn6 + locals.var_t2_dn6))) + ((locals.var_auxb1_dn6 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn6))) + (locals.var_k2 * (((locals.var_dq2_dn6 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn6)) + ((locals.var_q2_dn6 * assign9100_e8828) + (locals.var_q2 * locals.var_dqcoth_dn6)))));
        locals.var_df_dn7 = (((locals.var_aaux_dn7 - (locals.var_k1 * (locals.var_auxb1_dn7 + locals.var_t2_dn7))) + ((locals.var_auxb1_dn7 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn7))) + (locals.var_k2 * (((locals.var_dq2_dn7 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn7)) + ((locals.var_q2_dn7 * assign9100_e8828) + (locals.var_q2 * locals.var_dqcoth_dn7)))));
        locals.var_df_dn8 = (((locals.var_aaux_dn8 - (locals.var_k1 * (locals.var_auxb1_dn8 + locals.var_t2_dn8))) + ((locals.var_auxb1_dn8 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn8))) + (locals.var_k2 * (((locals.var_dq2_dn8 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn8)) + ((locals.var_q2_dn8 * assign9100_e8828) + (locals.var_q2 * locals.var_dqcoth_dn8)))));
        locals.var_df_rv = 0.0;

        let assign9110_e8834: f64 = (-locals.var_f);
        let assign9110_e8836: f64 = (assign9110_e8834 / locals.var_df);
        locals.var_delta = assign9110_e8836;
        locals.var_delta_dn3 = ((((-locals.var_f_dn3) * locals.var_df) - (assign9110_e8834 * locals.var_df_dn3)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn4 = ((((-locals.var_f_dn4) * locals.var_df) - (assign9110_e8834 * locals.var_df_dn4)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn5 = ((((-locals.var_f_dn5) * locals.var_df) - (assign9110_e8834 * locals.var_df_dn5)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn6 = ((((-locals.var_f_dn6) * locals.var_df) - (assign9110_e8834 * locals.var_df_dn6)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn7 = ((((-locals.var_f_dn7) * locals.var_df) - (assign9110_e8834 * locals.var_df_dn7)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn8 = ((((-locals.var_f_dn8) * locals.var_df) - (assign9110_e8834 * locals.var_df_dn8)) / (locals.var_df * locals.var_df));
        locals.var_delta_rv = 0.0;

        let assign9120_e8839: f64 = (locals.var_phi1 + locals.var_delta);
        locals.var_phi1 = assign9120_e8839;
        locals.var_phi1_dn3 = (locals.var_phi1_dn3 + locals.var_delta_dn3);
        locals.var_phi1_dn4 = (locals.var_phi1_dn4 + locals.var_delta_dn4);
        locals.var_phi1_dn5 = (locals.var_phi1_dn5 + locals.var_delta_dn5);
        locals.var_phi1_dn6 = (locals.var_phi1_dn6 + locals.var_delta_dn6);
        locals.var_phi1_dn7 = (locals.var_phi1_dn7 + locals.var_delta_dn7);
        locals.var_phi1_dn8 = (locals.var_phi1_dn8 + locals.var_delta_dn8);
        locals.var_phi1_rv = 0.0;

        let assign9130_e8842: f64 = (locals.var_xg1 - locals.var_phi1);
        locals.var_q1 = assign9130_e8842;
        locals.var_q1_dn3 = (locals.var_xg1_dn3 - locals.var_phi1_dn3);
        locals.var_q1_dn4 = (locals.var_xg1_dn4 - locals.var_phi1_dn4);
        locals.var_q1_dn5 = (locals.var_xg1_dn5 - locals.var_phi1_dn5);
        locals.var_q1_dn6 = (locals.var_xg1_dn6 - locals.var_phi1_dn6);
        locals.var_q1_dn7 = (locals.var_xg1_dn7 - locals.var_phi1_dn7);
        locals.var_q1_dn8 = (locals.var_xg1_dn8 - locals.var_phi1_dn8);
        locals.var_q1_rv = 0.0;

        let assign9140_e8845: f64 = (locals.var_k1 * locals.var_q1);
        locals.var_auxb1 = assign9140_e8845;
        locals.var_auxb1_dn3 = (locals.var_k1 * locals.var_q1_dn3);
        locals.var_auxb1_dn4 = (locals.var_k1 * locals.var_q1_dn4);
        locals.var_auxb1_dn5 = (locals.var_k1 * locals.var_q1_dn5);
        locals.var_auxb1_dn6 = (locals.var_k1 * locals.var_q1_dn6);
        locals.var_auxb1_dn7 = (locals.var_k1 * locals.var_q1_dn7);
        locals.var_auxb1_dn8 = (locals.var_k1 * locals.var_q1_dn8);
        locals.var_auxb1_rv = 0.0;

        let assign9150_e8847: f64 = (-locals.var_a0);
        let assign9150_e8849: f64 = (locals.var_phi1).exp();
        let assign9150_e8850: f64 = (assign9150_e8847 * assign9150_e8849);
        locals.var_aaux = assign9150_e8850;
        locals.var_aaux_dn3 = (((-locals.var_a0_dn3) * assign9150_e8849) + (assign9150_e8847 * (assign9150_e8849 * locals.var_phi1_dn3)));
        locals.var_aaux_dn4 = (((-locals.var_a0_dn4) * assign9150_e8849) + (assign9150_e8847 * (assign9150_e8849 * locals.var_phi1_dn4)));
        locals.var_aaux_dn5 = (((-locals.var_a0_dn5) * assign9150_e8849) + (assign9150_e8847 * (assign9150_e8849 * locals.var_phi1_dn5)));
        locals.var_aaux_dn6 = (((-locals.var_a0_dn6) * assign9150_e8849) + (assign9150_e8847 * (assign9150_e8849 * locals.var_phi1_dn6)));
        locals.var_aaux_dn7 = (((-locals.var_a0_dn7) * assign9150_e8849) + (assign9150_e8847 * (assign9150_e8849 * locals.var_phi1_dn7)));
        locals.var_aaux_dn8 = (((-locals.var_a0_dn8) * assign9150_e8849) + (assign9150_e8847 * (assign9150_e8849 * locals.var_phi1_dn8)));
        locals.var_aaux_rv = 0.0;

        let assign9160_e8853: f64 = (locals.var_auxb1 * locals.var_auxb1);
        let assign9160_e8855: f64 = (assign9160_e8853 + locals.var_aaux);
        locals.var_qsqrt = assign9160_e8855;
        locals.var_qsqrt_dn3 = (((locals.var_auxb1_dn3 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn3)) + locals.var_aaux_dn3);
        locals.var_qsqrt_dn4 = (((locals.var_auxb1_dn4 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn4)) + locals.var_aaux_dn4);
        locals.var_qsqrt_dn5 = (((locals.var_auxb1_dn5 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn5)) + locals.var_aaux_dn5);
        locals.var_qsqrt_dn6 = (((locals.var_auxb1_dn6 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn6)) + locals.var_aaux_dn6);
        locals.var_qsqrt_dn7 = (((locals.var_auxb1_dn7 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn7)) + locals.var_aaux_dn7);
        locals.var_qsqrt_dn8 = (((locals.var_auxb1_dn8 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn8)) + locals.var_aaux_dn8);
        locals.var_qsqrt_rv = 0.0;

        let assign9170_e8858: f64 = if locals.var_qsqrt < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard94 = assign9170_e8858;
        locals.var_guard94_rv = 0.0;

        let (assign9180_e8864, assign9180_e8864_d_n3, assign9180_e8864_d_n4, assign9180_e8864_d_n5, assign9180_e8864_d_n6, assign9180_e8864_d_n7, assign9180_e8864_d_n8,) = {
    if (locals.var_guard94 != 0.0) {
        let assign9180_e8861: f64 = (-locals.var_qsqrt);
        let assign9180_e8862: f64 = (assign9180_e8861).sqrt();
        (assign9180_e8862, ((-locals.var_qsqrt_dn3) / (2.0 * assign9180_e8862)), ((-locals.var_qsqrt_dn4) / (2.0 * assign9180_e8862)), ((-locals.var_qsqrt_dn5) / (2.0 * assign9180_e8862)), ((-locals.var_qsqrt_dn6) / (2.0 * assign9180_e8862)), ((-locals.var_qsqrt_dn7) / (2.0 * assign9180_e8862)), ((-locals.var_qsqrt_dn8) / (2.0 * assign9180_e8862)),)
    } else {
        (locals.var_q, locals.var_q_dn3, locals.var_q_dn4, locals.var_q_dn5, locals.var_q_dn6, locals.var_q_dn7, locals.var_q_dn8,)
    }
};
        locals.var_q = assign9180_e8864;
        locals.var_q_dn3 = assign9180_e8864_d_n3;
        locals.var_q_dn4 = assign9180_e8864_d_n4;
        locals.var_q_dn5 = assign9180_e8864_d_n5;
        locals.var_q_dn6 = assign9180_e8864_d_n6;
        locals.var_q_dn7 = assign9180_e8864_d_n7;
        locals.var_q_dn8 = assign9180_e8864_d_n8;
        locals.var_q_rv = 0.0;

        let (assign9190_e8873, assign9190_e8873_d_n3, assign9190_e8873_d_n4, assign9190_e8873_d_n5, assign9190_e8873_d_n6, assign9190_e8873_d_n7, assign9190_e8873_d_n8,) = {
    if (locals.var_guard94 != 0.0) {
        let assign9190_e8869: f64 = (0.5 * locals.var_q);
        let assign9190_e8870: f64 = (assign9190_e8869).sin();
        let assign9190_e8871: f64 = (1.0 / assign9190_e8870);
        (assign9190_e8871, (-(((assign9190_e8869).cos() * (0.5 * locals.var_q_dn3)) / (assign9190_e8870 * assign9190_e8870))), (-(((assign9190_e8869).cos() * (0.5 * locals.var_q_dn4)) / (assign9190_e8870 * assign9190_e8870))), (-(((assign9190_e8869).cos() * (0.5 * locals.var_q_dn5)) / (assign9190_e8870 * assign9190_e8870))), (-(((assign9190_e8869).cos() * (0.5 * locals.var_q_dn6)) / (assign9190_e8870 * assign9190_e8870))), (-(((assign9190_e8869).cos() * (0.5 * locals.var_q_dn7)) / (assign9190_e8870 * assign9190_e8870))), (-(((assign9190_e8869).cos() * (0.5 * locals.var_q_dn8)) / (assign9190_e8870 * assign9190_e8870))),)
    } else {
        (locals.var_csc1, locals.var_csc1_dn3, locals.var_csc1_dn4, locals.var_csc1_dn5, locals.var_csc1_dn6, locals.var_csc1_dn7, locals.var_csc1_dn8,)
    }
};
        locals.var_csc1 = assign9190_e8873;
        locals.var_csc1_dn3 = assign9190_e8873_d_n3;
        locals.var_csc1_dn4 = assign9190_e8873_d_n4;
        locals.var_csc1_dn5 = assign9190_e8873_d_n5;
        locals.var_csc1_dn6 = assign9190_e8873_d_n6;
        locals.var_csc1_dn7 = assign9190_e8873_d_n7;
        locals.var_csc1_dn8 = assign9190_e8873_d_n8;
        locals.var_csc1_rv = 0.0;

        let (assign9200_e8879, assign9200_e8879_d_n3, assign9200_e8879_d_n4, assign9200_e8879_d_n5, assign9200_e8879_d_n6, assign9200_e8879_d_n7, assign9200_e8879_d_n8,) = {
    if (locals.var_guard94 != 0.0) {
        let assign9200_e8877: f64 = (locals.var_csc1 * locals.var_csc1);
        (assign9200_e8877, ((locals.var_csc1_dn3 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn3)), ((locals.var_csc1_dn4 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn4)), ((locals.var_csc1_dn5 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn5)), ((locals.var_csc1_dn6 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn6)), ((locals.var_csc1_dn7 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn7)), ((locals.var_csc1_dn8 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn8)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign9200_e8879;
        locals.var_t1_dn3 = assign9200_e8879_d_n3;
        locals.var_t1_dn4 = assign9200_e8879_d_n4;
        locals.var_t1_dn5 = assign9200_e8879_d_n5;
        locals.var_t1_dn6 = assign9200_e8879_d_n6;
        locals.var_t1_dn7 = assign9200_e8879_d_n7;
        locals.var_t1_dn8 = assign9200_e8879_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign9210_e8888, assign9210_e8888_d_n3, assign9210_e8888_d_n4, assign9210_e8888_d_n5, assign9210_e8888_d_n6, assign9210_e8888_d_n7, assign9210_e8888_d_n8,) = {
    if (locals.var_guard94 != 0.0) {
        let assign9210_e8883: f64 = (0.5 * locals.var_q);
        let assign9210_e8884: f64 = (assign9210_e8883).cos();
        let assign9210_e8886: f64 = (assign9210_e8884 * locals.var_csc1);
        (assign9210_e8886, (((-(assign9210_e8883).sin() * (0.5 * locals.var_q_dn3)) * locals.var_csc1) + (assign9210_e8884 * locals.var_csc1_dn3)), (((-(assign9210_e8883).sin() * (0.5 * locals.var_q_dn4)) * locals.var_csc1) + (assign9210_e8884 * locals.var_csc1_dn4)), (((-(assign9210_e8883).sin() * (0.5 * locals.var_q_dn5)) * locals.var_csc1) + (assign9210_e8884 * locals.var_csc1_dn5)), (((-(assign9210_e8883).sin() * (0.5 * locals.var_q_dn6)) * locals.var_csc1) + (assign9210_e8884 * locals.var_csc1_dn6)), (((-(assign9210_e8883).sin() * (0.5 * locals.var_q_dn7)) * locals.var_csc1) + (assign9210_e8884 * locals.var_csc1_dn7)), (((-(assign9210_e8883).sin() * (0.5 * locals.var_q_dn8)) * locals.var_csc1) + (assign9210_e8884 * locals.var_csc1_dn8)),)
    } else {
        (locals.var_coth1, locals.var_coth1_dn3, locals.var_coth1_dn4, locals.var_coth1_dn5, locals.var_coth1_dn6, locals.var_coth1_dn7, locals.var_coth1_dn8,)
    }
};
        locals.var_coth1 = assign9210_e8888;
        locals.var_coth1_dn3 = assign9210_e8888_d_n3;
        locals.var_coth1_dn4 = assign9210_e8888_d_n4;
        locals.var_coth1_dn5 = assign9210_e8888_d_n5;
        locals.var_coth1_dn6 = assign9210_e8888_d_n6;
        locals.var_coth1_dn7 = assign9210_e8888_d_n7;
        locals.var_coth1_dn8 = assign9210_e8888_d_n8;
        locals.var_coth1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_20(
        locals: &mut StampLocals,
    ) {
        let (assign9220_e8897, assign9220_e8897_d_n3, assign9220_e8897_d_n4, assign9220_e8897_d_n5, assign9220_e8897_d_n6, assign9220_e8897_d_n7, assign9220_e8897_d_n8,) = {
    if (locals.var_guard94 != 0.0) {
        let assign9220_e8891: f64 = (-0.5);
        let assign9220_e8893: f64 = (assign9220_e8891 * locals.var_coth1);
        let assign9220_e8895: f64 = (assign9220_e8893 / locals.var_q);
        (assign9220_e8895, ((((assign9220_e8891 * locals.var_coth1_dn3) * locals.var_q) - (assign9220_e8893 * locals.var_q_dn3)) / (locals.var_q * locals.var_q)), ((((assign9220_e8891 * locals.var_coth1_dn4) * locals.var_q) - (assign9220_e8893 * locals.var_q_dn4)) / (locals.var_q * locals.var_q)), ((((assign9220_e8891 * locals.var_coth1_dn5) * locals.var_q) - (assign9220_e8893 * locals.var_q_dn5)) / (locals.var_q * locals.var_q)), ((((assign9220_e8891 * locals.var_coth1_dn6) * locals.var_q) - (assign9220_e8893 * locals.var_q_dn6)) / (locals.var_q * locals.var_q)), ((((assign9220_e8891 * locals.var_coth1_dn7) * locals.var_q) - (assign9220_e8893 * locals.var_q_dn7)) / (locals.var_q * locals.var_q)), ((((assign9220_e8891 * locals.var_coth1_dn8) * locals.var_q) - (assign9220_e8893 * locals.var_q_dn8)) / (locals.var_q * locals.var_q)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    }
};
        locals.var_t0 = assign9220_e8897;
        locals.var_t0_dn3 = assign9220_e8897_d_n3;
        locals.var_t0_dn4 = assign9220_e8897_d_n4;
        locals.var_t0_dn5 = assign9220_e8897_d_n5;
        locals.var_t0_dn6 = assign9220_e8897_d_n6;
        locals.var_t0_dn7 = assign9220_e8897_d_n7;
        locals.var_t0_dn8 = assign9220_e8897_d_n8;
        locals.var_t0_rv = 0.0;

        let (assign9230_e8905, assign9230_e8905_d_n3, assign9230_e8905_d_n4, assign9230_e8905_d_n5, assign9230_e8905_d_n6, assign9230_e8905_d_n7, assign9230_e8905_d_n8,) = {
    if (locals.var_guard94 != 0.0) {
        let assign9230_e8901: f64 = (0.25 * locals.var_t1);
        let assign9230_e8903: f64 = (assign9230_e8901 + locals.var_t0);
        (assign9230_e8903, ((0.25 * locals.var_t1_dn3) + locals.var_t0_dn3), ((0.25 * locals.var_t1_dn4) + locals.var_t0_dn4), ((0.25 * locals.var_t1_dn5) + locals.var_t0_dn5), ((0.25 * locals.var_t1_dn6) + locals.var_t0_dn6), ((0.25 * locals.var_t1_dn7) + locals.var_t0_dn7), ((0.25 * locals.var_t1_dn8) + locals.var_t0_dn8),)
    } else {
        (locals.var_dqcothqdqsqrt, locals.var_dqcothqdqsqrt_dn3, locals.var_dqcothqdqsqrt_dn4, locals.var_dqcothqdqsqrt_dn5, locals.var_dqcothqdqsqrt_dn6, locals.var_dqcothqdqsqrt_dn7, locals.var_dqcothqdqsqrt_dn8,)
    }
};
        locals.var_dqcothqdqsqrt = assign9230_e8905;
        locals.var_dqcothqdqsqrt_dn3 = assign9230_e8905_d_n3;
        locals.var_dqcothqdqsqrt_dn4 = assign9230_e8905_d_n4;
        locals.var_dqcothqdqsqrt_dn5 = assign9230_e8905_d_n5;
        locals.var_dqcothqdqsqrt_dn6 = assign9230_e8905_d_n6;
        locals.var_dqcothqdqsqrt_dn7 = assign9230_e8905_d_n7;
        locals.var_dqcothqdqsqrt_dn8 = assign9230_e8905_d_n8;
        locals.var_dqcothqdqsqrt_rv = 0.0;

        let (assign9240_e8911, assign9240_e8911_d_n3, assign9240_e8911_d_n4, assign9240_e8911_d_n5, assign9240_e8911_d_n6, assign9240_e8911_d_n7, assign9240_e8911_d_n8,) = {
    if (locals.var_guard94 == 0.0) {
        let assign9240_e8909: f64 = (locals.var_qsqrt).sqrt();
        (assign9240_e8909, (locals.var_qsqrt_dn3 / (2.0 * assign9240_e8909)), (locals.var_qsqrt_dn4 / (2.0 * assign9240_e8909)), (locals.var_qsqrt_dn5 / (2.0 * assign9240_e8909)), (locals.var_qsqrt_dn6 / (2.0 * assign9240_e8909)), (locals.var_qsqrt_dn7 / (2.0 * assign9240_e8909)), (locals.var_qsqrt_dn8 / (2.0 * assign9240_e8909)),)
    } else {
        (locals.var_q, locals.var_q_dn3, locals.var_q_dn4, locals.var_q_dn5, locals.var_q_dn6, locals.var_q_dn7, locals.var_q_dn8,)
    }
};
        locals.var_q = assign9240_e8911;
        locals.var_q_dn3 = assign9240_e8911_d_n3;
        locals.var_q_dn4 = assign9240_e8911_d_n4;
        locals.var_q_dn5 = assign9240_e8911_d_n5;
        locals.var_q_dn6 = assign9240_e8911_d_n6;
        locals.var_q_dn7 = assign9240_e8911_d_n7;
        locals.var_q_dn8 = assign9240_e8911_d_n8;
        locals.var_q_rv = 0.0;

        let (assign9250_e8921, assign9250_e8921_d_n3, assign9250_e8921_d_n4, assign9250_e8921_d_n5, assign9250_e8921_d_n6, assign9250_e8921_d_n7, assign9250_e8921_d_n8,) = {
    if (locals.var_guard94 == 0.0) {
        let assign9250_e8917: f64 = (0.5 * locals.var_q);
        let assign9250_e8918: f64 = (assign9250_e8917).sinh();
        let assign9250_e8919: f64 = (1.0 / assign9250_e8918);
        (assign9250_e8919, (-(((assign9250_e8917).cosh() * (0.5 * locals.var_q_dn3)) / (assign9250_e8918 * assign9250_e8918))), (-(((assign9250_e8917).cosh() * (0.5 * locals.var_q_dn4)) / (assign9250_e8918 * assign9250_e8918))), (-(((assign9250_e8917).cosh() * (0.5 * locals.var_q_dn5)) / (assign9250_e8918 * assign9250_e8918))), (-(((assign9250_e8917).cosh() * (0.5 * locals.var_q_dn6)) / (assign9250_e8918 * assign9250_e8918))), (-(((assign9250_e8917).cosh() * (0.5 * locals.var_q_dn7)) / (assign9250_e8918 * assign9250_e8918))), (-(((assign9250_e8917).cosh() * (0.5 * locals.var_q_dn8)) / (assign9250_e8918 * assign9250_e8918))),)
    } else {
        (locals.var_csc1, locals.var_csc1_dn3, locals.var_csc1_dn4, locals.var_csc1_dn5, locals.var_csc1_dn6, locals.var_csc1_dn7, locals.var_csc1_dn8,)
    }
};
        locals.var_csc1 = assign9250_e8921;
        locals.var_csc1_dn3 = assign9250_e8921_d_n3;
        locals.var_csc1_dn4 = assign9250_e8921_d_n4;
        locals.var_csc1_dn5 = assign9250_e8921_d_n5;
        locals.var_csc1_dn6 = assign9250_e8921_d_n6;
        locals.var_csc1_dn7 = assign9250_e8921_d_n7;
        locals.var_csc1_dn8 = assign9250_e8921_d_n8;
        locals.var_csc1_rv = 0.0;

        let (assign9260_e8928, assign9260_e8928_d_n3, assign9260_e8928_d_n4, assign9260_e8928_d_n5, assign9260_e8928_d_n6, assign9260_e8928_d_n7, assign9260_e8928_d_n8,) = {
    if (locals.var_guard94 == 0.0) {
        let assign9260_e8926: f64 = (locals.var_csc1 * locals.var_csc1);
        (assign9260_e8926, ((locals.var_csc1_dn3 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn3)), ((locals.var_csc1_dn4 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn4)), ((locals.var_csc1_dn5 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn5)), ((locals.var_csc1_dn6 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn6)), ((locals.var_csc1_dn7 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn7)), ((locals.var_csc1_dn8 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn8)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign9260_e8928;
        locals.var_t1_dn3 = assign9260_e8928_d_n3;
        locals.var_t1_dn4 = assign9260_e8928_d_n4;
        locals.var_t1_dn5 = assign9260_e8928_d_n5;
        locals.var_t1_dn6 = assign9260_e8928_d_n6;
        locals.var_t1_dn7 = assign9260_e8928_d_n7;
        locals.var_t1_dn8 = assign9260_e8928_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign9270_e8936, assign9270_e8936_d_n3, assign9270_e8936_d_n4, assign9270_e8936_d_n5, assign9270_e8936_d_n6, assign9270_e8936_d_n7, assign9270_e8936_d_n8,) = {
    if (locals.var_guard94 == 0.0) {
        let assign9270_e8933: f64 = (1.0 + locals.var_t1);
        let assign9270_e8934: f64 = (assign9270_e8933).sqrt();
        (assign9270_e8934, (locals.var_t1_dn3 / (2.0 * assign9270_e8934)), (locals.var_t1_dn4 / (2.0 * assign9270_e8934)), (locals.var_t1_dn5 / (2.0 * assign9270_e8934)), (locals.var_t1_dn6 / (2.0 * assign9270_e8934)), (locals.var_t1_dn7 / (2.0 * assign9270_e8934)), (locals.var_t1_dn8 / (2.0 * assign9270_e8934)),)
    } else {
        (locals.var_coth1, locals.var_coth1_dn3, locals.var_coth1_dn4, locals.var_coth1_dn5, locals.var_coth1_dn6, locals.var_coth1_dn7, locals.var_coth1_dn8,)
    }
};
        locals.var_coth1 = assign9270_e8936;
        locals.var_coth1_dn3 = assign9270_e8936_d_n3;
        locals.var_coth1_dn4 = assign9270_e8936_d_n4;
        locals.var_coth1_dn5 = assign9270_e8936_d_n5;
        locals.var_coth1_dn6 = assign9270_e8936_d_n6;
        locals.var_coth1_dn7 = assign9270_e8936_d_n7;
        locals.var_coth1_dn8 = assign9270_e8936_d_n8;
        locals.var_coth1_rv = 0.0;

        let (assign9280_e8945, assign9280_e8945_d_n3, assign9280_e8945_d_n4, assign9280_e8945_d_n5, assign9280_e8945_d_n6, assign9280_e8945_d_n7, assign9280_e8945_d_n8,) = {
    if (locals.var_guard94 == 0.0) {
        let assign9280_e8941: f64 = (0.5 * locals.var_coth1);
        let assign9280_e8943: f64 = (assign9280_e8941 / locals.var_q);
        (assign9280_e8943, ((((0.5 * locals.var_coth1_dn3) * locals.var_q) - (assign9280_e8941 * locals.var_q_dn3)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn4) * locals.var_q) - (assign9280_e8941 * locals.var_q_dn4)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn5) * locals.var_q) - (assign9280_e8941 * locals.var_q_dn5)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn6) * locals.var_q) - (assign9280_e8941 * locals.var_q_dn6)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn7) * locals.var_q) - (assign9280_e8941 * locals.var_q_dn7)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn8) * locals.var_q) - (assign9280_e8941 * locals.var_q_dn8)) / (locals.var_q * locals.var_q)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    }
};
        locals.var_t0 = assign9280_e8945;
        locals.var_t0_dn3 = assign9280_e8945_d_n3;
        locals.var_t0_dn4 = assign9280_e8945_d_n4;
        locals.var_t0_dn5 = assign9280_e8945_d_n5;
        locals.var_t0_dn6 = assign9280_e8945_d_n6;
        locals.var_t0_dn7 = assign9280_e8945_d_n7;
        locals.var_t0_dn8 = assign9280_e8945_d_n8;
        locals.var_t0_rv = 0.0;

        let (assign9290_e8955, assign9290_e8955_d_n3, assign9290_e8955_d_n4, assign9290_e8955_d_n5, assign9290_e8955_d_n6, assign9290_e8955_d_n7, assign9290_e8955_d_n8,) = {
    if (locals.var_guard94 == 0.0) {
        let assign9290_e8949: f64 = (-0.25);
        let assign9290_e8951: f64 = (assign9290_e8949 * locals.var_t1);
        let assign9290_e8953: f64 = (assign9290_e8951 + locals.var_t0);
        (assign9290_e8953, ((assign9290_e8949 * locals.var_t1_dn3) + locals.var_t0_dn3), ((assign9290_e8949 * locals.var_t1_dn4) + locals.var_t0_dn4), ((assign9290_e8949 * locals.var_t1_dn5) + locals.var_t0_dn5), ((assign9290_e8949 * locals.var_t1_dn6) + locals.var_t0_dn6), ((assign9290_e8949 * locals.var_t1_dn7) + locals.var_t0_dn7), ((assign9290_e8949 * locals.var_t1_dn8) + locals.var_t0_dn8),)
    } else {
        (locals.var_dqcothqdqsqrt, locals.var_dqcothqdqsqrt_dn3, locals.var_dqcothqdqsqrt_dn4, locals.var_dqcothqdqsqrt_dn5, locals.var_dqcothqdqsqrt_dn6, locals.var_dqcothqdqsqrt_dn7, locals.var_dqcothqdqsqrt_dn8,)
    }
};
        locals.var_dqcothqdqsqrt = assign9290_e8955;
        locals.var_dqcothqdqsqrt_dn3 = assign9290_e8955_d_n3;
        locals.var_dqcothqdqsqrt_dn4 = assign9290_e8955_d_n4;
        locals.var_dqcothqdqsqrt_dn5 = assign9290_e8955_d_n5;
        locals.var_dqcothqdqsqrt_dn6 = assign9290_e8955_d_n6;
        locals.var_dqcothqdqsqrt_dn7 = assign9290_e8955_d_n7;
        locals.var_dqcothqdqsqrt_dn8 = assign9290_e8955_d_n8;
        locals.var_dqcothqdqsqrt_rv = 0.0;

        let assign9300_e8958: f64 = (locals.var_q * locals.var_coth1);
        locals.var_qcoth = assign9300_e8958;
        locals.var_qcoth_dn3 = ((locals.var_q_dn3 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn3));
        locals.var_qcoth_dn4 = ((locals.var_q_dn4 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn4));
        locals.var_qcoth_dn5 = ((locals.var_q_dn5 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn5));
        locals.var_qcoth_dn6 = ((locals.var_q_dn6 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn6));
        locals.var_qcoth_dn7 = ((locals.var_q_dn7 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn7));
        locals.var_qcoth_dn8 = ((locals.var_q_dn8 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn8));
        locals.var_qcoth_rv = 0.0;

        let assign9310_e8961: f64 = (locals.var_auxb1 + locals.var_qcoth);
        locals.var_t2 = assign9310_e8961;
        locals.var_t2_dn3 = (locals.var_auxb1_dn3 + locals.var_qcoth_dn3);
        locals.var_t2_dn4 = (locals.var_auxb1_dn4 + locals.var_qcoth_dn4);
        locals.var_t2_dn5 = (locals.var_auxb1_dn5 + locals.var_qcoth_dn5);
        locals.var_t2_dn6 = (locals.var_auxb1_dn6 + locals.var_qcoth_dn6);
        locals.var_t2_dn7 = (locals.var_auxb1_dn7 + locals.var_qcoth_dn7);
        locals.var_t2_dn8 = (locals.var_auxb1_dn8 + locals.var_qcoth_dn8);
        locals.var_t2_rv = 0.0;

        let assign9320_e8964: f64 = (1.0 / locals.var_t2);
        locals.var_t3 = assign9320_e8964;
        locals.var_t3_dn3 = (-(locals.var_t2_dn3 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn4 = (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn5 = (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn6 = (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn7 = (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn8 = (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_rv = 0.0;

        let assign9330_e8967: f64 = (locals.var_xg2 - locals.var_xg1);
        let assign9330_e8969: f64 = (assign9330_e8967 + locals.var_q1);
        let assign9330_e8972: f64 = (locals.var_qsqrt * locals.var_t1);
        let assign9330_e8974: f64 = (assign9330_e8972 * locals.var_t3);
        let assign9330_e8976: f64 = (assign9330_e8974 * locals.var_t3);
        let assign9330_e8977: f64 = (assign9330_e8976).abs();
        let assign9330_e8978: f64 = (assign9330_e8977).ln();
        let assign9330_e8979: f64 = (assign9330_e8969 - assign9330_e8978);
        locals.var_q2 = assign9330_e8979;
        locals.var_q2_dn3 = (((locals.var_xg2_dn3 - locals.var_xg1_dn3) + locals.var_q1_dn3) - (if assign9330_e8976 >= 0.0 { ((((((locals.var_qsqrt_dn3 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn3)) * locals.var_t3) + (assign9330_e8972 * locals.var_t3_dn3)) * locals.var_t3) + (assign9330_e8974 * locals.var_t3_dn3)) } else { (-((((((locals.var_qsqrt_dn3 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn3)) * locals.var_t3) + (assign9330_e8972 * locals.var_t3_dn3)) * locals.var_t3) + (assign9330_e8974 * locals.var_t3_dn3))) } / assign9330_e8977));
        locals.var_q2_dn4 = (((locals.var_xg2_dn4 - locals.var_xg1_dn4) + locals.var_q1_dn4) - (if assign9330_e8976 >= 0.0 { ((((((locals.var_qsqrt_dn4 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn4)) * locals.var_t3) + (assign9330_e8972 * locals.var_t3_dn4)) * locals.var_t3) + (assign9330_e8974 * locals.var_t3_dn4)) } else { (-((((((locals.var_qsqrt_dn4 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn4)) * locals.var_t3) + (assign9330_e8972 * locals.var_t3_dn4)) * locals.var_t3) + (assign9330_e8974 * locals.var_t3_dn4))) } / assign9330_e8977));
        locals.var_q2_dn5 = (((locals.var_xg2_dn5 - locals.var_xg1_dn5) + locals.var_q1_dn5) - (if assign9330_e8976 >= 0.0 { ((((((locals.var_qsqrt_dn5 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn5)) * locals.var_t3) + (assign9330_e8972 * locals.var_t3_dn5)) * locals.var_t3) + (assign9330_e8974 * locals.var_t3_dn5)) } else { (-((((((locals.var_qsqrt_dn5 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn5)) * locals.var_t3) + (assign9330_e8972 * locals.var_t3_dn5)) * locals.var_t3) + (assign9330_e8974 * locals.var_t3_dn5))) } / assign9330_e8977));
        locals.var_q2_dn6 = (((locals.var_xg2_dn6 - locals.var_xg1_dn6) + locals.var_q1_dn6) - (if assign9330_e8976 >= 0.0 { ((((((locals.var_qsqrt_dn6 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn6)) * locals.var_t3) + (assign9330_e8972 * locals.var_t3_dn6)) * locals.var_t3) + (assign9330_e8974 * locals.var_t3_dn6)) } else { (-((((((locals.var_qsqrt_dn6 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn6)) * locals.var_t3) + (assign9330_e8972 * locals.var_t3_dn6)) * locals.var_t3) + (assign9330_e8974 * locals.var_t3_dn6))) } / assign9330_e8977));
        locals.var_q2_dn7 = (((locals.var_xg2_dn7 - locals.var_xg1_dn7) + locals.var_q1_dn7) - (if assign9330_e8976 >= 0.0 { ((((((locals.var_qsqrt_dn7 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn7)) * locals.var_t3) + (assign9330_e8972 * locals.var_t3_dn7)) * locals.var_t3) + (assign9330_e8974 * locals.var_t3_dn7)) } else { (-((((((locals.var_qsqrt_dn7 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn7)) * locals.var_t3) + (assign9330_e8972 * locals.var_t3_dn7)) * locals.var_t3) + (assign9330_e8974 * locals.var_t3_dn7))) } / assign9330_e8977));
        locals.var_q2_dn8 = (((locals.var_xg2_dn8 - locals.var_xg1_dn8) + locals.var_q1_dn8) - (if assign9330_e8976 >= 0.0 { ((((((locals.var_qsqrt_dn8 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn8)) * locals.var_t3) + (assign9330_e8972 * locals.var_t3_dn8)) * locals.var_t3) + (assign9330_e8974 * locals.var_t3_dn8)) } else { (-((((((locals.var_qsqrt_dn8 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn8)) * locals.var_t3) + (assign9330_e8972 * locals.var_t3_dn8)) * locals.var_t3) + (assign9330_e8974 * locals.var_t3_dn8))) } / assign9330_e8977));
        locals.var_q2_rv = 0.0;

        let assign9340_e8983: f64 = (locals.var_auxb1 + locals.var_qcoth);
        let assign9340_e8986: f64 = (locals.var_k2 * locals.var_q2);
        let assign9340_e8988: f64 = (assign9340_e8986 + locals.var_auxb1);
        let assign9340_e8989: f64 = (assign9340_e8983 * assign9340_e8988);
        let assign9340_e8990: f64 = (locals.var_aaux + assign9340_e8989);
        locals.var_f = assign9340_e8990;
        locals.var_f_dn3 = (locals.var_aaux_dn3 + (((locals.var_auxb1_dn3 + locals.var_qcoth_dn3) * assign9340_e8988) + (assign9340_e8983 * ((locals.var_k2 * locals.var_q2_dn3) + locals.var_auxb1_dn3))));
        locals.var_f_dn4 = (locals.var_aaux_dn4 + (((locals.var_auxb1_dn4 + locals.var_qcoth_dn4) * assign9340_e8988) + (assign9340_e8983 * ((locals.var_k2 * locals.var_q2_dn4) + locals.var_auxb1_dn4))));
        locals.var_f_dn5 = (locals.var_aaux_dn5 + (((locals.var_auxb1_dn5 + locals.var_qcoth_dn5) * assign9340_e8988) + (assign9340_e8983 * ((locals.var_k2 * locals.var_q2_dn5) + locals.var_auxb1_dn5))));
        locals.var_f_dn6 = (locals.var_aaux_dn6 + (((locals.var_auxb1_dn6 + locals.var_qcoth_dn6) * assign9340_e8988) + (assign9340_e8983 * ((locals.var_k2 * locals.var_q2_dn6) + locals.var_auxb1_dn6))));
        locals.var_f_dn7 = (locals.var_aaux_dn7 + (((locals.var_auxb1_dn7 + locals.var_qcoth_dn7) * assign9340_e8988) + (assign9340_e8983 * ((locals.var_k2 * locals.var_q2_dn7) + locals.var_auxb1_dn7))));
        locals.var_f_dn8 = (locals.var_aaux_dn8 + (((locals.var_auxb1_dn8 + locals.var_qcoth_dn8) * assign9340_e8988) + (assign9340_e8983 * ((locals.var_k2 * locals.var_q2_dn8) + locals.var_auxb1_dn8))));
        locals.var_f_rv = 0.0;

        let assign9350_e8993: f64 = (1.0 / locals.var_qsqrt);
        let assign9350_e8995: f64 = (assign9350_e8993 - locals.var_t0);
        locals.var_dlogsinhqsqdqsqrt = assign9350_e8995;
        locals.var_dlogsinhqsqdqsqrt_dn3 = ((-(locals.var_qsqrt_dn3 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn3);
        locals.var_dlogsinhqsqdqsqrt_dn4 = ((-(locals.var_qsqrt_dn4 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn4);
        locals.var_dlogsinhqsqdqsqrt_dn5 = ((-(locals.var_qsqrt_dn5 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn5);
        locals.var_dlogsinhqsqdqsqrt_dn6 = ((-(locals.var_qsqrt_dn6 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn6);
        locals.var_dlogsinhqsqdqsqrt_dn7 = ((-(locals.var_qsqrt_dn7 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn7);
        locals.var_dlogsinhqsqdqsqrt_dn8 = ((-(locals.var_qsqrt_dn8 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn8);
        locals.var_dlogsinhqsqdqsqrt_rv = 0.0;

        let assign9360_e8997: f64 = (-2.0);
        let assign9360_e8999: f64 = (assign9360_e8997 * locals.var_k1);
        let assign9360_e9001: f64 = (assign9360_e8999 * locals.var_auxb1);
        let assign9360_e9003: f64 = (assign9360_e9001 + locals.var_aaux);
        locals.var_dqsqrt = assign9360_e9003;
        locals.var_dqsqrt_dn3 = ((assign9360_e8999 * locals.var_auxb1_dn3) + locals.var_aaux_dn3);
        locals.var_dqsqrt_dn4 = ((assign9360_e8999 * locals.var_auxb1_dn4) + locals.var_aaux_dn4);
        locals.var_dqsqrt_dn5 = ((assign9360_e8999 * locals.var_auxb1_dn5) + locals.var_aaux_dn5);
        locals.var_dqsqrt_dn6 = ((assign9360_e8999 * locals.var_auxb1_dn6) + locals.var_aaux_dn6);
        locals.var_dqsqrt_dn7 = ((assign9360_e8999 * locals.var_auxb1_dn7) + locals.var_aaux_dn7);
        locals.var_dqsqrt_dn8 = ((assign9360_e8999 * locals.var_auxb1_dn8) + locals.var_aaux_dn8);
        locals.var_dqsqrt_rv = 0.0;

        let assign9370_e9006: f64 = (locals.var_dqcothqdqsqrt * locals.var_dqsqrt);
        locals.var_dqcoth = assign9370_e9006;
        locals.var_dqcoth_dn3 = ((locals.var_dqcothqdqsqrt_dn3 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn3));
        locals.var_dqcoth_dn4 = ((locals.var_dqcothqdqsqrt_dn4 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn4));
        locals.var_dqcoth_dn5 = ((locals.var_dqcothqdqsqrt_dn5 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn5));
        locals.var_dqcoth_dn6 = ((locals.var_dqcothqdqsqrt_dn6 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn6));
        locals.var_dqcoth_dn7 = ((locals.var_dqcothqdqsqrt_dn7 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn7));
        locals.var_dqcoth_dn8 = ((locals.var_dqcothqdqsqrt_dn8 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn8));
        locals.var_dqcoth_rv = 0.0;

        let assign9380_e9008: f64 = (-1.0);
        let assign9380_e9011: f64 = (-locals.var_k1);
        let assign9380_e9013: f64 = (assign9380_e9011 + locals.var_dqcoth);
        let assign9380_e9015: f64 = (assign9380_e9013 * locals.var_t3);
        let assign9380_e9016: f64 = (2.0 * assign9380_e9015);
        let assign9380_e9017: f64 = (assign9380_e9008 + assign9380_e9016);
        let assign9380_e9020: f64 = (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt);
        let assign9380_e9021: f64 = (assign9380_e9017 - assign9380_e9020);
        locals.var_dq2 = assign9380_e9021;
        locals.var_dq2_dn3 = ((2.0 * ((locals.var_dqcoth_dn3 * locals.var_t3) + (assign9380_e9013 * locals.var_t3_dn3))) - ((locals.var_dlogsinhqsqdqsqrt_dn3 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn3)));
        locals.var_dq2_dn4 = ((2.0 * ((locals.var_dqcoth_dn4 * locals.var_t3) + (assign9380_e9013 * locals.var_t3_dn4))) - ((locals.var_dlogsinhqsqdqsqrt_dn4 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn4)));
        locals.var_dq2_dn5 = ((2.0 * ((locals.var_dqcoth_dn5 * locals.var_t3) + (assign9380_e9013 * locals.var_t3_dn5))) - ((locals.var_dlogsinhqsqdqsqrt_dn5 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn5)));
        locals.var_dq2_dn6 = ((2.0 * ((locals.var_dqcoth_dn6 * locals.var_t3) + (assign9380_e9013 * locals.var_t3_dn6))) - ((locals.var_dlogsinhqsqdqsqrt_dn6 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn6)));
        locals.var_dq2_dn7 = ((2.0 * ((locals.var_dqcoth_dn7 * locals.var_t3) + (assign9380_e9013 * locals.var_t3_dn7))) - ((locals.var_dlogsinhqsqdqsqrt_dn7 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn7)));
        locals.var_dq2_dn8 = ((2.0 * ((locals.var_dqcoth_dn8 * locals.var_t3) + (assign9380_e9013 * locals.var_t3_dn8))) - ((locals.var_dlogsinhqsqdqsqrt_dn8 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn8)));
        locals.var_dq2_rv = 0.0;

        let assign9390_e9026: f64 = (locals.var_auxb1 + locals.var_t2);
        let assign9390_e9027: f64 = (locals.var_k1 * assign9390_e9026);
        let assign9390_e9028: f64 = (locals.var_aaux - assign9390_e9027);
        let assign9390_e9031: f64 = (locals.var_auxb1 * locals.var_dqcoth);
        let assign9390_e9032: f64 = (assign9390_e9028 + assign9390_e9031);
        let assign9390_e9036: f64 = (locals.var_dq2 * locals.var_t2);
        let assign9390_e9040: f64 = (locals.var_dqcoth - locals.var_k1);
        let assign9390_e9041: f64 = (locals.var_q2 * assign9390_e9040);
        let assign9390_e9042: f64 = (assign9390_e9036 + assign9390_e9041);
        let assign9390_e9043: f64 = (locals.var_k2 * assign9390_e9042);
        let assign9390_e9044: f64 = (assign9390_e9032 + assign9390_e9043);
        locals.var_df = assign9390_e9044;
        locals.var_df_dn3 = (((locals.var_aaux_dn3 - (locals.var_k1 * (locals.var_auxb1_dn3 + locals.var_t2_dn3))) + ((locals.var_auxb1_dn3 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn3))) + (locals.var_k2 * (((locals.var_dq2_dn3 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn3)) + ((locals.var_q2_dn3 * assign9390_e9040) + (locals.var_q2 * locals.var_dqcoth_dn3)))));
        locals.var_df_dn4 = (((locals.var_aaux_dn4 - (locals.var_k1 * (locals.var_auxb1_dn4 + locals.var_t2_dn4))) + ((locals.var_auxb1_dn4 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn4))) + (locals.var_k2 * (((locals.var_dq2_dn4 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn4)) + ((locals.var_q2_dn4 * assign9390_e9040) + (locals.var_q2 * locals.var_dqcoth_dn4)))));
        locals.var_df_dn5 = (((locals.var_aaux_dn5 - (locals.var_k1 * (locals.var_auxb1_dn5 + locals.var_t2_dn5))) + ((locals.var_auxb1_dn5 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn5))) + (locals.var_k2 * (((locals.var_dq2_dn5 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn5)) + ((locals.var_q2_dn5 * assign9390_e9040) + (locals.var_q2 * locals.var_dqcoth_dn5)))));
        locals.var_df_dn6 = (((locals.var_aaux_dn6 - (locals.var_k1 * (locals.var_auxb1_dn6 + locals.var_t2_dn6))) + ((locals.var_auxb1_dn6 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn6))) + (locals.var_k2 * (((locals.var_dq2_dn6 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn6)) + ((locals.var_q2_dn6 * assign9390_e9040) + (locals.var_q2 * locals.var_dqcoth_dn6)))));
        locals.var_df_dn7 = (((locals.var_aaux_dn7 - (locals.var_k1 * (locals.var_auxb1_dn7 + locals.var_t2_dn7))) + ((locals.var_auxb1_dn7 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn7))) + (locals.var_k2 * (((locals.var_dq2_dn7 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn7)) + ((locals.var_q2_dn7 * assign9390_e9040) + (locals.var_q2 * locals.var_dqcoth_dn7)))));
        locals.var_df_dn8 = (((locals.var_aaux_dn8 - (locals.var_k1 * (locals.var_auxb1_dn8 + locals.var_t2_dn8))) + ((locals.var_auxb1_dn8 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn8))) + (locals.var_k2 * (((locals.var_dq2_dn8 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn8)) + ((locals.var_q2_dn8 * assign9390_e9040) + (locals.var_q2 * locals.var_dqcoth_dn8)))));
        locals.var_df_rv = 0.0;

        let assign9400_e9046: f64 = (-locals.var_f);
        let assign9400_e9048: f64 = (assign9400_e9046 / locals.var_df);
        locals.var_delta = assign9400_e9048;
        locals.var_delta_dn3 = ((((-locals.var_f_dn3) * locals.var_df) - (assign9400_e9046 * locals.var_df_dn3)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn4 = ((((-locals.var_f_dn4) * locals.var_df) - (assign9400_e9046 * locals.var_df_dn4)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn5 = ((((-locals.var_f_dn5) * locals.var_df) - (assign9400_e9046 * locals.var_df_dn5)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn6 = ((((-locals.var_f_dn6) * locals.var_df) - (assign9400_e9046 * locals.var_df_dn6)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn7 = ((((-locals.var_f_dn7) * locals.var_df) - (assign9400_e9046 * locals.var_df_dn7)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn8 = ((((-locals.var_f_dn8) * locals.var_df) - (assign9400_e9046 * locals.var_df_dn8)) / (locals.var_df * locals.var_df));
        locals.var_delta_rv = 0.0;

        let assign9410_e9051: f64 = (locals.var_phi1 + locals.var_delta);
        locals.var_phi1 = assign9410_e9051;
        locals.var_phi1_dn3 = (locals.var_phi1_dn3 + locals.var_delta_dn3);
        locals.var_phi1_dn4 = (locals.var_phi1_dn4 + locals.var_delta_dn4);
        locals.var_phi1_dn5 = (locals.var_phi1_dn5 + locals.var_delta_dn5);
        locals.var_phi1_dn6 = (locals.var_phi1_dn6 + locals.var_delta_dn6);
        locals.var_phi1_dn7 = (locals.var_phi1_dn7 + locals.var_delta_dn7);
        locals.var_phi1_dn8 = (locals.var_phi1_dn8 + locals.var_delta_dn8);
        locals.var_phi1_rv = 0.0;

        let assign9420_e9054: f64 = (locals.var_xg1 - locals.var_phi1);
        locals.var_q1 = assign9420_e9054;
        locals.var_q1_dn3 = (locals.var_xg1_dn3 - locals.var_phi1_dn3);
        locals.var_q1_dn4 = (locals.var_xg1_dn4 - locals.var_phi1_dn4);
        locals.var_q1_dn5 = (locals.var_xg1_dn5 - locals.var_phi1_dn5);
        locals.var_q1_dn6 = (locals.var_xg1_dn6 - locals.var_phi1_dn6);
        locals.var_q1_dn7 = (locals.var_xg1_dn7 - locals.var_phi1_dn7);
        locals.var_q1_dn8 = (locals.var_xg1_dn8 - locals.var_phi1_dn8);
        locals.var_q1_rv = 0.0;

        let assign9430_e9057: f64 = (locals.var_k1 * locals.var_q1);
        locals.var_auxb1 = assign9430_e9057;
        locals.var_auxb1_dn3 = (locals.var_k1 * locals.var_q1_dn3);
        locals.var_auxb1_dn4 = (locals.var_k1 * locals.var_q1_dn4);
        locals.var_auxb1_dn5 = (locals.var_k1 * locals.var_q1_dn5);
        locals.var_auxb1_dn6 = (locals.var_k1 * locals.var_q1_dn6);
        locals.var_auxb1_dn7 = (locals.var_k1 * locals.var_q1_dn7);
        locals.var_auxb1_dn8 = (locals.var_k1 * locals.var_q1_dn8);
        locals.var_auxb1_rv = 0.0;

        let assign9440_e9059: f64 = (-locals.var_a0);
        let assign9440_e9061: f64 = (locals.var_phi1).exp();
        let assign9440_e9062: f64 = (assign9440_e9059 * assign9440_e9061);
        locals.var_aaux = assign9440_e9062;
        locals.var_aaux_dn3 = (((-locals.var_a0_dn3) * assign9440_e9061) + (assign9440_e9059 * (assign9440_e9061 * locals.var_phi1_dn3)));
        locals.var_aaux_dn4 = (((-locals.var_a0_dn4) * assign9440_e9061) + (assign9440_e9059 * (assign9440_e9061 * locals.var_phi1_dn4)));
        locals.var_aaux_dn5 = (((-locals.var_a0_dn5) * assign9440_e9061) + (assign9440_e9059 * (assign9440_e9061 * locals.var_phi1_dn5)));
        locals.var_aaux_dn6 = (((-locals.var_a0_dn6) * assign9440_e9061) + (assign9440_e9059 * (assign9440_e9061 * locals.var_phi1_dn6)));
        locals.var_aaux_dn7 = (((-locals.var_a0_dn7) * assign9440_e9061) + (assign9440_e9059 * (assign9440_e9061 * locals.var_phi1_dn7)));
        locals.var_aaux_dn8 = (((-locals.var_a0_dn8) * assign9440_e9061) + (assign9440_e9059 * (assign9440_e9061 * locals.var_phi1_dn8)));
        locals.var_aaux_rv = 0.0;

        let assign9450_e9065: f64 = (locals.var_auxb1 * locals.var_auxb1);
        let assign9450_e9067: f64 = (assign9450_e9065 + locals.var_aaux);
        locals.var_qsqrt = assign9450_e9067;
        locals.var_qsqrt_dn3 = (((locals.var_auxb1_dn3 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn3)) + locals.var_aaux_dn3);
        locals.var_qsqrt_dn4 = (((locals.var_auxb1_dn4 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn4)) + locals.var_aaux_dn4);
        locals.var_qsqrt_dn5 = (((locals.var_auxb1_dn5 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn5)) + locals.var_aaux_dn5);
        locals.var_qsqrt_dn6 = (((locals.var_auxb1_dn6 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn6)) + locals.var_aaux_dn6);
        locals.var_qsqrt_dn7 = (((locals.var_auxb1_dn7 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn7)) + locals.var_aaux_dn7);
        locals.var_qsqrt_dn8 = (((locals.var_auxb1_dn8 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn8)) + locals.var_aaux_dn8);
        locals.var_qsqrt_rv = 0.0;

        let assign9460_e9070: f64 = if locals.var_qsqrt < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard95 = assign9460_e9070;
        locals.var_guard95_rv = 0.0;

        let (assign9470_e9076, assign9470_e9076_d_n3, assign9470_e9076_d_n4, assign9470_e9076_d_n5, assign9470_e9076_d_n6, assign9470_e9076_d_n7, assign9470_e9076_d_n8,) = {
    if (locals.var_guard95 != 0.0) {
        let assign9470_e9073: f64 = (-locals.var_qsqrt);
        let assign9470_e9074: f64 = (assign9470_e9073).sqrt();
        (assign9470_e9074, ((-locals.var_qsqrt_dn3) / (2.0 * assign9470_e9074)), ((-locals.var_qsqrt_dn4) / (2.0 * assign9470_e9074)), ((-locals.var_qsqrt_dn5) / (2.0 * assign9470_e9074)), ((-locals.var_qsqrt_dn6) / (2.0 * assign9470_e9074)), ((-locals.var_qsqrt_dn7) / (2.0 * assign9470_e9074)), ((-locals.var_qsqrt_dn8) / (2.0 * assign9470_e9074)),)
    } else {
        (locals.var_q, locals.var_q_dn3, locals.var_q_dn4, locals.var_q_dn5, locals.var_q_dn6, locals.var_q_dn7, locals.var_q_dn8,)
    }
};
        locals.var_q = assign9470_e9076;
        locals.var_q_dn3 = assign9470_e9076_d_n3;
        locals.var_q_dn4 = assign9470_e9076_d_n4;
        locals.var_q_dn5 = assign9470_e9076_d_n5;
        locals.var_q_dn6 = assign9470_e9076_d_n6;
        locals.var_q_dn7 = assign9470_e9076_d_n7;
        locals.var_q_dn8 = assign9470_e9076_d_n8;
        locals.var_q_rv = 0.0;

        let (assign9480_e9085, assign9480_e9085_d_n3, assign9480_e9085_d_n4, assign9480_e9085_d_n5, assign9480_e9085_d_n6, assign9480_e9085_d_n7, assign9480_e9085_d_n8,) = {
    if (locals.var_guard95 != 0.0) {
        let assign9480_e9081: f64 = (0.5 * locals.var_q);
        let assign9480_e9082: f64 = (assign9480_e9081).sin();
        let assign9480_e9083: f64 = (1.0 / assign9480_e9082);
        (assign9480_e9083, (-(((assign9480_e9081).cos() * (0.5 * locals.var_q_dn3)) / (assign9480_e9082 * assign9480_e9082))), (-(((assign9480_e9081).cos() * (0.5 * locals.var_q_dn4)) / (assign9480_e9082 * assign9480_e9082))), (-(((assign9480_e9081).cos() * (0.5 * locals.var_q_dn5)) / (assign9480_e9082 * assign9480_e9082))), (-(((assign9480_e9081).cos() * (0.5 * locals.var_q_dn6)) / (assign9480_e9082 * assign9480_e9082))), (-(((assign9480_e9081).cos() * (0.5 * locals.var_q_dn7)) / (assign9480_e9082 * assign9480_e9082))), (-(((assign9480_e9081).cos() * (0.5 * locals.var_q_dn8)) / (assign9480_e9082 * assign9480_e9082))),)
    } else {
        (locals.var_csc1, locals.var_csc1_dn3, locals.var_csc1_dn4, locals.var_csc1_dn5, locals.var_csc1_dn6, locals.var_csc1_dn7, locals.var_csc1_dn8,)
    }
};
        locals.var_csc1 = assign9480_e9085;
        locals.var_csc1_dn3 = assign9480_e9085_d_n3;
        locals.var_csc1_dn4 = assign9480_e9085_d_n4;
        locals.var_csc1_dn5 = assign9480_e9085_d_n5;
        locals.var_csc1_dn6 = assign9480_e9085_d_n6;
        locals.var_csc1_dn7 = assign9480_e9085_d_n7;
        locals.var_csc1_dn8 = assign9480_e9085_d_n8;
        locals.var_csc1_rv = 0.0;

        let (assign9490_e9091, assign9490_e9091_d_n3, assign9490_e9091_d_n4, assign9490_e9091_d_n5, assign9490_e9091_d_n6, assign9490_e9091_d_n7, assign9490_e9091_d_n8,) = {
    if (locals.var_guard95 != 0.0) {
        let assign9490_e9089: f64 = (locals.var_csc1 * locals.var_csc1);
        (assign9490_e9089, ((locals.var_csc1_dn3 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn3)), ((locals.var_csc1_dn4 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn4)), ((locals.var_csc1_dn5 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn5)), ((locals.var_csc1_dn6 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn6)), ((locals.var_csc1_dn7 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn7)), ((locals.var_csc1_dn8 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn8)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign9490_e9091;
        locals.var_t1_dn3 = assign9490_e9091_d_n3;
        locals.var_t1_dn4 = assign9490_e9091_d_n4;
        locals.var_t1_dn5 = assign9490_e9091_d_n5;
        locals.var_t1_dn6 = assign9490_e9091_d_n6;
        locals.var_t1_dn7 = assign9490_e9091_d_n7;
        locals.var_t1_dn8 = assign9490_e9091_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign9500_e9100, assign9500_e9100_d_n3, assign9500_e9100_d_n4, assign9500_e9100_d_n5, assign9500_e9100_d_n6, assign9500_e9100_d_n7, assign9500_e9100_d_n8,) = {
    if (locals.var_guard95 != 0.0) {
        let assign9500_e9095: f64 = (0.5 * locals.var_q);
        let assign9500_e9096: f64 = (assign9500_e9095).cos();
        let assign9500_e9098: f64 = (assign9500_e9096 * locals.var_csc1);
        (assign9500_e9098, (((-(assign9500_e9095).sin() * (0.5 * locals.var_q_dn3)) * locals.var_csc1) + (assign9500_e9096 * locals.var_csc1_dn3)), (((-(assign9500_e9095).sin() * (0.5 * locals.var_q_dn4)) * locals.var_csc1) + (assign9500_e9096 * locals.var_csc1_dn4)), (((-(assign9500_e9095).sin() * (0.5 * locals.var_q_dn5)) * locals.var_csc1) + (assign9500_e9096 * locals.var_csc1_dn5)), (((-(assign9500_e9095).sin() * (0.5 * locals.var_q_dn6)) * locals.var_csc1) + (assign9500_e9096 * locals.var_csc1_dn6)), (((-(assign9500_e9095).sin() * (0.5 * locals.var_q_dn7)) * locals.var_csc1) + (assign9500_e9096 * locals.var_csc1_dn7)), (((-(assign9500_e9095).sin() * (0.5 * locals.var_q_dn8)) * locals.var_csc1) + (assign9500_e9096 * locals.var_csc1_dn8)),)
    } else {
        (locals.var_coth1, locals.var_coth1_dn3, locals.var_coth1_dn4, locals.var_coth1_dn5, locals.var_coth1_dn6, locals.var_coth1_dn7, locals.var_coth1_dn8,)
    }
};
        locals.var_coth1 = assign9500_e9100;
        locals.var_coth1_dn3 = assign9500_e9100_d_n3;
        locals.var_coth1_dn4 = assign9500_e9100_d_n4;
        locals.var_coth1_dn5 = assign9500_e9100_d_n5;
        locals.var_coth1_dn6 = assign9500_e9100_d_n6;
        locals.var_coth1_dn7 = assign9500_e9100_d_n7;
        locals.var_coth1_dn8 = assign9500_e9100_d_n8;
        locals.var_coth1_rv = 0.0;

        let (assign9510_e9109, assign9510_e9109_d_n3, assign9510_e9109_d_n4, assign9510_e9109_d_n5, assign9510_e9109_d_n6, assign9510_e9109_d_n7, assign9510_e9109_d_n8,) = {
    if (locals.var_guard95 != 0.0) {
        let assign9510_e9103: f64 = (-0.5);
        let assign9510_e9105: f64 = (assign9510_e9103 * locals.var_coth1);
        let assign9510_e9107: f64 = (assign9510_e9105 / locals.var_q);
        (assign9510_e9107, ((((assign9510_e9103 * locals.var_coth1_dn3) * locals.var_q) - (assign9510_e9105 * locals.var_q_dn3)) / (locals.var_q * locals.var_q)), ((((assign9510_e9103 * locals.var_coth1_dn4) * locals.var_q) - (assign9510_e9105 * locals.var_q_dn4)) / (locals.var_q * locals.var_q)), ((((assign9510_e9103 * locals.var_coth1_dn5) * locals.var_q) - (assign9510_e9105 * locals.var_q_dn5)) / (locals.var_q * locals.var_q)), ((((assign9510_e9103 * locals.var_coth1_dn6) * locals.var_q) - (assign9510_e9105 * locals.var_q_dn6)) / (locals.var_q * locals.var_q)), ((((assign9510_e9103 * locals.var_coth1_dn7) * locals.var_q) - (assign9510_e9105 * locals.var_q_dn7)) / (locals.var_q * locals.var_q)), ((((assign9510_e9103 * locals.var_coth1_dn8) * locals.var_q) - (assign9510_e9105 * locals.var_q_dn8)) / (locals.var_q * locals.var_q)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    }
};
        locals.var_t0 = assign9510_e9109;
        locals.var_t0_dn3 = assign9510_e9109_d_n3;
        locals.var_t0_dn4 = assign9510_e9109_d_n4;
        locals.var_t0_dn5 = assign9510_e9109_d_n5;
        locals.var_t0_dn6 = assign9510_e9109_d_n6;
        locals.var_t0_dn7 = assign9510_e9109_d_n7;
        locals.var_t0_dn8 = assign9510_e9109_d_n8;
        locals.var_t0_rv = 0.0;

        let (assign9520_e9117, assign9520_e9117_d_n3, assign9520_e9117_d_n4, assign9520_e9117_d_n5, assign9520_e9117_d_n6, assign9520_e9117_d_n7, assign9520_e9117_d_n8,) = {
    if (locals.var_guard95 != 0.0) {
        let assign9520_e9113: f64 = (0.25 * locals.var_t1);
        let assign9520_e9115: f64 = (assign9520_e9113 + locals.var_t0);
        (assign9520_e9115, ((0.25 * locals.var_t1_dn3) + locals.var_t0_dn3), ((0.25 * locals.var_t1_dn4) + locals.var_t0_dn4), ((0.25 * locals.var_t1_dn5) + locals.var_t0_dn5), ((0.25 * locals.var_t1_dn6) + locals.var_t0_dn6), ((0.25 * locals.var_t1_dn7) + locals.var_t0_dn7), ((0.25 * locals.var_t1_dn8) + locals.var_t0_dn8),)
    } else {
        (locals.var_dqcothqdqsqrt, locals.var_dqcothqdqsqrt_dn3, locals.var_dqcothqdqsqrt_dn4, locals.var_dqcothqdqsqrt_dn5, locals.var_dqcothqdqsqrt_dn6, locals.var_dqcothqdqsqrt_dn7, locals.var_dqcothqdqsqrt_dn8,)
    }
};
        locals.var_dqcothqdqsqrt = assign9520_e9117;
        locals.var_dqcothqdqsqrt_dn3 = assign9520_e9117_d_n3;
        locals.var_dqcothqdqsqrt_dn4 = assign9520_e9117_d_n4;
        locals.var_dqcothqdqsqrt_dn5 = assign9520_e9117_d_n5;
        locals.var_dqcothqdqsqrt_dn6 = assign9520_e9117_d_n6;
        locals.var_dqcothqdqsqrt_dn7 = assign9520_e9117_d_n7;
        locals.var_dqcothqdqsqrt_dn8 = assign9520_e9117_d_n8;
        locals.var_dqcothqdqsqrt_rv = 0.0;

        let (assign9530_e9123, assign9530_e9123_d_n3, assign9530_e9123_d_n4, assign9530_e9123_d_n5, assign9530_e9123_d_n6, assign9530_e9123_d_n7, assign9530_e9123_d_n8,) = {
    if (locals.var_guard95 == 0.0) {
        let assign9530_e9121: f64 = (locals.var_qsqrt).sqrt();
        (assign9530_e9121, (locals.var_qsqrt_dn3 / (2.0 * assign9530_e9121)), (locals.var_qsqrt_dn4 / (2.0 * assign9530_e9121)), (locals.var_qsqrt_dn5 / (2.0 * assign9530_e9121)), (locals.var_qsqrt_dn6 / (2.0 * assign9530_e9121)), (locals.var_qsqrt_dn7 / (2.0 * assign9530_e9121)), (locals.var_qsqrt_dn8 / (2.0 * assign9530_e9121)),)
    } else {
        (locals.var_q, locals.var_q_dn3, locals.var_q_dn4, locals.var_q_dn5, locals.var_q_dn6, locals.var_q_dn7, locals.var_q_dn8,)
    }
};
        locals.var_q = assign9530_e9123;
        locals.var_q_dn3 = assign9530_e9123_d_n3;
        locals.var_q_dn4 = assign9530_e9123_d_n4;
        locals.var_q_dn5 = assign9530_e9123_d_n5;
        locals.var_q_dn6 = assign9530_e9123_d_n6;
        locals.var_q_dn7 = assign9530_e9123_d_n7;
        locals.var_q_dn8 = assign9530_e9123_d_n8;
        locals.var_q_rv = 0.0;

        let (assign9540_e9133, assign9540_e9133_d_n3, assign9540_e9133_d_n4, assign9540_e9133_d_n5, assign9540_e9133_d_n6, assign9540_e9133_d_n7, assign9540_e9133_d_n8,) = {
    if (locals.var_guard95 == 0.0) {
        let assign9540_e9129: f64 = (0.5 * locals.var_q);
        let assign9540_e9130: f64 = (assign9540_e9129).sinh();
        let assign9540_e9131: f64 = (1.0 / assign9540_e9130);
        (assign9540_e9131, (-(((assign9540_e9129).cosh() * (0.5 * locals.var_q_dn3)) / (assign9540_e9130 * assign9540_e9130))), (-(((assign9540_e9129).cosh() * (0.5 * locals.var_q_dn4)) / (assign9540_e9130 * assign9540_e9130))), (-(((assign9540_e9129).cosh() * (0.5 * locals.var_q_dn5)) / (assign9540_e9130 * assign9540_e9130))), (-(((assign9540_e9129).cosh() * (0.5 * locals.var_q_dn6)) / (assign9540_e9130 * assign9540_e9130))), (-(((assign9540_e9129).cosh() * (0.5 * locals.var_q_dn7)) / (assign9540_e9130 * assign9540_e9130))), (-(((assign9540_e9129).cosh() * (0.5 * locals.var_q_dn8)) / (assign9540_e9130 * assign9540_e9130))),)
    } else {
        (locals.var_csc1, locals.var_csc1_dn3, locals.var_csc1_dn4, locals.var_csc1_dn5, locals.var_csc1_dn6, locals.var_csc1_dn7, locals.var_csc1_dn8,)
    }
};
        locals.var_csc1 = assign9540_e9133;
        locals.var_csc1_dn3 = assign9540_e9133_d_n3;
        locals.var_csc1_dn4 = assign9540_e9133_d_n4;
        locals.var_csc1_dn5 = assign9540_e9133_d_n5;
        locals.var_csc1_dn6 = assign9540_e9133_d_n6;
        locals.var_csc1_dn7 = assign9540_e9133_d_n7;
        locals.var_csc1_dn8 = assign9540_e9133_d_n8;
        locals.var_csc1_rv = 0.0;

        let (assign9550_e9140, assign9550_e9140_d_n3, assign9550_e9140_d_n4, assign9550_e9140_d_n5, assign9550_e9140_d_n6, assign9550_e9140_d_n7, assign9550_e9140_d_n8,) = {
    if (locals.var_guard95 == 0.0) {
        let assign9550_e9138: f64 = (locals.var_csc1 * locals.var_csc1);
        (assign9550_e9138, ((locals.var_csc1_dn3 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn3)), ((locals.var_csc1_dn4 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn4)), ((locals.var_csc1_dn5 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn5)), ((locals.var_csc1_dn6 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn6)), ((locals.var_csc1_dn7 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn7)), ((locals.var_csc1_dn8 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn8)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign9550_e9140;
        locals.var_t1_dn3 = assign9550_e9140_d_n3;
        locals.var_t1_dn4 = assign9550_e9140_d_n4;
        locals.var_t1_dn5 = assign9550_e9140_d_n5;
        locals.var_t1_dn6 = assign9550_e9140_d_n6;
        locals.var_t1_dn7 = assign9550_e9140_d_n7;
        locals.var_t1_dn8 = assign9550_e9140_d_n8;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_21(
        locals: &mut StampLocals,
    ) {
        let (assign9560_e9148, assign9560_e9148_d_n3, assign9560_e9148_d_n4, assign9560_e9148_d_n5, assign9560_e9148_d_n6, assign9560_e9148_d_n7, assign9560_e9148_d_n8,) = {
    if (locals.var_guard95 == 0.0) {
        let assign9560_e9145: f64 = (1.0 + locals.var_t1);
        let assign9560_e9146: f64 = (assign9560_e9145).sqrt();
        (assign9560_e9146, (locals.var_t1_dn3 / (2.0 * assign9560_e9146)), (locals.var_t1_dn4 / (2.0 * assign9560_e9146)), (locals.var_t1_dn5 / (2.0 * assign9560_e9146)), (locals.var_t1_dn6 / (2.0 * assign9560_e9146)), (locals.var_t1_dn7 / (2.0 * assign9560_e9146)), (locals.var_t1_dn8 / (2.0 * assign9560_e9146)),)
    } else {
        (locals.var_coth1, locals.var_coth1_dn3, locals.var_coth1_dn4, locals.var_coth1_dn5, locals.var_coth1_dn6, locals.var_coth1_dn7, locals.var_coth1_dn8,)
    }
};
        locals.var_coth1 = assign9560_e9148;
        locals.var_coth1_dn3 = assign9560_e9148_d_n3;
        locals.var_coth1_dn4 = assign9560_e9148_d_n4;
        locals.var_coth1_dn5 = assign9560_e9148_d_n5;
        locals.var_coth1_dn6 = assign9560_e9148_d_n6;
        locals.var_coth1_dn7 = assign9560_e9148_d_n7;
        locals.var_coth1_dn8 = assign9560_e9148_d_n8;
        locals.var_coth1_rv = 0.0;

        let (assign9570_e9157, assign9570_e9157_d_n3, assign9570_e9157_d_n4, assign9570_e9157_d_n5, assign9570_e9157_d_n6, assign9570_e9157_d_n7, assign9570_e9157_d_n8,) = {
    if (locals.var_guard95 == 0.0) {
        let assign9570_e9153: f64 = (0.5 * locals.var_coth1);
        let assign9570_e9155: f64 = (assign9570_e9153 / locals.var_q);
        (assign9570_e9155, ((((0.5 * locals.var_coth1_dn3) * locals.var_q) - (assign9570_e9153 * locals.var_q_dn3)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn4) * locals.var_q) - (assign9570_e9153 * locals.var_q_dn4)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn5) * locals.var_q) - (assign9570_e9153 * locals.var_q_dn5)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn6) * locals.var_q) - (assign9570_e9153 * locals.var_q_dn6)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn7) * locals.var_q) - (assign9570_e9153 * locals.var_q_dn7)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn8) * locals.var_q) - (assign9570_e9153 * locals.var_q_dn8)) / (locals.var_q * locals.var_q)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    }
};
        locals.var_t0 = assign9570_e9157;
        locals.var_t0_dn3 = assign9570_e9157_d_n3;
        locals.var_t0_dn4 = assign9570_e9157_d_n4;
        locals.var_t0_dn5 = assign9570_e9157_d_n5;
        locals.var_t0_dn6 = assign9570_e9157_d_n6;
        locals.var_t0_dn7 = assign9570_e9157_d_n7;
        locals.var_t0_dn8 = assign9570_e9157_d_n8;
        locals.var_t0_rv = 0.0;

        let (assign9580_e9167, assign9580_e9167_d_n3, assign9580_e9167_d_n4, assign9580_e9167_d_n5, assign9580_e9167_d_n6, assign9580_e9167_d_n7, assign9580_e9167_d_n8,) = {
    if (locals.var_guard95 == 0.0) {
        let assign9580_e9161: f64 = (-0.25);
        let assign9580_e9163: f64 = (assign9580_e9161 * locals.var_t1);
        let assign9580_e9165: f64 = (assign9580_e9163 + locals.var_t0);
        (assign9580_e9165, ((assign9580_e9161 * locals.var_t1_dn3) + locals.var_t0_dn3), ((assign9580_e9161 * locals.var_t1_dn4) + locals.var_t0_dn4), ((assign9580_e9161 * locals.var_t1_dn5) + locals.var_t0_dn5), ((assign9580_e9161 * locals.var_t1_dn6) + locals.var_t0_dn6), ((assign9580_e9161 * locals.var_t1_dn7) + locals.var_t0_dn7), ((assign9580_e9161 * locals.var_t1_dn8) + locals.var_t0_dn8),)
    } else {
        (locals.var_dqcothqdqsqrt, locals.var_dqcothqdqsqrt_dn3, locals.var_dqcothqdqsqrt_dn4, locals.var_dqcothqdqsqrt_dn5, locals.var_dqcothqdqsqrt_dn6, locals.var_dqcothqdqsqrt_dn7, locals.var_dqcothqdqsqrt_dn8,)
    }
};
        locals.var_dqcothqdqsqrt = assign9580_e9167;
        locals.var_dqcothqdqsqrt_dn3 = assign9580_e9167_d_n3;
        locals.var_dqcothqdqsqrt_dn4 = assign9580_e9167_d_n4;
        locals.var_dqcothqdqsqrt_dn5 = assign9580_e9167_d_n5;
        locals.var_dqcothqdqsqrt_dn6 = assign9580_e9167_d_n6;
        locals.var_dqcothqdqsqrt_dn7 = assign9580_e9167_d_n7;
        locals.var_dqcothqdqsqrt_dn8 = assign9580_e9167_d_n8;
        locals.var_dqcothqdqsqrt_rv = 0.0;

        let assign9590_e9170: f64 = (locals.var_q * locals.var_coth1);
        locals.var_qcoth = assign9590_e9170;
        locals.var_qcoth_dn3 = ((locals.var_q_dn3 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn3));
        locals.var_qcoth_dn4 = ((locals.var_q_dn4 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn4));
        locals.var_qcoth_dn5 = ((locals.var_q_dn5 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn5));
        locals.var_qcoth_dn6 = ((locals.var_q_dn6 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn6));
        locals.var_qcoth_dn7 = ((locals.var_q_dn7 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn7));
        locals.var_qcoth_dn8 = ((locals.var_q_dn8 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn8));
        locals.var_qcoth_rv = 0.0;

        let assign9600_e9173: f64 = (locals.var_auxb1 + locals.var_qcoth);
        locals.var_t2 = assign9600_e9173;
        locals.var_t2_dn3 = (locals.var_auxb1_dn3 + locals.var_qcoth_dn3);
        locals.var_t2_dn4 = (locals.var_auxb1_dn4 + locals.var_qcoth_dn4);
        locals.var_t2_dn5 = (locals.var_auxb1_dn5 + locals.var_qcoth_dn5);
        locals.var_t2_dn6 = (locals.var_auxb1_dn6 + locals.var_qcoth_dn6);
        locals.var_t2_dn7 = (locals.var_auxb1_dn7 + locals.var_qcoth_dn7);
        locals.var_t2_dn8 = (locals.var_auxb1_dn8 + locals.var_qcoth_dn8);
        locals.var_t2_rv = 0.0;

        let assign9610_e9176: f64 = (1.0 / locals.var_t2);
        locals.var_t3 = assign9610_e9176;
        locals.var_t3_dn3 = (-(locals.var_t2_dn3 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn4 = (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn5 = (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn6 = (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn7 = (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn8 = (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_rv = 0.0;

        let assign9620_e9179: f64 = (locals.var_xg2 - locals.var_xg1);
        let assign9620_e9181: f64 = (assign9620_e9179 + locals.var_q1);
        let assign9620_e9184: f64 = (locals.var_qsqrt * locals.var_t1);
        let assign9620_e9186: f64 = (assign9620_e9184 * locals.var_t3);
        let assign9620_e9188: f64 = (assign9620_e9186 * locals.var_t3);
        let assign9620_e9189: f64 = (assign9620_e9188).abs();
        let assign9620_e9190: f64 = (assign9620_e9189).ln();
        let assign9620_e9191: f64 = (assign9620_e9181 - assign9620_e9190);
        locals.var_q2 = assign9620_e9191;
        locals.var_q2_dn3 = (((locals.var_xg2_dn3 - locals.var_xg1_dn3) + locals.var_q1_dn3) - (if assign9620_e9188 >= 0.0 { ((((((locals.var_qsqrt_dn3 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn3)) * locals.var_t3) + (assign9620_e9184 * locals.var_t3_dn3)) * locals.var_t3) + (assign9620_e9186 * locals.var_t3_dn3)) } else { (-((((((locals.var_qsqrt_dn3 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn3)) * locals.var_t3) + (assign9620_e9184 * locals.var_t3_dn3)) * locals.var_t3) + (assign9620_e9186 * locals.var_t3_dn3))) } / assign9620_e9189));
        locals.var_q2_dn4 = (((locals.var_xg2_dn4 - locals.var_xg1_dn4) + locals.var_q1_dn4) - (if assign9620_e9188 >= 0.0 { ((((((locals.var_qsqrt_dn4 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn4)) * locals.var_t3) + (assign9620_e9184 * locals.var_t3_dn4)) * locals.var_t3) + (assign9620_e9186 * locals.var_t3_dn4)) } else { (-((((((locals.var_qsqrt_dn4 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn4)) * locals.var_t3) + (assign9620_e9184 * locals.var_t3_dn4)) * locals.var_t3) + (assign9620_e9186 * locals.var_t3_dn4))) } / assign9620_e9189));
        locals.var_q2_dn5 = (((locals.var_xg2_dn5 - locals.var_xg1_dn5) + locals.var_q1_dn5) - (if assign9620_e9188 >= 0.0 { ((((((locals.var_qsqrt_dn5 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn5)) * locals.var_t3) + (assign9620_e9184 * locals.var_t3_dn5)) * locals.var_t3) + (assign9620_e9186 * locals.var_t3_dn5)) } else { (-((((((locals.var_qsqrt_dn5 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn5)) * locals.var_t3) + (assign9620_e9184 * locals.var_t3_dn5)) * locals.var_t3) + (assign9620_e9186 * locals.var_t3_dn5))) } / assign9620_e9189));
        locals.var_q2_dn6 = (((locals.var_xg2_dn6 - locals.var_xg1_dn6) + locals.var_q1_dn6) - (if assign9620_e9188 >= 0.0 { ((((((locals.var_qsqrt_dn6 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn6)) * locals.var_t3) + (assign9620_e9184 * locals.var_t3_dn6)) * locals.var_t3) + (assign9620_e9186 * locals.var_t3_dn6)) } else { (-((((((locals.var_qsqrt_dn6 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn6)) * locals.var_t3) + (assign9620_e9184 * locals.var_t3_dn6)) * locals.var_t3) + (assign9620_e9186 * locals.var_t3_dn6))) } / assign9620_e9189));
        locals.var_q2_dn7 = (((locals.var_xg2_dn7 - locals.var_xg1_dn7) + locals.var_q1_dn7) - (if assign9620_e9188 >= 0.0 { ((((((locals.var_qsqrt_dn7 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn7)) * locals.var_t3) + (assign9620_e9184 * locals.var_t3_dn7)) * locals.var_t3) + (assign9620_e9186 * locals.var_t3_dn7)) } else { (-((((((locals.var_qsqrt_dn7 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn7)) * locals.var_t3) + (assign9620_e9184 * locals.var_t3_dn7)) * locals.var_t3) + (assign9620_e9186 * locals.var_t3_dn7))) } / assign9620_e9189));
        locals.var_q2_dn8 = (((locals.var_xg2_dn8 - locals.var_xg1_dn8) + locals.var_q1_dn8) - (if assign9620_e9188 >= 0.0 { ((((((locals.var_qsqrt_dn8 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn8)) * locals.var_t3) + (assign9620_e9184 * locals.var_t3_dn8)) * locals.var_t3) + (assign9620_e9186 * locals.var_t3_dn8)) } else { (-((((((locals.var_qsqrt_dn8 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn8)) * locals.var_t3) + (assign9620_e9184 * locals.var_t3_dn8)) * locals.var_t3) + (assign9620_e9186 * locals.var_t3_dn8))) } / assign9620_e9189));
        locals.var_q2_rv = 0.0;

        let assign9630_e9195: f64 = (locals.var_auxb1 + locals.var_qcoth);
        let assign9630_e9198: f64 = (locals.var_k2 * locals.var_q2);
        let assign9630_e9200: f64 = (assign9630_e9198 + locals.var_auxb1);
        let assign9630_e9201: f64 = (assign9630_e9195 * assign9630_e9200);
        let assign9630_e9202: f64 = (locals.var_aaux + assign9630_e9201);
        locals.var_f = assign9630_e9202;
        locals.var_f_dn3 = (locals.var_aaux_dn3 + (((locals.var_auxb1_dn3 + locals.var_qcoth_dn3) * assign9630_e9200) + (assign9630_e9195 * ((locals.var_k2 * locals.var_q2_dn3) + locals.var_auxb1_dn3))));
        locals.var_f_dn4 = (locals.var_aaux_dn4 + (((locals.var_auxb1_dn4 + locals.var_qcoth_dn4) * assign9630_e9200) + (assign9630_e9195 * ((locals.var_k2 * locals.var_q2_dn4) + locals.var_auxb1_dn4))));
        locals.var_f_dn5 = (locals.var_aaux_dn5 + (((locals.var_auxb1_dn5 + locals.var_qcoth_dn5) * assign9630_e9200) + (assign9630_e9195 * ((locals.var_k2 * locals.var_q2_dn5) + locals.var_auxb1_dn5))));
        locals.var_f_dn6 = (locals.var_aaux_dn6 + (((locals.var_auxb1_dn6 + locals.var_qcoth_dn6) * assign9630_e9200) + (assign9630_e9195 * ((locals.var_k2 * locals.var_q2_dn6) + locals.var_auxb1_dn6))));
        locals.var_f_dn7 = (locals.var_aaux_dn7 + (((locals.var_auxb1_dn7 + locals.var_qcoth_dn7) * assign9630_e9200) + (assign9630_e9195 * ((locals.var_k2 * locals.var_q2_dn7) + locals.var_auxb1_dn7))));
        locals.var_f_dn8 = (locals.var_aaux_dn8 + (((locals.var_auxb1_dn8 + locals.var_qcoth_dn8) * assign9630_e9200) + (assign9630_e9195 * ((locals.var_k2 * locals.var_q2_dn8) + locals.var_auxb1_dn8))));
        locals.var_f_rv = 0.0;

        let assign9640_e9205: f64 = (1.0 / locals.var_qsqrt);
        let assign9640_e9207: f64 = (assign9640_e9205 - locals.var_t0);
        locals.var_dlogsinhqsqdqsqrt = assign9640_e9207;
        locals.var_dlogsinhqsqdqsqrt_dn3 = ((-(locals.var_qsqrt_dn3 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn3);
        locals.var_dlogsinhqsqdqsqrt_dn4 = ((-(locals.var_qsqrt_dn4 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn4);
        locals.var_dlogsinhqsqdqsqrt_dn5 = ((-(locals.var_qsqrt_dn5 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn5);
        locals.var_dlogsinhqsqdqsqrt_dn6 = ((-(locals.var_qsqrt_dn6 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn6);
        locals.var_dlogsinhqsqdqsqrt_dn7 = ((-(locals.var_qsqrt_dn7 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn7);
        locals.var_dlogsinhqsqdqsqrt_dn8 = ((-(locals.var_qsqrt_dn8 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn8);
        locals.var_dlogsinhqsqdqsqrt_rv = 0.0;

        let assign9650_e9209: f64 = (-2.0);
        let assign9650_e9211: f64 = (assign9650_e9209 * locals.var_k1);
        let assign9650_e9213: f64 = (assign9650_e9211 * locals.var_auxb1);
        let assign9650_e9215: f64 = (assign9650_e9213 + locals.var_aaux);
        locals.var_dqsqrt = assign9650_e9215;
        locals.var_dqsqrt_dn3 = ((assign9650_e9211 * locals.var_auxb1_dn3) + locals.var_aaux_dn3);
        locals.var_dqsqrt_dn4 = ((assign9650_e9211 * locals.var_auxb1_dn4) + locals.var_aaux_dn4);
        locals.var_dqsqrt_dn5 = ((assign9650_e9211 * locals.var_auxb1_dn5) + locals.var_aaux_dn5);
        locals.var_dqsqrt_dn6 = ((assign9650_e9211 * locals.var_auxb1_dn6) + locals.var_aaux_dn6);
        locals.var_dqsqrt_dn7 = ((assign9650_e9211 * locals.var_auxb1_dn7) + locals.var_aaux_dn7);
        locals.var_dqsqrt_dn8 = ((assign9650_e9211 * locals.var_auxb1_dn8) + locals.var_aaux_dn8);
        locals.var_dqsqrt_rv = 0.0;

        let assign9660_e9218: f64 = (locals.var_dqcothqdqsqrt * locals.var_dqsqrt);
        locals.var_dqcoth = assign9660_e9218;
        locals.var_dqcoth_dn3 = ((locals.var_dqcothqdqsqrt_dn3 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn3));
        locals.var_dqcoth_dn4 = ((locals.var_dqcothqdqsqrt_dn4 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn4));
        locals.var_dqcoth_dn5 = ((locals.var_dqcothqdqsqrt_dn5 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn5));
        locals.var_dqcoth_dn6 = ((locals.var_dqcothqdqsqrt_dn6 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn6));
        locals.var_dqcoth_dn7 = ((locals.var_dqcothqdqsqrt_dn7 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn7));
        locals.var_dqcoth_dn8 = ((locals.var_dqcothqdqsqrt_dn8 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn8));
        locals.var_dqcoth_rv = 0.0;

        let assign9670_e9220: f64 = (-1.0);
        let assign9670_e9223: f64 = (-locals.var_k1);
        let assign9670_e9225: f64 = (assign9670_e9223 + locals.var_dqcoth);
        let assign9670_e9227: f64 = (assign9670_e9225 * locals.var_t3);
        let assign9670_e9228: f64 = (2.0 * assign9670_e9227);
        let assign9670_e9229: f64 = (assign9670_e9220 + assign9670_e9228);
        let assign9670_e9232: f64 = (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt);
        let assign9670_e9233: f64 = (assign9670_e9229 - assign9670_e9232);
        locals.var_dq2 = assign9670_e9233;
        locals.var_dq2_dn3 = ((2.0 * ((locals.var_dqcoth_dn3 * locals.var_t3) + (assign9670_e9225 * locals.var_t3_dn3))) - ((locals.var_dlogsinhqsqdqsqrt_dn3 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn3)));
        locals.var_dq2_dn4 = ((2.0 * ((locals.var_dqcoth_dn4 * locals.var_t3) + (assign9670_e9225 * locals.var_t3_dn4))) - ((locals.var_dlogsinhqsqdqsqrt_dn4 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn4)));
        locals.var_dq2_dn5 = ((2.0 * ((locals.var_dqcoth_dn5 * locals.var_t3) + (assign9670_e9225 * locals.var_t3_dn5))) - ((locals.var_dlogsinhqsqdqsqrt_dn5 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn5)));
        locals.var_dq2_dn6 = ((2.0 * ((locals.var_dqcoth_dn6 * locals.var_t3) + (assign9670_e9225 * locals.var_t3_dn6))) - ((locals.var_dlogsinhqsqdqsqrt_dn6 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn6)));
        locals.var_dq2_dn7 = ((2.0 * ((locals.var_dqcoth_dn7 * locals.var_t3) + (assign9670_e9225 * locals.var_t3_dn7))) - ((locals.var_dlogsinhqsqdqsqrt_dn7 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn7)));
        locals.var_dq2_dn8 = ((2.0 * ((locals.var_dqcoth_dn8 * locals.var_t3) + (assign9670_e9225 * locals.var_t3_dn8))) - ((locals.var_dlogsinhqsqdqsqrt_dn8 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn8)));
        locals.var_dq2_rv = 0.0;

        let assign9680_e9238: f64 = (locals.var_auxb1 + locals.var_t2);
        let assign9680_e9239: f64 = (locals.var_k1 * assign9680_e9238);
        let assign9680_e9240: f64 = (locals.var_aaux - assign9680_e9239);
        let assign9680_e9243: f64 = (locals.var_auxb1 * locals.var_dqcoth);
        let assign9680_e9244: f64 = (assign9680_e9240 + assign9680_e9243);
        let assign9680_e9248: f64 = (locals.var_dq2 * locals.var_t2);
        let assign9680_e9252: f64 = (locals.var_dqcoth - locals.var_k1);
        let assign9680_e9253: f64 = (locals.var_q2 * assign9680_e9252);
        let assign9680_e9254: f64 = (assign9680_e9248 + assign9680_e9253);
        let assign9680_e9255: f64 = (locals.var_k2 * assign9680_e9254);
        let assign9680_e9256: f64 = (assign9680_e9244 + assign9680_e9255);
        locals.var_df = assign9680_e9256;
        locals.var_df_dn3 = (((locals.var_aaux_dn3 - (locals.var_k1 * (locals.var_auxb1_dn3 + locals.var_t2_dn3))) + ((locals.var_auxb1_dn3 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn3))) + (locals.var_k2 * (((locals.var_dq2_dn3 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn3)) + ((locals.var_q2_dn3 * assign9680_e9252) + (locals.var_q2 * locals.var_dqcoth_dn3)))));
        locals.var_df_dn4 = (((locals.var_aaux_dn4 - (locals.var_k1 * (locals.var_auxb1_dn4 + locals.var_t2_dn4))) + ((locals.var_auxb1_dn4 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn4))) + (locals.var_k2 * (((locals.var_dq2_dn4 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn4)) + ((locals.var_q2_dn4 * assign9680_e9252) + (locals.var_q2 * locals.var_dqcoth_dn4)))));
        locals.var_df_dn5 = (((locals.var_aaux_dn5 - (locals.var_k1 * (locals.var_auxb1_dn5 + locals.var_t2_dn5))) + ((locals.var_auxb1_dn5 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn5))) + (locals.var_k2 * (((locals.var_dq2_dn5 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn5)) + ((locals.var_q2_dn5 * assign9680_e9252) + (locals.var_q2 * locals.var_dqcoth_dn5)))));
        locals.var_df_dn6 = (((locals.var_aaux_dn6 - (locals.var_k1 * (locals.var_auxb1_dn6 + locals.var_t2_dn6))) + ((locals.var_auxb1_dn6 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn6))) + (locals.var_k2 * (((locals.var_dq2_dn6 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn6)) + ((locals.var_q2_dn6 * assign9680_e9252) + (locals.var_q2 * locals.var_dqcoth_dn6)))));
        locals.var_df_dn7 = (((locals.var_aaux_dn7 - (locals.var_k1 * (locals.var_auxb1_dn7 + locals.var_t2_dn7))) + ((locals.var_auxb1_dn7 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn7))) + (locals.var_k2 * (((locals.var_dq2_dn7 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn7)) + ((locals.var_q2_dn7 * assign9680_e9252) + (locals.var_q2 * locals.var_dqcoth_dn7)))));
        locals.var_df_dn8 = (((locals.var_aaux_dn8 - (locals.var_k1 * (locals.var_auxb1_dn8 + locals.var_t2_dn8))) + ((locals.var_auxb1_dn8 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn8))) + (locals.var_k2 * (((locals.var_dq2_dn8 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn8)) + ((locals.var_q2_dn8 * assign9680_e9252) + (locals.var_q2 * locals.var_dqcoth_dn8)))));
        locals.var_df_rv = 0.0;

        let assign9690_e9258: f64 = (-locals.var_f);
        let assign9690_e9260: f64 = (assign9690_e9258 / locals.var_df);
        locals.var_delta = assign9690_e9260;
        locals.var_delta_dn3 = ((((-locals.var_f_dn3) * locals.var_df) - (assign9690_e9258 * locals.var_df_dn3)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn4 = ((((-locals.var_f_dn4) * locals.var_df) - (assign9690_e9258 * locals.var_df_dn4)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn5 = ((((-locals.var_f_dn5) * locals.var_df) - (assign9690_e9258 * locals.var_df_dn5)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn6 = ((((-locals.var_f_dn6) * locals.var_df) - (assign9690_e9258 * locals.var_df_dn6)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn7 = ((((-locals.var_f_dn7) * locals.var_df) - (assign9690_e9258 * locals.var_df_dn7)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn8 = ((((-locals.var_f_dn8) * locals.var_df) - (assign9690_e9258 * locals.var_df_dn8)) / (locals.var_df * locals.var_df));
        locals.var_delta_rv = 0.0;

        let assign9700_e9263: f64 = (locals.var_phi1 + locals.var_delta);
        locals.var_phi1 = assign9700_e9263;
        locals.var_phi1_dn3 = (locals.var_phi1_dn3 + locals.var_delta_dn3);
        locals.var_phi1_dn4 = (locals.var_phi1_dn4 + locals.var_delta_dn4);
        locals.var_phi1_dn5 = (locals.var_phi1_dn5 + locals.var_delta_dn5);
        locals.var_phi1_dn6 = (locals.var_phi1_dn6 + locals.var_delta_dn6);
        locals.var_phi1_dn7 = (locals.var_phi1_dn7 + locals.var_delta_dn7);
        locals.var_phi1_dn8 = (locals.var_phi1_dn8 + locals.var_delta_dn8);
        locals.var_phi1_rv = 0.0;

        let assign9710_e9266: f64 = (locals.var_xg1 - locals.var_phi1);
        locals.var_q1 = assign9710_e9266;
        locals.var_q1_dn3 = (locals.var_xg1_dn3 - locals.var_phi1_dn3);
        locals.var_q1_dn4 = (locals.var_xg1_dn4 - locals.var_phi1_dn4);
        locals.var_q1_dn5 = (locals.var_xg1_dn5 - locals.var_phi1_dn5);
        locals.var_q1_dn6 = (locals.var_xg1_dn6 - locals.var_phi1_dn6);
        locals.var_q1_dn7 = (locals.var_xg1_dn7 - locals.var_phi1_dn7);
        locals.var_q1_dn8 = (locals.var_xg1_dn8 - locals.var_phi1_dn8);
        locals.var_q1_rv = 0.0;

        let assign9720_e9269: f64 = (locals.var_k1 * locals.var_q1);
        locals.var_auxb1 = assign9720_e9269;
        locals.var_auxb1_dn3 = (locals.var_k1 * locals.var_q1_dn3);
        locals.var_auxb1_dn4 = (locals.var_k1 * locals.var_q1_dn4);
        locals.var_auxb1_dn5 = (locals.var_k1 * locals.var_q1_dn5);
        locals.var_auxb1_dn6 = (locals.var_k1 * locals.var_q1_dn6);
        locals.var_auxb1_dn7 = (locals.var_k1 * locals.var_q1_dn7);
        locals.var_auxb1_dn8 = (locals.var_k1 * locals.var_q1_dn8);
        locals.var_auxb1_rv = 0.0;

        let assign9730_e9271: f64 = (-locals.var_a0);
        let assign9730_e9273: f64 = (locals.var_phi1).exp();
        let assign9730_e9274: f64 = (assign9730_e9271 * assign9730_e9273);
        locals.var_aaux = assign9730_e9274;
        locals.var_aaux_dn3 = (((-locals.var_a0_dn3) * assign9730_e9273) + (assign9730_e9271 * (assign9730_e9273 * locals.var_phi1_dn3)));
        locals.var_aaux_dn4 = (((-locals.var_a0_dn4) * assign9730_e9273) + (assign9730_e9271 * (assign9730_e9273 * locals.var_phi1_dn4)));
        locals.var_aaux_dn5 = (((-locals.var_a0_dn5) * assign9730_e9273) + (assign9730_e9271 * (assign9730_e9273 * locals.var_phi1_dn5)));
        locals.var_aaux_dn6 = (((-locals.var_a0_dn6) * assign9730_e9273) + (assign9730_e9271 * (assign9730_e9273 * locals.var_phi1_dn6)));
        locals.var_aaux_dn7 = (((-locals.var_a0_dn7) * assign9730_e9273) + (assign9730_e9271 * (assign9730_e9273 * locals.var_phi1_dn7)));
        locals.var_aaux_dn8 = (((-locals.var_a0_dn8) * assign9730_e9273) + (assign9730_e9271 * (assign9730_e9273 * locals.var_phi1_dn8)));
        locals.var_aaux_rv = 0.0;

        let assign9740_e9277: f64 = (locals.var_auxb1 * locals.var_auxb1);
        let assign9740_e9279: f64 = (assign9740_e9277 + locals.var_aaux);
        locals.var_qsqrt = assign9740_e9279;
        locals.var_qsqrt_dn3 = (((locals.var_auxb1_dn3 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn3)) + locals.var_aaux_dn3);
        locals.var_qsqrt_dn4 = (((locals.var_auxb1_dn4 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn4)) + locals.var_aaux_dn4);
        locals.var_qsqrt_dn5 = (((locals.var_auxb1_dn5 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn5)) + locals.var_aaux_dn5);
        locals.var_qsqrt_dn6 = (((locals.var_auxb1_dn6 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn6)) + locals.var_aaux_dn6);
        locals.var_qsqrt_dn7 = (((locals.var_auxb1_dn7 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn7)) + locals.var_aaux_dn7);
        locals.var_qsqrt_dn8 = (((locals.var_auxb1_dn8 * locals.var_auxb1) + (locals.var_auxb1 * locals.var_auxb1_dn8)) + locals.var_aaux_dn8);
        locals.var_qsqrt_rv = 0.0;

        let assign9750_e9282: f64 = if locals.var_qsqrt < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard96 = assign9750_e9282;
        locals.var_guard96_rv = 0.0;

        let (assign9760_e9288, assign9760_e9288_d_n3, assign9760_e9288_d_n4, assign9760_e9288_d_n5, assign9760_e9288_d_n6, assign9760_e9288_d_n7, assign9760_e9288_d_n8,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9760_e9285: f64 = (-locals.var_qsqrt);
        let assign9760_e9286: f64 = (assign9760_e9285).sqrt();
        (assign9760_e9286, ((-locals.var_qsqrt_dn3) / (2.0 * assign9760_e9286)), ((-locals.var_qsqrt_dn4) / (2.0 * assign9760_e9286)), ((-locals.var_qsqrt_dn5) / (2.0 * assign9760_e9286)), ((-locals.var_qsqrt_dn6) / (2.0 * assign9760_e9286)), ((-locals.var_qsqrt_dn7) / (2.0 * assign9760_e9286)), ((-locals.var_qsqrt_dn8) / (2.0 * assign9760_e9286)),)
    } else {
        (locals.var_q, locals.var_q_dn3, locals.var_q_dn4, locals.var_q_dn5, locals.var_q_dn6, locals.var_q_dn7, locals.var_q_dn8,)
    }
};
        locals.var_q = assign9760_e9288;
        locals.var_q_dn3 = assign9760_e9288_d_n3;
        locals.var_q_dn4 = assign9760_e9288_d_n4;
        locals.var_q_dn5 = assign9760_e9288_d_n5;
        locals.var_q_dn6 = assign9760_e9288_d_n6;
        locals.var_q_dn7 = assign9760_e9288_d_n7;
        locals.var_q_dn8 = assign9760_e9288_d_n8;
        locals.var_q_rv = 0.0;

        let (assign9770_e9297, assign9770_e9297_d_n3, assign9770_e9297_d_n4, assign9770_e9297_d_n5, assign9770_e9297_d_n6, assign9770_e9297_d_n7, assign9770_e9297_d_n8,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9770_e9293: f64 = (0.5 * locals.var_q);
        let assign9770_e9294: f64 = (assign9770_e9293).sin();
        let assign9770_e9295: f64 = (1.0 / assign9770_e9294);
        (assign9770_e9295, (-(((assign9770_e9293).cos() * (0.5 * locals.var_q_dn3)) / (assign9770_e9294 * assign9770_e9294))), (-(((assign9770_e9293).cos() * (0.5 * locals.var_q_dn4)) / (assign9770_e9294 * assign9770_e9294))), (-(((assign9770_e9293).cos() * (0.5 * locals.var_q_dn5)) / (assign9770_e9294 * assign9770_e9294))), (-(((assign9770_e9293).cos() * (0.5 * locals.var_q_dn6)) / (assign9770_e9294 * assign9770_e9294))), (-(((assign9770_e9293).cos() * (0.5 * locals.var_q_dn7)) / (assign9770_e9294 * assign9770_e9294))), (-(((assign9770_e9293).cos() * (0.5 * locals.var_q_dn8)) / (assign9770_e9294 * assign9770_e9294))),)
    } else {
        (locals.var_csc1, locals.var_csc1_dn3, locals.var_csc1_dn4, locals.var_csc1_dn5, locals.var_csc1_dn6, locals.var_csc1_dn7, locals.var_csc1_dn8,)
    }
};
        locals.var_csc1 = assign9770_e9297;
        locals.var_csc1_dn3 = assign9770_e9297_d_n3;
        locals.var_csc1_dn4 = assign9770_e9297_d_n4;
        locals.var_csc1_dn5 = assign9770_e9297_d_n5;
        locals.var_csc1_dn6 = assign9770_e9297_d_n6;
        locals.var_csc1_dn7 = assign9770_e9297_d_n7;
        locals.var_csc1_dn8 = assign9770_e9297_d_n8;
        locals.var_csc1_rv = 0.0;

        let (assign9780_e9303, assign9780_e9303_d_n3, assign9780_e9303_d_n4, assign9780_e9303_d_n5, assign9780_e9303_d_n6, assign9780_e9303_d_n7, assign9780_e9303_d_n8,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9780_e9301: f64 = (locals.var_csc1 * locals.var_csc1);
        (assign9780_e9301, ((locals.var_csc1_dn3 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn3)), ((locals.var_csc1_dn4 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn4)), ((locals.var_csc1_dn5 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn5)), ((locals.var_csc1_dn6 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn6)), ((locals.var_csc1_dn7 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn7)), ((locals.var_csc1_dn8 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn8)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign9780_e9303;
        locals.var_t1_dn3 = assign9780_e9303_d_n3;
        locals.var_t1_dn4 = assign9780_e9303_d_n4;
        locals.var_t1_dn5 = assign9780_e9303_d_n5;
        locals.var_t1_dn6 = assign9780_e9303_d_n6;
        locals.var_t1_dn7 = assign9780_e9303_d_n7;
        locals.var_t1_dn8 = assign9780_e9303_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign9790_e9312, assign9790_e9312_d_n3, assign9790_e9312_d_n4, assign9790_e9312_d_n5, assign9790_e9312_d_n6, assign9790_e9312_d_n7, assign9790_e9312_d_n8,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9790_e9307: f64 = (0.5 * locals.var_q);
        let assign9790_e9308: f64 = (assign9790_e9307).cos();
        let assign9790_e9310: f64 = (assign9790_e9308 * locals.var_csc1);
        (assign9790_e9310, (((-(assign9790_e9307).sin() * (0.5 * locals.var_q_dn3)) * locals.var_csc1) + (assign9790_e9308 * locals.var_csc1_dn3)), (((-(assign9790_e9307).sin() * (0.5 * locals.var_q_dn4)) * locals.var_csc1) + (assign9790_e9308 * locals.var_csc1_dn4)), (((-(assign9790_e9307).sin() * (0.5 * locals.var_q_dn5)) * locals.var_csc1) + (assign9790_e9308 * locals.var_csc1_dn5)), (((-(assign9790_e9307).sin() * (0.5 * locals.var_q_dn6)) * locals.var_csc1) + (assign9790_e9308 * locals.var_csc1_dn6)), (((-(assign9790_e9307).sin() * (0.5 * locals.var_q_dn7)) * locals.var_csc1) + (assign9790_e9308 * locals.var_csc1_dn7)), (((-(assign9790_e9307).sin() * (0.5 * locals.var_q_dn8)) * locals.var_csc1) + (assign9790_e9308 * locals.var_csc1_dn8)),)
    } else {
        (locals.var_coth1, locals.var_coth1_dn3, locals.var_coth1_dn4, locals.var_coth1_dn5, locals.var_coth1_dn6, locals.var_coth1_dn7, locals.var_coth1_dn8,)
    }
};
        locals.var_coth1 = assign9790_e9312;
        locals.var_coth1_dn3 = assign9790_e9312_d_n3;
        locals.var_coth1_dn4 = assign9790_e9312_d_n4;
        locals.var_coth1_dn5 = assign9790_e9312_d_n5;
        locals.var_coth1_dn6 = assign9790_e9312_d_n6;
        locals.var_coth1_dn7 = assign9790_e9312_d_n7;
        locals.var_coth1_dn8 = assign9790_e9312_d_n8;
        locals.var_coth1_rv = 0.0;

        let (assign9800_e9321, assign9800_e9321_d_n3, assign9800_e9321_d_n4, assign9800_e9321_d_n5, assign9800_e9321_d_n6, assign9800_e9321_d_n7, assign9800_e9321_d_n8,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9800_e9315: f64 = (-0.5);
        let assign9800_e9317: f64 = (assign9800_e9315 * locals.var_coth1);
        let assign9800_e9319: f64 = (assign9800_e9317 / locals.var_q);
        (assign9800_e9319, ((((assign9800_e9315 * locals.var_coth1_dn3) * locals.var_q) - (assign9800_e9317 * locals.var_q_dn3)) / (locals.var_q * locals.var_q)), ((((assign9800_e9315 * locals.var_coth1_dn4) * locals.var_q) - (assign9800_e9317 * locals.var_q_dn4)) / (locals.var_q * locals.var_q)), ((((assign9800_e9315 * locals.var_coth1_dn5) * locals.var_q) - (assign9800_e9317 * locals.var_q_dn5)) / (locals.var_q * locals.var_q)), ((((assign9800_e9315 * locals.var_coth1_dn6) * locals.var_q) - (assign9800_e9317 * locals.var_q_dn6)) / (locals.var_q * locals.var_q)), ((((assign9800_e9315 * locals.var_coth1_dn7) * locals.var_q) - (assign9800_e9317 * locals.var_q_dn7)) / (locals.var_q * locals.var_q)), ((((assign9800_e9315 * locals.var_coth1_dn8) * locals.var_q) - (assign9800_e9317 * locals.var_q_dn8)) / (locals.var_q * locals.var_q)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    }
};
        locals.var_t0 = assign9800_e9321;
        locals.var_t0_dn3 = assign9800_e9321_d_n3;
        locals.var_t0_dn4 = assign9800_e9321_d_n4;
        locals.var_t0_dn5 = assign9800_e9321_d_n5;
        locals.var_t0_dn6 = assign9800_e9321_d_n6;
        locals.var_t0_dn7 = assign9800_e9321_d_n7;
        locals.var_t0_dn8 = assign9800_e9321_d_n8;
        locals.var_t0_rv = 0.0;

        let (assign9810_e9329, assign9810_e9329_d_n3, assign9810_e9329_d_n4, assign9810_e9329_d_n5, assign9810_e9329_d_n6, assign9810_e9329_d_n7, assign9810_e9329_d_n8,) = {
    if (locals.var_guard96 != 0.0) {
        let assign9810_e9325: f64 = (0.25 * locals.var_t1);
        let assign9810_e9327: f64 = (assign9810_e9325 + locals.var_t0);
        (assign9810_e9327, ((0.25 * locals.var_t1_dn3) + locals.var_t0_dn3), ((0.25 * locals.var_t1_dn4) + locals.var_t0_dn4), ((0.25 * locals.var_t1_dn5) + locals.var_t0_dn5), ((0.25 * locals.var_t1_dn6) + locals.var_t0_dn6), ((0.25 * locals.var_t1_dn7) + locals.var_t0_dn7), ((0.25 * locals.var_t1_dn8) + locals.var_t0_dn8),)
    } else {
        (locals.var_dqcothqdqsqrt, locals.var_dqcothqdqsqrt_dn3, locals.var_dqcothqdqsqrt_dn4, locals.var_dqcothqdqsqrt_dn5, locals.var_dqcothqdqsqrt_dn6, locals.var_dqcothqdqsqrt_dn7, locals.var_dqcothqdqsqrt_dn8,)
    }
};
        locals.var_dqcothqdqsqrt = assign9810_e9329;
        locals.var_dqcothqdqsqrt_dn3 = assign9810_e9329_d_n3;
        locals.var_dqcothqdqsqrt_dn4 = assign9810_e9329_d_n4;
        locals.var_dqcothqdqsqrt_dn5 = assign9810_e9329_d_n5;
        locals.var_dqcothqdqsqrt_dn6 = assign9810_e9329_d_n6;
        locals.var_dqcothqdqsqrt_dn7 = assign9810_e9329_d_n7;
        locals.var_dqcothqdqsqrt_dn8 = assign9810_e9329_d_n8;
        locals.var_dqcothqdqsqrt_rv = 0.0;

        let (assign9820_e9335, assign9820_e9335_d_n3, assign9820_e9335_d_n4, assign9820_e9335_d_n5, assign9820_e9335_d_n6, assign9820_e9335_d_n7, assign9820_e9335_d_n8,) = {
    if (locals.var_guard96 == 0.0) {
        let assign9820_e9333: f64 = (locals.var_qsqrt).sqrt();
        (assign9820_e9333, (locals.var_qsqrt_dn3 / (2.0 * assign9820_e9333)), (locals.var_qsqrt_dn4 / (2.0 * assign9820_e9333)), (locals.var_qsqrt_dn5 / (2.0 * assign9820_e9333)), (locals.var_qsqrt_dn6 / (2.0 * assign9820_e9333)), (locals.var_qsqrt_dn7 / (2.0 * assign9820_e9333)), (locals.var_qsqrt_dn8 / (2.0 * assign9820_e9333)),)
    } else {
        (locals.var_q, locals.var_q_dn3, locals.var_q_dn4, locals.var_q_dn5, locals.var_q_dn6, locals.var_q_dn7, locals.var_q_dn8,)
    }
};
        locals.var_q = assign9820_e9335;
        locals.var_q_dn3 = assign9820_e9335_d_n3;
        locals.var_q_dn4 = assign9820_e9335_d_n4;
        locals.var_q_dn5 = assign9820_e9335_d_n5;
        locals.var_q_dn6 = assign9820_e9335_d_n6;
        locals.var_q_dn7 = assign9820_e9335_d_n7;
        locals.var_q_dn8 = assign9820_e9335_d_n8;
        locals.var_q_rv = 0.0;

        let (assign9830_e9345, assign9830_e9345_d_n3, assign9830_e9345_d_n4, assign9830_e9345_d_n5, assign9830_e9345_d_n6, assign9830_e9345_d_n7, assign9830_e9345_d_n8,) = {
    if (locals.var_guard96 == 0.0) {
        let assign9830_e9341: f64 = (0.5 * locals.var_q);
        let assign9830_e9342: f64 = (assign9830_e9341).sinh();
        let assign9830_e9343: f64 = (1.0 / assign9830_e9342);
        (assign9830_e9343, (-(((assign9830_e9341).cosh() * (0.5 * locals.var_q_dn3)) / (assign9830_e9342 * assign9830_e9342))), (-(((assign9830_e9341).cosh() * (0.5 * locals.var_q_dn4)) / (assign9830_e9342 * assign9830_e9342))), (-(((assign9830_e9341).cosh() * (0.5 * locals.var_q_dn5)) / (assign9830_e9342 * assign9830_e9342))), (-(((assign9830_e9341).cosh() * (0.5 * locals.var_q_dn6)) / (assign9830_e9342 * assign9830_e9342))), (-(((assign9830_e9341).cosh() * (0.5 * locals.var_q_dn7)) / (assign9830_e9342 * assign9830_e9342))), (-(((assign9830_e9341).cosh() * (0.5 * locals.var_q_dn8)) / (assign9830_e9342 * assign9830_e9342))),)
    } else {
        (locals.var_csc1, locals.var_csc1_dn3, locals.var_csc1_dn4, locals.var_csc1_dn5, locals.var_csc1_dn6, locals.var_csc1_dn7, locals.var_csc1_dn8,)
    }
};
        locals.var_csc1 = assign9830_e9345;
        locals.var_csc1_dn3 = assign9830_e9345_d_n3;
        locals.var_csc1_dn4 = assign9830_e9345_d_n4;
        locals.var_csc1_dn5 = assign9830_e9345_d_n5;
        locals.var_csc1_dn6 = assign9830_e9345_d_n6;
        locals.var_csc1_dn7 = assign9830_e9345_d_n7;
        locals.var_csc1_dn8 = assign9830_e9345_d_n8;
        locals.var_csc1_rv = 0.0;

        let (assign9840_e9352, assign9840_e9352_d_n3, assign9840_e9352_d_n4, assign9840_e9352_d_n5, assign9840_e9352_d_n6, assign9840_e9352_d_n7, assign9840_e9352_d_n8,) = {
    if (locals.var_guard96 == 0.0) {
        let assign9840_e9350: f64 = (locals.var_csc1 * locals.var_csc1);
        (assign9840_e9350, ((locals.var_csc1_dn3 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn3)), ((locals.var_csc1_dn4 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn4)), ((locals.var_csc1_dn5 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn5)), ((locals.var_csc1_dn6 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn6)), ((locals.var_csc1_dn7 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn7)), ((locals.var_csc1_dn8 * locals.var_csc1) + (locals.var_csc1 * locals.var_csc1_dn8)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign9840_e9352;
        locals.var_t1_dn3 = assign9840_e9352_d_n3;
        locals.var_t1_dn4 = assign9840_e9352_d_n4;
        locals.var_t1_dn5 = assign9840_e9352_d_n5;
        locals.var_t1_dn6 = assign9840_e9352_d_n6;
        locals.var_t1_dn7 = assign9840_e9352_d_n7;
        locals.var_t1_dn8 = assign9840_e9352_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign9850_e9360, assign9850_e9360_d_n3, assign9850_e9360_d_n4, assign9850_e9360_d_n5, assign9850_e9360_d_n6, assign9850_e9360_d_n7, assign9850_e9360_d_n8,) = {
    if (locals.var_guard96 == 0.0) {
        let assign9850_e9357: f64 = (1.0 + locals.var_t1);
        let assign9850_e9358: f64 = (assign9850_e9357).sqrt();
        (assign9850_e9358, (locals.var_t1_dn3 / (2.0 * assign9850_e9358)), (locals.var_t1_dn4 / (2.0 * assign9850_e9358)), (locals.var_t1_dn5 / (2.0 * assign9850_e9358)), (locals.var_t1_dn6 / (2.0 * assign9850_e9358)), (locals.var_t1_dn7 / (2.0 * assign9850_e9358)), (locals.var_t1_dn8 / (2.0 * assign9850_e9358)),)
    } else {
        (locals.var_coth1, locals.var_coth1_dn3, locals.var_coth1_dn4, locals.var_coth1_dn5, locals.var_coth1_dn6, locals.var_coth1_dn7, locals.var_coth1_dn8,)
    }
};
        locals.var_coth1 = assign9850_e9360;
        locals.var_coth1_dn3 = assign9850_e9360_d_n3;
        locals.var_coth1_dn4 = assign9850_e9360_d_n4;
        locals.var_coth1_dn5 = assign9850_e9360_d_n5;
        locals.var_coth1_dn6 = assign9850_e9360_d_n6;
        locals.var_coth1_dn7 = assign9850_e9360_d_n7;
        locals.var_coth1_dn8 = assign9850_e9360_d_n8;
        locals.var_coth1_rv = 0.0;

        let (assign9860_e9369, assign9860_e9369_d_n3, assign9860_e9369_d_n4, assign9860_e9369_d_n5, assign9860_e9369_d_n6, assign9860_e9369_d_n7, assign9860_e9369_d_n8,) = {
    if (locals.var_guard96 == 0.0) {
        let assign9860_e9365: f64 = (0.5 * locals.var_coth1);
        let assign9860_e9367: f64 = (assign9860_e9365 / locals.var_q);
        (assign9860_e9367, ((((0.5 * locals.var_coth1_dn3) * locals.var_q) - (assign9860_e9365 * locals.var_q_dn3)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn4) * locals.var_q) - (assign9860_e9365 * locals.var_q_dn4)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn5) * locals.var_q) - (assign9860_e9365 * locals.var_q_dn5)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn6) * locals.var_q) - (assign9860_e9365 * locals.var_q_dn6)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn7) * locals.var_q) - (assign9860_e9365 * locals.var_q_dn7)) / (locals.var_q * locals.var_q)), ((((0.5 * locals.var_coth1_dn8) * locals.var_q) - (assign9860_e9365 * locals.var_q_dn8)) / (locals.var_q * locals.var_q)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    }
};
        locals.var_t0 = assign9860_e9369;
        locals.var_t0_dn3 = assign9860_e9369_d_n3;
        locals.var_t0_dn4 = assign9860_e9369_d_n4;
        locals.var_t0_dn5 = assign9860_e9369_d_n5;
        locals.var_t0_dn6 = assign9860_e9369_d_n6;
        locals.var_t0_dn7 = assign9860_e9369_d_n7;
        locals.var_t0_dn8 = assign9860_e9369_d_n8;
        locals.var_t0_rv = 0.0;

        let (assign9870_e9379, assign9870_e9379_d_n3, assign9870_e9379_d_n4, assign9870_e9379_d_n5, assign9870_e9379_d_n6, assign9870_e9379_d_n7, assign9870_e9379_d_n8,) = {
    if (locals.var_guard96 == 0.0) {
        let assign9870_e9373: f64 = (-0.25);
        let assign9870_e9375: f64 = (assign9870_e9373 * locals.var_t1);
        let assign9870_e9377: f64 = (assign9870_e9375 + locals.var_t0);
        (assign9870_e9377, ((assign9870_e9373 * locals.var_t1_dn3) + locals.var_t0_dn3), ((assign9870_e9373 * locals.var_t1_dn4) + locals.var_t0_dn4), ((assign9870_e9373 * locals.var_t1_dn5) + locals.var_t0_dn5), ((assign9870_e9373 * locals.var_t1_dn6) + locals.var_t0_dn6), ((assign9870_e9373 * locals.var_t1_dn7) + locals.var_t0_dn7), ((assign9870_e9373 * locals.var_t1_dn8) + locals.var_t0_dn8),)
    } else {
        (locals.var_dqcothqdqsqrt, locals.var_dqcothqdqsqrt_dn3, locals.var_dqcothqdqsqrt_dn4, locals.var_dqcothqdqsqrt_dn5, locals.var_dqcothqdqsqrt_dn6, locals.var_dqcothqdqsqrt_dn7, locals.var_dqcothqdqsqrt_dn8,)
    }
};
        locals.var_dqcothqdqsqrt = assign9870_e9379;
        locals.var_dqcothqdqsqrt_dn3 = assign9870_e9379_d_n3;
        locals.var_dqcothqdqsqrt_dn4 = assign9870_e9379_d_n4;
        locals.var_dqcothqdqsqrt_dn5 = assign9870_e9379_d_n5;
        locals.var_dqcothqdqsqrt_dn6 = assign9870_e9379_d_n6;
        locals.var_dqcothqdqsqrt_dn7 = assign9870_e9379_d_n7;
        locals.var_dqcothqdqsqrt_dn8 = assign9870_e9379_d_n8;
        locals.var_dqcothqdqsqrt_rv = 0.0;

        let assign9880_e9382: f64 = (locals.var_q * locals.var_coth1);
        locals.var_qcoth = assign9880_e9382;
        locals.var_qcoth_dn3 = ((locals.var_q_dn3 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn3));
        locals.var_qcoth_dn4 = ((locals.var_q_dn4 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn4));
        locals.var_qcoth_dn5 = ((locals.var_q_dn5 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn5));
        locals.var_qcoth_dn6 = ((locals.var_q_dn6 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn6));
        locals.var_qcoth_dn7 = ((locals.var_q_dn7 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn7));
        locals.var_qcoth_dn8 = ((locals.var_q_dn8 * locals.var_coth1) + (locals.var_q * locals.var_coth1_dn8));
        locals.var_qcoth_rv = 0.0;

        let assign9890_e9385: f64 = (locals.var_auxb1 + locals.var_qcoth);
        locals.var_t2 = assign9890_e9385;
        locals.var_t2_dn3 = (locals.var_auxb1_dn3 + locals.var_qcoth_dn3);
        locals.var_t2_dn4 = (locals.var_auxb1_dn4 + locals.var_qcoth_dn4);
        locals.var_t2_dn5 = (locals.var_auxb1_dn5 + locals.var_qcoth_dn5);
        locals.var_t2_dn6 = (locals.var_auxb1_dn6 + locals.var_qcoth_dn6);
        locals.var_t2_dn7 = (locals.var_auxb1_dn7 + locals.var_qcoth_dn7);
        locals.var_t2_dn8 = (locals.var_auxb1_dn8 + locals.var_qcoth_dn8);
        locals.var_t2_rv = 0.0;

        let assign9900_e9388: f64 = (1.0 / locals.var_t2);
        locals.var_t3 = assign9900_e9388;
        locals.var_t3_dn3 = (-(locals.var_t2_dn3 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn4 = (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn5 = (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn6 = (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn7 = (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_dn8 = (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2)));
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_22(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign9910_e9391: f64 = (locals.var_xg2 - locals.var_xg1);
        let assign9910_e9393: f64 = (assign9910_e9391 + locals.var_q1);
        let assign9910_e9396: f64 = (locals.var_qsqrt * locals.var_t1);
        let assign9910_e9398: f64 = (assign9910_e9396 * locals.var_t3);
        let assign9910_e9400: f64 = (assign9910_e9398 * locals.var_t3);
        let assign9910_e9401: f64 = (assign9910_e9400).abs();
        let assign9910_e9402: f64 = (assign9910_e9401).ln();
        let assign9910_e9403: f64 = (assign9910_e9393 - assign9910_e9402);
        locals.var_q2 = assign9910_e9403;
        locals.var_q2_dn3 = (((locals.var_xg2_dn3 - locals.var_xg1_dn3) + locals.var_q1_dn3) - (if assign9910_e9400 >= 0.0 { ((((((locals.var_qsqrt_dn3 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn3)) * locals.var_t3) + (assign9910_e9396 * locals.var_t3_dn3)) * locals.var_t3) + (assign9910_e9398 * locals.var_t3_dn3)) } else { (-((((((locals.var_qsqrt_dn3 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn3)) * locals.var_t3) + (assign9910_e9396 * locals.var_t3_dn3)) * locals.var_t3) + (assign9910_e9398 * locals.var_t3_dn3))) } / assign9910_e9401));
        locals.var_q2_dn4 = (((locals.var_xg2_dn4 - locals.var_xg1_dn4) + locals.var_q1_dn4) - (if assign9910_e9400 >= 0.0 { ((((((locals.var_qsqrt_dn4 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn4)) * locals.var_t3) + (assign9910_e9396 * locals.var_t3_dn4)) * locals.var_t3) + (assign9910_e9398 * locals.var_t3_dn4)) } else { (-((((((locals.var_qsqrt_dn4 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn4)) * locals.var_t3) + (assign9910_e9396 * locals.var_t3_dn4)) * locals.var_t3) + (assign9910_e9398 * locals.var_t3_dn4))) } / assign9910_e9401));
        locals.var_q2_dn5 = (((locals.var_xg2_dn5 - locals.var_xg1_dn5) + locals.var_q1_dn5) - (if assign9910_e9400 >= 0.0 { ((((((locals.var_qsqrt_dn5 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn5)) * locals.var_t3) + (assign9910_e9396 * locals.var_t3_dn5)) * locals.var_t3) + (assign9910_e9398 * locals.var_t3_dn5)) } else { (-((((((locals.var_qsqrt_dn5 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn5)) * locals.var_t3) + (assign9910_e9396 * locals.var_t3_dn5)) * locals.var_t3) + (assign9910_e9398 * locals.var_t3_dn5))) } / assign9910_e9401));
        locals.var_q2_dn6 = (((locals.var_xg2_dn6 - locals.var_xg1_dn6) + locals.var_q1_dn6) - (if assign9910_e9400 >= 0.0 { ((((((locals.var_qsqrt_dn6 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn6)) * locals.var_t3) + (assign9910_e9396 * locals.var_t3_dn6)) * locals.var_t3) + (assign9910_e9398 * locals.var_t3_dn6)) } else { (-((((((locals.var_qsqrt_dn6 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn6)) * locals.var_t3) + (assign9910_e9396 * locals.var_t3_dn6)) * locals.var_t3) + (assign9910_e9398 * locals.var_t3_dn6))) } / assign9910_e9401));
        locals.var_q2_dn7 = (((locals.var_xg2_dn7 - locals.var_xg1_dn7) + locals.var_q1_dn7) - (if assign9910_e9400 >= 0.0 { ((((((locals.var_qsqrt_dn7 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn7)) * locals.var_t3) + (assign9910_e9396 * locals.var_t3_dn7)) * locals.var_t3) + (assign9910_e9398 * locals.var_t3_dn7)) } else { (-((((((locals.var_qsqrt_dn7 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn7)) * locals.var_t3) + (assign9910_e9396 * locals.var_t3_dn7)) * locals.var_t3) + (assign9910_e9398 * locals.var_t3_dn7))) } / assign9910_e9401));
        locals.var_q2_dn8 = (((locals.var_xg2_dn8 - locals.var_xg1_dn8) + locals.var_q1_dn8) - (if assign9910_e9400 >= 0.0 { ((((((locals.var_qsqrt_dn8 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn8)) * locals.var_t3) + (assign9910_e9396 * locals.var_t3_dn8)) * locals.var_t3) + (assign9910_e9398 * locals.var_t3_dn8)) } else { (-((((((locals.var_qsqrt_dn8 * locals.var_t1) + (locals.var_qsqrt * locals.var_t1_dn8)) * locals.var_t3) + (assign9910_e9396 * locals.var_t3_dn8)) * locals.var_t3) + (assign9910_e9398 * locals.var_t3_dn8))) } / assign9910_e9401));
        locals.var_q2_rv = 0.0;

        let assign9920_e9407: f64 = (locals.var_auxb1 + locals.var_qcoth);
        let assign9920_e9410: f64 = (locals.var_k2 * locals.var_q2);
        let assign9920_e9412: f64 = (assign9920_e9410 + locals.var_auxb1);
        let assign9920_e9413: f64 = (assign9920_e9407 * assign9920_e9412);
        let assign9920_e9414: f64 = (locals.var_aaux + assign9920_e9413);
        locals.var_f = assign9920_e9414;
        locals.var_f_dn3 = (locals.var_aaux_dn3 + (((locals.var_auxb1_dn3 + locals.var_qcoth_dn3) * assign9920_e9412) + (assign9920_e9407 * ((locals.var_k2 * locals.var_q2_dn3) + locals.var_auxb1_dn3))));
        locals.var_f_dn4 = (locals.var_aaux_dn4 + (((locals.var_auxb1_dn4 + locals.var_qcoth_dn4) * assign9920_e9412) + (assign9920_e9407 * ((locals.var_k2 * locals.var_q2_dn4) + locals.var_auxb1_dn4))));
        locals.var_f_dn5 = (locals.var_aaux_dn5 + (((locals.var_auxb1_dn5 + locals.var_qcoth_dn5) * assign9920_e9412) + (assign9920_e9407 * ((locals.var_k2 * locals.var_q2_dn5) + locals.var_auxb1_dn5))));
        locals.var_f_dn6 = (locals.var_aaux_dn6 + (((locals.var_auxb1_dn6 + locals.var_qcoth_dn6) * assign9920_e9412) + (assign9920_e9407 * ((locals.var_k2 * locals.var_q2_dn6) + locals.var_auxb1_dn6))));
        locals.var_f_dn7 = (locals.var_aaux_dn7 + (((locals.var_auxb1_dn7 + locals.var_qcoth_dn7) * assign9920_e9412) + (assign9920_e9407 * ((locals.var_k2 * locals.var_q2_dn7) + locals.var_auxb1_dn7))));
        locals.var_f_dn8 = (locals.var_aaux_dn8 + (((locals.var_auxb1_dn8 + locals.var_qcoth_dn8) * assign9920_e9412) + (assign9920_e9407 * ((locals.var_k2 * locals.var_q2_dn8) + locals.var_auxb1_dn8))));
        locals.var_f_rv = 0.0;

        let assign9930_e9417: f64 = (1.0 / locals.var_qsqrt);
        let assign9930_e9419: f64 = (assign9930_e9417 - locals.var_t0);
        locals.var_dlogsinhqsqdqsqrt = assign9930_e9419;
        locals.var_dlogsinhqsqdqsqrt_dn3 = ((-(locals.var_qsqrt_dn3 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn3);
        locals.var_dlogsinhqsqdqsqrt_dn4 = ((-(locals.var_qsqrt_dn4 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn4);
        locals.var_dlogsinhqsqdqsqrt_dn5 = ((-(locals.var_qsqrt_dn5 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn5);
        locals.var_dlogsinhqsqdqsqrt_dn6 = ((-(locals.var_qsqrt_dn6 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn6);
        locals.var_dlogsinhqsqdqsqrt_dn7 = ((-(locals.var_qsqrt_dn7 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn7);
        locals.var_dlogsinhqsqdqsqrt_dn8 = ((-(locals.var_qsqrt_dn8 / (locals.var_qsqrt * locals.var_qsqrt))) - locals.var_t0_dn8);
        locals.var_dlogsinhqsqdqsqrt_rv = 0.0;

        let assign9940_e9421: f64 = (-2.0);
        let assign9940_e9423: f64 = (assign9940_e9421 * locals.var_k1);
        let assign9940_e9425: f64 = (assign9940_e9423 * locals.var_auxb1);
        let assign9940_e9427: f64 = (assign9940_e9425 + locals.var_aaux);
        locals.var_dqsqrt = assign9940_e9427;
        locals.var_dqsqrt_dn3 = ((assign9940_e9423 * locals.var_auxb1_dn3) + locals.var_aaux_dn3);
        locals.var_dqsqrt_dn4 = ((assign9940_e9423 * locals.var_auxb1_dn4) + locals.var_aaux_dn4);
        locals.var_dqsqrt_dn5 = ((assign9940_e9423 * locals.var_auxb1_dn5) + locals.var_aaux_dn5);
        locals.var_dqsqrt_dn6 = ((assign9940_e9423 * locals.var_auxb1_dn6) + locals.var_aaux_dn6);
        locals.var_dqsqrt_dn7 = ((assign9940_e9423 * locals.var_auxb1_dn7) + locals.var_aaux_dn7);
        locals.var_dqsqrt_dn8 = ((assign9940_e9423 * locals.var_auxb1_dn8) + locals.var_aaux_dn8);
        locals.var_dqsqrt_rv = 0.0;

        let assign9950_e9430: f64 = (locals.var_dqcothqdqsqrt * locals.var_dqsqrt);
        locals.var_dqcoth = assign9950_e9430;
        locals.var_dqcoth_dn3 = ((locals.var_dqcothqdqsqrt_dn3 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn3));
        locals.var_dqcoth_dn4 = ((locals.var_dqcothqdqsqrt_dn4 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn4));
        locals.var_dqcoth_dn5 = ((locals.var_dqcothqdqsqrt_dn5 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn5));
        locals.var_dqcoth_dn6 = ((locals.var_dqcothqdqsqrt_dn6 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn6));
        locals.var_dqcoth_dn7 = ((locals.var_dqcothqdqsqrt_dn7 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn7));
        locals.var_dqcoth_dn8 = ((locals.var_dqcothqdqsqrt_dn8 * locals.var_dqsqrt) + (locals.var_dqcothqdqsqrt * locals.var_dqsqrt_dn8));
        locals.var_dqcoth_rv = 0.0;

        let assign9960_e9432: f64 = (-1.0);
        let assign9960_e9435: f64 = (-locals.var_k1);
        let assign9960_e9437: f64 = (assign9960_e9435 + locals.var_dqcoth);
        let assign9960_e9439: f64 = (assign9960_e9437 * locals.var_t3);
        let assign9960_e9440: f64 = (2.0 * assign9960_e9439);
        let assign9960_e9441: f64 = (assign9960_e9432 + assign9960_e9440);
        let assign9960_e9444: f64 = (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt);
        let assign9960_e9445: f64 = (assign9960_e9441 - assign9960_e9444);
        locals.var_dq2 = assign9960_e9445;
        locals.var_dq2_dn3 = ((2.0 * ((locals.var_dqcoth_dn3 * locals.var_t3) + (assign9960_e9437 * locals.var_t3_dn3))) - ((locals.var_dlogsinhqsqdqsqrt_dn3 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn3)));
        locals.var_dq2_dn4 = ((2.0 * ((locals.var_dqcoth_dn4 * locals.var_t3) + (assign9960_e9437 * locals.var_t3_dn4))) - ((locals.var_dlogsinhqsqdqsqrt_dn4 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn4)));
        locals.var_dq2_dn5 = ((2.0 * ((locals.var_dqcoth_dn5 * locals.var_t3) + (assign9960_e9437 * locals.var_t3_dn5))) - ((locals.var_dlogsinhqsqdqsqrt_dn5 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn5)));
        locals.var_dq2_dn6 = ((2.0 * ((locals.var_dqcoth_dn6 * locals.var_t3) + (assign9960_e9437 * locals.var_t3_dn6))) - ((locals.var_dlogsinhqsqdqsqrt_dn6 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn6)));
        locals.var_dq2_dn7 = ((2.0 * ((locals.var_dqcoth_dn7 * locals.var_t3) + (assign9960_e9437 * locals.var_t3_dn7))) - ((locals.var_dlogsinhqsqdqsqrt_dn7 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn7)));
        locals.var_dq2_dn8 = ((2.0 * ((locals.var_dqcoth_dn8 * locals.var_t3) + (assign9960_e9437 * locals.var_t3_dn8))) - ((locals.var_dlogsinhqsqdqsqrt_dn8 * locals.var_dqsqrt) + (locals.var_dlogsinhqsqdqsqrt * locals.var_dqsqrt_dn8)));
        locals.var_dq2_rv = 0.0;

        let assign9970_e9450: f64 = (locals.var_auxb1 + locals.var_t2);
        let assign9970_e9451: f64 = (locals.var_k1 * assign9970_e9450);
        let assign9970_e9452: f64 = (locals.var_aaux - assign9970_e9451);
        let assign9970_e9455: f64 = (locals.var_auxb1 * locals.var_dqcoth);
        let assign9970_e9456: f64 = (assign9970_e9452 + assign9970_e9455);
        let assign9970_e9460: f64 = (locals.var_dq2 * locals.var_t2);
        let assign9970_e9464: f64 = (locals.var_dqcoth - locals.var_k1);
        let assign9970_e9465: f64 = (locals.var_q2 * assign9970_e9464);
        let assign9970_e9466: f64 = (assign9970_e9460 + assign9970_e9465);
        let assign9970_e9467: f64 = (locals.var_k2 * assign9970_e9466);
        let assign9970_e9468: f64 = (assign9970_e9456 + assign9970_e9467);
        locals.var_df = assign9970_e9468;
        locals.var_df_dn3 = (((locals.var_aaux_dn3 - (locals.var_k1 * (locals.var_auxb1_dn3 + locals.var_t2_dn3))) + ((locals.var_auxb1_dn3 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn3))) + (locals.var_k2 * (((locals.var_dq2_dn3 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn3)) + ((locals.var_q2_dn3 * assign9970_e9464) + (locals.var_q2 * locals.var_dqcoth_dn3)))));
        locals.var_df_dn4 = (((locals.var_aaux_dn4 - (locals.var_k1 * (locals.var_auxb1_dn4 + locals.var_t2_dn4))) + ((locals.var_auxb1_dn4 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn4))) + (locals.var_k2 * (((locals.var_dq2_dn4 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn4)) + ((locals.var_q2_dn4 * assign9970_e9464) + (locals.var_q2 * locals.var_dqcoth_dn4)))));
        locals.var_df_dn5 = (((locals.var_aaux_dn5 - (locals.var_k1 * (locals.var_auxb1_dn5 + locals.var_t2_dn5))) + ((locals.var_auxb1_dn5 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn5))) + (locals.var_k2 * (((locals.var_dq2_dn5 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn5)) + ((locals.var_q2_dn5 * assign9970_e9464) + (locals.var_q2 * locals.var_dqcoth_dn5)))));
        locals.var_df_dn6 = (((locals.var_aaux_dn6 - (locals.var_k1 * (locals.var_auxb1_dn6 + locals.var_t2_dn6))) + ((locals.var_auxb1_dn6 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn6))) + (locals.var_k2 * (((locals.var_dq2_dn6 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn6)) + ((locals.var_q2_dn6 * assign9970_e9464) + (locals.var_q2 * locals.var_dqcoth_dn6)))));
        locals.var_df_dn7 = (((locals.var_aaux_dn7 - (locals.var_k1 * (locals.var_auxb1_dn7 + locals.var_t2_dn7))) + ((locals.var_auxb1_dn7 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn7))) + (locals.var_k2 * (((locals.var_dq2_dn7 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn7)) + ((locals.var_q2_dn7 * assign9970_e9464) + (locals.var_q2 * locals.var_dqcoth_dn7)))));
        locals.var_df_dn8 = (((locals.var_aaux_dn8 - (locals.var_k1 * (locals.var_auxb1_dn8 + locals.var_t2_dn8))) + ((locals.var_auxb1_dn8 * locals.var_dqcoth) + (locals.var_auxb1 * locals.var_dqcoth_dn8))) + (locals.var_k2 * (((locals.var_dq2_dn8 * locals.var_t2) + (locals.var_dq2 * locals.var_t2_dn8)) + ((locals.var_q2_dn8 * assign9970_e9464) + (locals.var_q2 * locals.var_dqcoth_dn8)))));
        locals.var_df_rv = 0.0;

        let assign9980_e9470: f64 = (-locals.var_f);
        let assign9980_e9472: f64 = (assign9980_e9470 / locals.var_df);
        locals.var_delta = assign9980_e9472;
        locals.var_delta_dn3 = ((((-locals.var_f_dn3) * locals.var_df) - (assign9980_e9470 * locals.var_df_dn3)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn4 = ((((-locals.var_f_dn4) * locals.var_df) - (assign9980_e9470 * locals.var_df_dn4)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn5 = ((((-locals.var_f_dn5) * locals.var_df) - (assign9980_e9470 * locals.var_df_dn5)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn6 = ((((-locals.var_f_dn6) * locals.var_df) - (assign9980_e9470 * locals.var_df_dn6)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn7 = ((((-locals.var_f_dn7) * locals.var_df) - (assign9980_e9470 * locals.var_df_dn7)) / (locals.var_df * locals.var_df));
        locals.var_delta_dn8 = ((((-locals.var_f_dn8) * locals.var_df) - (assign9980_e9470 * locals.var_df_dn8)) / (locals.var_df * locals.var_df));
        locals.var_delta_rv = 0.0;

        let assign9990_e9475: f64 = (locals.var_phi1 + locals.var_delta);
        locals.var_phi1 = assign9990_e9475;
        locals.var_phi1_dn3 = (locals.var_phi1_dn3 + locals.var_delta_dn3);
        locals.var_phi1_dn4 = (locals.var_phi1_dn4 + locals.var_delta_dn4);
        locals.var_phi1_dn5 = (locals.var_phi1_dn5 + locals.var_delta_dn5);
        locals.var_phi1_dn6 = (locals.var_phi1_dn6 + locals.var_delta_dn6);
        locals.var_phi1_dn7 = (locals.var_phi1_dn7 + locals.var_delta_dn7);
        locals.var_phi1_dn8 = (locals.var_phi1_dn8 + locals.var_delta_dn8);
        locals.var_phi1_rv = 0.0;

        let assign10000_e9478: f64 = (locals.var_xg1 - locals.var_phi1);
        locals.var_q1 = assign10000_e9478;
        locals.var_q1_dn3 = (locals.var_xg1_dn3 - locals.var_phi1_dn3);
        locals.var_q1_dn4 = (locals.var_xg1_dn4 - locals.var_phi1_dn4);
        locals.var_q1_dn5 = (locals.var_xg1_dn5 - locals.var_phi1_dn5);
        locals.var_q1_dn6 = (locals.var_xg1_dn6 - locals.var_phi1_dn6);
        locals.var_q1_dn7 = (locals.var_xg1_dn7 - locals.var_phi1_dn7);
        locals.var_q1_dn8 = (locals.var_xg1_dn8 - locals.var_phi1_dn8);
        locals.var_q1_rv = 0.0;

        let assign10010_e9481: f64 = (locals.var_phi1).exp();
        let assign10010_e9482: f64 = (locals.var_a0 * assign10010_e9481);
        locals.var_t0 = assign10010_e9482;
        locals.var_t0_dn3 = ((locals.var_a0_dn3 * assign10010_e9481) + (locals.var_a0 * (assign10010_e9481 * locals.var_phi1_dn3)));
        locals.var_t0_dn4 = ((locals.var_a0_dn4 * assign10010_e9481) + (locals.var_a0 * (assign10010_e9481 * locals.var_phi1_dn4)));
        locals.var_t0_dn5 = ((locals.var_a0_dn5 * assign10010_e9481) + (locals.var_a0 * (assign10010_e9481 * locals.var_phi1_dn5)));
        locals.var_t0_dn6 = ((locals.var_a0_dn6 * assign10010_e9481) + (locals.var_a0 * (assign10010_e9481 * locals.var_phi1_dn6)));
        locals.var_t0_dn7 = ((locals.var_a0_dn7 * assign10010_e9481) + (locals.var_a0 * (assign10010_e9481 * locals.var_phi1_dn7)));
        locals.var_t0_dn8 = ((locals.var_a0_dn8 * assign10010_e9481) + (locals.var_a0 * (assign10010_e9481 * locals.var_phi1_dn8)));
        locals.var_t0_rv = 0.0;

        let assign10020_e9485: f64 = (locals.var_k1_2 * locals.var_q1);
        let assign10020_e9487: f64 = (assign10020_e9485 * locals.var_q1);
        let assign10020_e9489: f64 = (assign10020_e9487 - locals.var_t0);
        locals.var_qsqrt = assign10020_e9489;
        locals.var_qsqrt_dn3 = ((((locals.var_k1_2 * locals.var_q1_dn3) * locals.var_q1) + (assign10020_e9485 * locals.var_q1_dn3)) - locals.var_t0_dn3);
        locals.var_qsqrt_dn4 = ((((locals.var_k1_2 * locals.var_q1_dn4) * locals.var_q1) + (assign10020_e9485 * locals.var_q1_dn4)) - locals.var_t0_dn4);
        locals.var_qsqrt_dn5 = ((((locals.var_k1_2 * locals.var_q1_dn5) * locals.var_q1) + (assign10020_e9485 * locals.var_q1_dn5)) - locals.var_t0_dn5);
        locals.var_qsqrt_dn6 = ((((locals.var_k1_2 * locals.var_q1_dn6) * locals.var_q1) + (assign10020_e9485 * locals.var_q1_dn6)) - locals.var_t0_dn6);
        locals.var_qsqrt_dn7 = ((((locals.var_k1_2 * locals.var_q1_dn7) * locals.var_q1) + (assign10020_e9485 * locals.var_q1_dn7)) - locals.var_t0_dn7);
        locals.var_qsqrt_dn8 = ((((locals.var_k1_2 * locals.var_q1_dn8) * locals.var_q1) + (assign10020_e9485 * locals.var_q1_dn8)) - locals.var_t0_dn8);
        locals.var_qsqrt_rv = 0.0;

        let assign10030_e9492: f64 = if locals.var_qsqrt < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard97 = assign10030_e9492;
        locals.var_guard97_rv = 0.0;

        let (assign10040_e9498, assign10040_e9498_d_n3, assign10040_e9498_d_n4, assign10040_e9498_d_n5, assign10040_e9498_d_n6, assign10040_e9498_d_n7, assign10040_e9498_d_n8,) = {
    if (locals.var_guard97 != 0.0) {
        let assign10040_e9495: f64 = (-locals.var_qsqrt);
        let assign10040_e9496: f64 = (assign10040_e9495).sqrt();
        (assign10040_e9496, ((-locals.var_qsqrt_dn3) / (2.0 * assign10040_e9496)), ((-locals.var_qsqrt_dn4) / (2.0 * assign10040_e9496)), ((-locals.var_qsqrt_dn5) / (2.0 * assign10040_e9496)), ((-locals.var_qsqrt_dn6) / (2.0 * assign10040_e9496)), ((-locals.var_qsqrt_dn7) / (2.0 * assign10040_e9496)), ((-locals.var_qsqrt_dn8) / (2.0 * assign10040_e9496)),)
    } else {
        (locals.var_q, locals.var_q_dn3, locals.var_q_dn4, locals.var_q_dn5, locals.var_q_dn6, locals.var_q_dn7, locals.var_q_dn8,)
    }
};
        locals.var_q = assign10040_e9498;
        locals.var_q_dn3 = assign10040_e9498_d_n3;
        locals.var_q_dn4 = assign10040_e9498_d_n4;
        locals.var_q_dn5 = assign10040_e9498_d_n5;
        locals.var_q_dn6 = assign10040_e9498_d_n6;
        locals.var_q_dn7 = assign10040_e9498_d_n7;
        locals.var_q_dn8 = assign10040_e9498_d_n8;
        locals.var_q_rv = 0.0;

        let (assign10050_e9504, assign10050_e9504_d_n3, assign10050_e9504_d_n4, assign10050_e9504_d_n5, assign10050_e9504_d_n6, assign10050_e9504_d_n7, assign10050_e9504_d_n8,) = {
    if (locals.var_guard97 != 0.0) {
        let assign10050_e9502: f64 = (0.5 * locals.var_q);
        (assign10050_e9502, (0.5 * locals.var_q_dn3), (0.5 * locals.var_q_dn4), (0.5 * locals.var_q_dn5), (0.5 * locals.var_q_dn6), (0.5 * locals.var_q_dn7), (0.5 * locals.var_q_dn8),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8,)
    }
};
        locals.var_t2 = assign10050_e9504;
        locals.var_t2_dn3 = assign10050_e9504_d_n3;
        locals.var_t2_dn4 = assign10050_e9504_d_n4;
        locals.var_t2_dn5 = assign10050_e9504_d_n5;
        locals.var_t2_dn6 = assign10050_e9504_d_n6;
        locals.var_t2_dn7 = assign10050_e9504_d_n7;
        locals.var_t2_dn8 = assign10050_e9504_d_n8;
        locals.var_t2_rv = 0.0;

        let (assign10060_e9511, assign10060_e9511_d_n3, assign10060_e9511_d_n4, assign10060_e9511_d_n5, assign10060_e9511_d_n6, assign10060_e9511_d_n7, assign10060_e9511_d_n8,) = {
    if (locals.var_guard97 != 0.0) {
        let assign10060_e9508: f64 = (locals.var_t2).tan();
        let assign10060_e9509: f64 = (locals.var_q / assign10060_e9508);
        (assign10060_e9509, (((locals.var_q_dn3 * assign10060_e9508) - (locals.var_q * (locals.var_t2_dn3 / ((locals.var_t2).cos() * (locals.var_t2).cos())))) / (assign10060_e9508 * assign10060_e9508)), (((locals.var_q_dn4 * assign10060_e9508) - (locals.var_q * (locals.var_t2_dn4 / ((locals.var_t2).cos() * (locals.var_t2).cos())))) / (assign10060_e9508 * assign10060_e9508)), (((locals.var_q_dn5 * assign10060_e9508) - (locals.var_q * (locals.var_t2_dn5 / ((locals.var_t2).cos() * (locals.var_t2).cos())))) / (assign10060_e9508 * assign10060_e9508)), (((locals.var_q_dn6 * assign10060_e9508) - (locals.var_q * (locals.var_t2_dn6 / ((locals.var_t2).cos() * (locals.var_t2).cos())))) / (assign10060_e9508 * assign10060_e9508)), (((locals.var_q_dn7 * assign10060_e9508) - (locals.var_q * (locals.var_t2_dn7 / ((locals.var_t2).cos() * (locals.var_t2).cos())))) / (assign10060_e9508 * assign10060_e9508)), (((locals.var_q_dn8 * assign10060_e9508) - (locals.var_q * (locals.var_t2_dn8 / ((locals.var_t2).cos() * (locals.var_t2).cos())))) / (assign10060_e9508 * assign10060_e9508)),)
    } else {
        (locals.var_qcoth, locals.var_qcoth_dn3, locals.var_qcoth_dn4, locals.var_qcoth_dn5, locals.var_qcoth_dn6, locals.var_qcoth_dn7, locals.var_qcoth_dn8,)
    }
};
        locals.var_qcoth = assign10060_e9511;
        locals.var_qcoth_dn3 = assign10060_e9511_d_n3;
        locals.var_qcoth_dn4 = assign10060_e9511_d_n4;
        locals.var_qcoth_dn5 = assign10060_e9511_d_n5;
        locals.var_qcoth_dn6 = assign10060_e9511_d_n6;
        locals.var_qcoth_dn7 = assign10060_e9511_d_n7;
        locals.var_qcoth_dn8 = assign10060_e9511_d_n8;
        locals.var_qcoth_rv = 0.0;

        let (assign10070_e9516, assign10070_e9516_d_n3, assign10070_e9516_d_n4, assign10070_e9516_d_n5, assign10070_e9516_d_n6, assign10070_e9516_d_n7, assign10070_e9516_d_n8,) = {
    if (locals.var_guard97 != 0.0) {
        let assign10070_e9514: f64 = (locals.var_t2).sin();
        (assign10070_e9514, ((locals.var_t2).cos() * locals.var_t2_dn3), ((locals.var_t2).cos() * locals.var_t2_dn4), ((locals.var_t2).cos() * locals.var_t2_dn5), ((locals.var_t2).cos() * locals.var_t2_dn6), ((locals.var_t2).cos() * locals.var_t2_dn7), ((locals.var_t2).cos() * locals.var_t2_dn8),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8,)
    }
};
        locals.var_t6 = assign10070_e9516;
        locals.var_t6_dn3 = assign10070_e9516_d_n3;
        locals.var_t6_dn4 = assign10070_e9516_d_n4;
        locals.var_t6_dn5 = assign10070_e9516_d_n5;
        locals.var_t6_dn6 = assign10070_e9516_d_n6;
        locals.var_t6_dn7 = assign10070_e9516_d_n7;
        locals.var_t6_dn8 = assign10070_e9516_d_n8;
        locals.var_t6_rv = 0.0;

        let (assign10080_e9523, assign10080_e9523_d_n3, assign10080_e9523_d_n4, assign10080_e9523_d_n5, assign10080_e9523_d_n6, assign10080_e9523_d_n7, assign10080_e9523_d_n8,) = {
    if (locals.var_guard97 != 0.0) {
        let assign10080_e9519: f64 = (-locals.var_t6);
        let assign10080_e9521: f64 = (assign10080_e9519 * locals.var_t6);
        (assign10080_e9521, (((-locals.var_t6_dn3) * locals.var_t6) + (assign10080_e9519 * locals.var_t6_dn3)), (((-locals.var_t6_dn4) * locals.var_t6) + (assign10080_e9519 * locals.var_t6_dn4)), (((-locals.var_t6_dn5) * locals.var_t6) + (assign10080_e9519 * locals.var_t6_dn5)), (((-locals.var_t6_dn6) * locals.var_t6) + (assign10080_e9519 * locals.var_t6_dn6)), (((-locals.var_t6_dn7) * locals.var_t6) + (assign10080_e9519 * locals.var_t6_dn7)), (((-locals.var_t6_dn8) * locals.var_t6) + (assign10080_e9519 * locals.var_t6_dn8)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign10080_e9523;
        locals.var_t1_dn3 = assign10080_e9523_d_n3;
        locals.var_t1_dn4 = assign10080_e9523_d_n4;
        locals.var_t1_dn5 = assign10080_e9523_d_n5;
        locals.var_t1_dn6 = assign10080_e9523_d_n6;
        locals.var_t1_dn7 = assign10080_e9523_d_n7;
        locals.var_t1_dn8 = assign10080_e9523_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign10090_e9529, assign10090_e9529_d_n3, assign10090_e9529_d_n4, assign10090_e9529_d_n5, assign10090_e9529_d_n6, assign10090_e9529_d_n7, assign10090_e9529_d_n8,) = {
    if (locals.var_guard97 == 0.0) {
        let assign10090_e9527: f64 = (locals.var_qsqrt).sqrt();
        (assign10090_e9527, (locals.var_qsqrt_dn3 / (2.0 * assign10090_e9527)), (locals.var_qsqrt_dn4 / (2.0 * assign10090_e9527)), (locals.var_qsqrt_dn5 / (2.0 * assign10090_e9527)), (locals.var_qsqrt_dn6 / (2.0 * assign10090_e9527)), (locals.var_qsqrt_dn7 / (2.0 * assign10090_e9527)), (locals.var_qsqrt_dn8 / (2.0 * assign10090_e9527)),)
    } else {
        (locals.var_q, locals.var_q_dn3, locals.var_q_dn4, locals.var_q_dn5, locals.var_q_dn6, locals.var_q_dn7, locals.var_q_dn8,)
    }
};
        locals.var_q = assign10090_e9529;
        locals.var_q_dn3 = assign10090_e9529_d_n3;
        locals.var_q_dn4 = assign10090_e9529_d_n4;
        locals.var_q_dn5 = assign10090_e9529_d_n5;
        locals.var_q_dn6 = assign10090_e9529_d_n6;
        locals.var_q_dn7 = assign10090_e9529_d_n7;
        locals.var_q_dn8 = assign10090_e9529_d_n8;
        locals.var_q_rv = 0.0;

        let (assign10100_e9536, assign10100_e9536_d_n3, assign10100_e9536_d_n4, assign10100_e9536_d_n5, assign10100_e9536_d_n6, assign10100_e9536_d_n7, assign10100_e9536_d_n8,) = {
    if (locals.var_guard97 == 0.0) {
        let assign10100_e9534: f64 = (0.5 * locals.var_q);
        (assign10100_e9534, (0.5 * locals.var_q_dn3), (0.5 * locals.var_q_dn4), (0.5 * locals.var_q_dn5), (0.5 * locals.var_q_dn6), (0.5 * locals.var_q_dn7), (0.5 * locals.var_q_dn8),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8,)
    }
};
        locals.var_t2 = assign10100_e9536;
        locals.var_t2_dn3 = assign10100_e9536_d_n3;
        locals.var_t2_dn4 = assign10100_e9536_d_n4;
        locals.var_t2_dn5 = assign10100_e9536_d_n5;
        locals.var_t2_dn6 = assign10100_e9536_d_n6;
        locals.var_t2_dn7 = assign10100_e9536_d_n7;
        locals.var_t2_dn8 = assign10100_e9536_d_n8;
        locals.var_t2_rv = 0.0;

        let (assign10110_e9542, assign10110_e9542_d_n3, assign10110_e9542_d_n4, assign10110_e9542_d_n5, assign10110_e9542_d_n6, assign10110_e9542_d_n7, assign10110_e9542_d_n8,) = {
    if (locals.var_guard97 == 0.0) {
        let assign10110_e9540: f64 = (locals.var_t2).sinh();
        (assign10110_e9540, ((locals.var_t2).cosh() * locals.var_t2_dn3), ((locals.var_t2).cosh() * locals.var_t2_dn4), ((locals.var_t2).cosh() * locals.var_t2_dn5), ((locals.var_t2).cosh() * locals.var_t2_dn6), ((locals.var_t2).cosh() * locals.var_t2_dn7), ((locals.var_t2).cosh() * locals.var_t2_dn8),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8,)
    }
};
        locals.var_t6 = assign10110_e9542;
        locals.var_t6_dn3 = assign10110_e9542_d_n3;
        locals.var_t6_dn4 = assign10110_e9542_d_n4;
        locals.var_t6_dn5 = assign10110_e9542_d_n5;
        locals.var_t6_dn6 = assign10110_e9542_d_n6;
        locals.var_t6_dn7 = assign10110_e9542_d_n7;
        locals.var_t6_dn8 = assign10110_e9542_d_n8;
        locals.var_t6_rv = 0.0;

        let (assign10120_e9549, assign10120_e9549_d_n3, assign10120_e9549_d_n4, assign10120_e9549_d_n5, assign10120_e9549_d_n6, assign10120_e9549_d_n7, assign10120_e9549_d_n8,) = {
    if (locals.var_guard97 == 0.0) {
        let assign10120_e9547: f64 = (locals.var_t6 * locals.var_t6);
        (assign10120_e9547, ((locals.var_t6_dn3 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn3)), ((locals.var_t6_dn4 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn4)), ((locals.var_t6_dn5 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn5)), ((locals.var_t6_dn6 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn6)), ((locals.var_t6_dn7 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn7)), ((locals.var_t6_dn8 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn8)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign10120_e9549;
        locals.var_t1_dn3 = assign10120_e9549_d_n3;
        locals.var_t1_dn4 = assign10120_e9549_d_n4;
        locals.var_t1_dn5 = assign10120_e9549_d_n5;
        locals.var_t1_dn6 = assign10120_e9549_d_n6;
        locals.var_t1_dn7 = assign10120_e9549_d_n7;
        locals.var_t1_dn8 = assign10120_e9549_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign10130_e9557, assign10130_e9557_d_n3, assign10130_e9557_d_n4, assign10130_e9557_d_n5, assign10130_e9557_d_n6, assign10130_e9557_d_n7, assign10130_e9557_d_n8,) = {
    if (locals.var_guard97 == 0.0) {
        let assign10130_e9554: f64 = (locals.var_t2).tanh();
        let assign10130_e9555: f64 = (locals.var_q / assign10130_e9554);
        (assign10130_e9555, (((locals.var_q_dn3 * assign10130_e9554) - (locals.var_q * (locals.var_t2_dn3 / ((locals.var_t2).cosh() * (locals.var_t2).cosh())))) / (assign10130_e9554 * assign10130_e9554)), (((locals.var_q_dn4 * assign10130_e9554) - (locals.var_q * (locals.var_t2_dn4 / ((locals.var_t2).cosh() * (locals.var_t2).cosh())))) / (assign10130_e9554 * assign10130_e9554)), (((locals.var_q_dn5 * assign10130_e9554) - (locals.var_q * (locals.var_t2_dn5 / ((locals.var_t2).cosh() * (locals.var_t2).cosh())))) / (assign10130_e9554 * assign10130_e9554)), (((locals.var_q_dn6 * assign10130_e9554) - (locals.var_q * (locals.var_t2_dn6 / ((locals.var_t2).cosh() * (locals.var_t2).cosh())))) / (assign10130_e9554 * assign10130_e9554)), (((locals.var_q_dn7 * assign10130_e9554) - (locals.var_q * (locals.var_t2_dn7 / ((locals.var_t2).cosh() * (locals.var_t2).cosh())))) / (assign10130_e9554 * assign10130_e9554)), (((locals.var_q_dn8 * assign10130_e9554) - (locals.var_q * (locals.var_t2_dn8 / ((locals.var_t2).cosh() * (locals.var_t2).cosh())))) / (assign10130_e9554 * assign10130_e9554)),)
    } else {
        (locals.var_qcoth, locals.var_qcoth_dn3, locals.var_qcoth_dn4, locals.var_qcoth_dn5, locals.var_qcoth_dn6, locals.var_qcoth_dn7, locals.var_qcoth_dn8,)
    }
};
        locals.var_qcoth = assign10130_e9557;
        locals.var_qcoth_dn3 = assign10130_e9557_d_n3;
        locals.var_qcoth_dn4 = assign10130_e9557_d_n4;
        locals.var_qcoth_dn5 = assign10130_e9557_d_n5;
        locals.var_qcoth_dn6 = assign10130_e9557_d_n6;
        locals.var_qcoth_dn7 = assign10130_e9557_d_n7;
        locals.var_qcoth_dn8 = assign10130_e9557_d_n8;
        locals.var_qcoth_rv = 0.0;

        let assign10140_e9560: f64 = (locals.var_k1 * locals.var_q1);
        let assign10140_e9562: f64 = (assign10140_e9560 - locals.var_qcoth);
        let assign10140_e9567: f64 = (locals.var_t1 * locals.var_t0);
        let assign10140_e9568: f64 = (locals.var_qsqrt / assign10140_e9567);
        let assign10140_e9569: f64 = (1.0 - assign10140_e9568);
        let assign10140_e9570: f64 = (assign10140_e9562 / assign10140_e9569);
        locals.var_qicored = assign10140_e9570;
        locals.var_qicored_dn3 = (((((locals.var_k1 * locals.var_q1_dn3) - locals.var_qcoth_dn3) * assign10140_e9569) - (assign10140_e9562 * (-(((locals.var_qsqrt_dn3 * assign10140_e9567) - (locals.var_qsqrt * ((locals.var_t1_dn3 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn3)))) / (assign10140_e9567 * assign10140_e9567))))) / (assign10140_e9569 * assign10140_e9569));
        locals.var_qicored_dn4 = (((((locals.var_k1 * locals.var_q1_dn4) - locals.var_qcoth_dn4) * assign10140_e9569) - (assign10140_e9562 * (-(((locals.var_qsqrt_dn4 * assign10140_e9567) - (locals.var_qsqrt * ((locals.var_t1_dn4 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn4)))) / (assign10140_e9567 * assign10140_e9567))))) / (assign10140_e9569 * assign10140_e9569));
        locals.var_qicored_dn5 = (((((locals.var_k1 * locals.var_q1_dn5) - locals.var_qcoth_dn5) * assign10140_e9569) - (assign10140_e9562 * (-(((locals.var_qsqrt_dn5 * assign10140_e9567) - (locals.var_qsqrt * ((locals.var_t1_dn5 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn5)))) / (assign10140_e9567 * assign10140_e9567))))) / (assign10140_e9569 * assign10140_e9569));
        locals.var_qicored_dn6 = (((((locals.var_k1 * locals.var_q1_dn6) - locals.var_qcoth_dn6) * assign10140_e9569) - (assign10140_e9562 * (-(((locals.var_qsqrt_dn6 * assign10140_e9567) - (locals.var_qsqrt * ((locals.var_t1_dn6 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn6)))) / (assign10140_e9567 * assign10140_e9567))))) / (assign10140_e9569 * assign10140_e9569));
        locals.var_qicored_dn7 = (((((locals.var_k1 * locals.var_q1_dn7) - locals.var_qcoth_dn7) * assign10140_e9569) - (assign10140_e9562 * (-(((locals.var_qsqrt_dn7 * assign10140_e9567) - (locals.var_qsqrt * ((locals.var_t1_dn7 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn7)))) / (assign10140_e9567 * assign10140_e9567))))) / (assign10140_e9569 * assign10140_e9569));
        locals.var_qicored_dn8 = (((((locals.var_k1 * locals.var_q1_dn8) - locals.var_qcoth_dn8) * assign10140_e9569) - (assign10140_e9562 * (-(((locals.var_qsqrt_dn8 * assign10140_e9567) - (locals.var_qsqrt * ((locals.var_t1_dn8 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn8)))) / (assign10140_e9567 * assign10140_e9567))))) / (assign10140_e9569 * assign10140_e9569));
        locals.var_qicored_rv = 0.0;

        let assign10150_e9573: f64 = (locals.var_q1 * locals.var_cox1);
        let assign10150_e9575: f64 = (assign10150_e9573 * locals.var_nvtm);
        locals.var_qfrontd = assign10150_e9575;
        locals.var_qfrontd_dn3 = (((locals.var_q1_dn3 * locals.var_cox1) * locals.var_nvtm) + (assign10150_e9573 * locals.var_nvtm_dn3));
        locals.var_qfrontd_dn4 = (((locals.var_q1_dn4 * locals.var_cox1) * locals.var_nvtm) + (assign10150_e9573 * locals.var_nvtm_dn4));
        locals.var_qfrontd_dn5 = (((locals.var_q1_dn5 * locals.var_cox1) * locals.var_nvtm) + (assign10150_e9573 * locals.var_nvtm_dn5));
        locals.var_qfrontd_dn6 = (((locals.var_q1_dn6 * locals.var_cox1) * locals.var_nvtm) + (assign10150_e9573 * locals.var_nvtm_dn6));
        locals.var_qfrontd_dn7 = (((locals.var_q1_dn7 * locals.var_cox1) * locals.var_nvtm) + (assign10150_e9573 * locals.var_nvtm_dn7));
        locals.var_qfrontd_dn8 = (((locals.var_q1_dn8 * locals.var_cox1) * locals.var_nvtm) + (assign10150_e9573 * locals.var_nvtm_dn8));
        locals.var_qfrontd_rv = 0.0;

        let assign10160_e9578: f64 = (locals.var_qicored * locals.var_csi);
        let assign10160_e9580: f64 = (assign10160_e9578 * locals.var_nvtm);
        locals.var_qtotd = assign10160_e9580;
        locals.var_qtotd_dn3 = (((locals.var_qicored_dn3 * locals.var_csi) * locals.var_nvtm) + (assign10160_e9578 * locals.var_nvtm_dn3));
        locals.var_qtotd_dn4 = (((locals.var_qicored_dn4 * locals.var_csi) * locals.var_nvtm) + (assign10160_e9578 * locals.var_nvtm_dn4));
        locals.var_qtotd_dn5 = (((locals.var_qicored_dn5 * locals.var_csi) * locals.var_nvtm) + (assign10160_e9578 * locals.var_nvtm_dn5));
        locals.var_qtotd_dn6 = (((locals.var_qicored_dn6 * locals.var_csi) * locals.var_nvtm) + (assign10160_e9578 * locals.var_nvtm_dn6));
        locals.var_qtotd_dn7 = (((locals.var_qicored_dn7 * locals.var_csi) * locals.var_nvtm) + (assign10160_e9578 * locals.var_nvtm_dn7));
        locals.var_qtotd_dn8 = (((locals.var_qicored_dn8 * locals.var_csi) * locals.var_nvtm) + (assign10160_e9578 * locals.var_nvtm_dn8));
        locals.var_qtotd_rv = 0.0;

        let assign10170_e9583: f64 = (locals.var_qtotd - locals.var_qfrontd);
        locals.var_qbackd = assign10170_e9583;
        locals.var_qbackd_dn3 = (locals.var_qtotd_dn3 - locals.var_qfrontd_dn3);
        locals.var_qbackd_dn4 = (locals.var_qtotd_dn4 - locals.var_qfrontd_dn4);
        locals.var_qbackd_dn5 = (locals.var_qtotd_dn5 - locals.var_qfrontd_dn5);
        locals.var_qbackd_dn6 = (locals.var_qtotd_dn6 - locals.var_qfrontd_dn6);
        locals.var_qbackd_dn7 = (locals.var_qtotd_dn7 - locals.var_qfrontd_dn7);
        locals.var_qbackd_dn8 = (locals.var_qtotd_dn8 - locals.var_qfrontd_dn8);
        locals.var_qbackd_rv = 0.0;

        let assign10180_e9588: f64 = (locals.var_cox2 * locals.var_nvtm);
        let assign10180_e9589: f64 = (locals.var_qbackd / assign10180_e9588);
        let assign10180_e9590: f64 = (locals.var_xg2 - assign10180_e9589);
        locals.var_phi2 = assign10180_e9590;
        locals.var_phi2_dn3 = (locals.var_xg2_dn3 - (((locals.var_qbackd_dn3 * assign10180_e9588) - (locals.var_qbackd * (locals.var_cox2 * locals.var_nvtm_dn3))) / (assign10180_e9588 * assign10180_e9588)));
        locals.var_phi2_dn4 = (locals.var_xg2_dn4 - (((locals.var_qbackd_dn4 * assign10180_e9588) - (locals.var_qbackd * (locals.var_cox2 * locals.var_nvtm_dn4))) / (assign10180_e9588 * assign10180_e9588)));
        locals.var_phi2_dn5 = (locals.var_xg2_dn5 - (((locals.var_qbackd_dn5 * assign10180_e9588) - (locals.var_qbackd * (locals.var_cox2 * locals.var_nvtm_dn5))) / (assign10180_e9588 * assign10180_e9588)));
        locals.var_phi2_dn6 = (locals.var_xg2_dn6 - (((locals.var_qbackd_dn6 * assign10180_e9588) - (locals.var_qbackd * (locals.var_cox2 * locals.var_nvtm_dn6))) / (assign10180_e9588 * assign10180_e9588)));
        locals.var_phi2_dn7 = (locals.var_xg2_dn7 - (((locals.var_qbackd_dn7 * assign10180_e9588) - (locals.var_qbackd * (locals.var_cox2 * locals.var_nvtm_dn7))) / (assign10180_e9588 * assign10180_e9588)));
        locals.var_phi2_dn8 = (locals.var_xg2_dn8 - (((locals.var_qbackd_dn8 * assign10180_e9588) - (locals.var_qbackd * (locals.var_cox2 * locals.var_nvtm_dn8))) / (assign10180_e9588 * assign10180_e9588)));
        locals.var_phi2_rv = 0.0;

        let assign10190_e9593: f64 = (locals.var_qtotd / locals.var_cox1);
        locals.var_qid = assign10190_e9593;
        locals.var_qid_dn3 = (locals.var_qtotd_dn3 / locals.var_cox1);
        locals.var_qid_dn4 = (locals.var_qtotd_dn4 / locals.var_cox1);
        locals.var_qid_dn5 = (locals.var_qtotd_dn5 / locals.var_cox1);
        locals.var_qid_dn6 = (locals.var_qtotd_dn6 / locals.var_cox1);
        locals.var_qid_dn7 = (locals.var_qtotd_dn7 / locals.var_cox1);
        locals.var_qid_dn8 = (locals.var_qtotd_dn8 / locals.var_cox1);
        locals.var_qid_rv = 0.0;

        let assign10200_e9597: f64 = (locals.var_qis + locals.var_qid);
        let assign10200_e9598: f64 = (0.5 * assign10200_e9597);
        locals.var_qia = assign10200_e9598;
        locals.var_qia_dn3 = (0.5 * (locals.var_qis_dn3 + locals.var_qid_dn3));
        locals.var_qia_dn4 = (0.5 * (locals.var_qis_dn4 + locals.var_qid_dn4));
        locals.var_qia_dn5 = (0.5 * (locals.var_qis_dn5 + locals.var_qid_dn5));
        locals.var_qia_dn6 = (0.5 * (locals.var_qis_dn6 + locals.var_qid_dn6));
        locals.var_qia_dn7 = (0.5 * (locals.var_qis_dn7 + locals.var_qid_dn7));
        locals.var_qia_dn8 = (0.5 * (locals.var_qis_dn8 + locals.var_qid_dn8));
        locals.var_qia_rv = 0.0;

        let assign10210_e9601: f64 = (locals.var_qis - locals.var_qid);
        locals.var_dqi = assign10210_e9601;
        locals.var_dqi_dn3 = (locals.var_qis_dn3 - locals.var_qid_dn3);
        locals.var_dqi_dn4 = (locals.var_qis_dn4 - locals.var_qid_dn4);
        locals.var_dqi_dn5 = (locals.var_qis_dn5 - locals.var_qid_dn5);
        locals.var_dqi_dn6 = (locals.var_qis_dn6 - locals.var_qid_dn6);
        locals.var_dqi_dn7 = (locals.var_qis_dn7 - locals.var_qid_dn7);
        locals.var_dqi_dn8 = (locals.var_qis_dn8 - locals.var_qid_dn8);
        locals.var_dqi_rv = 0.0;

        let assign10220_e9604: f64 = (1.60219e-19 * locals.var_nbody_i);
        let assign10220_e9606: f64 = (assign10220_e9604 * p.p49);
        let assign10220_e9608: f64 = (assign10220_e9606 / locals.var_cox1);
        locals.var_qba = assign10220_e9608;
        locals.var_qba_rv = 0.0;

        let assign10230_e9611: f64 = (locals.var_vdseff).powf(2.0);
        let assign10230_e9613: f64 = (assign10230_e9611 / 0.000625);
        locals.var_t0 = assign10230_e9613;
        locals.var_t0_dn3 = (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vdseff).powf(2.0 - 1.0) * locals.var_vdseff_dn3)) } } else { (assign10230_e9611 * (2.0 * (locals.var_vdseff_dn3 / locals.var_vdseff))) } / 0.000625);
        locals.var_t0_dn4 = (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vdseff).powf(2.0 - 1.0) * locals.var_vdseff_dn4)) } } else { (assign10230_e9611 * (2.0 * (locals.var_vdseff_dn4 / locals.var_vdseff))) } / 0.000625);
        locals.var_t0_dn5 = (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vdseff).powf(2.0 - 1.0) * locals.var_vdseff_dn5)) } } else { (assign10230_e9611 * (2.0 * (locals.var_vdseff_dn5 / locals.var_vdseff))) } / 0.000625);
        locals.var_t0_dn6 = (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vdseff).powf(2.0 - 1.0) * locals.var_vdseff_dn6)) } } else { (assign10230_e9611 * (2.0 * (locals.var_vdseff_dn6 / locals.var_vdseff))) } / 0.000625);
        locals.var_t0_dn7 = (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vdseff).powf(2.0 - 1.0) * locals.var_vdseff_dn7)) } } else { (assign10230_e9611 * (2.0 * (locals.var_vdseff_dn7 / locals.var_vdseff))) } / 0.000625);
        locals.var_t0_dn8 = (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((locals.var_vdseff).powf(2.0 - 1.0) * locals.var_vdseff_dn8)) } } else { (assign10230_e9611 * (2.0 * (locals.var_vdseff_dn8 / locals.var_vdseff))) } / 0.000625);
        locals.var_t0_rv = 0.0;

        let assign10240_e9616: f64 = if p.p162 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard98 = assign10240_e9616;
        locals.var_guard98_rv = 0.0;

        let (assign10250_e9642, assign10250_e9642_d_n3, assign10250_e9642_d_n4, assign10250_e9642_d_n5, assign10250_e9642_d_n6, assign10250_e9642_d_n7, assign10250_e9642_d_n8,) = {
    if (locals.var_guard98 != 0.0) {
        let assign10250_e9620: f64 = (locals.var_qfronts + locals.var_qfrontd);
        let assign10250_e9623: f64 = (2.0 * locals.var_cox1);
        let assign10250_e9624: f64 = (assign10250_e9620 / assign10250_e9623);
        let assign10250_e9628: f64 = (-locals.var_t0);
        let assign10250_e9629: f64 = { let limited_exp_arg = assign10250_e9628; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign10250_e9630: f64 = (1.0 - assign10250_e9629);
        let assign10250_e9631: f64 = (p.p162 * assign10250_e9630);
        let assign10250_e9633: f64 = (assign10250_e9631 * 0.5);
        let assign10250_e9636: f64 = (locals.var_qfronts - locals.var_qfrontd);
        let assign10250_e9637: f64 = (assign10250_e9633 * assign10250_e9636);
        let assign10250_e9639: f64 = (assign10250_e9637 / locals.var_cox1);
        let assign10250_e9640: f64 = (assign10250_e9624 + assign10250_e9639);
        (assign10250_e9640, (((locals.var_qfronts_dn3 + locals.var_qfrontd_dn3) / assign10250_e9623) + (((((p.p162 * (-({ let limited_exp_arg = assign10250_e9628; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn3)))) * 0.5) * assign10250_e9636) + (assign10250_e9633 * (locals.var_qfronts_dn3 - locals.var_qfrontd_dn3))) / locals.var_cox1)), (((locals.var_qfronts_dn4 + locals.var_qfrontd_dn4) / assign10250_e9623) + (((((p.p162 * (-({ let limited_exp_arg = assign10250_e9628; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4)))) * 0.5) * assign10250_e9636) + (assign10250_e9633 * (locals.var_qfronts_dn4 - locals.var_qfrontd_dn4))) / locals.var_cox1)), (((locals.var_qfronts_dn5 + locals.var_qfrontd_dn5) / assign10250_e9623) + (((((p.p162 * (-({ let limited_exp_arg = assign10250_e9628; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5)))) * 0.5) * assign10250_e9636) + (assign10250_e9633 * (locals.var_qfronts_dn5 - locals.var_qfrontd_dn5))) / locals.var_cox1)), (((locals.var_qfronts_dn6 + locals.var_qfrontd_dn6) / assign10250_e9623) + (((((p.p162 * (-({ let limited_exp_arg = assign10250_e9628; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6)))) * 0.5) * assign10250_e9636) + (assign10250_e9633 * (locals.var_qfronts_dn6 - locals.var_qfrontd_dn6))) / locals.var_cox1)), (((locals.var_qfronts_dn7 + locals.var_qfrontd_dn7) / assign10250_e9623) + (((((p.p162 * (-({ let limited_exp_arg = assign10250_e9628; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7)))) * 0.5) * assign10250_e9636) + (assign10250_e9633 * (locals.var_qfronts_dn7 - locals.var_qfrontd_dn7))) / locals.var_cox1)), (((locals.var_qfronts_dn8 + locals.var_qfrontd_dn8) / assign10250_e9623) + (((((p.p162 * (-({ let limited_exp_arg = assign10250_e9628; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8)))) * 0.5) * assign10250_e9636) + (assign10250_e9633 * (locals.var_qfronts_dn8 - locals.var_qfrontd_dn8))) / locals.var_cox1)),)
    } else {
        (locals.var_qia2, locals.var_qia2_dn3, locals.var_qia2_dn4, locals.var_qia2_dn5, locals.var_qia2_dn6, locals.var_qia2_dn7, locals.var_qia2_dn8,)
    }
};
        locals.var_qia2 = assign10250_e9642;
        locals.var_qia2_dn3 = assign10250_e9642_d_n3;
        locals.var_qia2_dn4 = assign10250_e9642_d_n4;
        locals.var_qia2_dn5 = assign10250_e9642_d_n5;
        locals.var_qia2_dn6 = assign10250_e9642_d_n6;
        locals.var_qia2_dn7 = assign10250_e9642_d_n7;
        locals.var_qia2_dn8 = assign10250_e9642_d_n8;
        locals.var_qia2_rv = 0.0;

        let (assign10260_e9653, assign10260_e9653_d_n3, assign10260_e9653_d_n4, assign10260_e9653_d_n5, assign10260_e9653_d_n6, assign10260_e9653_d_n7, assign10260_e9653_d_n8,) = {
    if (locals.var_guard98 == 0.0) {
        let assign10260_e9647: f64 = (locals.var_qfronts + locals.var_qfrontd);
        let assign10260_e9650: f64 = (2.0 * locals.var_cox1);
        let assign10260_e9651: f64 = (assign10260_e9647 / assign10260_e9650);
        (assign10260_e9651, ((locals.var_qfronts_dn3 + locals.var_qfrontd_dn3) / assign10260_e9650), ((locals.var_qfronts_dn4 + locals.var_qfrontd_dn4) / assign10260_e9650), ((locals.var_qfronts_dn5 + locals.var_qfrontd_dn5) / assign10260_e9650), ((locals.var_qfronts_dn6 + locals.var_qfrontd_dn6) / assign10260_e9650), ((locals.var_qfronts_dn7 + locals.var_qfrontd_dn7) / assign10260_e9650), ((locals.var_qfronts_dn8 + locals.var_qfrontd_dn8) / assign10260_e9650),)
    } else {
        (locals.var_qia2, locals.var_qia2_dn3, locals.var_qia2_dn4, locals.var_qia2_dn5, locals.var_qia2_dn6, locals.var_qia2_dn7, locals.var_qia2_dn8,)
    }
};
        locals.var_qia2 = assign10260_e9653;
        locals.var_qia2_dn3 = assign10260_e9653_d_n3;
        locals.var_qia2_dn4 = assign10260_e9653_d_n4;
        locals.var_qia2_dn5 = assign10260_e9653_d_n5;
        locals.var_qia2_dn6 = assign10260_e9653_d_n6;
        locals.var_qia2_dn7 = assign10260_e9653_d_n7;
        locals.var_qia2_dn8 = assign10260_e9653_d_n8;
        locals.var_qia2_rv = 0.0;

        let assign10270_e9656: f64 = if p.p189 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard99 = assign10270_e9656;
        locals.var_guard99_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_23(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10280_e9682, assign10280_e9682_d_n3, assign10280_e9682_d_n4, assign10280_e9682_d_n5, assign10280_e9682_d_n6, assign10280_e9682_d_n7, assign10280_e9682_d_n8,) = {
    if (locals.var_guard99 != 0.0) {
        let assign10280_e9660: f64 = (locals.var_qbacks + locals.var_qbackd);
        let assign10280_e9663: f64 = (2.0 * locals.var_cox2);
        let assign10280_e9664: f64 = (assign10280_e9660 / assign10280_e9663);
        let assign10280_e9668: f64 = (-locals.var_t0);
        let assign10280_e9669: f64 = { let limited_exp_arg = assign10280_e9668; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign10280_e9670: f64 = (1.0 - assign10280_e9669);
        let assign10280_e9671: f64 = (p.p189 * assign10280_e9670);
        let assign10280_e9673: f64 = (assign10280_e9671 * 0.5);
        let assign10280_e9676: f64 = (locals.var_qbacks - locals.var_qbackd);
        let assign10280_e9677: f64 = (assign10280_e9673 * assign10280_e9676);
        let assign10280_e9679: f64 = (assign10280_e9677 / locals.var_cox2);
        let assign10280_e9680: f64 = (assign10280_e9664 + assign10280_e9679);
        (assign10280_e9680, (((locals.var_qbacks_dn3 + locals.var_qbackd_dn3) / assign10280_e9663) + (((((p.p189 * (-({ let limited_exp_arg = assign10280_e9668; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn3)))) * 0.5) * assign10280_e9676) + (assign10280_e9673 * (locals.var_qbacks_dn3 - locals.var_qbackd_dn3))) / locals.var_cox2)), (((locals.var_qbacks_dn4 + locals.var_qbackd_dn4) / assign10280_e9663) + (((((p.p189 * (-({ let limited_exp_arg = assign10280_e9668; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4)))) * 0.5) * assign10280_e9676) + (assign10280_e9673 * (locals.var_qbacks_dn4 - locals.var_qbackd_dn4))) / locals.var_cox2)), (((locals.var_qbacks_dn5 + locals.var_qbackd_dn5) / assign10280_e9663) + (((((p.p189 * (-({ let limited_exp_arg = assign10280_e9668; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5)))) * 0.5) * assign10280_e9676) + (assign10280_e9673 * (locals.var_qbacks_dn5 - locals.var_qbackd_dn5))) / locals.var_cox2)), (((locals.var_qbacks_dn6 + locals.var_qbackd_dn6) / assign10280_e9663) + (((((p.p189 * (-({ let limited_exp_arg = assign10280_e9668; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6)))) * 0.5) * assign10280_e9676) + (assign10280_e9673 * (locals.var_qbacks_dn6 - locals.var_qbackd_dn6))) / locals.var_cox2)), (((locals.var_qbacks_dn7 + locals.var_qbackd_dn7) / assign10280_e9663) + (((((p.p189 * (-({ let limited_exp_arg = assign10280_e9668; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7)))) * 0.5) * assign10280_e9676) + (assign10280_e9673 * (locals.var_qbacks_dn7 - locals.var_qbackd_dn7))) / locals.var_cox2)), (((locals.var_qbacks_dn8 + locals.var_qbackd_dn8) / assign10280_e9663) + (((((p.p189 * (-({ let limited_exp_arg = assign10280_e9668; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8)))) * 0.5) * assign10280_e9676) + (assign10280_e9673 * (locals.var_qbacks_dn8 - locals.var_qbackd_dn8))) / locals.var_cox2)),)
    } else {
        (locals.var_qib2, locals.var_qib2_dn3, locals.var_qib2_dn4, locals.var_qib2_dn5, locals.var_qib2_dn6, locals.var_qib2_dn7, locals.var_qib2_dn8,)
    }
};
        locals.var_qib2 = assign10280_e9682;
        locals.var_qib2_dn3 = assign10280_e9682_d_n3;
        locals.var_qib2_dn4 = assign10280_e9682_d_n4;
        locals.var_qib2_dn5 = assign10280_e9682_d_n5;
        locals.var_qib2_dn6 = assign10280_e9682_d_n6;
        locals.var_qib2_dn7 = assign10280_e9682_d_n7;
        locals.var_qib2_dn8 = assign10280_e9682_d_n8;
        locals.var_qib2_rv = 0.0;

        let (assign10290_e9693, assign10290_e9693_d_n3, assign10290_e9693_d_n4, assign10290_e9693_d_n5, assign10290_e9693_d_n6, assign10290_e9693_d_n7, assign10290_e9693_d_n8,) = {
    if (locals.var_guard99 == 0.0) {
        let assign10290_e9687: f64 = (locals.var_qbacks + locals.var_qbackd);
        let assign10290_e9690: f64 = (2.0 * locals.var_cox2);
        let assign10290_e9691: f64 = (assign10290_e9687 / assign10290_e9690);
        (assign10290_e9691, ((locals.var_qbacks_dn3 + locals.var_qbackd_dn3) / assign10290_e9690), ((locals.var_qbacks_dn4 + locals.var_qbackd_dn4) / assign10290_e9690), ((locals.var_qbacks_dn5 + locals.var_qbackd_dn5) / assign10290_e9690), ((locals.var_qbacks_dn6 + locals.var_qbackd_dn6) / assign10290_e9690), ((locals.var_qbacks_dn7 + locals.var_qbackd_dn7) / assign10290_e9690), ((locals.var_qbacks_dn8 + locals.var_qbackd_dn8) / assign10290_e9690),)
    } else {
        (locals.var_qib2, locals.var_qib2_dn3, locals.var_qib2_dn4, locals.var_qib2_dn5, locals.var_qib2_dn6, locals.var_qib2_dn7, locals.var_qib2_dn8,)
    }
};
        locals.var_qib2 = assign10290_e9693;
        locals.var_qib2_dn3 = assign10290_e9693_d_n3;
        locals.var_qib2_dn4 = assign10290_e9693_d_n4;
        locals.var_qib2_dn5 = assign10290_e9693_d_n5;
        locals.var_qib2_dn6 = assign10290_e9693_d_n6;
        locals.var_qib2_dn7 = assign10290_e9693_d_n7;
        locals.var_qib2_dn8 = assign10290_e9693_d_n8;
        locals.var_qib2_rv = 0.0;

        let assign10300_e9696: f64 = (locals.var_eta_mu * locals.var_qia2);
        let assign10300_e9698: f64 = (assign10300_e9696 + locals.var_qba);
        locals.var_t2 = assign10300_e9698;
        locals.var_t2_dn3 = (locals.var_eta_mu * locals.var_qia2_dn3);
        locals.var_t2_dn4 = (locals.var_eta_mu * locals.var_qia2_dn4);
        locals.var_t2_dn5 = (locals.var_eta_mu * locals.var_qia2_dn5);
        locals.var_t2_dn6 = (locals.var_eta_mu * locals.var_qia2_dn6);
        locals.var_t2_dn7 = (locals.var_eta_mu * locals.var_qia2_dn7);
        locals.var_t2_dn8 = (locals.var_eta_mu * locals.var_qia2_dn8);
        locals.var_t2_rv = 0.0;

        let assign10310_e9703: f64 = (locals.var_t2 * locals.var_t2);
        let assign10310_e9705: f64 = (assign10310_e9703 + 0.001);
        let assign10310_e9706: f64 = (assign10310_e9705).sqrt();
        let assign10310_e9707: f64 = (locals.var_t2 + assign10310_e9706);
        let assign10310_e9708: f64 = (0.5 * assign10310_e9707);
        locals.var_t3 = assign10310_e9708;
        locals.var_t3_dn3 = (0.5 * (locals.var_t2_dn3 + (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign10310_e9706))));
        locals.var_t3_dn4 = (0.5 * (locals.var_t2_dn4 + (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign10310_e9706))));
        locals.var_t3_dn5 = (0.5 * (locals.var_t2_dn5 + (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign10310_e9706))));
        locals.var_t3_dn6 = (0.5 * (locals.var_t2_dn6 + (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign10310_e9706))));
        locals.var_t3_dn7 = (0.5 * (locals.var_t2_dn7 + (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign10310_e9706))));
        locals.var_t3_dn8 = (0.5 * (locals.var_t2_dn8 + (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign10310_e9706))));
        locals.var_t3_rv = 0.0;

        let assign10320_e9711: f64 = (locals.var_eefffactor * locals.var_t3);
        locals.var_eeffm = assign10320_e9711;
        locals.var_eeffm_dn3 = (locals.var_eefffactor * locals.var_t3_dn3);
        locals.var_eeffm_dn4 = (locals.var_eefffactor * locals.var_t3_dn4);
        locals.var_eeffm_dn5 = (locals.var_eefffactor * locals.var_t3_dn5);
        locals.var_eeffm_dn6 = (locals.var_eefffactor * locals.var_t3_dn6);
        locals.var_eeffm_dn7 = (locals.var_eefffactor * locals.var_t3_dn7);
        locals.var_eeffm_dn8 = (locals.var_eefffactor * locals.var_t3_dn8);
        locals.var_eeffm_rv = 0.0;

        let assign10330_e9714: f64 = (locals.var_eta_mu2 * locals.var_qib2);
        let assign10330_e9716: f64 = (assign10330_e9714 + locals.var_qba);
        locals.var_t2 = assign10330_e9716;
        locals.var_t2_dn3 = (locals.var_eta_mu2 * locals.var_qib2_dn3);
        locals.var_t2_dn4 = (locals.var_eta_mu2 * locals.var_qib2_dn4);
        locals.var_t2_dn5 = (locals.var_eta_mu2 * locals.var_qib2_dn5);
        locals.var_t2_dn6 = (locals.var_eta_mu2 * locals.var_qib2_dn6);
        locals.var_t2_dn7 = (locals.var_eta_mu2 * locals.var_qib2_dn7);
        locals.var_t2_dn8 = (locals.var_eta_mu2 * locals.var_qib2_dn8);
        locals.var_t2_rv = 0.0;

        let assign10340_e9721: f64 = (locals.var_t2 * locals.var_t2);
        let assign10340_e9723: f64 = (assign10340_e9721 + 0.001);
        let assign10340_e9724: f64 = (assign10340_e9723).sqrt();
        let assign10340_e9725: f64 = (locals.var_t2 + assign10340_e9724);
        let assign10340_e9726: f64 = (0.5 * assign10340_e9725);
        locals.var_t3 = assign10340_e9726;
        locals.var_t3_dn3 = (0.5 * (locals.var_t2_dn3 + (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign10340_e9724))));
        locals.var_t3_dn4 = (0.5 * (locals.var_t2_dn4 + (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign10340_e9724))));
        locals.var_t3_dn5 = (0.5 * (locals.var_t2_dn5 + (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign10340_e9724))));
        locals.var_t3_dn6 = (0.5 * (locals.var_t2_dn6 + (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign10340_e9724))));
        locals.var_t3_dn7 = (0.5 * (locals.var_t2_dn7 + (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign10340_e9724))));
        locals.var_t3_dn8 = (0.5 * (locals.var_t2_dn8 + (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign10340_e9724))));
        locals.var_t3_rv = 0.0;

        let assign10350_e9729: f64 = (locals.var_eefffactor2 * locals.var_t3);
        locals.var_eeffm2 = assign10350_e9729;
        locals.var_eeffm2_dn3 = (locals.var_eefffactor2 * locals.var_t3_dn3);
        locals.var_eeffm2_dn4 = (locals.var_eefffactor2 * locals.var_t3_dn4);
        locals.var_eeffm2_dn5 = (locals.var_eefffactor2 * locals.var_t3_dn5);
        locals.var_eeffm2_dn6 = (locals.var_eefffactor2 * locals.var_t3_dn6);
        locals.var_eeffm2_dn7 = (locals.var_eefffactor2 * locals.var_t3_dn7);
        locals.var_eeffm2_dn8 = (locals.var_eefffactor2 * locals.var_t3_dn8);
        locals.var_eeffm2_rv = 0.0;

        let assign10360_e9734: f64 = (locals.var_qia / locals.var_qb0);
        let assign10360_e9735: f64 = (assign10360_e9734).abs();
        let assign10360_e9736: f64 = (1.0 + assign10360_e9735);
        let assign10360_e9737: f64 = (0.5 * assign10360_e9736);
        let assign10360_e9739: f64 = (assign10360_e9737).powf(locals.var_ucs_t);
        locals.var_t2__blk100 = assign10360_e9739;
        locals.var_t2__blk100_dn3 = if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign10360_e9737).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign10360_e9734 >= 0.0 { (locals.var_qia_dn3 / locals.var_qb0) } else { (-(locals.var_qia_dn3 / locals.var_qb0)) }))) } } else { (assign10360_e9739 * (locals.var_ucs_t * ((0.5 * if assign10360_e9734 >= 0.0 { (locals.var_qia_dn3 / locals.var_qb0) } else { (-(locals.var_qia_dn3 / locals.var_qb0)) }) / assign10360_e9737))) };
        locals.var_t2__blk100_dn4 = if locals.var_ucs_t_dn4 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign10360_e9737).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign10360_e9734 >= 0.0 { (locals.var_qia_dn4 / locals.var_qb0) } else { (-(locals.var_qia_dn4 / locals.var_qb0)) }))) } } else { (assign10360_e9739 * ((locals.var_ucs_t_dn4 * (assign10360_e9737).ln()) + (locals.var_ucs_t * ((0.5 * if assign10360_e9734 >= 0.0 { (locals.var_qia_dn4 / locals.var_qb0) } else { (-(locals.var_qia_dn4 / locals.var_qb0)) }) / assign10360_e9737)))) };
        locals.var_t2__blk100_dn5 = if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign10360_e9737).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign10360_e9734 >= 0.0 { (locals.var_qia_dn5 / locals.var_qb0) } else { (-(locals.var_qia_dn5 / locals.var_qb0)) }))) } } else { (assign10360_e9739 * (locals.var_ucs_t * ((0.5 * if assign10360_e9734 >= 0.0 { (locals.var_qia_dn5 / locals.var_qb0) } else { (-(locals.var_qia_dn5 / locals.var_qb0)) }) / assign10360_e9737))) };
        locals.var_t2__blk100_dn6 = if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign10360_e9737).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign10360_e9734 >= 0.0 { (locals.var_qia_dn6 / locals.var_qb0) } else { (-(locals.var_qia_dn6 / locals.var_qb0)) }))) } } else { (assign10360_e9739 * (locals.var_ucs_t * ((0.5 * if assign10360_e9734 >= 0.0 { (locals.var_qia_dn6 / locals.var_qb0) } else { (-(locals.var_qia_dn6 / locals.var_qb0)) }) / assign10360_e9737))) };
        locals.var_t2__blk100_dn7 = if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign10360_e9737).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign10360_e9734 >= 0.0 { (locals.var_qia_dn7 / locals.var_qb0) } else { (-(locals.var_qia_dn7 / locals.var_qb0)) }))) } } else { (assign10360_e9739 * (locals.var_ucs_t * ((0.5 * if assign10360_e9734 >= 0.0 { (locals.var_qia_dn7 / locals.var_qb0) } else { (-(locals.var_qia_dn7 / locals.var_qb0)) }) / assign10360_e9737))) };
        locals.var_t2__blk100_dn8 = if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign10360_e9737).powf(locals.var_ucs_t - 1.0) * (0.5 * if assign10360_e9734 >= 0.0 { (locals.var_qia_dn8 / locals.var_qb0) } else { (-(locals.var_qia_dn8 / locals.var_qb0)) }))) } } else { (assign10360_e9739 * (locals.var_ucs_t * ((0.5 * if assign10360_e9734 >= 0.0 { (locals.var_qia_dn8 / locals.var_qb0) } else { (-(locals.var_qia_dn8 / locals.var_qb0)) }) / assign10360_e9737))) };
        locals.var_t2__blk100_rv = 0.0;

        let assign10370_e9743: f64 = (locals.var_vbgx * locals.var_uc_t);
        let assign10370_e9744: f64 = (locals.var_ua_t + assign10370_e9743);
        let assign10370_e9746: f64 = (locals.var_eeffm).abs();
        let assign10370_e9750: f64 = (locals.var_eub_i * locals.var_vbgx);
        let assign10370_e9751: f64 = (locals.var_eu_i + assign10370_e9750);
        let assign10370_e9752: f64 = (assign10370_e9746).powf(assign10370_e9751);
        let assign10370_e9753: f64 = (assign10370_e9744 * assign10370_e9752);
        let assign10370_e9757: f64 = (locals.var_vbgx * locals.var_udb_i);
        let assign10370_e9758: f64 = (locals.var_ud_t + assign10370_e9757);
        let assign10370_e9760: f64 = (assign10370_e9758 / locals.var_t2__blk100);
        let assign10370_e9761: f64 = (assign10370_e9753 + assign10370_e9760);
        locals.var_t3__blk101 = assign10370_e9761;
        locals.var_t3__blk101_dn3 = ((((locals.var_vbgx_dn3 * locals.var_uc_t) * assign10370_e9752) + (assign10370_e9744 * if (locals.var_eub_i * locals.var_vbgx_dn3) == 0.0 && ((assign10370_e9751) as f64).is_finite() && ((assign10370_e9751) as f64).fract() == 0.0 { if assign10370_e9751 == 0.0 { 0.0 } else { (assign10370_e9751 * ((assign10370_e9746).powf(assign10370_e9751 - 1.0) * if locals.var_eeffm >= 0.0 { locals.var_eeffm_dn3 } else { (-locals.var_eeffm_dn3) })) } } else { (assign10370_e9752 * (((locals.var_eub_i * locals.var_vbgx_dn3) * (assign10370_e9746).ln()) + (assign10370_e9751 * (if locals.var_eeffm >= 0.0 { locals.var_eeffm_dn3 } else { (-locals.var_eeffm_dn3) } / assign10370_e9746)))) })) + ((((locals.var_vbgx_dn3 * locals.var_udb_i) * locals.var_t2__blk100) - (assign10370_e9758 * locals.var_t2__blk100_dn3)) / (locals.var_t2__blk100 * locals.var_t2__blk100)));
        locals.var_t3__blk101_dn4 = ((((locals.var_ua_t_dn4 + (locals.var_vbgx * locals.var_uc_t_dn4)) * assign10370_e9752) + (assign10370_e9744 * if 0.0 == 0.0 && ((assign10370_e9751) as f64).is_finite() && ((assign10370_e9751) as f64).fract() == 0.0 { if assign10370_e9751 == 0.0 { 0.0 } else { (assign10370_e9751 * ((assign10370_e9746).powf(assign10370_e9751 - 1.0) * if locals.var_eeffm >= 0.0 { locals.var_eeffm_dn4 } else { (-locals.var_eeffm_dn4) })) } } else { (assign10370_e9752 * (assign10370_e9751 * (if locals.var_eeffm >= 0.0 { locals.var_eeffm_dn4 } else { (-locals.var_eeffm_dn4) } / assign10370_e9746))) })) + (((locals.var_ud_t_dn4 * locals.var_t2__blk100) - (assign10370_e9758 * locals.var_t2__blk100_dn4)) / (locals.var_t2__blk100 * locals.var_t2__blk100)));
        locals.var_t3__blk101_dn5 = ((((locals.var_vbgx_dn5 * locals.var_uc_t) * assign10370_e9752) + (assign10370_e9744 * if (locals.var_eub_i * locals.var_vbgx_dn5) == 0.0 && ((assign10370_e9751) as f64).is_finite() && ((assign10370_e9751) as f64).fract() == 0.0 { if assign10370_e9751 == 0.0 { 0.0 } else { (assign10370_e9751 * ((assign10370_e9746).powf(assign10370_e9751 - 1.0) * if locals.var_eeffm >= 0.0 { locals.var_eeffm_dn5 } else { (-locals.var_eeffm_dn5) })) } } else { (assign10370_e9752 * (((locals.var_eub_i * locals.var_vbgx_dn5) * (assign10370_e9746).ln()) + (assign10370_e9751 * (if locals.var_eeffm >= 0.0 { locals.var_eeffm_dn5 } else { (-locals.var_eeffm_dn5) } / assign10370_e9746)))) })) + ((((locals.var_vbgx_dn5 * locals.var_udb_i) * locals.var_t2__blk100) - (assign10370_e9758 * locals.var_t2__blk100_dn5)) / (locals.var_t2__blk100 * locals.var_t2__blk100)));
        locals.var_t3__blk101_dn6 = ((((locals.var_vbgx_dn6 * locals.var_uc_t) * assign10370_e9752) + (assign10370_e9744 * if (locals.var_eub_i * locals.var_vbgx_dn6) == 0.0 && ((assign10370_e9751) as f64).is_finite() && ((assign10370_e9751) as f64).fract() == 0.0 { if assign10370_e9751 == 0.0 { 0.0 } else { (assign10370_e9751 * ((assign10370_e9746).powf(assign10370_e9751 - 1.0) * if locals.var_eeffm >= 0.0 { locals.var_eeffm_dn6 } else { (-locals.var_eeffm_dn6) })) } } else { (assign10370_e9752 * (((locals.var_eub_i * locals.var_vbgx_dn6) * (assign10370_e9746).ln()) + (assign10370_e9751 * (if locals.var_eeffm >= 0.0 { locals.var_eeffm_dn6 } else { (-locals.var_eeffm_dn6) } / assign10370_e9746)))) })) + ((((locals.var_vbgx_dn6 * locals.var_udb_i) * locals.var_t2__blk100) - (assign10370_e9758 * locals.var_t2__blk100_dn6)) / (locals.var_t2__blk100 * locals.var_t2__blk100)));
        locals.var_t3__blk101_dn7 = ((assign10370_e9744 * if 0.0 == 0.0 && ((assign10370_e9751) as f64).is_finite() && ((assign10370_e9751) as f64).fract() == 0.0 { if assign10370_e9751 == 0.0 { 0.0 } else { (assign10370_e9751 * ((assign10370_e9746).powf(assign10370_e9751 - 1.0) * if locals.var_eeffm >= 0.0 { locals.var_eeffm_dn7 } else { (-locals.var_eeffm_dn7) })) } } else { (assign10370_e9752 * (assign10370_e9751 * (if locals.var_eeffm >= 0.0 { locals.var_eeffm_dn7 } else { (-locals.var_eeffm_dn7) } / assign10370_e9746))) }) + (-((assign10370_e9758 * locals.var_t2__blk100_dn7) / (locals.var_t2__blk100 * locals.var_t2__blk100))));
        locals.var_t3__blk101_dn8 = ((assign10370_e9744 * if 0.0 == 0.0 && ((assign10370_e9751) as f64).is_finite() && ((assign10370_e9751) as f64).fract() == 0.0 { if assign10370_e9751 == 0.0 { 0.0 } else { (assign10370_e9751 * ((assign10370_e9746).powf(assign10370_e9751 - 1.0) * if locals.var_eeffm >= 0.0 { locals.var_eeffm_dn8 } else { (-locals.var_eeffm_dn8) })) } } else { (assign10370_e9752 * (assign10370_e9751 * (if locals.var_eeffm >= 0.0 { locals.var_eeffm_dn8 } else { (-locals.var_eeffm_dn8) } / assign10370_e9746))) }) + (-((assign10370_e9758 * locals.var_t2__blk100_dn8) / (locals.var_t2__blk100 * locals.var_t2__blk100))));
        locals.var_t3__blk101_rv = 0.0;

        let assign10380_e9764: f64 = (1.0 + locals.var_t3__blk101);
        locals.var_dmob = assign10380_e9764;
        locals.var_dmob_dn3 = locals.var_t3__blk101_dn3;
        locals.var_dmob_dn4 = locals.var_t3__blk101_dn4;
        locals.var_dmob_dn5 = locals.var_t3__blk101_dn5;
        locals.var_dmob_dn6 = locals.var_t3__blk101_dn6;
        locals.var_dmob_dn7 = locals.var_t3__blk101_dn7;
        locals.var_dmob_dn8 = locals.var_t3__blk101_dn8;
        locals.var_dmob_rv = 0.0;

        let assign10390_e9768: f64 = (locals.var_dmob + 1.0);
        let assign10390_e9771: f64 = (locals.var_dmob - 1.0);
        let assign10390_e9774: f64 = (locals.var_dmob - 1.0);
        let assign10390_e9775: f64 = (assign10390_e9771 * assign10390_e9774);
        let assign10390_e9778: f64 = (0.25 * p.p154);
        let assign10390_e9780: f64 = (assign10390_e9778 * p.p154);
        let assign10390_e9781: f64 = (assign10390_e9775 + assign10390_e9780);
        let assign10390_e9782: f64 = (assign10390_e9781).sqrt();
        let assign10390_e9783: f64 = (assign10390_e9768 + assign10390_e9782);
        let assign10390_e9784: f64 = (0.5 * assign10390_e9783);
        locals.var_dmob = assign10390_e9784;
        locals.var_dmob_dn3 = (0.5 * (locals.var_dmob_dn3 + (((locals.var_dmob_dn3 * assign10390_e9774) + (assign10390_e9771 * locals.var_dmob_dn3)) / (2.0 * assign10390_e9782))));
        locals.var_dmob_dn4 = (0.5 * (locals.var_dmob_dn4 + (((locals.var_dmob_dn4 * assign10390_e9774) + (assign10390_e9771 * locals.var_dmob_dn4)) / (2.0 * assign10390_e9782))));
        locals.var_dmob_dn5 = (0.5 * (locals.var_dmob_dn5 + (((locals.var_dmob_dn5 * assign10390_e9774) + (assign10390_e9771 * locals.var_dmob_dn5)) / (2.0 * assign10390_e9782))));
        locals.var_dmob_dn6 = (0.5 * (locals.var_dmob_dn6 + (((locals.var_dmob_dn6 * assign10390_e9774) + (assign10390_e9771 * locals.var_dmob_dn6)) / (2.0 * assign10390_e9782))));
        locals.var_dmob_dn7 = (0.5 * (locals.var_dmob_dn7 + (((locals.var_dmob_dn7 * assign10390_e9774) + (assign10390_e9771 * locals.var_dmob_dn7)) / (2.0 * assign10390_e9782))));
        locals.var_dmob_dn8 = (0.5 * (locals.var_dmob_dn8 + (((locals.var_dmob_dn8 * assign10390_e9774) + (assign10390_e9771 * locals.var_dmob_dn8)) / (2.0 * assign10390_e9782))));
        locals.var_dmob_rv = 0.0;

        let assign10400_e9787: f64 = (locals.var_dmob / p.p11);
        locals.var_dmob = assign10400_e9787;
        locals.var_dmob_dn3 = (locals.var_dmob_dn3 / p.p11);
        locals.var_dmob_dn4 = (locals.var_dmob_dn4 / p.p11);
        locals.var_dmob_dn5 = (locals.var_dmob_dn5 / p.p11);
        locals.var_dmob_dn6 = (locals.var_dmob_dn6 / p.p11);
        locals.var_dmob_dn7 = (locals.var_dmob_dn7 / p.p11);
        locals.var_dmob_dn8 = (locals.var_dmob_dn8 / p.p11);
        locals.var_dmob_rv = 0.0;

        let assign10410_e9790: f64 = (locals.var_u0_t / locals.var_dmob);
        locals.var_ueff1 = assign10410_e9790;
        locals.var_ueff1_dn3 = (-((locals.var_u0_t * locals.var_dmob_dn3) / (locals.var_dmob * locals.var_dmob)));
        locals.var_ueff1_dn4 = (((locals.var_u0_t_dn4 * locals.var_dmob) - (locals.var_u0_t * locals.var_dmob_dn4)) / (locals.var_dmob * locals.var_dmob));
        locals.var_ueff1_dn5 = (-((locals.var_u0_t * locals.var_dmob_dn5) / (locals.var_dmob * locals.var_dmob)));
        locals.var_ueff1_dn6 = (-((locals.var_u0_t * locals.var_dmob_dn6) / (locals.var_dmob * locals.var_dmob)));
        locals.var_ueff1_dn7 = (-((locals.var_u0_t * locals.var_dmob_dn7) / (locals.var_dmob * locals.var_dmob)));
        locals.var_ueff1_dn8 = (-((locals.var_u0_t * locals.var_dmob_dn8) / (locals.var_dmob * locals.var_dmob)));
        locals.var_ueff1_rv = 0.0;

        let assign10420_e9795: f64 = (locals.var_qia / locals.var_qb0);
        let assign10420_e9796: f64 = (assign10420_e9795).abs();
        let assign10420_e9797: f64 = (1.0 + assign10420_e9796);
        let assign10420_e9798: f64 = (0.5 * assign10420_e9797);
        let assign10420_e9800: f64 = (assign10420_e9798).powf(locals.var_ucs2_i);
        locals.var_t2__blk102 = assign10420_e9800;
        locals.var_t2__blk102_dn3 = if 0.0 == 0.0 && ((locals.var_ucs2_i) as f64).is_finite() && ((locals.var_ucs2_i) as f64).fract() == 0.0 { if locals.var_ucs2_i == 0.0 { 0.0 } else { (locals.var_ucs2_i * ((assign10420_e9798).powf(locals.var_ucs2_i - 1.0) * (0.5 * if assign10420_e9795 >= 0.0 { (locals.var_qia_dn3 / locals.var_qb0) } else { (-(locals.var_qia_dn3 / locals.var_qb0)) }))) } } else { (assign10420_e9800 * (locals.var_ucs2_i * ((0.5 * if assign10420_e9795 >= 0.0 { (locals.var_qia_dn3 / locals.var_qb0) } else { (-(locals.var_qia_dn3 / locals.var_qb0)) }) / assign10420_e9798))) };
        locals.var_t2__blk102_dn4 = if 0.0 == 0.0 && ((locals.var_ucs2_i) as f64).is_finite() && ((locals.var_ucs2_i) as f64).fract() == 0.0 { if locals.var_ucs2_i == 0.0 { 0.0 } else { (locals.var_ucs2_i * ((assign10420_e9798).powf(locals.var_ucs2_i - 1.0) * (0.5 * if assign10420_e9795 >= 0.0 { (locals.var_qia_dn4 / locals.var_qb0) } else { (-(locals.var_qia_dn4 / locals.var_qb0)) }))) } } else { (assign10420_e9800 * (locals.var_ucs2_i * ((0.5 * if assign10420_e9795 >= 0.0 { (locals.var_qia_dn4 / locals.var_qb0) } else { (-(locals.var_qia_dn4 / locals.var_qb0)) }) / assign10420_e9798))) };
        locals.var_t2__blk102_dn5 = if 0.0 == 0.0 && ((locals.var_ucs2_i) as f64).is_finite() && ((locals.var_ucs2_i) as f64).fract() == 0.0 { if locals.var_ucs2_i == 0.0 { 0.0 } else { (locals.var_ucs2_i * ((assign10420_e9798).powf(locals.var_ucs2_i - 1.0) * (0.5 * if assign10420_e9795 >= 0.0 { (locals.var_qia_dn5 / locals.var_qb0) } else { (-(locals.var_qia_dn5 / locals.var_qb0)) }))) } } else { (assign10420_e9800 * (locals.var_ucs2_i * ((0.5 * if assign10420_e9795 >= 0.0 { (locals.var_qia_dn5 / locals.var_qb0) } else { (-(locals.var_qia_dn5 / locals.var_qb0)) }) / assign10420_e9798))) };
        locals.var_t2__blk102_dn6 = if 0.0 == 0.0 && ((locals.var_ucs2_i) as f64).is_finite() && ((locals.var_ucs2_i) as f64).fract() == 0.0 { if locals.var_ucs2_i == 0.0 { 0.0 } else { (locals.var_ucs2_i * ((assign10420_e9798).powf(locals.var_ucs2_i - 1.0) * (0.5 * if assign10420_e9795 >= 0.0 { (locals.var_qia_dn6 / locals.var_qb0) } else { (-(locals.var_qia_dn6 / locals.var_qb0)) }))) } } else { (assign10420_e9800 * (locals.var_ucs2_i * ((0.5 * if assign10420_e9795 >= 0.0 { (locals.var_qia_dn6 / locals.var_qb0) } else { (-(locals.var_qia_dn6 / locals.var_qb0)) }) / assign10420_e9798))) };
        locals.var_t2__blk102_dn7 = if 0.0 == 0.0 && ((locals.var_ucs2_i) as f64).is_finite() && ((locals.var_ucs2_i) as f64).fract() == 0.0 { if locals.var_ucs2_i == 0.0 { 0.0 } else { (locals.var_ucs2_i * ((assign10420_e9798).powf(locals.var_ucs2_i - 1.0) * (0.5 * if assign10420_e9795 >= 0.0 { (locals.var_qia_dn7 / locals.var_qb0) } else { (-(locals.var_qia_dn7 / locals.var_qb0)) }))) } } else { (assign10420_e9800 * (locals.var_ucs2_i * ((0.5 * if assign10420_e9795 >= 0.0 { (locals.var_qia_dn7 / locals.var_qb0) } else { (-(locals.var_qia_dn7 / locals.var_qb0)) }) / assign10420_e9798))) };
        locals.var_t2__blk102_dn8 = if 0.0 == 0.0 && ((locals.var_ucs2_i) as f64).is_finite() && ((locals.var_ucs2_i) as f64).fract() == 0.0 { if locals.var_ucs2_i == 0.0 { 0.0 } else { (locals.var_ucs2_i * ((assign10420_e9798).powf(locals.var_ucs2_i - 1.0) * (0.5 * if assign10420_e9795 >= 0.0 { (locals.var_qia_dn8 / locals.var_qb0) } else { (-(locals.var_qia_dn8 / locals.var_qb0)) }))) } } else { (assign10420_e9800 * (locals.var_ucs2_i * ((0.5 * if assign10420_e9795 >= 0.0 { (locals.var_qia_dn8 / locals.var_qb0) } else { (-(locals.var_qia_dn8 / locals.var_qb0)) }) / assign10420_e9798))) };
        locals.var_t2__blk102_rv = 0.0;

        let assign10430_e9804: f64 = (locals.var_vbgx * locals.var_uc2_i);
        let assign10430_e9805: f64 = (locals.var_ua2_i + assign10430_e9804);
        let assign10430_e9807: f64 = (locals.var_eeffm2).abs();
        let assign10430_e9811: f64 = (locals.var_eub2_i * locals.var_vbgx);
        let assign10430_e9812: f64 = (locals.var_eu2_i + assign10430_e9811);
        let assign10430_e9813: f64 = (assign10430_e9807).powf(assign10430_e9812);
        let assign10430_e9814: f64 = (assign10430_e9805 * assign10430_e9813);
        let assign10430_e9818: f64 = (locals.var_vbgx * locals.var_udb2_i);
        let assign10430_e9819: f64 = (locals.var_ud2_i + assign10430_e9818);
        let assign10430_e9821: f64 = (assign10430_e9819 / locals.var_t2__blk102);
        let assign10430_e9822: f64 = (assign10430_e9814 + assign10430_e9821);
        locals.var_t3__blk103 = assign10430_e9822;
        locals.var_t3__blk103_dn3 = ((((locals.var_vbgx_dn3 * locals.var_uc2_i) * assign10430_e9813) + (assign10430_e9805 * if (locals.var_eub2_i * locals.var_vbgx_dn3) == 0.0 && ((assign10430_e9812) as f64).is_finite() && ((assign10430_e9812) as f64).fract() == 0.0 { if assign10430_e9812 == 0.0 { 0.0 } else { (assign10430_e9812 * ((assign10430_e9807).powf(assign10430_e9812 - 1.0) * if locals.var_eeffm2 >= 0.0 { locals.var_eeffm2_dn3 } else { (-locals.var_eeffm2_dn3) })) } } else { (assign10430_e9813 * (((locals.var_eub2_i * locals.var_vbgx_dn3) * (assign10430_e9807).ln()) + (assign10430_e9812 * (if locals.var_eeffm2 >= 0.0 { locals.var_eeffm2_dn3 } else { (-locals.var_eeffm2_dn3) } / assign10430_e9807)))) })) + ((((locals.var_vbgx_dn3 * locals.var_udb2_i) * locals.var_t2__blk102) - (assign10430_e9819 * locals.var_t2__blk102_dn3)) / (locals.var_t2__blk102 * locals.var_t2__blk102)));
        locals.var_t3__blk103_dn4 = ((assign10430_e9805 * if 0.0 == 0.0 && ((assign10430_e9812) as f64).is_finite() && ((assign10430_e9812) as f64).fract() == 0.0 { if assign10430_e9812 == 0.0 { 0.0 } else { (assign10430_e9812 * ((assign10430_e9807).powf(assign10430_e9812 - 1.0) * if locals.var_eeffm2 >= 0.0 { locals.var_eeffm2_dn4 } else { (-locals.var_eeffm2_dn4) })) } } else { (assign10430_e9813 * (assign10430_e9812 * (if locals.var_eeffm2 >= 0.0 { locals.var_eeffm2_dn4 } else { (-locals.var_eeffm2_dn4) } / assign10430_e9807))) }) + (-((assign10430_e9819 * locals.var_t2__blk102_dn4) / (locals.var_t2__blk102 * locals.var_t2__blk102))));
        locals.var_t3__blk103_dn5 = ((((locals.var_vbgx_dn5 * locals.var_uc2_i) * assign10430_e9813) + (assign10430_e9805 * if (locals.var_eub2_i * locals.var_vbgx_dn5) == 0.0 && ((assign10430_e9812) as f64).is_finite() && ((assign10430_e9812) as f64).fract() == 0.0 { if assign10430_e9812 == 0.0 { 0.0 } else { (assign10430_e9812 * ((assign10430_e9807).powf(assign10430_e9812 - 1.0) * if locals.var_eeffm2 >= 0.0 { locals.var_eeffm2_dn5 } else { (-locals.var_eeffm2_dn5) })) } } else { (assign10430_e9813 * (((locals.var_eub2_i * locals.var_vbgx_dn5) * (assign10430_e9807).ln()) + (assign10430_e9812 * (if locals.var_eeffm2 >= 0.0 { locals.var_eeffm2_dn5 } else { (-locals.var_eeffm2_dn5) } / assign10430_e9807)))) })) + ((((locals.var_vbgx_dn5 * locals.var_udb2_i) * locals.var_t2__blk102) - (assign10430_e9819 * locals.var_t2__blk102_dn5)) / (locals.var_t2__blk102 * locals.var_t2__blk102)));
        locals.var_t3__blk103_dn6 = ((((locals.var_vbgx_dn6 * locals.var_uc2_i) * assign10430_e9813) + (assign10430_e9805 * if (locals.var_eub2_i * locals.var_vbgx_dn6) == 0.0 && ((assign10430_e9812) as f64).is_finite() && ((assign10430_e9812) as f64).fract() == 0.0 { if assign10430_e9812 == 0.0 { 0.0 } else { (assign10430_e9812 * ((assign10430_e9807).powf(assign10430_e9812 - 1.0) * if locals.var_eeffm2 >= 0.0 { locals.var_eeffm2_dn6 } else { (-locals.var_eeffm2_dn6) })) } } else { (assign10430_e9813 * (((locals.var_eub2_i * locals.var_vbgx_dn6) * (assign10430_e9807).ln()) + (assign10430_e9812 * (if locals.var_eeffm2 >= 0.0 { locals.var_eeffm2_dn6 } else { (-locals.var_eeffm2_dn6) } / assign10430_e9807)))) })) + ((((locals.var_vbgx_dn6 * locals.var_udb2_i) * locals.var_t2__blk102) - (assign10430_e9819 * locals.var_t2__blk102_dn6)) / (locals.var_t2__blk102 * locals.var_t2__blk102)));
        locals.var_t3__blk103_dn7 = ((assign10430_e9805 * if 0.0 == 0.0 && ((assign10430_e9812) as f64).is_finite() && ((assign10430_e9812) as f64).fract() == 0.0 { if assign10430_e9812 == 0.0 { 0.0 } else { (assign10430_e9812 * ((assign10430_e9807).powf(assign10430_e9812 - 1.0) * if locals.var_eeffm2 >= 0.0 { locals.var_eeffm2_dn7 } else { (-locals.var_eeffm2_dn7) })) } } else { (assign10430_e9813 * (assign10430_e9812 * (if locals.var_eeffm2 >= 0.0 { locals.var_eeffm2_dn7 } else { (-locals.var_eeffm2_dn7) } / assign10430_e9807))) }) + (-((assign10430_e9819 * locals.var_t2__blk102_dn7) / (locals.var_t2__blk102 * locals.var_t2__blk102))));
        locals.var_t3__blk103_dn8 = ((assign10430_e9805 * if 0.0 == 0.0 && ((assign10430_e9812) as f64).is_finite() && ((assign10430_e9812) as f64).fract() == 0.0 { if assign10430_e9812 == 0.0 { 0.0 } else { (assign10430_e9812 * ((assign10430_e9807).powf(assign10430_e9812 - 1.0) * if locals.var_eeffm2 >= 0.0 { locals.var_eeffm2_dn8 } else { (-locals.var_eeffm2_dn8) })) } } else { (assign10430_e9813 * (assign10430_e9812 * (if locals.var_eeffm2 >= 0.0 { locals.var_eeffm2_dn8 } else { (-locals.var_eeffm2_dn8) } / assign10430_e9807))) }) + (-((assign10430_e9819 * locals.var_t2__blk102_dn8) / (locals.var_t2__blk102 * locals.var_t2__blk102))));
        locals.var_t3__blk103_rv = 0.0;

        let assign10440_e9825: f64 = (1.0 + locals.var_t3__blk103);
        locals.var_dmob = assign10440_e9825;
        locals.var_dmob_dn3 = locals.var_t3__blk103_dn3;
        locals.var_dmob_dn4 = locals.var_t3__blk103_dn4;
        locals.var_dmob_dn5 = locals.var_t3__blk103_dn5;
        locals.var_dmob_dn6 = locals.var_t3__blk103_dn6;
        locals.var_dmob_dn7 = locals.var_t3__blk103_dn7;
        locals.var_dmob_dn8 = locals.var_t3__blk103_dn8;
        locals.var_dmob_rv = 0.0;

        let assign10450_e9829: f64 = (locals.var_dmob + 1.0);
        let assign10450_e9832: f64 = (locals.var_dmob - 1.0);
        let assign10450_e9835: f64 = (locals.var_dmob - 1.0);
        let assign10450_e9836: f64 = (assign10450_e9832 * assign10450_e9835);
        let assign10450_e9839: f64 = (0.25 * p.p154);
        let assign10450_e9841: f64 = (assign10450_e9839 * p.p154);
        let assign10450_e9842: f64 = (assign10450_e9836 + assign10450_e9841);
        let assign10450_e9843: f64 = (assign10450_e9842).sqrt();
        let assign10450_e9844: f64 = (assign10450_e9829 + assign10450_e9843);
        let assign10450_e9845: f64 = (0.5 * assign10450_e9844);
        locals.var_dmob = assign10450_e9845;
        locals.var_dmob_dn3 = (0.5 * (locals.var_dmob_dn3 + (((locals.var_dmob_dn3 * assign10450_e9835) + (assign10450_e9832 * locals.var_dmob_dn3)) / (2.0 * assign10450_e9843))));
        locals.var_dmob_dn4 = (0.5 * (locals.var_dmob_dn4 + (((locals.var_dmob_dn4 * assign10450_e9835) + (assign10450_e9832 * locals.var_dmob_dn4)) / (2.0 * assign10450_e9843))));
        locals.var_dmob_dn5 = (0.5 * (locals.var_dmob_dn5 + (((locals.var_dmob_dn5 * assign10450_e9835) + (assign10450_e9832 * locals.var_dmob_dn5)) / (2.0 * assign10450_e9843))));
        locals.var_dmob_dn6 = (0.5 * (locals.var_dmob_dn6 + (((locals.var_dmob_dn6 * assign10450_e9835) + (assign10450_e9832 * locals.var_dmob_dn6)) / (2.0 * assign10450_e9843))));
        locals.var_dmob_dn7 = (0.5 * (locals.var_dmob_dn7 + (((locals.var_dmob_dn7 * assign10450_e9835) + (assign10450_e9832 * locals.var_dmob_dn7)) / (2.0 * assign10450_e9843))));
        locals.var_dmob_dn8 = (0.5 * (locals.var_dmob_dn8 + (((locals.var_dmob_dn8 * assign10450_e9835) + (assign10450_e9832 * locals.var_dmob_dn8)) / (2.0 * assign10450_e9843))));
        locals.var_dmob_rv = 0.0;

        let assign10460_e9848: f64 = (locals.var_dmob / p.p11);
        locals.var_dmob = assign10460_e9848;
        locals.var_dmob_dn3 = (locals.var_dmob_dn3 / p.p11);
        locals.var_dmob_dn4 = (locals.var_dmob_dn4 / p.p11);
        locals.var_dmob_dn5 = (locals.var_dmob_dn5 / p.p11);
        locals.var_dmob_dn6 = (locals.var_dmob_dn6 / p.p11);
        locals.var_dmob_dn7 = (locals.var_dmob_dn7 / p.p11);
        locals.var_dmob_dn8 = (locals.var_dmob_dn8 / p.p11);
        locals.var_dmob_rv = 0.0;

        let assign10470_e9851: f64 = (locals.var_u02_i / locals.var_dmob);
        locals.var_ueff2 = assign10470_e9851;
        locals.var_ueff2_dn3 = (-((locals.var_u02_i * locals.var_dmob_dn3) / (locals.var_dmob * locals.var_dmob)));
        locals.var_ueff2_dn4 = (-((locals.var_u02_i * locals.var_dmob_dn4) / (locals.var_dmob * locals.var_dmob)));
        locals.var_ueff2_dn5 = (-((locals.var_u02_i * locals.var_dmob_dn5) / (locals.var_dmob * locals.var_dmob)));
        locals.var_ueff2_dn6 = (-((locals.var_u02_i * locals.var_dmob_dn6) / (locals.var_dmob * locals.var_dmob)));
        locals.var_ueff2_dn7 = (-((locals.var_u02_i * locals.var_dmob_dn7) / (locals.var_dmob * locals.var_dmob)));
        locals.var_ueff2_dn8 = (-((locals.var_u02_i * locals.var_dmob_dn8) / (locals.var_dmob * locals.var_dmob)));
        locals.var_ueff2_rv = 0.0;

        let assign10480_e9855: f64 = (locals.var_qfronts + locals.var_qfrontd);
        let assign10480_e9858: f64 = (2.0 * locals.var_cox1);
        let assign10480_e9859: f64 = (assign10480_e9855 / assign10480_e9858);
        let assign10480_e9860: f64 = (locals.var_vgfb1eff - assign10480_e9859);
        locals.var_t0 = assign10480_e9860;
        locals.var_t0_dn3 = (locals.var_vgfb1eff_dn3 - ((locals.var_qfronts_dn3 + locals.var_qfrontd_dn3) / assign10480_e9858));
        locals.var_t0_dn4 = (locals.var_vgfb1eff_dn4 - ((locals.var_qfronts_dn4 + locals.var_qfrontd_dn4) / assign10480_e9858));
        locals.var_t0_dn5 = (locals.var_vgfb1eff_dn5 - ((locals.var_qfronts_dn5 + locals.var_qfrontd_dn5) / assign10480_e9858));
        locals.var_t0_dn6 = (locals.var_vgfb1eff_dn6 - ((locals.var_qfronts_dn6 + locals.var_qfrontd_dn6) / assign10480_e9858));
        locals.var_t0_dn7 = (locals.var_vgfb1eff_dn7 - ((locals.var_qfronts_dn7 + locals.var_qfrontd_dn7) / assign10480_e9858));
        locals.var_t0_dn8 = (locals.var_vgfb1eff_dn8 - ((locals.var_qfronts_dn8 + locals.var_qfrontd_dn8) / assign10480_e9858));
        locals.var_t0_rv = 0.0;

        let assign10490_e9863: f64 = (locals.var_vgfb2 - locals.var_dvth_all);
        let assign10490_e9866: f64 = (locals.var_qbacks + locals.var_qbackd);
        let assign10490_e9869: f64 = (2.0 * locals.var_cox2);
        let assign10490_e9870: f64 = (assign10490_e9866 / assign10490_e9869);
        let assign10490_e9871: f64 = (assign10490_e9863 - assign10490_e9870);
        locals.var_t1 = assign10490_e9871;
        locals.var_t1_dn3 = ((locals.var_vgfb2_dn3 - locals.var_dvth_all_dn3) - ((locals.var_qbacks_dn3 + locals.var_qbackd_dn3) / assign10490_e9869));
        locals.var_t1_dn4 = ((locals.var_vgfb2_dn4 - locals.var_dvth_all_dn4) - ((locals.var_qbacks_dn4 + locals.var_qbackd_dn4) / assign10490_e9869));
        locals.var_t1_dn5 = ((locals.var_vgfb2_dn5 - locals.var_dvth_all_dn5) - ((locals.var_qbacks_dn5 + locals.var_qbackd_dn5) / assign10490_e9869));
        locals.var_t1_dn6 = ((locals.var_vgfb2_dn6 - locals.var_dvth_all_dn6) - ((locals.var_qbacks_dn6 + locals.var_qbackd_dn6) / assign10490_e9869));
        locals.var_t1_dn7 = ((locals.var_vgfb2_dn7 - locals.var_dvth_all_dn7) - ((locals.var_qbacks_dn7 + locals.var_qbackd_dn7) / assign10490_e9869));
        locals.var_t1_dn8 = ((locals.var_vgfb2_dn8 - locals.var_dvth_all_dn8) - ((locals.var_qbacks_dn8 + locals.var_qbackd_dn8) / assign10490_e9869));
        locals.var_t1_rv = 0.0;

        let assign10500_e9874: f64 = (locals.var_t0 / locals.var_nvtm);
        let assign10500_e9875: f64 = (assign10500_e9874).exp();
        let assign10500_e9878: f64 = (locals.var_t0 / locals.var_nvtm);
        let assign10500_e9879: f64 = (assign10500_e9878).exp();
        let assign10500_e9882: f64 = (locals.var_t1 / locals.var_nvtm);
        let assign10500_e9883: f64 = (assign10500_e9882).exp();
        let assign10500_e9884: f64 = (assign10500_e9879 + assign10500_e9883);
        let assign10500_e9885: f64 = (assign10500_e9875 / assign10500_e9884);
        locals.var_w1 = assign10500_e9885;
        locals.var_w1_dn3 = ((((assign10500_e9875 * (((locals.var_t0_dn3 * locals.var_nvtm) - (locals.var_t0 * locals.var_nvtm_dn3)) / (locals.var_nvtm * locals.var_nvtm))) * assign10500_e9884) - (assign10500_e9875 * ((assign10500_e9879 * (((locals.var_t0_dn3 * locals.var_nvtm) - (locals.var_t0 * locals.var_nvtm_dn3)) / (locals.var_nvtm * locals.var_nvtm))) + (assign10500_e9883 * (((locals.var_t1_dn3 * locals.var_nvtm) - (locals.var_t1 * locals.var_nvtm_dn3)) / (locals.var_nvtm * locals.var_nvtm)))))) / (assign10500_e9884 * assign10500_e9884));
        locals.var_w1_dn4 = ((((assign10500_e9875 * (((locals.var_t0_dn4 * locals.var_nvtm) - (locals.var_t0 * locals.var_nvtm_dn4)) / (locals.var_nvtm * locals.var_nvtm))) * assign10500_e9884) - (assign10500_e9875 * ((assign10500_e9879 * (((locals.var_t0_dn4 * locals.var_nvtm) - (locals.var_t0 * locals.var_nvtm_dn4)) / (locals.var_nvtm * locals.var_nvtm))) + (assign10500_e9883 * (((locals.var_t1_dn4 * locals.var_nvtm) - (locals.var_t1 * locals.var_nvtm_dn4)) / (locals.var_nvtm * locals.var_nvtm)))))) / (assign10500_e9884 * assign10500_e9884));
        locals.var_w1_dn5 = ((((assign10500_e9875 * (((locals.var_t0_dn5 * locals.var_nvtm) - (locals.var_t0 * locals.var_nvtm_dn5)) / (locals.var_nvtm * locals.var_nvtm))) * assign10500_e9884) - (assign10500_e9875 * ((assign10500_e9879 * (((locals.var_t0_dn5 * locals.var_nvtm) - (locals.var_t0 * locals.var_nvtm_dn5)) / (locals.var_nvtm * locals.var_nvtm))) + (assign10500_e9883 * (((locals.var_t1_dn5 * locals.var_nvtm) - (locals.var_t1 * locals.var_nvtm_dn5)) / (locals.var_nvtm * locals.var_nvtm)))))) / (assign10500_e9884 * assign10500_e9884));
        locals.var_w1_dn6 = ((((assign10500_e9875 * (((locals.var_t0_dn6 * locals.var_nvtm) - (locals.var_t0 * locals.var_nvtm_dn6)) / (locals.var_nvtm * locals.var_nvtm))) * assign10500_e9884) - (assign10500_e9875 * ((assign10500_e9879 * (((locals.var_t0_dn6 * locals.var_nvtm) - (locals.var_t0 * locals.var_nvtm_dn6)) / (locals.var_nvtm * locals.var_nvtm))) + (assign10500_e9883 * (((locals.var_t1_dn6 * locals.var_nvtm) - (locals.var_t1 * locals.var_nvtm_dn6)) / (locals.var_nvtm * locals.var_nvtm)))))) / (assign10500_e9884 * assign10500_e9884));
        locals.var_w1_dn7 = ((((assign10500_e9875 * (((locals.var_t0_dn7 * locals.var_nvtm) - (locals.var_t0 * locals.var_nvtm_dn7)) / (locals.var_nvtm * locals.var_nvtm))) * assign10500_e9884) - (assign10500_e9875 * ((assign10500_e9879 * (((locals.var_t0_dn7 * locals.var_nvtm) - (locals.var_t0 * locals.var_nvtm_dn7)) / (locals.var_nvtm * locals.var_nvtm))) + (assign10500_e9883 * (((locals.var_t1_dn7 * locals.var_nvtm) - (locals.var_t1 * locals.var_nvtm_dn7)) / (locals.var_nvtm * locals.var_nvtm)))))) / (assign10500_e9884 * assign10500_e9884));
        locals.var_w1_dn8 = ((((assign10500_e9875 * (((locals.var_t0_dn8 * locals.var_nvtm) - (locals.var_t0 * locals.var_nvtm_dn8)) / (locals.var_nvtm * locals.var_nvtm))) * assign10500_e9884) - (assign10500_e9875 * ((assign10500_e9879 * (((locals.var_t0_dn8 * locals.var_nvtm) - (locals.var_t0 * locals.var_nvtm_dn8)) / (locals.var_nvtm * locals.var_nvtm))) + (assign10500_e9883 * (((locals.var_t1_dn8 * locals.var_nvtm) - (locals.var_t1 * locals.var_nvtm_dn8)) / (locals.var_nvtm * locals.var_nvtm)))))) / (assign10500_e9884 * assign10500_e9884));
        locals.var_w1_rv = 0.0;

        let assign10510_e9888: f64 = (locals.var_t1 / locals.var_nvtm);
        let assign10510_e9889: f64 = (assign10510_e9888).exp();
        let assign10510_e9892: f64 = (locals.var_t0 / locals.var_nvtm);
        let assign10510_e9893: f64 = (assign10510_e9892).exp();
        let assign10510_e9896: f64 = (locals.var_t1 / locals.var_nvtm);
        let assign10510_e9897: f64 = (assign10510_e9896).exp();
        let assign10510_e9898: f64 = (assign10510_e9893 + assign10510_e9897);
        let assign10510_e9899: f64 = (assign10510_e9889 / assign10510_e9898);
        locals.var_w2 = assign10510_e9899;
        locals.var_w2_dn3 = ((((assign10510_e9889 * (((locals.var_t1_dn3 * locals.var_nvtm) - (locals.var_t1 * locals.var_nvtm_dn3)) / (locals.var_nvtm * locals.var_nvtm))) * assign10510_e9898) - (assign10510_e9889 * ((assign10510_e9893 * (((locals.var_t0_dn3 * locals.var_nvtm) - (locals.var_t0 * locals.var_nvtm_dn3)) / (locals.var_nvtm * locals.var_nvtm))) + (assign10510_e9897 * (((locals.var_t1_dn3 * locals.var_nvtm) - (locals.var_t1 * locals.var_nvtm_dn3)) / (locals.var_nvtm * locals.var_nvtm)))))) / (assign10510_e9898 * assign10510_e9898));
        locals.var_w2_dn4 = ((((assign10510_e9889 * (((locals.var_t1_dn4 * locals.var_nvtm) - (locals.var_t1 * locals.var_nvtm_dn4)) / (locals.var_nvtm * locals.var_nvtm))) * assign10510_e9898) - (assign10510_e9889 * ((assign10510_e9893 * (((locals.var_t0_dn4 * locals.var_nvtm) - (locals.var_t0 * locals.var_nvtm_dn4)) / (locals.var_nvtm * locals.var_nvtm))) + (assign10510_e9897 * (((locals.var_t1_dn4 * locals.var_nvtm) - (locals.var_t1 * locals.var_nvtm_dn4)) / (locals.var_nvtm * locals.var_nvtm)))))) / (assign10510_e9898 * assign10510_e9898));
        locals.var_w2_dn5 = ((((assign10510_e9889 * (((locals.var_t1_dn5 * locals.var_nvtm) - (locals.var_t1 * locals.var_nvtm_dn5)) / (locals.var_nvtm * locals.var_nvtm))) * assign10510_e9898) - (assign10510_e9889 * ((assign10510_e9893 * (((locals.var_t0_dn5 * locals.var_nvtm) - (locals.var_t0 * locals.var_nvtm_dn5)) / (locals.var_nvtm * locals.var_nvtm))) + (assign10510_e9897 * (((locals.var_t1_dn5 * locals.var_nvtm) - (locals.var_t1 * locals.var_nvtm_dn5)) / (locals.var_nvtm * locals.var_nvtm)))))) / (assign10510_e9898 * assign10510_e9898));
        locals.var_w2_dn6 = ((((assign10510_e9889 * (((locals.var_t1_dn6 * locals.var_nvtm) - (locals.var_t1 * locals.var_nvtm_dn6)) / (locals.var_nvtm * locals.var_nvtm))) * assign10510_e9898) - (assign10510_e9889 * ((assign10510_e9893 * (((locals.var_t0_dn6 * locals.var_nvtm) - (locals.var_t0 * locals.var_nvtm_dn6)) / (locals.var_nvtm * locals.var_nvtm))) + (assign10510_e9897 * (((locals.var_t1_dn6 * locals.var_nvtm) - (locals.var_t1 * locals.var_nvtm_dn6)) / (locals.var_nvtm * locals.var_nvtm)))))) / (assign10510_e9898 * assign10510_e9898));
        locals.var_w2_dn7 = ((((assign10510_e9889 * (((locals.var_t1_dn7 * locals.var_nvtm) - (locals.var_t1 * locals.var_nvtm_dn7)) / (locals.var_nvtm * locals.var_nvtm))) * assign10510_e9898) - (assign10510_e9889 * ((assign10510_e9893 * (((locals.var_t0_dn7 * locals.var_nvtm) - (locals.var_t0 * locals.var_nvtm_dn7)) / (locals.var_nvtm * locals.var_nvtm))) + (assign10510_e9897 * (((locals.var_t1_dn7 * locals.var_nvtm) - (locals.var_t1 * locals.var_nvtm_dn7)) / (locals.var_nvtm * locals.var_nvtm)))))) / (assign10510_e9898 * assign10510_e9898));
        locals.var_w2_dn8 = ((((assign10510_e9889 * (((locals.var_t1_dn8 * locals.var_nvtm) - (locals.var_t1 * locals.var_nvtm_dn8)) / (locals.var_nvtm * locals.var_nvtm))) * assign10510_e9898) - (assign10510_e9889 * ((assign10510_e9893 * (((locals.var_t0_dn8 * locals.var_nvtm) - (locals.var_t0 * locals.var_nvtm_dn8)) / (locals.var_nvtm * locals.var_nvtm))) + (assign10510_e9897 * (((locals.var_t1_dn8 * locals.var_nvtm) - (locals.var_t1 * locals.var_nvtm_dn8)) / (locals.var_nvtm * locals.var_nvtm)))))) / (assign10510_e9898 * assign10510_e9898));
        locals.var_w2_rv = 0.0;

        let assign10520_e9902: f64 = (locals.var_w1 * locals.var_ueff1);
        let assign10520_e9905: f64 = (locals.var_w2 * locals.var_ueff2);
        let assign10520_e9906: f64 = (assign10520_e9902 + assign10520_e9905);
        locals.var_utotal = assign10520_e9906;
        locals.var_utotal_dn3 = (((locals.var_w1_dn3 * locals.var_ueff1) + (locals.var_w1 * locals.var_ueff1_dn3)) + ((locals.var_w2_dn3 * locals.var_ueff2) + (locals.var_w2 * locals.var_ueff2_dn3)));
        locals.var_utotal_dn4 = (((locals.var_w1_dn4 * locals.var_ueff1) + (locals.var_w1 * locals.var_ueff1_dn4)) + ((locals.var_w2_dn4 * locals.var_ueff2) + (locals.var_w2 * locals.var_ueff2_dn4)));
        locals.var_utotal_dn5 = (((locals.var_w1_dn5 * locals.var_ueff1) + (locals.var_w1 * locals.var_ueff1_dn5)) + ((locals.var_w2_dn5 * locals.var_ueff2) + (locals.var_w2 * locals.var_ueff2_dn5)));
        locals.var_utotal_dn6 = (((locals.var_w1_dn6 * locals.var_ueff1) + (locals.var_w1 * locals.var_ueff1_dn6)) + ((locals.var_w2_dn6 * locals.var_ueff2) + (locals.var_w2 * locals.var_ueff2_dn6)));
        locals.var_utotal_dn7 = (((locals.var_w1_dn7 * locals.var_ueff1) + (locals.var_w1 * locals.var_ueff1_dn7)) + ((locals.var_w2_dn7 * locals.var_ueff2) + (locals.var_w2 * locals.var_ueff2_dn7)));
        locals.var_utotal_dn8 = (((locals.var_w1_dn8 * locals.var_ueff1) + (locals.var_w1 * locals.var_ueff1_dn8)) + ((locals.var_w2_dn8 * locals.var_ueff2) + (locals.var_w2 * locals.var_ueff2_dn8)));
        locals.var_utotal_rv = 0.0;

        let assign10530_e9909: f64 = (locals.var_utotal * locals.var_cox1);
        let assign10530_e9911: f64 = (assign10530_e9909 * locals.var_weff);
        let assign10530_e9913: f64 = (assign10530_e9911 / locals.var_leff);
        locals.var_beta = assign10530_e9913;
        locals.var_beta_dn3 = (((locals.var_utotal_dn3 * locals.var_cox1) * locals.var_weff) / locals.var_leff);
        locals.var_beta_dn4 = (((locals.var_utotal_dn4 * locals.var_cox1) * locals.var_weff) / locals.var_leff);
        locals.var_beta_dn5 = (((locals.var_utotal_dn5 * locals.var_cox1) * locals.var_weff) / locals.var_leff);
        locals.var_beta_dn6 = (((locals.var_utotal_dn6 * locals.var_cox1) * locals.var_weff) / locals.var_leff);
        locals.var_beta_dn7 = (((locals.var_utotal_dn7 * locals.var_cox1) * locals.var_weff) / locals.var_leff);
        locals.var_beta_dn8 = (((locals.var_utotal_dn8 * locals.var_cox1) * locals.var_weff) / locals.var_leff);
        locals.var_beta_rv = 0.0;

        let assign10540_e9918: f64 = (locals.var_eta_mu_cv * locals.var_qia);
        let assign10540_e9919: f64 = (locals.var_qba + assign10540_e9918);
        let assign10540_e9920: f64 = (locals.var_eefffactor * assign10540_e9919);
        locals.var_eeffm_cv = assign10540_e9920;
        locals.var_eeffm_cv_dn3 = (locals.var_eefffactor * (locals.var_eta_mu_cv * locals.var_qia_dn3));
        locals.var_eeffm_cv_dn4 = (locals.var_eefffactor * (locals.var_eta_mu_cv * locals.var_qia_dn4));
        locals.var_eeffm_cv_dn5 = (locals.var_eefffactor * (locals.var_eta_mu_cv * locals.var_qia_dn5));
        locals.var_eeffm_cv_dn6 = (locals.var_eefffactor * (locals.var_eta_mu_cv * locals.var_qia_dn6));
        locals.var_eeffm_cv_dn7 = (locals.var_eefffactor * (locals.var_eta_mu_cv * locals.var_qia_dn7));
        locals.var_eeffm_cv_dn8 = (locals.var_eefffactor * (locals.var_eta_mu_cv * locals.var_qia_dn8));
        locals.var_eeffm_cv_rv = 0.0;

        let assign10550_e9923: f64 = (locals.var_eeffm_cv).abs();
        let assign10550_e9925: f64 = (assign10550_e9923).powf(locals.var_eu_i);
        let assign10550_e9926: f64 = (locals.var_ua_t * assign10550_e9925);
        locals.var_t3 = assign10550_e9926;
        locals.var_t3_dn3 = (locals.var_ua_t * if 0.0 == 0.0 && ((locals.var_eu_i) as f64).is_finite() && ((locals.var_eu_i) as f64).fract() == 0.0 { if locals.var_eu_i == 0.0 { 0.0 } else { (locals.var_eu_i * ((assign10550_e9923).powf(locals.var_eu_i - 1.0) * if locals.var_eeffm_cv >= 0.0 { locals.var_eeffm_cv_dn3 } else { (-locals.var_eeffm_cv_dn3) })) } } else { (assign10550_e9925 * (locals.var_eu_i * (if locals.var_eeffm_cv >= 0.0 { locals.var_eeffm_cv_dn3 } else { (-locals.var_eeffm_cv_dn3) } / assign10550_e9923))) });
        locals.var_t3_dn4 = ((locals.var_ua_t_dn4 * assign10550_e9925) + (locals.var_ua_t * if 0.0 == 0.0 && ((locals.var_eu_i) as f64).is_finite() && ((locals.var_eu_i) as f64).fract() == 0.0 { if locals.var_eu_i == 0.0 { 0.0 } else { (locals.var_eu_i * ((assign10550_e9923).powf(locals.var_eu_i - 1.0) * if locals.var_eeffm_cv >= 0.0 { locals.var_eeffm_cv_dn4 } else { (-locals.var_eeffm_cv_dn4) })) } } else { (assign10550_e9925 * (locals.var_eu_i * (if locals.var_eeffm_cv >= 0.0 { locals.var_eeffm_cv_dn4 } else { (-locals.var_eeffm_cv_dn4) } / assign10550_e9923))) }));
        locals.var_t3_dn5 = (locals.var_ua_t * if 0.0 == 0.0 && ((locals.var_eu_i) as f64).is_finite() && ((locals.var_eu_i) as f64).fract() == 0.0 { if locals.var_eu_i == 0.0 { 0.0 } else { (locals.var_eu_i * ((assign10550_e9923).powf(locals.var_eu_i - 1.0) * if locals.var_eeffm_cv >= 0.0 { locals.var_eeffm_cv_dn5 } else { (-locals.var_eeffm_cv_dn5) })) } } else { (assign10550_e9925 * (locals.var_eu_i * (if locals.var_eeffm_cv >= 0.0 { locals.var_eeffm_cv_dn5 } else { (-locals.var_eeffm_cv_dn5) } / assign10550_e9923))) });
        locals.var_t3_dn6 = (locals.var_ua_t * if 0.0 == 0.0 && ((locals.var_eu_i) as f64).is_finite() && ((locals.var_eu_i) as f64).fract() == 0.0 { if locals.var_eu_i == 0.0 { 0.0 } else { (locals.var_eu_i * ((assign10550_e9923).powf(locals.var_eu_i - 1.0) * if locals.var_eeffm_cv >= 0.0 { locals.var_eeffm_cv_dn6 } else { (-locals.var_eeffm_cv_dn6) })) } } else { (assign10550_e9925 * (locals.var_eu_i * (if locals.var_eeffm_cv >= 0.0 { locals.var_eeffm_cv_dn6 } else { (-locals.var_eeffm_cv_dn6) } / assign10550_e9923))) });
        locals.var_t3_dn7 = (locals.var_ua_t * if 0.0 == 0.0 && ((locals.var_eu_i) as f64).is_finite() && ((locals.var_eu_i) as f64).fract() == 0.0 { if locals.var_eu_i == 0.0 { 0.0 } else { (locals.var_eu_i * ((assign10550_e9923).powf(locals.var_eu_i - 1.0) * if locals.var_eeffm_cv >= 0.0 { locals.var_eeffm_cv_dn7 } else { (-locals.var_eeffm_cv_dn7) })) } } else { (assign10550_e9925 * (locals.var_eu_i * (if locals.var_eeffm_cv >= 0.0 { locals.var_eeffm_cv_dn7 } else { (-locals.var_eeffm_cv_dn7) } / assign10550_e9923))) });
        locals.var_t3_dn8 = (locals.var_ua_t * if 0.0 == 0.0 && ((locals.var_eu_i) as f64).is_finite() && ((locals.var_eu_i) as f64).fract() == 0.0 { if locals.var_eu_i == 0.0 { 0.0 } else { (locals.var_eu_i * ((assign10550_e9923).powf(locals.var_eu_i - 1.0) * if locals.var_eeffm_cv >= 0.0 { locals.var_eeffm_cv_dn8 } else { (-locals.var_eeffm_cv_dn8) })) } } else { (assign10550_e9925 * (locals.var_eu_i * (if locals.var_eeffm_cv >= 0.0 { locals.var_eeffm_cv_dn8 } else { (-locals.var_eeffm_cv_dn8) } / assign10550_e9923))) });
        locals.var_t3_rv = 0.0;

        let assign10560_e9929: f64 = (1.0 + locals.var_t3);
        locals.var_dmob_cv = assign10560_e9929;
        locals.var_dmob_cv_dn3 = locals.var_t3_dn3;
        locals.var_dmob_cv_dn4 = locals.var_t3_dn4;
        locals.var_dmob_cv_dn5 = locals.var_t3_dn5;
        locals.var_dmob_cv_dn6 = locals.var_t3_dn6;
        locals.var_dmob_cv_dn7 = locals.var_t3_dn7;
        locals.var_dmob_cv_dn8 = locals.var_t3_dn8;
        locals.var_dmob_cv_rv = 0.0;

        let assign10570_e9933: f64 = (locals.var_dmob_cv + 1.0);
        let assign10570_e9936: f64 = (locals.var_dmob_cv - 1.0);
        let assign10570_e9939: f64 = (locals.var_dmob_cv - 1.0);
        let assign10570_e9940: f64 = (assign10570_e9936 * assign10570_e9939);
        let assign10570_e9943: f64 = (0.25 * p.p154);
        let assign10570_e9945: f64 = (assign10570_e9943 * p.p154);
        let assign10570_e9946: f64 = (assign10570_e9940 + assign10570_e9945);
        let assign10570_e9947: f64 = (assign10570_e9946).sqrt();
        let assign10570_e9948: f64 = (assign10570_e9933 + assign10570_e9947);
        let assign10570_e9949: f64 = (0.5 * assign10570_e9948);
        locals.var_dmob_cv = assign10570_e9949;
        locals.var_dmob_cv_dn3 = (0.5 * (locals.var_dmob_cv_dn3 + (((locals.var_dmob_cv_dn3 * assign10570_e9939) + (assign10570_e9936 * locals.var_dmob_cv_dn3)) / (2.0 * assign10570_e9947))));
        locals.var_dmob_cv_dn4 = (0.5 * (locals.var_dmob_cv_dn4 + (((locals.var_dmob_cv_dn4 * assign10570_e9939) + (assign10570_e9936 * locals.var_dmob_cv_dn4)) / (2.0 * assign10570_e9947))));
        locals.var_dmob_cv_dn5 = (0.5 * (locals.var_dmob_cv_dn5 + (((locals.var_dmob_cv_dn5 * assign10570_e9939) + (assign10570_e9936 * locals.var_dmob_cv_dn5)) / (2.0 * assign10570_e9947))));
        locals.var_dmob_cv_dn6 = (0.5 * (locals.var_dmob_cv_dn6 + (((locals.var_dmob_cv_dn6 * assign10570_e9939) + (assign10570_e9936 * locals.var_dmob_cv_dn6)) / (2.0 * assign10570_e9947))));
        locals.var_dmob_cv_dn7 = (0.5 * (locals.var_dmob_cv_dn7 + (((locals.var_dmob_cv_dn7 * assign10570_e9939) + (assign10570_e9936 * locals.var_dmob_cv_dn7)) / (2.0 * assign10570_e9947))));
        locals.var_dmob_cv_dn8 = (0.5 * (locals.var_dmob_cv_dn8 + (((locals.var_dmob_cv_dn8 * assign10570_e9939) + (assign10570_e9936 * locals.var_dmob_cv_dn8)) / (2.0 * assign10570_e9947))));
        locals.var_dmob_cv_rv = 0.0;

        let assign10580_e9952: f64 = (locals.var_dmob_cv / p.p11);
        locals.var_dmob_cv = assign10580_e9952;
        locals.var_dmob_cv_dn3 = (locals.var_dmob_cv_dn3 / p.p11);
        locals.var_dmob_cv_dn4 = (locals.var_dmob_cv_dn4 / p.p11);
        locals.var_dmob_cv_dn5 = (locals.var_dmob_cv_dn5 / p.p11);
        locals.var_dmob_cv_dn6 = (locals.var_dmob_cv_dn6 / p.p11);
        locals.var_dmob_cv_dn7 = (locals.var_dmob_cv_dn7 / p.p11);
        locals.var_dmob_cv_dn8 = (locals.var_dmob_cv_dn8 / p.p11);
        locals.var_dmob_cv_rv = 0.0;

        let assign10590_e9955: f64 = (2.0 * locals.var_vsat1_t);
        let assign10590_e9957: f64 = (assign10590_e9955 / locals.var_utotal);
        locals.var_esat1 = assign10590_e9957;
        locals.var_esat1_dn3 = (-((assign10590_e9955 * locals.var_utotal_dn3) / (locals.var_utotal * locals.var_utotal)));
        locals.var_esat1_dn4 = ((((2.0 * locals.var_vsat1_t_dn4) * locals.var_utotal) - (assign10590_e9955 * locals.var_utotal_dn4)) / (locals.var_utotal * locals.var_utotal));
        locals.var_esat1_dn5 = (-((assign10590_e9955 * locals.var_utotal_dn5) / (locals.var_utotal * locals.var_utotal)));
        locals.var_esat1_dn6 = (-((assign10590_e9955 * locals.var_utotal_dn6) / (locals.var_utotal * locals.var_utotal)));
        locals.var_esat1_dn7 = (-((assign10590_e9955 * locals.var_utotal_dn7) / (locals.var_utotal * locals.var_utotal)));
        locals.var_esat1_dn8 = (-((assign10590_e9955 * locals.var_utotal_dn8) / (locals.var_utotal * locals.var_utotal)));
        locals.var_esat1_rv = 0.0;

        let assign10600_e9960: f64 = (locals.var_esat1 * locals.var_leff);
        locals.var_esat1l = assign10600_e9960;
        locals.var_esat1l_dn3 = (locals.var_esat1_dn3 * locals.var_leff);
        locals.var_esat1l_dn4 = (locals.var_esat1_dn4 * locals.var_leff);
        locals.var_esat1l_dn5 = (locals.var_esat1_dn5 * locals.var_leff);
        locals.var_esat1l_dn6 = (locals.var_esat1_dn6 * locals.var_leff);
        locals.var_esat1l_dn7 = (locals.var_esat1_dn7 * locals.var_leff);
        locals.var_esat1l_dn8 = (locals.var_esat1_dn8 * locals.var_leff);
        locals.var_esat1l_rv = 0.0;

        let assign10610_e9964: f64 = (locals.var_vsatb_t * locals.var_vbgx);
        let assign10610_e9965: f64 = (0.8 + assign10610_e9964);
        locals.var_t0 = assign10610_e9965;
        locals.var_t0_dn3 = (locals.var_vsatb_t * locals.var_vbgx_dn3);
        locals.var_t0_dn4 = (locals.var_vsatb_t_dn4 * locals.var_vbgx);
        locals.var_t0_dn5 = (locals.var_vsatb_t * locals.var_vbgx_dn5);
        locals.var_t0_dn6 = (locals.var_vsatb_t * locals.var_vbgx_dn6);
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign10620_e9971: f64 = (locals.var_t0 * locals.var_t0);
        let assign10620_e9973: f64 = (assign10620_e9971 + 0.01);
        let assign10620_e9974: f64 = (assign10620_e9973).sqrt();
        let assign10620_e9975: f64 = (locals.var_t0 + assign10620_e9974);
        let assign10620_e9976: f64 = (0.5 * assign10620_e9975);
        let assign10620_e9977: f64 = (0.2 + assign10620_e9976);
        locals.var_xsat = assign10620_e9977;
        locals.var_xsat_dn3 = (0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign10620_e9974))));
        locals.var_xsat_dn4 = (0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign10620_e9974))));
        locals.var_xsat_dn5 = (0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign10620_e9974))));
        locals.var_xsat_dn6 = (0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign10620_e9974))));
        locals.var_xsat_dn7 = (0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign10620_e9974))));
        locals.var_xsat_dn8 = (0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign10620_e9974))));
        locals.var_xsat_rv = 0.0;

        let assign10630_e9980: f64 = (locals.var_dqi / locals.var_esat1l);
        let assign10630_e9982: f64 = (assign10630_e9980 * locals.var_xsat);
        locals.var_t0 = assign10630_e9982;
        locals.var_t0_dn3 = (((((locals.var_dqi_dn3 * locals.var_esat1l) - (locals.var_dqi * locals.var_esat1l_dn3)) / (locals.var_esat1l * locals.var_esat1l)) * locals.var_xsat) + (assign10630_e9980 * locals.var_xsat_dn3));
        locals.var_t0_dn4 = (((((locals.var_dqi_dn4 * locals.var_esat1l) - (locals.var_dqi * locals.var_esat1l_dn4)) / (locals.var_esat1l * locals.var_esat1l)) * locals.var_xsat) + (assign10630_e9980 * locals.var_xsat_dn4));
        locals.var_t0_dn5 = (((((locals.var_dqi_dn5 * locals.var_esat1l) - (locals.var_dqi * locals.var_esat1l_dn5)) / (locals.var_esat1l * locals.var_esat1l)) * locals.var_xsat) + (assign10630_e9980 * locals.var_xsat_dn5));
        locals.var_t0_dn6 = (((((locals.var_dqi_dn6 * locals.var_esat1l) - (locals.var_dqi * locals.var_esat1l_dn6)) / (locals.var_esat1l * locals.var_esat1l)) * locals.var_xsat) + (assign10630_e9980 * locals.var_xsat_dn6));
        locals.var_t0_dn7 = (((((locals.var_dqi_dn7 * locals.var_esat1l) - (locals.var_dqi * locals.var_esat1l_dn7)) / (locals.var_esat1l * locals.var_esat1l)) * locals.var_xsat) + (assign10630_e9980 * locals.var_xsat_dn7));
        locals.var_t0_dn8 = (((((locals.var_dqi_dn8 * locals.var_esat1l) - (locals.var_dqi * locals.var_esat1l_dn8)) / (locals.var_esat1l * locals.var_esat1l)) * locals.var_xsat) + (assign10630_e9980 * locals.var_xsat_dn8));
        locals.var_t0_rv = 0.0;

        let assign10640_e9987: f64 = (locals.var_t0 * locals.var_t0);
        let assign10640_e9988: f64 = (p.p109 + assign10640_e9987);
        let assign10640_e9989: f64 = (assign10640_e9988).sqrt();
        let assign10640_e9990: f64 = (1.0 + assign10640_e9989);
        let assign10640_e9993: f64 = (p.p109).sqrt();
        let assign10640_e9994: f64 = (1.0 + assign10640_e9993);
        let assign10640_e9995: f64 = (assign10640_e9990 / assign10640_e9994);
        locals.var_dvsat = assign10640_e9995;
        locals.var_dvsat_dn3 = ((((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign10640_e9989)) / assign10640_e9994);
        locals.var_dvsat_dn4 = ((((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign10640_e9989)) / assign10640_e9994);
        locals.var_dvsat_dn5 = ((((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign10640_e9989)) / assign10640_e9994);
        locals.var_dvsat_dn6 = ((((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign10640_e9989)) / assign10640_e9994);
        locals.var_dvsat_dn7 = ((((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign10640_e9989)) / assign10640_e9994);
        locals.var_dvsat_dn8 = ((((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign10640_e9989)) / assign10640_e9994);
        locals.var_dvsat_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_24(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign10650_e10001: f64 = (locals.var_ptwgb_i * locals.var_vbgxpos);
        let assign10650_e10002: f64 = (locals.var_ptwg_t - assign10650_e10001);
        let assign10650_e10005: f64 = (locals.var_ptwgb2_i * locals.var_vbgx);
        let assign10650_e10006: f64 = (assign10650_e10002 - assign10650_e10005);
        let assign10650_e10007: f64 = (0.5 * assign10650_e10006);
        let assign10650_e10009: f64 = (assign10650_e10007 * locals.var_qia);
        let assign10650_e10011: f64 = (assign10650_e10009 * locals.var_dqi);
        let assign10650_e10013: f64 = (assign10650_e10011 * locals.var_dqi);
        let assign10650_e10014: f64 = (locals.var_dvsat + assign10650_e10013);
        locals.var_dvsat = assign10650_e10014;
        locals.var_dvsat_dn3 = (locals.var_dvsat_dn3 + (((((((0.5 * ((-(locals.var_ptwgb_i * locals.var_vbgxpos_dn3)) - (locals.var_ptwgb2_i * locals.var_vbgx_dn3))) * locals.var_qia) + (assign10650_e10007 * locals.var_qia_dn3)) * locals.var_dqi) + (assign10650_e10009 * locals.var_dqi_dn3)) * locals.var_dqi) + (assign10650_e10011 * locals.var_dqi_dn3)));
        locals.var_dvsat_dn4 = (locals.var_dvsat_dn4 + (((((((0.5 * locals.var_ptwg_t_dn4) * locals.var_qia) + (assign10650_e10007 * locals.var_qia_dn4)) * locals.var_dqi) + (assign10650_e10009 * locals.var_dqi_dn4)) * locals.var_dqi) + (assign10650_e10011 * locals.var_dqi_dn4)));
        locals.var_dvsat_dn5 = (locals.var_dvsat_dn5 + (((((((0.5 * ((-(locals.var_ptwgb_i * locals.var_vbgxpos_dn5)) - (locals.var_ptwgb2_i * locals.var_vbgx_dn5))) * locals.var_qia) + (assign10650_e10007 * locals.var_qia_dn5)) * locals.var_dqi) + (assign10650_e10009 * locals.var_dqi_dn5)) * locals.var_dqi) + (assign10650_e10011 * locals.var_dqi_dn5)));
        locals.var_dvsat_dn6 = (locals.var_dvsat_dn6 + (((((((0.5 * ((-(locals.var_ptwgb_i * locals.var_vbgxpos_dn6)) - (locals.var_ptwgb2_i * locals.var_vbgx_dn6))) * locals.var_qia) + (assign10650_e10007 * locals.var_qia_dn6)) * locals.var_dqi) + (assign10650_e10009 * locals.var_dqi_dn6)) * locals.var_dqi) + (assign10650_e10011 * locals.var_dqi_dn6)));
        locals.var_dvsat_dn7 = (locals.var_dvsat_dn7 + (((((assign10650_e10007 * locals.var_qia_dn7) * locals.var_dqi) + (assign10650_e10009 * locals.var_dqi_dn7)) * locals.var_dqi) + (assign10650_e10011 * locals.var_dqi_dn7)));
        locals.var_dvsat_dn8 = (locals.var_dvsat_dn8 + (((((assign10650_e10007 * locals.var_qia_dn8) * locals.var_dqi) + (assign10650_e10009 * locals.var_dqi_dn8)) * locals.var_dqi) + (assign10650_e10011 * locals.var_dqi_dn8)));
        locals.var_dvsat_rv = 0.0;

        let assign10660_e10018: f64 = (locals.var_dvsat + 1.0);
        let assign10660_e10021: f64 = (locals.var_dvsat - 1.0);
        let assign10660_e10024: f64 = (locals.var_dvsat - 1.0);
        let assign10660_e10025: f64 = (assign10660_e10021 * assign10660_e10024);
        let assign10660_e10028: f64 = (0.25 * p.p134);
        let assign10660_e10030: f64 = (assign10660_e10028 * p.p134);
        let assign10660_e10031: f64 = (assign10660_e10025 + assign10660_e10030);
        let assign10660_e10032: f64 = (assign10660_e10031).sqrt();
        let assign10660_e10033: f64 = (assign10660_e10018 + assign10660_e10032);
        let assign10660_e10034: f64 = (0.5 * assign10660_e10033);
        locals.var_dvsat = assign10660_e10034;
        locals.var_dvsat_dn3 = (0.5 * (locals.var_dvsat_dn3 + (((locals.var_dvsat_dn3 * assign10660_e10024) + (assign10660_e10021 * locals.var_dvsat_dn3)) / (2.0 * assign10660_e10032))));
        locals.var_dvsat_dn4 = (0.5 * (locals.var_dvsat_dn4 + (((locals.var_dvsat_dn4 * assign10660_e10024) + (assign10660_e10021 * locals.var_dvsat_dn4)) / (2.0 * assign10660_e10032))));
        locals.var_dvsat_dn5 = (0.5 * (locals.var_dvsat_dn5 + (((locals.var_dvsat_dn5 * assign10660_e10024) + (assign10660_e10021 * locals.var_dvsat_dn5)) / (2.0 * assign10660_e10032))));
        locals.var_dvsat_dn6 = (0.5 * (locals.var_dvsat_dn6 + (((locals.var_dvsat_dn6 * assign10660_e10024) + (assign10660_e10021 * locals.var_dvsat_dn6)) / (2.0 * assign10660_e10032))));
        locals.var_dvsat_dn7 = (0.5 * (locals.var_dvsat_dn7 + (((locals.var_dvsat_dn7 * assign10660_e10024) + (assign10660_e10021 * locals.var_dvsat_dn7)) / (2.0 * assign10660_e10032))));
        locals.var_dvsat_dn8 = (0.5 * (locals.var_dvsat_dn8 + (((locals.var_dvsat_dn8 * assign10660_e10024) + (assign10660_e10021 * locals.var_dvsat_dn8)) / (2.0 * assign10660_e10032))));
        locals.var_dvsat_rv = 0.0;

        let assign10670_e10037: f64 = (2.0 * locals.var_vsatcv_t);
        let assign10670_e10039: f64 = (assign10670_e10037 * locals.var_dmob_cv);
        let assign10670_e10041: f64 = (assign10670_e10039 / locals.var_u0_t);
        locals.var_esatcv = assign10670_e10041;
        locals.var_esatcv_dn3 = ((assign10670_e10037 * locals.var_dmob_cv_dn3) / locals.var_u0_t);
        locals.var_esatcv_dn4 = ((((((2.0 * locals.var_vsatcv_t_dn4) * locals.var_dmob_cv) + (assign10670_e10037 * locals.var_dmob_cv_dn4)) * locals.var_u0_t) - (assign10670_e10039 * locals.var_u0_t_dn4)) / (locals.var_u0_t * locals.var_u0_t));
        locals.var_esatcv_dn5 = ((assign10670_e10037 * locals.var_dmob_cv_dn5) / locals.var_u0_t);
        locals.var_esatcv_dn6 = ((assign10670_e10037 * locals.var_dmob_cv_dn6) / locals.var_u0_t);
        locals.var_esatcv_dn7 = ((assign10670_e10037 * locals.var_dmob_cv_dn7) / locals.var_u0_t);
        locals.var_esatcv_dn8 = ((assign10670_e10037 * locals.var_dmob_cv_dn8) / locals.var_u0_t);
        locals.var_esatcv_rv = 0.0;

        let assign10680_e10044: f64 = (locals.var_esatcv * locals.var_leffcv);
        locals.var_esatcvl = assign10680_e10044;
        locals.var_esatcvl_dn3 = (locals.var_esatcv_dn3 * locals.var_leffcv);
        locals.var_esatcvl_dn4 = (locals.var_esatcv_dn4 * locals.var_leffcv);
        locals.var_esatcvl_dn5 = (locals.var_esatcv_dn5 * locals.var_leffcv);
        locals.var_esatcvl_dn6 = (locals.var_esatcv_dn6 * locals.var_leffcv);
        locals.var_esatcvl_dn7 = (locals.var_esatcv_dn7 * locals.var_leffcv);
        locals.var_esatcvl_dn8 = (locals.var_esatcv_dn8 * locals.var_leffcv);
        locals.var_esatcvl_rv = 0.0;

        let assign10690_e10047: f64 = if locals.var_pvag_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard104 = assign10690_e10047;
        locals.var_guard104_rv = 0.0;

        let (assign10700_e10057, assign10700_e10057_d_n3, assign10700_e10057_d_n4, assign10700_e10057_d_n5, assign10700_e10057_d_n6, assign10700_e10057_d_n7, assign10700_e10057_d_n8,) = {
    if (locals.var_guard104 != 0.0) {
        let assign10700_e10052: f64 = (locals.var_pvag_i * locals.var_qia);
        let assign10700_e10054: f64 = (assign10700_e10052 / locals.var_esatl);
        let assign10700_e10055: f64 = (1.0 + assign10700_e10054);
        (assign10700_e10055, ((((locals.var_pvag_i * locals.var_qia_dn3) * locals.var_esatl) - (assign10700_e10052 * locals.var_esatl_dn3)) / (locals.var_esatl * locals.var_esatl)), ((((locals.var_pvag_i * locals.var_qia_dn4) * locals.var_esatl) - (assign10700_e10052 * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl)), ((((locals.var_pvag_i * locals.var_qia_dn5) * locals.var_esatl) - (assign10700_e10052 * locals.var_esatl_dn5)) / (locals.var_esatl * locals.var_esatl)), ((((locals.var_pvag_i * locals.var_qia_dn6) * locals.var_esatl) - (assign10700_e10052 * locals.var_esatl_dn6)) / (locals.var_esatl * locals.var_esatl)), ((((locals.var_pvag_i * locals.var_qia_dn7) * locals.var_esatl) - (assign10700_e10052 * locals.var_esatl_dn7)) / (locals.var_esatl * locals.var_esatl)), ((((locals.var_pvag_i * locals.var_qia_dn8) * locals.var_esatl) - (assign10700_e10052 * locals.var_esatl_dn8)) / (locals.var_esatl * locals.var_esatl)),)
    } else {
        (locals.var_pvagfactor, locals.var_pvagfactor_dn3, locals.var_pvagfactor_dn4, locals.var_pvagfactor_dn5, locals.var_pvagfactor_dn6, locals.var_pvagfactor_dn7, locals.var_pvagfactor_dn8,)
    }
};
        locals.var_pvagfactor = assign10700_e10057;
        locals.var_pvagfactor_dn3 = assign10700_e10057_d_n3;
        locals.var_pvagfactor_dn4 = assign10700_e10057_d_n4;
        locals.var_pvagfactor_dn5 = assign10700_e10057_d_n5;
        locals.var_pvagfactor_dn6 = assign10700_e10057_d_n6;
        locals.var_pvagfactor_dn7 = assign10700_e10057_d_n7;
        locals.var_pvagfactor_dn8 = assign10700_e10057_d_n8;
        locals.var_pvagfactor_rv = 0.0;

        let (assign10710_e10070, assign10710_e10070_d_n3, assign10710_e10070_d_n4, assign10710_e10070_d_n5, assign10710_e10070_d_n6, assign10710_e10070_d_n7, assign10710_e10070_d_n8,) = {
    if (locals.var_guard104 == 0.0) {
        let assign10710_e10064: f64 = (locals.var_pvag_i * locals.var_qia);
        let assign10710_e10066: f64 = (assign10710_e10064 / locals.var_esatl);
        let assign10710_e10067: f64 = (1.0 - assign10710_e10066);
        let assign10710_e10068: f64 = (1.0 / assign10710_e10067);
        (assign10710_e10068, (-((-((((locals.var_pvag_i * locals.var_qia_dn3) * locals.var_esatl) - (assign10710_e10064 * locals.var_esatl_dn3)) / (locals.var_esatl * locals.var_esatl))) / (assign10710_e10067 * assign10710_e10067))), (-((-((((locals.var_pvag_i * locals.var_qia_dn4) * locals.var_esatl) - (assign10710_e10064 * locals.var_esatl_dn4)) / (locals.var_esatl * locals.var_esatl))) / (assign10710_e10067 * assign10710_e10067))), (-((-((((locals.var_pvag_i * locals.var_qia_dn5) * locals.var_esatl) - (assign10710_e10064 * locals.var_esatl_dn5)) / (locals.var_esatl * locals.var_esatl))) / (assign10710_e10067 * assign10710_e10067))), (-((-((((locals.var_pvag_i * locals.var_qia_dn6) * locals.var_esatl) - (assign10710_e10064 * locals.var_esatl_dn6)) / (locals.var_esatl * locals.var_esatl))) / (assign10710_e10067 * assign10710_e10067))), (-((-((((locals.var_pvag_i * locals.var_qia_dn7) * locals.var_esatl) - (assign10710_e10064 * locals.var_esatl_dn7)) / (locals.var_esatl * locals.var_esatl))) / (assign10710_e10067 * assign10710_e10067))), (-((-((((locals.var_pvag_i * locals.var_qia_dn8) * locals.var_esatl) - (assign10710_e10064 * locals.var_esatl_dn8)) / (locals.var_esatl * locals.var_esatl))) / (assign10710_e10067 * assign10710_e10067))),)
    } else {
        (locals.var_pvagfactor, locals.var_pvagfactor_dn3, locals.var_pvagfactor_dn4, locals.var_pvagfactor_dn5, locals.var_pvagfactor_dn6, locals.var_pvagfactor_dn7, locals.var_pvagfactor_dn8,)
    }
};
        locals.var_pvagfactor = assign10710_e10070;
        locals.var_pvagfactor_dn3 = assign10710_e10070_d_n3;
        locals.var_pvagfactor_dn4 = assign10710_e10070_d_n4;
        locals.var_pvagfactor_dn5 = assign10710_e10070_d_n5;
        locals.var_pvagfactor_dn6 = assign10710_e10070_d_n6;
        locals.var_pvagfactor_dn7 = assign10710_e10070_d_n7;
        locals.var_pvagfactor_dn8 = assign10710_e10070_d_n8;
        locals.var_pvagfactor_rv = 0.0;

        let assign10720_e10073: f64 = (locals.var_vds - locals.var_vdseff);
        locals.var_diffvds = assign10720_e10073;
        locals.var_diffvds_dn3 = (-locals.var_vdseff_dn3);
        locals.var_diffvds_dn4 = (-locals.var_vdseff_dn4);
        locals.var_diffvds_dn5 = (locals.var_vds_dn5 - locals.var_vdseff_dn5);
        locals.var_diffvds_dn6 = (locals.var_vds_dn6 - locals.var_vdseff_dn6);
        locals.var_diffvds_dn7 = (-locals.var_vdseff_dn7);
        locals.var_diffvds_dn8 = (-locals.var_vdseff_dn8);
        locals.var_diffvds_rv = 0.0;

        let assign10730_e10077: f64 = (2.0 * locals.var_vtm);
        let assign10730_e10078: f64 = (locals.var_qia + assign10730_e10077);
        locals.var_vgst2vtm = assign10730_e10078;
        locals.var_vgst2vtm_dn3 = locals.var_qia_dn3;
        locals.var_vgst2vtm_dn4 = (locals.var_qia_dn4 + (2.0 * locals.var_vtm_dn4));
        locals.var_vgst2vtm_dn5 = locals.var_qia_dn5;
        locals.var_vgst2vtm_dn6 = locals.var_qia_dn6;
        locals.var_vgst2vtm_dn7 = locals.var_qia_dn7;
        locals.var_vgst2vtm_dn8 = locals.var_qia_dn8;
        locals.var_vgst2vtm_rv = 0.0;

        let assign10740_e10081: f64 = if locals.var_diblfactor > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard105 = assign10740_e10081;
        locals.var_guard105_rv = 0.0;

        let (assign10750_e10085, assign10750_e10085_d_n3, assign10750_e10085_d_n4, assign10750_e10085_d_n5, assign10750_e10085_d_n6, assign10750_e10085_d_n7, assign10750_e10085_d_n8,) = {
    if (locals.var_guard105 != 0.0) {
        (locals.var_vgst2vtm, locals.var_vgst2vtm_dn3, locals.var_vgst2vtm_dn4, locals.var_vgst2vtm_dn5, locals.var_vgst2vtm_dn6, locals.var_vgst2vtm_dn7, locals.var_vgst2vtm_dn8,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign10750_e10085;
        locals.var_t1_dn3 = assign10750_e10085_d_n3;
        locals.var_t1_dn4 = assign10750_e10085_d_n4;
        locals.var_t1_dn5 = assign10750_e10085_d_n5;
        locals.var_t1_dn6 = assign10750_e10085_d_n6;
        locals.var_t1_dn7 = assign10750_e10085_d_n7;
        locals.var_t1_dn8 = assign10750_e10085_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign10760_e10093, assign10760_e10093_d_n3, assign10760_e10093_d_n4, assign10760_e10093_d_n5, assign10760_e10093_d_n6, assign10760_e10093_d_n7, assign10760_e10093_d_n8,) = {
    if (locals.var_guard105 != 0.0) {
        let assign10760_e10090: f64 = (locals.var_vdsat + locals.var_t1);
        let assign10760_e10091: f64 = (locals.var_t1 / assign10760_e10090);
        (assign10760_e10091, (((locals.var_t1_dn3 * assign10760_e10090) - (locals.var_t1 * (locals.var_vdsat_dn3 + locals.var_t1_dn3))) / (assign10760_e10090 * assign10760_e10090)), (((locals.var_t1_dn4 * assign10760_e10090) - (locals.var_t1 * (locals.var_vdsat_dn4 + locals.var_t1_dn4))) / (assign10760_e10090 * assign10760_e10090)), (((locals.var_t1_dn5 * assign10760_e10090) - (locals.var_t1 * (locals.var_vdsat_dn5 + locals.var_t1_dn5))) / (assign10760_e10090 * assign10760_e10090)), (((locals.var_t1_dn6 * assign10760_e10090) - (locals.var_t1 * (locals.var_vdsat_dn6 + locals.var_t1_dn6))) / (assign10760_e10090 * assign10760_e10090)), (((locals.var_t1_dn7 * assign10760_e10090) - (locals.var_t1 * (locals.var_vdsat_dn7 + locals.var_t1_dn7))) / (assign10760_e10090 * assign10760_e10090)), (((locals.var_t1_dn8 * assign10760_e10090) - (locals.var_t1 * (locals.var_vdsat_dn8 + locals.var_t1_dn8))) / (assign10760_e10090 * assign10760_e10090)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8,)
    }
};
        locals.var_t3 = assign10760_e10093;
        locals.var_t3_dn3 = assign10760_e10093_d_n3;
        locals.var_t3_dn4 = assign10760_e10093_d_n4;
        locals.var_t3_dn5 = assign10760_e10093_d_n5;
        locals.var_t3_dn6 = assign10760_e10093_d_n6;
        locals.var_t3_dn7 = assign10760_e10093_d_n7;
        locals.var_t3_dn8 = assign10760_e10093_d_n8;
        locals.var_t3_rv = 0.0;

        let (assign10770_e10103, assign10770_e10103_d_n3, assign10770_e10103_d_n4, assign10770_e10103_d_n5, assign10770_e10103_d_n6, assign10770_e10103_d_n7, assign10770_e10103_d_n8,) = {
    if (locals.var_guard105 != 0.0) {
        let assign10770_e10097: f64 = (locals.var_t1 / locals.var_diblfactor);
        let assign10770_e10099: f64 = (assign10770_e10097 * locals.var_t3);
        let assign10770_e10101: f64 = (assign10770_e10099 * locals.var_pvagfactor);
        (assign10770_e10101, (((((((locals.var_t1_dn3 * locals.var_diblfactor) - (locals.var_t1 * locals.var_diblfactor_dn3)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign10770_e10097 * locals.var_t3_dn3)) * locals.var_pvagfactor) + (assign10770_e10099 * locals.var_pvagfactor_dn3)), (((((((locals.var_t1_dn4 * locals.var_diblfactor) - (locals.var_t1 * locals.var_diblfactor_dn4)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign10770_e10097 * locals.var_t3_dn4)) * locals.var_pvagfactor) + (assign10770_e10099 * locals.var_pvagfactor_dn4)), (((((((locals.var_t1_dn5 * locals.var_diblfactor) - (locals.var_t1 * locals.var_diblfactor_dn5)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign10770_e10097 * locals.var_t3_dn5)) * locals.var_pvagfactor) + (assign10770_e10099 * locals.var_pvagfactor_dn5)), (((((((locals.var_t1_dn6 * locals.var_diblfactor) - (locals.var_t1 * locals.var_diblfactor_dn6)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign10770_e10097 * locals.var_t3_dn6)) * locals.var_pvagfactor) + (assign10770_e10099 * locals.var_pvagfactor_dn6)), (((((((locals.var_t1_dn7 * locals.var_diblfactor) - (locals.var_t1 * locals.var_diblfactor_dn7)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign10770_e10097 * locals.var_t3_dn7)) * locals.var_pvagfactor) + (assign10770_e10099 * locals.var_pvagfactor_dn7)), (((((((locals.var_t1_dn8 * locals.var_diblfactor) - (locals.var_t1 * locals.var_diblfactor_dn8)) / (locals.var_diblfactor * locals.var_diblfactor)) * locals.var_t3) + (assign10770_e10097 * locals.var_t3_dn8)) * locals.var_pvagfactor) + (assign10770_e10099 * locals.var_pvagfactor_dn8)),)
    } else {
        (locals.var_vadibl, locals.var_vadibl_dn3, locals.var_vadibl_dn4, locals.var_vadibl_dn5, locals.var_vadibl_dn6, locals.var_vadibl_dn7, locals.var_vadibl_dn8,)
    }
};
        locals.var_vadibl = assign10770_e10103;
        locals.var_vadibl_dn3 = assign10770_e10103_d_n3;
        locals.var_vadibl_dn4 = assign10770_e10103_d_n4;
        locals.var_vadibl_dn5 = assign10770_e10103_d_n5;
        locals.var_vadibl_dn6 = assign10770_e10103_d_n6;
        locals.var_vadibl_dn7 = assign10770_e10103_d_n7;
        locals.var_vadibl_dn8 = assign10770_e10103_d_n8;
        locals.var_vadibl_rv = 0.0;

        let (assign10780_e10111, assign10780_e10111_d_n3, assign10780_e10111_d_n4, assign10780_e10111_d_n5, assign10780_e10111_d_n6, assign10780_e10111_d_n7, assign10780_e10111_d_n8,) = {
    if (locals.var_guard105 != 0.0) {
        let assign10780_e10108: f64 = (locals.var_diffvds / locals.var_vadibl);
        let assign10780_e10109: f64 = (1.0 + assign10780_e10108);
        (assign10780_e10109, (((locals.var_diffvds_dn3 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn3)) / (locals.var_vadibl * locals.var_vadibl)), (((locals.var_diffvds_dn4 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn4)) / (locals.var_vadibl * locals.var_vadibl)), (((locals.var_diffvds_dn5 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn5)) / (locals.var_vadibl * locals.var_vadibl)), (((locals.var_diffvds_dn6 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn6)) / (locals.var_vadibl * locals.var_vadibl)), (((locals.var_diffvds_dn7 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn7)) / (locals.var_vadibl * locals.var_vadibl)), (((locals.var_diffvds_dn8 * locals.var_vadibl) - (locals.var_diffvds * locals.var_vadibl_dn8)) / (locals.var_vadibl * locals.var_vadibl)),)
    } else {
        (locals.var_moc, locals.var_moc_dn3, locals.var_moc_dn4, locals.var_moc_dn5, locals.var_moc_dn6, locals.var_moc_dn7, locals.var_moc_dn8,)
    }
};
        locals.var_moc = assign10780_e10111;
        locals.var_moc_dn3 = assign10780_e10111_d_n3;
        locals.var_moc_dn4 = assign10780_e10111_d_n4;
        locals.var_moc_dn5 = assign10780_e10111_d_n5;
        locals.var_moc_dn6 = assign10780_e10111_d_n6;
        locals.var_moc_dn7 = assign10780_e10111_d_n7;
        locals.var_moc_dn8 = assign10780_e10111_d_n8;
        locals.var_moc_rv = 0.0;

        let (assign10790_e10116, assign10790_e10116_d_n3, assign10790_e10116_d_n4, assign10790_e10116_d_n5, assign10790_e10116_d_n6, assign10790_e10116_d_n7, assign10790_e10116_d_n8,) = {
    if (locals.var_guard105 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_moc, locals.var_moc_dn3, locals.var_moc_dn4, locals.var_moc_dn5, locals.var_moc_dn6, locals.var_moc_dn7, locals.var_moc_dn8,)
    }
};
        locals.var_moc = assign10790_e10116;
        locals.var_moc_dn3 = assign10790_e10116_d_n3;
        locals.var_moc_dn4 = assign10790_e10116_d_n4;
        locals.var_moc_dn5 = assign10790_e10116_d_n5;
        locals.var_moc_dn6 = assign10790_e10116_d_n6;
        locals.var_moc_dn7 = assign10790_e10116_d_n7;
        locals.var_moc_dn8 = assign10790_e10116_d_n8;
        locals.var_moc_rv = 0.0;

        let assign10800_e10119: f64 = if locals.var_pclm_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard106 = assign10800_e10119;
        locals.var_guard106_rv = 0.0;

        let assign10810_e10122: f64 = if p.p213 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard107 = assign10810_e10122;
        locals.var_guard107_rv = 0.0;

        let (assign10820_e10136, assign10820_e10136_d_n3, assign10820_e10136_d_n4, assign10820_e10136_d_n5, assign10820_e10136_d_n6, assign10820_e10136_d_n7, assign10820_e10136_d_n8,) = {
    if ((locals.var_guard106 != 0.0) && (locals.var_guard107 != 0.0)) {
        let assign10820_e10129: f64 = (1.0 / locals.var_pclm_i);
        let assign10820_e10132: f64 = (p.p213 * locals.var_qia);
        let assign10820_e10133: f64 = (assign10820_e10129 - assign10820_e10132);
        let assign10820_e10134: f64 = (1.0 / assign10820_e10133);
        (assign10820_e10134, (-((-(p.p213 * locals.var_qia_dn3)) / (assign10820_e10133 * assign10820_e10133))), (-((-(p.p213 * locals.var_qia_dn4)) / (assign10820_e10133 * assign10820_e10133))), (-((-(p.p213 * locals.var_qia_dn5)) / (assign10820_e10133 * assign10820_e10133))), (-((-(p.p213 * locals.var_qia_dn6)) / (assign10820_e10133 * assign10820_e10133))), (-((-(p.p213 * locals.var_qia_dn7)) / (assign10820_e10133 * assign10820_e10133))), (-((-(p.p213 * locals.var_qia_dn8)) / (assign10820_e10133 * assign10820_e10133))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign10820_e10136;
        locals.var_t1_dn3 = assign10820_e10136_d_n3;
        locals.var_t1_dn4 = assign10820_e10136_d_n4;
        locals.var_t1_dn5 = assign10820_e10136_d_n5;
        locals.var_t1_dn6 = assign10820_e10136_d_n6;
        locals.var_t1_dn7 = assign10820_e10136_d_n7;
        locals.var_t1_dn8 = assign10820_e10136_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign10830_e10149, assign10830_e10149_d_n3, assign10830_e10149_d_n4, assign10830_e10149_d_n5, assign10830_e10149_d_n6, assign10830_e10149_d_n7, assign10830_e10149_d_n8,) = {
    if ((locals.var_guard106 != 0.0) && (locals.var_guard107 == 0.0)) {
        let assign10830_e10145: f64 = (p.p213 * locals.var_qia);
        let assign10830_e10146: f64 = (1.0 + assign10830_e10145);
        let assign10830_e10147: f64 = (locals.var_pclm_i * assign10830_e10146);
        (assign10830_e10147, (locals.var_pclm_i * (p.p213 * locals.var_qia_dn3)), (locals.var_pclm_i * (p.p213 * locals.var_qia_dn4)), (locals.var_pclm_i * (p.p213 * locals.var_qia_dn5)), (locals.var_pclm_i * (p.p213 * locals.var_qia_dn6)), (locals.var_pclm_i * (p.p213 * locals.var_qia_dn7)), (locals.var_pclm_i * (p.p213 * locals.var_qia_dn8)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign10830_e10149;
        locals.var_t1_dn3 = assign10830_e10149_d_n3;
        locals.var_t1_dn4 = assign10830_e10149_d_n4;
        locals.var_t1_dn5 = assign10830_e10149_d_n5;
        locals.var_t1_dn6 = assign10830_e10149_d_n6;
        locals.var_t1_dn7 = assign10830_e10149_d_n7;
        locals.var_t1_dn8 = assign10830_e10149_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign10840_e10168, assign10840_e10168_d_n3, assign10840_e10168_d_n4, assign10840_e10168_d_n5, assign10840_e10168_d_n6, assign10840_e10168_d_n7, assign10840_e10168_d_n8,) = {
    if (locals.var_guard106 != 0.0) {
        let assign10840_e10156: f64 = (locals.var_diffvds / locals.var_t1);
        let assign10840_e10159: f64 = (locals.var_vdsat + locals.var_esatl);
        let assign10840_e10160: f64 = (assign10840_e10156 / assign10840_e10159);
        let assign10840_e10161: f64 = (1.0 + assign10840_e10160);
        let assign10840_e10163: f64 = (assign10840_e10161).max(1e-38);
        let assign10840_e10164: f64 = (assign10840_e10163).ln();
        let assign10840_e10165: f64 = (locals.var_t1 * assign10840_e10164);
        let assign10840_e10166: f64 = (1.0 + assign10840_e10165);
        (assign10840_e10166, ((locals.var_t1_dn3 * assign10840_e10164) + (locals.var_t1 * (if assign10840_e10161 >= 1e-38 { ((((((locals.var_diffvds_dn3 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)) * assign10840_e10159) - (assign10840_e10156 * (locals.var_vdsat_dn3 + locals.var_esatl_dn3))) / (assign10840_e10159 * assign10840_e10159)) } else { 0.0 } / assign10840_e10163))), ((locals.var_t1_dn4 * assign10840_e10164) + (locals.var_t1 * (if assign10840_e10161 >= 1e-38 { ((((((locals.var_diffvds_dn4 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)) * assign10840_e10159) - (assign10840_e10156 * (locals.var_vdsat_dn4 + locals.var_esatl_dn4))) / (assign10840_e10159 * assign10840_e10159)) } else { 0.0 } / assign10840_e10163))), ((locals.var_t1_dn5 * assign10840_e10164) + (locals.var_t1 * (if assign10840_e10161 >= 1e-38 { ((((((locals.var_diffvds_dn5 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)) * assign10840_e10159) - (assign10840_e10156 * (locals.var_vdsat_dn5 + locals.var_esatl_dn5))) / (assign10840_e10159 * assign10840_e10159)) } else { 0.0 } / assign10840_e10163))), ((locals.var_t1_dn6 * assign10840_e10164) + (locals.var_t1 * (if assign10840_e10161 >= 1e-38 { ((((((locals.var_diffvds_dn6 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)) * assign10840_e10159) - (assign10840_e10156 * (locals.var_vdsat_dn6 + locals.var_esatl_dn6))) / (assign10840_e10159 * assign10840_e10159)) } else { 0.0 } / assign10840_e10163))), ((locals.var_t1_dn7 * assign10840_e10164) + (locals.var_t1 * (if assign10840_e10161 >= 1e-38 { ((((((locals.var_diffvds_dn7 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)) * assign10840_e10159) - (assign10840_e10156 * (locals.var_vdsat_dn7 + locals.var_esatl_dn7))) / (assign10840_e10159 * assign10840_e10159)) } else { 0.0 } / assign10840_e10163))), ((locals.var_t1_dn8 * assign10840_e10164) + (locals.var_t1 * (if assign10840_e10161 >= 1e-38 { ((((((locals.var_diffvds_dn8 * locals.var_t1) - (locals.var_diffvds * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)) * assign10840_e10159) - (assign10840_e10156 * (locals.var_vdsat_dn8 + locals.var_esatl_dn8))) / (assign10840_e10159 * assign10840_e10159)) } else { 0.0 } / assign10840_e10163))),)
    } else {
        (locals.var_mclm, locals.var_mclm_dn3, locals.var_mclm_dn4, locals.var_mclm_dn5, locals.var_mclm_dn6, locals.var_mclm_dn7, locals.var_mclm_dn8,)
    }
};
        locals.var_mclm = assign10840_e10168;
        locals.var_mclm_dn3 = assign10840_e10168_d_n3;
        locals.var_mclm_dn4 = assign10840_e10168_d_n4;
        locals.var_mclm_dn5 = assign10840_e10168_d_n5;
        locals.var_mclm_dn6 = assign10840_e10168_d_n6;
        locals.var_mclm_dn7 = assign10840_e10168_d_n7;
        locals.var_mclm_dn8 = assign10840_e10168_d_n8;
        locals.var_mclm_rv = 0.0;

        let (assign10850_e10173, assign10850_e10173_d_n3, assign10850_e10173_d_n4, assign10850_e10173_d_n5, assign10850_e10173_d_n6, assign10850_e10173_d_n7, assign10850_e10173_d_n8,) = {
    if (locals.var_guard106 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mclm, locals.var_mclm_dn3, locals.var_mclm_dn4, locals.var_mclm_dn5, locals.var_mclm_dn6, locals.var_mclm_dn7, locals.var_mclm_dn8,)
    }
};
        locals.var_mclm = assign10850_e10173;
        locals.var_mclm_dn3 = assign10850_e10173_d_n3;
        locals.var_mclm_dn4 = assign10850_e10173_d_n4;
        locals.var_mclm_dn5 = assign10850_e10173_d_n5;
        locals.var_mclm_dn6 = assign10850_e10173_d_n6;
        locals.var_mclm_dn7 = assign10850_e10173_d_n7;
        locals.var_mclm_dn8 = assign10850_e10173_d_n8;
        locals.var_mclm_rv = 0.0;

        let assign10860_e10176: f64 = (locals.var_moc * locals.var_mclm);
        locals.var_moc = assign10860_e10176;
        locals.var_moc_dn3 = ((locals.var_moc_dn3 * locals.var_mclm) + (locals.var_moc * locals.var_mclm_dn3));
        locals.var_moc_dn4 = ((locals.var_moc_dn4 * locals.var_mclm) + (locals.var_moc * locals.var_mclm_dn4));
        locals.var_moc_dn5 = ((locals.var_moc_dn5 * locals.var_mclm) + (locals.var_moc * locals.var_mclm_dn5));
        locals.var_moc_dn6 = ((locals.var_moc_dn6 * locals.var_mclm) + (locals.var_moc * locals.var_mclm_dn6));
        locals.var_moc_dn7 = ((locals.var_moc_dn7 * locals.var_mclm) + (locals.var_moc * locals.var_mclm_dn7));
        locals.var_moc_dn8 = ((locals.var_moc_dn8 * locals.var_mclm) + (locals.var_moc * locals.var_mclm_dn8));
        locals.var_moc_rv = 0.0;

        let assign10870_e10179: f64 = if locals.var_pclmcv_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard108 = assign10870_e10179;
        locals.var_guard108_rv = 0.0;

        let (assign10880_e10200, assign10880_e10200_d_n3, assign10880_e10200_d_n4, assign10880_e10200_d_n5, assign10880_e10200_d_n6, assign10880_e10200_d_n7, assign10880_e10200_d_n8,) = {
    if (locals.var_guard108 != 0.0) {
        let assign10880_e10186: f64 = (locals.var_vds - locals.var_vdseff);
        let assign10880_e10188: f64 = (assign10880_e10186 / locals.var_pclmcv_i);
        let assign10880_e10191: f64 = (locals.var_vdsat + locals.var_esatcvl);
        let assign10880_e10192: f64 = (assign10880_e10188 / assign10880_e10191);
        let assign10880_e10193: f64 = (1.0 + assign10880_e10192);
        let assign10880_e10195: f64 = (assign10880_e10193).max(1e-38);
        let assign10880_e10196: f64 = (assign10880_e10195).ln();
        let assign10880_e10197: f64 = (locals.var_pclmcv_i * assign10880_e10196);
        let assign10880_e10198: f64 = (1.0 + assign10880_e10197);
        (assign10880_e10198, (locals.var_pclmcv_i * (if assign10880_e10193 >= 1e-38 { (((((-locals.var_vdseff_dn3) / locals.var_pclmcv_i) * assign10880_e10191) - (assign10880_e10188 * (locals.var_vdsat_dn3 + locals.var_esatcvl_dn3))) / (assign10880_e10191 * assign10880_e10191)) } else { 0.0 } / assign10880_e10195)), (locals.var_pclmcv_i * (if assign10880_e10193 >= 1e-38 { (((((-locals.var_vdseff_dn4) / locals.var_pclmcv_i) * assign10880_e10191) - (assign10880_e10188 * (locals.var_vdsat_dn4 + locals.var_esatcvl_dn4))) / (assign10880_e10191 * assign10880_e10191)) } else { 0.0 } / assign10880_e10195)), (locals.var_pclmcv_i * (if assign10880_e10193 >= 1e-38 { (((((locals.var_vds_dn5 - locals.var_vdseff_dn5) / locals.var_pclmcv_i) * assign10880_e10191) - (assign10880_e10188 * (locals.var_vdsat_dn5 + locals.var_esatcvl_dn5))) / (assign10880_e10191 * assign10880_e10191)) } else { 0.0 } / assign10880_e10195)), (locals.var_pclmcv_i * (if assign10880_e10193 >= 1e-38 { (((((locals.var_vds_dn6 - locals.var_vdseff_dn6) / locals.var_pclmcv_i) * assign10880_e10191) - (assign10880_e10188 * (locals.var_vdsat_dn6 + locals.var_esatcvl_dn6))) / (assign10880_e10191 * assign10880_e10191)) } else { 0.0 } / assign10880_e10195)), (locals.var_pclmcv_i * (if assign10880_e10193 >= 1e-38 { (((((-locals.var_vdseff_dn7) / locals.var_pclmcv_i) * assign10880_e10191) - (assign10880_e10188 * (locals.var_vdsat_dn7 + locals.var_esatcvl_dn7))) / (assign10880_e10191 * assign10880_e10191)) } else { 0.0 } / assign10880_e10195)), (locals.var_pclmcv_i * (if assign10880_e10193 >= 1e-38 { (((((-locals.var_vdseff_dn8) / locals.var_pclmcv_i) * assign10880_e10191) - (assign10880_e10188 * (locals.var_vdsat_dn8 + locals.var_esatcvl_dn8))) / (assign10880_e10191 * assign10880_e10191)) } else { 0.0 } / assign10880_e10195)),)
    } else {
        (locals.var_mclmcv, locals.var_mclmcv_dn3, locals.var_mclmcv_dn4, locals.var_mclmcv_dn5, locals.var_mclmcv_dn6, locals.var_mclmcv_dn7, locals.var_mclmcv_dn8,)
    }
};
        locals.var_mclmcv = assign10880_e10200;
        locals.var_mclmcv_dn3 = assign10880_e10200_d_n3;
        locals.var_mclmcv_dn4 = assign10880_e10200_d_n4;
        locals.var_mclmcv_dn5 = assign10880_e10200_d_n5;
        locals.var_mclmcv_dn6 = assign10880_e10200_d_n6;
        locals.var_mclmcv_dn7 = assign10880_e10200_d_n7;
        locals.var_mclmcv_dn8 = assign10880_e10200_d_n8;
        locals.var_mclmcv_rv = 0.0;

        let (assign10890_e10205, assign10890_e10205_d_n3, assign10890_e10205_d_n4, assign10890_e10205_d_n5, assign10890_e10205_d_n6, assign10890_e10205_d_n7, assign10890_e10205_d_n8,) = {
    if (locals.var_guard108 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mclmcv, locals.var_mclmcv_dn3, locals.var_mclmcv_dn4, locals.var_mclmcv_dn5, locals.var_mclmcv_dn6, locals.var_mclmcv_dn7, locals.var_mclmcv_dn8,)
    }
};
        locals.var_mclmcv = assign10890_e10205;
        locals.var_mclmcv_dn3 = assign10890_e10205_d_n3;
        locals.var_mclmcv_dn4 = assign10890_e10205_d_n4;
        locals.var_mclmcv_dn5 = assign10890_e10205_d_n5;
        locals.var_mclmcv_dn6 = assign10890_e10205_d_n6;
        locals.var_mclmcv_dn7 = assign10890_e10205_d_n7;
        locals.var_mclmcv_dn8 = assign10890_e10205_d_n8;
        locals.var_mclmcv_rv = 0.0;

        let assign10900_e10208: f64 = if locals.var_k0_t != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard109 = assign10900_e10208;
        locals.var_guard109_rv = 0.0;

        let (assign10910_e10228, assign10910_e10228_d_n3, assign10910_e10228_d_n4, assign10910_e10228_d_n5, assign10910_e10228_d_n6, assign10910_e10228_d_n7, assign10910_e10228_d_n8,) = {
    if (locals.var_guard109 != 0.0) {
        let assign10910_e10215: f64 = (locals.var_k0sisat_t * locals.var_dqi);
        let assign10910_e10217: f64 = (assign10910_e10215 * locals.var_dqi);
        let assign10910_e10218: f64 = (locals.var_k0si_t + assign10910_e10217);
        let assign10910_e10219: f64 = (0.0_f64).max(assign10910_e10218);
        let assign10910_e10221: f64 = (assign10910_e10219 * locals.var_qia);
        let assign10910_e10224: f64 = (2.0 * locals.var_nvtm);
        let assign10910_e10225: f64 = (assign10910_e10221 + assign10910_e10224);
        let assign10910_e10226: f64 = (locals.var_k0_t / assign10910_e10225);
        (assign10910_e10226, (-((locals.var_k0_t * (((if 0.0 >= assign10910_e10218 { 0.0 } else { (((locals.var_k0sisat_t * locals.var_dqi_dn3) * locals.var_dqi) + (assign10910_e10215 * locals.var_dqi_dn3)) } * locals.var_qia) + (assign10910_e10219 * locals.var_qia_dn3)) + (2.0 * locals.var_nvtm_dn3))) / (assign10910_e10225 * assign10910_e10225))), (((locals.var_k0_t_dn4 * assign10910_e10225) - (locals.var_k0_t * (((if 0.0 >= assign10910_e10218 { 0.0 } else { (locals.var_k0si_t_dn4 + ((((locals.var_k0sisat_t_dn4 * locals.var_dqi) + (locals.var_k0sisat_t * locals.var_dqi_dn4)) * locals.var_dqi) + (assign10910_e10215 * locals.var_dqi_dn4))) } * locals.var_qia) + (assign10910_e10219 * locals.var_qia_dn4)) + (2.0 * locals.var_nvtm_dn4)))) / (assign10910_e10225 * assign10910_e10225)), (-((locals.var_k0_t * (((if 0.0 >= assign10910_e10218 { 0.0 } else { (((locals.var_k0sisat_t * locals.var_dqi_dn5) * locals.var_dqi) + (assign10910_e10215 * locals.var_dqi_dn5)) } * locals.var_qia) + (assign10910_e10219 * locals.var_qia_dn5)) + (2.0 * locals.var_nvtm_dn5))) / (assign10910_e10225 * assign10910_e10225))), (-((locals.var_k0_t * (((if 0.0 >= assign10910_e10218 { 0.0 } else { (((locals.var_k0sisat_t * locals.var_dqi_dn6) * locals.var_dqi) + (assign10910_e10215 * locals.var_dqi_dn6)) } * locals.var_qia) + (assign10910_e10219 * locals.var_qia_dn6)) + (2.0 * locals.var_nvtm_dn6))) / (assign10910_e10225 * assign10910_e10225))), (-((locals.var_k0_t * (((if 0.0 >= assign10910_e10218 { 0.0 } else { (((locals.var_k0sisat_t * locals.var_dqi_dn7) * locals.var_dqi) + (assign10910_e10215 * locals.var_dqi_dn7)) } * locals.var_qia) + (assign10910_e10219 * locals.var_qia_dn7)) + (2.0 * locals.var_nvtm_dn7))) / (assign10910_e10225 * assign10910_e10225))), (-((locals.var_k0_t * (((if 0.0 >= assign10910_e10218 { 0.0 } else { (((locals.var_k0sisat_t * locals.var_dqi_dn8) * locals.var_dqi) + (assign10910_e10215 * locals.var_dqi_dn8)) } * locals.var_qia) + (assign10910_e10219 * locals.var_qia_dn8)) + (2.0 * locals.var_nvtm_dn8))) / (assign10910_e10225 * assign10910_e10225))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign10910_e10228;
        locals.var_t1_dn3 = assign10910_e10228_d_n3;
        locals.var_t1_dn4 = assign10910_e10228_d_n4;
        locals.var_t1_dn5 = assign10910_e10228_d_n5;
        locals.var_t1_dn6 = assign10910_e10228_d_n6;
        locals.var_t1_dn7 = assign10910_e10228_d_n7;
        locals.var_t1_dn8 = assign10910_e10228_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign10920_e10234, assign10920_e10234_d_n3, assign10920_e10234_d_n4, assign10920_e10234_d_n5, assign10920_e10234_d_n6, assign10920_e10234_d_n7, assign10920_e10234_d_n8,) = {
    if (locals.var_guard109 != 0.0) {
        let assign10920_e10231: f64 = (-locals.var_t1);
        let assign10920_e10232: f64 = { let limited_exp_arg = assign10920_e10231; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign10920_e10232, ({ let limited_exp_arg = assign10920_e10231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn3)), ({ let limited_exp_arg = assign10920_e10231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn4)), ({ let limited_exp_arg = assign10920_e10231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn5)), ({ let limited_exp_arg = assign10920_e10231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn6)), ({ let limited_exp_arg = assign10920_e10231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn7)), ({ let limited_exp_arg = assign10920_e10231; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn8)),)
    } else {
        (locals.var_mnud, locals.var_mnud_dn3, locals.var_mnud_dn4, locals.var_mnud_dn5, locals.var_mnud_dn6, locals.var_mnud_dn7, locals.var_mnud_dn8,)
    }
};
        locals.var_mnud = assign10920_e10234;
        locals.var_mnud_dn3 = assign10920_e10234_d_n3;
        locals.var_mnud_dn4 = assign10920_e10234_d_n4;
        locals.var_mnud_dn5 = assign10920_e10234_d_n5;
        locals.var_mnud_dn6 = assign10920_e10234_d_n6;
        locals.var_mnud_dn7 = assign10920_e10234_d_n7;
        locals.var_mnud_dn8 = assign10920_e10234_d_n8;
        locals.var_mnud_rv = 0.0;

        let (assign10930_e10239, assign10930_e10239_d_n3, assign10930_e10239_d_n4, assign10930_e10239_d_n5, assign10930_e10239_d_n6, assign10930_e10239_d_n7, assign10930_e10239_d_n8,) = {
    if (locals.var_guard109 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mnud, locals.var_mnud_dn3, locals.var_mnud_dn4, locals.var_mnud_dn5, locals.var_mnud_dn6, locals.var_mnud_dn7, locals.var_mnud_dn8,)
    }
};
        locals.var_mnud = assign10930_e10239;
        locals.var_mnud_dn3 = assign10930_e10239_d_n3;
        locals.var_mnud_dn4 = assign10930_e10239_d_n4;
        locals.var_mnud_dn5 = assign10930_e10239_d_n5;
        locals.var_mnud_dn6 = assign10930_e10239_d_n6;
        locals.var_mnud_dn7 = assign10930_e10239_d_n7;
        locals.var_mnud_dn8 = assign10930_e10239_d_n8;
        locals.var_mnud_rv = 0.0;

        let assign10940_e10242: f64 = (locals.var_qicores - locals.var_qicored);
        locals.var_t0 = assign10940_e10242;
        locals.var_t0_dn3 = (locals.var_qicores_dn3 - locals.var_qicored_dn3);
        locals.var_t0_dn4 = (locals.var_qicores_dn4 - locals.var_qicored_dn4);
        locals.var_t0_dn5 = (locals.var_qicores_dn5 - locals.var_qicored_dn5);
        locals.var_t0_dn6 = (locals.var_qicores_dn6 - locals.var_qicored_dn6);
        locals.var_t0_dn7 = (locals.var_qicores_dn7 - locals.var_qicored_dn7);
        locals.var_t0_dn8 = (locals.var_qicores_dn8 - locals.var_qicored_dn8);
        locals.var_t0_rv = 0.0;

        let assign10950_e10245: f64 = (locals.var_qicores * locals.var_qicores);
        let assign10950_e10248: f64 = (locals.var_qicored * locals.var_qicored);
        let assign10950_e10249: f64 = (assign10950_e10245 - assign10950_e10248);
        locals.var_t1 = assign10950_e10249;
        locals.var_t1_dn3 = (((locals.var_qicores_dn3 * locals.var_qicores) + (locals.var_qicores * locals.var_qicores_dn3)) - ((locals.var_qicored_dn3 * locals.var_qicored) + (locals.var_qicored * locals.var_qicored_dn3)));
        locals.var_t1_dn4 = (((locals.var_qicores_dn4 * locals.var_qicores) + (locals.var_qicores * locals.var_qicores_dn4)) - ((locals.var_qicored_dn4 * locals.var_qicored) + (locals.var_qicored * locals.var_qicored_dn4)));
        locals.var_t1_dn5 = (((locals.var_qicores_dn5 * locals.var_qicores) + (locals.var_qicores * locals.var_qicores_dn5)) - ((locals.var_qicored_dn5 * locals.var_qicored) + (locals.var_qicored * locals.var_qicored_dn5)));
        locals.var_t1_dn6 = (((locals.var_qicores_dn6 * locals.var_qicores) + (locals.var_qicores * locals.var_qicores_dn6)) - ((locals.var_qicored_dn6 * locals.var_qicored) + (locals.var_qicored * locals.var_qicored_dn6)));
        locals.var_t1_dn7 = (((locals.var_qicores_dn7 * locals.var_qicores) + (locals.var_qicores * locals.var_qicores_dn7)) - ((locals.var_qicored_dn7 * locals.var_qicored) + (locals.var_qicored * locals.var_qicored_dn7)));
        locals.var_t1_dn8 = (((locals.var_qicores_dn8 * locals.var_qicores) + (locals.var_qicores * locals.var_qicores_dn8)) - ((locals.var_qicored_dn8 * locals.var_qicored) + (locals.var_qicored * locals.var_qicored_dn8)));
        locals.var_t1_rv = 0.0;

        let assign10960_e10252: f64 = (locals.var_csi * locals.var_nvtm);
        let assign10960_e10254: f64 = (assign10960_e10252 * 2.0);
        let assign10960_e10256: f64 = (assign10960_e10254 * locals.var_vtm);
        let assign10960_e10258: f64 = (assign10960_e10256 * locals.var_t0);
        let assign10960_e10261: f64 = (locals.var_csi * locals.var_nvtm);
        let assign10960_e10263: f64 = (assign10960_e10261 * locals.var_csi);
        let assign10960_e10265: f64 = (assign10960_e10263 * locals.var_nvtm);
        let assign10960_e10267: f64 = (assign10960_e10265 * 0.5);
        let assign10960_e10269: f64 = (assign10960_e10267 * locals.var_t1);
        let assign10960_e10271: f64 = (assign10960_e10269 / locals.var_cox1);
        let assign10960_e10272: f64 = (assign10960_e10258 + assign10960_e10271);
        locals.var_ids0 = assign10960_e10272;
        locals.var_ids0_dn3 = ((((((locals.var_csi * locals.var_nvtm_dn3) * 2.0) * locals.var_vtm) * locals.var_t0) + (assign10960_e10256 * locals.var_t0_dn3)) + ((((((((locals.var_csi * locals.var_nvtm_dn3) * locals.var_csi) * locals.var_nvtm) + (assign10960_e10263 * locals.var_nvtm_dn3)) * 0.5) * locals.var_t1) + (assign10960_e10267 * locals.var_t1_dn3)) / locals.var_cox1));
        locals.var_ids0_dn4 = (((((((locals.var_csi * locals.var_nvtm_dn4) * 2.0) * locals.var_vtm) + (assign10960_e10254 * locals.var_vtm_dn4)) * locals.var_t0) + (assign10960_e10256 * locals.var_t0_dn4)) + ((((((((locals.var_csi * locals.var_nvtm_dn4) * locals.var_csi) * locals.var_nvtm) + (assign10960_e10263 * locals.var_nvtm_dn4)) * 0.5) * locals.var_t1) + (assign10960_e10267 * locals.var_t1_dn4)) / locals.var_cox1));
        locals.var_ids0_dn5 = ((((((locals.var_csi * locals.var_nvtm_dn5) * 2.0) * locals.var_vtm) * locals.var_t0) + (assign10960_e10256 * locals.var_t0_dn5)) + ((((((((locals.var_csi * locals.var_nvtm_dn5) * locals.var_csi) * locals.var_nvtm) + (assign10960_e10263 * locals.var_nvtm_dn5)) * 0.5) * locals.var_t1) + (assign10960_e10267 * locals.var_t1_dn5)) / locals.var_cox1));
        locals.var_ids0_dn6 = ((((((locals.var_csi * locals.var_nvtm_dn6) * 2.0) * locals.var_vtm) * locals.var_t0) + (assign10960_e10256 * locals.var_t0_dn6)) + ((((((((locals.var_csi * locals.var_nvtm_dn6) * locals.var_csi) * locals.var_nvtm) + (assign10960_e10263 * locals.var_nvtm_dn6)) * 0.5) * locals.var_t1) + (assign10960_e10267 * locals.var_t1_dn6)) / locals.var_cox1));
        locals.var_ids0_dn7 = ((((((locals.var_csi * locals.var_nvtm_dn7) * 2.0) * locals.var_vtm) * locals.var_t0) + (assign10960_e10256 * locals.var_t0_dn7)) + ((((((((locals.var_csi * locals.var_nvtm_dn7) * locals.var_csi) * locals.var_nvtm) + (assign10960_e10263 * locals.var_nvtm_dn7)) * 0.5) * locals.var_t1) + (assign10960_e10267 * locals.var_t1_dn7)) / locals.var_cox1));
        locals.var_ids0_dn8 = ((((((locals.var_csi * locals.var_nvtm_dn8) * 2.0) * locals.var_vtm) * locals.var_t0) + (assign10960_e10256 * locals.var_t0_dn8)) + ((((((((locals.var_csi * locals.var_nvtm_dn8) * locals.var_csi) * locals.var_nvtm) + (assign10960_e10263 * locals.var_nvtm_dn8)) * 0.5) * locals.var_t1) + (assign10960_e10267 * locals.var_t1_dn8)) / locals.var_cox1));
        locals.var_ids0_rv = 0.0;

        let assign10970_e10276: f64 = (locals.var_qis + locals.var_qid);
        let assign10970_e10277: f64 = (0.5 * assign10970_e10276);
        let assign10970_e10279: f64 = (assign10970_e10277 + locals.var_vtm);
        locals.var_ids0_ov_dqi = assign10970_e10279;
        locals.var_ids0_ov_dqi_dn3 = (0.5 * (locals.var_qis_dn3 + locals.var_qid_dn3));
        locals.var_ids0_ov_dqi_dn4 = ((0.5 * (locals.var_qis_dn4 + locals.var_qid_dn4)) + locals.var_vtm_dn4);
        locals.var_ids0_ov_dqi_dn5 = (0.5 * (locals.var_qis_dn5 + locals.var_qid_dn5));
        locals.var_ids0_ov_dqi_dn6 = (0.5 * (locals.var_qis_dn6 + locals.var_qid_dn6));
        locals.var_ids0_ov_dqi_dn7 = (0.5 * (locals.var_qis_dn7 + locals.var_qid_dn7));
        locals.var_ids0_ov_dqi_dn8 = (0.5 * (locals.var_qis_dn8 + locals.var_qid_dn8));
        locals.var_ids0_ov_dqi_rv = 0.0;

        let assign10980_e10282: f64 = if p.p14 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard116 = assign10980_e10282;
        locals.var_guard116_rv = 0.0;

        let (assign10990_e10286, assign10990_e10286_d_n3, assign10990_e10286_d_n4, assign10990_e10286_d_n5, assign10990_e10286_d_n6, assign10990_e10286_d_n7, assign10990_e10286_d_n8,) = {
    if (locals.var_guard116 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdsi, locals.var_rdsi_dn3, locals.var_rdsi_dn4, locals.var_rdsi_dn5, locals.var_rdsi_dn6, locals.var_rdsi_dn7, locals.var_rdsi_dn8,)
    }
};
        locals.var_rdsi = assign10990_e10286;
        locals.var_rdsi_dn3 = assign10990_e10286_d_n3;
        locals.var_rdsi_dn4 = assign10990_e10286_d_n4;
        locals.var_rdsi_dn5 = assign10990_e10286_d_n5;
        locals.var_rdsi_dn6 = assign10990_e10286_d_n6;
        locals.var_rdsi_dn7 = assign10990_e10286_d_n7;
        locals.var_rdsi_dn8 = assign10990_e10286_d_n8;
        locals.var_rdsi_rv = 0.0;

        let (assign11000_e10290, assign11000_e10290_d_n3, assign11000_e10290_d_n4, assign11000_e10290_d_n5, assign11000_e10290_d_n6, assign11000_e10290_d_n7, assign11000_e10290_d_n8,) = {
    if (locals.var_guard116 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dr, locals.var_dr_dn3, locals.var_dr_dn4, locals.var_dr_dn5, locals.var_dr_dn6, locals.var_dr_dn7, locals.var_dr_dn8,)
    }
};
        locals.var_dr = assign11000_e10290;
        locals.var_dr_dn3 = assign11000_e10290_d_n3;
        locals.var_dr_dn4 = assign11000_e10290_d_n4;
        locals.var_dr_dn5 = assign11000_e10290_d_n5;
        locals.var_dr_dn6 = assign11000_e10290_d_n6;
        locals.var_dr_dn7 = assign11000_e10290_d_n7;
        locals.var_dr_dn8 = assign11000_e10290_d_n8;
        locals.var_dr_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_25(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11010_e10296, assign11010_e10296_d_n3, assign11010_e10296_d_n4, assign11010_e10296_d_n5, assign11010_e10296_d_n6, assign11010_e10296_d_n7, assign11010_e10296_d_n8,) = {
    if (locals.var_guard116 != 0.0) {
        let assign11010_e10294: f64 = (locals.var_vgs_noswap - locals.var_vfbsd);
        (assign11010_e10294, (-locals.var_vfbsd_dn3), (-locals.var_vfbsd_dn4), (-locals.var_vfbsd_dn5), (locals.var_vgs_noswap_dn6 - locals.var_vfbsd_dn6), (-locals.var_vfbsd_dn7), (locals.var_vgs_noswap_dn8 - locals.var_vfbsd_dn8),)
    } else {
        (locals.var_t2__blk114, locals.var_t2__blk114_dn3, locals.var_t2__blk114_dn4, locals.var_t2__blk114_dn5, locals.var_t2__blk114_dn6, locals.var_t2__blk114_dn7, locals.var_t2__blk114_dn8,)
    }
};
        locals.var_t2__blk114 = assign11010_e10296;
        locals.var_t2__blk114_dn3 = assign11010_e10296_d_n3;
        locals.var_t2__blk114_dn4 = assign11010_e10296_d_n4;
        locals.var_t2__blk114_dn5 = assign11010_e10296_d_n5;
        locals.var_t2__blk114_dn6 = assign11010_e10296_d_n6;
        locals.var_t2__blk114_dn7 = assign11010_e10296_d_n7;
        locals.var_t2__blk114_dn8 = assign11010_e10296_d_n8;
        locals.var_t2__blk114_rv = 0.0;

        let (assign11020_e10305, assign11020_e10305_d_n3, assign11020_e10305_d_n4, assign11020_e10305_d_n5, assign11020_e10305_d_n6, assign11020_e10305_d_n7, assign11020_e10305_d_n8,) = {
    if (locals.var_guard116 != 0.0) {
        let assign11020_e10300: f64 = (locals.var_t2__blk114 * locals.var_t2__blk114);
        let assign11020_e10302: f64 = (assign11020_e10300 + 0.0001);
        let assign11020_e10303: f64 = (assign11020_e10302).sqrt();
        (assign11020_e10303, (((locals.var_t2__blk114_dn3 * locals.var_t2__blk114) + (locals.var_t2__blk114 * locals.var_t2__blk114_dn3)) / (2.0 * assign11020_e10303)), (((locals.var_t2__blk114_dn4 * locals.var_t2__blk114) + (locals.var_t2__blk114 * locals.var_t2__blk114_dn4)) / (2.0 * assign11020_e10303)), (((locals.var_t2__blk114_dn5 * locals.var_t2__blk114) + (locals.var_t2__blk114 * locals.var_t2__blk114_dn5)) / (2.0 * assign11020_e10303)), (((locals.var_t2__blk114_dn6 * locals.var_t2__blk114) + (locals.var_t2__blk114 * locals.var_t2__blk114_dn6)) / (2.0 * assign11020_e10303)), (((locals.var_t2__blk114_dn7 * locals.var_t2__blk114) + (locals.var_t2__blk114 * locals.var_t2__blk114_dn7)) / (2.0 * assign11020_e10303)), (((locals.var_t2__blk114_dn8 * locals.var_t2__blk114) + (locals.var_t2__blk114 * locals.var_t2__blk114_dn8)) / (2.0 * assign11020_e10303)),)
    } else {
        (locals.var_t3__blk115, locals.var_t3__blk115_dn3, locals.var_t3__blk115_dn4, locals.var_t3__blk115_dn5, locals.var_t3__blk115_dn6, locals.var_t3__blk115_dn7, locals.var_t3__blk115_dn8,)
    }
};
        locals.var_t3__blk115 = assign11020_e10305;
        locals.var_t3__blk115_dn3 = assign11020_e10305_d_n3;
        locals.var_t3__blk115_dn4 = assign11020_e10305_d_n4;
        locals.var_t3__blk115_dn5 = assign11020_e10305_d_n5;
        locals.var_t3__blk115_dn6 = assign11020_e10305_d_n6;
        locals.var_t3__blk115_dn7 = assign11020_e10305_d_n7;
        locals.var_t3__blk115_dn8 = assign11020_e10305_d_n8;
        locals.var_t3__blk115_rv = 0.0;

        let (assign11030_e10313, assign11030_e10313_d_n3, assign11030_e10313_d_n4, assign11030_e10313_d_n5, assign11030_e10313_d_n6, assign11030_e10313_d_n7, assign11030_e10313_d_n8,) = {
    if (locals.var_guard116 != 0.0) {
        let assign11030_e10310: f64 = (locals.var_t2__blk114 + locals.var_t3__blk115);
        let assign11030_e10311: f64 = (0.5 * assign11030_e10310);
        (assign11030_e10311, (0.5 * (locals.var_t2__blk114_dn3 + locals.var_t3__blk115_dn3)), (0.5 * (locals.var_t2__blk114_dn4 + locals.var_t3__blk115_dn4)), (0.5 * (locals.var_t2__blk114_dn5 + locals.var_t3__blk115_dn5)), (0.5 * (locals.var_t2__blk114_dn6 + locals.var_t3__blk115_dn6)), (0.5 * (locals.var_t2__blk114_dn7 + locals.var_t3__blk115_dn7)), (0.5 * (locals.var_t2__blk114_dn8 + locals.var_t3__blk115_dn8)),)
    } else {
        (locals.var_vgs_eff, locals.var_vgs_eff_dn3, locals.var_vgs_eff_dn4, locals.var_vgs_eff_dn5, locals.var_vgs_eff_dn6, locals.var_vgs_eff_dn7, locals.var_vgs_eff_dn8,)
    }
};
        locals.var_vgs_eff = assign11030_e10313;
        locals.var_vgs_eff_dn3 = assign11030_e10313_d_n3;
        locals.var_vgs_eff_dn4 = assign11030_e10313_d_n4;
        locals.var_vgs_eff_dn5 = assign11030_e10313_d_n5;
        locals.var_vgs_eff_dn6 = assign11030_e10313_d_n6;
        locals.var_vgs_eff_dn7 = assign11030_e10313_d_n7;
        locals.var_vgs_eff_dn8 = assign11030_e10313_d_n8;
        locals.var_vgs_eff_rv = 0.0;

        let (assign11040_e10321, assign11040_e10321_d_n3, assign11040_e10321_d_n4, assign11040_e10321_d_n5, assign11040_e10321_d_n6, assign11040_e10321_d_n7, assign11040_e10321_d_n8,) = {
    if (locals.var_guard116 != 0.0) {
        let assign11040_e10318: f64 = (locals.var_prwg_i * locals.var_vgs_eff);
        let assign11040_e10319: f64 = (1.0 + assign11040_e10318);
        (assign11040_e10319, (locals.var_prwg_i * locals.var_vgs_eff_dn3), (locals.var_prwg_i * locals.var_vgs_eff_dn4), (locals.var_prwg_i * locals.var_vgs_eff_dn5), (locals.var_prwg_i * locals.var_vgs_eff_dn6), (locals.var_prwg_i * locals.var_vgs_eff_dn7), (locals.var_prwg_i * locals.var_vgs_eff_dn8),)
    } else {
        (locals.var_t4__blk111, locals.var_t4__blk111_dn3, locals.var_t4__blk111_dn4, locals.var_t4__blk111_dn5, locals.var_t4__blk111_dn6, locals.var_t4__blk111_dn7, locals.var_t4__blk111_dn8,)
    }
};
        locals.var_t4__blk111 = assign11040_e10321;
        locals.var_t4__blk111_dn3 = assign11040_e10321_d_n3;
        locals.var_t4__blk111_dn4 = assign11040_e10321_d_n4;
        locals.var_t4__blk111_dn5 = assign11040_e10321_d_n5;
        locals.var_t4__blk111_dn6 = assign11040_e10321_d_n6;
        locals.var_t4__blk111_dn7 = assign11040_e10321_d_n7;
        locals.var_t4__blk111_dn8 = assign11040_e10321_d_n8;
        locals.var_t4__blk111_rv = 0.0;

        let (assign11050_e10327, assign11050_e10327_d_n3, assign11050_e10327_d_n4, assign11050_e10327_d_n5, assign11050_e10327_d_n6, assign11050_e10327_d_n7, assign11050_e10327_d_n8,) = {
    if (locals.var_guard116 != 0.0) {
        let assign11050_e10325: f64 = (1.0 / locals.var_t4__blk111);
        (assign11050_e10325, (-(locals.var_t4__blk111_dn3 / (locals.var_t4__blk111 * locals.var_t4__blk111))), (-(locals.var_t4__blk111_dn4 / (locals.var_t4__blk111 * locals.var_t4__blk111))), (-(locals.var_t4__blk111_dn5 / (locals.var_t4__blk111 * locals.var_t4__blk111))), (-(locals.var_t4__blk111_dn6 / (locals.var_t4__blk111 * locals.var_t4__blk111))), (-(locals.var_t4__blk111_dn7 / (locals.var_t4__blk111 * locals.var_t4__blk111))), (-(locals.var_t4__blk111_dn8 / (locals.var_t4__blk111 * locals.var_t4__blk111))),)
    } else {
        (locals.var_t1__blk110, locals.var_t1__blk110_dn3, locals.var_t1__blk110_dn4, locals.var_t1__blk110_dn5, locals.var_t1__blk110_dn6, locals.var_t1__blk110_dn7, locals.var_t1__blk110_dn8,)
    }
};
        locals.var_t1__blk110 = assign11050_e10327;
        locals.var_t1__blk110_dn3 = assign11050_e10327_d_n3;
        locals.var_t1__blk110_dn4 = assign11050_e10327_d_n4;
        locals.var_t1__blk110_dn5 = assign11050_e10327_d_n5;
        locals.var_t1__blk110_dn6 = assign11050_e10327_d_n6;
        locals.var_t1__blk110_dn7 = assign11050_e10327_d_n7;
        locals.var_t1__blk110_dn8 = assign11050_e10327_d_n8;
        locals.var_t1__blk110_rv = 0.0;

        let (assign11060_e10337, assign11060_e10337_d_n3, assign11060_e10337_d_n4, assign11060_e10337_d_n5, assign11060_e10337_d_n6, assign11060_e10337_d_n7, assign11060_e10337_d_n8,) = {
    if (locals.var_guard116 != 0.0) {
        let assign11060_e10332: f64 = (0.5 * locals.var_vbgs_noswap);
        let assign11060_e10334: f64 = (assign11060_e10332 * locals.var_prwb_i);
        let assign11060_e10335: f64 = (locals.var_t1__blk110 - assign11060_e10334);
        (assign11060_e10335, (locals.var_t1__blk110_dn3 - ((0.5 * locals.var_vbgs_noswap_dn3) * locals.var_prwb_i)), locals.var_t1__blk110_dn4, locals.var_t1__blk110_dn5, (locals.var_t1__blk110_dn6 - ((0.5 * locals.var_vbgs_noswap_dn6) * locals.var_prwb_i)), locals.var_t1__blk110_dn7, locals.var_t1__blk110_dn8,)
    } else {
        (locals.var_t1__blk110, locals.var_t1__blk110_dn3, locals.var_t1__blk110_dn4, locals.var_t1__blk110_dn5, locals.var_t1__blk110_dn6, locals.var_t1__blk110_dn7, locals.var_t1__blk110_dn8,)
    }
};
        locals.var_t1__blk110 = assign11060_e10337;
        locals.var_t1__blk110_dn3 = assign11060_e10337_d_n3;
        locals.var_t1__blk110_dn4 = assign11060_e10337_d_n4;
        locals.var_t1__blk110_dn5 = assign11060_e10337_d_n5;
        locals.var_t1__blk110_dn6 = assign11060_e10337_d_n6;
        locals.var_t1__blk110_dn7 = assign11060_e10337_d_n7;
        locals.var_t1__blk110_dn8 = assign11060_e10337_d_n8;
        locals.var_t1__blk110_rv = 0.0;

        let (assign11070_e10350, assign11070_e10350_d_n3, assign11070_e10350_d_n4, assign11070_e10350_d_n5, assign11070_e10350_d_n6, assign11070_e10350_d_n7, assign11070_e10350_d_n8,) = {
    if (locals.var_guard116 != 0.0) {
        let assign11070_e10343: f64 = (locals.var_t1__blk110 * locals.var_t1__blk110);
        let assign11070_e10345: f64 = (assign11070_e10343 + 0.01);
        let assign11070_e10346: f64 = (assign11070_e10345).sqrt();
        let assign11070_e10347: f64 = (locals.var_t1__blk110 + assign11070_e10346);
        let assign11070_e10348: f64 = (0.5 * assign11070_e10347);
        (assign11070_e10348, (0.5 * (locals.var_t1__blk110_dn3 + (((locals.var_t1__blk110_dn3 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn3)) / (2.0 * assign11070_e10346)))), (0.5 * (locals.var_t1__blk110_dn4 + (((locals.var_t1__blk110_dn4 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn4)) / (2.0 * assign11070_e10346)))), (0.5 * (locals.var_t1__blk110_dn5 + (((locals.var_t1__blk110_dn5 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn5)) / (2.0 * assign11070_e10346)))), (0.5 * (locals.var_t1__blk110_dn6 + (((locals.var_t1__blk110_dn6 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn6)) / (2.0 * assign11070_e10346)))), (0.5 * (locals.var_t1__blk110_dn7 + (((locals.var_t1__blk110_dn7 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn7)) / (2.0 * assign11070_e10346)))), (0.5 * (locals.var_t1__blk110_dn8 + (((locals.var_t1__blk110_dn8 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn8)) / (2.0 * assign11070_e10346)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    }
};
        locals.var_t0 = assign11070_e10350;
        locals.var_t0_dn3 = assign11070_e10350_d_n3;
        locals.var_t0_dn4 = assign11070_e10350_d_n4;
        locals.var_t0_dn5 = assign11070_e10350_d_n5;
        locals.var_t0_dn6 = assign11070_e10350_d_n6;
        locals.var_t0_dn7 = assign11070_e10350_d_n7;
        locals.var_t0_dn8 = assign11070_e10350_d_n8;
        locals.var_t0_rv = 0.0;

        let (assign11090_e10370, assign11090_e10370_d_n3, assign11090_e10370_d_n4, assign11090_e10370_d_n5, assign11090_e10370_d_n6, assign11090_e10370_d_n7, assign11090_e10370_d_n8,) = {
    if (locals.var_guard116 != 0.0) {
        let assign11090_e10368: f64 = (locals.var_vgd_noswap - locals.var_vfbsd);
        (assign11090_e10368, (-locals.var_vfbsd_dn3), (-locals.var_vfbsd_dn4), (locals.var_vgd_noswap_dn5 - locals.var_vfbsd_dn5), (-locals.var_vfbsd_dn6), (-locals.var_vfbsd_dn7), (locals.var_vgd_noswap_dn8 - locals.var_vfbsd_dn8),)
    } else {
        (locals.var_t2__blk114, locals.var_t2__blk114_dn3, locals.var_t2__blk114_dn4, locals.var_t2__blk114_dn5, locals.var_t2__blk114_dn6, locals.var_t2__blk114_dn7, locals.var_t2__blk114_dn8,)
    }
};
        locals.var_t2__blk114 = assign11090_e10370;
        locals.var_t2__blk114_dn3 = assign11090_e10370_d_n3;
        locals.var_t2__blk114_dn4 = assign11090_e10370_d_n4;
        locals.var_t2__blk114_dn5 = assign11090_e10370_d_n5;
        locals.var_t2__blk114_dn6 = assign11090_e10370_d_n6;
        locals.var_t2__blk114_dn7 = assign11090_e10370_d_n7;
        locals.var_t2__blk114_dn8 = assign11090_e10370_d_n8;
        locals.var_t2__blk114_rv = 0.0;

        let (assign11100_e10379, assign11100_e10379_d_n3, assign11100_e10379_d_n4, assign11100_e10379_d_n5, assign11100_e10379_d_n6, assign11100_e10379_d_n7, assign11100_e10379_d_n8,) = {
    if (locals.var_guard116 != 0.0) {
        let assign11100_e10374: f64 = (locals.var_t2__blk114 * locals.var_t2__blk114);
        let assign11100_e10376: f64 = (assign11100_e10374 + 0.0001);
        let assign11100_e10377: f64 = (assign11100_e10376).sqrt();
        (assign11100_e10377, (((locals.var_t2__blk114_dn3 * locals.var_t2__blk114) + (locals.var_t2__blk114 * locals.var_t2__blk114_dn3)) / (2.0 * assign11100_e10377)), (((locals.var_t2__blk114_dn4 * locals.var_t2__blk114) + (locals.var_t2__blk114 * locals.var_t2__blk114_dn4)) / (2.0 * assign11100_e10377)), (((locals.var_t2__blk114_dn5 * locals.var_t2__blk114) + (locals.var_t2__blk114 * locals.var_t2__blk114_dn5)) / (2.0 * assign11100_e10377)), (((locals.var_t2__blk114_dn6 * locals.var_t2__blk114) + (locals.var_t2__blk114 * locals.var_t2__blk114_dn6)) / (2.0 * assign11100_e10377)), (((locals.var_t2__blk114_dn7 * locals.var_t2__blk114) + (locals.var_t2__blk114 * locals.var_t2__blk114_dn7)) / (2.0 * assign11100_e10377)), (((locals.var_t2__blk114_dn8 * locals.var_t2__blk114) + (locals.var_t2__blk114 * locals.var_t2__blk114_dn8)) / (2.0 * assign11100_e10377)),)
    } else {
        (locals.var_t3__blk115, locals.var_t3__blk115_dn3, locals.var_t3__blk115_dn4, locals.var_t3__blk115_dn5, locals.var_t3__blk115_dn6, locals.var_t3__blk115_dn7, locals.var_t3__blk115_dn8,)
    }
};
        locals.var_t3__blk115 = assign11100_e10379;
        locals.var_t3__blk115_dn3 = assign11100_e10379_d_n3;
        locals.var_t3__blk115_dn4 = assign11100_e10379_d_n4;
        locals.var_t3__blk115_dn5 = assign11100_e10379_d_n5;
        locals.var_t3__blk115_dn6 = assign11100_e10379_d_n6;
        locals.var_t3__blk115_dn7 = assign11100_e10379_d_n7;
        locals.var_t3__blk115_dn8 = assign11100_e10379_d_n8;
        locals.var_t3__blk115_rv = 0.0;

        let (assign11110_e10387, assign11110_e10387_d_n3, assign11110_e10387_d_n4, assign11110_e10387_d_n5, assign11110_e10387_d_n6, assign11110_e10387_d_n7, assign11110_e10387_d_n8,) = {
    if (locals.var_guard116 != 0.0) {
        let assign11110_e10384: f64 = (locals.var_t2__blk114 + locals.var_t3__blk115);
        let assign11110_e10385: f64 = (0.5 * assign11110_e10384);
        (assign11110_e10385, (0.5 * (locals.var_t2__blk114_dn3 + locals.var_t3__blk115_dn3)), (0.5 * (locals.var_t2__blk114_dn4 + locals.var_t3__blk115_dn4)), (0.5 * (locals.var_t2__blk114_dn5 + locals.var_t3__blk115_dn5)), (0.5 * (locals.var_t2__blk114_dn6 + locals.var_t3__blk115_dn6)), (0.5 * (locals.var_t2__blk114_dn7 + locals.var_t3__blk115_dn7)), (0.5 * (locals.var_t2__blk114_dn8 + locals.var_t3__blk115_dn8)),)
    } else {
        (locals.var_vgd_eff, locals.var_vgd_eff_dn3, locals.var_vgd_eff_dn4, locals.var_vgd_eff_dn5, locals.var_vgd_eff_dn6, locals.var_vgd_eff_dn7, locals.var_vgd_eff_dn8,)
    }
};
        locals.var_vgd_eff = assign11110_e10387;
        locals.var_vgd_eff_dn3 = assign11110_e10387_d_n3;
        locals.var_vgd_eff_dn4 = assign11110_e10387_d_n4;
        locals.var_vgd_eff_dn5 = assign11110_e10387_d_n5;
        locals.var_vgd_eff_dn6 = assign11110_e10387_d_n6;
        locals.var_vgd_eff_dn7 = assign11110_e10387_d_n7;
        locals.var_vgd_eff_dn8 = assign11110_e10387_d_n8;
        locals.var_vgd_eff_rv = 0.0;

        let (assign11120_e10395, assign11120_e10395_d_n3, assign11120_e10395_d_n4, assign11120_e10395_d_n5, assign11120_e10395_d_n6, assign11120_e10395_d_n7, assign11120_e10395_d_n8,) = {
    if (locals.var_guard116 != 0.0) {
        let assign11120_e10392: f64 = (locals.var_prwg_i * locals.var_vgd_eff);
        let assign11120_e10393: f64 = (1.0 + assign11120_e10392);
        (assign11120_e10393, (locals.var_prwg_i * locals.var_vgd_eff_dn3), (locals.var_prwg_i * locals.var_vgd_eff_dn4), (locals.var_prwg_i * locals.var_vgd_eff_dn5), (locals.var_prwg_i * locals.var_vgd_eff_dn6), (locals.var_prwg_i * locals.var_vgd_eff_dn7), (locals.var_prwg_i * locals.var_vgd_eff_dn8),)
    } else {
        (locals.var_t4__blk111, locals.var_t4__blk111_dn3, locals.var_t4__blk111_dn4, locals.var_t4__blk111_dn5, locals.var_t4__blk111_dn6, locals.var_t4__blk111_dn7, locals.var_t4__blk111_dn8,)
    }
};
        locals.var_t4__blk111 = assign11120_e10395;
        locals.var_t4__blk111_dn3 = assign11120_e10395_d_n3;
        locals.var_t4__blk111_dn4 = assign11120_e10395_d_n4;
        locals.var_t4__blk111_dn5 = assign11120_e10395_d_n5;
        locals.var_t4__blk111_dn6 = assign11120_e10395_d_n6;
        locals.var_t4__blk111_dn7 = assign11120_e10395_d_n7;
        locals.var_t4__blk111_dn8 = assign11120_e10395_d_n8;
        locals.var_t4__blk111_rv = 0.0;

        let (assign11130_e10401, assign11130_e10401_d_n3, assign11130_e10401_d_n4, assign11130_e10401_d_n5, assign11130_e10401_d_n6, assign11130_e10401_d_n7, assign11130_e10401_d_n8,) = {
    if (locals.var_guard116 != 0.0) {
        let assign11130_e10399: f64 = (1.0 / locals.var_t4__blk111);
        (assign11130_e10399, (-(locals.var_t4__blk111_dn3 / (locals.var_t4__blk111 * locals.var_t4__blk111))), (-(locals.var_t4__blk111_dn4 / (locals.var_t4__blk111 * locals.var_t4__blk111))), (-(locals.var_t4__blk111_dn5 / (locals.var_t4__blk111 * locals.var_t4__blk111))), (-(locals.var_t4__blk111_dn6 / (locals.var_t4__blk111 * locals.var_t4__blk111))), (-(locals.var_t4__blk111_dn7 / (locals.var_t4__blk111 * locals.var_t4__blk111))), (-(locals.var_t4__blk111_dn8 / (locals.var_t4__blk111 * locals.var_t4__blk111))),)
    } else {
        (locals.var_t1__blk110, locals.var_t1__blk110_dn3, locals.var_t1__blk110_dn4, locals.var_t1__blk110_dn5, locals.var_t1__blk110_dn6, locals.var_t1__blk110_dn7, locals.var_t1__blk110_dn8,)
    }
};
        locals.var_t1__blk110 = assign11130_e10401;
        locals.var_t1__blk110_dn3 = assign11130_e10401_d_n3;
        locals.var_t1__blk110_dn4 = assign11130_e10401_d_n4;
        locals.var_t1__blk110_dn5 = assign11130_e10401_d_n5;
        locals.var_t1__blk110_dn6 = assign11130_e10401_d_n6;
        locals.var_t1__blk110_dn7 = assign11130_e10401_d_n7;
        locals.var_t1__blk110_dn8 = assign11130_e10401_d_n8;
        locals.var_t1__blk110_rv = 0.0;

        let (assign11140_e10411, assign11140_e10411_d_n3, assign11140_e10411_d_n4, assign11140_e10411_d_n5, assign11140_e10411_d_n6, assign11140_e10411_d_n7, assign11140_e10411_d_n8,) = {
    if (locals.var_guard116 != 0.0) {
        let assign11140_e10406: f64 = (0.5 * locals.var_vbgd_noswap);
        let assign11140_e10408: f64 = (assign11140_e10406 * locals.var_prwb_i);
        let assign11140_e10409: f64 = (locals.var_t1__blk110 - assign11140_e10408);
        (assign11140_e10409, (locals.var_t1__blk110_dn3 - ((0.5 * locals.var_vbgd_noswap_dn3) * locals.var_prwb_i)), locals.var_t1__blk110_dn4, (locals.var_t1__blk110_dn5 - ((0.5 * locals.var_vbgd_noswap_dn5) * locals.var_prwb_i)), locals.var_t1__blk110_dn6, locals.var_t1__blk110_dn7, locals.var_t1__blk110_dn8,)
    } else {
        (locals.var_t1__blk110, locals.var_t1__blk110_dn3, locals.var_t1__blk110_dn4, locals.var_t1__blk110_dn5, locals.var_t1__blk110_dn6, locals.var_t1__blk110_dn7, locals.var_t1__blk110_dn8,)
    }
};
        locals.var_t1__blk110 = assign11140_e10411;
        locals.var_t1__blk110_dn3 = assign11140_e10411_d_n3;
        locals.var_t1__blk110_dn4 = assign11140_e10411_d_n4;
        locals.var_t1__blk110_dn5 = assign11140_e10411_d_n5;
        locals.var_t1__blk110_dn6 = assign11140_e10411_d_n6;
        locals.var_t1__blk110_dn7 = assign11140_e10411_d_n7;
        locals.var_t1__blk110_dn8 = assign11140_e10411_d_n8;
        locals.var_t1__blk110_rv = 0.0;

        let (assign11150_e10424, assign11150_e10424_d_n3, assign11150_e10424_d_n4, assign11150_e10424_d_n5, assign11150_e10424_d_n6, assign11150_e10424_d_n7, assign11150_e10424_d_n8,) = {
    if (locals.var_guard116 != 0.0) {
        let assign11150_e10417: f64 = (locals.var_t1__blk110 * locals.var_t1__blk110);
        let assign11150_e10419: f64 = (assign11150_e10417 + 0.01);
        let assign11150_e10420: f64 = (assign11150_e10419).sqrt();
        let assign11150_e10421: f64 = (locals.var_t1__blk110 + assign11150_e10420);
        let assign11150_e10422: f64 = (0.5 * assign11150_e10421);
        (assign11150_e10422, (0.5 * (locals.var_t1__blk110_dn3 + (((locals.var_t1__blk110_dn3 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn3)) / (2.0 * assign11150_e10420)))), (0.5 * (locals.var_t1__blk110_dn4 + (((locals.var_t1__blk110_dn4 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn4)) / (2.0 * assign11150_e10420)))), (0.5 * (locals.var_t1__blk110_dn5 + (((locals.var_t1__blk110_dn5 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn5)) / (2.0 * assign11150_e10420)))), (0.5 * (locals.var_t1__blk110_dn6 + (((locals.var_t1__blk110_dn6 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn6)) / (2.0 * assign11150_e10420)))), (0.5 * (locals.var_t1__blk110_dn7 + (((locals.var_t1__blk110_dn7 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn7)) / (2.0 * assign11150_e10420)))), (0.5 * (locals.var_t1__blk110_dn8 + (((locals.var_t1__blk110_dn8 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn8)) / (2.0 * assign11150_e10420)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    }
};
        locals.var_t0 = assign11150_e10424;
        locals.var_t0_dn3 = assign11150_e10424_d_n3;
        locals.var_t0_dn4 = assign11150_e10424_d_n4;
        locals.var_t0_dn5 = assign11150_e10424_d_n5;
        locals.var_t0_dn6 = assign11150_e10424_d_n6;
        locals.var_t0_dn7 = assign11150_e10424_d_n7;
        locals.var_t0_dn8 = assign11150_e10424_d_n8;
        locals.var_t0_rv = 0.0;

        let (assign11170_e10447, assign11170_e10447_d_n3, assign11170_e10447_d_n4, assign11170_e10447_d_n5, assign11170_e10447_d_n6, assign11170_e10447_d_n7, assign11170_e10447_d_n8,) = {
    if (locals.var_guard116 == 0.0) {
        let assign11170_e10444: f64 = (locals.var_prwg_i * locals.var_qia);
        let assign11170_e10445: f64 = (1.0 + assign11170_e10444);
        (assign11170_e10445, (locals.var_prwg_i * locals.var_qia_dn3), (locals.var_prwg_i * locals.var_qia_dn4), (locals.var_prwg_i * locals.var_qia_dn5), (locals.var_prwg_i * locals.var_qia_dn6), (locals.var_prwg_i * locals.var_qia_dn7), (locals.var_prwg_i * locals.var_qia_dn8),)
    } else {
        (locals.var_t4__blk111, locals.var_t4__blk111_dn3, locals.var_t4__blk111_dn4, locals.var_t4__blk111_dn5, locals.var_t4__blk111_dn6, locals.var_t4__blk111_dn7, locals.var_t4__blk111_dn8,)
    }
};
        locals.var_t4__blk111 = assign11170_e10447;
        locals.var_t4__blk111_dn3 = assign11170_e10447_d_n3;
        locals.var_t4__blk111_dn4 = assign11170_e10447_d_n4;
        locals.var_t4__blk111_dn5 = assign11170_e10447_d_n5;
        locals.var_t4__blk111_dn6 = assign11170_e10447_d_n6;
        locals.var_t4__blk111_dn7 = assign11170_e10447_d_n7;
        locals.var_t4__blk111_dn8 = assign11170_e10447_d_n8;
        locals.var_t4__blk111_rv = 0.0;

        let (assign11180_e10454, assign11180_e10454_d_n3, assign11180_e10454_d_n4, assign11180_e10454_d_n5, assign11180_e10454_d_n6, assign11180_e10454_d_n7, assign11180_e10454_d_n8,) = {
    if (locals.var_guard116 == 0.0) {
        let assign11180_e10452: f64 = (1.0 / locals.var_t4__blk111);
        (assign11180_e10452, (-(locals.var_t4__blk111_dn3 / (locals.var_t4__blk111 * locals.var_t4__blk111))), (-(locals.var_t4__blk111_dn4 / (locals.var_t4__blk111 * locals.var_t4__blk111))), (-(locals.var_t4__blk111_dn5 / (locals.var_t4__blk111 * locals.var_t4__blk111))), (-(locals.var_t4__blk111_dn6 / (locals.var_t4__blk111 * locals.var_t4__blk111))), (-(locals.var_t4__blk111_dn7 / (locals.var_t4__blk111 * locals.var_t4__blk111))), (-(locals.var_t4__blk111_dn8 / (locals.var_t4__blk111 * locals.var_t4__blk111))),)
    } else {
        (locals.var_t1__blk110, locals.var_t1__blk110_dn3, locals.var_t1__blk110_dn4, locals.var_t1__blk110_dn5, locals.var_t1__blk110_dn6, locals.var_t1__blk110_dn7, locals.var_t1__blk110_dn8,)
    }
};
        locals.var_t1__blk110 = assign11180_e10454;
        locals.var_t1__blk110_dn3 = assign11180_e10454_d_n3;
        locals.var_t1__blk110_dn4 = assign11180_e10454_d_n4;
        locals.var_t1__blk110_dn5 = assign11180_e10454_d_n5;
        locals.var_t1__blk110_dn6 = assign11180_e10454_d_n6;
        locals.var_t1__blk110_dn7 = assign11180_e10454_d_n7;
        locals.var_t1__blk110_dn8 = assign11180_e10454_d_n8;
        locals.var_t1__blk110_rv = 0.0;

        let (assign11190_e10467, assign11190_e10467_d_n3, assign11190_e10467_d_n4, assign11190_e10467_d_n5, assign11190_e10467_d_n6, assign11190_e10467_d_n7, assign11190_e10467_d_n8,) = {
    if (locals.var_guard116 == 0.0) {
        let assign11190_e10461: f64 = (locals.var_vbgd + locals.var_vbgs);
        let assign11190_e10462: f64 = (0.5 * assign11190_e10461);
        let assign11190_e10464: f64 = (assign11190_e10462 * locals.var_prwb_i);
        let assign11190_e10465: f64 = (locals.var_t1__blk110 - assign11190_e10464);
        (assign11190_e10465, (locals.var_t1__blk110_dn3 - ((0.5 * (locals.var_vbgd_dn3 + locals.var_vbgs_dn3)) * locals.var_prwb_i)), locals.var_t1__blk110_dn4, (locals.var_t1__blk110_dn5 - ((0.5 * (locals.var_vbgd_dn5 + locals.var_vbgs_dn5)) * locals.var_prwb_i)), (locals.var_t1__blk110_dn6 - ((0.5 * (locals.var_vbgd_dn6 + locals.var_vbgs_dn6)) * locals.var_prwb_i)), locals.var_t1__blk110_dn7, locals.var_t1__blk110_dn8,)
    } else {
        (locals.var_t1__blk110, locals.var_t1__blk110_dn3, locals.var_t1__blk110_dn4, locals.var_t1__blk110_dn5, locals.var_t1__blk110_dn6, locals.var_t1__blk110_dn7, locals.var_t1__blk110_dn8,)
    }
};
        locals.var_t1__blk110 = assign11190_e10467;
        locals.var_t1__blk110_dn3 = assign11190_e10467_d_n3;
        locals.var_t1__blk110_dn4 = assign11190_e10467_d_n4;
        locals.var_t1__blk110_dn5 = assign11190_e10467_d_n5;
        locals.var_t1__blk110_dn6 = assign11190_e10467_d_n6;
        locals.var_t1__blk110_dn7 = assign11190_e10467_d_n7;
        locals.var_t1__blk110_dn8 = assign11190_e10467_d_n8;
        locals.var_t1__blk110_rv = 0.0;

        let (assign11200_e10481, assign11200_e10481_d_n3, assign11200_e10481_d_n4, assign11200_e10481_d_n5, assign11200_e10481_d_n6, assign11200_e10481_d_n7, assign11200_e10481_d_n8,) = {
    if (locals.var_guard116 == 0.0) {
        let assign11200_e10474: f64 = (locals.var_t1__blk110 * locals.var_t1__blk110);
        let assign11200_e10476: f64 = (assign11200_e10474 + 0.01);
        let assign11200_e10477: f64 = (assign11200_e10476).sqrt();
        let assign11200_e10478: f64 = (locals.var_t1__blk110 + assign11200_e10477);
        let assign11200_e10479: f64 = (0.5 * assign11200_e10478);
        (assign11200_e10479, (0.5 * (locals.var_t1__blk110_dn3 + (((locals.var_t1__blk110_dn3 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn3)) / (2.0 * assign11200_e10477)))), (0.5 * (locals.var_t1__blk110_dn4 + (((locals.var_t1__blk110_dn4 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn4)) / (2.0 * assign11200_e10477)))), (0.5 * (locals.var_t1__blk110_dn5 + (((locals.var_t1__blk110_dn5 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn5)) / (2.0 * assign11200_e10477)))), (0.5 * (locals.var_t1__blk110_dn6 + (((locals.var_t1__blk110_dn6 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn6)) / (2.0 * assign11200_e10477)))), (0.5 * (locals.var_t1__blk110_dn7 + (((locals.var_t1__blk110_dn7 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn7)) / (2.0 * assign11200_e10477)))), (0.5 * (locals.var_t1__blk110_dn8 + (((locals.var_t1__blk110_dn8 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn8)) / (2.0 * assign11200_e10477)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    }
};
        locals.var_t0 = assign11200_e10481;
        locals.var_t0_dn3 = assign11200_e10481_d_n3;
        locals.var_t0_dn4 = assign11200_e10481_d_n4;
        locals.var_t0_dn5 = assign11200_e10481_d_n5;
        locals.var_t0_dn6 = assign11200_e10481_d_n6;
        locals.var_t0_dn7 = assign11200_e10481_d_n7;
        locals.var_t0_dn8 = assign11200_e10481_d_n8;
        locals.var_t0_rv = 0.0;

        let (assign11210_e10494, assign11210_e10494_d_n3, assign11210_e10494_d_n4, assign11210_e10494_d_n5, assign11210_e10494_d_n6, assign11210_e10494_d_n7, assign11210_e10494_d_n8,) = {
    if (locals.var_guard116 == 0.0) {
        let assign11210_e10488: f64 = (locals.var_rdsw_i * locals.var_t0);
        let assign11210_e10489: f64 = (locals.var_rdswmin_i + assign11210_e10488);
        let assign11210_e10491: f64 = (assign11210_e10489 * locals.var_weffwrfactor);
        let assign11210_e10492: f64 = (locals.var_rdstemp * assign11210_e10491);
        (assign11210_e10492, (locals.var_rdstemp * ((locals.var_rdsw_i * locals.var_t0_dn3) * locals.var_weffwrfactor)), ((locals.var_rdstemp_dn4 * assign11210_e10491) + (locals.var_rdstemp * ((locals.var_rdsw_i * locals.var_t0_dn4) * locals.var_weffwrfactor))), (locals.var_rdstemp * ((locals.var_rdsw_i * locals.var_t0_dn5) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdsw_i * locals.var_t0_dn6) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdsw_i * locals.var_t0_dn7) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdsw_i * locals.var_t0_dn8) * locals.var_weffwrfactor)),)
    } else {
        (locals.var_rdsi, locals.var_rdsi_dn3, locals.var_rdsi_dn4, locals.var_rdsi_dn5, locals.var_rdsi_dn6, locals.var_rdsi_dn7, locals.var_rdsi_dn8,)
    }
};
        locals.var_rdsi = assign11210_e10494;
        locals.var_rdsi_dn3 = assign11210_e10494_d_n3;
        locals.var_rdsi_dn4 = assign11210_e10494_d_n4;
        locals.var_rdsi_dn5 = assign11210_e10494_d_n5;
        locals.var_rdsi_dn6 = assign11210_e10494_d_n6;
        locals.var_rdsi_dn7 = assign11210_e10494_d_n7;
        locals.var_rdsi_dn8 = assign11210_e10494_d_n8;
        locals.var_rdsi_rv = 0.0;

        let (assign11220_e10509, assign11220_e10509_d_n3, assign11220_e10509_d_n4, assign11220_e10509_d_n5, assign11220_e10509_d_n6, assign11220_e10509_d_n7, assign11220_e10509_d_n8,) = {
    if (locals.var_guard116 == 0.0) {
        let assign11220_e10500: f64 = (p.p2 * locals.var_beta);
        let assign11220_e10502: f64 = (assign11220_e10500 * locals.var_ids0_ov_dqi);
        let assign11220_e10504: f64 = (assign11220_e10502 / locals.var_dvsat);
        let assign11220_e10506: f64 = (assign11220_e10504 * locals.var_rdsi);
        let assign11220_e10507: f64 = (1.0 + assign11220_e10506);
        (assign11220_e10507, ((((((((p.p2 * locals.var_beta_dn3) * locals.var_ids0_ov_dqi) + (assign11220_e10500 * locals.var_ids0_ov_dqi_dn3)) * locals.var_dvsat) - (assign11220_e10502 * locals.var_dvsat_dn3)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_rdsi) + (assign11220_e10504 * locals.var_rdsi_dn3)), ((((((((p.p2 * locals.var_beta_dn4) * locals.var_ids0_ov_dqi) + (assign11220_e10500 * locals.var_ids0_ov_dqi_dn4)) * locals.var_dvsat) - (assign11220_e10502 * locals.var_dvsat_dn4)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_rdsi) + (assign11220_e10504 * locals.var_rdsi_dn4)), ((((((((p.p2 * locals.var_beta_dn5) * locals.var_ids0_ov_dqi) + (assign11220_e10500 * locals.var_ids0_ov_dqi_dn5)) * locals.var_dvsat) - (assign11220_e10502 * locals.var_dvsat_dn5)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_rdsi) + (assign11220_e10504 * locals.var_rdsi_dn5)), ((((((((p.p2 * locals.var_beta_dn6) * locals.var_ids0_ov_dqi) + (assign11220_e10500 * locals.var_ids0_ov_dqi_dn6)) * locals.var_dvsat) - (assign11220_e10502 * locals.var_dvsat_dn6)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_rdsi) + (assign11220_e10504 * locals.var_rdsi_dn6)), ((((((((p.p2 * locals.var_beta_dn7) * locals.var_ids0_ov_dqi) + (assign11220_e10500 * locals.var_ids0_ov_dqi_dn7)) * locals.var_dvsat) - (assign11220_e10502 * locals.var_dvsat_dn7)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_rdsi) + (assign11220_e10504 * locals.var_rdsi_dn7)), ((((((((p.p2 * locals.var_beta_dn8) * locals.var_ids0_ov_dqi) + (assign11220_e10500 * locals.var_ids0_ov_dqi_dn8)) * locals.var_dvsat) - (assign11220_e10502 * locals.var_dvsat_dn8)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_rdsi) + (assign11220_e10504 * locals.var_rdsi_dn8)),)
    } else {
        (locals.var_dr, locals.var_dr_dn3, locals.var_dr_dn4, locals.var_dr_dn5, locals.var_dr_dn6, locals.var_dr_dn7, locals.var_dr_dn8,)
    }
};
        locals.var_dr = assign11220_e10509;
        locals.var_dr_dn3 = assign11220_e10509_d_n3;
        locals.var_dr_dn4 = assign11220_e10509_d_n4;
        locals.var_dr_dn5 = assign11220_e10509_d_n5;
        locals.var_dr_dn6 = assign11220_e10509_d_n6;
        locals.var_dr_dn7 = assign11220_e10509_d_n7;
        locals.var_dr_dn8 = assign11220_e10509_d_n8;
        locals.var_dr_rv = 0.0;

        let assign11250_e10522: f64 = if p.p14 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard117 = assign11250_e10522;
        locals.var_guard117_rv = 0.0;

        let (assign11260_e10533, assign11260_e10533_d_n3, assign11260_e10533_d_n4, assign11260_e10533_d_n5, assign11260_e10533_d_n6, assign11260_e10533_d_n7, assign11260_e10533_d_n8,) = {
    if ((locals.var_guard116 == 0.0) && (locals.var_guard117 != 0.0)) {
        let assign11260_e10530: f64 = (locals.var_prwg_i * locals.var_qia);
        let assign11260_e10531: f64 = (1.0 + assign11260_e10530);
        (assign11260_e10531, (locals.var_prwg_i * locals.var_qia_dn3), (locals.var_prwg_i * locals.var_qia_dn4), (locals.var_prwg_i * locals.var_qia_dn5), (locals.var_prwg_i * locals.var_qia_dn6), (locals.var_prwg_i * locals.var_qia_dn7), (locals.var_prwg_i * locals.var_qia_dn8),)
    } else {
        (locals.var_t4__blk111, locals.var_t4__blk111_dn3, locals.var_t4__blk111_dn4, locals.var_t4__blk111_dn5, locals.var_t4__blk111_dn6, locals.var_t4__blk111_dn7, locals.var_t4__blk111_dn8,)
    }
};
        locals.var_t4__blk111 = assign11260_e10533;
        locals.var_t4__blk111_dn3 = assign11260_e10533_d_n3;
        locals.var_t4__blk111_dn4 = assign11260_e10533_d_n4;
        locals.var_t4__blk111_dn5 = assign11260_e10533_d_n5;
        locals.var_t4__blk111_dn6 = assign11260_e10533_d_n6;
        locals.var_t4__blk111_dn7 = assign11260_e10533_d_n7;
        locals.var_t4__blk111_dn8 = assign11260_e10533_d_n8;
        locals.var_t4__blk111_rv = 0.0;

        let (assign11270_e10542, assign11270_e10542_d_n3, assign11270_e10542_d_n4, assign11270_e10542_d_n5, assign11270_e10542_d_n6, assign11270_e10542_d_n7, assign11270_e10542_d_n8,) = {
    if ((locals.var_guard116 == 0.0) && (locals.var_guard117 != 0.0)) {
        let assign11270_e10540: f64 = (1.0 / locals.var_t4__blk111);
        (assign11270_e10540, (-(locals.var_t4__blk111_dn3 / (locals.var_t4__blk111 * locals.var_t4__blk111))), (-(locals.var_t4__blk111_dn4 / (locals.var_t4__blk111 * locals.var_t4__blk111))), (-(locals.var_t4__blk111_dn5 / (locals.var_t4__blk111 * locals.var_t4__blk111))), (-(locals.var_t4__blk111_dn6 / (locals.var_t4__blk111 * locals.var_t4__blk111))), (-(locals.var_t4__blk111_dn7 / (locals.var_t4__blk111 * locals.var_t4__blk111))), (-(locals.var_t4__blk111_dn8 / (locals.var_t4__blk111 * locals.var_t4__blk111))),)
    } else {
        (locals.var_t1__blk110, locals.var_t1__blk110_dn3, locals.var_t1__blk110_dn4, locals.var_t1__blk110_dn5, locals.var_t1__blk110_dn6, locals.var_t1__blk110_dn7, locals.var_t1__blk110_dn8,)
    }
};
        locals.var_t1__blk110 = assign11270_e10542;
        locals.var_t1__blk110_dn3 = assign11270_e10542_d_n3;
        locals.var_t1__blk110_dn4 = assign11270_e10542_d_n4;
        locals.var_t1__blk110_dn5 = assign11270_e10542_d_n5;
        locals.var_t1__blk110_dn6 = assign11270_e10542_d_n6;
        locals.var_t1__blk110_dn7 = assign11270_e10542_d_n7;
        locals.var_t1__blk110_dn8 = assign11270_e10542_d_n8;
        locals.var_t1__blk110_rv = 0.0;

        let (assign11280_e10557, assign11280_e10557_d_n3, assign11280_e10557_d_n4, assign11280_e10557_d_n5, assign11280_e10557_d_n6, assign11280_e10557_d_n7, assign11280_e10557_d_n8,) = {
    if ((locals.var_guard116 == 0.0) && (locals.var_guard117 != 0.0)) {
        let assign11280_e10551: f64 = (locals.var_vbgd + locals.var_vbgs);
        let assign11280_e10552: f64 = (0.5 * assign11280_e10551);
        let assign11280_e10554: f64 = (assign11280_e10552 * locals.var_prwb_i);
        let assign11280_e10555: f64 = (locals.var_t1__blk110 - assign11280_e10554);
        (assign11280_e10555, (locals.var_t1__blk110_dn3 - ((0.5 * (locals.var_vbgd_dn3 + locals.var_vbgs_dn3)) * locals.var_prwb_i)), locals.var_t1__blk110_dn4, (locals.var_t1__blk110_dn5 - ((0.5 * (locals.var_vbgd_dn5 + locals.var_vbgs_dn5)) * locals.var_prwb_i)), (locals.var_t1__blk110_dn6 - ((0.5 * (locals.var_vbgd_dn6 + locals.var_vbgs_dn6)) * locals.var_prwb_i)), locals.var_t1__blk110_dn7, locals.var_t1__blk110_dn8,)
    } else {
        (locals.var_t1__blk110, locals.var_t1__blk110_dn3, locals.var_t1__blk110_dn4, locals.var_t1__blk110_dn5, locals.var_t1__blk110_dn6, locals.var_t1__blk110_dn7, locals.var_t1__blk110_dn8,)
    }
};
        locals.var_t1__blk110 = assign11280_e10557;
        locals.var_t1__blk110_dn3 = assign11280_e10557_d_n3;
        locals.var_t1__blk110_dn4 = assign11280_e10557_d_n4;
        locals.var_t1__blk110_dn5 = assign11280_e10557_d_n5;
        locals.var_t1__blk110_dn6 = assign11280_e10557_d_n6;
        locals.var_t1__blk110_dn7 = assign11280_e10557_d_n7;
        locals.var_t1__blk110_dn8 = assign11280_e10557_d_n8;
        locals.var_t1__blk110_rv = 0.0;

        let (assign11290_e10573, assign11290_e10573_d_n3, assign11290_e10573_d_n4, assign11290_e10573_d_n5, assign11290_e10573_d_n6, assign11290_e10573_d_n7, assign11290_e10573_d_n8,) = {
    if ((locals.var_guard116 == 0.0) && (locals.var_guard117 != 0.0)) {
        let assign11290_e10566: f64 = (locals.var_t1__blk110 * locals.var_t1__blk110);
        let assign11290_e10568: f64 = (assign11290_e10566 + 0.01);
        let assign11290_e10569: f64 = (assign11290_e10568).sqrt();
        let assign11290_e10570: f64 = (locals.var_t1__blk110 + assign11290_e10569);
        let assign11290_e10571: f64 = (0.5 * assign11290_e10570);
        (assign11290_e10571, (0.5 * (locals.var_t1__blk110_dn3 + (((locals.var_t1__blk110_dn3 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn3)) / (2.0 * assign11290_e10569)))), (0.5 * (locals.var_t1__blk110_dn4 + (((locals.var_t1__blk110_dn4 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn4)) / (2.0 * assign11290_e10569)))), (0.5 * (locals.var_t1__blk110_dn5 + (((locals.var_t1__blk110_dn5 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn5)) / (2.0 * assign11290_e10569)))), (0.5 * (locals.var_t1__blk110_dn6 + (((locals.var_t1__blk110_dn6 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn6)) / (2.0 * assign11290_e10569)))), (0.5 * (locals.var_t1__blk110_dn7 + (((locals.var_t1__blk110_dn7 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn7)) / (2.0 * assign11290_e10569)))), (0.5 * (locals.var_t1__blk110_dn8 + (((locals.var_t1__blk110_dn8 * locals.var_t1__blk110) + (locals.var_t1__blk110 * locals.var_t1__blk110_dn8)) / (2.0 * assign11290_e10569)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    }
};
        locals.var_t0 = assign11290_e10573;
        locals.var_t0_dn3 = assign11290_e10573_d_n3;
        locals.var_t0_dn4 = assign11290_e10573_d_n4;
        locals.var_t0_dn5 = assign11290_e10573_d_n5;
        locals.var_t0_dn6 = assign11290_e10573_d_n6;
        locals.var_t0_dn7 = assign11290_e10573_d_n7;
        locals.var_t0_dn8 = assign11290_e10573_d_n8;
        locals.var_t0_rv = 0.0;

        let (assign11300_e10592, assign11300_e10592_d_n3, assign11300_e10592_d_n4, assign11300_e10592_d_n5, assign11300_e10592_d_n6, assign11300_e10592_d_n7, assign11300_e10592_d_n8,) = {
    if ((locals.var_guard116 == 0.0) && (locals.var_guard117 != 0.0)) {
        let assign11300_e10581: f64 = (locals.var_rsourcegeo + locals.var_rdraingeo);
        let assign11300_e10583: f64 = (assign11300_e10581 + locals.var_rdswmin_i);
        let assign11300_e10586: f64 = (locals.var_rdsw_i * locals.var_t0);
        let assign11300_e10587: f64 = (assign11300_e10583 + assign11300_e10586);
        let assign11300_e10588: f64 = (locals.var_rdstemp * assign11300_e10587);
        let assign11300_e10590: f64 = (assign11300_e10588 * locals.var_weffwrfactor);
        (assign11300_e10590, ((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t0_dn3)) * locals.var_weffwrfactor), (((locals.var_rdstemp_dn4 * assign11300_e10587) + (locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t0_dn4))) * locals.var_weffwrfactor), ((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t0_dn5)) * locals.var_weffwrfactor), ((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t0_dn6)) * locals.var_weffwrfactor), ((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t0_dn7)) * locals.var_weffwrfactor), ((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t0_dn8)) * locals.var_weffwrfactor),)
    } else {
        (locals.var_rdsi, locals.var_rdsi_dn3, locals.var_rdsi_dn4, locals.var_rdsi_dn5, locals.var_rdsi_dn6, locals.var_rdsi_dn7, locals.var_rdsi_dn8,)
    }
};
        locals.var_rdsi = assign11300_e10592;
        locals.var_rdsi_dn3 = assign11300_e10592_d_n3;
        locals.var_rdsi_dn4 = assign11300_e10592_d_n4;
        locals.var_rdsi_dn5 = assign11300_e10592_d_n5;
        locals.var_rdsi_dn6 = assign11300_e10592_d_n6;
        locals.var_rdsi_dn7 = assign11300_e10592_d_n7;
        locals.var_rdsi_dn8 = assign11300_e10592_d_n8;
        locals.var_rdsi_rv = 0.0;

        let (assign11310_e10609, assign11310_e10609_d_n3, assign11310_e10609_d_n4, assign11310_e10609_d_n5, assign11310_e10609_d_n6, assign11310_e10609_d_n7, assign11310_e10609_d_n8,) = {
    if ((locals.var_guard116 == 0.0) && (locals.var_guard117 != 0.0)) {
        let assign11310_e10600: f64 = (p.p2 * locals.var_beta);
        let assign11310_e10602: f64 = (assign11310_e10600 * locals.var_ids0_ov_dqi);
        let assign11310_e10604: f64 = (assign11310_e10602 / locals.var_dvsat);
        let assign11310_e10606: f64 = (assign11310_e10604 * locals.var_rdsi);
        let assign11310_e10607: f64 = (1.0 + assign11310_e10606);
        (assign11310_e10607, ((((((((p.p2 * locals.var_beta_dn3) * locals.var_ids0_ov_dqi) + (assign11310_e10600 * locals.var_ids0_ov_dqi_dn3)) * locals.var_dvsat) - (assign11310_e10602 * locals.var_dvsat_dn3)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_rdsi) + (assign11310_e10604 * locals.var_rdsi_dn3)), ((((((((p.p2 * locals.var_beta_dn4) * locals.var_ids0_ov_dqi) + (assign11310_e10600 * locals.var_ids0_ov_dqi_dn4)) * locals.var_dvsat) - (assign11310_e10602 * locals.var_dvsat_dn4)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_rdsi) + (assign11310_e10604 * locals.var_rdsi_dn4)), ((((((((p.p2 * locals.var_beta_dn5) * locals.var_ids0_ov_dqi) + (assign11310_e10600 * locals.var_ids0_ov_dqi_dn5)) * locals.var_dvsat) - (assign11310_e10602 * locals.var_dvsat_dn5)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_rdsi) + (assign11310_e10604 * locals.var_rdsi_dn5)), ((((((((p.p2 * locals.var_beta_dn6) * locals.var_ids0_ov_dqi) + (assign11310_e10600 * locals.var_ids0_ov_dqi_dn6)) * locals.var_dvsat) - (assign11310_e10602 * locals.var_dvsat_dn6)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_rdsi) + (assign11310_e10604 * locals.var_rdsi_dn6)), ((((((((p.p2 * locals.var_beta_dn7) * locals.var_ids0_ov_dqi) + (assign11310_e10600 * locals.var_ids0_ov_dqi_dn7)) * locals.var_dvsat) - (assign11310_e10602 * locals.var_dvsat_dn7)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_rdsi) + (assign11310_e10604 * locals.var_rdsi_dn7)), ((((((((p.p2 * locals.var_beta_dn8) * locals.var_ids0_ov_dqi) + (assign11310_e10600 * locals.var_ids0_ov_dqi_dn8)) * locals.var_dvsat) - (assign11310_e10602 * locals.var_dvsat_dn8)) / (locals.var_dvsat * locals.var_dvsat)) * locals.var_rdsi) + (assign11310_e10604 * locals.var_rdsi_dn8)),)
    } else {
        (locals.var_dr, locals.var_dr_dn3, locals.var_dr_dn4, locals.var_dr_dn5, locals.var_dr_dn6, locals.var_dr_dn7, locals.var_dr_dn8,)
    }
};
        locals.var_dr = assign11310_e10609;
        locals.var_dr_dn3 = assign11310_e10609_d_n3;
        locals.var_dr_dn4 = assign11310_e10609_d_n4;
        locals.var_dr_dn5 = assign11310_e10609_d_n5;
        locals.var_dr_dn6 = assign11310_e10609_d_n6;
        locals.var_dr_dn7 = assign11310_e10609_d_n7;
        locals.var_dr_dn8 = assign11310_e10609_d_n8;
        locals.var_dr_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_26(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let assign11340_e10626: f64 = (locals.var_beta / locals.var_cox1);
        let assign11340_e10628: f64 = (assign11340_e10626 * locals.var_ids0);
        let assign11340_e10630: f64 = (assign11340_e10628 * locals.var_moc);
        let assign11340_e10632: f64 = (assign11340_e10630 * locals.var_mnud);
        let assign11340_e10635: f64 = (locals.var_dvsat * locals.var_dr);
        let assign11340_e10636: f64 = (assign11340_e10632 / assign11340_e10635);
        locals.var_ids = assign11340_e10636;
        locals.var_ids_dn3 = ((((((((((locals.var_beta_dn3 / locals.var_cox1) * locals.var_ids0) + (assign11340_e10626 * locals.var_ids0_dn3)) * locals.var_moc) + (assign11340_e10628 * locals.var_moc_dn3)) * locals.var_mnud) + (assign11340_e10630 * locals.var_mnud_dn3)) * assign11340_e10635) - (assign11340_e10632 * ((locals.var_dvsat_dn3 * locals.var_dr) + (locals.var_dvsat * locals.var_dr_dn3)))) / (assign11340_e10635 * assign11340_e10635));
        locals.var_ids_dn4 = ((((((((((locals.var_beta_dn4 / locals.var_cox1) * locals.var_ids0) + (assign11340_e10626 * locals.var_ids0_dn4)) * locals.var_moc) + (assign11340_e10628 * locals.var_moc_dn4)) * locals.var_mnud) + (assign11340_e10630 * locals.var_mnud_dn4)) * assign11340_e10635) - (assign11340_e10632 * ((locals.var_dvsat_dn4 * locals.var_dr) + (locals.var_dvsat * locals.var_dr_dn4)))) / (assign11340_e10635 * assign11340_e10635));
        locals.var_ids_dn5 = ((((((((((locals.var_beta_dn5 / locals.var_cox1) * locals.var_ids0) + (assign11340_e10626 * locals.var_ids0_dn5)) * locals.var_moc) + (assign11340_e10628 * locals.var_moc_dn5)) * locals.var_mnud) + (assign11340_e10630 * locals.var_mnud_dn5)) * assign11340_e10635) - (assign11340_e10632 * ((locals.var_dvsat_dn5 * locals.var_dr) + (locals.var_dvsat * locals.var_dr_dn5)))) / (assign11340_e10635 * assign11340_e10635));
        locals.var_ids_dn6 = ((((((((((locals.var_beta_dn6 / locals.var_cox1) * locals.var_ids0) + (assign11340_e10626 * locals.var_ids0_dn6)) * locals.var_moc) + (assign11340_e10628 * locals.var_moc_dn6)) * locals.var_mnud) + (assign11340_e10630 * locals.var_mnud_dn6)) * assign11340_e10635) - (assign11340_e10632 * ((locals.var_dvsat_dn6 * locals.var_dr) + (locals.var_dvsat * locals.var_dr_dn6)))) / (assign11340_e10635 * assign11340_e10635));
        locals.var_ids_dn7 = ((((((((((locals.var_beta_dn7 / locals.var_cox1) * locals.var_ids0) + (assign11340_e10626 * locals.var_ids0_dn7)) * locals.var_moc) + (assign11340_e10628 * locals.var_moc_dn7)) * locals.var_mnud) + (assign11340_e10630 * locals.var_mnud_dn7)) * assign11340_e10635) - (assign11340_e10632 * ((locals.var_dvsat_dn7 * locals.var_dr) + (locals.var_dvsat * locals.var_dr_dn7)))) / (assign11340_e10635 * assign11340_e10635));
        locals.var_ids_dn8 = ((((((((((locals.var_beta_dn8 / locals.var_cox1) * locals.var_ids0) + (assign11340_e10626 * locals.var_ids0_dn8)) * locals.var_moc) + (assign11340_e10628 * locals.var_moc_dn8)) * locals.var_mnud) + (assign11340_e10630 * locals.var_mnud_dn8)) * assign11340_e10635) - (assign11340_e10632 * ((locals.var_dvsat_dn8 * locals.var_dr) + (locals.var_dvsat * locals.var_dr_dn8)))) / (assign11340_e10635 * assign11340_e10635));
        locals.var_ids_rv = 0.0;

        let assign11350_e10639: f64 = (p.p2 * locals.var_ids);
        locals.var_ids = assign11350_e10639;
        locals.var_ids_dn3 = (p.p2 * locals.var_ids_dn3);
        locals.var_ids_dn4 = (p.p2 * locals.var_ids_dn4);
        locals.var_ids_dn5 = (p.p2 * locals.var_ids_dn5);
        locals.var_ids_dn6 = (p.p2 * locals.var_ids_dn6);
        locals.var_ids_dn7 = (p.p2 * locals.var_ids_dn7);
        locals.var_ids_dn8 = (p.p2 * locals.var_ids_dn8);
        locals.var_ids_rv = 0.0;

        let assign11360_e10642: f64 = (locals.var_qfrontd + locals.var_qfronts);
        let assign11360_e10644: f64 = (assign11360_e10642 / 2.0);
        locals.var_qfg = assign11360_e10644;
        locals.var_qfg_dn3 = ((locals.var_qfrontd_dn3 + locals.var_qfronts_dn3) / 2.0);
        locals.var_qfg_dn4 = ((locals.var_qfrontd_dn4 + locals.var_qfronts_dn4) / 2.0);
        locals.var_qfg_dn5 = ((locals.var_qfrontd_dn5 + locals.var_qfronts_dn5) / 2.0);
        locals.var_qfg_dn6 = ((locals.var_qfrontd_dn6 + locals.var_qfronts_dn6) / 2.0);
        locals.var_qfg_dn7 = ((locals.var_qfrontd_dn7 + locals.var_qfronts_dn7) / 2.0);
        locals.var_qfg_dn8 = ((locals.var_qfrontd_dn8 + locals.var_qfronts_dn8) / 2.0);
        locals.var_qfg_rv = 0.0;

        let assign11370_e10647: f64 = (1.0 / 6.0);
        let assign11370_e10651: f64 = (2.0 * locals.var_qtotd);
        let assign11370_e10652: f64 = (locals.var_qtots + assign11370_e10651);
        let assign11370_e10653: f64 = (assign11370_e10647 * assign11370_e10652);
        locals.var_qd = assign11370_e10653;
        locals.var_qd_dn3 = (assign11370_e10647 * (locals.var_qtots_dn3 + (2.0 * locals.var_qtotd_dn3)));
        locals.var_qd_dn4 = (assign11370_e10647 * (locals.var_qtots_dn4 + (2.0 * locals.var_qtotd_dn4)));
        locals.var_qd_dn5 = (assign11370_e10647 * (locals.var_qtots_dn5 + (2.0 * locals.var_qtotd_dn5)));
        locals.var_qd_dn6 = (assign11370_e10647 * (locals.var_qtots_dn6 + (2.0 * locals.var_qtotd_dn6)));
        locals.var_qd_dn7 = (assign11370_e10647 * (locals.var_qtots_dn7 + (2.0 * locals.var_qtotd_dn7)));
        locals.var_qd_dn8 = (assign11370_e10647 * (locals.var_qtots_dn8 + (2.0 * locals.var_qtotd_dn8)));
        locals.var_qd_rv = 0.0;

        let assign11380_e10656: f64 = (1.0 / 6.0);
        let assign11380_e10659: f64 = (2.0 * locals.var_qtots);
        let assign11380_e10661: f64 = (assign11380_e10659 + locals.var_qtotd);
        let assign11380_e10662: f64 = (assign11380_e10656 * assign11380_e10661);
        locals.var_qs = assign11380_e10662;
        locals.var_qs_dn3 = (assign11380_e10656 * ((2.0 * locals.var_qtots_dn3) + locals.var_qtotd_dn3));
        locals.var_qs_dn4 = (assign11380_e10656 * ((2.0 * locals.var_qtots_dn4) + locals.var_qtotd_dn4));
        locals.var_qs_dn5 = (assign11380_e10656 * ((2.0 * locals.var_qtots_dn5) + locals.var_qtotd_dn5));
        locals.var_qs_dn6 = (assign11380_e10656 * ((2.0 * locals.var_qtots_dn6) + locals.var_qtotd_dn6));
        locals.var_qs_dn7 = (assign11380_e10656 * ((2.0 * locals.var_qtots_dn7) + locals.var_qtotd_dn7));
        locals.var_qs_dn8 = (assign11380_e10656 * ((2.0 * locals.var_qtots_dn8) + locals.var_qtotd_dn8));
        locals.var_qs_rv = 0.0;

        let assign11390_e10665: f64 = (locals.var_qbackd + locals.var_qbacks);
        let assign11390_e10667: f64 = (assign11390_e10665 / 2.0);
        locals.var_qbg = assign11390_e10667;
        locals.var_qbg_dn3 = ((locals.var_qbackd_dn3 + locals.var_qbacks_dn3) / 2.0);
        locals.var_qbg_dn4 = ((locals.var_qbackd_dn4 + locals.var_qbacks_dn4) / 2.0);
        locals.var_qbg_dn5 = ((locals.var_qbackd_dn5 + locals.var_qbacks_dn5) / 2.0);
        locals.var_qbg_dn6 = ((locals.var_qbackd_dn6 + locals.var_qbacks_dn6) / 2.0);
        locals.var_qbg_dn7 = ((locals.var_qbackd_dn7 + locals.var_qbacks_dn7) / 2.0);
        locals.var_qbg_dn8 = ((locals.var_qbackd_dn8 + locals.var_qbacks_dn8) / 2.0);
        locals.var_qbg_rv = 0.0;

        let assign11400_e10670: f64 = if locals.var_qmtcencv_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard118 = assign11400_e10670;
        locals.var_guard118_rv = 0.0;

        let (assign11410_e10680, assign11410_e10680_d_n3, assign11410_e10680_d_n4, assign11410_e10680_d_n5, assign11410_e10680_d_n6, assign11410_e10680_d_n7, assign11410_e10680_d_n8,) = {
    if (locals.var_guard118 != 0.0) {
        let assign11410_e10675: f64 = (locals.var_etaqm_i * locals.var_qba);
        let assign11410_e10676: f64 = (locals.var_qia + assign11410_e10675);
        let assign11410_e10678: f64 = (assign11410_e10676 / locals.var_qm0_i);
        (assign11410_e10678, (locals.var_qia_dn3 / locals.var_qm0_i), (locals.var_qia_dn4 / locals.var_qm0_i), (locals.var_qia_dn5 / locals.var_qm0_i), (locals.var_qia_dn6 / locals.var_qm0_i), (locals.var_qia_dn7 / locals.var_qm0_i), (locals.var_qia_dn8 / locals.var_qm0_i),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8,)
    }
};
        locals.var_t4 = assign11410_e10680;
        locals.var_t4_dn3 = assign11410_e10680_d_n3;
        locals.var_t4_dn4 = assign11410_e10680_d_n4;
        locals.var_t4_dn5 = assign11410_e10680_d_n5;
        locals.var_t4_dn6 = assign11410_e10680_d_n6;
        locals.var_t4_dn7 = assign11410_e10680_d_n7;
        locals.var_t4_dn8 = assign11410_e10680_d_n8;
        locals.var_t4_rv = 0.0;

        let (assign11420_e10688, assign11420_e10688_d_n3, assign11420_e10688_d_n4, assign11420_e10688_d_n5, assign11420_e10688_d_n6, assign11420_e10688_d_n7, assign11420_e10688_d_n8,) = {
    if (locals.var_guard118 != 0.0) {
        let assign11420_e10685: f64 = (locals.var_t4).powf(locals.var_pqm_i);
        let assign11420_e10686: f64 = (1.0 + assign11420_e10685);
        (assign11420_e10686, if 0.0 == 0.0 && ((locals.var_pqm_i) as f64).is_finite() && ((locals.var_pqm_i) as f64).fract() == 0.0 { if locals.var_pqm_i == 0.0 { 0.0 } else { (locals.var_pqm_i * ((locals.var_t4).powf(locals.var_pqm_i - 1.0) * locals.var_t4_dn3)) } } else { (assign11420_e10685 * (locals.var_pqm_i * (locals.var_t4_dn3 / locals.var_t4))) }, if 0.0 == 0.0 && ((locals.var_pqm_i) as f64).is_finite() && ((locals.var_pqm_i) as f64).fract() == 0.0 { if locals.var_pqm_i == 0.0 { 0.0 } else { (locals.var_pqm_i * ((locals.var_t4).powf(locals.var_pqm_i - 1.0) * locals.var_t4_dn4)) } } else { (assign11420_e10685 * (locals.var_pqm_i * (locals.var_t4_dn4 / locals.var_t4))) }, if 0.0 == 0.0 && ((locals.var_pqm_i) as f64).is_finite() && ((locals.var_pqm_i) as f64).fract() == 0.0 { if locals.var_pqm_i == 0.0 { 0.0 } else { (locals.var_pqm_i * ((locals.var_t4).powf(locals.var_pqm_i - 1.0) * locals.var_t4_dn5)) } } else { (assign11420_e10685 * (locals.var_pqm_i * (locals.var_t4_dn5 / locals.var_t4))) }, if 0.0 == 0.0 && ((locals.var_pqm_i) as f64).is_finite() && ((locals.var_pqm_i) as f64).fract() == 0.0 { if locals.var_pqm_i == 0.0 { 0.0 } else { (locals.var_pqm_i * ((locals.var_t4).powf(locals.var_pqm_i - 1.0) * locals.var_t4_dn6)) } } else { (assign11420_e10685 * (locals.var_pqm_i * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((locals.var_pqm_i) as f64).is_finite() && ((locals.var_pqm_i) as f64).fract() == 0.0 { if locals.var_pqm_i == 0.0 { 0.0 } else { (locals.var_pqm_i * ((locals.var_t4).powf(locals.var_pqm_i - 1.0) * locals.var_t4_dn7)) } } else { (assign11420_e10685 * (locals.var_pqm_i * (locals.var_t4_dn7 / locals.var_t4))) }, if 0.0 == 0.0 && ((locals.var_pqm_i) as f64).is_finite() && ((locals.var_pqm_i) as f64).fract() == 0.0 { if locals.var_pqm_i == 0.0 { 0.0 } else { (locals.var_pqm_i * ((locals.var_t4).powf(locals.var_pqm_i - 1.0) * locals.var_t4_dn8)) } } else { (assign11420_e10685 * (locals.var_pqm_i * (locals.var_t4_dn8 / locals.var_t4))) },)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8,)
    }
};
        locals.var_t5 = assign11420_e10688;
        locals.var_t5_dn3 = assign11420_e10688_d_n3;
        locals.var_t5_dn4 = assign11420_e10688_d_n4;
        locals.var_t5_dn5 = assign11420_e10688_d_n5;
        locals.var_t5_dn6 = assign11420_e10688_d_n6;
        locals.var_t5_dn7 = assign11420_e10688_d_n7;
        locals.var_t5_dn8 = assign11420_e10688_d_n8;
        locals.var_t5_rv = 0.0;

        let (assign11430_e10692,) = {
    if (locals.var_guard118 != 0.0) {
        (p.p49,)
    } else {
        (locals.var_tcen0,)
    }
};
        locals.var_tcen0 = assign11430_e10692;
        locals.var_tcen0_rv = 0.0;

        let (assign11440_e10698, assign11440_e10698_d_n3, assign11440_e10698_d_n4, assign11440_e10698_d_n5, assign11440_e10698_d_n6, assign11440_e10698_d_n7, assign11440_e10698_d_n8,) = {
    if (locals.var_guard118 != 0.0) {
        let assign11440_e10696: f64 = (locals.var_tcen0 / locals.var_t5);
        (assign11440_e10696, (-((locals.var_tcen0 * locals.var_t5_dn3) / (locals.var_t5 * locals.var_t5))), (-((locals.var_tcen0 * locals.var_t5_dn4) / (locals.var_t5 * locals.var_t5))), (-((locals.var_tcen0 * locals.var_t5_dn5) / (locals.var_t5 * locals.var_t5))), (-((locals.var_tcen0 * locals.var_t5_dn6) / (locals.var_t5 * locals.var_t5))), (-((locals.var_tcen0 * locals.var_t5_dn7) / (locals.var_t5 * locals.var_t5))), (-((locals.var_tcen0 * locals.var_t5_dn8) / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_tcen, locals.var_tcen_dn3, locals.var_tcen_dn4, locals.var_tcen_dn5, locals.var_tcen_dn6, locals.var_tcen_dn7, locals.var_tcen_dn8,)
    }
};
        locals.var_tcen = assign11440_e10698;
        locals.var_tcen_dn3 = assign11440_e10698_d_n3;
        locals.var_tcen_dn4 = assign11440_e10698_d_n4;
        locals.var_tcen_dn5 = assign11440_e10698_d_n5;
        locals.var_tcen_dn6 = assign11440_e10698_d_n6;
        locals.var_tcen_dn7 = assign11440_e10698_d_n7;
        locals.var_tcen_dn8 = assign11440_e10698_d_n8;
        locals.var_tcen_rv = 0.0;

        let (assign11450_e10716, assign11450_e10716_d_n3, assign11450_e10716_d_n4, assign11450_e10716_d_n5, assign11450_e10716_d_n6, assign11450_e10716_d_n7, assign11450_e10716_d_n8,) = {
    if (locals.var_guard118 != 0.0) {
        let assign11450_e10702: f64 = (3.9 * 8.85418e-12);
        let assign11450_e10705: f64 = (locals.var_imgtoxp * 3.9);
        let assign11450_e10707: f64 = (assign11450_e10705 / p.p60);
        let assign11450_e10710: f64 = (locals.var_tcen * locals.var_qmtcencv_i);
        let assign11450_e10712: f64 = (assign11450_e10710 / locals.var_epsratio);
        let assign11450_e10713: f64 = (assign11450_e10707 + assign11450_e10712);
        let assign11450_e10714: f64 = (assign11450_e10702 / assign11450_e10713);
        (assign11450_e10714, (-((assign11450_e10702 * ((locals.var_tcen_dn3 * locals.var_qmtcencv_i) / locals.var_epsratio)) / (assign11450_e10713 * assign11450_e10713))), (-((assign11450_e10702 * ((locals.var_tcen_dn4 * locals.var_qmtcencv_i) / locals.var_epsratio)) / (assign11450_e10713 * assign11450_e10713))), (-((assign11450_e10702 * ((locals.var_tcen_dn5 * locals.var_qmtcencv_i) / locals.var_epsratio)) / (assign11450_e10713 * assign11450_e10713))), (-((assign11450_e10702 * ((locals.var_tcen_dn6 * locals.var_qmtcencv_i) / locals.var_epsratio)) / (assign11450_e10713 * assign11450_e10713))), (-((assign11450_e10702 * ((locals.var_tcen_dn7 * locals.var_qmtcencv_i) / locals.var_epsratio)) / (assign11450_e10713 * assign11450_e10713))), (-((assign11450_e10702 * ((locals.var_tcen_dn8 * locals.var_qmtcencv_i) / locals.var_epsratio)) / (assign11450_e10713 * assign11450_e10713))),)
    } else {
        (locals.var_coxeff, locals.var_coxeff_dn3, locals.var_coxeff_dn4, locals.var_coxeff_dn5, locals.var_coxeff_dn6, locals.var_coxeff_dn7, locals.var_coxeff_dn8,)
    }
};
        locals.var_coxeff = assign11450_e10716;
        locals.var_coxeff_dn3 = assign11450_e10716_d_n3;
        locals.var_coxeff_dn4 = assign11450_e10716_d_n4;
        locals.var_coxeff_dn5 = assign11450_e10716_d_n5;
        locals.var_coxeff_dn6 = assign11450_e10716_d_n6;
        locals.var_coxeff_dn7 = assign11450_e10716_d_n7;
        locals.var_coxeff_dn8 = assign11450_e10716_d_n8;
        locals.var_coxeff_rv = 0.0;

        let (assign11460_e10721, assign11460_e10721_d_n3, assign11460_e10721_d_n4, assign11460_e10721_d_n5, assign11460_e10721_d_n6, assign11460_e10721_d_n7, assign11460_e10721_d_n8,) = {
    if (locals.var_guard118 == 0.0) {
        (locals.var_cox1p, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_coxeff, locals.var_coxeff_dn3, locals.var_coxeff_dn4, locals.var_coxeff_dn5, locals.var_coxeff_dn6, locals.var_coxeff_dn7, locals.var_coxeff_dn8,)
    }
};
        locals.var_coxeff = assign11460_e10721;
        locals.var_coxeff_dn3 = assign11460_e10721_d_n3;
        locals.var_coxeff_dn4 = assign11460_e10721_d_n4;
        locals.var_coxeff_dn5 = assign11460_e10721_d_n5;
        locals.var_coxeff_dn6 = assign11460_e10721_d_n6;
        locals.var_coxeff_dn7 = assign11460_e10721_d_n7;
        locals.var_coxeff_dn8 = assign11460_e10721_d_n8;
        locals.var_coxeff_rv = 0.0;

        let assign11470_e10724: f64 = (locals.var_weffcv * locals.var_leffcv);
        let assign11470_e10726: f64 = (assign11470_e10724 / locals.var_mclmcv);
        locals.var_t0 = assign11470_e10726;
        locals.var_t0_dn3 = (-((assign11470_e10724 * locals.var_mclmcv_dn3) / (locals.var_mclmcv * locals.var_mclmcv)));
        locals.var_t0_dn4 = (-((assign11470_e10724 * locals.var_mclmcv_dn4) / (locals.var_mclmcv * locals.var_mclmcv)));
        locals.var_t0_dn5 = (-((assign11470_e10724 * locals.var_mclmcv_dn5) / (locals.var_mclmcv * locals.var_mclmcv)));
        locals.var_t0_dn6 = (-((assign11470_e10724 * locals.var_mclmcv_dn6) / (locals.var_mclmcv * locals.var_mclmcv)));
        locals.var_t0_dn7 = (-((assign11470_e10724 * locals.var_mclmcv_dn7) / (locals.var_mclmcv * locals.var_mclmcv)));
        locals.var_t0_dn8 = (-((assign11470_e10724 * locals.var_mclmcv_dn8) / (locals.var_mclmcv * locals.var_mclmcv)));
        locals.var_t0_rv = 0.0;

        let assign11480_e10729: f64 = (locals.var_qfg * locals.var_t0);
        locals.var_qfg = assign11480_e10729;
        locals.var_qfg_dn3 = ((locals.var_qfg_dn3 * locals.var_t0) + (locals.var_qfg * locals.var_t0_dn3));
        locals.var_qfg_dn4 = ((locals.var_qfg_dn4 * locals.var_t0) + (locals.var_qfg * locals.var_t0_dn4));
        locals.var_qfg_dn5 = ((locals.var_qfg_dn5 * locals.var_t0) + (locals.var_qfg * locals.var_t0_dn5));
        locals.var_qfg_dn6 = ((locals.var_qfg_dn6 * locals.var_t0) + (locals.var_qfg * locals.var_t0_dn6));
        locals.var_qfg_dn7 = ((locals.var_qfg_dn7 * locals.var_t0) + (locals.var_qfg * locals.var_t0_dn7));
        locals.var_qfg_dn8 = ((locals.var_qfg_dn8 * locals.var_t0) + (locals.var_qfg * locals.var_t0_dn8));
        locals.var_qfg_rv = 0.0;

        let assign11490_e10731: f64 = (-locals.var_qd);
        let assign11490_e10733: f64 = (assign11490_e10731 * locals.var_t0);
        locals.var_qd = assign11490_e10733;
        locals.var_qd_dn3 = (((-locals.var_qd_dn3) * locals.var_t0) + (assign11490_e10731 * locals.var_t0_dn3));
        locals.var_qd_dn4 = (((-locals.var_qd_dn4) * locals.var_t0) + (assign11490_e10731 * locals.var_t0_dn4));
        locals.var_qd_dn5 = (((-locals.var_qd_dn5) * locals.var_t0) + (assign11490_e10731 * locals.var_t0_dn5));
        locals.var_qd_dn6 = (((-locals.var_qd_dn6) * locals.var_t0) + (assign11490_e10731 * locals.var_t0_dn6));
        locals.var_qd_dn7 = (((-locals.var_qd_dn7) * locals.var_t0) + (assign11490_e10731 * locals.var_t0_dn7));
        locals.var_qd_dn8 = (((-locals.var_qd_dn8) * locals.var_t0) + (assign11490_e10731 * locals.var_t0_dn8));
        locals.var_qd_rv = 0.0;

        let assign11500_e10736: f64 = (locals.var_qbg * locals.var_t0);
        locals.var_qbg = assign11500_e10736;
        locals.var_qbg_dn3 = ((locals.var_qbg_dn3 * locals.var_t0) + (locals.var_qbg * locals.var_t0_dn3));
        locals.var_qbg_dn4 = ((locals.var_qbg_dn4 * locals.var_t0) + (locals.var_qbg * locals.var_t0_dn4));
        locals.var_qbg_dn5 = ((locals.var_qbg_dn5 * locals.var_t0) + (locals.var_qbg * locals.var_t0_dn5));
        locals.var_qbg_dn6 = ((locals.var_qbg_dn6 * locals.var_t0) + (locals.var_qbg * locals.var_t0_dn6));
        locals.var_qbg_dn7 = ((locals.var_qbg_dn7 * locals.var_t0) + (locals.var_qbg * locals.var_t0_dn7));
        locals.var_qbg_dn8 = ((locals.var_qbg_dn8 * locals.var_t0) + (locals.var_qbg * locals.var_t0_dn8));
        locals.var_qbg_rv = 0.0;

        let assign11510_e10738: f64 = (-locals.var_qs);
        let assign11510_e10740: f64 = (assign11510_e10738 * locals.var_t0);
        locals.var_qs = assign11510_e10740;
        locals.var_qs_dn3 = (((-locals.var_qs_dn3) * locals.var_t0) + (assign11510_e10738 * locals.var_t0_dn3));
        locals.var_qs_dn4 = (((-locals.var_qs_dn4) * locals.var_t0) + (assign11510_e10738 * locals.var_t0_dn4));
        locals.var_qs_dn5 = (((-locals.var_qs_dn5) * locals.var_t0) + (assign11510_e10738 * locals.var_t0_dn5));
        locals.var_qs_dn6 = (((-locals.var_qs_dn6) * locals.var_t0) + (assign11510_e10738 * locals.var_t0_dn6));
        locals.var_qs_dn7 = (((-locals.var_qs_dn7) * locals.var_t0) + (assign11510_e10738 * locals.var_t0_dn7));
        locals.var_qs_dn8 = (((-locals.var_qs_dn8) * locals.var_t0) + (assign11510_e10738 * locals.var_t0_dn8));
        locals.var_qs_rv = 0.0;

        let assign11520_e10743: f64 = (locals.var_weffcv * locals.var_lovs_i);
        let assign11520_e10745: f64 = (assign11520_e10743 * locals.var_cox1);
        let assign11520_e10747: f64 = (assign11520_e10745 * (nv7 - nv6));
        locals.var_qfgs_ov = assign11520_e10747;
        locals.var_qfgs_ov_dn3 = 0.0;
        locals.var_qfgs_ov_dn4 = 0.0;
        locals.var_qfgs_ov_dn5 = 0.0;
        locals.var_qfgs_ov_dn6 = (-assign11520_e10745);
        locals.var_qfgs_ov_dn7 = assign11520_e10745;
        locals.var_qfgs_ov_dn8 = 0.0;
        locals.var_qfgs_ov_rv = 0.0;

        let assign11530_e10750: f64 = (locals.var_weffcv * locals.var_lovd_i);
        let assign11530_e10752: f64 = (assign11530_e10750 * locals.var_cox1);
        let assign11530_e10754: f64 = (assign11530_e10752 * (nv7 - nv5));
        locals.var_qfgd_ov = assign11530_e10754;
        locals.var_qfgd_ov_dn3 = 0.0;
        locals.var_qfgd_ov_dn4 = 0.0;
        locals.var_qfgd_ov_dn5 = (-assign11530_e10752);
        locals.var_qfgd_ov_dn6 = 0.0;
        locals.var_qfgd_ov_dn7 = assign11530_e10752;
        locals.var_qfgd_ov_dn8 = 0.0;
        locals.var_qfgd_ov_rv = 0.0;

        let assign11540_e10758: f64 = (locals.var_phig2_i - locals.var_phisd);
        let assign11540_e10759: f64 = (locals.var_devsign * assign11540_e10758);
        locals.var_vfbsd_bg = assign11540_e10759;
        locals.var_vfbsd_bg_dn3 = (locals.var_devsign * (locals.var_phig2_i_dn3 - locals.var_phisd_dn3));
        locals.var_vfbsd_bg_dn4 = (locals.var_devsign * (locals.var_phig2_i_dn4 - locals.var_phisd_dn4));
        locals.var_vfbsd_bg_dn5 = (locals.var_devsign * (locals.var_phig2_i_dn5 - locals.var_phisd_dn5));
        locals.var_vfbsd_bg_dn6 = (locals.var_devsign * (locals.var_phig2_i_dn6 - locals.var_phisd_dn6));
        locals.var_vfbsd_bg_dn7 = (locals.var_devsign * (locals.var_phig2_i_dn7 - locals.var_phisd_dn7));
        locals.var_vfbsd_bg_dn8 = (locals.var_devsign * (locals.var_phig2_i_dn8 - locals.var_phisd_dn8));
        locals.var_vfbsd_bg_rv = 0.0;

        let assign11550_e10762: f64 = (locals.var_vgs_ov_noswap - locals.var_vfbsd);
        let assign11550_e10764: f64 = (assign11550_e10762 + 0.02);
        let assign11550_e10767: f64 = (p.p45 / p.p46);
        let assign11550_e10770: f64 = (locals.var_vbgs_noswap - locals.var_vfbsd_bg);
        let assign11550_e10772: f64 = (assign11550_e10770 - p.p268);
        let assign11550_e10773: f64 = (assign11550_e10767 * assign11550_e10772);
        let assign11550_e10775: f64 = (assign11550_e10773 * p.p269);
        let assign11550_e10776: f64 = (assign11550_e10764 + assign11550_e10775);
        locals.var_t0 = assign11550_e10776;
        locals.var_t0_dn3 = ((-locals.var_vfbsd_dn3) + ((assign11550_e10767 * (locals.var_vbgs_noswap_dn3 - locals.var_vfbsd_bg_dn3)) * p.p269));
        locals.var_t0_dn4 = ((-locals.var_vfbsd_dn4) + ((assign11550_e10767 * (-locals.var_vfbsd_bg_dn4)) * p.p269));
        locals.var_t0_dn5 = ((-locals.var_vfbsd_dn5) + ((assign11550_e10767 * (-locals.var_vfbsd_bg_dn5)) * p.p269));
        locals.var_t0_dn6 = ((locals.var_vgs_ov_noswap_dn6 - locals.var_vfbsd_dn6) + ((assign11550_e10767 * (locals.var_vbgs_noswap_dn6 - locals.var_vfbsd_bg_dn6)) * p.p269));
        locals.var_t0_dn7 = ((locals.var_vgs_ov_noswap_dn7 - locals.var_vfbsd_dn7) + ((assign11550_e10767 * (-locals.var_vfbsd_bg_dn7)) * p.p269));
        locals.var_t0_dn8 = ((-locals.var_vfbsd_dn8) + ((assign11550_e10767 * (-locals.var_vfbsd_bg_dn8)) * p.p269));
        locals.var_t0_rv = 0.0;

        let assign11560_e10781: f64 = (locals.var_t0 * locals.var_t0);
        let assign11560_e10784: f64 = (4.0 * 0.02);
        let assign11560_e10785: f64 = (assign11560_e10781 + assign11560_e10784);
        let assign11560_e10786: f64 = (assign11560_e10785).sqrt();
        let assign11560_e10787: f64 = (locals.var_t0 - assign11560_e10786);
        let assign11560_e10788: f64 = (0.5 * assign11560_e10787);
        locals.var_vfgs_ov = assign11560_e10788;
        locals.var_vfgs_ov_dn3 = (0.5 * (locals.var_t0_dn3 - (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign11560_e10786))));
        locals.var_vfgs_ov_dn4 = (0.5 * (locals.var_t0_dn4 - (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign11560_e10786))));
        locals.var_vfgs_ov_dn5 = (0.5 * (locals.var_t0_dn5 - (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign11560_e10786))));
        locals.var_vfgs_ov_dn6 = (0.5 * (locals.var_t0_dn6 - (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign11560_e10786))));
        locals.var_vfgs_ov_dn7 = (0.5 * (locals.var_t0_dn7 - (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign11560_e10786))));
        locals.var_vfgs_ov_dn8 = (0.5 * (locals.var_t0_dn8 - (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign11560_e10786))));
        locals.var_vfgs_ov_rv = 0.0;

        let assign11570_e10791: f64 = (locals.var_vgs_ov_noswap - locals.var_vfbsd);
        let assign11570_e10793: f64 = (assign11570_e10791 - locals.var_vfgs_ov);
        locals.var_t1 = assign11570_e10793;
        locals.var_t1_dn3 = ((-locals.var_vfbsd_dn3) - locals.var_vfgs_ov_dn3);
        locals.var_t1_dn4 = ((-locals.var_vfbsd_dn4) - locals.var_vfgs_ov_dn4);
        locals.var_t1_dn5 = ((-locals.var_vfbsd_dn5) - locals.var_vfgs_ov_dn5);
        locals.var_t1_dn6 = ((locals.var_vgs_ov_noswap_dn6 - locals.var_vfbsd_dn6) - locals.var_vfgs_ov_dn6);
        locals.var_t1_dn7 = ((locals.var_vgs_ov_noswap_dn7 - locals.var_vfbsd_dn7) - locals.var_vfgs_ov_dn7);
        locals.var_t1_dn8 = ((-locals.var_vfbsd_dn8) - locals.var_vfgs_ov_dn8);
        locals.var_t1_rv = 0.0;

        let assign11580_e10797: f64 = (locals.var_devsign * locals.var_weffcv);
        let assign11580_e10799: f64 = (assign11580_e10797 * p.p263);
        let assign11580_e10803: f64 = (0.5 * p.p265);
        let assign11580_e10807: f64 = (4.0 * locals.var_vfgs_ov);
        let assign11580_e10809: f64 = (assign11580_e10807 / p.p265);
        let assign11580_e10810: f64 = (1.0 - assign11580_e10809);
        let assign11580_e10811: f64 = (assign11580_e10810).sqrt();
        let assign11580_e10813: f64 = (assign11580_e10811 - 1.0);
        let assign11580_e10814: f64 = (assign11580_e10803 * assign11580_e10813);
        let assign11580_e10815: f64 = (locals.var_t1 - assign11580_e10814);
        let assign11580_e10816: f64 = (assign11580_e10799 * assign11580_e10815);
        let assign11580_e10817: f64 = (locals.var_qfgs_ov + assign11580_e10816);
        locals.var_qfgs_ov = assign11580_e10817;
        locals.var_qfgs_ov_dn3 = (locals.var_qfgs_ov_dn3 + (assign11580_e10799 * (locals.var_t1_dn3 - (assign11580_e10803 * ((-((4.0 * locals.var_vfgs_ov_dn3) / p.p265)) / (2.0 * assign11580_e10811))))));
        locals.var_qfgs_ov_dn4 = (locals.var_qfgs_ov_dn4 + (assign11580_e10799 * (locals.var_t1_dn4 - (assign11580_e10803 * ((-((4.0 * locals.var_vfgs_ov_dn4) / p.p265)) / (2.0 * assign11580_e10811))))));
        locals.var_qfgs_ov_dn5 = (locals.var_qfgs_ov_dn5 + (assign11580_e10799 * (locals.var_t1_dn5 - (assign11580_e10803 * ((-((4.0 * locals.var_vfgs_ov_dn5) / p.p265)) / (2.0 * assign11580_e10811))))));
        locals.var_qfgs_ov_dn6 = (locals.var_qfgs_ov_dn6 + (assign11580_e10799 * (locals.var_t1_dn6 - (assign11580_e10803 * ((-((4.0 * locals.var_vfgs_ov_dn6) / p.p265)) / (2.0 * assign11580_e10811))))));
        locals.var_qfgs_ov_dn7 = (locals.var_qfgs_ov_dn7 + (assign11580_e10799 * (locals.var_t1_dn7 - (assign11580_e10803 * ((-((4.0 * locals.var_vfgs_ov_dn7) / p.p265)) / (2.0 * assign11580_e10811))))));
        locals.var_qfgs_ov_dn8 = (locals.var_qfgs_ov_dn8 + (assign11580_e10799 * (locals.var_t1_dn8 - (assign11580_e10803 * ((-((4.0 * locals.var_vfgs_ov_dn8) / p.p265)) / (2.0 * assign11580_e10811))))));
        locals.var_qfgs_ov_rv = 0.0;

        let assign11590_e10820: f64 = (locals.var_vgd_ov_noswap - locals.var_vfbsd);
        let assign11590_e10822: f64 = (assign11590_e10820 + 0.02);
        let assign11590_e10825: f64 = (p.p45 / p.p46);
        let assign11590_e10828: f64 = (locals.var_vbgd_noswap - locals.var_vfbsd_bg);
        let assign11590_e10830: f64 = (assign11590_e10828 - p.p270);
        let assign11590_e10831: f64 = (assign11590_e10825 * assign11590_e10830);
        let assign11590_e10833: f64 = (assign11590_e10831 * p.p271);
        let assign11590_e10834: f64 = (assign11590_e10822 + assign11590_e10833);
        locals.var_t0 = assign11590_e10834;
        locals.var_t0_dn3 = ((-locals.var_vfbsd_dn3) + ((assign11590_e10825 * (locals.var_vbgd_noswap_dn3 - locals.var_vfbsd_bg_dn3)) * p.p271));
        locals.var_t0_dn4 = ((-locals.var_vfbsd_dn4) + ((assign11590_e10825 * (-locals.var_vfbsd_bg_dn4)) * p.p271));
        locals.var_t0_dn5 = ((locals.var_vgd_ov_noswap_dn5 - locals.var_vfbsd_dn5) + ((assign11590_e10825 * (locals.var_vbgd_noswap_dn5 - locals.var_vfbsd_bg_dn5)) * p.p271));
        locals.var_t0_dn6 = ((-locals.var_vfbsd_dn6) + ((assign11590_e10825 * (-locals.var_vfbsd_bg_dn6)) * p.p271));
        locals.var_t0_dn7 = ((locals.var_vgd_ov_noswap_dn7 - locals.var_vfbsd_dn7) + ((assign11590_e10825 * (-locals.var_vfbsd_bg_dn7)) * p.p271));
        locals.var_t0_dn8 = ((-locals.var_vfbsd_dn8) + ((assign11590_e10825 * (-locals.var_vfbsd_bg_dn8)) * p.p271));
        locals.var_t0_rv = 0.0;

        let assign11600_e10839: f64 = (locals.var_t0 * locals.var_t0);
        let assign11600_e10842: f64 = (4.0 * 0.02);
        let assign11600_e10843: f64 = (assign11600_e10839 + assign11600_e10842);
        let assign11600_e10844: f64 = (assign11600_e10843).sqrt();
        let assign11600_e10845: f64 = (locals.var_t0 - assign11600_e10844);
        let assign11600_e10846: f64 = (0.5 * assign11600_e10845);
        locals.var_vfgd_ov = assign11600_e10846;
        locals.var_vfgd_ov_dn3 = (0.5 * (locals.var_t0_dn3 - (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign11600_e10844))));
        locals.var_vfgd_ov_dn4 = (0.5 * (locals.var_t0_dn4 - (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign11600_e10844))));
        locals.var_vfgd_ov_dn5 = (0.5 * (locals.var_t0_dn5 - (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign11600_e10844))));
        locals.var_vfgd_ov_dn6 = (0.5 * (locals.var_t0_dn6 - (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign11600_e10844))));
        locals.var_vfgd_ov_dn7 = (0.5 * (locals.var_t0_dn7 - (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign11600_e10844))));
        locals.var_vfgd_ov_dn8 = (0.5 * (locals.var_t0_dn8 - (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign11600_e10844))));
        locals.var_vfgd_ov_rv = 0.0;

        let assign11610_e10849: f64 = (locals.var_vgd_ov_noswap - locals.var_vfbsd);
        let assign11610_e10851: f64 = (assign11610_e10849 - locals.var_vfgd_ov);
        locals.var_t1 = assign11610_e10851;
        locals.var_t1_dn3 = ((-locals.var_vfbsd_dn3) - locals.var_vfgd_ov_dn3);
        locals.var_t1_dn4 = ((-locals.var_vfbsd_dn4) - locals.var_vfgd_ov_dn4);
        locals.var_t1_dn5 = ((locals.var_vgd_ov_noswap_dn5 - locals.var_vfbsd_dn5) - locals.var_vfgd_ov_dn5);
        locals.var_t1_dn6 = ((-locals.var_vfbsd_dn6) - locals.var_vfgd_ov_dn6);
        locals.var_t1_dn7 = ((locals.var_vgd_ov_noswap_dn7 - locals.var_vfbsd_dn7) - locals.var_vfgd_ov_dn7);
        locals.var_t1_dn8 = ((-locals.var_vfbsd_dn8) - locals.var_vfgd_ov_dn8);
        locals.var_t1_rv = 0.0;

        let assign11620_e10855: f64 = (locals.var_devsign * locals.var_weffcv);
        let assign11620_e10857: f64 = (assign11620_e10855 * p.p264);
        let assign11620_e10861: f64 = (0.5 * p.p266);
        let assign11620_e10865: f64 = (4.0 * locals.var_vfgd_ov);
        let assign11620_e10867: f64 = (assign11620_e10865 / p.p266);
        let assign11620_e10868: f64 = (1.0 - assign11620_e10867);
        let assign11620_e10869: f64 = (assign11620_e10868).sqrt();
        let assign11620_e10871: f64 = (assign11620_e10869 - 1.0);
        let assign11620_e10872: f64 = (assign11620_e10861 * assign11620_e10871);
        let assign11620_e10873: f64 = (locals.var_t1 - assign11620_e10872);
        let assign11620_e10874: f64 = (assign11620_e10857 * assign11620_e10873);
        let assign11620_e10875: f64 = (locals.var_qfgd_ov + assign11620_e10874);
        locals.var_qfgd_ov = assign11620_e10875;
        locals.var_qfgd_ov_dn3 = (locals.var_qfgd_ov_dn3 + (assign11620_e10857 * (locals.var_t1_dn3 - (assign11620_e10861 * ((-((4.0 * locals.var_vfgd_ov_dn3) / p.p266)) / (2.0 * assign11620_e10869))))));
        locals.var_qfgd_ov_dn4 = (locals.var_qfgd_ov_dn4 + (assign11620_e10857 * (locals.var_t1_dn4 - (assign11620_e10861 * ((-((4.0 * locals.var_vfgd_ov_dn4) / p.p266)) / (2.0 * assign11620_e10869))))));
        locals.var_qfgd_ov_dn5 = (locals.var_qfgd_ov_dn5 + (assign11620_e10857 * (locals.var_t1_dn5 - (assign11620_e10861 * ((-((4.0 * locals.var_vfgd_ov_dn5) / p.p266)) / (2.0 * assign11620_e10869))))));
        locals.var_qfgd_ov_dn6 = (locals.var_qfgd_ov_dn6 + (assign11620_e10857 * (locals.var_t1_dn6 - (assign11620_e10861 * ((-((4.0 * locals.var_vfgd_ov_dn6) / p.p266)) / (2.0 * assign11620_e10869))))));
        locals.var_qfgd_ov_dn7 = (locals.var_qfgd_ov_dn7 + (assign11620_e10857 * (locals.var_t1_dn7 - (assign11620_e10861 * ((-((4.0 * locals.var_vfgd_ov_dn7) / p.p266)) / (2.0 * assign11620_e10869))))));
        locals.var_qfgd_ov_dn8 = (locals.var_qfgd_ov_dn8 + (assign11620_e10857 * (locals.var_t1_dn8 - (assign11620_e10861 * ((-((4.0 * locals.var_vfgd_ov_dn8) / p.p266)) / (2.0 * assign11620_e10869))))));
        locals.var_qfgd_ov_rv = 0.0;

        let assign11630_e10878: f64 = (locals.var_weffcv * locals.var_cfs_i);
        let assign11630_e10880: f64 = (assign11630_e10878 * (nv7 - nv6));
        locals.var_qfgs_of = assign11630_e10880;
        locals.var_qfgs_of_dn6 = (-assign11630_e10878);
        locals.var_qfgs_of_dn7 = assign11630_e10878;
        locals.var_qfgs_of_rv = 0.0;

        let assign11640_e10883: f64 = (locals.var_weffcv * locals.var_cfd_i);
        let assign11640_e10885: f64 = (assign11640_e10883 * (nv7 - nv5));
        locals.var_qfgd_of = assign11640_e10885;
        locals.var_qfgd_of_dn5 = (-assign11640_e10883);
        locals.var_qfgd_of_dn7 = assign11640_e10883;
        locals.var_qfgd_of_rv = 0.0;

        let assign11650_e10888: f64 = (locals.var_qfgs_ov + locals.var_qfgs_of);
        locals.var_qfgs_parasitic = assign11650_e10888;
        locals.var_qfgs_parasitic_dn3 = locals.var_qfgs_ov_dn3;
        locals.var_qfgs_parasitic_dn4 = locals.var_qfgs_ov_dn4;
        locals.var_qfgs_parasitic_dn5 = locals.var_qfgs_ov_dn5;
        locals.var_qfgs_parasitic_dn6 = (locals.var_qfgs_ov_dn6 + locals.var_qfgs_of_dn6);
        locals.var_qfgs_parasitic_dn7 = (locals.var_qfgs_ov_dn7 + locals.var_qfgs_of_dn7);
        locals.var_qfgs_parasitic_dn8 = locals.var_qfgs_ov_dn8;
        locals.var_qfgs_parasitic_rv = 0.0;

        let assign11660_e10891: f64 = (locals.var_qfgd_ov + locals.var_qfgd_of);
        locals.var_qfgd_parasitic = assign11660_e10891;
        locals.var_qfgd_parasitic_dn3 = locals.var_qfgd_ov_dn3;
        locals.var_qfgd_parasitic_dn4 = locals.var_qfgd_ov_dn4;
        locals.var_qfgd_parasitic_dn5 = (locals.var_qfgd_ov_dn5 + locals.var_qfgd_of_dn5);
        locals.var_qfgd_parasitic_dn6 = locals.var_qfgd_ov_dn6;
        locals.var_qfgd_parasitic_dn7 = (locals.var_qfgd_ov_dn7 + locals.var_qfgd_of_dn7);
        locals.var_qfgd_parasitic_dn8 = locals.var_qfgd_ov_dn8;
        locals.var_qfgd_parasitic_rv = 0.0;

        let assign11670_e10894: f64 = (locals.var_devsign * locals.var_csbox);
        let assign11670_e10896: f64 = (assign11670_e10894 * (nv6 - nv3));
        locals.var_qsbg = assign11670_e10896;
        locals.var_qsbg_dn3 = (((locals.var_devsign * locals.var_csbox_dn3) * (nv6 - nv3)) + (-assign11670_e10894));
        locals.var_qsbg_dn4 = ((locals.var_devsign * locals.var_csbox_dn4) * (nv6 - nv3));
        locals.var_qsbg_dn5 = ((locals.var_devsign * locals.var_csbox_dn5) * (nv6 - nv3));
        locals.var_qsbg_dn6 = (((locals.var_devsign * locals.var_csbox_dn6) * (nv6 - nv3)) + assign11670_e10894);
        locals.var_qsbg_dn7 = ((locals.var_devsign * locals.var_csbox_dn7) * (nv6 - nv3));
        locals.var_qsbg_dn8 = ((locals.var_devsign * locals.var_csbox_dn8) * (nv6 - nv3));
        locals.var_qsbg_rv = 0.0;

        let assign11680_e10899: f64 = (locals.var_devsign * locals.var_cdbox);
        let assign11680_e10901: f64 = (assign11680_e10899 * (nv5 - nv3));
        locals.var_qdbg = assign11680_e10901;
        locals.var_qdbg_dn3 = (((locals.var_devsign * locals.var_cdbox_dn3) * (nv5 - nv3)) + (-assign11680_e10899));
        locals.var_qdbg_dn4 = ((locals.var_devsign * locals.var_cdbox_dn4) * (nv5 - nv3));
        locals.var_qdbg_dn5 = (((locals.var_devsign * locals.var_cdbox_dn5) * (nv5 - nv3)) + assign11680_e10899);
        locals.var_qdbg_dn6 = ((locals.var_devsign * locals.var_cdbox_dn6) * (nv5 - nv3));
        locals.var_qdbg_dn7 = ((locals.var_devsign * locals.var_cdbox_dn7) * (nv5 - nv3));
        locals.var_qdbg_dn8 = ((locals.var_devsign * locals.var_cdbox_dn8) * (nv5 - nv3));
        locals.var_qdbg_rv = 0.0;

        let assign11690_e10905: f64 = (locals.var_alpha1_i * locals.var_leff);
        let assign11690_e10906: f64 = (locals.var_alpha0_i + assign11690_e10905);
        let assign11690_e10908: f64 = (assign11690_e10906 / locals.var_leff);
        locals.var_t0 = assign11690_e10908;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign11700_e10915: f64 = if ((locals.var_t0 <= 0.0) || (locals.var_beta0_t <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard119 = assign11700_e10915;
        locals.var_guard119_rv = 0.0;

        let assign11720_e10923: f64 = (locals.var_beta0_t / 80.0);
        let assign11720_e10924: f64 = if locals.var_diffvds > assign11720_e10923 { 1.0 } else { 0.0 };
        locals.var_guard120 = assign11720_e10924;
        locals.var_guard120_rv = 0.0;

        let (assign11730_e10934, assign11730_e10934_d_n3, assign11730_e10934_d_n4, assign11730_e10934_d_n5, assign11730_e10934_d_n6, assign11730_e10934_d_n7, assign11730_e10934_d_n8,) = {
    if ((locals.var_guard119 == 0.0) && (locals.var_guard120 != 0.0)) {
        let assign11730_e10930: f64 = (-locals.var_beta0_t);
        let assign11730_e10932: f64 = (assign11730_e10930 / locals.var_diffvds);
        (assign11730_e10932, (-((assign11730_e10930 * locals.var_diffvds_dn3) / (locals.var_diffvds * locals.var_diffvds))), ((((-locals.var_beta0_t_dn4) * locals.var_diffvds) - (assign11730_e10930 * locals.var_diffvds_dn4)) / (locals.var_diffvds * locals.var_diffvds)), (-((assign11730_e10930 * locals.var_diffvds_dn5) / (locals.var_diffvds * locals.var_diffvds))), (-((assign11730_e10930 * locals.var_diffvds_dn6) / (locals.var_diffvds * locals.var_diffvds))), (-((assign11730_e10930 * locals.var_diffvds_dn7) / (locals.var_diffvds * locals.var_diffvds))), (-((assign11730_e10930 * locals.var_diffvds_dn8) / (locals.var_diffvds * locals.var_diffvds))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign11730_e10934;
        locals.var_t1_dn3 = assign11730_e10934_d_n3;
        locals.var_t1_dn4 = assign11730_e10934_d_n4;
        locals.var_t1_dn5 = assign11730_e10934_d_n5;
        locals.var_t1_dn6 = assign11730_e10934_d_n6;
        locals.var_t1_dn7 = assign11730_e10934_d_n7;
        locals.var_t1_dn8 = assign11730_e10934_d_n8;
        locals.var_t1_rv = 0.0;

        let assign11820_e10971: f64 = if p.p17 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard121 = assign11820_e10971;
        locals.var_guard121_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_27(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11830_e10981, assign11830_e10981_d_n3, assign11830_e10981_d_n4, assign11830_e10981_d_n5, assign11830_e10981_d_n6, assign11830_e10981_d_n7, assign11830_e10981_d_n8,) = {
    if (locals.var_guard121 != 0.0) {
        let assign11830_e10975: f64 = (locals.var_qia - locals.var_eigbinv_i);
        let assign11830_e10977: f64 = (assign11830_e10975 / locals.var_nigbinv_i);
        let assign11830_e10979: f64 = (assign11830_e10977 / locals.var_vtm);
        (assign11830_e10979, ((locals.var_qia_dn3 / locals.var_nigbinv_i) / locals.var_vtm), ((((locals.var_qia_dn4 / locals.var_nigbinv_i) * locals.var_vtm) - (assign11830_e10977 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)), ((locals.var_qia_dn5 / locals.var_nigbinv_i) / locals.var_vtm), ((locals.var_qia_dn6 / locals.var_nigbinv_i) / locals.var_vtm), ((locals.var_qia_dn7 / locals.var_nigbinv_i) / locals.var_vtm), ((locals.var_qia_dn8 / locals.var_nigbinv_i) / locals.var_vtm),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign11830_e10981;
        locals.var_t1_dn3 = assign11830_e10981_d_n3;
        locals.var_t1_dn4 = assign11830_e10981_d_n4;
        locals.var_t1_dn5 = assign11830_e10981_d_n5;
        locals.var_t1_dn6 = assign11830_e10981_d_n6;
        locals.var_t1_dn7 = assign11830_e10981_d_n7;
        locals.var_t1_dn8 = assign11830_e10981_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign11850_e11003, assign11850_e11003_d_n3, assign11850_e11003_d_n4, assign11850_e11003_d_n5, assign11850_e11003_d_n6, assign11850_e11003_d_n7, assign11850_e11003_d_n8,) = {
    if (locals.var_guard121 != 0.0) {
        let assign11850_e11000: f64 = (locals.var_bigbinv_i * locals.var_qia);
        let assign11850_e11001: f64 = (locals.var_aigbinv_i - assign11850_e11000);
        (assign11850_e11001, (-(locals.var_bigbinv_i * locals.var_qia_dn3)), (-(locals.var_bigbinv_i * locals.var_qia_dn4)), (-(locals.var_bigbinv_i * locals.var_qia_dn5)), (-(locals.var_bigbinv_i * locals.var_qia_dn6)), (-(locals.var_bigbinv_i * locals.var_qia_dn7)), (-(locals.var_bigbinv_i * locals.var_qia_dn8)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8,)
    }
};
        locals.var_t2 = assign11850_e11003;
        locals.var_t2_dn3 = assign11850_e11003_d_n3;
        locals.var_t2_dn4 = assign11850_e11003_d_n4;
        locals.var_t2_dn5 = assign11850_e11003_d_n5;
        locals.var_t2_dn6 = assign11850_e11003_d_n6;
        locals.var_t2_dn7 = assign11850_e11003_d_n7;
        locals.var_t2_dn8 = assign11850_e11003_d_n8;
        locals.var_t2_rv = 0.0;

        let (assign11860_e11011, assign11860_e11011_d_n3, assign11860_e11011_d_n4, assign11860_e11011_d_n5, assign11860_e11011_d_n6, assign11860_e11011_d_n7, assign11860_e11011_d_n8,) = {
    if (locals.var_guard121 != 0.0) {
        let assign11860_e11008: f64 = (locals.var_cigbinv_i * locals.var_qia);
        let assign11860_e11009: f64 = (1.0 + assign11860_e11008);
        (assign11860_e11009, (locals.var_cigbinv_i * locals.var_qia_dn3), (locals.var_cigbinv_i * locals.var_qia_dn4), (locals.var_cigbinv_i * locals.var_qia_dn5), (locals.var_cigbinv_i * locals.var_qia_dn6), (locals.var_cigbinv_i * locals.var_qia_dn7), (locals.var_cigbinv_i * locals.var_qia_dn8),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8,)
    }
};
        locals.var_t3 = assign11860_e11011;
        locals.var_t3_dn3 = assign11860_e11011_d_n3;
        locals.var_t3_dn4 = assign11860_e11011_d_n4;
        locals.var_t3_dn5 = assign11860_e11011_d_n5;
        locals.var_t3_dn6 = assign11860_e11011_d_n6;
        locals.var_t3_dn7 = assign11860_e11011_d_n7;
        locals.var_t3_dn8 = assign11860_e11011_d_n8;
        locals.var_t3_rv = 0.0;

        let (assign11870_e11022, assign11870_e11022_d_n3, assign11870_e11022_d_n4, assign11870_e11022_d_n5, assign11870_e11022_d_n6, assign11870_e11022_d_n7, assign11870_e11022_d_n8,) = {
    if (locals.var_guard121 != 0.0) {
        let assign11870_e11014: f64 = (-982222000000.0);
        let assign11870_e11016: f64 = (assign11870_e11014 * p.p99);
        let assign11870_e11018: f64 = (assign11870_e11016 * locals.var_t2);
        let assign11870_e11020: f64 = (assign11870_e11018 * locals.var_t3);
        (assign11870_e11020, (((assign11870_e11016 * locals.var_t2_dn3) * locals.var_t3) + (assign11870_e11018 * locals.var_t3_dn3)), (((assign11870_e11016 * locals.var_t2_dn4) * locals.var_t3) + (assign11870_e11018 * locals.var_t3_dn4)), (((assign11870_e11016 * locals.var_t2_dn5) * locals.var_t3) + (assign11870_e11018 * locals.var_t3_dn5)), (((assign11870_e11016 * locals.var_t2_dn6) * locals.var_t3) + (assign11870_e11018 * locals.var_t3_dn6)), (((assign11870_e11016 * locals.var_t2_dn7) * locals.var_t3) + (assign11870_e11018 * locals.var_t3_dn7)), (((assign11870_e11016 * locals.var_t2_dn8) * locals.var_t3) + (assign11870_e11018 * locals.var_t3_dn8)),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8,)
    }
};
        locals.var_t4 = assign11870_e11022;
        locals.var_t4_dn3 = assign11870_e11022_d_n3;
        locals.var_t4_dn4 = assign11870_e11022_d_n4;
        locals.var_t4_dn5 = assign11870_e11022_d_n5;
        locals.var_t4_dn6 = assign11870_e11022_d_n6;
        locals.var_t4_dn7 = assign11870_e11022_d_n7;
        locals.var_t4_dn8 = assign11870_e11022_d_n8;
        locals.var_t4_rv = 0.0;

        let (assign11880_e11027, assign11880_e11027_d_n3, assign11880_e11027_d_n4, assign11880_e11027_d_n5, assign11880_e11027_d_n6, assign11880_e11027_d_n7, assign11880_e11027_d_n8,) = {
    if (locals.var_guard121 != 0.0) {
        let assign11880_e11025: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign11880_e11025, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8,)
    }
};
        locals.var_t5 = assign11880_e11027;
        locals.var_t5_dn3 = assign11880_e11027_d_n3;
        locals.var_t5_dn4 = assign11880_e11027_d_n4;
        locals.var_t5_dn5 = assign11880_e11027_d_n5;
        locals.var_t5_dn6 = assign11880_e11027_d_n6;
        locals.var_t5_dn7 = assign11880_e11027_d_n7;
        locals.var_t5_dn8 = assign11880_e11027_d_n8;
        locals.var_t5_rv = 0.0;

        let (assign11890_e11031, assign11890_e11031_d_n3, assign11890_e11031_d_n4, assign11890_e11031_d_n5, assign11890_e11031_d_n6, assign11890_e11031_d_n7, assign11890_e11031_d_n8,) = {
    if (locals.var_guard121 != 0.0) {
        (3.75956e-7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8,)
    }
};
        locals.var_t6 = assign11890_e11031;
        locals.var_t6_dn3 = assign11890_e11031_d_n3;
        locals.var_t6_dn4 = assign11890_e11031_d_n4;
        locals.var_t6_dn5 = assign11890_e11031_d_n5;
        locals.var_t6_dn6 = assign11890_e11031_d_n6;
        locals.var_t6_dn7 = assign11890_e11031_d_n7;
        locals.var_t6_dn8 = assign11890_e11031_d_n8;
        locals.var_t6_rv = 0.0;

        let (assign11920_e11059, assign11920_e11059_d_n3, assign11920_e11059_d_n4, assign11920_e11059_d_n5, assign11920_e11059_d_n6, assign11920_e11059_d_n7, assign11920_e11059_d_n8,) = {
    if (locals.var_guard121 != 0.0) {
        let assign11920_e11057: f64 = (locals.var_deltaphi1 - locals.var_phib);
        (assign11920_e11057, (-locals.var_phib_dn3), (locals.var_deltaphi1_dn4 - locals.var_phib_dn4), (-locals.var_phib_dn5), (-locals.var_phib_dn6), (-locals.var_phib_dn7), (-locals.var_phib_dn8),)
    } else {
        (locals.var_vfbzb, locals.var_vfbzb_dn3, locals.var_vfbzb_dn4, locals.var_vfbzb_dn5, locals.var_vfbzb_dn6, locals.var_vfbzb_dn7, locals.var_vfbzb_dn8,)
    }
};
        locals.var_vfbzb = assign11920_e11059;
        locals.var_vfbzb_dn3 = assign11920_e11059_d_n3;
        locals.var_vfbzb_dn4 = assign11920_e11059_d_n4;
        locals.var_vfbzb_dn5 = assign11920_e11059_d_n5;
        locals.var_vfbzb_dn6 = assign11920_e11059_d_n6;
        locals.var_vfbzb_dn7 = assign11920_e11059_d_n7;
        locals.var_vfbzb_dn8 = assign11920_e11059_d_n8;
        locals.var_vfbzb_rv = 0.0;

        let (assign11930_e11065, assign11930_e11065_d_n3, assign11930_e11065_d_n4, assign11930_e11065_d_n5, assign11930_e11065_d_n6, assign11930_e11065_d_n7, assign11930_e11065_d_n8,) = {
    if (locals.var_guard121 != 0.0) {
        let assign11930_e11063: f64 = (locals.var_vfbzb - locals.var_vgbg);
        (assign11930_e11063, (locals.var_vfbzb_dn3 - locals.var_vgbg_dn3), locals.var_vfbzb_dn4, locals.var_vfbzb_dn5, locals.var_vfbzb_dn6, locals.var_vfbzb_dn7, (locals.var_vfbzb_dn8 - locals.var_vgbg_dn8),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    }
};
        locals.var_t0 = assign11930_e11065;
        locals.var_t0_dn3 = assign11930_e11065_d_n3;
        locals.var_t0_dn4 = assign11930_e11065_d_n4;
        locals.var_t0_dn5 = assign11930_e11065_d_n5;
        locals.var_t0_dn6 = assign11930_e11065_d_n6;
        locals.var_t0_dn7 = assign11930_e11065_d_n7;
        locals.var_t0_dn8 = assign11930_e11065_d_n8;
        locals.var_t0_rv = 0.0;

        let (assign11940_e11073, assign11940_e11073_d_n3, assign11940_e11073_d_n4, assign11940_e11073_d_n5, assign11940_e11073_d_n6, assign11940_e11073_d_n7, assign11940_e11073_d_n8,) = {
    if (locals.var_guard121 != 0.0) {
        let assign11940_e11069: f64 = (locals.var_t0 / locals.var_nigbacc_i);
        let assign11940_e11071: f64 = (assign11940_e11069 / locals.var_vtm);
        (assign11940_e11071, ((locals.var_t0_dn3 / locals.var_nigbacc_i) / locals.var_vtm), ((((locals.var_t0_dn4 / locals.var_nigbacc_i) * locals.var_vtm) - (assign11940_e11069 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)), ((locals.var_t0_dn5 / locals.var_nigbacc_i) / locals.var_vtm), ((locals.var_t0_dn6 / locals.var_nigbacc_i) / locals.var_vtm), ((locals.var_t0_dn7 / locals.var_nigbacc_i) / locals.var_vtm), ((locals.var_t0_dn8 / locals.var_nigbacc_i) / locals.var_vtm),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign11940_e11073;
        locals.var_t1_dn3 = assign11940_e11073_d_n3;
        locals.var_t1_dn4 = assign11940_e11073_d_n4;
        locals.var_t1_dn5 = assign11940_e11073_d_n5;
        locals.var_t1_dn6 = assign11940_e11073_d_n6;
        locals.var_t1_dn7 = assign11940_e11073_d_n7;
        locals.var_t1_dn8 = assign11940_e11073_d_n8;
        locals.var_t1_rv = 0.0;

        let assign11960_e11090: f64 = if locals.var_vfbzb <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard122 = assign11960_e11090;
        locals.var_guard122_rv = 0.0;

        let (assign11970_e11113, assign11970_e11113_d_n3, assign11970_e11113_d_n4, assign11970_e11113_d_n5, assign11970_e11113_d_n6, assign11970_e11113_d_n7, assign11970_e11113_d_n8,) = {
    if ((locals.var_guard121 != 0.0) && (locals.var_guard122 != 0.0)) {
        let assign11970_e11097: f64 = (locals.var_t0 - 0.02);
        let assign11970_e11100: f64 = (locals.var_t0 - 0.02);
        let assign11970_e11103: f64 = (locals.var_t0 - 0.02);
        let assign11970_e11104: f64 = (assign11970_e11100 * assign11970_e11103);
        let assign11970_e11107: f64 = (0.08 * locals.var_vfbzb);
        let assign11970_e11108: f64 = (assign11970_e11104 - assign11970_e11107);
        let assign11970_e11109: f64 = (assign11970_e11108).sqrt();
        let assign11970_e11110: f64 = (assign11970_e11097 + assign11970_e11109);
        let assign11970_e11111: f64 = (0.5 * assign11970_e11110);
        (assign11970_e11111, (0.5 * (locals.var_t0_dn3 + ((((locals.var_t0_dn3 * assign11970_e11103) + (assign11970_e11100 * locals.var_t0_dn3)) - (0.08 * locals.var_vfbzb_dn3)) / (2.0 * assign11970_e11109)))), (0.5 * (locals.var_t0_dn4 + ((((locals.var_t0_dn4 * assign11970_e11103) + (assign11970_e11100 * locals.var_t0_dn4)) - (0.08 * locals.var_vfbzb_dn4)) / (2.0 * assign11970_e11109)))), (0.5 * (locals.var_t0_dn5 + ((((locals.var_t0_dn5 * assign11970_e11103) + (assign11970_e11100 * locals.var_t0_dn5)) - (0.08 * locals.var_vfbzb_dn5)) / (2.0 * assign11970_e11109)))), (0.5 * (locals.var_t0_dn6 + ((((locals.var_t0_dn6 * assign11970_e11103) + (assign11970_e11100 * locals.var_t0_dn6)) - (0.08 * locals.var_vfbzb_dn6)) / (2.0 * assign11970_e11109)))), (0.5 * (locals.var_t0_dn7 + ((((locals.var_t0_dn7 * assign11970_e11103) + (assign11970_e11100 * locals.var_t0_dn7)) - (0.08 * locals.var_vfbzb_dn7)) / (2.0 * assign11970_e11109)))), (0.5 * (locals.var_t0_dn8 + ((((locals.var_t0_dn8 * assign11970_e11103) + (assign11970_e11100 * locals.var_t0_dn8)) - (0.08 * locals.var_vfbzb_dn8)) / (2.0 * assign11970_e11109)))),)
    } else {
        (locals.var_voxacc, locals.var_voxacc_dn3, locals.var_voxacc_dn4, locals.var_voxacc_dn5, locals.var_voxacc_dn6, locals.var_voxacc_dn7, locals.var_voxacc_dn8,)
    }
};
        locals.var_voxacc = assign11970_e11113;
        locals.var_voxacc_dn3 = assign11970_e11113_d_n3;
        locals.var_voxacc_dn4 = assign11970_e11113_d_n4;
        locals.var_voxacc_dn5 = assign11970_e11113_d_n5;
        locals.var_voxacc_dn6 = assign11970_e11113_d_n6;
        locals.var_voxacc_dn7 = assign11970_e11113_d_n7;
        locals.var_voxacc_dn8 = assign11970_e11113_d_n8;
        locals.var_voxacc_rv = 0.0;

        let (assign11980_e11137, assign11980_e11137_d_n3, assign11980_e11137_d_n4, assign11980_e11137_d_n5, assign11980_e11137_d_n6, assign11980_e11137_d_n7, assign11980_e11137_d_n8,) = {
    if ((locals.var_guard121 != 0.0) && (locals.var_guard122 == 0.0)) {
        let assign11980_e11121: f64 = (locals.var_t0 - 0.02);
        let assign11980_e11124: f64 = (locals.var_t0 - 0.02);
        let assign11980_e11127: f64 = (locals.var_t0 - 0.02);
        let assign11980_e11128: f64 = (assign11980_e11124 * assign11980_e11127);
        let assign11980_e11131: f64 = (0.08 * locals.var_vfbzb);
        let assign11980_e11132: f64 = (assign11980_e11128 + assign11980_e11131);
        let assign11980_e11133: f64 = (assign11980_e11132).sqrt();
        let assign11980_e11134: f64 = (assign11980_e11121 + assign11980_e11133);
        let assign11980_e11135: f64 = (0.5 * assign11980_e11134);
        (assign11980_e11135, (0.5 * (locals.var_t0_dn3 + ((((locals.var_t0_dn3 * assign11980_e11127) + (assign11980_e11124 * locals.var_t0_dn3)) + (0.08 * locals.var_vfbzb_dn3)) / (2.0 * assign11980_e11133)))), (0.5 * (locals.var_t0_dn4 + ((((locals.var_t0_dn4 * assign11980_e11127) + (assign11980_e11124 * locals.var_t0_dn4)) + (0.08 * locals.var_vfbzb_dn4)) / (2.0 * assign11980_e11133)))), (0.5 * (locals.var_t0_dn5 + ((((locals.var_t0_dn5 * assign11980_e11127) + (assign11980_e11124 * locals.var_t0_dn5)) + (0.08 * locals.var_vfbzb_dn5)) / (2.0 * assign11980_e11133)))), (0.5 * (locals.var_t0_dn6 + ((((locals.var_t0_dn6 * assign11980_e11127) + (assign11980_e11124 * locals.var_t0_dn6)) + (0.08 * locals.var_vfbzb_dn6)) / (2.0 * assign11980_e11133)))), (0.5 * (locals.var_t0_dn7 + ((((locals.var_t0_dn7 * assign11980_e11127) + (assign11980_e11124 * locals.var_t0_dn7)) + (0.08 * locals.var_vfbzb_dn7)) / (2.0 * assign11980_e11133)))), (0.5 * (locals.var_t0_dn8 + ((((locals.var_t0_dn8 * assign11980_e11127) + (assign11980_e11124 * locals.var_t0_dn8)) + (0.08 * locals.var_vfbzb_dn8)) / (2.0 * assign11980_e11133)))),)
    } else {
        (locals.var_voxacc, locals.var_voxacc_dn3, locals.var_voxacc_dn4, locals.var_voxacc_dn5, locals.var_voxacc_dn6, locals.var_voxacc_dn7, locals.var_voxacc_dn8,)
    }
};
        locals.var_voxacc = assign11980_e11137;
        locals.var_voxacc_dn3 = assign11980_e11137_d_n3;
        locals.var_voxacc_dn4 = assign11980_e11137_d_n4;
        locals.var_voxacc_dn5 = assign11980_e11137_d_n5;
        locals.var_voxacc_dn6 = assign11980_e11137_d_n6;
        locals.var_voxacc_dn7 = assign11980_e11137_d_n7;
        locals.var_voxacc_dn8 = assign11980_e11137_d_n8;
        locals.var_voxacc_rv = 0.0;

        let (assign11990_e11145, assign11990_e11145_d_n3, assign11990_e11145_d_n4, assign11990_e11145_d_n5, assign11990_e11145_d_n6, assign11990_e11145_d_n7, assign11990_e11145_d_n8,) = {
    if (locals.var_guard121 != 0.0) {
        let assign11990_e11142: f64 = (locals.var_bigbacc_i * locals.var_voxacc);
        let assign11990_e11143: f64 = (locals.var_aigbacc_i - assign11990_e11142);
        (assign11990_e11143, (-(locals.var_bigbacc_i * locals.var_voxacc_dn3)), (-(locals.var_bigbacc_i * locals.var_voxacc_dn4)), (-(locals.var_bigbacc_i * locals.var_voxacc_dn5)), (-(locals.var_bigbacc_i * locals.var_voxacc_dn6)), (-(locals.var_bigbacc_i * locals.var_voxacc_dn7)), (-(locals.var_bigbacc_i * locals.var_voxacc_dn8)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8,)
    }
};
        locals.var_t2 = assign11990_e11145;
        locals.var_t2_dn3 = assign11990_e11145_d_n3;
        locals.var_t2_dn4 = assign11990_e11145_d_n4;
        locals.var_t2_dn5 = assign11990_e11145_d_n5;
        locals.var_t2_dn6 = assign11990_e11145_d_n6;
        locals.var_t2_dn7 = assign11990_e11145_d_n7;
        locals.var_t2_dn8 = assign11990_e11145_d_n8;
        locals.var_t2_rv = 0.0;

        let (assign12000_e11153, assign12000_e11153_d_n3, assign12000_e11153_d_n4, assign12000_e11153_d_n5, assign12000_e11153_d_n6, assign12000_e11153_d_n7, assign12000_e11153_d_n8,) = {
    if (locals.var_guard121 != 0.0) {
        let assign12000_e11150: f64 = (locals.var_cigbacc_i * locals.var_voxacc);
        let assign12000_e11151: f64 = (1.0 + assign12000_e11150);
        (assign12000_e11151, (locals.var_cigbacc_i * locals.var_voxacc_dn3), (locals.var_cigbacc_i * locals.var_voxacc_dn4), (locals.var_cigbacc_i * locals.var_voxacc_dn5), (locals.var_cigbacc_i * locals.var_voxacc_dn6), (locals.var_cigbacc_i * locals.var_voxacc_dn7), (locals.var_cigbacc_i * locals.var_voxacc_dn8),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8,)
    }
};
        locals.var_t3 = assign12000_e11153;
        locals.var_t3_dn3 = assign12000_e11153_d_n3;
        locals.var_t3_dn4 = assign12000_e11153_d_n4;
        locals.var_t3_dn5 = assign12000_e11153_d_n5;
        locals.var_t3_dn6 = assign12000_e11153_d_n6;
        locals.var_t3_dn7 = assign12000_e11153_d_n7;
        locals.var_t3_dn8 = assign12000_e11153_d_n8;
        locals.var_t3_rv = 0.0;

        let (assign12010_e11164, assign12010_e11164_d_n3, assign12010_e11164_d_n4, assign12010_e11164_d_n5, assign12010_e11164_d_n6, assign12010_e11164_d_n7, assign12010_e11164_d_n8,) = {
    if (locals.var_guard121 != 0.0) {
        let assign12010_e11156: f64 = (-745669000000.0);
        let assign12010_e11158: f64 = (assign12010_e11156 * p.p99);
        let assign12010_e11160: f64 = (assign12010_e11158 * locals.var_t2);
        let assign12010_e11162: f64 = (assign12010_e11160 * locals.var_t3);
        (assign12010_e11162, (((assign12010_e11158 * locals.var_t2_dn3) * locals.var_t3) + (assign12010_e11160 * locals.var_t3_dn3)), (((assign12010_e11158 * locals.var_t2_dn4) * locals.var_t3) + (assign12010_e11160 * locals.var_t3_dn4)), (((assign12010_e11158 * locals.var_t2_dn5) * locals.var_t3) + (assign12010_e11160 * locals.var_t3_dn5)), (((assign12010_e11158 * locals.var_t2_dn6) * locals.var_t3) + (assign12010_e11160 * locals.var_t3_dn6)), (((assign12010_e11158 * locals.var_t2_dn7) * locals.var_t3) + (assign12010_e11160 * locals.var_t3_dn7)), (((assign12010_e11158 * locals.var_t2_dn8) * locals.var_t3) + (assign12010_e11160 * locals.var_t3_dn8)),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8,)
    }
};
        locals.var_t4 = assign12010_e11164;
        locals.var_t4_dn3 = assign12010_e11164_d_n3;
        locals.var_t4_dn4 = assign12010_e11164_d_n4;
        locals.var_t4_dn5 = assign12010_e11164_d_n5;
        locals.var_t4_dn6 = assign12010_e11164_d_n6;
        locals.var_t4_dn7 = assign12010_e11164_d_n7;
        locals.var_t4_dn8 = assign12010_e11164_d_n8;
        locals.var_t4_rv = 0.0;

        let (assign12020_e11169, assign12020_e11169_d_n3, assign12020_e11169_d_n4, assign12020_e11169_d_n5, assign12020_e11169_d_n6, assign12020_e11169_d_n7, assign12020_e11169_d_n8,) = {
    if (locals.var_guard121 != 0.0) {
        let assign12020_e11167: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign12020_e11167, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8,)
    }
};
        locals.var_t5 = assign12020_e11169;
        locals.var_t5_dn3 = assign12020_e11169_d_n3;
        locals.var_t5_dn4 = assign12020_e11169_d_n4;
        locals.var_t5_dn5 = assign12020_e11169_d_n5;
        locals.var_t5_dn6 = assign12020_e11169_d_n6;
        locals.var_t5_dn7 = assign12020_e11169_d_n7;
        locals.var_t5_dn8 = assign12020_e11169_d_n8;
        locals.var_t5_rv = 0.0;

        let (assign12030_e11173, assign12030_e11173_d_n3, assign12030_e11173_d_n4, assign12030_e11173_d_n5, assign12030_e11173_d_n6, assign12030_e11173_d_n7, assign12030_e11173_d_n8,) = {
    if (locals.var_guard121 != 0.0) {
        (4.97232e-7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8,)
    }
};
        locals.var_t6 = assign12030_e11173;
        locals.var_t6_dn3 = assign12030_e11173_d_n3;
        locals.var_t6_dn4 = assign12030_e11173_d_n4;
        locals.var_t6_dn5 = assign12030_e11173_d_n5;
        locals.var_t6_dn6 = assign12030_e11173_d_n6;
        locals.var_t6_dn7 = assign12030_e11173_d_n7;
        locals.var_t6_dn8 = assign12030_e11173_d_n8;
        locals.var_t6_rv = 0.0;

        let assign12060_e11198: f64 = (0.6 * locals.var_vds_noswap);
        let assign12060_e11200: f64 = (assign12060_e11198 / locals.var_vtm);
        let assign12060_e11201: f64 = (assign12060_e11200).tanh();
        locals.var_t0 = assign12060_e11201;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = ((-((assign12060_e11198 * locals.var_vtm_dn4) / (locals.var_vtm * locals.var_vtm))) / ((assign12060_e11200).cosh() * (assign12060_e11200).cosh()));
        locals.var_t0_dn5 = (((0.6 * locals.var_vds_noswap_dn5) / locals.var_vtm) / ((assign12060_e11200).cosh() * (assign12060_e11200).cosh()));
        locals.var_t0_dn6 = (((0.6 * locals.var_vds_noswap_dn6) / locals.var_vtm) / ((assign12060_e11200).cosh() * (assign12060_e11200).cosh()));
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign12110_e11222: f64 = if p.p16 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard123 = assign12110_e11222;
        locals.var_guard123_rv = 0.0;

        let (assign12120_e11234, assign12120_e11234_d_n3, assign12120_e11234_d_n4, assign12120_e11234_d_n5, assign12120_e11234_d_n6, assign12120_e11234_d_n7, assign12120_e11234_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12120_e11229: f64 = (locals.var_digc_i * locals.var_phifs);
        let assign12120_e11230: f64 = (locals.var_vgfb1 - assign12120_e11229);
        let assign12120_e11231: f64 = (locals.var_bigc_i * assign12120_e11230);
        let assign12120_e11232: f64 = (locals.var_aigc_i - assign12120_e11231);
        (assign12120_e11232, (-(locals.var_bigc_i * (-(locals.var_digc_i * locals.var_phifs_dn3)))), (-(locals.var_bigc_i * (locals.var_vgfb1_dn4 - (locals.var_digc_i * locals.var_phifs_dn4)))), (-(locals.var_bigc_i * (locals.var_vgfb1_dn5 - (locals.var_digc_i * locals.var_phifs_dn5)))), (-(locals.var_bigc_i * (locals.var_vgfb1_dn6 - (locals.var_digc_i * locals.var_phifs_dn6)))), (-(locals.var_bigc_i * (-(locals.var_digc_i * locals.var_phifs_dn7)))), (-(locals.var_bigc_i * (locals.var_vgfb1_dn8 - (locals.var_digc_i * locals.var_phifs_dn8)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign12120_e11234;
        locals.var_t1_dn3 = assign12120_e11234_d_n3;
        locals.var_t1_dn4 = assign12120_e11234_d_n4;
        locals.var_t1_dn5 = assign12120_e11234_d_n5;
        locals.var_t1_dn6 = assign12120_e11234_d_n6;
        locals.var_t1_dn7 = assign12120_e11234_d_n7;
        locals.var_t1_dn8 = assign12120_e11234_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign12130_e11246, assign12130_e11246_d_n3, assign12130_e11246_d_n4, assign12130_e11246_d_n5, assign12130_e11246_d_n6, assign12130_e11246_d_n7, assign12130_e11246_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12130_e11241: f64 = (locals.var_digc_i * locals.var_phifs);
        let assign12130_e11242: f64 = (locals.var_vgfb1 - assign12130_e11241);
        let assign12130_e11243: f64 = (locals.var_cigc_i * assign12130_e11242);
        let assign12130_e11244: f64 = (1.0 + assign12130_e11243);
        (assign12130_e11244, (locals.var_cigc_i * (-(locals.var_digc_i * locals.var_phifs_dn3))), (locals.var_cigc_i * (locals.var_vgfb1_dn4 - (locals.var_digc_i * locals.var_phifs_dn4))), (locals.var_cigc_i * (locals.var_vgfb1_dn5 - (locals.var_digc_i * locals.var_phifs_dn5))), (locals.var_cigc_i * (locals.var_vgfb1_dn6 - (locals.var_digc_i * locals.var_phifs_dn6))), (locals.var_cigc_i * (-(locals.var_digc_i * locals.var_phifs_dn7))), (locals.var_cigc_i * (locals.var_vgfb1_dn8 - (locals.var_digc_i * locals.var_phifs_dn8))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8,)
    }
};
        locals.var_t2 = assign12130_e11246;
        locals.var_t2_dn3 = assign12130_e11246_d_n3;
        locals.var_t2_dn4 = assign12130_e11246_d_n4;
        locals.var_t2_dn5 = assign12130_e11246_d_n5;
        locals.var_t2_dn6 = assign12130_e11246_d_n6;
        locals.var_t2_dn7 = assign12130_e11246_d_n7;
        locals.var_t2_dn8 = assign12130_e11246_d_n8;
        locals.var_t2_rv = 0.0;

        let (assign12140_e11257, assign12140_e11257_d_n3, assign12140_e11257_d_n4, assign12140_e11257_d_n5, assign12140_e11257_d_n6, assign12140_e11257_d_n7, assign12140_e11257_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12140_e11249: f64 = (-locals.var_bechvb);
        let assign12140_e11251: f64 = (assign12140_e11249 * p.p99);
        let assign12140_e11253: f64 = (assign12140_e11251 * locals.var_t1);
        let assign12140_e11255: f64 = (assign12140_e11253 * locals.var_t2);
        (assign12140_e11255, (((assign12140_e11251 * locals.var_t1_dn3) * locals.var_t2) + (assign12140_e11253 * locals.var_t2_dn3)), (((assign12140_e11251 * locals.var_t1_dn4) * locals.var_t2) + (assign12140_e11253 * locals.var_t2_dn4)), (((assign12140_e11251 * locals.var_t1_dn5) * locals.var_t2) + (assign12140_e11253 * locals.var_t2_dn5)), (((assign12140_e11251 * locals.var_t1_dn6) * locals.var_t2) + (assign12140_e11253 * locals.var_t2_dn6)), (((assign12140_e11251 * locals.var_t1_dn7) * locals.var_t2) + (assign12140_e11253 * locals.var_t2_dn7)), (((assign12140_e11251 * locals.var_t1_dn8) * locals.var_t2) + (assign12140_e11253 * locals.var_t2_dn8)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8,)
    }
};
        locals.var_t3 = assign12140_e11257;
        locals.var_t3_dn3 = assign12140_e11257_d_n3;
        locals.var_t3_dn4 = assign12140_e11257_d_n4;
        locals.var_t3_dn5 = assign12140_e11257_d_n5;
        locals.var_t3_dn6 = assign12140_e11257_d_n6;
        locals.var_t3_dn7 = assign12140_e11257_d_n7;
        locals.var_t3_dn8 = assign12140_e11257_d_n8;
        locals.var_t3_rv = 0.0;

        let (assign12150_e11264, assign12150_e11264_d_n3, assign12150_e11264_d_n4, assign12150_e11264_d_n5, assign12150_e11264_d_n6, assign12150_e11264_d_n7, assign12150_e11264_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12150_e11261: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign12150_e11262: f64 = (locals.var_qia * assign12150_e11261);
        (assign12150_e11262, ((locals.var_qia_dn3 * assign12150_e11261) + (locals.var_qia * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3))), ((locals.var_qia_dn4 * assign12150_e11261) + (locals.var_qia * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4))), ((locals.var_qia_dn5 * assign12150_e11261) + (locals.var_qia * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5))), ((locals.var_qia_dn6 * assign12150_e11261) + (locals.var_qia * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6))), ((locals.var_qia_dn7 * assign12150_e11261) + (locals.var_qia * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7))), ((locals.var_qia_dn8 * assign12150_e11261) + (locals.var_qia * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8,)
    }
};
        locals.var_t4 = assign12150_e11264;
        locals.var_t4_dn3 = assign12150_e11264_d_n3;
        locals.var_t4_dn4 = assign12150_e11264_d_n4;
        locals.var_t4_dn5 = assign12150_e11264_d_n5;
        locals.var_t4_dn6 = assign12150_e11264_d_n6;
        locals.var_t4_dn7 = assign12150_e11264_d_n7;
        locals.var_t4_dn8 = assign12150_e11264_d_n8;
        locals.var_t4_rv = 0.0;

        let (assign12160_e11278, assign12160_e11278_d_n3, assign12160_e11278_d_n4, assign12160_e11278_d_n5, assign12160_e11278_d_n6, assign12160_e11278_d_n7, assign12160_e11278_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12160_e11269: f64 = (0.5 * locals.var_vdsx);
        let assign12160_e11270: f64 = (locals.var_vgbg + assign12160_e11269);
        let assign12160_e11274: f64 = (locals.var_vbgs_noswap + locals.var_vbgd_noswap);
        let assign12160_e11275: f64 = (0.5 * assign12160_e11274);
        let assign12160_e11276: f64 = (assign12160_e11270 + assign12160_e11275);
        (assign12160_e11276, (locals.var_vgbg_dn3 + (0.5 * (locals.var_vbgs_noswap_dn3 + locals.var_vbgd_noswap_dn3))), 0.0, ((0.5 * locals.var_vdsx_dn5) + (0.5 * locals.var_vbgd_noswap_dn5)), ((0.5 * locals.var_vdsx_dn6) + (0.5 * locals.var_vbgs_noswap_dn6)), 0.0, locals.var_vgbg_dn8,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8,)
    }
};
        locals.var_t5 = assign12160_e11278;
        locals.var_t5_dn3 = assign12160_e11278_d_n3;
        locals.var_t5_dn4 = assign12160_e11278_d_n4;
        locals.var_t5_dn5 = assign12160_e11278_d_n5;
        locals.var_t5_dn6 = assign12160_e11278_d_n6;
        locals.var_t5_dn7 = assign12160_e11278_d_n7;
        locals.var_t5_dn8 = assign12160_e11278_d_n8;
        locals.var_t5_rv = 0.0;

        let (assign12180_e11305, assign12180_e11305_d_n3, assign12180_e11305_d_n4, assign12180_e11305_d_n5, assign12180_e11305_d_n6, assign12180_e11305_d_n7, assign12180_e11305_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12180_e11298: f64 = (locals.var_vdseff * locals.var_vdseff);
        let assign12180_e11300: f64 = (assign12180_e11298 + 0.01);
        let assign12180_e11301: f64 = (assign12180_e11300).sqrt();
        let assign12180_e11303: f64 = (assign12180_e11301 - 0.1);
        (assign12180_e11303, (((locals.var_vdseff_dn3 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn3)) / (2.0 * assign12180_e11301)), (((locals.var_vdseff_dn4 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn4)) / (2.0 * assign12180_e11301)), (((locals.var_vdseff_dn5 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn5)) / (2.0 * assign12180_e11301)), (((locals.var_vdseff_dn6 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn6)) / (2.0 * assign12180_e11301)), (((locals.var_vdseff_dn7 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn7)) / (2.0 * assign12180_e11301)), (((locals.var_vdseff_dn8 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn8)) / (2.0 * assign12180_e11301)),)
    } else {
        (locals.var_vdseffx, locals.var_vdseffx_dn3, locals.var_vdseffx_dn4, locals.var_vdseffx_dn5, locals.var_vdseffx_dn6, locals.var_vdseffx_dn7, locals.var_vdseffx_dn8,)
    }
};
        locals.var_vdseffx = assign12180_e11305;
        locals.var_vdseffx_dn3 = assign12180_e11305_d_n3;
        locals.var_vdseffx_dn4 = assign12180_e11305_d_n4;
        locals.var_vdseffx_dn5 = assign12180_e11305_d_n5;
        locals.var_vdseffx_dn6 = assign12180_e11305_d_n6;
        locals.var_vdseffx_dn7 = assign12180_e11305_d_n7;
        locals.var_vdseffx_dn8 = assign12180_e11305_d_n8;
        locals.var_vdseffx_rv = 0.0;

        let (assign12190_e11311, assign12190_e11311_d_n3, assign12190_e11311_d_n4, assign12190_e11311_d_n5, assign12190_e11311_d_n6, assign12190_e11311_d_n7, assign12190_e11311_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12190_e11309: f64 = (locals.var_pigcd_i * locals.var_vdseffx);
        (assign12190_e11309, (locals.var_pigcd_i * locals.var_vdseffx_dn3), (locals.var_pigcd_i * locals.var_vdseffx_dn4), (locals.var_pigcd_i * locals.var_vdseffx_dn5), (locals.var_pigcd_i * locals.var_vdseffx_dn6), (locals.var_pigcd_i * locals.var_vdseffx_dn7), (locals.var_pigcd_i * locals.var_vdseffx_dn8),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign12190_e11311;
        locals.var_t1_dn3 = assign12190_e11311_d_n3;
        locals.var_t1_dn4 = assign12190_e11311_d_n4;
        locals.var_t1_dn5 = assign12190_e11311_d_n5;
        locals.var_t1_dn6 = assign12190_e11311_d_n6;
        locals.var_t1_dn7 = assign12190_e11311_d_n7;
        locals.var_t1_dn8 = assign12190_e11311_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign12200_e11317, assign12200_e11317_d_n3, assign12200_e11317_d_n4, assign12200_e11317_d_n5, assign12200_e11317_d_n6, assign12200_e11317_d_n7, assign12200_e11317_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12200_e11314: f64 = (-locals.var_t1);
        let assign12200_e11315: f64 = { let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign12200_e11315, ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn3)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn4)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn5)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn6)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn7)), ({ let limited_exp_arg = assign12200_e11314; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn8)),)
    } else {
        (locals.var_t1_exp, locals.var_t1_exp_dn3, locals.var_t1_exp_dn4, locals.var_t1_exp_dn5, locals.var_t1_exp_dn6, locals.var_t1_exp_dn7, locals.var_t1_exp_dn8,)
    }
};
        locals.var_t1_exp = assign12200_e11317;
        locals.var_t1_exp_dn3 = assign12200_e11317_d_n3;
        locals.var_t1_exp_dn4 = assign12200_e11317_d_n4;
        locals.var_t1_exp_dn5 = assign12200_e11317_d_n5;
        locals.var_t1_exp_dn6 = assign12200_e11317_d_n6;
        locals.var_t1_exp_dn7 = assign12200_e11317_d_n7;
        locals.var_t1_exp_dn8 = assign12200_e11317_d_n8;
        locals.var_t1_exp_rv = 0.0;

        let (assign12210_e11327, assign12210_e11327_d_n3, assign12210_e11327_d_n4, assign12210_e11327_d_n5, assign12210_e11327_d_n6, assign12210_e11327_d_n7, assign12210_e11327_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12210_e11321: f64 = (locals.var_t1 + locals.var_t1_exp);
        let assign12210_e11323: f64 = (assign12210_e11321 - 1.0);
        let assign12210_e11325: f64 = (assign12210_e11323 + 0.0001);
        (assign12210_e11325, (locals.var_t1_dn3 + locals.var_t1_exp_dn3), (locals.var_t1_dn4 + locals.var_t1_exp_dn4), (locals.var_t1_dn5 + locals.var_t1_exp_dn5), (locals.var_t1_dn6 + locals.var_t1_exp_dn6), (locals.var_t1_dn7 + locals.var_t1_exp_dn7), (locals.var_t1_dn8 + locals.var_t1_exp_dn8),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8,)
    }
};
        locals.var_t3 = assign12210_e11327;
        locals.var_t3_dn3 = assign12210_e11327_d_n3;
        locals.var_t3_dn4 = assign12210_e11327_d_n4;
        locals.var_t3_dn5 = assign12210_e11327_d_n5;
        locals.var_t3_dn6 = assign12210_e11327_d_n6;
        locals.var_t3_dn7 = assign12210_e11327_d_n7;
        locals.var_t3_dn8 = assign12210_e11327_d_n8;
        locals.var_t3_rv = 0.0;

        let (assign12220_e11339, assign12220_e11339_d_n3, assign12220_e11339_d_n4, assign12220_e11339_d_n5, assign12220_e11339_d_n6, assign12220_e11339_d_n7, assign12220_e11339_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12220_e11332: f64 = (locals.var_t1 + 1.0);
        let assign12220_e11334: f64 = (assign12220_e11332 * locals.var_t1_exp);
        let assign12220_e11335: f64 = (1.0 - assign12220_e11334);
        let assign12220_e11337: f64 = (assign12220_e11335 + 0.0001);
        (assign12220_e11337, (-((locals.var_t1_dn3 * locals.var_t1_exp) + (assign12220_e11332 * locals.var_t1_exp_dn3))), (-((locals.var_t1_dn4 * locals.var_t1_exp) + (assign12220_e11332 * locals.var_t1_exp_dn4))), (-((locals.var_t1_dn5 * locals.var_t1_exp) + (assign12220_e11332 * locals.var_t1_exp_dn5))), (-((locals.var_t1_dn6 * locals.var_t1_exp) + (assign12220_e11332 * locals.var_t1_exp_dn6))), (-((locals.var_t1_dn7 * locals.var_t1_exp) + (assign12220_e11332 * locals.var_t1_exp_dn7))), (-((locals.var_t1_dn8 * locals.var_t1_exp) + (assign12220_e11332 * locals.var_t1_exp_dn8))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8,)
    }
};
        locals.var_t4 = assign12220_e11339;
        locals.var_t4_dn3 = assign12220_e11339_d_n3;
        locals.var_t4_dn4 = assign12220_e11339_d_n4;
        locals.var_t4_dn5 = assign12220_e11339_d_n5;
        locals.var_t4_dn6 = assign12220_e11339_d_n6;
        locals.var_t4_dn7 = assign12220_e11339_d_n7;
        locals.var_t4_dn8 = assign12220_e11339_d_n8;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_28(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12230_e11347, assign12230_e11347_d_n3, assign12230_e11347_d_n4, assign12230_e11347_d_n5, assign12230_e11347_d_n6, assign12230_e11347_d_n7, assign12230_e11347_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12230_e11343: f64 = (locals.var_t1 * locals.var_t1);
        let assign12230_e11345: f64 = (assign12230_e11343 + 0.0002);
        (assign12230_e11345, ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8,)
    }
};
        locals.var_t5 = assign12230_e11347;
        locals.var_t5_dn3 = assign12230_e11347_d_n3;
        locals.var_t5_dn4 = assign12230_e11347_d_n4;
        locals.var_t5_dn5 = assign12230_e11347_d_n5;
        locals.var_t5_dn6 = assign12230_e11347_d_n6;
        locals.var_t5_dn7 = assign12230_e11347_d_n7;
        locals.var_t5_dn8 = assign12230_e11347_d_n8;
        locals.var_t5_rv = 0.0;

        let (assign12260_e11377, assign12260_e11377_d_n3, assign12260_e11377_d_n4, assign12260_e11377_d_n5, assign12260_e11377_d_n6, assign12260_e11377_d_n7, assign12260_e11377_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12260_e11367: f64 = (locals.var_vgs_noswap - locals.var_vfbsd);
        let assign12260_e11370: f64 = (locals.var_digs_i * locals.var_gamma0);
        let assign12260_e11373: f64 = (locals.var_vbgs - locals.var_vfbsd_bg);
        let assign12260_e11374: f64 = (assign12260_e11370 * assign12260_e11373);
        let assign12260_e11375: f64 = (assign12260_e11367 + assign12260_e11374);
        (assign12260_e11375, ((-locals.var_vfbsd_dn3) + (assign12260_e11370 * (locals.var_vbgs_dn3 - locals.var_vfbsd_bg_dn3))), ((-locals.var_vfbsd_dn4) + (assign12260_e11370 * (-locals.var_vfbsd_bg_dn4))), ((-locals.var_vfbsd_dn5) + (assign12260_e11370 * (locals.var_vbgs_dn5 - locals.var_vfbsd_bg_dn5))), ((locals.var_vgs_noswap_dn6 - locals.var_vfbsd_dn6) + (assign12260_e11370 * (locals.var_vbgs_dn6 - locals.var_vfbsd_bg_dn6))), ((-locals.var_vfbsd_dn7) + (assign12260_e11370 * (-locals.var_vfbsd_bg_dn7))), ((locals.var_vgs_noswap_dn8 - locals.var_vfbsd_dn8) + (assign12260_e11370 * (-locals.var_vfbsd_bg_dn8))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    }
};
        locals.var_t0 = assign12260_e11377;
        locals.var_t0_dn3 = assign12260_e11377_d_n3;
        locals.var_t0_dn4 = assign12260_e11377_d_n4;
        locals.var_t0_dn5 = assign12260_e11377_d_n5;
        locals.var_t0_dn6 = assign12260_e11377_d_n6;
        locals.var_t0_dn7 = assign12260_e11377_d_n7;
        locals.var_t0_dn8 = assign12260_e11377_d_n8;
        locals.var_t0_rv = 0.0;

        let (assign12270_e11386, assign12270_e11386_d_n3, assign12270_e11386_d_n4, assign12270_e11386_d_n5, assign12270_e11386_d_n6, assign12270_e11386_d_n7, assign12270_e11386_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12270_e11381: f64 = (locals.var_t0 * locals.var_t0);
        let assign12270_e11383: f64 = (assign12270_e11381 + 0.0001);
        let assign12270_e11384: f64 = (assign12270_e11383).sqrt();
        (assign12270_e11384, (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign12270_e11384)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign12270_e11384)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign12270_e11384)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign12270_e11384)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign12270_e11384)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign12270_e11384)),)
    } else {
        (locals.var_vfgs_eff, locals.var_vfgs_eff_dn3, locals.var_vfgs_eff_dn4, locals.var_vfgs_eff_dn5, locals.var_vfgs_eff_dn6, locals.var_vfgs_eff_dn7, locals.var_vfgs_eff_dn8,)
    }
};
        locals.var_vfgs_eff = assign12270_e11386;
        locals.var_vfgs_eff_dn3 = assign12270_e11386_d_n3;
        locals.var_vfgs_eff_dn4 = assign12270_e11386_d_n4;
        locals.var_vfgs_eff_dn5 = assign12270_e11386_d_n5;
        locals.var_vfgs_eff_dn6 = assign12270_e11386_d_n6;
        locals.var_vfgs_eff_dn7 = assign12270_e11386_d_n7;
        locals.var_vfgs_eff_dn8 = assign12270_e11386_d_n8;
        locals.var_vfgs_eff_rv = 0.0;

        let (assign12280_e11394, assign12280_e11394_d_n3, assign12280_e11394_d_n4, assign12280_e11394_d_n5, assign12280_e11394_d_n6, assign12280_e11394_d_n7, assign12280_e11394_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12280_e11391: f64 = (locals.var_bigs_i * locals.var_vfgs_eff);
        let assign12280_e11392: f64 = (locals.var_aigs_i - assign12280_e11391);
        (assign12280_e11392, (-(locals.var_bigs_i * locals.var_vfgs_eff_dn3)), (-(locals.var_bigs_i * locals.var_vfgs_eff_dn4)), (-(locals.var_bigs_i * locals.var_vfgs_eff_dn5)), (-(locals.var_bigs_i * locals.var_vfgs_eff_dn6)), (-(locals.var_bigs_i * locals.var_vfgs_eff_dn7)), (-(locals.var_bigs_i * locals.var_vfgs_eff_dn8)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign12280_e11394;
        locals.var_t1_dn3 = assign12280_e11394_d_n3;
        locals.var_t1_dn4 = assign12280_e11394_d_n4;
        locals.var_t1_dn5 = assign12280_e11394_d_n5;
        locals.var_t1_dn6 = assign12280_e11394_d_n6;
        locals.var_t1_dn7 = assign12280_e11394_d_n7;
        locals.var_t1_dn8 = assign12280_e11394_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign12290_e11402, assign12290_e11402_d_n3, assign12290_e11402_d_n4, assign12290_e11402_d_n5, assign12290_e11402_d_n6, assign12290_e11402_d_n7, assign12290_e11402_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12290_e11399: f64 = (locals.var_cigs_i * locals.var_vfgs_eff);
        let assign12290_e11400: f64 = (1.0 + assign12290_e11399);
        (assign12290_e11400, (locals.var_cigs_i * locals.var_vfgs_eff_dn3), (locals.var_cigs_i * locals.var_vfgs_eff_dn4), (locals.var_cigs_i * locals.var_vfgs_eff_dn5), (locals.var_cigs_i * locals.var_vfgs_eff_dn6), (locals.var_cigs_i * locals.var_vfgs_eff_dn7), (locals.var_cigs_i * locals.var_vfgs_eff_dn8),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8,)
    }
};
        locals.var_t2 = assign12290_e11402;
        locals.var_t2_dn3 = assign12290_e11402_d_n3;
        locals.var_t2_dn4 = assign12290_e11402_d_n4;
        locals.var_t2_dn5 = assign12290_e11402_d_n5;
        locals.var_t2_dn6 = assign12290_e11402_d_n6;
        locals.var_t2_dn7 = assign12290_e11402_d_n7;
        locals.var_t2_dn8 = assign12290_e11402_d_n8;
        locals.var_t2_rv = 0.0;

        let (assign12300_e11415, assign12300_e11415_d_n3, assign12300_e11415_d_n4, assign12300_e11415_d_n5, assign12300_e11415_d_n6, assign12300_e11415_d_n7, assign12300_e11415_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12300_e11405: f64 = (-locals.var_bechvb);
        let assign12300_e11407: f64 = (assign12300_e11405 * p.p99);
        let assign12300_e11409: f64 = (assign12300_e11407 * locals.var_poxedge_i);
        let assign12300_e11411: f64 = (assign12300_e11409 * locals.var_t1);
        let assign12300_e11413: f64 = (assign12300_e11411 * locals.var_t2);
        (assign12300_e11413, (((assign12300_e11409 * locals.var_t1_dn3) * locals.var_t2) + (assign12300_e11411 * locals.var_t2_dn3)), (((assign12300_e11409 * locals.var_t1_dn4) * locals.var_t2) + (assign12300_e11411 * locals.var_t2_dn4)), (((assign12300_e11409 * locals.var_t1_dn5) * locals.var_t2) + (assign12300_e11411 * locals.var_t2_dn5)), (((assign12300_e11409 * locals.var_t1_dn6) * locals.var_t2) + (assign12300_e11411 * locals.var_t2_dn6)), (((assign12300_e11409 * locals.var_t1_dn7) * locals.var_t2) + (assign12300_e11411 * locals.var_t2_dn7)), (((assign12300_e11409 * locals.var_t1_dn8) * locals.var_t2) + (assign12300_e11411 * locals.var_t2_dn8)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8,)
    }
};
        locals.var_t3 = assign12300_e11415;
        locals.var_t3_dn3 = assign12300_e11415_d_n3;
        locals.var_t3_dn4 = assign12300_e11415_d_n4;
        locals.var_t3_dn5 = assign12300_e11415_d_n5;
        locals.var_t3_dn6 = assign12300_e11415_d_n6;
        locals.var_t3_dn7 = assign12300_e11415_d_n7;
        locals.var_t3_dn8 = assign12300_e11415_d_n8;
        locals.var_t3_rv = 0.0;

        let (assign12310_e11420, assign12310_e11420_d_n3, assign12310_e11420_d_n4, assign12310_e11420_d_n5, assign12310_e11420_d_n6, assign12310_e11420_d_n7, assign12310_e11420_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12310_e11418: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign12310_e11418, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8,)
    }
};
        locals.var_t4 = assign12310_e11420;
        locals.var_t4_dn3 = assign12310_e11420_d_n3;
        locals.var_t4_dn4 = assign12310_e11420_d_n4;
        locals.var_t4_dn5 = assign12310_e11420_d_n5;
        locals.var_t4_dn6 = assign12310_e11420_d_n6;
        locals.var_t4_dn7 = assign12310_e11420_d_n7;
        locals.var_t4_dn8 = assign12310_e11420_d_n8;
        locals.var_t4_rv = 0.0;

        let (assign12350_e11466, assign12350_e11466_d_n3, assign12350_e11466_d_n4, assign12350_e11466_d_n5, assign12350_e11466_d_n6, assign12350_e11466_d_n7, assign12350_e11466_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12350_e11456: f64 = (locals.var_vgd_noswap - locals.var_vfbsd);
        let assign12350_e11459: f64 = (locals.var_digd_i * locals.var_gamma0);
        let assign12350_e11462: f64 = (locals.var_vbgs - locals.var_vfbsd_bg);
        let assign12350_e11463: f64 = (assign12350_e11459 * assign12350_e11462);
        let assign12350_e11464: f64 = (assign12350_e11456 + assign12350_e11463);
        (assign12350_e11464, ((-locals.var_vfbsd_dn3) + (assign12350_e11459 * (locals.var_vbgs_dn3 - locals.var_vfbsd_bg_dn3))), ((-locals.var_vfbsd_dn4) + (assign12350_e11459 * (-locals.var_vfbsd_bg_dn4))), ((locals.var_vgd_noswap_dn5 - locals.var_vfbsd_dn5) + (assign12350_e11459 * (locals.var_vbgs_dn5 - locals.var_vfbsd_bg_dn5))), ((-locals.var_vfbsd_dn6) + (assign12350_e11459 * (locals.var_vbgs_dn6 - locals.var_vfbsd_bg_dn6))), ((-locals.var_vfbsd_dn7) + (assign12350_e11459 * (-locals.var_vfbsd_bg_dn7))), ((locals.var_vgd_noswap_dn8 - locals.var_vfbsd_dn8) + (assign12350_e11459 * (-locals.var_vfbsd_bg_dn8))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    }
};
        locals.var_t0 = assign12350_e11466;
        locals.var_t0_dn3 = assign12350_e11466_d_n3;
        locals.var_t0_dn4 = assign12350_e11466_d_n4;
        locals.var_t0_dn5 = assign12350_e11466_d_n5;
        locals.var_t0_dn6 = assign12350_e11466_d_n6;
        locals.var_t0_dn7 = assign12350_e11466_d_n7;
        locals.var_t0_dn8 = assign12350_e11466_d_n8;
        locals.var_t0_rv = 0.0;

        let (assign12360_e11475, assign12360_e11475_d_n3, assign12360_e11475_d_n4, assign12360_e11475_d_n5, assign12360_e11475_d_n6, assign12360_e11475_d_n7, assign12360_e11475_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12360_e11470: f64 = (locals.var_t0 * locals.var_t0);
        let assign12360_e11472: f64 = (assign12360_e11470 + 0.0001);
        let assign12360_e11473: f64 = (assign12360_e11472).sqrt();
        (assign12360_e11473, (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign12360_e11473)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign12360_e11473)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign12360_e11473)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign12360_e11473)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign12360_e11473)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign12360_e11473)),)
    } else {
        (locals.var_vfgd_eff, locals.var_vfgd_eff_dn3, locals.var_vfgd_eff_dn4, locals.var_vfgd_eff_dn5, locals.var_vfgd_eff_dn6, locals.var_vfgd_eff_dn7, locals.var_vfgd_eff_dn8,)
    }
};
        locals.var_vfgd_eff = assign12360_e11475;
        locals.var_vfgd_eff_dn3 = assign12360_e11475_d_n3;
        locals.var_vfgd_eff_dn4 = assign12360_e11475_d_n4;
        locals.var_vfgd_eff_dn5 = assign12360_e11475_d_n5;
        locals.var_vfgd_eff_dn6 = assign12360_e11475_d_n6;
        locals.var_vfgd_eff_dn7 = assign12360_e11475_d_n7;
        locals.var_vfgd_eff_dn8 = assign12360_e11475_d_n8;
        locals.var_vfgd_eff_rv = 0.0;

        let (assign12370_e11483, assign12370_e11483_d_n3, assign12370_e11483_d_n4, assign12370_e11483_d_n5, assign12370_e11483_d_n6, assign12370_e11483_d_n7, assign12370_e11483_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12370_e11480: f64 = (locals.var_bigd_i * locals.var_vfgd_eff);
        let assign12370_e11481: f64 = (locals.var_aigd_i - assign12370_e11480);
        (assign12370_e11481, (-(locals.var_bigd_i * locals.var_vfgd_eff_dn3)), (-(locals.var_bigd_i * locals.var_vfgd_eff_dn4)), (-(locals.var_bigd_i * locals.var_vfgd_eff_dn5)), (-(locals.var_bigd_i * locals.var_vfgd_eff_dn6)), (-(locals.var_bigd_i * locals.var_vfgd_eff_dn7)), (-(locals.var_bigd_i * locals.var_vfgd_eff_dn8)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign12370_e11483;
        locals.var_t1_dn3 = assign12370_e11483_d_n3;
        locals.var_t1_dn4 = assign12370_e11483_d_n4;
        locals.var_t1_dn5 = assign12370_e11483_d_n5;
        locals.var_t1_dn6 = assign12370_e11483_d_n6;
        locals.var_t1_dn7 = assign12370_e11483_d_n7;
        locals.var_t1_dn8 = assign12370_e11483_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign12380_e11491, assign12380_e11491_d_n3, assign12380_e11491_d_n4, assign12380_e11491_d_n5, assign12380_e11491_d_n6, assign12380_e11491_d_n7, assign12380_e11491_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12380_e11488: f64 = (locals.var_cigd_i * locals.var_vfgd_eff);
        let assign12380_e11489: f64 = (1.0 + assign12380_e11488);
        (assign12380_e11489, (locals.var_cigd_i * locals.var_vfgd_eff_dn3), (locals.var_cigd_i * locals.var_vfgd_eff_dn4), (locals.var_cigd_i * locals.var_vfgd_eff_dn5), (locals.var_cigd_i * locals.var_vfgd_eff_dn6), (locals.var_cigd_i * locals.var_vfgd_eff_dn7), (locals.var_cigd_i * locals.var_vfgd_eff_dn8),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8,)
    }
};
        locals.var_t2 = assign12380_e11491;
        locals.var_t2_dn3 = assign12380_e11491_d_n3;
        locals.var_t2_dn4 = assign12380_e11491_d_n4;
        locals.var_t2_dn5 = assign12380_e11491_d_n5;
        locals.var_t2_dn6 = assign12380_e11491_d_n6;
        locals.var_t2_dn7 = assign12380_e11491_d_n7;
        locals.var_t2_dn8 = assign12380_e11491_d_n8;
        locals.var_t2_rv = 0.0;

        let (assign12390_e11504, assign12390_e11504_d_n3, assign12390_e11504_d_n4, assign12390_e11504_d_n5, assign12390_e11504_d_n6, assign12390_e11504_d_n7, assign12390_e11504_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12390_e11494: f64 = (-locals.var_bechvb);
        let assign12390_e11496: f64 = (assign12390_e11494 * p.p99);
        let assign12390_e11498: f64 = (assign12390_e11496 * locals.var_poxedge_i);
        let assign12390_e11500: f64 = (assign12390_e11498 * locals.var_t1);
        let assign12390_e11502: f64 = (assign12390_e11500 * locals.var_t2);
        (assign12390_e11502, (((assign12390_e11498 * locals.var_t1_dn3) * locals.var_t2) + (assign12390_e11500 * locals.var_t2_dn3)), (((assign12390_e11498 * locals.var_t1_dn4) * locals.var_t2) + (assign12390_e11500 * locals.var_t2_dn4)), (((assign12390_e11498 * locals.var_t1_dn5) * locals.var_t2) + (assign12390_e11500 * locals.var_t2_dn5)), (((assign12390_e11498 * locals.var_t1_dn6) * locals.var_t2) + (assign12390_e11500 * locals.var_t2_dn6)), (((assign12390_e11498 * locals.var_t1_dn7) * locals.var_t2) + (assign12390_e11500 * locals.var_t2_dn7)), (((assign12390_e11498 * locals.var_t1_dn8) * locals.var_t2) + (assign12390_e11500 * locals.var_t2_dn8)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8,)
    }
};
        locals.var_t3 = assign12390_e11504;
        locals.var_t3_dn3 = assign12390_e11504_d_n3;
        locals.var_t3_dn4 = assign12390_e11504_d_n4;
        locals.var_t3_dn5 = assign12390_e11504_d_n5;
        locals.var_t3_dn6 = assign12390_e11504_d_n6;
        locals.var_t3_dn7 = assign12390_e11504_d_n7;
        locals.var_t3_dn8 = assign12390_e11504_d_n8;
        locals.var_t3_rv = 0.0;

        let (assign12400_e11509, assign12400_e11509_d_n3, assign12400_e11509_d_n4, assign12400_e11509_d_n5, assign12400_e11509_d_n6, assign12400_e11509_d_n7, assign12400_e11509_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign12400_e11507: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign12400_e11507, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8,)
    }
};
        locals.var_t4 = assign12400_e11509;
        locals.var_t4_dn3 = assign12400_e11509_d_n3;
        locals.var_t4_dn4 = assign12400_e11509_d_n4;
        locals.var_t4_dn5 = assign12400_e11509_d_n5;
        locals.var_t4_dn6 = assign12400_e11509_d_n6;
        locals.var_t4_dn7 = assign12400_e11509_d_n7;
        locals.var_t4_dn8 = assign12400_e11509_d_n8;
        locals.var_t4_rv = 0.0;

        let assign12460_e11546: f64 = if p.p15 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard126 = assign12460_e11546;
        locals.var_guard126_rv = 0.0;

        let (assign12470_e11552, assign12470_e11552_d_n3, assign12470_e11552_d_n4, assign12470_e11552_d_n5, assign12470_e11552_d_n6, assign12470_e11552_d_n7, assign12470_e11552_d_n8,) = {
    if (locals.var_guard126 != 0.0) {
        let assign12470_e11550: f64 = (locals.var_epsratio * p.p45);
        (assign12470_e11550, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    }
};
        locals.var_t0 = assign12470_e11552;
        locals.var_t0_dn3 = assign12470_e11552_d_n3;
        locals.var_t0_dn4 = assign12470_e11552_d_n4;
        locals.var_t0_dn5 = assign12470_e11552_d_n5;
        locals.var_t0_dn6 = assign12470_e11552_d_n6;
        locals.var_t0_dn7 = assign12470_e11552_d_n7;
        locals.var_t0_dn8 = assign12470_e11552_d_n8;
        locals.var_t0_rv = 0.0;

        let assign12480_e11559: f64 = if ((locals.var_agidl_i <= 0.0) || (locals.var_bgidl_t <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard127 = assign12480_e11559;
        locals.var_guard127_rv = 0.0;

        let (assign12490_e11565, assign12490_e11565_d_n3, assign12490_e11565_d_n4, assign12490_e11565_d_n5, assign12490_e11565_d_n6, assign12490_e11565_d_n7, assign12490_e11565_d_n8,) = {
    if ((locals.var_guard126 != 0.0) && (locals.var_guard127 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8,)
    }
};
        locals.var_t6 = assign12490_e11565;
        locals.var_t6_dn3 = assign12490_e11565_d_n3;
        locals.var_t6_dn4 = assign12490_e11565_d_n4;
        locals.var_t6_dn5 = assign12490_e11565_d_n5;
        locals.var_t6_dn6 = assign12490_e11565_d_n6;
        locals.var_t6_dn7 = assign12490_e11565_d_n7;
        locals.var_t6_dn8 = assign12490_e11565_d_n8;
        locals.var_t6_rv = 0.0;

        let (assign12500_e11589, assign12500_e11589_d_n3, assign12500_e11589_d_n4, assign12500_e11589_d_n5, assign12500_e11589_d_n6, assign12500_e11589_d_n7, assign12500_e11589_d_n8,) = {
    if ((locals.var_guard126 != 0.0) && (locals.var_guard127 == 0.0)) {
        let assign12500_e11571: f64 = (-locals.var_vgd_noswap);
        let assign12500_e11573: f64 = (assign12500_e11571 - locals.var_egidl_i);
        let assign12500_e11575: f64 = (assign12500_e11573 + locals.var_vfbsd);
        let assign12500_e11578: f64 = (locals.var_vbgidl_i * locals.var_gamma0);
        let assign12500_e11581: f64 = (locals.var_vbgs - locals.var_vfbsd_bg);
        let assign12500_e11583: f64 = (assign12500_e11581 - locals.var_vbegidl_i);
        let assign12500_e11584: f64 = (assign12500_e11578 * assign12500_e11583);
        let assign12500_e11585: f64 = (assign12500_e11575 + assign12500_e11584);
        let assign12500_e11587: f64 = (assign12500_e11585 / locals.var_t0);
        (assign12500_e11587, ((((locals.var_vfbsd_dn3 + (assign12500_e11578 * (locals.var_vbgs_dn3 - locals.var_vfbsd_bg_dn3))) * locals.var_t0) - (assign12500_e11585 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), ((((locals.var_vfbsd_dn4 + (assign12500_e11578 * (-locals.var_vfbsd_bg_dn4))) * locals.var_t0) - (assign12500_e11585 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), ((((((-locals.var_vgd_noswap_dn5) + locals.var_vfbsd_dn5) + (assign12500_e11578 * (locals.var_vbgs_dn5 - locals.var_vfbsd_bg_dn5))) * locals.var_t0) - (assign12500_e11585 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((locals.var_vfbsd_dn6 + (assign12500_e11578 * (locals.var_vbgs_dn6 - locals.var_vfbsd_bg_dn6))) * locals.var_t0) - (assign12500_e11585 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((locals.var_vfbsd_dn7 + (assign12500_e11578 * (-locals.var_vfbsd_bg_dn7))) * locals.var_t0) - (assign12500_e11585 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((((-locals.var_vgd_noswap_dn8) + locals.var_vfbsd_dn8) + (assign12500_e11578 * (-locals.var_vfbsd_bg_dn8))) * locals.var_t0) - (assign12500_e11585 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign12500_e11589;
        locals.var_t1_dn3 = assign12500_e11589_d_n3;
        locals.var_t1_dn4 = assign12500_e11589_d_n4;
        locals.var_t1_dn5 = assign12500_e11589_d_n5;
        locals.var_t1_dn6 = assign12500_e11589_d_n6;
        locals.var_t1_dn7 = assign12500_e11589_d_n7;
        locals.var_t1_dn8 = assign12500_e11589_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign12510_e11609, assign12510_e11609_d_n3, assign12510_e11609_d_n4, assign12510_e11609_d_n5, assign12510_e11609_d_n6, assign12510_e11609_d_n7, assign12510_e11609_d_n8,) = {
    if ((locals.var_guard126 != 0.0) && (locals.var_guard127 == 0.0)) {
        let assign12510_e11598: f64 = (locals.var_t1 * locals.var_t1);
        let assign12510_e11601: f64 = (4.0 * 0.01);
        let assign12510_e11603: f64 = (assign12510_e11601 * 0.01);
        let assign12510_e11604: f64 = (assign12510_e11598 + assign12510_e11603);
        let assign12510_e11605: f64 = (assign12510_e11604).sqrt();
        let assign12510_e11606: f64 = (locals.var_t1 + assign12510_e11605);
        let assign12510_e11607: f64 = (0.5 * assign12510_e11606);
        (assign12510_e11607, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign12510_e11605)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign12510_e11605)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign12510_e11605)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign12510_e11605)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign12510_e11605)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign12510_e11605)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign12510_e11609;
        locals.var_t1_dn3 = assign12510_e11609_d_n3;
        locals.var_t1_dn4 = assign12510_e11609_d_n4;
        locals.var_t1_dn5 = assign12510_e11609_d_n5;
        locals.var_t1_dn6 = assign12510_e11609_d_n6;
        locals.var_t1_dn7 = assign12510_e11609_d_n7;
        locals.var_t1_dn8 = assign12510_e11609_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign12520_e11620, assign12520_e11620_d_n3, assign12520_e11620_d_n4, assign12520_e11620_d_n5, assign12520_e11620_d_n6, assign12520_e11620_d_n7, assign12520_e11620_d_n8,) = {
    if ((locals.var_guard126 != 0.0) && (locals.var_guard127 == 0.0)) {
        let assign12520_e11617: f64 = (locals.var_t1 + 0.001);
        let assign12520_e11618: f64 = (locals.var_bgidl_t / assign12520_e11617);
        (assign12520_e11618, (-((locals.var_bgidl_t * locals.var_t1_dn3) / (assign12520_e11617 * assign12520_e11617))), (((locals.var_bgidl_t_dn4 * assign12520_e11617) - (locals.var_bgidl_t * locals.var_t1_dn4)) / (assign12520_e11617 * assign12520_e11617)), (-((locals.var_bgidl_t * locals.var_t1_dn5) / (assign12520_e11617 * assign12520_e11617))), (-((locals.var_bgidl_t * locals.var_t1_dn6) / (assign12520_e11617 * assign12520_e11617))), (-((locals.var_bgidl_t * locals.var_t1_dn7) / (assign12520_e11617 * assign12520_e11617))), (-((locals.var_bgidl_t * locals.var_t1_dn8) / (assign12520_e11617 * assign12520_e11617))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8,)
    }
};
        locals.var_t2 = assign12520_e11620;
        locals.var_t2_dn3 = assign12520_e11620_d_n3;
        locals.var_t2_dn4 = assign12520_e11620_d_n4;
        locals.var_t2_dn5 = assign12520_e11620_d_n5;
        locals.var_t2_dn6 = assign12520_e11620_d_n6;
        locals.var_t2_dn7 = assign12520_e11620_d_n7;
        locals.var_t2_dn8 = assign12520_e11620_d_n8;
        locals.var_t2_rv = 0.0;

        let (assign12530_e11633, assign12530_e11633_d_n3, assign12530_e11633_d_n4, assign12530_e11633_d_n5, assign12530_e11633_d_n6, assign12530_e11633_d_n7, assign12530_e11633_d_n8,) = {
    if ((locals.var_guard126 != 0.0) && (locals.var_guard127 == 0.0)) {
        let assign12530_e11628: f64 = (locals.var_t1).max(1e-38);
        let assign12530_e11629: f64 = (assign12530_e11628).ln();
        let assign12530_e11630: f64 = (locals.var_pgidl_i * assign12530_e11629);
        let assign12530_e11631: f64 = { let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign12530_e11631, ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_pgidl_i * (if locals.var_t1 >= 1e-38 { locals.var_t1_dn3 } else { 0.0 } / assign12530_e11628))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_pgidl_i * (if locals.var_t1 >= 1e-38 { locals.var_t1_dn4 } else { 0.0 } / assign12530_e11628))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_pgidl_i * (if locals.var_t1 >= 1e-38 { locals.var_t1_dn5 } else { 0.0 } / assign12530_e11628))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_pgidl_i * (if locals.var_t1 >= 1e-38 { locals.var_t1_dn6 } else { 0.0 } / assign12530_e11628))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_pgidl_i * (if locals.var_t1 >= 1e-38 { locals.var_t1_dn7 } else { 0.0 } / assign12530_e11628))), ({ let limited_exp_arg = assign12530_e11630; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_pgidl_i * (if locals.var_t1 >= 1e-38 { locals.var_t1_dn8 } else { 0.0 } / assign12530_e11628))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8,)
    }
};
        locals.var_t3 = assign12530_e11633;
        locals.var_t3_dn3 = assign12530_e11633_d_n3;
        locals.var_t3_dn4 = assign12530_e11633_d_n4;
        locals.var_t3_dn5 = assign12530_e11633_d_n5;
        locals.var_t3_dn6 = assign12530_e11633_d_n6;
        locals.var_t3_dn7 = assign12530_e11633_d_n7;
        locals.var_t3_dn8 = assign12530_e11633_d_n8;
        locals.var_t3_rv = 0.0;

        let (assign12540_e11650, assign12540_e11650_d_n3, assign12540_e11650_d_n4, assign12540_e11650_d_n5, assign12540_e11650_d_n6, assign12540_e11650_d_n7, assign12540_e11650_d_n8,) = {
    if ((locals.var_guard126 != 0.0) && (locals.var_guard127 == 0.0)) {
        let assign12540_e11640: f64 = (locals.var_agidl_i * locals.var_weff);
        let assign12540_e11642: f64 = (assign12540_e11640 * locals.var_t3);
        let assign12540_e11644: f64 = (-locals.var_t2);
        let assign12540_e11645: f64 = { let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign12540_e11646: f64 = (assign12540_e11642 * assign12540_e11645);
        let assign12540_e11648: f64 = (assign12540_e11646 * locals.var_vds_noswap);
        (assign12540_e11648, ((((assign12540_e11640 * locals.var_t3_dn3) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * locals.var_vds_noswap), ((((assign12540_e11640 * locals.var_t3_dn4) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * locals.var_vds_noswap), (((((assign12540_e11640 * locals.var_t3_dn5) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * locals.var_vds_noswap) + (assign12540_e11646 * locals.var_vds_noswap_dn5)), (((((assign12540_e11640 * locals.var_t3_dn6) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * locals.var_vds_noswap) + (assign12540_e11646 * locals.var_vds_noswap_dn6)), ((((assign12540_e11640 * locals.var_t3_dn7) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * locals.var_vds_noswap), ((((assign12540_e11640 * locals.var_t3_dn8) * assign12540_e11645) + (assign12540_e11642 * ({ let limited_exp_arg = assign12540_e11644; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * locals.var_vds_noswap),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8,)
    }
};
        locals.var_t6 = assign12540_e11650;
        locals.var_t6_dn3 = assign12540_e11650_d_n3;
        locals.var_t6_dn4 = assign12540_e11650_d_n4;
        locals.var_t6_dn5 = assign12540_e11650_d_n5;
        locals.var_t6_dn6 = assign12540_e11650_d_n6;
        locals.var_t6_dn7 = assign12540_e11650_d_n7;
        locals.var_t6_dn8 = assign12540_e11650_d_n8;
        locals.var_t6_rv = 0.0;

        let assign12580_e11673: f64 = if ((locals.var_agisl_i <= 0.0) || (locals.var_bgisl_t <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard129 = assign12580_e11673;
        locals.var_guard129_rv = 0.0;

        let (assign12590_e11679, assign12590_e11679_d_n3, assign12590_e11679_d_n4, assign12590_e11679_d_n5, assign12590_e11679_d_n6, assign12590_e11679_d_n7, assign12590_e11679_d_n8,) = {
    if ((locals.var_guard126 != 0.0) && (locals.var_guard129 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8,)
    }
};
        locals.var_t6 = assign12590_e11679;
        locals.var_t6_dn3 = assign12590_e11679_d_n3;
        locals.var_t6_dn4 = assign12590_e11679_d_n4;
        locals.var_t6_dn5 = assign12590_e11679_d_n5;
        locals.var_t6_dn6 = assign12590_e11679_d_n6;
        locals.var_t6_dn7 = assign12590_e11679_d_n7;
        locals.var_t6_dn8 = assign12590_e11679_d_n8;
        locals.var_t6_rv = 0.0;

        let (assign12600_e11703, assign12600_e11703_d_n3, assign12600_e11703_d_n4, assign12600_e11703_d_n5, assign12600_e11703_d_n6, assign12600_e11703_d_n7, assign12600_e11703_d_n8,) = {
    if ((locals.var_guard126 != 0.0) && (locals.var_guard129 == 0.0)) {
        let assign12600_e11685: f64 = (-locals.var_vgs_noswap);
        let assign12600_e11687: f64 = (assign12600_e11685 - locals.var_egisl_i);
        let assign12600_e11689: f64 = (assign12600_e11687 + locals.var_vfbsd);
        let assign12600_e11692: f64 = (locals.var_vbgisl_i * locals.var_gamma0);
        let assign12600_e11695: f64 = (locals.var_vbgs - locals.var_vfbsd_bg);
        let assign12600_e11697: f64 = (assign12600_e11695 - locals.var_vbegisl_i);
        let assign12600_e11698: f64 = (assign12600_e11692 * assign12600_e11697);
        let assign12600_e11699: f64 = (assign12600_e11689 + assign12600_e11698);
        let assign12600_e11701: f64 = (assign12600_e11699 / locals.var_t0);
        (assign12600_e11701, ((((locals.var_vfbsd_dn3 + (assign12600_e11692 * (locals.var_vbgs_dn3 - locals.var_vfbsd_bg_dn3))) * locals.var_t0) - (assign12600_e11699 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), ((((locals.var_vfbsd_dn4 + (assign12600_e11692 * (-locals.var_vfbsd_bg_dn4))) * locals.var_t0) - (assign12600_e11699 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), ((((locals.var_vfbsd_dn5 + (assign12600_e11692 * (locals.var_vbgs_dn5 - locals.var_vfbsd_bg_dn5))) * locals.var_t0) - (assign12600_e11699 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((((-locals.var_vgs_noswap_dn6) + locals.var_vfbsd_dn6) + (assign12600_e11692 * (locals.var_vbgs_dn6 - locals.var_vfbsd_bg_dn6))) * locals.var_t0) - (assign12600_e11699 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((locals.var_vfbsd_dn7 + (assign12600_e11692 * (-locals.var_vfbsd_bg_dn7))) * locals.var_t0) - (assign12600_e11699 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((((-locals.var_vgs_noswap_dn8) + locals.var_vfbsd_dn8) + (assign12600_e11692 * (-locals.var_vfbsd_bg_dn8))) * locals.var_t0) - (assign12600_e11699 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign12600_e11703;
        locals.var_t1_dn3 = assign12600_e11703_d_n3;
        locals.var_t1_dn4 = assign12600_e11703_d_n4;
        locals.var_t1_dn5 = assign12600_e11703_d_n5;
        locals.var_t1_dn6 = assign12600_e11703_d_n6;
        locals.var_t1_dn7 = assign12600_e11703_d_n7;
        locals.var_t1_dn8 = assign12600_e11703_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign12610_e11723, assign12610_e11723_d_n3, assign12610_e11723_d_n4, assign12610_e11723_d_n5, assign12610_e11723_d_n6, assign12610_e11723_d_n7, assign12610_e11723_d_n8,) = {
    if ((locals.var_guard126 != 0.0) && (locals.var_guard129 == 0.0)) {
        let assign12610_e11712: f64 = (locals.var_t1 * locals.var_t1);
        let assign12610_e11715: f64 = (4.0 * 0.01);
        let assign12610_e11717: f64 = (assign12610_e11715 * 0.01);
        let assign12610_e11718: f64 = (assign12610_e11712 + assign12610_e11717);
        let assign12610_e11719: f64 = (assign12610_e11718).sqrt();
        let assign12610_e11720: f64 = (locals.var_t1 + assign12610_e11719);
        let assign12610_e11721: f64 = (0.5 * assign12610_e11720);
        (assign12610_e11721, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign12610_e11719)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign12610_e11719)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign12610_e11719)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign12610_e11719)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign12610_e11719)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign12610_e11719)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign12610_e11723;
        locals.var_t1_dn3 = assign12610_e11723_d_n3;
        locals.var_t1_dn4 = assign12610_e11723_d_n4;
        locals.var_t1_dn5 = assign12610_e11723_d_n5;
        locals.var_t1_dn6 = assign12610_e11723_d_n6;
        locals.var_t1_dn7 = assign12610_e11723_d_n7;
        locals.var_t1_dn8 = assign12610_e11723_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign12620_e11734, assign12620_e11734_d_n3, assign12620_e11734_d_n4, assign12620_e11734_d_n5, assign12620_e11734_d_n6, assign12620_e11734_d_n7, assign12620_e11734_d_n8,) = {
    if ((locals.var_guard126 != 0.0) && (locals.var_guard129 == 0.0)) {
        let assign12620_e11731: f64 = (locals.var_t1 + 0.001);
        let assign12620_e11732: f64 = (locals.var_bgisl_t / assign12620_e11731);
        (assign12620_e11732, (-((locals.var_bgisl_t * locals.var_t1_dn3) / (assign12620_e11731 * assign12620_e11731))), (((locals.var_bgisl_t_dn4 * assign12620_e11731) - (locals.var_bgisl_t * locals.var_t1_dn4)) / (assign12620_e11731 * assign12620_e11731)), (-((locals.var_bgisl_t * locals.var_t1_dn5) / (assign12620_e11731 * assign12620_e11731))), (-((locals.var_bgisl_t * locals.var_t1_dn6) / (assign12620_e11731 * assign12620_e11731))), (-((locals.var_bgisl_t * locals.var_t1_dn7) / (assign12620_e11731 * assign12620_e11731))), (-((locals.var_bgisl_t * locals.var_t1_dn8) / (assign12620_e11731 * assign12620_e11731))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8,)
    }
};
        locals.var_t2 = assign12620_e11734;
        locals.var_t2_dn3 = assign12620_e11734_d_n3;
        locals.var_t2_dn4 = assign12620_e11734_d_n4;
        locals.var_t2_dn5 = assign12620_e11734_d_n5;
        locals.var_t2_dn6 = assign12620_e11734_d_n6;
        locals.var_t2_dn7 = assign12620_e11734_d_n7;
        locals.var_t2_dn8 = assign12620_e11734_d_n8;
        locals.var_t2_rv = 0.0;

        let (assign12630_e11747, assign12630_e11747_d_n3, assign12630_e11747_d_n4, assign12630_e11747_d_n5, assign12630_e11747_d_n6, assign12630_e11747_d_n7, assign12630_e11747_d_n8,) = {
    if ((locals.var_guard126 != 0.0) && (locals.var_guard129 == 0.0)) {
        let assign12630_e11742: f64 = (locals.var_t1).max(1e-38);
        let assign12630_e11743: f64 = (assign12630_e11742).ln();
        let assign12630_e11744: f64 = (locals.var_pgisl_i * assign12630_e11743);
        let assign12630_e11745: f64 = { let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign12630_e11745, ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_pgisl_i * (if locals.var_t1 >= 1e-38 { locals.var_t1_dn3 } else { 0.0 } / assign12630_e11742))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_pgisl_i * (if locals.var_t1 >= 1e-38 { locals.var_t1_dn4 } else { 0.0 } / assign12630_e11742))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_pgisl_i * (if locals.var_t1 >= 1e-38 { locals.var_t1_dn5 } else { 0.0 } / assign12630_e11742))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_pgisl_i * (if locals.var_t1 >= 1e-38 { locals.var_t1_dn6 } else { 0.0 } / assign12630_e11742))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_pgisl_i * (if locals.var_t1 >= 1e-38 { locals.var_t1_dn7 } else { 0.0 } / assign12630_e11742))), ({ let limited_exp_arg = assign12630_e11744; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_pgisl_i * (if locals.var_t1 >= 1e-38 { locals.var_t1_dn8 } else { 0.0 } / assign12630_e11742))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8,)
    }
};
        locals.var_t3 = assign12630_e11747;
        locals.var_t3_dn3 = assign12630_e11747_d_n3;
        locals.var_t3_dn4 = assign12630_e11747_d_n4;
        locals.var_t3_dn5 = assign12630_e11747_d_n5;
        locals.var_t3_dn6 = assign12630_e11747_d_n6;
        locals.var_t3_dn7 = assign12630_e11747_d_n7;
        locals.var_t3_dn8 = assign12630_e11747_d_n8;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_29(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12640_e11765, assign12640_e11765_d_n3, assign12640_e11765_d_n4, assign12640_e11765_d_n5, assign12640_e11765_d_n6, assign12640_e11765_d_n7, assign12640_e11765_d_n8,) = {
    if ((locals.var_guard126 != 0.0) && (locals.var_guard129 == 0.0)) {
        let assign12640_e11753: f64 = (-locals.var_vds_noswap);
        let assign12640_e11755: f64 = (assign12640_e11753 * locals.var_agisl_i);
        let assign12640_e11757: f64 = (assign12640_e11755 * locals.var_weff);
        let assign12640_e11759: f64 = (assign12640_e11757 * locals.var_t3);
        let assign12640_e11761: f64 = (-locals.var_t2);
        let assign12640_e11762: f64 = { let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign12640_e11763: f64 = (assign12640_e11759 * assign12640_e11762);
        (assign12640_e11763, (((assign12640_e11757 * locals.var_t3_dn3) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))), (((assign12640_e11757 * locals.var_t3_dn4) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))), (((((((-locals.var_vds_noswap_dn5) * locals.var_agisl_i) * locals.var_weff) * locals.var_t3) + (assign12640_e11757 * locals.var_t3_dn5)) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))), (((((((-locals.var_vds_noswap_dn6) * locals.var_agisl_i) * locals.var_weff) * locals.var_t3) + (assign12640_e11757 * locals.var_t3_dn6)) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))), (((assign12640_e11757 * locals.var_t3_dn7) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))), (((assign12640_e11757 * locals.var_t3_dn8) * assign12640_e11762) + (assign12640_e11759 * ({ let limited_exp_arg = assign12640_e11761; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8,)
    }
};
        locals.var_t6 = assign12640_e11765;
        locals.var_t6_dn3 = assign12640_e11765_d_n3;
        locals.var_t6_dn4 = assign12640_e11765_d_n4;
        locals.var_t6_dn5 = assign12640_e11765_d_n5;
        locals.var_t6_dn6 = assign12640_e11765_d_n6;
        locals.var_t6_dn7 = assign12640_e11765_d_n7;
        locals.var_t6_dn8 = assign12640_e11765_d_n8;
        locals.var_t6_rv = 0.0;

        let assign12680_e11784: f64 = (2.0 * locals.var_vsat_t);
        let assign12680_e11786: f64 = (assign12680_e11784 / locals.var_utotal);
        locals.var_esatnoi = assign12680_e11786;
        locals.var_esatnoi_dn3 = (-((assign12680_e11784 * locals.var_utotal_dn3) / (locals.var_utotal * locals.var_utotal)));
        locals.var_esatnoi_dn4 = ((((2.0 * locals.var_vsat_t_dn4) * locals.var_utotal) - (assign12680_e11784 * locals.var_utotal_dn4)) / (locals.var_utotal * locals.var_utotal));
        locals.var_esatnoi_dn5 = (-((assign12680_e11784 * locals.var_utotal_dn5) / (locals.var_utotal * locals.var_utotal)));
        locals.var_esatnoi_dn6 = (-((assign12680_e11784 * locals.var_utotal_dn6) / (locals.var_utotal * locals.var_utotal)));
        locals.var_esatnoi_dn7 = (-((assign12680_e11784 * locals.var_utotal_dn7) / (locals.var_utotal * locals.var_utotal)));
        locals.var_esatnoi_dn8 = (-((assign12680_e11784 * locals.var_utotal_dn8) / (locals.var_utotal * locals.var_utotal)));
        locals.var_esatnoi_rv = 0.0;

        let assign12690_e11797: f64 = if (((p.p288 > 0.0) || (p.p289 > 0.0)) || (p.p290 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard131 = assign12690_e11797;
        locals.var_guard131_rv = 0.0;

        let (assign12700_e11805,) = {
    if (locals.var_guard131 != 0.0) {
        let assign12700_e11802: f64 = (2.0 * locals.var_lintnoi_i);
        let assign12700_e11803: f64 = (locals.var_leff - assign12700_e11802);
        (assign12700_e11803,)
    } else {
        (locals.var_leffnoi,)
    }
};
        locals.var_leffnoi = assign12700_e11805;
        locals.var_leffnoi_rv = 0.0;

        let (assign12710_e11811,) = {
    if (locals.var_guard131 != 0.0) {
        let assign12710_e11809: f64 = (locals.var_leffnoi * locals.var_leffnoi);
        (assign12710_e11809,)
    } else {
        (locals.var_leffnoisq,)
    }
};
        locals.var_leffnoisq = assign12710_e11811;
        locals.var_leffnoisq_rv = 0.0;

        let assign12720_e11814: f64 = if p.p287 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard132 = assign12720_e11814;
        locals.var_guard132_rv = 0.0;

        let (assign12730_e11820, assign12730_e11820_d_n3, assign12730_e11820_d_n4, assign12730_e11820_d_n5, assign12730_e11820_d_n6, assign12730_e11820_d_n7, assign12730_e11820_d_n8,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard132 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delclm, locals.var_delclm_dn3, locals.var_delclm_dn4, locals.var_delclm_dn5, locals.var_delclm_dn6, locals.var_delclm_dn7, locals.var_delclm_dn8,)
    }
};
        locals.var_delclm = assign12730_e11820;
        locals.var_delclm_dn3 = assign12730_e11820_d_n3;
        locals.var_delclm_dn4 = assign12730_e11820_d_n4;
        locals.var_delclm_dn5 = assign12730_e11820_d_n5;
        locals.var_delclm_dn6 = assign12730_e11820_d_n6;
        locals.var_delclm_dn7 = assign12730_e11820_d_n7;
        locals.var_delclm_dn8 = assign12730_e11820_d_n8;
        locals.var_delclm_rv = 0.0;

        let (assign12740_e11833, assign12740_e11833_d_n3, assign12740_e11833_d_n4, assign12740_e11833_d_n5, assign12740_e11833_d_n6, assign12740_e11833_d_n7, assign12740_e11833_d_n8,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard132 == 0.0)) {
        let assign12740_e11827: f64 = (locals.var_diffvds / locals.var_litl);
        let assign12740_e11829: f64 = (assign12740_e11827 + p.p287);
        let assign12740_e11831: f64 = (assign12740_e11829 / locals.var_esatnoi);
        (assign12740_e11831, ((((locals.var_diffvds_dn3 / locals.var_litl) * locals.var_esatnoi) - (assign12740_e11829 * locals.var_esatnoi_dn3)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn4 / locals.var_litl) * locals.var_esatnoi) - (assign12740_e11829 * locals.var_esatnoi_dn4)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn5 / locals.var_litl) * locals.var_esatnoi) - (assign12740_e11829 * locals.var_esatnoi_dn5)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn6 / locals.var_litl) * locals.var_esatnoi) - (assign12740_e11829 * locals.var_esatnoi_dn6)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn7 / locals.var_litl) * locals.var_esatnoi) - (assign12740_e11829 * locals.var_esatnoi_dn7)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn8 / locals.var_litl) * locals.var_esatnoi) - (assign12740_e11829 * locals.var_esatnoi_dn8)) / (locals.var_esatnoi * locals.var_esatnoi)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    }
};
        locals.var_t0 = assign12740_e11833;
        locals.var_t0_dn3 = assign12740_e11833_d_n3;
        locals.var_t0_dn4 = assign12740_e11833_d_n4;
        locals.var_t0_dn5 = assign12740_e11833_d_n5;
        locals.var_t0_dn6 = assign12740_e11833_d_n6;
        locals.var_t0_dn7 = assign12740_e11833_d_n7;
        locals.var_t0_dn8 = assign12740_e11833_d_n8;
        locals.var_t0_rv = 0.0;

        let (assign12750_e11845, assign12750_e11845_d_n3, assign12750_e11845_d_n4, assign12750_e11845_d_n5, assign12750_e11845_d_n6, assign12750_e11845_d_n7, assign12750_e11845_d_n8,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard132 == 0.0)) {
        let assign12750_e11841: f64 = (locals.var_t0).max(1e-38);
        let assign12750_e11842: f64 = (assign12750_e11841).ln();
        let assign12750_e11843: f64 = (locals.var_litl * assign12750_e11842);
        (assign12750_e11843, (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn3 } else { 0.0 } / assign12750_e11841)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn4 } else { 0.0 } / assign12750_e11841)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn5 } else { 0.0 } / assign12750_e11841)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn6 } else { 0.0 } / assign12750_e11841)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn7 } else { 0.0 } / assign12750_e11841)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn8 } else { 0.0 } / assign12750_e11841)),)
    } else {
        (locals.var_delclm, locals.var_delclm_dn3, locals.var_delclm_dn4, locals.var_delclm_dn5, locals.var_delclm_dn6, locals.var_delclm_dn7, locals.var_delclm_dn8,)
    }
};
        locals.var_delclm = assign12750_e11845;
        locals.var_delclm_dn3 = assign12750_e11845_d_n3;
        locals.var_delclm_dn4 = assign12750_e11845_d_n4;
        locals.var_delclm_dn5 = assign12750_e11845_d_n5;
        locals.var_delclm_dn6 = assign12750_e11845_d_n6;
        locals.var_delclm_dn7 = assign12750_e11845_d_n7;
        locals.var_delclm_dn8 = assign12750_e11845_d_n8;
        locals.var_delclm_rv = 0.0;

        let assign12760_e11848: f64 = if locals.var_delclm < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard133 = assign12760_e11848;
        locals.var_guard133_rv = 0.0;

        let (assign12770_e11857, assign12770_e11857_d_n3, assign12770_e11857_d_n4, assign12770_e11857_d_n5, assign12770_e11857_d_n6, assign12770_e11857_d_n7, assign12770_e11857_d_n8,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard132 == 0.0)) && (locals.var_guard133 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delclm, locals.var_delclm_dn3, locals.var_delclm_dn4, locals.var_delclm_dn5, locals.var_delclm_dn6, locals.var_delclm_dn7, locals.var_delclm_dn8,)
    }
};
        locals.var_delclm = assign12770_e11857;
        locals.var_delclm_dn3 = assign12770_e11857_d_n3;
        locals.var_delclm_dn4 = assign12770_e11857_d_n4;
        locals.var_delclm_dn5 = assign12770_e11857_d_n5;
        locals.var_delclm_dn6 = assign12770_e11857_d_n6;
        locals.var_delclm_dn7 = assign12770_e11857_d_n7;
        locals.var_delclm_dn8 = assign12770_e11857_d_n8;
        locals.var_delclm_rv = 0.0;

        let assign12780_e11860: f64 = if p.p22 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard134 = assign12780_e11860;
        locals.var_guard134_rv = 0.0;

        let (assign12790_e11868, assign12790_e11868_d_n3, assign12790_e11868_d_n4, assign12790_e11868_d_n5, assign12790_e11868_d_n6, assign12790_e11868_d_n7, assign12790_e11868_d_n8,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign12790_e11866: f64 = (locals.var_qia2 / locals.var_qsref_i);
        (assign12790_e11866, (locals.var_qia2_dn3 / locals.var_qsref_i), (locals.var_qia2_dn4 / locals.var_qsref_i), (locals.var_qia2_dn5 / locals.var_qsref_i), (locals.var_qia2_dn6 / locals.var_qsref_i), (locals.var_qia2_dn7 / locals.var_qsref_i), (locals.var_qia2_dn8 / locals.var_qsref_i),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign12790_e11868;
        locals.var_t1_dn3 = assign12790_e11868_d_n3;
        locals.var_t1_dn4 = assign12790_e11868_d_n4;
        locals.var_t1_dn5 = assign12790_e11868_d_n5;
        locals.var_t1_dn6 = assign12790_e11868_d_n6;
        locals.var_t1_dn7 = assign12790_e11868_d_n7;
        locals.var_t1_dn8 = assign12790_e11868_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign12800_e11878, assign12800_e11878_d_n3, assign12800_e11878_d_n4, assign12800_e11878_d_n5, assign12800_e11878_d_n6, assign12800_e11878_d_n7, assign12800_e11878_d_n8,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign12800_e11875: f64 = (locals.var_t1).powf(locals.var_mpower_i);
        let assign12800_e11876: f64 = (1.0 + assign12800_e11875);
        (assign12800_e11876, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn3)) } } else { (assign12800_e11875 * (locals.var_mpower_i * (locals.var_t1_dn3 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn4)) } } else { (assign12800_e11875 * (locals.var_mpower_i * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn5)) } } else { (assign12800_e11875 * (locals.var_mpower_i * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn6)) } } else { (assign12800_e11875 * (locals.var_mpower_i * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn7)) } } else { (assign12800_e11875 * (locals.var_mpower_i * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((locals.var_mpower_i) as f64).is_finite() && ((locals.var_mpower_i) as f64).fract() == 0.0 { if locals.var_mpower_i == 0.0 { 0.0 } else { (locals.var_mpower_i * ((locals.var_t1).powf(locals.var_mpower_i - 1.0) * locals.var_t1_dn8)) } } else { (assign12800_e11875 * (locals.var_mpower_i * (locals.var_t1_dn8 / locals.var_t1))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8,)
    }
};
        locals.var_t2 = assign12800_e11878;
        locals.var_t2_dn3 = assign12800_e11878_d_n3;
        locals.var_t2_dn4 = assign12800_e11878_d_n4;
        locals.var_t2_dn5 = assign12800_e11878_d_n5;
        locals.var_t2_dn6 = assign12800_e11878_d_n6;
        locals.var_t2_dn7 = assign12800_e11878_d_n7;
        locals.var_t2_dn8 = assign12800_e11878_d_n8;
        locals.var_t2_rv = 0.0;

        let (assign12810_e11886, assign12810_e11886_d_n3, assign12810_e11886_d_n4, assign12810_e11886_d_n5, assign12810_e11886_d_n6, assign12810_e11886_d_n7, assign12810_e11886_d_n8,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign12810_e11884: f64 = (locals.var_noia2_i / locals.var_t2);
        (assign12810_e11884, (-((locals.var_noia2_i * locals.var_t2_dn3) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))), (-((locals.var_noia2_i * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8,)
    }
};
        locals.var_t3 = assign12810_e11886;
        locals.var_t3_dn3 = assign12810_e11886_d_n3;
        locals.var_t3_dn4 = assign12810_e11886_d_n4;
        locals.var_t3_dn5 = assign12810_e11886_d_n5;
        locals.var_t3_dn6 = assign12810_e11886_d_n6;
        locals.var_t3_dn7 = assign12810_e11886_d_n7;
        locals.var_t3_dn8 = assign12810_e11886_d_n8;
        locals.var_t3_rv = 0.0;

        let (assign12820_e11894, assign12820_e11894_d_n3, assign12820_e11894_d_n4, assign12820_e11894_d_n5, assign12820_e11894_d_n6, assign12820_e11894_d_n7, assign12820_e11894_d_n8,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign12820_e11892: f64 = (locals.var_t3 / p.p288);
        (assign12820_e11892, (locals.var_t3_dn3 / p.p288), (locals.var_t3_dn4 / p.p288), (locals.var_t3_dn5 / p.p288), (locals.var_t3_dn6 / p.p288), (locals.var_t3_dn7 / p.p288), (locals.var_t3_dn8 / p.p288),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8,)
    }
};
        locals.var_t4 = assign12820_e11894;
        locals.var_t4_dn3 = assign12820_e11894_d_n3;
        locals.var_t4_dn4 = assign12820_e11894_d_n4;
        locals.var_t4_dn5 = assign12820_e11894_d_n5;
        locals.var_t4_dn6 = assign12820_e11894_d_n6;
        locals.var_t4_dn7 = assign12820_e11894_d_n7;
        locals.var_t4_dn8 = assign12820_e11894_d_n8;
        locals.var_t4_rv = 0.0;

        let (assign12830_e11919, assign12830_e11919_d_n3, assign12830_e11919_d_n4, assign12830_e11919_d_n5, assign12830_e11919_d_n6, assign12830_e11919_d_n7, assign12830_e11919_d_n8,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign12830_e11901: f64 = (locals.var_t4 + 1.0);
        let assign12830_e11904: f64 = (locals.var_t4 - 1.0);
        let assign12830_e11907: f64 = (locals.var_t4 - 1.0);
        let assign12830_e11908: f64 = (assign12830_e11904 * assign12830_e11907);
        let assign12830_e11911: f64 = (0.25 * p.p292);
        let assign12830_e11913: f64 = (assign12830_e11911 * p.p292);
        let assign12830_e11914: f64 = (assign12830_e11908 + assign12830_e11913);
        let assign12830_e11915: f64 = (assign12830_e11914).sqrt();
        let assign12830_e11916: f64 = (assign12830_e11901 + assign12830_e11915);
        let assign12830_e11917: f64 = (0.5 * assign12830_e11916);
        (assign12830_e11917, (0.5 * (locals.var_t4_dn3 + (((locals.var_t4_dn3 * assign12830_e11907) + (assign12830_e11904 * locals.var_t4_dn3)) / (2.0 * assign12830_e11915)))), (0.5 * (locals.var_t4_dn4 + (((locals.var_t4_dn4 * assign12830_e11907) + (assign12830_e11904 * locals.var_t4_dn4)) / (2.0 * assign12830_e11915)))), (0.5 * (locals.var_t4_dn5 + (((locals.var_t4_dn5 * assign12830_e11907) + (assign12830_e11904 * locals.var_t4_dn5)) / (2.0 * assign12830_e11915)))), (0.5 * (locals.var_t4_dn6 + (((locals.var_t4_dn6 * assign12830_e11907) + (assign12830_e11904 * locals.var_t4_dn6)) / (2.0 * assign12830_e11915)))), (0.5 * (locals.var_t4_dn7 + (((locals.var_t4_dn7 * assign12830_e11907) + (assign12830_e11904 * locals.var_t4_dn7)) / (2.0 * assign12830_e11915)))), (0.5 * (locals.var_t4_dn8 + (((locals.var_t4_dn8 * assign12830_e11907) + (assign12830_e11904 * locals.var_t4_dn8)) / (2.0 * assign12830_e11915)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8,)
    }
};
        locals.var_t5 = assign12830_e11919;
        locals.var_t5_dn3 = assign12830_e11919_d_n3;
        locals.var_t5_dn4 = assign12830_e11919_d_n4;
        locals.var_t5_dn5 = assign12830_e11919_d_n5;
        locals.var_t5_dn6 = assign12830_e11919_d_n6;
        locals.var_t5_dn7 = assign12830_e11919_d_n7;
        locals.var_t5_dn8 = assign12830_e11919_d_n8;
        locals.var_t5_rv = 0.0;

        let (assign12840_e11927, assign12840_e11927_d_n3, assign12840_e11927_d_n4, assign12840_e11927_d_n5, assign12840_e11927_d_n6, assign12840_e11927_d_n7, assign12840_e11927_d_n8,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign12840_e11925: f64 = (p.p288 * locals.var_t5);
        (assign12840_e11925, (p.p288 * locals.var_t5_dn3), (p.p288 * locals.var_t5_dn4), (p.p288 * locals.var_t5_dn5), (p.p288 * locals.var_t5_dn6), (p.p288 * locals.var_t5_dn7), (p.p288 * locals.var_t5_dn8),)
    } else {
        (locals.var_noiaeff, locals.var_noiaeff_dn3, locals.var_noiaeff_dn4, locals.var_noiaeff_dn5, locals.var_noiaeff_dn6, locals.var_noiaeff_dn7, locals.var_noiaeff_dn8,)
    }
};
        locals.var_noiaeff = assign12840_e11927;
        locals.var_noiaeff_dn3 = assign12840_e11927_d_n3;
        locals.var_noiaeff_dn4 = assign12840_e11927_d_n4;
        locals.var_noiaeff_dn5 = assign12840_e11927_d_n5;
        locals.var_noiaeff_dn6 = assign12840_e11927_d_n6;
        locals.var_noiaeff_dn7 = assign12840_e11927_d_n7;
        locals.var_noiaeff_dn8 = assign12840_e11927_d_n8;
        locals.var_noiaeff_rv = 0.0;

        let (assign12850_e11934, assign12850_e11934_d_n3, assign12850_e11934_d_n4, assign12850_e11934_d_n5, assign12850_e11934_d_n6, assign12850_e11934_d_n7, assign12850_e11934_d_n8,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard134 == 0.0)) {
        (p.p288, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_noiaeff, locals.var_noiaeff_dn3, locals.var_noiaeff_dn4, locals.var_noiaeff_dn5, locals.var_noiaeff_dn6, locals.var_noiaeff_dn7, locals.var_noiaeff_dn8,)
    }
};
        locals.var_noiaeff = assign12850_e11934;
        locals.var_noiaeff_dn3 = assign12850_e11934_d_n3;
        locals.var_noiaeff_dn4 = assign12850_e11934_d_n4;
        locals.var_noiaeff_dn5 = assign12850_e11934_d_n5;
        locals.var_noiaeff_dn6 = assign12850_e11934_d_n6;
        locals.var_noiaeff_dn7 = assign12850_e11934_d_n7;
        locals.var_noiaeff_dn8 = assign12850_e11934_d_n8;
        locals.var_noiaeff_rv = 0.0;

        let (assign12860_e11949, assign12860_e11949_d_n3, assign12860_e11949_d_n4, assign12860_e11949_d_n5, assign12860_e11949_d_n6, assign12860_e11949_d_n7, assign12860_e11949_d_n8,) = {
    if (locals.var_guard131 != 0.0) {
        let assign12860_e11938: f64 = (1.60219e-19 * 1.60219e-19);
        let assign12860_e11940: f64 = (assign12860_e11938 * 1.60219e-19);
        let assign12860_e11942: f64 = (assign12860_e11940 * locals.var_vtm);
        let assign12860_e11944: f64 = (locals.var_ids).abs();
        let assign12860_e11945: f64 = (assign12860_e11942 * assign12860_e11944);
        let assign12860_e11947: f64 = (assign12860_e11945 * locals.var_utotal);
        (assign12860_e11947, (((assign12860_e11942 * if locals.var_ids >= 0.0 { locals.var_ids_dn3 } else { (-locals.var_ids_dn3) }) * locals.var_utotal) + (assign12860_e11945 * locals.var_utotal_dn3)), (((((assign12860_e11940 * locals.var_vtm_dn4) * assign12860_e11944) + (assign12860_e11942 * if locals.var_ids >= 0.0 { locals.var_ids_dn4 } else { (-locals.var_ids_dn4) })) * locals.var_utotal) + (assign12860_e11945 * locals.var_utotal_dn4)), (((assign12860_e11942 * if locals.var_ids >= 0.0 { locals.var_ids_dn5 } else { (-locals.var_ids_dn5) }) * locals.var_utotal) + (assign12860_e11945 * locals.var_utotal_dn5)), (((assign12860_e11942 * if locals.var_ids >= 0.0 { locals.var_ids_dn6 } else { (-locals.var_ids_dn6) }) * locals.var_utotal) + (assign12860_e11945 * locals.var_utotal_dn6)), (((assign12860_e11942 * if locals.var_ids >= 0.0 { locals.var_ids_dn7 } else { (-locals.var_ids_dn7) }) * locals.var_utotal) + (assign12860_e11945 * locals.var_utotal_dn7)), (((assign12860_e11942 * if locals.var_ids >= 0.0 { locals.var_ids_dn8 } else { (-locals.var_ids_dn8) }) * locals.var_utotal) + (assign12860_e11945 * locals.var_utotal_dn8)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign12860_e11949;
        locals.var_t1_dn3 = assign12860_e11949_d_n3;
        locals.var_t1_dn4 = assign12860_e11949_d_n4;
        locals.var_t1_dn5 = assign12860_e11949_d_n5;
        locals.var_t1_dn6 = assign12860_e11949_d_n6;
        locals.var_t1_dn7 = assign12860_e11949_d_n7;
        locals.var_t1_dn8 = assign12860_e11949_d_n8;
        locals.var_t1_rv = 0.0;

        let (assign12870_e11957, assign12870_e11957_d_n3, assign12870_e11957_d_n4, assign12870_e11957_d_n5, assign12870_e11957_d_n6, assign12870_e11957_d_n7, assign12870_e11957_d_n8,) = {
    if (locals.var_guard131 != 0.0) {
        let assign12870_e11953: f64 = (10000000000.0 * locals.var_coxeff);
        let assign12870_e11955: f64 = (assign12870_e11953 * locals.var_leffnoisq);
        (assign12870_e11955, ((10000000000.0 * locals.var_coxeff_dn3) * locals.var_leffnoisq), ((10000000000.0 * locals.var_coxeff_dn4) * locals.var_leffnoisq), ((10000000000.0 * locals.var_coxeff_dn5) * locals.var_leffnoisq), ((10000000000.0 * locals.var_coxeff_dn6) * locals.var_leffnoisq), ((10000000000.0 * locals.var_coxeff_dn7) * locals.var_leffnoisq), ((10000000000.0 * locals.var_coxeff_dn8) * locals.var_leffnoisq),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8,)
    }
};
        locals.var_t2 = assign12870_e11957;
        locals.var_t2_dn3 = assign12870_e11957_d_n3;
        locals.var_t2_dn4 = assign12870_e11957_d_n4;
        locals.var_t2_dn5 = assign12870_e11957_d_n5;
        locals.var_t2_dn6 = assign12870_e11957_d_n6;
        locals.var_t2_dn7 = assign12870_e11957_d_n7;
        locals.var_t2_dn8 = assign12870_e11957_d_n8;
        locals.var_t2_rv = 0.0;

        let (assign12880_e11965, assign12880_e11965_d_n3, assign12880_e11965_d_n4, assign12880_e11965_d_n5, assign12880_e11965_d_n6, assign12880_e11965_d_n7, assign12880_e11965_d_n8,) = {
    if (locals.var_guard131 != 0.0) {
        let assign12880_e11961: f64 = (locals.var_coxeff * locals.var_qis);
        let assign12880_e11963: f64 = (assign12880_e11961 / 1.60219e-19);
        (assign12880_e11963, (((locals.var_coxeff_dn3 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn3)) / 1.60219e-19), (((locals.var_coxeff_dn4 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn4)) / 1.60219e-19), (((locals.var_coxeff_dn5 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn5)) / 1.60219e-19), (((locals.var_coxeff_dn6 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn6)) / 1.60219e-19), (((locals.var_coxeff_dn7 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn7)) / 1.60219e-19), (((locals.var_coxeff_dn8 * locals.var_qis) + (locals.var_coxeff * locals.var_qis_dn8)) / 1.60219e-19),)
    } else {
        (locals.var_n0, locals.var_n0_dn3, locals.var_n0_dn4, locals.var_n0_dn5, locals.var_n0_dn6, locals.var_n0_dn7, locals.var_n0_dn8,)
    }
};
        locals.var_n0 = assign12880_e11965;
        locals.var_n0_dn3 = assign12880_e11965_d_n3;
        locals.var_n0_dn4 = assign12880_e11965_d_n4;
        locals.var_n0_dn5 = assign12880_e11965_d_n5;
        locals.var_n0_dn6 = assign12880_e11965_d_n6;
        locals.var_n0_dn7 = assign12880_e11965_d_n7;
        locals.var_n0_dn8 = assign12880_e11965_d_n8;
        locals.var_n0_rv = 0.0;

        let (assign12890_e11973, assign12890_e11973_d_n3, assign12890_e11973_d_n4, assign12890_e11973_d_n5, assign12890_e11973_d_n6, assign12890_e11973_d_n7, assign12890_e11973_d_n8,) = {
    if (locals.var_guard131 != 0.0) {
        let assign12890_e11969: f64 = (locals.var_coxeff * locals.var_qid);
        let assign12890_e11971: f64 = (assign12890_e11969 / 1.60219e-19);
        (assign12890_e11971, (((locals.var_coxeff_dn3 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn3)) / 1.60219e-19), (((locals.var_coxeff_dn4 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn4)) / 1.60219e-19), (((locals.var_coxeff_dn5 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn5)) / 1.60219e-19), (((locals.var_coxeff_dn6 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn6)) / 1.60219e-19), (((locals.var_coxeff_dn7 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn7)) / 1.60219e-19), (((locals.var_coxeff_dn8 * locals.var_qid) + (locals.var_coxeff * locals.var_qid_dn8)) / 1.60219e-19),)
    } else {
        (locals.var_nl, locals.var_nl_dn3, locals.var_nl_dn4, locals.var_nl_dn5, locals.var_nl_dn6, locals.var_nl_dn7, locals.var_nl_dn8,)
    }
};
        locals.var_nl = assign12890_e11973;
        locals.var_nl_dn3 = assign12890_e11973_d_n3;
        locals.var_nl_dn4 = assign12890_e11973_d_n4;
        locals.var_nl_dn5 = assign12890_e11973_d_n5;
        locals.var_nl_dn6 = assign12890_e11973_d_n6;
        locals.var_nl_dn7 = assign12890_e11973_d_n7;
        locals.var_nl_dn8 = assign12890_e11973_d_n8;
        locals.var_nl_rv = 0.0;

        let (assign12900_e11983, assign12900_e11983_d_n3, assign12900_e11983_d_n4, assign12900_e11983_d_n5, assign12900_e11983_d_n6, assign12900_e11983_d_n7, assign12900_e11983_d_n8,) = {
    if (locals.var_guard131 != 0.0) {
        let assign12900_e11977: f64 = (locals.var_vtm / 1.60219e-19);
        let assign12900_e11980: f64 = (locals.var_coxeff + locals.var_cit_i);
        let assign12900_e11981: f64 = (assign12900_e11977 * assign12900_e11980);
        (assign12900_e11981, (assign12900_e11977 * locals.var_coxeff_dn3), (((locals.var_vtm_dn4 / 1.60219e-19) * assign12900_e11980) + (assign12900_e11977 * locals.var_coxeff_dn4)), (assign12900_e11977 * locals.var_coxeff_dn5), (assign12900_e11977 * locals.var_coxeff_dn6), (assign12900_e11977 * locals.var_coxeff_dn7), (assign12900_e11977 * locals.var_coxeff_dn8),)
    } else {
        (locals.var_nstar, locals.var_nstar_dn3, locals.var_nstar_dn4, locals.var_nstar_dn5, locals.var_nstar_dn6, locals.var_nstar_dn7, locals.var_nstar_dn8,)
    }
};
        locals.var_nstar = assign12900_e11983;
        locals.var_nstar_dn3 = assign12900_e11983_d_n3;
        locals.var_nstar_dn4 = assign12900_e11983_d_n4;
        locals.var_nstar_dn5 = assign12900_e11983_d_n5;
        locals.var_nstar_dn6 = assign12900_e11983_d_n6;
        locals.var_nstar_dn7 = assign12900_e11983_d_n7;
        locals.var_nstar_dn8 = assign12900_e11983_d_n8;
        locals.var_nstar_rv = 0.0;

        let (assign12910_e11998, assign12910_e11998_d_n3, assign12910_e11998_d_n4, assign12910_e11998_d_n5, assign12910_e11998_d_n6, assign12910_e11998_d_n7, assign12910_e11998_d_n8,) = {
    if (locals.var_guard131 != 0.0) {
        let assign12910_e11988: f64 = (locals.var_n0 + locals.var_nstar);
        let assign12910_e11991: f64 = (locals.var_nl + locals.var_nstar);
        let assign12910_e11992: f64 = (assign12910_e11988 / assign12910_e11991);
        let assign12910_e11994: f64 = (assign12910_e11992).max(1e-38);
        let assign12910_e11995: f64 = (assign12910_e11994).ln();
        let assign12910_e11996: f64 = (locals.var_noiaeff * assign12910_e11995);
        (assign12910_e11996, ((locals.var_noiaeff_dn3 * assign12910_e11995) + (locals.var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((locals.var_n0_dn3 + locals.var_nstar_dn3) * assign12910_e11991) - (assign12910_e11988 * (locals.var_nl_dn3 + locals.var_nstar_dn3))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((locals.var_noiaeff_dn4 * assign12910_e11995) + (locals.var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((locals.var_n0_dn4 + locals.var_nstar_dn4) * assign12910_e11991) - (assign12910_e11988 * (locals.var_nl_dn4 + locals.var_nstar_dn4))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((locals.var_noiaeff_dn5 * assign12910_e11995) + (locals.var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((locals.var_n0_dn5 + locals.var_nstar_dn5) * assign12910_e11991) - (assign12910_e11988 * (locals.var_nl_dn5 + locals.var_nstar_dn5))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((locals.var_noiaeff_dn6 * assign12910_e11995) + (locals.var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((locals.var_n0_dn6 + locals.var_nstar_dn6) * assign12910_e11991) - (assign12910_e11988 * (locals.var_nl_dn6 + locals.var_nstar_dn6))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((locals.var_noiaeff_dn7 * assign12910_e11995) + (locals.var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((locals.var_n0_dn7 + locals.var_nstar_dn7) * assign12910_e11991) - (assign12910_e11988 * (locals.var_nl_dn7 + locals.var_nstar_dn7))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))), ((locals.var_noiaeff_dn8 * assign12910_e11995) + (locals.var_noiaeff * (if assign12910_e11992 >= 1e-38 { ((((locals.var_n0_dn8 + locals.var_nstar_dn8) * assign12910_e11991) - (assign12910_e11988 * (locals.var_nl_dn8 + locals.var_nstar_dn8))) / (assign12910_e11991 * assign12910_e11991)) } else { 0.0 } / assign12910_e11994))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8,)
    }
};
        locals.var_t3 = assign12910_e11998;
        locals.var_t3_dn3 = assign12910_e11998_d_n3;
        locals.var_t3_dn4 = assign12910_e11998_d_n4;
        locals.var_t3_dn5 = assign12910_e11998_d_n5;
        locals.var_t3_dn6 = assign12910_e11998_d_n6;
        locals.var_t3_dn7 = assign12910_e11998_d_n7;
        locals.var_t3_dn8 = assign12910_e11998_d_n8;
        locals.var_t3_rv = 0.0;

        let (assign12920_e12006, assign12920_e12006_d_n3, assign12920_e12006_d_n4, assign12920_e12006_d_n5, assign12920_e12006_d_n6, assign12920_e12006_d_n7, assign12920_e12006_d_n8,) = {
    if (locals.var_guard131 != 0.0) {
        let assign12920_e12003: f64 = (locals.var_n0 - locals.var_nl);
        let assign12920_e12004: f64 = (p.p289 * assign12920_e12003);
        (assign12920_e12004, (p.p289 * (locals.var_n0_dn3 - locals.var_nl_dn3)), (p.p289 * (locals.var_n0_dn4 - locals.var_nl_dn4)), (p.p289 * (locals.var_n0_dn5 - locals.var_nl_dn5)), (p.p289 * (locals.var_n0_dn6 - locals.var_nl_dn6)), (p.p289 * (locals.var_n0_dn7 - locals.var_nl_dn7)), (p.p289 * (locals.var_n0_dn8 - locals.var_nl_dn8)),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8,)
    }
};
        locals.var_t4 = assign12920_e12006;
        locals.var_t4_dn3 = assign12920_e12006_d_n3;
        locals.var_t4_dn4 = assign12920_e12006_d_n4;
        locals.var_t4_dn5 = assign12920_e12006_d_n5;
        locals.var_t4_dn6 = assign12920_e12006_d_n6;
        locals.var_t4_dn7 = assign12920_e12006_d_n7;
        locals.var_t4_dn8 = assign12920_e12006_d_n8;
        locals.var_t4_rv = 0.0;

        let (assign12930_e12020, assign12930_e12020_d_n3, assign12930_e12020_d_n4, assign12930_e12020_d_n5, assign12930_e12020_d_n6, assign12930_e12020_d_n7, assign12930_e12020_d_n8,) = {
    if (locals.var_guard131 != 0.0) {
        let assign12930_e12010: f64 = (0.5 * p.p290);
        let assign12930_e12013: f64 = (locals.var_n0 * locals.var_n0);
        let assign12930_e12016: f64 = (locals.var_nl * locals.var_nl);
        let assign12930_e12017: f64 = (assign12930_e12013 - assign12930_e12016);
        let assign12930_e12018: f64 = (assign12930_e12010 * assign12930_e12017);
        (assign12930_e12018, (assign12930_e12010 * (((locals.var_n0_dn3 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn3)) - ((locals.var_nl_dn3 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn3)))), (assign12930_e12010 * (((locals.var_n0_dn4 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn4)) - ((locals.var_nl_dn4 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn4)))), (assign12930_e12010 * (((locals.var_n0_dn5 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn5)) - ((locals.var_nl_dn5 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn5)))), (assign12930_e12010 * (((locals.var_n0_dn6 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn6)) - ((locals.var_nl_dn6 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn6)))), (assign12930_e12010 * (((locals.var_n0_dn7 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn7)) - ((locals.var_nl_dn7 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn7)))), (assign12930_e12010 * (((locals.var_n0_dn8 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn8)) - ((locals.var_nl_dn8 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn8)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8,)
    }
};
        locals.var_t5 = assign12930_e12020;
        locals.var_t5_dn3 = assign12930_e12020_d_n3;
        locals.var_t5_dn4 = assign12930_e12020_d_n4;
        locals.var_t5_dn5 = assign12930_e12020_d_n5;
        locals.var_t5_dn6 = assign12930_e12020_d_n6;
        locals.var_t5_dn7 = assign12930_e12020_d_n7;
        locals.var_t5_dn8 = assign12930_e12020_d_n8;
        locals.var_t5_rv = 0.0;

        let (assign12940_e12030, assign12940_e12030_d_n3, assign12940_e12030_d_n4, assign12940_e12030_d_n5, assign12940_e12030_d_n6, assign12940_e12030_d_n7, assign12940_e12030_d_n8,) = {
    if (locals.var_guard131 != 0.0) {
        let assign12940_e12024: f64 = (1.60219e-19 * locals.var_vtm);
        let assign12940_e12026: f64 = (assign12940_e12024 * locals.var_ids);
        let assign12940_e12028: f64 = (assign12940_e12026 * locals.var_ids);
        (assign12940_e12028, (((assign12940_e12024 * locals.var_ids_dn3) * locals.var_ids) + (assign12940_e12026 * locals.var_ids_dn3)), (((((1.60219e-19 * locals.var_vtm_dn4) * locals.var_ids) + (assign12940_e12024 * locals.var_ids_dn4)) * locals.var_ids) + (assign12940_e12026 * locals.var_ids_dn4)), (((assign12940_e12024 * locals.var_ids_dn5) * locals.var_ids) + (assign12940_e12026 * locals.var_ids_dn5)), (((assign12940_e12024 * locals.var_ids_dn6) * locals.var_ids) + (assign12940_e12026 * locals.var_ids_dn6)), (((assign12940_e12024 * locals.var_ids_dn7) * locals.var_ids) + (assign12940_e12026 * locals.var_ids_dn7)), (((assign12940_e12024 * locals.var_ids_dn8) * locals.var_ids) + (assign12940_e12026 * locals.var_ids_dn8)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8,)
    }
};
        locals.var_t6 = assign12940_e12030;
        locals.var_t6_dn3 = assign12940_e12030_d_n3;
        locals.var_t6_dn4 = assign12940_e12030_d_n4;
        locals.var_t6_dn5 = assign12940_e12030_d_n5;
        locals.var_t6_dn6 = assign12940_e12030_d_n6;
        locals.var_t6_dn7 = assign12940_e12030_d_n7;
        locals.var_t6_dn8 = assign12940_e12030_d_n8;
        locals.var_t6_rv = 0.0;

        let (assign12950_e12040, assign12950_e12040_d_n3, assign12950_e12040_d_n4, assign12950_e12040_d_n5, assign12950_e12040_d_n6, assign12950_e12040_d_n7, assign12950_e12040_d_n8,) = {
    if (locals.var_guard131 != 0.0) {
        let assign12950_e12034: f64 = (10000000000.0 * locals.var_leffnoisq);
        let assign12950_e12036: f64 = (assign12950_e12034 * locals.var_weff);
        let assign12950_e12038: f64 = (assign12950_e12036 * p.p2);
        (assign12950_e12038, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8,)
    }
};
        locals.var_t7 = assign12950_e12040;
        locals.var_t7_dn3 = assign12950_e12040_d_n3;
        locals.var_t7_dn4 = assign12950_e12040_d_n4;
        locals.var_t7_dn5 = assign12950_e12040_d_n5;
        locals.var_t7_dn6 = assign12950_e12040_d_n6;
        locals.var_t7_dn7 = assign12950_e12040_d_n7;
        locals.var_t7_dn8 = assign12950_e12040_d_n8;
        locals.var_t7_rv = 0.0;

        let (assign12960_e12054, assign12960_e12054_d_n3, assign12960_e12054_d_n4, assign12960_e12054_d_n5, assign12960_e12054_d_n6, assign12960_e12054_d_n7, assign12960_e12054_d_n8,) = {
    if (locals.var_guard131 != 0.0) {
        let assign12960_e12045: f64 = (p.p289 * locals.var_nl);
        let assign12960_e12046: f64 = (locals.var_noiaeff + assign12960_e12045);
        let assign12960_e12049: f64 = (p.p290 * locals.var_nl);
        let assign12960_e12051: f64 = (assign12960_e12049 * locals.var_nl);
        let assign12960_e12052: f64 = (assign12960_e12046 + assign12960_e12051);
        (assign12960_e12052, ((locals.var_noiaeff_dn3 + (p.p289 * locals.var_nl_dn3)) + (((p.p290 * locals.var_nl_dn3) * locals.var_nl) + (assign12960_e12049 * locals.var_nl_dn3))), ((locals.var_noiaeff_dn4 + (p.p289 * locals.var_nl_dn4)) + (((p.p290 * locals.var_nl_dn4) * locals.var_nl) + (assign12960_e12049 * locals.var_nl_dn4))), ((locals.var_noiaeff_dn5 + (p.p289 * locals.var_nl_dn5)) + (((p.p290 * locals.var_nl_dn5) * locals.var_nl) + (assign12960_e12049 * locals.var_nl_dn5))), ((locals.var_noiaeff_dn6 + (p.p289 * locals.var_nl_dn6)) + (((p.p290 * locals.var_nl_dn6) * locals.var_nl) + (assign12960_e12049 * locals.var_nl_dn6))), ((locals.var_noiaeff_dn7 + (p.p289 * locals.var_nl_dn7)) + (((p.p290 * locals.var_nl_dn7) * locals.var_nl) + (assign12960_e12049 * locals.var_nl_dn7))), ((locals.var_noiaeff_dn8 + (p.p289 * locals.var_nl_dn8)) + (((p.p290 * locals.var_nl_dn8) * locals.var_nl) + (assign12960_e12049 * locals.var_nl_dn8))),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8,)
    }
};
        locals.var_t8 = assign12960_e12054;
        locals.var_t8_dn3 = assign12960_e12054_d_n3;
        locals.var_t8_dn4 = assign12960_e12054_d_n4;
        locals.var_t8_dn5 = assign12960_e12054_d_n5;
        locals.var_t8_dn6 = assign12960_e12054_d_n6;
        locals.var_t8_dn7 = assign12960_e12054_d_n7;
        locals.var_t8_dn8 = assign12960_e12054_d_n8;
        locals.var_t8_rv = 0.0;

        let (assign12970_e12064, assign12970_e12064_d_n3, assign12970_e12064_d_n4, assign12970_e12064_d_n5, assign12970_e12064_d_n6, assign12970_e12064_d_n7, assign12970_e12064_d_n8,) = {
    if (locals.var_guard131 != 0.0) {
        let assign12970_e12058: f64 = (locals.var_nl + locals.var_nstar);
        let assign12970_e12061: f64 = (locals.var_nl + locals.var_nstar);
        let assign12970_e12062: f64 = (assign12970_e12058 * assign12970_e12061);
        (assign12970_e12062, (((locals.var_nl_dn3 + locals.var_nstar_dn3) * assign12970_e12061) + (assign12970_e12058 * (locals.var_nl_dn3 + locals.var_nstar_dn3))), (((locals.var_nl_dn4 + locals.var_nstar_dn4) * assign12970_e12061) + (assign12970_e12058 * (locals.var_nl_dn4 + locals.var_nstar_dn4))), (((locals.var_nl_dn5 + locals.var_nstar_dn5) * assign12970_e12061) + (assign12970_e12058 * (locals.var_nl_dn5 + locals.var_nstar_dn5))), (((locals.var_nl_dn6 + locals.var_nstar_dn6) * assign12970_e12061) + (assign12970_e12058 * (locals.var_nl_dn6 + locals.var_nstar_dn6))), (((locals.var_nl_dn7 + locals.var_nstar_dn7) * assign12970_e12061) + (assign12970_e12058 * (locals.var_nl_dn7 + locals.var_nstar_dn7))), (((locals.var_nl_dn8 + locals.var_nstar_dn8) * assign12970_e12061) + (assign12970_e12058 * (locals.var_nl_dn8 + locals.var_nstar_dn8))),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8,)
    }
};
        locals.var_t9 = assign12970_e12064;
        locals.var_t9_dn3 = assign12970_e12064_d_n3;
        locals.var_t9_dn4 = assign12970_e12064_d_n4;
        locals.var_t9_dn5 = assign12970_e12064_d_n5;
        locals.var_t9_dn6 = assign12970_e12064_d_n6;
        locals.var_t9_dn7 = assign12970_e12064_d_n7;
        locals.var_t9_dn8 = assign12970_e12064_d_n8;
        locals.var_t9_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_30(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12980_e12086, assign12980_e12086_d_n3, assign12980_e12086_d_n4, assign12980_e12086_d_n5, assign12980_e12086_d_n6, assign12980_e12086_d_n7, assign12980_e12086_d_n8,) = {
    if (locals.var_guard131 != 0.0) {
        let assign12980_e12068: f64 = (locals.var_t1 / locals.var_t2);
        let assign12980_e12071: f64 = (locals.var_t3 + locals.var_t4);
        let assign12980_e12073: f64 = (assign12980_e12071 + locals.var_t5);
        let assign12980_e12074: f64 = (assign12980_e12068 * assign12980_e12073);
        let assign12980_e12077: f64 = (locals.var_t6 / locals.var_t7);
        let assign12980_e12079: f64 = (assign12980_e12077 * locals.var_delclm);
        let assign12980_e12081: f64 = (assign12980_e12079 * locals.var_t8);
        let assign12980_e12083: f64 = (assign12980_e12081 / locals.var_t9);
        let assign12980_e12084: f64 = (assign12980_e12074 + assign12980_e12083);
        (assign12980_e12084, ((((((locals.var_t1_dn3 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn3)) / (locals.var_t2 * locals.var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((locals.var_t3_dn3 + locals.var_t4_dn3) + locals.var_t5_dn3))) + ((((((((((locals.var_t6_dn3 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn3)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign12980_e12077 * locals.var_delclm_dn3)) * locals.var_t8) + (assign12980_e12079 * locals.var_t8_dn3)) * locals.var_t9) - (assign12980_e12081 * locals.var_t9_dn3)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((locals.var_t3_dn4 + locals.var_t4_dn4) + locals.var_t5_dn4))) + ((((((((((locals.var_t6_dn4 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn4)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign12980_e12077 * locals.var_delclm_dn4)) * locals.var_t8) + (assign12980_e12079 * locals.var_t8_dn4)) * locals.var_t9) - (assign12980_e12081 * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((locals.var_t3_dn5 + locals.var_t4_dn5) + locals.var_t5_dn5))) + ((((((((((locals.var_t6_dn5 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn5)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign12980_e12077 * locals.var_delclm_dn5)) * locals.var_t8) + (assign12980_e12079 * locals.var_t8_dn5)) * locals.var_t9) - (assign12980_e12081 * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((locals.var_t3_dn6 + locals.var_t4_dn6) + locals.var_t5_dn6))) + ((((((((((locals.var_t6_dn6 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn6)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign12980_e12077 * locals.var_delclm_dn6)) * locals.var_t8) + (assign12980_e12079 * locals.var_t8_dn6)) * locals.var_t9) - (assign12980_e12081 * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((locals.var_t3_dn7 + locals.var_t4_dn7) + locals.var_t5_dn7))) + ((((((((((locals.var_t6_dn7 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn7)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign12980_e12077 * locals.var_delclm_dn7)) * locals.var_t8) + (assign12980_e12079 * locals.var_t8_dn7)) * locals.var_t9) - (assign12980_e12081 * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9))), ((((((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)) * assign12980_e12073) + (assign12980_e12068 * ((locals.var_t3_dn8 + locals.var_t4_dn8) + locals.var_t5_dn8))) + ((((((((((locals.var_t6_dn8 * locals.var_t7) - (locals.var_t6 * locals.var_t7_dn8)) / (locals.var_t7 * locals.var_t7)) * locals.var_delclm) + (assign12980_e12077 * locals.var_delclm_dn8)) * locals.var_t8) + (assign12980_e12079 * locals.var_t8_dn8)) * locals.var_t9) - (assign12980_e12081 * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9))),)
    } else {
        (locals.var_ssi, locals.var_ssi_dn3, locals.var_ssi_dn4, locals.var_ssi_dn5, locals.var_ssi_dn6, locals.var_ssi_dn7, locals.var_ssi_dn8,)
    }
};
        locals.var_ssi = assign12980_e12086;
        locals.var_ssi_dn3 = assign12980_e12086_d_n3;
        locals.var_ssi_dn4 = assign12980_e12086_d_n4;
        locals.var_ssi_dn5 = assign12980_e12086_d_n5;
        locals.var_ssi_dn6 = assign12980_e12086_d_n6;
        locals.var_ssi_dn7 = assign12980_e12086_d_n7;
        locals.var_ssi_dn8 = assign12980_e12086_d_n8;
        locals.var_ssi_rv = 0.0;

        let (assign12990_e12094, assign12990_e12094_d_n3, assign12990_e12094_d_n4, assign12990_e12094_d_n5, assign12990_e12094_d_n6, assign12990_e12094_d_n7, assign12990_e12094_d_n8,) = {
    if (locals.var_guard131 != 0.0) {
        let assign12990_e12090: f64 = (locals.var_noiaeff * 1.60219e-19);
        let assign12990_e12092: f64 = (assign12990_e12090 * locals.var_vtm);
        (assign12990_e12092, ((locals.var_noiaeff_dn3 * 1.60219e-19) * locals.var_vtm), (((locals.var_noiaeff_dn4 * 1.60219e-19) * locals.var_vtm) + (assign12990_e12090 * locals.var_vtm_dn4)), ((locals.var_noiaeff_dn5 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn6 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn7 * 1.60219e-19) * locals.var_vtm), ((locals.var_noiaeff_dn8 * 1.60219e-19) * locals.var_vtm),)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8,)
    }
};
        locals.var_t10 = assign12990_e12094;
        locals.var_t10_dn3 = assign12990_e12094_d_n3;
        locals.var_t10_dn4 = assign12990_e12094_d_n4;
        locals.var_t10_dn5 = assign12990_e12094_d_n5;
        locals.var_t10_dn6 = assign12990_e12094_d_n6;
        locals.var_t10_dn7 = assign12990_e12094_d_n7;
        locals.var_t10_dn8 = assign12990_e12094_d_n8;
        locals.var_t10_rv = 0.0;

        let (assign13000_e12108, assign13000_e12108_d_n3, assign13000_e12108_d_n4, assign13000_e12108_d_n5, assign13000_e12108_d_n6, assign13000_e12108_d_n7, assign13000_e12108_d_n8,) = {
    if (locals.var_guard131 != 0.0) {
        let assign13000_e12098: f64 = (locals.var_weff * p.p2);
        let assign13000_e12100: f64 = (assign13000_e12098 * locals.var_leffnoi);
        let assign13000_e12102: f64 = (assign13000_e12100 * 10000000000.0);
        let assign13000_e12104: f64 = (assign13000_e12102 * locals.var_nstar);
        let assign13000_e12106: f64 = (assign13000_e12104 * locals.var_nstar);
        (assign13000_e12106, (((assign13000_e12102 * locals.var_nstar_dn3) * locals.var_nstar) + (assign13000_e12104 * locals.var_nstar_dn3)), (((assign13000_e12102 * locals.var_nstar_dn4) * locals.var_nstar) + (assign13000_e12104 * locals.var_nstar_dn4)), (((assign13000_e12102 * locals.var_nstar_dn5) * locals.var_nstar) + (assign13000_e12104 * locals.var_nstar_dn5)), (((assign13000_e12102 * locals.var_nstar_dn6) * locals.var_nstar) + (assign13000_e12104 * locals.var_nstar_dn6)), (((assign13000_e12102 * locals.var_nstar_dn7) * locals.var_nstar) + (assign13000_e12104 * locals.var_nstar_dn7)), (((assign13000_e12102 * locals.var_nstar_dn8) * locals.var_nstar) + (assign13000_e12104 * locals.var_nstar_dn8)),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8,)
    }
};
        locals.var_t11 = assign13000_e12108;
        locals.var_t11_dn3 = assign13000_e12108_d_n3;
        locals.var_t11_dn4 = assign13000_e12108_d_n4;
        locals.var_t11_dn5 = assign13000_e12108_d_n5;
        locals.var_t11_dn6 = assign13000_e12108_d_n6;
        locals.var_t11_dn7 = assign13000_e12108_d_n7;
        locals.var_t11_dn8 = assign13000_e12108_d_n8;
        locals.var_t11_rv = 0.0;

        let (assign13010_e12118, assign13010_e12118_d_n3, assign13010_e12118_d_n4, assign13010_e12118_d_n5, assign13010_e12118_d_n6, assign13010_e12118_d_n7, assign13010_e12118_d_n8,) = {
    if (locals.var_guard131 != 0.0) {
        let assign13010_e12112: f64 = (locals.var_t10 / locals.var_t11);
        let assign13010_e12114: f64 = (assign13010_e12112 * locals.var_ids);
        let assign13010_e12116: f64 = (assign13010_e12114 * locals.var_ids);
        (assign13010_e12116, (((((((locals.var_t10_dn3 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn3)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids) + (assign13010_e12112 * locals.var_ids_dn3)) * locals.var_ids) + (assign13010_e12114 * locals.var_ids_dn3)), (((((((locals.var_t10_dn4 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn4)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids) + (assign13010_e12112 * locals.var_ids_dn4)) * locals.var_ids) + (assign13010_e12114 * locals.var_ids_dn4)), (((((((locals.var_t10_dn5 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn5)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids) + (assign13010_e12112 * locals.var_ids_dn5)) * locals.var_ids) + (assign13010_e12114 * locals.var_ids_dn5)), (((((((locals.var_t10_dn6 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn6)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids) + (assign13010_e12112 * locals.var_ids_dn6)) * locals.var_ids) + (assign13010_e12114 * locals.var_ids_dn6)), (((((((locals.var_t10_dn7 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn7)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids) + (assign13010_e12112 * locals.var_ids_dn7)) * locals.var_ids) + (assign13010_e12114 * locals.var_ids_dn7)), (((((((locals.var_t10_dn8 * locals.var_t11) - (locals.var_t10 * locals.var_t11_dn8)) / (locals.var_t11 * locals.var_t11)) * locals.var_ids) + (assign13010_e12112 * locals.var_ids_dn8)) * locals.var_ids) + (assign13010_e12114 * locals.var_ids_dn8)),)
    } else {
        (locals.var_swi, locals.var_swi_dn3, locals.var_swi_dn4, locals.var_swi_dn5, locals.var_swi_dn6, locals.var_swi_dn7, locals.var_swi_dn8,)
    }
};
        locals.var_swi = assign13010_e12118;
        locals.var_swi_dn3 = assign13010_e12118_d_n3;
        locals.var_swi_dn4 = assign13010_e12118_d_n4;
        locals.var_swi_dn5 = assign13010_e12118_d_n5;
        locals.var_swi_dn6 = assign13010_e12118_d_n6;
        locals.var_swi_dn7 = assign13010_e12118_d_n7;
        locals.var_swi_dn8 = assign13010_e12118_d_n8;
        locals.var_swi_rv = 0.0;

        let (assign13020_e12124, assign13020_e12124_d_n3, assign13020_e12124_d_n4, assign13020_e12124_d_n5, assign13020_e12124_d_n6, assign13020_e12124_d_n7, assign13020_e12124_d_n8,) = {
    if (locals.var_guard131 != 0.0) {
        let assign13020_e12122: f64 = (locals.var_swi + locals.var_ssi);
        (assign13020_e12122, (locals.var_swi_dn3 + locals.var_ssi_dn3), (locals.var_swi_dn4 + locals.var_ssi_dn4), (locals.var_swi_dn5 + locals.var_ssi_dn5), (locals.var_swi_dn6 + locals.var_ssi_dn6), (locals.var_swi_dn7 + locals.var_ssi_dn7), (locals.var_swi_dn8 + locals.var_ssi_dn8),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8,)
    }
};
        locals.var_t1 = assign13020_e12124;
        locals.var_t1_dn3 = assign13020_e12124_d_n3;
        locals.var_t1_dn4 = assign13020_e12124_d_n4;
        locals.var_t1_dn5 = assign13020_e12124_d_n5;
        locals.var_t1_dn6 = assign13020_e12124_d_n6;
        locals.var_t1_dn7 = assign13020_e12124_d_n7;
        locals.var_t1_dn8 = assign13020_e12124_d_n8;
        locals.var_t1_rv = 0.0;

        let assign13070_e12152: f64 = (locals.var_devsign * p.p2);
        let assign13070_e12154: f64 = (assign13070_e12152 * locals.var_qfg);
        locals.var_qfgi = assign13070_e12154;
        locals.var_qfgi_dn3 = (assign13070_e12152 * locals.var_qfg_dn3);
        locals.var_qfgi_dn4 = (assign13070_e12152 * locals.var_qfg_dn4);
        locals.var_qfgi_dn5 = (assign13070_e12152 * locals.var_qfg_dn5);
        locals.var_qfgi_dn6 = (assign13070_e12152 * locals.var_qfg_dn6);
        locals.var_qfgi_dn7 = (assign13070_e12152 * locals.var_qfg_dn7);
        locals.var_qfgi_dn8 = (assign13070_e12152 * locals.var_qfg_dn8);
        locals.var_qfgi_rv = 0.0;

        let assign13080_e12157: f64 = (p.p2 * locals.var_qbg);
        locals.var_qbgi = assign13080_e12157;
        locals.var_qbgi_dn3 = (p.p2 * locals.var_qbg_dn3);
        locals.var_qbgi_dn4 = (p.p2 * locals.var_qbg_dn4);
        locals.var_qbgi_dn5 = (p.p2 * locals.var_qbg_dn5);
        locals.var_qbgi_dn6 = (p.p2 * locals.var_qbg_dn6);
        locals.var_qbgi_dn7 = (p.p2 * locals.var_qbg_dn7);
        locals.var_qbgi_dn8 = (p.p2 * locals.var_qbg_dn8);
        locals.var_qbgi_rv = 0.0;

        let assign13090_e12160: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard136 = assign13090_e12160;
        locals.var_guard136_rv = 0.0;

        let (assign13100_e12166, assign13100_e12166_d_n3, assign13100_e12166_d_n4, assign13100_e12166_d_n5, assign13100_e12166_d_n6, assign13100_e12166_d_n7, assign13100_e12166_d_n8,) = {
    if (locals.var_guard136 != 0.0) {
        let assign13100_e12164: f64 = (p.p2 * locals.var_qs);
        (assign13100_e12164, (p.p2 * locals.var_qs_dn3), (p.p2 * locals.var_qs_dn4), (p.p2 * locals.var_qs_dn5), (p.p2 * locals.var_qs_dn6), (p.p2 * locals.var_qs_dn7), (p.p2 * locals.var_qs_dn8),)
    } else {
        (locals.var_qsi, locals.var_qsi_dn3, locals.var_qsi_dn4, locals.var_qsi_dn5, locals.var_qsi_dn6, locals.var_qsi_dn7, locals.var_qsi_dn8,)
    }
};
        locals.var_qsi = assign13100_e12166;
        locals.var_qsi_dn3 = assign13100_e12166_d_n3;
        locals.var_qsi_dn4 = assign13100_e12166_d_n4;
        locals.var_qsi_dn5 = assign13100_e12166_d_n5;
        locals.var_qsi_dn6 = assign13100_e12166_d_n6;
        locals.var_qsi_dn7 = assign13100_e12166_d_n7;
        locals.var_qsi_dn8 = assign13100_e12166_d_n8;
        locals.var_qsi_rv = 0.0;

        let (assign13110_e12172, assign13110_e12172_d_n3, assign13110_e12172_d_n4, assign13110_e12172_d_n5, assign13110_e12172_d_n6, assign13110_e12172_d_n7, assign13110_e12172_d_n8,) = {
    if (locals.var_guard136 != 0.0) {
        let assign13110_e12170: f64 = (p.p2 * locals.var_qd);
        (assign13110_e12170, (p.p2 * locals.var_qd_dn3), (p.p2 * locals.var_qd_dn4), (p.p2 * locals.var_qd_dn5), (p.p2 * locals.var_qd_dn6), (p.p2 * locals.var_qd_dn7), (p.p2 * locals.var_qd_dn8),)
    } else {
        (locals.var_qdi, locals.var_qdi_dn3, locals.var_qdi_dn4, locals.var_qdi_dn5, locals.var_qdi_dn6, locals.var_qdi_dn7, locals.var_qdi_dn8,)
    }
};
        locals.var_qdi = assign13110_e12172;
        locals.var_qdi_dn3 = assign13110_e12172_d_n3;
        locals.var_qdi_dn4 = assign13110_e12172_d_n4;
        locals.var_qdi_dn5 = assign13110_e12172_d_n5;
        locals.var_qdi_dn6 = assign13110_e12172_d_n6;
        locals.var_qdi_dn7 = assign13110_e12172_d_n7;
        locals.var_qdi_dn8 = assign13110_e12172_d_n8;
        locals.var_qdi_rv = 0.0;

        let (assign13120_e12182, assign13120_e12182_d_n3, assign13120_e12182_d_n4, assign13120_e12182_d_n5, assign13120_e12182_d_n6, assign13120_e12182_d_n7, assign13120_e12182_d_n8,) = {
    if (locals.var_guard136 != 0.0) {
        let assign13120_e12177: f64 = (locals.var_qs - locals.var_qfgs_parasitic);
        let assign13120_e12178: f64 = (p.p2 * assign13120_e12177);
        let assign13120_e12180: f64 = (assign13120_e12178 + locals.var_qsbg);
        (assign13120_e12180, ((p.p2 * (locals.var_qs_dn3 - locals.var_qfgs_parasitic_dn3)) + locals.var_qsbg_dn3), ((p.p2 * (locals.var_qs_dn4 - locals.var_qfgs_parasitic_dn4)) + locals.var_qsbg_dn4), ((p.p2 * (locals.var_qs_dn5 - locals.var_qfgs_parasitic_dn5)) + locals.var_qsbg_dn5), ((p.p2 * (locals.var_qs_dn6 - locals.var_qfgs_parasitic_dn6)) + locals.var_qsbg_dn6), ((p.p2 * (locals.var_qs_dn7 - locals.var_qfgs_parasitic_dn7)) + locals.var_qsbg_dn7), ((p.p2 * (locals.var_qs_dn8 - locals.var_qfgs_parasitic_dn8)) + locals.var_qsbg_dn8),)
    } else {
        (locals.var_qs, locals.var_qs_dn3, locals.var_qs_dn4, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8,)
    }
};
        locals.var_qs = assign13120_e12182;
        locals.var_qs_dn3 = assign13120_e12182_d_n3;
        locals.var_qs_dn4 = assign13120_e12182_d_n4;
        locals.var_qs_dn5 = assign13120_e12182_d_n5;
        locals.var_qs_dn6 = assign13120_e12182_d_n6;
        locals.var_qs_dn7 = assign13120_e12182_d_n7;
        locals.var_qs_dn8 = assign13120_e12182_d_n8;
        locals.var_qs_rv = 0.0;

        let (assign13130_e12192, assign13130_e12192_d_n3, assign13130_e12192_d_n4, assign13130_e12192_d_n5, assign13130_e12192_d_n6, assign13130_e12192_d_n7, assign13130_e12192_d_n8,) = {
    if (locals.var_guard136 != 0.0) {
        let assign13130_e12187: f64 = (locals.var_qd - locals.var_qfgd_parasitic);
        let assign13130_e12188: f64 = (p.p2 * assign13130_e12187);
        let assign13130_e12190: f64 = (assign13130_e12188 + locals.var_qdbg);
        (assign13130_e12190, ((p.p2 * (locals.var_qd_dn3 - locals.var_qfgd_parasitic_dn3)) + locals.var_qdbg_dn3), ((p.p2 * (locals.var_qd_dn4 - locals.var_qfgd_parasitic_dn4)) + locals.var_qdbg_dn4), ((p.p2 * (locals.var_qd_dn5 - locals.var_qfgd_parasitic_dn5)) + locals.var_qdbg_dn5), ((p.p2 * (locals.var_qd_dn6 - locals.var_qfgd_parasitic_dn6)) + locals.var_qdbg_dn6), ((p.p2 * (locals.var_qd_dn7 - locals.var_qfgd_parasitic_dn7)) + locals.var_qdbg_dn7), ((p.p2 * (locals.var_qd_dn8 - locals.var_qfgd_parasitic_dn8)) + locals.var_qdbg_dn8),)
    } else {
        (locals.var_qd, locals.var_qd_dn3, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8,)
    }
};
        locals.var_qd = assign13130_e12192;
        locals.var_qd_dn3 = assign13130_e12192_d_n3;
        locals.var_qd_dn4 = assign13130_e12192_d_n4;
        locals.var_qd_dn5 = assign13130_e12192_d_n5;
        locals.var_qd_dn6 = assign13130_e12192_d_n6;
        locals.var_qd_dn7 = assign13130_e12192_d_n7;
        locals.var_qd_dn8 = assign13130_e12192_d_n8;
        locals.var_qd_rv = 0.0;

        let (assign13140_e12199, assign13140_e12199_d_n3, assign13140_e12199_d_n4, assign13140_e12199_d_n5, assign13140_e12199_d_n6, assign13140_e12199_d_n7, assign13140_e12199_d_n8,) = {
    if (locals.var_guard136 == 0.0) {
        let assign13140_e12197: f64 = (p.p2 * locals.var_qd);
        (assign13140_e12197, (p.p2 * locals.var_qd_dn3), (p.p2 * locals.var_qd_dn4), (p.p2 * locals.var_qd_dn5), (p.p2 * locals.var_qd_dn6), (p.p2 * locals.var_qd_dn7), (p.p2 * locals.var_qd_dn8),)
    } else {
        (locals.var_qsi, locals.var_qsi_dn3, locals.var_qsi_dn4, locals.var_qsi_dn5, locals.var_qsi_dn6, locals.var_qsi_dn7, locals.var_qsi_dn8,)
    }
};
        locals.var_qsi = assign13140_e12199;
        locals.var_qsi_dn3 = assign13140_e12199_d_n3;
        locals.var_qsi_dn4 = assign13140_e12199_d_n4;
        locals.var_qsi_dn5 = assign13140_e12199_d_n5;
        locals.var_qsi_dn6 = assign13140_e12199_d_n6;
        locals.var_qsi_dn7 = assign13140_e12199_d_n7;
        locals.var_qsi_dn8 = assign13140_e12199_d_n8;
        locals.var_qsi_rv = 0.0;

        let (assign13150_e12206, assign13150_e12206_d_n3, assign13150_e12206_d_n4, assign13150_e12206_d_n5, assign13150_e12206_d_n6, assign13150_e12206_d_n7, assign13150_e12206_d_n8,) = {
    if (locals.var_guard136 == 0.0) {
        let assign13150_e12204: f64 = (p.p2 * locals.var_qs);
        (assign13150_e12204, (p.p2 * locals.var_qs_dn3), (p.p2 * locals.var_qs_dn4), (p.p2 * locals.var_qs_dn5), (p.p2 * locals.var_qs_dn6), (p.p2 * locals.var_qs_dn7), (p.p2 * locals.var_qs_dn8),)
    } else {
        (locals.var_qdi, locals.var_qdi_dn3, locals.var_qdi_dn4, locals.var_qdi_dn5, locals.var_qdi_dn6, locals.var_qdi_dn7, locals.var_qdi_dn8,)
    }
};
        locals.var_qdi = assign13150_e12206;
        locals.var_qdi_dn3 = assign13150_e12206_d_n3;
        locals.var_qdi_dn4 = assign13150_e12206_d_n4;
        locals.var_qdi_dn5 = assign13150_e12206_d_n5;
        locals.var_qdi_dn6 = assign13150_e12206_d_n6;
        locals.var_qdi_dn7 = assign13150_e12206_d_n7;
        locals.var_qdi_dn8 = assign13150_e12206_d_n8;
        locals.var_qdi_rv = 0.0;

        let (assign13160_e12217, assign13160_e12217_d_n3, assign13160_e12217_d_n4, assign13160_e12217_d_n5, assign13160_e12217_d_n6, assign13160_e12217_d_n7, assign13160_e12217_d_n8,) = {
    if (locals.var_guard136 == 0.0) {
        let assign13160_e12212: f64 = (locals.var_qd - locals.var_qfgs_parasitic);
        let assign13160_e12213: f64 = (p.p2 * assign13160_e12212);
        let assign13160_e12215: f64 = (assign13160_e12213 + locals.var_qsbg);
        (assign13160_e12215, ((p.p2 * (locals.var_qd_dn3 - locals.var_qfgs_parasitic_dn3)) + locals.var_qsbg_dn3), ((p.p2 * (locals.var_qd_dn4 - locals.var_qfgs_parasitic_dn4)) + locals.var_qsbg_dn4), ((p.p2 * (locals.var_qd_dn5 - locals.var_qfgs_parasitic_dn5)) + locals.var_qsbg_dn5), ((p.p2 * (locals.var_qd_dn6 - locals.var_qfgs_parasitic_dn6)) + locals.var_qsbg_dn6), ((p.p2 * (locals.var_qd_dn7 - locals.var_qfgs_parasitic_dn7)) + locals.var_qsbg_dn7), ((p.p2 * (locals.var_qd_dn8 - locals.var_qfgs_parasitic_dn8)) + locals.var_qsbg_dn8),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    }
};
        locals.var_t0 = assign13160_e12217;
        locals.var_t0_dn3 = assign13160_e12217_d_n3;
        locals.var_t0_dn4 = assign13160_e12217_d_n4;
        locals.var_t0_dn5 = assign13160_e12217_d_n5;
        locals.var_t0_dn6 = assign13160_e12217_d_n6;
        locals.var_t0_dn7 = assign13160_e12217_d_n7;
        locals.var_t0_dn8 = assign13160_e12217_d_n8;
        locals.var_t0_rv = 0.0;

        let (assign13170_e12228, assign13170_e12228_d_n3, assign13170_e12228_d_n4, assign13170_e12228_d_n5, assign13170_e12228_d_n6, assign13170_e12228_d_n7, assign13170_e12228_d_n8,) = {
    if (locals.var_guard136 == 0.0) {
        let assign13170_e12223: f64 = (locals.var_qs - locals.var_qfgd_parasitic);
        let assign13170_e12224: f64 = (p.p2 * assign13170_e12223);
        let assign13170_e12226: f64 = (assign13170_e12224 + locals.var_qdbg);
        (assign13170_e12226, ((p.p2 * (locals.var_qs_dn3 - locals.var_qfgd_parasitic_dn3)) + locals.var_qdbg_dn3), ((p.p2 * (locals.var_qs_dn4 - locals.var_qfgd_parasitic_dn4)) + locals.var_qdbg_dn4), ((p.p2 * (locals.var_qs_dn5 - locals.var_qfgd_parasitic_dn5)) + locals.var_qdbg_dn5), ((p.p2 * (locals.var_qs_dn6 - locals.var_qfgd_parasitic_dn6)) + locals.var_qdbg_dn6), ((p.p2 * (locals.var_qs_dn7 - locals.var_qfgd_parasitic_dn7)) + locals.var_qdbg_dn7), ((p.p2 * (locals.var_qs_dn8 - locals.var_qfgd_parasitic_dn8)) + locals.var_qdbg_dn8),)
    } else {
        (locals.var_qd, locals.var_qd_dn3, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8,)
    }
};
        locals.var_qd = assign13170_e12228;
        locals.var_qd_dn3 = assign13170_e12228_d_n3;
        locals.var_qd_dn4 = assign13170_e12228_d_n4;
        locals.var_qd_dn5 = assign13170_e12228_d_n5;
        locals.var_qd_dn6 = assign13170_e12228_d_n6;
        locals.var_qd_dn7 = assign13170_e12228_d_n7;
        locals.var_qd_dn8 = assign13170_e12228_d_n8;
        locals.var_qd_rv = 0.0;

        let (assign13180_e12233, assign13180_e12233_d_n3, assign13180_e12233_d_n4, assign13180_e12233_d_n5, assign13180_e12233_d_n6, assign13180_e12233_d_n7, assign13180_e12233_d_n8,) = {
    if (locals.var_guard136 == 0.0) {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    } else {
        (locals.var_qs, locals.var_qs_dn3, locals.var_qs_dn4, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8,)
    }
};
        locals.var_qs = assign13180_e12233;
        locals.var_qs_dn3 = assign13180_e12233_d_n3;
        locals.var_qs_dn4 = assign13180_e12233_d_n4;
        locals.var_qs_dn5 = assign13180_e12233_d_n5;
        locals.var_qs_dn6 = assign13180_e12233_d_n6;
        locals.var_qs_dn7 = assign13180_e12233_d_n7;
        locals.var_qs_dn8 = assign13180_e12233_d_n8;
        locals.var_qs_rv = 0.0;

        let assign13190_e12238: f64 = (locals.var_qfgs_parasitic + locals.var_qfgd_parasitic);
        let assign13190_e12239: f64 = (p.p2 * assign13190_e12238);
        let assign13190_e12240: f64 = (locals.var_qfgi + assign13190_e12239);
        locals.var_qfg = assign13190_e12240;
        locals.var_qfg_dn3 = (locals.var_qfgi_dn3 + (p.p2 * (locals.var_qfgs_parasitic_dn3 + locals.var_qfgd_parasitic_dn3)));
        locals.var_qfg_dn4 = (locals.var_qfgi_dn4 + (p.p2 * (locals.var_qfgs_parasitic_dn4 + locals.var_qfgd_parasitic_dn4)));
        locals.var_qfg_dn5 = (locals.var_qfgi_dn5 + (p.p2 * (locals.var_qfgs_parasitic_dn5 + locals.var_qfgd_parasitic_dn5)));
        locals.var_qfg_dn6 = (locals.var_qfgi_dn6 + (p.p2 * (locals.var_qfgs_parasitic_dn6 + locals.var_qfgd_parasitic_dn6)));
        locals.var_qfg_dn7 = (locals.var_qfgi_dn7 + (p.p2 * (locals.var_qfgs_parasitic_dn7 + locals.var_qfgd_parasitic_dn7)));
        locals.var_qfg_dn8 = (locals.var_qfgi_dn8 + (p.p2 * (locals.var_qfgs_parasitic_dn8 + locals.var_qfgd_parasitic_dn8)));
        locals.var_qfg_rv = 0.0;

        let assign13200_e12243: f64 = (p.p2 * locals.var_qbg);
        let assign13200_e12245: f64 = (assign13200_e12243 - locals.var_qsbg);
        let assign13200_e12247: f64 = (assign13200_e12245 - locals.var_qdbg);
        locals.var_qbg = assign13200_e12247;
        locals.var_qbg_dn3 = (((p.p2 * locals.var_qbg_dn3) - locals.var_qsbg_dn3) - locals.var_qdbg_dn3);
        locals.var_qbg_dn4 = (((p.p2 * locals.var_qbg_dn4) - locals.var_qsbg_dn4) - locals.var_qdbg_dn4);
        locals.var_qbg_dn5 = (((p.p2 * locals.var_qbg_dn5) - locals.var_qsbg_dn5) - locals.var_qdbg_dn5);
        locals.var_qbg_dn6 = (((p.p2 * locals.var_qbg_dn6) - locals.var_qsbg_dn6) - locals.var_qdbg_dn6);
        locals.var_qbg_dn7 = (((p.p2 * locals.var_qbg_dn7) - locals.var_qsbg_dn7) - locals.var_qdbg_dn7);
        locals.var_qbg_dn8 = (((p.p2 * locals.var_qbg_dn8) - locals.var_qsbg_dn8) - locals.var_qdbg_dn8);
        locals.var_qbg_rv = 0.0;

        let assign13210_e12250: f64 = (p.p2 * locals.var_qfgs_parasitic);
        locals.var_qfgs_parasitic = assign13210_e12250;
        locals.var_qfgs_parasitic_dn3 = (p.p2 * locals.var_qfgs_parasitic_dn3);
        locals.var_qfgs_parasitic_dn4 = (p.p2 * locals.var_qfgs_parasitic_dn4);
        locals.var_qfgs_parasitic_dn5 = (p.p2 * locals.var_qfgs_parasitic_dn5);
        locals.var_qfgs_parasitic_dn6 = (p.p2 * locals.var_qfgs_parasitic_dn6);
        locals.var_qfgs_parasitic_dn7 = (p.p2 * locals.var_qfgs_parasitic_dn7);
        locals.var_qfgs_parasitic_dn8 = (p.p2 * locals.var_qfgs_parasitic_dn8);
        locals.var_qfgs_parasitic_rv = 0.0;

        let assign13220_e12253: f64 = (p.p2 * locals.var_qfgd_parasitic);
        locals.var_qfgd_parasitic = assign13220_e12253;
        locals.var_qfgd_parasitic_dn3 = (p.p2 * locals.var_qfgd_parasitic_dn3);
        locals.var_qfgd_parasitic_dn4 = (p.p2 * locals.var_qfgd_parasitic_dn4);
        locals.var_qfgd_parasitic_dn5 = (p.p2 * locals.var_qfgd_parasitic_dn5);
        locals.var_qfgd_parasitic_dn6 = (p.p2 * locals.var_qfgd_parasitic_dn6);
        locals.var_qfgd_parasitic_dn7 = (p.p2 * locals.var_qfgd_parasitic_dn7);
        locals.var_qfgd_parasitic_dn8 = (p.p2 * locals.var_qfgd_parasitic_dn8);
        locals.var_qfgd_parasitic_rv = 0.0;

        let assign13230_e12256: f64 = (locals.var_qsi + locals.var_qdi);
        let assign13230_e12257: f64 = (-assign13230_e12256);
        locals.var_qinv = assign13230_e12257;
        locals.var_qinv_dn3 = (-(locals.var_qsi_dn3 + locals.var_qdi_dn3));
        locals.var_qinv_dn4 = (-(locals.var_qsi_dn4 + locals.var_qdi_dn4));
        locals.var_qinv_dn5 = (-(locals.var_qsi_dn5 + locals.var_qdi_dn5));
        locals.var_qinv_dn6 = (-(locals.var_qsi_dn6 + locals.var_qdi_dn6));
        locals.var_qinv_dn7 = (-(locals.var_qsi_dn7 + locals.var_qdi_dn7));
        locals.var_qinv_dn8 = (-(locals.var_qsi_dn8 + locals.var_qdi_dn8));
        locals.var_qinv_rv = 0.0;

        let assign13240_e12260: f64 = (locals.var_utotal * locals.var_qinv);
        locals.var_t0 = assign13240_e12260;
        locals.var_t0_dn3 = ((locals.var_utotal_dn3 * locals.var_qinv) + (locals.var_utotal * locals.var_qinv_dn3));
        locals.var_t0_dn4 = ((locals.var_utotal_dn4 * locals.var_qinv) + (locals.var_utotal * locals.var_qinv_dn4));
        locals.var_t0_dn5 = ((locals.var_utotal_dn5 * locals.var_qinv) + (locals.var_utotal * locals.var_qinv_dn5));
        locals.var_t0_dn6 = ((locals.var_utotal_dn6 * locals.var_qinv) + (locals.var_utotal * locals.var_qinv_dn6));
        locals.var_t0_dn7 = ((locals.var_utotal_dn7 * locals.var_qinv) + (locals.var_utotal * locals.var_qinv_dn7));
        locals.var_t0_dn8 = ((locals.var_utotal_dn8 * locals.var_qinv) + (locals.var_utotal * locals.var_qinv_dn8));
        locals.var_t0_rv = 0.0;

        let assign13250_e12263: f64 = (locals.var_t0 * locals.var_rdsi);
        let assign13250_e12266: f64 = (locals.var_leff * locals.var_leff);
        let assign13250_e12267: f64 = (assign13250_e12263 + assign13250_e12266);
        locals.var_t1 = assign13250_e12267;
        locals.var_t1_dn3 = ((locals.var_t0_dn3 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn3));
        locals.var_t1_dn4 = ((locals.var_t0_dn4 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn4));
        locals.var_t1_dn5 = ((locals.var_t0_dn5 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn5));
        locals.var_t1_dn6 = ((locals.var_t0_dn6 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn6));
        locals.var_t1_dn7 = ((locals.var_t0_dn7 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn7));
        locals.var_t1_dn8 = ((locals.var_t0_dn8 * locals.var_rdsi) + (locals.var_t0 * locals.var_rdsi_dn8));
        locals.var_t1_rv = 0.0;

        let assign13290_e12287: f64 = if ((p.p20 == 1.0) && (locals.var_xrcrg1_i != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard137 = assign13290_e12287;
        locals.var_guard137_rv = 0.0;

        let (assign13300_e12297, assign13300_e12297_d_n3, assign13300_e12297_d_n4, assign13300_e12297_d_n5, assign13300_e12297_d_n6, assign13300_e12297_d_n7, assign13300_e12297_d_n8,) = {
    if (locals.var_guard137 != 0.0) {
        let assign13300_e12291: f64 = (locals.var_utotal * locals.var_cox1);
        let assign13300_e12293: f64 = (assign13300_e12291 * locals.var_weff);
        let assign13300_e12295: f64 = (assign13300_e12293 / locals.var_leff);
        (assign13300_e12295, (((locals.var_utotal_dn3 * locals.var_cox1) * locals.var_weff) / locals.var_leff), (((locals.var_utotal_dn4 * locals.var_cox1) * locals.var_weff) / locals.var_leff), (((locals.var_utotal_dn5 * locals.var_cox1) * locals.var_weff) / locals.var_leff), (((locals.var_utotal_dn6 * locals.var_cox1) * locals.var_weff) / locals.var_leff), (((locals.var_utotal_dn7 * locals.var_cox1) * locals.var_weff) / locals.var_leff), (((locals.var_utotal_dn8 * locals.var_cox1) * locals.var_weff) / locals.var_leff),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8,)
    }
};
        locals.var_t0 = assign13300_e12297;
        locals.var_t0_dn3 = assign13300_e12297_d_n3;
        locals.var_t0_dn4 = assign13300_e12297_d_n4;
        locals.var_t0_dn5 = assign13300_e12297_d_n5;
        locals.var_t0_dn6 = assign13300_e12297_d_n6;
        locals.var_t0_dn7 = assign13300_e12297_d_n7;
        locals.var_t0_dn8 = assign13300_e12297_d_n8;
        locals.var_t0_rv = 0.0;

        let assign13510_e12396: f64 = if ((p.p18 != 0.0) && (p.p310 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard147 = assign13510_e12396;
        locals.var_guard147_rv = 0.0;

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq0_e787, eq0_e787_d_n3, eq0_e787_d_n4, eq0_e787_d_n5, eq0_e787_d_n6, eq0_e787_d_n7, eq0_e787_d_n8,) = {
    if (locals.var_guard138 != 0.0) {
        let eq0_e779: f64 = (locals.var_devsign * locals.var_ids);
        let eq0_e779_d_n3: f64 = (locals.var_devsign * locals.var_ids_dn3);
        let eq0_e779_d_n4: f64 = (locals.var_devsign * locals.var_ids_dn4);
        let eq0_e779_d_n5: f64 = (locals.var_devsign * locals.var_ids_dn5);
        let eq0_e779_d_n6: f64 = (locals.var_devsign * locals.var_ids_dn6);
        let eq0_e779_d_n7: f64 = (locals.var_devsign * locals.var_ids_dn7);
        let eq0_e779_d_n8: f64 = (locals.var_devsign * locals.var_ids_dn8);
        let eq0_e782: f64 = 1e-12;
        let eq0_e784: f64 = (eq0_e782 * (nv5 - nv6));
        let eq0_e785: f64 = (eq0_e779 + eq0_e784);
        let eq0_e785_d_n5: f64 = (eq0_e779_d_n5 + eq0_e782);
        let eq0_e785_d_n6: f64 = (eq0_e779_d_n6 + (-eq0_e782));
        (eq0_e785, eq0_e779_d_n3, eq0_e779_d_n4, eq0_e785_d_n5, eq0_e785_d_n6, eq0_e779_d_n7, eq0_e779_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e787;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq0_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq0_e787_d_n3), multiplicity * (eq0_e787_d_n4), multiplicity * (eq0_e787_d_n5), multiplicity * (eq0_e787_d_n6), multiplicity * (eq0_e787_d_n7), multiplicity * (eq0_e787_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq1_e795, eq1_e795_d_n3, eq1_e795_d_n4, eq1_e795_d_n5, eq1_e795_d_n6, eq1_e795_d_n7, eq1_e795_d_n8,) = {
    if (locals.var_guard138 != 0.0) {
        let eq1_e792: f64 = (locals.var_igidl + locals.var_iii);
        let eq1_e792_d_n3: f64 = (locals.var_igidl_dn3 + locals.var_iii_dn3);
        let eq1_e792_d_n4: f64 = (locals.var_igidl_dn4 + locals.var_iii_dn4);
        let eq1_e792_d_n5: f64 = (locals.var_igidl_dn5 + locals.var_iii_dn5);
        let eq1_e792_d_n6: f64 = (locals.var_igidl_dn6 + locals.var_iii_dn6);
        let eq1_e792_d_n7: f64 = (locals.var_igidl_dn7 + locals.var_iii_dn7);
        let eq1_e792_d_n8: f64 = (locals.var_igidl_dn8 + locals.var_iii_dn8);
        let eq1_e793: f64 = (locals.var_devsign * eq1_e792);
        let eq1_e793_d_n3: f64 = (locals.var_devsign * eq1_e792_d_n3);
        let eq1_e793_d_n4: f64 = (locals.var_devsign * eq1_e792_d_n4);
        let eq1_e793_d_n5: f64 = (locals.var_devsign * eq1_e792_d_n5);
        let eq1_e793_d_n6: f64 = (locals.var_devsign * eq1_e792_d_n6);
        let eq1_e793_d_n7: f64 = (locals.var_devsign * eq1_e792_d_n7);
        let eq1_e793_d_n8: f64 = (locals.var_devsign * eq1_e792_d_n8);
        (eq1_e793, eq1_e793_d_n3, eq1_e793_d_n4, eq1_e793_d_n5, eq1_e793_d_n6, eq1_e793_d_n7, eq1_e793_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e795;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq1_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq1_e795_d_n3), multiplicity * (eq1_e795_d_n4), multiplicity * (eq1_e795_d_n5), multiplicity * (eq1_e795_d_n6), multiplicity * (eq1_e795_d_n7), multiplicity * (eq1_e795_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq2_e801, eq2_e801_d_n3, eq2_e801_d_n4, eq2_e801_d_n5, eq2_e801_d_n6, eq2_e801_d_n7, eq2_e801_d_n8,) = {
    if (locals.var_guard138 != 0.0) {
        let eq2_e799: f64 = (locals.var_devsign * locals.var_igisl);
        let eq2_e799_d_n3: f64 = (locals.var_devsign * locals.var_igisl_dn3);
        let eq2_e799_d_n4: f64 = (locals.var_devsign * locals.var_igisl_dn4);
        let eq2_e799_d_n5: f64 = (locals.var_devsign * locals.var_igisl_dn5);
        let eq2_e799_d_n6: f64 = (locals.var_devsign * locals.var_igisl_dn6);
        let eq2_e799_d_n7: f64 = (locals.var_devsign * locals.var_igisl_dn7);
        let eq2_e799_d_n8: f64 = (locals.var_devsign * locals.var_igisl_dn8);
        (eq2_e799, eq2_e799_d_n3, eq2_e799_d_n4, eq2_e799_d_n5, eq2_e799_d_n6, eq2_e799_d_n7, eq2_e799_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e801;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(5),
            multiplicity * (eq2_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq2_e801_d_n3), multiplicity * (eq2_e801_d_n4), multiplicity * (eq2_e801_d_n5), multiplicity * (eq2_e801_d_n6), multiplicity * (eq2_e801_d_n7), multiplicity * (eq2_e801_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq3_e809, eq3_e809_d_n3, eq3_e809_d_n4, eq3_e809_d_n5, eq3_e809_d_n6, eq3_e809_d_n7, eq3_e809_d_n8,) = {
    if (locals.var_guard138 != 0.0) {
        let eq3_e806: f64 = (locals.var_igcs + locals.var_igs);
        let eq3_e806_d_n3: f64 = (locals.var_igcs_dn3 + locals.var_igs_dn3);
        let eq3_e806_d_n4: f64 = (locals.var_igcs_dn4 + locals.var_igs_dn4);
        let eq3_e806_d_n5: f64 = (locals.var_igcs_dn5 + locals.var_igs_dn5);
        let eq3_e806_d_n6: f64 = (locals.var_igcs_dn6 + locals.var_igs_dn6);
        let eq3_e806_d_n7: f64 = (locals.var_igcs_dn7 + locals.var_igs_dn7);
        let eq3_e806_d_n8: f64 = (locals.var_igcs_dn8 + locals.var_igs_dn8);
        let eq3_e807: f64 = (locals.var_devsign * eq3_e806);
        let eq3_e807_d_n3: f64 = (locals.var_devsign * eq3_e806_d_n3);
        let eq3_e807_d_n4: f64 = (locals.var_devsign * eq3_e806_d_n4);
        let eq3_e807_d_n5: f64 = (locals.var_devsign * eq3_e806_d_n5);
        let eq3_e807_d_n6: f64 = (locals.var_devsign * eq3_e806_d_n6);
        let eq3_e807_d_n7: f64 = (locals.var_devsign * eq3_e806_d_n7);
        let eq3_e807_d_n8: f64 = (locals.var_devsign * eq3_e806_d_n8);
        (eq3_e807, eq3_e807_d_n3, eq3_e807_d_n4, eq3_e807_d_n5, eq3_e807_d_n6, eq3_e807_d_n7, eq3_e807_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e809;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq3_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq3_e809_d_n3), multiplicity * (eq3_e809_d_n4), multiplicity * (eq3_e809_d_n5), multiplicity * (eq3_e809_d_n6), multiplicity * (eq3_e809_d_n7), multiplicity * (eq3_e809_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq4_e817, eq4_e817_d_n3, eq4_e817_d_n4, eq4_e817_d_n5, eq4_e817_d_n6, eq4_e817_d_n7, eq4_e817_d_n8,) = {
    if (locals.var_guard138 != 0.0) {
        let eq4_e814: f64 = (locals.var_igcd + locals.var_igd);
        let eq4_e814_d_n3: f64 = (locals.var_igcd_dn3 + locals.var_igd_dn3);
        let eq4_e814_d_n4: f64 = (locals.var_igcd_dn4 + locals.var_igd_dn4);
        let eq4_e814_d_n5: f64 = (locals.var_igcd_dn5 + locals.var_igd_dn5);
        let eq4_e814_d_n6: f64 = (locals.var_igcd_dn6 + locals.var_igd_dn6);
        let eq4_e814_d_n7: f64 = (locals.var_igcd_dn7 + locals.var_igd_dn7);
        let eq4_e814_d_n8: f64 = (locals.var_igcd_dn8 + locals.var_igd_dn8);
        let eq4_e815: f64 = (locals.var_devsign * eq4_e814);
        let eq4_e815_d_n3: f64 = (locals.var_devsign * eq4_e814_d_n3);
        let eq4_e815_d_n4: f64 = (locals.var_devsign * eq4_e814_d_n4);
        let eq4_e815_d_n5: f64 = (locals.var_devsign * eq4_e814_d_n5);
        let eq4_e815_d_n6: f64 = (locals.var_devsign * eq4_e814_d_n6);
        let eq4_e815_d_n7: f64 = (locals.var_devsign * eq4_e814_d_n7);
        let eq4_e815_d_n8: f64 = (locals.var_devsign * eq4_e814_d_n8);
        (eq4_e815, eq4_e815_d_n3, eq4_e815_d_n4, eq4_e815_d_n5, eq4_e815_d_n6, eq4_e815_d_n7, eq4_e815_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e817;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq4_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq4_e817_d_n3), multiplicity * (eq4_e817_d_n4), multiplicity * (eq4_e817_d_n5), multiplicity * (eq4_e817_d_n6), multiplicity * (eq4_e817_d_n7), multiplicity * (eq4_e817_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq5_e830, eq5_e830_d_n3, eq5_e830_d_n4, eq5_e830_d_n5, eq5_e830_d_n6, eq5_e830_d_n7, eq5_e830_d_n8,) = {
    if (locals.var_guard138 == 0.0) {
        let eq5_e822: f64 = (locals.var_devsign * locals.var_ids);
        let eq5_e822_d_n3: f64 = (locals.var_devsign * locals.var_ids_dn3);
        let eq5_e822_d_n4: f64 = (locals.var_devsign * locals.var_ids_dn4);
        let eq5_e822_d_n5: f64 = (locals.var_devsign * locals.var_ids_dn5);
        let eq5_e822_d_n6: f64 = (locals.var_devsign * locals.var_ids_dn6);
        let eq5_e822_d_n7: f64 = (locals.var_devsign * locals.var_ids_dn7);
        let eq5_e822_d_n8: f64 = (locals.var_devsign * locals.var_ids_dn8);
        let eq5_e825: f64 = 1e-12;
        let eq5_e827: f64 = (eq5_e825 * (nv6 - nv5));
        let eq5_e828: f64 = (eq5_e822 + eq5_e827);
        let eq5_e828_d_n5: f64 = (eq5_e822_d_n5 + (-eq5_e825));
        let eq5_e828_d_n6: f64 = (eq5_e822_d_n6 + eq5_e825);
        (eq5_e828, eq5_e822_d_n3, eq5_e822_d_n4, eq5_e828_d_n5, eq5_e828_d_n6, eq5_e822_d_n7, eq5_e822_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e830;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(5),
            multiplicity * (eq5_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq5_e830_d_n3), multiplicity * (eq5_e830_d_n4), multiplicity * (eq5_e830_d_n5), multiplicity * (eq5_e830_d_n6), multiplicity * (eq5_e830_d_n7), multiplicity * (eq5_e830_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq6_e839, eq6_e839_d_n3, eq6_e839_d_n4, eq6_e839_d_n5, eq6_e839_d_n6, eq6_e839_d_n7, eq6_e839_d_n8,) = {
    if (locals.var_guard138 == 0.0) {
        let eq6_e836: f64 = (locals.var_igidl + locals.var_iii);
        let eq6_e836_d_n3: f64 = (locals.var_igidl_dn3 + locals.var_iii_dn3);
        let eq6_e836_d_n4: f64 = (locals.var_igidl_dn4 + locals.var_iii_dn4);
        let eq6_e836_d_n5: f64 = (locals.var_igidl_dn5 + locals.var_iii_dn5);
        let eq6_e836_d_n6: f64 = (locals.var_igidl_dn6 + locals.var_iii_dn6);
        let eq6_e836_d_n7: f64 = (locals.var_igidl_dn7 + locals.var_iii_dn7);
        let eq6_e836_d_n8: f64 = (locals.var_igidl_dn8 + locals.var_iii_dn8);
        let eq6_e837: f64 = (locals.var_devsign * eq6_e836);
        let eq6_e837_d_n3: f64 = (locals.var_devsign * eq6_e836_d_n3);
        let eq6_e837_d_n4: f64 = (locals.var_devsign * eq6_e836_d_n4);
        let eq6_e837_d_n5: f64 = (locals.var_devsign * eq6_e836_d_n5);
        let eq6_e837_d_n6: f64 = (locals.var_devsign * eq6_e836_d_n6);
        let eq6_e837_d_n7: f64 = (locals.var_devsign * eq6_e836_d_n7);
        let eq6_e837_d_n8: f64 = (locals.var_devsign * eq6_e836_d_n8);
        (eq6_e837, eq6_e837_d_n3, eq6_e837_d_n4, eq6_e837_d_n5, eq6_e837_d_n6, eq6_e837_d_n7, eq6_e837_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e839;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(5),
            multiplicity * (eq6_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq6_e839_d_n3), multiplicity * (eq6_e839_d_n4), multiplicity * (eq6_e839_d_n5), multiplicity * (eq6_e839_d_n6), multiplicity * (eq6_e839_d_n7), multiplicity * (eq6_e839_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq7_e846, eq7_e846_d_n3, eq7_e846_d_n4, eq7_e846_d_n5, eq7_e846_d_n6, eq7_e846_d_n7, eq7_e846_d_n8,) = {
    if (locals.var_guard138 == 0.0) {
        let eq7_e844: f64 = (locals.var_devsign * locals.var_igisl);
        let eq7_e844_d_n3: f64 = (locals.var_devsign * locals.var_igisl_dn3);
        let eq7_e844_d_n4: f64 = (locals.var_devsign * locals.var_igisl_dn4);
        let eq7_e844_d_n5: f64 = (locals.var_devsign * locals.var_igisl_dn5);
        let eq7_e844_d_n6: f64 = (locals.var_devsign * locals.var_igisl_dn6);
        let eq7_e844_d_n7: f64 = (locals.var_devsign * locals.var_igisl_dn7);
        let eq7_e844_d_n8: f64 = (locals.var_devsign * locals.var_igisl_dn8);
        (eq7_e844, eq7_e844_d_n3, eq7_e844_d_n4, eq7_e844_d_n5, eq7_e844_d_n6, eq7_e844_d_n7, eq7_e844_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e846;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq7_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq7_e846_d_n3), multiplicity * (eq7_e846_d_n4), multiplicity * (eq7_e846_d_n5), multiplicity * (eq7_e846_d_n6), multiplicity * (eq7_e846_d_n7), multiplicity * (eq7_e846_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq8_e855, eq8_e855_d_n3, eq8_e855_d_n4, eq8_e855_d_n5, eq8_e855_d_n6, eq8_e855_d_n7, eq8_e855_d_n8,) = {
    if (locals.var_guard138 == 0.0) {
        let eq8_e852: f64 = (locals.var_igcs + locals.var_igs);
        let eq8_e852_d_n3: f64 = (locals.var_igcs_dn3 + locals.var_igs_dn3);
        let eq8_e852_d_n4: f64 = (locals.var_igcs_dn4 + locals.var_igs_dn4);
        let eq8_e852_d_n5: f64 = (locals.var_igcs_dn5 + locals.var_igs_dn5);
        let eq8_e852_d_n6: f64 = (locals.var_igcs_dn6 + locals.var_igs_dn6);
        let eq8_e852_d_n7: f64 = (locals.var_igcs_dn7 + locals.var_igs_dn7);
        let eq8_e852_d_n8: f64 = (locals.var_igcs_dn8 + locals.var_igs_dn8);
        let eq8_e853: f64 = (locals.var_devsign * eq8_e852);
        let eq8_e853_d_n3: f64 = (locals.var_devsign * eq8_e852_d_n3);
        let eq8_e853_d_n4: f64 = (locals.var_devsign * eq8_e852_d_n4);
        let eq8_e853_d_n5: f64 = (locals.var_devsign * eq8_e852_d_n5);
        let eq8_e853_d_n6: f64 = (locals.var_devsign * eq8_e852_d_n6);
        let eq8_e853_d_n7: f64 = (locals.var_devsign * eq8_e852_d_n7);
        let eq8_e853_d_n8: f64 = (locals.var_devsign * eq8_e852_d_n8);
        (eq8_e853, eq8_e853_d_n3, eq8_e853_d_n4, eq8_e853_d_n5, eq8_e853_d_n6, eq8_e853_d_n7, eq8_e853_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e855;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq8_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq8_e855_d_n3), multiplicity * (eq8_e855_d_n4), multiplicity * (eq8_e855_d_n5), multiplicity * (eq8_e855_d_n6), multiplicity * (eq8_e855_d_n7), multiplicity * (eq8_e855_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq9_e864, eq9_e864_d_n3, eq9_e864_d_n4, eq9_e864_d_n5, eq9_e864_d_n6, eq9_e864_d_n7, eq9_e864_d_n8,) = {
    if (locals.var_guard138 == 0.0) {
        let eq9_e861: f64 = (locals.var_igcd + locals.var_igd);
        let eq9_e861_d_n3: f64 = (locals.var_igcd_dn3 + locals.var_igd_dn3);
        let eq9_e861_d_n4: f64 = (locals.var_igcd_dn4 + locals.var_igd_dn4);
        let eq9_e861_d_n5: f64 = (locals.var_igcd_dn5 + locals.var_igd_dn5);
        let eq9_e861_d_n6: f64 = (locals.var_igcd_dn6 + locals.var_igd_dn6);
        let eq9_e861_d_n7: f64 = (locals.var_igcd_dn7 + locals.var_igd_dn7);
        let eq9_e861_d_n8: f64 = (locals.var_igcd_dn8 + locals.var_igd_dn8);
        let eq9_e862: f64 = (locals.var_devsign * eq9_e861);
        let eq9_e862_d_n3: f64 = (locals.var_devsign * eq9_e861_d_n3);
        let eq9_e862_d_n4: f64 = (locals.var_devsign * eq9_e861_d_n4);
        let eq9_e862_d_n5: f64 = (locals.var_devsign * eq9_e861_d_n5);
        let eq9_e862_d_n6: f64 = (locals.var_devsign * eq9_e861_d_n6);
        let eq9_e862_d_n7: f64 = (locals.var_devsign * eq9_e861_d_n7);
        let eq9_e862_d_n8: f64 = (locals.var_devsign * eq9_e861_d_n8);
        (eq9_e862, eq9_e862_d_n3, eq9_e862_d_n4, eq9_e862_d_n5, eq9_e862_d_n6, eq9_e862_d_n7, eq9_e862_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e864;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq9_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq9_e864_d_n3), multiplicity * (eq9_e864_d_n4), multiplicity * (eq9_e864_d_n5), multiplicity * (eq9_e864_d_n6), multiplicity * (eq9_e864_d_n7), multiplicity * (eq9_e864_d_n8)],
            [],
            [],
            1.0,
        );
        let eq10_e867: f64 = (locals.var_devsign * locals.var_igbs);
        let eq10_e867_d_n3: f64 = (locals.var_devsign * locals.var_igbs_dn3);
        let eq10_e867_d_n4: f64 = (locals.var_devsign * locals.var_igbs_dn4);
        let eq10_e867_d_n5: f64 = (locals.var_devsign * locals.var_igbs_dn5);
        let eq10_e867_d_n6: f64 = (locals.var_devsign * locals.var_igbs_dn6);
        let eq10_e867_d_n7: f64 = (locals.var_devsign * locals.var_igbs_dn7);
        let eq10_e867_d_n8: f64 = (locals.var_devsign * locals.var_igbs_dn8);
        let eq10_value: f64 = eq10_e867;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq10_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq10_e867_d_n3), multiplicity * (eq10_e867_d_n4), multiplicity * (eq10_e867_d_n5), multiplicity * (eq10_e867_d_n6), multiplicity * (eq10_e867_d_n7), multiplicity * (eq10_e867_d_n8)],
            [],
            [],
            1.0,
        );
        let eq11_e870: f64 = (locals.var_devsign * locals.var_igbd);
        let eq11_e870_d_n3: f64 = (locals.var_devsign * locals.var_igbd_dn3);
        let eq11_e870_d_n4: f64 = (locals.var_devsign * locals.var_igbd_dn4);
        let eq11_e870_d_n5: f64 = (locals.var_devsign * locals.var_igbd_dn5);
        let eq11_e870_d_n6: f64 = (locals.var_devsign * locals.var_igbd_dn6);
        let eq11_e870_d_n7: f64 = (locals.var_devsign * locals.var_igbd_dn7);
        let eq11_e870_d_n8: f64 = (locals.var_devsign * locals.var_igbd_dn8);
        let eq11_value: f64 = eq11_e870;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq11_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq11_e870_d_n3), multiplicity * (eq11_e870_d_n4), multiplicity * (eq11_e870_d_n5), multiplicity * (eq11_e870_d_n6), multiplicity * (eq11_e870_d_n7), multiplicity * (eq11_e870_d_n8)],
            [],
            [],
            1.0,
        );
        let eq12_e873: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, locals.var_qdi);
        let eq12_e874: f64 = (locals.var_devsign * eq12_e873);
        let eq12_e874_d_n3: f64 = (locals.var_devsign * (locals.var_qdi_dn3 * ddt_scale));
        let eq12_e874_d_n4: f64 = (locals.var_devsign * (locals.var_qdi_dn4 * ddt_scale));
        let eq12_e874_d_n5: f64 = (locals.var_devsign * (locals.var_qdi_dn5 * ddt_scale));
        let eq12_e874_d_n6: f64 = (locals.var_devsign * (locals.var_qdi_dn6 * ddt_scale));
        let eq12_e874_d_n7: f64 = (locals.var_devsign * (locals.var_qdi_dn7 * ddt_scale));
        let eq12_e874_d_n8: f64 = (locals.var_devsign * (locals.var_qdi_dn8 * ddt_scale));
        let eq12_value: f64 = eq12_e874;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq12_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq12_e874_d_n3), multiplicity * (eq12_e874_d_n4), multiplicity * (eq12_e874_d_n5), multiplicity * (eq12_e874_d_n6), multiplicity * (eq12_e874_d_n7), multiplicity * (eq12_e874_d_n8)],
            [],
            [],
            1.0,
        );
        let eq13_e876: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, locals.var_qfgi);
        let eq13_value: f64 = eq13_e876;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq13_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * ((locals.var_qfgi_dn3 * ddt_scale)), multiplicity * ((locals.var_qfgi_dn4 * ddt_scale)), multiplicity * ((locals.var_qfgi_dn5 * ddt_scale)), multiplicity * ((locals.var_qfgi_dn6 * ddt_scale)), multiplicity * ((locals.var_qfgi_dn7 * ddt_scale)), multiplicity * ((locals.var_qfgi_dn8 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq14_e879: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, locals.var_qbgi);
        let eq14_e880: f64 = (locals.var_devsign * eq14_e879);
        let eq14_e880_d_n3: f64 = (locals.var_devsign * (locals.var_qbgi_dn3 * ddt_scale));
        let eq14_e880_d_n4: f64 = (locals.var_devsign * (locals.var_qbgi_dn4 * ddt_scale));
        let eq14_e880_d_n5: f64 = (locals.var_devsign * (locals.var_qbgi_dn5 * ddt_scale));
        let eq14_e880_d_n6: f64 = (locals.var_devsign * (locals.var_qbgi_dn6 * ddt_scale));
        let eq14_e880_d_n7: f64 = (locals.var_devsign * (locals.var_qbgi_dn7 * ddt_scale));
        let eq14_e880_d_n8: f64 = (locals.var_devsign * (locals.var_qbgi_dn8 * ddt_scale));
        let eq14_value: f64 = eq14_e880;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(6),
            multiplicity * (eq14_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq14_e880_d_n3), multiplicity * (eq14_e880_d_n4), multiplicity * (eq14_e880_d_n5), multiplicity * (eq14_e880_d_n6), multiplicity * (eq14_e880_d_n7), multiplicity * (eq14_e880_d_n8)],
            [],
            [],
            1.0,
        );
        let eq15_e882: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, locals.var_qfgs_parasitic);
        let eq15_value: f64 = eq15_e882;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq15_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * ((locals.var_qfgs_parasitic_dn3 * ddt_scale)), multiplicity * ((locals.var_qfgs_parasitic_dn4 * ddt_scale)), multiplicity * ((locals.var_qfgs_parasitic_dn5 * ddt_scale)), multiplicity * ((locals.var_qfgs_parasitic_dn6 * ddt_scale)), multiplicity * ((locals.var_qfgs_parasitic_dn7 * ddt_scale)), multiplicity * ((locals.var_qfgs_parasitic_dn8 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq16_e884: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, locals.var_qfgd_parasitic);
        let eq16_value: f64 = eq16_e884;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(5),
            multiplicity * (eq16_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * ((locals.var_qfgd_parasitic_dn3 * ddt_scale)), multiplicity * ((locals.var_qfgd_parasitic_dn4 * ddt_scale)), multiplicity * ((locals.var_qfgd_parasitic_dn5 * ddt_scale)), multiplicity * ((locals.var_qfgd_parasitic_dn6 * ddt_scale)), multiplicity * ((locals.var_qfgd_parasitic_dn7 * ddt_scale)), multiplicity * ((locals.var_qfgd_parasitic_dn8 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq17_e887: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, locals.var_qsbg);
        let eq17_e888: f64 = (locals.var_devsign * eq17_e887);
        let eq17_e888_d_n3: f64 = (locals.var_devsign * (locals.var_qsbg_dn3 * ddt_scale));
        let eq17_e888_d_n4: f64 = (locals.var_devsign * (locals.var_qsbg_dn4 * ddt_scale));
        let eq17_e888_d_n5: f64 = (locals.var_devsign * (locals.var_qsbg_dn5 * ddt_scale));
        let eq17_e888_d_n6: f64 = (locals.var_devsign * (locals.var_qsbg_dn6 * ddt_scale));
        let eq17_e888_d_n7: f64 = (locals.var_devsign * (locals.var_qsbg_dn7 * ddt_scale));
        let eq17_e888_d_n8: f64 = (locals.var_devsign * (locals.var_qsbg_dn8 * ddt_scale));
        let eq17_value: f64 = eq17_e888;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(3),
            multiplicity * (eq17_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq17_e888_d_n3), multiplicity * (eq17_e888_d_n4), multiplicity * (eq17_e888_d_n5), multiplicity * (eq17_e888_d_n6), multiplicity * (eq17_e888_d_n7), multiplicity * (eq17_e888_d_n8)],
            [],
            [],
            1.0,
        );
        let eq18_e891: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, locals.var_qdbg);
        let eq18_e892: f64 = (locals.var_devsign * eq18_e891);
        let eq18_e892_d_n3: f64 = (locals.var_devsign * (locals.var_qdbg_dn3 * ddt_scale));
        let eq18_e892_d_n4: f64 = (locals.var_devsign * (locals.var_qdbg_dn4 * ddt_scale));
        let eq18_e892_d_n5: f64 = (locals.var_devsign * (locals.var_qdbg_dn5 * ddt_scale));
        let eq18_e892_d_n6: f64 = (locals.var_devsign * (locals.var_qdbg_dn6 * ddt_scale));
        let eq18_e892_d_n7: f64 = (locals.var_devsign * (locals.var_qdbg_dn7 * ddt_scale));
        let eq18_e892_d_n8: f64 = (locals.var_devsign * (locals.var_qdbg_dn8 * ddt_scale));
        let eq18_value: f64 = eq18_e892;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(3),
            multiplicity * (eq18_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq18_e892_d_n3), multiplicity * (eq18_e892_d_n4), multiplicity * (eq18_e892_d_n5), multiplicity * (eq18_e892_d_n6), multiplicity * (eq18_e892_d_n7), multiplicity * (eq18_e892_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq21_e907, eq21_e907_d_n0, eq21_e907_d_n3, eq21_e907_d_n4, eq21_e907_d_n5, eq21_e907_d_n6, eq21_e907_d_n7, eq21_e907_d_n8,) = {
    if (locals.var_guard139 == 0.0) {
        let eq21_e905: f64 = ((nv0 - nv5) * locals.var_gdpr);
        let eq21_e905_d_n3: f64 = ((nv0 - nv5) * locals.var_gdpr_dn3);
        let eq21_e905_d_n4: f64 = ((nv0 - nv5) * locals.var_gdpr_dn4);
        let eq21_e905_d_n5: f64 = ((-locals.var_gdpr) + ((nv0 - nv5) * locals.var_gdpr_dn5));
        let eq21_e905_d_n6: f64 = ((nv0 - nv5) * locals.var_gdpr_dn6);
        let eq21_e905_d_n7: f64 = ((nv0 - nv5) * locals.var_gdpr_dn7);
        let eq21_e905_d_n8: f64 = ((nv0 - nv5) * locals.var_gdpr_dn8);
        (eq21_e905, locals.var_gdpr, eq21_e905_d_n3, eq21_e905_d_n4, eq21_e905_d_n5, eq21_e905_d_n6, eq21_e905_d_n7, eq21_e905_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e907;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(0),
            Some(5),
            multiplicity * (eq21_value),
            [0, 3, 4, 5, 6, 7, 8],
            [multiplicity * (eq21_e907_d_n0), multiplicity * (eq21_e907_d_n3), multiplicity * (eq21_e907_d_n4), multiplicity * (eq21_e907_d_n5), multiplicity * (eq21_e907_d_n6), multiplicity * (eq21_e907_d_n7), multiplicity * (eq21_e907_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq22_e914, eq22_e914_d_n2, eq22_e914_d_n3, eq22_e914_d_n4, eq22_e914_d_n5, eq22_e914_d_n6, eq22_e914_d_n7, eq22_e914_d_n8,) = {
    if (locals.var_guard139 == 0.0) {
        let eq22_e912: f64 = ((nv2 - nv6) * locals.var_gspr);
        let eq22_e912_d_n3: f64 = ((nv2 - nv6) * locals.var_gspr_dn3);
        let eq22_e912_d_n4: f64 = ((nv2 - nv6) * locals.var_gspr_dn4);
        let eq22_e912_d_n5: f64 = ((nv2 - nv6) * locals.var_gspr_dn5);
        let eq22_e912_d_n6: f64 = ((-locals.var_gspr) + ((nv2 - nv6) * locals.var_gspr_dn6));
        let eq22_e912_d_n7: f64 = ((nv2 - nv6) * locals.var_gspr_dn7);
        let eq22_e912_d_n8: f64 = ((nv2 - nv6) * locals.var_gspr_dn8);
        (eq22_e912, locals.var_gspr, eq22_e912_d_n3, eq22_e912_d_n4, eq22_e912_d_n5, eq22_e912_d_n6, eq22_e912_d_n7, eq22_e912_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e914;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(2),
            Some(6),
            multiplicity * (eq22_value),
            [2, 3, 4, 5, 6, 7, 8],
            [multiplicity * (eq22_e914_d_n2), multiplicity * (eq22_e914_d_n3), multiplicity * (eq22_e914_d_n4), multiplicity * (eq22_e914_d_n5), multiplicity * (eq22_e914_d_n6), multiplicity * (eq22_e914_d_n7), multiplicity * (eq22_e914_d_n8)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq25_e938, eq25_e938_d_n3, eq25_e938_d_n4, eq25_e938_d_n5, eq25_e938_d_n6, eq25_e938_d_n7, eq25_e938_d_n8,) = {
    if (locals.var_guard140 != 0.0) {
        let eq25_e936: f64 = ((nv7 - nv8) * locals.var_gcrg);
        let eq25_e936_d_n3: f64 = ((nv7 - nv8) * locals.var_gcrg_dn3);
        let eq25_e936_d_n4: f64 = ((nv7 - nv8) * locals.var_gcrg_dn4);
        let eq25_e936_d_n5: f64 = ((nv7 - nv8) * locals.var_gcrg_dn5);
        let eq25_e936_d_n6: f64 = ((nv7 - nv8) * locals.var_gcrg_dn6);
        let eq25_e936_d_n7: f64 = (locals.var_gcrg + ((nv7 - nv8) * locals.var_gcrg_dn7));
        let eq25_e936_d_n8: f64 = ((-locals.var_gcrg) + ((nv7 - nv8) * locals.var_gcrg_dn8));
        (eq25_e936, eq25_e936_d_n3, eq25_e936_d_n4, eq25_e936_d_n5, eq25_e936_d_n6, eq25_e936_d_n7, eq25_e936_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e938;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(8),
            multiplicity * (eq25_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq25_e938_d_n3), multiplicity * (eq25_e938_d_n4), multiplicity * (eq25_e938_d_n5), multiplicity * (eq25_e938_d_n6), multiplicity * (eq25_e938_d_n7), multiplicity * (eq25_e938_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq38_e1079, eq38_e1079_d_n0, eq38_e1079_d_n2, eq38_e1079_d_n3, eq38_e1079_d_n4, eq38_e1079_d_n5, eq38_e1079_d_n6, eq38_e1079_d_n7, eq38_e1079_d_n8,) = {
    if ((locals.var_guard147 != 0.0) && (locals.var_guard148 != 0.0)) {
        let eq38_e1060: f64 = (locals.var_devsign * locals.var_sigvds);
        let eq38_e1062: f64 = (eq38_e1060 * (nv5 - nv6));
        let eq38_e1064: f64 = (eq38_e1062 * locals.var_ids);
        let eq38_e1064_d_n3: f64 = (eq38_e1062 * locals.var_ids_dn3);
        let eq38_e1064_d_n4: f64 = (eq38_e1062 * locals.var_ids_dn4);
        let eq38_e1064_d_n5: f64 = ((eq38_e1060 * locals.var_ids) + (eq38_e1062 * locals.var_ids_dn5));
        let eq38_e1064_d_n6: f64 = (((-eq38_e1060) * locals.var_ids) + (eq38_e1062 * locals.var_ids_dn6));
        let eq38_e1064_d_n7: f64 = (eq38_e1062 * locals.var_ids_dn7);
        let eq38_e1064_d_n8: f64 = (eq38_e1062 * locals.var_ids_dn8);
        let eq38_e1067: f64 = ((nv0 - nv5) * (nv0 - nv5));
        let eq38_e1067_d_n0: f64 = ((nv0 - nv5) + (nv0 - nv5));
        let eq38_e1067_d_n5: f64 = ((-(nv0 - nv5)) + (-(nv0 - nv5)));
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_rdrain;
        let eq38_e1069: f64 = (eq38_e1067 * __rspice_inv_cse_0);
        let eq38_e1069_d_n0: f64 = (eq38_e1067_d_n0 * __rspice_inv_cse_0);
        let eq38_e1069_d_n3: f64 = (-((eq38_e1067 * locals.var_rdrain_dn3) / (locals.var_rdrain * locals.var_rdrain)));
        let eq38_e1069_d_n4: f64 = (-((eq38_e1067 * locals.var_rdrain_dn4) / (locals.var_rdrain * locals.var_rdrain)));
        let eq38_e1069_d_n5: f64 = (((eq38_e1067_d_n5 * locals.var_rdrain) - (eq38_e1067 * locals.var_rdrain_dn5)) / (locals.var_rdrain * locals.var_rdrain));
        let eq38_e1069_d_n6: f64 = (-((eq38_e1067 * locals.var_rdrain_dn6) / (locals.var_rdrain * locals.var_rdrain)));
        let eq38_e1069_d_n7: f64 = (-((eq38_e1067 * locals.var_rdrain_dn7) / (locals.var_rdrain * locals.var_rdrain)));
        let eq38_e1069_d_n8: f64 = (-((eq38_e1067 * locals.var_rdrain_dn8) / (locals.var_rdrain * locals.var_rdrain)));
        let eq38_e1070: f64 = (eq38_e1064 + eq38_e1069);
        let eq38_e1070_d_n3: f64 = (eq38_e1064_d_n3 + eq38_e1069_d_n3);
        let eq38_e1070_d_n4: f64 = (eq38_e1064_d_n4 + eq38_e1069_d_n4);
        let eq38_e1070_d_n5: f64 = (eq38_e1064_d_n5 + eq38_e1069_d_n5);
        let eq38_e1070_d_n6: f64 = (eq38_e1064_d_n6 + eq38_e1069_d_n6);
        let eq38_e1070_d_n7: f64 = (eq38_e1064_d_n7 + eq38_e1069_d_n7);
        let eq38_e1070_d_n8: f64 = (eq38_e1064_d_n8 + eq38_e1069_d_n8);
        let eq38_e1073: f64 = ((nv2 - nv6) * (nv2 - nv6));
        let eq38_e1073_d_n2: f64 = ((nv2 - nv6) + (nv2 - nv6));
        let eq38_e1073_d_n6: f64 = ((-(nv2 - nv6)) + (-(nv2 - nv6)));
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_rsource;
        let eq38_e1075: f64 = (eq38_e1073 * __rspice_inv_cse_1);
        let eq38_e1075_d_n2: f64 = (eq38_e1073_d_n2 * __rspice_inv_cse_1);
        let eq38_e1075_d_n3: f64 = (-((eq38_e1073 * locals.var_rsource_dn3) / (locals.var_rsource * locals.var_rsource)));
        let eq38_e1075_d_n4: f64 = (-((eq38_e1073 * locals.var_rsource_dn4) / (locals.var_rsource * locals.var_rsource)));
        let eq38_e1075_d_n5: f64 = (-((eq38_e1073 * locals.var_rsource_dn5) / (locals.var_rsource * locals.var_rsource)));
        let eq38_e1075_d_n6: f64 = (((eq38_e1073_d_n6 * locals.var_rsource) - (eq38_e1073 * locals.var_rsource_dn6)) / (locals.var_rsource * locals.var_rsource));
        let eq38_e1075_d_n7: f64 = (-((eq38_e1073 * locals.var_rsource_dn7) / (locals.var_rsource * locals.var_rsource)));
        let eq38_e1075_d_n8: f64 = (-((eq38_e1073 * locals.var_rsource_dn8) / (locals.var_rsource * locals.var_rsource)));
        let eq38_e1076: f64 = (eq38_e1070 + eq38_e1075);
        let eq38_e1076_d_n3: f64 = (eq38_e1070_d_n3 + eq38_e1075_d_n3);
        let eq38_e1076_d_n4: f64 = (eq38_e1070_d_n4 + eq38_e1075_d_n4);
        let eq38_e1076_d_n5: f64 = (eq38_e1070_d_n5 + eq38_e1075_d_n5);
        let eq38_e1076_d_n6: f64 = (eq38_e1070_d_n6 + eq38_e1075_d_n6);
        let eq38_e1076_d_n7: f64 = (eq38_e1070_d_n7 + eq38_e1075_d_n7);
        let eq38_e1076_d_n8: f64 = (eq38_e1070_d_n8 + eq38_e1075_d_n8);
        let eq38_e1077: f64 = (-eq38_e1076);
        (eq38_e1077, (-eq38_e1069_d_n0), (-eq38_e1075_d_n2), (-eq38_e1076_d_n3), (-eq38_e1076_d_n4), (-eq38_e1076_d_n5), (-eq38_e1076_d_n6), (-eq38_e1076_d_n7), (-eq38_e1076_d_n8),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e1079;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(4),
            None,
            multiplicity * (eq38_value),
            [0, 2, 3, 4, 5, 6, 7, 8],
            [multiplicity * (eq38_e1079_d_n0), multiplicity * (eq38_e1079_d_n2), multiplicity * (eq38_e1079_d_n3), multiplicity * (eq38_e1079_d_n4), multiplicity * (eq38_e1079_d_n5), multiplicity * (eq38_e1079_d_n6), multiplicity * (eq38_e1079_d_n7), multiplicity * (eq38_e1079_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq39_e1093, eq39_e1093_d_n3, eq39_e1093_d_n4, eq39_e1093_d_n5, eq39_e1093_d_n6, eq39_e1093_d_n7, eq39_e1093_d_n8,) = {
    if ((locals.var_guard147 != 0.0) && (locals.var_guard148 == 0.0)) {
        let eq39_e1086: f64 = (locals.var_devsign * locals.var_sigvds);
        let eq39_e1088: f64 = (eq39_e1086 * (nv5 - nv6));
        let eq39_e1090: f64 = (eq39_e1088 * locals.var_ids);
        let eq39_e1090_d_n3: f64 = (eq39_e1088 * locals.var_ids_dn3);
        let eq39_e1090_d_n4: f64 = (eq39_e1088 * locals.var_ids_dn4);
        let eq39_e1090_d_n5: f64 = ((eq39_e1086 * locals.var_ids) + (eq39_e1088 * locals.var_ids_dn5));
        let eq39_e1090_d_n6: f64 = (((-eq39_e1086) * locals.var_ids) + (eq39_e1088 * locals.var_ids_dn6));
        let eq39_e1090_d_n7: f64 = (eq39_e1088 * locals.var_ids_dn7);
        let eq39_e1090_d_n8: f64 = (eq39_e1088 * locals.var_ids_dn8);
        let eq39_e1091: f64 = (-eq39_e1090);
        (eq39_e1091, (-eq39_e1090_d_n3), (-eq39_e1090_d_n4), (-eq39_e1090_d_n5), (-eq39_e1090_d_n6), (-eq39_e1090_d_n7), (-eq39_e1090_d_n8),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e1093;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(4),
            None,
            multiplicity * (eq39_value),
            [3, 4, 5, 6, 7, 8],
            [multiplicity * (eq39_e1093_d_n3), multiplicity * (eq39_e1093_d_n4), multiplicity * (eq39_e1093_d_n5), multiplicity * (eq39_e1093_d_n6), multiplicity * (eq39_e1093_d_n7), multiplicity * (eq39_e1093_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq40_e1099, eq40_e1099_d_n4,) = {
    if (locals.var_guard147 != 0.0) {
        let eq40_e1097: f64 = ((nv4 - 0.0) * locals.var_gth);
        (eq40_e1097, locals.var_gth,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e1099;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq40_value),
            4,
            multiplicity * (eq40_e1099_d_n4),
        );
        let (eq41_e1106, eq41_e1106_d_n4,) = {
    if (locals.var_guard147 != 0.0) {
        let eq41_e1103: f64 = ((nv4 - 0.0) * locals.var_cth);
        let eq41_e1104: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq41_e1103);
        (eq41_e1104, (locals.var_cth * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq41_value: f64 = eq41_e1106;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq41_value),
            4,
            multiplicity * (eq41_e1106_d_n4),
        );
    }
}
