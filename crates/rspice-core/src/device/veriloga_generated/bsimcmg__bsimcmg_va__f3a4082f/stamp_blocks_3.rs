#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_48(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign16000_e26095, assign16000_e26095_d_n0, assign16000_e26095_d_n2, assign16000_e26095_d_n3, assign16000_e26095_d_n4, assign16000_e26095_d_n5, assign16000_e26095_d_n6, assign16000_e26095_d_n7, assign16000_e26095_d_n8, assign16000_e26095_d_n9, assign16000_e26095_d_n10, assign16000_e26095_d_n11, assign16000_e26095_d_n13, assign16000_e26095_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard281 != 0.0)) {
        let assign16000_e26045: f64 = (-0.9);
        let assign16000_e26047: f64 = (assign16000_e26045 * locals.var_t1);
        let assign16000_e26051: f64 = (locals.var_utlr_i * locals.var_deltemp);
        let assign16000_e26053: f64 = (-0.9);
        let assign16000_e26055: f64 = (assign16000_e26053 * locals.var_t1);
        let assign16000_e26056: f64 = (assign16000_e26051 - assign16000_e26055);
        let assign16000_e26058: f64 = (assign16000_e26056 - 0.0001);
        let assign16000_e26061: f64 = (locals.var_utlr_i * locals.var_deltemp);
        let assign16000_e26063: f64 = (-0.9);
        let assign16000_e26065: f64 = (assign16000_e26063 * locals.var_t1);
        let assign16000_e26066: f64 = (assign16000_e26061 - assign16000_e26065);
        let assign16000_e26068: f64 = (assign16000_e26066 - 0.0001);
        let assign16000_e26071: f64 = (locals.var_utlr_i * locals.var_deltemp);
        let assign16000_e26073: f64 = (-0.9);
        let assign16000_e26075: f64 = (assign16000_e26073 * locals.var_t1);
        let assign16000_e26076: f64 = (assign16000_e26071 - assign16000_e26075);
        let assign16000_e26078: f64 = (assign16000_e26076 - 0.0001);
        let assign16000_e26079: f64 = (assign16000_e26068 * assign16000_e26078);
        let assign16000_e26082: f64 = (-0.9);
        let assign16000_e26084: f64 = (assign16000_e26082 * locals.var_t1);
        let assign16000_e26085: f64 = (4.0 * assign16000_e26084);
        let assign16000_e26087: f64 = (assign16000_e26085 * 0.0001);
        let assign16000_e26088: f64 = (assign16000_e26079 - assign16000_e26087);
        let assign16000_e26089: f64 = (assign16000_e26088).sqrt();
        let assign16000_e26090: f64 = (assign16000_e26058 + assign16000_e26089);
        let assign16000_e26091: f64 = (0.5 * assign16000_e26090);
        let assign16000_e26092: f64 = (assign16000_e26047 + assign16000_e26091);
        let assign16000_e26093: f64 = (locals.var_t1 + assign16000_e26092);
        (assign16000_e26093, (locals.var_t1_dn0 + ((assign16000_e26045 * locals.var_t1_dn0) + (0.5 * ((-(assign16000_e26053 * locals.var_t1_dn0)) + (((((-(assign16000_e26063 * locals.var_t1_dn0)) * assign16000_e26078) + (assign16000_e26068 * (-(assign16000_e26073 * locals.var_t1_dn0)))) - ((4.0 * (assign16000_e26082 * locals.var_t1_dn0)) * 0.0001)) / (2.0 * assign16000_e26089)))))), (locals.var_t1_dn2 + ((assign16000_e26045 * locals.var_t1_dn2) + (0.5 * ((-(assign16000_e26053 * locals.var_t1_dn2)) + (((((-(assign16000_e26063 * locals.var_t1_dn2)) * assign16000_e26078) + (assign16000_e26068 * (-(assign16000_e26073 * locals.var_t1_dn2)))) - ((4.0 * (assign16000_e26082 * locals.var_t1_dn2)) * 0.0001)) / (2.0 * assign16000_e26089)))))), (locals.var_t1_dn3 + ((assign16000_e26045 * locals.var_t1_dn3) + (0.5 * ((-(assign16000_e26053 * locals.var_t1_dn3)) + (((((-(assign16000_e26063 * locals.var_t1_dn3)) * assign16000_e26078) + (assign16000_e26068 * (-(assign16000_e26073 * locals.var_t1_dn3)))) - ((4.0 * (assign16000_e26082 * locals.var_t1_dn3)) * 0.0001)) / (2.0 * assign16000_e26089)))))), (locals.var_t1_dn4 + ((assign16000_e26045 * locals.var_t1_dn4) + (0.5 * (((locals.var_utlr_i * locals.var_deltemp_dn4) - (assign16000_e26053 * locals.var_t1_dn4)) + ((((((locals.var_utlr_i * locals.var_deltemp_dn4) - (assign16000_e26063 * locals.var_t1_dn4)) * assign16000_e26078) + (assign16000_e26068 * ((locals.var_utlr_i * locals.var_deltemp_dn4) - (assign16000_e26073 * locals.var_t1_dn4)))) - ((4.0 * (assign16000_e26082 * locals.var_t1_dn4)) * 0.0001)) / (2.0 * assign16000_e26089)))))), (locals.var_t1_dn5 + ((assign16000_e26045 * locals.var_t1_dn5) + (0.5 * ((-(assign16000_e26053 * locals.var_t1_dn5)) + (((((-(assign16000_e26063 * locals.var_t1_dn5)) * assign16000_e26078) + (assign16000_e26068 * (-(assign16000_e26073 * locals.var_t1_dn5)))) - ((4.0 * (assign16000_e26082 * locals.var_t1_dn5)) * 0.0001)) / (2.0 * assign16000_e26089)))))), (locals.var_t1_dn6 + ((assign16000_e26045 * locals.var_t1_dn6) + (0.5 * ((-(assign16000_e26053 * locals.var_t1_dn6)) + (((((-(assign16000_e26063 * locals.var_t1_dn6)) * assign16000_e26078) + (assign16000_e26068 * (-(assign16000_e26073 * locals.var_t1_dn6)))) - ((4.0 * (assign16000_e26082 * locals.var_t1_dn6)) * 0.0001)) / (2.0 * assign16000_e26089)))))), (locals.var_t1_dn7 + ((assign16000_e26045 * locals.var_t1_dn7) + (0.5 * ((-(assign16000_e26053 * locals.var_t1_dn7)) + (((((-(assign16000_e26063 * locals.var_t1_dn7)) * assign16000_e26078) + (assign16000_e26068 * (-(assign16000_e26073 * locals.var_t1_dn7)))) - ((4.0 * (assign16000_e26082 * locals.var_t1_dn7)) * 0.0001)) / (2.0 * assign16000_e26089)))))), (locals.var_t1_dn8 + ((assign16000_e26045 * locals.var_t1_dn8) + (0.5 * ((-(assign16000_e26053 * locals.var_t1_dn8)) + (((((-(assign16000_e26063 * locals.var_t1_dn8)) * assign16000_e26078) + (assign16000_e26068 * (-(assign16000_e26073 * locals.var_t1_dn8)))) - ((4.0 * (assign16000_e26082 * locals.var_t1_dn8)) * 0.0001)) / (2.0 * assign16000_e26089)))))), (locals.var_t1_dn9 + ((assign16000_e26045 * locals.var_t1_dn9) + (0.5 * ((-(assign16000_e26053 * locals.var_t1_dn9)) + (((((-(assign16000_e26063 * locals.var_t1_dn9)) * assign16000_e26078) + (assign16000_e26068 * (-(assign16000_e26073 * locals.var_t1_dn9)))) - ((4.0 * (assign16000_e26082 * locals.var_t1_dn9)) * 0.0001)) / (2.0 * assign16000_e26089)))))), (locals.var_t1_dn10 + ((assign16000_e26045 * locals.var_t1_dn10) + (0.5 * ((-(assign16000_e26053 * locals.var_t1_dn10)) + (((((-(assign16000_e26063 * locals.var_t1_dn10)) * assign16000_e26078) + (assign16000_e26068 * (-(assign16000_e26073 * locals.var_t1_dn10)))) - ((4.0 * (assign16000_e26082 * locals.var_t1_dn10)) * 0.0001)) / (2.0 * assign16000_e26089)))))), (locals.var_t1_dn11 + ((assign16000_e26045 * locals.var_t1_dn11) + (0.5 * ((-(assign16000_e26053 * locals.var_t1_dn11)) + (((((-(assign16000_e26063 * locals.var_t1_dn11)) * assign16000_e26078) + (assign16000_e26068 * (-(assign16000_e26073 * locals.var_t1_dn11)))) - ((4.0 * (assign16000_e26082 * locals.var_t1_dn11)) * 0.0001)) / (2.0 * assign16000_e26089)))))), (locals.var_t1_dn13 + ((assign16000_e26045 * locals.var_t1_dn13) + (0.5 * ((-(assign16000_e26053 * locals.var_t1_dn13)) + (((((-(assign16000_e26063 * locals.var_t1_dn13)) * assign16000_e26078) + (assign16000_e26068 * (-(assign16000_e26073 * locals.var_t1_dn13)))) - ((4.0 * (assign16000_e26082 * locals.var_t1_dn13)) * 0.0001)) / (2.0 * assign16000_e26089)))))), (locals.var_t1_dn14 + ((assign16000_e26045 * locals.var_t1_dn14) + (0.5 * ((-(assign16000_e26053 * locals.var_t1_dn14)) + (((((-(assign16000_e26063 * locals.var_t1_dn14)) * assign16000_e26078) + (assign16000_e26068 * (-(assign16000_e26073 * locals.var_t1_dn14)))) - ((4.0 * (assign16000_e26082 * locals.var_t1_dn14)) * 0.0001)) / (2.0 * assign16000_e26089)))))),)
    } else {
        (locals.var_u0r_t, locals.var_u0r_t_dn0, locals.var_u0r_t_dn2, locals.var_u0r_t_dn3, locals.var_u0r_t_dn4, locals.var_u0r_t_dn5, locals.var_u0r_t_dn6, locals.var_u0r_t_dn7, locals.var_u0r_t_dn8, locals.var_u0r_t_dn9, locals.var_u0r_t_dn10, locals.var_u0r_t_dn11, locals.var_u0r_t_dn13, locals.var_u0r_t_dn14,)
    }
};
        locals.var_u0r_t = assign16000_e26095;
        locals.var_u0r_t_dn0 = assign16000_e26095_d_n0;
        locals.var_u0r_t_dn2 = assign16000_e26095_d_n2;
        locals.var_u0r_t_dn3 = assign16000_e26095_d_n3;
        locals.var_u0r_t_dn4 = assign16000_e26095_d_n4;
        locals.var_u0r_t_dn5 = assign16000_e26095_d_n5;
        locals.var_u0r_t_dn6 = assign16000_e26095_d_n6;
        locals.var_u0r_t_dn7 = assign16000_e26095_d_n7;
        locals.var_u0r_t_dn8 = assign16000_e26095_d_n8;
        locals.var_u0r_t_dn9 = assign16000_e26095_d_n9;
        locals.var_u0r_t_dn10 = assign16000_e26095_d_n10;
        locals.var_u0r_t_dn11 = assign16000_e26095_d_n11;
        locals.var_u0r_t_dn13 = assign16000_e26095_d_n13;
        locals.var_u0r_t_dn14 = assign16000_e26095_d_n14;

        let (assign16010_e26105, assign16010_e26105_d_n0, assign16010_e26105_d_n2, assign16010_e26105_d_n3, assign16010_e26105_d_n4, assign16010_e26105_d_n5, assign16010_e26105_d_n6, assign16010_e26105_d_n7, assign16010_e26105_d_n8, assign16010_e26105_d_n9, assign16010_e26105_d_n10, assign16010_e26105_d_n11, assign16010_e26105_d_n13, assign16010_e26105_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard281 != 0.0)) {
        (locals.var_u0r_t, locals.var_u0r_t_dn0, locals.var_u0r_t_dn2, locals.var_u0r_t_dn3, locals.var_u0r_t_dn4, locals.var_u0r_t_dn5, locals.var_u0r_t_dn6, locals.var_u0r_t_dn7, locals.var_u0r_t_dn8, locals.var_u0r_t_dn9, locals.var_u0r_t_dn10, locals.var_u0r_t_dn11, locals.var_u0r_t_dn13, locals.var_u0r_t_dn14,)
    } else {
        (locals.var_u0r_v, locals.var_u0r_v_dn0, locals.var_u0r_v_dn2, locals.var_u0r_v_dn3, locals.var_u0r_v_dn4, locals.var_u0r_v_dn5, locals.var_u0r_v_dn6, locals.var_u0r_v_dn7, locals.var_u0r_v_dn8, locals.var_u0r_v_dn9, locals.var_u0r_v_dn10, locals.var_u0r_v_dn11, locals.var_u0r_v_dn13, locals.var_u0r_v_dn14,)
    }
};
        locals.var_u0r_v = assign16010_e26105;
        locals.var_u0r_v_dn0 = assign16010_e26105_d_n0;
        locals.var_u0r_v_dn2 = assign16010_e26105_d_n2;
        locals.var_u0r_v_dn3 = assign16010_e26105_d_n3;
        locals.var_u0r_v_dn4 = assign16010_e26105_d_n4;
        locals.var_u0r_v_dn5 = assign16010_e26105_d_n5;
        locals.var_u0r_v_dn6 = assign16010_e26105_d_n6;
        locals.var_u0r_v_dn7 = assign16010_e26105_d_n7;
        locals.var_u0r_v_dn8 = assign16010_e26105_d_n8;
        locals.var_u0r_v_dn9 = assign16010_e26105_d_n9;
        locals.var_u0r_v_dn10 = assign16010_e26105_d_n10;
        locals.var_u0r_v_dn11 = assign16010_e26105_d_n11;
        locals.var_u0r_v_dn13 = assign16010_e26105_d_n13;
        locals.var_u0r_v_dn14 = assign16010_e26105_d_n14;

        let assign16020_e26108: f64 = if locals.var_tnom > 210.0 { 1.0 } else { 0.0 };
        locals.var_guard282 = assign16020_e26108;

        let (assign16030_e26155, assign16030_e26155_d_n0, assign16030_e26155_d_n2, assign16030_e26155_d_n3, assign16030_e26155_d_n4, assign16030_e26155_d_n5, assign16030_e26155_d_n6, assign16030_e26155_d_n7, assign16030_e26155_d_n8, assign16030_e26155_d_n9, assign16030_e26155_d_n10, assign16030_e26155_d_n11, assign16030_e26155_d_n13, assign16030_e26155_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard282 != 0.0)) {
        let assign16030_e26122: f64 = (210.0 - locals.var_tnom);
        let assign16030_e26123: f64 = (locals.var_ua1_i * assign16030_e26122);
        let assign16030_e26124: f64 = (locals.var_ua_i + assign16030_e26123);
        let assign16030_e26125: f64 = (locals.var_ua1_i / assign16030_e26124);
        let assign16030_e26129: f64 = (210.0 / locals.var_tnom);
        let (assign16030_e26146,) = {
            if (!(assign16030_e26129 > 1e-38)) {
                let assign16030_e26134: f64 = (-87.498233534);
                (assign16030_e26134,)
            } else {
                let assign16030_e26137: f64 = (210.0 / locals.var_tnom);
                let (assign16030_e26145,) = {
                    if (assign16030_e26137 > 1e-38) {
                        let assign16030_e26142: f64 = (210.0 / locals.var_tnom);
                        let assign16030_e26143: f64 = (assign16030_e26142).ln();
                        (assign16030_e26143,)
                    } else {
                        (0.0,)
                    }
                };
                (assign16030_e26145,)
            }
        };
        let assign16030_e26148: f64 = (assign16030_e26146 + 1.0);
        let assign16030_e26149: f64 = (locals.var_ua2_i * assign16030_e26148);
        let assign16030_e26151: f64 = (assign16030_e26149 / locals.var_tnom);
        let assign16030_e26152: f64 = (assign16030_e26125 - assign16030_e26151);
        let assign16030_e26153: f64 = (210.0 * assign16030_e26152);
        (assign16030_e26153, (210.0 * (-((locals.var_ua1_i * locals.var_ua_i_dn0) / (assign16030_e26124 * assign16030_e26124)))), (210.0 * (-((locals.var_ua1_i * locals.var_ua_i_dn2) / (assign16030_e26124 * assign16030_e26124)))), (210.0 * (-((locals.var_ua1_i * locals.var_ua_i_dn3) / (assign16030_e26124 * assign16030_e26124)))), (210.0 * (-((locals.var_ua1_i * locals.var_ua_i_dn4) / (assign16030_e26124 * assign16030_e26124)))), (210.0 * (-((locals.var_ua1_i * locals.var_ua_i_dn5) / (assign16030_e26124 * assign16030_e26124)))), (210.0 * (-((locals.var_ua1_i * locals.var_ua_i_dn6) / (assign16030_e26124 * assign16030_e26124)))), (210.0 * (-((locals.var_ua1_i * locals.var_ua_i_dn7) / (assign16030_e26124 * assign16030_e26124)))), (210.0 * (-((locals.var_ua1_i * locals.var_ua_i_dn8) / (assign16030_e26124 * assign16030_e26124)))), (210.0 * (-((locals.var_ua1_i * locals.var_ua_i_dn9) / (assign16030_e26124 * assign16030_e26124)))), (210.0 * (-((locals.var_ua1_i * locals.var_ua_i_dn10) / (assign16030_e26124 * assign16030_e26124)))), (210.0 * (-((locals.var_ua1_i * locals.var_ua_i_dn11) / (assign16030_e26124 * assign16030_e26124)))), (210.0 * (-((locals.var_ua1_i * locals.var_ua_i_dn13) / (assign16030_e26124 * assign16030_e26124)))), (210.0 * (-((locals.var_ua1_i * locals.var_ua_i_dn14) / (assign16030_e26124 * assign16030_e26124)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign16030_e26155;
        locals.var_t2_dn0 = assign16030_e26155_d_n0;
        locals.var_t2_dn2 = assign16030_e26155_d_n2;
        locals.var_t2_dn3 = assign16030_e26155_d_n3;
        locals.var_t2_dn4 = assign16030_e26155_d_n4;
        locals.var_t2_dn5 = assign16030_e26155_d_n5;
        locals.var_t2_dn6 = assign16030_e26155_d_n6;
        locals.var_t2_dn7 = assign16030_e26155_d_n7;
        locals.var_t2_dn8 = assign16030_e26155_d_n8;
        locals.var_t2_dn9 = assign16030_e26155_d_n9;
        locals.var_t2_dn10 = assign16030_e26155_d_n10;
        locals.var_t2_dn11 = assign16030_e26155_d_n11;
        locals.var_t2_dn13 = assign16030_e26155_d_n13;
        locals.var_t2_dn14 = assign16030_e26155_d_n14;

        let (assign16040_e26183, assign16040_e26183_d_n0, assign16040_e26183_d_n2, assign16040_e26183_d_n3, assign16040_e26183_d_n4, assign16040_e26183_d_n5, assign16040_e26183_d_n6, assign16040_e26183_d_n7, assign16040_e26183_d_n8, assign16040_e26183_d_n9, assign16040_e26183_d_n10, assign16040_e26183_d_n11, assign16040_e26183_d_n13, assign16040_e26183_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard282 != 0.0)) {
        let assign16040_e26167: f64 = (210.0 - locals.var_tnom);
        let assign16040_e26168: f64 = (locals.var_ua1_i * assign16040_e26167);
        let assign16040_e26169: f64 = (locals.var_ua_i + assign16040_e26168);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_tnom;
        let assign16040_e26172: f64 = (210.0 * __rspice_inv_cse_0);
        let assign16040_e26177: f64 = (210.0 * __rspice_inv_cse_0);
        let assign16040_e26178: f64 = (locals.var_ua2_i * assign16040_e26177);
        let assign16040_e26179: f64 = (locals.var_t2 + assign16040_e26178);
        let assign16040_e26180: f64 = (assign16040_e26172).powf(assign16040_e26179);
        let assign16040_e26181: f64 = (assign16040_e26169 / assign16040_e26180);
        (assign16040_e26181, (((locals.var_ua_i_dn0 * assign16040_e26180) - (assign16040_e26169 * if locals.var_t2_dn0 == 0.0 && ((assign16040_e26179) as f64).is_finite() && ((assign16040_e26179) as f64).fract() == 0.0 { 0.0 } else { (assign16040_e26180 * (locals.var_t2_dn0 * (assign16040_e26172).ln())) })) / (assign16040_e26180 * assign16040_e26180)), (((locals.var_ua_i_dn2 * assign16040_e26180) - (assign16040_e26169 * if locals.var_t2_dn2 == 0.0 && ((assign16040_e26179) as f64).is_finite() && ((assign16040_e26179) as f64).fract() == 0.0 { 0.0 } else { (assign16040_e26180 * (locals.var_t2_dn2 * (assign16040_e26172).ln())) })) / (assign16040_e26180 * assign16040_e26180)), (((locals.var_ua_i_dn3 * assign16040_e26180) - (assign16040_e26169 * if locals.var_t2_dn3 == 0.0 && ((assign16040_e26179) as f64).is_finite() && ((assign16040_e26179) as f64).fract() == 0.0 { 0.0 } else { (assign16040_e26180 * (locals.var_t2_dn3 * (assign16040_e26172).ln())) })) / (assign16040_e26180 * assign16040_e26180)), (((locals.var_ua_i_dn4 * assign16040_e26180) - (assign16040_e26169 * if locals.var_t2_dn4 == 0.0 && ((assign16040_e26179) as f64).is_finite() && ((assign16040_e26179) as f64).fract() == 0.0 { 0.0 } else { (assign16040_e26180 * (locals.var_t2_dn4 * (assign16040_e26172).ln())) })) / (assign16040_e26180 * assign16040_e26180)), (((locals.var_ua_i_dn5 * assign16040_e26180) - (assign16040_e26169 * if locals.var_t2_dn5 == 0.0 && ((assign16040_e26179) as f64).is_finite() && ((assign16040_e26179) as f64).fract() == 0.0 { 0.0 } else { (assign16040_e26180 * (locals.var_t2_dn5 * (assign16040_e26172).ln())) })) / (assign16040_e26180 * assign16040_e26180)), (((locals.var_ua_i_dn6 * assign16040_e26180) - (assign16040_e26169 * if locals.var_t2_dn6 == 0.0 && ((assign16040_e26179) as f64).is_finite() && ((assign16040_e26179) as f64).fract() == 0.0 { 0.0 } else { (assign16040_e26180 * (locals.var_t2_dn6 * (assign16040_e26172).ln())) })) / (assign16040_e26180 * assign16040_e26180)), (((locals.var_ua_i_dn7 * assign16040_e26180) - (assign16040_e26169 * if locals.var_t2_dn7 == 0.0 && ((assign16040_e26179) as f64).is_finite() && ((assign16040_e26179) as f64).fract() == 0.0 { 0.0 } else { (assign16040_e26180 * (locals.var_t2_dn7 * (assign16040_e26172).ln())) })) / (assign16040_e26180 * assign16040_e26180)), (((locals.var_ua_i_dn8 * assign16040_e26180) - (assign16040_e26169 * if locals.var_t2_dn8 == 0.0 && ((assign16040_e26179) as f64).is_finite() && ((assign16040_e26179) as f64).fract() == 0.0 { 0.0 } else { (assign16040_e26180 * (locals.var_t2_dn8 * (assign16040_e26172).ln())) })) / (assign16040_e26180 * assign16040_e26180)), (((locals.var_ua_i_dn9 * assign16040_e26180) - (assign16040_e26169 * if locals.var_t2_dn9 == 0.0 && ((assign16040_e26179) as f64).is_finite() && ((assign16040_e26179) as f64).fract() == 0.0 { 0.0 } else { (assign16040_e26180 * (locals.var_t2_dn9 * (assign16040_e26172).ln())) })) / (assign16040_e26180 * assign16040_e26180)), (((locals.var_ua_i_dn10 * assign16040_e26180) - (assign16040_e26169 * if locals.var_t2_dn10 == 0.0 && ((assign16040_e26179) as f64).is_finite() && ((assign16040_e26179) as f64).fract() == 0.0 { 0.0 } else { (assign16040_e26180 * (locals.var_t2_dn10 * (assign16040_e26172).ln())) })) / (assign16040_e26180 * assign16040_e26180)), (((locals.var_ua_i_dn11 * assign16040_e26180) - (assign16040_e26169 * if locals.var_t2_dn11 == 0.0 && ((assign16040_e26179) as f64).is_finite() && ((assign16040_e26179) as f64).fract() == 0.0 { 0.0 } else { (assign16040_e26180 * (locals.var_t2_dn11 * (assign16040_e26172).ln())) })) / (assign16040_e26180 * assign16040_e26180)), (((locals.var_ua_i_dn13 * assign16040_e26180) - (assign16040_e26169 * if locals.var_t2_dn13 == 0.0 && ((assign16040_e26179) as f64).is_finite() && ((assign16040_e26179) as f64).fract() == 0.0 { 0.0 } else { (assign16040_e26180 * (locals.var_t2_dn13 * (assign16040_e26172).ln())) })) / (assign16040_e26180 * assign16040_e26180)), (((locals.var_ua_i_dn14 * assign16040_e26180) - (assign16040_e26169 * if locals.var_t2_dn14 == 0.0 && ((assign16040_e26179) as f64).is_finite() && ((assign16040_e26179) as f64).fract() == 0.0 { 0.0 } else { (assign16040_e26180 * (locals.var_t2_dn14 * (assign16040_e26172).ln())) })) / (assign16040_e26180 * assign16040_e26180)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign16040_e26183;
        locals.var_t1_dn0 = assign16040_e26183_d_n0;
        locals.var_t1_dn2 = assign16040_e26183_d_n2;
        locals.var_t1_dn3 = assign16040_e26183_d_n3;
        locals.var_t1_dn4 = assign16040_e26183_d_n4;
        locals.var_t1_dn5 = assign16040_e26183_d_n5;
        locals.var_t1_dn6 = assign16040_e26183_d_n6;
        locals.var_t1_dn7 = assign16040_e26183_d_n7;
        locals.var_t1_dn8 = assign16040_e26183_d_n8;
        locals.var_t1_dn9 = assign16040_e26183_d_n9;
        locals.var_t1_dn10 = assign16040_e26183_d_n10;
        locals.var_t1_dn11 = assign16040_e26183_d_n11;
        locals.var_t1_dn13 = assign16040_e26183_d_n13;
        locals.var_t1_dn14 = assign16040_e26183_d_n14;

        let (assign16050_e26201, assign16050_e26201_d_n0, assign16050_e26201_d_n2, assign16050_e26201_d_n3, assign16050_e26201_d_n4, assign16050_e26201_d_n5, assign16050_e26201_d_n6, assign16050_e26201_d_n7, assign16050_e26201_d_n8, assign16050_e26201_d_n9, assign16050_e26201_d_n10, assign16050_e26201_d_n11, assign16050_e26201_d_n13, assign16050_e26201_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard282 != 0.0)) {
        let assign16050_e26196: f64 = (locals.var_ua2_i * locals.var_tratio);
        let assign16050_e26197: f64 = (locals.var_t2 + assign16050_e26196);
        let assign16050_e26198: f64 = (locals.var_tratio).powf(assign16050_e26197);
        let assign16050_e26199: f64 = (locals.var_t1 * assign16050_e26198);
        (assign16050_e26199, ((locals.var_t1_dn0 * assign16050_e26198) + (locals.var_t1 * if locals.var_t2_dn0 == 0.0 && ((assign16050_e26197) as f64).is_finite() && ((assign16050_e26197) as f64).fract() == 0.0 { 0.0 } else { (assign16050_e26198 * (locals.var_t2_dn0 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn2 * assign16050_e26198) + (locals.var_t1 * if locals.var_t2_dn2 == 0.0 && ((assign16050_e26197) as f64).is_finite() && ((assign16050_e26197) as f64).fract() == 0.0 { 0.0 } else { (assign16050_e26198 * (locals.var_t2_dn2 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn3 * assign16050_e26198) + (locals.var_t1 * if locals.var_t2_dn3 == 0.0 && ((assign16050_e26197) as f64).is_finite() && ((assign16050_e26197) as f64).fract() == 0.0 { 0.0 } else { (assign16050_e26198 * (locals.var_t2_dn3 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn4 * assign16050_e26198) + (locals.var_t1 * if (locals.var_t2_dn4 + (locals.var_ua2_i * locals.var_tratio_dn4)) == 0.0 && ((assign16050_e26197) as f64).is_finite() && ((assign16050_e26197) as f64).fract() == 0.0 { if assign16050_e26197 == 0.0 { 0.0 } else { (assign16050_e26197 * ((locals.var_tratio).powf(assign16050_e26197 - 1.0) * locals.var_tratio_dn4)) } } else { (assign16050_e26198 * (((locals.var_t2_dn4 + (locals.var_ua2_i * locals.var_tratio_dn4)) * (locals.var_tratio).ln()) + (assign16050_e26197 * (locals.var_tratio_dn4 / locals.var_tratio)))) })), ((locals.var_t1_dn5 * assign16050_e26198) + (locals.var_t1 * if locals.var_t2_dn5 == 0.0 && ((assign16050_e26197) as f64).is_finite() && ((assign16050_e26197) as f64).fract() == 0.0 { 0.0 } else { (assign16050_e26198 * (locals.var_t2_dn5 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn6 * assign16050_e26198) + (locals.var_t1 * if locals.var_t2_dn6 == 0.0 && ((assign16050_e26197) as f64).is_finite() && ((assign16050_e26197) as f64).fract() == 0.0 { 0.0 } else { (assign16050_e26198 * (locals.var_t2_dn6 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn7 * assign16050_e26198) + (locals.var_t1 * if locals.var_t2_dn7 == 0.0 && ((assign16050_e26197) as f64).is_finite() && ((assign16050_e26197) as f64).fract() == 0.0 { 0.0 } else { (assign16050_e26198 * (locals.var_t2_dn7 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn8 * assign16050_e26198) + (locals.var_t1 * if locals.var_t2_dn8 == 0.0 && ((assign16050_e26197) as f64).is_finite() && ((assign16050_e26197) as f64).fract() == 0.0 { 0.0 } else { (assign16050_e26198 * (locals.var_t2_dn8 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn9 * assign16050_e26198) + (locals.var_t1 * if locals.var_t2_dn9 == 0.0 && ((assign16050_e26197) as f64).is_finite() && ((assign16050_e26197) as f64).fract() == 0.0 { 0.0 } else { (assign16050_e26198 * (locals.var_t2_dn9 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn10 * assign16050_e26198) + (locals.var_t1 * if locals.var_t2_dn10 == 0.0 && ((assign16050_e26197) as f64).is_finite() && ((assign16050_e26197) as f64).fract() == 0.0 { 0.0 } else { (assign16050_e26198 * (locals.var_t2_dn10 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn11 * assign16050_e26198) + (locals.var_t1 * if locals.var_t2_dn11 == 0.0 && ((assign16050_e26197) as f64).is_finite() && ((assign16050_e26197) as f64).fract() == 0.0 { 0.0 } else { (assign16050_e26198 * (locals.var_t2_dn11 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn13 * assign16050_e26198) + (locals.var_t1 * if locals.var_t2_dn13 == 0.0 && ((assign16050_e26197) as f64).is_finite() && ((assign16050_e26197) as f64).fract() == 0.0 { 0.0 } else { (assign16050_e26198 * (locals.var_t2_dn13 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn14 * assign16050_e26198) + (locals.var_t1 * if locals.var_t2_dn14 == 0.0 && ((assign16050_e26197) as f64).is_finite() && ((assign16050_e26197) as f64).fract() == 0.0 { 0.0 } else { (assign16050_e26198 * (locals.var_t2_dn14 * (locals.var_tratio).ln())) })),)
    } else {
        (locals.var_ua_tl, locals.var_ua_tl_dn0, locals.var_ua_tl_dn2, locals.var_ua_tl_dn3, locals.var_ua_tl_dn4, locals.var_ua_tl_dn5, locals.var_ua_tl_dn6, locals.var_ua_tl_dn7, locals.var_ua_tl_dn8, locals.var_ua_tl_dn9, locals.var_ua_tl_dn10, locals.var_ua_tl_dn11, locals.var_ua_tl_dn13, locals.var_ua_tl_dn14,)
    }
};
        locals.var_ua_tl = assign16050_e26201;
        locals.var_ua_tl_dn0 = assign16050_e26201_d_n0;
        locals.var_ua_tl_dn2 = assign16050_e26201_d_n2;
        locals.var_ua_tl_dn3 = assign16050_e26201_d_n3;
        locals.var_ua_tl_dn4 = assign16050_e26201_d_n4;
        locals.var_ua_tl_dn5 = assign16050_e26201_d_n5;
        locals.var_ua_tl_dn6 = assign16050_e26201_d_n6;
        locals.var_ua_tl_dn7 = assign16050_e26201_d_n7;
        locals.var_ua_tl_dn8 = assign16050_e26201_d_n8;
        locals.var_ua_tl_dn9 = assign16050_e26201_d_n9;
        locals.var_ua_tl_dn10 = assign16050_e26201_d_n10;
        locals.var_ua_tl_dn11 = assign16050_e26201_d_n11;
        locals.var_ua_tl_dn13 = assign16050_e26201_d_n13;
        locals.var_ua_tl_dn14 = assign16050_e26201_d_n14;

        let (assign16060_e26215, assign16060_e26215_d_n0, assign16060_e26215_d_n2, assign16060_e26215_d_n3, assign16060_e26215_d_n4, assign16060_e26215_d_n5, assign16060_e26215_d_n6, assign16060_e26215_d_n7, assign16060_e26215_d_n8, assign16060_e26215_d_n9, assign16060_e26215_d_n10, assign16060_e26215_d_n11, assign16060_e26215_d_n13, assign16060_e26215_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard282 != 0.0)) {
        let assign16060_e26212: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign16060_e26213: f64 = (locals.var_ua_i + assign16060_e26212);
        (assign16060_e26213, locals.var_ua_i_dn0, locals.var_ua_i_dn2, locals.var_ua_i_dn3, (locals.var_ua_i_dn4 + (locals.var_ua1_i * locals.var_deltemp_dn4)), locals.var_ua_i_dn5, locals.var_ua_i_dn6, locals.var_ua_i_dn7, locals.var_ua_i_dn8, locals.var_ua_i_dn9, locals.var_ua_i_dn10, locals.var_ua_i_dn11, locals.var_ua_i_dn13, locals.var_ua_i_dn14,)
    } else {
        (locals.var_ua_th, locals.var_ua_th_dn0, locals.var_ua_th_dn2, locals.var_ua_th_dn3, locals.var_ua_th_dn4, locals.var_ua_th_dn5, locals.var_ua_th_dn6, locals.var_ua_th_dn7, locals.var_ua_th_dn8, locals.var_ua_th_dn9, locals.var_ua_th_dn10, locals.var_ua_th_dn11, locals.var_ua_th_dn13, locals.var_ua_th_dn14,)
    }
};
        locals.var_ua_th = assign16060_e26215;
        locals.var_ua_th_dn0 = assign16060_e26215_d_n0;
        locals.var_ua_th_dn2 = assign16060_e26215_d_n2;
        locals.var_ua_th_dn3 = assign16060_e26215_d_n3;
        locals.var_ua_th_dn4 = assign16060_e26215_d_n4;
        locals.var_ua_th_dn5 = assign16060_e26215_d_n5;
        locals.var_ua_th_dn6 = assign16060_e26215_d_n6;
        locals.var_ua_th_dn7 = assign16060_e26215_d_n7;
        locals.var_ua_th_dn8 = assign16060_e26215_d_n8;
        locals.var_ua_th_dn9 = assign16060_e26215_d_n9;
        locals.var_ua_th_dn10 = assign16060_e26215_d_n10;
        locals.var_ua_th_dn11 = assign16060_e26215_d_n11;
        locals.var_ua_th_dn13 = assign16060_e26215_d_n13;
        locals.var_ua_th_dn14 = assign16060_e26215_d_n14;

        let (assign16070_e26269, assign16070_e26269_d_n0, assign16070_e26269_d_n2, assign16070_e26269_d_n3, assign16070_e26269_d_n4, assign16070_e26269_d_n5, assign16070_e26269_d_n6, assign16070_e26269_d_n7, assign16070_e26269_d_n8, assign16070_e26269_d_n9, assign16070_e26269_d_n10, assign16070_e26269_d_n11, assign16070_e26269_d_n13, assign16070_e26269_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard282 == 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_tnom;
        let assign16070_e26227: f64 = (210.0 * __rspice_inv_cse_1);
        let assign16070_e26232: f64 = (210.0 * __rspice_inv_cse_1);
        let assign16070_e26233: f64 = (locals.var_ua2_i * assign16070_e26232);
        let assign16070_e26234: f64 = (locals.var_ua1_i + assign16070_e26233);
        let assign16070_e26235: f64 = (assign16070_e26227).powf(assign16070_e26234);
        let assign16070_e26236: f64 = (locals.var_ua_i * assign16070_e26235);
        let assign16070_e26239: f64 = (locals.var_ua1_i / 210.0);
        let assign16070_e26243: f64 = (210.0 / locals.var_tnom);
        let (assign16070_e26260,) = {
            if (!(assign16070_e26243 > 1e-38)) {
                let assign16070_e26248: f64 = (-87.498233534);
                (assign16070_e26248,)
            } else {
                let assign16070_e26251: f64 = (210.0 / locals.var_tnom);
                let (assign16070_e26259,) = {
                    if (assign16070_e26251 > 1e-38) {
                        let assign16070_e26256: f64 = (210.0 / locals.var_tnom);
                        let assign16070_e26257: f64 = (assign16070_e26256).ln();
                        (assign16070_e26257,)
                    } else {
                        (0.0,)
                    }
                };
                (assign16070_e26259,)
            }
        };
        let assign16070_e26262: f64 = (assign16070_e26260 + 1.0);
        let assign16070_e26263: f64 = (locals.var_ua2_i * assign16070_e26262);
        let assign16070_e26265: f64 = (assign16070_e26263 / locals.var_tnom);
        let assign16070_e26266: f64 = (assign16070_e26239 + assign16070_e26265);
        let assign16070_e26267: f64 = (assign16070_e26236 * assign16070_e26266);
        (assign16070_e26267, ((locals.var_ua_i_dn0 * assign16070_e26235) * assign16070_e26266), ((locals.var_ua_i_dn2 * assign16070_e26235) * assign16070_e26266), ((locals.var_ua_i_dn3 * assign16070_e26235) * assign16070_e26266), ((locals.var_ua_i_dn4 * assign16070_e26235) * assign16070_e26266), ((locals.var_ua_i_dn5 * assign16070_e26235) * assign16070_e26266), ((locals.var_ua_i_dn6 * assign16070_e26235) * assign16070_e26266), ((locals.var_ua_i_dn7 * assign16070_e26235) * assign16070_e26266), ((locals.var_ua_i_dn8 * assign16070_e26235) * assign16070_e26266), ((locals.var_ua_i_dn9 * assign16070_e26235) * assign16070_e26266), ((locals.var_ua_i_dn10 * assign16070_e26235) * assign16070_e26266), ((locals.var_ua_i_dn11 * assign16070_e26235) * assign16070_e26266), ((locals.var_ua_i_dn13 * assign16070_e26235) * assign16070_e26266), ((locals.var_ua_i_dn14 * assign16070_e26235) * assign16070_e26266),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign16070_e26269;
        locals.var_t2_dn0 = assign16070_e26269_d_n0;
        locals.var_t2_dn2 = assign16070_e26269_d_n2;
        locals.var_t2_dn3 = assign16070_e26269_d_n3;
        locals.var_t2_dn4 = assign16070_e26269_d_n4;
        locals.var_t2_dn5 = assign16070_e26269_d_n5;
        locals.var_t2_dn6 = assign16070_e26269_d_n6;
        locals.var_t2_dn7 = assign16070_e26269_d_n7;
        locals.var_t2_dn8 = assign16070_e26269_d_n8;
        locals.var_t2_dn9 = assign16070_e26269_d_n9;
        locals.var_t2_dn10 = assign16070_e26269_d_n10;
        locals.var_t2_dn11 = assign16070_e26269_d_n11;
        locals.var_t2_dn13 = assign16070_e26269_d_n13;
        locals.var_t2_dn14 = assign16070_e26269_d_n14;

        let (assign16080_e26298, assign16080_e26298_d_n0, assign16080_e26298_d_n2, assign16080_e26298_d_n3, assign16080_e26298_d_n4, assign16080_e26298_d_n5, assign16080_e26298_d_n6, assign16080_e26298_d_n7, assign16080_e26298_d_n8, assign16080_e26298_d_n9, assign16080_e26298_d_n10, assign16080_e26298_d_n11, assign16080_e26298_d_n13, assign16080_e26298_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard282 == 0.0)) {
        let __rspice_inv_cse_2: f64 = 1.0 / locals.var_tnom;
        let assign16080_e26281: f64 = (210.0 * __rspice_inv_cse_2);
        let assign16080_e26286: f64 = (210.0 * __rspice_inv_cse_2);
        let assign16080_e26287: f64 = (locals.var_ua2_i * assign16080_e26286);
        let assign16080_e26288: f64 = (locals.var_ua1_i + assign16080_e26287);
        let assign16080_e26289: f64 = (assign16080_e26281).powf(assign16080_e26288);
        let assign16080_e26290: f64 = (locals.var_ua_i * assign16080_e26289);
        let assign16080_e26294: f64 = (210.0 - locals.var_tnom);
        let assign16080_e26295: f64 = (locals.var_t2 * assign16080_e26294);
        let assign16080_e26296: f64 = (assign16080_e26290 - assign16080_e26295);
        (assign16080_e26296, ((locals.var_ua_i_dn0 * assign16080_e26289) - (locals.var_t2_dn0 * assign16080_e26294)), ((locals.var_ua_i_dn2 * assign16080_e26289) - (locals.var_t2_dn2 * assign16080_e26294)), ((locals.var_ua_i_dn3 * assign16080_e26289) - (locals.var_t2_dn3 * assign16080_e26294)), ((locals.var_ua_i_dn4 * assign16080_e26289) - (locals.var_t2_dn4 * assign16080_e26294)), ((locals.var_ua_i_dn5 * assign16080_e26289) - (locals.var_t2_dn5 * assign16080_e26294)), ((locals.var_ua_i_dn6 * assign16080_e26289) - (locals.var_t2_dn6 * assign16080_e26294)), ((locals.var_ua_i_dn7 * assign16080_e26289) - (locals.var_t2_dn7 * assign16080_e26294)), ((locals.var_ua_i_dn8 * assign16080_e26289) - (locals.var_t2_dn8 * assign16080_e26294)), ((locals.var_ua_i_dn9 * assign16080_e26289) - (locals.var_t2_dn9 * assign16080_e26294)), ((locals.var_ua_i_dn10 * assign16080_e26289) - (locals.var_t2_dn10 * assign16080_e26294)), ((locals.var_ua_i_dn11 * assign16080_e26289) - (locals.var_t2_dn11 * assign16080_e26294)), ((locals.var_ua_i_dn13 * assign16080_e26289) - (locals.var_t2_dn13 * assign16080_e26294)), ((locals.var_ua_i_dn14 * assign16080_e26289) - (locals.var_t2_dn14 * assign16080_e26294)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign16080_e26298;
        locals.var_t1_dn0 = assign16080_e26298_d_n0;
        locals.var_t1_dn2 = assign16080_e26298_d_n2;
        locals.var_t1_dn3 = assign16080_e26298_d_n3;
        locals.var_t1_dn4 = assign16080_e26298_d_n4;
        locals.var_t1_dn5 = assign16080_e26298_d_n5;
        locals.var_t1_dn6 = assign16080_e26298_d_n6;
        locals.var_t1_dn7 = assign16080_e26298_d_n7;
        locals.var_t1_dn8 = assign16080_e26298_d_n8;
        locals.var_t1_dn9 = assign16080_e26298_d_n9;
        locals.var_t1_dn10 = assign16080_e26298_d_n10;
        locals.var_t1_dn11 = assign16080_e26298_d_n11;
        locals.var_t1_dn13 = assign16080_e26298_d_n13;
        locals.var_t1_dn14 = assign16080_e26298_d_n14;

        let (assign16090_e26317, assign16090_e26317_d_n0, assign16090_e26317_d_n2, assign16090_e26317_d_n3, assign16090_e26317_d_n4, assign16090_e26317_d_n5, assign16090_e26317_d_n6, assign16090_e26317_d_n7, assign16090_e26317_d_n8, assign16090_e26317_d_n9, assign16090_e26317_d_n10, assign16090_e26317_d_n11, assign16090_e26317_d_n13, assign16090_e26317_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard282 == 0.0)) {
        let assign16090_e26312: f64 = (locals.var_ua2_i * locals.var_tratio);
        let assign16090_e26313: f64 = (locals.var_ua1_i + assign16090_e26312);
        let assign16090_e26314: f64 = (locals.var_tratio).powf(assign16090_e26313);
        let assign16090_e26315: f64 = (locals.var_ua_i * assign16090_e26314);
        (assign16090_e26315, (locals.var_ua_i_dn0 * assign16090_e26314), (locals.var_ua_i_dn2 * assign16090_e26314), (locals.var_ua_i_dn3 * assign16090_e26314), ((locals.var_ua_i_dn4 * assign16090_e26314) + (locals.var_ua_i * if (locals.var_ua2_i * locals.var_tratio_dn4) == 0.0 && ((assign16090_e26313) as f64).is_finite() && ((assign16090_e26313) as f64).fract() == 0.0 { if assign16090_e26313 == 0.0 { 0.0 } else { (assign16090_e26313 * ((locals.var_tratio).powf(assign16090_e26313 - 1.0) * locals.var_tratio_dn4)) } } else { (assign16090_e26314 * (((locals.var_ua2_i * locals.var_tratio_dn4) * (locals.var_tratio).ln()) + (assign16090_e26313 * (locals.var_tratio_dn4 / locals.var_tratio)))) })), (locals.var_ua_i_dn5 * assign16090_e26314), (locals.var_ua_i_dn6 * assign16090_e26314), (locals.var_ua_i_dn7 * assign16090_e26314), (locals.var_ua_i_dn8 * assign16090_e26314), (locals.var_ua_i_dn9 * assign16090_e26314), (locals.var_ua_i_dn10 * assign16090_e26314), (locals.var_ua_i_dn11 * assign16090_e26314), (locals.var_ua_i_dn13 * assign16090_e26314), (locals.var_ua_i_dn14 * assign16090_e26314),)
    } else {
        (locals.var_ua_tl, locals.var_ua_tl_dn0, locals.var_ua_tl_dn2, locals.var_ua_tl_dn3, locals.var_ua_tl_dn4, locals.var_ua_tl_dn5, locals.var_ua_tl_dn6, locals.var_ua_tl_dn7, locals.var_ua_tl_dn8, locals.var_ua_tl_dn9, locals.var_ua_tl_dn10, locals.var_ua_tl_dn11, locals.var_ua_tl_dn13, locals.var_ua_tl_dn14,)
    }
};
        locals.var_ua_tl = assign16090_e26317;
        locals.var_ua_tl_dn0 = assign16090_e26317_d_n0;
        locals.var_ua_tl_dn2 = assign16090_e26317_d_n2;
        locals.var_ua_tl_dn3 = assign16090_e26317_d_n3;
        locals.var_ua_tl_dn4 = assign16090_e26317_d_n4;
        locals.var_ua_tl_dn5 = assign16090_e26317_d_n5;
        locals.var_ua_tl_dn6 = assign16090_e26317_d_n6;
        locals.var_ua_tl_dn7 = assign16090_e26317_d_n7;
        locals.var_ua_tl_dn8 = assign16090_e26317_d_n8;
        locals.var_ua_tl_dn9 = assign16090_e26317_d_n9;
        locals.var_ua_tl_dn10 = assign16090_e26317_d_n10;
        locals.var_ua_tl_dn11 = assign16090_e26317_d_n11;
        locals.var_ua_tl_dn13 = assign16090_e26317_d_n13;
        locals.var_ua_tl_dn14 = assign16090_e26317_d_n14;

        let (assign16100_e26332, assign16100_e26332_d_n0, assign16100_e26332_d_n2, assign16100_e26332_d_n3, assign16100_e26332_d_n4, assign16100_e26332_d_n5, assign16100_e26332_d_n6, assign16100_e26332_d_n7, assign16100_e26332_d_n8, assign16100_e26332_d_n9, assign16100_e26332_d_n10, assign16100_e26332_d_n11, assign16100_e26332_d_n13, assign16100_e26332_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard282 == 0.0)) {
        let assign16100_e26329: f64 = (locals.var_t2 * locals.var_deltemp);
        let assign16100_e26330: f64 = (locals.var_t1 + assign16100_e26329);
        (assign16100_e26330, (locals.var_t1_dn0 + (locals.var_t2_dn0 * locals.var_deltemp)), (locals.var_t1_dn2 + (locals.var_t2_dn2 * locals.var_deltemp)), (locals.var_t1_dn3 + (locals.var_t2_dn3 * locals.var_deltemp)), (locals.var_t1_dn4 + ((locals.var_t2_dn4 * locals.var_deltemp) + (locals.var_t2 * locals.var_deltemp_dn4))), (locals.var_t1_dn5 + (locals.var_t2_dn5 * locals.var_deltemp)), (locals.var_t1_dn6 + (locals.var_t2_dn6 * locals.var_deltemp)), (locals.var_t1_dn7 + (locals.var_t2_dn7 * locals.var_deltemp)), (locals.var_t1_dn8 + (locals.var_t2_dn8 * locals.var_deltemp)), (locals.var_t1_dn9 + (locals.var_t2_dn9 * locals.var_deltemp)), (locals.var_t1_dn10 + (locals.var_t2_dn10 * locals.var_deltemp)), (locals.var_t1_dn11 + (locals.var_t2_dn11 * locals.var_deltemp)), (locals.var_t1_dn13 + (locals.var_t2_dn13 * locals.var_deltemp)), (locals.var_t1_dn14 + (locals.var_t2_dn14 * locals.var_deltemp)),)
    } else {
        (locals.var_ua_th, locals.var_ua_th_dn0, locals.var_ua_th_dn2, locals.var_ua_th_dn3, locals.var_ua_th_dn4, locals.var_ua_th_dn5, locals.var_ua_th_dn6, locals.var_ua_th_dn7, locals.var_ua_th_dn8, locals.var_ua_th_dn9, locals.var_ua_th_dn10, locals.var_ua_th_dn11, locals.var_ua_th_dn13, locals.var_ua_th_dn14,)
    }
};
        locals.var_ua_th = assign16100_e26332;
        locals.var_ua_th_dn0 = assign16100_e26332_d_n0;
        locals.var_ua_th_dn2 = assign16100_e26332_d_n2;
        locals.var_ua_th_dn3 = assign16100_e26332_d_n3;
        locals.var_ua_th_dn4 = assign16100_e26332_d_n4;
        locals.var_ua_th_dn5 = assign16100_e26332_d_n5;
        locals.var_ua_th_dn6 = assign16100_e26332_d_n6;
        locals.var_ua_th_dn7 = assign16100_e26332_d_n7;
        locals.var_ua_th_dn8 = assign16100_e26332_d_n8;
        locals.var_ua_th_dn9 = assign16100_e26332_d_n9;
        locals.var_ua_th_dn10 = assign16100_e26332_d_n10;
        locals.var_ua_th_dn11 = assign16100_e26332_d_n11;
        locals.var_ua_th_dn13 = assign16100_e26332_d_n13;
        locals.var_ua_th_dn14 = assign16100_e26332_d_n14;

        let (assign16110_e26346, assign16110_e26346_d_n0, assign16110_e26346_d_n2, assign16110_e26346_d_n3, assign16110_e26346_d_n4, assign16110_e26346_d_n5, assign16110_e26346_d_n6, assign16110_e26346_d_n7, assign16110_e26346_d_n8, assign16110_e26346_d_n9, assign16110_e26346_d_n10, assign16110_e26346_d_n11, assign16110_e26346_d_n13, assign16110_e26346_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) {
        let assign16110_e26340: f64 = (locals.var_wl * locals.var_ua_tl);
        let assign16110_e26343: f64 = (locals.var_wh * locals.var_ua_th);
        let assign16110_e26344: f64 = (assign16110_e26340 + assign16110_e26343);
        (assign16110_e26344, ((locals.var_wl * locals.var_ua_tl_dn0) + (locals.var_wh * locals.var_ua_th_dn0)), ((locals.var_wl * locals.var_ua_tl_dn2) + (locals.var_wh * locals.var_ua_th_dn2)), ((locals.var_wl * locals.var_ua_tl_dn3) + (locals.var_wh * locals.var_ua_th_dn3)), (((locals.var_wl_dn4 * locals.var_ua_tl) + (locals.var_wl * locals.var_ua_tl_dn4)) + ((locals.var_wh_dn4 * locals.var_ua_th) + (locals.var_wh * locals.var_ua_th_dn4))), ((locals.var_wl * locals.var_ua_tl_dn5) + (locals.var_wh * locals.var_ua_th_dn5)), ((locals.var_wl * locals.var_ua_tl_dn6) + (locals.var_wh * locals.var_ua_th_dn6)), ((locals.var_wl * locals.var_ua_tl_dn7) + (locals.var_wh * locals.var_ua_th_dn7)), ((locals.var_wl * locals.var_ua_tl_dn8) + (locals.var_wh * locals.var_ua_th_dn8)), ((locals.var_wl * locals.var_ua_tl_dn9) + (locals.var_wh * locals.var_ua_th_dn9)), ((locals.var_wl * locals.var_ua_tl_dn10) + (locals.var_wh * locals.var_ua_th_dn10)), ((locals.var_wl * locals.var_ua_tl_dn11) + (locals.var_wh * locals.var_ua_th_dn11)), ((locals.var_wl * locals.var_ua_tl_dn13) + (locals.var_wh * locals.var_ua_th_dn13)), ((locals.var_wl * locals.var_ua_tl_dn14) + (locals.var_wh * locals.var_ua_th_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign16110_e26346;
        locals.var_t0_dn0 = assign16110_e26346_d_n0;
        locals.var_t0_dn2 = assign16110_e26346_d_n2;
        locals.var_t0_dn3 = assign16110_e26346_d_n3;
        locals.var_t0_dn4 = assign16110_e26346_d_n4;
        locals.var_t0_dn5 = assign16110_e26346_d_n5;
        locals.var_t0_dn6 = assign16110_e26346_d_n6;
        locals.var_t0_dn7 = assign16110_e26346_d_n7;
        locals.var_t0_dn8 = assign16110_e26346_d_n8;
        locals.var_t0_dn9 = assign16110_e26346_d_n9;
        locals.var_t0_dn10 = assign16110_e26346_d_n10;
        locals.var_t0_dn11 = assign16110_e26346_d_n11;
        locals.var_t0_dn13 = assign16110_e26346_d_n13;
        locals.var_t0_dn14 = assign16110_e26346_d_n14;

        let (assign16120_e26389, assign16120_e26389_d_n0, assign16120_e26389_d_n2, assign16120_e26389_d_n3, assign16120_e26389_d_n4, assign16120_e26389_d_n5, assign16120_e26389_d_n6, assign16120_e26389_d_n7, assign16120_e26389_d_n8, assign16120_e26389_d_n9, assign16120_e26389_d_n10, assign16120_e26389_d_n11, assign16120_e26389_d_n13, assign16120_e26389_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) {
        let assign16120_e26354: f64 = (-10000.0);
        let assign16120_e26356: f64 = (assign16120_e26354 * 1e-6);
        let (assign16120_e26387, assign16120_e26387_d_n0, assign16120_e26387_d_n2, assign16120_e26387_d_n3, assign16120_e26387_d_n4, assign16120_e26387_d_n5, assign16120_e26387_d_n6, assign16120_e26387_d_n7, assign16120_e26387_d_n8, assign16120_e26387_d_n9, assign16120_e26387_d_n10, assign16120_e26387_d_n11, assign16120_e26387_d_n13, assign16120_e26387_d_n14,) = {
            if (!(locals.var_t0 < assign16120_e26356)) {
                let assign16120_e26363: f64 = (locals.var_t0 * locals.var_t0);
                let assign16120_e26366: f64 = (4.0 * 1e-6);
                let assign16120_e26368: f64 = (assign16120_e26366 * 1e-6);
                let assign16120_e26369: f64 = (assign16120_e26363 + assign16120_e26368);
                let assign16120_e26370: f64 = (assign16120_e26369).sqrt();
                let assign16120_e26371: f64 = (locals.var_t0 + assign16120_e26370);
                let assign16120_e26372: f64 = (0.5 * assign16120_e26371);
                (assign16120_e26372, (0.5 * (locals.var_t0_dn0 + (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign16120_e26370)))), (0.5 * (locals.var_t0_dn2 + (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign16120_e26370)))), (0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign16120_e26370)))), (0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign16120_e26370)))), (0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign16120_e26370)))), (0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign16120_e26370)))), (0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign16120_e26370)))), (0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign16120_e26370)))), (0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign16120_e26370)))), (0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign16120_e26370)))), (0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign16120_e26370)))), (0.5 * (locals.var_t0_dn13 + (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign16120_e26370)))), (0.5 * (locals.var_t0_dn14 + (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign16120_e26370)))),)
            } else {
                let assign16120_e26375: f64 = (-10000.0);
                let assign16120_e26377: f64 = (assign16120_e26375 * 1e-6);
                let (assign16120_e26386, assign16120_e26386_d_n0, assign16120_e26386_d_n2, assign16120_e26386_d_n3, assign16120_e26386_d_n4, assign16120_e26386_d_n5, assign16120_e26386_d_n6, assign16120_e26386_d_n7, assign16120_e26386_d_n8, assign16120_e26386_d_n9, assign16120_e26386_d_n10, assign16120_e26386_d_n11, assign16120_e26386_d_n13, assign16120_e26386_d_n14,) = {
                    if (locals.var_t0 < assign16120_e26377) {
                        let assign16120_e26380: f64 = (-1e-6);
                        let assign16120_e26382: f64 = (assign16120_e26380 * 1e-6);
                        let assign16120_e26384: f64 = (assign16120_e26382 / locals.var_t0);
                        (assign16120_e26384, (-((assign16120_e26382 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((assign16120_e26382 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((assign16120_e26382 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (-((assign16120_e26382 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((assign16120_e26382 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((assign16120_e26382 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((assign16120_e26382 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((assign16120_e26382 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((assign16120_e26382 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), (-((assign16120_e26382 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((assign16120_e26382 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((assign16120_e26382 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))), (-((assign16120_e26382 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign16120_e26386, assign16120_e26386_d_n0, assign16120_e26386_d_n2, assign16120_e26386_d_n3, assign16120_e26386_d_n4, assign16120_e26386_d_n5, assign16120_e26386_d_n6, assign16120_e26386_d_n7, assign16120_e26386_d_n8, assign16120_e26386_d_n9, assign16120_e26386_d_n10, assign16120_e26386_d_n11, assign16120_e26386_d_n13, assign16120_e26386_d_n14,)
            }
        };
        (assign16120_e26387, assign16120_e26387_d_n0, assign16120_e26387_d_n2, assign16120_e26387_d_n3, assign16120_e26387_d_n4, assign16120_e26387_d_n5, assign16120_e26387_d_n6, assign16120_e26387_d_n7, assign16120_e26387_d_n8, assign16120_e26387_d_n9, assign16120_e26387_d_n10, assign16120_e26387_d_n11, assign16120_e26387_d_n13, assign16120_e26387_d_n14,)
    } else {
        (locals.var_ua_t, locals.var_ua_t_dn0, locals.var_ua_t_dn2, locals.var_ua_t_dn3, locals.var_ua_t_dn4, locals.var_ua_t_dn5, locals.var_ua_t_dn6, locals.var_ua_t_dn7, locals.var_ua_t_dn8, locals.var_ua_t_dn9, locals.var_ua_t_dn10, locals.var_ua_t_dn11, locals.var_ua_t_dn13, locals.var_ua_t_dn14,)
    }
};
        locals.var_ua_t = assign16120_e26389;
        locals.var_ua_t_dn0 = assign16120_e26389_d_n0;
        locals.var_ua_t_dn2 = assign16120_e26389_d_n2;
        locals.var_ua_t_dn3 = assign16120_e26389_d_n3;
        locals.var_ua_t_dn4 = assign16120_e26389_d_n4;
        locals.var_ua_t_dn5 = assign16120_e26389_d_n5;
        locals.var_ua_t_dn6 = assign16120_e26389_d_n6;
        locals.var_ua_t_dn7 = assign16120_e26389_d_n7;
        locals.var_ua_t_dn8 = assign16120_e26389_d_n8;
        locals.var_ua_t_dn9 = assign16120_e26389_d_n9;
        locals.var_ua_t_dn10 = assign16120_e26389_d_n10;
        locals.var_ua_t_dn11 = assign16120_e26389_d_n11;
        locals.var_ua_t_dn13 = assign16120_e26389_d_n13;
        locals.var_ua_t_dn14 = assign16120_e26389_d_n14;

        let assign16130_e26392: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard283 = assign16130_e26392;

        let assign16140_e26395: f64 = if locals.var_tnom > 210.0 { 1.0 } else { 0.0 };
        locals.var_guard284 = assign16140_e26395;

        let (assign16150_e26444, assign16150_e26444_d_n0, assign16150_e26444_d_n2, assign16150_e26444_d_n3, assign16150_e26444_d_n4, assign16150_e26444_d_n5, assign16150_e26444_d_n6, assign16150_e26444_d_n7, assign16150_e26444_d_n8, assign16150_e26444_d_n9, assign16150_e26444_d_n10, assign16150_e26444_d_n11, assign16150_e26444_d_n13, assign16150_e26444_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard283 != 0.0)) && (locals.var_guard284 != 0.0)) {
        let assign16150_e26411: f64 = (210.0 - locals.var_tnom);
        let assign16150_e26412: f64 = (locals.var_ua1r_i * assign16150_e26411);
        let assign16150_e26413: f64 = (locals.var_uar_i + assign16150_e26412);
        let assign16150_e26414: f64 = (locals.var_ua1r_i / assign16150_e26413);
        let assign16150_e26418: f64 = (210.0 / locals.var_tnom);
        let (assign16150_e26435,) = {
            if (!(assign16150_e26418 > 1e-38)) {
                let assign16150_e26423: f64 = (-87.498233534);
                (assign16150_e26423,)
            } else {
                let assign16150_e26426: f64 = (210.0 / locals.var_tnom);
                let (assign16150_e26434,) = {
                    if (assign16150_e26426 > 1e-38) {
                        let assign16150_e26431: f64 = (210.0 / locals.var_tnom);
                        let assign16150_e26432: f64 = (assign16150_e26431).ln();
                        (assign16150_e26432,)
                    } else {
                        (0.0,)
                    }
                };
                (assign16150_e26434,)
            }
        };
        let assign16150_e26437: f64 = (assign16150_e26435 + 1.0);
        let assign16150_e26438: f64 = (locals.var_ua2_i * assign16150_e26437);
        let assign16150_e26440: f64 = (assign16150_e26438 / locals.var_tnom);
        let assign16150_e26441: f64 = (assign16150_e26414 - assign16150_e26440);
        let assign16150_e26442: f64 = (210.0 * assign16150_e26441);
        (assign16150_e26442, (210.0 * (-((locals.var_ua1r_i * locals.var_uar_i_dn0) / (assign16150_e26413 * assign16150_e26413)))), (210.0 * (-((locals.var_ua1r_i * locals.var_uar_i_dn2) / (assign16150_e26413 * assign16150_e26413)))), (210.0 * (-((locals.var_ua1r_i * locals.var_uar_i_dn3) / (assign16150_e26413 * assign16150_e26413)))), (210.0 * (-((locals.var_ua1r_i * locals.var_uar_i_dn4) / (assign16150_e26413 * assign16150_e26413)))), (210.0 * (-((locals.var_ua1r_i * locals.var_uar_i_dn5) / (assign16150_e26413 * assign16150_e26413)))), (210.0 * (-((locals.var_ua1r_i * locals.var_uar_i_dn6) / (assign16150_e26413 * assign16150_e26413)))), (210.0 * (-((locals.var_ua1r_i * locals.var_uar_i_dn7) / (assign16150_e26413 * assign16150_e26413)))), (210.0 * (-((locals.var_ua1r_i * locals.var_uar_i_dn8) / (assign16150_e26413 * assign16150_e26413)))), (210.0 * (-((locals.var_ua1r_i * locals.var_uar_i_dn9) / (assign16150_e26413 * assign16150_e26413)))), (210.0 * (-((locals.var_ua1r_i * locals.var_uar_i_dn10) / (assign16150_e26413 * assign16150_e26413)))), (210.0 * (-((locals.var_ua1r_i * locals.var_uar_i_dn11) / (assign16150_e26413 * assign16150_e26413)))), (210.0 * (-((locals.var_ua1r_i * locals.var_uar_i_dn13) / (assign16150_e26413 * assign16150_e26413)))), (210.0 * (-((locals.var_ua1r_i * locals.var_uar_i_dn14) / (assign16150_e26413 * assign16150_e26413)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign16150_e26444;
        locals.var_t2_dn0 = assign16150_e26444_d_n0;
        locals.var_t2_dn2 = assign16150_e26444_d_n2;
        locals.var_t2_dn3 = assign16150_e26444_d_n3;
        locals.var_t2_dn4 = assign16150_e26444_d_n4;
        locals.var_t2_dn5 = assign16150_e26444_d_n5;
        locals.var_t2_dn6 = assign16150_e26444_d_n6;
        locals.var_t2_dn7 = assign16150_e26444_d_n7;
        locals.var_t2_dn8 = assign16150_e26444_d_n8;
        locals.var_t2_dn9 = assign16150_e26444_d_n9;
        locals.var_t2_dn10 = assign16150_e26444_d_n10;
        locals.var_t2_dn11 = assign16150_e26444_d_n11;
        locals.var_t2_dn13 = assign16150_e26444_d_n13;
        locals.var_t2_dn14 = assign16150_e26444_d_n14;

        let (assign16160_e26474, assign16160_e26474_d_n0, assign16160_e26474_d_n2, assign16160_e26474_d_n3, assign16160_e26474_d_n4, assign16160_e26474_d_n5, assign16160_e26474_d_n6, assign16160_e26474_d_n7, assign16160_e26474_d_n8, assign16160_e26474_d_n9, assign16160_e26474_d_n10, assign16160_e26474_d_n11, assign16160_e26474_d_n13, assign16160_e26474_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard283 != 0.0)) && (locals.var_guard284 != 0.0)) {
        let assign16160_e26458: f64 = (210.0 - locals.var_tnom);
        let assign16160_e26459: f64 = (locals.var_ua1r_i * assign16160_e26458);
        let assign16160_e26460: f64 = (locals.var_uar_i + assign16160_e26459);
        let __rspice_inv_cse_3: f64 = 1.0 / locals.var_tnom;
        let assign16160_e26463: f64 = (210.0 * __rspice_inv_cse_3);
        let assign16160_e26468: f64 = (210.0 * __rspice_inv_cse_3);
        let assign16160_e26469: f64 = (locals.var_ua2_i * assign16160_e26468);
        let assign16160_e26470: f64 = (locals.var_t2 + assign16160_e26469);
        let assign16160_e26471: f64 = (assign16160_e26463).powf(assign16160_e26470);
        let assign16160_e26472: f64 = (assign16160_e26460 / assign16160_e26471);
        (assign16160_e26472, (((locals.var_uar_i_dn0 * assign16160_e26471) - (assign16160_e26460 * if locals.var_t2_dn0 == 0.0 && ((assign16160_e26470) as f64).is_finite() && ((assign16160_e26470) as f64).fract() == 0.0 { 0.0 } else { (assign16160_e26471 * (locals.var_t2_dn0 * (assign16160_e26463).ln())) })) / (assign16160_e26471 * assign16160_e26471)), (((locals.var_uar_i_dn2 * assign16160_e26471) - (assign16160_e26460 * if locals.var_t2_dn2 == 0.0 && ((assign16160_e26470) as f64).is_finite() && ((assign16160_e26470) as f64).fract() == 0.0 { 0.0 } else { (assign16160_e26471 * (locals.var_t2_dn2 * (assign16160_e26463).ln())) })) / (assign16160_e26471 * assign16160_e26471)), (((locals.var_uar_i_dn3 * assign16160_e26471) - (assign16160_e26460 * if locals.var_t2_dn3 == 0.0 && ((assign16160_e26470) as f64).is_finite() && ((assign16160_e26470) as f64).fract() == 0.0 { 0.0 } else { (assign16160_e26471 * (locals.var_t2_dn3 * (assign16160_e26463).ln())) })) / (assign16160_e26471 * assign16160_e26471)), (((locals.var_uar_i_dn4 * assign16160_e26471) - (assign16160_e26460 * if locals.var_t2_dn4 == 0.0 && ((assign16160_e26470) as f64).is_finite() && ((assign16160_e26470) as f64).fract() == 0.0 { 0.0 } else { (assign16160_e26471 * (locals.var_t2_dn4 * (assign16160_e26463).ln())) })) / (assign16160_e26471 * assign16160_e26471)), (((locals.var_uar_i_dn5 * assign16160_e26471) - (assign16160_e26460 * if locals.var_t2_dn5 == 0.0 && ((assign16160_e26470) as f64).is_finite() && ((assign16160_e26470) as f64).fract() == 0.0 { 0.0 } else { (assign16160_e26471 * (locals.var_t2_dn5 * (assign16160_e26463).ln())) })) / (assign16160_e26471 * assign16160_e26471)), (((locals.var_uar_i_dn6 * assign16160_e26471) - (assign16160_e26460 * if locals.var_t2_dn6 == 0.0 && ((assign16160_e26470) as f64).is_finite() && ((assign16160_e26470) as f64).fract() == 0.0 { 0.0 } else { (assign16160_e26471 * (locals.var_t2_dn6 * (assign16160_e26463).ln())) })) / (assign16160_e26471 * assign16160_e26471)), (((locals.var_uar_i_dn7 * assign16160_e26471) - (assign16160_e26460 * if locals.var_t2_dn7 == 0.0 && ((assign16160_e26470) as f64).is_finite() && ((assign16160_e26470) as f64).fract() == 0.0 { 0.0 } else { (assign16160_e26471 * (locals.var_t2_dn7 * (assign16160_e26463).ln())) })) / (assign16160_e26471 * assign16160_e26471)), (((locals.var_uar_i_dn8 * assign16160_e26471) - (assign16160_e26460 * if locals.var_t2_dn8 == 0.0 && ((assign16160_e26470) as f64).is_finite() && ((assign16160_e26470) as f64).fract() == 0.0 { 0.0 } else { (assign16160_e26471 * (locals.var_t2_dn8 * (assign16160_e26463).ln())) })) / (assign16160_e26471 * assign16160_e26471)), (((locals.var_uar_i_dn9 * assign16160_e26471) - (assign16160_e26460 * if locals.var_t2_dn9 == 0.0 && ((assign16160_e26470) as f64).is_finite() && ((assign16160_e26470) as f64).fract() == 0.0 { 0.0 } else { (assign16160_e26471 * (locals.var_t2_dn9 * (assign16160_e26463).ln())) })) / (assign16160_e26471 * assign16160_e26471)), (((locals.var_uar_i_dn10 * assign16160_e26471) - (assign16160_e26460 * if locals.var_t2_dn10 == 0.0 && ((assign16160_e26470) as f64).is_finite() && ((assign16160_e26470) as f64).fract() == 0.0 { 0.0 } else { (assign16160_e26471 * (locals.var_t2_dn10 * (assign16160_e26463).ln())) })) / (assign16160_e26471 * assign16160_e26471)), (((locals.var_uar_i_dn11 * assign16160_e26471) - (assign16160_e26460 * if locals.var_t2_dn11 == 0.0 && ((assign16160_e26470) as f64).is_finite() && ((assign16160_e26470) as f64).fract() == 0.0 { 0.0 } else { (assign16160_e26471 * (locals.var_t2_dn11 * (assign16160_e26463).ln())) })) / (assign16160_e26471 * assign16160_e26471)), (((locals.var_uar_i_dn13 * assign16160_e26471) - (assign16160_e26460 * if locals.var_t2_dn13 == 0.0 && ((assign16160_e26470) as f64).is_finite() && ((assign16160_e26470) as f64).fract() == 0.0 { 0.0 } else { (assign16160_e26471 * (locals.var_t2_dn13 * (assign16160_e26463).ln())) })) / (assign16160_e26471 * assign16160_e26471)), (((locals.var_uar_i_dn14 * assign16160_e26471) - (assign16160_e26460 * if locals.var_t2_dn14 == 0.0 && ((assign16160_e26470) as f64).is_finite() && ((assign16160_e26470) as f64).fract() == 0.0 { 0.0 } else { (assign16160_e26471 * (locals.var_t2_dn14 * (assign16160_e26463).ln())) })) / (assign16160_e26471 * assign16160_e26471)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign16160_e26474;
        locals.var_t1_dn0 = assign16160_e26474_d_n0;
        locals.var_t1_dn2 = assign16160_e26474_d_n2;
        locals.var_t1_dn3 = assign16160_e26474_d_n3;
        locals.var_t1_dn4 = assign16160_e26474_d_n4;
        locals.var_t1_dn5 = assign16160_e26474_d_n5;
        locals.var_t1_dn6 = assign16160_e26474_d_n6;
        locals.var_t1_dn7 = assign16160_e26474_d_n7;
        locals.var_t1_dn8 = assign16160_e26474_d_n8;
        locals.var_t1_dn9 = assign16160_e26474_d_n9;
        locals.var_t1_dn10 = assign16160_e26474_d_n10;
        locals.var_t1_dn11 = assign16160_e26474_d_n11;
        locals.var_t1_dn13 = assign16160_e26474_d_n13;
        locals.var_t1_dn14 = assign16160_e26474_d_n14;

    }

    pub(super) fn stamp_transient_block_49(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign16170_e26494, assign16170_e26494_d_n0, assign16170_e26494_d_n2, assign16170_e26494_d_n3, assign16170_e26494_d_n4, assign16170_e26494_d_n5, assign16170_e26494_d_n6, assign16170_e26494_d_n7, assign16170_e26494_d_n8, assign16170_e26494_d_n9, assign16170_e26494_d_n10, assign16170_e26494_d_n11, assign16170_e26494_d_n13, assign16170_e26494_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard283 != 0.0)) && (locals.var_guard284 != 0.0)) {
        let assign16170_e26489: f64 = (locals.var_ua2_i * locals.var_tratio);
        let assign16170_e26490: f64 = (locals.var_t2 + assign16170_e26489);
        let assign16170_e26491: f64 = (locals.var_tratio).powf(assign16170_e26490);
        let assign16170_e26492: f64 = (locals.var_t1 * assign16170_e26491);
        (assign16170_e26492, ((locals.var_t1_dn0 * assign16170_e26491) + (locals.var_t1 * if locals.var_t2_dn0 == 0.0 && ((assign16170_e26490) as f64).is_finite() && ((assign16170_e26490) as f64).fract() == 0.0 { 0.0 } else { (assign16170_e26491 * (locals.var_t2_dn0 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn2 * assign16170_e26491) + (locals.var_t1 * if locals.var_t2_dn2 == 0.0 && ((assign16170_e26490) as f64).is_finite() && ((assign16170_e26490) as f64).fract() == 0.0 { 0.0 } else { (assign16170_e26491 * (locals.var_t2_dn2 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn3 * assign16170_e26491) + (locals.var_t1 * if locals.var_t2_dn3 == 0.0 && ((assign16170_e26490) as f64).is_finite() && ((assign16170_e26490) as f64).fract() == 0.0 { 0.0 } else { (assign16170_e26491 * (locals.var_t2_dn3 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn4 * assign16170_e26491) + (locals.var_t1 * if (locals.var_t2_dn4 + (locals.var_ua2_i * locals.var_tratio_dn4)) == 0.0 && ((assign16170_e26490) as f64).is_finite() && ((assign16170_e26490) as f64).fract() == 0.0 { if assign16170_e26490 == 0.0 { 0.0 } else { (assign16170_e26490 * ((locals.var_tratio).powf(assign16170_e26490 - 1.0) * locals.var_tratio_dn4)) } } else { (assign16170_e26491 * (((locals.var_t2_dn4 + (locals.var_ua2_i * locals.var_tratio_dn4)) * (locals.var_tratio).ln()) + (assign16170_e26490 * (locals.var_tratio_dn4 / locals.var_tratio)))) })), ((locals.var_t1_dn5 * assign16170_e26491) + (locals.var_t1 * if locals.var_t2_dn5 == 0.0 && ((assign16170_e26490) as f64).is_finite() && ((assign16170_e26490) as f64).fract() == 0.0 { 0.0 } else { (assign16170_e26491 * (locals.var_t2_dn5 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn6 * assign16170_e26491) + (locals.var_t1 * if locals.var_t2_dn6 == 0.0 && ((assign16170_e26490) as f64).is_finite() && ((assign16170_e26490) as f64).fract() == 0.0 { 0.0 } else { (assign16170_e26491 * (locals.var_t2_dn6 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn7 * assign16170_e26491) + (locals.var_t1 * if locals.var_t2_dn7 == 0.0 && ((assign16170_e26490) as f64).is_finite() && ((assign16170_e26490) as f64).fract() == 0.0 { 0.0 } else { (assign16170_e26491 * (locals.var_t2_dn7 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn8 * assign16170_e26491) + (locals.var_t1 * if locals.var_t2_dn8 == 0.0 && ((assign16170_e26490) as f64).is_finite() && ((assign16170_e26490) as f64).fract() == 0.0 { 0.0 } else { (assign16170_e26491 * (locals.var_t2_dn8 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn9 * assign16170_e26491) + (locals.var_t1 * if locals.var_t2_dn9 == 0.0 && ((assign16170_e26490) as f64).is_finite() && ((assign16170_e26490) as f64).fract() == 0.0 { 0.0 } else { (assign16170_e26491 * (locals.var_t2_dn9 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn10 * assign16170_e26491) + (locals.var_t1 * if locals.var_t2_dn10 == 0.0 && ((assign16170_e26490) as f64).is_finite() && ((assign16170_e26490) as f64).fract() == 0.0 { 0.0 } else { (assign16170_e26491 * (locals.var_t2_dn10 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn11 * assign16170_e26491) + (locals.var_t1 * if locals.var_t2_dn11 == 0.0 && ((assign16170_e26490) as f64).is_finite() && ((assign16170_e26490) as f64).fract() == 0.0 { 0.0 } else { (assign16170_e26491 * (locals.var_t2_dn11 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn13 * assign16170_e26491) + (locals.var_t1 * if locals.var_t2_dn13 == 0.0 && ((assign16170_e26490) as f64).is_finite() && ((assign16170_e26490) as f64).fract() == 0.0 { 0.0 } else { (assign16170_e26491 * (locals.var_t2_dn13 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn14 * assign16170_e26491) + (locals.var_t1 * if locals.var_t2_dn14 == 0.0 && ((assign16170_e26490) as f64).is_finite() && ((assign16170_e26490) as f64).fract() == 0.0 { 0.0 } else { (assign16170_e26491 * (locals.var_t2_dn14 * (locals.var_tratio).ln())) })),)
    } else {
        (locals.var_uar_tl, locals.var_uar_tl_dn0, locals.var_uar_tl_dn2, locals.var_uar_tl_dn3, locals.var_uar_tl_dn4, locals.var_uar_tl_dn5, locals.var_uar_tl_dn6, locals.var_uar_tl_dn7, locals.var_uar_tl_dn8, locals.var_uar_tl_dn9, locals.var_uar_tl_dn10, locals.var_uar_tl_dn11, locals.var_uar_tl_dn13, locals.var_uar_tl_dn14,)
    }
};
        locals.var_uar_tl = assign16170_e26494;
        locals.var_uar_tl_dn0 = assign16170_e26494_d_n0;
        locals.var_uar_tl_dn2 = assign16170_e26494_d_n2;
        locals.var_uar_tl_dn3 = assign16170_e26494_d_n3;
        locals.var_uar_tl_dn4 = assign16170_e26494_d_n4;
        locals.var_uar_tl_dn5 = assign16170_e26494_d_n5;
        locals.var_uar_tl_dn6 = assign16170_e26494_d_n6;
        locals.var_uar_tl_dn7 = assign16170_e26494_d_n7;
        locals.var_uar_tl_dn8 = assign16170_e26494_d_n8;
        locals.var_uar_tl_dn9 = assign16170_e26494_d_n9;
        locals.var_uar_tl_dn10 = assign16170_e26494_d_n10;
        locals.var_uar_tl_dn11 = assign16170_e26494_d_n11;
        locals.var_uar_tl_dn13 = assign16170_e26494_d_n13;
        locals.var_uar_tl_dn14 = assign16170_e26494_d_n14;

        let (assign16180_e26510, assign16180_e26510_d_n0, assign16180_e26510_d_n2, assign16180_e26510_d_n3, assign16180_e26510_d_n4, assign16180_e26510_d_n5, assign16180_e26510_d_n6, assign16180_e26510_d_n7, assign16180_e26510_d_n8, assign16180_e26510_d_n9, assign16180_e26510_d_n10, assign16180_e26510_d_n11, assign16180_e26510_d_n13, assign16180_e26510_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard283 != 0.0)) && (locals.var_guard284 != 0.0)) {
        let assign16180_e26507: f64 = (locals.var_ua1r_i * locals.var_deltemp);
        let assign16180_e26508: f64 = (locals.var_uar_i + assign16180_e26507);
        (assign16180_e26508, locals.var_uar_i_dn0, locals.var_uar_i_dn2, locals.var_uar_i_dn3, (locals.var_uar_i_dn4 + (locals.var_ua1r_i * locals.var_deltemp_dn4)), locals.var_uar_i_dn5, locals.var_uar_i_dn6, locals.var_uar_i_dn7, locals.var_uar_i_dn8, locals.var_uar_i_dn9, locals.var_uar_i_dn10, locals.var_uar_i_dn11, locals.var_uar_i_dn13, locals.var_uar_i_dn14,)
    } else {
        (locals.var_uar_th, locals.var_uar_th_dn0, locals.var_uar_th_dn2, locals.var_uar_th_dn3, locals.var_uar_th_dn4, locals.var_uar_th_dn5, locals.var_uar_th_dn6, locals.var_uar_th_dn7, locals.var_uar_th_dn8, locals.var_uar_th_dn9, locals.var_uar_th_dn10, locals.var_uar_th_dn11, locals.var_uar_th_dn13, locals.var_uar_th_dn14,)
    }
};
        locals.var_uar_th = assign16180_e26510;
        locals.var_uar_th_dn0 = assign16180_e26510_d_n0;
        locals.var_uar_th_dn2 = assign16180_e26510_d_n2;
        locals.var_uar_th_dn3 = assign16180_e26510_d_n3;
        locals.var_uar_th_dn4 = assign16180_e26510_d_n4;
        locals.var_uar_th_dn5 = assign16180_e26510_d_n5;
        locals.var_uar_th_dn6 = assign16180_e26510_d_n6;
        locals.var_uar_th_dn7 = assign16180_e26510_d_n7;
        locals.var_uar_th_dn8 = assign16180_e26510_d_n8;
        locals.var_uar_th_dn9 = assign16180_e26510_d_n9;
        locals.var_uar_th_dn10 = assign16180_e26510_d_n10;
        locals.var_uar_th_dn11 = assign16180_e26510_d_n11;
        locals.var_uar_th_dn13 = assign16180_e26510_d_n13;
        locals.var_uar_th_dn14 = assign16180_e26510_d_n14;

        let (assign16190_e26566, assign16190_e26566_d_n0, assign16190_e26566_d_n2, assign16190_e26566_d_n3, assign16190_e26566_d_n4, assign16190_e26566_d_n5, assign16190_e26566_d_n6, assign16190_e26566_d_n7, assign16190_e26566_d_n8, assign16190_e26566_d_n9, assign16190_e26566_d_n10, assign16190_e26566_d_n11, assign16190_e26566_d_n13, assign16190_e26566_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard283 != 0.0)) && (locals.var_guard284 == 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_tnom;
        let assign16190_e26524: f64 = (210.0 * __rspice_inv_cse_0);
        let assign16190_e26529: f64 = (210.0 * __rspice_inv_cse_0);
        let assign16190_e26530: f64 = (locals.var_ua2_i * assign16190_e26529);
        let assign16190_e26531: f64 = (locals.var_ua1r_i + assign16190_e26530);
        let assign16190_e26532: f64 = (assign16190_e26524).powf(assign16190_e26531);
        let assign16190_e26533: f64 = (locals.var_uar_i * assign16190_e26532);
        let assign16190_e26536: f64 = (locals.var_ua1r_i / 210.0);
        let assign16190_e26540: f64 = (210.0 / locals.var_tnom);
        let (assign16190_e26557,) = {
            if (!(assign16190_e26540 > 1e-38)) {
                let assign16190_e26545: f64 = (-87.498233534);
                (assign16190_e26545,)
            } else {
                let assign16190_e26548: f64 = (210.0 / locals.var_tnom);
                let (assign16190_e26556,) = {
                    if (assign16190_e26548 > 1e-38) {
                        let assign16190_e26553: f64 = (210.0 / locals.var_tnom);
                        let assign16190_e26554: f64 = (assign16190_e26553).ln();
                        (assign16190_e26554,)
                    } else {
                        (0.0,)
                    }
                };
                (assign16190_e26556,)
            }
        };
        let assign16190_e26559: f64 = (assign16190_e26557 + 1.0);
        let assign16190_e26560: f64 = (locals.var_ua2_i * assign16190_e26559);
        let assign16190_e26562: f64 = (assign16190_e26560 / locals.var_tnom);
        let assign16190_e26563: f64 = (assign16190_e26536 + assign16190_e26562);
        let assign16190_e26564: f64 = (assign16190_e26533 * assign16190_e26563);
        (assign16190_e26564, ((locals.var_uar_i_dn0 * assign16190_e26532) * assign16190_e26563), ((locals.var_uar_i_dn2 * assign16190_e26532) * assign16190_e26563), ((locals.var_uar_i_dn3 * assign16190_e26532) * assign16190_e26563), ((locals.var_uar_i_dn4 * assign16190_e26532) * assign16190_e26563), ((locals.var_uar_i_dn5 * assign16190_e26532) * assign16190_e26563), ((locals.var_uar_i_dn6 * assign16190_e26532) * assign16190_e26563), ((locals.var_uar_i_dn7 * assign16190_e26532) * assign16190_e26563), ((locals.var_uar_i_dn8 * assign16190_e26532) * assign16190_e26563), ((locals.var_uar_i_dn9 * assign16190_e26532) * assign16190_e26563), ((locals.var_uar_i_dn10 * assign16190_e26532) * assign16190_e26563), ((locals.var_uar_i_dn11 * assign16190_e26532) * assign16190_e26563), ((locals.var_uar_i_dn13 * assign16190_e26532) * assign16190_e26563), ((locals.var_uar_i_dn14 * assign16190_e26532) * assign16190_e26563),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign16190_e26566;
        locals.var_t2_dn0 = assign16190_e26566_d_n0;
        locals.var_t2_dn2 = assign16190_e26566_d_n2;
        locals.var_t2_dn3 = assign16190_e26566_d_n3;
        locals.var_t2_dn4 = assign16190_e26566_d_n4;
        locals.var_t2_dn5 = assign16190_e26566_d_n5;
        locals.var_t2_dn6 = assign16190_e26566_d_n6;
        locals.var_t2_dn7 = assign16190_e26566_d_n7;
        locals.var_t2_dn8 = assign16190_e26566_d_n8;
        locals.var_t2_dn9 = assign16190_e26566_d_n9;
        locals.var_t2_dn10 = assign16190_e26566_d_n10;
        locals.var_t2_dn11 = assign16190_e26566_d_n11;
        locals.var_t2_dn13 = assign16190_e26566_d_n13;
        locals.var_t2_dn14 = assign16190_e26566_d_n14;

        let (assign16200_e26597, assign16200_e26597_d_n0, assign16200_e26597_d_n2, assign16200_e26597_d_n3, assign16200_e26597_d_n4, assign16200_e26597_d_n5, assign16200_e26597_d_n6, assign16200_e26597_d_n7, assign16200_e26597_d_n8, assign16200_e26597_d_n9, assign16200_e26597_d_n10, assign16200_e26597_d_n11, assign16200_e26597_d_n13, assign16200_e26597_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard283 != 0.0)) && (locals.var_guard284 == 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_tnom;
        let assign16200_e26580: f64 = (210.0 * __rspice_inv_cse_1);
        let assign16200_e26585: f64 = (210.0 * __rspice_inv_cse_1);
        let assign16200_e26586: f64 = (locals.var_ua2_i * assign16200_e26585);
        let assign16200_e26587: f64 = (locals.var_ua1r_i + assign16200_e26586);
        let assign16200_e26588: f64 = (assign16200_e26580).powf(assign16200_e26587);
        let assign16200_e26589: f64 = (locals.var_uar_i * assign16200_e26588);
        let assign16200_e26593: f64 = (210.0 - locals.var_tnom);
        let assign16200_e26594: f64 = (locals.var_t2 * assign16200_e26593);
        let assign16200_e26595: f64 = (assign16200_e26589 - assign16200_e26594);
        (assign16200_e26595, ((locals.var_uar_i_dn0 * assign16200_e26588) - (locals.var_t2_dn0 * assign16200_e26593)), ((locals.var_uar_i_dn2 * assign16200_e26588) - (locals.var_t2_dn2 * assign16200_e26593)), ((locals.var_uar_i_dn3 * assign16200_e26588) - (locals.var_t2_dn3 * assign16200_e26593)), ((locals.var_uar_i_dn4 * assign16200_e26588) - (locals.var_t2_dn4 * assign16200_e26593)), ((locals.var_uar_i_dn5 * assign16200_e26588) - (locals.var_t2_dn5 * assign16200_e26593)), ((locals.var_uar_i_dn6 * assign16200_e26588) - (locals.var_t2_dn6 * assign16200_e26593)), ((locals.var_uar_i_dn7 * assign16200_e26588) - (locals.var_t2_dn7 * assign16200_e26593)), ((locals.var_uar_i_dn8 * assign16200_e26588) - (locals.var_t2_dn8 * assign16200_e26593)), ((locals.var_uar_i_dn9 * assign16200_e26588) - (locals.var_t2_dn9 * assign16200_e26593)), ((locals.var_uar_i_dn10 * assign16200_e26588) - (locals.var_t2_dn10 * assign16200_e26593)), ((locals.var_uar_i_dn11 * assign16200_e26588) - (locals.var_t2_dn11 * assign16200_e26593)), ((locals.var_uar_i_dn13 * assign16200_e26588) - (locals.var_t2_dn13 * assign16200_e26593)), ((locals.var_uar_i_dn14 * assign16200_e26588) - (locals.var_t2_dn14 * assign16200_e26593)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign16200_e26597;
        locals.var_t1_dn0 = assign16200_e26597_d_n0;
        locals.var_t1_dn2 = assign16200_e26597_d_n2;
        locals.var_t1_dn3 = assign16200_e26597_d_n3;
        locals.var_t1_dn4 = assign16200_e26597_d_n4;
        locals.var_t1_dn5 = assign16200_e26597_d_n5;
        locals.var_t1_dn6 = assign16200_e26597_d_n6;
        locals.var_t1_dn7 = assign16200_e26597_d_n7;
        locals.var_t1_dn8 = assign16200_e26597_d_n8;
        locals.var_t1_dn9 = assign16200_e26597_d_n9;
        locals.var_t1_dn10 = assign16200_e26597_d_n10;
        locals.var_t1_dn11 = assign16200_e26597_d_n11;
        locals.var_t1_dn13 = assign16200_e26597_d_n13;
        locals.var_t1_dn14 = assign16200_e26597_d_n14;

        let (assign16210_e26618, assign16210_e26618_d_n0, assign16210_e26618_d_n2, assign16210_e26618_d_n3, assign16210_e26618_d_n4, assign16210_e26618_d_n5, assign16210_e26618_d_n6, assign16210_e26618_d_n7, assign16210_e26618_d_n8, assign16210_e26618_d_n9, assign16210_e26618_d_n10, assign16210_e26618_d_n11, assign16210_e26618_d_n13, assign16210_e26618_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard283 != 0.0)) && (locals.var_guard284 == 0.0)) {
        let assign16210_e26613: f64 = (locals.var_ua2_i * locals.var_tratio);
        let assign16210_e26614: f64 = (locals.var_ua1r_i + assign16210_e26613);
        let assign16210_e26615: f64 = (locals.var_tratio).powf(assign16210_e26614);
        let assign16210_e26616: f64 = (locals.var_uar_i * assign16210_e26615);
        (assign16210_e26616, (locals.var_uar_i_dn0 * assign16210_e26615), (locals.var_uar_i_dn2 * assign16210_e26615), (locals.var_uar_i_dn3 * assign16210_e26615), ((locals.var_uar_i_dn4 * assign16210_e26615) + (locals.var_uar_i * if (locals.var_ua2_i * locals.var_tratio_dn4) == 0.0 && ((assign16210_e26614) as f64).is_finite() && ((assign16210_e26614) as f64).fract() == 0.0 { if assign16210_e26614 == 0.0 { 0.0 } else { (assign16210_e26614 * ((locals.var_tratio).powf(assign16210_e26614 - 1.0) * locals.var_tratio_dn4)) } } else { (assign16210_e26615 * (((locals.var_ua2_i * locals.var_tratio_dn4) * (locals.var_tratio).ln()) + (assign16210_e26614 * (locals.var_tratio_dn4 / locals.var_tratio)))) })), (locals.var_uar_i_dn5 * assign16210_e26615), (locals.var_uar_i_dn6 * assign16210_e26615), (locals.var_uar_i_dn7 * assign16210_e26615), (locals.var_uar_i_dn8 * assign16210_e26615), (locals.var_uar_i_dn9 * assign16210_e26615), (locals.var_uar_i_dn10 * assign16210_e26615), (locals.var_uar_i_dn11 * assign16210_e26615), (locals.var_uar_i_dn13 * assign16210_e26615), (locals.var_uar_i_dn14 * assign16210_e26615),)
    } else {
        (locals.var_uar_tl, locals.var_uar_tl_dn0, locals.var_uar_tl_dn2, locals.var_uar_tl_dn3, locals.var_uar_tl_dn4, locals.var_uar_tl_dn5, locals.var_uar_tl_dn6, locals.var_uar_tl_dn7, locals.var_uar_tl_dn8, locals.var_uar_tl_dn9, locals.var_uar_tl_dn10, locals.var_uar_tl_dn11, locals.var_uar_tl_dn13, locals.var_uar_tl_dn14,)
    }
};
        locals.var_uar_tl = assign16210_e26618;
        locals.var_uar_tl_dn0 = assign16210_e26618_d_n0;
        locals.var_uar_tl_dn2 = assign16210_e26618_d_n2;
        locals.var_uar_tl_dn3 = assign16210_e26618_d_n3;
        locals.var_uar_tl_dn4 = assign16210_e26618_d_n4;
        locals.var_uar_tl_dn5 = assign16210_e26618_d_n5;
        locals.var_uar_tl_dn6 = assign16210_e26618_d_n6;
        locals.var_uar_tl_dn7 = assign16210_e26618_d_n7;
        locals.var_uar_tl_dn8 = assign16210_e26618_d_n8;
        locals.var_uar_tl_dn9 = assign16210_e26618_d_n9;
        locals.var_uar_tl_dn10 = assign16210_e26618_d_n10;
        locals.var_uar_tl_dn11 = assign16210_e26618_d_n11;
        locals.var_uar_tl_dn13 = assign16210_e26618_d_n13;
        locals.var_uar_tl_dn14 = assign16210_e26618_d_n14;

        let (assign16220_e26635, assign16220_e26635_d_n0, assign16220_e26635_d_n2, assign16220_e26635_d_n3, assign16220_e26635_d_n4, assign16220_e26635_d_n5, assign16220_e26635_d_n6, assign16220_e26635_d_n7, assign16220_e26635_d_n8, assign16220_e26635_d_n9, assign16220_e26635_d_n10, assign16220_e26635_d_n11, assign16220_e26635_d_n13, assign16220_e26635_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard283 != 0.0)) && (locals.var_guard284 == 0.0)) {
        let assign16220_e26632: f64 = (locals.var_t2 * locals.var_deltemp);
        let assign16220_e26633: f64 = (locals.var_t1 + assign16220_e26632);
        (assign16220_e26633, (locals.var_t1_dn0 + (locals.var_t2_dn0 * locals.var_deltemp)), (locals.var_t1_dn2 + (locals.var_t2_dn2 * locals.var_deltemp)), (locals.var_t1_dn3 + (locals.var_t2_dn3 * locals.var_deltemp)), (locals.var_t1_dn4 + ((locals.var_t2_dn4 * locals.var_deltemp) + (locals.var_t2 * locals.var_deltemp_dn4))), (locals.var_t1_dn5 + (locals.var_t2_dn5 * locals.var_deltemp)), (locals.var_t1_dn6 + (locals.var_t2_dn6 * locals.var_deltemp)), (locals.var_t1_dn7 + (locals.var_t2_dn7 * locals.var_deltemp)), (locals.var_t1_dn8 + (locals.var_t2_dn8 * locals.var_deltemp)), (locals.var_t1_dn9 + (locals.var_t2_dn9 * locals.var_deltemp)), (locals.var_t1_dn10 + (locals.var_t2_dn10 * locals.var_deltemp)), (locals.var_t1_dn11 + (locals.var_t2_dn11 * locals.var_deltemp)), (locals.var_t1_dn13 + (locals.var_t2_dn13 * locals.var_deltemp)), (locals.var_t1_dn14 + (locals.var_t2_dn14 * locals.var_deltemp)),)
    } else {
        (locals.var_uar_th, locals.var_uar_th_dn0, locals.var_uar_th_dn2, locals.var_uar_th_dn3, locals.var_uar_th_dn4, locals.var_uar_th_dn5, locals.var_uar_th_dn6, locals.var_uar_th_dn7, locals.var_uar_th_dn8, locals.var_uar_th_dn9, locals.var_uar_th_dn10, locals.var_uar_th_dn11, locals.var_uar_th_dn13, locals.var_uar_th_dn14,)
    }
};
        locals.var_uar_th = assign16220_e26635;
        locals.var_uar_th_dn0 = assign16220_e26635_d_n0;
        locals.var_uar_th_dn2 = assign16220_e26635_d_n2;
        locals.var_uar_th_dn3 = assign16220_e26635_d_n3;
        locals.var_uar_th_dn4 = assign16220_e26635_d_n4;
        locals.var_uar_th_dn5 = assign16220_e26635_d_n5;
        locals.var_uar_th_dn6 = assign16220_e26635_d_n6;
        locals.var_uar_th_dn7 = assign16220_e26635_d_n7;
        locals.var_uar_th_dn8 = assign16220_e26635_d_n8;
        locals.var_uar_th_dn9 = assign16220_e26635_d_n9;
        locals.var_uar_th_dn10 = assign16220_e26635_d_n10;
        locals.var_uar_th_dn11 = assign16220_e26635_d_n11;
        locals.var_uar_th_dn13 = assign16220_e26635_d_n13;
        locals.var_uar_th_dn14 = assign16220_e26635_d_n14;

        let (assign16230_e26651, assign16230_e26651_d_n0, assign16230_e26651_d_n2, assign16230_e26651_d_n3, assign16230_e26651_d_n4, assign16230_e26651_d_n5, assign16230_e26651_d_n6, assign16230_e26651_d_n7, assign16230_e26651_d_n8, assign16230_e26651_d_n9, assign16230_e26651_d_n10, assign16230_e26651_d_n11, assign16230_e26651_d_n13, assign16230_e26651_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard283 != 0.0)) {
        let assign16230_e26645: f64 = (locals.var_wl * locals.var_uar_tl);
        let assign16230_e26648: f64 = (locals.var_wh * locals.var_uar_th);
        let assign16230_e26649: f64 = (assign16230_e26645 + assign16230_e26648);
        (assign16230_e26649, ((locals.var_wl * locals.var_uar_tl_dn0) + (locals.var_wh * locals.var_uar_th_dn0)), ((locals.var_wl * locals.var_uar_tl_dn2) + (locals.var_wh * locals.var_uar_th_dn2)), ((locals.var_wl * locals.var_uar_tl_dn3) + (locals.var_wh * locals.var_uar_th_dn3)), (((locals.var_wl_dn4 * locals.var_uar_tl) + (locals.var_wl * locals.var_uar_tl_dn4)) + ((locals.var_wh_dn4 * locals.var_uar_th) + (locals.var_wh * locals.var_uar_th_dn4))), ((locals.var_wl * locals.var_uar_tl_dn5) + (locals.var_wh * locals.var_uar_th_dn5)), ((locals.var_wl * locals.var_uar_tl_dn6) + (locals.var_wh * locals.var_uar_th_dn6)), ((locals.var_wl * locals.var_uar_tl_dn7) + (locals.var_wh * locals.var_uar_th_dn7)), ((locals.var_wl * locals.var_uar_tl_dn8) + (locals.var_wh * locals.var_uar_th_dn8)), ((locals.var_wl * locals.var_uar_tl_dn9) + (locals.var_wh * locals.var_uar_th_dn9)), ((locals.var_wl * locals.var_uar_tl_dn10) + (locals.var_wh * locals.var_uar_th_dn10)), ((locals.var_wl * locals.var_uar_tl_dn11) + (locals.var_wh * locals.var_uar_th_dn11)), ((locals.var_wl * locals.var_uar_tl_dn13) + (locals.var_wh * locals.var_uar_th_dn13)), ((locals.var_wl * locals.var_uar_tl_dn14) + (locals.var_wh * locals.var_uar_th_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign16230_e26651;
        locals.var_t0_dn0 = assign16230_e26651_d_n0;
        locals.var_t0_dn2 = assign16230_e26651_d_n2;
        locals.var_t0_dn3 = assign16230_e26651_d_n3;
        locals.var_t0_dn4 = assign16230_e26651_d_n4;
        locals.var_t0_dn5 = assign16230_e26651_d_n5;
        locals.var_t0_dn6 = assign16230_e26651_d_n6;
        locals.var_t0_dn7 = assign16230_e26651_d_n7;
        locals.var_t0_dn8 = assign16230_e26651_d_n8;
        locals.var_t0_dn9 = assign16230_e26651_d_n9;
        locals.var_t0_dn10 = assign16230_e26651_d_n10;
        locals.var_t0_dn11 = assign16230_e26651_d_n11;
        locals.var_t0_dn13 = assign16230_e26651_d_n13;
        locals.var_t0_dn14 = assign16230_e26651_d_n14;

        let (assign16240_e26696, assign16240_e26696_d_n0, assign16240_e26696_d_n2, assign16240_e26696_d_n3, assign16240_e26696_d_n4, assign16240_e26696_d_n5, assign16240_e26696_d_n6, assign16240_e26696_d_n7, assign16240_e26696_d_n8, assign16240_e26696_d_n9, assign16240_e26696_d_n10, assign16240_e26696_d_n11, assign16240_e26696_d_n13, assign16240_e26696_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard283 != 0.0)) {
        let assign16240_e26661: f64 = (-10000.0);
        let assign16240_e26663: f64 = (assign16240_e26661 * 1e-6);
        let (assign16240_e26694, assign16240_e26694_d_n0, assign16240_e26694_d_n2, assign16240_e26694_d_n3, assign16240_e26694_d_n4, assign16240_e26694_d_n5, assign16240_e26694_d_n6, assign16240_e26694_d_n7, assign16240_e26694_d_n8, assign16240_e26694_d_n9, assign16240_e26694_d_n10, assign16240_e26694_d_n11, assign16240_e26694_d_n13, assign16240_e26694_d_n14,) = {
            if (!(locals.var_t0 < assign16240_e26663)) {
                let assign16240_e26670: f64 = (locals.var_t0 * locals.var_t0);
                let assign16240_e26673: f64 = (4.0 * 1e-6);
                let assign16240_e26675: f64 = (assign16240_e26673 * 1e-6);
                let assign16240_e26676: f64 = (assign16240_e26670 + assign16240_e26675);
                let assign16240_e26677: f64 = (assign16240_e26676).sqrt();
                let assign16240_e26678: f64 = (locals.var_t0 + assign16240_e26677);
                let assign16240_e26679: f64 = (0.5 * assign16240_e26678);
                (assign16240_e26679, (0.5 * (locals.var_t0_dn0 + (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign16240_e26677)))), (0.5 * (locals.var_t0_dn2 + (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign16240_e26677)))), (0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign16240_e26677)))), (0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign16240_e26677)))), (0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign16240_e26677)))), (0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign16240_e26677)))), (0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign16240_e26677)))), (0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign16240_e26677)))), (0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign16240_e26677)))), (0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign16240_e26677)))), (0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign16240_e26677)))), (0.5 * (locals.var_t0_dn13 + (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign16240_e26677)))), (0.5 * (locals.var_t0_dn14 + (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign16240_e26677)))),)
            } else {
                let assign16240_e26682: f64 = (-10000.0);
                let assign16240_e26684: f64 = (assign16240_e26682 * 1e-6);
                let (assign16240_e26693, assign16240_e26693_d_n0, assign16240_e26693_d_n2, assign16240_e26693_d_n3, assign16240_e26693_d_n4, assign16240_e26693_d_n5, assign16240_e26693_d_n6, assign16240_e26693_d_n7, assign16240_e26693_d_n8, assign16240_e26693_d_n9, assign16240_e26693_d_n10, assign16240_e26693_d_n11, assign16240_e26693_d_n13, assign16240_e26693_d_n14,) = {
                    if (locals.var_t0 < assign16240_e26684) {
                        let assign16240_e26687: f64 = (-1e-6);
                        let assign16240_e26689: f64 = (assign16240_e26687 * 1e-6);
                        let assign16240_e26691: f64 = (assign16240_e26689 / locals.var_t0);
                        (assign16240_e26691, (-((assign16240_e26689 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((assign16240_e26689 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((assign16240_e26689 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (-((assign16240_e26689 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((assign16240_e26689 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((assign16240_e26689 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((assign16240_e26689 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((assign16240_e26689 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((assign16240_e26689 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), (-((assign16240_e26689 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((assign16240_e26689 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((assign16240_e26689 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))), (-((assign16240_e26689 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign16240_e26693, assign16240_e26693_d_n0, assign16240_e26693_d_n2, assign16240_e26693_d_n3, assign16240_e26693_d_n4, assign16240_e26693_d_n5, assign16240_e26693_d_n6, assign16240_e26693_d_n7, assign16240_e26693_d_n8, assign16240_e26693_d_n9, assign16240_e26693_d_n10, assign16240_e26693_d_n11, assign16240_e26693_d_n13, assign16240_e26693_d_n14,)
            }
        };
        (assign16240_e26694, assign16240_e26694_d_n0, assign16240_e26694_d_n2, assign16240_e26694_d_n3, assign16240_e26694_d_n4, assign16240_e26694_d_n5, assign16240_e26694_d_n6, assign16240_e26694_d_n7, assign16240_e26694_d_n8, assign16240_e26694_d_n9, assign16240_e26694_d_n10, assign16240_e26694_d_n11, assign16240_e26694_d_n13, assign16240_e26694_d_n14,)
    } else {
        (locals.var_uar_t, locals.var_uar_t_dn0, locals.var_uar_t_dn2, locals.var_uar_t_dn3, locals.var_uar_t_dn4, locals.var_uar_t_dn5, locals.var_uar_t_dn6, locals.var_uar_t_dn7, locals.var_uar_t_dn8, locals.var_uar_t_dn9, locals.var_uar_t_dn10, locals.var_uar_t_dn11, locals.var_uar_t_dn13, locals.var_uar_t_dn14,)
    }
};
        locals.var_uar_t = assign16240_e26696;
        locals.var_uar_t_dn0 = assign16240_e26696_d_n0;
        locals.var_uar_t_dn2 = assign16240_e26696_d_n2;
        locals.var_uar_t_dn3 = assign16240_e26696_d_n3;
        locals.var_uar_t_dn4 = assign16240_e26696_d_n4;
        locals.var_uar_t_dn5 = assign16240_e26696_d_n5;
        locals.var_uar_t_dn6 = assign16240_e26696_d_n6;
        locals.var_uar_t_dn7 = assign16240_e26696_d_n7;
        locals.var_uar_t_dn8 = assign16240_e26696_d_n8;
        locals.var_uar_t_dn9 = assign16240_e26696_d_n9;
        locals.var_uar_t_dn10 = assign16240_e26696_d_n10;
        locals.var_uar_t_dn11 = assign16240_e26696_d_n11;
        locals.var_uar_t_dn13 = assign16240_e26696_d_n13;
        locals.var_uar_t_dn14 = assign16240_e26696_d_n14;

        let (assign16250_e26713, assign16250_e26713_d_n0, assign16250_e26713_d_n2, assign16250_e26713_d_n3, assign16250_e26713_d_n4, assign16250_e26713_d_n5, assign16250_e26713_d_n6, assign16250_e26713_d_n7, assign16250_e26713_d_n8, assign16250_e26713_d_n9, assign16250_e26713_d_n10, assign16250_e26713_d_n11, assign16250_e26713_d_n13, assign16250_e26713_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) {
        let assign16250_e26706: f64 = (locals.var_ud2_i * locals.var_deltratio1);
        let assign16250_e26707: f64 = (locals.var_ud1_i + assign16250_e26706);
        let assign16250_e26709: f64 = (assign16250_e26707 * locals.var_trat_ln);
        let assign16250_e26710: f64 = (assign16250_e26709).exp();
        let assign16250_e26711: f64 = (locals.var_ud_i * assign16250_e26710);
        (assign16250_e26711, (locals.var_ud_i_dn0 * assign16250_e26710), (locals.var_ud_i_dn2 * assign16250_e26710), (locals.var_ud_i_dn3 * assign16250_e26710), ((locals.var_ud_i_dn4 * assign16250_e26710) + (locals.var_ud_i * (assign16250_e26710 * (((locals.var_ud2_i * locals.var_deltratio1_dn4) * locals.var_trat_ln) + (assign16250_e26707 * locals.var_trat_ln_dn4))))), (locals.var_ud_i_dn5 * assign16250_e26710), (locals.var_ud_i_dn6 * assign16250_e26710), (locals.var_ud_i_dn7 * assign16250_e26710), (locals.var_ud_i_dn8 * assign16250_e26710), (locals.var_ud_i_dn9 * assign16250_e26710), (locals.var_ud_i_dn10 * assign16250_e26710), (locals.var_ud_i_dn11 * assign16250_e26710), (locals.var_ud_i_dn13 * assign16250_e26710), (locals.var_ud_i_dn14 * assign16250_e26710),)
    } else {
        (locals.var_ud_t, locals.var_ud_t_dn0, locals.var_ud_t_dn2, locals.var_ud_t_dn3, locals.var_ud_t_dn4, locals.var_ud_t_dn5, locals.var_ud_t_dn6, locals.var_ud_t_dn7, locals.var_ud_t_dn8, locals.var_ud_t_dn9, locals.var_ud_t_dn10, locals.var_ud_t_dn11, locals.var_ud_t_dn13, locals.var_ud_t_dn14,)
    }
};
        locals.var_ud_t = assign16250_e26713;
        locals.var_ud_t_dn0 = assign16250_e26713_d_n0;
        locals.var_ud_t_dn2 = assign16250_e26713_d_n2;
        locals.var_ud_t_dn3 = assign16250_e26713_d_n3;
        locals.var_ud_t_dn4 = assign16250_e26713_d_n4;
        locals.var_ud_t_dn5 = assign16250_e26713_d_n5;
        locals.var_ud_t_dn6 = assign16250_e26713_d_n6;
        locals.var_ud_t_dn7 = assign16250_e26713_d_n7;
        locals.var_ud_t_dn8 = assign16250_e26713_d_n8;
        locals.var_ud_t_dn9 = assign16250_e26713_d_n9;
        locals.var_ud_t_dn10 = assign16250_e26713_d_n10;
        locals.var_ud_t_dn11 = assign16250_e26713_d_n11;
        locals.var_ud_t_dn13 = assign16250_e26713_d_n13;
        locals.var_ud_t_dn14 = assign16250_e26713_d_n14;

        let assign16260_e26716: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard285 = assign16260_e26716;

        let (assign16270_e26735, assign16270_e26735_d_n0, assign16270_e26735_d_n2, assign16270_e26735_d_n3, assign16270_e26735_d_n4, assign16270_e26735_d_n5, assign16270_e26735_d_n6, assign16270_e26735_d_n7, assign16270_e26735_d_n8, assign16270_e26735_d_n9, assign16270_e26735_d_n10, assign16270_e26735_d_n11, assign16270_e26735_d_n13, assign16270_e26735_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard285 != 0.0)) {
        let assign16270_e26728: f64 = (locals.var_ud2_i * locals.var_deltratio1);
        let assign16270_e26729: f64 = (locals.var_ud1r_i + assign16270_e26728);
        let assign16270_e26731: f64 = (assign16270_e26729 * locals.var_trat_ln);
        let assign16270_e26732: f64 = (assign16270_e26731).exp();
        let assign16270_e26733: f64 = (locals.var_udr_i * assign16270_e26732);
        (assign16270_e26733, (locals.var_udr_i_dn0 * assign16270_e26732), (locals.var_udr_i_dn2 * assign16270_e26732), (locals.var_udr_i_dn3 * assign16270_e26732), ((locals.var_udr_i_dn4 * assign16270_e26732) + (locals.var_udr_i * (assign16270_e26732 * (((locals.var_ud2_i * locals.var_deltratio1_dn4) * locals.var_trat_ln) + (assign16270_e26729 * locals.var_trat_ln_dn4))))), (locals.var_udr_i_dn5 * assign16270_e26732), (locals.var_udr_i_dn6 * assign16270_e26732), (locals.var_udr_i_dn7 * assign16270_e26732), (locals.var_udr_i_dn8 * assign16270_e26732), (locals.var_udr_i_dn9 * assign16270_e26732), (locals.var_udr_i_dn10 * assign16270_e26732), (locals.var_udr_i_dn11 * assign16270_e26732), (locals.var_udr_i_dn13 * assign16270_e26732), (locals.var_udr_i_dn14 * assign16270_e26732),)
    } else {
        (locals.var_udr_t, locals.var_udr_t_dn0, locals.var_udr_t_dn2, locals.var_udr_t_dn3, locals.var_udr_t_dn4, locals.var_udr_t_dn5, locals.var_udr_t_dn6, locals.var_udr_t_dn7, locals.var_udr_t_dn8, locals.var_udr_t_dn9, locals.var_udr_t_dn10, locals.var_udr_t_dn11, locals.var_udr_t_dn13, locals.var_udr_t_dn14,)
    }
};
        locals.var_udr_t = assign16270_e26735;
        locals.var_udr_t_dn0 = assign16270_e26735_d_n0;
        locals.var_udr_t_dn2 = assign16270_e26735_d_n2;
        locals.var_udr_t_dn3 = assign16270_e26735_d_n3;
        locals.var_udr_t_dn4 = assign16270_e26735_d_n4;
        locals.var_udr_t_dn5 = assign16270_e26735_d_n5;
        locals.var_udr_t_dn6 = assign16270_e26735_d_n6;
        locals.var_udr_t_dn7 = assign16270_e26735_d_n7;
        locals.var_udr_t_dn8 = assign16270_e26735_d_n8;
        locals.var_udr_t_dn9 = assign16270_e26735_d_n9;
        locals.var_udr_t_dn10 = assign16270_e26735_d_n10;
        locals.var_udr_t_dn11 = assign16270_e26735_d_n11;
        locals.var_udr_t_dn13 = assign16270_e26735_d_n13;
        locals.var_udr_t_dn14 = assign16270_e26735_d_n14;

        let (assign16280_e26752, assign16280_e26752_d_n4,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) {
        let assign16280_e26745: f64 = (locals.var_ucste1_i * locals.var_deltratio1);
        let assign16280_e26746: f64 = (locals.var_ucste_i + assign16280_e26745);
        let assign16280_e26748: f64 = (assign16280_e26746 * locals.var_trat_ln);
        let assign16280_e26749: f64 = (assign16280_e26748).exp();
        let assign16280_e26750: f64 = (locals.var_ucs_i * assign16280_e26749);
        (assign16280_e26750, (locals.var_ucs_i * (assign16280_e26749 * (((locals.var_ucste1_i * locals.var_deltratio1_dn4) * locals.var_trat_ln) + (assign16280_e26746 * locals.var_trat_ln_dn4)))),)
    } else {
        (locals.var_ucs_t, locals.var_ucs_t_dn4,)
    }
};
        locals.var_ucs_t = assign16280_e26752;
        locals.var_ucs_t_dn4 = assign16280_e26752_d_n4;

        let assign16290_e26756: f64 = (locals.var_tnom - 210.0);
        let assign16290_e26757: f64 = (locals.var_uds1_i * assign16290_e26756);
        let assign16290_e26759: f64 = (assign16290_e26757 / locals.var_tnom);
        let assign16290_e26760: f64 = (assign16290_e26759).abs();
        let assign16290_e26762: f64 = if assign16290_e26760 < 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard286 = assign16290_e26762;

        let (assign16300_e26779, assign16300_e26779_d_n4,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard286 != 0.0)) {
        let assign16300_e26773: f64 = (locals.var_uds1_i * locals.var_deltratio1);
        let assign16300_e26774: f64 = { let limited_exp_arg = assign16300_e26773; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign16300_e26776: f64 = (assign16300_e26774 - 1.0);
        let assign16300_e26777: f64 = (locals.var_uds_i * assign16300_e26776);
        (assign16300_e26777, (locals.var_uds_i * ({ let limited_exp_arg = assign16300_e26773; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_uds1_i * locals.var_deltratio1_dn4))),)
    } else {
        (locals.var_uds_t, locals.var_uds_t_dn4,)
    }
};
        locals.var_uds_t = assign16300_e26779;
        locals.var_uds_t_dn4 = assign16300_e26779_d_n4;

        let (assign16310_e26809, assign16310_e26809_d_n4,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard286 == 0.0)) {
        let assign16310_e26791: f64 = (locals.var_uds1_i * locals.var_deltratio1);
        let assign16310_e26792: f64 = { let limited_exp_arg = assign16310_e26791; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign16310_e26794: f64 = (assign16310_e26792 - 1.0);
        let assign16310_e26795: f64 = (locals.var_uds_i * assign16310_e26794);
        let assign16310_e26799: f64 = (locals.var_tnom - 210.0);
        let assign16310_e26800: f64 = (locals.var_uds1_i * assign16310_e26799);
        let assign16310_e26802: f64 = (assign16310_e26800 / locals.var_tnom);
        let assign16310_e26803: f64 = { let limited_exp_arg = assign16310_e26802; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign16310_e26805: f64 = (assign16310_e26803 - 1.0);
        let assign16310_e26806: f64 = (assign16310_e26805).abs();
        let assign16310_e26807: f64 = (assign16310_e26795 / assign16310_e26806);
        (assign16310_e26807, ((locals.var_uds_i * ({ let limited_exp_arg = assign16310_e26791; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_uds1_i * locals.var_deltratio1_dn4))) / assign16310_e26806),)
    } else {
        (locals.var_uds_t, locals.var_uds_t_dn4,)
    }
};
        locals.var_uds_t = assign16310_e26809;
        locals.var_uds_t_dn4 = assign16310_e26809_d_n4;

        let assign16320_e26813: f64 = (locals.var_tnom - 210.0);
        let assign16320_e26814: f64 = (locals.var_udd1_i * assign16320_e26813);
        let assign16320_e26816: f64 = (assign16320_e26814 / locals.var_tnom);
        let assign16320_e26817: f64 = (assign16320_e26816).abs();
        let assign16320_e26819: f64 = if assign16320_e26817 < 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard287 = assign16320_e26819;

        let (assign16330_e26836, assign16330_e26836_d_n4,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard287 != 0.0)) {
        let assign16330_e26830: f64 = (locals.var_udd1_i * locals.var_deltratio1);
        let assign16330_e26831: f64 = { let limited_exp_arg = assign16330_e26830; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign16330_e26833: f64 = (assign16330_e26831 - 1.0);
        let assign16330_e26834: f64 = (locals.var_udd_i * assign16330_e26833);
        (assign16330_e26834, (locals.var_udd_i * ({ let limited_exp_arg = assign16330_e26830; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_udd1_i * locals.var_deltratio1_dn4))),)
    } else {
        (locals.var_udd_t, locals.var_udd_t_dn4,)
    }
};
        locals.var_udd_t = assign16330_e26836;
        locals.var_udd_t_dn4 = assign16330_e26836_d_n4;

        let (assign16340_e26866, assign16340_e26866_d_n4,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard287 == 0.0)) {
        let assign16340_e26848: f64 = (locals.var_udd1_i * locals.var_deltratio1);
        let assign16340_e26849: f64 = { let limited_exp_arg = assign16340_e26848; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign16340_e26851: f64 = (assign16340_e26849 - 1.0);
        let assign16340_e26852: f64 = (locals.var_udd_i * assign16340_e26851);
        let assign16340_e26856: f64 = (locals.var_tnom - 210.0);
        let assign16340_e26857: f64 = (locals.var_udd1_i * assign16340_e26856);
        let assign16340_e26859: f64 = (assign16340_e26857 / locals.var_tnom);
        let assign16340_e26860: f64 = { let limited_exp_arg = assign16340_e26859; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign16340_e26862: f64 = (assign16340_e26860 - 1.0);
        let assign16340_e26863: f64 = (assign16340_e26862).abs();
        let assign16340_e26864: f64 = (assign16340_e26852 / assign16340_e26863);
        (assign16340_e26864, ((locals.var_udd_i * ({ let limited_exp_arg = assign16340_e26848; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_udd1_i * locals.var_deltratio1_dn4))) / assign16340_e26863),)
    } else {
        (locals.var_udd_t, locals.var_udd_t_dn4,)
    }
};
        locals.var_udd_t = assign16340_e26866;
        locals.var_udd_t_dn4 = assign16340_e26866_d_n4;

        let (assign16350_e26876, assign16350_e26876_d_n4,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) {
        let assign16350_e26874: f64 = (0.5 + locals.var_uds_t);
        (assign16350_e26874, locals.var_uds_t_dn4,)
    } else {
        (locals.var_udseff_t, locals.var_udseff_t_dn4,)
    }
};
        locals.var_udseff_t = assign16350_e26876;
        locals.var_udseff_t_dn4 = assign16350_e26876_d_n4;

        let (assign16360_e26886, assign16360_e26886_d_n4,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) {
        let assign16360_e26884: f64 = (0.5 + locals.var_udd_t);
        (assign16360_e26884, locals.var_udd_t_dn4,)
    } else {
        (locals.var_uddeff_t, locals.var_uddeff_t_dn4,)
    }
};
        locals.var_uddeff_t = assign16360_e26886;
        locals.var_uddeff_t_dn4 = assign16360_e26886_d_n4;

        let assign16370_e26889: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard288 = assign16370_e26889;

        let (assign16380_e26939, assign16380_e26939_d_n0, assign16380_e26939_d_n2, assign16380_e26939_d_n3, assign16380_e26939_d_n4, assign16380_e26939_d_n5, assign16380_e26939_d_n6, assign16380_e26939_d_n7, assign16380_e26939_d_n8, assign16380_e26939_d_n9, assign16380_e26939_d_n10, assign16380_e26939_d_n11, assign16380_e26939_d_n13, assign16380_e26939_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard288 != 0.0)) {
        let assign16380_e26899: f64 = (-locals.var_eu_i);
        let assign16380_e26903: f64 = (locals.var_eu1_i * locals.var_deltemp1);
        let assign16380_e26905: f64 = (-locals.var_eu_i);
        let assign16380_e26906: f64 = (assign16380_e26903 - assign16380_e26905);
        let assign16380_e26908: f64 = (assign16380_e26906 - 1e-6);
        let assign16380_e26911: f64 = (locals.var_eu1_i * locals.var_deltemp1);
        let assign16380_e26913: f64 = (-locals.var_eu_i);
        let assign16380_e26914: f64 = (assign16380_e26911 - assign16380_e26913);
        let assign16380_e26916: f64 = (assign16380_e26914 - 1e-6);
        let assign16380_e26919: f64 = (locals.var_eu1_i * locals.var_deltemp1);
        let assign16380_e26921: f64 = (-locals.var_eu_i);
        let assign16380_e26922: f64 = (assign16380_e26919 - assign16380_e26921);
        let assign16380_e26924: f64 = (assign16380_e26922 - 1e-6);
        let assign16380_e26925: f64 = (assign16380_e26916 * assign16380_e26924);
        let assign16380_e26928: f64 = (-locals.var_eu_i);
        let assign16380_e26929: f64 = (4.0 * assign16380_e26928);
        let assign16380_e26931: f64 = (assign16380_e26929 * 1e-6);
        let assign16380_e26932: f64 = (assign16380_e26925 - assign16380_e26931);
        let assign16380_e26933: f64 = (assign16380_e26932).sqrt();
        let assign16380_e26934: f64 = (assign16380_e26908 + assign16380_e26933);
        let assign16380_e26935: f64 = (0.5 * assign16380_e26934);
        let assign16380_e26936: f64 = (assign16380_e26899 + assign16380_e26935);
        let assign16380_e26937: f64 = (locals.var_eu_i + assign16380_e26936);
        (assign16380_e26937, (locals.var_eu_i_dn0 + ((-locals.var_eu_i_dn0) + (0.5 * ((-(-locals.var_eu_i_dn0)) + (((((-(-locals.var_eu_i_dn0)) * assign16380_e26924) + (assign16380_e26916 * (-(-locals.var_eu_i_dn0)))) - ((4.0 * (-locals.var_eu_i_dn0)) * 1e-6)) / (2.0 * assign16380_e26933)))))), (locals.var_eu_i_dn2 + ((-locals.var_eu_i_dn2) + (0.5 * ((-(-locals.var_eu_i_dn2)) + (((((-(-locals.var_eu_i_dn2)) * assign16380_e26924) + (assign16380_e26916 * (-(-locals.var_eu_i_dn2)))) - ((4.0 * (-locals.var_eu_i_dn2)) * 1e-6)) / (2.0 * assign16380_e26933)))))), (locals.var_eu_i_dn3 + ((-locals.var_eu_i_dn3) + (0.5 * ((-(-locals.var_eu_i_dn3)) + (((((-(-locals.var_eu_i_dn3)) * assign16380_e26924) + (assign16380_e26916 * (-(-locals.var_eu_i_dn3)))) - ((4.0 * (-locals.var_eu_i_dn3)) * 1e-6)) / (2.0 * assign16380_e26933)))))), (locals.var_eu_i_dn4 + ((-locals.var_eu_i_dn4) + (0.5 * (((locals.var_eu1_i * locals.var_deltemp1_dn4) - (-locals.var_eu_i_dn4)) + ((((((locals.var_eu1_i * locals.var_deltemp1_dn4) - (-locals.var_eu_i_dn4)) * assign16380_e26924) + (assign16380_e26916 * ((locals.var_eu1_i * locals.var_deltemp1_dn4) - (-locals.var_eu_i_dn4)))) - ((4.0 * (-locals.var_eu_i_dn4)) * 1e-6)) / (2.0 * assign16380_e26933)))))), (locals.var_eu_i_dn5 + ((-locals.var_eu_i_dn5) + (0.5 * ((-(-locals.var_eu_i_dn5)) + (((((-(-locals.var_eu_i_dn5)) * assign16380_e26924) + (assign16380_e26916 * (-(-locals.var_eu_i_dn5)))) - ((4.0 * (-locals.var_eu_i_dn5)) * 1e-6)) / (2.0 * assign16380_e26933)))))), (locals.var_eu_i_dn6 + ((-locals.var_eu_i_dn6) + (0.5 * ((-(-locals.var_eu_i_dn6)) + (((((-(-locals.var_eu_i_dn6)) * assign16380_e26924) + (assign16380_e26916 * (-(-locals.var_eu_i_dn6)))) - ((4.0 * (-locals.var_eu_i_dn6)) * 1e-6)) / (2.0 * assign16380_e26933)))))), (locals.var_eu_i_dn7 + ((-locals.var_eu_i_dn7) + (0.5 * ((-(-locals.var_eu_i_dn7)) + (((((-(-locals.var_eu_i_dn7)) * assign16380_e26924) + (assign16380_e26916 * (-(-locals.var_eu_i_dn7)))) - ((4.0 * (-locals.var_eu_i_dn7)) * 1e-6)) / (2.0 * assign16380_e26933)))))), (locals.var_eu_i_dn8 + ((-locals.var_eu_i_dn8) + (0.5 * ((-(-locals.var_eu_i_dn8)) + (((((-(-locals.var_eu_i_dn8)) * assign16380_e26924) + (assign16380_e26916 * (-(-locals.var_eu_i_dn8)))) - ((4.0 * (-locals.var_eu_i_dn8)) * 1e-6)) / (2.0 * assign16380_e26933)))))), (locals.var_eu_i_dn9 + ((-locals.var_eu_i_dn9) + (0.5 * ((-(-locals.var_eu_i_dn9)) + (((((-(-locals.var_eu_i_dn9)) * assign16380_e26924) + (assign16380_e26916 * (-(-locals.var_eu_i_dn9)))) - ((4.0 * (-locals.var_eu_i_dn9)) * 1e-6)) / (2.0 * assign16380_e26933)))))), (locals.var_eu_i_dn10 + ((-locals.var_eu_i_dn10) + (0.5 * ((-(-locals.var_eu_i_dn10)) + (((((-(-locals.var_eu_i_dn10)) * assign16380_e26924) + (assign16380_e26916 * (-(-locals.var_eu_i_dn10)))) - ((4.0 * (-locals.var_eu_i_dn10)) * 1e-6)) / (2.0 * assign16380_e26933)))))), (locals.var_eu_i_dn11 + ((-locals.var_eu_i_dn11) + (0.5 * ((-(-locals.var_eu_i_dn11)) + (((((-(-locals.var_eu_i_dn11)) * assign16380_e26924) + (assign16380_e26916 * (-(-locals.var_eu_i_dn11)))) - ((4.0 * (-locals.var_eu_i_dn11)) * 1e-6)) / (2.0 * assign16380_e26933)))))), (locals.var_eu_i_dn13 + ((-locals.var_eu_i_dn13) + (0.5 * ((-(-locals.var_eu_i_dn13)) + (((((-(-locals.var_eu_i_dn13)) * assign16380_e26924) + (assign16380_e26916 * (-(-locals.var_eu_i_dn13)))) - ((4.0 * (-locals.var_eu_i_dn13)) * 1e-6)) / (2.0 * assign16380_e26933)))))), (locals.var_eu_i_dn14 + ((-locals.var_eu_i_dn14) + (0.5 * ((-(-locals.var_eu_i_dn14)) + (((((-(-locals.var_eu_i_dn14)) * assign16380_e26924) + (assign16380_e26916 * (-(-locals.var_eu_i_dn14)))) - ((4.0 * (-locals.var_eu_i_dn14)) * 1e-6)) / (2.0 * assign16380_e26933)))))),)
    } else {
        (locals.var_eu_t, locals.var_eu_t_dn0, locals.var_eu_t_dn2, locals.var_eu_t_dn3, locals.var_eu_t_dn4, locals.var_eu_t_dn5, locals.var_eu_t_dn6, locals.var_eu_t_dn7, locals.var_eu_t_dn8, locals.var_eu_t_dn9, locals.var_eu_t_dn10, locals.var_eu_t_dn11, locals.var_eu_t_dn13, locals.var_eu_t_dn14,)
    }
};
        locals.var_eu_t = assign16380_e26939;
        locals.var_eu_t_dn0 = assign16380_e26939_d_n0;
        locals.var_eu_t_dn2 = assign16380_e26939_d_n2;
        locals.var_eu_t_dn3 = assign16380_e26939_d_n3;
        locals.var_eu_t_dn4 = assign16380_e26939_d_n4;
        locals.var_eu_t_dn5 = assign16380_e26939_d_n5;
        locals.var_eu_t_dn6 = assign16380_e26939_d_n6;
        locals.var_eu_t_dn7 = assign16380_e26939_d_n7;
        locals.var_eu_t_dn8 = assign16380_e26939_d_n8;
        locals.var_eu_t_dn9 = assign16380_e26939_d_n9;
        locals.var_eu_t_dn10 = assign16380_e26939_d_n10;
        locals.var_eu_t_dn11 = assign16380_e26939_d_n11;
        locals.var_eu_t_dn13 = assign16380_e26939_d_n13;
        locals.var_eu_t_dn14 = assign16380_e26939_d_n14;

    }

    pub(super) fn stamp_transient_block_50(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign16390_e27010, assign16390_e27010_d_n0, assign16390_e27010_d_n2, assign16390_e27010_d_n3, assign16390_e27010_d_n4, assign16390_e27010_d_n5, assign16390_e27010_d_n6, assign16390_e27010_d_n7, assign16390_e27010_d_n8, assign16390_e27010_d_n9, assign16390_e27010_d_n10, assign16390_e27010_d_n11, assign16390_e27010_d_n13, assign16390_e27010_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard288 != 0.0)) {
        let assign16390_e26949: f64 = (-locals.var_vsat_i);
        let assign16390_e26952: f64 = (-locals.var_at_i);
        let assign16390_e26954: f64 = (assign16390_e26952 * locals.var_deltemp);
        let assign16390_e26957: f64 = (p.p561 * locals.var_deltemp1);
        let assign16390_e26959: f64 = (assign16390_e26957 * locals.var_deltemp1);
        let assign16390_e26960: f64 = (assign16390_e26954 + assign16390_e26959);
        let assign16390_e26962: f64 = (-locals.var_vsat_i);
        let assign16390_e26963: f64 = (assign16390_e26960 - assign16390_e26962);
        let assign16390_e26965: f64 = (assign16390_e26963 - 1e-6);
        let assign16390_e26967: f64 = (-locals.var_at_i);
        let assign16390_e26969: f64 = (assign16390_e26967 * locals.var_deltemp);
        let assign16390_e26972: f64 = (p.p561 * locals.var_deltemp1);
        let assign16390_e26974: f64 = (assign16390_e26972 * locals.var_deltemp1);
        let assign16390_e26975: f64 = (assign16390_e26969 + assign16390_e26974);
        let assign16390_e26977: f64 = (-locals.var_vsat_i);
        let assign16390_e26978: f64 = (assign16390_e26975 - assign16390_e26977);
        let assign16390_e26980: f64 = (assign16390_e26978 - 1e-6);
        let assign16390_e26982: f64 = (-locals.var_at_i);
        let assign16390_e26984: f64 = (assign16390_e26982 * locals.var_deltemp);
        let assign16390_e26987: f64 = (p.p561 * locals.var_deltemp1);
        let assign16390_e26989: f64 = (assign16390_e26987 * locals.var_deltemp1);
        let assign16390_e26990: f64 = (assign16390_e26984 + assign16390_e26989);
        let assign16390_e26992: f64 = (-locals.var_vsat_i);
        let assign16390_e26993: f64 = (assign16390_e26990 - assign16390_e26992);
        let assign16390_e26995: f64 = (assign16390_e26993 - 1e-6);
        let assign16390_e26996: f64 = (assign16390_e26980 * assign16390_e26995);
        let assign16390_e26999: f64 = (-locals.var_vsat_i);
        let assign16390_e27000: f64 = (4.0 * assign16390_e26999);
        let assign16390_e27002: f64 = (assign16390_e27000 * 1e-6);
        let assign16390_e27003: f64 = (assign16390_e26996 - assign16390_e27002);
        let assign16390_e27004: f64 = (assign16390_e27003).sqrt();
        let assign16390_e27005: f64 = (assign16390_e26965 + assign16390_e27004);
        let assign16390_e27006: f64 = (0.5 * assign16390_e27005);
        let assign16390_e27007: f64 = (assign16390_e26949 + assign16390_e27006);
        let assign16390_e27008: f64 = (locals.var_vsat_i + assign16390_e27007);
        (assign16390_e27008, (locals.var_vsat_i_dn0 + ((-locals.var_vsat_i_dn0) + (0.5 * ((-(-locals.var_vsat_i_dn0)) + (((((-(-locals.var_vsat_i_dn0)) * assign16390_e26995) + (assign16390_e26980 * (-(-locals.var_vsat_i_dn0)))) - ((4.0 * (-locals.var_vsat_i_dn0)) * 1e-6)) / (2.0 * assign16390_e27004)))))), (locals.var_vsat_i_dn2 + ((-locals.var_vsat_i_dn2) + (0.5 * ((-(-locals.var_vsat_i_dn2)) + (((((-(-locals.var_vsat_i_dn2)) * assign16390_e26995) + (assign16390_e26980 * (-(-locals.var_vsat_i_dn2)))) - ((4.0 * (-locals.var_vsat_i_dn2)) * 1e-6)) / (2.0 * assign16390_e27004)))))), (locals.var_vsat_i_dn3 + ((-locals.var_vsat_i_dn3) + (0.5 * ((-(-locals.var_vsat_i_dn3)) + (((((-(-locals.var_vsat_i_dn3)) * assign16390_e26995) + (assign16390_e26980 * (-(-locals.var_vsat_i_dn3)))) - ((4.0 * (-locals.var_vsat_i_dn3)) * 1e-6)) / (2.0 * assign16390_e27004)))))), (locals.var_vsat_i_dn4 + ((-locals.var_vsat_i_dn4) + (0.5 * ((((assign16390_e26952 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16390_e26957 * locals.var_deltemp1_dn4))) - (-locals.var_vsat_i_dn4)) + (((((((assign16390_e26967 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16390_e26972 * locals.var_deltemp1_dn4))) - (-locals.var_vsat_i_dn4)) * assign16390_e26995) + (assign16390_e26980 * (((assign16390_e26982 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16390_e26987 * locals.var_deltemp1_dn4))) - (-locals.var_vsat_i_dn4)))) - ((4.0 * (-locals.var_vsat_i_dn4)) * 1e-6)) / (2.0 * assign16390_e27004)))))), (locals.var_vsat_i_dn5 + ((-locals.var_vsat_i_dn5) + (0.5 * ((-(-locals.var_vsat_i_dn5)) + (((((-(-locals.var_vsat_i_dn5)) * assign16390_e26995) + (assign16390_e26980 * (-(-locals.var_vsat_i_dn5)))) - ((4.0 * (-locals.var_vsat_i_dn5)) * 1e-6)) / (2.0 * assign16390_e27004)))))), (locals.var_vsat_i_dn6 + ((-locals.var_vsat_i_dn6) + (0.5 * ((-(-locals.var_vsat_i_dn6)) + (((((-(-locals.var_vsat_i_dn6)) * assign16390_e26995) + (assign16390_e26980 * (-(-locals.var_vsat_i_dn6)))) - ((4.0 * (-locals.var_vsat_i_dn6)) * 1e-6)) / (2.0 * assign16390_e27004)))))), (locals.var_vsat_i_dn7 + ((-locals.var_vsat_i_dn7) + (0.5 * ((-(-locals.var_vsat_i_dn7)) + (((((-(-locals.var_vsat_i_dn7)) * assign16390_e26995) + (assign16390_e26980 * (-(-locals.var_vsat_i_dn7)))) - ((4.0 * (-locals.var_vsat_i_dn7)) * 1e-6)) / (2.0 * assign16390_e27004)))))), (locals.var_vsat_i_dn8 + ((-locals.var_vsat_i_dn8) + (0.5 * ((-(-locals.var_vsat_i_dn8)) + (((((-(-locals.var_vsat_i_dn8)) * assign16390_e26995) + (assign16390_e26980 * (-(-locals.var_vsat_i_dn8)))) - ((4.0 * (-locals.var_vsat_i_dn8)) * 1e-6)) / (2.0 * assign16390_e27004)))))), (locals.var_vsat_i_dn9 + ((-locals.var_vsat_i_dn9) + (0.5 * ((-(-locals.var_vsat_i_dn9)) + (((((-(-locals.var_vsat_i_dn9)) * assign16390_e26995) + (assign16390_e26980 * (-(-locals.var_vsat_i_dn9)))) - ((4.0 * (-locals.var_vsat_i_dn9)) * 1e-6)) / (2.0 * assign16390_e27004)))))), (locals.var_vsat_i_dn10 + ((-locals.var_vsat_i_dn10) + (0.5 * ((-(-locals.var_vsat_i_dn10)) + (((((-(-locals.var_vsat_i_dn10)) * assign16390_e26995) + (assign16390_e26980 * (-(-locals.var_vsat_i_dn10)))) - ((4.0 * (-locals.var_vsat_i_dn10)) * 1e-6)) / (2.0 * assign16390_e27004)))))), (locals.var_vsat_i_dn11 + ((-locals.var_vsat_i_dn11) + (0.5 * ((-(-locals.var_vsat_i_dn11)) + (((((-(-locals.var_vsat_i_dn11)) * assign16390_e26995) + (assign16390_e26980 * (-(-locals.var_vsat_i_dn11)))) - ((4.0 * (-locals.var_vsat_i_dn11)) * 1e-6)) / (2.0 * assign16390_e27004)))))), (locals.var_vsat_i_dn13 + ((-locals.var_vsat_i_dn13) + (0.5 * ((-(-locals.var_vsat_i_dn13)) + (((((-(-locals.var_vsat_i_dn13)) * assign16390_e26995) + (assign16390_e26980 * (-(-locals.var_vsat_i_dn13)))) - ((4.0 * (-locals.var_vsat_i_dn13)) * 1e-6)) / (2.0 * assign16390_e27004)))))), (locals.var_vsat_i_dn14 + ((-locals.var_vsat_i_dn14) + (0.5 * ((-(-locals.var_vsat_i_dn14)) + (((((-(-locals.var_vsat_i_dn14)) * assign16390_e26995) + (assign16390_e26980 * (-(-locals.var_vsat_i_dn14)))) - ((4.0 * (-locals.var_vsat_i_dn14)) * 1e-6)) / (2.0 * assign16390_e27004)))))),)
    } else {
        (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14,)
    }
};
        locals.var_vsat_t = assign16390_e27010;
        locals.var_vsat_t_dn0 = assign16390_e27010_d_n0;
        locals.var_vsat_t_dn2 = assign16390_e27010_d_n2;
        locals.var_vsat_t_dn3 = assign16390_e27010_d_n3;
        locals.var_vsat_t_dn4 = assign16390_e27010_d_n4;
        locals.var_vsat_t_dn5 = assign16390_e27010_d_n5;
        locals.var_vsat_t_dn6 = assign16390_e27010_d_n6;
        locals.var_vsat_t_dn7 = assign16390_e27010_d_n7;
        locals.var_vsat_t_dn8 = assign16390_e27010_d_n8;
        locals.var_vsat_t_dn9 = assign16390_e27010_d_n9;
        locals.var_vsat_t_dn10 = assign16390_e27010_d_n10;
        locals.var_vsat_t_dn11 = assign16390_e27010_d_n11;
        locals.var_vsat_t_dn13 = assign16390_e27010_d_n13;
        locals.var_vsat_t_dn14 = assign16390_e27010_d_n14;

        let assign16400_e27013: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard289 = assign16400_e27013;

        let (assign16410_e27086, assign16410_e27086_d_n4,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 != 0.0)) {
        let assign16410_e27025: f64 = (-locals.var_vsatr_i);
        let assign16410_e27028: f64 = (-locals.var_atr_i);
        let assign16410_e27030: f64 = (assign16410_e27028 * locals.var_deltemp);
        let assign16410_e27033: f64 = (p.p561 * locals.var_deltemp1);
        let assign16410_e27035: f64 = (assign16410_e27033 * locals.var_deltemp1);
        let assign16410_e27036: f64 = (assign16410_e27030 + assign16410_e27035);
        let assign16410_e27038: f64 = (-locals.var_vsatr_i);
        let assign16410_e27039: f64 = (assign16410_e27036 - assign16410_e27038);
        let assign16410_e27041: f64 = (assign16410_e27039 - 1e-6);
        let assign16410_e27043: f64 = (-locals.var_atr_i);
        let assign16410_e27045: f64 = (assign16410_e27043 * locals.var_deltemp);
        let assign16410_e27048: f64 = (p.p561 * locals.var_deltemp1);
        let assign16410_e27050: f64 = (assign16410_e27048 * locals.var_deltemp1);
        let assign16410_e27051: f64 = (assign16410_e27045 + assign16410_e27050);
        let assign16410_e27053: f64 = (-locals.var_vsatr_i);
        let assign16410_e27054: f64 = (assign16410_e27051 - assign16410_e27053);
        let assign16410_e27056: f64 = (assign16410_e27054 - 1e-6);
        let assign16410_e27058: f64 = (-locals.var_atr_i);
        let assign16410_e27060: f64 = (assign16410_e27058 * locals.var_deltemp);
        let assign16410_e27063: f64 = (p.p561 * locals.var_deltemp1);
        let assign16410_e27065: f64 = (assign16410_e27063 * locals.var_deltemp1);
        let assign16410_e27066: f64 = (assign16410_e27060 + assign16410_e27065);
        let assign16410_e27068: f64 = (-locals.var_vsatr_i);
        let assign16410_e27069: f64 = (assign16410_e27066 - assign16410_e27068);
        let assign16410_e27071: f64 = (assign16410_e27069 - 1e-6);
        let assign16410_e27072: f64 = (assign16410_e27056 * assign16410_e27071);
        let assign16410_e27075: f64 = (-locals.var_vsatr_i);
        let assign16410_e27076: f64 = (4.0 * assign16410_e27075);
        let assign16410_e27078: f64 = (assign16410_e27076 * 1e-6);
        let assign16410_e27079: f64 = (assign16410_e27072 - assign16410_e27078);
        let assign16410_e27080: f64 = (assign16410_e27079).sqrt();
        let assign16410_e27081: f64 = (assign16410_e27041 + assign16410_e27080);
        let assign16410_e27082: f64 = (0.5 * assign16410_e27081);
        let assign16410_e27083: f64 = (assign16410_e27025 + assign16410_e27082);
        let assign16410_e27084: f64 = (locals.var_vsatr_i + assign16410_e27083);
        (assign16410_e27084, (0.5 * (((assign16410_e27028 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16410_e27033 * locals.var_deltemp1_dn4))) + (((((assign16410_e27043 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16410_e27048 * locals.var_deltemp1_dn4))) * assign16410_e27071) + (assign16410_e27056 * ((assign16410_e27058 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16410_e27063 * locals.var_deltemp1_dn4))))) / (2.0 * assign16410_e27080)))),)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn4,)
    }
};
        locals.var_vsatr_t = assign16410_e27086;
        locals.var_vsatr_t_dn4 = assign16410_e27086_d_n4;

        let assign16420_e27089: f64 = if locals.var_vsatr_t < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard290 = assign16420_e27089;

        let (assign16430_e27103, assign16430_e27103_d_n4,) = {
    if (((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard288 != 0.0)) && (locals.var_guard289 != 0.0)) && (locals.var_guard290 != 0.0)) {
        (1000.0, 0.0,)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn4,)
    }
};
        locals.var_vsatr_t = assign16430_e27103;
        locals.var_vsatr_t_dn4 = assign16430_e27103_d_n4;

        let (assign16440_e27174, assign16440_e27174_d_n0, assign16440_e27174_d_n2, assign16440_e27174_d_n3, assign16440_e27174_d_n4, assign16440_e27174_d_n5, assign16440_e27174_d_n6, assign16440_e27174_d_n7, assign16440_e27174_d_n8, assign16440_e27174_d_n9, assign16440_e27174_d_n10, assign16440_e27174_d_n11, assign16440_e27174_d_n13, assign16440_e27174_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard288 != 0.0)) {
        let assign16440_e27113: f64 = (-locals.var_vsat1_i);
        let assign16440_e27116: f64 = (-locals.var_at_i);
        let assign16440_e27118: f64 = (assign16440_e27116 * locals.var_deltemp);
        let assign16440_e27121: f64 = (p.p561 * locals.var_deltemp1);
        let assign16440_e27123: f64 = (assign16440_e27121 * locals.var_deltemp1);
        let assign16440_e27124: f64 = (assign16440_e27118 + assign16440_e27123);
        let assign16440_e27126: f64 = (-locals.var_vsat1_i);
        let assign16440_e27127: f64 = (assign16440_e27124 - assign16440_e27126);
        let assign16440_e27129: f64 = (assign16440_e27127 - 1e-6);
        let assign16440_e27131: f64 = (-locals.var_at_i);
        let assign16440_e27133: f64 = (assign16440_e27131 * locals.var_deltemp);
        let assign16440_e27136: f64 = (p.p561 * locals.var_deltemp1);
        let assign16440_e27138: f64 = (assign16440_e27136 * locals.var_deltemp1);
        let assign16440_e27139: f64 = (assign16440_e27133 + assign16440_e27138);
        let assign16440_e27141: f64 = (-locals.var_vsat1_i);
        let assign16440_e27142: f64 = (assign16440_e27139 - assign16440_e27141);
        let assign16440_e27144: f64 = (assign16440_e27142 - 1e-6);
        let assign16440_e27146: f64 = (-locals.var_at_i);
        let assign16440_e27148: f64 = (assign16440_e27146 * locals.var_deltemp);
        let assign16440_e27151: f64 = (p.p561 * locals.var_deltemp1);
        let assign16440_e27153: f64 = (assign16440_e27151 * locals.var_deltemp1);
        let assign16440_e27154: f64 = (assign16440_e27148 + assign16440_e27153);
        let assign16440_e27156: f64 = (-locals.var_vsat1_i);
        let assign16440_e27157: f64 = (assign16440_e27154 - assign16440_e27156);
        let assign16440_e27159: f64 = (assign16440_e27157 - 1e-6);
        let assign16440_e27160: f64 = (assign16440_e27144 * assign16440_e27159);
        let assign16440_e27163: f64 = (-locals.var_vsat1_i);
        let assign16440_e27164: f64 = (4.0 * assign16440_e27163);
        let assign16440_e27166: f64 = (assign16440_e27164 * 1e-6);
        let assign16440_e27167: f64 = (assign16440_e27160 - assign16440_e27166);
        let assign16440_e27168: f64 = (assign16440_e27167).sqrt();
        let assign16440_e27169: f64 = (assign16440_e27129 + assign16440_e27168);
        let assign16440_e27170: f64 = (0.5 * assign16440_e27169);
        let assign16440_e27171: f64 = (assign16440_e27113 + assign16440_e27170);
        let assign16440_e27172: f64 = (locals.var_vsat1_i + assign16440_e27171);
        (assign16440_e27172, (locals.var_vsat1_i_dn0 + ((-locals.var_vsat1_i_dn0) + (0.5 * ((-(-locals.var_vsat1_i_dn0)) + (((((-(-locals.var_vsat1_i_dn0)) * assign16440_e27159) + (assign16440_e27144 * (-(-locals.var_vsat1_i_dn0)))) - ((4.0 * (-locals.var_vsat1_i_dn0)) * 1e-6)) / (2.0 * assign16440_e27168)))))), (locals.var_vsat1_i_dn2 + ((-locals.var_vsat1_i_dn2) + (0.5 * ((-(-locals.var_vsat1_i_dn2)) + (((((-(-locals.var_vsat1_i_dn2)) * assign16440_e27159) + (assign16440_e27144 * (-(-locals.var_vsat1_i_dn2)))) - ((4.0 * (-locals.var_vsat1_i_dn2)) * 1e-6)) / (2.0 * assign16440_e27168)))))), (locals.var_vsat1_i_dn3 + ((-locals.var_vsat1_i_dn3) + (0.5 * ((-(-locals.var_vsat1_i_dn3)) + (((((-(-locals.var_vsat1_i_dn3)) * assign16440_e27159) + (assign16440_e27144 * (-(-locals.var_vsat1_i_dn3)))) - ((4.0 * (-locals.var_vsat1_i_dn3)) * 1e-6)) / (2.0 * assign16440_e27168)))))), (locals.var_vsat1_i_dn4 + ((-locals.var_vsat1_i_dn4) + (0.5 * ((((assign16440_e27116 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16440_e27121 * locals.var_deltemp1_dn4))) - (-locals.var_vsat1_i_dn4)) + (((((((assign16440_e27131 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16440_e27136 * locals.var_deltemp1_dn4))) - (-locals.var_vsat1_i_dn4)) * assign16440_e27159) + (assign16440_e27144 * (((assign16440_e27146 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16440_e27151 * locals.var_deltemp1_dn4))) - (-locals.var_vsat1_i_dn4)))) - ((4.0 * (-locals.var_vsat1_i_dn4)) * 1e-6)) / (2.0 * assign16440_e27168)))))), (locals.var_vsat1_i_dn5 + ((-locals.var_vsat1_i_dn5) + (0.5 * ((-(-locals.var_vsat1_i_dn5)) + (((((-(-locals.var_vsat1_i_dn5)) * assign16440_e27159) + (assign16440_e27144 * (-(-locals.var_vsat1_i_dn5)))) - ((4.0 * (-locals.var_vsat1_i_dn5)) * 1e-6)) / (2.0 * assign16440_e27168)))))), (locals.var_vsat1_i_dn6 + ((-locals.var_vsat1_i_dn6) + (0.5 * ((-(-locals.var_vsat1_i_dn6)) + (((((-(-locals.var_vsat1_i_dn6)) * assign16440_e27159) + (assign16440_e27144 * (-(-locals.var_vsat1_i_dn6)))) - ((4.0 * (-locals.var_vsat1_i_dn6)) * 1e-6)) / (2.0 * assign16440_e27168)))))), (locals.var_vsat1_i_dn7 + ((-locals.var_vsat1_i_dn7) + (0.5 * ((-(-locals.var_vsat1_i_dn7)) + (((((-(-locals.var_vsat1_i_dn7)) * assign16440_e27159) + (assign16440_e27144 * (-(-locals.var_vsat1_i_dn7)))) - ((4.0 * (-locals.var_vsat1_i_dn7)) * 1e-6)) / (2.0 * assign16440_e27168)))))), (locals.var_vsat1_i_dn8 + ((-locals.var_vsat1_i_dn8) + (0.5 * ((-(-locals.var_vsat1_i_dn8)) + (((((-(-locals.var_vsat1_i_dn8)) * assign16440_e27159) + (assign16440_e27144 * (-(-locals.var_vsat1_i_dn8)))) - ((4.0 * (-locals.var_vsat1_i_dn8)) * 1e-6)) / (2.0 * assign16440_e27168)))))), (locals.var_vsat1_i_dn9 + ((-locals.var_vsat1_i_dn9) + (0.5 * ((-(-locals.var_vsat1_i_dn9)) + (((((-(-locals.var_vsat1_i_dn9)) * assign16440_e27159) + (assign16440_e27144 * (-(-locals.var_vsat1_i_dn9)))) - ((4.0 * (-locals.var_vsat1_i_dn9)) * 1e-6)) / (2.0 * assign16440_e27168)))))), (locals.var_vsat1_i_dn10 + ((-locals.var_vsat1_i_dn10) + (0.5 * ((-(-locals.var_vsat1_i_dn10)) + (((((-(-locals.var_vsat1_i_dn10)) * assign16440_e27159) + (assign16440_e27144 * (-(-locals.var_vsat1_i_dn10)))) - ((4.0 * (-locals.var_vsat1_i_dn10)) * 1e-6)) / (2.0 * assign16440_e27168)))))), (locals.var_vsat1_i_dn11 + ((-locals.var_vsat1_i_dn11) + (0.5 * ((-(-locals.var_vsat1_i_dn11)) + (((((-(-locals.var_vsat1_i_dn11)) * assign16440_e27159) + (assign16440_e27144 * (-(-locals.var_vsat1_i_dn11)))) - ((4.0 * (-locals.var_vsat1_i_dn11)) * 1e-6)) / (2.0 * assign16440_e27168)))))), (locals.var_vsat1_i_dn13 + ((-locals.var_vsat1_i_dn13) + (0.5 * ((-(-locals.var_vsat1_i_dn13)) + (((((-(-locals.var_vsat1_i_dn13)) * assign16440_e27159) + (assign16440_e27144 * (-(-locals.var_vsat1_i_dn13)))) - ((4.0 * (-locals.var_vsat1_i_dn13)) * 1e-6)) / (2.0 * assign16440_e27168)))))), (locals.var_vsat1_i_dn14 + ((-locals.var_vsat1_i_dn14) + (0.5 * ((-(-locals.var_vsat1_i_dn14)) + (((((-(-locals.var_vsat1_i_dn14)) * assign16440_e27159) + (assign16440_e27144 * (-(-locals.var_vsat1_i_dn14)))) - ((4.0 * (-locals.var_vsat1_i_dn14)) * 1e-6)) / (2.0 * assign16440_e27168)))))),)
    } else {
        (locals.var_vsat1_t, locals.var_vsat1_t_dn0, locals.var_vsat1_t_dn2, locals.var_vsat1_t_dn3, locals.var_vsat1_t_dn4, locals.var_vsat1_t_dn5, locals.var_vsat1_t_dn6, locals.var_vsat1_t_dn7, locals.var_vsat1_t_dn8, locals.var_vsat1_t_dn9, locals.var_vsat1_t_dn10, locals.var_vsat1_t_dn11, locals.var_vsat1_t_dn13, locals.var_vsat1_t_dn14,)
    }
};
        locals.var_vsat1_t = assign16440_e27174;
        locals.var_vsat1_t_dn0 = assign16440_e27174_d_n0;
        locals.var_vsat1_t_dn2 = assign16440_e27174_d_n2;
        locals.var_vsat1_t_dn3 = assign16440_e27174_d_n3;
        locals.var_vsat1_t_dn4 = assign16440_e27174_d_n4;
        locals.var_vsat1_t_dn5 = assign16440_e27174_d_n5;
        locals.var_vsat1_t_dn6 = assign16440_e27174_d_n6;
        locals.var_vsat1_t_dn7 = assign16440_e27174_d_n7;
        locals.var_vsat1_t_dn8 = assign16440_e27174_d_n8;
        locals.var_vsat1_t_dn9 = assign16440_e27174_d_n9;
        locals.var_vsat1_t_dn10 = assign16440_e27174_d_n10;
        locals.var_vsat1_t_dn11 = assign16440_e27174_d_n11;
        locals.var_vsat1_t_dn13 = assign16440_e27174_d_n13;
        locals.var_vsat1_t_dn14 = assign16440_e27174_d_n14;

        let assign16450_e27177: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard291 = assign16450_e27177;

        let (assign16460_e27250, assign16460_e27250_d_n0, assign16460_e27250_d_n2, assign16460_e27250_d_n3, assign16460_e27250_d_n4, assign16460_e27250_d_n5, assign16460_e27250_d_n6, assign16460_e27250_d_n7, assign16460_e27250_d_n8, assign16460_e27250_d_n9, assign16460_e27250_d_n10, assign16460_e27250_d_n11, assign16460_e27250_d_n13, assign16460_e27250_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard288 != 0.0)) && (locals.var_guard291 != 0.0)) {
        let assign16460_e27189: f64 = (-locals.var_vsat1r_i);
        let assign16460_e27192: f64 = (-locals.var_at_i);
        let assign16460_e27194: f64 = (assign16460_e27192 * locals.var_deltemp);
        let assign16460_e27197: f64 = (p.p561 * locals.var_deltemp1);
        let assign16460_e27199: f64 = (assign16460_e27197 * locals.var_deltemp1);
        let assign16460_e27200: f64 = (assign16460_e27194 + assign16460_e27199);
        let assign16460_e27202: f64 = (-locals.var_vsat1r_i);
        let assign16460_e27203: f64 = (assign16460_e27200 - assign16460_e27202);
        let assign16460_e27205: f64 = (assign16460_e27203 - 1e-6);
        let assign16460_e27207: f64 = (-locals.var_at_i);
        let assign16460_e27209: f64 = (assign16460_e27207 * locals.var_deltemp);
        let assign16460_e27212: f64 = (p.p561 * locals.var_deltemp1);
        let assign16460_e27214: f64 = (assign16460_e27212 * locals.var_deltemp1);
        let assign16460_e27215: f64 = (assign16460_e27209 + assign16460_e27214);
        let assign16460_e27217: f64 = (-locals.var_vsat1r_i);
        let assign16460_e27218: f64 = (assign16460_e27215 - assign16460_e27217);
        let assign16460_e27220: f64 = (assign16460_e27218 - 1e-6);
        let assign16460_e27222: f64 = (-locals.var_at_i);
        let assign16460_e27224: f64 = (assign16460_e27222 * locals.var_deltemp);
        let assign16460_e27227: f64 = (p.p561 * locals.var_deltemp1);
        let assign16460_e27229: f64 = (assign16460_e27227 * locals.var_deltemp1);
        let assign16460_e27230: f64 = (assign16460_e27224 + assign16460_e27229);
        let assign16460_e27232: f64 = (-locals.var_vsat1r_i);
        let assign16460_e27233: f64 = (assign16460_e27230 - assign16460_e27232);
        let assign16460_e27235: f64 = (assign16460_e27233 - 1e-6);
        let assign16460_e27236: f64 = (assign16460_e27220 * assign16460_e27235);
        let assign16460_e27239: f64 = (-locals.var_vsat1r_i);
        let assign16460_e27240: f64 = (4.0 * assign16460_e27239);
        let assign16460_e27242: f64 = (assign16460_e27240 * 1e-6);
        let assign16460_e27243: f64 = (assign16460_e27236 - assign16460_e27242);
        let assign16460_e27244: f64 = (assign16460_e27243).sqrt();
        let assign16460_e27245: f64 = (assign16460_e27205 + assign16460_e27244);
        let assign16460_e27246: f64 = (0.5 * assign16460_e27245);
        let assign16460_e27247: f64 = (assign16460_e27189 + assign16460_e27246);
        let assign16460_e27248: f64 = (locals.var_vsat1r_i + assign16460_e27247);
        (assign16460_e27248, (locals.var_vsat1r_i_dn0 + ((-locals.var_vsat1r_i_dn0) + (0.5 * ((-(-locals.var_vsat1r_i_dn0)) + (((((-(-locals.var_vsat1r_i_dn0)) * assign16460_e27235) + (assign16460_e27220 * (-(-locals.var_vsat1r_i_dn0)))) - ((4.0 * (-locals.var_vsat1r_i_dn0)) * 1e-6)) / (2.0 * assign16460_e27244)))))), (locals.var_vsat1r_i_dn2 + ((-locals.var_vsat1r_i_dn2) + (0.5 * ((-(-locals.var_vsat1r_i_dn2)) + (((((-(-locals.var_vsat1r_i_dn2)) * assign16460_e27235) + (assign16460_e27220 * (-(-locals.var_vsat1r_i_dn2)))) - ((4.0 * (-locals.var_vsat1r_i_dn2)) * 1e-6)) / (2.0 * assign16460_e27244)))))), (locals.var_vsat1r_i_dn3 + ((-locals.var_vsat1r_i_dn3) + (0.5 * ((-(-locals.var_vsat1r_i_dn3)) + (((((-(-locals.var_vsat1r_i_dn3)) * assign16460_e27235) + (assign16460_e27220 * (-(-locals.var_vsat1r_i_dn3)))) - ((4.0 * (-locals.var_vsat1r_i_dn3)) * 1e-6)) / (2.0 * assign16460_e27244)))))), (locals.var_vsat1r_i_dn4 + ((-locals.var_vsat1r_i_dn4) + (0.5 * ((((assign16460_e27192 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16460_e27197 * locals.var_deltemp1_dn4))) - (-locals.var_vsat1r_i_dn4)) + (((((((assign16460_e27207 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16460_e27212 * locals.var_deltemp1_dn4))) - (-locals.var_vsat1r_i_dn4)) * assign16460_e27235) + (assign16460_e27220 * (((assign16460_e27222 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16460_e27227 * locals.var_deltemp1_dn4))) - (-locals.var_vsat1r_i_dn4)))) - ((4.0 * (-locals.var_vsat1r_i_dn4)) * 1e-6)) / (2.0 * assign16460_e27244)))))), (locals.var_vsat1r_i_dn5 + ((-locals.var_vsat1r_i_dn5) + (0.5 * ((-(-locals.var_vsat1r_i_dn5)) + (((((-(-locals.var_vsat1r_i_dn5)) * assign16460_e27235) + (assign16460_e27220 * (-(-locals.var_vsat1r_i_dn5)))) - ((4.0 * (-locals.var_vsat1r_i_dn5)) * 1e-6)) / (2.0 * assign16460_e27244)))))), (locals.var_vsat1r_i_dn6 + ((-locals.var_vsat1r_i_dn6) + (0.5 * ((-(-locals.var_vsat1r_i_dn6)) + (((((-(-locals.var_vsat1r_i_dn6)) * assign16460_e27235) + (assign16460_e27220 * (-(-locals.var_vsat1r_i_dn6)))) - ((4.0 * (-locals.var_vsat1r_i_dn6)) * 1e-6)) / (2.0 * assign16460_e27244)))))), (locals.var_vsat1r_i_dn7 + ((-locals.var_vsat1r_i_dn7) + (0.5 * ((-(-locals.var_vsat1r_i_dn7)) + (((((-(-locals.var_vsat1r_i_dn7)) * assign16460_e27235) + (assign16460_e27220 * (-(-locals.var_vsat1r_i_dn7)))) - ((4.0 * (-locals.var_vsat1r_i_dn7)) * 1e-6)) / (2.0 * assign16460_e27244)))))), (locals.var_vsat1r_i_dn8 + ((-locals.var_vsat1r_i_dn8) + (0.5 * ((-(-locals.var_vsat1r_i_dn8)) + (((((-(-locals.var_vsat1r_i_dn8)) * assign16460_e27235) + (assign16460_e27220 * (-(-locals.var_vsat1r_i_dn8)))) - ((4.0 * (-locals.var_vsat1r_i_dn8)) * 1e-6)) / (2.0 * assign16460_e27244)))))), (locals.var_vsat1r_i_dn9 + ((-locals.var_vsat1r_i_dn9) + (0.5 * ((-(-locals.var_vsat1r_i_dn9)) + (((((-(-locals.var_vsat1r_i_dn9)) * assign16460_e27235) + (assign16460_e27220 * (-(-locals.var_vsat1r_i_dn9)))) - ((4.0 * (-locals.var_vsat1r_i_dn9)) * 1e-6)) / (2.0 * assign16460_e27244)))))), (locals.var_vsat1r_i_dn10 + ((-locals.var_vsat1r_i_dn10) + (0.5 * ((-(-locals.var_vsat1r_i_dn10)) + (((((-(-locals.var_vsat1r_i_dn10)) * assign16460_e27235) + (assign16460_e27220 * (-(-locals.var_vsat1r_i_dn10)))) - ((4.0 * (-locals.var_vsat1r_i_dn10)) * 1e-6)) / (2.0 * assign16460_e27244)))))), (locals.var_vsat1r_i_dn11 + ((-locals.var_vsat1r_i_dn11) + (0.5 * ((-(-locals.var_vsat1r_i_dn11)) + (((((-(-locals.var_vsat1r_i_dn11)) * assign16460_e27235) + (assign16460_e27220 * (-(-locals.var_vsat1r_i_dn11)))) - ((4.0 * (-locals.var_vsat1r_i_dn11)) * 1e-6)) / (2.0 * assign16460_e27244)))))), (locals.var_vsat1r_i_dn13 + ((-locals.var_vsat1r_i_dn13) + (0.5 * ((-(-locals.var_vsat1r_i_dn13)) + (((((-(-locals.var_vsat1r_i_dn13)) * assign16460_e27235) + (assign16460_e27220 * (-(-locals.var_vsat1r_i_dn13)))) - ((4.0 * (-locals.var_vsat1r_i_dn13)) * 1e-6)) / (2.0 * assign16460_e27244)))))), (locals.var_vsat1r_i_dn14 + ((-locals.var_vsat1r_i_dn14) + (0.5 * ((-(-locals.var_vsat1r_i_dn14)) + (((((-(-locals.var_vsat1r_i_dn14)) * assign16460_e27235) + (assign16460_e27220 * (-(-locals.var_vsat1r_i_dn14)))) - ((4.0 * (-locals.var_vsat1r_i_dn14)) * 1e-6)) / (2.0 * assign16460_e27244)))))),)
    } else {
        (locals.var_vsat1r_t, locals.var_vsat1r_t_dn0, locals.var_vsat1r_t_dn2, locals.var_vsat1r_t_dn3, locals.var_vsat1r_t_dn4, locals.var_vsat1r_t_dn5, locals.var_vsat1r_t_dn6, locals.var_vsat1r_t_dn7, locals.var_vsat1r_t_dn8, locals.var_vsat1r_t_dn9, locals.var_vsat1r_t_dn10, locals.var_vsat1r_t_dn11, locals.var_vsat1r_t_dn13, locals.var_vsat1r_t_dn14,)
    }
};
        locals.var_vsat1r_t = assign16460_e27250;
        locals.var_vsat1r_t_dn0 = assign16460_e27250_d_n0;
        locals.var_vsat1r_t_dn2 = assign16460_e27250_d_n2;
        locals.var_vsat1r_t_dn3 = assign16460_e27250_d_n3;
        locals.var_vsat1r_t_dn4 = assign16460_e27250_d_n4;
        locals.var_vsat1r_t_dn5 = assign16460_e27250_d_n5;
        locals.var_vsat1r_t_dn6 = assign16460_e27250_d_n6;
        locals.var_vsat1r_t_dn7 = assign16460_e27250_d_n7;
        locals.var_vsat1r_t_dn8 = assign16460_e27250_d_n8;
        locals.var_vsat1r_t_dn9 = assign16460_e27250_d_n9;
        locals.var_vsat1r_t_dn10 = assign16460_e27250_d_n10;
        locals.var_vsat1r_t_dn11 = assign16460_e27250_d_n11;
        locals.var_vsat1r_t_dn13 = assign16460_e27250_d_n13;
        locals.var_vsat1r_t_dn14 = assign16460_e27250_d_n14;

        let assign16470_e27253: f64 = if locals.var_vsat1r_t < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard292 = assign16470_e27253;

        let (assign16480_e27267, assign16480_e27267_d_n0, assign16480_e27267_d_n2, assign16480_e27267_d_n3, assign16480_e27267_d_n4, assign16480_e27267_d_n5, assign16480_e27267_d_n6, assign16480_e27267_d_n7, assign16480_e27267_d_n8, assign16480_e27267_d_n9, assign16480_e27267_d_n10, assign16480_e27267_d_n11, assign16480_e27267_d_n13, assign16480_e27267_d_n14,) = {
    if (((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard288 != 0.0)) && (locals.var_guard291 != 0.0)) && (locals.var_guard292 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsat1r_t, locals.var_vsat1r_t_dn0, locals.var_vsat1r_t_dn2, locals.var_vsat1r_t_dn3, locals.var_vsat1r_t_dn4, locals.var_vsat1r_t_dn5, locals.var_vsat1r_t_dn6, locals.var_vsat1r_t_dn7, locals.var_vsat1r_t_dn8, locals.var_vsat1r_t_dn9, locals.var_vsat1r_t_dn10, locals.var_vsat1r_t_dn11, locals.var_vsat1r_t_dn13, locals.var_vsat1r_t_dn14,)
    }
};
        locals.var_vsat1r_t = assign16480_e27267;
        locals.var_vsat1r_t_dn0 = assign16480_e27267_d_n0;
        locals.var_vsat1r_t_dn2 = assign16480_e27267_d_n2;
        locals.var_vsat1r_t_dn3 = assign16480_e27267_d_n3;
        locals.var_vsat1r_t_dn4 = assign16480_e27267_d_n4;
        locals.var_vsat1r_t_dn5 = assign16480_e27267_d_n5;
        locals.var_vsat1r_t_dn6 = assign16480_e27267_d_n6;
        locals.var_vsat1r_t_dn7 = assign16480_e27267_d_n7;
        locals.var_vsat1r_t_dn8 = assign16480_e27267_d_n8;
        locals.var_vsat1r_t_dn9 = assign16480_e27267_d_n9;
        locals.var_vsat1r_t_dn10 = assign16480_e27267_d_n10;
        locals.var_vsat1r_t_dn11 = assign16480_e27267_d_n11;
        locals.var_vsat1r_t_dn13 = assign16480_e27267_d_n13;
        locals.var_vsat1r_t_dn14 = assign16480_e27267_d_n14;

        let (assign16490_e27338, assign16490_e27338_d_n0, assign16490_e27338_d_n2, assign16490_e27338_d_n3, assign16490_e27338_d_n4, assign16490_e27338_d_n5, assign16490_e27338_d_n6, assign16490_e27338_d_n7, assign16490_e27338_d_n8, assign16490_e27338_d_n9, assign16490_e27338_d_n10, assign16490_e27338_d_n11, assign16490_e27338_d_n13, assign16490_e27338_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard288 != 0.0)) {
        let assign16490_e27277: f64 = (-locals.var_vsatcv_i);
        let assign16490_e27280: f64 = (-locals.var_atcv_i);
        let assign16490_e27282: f64 = (assign16490_e27280 * locals.var_deltemp);
        let assign16490_e27285: f64 = (p.p574 * locals.var_deltemp1);
        let assign16490_e27287: f64 = (assign16490_e27285 * locals.var_deltemp1);
        let assign16490_e27288: f64 = (assign16490_e27282 + assign16490_e27287);
        let assign16490_e27290: f64 = (-locals.var_vsatcv_i);
        let assign16490_e27291: f64 = (assign16490_e27288 - assign16490_e27290);
        let assign16490_e27293: f64 = (assign16490_e27291 - 1e-6);
        let assign16490_e27295: f64 = (-locals.var_atcv_i);
        let assign16490_e27297: f64 = (assign16490_e27295 * locals.var_deltemp);
        let assign16490_e27300: f64 = (p.p574 * locals.var_deltemp1);
        let assign16490_e27302: f64 = (assign16490_e27300 * locals.var_deltemp1);
        let assign16490_e27303: f64 = (assign16490_e27297 + assign16490_e27302);
        let assign16490_e27305: f64 = (-locals.var_vsatcv_i);
        let assign16490_e27306: f64 = (assign16490_e27303 - assign16490_e27305);
        let assign16490_e27308: f64 = (assign16490_e27306 - 1e-6);
        let assign16490_e27310: f64 = (-locals.var_atcv_i);
        let assign16490_e27312: f64 = (assign16490_e27310 * locals.var_deltemp);
        let assign16490_e27315: f64 = (p.p574 * locals.var_deltemp1);
        let assign16490_e27317: f64 = (assign16490_e27315 * locals.var_deltemp1);
        let assign16490_e27318: f64 = (assign16490_e27312 + assign16490_e27317);
        let assign16490_e27320: f64 = (-locals.var_vsatcv_i);
        let assign16490_e27321: f64 = (assign16490_e27318 - assign16490_e27320);
        let assign16490_e27323: f64 = (assign16490_e27321 - 1e-6);
        let assign16490_e27324: f64 = (assign16490_e27308 * assign16490_e27323);
        let assign16490_e27327: f64 = (-locals.var_vsatcv_i);
        let assign16490_e27328: f64 = (4.0 * assign16490_e27327);
        let assign16490_e27330: f64 = (assign16490_e27328 * 1e-6);
        let assign16490_e27331: f64 = (assign16490_e27324 - assign16490_e27330);
        let assign16490_e27332: f64 = (assign16490_e27331).sqrt();
        let assign16490_e27333: f64 = (assign16490_e27293 + assign16490_e27332);
        let assign16490_e27334: f64 = (0.5 * assign16490_e27333);
        let assign16490_e27335: f64 = (assign16490_e27277 + assign16490_e27334);
        let assign16490_e27336: f64 = (locals.var_vsatcv_i + assign16490_e27335);
        (assign16490_e27336, (locals.var_vsatcv_i_dn0 + ((-locals.var_vsatcv_i_dn0) + (0.5 * ((-(-locals.var_vsatcv_i_dn0)) + (((((-(-locals.var_vsatcv_i_dn0)) * assign16490_e27323) + (assign16490_e27308 * (-(-locals.var_vsatcv_i_dn0)))) - ((4.0 * (-locals.var_vsatcv_i_dn0)) * 1e-6)) / (2.0 * assign16490_e27332)))))), (locals.var_vsatcv_i_dn2 + ((-locals.var_vsatcv_i_dn2) + (0.5 * ((-(-locals.var_vsatcv_i_dn2)) + (((((-(-locals.var_vsatcv_i_dn2)) * assign16490_e27323) + (assign16490_e27308 * (-(-locals.var_vsatcv_i_dn2)))) - ((4.0 * (-locals.var_vsatcv_i_dn2)) * 1e-6)) / (2.0 * assign16490_e27332)))))), (locals.var_vsatcv_i_dn3 + ((-locals.var_vsatcv_i_dn3) + (0.5 * ((-(-locals.var_vsatcv_i_dn3)) + (((((-(-locals.var_vsatcv_i_dn3)) * assign16490_e27323) + (assign16490_e27308 * (-(-locals.var_vsatcv_i_dn3)))) - ((4.0 * (-locals.var_vsatcv_i_dn3)) * 1e-6)) / (2.0 * assign16490_e27332)))))), (locals.var_vsatcv_i_dn4 + ((-locals.var_vsatcv_i_dn4) + (0.5 * ((((assign16490_e27280 * locals.var_deltemp_dn4) + (((p.p574 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16490_e27285 * locals.var_deltemp1_dn4))) - (-locals.var_vsatcv_i_dn4)) + (((((((assign16490_e27295 * locals.var_deltemp_dn4) + (((p.p574 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16490_e27300 * locals.var_deltemp1_dn4))) - (-locals.var_vsatcv_i_dn4)) * assign16490_e27323) + (assign16490_e27308 * (((assign16490_e27310 * locals.var_deltemp_dn4) + (((p.p574 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16490_e27315 * locals.var_deltemp1_dn4))) - (-locals.var_vsatcv_i_dn4)))) - ((4.0 * (-locals.var_vsatcv_i_dn4)) * 1e-6)) / (2.0 * assign16490_e27332)))))), (locals.var_vsatcv_i_dn5 + ((-locals.var_vsatcv_i_dn5) + (0.5 * ((-(-locals.var_vsatcv_i_dn5)) + (((((-(-locals.var_vsatcv_i_dn5)) * assign16490_e27323) + (assign16490_e27308 * (-(-locals.var_vsatcv_i_dn5)))) - ((4.0 * (-locals.var_vsatcv_i_dn5)) * 1e-6)) / (2.0 * assign16490_e27332)))))), (locals.var_vsatcv_i_dn6 + ((-locals.var_vsatcv_i_dn6) + (0.5 * ((-(-locals.var_vsatcv_i_dn6)) + (((((-(-locals.var_vsatcv_i_dn6)) * assign16490_e27323) + (assign16490_e27308 * (-(-locals.var_vsatcv_i_dn6)))) - ((4.0 * (-locals.var_vsatcv_i_dn6)) * 1e-6)) / (2.0 * assign16490_e27332)))))), (locals.var_vsatcv_i_dn7 + ((-locals.var_vsatcv_i_dn7) + (0.5 * ((-(-locals.var_vsatcv_i_dn7)) + (((((-(-locals.var_vsatcv_i_dn7)) * assign16490_e27323) + (assign16490_e27308 * (-(-locals.var_vsatcv_i_dn7)))) - ((4.0 * (-locals.var_vsatcv_i_dn7)) * 1e-6)) / (2.0 * assign16490_e27332)))))), (locals.var_vsatcv_i_dn8 + ((-locals.var_vsatcv_i_dn8) + (0.5 * ((-(-locals.var_vsatcv_i_dn8)) + (((((-(-locals.var_vsatcv_i_dn8)) * assign16490_e27323) + (assign16490_e27308 * (-(-locals.var_vsatcv_i_dn8)))) - ((4.0 * (-locals.var_vsatcv_i_dn8)) * 1e-6)) / (2.0 * assign16490_e27332)))))), (locals.var_vsatcv_i_dn9 + ((-locals.var_vsatcv_i_dn9) + (0.5 * ((-(-locals.var_vsatcv_i_dn9)) + (((((-(-locals.var_vsatcv_i_dn9)) * assign16490_e27323) + (assign16490_e27308 * (-(-locals.var_vsatcv_i_dn9)))) - ((4.0 * (-locals.var_vsatcv_i_dn9)) * 1e-6)) / (2.0 * assign16490_e27332)))))), (locals.var_vsatcv_i_dn10 + ((-locals.var_vsatcv_i_dn10) + (0.5 * ((-(-locals.var_vsatcv_i_dn10)) + (((((-(-locals.var_vsatcv_i_dn10)) * assign16490_e27323) + (assign16490_e27308 * (-(-locals.var_vsatcv_i_dn10)))) - ((4.0 * (-locals.var_vsatcv_i_dn10)) * 1e-6)) / (2.0 * assign16490_e27332)))))), (locals.var_vsatcv_i_dn11 + ((-locals.var_vsatcv_i_dn11) + (0.5 * ((-(-locals.var_vsatcv_i_dn11)) + (((((-(-locals.var_vsatcv_i_dn11)) * assign16490_e27323) + (assign16490_e27308 * (-(-locals.var_vsatcv_i_dn11)))) - ((4.0 * (-locals.var_vsatcv_i_dn11)) * 1e-6)) / (2.0 * assign16490_e27332)))))), (locals.var_vsatcv_i_dn13 + ((-locals.var_vsatcv_i_dn13) + (0.5 * ((-(-locals.var_vsatcv_i_dn13)) + (((((-(-locals.var_vsatcv_i_dn13)) * assign16490_e27323) + (assign16490_e27308 * (-(-locals.var_vsatcv_i_dn13)))) - ((4.0 * (-locals.var_vsatcv_i_dn13)) * 1e-6)) / (2.0 * assign16490_e27332)))))), (locals.var_vsatcv_i_dn14 + ((-locals.var_vsatcv_i_dn14) + (0.5 * ((-(-locals.var_vsatcv_i_dn14)) + (((((-(-locals.var_vsatcv_i_dn14)) * assign16490_e27323) + (assign16490_e27308 * (-(-locals.var_vsatcv_i_dn14)))) - ((4.0 * (-locals.var_vsatcv_i_dn14)) * 1e-6)) / (2.0 * assign16490_e27332)))))),)
    } else {
        (locals.var_vsatcv_t, locals.var_vsatcv_t_dn0, locals.var_vsatcv_t_dn2, locals.var_vsatcv_t_dn3, locals.var_vsatcv_t_dn4, locals.var_vsatcv_t_dn5, locals.var_vsatcv_t_dn6, locals.var_vsatcv_t_dn7, locals.var_vsatcv_t_dn8, locals.var_vsatcv_t_dn9, locals.var_vsatcv_t_dn10, locals.var_vsatcv_t_dn11, locals.var_vsatcv_t_dn13, locals.var_vsatcv_t_dn14,)
    }
};
        locals.var_vsatcv_t = assign16490_e27338;
        locals.var_vsatcv_t_dn0 = assign16490_e27338_d_n0;
        locals.var_vsatcv_t_dn2 = assign16490_e27338_d_n2;
        locals.var_vsatcv_t_dn3 = assign16490_e27338_d_n3;
        locals.var_vsatcv_t_dn4 = assign16490_e27338_d_n4;
        locals.var_vsatcv_t_dn5 = assign16490_e27338_d_n5;
        locals.var_vsatcv_t_dn6 = assign16490_e27338_d_n6;
        locals.var_vsatcv_t_dn7 = assign16490_e27338_d_n7;
        locals.var_vsatcv_t_dn8 = assign16490_e27338_d_n8;
        locals.var_vsatcv_t_dn9 = assign16490_e27338_d_n9;
        locals.var_vsatcv_t_dn10 = assign16490_e27338_d_n10;
        locals.var_vsatcv_t_dn11 = assign16490_e27338_d_n11;
        locals.var_vsatcv_t_dn13 = assign16490_e27338_d_n13;
        locals.var_vsatcv_t_dn14 = assign16490_e27338_d_n14;

        let (assign16500_e27406, assign16500_e27406_d_n4,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard288 != 0.0)) {
        let assign16500_e27348: f64 = (-locals.var_ksativ_i);
        let assign16500_e27352: f64 = (p.p498 * locals.var_deltemp1);
        let assign16500_e27355: f64 = (p.p499 * locals.var_deltemp1);
        let assign16500_e27357: f64 = (assign16500_e27355 * locals.var_deltemp1);
        let assign16500_e27358: f64 = (assign16500_e27352 + assign16500_e27357);
        let assign16500_e27360: f64 = (-locals.var_ksativ_i);
        let assign16500_e27361: f64 = (assign16500_e27358 - assign16500_e27360);
        let assign16500_e27363: f64 = (assign16500_e27361 - 1e-6);
        let assign16500_e27366: f64 = (p.p498 * locals.var_deltemp1);
        let assign16500_e27369: f64 = (p.p499 * locals.var_deltemp1);
        let assign16500_e27371: f64 = (assign16500_e27369 * locals.var_deltemp1);
        let assign16500_e27372: f64 = (assign16500_e27366 + assign16500_e27371);
        let assign16500_e27374: f64 = (-locals.var_ksativ_i);
        let assign16500_e27375: f64 = (assign16500_e27372 - assign16500_e27374);
        let assign16500_e27377: f64 = (assign16500_e27375 - 1e-6);
        let assign16500_e27380: f64 = (p.p498 * locals.var_deltemp1);
        let assign16500_e27383: f64 = (p.p499 * locals.var_deltemp1);
        let assign16500_e27385: f64 = (assign16500_e27383 * locals.var_deltemp1);
        let assign16500_e27386: f64 = (assign16500_e27380 + assign16500_e27385);
        let assign16500_e27388: f64 = (-locals.var_ksativ_i);
        let assign16500_e27389: f64 = (assign16500_e27386 - assign16500_e27388);
        let assign16500_e27391: f64 = (assign16500_e27389 - 1e-6);
        let assign16500_e27392: f64 = (assign16500_e27377 * assign16500_e27391);
        let assign16500_e27395: f64 = (-locals.var_ksativ_i);
        let assign16500_e27396: f64 = (4.0 * assign16500_e27395);
        let assign16500_e27398: f64 = (assign16500_e27396 * 1e-6);
        let assign16500_e27399: f64 = (assign16500_e27392 - assign16500_e27398);
        let assign16500_e27400: f64 = (assign16500_e27399).sqrt();
        let assign16500_e27401: f64 = (assign16500_e27363 + assign16500_e27400);
        let assign16500_e27402: f64 = (0.5 * assign16500_e27401);
        let assign16500_e27403: f64 = (assign16500_e27348 + assign16500_e27402);
        let assign16500_e27404: f64 = (locals.var_ksativ_i + assign16500_e27403);
        (assign16500_e27404, (0.5 * (((p.p498 * locals.var_deltemp1_dn4) + (((p.p499 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16500_e27355 * locals.var_deltemp1_dn4))) + (((((p.p498 * locals.var_deltemp1_dn4) + (((p.p499 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16500_e27369 * locals.var_deltemp1_dn4))) * assign16500_e27391) + (assign16500_e27377 * ((p.p498 * locals.var_deltemp1_dn4) + (((p.p499 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16500_e27383 * locals.var_deltemp1_dn4))))) / (2.0 * assign16500_e27400)))),)
    } else {
        (locals.var_ksativ_t, locals.var_ksativ_t_dn4,)
    }
};
        locals.var_ksativ_t = assign16500_e27406;
        locals.var_ksativ_t_dn4 = assign16500_e27406_d_n4;

        let (assign16510_e27456, assign16510_e27456_d_n0, assign16510_e27456_d_n2, assign16510_e27456_d_n3, assign16510_e27456_d_n4, assign16510_e27456_d_n5, assign16510_e27456_d_n6, assign16510_e27456_d_n7, assign16510_e27456_d_n8, assign16510_e27456_d_n9, assign16510_e27456_d_n10, assign16510_e27456_d_n11, assign16510_e27456_d_n13, assign16510_e27456_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard288 != 0.0)) {
        let assign16510_e27416: f64 = (-locals.var_pclm_i);
        let assign16510_e27420: f64 = (p.p1026 * locals.var_deltemp1);
        let assign16510_e27422: f64 = (-locals.var_pclm_i);
        let assign16510_e27423: f64 = (assign16510_e27420 - assign16510_e27422);
        let assign16510_e27425: f64 = (assign16510_e27423 - 1e-6);
        let assign16510_e27428: f64 = (p.p1026 * locals.var_deltemp1);
        let assign16510_e27430: f64 = (-locals.var_pclm_i);
        let assign16510_e27431: f64 = (assign16510_e27428 - assign16510_e27430);
        let assign16510_e27433: f64 = (assign16510_e27431 - 1e-6);
        let assign16510_e27436: f64 = (p.p1026 * locals.var_deltemp1);
        let assign16510_e27438: f64 = (-locals.var_pclm_i);
        let assign16510_e27439: f64 = (assign16510_e27436 - assign16510_e27438);
        let assign16510_e27441: f64 = (assign16510_e27439 - 1e-6);
        let assign16510_e27442: f64 = (assign16510_e27433 * assign16510_e27441);
        let assign16510_e27445: f64 = (-locals.var_pclm_i);
        let assign16510_e27446: f64 = (4.0 * assign16510_e27445);
        let assign16510_e27448: f64 = (assign16510_e27446 * 1e-6);
        let assign16510_e27449: f64 = (assign16510_e27442 - assign16510_e27448);
        let assign16510_e27450: f64 = (assign16510_e27449).sqrt();
        let assign16510_e27451: f64 = (assign16510_e27425 + assign16510_e27450);
        let assign16510_e27452: f64 = (0.5 * assign16510_e27451);
        let assign16510_e27453: f64 = (assign16510_e27416 + assign16510_e27452);
        let assign16510_e27454: f64 = (locals.var_pclm_i + assign16510_e27453);
        (assign16510_e27454, (locals.var_pclm_i_dn0 + ((-locals.var_pclm_i_dn0) + (0.5 * ((-(-locals.var_pclm_i_dn0)) + (((((-(-locals.var_pclm_i_dn0)) * assign16510_e27441) + (assign16510_e27433 * (-(-locals.var_pclm_i_dn0)))) - ((4.0 * (-locals.var_pclm_i_dn0)) * 1e-6)) / (2.0 * assign16510_e27450)))))), (locals.var_pclm_i_dn2 + ((-locals.var_pclm_i_dn2) + (0.5 * ((-(-locals.var_pclm_i_dn2)) + (((((-(-locals.var_pclm_i_dn2)) * assign16510_e27441) + (assign16510_e27433 * (-(-locals.var_pclm_i_dn2)))) - ((4.0 * (-locals.var_pclm_i_dn2)) * 1e-6)) / (2.0 * assign16510_e27450)))))), (locals.var_pclm_i_dn3 + ((-locals.var_pclm_i_dn3) + (0.5 * ((-(-locals.var_pclm_i_dn3)) + (((((-(-locals.var_pclm_i_dn3)) * assign16510_e27441) + (assign16510_e27433 * (-(-locals.var_pclm_i_dn3)))) - ((4.0 * (-locals.var_pclm_i_dn3)) * 1e-6)) / (2.0 * assign16510_e27450)))))), (locals.var_pclm_i_dn4 + ((-locals.var_pclm_i_dn4) + (0.5 * (((p.p1026 * locals.var_deltemp1_dn4) - (-locals.var_pclm_i_dn4)) + ((((((p.p1026 * locals.var_deltemp1_dn4) - (-locals.var_pclm_i_dn4)) * assign16510_e27441) + (assign16510_e27433 * ((p.p1026 * locals.var_deltemp1_dn4) - (-locals.var_pclm_i_dn4)))) - ((4.0 * (-locals.var_pclm_i_dn4)) * 1e-6)) / (2.0 * assign16510_e27450)))))), (locals.var_pclm_i_dn5 + ((-locals.var_pclm_i_dn5) + (0.5 * ((-(-locals.var_pclm_i_dn5)) + (((((-(-locals.var_pclm_i_dn5)) * assign16510_e27441) + (assign16510_e27433 * (-(-locals.var_pclm_i_dn5)))) - ((4.0 * (-locals.var_pclm_i_dn5)) * 1e-6)) / (2.0 * assign16510_e27450)))))), (locals.var_pclm_i_dn6 + ((-locals.var_pclm_i_dn6) + (0.5 * ((-(-locals.var_pclm_i_dn6)) + (((((-(-locals.var_pclm_i_dn6)) * assign16510_e27441) + (assign16510_e27433 * (-(-locals.var_pclm_i_dn6)))) - ((4.0 * (-locals.var_pclm_i_dn6)) * 1e-6)) / (2.0 * assign16510_e27450)))))), (locals.var_pclm_i_dn7 + ((-locals.var_pclm_i_dn7) + (0.5 * ((-(-locals.var_pclm_i_dn7)) + (((((-(-locals.var_pclm_i_dn7)) * assign16510_e27441) + (assign16510_e27433 * (-(-locals.var_pclm_i_dn7)))) - ((4.0 * (-locals.var_pclm_i_dn7)) * 1e-6)) / (2.0 * assign16510_e27450)))))), (locals.var_pclm_i_dn8 + ((-locals.var_pclm_i_dn8) + (0.5 * ((-(-locals.var_pclm_i_dn8)) + (((((-(-locals.var_pclm_i_dn8)) * assign16510_e27441) + (assign16510_e27433 * (-(-locals.var_pclm_i_dn8)))) - ((4.0 * (-locals.var_pclm_i_dn8)) * 1e-6)) / (2.0 * assign16510_e27450)))))), (locals.var_pclm_i_dn9 + ((-locals.var_pclm_i_dn9) + (0.5 * ((-(-locals.var_pclm_i_dn9)) + (((((-(-locals.var_pclm_i_dn9)) * assign16510_e27441) + (assign16510_e27433 * (-(-locals.var_pclm_i_dn9)))) - ((4.0 * (-locals.var_pclm_i_dn9)) * 1e-6)) / (2.0 * assign16510_e27450)))))), (locals.var_pclm_i_dn10 + ((-locals.var_pclm_i_dn10) + (0.5 * ((-(-locals.var_pclm_i_dn10)) + (((((-(-locals.var_pclm_i_dn10)) * assign16510_e27441) + (assign16510_e27433 * (-(-locals.var_pclm_i_dn10)))) - ((4.0 * (-locals.var_pclm_i_dn10)) * 1e-6)) / (2.0 * assign16510_e27450)))))), (locals.var_pclm_i_dn11 + ((-locals.var_pclm_i_dn11) + (0.5 * ((-(-locals.var_pclm_i_dn11)) + (((((-(-locals.var_pclm_i_dn11)) * assign16510_e27441) + (assign16510_e27433 * (-(-locals.var_pclm_i_dn11)))) - ((4.0 * (-locals.var_pclm_i_dn11)) * 1e-6)) / (2.0 * assign16510_e27450)))))), (locals.var_pclm_i_dn13 + ((-locals.var_pclm_i_dn13) + (0.5 * ((-(-locals.var_pclm_i_dn13)) + (((((-(-locals.var_pclm_i_dn13)) * assign16510_e27441) + (assign16510_e27433 * (-(-locals.var_pclm_i_dn13)))) - ((4.0 * (-locals.var_pclm_i_dn13)) * 1e-6)) / (2.0 * assign16510_e27450)))))), (locals.var_pclm_i_dn14 + ((-locals.var_pclm_i_dn14) + (0.5 * ((-(-locals.var_pclm_i_dn14)) + (((((-(-locals.var_pclm_i_dn14)) * assign16510_e27441) + (assign16510_e27433 * (-(-locals.var_pclm_i_dn14)))) - ((4.0 * (-locals.var_pclm_i_dn14)) * 1e-6)) / (2.0 * assign16510_e27450)))))),)
    } else {
        (locals.var_pclm_t, locals.var_pclm_t_dn0, locals.var_pclm_t_dn2, locals.var_pclm_t_dn3, locals.var_pclm_t_dn4, locals.var_pclm_t_dn5, locals.var_pclm_t_dn6, locals.var_pclm_t_dn7, locals.var_pclm_t_dn8, locals.var_pclm_t_dn9, locals.var_pclm_t_dn10, locals.var_pclm_t_dn11, locals.var_pclm_t_dn13, locals.var_pclm_t_dn14,)
    }
};
        locals.var_pclm_t = assign16510_e27456;
        locals.var_pclm_t_dn0 = assign16510_e27456_d_n0;
        locals.var_pclm_t_dn2 = assign16510_e27456_d_n2;
        locals.var_pclm_t_dn3 = assign16510_e27456_d_n3;
        locals.var_pclm_t_dn4 = assign16510_e27456_d_n4;
        locals.var_pclm_t_dn5 = assign16510_e27456_d_n5;
        locals.var_pclm_t_dn6 = assign16510_e27456_d_n6;
        locals.var_pclm_t_dn7 = assign16510_e27456_d_n7;
        locals.var_pclm_t_dn8 = assign16510_e27456_d_n8;
        locals.var_pclm_t_dn9 = assign16510_e27456_d_n9;
        locals.var_pclm_t_dn10 = assign16510_e27456_d_n10;
        locals.var_pclm_t_dn11 = assign16510_e27456_d_n11;
        locals.var_pclm_t_dn13 = assign16510_e27456_d_n13;
        locals.var_pclm_t_dn14 = assign16510_e27456_d_n14;

        let (assign16520_e27540, assign16520_e27540_d_n0, assign16520_e27540_d_n2, assign16520_e27540_d_n3, assign16520_e27540_d_n4, assign16520_e27540_d_n5, assign16520_e27540_d_n6, assign16520_e27540_d_n7, assign16520_e27540_d_n8, assign16520_e27540_d_n9, assign16520_e27540_d_n10, assign16520_e27540_d_n11, assign16520_e27540_d_n13, assign16520_e27540_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard288 == 0.0)) {
        let assign16520_e27469: f64 = (locals.var_eu1_i * locals.var_deltemp1);
        let assign16520_e27470: f64 = (1.0 + assign16520_e27469);
        let assign16520_e27472: f64 = (assign16520_e27470 - 1e-6);
        let assign16520_e27474: f64 = (-10000.0);
        let assign16520_e27476: f64 = (assign16520_e27474 * 0.001);
        let (assign16520_e27537, assign16520_e27537_d_n4,) = {
            if (!(assign16520_e27472 < assign16520_e27476)) {
                let assign16520_e27483: f64 = (locals.var_eu1_i * locals.var_deltemp1);
                let assign16520_e27484: f64 = (1.0 + assign16520_e27483);
                let assign16520_e27486: f64 = (assign16520_e27484 - 1e-6);
                let assign16520_e27490: f64 = (locals.var_eu1_i * locals.var_deltemp1);
                let assign16520_e27491: f64 = (1.0 + assign16520_e27490);
                let assign16520_e27493: f64 = (assign16520_e27491 - 1e-6);
                let assign16520_e27497: f64 = (locals.var_eu1_i * locals.var_deltemp1);
                let assign16520_e27498: f64 = (1.0 + assign16520_e27497);
                let assign16520_e27500: f64 = (assign16520_e27498 - 1e-6);
                let assign16520_e27501: f64 = (assign16520_e27493 * assign16520_e27500);
                let assign16520_e27504: f64 = (4.0 * 0.001);
                let assign16520_e27506: f64 = (assign16520_e27504 * 0.001);
                let assign16520_e27507: f64 = (assign16520_e27501 + assign16520_e27506);
                let assign16520_e27508: f64 = (assign16520_e27507).sqrt();
                let assign16520_e27509: f64 = (assign16520_e27486 + assign16520_e27508);
                let assign16520_e27510: f64 = (0.5 * assign16520_e27509);
                (assign16520_e27510, (0.5 * ((locals.var_eu1_i * locals.var_deltemp1_dn4) + ((((locals.var_eu1_i * locals.var_deltemp1_dn4) * assign16520_e27500) + (assign16520_e27493 * (locals.var_eu1_i * locals.var_deltemp1_dn4))) / (2.0 * assign16520_e27508)))),)
            } else {
                let assign16520_e27514: f64 = (locals.var_eu1_i * locals.var_deltemp1);
                let assign16520_e27515: f64 = (1.0 + assign16520_e27514);
                let assign16520_e27517: f64 = (assign16520_e27515 - 1e-6);
                let assign16520_e27519: f64 = (-10000.0);
                let assign16520_e27521: f64 = (assign16520_e27519 * 0.001);
                let (assign16520_e27536, assign16520_e27536_d_n4,) = {
                    if (assign16520_e27517 < assign16520_e27521) {
                        let assign16520_e27524: f64 = (-0.001);
                        let assign16520_e27526: f64 = (assign16520_e27524 * 0.001);
                        let assign16520_e27530: f64 = (locals.var_eu1_i * locals.var_deltemp1);
                        let assign16520_e27531: f64 = (1.0 + assign16520_e27530);
                        let assign16520_e27533: f64 = (assign16520_e27531 - 1e-6);
                        let assign16520_e27534: f64 = (assign16520_e27526 / assign16520_e27533);
                        (assign16520_e27534, (-((assign16520_e27526 * (locals.var_eu1_i * locals.var_deltemp1_dn4)) / (assign16520_e27533 * assign16520_e27533))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign16520_e27536, assign16520_e27536_d_n4,)
            }
        };
        let assign16520_e27538: f64 = (locals.var_eu_i * assign16520_e27537);
        (assign16520_e27538, (locals.var_eu_i_dn0 * assign16520_e27537), (locals.var_eu_i_dn2 * assign16520_e27537), (locals.var_eu_i_dn3 * assign16520_e27537), ((locals.var_eu_i_dn4 * assign16520_e27537) + (locals.var_eu_i * assign16520_e27537_d_n4)), (locals.var_eu_i_dn5 * assign16520_e27537), (locals.var_eu_i_dn6 * assign16520_e27537), (locals.var_eu_i_dn7 * assign16520_e27537), (locals.var_eu_i_dn8 * assign16520_e27537), (locals.var_eu_i_dn9 * assign16520_e27537), (locals.var_eu_i_dn10 * assign16520_e27537), (locals.var_eu_i_dn11 * assign16520_e27537), (locals.var_eu_i_dn13 * assign16520_e27537), (locals.var_eu_i_dn14 * assign16520_e27537),)
    } else {
        (locals.var_eu_t, locals.var_eu_t_dn0, locals.var_eu_t_dn2, locals.var_eu_t_dn3, locals.var_eu_t_dn4, locals.var_eu_t_dn5, locals.var_eu_t_dn6, locals.var_eu_t_dn7, locals.var_eu_t_dn8, locals.var_eu_t_dn9, locals.var_eu_t_dn10, locals.var_eu_t_dn11, locals.var_eu_t_dn13, locals.var_eu_t_dn14,)
    }
};
        locals.var_eu_t = assign16520_e27540;
        locals.var_eu_t_dn0 = assign16520_e27540_d_n0;
        locals.var_eu_t_dn2 = assign16520_e27540_d_n2;
        locals.var_eu_t_dn3 = assign16520_e27540_d_n3;
        locals.var_eu_t_dn4 = assign16520_e27540_d_n4;
        locals.var_eu_t_dn5 = assign16520_e27540_d_n5;
        locals.var_eu_t_dn6 = assign16520_e27540_d_n6;
        locals.var_eu_t_dn7 = assign16520_e27540_d_n7;
        locals.var_eu_t_dn8 = assign16520_e27540_d_n8;
        locals.var_eu_t_dn9 = assign16520_e27540_d_n9;
        locals.var_eu_t_dn10 = assign16520_e27540_d_n10;
        locals.var_eu_t_dn11 = assign16520_e27540_d_n11;
        locals.var_eu_t_dn13 = assign16520_e27540_d_n13;
        locals.var_eu_t_dn14 = assign16520_e27540_d_n14;

    }

    pub(super) fn stamp_transient_block_51(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign16530_e27660, assign16530_e27660_d_n0, assign16530_e27660_d_n2, assign16530_e27660_d_n3, assign16530_e27660_d_n4, assign16530_e27660_d_n5, assign16530_e27660_d_n6, assign16530_e27660_d_n7, assign16530_e27660_d_n8, assign16530_e27660_d_n9, assign16530_e27660_d_n10, assign16530_e27660_d_n11, assign16530_e27660_d_n13, assign16530_e27660_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard288 == 0.0)) {
        let assign16530_e27553: f64 = (locals.var_at_i * locals.var_deltemp);
        let assign16530_e27554: f64 = (1.0 - assign16530_e27553);
        let assign16530_e27557: f64 = (p.p561 * locals.var_deltemp1);
        let assign16530_e27559: f64 = (assign16530_e27557 * locals.var_deltemp1);
        let assign16530_e27560: f64 = (assign16530_e27554 + assign16530_e27559);
        let assign16530_e27562: f64 = (assign16530_e27560 - 1e-6);
        let assign16530_e27564: f64 = (-10000.0);
        let assign16530_e27566: f64 = (assign16530_e27564 * 0.001);
        let (assign16530_e27657, assign16530_e27657_d_n4,) = {
            if (!(assign16530_e27562 < assign16530_e27566)) {
                let assign16530_e27573: f64 = (locals.var_at_i * locals.var_deltemp);
                let assign16530_e27574: f64 = (1.0 - assign16530_e27573);
                let assign16530_e27577: f64 = (p.p561 * locals.var_deltemp1);
                let assign16530_e27579: f64 = (assign16530_e27577 * locals.var_deltemp1);
                let assign16530_e27580: f64 = (assign16530_e27574 + assign16530_e27579);
                let assign16530_e27582: f64 = (assign16530_e27580 - 1e-6);
                let assign16530_e27586: f64 = (locals.var_at_i * locals.var_deltemp);
                let assign16530_e27587: f64 = (1.0 - assign16530_e27586);
                let assign16530_e27590: f64 = (p.p561 * locals.var_deltemp1);
                let assign16530_e27592: f64 = (assign16530_e27590 * locals.var_deltemp1);
                let assign16530_e27593: f64 = (assign16530_e27587 + assign16530_e27592);
                let assign16530_e27595: f64 = (assign16530_e27593 - 1e-6);
                let assign16530_e27599: f64 = (locals.var_at_i * locals.var_deltemp);
                let assign16530_e27600: f64 = (1.0 - assign16530_e27599);
                let assign16530_e27603: f64 = (p.p561 * locals.var_deltemp1);
                let assign16530_e27605: f64 = (assign16530_e27603 * locals.var_deltemp1);
                let assign16530_e27606: f64 = (assign16530_e27600 + assign16530_e27605);
                let assign16530_e27608: f64 = (assign16530_e27606 - 1e-6);
                let assign16530_e27609: f64 = (assign16530_e27595 * assign16530_e27608);
                let assign16530_e27612: f64 = (4.0 * 0.001);
                let assign16530_e27614: f64 = (assign16530_e27612 * 0.001);
                let assign16530_e27615: f64 = (assign16530_e27609 + assign16530_e27614);
                let assign16530_e27616: f64 = (assign16530_e27615).sqrt();
                let assign16530_e27617: f64 = (assign16530_e27582 + assign16530_e27616);
                let assign16530_e27618: f64 = (0.5 * assign16530_e27617);
                (assign16530_e27618, (0.5 * (((-(locals.var_at_i * locals.var_deltemp_dn4)) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16530_e27577 * locals.var_deltemp1_dn4))) + (((((-(locals.var_at_i * locals.var_deltemp_dn4)) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16530_e27590 * locals.var_deltemp1_dn4))) * assign16530_e27608) + (assign16530_e27595 * ((-(locals.var_at_i * locals.var_deltemp_dn4)) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16530_e27603 * locals.var_deltemp1_dn4))))) / (2.0 * assign16530_e27616)))),)
            } else {
                let assign16530_e27622: f64 = (locals.var_at_i * locals.var_deltemp);
                let assign16530_e27623: f64 = (1.0 - assign16530_e27622);
                let assign16530_e27626: f64 = (p.p561 * locals.var_deltemp1);
                let assign16530_e27628: f64 = (assign16530_e27626 * locals.var_deltemp1);
                let assign16530_e27629: f64 = (assign16530_e27623 + assign16530_e27628);
                let assign16530_e27631: f64 = (assign16530_e27629 - 1e-6);
                let assign16530_e27633: f64 = (-10000.0);
                let assign16530_e27635: f64 = (assign16530_e27633 * 0.001);
                let (assign16530_e27656, assign16530_e27656_d_n4,) = {
                    if (assign16530_e27631 < assign16530_e27635) {
                        let assign16530_e27638: f64 = (-0.001);
                        let assign16530_e27640: f64 = (assign16530_e27638 * 0.001);
                        let assign16530_e27644: f64 = (locals.var_at_i * locals.var_deltemp);
                        let assign16530_e27645: f64 = (1.0 - assign16530_e27644);
                        let assign16530_e27648: f64 = (p.p561 * locals.var_deltemp1);
                        let assign16530_e27650: f64 = (assign16530_e27648 * locals.var_deltemp1);
                        let assign16530_e27651: f64 = (assign16530_e27645 + assign16530_e27650);
                        let assign16530_e27653: f64 = (assign16530_e27651 - 1e-6);
                        let assign16530_e27654: f64 = (assign16530_e27640 / assign16530_e27653);
                        (assign16530_e27654, (-((assign16530_e27640 * ((-(locals.var_at_i * locals.var_deltemp_dn4)) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16530_e27648 * locals.var_deltemp1_dn4)))) / (assign16530_e27653 * assign16530_e27653))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign16530_e27656, assign16530_e27656_d_n4,)
            }
        };
        let assign16530_e27658: f64 = (locals.var_vsat_i * assign16530_e27657);
        (assign16530_e27658, (locals.var_vsat_i_dn0 * assign16530_e27657), (locals.var_vsat_i_dn2 * assign16530_e27657), (locals.var_vsat_i_dn3 * assign16530_e27657), ((locals.var_vsat_i_dn4 * assign16530_e27657) + (locals.var_vsat_i * assign16530_e27657_d_n4)), (locals.var_vsat_i_dn5 * assign16530_e27657), (locals.var_vsat_i_dn6 * assign16530_e27657), (locals.var_vsat_i_dn7 * assign16530_e27657), (locals.var_vsat_i_dn8 * assign16530_e27657), (locals.var_vsat_i_dn9 * assign16530_e27657), (locals.var_vsat_i_dn10 * assign16530_e27657), (locals.var_vsat_i_dn11 * assign16530_e27657), (locals.var_vsat_i_dn13 * assign16530_e27657), (locals.var_vsat_i_dn14 * assign16530_e27657),)
    } else {
        (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14,)
    }
};
        locals.var_vsat_t = assign16530_e27660;
        locals.var_vsat_t_dn0 = assign16530_e27660_d_n0;
        locals.var_vsat_t_dn2 = assign16530_e27660_d_n2;
        locals.var_vsat_t_dn3 = assign16530_e27660_d_n3;
        locals.var_vsat_t_dn4 = assign16530_e27660_d_n4;
        locals.var_vsat_t_dn5 = assign16530_e27660_d_n5;
        locals.var_vsat_t_dn6 = assign16530_e27660_d_n6;
        locals.var_vsat_t_dn7 = assign16530_e27660_d_n7;
        locals.var_vsat_t_dn8 = assign16530_e27660_d_n8;
        locals.var_vsat_t_dn9 = assign16530_e27660_d_n9;
        locals.var_vsat_t_dn10 = assign16530_e27660_d_n10;
        locals.var_vsat_t_dn11 = assign16530_e27660_d_n11;
        locals.var_vsat_t_dn13 = assign16530_e27660_d_n13;
        locals.var_vsat_t_dn14 = assign16530_e27660_d_n14;

        let assign16540_e27663: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard293 = assign16540_e27663;

        let (assign16550_e27785, assign16550_e27785_d_n4,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard288 == 0.0)) && (locals.var_guard293 != 0.0)) {
        let assign16550_e27678: f64 = (locals.var_atr_i * locals.var_deltemp);
        let assign16550_e27679: f64 = (1.0 - assign16550_e27678);
        let assign16550_e27682: f64 = (p.p561 * locals.var_deltemp1);
        let assign16550_e27684: f64 = (assign16550_e27682 * locals.var_deltemp1);
        let assign16550_e27685: f64 = (assign16550_e27679 + assign16550_e27684);
        let assign16550_e27687: f64 = (assign16550_e27685 - 1e-6);
        let assign16550_e27689: f64 = (-10000.0);
        let assign16550_e27691: f64 = (assign16550_e27689 * 0.001);
        let (assign16550_e27782, assign16550_e27782_d_n4,) = {
            if (!(assign16550_e27687 < assign16550_e27691)) {
                let assign16550_e27698: f64 = (locals.var_atr_i * locals.var_deltemp);
                let assign16550_e27699: f64 = (1.0 - assign16550_e27698);
                let assign16550_e27702: f64 = (p.p561 * locals.var_deltemp1);
                let assign16550_e27704: f64 = (assign16550_e27702 * locals.var_deltemp1);
                let assign16550_e27705: f64 = (assign16550_e27699 + assign16550_e27704);
                let assign16550_e27707: f64 = (assign16550_e27705 - 1e-6);
                let assign16550_e27711: f64 = (locals.var_atr_i * locals.var_deltemp);
                let assign16550_e27712: f64 = (1.0 - assign16550_e27711);
                let assign16550_e27715: f64 = (p.p561 * locals.var_deltemp1);
                let assign16550_e27717: f64 = (assign16550_e27715 * locals.var_deltemp1);
                let assign16550_e27718: f64 = (assign16550_e27712 + assign16550_e27717);
                let assign16550_e27720: f64 = (assign16550_e27718 - 1e-6);
                let assign16550_e27724: f64 = (locals.var_atr_i * locals.var_deltemp);
                let assign16550_e27725: f64 = (1.0 - assign16550_e27724);
                let assign16550_e27728: f64 = (p.p561 * locals.var_deltemp1);
                let assign16550_e27730: f64 = (assign16550_e27728 * locals.var_deltemp1);
                let assign16550_e27731: f64 = (assign16550_e27725 + assign16550_e27730);
                let assign16550_e27733: f64 = (assign16550_e27731 - 1e-6);
                let assign16550_e27734: f64 = (assign16550_e27720 * assign16550_e27733);
                let assign16550_e27737: f64 = (4.0 * 0.001);
                let assign16550_e27739: f64 = (assign16550_e27737 * 0.001);
                let assign16550_e27740: f64 = (assign16550_e27734 + assign16550_e27739);
                let assign16550_e27741: f64 = (assign16550_e27740).sqrt();
                let assign16550_e27742: f64 = (assign16550_e27707 + assign16550_e27741);
                let assign16550_e27743: f64 = (0.5 * assign16550_e27742);
                (assign16550_e27743, (0.5 * (((-(locals.var_atr_i * locals.var_deltemp_dn4)) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16550_e27702 * locals.var_deltemp1_dn4))) + (((((-(locals.var_atr_i * locals.var_deltemp_dn4)) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16550_e27715 * locals.var_deltemp1_dn4))) * assign16550_e27733) + (assign16550_e27720 * ((-(locals.var_atr_i * locals.var_deltemp_dn4)) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16550_e27728 * locals.var_deltemp1_dn4))))) / (2.0 * assign16550_e27741)))),)
            } else {
                let assign16550_e27747: f64 = (locals.var_atr_i * locals.var_deltemp);
                let assign16550_e27748: f64 = (1.0 - assign16550_e27747);
                let assign16550_e27751: f64 = (p.p561 * locals.var_deltemp1);
                let assign16550_e27753: f64 = (assign16550_e27751 * locals.var_deltemp1);
                let assign16550_e27754: f64 = (assign16550_e27748 + assign16550_e27753);
                let assign16550_e27756: f64 = (assign16550_e27754 - 1e-6);
                let assign16550_e27758: f64 = (-10000.0);
                let assign16550_e27760: f64 = (assign16550_e27758 * 0.001);
                let (assign16550_e27781, assign16550_e27781_d_n4,) = {
                    if (assign16550_e27756 < assign16550_e27760) {
                        let assign16550_e27763: f64 = (-0.001);
                        let assign16550_e27765: f64 = (assign16550_e27763 * 0.001);
                        let assign16550_e27769: f64 = (locals.var_atr_i * locals.var_deltemp);
                        let assign16550_e27770: f64 = (1.0 - assign16550_e27769);
                        let assign16550_e27773: f64 = (p.p561 * locals.var_deltemp1);
                        let assign16550_e27775: f64 = (assign16550_e27773 * locals.var_deltemp1);
                        let assign16550_e27776: f64 = (assign16550_e27770 + assign16550_e27775);
                        let assign16550_e27778: f64 = (assign16550_e27776 - 1e-6);
                        let assign16550_e27779: f64 = (assign16550_e27765 / assign16550_e27778);
                        (assign16550_e27779, (-((assign16550_e27765 * ((-(locals.var_atr_i * locals.var_deltemp_dn4)) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16550_e27773 * locals.var_deltemp1_dn4)))) / (assign16550_e27778 * assign16550_e27778))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign16550_e27781, assign16550_e27781_d_n4,)
            }
        };
        let assign16550_e27783: f64 = (locals.var_vsatr_i * assign16550_e27782);
        (assign16550_e27783, (locals.var_vsatr_i * assign16550_e27782_d_n4),)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn4,)
    }
};
        locals.var_vsatr_t = assign16550_e27785;
        locals.var_vsatr_t_dn4 = assign16550_e27785_d_n4;

        let assign16560_e27788: f64 = if locals.var_vsatr_t < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard294 = assign16560_e27788;

        let (assign16570_e27803, assign16570_e27803_d_n4,) = {
    if (((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard288 == 0.0)) && (locals.var_guard293 != 0.0)) && (locals.var_guard294 != 0.0)) {
        (1000.0, 0.0,)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn4,)
    }
};
        locals.var_vsatr_t = assign16570_e27803;
        locals.var_vsatr_t_dn4 = assign16570_e27803_d_n4;

        let (assign16580_e27923, assign16580_e27923_d_n0, assign16580_e27923_d_n2, assign16580_e27923_d_n3, assign16580_e27923_d_n4, assign16580_e27923_d_n5, assign16580_e27923_d_n6, assign16580_e27923_d_n7, assign16580_e27923_d_n8, assign16580_e27923_d_n9, assign16580_e27923_d_n10, assign16580_e27923_d_n11, assign16580_e27923_d_n13, assign16580_e27923_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard288 == 0.0)) {
        let assign16580_e27816: f64 = (locals.var_at_i * locals.var_deltemp);
        let assign16580_e27817: f64 = (1.0 - assign16580_e27816);
        let assign16580_e27820: f64 = (p.p561 * locals.var_deltemp1);
        let assign16580_e27822: f64 = (assign16580_e27820 * locals.var_deltemp1);
        let assign16580_e27823: f64 = (assign16580_e27817 + assign16580_e27822);
        let assign16580_e27825: f64 = (assign16580_e27823 - 1e-6);
        let assign16580_e27827: f64 = (-10000.0);
        let assign16580_e27829: f64 = (assign16580_e27827 * 0.001);
        let (assign16580_e27920, assign16580_e27920_d_n4,) = {
            if (!(assign16580_e27825 < assign16580_e27829)) {
                let assign16580_e27836: f64 = (locals.var_at_i * locals.var_deltemp);
                let assign16580_e27837: f64 = (1.0 - assign16580_e27836);
                let assign16580_e27840: f64 = (p.p561 * locals.var_deltemp1);
                let assign16580_e27842: f64 = (assign16580_e27840 * locals.var_deltemp1);
                let assign16580_e27843: f64 = (assign16580_e27837 + assign16580_e27842);
                let assign16580_e27845: f64 = (assign16580_e27843 - 1e-6);
                let assign16580_e27849: f64 = (locals.var_at_i * locals.var_deltemp);
                let assign16580_e27850: f64 = (1.0 - assign16580_e27849);
                let assign16580_e27853: f64 = (p.p561 * locals.var_deltemp1);
                let assign16580_e27855: f64 = (assign16580_e27853 * locals.var_deltemp1);
                let assign16580_e27856: f64 = (assign16580_e27850 + assign16580_e27855);
                let assign16580_e27858: f64 = (assign16580_e27856 - 1e-6);
                let assign16580_e27862: f64 = (locals.var_at_i * locals.var_deltemp);
                let assign16580_e27863: f64 = (1.0 - assign16580_e27862);
                let assign16580_e27866: f64 = (p.p561 * locals.var_deltemp1);
                let assign16580_e27868: f64 = (assign16580_e27866 * locals.var_deltemp1);
                let assign16580_e27869: f64 = (assign16580_e27863 + assign16580_e27868);
                let assign16580_e27871: f64 = (assign16580_e27869 - 1e-6);
                let assign16580_e27872: f64 = (assign16580_e27858 * assign16580_e27871);
                let assign16580_e27875: f64 = (4.0 * 0.001);
                let assign16580_e27877: f64 = (assign16580_e27875 * 0.001);
                let assign16580_e27878: f64 = (assign16580_e27872 + assign16580_e27877);
                let assign16580_e27879: f64 = (assign16580_e27878).sqrt();
                let assign16580_e27880: f64 = (assign16580_e27845 + assign16580_e27879);
                let assign16580_e27881: f64 = (0.5 * assign16580_e27880);
                (assign16580_e27881, (0.5 * (((-(locals.var_at_i * locals.var_deltemp_dn4)) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16580_e27840 * locals.var_deltemp1_dn4))) + (((((-(locals.var_at_i * locals.var_deltemp_dn4)) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16580_e27853 * locals.var_deltemp1_dn4))) * assign16580_e27871) + (assign16580_e27858 * ((-(locals.var_at_i * locals.var_deltemp_dn4)) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16580_e27866 * locals.var_deltemp1_dn4))))) / (2.0 * assign16580_e27879)))),)
            } else {
                let assign16580_e27885: f64 = (locals.var_at_i * locals.var_deltemp);
                let assign16580_e27886: f64 = (1.0 - assign16580_e27885);
                let assign16580_e27889: f64 = (p.p561 * locals.var_deltemp1);
                let assign16580_e27891: f64 = (assign16580_e27889 * locals.var_deltemp1);
                let assign16580_e27892: f64 = (assign16580_e27886 + assign16580_e27891);
                let assign16580_e27894: f64 = (assign16580_e27892 - 1e-6);
                let assign16580_e27896: f64 = (-10000.0);
                let assign16580_e27898: f64 = (assign16580_e27896 * 0.001);
                let (assign16580_e27919, assign16580_e27919_d_n4,) = {
                    if (assign16580_e27894 < assign16580_e27898) {
                        let assign16580_e27901: f64 = (-0.001);
                        let assign16580_e27903: f64 = (assign16580_e27901 * 0.001);
                        let assign16580_e27907: f64 = (locals.var_at_i * locals.var_deltemp);
                        let assign16580_e27908: f64 = (1.0 - assign16580_e27907);
                        let assign16580_e27911: f64 = (p.p561 * locals.var_deltemp1);
                        let assign16580_e27913: f64 = (assign16580_e27911 * locals.var_deltemp1);
                        let assign16580_e27914: f64 = (assign16580_e27908 + assign16580_e27913);
                        let assign16580_e27916: f64 = (assign16580_e27914 - 1e-6);
                        let assign16580_e27917: f64 = (assign16580_e27903 / assign16580_e27916);
                        (assign16580_e27917, (-((assign16580_e27903 * ((-(locals.var_at_i * locals.var_deltemp_dn4)) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16580_e27911 * locals.var_deltemp1_dn4)))) / (assign16580_e27916 * assign16580_e27916))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign16580_e27919, assign16580_e27919_d_n4,)
            }
        };
        let assign16580_e27921: f64 = (locals.var_vsat1_i * assign16580_e27920);
        (assign16580_e27921, (locals.var_vsat1_i_dn0 * assign16580_e27920), (locals.var_vsat1_i_dn2 * assign16580_e27920), (locals.var_vsat1_i_dn3 * assign16580_e27920), ((locals.var_vsat1_i_dn4 * assign16580_e27920) + (locals.var_vsat1_i * assign16580_e27920_d_n4)), (locals.var_vsat1_i_dn5 * assign16580_e27920), (locals.var_vsat1_i_dn6 * assign16580_e27920), (locals.var_vsat1_i_dn7 * assign16580_e27920), (locals.var_vsat1_i_dn8 * assign16580_e27920), (locals.var_vsat1_i_dn9 * assign16580_e27920), (locals.var_vsat1_i_dn10 * assign16580_e27920), (locals.var_vsat1_i_dn11 * assign16580_e27920), (locals.var_vsat1_i_dn13 * assign16580_e27920), (locals.var_vsat1_i_dn14 * assign16580_e27920),)
    } else {
        (locals.var_vsat1_t, locals.var_vsat1_t_dn0, locals.var_vsat1_t_dn2, locals.var_vsat1_t_dn3, locals.var_vsat1_t_dn4, locals.var_vsat1_t_dn5, locals.var_vsat1_t_dn6, locals.var_vsat1_t_dn7, locals.var_vsat1_t_dn8, locals.var_vsat1_t_dn9, locals.var_vsat1_t_dn10, locals.var_vsat1_t_dn11, locals.var_vsat1_t_dn13, locals.var_vsat1_t_dn14,)
    }
};
        locals.var_vsat1_t = assign16580_e27923;
        locals.var_vsat1_t_dn0 = assign16580_e27923_d_n0;
        locals.var_vsat1_t_dn2 = assign16580_e27923_d_n2;
        locals.var_vsat1_t_dn3 = assign16580_e27923_d_n3;
        locals.var_vsat1_t_dn4 = assign16580_e27923_d_n4;
        locals.var_vsat1_t_dn5 = assign16580_e27923_d_n5;
        locals.var_vsat1_t_dn6 = assign16580_e27923_d_n6;
        locals.var_vsat1_t_dn7 = assign16580_e27923_d_n7;
        locals.var_vsat1_t_dn8 = assign16580_e27923_d_n8;
        locals.var_vsat1_t_dn9 = assign16580_e27923_d_n9;
        locals.var_vsat1_t_dn10 = assign16580_e27923_d_n10;
        locals.var_vsat1_t_dn11 = assign16580_e27923_d_n11;
        locals.var_vsat1_t_dn13 = assign16580_e27923_d_n13;
        locals.var_vsat1_t_dn14 = assign16580_e27923_d_n14;

        let assign16590_e27926: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard295 = assign16590_e27926;

        let (assign16600_e28048, assign16600_e28048_d_n0, assign16600_e28048_d_n2, assign16600_e28048_d_n3, assign16600_e28048_d_n4, assign16600_e28048_d_n5, assign16600_e28048_d_n6, assign16600_e28048_d_n7, assign16600_e28048_d_n8, assign16600_e28048_d_n9, assign16600_e28048_d_n10, assign16600_e28048_d_n11, assign16600_e28048_d_n13, assign16600_e28048_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard288 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign16600_e27941: f64 = (locals.var_at_i * locals.var_deltemp);
        let assign16600_e27942: f64 = (1.0 - assign16600_e27941);
        let assign16600_e27945: f64 = (p.p561 * locals.var_deltemp1);
        let assign16600_e27947: f64 = (assign16600_e27945 * locals.var_deltemp1);
        let assign16600_e27948: f64 = (assign16600_e27942 + assign16600_e27947);
        let assign16600_e27950: f64 = (assign16600_e27948 - 1e-6);
        let assign16600_e27952: f64 = (-10000.0);
        let assign16600_e27954: f64 = (assign16600_e27952 * 0.001);
        let (assign16600_e28045, assign16600_e28045_d_n4,) = {
            if (!(assign16600_e27950 < assign16600_e27954)) {
                let assign16600_e27961: f64 = (locals.var_at_i * locals.var_deltemp);
                let assign16600_e27962: f64 = (1.0 - assign16600_e27961);
                let assign16600_e27965: f64 = (p.p561 * locals.var_deltemp1);
                let assign16600_e27967: f64 = (assign16600_e27965 * locals.var_deltemp1);
                let assign16600_e27968: f64 = (assign16600_e27962 + assign16600_e27967);
                let assign16600_e27970: f64 = (assign16600_e27968 - 1e-6);
                let assign16600_e27974: f64 = (locals.var_at_i * locals.var_deltemp);
                let assign16600_e27975: f64 = (1.0 - assign16600_e27974);
                let assign16600_e27978: f64 = (p.p561 * locals.var_deltemp1);
                let assign16600_e27980: f64 = (assign16600_e27978 * locals.var_deltemp1);
                let assign16600_e27981: f64 = (assign16600_e27975 + assign16600_e27980);
                let assign16600_e27983: f64 = (assign16600_e27981 - 1e-6);
                let assign16600_e27987: f64 = (locals.var_at_i * locals.var_deltemp);
                let assign16600_e27988: f64 = (1.0 - assign16600_e27987);
                let assign16600_e27991: f64 = (p.p561 * locals.var_deltemp1);
                let assign16600_e27993: f64 = (assign16600_e27991 * locals.var_deltemp1);
                let assign16600_e27994: f64 = (assign16600_e27988 + assign16600_e27993);
                let assign16600_e27996: f64 = (assign16600_e27994 - 1e-6);
                let assign16600_e27997: f64 = (assign16600_e27983 * assign16600_e27996);
                let assign16600_e28000: f64 = (4.0 * 0.001);
                let assign16600_e28002: f64 = (assign16600_e28000 * 0.001);
                let assign16600_e28003: f64 = (assign16600_e27997 + assign16600_e28002);
                let assign16600_e28004: f64 = (assign16600_e28003).sqrt();
                let assign16600_e28005: f64 = (assign16600_e27970 + assign16600_e28004);
                let assign16600_e28006: f64 = (0.5 * assign16600_e28005);
                (assign16600_e28006, (0.5 * (((-(locals.var_at_i * locals.var_deltemp_dn4)) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16600_e27965 * locals.var_deltemp1_dn4))) + (((((-(locals.var_at_i * locals.var_deltemp_dn4)) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16600_e27978 * locals.var_deltemp1_dn4))) * assign16600_e27996) + (assign16600_e27983 * ((-(locals.var_at_i * locals.var_deltemp_dn4)) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16600_e27991 * locals.var_deltemp1_dn4))))) / (2.0 * assign16600_e28004)))),)
            } else {
                let assign16600_e28010: f64 = (locals.var_at_i * locals.var_deltemp);
                let assign16600_e28011: f64 = (1.0 - assign16600_e28010);
                let assign16600_e28014: f64 = (p.p561 * locals.var_deltemp1);
                let assign16600_e28016: f64 = (assign16600_e28014 * locals.var_deltemp1);
                let assign16600_e28017: f64 = (assign16600_e28011 + assign16600_e28016);
                let assign16600_e28019: f64 = (assign16600_e28017 - 1e-6);
                let assign16600_e28021: f64 = (-10000.0);
                let assign16600_e28023: f64 = (assign16600_e28021 * 0.001);
                let (assign16600_e28044, assign16600_e28044_d_n4,) = {
                    if (assign16600_e28019 < assign16600_e28023) {
                        let assign16600_e28026: f64 = (-0.001);
                        let assign16600_e28028: f64 = (assign16600_e28026 * 0.001);
                        let assign16600_e28032: f64 = (locals.var_at_i * locals.var_deltemp);
                        let assign16600_e28033: f64 = (1.0 - assign16600_e28032);
                        let assign16600_e28036: f64 = (p.p561 * locals.var_deltemp1);
                        let assign16600_e28038: f64 = (assign16600_e28036 * locals.var_deltemp1);
                        let assign16600_e28039: f64 = (assign16600_e28033 + assign16600_e28038);
                        let assign16600_e28041: f64 = (assign16600_e28039 - 1e-6);
                        let assign16600_e28042: f64 = (assign16600_e28028 / assign16600_e28041);
                        (assign16600_e28042, (-((assign16600_e28028 * ((-(locals.var_at_i * locals.var_deltemp_dn4)) + (((p.p561 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16600_e28036 * locals.var_deltemp1_dn4)))) / (assign16600_e28041 * assign16600_e28041))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign16600_e28044, assign16600_e28044_d_n4,)
            }
        };
        let assign16600_e28046: f64 = (locals.var_vsat1r_i * assign16600_e28045);
        (assign16600_e28046, (locals.var_vsat1r_i_dn0 * assign16600_e28045), (locals.var_vsat1r_i_dn2 * assign16600_e28045), (locals.var_vsat1r_i_dn3 * assign16600_e28045), ((locals.var_vsat1r_i_dn4 * assign16600_e28045) + (locals.var_vsat1r_i * assign16600_e28045_d_n4)), (locals.var_vsat1r_i_dn5 * assign16600_e28045), (locals.var_vsat1r_i_dn6 * assign16600_e28045), (locals.var_vsat1r_i_dn7 * assign16600_e28045), (locals.var_vsat1r_i_dn8 * assign16600_e28045), (locals.var_vsat1r_i_dn9 * assign16600_e28045), (locals.var_vsat1r_i_dn10 * assign16600_e28045), (locals.var_vsat1r_i_dn11 * assign16600_e28045), (locals.var_vsat1r_i_dn13 * assign16600_e28045), (locals.var_vsat1r_i_dn14 * assign16600_e28045),)
    } else {
        (locals.var_vsat1r_t, locals.var_vsat1r_t_dn0, locals.var_vsat1r_t_dn2, locals.var_vsat1r_t_dn3, locals.var_vsat1r_t_dn4, locals.var_vsat1r_t_dn5, locals.var_vsat1r_t_dn6, locals.var_vsat1r_t_dn7, locals.var_vsat1r_t_dn8, locals.var_vsat1r_t_dn9, locals.var_vsat1r_t_dn10, locals.var_vsat1r_t_dn11, locals.var_vsat1r_t_dn13, locals.var_vsat1r_t_dn14,)
    }
};
        locals.var_vsat1r_t = assign16600_e28048;
        locals.var_vsat1r_t_dn0 = assign16600_e28048_d_n0;
        locals.var_vsat1r_t_dn2 = assign16600_e28048_d_n2;
        locals.var_vsat1r_t_dn3 = assign16600_e28048_d_n3;
        locals.var_vsat1r_t_dn4 = assign16600_e28048_d_n4;
        locals.var_vsat1r_t_dn5 = assign16600_e28048_d_n5;
        locals.var_vsat1r_t_dn6 = assign16600_e28048_d_n6;
        locals.var_vsat1r_t_dn7 = assign16600_e28048_d_n7;
        locals.var_vsat1r_t_dn8 = assign16600_e28048_d_n8;
        locals.var_vsat1r_t_dn9 = assign16600_e28048_d_n9;
        locals.var_vsat1r_t_dn10 = assign16600_e28048_d_n10;
        locals.var_vsat1r_t_dn11 = assign16600_e28048_d_n11;
        locals.var_vsat1r_t_dn13 = assign16600_e28048_d_n13;
        locals.var_vsat1r_t_dn14 = assign16600_e28048_d_n14;

        let assign16610_e28051: f64 = if locals.var_vsat1r_t < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard296 = assign16610_e28051;

        let (assign16620_e28066, assign16620_e28066_d_n0, assign16620_e28066_d_n2, assign16620_e28066_d_n3, assign16620_e28066_d_n4, assign16620_e28066_d_n5, assign16620_e28066_d_n6, assign16620_e28066_d_n7, assign16620_e28066_d_n8, assign16620_e28066_d_n9, assign16620_e28066_d_n10, assign16620_e28066_d_n11, assign16620_e28066_d_n13, assign16620_e28066_d_n14,) = {
    if (((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard288 == 0.0)) && (locals.var_guard295 != 0.0)) && (locals.var_guard296 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsat1r_t, locals.var_vsat1r_t_dn0, locals.var_vsat1r_t_dn2, locals.var_vsat1r_t_dn3, locals.var_vsat1r_t_dn4, locals.var_vsat1r_t_dn5, locals.var_vsat1r_t_dn6, locals.var_vsat1r_t_dn7, locals.var_vsat1r_t_dn8, locals.var_vsat1r_t_dn9, locals.var_vsat1r_t_dn10, locals.var_vsat1r_t_dn11, locals.var_vsat1r_t_dn13, locals.var_vsat1r_t_dn14,)
    }
};
        locals.var_vsat1r_t = assign16620_e28066;
        locals.var_vsat1r_t_dn0 = assign16620_e28066_d_n0;
        locals.var_vsat1r_t_dn2 = assign16620_e28066_d_n2;
        locals.var_vsat1r_t_dn3 = assign16620_e28066_d_n3;
        locals.var_vsat1r_t_dn4 = assign16620_e28066_d_n4;
        locals.var_vsat1r_t_dn5 = assign16620_e28066_d_n5;
        locals.var_vsat1r_t_dn6 = assign16620_e28066_d_n6;
        locals.var_vsat1r_t_dn7 = assign16620_e28066_d_n7;
        locals.var_vsat1r_t_dn8 = assign16620_e28066_d_n8;
        locals.var_vsat1r_t_dn9 = assign16620_e28066_d_n9;
        locals.var_vsat1r_t_dn10 = assign16620_e28066_d_n10;
        locals.var_vsat1r_t_dn11 = assign16620_e28066_d_n11;
        locals.var_vsat1r_t_dn13 = assign16620_e28066_d_n13;
        locals.var_vsat1r_t_dn14 = assign16620_e28066_d_n14;

        let (assign16630_e28186, assign16630_e28186_d_n0, assign16630_e28186_d_n2, assign16630_e28186_d_n3, assign16630_e28186_d_n4, assign16630_e28186_d_n5, assign16630_e28186_d_n6, assign16630_e28186_d_n7, assign16630_e28186_d_n8, assign16630_e28186_d_n9, assign16630_e28186_d_n10, assign16630_e28186_d_n11, assign16630_e28186_d_n13, assign16630_e28186_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard288 == 0.0)) {
        let assign16630_e28079: f64 = (locals.var_atcv_i * locals.var_deltemp);
        let assign16630_e28080: f64 = (1.0 - assign16630_e28079);
        let assign16630_e28083: f64 = (p.p574 * locals.var_deltemp1);
        let assign16630_e28085: f64 = (assign16630_e28083 * locals.var_deltemp1);
        let assign16630_e28086: f64 = (assign16630_e28080 + assign16630_e28085);
        let assign16630_e28088: f64 = (assign16630_e28086 - 1e-6);
        let assign16630_e28090: f64 = (-10000.0);
        let assign16630_e28092: f64 = (assign16630_e28090 * 0.001);
        let (assign16630_e28183, assign16630_e28183_d_n4,) = {
            if (!(assign16630_e28088 < assign16630_e28092)) {
                let assign16630_e28099: f64 = (locals.var_atcv_i * locals.var_deltemp);
                let assign16630_e28100: f64 = (1.0 - assign16630_e28099);
                let assign16630_e28103: f64 = (p.p574 * locals.var_deltemp1);
                let assign16630_e28105: f64 = (assign16630_e28103 * locals.var_deltemp1);
                let assign16630_e28106: f64 = (assign16630_e28100 + assign16630_e28105);
                let assign16630_e28108: f64 = (assign16630_e28106 - 1e-6);
                let assign16630_e28112: f64 = (locals.var_atcv_i * locals.var_deltemp);
                let assign16630_e28113: f64 = (1.0 - assign16630_e28112);
                let assign16630_e28116: f64 = (p.p574 * locals.var_deltemp1);
                let assign16630_e28118: f64 = (assign16630_e28116 * locals.var_deltemp1);
                let assign16630_e28119: f64 = (assign16630_e28113 + assign16630_e28118);
                let assign16630_e28121: f64 = (assign16630_e28119 - 1e-6);
                let assign16630_e28125: f64 = (locals.var_atcv_i * locals.var_deltemp);
                let assign16630_e28126: f64 = (1.0 - assign16630_e28125);
                let assign16630_e28129: f64 = (p.p574 * locals.var_deltemp1);
                let assign16630_e28131: f64 = (assign16630_e28129 * locals.var_deltemp1);
                let assign16630_e28132: f64 = (assign16630_e28126 + assign16630_e28131);
                let assign16630_e28134: f64 = (assign16630_e28132 - 1e-6);
                let assign16630_e28135: f64 = (assign16630_e28121 * assign16630_e28134);
                let assign16630_e28138: f64 = (4.0 * 0.001);
                let assign16630_e28140: f64 = (assign16630_e28138 * 0.001);
                let assign16630_e28141: f64 = (assign16630_e28135 + assign16630_e28140);
                let assign16630_e28142: f64 = (assign16630_e28141).sqrt();
                let assign16630_e28143: f64 = (assign16630_e28108 + assign16630_e28142);
                let assign16630_e28144: f64 = (0.5 * assign16630_e28143);
                (assign16630_e28144, (0.5 * (((-(locals.var_atcv_i * locals.var_deltemp_dn4)) + (((p.p574 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16630_e28103 * locals.var_deltemp1_dn4))) + (((((-(locals.var_atcv_i * locals.var_deltemp_dn4)) + (((p.p574 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16630_e28116 * locals.var_deltemp1_dn4))) * assign16630_e28134) + (assign16630_e28121 * ((-(locals.var_atcv_i * locals.var_deltemp_dn4)) + (((p.p574 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16630_e28129 * locals.var_deltemp1_dn4))))) / (2.0 * assign16630_e28142)))),)
            } else {
                let assign16630_e28148: f64 = (locals.var_atcv_i * locals.var_deltemp);
                let assign16630_e28149: f64 = (1.0 - assign16630_e28148);
                let assign16630_e28152: f64 = (p.p574 * locals.var_deltemp1);
                let assign16630_e28154: f64 = (assign16630_e28152 * locals.var_deltemp1);
                let assign16630_e28155: f64 = (assign16630_e28149 + assign16630_e28154);
                let assign16630_e28157: f64 = (assign16630_e28155 - 1e-6);
                let assign16630_e28159: f64 = (-10000.0);
                let assign16630_e28161: f64 = (assign16630_e28159 * 0.001);
                let (assign16630_e28182, assign16630_e28182_d_n4,) = {
                    if (assign16630_e28157 < assign16630_e28161) {
                        let assign16630_e28164: f64 = (-0.001);
                        let assign16630_e28166: f64 = (assign16630_e28164 * 0.001);
                        let assign16630_e28170: f64 = (locals.var_atcv_i * locals.var_deltemp);
                        let assign16630_e28171: f64 = (1.0 - assign16630_e28170);
                        let assign16630_e28174: f64 = (p.p574 * locals.var_deltemp1);
                        let assign16630_e28176: f64 = (assign16630_e28174 * locals.var_deltemp1);
                        let assign16630_e28177: f64 = (assign16630_e28171 + assign16630_e28176);
                        let assign16630_e28179: f64 = (assign16630_e28177 - 1e-6);
                        let assign16630_e28180: f64 = (assign16630_e28166 / assign16630_e28179);
                        (assign16630_e28180, (-((assign16630_e28166 * ((-(locals.var_atcv_i * locals.var_deltemp_dn4)) + (((p.p574 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16630_e28174 * locals.var_deltemp1_dn4)))) / (assign16630_e28179 * assign16630_e28179))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign16630_e28182, assign16630_e28182_d_n4,)
            }
        };
        let assign16630_e28184: f64 = (locals.var_vsatcv_i * assign16630_e28183);
        (assign16630_e28184, (locals.var_vsatcv_i_dn0 * assign16630_e28183), (locals.var_vsatcv_i_dn2 * assign16630_e28183), (locals.var_vsatcv_i_dn3 * assign16630_e28183), ((locals.var_vsatcv_i_dn4 * assign16630_e28183) + (locals.var_vsatcv_i * assign16630_e28183_d_n4)), (locals.var_vsatcv_i_dn5 * assign16630_e28183), (locals.var_vsatcv_i_dn6 * assign16630_e28183), (locals.var_vsatcv_i_dn7 * assign16630_e28183), (locals.var_vsatcv_i_dn8 * assign16630_e28183), (locals.var_vsatcv_i_dn9 * assign16630_e28183), (locals.var_vsatcv_i_dn10 * assign16630_e28183), (locals.var_vsatcv_i_dn11 * assign16630_e28183), (locals.var_vsatcv_i_dn13 * assign16630_e28183), (locals.var_vsatcv_i_dn14 * assign16630_e28183),)
    } else {
        (locals.var_vsatcv_t, locals.var_vsatcv_t_dn0, locals.var_vsatcv_t_dn2, locals.var_vsatcv_t_dn3, locals.var_vsatcv_t_dn4, locals.var_vsatcv_t_dn5, locals.var_vsatcv_t_dn6, locals.var_vsatcv_t_dn7, locals.var_vsatcv_t_dn8, locals.var_vsatcv_t_dn9, locals.var_vsatcv_t_dn10, locals.var_vsatcv_t_dn11, locals.var_vsatcv_t_dn13, locals.var_vsatcv_t_dn14,)
    }
};
        locals.var_vsatcv_t = assign16630_e28186;
        locals.var_vsatcv_t_dn0 = assign16630_e28186_d_n0;
        locals.var_vsatcv_t_dn2 = assign16630_e28186_d_n2;
        locals.var_vsatcv_t_dn3 = assign16630_e28186_d_n3;
        locals.var_vsatcv_t_dn4 = assign16630_e28186_d_n4;
        locals.var_vsatcv_t_dn5 = assign16630_e28186_d_n5;
        locals.var_vsatcv_t_dn6 = assign16630_e28186_d_n6;
        locals.var_vsatcv_t_dn7 = assign16630_e28186_d_n7;
        locals.var_vsatcv_t_dn8 = assign16630_e28186_d_n8;
        locals.var_vsatcv_t_dn9 = assign16630_e28186_d_n9;
        locals.var_vsatcv_t_dn10 = assign16630_e28186_d_n10;
        locals.var_vsatcv_t_dn11 = assign16630_e28186_d_n11;
        locals.var_vsatcv_t_dn13 = assign16630_e28186_d_n13;
        locals.var_vsatcv_t_dn14 = assign16630_e28186_d_n14;

    }

    pub(super) fn stamp_transient_block_52(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign16640_e28306, assign16640_e28306_d_n4,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard288 == 0.0)) {
        let assign16640_e28199: f64 = (p.p498 * locals.var_deltemp1);
        let assign16640_e28200: f64 = (1.0 + assign16640_e28199);
        let assign16640_e28203: f64 = (p.p499 * locals.var_deltemp1);
        let assign16640_e28205: f64 = (assign16640_e28203 * locals.var_deltemp1);
        let assign16640_e28206: f64 = (assign16640_e28200 + assign16640_e28205);
        let assign16640_e28208: f64 = (assign16640_e28206 - 1e-6);
        let assign16640_e28210: f64 = (-10000.0);
        let assign16640_e28212: f64 = (assign16640_e28210 * 0.001);
        let (assign16640_e28303, assign16640_e28303_d_n4,) = {
            if (!(assign16640_e28208 < assign16640_e28212)) {
                let assign16640_e28219: f64 = (p.p498 * locals.var_deltemp1);
                let assign16640_e28220: f64 = (1.0 + assign16640_e28219);
                let assign16640_e28223: f64 = (p.p499 * locals.var_deltemp1);
                let assign16640_e28225: f64 = (assign16640_e28223 * locals.var_deltemp1);
                let assign16640_e28226: f64 = (assign16640_e28220 + assign16640_e28225);
                let assign16640_e28228: f64 = (assign16640_e28226 - 1e-6);
                let assign16640_e28232: f64 = (p.p498 * locals.var_deltemp1);
                let assign16640_e28233: f64 = (1.0 + assign16640_e28232);
                let assign16640_e28236: f64 = (p.p499 * locals.var_deltemp1);
                let assign16640_e28238: f64 = (assign16640_e28236 * locals.var_deltemp1);
                let assign16640_e28239: f64 = (assign16640_e28233 + assign16640_e28238);
                let assign16640_e28241: f64 = (assign16640_e28239 - 1e-6);
                let assign16640_e28245: f64 = (p.p498 * locals.var_deltemp1);
                let assign16640_e28246: f64 = (1.0 + assign16640_e28245);
                let assign16640_e28249: f64 = (p.p499 * locals.var_deltemp1);
                let assign16640_e28251: f64 = (assign16640_e28249 * locals.var_deltemp1);
                let assign16640_e28252: f64 = (assign16640_e28246 + assign16640_e28251);
                let assign16640_e28254: f64 = (assign16640_e28252 - 1e-6);
                let assign16640_e28255: f64 = (assign16640_e28241 * assign16640_e28254);
                let assign16640_e28258: f64 = (4.0 * 0.001);
                let assign16640_e28260: f64 = (assign16640_e28258 * 0.001);
                let assign16640_e28261: f64 = (assign16640_e28255 + assign16640_e28260);
                let assign16640_e28262: f64 = (assign16640_e28261).sqrt();
                let assign16640_e28263: f64 = (assign16640_e28228 + assign16640_e28262);
                let assign16640_e28264: f64 = (0.5 * assign16640_e28263);
                (assign16640_e28264, (0.5 * (((p.p498 * locals.var_deltemp1_dn4) + (((p.p499 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16640_e28223 * locals.var_deltemp1_dn4))) + (((((p.p498 * locals.var_deltemp1_dn4) + (((p.p499 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16640_e28236 * locals.var_deltemp1_dn4))) * assign16640_e28254) + (assign16640_e28241 * ((p.p498 * locals.var_deltemp1_dn4) + (((p.p499 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16640_e28249 * locals.var_deltemp1_dn4))))) / (2.0 * assign16640_e28262)))),)
            } else {
                let assign16640_e28268: f64 = (p.p498 * locals.var_deltemp1);
                let assign16640_e28269: f64 = (1.0 + assign16640_e28268);
                let assign16640_e28272: f64 = (p.p499 * locals.var_deltemp1);
                let assign16640_e28274: f64 = (assign16640_e28272 * locals.var_deltemp1);
                let assign16640_e28275: f64 = (assign16640_e28269 + assign16640_e28274);
                let assign16640_e28277: f64 = (assign16640_e28275 - 1e-6);
                let assign16640_e28279: f64 = (-10000.0);
                let assign16640_e28281: f64 = (assign16640_e28279 * 0.001);
                let (assign16640_e28302, assign16640_e28302_d_n4,) = {
                    if (assign16640_e28277 < assign16640_e28281) {
                        let assign16640_e28284: f64 = (-0.001);
                        let assign16640_e28286: f64 = (assign16640_e28284 * 0.001);
                        let assign16640_e28290: f64 = (p.p498 * locals.var_deltemp1);
                        let assign16640_e28291: f64 = (1.0 + assign16640_e28290);
                        let assign16640_e28294: f64 = (p.p499 * locals.var_deltemp1);
                        let assign16640_e28296: f64 = (assign16640_e28294 * locals.var_deltemp1);
                        let assign16640_e28297: f64 = (assign16640_e28291 + assign16640_e28296);
                        let assign16640_e28299: f64 = (assign16640_e28297 - 1e-6);
                        let assign16640_e28300: f64 = (assign16640_e28286 / assign16640_e28299);
                        (assign16640_e28300, (-((assign16640_e28286 * ((p.p498 * locals.var_deltemp1_dn4) + (((p.p499 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16640_e28294 * locals.var_deltemp1_dn4)))) / (assign16640_e28299 * assign16640_e28299))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign16640_e28302, assign16640_e28302_d_n4,)
            }
        };
        let assign16640_e28304: f64 = (locals.var_ksativ_i * assign16640_e28303);
        (assign16640_e28304, (locals.var_ksativ_i * assign16640_e28303_d_n4),)
    } else {
        (locals.var_ksativ_t, locals.var_ksativ_t_dn4,)
    }
};
        locals.var_ksativ_t = assign16640_e28306;
        locals.var_ksativ_t_dn4 = assign16640_e28306_d_n4;

        let (assign16650_e28390, assign16650_e28390_d_n0, assign16650_e28390_d_n2, assign16650_e28390_d_n3, assign16650_e28390_d_n4, assign16650_e28390_d_n5, assign16650_e28390_d_n6, assign16650_e28390_d_n7, assign16650_e28390_d_n8, assign16650_e28390_d_n9, assign16650_e28390_d_n10, assign16650_e28390_d_n11, assign16650_e28390_d_n13, assign16650_e28390_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard288 == 0.0)) {
        let assign16650_e28319: f64 = (p.p1026 * locals.var_deltemp1);
        let assign16650_e28320: f64 = (1.0 + assign16650_e28319);
        let assign16650_e28322: f64 = (assign16650_e28320 - 1e-6);
        let assign16650_e28324: f64 = (-10000.0);
        let assign16650_e28326: f64 = (assign16650_e28324 * 0.001);
        let (assign16650_e28387, assign16650_e28387_d_n4,) = {
            if (!(assign16650_e28322 < assign16650_e28326)) {
                let assign16650_e28333: f64 = (p.p1026 * locals.var_deltemp1);
                let assign16650_e28334: f64 = (1.0 + assign16650_e28333);
                let assign16650_e28336: f64 = (assign16650_e28334 - 1e-6);
                let assign16650_e28340: f64 = (p.p1026 * locals.var_deltemp1);
                let assign16650_e28341: f64 = (1.0 + assign16650_e28340);
                let assign16650_e28343: f64 = (assign16650_e28341 - 1e-6);
                let assign16650_e28347: f64 = (p.p1026 * locals.var_deltemp1);
                let assign16650_e28348: f64 = (1.0 + assign16650_e28347);
                let assign16650_e28350: f64 = (assign16650_e28348 - 1e-6);
                let assign16650_e28351: f64 = (assign16650_e28343 * assign16650_e28350);
                let assign16650_e28354: f64 = (4.0 * 0.001);
                let assign16650_e28356: f64 = (assign16650_e28354 * 0.001);
                let assign16650_e28357: f64 = (assign16650_e28351 + assign16650_e28356);
                let assign16650_e28358: f64 = (assign16650_e28357).sqrt();
                let assign16650_e28359: f64 = (assign16650_e28336 + assign16650_e28358);
                let assign16650_e28360: f64 = (0.5 * assign16650_e28359);
                (assign16650_e28360, (0.5 * ((p.p1026 * locals.var_deltemp1_dn4) + ((((p.p1026 * locals.var_deltemp1_dn4) * assign16650_e28350) + (assign16650_e28343 * (p.p1026 * locals.var_deltemp1_dn4))) / (2.0 * assign16650_e28358)))),)
            } else {
                let assign16650_e28364: f64 = (p.p1026 * locals.var_deltemp1);
                let assign16650_e28365: f64 = (1.0 + assign16650_e28364);
                let assign16650_e28367: f64 = (assign16650_e28365 - 1e-6);
                let assign16650_e28369: f64 = (-10000.0);
                let assign16650_e28371: f64 = (assign16650_e28369 * 0.001);
                let (assign16650_e28386, assign16650_e28386_d_n4,) = {
                    if (assign16650_e28367 < assign16650_e28371) {
                        let assign16650_e28374: f64 = (-0.001);
                        let assign16650_e28376: f64 = (assign16650_e28374 * 0.001);
                        let assign16650_e28380: f64 = (p.p1026 * locals.var_deltemp1);
                        let assign16650_e28381: f64 = (1.0 + assign16650_e28380);
                        let assign16650_e28383: f64 = (assign16650_e28381 - 1e-6);
                        let assign16650_e28384: f64 = (assign16650_e28376 / assign16650_e28383);
                        (assign16650_e28384, (-((assign16650_e28376 * (p.p1026 * locals.var_deltemp1_dn4)) / (assign16650_e28383 * assign16650_e28383))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign16650_e28386, assign16650_e28386_d_n4,)
            }
        };
        let assign16650_e28388: f64 = (locals.var_pclm_i * assign16650_e28387);
        (assign16650_e28388, (locals.var_pclm_i_dn0 * assign16650_e28387), (locals.var_pclm_i_dn2 * assign16650_e28387), (locals.var_pclm_i_dn3 * assign16650_e28387), ((locals.var_pclm_i_dn4 * assign16650_e28387) + (locals.var_pclm_i * assign16650_e28387_d_n4)), (locals.var_pclm_i_dn5 * assign16650_e28387), (locals.var_pclm_i_dn6 * assign16650_e28387), (locals.var_pclm_i_dn7 * assign16650_e28387), (locals.var_pclm_i_dn8 * assign16650_e28387), (locals.var_pclm_i_dn9 * assign16650_e28387), (locals.var_pclm_i_dn10 * assign16650_e28387), (locals.var_pclm_i_dn11 * assign16650_e28387), (locals.var_pclm_i_dn13 * assign16650_e28387), (locals.var_pclm_i_dn14 * assign16650_e28387),)
    } else {
        (locals.var_pclm_t, locals.var_pclm_t_dn0, locals.var_pclm_t_dn2, locals.var_pclm_t_dn3, locals.var_pclm_t_dn4, locals.var_pclm_t_dn5, locals.var_pclm_t_dn6, locals.var_pclm_t_dn7, locals.var_pclm_t_dn8, locals.var_pclm_t_dn9, locals.var_pclm_t_dn10, locals.var_pclm_t_dn11, locals.var_pclm_t_dn13, locals.var_pclm_t_dn14,)
    }
};
        locals.var_pclm_t = assign16650_e28390;
        locals.var_pclm_t_dn0 = assign16650_e28390_d_n0;
        locals.var_pclm_t_dn2 = assign16650_e28390_d_n2;
        locals.var_pclm_t_dn3 = assign16650_e28390_d_n3;
        locals.var_pclm_t_dn4 = assign16650_e28390_d_n4;
        locals.var_pclm_t_dn5 = assign16650_e28390_d_n5;
        locals.var_pclm_t_dn6 = assign16650_e28390_d_n6;
        locals.var_pclm_t_dn7 = assign16650_e28390_d_n7;
        locals.var_pclm_t_dn8 = assign16650_e28390_d_n8;
        locals.var_pclm_t_dn9 = assign16650_e28390_d_n9;
        locals.var_pclm_t_dn10 = assign16650_e28390_d_n10;
        locals.var_pclm_t_dn11 = assign16650_e28390_d_n11;
        locals.var_pclm_t_dn13 = assign16650_e28390_d_n13;
        locals.var_pclm_t_dn14 = assign16650_e28390_d_n14;

        let (assign16660_e28519, assign16660_e28519_d_n0, assign16660_e28519_d_n2, assign16660_e28519_d_n3, assign16660_e28519_d_n4, assign16660_e28519_d_n5, assign16660_e28519_d_n6, assign16660_e28519_d_n7, assign16660_e28519_d_n8, assign16660_e28519_d_n9, assign16660_e28519_d_n10, assign16660_e28519_d_n11, assign16660_e28519_d_n13, assign16660_e28519_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) {
        let assign16660_e28400: f64 = (p.p450 * locals.var_deltemp);
        let assign16660_e28401: f64 = (1.0 + assign16660_e28400);
        let assign16660_e28404: f64 = (p.p451 * locals.var_deltemp1);
        let assign16660_e28406: f64 = (assign16660_e28404 * locals.var_deltemp1);
        let assign16660_e28407: f64 = (assign16660_e28401 + assign16660_e28406);
        let assign16660_e28408: f64 = (locals.var_mexp_i * assign16660_e28407);
        let assign16660_e28410: f64 = (assign16660_e28408 - 2.0);
        let assign16660_e28412: f64 = (-10000.0);
        let assign16660_e28414: f64 = (assign16660_e28412 * 0.001);
        let (assign16660_e28515, assign16660_e28515_d_n0, assign16660_e28515_d_n2, assign16660_e28515_d_n3, assign16660_e28515_d_n4, assign16660_e28515_d_n5, assign16660_e28515_d_n6, assign16660_e28515_d_n7, assign16660_e28515_d_n8, assign16660_e28515_d_n9, assign16660_e28515_d_n10, assign16660_e28515_d_n11, assign16660_e28515_d_n13, assign16660_e28515_d_n14,) = {
            if (!(assign16660_e28410 < assign16660_e28414)) {
                let assign16660_e28422: f64 = (p.p450 * locals.var_deltemp);
                let assign16660_e28423: f64 = (1.0 + assign16660_e28422);
                let assign16660_e28426: f64 = (p.p451 * locals.var_deltemp1);
                let assign16660_e28428: f64 = (assign16660_e28426 * locals.var_deltemp1);
                let assign16660_e28429: f64 = (assign16660_e28423 + assign16660_e28428);
                let assign16660_e28430: f64 = (locals.var_mexp_i * assign16660_e28429);
                let assign16660_e28432: f64 = (assign16660_e28430 - 2.0);
                let assign16660_e28437: f64 = (p.p450 * locals.var_deltemp);
                let assign16660_e28438: f64 = (1.0 + assign16660_e28437);
                let assign16660_e28441: f64 = (p.p451 * locals.var_deltemp1);
                let assign16660_e28443: f64 = (assign16660_e28441 * locals.var_deltemp1);
                let assign16660_e28444: f64 = (assign16660_e28438 + assign16660_e28443);
                let assign16660_e28445: f64 = (locals.var_mexp_i * assign16660_e28444);
                let assign16660_e28447: f64 = (assign16660_e28445 - 2.0);
                let assign16660_e28452: f64 = (p.p450 * locals.var_deltemp);
                let assign16660_e28453: f64 = (1.0 + assign16660_e28452);
                let assign16660_e28456: f64 = (p.p451 * locals.var_deltemp1);
                let assign16660_e28458: f64 = (assign16660_e28456 * locals.var_deltemp1);
                let assign16660_e28459: f64 = (assign16660_e28453 + assign16660_e28458);
                let assign16660_e28460: f64 = (locals.var_mexp_i * assign16660_e28459);
                let assign16660_e28462: f64 = (assign16660_e28460 - 2.0);
                let assign16660_e28463: f64 = (assign16660_e28447 * assign16660_e28462);
                let assign16660_e28466: f64 = (4.0 * 0.001);
                let assign16660_e28468: f64 = (assign16660_e28466 * 0.001);
                let assign16660_e28469: f64 = (assign16660_e28463 + assign16660_e28468);
                let assign16660_e28470: f64 = (assign16660_e28469).sqrt();
                let assign16660_e28471: f64 = (assign16660_e28432 + assign16660_e28470);
                let assign16660_e28472: f64 = (0.5 * assign16660_e28471);
                (assign16660_e28472, (0.5 * ((locals.var_mexp_i_dn0 * assign16660_e28429) + ((((locals.var_mexp_i_dn0 * assign16660_e28444) * assign16660_e28462) + (assign16660_e28447 * (locals.var_mexp_i_dn0 * assign16660_e28459))) / (2.0 * assign16660_e28470)))), (0.5 * ((locals.var_mexp_i_dn2 * assign16660_e28429) + ((((locals.var_mexp_i_dn2 * assign16660_e28444) * assign16660_e28462) + (assign16660_e28447 * (locals.var_mexp_i_dn2 * assign16660_e28459))) / (2.0 * assign16660_e28470)))), (0.5 * ((locals.var_mexp_i_dn3 * assign16660_e28429) + ((((locals.var_mexp_i_dn3 * assign16660_e28444) * assign16660_e28462) + (assign16660_e28447 * (locals.var_mexp_i_dn3 * assign16660_e28459))) / (2.0 * assign16660_e28470)))), (0.5 * (((locals.var_mexp_i_dn4 * assign16660_e28429) + (locals.var_mexp_i * ((p.p450 * locals.var_deltemp_dn4) + (((p.p451 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16660_e28426 * locals.var_deltemp1_dn4))))) + (((((locals.var_mexp_i_dn4 * assign16660_e28444) + (locals.var_mexp_i * ((p.p450 * locals.var_deltemp_dn4) + (((p.p451 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16660_e28441 * locals.var_deltemp1_dn4))))) * assign16660_e28462) + (assign16660_e28447 * ((locals.var_mexp_i_dn4 * assign16660_e28459) + (locals.var_mexp_i * ((p.p450 * locals.var_deltemp_dn4) + (((p.p451 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16660_e28456 * locals.var_deltemp1_dn4))))))) / (2.0 * assign16660_e28470)))), (0.5 * ((locals.var_mexp_i_dn5 * assign16660_e28429) + ((((locals.var_mexp_i_dn5 * assign16660_e28444) * assign16660_e28462) + (assign16660_e28447 * (locals.var_mexp_i_dn5 * assign16660_e28459))) / (2.0 * assign16660_e28470)))), (0.5 * ((locals.var_mexp_i_dn6 * assign16660_e28429) + ((((locals.var_mexp_i_dn6 * assign16660_e28444) * assign16660_e28462) + (assign16660_e28447 * (locals.var_mexp_i_dn6 * assign16660_e28459))) / (2.0 * assign16660_e28470)))), (0.5 * ((locals.var_mexp_i_dn7 * assign16660_e28429) + ((((locals.var_mexp_i_dn7 * assign16660_e28444) * assign16660_e28462) + (assign16660_e28447 * (locals.var_mexp_i_dn7 * assign16660_e28459))) / (2.0 * assign16660_e28470)))), (0.5 * ((locals.var_mexp_i_dn8 * assign16660_e28429) + ((((locals.var_mexp_i_dn8 * assign16660_e28444) * assign16660_e28462) + (assign16660_e28447 * (locals.var_mexp_i_dn8 * assign16660_e28459))) / (2.0 * assign16660_e28470)))), (0.5 * ((locals.var_mexp_i_dn9 * assign16660_e28429) + ((((locals.var_mexp_i_dn9 * assign16660_e28444) * assign16660_e28462) + (assign16660_e28447 * (locals.var_mexp_i_dn9 * assign16660_e28459))) / (2.0 * assign16660_e28470)))), (0.5 * ((locals.var_mexp_i_dn10 * assign16660_e28429) + ((((locals.var_mexp_i_dn10 * assign16660_e28444) * assign16660_e28462) + (assign16660_e28447 * (locals.var_mexp_i_dn10 * assign16660_e28459))) / (2.0 * assign16660_e28470)))), (0.5 * ((locals.var_mexp_i_dn11 * assign16660_e28429) + ((((locals.var_mexp_i_dn11 * assign16660_e28444) * assign16660_e28462) + (assign16660_e28447 * (locals.var_mexp_i_dn11 * assign16660_e28459))) / (2.0 * assign16660_e28470)))), (0.5 * ((locals.var_mexp_i_dn13 * assign16660_e28429) + ((((locals.var_mexp_i_dn13 * assign16660_e28444) * assign16660_e28462) + (assign16660_e28447 * (locals.var_mexp_i_dn13 * assign16660_e28459))) / (2.0 * assign16660_e28470)))), (0.5 * ((locals.var_mexp_i_dn14 * assign16660_e28429) + ((((locals.var_mexp_i_dn14 * assign16660_e28444) * assign16660_e28462) + (assign16660_e28447 * (locals.var_mexp_i_dn14 * assign16660_e28459))) / (2.0 * assign16660_e28470)))),)
            } else {
                let assign16660_e28477: f64 = (p.p450 * locals.var_deltemp);
                let assign16660_e28478: f64 = (1.0 + assign16660_e28477);
                let assign16660_e28481: f64 = (p.p451 * locals.var_deltemp1);
                let assign16660_e28483: f64 = (assign16660_e28481 * locals.var_deltemp1);
                let assign16660_e28484: f64 = (assign16660_e28478 + assign16660_e28483);
                let assign16660_e28485: f64 = (locals.var_mexp_i * assign16660_e28484);
                let assign16660_e28487: f64 = (assign16660_e28485 - 2.0);
                let assign16660_e28489: f64 = (-10000.0);
                let assign16660_e28491: f64 = (assign16660_e28489 * 0.001);
                let (assign16660_e28514, assign16660_e28514_d_n0, assign16660_e28514_d_n2, assign16660_e28514_d_n3, assign16660_e28514_d_n4, assign16660_e28514_d_n5, assign16660_e28514_d_n6, assign16660_e28514_d_n7, assign16660_e28514_d_n8, assign16660_e28514_d_n9, assign16660_e28514_d_n10, assign16660_e28514_d_n11, assign16660_e28514_d_n13, assign16660_e28514_d_n14,) = {
                    if (assign16660_e28487 < assign16660_e28491) {
                        let assign16660_e28494: f64 = (-0.001);
                        let assign16660_e28496: f64 = (assign16660_e28494 * 0.001);
                        let assign16660_e28501: f64 = (p.p450 * locals.var_deltemp);
                        let assign16660_e28502: f64 = (1.0 + assign16660_e28501);
                        let assign16660_e28505: f64 = (p.p451 * locals.var_deltemp1);
                        let assign16660_e28507: f64 = (assign16660_e28505 * locals.var_deltemp1);
                        let assign16660_e28508: f64 = (assign16660_e28502 + assign16660_e28507);
                        let assign16660_e28509: f64 = (locals.var_mexp_i * assign16660_e28508);
                        let assign16660_e28511: f64 = (assign16660_e28509 - 2.0);
                        let assign16660_e28512: f64 = (assign16660_e28496 / assign16660_e28511);
                        (assign16660_e28512, (-((assign16660_e28496 * (locals.var_mexp_i_dn0 * assign16660_e28508)) / (assign16660_e28511 * assign16660_e28511))), (-((assign16660_e28496 * (locals.var_mexp_i_dn2 * assign16660_e28508)) / (assign16660_e28511 * assign16660_e28511))), (-((assign16660_e28496 * (locals.var_mexp_i_dn3 * assign16660_e28508)) / (assign16660_e28511 * assign16660_e28511))), (-((assign16660_e28496 * ((locals.var_mexp_i_dn4 * assign16660_e28508) + (locals.var_mexp_i * ((p.p450 * locals.var_deltemp_dn4) + (((p.p451 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16660_e28505 * locals.var_deltemp1_dn4)))))) / (assign16660_e28511 * assign16660_e28511))), (-((assign16660_e28496 * (locals.var_mexp_i_dn5 * assign16660_e28508)) / (assign16660_e28511 * assign16660_e28511))), (-((assign16660_e28496 * (locals.var_mexp_i_dn6 * assign16660_e28508)) / (assign16660_e28511 * assign16660_e28511))), (-((assign16660_e28496 * (locals.var_mexp_i_dn7 * assign16660_e28508)) / (assign16660_e28511 * assign16660_e28511))), (-((assign16660_e28496 * (locals.var_mexp_i_dn8 * assign16660_e28508)) / (assign16660_e28511 * assign16660_e28511))), (-((assign16660_e28496 * (locals.var_mexp_i_dn9 * assign16660_e28508)) / (assign16660_e28511 * assign16660_e28511))), (-((assign16660_e28496 * (locals.var_mexp_i_dn10 * assign16660_e28508)) / (assign16660_e28511 * assign16660_e28511))), (-((assign16660_e28496 * (locals.var_mexp_i_dn11 * assign16660_e28508)) / (assign16660_e28511 * assign16660_e28511))), (-((assign16660_e28496 * (locals.var_mexp_i_dn13 * assign16660_e28508)) / (assign16660_e28511 * assign16660_e28511))), (-((assign16660_e28496 * (locals.var_mexp_i_dn14 * assign16660_e28508)) / (assign16660_e28511 * assign16660_e28511))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign16660_e28514, assign16660_e28514_d_n0, assign16660_e28514_d_n2, assign16660_e28514_d_n3, assign16660_e28514_d_n4, assign16660_e28514_d_n5, assign16660_e28514_d_n6, assign16660_e28514_d_n7, assign16660_e28514_d_n8, assign16660_e28514_d_n9, assign16660_e28514_d_n10, assign16660_e28514_d_n11, assign16660_e28514_d_n13, assign16660_e28514_d_n14,)
            }
        };
        let assign16660_e28517: f64 = (assign16660_e28515 + 2.0);
        (assign16660_e28517, assign16660_e28515_d_n0, assign16660_e28515_d_n2, assign16660_e28515_d_n3, assign16660_e28515_d_n4, assign16660_e28515_d_n5, assign16660_e28515_d_n6, assign16660_e28515_d_n7, assign16660_e28515_d_n8, assign16660_e28515_d_n9, assign16660_e28515_d_n10, assign16660_e28515_d_n11, assign16660_e28515_d_n13, assign16660_e28515_d_n14,)
    } else {
        (locals.var_mexp_t, locals.var_mexp_t_dn0, locals.var_mexp_t_dn2, locals.var_mexp_t_dn3, locals.var_mexp_t_dn4, locals.var_mexp_t_dn5, locals.var_mexp_t_dn6, locals.var_mexp_t_dn7, locals.var_mexp_t_dn8, locals.var_mexp_t_dn9, locals.var_mexp_t_dn10, locals.var_mexp_t_dn11, locals.var_mexp_t_dn13, locals.var_mexp_t_dn14,)
    }
};
        locals.var_mexp_t = assign16660_e28519;
        locals.var_mexp_t_dn0 = assign16660_e28519_d_n0;
        locals.var_mexp_t_dn2 = assign16660_e28519_d_n2;
        locals.var_mexp_t_dn3 = assign16660_e28519_d_n3;
        locals.var_mexp_t_dn4 = assign16660_e28519_d_n4;
        locals.var_mexp_t_dn5 = assign16660_e28519_d_n5;
        locals.var_mexp_t_dn6 = assign16660_e28519_d_n6;
        locals.var_mexp_t_dn7 = assign16660_e28519_d_n7;
        locals.var_mexp_t_dn8 = assign16660_e28519_d_n8;
        locals.var_mexp_t_dn9 = assign16660_e28519_d_n9;
        locals.var_mexp_t_dn10 = assign16660_e28519_d_n10;
        locals.var_mexp_t_dn11 = assign16660_e28519_d_n11;
        locals.var_mexp_t_dn13 = assign16660_e28519_d_n13;
        locals.var_mexp_t_dn14 = assign16660_e28519_d_n14;

        let assign16670_e28522: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard297 = assign16670_e28522;

        let (assign16680_e28653, assign16680_e28653_d_n0, assign16680_e28653_d_n2, assign16680_e28653_d_n3, assign16680_e28653_d_n4, assign16680_e28653_d_n5, assign16680_e28653_d_n6, assign16680_e28653_d_n7, assign16680_e28653_d_n8, assign16680_e28653_d_n9, assign16680_e28653_d_n10, assign16680_e28653_d_n11, assign16680_e28653_d_n13, assign16680_e28653_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard297 != 0.0)) {
        let assign16680_e28534: f64 = (p.p452 * locals.var_deltemp);
        let assign16680_e28535: f64 = (1.0 + assign16680_e28534);
        let assign16680_e28538: f64 = (p.p451 * locals.var_deltemp1);
        let assign16680_e28540: f64 = (assign16680_e28538 * locals.var_deltemp1);
        let assign16680_e28541: f64 = (assign16680_e28535 + assign16680_e28540);
        let assign16680_e28542: f64 = (locals.var_mexpr_i * assign16680_e28541);
        let assign16680_e28544: f64 = (assign16680_e28542 - 2.0);
        let assign16680_e28546: f64 = (-10000.0);
        let assign16680_e28548: f64 = (assign16680_e28546 * 0.001);
        let (assign16680_e28649, assign16680_e28649_d_n0, assign16680_e28649_d_n2, assign16680_e28649_d_n3, assign16680_e28649_d_n4, assign16680_e28649_d_n5, assign16680_e28649_d_n6, assign16680_e28649_d_n7, assign16680_e28649_d_n8, assign16680_e28649_d_n9, assign16680_e28649_d_n10, assign16680_e28649_d_n11, assign16680_e28649_d_n13, assign16680_e28649_d_n14,) = {
            if (!(assign16680_e28544 < assign16680_e28548)) {
                let assign16680_e28556: f64 = (p.p452 * locals.var_deltemp);
                let assign16680_e28557: f64 = (1.0 + assign16680_e28556);
                let assign16680_e28560: f64 = (p.p451 * locals.var_deltemp1);
                let assign16680_e28562: f64 = (assign16680_e28560 * locals.var_deltemp1);
                let assign16680_e28563: f64 = (assign16680_e28557 + assign16680_e28562);
                let assign16680_e28564: f64 = (locals.var_mexpr_i * assign16680_e28563);
                let assign16680_e28566: f64 = (assign16680_e28564 - 2.0);
                let assign16680_e28571: f64 = (p.p452 * locals.var_deltemp);
                let assign16680_e28572: f64 = (1.0 + assign16680_e28571);
                let assign16680_e28575: f64 = (p.p451 * locals.var_deltemp1);
                let assign16680_e28577: f64 = (assign16680_e28575 * locals.var_deltemp1);
                let assign16680_e28578: f64 = (assign16680_e28572 + assign16680_e28577);
                let assign16680_e28579: f64 = (locals.var_mexpr_i * assign16680_e28578);
                let assign16680_e28581: f64 = (assign16680_e28579 - 2.0);
                let assign16680_e28586: f64 = (p.p452 * locals.var_deltemp);
                let assign16680_e28587: f64 = (1.0 + assign16680_e28586);
                let assign16680_e28590: f64 = (p.p451 * locals.var_deltemp1);
                let assign16680_e28592: f64 = (assign16680_e28590 * locals.var_deltemp1);
                let assign16680_e28593: f64 = (assign16680_e28587 + assign16680_e28592);
                let assign16680_e28594: f64 = (locals.var_mexpr_i * assign16680_e28593);
                let assign16680_e28596: f64 = (assign16680_e28594 - 2.0);
                let assign16680_e28597: f64 = (assign16680_e28581 * assign16680_e28596);
                let assign16680_e28600: f64 = (4.0 * 0.001);
                let assign16680_e28602: f64 = (assign16680_e28600 * 0.001);
                let assign16680_e28603: f64 = (assign16680_e28597 + assign16680_e28602);
                let assign16680_e28604: f64 = (assign16680_e28603).sqrt();
                let assign16680_e28605: f64 = (assign16680_e28566 + assign16680_e28604);
                let assign16680_e28606: f64 = (0.5 * assign16680_e28605);
                (assign16680_e28606, (0.5 * ((locals.var_mexpr_i_dn0 * assign16680_e28563) + ((((locals.var_mexpr_i_dn0 * assign16680_e28578) * assign16680_e28596) + (assign16680_e28581 * (locals.var_mexpr_i_dn0 * assign16680_e28593))) / (2.0 * assign16680_e28604)))), (0.5 * ((locals.var_mexpr_i_dn2 * assign16680_e28563) + ((((locals.var_mexpr_i_dn2 * assign16680_e28578) * assign16680_e28596) + (assign16680_e28581 * (locals.var_mexpr_i_dn2 * assign16680_e28593))) / (2.0 * assign16680_e28604)))), (0.5 * ((locals.var_mexpr_i_dn3 * assign16680_e28563) + ((((locals.var_mexpr_i_dn3 * assign16680_e28578) * assign16680_e28596) + (assign16680_e28581 * (locals.var_mexpr_i_dn3 * assign16680_e28593))) / (2.0 * assign16680_e28604)))), (0.5 * (((locals.var_mexpr_i_dn4 * assign16680_e28563) + (locals.var_mexpr_i * ((p.p452 * locals.var_deltemp_dn4) + (((p.p451 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16680_e28560 * locals.var_deltemp1_dn4))))) + (((((locals.var_mexpr_i_dn4 * assign16680_e28578) + (locals.var_mexpr_i * ((p.p452 * locals.var_deltemp_dn4) + (((p.p451 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16680_e28575 * locals.var_deltemp1_dn4))))) * assign16680_e28596) + (assign16680_e28581 * ((locals.var_mexpr_i_dn4 * assign16680_e28593) + (locals.var_mexpr_i * ((p.p452 * locals.var_deltemp_dn4) + (((p.p451 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16680_e28590 * locals.var_deltemp1_dn4))))))) / (2.0 * assign16680_e28604)))), (0.5 * ((locals.var_mexpr_i_dn5 * assign16680_e28563) + ((((locals.var_mexpr_i_dn5 * assign16680_e28578) * assign16680_e28596) + (assign16680_e28581 * (locals.var_mexpr_i_dn5 * assign16680_e28593))) / (2.0 * assign16680_e28604)))), (0.5 * ((locals.var_mexpr_i_dn6 * assign16680_e28563) + ((((locals.var_mexpr_i_dn6 * assign16680_e28578) * assign16680_e28596) + (assign16680_e28581 * (locals.var_mexpr_i_dn6 * assign16680_e28593))) / (2.0 * assign16680_e28604)))), (0.5 * ((locals.var_mexpr_i_dn7 * assign16680_e28563) + ((((locals.var_mexpr_i_dn7 * assign16680_e28578) * assign16680_e28596) + (assign16680_e28581 * (locals.var_mexpr_i_dn7 * assign16680_e28593))) / (2.0 * assign16680_e28604)))), (0.5 * ((locals.var_mexpr_i_dn8 * assign16680_e28563) + ((((locals.var_mexpr_i_dn8 * assign16680_e28578) * assign16680_e28596) + (assign16680_e28581 * (locals.var_mexpr_i_dn8 * assign16680_e28593))) / (2.0 * assign16680_e28604)))), (0.5 * ((locals.var_mexpr_i_dn9 * assign16680_e28563) + ((((locals.var_mexpr_i_dn9 * assign16680_e28578) * assign16680_e28596) + (assign16680_e28581 * (locals.var_mexpr_i_dn9 * assign16680_e28593))) / (2.0 * assign16680_e28604)))), (0.5 * ((locals.var_mexpr_i_dn10 * assign16680_e28563) + ((((locals.var_mexpr_i_dn10 * assign16680_e28578) * assign16680_e28596) + (assign16680_e28581 * (locals.var_mexpr_i_dn10 * assign16680_e28593))) / (2.0 * assign16680_e28604)))), (0.5 * ((locals.var_mexpr_i_dn11 * assign16680_e28563) + ((((locals.var_mexpr_i_dn11 * assign16680_e28578) * assign16680_e28596) + (assign16680_e28581 * (locals.var_mexpr_i_dn11 * assign16680_e28593))) / (2.0 * assign16680_e28604)))), (0.5 * ((locals.var_mexpr_i_dn13 * assign16680_e28563) + ((((locals.var_mexpr_i_dn13 * assign16680_e28578) * assign16680_e28596) + (assign16680_e28581 * (locals.var_mexpr_i_dn13 * assign16680_e28593))) / (2.0 * assign16680_e28604)))), (0.5 * ((locals.var_mexpr_i_dn14 * assign16680_e28563) + ((((locals.var_mexpr_i_dn14 * assign16680_e28578) * assign16680_e28596) + (assign16680_e28581 * (locals.var_mexpr_i_dn14 * assign16680_e28593))) / (2.0 * assign16680_e28604)))),)
            } else {
                let assign16680_e28611: f64 = (p.p452 * locals.var_deltemp);
                let assign16680_e28612: f64 = (1.0 + assign16680_e28611);
                let assign16680_e28615: f64 = (p.p451 * locals.var_deltemp1);
                let assign16680_e28617: f64 = (assign16680_e28615 * locals.var_deltemp1);
                let assign16680_e28618: f64 = (assign16680_e28612 + assign16680_e28617);
                let assign16680_e28619: f64 = (locals.var_mexpr_i * assign16680_e28618);
                let assign16680_e28621: f64 = (assign16680_e28619 - 2.0);
                let assign16680_e28623: f64 = (-10000.0);
                let assign16680_e28625: f64 = (assign16680_e28623 * 0.001);
                let (assign16680_e28648, assign16680_e28648_d_n0, assign16680_e28648_d_n2, assign16680_e28648_d_n3, assign16680_e28648_d_n4, assign16680_e28648_d_n5, assign16680_e28648_d_n6, assign16680_e28648_d_n7, assign16680_e28648_d_n8, assign16680_e28648_d_n9, assign16680_e28648_d_n10, assign16680_e28648_d_n11, assign16680_e28648_d_n13, assign16680_e28648_d_n14,) = {
                    if (assign16680_e28621 < assign16680_e28625) {
                        let assign16680_e28628: f64 = (-0.001);
                        let assign16680_e28630: f64 = (assign16680_e28628 * 0.001);
                        let assign16680_e28635: f64 = (p.p452 * locals.var_deltemp);
                        let assign16680_e28636: f64 = (1.0 + assign16680_e28635);
                        let assign16680_e28639: f64 = (p.p451 * locals.var_deltemp1);
                        let assign16680_e28641: f64 = (assign16680_e28639 * locals.var_deltemp1);
                        let assign16680_e28642: f64 = (assign16680_e28636 + assign16680_e28641);
                        let assign16680_e28643: f64 = (locals.var_mexpr_i * assign16680_e28642);
                        let assign16680_e28645: f64 = (assign16680_e28643 - 2.0);
                        let assign16680_e28646: f64 = (assign16680_e28630 / assign16680_e28645);
                        (assign16680_e28646, (-((assign16680_e28630 * (locals.var_mexpr_i_dn0 * assign16680_e28642)) / (assign16680_e28645 * assign16680_e28645))), (-((assign16680_e28630 * (locals.var_mexpr_i_dn2 * assign16680_e28642)) / (assign16680_e28645 * assign16680_e28645))), (-((assign16680_e28630 * (locals.var_mexpr_i_dn3 * assign16680_e28642)) / (assign16680_e28645 * assign16680_e28645))), (-((assign16680_e28630 * ((locals.var_mexpr_i_dn4 * assign16680_e28642) + (locals.var_mexpr_i * ((p.p452 * locals.var_deltemp_dn4) + (((p.p451 * locals.var_deltemp1_dn4) * locals.var_deltemp1) + (assign16680_e28639 * locals.var_deltemp1_dn4)))))) / (assign16680_e28645 * assign16680_e28645))), (-((assign16680_e28630 * (locals.var_mexpr_i_dn5 * assign16680_e28642)) / (assign16680_e28645 * assign16680_e28645))), (-((assign16680_e28630 * (locals.var_mexpr_i_dn6 * assign16680_e28642)) / (assign16680_e28645 * assign16680_e28645))), (-((assign16680_e28630 * (locals.var_mexpr_i_dn7 * assign16680_e28642)) / (assign16680_e28645 * assign16680_e28645))), (-((assign16680_e28630 * (locals.var_mexpr_i_dn8 * assign16680_e28642)) / (assign16680_e28645 * assign16680_e28645))), (-((assign16680_e28630 * (locals.var_mexpr_i_dn9 * assign16680_e28642)) / (assign16680_e28645 * assign16680_e28645))), (-((assign16680_e28630 * (locals.var_mexpr_i_dn10 * assign16680_e28642)) / (assign16680_e28645 * assign16680_e28645))), (-((assign16680_e28630 * (locals.var_mexpr_i_dn11 * assign16680_e28642)) / (assign16680_e28645 * assign16680_e28645))), (-((assign16680_e28630 * (locals.var_mexpr_i_dn13 * assign16680_e28642)) / (assign16680_e28645 * assign16680_e28645))), (-((assign16680_e28630 * (locals.var_mexpr_i_dn14 * assign16680_e28642)) / (assign16680_e28645 * assign16680_e28645))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign16680_e28648, assign16680_e28648_d_n0, assign16680_e28648_d_n2, assign16680_e28648_d_n3, assign16680_e28648_d_n4, assign16680_e28648_d_n5, assign16680_e28648_d_n6, assign16680_e28648_d_n7, assign16680_e28648_d_n8, assign16680_e28648_d_n9, assign16680_e28648_d_n10, assign16680_e28648_d_n11, assign16680_e28648_d_n13, assign16680_e28648_d_n14,)
            }
        };
        let assign16680_e28651: f64 = (assign16680_e28649 + 2.0);
        (assign16680_e28651, assign16680_e28649_d_n0, assign16680_e28649_d_n2, assign16680_e28649_d_n3, assign16680_e28649_d_n4, assign16680_e28649_d_n5, assign16680_e28649_d_n6, assign16680_e28649_d_n7, assign16680_e28649_d_n8, assign16680_e28649_d_n9, assign16680_e28649_d_n10, assign16680_e28649_d_n11, assign16680_e28649_d_n13, assign16680_e28649_d_n14,)
    } else {
        (locals.var_mexpr_t, locals.var_mexpr_t_dn0, locals.var_mexpr_t_dn2, locals.var_mexpr_t_dn3, locals.var_mexpr_t_dn4, locals.var_mexpr_t_dn5, locals.var_mexpr_t_dn6, locals.var_mexpr_t_dn7, locals.var_mexpr_t_dn8, locals.var_mexpr_t_dn9, locals.var_mexpr_t_dn10, locals.var_mexpr_t_dn11, locals.var_mexpr_t_dn13, locals.var_mexpr_t_dn14,)
    }
};
        locals.var_mexpr_t = assign16680_e28653;
        locals.var_mexpr_t_dn0 = assign16680_e28653_d_n0;
        locals.var_mexpr_t_dn2 = assign16680_e28653_d_n2;
        locals.var_mexpr_t_dn3 = assign16680_e28653_d_n3;
        locals.var_mexpr_t_dn4 = assign16680_e28653_d_n4;
        locals.var_mexpr_t_dn5 = assign16680_e28653_d_n5;
        locals.var_mexpr_t_dn6 = assign16680_e28653_d_n6;
        locals.var_mexpr_t_dn7 = assign16680_e28653_d_n7;
        locals.var_mexpr_t_dn8 = assign16680_e28653_d_n8;
        locals.var_mexpr_t_dn9 = assign16680_e28653_d_n9;
        locals.var_mexpr_t_dn10 = assign16680_e28653_d_n10;
        locals.var_mexpr_t_dn11 = assign16680_e28653_d_n11;
        locals.var_mexpr_t_dn13 = assign16680_e28653_d_n13;
        locals.var_mexpr_t_dn14 = assign16680_e28653_d_n14;

        let assign16690_e28656: f64 = if p.p67 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard298 = assign16690_e28656;

        let (assign16700_e28675, assign16700_e28675_d_n0, assign16700_e28675_d_n2, assign16700_e28675_d_n3, assign16700_e28675_d_n4, assign16700_e28675_d_n5, assign16700_e28675_d_n6, assign16700_e28675_d_n7, assign16700_e28675_d_n8, assign16700_e28675_d_n9, assign16700_e28675_d_n10, assign16700_e28675_d_n11, assign16700_e28675_d_n13, assign16700_e28675_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard298 != 0.0)) {
        let assign16700_e28668: f64 = (locals.var_ute1cv_i * locals.var_deltratio1);
        let assign16700_e28669: f64 = (locals.var_utecv_i + assign16700_e28668);
        let assign16700_e28671: f64 = (assign16700_e28669 * locals.var_trat_ln);
        let assign16700_e28672: f64 = (assign16700_e28671).exp();
        let assign16700_e28673: f64 = (locals.var_u0cv_i * assign16700_e28672);
        (assign16700_e28673, (locals.var_u0cv_i_dn0 * assign16700_e28672), (locals.var_u0cv_i_dn2 * assign16700_e28672), (locals.var_u0cv_i_dn3 * assign16700_e28672), ((locals.var_u0cv_i_dn4 * assign16700_e28672) + (locals.var_u0cv_i * (assign16700_e28672 * (((locals.var_ute1cv_i * locals.var_deltratio1_dn4) * locals.var_trat_ln) + (assign16700_e28669 * locals.var_trat_ln_dn4))))), (locals.var_u0cv_i_dn5 * assign16700_e28672), (locals.var_u0cv_i_dn6 * assign16700_e28672), (locals.var_u0cv_i_dn7 * assign16700_e28672), (locals.var_u0cv_i_dn8 * assign16700_e28672), (locals.var_u0cv_i_dn9 * assign16700_e28672), (locals.var_u0cv_i_dn10 * assign16700_e28672), (locals.var_u0cv_i_dn11 * assign16700_e28672), (locals.var_u0cv_i_dn13 * assign16700_e28672), (locals.var_u0cv_i_dn14 * assign16700_e28672),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign16700_e28675;
        locals.var_t1_dn0 = assign16700_e28675_d_n0;
        locals.var_t1_dn2 = assign16700_e28675_d_n2;
        locals.var_t1_dn3 = assign16700_e28675_d_n3;
        locals.var_t1_dn4 = assign16700_e28675_d_n4;
        locals.var_t1_dn5 = assign16700_e28675_d_n5;
        locals.var_t1_dn6 = assign16700_e28675_d_n6;
        locals.var_t1_dn7 = assign16700_e28675_d_n7;
        locals.var_t1_dn8 = assign16700_e28675_d_n8;
        locals.var_t1_dn9 = assign16700_e28675_d_n9;
        locals.var_t1_dn10 = assign16700_e28675_d_n10;
        locals.var_t1_dn11 = assign16700_e28675_d_n11;
        locals.var_t1_dn13 = assign16700_e28675_d_n13;
        locals.var_t1_dn14 = assign16700_e28675_d_n14;

        let (assign16710_e28735, assign16710_e28735_d_n0, assign16710_e28735_d_n2, assign16710_e28735_d_n3, assign16710_e28735_d_n4, assign16710_e28735_d_n5, assign16710_e28735_d_n6, assign16710_e28735_d_n7, assign16710_e28735_d_n8, assign16710_e28735_d_n9, assign16710_e28735_d_n10, assign16710_e28735_d_n11, assign16710_e28735_d_n13, assign16710_e28735_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard298 != 0.0)) {
        let assign16710_e28685: f64 = (-0.9);
        let assign16710_e28687: f64 = (assign16710_e28685 * locals.var_t1);
        let assign16710_e28691: f64 = (locals.var_utlcv_i * locals.var_deltemp);
        let assign16710_e28693: f64 = (-0.9);
        let assign16710_e28695: f64 = (assign16710_e28693 * locals.var_t1);
        let assign16710_e28696: f64 = (assign16710_e28691 - assign16710_e28695);
        let assign16710_e28698: f64 = (assign16710_e28696 - 0.0001);
        let assign16710_e28701: f64 = (locals.var_utlcv_i * locals.var_deltemp);
        let assign16710_e28703: f64 = (-0.9);
        let assign16710_e28705: f64 = (assign16710_e28703 * locals.var_t1);
        let assign16710_e28706: f64 = (assign16710_e28701 - assign16710_e28705);
        let assign16710_e28708: f64 = (assign16710_e28706 - 0.0001);
        let assign16710_e28711: f64 = (locals.var_utlcv_i * locals.var_deltemp);
        let assign16710_e28713: f64 = (-0.9);
        let assign16710_e28715: f64 = (assign16710_e28713 * locals.var_t1);
        let assign16710_e28716: f64 = (assign16710_e28711 - assign16710_e28715);
        let assign16710_e28718: f64 = (assign16710_e28716 - 0.0001);
        let assign16710_e28719: f64 = (assign16710_e28708 * assign16710_e28718);
        let assign16710_e28722: f64 = (-0.9);
        let assign16710_e28724: f64 = (assign16710_e28722 * locals.var_t1);
        let assign16710_e28725: f64 = (4.0 * assign16710_e28724);
        let assign16710_e28727: f64 = (assign16710_e28725 * 0.0001);
        let assign16710_e28728: f64 = (assign16710_e28719 - assign16710_e28727);
        let assign16710_e28729: f64 = (assign16710_e28728).sqrt();
        let assign16710_e28730: f64 = (assign16710_e28698 + assign16710_e28729);
        let assign16710_e28731: f64 = (0.5 * assign16710_e28730);
        let assign16710_e28732: f64 = (assign16710_e28687 + assign16710_e28731);
        let assign16710_e28733: f64 = (locals.var_t1 + assign16710_e28732);
        (assign16710_e28733, (locals.var_t1_dn0 + ((assign16710_e28685 * locals.var_t1_dn0) + (0.5 * ((-(assign16710_e28693 * locals.var_t1_dn0)) + (((((-(assign16710_e28703 * locals.var_t1_dn0)) * assign16710_e28718) + (assign16710_e28708 * (-(assign16710_e28713 * locals.var_t1_dn0)))) - ((4.0 * (assign16710_e28722 * locals.var_t1_dn0)) * 0.0001)) / (2.0 * assign16710_e28729)))))), (locals.var_t1_dn2 + ((assign16710_e28685 * locals.var_t1_dn2) + (0.5 * ((-(assign16710_e28693 * locals.var_t1_dn2)) + (((((-(assign16710_e28703 * locals.var_t1_dn2)) * assign16710_e28718) + (assign16710_e28708 * (-(assign16710_e28713 * locals.var_t1_dn2)))) - ((4.0 * (assign16710_e28722 * locals.var_t1_dn2)) * 0.0001)) / (2.0 * assign16710_e28729)))))), (locals.var_t1_dn3 + ((assign16710_e28685 * locals.var_t1_dn3) + (0.5 * ((-(assign16710_e28693 * locals.var_t1_dn3)) + (((((-(assign16710_e28703 * locals.var_t1_dn3)) * assign16710_e28718) + (assign16710_e28708 * (-(assign16710_e28713 * locals.var_t1_dn3)))) - ((4.0 * (assign16710_e28722 * locals.var_t1_dn3)) * 0.0001)) / (2.0 * assign16710_e28729)))))), (locals.var_t1_dn4 + ((assign16710_e28685 * locals.var_t1_dn4) + (0.5 * (((locals.var_utlcv_i * locals.var_deltemp_dn4) - (assign16710_e28693 * locals.var_t1_dn4)) + ((((((locals.var_utlcv_i * locals.var_deltemp_dn4) - (assign16710_e28703 * locals.var_t1_dn4)) * assign16710_e28718) + (assign16710_e28708 * ((locals.var_utlcv_i * locals.var_deltemp_dn4) - (assign16710_e28713 * locals.var_t1_dn4)))) - ((4.0 * (assign16710_e28722 * locals.var_t1_dn4)) * 0.0001)) / (2.0 * assign16710_e28729)))))), (locals.var_t1_dn5 + ((assign16710_e28685 * locals.var_t1_dn5) + (0.5 * ((-(assign16710_e28693 * locals.var_t1_dn5)) + (((((-(assign16710_e28703 * locals.var_t1_dn5)) * assign16710_e28718) + (assign16710_e28708 * (-(assign16710_e28713 * locals.var_t1_dn5)))) - ((4.0 * (assign16710_e28722 * locals.var_t1_dn5)) * 0.0001)) / (2.0 * assign16710_e28729)))))), (locals.var_t1_dn6 + ((assign16710_e28685 * locals.var_t1_dn6) + (0.5 * ((-(assign16710_e28693 * locals.var_t1_dn6)) + (((((-(assign16710_e28703 * locals.var_t1_dn6)) * assign16710_e28718) + (assign16710_e28708 * (-(assign16710_e28713 * locals.var_t1_dn6)))) - ((4.0 * (assign16710_e28722 * locals.var_t1_dn6)) * 0.0001)) / (2.0 * assign16710_e28729)))))), (locals.var_t1_dn7 + ((assign16710_e28685 * locals.var_t1_dn7) + (0.5 * ((-(assign16710_e28693 * locals.var_t1_dn7)) + (((((-(assign16710_e28703 * locals.var_t1_dn7)) * assign16710_e28718) + (assign16710_e28708 * (-(assign16710_e28713 * locals.var_t1_dn7)))) - ((4.0 * (assign16710_e28722 * locals.var_t1_dn7)) * 0.0001)) / (2.0 * assign16710_e28729)))))), (locals.var_t1_dn8 + ((assign16710_e28685 * locals.var_t1_dn8) + (0.5 * ((-(assign16710_e28693 * locals.var_t1_dn8)) + (((((-(assign16710_e28703 * locals.var_t1_dn8)) * assign16710_e28718) + (assign16710_e28708 * (-(assign16710_e28713 * locals.var_t1_dn8)))) - ((4.0 * (assign16710_e28722 * locals.var_t1_dn8)) * 0.0001)) / (2.0 * assign16710_e28729)))))), (locals.var_t1_dn9 + ((assign16710_e28685 * locals.var_t1_dn9) + (0.5 * ((-(assign16710_e28693 * locals.var_t1_dn9)) + (((((-(assign16710_e28703 * locals.var_t1_dn9)) * assign16710_e28718) + (assign16710_e28708 * (-(assign16710_e28713 * locals.var_t1_dn9)))) - ((4.0 * (assign16710_e28722 * locals.var_t1_dn9)) * 0.0001)) / (2.0 * assign16710_e28729)))))), (locals.var_t1_dn10 + ((assign16710_e28685 * locals.var_t1_dn10) + (0.5 * ((-(assign16710_e28693 * locals.var_t1_dn10)) + (((((-(assign16710_e28703 * locals.var_t1_dn10)) * assign16710_e28718) + (assign16710_e28708 * (-(assign16710_e28713 * locals.var_t1_dn10)))) - ((4.0 * (assign16710_e28722 * locals.var_t1_dn10)) * 0.0001)) / (2.0 * assign16710_e28729)))))), (locals.var_t1_dn11 + ((assign16710_e28685 * locals.var_t1_dn11) + (0.5 * ((-(assign16710_e28693 * locals.var_t1_dn11)) + (((((-(assign16710_e28703 * locals.var_t1_dn11)) * assign16710_e28718) + (assign16710_e28708 * (-(assign16710_e28713 * locals.var_t1_dn11)))) - ((4.0 * (assign16710_e28722 * locals.var_t1_dn11)) * 0.0001)) / (2.0 * assign16710_e28729)))))), (locals.var_t1_dn13 + ((assign16710_e28685 * locals.var_t1_dn13) + (0.5 * ((-(assign16710_e28693 * locals.var_t1_dn13)) + (((((-(assign16710_e28703 * locals.var_t1_dn13)) * assign16710_e28718) + (assign16710_e28708 * (-(assign16710_e28713 * locals.var_t1_dn13)))) - ((4.0 * (assign16710_e28722 * locals.var_t1_dn13)) * 0.0001)) / (2.0 * assign16710_e28729)))))), (locals.var_t1_dn14 + ((assign16710_e28685 * locals.var_t1_dn14) + (0.5 * ((-(assign16710_e28693 * locals.var_t1_dn14)) + (((((-(assign16710_e28703 * locals.var_t1_dn14)) * assign16710_e28718) + (assign16710_e28708 * (-(assign16710_e28713 * locals.var_t1_dn14)))) - ((4.0 * (assign16710_e28722 * locals.var_t1_dn14)) * 0.0001)) / (2.0 * assign16710_e28729)))))),)
    } else {
        (locals.var_u0_cv, locals.var_u0_cv_dn0, locals.var_u0_cv_dn2, locals.var_u0_cv_dn3, locals.var_u0_cv_dn4, locals.var_u0_cv_dn5, locals.var_u0_cv_dn6, locals.var_u0_cv_dn7, locals.var_u0_cv_dn8, locals.var_u0_cv_dn9, locals.var_u0_cv_dn10, locals.var_u0_cv_dn11, locals.var_u0_cv_dn13, locals.var_u0_cv_dn14,)
    }
};
        locals.var_u0_cv = assign16710_e28735;
        locals.var_u0_cv_dn0 = assign16710_e28735_d_n0;
        locals.var_u0_cv_dn2 = assign16710_e28735_d_n2;
        locals.var_u0_cv_dn3 = assign16710_e28735_d_n3;
        locals.var_u0_cv_dn4 = assign16710_e28735_d_n4;
        locals.var_u0_cv_dn5 = assign16710_e28735_d_n5;
        locals.var_u0_cv_dn6 = assign16710_e28735_d_n6;
        locals.var_u0_cv_dn7 = assign16710_e28735_d_n7;
        locals.var_u0_cv_dn8 = assign16710_e28735_d_n8;
        locals.var_u0_cv_dn9 = assign16710_e28735_d_n9;
        locals.var_u0_cv_dn10 = assign16710_e28735_d_n10;
        locals.var_u0_cv_dn11 = assign16710_e28735_d_n11;
        locals.var_u0_cv_dn13 = assign16710_e28735_d_n13;
        locals.var_u0_cv_dn14 = assign16710_e28735_d_n14;

        let assign16720_e28738: f64 = if locals.var_tnom > 210.0 { 1.0 } else { 0.0 };
        locals.var_guard299 = assign16720_e28738;

        let (assign16730_e28787, assign16730_e28787_d_n0, assign16730_e28787_d_n2, assign16730_e28787_d_n3, assign16730_e28787_d_n4, assign16730_e28787_d_n5, assign16730_e28787_d_n6, assign16730_e28787_d_n7, assign16730_e28787_d_n8, assign16730_e28787_d_n9, assign16730_e28787_d_n10, assign16730_e28787_d_n11, assign16730_e28787_d_n13, assign16730_e28787_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard298 != 0.0)) && (locals.var_guard299 != 0.0)) {
        let assign16730_e28754: f64 = (210.0 - locals.var_tnom);
        let assign16730_e28755: f64 = (locals.var_ua1cv_i * assign16730_e28754);
        let assign16730_e28756: f64 = (locals.var_uacv_i + assign16730_e28755);
        let assign16730_e28757: f64 = (locals.var_ua1cv_i / assign16730_e28756);
        let assign16730_e28761: f64 = (210.0 / locals.var_tnom);
        let (assign16730_e28778,) = {
            if (!(assign16730_e28761 > 1e-38)) {
                let assign16730_e28766: f64 = (-87.498233534);
                (assign16730_e28766,)
            } else {
                let assign16730_e28769: f64 = (210.0 / locals.var_tnom);
                let (assign16730_e28777,) = {
                    if (assign16730_e28769 > 1e-38) {
                        let assign16730_e28774: f64 = (210.0 / locals.var_tnom);
                        let assign16730_e28775: f64 = (assign16730_e28774).ln();
                        (assign16730_e28775,)
                    } else {
                        (0.0,)
                    }
                };
                (assign16730_e28777,)
            }
        };
        let assign16730_e28780: f64 = (assign16730_e28778 + 1.0);
        let assign16730_e28781: f64 = (locals.var_ua2cv_i * assign16730_e28780);
        let assign16730_e28783: f64 = (assign16730_e28781 / locals.var_tnom);
        let assign16730_e28784: f64 = (assign16730_e28757 - assign16730_e28783);
        let assign16730_e28785: f64 = (210.0 * assign16730_e28784);
        (assign16730_e28785, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign16730_e28787;
        locals.var_t2_dn0 = assign16730_e28787_d_n0;
        locals.var_t2_dn2 = assign16730_e28787_d_n2;
        locals.var_t2_dn3 = assign16730_e28787_d_n3;
        locals.var_t2_dn4 = assign16730_e28787_d_n4;
        locals.var_t2_dn5 = assign16730_e28787_d_n5;
        locals.var_t2_dn6 = assign16730_e28787_d_n6;
        locals.var_t2_dn7 = assign16730_e28787_d_n7;
        locals.var_t2_dn8 = assign16730_e28787_d_n8;
        locals.var_t2_dn9 = assign16730_e28787_d_n9;
        locals.var_t2_dn10 = assign16730_e28787_d_n10;
        locals.var_t2_dn11 = assign16730_e28787_d_n11;
        locals.var_t2_dn13 = assign16730_e28787_d_n13;
        locals.var_t2_dn14 = assign16730_e28787_d_n14;

        let (assign16740_e28817, assign16740_e28817_d_n0, assign16740_e28817_d_n2, assign16740_e28817_d_n3, assign16740_e28817_d_n4, assign16740_e28817_d_n5, assign16740_e28817_d_n6, assign16740_e28817_d_n7, assign16740_e28817_d_n8, assign16740_e28817_d_n9, assign16740_e28817_d_n10, assign16740_e28817_d_n11, assign16740_e28817_d_n13, assign16740_e28817_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard298 != 0.0)) && (locals.var_guard299 != 0.0)) {
        let assign16740_e28801: f64 = (210.0 - locals.var_tnom);
        let assign16740_e28802: f64 = (locals.var_ua1cv_i * assign16740_e28801);
        let assign16740_e28803: f64 = (locals.var_uacv_i + assign16740_e28802);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_tnom;
        let assign16740_e28806: f64 = (210.0 * __rspice_inv_cse_0);
        let assign16740_e28811: f64 = (210.0 * __rspice_inv_cse_0);
        let assign16740_e28812: f64 = (locals.var_ua2cv_i * assign16740_e28811);
        let assign16740_e28813: f64 = (locals.var_t2 + assign16740_e28812);
        let assign16740_e28814: f64 = (assign16740_e28806).powf(assign16740_e28813);
        let assign16740_e28815: f64 = (assign16740_e28803 / assign16740_e28814);
        (assign16740_e28815, (-((assign16740_e28803 * if locals.var_t2_dn0 == 0.0 && ((assign16740_e28813) as f64).is_finite() && ((assign16740_e28813) as f64).fract() == 0.0 { 0.0 } else { (assign16740_e28814 * (locals.var_t2_dn0 * (assign16740_e28806).ln())) }) / (assign16740_e28814 * assign16740_e28814))), (-((assign16740_e28803 * if locals.var_t2_dn2 == 0.0 && ((assign16740_e28813) as f64).is_finite() && ((assign16740_e28813) as f64).fract() == 0.0 { 0.0 } else { (assign16740_e28814 * (locals.var_t2_dn2 * (assign16740_e28806).ln())) }) / (assign16740_e28814 * assign16740_e28814))), (-((assign16740_e28803 * if locals.var_t2_dn3 == 0.0 && ((assign16740_e28813) as f64).is_finite() && ((assign16740_e28813) as f64).fract() == 0.0 { 0.0 } else { (assign16740_e28814 * (locals.var_t2_dn3 * (assign16740_e28806).ln())) }) / (assign16740_e28814 * assign16740_e28814))), (-((assign16740_e28803 * if locals.var_t2_dn4 == 0.0 && ((assign16740_e28813) as f64).is_finite() && ((assign16740_e28813) as f64).fract() == 0.0 { 0.0 } else { (assign16740_e28814 * (locals.var_t2_dn4 * (assign16740_e28806).ln())) }) / (assign16740_e28814 * assign16740_e28814))), (-((assign16740_e28803 * if locals.var_t2_dn5 == 0.0 && ((assign16740_e28813) as f64).is_finite() && ((assign16740_e28813) as f64).fract() == 0.0 { 0.0 } else { (assign16740_e28814 * (locals.var_t2_dn5 * (assign16740_e28806).ln())) }) / (assign16740_e28814 * assign16740_e28814))), (-((assign16740_e28803 * if locals.var_t2_dn6 == 0.0 && ((assign16740_e28813) as f64).is_finite() && ((assign16740_e28813) as f64).fract() == 0.0 { 0.0 } else { (assign16740_e28814 * (locals.var_t2_dn6 * (assign16740_e28806).ln())) }) / (assign16740_e28814 * assign16740_e28814))), (-((assign16740_e28803 * if locals.var_t2_dn7 == 0.0 && ((assign16740_e28813) as f64).is_finite() && ((assign16740_e28813) as f64).fract() == 0.0 { 0.0 } else { (assign16740_e28814 * (locals.var_t2_dn7 * (assign16740_e28806).ln())) }) / (assign16740_e28814 * assign16740_e28814))), (-((assign16740_e28803 * if locals.var_t2_dn8 == 0.0 && ((assign16740_e28813) as f64).is_finite() && ((assign16740_e28813) as f64).fract() == 0.0 { 0.0 } else { (assign16740_e28814 * (locals.var_t2_dn8 * (assign16740_e28806).ln())) }) / (assign16740_e28814 * assign16740_e28814))), (-((assign16740_e28803 * if locals.var_t2_dn9 == 0.0 && ((assign16740_e28813) as f64).is_finite() && ((assign16740_e28813) as f64).fract() == 0.0 { 0.0 } else { (assign16740_e28814 * (locals.var_t2_dn9 * (assign16740_e28806).ln())) }) / (assign16740_e28814 * assign16740_e28814))), (-((assign16740_e28803 * if locals.var_t2_dn10 == 0.0 && ((assign16740_e28813) as f64).is_finite() && ((assign16740_e28813) as f64).fract() == 0.0 { 0.0 } else { (assign16740_e28814 * (locals.var_t2_dn10 * (assign16740_e28806).ln())) }) / (assign16740_e28814 * assign16740_e28814))), (-((assign16740_e28803 * if locals.var_t2_dn11 == 0.0 && ((assign16740_e28813) as f64).is_finite() && ((assign16740_e28813) as f64).fract() == 0.0 { 0.0 } else { (assign16740_e28814 * (locals.var_t2_dn11 * (assign16740_e28806).ln())) }) / (assign16740_e28814 * assign16740_e28814))), (-((assign16740_e28803 * if locals.var_t2_dn13 == 0.0 && ((assign16740_e28813) as f64).is_finite() && ((assign16740_e28813) as f64).fract() == 0.0 { 0.0 } else { (assign16740_e28814 * (locals.var_t2_dn13 * (assign16740_e28806).ln())) }) / (assign16740_e28814 * assign16740_e28814))), (-((assign16740_e28803 * if locals.var_t2_dn14 == 0.0 && ((assign16740_e28813) as f64).is_finite() && ((assign16740_e28813) as f64).fract() == 0.0 { 0.0 } else { (assign16740_e28814 * (locals.var_t2_dn14 * (assign16740_e28806).ln())) }) / (assign16740_e28814 * assign16740_e28814))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign16740_e28817;
        locals.var_t1_dn0 = assign16740_e28817_d_n0;
        locals.var_t1_dn2 = assign16740_e28817_d_n2;
        locals.var_t1_dn3 = assign16740_e28817_d_n3;
        locals.var_t1_dn4 = assign16740_e28817_d_n4;
        locals.var_t1_dn5 = assign16740_e28817_d_n5;
        locals.var_t1_dn6 = assign16740_e28817_d_n6;
        locals.var_t1_dn7 = assign16740_e28817_d_n7;
        locals.var_t1_dn8 = assign16740_e28817_d_n8;
        locals.var_t1_dn9 = assign16740_e28817_d_n9;
        locals.var_t1_dn10 = assign16740_e28817_d_n10;
        locals.var_t1_dn11 = assign16740_e28817_d_n11;
        locals.var_t1_dn13 = assign16740_e28817_d_n13;
        locals.var_t1_dn14 = assign16740_e28817_d_n14;

    }

    pub(super) fn stamp_transient_block_53(
        locals: &mut StampLocals,
    ) {
        let (assign16750_e28837, assign16750_e28837_d_n0, assign16750_e28837_d_n2, assign16750_e28837_d_n3, assign16750_e28837_d_n4, assign16750_e28837_d_n5, assign16750_e28837_d_n6, assign16750_e28837_d_n7, assign16750_e28837_d_n8, assign16750_e28837_d_n9, assign16750_e28837_d_n10, assign16750_e28837_d_n11, assign16750_e28837_d_n13, assign16750_e28837_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard298 != 0.0)) && (locals.var_guard299 != 0.0)) {
        let assign16750_e28832: f64 = (locals.var_ua2cv_i * locals.var_tratio);
        let assign16750_e28833: f64 = (locals.var_t2 + assign16750_e28832);
        let assign16750_e28834: f64 = (locals.var_tratio).powf(assign16750_e28833);
        let assign16750_e28835: f64 = (locals.var_t1 * assign16750_e28834);
        (assign16750_e28835, ((locals.var_t1_dn0 * assign16750_e28834) + (locals.var_t1 * if locals.var_t2_dn0 == 0.0 && ((assign16750_e28833) as f64).is_finite() && ((assign16750_e28833) as f64).fract() == 0.0 { 0.0 } else { (assign16750_e28834 * (locals.var_t2_dn0 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn2 * assign16750_e28834) + (locals.var_t1 * if locals.var_t2_dn2 == 0.0 && ((assign16750_e28833) as f64).is_finite() && ((assign16750_e28833) as f64).fract() == 0.0 { 0.0 } else { (assign16750_e28834 * (locals.var_t2_dn2 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn3 * assign16750_e28834) + (locals.var_t1 * if locals.var_t2_dn3 == 0.0 && ((assign16750_e28833) as f64).is_finite() && ((assign16750_e28833) as f64).fract() == 0.0 { 0.0 } else { (assign16750_e28834 * (locals.var_t2_dn3 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn4 * assign16750_e28834) + (locals.var_t1 * if (locals.var_t2_dn4 + (locals.var_ua2cv_i * locals.var_tratio_dn4)) == 0.0 && ((assign16750_e28833) as f64).is_finite() && ((assign16750_e28833) as f64).fract() == 0.0 { if assign16750_e28833 == 0.0 { 0.0 } else { (assign16750_e28833 * ((locals.var_tratio).powf(assign16750_e28833 - 1.0) * locals.var_tratio_dn4)) } } else { (assign16750_e28834 * (((locals.var_t2_dn4 + (locals.var_ua2cv_i * locals.var_tratio_dn4)) * (locals.var_tratio).ln()) + (assign16750_e28833 * (locals.var_tratio_dn4 / locals.var_tratio)))) })), ((locals.var_t1_dn5 * assign16750_e28834) + (locals.var_t1 * if locals.var_t2_dn5 == 0.0 && ((assign16750_e28833) as f64).is_finite() && ((assign16750_e28833) as f64).fract() == 0.0 { 0.0 } else { (assign16750_e28834 * (locals.var_t2_dn5 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn6 * assign16750_e28834) + (locals.var_t1 * if locals.var_t2_dn6 == 0.0 && ((assign16750_e28833) as f64).is_finite() && ((assign16750_e28833) as f64).fract() == 0.0 { 0.0 } else { (assign16750_e28834 * (locals.var_t2_dn6 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn7 * assign16750_e28834) + (locals.var_t1 * if locals.var_t2_dn7 == 0.0 && ((assign16750_e28833) as f64).is_finite() && ((assign16750_e28833) as f64).fract() == 0.0 { 0.0 } else { (assign16750_e28834 * (locals.var_t2_dn7 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn8 * assign16750_e28834) + (locals.var_t1 * if locals.var_t2_dn8 == 0.0 && ((assign16750_e28833) as f64).is_finite() && ((assign16750_e28833) as f64).fract() == 0.0 { 0.0 } else { (assign16750_e28834 * (locals.var_t2_dn8 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn9 * assign16750_e28834) + (locals.var_t1 * if locals.var_t2_dn9 == 0.0 && ((assign16750_e28833) as f64).is_finite() && ((assign16750_e28833) as f64).fract() == 0.0 { 0.0 } else { (assign16750_e28834 * (locals.var_t2_dn9 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn10 * assign16750_e28834) + (locals.var_t1 * if locals.var_t2_dn10 == 0.0 && ((assign16750_e28833) as f64).is_finite() && ((assign16750_e28833) as f64).fract() == 0.0 { 0.0 } else { (assign16750_e28834 * (locals.var_t2_dn10 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn11 * assign16750_e28834) + (locals.var_t1 * if locals.var_t2_dn11 == 0.0 && ((assign16750_e28833) as f64).is_finite() && ((assign16750_e28833) as f64).fract() == 0.0 { 0.0 } else { (assign16750_e28834 * (locals.var_t2_dn11 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn13 * assign16750_e28834) + (locals.var_t1 * if locals.var_t2_dn13 == 0.0 && ((assign16750_e28833) as f64).is_finite() && ((assign16750_e28833) as f64).fract() == 0.0 { 0.0 } else { (assign16750_e28834 * (locals.var_t2_dn13 * (locals.var_tratio).ln())) })), ((locals.var_t1_dn14 * assign16750_e28834) + (locals.var_t1 * if locals.var_t2_dn14 == 0.0 && ((assign16750_e28833) as f64).is_finite() && ((assign16750_e28833) as f64).fract() == 0.0 { 0.0 } else { (assign16750_e28834 * (locals.var_t2_dn14 * (locals.var_tratio).ln())) })),)
    } else {
        (locals.var_uacv_tl, locals.var_uacv_tl_dn0, locals.var_uacv_tl_dn2, locals.var_uacv_tl_dn3, locals.var_uacv_tl_dn4, locals.var_uacv_tl_dn5, locals.var_uacv_tl_dn6, locals.var_uacv_tl_dn7, locals.var_uacv_tl_dn8, locals.var_uacv_tl_dn9, locals.var_uacv_tl_dn10, locals.var_uacv_tl_dn11, locals.var_uacv_tl_dn13, locals.var_uacv_tl_dn14,)
    }
};
        locals.var_uacv_tl = assign16750_e28837;
        locals.var_uacv_tl_dn0 = assign16750_e28837_d_n0;
        locals.var_uacv_tl_dn2 = assign16750_e28837_d_n2;
        locals.var_uacv_tl_dn3 = assign16750_e28837_d_n3;
        locals.var_uacv_tl_dn4 = assign16750_e28837_d_n4;
        locals.var_uacv_tl_dn5 = assign16750_e28837_d_n5;
        locals.var_uacv_tl_dn6 = assign16750_e28837_d_n6;
        locals.var_uacv_tl_dn7 = assign16750_e28837_d_n7;
        locals.var_uacv_tl_dn8 = assign16750_e28837_d_n8;
        locals.var_uacv_tl_dn9 = assign16750_e28837_d_n9;
        locals.var_uacv_tl_dn10 = assign16750_e28837_d_n10;
        locals.var_uacv_tl_dn11 = assign16750_e28837_d_n11;
        locals.var_uacv_tl_dn13 = assign16750_e28837_d_n13;
        locals.var_uacv_tl_dn14 = assign16750_e28837_d_n14;

        let (assign16760_e28853, assign16760_e28853_d_n0, assign16760_e28853_d_n2, assign16760_e28853_d_n3, assign16760_e28853_d_n4, assign16760_e28853_d_n5, assign16760_e28853_d_n6, assign16760_e28853_d_n7, assign16760_e28853_d_n8, assign16760_e28853_d_n9, assign16760_e28853_d_n10, assign16760_e28853_d_n11, assign16760_e28853_d_n13, assign16760_e28853_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard298 != 0.0)) && (locals.var_guard299 != 0.0)) {
        let assign16760_e28850: f64 = (locals.var_ua1cv_i * locals.var_deltemp);
        let assign16760_e28851: f64 = (locals.var_uacv_i + assign16760_e28850);
        (assign16760_e28851, 0.0, 0.0, 0.0, (locals.var_ua1cv_i * locals.var_deltemp_dn4), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uacv_th, locals.var_uacv_th_dn0, locals.var_uacv_th_dn2, locals.var_uacv_th_dn3, locals.var_uacv_th_dn4, locals.var_uacv_th_dn5, locals.var_uacv_th_dn6, locals.var_uacv_th_dn7, locals.var_uacv_th_dn8, locals.var_uacv_th_dn9, locals.var_uacv_th_dn10, locals.var_uacv_th_dn11, locals.var_uacv_th_dn13, locals.var_uacv_th_dn14,)
    }
};
        locals.var_uacv_th = assign16760_e28853;
        locals.var_uacv_th_dn0 = assign16760_e28853_d_n0;
        locals.var_uacv_th_dn2 = assign16760_e28853_d_n2;
        locals.var_uacv_th_dn3 = assign16760_e28853_d_n3;
        locals.var_uacv_th_dn4 = assign16760_e28853_d_n4;
        locals.var_uacv_th_dn5 = assign16760_e28853_d_n5;
        locals.var_uacv_th_dn6 = assign16760_e28853_d_n6;
        locals.var_uacv_th_dn7 = assign16760_e28853_d_n7;
        locals.var_uacv_th_dn8 = assign16760_e28853_d_n8;
        locals.var_uacv_th_dn9 = assign16760_e28853_d_n9;
        locals.var_uacv_th_dn10 = assign16760_e28853_d_n10;
        locals.var_uacv_th_dn11 = assign16760_e28853_d_n11;
        locals.var_uacv_th_dn13 = assign16760_e28853_d_n13;
        locals.var_uacv_th_dn14 = assign16760_e28853_d_n14;

        let (assign16770_e28909, assign16770_e28909_d_n0, assign16770_e28909_d_n2, assign16770_e28909_d_n3, assign16770_e28909_d_n4, assign16770_e28909_d_n5, assign16770_e28909_d_n6, assign16770_e28909_d_n7, assign16770_e28909_d_n8, assign16770_e28909_d_n9, assign16770_e28909_d_n10, assign16770_e28909_d_n11, assign16770_e28909_d_n13, assign16770_e28909_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard298 != 0.0)) && (locals.var_guard299 == 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_tnom;
        let assign16770_e28867: f64 = (210.0 * __rspice_inv_cse_0);
        let assign16770_e28872: f64 = (210.0 * __rspice_inv_cse_0);
        let assign16770_e28873: f64 = (locals.var_ua2cv_i * assign16770_e28872);
        let assign16770_e28874: f64 = (locals.var_ua1cv_i + assign16770_e28873);
        let assign16770_e28875: f64 = (assign16770_e28867).powf(assign16770_e28874);
        let assign16770_e28876: f64 = (locals.var_uacv_i * assign16770_e28875);
        let assign16770_e28879: f64 = (locals.var_ua1cv_i / 210.0);
        let assign16770_e28883: f64 = (210.0 / locals.var_tnom);
        let (assign16770_e28900,) = {
            if (!(assign16770_e28883 > 1e-38)) {
                let assign16770_e28888: f64 = (-87.498233534);
                (assign16770_e28888,)
            } else {
                let assign16770_e28891: f64 = (210.0 / locals.var_tnom);
                let (assign16770_e28899,) = {
                    if (assign16770_e28891 > 1e-38) {
                        let assign16770_e28896: f64 = (210.0 / locals.var_tnom);
                        let assign16770_e28897: f64 = (assign16770_e28896).ln();
                        (assign16770_e28897,)
                    } else {
                        (0.0,)
                    }
                };
                (assign16770_e28899,)
            }
        };
        let assign16770_e28902: f64 = (assign16770_e28900 + 1.0);
        let assign16770_e28903: f64 = (locals.var_ua2cv_i * assign16770_e28902);
        let assign16770_e28905: f64 = (assign16770_e28903 / locals.var_tnom);
        let assign16770_e28906: f64 = (assign16770_e28879 + assign16770_e28905);
        let assign16770_e28907: f64 = (assign16770_e28876 * assign16770_e28906);
        (assign16770_e28907, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign16770_e28909;
        locals.var_t2_dn0 = assign16770_e28909_d_n0;
        locals.var_t2_dn2 = assign16770_e28909_d_n2;
        locals.var_t2_dn3 = assign16770_e28909_d_n3;
        locals.var_t2_dn4 = assign16770_e28909_d_n4;
        locals.var_t2_dn5 = assign16770_e28909_d_n5;
        locals.var_t2_dn6 = assign16770_e28909_d_n6;
        locals.var_t2_dn7 = assign16770_e28909_d_n7;
        locals.var_t2_dn8 = assign16770_e28909_d_n8;
        locals.var_t2_dn9 = assign16770_e28909_d_n9;
        locals.var_t2_dn10 = assign16770_e28909_d_n10;
        locals.var_t2_dn11 = assign16770_e28909_d_n11;
        locals.var_t2_dn13 = assign16770_e28909_d_n13;
        locals.var_t2_dn14 = assign16770_e28909_d_n14;

        let (assign16780_e28940, assign16780_e28940_d_n0, assign16780_e28940_d_n2, assign16780_e28940_d_n3, assign16780_e28940_d_n4, assign16780_e28940_d_n5, assign16780_e28940_d_n6, assign16780_e28940_d_n7, assign16780_e28940_d_n8, assign16780_e28940_d_n9, assign16780_e28940_d_n10, assign16780_e28940_d_n11, assign16780_e28940_d_n13, assign16780_e28940_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard298 != 0.0)) && (locals.var_guard299 == 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_tnom;
        let assign16780_e28923: f64 = (210.0 * __rspice_inv_cse_1);
        let assign16780_e28928: f64 = (210.0 * __rspice_inv_cse_1);
        let assign16780_e28929: f64 = (locals.var_ua2cv_i * assign16780_e28928);
        let assign16780_e28930: f64 = (locals.var_ua1cv_i + assign16780_e28929);
        let assign16780_e28931: f64 = (assign16780_e28923).powf(assign16780_e28930);
        let assign16780_e28932: f64 = (locals.var_uacv_i * assign16780_e28931);
        let assign16780_e28936: f64 = (210.0 - locals.var_tnom);
        let assign16780_e28937: f64 = (locals.var_t2 * assign16780_e28936);
        let assign16780_e28938: f64 = (assign16780_e28932 - assign16780_e28937);
        (assign16780_e28938, (-(locals.var_t2_dn0 * assign16780_e28936)), (-(locals.var_t2_dn2 * assign16780_e28936)), (-(locals.var_t2_dn3 * assign16780_e28936)), (-(locals.var_t2_dn4 * assign16780_e28936)), (-(locals.var_t2_dn5 * assign16780_e28936)), (-(locals.var_t2_dn6 * assign16780_e28936)), (-(locals.var_t2_dn7 * assign16780_e28936)), (-(locals.var_t2_dn8 * assign16780_e28936)), (-(locals.var_t2_dn9 * assign16780_e28936)), (-(locals.var_t2_dn10 * assign16780_e28936)), (-(locals.var_t2_dn11 * assign16780_e28936)), (-(locals.var_t2_dn13 * assign16780_e28936)), (-(locals.var_t2_dn14 * assign16780_e28936)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign16780_e28940;
        locals.var_t1_dn0 = assign16780_e28940_d_n0;
        locals.var_t1_dn2 = assign16780_e28940_d_n2;
        locals.var_t1_dn3 = assign16780_e28940_d_n3;
        locals.var_t1_dn4 = assign16780_e28940_d_n4;
        locals.var_t1_dn5 = assign16780_e28940_d_n5;
        locals.var_t1_dn6 = assign16780_e28940_d_n6;
        locals.var_t1_dn7 = assign16780_e28940_d_n7;
        locals.var_t1_dn8 = assign16780_e28940_d_n8;
        locals.var_t1_dn9 = assign16780_e28940_d_n9;
        locals.var_t1_dn10 = assign16780_e28940_d_n10;
        locals.var_t1_dn11 = assign16780_e28940_d_n11;
        locals.var_t1_dn13 = assign16780_e28940_d_n13;
        locals.var_t1_dn14 = assign16780_e28940_d_n14;

        let (assign16790_e28961, assign16790_e28961_d_n0, assign16790_e28961_d_n2, assign16790_e28961_d_n3, assign16790_e28961_d_n4, assign16790_e28961_d_n5, assign16790_e28961_d_n6, assign16790_e28961_d_n7, assign16790_e28961_d_n8, assign16790_e28961_d_n9, assign16790_e28961_d_n10, assign16790_e28961_d_n11, assign16790_e28961_d_n13, assign16790_e28961_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard298 != 0.0)) && (locals.var_guard299 == 0.0)) {
        let assign16790_e28956: f64 = (locals.var_ua2cv_i * locals.var_tratio);
        let assign16790_e28957: f64 = (locals.var_ua1cv_i + assign16790_e28956);
        let assign16790_e28958: f64 = (locals.var_tratio).powf(assign16790_e28957);
        let assign16790_e28959: f64 = (locals.var_uacv_i * assign16790_e28958);
        (assign16790_e28959, 0.0, 0.0, 0.0, (locals.var_uacv_i * if (locals.var_ua2cv_i * locals.var_tratio_dn4) == 0.0 && ((assign16790_e28957) as f64).is_finite() && ((assign16790_e28957) as f64).fract() == 0.0 { if assign16790_e28957 == 0.0 { 0.0 } else { (assign16790_e28957 * ((locals.var_tratio).powf(assign16790_e28957 - 1.0) * locals.var_tratio_dn4)) } } else { (assign16790_e28958 * (((locals.var_ua2cv_i * locals.var_tratio_dn4) * (locals.var_tratio).ln()) + (assign16790_e28957 * (locals.var_tratio_dn4 / locals.var_tratio)))) }), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uacv_tl, locals.var_uacv_tl_dn0, locals.var_uacv_tl_dn2, locals.var_uacv_tl_dn3, locals.var_uacv_tl_dn4, locals.var_uacv_tl_dn5, locals.var_uacv_tl_dn6, locals.var_uacv_tl_dn7, locals.var_uacv_tl_dn8, locals.var_uacv_tl_dn9, locals.var_uacv_tl_dn10, locals.var_uacv_tl_dn11, locals.var_uacv_tl_dn13, locals.var_uacv_tl_dn14,)
    }
};
        locals.var_uacv_tl = assign16790_e28961;
        locals.var_uacv_tl_dn0 = assign16790_e28961_d_n0;
        locals.var_uacv_tl_dn2 = assign16790_e28961_d_n2;
        locals.var_uacv_tl_dn3 = assign16790_e28961_d_n3;
        locals.var_uacv_tl_dn4 = assign16790_e28961_d_n4;
        locals.var_uacv_tl_dn5 = assign16790_e28961_d_n5;
        locals.var_uacv_tl_dn6 = assign16790_e28961_d_n6;
        locals.var_uacv_tl_dn7 = assign16790_e28961_d_n7;
        locals.var_uacv_tl_dn8 = assign16790_e28961_d_n8;
        locals.var_uacv_tl_dn9 = assign16790_e28961_d_n9;
        locals.var_uacv_tl_dn10 = assign16790_e28961_d_n10;
        locals.var_uacv_tl_dn11 = assign16790_e28961_d_n11;
        locals.var_uacv_tl_dn13 = assign16790_e28961_d_n13;
        locals.var_uacv_tl_dn14 = assign16790_e28961_d_n14;

        let (assign16800_e28978, assign16800_e28978_d_n0, assign16800_e28978_d_n2, assign16800_e28978_d_n3, assign16800_e28978_d_n4, assign16800_e28978_d_n5, assign16800_e28978_d_n6, assign16800_e28978_d_n7, assign16800_e28978_d_n8, assign16800_e28978_d_n9, assign16800_e28978_d_n10, assign16800_e28978_d_n11, assign16800_e28978_d_n13, assign16800_e28978_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard298 != 0.0)) && (locals.var_guard299 == 0.0)) {
        let assign16800_e28975: f64 = (locals.var_t2 * locals.var_deltemp);
        let assign16800_e28976: f64 = (locals.var_t1 + assign16800_e28975);
        (assign16800_e28976, (locals.var_t1_dn0 + (locals.var_t2_dn0 * locals.var_deltemp)), (locals.var_t1_dn2 + (locals.var_t2_dn2 * locals.var_deltemp)), (locals.var_t1_dn3 + (locals.var_t2_dn3 * locals.var_deltemp)), (locals.var_t1_dn4 + ((locals.var_t2_dn4 * locals.var_deltemp) + (locals.var_t2 * locals.var_deltemp_dn4))), (locals.var_t1_dn5 + (locals.var_t2_dn5 * locals.var_deltemp)), (locals.var_t1_dn6 + (locals.var_t2_dn6 * locals.var_deltemp)), (locals.var_t1_dn7 + (locals.var_t2_dn7 * locals.var_deltemp)), (locals.var_t1_dn8 + (locals.var_t2_dn8 * locals.var_deltemp)), (locals.var_t1_dn9 + (locals.var_t2_dn9 * locals.var_deltemp)), (locals.var_t1_dn10 + (locals.var_t2_dn10 * locals.var_deltemp)), (locals.var_t1_dn11 + (locals.var_t2_dn11 * locals.var_deltemp)), (locals.var_t1_dn13 + (locals.var_t2_dn13 * locals.var_deltemp)), (locals.var_t1_dn14 + (locals.var_t2_dn14 * locals.var_deltemp)),)
    } else {
        (locals.var_uacv_th, locals.var_uacv_th_dn0, locals.var_uacv_th_dn2, locals.var_uacv_th_dn3, locals.var_uacv_th_dn4, locals.var_uacv_th_dn5, locals.var_uacv_th_dn6, locals.var_uacv_th_dn7, locals.var_uacv_th_dn8, locals.var_uacv_th_dn9, locals.var_uacv_th_dn10, locals.var_uacv_th_dn11, locals.var_uacv_th_dn13, locals.var_uacv_th_dn14,)
    }
};
        locals.var_uacv_th = assign16800_e28978;
        locals.var_uacv_th_dn0 = assign16800_e28978_d_n0;
        locals.var_uacv_th_dn2 = assign16800_e28978_d_n2;
        locals.var_uacv_th_dn3 = assign16800_e28978_d_n3;
        locals.var_uacv_th_dn4 = assign16800_e28978_d_n4;
        locals.var_uacv_th_dn5 = assign16800_e28978_d_n5;
        locals.var_uacv_th_dn6 = assign16800_e28978_d_n6;
        locals.var_uacv_th_dn7 = assign16800_e28978_d_n7;
        locals.var_uacv_th_dn8 = assign16800_e28978_d_n8;
        locals.var_uacv_th_dn9 = assign16800_e28978_d_n9;
        locals.var_uacv_th_dn10 = assign16800_e28978_d_n10;
        locals.var_uacv_th_dn11 = assign16800_e28978_d_n11;
        locals.var_uacv_th_dn13 = assign16800_e28978_d_n13;
        locals.var_uacv_th_dn14 = assign16800_e28978_d_n14;

        let (assign16810_e28994, assign16810_e28994_d_n0, assign16810_e28994_d_n2, assign16810_e28994_d_n3, assign16810_e28994_d_n4, assign16810_e28994_d_n5, assign16810_e28994_d_n6, assign16810_e28994_d_n7, assign16810_e28994_d_n8, assign16810_e28994_d_n9, assign16810_e28994_d_n10, assign16810_e28994_d_n11, assign16810_e28994_d_n13, assign16810_e28994_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard298 != 0.0)) {
        let assign16810_e28988: f64 = (locals.var_wl * locals.var_uacv_tl);
        let assign16810_e28991: f64 = (locals.var_wh * locals.var_uacv_th);
        let assign16810_e28992: f64 = (assign16810_e28988 + assign16810_e28991);
        (assign16810_e28992, ((locals.var_wl * locals.var_uacv_tl_dn0) + (locals.var_wh * locals.var_uacv_th_dn0)), ((locals.var_wl * locals.var_uacv_tl_dn2) + (locals.var_wh * locals.var_uacv_th_dn2)), ((locals.var_wl * locals.var_uacv_tl_dn3) + (locals.var_wh * locals.var_uacv_th_dn3)), (((locals.var_wl_dn4 * locals.var_uacv_tl) + (locals.var_wl * locals.var_uacv_tl_dn4)) + ((locals.var_wh_dn4 * locals.var_uacv_th) + (locals.var_wh * locals.var_uacv_th_dn4))), ((locals.var_wl * locals.var_uacv_tl_dn5) + (locals.var_wh * locals.var_uacv_th_dn5)), ((locals.var_wl * locals.var_uacv_tl_dn6) + (locals.var_wh * locals.var_uacv_th_dn6)), ((locals.var_wl * locals.var_uacv_tl_dn7) + (locals.var_wh * locals.var_uacv_th_dn7)), ((locals.var_wl * locals.var_uacv_tl_dn8) + (locals.var_wh * locals.var_uacv_th_dn8)), ((locals.var_wl * locals.var_uacv_tl_dn9) + (locals.var_wh * locals.var_uacv_th_dn9)), ((locals.var_wl * locals.var_uacv_tl_dn10) + (locals.var_wh * locals.var_uacv_th_dn10)), ((locals.var_wl * locals.var_uacv_tl_dn11) + (locals.var_wh * locals.var_uacv_th_dn11)), ((locals.var_wl * locals.var_uacv_tl_dn13) + (locals.var_wh * locals.var_uacv_th_dn13)), ((locals.var_wl * locals.var_uacv_tl_dn14) + (locals.var_wh * locals.var_uacv_th_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign16810_e28994;
        locals.var_t0_dn0 = assign16810_e28994_d_n0;
        locals.var_t0_dn2 = assign16810_e28994_d_n2;
        locals.var_t0_dn3 = assign16810_e28994_d_n3;
        locals.var_t0_dn4 = assign16810_e28994_d_n4;
        locals.var_t0_dn5 = assign16810_e28994_d_n5;
        locals.var_t0_dn6 = assign16810_e28994_d_n6;
        locals.var_t0_dn7 = assign16810_e28994_d_n7;
        locals.var_t0_dn8 = assign16810_e28994_d_n8;
        locals.var_t0_dn9 = assign16810_e28994_d_n9;
        locals.var_t0_dn10 = assign16810_e28994_d_n10;
        locals.var_t0_dn11 = assign16810_e28994_d_n11;
        locals.var_t0_dn13 = assign16810_e28994_d_n13;
        locals.var_t0_dn14 = assign16810_e28994_d_n14;

        let (assign16820_e29039, assign16820_e29039_d_n0, assign16820_e29039_d_n2, assign16820_e29039_d_n3, assign16820_e29039_d_n4, assign16820_e29039_d_n5, assign16820_e29039_d_n6, assign16820_e29039_d_n7, assign16820_e29039_d_n8, assign16820_e29039_d_n9, assign16820_e29039_d_n10, assign16820_e29039_d_n11, assign16820_e29039_d_n13, assign16820_e29039_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard298 != 0.0)) {
        let assign16820_e29004: f64 = (-10000.0);
        let assign16820_e29006: f64 = (assign16820_e29004 * 1e-6);
        let (assign16820_e29037, assign16820_e29037_d_n0, assign16820_e29037_d_n2, assign16820_e29037_d_n3, assign16820_e29037_d_n4, assign16820_e29037_d_n5, assign16820_e29037_d_n6, assign16820_e29037_d_n7, assign16820_e29037_d_n8, assign16820_e29037_d_n9, assign16820_e29037_d_n10, assign16820_e29037_d_n11, assign16820_e29037_d_n13, assign16820_e29037_d_n14,) = {
            if (!(locals.var_t0 < assign16820_e29006)) {
                let assign16820_e29013: f64 = (locals.var_t0 * locals.var_t0);
                let assign16820_e29016: f64 = (4.0 * 1e-6);
                let assign16820_e29018: f64 = (assign16820_e29016 * 1e-6);
                let assign16820_e29019: f64 = (assign16820_e29013 + assign16820_e29018);
                let assign16820_e29020: f64 = (assign16820_e29019).sqrt();
                let assign16820_e29021: f64 = (locals.var_t0 + assign16820_e29020);
                let assign16820_e29022: f64 = (0.5 * assign16820_e29021);
                (assign16820_e29022, (0.5 * (locals.var_t0_dn0 + (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign16820_e29020)))), (0.5 * (locals.var_t0_dn2 + (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign16820_e29020)))), (0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign16820_e29020)))), (0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign16820_e29020)))), (0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign16820_e29020)))), (0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign16820_e29020)))), (0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign16820_e29020)))), (0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign16820_e29020)))), (0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign16820_e29020)))), (0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign16820_e29020)))), (0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign16820_e29020)))), (0.5 * (locals.var_t0_dn13 + (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign16820_e29020)))), (0.5 * (locals.var_t0_dn14 + (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign16820_e29020)))),)
            } else {
                let assign16820_e29025: f64 = (-10000.0);
                let assign16820_e29027: f64 = (assign16820_e29025 * 1e-6);
                let (assign16820_e29036, assign16820_e29036_d_n0, assign16820_e29036_d_n2, assign16820_e29036_d_n3, assign16820_e29036_d_n4, assign16820_e29036_d_n5, assign16820_e29036_d_n6, assign16820_e29036_d_n7, assign16820_e29036_d_n8, assign16820_e29036_d_n9, assign16820_e29036_d_n10, assign16820_e29036_d_n11, assign16820_e29036_d_n13, assign16820_e29036_d_n14,) = {
                    if (locals.var_t0 < assign16820_e29027) {
                        let assign16820_e29030: f64 = (-1e-6);
                        let assign16820_e29032: f64 = (assign16820_e29030 * 1e-6);
                        let assign16820_e29034: f64 = (assign16820_e29032 / locals.var_t0);
                        (assign16820_e29034, (-((assign16820_e29032 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((assign16820_e29032 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((assign16820_e29032 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (-((assign16820_e29032 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((assign16820_e29032 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((assign16820_e29032 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((assign16820_e29032 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((assign16820_e29032 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((assign16820_e29032 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), (-((assign16820_e29032 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((assign16820_e29032 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((assign16820_e29032 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))), (-((assign16820_e29032 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign16820_e29036, assign16820_e29036_d_n0, assign16820_e29036_d_n2, assign16820_e29036_d_n3, assign16820_e29036_d_n4, assign16820_e29036_d_n5, assign16820_e29036_d_n6, assign16820_e29036_d_n7, assign16820_e29036_d_n8, assign16820_e29036_d_n9, assign16820_e29036_d_n10, assign16820_e29036_d_n11, assign16820_e29036_d_n13, assign16820_e29036_d_n14,)
            }
        };
        (assign16820_e29037, assign16820_e29037_d_n0, assign16820_e29037_d_n2, assign16820_e29037_d_n3, assign16820_e29037_d_n4, assign16820_e29037_d_n5, assign16820_e29037_d_n6, assign16820_e29037_d_n7, assign16820_e29037_d_n8, assign16820_e29037_d_n9, assign16820_e29037_d_n10, assign16820_e29037_d_n11, assign16820_e29037_d_n13, assign16820_e29037_d_n14,)
    } else {
        (locals.var_uacv_t, locals.var_uacv_t_dn0, locals.var_uacv_t_dn2, locals.var_uacv_t_dn3, locals.var_uacv_t_dn4, locals.var_uacv_t_dn5, locals.var_uacv_t_dn6, locals.var_uacv_t_dn7, locals.var_uacv_t_dn8, locals.var_uacv_t_dn9, locals.var_uacv_t_dn10, locals.var_uacv_t_dn11, locals.var_uacv_t_dn13, locals.var_uacv_t_dn14,)
    }
};
        locals.var_uacv_t = assign16820_e29039;
        locals.var_uacv_t_dn0 = assign16820_e29039_d_n0;
        locals.var_uacv_t_dn2 = assign16820_e29039_d_n2;
        locals.var_uacv_t_dn3 = assign16820_e29039_d_n3;
        locals.var_uacv_t_dn4 = assign16820_e29039_d_n4;
        locals.var_uacv_t_dn5 = assign16820_e29039_d_n5;
        locals.var_uacv_t_dn6 = assign16820_e29039_d_n6;
        locals.var_uacv_t_dn7 = assign16820_e29039_d_n7;
        locals.var_uacv_t_dn8 = assign16820_e29039_d_n8;
        locals.var_uacv_t_dn9 = assign16820_e29039_d_n9;
        locals.var_uacv_t_dn10 = assign16820_e29039_d_n10;
        locals.var_uacv_t_dn11 = assign16820_e29039_d_n11;
        locals.var_uacv_t_dn13 = assign16820_e29039_d_n13;
        locals.var_uacv_t_dn14 = assign16820_e29039_d_n14;

        let (assign16830_e29058, assign16830_e29058_d_n4,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard298 != 0.0)) {
        let assign16830_e29051: f64 = (locals.var_ud2cv_i * locals.var_deltratio1);
        let assign16830_e29052: f64 = (locals.var_ud1cv_i + assign16830_e29051);
        let assign16830_e29054: f64 = (assign16830_e29052 * locals.var_trat_ln);
        let assign16830_e29055: f64 = (assign16830_e29054).exp();
        let assign16830_e29056: f64 = (locals.var_udcv_i * assign16830_e29055);
        (assign16830_e29056, (locals.var_udcv_i * (assign16830_e29055 * (((locals.var_ud2cv_i * locals.var_deltratio1_dn4) * locals.var_trat_ln) + (assign16830_e29052 * locals.var_trat_ln_dn4)))),)
    } else {
        (locals.var_udcv_t, locals.var_udcv_t_dn4,)
    }
};
        locals.var_udcv_t = assign16830_e29058;
        locals.var_udcv_t_dn4 = assign16830_e29058_d_n4;

        let assign16840_e29061: f64 = if locals.var_prt_i == locals.var_prt1_i { 1.0 } else { 0.0 };
        locals.var_guard300 = assign16840_e29061;

        let (assign16850_e29075, assign16850_e29075_d_n0, assign16850_e29075_d_n2, assign16850_e29075_d_n3, assign16850_e29075_d_n4, assign16850_e29075_d_n5, assign16850_e29075_d_n6, assign16850_e29075_d_n7, assign16850_e29075_d_n8, assign16850_e29075_d_n9, assign16850_e29075_d_n10, assign16850_e29075_d_n11, assign16850_e29075_d_n13, assign16850_e29075_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 != 0.0)) {
        let assign16850_e29072: f64 = (locals.var_prt_i * locals.var_deltemp);
        let assign16850_e29073: f64 = (1.0 + assign16850_e29072);
        (assign16850_e29073, 0.0, 0.0, 0.0, (locals.var_prt_i * locals.var_deltemp_dn4), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign16850_e29075;
        locals.var_t2_dn0 = assign16850_e29075_d_n0;
        locals.var_t2_dn2 = assign16850_e29075_d_n2;
        locals.var_t2_dn3 = assign16850_e29075_d_n3;
        locals.var_t2_dn4 = assign16850_e29075_d_n4;
        locals.var_t2_dn5 = assign16850_e29075_d_n5;
        locals.var_t2_dn6 = assign16850_e29075_d_n6;
        locals.var_t2_dn7 = assign16850_e29075_d_n7;
        locals.var_t2_dn8 = assign16850_e29075_d_n8;
        locals.var_t2_dn9 = assign16850_e29075_d_n9;
        locals.var_t2_dn10 = assign16850_e29075_d_n10;
        locals.var_t2_dn11 = assign16850_e29075_d_n11;
        locals.var_t2_dn13 = assign16850_e29075_d_n13;
        locals.var_t2_dn14 = assign16850_e29075_d_n14;

        let assign16860_e29078: f64 = if locals.var_tr0_i < 210.0 { 1.0 } else { 0.0 };
        locals.var_guard301 = assign16860_e29078;

        let assign16870_e29081: f64 = if locals.var_tnom > 210.0 { 1.0 } else { 0.0 };
        locals.var_guard302 = assign16870_e29081;

        let (assign16880_e29100, assign16880_e29100_d_n4,) = {
    if (((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 != 0.0)) {
        let assign16880_e29097: f64 = (locals.var_prt_i * locals.var_deltemp);
        let assign16880_e29098: f64 = (1.0 + assign16880_e29097);
        (assign16880_e29098, (locals.var_prt_i * locals.var_deltemp_dn4),)
    } else {
        (locals.var_rdstemp0, locals.var_rdstemp0_dn4,)
    }
};
        locals.var_rdstemp0 = assign16880_e29100;
        locals.var_rdstemp0_dn4 = assign16880_e29100_d_n4;

        let (assign16890_e29127, assign16890_e29127_d_n4,) = {
    if (((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 != 0.0)) {
        let assign16890_e29117: f64 = (locals.var_devtemp - locals.var_tr0_i);
        let assign16890_e29118: f64 = (locals.var_prt1_i * assign16890_e29117);
        let assign16890_e29119: f64 = (1.0 + assign16890_e29118);
        let assign16890_e29123: f64 = (locals.var_tr0_i - locals.var_tnom);
        let assign16890_e29124: f64 = (locals.var_prt_i * assign16890_e29123);
        let assign16890_e29125: f64 = (assign16890_e29119 + assign16890_e29124);
        (assign16890_e29125, (locals.var_prt1_i * locals.var_devtemp_dn4),)
    } else {
        (locals.var_rdstemp1, locals.var_rdstemp1_dn4,)
    }
};
        locals.var_rdstemp1 = assign16890_e29127;
        locals.var_rdstemp1_dn4 = assign16890_e29127_d_n4;

        let (assign16900_e29148, assign16900_e29148_d_n0, assign16900_e29148_d_n2, assign16900_e29148_d_n3, assign16900_e29148_d_n4, assign16900_e29148_d_n5, assign16900_e29148_d_n6, assign16900_e29148_d_n7, assign16900_e29148_d_n8, assign16900_e29148_d_n9, assign16900_e29148_d_n10, assign16900_e29148_d_n11, assign16900_e29148_d_n13, assign16900_e29148_d_n14,) = {
    if (((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 != 0.0)) {
        let assign16900_e29144: f64 = (210.0 - locals.var_tnom);
        let assign16900_e29145: f64 = (locals.var_prt_i * assign16900_e29144);
        let assign16900_e29146: f64 = (1.0 + assign16900_e29145);
        (assign16900_e29146, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign16900_e29148;
        locals.var_t3_dn0 = assign16900_e29148_d_n0;
        locals.var_t3_dn2 = assign16900_e29148_d_n2;
        locals.var_t3_dn3 = assign16900_e29148_d_n3;
        locals.var_t3_dn4 = assign16900_e29148_d_n4;
        locals.var_t3_dn5 = assign16900_e29148_d_n5;
        locals.var_t3_dn6 = assign16900_e29148_d_n6;
        locals.var_t3_dn7 = assign16900_e29148_d_n7;
        locals.var_t3_dn8 = assign16900_e29148_d_n8;
        locals.var_t3_dn9 = assign16900_e29148_d_n9;
        locals.var_t3_dn10 = assign16900_e29148_d_n10;
        locals.var_t3_dn11 = assign16900_e29148_d_n11;
        locals.var_t3_dn13 = assign16900_e29148_d_n13;
        locals.var_t3_dn14 = assign16900_e29148_d_n14;

        let (assign16910_e29175, assign16910_e29175_d_n0, assign16910_e29175_d_n2, assign16910_e29175_d_n3, assign16910_e29175_d_n4, assign16910_e29175_d_n5, assign16910_e29175_d_n6, assign16910_e29175_d_n7, assign16910_e29175_d_n8, assign16910_e29175_d_n9, assign16910_e29175_d_n10, assign16910_e29175_d_n11, assign16910_e29175_d_n13, assign16910_e29175_d_n14,) = {
    if (((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 != 0.0)) {
        let assign16910_e29165: f64 = (210.0 - locals.var_tr0_i);
        let assign16910_e29166: f64 = (locals.var_prt1_i * assign16910_e29165);
        let assign16910_e29167: f64 = (1.0 + assign16910_e29166);
        let assign16910_e29171: f64 = (locals.var_tr0_i - locals.var_tnom);
        let assign16910_e29172: f64 = (locals.var_prt_i * assign16910_e29171);
        let assign16910_e29173: f64 = (assign16910_e29167 + assign16910_e29172);
        (assign16910_e29173, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign16910_e29175;
        locals.var_t4_dn0 = assign16910_e29175_d_n0;
        locals.var_t4_dn2 = assign16910_e29175_d_n2;
        locals.var_t4_dn3 = assign16910_e29175_d_n3;
        locals.var_t4_dn4 = assign16910_e29175_d_n4;
        locals.var_t4_dn5 = assign16910_e29175_d_n5;
        locals.var_t4_dn6 = assign16910_e29175_d_n6;
        locals.var_t4_dn7 = assign16910_e29175_d_n7;
        locals.var_t4_dn8 = assign16910_e29175_d_n8;
        locals.var_t4_dn9 = assign16910_e29175_d_n9;
        locals.var_t4_dn10 = assign16910_e29175_d_n10;
        locals.var_t4_dn11 = assign16910_e29175_d_n11;
        locals.var_t4_dn13 = assign16910_e29175_d_n13;
        locals.var_t4_dn14 = assign16910_e29175_d_n14;

        let assign16920_e29178: f64 = if locals.var_prt1_i < locals.var_prt_i { 1.0 } else { 0.0 };
        locals.var_guard303 = assign16920_e29178;

        let (assign16930_e29237, assign16930_e29237_d_n0, assign16930_e29237_d_n2, assign16930_e29237_d_n3, assign16930_e29237_d_n4, assign16930_e29237_d_n5, assign16930_e29237_d_n6, assign16930_e29237_d_n7, assign16930_e29237_d_n8, assign16930_e29237_d_n9, assign16930_e29237_d_n10, assign16930_e29237_d_n11, assign16930_e29237_d_n13, assign16930_e29237_d_n14,) = {
    if ((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 != 0.0)) && (locals.var_guard303 != 0.0)) {
        let assign16930_e29196: f64 = (locals.var_rdstemp0 + locals.var_rdstemp1);
        let assign16930_e29199: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign16930_e29202: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign16930_e29203: f64 = (assign16930_e29199 * assign16930_e29202);
        let assign16930_e29206: f64 = (0.25 * locals.var_sprt_i);
        let assign16930_e29208: f64 = (assign16930_e29206 * locals.var_sprt_i);
        let assign16930_e29209: f64 = (assign16930_e29203 + assign16930_e29208);
        let assign16930_e29210: f64 = (assign16930_e29209).sqrt();
        let assign16930_e29211: f64 = (assign16930_e29196 + assign16930_e29210);
        let assign16930_e29212: f64 = (0.5 * assign16930_e29211);
        let assign16930_e29216: f64 = (locals.var_t3 + locals.var_t4);
        let assign16930_e29219: f64 = (locals.var_t3 - locals.var_t4);
        let assign16930_e29222: f64 = (locals.var_t3 - locals.var_t4);
        let assign16930_e29223: f64 = (assign16930_e29219 * assign16930_e29222);
        let assign16930_e29226: f64 = (0.25 * locals.var_sprt_i);
        let assign16930_e29228: f64 = (assign16930_e29226 * locals.var_sprt_i);
        let assign16930_e29229: f64 = (assign16930_e29223 + assign16930_e29228);
        let assign16930_e29230: f64 = (assign16930_e29229).sqrt();
        let assign16930_e29231: f64 = (assign16930_e29216 + assign16930_e29230);
        let assign16930_e29232: f64 = (0.5 * assign16930_e29231);
        let assign16930_e29233: f64 = (assign16930_e29212 - assign16930_e29232);
        let assign16930_e29235: f64 = (assign16930_e29233 + locals.var_t3);
        (assign16930_e29235, ((-(0.5 * ((locals.var_t3_dn0 + locals.var_t4_dn0) + ((((locals.var_t3_dn0 - locals.var_t4_dn0) * assign16930_e29222) + (assign16930_e29219 * (locals.var_t3_dn0 - locals.var_t4_dn0))) / (2.0 * assign16930_e29230))))) + locals.var_t3_dn0), ((-(0.5 * ((locals.var_t3_dn2 + locals.var_t4_dn2) + ((((locals.var_t3_dn2 - locals.var_t4_dn2) * assign16930_e29222) + (assign16930_e29219 * (locals.var_t3_dn2 - locals.var_t4_dn2))) / (2.0 * assign16930_e29230))))) + locals.var_t3_dn2), ((-(0.5 * ((locals.var_t3_dn3 + locals.var_t4_dn3) + ((((locals.var_t3_dn3 - locals.var_t4_dn3) * assign16930_e29222) + (assign16930_e29219 * (locals.var_t3_dn3 - locals.var_t4_dn3))) / (2.0 * assign16930_e29230))))) + locals.var_t3_dn3), (((0.5 * ((locals.var_rdstemp0_dn4 + locals.var_rdstemp1_dn4) + ((((locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4) * assign16930_e29202) + (assign16930_e29199 * (locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4))) / (2.0 * assign16930_e29210)))) - (0.5 * ((locals.var_t3_dn4 + locals.var_t4_dn4) + ((((locals.var_t3_dn4 - locals.var_t4_dn4) * assign16930_e29222) + (assign16930_e29219 * (locals.var_t3_dn4 - locals.var_t4_dn4))) / (2.0 * assign16930_e29230))))) + locals.var_t3_dn4), ((-(0.5 * ((locals.var_t3_dn5 + locals.var_t4_dn5) + ((((locals.var_t3_dn5 - locals.var_t4_dn5) * assign16930_e29222) + (assign16930_e29219 * (locals.var_t3_dn5 - locals.var_t4_dn5))) / (2.0 * assign16930_e29230))))) + locals.var_t3_dn5), ((-(0.5 * ((locals.var_t3_dn6 + locals.var_t4_dn6) + ((((locals.var_t3_dn6 - locals.var_t4_dn6) * assign16930_e29222) + (assign16930_e29219 * (locals.var_t3_dn6 - locals.var_t4_dn6))) / (2.0 * assign16930_e29230))))) + locals.var_t3_dn6), ((-(0.5 * ((locals.var_t3_dn7 + locals.var_t4_dn7) + ((((locals.var_t3_dn7 - locals.var_t4_dn7) * assign16930_e29222) + (assign16930_e29219 * (locals.var_t3_dn7 - locals.var_t4_dn7))) / (2.0 * assign16930_e29230))))) + locals.var_t3_dn7), ((-(0.5 * ((locals.var_t3_dn8 + locals.var_t4_dn8) + ((((locals.var_t3_dn8 - locals.var_t4_dn8) * assign16930_e29222) + (assign16930_e29219 * (locals.var_t3_dn8 - locals.var_t4_dn8))) / (2.0 * assign16930_e29230))))) + locals.var_t3_dn8), ((-(0.5 * ((locals.var_t3_dn9 + locals.var_t4_dn9) + ((((locals.var_t3_dn9 - locals.var_t4_dn9) * assign16930_e29222) + (assign16930_e29219 * (locals.var_t3_dn9 - locals.var_t4_dn9))) / (2.0 * assign16930_e29230))))) + locals.var_t3_dn9), ((-(0.5 * ((locals.var_t3_dn10 + locals.var_t4_dn10) + ((((locals.var_t3_dn10 - locals.var_t4_dn10) * assign16930_e29222) + (assign16930_e29219 * (locals.var_t3_dn10 - locals.var_t4_dn10))) / (2.0 * assign16930_e29230))))) + locals.var_t3_dn10), ((-(0.5 * ((locals.var_t3_dn11 + locals.var_t4_dn11) + ((((locals.var_t3_dn11 - locals.var_t4_dn11) * assign16930_e29222) + (assign16930_e29219 * (locals.var_t3_dn11 - locals.var_t4_dn11))) / (2.0 * assign16930_e29230))))) + locals.var_t3_dn11), ((-(0.5 * ((locals.var_t3_dn13 + locals.var_t4_dn13) + ((((locals.var_t3_dn13 - locals.var_t4_dn13) * assign16930_e29222) + (assign16930_e29219 * (locals.var_t3_dn13 - locals.var_t4_dn13))) / (2.0 * assign16930_e29230))))) + locals.var_t3_dn13), ((-(0.5 * ((locals.var_t3_dn14 + locals.var_t4_dn14) + ((((locals.var_t3_dn14 - locals.var_t4_dn14) * assign16930_e29222) + (assign16930_e29219 * (locals.var_t3_dn14 - locals.var_t4_dn14))) / (2.0 * assign16930_e29230))))) + locals.var_t3_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign16930_e29237;
        locals.var_t5_dn0 = assign16930_e29237_d_n0;
        locals.var_t5_dn2 = assign16930_e29237_d_n2;
        locals.var_t5_dn3 = assign16930_e29237_d_n3;
        locals.var_t5_dn4 = assign16930_e29237_d_n4;
        locals.var_t5_dn5 = assign16930_e29237_d_n5;
        locals.var_t5_dn6 = assign16930_e29237_d_n6;
        locals.var_t5_dn7 = assign16930_e29237_d_n7;
        locals.var_t5_dn8 = assign16930_e29237_d_n8;
        locals.var_t5_dn9 = assign16930_e29237_d_n9;
        locals.var_t5_dn10 = assign16930_e29237_d_n10;
        locals.var_t5_dn11 = assign16930_e29237_d_n11;
        locals.var_t5_dn13 = assign16930_e29237_d_n13;
        locals.var_t5_dn14 = assign16930_e29237_d_n14;

        let (assign16940_e29273, assign16940_e29273_d_n0, assign16940_e29273_d_n2, assign16940_e29273_d_n3, assign16940_e29273_d_n4, assign16940_e29273_d_n5, assign16940_e29273_d_n6, assign16940_e29273_d_n7, assign16940_e29273_d_n8, assign16940_e29273_d_n9, assign16940_e29273_d_n10, assign16940_e29273_d_n11, assign16940_e29273_d_n13, assign16940_e29273_d_n14,) = {
    if ((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 != 0.0)) && (locals.var_guard303 != 0.0)) {
        let assign16940_e29255: f64 = (locals.var_t5 + locals.var_rdstemp0);
        let assign16940_e29258: f64 = (locals.var_t5 - locals.var_rdstemp0);
        let assign16940_e29261: f64 = (locals.var_t5 - locals.var_rdstemp0);
        let assign16940_e29262: f64 = (assign16940_e29258 * assign16940_e29261);
        let assign16940_e29265: f64 = (0.25 * 0.001);
        let assign16940_e29267: f64 = (assign16940_e29265 * 0.001);
        let assign16940_e29268: f64 = (assign16940_e29262 + assign16940_e29267);
        let assign16940_e29269: f64 = (assign16940_e29268).sqrt();
        let assign16940_e29270: f64 = (assign16940_e29255 + assign16940_e29269);
        let assign16940_e29271: f64 = (0.5 * assign16940_e29270);
        (assign16940_e29271, (0.5 * (locals.var_t5_dn0 + (((locals.var_t5_dn0 * assign16940_e29261) + (assign16940_e29258 * locals.var_t5_dn0)) / (2.0 * assign16940_e29269)))), (0.5 * (locals.var_t5_dn2 + (((locals.var_t5_dn2 * assign16940_e29261) + (assign16940_e29258 * locals.var_t5_dn2)) / (2.0 * assign16940_e29269)))), (0.5 * (locals.var_t5_dn3 + (((locals.var_t5_dn3 * assign16940_e29261) + (assign16940_e29258 * locals.var_t5_dn3)) / (2.0 * assign16940_e29269)))), (0.5 * ((locals.var_t5_dn4 + locals.var_rdstemp0_dn4) + ((((locals.var_t5_dn4 - locals.var_rdstemp0_dn4) * assign16940_e29261) + (assign16940_e29258 * (locals.var_t5_dn4 - locals.var_rdstemp0_dn4))) / (2.0 * assign16940_e29269)))), (0.5 * (locals.var_t5_dn5 + (((locals.var_t5_dn5 * assign16940_e29261) + (assign16940_e29258 * locals.var_t5_dn5)) / (2.0 * assign16940_e29269)))), (0.5 * (locals.var_t5_dn6 + (((locals.var_t5_dn6 * assign16940_e29261) + (assign16940_e29258 * locals.var_t5_dn6)) / (2.0 * assign16940_e29269)))), (0.5 * (locals.var_t5_dn7 + (((locals.var_t5_dn7 * assign16940_e29261) + (assign16940_e29258 * locals.var_t5_dn7)) / (2.0 * assign16940_e29269)))), (0.5 * (locals.var_t5_dn8 + (((locals.var_t5_dn8 * assign16940_e29261) + (assign16940_e29258 * locals.var_t5_dn8)) / (2.0 * assign16940_e29269)))), (0.5 * (locals.var_t5_dn9 + (((locals.var_t5_dn9 * assign16940_e29261) + (assign16940_e29258 * locals.var_t5_dn9)) / (2.0 * assign16940_e29269)))), (0.5 * (locals.var_t5_dn10 + (((locals.var_t5_dn10 * assign16940_e29261) + (assign16940_e29258 * locals.var_t5_dn10)) / (2.0 * assign16940_e29269)))), (0.5 * (locals.var_t5_dn11 + (((locals.var_t5_dn11 * assign16940_e29261) + (assign16940_e29258 * locals.var_t5_dn11)) / (2.0 * assign16940_e29269)))), (0.5 * (locals.var_t5_dn13 + (((locals.var_t5_dn13 * assign16940_e29261) + (assign16940_e29258 * locals.var_t5_dn13)) / (2.0 * assign16940_e29269)))), (0.5 * (locals.var_t5_dn14 + (((locals.var_t5_dn14 * assign16940_e29261) + (assign16940_e29258 * locals.var_t5_dn14)) / (2.0 * assign16940_e29269)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign16940_e29273;
        locals.var_t2_dn0 = assign16940_e29273_d_n0;
        locals.var_t2_dn2 = assign16940_e29273_d_n2;
        locals.var_t2_dn3 = assign16940_e29273_d_n3;
        locals.var_t2_dn4 = assign16940_e29273_d_n4;
        locals.var_t2_dn5 = assign16940_e29273_d_n5;
        locals.var_t2_dn6 = assign16940_e29273_d_n6;
        locals.var_t2_dn7 = assign16940_e29273_d_n7;
        locals.var_t2_dn8 = assign16940_e29273_d_n8;
        locals.var_t2_dn9 = assign16940_e29273_d_n9;
        locals.var_t2_dn10 = assign16940_e29273_d_n10;
        locals.var_t2_dn11 = assign16940_e29273_d_n11;
        locals.var_t2_dn13 = assign16940_e29273_d_n13;
        locals.var_t2_dn14 = assign16940_e29273_d_n14;

        let (assign16950_e29333, assign16950_e29333_d_n0, assign16950_e29333_d_n2, assign16950_e29333_d_n3, assign16950_e29333_d_n4, assign16950_e29333_d_n5, assign16950_e29333_d_n6, assign16950_e29333_d_n7, assign16950_e29333_d_n8, assign16950_e29333_d_n9, assign16950_e29333_d_n10, assign16950_e29333_d_n11, assign16950_e29333_d_n13, assign16950_e29333_d_n14,) = {
    if ((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 != 0.0)) && (locals.var_guard303 == 0.0)) {
        let assign16950_e29292: f64 = (locals.var_rdstemp0 + locals.var_rdstemp1);
        let assign16950_e29295: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign16950_e29298: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign16950_e29299: f64 = (assign16950_e29295 * assign16950_e29298);
        let assign16950_e29302: f64 = (0.25 * locals.var_sprt_i);
        let assign16950_e29304: f64 = (assign16950_e29302 * locals.var_sprt_i);
        let assign16950_e29305: f64 = (assign16950_e29299 + assign16950_e29304);
        let assign16950_e29306: f64 = (assign16950_e29305).sqrt();
        let assign16950_e29307: f64 = (assign16950_e29292 - assign16950_e29306);
        let assign16950_e29308: f64 = (0.5 * assign16950_e29307);
        let assign16950_e29312: f64 = (locals.var_t3 + locals.var_t4);
        let assign16950_e29315: f64 = (locals.var_t3 - locals.var_t4);
        let assign16950_e29318: f64 = (locals.var_t3 - locals.var_t4);
        let assign16950_e29319: f64 = (assign16950_e29315 * assign16950_e29318);
        let assign16950_e29322: f64 = (0.25 * locals.var_sprt_i);
        let assign16950_e29324: f64 = (assign16950_e29322 * locals.var_sprt_i);
        let assign16950_e29325: f64 = (assign16950_e29319 + assign16950_e29324);
        let assign16950_e29326: f64 = (assign16950_e29325).sqrt();
        let assign16950_e29327: f64 = (assign16950_e29312 - assign16950_e29326);
        let assign16950_e29328: f64 = (0.5 * assign16950_e29327);
        let assign16950_e29329: f64 = (assign16950_e29308 - assign16950_e29328);
        let assign16950_e29331: f64 = (assign16950_e29329 + locals.var_t3);
        (assign16950_e29331, ((-(0.5 * ((locals.var_t3_dn0 + locals.var_t4_dn0) - ((((locals.var_t3_dn0 - locals.var_t4_dn0) * assign16950_e29318) + (assign16950_e29315 * (locals.var_t3_dn0 - locals.var_t4_dn0))) / (2.0 * assign16950_e29326))))) + locals.var_t3_dn0), ((-(0.5 * ((locals.var_t3_dn2 + locals.var_t4_dn2) - ((((locals.var_t3_dn2 - locals.var_t4_dn2) * assign16950_e29318) + (assign16950_e29315 * (locals.var_t3_dn2 - locals.var_t4_dn2))) / (2.0 * assign16950_e29326))))) + locals.var_t3_dn2), ((-(0.5 * ((locals.var_t3_dn3 + locals.var_t4_dn3) - ((((locals.var_t3_dn3 - locals.var_t4_dn3) * assign16950_e29318) + (assign16950_e29315 * (locals.var_t3_dn3 - locals.var_t4_dn3))) / (2.0 * assign16950_e29326))))) + locals.var_t3_dn3), (((0.5 * ((locals.var_rdstemp0_dn4 + locals.var_rdstemp1_dn4) - ((((locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4) * assign16950_e29298) + (assign16950_e29295 * (locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4))) / (2.0 * assign16950_e29306)))) - (0.5 * ((locals.var_t3_dn4 + locals.var_t4_dn4) - ((((locals.var_t3_dn4 - locals.var_t4_dn4) * assign16950_e29318) + (assign16950_e29315 * (locals.var_t3_dn4 - locals.var_t4_dn4))) / (2.0 * assign16950_e29326))))) + locals.var_t3_dn4), ((-(0.5 * ((locals.var_t3_dn5 + locals.var_t4_dn5) - ((((locals.var_t3_dn5 - locals.var_t4_dn5) * assign16950_e29318) + (assign16950_e29315 * (locals.var_t3_dn5 - locals.var_t4_dn5))) / (2.0 * assign16950_e29326))))) + locals.var_t3_dn5), ((-(0.5 * ((locals.var_t3_dn6 + locals.var_t4_dn6) - ((((locals.var_t3_dn6 - locals.var_t4_dn6) * assign16950_e29318) + (assign16950_e29315 * (locals.var_t3_dn6 - locals.var_t4_dn6))) / (2.0 * assign16950_e29326))))) + locals.var_t3_dn6), ((-(0.5 * ((locals.var_t3_dn7 + locals.var_t4_dn7) - ((((locals.var_t3_dn7 - locals.var_t4_dn7) * assign16950_e29318) + (assign16950_e29315 * (locals.var_t3_dn7 - locals.var_t4_dn7))) / (2.0 * assign16950_e29326))))) + locals.var_t3_dn7), ((-(0.5 * ((locals.var_t3_dn8 + locals.var_t4_dn8) - ((((locals.var_t3_dn8 - locals.var_t4_dn8) * assign16950_e29318) + (assign16950_e29315 * (locals.var_t3_dn8 - locals.var_t4_dn8))) / (2.0 * assign16950_e29326))))) + locals.var_t3_dn8), ((-(0.5 * ((locals.var_t3_dn9 + locals.var_t4_dn9) - ((((locals.var_t3_dn9 - locals.var_t4_dn9) * assign16950_e29318) + (assign16950_e29315 * (locals.var_t3_dn9 - locals.var_t4_dn9))) / (2.0 * assign16950_e29326))))) + locals.var_t3_dn9), ((-(0.5 * ((locals.var_t3_dn10 + locals.var_t4_dn10) - ((((locals.var_t3_dn10 - locals.var_t4_dn10) * assign16950_e29318) + (assign16950_e29315 * (locals.var_t3_dn10 - locals.var_t4_dn10))) / (2.0 * assign16950_e29326))))) + locals.var_t3_dn10), ((-(0.5 * ((locals.var_t3_dn11 + locals.var_t4_dn11) - ((((locals.var_t3_dn11 - locals.var_t4_dn11) * assign16950_e29318) + (assign16950_e29315 * (locals.var_t3_dn11 - locals.var_t4_dn11))) / (2.0 * assign16950_e29326))))) + locals.var_t3_dn11), ((-(0.5 * ((locals.var_t3_dn13 + locals.var_t4_dn13) - ((((locals.var_t3_dn13 - locals.var_t4_dn13) * assign16950_e29318) + (assign16950_e29315 * (locals.var_t3_dn13 - locals.var_t4_dn13))) / (2.0 * assign16950_e29326))))) + locals.var_t3_dn13), ((-(0.5 * ((locals.var_t3_dn14 + locals.var_t4_dn14) - ((((locals.var_t3_dn14 - locals.var_t4_dn14) * assign16950_e29318) + (assign16950_e29315 * (locals.var_t3_dn14 - locals.var_t4_dn14))) / (2.0 * assign16950_e29326))))) + locals.var_t3_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign16950_e29333;
        locals.var_t5_dn0 = assign16950_e29333_d_n0;
        locals.var_t5_dn2 = assign16950_e29333_d_n2;
        locals.var_t5_dn3 = assign16950_e29333_d_n3;
        locals.var_t5_dn4 = assign16950_e29333_d_n4;
        locals.var_t5_dn5 = assign16950_e29333_d_n5;
        locals.var_t5_dn6 = assign16950_e29333_d_n6;
        locals.var_t5_dn7 = assign16950_e29333_d_n7;
        locals.var_t5_dn8 = assign16950_e29333_d_n8;
        locals.var_t5_dn9 = assign16950_e29333_d_n9;
        locals.var_t5_dn10 = assign16950_e29333_d_n10;
        locals.var_t5_dn11 = assign16950_e29333_d_n11;
        locals.var_t5_dn13 = assign16950_e29333_d_n13;
        locals.var_t5_dn14 = assign16950_e29333_d_n14;

    }

    pub(super) fn stamp_transient_block_54(
        locals: &mut StampLocals,
    ) {
        let (assign16960_e29370, assign16960_e29370_d_n0, assign16960_e29370_d_n2, assign16960_e29370_d_n3, assign16960_e29370_d_n4, assign16960_e29370_d_n5, assign16960_e29370_d_n6, assign16960_e29370_d_n7, assign16960_e29370_d_n8, assign16960_e29370_d_n9, assign16960_e29370_d_n10, assign16960_e29370_d_n11, assign16960_e29370_d_n13, assign16960_e29370_d_n14,) = {
    if ((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 != 0.0)) && (locals.var_guard303 == 0.0)) {
        let assign16960_e29352: f64 = (locals.var_t5 + locals.var_rdstemp0);
        let assign16960_e29355: f64 = (locals.var_t5 - locals.var_rdstemp0);
        let assign16960_e29358: f64 = (locals.var_t5 - locals.var_rdstemp0);
        let assign16960_e29359: f64 = (assign16960_e29355 * assign16960_e29358);
        let assign16960_e29362: f64 = (0.25 * 0.001);
        let assign16960_e29364: f64 = (assign16960_e29362 * 0.001);
        let assign16960_e29365: f64 = (assign16960_e29359 + assign16960_e29364);
        let assign16960_e29366: f64 = (assign16960_e29365).sqrt();
        let assign16960_e29367: f64 = (assign16960_e29352 - assign16960_e29366);
        let assign16960_e29368: f64 = (0.5 * assign16960_e29367);
        (assign16960_e29368, (0.5 * (locals.var_t5_dn0 - (((locals.var_t5_dn0 * assign16960_e29358) + (assign16960_e29355 * locals.var_t5_dn0)) / (2.0 * assign16960_e29366)))), (0.5 * (locals.var_t5_dn2 - (((locals.var_t5_dn2 * assign16960_e29358) + (assign16960_e29355 * locals.var_t5_dn2)) / (2.0 * assign16960_e29366)))), (0.5 * (locals.var_t5_dn3 - (((locals.var_t5_dn3 * assign16960_e29358) + (assign16960_e29355 * locals.var_t5_dn3)) / (2.0 * assign16960_e29366)))), (0.5 * ((locals.var_t5_dn4 + locals.var_rdstemp0_dn4) - ((((locals.var_t5_dn4 - locals.var_rdstemp0_dn4) * assign16960_e29358) + (assign16960_e29355 * (locals.var_t5_dn4 - locals.var_rdstemp0_dn4))) / (2.0 * assign16960_e29366)))), (0.5 * (locals.var_t5_dn5 - (((locals.var_t5_dn5 * assign16960_e29358) + (assign16960_e29355 * locals.var_t5_dn5)) / (2.0 * assign16960_e29366)))), (0.5 * (locals.var_t5_dn6 - (((locals.var_t5_dn6 * assign16960_e29358) + (assign16960_e29355 * locals.var_t5_dn6)) / (2.0 * assign16960_e29366)))), (0.5 * (locals.var_t5_dn7 - (((locals.var_t5_dn7 * assign16960_e29358) + (assign16960_e29355 * locals.var_t5_dn7)) / (2.0 * assign16960_e29366)))), (0.5 * (locals.var_t5_dn8 - (((locals.var_t5_dn8 * assign16960_e29358) + (assign16960_e29355 * locals.var_t5_dn8)) / (2.0 * assign16960_e29366)))), (0.5 * (locals.var_t5_dn9 - (((locals.var_t5_dn9 * assign16960_e29358) + (assign16960_e29355 * locals.var_t5_dn9)) / (2.0 * assign16960_e29366)))), (0.5 * (locals.var_t5_dn10 - (((locals.var_t5_dn10 * assign16960_e29358) + (assign16960_e29355 * locals.var_t5_dn10)) / (2.0 * assign16960_e29366)))), (0.5 * (locals.var_t5_dn11 - (((locals.var_t5_dn11 * assign16960_e29358) + (assign16960_e29355 * locals.var_t5_dn11)) / (2.0 * assign16960_e29366)))), (0.5 * (locals.var_t5_dn13 - (((locals.var_t5_dn13 * assign16960_e29358) + (assign16960_e29355 * locals.var_t5_dn13)) / (2.0 * assign16960_e29366)))), (0.5 * (locals.var_t5_dn14 - (((locals.var_t5_dn14 * assign16960_e29358) + (assign16960_e29355 * locals.var_t5_dn14)) / (2.0 * assign16960_e29366)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign16960_e29370;
        locals.var_t2_dn0 = assign16960_e29370_d_n0;
        locals.var_t2_dn2 = assign16960_e29370_d_n2;
        locals.var_t2_dn3 = assign16960_e29370_d_n3;
        locals.var_t2_dn4 = assign16960_e29370_d_n4;
        locals.var_t2_dn5 = assign16960_e29370_d_n5;
        locals.var_t2_dn6 = assign16960_e29370_d_n6;
        locals.var_t2_dn7 = assign16960_e29370_d_n7;
        locals.var_t2_dn8 = assign16960_e29370_d_n8;
        locals.var_t2_dn9 = assign16960_e29370_d_n9;
        locals.var_t2_dn10 = assign16960_e29370_d_n10;
        locals.var_t2_dn11 = assign16960_e29370_d_n11;
        locals.var_t2_dn13 = assign16960_e29370_d_n13;
        locals.var_t2_dn14 = assign16960_e29370_d_n14;

        let assign16970_e29373: f64 = if locals.var_tnom > locals.var_tr0_i { 1.0 } else { 0.0 };
        locals.var_guard304 = assign16970_e29373;

        let (assign16980_e29395, assign16980_e29395_d_n4,) = {
    if ((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 != 0.0)) {
        let assign16980_e29392: f64 = (locals.var_prt_i * locals.var_deltemp);
        let assign16980_e29393: f64 = (1.0 + assign16980_e29392);
        (assign16980_e29393, (locals.var_prt_i * locals.var_deltemp_dn4),)
    } else {
        (locals.var_rdstemp0, locals.var_rdstemp0_dn4,)
    }
};
        locals.var_rdstemp0 = assign16980_e29395;
        locals.var_rdstemp0_dn4 = assign16980_e29395_d_n4;

        let (assign16990_e29425, assign16990_e29425_d_n4,) = {
    if ((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 != 0.0)) {
        let assign16990_e29415: f64 = (locals.var_devtemp - locals.var_tr0_i);
        let assign16990_e29416: f64 = (locals.var_prt1_i * assign16990_e29415);
        let assign16990_e29417: f64 = (1.0 + assign16990_e29416);
        let assign16990_e29421: f64 = (locals.var_tr0_i - locals.var_tnom);
        let assign16990_e29422: f64 = (locals.var_prt_i * assign16990_e29421);
        let assign16990_e29423: f64 = (assign16990_e29417 + assign16990_e29422);
        (assign16990_e29423, (locals.var_prt1_i * locals.var_devtemp_dn4),)
    } else {
        (locals.var_rdstemp1, locals.var_rdstemp1_dn4,)
    }
};
        locals.var_rdstemp1 = assign16990_e29425;
        locals.var_rdstemp1_dn4 = assign16990_e29425_d_n4;

        let (assign17000_e29449, assign17000_e29449_d_n0, assign17000_e29449_d_n2, assign17000_e29449_d_n3, assign17000_e29449_d_n4, assign17000_e29449_d_n5, assign17000_e29449_d_n6, assign17000_e29449_d_n7, assign17000_e29449_d_n8, assign17000_e29449_d_n9, assign17000_e29449_d_n10, assign17000_e29449_d_n11, assign17000_e29449_d_n13, assign17000_e29449_d_n14,) = {
    if ((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 != 0.0)) {
        let assign17000_e29443: f64 = (locals.var_prt_i - locals.var_prt1_i);
        let assign17000_e29446: f64 = (locals.var_tr0_i - locals.var_tnom);
        let assign17000_e29447: f64 = (assign17000_e29443 * assign17000_e29446);
        (assign17000_e29447, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign17000_e29449;
        locals.var_t3_dn0 = assign17000_e29449_d_n0;
        locals.var_t3_dn2 = assign17000_e29449_d_n2;
        locals.var_t3_dn3 = assign17000_e29449_d_n3;
        locals.var_t3_dn4 = assign17000_e29449_d_n4;
        locals.var_t3_dn5 = assign17000_e29449_d_n5;
        locals.var_t3_dn6 = assign17000_e29449_d_n6;
        locals.var_t3_dn7 = assign17000_e29449_d_n7;
        locals.var_t3_dn8 = assign17000_e29449_d_n8;
        locals.var_t3_dn9 = assign17000_e29449_d_n9;
        locals.var_t3_dn10 = assign17000_e29449_d_n10;
        locals.var_t3_dn11 = assign17000_e29449_d_n11;
        locals.var_t3_dn13 = assign17000_e29449_d_n13;
        locals.var_t3_dn14 = assign17000_e29449_d_n14;

        let (assign17010_e29473, assign17010_e29473_d_n0, assign17010_e29473_d_n2, assign17010_e29473_d_n3, assign17010_e29473_d_n4, assign17010_e29473_d_n5, assign17010_e29473_d_n6, assign17010_e29473_d_n7, assign17010_e29473_d_n8, assign17010_e29473_d_n9, assign17010_e29473_d_n10, assign17010_e29473_d_n11, assign17010_e29473_d_n13, assign17010_e29473_d_n14,) = {
    if ((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 != 0.0)) {
        let assign17010_e29469: f64 = (210.0 - locals.var_tnom);
        let assign17010_e29470: f64 = (locals.var_prt_i * assign17010_e29469);
        let assign17010_e29471: f64 = (1.0 + assign17010_e29470);
        (assign17010_e29471, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign17010_e29473;
        locals.var_t4_dn0 = assign17010_e29473_d_n0;
        locals.var_t4_dn2 = assign17010_e29473_d_n2;
        locals.var_t4_dn3 = assign17010_e29473_d_n3;
        locals.var_t4_dn4 = assign17010_e29473_d_n4;
        locals.var_t4_dn5 = assign17010_e29473_d_n5;
        locals.var_t4_dn6 = assign17010_e29473_d_n6;
        locals.var_t4_dn7 = assign17010_e29473_d_n7;
        locals.var_t4_dn8 = assign17010_e29473_d_n8;
        locals.var_t4_dn9 = assign17010_e29473_d_n9;
        locals.var_t4_dn10 = assign17010_e29473_d_n10;
        locals.var_t4_dn11 = assign17010_e29473_d_n11;
        locals.var_t4_dn13 = assign17010_e29473_d_n13;
        locals.var_t4_dn14 = assign17010_e29473_d_n14;

        let (assign17020_e29503, assign17020_e29503_d_n0, assign17020_e29503_d_n2, assign17020_e29503_d_n3, assign17020_e29503_d_n4, assign17020_e29503_d_n5, assign17020_e29503_d_n6, assign17020_e29503_d_n7, assign17020_e29503_d_n8, assign17020_e29503_d_n9, assign17020_e29503_d_n10, assign17020_e29503_d_n11, assign17020_e29503_d_n13, assign17020_e29503_d_n14,) = {
    if ((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 != 0.0)) {
        let assign17020_e29493: f64 = (210.0 - locals.var_tr0_i);
        let assign17020_e29494: f64 = (locals.var_prt1_i * assign17020_e29493);
        let assign17020_e29495: f64 = (1.0 + assign17020_e29494);
        let assign17020_e29499: f64 = (locals.var_tr0_i - locals.var_tnom);
        let assign17020_e29500: f64 = (locals.var_prt_i * assign17020_e29499);
        let assign17020_e29501: f64 = (assign17020_e29495 + assign17020_e29500);
        (assign17020_e29501, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign17020_e29503;
        locals.var_t5_dn0 = assign17020_e29503_d_n0;
        locals.var_t5_dn2 = assign17020_e29503_d_n2;
        locals.var_t5_dn3 = assign17020_e29503_d_n3;
        locals.var_t5_dn4 = assign17020_e29503_d_n4;
        locals.var_t5_dn5 = assign17020_e29503_d_n5;
        locals.var_t5_dn6 = assign17020_e29503_d_n6;
        locals.var_t5_dn7 = assign17020_e29503_d_n7;
        locals.var_t5_dn8 = assign17020_e29503_d_n8;
        locals.var_t5_dn9 = assign17020_e29503_d_n9;
        locals.var_t5_dn10 = assign17020_e29503_d_n10;
        locals.var_t5_dn11 = assign17020_e29503_d_n11;
        locals.var_t5_dn13 = assign17020_e29503_d_n13;
        locals.var_t5_dn14 = assign17020_e29503_d_n14;

        let assign17030_e29506: f64 = if locals.var_prt1_i < locals.var_prt_i { 1.0 } else { 0.0 };
        locals.var_guard305 = assign17030_e29506;

        let (assign17040_e29566, assign17040_e29566_d_n0, assign17040_e29566_d_n2, assign17040_e29566_d_n3, assign17040_e29566_d_n4, assign17040_e29566_d_n5, assign17040_e29566_d_n6, assign17040_e29566_d_n7, assign17040_e29566_d_n8, assign17040_e29566_d_n9, assign17040_e29566_d_n10, assign17040_e29566_d_n11, assign17040_e29566_d_n13, assign17040_e29566_d_n14,) = {
    if (((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 != 0.0)) && (locals.var_guard305 != 0.0)) {
        let assign17040_e29527: f64 = (locals.var_rdstemp0 + locals.var_rdstemp1);
        let assign17040_e29530: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign17040_e29533: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign17040_e29534: f64 = (assign17040_e29530 * assign17040_e29533);
        let assign17040_e29537: f64 = (0.25 * locals.var_sprt_i);
        let assign17040_e29539: f64 = (assign17040_e29537 * locals.var_sprt_i);
        let assign17040_e29540: f64 = (assign17040_e29534 + assign17040_e29539);
        let assign17040_e29541: f64 = (assign17040_e29540).sqrt();
        let assign17040_e29542: f64 = (assign17040_e29527 + assign17040_e29541);
        let assign17040_e29543: f64 = (0.5 * assign17040_e29542);
        let assign17040_e29547: f64 = locals.var_t3;
        let assign17040_e29550: f64 = locals.var_t3;
        let assign17040_e29553: f64 = locals.var_t3;
        let assign17040_e29554: f64 = (assign17040_e29550 * assign17040_e29553);
        let assign17040_e29557: f64 = (0.25 * locals.var_sprt_i);
        let assign17040_e29559: f64 = (assign17040_e29557 * locals.var_sprt_i);
        let assign17040_e29560: f64 = (assign17040_e29554 + assign17040_e29559);
        let assign17040_e29561: f64 = (assign17040_e29560).sqrt();
        let assign17040_e29562: f64 = (assign17040_e29547 + assign17040_e29561);
        let assign17040_e29563: f64 = (0.5 * assign17040_e29562);
        let assign17040_e29564: f64 = (assign17040_e29543 - assign17040_e29563);
        (assign17040_e29564, (-(0.5 * (locals.var_t3_dn0 + (((locals.var_t3_dn0 * assign17040_e29553) + (assign17040_e29550 * locals.var_t3_dn0)) / (2.0 * assign17040_e29561))))), (-(0.5 * (locals.var_t3_dn2 + (((locals.var_t3_dn2 * assign17040_e29553) + (assign17040_e29550 * locals.var_t3_dn2)) / (2.0 * assign17040_e29561))))), (-(0.5 * (locals.var_t3_dn3 + (((locals.var_t3_dn3 * assign17040_e29553) + (assign17040_e29550 * locals.var_t3_dn3)) / (2.0 * assign17040_e29561))))), ((0.5 * ((locals.var_rdstemp0_dn4 + locals.var_rdstemp1_dn4) + ((((locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4) * assign17040_e29533) + (assign17040_e29530 * (locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4))) / (2.0 * assign17040_e29541)))) - (0.5 * (locals.var_t3_dn4 + (((locals.var_t3_dn4 * assign17040_e29553) + (assign17040_e29550 * locals.var_t3_dn4)) / (2.0 * assign17040_e29561))))), (-(0.5 * (locals.var_t3_dn5 + (((locals.var_t3_dn5 * assign17040_e29553) + (assign17040_e29550 * locals.var_t3_dn5)) / (2.0 * assign17040_e29561))))), (-(0.5 * (locals.var_t3_dn6 + (((locals.var_t3_dn6 * assign17040_e29553) + (assign17040_e29550 * locals.var_t3_dn6)) / (2.0 * assign17040_e29561))))), (-(0.5 * (locals.var_t3_dn7 + (((locals.var_t3_dn7 * assign17040_e29553) + (assign17040_e29550 * locals.var_t3_dn7)) / (2.0 * assign17040_e29561))))), (-(0.5 * (locals.var_t3_dn8 + (((locals.var_t3_dn8 * assign17040_e29553) + (assign17040_e29550 * locals.var_t3_dn8)) / (2.0 * assign17040_e29561))))), (-(0.5 * (locals.var_t3_dn9 + (((locals.var_t3_dn9 * assign17040_e29553) + (assign17040_e29550 * locals.var_t3_dn9)) / (2.0 * assign17040_e29561))))), (-(0.5 * (locals.var_t3_dn10 + (((locals.var_t3_dn10 * assign17040_e29553) + (assign17040_e29550 * locals.var_t3_dn10)) / (2.0 * assign17040_e29561))))), (-(0.5 * (locals.var_t3_dn11 + (((locals.var_t3_dn11 * assign17040_e29553) + (assign17040_e29550 * locals.var_t3_dn11)) / (2.0 * assign17040_e29561))))), (-(0.5 * (locals.var_t3_dn13 + (((locals.var_t3_dn13 * assign17040_e29553) + (assign17040_e29550 * locals.var_t3_dn13)) / (2.0 * assign17040_e29561))))), (-(0.5 * (locals.var_t3_dn14 + (((locals.var_t3_dn14 * assign17040_e29553) + (assign17040_e29550 * locals.var_t3_dn14)) / (2.0 * assign17040_e29561))))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign17040_e29566;
        locals.var_t6_dn0 = assign17040_e29566_d_n0;
        locals.var_t6_dn2 = assign17040_e29566_d_n2;
        locals.var_t6_dn3 = assign17040_e29566_d_n3;
        locals.var_t6_dn4 = assign17040_e29566_d_n4;
        locals.var_t6_dn5 = assign17040_e29566_d_n5;
        locals.var_t6_dn6 = assign17040_e29566_d_n6;
        locals.var_t6_dn7 = assign17040_e29566_d_n7;
        locals.var_t6_dn8 = assign17040_e29566_d_n8;
        locals.var_t6_dn9 = assign17040_e29566_d_n9;
        locals.var_t6_dn10 = assign17040_e29566_d_n10;
        locals.var_t6_dn11 = assign17040_e29566_d_n11;
        locals.var_t6_dn13 = assign17040_e29566_d_n13;
        locals.var_t6_dn14 = assign17040_e29566_d_n14;

        let (assign17050_e29626, assign17050_e29626_d_n0, assign17050_e29626_d_n2, assign17050_e29626_d_n3, assign17050_e29626_d_n4, assign17050_e29626_d_n5, assign17050_e29626_d_n6, assign17050_e29626_d_n7, assign17050_e29626_d_n8, assign17050_e29626_d_n9, assign17050_e29626_d_n10, assign17050_e29626_d_n11, assign17050_e29626_d_n13, assign17050_e29626_d_n14,) = {
    if (((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 != 0.0)) && (locals.var_guard305 != 0.0)) {
        let assign17050_e29587: f64 = (locals.var_t4 + locals.var_t5);
        let assign17050_e29590: f64 = (locals.var_t4 - locals.var_t5);
        let assign17050_e29593: f64 = (locals.var_t4 - locals.var_t5);
        let assign17050_e29594: f64 = (assign17050_e29590 * assign17050_e29593);
        let assign17050_e29597: f64 = (0.25 * locals.var_sprt_i);
        let assign17050_e29599: f64 = (assign17050_e29597 * locals.var_sprt_i);
        let assign17050_e29600: f64 = (assign17050_e29594 + assign17050_e29599);
        let assign17050_e29601: f64 = (assign17050_e29600).sqrt();
        let assign17050_e29602: f64 = (assign17050_e29587 + assign17050_e29601);
        let assign17050_e29603: f64 = (0.5 * assign17050_e29602);
        let assign17050_e29607: f64 = locals.var_t3;
        let assign17050_e29610: f64 = locals.var_t3;
        let assign17050_e29613: f64 = locals.var_t3;
        let assign17050_e29614: f64 = (assign17050_e29610 * assign17050_e29613);
        let assign17050_e29617: f64 = (0.25 * locals.var_sprt_i);
        let assign17050_e29619: f64 = (assign17050_e29617 * locals.var_sprt_i);
        let assign17050_e29620: f64 = (assign17050_e29614 + assign17050_e29619);
        let assign17050_e29621: f64 = (assign17050_e29620).sqrt();
        let assign17050_e29622: f64 = (assign17050_e29607 + assign17050_e29621);
        let assign17050_e29623: f64 = (0.5 * assign17050_e29622);
        let assign17050_e29624: f64 = (assign17050_e29603 - assign17050_e29623);
        (assign17050_e29624, ((0.5 * ((locals.var_t4_dn0 + locals.var_t5_dn0) + ((((locals.var_t4_dn0 - locals.var_t5_dn0) * assign17050_e29593) + (assign17050_e29590 * (locals.var_t4_dn0 - locals.var_t5_dn0))) / (2.0 * assign17050_e29601)))) - (0.5 * (locals.var_t3_dn0 + (((locals.var_t3_dn0 * assign17050_e29613) + (assign17050_e29610 * locals.var_t3_dn0)) / (2.0 * assign17050_e29621))))), ((0.5 * ((locals.var_t4_dn2 + locals.var_t5_dn2) + ((((locals.var_t4_dn2 - locals.var_t5_dn2) * assign17050_e29593) + (assign17050_e29590 * (locals.var_t4_dn2 - locals.var_t5_dn2))) / (2.0 * assign17050_e29601)))) - (0.5 * (locals.var_t3_dn2 + (((locals.var_t3_dn2 * assign17050_e29613) + (assign17050_e29610 * locals.var_t3_dn2)) / (2.0 * assign17050_e29621))))), ((0.5 * ((locals.var_t4_dn3 + locals.var_t5_dn3) + ((((locals.var_t4_dn3 - locals.var_t5_dn3) * assign17050_e29593) + (assign17050_e29590 * (locals.var_t4_dn3 - locals.var_t5_dn3))) / (2.0 * assign17050_e29601)))) - (0.5 * (locals.var_t3_dn3 + (((locals.var_t3_dn3 * assign17050_e29613) + (assign17050_e29610 * locals.var_t3_dn3)) / (2.0 * assign17050_e29621))))), ((0.5 * ((locals.var_t4_dn4 + locals.var_t5_dn4) + ((((locals.var_t4_dn4 - locals.var_t5_dn4) * assign17050_e29593) + (assign17050_e29590 * (locals.var_t4_dn4 - locals.var_t5_dn4))) / (2.0 * assign17050_e29601)))) - (0.5 * (locals.var_t3_dn4 + (((locals.var_t3_dn4 * assign17050_e29613) + (assign17050_e29610 * locals.var_t3_dn4)) / (2.0 * assign17050_e29621))))), ((0.5 * ((locals.var_t4_dn5 + locals.var_t5_dn5) + ((((locals.var_t4_dn5 - locals.var_t5_dn5) * assign17050_e29593) + (assign17050_e29590 * (locals.var_t4_dn5 - locals.var_t5_dn5))) / (2.0 * assign17050_e29601)))) - (0.5 * (locals.var_t3_dn5 + (((locals.var_t3_dn5 * assign17050_e29613) + (assign17050_e29610 * locals.var_t3_dn5)) / (2.0 * assign17050_e29621))))), ((0.5 * ((locals.var_t4_dn6 + locals.var_t5_dn6) + ((((locals.var_t4_dn6 - locals.var_t5_dn6) * assign17050_e29593) + (assign17050_e29590 * (locals.var_t4_dn6 - locals.var_t5_dn6))) / (2.0 * assign17050_e29601)))) - (0.5 * (locals.var_t3_dn6 + (((locals.var_t3_dn6 * assign17050_e29613) + (assign17050_e29610 * locals.var_t3_dn6)) / (2.0 * assign17050_e29621))))), ((0.5 * ((locals.var_t4_dn7 + locals.var_t5_dn7) + ((((locals.var_t4_dn7 - locals.var_t5_dn7) * assign17050_e29593) + (assign17050_e29590 * (locals.var_t4_dn7 - locals.var_t5_dn7))) / (2.0 * assign17050_e29601)))) - (0.5 * (locals.var_t3_dn7 + (((locals.var_t3_dn7 * assign17050_e29613) + (assign17050_e29610 * locals.var_t3_dn7)) / (2.0 * assign17050_e29621))))), ((0.5 * ((locals.var_t4_dn8 + locals.var_t5_dn8) + ((((locals.var_t4_dn8 - locals.var_t5_dn8) * assign17050_e29593) + (assign17050_e29590 * (locals.var_t4_dn8 - locals.var_t5_dn8))) / (2.0 * assign17050_e29601)))) - (0.5 * (locals.var_t3_dn8 + (((locals.var_t3_dn8 * assign17050_e29613) + (assign17050_e29610 * locals.var_t3_dn8)) / (2.0 * assign17050_e29621))))), ((0.5 * ((locals.var_t4_dn9 + locals.var_t5_dn9) + ((((locals.var_t4_dn9 - locals.var_t5_dn9) * assign17050_e29593) + (assign17050_e29590 * (locals.var_t4_dn9 - locals.var_t5_dn9))) / (2.0 * assign17050_e29601)))) - (0.5 * (locals.var_t3_dn9 + (((locals.var_t3_dn9 * assign17050_e29613) + (assign17050_e29610 * locals.var_t3_dn9)) / (2.0 * assign17050_e29621))))), ((0.5 * ((locals.var_t4_dn10 + locals.var_t5_dn10) + ((((locals.var_t4_dn10 - locals.var_t5_dn10) * assign17050_e29593) + (assign17050_e29590 * (locals.var_t4_dn10 - locals.var_t5_dn10))) / (2.0 * assign17050_e29601)))) - (0.5 * (locals.var_t3_dn10 + (((locals.var_t3_dn10 * assign17050_e29613) + (assign17050_e29610 * locals.var_t3_dn10)) / (2.0 * assign17050_e29621))))), ((0.5 * ((locals.var_t4_dn11 + locals.var_t5_dn11) + ((((locals.var_t4_dn11 - locals.var_t5_dn11) * assign17050_e29593) + (assign17050_e29590 * (locals.var_t4_dn11 - locals.var_t5_dn11))) / (2.0 * assign17050_e29601)))) - (0.5 * (locals.var_t3_dn11 + (((locals.var_t3_dn11 * assign17050_e29613) + (assign17050_e29610 * locals.var_t3_dn11)) / (2.0 * assign17050_e29621))))), ((0.5 * ((locals.var_t4_dn13 + locals.var_t5_dn13) + ((((locals.var_t4_dn13 - locals.var_t5_dn13) * assign17050_e29593) + (assign17050_e29590 * (locals.var_t4_dn13 - locals.var_t5_dn13))) / (2.0 * assign17050_e29601)))) - (0.5 * (locals.var_t3_dn13 + (((locals.var_t3_dn13 * assign17050_e29613) + (assign17050_e29610 * locals.var_t3_dn13)) / (2.0 * assign17050_e29621))))), ((0.5 * ((locals.var_t4_dn14 + locals.var_t5_dn14) + ((((locals.var_t4_dn14 - locals.var_t5_dn14) * assign17050_e29593) + (assign17050_e29590 * (locals.var_t4_dn14 - locals.var_t5_dn14))) / (2.0 * assign17050_e29601)))) - (0.5 * (locals.var_t3_dn14 + (((locals.var_t3_dn14 * assign17050_e29613) + (assign17050_e29610 * locals.var_t3_dn14)) / (2.0 * assign17050_e29621))))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign17050_e29626;
        locals.var_t7_dn0 = assign17050_e29626_d_n0;
        locals.var_t7_dn2 = assign17050_e29626_d_n2;
        locals.var_t7_dn3 = assign17050_e29626_d_n3;
        locals.var_t7_dn4 = assign17050_e29626_d_n4;
        locals.var_t7_dn5 = assign17050_e29626_d_n5;
        locals.var_t7_dn6 = assign17050_e29626_d_n6;
        locals.var_t7_dn7 = assign17050_e29626_d_n7;
        locals.var_t7_dn8 = assign17050_e29626_d_n8;
        locals.var_t7_dn9 = assign17050_e29626_d_n9;
        locals.var_t7_dn10 = assign17050_e29626_d_n10;
        locals.var_t7_dn11 = assign17050_e29626_d_n11;
        locals.var_t7_dn13 = assign17050_e29626_d_n13;
        locals.var_t7_dn14 = assign17050_e29626_d_n14;

        let (assign17060_e29652, assign17060_e29652_d_n0, assign17060_e29652_d_n2, assign17060_e29652_d_n3, assign17060_e29652_d_n4, assign17060_e29652_d_n5, assign17060_e29652_d_n6, assign17060_e29652_d_n7, assign17060_e29652_d_n8, assign17060_e29652_d_n9, assign17060_e29652_d_n10, assign17060_e29652_d_n11, assign17060_e29652_d_n13, assign17060_e29652_d_n14,) = {
    if (((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 != 0.0)) && (locals.var_guard305 != 0.0)) {
        let assign17060_e29648: f64 = (locals.var_devtemp - 210.0);
        let assign17060_e29649: f64 = (locals.var_prt_i * assign17060_e29648);
        let assign17060_e29650: f64 = (locals.var_t7 + assign17060_e29649);
        (assign17060_e29650, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, (locals.var_t7_dn4 + (locals.var_prt_i * locals.var_devtemp_dn4)), locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn13, locals.var_t7_dn14,)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign17060_e29652;
        locals.var_t8_dn0 = assign17060_e29652_d_n0;
        locals.var_t8_dn2 = assign17060_e29652_d_n2;
        locals.var_t8_dn3 = assign17060_e29652_d_n3;
        locals.var_t8_dn4 = assign17060_e29652_d_n4;
        locals.var_t8_dn5 = assign17060_e29652_d_n5;
        locals.var_t8_dn6 = assign17060_e29652_d_n6;
        locals.var_t8_dn7 = assign17060_e29652_d_n7;
        locals.var_t8_dn8 = assign17060_e29652_d_n8;
        locals.var_t8_dn9 = assign17060_e29652_d_n9;
        locals.var_t8_dn10 = assign17060_e29652_d_n10;
        locals.var_t8_dn11 = assign17060_e29652_d_n11;
        locals.var_t8_dn13 = assign17060_e29652_d_n13;
        locals.var_t8_dn14 = assign17060_e29652_d_n14;

        let (assign17070_e29691, assign17070_e29691_d_n0, assign17070_e29691_d_n2, assign17070_e29691_d_n3, assign17070_e29691_d_n4, assign17070_e29691_d_n5, assign17070_e29691_d_n6, assign17070_e29691_d_n7, assign17070_e29691_d_n8, assign17070_e29691_d_n9, assign17070_e29691_d_n10, assign17070_e29691_d_n11, assign17070_e29691_d_n13, assign17070_e29691_d_n14,) = {
    if (((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 != 0.0)) && (locals.var_guard305 != 0.0)) {
        let assign17070_e29673: f64 = (locals.var_t6 + locals.var_t8);
        let assign17070_e29676: f64 = (locals.var_t6 - locals.var_t8);
        let assign17070_e29679: f64 = (locals.var_t6 - locals.var_t8);
        let assign17070_e29680: f64 = (assign17070_e29676 * assign17070_e29679);
        let assign17070_e29683: f64 = (0.25 * 0.001);
        let assign17070_e29685: f64 = (assign17070_e29683 * 0.001);
        let assign17070_e29686: f64 = (assign17070_e29680 + assign17070_e29685);
        let assign17070_e29687: f64 = (assign17070_e29686).sqrt();
        let assign17070_e29688: f64 = (assign17070_e29673 + assign17070_e29687);
        let assign17070_e29689: f64 = (0.5 * assign17070_e29688);
        (assign17070_e29689, (0.5 * ((locals.var_t6_dn0 + locals.var_t8_dn0) + ((((locals.var_t6_dn0 - locals.var_t8_dn0) * assign17070_e29679) + (assign17070_e29676 * (locals.var_t6_dn0 - locals.var_t8_dn0))) / (2.0 * assign17070_e29687)))), (0.5 * ((locals.var_t6_dn2 + locals.var_t8_dn2) + ((((locals.var_t6_dn2 - locals.var_t8_dn2) * assign17070_e29679) + (assign17070_e29676 * (locals.var_t6_dn2 - locals.var_t8_dn2))) / (2.0 * assign17070_e29687)))), (0.5 * ((locals.var_t6_dn3 + locals.var_t8_dn3) + ((((locals.var_t6_dn3 - locals.var_t8_dn3) * assign17070_e29679) + (assign17070_e29676 * (locals.var_t6_dn3 - locals.var_t8_dn3))) / (2.0 * assign17070_e29687)))), (0.5 * ((locals.var_t6_dn4 + locals.var_t8_dn4) + ((((locals.var_t6_dn4 - locals.var_t8_dn4) * assign17070_e29679) + (assign17070_e29676 * (locals.var_t6_dn4 - locals.var_t8_dn4))) / (2.0 * assign17070_e29687)))), (0.5 * ((locals.var_t6_dn5 + locals.var_t8_dn5) + ((((locals.var_t6_dn5 - locals.var_t8_dn5) * assign17070_e29679) + (assign17070_e29676 * (locals.var_t6_dn5 - locals.var_t8_dn5))) / (2.0 * assign17070_e29687)))), (0.5 * ((locals.var_t6_dn6 + locals.var_t8_dn6) + ((((locals.var_t6_dn6 - locals.var_t8_dn6) * assign17070_e29679) + (assign17070_e29676 * (locals.var_t6_dn6 - locals.var_t8_dn6))) / (2.0 * assign17070_e29687)))), (0.5 * ((locals.var_t6_dn7 + locals.var_t8_dn7) + ((((locals.var_t6_dn7 - locals.var_t8_dn7) * assign17070_e29679) + (assign17070_e29676 * (locals.var_t6_dn7 - locals.var_t8_dn7))) / (2.0 * assign17070_e29687)))), (0.5 * ((locals.var_t6_dn8 + locals.var_t8_dn8) + ((((locals.var_t6_dn8 - locals.var_t8_dn8) * assign17070_e29679) + (assign17070_e29676 * (locals.var_t6_dn8 - locals.var_t8_dn8))) / (2.0 * assign17070_e29687)))), (0.5 * ((locals.var_t6_dn9 + locals.var_t8_dn9) + ((((locals.var_t6_dn9 - locals.var_t8_dn9) * assign17070_e29679) + (assign17070_e29676 * (locals.var_t6_dn9 - locals.var_t8_dn9))) / (2.0 * assign17070_e29687)))), (0.5 * ((locals.var_t6_dn10 + locals.var_t8_dn10) + ((((locals.var_t6_dn10 - locals.var_t8_dn10) * assign17070_e29679) + (assign17070_e29676 * (locals.var_t6_dn10 - locals.var_t8_dn10))) / (2.0 * assign17070_e29687)))), (0.5 * ((locals.var_t6_dn11 + locals.var_t8_dn11) + ((((locals.var_t6_dn11 - locals.var_t8_dn11) * assign17070_e29679) + (assign17070_e29676 * (locals.var_t6_dn11 - locals.var_t8_dn11))) / (2.0 * assign17070_e29687)))), (0.5 * ((locals.var_t6_dn13 + locals.var_t8_dn13) + ((((locals.var_t6_dn13 - locals.var_t8_dn13) * assign17070_e29679) + (assign17070_e29676 * (locals.var_t6_dn13 - locals.var_t8_dn13))) / (2.0 * assign17070_e29687)))), (0.5 * ((locals.var_t6_dn14 + locals.var_t8_dn14) + ((((locals.var_t6_dn14 - locals.var_t8_dn14) * assign17070_e29679) + (assign17070_e29676 * (locals.var_t6_dn14 - locals.var_t8_dn14))) / (2.0 * assign17070_e29687)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign17070_e29691;
        locals.var_t2_dn0 = assign17070_e29691_d_n0;
        locals.var_t2_dn2 = assign17070_e29691_d_n2;
        locals.var_t2_dn3 = assign17070_e29691_d_n3;
        locals.var_t2_dn4 = assign17070_e29691_d_n4;
        locals.var_t2_dn5 = assign17070_e29691_d_n5;
        locals.var_t2_dn6 = assign17070_e29691_d_n6;
        locals.var_t2_dn7 = assign17070_e29691_d_n7;
        locals.var_t2_dn8 = assign17070_e29691_d_n8;
        locals.var_t2_dn9 = assign17070_e29691_d_n9;
        locals.var_t2_dn10 = assign17070_e29691_d_n10;
        locals.var_t2_dn11 = assign17070_e29691_d_n11;
        locals.var_t2_dn13 = assign17070_e29691_d_n13;
        locals.var_t2_dn14 = assign17070_e29691_d_n14;

        let (assign17080_e29752, assign17080_e29752_d_n0, assign17080_e29752_d_n2, assign17080_e29752_d_n3, assign17080_e29752_d_n4, assign17080_e29752_d_n5, assign17080_e29752_d_n6, assign17080_e29752_d_n7, assign17080_e29752_d_n8, assign17080_e29752_d_n9, assign17080_e29752_d_n10, assign17080_e29752_d_n11, assign17080_e29752_d_n13, assign17080_e29752_d_n14,) = {
    if (((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 != 0.0)) && (locals.var_guard305 == 0.0)) {
        let assign17080_e29713: f64 = (locals.var_rdstemp0 + locals.var_rdstemp1);
        let assign17080_e29716: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign17080_e29719: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign17080_e29720: f64 = (assign17080_e29716 * assign17080_e29719);
        let assign17080_e29723: f64 = (0.25 * locals.var_sprt_i);
        let assign17080_e29725: f64 = (assign17080_e29723 * locals.var_sprt_i);
        let assign17080_e29726: f64 = (assign17080_e29720 + assign17080_e29725);
        let assign17080_e29727: f64 = (assign17080_e29726).sqrt();
        let assign17080_e29728: f64 = (assign17080_e29713 - assign17080_e29727);
        let assign17080_e29729: f64 = (0.5 * assign17080_e29728);
        let assign17080_e29733: f64 = locals.var_t3;
        let assign17080_e29736: f64 = locals.var_t3;
        let assign17080_e29739: f64 = locals.var_t3;
        let assign17080_e29740: f64 = (assign17080_e29736 * assign17080_e29739);
        let assign17080_e29743: f64 = (0.25 * locals.var_sprt_i);
        let assign17080_e29745: f64 = (assign17080_e29743 * locals.var_sprt_i);
        let assign17080_e29746: f64 = (assign17080_e29740 + assign17080_e29745);
        let assign17080_e29747: f64 = (assign17080_e29746).sqrt();
        let assign17080_e29748: f64 = (assign17080_e29733 - assign17080_e29747);
        let assign17080_e29749: f64 = (0.5 * assign17080_e29748);
        let assign17080_e29750: f64 = (assign17080_e29729 - assign17080_e29749);
        (assign17080_e29750, (-(0.5 * (locals.var_t3_dn0 - (((locals.var_t3_dn0 * assign17080_e29739) + (assign17080_e29736 * locals.var_t3_dn0)) / (2.0 * assign17080_e29747))))), (-(0.5 * (locals.var_t3_dn2 - (((locals.var_t3_dn2 * assign17080_e29739) + (assign17080_e29736 * locals.var_t3_dn2)) / (2.0 * assign17080_e29747))))), (-(0.5 * (locals.var_t3_dn3 - (((locals.var_t3_dn3 * assign17080_e29739) + (assign17080_e29736 * locals.var_t3_dn3)) / (2.0 * assign17080_e29747))))), ((0.5 * ((locals.var_rdstemp0_dn4 + locals.var_rdstemp1_dn4) - ((((locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4) * assign17080_e29719) + (assign17080_e29716 * (locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4))) / (2.0 * assign17080_e29727)))) - (0.5 * (locals.var_t3_dn4 - (((locals.var_t3_dn4 * assign17080_e29739) + (assign17080_e29736 * locals.var_t3_dn4)) / (2.0 * assign17080_e29747))))), (-(0.5 * (locals.var_t3_dn5 - (((locals.var_t3_dn5 * assign17080_e29739) + (assign17080_e29736 * locals.var_t3_dn5)) / (2.0 * assign17080_e29747))))), (-(0.5 * (locals.var_t3_dn6 - (((locals.var_t3_dn6 * assign17080_e29739) + (assign17080_e29736 * locals.var_t3_dn6)) / (2.0 * assign17080_e29747))))), (-(0.5 * (locals.var_t3_dn7 - (((locals.var_t3_dn7 * assign17080_e29739) + (assign17080_e29736 * locals.var_t3_dn7)) / (2.0 * assign17080_e29747))))), (-(0.5 * (locals.var_t3_dn8 - (((locals.var_t3_dn8 * assign17080_e29739) + (assign17080_e29736 * locals.var_t3_dn8)) / (2.0 * assign17080_e29747))))), (-(0.5 * (locals.var_t3_dn9 - (((locals.var_t3_dn9 * assign17080_e29739) + (assign17080_e29736 * locals.var_t3_dn9)) / (2.0 * assign17080_e29747))))), (-(0.5 * (locals.var_t3_dn10 - (((locals.var_t3_dn10 * assign17080_e29739) + (assign17080_e29736 * locals.var_t3_dn10)) / (2.0 * assign17080_e29747))))), (-(0.5 * (locals.var_t3_dn11 - (((locals.var_t3_dn11 * assign17080_e29739) + (assign17080_e29736 * locals.var_t3_dn11)) / (2.0 * assign17080_e29747))))), (-(0.5 * (locals.var_t3_dn13 - (((locals.var_t3_dn13 * assign17080_e29739) + (assign17080_e29736 * locals.var_t3_dn13)) / (2.0 * assign17080_e29747))))), (-(0.5 * (locals.var_t3_dn14 - (((locals.var_t3_dn14 * assign17080_e29739) + (assign17080_e29736 * locals.var_t3_dn14)) / (2.0 * assign17080_e29747))))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign17080_e29752;
        locals.var_t6_dn0 = assign17080_e29752_d_n0;
        locals.var_t6_dn2 = assign17080_e29752_d_n2;
        locals.var_t6_dn3 = assign17080_e29752_d_n3;
        locals.var_t6_dn4 = assign17080_e29752_d_n4;
        locals.var_t6_dn5 = assign17080_e29752_d_n5;
        locals.var_t6_dn6 = assign17080_e29752_d_n6;
        locals.var_t6_dn7 = assign17080_e29752_d_n7;
        locals.var_t6_dn8 = assign17080_e29752_d_n8;
        locals.var_t6_dn9 = assign17080_e29752_d_n9;
        locals.var_t6_dn10 = assign17080_e29752_d_n10;
        locals.var_t6_dn11 = assign17080_e29752_d_n11;
        locals.var_t6_dn13 = assign17080_e29752_d_n13;
        locals.var_t6_dn14 = assign17080_e29752_d_n14;

        let (assign17090_e29813, assign17090_e29813_d_n0, assign17090_e29813_d_n2, assign17090_e29813_d_n3, assign17090_e29813_d_n4, assign17090_e29813_d_n5, assign17090_e29813_d_n6, assign17090_e29813_d_n7, assign17090_e29813_d_n8, assign17090_e29813_d_n9, assign17090_e29813_d_n10, assign17090_e29813_d_n11, assign17090_e29813_d_n13, assign17090_e29813_d_n14,) = {
    if (((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 != 0.0)) && (locals.var_guard305 == 0.0)) {
        let assign17090_e29774: f64 = (locals.var_t4 + locals.var_t5);
        let assign17090_e29777: f64 = (locals.var_t4 - locals.var_t5);
        let assign17090_e29780: f64 = (locals.var_t4 - locals.var_t5);
        let assign17090_e29781: f64 = (assign17090_e29777 * assign17090_e29780);
        let assign17090_e29784: f64 = (0.25 * locals.var_sprt_i);
        let assign17090_e29786: f64 = (assign17090_e29784 * locals.var_sprt_i);
        let assign17090_e29787: f64 = (assign17090_e29781 + assign17090_e29786);
        let assign17090_e29788: f64 = (assign17090_e29787).sqrt();
        let assign17090_e29789: f64 = (assign17090_e29774 - assign17090_e29788);
        let assign17090_e29790: f64 = (0.5 * assign17090_e29789);
        let assign17090_e29794: f64 = locals.var_t3;
        let assign17090_e29797: f64 = locals.var_t3;
        let assign17090_e29800: f64 = locals.var_t3;
        let assign17090_e29801: f64 = (assign17090_e29797 * assign17090_e29800);
        let assign17090_e29804: f64 = (0.25 * locals.var_sprt_i);
        let assign17090_e29806: f64 = (assign17090_e29804 * locals.var_sprt_i);
        let assign17090_e29807: f64 = (assign17090_e29801 + assign17090_e29806);
        let assign17090_e29808: f64 = (assign17090_e29807).sqrt();
        let assign17090_e29809: f64 = (assign17090_e29794 - assign17090_e29808);
        let assign17090_e29810: f64 = (0.5 * assign17090_e29809);
        let assign17090_e29811: f64 = (assign17090_e29790 - assign17090_e29810);
        (assign17090_e29811, ((0.5 * ((locals.var_t4_dn0 + locals.var_t5_dn0) - ((((locals.var_t4_dn0 - locals.var_t5_dn0) * assign17090_e29780) + (assign17090_e29777 * (locals.var_t4_dn0 - locals.var_t5_dn0))) / (2.0 * assign17090_e29788)))) - (0.5 * (locals.var_t3_dn0 - (((locals.var_t3_dn0 * assign17090_e29800) + (assign17090_e29797 * locals.var_t3_dn0)) / (2.0 * assign17090_e29808))))), ((0.5 * ((locals.var_t4_dn2 + locals.var_t5_dn2) - ((((locals.var_t4_dn2 - locals.var_t5_dn2) * assign17090_e29780) + (assign17090_e29777 * (locals.var_t4_dn2 - locals.var_t5_dn2))) / (2.0 * assign17090_e29788)))) - (0.5 * (locals.var_t3_dn2 - (((locals.var_t3_dn2 * assign17090_e29800) + (assign17090_e29797 * locals.var_t3_dn2)) / (2.0 * assign17090_e29808))))), ((0.5 * ((locals.var_t4_dn3 + locals.var_t5_dn3) - ((((locals.var_t4_dn3 - locals.var_t5_dn3) * assign17090_e29780) + (assign17090_e29777 * (locals.var_t4_dn3 - locals.var_t5_dn3))) / (2.0 * assign17090_e29788)))) - (0.5 * (locals.var_t3_dn3 - (((locals.var_t3_dn3 * assign17090_e29800) + (assign17090_e29797 * locals.var_t3_dn3)) / (2.0 * assign17090_e29808))))), ((0.5 * ((locals.var_t4_dn4 + locals.var_t5_dn4) - ((((locals.var_t4_dn4 - locals.var_t5_dn4) * assign17090_e29780) + (assign17090_e29777 * (locals.var_t4_dn4 - locals.var_t5_dn4))) / (2.0 * assign17090_e29788)))) - (0.5 * (locals.var_t3_dn4 - (((locals.var_t3_dn4 * assign17090_e29800) + (assign17090_e29797 * locals.var_t3_dn4)) / (2.0 * assign17090_e29808))))), ((0.5 * ((locals.var_t4_dn5 + locals.var_t5_dn5) - ((((locals.var_t4_dn5 - locals.var_t5_dn5) * assign17090_e29780) + (assign17090_e29777 * (locals.var_t4_dn5 - locals.var_t5_dn5))) / (2.0 * assign17090_e29788)))) - (0.5 * (locals.var_t3_dn5 - (((locals.var_t3_dn5 * assign17090_e29800) + (assign17090_e29797 * locals.var_t3_dn5)) / (2.0 * assign17090_e29808))))), ((0.5 * ((locals.var_t4_dn6 + locals.var_t5_dn6) - ((((locals.var_t4_dn6 - locals.var_t5_dn6) * assign17090_e29780) + (assign17090_e29777 * (locals.var_t4_dn6 - locals.var_t5_dn6))) / (2.0 * assign17090_e29788)))) - (0.5 * (locals.var_t3_dn6 - (((locals.var_t3_dn6 * assign17090_e29800) + (assign17090_e29797 * locals.var_t3_dn6)) / (2.0 * assign17090_e29808))))), ((0.5 * ((locals.var_t4_dn7 + locals.var_t5_dn7) - ((((locals.var_t4_dn7 - locals.var_t5_dn7) * assign17090_e29780) + (assign17090_e29777 * (locals.var_t4_dn7 - locals.var_t5_dn7))) / (2.0 * assign17090_e29788)))) - (0.5 * (locals.var_t3_dn7 - (((locals.var_t3_dn7 * assign17090_e29800) + (assign17090_e29797 * locals.var_t3_dn7)) / (2.0 * assign17090_e29808))))), ((0.5 * ((locals.var_t4_dn8 + locals.var_t5_dn8) - ((((locals.var_t4_dn8 - locals.var_t5_dn8) * assign17090_e29780) + (assign17090_e29777 * (locals.var_t4_dn8 - locals.var_t5_dn8))) / (2.0 * assign17090_e29788)))) - (0.5 * (locals.var_t3_dn8 - (((locals.var_t3_dn8 * assign17090_e29800) + (assign17090_e29797 * locals.var_t3_dn8)) / (2.0 * assign17090_e29808))))), ((0.5 * ((locals.var_t4_dn9 + locals.var_t5_dn9) - ((((locals.var_t4_dn9 - locals.var_t5_dn9) * assign17090_e29780) + (assign17090_e29777 * (locals.var_t4_dn9 - locals.var_t5_dn9))) / (2.0 * assign17090_e29788)))) - (0.5 * (locals.var_t3_dn9 - (((locals.var_t3_dn9 * assign17090_e29800) + (assign17090_e29797 * locals.var_t3_dn9)) / (2.0 * assign17090_e29808))))), ((0.5 * ((locals.var_t4_dn10 + locals.var_t5_dn10) - ((((locals.var_t4_dn10 - locals.var_t5_dn10) * assign17090_e29780) + (assign17090_e29777 * (locals.var_t4_dn10 - locals.var_t5_dn10))) / (2.0 * assign17090_e29788)))) - (0.5 * (locals.var_t3_dn10 - (((locals.var_t3_dn10 * assign17090_e29800) + (assign17090_e29797 * locals.var_t3_dn10)) / (2.0 * assign17090_e29808))))), ((0.5 * ((locals.var_t4_dn11 + locals.var_t5_dn11) - ((((locals.var_t4_dn11 - locals.var_t5_dn11) * assign17090_e29780) + (assign17090_e29777 * (locals.var_t4_dn11 - locals.var_t5_dn11))) / (2.0 * assign17090_e29788)))) - (0.5 * (locals.var_t3_dn11 - (((locals.var_t3_dn11 * assign17090_e29800) + (assign17090_e29797 * locals.var_t3_dn11)) / (2.0 * assign17090_e29808))))), ((0.5 * ((locals.var_t4_dn13 + locals.var_t5_dn13) - ((((locals.var_t4_dn13 - locals.var_t5_dn13) * assign17090_e29780) + (assign17090_e29777 * (locals.var_t4_dn13 - locals.var_t5_dn13))) / (2.0 * assign17090_e29788)))) - (0.5 * (locals.var_t3_dn13 - (((locals.var_t3_dn13 * assign17090_e29800) + (assign17090_e29797 * locals.var_t3_dn13)) / (2.0 * assign17090_e29808))))), ((0.5 * ((locals.var_t4_dn14 + locals.var_t5_dn14) - ((((locals.var_t4_dn14 - locals.var_t5_dn14) * assign17090_e29780) + (assign17090_e29777 * (locals.var_t4_dn14 - locals.var_t5_dn14))) / (2.0 * assign17090_e29788)))) - (0.5 * (locals.var_t3_dn14 - (((locals.var_t3_dn14 * assign17090_e29800) + (assign17090_e29797 * locals.var_t3_dn14)) / (2.0 * assign17090_e29808))))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign17090_e29813;
        locals.var_t7_dn0 = assign17090_e29813_d_n0;
        locals.var_t7_dn2 = assign17090_e29813_d_n2;
        locals.var_t7_dn3 = assign17090_e29813_d_n3;
        locals.var_t7_dn4 = assign17090_e29813_d_n4;
        locals.var_t7_dn5 = assign17090_e29813_d_n5;
        locals.var_t7_dn6 = assign17090_e29813_d_n6;
        locals.var_t7_dn7 = assign17090_e29813_d_n7;
        locals.var_t7_dn8 = assign17090_e29813_d_n8;
        locals.var_t7_dn9 = assign17090_e29813_d_n9;
        locals.var_t7_dn10 = assign17090_e29813_d_n10;
        locals.var_t7_dn11 = assign17090_e29813_d_n11;
        locals.var_t7_dn13 = assign17090_e29813_d_n13;
        locals.var_t7_dn14 = assign17090_e29813_d_n14;

        let (assign17100_e29840, assign17100_e29840_d_n0, assign17100_e29840_d_n2, assign17100_e29840_d_n3, assign17100_e29840_d_n4, assign17100_e29840_d_n5, assign17100_e29840_d_n6, assign17100_e29840_d_n7, assign17100_e29840_d_n8, assign17100_e29840_d_n9, assign17100_e29840_d_n10, assign17100_e29840_d_n11, assign17100_e29840_d_n13, assign17100_e29840_d_n14,) = {
    if (((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 != 0.0)) && (locals.var_guard305 == 0.0)) {
        let assign17100_e29836: f64 = (locals.var_devtemp - 210.0);
        let assign17100_e29837: f64 = (locals.var_prt_i * assign17100_e29836);
        let assign17100_e29838: f64 = (locals.var_t7 + assign17100_e29837);
        (assign17100_e29838, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, (locals.var_t7_dn4 + (locals.var_prt_i * locals.var_devtemp_dn4)), locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn13, locals.var_t7_dn14,)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign17100_e29840;
        locals.var_t8_dn0 = assign17100_e29840_d_n0;
        locals.var_t8_dn2 = assign17100_e29840_d_n2;
        locals.var_t8_dn3 = assign17100_e29840_d_n3;
        locals.var_t8_dn4 = assign17100_e29840_d_n4;
        locals.var_t8_dn5 = assign17100_e29840_d_n5;
        locals.var_t8_dn6 = assign17100_e29840_d_n6;
        locals.var_t8_dn7 = assign17100_e29840_d_n7;
        locals.var_t8_dn8 = assign17100_e29840_d_n8;
        locals.var_t8_dn9 = assign17100_e29840_d_n9;
        locals.var_t8_dn10 = assign17100_e29840_d_n10;
        locals.var_t8_dn11 = assign17100_e29840_d_n11;
        locals.var_t8_dn13 = assign17100_e29840_d_n13;
        locals.var_t8_dn14 = assign17100_e29840_d_n14;

        let (assign17110_e29880, assign17110_e29880_d_n0, assign17110_e29880_d_n2, assign17110_e29880_d_n3, assign17110_e29880_d_n4, assign17110_e29880_d_n5, assign17110_e29880_d_n6, assign17110_e29880_d_n7, assign17110_e29880_d_n8, assign17110_e29880_d_n9, assign17110_e29880_d_n10, assign17110_e29880_d_n11, assign17110_e29880_d_n13, assign17110_e29880_d_n14,) = {
    if (((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 != 0.0)) && (locals.var_guard305 == 0.0)) {
        let assign17110_e29862: f64 = (locals.var_t6 + locals.var_t8);
        let assign17110_e29865: f64 = (locals.var_t6 - locals.var_t8);
        let assign17110_e29868: f64 = (locals.var_t6 - locals.var_t8);
        let assign17110_e29869: f64 = (assign17110_e29865 * assign17110_e29868);
        let assign17110_e29872: f64 = (0.25 * 0.001);
        let assign17110_e29874: f64 = (assign17110_e29872 * 0.001);
        let assign17110_e29875: f64 = (assign17110_e29869 + assign17110_e29874);
        let assign17110_e29876: f64 = (assign17110_e29875).sqrt();
        let assign17110_e29877: f64 = (assign17110_e29862 - assign17110_e29876);
        let assign17110_e29878: f64 = (0.5 * assign17110_e29877);
        (assign17110_e29878, (0.5 * ((locals.var_t6_dn0 + locals.var_t8_dn0) - ((((locals.var_t6_dn0 - locals.var_t8_dn0) * assign17110_e29868) + (assign17110_e29865 * (locals.var_t6_dn0 - locals.var_t8_dn0))) / (2.0 * assign17110_e29876)))), (0.5 * ((locals.var_t6_dn2 + locals.var_t8_dn2) - ((((locals.var_t6_dn2 - locals.var_t8_dn2) * assign17110_e29868) + (assign17110_e29865 * (locals.var_t6_dn2 - locals.var_t8_dn2))) / (2.0 * assign17110_e29876)))), (0.5 * ((locals.var_t6_dn3 + locals.var_t8_dn3) - ((((locals.var_t6_dn3 - locals.var_t8_dn3) * assign17110_e29868) + (assign17110_e29865 * (locals.var_t6_dn3 - locals.var_t8_dn3))) / (2.0 * assign17110_e29876)))), (0.5 * ((locals.var_t6_dn4 + locals.var_t8_dn4) - ((((locals.var_t6_dn4 - locals.var_t8_dn4) * assign17110_e29868) + (assign17110_e29865 * (locals.var_t6_dn4 - locals.var_t8_dn4))) / (2.0 * assign17110_e29876)))), (0.5 * ((locals.var_t6_dn5 + locals.var_t8_dn5) - ((((locals.var_t6_dn5 - locals.var_t8_dn5) * assign17110_e29868) + (assign17110_e29865 * (locals.var_t6_dn5 - locals.var_t8_dn5))) / (2.0 * assign17110_e29876)))), (0.5 * ((locals.var_t6_dn6 + locals.var_t8_dn6) - ((((locals.var_t6_dn6 - locals.var_t8_dn6) * assign17110_e29868) + (assign17110_e29865 * (locals.var_t6_dn6 - locals.var_t8_dn6))) / (2.0 * assign17110_e29876)))), (0.5 * ((locals.var_t6_dn7 + locals.var_t8_dn7) - ((((locals.var_t6_dn7 - locals.var_t8_dn7) * assign17110_e29868) + (assign17110_e29865 * (locals.var_t6_dn7 - locals.var_t8_dn7))) / (2.0 * assign17110_e29876)))), (0.5 * ((locals.var_t6_dn8 + locals.var_t8_dn8) - ((((locals.var_t6_dn8 - locals.var_t8_dn8) * assign17110_e29868) + (assign17110_e29865 * (locals.var_t6_dn8 - locals.var_t8_dn8))) / (2.0 * assign17110_e29876)))), (0.5 * ((locals.var_t6_dn9 + locals.var_t8_dn9) - ((((locals.var_t6_dn9 - locals.var_t8_dn9) * assign17110_e29868) + (assign17110_e29865 * (locals.var_t6_dn9 - locals.var_t8_dn9))) / (2.0 * assign17110_e29876)))), (0.5 * ((locals.var_t6_dn10 + locals.var_t8_dn10) - ((((locals.var_t6_dn10 - locals.var_t8_dn10) * assign17110_e29868) + (assign17110_e29865 * (locals.var_t6_dn10 - locals.var_t8_dn10))) / (2.0 * assign17110_e29876)))), (0.5 * ((locals.var_t6_dn11 + locals.var_t8_dn11) - ((((locals.var_t6_dn11 - locals.var_t8_dn11) * assign17110_e29868) + (assign17110_e29865 * (locals.var_t6_dn11 - locals.var_t8_dn11))) / (2.0 * assign17110_e29876)))), (0.5 * ((locals.var_t6_dn13 + locals.var_t8_dn13) - ((((locals.var_t6_dn13 - locals.var_t8_dn13) * assign17110_e29868) + (assign17110_e29865 * (locals.var_t6_dn13 - locals.var_t8_dn13))) / (2.0 * assign17110_e29876)))), (0.5 * ((locals.var_t6_dn14 + locals.var_t8_dn14) - ((((locals.var_t6_dn14 - locals.var_t8_dn14) * assign17110_e29868) + (assign17110_e29865 * (locals.var_t6_dn14 - locals.var_t8_dn14))) / (2.0 * assign17110_e29876)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign17110_e29880;
        locals.var_t2_dn0 = assign17110_e29880_d_n0;
        locals.var_t2_dn2 = assign17110_e29880_d_n2;
        locals.var_t2_dn3 = assign17110_e29880_d_n3;
        locals.var_t2_dn4 = assign17110_e29880_d_n4;
        locals.var_t2_dn5 = assign17110_e29880_d_n5;
        locals.var_t2_dn6 = assign17110_e29880_d_n6;
        locals.var_t2_dn7 = assign17110_e29880_d_n7;
        locals.var_t2_dn8 = assign17110_e29880_d_n8;
        locals.var_t2_dn9 = assign17110_e29880_d_n9;
        locals.var_t2_dn10 = assign17110_e29880_d_n10;
        locals.var_t2_dn11 = assign17110_e29880_d_n11;
        locals.var_t2_dn13 = assign17110_e29880_d_n13;
        locals.var_t2_dn14 = assign17110_e29880_d_n14;

        let (assign17120_e29903, assign17120_e29903_d_n4,) = {
    if ((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 == 0.0)) {
        let assign17120_e29900: f64 = (locals.var_prt1_i * locals.var_deltemp);
        let assign17120_e29901: f64 = (1.0 + assign17120_e29900);
        (assign17120_e29901, (locals.var_prt1_i * locals.var_deltemp_dn4),)
    } else {
        (locals.var_rdstemp1, locals.var_rdstemp1_dn4,)
    }
};
        locals.var_rdstemp1 = assign17120_e29903;
        locals.var_rdstemp1_dn4 = assign17120_e29903_d_n4;

        let (assign17130_e29934, assign17130_e29934_d_n4,) = {
    if ((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 == 0.0)) {
        let assign17130_e29924: f64 = (locals.var_devtemp - locals.var_tr0_i);
        let assign17130_e29925: f64 = (locals.var_prt_i * assign17130_e29924);
        let assign17130_e29926: f64 = (1.0 + assign17130_e29925);
        let assign17130_e29930: f64 = (locals.var_tr0_i - locals.var_tnom);
        let assign17130_e29931: f64 = (locals.var_prt1_i * assign17130_e29930);
        let assign17130_e29932: f64 = (assign17130_e29926 + assign17130_e29931);
        (assign17130_e29932, (locals.var_prt_i * locals.var_devtemp_dn4),)
    } else {
        (locals.var_rdstemp0, locals.var_rdstemp0_dn4,)
    }
};
        locals.var_rdstemp0 = assign17130_e29934;
        locals.var_rdstemp0_dn4 = assign17130_e29934_d_n4;

        let (assign17140_e29959, assign17140_e29959_d_n0, assign17140_e29959_d_n2, assign17140_e29959_d_n3, assign17140_e29959_d_n4, assign17140_e29959_d_n5, assign17140_e29959_d_n6, assign17140_e29959_d_n7, assign17140_e29959_d_n8, assign17140_e29959_d_n9, assign17140_e29959_d_n10, assign17140_e29959_d_n11, assign17140_e29959_d_n13, assign17140_e29959_d_n14,) = {
    if ((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 == 0.0)) {
        let assign17140_e29953: f64 = (locals.var_prt1_i - locals.var_prt_i);
        let assign17140_e29956: f64 = (locals.var_tr0_i - locals.var_tnom);
        let assign17140_e29957: f64 = (assign17140_e29953 * assign17140_e29956);
        (assign17140_e29957, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign17140_e29959;
        locals.var_t3_dn0 = assign17140_e29959_d_n0;
        locals.var_t3_dn2 = assign17140_e29959_d_n2;
        locals.var_t3_dn3 = assign17140_e29959_d_n3;
        locals.var_t3_dn4 = assign17140_e29959_d_n4;
        locals.var_t3_dn5 = assign17140_e29959_d_n5;
        locals.var_t3_dn6 = assign17140_e29959_d_n6;
        locals.var_t3_dn7 = assign17140_e29959_d_n7;
        locals.var_t3_dn8 = assign17140_e29959_d_n8;
        locals.var_t3_dn9 = assign17140_e29959_d_n9;
        locals.var_t3_dn10 = assign17140_e29959_d_n10;
        locals.var_t3_dn11 = assign17140_e29959_d_n11;
        locals.var_t3_dn13 = assign17140_e29959_d_n13;
        locals.var_t3_dn14 = assign17140_e29959_d_n14;

        let (assign17150_e29984, assign17150_e29984_d_n0, assign17150_e29984_d_n2, assign17150_e29984_d_n3, assign17150_e29984_d_n4, assign17150_e29984_d_n5, assign17150_e29984_d_n6, assign17150_e29984_d_n7, assign17150_e29984_d_n8, assign17150_e29984_d_n9, assign17150_e29984_d_n10, assign17150_e29984_d_n11, assign17150_e29984_d_n13, assign17150_e29984_d_n14,) = {
    if ((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 == 0.0)) {
        let assign17150_e29980: f64 = (210.0 - locals.var_tnom);
        let assign17150_e29981: f64 = (locals.var_prt1_i * assign17150_e29980);
        let assign17150_e29982: f64 = (1.0 + assign17150_e29981);
        (assign17150_e29982, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign17150_e29984;
        locals.var_t4_dn0 = assign17150_e29984_d_n0;
        locals.var_t4_dn2 = assign17150_e29984_d_n2;
        locals.var_t4_dn3 = assign17150_e29984_d_n3;
        locals.var_t4_dn4 = assign17150_e29984_d_n4;
        locals.var_t4_dn5 = assign17150_e29984_d_n5;
        locals.var_t4_dn6 = assign17150_e29984_d_n6;
        locals.var_t4_dn7 = assign17150_e29984_d_n7;
        locals.var_t4_dn8 = assign17150_e29984_d_n8;
        locals.var_t4_dn9 = assign17150_e29984_d_n9;
        locals.var_t4_dn10 = assign17150_e29984_d_n10;
        locals.var_t4_dn11 = assign17150_e29984_d_n11;
        locals.var_t4_dn13 = assign17150_e29984_d_n13;
        locals.var_t4_dn14 = assign17150_e29984_d_n14;

    }

    pub(super) fn stamp_transient_block_55(
        locals: &mut StampLocals,
    ) {
        let (assign17160_e30015, assign17160_e30015_d_n0, assign17160_e30015_d_n2, assign17160_e30015_d_n3, assign17160_e30015_d_n4, assign17160_e30015_d_n5, assign17160_e30015_d_n6, assign17160_e30015_d_n7, assign17160_e30015_d_n8, assign17160_e30015_d_n9, assign17160_e30015_d_n10, assign17160_e30015_d_n11, assign17160_e30015_d_n13, assign17160_e30015_d_n14,) = {
    if ((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 == 0.0)) {
        let assign17160_e30005: f64 = (210.0 - locals.var_tr0_i);
        let assign17160_e30006: f64 = (locals.var_prt_i * assign17160_e30005);
        let assign17160_e30007: f64 = (1.0 + assign17160_e30006);
        let assign17160_e30011: f64 = (locals.var_tr0_i - locals.var_tnom);
        let assign17160_e30012: f64 = (locals.var_prt1_i * assign17160_e30011);
        let assign17160_e30013: f64 = (assign17160_e30007 + assign17160_e30012);
        (assign17160_e30013, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign17160_e30015;
        locals.var_t5_dn0 = assign17160_e30015_d_n0;
        locals.var_t5_dn2 = assign17160_e30015_d_n2;
        locals.var_t5_dn3 = assign17160_e30015_d_n3;
        locals.var_t5_dn4 = assign17160_e30015_d_n4;
        locals.var_t5_dn5 = assign17160_e30015_d_n5;
        locals.var_t5_dn6 = assign17160_e30015_d_n6;
        locals.var_t5_dn7 = assign17160_e30015_d_n7;
        locals.var_t5_dn8 = assign17160_e30015_d_n8;
        locals.var_t5_dn9 = assign17160_e30015_d_n9;
        locals.var_t5_dn10 = assign17160_e30015_d_n10;
        locals.var_t5_dn11 = assign17160_e30015_d_n11;
        locals.var_t5_dn13 = assign17160_e30015_d_n13;
        locals.var_t5_dn14 = assign17160_e30015_d_n14;

        let assign17170_e30018: f64 = if locals.var_prt1_i < locals.var_prt_i { 1.0 } else { 0.0 };
        locals.var_guard306 = assign17170_e30018;

        let (assign17180_e30079, assign17180_e30079_d_n0, assign17180_e30079_d_n2, assign17180_e30079_d_n3, assign17180_e30079_d_n4, assign17180_e30079_d_n5, assign17180_e30079_d_n6, assign17180_e30079_d_n7, assign17180_e30079_d_n8, assign17180_e30079_d_n9, assign17180_e30079_d_n10, assign17180_e30079_d_n11, assign17180_e30079_d_n13, assign17180_e30079_d_n14,) = {
    if (((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 == 0.0)) && (locals.var_guard306 != 0.0)) {
        let assign17180_e30040: f64 = (locals.var_rdstemp0 + locals.var_rdstemp1);
        let assign17180_e30043: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign17180_e30046: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign17180_e30047: f64 = (assign17180_e30043 * assign17180_e30046);
        let assign17180_e30050: f64 = (0.25 * locals.var_sprt_i);
        let assign17180_e30052: f64 = (assign17180_e30050 * locals.var_sprt_i);
        let assign17180_e30053: f64 = (assign17180_e30047 + assign17180_e30052);
        let assign17180_e30054: f64 = (assign17180_e30053).sqrt();
        let assign17180_e30055: f64 = (assign17180_e30040 + assign17180_e30054);
        let assign17180_e30056: f64 = (0.5 * assign17180_e30055);
        let assign17180_e30060: f64 = locals.var_t3;
        let assign17180_e30063: f64 = locals.var_t3;
        let assign17180_e30066: f64 = locals.var_t3;
        let assign17180_e30067: f64 = (assign17180_e30063 * assign17180_e30066);
        let assign17180_e30070: f64 = (0.25 * locals.var_sprt_i);
        let assign17180_e30072: f64 = (assign17180_e30070 * locals.var_sprt_i);
        let assign17180_e30073: f64 = (assign17180_e30067 + assign17180_e30072);
        let assign17180_e30074: f64 = (assign17180_e30073).sqrt();
        let assign17180_e30075: f64 = (assign17180_e30060 + assign17180_e30074);
        let assign17180_e30076: f64 = (0.5 * assign17180_e30075);
        let assign17180_e30077: f64 = (assign17180_e30056 - assign17180_e30076);
        (assign17180_e30077, (-(0.5 * (locals.var_t3_dn0 + (((locals.var_t3_dn0 * assign17180_e30066) + (assign17180_e30063 * locals.var_t3_dn0)) / (2.0 * assign17180_e30074))))), (-(0.5 * (locals.var_t3_dn2 + (((locals.var_t3_dn2 * assign17180_e30066) + (assign17180_e30063 * locals.var_t3_dn2)) / (2.0 * assign17180_e30074))))), (-(0.5 * (locals.var_t3_dn3 + (((locals.var_t3_dn3 * assign17180_e30066) + (assign17180_e30063 * locals.var_t3_dn3)) / (2.0 * assign17180_e30074))))), ((0.5 * ((locals.var_rdstemp0_dn4 + locals.var_rdstemp1_dn4) + ((((locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4) * assign17180_e30046) + (assign17180_e30043 * (locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4))) / (2.0 * assign17180_e30054)))) - (0.5 * (locals.var_t3_dn4 + (((locals.var_t3_dn4 * assign17180_e30066) + (assign17180_e30063 * locals.var_t3_dn4)) / (2.0 * assign17180_e30074))))), (-(0.5 * (locals.var_t3_dn5 + (((locals.var_t3_dn5 * assign17180_e30066) + (assign17180_e30063 * locals.var_t3_dn5)) / (2.0 * assign17180_e30074))))), (-(0.5 * (locals.var_t3_dn6 + (((locals.var_t3_dn6 * assign17180_e30066) + (assign17180_e30063 * locals.var_t3_dn6)) / (2.0 * assign17180_e30074))))), (-(0.5 * (locals.var_t3_dn7 + (((locals.var_t3_dn7 * assign17180_e30066) + (assign17180_e30063 * locals.var_t3_dn7)) / (2.0 * assign17180_e30074))))), (-(0.5 * (locals.var_t3_dn8 + (((locals.var_t3_dn8 * assign17180_e30066) + (assign17180_e30063 * locals.var_t3_dn8)) / (2.0 * assign17180_e30074))))), (-(0.5 * (locals.var_t3_dn9 + (((locals.var_t3_dn9 * assign17180_e30066) + (assign17180_e30063 * locals.var_t3_dn9)) / (2.0 * assign17180_e30074))))), (-(0.5 * (locals.var_t3_dn10 + (((locals.var_t3_dn10 * assign17180_e30066) + (assign17180_e30063 * locals.var_t3_dn10)) / (2.0 * assign17180_e30074))))), (-(0.5 * (locals.var_t3_dn11 + (((locals.var_t3_dn11 * assign17180_e30066) + (assign17180_e30063 * locals.var_t3_dn11)) / (2.0 * assign17180_e30074))))), (-(0.5 * (locals.var_t3_dn13 + (((locals.var_t3_dn13 * assign17180_e30066) + (assign17180_e30063 * locals.var_t3_dn13)) / (2.0 * assign17180_e30074))))), (-(0.5 * (locals.var_t3_dn14 + (((locals.var_t3_dn14 * assign17180_e30066) + (assign17180_e30063 * locals.var_t3_dn14)) / (2.0 * assign17180_e30074))))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign17180_e30079;
        locals.var_t6_dn0 = assign17180_e30079_d_n0;
        locals.var_t6_dn2 = assign17180_e30079_d_n2;
        locals.var_t6_dn3 = assign17180_e30079_d_n3;
        locals.var_t6_dn4 = assign17180_e30079_d_n4;
        locals.var_t6_dn5 = assign17180_e30079_d_n5;
        locals.var_t6_dn6 = assign17180_e30079_d_n6;
        locals.var_t6_dn7 = assign17180_e30079_d_n7;
        locals.var_t6_dn8 = assign17180_e30079_d_n8;
        locals.var_t6_dn9 = assign17180_e30079_d_n9;
        locals.var_t6_dn10 = assign17180_e30079_d_n10;
        locals.var_t6_dn11 = assign17180_e30079_d_n11;
        locals.var_t6_dn13 = assign17180_e30079_d_n13;
        locals.var_t6_dn14 = assign17180_e30079_d_n14;

        let (assign17190_e30140, assign17190_e30140_d_n0, assign17190_e30140_d_n2, assign17190_e30140_d_n3, assign17190_e30140_d_n4, assign17190_e30140_d_n5, assign17190_e30140_d_n6, assign17190_e30140_d_n7, assign17190_e30140_d_n8, assign17190_e30140_d_n9, assign17190_e30140_d_n10, assign17190_e30140_d_n11, assign17190_e30140_d_n13, assign17190_e30140_d_n14,) = {
    if (((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 == 0.0)) && (locals.var_guard306 != 0.0)) {
        let assign17190_e30101: f64 = (locals.var_t4 + locals.var_t5);
        let assign17190_e30104: f64 = (locals.var_t4 - locals.var_t5);
        let assign17190_e30107: f64 = (locals.var_t4 - locals.var_t5);
        let assign17190_e30108: f64 = (assign17190_e30104 * assign17190_e30107);
        let assign17190_e30111: f64 = (0.25 * locals.var_sprt_i);
        let assign17190_e30113: f64 = (assign17190_e30111 * locals.var_sprt_i);
        let assign17190_e30114: f64 = (assign17190_e30108 + assign17190_e30113);
        let assign17190_e30115: f64 = (assign17190_e30114).sqrt();
        let assign17190_e30116: f64 = (assign17190_e30101 + assign17190_e30115);
        let assign17190_e30117: f64 = (0.5 * assign17190_e30116);
        let assign17190_e30121: f64 = locals.var_t3;
        let assign17190_e30124: f64 = locals.var_t3;
        let assign17190_e30127: f64 = locals.var_t3;
        let assign17190_e30128: f64 = (assign17190_e30124 * assign17190_e30127);
        let assign17190_e30131: f64 = (0.25 * locals.var_sprt_i);
        let assign17190_e30133: f64 = (assign17190_e30131 * locals.var_sprt_i);
        let assign17190_e30134: f64 = (assign17190_e30128 + assign17190_e30133);
        let assign17190_e30135: f64 = (assign17190_e30134).sqrt();
        let assign17190_e30136: f64 = (assign17190_e30121 + assign17190_e30135);
        let assign17190_e30137: f64 = (0.5 * assign17190_e30136);
        let assign17190_e30138: f64 = (assign17190_e30117 - assign17190_e30137);
        (assign17190_e30138, ((0.5 * ((locals.var_t4_dn0 + locals.var_t5_dn0) + ((((locals.var_t4_dn0 - locals.var_t5_dn0) * assign17190_e30107) + (assign17190_e30104 * (locals.var_t4_dn0 - locals.var_t5_dn0))) / (2.0 * assign17190_e30115)))) - (0.5 * (locals.var_t3_dn0 + (((locals.var_t3_dn0 * assign17190_e30127) + (assign17190_e30124 * locals.var_t3_dn0)) / (2.0 * assign17190_e30135))))), ((0.5 * ((locals.var_t4_dn2 + locals.var_t5_dn2) + ((((locals.var_t4_dn2 - locals.var_t5_dn2) * assign17190_e30107) + (assign17190_e30104 * (locals.var_t4_dn2 - locals.var_t5_dn2))) / (2.0 * assign17190_e30115)))) - (0.5 * (locals.var_t3_dn2 + (((locals.var_t3_dn2 * assign17190_e30127) + (assign17190_e30124 * locals.var_t3_dn2)) / (2.0 * assign17190_e30135))))), ((0.5 * ((locals.var_t4_dn3 + locals.var_t5_dn3) + ((((locals.var_t4_dn3 - locals.var_t5_dn3) * assign17190_e30107) + (assign17190_e30104 * (locals.var_t4_dn3 - locals.var_t5_dn3))) / (2.0 * assign17190_e30115)))) - (0.5 * (locals.var_t3_dn3 + (((locals.var_t3_dn3 * assign17190_e30127) + (assign17190_e30124 * locals.var_t3_dn3)) / (2.0 * assign17190_e30135))))), ((0.5 * ((locals.var_t4_dn4 + locals.var_t5_dn4) + ((((locals.var_t4_dn4 - locals.var_t5_dn4) * assign17190_e30107) + (assign17190_e30104 * (locals.var_t4_dn4 - locals.var_t5_dn4))) / (2.0 * assign17190_e30115)))) - (0.5 * (locals.var_t3_dn4 + (((locals.var_t3_dn4 * assign17190_e30127) + (assign17190_e30124 * locals.var_t3_dn4)) / (2.0 * assign17190_e30135))))), ((0.5 * ((locals.var_t4_dn5 + locals.var_t5_dn5) + ((((locals.var_t4_dn5 - locals.var_t5_dn5) * assign17190_e30107) + (assign17190_e30104 * (locals.var_t4_dn5 - locals.var_t5_dn5))) / (2.0 * assign17190_e30115)))) - (0.5 * (locals.var_t3_dn5 + (((locals.var_t3_dn5 * assign17190_e30127) + (assign17190_e30124 * locals.var_t3_dn5)) / (2.0 * assign17190_e30135))))), ((0.5 * ((locals.var_t4_dn6 + locals.var_t5_dn6) + ((((locals.var_t4_dn6 - locals.var_t5_dn6) * assign17190_e30107) + (assign17190_e30104 * (locals.var_t4_dn6 - locals.var_t5_dn6))) / (2.0 * assign17190_e30115)))) - (0.5 * (locals.var_t3_dn6 + (((locals.var_t3_dn6 * assign17190_e30127) + (assign17190_e30124 * locals.var_t3_dn6)) / (2.0 * assign17190_e30135))))), ((0.5 * ((locals.var_t4_dn7 + locals.var_t5_dn7) + ((((locals.var_t4_dn7 - locals.var_t5_dn7) * assign17190_e30107) + (assign17190_e30104 * (locals.var_t4_dn7 - locals.var_t5_dn7))) / (2.0 * assign17190_e30115)))) - (0.5 * (locals.var_t3_dn7 + (((locals.var_t3_dn7 * assign17190_e30127) + (assign17190_e30124 * locals.var_t3_dn7)) / (2.0 * assign17190_e30135))))), ((0.5 * ((locals.var_t4_dn8 + locals.var_t5_dn8) + ((((locals.var_t4_dn8 - locals.var_t5_dn8) * assign17190_e30107) + (assign17190_e30104 * (locals.var_t4_dn8 - locals.var_t5_dn8))) / (2.0 * assign17190_e30115)))) - (0.5 * (locals.var_t3_dn8 + (((locals.var_t3_dn8 * assign17190_e30127) + (assign17190_e30124 * locals.var_t3_dn8)) / (2.0 * assign17190_e30135))))), ((0.5 * ((locals.var_t4_dn9 + locals.var_t5_dn9) + ((((locals.var_t4_dn9 - locals.var_t5_dn9) * assign17190_e30107) + (assign17190_e30104 * (locals.var_t4_dn9 - locals.var_t5_dn9))) / (2.0 * assign17190_e30115)))) - (0.5 * (locals.var_t3_dn9 + (((locals.var_t3_dn9 * assign17190_e30127) + (assign17190_e30124 * locals.var_t3_dn9)) / (2.0 * assign17190_e30135))))), ((0.5 * ((locals.var_t4_dn10 + locals.var_t5_dn10) + ((((locals.var_t4_dn10 - locals.var_t5_dn10) * assign17190_e30107) + (assign17190_e30104 * (locals.var_t4_dn10 - locals.var_t5_dn10))) / (2.0 * assign17190_e30115)))) - (0.5 * (locals.var_t3_dn10 + (((locals.var_t3_dn10 * assign17190_e30127) + (assign17190_e30124 * locals.var_t3_dn10)) / (2.0 * assign17190_e30135))))), ((0.5 * ((locals.var_t4_dn11 + locals.var_t5_dn11) + ((((locals.var_t4_dn11 - locals.var_t5_dn11) * assign17190_e30107) + (assign17190_e30104 * (locals.var_t4_dn11 - locals.var_t5_dn11))) / (2.0 * assign17190_e30115)))) - (0.5 * (locals.var_t3_dn11 + (((locals.var_t3_dn11 * assign17190_e30127) + (assign17190_e30124 * locals.var_t3_dn11)) / (2.0 * assign17190_e30135))))), ((0.5 * ((locals.var_t4_dn13 + locals.var_t5_dn13) + ((((locals.var_t4_dn13 - locals.var_t5_dn13) * assign17190_e30107) + (assign17190_e30104 * (locals.var_t4_dn13 - locals.var_t5_dn13))) / (2.0 * assign17190_e30115)))) - (0.5 * (locals.var_t3_dn13 + (((locals.var_t3_dn13 * assign17190_e30127) + (assign17190_e30124 * locals.var_t3_dn13)) / (2.0 * assign17190_e30135))))), ((0.5 * ((locals.var_t4_dn14 + locals.var_t5_dn14) + ((((locals.var_t4_dn14 - locals.var_t5_dn14) * assign17190_e30107) + (assign17190_e30104 * (locals.var_t4_dn14 - locals.var_t5_dn14))) / (2.0 * assign17190_e30115)))) - (0.5 * (locals.var_t3_dn14 + (((locals.var_t3_dn14 * assign17190_e30127) + (assign17190_e30124 * locals.var_t3_dn14)) / (2.0 * assign17190_e30135))))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign17190_e30140;
        locals.var_t7_dn0 = assign17190_e30140_d_n0;
        locals.var_t7_dn2 = assign17190_e30140_d_n2;
        locals.var_t7_dn3 = assign17190_e30140_d_n3;
        locals.var_t7_dn4 = assign17190_e30140_d_n4;
        locals.var_t7_dn5 = assign17190_e30140_d_n5;
        locals.var_t7_dn6 = assign17190_e30140_d_n6;
        locals.var_t7_dn7 = assign17190_e30140_d_n7;
        locals.var_t7_dn8 = assign17190_e30140_d_n8;
        locals.var_t7_dn9 = assign17190_e30140_d_n9;
        locals.var_t7_dn10 = assign17190_e30140_d_n10;
        locals.var_t7_dn11 = assign17190_e30140_d_n11;
        locals.var_t7_dn13 = assign17190_e30140_d_n13;
        locals.var_t7_dn14 = assign17190_e30140_d_n14;

        let (assign17200_e30167, assign17200_e30167_d_n0, assign17200_e30167_d_n2, assign17200_e30167_d_n3, assign17200_e30167_d_n4, assign17200_e30167_d_n5, assign17200_e30167_d_n6, assign17200_e30167_d_n7, assign17200_e30167_d_n8, assign17200_e30167_d_n9, assign17200_e30167_d_n10, assign17200_e30167_d_n11, assign17200_e30167_d_n13, assign17200_e30167_d_n14,) = {
    if (((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 == 0.0)) && (locals.var_guard306 != 0.0)) {
        let assign17200_e30163: f64 = (locals.var_devtemp - 210.0);
        let assign17200_e30164: f64 = (locals.var_prt_i * assign17200_e30163);
        let assign17200_e30165: f64 = (locals.var_t7 + assign17200_e30164);
        (assign17200_e30165, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, (locals.var_t7_dn4 + (locals.var_prt_i * locals.var_devtemp_dn4)), locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn13, locals.var_t7_dn14,)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign17200_e30167;
        locals.var_t8_dn0 = assign17200_e30167_d_n0;
        locals.var_t8_dn2 = assign17200_e30167_d_n2;
        locals.var_t8_dn3 = assign17200_e30167_d_n3;
        locals.var_t8_dn4 = assign17200_e30167_d_n4;
        locals.var_t8_dn5 = assign17200_e30167_d_n5;
        locals.var_t8_dn6 = assign17200_e30167_d_n6;
        locals.var_t8_dn7 = assign17200_e30167_d_n7;
        locals.var_t8_dn8 = assign17200_e30167_d_n8;
        locals.var_t8_dn9 = assign17200_e30167_d_n9;
        locals.var_t8_dn10 = assign17200_e30167_d_n10;
        locals.var_t8_dn11 = assign17200_e30167_d_n11;
        locals.var_t8_dn13 = assign17200_e30167_d_n13;
        locals.var_t8_dn14 = assign17200_e30167_d_n14;

        let (assign17210_e30207, assign17210_e30207_d_n0, assign17210_e30207_d_n2, assign17210_e30207_d_n3, assign17210_e30207_d_n4, assign17210_e30207_d_n5, assign17210_e30207_d_n6, assign17210_e30207_d_n7, assign17210_e30207_d_n8, assign17210_e30207_d_n9, assign17210_e30207_d_n10, assign17210_e30207_d_n11, assign17210_e30207_d_n13, assign17210_e30207_d_n14,) = {
    if (((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 == 0.0)) && (locals.var_guard306 != 0.0)) {
        let assign17210_e30189: f64 = (locals.var_t6 + locals.var_t8);
        let assign17210_e30192: f64 = (locals.var_t6 - locals.var_t8);
        let assign17210_e30195: f64 = (locals.var_t6 - locals.var_t8);
        let assign17210_e30196: f64 = (assign17210_e30192 * assign17210_e30195);
        let assign17210_e30199: f64 = (0.25 * 0.001);
        let assign17210_e30201: f64 = (assign17210_e30199 * 0.001);
        let assign17210_e30202: f64 = (assign17210_e30196 + assign17210_e30201);
        let assign17210_e30203: f64 = (assign17210_e30202).sqrt();
        let assign17210_e30204: f64 = (assign17210_e30189 + assign17210_e30203);
        let assign17210_e30205: f64 = (0.5 * assign17210_e30204);
        (assign17210_e30205, (0.5 * ((locals.var_t6_dn0 + locals.var_t8_dn0) + ((((locals.var_t6_dn0 - locals.var_t8_dn0) * assign17210_e30195) + (assign17210_e30192 * (locals.var_t6_dn0 - locals.var_t8_dn0))) / (2.0 * assign17210_e30203)))), (0.5 * ((locals.var_t6_dn2 + locals.var_t8_dn2) + ((((locals.var_t6_dn2 - locals.var_t8_dn2) * assign17210_e30195) + (assign17210_e30192 * (locals.var_t6_dn2 - locals.var_t8_dn2))) / (2.0 * assign17210_e30203)))), (0.5 * ((locals.var_t6_dn3 + locals.var_t8_dn3) + ((((locals.var_t6_dn3 - locals.var_t8_dn3) * assign17210_e30195) + (assign17210_e30192 * (locals.var_t6_dn3 - locals.var_t8_dn3))) / (2.0 * assign17210_e30203)))), (0.5 * ((locals.var_t6_dn4 + locals.var_t8_dn4) + ((((locals.var_t6_dn4 - locals.var_t8_dn4) * assign17210_e30195) + (assign17210_e30192 * (locals.var_t6_dn4 - locals.var_t8_dn4))) / (2.0 * assign17210_e30203)))), (0.5 * ((locals.var_t6_dn5 + locals.var_t8_dn5) + ((((locals.var_t6_dn5 - locals.var_t8_dn5) * assign17210_e30195) + (assign17210_e30192 * (locals.var_t6_dn5 - locals.var_t8_dn5))) / (2.0 * assign17210_e30203)))), (0.5 * ((locals.var_t6_dn6 + locals.var_t8_dn6) + ((((locals.var_t6_dn6 - locals.var_t8_dn6) * assign17210_e30195) + (assign17210_e30192 * (locals.var_t6_dn6 - locals.var_t8_dn6))) / (2.0 * assign17210_e30203)))), (0.5 * ((locals.var_t6_dn7 + locals.var_t8_dn7) + ((((locals.var_t6_dn7 - locals.var_t8_dn7) * assign17210_e30195) + (assign17210_e30192 * (locals.var_t6_dn7 - locals.var_t8_dn7))) / (2.0 * assign17210_e30203)))), (0.5 * ((locals.var_t6_dn8 + locals.var_t8_dn8) + ((((locals.var_t6_dn8 - locals.var_t8_dn8) * assign17210_e30195) + (assign17210_e30192 * (locals.var_t6_dn8 - locals.var_t8_dn8))) / (2.0 * assign17210_e30203)))), (0.5 * ((locals.var_t6_dn9 + locals.var_t8_dn9) + ((((locals.var_t6_dn9 - locals.var_t8_dn9) * assign17210_e30195) + (assign17210_e30192 * (locals.var_t6_dn9 - locals.var_t8_dn9))) / (2.0 * assign17210_e30203)))), (0.5 * ((locals.var_t6_dn10 + locals.var_t8_dn10) + ((((locals.var_t6_dn10 - locals.var_t8_dn10) * assign17210_e30195) + (assign17210_e30192 * (locals.var_t6_dn10 - locals.var_t8_dn10))) / (2.0 * assign17210_e30203)))), (0.5 * ((locals.var_t6_dn11 + locals.var_t8_dn11) + ((((locals.var_t6_dn11 - locals.var_t8_dn11) * assign17210_e30195) + (assign17210_e30192 * (locals.var_t6_dn11 - locals.var_t8_dn11))) / (2.0 * assign17210_e30203)))), (0.5 * ((locals.var_t6_dn13 + locals.var_t8_dn13) + ((((locals.var_t6_dn13 - locals.var_t8_dn13) * assign17210_e30195) + (assign17210_e30192 * (locals.var_t6_dn13 - locals.var_t8_dn13))) / (2.0 * assign17210_e30203)))), (0.5 * ((locals.var_t6_dn14 + locals.var_t8_dn14) + ((((locals.var_t6_dn14 - locals.var_t8_dn14) * assign17210_e30195) + (assign17210_e30192 * (locals.var_t6_dn14 - locals.var_t8_dn14))) / (2.0 * assign17210_e30203)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign17210_e30207;
        locals.var_t2_dn0 = assign17210_e30207_d_n0;
        locals.var_t2_dn2 = assign17210_e30207_d_n2;
        locals.var_t2_dn3 = assign17210_e30207_d_n3;
        locals.var_t2_dn4 = assign17210_e30207_d_n4;
        locals.var_t2_dn5 = assign17210_e30207_d_n5;
        locals.var_t2_dn6 = assign17210_e30207_d_n6;
        locals.var_t2_dn7 = assign17210_e30207_d_n7;
        locals.var_t2_dn8 = assign17210_e30207_d_n8;
        locals.var_t2_dn9 = assign17210_e30207_d_n9;
        locals.var_t2_dn10 = assign17210_e30207_d_n10;
        locals.var_t2_dn11 = assign17210_e30207_d_n11;
        locals.var_t2_dn13 = assign17210_e30207_d_n13;
        locals.var_t2_dn14 = assign17210_e30207_d_n14;

        let (assign17220_e30269, assign17220_e30269_d_n0, assign17220_e30269_d_n2, assign17220_e30269_d_n3, assign17220_e30269_d_n4, assign17220_e30269_d_n5, assign17220_e30269_d_n6, assign17220_e30269_d_n7, assign17220_e30269_d_n8, assign17220_e30269_d_n9, assign17220_e30269_d_n10, assign17220_e30269_d_n11, assign17220_e30269_d_n13, assign17220_e30269_d_n14,) = {
    if (((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 == 0.0)) && (locals.var_guard306 == 0.0)) {
        let assign17220_e30230: f64 = (locals.var_rdstemp0 + locals.var_rdstemp1);
        let assign17220_e30233: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign17220_e30236: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign17220_e30237: f64 = (assign17220_e30233 * assign17220_e30236);
        let assign17220_e30240: f64 = (0.25 * locals.var_sprt_i);
        let assign17220_e30242: f64 = (assign17220_e30240 * locals.var_sprt_i);
        let assign17220_e30243: f64 = (assign17220_e30237 + assign17220_e30242);
        let assign17220_e30244: f64 = (assign17220_e30243).sqrt();
        let assign17220_e30245: f64 = (assign17220_e30230 - assign17220_e30244);
        let assign17220_e30246: f64 = (0.5 * assign17220_e30245);
        let assign17220_e30250: f64 = locals.var_t3;
        let assign17220_e30253: f64 = locals.var_t3;
        let assign17220_e30256: f64 = locals.var_t3;
        let assign17220_e30257: f64 = (assign17220_e30253 * assign17220_e30256);
        let assign17220_e30260: f64 = (0.25 * locals.var_sprt_i);
        let assign17220_e30262: f64 = (assign17220_e30260 * locals.var_sprt_i);
        let assign17220_e30263: f64 = (assign17220_e30257 + assign17220_e30262);
        let assign17220_e30264: f64 = (assign17220_e30263).sqrt();
        let assign17220_e30265: f64 = (assign17220_e30250 - assign17220_e30264);
        let assign17220_e30266: f64 = (0.5 * assign17220_e30265);
        let assign17220_e30267: f64 = (assign17220_e30246 - assign17220_e30266);
        (assign17220_e30267, (-(0.5 * (locals.var_t3_dn0 - (((locals.var_t3_dn0 * assign17220_e30256) + (assign17220_e30253 * locals.var_t3_dn0)) / (2.0 * assign17220_e30264))))), (-(0.5 * (locals.var_t3_dn2 - (((locals.var_t3_dn2 * assign17220_e30256) + (assign17220_e30253 * locals.var_t3_dn2)) / (2.0 * assign17220_e30264))))), (-(0.5 * (locals.var_t3_dn3 - (((locals.var_t3_dn3 * assign17220_e30256) + (assign17220_e30253 * locals.var_t3_dn3)) / (2.0 * assign17220_e30264))))), ((0.5 * ((locals.var_rdstemp0_dn4 + locals.var_rdstemp1_dn4) - ((((locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4) * assign17220_e30236) + (assign17220_e30233 * (locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4))) / (2.0 * assign17220_e30244)))) - (0.5 * (locals.var_t3_dn4 - (((locals.var_t3_dn4 * assign17220_e30256) + (assign17220_e30253 * locals.var_t3_dn4)) / (2.0 * assign17220_e30264))))), (-(0.5 * (locals.var_t3_dn5 - (((locals.var_t3_dn5 * assign17220_e30256) + (assign17220_e30253 * locals.var_t3_dn5)) / (2.0 * assign17220_e30264))))), (-(0.5 * (locals.var_t3_dn6 - (((locals.var_t3_dn6 * assign17220_e30256) + (assign17220_e30253 * locals.var_t3_dn6)) / (2.0 * assign17220_e30264))))), (-(0.5 * (locals.var_t3_dn7 - (((locals.var_t3_dn7 * assign17220_e30256) + (assign17220_e30253 * locals.var_t3_dn7)) / (2.0 * assign17220_e30264))))), (-(0.5 * (locals.var_t3_dn8 - (((locals.var_t3_dn8 * assign17220_e30256) + (assign17220_e30253 * locals.var_t3_dn8)) / (2.0 * assign17220_e30264))))), (-(0.5 * (locals.var_t3_dn9 - (((locals.var_t3_dn9 * assign17220_e30256) + (assign17220_e30253 * locals.var_t3_dn9)) / (2.0 * assign17220_e30264))))), (-(0.5 * (locals.var_t3_dn10 - (((locals.var_t3_dn10 * assign17220_e30256) + (assign17220_e30253 * locals.var_t3_dn10)) / (2.0 * assign17220_e30264))))), (-(0.5 * (locals.var_t3_dn11 - (((locals.var_t3_dn11 * assign17220_e30256) + (assign17220_e30253 * locals.var_t3_dn11)) / (2.0 * assign17220_e30264))))), (-(0.5 * (locals.var_t3_dn13 - (((locals.var_t3_dn13 * assign17220_e30256) + (assign17220_e30253 * locals.var_t3_dn13)) / (2.0 * assign17220_e30264))))), (-(0.5 * (locals.var_t3_dn14 - (((locals.var_t3_dn14 * assign17220_e30256) + (assign17220_e30253 * locals.var_t3_dn14)) / (2.0 * assign17220_e30264))))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign17220_e30269;
        locals.var_t6_dn0 = assign17220_e30269_d_n0;
        locals.var_t6_dn2 = assign17220_e30269_d_n2;
        locals.var_t6_dn3 = assign17220_e30269_d_n3;
        locals.var_t6_dn4 = assign17220_e30269_d_n4;
        locals.var_t6_dn5 = assign17220_e30269_d_n5;
        locals.var_t6_dn6 = assign17220_e30269_d_n6;
        locals.var_t6_dn7 = assign17220_e30269_d_n7;
        locals.var_t6_dn8 = assign17220_e30269_d_n8;
        locals.var_t6_dn9 = assign17220_e30269_d_n9;
        locals.var_t6_dn10 = assign17220_e30269_d_n10;
        locals.var_t6_dn11 = assign17220_e30269_d_n11;
        locals.var_t6_dn13 = assign17220_e30269_d_n13;
        locals.var_t6_dn14 = assign17220_e30269_d_n14;

        let (assign17230_e30331, assign17230_e30331_d_n0, assign17230_e30331_d_n2, assign17230_e30331_d_n3, assign17230_e30331_d_n4, assign17230_e30331_d_n5, assign17230_e30331_d_n6, assign17230_e30331_d_n7, assign17230_e30331_d_n8, assign17230_e30331_d_n9, assign17230_e30331_d_n10, assign17230_e30331_d_n11, assign17230_e30331_d_n13, assign17230_e30331_d_n14,) = {
    if (((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 == 0.0)) && (locals.var_guard306 == 0.0)) {
        let assign17230_e30292: f64 = (locals.var_t4 + locals.var_t5);
        let assign17230_e30295: f64 = (locals.var_t4 - locals.var_t5);
        let assign17230_e30298: f64 = (locals.var_t4 - locals.var_t5);
        let assign17230_e30299: f64 = (assign17230_e30295 * assign17230_e30298);
        let assign17230_e30302: f64 = (0.25 * locals.var_sprt_i);
        let assign17230_e30304: f64 = (assign17230_e30302 * locals.var_sprt_i);
        let assign17230_e30305: f64 = (assign17230_e30299 + assign17230_e30304);
        let assign17230_e30306: f64 = (assign17230_e30305).sqrt();
        let assign17230_e30307: f64 = (assign17230_e30292 - assign17230_e30306);
        let assign17230_e30308: f64 = (0.5 * assign17230_e30307);
        let assign17230_e30312: f64 = locals.var_t3;
        let assign17230_e30315: f64 = locals.var_t3;
        let assign17230_e30318: f64 = locals.var_t3;
        let assign17230_e30319: f64 = (assign17230_e30315 * assign17230_e30318);
        let assign17230_e30322: f64 = (0.25 * locals.var_sprt_i);
        let assign17230_e30324: f64 = (assign17230_e30322 * locals.var_sprt_i);
        let assign17230_e30325: f64 = (assign17230_e30319 + assign17230_e30324);
        let assign17230_e30326: f64 = (assign17230_e30325).sqrt();
        let assign17230_e30327: f64 = (assign17230_e30312 - assign17230_e30326);
        let assign17230_e30328: f64 = (0.5 * assign17230_e30327);
        let assign17230_e30329: f64 = (assign17230_e30308 - assign17230_e30328);
        (assign17230_e30329, ((0.5 * ((locals.var_t4_dn0 + locals.var_t5_dn0) - ((((locals.var_t4_dn0 - locals.var_t5_dn0) * assign17230_e30298) + (assign17230_e30295 * (locals.var_t4_dn0 - locals.var_t5_dn0))) / (2.0 * assign17230_e30306)))) - (0.5 * (locals.var_t3_dn0 - (((locals.var_t3_dn0 * assign17230_e30318) + (assign17230_e30315 * locals.var_t3_dn0)) / (2.0 * assign17230_e30326))))), ((0.5 * ((locals.var_t4_dn2 + locals.var_t5_dn2) - ((((locals.var_t4_dn2 - locals.var_t5_dn2) * assign17230_e30298) + (assign17230_e30295 * (locals.var_t4_dn2 - locals.var_t5_dn2))) / (2.0 * assign17230_e30306)))) - (0.5 * (locals.var_t3_dn2 - (((locals.var_t3_dn2 * assign17230_e30318) + (assign17230_e30315 * locals.var_t3_dn2)) / (2.0 * assign17230_e30326))))), ((0.5 * ((locals.var_t4_dn3 + locals.var_t5_dn3) - ((((locals.var_t4_dn3 - locals.var_t5_dn3) * assign17230_e30298) + (assign17230_e30295 * (locals.var_t4_dn3 - locals.var_t5_dn3))) / (2.0 * assign17230_e30306)))) - (0.5 * (locals.var_t3_dn3 - (((locals.var_t3_dn3 * assign17230_e30318) + (assign17230_e30315 * locals.var_t3_dn3)) / (2.0 * assign17230_e30326))))), ((0.5 * ((locals.var_t4_dn4 + locals.var_t5_dn4) - ((((locals.var_t4_dn4 - locals.var_t5_dn4) * assign17230_e30298) + (assign17230_e30295 * (locals.var_t4_dn4 - locals.var_t5_dn4))) / (2.0 * assign17230_e30306)))) - (0.5 * (locals.var_t3_dn4 - (((locals.var_t3_dn4 * assign17230_e30318) + (assign17230_e30315 * locals.var_t3_dn4)) / (2.0 * assign17230_e30326))))), ((0.5 * ((locals.var_t4_dn5 + locals.var_t5_dn5) - ((((locals.var_t4_dn5 - locals.var_t5_dn5) * assign17230_e30298) + (assign17230_e30295 * (locals.var_t4_dn5 - locals.var_t5_dn5))) / (2.0 * assign17230_e30306)))) - (0.5 * (locals.var_t3_dn5 - (((locals.var_t3_dn5 * assign17230_e30318) + (assign17230_e30315 * locals.var_t3_dn5)) / (2.0 * assign17230_e30326))))), ((0.5 * ((locals.var_t4_dn6 + locals.var_t5_dn6) - ((((locals.var_t4_dn6 - locals.var_t5_dn6) * assign17230_e30298) + (assign17230_e30295 * (locals.var_t4_dn6 - locals.var_t5_dn6))) / (2.0 * assign17230_e30306)))) - (0.5 * (locals.var_t3_dn6 - (((locals.var_t3_dn6 * assign17230_e30318) + (assign17230_e30315 * locals.var_t3_dn6)) / (2.0 * assign17230_e30326))))), ((0.5 * ((locals.var_t4_dn7 + locals.var_t5_dn7) - ((((locals.var_t4_dn7 - locals.var_t5_dn7) * assign17230_e30298) + (assign17230_e30295 * (locals.var_t4_dn7 - locals.var_t5_dn7))) / (2.0 * assign17230_e30306)))) - (0.5 * (locals.var_t3_dn7 - (((locals.var_t3_dn7 * assign17230_e30318) + (assign17230_e30315 * locals.var_t3_dn7)) / (2.0 * assign17230_e30326))))), ((0.5 * ((locals.var_t4_dn8 + locals.var_t5_dn8) - ((((locals.var_t4_dn8 - locals.var_t5_dn8) * assign17230_e30298) + (assign17230_e30295 * (locals.var_t4_dn8 - locals.var_t5_dn8))) / (2.0 * assign17230_e30306)))) - (0.5 * (locals.var_t3_dn8 - (((locals.var_t3_dn8 * assign17230_e30318) + (assign17230_e30315 * locals.var_t3_dn8)) / (2.0 * assign17230_e30326))))), ((0.5 * ((locals.var_t4_dn9 + locals.var_t5_dn9) - ((((locals.var_t4_dn9 - locals.var_t5_dn9) * assign17230_e30298) + (assign17230_e30295 * (locals.var_t4_dn9 - locals.var_t5_dn9))) / (2.0 * assign17230_e30306)))) - (0.5 * (locals.var_t3_dn9 - (((locals.var_t3_dn9 * assign17230_e30318) + (assign17230_e30315 * locals.var_t3_dn9)) / (2.0 * assign17230_e30326))))), ((0.5 * ((locals.var_t4_dn10 + locals.var_t5_dn10) - ((((locals.var_t4_dn10 - locals.var_t5_dn10) * assign17230_e30298) + (assign17230_e30295 * (locals.var_t4_dn10 - locals.var_t5_dn10))) / (2.0 * assign17230_e30306)))) - (0.5 * (locals.var_t3_dn10 - (((locals.var_t3_dn10 * assign17230_e30318) + (assign17230_e30315 * locals.var_t3_dn10)) / (2.0 * assign17230_e30326))))), ((0.5 * ((locals.var_t4_dn11 + locals.var_t5_dn11) - ((((locals.var_t4_dn11 - locals.var_t5_dn11) * assign17230_e30298) + (assign17230_e30295 * (locals.var_t4_dn11 - locals.var_t5_dn11))) / (2.0 * assign17230_e30306)))) - (0.5 * (locals.var_t3_dn11 - (((locals.var_t3_dn11 * assign17230_e30318) + (assign17230_e30315 * locals.var_t3_dn11)) / (2.0 * assign17230_e30326))))), ((0.5 * ((locals.var_t4_dn13 + locals.var_t5_dn13) - ((((locals.var_t4_dn13 - locals.var_t5_dn13) * assign17230_e30298) + (assign17230_e30295 * (locals.var_t4_dn13 - locals.var_t5_dn13))) / (2.0 * assign17230_e30306)))) - (0.5 * (locals.var_t3_dn13 - (((locals.var_t3_dn13 * assign17230_e30318) + (assign17230_e30315 * locals.var_t3_dn13)) / (2.0 * assign17230_e30326))))), ((0.5 * ((locals.var_t4_dn14 + locals.var_t5_dn14) - ((((locals.var_t4_dn14 - locals.var_t5_dn14) * assign17230_e30298) + (assign17230_e30295 * (locals.var_t4_dn14 - locals.var_t5_dn14))) / (2.0 * assign17230_e30306)))) - (0.5 * (locals.var_t3_dn14 - (((locals.var_t3_dn14 * assign17230_e30318) + (assign17230_e30315 * locals.var_t3_dn14)) / (2.0 * assign17230_e30326))))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign17230_e30331;
        locals.var_t7_dn0 = assign17230_e30331_d_n0;
        locals.var_t7_dn2 = assign17230_e30331_d_n2;
        locals.var_t7_dn3 = assign17230_e30331_d_n3;
        locals.var_t7_dn4 = assign17230_e30331_d_n4;
        locals.var_t7_dn5 = assign17230_e30331_d_n5;
        locals.var_t7_dn6 = assign17230_e30331_d_n6;
        locals.var_t7_dn7 = assign17230_e30331_d_n7;
        locals.var_t7_dn8 = assign17230_e30331_d_n8;
        locals.var_t7_dn9 = assign17230_e30331_d_n9;
        locals.var_t7_dn10 = assign17230_e30331_d_n10;
        locals.var_t7_dn11 = assign17230_e30331_d_n11;
        locals.var_t7_dn13 = assign17230_e30331_d_n13;
        locals.var_t7_dn14 = assign17230_e30331_d_n14;

        let (assign17240_e30359, assign17240_e30359_d_n0, assign17240_e30359_d_n2, assign17240_e30359_d_n3, assign17240_e30359_d_n4, assign17240_e30359_d_n5, assign17240_e30359_d_n6, assign17240_e30359_d_n7, assign17240_e30359_d_n8, assign17240_e30359_d_n9, assign17240_e30359_d_n10, assign17240_e30359_d_n11, assign17240_e30359_d_n13, assign17240_e30359_d_n14,) = {
    if (((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 == 0.0)) && (locals.var_guard306 == 0.0)) {
        let assign17240_e30355: f64 = (locals.var_devtemp - 210.0);
        let assign17240_e30356: f64 = (locals.var_prt_i * assign17240_e30355);
        let assign17240_e30357: f64 = (locals.var_t7 + assign17240_e30356);
        (assign17240_e30357, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, (locals.var_t7_dn4 + (locals.var_prt_i * locals.var_devtemp_dn4)), locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn13, locals.var_t7_dn14,)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign17240_e30359;
        locals.var_t8_dn0 = assign17240_e30359_d_n0;
        locals.var_t8_dn2 = assign17240_e30359_d_n2;
        locals.var_t8_dn3 = assign17240_e30359_d_n3;
        locals.var_t8_dn4 = assign17240_e30359_d_n4;
        locals.var_t8_dn5 = assign17240_e30359_d_n5;
        locals.var_t8_dn6 = assign17240_e30359_d_n6;
        locals.var_t8_dn7 = assign17240_e30359_d_n7;
        locals.var_t8_dn8 = assign17240_e30359_d_n8;
        locals.var_t8_dn9 = assign17240_e30359_d_n9;
        locals.var_t8_dn10 = assign17240_e30359_d_n10;
        locals.var_t8_dn11 = assign17240_e30359_d_n11;
        locals.var_t8_dn13 = assign17240_e30359_d_n13;
        locals.var_t8_dn14 = assign17240_e30359_d_n14;

        let (assign17250_e30400, assign17250_e30400_d_n0, assign17250_e30400_d_n2, assign17250_e30400_d_n3, assign17250_e30400_d_n4, assign17250_e30400_d_n5, assign17250_e30400_d_n6, assign17250_e30400_d_n7, assign17250_e30400_d_n8, assign17250_e30400_d_n9, assign17250_e30400_d_n10, assign17250_e30400_d_n11, assign17250_e30400_d_n13, assign17250_e30400_d_n14,) = {
    if (((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 != 0.0)) && (locals.var_guard302 == 0.0)) && (locals.var_guard304 == 0.0)) && (locals.var_guard306 == 0.0)) {
        let assign17250_e30382: f64 = (locals.var_t6 + locals.var_t8);
        let assign17250_e30385: f64 = (locals.var_t6 - locals.var_t8);
        let assign17250_e30388: f64 = (locals.var_t6 - locals.var_t8);
        let assign17250_e30389: f64 = (assign17250_e30385 * assign17250_e30388);
        let assign17250_e30392: f64 = (0.25 * 0.001);
        let assign17250_e30394: f64 = (assign17250_e30392 * 0.001);
        let assign17250_e30395: f64 = (assign17250_e30389 + assign17250_e30394);
        let assign17250_e30396: f64 = (assign17250_e30395).sqrt();
        let assign17250_e30397: f64 = (assign17250_e30382 - assign17250_e30396);
        let assign17250_e30398: f64 = (0.5 * assign17250_e30397);
        (assign17250_e30398, (0.5 * ((locals.var_t6_dn0 + locals.var_t8_dn0) - ((((locals.var_t6_dn0 - locals.var_t8_dn0) * assign17250_e30388) + (assign17250_e30385 * (locals.var_t6_dn0 - locals.var_t8_dn0))) / (2.0 * assign17250_e30396)))), (0.5 * ((locals.var_t6_dn2 + locals.var_t8_dn2) - ((((locals.var_t6_dn2 - locals.var_t8_dn2) * assign17250_e30388) + (assign17250_e30385 * (locals.var_t6_dn2 - locals.var_t8_dn2))) / (2.0 * assign17250_e30396)))), (0.5 * ((locals.var_t6_dn3 + locals.var_t8_dn3) - ((((locals.var_t6_dn3 - locals.var_t8_dn3) * assign17250_e30388) + (assign17250_e30385 * (locals.var_t6_dn3 - locals.var_t8_dn3))) / (2.0 * assign17250_e30396)))), (0.5 * ((locals.var_t6_dn4 + locals.var_t8_dn4) - ((((locals.var_t6_dn4 - locals.var_t8_dn4) * assign17250_e30388) + (assign17250_e30385 * (locals.var_t6_dn4 - locals.var_t8_dn4))) / (2.0 * assign17250_e30396)))), (0.5 * ((locals.var_t6_dn5 + locals.var_t8_dn5) - ((((locals.var_t6_dn5 - locals.var_t8_dn5) * assign17250_e30388) + (assign17250_e30385 * (locals.var_t6_dn5 - locals.var_t8_dn5))) / (2.0 * assign17250_e30396)))), (0.5 * ((locals.var_t6_dn6 + locals.var_t8_dn6) - ((((locals.var_t6_dn6 - locals.var_t8_dn6) * assign17250_e30388) + (assign17250_e30385 * (locals.var_t6_dn6 - locals.var_t8_dn6))) / (2.0 * assign17250_e30396)))), (0.5 * ((locals.var_t6_dn7 + locals.var_t8_dn7) - ((((locals.var_t6_dn7 - locals.var_t8_dn7) * assign17250_e30388) + (assign17250_e30385 * (locals.var_t6_dn7 - locals.var_t8_dn7))) / (2.0 * assign17250_e30396)))), (0.5 * ((locals.var_t6_dn8 + locals.var_t8_dn8) - ((((locals.var_t6_dn8 - locals.var_t8_dn8) * assign17250_e30388) + (assign17250_e30385 * (locals.var_t6_dn8 - locals.var_t8_dn8))) / (2.0 * assign17250_e30396)))), (0.5 * ((locals.var_t6_dn9 + locals.var_t8_dn9) - ((((locals.var_t6_dn9 - locals.var_t8_dn9) * assign17250_e30388) + (assign17250_e30385 * (locals.var_t6_dn9 - locals.var_t8_dn9))) / (2.0 * assign17250_e30396)))), (0.5 * ((locals.var_t6_dn10 + locals.var_t8_dn10) - ((((locals.var_t6_dn10 - locals.var_t8_dn10) * assign17250_e30388) + (assign17250_e30385 * (locals.var_t6_dn10 - locals.var_t8_dn10))) / (2.0 * assign17250_e30396)))), (0.5 * ((locals.var_t6_dn11 + locals.var_t8_dn11) - ((((locals.var_t6_dn11 - locals.var_t8_dn11) * assign17250_e30388) + (assign17250_e30385 * (locals.var_t6_dn11 - locals.var_t8_dn11))) / (2.0 * assign17250_e30396)))), (0.5 * ((locals.var_t6_dn13 + locals.var_t8_dn13) - ((((locals.var_t6_dn13 - locals.var_t8_dn13) * assign17250_e30388) + (assign17250_e30385 * (locals.var_t6_dn13 - locals.var_t8_dn13))) / (2.0 * assign17250_e30396)))), (0.5 * ((locals.var_t6_dn14 + locals.var_t8_dn14) - ((((locals.var_t6_dn14 - locals.var_t8_dn14) * assign17250_e30388) + (assign17250_e30385 * (locals.var_t6_dn14 - locals.var_t8_dn14))) / (2.0 * assign17250_e30396)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign17250_e30400;
        locals.var_t2_dn0 = assign17250_e30400_d_n0;
        locals.var_t2_dn2 = assign17250_e30400_d_n2;
        locals.var_t2_dn3 = assign17250_e30400_d_n3;
        locals.var_t2_dn4 = assign17250_e30400_d_n4;
        locals.var_t2_dn5 = assign17250_e30400_d_n5;
        locals.var_t2_dn6 = assign17250_e30400_d_n6;
        locals.var_t2_dn7 = assign17250_e30400_d_n7;
        locals.var_t2_dn8 = assign17250_e30400_d_n8;
        locals.var_t2_dn9 = assign17250_e30400_d_n9;
        locals.var_t2_dn10 = assign17250_e30400_d_n10;
        locals.var_t2_dn11 = assign17250_e30400_d_n11;
        locals.var_t2_dn13 = assign17250_e30400_d_n13;
        locals.var_t2_dn14 = assign17250_e30400_d_n14;

        let assign17260_e30403: f64 = if locals.var_tnom > 210.0 { 1.0 } else { 0.0 };
        locals.var_guard307 = assign17260_e30403;

        let (assign17270_e30423, assign17270_e30423_d_n4,) = {
    if (((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 == 0.0)) && (locals.var_guard307 != 0.0)) {
        let assign17270_e30420: f64 = (locals.var_prt_i * locals.var_deltemp);
        let assign17270_e30421: f64 = (1.0 + assign17270_e30420);
        (assign17270_e30421, (locals.var_prt_i * locals.var_deltemp_dn4),)
    } else {
        (locals.var_rdstemp0, locals.var_rdstemp0_dn4,)
    }
};
        locals.var_rdstemp0 = assign17270_e30423;
        locals.var_rdstemp0_dn4 = assign17270_e30423_d_n4;

        let (assign17280_e30451, assign17280_e30451_d_n4,) = {
    if (((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 == 0.0)) && (locals.var_guard307 != 0.0)) {
        let assign17280_e30441: f64 = (locals.var_devtemp - 210.0);
        let assign17280_e30442: f64 = (locals.var_prt1_i * assign17280_e30441);
        let assign17280_e30443: f64 = (1.0 + assign17280_e30442);
        let assign17280_e30447: f64 = (210.0 - locals.var_tnom);
        let assign17280_e30448: f64 = (locals.var_prt_i * assign17280_e30447);
        let assign17280_e30449: f64 = (assign17280_e30443 + assign17280_e30448);
        (assign17280_e30449, (locals.var_prt1_i * locals.var_devtemp_dn4),)
    } else {
        (locals.var_rdstemp1, locals.var_rdstemp1_dn4,)
    }
};
        locals.var_rdstemp1 = assign17280_e30451;
        locals.var_rdstemp1_dn4 = assign17280_e30451_d_n4;

        let assign17290_e30454: f64 = if locals.var_prt1_i < locals.var_prt_i { 1.0 } else { 0.0 };
        locals.var_guard308 = assign17290_e30454;

        let (assign17300_e30491, assign17300_e30491_d_n0, assign17300_e30491_d_n2, assign17300_e30491_d_n3, assign17300_e30491_d_n4, assign17300_e30491_d_n5, assign17300_e30491_d_n6, assign17300_e30491_d_n7, assign17300_e30491_d_n8, assign17300_e30491_d_n9, assign17300_e30491_d_n10, assign17300_e30491_d_n11, assign17300_e30491_d_n13, assign17300_e30491_d_n14,) = {
    if ((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 == 0.0)) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 != 0.0)) {
        let assign17300_e30473: f64 = (locals.var_rdstemp0 + locals.var_rdstemp1);
        let assign17300_e30476: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign17300_e30479: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign17300_e30480: f64 = (assign17300_e30476 * assign17300_e30479);
        let assign17300_e30483: f64 = (0.25 * 0.01);
        let assign17300_e30485: f64 = (assign17300_e30483 * 0.01);
        let assign17300_e30486: f64 = (assign17300_e30480 + assign17300_e30485);
        let assign17300_e30487: f64 = (assign17300_e30486).sqrt();
        let assign17300_e30488: f64 = (assign17300_e30473 + assign17300_e30487);
        let assign17300_e30489: f64 = (0.5 * assign17300_e30488);
        (assign17300_e30489, 0.0, 0.0, 0.0, (0.5 * ((locals.var_rdstemp0_dn4 + locals.var_rdstemp1_dn4) + ((((locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4) * assign17300_e30479) + (assign17300_e30476 * (locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4))) / (2.0 * assign17300_e30487)))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign17300_e30491;
        locals.var_t2_dn0 = assign17300_e30491_d_n0;
        locals.var_t2_dn2 = assign17300_e30491_d_n2;
        locals.var_t2_dn3 = assign17300_e30491_d_n3;
        locals.var_t2_dn4 = assign17300_e30491_d_n4;
        locals.var_t2_dn5 = assign17300_e30491_d_n5;
        locals.var_t2_dn6 = assign17300_e30491_d_n6;
        locals.var_t2_dn7 = assign17300_e30491_d_n7;
        locals.var_t2_dn8 = assign17300_e30491_d_n8;
        locals.var_t2_dn9 = assign17300_e30491_d_n9;
        locals.var_t2_dn10 = assign17300_e30491_d_n10;
        locals.var_t2_dn11 = assign17300_e30491_d_n11;
        locals.var_t2_dn13 = assign17300_e30491_d_n13;
        locals.var_t2_dn14 = assign17300_e30491_d_n14;

        let (assign17310_e30529, assign17310_e30529_d_n0, assign17310_e30529_d_n2, assign17310_e30529_d_n3, assign17310_e30529_d_n4, assign17310_e30529_d_n5, assign17310_e30529_d_n6, assign17310_e30529_d_n7, assign17310_e30529_d_n8, assign17310_e30529_d_n9, assign17310_e30529_d_n10, assign17310_e30529_d_n11, assign17310_e30529_d_n13, assign17310_e30529_d_n14,) = {
    if ((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 == 0.0)) && (locals.var_guard307 != 0.0)) && (locals.var_guard308 == 0.0)) {
        let assign17310_e30511: f64 = (locals.var_rdstemp0 + locals.var_rdstemp1);
        let assign17310_e30514: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign17310_e30517: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign17310_e30518: f64 = (assign17310_e30514 * assign17310_e30517);
        let assign17310_e30521: f64 = (0.25 * 0.01);
        let assign17310_e30523: f64 = (assign17310_e30521 * 0.01);
        let assign17310_e30524: f64 = (assign17310_e30518 + assign17310_e30523);
        let assign17310_e30525: f64 = (assign17310_e30524).sqrt();
        let assign17310_e30526: f64 = (assign17310_e30511 - assign17310_e30525);
        let assign17310_e30527: f64 = (0.5 * assign17310_e30526);
        (assign17310_e30527, 0.0, 0.0, 0.0, (0.5 * ((locals.var_rdstemp0_dn4 + locals.var_rdstemp1_dn4) - ((((locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4) * assign17310_e30517) + (assign17310_e30514 * (locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4))) / (2.0 * assign17310_e30525)))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign17310_e30529;
        locals.var_t2_dn0 = assign17310_e30529_d_n0;
        locals.var_t2_dn2 = assign17310_e30529_d_n2;
        locals.var_t2_dn3 = assign17310_e30529_d_n3;
        locals.var_t2_dn4 = assign17310_e30529_d_n4;
        locals.var_t2_dn5 = assign17310_e30529_d_n5;
        locals.var_t2_dn6 = assign17310_e30529_d_n6;
        locals.var_t2_dn7 = assign17310_e30529_d_n7;
        locals.var_t2_dn8 = assign17310_e30529_d_n8;
        locals.var_t2_dn9 = assign17310_e30529_d_n9;
        locals.var_t2_dn10 = assign17310_e30529_d_n10;
        locals.var_t2_dn11 = assign17310_e30529_d_n11;
        locals.var_t2_dn13 = assign17310_e30529_d_n13;
        locals.var_t2_dn14 = assign17310_e30529_d_n14;

        let (assign17320_e30550, assign17320_e30550_d_n4,) = {
    if (((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 == 0.0)) && (locals.var_guard307 == 0.0)) {
        let assign17320_e30547: f64 = (locals.var_prt1_i * locals.var_deltemp);
        let assign17320_e30548: f64 = (1.0 + assign17320_e30547);
        (assign17320_e30548, (locals.var_prt1_i * locals.var_deltemp_dn4),)
    } else {
        (locals.var_rdstemp1, locals.var_rdstemp1_dn4,)
    }
};
        locals.var_rdstemp1 = assign17320_e30550;
        locals.var_rdstemp1_dn4 = assign17320_e30550_d_n4;

        let (assign17330_e30579, assign17330_e30579_d_n4,) = {
    if (((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 == 0.0)) && (locals.var_guard307 == 0.0)) {
        let assign17330_e30569: f64 = (locals.var_devtemp - 210.0);
        let assign17330_e30570: f64 = (locals.var_prt_i * assign17330_e30569);
        let assign17330_e30571: f64 = (1.0 + assign17330_e30570);
        let assign17330_e30575: f64 = (210.0 - locals.var_tnom);
        let assign17330_e30576: f64 = (locals.var_prt1_i * assign17330_e30575);
        let assign17330_e30577: f64 = (assign17330_e30571 + assign17330_e30576);
        (assign17330_e30577, (locals.var_prt_i * locals.var_devtemp_dn4),)
    } else {
        (locals.var_rdstemp0, locals.var_rdstemp0_dn4,)
    }
};
        locals.var_rdstemp0 = assign17330_e30579;
        locals.var_rdstemp0_dn4 = assign17330_e30579_d_n4;

        let assign17340_e30582: f64 = if locals.var_prt1_i < locals.var_prt_i { 1.0 } else { 0.0 };
        locals.var_guard309 = assign17340_e30582;

        let (assign17350_e30620, assign17350_e30620_d_n0, assign17350_e30620_d_n2, assign17350_e30620_d_n3, assign17350_e30620_d_n4, assign17350_e30620_d_n5, assign17350_e30620_d_n6, assign17350_e30620_d_n7, assign17350_e30620_d_n8, assign17350_e30620_d_n9, assign17350_e30620_d_n10, assign17350_e30620_d_n11, assign17350_e30620_d_n13, assign17350_e30620_d_n14,) = {
    if ((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 == 0.0)) && (locals.var_guard307 == 0.0)) && (locals.var_guard309 != 0.0)) {
        let assign17350_e30602: f64 = (locals.var_rdstemp0 + locals.var_rdstemp1);
        let assign17350_e30605: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign17350_e30608: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign17350_e30609: f64 = (assign17350_e30605 * assign17350_e30608);
        let assign17350_e30612: f64 = (0.25 * 0.01);
        let assign17350_e30614: f64 = (assign17350_e30612 * 0.01);
        let assign17350_e30615: f64 = (assign17350_e30609 + assign17350_e30614);
        let assign17350_e30616: f64 = (assign17350_e30615).sqrt();
        let assign17350_e30617: f64 = (assign17350_e30602 + assign17350_e30616);
        let assign17350_e30618: f64 = (0.5 * assign17350_e30617);
        (assign17350_e30618, 0.0, 0.0, 0.0, (0.5 * ((locals.var_rdstemp0_dn4 + locals.var_rdstemp1_dn4) + ((((locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4) * assign17350_e30608) + (assign17350_e30605 * (locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4))) / (2.0 * assign17350_e30616)))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign17350_e30620;
        locals.var_t2_dn0 = assign17350_e30620_d_n0;
        locals.var_t2_dn2 = assign17350_e30620_d_n2;
        locals.var_t2_dn3 = assign17350_e30620_d_n3;
        locals.var_t2_dn4 = assign17350_e30620_d_n4;
        locals.var_t2_dn5 = assign17350_e30620_d_n5;
        locals.var_t2_dn6 = assign17350_e30620_d_n6;
        locals.var_t2_dn7 = assign17350_e30620_d_n7;
        locals.var_t2_dn8 = assign17350_e30620_d_n8;
        locals.var_t2_dn9 = assign17350_e30620_d_n9;
        locals.var_t2_dn10 = assign17350_e30620_d_n10;
        locals.var_t2_dn11 = assign17350_e30620_d_n11;
        locals.var_t2_dn13 = assign17350_e30620_d_n13;
        locals.var_t2_dn14 = assign17350_e30620_d_n14;

        let (assign17360_e30659, assign17360_e30659_d_n0, assign17360_e30659_d_n2, assign17360_e30659_d_n3, assign17360_e30659_d_n4, assign17360_e30659_d_n5, assign17360_e30659_d_n6, assign17360_e30659_d_n7, assign17360_e30659_d_n8, assign17360_e30659_d_n9, assign17360_e30659_d_n10, assign17360_e30659_d_n11, assign17360_e30659_d_n13, assign17360_e30659_d_n14,) = {
    if ((((((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) && (locals.var_guard300 == 0.0)) && (locals.var_guard301 == 0.0)) && (locals.var_guard307 == 0.0)) && (locals.var_guard309 == 0.0)) {
        let assign17360_e30641: f64 = (locals.var_rdstemp0 + locals.var_rdstemp1);
        let assign17360_e30644: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign17360_e30647: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign17360_e30648: f64 = (assign17360_e30644 * assign17360_e30647);
        let assign17360_e30651: f64 = (0.25 * 0.01);
        let assign17360_e30653: f64 = (assign17360_e30651 * 0.01);
        let assign17360_e30654: f64 = (assign17360_e30648 + assign17360_e30653);
        let assign17360_e30655: f64 = (assign17360_e30654).sqrt();
        let assign17360_e30656: f64 = (assign17360_e30641 - assign17360_e30655);
        let assign17360_e30657: f64 = (0.5 * assign17360_e30656);
        (assign17360_e30657, 0.0, 0.0, 0.0, (0.5 * ((locals.var_rdstemp0_dn4 + locals.var_rdstemp1_dn4) - ((((locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4) * assign17360_e30647) + (assign17360_e30644 * (locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4))) / (2.0 * assign17360_e30655)))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign17360_e30659;
        locals.var_t2_dn0 = assign17360_e30659_d_n0;
        locals.var_t2_dn2 = assign17360_e30659_d_n2;
        locals.var_t2_dn3 = assign17360_e30659_d_n3;
        locals.var_t2_dn4 = assign17360_e30659_d_n4;
        locals.var_t2_dn5 = assign17360_e30659_d_n5;
        locals.var_t2_dn6 = assign17360_e30659_d_n6;
        locals.var_t2_dn7 = assign17360_e30659_d_n7;
        locals.var_t2_dn8 = assign17360_e30659_d_n8;
        locals.var_t2_dn9 = assign17360_e30659_d_n9;
        locals.var_t2_dn10 = assign17360_e30659_d_n10;
        locals.var_t2_dn11 = assign17360_e30659_d_n11;
        locals.var_t2_dn13 = assign17360_e30659_d_n13;
        locals.var_t2_dn14 = assign17360_e30659_d_n14;

    }

    pub(super) fn stamp_transient_block_56(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17370_e30714, assign17370_e30714_d_n0, assign17370_e30714_d_n2, assign17370_e30714_d_n3, assign17370_e30714_d_n4, assign17370_e30714_d_n5, assign17370_e30714_d_n6, assign17370_e30714_d_n7, assign17370_e30714_d_n8, assign17370_e30714_d_n9, assign17370_e30714_d_n10, assign17370_e30714_d_n11, assign17370_e30714_d_n13, assign17370_e30714_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) {
        let assign17370_e30667: f64 = (locals.var_t2 - 1e-6);
        let assign17370_e30669: f64 = (-10000.0);
        let assign17370_e30671: f64 = (assign17370_e30669 * 0.001);
        let (assign17370_e30712, assign17370_e30712_d_n0, assign17370_e30712_d_n2, assign17370_e30712_d_n3, assign17370_e30712_d_n4, assign17370_e30712_d_n5, assign17370_e30712_d_n6, assign17370_e30712_d_n7, assign17370_e30712_d_n8, assign17370_e30712_d_n9, assign17370_e30712_d_n10, assign17370_e30712_d_n11, assign17370_e30712_d_n13, assign17370_e30712_d_n14,) = {
            if (!(assign17370_e30667 < assign17370_e30671)) {
                let assign17370_e30677: f64 = (locals.var_t2 - 1e-6);
                let assign17370_e30680: f64 = (locals.var_t2 - 1e-6);
                let assign17370_e30683: f64 = (locals.var_t2 - 1e-6);
                let assign17370_e30684: f64 = (assign17370_e30680 * assign17370_e30683);
                let assign17370_e30687: f64 = (4.0 * 0.001);
                let assign17370_e30689: f64 = (assign17370_e30687 * 0.001);
                let assign17370_e30690: f64 = (assign17370_e30684 + assign17370_e30689);
                let assign17370_e30691: f64 = (assign17370_e30690).sqrt();
                let assign17370_e30692: f64 = (assign17370_e30677 + assign17370_e30691);
                let assign17370_e30693: f64 = (0.5 * assign17370_e30692);
                (assign17370_e30693, (0.5 * (locals.var_t2_dn0 + (((locals.var_t2_dn0 * assign17370_e30683) + (assign17370_e30680 * locals.var_t2_dn0)) / (2.0 * assign17370_e30691)))), (0.5 * (locals.var_t2_dn2 + (((locals.var_t2_dn2 * assign17370_e30683) + (assign17370_e30680 * locals.var_t2_dn2)) / (2.0 * assign17370_e30691)))), (0.5 * (locals.var_t2_dn3 + (((locals.var_t2_dn3 * assign17370_e30683) + (assign17370_e30680 * locals.var_t2_dn3)) / (2.0 * assign17370_e30691)))), (0.5 * (locals.var_t2_dn4 + (((locals.var_t2_dn4 * assign17370_e30683) + (assign17370_e30680 * locals.var_t2_dn4)) / (2.0 * assign17370_e30691)))), (0.5 * (locals.var_t2_dn5 + (((locals.var_t2_dn5 * assign17370_e30683) + (assign17370_e30680 * locals.var_t2_dn5)) / (2.0 * assign17370_e30691)))), (0.5 * (locals.var_t2_dn6 + (((locals.var_t2_dn6 * assign17370_e30683) + (assign17370_e30680 * locals.var_t2_dn6)) / (2.0 * assign17370_e30691)))), (0.5 * (locals.var_t2_dn7 + (((locals.var_t2_dn7 * assign17370_e30683) + (assign17370_e30680 * locals.var_t2_dn7)) / (2.0 * assign17370_e30691)))), (0.5 * (locals.var_t2_dn8 + (((locals.var_t2_dn8 * assign17370_e30683) + (assign17370_e30680 * locals.var_t2_dn8)) / (2.0 * assign17370_e30691)))), (0.5 * (locals.var_t2_dn9 + (((locals.var_t2_dn9 * assign17370_e30683) + (assign17370_e30680 * locals.var_t2_dn9)) / (2.0 * assign17370_e30691)))), (0.5 * (locals.var_t2_dn10 + (((locals.var_t2_dn10 * assign17370_e30683) + (assign17370_e30680 * locals.var_t2_dn10)) / (2.0 * assign17370_e30691)))), (0.5 * (locals.var_t2_dn11 + (((locals.var_t2_dn11 * assign17370_e30683) + (assign17370_e30680 * locals.var_t2_dn11)) / (2.0 * assign17370_e30691)))), (0.5 * (locals.var_t2_dn13 + (((locals.var_t2_dn13 * assign17370_e30683) + (assign17370_e30680 * locals.var_t2_dn13)) / (2.0 * assign17370_e30691)))), (0.5 * (locals.var_t2_dn14 + (((locals.var_t2_dn14 * assign17370_e30683) + (assign17370_e30680 * locals.var_t2_dn14)) / (2.0 * assign17370_e30691)))),)
            } else {
                let assign17370_e30696: f64 = (locals.var_t2 - 1e-6);
                let assign17370_e30698: f64 = (-10000.0);
                let assign17370_e30700: f64 = (assign17370_e30698 * 0.001);
                let (assign17370_e30711, assign17370_e30711_d_n0, assign17370_e30711_d_n2, assign17370_e30711_d_n3, assign17370_e30711_d_n4, assign17370_e30711_d_n5, assign17370_e30711_d_n6, assign17370_e30711_d_n7, assign17370_e30711_d_n8, assign17370_e30711_d_n9, assign17370_e30711_d_n10, assign17370_e30711_d_n11, assign17370_e30711_d_n13, assign17370_e30711_d_n14,) = {
                    if (assign17370_e30696 < assign17370_e30700) {
                        let assign17370_e30703: f64 = (-0.001);
                        let assign17370_e30705: f64 = (assign17370_e30703 * 0.001);
                        let assign17370_e30708: f64 = (locals.var_t2 - 1e-6);
                        let assign17370_e30709: f64 = (assign17370_e30705 / assign17370_e30708);
                        (assign17370_e30709, (-((assign17370_e30705 * locals.var_t2_dn0) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * locals.var_t2_dn2) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * locals.var_t2_dn3) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * locals.var_t2_dn4) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * locals.var_t2_dn5) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * locals.var_t2_dn6) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * locals.var_t2_dn7) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * locals.var_t2_dn8) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * locals.var_t2_dn9) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * locals.var_t2_dn10) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * locals.var_t2_dn11) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * locals.var_t2_dn13) / (assign17370_e30708 * assign17370_e30708))), (-((assign17370_e30705 * locals.var_t2_dn14) / (assign17370_e30708 * assign17370_e30708))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign17370_e30711, assign17370_e30711_d_n0, assign17370_e30711_d_n2, assign17370_e30711_d_n3, assign17370_e30711_d_n4, assign17370_e30711_d_n5, assign17370_e30711_d_n6, assign17370_e30711_d_n7, assign17370_e30711_d_n8, assign17370_e30711_d_n9, assign17370_e30711_d_n10, assign17370_e30711_d_n11, assign17370_e30711_d_n13, assign17370_e30711_d_n14,)
            }
        };
        (assign17370_e30712, assign17370_e30712_d_n0, assign17370_e30712_d_n2, assign17370_e30712_d_n3, assign17370_e30712_d_n4, assign17370_e30712_d_n5, assign17370_e30712_d_n6, assign17370_e30712_d_n7, assign17370_e30712_d_n8, assign17370_e30712_d_n9, assign17370_e30712_d_n10, assign17370_e30712_d_n11, assign17370_e30712_d_n13, assign17370_e30712_d_n14,)
    } else {
        (locals.var_rdstemp, locals.var_rdstemp_dn0, locals.var_rdstemp_dn2, locals.var_rdstemp_dn3, locals.var_rdstemp_dn4, locals.var_rdstemp_dn5, locals.var_rdstemp_dn6, locals.var_rdstemp_dn7, locals.var_rdstemp_dn8, locals.var_rdstemp_dn9, locals.var_rdstemp_dn10, locals.var_rdstemp_dn11, locals.var_rdstemp_dn13, locals.var_rdstemp_dn14,)
    }
};
        locals.var_rdstemp = assign17370_e30714;
        locals.var_rdstemp_dn0 = assign17370_e30714_d_n0;
        locals.var_rdstemp_dn2 = assign17370_e30714_d_n2;
        locals.var_rdstemp_dn3 = assign17370_e30714_d_n3;
        locals.var_rdstemp_dn4 = assign17370_e30714_d_n4;
        locals.var_rdstemp_dn5 = assign17370_e30714_d_n5;
        locals.var_rdstemp_dn6 = assign17370_e30714_d_n6;
        locals.var_rdstemp_dn7 = assign17370_e30714_d_n7;
        locals.var_rdstemp_dn8 = assign17370_e30714_d_n8;
        locals.var_rdstemp_dn9 = assign17370_e30714_d_n9;
        locals.var_rdstemp_dn10 = assign17370_e30714_d_n10;
        locals.var_rdstemp_dn11 = assign17370_e30714_d_n11;
        locals.var_rdstemp_dn13 = assign17370_e30714_d_n13;
        locals.var_rdstemp_dn14 = assign17370_e30714_d_n14;

        let (assign17380_e30741, assign17380_e30741_d_n0, assign17380_e30741_d_n2, assign17380_e30741_d_n3, assign17380_e30741_d_n4, assign17380_e30741_d_n5, assign17380_e30741_d_n6, assign17380_e30741_d_n7, assign17380_e30741_d_n8, assign17380_e30741_d_n9, assign17380_e30741_d_n10, assign17380_e30741_d_n11, assign17380_e30741_d_n13, assign17380_e30741_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) {
        let assign17380_e30723: f64 = (locals.var_tnom + 210.0);
        let assign17380_e30726: f64 = (locals.var_tnom - 210.0);
        let assign17380_e30729: f64 = (locals.var_tnom - 210.0);
        let assign17380_e30730: f64 = (assign17380_e30726 * assign17380_e30729);
        let assign17380_e30733: f64 = (0.25 * 0.2);
        let assign17380_e30735: f64 = (assign17380_e30733 * 0.2);
        let assign17380_e30736: f64 = (assign17380_e30730 + assign17380_e30735);
        let assign17380_e30737: f64 = (assign17380_e30736).sqrt();
        let assign17380_e30738: f64 = (assign17380_e30723 - assign17380_e30737);
        let assign17380_e30739: f64 = (0.5 * assign17380_e30738);
        (assign17380_e30739, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign17380_e30741;
        locals.var_t4_dn0 = assign17380_e30741_d_n0;
        locals.var_t4_dn2 = assign17380_e30741_d_n2;
        locals.var_t4_dn3 = assign17380_e30741_d_n3;
        locals.var_t4_dn4 = assign17380_e30741_d_n4;
        locals.var_t4_dn5 = assign17380_e30741_d_n5;
        locals.var_t4_dn6 = assign17380_e30741_d_n6;
        locals.var_t4_dn7 = assign17380_e30741_d_n7;
        locals.var_t4_dn8 = assign17380_e30741_d_n8;
        locals.var_t4_dn9 = assign17380_e30741_d_n9;
        locals.var_t4_dn10 = assign17380_e30741_d_n10;
        locals.var_t4_dn11 = assign17380_e30741_d_n11;
        locals.var_t4_dn13 = assign17380_e30741_d_n13;
        locals.var_t4_dn14 = assign17380_e30741_d_n14;

        let (assign17390_e30777, assign17390_e30777_d_n0, assign17390_e30777_d_n2, assign17390_e30777_d_n3, assign17390_e30777_d_n4, assign17390_e30777_d_n5, assign17390_e30777_d_n6, assign17390_e30777_d_n7, assign17390_e30777_d_n8, assign17390_e30777_d_n9, assign17390_e30777_d_n10, assign17390_e30777_d_n11, assign17390_e30777_d_n13, assign17390_e30777_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 == 0.0)) {
        let assign17390_e30750: f64 = (p.p1720 / locals.var_leff_1);
        let assign17390_e30751: f64 = (locals.var_kt1_i + assign17390_e30750);
        let assign17390_e30753: f64 = (assign17390_e30751 * locals.var_tratio_m1);
        let assign17390_e30759: f64 = (locals.var_devtemp1 - p.p1749);
        let assign17390_e30760: f64 = (p.p1748 * assign17390_e30759);
        let assign17390_e30761: f64 = { let limited_exp_arg = assign17390_e30760; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign17390_e30762: f64 = (1.0 + assign17390_e30761);
        let assign17390_e30763: f64 = (p.p1747 / assign17390_e30762);
        let assign17390_e30764: f64 = (assign17390_e30753 + assign17390_e30763);
        let assign17390_e30770: f64 = (locals.var_t4 - p.p1749);
        let assign17390_e30771: f64 = (p.p1748 * assign17390_e30770);
        let assign17390_e30772: f64 = { let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign17390_e30773: f64 = (1.0 + assign17390_e30772);
        let assign17390_e30774: f64 = (p.p1747 / assign17390_e30773);
        let assign17390_e30775: f64 = (assign17390_e30764 - assign17390_e30774);
        (assign17390_e30775, (((-((p.p1720 * locals.var_leff_1_dn0) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * locals.var_t4_dn0))) / (assign17390_e30773 * assign17390_e30773)))), (((-((p.p1720 * locals.var_leff_1_dn2) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * locals.var_t4_dn2))) / (assign17390_e30773 * assign17390_e30773)))), (((-((p.p1720 * locals.var_leff_1_dn3) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * locals.var_t4_dn3))) / (assign17390_e30773 * assign17390_e30773)))), (((((-((p.p1720 * locals.var_leff_1_dn4) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1) + (assign17390_e30751 * locals.var_tratio_m1_dn4)) + (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30760; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * locals.var_devtemp1_dn4))) / (assign17390_e30762 * assign17390_e30762)))) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * locals.var_t4_dn4))) / (assign17390_e30773 * assign17390_e30773)))), (((-((p.p1720 * locals.var_leff_1_dn5) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * locals.var_t4_dn5))) / (assign17390_e30773 * assign17390_e30773)))), (((-((p.p1720 * locals.var_leff_1_dn6) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * locals.var_t4_dn6))) / (assign17390_e30773 * assign17390_e30773)))), (((-((p.p1720 * locals.var_leff_1_dn7) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * locals.var_t4_dn7))) / (assign17390_e30773 * assign17390_e30773)))), (((-((p.p1720 * locals.var_leff_1_dn8) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * locals.var_t4_dn8))) / (assign17390_e30773 * assign17390_e30773)))), (((-((p.p1720 * locals.var_leff_1_dn9) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * locals.var_t4_dn9))) / (assign17390_e30773 * assign17390_e30773)))), (((-((p.p1720 * locals.var_leff_1_dn10) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * locals.var_t4_dn10))) / (assign17390_e30773 * assign17390_e30773)))), (((-((p.p1720 * locals.var_leff_1_dn11) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * locals.var_t4_dn11))) / (assign17390_e30773 * assign17390_e30773)))), (((-((p.p1720 * locals.var_leff_1_dn13) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * locals.var_t4_dn13))) / (assign17390_e30773 * assign17390_e30773)))), (((-((p.p1720 * locals.var_leff_1_dn14) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1) - (-((p.p1747 * ({ let limited_exp_arg = assign17390_e30771; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p1748 * locals.var_t4_dn14))) / (assign17390_e30773 * assign17390_e30773)))),)
    } else {
        (locals.var_dvth_temp, locals.var_dvth_temp_dn0, locals.var_dvth_temp_dn2, locals.var_dvth_temp_dn3, locals.var_dvth_temp_dn4, locals.var_dvth_temp_dn5, locals.var_dvth_temp_dn6, locals.var_dvth_temp_dn7, locals.var_dvth_temp_dn8, locals.var_dvth_temp_dn9, locals.var_dvth_temp_dn10, locals.var_dvth_temp_dn11, locals.var_dvth_temp_dn13, locals.var_dvth_temp_dn14,)
    }
};
        locals.var_dvth_temp = assign17390_e30777;
        locals.var_dvth_temp_dn0 = assign17390_e30777_d_n0;
        locals.var_dvth_temp_dn2 = assign17390_e30777_d_n2;
        locals.var_dvth_temp_dn3 = assign17390_e30777_d_n3;
        locals.var_dvth_temp_dn4 = assign17390_e30777_d_n4;
        locals.var_dvth_temp_dn5 = assign17390_e30777_d_n5;
        locals.var_dvth_temp_dn6 = assign17390_e30777_d_n6;
        locals.var_dvth_temp_dn7 = assign17390_e30777_d_n7;
        locals.var_dvth_temp_dn8 = assign17390_e30777_d_n8;
        locals.var_dvth_temp_dn9 = assign17390_e30777_d_n9;
        locals.var_dvth_temp_dn10 = assign17390_e30777_d_n10;
        locals.var_dvth_temp_dn11 = assign17390_e30777_d_n11;
        locals.var_dvth_temp_dn13 = assign17390_e30777_d_n13;
        locals.var_dvth_temp_dn14 = assign17390_e30777_d_n14;

        let assign17400_e30780: f64 = if locals.var_vsat_t < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard310 = assign17400_e30780;

        let (assign17410_e30784, assign17410_e30784_d_n0, assign17410_e30784_d_n2, assign17410_e30784_d_n3, assign17410_e30784_d_n4, assign17410_e30784_d_n5, assign17410_e30784_d_n6, assign17410_e30784_d_n7, assign17410_e30784_d_n8, assign17410_e30784_d_n9, assign17410_e30784_d_n10, assign17410_e30784_d_n11, assign17410_e30784_d_n13, assign17410_e30784_d_n14,) = {
    if (locals.var_guard310 != 0.0) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14,)
    }
};
        locals.var_vsat_t = assign17410_e30784;
        locals.var_vsat_t_dn0 = assign17410_e30784_d_n0;
        locals.var_vsat_t_dn2 = assign17410_e30784_d_n2;
        locals.var_vsat_t_dn3 = assign17410_e30784_d_n3;
        locals.var_vsat_t_dn4 = assign17410_e30784_d_n4;
        locals.var_vsat_t_dn5 = assign17410_e30784_d_n5;
        locals.var_vsat_t_dn6 = assign17410_e30784_d_n6;
        locals.var_vsat_t_dn7 = assign17410_e30784_d_n7;
        locals.var_vsat_t_dn8 = assign17410_e30784_d_n8;
        locals.var_vsat_t_dn9 = assign17410_e30784_d_n9;
        locals.var_vsat_t_dn10 = assign17410_e30784_d_n10;
        locals.var_vsat_t_dn11 = assign17410_e30784_d_n11;
        locals.var_vsat_t_dn13 = assign17410_e30784_d_n13;
        locals.var_vsat_t_dn14 = assign17410_e30784_d_n14;

        let assign17420_e30787: f64 = if locals.var_vsat1_t < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard311 = assign17420_e30787;

        let (assign17430_e30791, assign17430_e30791_d_n0, assign17430_e30791_d_n2, assign17430_e30791_d_n3, assign17430_e30791_d_n4, assign17430_e30791_d_n5, assign17430_e30791_d_n6, assign17430_e30791_d_n7, assign17430_e30791_d_n8, assign17430_e30791_d_n9, assign17430_e30791_d_n10, assign17430_e30791_d_n11, assign17430_e30791_d_n13, assign17430_e30791_d_n14,) = {
    if (locals.var_guard311 != 0.0) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsat1_t, locals.var_vsat1_t_dn0, locals.var_vsat1_t_dn2, locals.var_vsat1_t_dn3, locals.var_vsat1_t_dn4, locals.var_vsat1_t_dn5, locals.var_vsat1_t_dn6, locals.var_vsat1_t_dn7, locals.var_vsat1_t_dn8, locals.var_vsat1_t_dn9, locals.var_vsat1_t_dn10, locals.var_vsat1_t_dn11, locals.var_vsat1_t_dn13, locals.var_vsat1_t_dn14,)
    }
};
        locals.var_vsat1_t = assign17430_e30791;
        locals.var_vsat1_t_dn0 = assign17430_e30791_d_n0;
        locals.var_vsat1_t_dn2 = assign17430_e30791_d_n2;
        locals.var_vsat1_t_dn3 = assign17430_e30791_d_n3;
        locals.var_vsat1_t_dn4 = assign17430_e30791_d_n4;
        locals.var_vsat1_t_dn5 = assign17430_e30791_d_n5;
        locals.var_vsat1_t_dn6 = assign17430_e30791_d_n6;
        locals.var_vsat1_t_dn7 = assign17430_e30791_d_n7;
        locals.var_vsat1_t_dn8 = assign17430_e30791_d_n8;
        locals.var_vsat1_t_dn9 = assign17430_e30791_d_n9;
        locals.var_vsat1_t_dn10 = assign17430_e30791_d_n10;
        locals.var_vsat1_t_dn11 = assign17430_e30791_d_n11;
        locals.var_vsat1_t_dn13 = assign17430_e30791_d_n13;
        locals.var_vsat1_t_dn14 = assign17430_e30791_d_n14;

        let assign17440_e30794: f64 = if locals.var_vsatcv_t < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard312 = assign17440_e30794;

        let (assign17450_e30798, assign17450_e30798_d_n0, assign17450_e30798_d_n2, assign17450_e30798_d_n3, assign17450_e30798_d_n4, assign17450_e30798_d_n5, assign17450_e30798_d_n6, assign17450_e30798_d_n7, assign17450_e30798_d_n8, assign17450_e30798_d_n9, assign17450_e30798_d_n10, assign17450_e30798_d_n11, assign17450_e30798_d_n13, assign17450_e30798_d_n14,) = {
    if (locals.var_guard312 != 0.0) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsatcv_t, locals.var_vsatcv_t_dn0, locals.var_vsatcv_t_dn2, locals.var_vsatcv_t_dn3, locals.var_vsatcv_t_dn4, locals.var_vsatcv_t_dn5, locals.var_vsatcv_t_dn6, locals.var_vsatcv_t_dn7, locals.var_vsatcv_t_dn8, locals.var_vsatcv_t_dn9, locals.var_vsatcv_t_dn10, locals.var_vsatcv_t_dn11, locals.var_vsatcv_t_dn13, locals.var_vsatcv_t_dn14,)
    }
};
        locals.var_vsatcv_t = assign17450_e30798;
        locals.var_vsatcv_t_dn0 = assign17450_e30798_d_n0;
        locals.var_vsatcv_t_dn2 = assign17450_e30798_d_n2;
        locals.var_vsatcv_t_dn3 = assign17450_e30798_d_n3;
        locals.var_vsatcv_t_dn4 = assign17450_e30798_d_n4;
        locals.var_vsatcv_t_dn5 = assign17450_e30798_d_n5;
        locals.var_vsatcv_t_dn6 = assign17450_e30798_d_n6;
        locals.var_vsatcv_t_dn7 = assign17450_e30798_d_n7;
        locals.var_vsatcv_t_dn8 = assign17450_e30798_d_n8;
        locals.var_vsatcv_t_dn9 = assign17450_e30798_d_n9;
        locals.var_vsatcv_t_dn10 = assign17450_e30798_d_n10;
        locals.var_vsatcv_t_dn11 = assign17450_e30798_d_n11;
        locals.var_vsatcv_t_dn13 = assign17450_e30798_d_n13;
        locals.var_vsatcv_t_dn14 = assign17450_e30798_d_n14;

        let assign17460_e30801: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard313 = assign17460_e30801;

        let assign17470_e30804: f64 = if p.p75 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard314 = assign17470_e30804;

        let assign17480_e30807: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard315 = assign17480_e30807;

        let (assign17490_e30855, assign17490_e30855_d_n4,) = {
    if (((locals.var_guard313 != 0.0) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 != 0.0)) {
        let assign17490_e30815: f64 = (-locals.var_uc_i);
        let assign17490_e30819: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign17490_e30821: f64 = (-locals.var_uc_i);
        let assign17490_e30822: f64 = (assign17490_e30819 - assign17490_e30821);
        let assign17490_e30824: f64 = (assign17490_e30822 - 1e-6);
        let assign17490_e30827: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign17490_e30829: f64 = (-locals.var_uc_i);
        let assign17490_e30830: f64 = (assign17490_e30827 - assign17490_e30829);
        let assign17490_e30832: f64 = (assign17490_e30830 - 1e-6);
        let assign17490_e30835: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign17490_e30837: f64 = (-locals.var_uc_i);
        let assign17490_e30838: f64 = (assign17490_e30835 - assign17490_e30837);
        let assign17490_e30840: f64 = (assign17490_e30838 - 1e-6);
        let assign17490_e30841: f64 = (assign17490_e30832 * assign17490_e30840);
        let assign17490_e30844: f64 = (-locals.var_uc_i);
        let assign17490_e30845: f64 = (4.0 * assign17490_e30844);
        let assign17490_e30847: f64 = (assign17490_e30845 * 1e-6);
        let assign17490_e30848: f64 = (assign17490_e30841 - assign17490_e30847);
        let assign17490_e30849: f64 = (assign17490_e30848).sqrt();
        let assign17490_e30850: f64 = (assign17490_e30824 + assign17490_e30849);
        let assign17490_e30851: f64 = (0.5 * assign17490_e30850);
        let assign17490_e30852: f64 = (assign17490_e30815 + assign17490_e30851);
        let assign17490_e30853: f64 = (locals.var_uc_i + assign17490_e30852);
        (assign17490_e30853, (0.5 * ((locals.var_uc1_i * locals.var_deltemp_dn4) + ((((locals.var_uc1_i * locals.var_deltemp_dn4) * assign17490_e30840) + (assign17490_e30832 * (locals.var_uc1_i * locals.var_deltemp_dn4))) / (2.0 * assign17490_e30849)))),)
    } else {
        (locals.var_uc_t, locals.var_uc_t_dn4,)
    }
};
        locals.var_uc_t = assign17490_e30855;
        locals.var_uc_t_dn4 = assign17490_e30855_d_n4;

        let (assign17500_e30937, assign17500_e30937_d_n4,) = {
    if (((locals.var_guard313 != 0.0) && (locals.var_guard314 != 0.0)) && (locals.var_guard315 == 0.0)) {
        let assign17500_e30866: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign17500_e30867: f64 = (1.0 + assign17500_e30866);
        let assign17500_e30869: f64 = (assign17500_e30867 - 1e-6);
        let assign17500_e30871: f64 = (-10000.0);
        let assign17500_e30873: f64 = (assign17500_e30871 * 0.001);
        let (assign17500_e30934, assign17500_e30934_d_n4,) = {
            if (!(assign17500_e30869 < assign17500_e30873)) {
                let assign17500_e30880: f64 = (locals.var_uc1_i * locals.var_deltemp);
                let assign17500_e30881: f64 = (1.0 + assign17500_e30880);
                let assign17500_e30883: f64 = (assign17500_e30881 - 1e-6);
                let assign17500_e30887: f64 = (locals.var_uc1_i * locals.var_deltemp);
                let assign17500_e30888: f64 = (1.0 + assign17500_e30887);
                let assign17500_e30890: f64 = (assign17500_e30888 - 1e-6);
                let assign17500_e30894: f64 = (locals.var_uc1_i * locals.var_deltemp);
                let assign17500_e30895: f64 = (1.0 + assign17500_e30894);
                let assign17500_e30897: f64 = (assign17500_e30895 - 1e-6);
                let assign17500_e30898: f64 = (assign17500_e30890 * assign17500_e30897);
                let assign17500_e30901: f64 = (4.0 * 0.001);
                let assign17500_e30903: f64 = (assign17500_e30901 * 0.001);
                let assign17500_e30904: f64 = (assign17500_e30898 + assign17500_e30903);
                let assign17500_e30905: f64 = (assign17500_e30904).sqrt();
                let assign17500_e30906: f64 = (assign17500_e30883 + assign17500_e30905);
                let assign17500_e30907: f64 = (0.5 * assign17500_e30906);
                (assign17500_e30907, (0.5 * ((locals.var_uc1_i * locals.var_deltemp_dn4) + ((((locals.var_uc1_i * locals.var_deltemp_dn4) * assign17500_e30897) + (assign17500_e30890 * (locals.var_uc1_i * locals.var_deltemp_dn4))) / (2.0 * assign17500_e30905)))),)
            } else {
                let assign17500_e30911: f64 = (locals.var_uc1_i * locals.var_deltemp);
                let assign17500_e30912: f64 = (1.0 + assign17500_e30911);
                let assign17500_e30914: f64 = (assign17500_e30912 - 1e-6);
                let assign17500_e30916: f64 = (-10000.0);
                let assign17500_e30918: f64 = (assign17500_e30916 * 0.001);
                let (assign17500_e30933, assign17500_e30933_d_n4,) = {
                    if (assign17500_e30914 < assign17500_e30918) {
                        let assign17500_e30921: f64 = (-0.001);
                        let assign17500_e30923: f64 = (assign17500_e30921 * 0.001);
                        let assign17500_e30927: f64 = (locals.var_uc1_i * locals.var_deltemp);
                        let assign17500_e30928: f64 = (1.0 + assign17500_e30927);
                        let assign17500_e30930: f64 = (assign17500_e30928 - 1e-6);
                        let assign17500_e30931: f64 = (assign17500_e30923 / assign17500_e30930);
                        (assign17500_e30931, (-((assign17500_e30923 * (locals.var_uc1_i * locals.var_deltemp_dn4)) / (assign17500_e30930 * assign17500_e30930))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17500_e30933, assign17500_e30933_d_n4,)
            }
        };
        let assign17500_e30935: f64 = (locals.var_uc_i * assign17500_e30934);
        (assign17500_e30935, (locals.var_uc_i * assign17500_e30934_d_n4),)
    } else {
        (locals.var_uc_t, locals.var_uc_t_dn4,)
    }
};
        locals.var_uc_t = assign17500_e30937;
        locals.var_uc_t_dn4 = assign17500_e30937_d_n4;

        let assign17510_e30940: f64 = if p.p67 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard316 = assign17510_e30940;

        let assign17520_e30943: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard317 = assign17520_e30943;

        let (assign17530_e30993, assign17530_e30993_d_n4,) = {
    if ((((locals.var_guard313 != 0.0) && (locals.var_guard314 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign17530_e30953: f64 = (-locals.var_uccv_i);
        let assign17530_e30957: f64 = (locals.var_uc1cv_i * locals.var_deltemp);
        let assign17530_e30959: f64 = (-locals.var_uccv_i);
        let assign17530_e30960: f64 = (assign17530_e30957 - assign17530_e30959);
        let assign17530_e30962: f64 = (assign17530_e30960 - 1e-6);
        let assign17530_e30965: f64 = (locals.var_uc1cv_i * locals.var_deltemp);
        let assign17530_e30967: f64 = (-locals.var_uccv_i);
        let assign17530_e30968: f64 = (assign17530_e30965 - assign17530_e30967);
        let assign17530_e30970: f64 = (assign17530_e30968 - 1e-6);
        let assign17530_e30973: f64 = (locals.var_uc1cv_i * locals.var_deltemp);
        let assign17530_e30975: f64 = (-locals.var_uccv_i);
        let assign17530_e30976: f64 = (assign17530_e30973 - assign17530_e30975);
        let assign17530_e30978: f64 = (assign17530_e30976 - 1e-6);
        let assign17530_e30979: f64 = (assign17530_e30970 * assign17530_e30978);
        let assign17530_e30982: f64 = (-locals.var_uccv_i);
        let assign17530_e30983: f64 = (4.0 * assign17530_e30982);
        let assign17530_e30985: f64 = (assign17530_e30983 * 1e-6);
        let assign17530_e30986: f64 = (assign17530_e30979 - assign17530_e30985);
        let assign17530_e30987: f64 = (assign17530_e30986).sqrt();
        let assign17530_e30988: f64 = (assign17530_e30962 + assign17530_e30987);
        let assign17530_e30989: f64 = (0.5 * assign17530_e30988);
        let assign17530_e30990: f64 = (assign17530_e30953 + assign17530_e30989);
        let assign17530_e30991: f64 = (locals.var_uccv_i + assign17530_e30990);
        (assign17530_e30991, (0.5 * ((locals.var_uc1cv_i * locals.var_deltemp_dn4) + ((((locals.var_uc1cv_i * locals.var_deltemp_dn4) * assign17530_e30978) + (assign17530_e30970 * (locals.var_uc1cv_i * locals.var_deltemp_dn4))) / (2.0 * assign17530_e30987)))),)
    } else {
        (locals.var_uccv_t, locals.var_uccv_t_dn4,)
    }
};
        locals.var_uccv_t = assign17530_e30993;
        locals.var_uccv_t_dn4 = assign17530_e30993_d_n4;

        let (assign17540_e31077, assign17540_e31077_d_n4,) = {
    if ((((locals.var_guard313 != 0.0) && (locals.var_guard314 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign17540_e31006: f64 = (locals.var_uc1cv_i * locals.var_deltemp);
        let assign17540_e31007: f64 = (1.0 + assign17540_e31006);
        let assign17540_e31009: f64 = (assign17540_e31007 - 1e-6);
        let assign17540_e31011: f64 = (-10000.0);
        let assign17540_e31013: f64 = (assign17540_e31011 * 0.001);
        let (assign17540_e31074, assign17540_e31074_d_n4,) = {
            if (!(assign17540_e31009 < assign17540_e31013)) {
                let assign17540_e31020: f64 = (locals.var_uc1cv_i * locals.var_deltemp);
                let assign17540_e31021: f64 = (1.0 + assign17540_e31020);
                let assign17540_e31023: f64 = (assign17540_e31021 - 1e-6);
                let assign17540_e31027: f64 = (locals.var_uc1cv_i * locals.var_deltemp);
                let assign17540_e31028: f64 = (1.0 + assign17540_e31027);
                let assign17540_e31030: f64 = (assign17540_e31028 - 1e-6);
                let assign17540_e31034: f64 = (locals.var_uc1cv_i * locals.var_deltemp);
                let assign17540_e31035: f64 = (1.0 + assign17540_e31034);
                let assign17540_e31037: f64 = (assign17540_e31035 - 1e-6);
                let assign17540_e31038: f64 = (assign17540_e31030 * assign17540_e31037);
                let assign17540_e31041: f64 = (4.0 * 0.001);
                let assign17540_e31043: f64 = (assign17540_e31041 * 0.001);
                let assign17540_e31044: f64 = (assign17540_e31038 + assign17540_e31043);
                let assign17540_e31045: f64 = (assign17540_e31044).sqrt();
                let assign17540_e31046: f64 = (assign17540_e31023 + assign17540_e31045);
                let assign17540_e31047: f64 = (0.5 * assign17540_e31046);
                (assign17540_e31047, (0.5 * ((locals.var_uc1cv_i * locals.var_deltemp_dn4) + ((((locals.var_uc1cv_i * locals.var_deltemp_dn4) * assign17540_e31037) + (assign17540_e31030 * (locals.var_uc1cv_i * locals.var_deltemp_dn4))) / (2.0 * assign17540_e31045)))),)
            } else {
                let assign17540_e31051: f64 = (locals.var_uc1cv_i * locals.var_deltemp);
                let assign17540_e31052: f64 = (1.0 + assign17540_e31051);
                let assign17540_e31054: f64 = (assign17540_e31052 - 1e-6);
                let assign17540_e31056: f64 = (-10000.0);
                let assign17540_e31058: f64 = (assign17540_e31056 * 0.001);
                let (assign17540_e31073, assign17540_e31073_d_n4,) = {
                    if (assign17540_e31054 < assign17540_e31058) {
                        let assign17540_e31061: f64 = (-0.001);
                        let assign17540_e31063: f64 = (assign17540_e31061 * 0.001);
                        let assign17540_e31067: f64 = (locals.var_uc1cv_i * locals.var_deltemp);
                        let assign17540_e31068: f64 = (1.0 + assign17540_e31067);
                        let assign17540_e31070: f64 = (assign17540_e31068 - 1e-6);
                        let assign17540_e31071: f64 = (assign17540_e31063 / assign17540_e31070);
                        (assign17540_e31071, (-((assign17540_e31063 * (locals.var_uc1cv_i * locals.var_deltemp_dn4)) / (assign17540_e31070 * assign17540_e31070))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17540_e31073, assign17540_e31073_d_n4,)
            }
        };
        let assign17540_e31075: f64 = (locals.var_uccv_i * assign17540_e31074);
        (assign17540_e31075, (locals.var_uccv_i * assign17540_e31074_d_n4),)
    } else {
        (locals.var_uccv_t, locals.var_uccv_t_dn4,)
    }
};
        locals.var_uccv_t = assign17540_e31077;
        locals.var_uccv_t_dn4 = assign17540_e31077_d_n4;

        let assign17550_e31080: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard318 = assign17550_e31080;

        let assign17560_e31083: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard319 = assign17560_e31083;

        let (assign17570_e31133, assign17570_e31133_d_n4,) = {
    if ((((locals.var_guard313 != 0.0) && (locals.var_guard314 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 != 0.0)) {
        let assign17570_e31093: f64 = (-locals.var_ucr_i);
        let assign17570_e31097: f64 = (locals.var_uc1r_i * locals.var_deltemp);
        let assign17570_e31099: f64 = (-locals.var_ucr_i);
        let assign17570_e31100: f64 = (assign17570_e31097 - assign17570_e31099);
        let assign17570_e31102: f64 = (assign17570_e31100 - 1e-6);
        let assign17570_e31105: f64 = (locals.var_uc1r_i * locals.var_deltemp);
        let assign17570_e31107: f64 = (-locals.var_ucr_i);
        let assign17570_e31108: f64 = (assign17570_e31105 - assign17570_e31107);
        let assign17570_e31110: f64 = (assign17570_e31108 - 1e-6);
        let assign17570_e31113: f64 = (locals.var_uc1r_i * locals.var_deltemp);
        let assign17570_e31115: f64 = (-locals.var_ucr_i);
        let assign17570_e31116: f64 = (assign17570_e31113 - assign17570_e31115);
        let assign17570_e31118: f64 = (assign17570_e31116 - 1e-6);
        let assign17570_e31119: f64 = (assign17570_e31110 * assign17570_e31118);
        let assign17570_e31122: f64 = (-locals.var_ucr_i);
        let assign17570_e31123: f64 = (4.0 * assign17570_e31122);
        let assign17570_e31125: f64 = (assign17570_e31123 * 1e-6);
        let assign17570_e31126: f64 = (assign17570_e31119 - assign17570_e31125);
        let assign17570_e31127: f64 = (assign17570_e31126).sqrt();
        let assign17570_e31128: f64 = (assign17570_e31102 + assign17570_e31127);
        let assign17570_e31129: f64 = (0.5 * assign17570_e31128);
        let assign17570_e31130: f64 = (assign17570_e31093 + assign17570_e31129);
        let assign17570_e31131: f64 = (locals.var_ucr_i + assign17570_e31130);
        (assign17570_e31131, (0.5 * ((locals.var_uc1r_i * locals.var_deltemp_dn4) + ((((locals.var_uc1r_i * locals.var_deltemp_dn4) * assign17570_e31118) + (assign17570_e31110 * (locals.var_uc1r_i * locals.var_deltemp_dn4))) / (2.0 * assign17570_e31127)))),)
    } else {
        (locals.var_ucr_t, locals.var_ucr_t_dn4,)
    }
};
        locals.var_ucr_t = assign17570_e31133;
        locals.var_ucr_t_dn4 = assign17570_e31133_d_n4;

        let (assign17580_e31217, assign17580_e31217_d_n4,) = {
    if ((((locals.var_guard313 != 0.0) && (locals.var_guard314 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 == 0.0)) {
        let assign17580_e31146: f64 = (locals.var_uc1r_i * locals.var_deltemp);
        let assign17580_e31147: f64 = (1.0 + assign17580_e31146);
        let assign17580_e31149: f64 = (assign17580_e31147 - 1e-6);
        let assign17580_e31151: f64 = (-10000.0);
        let assign17580_e31153: f64 = (assign17580_e31151 * 0.001);
        let (assign17580_e31214, assign17580_e31214_d_n4,) = {
            if (!(assign17580_e31149 < assign17580_e31153)) {
                let assign17580_e31160: f64 = (locals.var_uc1r_i * locals.var_deltemp);
                let assign17580_e31161: f64 = (1.0 + assign17580_e31160);
                let assign17580_e31163: f64 = (assign17580_e31161 - 1e-6);
                let assign17580_e31167: f64 = (locals.var_uc1r_i * locals.var_deltemp);
                let assign17580_e31168: f64 = (1.0 + assign17580_e31167);
                let assign17580_e31170: f64 = (assign17580_e31168 - 1e-6);
                let assign17580_e31174: f64 = (locals.var_uc1r_i * locals.var_deltemp);
                let assign17580_e31175: f64 = (1.0 + assign17580_e31174);
                let assign17580_e31177: f64 = (assign17580_e31175 - 1e-6);
                let assign17580_e31178: f64 = (assign17580_e31170 * assign17580_e31177);
                let assign17580_e31181: f64 = (4.0 * 0.001);
                let assign17580_e31183: f64 = (assign17580_e31181 * 0.001);
                let assign17580_e31184: f64 = (assign17580_e31178 + assign17580_e31183);
                let assign17580_e31185: f64 = (assign17580_e31184).sqrt();
                let assign17580_e31186: f64 = (assign17580_e31163 + assign17580_e31185);
                let assign17580_e31187: f64 = (0.5 * assign17580_e31186);
                (assign17580_e31187, (0.5 * ((locals.var_uc1r_i * locals.var_deltemp_dn4) + ((((locals.var_uc1r_i * locals.var_deltemp_dn4) * assign17580_e31177) + (assign17580_e31170 * (locals.var_uc1r_i * locals.var_deltemp_dn4))) / (2.0 * assign17580_e31185)))),)
            } else {
                let assign17580_e31191: f64 = (locals.var_uc1r_i * locals.var_deltemp);
                let assign17580_e31192: f64 = (1.0 + assign17580_e31191);
                let assign17580_e31194: f64 = (assign17580_e31192 - 1e-6);
                let assign17580_e31196: f64 = (-10000.0);
                let assign17580_e31198: f64 = (assign17580_e31196 * 0.001);
                let (assign17580_e31213, assign17580_e31213_d_n4,) = {
                    if (assign17580_e31194 < assign17580_e31198) {
                        let assign17580_e31201: f64 = (-0.001);
                        let assign17580_e31203: f64 = (assign17580_e31201 * 0.001);
                        let assign17580_e31207: f64 = (locals.var_uc1r_i * locals.var_deltemp);
                        let assign17580_e31208: f64 = (1.0 + assign17580_e31207);
                        let assign17580_e31210: f64 = (assign17580_e31208 - 1e-6);
                        let assign17580_e31211: f64 = (assign17580_e31203 / assign17580_e31210);
                        (assign17580_e31211, (-((assign17580_e31203 * (locals.var_uc1r_i * locals.var_deltemp_dn4)) / (assign17580_e31210 * assign17580_e31210))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17580_e31213, assign17580_e31213_d_n4,)
            }
        };
        let assign17580_e31215: f64 = (locals.var_ucr_i * assign17580_e31214);
        (assign17580_e31215, (locals.var_ucr_i * assign17580_e31214_d_n4),)
    } else {
        (locals.var_ucr_t, locals.var_ucr_t_dn4,)
    }
};
        locals.var_ucr_t = assign17580_e31217;
        locals.var_ucr_t_dn4 = assign17580_e31217_d_n4;

        let (assign17590_e31228, assign17590_e31228_d_n4,) = {
    if ((locals.var_guard313 != 0.0) && (locals.var_guard314 == 0.0)) {
        let assign17590_e31225: f64 = (locals.var_uc1_i * locals.var_deltemp);
        let assign17590_e31226: f64 = (locals.var_uc_i + assign17590_e31225);
        (assign17590_e31226, (locals.var_uc1_i * locals.var_deltemp_dn4),)
    } else {
        (locals.var_uc_t, locals.var_uc_t_dn4,)
    }
};
        locals.var_uc_t = assign17590_e31228;
        locals.var_uc_t_dn4 = assign17590_e31228_d_n4;

        let assign17600_e31231: f64 = if p.p67 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard320 = assign17600_e31231;

    }

    pub(super) fn stamp_transient_block_57(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17610_e31244, assign17610_e31244_d_n4,) = {
    if (((locals.var_guard313 != 0.0) && (locals.var_guard314 == 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign17610_e31241: f64 = (locals.var_uc1cv_i * locals.var_deltemp);
        let assign17610_e31242: f64 = (locals.var_uccv_i + assign17610_e31241);
        (assign17610_e31242, (locals.var_uc1cv_i * locals.var_deltemp_dn4),)
    } else {
        (locals.var_uccv_t, locals.var_uccv_t_dn4,)
    }
};
        locals.var_uccv_t = assign17610_e31244;
        locals.var_uccv_t_dn4 = assign17610_e31244_d_n4;

        let assign17620_e31247: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard321 = assign17620_e31247;

        let (assign17630_e31260, assign17630_e31260_d_n4,) = {
    if (((locals.var_guard313 != 0.0) && (locals.var_guard314 == 0.0)) && (locals.var_guard321 != 0.0)) {
        let assign17630_e31257: f64 = (locals.var_uc1r_i * locals.var_deltemp);
        let assign17630_e31258: f64 = (locals.var_ucr_i + assign17630_e31257);
        (assign17630_e31258, (locals.var_uc1r_i * locals.var_deltemp_dn4),)
    } else {
        (locals.var_ucr_t, locals.var_ucr_t_dn4,)
    }
};
        locals.var_ucr_t = assign17630_e31260;
        locals.var_ucr_t_dn4 = assign17630_e31260_d_n4;

        let assign17640_e31263: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard322 = assign17640_e31263;

        let (assign17650_e31307, assign17650_e31307_d_n0, assign17650_e31307_d_n2, assign17650_e31307_d_n3, assign17650_e31307_d_n4, assign17650_e31307_d_n5, assign17650_e31307_d_n6, assign17650_e31307_d_n7, assign17650_e31307_d_n8, assign17650_e31307_d_n9, assign17650_e31307_d_n10, assign17650_e31307_d_n11, assign17650_e31307_d_n13, assign17650_e31307_d_n14,) = {
    if (locals.var_guard322 != 0.0) {
        let assign17650_e31267: f64 = (-locals.var_eta0_i);
        let assign17650_e31271: f64 = (p.p164 * locals.var_deltemp);
        let assign17650_e31273: f64 = (-locals.var_eta0_i);
        let assign17650_e31274: f64 = (assign17650_e31271 - assign17650_e31273);
        let assign17650_e31276: f64 = (assign17650_e31274 - 1e-6);
        let assign17650_e31279: f64 = (p.p164 * locals.var_deltemp);
        let assign17650_e31281: f64 = (-locals.var_eta0_i);
        let assign17650_e31282: f64 = (assign17650_e31279 - assign17650_e31281);
        let assign17650_e31284: f64 = (assign17650_e31282 - 1e-6);
        let assign17650_e31287: f64 = (p.p164 * locals.var_deltemp);
        let assign17650_e31289: f64 = (-locals.var_eta0_i);
        let assign17650_e31290: f64 = (assign17650_e31287 - assign17650_e31289);
        let assign17650_e31292: f64 = (assign17650_e31290 - 1e-6);
        let assign17650_e31293: f64 = (assign17650_e31284 * assign17650_e31292);
        let assign17650_e31296: f64 = (-locals.var_eta0_i);
        let assign17650_e31297: f64 = (4.0 * assign17650_e31296);
        let assign17650_e31299: f64 = (assign17650_e31297 * 1e-6);
        let assign17650_e31300: f64 = (assign17650_e31293 - assign17650_e31299);
        let assign17650_e31301: f64 = (assign17650_e31300).sqrt();
        let assign17650_e31302: f64 = (assign17650_e31276 + assign17650_e31301);
        let assign17650_e31303: f64 = (0.5 * assign17650_e31302);
        let assign17650_e31304: f64 = (assign17650_e31267 + assign17650_e31303);
        let assign17650_e31305: f64 = (locals.var_eta0_i + assign17650_e31304);
        (assign17650_e31305, (locals.var_eta0_i_dn0 + ((-locals.var_eta0_i_dn0) + (0.5 * ((-(-locals.var_eta0_i_dn0)) + (((((-(-locals.var_eta0_i_dn0)) * assign17650_e31292) + (assign17650_e31284 * (-(-locals.var_eta0_i_dn0)))) - ((4.0 * (-locals.var_eta0_i_dn0)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (locals.var_eta0_i_dn2 + ((-locals.var_eta0_i_dn2) + (0.5 * ((-(-locals.var_eta0_i_dn2)) + (((((-(-locals.var_eta0_i_dn2)) * assign17650_e31292) + (assign17650_e31284 * (-(-locals.var_eta0_i_dn2)))) - ((4.0 * (-locals.var_eta0_i_dn2)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (locals.var_eta0_i_dn3 + ((-locals.var_eta0_i_dn3) + (0.5 * ((-(-locals.var_eta0_i_dn3)) + (((((-(-locals.var_eta0_i_dn3)) * assign17650_e31292) + (assign17650_e31284 * (-(-locals.var_eta0_i_dn3)))) - ((4.0 * (-locals.var_eta0_i_dn3)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (locals.var_eta0_i_dn4 + ((-locals.var_eta0_i_dn4) + (0.5 * (((p.p164 * locals.var_deltemp_dn4) - (-locals.var_eta0_i_dn4)) + ((((((p.p164 * locals.var_deltemp_dn4) - (-locals.var_eta0_i_dn4)) * assign17650_e31292) + (assign17650_e31284 * ((p.p164 * locals.var_deltemp_dn4) - (-locals.var_eta0_i_dn4)))) - ((4.0 * (-locals.var_eta0_i_dn4)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (locals.var_eta0_i_dn5 + ((-locals.var_eta0_i_dn5) + (0.5 * ((-(-locals.var_eta0_i_dn5)) + (((((-(-locals.var_eta0_i_dn5)) * assign17650_e31292) + (assign17650_e31284 * (-(-locals.var_eta0_i_dn5)))) - ((4.0 * (-locals.var_eta0_i_dn5)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (locals.var_eta0_i_dn6 + ((-locals.var_eta0_i_dn6) + (0.5 * ((-(-locals.var_eta0_i_dn6)) + (((((-(-locals.var_eta0_i_dn6)) * assign17650_e31292) + (assign17650_e31284 * (-(-locals.var_eta0_i_dn6)))) - ((4.0 * (-locals.var_eta0_i_dn6)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (locals.var_eta0_i_dn7 + ((-locals.var_eta0_i_dn7) + (0.5 * ((-(-locals.var_eta0_i_dn7)) + (((((-(-locals.var_eta0_i_dn7)) * assign17650_e31292) + (assign17650_e31284 * (-(-locals.var_eta0_i_dn7)))) - ((4.0 * (-locals.var_eta0_i_dn7)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (locals.var_eta0_i_dn8 + ((-locals.var_eta0_i_dn8) + (0.5 * ((-(-locals.var_eta0_i_dn8)) + (((((-(-locals.var_eta0_i_dn8)) * assign17650_e31292) + (assign17650_e31284 * (-(-locals.var_eta0_i_dn8)))) - ((4.0 * (-locals.var_eta0_i_dn8)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (locals.var_eta0_i_dn9 + ((-locals.var_eta0_i_dn9) + (0.5 * ((-(-locals.var_eta0_i_dn9)) + (((((-(-locals.var_eta0_i_dn9)) * assign17650_e31292) + (assign17650_e31284 * (-(-locals.var_eta0_i_dn9)))) - ((4.0 * (-locals.var_eta0_i_dn9)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (locals.var_eta0_i_dn10 + ((-locals.var_eta0_i_dn10) + (0.5 * ((-(-locals.var_eta0_i_dn10)) + (((((-(-locals.var_eta0_i_dn10)) * assign17650_e31292) + (assign17650_e31284 * (-(-locals.var_eta0_i_dn10)))) - ((4.0 * (-locals.var_eta0_i_dn10)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (locals.var_eta0_i_dn11 + ((-locals.var_eta0_i_dn11) + (0.5 * ((-(-locals.var_eta0_i_dn11)) + (((((-(-locals.var_eta0_i_dn11)) * assign17650_e31292) + (assign17650_e31284 * (-(-locals.var_eta0_i_dn11)))) - ((4.0 * (-locals.var_eta0_i_dn11)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (locals.var_eta0_i_dn13 + ((-locals.var_eta0_i_dn13) + (0.5 * ((-(-locals.var_eta0_i_dn13)) + (((((-(-locals.var_eta0_i_dn13)) * assign17650_e31292) + (assign17650_e31284 * (-(-locals.var_eta0_i_dn13)))) - ((4.0 * (-locals.var_eta0_i_dn13)) * 1e-6)) / (2.0 * assign17650_e31301)))))), (locals.var_eta0_i_dn14 + ((-locals.var_eta0_i_dn14) + (0.5 * ((-(-locals.var_eta0_i_dn14)) + (((((-(-locals.var_eta0_i_dn14)) * assign17650_e31292) + (assign17650_e31284 * (-(-locals.var_eta0_i_dn14)))) - ((4.0 * (-locals.var_eta0_i_dn14)) * 1e-6)) / (2.0 * assign17650_e31301)))))),)
    } else {
        (locals.var_eta0_t, locals.var_eta0_t_dn0, locals.var_eta0_t_dn2, locals.var_eta0_t_dn3, locals.var_eta0_t_dn4, locals.var_eta0_t_dn5, locals.var_eta0_t_dn6, locals.var_eta0_t_dn7, locals.var_eta0_t_dn8, locals.var_eta0_t_dn9, locals.var_eta0_t_dn10, locals.var_eta0_t_dn11, locals.var_eta0_t_dn13, locals.var_eta0_t_dn14,)
    }
};
        locals.var_eta0_t = assign17650_e31307;
        locals.var_eta0_t_dn0 = assign17650_e31307_d_n0;
        locals.var_eta0_t_dn2 = assign17650_e31307_d_n2;
        locals.var_eta0_t_dn3 = assign17650_e31307_d_n3;
        locals.var_eta0_t_dn4 = assign17650_e31307_d_n4;
        locals.var_eta0_t_dn5 = assign17650_e31307_d_n5;
        locals.var_eta0_t_dn6 = assign17650_e31307_d_n6;
        locals.var_eta0_t_dn7 = assign17650_e31307_d_n7;
        locals.var_eta0_t_dn8 = assign17650_e31307_d_n8;
        locals.var_eta0_t_dn9 = assign17650_e31307_d_n9;
        locals.var_eta0_t_dn10 = assign17650_e31307_d_n10;
        locals.var_eta0_t_dn11 = assign17650_e31307_d_n11;
        locals.var_eta0_t_dn13 = assign17650_e31307_d_n13;
        locals.var_eta0_t_dn14 = assign17650_e31307_d_n14;

        let (assign17660_e31385, assign17660_e31385_d_n0, assign17660_e31385_d_n2, assign17660_e31385_d_n3, assign17660_e31385_d_n4, assign17660_e31385_d_n5, assign17660_e31385_d_n6, assign17660_e31385_d_n7, assign17660_e31385_d_n8, assign17660_e31385_d_n9, assign17660_e31385_d_n10, assign17660_e31385_d_n11, assign17660_e31385_d_n13, assign17660_e31385_d_n14,) = {
    if (locals.var_guard322 == 0.0) {
        let assign17660_e31314: f64 = (p.p164 * locals.var_deltemp);
        let assign17660_e31315: f64 = (1.0 + assign17660_e31314);
        let assign17660_e31317: f64 = (assign17660_e31315 - 1e-6);
        let assign17660_e31319: f64 = (-10000.0);
        let assign17660_e31321: f64 = (assign17660_e31319 * 0.001);
        let (assign17660_e31382, assign17660_e31382_d_n4,) = {
            if (!(assign17660_e31317 < assign17660_e31321)) {
                let assign17660_e31328: f64 = (p.p164 * locals.var_deltemp);
                let assign17660_e31329: f64 = (1.0 + assign17660_e31328);
                let assign17660_e31331: f64 = (assign17660_e31329 - 1e-6);
                let assign17660_e31335: f64 = (p.p164 * locals.var_deltemp);
                let assign17660_e31336: f64 = (1.0 + assign17660_e31335);
                let assign17660_e31338: f64 = (assign17660_e31336 - 1e-6);
                let assign17660_e31342: f64 = (p.p164 * locals.var_deltemp);
                let assign17660_e31343: f64 = (1.0 + assign17660_e31342);
                let assign17660_e31345: f64 = (assign17660_e31343 - 1e-6);
                let assign17660_e31346: f64 = (assign17660_e31338 * assign17660_e31345);
                let assign17660_e31349: f64 = (4.0 * 0.001);
                let assign17660_e31351: f64 = (assign17660_e31349 * 0.001);
                let assign17660_e31352: f64 = (assign17660_e31346 + assign17660_e31351);
                let assign17660_e31353: f64 = (assign17660_e31352).sqrt();
                let assign17660_e31354: f64 = (assign17660_e31331 + assign17660_e31353);
                let assign17660_e31355: f64 = (0.5 * assign17660_e31354);
                (assign17660_e31355, (0.5 * ((p.p164 * locals.var_deltemp_dn4) + ((((p.p164 * locals.var_deltemp_dn4) * assign17660_e31345) + (assign17660_e31338 * (p.p164 * locals.var_deltemp_dn4))) / (2.0 * assign17660_e31353)))),)
            } else {
                let assign17660_e31359: f64 = (p.p164 * locals.var_deltemp);
                let assign17660_e31360: f64 = (1.0 + assign17660_e31359);
                let assign17660_e31362: f64 = (assign17660_e31360 - 1e-6);
                let assign17660_e31364: f64 = (-10000.0);
                let assign17660_e31366: f64 = (assign17660_e31364 * 0.001);
                let (assign17660_e31381, assign17660_e31381_d_n4,) = {
                    if (assign17660_e31362 < assign17660_e31366) {
                        let assign17660_e31369: f64 = (-0.001);
                        let assign17660_e31371: f64 = (assign17660_e31369 * 0.001);
                        let assign17660_e31375: f64 = (p.p164 * locals.var_deltemp);
                        let assign17660_e31376: f64 = (1.0 + assign17660_e31375);
                        let assign17660_e31378: f64 = (assign17660_e31376 - 1e-6);
                        let assign17660_e31379: f64 = (assign17660_e31371 / assign17660_e31378);
                        (assign17660_e31379, (-((assign17660_e31371 * (p.p164 * locals.var_deltemp_dn4)) / (assign17660_e31378 * assign17660_e31378))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17660_e31381, assign17660_e31381_d_n4,)
            }
        };
        let assign17660_e31383: f64 = (locals.var_eta0_i * assign17660_e31382);
        (assign17660_e31383, (locals.var_eta0_i_dn0 * assign17660_e31382), (locals.var_eta0_i_dn2 * assign17660_e31382), (locals.var_eta0_i_dn3 * assign17660_e31382), ((locals.var_eta0_i_dn4 * assign17660_e31382) + (locals.var_eta0_i * assign17660_e31382_d_n4)), (locals.var_eta0_i_dn5 * assign17660_e31382), (locals.var_eta0_i_dn6 * assign17660_e31382), (locals.var_eta0_i_dn7 * assign17660_e31382), (locals.var_eta0_i_dn8 * assign17660_e31382), (locals.var_eta0_i_dn9 * assign17660_e31382), (locals.var_eta0_i_dn10 * assign17660_e31382), (locals.var_eta0_i_dn11 * assign17660_e31382), (locals.var_eta0_i_dn13 * assign17660_e31382), (locals.var_eta0_i_dn14 * assign17660_e31382),)
    } else {
        (locals.var_eta0_t, locals.var_eta0_t_dn0, locals.var_eta0_t_dn2, locals.var_eta0_t_dn3, locals.var_eta0_t_dn4, locals.var_eta0_t_dn5, locals.var_eta0_t_dn6, locals.var_eta0_t_dn7, locals.var_eta0_t_dn8, locals.var_eta0_t_dn9, locals.var_eta0_t_dn10, locals.var_eta0_t_dn11, locals.var_eta0_t_dn13, locals.var_eta0_t_dn14,)
    }
};
        locals.var_eta0_t = assign17660_e31385;
        locals.var_eta0_t_dn0 = assign17660_e31385_d_n0;
        locals.var_eta0_t_dn2 = assign17660_e31385_d_n2;
        locals.var_eta0_t_dn3 = assign17660_e31385_d_n3;
        locals.var_eta0_t_dn4 = assign17660_e31385_d_n4;
        locals.var_eta0_t_dn5 = assign17660_e31385_d_n5;
        locals.var_eta0_t_dn6 = assign17660_e31385_d_n6;
        locals.var_eta0_t_dn7 = assign17660_e31385_d_n7;
        locals.var_eta0_t_dn8 = assign17660_e31385_d_n8;
        locals.var_eta0_t_dn9 = assign17660_e31385_d_n9;
        locals.var_eta0_t_dn10 = assign17660_e31385_d_n10;
        locals.var_eta0_t_dn11 = assign17660_e31385_d_n11;
        locals.var_eta0_t_dn13 = assign17660_e31385_d_n13;
        locals.var_eta0_t_dn14 = assign17660_e31385_d_n14;

        let assign17670_e31388: f64 = if p.p67 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard323 = assign17670_e31388;

        let assign17680_e31391: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard324 = assign17680_e31391;

        let (assign17690_e31437, assign17690_e31437_d_n0, assign17690_e31437_d_n2, assign17690_e31437_d_n3, assign17690_e31437_d_n4, assign17690_e31437_d_n5, assign17690_e31437_d_n6, assign17690_e31437_d_n7, assign17690_e31437_d_n8, assign17690_e31437_d_n9, assign17690_e31437_d_n10, assign17690_e31437_d_n11, assign17690_e31437_d_n13, assign17690_e31437_d_n14,) = {
    if ((locals.var_guard323 != 0.0) && (locals.var_guard324 != 0.0)) {
        let assign17690_e31397: f64 = (-locals.var_eta0cv_i);
        let assign17690_e31401: f64 = (p.p165 * locals.var_deltemp);
        let assign17690_e31403: f64 = (-locals.var_eta0cv_i);
        let assign17690_e31404: f64 = (assign17690_e31401 - assign17690_e31403);
        let assign17690_e31406: f64 = (assign17690_e31404 - 1e-6);
        let assign17690_e31409: f64 = (p.p165 * locals.var_deltemp);
        let assign17690_e31411: f64 = (-locals.var_eta0cv_i);
        let assign17690_e31412: f64 = (assign17690_e31409 - assign17690_e31411);
        let assign17690_e31414: f64 = (assign17690_e31412 - 1e-6);
        let assign17690_e31417: f64 = (p.p165 * locals.var_deltemp);
        let assign17690_e31419: f64 = (-locals.var_eta0cv_i);
        let assign17690_e31420: f64 = (assign17690_e31417 - assign17690_e31419);
        let assign17690_e31422: f64 = (assign17690_e31420 - 1e-6);
        let assign17690_e31423: f64 = (assign17690_e31414 * assign17690_e31422);
        let assign17690_e31426: f64 = (-locals.var_eta0cv_i);
        let assign17690_e31427: f64 = (4.0 * assign17690_e31426);
        let assign17690_e31429: f64 = (assign17690_e31427 * 1e-6);
        let assign17690_e31430: f64 = (assign17690_e31423 - assign17690_e31429);
        let assign17690_e31431: f64 = (assign17690_e31430).sqrt();
        let assign17690_e31432: f64 = (assign17690_e31406 + assign17690_e31431);
        let assign17690_e31433: f64 = (0.5 * assign17690_e31432);
        let assign17690_e31434: f64 = (assign17690_e31397 + assign17690_e31433);
        let assign17690_e31435: f64 = (locals.var_eta0cv_i + assign17690_e31434);
        (assign17690_e31435, (locals.var_eta0cv_i_dn0 + ((-locals.var_eta0cv_i_dn0) + (0.5 * ((-(-locals.var_eta0cv_i_dn0)) + (((((-(-locals.var_eta0cv_i_dn0)) * assign17690_e31422) + (assign17690_e31414 * (-(-locals.var_eta0cv_i_dn0)))) - ((4.0 * (-locals.var_eta0cv_i_dn0)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (locals.var_eta0cv_i_dn2 + ((-locals.var_eta0cv_i_dn2) + (0.5 * ((-(-locals.var_eta0cv_i_dn2)) + (((((-(-locals.var_eta0cv_i_dn2)) * assign17690_e31422) + (assign17690_e31414 * (-(-locals.var_eta0cv_i_dn2)))) - ((4.0 * (-locals.var_eta0cv_i_dn2)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (locals.var_eta0cv_i_dn3 + ((-locals.var_eta0cv_i_dn3) + (0.5 * ((-(-locals.var_eta0cv_i_dn3)) + (((((-(-locals.var_eta0cv_i_dn3)) * assign17690_e31422) + (assign17690_e31414 * (-(-locals.var_eta0cv_i_dn3)))) - ((4.0 * (-locals.var_eta0cv_i_dn3)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (locals.var_eta0cv_i_dn4 + ((-locals.var_eta0cv_i_dn4) + (0.5 * (((p.p165 * locals.var_deltemp_dn4) - (-locals.var_eta0cv_i_dn4)) + ((((((p.p165 * locals.var_deltemp_dn4) - (-locals.var_eta0cv_i_dn4)) * assign17690_e31422) + (assign17690_e31414 * ((p.p165 * locals.var_deltemp_dn4) - (-locals.var_eta0cv_i_dn4)))) - ((4.0 * (-locals.var_eta0cv_i_dn4)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (locals.var_eta0cv_i_dn5 + ((-locals.var_eta0cv_i_dn5) + (0.5 * ((-(-locals.var_eta0cv_i_dn5)) + (((((-(-locals.var_eta0cv_i_dn5)) * assign17690_e31422) + (assign17690_e31414 * (-(-locals.var_eta0cv_i_dn5)))) - ((4.0 * (-locals.var_eta0cv_i_dn5)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (locals.var_eta0cv_i_dn6 + ((-locals.var_eta0cv_i_dn6) + (0.5 * ((-(-locals.var_eta0cv_i_dn6)) + (((((-(-locals.var_eta0cv_i_dn6)) * assign17690_e31422) + (assign17690_e31414 * (-(-locals.var_eta0cv_i_dn6)))) - ((4.0 * (-locals.var_eta0cv_i_dn6)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (locals.var_eta0cv_i_dn7 + ((-locals.var_eta0cv_i_dn7) + (0.5 * ((-(-locals.var_eta0cv_i_dn7)) + (((((-(-locals.var_eta0cv_i_dn7)) * assign17690_e31422) + (assign17690_e31414 * (-(-locals.var_eta0cv_i_dn7)))) - ((4.0 * (-locals.var_eta0cv_i_dn7)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (locals.var_eta0cv_i_dn8 + ((-locals.var_eta0cv_i_dn8) + (0.5 * ((-(-locals.var_eta0cv_i_dn8)) + (((((-(-locals.var_eta0cv_i_dn8)) * assign17690_e31422) + (assign17690_e31414 * (-(-locals.var_eta0cv_i_dn8)))) - ((4.0 * (-locals.var_eta0cv_i_dn8)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (locals.var_eta0cv_i_dn9 + ((-locals.var_eta0cv_i_dn9) + (0.5 * ((-(-locals.var_eta0cv_i_dn9)) + (((((-(-locals.var_eta0cv_i_dn9)) * assign17690_e31422) + (assign17690_e31414 * (-(-locals.var_eta0cv_i_dn9)))) - ((4.0 * (-locals.var_eta0cv_i_dn9)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (locals.var_eta0cv_i_dn10 + ((-locals.var_eta0cv_i_dn10) + (0.5 * ((-(-locals.var_eta0cv_i_dn10)) + (((((-(-locals.var_eta0cv_i_dn10)) * assign17690_e31422) + (assign17690_e31414 * (-(-locals.var_eta0cv_i_dn10)))) - ((4.0 * (-locals.var_eta0cv_i_dn10)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (locals.var_eta0cv_i_dn11 + ((-locals.var_eta0cv_i_dn11) + (0.5 * ((-(-locals.var_eta0cv_i_dn11)) + (((((-(-locals.var_eta0cv_i_dn11)) * assign17690_e31422) + (assign17690_e31414 * (-(-locals.var_eta0cv_i_dn11)))) - ((4.0 * (-locals.var_eta0cv_i_dn11)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (locals.var_eta0cv_i_dn13 + ((-locals.var_eta0cv_i_dn13) + (0.5 * ((-(-locals.var_eta0cv_i_dn13)) + (((((-(-locals.var_eta0cv_i_dn13)) * assign17690_e31422) + (assign17690_e31414 * (-(-locals.var_eta0cv_i_dn13)))) - ((4.0 * (-locals.var_eta0cv_i_dn13)) * 1e-6)) / (2.0 * assign17690_e31431)))))), (locals.var_eta0cv_i_dn14 + ((-locals.var_eta0cv_i_dn14) + (0.5 * ((-(-locals.var_eta0cv_i_dn14)) + (((((-(-locals.var_eta0cv_i_dn14)) * assign17690_e31422) + (assign17690_e31414 * (-(-locals.var_eta0cv_i_dn14)))) - ((4.0 * (-locals.var_eta0cv_i_dn14)) * 1e-6)) / (2.0 * assign17690_e31431)))))),)
    } else {
        (locals.var_eta0cv_t, locals.var_eta0cv_t_dn0, locals.var_eta0cv_t_dn2, locals.var_eta0cv_t_dn3, locals.var_eta0cv_t_dn4, locals.var_eta0cv_t_dn5, locals.var_eta0cv_t_dn6, locals.var_eta0cv_t_dn7, locals.var_eta0cv_t_dn8, locals.var_eta0cv_t_dn9, locals.var_eta0cv_t_dn10, locals.var_eta0cv_t_dn11, locals.var_eta0cv_t_dn13, locals.var_eta0cv_t_dn14,)
    }
};
        locals.var_eta0cv_t = assign17690_e31437;
        locals.var_eta0cv_t_dn0 = assign17690_e31437_d_n0;
        locals.var_eta0cv_t_dn2 = assign17690_e31437_d_n2;
        locals.var_eta0cv_t_dn3 = assign17690_e31437_d_n3;
        locals.var_eta0cv_t_dn4 = assign17690_e31437_d_n4;
        locals.var_eta0cv_t_dn5 = assign17690_e31437_d_n5;
        locals.var_eta0cv_t_dn6 = assign17690_e31437_d_n6;
        locals.var_eta0cv_t_dn7 = assign17690_e31437_d_n7;
        locals.var_eta0cv_t_dn8 = assign17690_e31437_d_n8;
        locals.var_eta0cv_t_dn9 = assign17690_e31437_d_n9;
        locals.var_eta0cv_t_dn10 = assign17690_e31437_d_n10;
        locals.var_eta0cv_t_dn11 = assign17690_e31437_d_n11;
        locals.var_eta0cv_t_dn13 = assign17690_e31437_d_n13;
        locals.var_eta0cv_t_dn14 = assign17690_e31437_d_n14;

        let (assign17700_e31517, assign17700_e31517_d_n0, assign17700_e31517_d_n2, assign17700_e31517_d_n3, assign17700_e31517_d_n4, assign17700_e31517_d_n5, assign17700_e31517_d_n6, assign17700_e31517_d_n7, assign17700_e31517_d_n8, assign17700_e31517_d_n9, assign17700_e31517_d_n10, assign17700_e31517_d_n11, assign17700_e31517_d_n13, assign17700_e31517_d_n14,) = {
    if ((locals.var_guard323 != 0.0) && (locals.var_guard324 == 0.0)) {
        let assign17700_e31446: f64 = (p.p165 * locals.var_deltemp);
        let assign17700_e31447: f64 = (1.0 + assign17700_e31446);
        let assign17700_e31449: f64 = (assign17700_e31447 - 1e-6);
        let assign17700_e31451: f64 = (-10000.0);
        let assign17700_e31453: f64 = (assign17700_e31451 * 0.001);
        let (assign17700_e31514, assign17700_e31514_d_n4,) = {
            if (!(assign17700_e31449 < assign17700_e31453)) {
                let assign17700_e31460: f64 = (p.p165 * locals.var_deltemp);
                let assign17700_e31461: f64 = (1.0 + assign17700_e31460);
                let assign17700_e31463: f64 = (assign17700_e31461 - 1e-6);
                let assign17700_e31467: f64 = (p.p165 * locals.var_deltemp);
                let assign17700_e31468: f64 = (1.0 + assign17700_e31467);
                let assign17700_e31470: f64 = (assign17700_e31468 - 1e-6);
                let assign17700_e31474: f64 = (p.p165 * locals.var_deltemp);
                let assign17700_e31475: f64 = (1.0 + assign17700_e31474);
                let assign17700_e31477: f64 = (assign17700_e31475 - 1e-6);
                let assign17700_e31478: f64 = (assign17700_e31470 * assign17700_e31477);
                let assign17700_e31481: f64 = (4.0 * 0.001);
                let assign17700_e31483: f64 = (assign17700_e31481 * 0.001);
                let assign17700_e31484: f64 = (assign17700_e31478 + assign17700_e31483);
                let assign17700_e31485: f64 = (assign17700_e31484).sqrt();
                let assign17700_e31486: f64 = (assign17700_e31463 + assign17700_e31485);
                let assign17700_e31487: f64 = (0.5 * assign17700_e31486);
                (assign17700_e31487, (0.5 * ((p.p165 * locals.var_deltemp_dn4) + ((((p.p165 * locals.var_deltemp_dn4) * assign17700_e31477) + (assign17700_e31470 * (p.p165 * locals.var_deltemp_dn4))) / (2.0 * assign17700_e31485)))),)
            } else {
                let assign17700_e31491: f64 = (p.p165 * locals.var_deltemp);
                let assign17700_e31492: f64 = (1.0 + assign17700_e31491);
                let assign17700_e31494: f64 = (assign17700_e31492 - 1e-6);
                let assign17700_e31496: f64 = (-10000.0);
                let assign17700_e31498: f64 = (assign17700_e31496 * 0.001);
                let (assign17700_e31513, assign17700_e31513_d_n4,) = {
                    if (assign17700_e31494 < assign17700_e31498) {
                        let assign17700_e31501: f64 = (-0.001);
                        let assign17700_e31503: f64 = (assign17700_e31501 * 0.001);
                        let assign17700_e31507: f64 = (p.p165 * locals.var_deltemp);
                        let assign17700_e31508: f64 = (1.0 + assign17700_e31507);
                        let assign17700_e31510: f64 = (assign17700_e31508 - 1e-6);
                        let assign17700_e31511: f64 = (assign17700_e31503 / assign17700_e31510);
                        (assign17700_e31511, (-((assign17700_e31503 * (p.p165 * locals.var_deltemp_dn4)) / (assign17700_e31510 * assign17700_e31510))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17700_e31513, assign17700_e31513_d_n4,)
            }
        };
        let assign17700_e31515: f64 = (locals.var_eta0cv_i * assign17700_e31514);
        (assign17700_e31515, (locals.var_eta0cv_i_dn0 * assign17700_e31514), (locals.var_eta0cv_i_dn2 * assign17700_e31514), (locals.var_eta0cv_i_dn3 * assign17700_e31514), ((locals.var_eta0cv_i_dn4 * assign17700_e31514) + (locals.var_eta0cv_i * assign17700_e31514_d_n4)), (locals.var_eta0cv_i_dn5 * assign17700_e31514), (locals.var_eta0cv_i_dn6 * assign17700_e31514), (locals.var_eta0cv_i_dn7 * assign17700_e31514), (locals.var_eta0cv_i_dn8 * assign17700_e31514), (locals.var_eta0cv_i_dn9 * assign17700_e31514), (locals.var_eta0cv_i_dn10 * assign17700_e31514), (locals.var_eta0cv_i_dn11 * assign17700_e31514), (locals.var_eta0cv_i_dn13 * assign17700_e31514), (locals.var_eta0cv_i_dn14 * assign17700_e31514),)
    } else {
        (locals.var_eta0cv_t, locals.var_eta0cv_t_dn0, locals.var_eta0cv_t_dn2, locals.var_eta0cv_t_dn3, locals.var_eta0cv_t_dn4, locals.var_eta0cv_t_dn5, locals.var_eta0cv_t_dn6, locals.var_eta0cv_t_dn7, locals.var_eta0cv_t_dn8, locals.var_eta0cv_t_dn9, locals.var_eta0cv_t_dn10, locals.var_eta0cv_t_dn11, locals.var_eta0cv_t_dn13, locals.var_eta0cv_t_dn14,)
    }
};
        locals.var_eta0cv_t = assign17700_e31517;
        locals.var_eta0cv_t_dn0 = assign17700_e31517_d_n0;
        locals.var_eta0cv_t_dn2 = assign17700_e31517_d_n2;
        locals.var_eta0cv_t_dn3 = assign17700_e31517_d_n3;
        locals.var_eta0cv_t_dn4 = assign17700_e31517_d_n4;
        locals.var_eta0cv_t_dn5 = assign17700_e31517_d_n5;
        locals.var_eta0cv_t_dn6 = assign17700_e31517_d_n6;
        locals.var_eta0cv_t_dn7 = assign17700_e31517_d_n7;
        locals.var_eta0cv_t_dn8 = assign17700_e31517_d_n8;
        locals.var_eta0cv_t_dn9 = assign17700_e31517_d_n9;
        locals.var_eta0cv_t_dn10 = assign17700_e31517_d_n10;
        locals.var_eta0cv_t_dn11 = assign17700_e31517_d_n11;
        locals.var_eta0cv_t_dn13 = assign17700_e31517_d_n13;
        locals.var_eta0cv_t_dn14 = assign17700_e31517_d_n14;

        let assign17710_e31520: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard325 = assign17710_e31520;

        let (assign17720_e31564, assign17720_e31564_d_n4,) = {
    if (locals.var_guard325 != 0.0) {
        let assign17720_e31524: f64 = (-locals.var_eta0r_i);
        let assign17720_e31528: f64 = (p.p166 * locals.var_deltemp);
        let assign17720_e31530: f64 = (-locals.var_eta0r_i);
        let assign17720_e31531: f64 = (assign17720_e31528 - assign17720_e31530);
        let assign17720_e31533: f64 = (assign17720_e31531 - 1e-6);
        let assign17720_e31536: f64 = (p.p166 * locals.var_deltemp);
        let assign17720_e31538: f64 = (-locals.var_eta0r_i);
        let assign17720_e31539: f64 = (assign17720_e31536 - assign17720_e31538);
        let assign17720_e31541: f64 = (assign17720_e31539 - 1e-6);
        let assign17720_e31544: f64 = (p.p166 * locals.var_deltemp);
        let assign17720_e31546: f64 = (-locals.var_eta0r_i);
        let assign17720_e31547: f64 = (assign17720_e31544 - assign17720_e31546);
        let assign17720_e31549: f64 = (assign17720_e31547 - 1e-6);
        let assign17720_e31550: f64 = (assign17720_e31541 * assign17720_e31549);
        let assign17720_e31553: f64 = (-locals.var_eta0r_i);
        let assign17720_e31554: f64 = (4.0 * assign17720_e31553);
        let assign17720_e31556: f64 = (assign17720_e31554 * 1e-6);
        let assign17720_e31557: f64 = (assign17720_e31550 - assign17720_e31556);
        let assign17720_e31558: f64 = (assign17720_e31557).sqrt();
        let assign17720_e31559: f64 = (assign17720_e31533 + assign17720_e31558);
        let assign17720_e31560: f64 = (0.5 * assign17720_e31559);
        let assign17720_e31561: f64 = (assign17720_e31524 + assign17720_e31560);
        let assign17720_e31562: f64 = (locals.var_eta0r_i + assign17720_e31561);
        (assign17720_e31562, (0.5 * ((p.p166 * locals.var_deltemp_dn4) + ((((p.p166 * locals.var_deltemp_dn4) * assign17720_e31549) + (assign17720_e31541 * (p.p166 * locals.var_deltemp_dn4))) / (2.0 * assign17720_e31558)))),)
    } else {
        (locals.var_eta0r_t, locals.var_eta0r_t_dn4,)
    }
};
        locals.var_eta0r_t = assign17720_e31564;
        locals.var_eta0r_t_dn4 = assign17720_e31564_d_n4;

        let (assign17730_e31642, assign17730_e31642_d_n4,) = {
    if (locals.var_guard325 == 0.0) {
        let assign17730_e31571: f64 = (p.p166 * locals.var_deltemp);
        let assign17730_e31572: f64 = (1.0 + assign17730_e31571);
        let assign17730_e31574: f64 = (assign17730_e31572 - 1e-6);
        let assign17730_e31576: f64 = (-10000.0);
        let assign17730_e31578: f64 = (assign17730_e31576 * 0.001);
        let (assign17730_e31639, assign17730_e31639_d_n4,) = {
            if (!(assign17730_e31574 < assign17730_e31578)) {
                let assign17730_e31585: f64 = (p.p166 * locals.var_deltemp);
                let assign17730_e31586: f64 = (1.0 + assign17730_e31585);
                let assign17730_e31588: f64 = (assign17730_e31586 - 1e-6);
                let assign17730_e31592: f64 = (p.p166 * locals.var_deltemp);
                let assign17730_e31593: f64 = (1.0 + assign17730_e31592);
                let assign17730_e31595: f64 = (assign17730_e31593 - 1e-6);
                let assign17730_e31599: f64 = (p.p166 * locals.var_deltemp);
                let assign17730_e31600: f64 = (1.0 + assign17730_e31599);
                let assign17730_e31602: f64 = (assign17730_e31600 - 1e-6);
                let assign17730_e31603: f64 = (assign17730_e31595 * assign17730_e31602);
                let assign17730_e31606: f64 = (4.0 * 0.001);
                let assign17730_e31608: f64 = (assign17730_e31606 * 0.001);
                let assign17730_e31609: f64 = (assign17730_e31603 + assign17730_e31608);
                let assign17730_e31610: f64 = (assign17730_e31609).sqrt();
                let assign17730_e31611: f64 = (assign17730_e31588 + assign17730_e31610);
                let assign17730_e31612: f64 = (0.5 * assign17730_e31611);
                (assign17730_e31612, (0.5 * ((p.p166 * locals.var_deltemp_dn4) + ((((p.p166 * locals.var_deltemp_dn4) * assign17730_e31602) + (assign17730_e31595 * (p.p166 * locals.var_deltemp_dn4))) / (2.0 * assign17730_e31610)))),)
            } else {
                let assign17730_e31616: f64 = (p.p166 * locals.var_deltemp);
                let assign17730_e31617: f64 = (1.0 + assign17730_e31616);
                let assign17730_e31619: f64 = (assign17730_e31617 - 1e-6);
                let assign17730_e31621: f64 = (-10000.0);
                let assign17730_e31623: f64 = (assign17730_e31621 * 0.001);
                let (assign17730_e31638, assign17730_e31638_d_n4,) = {
                    if (assign17730_e31619 < assign17730_e31623) {
                        let assign17730_e31626: f64 = (-0.001);
                        let assign17730_e31628: f64 = (assign17730_e31626 * 0.001);
                        let assign17730_e31632: f64 = (p.p166 * locals.var_deltemp);
                        let assign17730_e31633: f64 = (1.0 + assign17730_e31632);
                        let assign17730_e31635: f64 = (assign17730_e31633 - 1e-6);
                        let assign17730_e31636: f64 = (assign17730_e31628 / assign17730_e31635);
                        (assign17730_e31636, (-((assign17730_e31628 * (p.p166 * locals.var_deltemp_dn4)) / (assign17730_e31635 * assign17730_e31635))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17730_e31638, assign17730_e31638_d_n4,)
            }
        };
        let assign17730_e31640: f64 = (locals.var_eta0r_i * assign17730_e31639);
        (assign17730_e31640, (locals.var_eta0r_i * assign17730_e31639_d_n4),)
    } else {
        (locals.var_eta0r_t, locals.var_eta0r_t_dn4,)
    }
};
        locals.var_eta0r_t = assign17730_e31642;
        locals.var_eta0r_t_dn4 = assign17730_e31642_d_n4;

        let assign17740_e31645: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard326 = assign17740_e31645;

        let (assign17750_e31689, assign17750_e31689_d_n4,) = {
    if (locals.var_guard326 != 0.0) {
        let assign17750_e31649: f64 = (-locals.var_etamob_i);
        let assign17750_e31653: f64 = (locals.var_emobt_i * locals.var_deltemp);
        let assign17750_e31655: f64 = (-locals.var_etamob_i);
        let assign17750_e31656: f64 = (assign17750_e31653 - assign17750_e31655);
        let assign17750_e31658: f64 = (assign17750_e31656 - 1e-6);
        let assign17750_e31661: f64 = (locals.var_emobt_i * locals.var_deltemp);
        let assign17750_e31663: f64 = (-locals.var_etamob_i);
        let assign17750_e31664: f64 = (assign17750_e31661 - assign17750_e31663);
        let assign17750_e31666: f64 = (assign17750_e31664 - 1e-6);
        let assign17750_e31669: f64 = (locals.var_emobt_i * locals.var_deltemp);
        let assign17750_e31671: f64 = (-locals.var_etamob_i);
        let assign17750_e31672: f64 = (assign17750_e31669 - assign17750_e31671);
        let assign17750_e31674: f64 = (assign17750_e31672 - 1e-6);
        let assign17750_e31675: f64 = (assign17750_e31666 * assign17750_e31674);
        let assign17750_e31678: f64 = (-locals.var_etamob_i);
        let assign17750_e31679: f64 = (4.0 * assign17750_e31678);
        let assign17750_e31681: f64 = (assign17750_e31679 * 1e-6);
        let assign17750_e31682: f64 = (assign17750_e31675 - assign17750_e31681);
        let assign17750_e31683: f64 = (assign17750_e31682).sqrt();
        let assign17750_e31684: f64 = (assign17750_e31658 + assign17750_e31683);
        let assign17750_e31685: f64 = (0.5 * assign17750_e31684);
        let assign17750_e31686: f64 = (assign17750_e31649 + assign17750_e31685);
        let assign17750_e31687: f64 = (locals.var_etamob_i + assign17750_e31686);
        (assign17750_e31687, (0.5 * ((locals.var_emobt_i * locals.var_deltemp_dn4) + ((((locals.var_emobt_i * locals.var_deltemp_dn4) * assign17750_e31674) + (assign17750_e31666 * (locals.var_emobt_i * locals.var_deltemp_dn4))) / (2.0 * assign17750_e31683)))),)
    } else {
        (locals.var_etamob_t, locals.var_etamob_t_dn4,)
    }
};
        locals.var_etamob_t = assign17750_e31689;
        locals.var_etamob_t_dn4 = assign17750_e31689_d_n4;

        let (assign17760_e31767, assign17760_e31767_d_n4,) = {
    if (locals.var_guard326 == 0.0) {
        let assign17760_e31696: f64 = (locals.var_emobt_i * locals.var_deltemp);
        let assign17760_e31697: f64 = (1.0 + assign17760_e31696);
        let assign17760_e31699: f64 = (assign17760_e31697 - 1e-6);
        let assign17760_e31701: f64 = (-10000.0);
        let assign17760_e31703: f64 = (assign17760_e31701 * 0.001);
        let (assign17760_e31764, assign17760_e31764_d_n4,) = {
            if (!(assign17760_e31699 < assign17760_e31703)) {
                let assign17760_e31710: f64 = (locals.var_emobt_i * locals.var_deltemp);
                let assign17760_e31711: f64 = (1.0 + assign17760_e31710);
                let assign17760_e31713: f64 = (assign17760_e31711 - 1e-6);
                let assign17760_e31717: f64 = (locals.var_emobt_i * locals.var_deltemp);
                let assign17760_e31718: f64 = (1.0 + assign17760_e31717);
                let assign17760_e31720: f64 = (assign17760_e31718 - 1e-6);
                let assign17760_e31724: f64 = (locals.var_emobt_i * locals.var_deltemp);
                let assign17760_e31725: f64 = (1.0 + assign17760_e31724);
                let assign17760_e31727: f64 = (assign17760_e31725 - 1e-6);
                let assign17760_e31728: f64 = (assign17760_e31720 * assign17760_e31727);
                let assign17760_e31731: f64 = (4.0 * 0.001);
                let assign17760_e31733: f64 = (assign17760_e31731 * 0.001);
                let assign17760_e31734: f64 = (assign17760_e31728 + assign17760_e31733);
                let assign17760_e31735: f64 = (assign17760_e31734).sqrt();
                let assign17760_e31736: f64 = (assign17760_e31713 + assign17760_e31735);
                let assign17760_e31737: f64 = (0.5 * assign17760_e31736);
                (assign17760_e31737, (0.5 * ((locals.var_emobt_i * locals.var_deltemp_dn4) + ((((locals.var_emobt_i * locals.var_deltemp_dn4) * assign17760_e31727) + (assign17760_e31720 * (locals.var_emobt_i * locals.var_deltemp_dn4))) / (2.0 * assign17760_e31735)))),)
            } else {
                let assign17760_e31741: f64 = (locals.var_emobt_i * locals.var_deltemp);
                let assign17760_e31742: f64 = (1.0 + assign17760_e31741);
                let assign17760_e31744: f64 = (assign17760_e31742 - 1e-6);
                let assign17760_e31746: f64 = (-10000.0);
                let assign17760_e31748: f64 = (assign17760_e31746 * 0.001);
                let (assign17760_e31763, assign17760_e31763_d_n4,) = {
                    if (assign17760_e31744 < assign17760_e31748) {
                        let assign17760_e31751: f64 = (-0.001);
                        let assign17760_e31753: f64 = (assign17760_e31751 * 0.001);
                        let assign17760_e31757: f64 = (locals.var_emobt_i * locals.var_deltemp);
                        let assign17760_e31758: f64 = (1.0 + assign17760_e31757);
                        let assign17760_e31760: f64 = (assign17760_e31758 - 1e-6);
                        let assign17760_e31761: f64 = (assign17760_e31753 / assign17760_e31760);
                        (assign17760_e31761, (-((assign17760_e31753 * (locals.var_emobt_i * locals.var_deltemp_dn4)) / (assign17760_e31760 * assign17760_e31760))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17760_e31763, assign17760_e31763_d_n4,)
            }
        };
        let assign17760_e31765: f64 = (locals.var_etamob_i * assign17760_e31764);
        (assign17760_e31765, (locals.var_etamob_i * assign17760_e31764_d_n4),)
    } else {
        (locals.var_etamob_t, locals.var_etamob_t_dn4,)
    }
};
        locals.var_etamob_t = assign17760_e31767;
        locals.var_etamob_t_dn4 = assign17760_e31767_d_n4;

        let assign17770_e31770: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard327 = assign17770_e31770;

        let (assign17780_e31814, assign17780_e31814_d_n4,) = {
    if (locals.var_guard327 != 0.0) {
        let assign17780_e31774: f64 = (-p.p917);
        let assign17780_e31778: f64 = (p.p923 * locals.var_deltemp);
        let assign17780_e31780: f64 = (-p.p917);
        let assign17780_e31781: f64 = (assign17780_e31778 - assign17780_e31780);
        let assign17780_e31783: f64 = (assign17780_e31781 - 1e-6);
        let assign17780_e31786: f64 = (p.p923 * locals.var_deltemp);
        let assign17780_e31788: f64 = (-p.p917);
        let assign17780_e31789: f64 = (assign17780_e31786 - assign17780_e31788);
        let assign17780_e31791: f64 = (assign17780_e31789 - 1e-6);
        let assign17780_e31794: f64 = (p.p923 * locals.var_deltemp);
        let assign17780_e31796: f64 = (-p.p917);
        let assign17780_e31797: f64 = (assign17780_e31794 - assign17780_e31796);
        let assign17780_e31799: f64 = (assign17780_e31797 - 1e-6);
        let assign17780_e31800: f64 = (assign17780_e31791 * assign17780_e31799);
        let assign17780_e31803: f64 = (-p.p917);
        let assign17780_e31804: f64 = (4.0 * assign17780_e31803);
        let assign17780_e31806: f64 = (assign17780_e31804 * 1e-6);
        let assign17780_e31807: f64 = (assign17780_e31800 - assign17780_e31806);
        let assign17780_e31808: f64 = (assign17780_e31807).sqrt();
        let assign17780_e31809: f64 = (assign17780_e31783 + assign17780_e31808);
        let assign17780_e31810: f64 = (0.5 * assign17780_e31809);
        let assign17780_e31811: f64 = (assign17780_e31774 + assign17780_e31810);
        let assign17780_e31812: f64 = (p.p917 + assign17780_e31811);
        (assign17780_e31812, (0.5 * ((p.p923 * locals.var_deltemp_dn4) + ((((p.p923 * locals.var_deltemp_dn4) * assign17780_e31799) + (assign17780_e31791 * (p.p923 * locals.var_deltemp_dn4))) / (2.0 * assign17780_e31808)))),)
    } else {
        (locals.var_rsdr_t, locals.var_rsdr_t_dn4,)
    }
};
        locals.var_rsdr_t = assign17780_e31814;
        locals.var_rsdr_t_dn4 = assign17780_e31814_d_n4;

    }

    pub(super) fn stamp_transient_block_58(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17790_e31892, assign17790_e31892_d_n4,) = {
    if (locals.var_guard327 == 0.0) {
        let assign17790_e31821: f64 = (p.p923 * locals.var_deltemp);
        let assign17790_e31822: f64 = (1.0 + assign17790_e31821);
        let assign17790_e31824: f64 = (assign17790_e31822 - 1e-6);
        let assign17790_e31826: f64 = (-10000.0);
        let assign17790_e31828: f64 = (assign17790_e31826 * 0.001);
        let (assign17790_e31889, assign17790_e31889_d_n4,) = {
            if (!(assign17790_e31824 < assign17790_e31828)) {
                let assign17790_e31835: f64 = (p.p923 * locals.var_deltemp);
                let assign17790_e31836: f64 = (1.0 + assign17790_e31835);
                let assign17790_e31838: f64 = (assign17790_e31836 - 1e-6);
                let assign17790_e31842: f64 = (p.p923 * locals.var_deltemp);
                let assign17790_e31843: f64 = (1.0 + assign17790_e31842);
                let assign17790_e31845: f64 = (assign17790_e31843 - 1e-6);
                let assign17790_e31849: f64 = (p.p923 * locals.var_deltemp);
                let assign17790_e31850: f64 = (1.0 + assign17790_e31849);
                let assign17790_e31852: f64 = (assign17790_e31850 - 1e-6);
                let assign17790_e31853: f64 = (assign17790_e31845 * assign17790_e31852);
                let assign17790_e31856: f64 = (4.0 * 0.001);
                let assign17790_e31858: f64 = (assign17790_e31856 * 0.001);
                let assign17790_e31859: f64 = (assign17790_e31853 + assign17790_e31858);
                let assign17790_e31860: f64 = (assign17790_e31859).sqrt();
                let assign17790_e31861: f64 = (assign17790_e31838 + assign17790_e31860);
                let assign17790_e31862: f64 = (0.5 * assign17790_e31861);
                (assign17790_e31862, (0.5 * ((p.p923 * locals.var_deltemp_dn4) + ((((p.p923 * locals.var_deltemp_dn4) * assign17790_e31852) + (assign17790_e31845 * (p.p923 * locals.var_deltemp_dn4))) / (2.0 * assign17790_e31860)))),)
            } else {
                let assign17790_e31866: f64 = (p.p923 * locals.var_deltemp);
                let assign17790_e31867: f64 = (1.0 + assign17790_e31866);
                let assign17790_e31869: f64 = (assign17790_e31867 - 1e-6);
                let assign17790_e31871: f64 = (-10000.0);
                let assign17790_e31873: f64 = (assign17790_e31871 * 0.001);
                let (assign17790_e31888, assign17790_e31888_d_n4,) = {
                    if (assign17790_e31869 < assign17790_e31873) {
                        let assign17790_e31876: f64 = (-0.001);
                        let assign17790_e31878: f64 = (assign17790_e31876 * 0.001);
                        let assign17790_e31882: f64 = (p.p923 * locals.var_deltemp);
                        let assign17790_e31883: f64 = (1.0 + assign17790_e31882);
                        let assign17790_e31885: f64 = (assign17790_e31883 - 1e-6);
                        let assign17790_e31886: f64 = (assign17790_e31878 / assign17790_e31885);
                        (assign17790_e31886, (-((assign17790_e31878 * (p.p923 * locals.var_deltemp_dn4)) / (assign17790_e31885 * assign17790_e31885))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17790_e31888, assign17790_e31888_d_n4,)
            }
        };
        let assign17790_e31890: f64 = (p.p917 * assign17790_e31889);
        (assign17790_e31890, (p.p917 * assign17790_e31889_d_n4),)
    } else {
        (locals.var_rsdr_t, locals.var_rsdr_t_dn4,)
    }
};
        locals.var_rsdr_t = assign17790_e31892;
        locals.var_rsdr_t_dn4 = assign17790_e31892_d_n4;

        let assign17800_e31895: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard328 = assign17800_e31895;

        let assign17810_e31898: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard329 = assign17810_e31898;

        let (assign17820_e31944, assign17820_e31944_d_n4,) = {
    if ((locals.var_guard328 != 0.0) && (locals.var_guard329 != 0.0)) {
        let assign17820_e31904: f64 = (-p.p918);
        let assign17820_e31908: f64 = (p.p923 * locals.var_deltemp);
        let assign17820_e31910: f64 = (-p.p918);
        let assign17820_e31911: f64 = (assign17820_e31908 - assign17820_e31910);
        let assign17820_e31913: f64 = (assign17820_e31911 - 1e-6);
        let assign17820_e31916: f64 = (p.p923 * locals.var_deltemp);
        let assign17820_e31918: f64 = (-p.p918);
        let assign17820_e31919: f64 = (assign17820_e31916 - assign17820_e31918);
        let assign17820_e31921: f64 = (assign17820_e31919 - 1e-6);
        let assign17820_e31924: f64 = (p.p923 * locals.var_deltemp);
        let assign17820_e31926: f64 = (-p.p918);
        let assign17820_e31927: f64 = (assign17820_e31924 - assign17820_e31926);
        let assign17820_e31929: f64 = (assign17820_e31927 - 1e-6);
        let assign17820_e31930: f64 = (assign17820_e31921 * assign17820_e31929);
        let assign17820_e31933: f64 = (-p.p918);
        let assign17820_e31934: f64 = (4.0 * assign17820_e31933);
        let assign17820_e31936: f64 = (assign17820_e31934 * 1e-6);
        let assign17820_e31937: f64 = (assign17820_e31930 - assign17820_e31936);
        let assign17820_e31938: f64 = (assign17820_e31937).sqrt();
        let assign17820_e31939: f64 = (assign17820_e31913 + assign17820_e31938);
        let assign17820_e31940: f64 = (0.5 * assign17820_e31939);
        let assign17820_e31941: f64 = (assign17820_e31904 + assign17820_e31940);
        let assign17820_e31942: f64 = (p.p918 + assign17820_e31941);
        (assign17820_e31942, (0.5 * ((p.p923 * locals.var_deltemp_dn4) + ((((p.p923 * locals.var_deltemp_dn4) * assign17820_e31929) + (assign17820_e31921 * (p.p923 * locals.var_deltemp_dn4))) / (2.0 * assign17820_e31938)))),)
    } else {
        (locals.var_rsdrr_t, locals.var_rsdrr_t_dn4,)
    }
};
        locals.var_rsdrr_t = assign17820_e31944;
        locals.var_rsdrr_t_dn4 = assign17820_e31944_d_n4;

        let (assign17830_e32024, assign17830_e32024_d_n4,) = {
    if ((locals.var_guard328 != 0.0) && (locals.var_guard329 == 0.0)) {
        let assign17830_e31953: f64 = (p.p923 * locals.var_deltemp);
        let assign17830_e31954: f64 = (1.0 + assign17830_e31953);
        let assign17830_e31956: f64 = (assign17830_e31954 - 1e-6);
        let assign17830_e31958: f64 = (-10000.0);
        let assign17830_e31960: f64 = (assign17830_e31958 * 0.001);
        let (assign17830_e32021, assign17830_e32021_d_n4,) = {
            if (!(assign17830_e31956 < assign17830_e31960)) {
                let assign17830_e31967: f64 = (p.p923 * locals.var_deltemp);
                let assign17830_e31968: f64 = (1.0 + assign17830_e31967);
                let assign17830_e31970: f64 = (assign17830_e31968 - 1e-6);
                let assign17830_e31974: f64 = (p.p923 * locals.var_deltemp);
                let assign17830_e31975: f64 = (1.0 + assign17830_e31974);
                let assign17830_e31977: f64 = (assign17830_e31975 - 1e-6);
                let assign17830_e31981: f64 = (p.p923 * locals.var_deltemp);
                let assign17830_e31982: f64 = (1.0 + assign17830_e31981);
                let assign17830_e31984: f64 = (assign17830_e31982 - 1e-6);
                let assign17830_e31985: f64 = (assign17830_e31977 * assign17830_e31984);
                let assign17830_e31988: f64 = (4.0 * 0.001);
                let assign17830_e31990: f64 = (assign17830_e31988 * 0.001);
                let assign17830_e31991: f64 = (assign17830_e31985 + assign17830_e31990);
                let assign17830_e31992: f64 = (assign17830_e31991).sqrt();
                let assign17830_e31993: f64 = (assign17830_e31970 + assign17830_e31992);
                let assign17830_e31994: f64 = (0.5 * assign17830_e31993);
                (assign17830_e31994, (0.5 * ((p.p923 * locals.var_deltemp_dn4) + ((((p.p923 * locals.var_deltemp_dn4) * assign17830_e31984) + (assign17830_e31977 * (p.p923 * locals.var_deltemp_dn4))) / (2.0 * assign17830_e31992)))),)
            } else {
                let assign17830_e31998: f64 = (p.p923 * locals.var_deltemp);
                let assign17830_e31999: f64 = (1.0 + assign17830_e31998);
                let assign17830_e32001: f64 = (assign17830_e31999 - 1e-6);
                let assign17830_e32003: f64 = (-10000.0);
                let assign17830_e32005: f64 = (assign17830_e32003 * 0.001);
                let (assign17830_e32020, assign17830_e32020_d_n4,) = {
                    if (assign17830_e32001 < assign17830_e32005) {
                        let assign17830_e32008: f64 = (-0.001);
                        let assign17830_e32010: f64 = (assign17830_e32008 * 0.001);
                        let assign17830_e32014: f64 = (p.p923 * locals.var_deltemp);
                        let assign17830_e32015: f64 = (1.0 + assign17830_e32014);
                        let assign17830_e32017: f64 = (assign17830_e32015 - 1e-6);
                        let assign17830_e32018: f64 = (assign17830_e32010 / assign17830_e32017);
                        (assign17830_e32018, (-((assign17830_e32010 * (p.p923 * locals.var_deltemp_dn4)) / (assign17830_e32017 * assign17830_e32017))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17830_e32020, assign17830_e32020_d_n4,)
            }
        };
        let assign17830_e32022: f64 = (p.p918 * assign17830_e32021);
        (assign17830_e32022, (p.p918 * assign17830_e32021_d_n4),)
    } else {
        (locals.var_rsdrr_t, locals.var_rsdrr_t_dn4,)
    }
};
        locals.var_rsdrr_t = assign17830_e32024;
        locals.var_rsdrr_t_dn4 = assign17830_e32024_d_n4;

        let assign17840_e32027: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard330 = assign17840_e32027;

        let (assign17850_e32071, assign17850_e32071_d_n4,) = {
    if (locals.var_guard330 != 0.0) {
        let assign17850_e32031: f64 = (-p.p919);
        let assign17850_e32035: f64 = (p.p924 * locals.var_deltemp);
        let assign17850_e32037: f64 = (-p.p919);
        let assign17850_e32038: f64 = (assign17850_e32035 - assign17850_e32037);
        let assign17850_e32040: f64 = (assign17850_e32038 - 1e-6);
        let assign17850_e32043: f64 = (p.p924 * locals.var_deltemp);
        let assign17850_e32045: f64 = (-p.p919);
        let assign17850_e32046: f64 = (assign17850_e32043 - assign17850_e32045);
        let assign17850_e32048: f64 = (assign17850_e32046 - 1e-6);
        let assign17850_e32051: f64 = (p.p924 * locals.var_deltemp);
        let assign17850_e32053: f64 = (-p.p919);
        let assign17850_e32054: f64 = (assign17850_e32051 - assign17850_e32053);
        let assign17850_e32056: f64 = (assign17850_e32054 - 1e-6);
        let assign17850_e32057: f64 = (assign17850_e32048 * assign17850_e32056);
        let assign17850_e32060: f64 = (-p.p919);
        let assign17850_e32061: f64 = (4.0 * assign17850_e32060);
        let assign17850_e32063: f64 = (assign17850_e32061 * 1e-6);
        let assign17850_e32064: f64 = (assign17850_e32057 - assign17850_e32063);
        let assign17850_e32065: f64 = (assign17850_e32064).sqrt();
        let assign17850_e32066: f64 = (assign17850_e32040 + assign17850_e32065);
        let assign17850_e32067: f64 = (0.5 * assign17850_e32066);
        let assign17850_e32068: f64 = (assign17850_e32031 + assign17850_e32067);
        let assign17850_e32069: f64 = (p.p919 + assign17850_e32068);
        (assign17850_e32069, (0.5 * ((p.p924 * locals.var_deltemp_dn4) + ((((p.p924 * locals.var_deltemp_dn4) * assign17850_e32056) + (assign17850_e32048 * (p.p924 * locals.var_deltemp_dn4))) / (2.0 * assign17850_e32065)))),)
    } else {
        (locals.var_rddr_t, locals.var_rddr_t_dn4,)
    }
};
        locals.var_rddr_t = assign17850_e32071;
        locals.var_rddr_t_dn4 = assign17850_e32071_d_n4;

        let (assign17860_e32149, assign17860_e32149_d_n4,) = {
    if (locals.var_guard330 == 0.0) {
        let assign17860_e32078: f64 = (p.p924 * locals.var_deltemp);
        let assign17860_e32079: f64 = (1.0 + assign17860_e32078);
        let assign17860_e32081: f64 = (assign17860_e32079 - 1e-6);
        let assign17860_e32083: f64 = (-10000.0);
        let assign17860_e32085: f64 = (assign17860_e32083 * 0.001);
        let (assign17860_e32146, assign17860_e32146_d_n4,) = {
            if (!(assign17860_e32081 < assign17860_e32085)) {
                let assign17860_e32092: f64 = (p.p924 * locals.var_deltemp);
                let assign17860_e32093: f64 = (1.0 + assign17860_e32092);
                let assign17860_e32095: f64 = (assign17860_e32093 - 1e-6);
                let assign17860_e32099: f64 = (p.p924 * locals.var_deltemp);
                let assign17860_e32100: f64 = (1.0 + assign17860_e32099);
                let assign17860_e32102: f64 = (assign17860_e32100 - 1e-6);
                let assign17860_e32106: f64 = (p.p924 * locals.var_deltemp);
                let assign17860_e32107: f64 = (1.0 + assign17860_e32106);
                let assign17860_e32109: f64 = (assign17860_e32107 - 1e-6);
                let assign17860_e32110: f64 = (assign17860_e32102 * assign17860_e32109);
                let assign17860_e32113: f64 = (4.0 * 0.001);
                let assign17860_e32115: f64 = (assign17860_e32113 * 0.001);
                let assign17860_e32116: f64 = (assign17860_e32110 + assign17860_e32115);
                let assign17860_e32117: f64 = (assign17860_e32116).sqrt();
                let assign17860_e32118: f64 = (assign17860_e32095 + assign17860_e32117);
                let assign17860_e32119: f64 = (0.5 * assign17860_e32118);
                (assign17860_e32119, (0.5 * ((p.p924 * locals.var_deltemp_dn4) + ((((p.p924 * locals.var_deltemp_dn4) * assign17860_e32109) + (assign17860_e32102 * (p.p924 * locals.var_deltemp_dn4))) / (2.0 * assign17860_e32117)))),)
            } else {
                let assign17860_e32123: f64 = (p.p924 * locals.var_deltemp);
                let assign17860_e32124: f64 = (1.0 + assign17860_e32123);
                let assign17860_e32126: f64 = (assign17860_e32124 - 1e-6);
                let assign17860_e32128: f64 = (-10000.0);
                let assign17860_e32130: f64 = (assign17860_e32128 * 0.001);
                let (assign17860_e32145, assign17860_e32145_d_n4,) = {
                    if (assign17860_e32126 < assign17860_e32130) {
                        let assign17860_e32133: f64 = (-0.001);
                        let assign17860_e32135: f64 = (assign17860_e32133 * 0.001);
                        let assign17860_e32139: f64 = (p.p924 * locals.var_deltemp);
                        let assign17860_e32140: f64 = (1.0 + assign17860_e32139);
                        let assign17860_e32142: f64 = (assign17860_e32140 - 1e-6);
                        let assign17860_e32143: f64 = (assign17860_e32135 / assign17860_e32142);
                        (assign17860_e32143, (-((assign17860_e32135 * (p.p924 * locals.var_deltemp_dn4)) / (assign17860_e32142 * assign17860_e32142))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17860_e32145, assign17860_e32145_d_n4,)
            }
        };
        let assign17860_e32147: f64 = (p.p919 * assign17860_e32146);
        (assign17860_e32147, (p.p919 * assign17860_e32146_d_n4),)
    } else {
        (locals.var_rddr_t, locals.var_rddr_t_dn4,)
    }
};
        locals.var_rddr_t = assign17860_e32149;
        locals.var_rddr_t_dn4 = assign17860_e32149_d_n4;

        let assign17870_e32152: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard331 = assign17870_e32152;

        let assign17880_e32155: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard332 = assign17880_e32155;

        let (assign17890_e32201, assign17890_e32201_d_n4,) = {
    if ((locals.var_guard331 != 0.0) && (locals.var_guard332 != 0.0)) {
        let assign17890_e32161: f64 = (-p.p920);
        let assign17890_e32165: f64 = (p.p924 * locals.var_deltemp);
        let assign17890_e32167: f64 = (-p.p920);
        let assign17890_e32168: f64 = (assign17890_e32165 - assign17890_e32167);
        let assign17890_e32170: f64 = (assign17890_e32168 - 1e-6);
        let assign17890_e32173: f64 = (p.p924 * locals.var_deltemp);
        let assign17890_e32175: f64 = (-p.p920);
        let assign17890_e32176: f64 = (assign17890_e32173 - assign17890_e32175);
        let assign17890_e32178: f64 = (assign17890_e32176 - 1e-6);
        let assign17890_e32181: f64 = (p.p924 * locals.var_deltemp);
        let assign17890_e32183: f64 = (-p.p920);
        let assign17890_e32184: f64 = (assign17890_e32181 - assign17890_e32183);
        let assign17890_e32186: f64 = (assign17890_e32184 - 1e-6);
        let assign17890_e32187: f64 = (assign17890_e32178 * assign17890_e32186);
        let assign17890_e32190: f64 = (-p.p920);
        let assign17890_e32191: f64 = (4.0 * assign17890_e32190);
        let assign17890_e32193: f64 = (assign17890_e32191 * 1e-6);
        let assign17890_e32194: f64 = (assign17890_e32187 - assign17890_e32193);
        let assign17890_e32195: f64 = (assign17890_e32194).sqrt();
        let assign17890_e32196: f64 = (assign17890_e32170 + assign17890_e32195);
        let assign17890_e32197: f64 = (0.5 * assign17890_e32196);
        let assign17890_e32198: f64 = (assign17890_e32161 + assign17890_e32197);
        let assign17890_e32199: f64 = (p.p920 + assign17890_e32198);
        (assign17890_e32199, (0.5 * ((p.p924 * locals.var_deltemp_dn4) + ((((p.p924 * locals.var_deltemp_dn4) * assign17890_e32186) + (assign17890_e32178 * (p.p924 * locals.var_deltemp_dn4))) / (2.0 * assign17890_e32195)))),)
    } else {
        (locals.var_rddrr_t, locals.var_rddrr_t_dn4,)
    }
};
        locals.var_rddrr_t = assign17890_e32201;
        locals.var_rddrr_t_dn4 = assign17890_e32201_d_n4;

        let (assign17900_e32281, assign17900_e32281_d_n4,) = {
    if ((locals.var_guard331 != 0.0) && (locals.var_guard332 == 0.0)) {
        let assign17900_e32210: f64 = (p.p924 * locals.var_deltemp);
        let assign17900_e32211: f64 = (1.0 + assign17900_e32210);
        let assign17900_e32213: f64 = (assign17900_e32211 - 1e-6);
        let assign17900_e32215: f64 = (-10000.0);
        let assign17900_e32217: f64 = (assign17900_e32215 * 0.001);
        let (assign17900_e32278, assign17900_e32278_d_n4,) = {
            if (!(assign17900_e32213 < assign17900_e32217)) {
                let assign17900_e32224: f64 = (p.p924 * locals.var_deltemp);
                let assign17900_e32225: f64 = (1.0 + assign17900_e32224);
                let assign17900_e32227: f64 = (assign17900_e32225 - 1e-6);
                let assign17900_e32231: f64 = (p.p924 * locals.var_deltemp);
                let assign17900_e32232: f64 = (1.0 + assign17900_e32231);
                let assign17900_e32234: f64 = (assign17900_e32232 - 1e-6);
                let assign17900_e32238: f64 = (p.p924 * locals.var_deltemp);
                let assign17900_e32239: f64 = (1.0 + assign17900_e32238);
                let assign17900_e32241: f64 = (assign17900_e32239 - 1e-6);
                let assign17900_e32242: f64 = (assign17900_e32234 * assign17900_e32241);
                let assign17900_e32245: f64 = (4.0 * 0.001);
                let assign17900_e32247: f64 = (assign17900_e32245 * 0.001);
                let assign17900_e32248: f64 = (assign17900_e32242 + assign17900_e32247);
                let assign17900_e32249: f64 = (assign17900_e32248).sqrt();
                let assign17900_e32250: f64 = (assign17900_e32227 + assign17900_e32249);
                let assign17900_e32251: f64 = (0.5 * assign17900_e32250);
                (assign17900_e32251, (0.5 * ((p.p924 * locals.var_deltemp_dn4) + ((((p.p924 * locals.var_deltemp_dn4) * assign17900_e32241) + (assign17900_e32234 * (p.p924 * locals.var_deltemp_dn4))) / (2.0 * assign17900_e32249)))),)
            } else {
                let assign17900_e32255: f64 = (p.p924 * locals.var_deltemp);
                let assign17900_e32256: f64 = (1.0 + assign17900_e32255);
                let assign17900_e32258: f64 = (assign17900_e32256 - 1e-6);
                let assign17900_e32260: f64 = (-10000.0);
                let assign17900_e32262: f64 = (assign17900_e32260 * 0.001);
                let (assign17900_e32277, assign17900_e32277_d_n4,) = {
                    if (assign17900_e32258 < assign17900_e32262) {
                        let assign17900_e32265: f64 = (-0.001);
                        let assign17900_e32267: f64 = (assign17900_e32265 * 0.001);
                        let assign17900_e32271: f64 = (p.p924 * locals.var_deltemp);
                        let assign17900_e32272: f64 = (1.0 + assign17900_e32271);
                        let assign17900_e32274: f64 = (assign17900_e32272 - 1e-6);
                        let assign17900_e32275: f64 = (assign17900_e32267 / assign17900_e32274);
                        (assign17900_e32275, (-((assign17900_e32267 * (p.p924 * locals.var_deltemp_dn4)) / (assign17900_e32274 * assign17900_e32274))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17900_e32277, assign17900_e32277_d_n4,)
            }
        };
        let assign17900_e32279: f64 = (p.p920 * assign17900_e32278);
        (assign17900_e32279, (p.p920 * assign17900_e32278_d_n4),)
    } else {
        (locals.var_rddrr_t, locals.var_rddrr_t_dn4,)
    }
};
        locals.var_rddrr_t = assign17900_e32281;
        locals.var_rddrr_t_dn4 = assign17900_e32281_d_n4;

        let assign17910_e32284: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard333 = assign17910_e32284;

        let (assign17920_e32331, assign17920_e32331_d_n0, assign17920_e32331_d_n2, assign17920_e32331_d_n3, assign17920_e32331_d_n4, assign17920_e32331_d_n5, assign17920_e32331_d_n6, assign17920_e32331_d_n7, assign17920_e32331_d_n8, assign17920_e32331_d_n9, assign17920_e32331_d_n10, assign17920_e32331_d_n11, assign17920_e32331_d_n13, assign17920_e32331_d_n14,) = {
    if (locals.var_guard333 != 0.0) {
        let assign17920_e32288: f64 = (-locals.var_ptwg_i);
        let assign17920_e32291: f64 = (-locals.var_ptwgt_i);
        let assign17920_e32293: f64 = (assign17920_e32291 * locals.var_deltemp);
        let assign17920_e32295: f64 = (-locals.var_ptwg_i);
        let assign17920_e32296: f64 = (assign17920_e32293 - assign17920_e32295);
        let assign17920_e32298: f64 = (assign17920_e32296 - 1e-6);
        let assign17920_e32300: f64 = (-locals.var_ptwgt_i);
        let assign17920_e32302: f64 = (assign17920_e32300 * locals.var_deltemp);
        let assign17920_e32304: f64 = (-locals.var_ptwg_i);
        let assign17920_e32305: f64 = (assign17920_e32302 - assign17920_e32304);
        let assign17920_e32307: f64 = (assign17920_e32305 - 1e-6);
        let assign17920_e32309: f64 = (-locals.var_ptwgt_i);
        let assign17920_e32311: f64 = (assign17920_e32309 * locals.var_deltemp);
        let assign17920_e32313: f64 = (-locals.var_ptwg_i);
        let assign17920_e32314: f64 = (assign17920_e32311 - assign17920_e32313);
        let assign17920_e32316: f64 = (assign17920_e32314 - 1e-6);
        let assign17920_e32317: f64 = (assign17920_e32307 * assign17920_e32316);
        let assign17920_e32320: f64 = (-locals.var_ptwg_i);
        let assign17920_e32321: f64 = (4.0 * assign17920_e32320);
        let assign17920_e32323: f64 = (assign17920_e32321 * 1e-6);
        let assign17920_e32324: f64 = (assign17920_e32317 - assign17920_e32323);
        let assign17920_e32325: f64 = (assign17920_e32324).sqrt();
        let assign17920_e32326: f64 = (assign17920_e32298 + assign17920_e32325);
        let assign17920_e32327: f64 = (0.5 * assign17920_e32326);
        let assign17920_e32328: f64 = (assign17920_e32288 + assign17920_e32327);
        let assign17920_e32329: f64 = (locals.var_ptwg_i + assign17920_e32328);
        (assign17920_e32329, (locals.var_ptwg_i_dn0 + ((-locals.var_ptwg_i_dn0) + (0.5 * ((-(-locals.var_ptwg_i_dn0)) + (((((-(-locals.var_ptwg_i_dn0)) * assign17920_e32316) + (assign17920_e32307 * (-(-locals.var_ptwg_i_dn0)))) - ((4.0 * (-locals.var_ptwg_i_dn0)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (locals.var_ptwg_i_dn2 + ((-locals.var_ptwg_i_dn2) + (0.5 * ((-(-locals.var_ptwg_i_dn2)) + (((((-(-locals.var_ptwg_i_dn2)) * assign17920_e32316) + (assign17920_e32307 * (-(-locals.var_ptwg_i_dn2)))) - ((4.0 * (-locals.var_ptwg_i_dn2)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (locals.var_ptwg_i_dn3 + ((-locals.var_ptwg_i_dn3) + (0.5 * ((-(-locals.var_ptwg_i_dn3)) + (((((-(-locals.var_ptwg_i_dn3)) * assign17920_e32316) + (assign17920_e32307 * (-(-locals.var_ptwg_i_dn3)))) - ((4.0 * (-locals.var_ptwg_i_dn3)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (locals.var_ptwg_i_dn4 + ((-locals.var_ptwg_i_dn4) + (0.5 * (((assign17920_e32291 * locals.var_deltemp_dn4) - (-locals.var_ptwg_i_dn4)) + ((((((assign17920_e32300 * locals.var_deltemp_dn4) - (-locals.var_ptwg_i_dn4)) * assign17920_e32316) + (assign17920_e32307 * ((assign17920_e32309 * locals.var_deltemp_dn4) - (-locals.var_ptwg_i_dn4)))) - ((4.0 * (-locals.var_ptwg_i_dn4)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (locals.var_ptwg_i_dn5 + ((-locals.var_ptwg_i_dn5) + (0.5 * ((-(-locals.var_ptwg_i_dn5)) + (((((-(-locals.var_ptwg_i_dn5)) * assign17920_e32316) + (assign17920_e32307 * (-(-locals.var_ptwg_i_dn5)))) - ((4.0 * (-locals.var_ptwg_i_dn5)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (locals.var_ptwg_i_dn6 + ((-locals.var_ptwg_i_dn6) + (0.5 * ((-(-locals.var_ptwg_i_dn6)) + (((((-(-locals.var_ptwg_i_dn6)) * assign17920_e32316) + (assign17920_e32307 * (-(-locals.var_ptwg_i_dn6)))) - ((4.0 * (-locals.var_ptwg_i_dn6)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (locals.var_ptwg_i_dn7 + ((-locals.var_ptwg_i_dn7) + (0.5 * ((-(-locals.var_ptwg_i_dn7)) + (((((-(-locals.var_ptwg_i_dn7)) * assign17920_e32316) + (assign17920_e32307 * (-(-locals.var_ptwg_i_dn7)))) - ((4.0 * (-locals.var_ptwg_i_dn7)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (locals.var_ptwg_i_dn8 + ((-locals.var_ptwg_i_dn8) + (0.5 * ((-(-locals.var_ptwg_i_dn8)) + (((((-(-locals.var_ptwg_i_dn8)) * assign17920_e32316) + (assign17920_e32307 * (-(-locals.var_ptwg_i_dn8)))) - ((4.0 * (-locals.var_ptwg_i_dn8)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (locals.var_ptwg_i_dn9 + ((-locals.var_ptwg_i_dn9) + (0.5 * ((-(-locals.var_ptwg_i_dn9)) + (((((-(-locals.var_ptwg_i_dn9)) * assign17920_e32316) + (assign17920_e32307 * (-(-locals.var_ptwg_i_dn9)))) - ((4.0 * (-locals.var_ptwg_i_dn9)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (locals.var_ptwg_i_dn10 + ((-locals.var_ptwg_i_dn10) + (0.5 * ((-(-locals.var_ptwg_i_dn10)) + (((((-(-locals.var_ptwg_i_dn10)) * assign17920_e32316) + (assign17920_e32307 * (-(-locals.var_ptwg_i_dn10)))) - ((4.0 * (-locals.var_ptwg_i_dn10)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (locals.var_ptwg_i_dn11 + ((-locals.var_ptwg_i_dn11) + (0.5 * ((-(-locals.var_ptwg_i_dn11)) + (((((-(-locals.var_ptwg_i_dn11)) * assign17920_e32316) + (assign17920_e32307 * (-(-locals.var_ptwg_i_dn11)))) - ((4.0 * (-locals.var_ptwg_i_dn11)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (locals.var_ptwg_i_dn13 + ((-locals.var_ptwg_i_dn13) + (0.5 * ((-(-locals.var_ptwg_i_dn13)) + (((((-(-locals.var_ptwg_i_dn13)) * assign17920_e32316) + (assign17920_e32307 * (-(-locals.var_ptwg_i_dn13)))) - ((4.0 * (-locals.var_ptwg_i_dn13)) * 1e-6)) / (2.0 * assign17920_e32325)))))), (locals.var_ptwg_i_dn14 + ((-locals.var_ptwg_i_dn14) + (0.5 * ((-(-locals.var_ptwg_i_dn14)) + (((((-(-locals.var_ptwg_i_dn14)) * assign17920_e32316) + (assign17920_e32307 * (-(-locals.var_ptwg_i_dn14)))) - ((4.0 * (-locals.var_ptwg_i_dn14)) * 1e-6)) / (2.0 * assign17920_e32325)))))),)
    } else {
        (locals.var_ptwg_t, locals.var_ptwg_t_dn0, locals.var_ptwg_t_dn2, locals.var_ptwg_t_dn3, locals.var_ptwg_t_dn4, locals.var_ptwg_t_dn5, locals.var_ptwg_t_dn6, locals.var_ptwg_t_dn7, locals.var_ptwg_t_dn8, locals.var_ptwg_t_dn9, locals.var_ptwg_t_dn10, locals.var_ptwg_t_dn11, locals.var_ptwg_t_dn13, locals.var_ptwg_t_dn14,)
    }
};
        locals.var_ptwg_t = assign17920_e32331;
        locals.var_ptwg_t_dn0 = assign17920_e32331_d_n0;
        locals.var_ptwg_t_dn2 = assign17920_e32331_d_n2;
        locals.var_ptwg_t_dn3 = assign17920_e32331_d_n3;
        locals.var_ptwg_t_dn4 = assign17920_e32331_d_n4;
        locals.var_ptwg_t_dn5 = assign17920_e32331_d_n5;
        locals.var_ptwg_t_dn6 = assign17920_e32331_d_n6;
        locals.var_ptwg_t_dn7 = assign17920_e32331_d_n7;
        locals.var_ptwg_t_dn8 = assign17920_e32331_d_n8;
        locals.var_ptwg_t_dn9 = assign17920_e32331_d_n9;
        locals.var_ptwg_t_dn10 = assign17920_e32331_d_n10;
        locals.var_ptwg_t_dn11 = assign17920_e32331_d_n11;
        locals.var_ptwg_t_dn13 = assign17920_e32331_d_n13;
        locals.var_ptwg_t_dn14 = assign17920_e32331_d_n14;

        let (assign17930_e32415, assign17930_e32415_d_n0, assign17930_e32415_d_n2, assign17930_e32415_d_n3, assign17930_e32415_d_n4, assign17930_e32415_d_n5, assign17930_e32415_d_n6, assign17930_e32415_d_n7, assign17930_e32415_d_n8, assign17930_e32415_d_n9, assign17930_e32415_d_n10, assign17930_e32415_d_n11, assign17930_e32415_d_n13, assign17930_e32415_d_n14,) = {
    if (locals.var_guard333 == 0.0) {
        let assign17930_e32337: f64 = (-locals.var_ptwgt_i);
        let assign17930_e32339: f64 = (assign17930_e32337 * locals.var_deltemp);
        let assign17930_e32340: f64 = (1.0 + assign17930_e32339);
        let assign17930_e32342: f64 = (assign17930_e32340 - 1e-6);
        let assign17930_e32344: f64 = (-10000.0);
        let assign17930_e32346: f64 = (assign17930_e32344 * 0.001);
        let (assign17930_e32412, assign17930_e32412_d_n4,) = {
            if (!(assign17930_e32342 < assign17930_e32346)) {
                let assign17930_e32352: f64 = (-locals.var_ptwgt_i);
                let assign17930_e32354: f64 = (assign17930_e32352 * locals.var_deltemp);
                let assign17930_e32355: f64 = (1.0 + assign17930_e32354);
                let assign17930_e32357: f64 = (assign17930_e32355 - 1e-6);
                let assign17930_e32360: f64 = (-locals.var_ptwgt_i);
                let assign17930_e32362: f64 = (assign17930_e32360 * locals.var_deltemp);
                let assign17930_e32363: f64 = (1.0 + assign17930_e32362);
                let assign17930_e32365: f64 = (assign17930_e32363 - 1e-6);
                let assign17930_e32368: f64 = (-locals.var_ptwgt_i);
                let assign17930_e32370: f64 = (assign17930_e32368 * locals.var_deltemp);
                let assign17930_e32371: f64 = (1.0 + assign17930_e32370);
                let assign17930_e32373: f64 = (assign17930_e32371 - 1e-6);
                let assign17930_e32374: f64 = (assign17930_e32365 * assign17930_e32373);
                let assign17930_e32377: f64 = (4.0 * 0.001);
                let assign17930_e32379: f64 = (assign17930_e32377 * 0.001);
                let assign17930_e32380: f64 = (assign17930_e32374 + assign17930_e32379);
                let assign17930_e32381: f64 = (assign17930_e32380).sqrt();
                let assign17930_e32382: f64 = (assign17930_e32357 + assign17930_e32381);
                let assign17930_e32383: f64 = (0.5 * assign17930_e32382);
                (assign17930_e32383, (0.5 * ((assign17930_e32352 * locals.var_deltemp_dn4) + ((((assign17930_e32360 * locals.var_deltemp_dn4) * assign17930_e32373) + (assign17930_e32365 * (assign17930_e32368 * locals.var_deltemp_dn4))) / (2.0 * assign17930_e32381)))),)
            } else {
                let assign17930_e32386: f64 = (-locals.var_ptwgt_i);
                let assign17930_e32388: f64 = (assign17930_e32386 * locals.var_deltemp);
                let assign17930_e32389: f64 = (1.0 + assign17930_e32388);
                let assign17930_e32391: f64 = (assign17930_e32389 - 1e-6);
                let assign17930_e32393: f64 = (-10000.0);
                let assign17930_e32395: f64 = (assign17930_e32393 * 0.001);
                let (assign17930_e32411, assign17930_e32411_d_n4,) = {
                    if (assign17930_e32391 < assign17930_e32395) {
                        let assign17930_e32398: f64 = (-0.001);
                        let assign17930_e32400: f64 = (assign17930_e32398 * 0.001);
                        let assign17930_e32403: f64 = (-locals.var_ptwgt_i);
                        let assign17930_e32405: f64 = (assign17930_e32403 * locals.var_deltemp);
                        let assign17930_e32406: f64 = (1.0 + assign17930_e32405);
                        let assign17930_e32408: f64 = (assign17930_e32406 - 1e-6);
                        let assign17930_e32409: f64 = (assign17930_e32400 / assign17930_e32408);
                        (assign17930_e32409, (-((assign17930_e32400 * (assign17930_e32403 * locals.var_deltemp_dn4)) / (assign17930_e32408 * assign17930_e32408))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17930_e32411, assign17930_e32411_d_n4,)
            }
        };
        let assign17930_e32413: f64 = (locals.var_ptwg_i * assign17930_e32412);
        (assign17930_e32413, (locals.var_ptwg_i_dn0 * assign17930_e32412), (locals.var_ptwg_i_dn2 * assign17930_e32412), (locals.var_ptwg_i_dn3 * assign17930_e32412), ((locals.var_ptwg_i_dn4 * assign17930_e32412) + (locals.var_ptwg_i * assign17930_e32412_d_n4)), (locals.var_ptwg_i_dn5 * assign17930_e32412), (locals.var_ptwg_i_dn6 * assign17930_e32412), (locals.var_ptwg_i_dn7 * assign17930_e32412), (locals.var_ptwg_i_dn8 * assign17930_e32412), (locals.var_ptwg_i_dn9 * assign17930_e32412), (locals.var_ptwg_i_dn10 * assign17930_e32412), (locals.var_ptwg_i_dn11 * assign17930_e32412), (locals.var_ptwg_i_dn13 * assign17930_e32412), (locals.var_ptwg_i_dn14 * assign17930_e32412),)
    } else {
        (locals.var_ptwg_t, locals.var_ptwg_t_dn0, locals.var_ptwg_t_dn2, locals.var_ptwg_t_dn3, locals.var_ptwg_t_dn4, locals.var_ptwg_t_dn5, locals.var_ptwg_t_dn6, locals.var_ptwg_t_dn7, locals.var_ptwg_t_dn8, locals.var_ptwg_t_dn9, locals.var_ptwg_t_dn10, locals.var_ptwg_t_dn11, locals.var_ptwg_t_dn13, locals.var_ptwg_t_dn14,)
    }
};
        locals.var_ptwg_t = assign17930_e32415;
        locals.var_ptwg_t_dn0 = assign17930_e32415_d_n0;
        locals.var_ptwg_t_dn2 = assign17930_e32415_d_n2;
        locals.var_ptwg_t_dn3 = assign17930_e32415_d_n3;
        locals.var_ptwg_t_dn4 = assign17930_e32415_d_n4;
        locals.var_ptwg_t_dn5 = assign17930_e32415_d_n5;
        locals.var_ptwg_t_dn6 = assign17930_e32415_d_n6;
        locals.var_ptwg_t_dn7 = assign17930_e32415_d_n7;
        locals.var_ptwg_t_dn8 = assign17930_e32415_d_n8;
        locals.var_ptwg_t_dn9 = assign17930_e32415_d_n9;
        locals.var_ptwg_t_dn10 = assign17930_e32415_d_n10;
        locals.var_ptwg_t_dn11 = assign17930_e32415_d_n11;
        locals.var_ptwg_t_dn13 = assign17930_e32415_d_n13;
        locals.var_ptwg_t_dn14 = assign17930_e32415_d_n14;

        let assign17940_e32418: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard334 = assign17940_e32418;

        let assign17950_e32421: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard335 = assign17950_e32421;

    }

    pub(super) fn stamp_transient_block_59(
        locals: &mut StampLocals,
    ) {
        let (assign17960_e32470, assign17960_e32470_d_n0, assign17960_e32470_d_n2, assign17960_e32470_d_n3, assign17960_e32470_d_n4, assign17960_e32470_d_n5, assign17960_e32470_d_n6, assign17960_e32470_d_n7, assign17960_e32470_d_n8, assign17960_e32470_d_n9, assign17960_e32470_d_n10, assign17960_e32470_d_n11, assign17960_e32470_d_n13, assign17960_e32470_d_n14,) = {
    if ((locals.var_guard334 != 0.0) && (locals.var_guard335 != 0.0)) {
        let assign17960_e32427: f64 = (-locals.var_ptwgr_i);
        let assign17960_e32430: f64 = (-locals.var_ptwgt_i);
        let assign17960_e32432: f64 = (assign17960_e32430 * locals.var_deltemp);
        let assign17960_e32434: f64 = (-locals.var_ptwgr_i);
        let assign17960_e32435: f64 = (assign17960_e32432 - assign17960_e32434);
        let assign17960_e32437: f64 = (assign17960_e32435 - 1e-6);
        let assign17960_e32439: f64 = (-locals.var_ptwgt_i);
        let assign17960_e32441: f64 = (assign17960_e32439 * locals.var_deltemp);
        let assign17960_e32443: f64 = (-locals.var_ptwgr_i);
        let assign17960_e32444: f64 = (assign17960_e32441 - assign17960_e32443);
        let assign17960_e32446: f64 = (assign17960_e32444 - 1e-6);
        let assign17960_e32448: f64 = (-locals.var_ptwgt_i);
        let assign17960_e32450: f64 = (assign17960_e32448 * locals.var_deltemp);
        let assign17960_e32452: f64 = (-locals.var_ptwgr_i);
        let assign17960_e32453: f64 = (assign17960_e32450 - assign17960_e32452);
        let assign17960_e32455: f64 = (assign17960_e32453 - 1e-6);
        let assign17960_e32456: f64 = (assign17960_e32446 * assign17960_e32455);
        let assign17960_e32459: f64 = (-locals.var_ptwgr_i);
        let assign17960_e32460: f64 = (4.0 * assign17960_e32459);
        let assign17960_e32462: f64 = (assign17960_e32460 * 1e-6);
        let assign17960_e32463: f64 = (assign17960_e32456 - assign17960_e32462);
        let assign17960_e32464: f64 = (assign17960_e32463).sqrt();
        let assign17960_e32465: f64 = (assign17960_e32437 + assign17960_e32464);
        let assign17960_e32466: f64 = (0.5 * assign17960_e32465);
        let assign17960_e32467: f64 = (assign17960_e32427 + assign17960_e32466);
        let assign17960_e32468: f64 = (locals.var_ptwgr_i + assign17960_e32467);
        (assign17960_e32468, (locals.var_ptwgr_i_dn0 + ((-locals.var_ptwgr_i_dn0) + (0.5 * ((-(-locals.var_ptwgr_i_dn0)) + (((((-(-locals.var_ptwgr_i_dn0)) * assign17960_e32455) + (assign17960_e32446 * (-(-locals.var_ptwgr_i_dn0)))) - ((4.0 * (-locals.var_ptwgr_i_dn0)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (locals.var_ptwgr_i_dn2 + ((-locals.var_ptwgr_i_dn2) + (0.5 * ((-(-locals.var_ptwgr_i_dn2)) + (((((-(-locals.var_ptwgr_i_dn2)) * assign17960_e32455) + (assign17960_e32446 * (-(-locals.var_ptwgr_i_dn2)))) - ((4.0 * (-locals.var_ptwgr_i_dn2)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (locals.var_ptwgr_i_dn3 + ((-locals.var_ptwgr_i_dn3) + (0.5 * ((-(-locals.var_ptwgr_i_dn3)) + (((((-(-locals.var_ptwgr_i_dn3)) * assign17960_e32455) + (assign17960_e32446 * (-(-locals.var_ptwgr_i_dn3)))) - ((4.0 * (-locals.var_ptwgr_i_dn3)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (locals.var_ptwgr_i_dn4 + ((-locals.var_ptwgr_i_dn4) + (0.5 * (((assign17960_e32430 * locals.var_deltemp_dn4) - (-locals.var_ptwgr_i_dn4)) + ((((((assign17960_e32439 * locals.var_deltemp_dn4) - (-locals.var_ptwgr_i_dn4)) * assign17960_e32455) + (assign17960_e32446 * ((assign17960_e32448 * locals.var_deltemp_dn4) - (-locals.var_ptwgr_i_dn4)))) - ((4.0 * (-locals.var_ptwgr_i_dn4)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (locals.var_ptwgr_i_dn5 + ((-locals.var_ptwgr_i_dn5) + (0.5 * ((-(-locals.var_ptwgr_i_dn5)) + (((((-(-locals.var_ptwgr_i_dn5)) * assign17960_e32455) + (assign17960_e32446 * (-(-locals.var_ptwgr_i_dn5)))) - ((4.0 * (-locals.var_ptwgr_i_dn5)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (locals.var_ptwgr_i_dn6 + ((-locals.var_ptwgr_i_dn6) + (0.5 * ((-(-locals.var_ptwgr_i_dn6)) + (((((-(-locals.var_ptwgr_i_dn6)) * assign17960_e32455) + (assign17960_e32446 * (-(-locals.var_ptwgr_i_dn6)))) - ((4.0 * (-locals.var_ptwgr_i_dn6)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (locals.var_ptwgr_i_dn7 + ((-locals.var_ptwgr_i_dn7) + (0.5 * ((-(-locals.var_ptwgr_i_dn7)) + (((((-(-locals.var_ptwgr_i_dn7)) * assign17960_e32455) + (assign17960_e32446 * (-(-locals.var_ptwgr_i_dn7)))) - ((4.0 * (-locals.var_ptwgr_i_dn7)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (locals.var_ptwgr_i_dn8 + ((-locals.var_ptwgr_i_dn8) + (0.5 * ((-(-locals.var_ptwgr_i_dn8)) + (((((-(-locals.var_ptwgr_i_dn8)) * assign17960_e32455) + (assign17960_e32446 * (-(-locals.var_ptwgr_i_dn8)))) - ((4.0 * (-locals.var_ptwgr_i_dn8)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (locals.var_ptwgr_i_dn9 + ((-locals.var_ptwgr_i_dn9) + (0.5 * ((-(-locals.var_ptwgr_i_dn9)) + (((((-(-locals.var_ptwgr_i_dn9)) * assign17960_e32455) + (assign17960_e32446 * (-(-locals.var_ptwgr_i_dn9)))) - ((4.0 * (-locals.var_ptwgr_i_dn9)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (locals.var_ptwgr_i_dn10 + ((-locals.var_ptwgr_i_dn10) + (0.5 * ((-(-locals.var_ptwgr_i_dn10)) + (((((-(-locals.var_ptwgr_i_dn10)) * assign17960_e32455) + (assign17960_e32446 * (-(-locals.var_ptwgr_i_dn10)))) - ((4.0 * (-locals.var_ptwgr_i_dn10)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (locals.var_ptwgr_i_dn11 + ((-locals.var_ptwgr_i_dn11) + (0.5 * ((-(-locals.var_ptwgr_i_dn11)) + (((((-(-locals.var_ptwgr_i_dn11)) * assign17960_e32455) + (assign17960_e32446 * (-(-locals.var_ptwgr_i_dn11)))) - ((4.0 * (-locals.var_ptwgr_i_dn11)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (locals.var_ptwgr_i_dn13 + ((-locals.var_ptwgr_i_dn13) + (0.5 * ((-(-locals.var_ptwgr_i_dn13)) + (((((-(-locals.var_ptwgr_i_dn13)) * assign17960_e32455) + (assign17960_e32446 * (-(-locals.var_ptwgr_i_dn13)))) - ((4.0 * (-locals.var_ptwgr_i_dn13)) * 1e-6)) / (2.0 * assign17960_e32464)))))), (locals.var_ptwgr_i_dn14 + ((-locals.var_ptwgr_i_dn14) + (0.5 * ((-(-locals.var_ptwgr_i_dn14)) + (((((-(-locals.var_ptwgr_i_dn14)) * assign17960_e32455) + (assign17960_e32446 * (-(-locals.var_ptwgr_i_dn14)))) - ((4.0 * (-locals.var_ptwgr_i_dn14)) * 1e-6)) / (2.0 * assign17960_e32464)))))),)
    } else {
        (locals.var_ptwgr_t, locals.var_ptwgr_t_dn0, locals.var_ptwgr_t_dn2, locals.var_ptwgr_t_dn3, locals.var_ptwgr_t_dn4, locals.var_ptwgr_t_dn5, locals.var_ptwgr_t_dn6, locals.var_ptwgr_t_dn7, locals.var_ptwgr_t_dn8, locals.var_ptwgr_t_dn9, locals.var_ptwgr_t_dn10, locals.var_ptwgr_t_dn11, locals.var_ptwgr_t_dn13, locals.var_ptwgr_t_dn14,)
    }
};
        locals.var_ptwgr_t = assign17960_e32470;
        locals.var_ptwgr_t_dn0 = assign17960_e32470_d_n0;
        locals.var_ptwgr_t_dn2 = assign17960_e32470_d_n2;
        locals.var_ptwgr_t_dn3 = assign17960_e32470_d_n3;
        locals.var_ptwgr_t_dn4 = assign17960_e32470_d_n4;
        locals.var_ptwgr_t_dn5 = assign17960_e32470_d_n5;
        locals.var_ptwgr_t_dn6 = assign17960_e32470_d_n6;
        locals.var_ptwgr_t_dn7 = assign17960_e32470_d_n7;
        locals.var_ptwgr_t_dn8 = assign17960_e32470_d_n8;
        locals.var_ptwgr_t_dn9 = assign17960_e32470_d_n9;
        locals.var_ptwgr_t_dn10 = assign17960_e32470_d_n10;
        locals.var_ptwgr_t_dn11 = assign17960_e32470_d_n11;
        locals.var_ptwgr_t_dn13 = assign17960_e32470_d_n13;
        locals.var_ptwgr_t_dn14 = assign17960_e32470_d_n14;

        let (assign17970_e32556, assign17970_e32556_d_n0, assign17970_e32556_d_n2, assign17970_e32556_d_n3, assign17970_e32556_d_n4, assign17970_e32556_d_n5, assign17970_e32556_d_n6, assign17970_e32556_d_n7, assign17970_e32556_d_n8, assign17970_e32556_d_n9, assign17970_e32556_d_n10, assign17970_e32556_d_n11, assign17970_e32556_d_n13, assign17970_e32556_d_n14,) = {
    if ((locals.var_guard334 != 0.0) && (locals.var_guard335 == 0.0)) {
        let assign17970_e32478: f64 = (-locals.var_ptwgt_i);
        let assign17970_e32480: f64 = (assign17970_e32478 * locals.var_deltemp);
        let assign17970_e32481: f64 = (1.0 + assign17970_e32480);
        let assign17970_e32483: f64 = (assign17970_e32481 - 1e-6);
        let assign17970_e32485: f64 = (-10000.0);
        let assign17970_e32487: f64 = (assign17970_e32485 * 0.001);
        let (assign17970_e32553, assign17970_e32553_d_n4,) = {
            if (!(assign17970_e32483 < assign17970_e32487)) {
                let assign17970_e32493: f64 = (-locals.var_ptwgt_i);
                let assign17970_e32495: f64 = (assign17970_e32493 * locals.var_deltemp);
                let assign17970_e32496: f64 = (1.0 + assign17970_e32495);
                let assign17970_e32498: f64 = (assign17970_e32496 - 1e-6);
                let assign17970_e32501: f64 = (-locals.var_ptwgt_i);
                let assign17970_e32503: f64 = (assign17970_e32501 * locals.var_deltemp);
                let assign17970_e32504: f64 = (1.0 + assign17970_e32503);
                let assign17970_e32506: f64 = (assign17970_e32504 - 1e-6);
                let assign17970_e32509: f64 = (-locals.var_ptwgt_i);
                let assign17970_e32511: f64 = (assign17970_e32509 * locals.var_deltemp);
                let assign17970_e32512: f64 = (1.0 + assign17970_e32511);
                let assign17970_e32514: f64 = (assign17970_e32512 - 1e-6);
                let assign17970_e32515: f64 = (assign17970_e32506 * assign17970_e32514);
                let assign17970_e32518: f64 = (4.0 * 0.001);
                let assign17970_e32520: f64 = (assign17970_e32518 * 0.001);
                let assign17970_e32521: f64 = (assign17970_e32515 + assign17970_e32520);
                let assign17970_e32522: f64 = (assign17970_e32521).sqrt();
                let assign17970_e32523: f64 = (assign17970_e32498 + assign17970_e32522);
                let assign17970_e32524: f64 = (0.5 * assign17970_e32523);
                (assign17970_e32524, (0.5 * ((assign17970_e32493 * locals.var_deltemp_dn4) + ((((assign17970_e32501 * locals.var_deltemp_dn4) * assign17970_e32514) + (assign17970_e32506 * (assign17970_e32509 * locals.var_deltemp_dn4))) / (2.0 * assign17970_e32522)))),)
            } else {
                let assign17970_e32527: f64 = (-locals.var_ptwgt_i);
                let assign17970_e32529: f64 = (assign17970_e32527 * locals.var_deltemp);
                let assign17970_e32530: f64 = (1.0 + assign17970_e32529);
                let assign17970_e32532: f64 = (assign17970_e32530 - 1e-6);
                let assign17970_e32534: f64 = (-10000.0);
                let assign17970_e32536: f64 = (assign17970_e32534 * 0.001);
                let (assign17970_e32552, assign17970_e32552_d_n4,) = {
                    if (assign17970_e32532 < assign17970_e32536) {
                        let assign17970_e32539: f64 = (-0.001);
                        let assign17970_e32541: f64 = (assign17970_e32539 * 0.001);
                        let assign17970_e32544: f64 = (-locals.var_ptwgt_i);
                        let assign17970_e32546: f64 = (assign17970_e32544 * locals.var_deltemp);
                        let assign17970_e32547: f64 = (1.0 + assign17970_e32546);
                        let assign17970_e32549: f64 = (assign17970_e32547 - 1e-6);
                        let assign17970_e32550: f64 = (assign17970_e32541 / assign17970_e32549);
                        (assign17970_e32550, (-((assign17970_e32541 * (assign17970_e32544 * locals.var_deltemp_dn4)) / (assign17970_e32549 * assign17970_e32549))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign17970_e32552, assign17970_e32552_d_n4,)
            }
        };
        let assign17970_e32554: f64 = (locals.var_ptwgr_i * assign17970_e32553);
        (assign17970_e32554, (locals.var_ptwgr_i_dn0 * assign17970_e32553), (locals.var_ptwgr_i_dn2 * assign17970_e32553), (locals.var_ptwgr_i_dn3 * assign17970_e32553), ((locals.var_ptwgr_i_dn4 * assign17970_e32553) + (locals.var_ptwgr_i * assign17970_e32553_d_n4)), (locals.var_ptwgr_i_dn5 * assign17970_e32553), (locals.var_ptwgr_i_dn6 * assign17970_e32553), (locals.var_ptwgr_i_dn7 * assign17970_e32553), (locals.var_ptwgr_i_dn8 * assign17970_e32553), (locals.var_ptwgr_i_dn9 * assign17970_e32553), (locals.var_ptwgr_i_dn10 * assign17970_e32553), (locals.var_ptwgr_i_dn11 * assign17970_e32553), (locals.var_ptwgr_i_dn13 * assign17970_e32553), (locals.var_ptwgr_i_dn14 * assign17970_e32553),)
    } else {
        (locals.var_ptwgr_t, locals.var_ptwgr_t_dn0, locals.var_ptwgr_t_dn2, locals.var_ptwgr_t_dn3, locals.var_ptwgr_t_dn4, locals.var_ptwgr_t_dn5, locals.var_ptwgr_t_dn6, locals.var_ptwgr_t_dn7, locals.var_ptwgr_t_dn8, locals.var_ptwgr_t_dn9, locals.var_ptwgr_t_dn10, locals.var_ptwgr_t_dn11, locals.var_ptwgr_t_dn13, locals.var_ptwgr_t_dn14,)
    }
};
        locals.var_ptwgr_t = assign17970_e32556;
        locals.var_ptwgr_t_dn0 = assign17970_e32556_d_n0;
        locals.var_ptwgr_t_dn2 = assign17970_e32556_d_n2;
        locals.var_ptwgr_t_dn3 = assign17970_e32556_d_n3;
        locals.var_ptwgr_t_dn4 = assign17970_e32556_d_n4;
        locals.var_ptwgr_t_dn5 = assign17970_e32556_d_n5;
        locals.var_ptwgr_t_dn6 = assign17970_e32556_d_n6;
        locals.var_ptwgr_t_dn7 = assign17970_e32556_d_n7;
        locals.var_ptwgr_t_dn8 = assign17970_e32556_d_n8;
        locals.var_ptwgr_t_dn9 = assign17970_e32556_d_n9;
        locals.var_ptwgr_t_dn10 = assign17970_e32556_d_n10;
        locals.var_ptwgr_t_dn11 = assign17970_e32556_d_n11;
        locals.var_ptwgr_t_dn13 = assign17970_e32556_d_n13;
        locals.var_ptwgr_t_dn14 = assign17970_e32556_d_n14;

        let assign17980_e32560: f64 = (locals.var_iit_i * locals.var_trat_ln);
        let assign17980_e32561: f64 = (assign17980_e32560).exp();
        let assign17980_e32562: f64 = (locals.var_beta0_i * assign17980_e32561);
        locals.var_beta0_t = assign17980_e32562;
        locals.var_beta0_t_dn4 = (locals.var_beta0_i * (assign17980_e32561 * (locals.var_iit_i * locals.var_trat_ln_dn4)));

        let assign17990_e32567: f64 = (locals.var_tii_i * locals.var_tratio_m1);
        let assign17990_e32568: f64 = (1.0 + assign17990_e32567);
        let assign17990_e32570: f64 = (assign17990_e32568 - 0.01);
        let assign17990_e32572: f64 = (-10000.0);
        let assign17990_e32574: f64 = (assign17990_e32572 * 0.001);
        let (assign17990_e32635, assign17990_e32635_d_n4,) = {
    if (!(assign17990_e32570 < assign17990_e32574)) {
        let assign17990_e32581: f64 = (locals.var_tii_i * locals.var_tratio_m1);
        let assign17990_e32582: f64 = (1.0 + assign17990_e32581);
        let assign17990_e32584: f64 = (assign17990_e32582 - 0.01);
        let assign17990_e32588: f64 = (locals.var_tii_i * locals.var_tratio_m1);
        let assign17990_e32589: f64 = (1.0 + assign17990_e32588);
        let assign17990_e32591: f64 = (assign17990_e32589 - 0.01);
        let assign17990_e32595: f64 = (locals.var_tii_i * locals.var_tratio_m1);
        let assign17990_e32596: f64 = (1.0 + assign17990_e32595);
        let assign17990_e32598: f64 = (assign17990_e32596 - 0.01);
        let assign17990_e32599: f64 = (assign17990_e32591 * assign17990_e32598);
        let assign17990_e32602: f64 = (4.0 * 0.001);
        let assign17990_e32604: f64 = (assign17990_e32602 * 0.001);
        let assign17990_e32605: f64 = (assign17990_e32599 + assign17990_e32604);
        let assign17990_e32606: f64 = (assign17990_e32605).sqrt();
        let assign17990_e32607: f64 = (assign17990_e32584 + assign17990_e32606);
        let assign17990_e32608: f64 = (0.5 * assign17990_e32607);
        (assign17990_e32608, (0.5 * ((locals.var_tii_i * locals.var_tratio_m1_dn4) + ((((locals.var_tii_i * locals.var_tratio_m1_dn4) * assign17990_e32598) + (assign17990_e32591 * (locals.var_tii_i * locals.var_tratio_m1_dn4))) / (2.0 * assign17990_e32606)))),)
    } else {
        let assign17990_e32612: f64 = (locals.var_tii_i * locals.var_tratio_m1);
        let assign17990_e32613: f64 = (1.0 + assign17990_e32612);
        let assign17990_e32615: f64 = (assign17990_e32613 - 0.01);
        let assign17990_e32617: f64 = (-10000.0);
        let assign17990_e32619: f64 = (assign17990_e32617 * 0.001);
        let (assign17990_e32634, assign17990_e32634_d_n4,) = {
            if (assign17990_e32615 < assign17990_e32619) {
                let assign17990_e32622: f64 = (-0.001);
                let assign17990_e32624: f64 = (assign17990_e32622 * 0.001);
                let assign17990_e32628: f64 = (locals.var_tii_i * locals.var_tratio_m1);
                let assign17990_e32629: f64 = (1.0 + assign17990_e32628);
                let assign17990_e32631: f64 = (assign17990_e32629 - 0.01);
                let assign17990_e32632: f64 = (assign17990_e32624 / assign17990_e32631);
                (assign17990_e32632, (-((assign17990_e32624 * (locals.var_tii_i * locals.var_tratio_m1_dn4)) / (assign17990_e32631 * assign17990_e32631))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign17990_e32634, assign17990_e32634_d_n4,)
    }
};
        let assign17990_e32637: f64 = (assign17990_e32635 + 0.01);
        let assign17990_e32638: f64 = (locals.var_sii0_i * assign17990_e32637);
        locals.var_sii0_t = assign17990_e32638;
        locals.var_sii0_t_dn4 = (locals.var_sii0_i * assign17990_e32635_d_n4);

        let assign18000_e32642: f64 = (locals.var_k01_i * locals.var_deltemp);
        let assign18000_e32643: f64 = (locals.var_k0_i + assign18000_e32642);
        locals.var_k0_t = assign18000_e32643;
        locals.var_k0_t_dn4 = (locals.var_k01_i * locals.var_deltemp_dn4);

        let assign18010_e32646: f64 = (-locals.var_k0si_i);
        let assign18010_e32650: f64 = (locals.var_k0si1_i * locals.var_deltemp);
        let assign18010_e32652: f64 = (-locals.var_k0si_i);
        let assign18010_e32653: f64 = (assign18010_e32650 - assign18010_e32652);
        let assign18010_e32655: f64 = (assign18010_e32653 - 1e-6);
        let assign18010_e32658: f64 = (locals.var_k0si1_i * locals.var_deltemp);
        let assign18010_e32660: f64 = (-locals.var_k0si_i);
        let assign18010_e32661: f64 = (assign18010_e32658 - assign18010_e32660);
        let assign18010_e32663: f64 = (assign18010_e32661 - 1e-6);
        let assign18010_e32666: f64 = (locals.var_k0si1_i * locals.var_deltemp);
        let assign18010_e32668: f64 = (-locals.var_k0si_i);
        let assign18010_e32669: f64 = (assign18010_e32666 - assign18010_e32668);
        let assign18010_e32671: f64 = (assign18010_e32669 - 1e-6);
        let assign18010_e32672: f64 = (assign18010_e32663 * assign18010_e32671);
        let assign18010_e32675: f64 = (-locals.var_k0si_i);
        let assign18010_e32676: f64 = (4.0 * assign18010_e32675);
        let assign18010_e32678: f64 = (assign18010_e32676 * 1e-6);
        let assign18010_e32679: f64 = (assign18010_e32672 - assign18010_e32678);
        let assign18010_e32680: f64 = (assign18010_e32679).sqrt();
        let assign18010_e32681: f64 = (assign18010_e32655 + assign18010_e32680);
        let assign18010_e32682: f64 = (0.5 * assign18010_e32681);
        let assign18010_e32683: f64 = (assign18010_e32646 + assign18010_e32682);
        let assign18010_e32684: f64 = (locals.var_k0si_i + assign18010_e32683);
        locals.var_k0si_t = assign18010_e32684;
        locals.var_k0si_t_dn4 = (0.5 * ((locals.var_k0si1_i * locals.var_deltemp_dn4) + ((((locals.var_k0si1_i * locals.var_deltemp_dn4) * assign18010_e32671) + (assign18010_e32663 * (locals.var_k0si1_i * locals.var_deltemp_dn4))) / (2.0 * assign18010_e32680))));

        let assign18020_e32687: f64 = (-locals.var_k2si_i);
        let assign18020_e32691: f64 = (locals.var_k2si1_i * locals.var_deltemp);
        let assign18020_e32693: f64 = (-locals.var_k2si_i);
        let assign18020_e32694: f64 = (assign18020_e32691 - assign18020_e32693);
        let assign18020_e32696: f64 = (assign18020_e32694 - 1e-6);
        let assign18020_e32699: f64 = (locals.var_k2si1_i * locals.var_deltemp);
        let assign18020_e32701: f64 = (-locals.var_k2si_i);
        let assign18020_e32702: f64 = (assign18020_e32699 - assign18020_e32701);
        let assign18020_e32704: f64 = (assign18020_e32702 - 1e-6);
        let assign18020_e32707: f64 = (locals.var_k2si1_i * locals.var_deltemp);
        let assign18020_e32709: f64 = (-locals.var_k2si_i);
        let assign18020_e32710: f64 = (assign18020_e32707 - assign18020_e32709);
        let assign18020_e32712: f64 = (assign18020_e32710 - 1e-6);
        let assign18020_e32713: f64 = (assign18020_e32704 * assign18020_e32712);
        let assign18020_e32716: f64 = (-locals.var_k2si_i);
        let assign18020_e32717: f64 = (4.0 * assign18020_e32716);
        let assign18020_e32719: f64 = (assign18020_e32717 * 1e-6);
        let assign18020_e32720: f64 = (assign18020_e32713 - assign18020_e32719);
        let assign18020_e32721: f64 = (assign18020_e32720).sqrt();
        let assign18020_e32722: f64 = (assign18020_e32696 + assign18020_e32721);
        let assign18020_e32723: f64 = (0.5 * assign18020_e32722);
        let assign18020_e32724: f64 = (assign18020_e32687 + assign18020_e32723);
        let assign18020_e32725: f64 = (locals.var_k2si_i + assign18020_e32724);
        locals.var_k2si_t = assign18020_e32725;
        locals.var_k2si_t_dn4 = (0.5 * ((locals.var_k2si1_i * locals.var_deltemp_dn4) + ((((locals.var_k2si1_i * locals.var_deltemp_dn4) * assign18020_e32712) + (assign18020_e32704 * (locals.var_k2si1_i * locals.var_deltemp_dn4))) / (2.0 * assign18020_e32721))));

        let assign18030_e32728: f64 = (-locals.var_k1_i);
        let assign18030_e32732: f64 = (locals.var_k11_i * locals.var_deltemp);
        let assign18030_e32734: f64 = (-locals.var_k1_i);
        let assign18030_e32735: f64 = (assign18030_e32732 - assign18030_e32734);
        let assign18030_e32737: f64 = (assign18030_e32735 - 1e-6);
        let assign18030_e32740: f64 = (locals.var_k11_i * locals.var_deltemp);
        let assign18030_e32742: f64 = (-locals.var_k1_i);
        let assign18030_e32743: f64 = (assign18030_e32740 - assign18030_e32742);
        let assign18030_e32745: f64 = (assign18030_e32743 - 1e-6);
        let assign18030_e32748: f64 = (locals.var_k11_i * locals.var_deltemp);
        let assign18030_e32750: f64 = (-locals.var_k1_i);
        let assign18030_e32751: f64 = (assign18030_e32748 - assign18030_e32750);
        let assign18030_e32753: f64 = (assign18030_e32751 - 1e-6);
        let assign18030_e32754: f64 = (assign18030_e32745 * assign18030_e32753);
        let assign18030_e32757: f64 = (-locals.var_k1_i);
        let assign18030_e32758: f64 = (4.0 * assign18030_e32757);
        let assign18030_e32760: f64 = (assign18030_e32758 * 1e-6);
        let assign18030_e32761: f64 = (assign18030_e32754 - assign18030_e32760);
        let assign18030_e32762: f64 = (assign18030_e32761).sqrt();
        let assign18030_e32763: f64 = (assign18030_e32737 + assign18030_e32762);
        let assign18030_e32764: f64 = (0.5 * assign18030_e32763);
        let assign18030_e32765: f64 = (assign18030_e32728 + assign18030_e32764);
        let assign18030_e32766: f64 = (locals.var_k1_i + assign18030_e32765);
        locals.var_k1_t = assign18030_e32766;
        locals.var_k1_t_dn4 = (0.5 * ((locals.var_k11_i * locals.var_deltemp_dn4) + ((((locals.var_k11_i * locals.var_deltemp_dn4) * assign18030_e32753) + (assign18030_e32745 * (locals.var_k11_i * locals.var_deltemp_dn4))) / (2.0 * assign18030_e32762))));

        let assign18040_e32770: f64 = (locals.var_k2sat1_i * locals.var_deltemp);
        let assign18040_e32771: f64 = (locals.var_k2sat_i + assign18040_e32770);
        locals.var_k2sat_t = assign18040_e32771;
        locals.var_k2sat_t_dn4 = (locals.var_k2sat1_i * locals.var_deltemp_dn4);

        let assign18050_e32775: f64 = (locals.var_a11_i * locals.var_deltemp);
        let assign18050_e32776: f64 = (locals.var_a1_i + assign18050_e32775);
        locals.var_a1_t = assign18050_e32776;
        locals.var_a1_t_dn4 = (locals.var_a11_i * locals.var_deltemp_dn4);

        let assign18060_e32780: f64 = (locals.var_a21_i * locals.var_deltemp);
        let assign18060_e32781: f64 = (locals.var_a2_i + assign18060_e32780);
        locals.var_a2_t = assign18060_e32781;
        locals.var_a2_t_dn4 = (locals.var_a21_i * locals.var_deltemp_dn4);

        let assign18070_e32784: f64 = (-locals.var_k2_i);
        let assign18070_e32788: f64 = (locals.var_k21_i * locals.var_deltemp);
        let assign18070_e32790: f64 = (-locals.var_k2_i);
        let assign18070_e32791: f64 = (assign18070_e32788 - assign18070_e32790);
        let assign18070_e32793: f64 = (assign18070_e32791 - 1e-6);
        let assign18070_e32796: f64 = (locals.var_k21_i * locals.var_deltemp);
        let assign18070_e32798: f64 = (-locals.var_k2_i);
        let assign18070_e32799: f64 = (assign18070_e32796 - assign18070_e32798);
        let assign18070_e32801: f64 = (assign18070_e32799 - 1e-6);
        let assign18070_e32804: f64 = (locals.var_k21_i * locals.var_deltemp);
        let assign18070_e32806: f64 = (-locals.var_k2_i);
        let assign18070_e32807: f64 = (assign18070_e32804 - assign18070_e32806);
        let assign18070_e32809: f64 = (assign18070_e32807 - 1e-6);
        let assign18070_e32810: f64 = (assign18070_e32801 * assign18070_e32809);
        let assign18070_e32813: f64 = (-locals.var_k2_i);
        let assign18070_e32814: f64 = (4.0 * assign18070_e32813);
        let assign18070_e32816: f64 = (assign18070_e32814 * 1e-6);
        let assign18070_e32817: f64 = (assign18070_e32810 - assign18070_e32816);
        let assign18070_e32818: f64 = (assign18070_e32817).sqrt();
        let assign18070_e32819: f64 = (assign18070_e32793 + assign18070_e32818);
        let assign18070_e32820: f64 = (0.5 * assign18070_e32819);
        let assign18070_e32821: f64 = (assign18070_e32784 + assign18070_e32820);
        let assign18070_e32822: f64 = (locals.var_k2_i + assign18070_e32821);
        locals.var_k2_t = assign18070_e32822;
        locals.var_k2_t_dn4 = (0.5 * ((locals.var_k21_i * locals.var_deltemp_dn4) + ((((locals.var_k21_i * locals.var_deltemp_dn4) * assign18070_e32809) + (assign18070_e32801 * (locals.var_k21_i * locals.var_deltemp_dn4))) / (2.0 * assign18070_e32818))));

        let assign18080_e32826: f64 = (locals.var_k0sisat1_i * locals.var_deltemp);
        let assign18080_e32827: f64 = (locals.var_k0sisat_i + assign18080_e32826);
        locals.var_k0sisat_t = assign18080_e32827;
        locals.var_k0sisat_t_dn4 = (locals.var_k0sisat1_i * locals.var_deltemp_dn4);

        let assign18090_e32831: f64 = (locals.var_k2sisat1_i * locals.var_deltemp);
        let assign18090_e32832: f64 = (locals.var_k2sisat_i + assign18090_e32831);
        locals.var_k2sisat_t = assign18090_e32832;
        locals.var_k2sisat_t_dn4 = (locals.var_k2sisat1_i * locals.var_deltemp_dn4);

        let assign18100_e32835: f64 = (-locals.var_aigbinv_i);
        let assign18100_e32839: f64 = (locals.var_aigbinv1_i * locals.var_deltemp);
        let assign18100_e32841: f64 = (-locals.var_aigbinv_i);
        let assign18100_e32842: f64 = (assign18100_e32839 - assign18100_e32841);
        let assign18100_e32844: f64 = (assign18100_e32842 - 1e-6);
        let assign18100_e32847: f64 = (locals.var_aigbinv1_i * locals.var_deltemp);
        let assign18100_e32849: f64 = (-locals.var_aigbinv_i);
        let assign18100_e32850: f64 = (assign18100_e32847 - assign18100_e32849);
        let assign18100_e32852: f64 = (assign18100_e32850 - 1e-6);
        let assign18100_e32855: f64 = (locals.var_aigbinv1_i * locals.var_deltemp);
        let assign18100_e32857: f64 = (-locals.var_aigbinv_i);
        let assign18100_e32858: f64 = (assign18100_e32855 - assign18100_e32857);
        let assign18100_e32860: f64 = (assign18100_e32858 - 1e-6);
        let assign18100_e32861: f64 = (assign18100_e32852 * assign18100_e32860);
        let assign18100_e32864: f64 = (-locals.var_aigbinv_i);
        let assign18100_e32865: f64 = (4.0 * assign18100_e32864);
        let assign18100_e32867: f64 = (assign18100_e32865 * 1e-6);
        let assign18100_e32868: f64 = (assign18100_e32861 - assign18100_e32867);
        let assign18100_e32869: f64 = (assign18100_e32868).sqrt();
        let assign18100_e32870: f64 = (assign18100_e32844 + assign18100_e32869);
        let assign18100_e32871: f64 = (0.5 * assign18100_e32870);
        let assign18100_e32872: f64 = (assign18100_e32835 + assign18100_e32871);
        let assign18100_e32873: f64 = (locals.var_aigbinv_i + assign18100_e32872);
        locals.var_aigbinv_t = assign18100_e32873;
        locals.var_aigbinv_t_dn4 = (0.5 * ((locals.var_aigbinv1_i * locals.var_deltemp_dn4) + ((((locals.var_aigbinv1_i * locals.var_deltemp_dn4) * assign18100_e32860) + (assign18100_e32852 * (locals.var_aigbinv1_i * locals.var_deltemp_dn4))) / (2.0 * assign18100_e32869))));

        let assign18110_e32876: f64 = (-locals.var_aigbacc_i);
        let assign18110_e32880: f64 = (locals.var_aigbacc1_i * locals.var_deltemp);
        let assign18110_e32882: f64 = (-locals.var_aigbacc_i);
        let assign18110_e32883: f64 = (assign18110_e32880 - assign18110_e32882);
        let assign18110_e32885: f64 = (assign18110_e32883 - 1e-6);
        let assign18110_e32888: f64 = (locals.var_aigbacc1_i * locals.var_deltemp);
        let assign18110_e32890: f64 = (-locals.var_aigbacc_i);
        let assign18110_e32891: f64 = (assign18110_e32888 - assign18110_e32890);
        let assign18110_e32893: f64 = (assign18110_e32891 - 1e-6);
        let assign18110_e32896: f64 = (locals.var_aigbacc1_i * locals.var_deltemp);
        let assign18110_e32898: f64 = (-locals.var_aigbacc_i);
        let assign18110_e32899: f64 = (assign18110_e32896 - assign18110_e32898);
        let assign18110_e32901: f64 = (assign18110_e32899 - 1e-6);
        let assign18110_e32902: f64 = (assign18110_e32893 * assign18110_e32901);
        let assign18110_e32905: f64 = (-locals.var_aigbacc_i);
        let assign18110_e32906: f64 = (4.0 * assign18110_e32905);
        let assign18110_e32908: f64 = (assign18110_e32906 * 1e-6);
        let assign18110_e32909: f64 = (assign18110_e32902 - assign18110_e32908);
        let assign18110_e32910: f64 = (assign18110_e32909).sqrt();
        let assign18110_e32911: f64 = (assign18110_e32885 + assign18110_e32910);
        let assign18110_e32912: f64 = (0.5 * assign18110_e32911);
        let assign18110_e32913: f64 = (assign18110_e32876 + assign18110_e32912);
        let assign18110_e32914: f64 = (locals.var_aigbacc_i + assign18110_e32913);
        locals.var_aigbacc_t = assign18110_e32914;
        locals.var_aigbacc_t_dn4 = (0.5 * ((locals.var_aigbacc1_i * locals.var_deltemp_dn4) + ((((locals.var_aigbacc1_i * locals.var_deltemp_dn4) * assign18110_e32901) + (assign18110_e32893 * (locals.var_aigbacc1_i * locals.var_deltemp_dn4))) / (2.0 * assign18110_e32910))));

        let assign18120_e32917: f64 = (-locals.var_aigc_i);
        let assign18120_e32921: f64 = (locals.var_aigc1_i * locals.var_deltemp);
        let assign18120_e32923: f64 = (-locals.var_aigc_i);
        let assign18120_e32924: f64 = (assign18120_e32921 - assign18120_e32923);
        let assign18120_e32926: f64 = (assign18120_e32924 - 1e-6);
        let assign18120_e32929: f64 = (locals.var_aigc1_i * locals.var_deltemp);
        let assign18120_e32931: f64 = (-locals.var_aigc_i);
        let assign18120_e32932: f64 = (assign18120_e32929 - assign18120_e32931);
        let assign18120_e32934: f64 = (assign18120_e32932 - 1e-6);
        let assign18120_e32937: f64 = (locals.var_aigc1_i * locals.var_deltemp);
        let assign18120_e32939: f64 = (-locals.var_aigc_i);
        let assign18120_e32940: f64 = (assign18120_e32937 - assign18120_e32939);
        let assign18120_e32942: f64 = (assign18120_e32940 - 1e-6);
        let assign18120_e32943: f64 = (assign18120_e32934 * assign18120_e32942);
        let assign18120_e32946: f64 = (-locals.var_aigc_i);
        let assign18120_e32947: f64 = (4.0 * assign18120_e32946);
        let assign18120_e32949: f64 = (assign18120_e32947 * 1e-6);
        let assign18120_e32950: f64 = (assign18120_e32943 - assign18120_e32949);
        let assign18120_e32951: f64 = (assign18120_e32950).sqrt();
        let assign18120_e32952: f64 = (assign18120_e32926 + assign18120_e32951);
        let assign18120_e32953: f64 = (0.5 * assign18120_e32952);
        let assign18120_e32954: f64 = (assign18120_e32917 + assign18120_e32953);
        let assign18120_e32955: f64 = (locals.var_aigc_i + assign18120_e32954);
        locals.var_aigc_t = assign18120_e32955;
        locals.var_aigc_t_dn4 = (0.5 * ((locals.var_aigc1_i * locals.var_deltemp_dn4) + ((((locals.var_aigc1_i * locals.var_deltemp_dn4) * assign18120_e32942) + (assign18120_e32934 * (locals.var_aigc1_i * locals.var_deltemp_dn4))) / (2.0 * assign18120_e32951))));

        let assign18130_e32958: f64 = (-locals.var_aigs_i);
        let assign18130_e32962: f64 = (locals.var_aigs1_i * locals.var_deltemp);
        let assign18130_e32964: f64 = (-locals.var_aigs_i);
        let assign18130_e32965: f64 = (assign18130_e32962 - assign18130_e32964);
        let assign18130_e32967: f64 = (assign18130_e32965 - 1e-6);
        let assign18130_e32970: f64 = (locals.var_aigs1_i * locals.var_deltemp);
        let assign18130_e32972: f64 = (-locals.var_aigs_i);
        let assign18130_e32973: f64 = (assign18130_e32970 - assign18130_e32972);
        let assign18130_e32975: f64 = (assign18130_e32973 - 1e-6);
        let assign18130_e32978: f64 = (locals.var_aigs1_i * locals.var_deltemp);
        let assign18130_e32980: f64 = (-locals.var_aigs_i);
        let assign18130_e32981: f64 = (assign18130_e32978 - assign18130_e32980);
        let assign18130_e32983: f64 = (assign18130_e32981 - 1e-6);
        let assign18130_e32984: f64 = (assign18130_e32975 * assign18130_e32983);
        let assign18130_e32987: f64 = (-locals.var_aigs_i);
        let assign18130_e32988: f64 = (4.0 * assign18130_e32987);
        let assign18130_e32990: f64 = (assign18130_e32988 * 1e-6);
        let assign18130_e32991: f64 = (assign18130_e32984 - assign18130_e32990);
        let assign18130_e32992: f64 = (assign18130_e32991).sqrt();
        let assign18130_e32993: f64 = (assign18130_e32967 + assign18130_e32992);
        let assign18130_e32994: f64 = (0.5 * assign18130_e32993);
        let assign18130_e32995: f64 = (assign18130_e32958 + assign18130_e32994);
        let assign18130_e32996: f64 = (locals.var_aigs_i + assign18130_e32995);
        locals.var_aigs_t = assign18130_e32996;
        locals.var_aigs_t_dn4 = (0.5 * ((locals.var_aigs1_i * locals.var_deltemp_dn4) + ((((locals.var_aigs1_i * locals.var_deltemp_dn4) * assign18130_e32983) + (assign18130_e32975 * (locals.var_aigs1_i * locals.var_deltemp_dn4))) / (2.0 * assign18130_e32992))));

        let assign18140_e32999: f64 = (-locals.var_aigd_i);
        let assign18140_e33003: f64 = (locals.var_aigd1_i * locals.var_deltemp);
        let assign18140_e33005: f64 = (-locals.var_aigd_i);
        let assign18140_e33006: f64 = (assign18140_e33003 - assign18140_e33005);
        let assign18140_e33008: f64 = (assign18140_e33006 - 1e-6);
        let assign18140_e33011: f64 = (locals.var_aigd1_i * locals.var_deltemp);
        let assign18140_e33013: f64 = (-locals.var_aigd_i);
        let assign18140_e33014: f64 = (assign18140_e33011 - assign18140_e33013);
        let assign18140_e33016: f64 = (assign18140_e33014 - 1e-6);
        let assign18140_e33019: f64 = (locals.var_aigd1_i * locals.var_deltemp);
        let assign18140_e33021: f64 = (-locals.var_aigd_i);
        let assign18140_e33022: f64 = (assign18140_e33019 - assign18140_e33021);
        let assign18140_e33024: f64 = (assign18140_e33022 - 1e-6);
        let assign18140_e33025: f64 = (assign18140_e33016 * assign18140_e33024);
        let assign18140_e33028: f64 = (-locals.var_aigd_i);
        let assign18140_e33029: f64 = (4.0 * assign18140_e33028);
        let assign18140_e33031: f64 = (assign18140_e33029 * 1e-6);
        let assign18140_e33032: f64 = (assign18140_e33025 - assign18140_e33031);
        let assign18140_e33033: f64 = (assign18140_e33032).sqrt();
        let assign18140_e33034: f64 = (assign18140_e33008 + assign18140_e33033);
        let assign18140_e33035: f64 = (0.5 * assign18140_e33034);
        let assign18140_e33036: f64 = (assign18140_e32999 + assign18140_e33035);
        let assign18140_e33037: f64 = (locals.var_aigd_i + assign18140_e33036);
        locals.var_aigd_t = assign18140_e33037;
        locals.var_aigd_t_dn4 = (0.5 * ((locals.var_aigd1_i * locals.var_deltemp_dn4) + ((((locals.var_aigd1_i * locals.var_deltemp_dn4) * assign18140_e33024) + (assign18140_e33016 * (locals.var_aigd1_i * locals.var_deltemp_dn4))) / (2.0 * assign18140_e33033))));

        let assign18150_e33042: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign18150_e33043: f64 = (1.0 + assign18150_e33042);
        let assign18150_e33045: f64 = (assign18150_e33043 - 1e-6);
        let assign18150_e33047: f64 = (-10000.0);
        let assign18150_e33049: f64 = (assign18150_e33047 * 0.001);
        let (assign18150_e33110, assign18150_e33110_d_n4,) = {
    if (!(assign18150_e33045 < assign18150_e33049)) {
        let assign18150_e33056: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign18150_e33057: f64 = (1.0 + assign18150_e33056);
        let assign18150_e33059: f64 = (assign18150_e33057 - 1e-6);
        let assign18150_e33063: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign18150_e33064: f64 = (1.0 + assign18150_e33063);
        let assign18150_e33066: f64 = (assign18150_e33064 - 1e-6);
        let assign18150_e33070: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign18150_e33071: f64 = (1.0 + assign18150_e33070);
        let assign18150_e33073: f64 = (assign18150_e33071 - 1e-6);
        let assign18150_e33074: f64 = (assign18150_e33066 * assign18150_e33073);
        let assign18150_e33077: f64 = (4.0 * 0.001);
        let assign18150_e33079: f64 = (assign18150_e33077 * 0.001);
        let assign18150_e33080: f64 = (assign18150_e33074 + assign18150_e33079);
        let assign18150_e33081: f64 = (assign18150_e33080).sqrt();
        let assign18150_e33082: f64 = (assign18150_e33059 + assign18150_e33081);
        let assign18150_e33083: f64 = (0.5 * assign18150_e33082);
        (assign18150_e33083, (0.5 * ((locals.var_tgidl_i * locals.var_deltemp_dn4) + ((((locals.var_tgidl_i * locals.var_deltemp_dn4) * assign18150_e33073) + (assign18150_e33066 * (locals.var_tgidl_i * locals.var_deltemp_dn4))) / (2.0 * assign18150_e33081)))),)
    } else {
        let assign18150_e33087: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign18150_e33088: f64 = (1.0 + assign18150_e33087);
        let assign18150_e33090: f64 = (assign18150_e33088 - 1e-6);
        let assign18150_e33092: f64 = (-10000.0);
        let assign18150_e33094: f64 = (assign18150_e33092 * 0.001);
        let (assign18150_e33109, assign18150_e33109_d_n4,) = {
            if (assign18150_e33090 < assign18150_e33094) {
                let assign18150_e33097: f64 = (-0.001);
                let assign18150_e33099: f64 = (assign18150_e33097 * 0.001);
                let assign18150_e33103: f64 = (locals.var_tgidl_i * locals.var_deltemp);
                let assign18150_e33104: f64 = (1.0 + assign18150_e33103);
                let assign18150_e33106: f64 = (assign18150_e33104 - 1e-6);
                let assign18150_e33107: f64 = (assign18150_e33099 / assign18150_e33106);
                (assign18150_e33107, (-((assign18150_e33099 * (locals.var_tgidl_i * locals.var_deltemp_dn4)) / (assign18150_e33106 * assign18150_e33106))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign18150_e33109, assign18150_e33109_d_n4,)
    }
};
        let assign18150_e33111: f64 = (locals.var_bgidl_i * assign18150_e33110);
        locals.var_bgidl_t = assign18150_e33111;
        locals.var_bgidl_t_dn4 = (locals.var_bgidl_i * assign18150_e33110_d_n4);

    }

    pub(super) fn stamp_transient_block_60(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign18160_e33116: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign18160_e33117: f64 = (1.0 + assign18160_e33116);
        let assign18160_e33119: f64 = (assign18160_e33117 - 1e-6);
        let assign18160_e33121: f64 = (-10000.0);
        let assign18160_e33123: f64 = (assign18160_e33121 * 0.001);
        let (assign18160_e33184, assign18160_e33184_d_n4,) = {
    if (!(assign18160_e33119 < assign18160_e33123)) {
        let assign18160_e33130: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign18160_e33131: f64 = (1.0 + assign18160_e33130);
        let assign18160_e33133: f64 = (assign18160_e33131 - 1e-6);
        let assign18160_e33137: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign18160_e33138: f64 = (1.0 + assign18160_e33137);
        let assign18160_e33140: f64 = (assign18160_e33138 - 1e-6);
        let assign18160_e33144: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign18160_e33145: f64 = (1.0 + assign18160_e33144);
        let assign18160_e33147: f64 = (assign18160_e33145 - 1e-6);
        let assign18160_e33148: f64 = (assign18160_e33140 * assign18160_e33147);
        let assign18160_e33151: f64 = (4.0 * 0.001);
        let assign18160_e33153: f64 = (assign18160_e33151 * 0.001);
        let assign18160_e33154: f64 = (assign18160_e33148 + assign18160_e33153);
        let assign18160_e33155: f64 = (assign18160_e33154).sqrt();
        let assign18160_e33156: f64 = (assign18160_e33133 + assign18160_e33155);
        let assign18160_e33157: f64 = (0.5 * assign18160_e33156);
        (assign18160_e33157, (0.5 * ((locals.var_tgidl_i * locals.var_deltemp_dn4) + ((((locals.var_tgidl_i * locals.var_deltemp_dn4) * assign18160_e33147) + (assign18160_e33140 * (locals.var_tgidl_i * locals.var_deltemp_dn4))) / (2.0 * assign18160_e33155)))),)
    } else {
        let assign18160_e33161: f64 = (locals.var_tgidl_i * locals.var_deltemp);
        let assign18160_e33162: f64 = (1.0 + assign18160_e33161);
        let assign18160_e33164: f64 = (assign18160_e33162 - 1e-6);
        let assign18160_e33166: f64 = (-10000.0);
        let assign18160_e33168: f64 = (assign18160_e33166 * 0.001);
        let (assign18160_e33183, assign18160_e33183_d_n4,) = {
            if (assign18160_e33164 < assign18160_e33168) {
                let assign18160_e33171: f64 = (-0.001);
                let assign18160_e33173: f64 = (assign18160_e33171 * 0.001);
                let assign18160_e33177: f64 = (locals.var_tgidl_i * locals.var_deltemp);
                let assign18160_e33178: f64 = (1.0 + assign18160_e33177);
                let assign18160_e33180: f64 = (assign18160_e33178 - 1e-6);
                let assign18160_e33181: f64 = (assign18160_e33173 / assign18160_e33180);
                (assign18160_e33181, (-((assign18160_e33173 * (locals.var_tgidl_i * locals.var_deltemp_dn4)) / (assign18160_e33180 * assign18160_e33180))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign18160_e33183, assign18160_e33183_d_n4,)
    }
};
        let assign18160_e33185: f64 = (locals.var_bgisl_i * assign18160_e33184);
        locals.var_bgisl_t = assign18160_e33185;
        locals.var_bgisl_t_dn4 = (locals.var_bgisl_i * assign18160_e33184_d_n4);

        let assign18170_e33188: f64 = (-locals.var_alpha0_i);
        let assign18170_e33192: f64 = (p.p1437 * locals.var_deltemp);
        let assign18170_e33194: f64 = (-locals.var_alpha0_i);
        let assign18170_e33195: f64 = (assign18170_e33192 - assign18170_e33194);
        let assign18170_e33197: f64 = (assign18170_e33195 - 1e-6);
        let assign18170_e33200: f64 = (p.p1437 * locals.var_deltemp);
        let assign18170_e33202: f64 = (-locals.var_alpha0_i);
        let assign18170_e33203: f64 = (assign18170_e33200 - assign18170_e33202);
        let assign18170_e33205: f64 = (assign18170_e33203 - 1e-6);
        let assign18170_e33208: f64 = (p.p1437 * locals.var_deltemp);
        let assign18170_e33210: f64 = (-locals.var_alpha0_i);
        let assign18170_e33211: f64 = (assign18170_e33208 - assign18170_e33210);
        let assign18170_e33213: f64 = (assign18170_e33211 - 1e-6);
        let assign18170_e33214: f64 = (assign18170_e33205 * assign18170_e33213);
        let assign18170_e33217: f64 = (-locals.var_alpha0_i);
        let assign18170_e33218: f64 = (4.0 * assign18170_e33217);
        let assign18170_e33220: f64 = (assign18170_e33218 * 1e-6);
        let assign18170_e33221: f64 = (assign18170_e33214 - assign18170_e33220);
        let assign18170_e33222: f64 = (assign18170_e33221).sqrt();
        let assign18170_e33223: f64 = (assign18170_e33197 + assign18170_e33222);
        let assign18170_e33224: f64 = (0.5 * assign18170_e33223);
        let assign18170_e33225: f64 = (assign18170_e33188 + assign18170_e33224);
        let assign18170_e33226: f64 = (locals.var_alpha0_i + assign18170_e33225);
        locals.var_alpha0_t = assign18170_e33226;
        locals.var_alpha0_t_dn4 = (0.5 * ((p.p1437 * locals.var_deltemp_dn4) + ((((p.p1437 * locals.var_deltemp_dn4) * assign18170_e33213) + (assign18170_e33205 * (p.p1437 * locals.var_deltemp_dn4))) / (2.0 * assign18170_e33222))));

        let assign18180_e33229: f64 = (-locals.var_alpha1_i);
        let assign18180_e33233: f64 = (p.p1438 * locals.var_deltemp);
        let assign18180_e33235: f64 = (-locals.var_alpha1_i);
        let assign18180_e33236: f64 = (assign18180_e33233 - assign18180_e33235);
        let assign18180_e33238: f64 = (assign18180_e33236 - 1e-6);
        let assign18180_e33241: f64 = (p.p1438 * locals.var_deltemp);
        let assign18180_e33243: f64 = (-locals.var_alpha1_i);
        let assign18180_e33244: f64 = (assign18180_e33241 - assign18180_e33243);
        let assign18180_e33246: f64 = (assign18180_e33244 - 1e-6);
        let assign18180_e33249: f64 = (p.p1438 * locals.var_deltemp);
        let assign18180_e33251: f64 = (-locals.var_alpha1_i);
        let assign18180_e33252: f64 = (assign18180_e33249 - assign18180_e33251);
        let assign18180_e33254: f64 = (assign18180_e33252 - 1e-6);
        let assign18180_e33255: f64 = (assign18180_e33246 * assign18180_e33254);
        let assign18180_e33258: f64 = (-locals.var_alpha1_i);
        let assign18180_e33259: f64 = (4.0 * assign18180_e33258);
        let assign18180_e33261: f64 = (assign18180_e33259 * 1e-6);
        let assign18180_e33262: f64 = (assign18180_e33255 - assign18180_e33261);
        let assign18180_e33263: f64 = (assign18180_e33262).sqrt();
        let assign18180_e33264: f64 = (assign18180_e33238 + assign18180_e33263);
        let assign18180_e33265: f64 = (0.5 * assign18180_e33264);
        let assign18180_e33266: f64 = (assign18180_e33229 + assign18180_e33265);
        let assign18180_e33267: f64 = (locals.var_alpha1_i + assign18180_e33266);
        locals.var_alpha1_t = assign18180_e33267;
        locals.var_alpha1_t_dn4 = (0.5 * ((p.p1438 * locals.var_deltemp_dn4) + ((((p.p1438 * locals.var_deltemp_dn4) * assign18180_e33254) + (assign18180_e33246 * (p.p1438 * locals.var_deltemp_dn4))) / (2.0 * assign18180_e33263))));

        let assign18190_e33270: f64 = (-locals.var_alphaii0_i);
        let assign18190_e33274: f64 = (p.p1439 * locals.var_deltemp);
        let assign18190_e33276: f64 = (-locals.var_alphaii0_i);
        let assign18190_e33277: f64 = (assign18190_e33274 - assign18190_e33276);
        let assign18190_e33279: f64 = (assign18190_e33277 - 1e-25);
        let assign18190_e33282: f64 = (p.p1439 * locals.var_deltemp);
        let assign18190_e33284: f64 = (-locals.var_alphaii0_i);
        let assign18190_e33285: f64 = (assign18190_e33282 - assign18190_e33284);
        let assign18190_e33287: f64 = (assign18190_e33285 - 1e-25);
        let assign18190_e33290: f64 = (p.p1439 * locals.var_deltemp);
        let assign18190_e33292: f64 = (-locals.var_alphaii0_i);
        let assign18190_e33293: f64 = (assign18190_e33290 - assign18190_e33292);
        let assign18190_e33295: f64 = (assign18190_e33293 - 1e-25);
        let assign18190_e33296: f64 = (assign18190_e33287 * assign18190_e33295);
        let assign18190_e33299: f64 = (-locals.var_alphaii0_i);
        let assign18190_e33300: f64 = (4.0 * assign18190_e33299);
        let assign18190_e33302: f64 = (assign18190_e33300 * 1e-25);
        let assign18190_e33303: f64 = (assign18190_e33296 - assign18190_e33302);
        let assign18190_e33304: f64 = (assign18190_e33303).sqrt();
        let assign18190_e33305: f64 = (assign18190_e33279 + assign18190_e33304);
        let assign18190_e33306: f64 = (0.5 * assign18190_e33305);
        let assign18190_e33307: f64 = (assign18190_e33270 + assign18190_e33306);
        let assign18190_e33308: f64 = (locals.var_alphaii0_i + assign18190_e33307);
        locals.var_alphaii0_t = assign18190_e33308;

        let assign18200_e33311: f64 = (-locals.var_alphaii1_i);
        let assign18200_e33315: f64 = (p.p1440 * locals.var_deltemp);
        let assign18200_e33317: f64 = (-locals.var_alphaii1_i);
        let assign18200_e33318: f64 = (assign18200_e33315 - assign18200_e33317);
        let assign18200_e33320: f64 = (assign18200_e33318 - 1e-20);
        let assign18200_e33323: f64 = (p.p1440 * locals.var_deltemp);
        let assign18200_e33325: f64 = (-locals.var_alphaii1_i);
        let assign18200_e33326: f64 = (assign18200_e33323 - assign18200_e33325);
        let assign18200_e33328: f64 = (assign18200_e33326 - 1e-20);
        let assign18200_e33331: f64 = (p.p1440 * locals.var_deltemp);
        let assign18200_e33333: f64 = (-locals.var_alphaii1_i);
        let assign18200_e33334: f64 = (assign18200_e33331 - assign18200_e33333);
        let assign18200_e33336: f64 = (assign18200_e33334 - 1e-20);
        let assign18200_e33337: f64 = (assign18200_e33328 * assign18200_e33336);
        let assign18200_e33340: f64 = (-locals.var_alphaii1_i);
        let assign18200_e33341: f64 = (4.0 * assign18200_e33340);
        let assign18200_e33343: f64 = (assign18200_e33341 * 1e-20);
        let assign18200_e33344: f64 = (assign18200_e33337 - assign18200_e33343);
        let assign18200_e33345: f64 = (assign18200_e33344).sqrt();
        let assign18200_e33346: f64 = (assign18200_e33320 + assign18200_e33345);
        let assign18200_e33347: f64 = (0.5 * assign18200_e33346);
        let assign18200_e33348: f64 = (assign18200_e33311 + assign18200_e33347);
        let assign18200_e33349: f64 = (locals.var_alphaii1_i + assign18200_e33348);
        locals.var_alphaii1_t = assign18200_e33349;

        let assign18230_e33359: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard336 = assign18230_e33359;

        let assign18240_e33362: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard337 = assign18240_e33362;

        let (assign18250_e33408, assign18250_e33408_d_n4,) = {
    if ((locals.var_guard336 != 0.0) && (locals.var_guard337 != 0.0)) {
        let assign18250_e33368: f64 = (-p.p1584);
        let assign18250_e33372: f64 = (p.p1721 * locals.var_deltemp);
        let assign18250_e33374: f64 = (-p.p1584);
        let assign18250_e33375: f64 = (assign18250_e33372 - assign18250_e33374);
        let assign18250_e33377: f64 = (assign18250_e33375 - 1e-6);
        let assign18250_e33380: f64 = (p.p1721 * locals.var_deltemp);
        let assign18250_e33382: f64 = (-p.p1584);
        let assign18250_e33383: f64 = (assign18250_e33380 - assign18250_e33382);
        let assign18250_e33385: f64 = (assign18250_e33383 - 1e-6);
        let assign18250_e33388: f64 = (p.p1721 * locals.var_deltemp);
        let assign18250_e33390: f64 = (-p.p1584);
        let assign18250_e33391: f64 = (assign18250_e33388 - assign18250_e33390);
        let assign18250_e33393: f64 = (assign18250_e33391 - 1e-6);
        let assign18250_e33394: f64 = (assign18250_e33385 * assign18250_e33393);
        let assign18250_e33397: f64 = (-p.p1584);
        let assign18250_e33398: f64 = (4.0 * assign18250_e33397);
        let assign18250_e33400: f64 = (assign18250_e33398 * 1e-6);
        let assign18250_e33401: f64 = (assign18250_e33394 - assign18250_e33400);
        let assign18250_e33402: f64 = (assign18250_e33401).sqrt();
        let assign18250_e33403: f64 = (assign18250_e33377 + assign18250_e33402);
        let assign18250_e33404: f64 = (0.5 * assign18250_e33403);
        let assign18250_e33405: f64 = (assign18250_e33368 + assign18250_e33404);
        let assign18250_e33406: f64 = (p.p1584 + assign18250_e33405);
        (assign18250_e33406, (0.5 * ((p.p1721 * locals.var_deltemp_dn4) + ((((p.p1721 * locals.var_deltemp_dn4) * assign18250_e33393) + (assign18250_e33385 * (p.p1721 * locals.var_deltemp_dn4))) / (2.0 * assign18250_e33402)))),)
    } else {
        (locals.var_cjs_t, locals.var_cjs_t_dn4,)
    }
};
        locals.var_cjs_t = assign18250_e33408;
        locals.var_cjs_t_dn4 = assign18250_e33408_d_n4;

        let (assign18260_e33488, assign18260_e33488_d_n4,) = {
    if ((locals.var_guard336 != 0.0) && (locals.var_guard337 == 0.0)) {
        let assign18260_e33417: f64 = (p.p1721 * locals.var_deltemp);
        let assign18260_e33418: f64 = (1.0 + assign18260_e33417);
        let assign18260_e33420: f64 = (assign18260_e33418 - 1e-6);
        let assign18260_e33422: f64 = (-10000.0);
        let assign18260_e33424: f64 = (assign18260_e33422 * 0.001);
        let (assign18260_e33485, assign18260_e33485_d_n4,) = {
            if (!(assign18260_e33420 < assign18260_e33424)) {
                let assign18260_e33431: f64 = (p.p1721 * locals.var_deltemp);
                let assign18260_e33432: f64 = (1.0 + assign18260_e33431);
                let assign18260_e33434: f64 = (assign18260_e33432 - 1e-6);
                let assign18260_e33438: f64 = (p.p1721 * locals.var_deltemp);
                let assign18260_e33439: f64 = (1.0 + assign18260_e33438);
                let assign18260_e33441: f64 = (assign18260_e33439 - 1e-6);
                let assign18260_e33445: f64 = (p.p1721 * locals.var_deltemp);
                let assign18260_e33446: f64 = (1.0 + assign18260_e33445);
                let assign18260_e33448: f64 = (assign18260_e33446 - 1e-6);
                let assign18260_e33449: f64 = (assign18260_e33441 * assign18260_e33448);
                let assign18260_e33452: f64 = (4.0 * 0.001);
                let assign18260_e33454: f64 = (assign18260_e33452 * 0.001);
                let assign18260_e33455: f64 = (assign18260_e33449 + assign18260_e33454);
                let assign18260_e33456: f64 = (assign18260_e33455).sqrt();
                let assign18260_e33457: f64 = (assign18260_e33434 + assign18260_e33456);
                let assign18260_e33458: f64 = (0.5 * assign18260_e33457);
                (assign18260_e33458, (0.5 * ((p.p1721 * locals.var_deltemp_dn4) + ((((p.p1721 * locals.var_deltemp_dn4) * assign18260_e33448) + (assign18260_e33441 * (p.p1721 * locals.var_deltemp_dn4))) / (2.0 * assign18260_e33456)))),)
            } else {
                let assign18260_e33462: f64 = (p.p1721 * locals.var_deltemp);
                let assign18260_e33463: f64 = (1.0 + assign18260_e33462);
                let assign18260_e33465: f64 = (assign18260_e33463 - 1e-6);
                let assign18260_e33467: f64 = (-10000.0);
                let assign18260_e33469: f64 = (assign18260_e33467 * 0.001);
                let (assign18260_e33484, assign18260_e33484_d_n4,) = {
                    if (assign18260_e33465 < assign18260_e33469) {
                        let assign18260_e33472: f64 = (-0.001);
                        let assign18260_e33474: f64 = (assign18260_e33472 * 0.001);
                        let assign18260_e33478: f64 = (p.p1721 * locals.var_deltemp);
                        let assign18260_e33479: f64 = (1.0 + assign18260_e33478);
                        let assign18260_e33481: f64 = (assign18260_e33479 - 1e-6);
                        let assign18260_e33482: f64 = (assign18260_e33474 / assign18260_e33481);
                        (assign18260_e33482, (-((assign18260_e33474 * (p.p1721 * locals.var_deltemp_dn4)) / (assign18260_e33481 * assign18260_e33481))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18260_e33484, assign18260_e33484_d_n4,)
            }
        };
        let assign18260_e33486: f64 = (p.p1584 * assign18260_e33485);
        (assign18260_e33486, (p.p1584 * assign18260_e33485_d_n4),)
    } else {
        (locals.var_cjs_t, locals.var_cjs_t_dn4,)
    }
};
        locals.var_cjs_t = assign18260_e33488;
        locals.var_cjs_t_dn4 = assign18260_e33488_d_n4;

        let assign18270_e33491: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard338 = assign18270_e33491;

        let (assign18280_e33537, assign18280_e33537_d_n4,) = {
    if ((locals.var_guard336 != 0.0) && (locals.var_guard338 != 0.0)) {
        let assign18280_e33497: f64 = (-p.p1585);
        let assign18280_e33501: f64 = (p.p1721 * locals.var_deltemp);
        let assign18280_e33503: f64 = (-p.p1585);
        let assign18280_e33504: f64 = (assign18280_e33501 - assign18280_e33503);
        let assign18280_e33506: f64 = (assign18280_e33504 - 1e-6);
        let assign18280_e33509: f64 = (p.p1721 * locals.var_deltemp);
        let assign18280_e33511: f64 = (-p.p1585);
        let assign18280_e33512: f64 = (assign18280_e33509 - assign18280_e33511);
        let assign18280_e33514: f64 = (assign18280_e33512 - 1e-6);
        let assign18280_e33517: f64 = (p.p1721 * locals.var_deltemp);
        let assign18280_e33519: f64 = (-p.p1585);
        let assign18280_e33520: f64 = (assign18280_e33517 - assign18280_e33519);
        let assign18280_e33522: f64 = (assign18280_e33520 - 1e-6);
        let assign18280_e33523: f64 = (assign18280_e33514 * assign18280_e33522);
        let assign18280_e33526: f64 = (-p.p1585);
        let assign18280_e33527: f64 = (4.0 * assign18280_e33526);
        let assign18280_e33529: f64 = (assign18280_e33527 * 1e-6);
        let assign18280_e33530: f64 = (assign18280_e33523 - assign18280_e33529);
        let assign18280_e33531: f64 = (assign18280_e33530).sqrt();
        let assign18280_e33532: f64 = (assign18280_e33506 + assign18280_e33531);
        let assign18280_e33533: f64 = (0.5 * assign18280_e33532);
        let assign18280_e33534: f64 = (assign18280_e33497 + assign18280_e33533);
        let assign18280_e33535: f64 = (p.p1585 + assign18280_e33534);
        (assign18280_e33535, (0.5 * ((p.p1721 * locals.var_deltemp_dn4) + ((((p.p1721 * locals.var_deltemp_dn4) * assign18280_e33522) + (assign18280_e33514 * (p.p1721 * locals.var_deltemp_dn4))) / (2.0 * assign18280_e33531)))),)
    } else {
        (locals.var_cjd_t, locals.var_cjd_t_dn4,)
    }
};
        locals.var_cjd_t = assign18280_e33537;
        locals.var_cjd_t_dn4 = assign18280_e33537_d_n4;

        let (assign18290_e33617, assign18290_e33617_d_n4,) = {
    if ((locals.var_guard336 != 0.0) && (locals.var_guard338 == 0.0)) {
        let assign18290_e33546: f64 = (p.p1721 * locals.var_deltemp);
        let assign18290_e33547: f64 = (1.0 + assign18290_e33546);
        let assign18290_e33549: f64 = (assign18290_e33547 - 1e-6);
        let assign18290_e33551: f64 = (-10000.0);
        let assign18290_e33553: f64 = (assign18290_e33551 * 0.001);
        let (assign18290_e33614, assign18290_e33614_d_n4,) = {
            if (!(assign18290_e33549 < assign18290_e33553)) {
                let assign18290_e33560: f64 = (p.p1721 * locals.var_deltemp);
                let assign18290_e33561: f64 = (1.0 + assign18290_e33560);
                let assign18290_e33563: f64 = (assign18290_e33561 - 1e-6);
                let assign18290_e33567: f64 = (p.p1721 * locals.var_deltemp);
                let assign18290_e33568: f64 = (1.0 + assign18290_e33567);
                let assign18290_e33570: f64 = (assign18290_e33568 - 1e-6);
                let assign18290_e33574: f64 = (p.p1721 * locals.var_deltemp);
                let assign18290_e33575: f64 = (1.0 + assign18290_e33574);
                let assign18290_e33577: f64 = (assign18290_e33575 - 1e-6);
                let assign18290_e33578: f64 = (assign18290_e33570 * assign18290_e33577);
                let assign18290_e33581: f64 = (4.0 * 0.001);
                let assign18290_e33583: f64 = (assign18290_e33581 * 0.001);
                let assign18290_e33584: f64 = (assign18290_e33578 + assign18290_e33583);
                let assign18290_e33585: f64 = (assign18290_e33584).sqrt();
                let assign18290_e33586: f64 = (assign18290_e33563 + assign18290_e33585);
                let assign18290_e33587: f64 = (0.5 * assign18290_e33586);
                (assign18290_e33587, (0.5 * ((p.p1721 * locals.var_deltemp_dn4) + ((((p.p1721 * locals.var_deltemp_dn4) * assign18290_e33577) + (assign18290_e33570 * (p.p1721 * locals.var_deltemp_dn4))) / (2.0 * assign18290_e33585)))),)
            } else {
                let assign18290_e33591: f64 = (p.p1721 * locals.var_deltemp);
                let assign18290_e33592: f64 = (1.0 + assign18290_e33591);
                let assign18290_e33594: f64 = (assign18290_e33592 - 1e-6);
                let assign18290_e33596: f64 = (-10000.0);
                let assign18290_e33598: f64 = (assign18290_e33596 * 0.001);
                let (assign18290_e33613, assign18290_e33613_d_n4,) = {
                    if (assign18290_e33594 < assign18290_e33598) {
                        let assign18290_e33601: f64 = (-0.001);
                        let assign18290_e33603: f64 = (assign18290_e33601 * 0.001);
                        let assign18290_e33607: f64 = (p.p1721 * locals.var_deltemp);
                        let assign18290_e33608: f64 = (1.0 + assign18290_e33607);
                        let assign18290_e33610: f64 = (assign18290_e33608 - 1e-6);
                        let assign18290_e33611: f64 = (assign18290_e33603 / assign18290_e33610);
                        (assign18290_e33611, (-((assign18290_e33603 * (p.p1721 * locals.var_deltemp_dn4)) / (assign18290_e33610 * assign18290_e33610))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18290_e33613, assign18290_e33613_d_n4,)
            }
        };
        let assign18290_e33615: f64 = (p.p1585 * assign18290_e33614);
        (assign18290_e33615, (p.p1585 * assign18290_e33614_d_n4),)
    } else {
        (locals.var_cjd_t, locals.var_cjd_t_dn4,)
    }
};
        locals.var_cjd_t = assign18290_e33617;
        locals.var_cjd_t_dn4 = assign18290_e33617_d_n4;

        let assign18300_e33620: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard339 = assign18300_e33620;

        let (assign18310_e33666, assign18310_e33666_d_n4,) = {
    if ((locals.var_guard336 != 0.0) && (locals.var_guard339 != 0.0)) {
        let assign18310_e33626: f64 = (-p.p1586);
        let assign18310_e33630: f64 = (p.p1722 * locals.var_deltemp);
        let assign18310_e33632: f64 = (-p.p1586);
        let assign18310_e33633: f64 = (assign18310_e33630 - assign18310_e33632);
        let assign18310_e33635: f64 = (assign18310_e33633 - 1e-6);
        let assign18310_e33638: f64 = (p.p1722 * locals.var_deltemp);
        let assign18310_e33640: f64 = (-p.p1586);
        let assign18310_e33641: f64 = (assign18310_e33638 - assign18310_e33640);
        let assign18310_e33643: f64 = (assign18310_e33641 - 1e-6);
        let assign18310_e33646: f64 = (p.p1722 * locals.var_deltemp);
        let assign18310_e33648: f64 = (-p.p1586);
        let assign18310_e33649: f64 = (assign18310_e33646 - assign18310_e33648);
        let assign18310_e33651: f64 = (assign18310_e33649 - 1e-6);
        let assign18310_e33652: f64 = (assign18310_e33643 * assign18310_e33651);
        let assign18310_e33655: f64 = (-p.p1586);
        let assign18310_e33656: f64 = (4.0 * assign18310_e33655);
        let assign18310_e33658: f64 = (assign18310_e33656 * 1e-6);
        let assign18310_e33659: f64 = (assign18310_e33652 - assign18310_e33658);
        let assign18310_e33660: f64 = (assign18310_e33659).sqrt();
        let assign18310_e33661: f64 = (assign18310_e33635 + assign18310_e33660);
        let assign18310_e33662: f64 = (0.5 * assign18310_e33661);
        let assign18310_e33663: f64 = (assign18310_e33626 + assign18310_e33662);
        let assign18310_e33664: f64 = (p.p1586 + assign18310_e33663);
        (assign18310_e33664, (0.5 * ((p.p1722 * locals.var_deltemp_dn4) + ((((p.p1722 * locals.var_deltemp_dn4) * assign18310_e33651) + (assign18310_e33643 * (p.p1722 * locals.var_deltemp_dn4))) / (2.0 * assign18310_e33660)))),)
    } else {
        (locals.var_cjsws_t, locals.var_cjsws_t_dn4,)
    }
};
        locals.var_cjsws_t = assign18310_e33666;
        locals.var_cjsws_t_dn4 = assign18310_e33666_d_n4;

        let (assign18320_e33746, assign18320_e33746_d_n4,) = {
    if ((locals.var_guard336 != 0.0) && (locals.var_guard339 == 0.0)) {
        let assign18320_e33675: f64 = (p.p1722 * locals.var_deltemp);
        let assign18320_e33676: f64 = (1.0 + assign18320_e33675);
        let assign18320_e33678: f64 = (assign18320_e33676 - 1e-6);
        let assign18320_e33680: f64 = (-10000.0);
        let assign18320_e33682: f64 = (assign18320_e33680 * 0.001);
        let (assign18320_e33743, assign18320_e33743_d_n4,) = {
            if (!(assign18320_e33678 < assign18320_e33682)) {
                let assign18320_e33689: f64 = (p.p1722 * locals.var_deltemp);
                let assign18320_e33690: f64 = (1.0 + assign18320_e33689);
                let assign18320_e33692: f64 = (assign18320_e33690 - 1e-6);
                let assign18320_e33696: f64 = (p.p1722 * locals.var_deltemp);
                let assign18320_e33697: f64 = (1.0 + assign18320_e33696);
                let assign18320_e33699: f64 = (assign18320_e33697 - 1e-6);
                let assign18320_e33703: f64 = (p.p1722 * locals.var_deltemp);
                let assign18320_e33704: f64 = (1.0 + assign18320_e33703);
                let assign18320_e33706: f64 = (assign18320_e33704 - 1e-6);
                let assign18320_e33707: f64 = (assign18320_e33699 * assign18320_e33706);
                let assign18320_e33710: f64 = (4.0 * 0.001);
                let assign18320_e33712: f64 = (assign18320_e33710 * 0.001);
                let assign18320_e33713: f64 = (assign18320_e33707 + assign18320_e33712);
                let assign18320_e33714: f64 = (assign18320_e33713).sqrt();
                let assign18320_e33715: f64 = (assign18320_e33692 + assign18320_e33714);
                let assign18320_e33716: f64 = (0.5 * assign18320_e33715);
                (assign18320_e33716, (0.5 * ((p.p1722 * locals.var_deltemp_dn4) + ((((p.p1722 * locals.var_deltemp_dn4) * assign18320_e33706) + (assign18320_e33699 * (p.p1722 * locals.var_deltemp_dn4))) / (2.0 * assign18320_e33714)))),)
            } else {
                let assign18320_e33720: f64 = (p.p1722 * locals.var_deltemp);
                let assign18320_e33721: f64 = (1.0 + assign18320_e33720);
                let assign18320_e33723: f64 = (assign18320_e33721 - 1e-6);
                let assign18320_e33725: f64 = (-10000.0);
                let assign18320_e33727: f64 = (assign18320_e33725 * 0.001);
                let (assign18320_e33742, assign18320_e33742_d_n4,) = {
                    if (assign18320_e33723 < assign18320_e33727) {
                        let assign18320_e33730: f64 = (-0.001);
                        let assign18320_e33732: f64 = (assign18320_e33730 * 0.001);
                        let assign18320_e33736: f64 = (p.p1722 * locals.var_deltemp);
                        let assign18320_e33737: f64 = (1.0 + assign18320_e33736);
                        let assign18320_e33739: f64 = (assign18320_e33737 - 1e-6);
                        let assign18320_e33740: f64 = (assign18320_e33732 / assign18320_e33739);
                        (assign18320_e33740, (-((assign18320_e33732 * (p.p1722 * locals.var_deltemp_dn4)) / (assign18320_e33739 * assign18320_e33739))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18320_e33742, assign18320_e33742_d_n4,)
            }
        };
        let assign18320_e33744: f64 = (p.p1586 * assign18320_e33743);
        (assign18320_e33744, (p.p1586 * assign18320_e33743_d_n4),)
    } else {
        (locals.var_cjsws_t, locals.var_cjsws_t_dn4,)
    }
};
        locals.var_cjsws_t = assign18320_e33746;
        locals.var_cjsws_t_dn4 = assign18320_e33746_d_n4;

        let assign18330_e33749: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard340 = assign18330_e33749;

        let (assign18340_e33795, assign18340_e33795_d_n4,) = {
    if ((locals.var_guard336 != 0.0) && (locals.var_guard340 != 0.0)) {
        let assign18340_e33755: f64 = (-p.p1587);
        let assign18340_e33759: f64 = (p.p1722 * locals.var_deltemp);
        let assign18340_e33761: f64 = (-p.p1587);
        let assign18340_e33762: f64 = (assign18340_e33759 - assign18340_e33761);
        let assign18340_e33764: f64 = (assign18340_e33762 - 1e-6);
        let assign18340_e33767: f64 = (p.p1722 * locals.var_deltemp);
        let assign18340_e33769: f64 = (-p.p1587);
        let assign18340_e33770: f64 = (assign18340_e33767 - assign18340_e33769);
        let assign18340_e33772: f64 = (assign18340_e33770 - 1e-6);
        let assign18340_e33775: f64 = (p.p1722 * locals.var_deltemp);
        let assign18340_e33777: f64 = (-p.p1587);
        let assign18340_e33778: f64 = (assign18340_e33775 - assign18340_e33777);
        let assign18340_e33780: f64 = (assign18340_e33778 - 1e-6);
        let assign18340_e33781: f64 = (assign18340_e33772 * assign18340_e33780);
        let assign18340_e33784: f64 = (-p.p1587);
        let assign18340_e33785: f64 = (4.0 * assign18340_e33784);
        let assign18340_e33787: f64 = (assign18340_e33785 * 1e-6);
        let assign18340_e33788: f64 = (assign18340_e33781 - assign18340_e33787);
        let assign18340_e33789: f64 = (assign18340_e33788).sqrt();
        let assign18340_e33790: f64 = (assign18340_e33764 + assign18340_e33789);
        let assign18340_e33791: f64 = (0.5 * assign18340_e33790);
        let assign18340_e33792: f64 = (assign18340_e33755 + assign18340_e33791);
        let assign18340_e33793: f64 = (p.p1587 + assign18340_e33792);
        (assign18340_e33793, (0.5 * ((p.p1722 * locals.var_deltemp_dn4) + ((((p.p1722 * locals.var_deltemp_dn4) * assign18340_e33780) + (assign18340_e33772 * (p.p1722 * locals.var_deltemp_dn4))) / (2.0 * assign18340_e33789)))),)
    } else {
        (locals.var_cjswd_t, locals.var_cjswd_t_dn4,)
    }
};
        locals.var_cjswd_t = assign18340_e33795;
        locals.var_cjswd_t_dn4 = assign18340_e33795_d_n4;

    }

    pub(super) fn stamp_transient_block_61(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18350_e33875, assign18350_e33875_d_n4,) = {
    if ((locals.var_guard336 != 0.0) && (locals.var_guard340 == 0.0)) {
        let assign18350_e33804: f64 = (p.p1722 * locals.var_deltemp);
        let assign18350_e33805: f64 = (1.0 + assign18350_e33804);
        let assign18350_e33807: f64 = (assign18350_e33805 - 1e-6);
        let assign18350_e33809: f64 = (-10000.0);
        let assign18350_e33811: f64 = (assign18350_e33809 * 0.001);
        let (assign18350_e33872, assign18350_e33872_d_n4,) = {
            if (!(assign18350_e33807 < assign18350_e33811)) {
                let assign18350_e33818: f64 = (p.p1722 * locals.var_deltemp);
                let assign18350_e33819: f64 = (1.0 + assign18350_e33818);
                let assign18350_e33821: f64 = (assign18350_e33819 - 1e-6);
                let assign18350_e33825: f64 = (p.p1722 * locals.var_deltemp);
                let assign18350_e33826: f64 = (1.0 + assign18350_e33825);
                let assign18350_e33828: f64 = (assign18350_e33826 - 1e-6);
                let assign18350_e33832: f64 = (p.p1722 * locals.var_deltemp);
                let assign18350_e33833: f64 = (1.0 + assign18350_e33832);
                let assign18350_e33835: f64 = (assign18350_e33833 - 1e-6);
                let assign18350_e33836: f64 = (assign18350_e33828 * assign18350_e33835);
                let assign18350_e33839: f64 = (4.0 * 0.001);
                let assign18350_e33841: f64 = (assign18350_e33839 * 0.001);
                let assign18350_e33842: f64 = (assign18350_e33836 + assign18350_e33841);
                let assign18350_e33843: f64 = (assign18350_e33842).sqrt();
                let assign18350_e33844: f64 = (assign18350_e33821 + assign18350_e33843);
                let assign18350_e33845: f64 = (0.5 * assign18350_e33844);
                (assign18350_e33845, (0.5 * ((p.p1722 * locals.var_deltemp_dn4) + ((((p.p1722 * locals.var_deltemp_dn4) * assign18350_e33835) + (assign18350_e33828 * (p.p1722 * locals.var_deltemp_dn4))) / (2.0 * assign18350_e33843)))),)
            } else {
                let assign18350_e33849: f64 = (p.p1722 * locals.var_deltemp);
                let assign18350_e33850: f64 = (1.0 + assign18350_e33849);
                let assign18350_e33852: f64 = (assign18350_e33850 - 1e-6);
                let assign18350_e33854: f64 = (-10000.0);
                let assign18350_e33856: f64 = (assign18350_e33854 * 0.001);
                let (assign18350_e33871, assign18350_e33871_d_n4,) = {
                    if (assign18350_e33852 < assign18350_e33856) {
                        let assign18350_e33859: f64 = (-0.001);
                        let assign18350_e33861: f64 = (assign18350_e33859 * 0.001);
                        let assign18350_e33865: f64 = (p.p1722 * locals.var_deltemp);
                        let assign18350_e33866: f64 = (1.0 + assign18350_e33865);
                        let assign18350_e33868: f64 = (assign18350_e33866 - 1e-6);
                        let assign18350_e33869: f64 = (assign18350_e33861 / assign18350_e33868);
                        (assign18350_e33869, (-((assign18350_e33861 * (p.p1722 * locals.var_deltemp_dn4)) / (assign18350_e33868 * assign18350_e33868))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18350_e33871, assign18350_e33871_d_n4,)
            }
        };
        let assign18350_e33873: f64 = (p.p1587 * assign18350_e33872);
        (assign18350_e33873, (p.p1587 * assign18350_e33872_d_n4),)
    } else {
        (locals.var_cjswd_t, locals.var_cjswd_t_dn4,)
    }
};
        locals.var_cjswd_t = assign18350_e33875;
        locals.var_cjswd_t_dn4 = assign18350_e33875_d_n4;

        let assign18360_e33878: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard341 = assign18360_e33878;

        let (assign18370_e33924, assign18370_e33924_d_n4,) = {
    if ((locals.var_guard336 != 0.0) && (locals.var_guard341 != 0.0)) {
        let assign18370_e33884: f64 = (-p.p1588);
        let assign18370_e33888: f64 = (p.p1723 * locals.var_deltemp);
        let assign18370_e33890: f64 = (-p.p1588);
        let assign18370_e33891: f64 = (assign18370_e33888 - assign18370_e33890);
        let assign18370_e33893: f64 = (assign18370_e33891 - 1e-6);
        let assign18370_e33896: f64 = (p.p1723 * locals.var_deltemp);
        let assign18370_e33898: f64 = (-p.p1588);
        let assign18370_e33899: f64 = (assign18370_e33896 - assign18370_e33898);
        let assign18370_e33901: f64 = (assign18370_e33899 - 1e-6);
        let assign18370_e33904: f64 = (p.p1723 * locals.var_deltemp);
        let assign18370_e33906: f64 = (-p.p1588);
        let assign18370_e33907: f64 = (assign18370_e33904 - assign18370_e33906);
        let assign18370_e33909: f64 = (assign18370_e33907 - 1e-6);
        let assign18370_e33910: f64 = (assign18370_e33901 * assign18370_e33909);
        let assign18370_e33913: f64 = (-p.p1588);
        let assign18370_e33914: f64 = (4.0 * assign18370_e33913);
        let assign18370_e33916: f64 = (assign18370_e33914 * 1e-6);
        let assign18370_e33917: f64 = (assign18370_e33910 - assign18370_e33916);
        let assign18370_e33918: f64 = (assign18370_e33917).sqrt();
        let assign18370_e33919: f64 = (assign18370_e33893 + assign18370_e33918);
        let assign18370_e33920: f64 = (0.5 * assign18370_e33919);
        let assign18370_e33921: f64 = (assign18370_e33884 + assign18370_e33920);
        let assign18370_e33922: f64 = (p.p1588 + assign18370_e33921);
        (assign18370_e33922, (0.5 * ((p.p1723 * locals.var_deltemp_dn4) + ((((p.p1723 * locals.var_deltemp_dn4) * assign18370_e33909) + (assign18370_e33901 * (p.p1723 * locals.var_deltemp_dn4))) / (2.0 * assign18370_e33918)))),)
    } else {
        (locals.var_cjswgs_t, locals.var_cjswgs_t_dn4,)
    }
};
        locals.var_cjswgs_t = assign18370_e33924;
        locals.var_cjswgs_t_dn4 = assign18370_e33924_d_n4;

        let (assign18380_e34004, assign18380_e34004_d_n4,) = {
    if ((locals.var_guard336 != 0.0) && (locals.var_guard341 == 0.0)) {
        let assign18380_e33933: f64 = (p.p1723 * locals.var_deltemp);
        let assign18380_e33934: f64 = (1.0 + assign18380_e33933);
        let assign18380_e33936: f64 = (assign18380_e33934 - 1e-6);
        let assign18380_e33938: f64 = (-10000.0);
        let assign18380_e33940: f64 = (assign18380_e33938 * 0.001);
        let (assign18380_e34001, assign18380_e34001_d_n4,) = {
            if (!(assign18380_e33936 < assign18380_e33940)) {
                let assign18380_e33947: f64 = (p.p1723 * locals.var_deltemp);
                let assign18380_e33948: f64 = (1.0 + assign18380_e33947);
                let assign18380_e33950: f64 = (assign18380_e33948 - 1e-6);
                let assign18380_e33954: f64 = (p.p1723 * locals.var_deltemp);
                let assign18380_e33955: f64 = (1.0 + assign18380_e33954);
                let assign18380_e33957: f64 = (assign18380_e33955 - 1e-6);
                let assign18380_e33961: f64 = (p.p1723 * locals.var_deltemp);
                let assign18380_e33962: f64 = (1.0 + assign18380_e33961);
                let assign18380_e33964: f64 = (assign18380_e33962 - 1e-6);
                let assign18380_e33965: f64 = (assign18380_e33957 * assign18380_e33964);
                let assign18380_e33968: f64 = (4.0 * 0.001);
                let assign18380_e33970: f64 = (assign18380_e33968 * 0.001);
                let assign18380_e33971: f64 = (assign18380_e33965 + assign18380_e33970);
                let assign18380_e33972: f64 = (assign18380_e33971).sqrt();
                let assign18380_e33973: f64 = (assign18380_e33950 + assign18380_e33972);
                let assign18380_e33974: f64 = (0.5 * assign18380_e33973);
                (assign18380_e33974, (0.5 * ((p.p1723 * locals.var_deltemp_dn4) + ((((p.p1723 * locals.var_deltemp_dn4) * assign18380_e33964) + (assign18380_e33957 * (p.p1723 * locals.var_deltemp_dn4))) / (2.0 * assign18380_e33972)))),)
            } else {
                let assign18380_e33978: f64 = (p.p1723 * locals.var_deltemp);
                let assign18380_e33979: f64 = (1.0 + assign18380_e33978);
                let assign18380_e33981: f64 = (assign18380_e33979 - 1e-6);
                let assign18380_e33983: f64 = (-10000.0);
                let assign18380_e33985: f64 = (assign18380_e33983 * 0.001);
                let (assign18380_e34000, assign18380_e34000_d_n4,) = {
                    if (assign18380_e33981 < assign18380_e33985) {
                        let assign18380_e33988: f64 = (-0.001);
                        let assign18380_e33990: f64 = (assign18380_e33988 * 0.001);
                        let assign18380_e33994: f64 = (p.p1723 * locals.var_deltemp);
                        let assign18380_e33995: f64 = (1.0 + assign18380_e33994);
                        let assign18380_e33997: f64 = (assign18380_e33995 - 1e-6);
                        let assign18380_e33998: f64 = (assign18380_e33990 / assign18380_e33997);
                        (assign18380_e33998, (-((assign18380_e33990 * (p.p1723 * locals.var_deltemp_dn4)) / (assign18380_e33997 * assign18380_e33997))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18380_e34000, assign18380_e34000_d_n4,)
            }
        };
        let assign18380_e34002: f64 = (p.p1588 * assign18380_e34001);
        (assign18380_e34002, (p.p1588 * assign18380_e34001_d_n4),)
    } else {
        (locals.var_cjswgs_t, locals.var_cjswgs_t_dn4,)
    }
};
        locals.var_cjswgs_t = assign18380_e34004;
        locals.var_cjswgs_t_dn4 = assign18380_e34004_d_n4;

        let assign18390_e34007: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard342 = assign18390_e34007;

        let (assign18400_e34053, assign18400_e34053_d_n4,) = {
    if ((locals.var_guard336 != 0.0) && (locals.var_guard342 != 0.0)) {
        let assign18400_e34013: f64 = (-p.p1589);
        let assign18400_e34017: f64 = (p.p1723 * locals.var_deltemp);
        let assign18400_e34019: f64 = (-p.p1589);
        let assign18400_e34020: f64 = (assign18400_e34017 - assign18400_e34019);
        let assign18400_e34022: f64 = (assign18400_e34020 - 1e-6);
        let assign18400_e34025: f64 = (p.p1723 * locals.var_deltemp);
        let assign18400_e34027: f64 = (-p.p1589);
        let assign18400_e34028: f64 = (assign18400_e34025 - assign18400_e34027);
        let assign18400_e34030: f64 = (assign18400_e34028 - 1e-6);
        let assign18400_e34033: f64 = (p.p1723 * locals.var_deltemp);
        let assign18400_e34035: f64 = (-p.p1589);
        let assign18400_e34036: f64 = (assign18400_e34033 - assign18400_e34035);
        let assign18400_e34038: f64 = (assign18400_e34036 - 1e-6);
        let assign18400_e34039: f64 = (assign18400_e34030 * assign18400_e34038);
        let assign18400_e34042: f64 = (-p.p1589);
        let assign18400_e34043: f64 = (4.0 * assign18400_e34042);
        let assign18400_e34045: f64 = (assign18400_e34043 * 1e-6);
        let assign18400_e34046: f64 = (assign18400_e34039 - assign18400_e34045);
        let assign18400_e34047: f64 = (assign18400_e34046).sqrt();
        let assign18400_e34048: f64 = (assign18400_e34022 + assign18400_e34047);
        let assign18400_e34049: f64 = (0.5 * assign18400_e34048);
        let assign18400_e34050: f64 = (assign18400_e34013 + assign18400_e34049);
        let assign18400_e34051: f64 = (p.p1589 + assign18400_e34050);
        (assign18400_e34051, (0.5 * ((p.p1723 * locals.var_deltemp_dn4) + ((((p.p1723 * locals.var_deltemp_dn4) * assign18400_e34038) + (assign18400_e34030 * (p.p1723 * locals.var_deltemp_dn4))) / (2.0 * assign18400_e34047)))),)
    } else {
        (locals.var_cjswgd_t, locals.var_cjswgd_t_dn4,)
    }
};
        locals.var_cjswgd_t = assign18400_e34053;
        locals.var_cjswgd_t_dn4 = assign18400_e34053_d_n4;

        let (assign18410_e34133, assign18410_e34133_d_n4,) = {
    if ((locals.var_guard336 != 0.0) && (locals.var_guard342 == 0.0)) {
        let assign18410_e34062: f64 = (p.p1723 * locals.var_deltemp);
        let assign18410_e34063: f64 = (1.0 + assign18410_e34062);
        let assign18410_e34065: f64 = (assign18410_e34063 - 1e-6);
        let assign18410_e34067: f64 = (-10000.0);
        let assign18410_e34069: f64 = (assign18410_e34067 * 0.001);
        let (assign18410_e34130, assign18410_e34130_d_n4,) = {
            if (!(assign18410_e34065 < assign18410_e34069)) {
                let assign18410_e34076: f64 = (p.p1723 * locals.var_deltemp);
                let assign18410_e34077: f64 = (1.0 + assign18410_e34076);
                let assign18410_e34079: f64 = (assign18410_e34077 - 1e-6);
                let assign18410_e34083: f64 = (p.p1723 * locals.var_deltemp);
                let assign18410_e34084: f64 = (1.0 + assign18410_e34083);
                let assign18410_e34086: f64 = (assign18410_e34084 - 1e-6);
                let assign18410_e34090: f64 = (p.p1723 * locals.var_deltemp);
                let assign18410_e34091: f64 = (1.0 + assign18410_e34090);
                let assign18410_e34093: f64 = (assign18410_e34091 - 1e-6);
                let assign18410_e34094: f64 = (assign18410_e34086 * assign18410_e34093);
                let assign18410_e34097: f64 = (4.0 * 0.001);
                let assign18410_e34099: f64 = (assign18410_e34097 * 0.001);
                let assign18410_e34100: f64 = (assign18410_e34094 + assign18410_e34099);
                let assign18410_e34101: f64 = (assign18410_e34100).sqrt();
                let assign18410_e34102: f64 = (assign18410_e34079 + assign18410_e34101);
                let assign18410_e34103: f64 = (0.5 * assign18410_e34102);
                (assign18410_e34103, (0.5 * ((p.p1723 * locals.var_deltemp_dn4) + ((((p.p1723 * locals.var_deltemp_dn4) * assign18410_e34093) + (assign18410_e34086 * (p.p1723 * locals.var_deltemp_dn4))) / (2.0 * assign18410_e34101)))),)
            } else {
                let assign18410_e34107: f64 = (p.p1723 * locals.var_deltemp);
                let assign18410_e34108: f64 = (1.0 + assign18410_e34107);
                let assign18410_e34110: f64 = (assign18410_e34108 - 1e-6);
                let assign18410_e34112: f64 = (-10000.0);
                let assign18410_e34114: f64 = (assign18410_e34112 * 0.001);
                let (assign18410_e34129, assign18410_e34129_d_n4,) = {
                    if (assign18410_e34110 < assign18410_e34114) {
                        let assign18410_e34117: f64 = (-0.001);
                        let assign18410_e34119: f64 = (assign18410_e34117 * 0.001);
                        let assign18410_e34123: f64 = (p.p1723 * locals.var_deltemp);
                        let assign18410_e34124: f64 = (1.0 + assign18410_e34123);
                        let assign18410_e34126: f64 = (assign18410_e34124 - 1e-6);
                        let assign18410_e34127: f64 = (assign18410_e34119 / assign18410_e34126);
                        (assign18410_e34127, (-((assign18410_e34119 * (p.p1723 * locals.var_deltemp_dn4)) / (assign18410_e34126 * assign18410_e34126))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18410_e34129, assign18410_e34129_d_n4,)
            }
        };
        let assign18410_e34131: f64 = (p.p1589 * assign18410_e34130);
        (assign18410_e34131, (p.p1589 * assign18410_e34130_d_n4),)
    } else {
        (locals.var_cjswgd_t, locals.var_cjswgd_t_dn4,)
    }
};
        locals.var_cjswgd_t = assign18410_e34133;
        locals.var_cjswgd_t_dn4 = assign18410_e34133_d_n4;

        let (assign18420_e34210, assign18420_e34210_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18420_e34138: f64 = (p.p1724 * locals.var_deltemp);
        let assign18420_e34139: f64 = (p.p1590 - assign18420_e34138);
        let assign18420_e34141: f64 = (assign18420_e34139 - 0.01);
        let assign18420_e34143: f64 = (-10000.0);
        let assign18420_e34145: f64 = (assign18420_e34143 * 0.001);
        let (assign18420_e34206, assign18420_e34206_d_n4,) = {
            if (!(assign18420_e34141 < assign18420_e34145)) {
                let assign18420_e34152: f64 = (p.p1724 * locals.var_deltemp);
                let assign18420_e34153: f64 = (p.p1590 - assign18420_e34152);
                let assign18420_e34155: f64 = (assign18420_e34153 - 0.01);
                let assign18420_e34159: f64 = (p.p1724 * locals.var_deltemp);
                let assign18420_e34160: f64 = (p.p1590 - assign18420_e34159);
                let assign18420_e34162: f64 = (assign18420_e34160 - 0.01);
                let assign18420_e34166: f64 = (p.p1724 * locals.var_deltemp);
                let assign18420_e34167: f64 = (p.p1590 - assign18420_e34166);
                let assign18420_e34169: f64 = (assign18420_e34167 - 0.01);
                let assign18420_e34170: f64 = (assign18420_e34162 * assign18420_e34169);
                let assign18420_e34173: f64 = (4.0 * 0.001);
                let assign18420_e34175: f64 = (assign18420_e34173 * 0.001);
                let assign18420_e34176: f64 = (assign18420_e34170 + assign18420_e34175);
                let assign18420_e34177: f64 = (assign18420_e34176).sqrt();
                let assign18420_e34178: f64 = (assign18420_e34155 + assign18420_e34177);
                let assign18420_e34179: f64 = (0.5 * assign18420_e34178);
                (assign18420_e34179, (0.5 * ((-(p.p1724 * locals.var_deltemp_dn4)) + ((((-(p.p1724 * locals.var_deltemp_dn4)) * assign18420_e34169) + (assign18420_e34162 * (-(p.p1724 * locals.var_deltemp_dn4)))) / (2.0 * assign18420_e34177)))),)
            } else {
                let assign18420_e34183: f64 = (p.p1724 * locals.var_deltemp);
                let assign18420_e34184: f64 = (p.p1590 - assign18420_e34183);
                let assign18420_e34186: f64 = (assign18420_e34184 - 0.01);
                let assign18420_e34188: f64 = (-10000.0);
                let assign18420_e34190: f64 = (assign18420_e34188 * 0.001);
                let (assign18420_e34205, assign18420_e34205_d_n4,) = {
                    if (assign18420_e34186 < assign18420_e34190) {
                        let assign18420_e34193: f64 = (-0.001);
                        let assign18420_e34195: f64 = (assign18420_e34193 * 0.001);
                        let assign18420_e34199: f64 = (p.p1724 * locals.var_deltemp);
                        let assign18420_e34200: f64 = (p.p1590 - assign18420_e34199);
                        let assign18420_e34202: f64 = (assign18420_e34200 - 0.01);
                        let assign18420_e34203: f64 = (assign18420_e34195 / assign18420_e34202);
                        (assign18420_e34203, (-((assign18420_e34195 * (-(p.p1724 * locals.var_deltemp_dn4))) / (assign18420_e34202 * assign18420_e34202))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18420_e34205, assign18420_e34205_d_n4,)
            }
        };
        let assign18420_e34208: f64 = (assign18420_e34206 + 0.01);
        (assign18420_e34208, assign18420_e34206_d_n4,)
    } else {
        (locals.var_pbs_t, locals.var_pbs_t_dn4,)
    }
};
        locals.var_pbs_t = assign18420_e34210;
        locals.var_pbs_t_dn4 = assign18420_e34210_d_n4;

        let (assign18430_e34287, assign18430_e34287_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18430_e34215: f64 = (p.p1724 * locals.var_deltemp);
        let assign18430_e34216: f64 = (p.p1591 - assign18430_e34215);
        let assign18430_e34218: f64 = (assign18430_e34216 - 0.01);
        let assign18430_e34220: f64 = (-10000.0);
        let assign18430_e34222: f64 = (assign18430_e34220 * 0.001);
        let (assign18430_e34283, assign18430_e34283_d_n4,) = {
            if (!(assign18430_e34218 < assign18430_e34222)) {
                let assign18430_e34229: f64 = (p.p1724 * locals.var_deltemp);
                let assign18430_e34230: f64 = (p.p1591 - assign18430_e34229);
                let assign18430_e34232: f64 = (assign18430_e34230 - 0.01);
                let assign18430_e34236: f64 = (p.p1724 * locals.var_deltemp);
                let assign18430_e34237: f64 = (p.p1591 - assign18430_e34236);
                let assign18430_e34239: f64 = (assign18430_e34237 - 0.01);
                let assign18430_e34243: f64 = (p.p1724 * locals.var_deltemp);
                let assign18430_e34244: f64 = (p.p1591 - assign18430_e34243);
                let assign18430_e34246: f64 = (assign18430_e34244 - 0.01);
                let assign18430_e34247: f64 = (assign18430_e34239 * assign18430_e34246);
                let assign18430_e34250: f64 = (4.0 * 0.001);
                let assign18430_e34252: f64 = (assign18430_e34250 * 0.001);
                let assign18430_e34253: f64 = (assign18430_e34247 + assign18430_e34252);
                let assign18430_e34254: f64 = (assign18430_e34253).sqrt();
                let assign18430_e34255: f64 = (assign18430_e34232 + assign18430_e34254);
                let assign18430_e34256: f64 = (0.5 * assign18430_e34255);
                (assign18430_e34256, (0.5 * ((-(p.p1724 * locals.var_deltemp_dn4)) + ((((-(p.p1724 * locals.var_deltemp_dn4)) * assign18430_e34246) + (assign18430_e34239 * (-(p.p1724 * locals.var_deltemp_dn4)))) / (2.0 * assign18430_e34254)))),)
            } else {
                let assign18430_e34260: f64 = (p.p1724 * locals.var_deltemp);
                let assign18430_e34261: f64 = (p.p1591 - assign18430_e34260);
                let assign18430_e34263: f64 = (assign18430_e34261 - 0.01);
                let assign18430_e34265: f64 = (-10000.0);
                let assign18430_e34267: f64 = (assign18430_e34265 * 0.001);
                let (assign18430_e34282, assign18430_e34282_d_n4,) = {
                    if (assign18430_e34263 < assign18430_e34267) {
                        let assign18430_e34270: f64 = (-0.001);
                        let assign18430_e34272: f64 = (assign18430_e34270 * 0.001);
                        let assign18430_e34276: f64 = (p.p1724 * locals.var_deltemp);
                        let assign18430_e34277: f64 = (p.p1591 - assign18430_e34276);
                        let assign18430_e34279: f64 = (assign18430_e34277 - 0.01);
                        let assign18430_e34280: f64 = (assign18430_e34272 / assign18430_e34279);
                        (assign18430_e34280, (-((assign18430_e34272 * (-(p.p1724 * locals.var_deltemp_dn4))) / (assign18430_e34279 * assign18430_e34279))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18430_e34282, assign18430_e34282_d_n4,)
            }
        };
        let assign18430_e34285: f64 = (assign18430_e34283 + 0.01);
        (assign18430_e34285, assign18430_e34283_d_n4,)
    } else {
        (locals.var_pbd_t, locals.var_pbd_t_dn4,)
    }
};
        locals.var_pbd_t = assign18430_e34287;
        locals.var_pbd_t_dn4 = assign18430_e34287_d_n4;

        let (assign18440_e34364, assign18440_e34364_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18440_e34292: f64 = (p.p1725 * locals.var_deltemp);
        let assign18440_e34293: f64 = (p.p1592 - assign18440_e34292);
        let assign18440_e34295: f64 = (assign18440_e34293 - 0.01);
        let assign18440_e34297: f64 = (-10000.0);
        let assign18440_e34299: f64 = (assign18440_e34297 * 0.001);
        let (assign18440_e34360, assign18440_e34360_d_n4,) = {
            if (!(assign18440_e34295 < assign18440_e34299)) {
                let assign18440_e34306: f64 = (p.p1725 * locals.var_deltemp);
                let assign18440_e34307: f64 = (p.p1592 - assign18440_e34306);
                let assign18440_e34309: f64 = (assign18440_e34307 - 0.01);
                let assign18440_e34313: f64 = (p.p1725 * locals.var_deltemp);
                let assign18440_e34314: f64 = (p.p1592 - assign18440_e34313);
                let assign18440_e34316: f64 = (assign18440_e34314 - 0.01);
                let assign18440_e34320: f64 = (p.p1725 * locals.var_deltemp);
                let assign18440_e34321: f64 = (p.p1592 - assign18440_e34320);
                let assign18440_e34323: f64 = (assign18440_e34321 - 0.01);
                let assign18440_e34324: f64 = (assign18440_e34316 * assign18440_e34323);
                let assign18440_e34327: f64 = (4.0 * 0.001);
                let assign18440_e34329: f64 = (assign18440_e34327 * 0.001);
                let assign18440_e34330: f64 = (assign18440_e34324 + assign18440_e34329);
                let assign18440_e34331: f64 = (assign18440_e34330).sqrt();
                let assign18440_e34332: f64 = (assign18440_e34309 + assign18440_e34331);
                let assign18440_e34333: f64 = (0.5 * assign18440_e34332);
                (assign18440_e34333, (0.5 * ((-(p.p1725 * locals.var_deltemp_dn4)) + ((((-(p.p1725 * locals.var_deltemp_dn4)) * assign18440_e34323) + (assign18440_e34316 * (-(p.p1725 * locals.var_deltemp_dn4)))) / (2.0 * assign18440_e34331)))),)
            } else {
                let assign18440_e34337: f64 = (p.p1725 * locals.var_deltemp);
                let assign18440_e34338: f64 = (p.p1592 - assign18440_e34337);
                let assign18440_e34340: f64 = (assign18440_e34338 - 0.01);
                let assign18440_e34342: f64 = (-10000.0);
                let assign18440_e34344: f64 = (assign18440_e34342 * 0.001);
                let (assign18440_e34359, assign18440_e34359_d_n4,) = {
                    if (assign18440_e34340 < assign18440_e34344) {
                        let assign18440_e34347: f64 = (-0.001);
                        let assign18440_e34349: f64 = (assign18440_e34347 * 0.001);
                        let assign18440_e34353: f64 = (p.p1725 * locals.var_deltemp);
                        let assign18440_e34354: f64 = (p.p1592 - assign18440_e34353);
                        let assign18440_e34356: f64 = (assign18440_e34354 - 0.01);
                        let assign18440_e34357: f64 = (assign18440_e34349 / assign18440_e34356);
                        (assign18440_e34357, (-((assign18440_e34349 * (-(p.p1725 * locals.var_deltemp_dn4))) / (assign18440_e34356 * assign18440_e34356))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18440_e34359, assign18440_e34359_d_n4,)
            }
        };
        let assign18440_e34362: f64 = (assign18440_e34360 + 0.01);
        (assign18440_e34362, assign18440_e34360_d_n4,)
    } else {
        (locals.var_pbsws_t, locals.var_pbsws_t_dn4,)
    }
};
        locals.var_pbsws_t = assign18440_e34364;
        locals.var_pbsws_t_dn4 = assign18440_e34364_d_n4;

        let (assign18450_e34441, assign18450_e34441_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18450_e34369: f64 = (p.p1725 * locals.var_deltemp);
        let assign18450_e34370: f64 = (p.p1593 - assign18450_e34369);
        let assign18450_e34372: f64 = (assign18450_e34370 - 0.01);
        let assign18450_e34374: f64 = (-10000.0);
        let assign18450_e34376: f64 = (assign18450_e34374 * 0.001);
        let (assign18450_e34437, assign18450_e34437_d_n4,) = {
            if (!(assign18450_e34372 < assign18450_e34376)) {
                let assign18450_e34383: f64 = (p.p1725 * locals.var_deltemp);
                let assign18450_e34384: f64 = (p.p1593 - assign18450_e34383);
                let assign18450_e34386: f64 = (assign18450_e34384 - 0.01);
                let assign18450_e34390: f64 = (p.p1725 * locals.var_deltemp);
                let assign18450_e34391: f64 = (p.p1593 - assign18450_e34390);
                let assign18450_e34393: f64 = (assign18450_e34391 - 0.01);
                let assign18450_e34397: f64 = (p.p1725 * locals.var_deltemp);
                let assign18450_e34398: f64 = (p.p1593 - assign18450_e34397);
                let assign18450_e34400: f64 = (assign18450_e34398 - 0.01);
                let assign18450_e34401: f64 = (assign18450_e34393 * assign18450_e34400);
                let assign18450_e34404: f64 = (4.0 * 0.001);
                let assign18450_e34406: f64 = (assign18450_e34404 * 0.001);
                let assign18450_e34407: f64 = (assign18450_e34401 + assign18450_e34406);
                let assign18450_e34408: f64 = (assign18450_e34407).sqrt();
                let assign18450_e34409: f64 = (assign18450_e34386 + assign18450_e34408);
                let assign18450_e34410: f64 = (0.5 * assign18450_e34409);
                (assign18450_e34410, (0.5 * ((-(p.p1725 * locals.var_deltemp_dn4)) + ((((-(p.p1725 * locals.var_deltemp_dn4)) * assign18450_e34400) + (assign18450_e34393 * (-(p.p1725 * locals.var_deltemp_dn4)))) / (2.0 * assign18450_e34408)))),)
            } else {
                let assign18450_e34414: f64 = (p.p1725 * locals.var_deltemp);
                let assign18450_e34415: f64 = (p.p1593 - assign18450_e34414);
                let assign18450_e34417: f64 = (assign18450_e34415 - 0.01);
                let assign18450_e34419: f64 = (-10000.0);
                let assign18450_e34421: f64 = (assign18450_e34419 * 0.001);
                let (assign18450_e34436, assign18450_e34436_d_n4,) = {
                    if (assign18450_e34417 < assign18450_e34421) {
                        let assign18450_e34424: f64 = (-0.001);
                        let assign18450_e34426: f64 = (assign18450_e34424 * 0.001);
                        let assign18450_e34430: f64 = (p.p1725 * locals.var_deltemp);
                        let assign18450_e34431: f64 = (p.p1593 - assign18450_e34430);
                        let assign18450_e34433: f64 = (assign18450_e34431 - 0.01);
                        let assign18450_e34434: f64 = (assign18450_e34426 / assign18450_e34433);
                        (assign18450_e34434, (-((assign18450_e34426 * (-(p.p1725 * locals.var_deltemp_dn4))) / (assign18450_e34433 * assign18450_e34433))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18450_e34436, assign18450_e34436_d_n4,)
            }
        };
        let assign18450_e34439: f64 = (assign18450_e34437 + 0.01);
        (assign18450_e34439, assign18450_e34437_d_n4,)
    } else {
        (locals.var_pbswd_t, locals.var_pbswd_t_dn4,)
    }
};
        locals.var_pbswd_t = assign18450_e34441;
        locals.var_pbswd_t_dn4 = assign18450_e34441_d_n4;

    }

    pub(super) fn stamp_transient_block_62(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18460_e34518, assign18460_e34518_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18460_e34446: f64 = (p.p1726 * locals.var_deltemp);
        let assign18460_e34447: f64 = (p.p1594 - assign18460_e34446);
        let assign18460_e34449: f64 = (assign18460_e34447 - 0.01);
        let assign18460_e34451: f64 = (-10000.0);
        let assign18460_e34453: f64 = (assign18460_e34451 * 0.001);
        let (assign18460_e34514, assign18460_e34514_d_n4,) = {
            if (!(assign18460_e34449 < assign18460_e34453)) {
                let assign18460_e34460: f64 = (p.p1726 * locals.var_deltemp);
                let assign18460_e34461: f64 = (p.p1594 - assign18460_e34460);
                let assign18460_e34463: f64 = (assign18460_e34461 - 0.01);
                let assign18460_e34467: f64 = (p.p1726 * locals.var_deltemp);
                let assign18460_e34468: f64 = (p.p1594 - assign18460_e34467);
                let assign18460_e34470: f64 = (assign18460_e34468 - 0.01);
                let assign18460_e34474: f64 = (p.p1726 * locals.var_deltemp);
                let assign18460_e34475: f64 = (p.p1594 - assign18460_e34474);
                let assign18460_e34477: f64 = (assign18460_e34475 - 0.01);
                let assign18460_e34478: f64 = (assign18460_e34470 * assign18460_e34477);
                let assign18460_e34481: f64 = (4.0 * 0.001);
                let assign18460_e34483: f64 = (assign18460_e34481 * 0.001);
                let assign18460_e34484: f64 = (assign18460_e34478 + assign18460_e34483);
                let assign18460_e34485: f64 = (assign18460_e34484).sqrt();
                let assign18460_e34486: f64 = (assign18460_e34463 + assign18460_e34485);
                let assign18460_e34487: f64 = (0.5 * assign18460_e34486);
                (assign18460_e34487, (0.5 * ((-(p.p1726 * locals.var_deltemp_dn4)) + ((((-(p.p1726 * locals.var_deltemp_dn4)) * assign18460_e34477) + (assign18460_e34470 * (-(p.p1726 * locals.var_deltemp_dn4)))) / (2.0 * assign18460_e34485)))),)
            } else {
                let assign18460_e34491: f64 = (p.p1726 * locals.var_deltemp);
                let assign18460_e34492: f64 = (p.p1594 - assign18460_e34491);
                let assign18460_e34494: f64 = (assign18460_e34492 - 0.01);
                let assign18460_e34496: f64 = (-10000.0);
                let assign18460_e34498: f64 = (assign18460_e34496 * 0.001);
                let (assign18460_e34513, assign18460_e34513_d_n4,) = {
                    if (assign18460_e34494 < assign18460_e34498) {
                        let assign18460_e34501: f64 = (-0.001);
                        let assign18460_e34503: f64 = (assign18460_e34501 * 0.001);
                        let assign18460_e34507: f64 = (p.p1726 * locals.var_deltemp);
                        let assign18460_e34508: f64 = (p.p1594 - assign18460_e34507);
                        let assign18460_e34510: f64 = (assign18460_e34508 - 0.01);
                        let assign18460_e34511: f64 = (assign18460_e34503 / assign18460_e34510);
                        (assign18460_e34511, (-((assign18460_e34503 * (-(p.p1726 * locals.var_deltemp_dn4))) / (assign18460_e34510 * assign18460_e34510))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18460_e34513, assign18460_e34513_d_n4,)
            }
        };
        let assign18460_e34516: f64 = (assign18460_e34514 + 0.01);
        (assign18460_e34516, assign18460_e34514_d_n4,)
    } else {
        (locals.var_pbswgs_t, locals.var_pbswgs_t_dn4,)
    }
};
        locals.var_pbswgs_t = assign18460_e34518;
        locals.var_pbswgs_t_dn4 = assign18460_e34518_d_n4;

        let (assign18470_e34595, assign18470_e34595_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18470_e34523: f64 = (p.p1726 * locals.var_deltemp);
        let assign18470_e34524: f64 = (p.p1595 - assign18470_e34523);
        let assign18470_e34526: f64 = (assign18470_e34524 - 0.01);
        let assign18470_e34528: f64 = (-10000.0);
        let assign18470_e34530: f64 = (assign18470_e34528 * 0.001);
        let (assign18470_e34591, assign18470_e34591_d_n4,) = {
            if (!(assign18470_e34526 < assign18470_e34530)) {
                let assign18470_e34537: f64 = (p.p1726 * locals.var_deltemp);
                let assign18470_e34538: f64 = (p.p1595 - assign18470_e34537);
                let assign18470_e34540: f64 = (assign18470_e34538 - 0.01);
                let assign18470_e34544: f64 = (p.p1726 * locals.var_deltemp);
                let assign18470_e34545: f64 = (p.p1595 - assign18470_e34544);
                let assign18470_e34547: f64 = (assign18470_e34545 - 0.01);
                let assign18470_e34551: f64 = (p.p1726 * locals.var_deltemp);
                let assign18470_e34552: f64 = (p.p1595 - assign18470_e34551);
                let assign18470_e34554: f64 = (assign18470_e34552 - 0.01);
                let assign18470_e34555: f64 = (assign18470_e34547 * assign18470_e34554);
                let assign18470_e34558: f64 = (4.0 * 0.001);
                let assign18470_e34560: f64 = (assign18470_e34558 * 0.001);
                let assign18470_e34561: f64 = (assign18470_e34555 + assign18470_e34560);
                let assign18470_e34562: f64 = (assign18470_e34561).sqrt();
                let assign18470_e34563: f64 = (assign18470_e34540 + assign18470_e34562);
                let assign18470_e34564: f64 = (0.5 * assign18470_e34563);
                (assign18470_e34564, (0.5 * ((-(p.p1726 * locals.var_deltemp_dn4)) + ((((-(p.p1726 * locals.var_deltemp_dn4)) * assign18470_e34554) + (assign18470_e34547 * (-(p.p1726 * locals.var_deltemp_dn4)))) / (2.0 * assign18470_e34562)))),)
            } else {
                let assign18470_e34568: f64 = (p.p1726 * locals.var_deltemp);
                let assign18470_e34569: f64 = (p.p1595 - assign18470_e34568);
                let assign18470_e34571: f64 = (assign18470_e34569 - 0.01);
                let assign18470_e34573: f64 = (-10000.0);
                let assign18470_e34575: f64 = (assign18470_e34573 * 0.001);
                let (assign18470_e34590, assign18470_e34590_d_n4,) = {
                    if (assign18470_e34571 < assign18470_e34575) {
                        let assign18470_e34578: f64 = (-0.001);
                        let assign18470_e34580: f64 = (assign18470_e34578 * 0.001);
                        let assign18470_e34584: f64 = (p.p1726 * locals.var_deltemp);
                        let assign18470_e34585: f64 = (p.p1595 - assign18470_e34584);
                        let assign18470_e34587: f64 = (assign18470_e34585 - 0.01);
                        let assign18470_e34588: f64 = (assign18470_e34580 / assign18470_e34587);
                        (assign18470_e34588, (-((assign18470_e34580 * (-(p.p1726 * locals.var_deltemp_dn4))) / (assign18470_e34587 * assign18470_e34587))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18470_e34590, assign18470_e34590_d_n4,)
            }
        };
        let assign18470_e34593: f64 = (assign18470_e34591 + 0.01);
        (assign18470_e34593, assign18470_e34591_d_n4,)
    } else {
        (locals.var_pbswgd_t, locals.var_pbswgd_t_dn4,)
    }
};
        locals.var_pbswgd_t = assign18470_e34595;
        locals.var_pbswgd_t_dn4 = assign18470_e34595_d_n4;

        let (assign18480_e34605, assign18480_e34605_d_n0, assign18480_e34605_d_n2, assign18480_e34605_d_n3, assign18480_e34605_d_n4, assign18480_e34605_d_n5, assign18480_e34605_d_n6, assign18480_e34605_d_n7, assign18480_e34605_d_n8, assign18480_e34605_d_n9, assign18480_e34605_d_n10, assign18480_e34605_d_n11, assign18480_e34605_d_n13, assign18480_e34605_d_n14,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18480_e34599: f64 = (locals.var_eg0 / locals.var_vtm0);
        let assign18480_e34602: f64 = (locals.var_eg / locals.var_vtm);
        let assign18480_e34603: f64 = (assign18480_e34599 - assign18480_e34602);
        (assign18480_e34603, 0.0, 0.0, 0.0, (-(((locals.var_eg_dn4 * locals.var_vtm) - (locals.var_eg * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18480_e34605;
        locals.var_t0_dn0 = assign18480_e34605_d_n0;
        locals.var_t0_dn2 = assign18480_e34605_d_n2;
        locals.var_t0_dn3 = assign18480_e34605_d_n3;
        locals.var_t0_dn4 = assign18480_e34605_d_n4;
        locals.var_t0_dn5 = assign18480_e34605_d_n5;
        locals.var_t0_dn6 = assign18480_e34605_d_n6;
        locals.var_t0_dn7 = assign18480_e34605_d_n7;
        locals.var_t0_dn8 = assign18480_e34605_d_n8;
        locals.var_t0_dn9 = assign18480_e34605_d_n9;
        locals.var_t0_dn10 = assign18480_e34605_d_n10;
        locals.var_t0_dn11 = assign18480_e34605_d_n11;
        locals.var_t0_dn13 = assign18480_e34605_d_n13;
        locals.var_t0_dn14 = assign18480_e34605_d_n14;

        let (assign18490_e34616, assign18490_e34616_d_n0, assign18490_e34616_d_n2, assign18490_e34616_d_n3, assign18490_e34616_d_n4, assign18490_e34616_d_n5, assign18490_e34616_d_n6, assign18490_e34616_d_n7, assign18490_e34616_d_n8, assign18490_e34616_d_n9, assign18490_e34616_d_n10, assign18490_e34616_d_n11, assign18490_e34616_d_n13, assign18490_e34616_d_n14,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18490_e34610: f64 = (p.p1727 * locals.var_trat_ln);
        let assign18490_e34611: f64 = (locals.var_t0 + assign18490_e34610);
        let assign18490_e34613: f64 = (assign18490_e34611 / p.p1620);
        let assign18490_e34614: f64 = { let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign18490_e34614, ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 + (p.p1727 * locals.var_trat_ln_dn4)) / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn10 / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn11 / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn13 / p.p1620)), ({ let limited_exp_arg = assign18490_e34613; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 / p.p1620)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign18490_e34616;
        locals.var_t3_dn0 = assign18490_e34616_d_n0;
        locals.var_t3_dn2 = assign18490_e34616_d_n2;
        locals.var_t3_dn3 = assign18490_e34616_d_n3;
        locals.var_t3_dn4 = assign18490_e34616_d_n4;
        locals.var_t3_dn5 = assign18490_e34616_d_n5;
        locals.var_t3_dn6 = assign18490_e34616_d_n6;
        locals.var_t3_dn7 = assign18490_e34616_d_n7;
        locals.var_t3_dn8 = assign18490_e34616_d_n8;
        locals.var_t3_dn9 = assign18490_e34616_d_n9;
        locals.var_t3_dn10 = assign18490_e34616_d_n10;
        locals.var_t3_dn11 = assign18490_e34616_d_n11;
        locals.var_t3_dn13 = assign18490_e34616_d_n13;
        locals.var_t3_dn14 = assign18490_e34616_d_n14;

        let (assign18500_e34622, assign18500_e34622_d_n0, assign18500_e34622_d_n2, assign18500_e34622_d_n3, assign18500_e34622_d_n4, assign18500_e34622_d_n5, assign18500_e34622_d_n6, assign18500_e34622_d_n7, assign18500_e34622_d_n8, assign18500_e34622_d_n9, assign18500_e34622_d_n10, assign18500_e34622_d_n11, assign18500_e34622_d_n13, assign18500_e34622_d_n14,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18500_e34620: f64 = (p.p1614 * locals.var_t3);
        (assign18500_e34620, (p.p1614 * locals.var_t3_dn0), (p.p1614 * locals.var_t3_dn2), (p.p1614 * locals.var_t3_dn3), (p.p1614 * locals.var_t3_dn4), (p.p1614 * locals.var_t3_dn5), (p.p1614 * locals.var_t3_dn6), (p.p1614 * locals.var_t3_dn7), (p.p1614 * locals.var_t3_dn8), (p.p1614 * locals.var_t3_dn9), (p.p1614 * locals.var_t3_dn10), (p.p1614 * locals.var_t3_dn11), (p.p1614 * locals.var_t3_dn13), (p.p1614 * locals.var_t3_dn14),)
    } else {
        (locals.var_jss_t, locals.var_jss_t_dn0, locals.var_jss_t_dn2, locals.var_jss_t_dn3, locals.var_jss_t_dn4, locals.var_jss_t_dn5, locals.var_jss_t_dn6, locals.var_jss_t_dn7, locals.var_jss_t_dn8, locals.var_jss_t_dn9, locals.var_jss_t_dn10, locals.var_jss_t_dn11, locals.var_jss_t_dn13, locals.var_jss_t_dn14,)
    }
};
        locals.var_jss_t = assign18500_e34622;
        locals.var_jss_t_dn0 = assign18500_e34622_d_n0;
        locals.var_jss_t_dn2 = assign18500_e34622_d_n2;
        locals.var_jss_t_dn3 = assign18500_e34622_d_n3;
        locals.var_jss_t_dn4 = assign18500_e34622_d_n4;
        locals.var_jss_t_dn5 = assign18500_e34622_d_n5;
        locals.var_jss_t_dn6 = assign18500_e34622_d_n6;
        locals.var_jss_t_dn7 = assign18500_e34622_d_n7;
        locals.var_jss_t_dn8 = assign18500_e34622_d_n8;
        locals.var_jss_t_dn9 = assign18500_e34622_d_n9;
        locals.var_jss_t_dn10 = assign18500_e34622_d_n10;
        locals.var_jss_t_dn11 = assign18500_e34622_d_n11;
        locals.var_jss_t_dn13 = assign18500_e34622_d_n13;
        locals.var_jss_t_dn14 = assign18500_e34622_d_n14;

        let (assign18510_e34628, assign18510_e34628_d_n0, assign18510_e34628_d_n2, assign18510_e34628_d_n3, assign18510_e34628_d_n4, assign18510_e34628_d_n5, assign18510_e34628_d_n6, assign18510_e34628_d_n7, assign18510_e34628_d_n8, assign18510_e34628_d_n9, assign18510_e34628_d_n10, assign18510_e34628_d_n11, assign18510_e34628_d_n13, assign18510_e34628_d_n14,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18510_e34626: f64 = (p.p1616 * locals.var_t3);
        (assign18510_e34626, (p.p1616 * locals.var_t3_dn0), (p.p1616 * locals.var_t3_dn2), (p.p1616 * locals.var_t3_dn3), (p.p1616 * locals.var_t3_dn4), (p.p1616 * locals.var_t3_dn5), (p.p1616 * locals.var_t3_dn6), (p.p1616 * locals.var_t3_dn7), (p.p1616 * locals.var_t3_dn8), (p.p1616 * locals.var_t3_dn9), (p.p1616 * locals.var_t3_dn10), (p.p1616 * locals.var_t3_dn11), (p.p1616 * locals.var_t3_dn13), (p.p1616 * locals.var_t3_dn14),)
    } else {
        (locals.var_jsws_t, locals.var_jsws_t_dn0, locals.var_jsws_t_dn2, locals.var_jsws_t_dn3, locals.var_jsws_t_dn4, locals.var_jsws_t_dn5, locals.var_jsws_t_dn6, locals.var_jsws_t_dn7, locals.var_jsws_t_dn8, locals.var_jsws_t_dn9, locals.var_jsws_t_dn10, locals.var_jsws_t_dn11, locals.var_jsws_t_dn13, locals.var_jsws_t_dn14,)
    }
};
        locals.var_jsws_t = assign18510_e34628;
        locals.var_jsws_t_dn0 = assign18510_e34628_d_n0;
        locals.var_jsws_t_dn2 = assign18510_e34628_d_n2;
        locals.var_jsws_t_dn3 = assign18510_e34628_d_n3;
        locals.var_jsws_t_dn4 = assign18510_e34628_d_n4;
        locals.var_jsws_t_dn5 = assign18510_e34628_d_n5;
        locals.var_jsws_t_dn6 = assign18510_e34628_d_n6;
        locals.var_jsws_t_dn7 = assign18510_e34628_d_n7;
        locals.var_jsws_t_dn8 = assign18510_e34628_d_n8;
        locals.var_jsws_t_dn9 = assign18510_e34628_d_n9;
        locals.var_jsws_t_dn10 = assign18510_e34628_d_n10;
        locals.var_jsws_t_dn11 = assign18510_e34628_d_n11;
        locals.var_jsws_t_dn13 = assign18510_e34628_d_n13;
        locals.var_jsws_t_dn14 = assign18510_e34628_d_n14;

        let (assign18520_e34634, assign18520_e34634_d_n0, assign18520_e34634_d_n2, assign18520_e34634_d_n3, assign18520_e34634_d_n4, assign18520_e34634_d_n5, assign18520_e34634_d_n6, assign18520_e34634_d_n7, assign18520_e34634_d_n8, assign18520_e34634_d_n9, assign18520_e34634_d_n10, assign18520_e34634_d_n11, assign18520_e34634_d_n13, assign18520_e34634_d_n14,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18520_e34632: f64 = (p.p1618 * locals.var_t3);
        (assign18520_e34632, (p.p1618 * locals.var_t3_dn0), (p.p1618 * locals.var_t3_dn2), (p.p1618 * locals.var_t3_dn3), (p.p1618 * locals.var_t3_dn4), (p.p1618 * locals.var_t3_dn5), (p.p1618 * locals.var_t3_dn6), (p.p1618 * locals.var_t3_dn7), (p.p1618 * locals.var_t3_dn8), (p.p1618 * locals.var_t3_dn9), (p.p1618 * locals.var_t3_dn10), (p.p1618 * locals.var_t3_dn11), (p.p1618 * locals.var_t3_dn13), (p.p1618 * locals.var_t3_dn14),)
    } else {
        (locals.var_jswgs_t, locals.var_jswgs_t_dn0, locals.var_jswgs_t_dn2, locals.var_jswgs_t_dn3, locals.var_jswgs_t_dn4, locals.var_jswgs_t_dn5, locals.var_jswgs_t_dn6, locals.var_jswgs_t_dn7, locals.var_jswgs_t_dn8, locals.var_jswgs_t_dn9, locals.var_jswgs_t_dn10, locals.var_jswgs_t_dn11, locals.var_jswgs_t_dn13, locals.var_jswgs_t_dn14,)
    }
};
        locals.var_jswgs_t = assign18520_e34634;
        locals.var_jswgs_t_dn0 = assign18520_e34634_d_n0;
        locals.var_jswgs_t_dn2 = assign18520_e34634_d_n2;
        locals.var_jswgs_t_dn3 = assign18520_e34634_d_n3;
        locals.var_jswgs_t_dn4 = assign18520_e34634_d_n4;
        locals.var_jswgs_t_dn5 = assign18520_e34634_d_n5;
        locals.var_jswgs_t_dn6 = assign18520_e34634_d_n6;
        locals.var_jswgs_t_dn7 = assign18520_e34634_d_n7;
        locals.var_jswgs_t_dn8 = assign18520_e34634_d_n8;
        locals.var_jswgs_t_dn9 = assign18520_e34634_d_n9;
        locals.var_jswgs_t_dn10 = assign18520_e34634_d_n10;
        locals.var_jswgs_t_dn11 = assign18520_e34634_d_n11;
        locals.var_jswgs_t_dn13 = assign18520_e34634_d_n13;
        locals.var_jswgs_t_dn14 = assign18520_e34634_d_n14;

        let (assign18530_e34645, assign18530_e34645_d_n0, assign18530_e34645_d_n2, assign18530_e34645_d_n3, assign18530_e34645_d_n4, assign18530_e34645_d_n5, assign18530_e34645_d_n6, assign18530_e34645_d_n7, assign18530_e34645_d_n8, assign18530_e34645_d_n9, assign18530_e34645_d_n10, assign18530_e34645_d_n11, assign18530_e34645_d_n13, assign18530_e34645_d_n14,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18530_e34639: f64 = (p.p1728 * locals.var_trat_ln);
        let assign18530_e34640: f64 = (locals.var_t0 + assign18530_e34639);
        let assign18530_e34642: f64 = (assign18530_e34640 / p.p1621);
        let assign18530_e34643: f64 = { let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign18530_e34643, ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 + (p.p1728 * locals.var_trat_ln_dn4)) / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn10 / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn11 / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn13 / p.p1621)), ({ let limited_exp_arg = assign18530_e34642; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 / p.p1621)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign18530_e34645;
        locals.var_t3_dn0 = assign18530_e34645_d_n0;
        locals.var_t3_dn2 = assign18530_e34645_d_n2;
        locals.var_t3_dn3 = assign18530_e34645_d_n3;
        locals.var_t3_dn4 = assign18530_e34645_d_n4;
        locals.var_t3_dn5 = assign18530_e34645_d_n5;
        locals.var_t3_dn6 = assign18530_e34645_d_n6;
        locals.var_t3_dn7 = assign18530_e34645_d_n7;
        locals.var_t3_dn8 = assign18530_e34645_d_n8;
        locals.var_t3_dn9 = assign18530_e34645_d_n9;
        locals.var_t3_dn10 = assign18530_e34645_d_n10;
        locals.var_t3_dn11 = assign18530_e34645_d_n11;
        locals.var_t3_dn13 = assign18530_e34645_d_n13;
        locals.var_t3_dn14 = assign18530_e34645_d_n14;

        let (assign18540_e34651, assign18540_e34651_d_n0, assign18540_e34651_d_n2, assign18540_e34651_d_n3, assign18540_e34651_d_n4, assign18540_e34651_d_n5, assign18540_e34651_d_n6, assign18540_e34651_d_n7, assign18540_e34651_d_n8, assign18540_e34651_d_n9, assign18540_e34651_d_n10, assign18540_e34651_d_n11, assign18540_e34651_d_n13, assign18540_e34651_d_n14,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18540_e34649: f64 = (p.p1615 * locals.var_t3);
        (assign18540_e34649, (p.p1615 * locals.var_t3_dn0), (p.p1615 * locals.var_t3_dn2), (p.p1615 * locals.var_t3_dn3), (p.p1615 * locals.var_t3_dn4), (p.p1615 * locals.var_t3_dn5), (p.p1615 * locals.var_t3_dn6), (p.p1615 * locals.var_t3_dn7), (p.p1615 * locals.var_t3_dn8), (p.p1615 * locals.var_t3_dn9), (p.p1615 * locals.var_t3_dn10), (p.p1615 * locals.var_t3_dn11), (p.p1615 * locals.var_t3_dn13), (p.p1615 * locals.var_t3_dn14),)
    } else {
        (locals.var_jsd_t, locals.var_jsd_t_dn0, locals.var_jsd_t_dn2, locals.var_jsd_t_dn3, locals.var_jsd_t_dn4, locals.var_jsd_t_dn5, locals.var_jsd_t_dn6, locals.var_jsd_t_dn7, locals.var_jsd_t_dn8, locals.var_jsd_t_dn9, locals.var_jsd_t_dn10, locals.var_jsd_t_dn11, locals.var_jsd_t_dn13, locals.var_jsd_t_dn14,)
    }
};
        locals.var_jsd_t = assign18540_e34651;
        locals.var_jsd_t_dn0 = assign18540_e34651_d_n0;
        locals.var_jsd_t_dn2 = assign18540_e34651_d_n2;
        locals.var_jsd_t_dn3 = assign18540_e34651_d_n3;
        locals.var_jsd_t_dn4 = assign18540_e34651_d_n4;
        locals.var_jsd_t_dn5 = assign18540_e34651_d_n5;
        locals.var_jsd_t_dn6 = assign18540_e34651_d_n6;
        locals.var_jsd_t_dn7 = assign18540_e34651_d_n7;
        locals.var_jsd_t_dn8 = assign18540_e34651_d_n8;
        locals.var_jsd_t_dn9 = assign18540_e34651_d_n9;
        locals.var_jsd_t_dn10 = assign18540_e34651_d_n10;
        locals.var_jsd_t_dn11 = assign18540_e34651_d_n11;
        locals.var_jsd_t_dn13 = assign18540_e34651_d_n13;
        locals.var_jsd_t_dn14 = assign18540_e34651_d_n14;

        let (assign18550_e34657, assign18550_e34657_d_n0, assign18550_e34657_d_n2, assign18550_e34657_d_n3, assign18550_e34657_d_n4, assign18550_e34657_d_n5, assign18550_e34657_d_n6, assign18550_e34657_d_n7, assign18550_e34657_d_n8, assign18550_e34657_d_n9, assign18550_e34657_d_n10, assign18550_e34657_d_n11, assign18550_e34657_d_n13, assign18550_e34657_d_n14,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18550_e34655: f64 = (p.p1617 * locals.var_t3);
        (assign18550_e34655, (p.p1617 * locals.var_t3_dn0), (p.p1617 * locals.var_t3_dn2), (p.p1617 * locals.var_t3_dn3), (p.p1617 * locals.var_t3_dn4), (p.p1617 * locals.var_t3_dn5), (p.p1617 * locals.var_t3_dn6), (p.p1617 * locals.var_t3_dn7), (p.p1617 * locals.var_t3_dn8), (p.p1617 * locals.var_t3_dn9), (p.p1617 * locals.var_t3_dn10), (p.p1617 * locals.var_t3_dn11), (p.p1617 * locals.var_t3_dn13), (p.p1617 * locals.var_t3_dn14),)
    } else {
        (locals.var_jswd_t, locals.var_jswd_t_dn0, locals.var_jswd_t_dn2, locals.var_jswd_t_dn3, locals.var_jswd_t_dn4, locals.var_jswd_t_dn5, locals.var_jswd_t_dn6, locals.var_jswd_t_dn7, locals.var_jswd_t_dn8, locals.var_jswd_t_dn9, locals.var_jswd_t_dn10, locals.var_jswd_t_dn11, locals.var_jswd_t_dn13, locals.var_jswd_t_dn14,)
    }
};
        locals.var_jswd_t = assign18550_e34657;
        locals.var_jswd_t_dn0 = assign18550_e34657_d_n0;
        locals.var_jswd_t_dn2 = assign18550_e34657_d_n2;
        locals.var_jswd_t_dn3 = assign18550_e34657_d_n3;
        locals.var_jswd_t_dn4 = assign18550_e34657_d_n4;
        locals.var_jswd_t_dn5 = assign18550_e34657_d_n5;
        locals.var_jswd_t_dn6 = assign18550_e34657_d_n6;
        locals.var_jswd_t_dn7 = assign18550_e34657_d_n7;
        locals.var_jswd_t_dn8 = assign18550_e34657_d_n8;
        locals.var_jswd_t_dn9 = assign18550_e34657_d_n9;
        locals.var_jswd_t_dn10 = assign18550_e34657_d_n10;
        locals.var_jswd_t_dn11 = assign18550_e34657_d_n11;
        locals.var_jswd_t_dn13 = assign18550_e34657_d_n13;
        locals.var_jswd_t_dn14 = assign18550_e34657_d_n14;

        let (assign18560_e34663, assign18560_e34663_d_n0, assign18560_e34663_d_n2, assign18560_e34663_d_n3, assign18560_e34663_d_n4, assign18560_e34663_d_n5, assign18560_e34663_d_n6, assign18560_e34663_d_n7, assign18560_e34663_d_n8, assign18560_e34663_d_n9, assign18560_e34663_d_n10, assign18560_e34663_d_n11, assign18560_e34663_d_n13, assign18560_e34663_d_n14,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18560_e34661: f64 = (p.p1619 * locals.var_t3);
        (assign18560_e34661, (p.p1619 * locals.var_t3_dn0), (p.p1619 * locals.var_t3_dn2), (p.p1619 * locals.var_t3_dn3), (p.p1619 * locals.var_t3_dn4), (p.p1619 * locals.var_t3_dn5), (p.p1619 * locals.var_t3_dn6), (p.p1619 * locals.var_t3_dn7), (p.p1619 * locals.var_t3_dn8), (p.p1619 * locals.var_t3_dn9), (p.p1619 * locals.var_t3_dn10), (p.p1619 * locals.var_t3_dn11), (p.p1619 * locals.var_t3_dn13), (p.p1619 * locals.var_t3_dn14),)
    } else {
        (locals.var_jswgd_t, locals.var_jswgd_t_dn0, locals.var_jswgd_t_dn2, locals.var_jswgd_t_dn3, locals.var_jswgd_t_dn4, locals.var_jswgd_t_dn5, locals.var_jswgd_t_dn6, locals.var_jswgd_t_dn7, locals.var_jswgd_t_dn8, locals.var_jswgd_t_dn9, locals.var_jswgd_t_dn10, locals.var_jswgd_t_dn11, locals.var_jswgd_t_dn13, locals.var_jswgd_t_dn14,)
    }
};
        locals.var_jswgd_t = assign18560_e34663;
        locals.var_jswgd_t_dn0 = assign18560_e34663_d_n0;
        locals.var_jswgd_t_dn2 = assign18560_e34663_d_n2;
        locals.var_jswgd_t_dn3 = assign18560_e34663_d_n3;
        locals.var_jswgd_t_dn4 = assign18560_e34663_d_n4;
        locals.var_jswgd_t_dn5 = assign18560_e34663_d_n5;
        locals.var_jswgd_t_dn6 = assign18560_e34663_d_n6;
        locals.var_jswgd_t_dn7 = assign18560_e34663_d_n7;
        locals.var_jswgd_t_dn8 = assign18560_e34663_d_n8;
        locals.var_jswgd_t_dn9 = assign18560_e34663_d_n9;
        locals.var_jswgd_t_dn10 = assign18560_e34663_d_n10;
        locals.var_jswgd_t_dn11 = assign18560_e34663_d_n11;
        locals.var_jswgd_t_dn13 = assign18560_e34663_d_n13;
        locals.var_jswgd_t_dn14 = assign18560_e34663_d_n14;

        let (assign18570_e34676,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18570_e34668: f64 = (locals.var_eg0 * p.p1729);
        let assign18570_e34670: f64 = (assign18570_e34668 * locals.var_tratio_m1);
        let assign18570_e34672: f64 = (assign18570_e34670 / locals.var_vtm);
        let assign18570_e34673: f64 = { let limited_exp_arg = assign18570_e34672; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign18570_e34674: f64 = (p.p1630 * assign18570_e34673);
        (assign18570_e34674,)
    } else {
        (locals.var_jtss_t,)
    }
};
        locals.var_jtss_t = assign18570_e34676;

        let (assign18580_e34689,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18580_e34681: f64 = (locals.var_eg0 * p.p1730);
        let assign18580_e34683: f64 = (assign18580_e34681 * locals.var_tratio_m1);
        let assign18580_e34685: f64 = (assign18580_e34683 / locals.var_vtm);
        let assign18580_e34686: f64 = { let limited_exp_arg = assign18580_e34685; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign18580_e34687: f64 = (p.p1631 * assign18580_e34686);
        (assign18580_e34687,)
    } else {
        (locals.var_jtsd_t,)
    }
};
        locals.var_jtsd_t = assign18580_e34689;

        let (assign18590_e34702,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18590_e34694: f64 = (locals.var_eg0 * p.p1731);
        let assign18590_e34696: f64 = (assign18590_e34694 * locals.var_tratio_m1);
        let assign18590_e34698: f64 = (assign18590_e34696 / locals.var_vtm);
        let assign18590_e34699: f64 = { let limited_exp_arg = assign18590_e34698; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign18590_e34700: f64 = (p.p1632 * assign18590_e34699);
        (assign18590_e34700,)
    } else {
        (locals.var_jtssws_t,)
    }
};
        locals.var_jtssws_t = assign18590_e34702;

        let (assign18600_e34715,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18600_e34707: f64 = (locals.var_eg0 * p.p1732);
        let assign18600_e34709: f64 = (assign18600_e34707 * locals.var_tratio_m1);
        let assign18600_e34711: f64 = (assign18600_e34709 / locals.var_vtm);
        let assign18600_e34712: f64 = { let limited_exp_arg = assign18600_e34711; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign18600_e34713: f64 = (p.p1633 * assign18600_e34712);
        (assign18600_e34713,)
    } else {
        (locals.var_jtsswd_t,)
    }
};
        locals.var_jtsswd_t = assign18600_e34715;

        let (assign18610_e34735,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18610_e34720: f64 = (p.p1636 / locals.var_weff0);
        let assign18610_e34721: f64 = (assign18610_e34720).sqrt();
        let assign18610_e34723: f64 = (assign18610_e34721 + 1.0);
        let assign18610_e34724: f64 = (p.p1634 * assign18610_e34723);
        let assign18610_e34727: f64 = (locals.var_eg0 * p.p1733);
        let assign18610_e34729: f64 = (assign18610_e34727 * locals.var_tratio_m1);
        let assign18610_e34731: f64 = (assign18610_e34729 / locals.var_vtm);
        let assign18610_e34732: f64 = { let limited_exp_arg = assign18610_e34731; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign18610_e34733: f64 = (assign18610_e34724 * assign18610_e34732);
        (assign18610_e34733,)
    } else {
        (locals.var_jtsswgs_t,)
    }
};
        locals.var_jtsswgs_t = assign18610_e34735;

        let (assign18620_e34755,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18620_e34740: f64 = (p.p1636 / locals.var_weff0);
        let assign18620_e34741: f64 = (assign18620_e34740).sqrt();
        let assign18620_e34743: f64 = (assign18620_e34741 + 1.0);
        let assign18620_e34744: f64 = (p.p1635 * assign18620_e34743);
        let assign18620_e34747: f64 = (locals.var_eg0 * p.p1734);
        let assign18620_e34749: f64 = (assign18620_e34747 * locals.var_tratio_m1);
        let assign18620_e34751: f64 = (assign18620_e34749 / locals.var_vtm);
        let assign18620_e34752: f64 = { let limited_exp_arg = assign18620_e34751; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign18620_e34753: f64 = (assign18620_e34744 * assign18620_e34752);
        (assign18620_e34753,)
    } else {
        (locals.var_jtsswgd_t,)
    }
};
        locals.var_jtsswgd_t = assign18620_e34755;

        let (assign18630_e34844, assign18630_e34844_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18630_e34761: f64 = (p.p1735 * locals.var_tratio_m1);
        let assign18630_e34762: f64 = (1.0 + assign18630_e34761);
        let assign18630_e34763: f64 = (p.p1637 * assign18630_e34762);
        let assign18630_e34765: f64 = (assign18630_e34763 - 0.01);
        let assign18630_e34767: f64 = (-10000.0);
        let assign18630_e34769: f64 = (assign18630_e34767 * 0.001);
        let (assign18630_e34840, assign18630_e34840_d_n4,) = {
            if (!(assign18630_e34765 < assign18630_e34769)) {
                let assign18630_e34777: f64 = (p.p1735 * locals.var_tratio_m1);
                let assign18630_e34778: f64 = (1.0 + assign18630_e34777);
                let assign18630_e34779: f64 = (p.p1637 * assign18630_e34778);
                let assign18630_e34781: f64 = (assign18630_e34779 - 0.01);
                let assign18630_e34786: f64 = (p.p1735 * locals.var_tratio_m1);
                let assign18630_e34787: f64 = (1.0 + assign18630_e34786);
                let assign18630_e34788: f64 = (p.p1637 * assign18630_e34787);
                let assign18630_e34790: f64 = (assign18630_e34788 - 0.01);
                let assign18630_e34795: f64 = (p.p1735 * locals.var_tratio_m1);
                let assign18630_e34796: f64 = (1.0 + assign18630_e34795);
                let assign18630_e34797: f64 = (p.p1637 * assign18630_e34796);
                let assign18630_e34799: f64 = (assign18630_e34797 - 0.01);
                let assign18630_e34800: f64 = (assign18630_e34790 * assign18630_e34799);
                let assign18630_e34803: f64 = (4.0 * 0.001);
                let assign18630_e34805: f64 = (assign18630_e34803 * 0.001);
                let assign18630_e34806: f64 = (assign18630_e34800 + assign18630_e34805);
                let assign18630_e34807: f64 = (assign18630_e34806).sqrt();
                let assign18630_e34808: f64 = (assign18630_e34781 + assign18630_e34807);
                let assign18630_e34809: f64 = (0.5 * assign18630_e34808);
                (assign18630_e34809, (0.5 * ((p.p1637 * (p.p1735 * locals.var_tratio_m1_dn4)) + ((((p.p1637 * (p.p1735 * locals.var_tratio_m1_dn4)) * assign18630_e34799) + (assign18630_e34790 * (p.p1637 * (p.p1735 * locals.var_tratio_m1_dn4)))) / (2.0 * assign18630_e34807)))),)
            } else {
                let assign18630_e34814: f64 = (p.p1735 * locals.var_tratio_m1);
                let assign18630_e34815: f64 = (1.0 + assign18630_e34814);
                let assign18630_e34816: f64 = (p.p1637 * assign18630_e34815);
                let assign18630_e34818: f64 = (assign18630_e34816 - 0.01);
                let assign18630_e34820: f64 = (-10000.0);
                let assign18630_e34822: f64 = (assign18630_e34820 * 0.001);
                let (assign18630_e34839, assign18630_e34839_d_n4,) = {
                    if (assign18630_e34818 < assign18630_e34822) {
                        let assign18630_e34825: f64 = (-0.001);
                        let assign18630_e34827: f64 = (assign18630_e34825 * 0.001);
                        let assign18630_e34832: f64 = (p.p1735 * locals.var_tratio_m1);
                        let assign18630_e34833: f64 = (1.0 + assign18630_e34832);
                        let assign18630_e34834: f64 = (p.p1637 * assign18630_e34833);
                        let assign18630_e34836: f64 = (assign18630_e34834 - 0.01);
                        let assign18630_e34837: f64 = (assign18630_e34827 / assign18630_e34836);
                        (assign18630_e34837, (-((assign18630_e34827 * (p.p1637 * (p.p1735 * locals.var_tratio_m1_dn4))) / (assign18630_e34836 * assign18630_e34836))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18630_e34839, assign18630_e34839_d_n4,)
            }
        };
        let assign18630_e34842: f64 = (assign18630_e34840 + 0.01);
        (assign18630_e34842, assign18630_e34840_d_n4,)
    } else {
        (locals.var_njts_t, locals.var_njts_t_dn4,)
    }
};
        locals.var_njts_t = assign18630_e34844;
        locals.var_njts_t_dn4 = assign18630_e34844_d_n4;

    }

    pub(super) fn stamp_transient_block_63(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign18640_e34933, assign18640_e34933_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18640_e34850: f64 = (p.p1736 * locals.var_tratio_m1);
        let assign18640_e34851: f64 = (1.0 + assign18640_e34850);
        let assign18640_e34852: f64 = (p.p1638 * assign18640_e34851);
        let assign18640_e34854: f64 = (assign18640_e34852 - 0.01);
        let assign18640_e34856: f64 = (-10000.0);
        let assign18640_e34858: f64 = (assign18640_e34856 * 0.001);
        let (assign18640_e34929, assign18640_e34929_d_n4,) = {
            if (!(assign18640_e34854 < assign18640_e34858)) {
                let assign18640_e34866: f64 = (p.p1736 * locals.var_tratio_m1);
                let assign18640_e34867: f64 = (1.0 + assign18640_e34866);
                let assign18640_e34868: f64 = (p.p1638 * assign18640_e34867);
                let assign18640_e34870: f64 = (assign18640_e34868 - 0.01);
                let assign18640_e34875: f64 = (p.p1736 * locals.var_tratio_m1);
                let assign18640_e34876: f64 = (1.0 + assign18640_e34875);
                let assign18640_e34877: f64 = (p.p1638 * assign18640_e34876);
                let assign18640_e34879: f64 = (assign18640_e34877 - 0.01);
                let assign18640_e34884: f64 = (p.p1736 * locals.var_tratio_m1);
                let assign18640_e34885: f64 = (1.0 + assign18640_e34884);
                let assign18640_e34886: f64 = (p.p1638 * assign18640_e34885);
                let assign18640_e34888: f64 = (assign18640_e34886 - 0.01);
                let assign18640_e34889: f64 = (assign18640_e34879 * assign18640_e34888);
                let assign18640_e34892: f64 = (4.0 * 0.001);
                let assign18640_e34894: f64 = (assign18640_e34892 * 0.001);
                let assign18640_e34895: f64 = (assign18640_e34889 + assign18640_e34894);
                let assign18640_e34896: f64 = (assign18640_e34895).sqrt();
                let assign18640_e34897: f64 = (assign18640_e34870 + assign18640_e34896);
                let assign18640_e34898: f64 = (0.5 * assign18640_e34897);
                (assign18640_e34898, (0.5 * ((p.p1638 * (p.p1736 * locals.var_tratio_m1_dn4)) + ((((p.p1638 * (p.p1736 * locals.var_tratio_m1_dn4)) * assign18640_e34888) + (assign18640_e34879 * (p.p1638 * (p.p1736 * locals.var_tratio_m1_dn4)))) / (2.0 * assign18640_e34896)))),)
            } else {
                let assign18640_e34903: f64 = (p.p1736 * locals.var_tratio_m1);
                let assign18640_e34904: f64 = (1.0 + assign18640_e34903);
                let assign18640_e34905: f64 = (p.p1638 * assign18640_e34904);
                let assign18640_e34907: f64 = (assign18640_e34905 - 0.01);
                let assign18640_e34909: f64 = (-10000.0);
                let assign18640_e34911: f64 = (assign18640_e34909 * 0.001);
                let (assign18640_e34928, assign18640_e34928_d_n4,) = {
                    if (assign18640_e34907 < assign18640_e34911) {
                        let assign18640_e34914: f64 = (-0.001);
                        let assign18640_e34916: f64 = (assign18640_e34914 * 0.001);
                        let assign18640_e34921: f64 = (p.p1736 * locals.var_tratio_m1);
                        let assign18640_e34922: f64 = (1.0 + assign18640_e34921);
                        let assign18640_e34923: f64 = (p.p1638 * assign18640_e34922);
                        let assign18640_e34925: f64 = (assign18640_e34923 - 0.01);
                        let assign18640_e34926: f64 = (assign18640_e34916 / assign18640_e34925);
                        (assign18640_e34926, (-((assign18640_e34916 * (p.p1638 * (p.p1736 * locals.var_tratio_m1_dn4))) / (assign18640_e34925 * assign18640_e34925))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18640_e34928, assign18640_e34928_d_n4,)
            }
        };
        let assign18640_e34931: f64 = (assign18640_e34929 + 0.01);
        (assign18640_e34931, assign18640_e34929_d_n4,)
    } else {
        (locals.var_njtsd_t, locals.var_njtsd_t_dn4,)
    }
};
        locals.var_njtsd_t = assign18640_e34933;
        locals.var_njtsd_t_dn4 = assign18640_e34933_d_n4;

        let (assign18650_e35022, assign18650_e35022_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18650_e34939: f64 = (p.p1737 * locals.var_tratio_m1);
        let assign18650_e34940: f64 = (1.0 + assign18650_e34939);
        let assign18650_e34941: f64 = (p.p1639 * assign18650_e34940);
        let assign18650_e34943: f64 = (assign18650_e34941 - 0.01);
        let assign18650_e34945: f64 = (-10000.0);
        let assign18650_e34947: f64 = (assign18650_e34945 * 0.001);
        let (assign18650_e35018, assign18650_e35018_d_n4,) = {
            if (!(assign18650_e34943 < assign18650_e34947)) {
                let assign18650_e34955: f64 = (p.p1737 * locals.var_tratio_m1);
                let assign18650_e34956: f64 = (1.0 + assign18650_e34955);
                let assign18650_e34957: f64 = (p.p1639 * assign18650_e34956);
                let assign18650_e34959: f64 = (assign18650_e34957 - 0.01);
                let assign18650_e34964: f64 = (p.p1737 * locals.var_tratio_m1);
                let assign18650_e34965: f64 = (1.0 + assign18650_e34964);
                let assign18650_e34966: f64 = (p.p1639 * assign18650_e34965);
                let assign18650_e34968: f64 = (assign18650_e34966 - 0.01);
                let assign18650_e34973: f64 = (p.p1737 * locals.var_tratio_m1);
                let assign18650_e34974: f64 = (1.0 + assign18650_e34973);
                let assign18650_e34975: f64 = (p.p1639 * assign18650_e34974);
                let assign18650_e34977: f64 = (assign18650_e34975 - 0.01);
                let assign18650_e34978: f64 = (assign18650_e34968 * assign18650_e34977);
                let assign18650_e34981: f64 = (4.0 * 0.001);
                let assign18650_e34983: f64 = (assign18650_e34981 * 0.001);
                let assign18650_e34984: f64 = (assign18650_e34978 + assign18650_e34983);
                let assign18650_e34985: f64 = (assign18650_e34984).sqrt();
                let assign18650_e34986: f64 = (assign18650_e34959 + assign18650_e34985);
                let assign18650_e34987: f64 = (0.5 * assign18650_e34986);
                (assign18650_e34987, (0.5 * ((p.p1639 * (p.p1737 * locals.var_tratio_m1_dn4)) + ((((p.p1639 * (p.p1737 * locals.var_tratio_m1_dn4)) * assign18650_e34977) + (assign18650_e34968 * (p.p1639 * (p.p1737 * locals.var_tratio_m1_dn4)))) / (2.0 * assign18650_e34985)))),)
            } else {
                let assign18650_e34992: f64 = (p.p1737 * locals.var_tratio_m1);
                let assign18650_e34993: f64 = (1.0 + assign18650_e34992);
                let assign18650_e34994: f64 = (p.p1639 * assign18650_e34993);
                let assign18650_e34996: f64 = (assign18650_e34994 - 0.01);
                let assign18650_e34998: f64 = (-10000.0);
                let assign18650_e35000: f64 = (assign18650_e34998 * 0.001);
                let (assign18650_e35017, assign18650_e35017_d_n4,) = {
                    if (assign18650_e34996 < assign18650_e35000) {
                        let assign18650_e35003: f64 = (-0.001);
                        let assign18650_e35005: f64 = (assign18650_e35003 * 0.001);
                        let assign18650_e35010: f64 = (p.p1737 * locals.var_tratio_m1);
                        let assign18650_e35011: f64 = (1.0 + assign18650_e35010);
                        let assign18650_e35012: f64 = (p.p1639 * assign18650_e35011);
                        let assign18650_e35014: f64 = (assign18650_e35012 - 0.01);
                        let assign18650_e35015: f64 = (assign18650_e35005 / assign18650_e35014);
                        (assign18650_e35015, (-((assign18650_e35005 * (p.p1639 * (p.p1737 * locals.var_tratio_m1_dn4))) / (assign18650_e35014 * assign18650_e35014))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18650_e35017, assign18650_e35017_d_n4,)
            }
        };
        let assign18650_e35020: f64 = (assign18650_e35018 + 0.01);
        (assign18650_e35020, assign18650_e35018_d_n4,)
    } else {
        (locals.var_njtssw_t, locals.var_njtssw_t_dn4,)
    }
};
        locals.var_njtssw_t = assign18650_e35022;
        locals.var_njtssw_t_dn4 = assign18650_e35022_d_n4;

        let (assign18660_e35111, assign18660_e35111_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18660_e35028: f64 = (p.p1738 * locals.var_tratio_m1);
        let assign18660_e35029: f64 = (1.0 + assign18660_e35028);
        let assign18660_e35030: f64 = (p.p1640 * assign18660_e35029);
        let assign18660_e35032: f64 = (assign18660_e35030 - 0.01);
        let assign18660_e35034: f64 = (-10000.0);
        let assign18660_e35036: f64 = (assign18660_e35034 * 0.001);
        let (assign18660_e35107, assign18660_e35107_d_n4,) = {
            if (!(assign18660_e35032 < assign18660_e35036)) {
                let assign18660_e35044: f64 = (p.p1738 * locals.var_tratio_m1);
                let assign18660_e35045: f64 = (1.0 + assign18660_e35044);
                let assign18660_e35046: f64 = (p.p1640 * assign18660_e35045);
                let assign18660_e35048: f64 = (assign18660_e35046 - 0.01);
                let assign18660_e35053: f64 = (p.p1738 * locals.var_tratio_m1);
                let assign18660_e35054: f64 = (1.0 + assign18660_e35053);
                let assign18660_e35055: f64 = (p.p1640 * assign18660_e35054);
                let assign18660_e35057: f64 = (assign18660_e35055 - 0.01);
                let assign18660_e35062: f64 = (p.p1738 * locals.var_tratio_m1);
                let assign18660_e35063: f64 = (1.0 + assign18660_e35062);
                let assign18660_e35064: f64 = (p.p1640 * assign18660_e35063);
                let assign18660_e35066: f64 = (assign18660_e35064 - 0.01);
                let assign18660_e35067: f64 = (assign18660_e35057 * assign18660_e35066);
                let assign18660_e35070: f64 = (4.0 * 0.001);
                let assign18660_e35072: f64 = (assign18660_e35070 * 0.001);
                let assign18660_e35073: f64 = (assign18660_e35067 + assign18660_e35072);
                let assign18660_e35074: f64 = (assign18660_e35073).sqrt();
                let assign18660_e35075: f64 = (assign18660_e35048 + assign18660_e35074);
                let assign18660_e35076: f64 = (0.5 * assign18660_e35075);
                (assign18660_e35076, (0.5 * ((p.p1640 * (p.p1738 * locals.var_tratio_m1_dn4)) + ((((p.p1640 * (p.p1738 * locals.var_tratio_m1_dn4)) * assign18660_e35066) + (assign18660_e35057 * (p.p1640 * (p.p1738 * locals.var_tratio_m1_dn4)))) / (2.0 * assign18660_e35074)))),)
            } else {
                let assign18660_e35081: f64 = (p.p1738 * locals.var_tratio_m1);
                let assign18660_e35082: f64 = (1.0 + assign18660_e35081);
                let assign18660_e35083: f64 = (p.p1640 * assign18660_e35082);
                let assign18660_e35085: f64 = (assign18660_e35083 - 0.01);
                let assign18660_e35087: f64 = (-10000.0);
                let assign18660_e35089: f64 = (assign18660_e35087 * 0.001);
                let (assign18660_e35106, assign18660_e35106_d_n4,) = {
                    if (assign18660_e35085 < assign18660_e35089) {
                        let assign18660_e35092: f64 = (-0.001);
                        let assign18660_e35094: f64 = (assign18660_e35092 * 0.001);
                        let assign18660_e35099: f64 = (p.p1738 * locals.var_tratio_m1);
                        let assign18660_e35100: f64 = (1.0 + assign18660_e35099);
                        let assign18660_e35101: f64 = (p.p1640 * assign18660_e35100);
                        let assign18660_e35103: f64 = (assign18660_e35101 - 0.01);
                        let assign18660_e35104: f64 = (assign18660_e35094 / assign18660_e35103);
                        (assign18660_e35104, (-((assign18660_e35094 * (p.p1640 * (p.p1738 * locals.var_tratio_m1_dn4))) / (assign18660_e35103 * assign18660_e35103))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18660_e35106, assign18660_e35106_d_n4,)
            }
        };
        let assign18660_e35109: f64 = (assign18660_e35107 + 0.01);
        (assign18660_e35109, assign18660_e35107_d_n4,)
    } else {
        (locals.var_njtsswd_t, locals.var_njtsswd_t_dn4,)
    }
};
        locals.var_njtsswd_t = assign18660_e35111;
        locals.var_njtsswd_t_dn4 = assign18660_e35111_d_n4;

        let (assign18670_e35200, assign18670_e35200_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18670_e35117: f64 = (p.p1739 * locals.var_tratio_m1);
        let assign18670_e35118: f64 = (1.0 + assign18670_e35117);
        let assign18670_e35119: f64 = (p.p1641 * assign18670_e35118);
        let assign18670_e35121: f64 = (assign18670_e35119 - 0.01);
        let assign18670_e35123: f64 = (-10000.0);
        let assign18670_e35125: f64 = (assign18670_e35123 * 0.001);
        let (assign18670_e35196, assign18670_e35196_d_n4,) = {
            if (!(assign18670_e35121 < assign18670_e35125)) {
                let assign18670_e35133: f64 = (p.p1739 * locals.var_tratio_m1);
                let assign18670_e35134: f64 = (1.0 + assign18670_e35133);
                let assign18670_e35135: f64 = (p.p1641 * assign18670_e35134);
                let assign18670_e35137: f64 = (assign18670_e35135 - 0.01);
                let assign18670_e35142: f64 = (p.p1739 * locals.var_tratio_m1);
                let assign18670_e35143: f64 = (1.0 + assign18670_e35142);
                let assign18670_e35144: f64 = (p.p1641 * assign18670_e35143);
                let assign18670_e35146: f64 = (assign18670_e35144 - 0.01);
                let assign18670_e35151: f64 = (p.p1739 * locals.var_tratio_m1);
                let assign18670_e35152: f64 = (1.0 + assign18670_e35151);
                let assign18670_e35153: f64 = (p.p1641 * assign18670_e35152);
                let assign18670_e35155: f64 = (assign18670_e35153 - 0.01);
                let assign18670_e35156: f64 = (assign18670_e35146 * assign18670_e35155);
                let assign18670_e35159: f64 = (4.0 * 0.001);
                let assign18670_e35161: f64 = (assign18670_e35159 * 0.001);
                let assign18670_e35162: f64 = (assign18670_e35156 + assign18670_e35161);
                let assign18670_e35163: f64 = (assign18670_e35162).sqrt();
                let assign18670_e35164: f64 = (assign18670_e35137 + assign18670_e35163);
                let assign18670_e35165: f64 = (0.5 * assign18670_e35164);
                (assign18670_e35165, (0.5 * ((p.p1641 * (p.p1739 * locals.var_tratio_m1_dn4)) + ((((p.p1641 * (p.p1739 * locals.var_tratio_m1_dn4)) * assign18670_e35155) + (assign18670_e35146 * (p.p1641 * (p.p1739 * locals.var_tratio_m1_dn4)))) / (2.0 * assign18670_e35163)))),)
            } else {
                let assign18670_e35170: f64 = (p.p1739 * locals.var_tratio_m1);
                let assign18670_e35171: f64 = (1.0 + assign18670_e35170);
                let assign18670_e35172: f64 = (p.p1641 * assign18670_e35171);
                let assign18670_e35174: f64 = (assign18670_e35172 - 0.01);
                let assign18670_e35176: f64 = (-10000.0);
                let assign18670_e35178: f64 = (assign18670_e35176 * 0.001);
                let (assign18670_e35195, assign18670_e35195_d_n4,) = {
                    if (assign18670_e35174 < assign18670_e35178) {
                        let assign18670_e35181: f64 = (-0.001);
                        let assign18670_e35183: f64 = (assign18670_e35181 * 0.001);
                        let assign18670_e35188: f64 = (p.p1739 * locals.var_tratio_m1);
                        let assign18670_e35189: f64 = (1.0 + assign18670_e35188);
                        let assign18670_e35190: f64 = (p.p1641 * assign18670_e35189);
                        let assign18670_e35192: f64 = (assign18670_e35190 - 0.01);
                        let assign18670_e35193: f64 = (assign18670_e35183 / assign18670_e35192);
                        (assign18670_e35193, (-((assign18670_e35183 * (p.p1641 * (p.p1739 * locals.var_tratio_m1_dn4))) / (assign18670_e35192 * assign18670_e35192))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18670_e35195, assign18670_e35195_d_n4,)
            }
        };
        let assign18670_e35198: f64 = (assign18670_e35196 + 0.01);
        (assign18670_e35198, assign18670_e35196_d_n4,)
    } else {
        (locals.var_njtsswg_t, locals.var_njtsswg_t_dn4,)
    }
};
        locals.var_njtsswg_t = assign18670_e35200;
        locals.var_njtsswg_t_dn4 = assign18670_e35200_d_n4;

        let (assign18680_e35289, assign18680_e35289_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18680_e35206: f64 = (p.p1740 * locals.var_tratio_m1);
        let assign18680_e35207: f64 = (1.0 + assign18680_e35206);
        let assign18680_e35208: f64 = (p.p1642 * assign18680_e35207);
        let assign18680_e35210: f64 = (assign18680_e35208 - 0.01);
        let assign18680_e35212: f64 = (-10000.0);
        let assign18680_e35214: f64 = (assign18680_e35212 * 0.001);
        let (assign18680_e35285, assign18680_e35285_d_n4,) = {
            if (!(assign18680_e35210 < assign18680_e35214)) {
                let assign18680_e35222: f64 = (p.p1740 * locals.var_tratio_m1);
                let assign18680_e35223: f64 = (1.0 + assign18680_e35222);
                let assign18680_e35224: f64 = (p.p1642 * assign18680_e35223);
                let assign18680_e35226: f64 = (assign18680_e35224 - 0.01);
                let assign18680_e35231: f64 = (p.p1740 * locals.var_tratio_m1);
                let assign18680_e35232: f64 = (1.0 + assign18680_e35231);
                let assign18680_e35233: f64 = (p.p1642 * assign18680_e35232);
                let assign18680_e35235: f64 = (assign18680_e35233 - 0.01);
                let assign18680_e35240: f64 = (p.p1740 * locals.var_tratio_m1);
                let assign18680_e35241: f64 = (1.0 + assign18680_e35240);
                let assign18680_e35242: f64 = (p.p1642 * assign18680_e35241);
                let assign18680_e35244: f64 = (assign18680_e35242 - 0.01);
                let assign18680_e35245: f64 = (assign18680_e35235 * assign18680_e35244);
                let assign18680_e35248: f64 = (4.0 * 0.001);
                let assign18680_e35250: f64 = (assign18680_e35248 * 0.001);
                let assign18680_e35251: f64 = (assign18680_e35245 + assign18680_e35250);
                let assign18680_e35252: f64 = (assign18680_e35251).sqrt();
                let assign18680_e35253: f64 = (assign18680_e35226 + assign18680_e35252);
                let assign18680_e35254: f64 = (0.5 * assign18680_e35253);
                (assign18680_e35254, (0.5 * ((p.p1642 * (p.p1740 * locals.var_tratio_m1_dn4)) + ((((p.p1642 * (p.p1740 * locals.var_tratio_m1_dn4)) * assign18680_e35244) + (assign18680_e35235 * (p.p1642 * (p.p1740 * locals.var_tratio_m1_dn4)))) / (2.0 * assign18680_e35252)))),)
            } else {
                let assign18680_e35259: f64 = (p.p1740 * locals.var_tratio_m1);
                let assign18680_e35260: f64 = (1.0 + assign18680_e35259);
                let assign18680_e35261: f64 = (p.p1642 * assign18680_e35260);
                let assign18680_e35263: f64 = (assign18680_e35261 - 0.01);
                let assign18680_e35265: f64 = (-10000.0);
                let assign18680_e35267: f64 = (assign18680_e35265 * 0.001);
                let (assign18680_e35284, assign18680_e35284_d_n4,) = {
                    if (assign18680_e35263 < assign18680_e35267) {
                        let assign18680_e35270: f64 = (-0.001);
                        let assign18680_e35272: f64 = (assign18680_e35270 * 0.001);
                        let assign18680_e35277: f64 = (p.p1740 * locals.var_tratio_m1);
                        let assign18680_e35278: f64 = (1.0 + assign18680_e35277);
                        let assign18680_e35279: f64 = (p.p1642 * assign18680_e35278);
                        let assign18680_e35281: f64 = (assign18680_e35279 - 0.01);
                        let assign18680_e35282: f64 = (assign18680_e35272 / assign18680_e35281);
                        (assign18680_e35282, (-((assign18680_e35272 * (p.p1642 * (p.p1740 * locals.var_tratio_m1_dn4))) / (assign18680_e35281 * assign18680_e35281))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18680_e35284, assign18680_e35284_d_n4,)
            }
        };
        let assign18680_e35287: f64 = (assign18680_e35285 + 0.01);
        (assign18680_e35287, assign18680_e35285_d_n4,)
    } else {
        (locals.var_njtsswgd_t, locals.var_njtsswgd_t_dn4,)
    }
};
        locals.var_njtsswgd_t = assign18680_e35289;
        locals.var_njtsswgd_t_dn4 = assign18680_e35289_d_n4;

        let assign18690_e35292: f64 = if (!param_given[1106]) { 1.0 } else { 0.0 };
        locals.var_guard343 = assign18690_e35292;

        let assign18700_e35295: f64 = if p.p145 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard344 = assign18700_e35295;

        let assign18710_e35298: f64 = if p.p80 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard345 = assign18710_e35298;

    }
}
