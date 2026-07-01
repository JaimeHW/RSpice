#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign6470_loop_guard: usize = 0;
        while {
            let assign6470_cond_e6823: f64 = (locals.var_d_q).abs();
            let assign6470_cond_e6823_d_n0: f64 = if locals.var_d_q >= 0.0 { locals.var_d_q_dn0 } else { (-locals.var_d_q_dn0) };
            let assign6470_cond_e6823_d_n1: f64 = if locals.var_d_q >= 0.0 { locals.var_d_q_dn1 } else { (-locals.var_d_q_dn1) };
            let assign6470_cond_e6823_d_n3: f64 = if locals.var_d_q >= 0.0 { locals.var_d_q_dn3 } else { (-locals.var_d_q_dn3) };
            let assign6470_cond_e6823_d_n4: f64 = if locals.var_d_q >= 0.0 { locals.var_d_q_dn4 } else { (-locals.var_d_q_dn4) };
            let assign6470_cond_e6823_d_n5: f64 = if locals.var_d_q >= 0.0 { locals.var_d_q_dn5 } else { (-locals.var_d_q_dn5) };
            let assign6470_cond_e6823_d_n6: f64 = if locals.var_d_q >= 0.0 { locals.var_d_q_dn6 } else { (-locals.var_d_q_dn6) };
            let assign6470_cond_e6823_d_n7: f64 = if locals.var_d_q >= 0.0 { locals.var_d_q_dn7 } else { (-locals.var_d_q_dn7) };
            let assign6470_cond_e6823_d_n8: f64 = if locals.var_d_q >= 0.0 { locals.var_d_q_dn8 } else { (-locals.var_d_q_dn8) };
            let assign6470_cond_e6823_d_n9: f64 = if locals.var_d_q >= 0.0 { locals.var_d_q_dn9 } else { (-locals.var_d_q_dn9) };
            let assign6470_cond_e6826: f64 = 1e-5;
            let assign6470_cond_e6828: f64 = (locals.var_q_pt).abs();
            let assign6470_cond_e6828_d_n0: f64 = if locals.var_q_pt >= 0.0 { locals.var_q_pt_dn0 } else { (-locals.var_q_pt_dn0) };
            let assign6470_cond_e6828_d_n1: f64 = if locals.var_q_pt >= 0.0 { locals.var_q_pt_dn1 } else { (-locals.var_q_pt_dn1) };
            let assign6470_cond_e6828_d_n3: f64 = if locals.var_q_pt >= 0.0 { locals.var_q_pt_dn3 } else { (-locals.var_q_pt_dn3) };
            let assign6470_cond_e6828_d_n4: f64 = if locals.var_q_pt >= 0.0 { locals.var_q_pt_dn4 } else { (-locals.var_q_pt_dn4) };
            let assign6470_cond_e6828_d_n5: f64 = if locals.var_q_pt >= 0.0 { locals.var_q_pt_dn5 } else { (-locals.var_q_pt_dn5) };
            let assign6470_cond_e6828_d_n6: f64 = if locals.var_q_pt >= 0.0 { locals.var_q_pt_dn6 } else { (-locals.var_q_pt_dn6) };
            let assign6470_cond_e6828_d_n7: f64 = if locals.var_q_pt >= 0.0 { locals.var_q_pt_dn7 } else { (-locals.var_q_pt_dn7) };
            let assign6470_cond_e6828_d_n8: f64 = if locals.var_q_pt >= 0.0 { locals.var_q_pt_dn8 } else { (-locals.var_q_pt_dn8) };
            let assign6470_cond_e6828_d_n9: f64 = if locals.var_q_pt >= 0.0 { locals.var_q_pt_dn9 } else { (-locals.var_q_pt_dn9) };
            let assign6470_cond_e6829: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828);
            let assign6470_cond_e6829_d_n0: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n0);
            let assign6470_cond_e6829_d_n1: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n1);
            let assign6470_cond_e6829_d_n3: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n3);
            let assign6470_cond_e6829_d_n4: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n4);
            let assign6470_cond_e6829_d_n5: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n5);
            let assign6470_cond_e6829_d_n6: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n6);
            let assign6470_cond_e6829_d_n7: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n7);
            let assign6470_cond_e6829_d_n8: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n8);
            let assign6470_cond_e6829_d_n9: f64 = (assign6470_cond_e6826 * assign6470_cond_e6828_d_n9);
            let assign6470_cond_e6835: f64 = if ((locals.var_guard131 != 0.0) && ((assign6470_cond_e6823 >= assign6470_cond_e6829) && (locals.var_l_it <= 100.0))) { 1.0 } else { 0.0 };
            assign6470_cond_e6835 != 0.0
        } {
            assign6470_loop_guard += 1;
            assert!(assign6470_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign6470_body0_e6841, assign6470_body0_e6841_d_n0, assign6470_body0_e6841_d_n1, assign6470_body0_e6841_d_n3, assign6470_body0_e6841_d_n4, assign6470_body0_e6841_d_n5, assign6470_body0_e6841_d_n6, assign6470_body0_e6841_d_n7, assign6470_body0_e6841_d_n8, assign6470_body0_e6841_d_n9,) = {
    if (locals.var_guard131 != 0.0) {
        let assign6470_body0_e6839: f64 = (locals.var_i_0f / locals.var_q_pt);
        (assign6470_body0_e6839, (-((locals.var_i_0f * locals.var_q_pt_dn0) / (locals.var_q_pt * locals.var_q_pt))), (-((locals.var_i_0f * locals.var_q_pt_dn1) / (locals.var_q_pt * locals.var_q_pt))), (-((locals.var_i_0f * locals.var_q_pt_dn3) / (locals.var_q_pt * locals.var_q_pt))), (((locals.var_i_0f_dn4 * locals.var_q_pt) - (locals.var_i_0f * locals.var_q_pt_dn4)) / (locals.var_q_pt * locals.var_q_pt)), (-((locals.var_i_0f * locals.var_q_pt_dn5) / (locals.var_q_pt * locals.var_q_pt))), (((locals.var_i_0f_dn6 * locals.var_q_pt) - (locals.var_i_0f * locals.var_q_pt_dn6)) / (locals.var_q_pt * locals.var_q_pt)), (-((locals.var_i_0f * locals.var_q_pt_dn7) / (locals.var_q_pt * locals.var_q_pt))), (((locals.var_i_0f_dn8 * locals.var_q_pt) - (locals.var_i_0f * locals.var_q_pt_dn8)) / (locals.var_q_pt * locals.var_q_pt)), (-((locals.var_i_0f * locals.var_q_pt_dn9) / (locals.var_q_pt * locals.var_q_pt))),)
    } else {
        (locals.var_itf, locals.var_itf_dn0, locals.var_itf_dn1, locals.var_itf_dn3, locals.var_itf_dn4, locals.var_itf_dn5, locals.var_itf_dn6, locals.var_itf_dn7, locals.var_itf_dn8, locals.var_itf_dn9,)
    }
};
            locals.var_itf = assign6470_body0_e6841;
            locals.var_itf_dn0 = assign6470_body0_e6841_d_n0;
            locals.var_itf_dn1 = assign6470_body0_e6841_d_n1;
            locals.var_itf_dn3 = assign6470_body0_e6841_d_n3;
            locals.var_itf_dn4 = assign6470_body0_e6841_d_n4;
            locals.var_itf_dn5 = assign6470_body0_e6841_d_n5;
            locals.var_itf_dn6 = assign6470_body0_e6841_d_n6;
            locals.var_itf_dn7 = assign6470_body0_e6841_d_n7;
            locals.var_itf_dn8 = assign6470_body0_e6841_d_n8;
            locals.var_itf_dn9 = assign6470_body0_e6841_d_n9;
            locals.var_itf_rv = 0.0;
            let (assign6470_body1_e6847, assign6470_body1_e6847_d_n0, assign6470_body1_e6847_d_n1, assign6470_body1_e6847_d_n3, assign6470_body1_e6847_d_n4, assign6470_body1_e6847_d_n5, assign6470_body1_e6847_d_n6, assign6470_body1_e6847_d_n7, assign6470_body1_e6847_d_n8, assign6470_body1_e6847_d_n9,) = {
    if (locals.var_guard131 != 0.0) {
        let assign6470_body1_e6845: f64 = (locals.var_i_0r / locals.var_q_pt);
        (assign6470_body1_e6845, (-((locals.var_i_0r * locals.var_q_pt_dn0) / (locals.var_q_pt * locals.var_q_pt))), (-((locals.var_i_0r * locals.var_q_pt_dn1) / (locals.var_q_pt * locals.var_q_pt))), (-((locals.var_i_0r * locals.var_q_pt_dn3) / (locals.var_q_pt * locals.var_q_pt))), (((locals.var_i_0r_dn4 * locals.var_q_pt) - (locals.var_i_0r * locals.var_q_pt_dn4)) / (locals.var_q_pt * locals.var_q_pt)), (((locals.var_i_0r_dn5 * locals.var_q_pt) - (locals.var_i_0r * locals.var_q_pt_dn5)) / (locals.var_q_pt * locals.var_q_pt)), (-((locals.var_i_0r * locals.var_q_pt_dn6) / (locals.var_q_pt * locals.var_q_pt))), (-((locals.var_i_0r * locals.var_q_pt_dn7) / (locals.var_q_pt * locals.var_q_pt))), (((locals.var_i_0r_dn8 * locals.var_q_pt) - (locals.var_i_0r * locals.var_q_pt_dn8)) / (locals.var_q_pt * locals.var_q_pt)), (-((locals.var_i_0r * locals.var_q_pt_dn9) / (locals.var_q_pt * locals.var_q_pt))),)
    } else {
        (locals.var_itr, locals.var_itr_dn0, locals.var_itr_dn1, locals.var_itr_dn3, locals.var_itr_dn4, locals.var_itr_dn5, locals.var_itr_dn6, locals.var_itr_dn7, locals.var_itr_dn8, locals.var_itr_dn9,)
    }
};
            locals.var_itr = assign6470_body1_e6847;
            locals.var_itr_dn0 = assign6470_body1_e6847_d_n0;
            locals.var_itr_dn1 = assign6470_body1_e6847_d_n1;
            locals.var_itr_dn3 = assign6470_body1_e6847_d_n3;
            locals.var_itr_dn4 = assign6470_body1_e6847_d_n4;
            locals.var_itr_dn5 = assign6470_body1_e6847_d_n5;
            locals.var_itr_dn6 = assign6470_body1_e6847_d_n6;
            locals.var_itr_dn7 = assign6470_body1_e6847_d_n7;
            locals.var_itr_dn8 = assign6470_body1_e6847_d_n8;
            locals.var_itr_dn9 = assign6470_body1_e6847_d_n9;
            locals.var_itr_rv = 0.0;
            let (assign6470_body2_e6851, assign6470_body2_e6851_d_n0, assign6470_body2_e6851_d_n1, assign6470_body2_e6851_d_n3, assign6470_body2_e6851_d_n4, assign6470_body2_e6851_d_n5, assign6470_body2_e6851_d_n6, assign6470_body2_e6851_d_n7, assign6470_body2_e6851_d_n8, assign6470_body2_e6851_d_n9,) = {
    if (locals.var_guard131 != 0.0) {
        (locals.var_t_f0, 0.0, 0.0, 0.0, locals.var_t_f0_dn4, locals.var_t_f0_dn5, 0.0, 0.0, locals.var_t_f0_dn8, 0.0,)
    } else {
        (locals.var_tf, locals.var_tf_dn0, locals.var_tf_dn1, locals.var_tf_dn3, locals.var_tf_dn4, locals.var_tf_dn5, locals.var_tf_dn6, locals.var_tf_dn7, locals.var_tf_dn8, locals.var_tf_dn9,)
    }
};
            locals.var_tf = assign6470_body2_e6851;
            locals.var_tf_dn0 = assign6470_body2_e6851_d_n0;
            locals.var_tf_dn1 = assign6470_body2_e6851_d_n1;
            locals.var_tf_dn3 = assign6470_body2_e6851_d_n3;
            locals.var_tf_dn4 = assign6470_body2_e6851_d_n4;
            locals.var_tf_dn5 = assign6470_body2_e6851_d_n5;
            locals.var_tf_dn6 = assign6470_body2_e6851_d_n6;
            locals.var_tf_dn7 = assign6470_body2_e6851_d_n7;
            locals.var_tf_dn8 = assign6470_body2_e6851_d_n8;
            locals.var_tf_dn9 = assign6470_body2_e6851_d_n9;
            locals.var_tf_rv = 0.0;
            let (assign6470_body3_e6857, assign6470_body3_e6857_d_n0, assign6470_body3_e6857_d_n1, assign6470_body3_e6857_d_n3, assign6470_body3_e6857_d_n4, assign6470_body3_e6857_d_n5, assign6470_body3_e6857_d_n6, assign6470_body3_e6857_d_n7, assign6470_body3_e6857_d_n8, assign6470_body3_e6857_d_n9,) = {
    if (locals.var_guard131 != 0.0) {
        let assign6470_body3_e6855: f64 = (locals.var_t_f0 * locals.var_itf);
        (assign6470_body3_e6855, (locals.var_t_f0 * locals.var_itf_dn0), (locals.var_t_f0 * locals.var_itf_dn1), (locals.var_t_f0 * locals.var_itf_dn3), ((locals.var_t_f0_dn4 * locals.var_itf) + (locals.var_t_f0 * locals.var_itf_dn4)), ((locals.var_t_f0_dn5 * locals.var_itf) + (locals.var_t_f0 * locals.var_itf_dn5)), (locals.var_t_f0 * locals.var_itf_dn6), (locals.var_t_f0 * locals.var_itf_dn7), ((locals.var_t_f0_dn8 * locals.var_itf) + (locals.var_t_f0 * locals.var_itf_dn8)), (locals.var_t_f0 * locals.var_itf_dn9),)
    } else {
        (locals.var_qf, locals.var_qf_dn0, locals.var_qf_dn1, locals.var_qf_dn3, locals.var_qf_dn4, locals.var_qf_dn5, locals.var_qf_dn6, locals.var_qf_dn7, locals.var_qf_dn8, locals.var_qf_dn9,)
    }
};
            locals.var_qf = assign6470_body3_e6857;
            locals.var_qf_dn0 = assign6470_body3_e6857_d_n0;
            locals.var_qf_dn1 = assign6470_body3_e6857_d_n1;
            locals.var_qf_dn3 = assign6470_body3_e6857_d_n3;
            locals.var_qf_dn4 = assign6470_body3_e6857_d_n4;
            locals.var_qf_dn5 = assign6470_body3_e6857_d_n5;
            locals.var_qf_dn6 = assign6470_body3_e6857_d_n6;
            locals.var_qf_dn7 = assign6470_body3_e6857_d_n7;
            locals.var_qf_dn8 = assign6470_body3_e6857_d_n8;
            locals.var_qf_dn9 = assign6470_body3_e6857_d_n9;
            locals.var_qf_rv = 0.0;
            let assign6470_body4_e6860: f64 = if p.p0 >= 310.0 { 1.0 } else { 0.0 };
            locals.var_guard133 = assign6470_body4_e6860;
            locals.var_guard133_rv = 0.0;
            let (assign6470_body5_e6868, assign6470_body5_e6868_d_n0, assign6470_body5_e6868_d_n1, assign6470_body5_e6868_d_n3, assign6470_body5_e6868_d_n4, assign6470_body5_e6868_d_n5, assign6470_body5_e6868_d_n6, assign6470_body5_e6868_d_n7, assign6470_body5_e6868_d_n8, assign6470_body5_e6868_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard133 != 0.0)) {
        let assign6470_body5_e6866: f64 = (locals.var_hf0_t * locals.var_t0_t);
        (assign6470_body5_e6866, 0.0, 0.0, 0.0, ((locals.var_hf0_t_dn4 * locals.var_t0_t) + (locals.var_hf0_t * locals.var_t0_t_dn4)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t_ft, locals.var_t_ft_dn0, locals.var_t_ft_dn1, locals.var_t_ft_dn3, locals.var_t_ft_dn4, locals.var_t_ft_dn5, locals.var_t_ft_dn6, locals.var_t_ft_dn7, locals.var_t_ft_dn8, locals.var_t_ft_dn9,)
    }
};
            locals.var_t_ft = assign6470_body5_e6868;
            locals.var_t_ft_dn0 = assign6470_body5_e6868_d_n0;
            locals.var_t_ft_dn1 = assign6470_body5_e6868_d_n1;
            locals.var_t_ft_dn3 = assign6470_body5_e6868_d_n3;
            locals.var_t_ft_dn4 = assign6470_body5_e6868_d_n4;
            locals.var_t_ft_dn5 = assign6470_body5_e6868_d_n5;
            locals.var_t_ft_dn6 = assign6470_body5_e6868_d_n6;
            locals.var_t_ft_dn7 = assign6470_body5_e6868_d_n7;
            locals.var_t_ft_dn8 = assign6470_body5_e6868_d_n8;
            locals.var_t_ft_dn9 = assign6470_body5_e6868_d_n9;
            locals.var_t_ft_rv = 0.0;
            let (assign6470_body6_e6876, assign6470_body6_e6876_d_n0, assign6470_body6_e6876_d_n1, assign6470_body6_e6876_d_n3, assign6470_body6_e6876_d_n4, assign6470_body6_e6876_d_n5, assign6470_body6_e6876_d_n6, assign6470_body6_e6876_d_n7, assign6470_body6_e6876_d_n8, assign6470_body6_e6876_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard133 != 0.0)) {
        let assign6470_body6_e6874: f64 = (locals.var_t_ft * locals.var_itf);
        (assign6470_body6_e6874, ((locals.var_t_ft_dn0 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn0)), ((locals.var_t_ft_dn1 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn1)), ((locals.var_t_ft_dn3 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn3)), ((locals.var_t_ft_dn4 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn4)), ((locals.var_t_ft_dn5 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn5)), ((locals.var_t_ft_dn6 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn6)), ((locals.var_t_ft_dn7 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn7)), ((locals.var_t_ft_dn8 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn8)), ((locals.var_t_ft_dn9 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn9)),)
    } else {
        (locals.var_q_ft, locals.var_q_ft_dn0, locals.var_q_ft_dn1, locals.var_q_ft_dn3, locals.var_q_ft_dn4, locals.var_q_ft_dn5, locals.var_q_ft_dn6, locals.var_q_ft_dn7, locals.var_q_ft_dn8, locals.var_q_ft_dn9,)
    }
};
            locals.var_q_ft = assign6470_body6_e6876;
            locals.var_q_ft_dn0 = assign6470_body6_e6876_d_n0;
            locals.var_q_ft_dn1 = assign6470_body6_e6876_d_n1;
            locals.var_q_ft_dn3 = assign6470_body6_e6876_d_n3;
            locals.var_q_ft_dn4 = assign6470_body6_e6876_d_n4;
            locals.var_q_ft_dn5 = assign6470_body6_e6876_d_n5;
            locals.var_q_ft_dn6 = assign6470_body6_e6876_d_n6;
            locals.var_q_ft_dn7 = assign6470_body6_e6876_d_n7;
            locals.var_q_ft_dn8 = assign6470_body6_e6876_d_n8;
            locals.var_q_ft_dn9 = assign6470_body6_e6876_d_n9;
            locals.var_q_ft_rv = 0.0;
            let (assign6470_body7_e6885, assign6470_body7_e6885_d_n0, assign6470_body7_e6885_d_n1, assign6470_body7_e6885_d_n3, assign6470_body7_e6885_d_n4, assign6470_body7_e6885_d_n5, assign6470_body7_e6885_d_n6, assign6470_body7_e6885_d_n7, assign6470_body7_e6885_d_n8, assign6470_body7_e6885_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard133 == 0.0)) {
        let assign6470_body7_e6883: f64 = (locals.var_hf0_t * locals.var_qf);
        (assign6470_body7_e6883, (locals.var_hf0_t * locals.var_qf_dn0), (locals.var_hf0_t * locals.var_qf_dn1), (locals.var_hf0_t * locals.var_qf_dn3), ((locals.var_hf0_t_dn4 * locals.var_qf) + (locals.var_hf0_t * locals.var_qf_dn4)), (locals.var_hf0_t * locals.var_qf_dn5), (locals.var_hf0_t * locals.var_qf_dn6), (locals.var_hf0_t * locals.var_qf_dn7), (locals.var_hf0_t * locals.var_qf_dn8), (locals.var_hf0_t * locals.var_qf_dn9),)
    } else {
        (locals.var_q_ft, locals.var_q_ft_dn0, locals.var_q_ft_dn1, locals.var_q_ft_dn3, locals.var_q_ft_dn4, locals.var_q_ft_dn5, locals.var_q_ft_dn6, locals.var_q_ft_dn7, locals.var_q_ft_dn8, locals.var_q_ft_dn9,)
    }
};
            locals.var_q_ft = assign6470_body7_e6885;
            locals.var_q_ft_dn0 = assign6470_body7_e6885_d_n0;
            locals.var_q_ft_dn1 = assign6470_body7_e6885_d_n1;
            locals.var_q_ft_dn3 = assign6470_body7_e6885_d_n3;
            locals.var_q_ft_dn4 = assign6470_body7_e6885_d_n4;
            locals.var_q_ft_dn5 = assign6470_body7_e6885_d_n5;
            locals.var_q_ft_dn6 = assign6470_body7_e6885_d_n6;
            locals.var_q_ft_dn7 = assign6470_body7_e6885_d_n7;
            locals.var_q_ft_dn8 = assign6470_body7_e6885_d_n8;
            locals.var_q_ft_dn9 = assign6470_body7_e6885_d_n9;
            locals.var_q_ft_rv = 0.0;
            let (assign6470_body8_e6894, assign6470_body8_e6894_d_n0, assign6470_body8_e6894_d_n1, assign6470_body8_e6894_d_n3, assign6470_body8_e6894_d_n4, assign6470_body8_e6894_d_n5, assign6470_body8_e6894_d_n6, assign6470_body8_e6894_d_n7, assign6470_body8_e6894_d_n8, assign6470_body8_e6894_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard133 == 0.0)) {
        let assign6470_body8_e6892: f64 = (locals.var_hf0_t * locals.var_tf);
        (assign6470_body8_e6892, (locals.var_hf0_t * locals.var_tf_dn0), (locals.var_hf0_t * locals.var_tf_dn1), (locals.var_hf0_t * locals.var_tf_dn3), ((locals.var_hf0_t_dn4 * locals.var_tf) + (locals.var_hf0_t * locals.var_tf_dn4)), (locals.var_hf0_t * locals.var_tf_dn5), (locals.var_hf0_t * locals.var_tf_dn6), (locals.var_hf0_t * locals.var_tf_dn7), (locals.var_hf0_t * locals.var_tf_dn8), (locals.var_hf0_t * locals.var_tf_dn9),)
    } else {
        (locals.var_t_ft, locals.var_t_ft_dn0, locals.var_t_ft_dn1, locals.var_t_ft_dn3, locals.var_t_ft_dn4, locals.var_t_ft_dn5, locals.var_t_ft_dn6, locals.var_t_ft_dn7, locals.var_t_ft_dn8, locals.var_t_ft_dn9,)
    }
};
            locals.var_t_ft = assign6470_body8_e6894;
            locals.var_t_ft_dn0 = assign6470_body8_e6894_d_n0;
            locals.var_t_ft_dn1 = assign6470_body8_e6894_d_n1;
            locals.var_t_ft_dn3 = assign6470_body8_e6894_d_n3;
            locals.var_t_ft_dn4 = assign6470_body8_e6894_d_n4;
            locals.var_t_ft_dn5 = assign6470_body8_e6894_d_n5;
            locals.var_t_ft_dn6 = assign6470_body8_e6894_d_n6;
            locals.var_t_ft_dn7 = assign6470_body8_e6894_d_n7;
            locals.var_t_ft_dn8 = assign6470_body8_e6894_d_n8;
            locals.var_t_ft_dn9 = assign6470_body8_e6894_d_n9;
            locals.var_t_ft_rv = 0.0;
            let (assign6470_body9_e6898, assign6470_body9_e6898_d_n0, assign6470_body9_e6898_d_n1, assign6470_body9_e6898_d_n3, assign6470_body9_e6898_d_n4, assign6470_body9_e6898_d_n5, assign6470_body9_e6898_d_n6, assign6470_body9_e6898_d_n7, assign6470_body9_e6898_d_n8, assign6470_body9_e6898_d_n9,) = {
    if (locals.var_guard131 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_bf, locals.var_q_bf_dn0, locals.var_q_bf_dn1, locals.var_q_bf_dn3, locals.var_q_bf_dn4, locals.var_q_bf_dn5, locals.var_q_bf_dn6, locals.var_q_bf_dn7, locals.var_q_bf_dn8, locals.var_q_bf_dn9,)
    }
};
            locals.var_q_bf = assign6470_body9_e6898;
            locals.var_q_bf_dn0 = assign6470_body9_e6898_d_n0;
            locals.var_q_bf_dn1 = assign6470_body9_e6898_d_n1;
            locals.var_q_bf_dn3 = assign6470_body9_e6898_d_n3;
            locals.var_q_bf_dn4 = assign6470_body9_e6898_d_n4;
            locals.var_q_bf_dn5 = assign6470_body9_e6898_d_n5;
            locals.var_q_bf_dn6 = assign6470_body9_e6898_d_n6;
            locals.var_q_bf_dn7 = assign6470_body9_e6898_d_n7;
            locals.var_q_bf_dn8 = assign6470_body9_e6898_d_n8;
            locals.var_q_bf_dn9 = assign6470_body9_e6898_d_n9;
            locals.var_q_bf_rv = 0.0;
            let assign6470_body10_e6902: f64 = (1e-6 * locals.var_ick);
            let assign6470_body10_e6907: f64 = if ((locals.var_itf >= assign6470_body10_e6902) || (p.p0 >= 320.0)) { 1.0 } else { 0.0 };
            locals.var_guard134 = assign6470_body10_e6907;
            locals.var_guard134_rv = 0.0;
            let (assign6470_body11_e6915, assign6470_body11_e6915_d_n0, assign6470_body11_e6915_d_n1, assign6470_body11_e6915_d_n3, assign6470_body11_e6915_d_n4, assign6470_body11_e6915_d_n5, assign6470_body11_e6915_d_n6, assign6470_body11_e6915_d_n7, assign6470_body11_e6915_d_n8, assign6470_body11_e6915_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign6470_body11_e6913: f64 = (locals.var_itf / locals.var_ick);
        (assign6470_body11_e6913, (locals.var_itf_dn0 / locals.var_ick), (locals.var_itf_dn1 / locals.var_ick), (locals.var_itf_dn3 / locals.var_ick), (((locals.var_itf_dn4 * locals.var_ick) - (locals.var_itf * locals.var_ick_dn4)) / (locals.var_ick * locals.var_ick)), (((locals.var_itf_dn5 * locals.var_ick) - (locals.var_itf * locals.var_ick_dn5)) / (locals.var_ick * locals.var_ick)), (((locals.var_itf_dn6 * locals.var_ick) - (locals.var_itf * locals.var_ick_dn6)) / (locals.var_ick * locals.var_ick)), (locals.var_itf_dn7 / locals.var_ick), (((locals.var_itf_dn8 * locals.var_ick) - (locals.var_itf * locals.var_ick_dn8)) / (locals.var_ick * locals.var_ick)), (locals.var_itf_dn9 / locals.var_ick),)
    } else {
        (locals.var_ffitf_ick, locals.var_ffitf_ick_dn0, locals.var_ffitf_ick_dn1, locals.var_ffitf_ick_dn3, locals.var_ffitf_ick_dn4, locals.var_ffitf_ick_dn5, locals.var_ffitf_ick_dn6, locals.var_ffitf_ick_dn7, locals.var_ffitf_ick_dn8, locals.var_ffitf_ick_dn9,)
    }
};
            locals.var_ffitf_ick = assign6470_body11_e6915;
            locals.var_ffitf_ick_dn0 = assign6470_body11_e6915_d_n0;
            locals.var_ffitf_ick_dn1 = assign6470_body11_e6915_d_n1;
            locals.var_ffitf_ick_dn3 = assign6470_body11_e6915_d_n3;
            locals.var_ffitf_ick_dn4 = assign6470_body11_e6915_d_n4;
            locals.var_ffitf_ick_dn5 = assign6470_body11_e6915_d_n5;
            locals.var_ffitf_ick_dn6 = assign6470_body11_e6915_d_n6;
            locals.var_ffitf_ick_dn7 = assign6470_body11_e6915_d_n7;
            locals.var_ffitf_ick_dn8 = assign6470_body11_e6915_d_n8;
            locals.var_ffitf_ick_dn9 = assign6470_body11_e6915_d_n9;
            locals.var_ffitf_ick_rv = 0.0;
            let (assign6470_body12_e6927, assign6470_body12_e6927_d_n0, assign6470_body12_e6927_d_n1, assign6470_body12_e6927_d_n3, assign6470_body12_e6927_d_n4, assign6470_body12_e6927_d_n5, assign6470_body12_e6927_d_n6, assign6470_body12_e6927_d_n7, assign6470_body12_e6927_d_n8, assign6470_body12_e6927_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign6470_body12_e6922: f64 = (locals.var_ffitf_ick).ln();
        let assign6470_body12_e6923: f64 = (p.p70 * assign6470_body12_e6922);
        let assign6470_body12_e6924: f64 = (assign6470_body12_e6923).exp();
        let assign6470_body12_e6925: f64 = (locals.var_tef0_t * assign6470_body12_e6924);
        (assign6470_body12_e6925, (locals.var_tef0_t * (assign6470_body12_e6924 * (p.p70 * (locals.var_ffitf_ick_dn0 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign6470_body12_e6924 * (p.p70 * (locals.var_ffitf_ick_dn1 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign6470_body12_e6924 * (p.p70 * (locals.var_ffitf_ick_dn3 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign6470_body12_e6924 * (p.p70 * (locals.var_ffitf_ick_dn4 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign6470_body12_e6924 * (p.p70 * (locals.var_ffitf_ick_dn5 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign6470_body12_e6924 * (p.p70 * (locals.var_ffitf_ick_dn6 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign6470_body12_e6924 * (p.p70 * (locals.var_ffitf_ick_dn7 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign6470_body12_e6924 * (p.p70 * (locals.var_ffitf_ick_dn8 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign6470_body12_e6924 * (p.p70 * (locals.var_ffitf_ick_dn9 / locals.var_ffitf_ick)))),)
    } else {
        (locals.var_ffdtef, locals.var_ffdtef_dn0, locals.var_ffdtef_dn1, locals.var_ffdtef_dn3, locals.var_ffdtef_dn4, locals.var_ffdtef_dn5, locals.var_ffdtef_dn6, locals.var_ffdtef_dn7, locals.var_ffdtef_dn8, locals.var_ffdtef_dn9,)
    }
};
            locals.var_ffdtef = assign6470_body12_e6927;
            locals.var_ffdtef_dn0 = assign6470_body12_e6927_d_n0;
            locals.var_ffdtef_dn1 = assign6470_body12_e6927_d_n1;
            locals.var_ffdtef_dn3 = assign6470_body12_e6927_d_n3;
            locals.var_ffdtef_dn4 = assign6470_body12_e6927_d_n4;
            locals.var_ffdtef_dn5 = assign6470_body12_e6927_d_n5;
            locals.var_ffdtef_dn6 = assign6470_body12_e6927_d_n6;
            locals.var_ffdtef_dn7 = assign6470_body12_e6927_d_n7;
            locals.var_ffdtef_dn8 = assign6470_body12_e6927_d_n8;
            locals.var_ffdtef_dn9 = assign6470_body12_e6927_d_n9;
            locals.var_ffdtef_rv = 0.0;
            let (assign6470_body13_e6939, assign6470_body13_e6939_d_n0, assign6470_body13_e6939_d_n1, assign6470_body13_e6939_d_n3, assign6470_body13_e6939_d_n4, assign6470_body13_e6939_d_n5, assign6470_body13_e6939_d_n6, assign6470_body13_e6939_d_n7, assign6470_body13_e6939_d_n8, assign6470_body13_e6939_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign6470_body13_e6933: f64 = (locals.var_ffdtef * locals.var_itf);
        let assign6470_body13_e6936: f64 = (1.0 + p.p70);
        let assign6470_body13_e6937: f64 = (assign6470_body13_e6933 / assign6470_body13_e6936);
        (assign6470_body13_e6937, (((locals.var_ffdtef_dn0 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn0)) / assign6470_body13_e6936), (((locals.var_ffdtef_dn1 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn1)) / assign6470_body13_e6936), (((locals.var_ffdtef_dn3 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn3)) / assign6470_body13_e6936), (((locals.var_ffdtef_dn4 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn4)) / assign6470_body13_e6936), (((locals.var_ffdtef_dn5 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn5)) / assign6470_body13_e6936), (((locals.var_ffdtef_dn6 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn6)) / assign6470_body13_e6936), (((locals.var_ffdtef_dn7 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn7)) / assign6470_body13_e6936), (((locals.var_ffdtef_dn8 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn8)) / assign6470_body13_e6936), (((locals.var_ffdtef_dn9 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn9)) / assign6470_body13_e6936),)
    } else {
        (locals.var_ffdqef, locals.var_ffdqef_dn0, locals.var_ffdqef_dn1, locals.var_ffdqef_dn3, locals.var_ffdqef_dn4, locals.var_ffdqef_dn5, locals.var_ffdqef_dn6, locals.var_ffdqef_dn7, locals.var_ffdqef_dn8, locals.var_ffdqef_dn9,)
    }
};
            locals.var_ffdqef = assign6470_body13_e6939;
            locals.var_ffdqef_dn0 = assign6470_body13_e6939_d_n0;
            locals.var_ffdqef_dn1 = assign6470_body13_e6939_d_n1;
            locals.var_ffdqef_dn3 = assign6470_body13_e6939_d_n3;
            locals.var_ffdqef_dn4 = assign6470_body13_e6939_d_n4;
            locals.var_ffdqef_dn5 = assign6470_body13_e6939_d_n5;
            locals.var_ffdqef_dn6 = assign6470_body13_e6939_d_n6;
            locals.var_ffdqef_dn7 = assign6470_body13_e6939_d_n7;
            locals.var_ffdqef_dn8 = assign6470_body13_e6939_d_n8;
            locals.var_ffdqef_dn9 = assign6470_body13_e6939_d_n9;
            locals.var_ffdqef_rv = 0.0;
            let assign6470_body14_e6944: f64 = (p.p75 / p.p74);
            let assign6470_body14_e6945: f64 = (0.05 * assign6470_body14_e6944);
            let assign6470_body14_e6946: f64 = if p.p83 < assign6470_body14_e6945 { 1.0 } else { 0.0 };
            locals.var_guard135 = assign6470_body14_e6946;
            locals.var_guard135_rv = 0.0;
            let (assign6470_body15_e6954, assign6470_body15_e6954_d_n0, assign6470_body15_e6954_d_n1, assign6470_body15_e6954_d_n3, assign6470_body15_e6954_d_n4, assign6470_body15_e6954_d_n5, assign6470_body15_e6954_d_n6, assign6470_body15_e6954_d_n7, assign6470_body15_e6954_d_n8, assign6470_body15_e6954_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard135 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ffdvc, locals.var_ffdvc_dn0, locals.var_ffdvc_dn1, locals.var_ffdvc_dn3, locals.var_ffdvc_dn4, locals.var_ffdvc_dn5, locals.var_ffdvc_dn6, locals.var_ffdvc_dn7, locals.var_ffdvc_dn8, locals.var_ffdvc_dn9,)
    }
};
            locals.var_ffdvc = assign6470_body15_e6954;
            locals.var_ffdvc_dn0 = assign6470_body15_e6954_d_n0;
            locals.var_ffdvc_dn1 = assign6470_body15_e6954_d_n1;
            locals.var_ffdvc_dn3 = assign6470_body15_e6954_d_n3;
            locals.var_ffdvc_dn4 = assign6470_body15_e6954_d_n4;
            locals.var_ffdvc_dn5 = assign6470_body15_e6954_d_n5;
            locals.var_ffdvc_dn6 = assign6470_body15_e6954_d_n6;
            locals.var_ffdvc_dn7 = assign6470_body15_e6954_d_n7;
            locals.var_ffdvc_dn8 = assign6470_body15_e6954_d_n8;
            locals.var_ffdvc_dn9 = assign6470_body15_e6954_d_n9;
            locals.var_ffdvc_rv = 0.0;
            let (assign6470_body16_e6962, assign6470_body16_e6962_d_n0, assign6470_body16_e6962_d_n1, assign6470_body16_e6962_d_n3, assign6470_body16_e6962_d_n4, assign6470_body16_e6962_d_n5, assign6470_body16_e6962_d_n6, assign6470_body16_e6962_d_n7, assign6470_body16_e6962_d_n8, assign6470_body16_e6962_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard135 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ffdvc_ditf, locals.var_ffdvc_ditf_dn0, locals.var_ffdvc_ditf_dn1, locals.var_ffdvc_ditf_dn3, locals.var_ffdvc_ditf_dn4, locals.var_ffdvc_ditf_dn5, locals.var_ffdvc_ditf_dn6, locals.var_ffdvc_ditf_dn7, locals.var_ffdvc_ditf_dn8, locals.var_ffdvc_ditf_dn9,)
    }
};
            locals.var_ffdvc_ditf = assign6470_body16_e6962;
            locals.var_ffdvc_ditf_dn0 = assign6470_body16_e6962_d_n0;
            locals.var_ffdvc_ditf_dn1 = assign6470_body16_e6962_d_n1;
            locals.var_ffdvc_ditf_dn3 = assign6470_body16_e6962_d_n3;
            locals.var_ffdvc_ditf_dn4 = assign6470_body16_e6962_d_n4;
            locals.var_ffdvc_ditf_dn5 = assign6470_body16_e6962_d_n5;
            locals.var_ffdvc_ditf_dn6 = assign6470_body16_e6962_d_n6;
            locals.var_ffdvc_ditf_dn7 = assign6470_body16_e6962_d_n7;
            locals.var_ffdvc_ditf_dn8 = assign6470_body16_e6962_d_n8;
            locals.var_ffdvc_ditf_dn9 = assign6470_body16_e6962_d_n9;
            locals.var_ffdvc_ditf_rv = 0.0;
            let (assign6470_body17_e6975, assign6470_body17_e6975_d_n0, assign6470_body17_e6975_d_n1, assign6470_body17_e6975_d_n3, assign6470_body17_e6975_d_n4, assign6470_body17_e6975_d_n5, assign6470_body17_e6975_d_n6, assign6470_body17_e6975_d_n7, assign6470_body17_e6975_d_n8, assign6470_body17_e6975_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard135 == 0.0)) {
        let assign6470_body17_e6971: f64 = (locals.var_itf - locals.var_ick);
        let assign6470_body17_e6973: f64 = (assign6470_body17_e6971 / p.p83);
        (assign6470_body17_e6973, (locals.var_itf_dn0 / p.p83), (locals.var_itf_dn1 / p.p83), (locals.var_itf_dn3 / p.p83), ((locals.var_itf_dn4 - locals.var_ick_dn4) / p.p83), ((locals.var_itf_dn5 - locals.var_ick_dn5) / p.p83), ((locals.var_itf_dn6 - locals.var_ick_dn6) / p.p83), (locals.var_itf_dn7 / p.p83), ((locals.var_itf_dn8 - locals.var_ick_dn8) / p.p83), (locals.var_itf_dn9 / p.p83),)
    } else {
        (locals.var_ffib, locals.var_ffib_dn0, locals.var_ffib_dn1, locals.var_ffib_dn3, locals.var_ffib_dn4, locals.var_ffib_dn5, locals.var_ffib_dn6, locals.var_ffib_dn7, locals.var_ffib_dn8, locals.var_ffib_dn9,)
    }
};
            locals.var_ffib = assign6470_body17_e6975;
            locals.var_ffib_dn0 = assign6470_body17_e6975_d_n0;
            locals.var_ffib_dn1 = assign6470_body17_e6975_d_n1;
            locals.var_ffib_dn3 = assign6470_body17_e6975_d_n3;
            locals.var_ffib_dn4 = assign6470_body17_e6975_d_n4;
            locals.var_ffib_dn5 = assign6470_body17_e6975_d_n5;
            locals.var_ffib_dn6 = assign6470_body17_e6975_d_n6;
            locals.var_ffib_dn7 = assign6470_body17_e6975_d_n7;
            locals.var_ffib_dn8 = assign6470_body17_e6975_d_n8;
            locals.var_ffib_dn9 = assign6470_body17_e6975_d_n9;
            locals.var_ffib_rv = 0.0;
            let assign6470_body18_e6978: f64 = (-10000000000.0);
            let assign6470_body18_e6979: f64 = if locals.var_ffib < assign6470_body18_e6978 { 1.0 } else { 0.0 };
            locals.var_guard136 = assign6470_body18_e6979;
            locals.var_guard136_rv = 0.0;
            let (assign6470_body19_e6991, assign6470_body19_e6991_d_n0, assign6470_body19_e6991_d_n1, assign6470_body19_e6991_d_n3, assign6470_body19_e6991_d_n4, assign6470_body19_e6991_d_n5, assign6470_body19_e6991_d_n6, assign6470_body19_e6991_d_n7, assign6470_body19_e6991_d_n8, assign6470_body19_e6991_d_n9,) = {
    if ((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard135 == 0.0)) && (locals.var_guard136 != 0.0)) {
        let assign6470_body19_e6989: f64 = (-10000000000.0);
        (assign6470_body19_e6989, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ffib, locals.var_ffib_dn0, locals.var_ffib_dn1, locals.var_ffib_dn3, locals.var_ffib_dn4, locals.var_ffib_dn5, locals.var_ffib_dn6, locals.var_ffib_dn7, locals.var_ffib_dn8, locals.var_ffib_dn9,)
    }
};
            locals.var_ffib = assign6470_body19_e6991;
            locals.var_ffib_dn0 = assign6470_body19_e6991_d_n0;
            locals.var_ffib_dn1 = assign6470_body19_e6991_d_n1;
            locals.var_ffib_dn3 = assign6470_body19_e6991_d_n3;
            locals.var_ffib_dn4 = assign6470_body19_e6991_d_n4;
            locals.var_ffib_dn5 = assign6470_body19_e6991_d_n5;
            locals.var_ffib_dn6 = assign6470_body19_e6991_d_n6;
            locals.var_ffib_dn7 = assign6470_body19_e6991_d_n7;
            locals.var_ffib_dn8 = assign6470_body19_e6991_d_n8;
            locals.var_ffib_dn9 = assign6470_body19_e6991_d_n9;
            locals.var_ffib_rv = 0.0;
            let (assign6470_body20_e7005, assign6470_body20_e7005_d_n0, assign6470_body20_e7005_d_n1, assign6470_body20_e7005_d_n3, assign6470_body20_e7005_d_n4, assign6470_body20_e7005_d_n5, assign6470_body20_e7005_d_n6, assign6470_body20_e7005_d_n7, assign6470_body20_e7005_d_n8, assign6470_body20_e7005_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard135 == 0.0)) {
        let assign6470_body20_e7000: f64 = (locals.var_ffib * locals.var_ffib);
        let assign6470_body20_e7002: f64 = (assign6470_body20_e7000 + p.p84);
        let assign6470_body20_e7003: f64 = (assign6470_body20_e7002).sqrt();
        (assign6470_body20_e7003, (((locals.var_ffib_dn0 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn0)) / (2.0 * assign6470_body20_e7003)), (((locals.var_ffib_dn1 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn1)) / (2.0 * assign6470_body20_e7003)), (((locals.var_ffib_dn3 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn3)) / (2.0 * assign6470_body20_e7003)), (((locals.var_ffib_dn4 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn4)) / (2.0 * assign6470_body20_e7003)), (((locals.var_ffib_dn5 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn5)) / (2.0 * assign6470_body20_e7003)), (((locals.var_ffib_dn6 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn6)) / (2.0 * assign6470_body20_e7003)), (((locals.var_ffib_dn7 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn7)) / (2.0 * assign6470_body20_e7003)), (((locals.var_ffib_dn8 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn8)) / (2.0 * assign6470_body20_e7003)), (((locals.var_ffib_dn9 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn9)) / (2.0 * assign6470_body20_e7003)),)
    } else {
        (locals.var_fffcbar, locals.var_fffcbar_dn0, locals.var_fffcbar_dn1, locals.var_fffcbar_dn3, locals.var_fffcbar_dn4, locals.var_fffcbar_dn5, locals.var_fffcbar_dn6, locals.var_fffcbar_dn7, locals.var_fffcbar_dn8, locals.var_fffcbar_dn9,)
    }
};
            locals.var_fffcbar = assign6470_body20_e7005;
            locals.var_fffcbar_dn0 = assign6470_body20_e7005_d_n0;
            locals.var_fffcbar_dn1 = assign6470_body20_e7005_d_n1;
            locals.var_fffcbar_dn3 = assign6470_body20_e7005_d_n3;
            locals.var_fffcbar_dn4 = assign6470_body20_e7005_d_n4;
            locals.var_fffcbar_dn5 = assign6470_body20_e7005_d_n5;
            locals.var_fffcbar_dn6 = assign6470_body20_e7005_d_n6;
            locals.var_fffcbar_dn7 = assign6470_body20_e7005_d_n7;
            locals.var_fffcbar_dn8 = assign6470_body20_e7005_d_n8;
            locals.var_fffcbar_dn9 = assign6470_body20_e7005_d_n9;
            locals.var_fffcbar_rv = 0.0;
            let (assign6470_body21_e7022, assign6470_body21_e7022_d_n0, assign6470_body21_e7022_d_n1, assign6470_body21_e7022_d_n3, assign6470_body21_e7022_d_n4, assign6470_body21_e7022_d_n5, assign6470_body21_e7022_d_n6, assign6470_body21_e7022_d_n7, assign6470_body21_e7022_d_n8, assign6470_body21_e7022_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard135 == 0.0)) {
        let assign6470_body21_e7014: f64 = (-2.0);
        let assign6470_body21_e7017: f64 = (locals.var_ffib + locals.var_fffcbar);
        let assign6470_body21_e7018: f64 = (assign6470_body21_e7014 / assign6470_body21_e7017);
        let assign6470_body21_e7019: f64 = (assign6470_body21_e7018).exp();
        let assign6470_body21_e7020: f64 = (p.p82 * assign6470_body21_e7019);
        (assign6470_body21_e7020, (p.p82 * (assign6470_body21_e7019 * (-((assign6470_body21_e7014 * (locals.var_ffib_dn0 + locals.var_fffcbar_dn0)) / (assign6470_body21_e7017 * assign6470_body21_e7017))))), (p.p82 * (assign6470_body21_e7019 * (-((assign6470_body21_e7014 * (locals.var_ffib_dn1 + locals.var_fffcbar_dn1)) / (assign6470_body21_e7017 * assign6470_body21_e7017))))), (p.p82 * (assign6470_body21_e7019 * (-((assign6470_body21_e7014 * (locals.var_ffib_dn3 + locals.var_fffcbar_dn3)) / (assign6470_body21_e7017 * assign6470_body21_e7017))))), (p.p82 * (assign6470_body21_e7019 * (-((assign6470_body21_e7014 * (locals.var_ffib_dn4 + locals.var_fffcbar_dn4)) / (assign6470_body21_e7017 * assign6470_body21_e7017))))), (p.p82 * (assign6470_body21_e7019 * (-((assign6470_body21_e7014 * (locals.var_ffib_dn5 + locals.var_fffcbar_dn5)) / (assign6470_body21_e7017 * assign6470_body21_e7017))))), (p.p82 * (assign6470_body21_e7019 * (-((assign6470_body21_e7014 * (locals.var_ffib_dn6 + locals.var_fffcbar_dn6)) / (assign6470_body21_e7017 * assign6470_body21_e7017))))), (p.p82 * (assign6470_body21_e7019 * (-((assign6470_body21_e7014 * (locals.var_ffib_dn7 + locals.var_fffcbar_dn7)) / (assign6470_body21_e7017 * assign6470_body21_e7017))))), (p.p82 * (assign6470_body21_e7019 * (-((assign6470_body21_e7014 * (locals.var_ffib_dn8 + locals.var_fffcbar_dn8)) / (assign6470_body21_e7017 * assign6470_body21_e7017))))), (p.p82 * (assign6470_body21_e7019 * (-((assign6470_body21_e7014 * (locals.var_ffib_dn9 + locals.var_fffcbar_dn9)) / (assign6470_body21_e7017 * assign6470_body21_e7017))))),)
    } else {
        (locals.var_ffdvc, locals.var_ffdvc_dn0, locals.var_ffdvc_dn1, locals.var_ffdvc_dn3, locals.var_ffdvc_dn4, locals.var_ffdvc_dn5, locals.var_ffdvc_dn6, locals.var_ffdvc_dn7, locals.var_ffdvc_dn8, locals.var_ffdvc_dn9,)
    }
};
            locals.var_ffdvc = assign6470_body21_e7022;
            locals.var_ffdvc_dn0 = assign6470_body21_e7022_d_n0;
            locals.var_ffdvc_dn1 = assign6470_body21_e7022_d_n1;
            locals.var_ffdvc_dn3 = assign6470_body21_e7022_d_n3;
            locals.var_ffdvc_dn4 = assign6470_body21_e7022_d_n4;
            locals.var_ffdvc_dn5 = assign6470_body21_e7022_d_n5;
            locals.var_ffdvc_dn6 = assign6470_body21_e7022_d_n6;
            locals.var_ffdvc_dn7 = assign6470_body21_e7022_d_n7;
            locals.var_ffdvc_dn8 = assign6470_body21_e7022_d_n8;
            locals.var_ffdvc_dn9 = assign6470_body21_e7022_d_n9;
            locals.var_ffdvc_rv = 0.0;
            let (assign6470_body22_e7041, assign6470_body22_e7041_d_n0, assign6470_body22_e7041_d_n1, assign6470_body22_e7041_d_n3, assign6470_body22_e7041_d_n4, assign6470_body22_e7041_d_n5, assign6470_body22_e7041_d_n6, assign6470_body22_e7041_d_n7, assign6470_body22_e7041_d_n8, assign6470_body22_e7041_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard135 == 0.0)) {
        let assign6470_body22_e7031: f64 = (2.0 * locals.var_ffdvc);
        let assign6470_body22_e7034: f64 = (p.p83 * locals.var_fffcbar);
        let assign6470_body22_e7037: f64 = (locals.var_ffib + locals.var_fffcbar);
        let assign6470_body22_e7038: f64 = (assign6470_body22_e7034 * assign6470_body22_e7037);
        let assign6470_body22_e7039: f64 = (assign6470_body22_e7031 / assign6470_body22_e7038);
        (assign6470_body22_e7039, ((((2.0 * locals.var_ffdvc_dn0) * assign6470_body22_e7038) - (assign6470_body22_e7031 * (((p.p83 * locals.var_fffcbar_dn0) * assign6470_body22_e7037) + (assign6470_body22_e7034 * (locals.var_ffib_dn0 + locals.var_fffcbar_dn0))))) / (assign6470_body22_e7038 * assign6470_body22_e7038)), ((((2.0 * locals.var_ffdvc_dn1) * assign6470_body22_e7038) - (assign6470_body22_e7031 * (((p.p83 * locals.var_fffcbar_dn1) * assign6470_body22_e7037) + (assign6470_body22_e7034 * (locals.var_ffib_dn1 + locals.var_fffcbar_dn1))))) / (assign6470_body22_e7038 * assign6470_body22_e7038)), ((((2.0 * locals.var_ffdvc_dn3) * assign6470_body22_e7038) - (assign6470_body22_e7031 * (((p.p83 * locals.var_fffcbar_dn3) * assign6470_body22_e7037) + (assign6470_body22_e7034 * (locals.var_ffib_dn3 + locals.var_fffcbar_dn3))))) / (assign6470_body22_e7038 * assign6470_body22_e7038)), ((((2.0 * locals.var_ffdvc_dn4) * assign6470_body22_e7038) - (assign6470_body22_e7031 * (((p.p83 * locals.var_fffcbar_dn4) * assign6470_body22_e7037) + (assign6470_body22_e7034 * (locals.var_ffib_dn4 + locals.var_fffcbar_dn4))))) / (assign6470_body22_e7038 * assign6470_body22_e7038)), ((((2.0 * locals.var_ffdvc_dn5) * assign6470_body22_e7038) - (assign6470_body22_e7031 * (((p.p83 * locals.var_fffcbar_dn5) * assign6470_body22_e7037) + (assign6470_body22_e7034 * (locals.var_ffib_dn5 + locals.var_fffcbar_dn5))))) / (assign6470_body22_e7038 * assign6470_body22_e7038)), ((((2.0 * locals.var_ffdvc_dn6) * assign6470_body22_e7038) - (assign6470_body22_e7031 * (((p.p83 * locals.var_fffcbar_dn6) * assign6470_body22_e7037) + (assign6470_body22_e7034 * (locals.var_ffib_dn6 + locals.var_fffcbar_dn6))))) / (assign6470_body22_e7038 * assign6470_body22_e7038)), ((((2.0 * locals.var_ffdvc_dn7) * assign6470_body22_e7038) - (assign6470_body22_e7031 * (((p.p83 * locals.var_fffcbar_dn7) * assign6470_body22_e7037) + (assign6470_body22_e7034 * (locals.var_ffib_dn7 + locals.var_fffcbar_dn7))))) / (assign6470_body22_e7038 * assign6470_body22_e7038)), ((((2.0 * locals.var_ffdvc_dn8) * assign6470_body22_e7038) - (assign6470_body22_e7031 * (((p.p83 * locals.var_fffcbar_dn8) * assign6470_body22_e7037) + (assign6470_body22_e7034 * (locals.var_ffib_dn8 + locals.var_fffcbar_dn8))))) / (assign6470_body22_e7038 * assign6470_body22_e7038)), ((((2.0 * locals.var_ffdvc_dn9) * assign6470_body22_e7038) - (assign6470_body22_e7031 * (((p.p83 * locals.var_fffcbar_dn9) * assign6470_body22_e7037) + (assign6470_body22_e7034 * (locals.var_ffib_dn9 + locals.var_fffcbar_dn9))))) / (assign6470_body22_e7038 * assign6470_body22_e7038)),)
    } else {
        (locals.var_ffdvc_ditf, locals.var_ffdvc_ditf_dn0, locals.var_ffdvc_ditf_dn1, locals.var_ffdvc_ditf_dn3, locals.var_ffdvc_ditf_dn4, locals.var_ffdvc_ditf_dn5, locals.var_ffdvc_ditf_dn6, locals.var_ffdvc_ditf_dn7, locals.var_ffdvc_ditf_dn8, locals.var_ffdvc_ditf_dn9,)
    }
};
            locals.var_ffdvc_ditf = assign6470_body22_e7041;
            locals.var_ffdvc_ditf_dn0 = assign6470_body22_e7041_d_n0;
            locals.var_ffdvc_ditf_dn1 = assign6470_body22_e7041_d_n1;
            locals.var_ffdvc_ditf_dn3 = assign6470_body22_e7041_d_n3;
            locals.var_ffdvc_ditf_dn4 = assign6470_body22_e7041_d_n4;
            locals.var_ffdvc_ditf_dn5 = assign6470_body22_e7041_d_n5;
            locals.var_ffdvc_ditf_dn6 = assign6470_body22_e7041_d_n6;
            locals.var_ffdvc_ditf_dn7 = assign6470_body22_e7041_d_n7;
            locals.var_ffdvc_ditf_dn8 = assign6470_body22_e7041_d_n8;
            locals.var_ffdvc_ditf_dn9 = assign6470_body22_e7041_d_n9;
            locals.var_ffdvc_ditf_rv = 0.0;
            let (assign6470_body23_e7058, assign6470_body23_e7058_d_n0, assign6470_body23_e7058_d_n1, assign6470_body23_e7058_d_n3, assign6470_body23_e7058_d_n4, assign6470_body23_e7058_d_n5, assign6470_body23_e7058_d_n6, assign6470_body23_e7058_d_n7, assign6470_body23_e7058_d_n8, assign6470_body23_e7058_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign6470_body23_e7047: f64 = (1.0 - p.p73);
        let assign6470_body23_e7049: f64 = (assign6470_body23_e7047 * locals.var_thcs_t);
        let assign6470_body23_e7052: f64 = (locals.var_ffdvc * locals.var_ovt);
        let assign6470_body23_e7053: f64 = (assign6470_body23_e7052).exp();
        let assign6470_body23_e7055: f64 = (assign6470_body23_e7053 - 1.0);
        let assign6470_body23_e7056: f64 = (assign6470_body23_e7049 * assign6470_body23_e7055);
        (assign6470_body23_e7056, (assign6470_body23_e7049 * (assign6470_body23_e7053 * (locals.var_ffdvc_dn0 * locals.var_ovt))), (assign6470_body23_e7049 * (assign6470_body23_e7053 * (locals.var_ffdvc_dn1 * locals.var_ovt))), (assign6470_body23_e7049 * (assign6470_body23_e7053 * (locals.var_ffdvc_dn3 * locals.var_ovt))), (((assign6470_body23_e7047 * locals.var_thcs_t_dn4) * assign6470_body23_e7055) + (assign6470_body23_e7049 * (assign6470_body23_e7053 * ((locals.var_ffdvc_dn4 * locals.var_ovt) + (locals.var_ffdvc * locals.var_ovt_dn4))))), (assign6470_body23_e7049 * (assign6470_body23_e7053 * (locals.var_ffdvc_dn5 * locals.var_ovt))), (assign6470_body23_e7049 * (assign6470_body23_e7053 * (locals.var_ffdvc_dn6 * locals.var_ovt))), (assign6470_body23_e7049 * (assign6470_body23_e7053 * (locals.var_ffdvc_dn7 * locals.var_ovt))), (assign6470_body23_e7049 * (assign6470_body23_e7053 * (locals.var_ffdvc_dn8 * locals.var_ovt))), (assign6470_body23_e7049 * (assign6470_body23_e7053 * (locals.var_ffdvc_dn9 * locals.var_ovt))),)
    } else {
        (locals.var_ffdqbfb, locals.var_ffdqbfb_dn0, locals.var_ffdqbfb_dn1, locals.var_ffdqbfb_dn3, locals.var_ffdqbfb_dn4, locals.var_ffdqbfb_dn5, locals.var_ffdqbfb_dn6, locals.var_ffdqbfb_dn7, locals.var_ffdqbfb_dn8, locals.var_ffdqbfb_dn9,)
    }
};
            locals.var_ffdqbfb = assign6470_body23_e7058;
            locals.var_ffdqbfb_dn0 = assign6470_body23_e7058_d_n0;
            locals.var_ffdqbfb_dn1 = assign6470_body23_e7058_d_n1;
            locals.var_ffdqbfb_dn3 = assign6470_body23_e7058_d_n3;
            locals.var_ffdqbfb_dn4 = assign6470_body23_e7058_d_n4;
            locals.var_ffdqbfb_dn5 = assign6470_body23_e7058_d_n5;
            locals.var_ffdqbfb_dn6 = assign6470_body23_e7058_d_n6;
            locals.var_ffdqbfb_dn7 = assign6470_body23_e7058_d_n7;
            locals.var_ffdqbfb_dn8 = assign6470_body23_e7058_d_n8;
            locals.var_ffdqbfb_dn9 = assign6470_body23_e7058_d_n9;
            locals.var_ffdqbfb_rv = 0.0;
            let (assign6470_body24_e7081, assign6470_body24_e7081_d_n0, assign6470_body24_e7081_d_n1, assign6470_body24_e7081_d_n3, assign6470_body24_e7081_d_n4, assign6470_body24_e7081_d_n5, assign6470_body24_e7081_d_n6, assign6470_body24_e7081_d_n7, assign6470_body24_e7081_d_n8, assign6470_body24_e7081_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign6470_body24_e7065: f64 = (1.0 - p.p73);
        let assign6470_body24_e7067: f64 = (assign6470_body24_e7065 * locals.var_thcs_t);
        let assign6470_body24_e7069: f64 = (assign6470_body24_e7067 * locals.var_itf);
        let assign6470_body24_e7072: f64 = (locals.var_ffdvc * locals.var_ovt);
        let assign6470_body24_e7073: f64 = (assign6470_body24_e7072).exp();
        let assign6470_body24_e7074: f64 = (assign6470_body24_e7069 * assign6470_body24_e7073);
        let assign6470_body24_e7076: f64 = (assign6470_body24_e7074 * locals.var_ovt);
        let assign6470_body24_e7078: f64 = (assign6470_body24_e7076 * locals.var_ffdvc_ditf);
        let assign6470_body24_e7079: f64 = (locals.var_ffdqbfb + assign6470_body24_e7078);
        (assign6470_body24_e7079, (locals.var_ffdqbfb_dn0 + ((((((assign6470_body24_e7067 * locals.var_itf_dn0) * assign6470_body24_e7073) + (assign6470_body24_e7069 * (assign6470_body24_e7073 * (locals.var_ffdvc_dn0 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign6470_body24_e7076 * locals.var_ffdvc_ditf_dn0))), (locals.var_ffdqbfb_dn1 + ((((((assign6470_body24_e7067 * locals.var_itf_dn1) * assign6470_body24_e7073) + (assign6470_body24_e7069 * (assign6470_body24_e7073 * (locals.var_ffdvc_dn1 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign6470_body24_e7076 * locals.var_ffdvc_ditf_dn1))), (locals.var_ffdqbfb_dn3 + ((((((assign6470_body24_e7067 * locals.var_itf_dn3) * assign6470_body24_e7073) + (assign6470_body24_e7069 * (assign6470_body24_e7073 * (locals.var_ffdvc_dn3 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign6470_body24_e7076 * locals.var_ffdvc_ditf_dn3))), (locals.var_ffdqbfb_dn4 + (((((((((assign6470_body24_e7065 * locals.var_thcs_t_dn4) * locals.var_itf) + (assign6470_body24_e7067 * locals.var_itf_dn4)) * assign6470_body24_e7073) + (assign6470_body24_e7069 * (assign6470_body24_e7073 * ((locals.var_ffdvc_dn4 * locals.var_ovt) + (locals.var_ffdvc * locals.var_ovt_dn4))))) * locals.var_ovt) + (assign6470_body24_e7074 * locals.var_ovt_dn4)) * locals.var_ffdvc_ditf) + (assign6470_body24_e7076 * locals.var_ffdvc_ditf_dn4))), (locals.var_ffdqbfb_dn5 + ((((((assign6470_body24_e7067 * locals.var_itf_dn5) * assign6470_body24_e7073) + (assign6470_body24_e7069 * (assign6470_body24_e7073 * (locals.var_ffdvc_dn5 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign6470_body24_e7076 * locals.var_ffdvc_ditf_dn5))), (locals.var_ffdqbfb_dn6 + ((((((assign6470_body24_e7067 * locals.var_itf_dn6) * assign6470_body24_e7073) + (assign6470_body24_e7069 * (assign6470_body24_e7073 * (locals.var_ffdvc_dn6 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign6470_body24_e7076 * locals.var_ffdvc_ditf_dn6))), (locals.var_ffdqbfb_dn7 + ((((((assign6470_body24_e7067 * locals.var_itf_dn7) * assign6470_body24_e7073) + (assign6470_body24_e7069 * (assign6470_body24_e7073 * (locals.var_ffdvc_dn7 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign6470_body24_e7076 * locals.var_ffdvc_ditf_dn7))), (locals.var_ffdqbfb_dn8 + ((((((assign6470_body24_e7067 * locals.var_itf_dn8) * assign6470_body24_e7073) + (assign6470_body24_e7069 * (assign6470_body24_e7073 * (locals.var_ffdvc_dn8 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign6470_body24_e7076 * locals.var_ffdvc_ditf_dn8))), (locals.var_ffdqbfb_dn9 + ((((((assign6470_body24_e7067 * locals.var_itf_dn9) * assign6470_body24_e7073) + (assign6470_body24_e7069 * (assign6470_body24_e7073 * (locals.var_ffdvc_dn9 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign6470_body24_e7076 * locals.var_ffdvc_ditf_dn9))),)
    } else {
        (locals.var_ffdtbfb, locals.var_ffdtbfb_dn0, locals.var_ffdtbfb_dn1, locals.var_ffdtbfb_dn3, locals.var_ffdtbfb_dn4, locals.var_ffdtbfb_dn5, locals.var_ffdtbfb_dn6, locals.var_ffdtbfb_dn7, locals.var_ffdtbfb_dn8, locals.var_ffdtbfb_dn9,)
    }
};
            locals.var_ffdtbfb = assign6470_body24_e7081;
            locals.var_ffdtbfb_dn0 = assign6470_body24_e7081_d_n0;
            locals.var_ffdtbfb_dn1 = assign6470_body24_e7081_d_n1;
            locals.var_ffdtbfb_dn3 = assign6470_body24_e7081_d_n3;
            locals.var_ffdtbfb_dn4 = assign6470_body24_e7081_d_n4;
            locals.var_ffdtbfb_dn5 = assign6470_body24_e7081_d_n5;
            locals.var_ffdtbfb_dn6 = assign6470_body24_e7081_d_n6;
            locals.var_ffdtbfb_dn7 = assign6470_body24_e7081_d_n7;
            locals.var_ffdtbfb_dn8 = assign6470_body24_e7081_d_n8;
            locals.var_ffdtbfb_dn9 = assign6470_body24_e7081_d_n9;
            locals.var_ffdtbfb_rv = 0.0;
            let (assign6470_body25_e7091, assign6470_body25_e7091_d_n0, assign6470_body25_e7091_d_n1, assign6470_body25_e7091_d_n3, assign6470_body25_e7091_d_n4, assign6470_body25_e7091_d_n5, assign6470_body25_e7091_d_n6, assign6470_body25_e7091_d_n7, assign6470_body25_e7091_d_n8, assign6470_body25_e7091_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign6470_body25_e7088: f64 = (1.0 / locals.var_ffitf_ick);
        let assign6470_body25_e7089: f64 = (1.0 - assign6470_body25_e7088);
        (assign6470_body25_e7089, (-(-(locals.var_ffitf_ick_dn0 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn1 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn3 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn4 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn5 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn6 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn7 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn8 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn9 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))),)
    } else {
        (locals.var_ffic, locals.var_ffic_dn0, locals.var_ffic_dn1, locals.var_ffic_dn3, locals.var_ffic_dn4, locals.var_ffic_dn5, locals.var_ffic_dn6, locals.var_ffic_dn7, locals.var_ffic_dn8, locals.var_ffic_dn9,)
    }
};
            locals.var_ffic = assign6470_body25_e7091;
            locals.var_ffic_dn0 = assign6470_body25_e7091_d_n0;
            locals.var_ffic_dn1 = assign6470_body25_e7091_d_n1;
            locals.var_ffic_dn3 = assign6470_body25_e7091_d_n3;
            locals.var_ffic_dn4 = assign6470_body25_e7091_d_n4;
            locals.var_ffic_dn5 = assign6470_body25_e7091_d_n5;
            locals.var_ffic_dn6 = assign6470_body25_e7091_d_n6;
            locals.var_ffic_dn7 = assign6470_body25_e7091_d_n7;
            locals.var_ffic_dn8 = assign6470_body25_e7091_d_n8;
            locals.var_ffic_dn9 = assign6470_body25_e7091_d_n9;
            locals.var_ffic_rv = 0.0;
            let (assign6470_body26_e7111, assign6470_body26_e7111_d_n0, assign6470_body26_e7111_d_n1, assign6470_body26_e7111_d_n3, assign6470_body26_e7111_d_n4, assign6470_body26_e7111_d_n5, assign6470_body26_e7111_d_n6, assign6470_body26_e7111_d_n7, assign6470_body26_e7111_d_n8, assign6470_body26_e7111_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign6470_body26_e7098: f64 = (locals.var_ffic * locals.var_ffic);
        let assign6470_body26_e7100: f64 = (assign6470_body26_e7098 + p.p72);
        let assign6470_body26_e7101: f64 = (assign6470_body26_e7100).sqrt();
        let assign6470_body26_e7102: f64 = (locals.var_ffic + assign6470_body26_e7101);
        let assign6470_body26_e7106: f64 = (1.0 + p.p72);
        let assign6470_body26_e7107: f64 = (assign6470_body26_e7106).sqrt();
        let assign6470_body26_e7108: f64 = (1.0 + assign6470_body26_e7107);
        let assign6470_body26_e7109: f64 = (assign6470_body26_e7102 / assign6470_body26_e7108);
        (assign6470_body26_e7109, ((locals.var_ffic_dn0 + (((locals.var_ffic_dn0 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn0)) / (2.0 * assign6470_body26_e7101))) / assign6470_body26_e7108), ((locals.var_ffic_dn1 + (((locals.var_ffic_dn1 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn1)) / (2.0 * assign6470_body26_e7101))) / assign6470_body26_e7108), ((locals.var_ffic_dn3 + (((locals.var_ffic_dn3 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn3)) / (2.0 * assign6470_body26_e7101))) / assign6470_body26_e7108), ((locals.var_ffic_dn4 + (((locals.var_ffic_dn4 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn4)) / (2.0 * assign6470_body26_e7101))) / assign6470_body26_e7108), ((locals.var_ffic_dn5 + (((locals.var_ffic_dn5 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn5)) / (2.0 * assign6470_body26_e7101))) / assign6470_body26_e7108), ((locals.var_ffic_dn6 + (((locals.var_ffic_dn6 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn6)) / (2.0 * assign6470_body26_e7101))) / assign6470_body26_e7108), ((locals.var_ffic_dn7 + (((locals.var_ffic_dn7 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn7)) / (2.0 * assign6470_body26_e7101))) / assign6470_body26_e7108), ((locals.var_ffic_dn8 + (((locals.var_ffic_dn8 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn8)) / (2.0 * assign6470_body26_e7101))) / assign6470_body26_e7108), ((locals.var_ffic_dn9 + (((locals.var_ffic_dn9 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn9)) / (2.0 * assign6470_body26_e7101))) / assign6470_body26_e7108),)
    } else {
        (locals.var_ffw, locals.var_ffw_dn0, locals.var_ffw_dn1, locals.var_ffw_dn3, locals.var_ffw_dn4, locals.var_ffw_dn5, locals.var_ffw_dn6, locals.var_ffw_dn7, locals.var_ffw_dn8, locals.var_ffw_dn9,)
    }
};
            locals.var_ffw = assign6470_body26_e7111;
            locals.var_ffw_dn0 = assign6470_body26_e7111_d_n0;
            locals.var_ffw_dn1 = assign6470_body26_e7111_d_n1;
            locals.var_ffw_dn3 = assign6470_body26_e7111_d_n3;
            locals.var_ffw_dn4 = assign6470_body26_e7111_d_n4;
            locals.var_ffw_dn5 = assign6470_body26_e7111_d_n5;
            locals.var_ffw_dn6 = assign6470_body26_e7111_d_n6;
            locals.var_ffw_dn7 = assign6470_body26_e7111_d_n7;
            locals.var_ffw_dn8 = assign6470_body26_e7111_d_n8;
            locals.var_ffw_dn9 = assign6470_body26_e7111_d_n9;
            locals.var_ffw_rv = 0.0;
            let (assign6470_body27_e7122, assign6470_body27_e7122_d_n0, assign6470_body27_e7122_d_n1, assign6470_body27_e7122_d_n3, assign6470_body27_e7122_d_n4, assign6470_body27_e7122_d_n5, assign6470_body27_e7122_d_n6, assign6470_body27_e7122_d_n7, assign6470_body27_e7122_d_n8, assign6470_body27_e7122_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign6470_body27_e7117: f64 = (locals.var_ffdvc - p.p82);
        let assign6470_body27_e7119: f64 = (assign6470_body27_e7117 * locals.var_ovt);
        let assign6470_body27_e7120: f64 = (assign6470_body27_e7119).exp();
        (assign6470_body27_e7120, (assign6470_body27_e7120 * (locals.var_ffdvc_dn0 * locals.var_ovt)), (assign6470_body27_e7120 * (locals.var_ffdvc_dn1 * locals.var_ovt)), (assign6470_body27_e7120 * (locals.var_ffdvc_dn3 * locals.var_ovt)), (assign6470_body27_e7120 * ((locals.var_ffdvc_dn4 * locals.var_ovt) + (assign6470_body27_e7117 * locals.var_ovt_dn4))), (assign6470_body27_e7120 * (locals.var_ffdvc_dn5 * locals.var_ovt)), (assign6470_body27_e7120 * (locals.var_ffdvc_dn6 * locals.var_ovt)), (assign6470_body27_e7120 * (locals.var_ffdvc_dn7 * locals.var_ovt)), (assign6470_body27_e7120 * (locals.var_ffdvc_dn8 * locals.var_ovt)), (assign6470_body27_e7120 * (locals.var_ffdvc_dn9 * locals.var_ovt)),)
    } else {
        (locals.var_ffvc_exp, locals.var_ffvc_exp_dn0, locals.var_ffvc_exp_dn1, locals.var_ffvc_exp_dn3, locals.var_ffvc_exp_dn4, locals.var_ffvc_exp_dn5, locals.var_ffvc_exp_dn6, locals.var_ffvc_exp_dn7, locals.var_ffvc_exp_dn8, locals.var_ffvc_exp_dn9,)
    }
};
            locals.var_ffvc_exp = assign6470_body27_e7122;
            locals.var_ffvc_exp_dn0 = assign6470_body27_e7122_d_n0;
            locals.var_ffvc_exp_dn1 = assign6470_body27_e7122_d_n1;
            locals.var_ffvc_exp_dn3 = assign6470_body27_e7122_d_n3;
            locals.var_ffvc_exp_dn4 = assign6470_body27_e7122_d_n4;
            locals.var_ffvc_exp_dn5 = assign6470_body27_e7122_d_n5;
            locals.var_ffvc_exp_dn6 = assign6470_body27_e7122_d_n6;
            locals.var_ffvc_exp_dn7 = assign6470_body27_e7122_d_n7;
            locals.var_ffvc_exp_dn8 = assign6470_body27_e7122_d_n8;
            locals.var_ffvc_exp_dn9 = assign6470_body27_e7122_d_n9;
            locals.var_ffvc_exp_rv = 0.0;
            let (assign6470_body28_e7134, assign6470_body28_e7134_d_n0, assign6470_body28_e7134_d_n1, assign6470_body28_e7134_d_n3, assign6470_body28_e7134_d_n4, assign6470_body28_e7134_d_n5, assign6470_body28_e7134_d_n6, assign6470_body28_e7134_d_n7, assign6470_body28_e7134_d_n8, assign6470_body28_e7134_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign6470_body28_e7128: f64 = (locals.var_thcs_t * locals.var_ffw);
        let assign6470_body28_e7130: f64 = (assign6470_body28_e7128 * locals.var_ffw);
        let assign6470_body28_e7132: f64 = (assign6470_body28_e7130 * locals.var_ffvc_exp);
        (assign6470_body28_e7132, (((((locals.var_thcs_t * locals.var_ffw_dn0) * locals.var_ffw) + (assign6470_body28_e7128 * locals.var_ffw_dn0)) * locals.var_ffvc_exp) + (assign6470_body28_e7130 * locals.var_ffvc_exp_dn0)), (((((locals.var_thcs_t * locals.var_ffw_dn1) * locals.var_ffw) + (assign6470_body28_e7128 * locals.var_ffw_dn1)) * locals.var_ffvc_exp) + (assign6470_body28_e7130 * locals.var_ffvc_exp_dn1)), (((((locals.var_thcs_t * locals.var_ffw_dn3) * locals.var_ffw) + (assign6470_body28_e7128 * locals.var_ffw_dn3)) * locals.var_ffvc_exp) + (assign6470_body28_e7130 * locals.var_ffvc_exp_dn3)), ((((((locals.var_thcs_t_dn4 * locals.var_ffw) + (locals.var_thcs_t * locals.var_ffw_dn4)) * locals.var_ffw) + (assign6470_body28_e7128 * locals.var_ffw_dn4)) * locals.var_ffvc_exp) + (assign6470_body28_e7130 * locals.var_ffvc_exp_dn4)), (((((locals.var_thcs_t * locals.var_ffw_dn5) * locals.var_ffw) + (assign6470_body28_e7128 * locals.var_ffw_dn5)) * locals.var_ffvc_exp) + (assign6470_body28_e7130 * locals.var_ffvc_exp_dn5)), (((((locals.var_thcs_t * locals.var_ffw_dn6) * locals.var_ffw) + (assign6470_body28_e7128 * locals.var_ffw_dn6)) * locals.var_ffvc_exp) + (assign6470_body28_e7130 * locals.var_ffvc_exp_dn6)), (((((locals.var_thcs_t * locals.var_ffw_dn7) * locals.var_ffw) + (assign6470_body28_e7128 * locals.var_ffw_dn7)) * locals.var_ffvc_exp) + (assign6470_body28_e7130 * locals.var_ffvc_exp_dn7)), (((((locals.var_thcs_t * locals.var_ffw_dn8) * locals.var_ffw) + (assign6470_body28_e7128 * locals.var_ffw_dn8)) * locals.var_ffvc_exp) + (assign6470_body28_e7130 * locals.var_ffvc_exp_dn8)), (((((locals.var_thcs_t * locals.var_ffw_dn9) * locals.var_ffw) + (assign6470_body28_e7128 * locals.var_ffw_dn9)) * locals.var_ffvc_exp) + (assign6470_body28_e7130 * locals.var_ffvc_exp_dn9)),)
    } else {
        (locals.var_ffdqfhc, locals.var_ffdqfhc_dn0, locals.var_ffdqfhc_dn1, locals.var_ffdqfhc_dn3, locals.var_ffdqfhc_dn4, locals.var_ffdqfhc_dn5, locals.var_ffdqfhc_dn6, locals.var_ffdqfhc_dn7, locals.var_ffdqfhc_dn8, locals.var_ffdqfhc_dn9,)
    }
};
            locals.var_ffdqfhc = assign6470_body28_e7134;
            locals.var_ffdqfhc_dn0 = assign6470_body28_e7134_d_n0;
            locals.var_ffdqfhc_dn1 = assign6470_body28_e7134_d_n1;
            locals.var_ffdqfhc_dn3 = assign6470_body28_e7134_d_n3;
            locals.var_ffdqfhc_dn4 = assign6470_body28_e7134_d_n4;
            locals.var_ffdqfhc_dn5 = assign6470_body28_e7134_d_n5;
            locals.var_ffdqfhc_dn6 = assign6470_body28_e7134_d_n6;
            locals.var_ffdqfhc_dn7 = assign6470_body28_e7134_d_n7;
            locals.var_ffdqfhc_dn8 = assign6470_body28_e7134_d_n8;
            locals.var_ffdqfhc_dn9 = assign6470_body28_e7134_d_n9;
            locals.var_ffdqfhc_rv = 0.0;
            let (assign6470_body29_e7159, assign6470_body29_e7159_d_n0, assign6470_body29_e7159_d_n1, assign6470_body29_e7159_d_n3, assign6470_body29_e7159_d_n4, assign6470_body29_e7159_d_n5, assign6470_body29_e7159_d_n6, assign6470_body29_e7159_d_n7, assign6470_body29_e7159_d_n8, assign6470_body29_e7159_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign6470_body29_e7144: f64 = (locals.var_ffic * locals.var_ffic);
        let assign6470_body29_e7146: f64 = (assign6470_body29_e7144 + p.p72);
        let assign6470_body29_e7147: f64 = (assign6470_body29_e7146).sqrt();
        let assign6470_body29_e7148: f64 = (locals.var_ffitf_ick * assign6470_body29_e7147);
        let assign6470_body29_e7149: f64 = (2.0 / assign6470_body29_e7148);
        let assign6470_body29_e7150: f64 = (1.0 + assign6470_body29_e7149);
        let assign6470_body29_e7153: f64 = (locals.var_ovt * locals.var_itf);
        let assign6470_body29_e7155: f64 = (assign6470_body29_e7153 * locals.var_ffdvc_ditf);
        let assign6470_body29_e7156: f64 = (assign6470_body29_e7150 + assign6470_body29_e7155);
        let assign6470_body29_e7157: f64 = (locals.var_ffdqfhc * assign6470_body29_e7156);
        (assign6470_body29_e7157, ((locals.var_ffdqfhc_dn0 * assign6470_body29_e7156) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn0 * assign6470_body29_e7147) + (locals.var_ffitf_ick * (((locals.var_ffic_dn0 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn0)) / (2.0 * assign6470_body29_e7147))))) / (assign6470_body29_e7148 * assign6470_body29_e7148))) + (((locals.var_ovt * locals.var_itf_dn0) * locals.var_ffdvc_ditf) + (assign6470_body29_e7153 * locals.var_ffdvc_ditf_dn0))))), ((locals.var_ffdqfhc_dn1 * assign6470_body29_e7156) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn1 * assign6470_body29_e7147) + (locals.var_ffitf_ick * (((locals.var_ffic_dn1 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn1)) / (2.0 * assign6470_body29_e7147))))) / (assign6470_body29_e7148 * assign6470_body29_e7148))) + (((locals.var_ovt * locals.var_itf_dn1) * locals.var_ffdvc_ditf) + (assign6470_body29_e7153 * locals.var_ffdvc_ditf_dn1))))), ((locals.var_ffdqfhc_dn3 * assign6470_body29_e7156) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn3 * assign6470_body29_e7147) + (locals.var_ffitf_ick * (((locals.var_ffic_dn3 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn3)) / (2.0 * assign6470_body29_e7147))))) / (assign6470_body29_e7148 * assign6470_body29_e7148))) + (((locals.var_ovt * locals.var_itf_dn3) * locals.var_ffdvc_ditf) + (assign6470_body29_e7153 * locals.var_ffdvc_ditf_dn3))))), ((locals.var_ffdqfhc_dn4 * assign6470_body29_e7156) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn4 * assign6470_body29_e7147) + (locals.var_ffitf_ick * (((locals.var_ffic_dn4 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn4)) / (2.0 * assign6470_body29_e7147))))) / (assign6470_body29_e7148 * assign6470_body29_e7148))) + ((((locals.var_ovt_dn4 * locals.var_itf) + (locals.var_ovt * locals.var_itf_dn4)) * locals.var_ffdvc_ditf) + (assign6470_body29_e7153 * locals.var_ffdvc_ditf_dn4))))), ((locals.var_ffdqfhc_dn5 * assign6470_body29_e7156) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn5 * assign6470_body29_e7147) + (locals.var_ffitf_ick * (((locals.var_ffic_dn5 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn5)) / (2.0 * assign6470_body29_e7147))))) / (assign6470_body29_e7148 * assign6470_body29_e7148))) + (((locals.var_ovt * locals.var_itf_dn5) * locals.var_ffdvc_ditf) + (assign6470_body29_e7153 * locals.var_ffdvc_ditf_dn5))))), ((locals.var_ffdqfhc_dn6 * assign6470_body29_e7156) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn6 * assign6470_body29_e7147) + (locals.var_ffitf_ick * (((locals.var_ffic_dn6 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn6)) / (2.0 * assign6470_body29_e7147))))) / (assign6470_body29_e7148 * assign6470_body29_e7148))) + (((locals.var_ovt * locals.var_itf_dn6) * locals.var_ffdvc_ditf) + (assign6470_body29_e7153 * locals.var_ffdvc_ditf_dn6))))), ((locals.var_ffdqfhc_dn7 * assign6470_body29_e7156) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn7 * assign6470_body29_e7147) + (locals.var_ffitf_ick * (((locals.var_ffic_dn7 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn7)) / (2.0 * assign6470_body29_e7147))))) / (assign6470_body29_e7148 * assign6470_body29_e7148))) + (((locals.var_ovt * locals.var_itf_dn7) * locals.var_ffdvc_ditf) + (assign6470_body29_e7153 * locals.var_ffdvc_ditf_dn7))))), ((locals.var_ffdqfhc_dn8 * assign6470_body29_e7156) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn8 * assign6470_body29_e7147) + (locals.var_ffitf_ick * (((locals.var_ffic_dn8 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn8)) / (2.0 * assign6470_body29_e7147))))) / (assign6470_body29_e7148 * assign6470_body29_e7148))) + (((locals.var_ovt * locals.var_itf_dn8) * locals.var_ffdvc_ditf) + (assign6470_body29_e7153 * locals.var_ffdvc_ditf_dn8))))), ((locals.var_ffdqfhc_dn9 * assign6470_body29_e7156) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn9 * assign6470_body29_e7147) + (locals.var_ffitf_ick * (((locals.var_ffic_dn9 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn9)) / (2.0 * assign6470_body29_e7147))))) / (assign6470_body29_e7148 * assign6470_body29_e7148))) + (((locals.var_ovt * locals.var_itf_dn9) * locals.var_ffdvc_ditf) + (assign6470_body29_e7153 * locals.var_ffdvc_ditf_dn9))))),)
    } else {
        (locals.var_ffdtfhc, locals.var_ffdtfhc_dn0, locals.var_ffdtfhc_dn1, locals.var_ffdtfhc_dn3, locals.var_ffdtfhc_dn4, locals.var_ffdtfhc_dn5, locals.var_ffdtfhc_dn6, locals.var_ffdtfhc_dn7, locals.var_ffdtfhc_dn8, locals.var_ffdtfhc_dn9,)
    }
};
            locals.var_ffdtfhc = assign6470_body29_e7159;
            locals.var_ffdtfhc_dn0 = assign6470_body29_e7159_d_n0;
            locals.var_ffdtfhc_dn1 = assign6470_body29_e7159_d_n1;
            locals.var_ffdtfhc_dn3 = assign6470_body29_e7159_d_n3;
            locals.var_ffdtfhc_dn4 = assign6470_body29_e7159_d_n4;
            locals.var_ffdtfhc_dn5 = assign6470_body29_e7159_d_n5;
            locals.var_ffdtfhc_dn6 = assign6470_body29_e7159_d_n6;
            locals.var_ffdtfhc_dn7 = assign6470_body29_e7159_d_n7;
            locals.var_ffdtfhc_dn8 = assign6470_body29_e7159_d_n8;
            locals.var_ffdtfhc_dn9 = assign6470_body29_e7159_d_n9;
            locals.var_ffdtfhc_rv = 0.0;
            let assign6470_body30_e7169: f64 = (locals.var_ffw * p.p115);
            let assign6470_body30_e7175: f64 = (locals.var_ffw * p.p116);
            let assign6470_body30_e7178: f64 = if ((((p.p115 < 0.01) && (p.p116 < 0.01)) && (assign6470_body30_e7169 < 0.005)) && (assign6470_body30_e7175 < 0.005)) { 1.0 } else { 0.0 };
            locals.var_guard137 = assign6470_body30_e7178;
            locals.var_guard137_rv = 0.0;
            let (assign6470_body31_e7190, assign6470_body31_e7190_d_n0, assign6470_body31_e7190_d_n1, assign6470_body31_e7190_d_n3, assign6470_body31_e7190_d_n4, assign6470_body31_e7190_d_n5, assign6470_body31_e7190_d_n6, assign6470_body31_e7190_d_n7, assign6470_body31_e7190_d_n8, assign6470_body31_e7190_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 != 0.0)) {
        let assign6470_body31_e7186: f64 = (p.p73 * locals.var_ffdqfhc);
        let assign6470_body31_e7188: f64 = (assign6470_body31_e7186 * locals.var_itf);
        (assign6470_body31_e7188, (((p.p73 * locals.var_ffdqfhc_dn0) * locals.var_itf) + (assign6470_body31_e7186 * locals.var_itf_dn0)), (((p.p73 * locals.var_ffdqfhc_dn1) * locals.var_itf) + (assign6470_body31_e7186 * locals.var_itf_dn1)), (((p.p73 * locals.var_ffdqfhc_dn3) * locals.var_itf) + (assign6470_body31_e7186 * locals.var_itf_dn3)), (((p.p73 * locals.var_ffdqfhc_dn4) * locals.var_itf) + (assign6470_body31_e7186 * locals.var_itf_dn4)), (((p.p73 * locals.var_ffdqfhc_dn5) * locals.var_itf) + (assign6470_body31_e7186 * locals.var_itf_dn5)), (((p.p73 * locals.var_ffdqfhc_dn6) * locals.var_itf) + (assign6470_body31_e7186 * locals.var_itf_dn6)), (((p.p73 * locals.var_ffdqfhc_dn7) * locals.var_itf) + (assign6470_body31_e7186 * locals.var_itf_dn7)), (((p.p73 * locals.var_ffdqfhc_dn8) * locals.var_itf) + (assign6470_body31_e7186 * locals.var_itf_dn8)), (((p.p73 * locals.var_ffdqfhc_dn9) * locals.var_itf) + (assign6470_body31_e7186 * locals.var_itf_dn9)),)
    } else {
        (locals.var_ffdqcfc, locals.var_ffdqcfc_dn0, locals.var_ffdqcfc_dn1, locals.var_ffdqcfc_dn3, locals.var_ffdqcfc_dn4, locals.var_ffdqcfc_dn5, locals.var_ffdqcfc_dn6, locals.var_ffdqcfc_dn7, locals.var_ffdqcfc_dn8, locals.var_ffdqcfc_dn9,)
    }
};
            locals.var_ffdqcfc = assign6470_body31_e7190;
            locals.var_ffdqcfc_dn0 = assign6470_body31_e7190_d_n0;
            locals.var_ffdqcfc_dn1 = assign6470_body31_e7190_d_n1;
            locals.var_ffdqcfc_dn3 = assign6470_body31_e7190_d_n3;
            locals.var_ffdqcfc_dn4 = assign6470_body31_e7190_d_n4;
            locals.var_ffdqcfc_dn5 = assign6470_body31_e7190_d_n5;
            locals.var_ffdqcfc_dn6 = assign6470_body31_e7190_d_n6;
            locals.var_ffdqcfc_dn7 = assign6470_body31_e7190_d_n7;
            locals.var_ffdqcfc_dn8 = assign6470_body31_e7190_d_n8;
            locals.var_ffdqcfc_dn9 = assign6470_body31_e7190_d_n9;
            locals.var_ffdqcfc_rv = 0.0;
            let (assign6470_body32_e7200, assign6470_body32_e7200_d_n0, assign6470_body32_e7200_d_n1, assign6470_body32_e7200_d_n3, assign6470_body32_e7200_d_n4, assign6470_body32_e7200_d_n5, assign6470_body32_e7200_d_n6, assign6470_body32_e7200_d_n7, assign6470_body32_e7200_d_n8, assign6470_body32_e7200_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 != 0.0)) {
        let assign6470_body32_e7198: f64 = (p.p73 * locals.var_ffdtfhc);
        (assign6470_body32_e7198, (p.p73 * locals.var_ffdtfhc_dn0), (p.p73 * locals.var_ffdtfhc_dn1), (p.p73 * locals.var_ffdtfhc_dn3), (p.p73 * locals.var_ffdtfhc_dn4), (p.p73 * locals.var_ffdtfhc_dn5), (p.p73 * locals.var_ffdtfhc_dn6), (p.p73 * locals.var_ffdtfhc_dn7), (p.p73 * locals.var_ffdtfhc_dn8), (p.p73 * locals.var_ffdtfhc_dn9),)
    } else {
        (locals.var_ffdtcfc, locals.var_ffdtcfc_dn0, locals.var_ffdtcfc_dn1, locals.var_ffdtcfc_dn3, locals.var_ffdtcfc_dn4, locals.var_ffdtcfc_dn5, locals.var_ffdtcfc_dn6, locals.var_ffdtcfc_dn7, locals.var_ffdtcfc_dn8, locals.var_ffdtcfc_dn9,)
    }
};
            locals.var_ffdtcfc = assign6470_body32_e7200;
            locals.var_ffdtcfc_dn0 = assign6470_body32_e7200_d_n0;
            locals.var_ffdtcfc_dn1 = assign6470_body32_e7200_d_n1;
            locals.var_ffdtcfc_dn3 = assign6470_body32_e7200_d_n3;
            locals.var_ffdtcfc_dn4 = assign6470_body32_e7200_d_n4;
            locals.var_ffdtcfc_dn5 = assign6470_body32_e7200_d_n5;
            locals.var_ffdtcfc_dn6 = assign6470_body32_e7200_d_n6;
            locals.var_ffdtcfc_dn7 = assign6470_body32_e7200_d_n7;
            locals.var_ffdtcfc_dn8 = assign6470_body32_e7200_d_n8;
            locals.var_ffdtcfc_dn9 = assign6470_body32_e7200_d_n9;
            locals.var_ffdtcfc_rv = 0.0;
            let (assign6470_body33_e7211, assign6470_body33_e7211_d_n0, assign6470_body33_e7211_d_n1, assign6470_body33_e7211_d_n3, assign6470_body33_e7211_d_n4, assign6470_body33_e7211_d_n5, assign6470_body33_e7211_d_n6, assign6470_body33_e7211_d_n7, assign6470_body33_e7211_d_n8, assign6470_body33_e7211_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) {
        let assign6470_body33_e7209: f64 = (1.0 - locals.var_ffw);
        (assign6470_body33_e7209, (-locals.var_ffw_dn0), (-locals.var_ffw_dn1), (-locals.var_ffw_dn3), (-locals.var_ffw_dn4), (-locals.var_ffw_dn5), (-locals.var_ffw_dn6), (-locals.var_ffw_dn7), (-locals.var_ffw_dn8), (-locals.var_ffw_dn9),)
    } else {
        (locals.var_fcick, locals.var_fcick_dn0, locals.var_fcick_dn1, locals.var_fcick_dn3, locals.var_fcick_dn4, locals.var_fcick_dn5, locals.var_fcick_dn6, locals.var_fcick_dn7, locals.var_fcick_dn8, locals.var_fcick_dn9,)
    }
};
            locals.var_fcick = assign6470_body33_e7211;
            locals.var_fcick_dn0 = assign6470_body33_e7211_d_n0;
            locals.var_fcick_dn1 = assign6470_body33_e7211_d_n1;
            locals.var_fcick_dn3 = assign6470_body33_e7211_d_n3;
            locals.var_fcick_dn4 = assign6470_body33_e7211_d_n4;
            locals.var_fcick_dn5 = assign6470_body33_e7211_d_n5;
            locals.var_fcick_dn6 = assign6470_body33_e7211_d_n6;
            locals.var_fcick_dn7 = assign6470_body33_e7211_d_n7;
            locals.var_fcick_dn8 = assign6470_body33_e7211_d_n8;
            locals.var_fcick_dn9 = assign6470_body33_e7211_d_n9;
            locals.var_fcick_rv = 0.0;
            let (assign6470_body34_e7235, assign6470_body34_e7235_d_n0, assign6470_body34_e7235_d_n1, assign6470_body34_e7235_d_n3, assign6470_body34_e7235_d_n4, assign6470_body34_e7235_d_n5, assign6470_body34_e7235_d_n6, assign6470_body34_e7235_d_n7, assign6470_body34_e7235_d_n8, assign6470_body34_e7235_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) {
        let assign6470_body34_e7220: f64 = (locals.var_fcick - 1.0);
        let assign6470_body34_e7223: f64 = (1.0 - locals.var_ffic);
        let assign6470_body34_e7224: f64 = (assign6470_body34_e7220 * assign6470_body34_e7223);
        let assign6470_body34_e7227: f64 = (locals.var_ffic * locals.var_ffic);
        let assign6470_body34_e7229: f64 = (assign6470_body34_e7227 + p.p72);
        let assign6470_body34_e7230: f64 = (assign6470_body34_e7229).sqrt();
        let assign6470_body34_e7232: f64 = (assign6470_body34_e7230 * locals.var_itf);
        let assign6470_body34_e7233: f64 = (assign6470_body34_e7224 / assign6470_body34_e7232);
        (assign6470_body34_e7233, (((((locals.var_fcick_dn0 * assign6470_body34_e7223) + (assign6470_body34_e7220 * (-locals.var_ffic_dn0))) * assign6470_body34_e7232) - (assign6470_body34_e7224 * (((((locals.var_ffic_dn0 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn0)) / (2.0 * assign6470_body34_e7230)) * locals.var_itf) + (assign6470_body34_e7230 * locals.var_itf_dn0)))) / (assign6470_body34_e7232 * assign6470_body34_e7232)), (((((locals.var_fcick_dn1 * assign6470_body34_e7223) + (assign6470_body34_e7220 * (-locals.var_ffic_dn1))) * assign6470_body34_e7232) - (assign6470_body34_e7224 * (((((locals.var_ffic_dn1 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn1)) / (2.0 * assign6470_body34_e7230)) * locals.var_itf) + (assign6470_body34_e7230 * locals.var_itf_dn1)))) / (assign6470_body34_e7232 * assign6470_body34_e7232)), (((((locals.var_fcick_dn3 * assign6470_body34_e7223) + (assign6470_body34_e7220 * (-locals.var_ffic_dn3))) * assign6470_body34_e7232) - (assign6470_body34_e7224 * (((((locals.var_ffic_dn3 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn3)) / (2.0 * assign6470_body34_e7230)) * locals.var_itf) + (assign6470_body34_e7230 * locals.var_itf_dn3)))) / (assign6470_body34_e7232 * assign6470_body34_e7232)), (((((locals.var_fcick_dn4 * assign6470_body34_e7223) + (assign6470_body34_e7220 * (-locals.var_ffic_dn4))) * assign6470_body34_e7232) - (assign6470_body34_e7224 * (((((locals.var_ffic_dn4 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn4)) / (2.0 * assign6470_body34_e7230)) * locals.var_itf) + (assign6470_body34_e7230 * locals.var_itf_dn4)))) / (assign6470_body34_e7232 * assign6470_body34_e7232)), (((((locals.var_fcick_dn5 * assign6470_body34_e7223) + (assign6470_body34_e7220 * (-locals.var_ffic_dn5))) * assign6470_body34_e7232) - (assign6470_body34_e7224 * (((((locals.var_ffic_dn5 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn5)) / (2.0 * assign6470_body34_e7230)) * locals.var_itf) + (assign6470_body34_e7230 * locals.var_itf_dn5)))) / (assign6470_body34_e7232 * assign6470_body34_e7232)), (((((locals.var_fcick_dn6 * assign6470_body34_e7223) + (assign6470_body34_e7220 * (-locals.var_ffic_dn6))) * assign6470_body34_e7232) - (assign6470_body34_e7224 * (((((locals.var_ffic_dn6 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn6)) / (2.0 * assign6470_body34_e7230)) * locals.var_itf) + (assign6470_body34_e7230 * locals.var_itf_dn6)))) / (assign6470_body34_e7232 * assign6470_body34_e7232)), (((((locals.var_fcick_dn7 * assign6470_body34_e7223) + (assign6470_body34_e7220 * (-locals.var_ffic_dn7))) * assign6470_body34_e7232) - (assign6470_body34_e7224 * (((((locals.var_ffic_dn7 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn7)) / (2.0 * assign6470_body34_e7230)) * locals.var_itf) + (assign6470_body34_e7230 * locals.var_itf_dn7)))) / (assign6470_body34_e7232 * assign6470_body34_e7232)), (((((locals.var_fcick_dn8 * assign6470_body34_e7223) + (assign6470_body34_e7220 * (-locals.var_ffic_dn8))) * assign6470_body34_e7232) - (assign6470_body34_e7224 * (((((locals.var_ffic_dn8 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn8)) / (2.0 * assign6470_body34_e7230)) * locals.var_itf) + (assign6470_body34_e7230 * locals.var_itf_dn8)))) / (assign6470_body34_e7232 * assign6470_body34_e7232)), (((((locals.var_fcick_dn9 * assign6470_body34_e7223) + (assign6470_body34_e7220 * (-locals.var_ffic_dn9))) * assign6470_body34_e7232) - (assign6470_body34_e7224 * (((((locals.var_ffic_dn9 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn9)) / (2.0 * assign6470_body34_e7230)) * locals.var_itf) + (assign6470_body34_e7230 * locals.var_itf_dn9)))) / (assign6470_body34_e7232 * assign6470_body34_e7232)),)
    } else {
        (locals.var_fcdick_ditf, locals.var_fcdick_ditf_dn0, locals.var_fcdick_ditf_dn1, locals.var_fcdick_ditf_dn3, locals.var_fcdick_ditf_dn4, locals.var_fcdick_ditf_dn5, locals.var_fcdick_ditf_dn6, locals.var_fcdick_ditf_dn7, locals.var_fcdick_ditf_dn8, locals.var_fcdick_ditf_dn9,)
    }
};
            locals.var_fcdick_ditf = assign6470_body34_e7235;
            locals.var_fcdick_ditf_dn0 = assign6470_body34_e7235_d_n0;
            locals.var_fcdick_ditf_dn1 = assign6470_body34_e7235_d_n1;
            locals.var_fcdick_ditf_dn3 = assign6470_body34_e7235_d_n3;
            locals.var_fcdick_ditf_dn4 = assign6470_body34_e7235_d_n4;
            locals.var_fcdick_ditf_dn5 = assign6470_body34_e7235_d_n5;
            locals.var_fcdick_ditf_dn6 = assign6470_body34_e7235_d_n6;
            locals.var_fcdick_ditf_dn7 = assign6470_body34_e7235_d_n7;
            locals.var_fcdick_ditf_dn8 = assign6470_body34_e7235_d_n8;
            locals.var_fcdick_ditf_dn9 = assign6470_body34_e7235_d_n9;
            locals.var_fcdick_ditf_rv = 0.0;
            let assign6470_body35_e7237: f64 = (locals.var_lat_delta).abs();
            let assign6470_body35_e7239: f64 = if assign6470_body35_e7237 > 0.001 { 1.0 } else { 0.0 };
            locals.var_guard138 = assign6470_body35_e7239;
            locals.var_guard138_rv = 0.0;
            let (assign6470_body36_e7255, assign6470_body36_e7255_d_n0, assign6470_body36_e7255_d_n1, assign6470_body36_e7255_d_n3, assign6470_body36_e7255_d_n4, assign6470_body36_e7255_d_n5, assign6470_body36_e7255_d_n6, assign6470_body36_e7255_d_n7, assign6470_body36_e7255_d_n8, assign6470_body36_e7255_d_n9,) = {
    if ((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 != 0.0)) {
        let assign6470_body36_e7250: f64 = (locals.var_fcick - 1.0);
        let assign6470_body36_e7252: f64 = (assign6470_body36_e7250 * locals.var_ln_lat);
        let assign6470_body36_e7253: f64 = (assign6470_body36_e7252).exp();
        (assign6470_body36_e7253, (assign6470_body36_e7253 * (locals.var_fcick_dn0 * locals.var_ln_lat)), (assign6470_body36_e7253 * (locals.var_fcick_dn1 * locals.var_ln_lat)), (assign6470_body36_e7253 * (locals.var_fcick_dn3 * locals.var_ln_lat)), (assign6470_body36_e7253 * (locals.var_fcick_dn4 * locals.var_ln_lat)), (assign6470_body36_e7253 * (locals.var_fcick_dn5 * locals.var_ln_lat)), (assign6470_body36_e7253 * (locals.var_fcick_dn6 * locals.var_ln_lat)), (assign6470_body36_e7253 * (locals.var_fcick_dn7 * locals.var_ln_lat)), (assign6470_body36_e7253 * (locals.var_fcick_dn8 * locals.var_ln_lat)), (assign6470_body36_e7253 * (locals.var_fcick_dn9 * locals.var_ln_lat)),)
    } else {
        (locals.var_fck, locals.var_fck_dn0, locals.var_fck_dn1, locals.var_fck_dn3, locals.var_fck_dn4, locals.var_fck_dn5, locals.var_fck_dn6, locals.var_fck_dn7, locals.var_fck_dn8, locals.var_fck_dn9,)
    }
};
            locals.var_fck = assign6470_body36_e7255;
            locals.var_fck_dn0 = assign6470_body36_e7255_d_n0;
            locals.var_fck_dn1 = assign6470_body36_e7255_d_n1;
            locals.var_fck_dn3 = assign6470_body36_e7255_d_n3;
            locals.var_fck_dn4 = assign6470_body36_e7255_d_n4;
            locals.var_fck_dn5 = assign6470_body36_e7255_d_n5;
            locals.var_fck_dn6 = assign6470_body36_e7255_d_n6;
            locals.var_fck_dn7 = assign6470_body36_e7255_d_n7;
            locals.var_fck_dn8 = assign6470_body36_e7255_d_n8;
            locals.var_fck_dn9 = assign6470_body36_e7255_d_n9;
            locals.var_fck_rv = 0.0;
            let assign6470_body37_e7258: f64 = if locals.var_latmin < 0.01 { 1.0 } else { 0.0 };
            locals.var_guard139 = assign6470_body37_e7258;
            locals.var_guard139_rv = 0.0;
            let (assign6470_body38_e7277, assign6470_body38_e7277_d_n0, assign6470_body38_e7277_d_n1, assign6470_body38_e7277_d_n3, assign6470_body38_e7277_d_n4, assign6470_body38_e7277_d_n5, assign6470_body38_e7277_d_n6, assign6470_body38_e7277_d_n7, assign6470_body38_e7277_d_n8, assign6470_body38_e7277_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard139 != 0.0)) {
        let assign6470_body38_e7271: f64 = (1.0 - locals.var_fck);
        let assign6470_body38_e7274: f64 = (locals.var_fck * locals.var_latmax);
        let assign6470_body38_e7275: f64 = (assign6470_body38_e7271 / assign6470_body38_e7274);
        (assign6470_body38_e7275, ((((-locals.var_fck_dn0) * assign6470_body38_e7274) - (assign6470_body38_e7271 * (locals.var_fck_dn0 * locals.var_latmax))) / (assign6470_body38_e7274 * assign6470_body38_e7274)), ((((-locals.var_fck_dn1) * assign6470_body38_e7274) - (assign6470_body38_e7271 * (locals.var_fck_dn1 * locals.var_latmax))) / (assign6470_body38_e7274 * assign6470_body38_e7274)), ((((-locals.var_fck_dn3) * assign6470_body38_e7274) - (assign6470_body38_e7271 * (locals.var_fck_dn3 * locals.var_latmax))) / (assign6470_body38_e7274 * assign6470_body38_e7274)), ((((-locals.var_fck_dn4) * assign6470_body38_e7274) - (assign6470_body38_e7271 * (locals.var_fck_dn4 * locals.var_latmax))) / (assign6470_body38_e7274 * assign6470_body38_e7274)), ((((-locals.var_fck_dn5) * assign6470_body38_e7274) - (assign6470_body38_e7271 * (locals.var_fck_dn5 * locals.var_latmax))) / (assign6470_body38_e7274 * assign6470_body38_e7274)), ((((-locals.var_fck_dn6) * assign6470_body38_e7274) - (assign6470_body38_e7271 * (locals.var_fck_dn6 * locals.var_latmax))) / (assign6470_body38_e7274 * assign6470_body38_e7274)), ((((-locals.var_fck_dn7) * assign6470_body38_e7274) - (assign6470_body38_e7271 * (locals.var_fck_dn7 * locals.var_latmax))) / (assign6470_body38_e7274 * assign6470_body38_e7274)), ((((-locals.var_fck_dn8) * assign6470_body38_e7274) - (assign6470_body38_e7271 * (locals.var_fck_dn8 * locals.var_latmax))) / (assign6470_body38_e7274 * assign6470_body38_e7274)), ((((-locals.var_fck_dn9) * assign6470_body38_e7274) - (assign6470_body38_e7271 * (locals.var_fck_dn9 * locals.var_latmax))) / (assign6470_body38_e7274 * assign6470_body38_e7274)),)
    } else {
        (locals.var_fcw, locals.var_fcw_dn0, locals.var_fcw_dn1, locals.var_fcw_dn3, locals.var_fcw_dn4, locals.var_fcw_dn5, locals.var_fcw_dn6, locals.var_fcw_dn7, locals.var_fcw_dn8, locals.var_fcw_dn9,)
    }
};
            locals.var_fcw = assign6470_body38_e7277;
            locals.var_fcw_dn0 = assign6470_body38_e7277_d_n0;
            locals.var_fcw_dn1 = assign6470_body38_e7277_d_n1;
            locals.var_fcw_dn3 = assign6470_body38_e7277_d_n3;
            locals.var_fcw_dn4 = assign6470_body38_e7277_d_n4;
            locals.var_fcw_dn5 = assign6470_body38_e7277_d_n5;
            locals.var_fcw_dn6 = assign6470_body38_e7277_d_n6;
            locals.var_fcw_dn7 = assign6470_body38_e7277_d_n7;
            locals.var_fcw_dn8 = assign6470_body38_e7277_d_n8;
            locals.var_fcw_dn9 = assign6470_body38_e7277_d_n9;
            locals.var_fcw_rv = 0.0;
            let (assign6470_body39_e7294, assign6470_body39_e7294_d_n0, assign6470_body39_e7294_d_n1, assign6470_body39_e7294_d_n3, assign6470_body39_e7294_d_n4, assign6470_body39_e7294_d_n5, assign6470_body39_e7294_d_n6, assign6470_body39_e7294_d_n7, assign6470_body39_e7294_d_n8, assign6470_body39_e7294_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard139 != 0.0)) {
        let assign6470_body39_e7291: f64 = (locals.var_latmax * locals.var_fcw);
        let assign6470_body39_e7292: f64 = (1.0 + assign6470_body39_e7291);
        (assign6470_body39_e7292, (locals.var_latmax * locals.var_fcw_dn0), (locals.var_latmax * locals.var_fcw_dn1), (locals.var_latmax * locals.var_fcw_dn3), (locals.var_latmax * locals.var_fcw_dn4), (locals.var_latmax * locals.var_fcw_dn5), (locals.var_latmax * locals.var_fcw_dn6), (locals.var_latmax * locals.var_fcw_dn7), (locals.var_latmax * locals.var_fcw_dn8), (locals.var_latmax * locals.var_fcw_dn9),)
    } else {
        (locals.var_fclatw_p1, locals.var_fclatw_p1_dn0, locals.var_fclatw_p1_dn1, locals.var_fclatw_p1_dn3, locals.var_fclatw_p1_dn4, locals.var_fclatw_p1_dn5, locals.var_fclatw_p1_dn6, locals.var_fclatw_p1_dn7, locals.var_fclatw_p1_dn8, locals.var_fclatw_p1_dn9,)
    }
};
            locals.var_fclatw_p1 = assign6470_body39_e7294;
            locals.var_fclatw_p1_dn0 = assign6470_body39_e7294_d_n0;
            locals.var_fclatw_p1_dn1 = assign6470_body39_e7294_d_n1;
            locals.var_fclatw_p1_dn3 = assign6470_body39_e7294_d_n3;
            locals.var_fclatw_p1_dn4 = assign6470_body39_e7294_d_n4;
            locals.var_fclatw_p1_dn5 = assign6470_body39_e7294_d_n5;
            locals.var_fclatw_p1_dn6 = assign6470_body39_e7294_d_n6;
            locals.var_fclatw_p1_dn7 = assign6470_body39_e7294_d_n7;
            locals.var_fclatw_p1_dn8 = assign6470_body39_e7294_d_n8;
            locals.var_fclatw_p1_dn9 = assign6470_body39_e7294_d_n9;
            locals.var_fclatw_p1_rv = 0.0;
            let (assign6470_body40_e7328, assign6470_body40_e7328_d_n0, assign6470_body40_e7328_d_n1, assign6470_body40_e7328_d_n3, assign6470_body40_e7328_d_n4, assign6470_body40_e7328_d_n5, assign6470_body40_e7328_d_n6, assign6470_body40_e7328_d_n7, assign6470_body40_e7328_d_n8, assign6470_body40_e7328_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard139 != 0.0)) {
        let assign6470_body40_e7308: f64 = (locals.var_latmax * locals.var_fcw);
        let assign6470_body40_e7312: f64 = (0.25 * locals.var_latmax);
        let assign6470_body40_e7314: f64 = (assign6470_body40_e7312 * locals.var_fcw);
        let assign6470_body40_e7315: f64 = (0.5 + assign6470_body40_e7314);
        let assign6470_body40_e7316: f64 = (assign6470_body40_e7308 * assign6470_body40_e7315);
        let assign6470_body40_e7319: f64 = (locals.var_fclatw_p1).ln();
        let assign6470_body40_e7320: f64 = (0.5 * assign6470_body40_e7319);
        let assign6470_body40_e7321: f64 = (assign6470_body40_e7316 - assign6470_body40_e7320);
        let assign6470_body40_e7322: f64 = (2.0 * assign6470_body40_e7321);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_latmax;
        let assign6470_body40_e7324: f64 = (assign6470_body40_e7322 * __rspice_inv_cse_0);
        let assign6470_body40_e7326: f64 = (assign6470_body40_e7324 * __rspice_inv_cse_0);
        (assign6470_body40_e7326, (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn0) * assign6470_body40_e7315) + (assign6470_body40_e7308 * (assign6470_body40_e7312 * locals.var_fcw_dn0))) - (0.5 * (locals.var_fclatw_p1_dn0 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn1) * assign6470_body40_e7315) + (assign6470_body40_e7308 * (assign6470_body40_e7312 * locals.var_fcw_dn1))) - (0.5 * (locals.var_fclatw_p1_dn1 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn3) * assign6470_body40_e7315) + (assign6470_body40_e7308 * (assign6470_body40_e7312 * locals.var_fcw_dn3))) - (0.5 * (locals.var_fclatw_p1_dn3 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn4) * assign6470_body40_e7315) + (assign6470_body40_e7308 * (assign6470_body40_e7312 * locals.var_fcw_dn4))) - (0.5 * (locals.var_fclatw_p1_dn4 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn5) * assign6470_body40_e7315) + (assign6470_body40_e7308 * (assign6470_body40_e7312 * locals.var_fcw_dn5))) - (0.5 * (locals.var_fclatw_p1_dn5 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn6) * assign6470_body40_e7315) + (assign6470_body40_e7308 * (assign6470_body40_e7312 * locals.var_fcw_dn6))) - (0.5 * (locals.var_fclatw_p1_dn6 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn7) * assign6470_body40_e7315) + (assign6470_body40_e7308 * (assign6470_body40_e7312 * locals.var_fcw_dn7))) - (0.5 * (locals.var_fclatw_p1_dn7 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn8) * assign6470_body40_e7315) + (assign6470_body40_e7308 * (assign6470_body40_e7312 * locals.var_fcw_dn8))) - (0.5 * (locals.var_fclatw_p1_dn8 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn9) * assign6470_body40_e7315) + (assign6470_body40_e7308 * (assign6470_body40_e7312 * locals.var_fcw_dn9))) - (0.5 * (locals.var_fclatw_p1_dn9 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax),)
    } else {
        (locals.var_fcf_ci, locals.var_fcf_ci_dn0, locals.var_fcf_ci_dn1, locals.var_fcf_ci_dn3, locals.var_fcf_ci_dn4, locals.var_fcf_ci_dn5, locals.var_fcf_ci_dn6, locals.var_fcf_ci_dn7, locals.var_fcf_ci_dn8, locals.var_fcf_ci_dn9,)
    }
};
            locals.var_fcf_ci = assign6470_body40_e7328;
            locals.var_fcf_ci_dn0 = assign6470_body40_e7328_d_n0;
            locals.var_fcf_ci_dn1 = assign6470_body40_e7328_d_n1;
            locals.var_fcf_ci_dn3 = assign6470_body40_e7328_d_n3;
            locals.var_fcf_ci_dn4 = assign6470_body40_e7328_d_n4;
            locals.var_fcf_ci_dn5 = assign6470_body40_e7328_d_n5;
            locals.var_fcf_ci_dn6 = assign6470_body40_e7328_d_n6;
            locals.var_fcf_ci_dn7 = assign6470_body40_e7328_d_n7;
            locals.var_fcf_ci_dn8 = assign6470_body40_e7328_d_n8;
            locals.var_fcf_ci_dn9 = assign6470_body40_e7328_d_n9;
            locals.var_fcf_ci_rv = 0.0;
            let (assign6470_body41_e7348, assign6470_body41_e7348_d_n0, assign6470_body41_e7348_d_n1, assign6470_body41_e7348_d_n3, assign6470_body41_e7348_d_n4, assign6470_body41_e7348_d_n5, assign6470_body41_e7348_d_n6, assign6470_body41_e7348_d_n7, assign6470_body41_e7348_d_n8, assign6470_body41_e7348_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard139 != 0.0)) {
        let assign6470_body41_e7340: f64 = (-locals.var_ln_lat);
        let assign6470_body41_e7342: f64 = (assign6470_body41_e7340 * locals.var_fcdick_ditf);
        let assign6470_body41_e7345: f64 = (locals.var_fck * locals.var_latmax);
        let assign6470_body41_e7346: f64 = (assign6470_body41_e7342 / assign6470_body41_e7345);
        (assign6470_body41_e7346, ((((assign6470_body41_e7340 * locals.var_fcdick_ditf_dn0) * assign6470_body41_e7345) - (assign6470_body41_e7342 * (locals.var_fck_dn0 * locals.var_latmax))) / (assign6470_body41_e7345 * assign6470_body41_e7345)), ((((assign6470_body41_e7340 * locals.var_fcdick_ditf_dn1) * assign6470_body41_e7345) - (assign6470_body41_e7342 * (locals.var_fck_dn1 * locals.var_latmax))) / (assign6470_body41_e7345 * assign6470_body41_e7345)), ((((assign6470_body41_e7340 * locals.var_fcdick_ditf_dn3) * assign6470_body41_e7345) - (assign6470_body41_e7342 * (locals.var_fck_dn3 * locals.var_latmax))) / (assign6470_body41_e7345 * assign6470_body41_e7345)), ((((assign6470_body41_e7340 * locals.var_fcdick_ditf_dn4) * assign6470_body41_e7345) - (assign6470_body41_e7342 * (locals.var_fck_dn4 * locals.var_latmax))) / (assign6470_body41_e7345 * assign6470_body41_e7345)), ((((assign6470_body41_e7340 * locals.var_fcdick_ditf_dn5) * assign6470_body41_e7345) - (assign6470_body41_e7342 * (locals.var_fck_dn5 * locals.var_latmax))) / (assign6470_body41_e7345 * assign6470_body41_e7345)), ((((assign6470_body41_e7340 * locals.var_fcdick_ditf_dn6) * assign6470_body41_e7345) - (assign6470_body41_e7342 * (locals.var_fck_dn6 * locals.var_latmax))) / (assign6470_body41_e7345 * assign6470_body41_e7345)), ((((assign6470_body41_e7340 * locals.var_fcdick_ditf_dn7) * assign6470_body41_e7345) - (assign6470_body41_e7342 * (locals.var_fck_dn7 * locals.var_latmax))) / (assign6470_body41_e7345 * assign6470_body41_e7345)), ((((assign6470_body41_e7340 * locals.var_fcdick_ditf_dn8) * assign6470_body41_e7345) - (assign6470_body41_e7342 * (locals.var_fck_dn8 * locals.var_latmax))) / (assign6470_body41_e7345 * assign6470_body41_e7345)), ((((assign6470_body41_e7340 * locals.var_fcdick_ditf_dn9) * assign6470_body41_e7345) - (assign6470_body41_e7342 * (locals.var_fck_dn9 * locals.var_latmax))) / (assign6470_body41_e7345 * assign6470_body41_e7345)),)
    } else {
        (locals.var_fcdw_ditf, locals.var_fcdw_ditf_dn0, locals.var_fcdw_ditf_dn1, locals.var_fcdw_ditf_dn3, locals.var_fcdw_ditf_dn4, locals.var_fcdw_ditf_dn5, locals.var_fcdw_ditf_dn6, locals.var_fcdw_ditf_dn7, locals.var_fcdw_ditf_dn8, locals.var_fcdw_ditf_dn9,)
    }
};
            locals.var_fcdw_ditf = assign6470_body41_e7348;
            locals.var_fcdw_ditf_dn0 = assign6470_body41_e7348_d_n0;
            locals.var_fcdw_ditf_dn1 = assign6470_body41_e7348_d_n1;
            locals.var_fcdw_ditf_dn3 = assign6470_body41_e7348_d_n3;
            locals.var_fcdw_ditf_dn4 = assign6470_body41_e7348_d_n4;
            locals.var_fcdw_ditf_dn5 = assign6470_body41_e7348_d_n5;
            locals.var_fcdw_ditf_dn6 = assign6470_body41_e7348_d_n6;
            locals.var_fcdw_ditf_dn7 = assign6470_body41_e7348_d_n7;
            locals.var_fcdw_ditf_dn8 = assign6470_body41_e7348_d_n8;
            locals.var_fcdw_ditf_dn9 = assign6470_body41_e7348_d_n9;
            locals.var_fcdw_ditf_rv = 0.0;
            let (assign6470_body42_e7369, assign6470_body42_e7369_d_n0, assign6470_body42_e7369_d_n1, assign6470_body42_e7369_d_n3, assign6470_body42_e7369_d_n4, assign6470_body42_e7369_d_n5, assign6470_body42_e7369_d_n6, assign6470_body42_e7369_d_n7, assign6470_body42_e7369_d_n8, assign6470_body42_e7369_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard139 != 0.0)) {
        let assign6470_body42_e7361: f64 = (1.0 + locals.var_fclatw_p1);
        let assign6470_body42_e7363: f64 = (assign6470_body42_e7361 * locals.var_fcw);
        let assign6470_body42_e7365: f64 = (assign6470_body42_e7363 * locals.var_fcdw_ditf);
        let assign6470_body42_e7367: f64 = (assign6470_body42_e7365 / locals.var_fclatw_p1);
        (assign6470_body42_e7367, (((((((locals.var_fclatw_p1_dn0 * locals.var_fcw) + (assign6470_body42_e7361 * locals.var_fcw_dn0)) * locals.var_fcdw_ditf) + (assign6470_body42_e7363 * locals.var_fcdw_ditf_dn0)) * locals.var_fclatw_p1) - (assign6470_body42_e7365 * locals.var_fclatw_p1_dn0)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn1 * locals.var_fcw) + (assign6470_body42_e7361 * locals.var_fcw_dn1)) * locals.var_fcdw_ditf) + (assign6470_body42_e7363 * locals.var_fcdw_ditf_dn1)) * locals.var_fclatw_p1) - (assign6470_body42_e7365 * locals.var_fclatw_p1_dn1)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn3 * locals.var_fcw) + (assign6470_body42_e7361 * locals.var_fcw_dn3)) * locals.var_fcdw_ditf) + (assign6470_body42_e7363 * locals.var_fcdw_ditf_dn3)) * locals.var_fclatw_p1) - (assign6470_body42_e7365 * locals.var_fclatw_p1_dn3)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn4 * locals.var_fcw) + (assign6470_body42_e7361 * locals.var_fcw_dn4)) * locals.var_fcdw_ditf) + (assign6470_body42_e7363 * locals.var_fcdw_ditf_dn4)) * locals.var_fclatw_p1) - (assign6470_body42_e7365 * locals.var_fclatw_p1_dn4)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn5 * locals.var_fcw) + (assign6470_body42_e7361 * locals.var_fcw_dn5)) * locals.var_fcdw_ditf) + (assign6470_body42_e7363 * locals.var_fcdw_ditf_dn5)) * locals.var_fclatw_p1) - (assign6470_body42_e7365 * locals.var_fclatw_p1_dn5)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn6 * locals.var_fcw) + (assign6470_body42_e7361 * locals.var_fcw_dn6)) * locals.var_fcdw_ditf) + (assign6470_body42_e7363 * locals.var_fcdw_ditf_dn6)) * locals.var_fclatw_p1) - (assign6470_body42_e7365 * locals.var_fclatw_p1_dn6)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn7 * locals.var_fcw) + (assign6470_body42_e7361 * locals.var_fcw_dn7)) * locals.var_fcdw_ditf) + (assign6470_body42_e7363 * locals.var_fcdw_ditf_dn7)) * locals.var_fclatw_p1) - (assign6470_body42_e7365 * locals.var_fclatw_p1_dn7)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn8 * locals.var_fcw) + (assign6470_body42_e7361 * locals.var_fcw_dn8)) * locals.var_fcdw_ditf) + (assign6470_body42_e7363 * locals.var_fcdw_ditf_dn8)) * locals.var_fclatw_p1) - (assign6470_body42_e7365 * locals.var_fclatw_p1_dn8)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn9 * locals.var_fcw) + (assign6470_body42_e7361 * locals.var_fcw_dn9)) * locals.var_fcdw_ditf) + (assign6470_body42_e7363 * locals.var_fcdw_ditf_dn9)) * locals.var_fclatw_p1) - (assign6470_body42_e7365 * locals.var_fclatw_p1_dn9)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)),)
    } else {
        (locals.var_fcdfc_ditf, locals.var_fcdfc_ditf_dn0, locals.var_fcdfc_ditf_dn1, locals.var_fcdfc_ditf_dn3, locals.var_fcdfc_ditf_dn4, locals.var_fcdfc_ditf_dn5, locals.var_fcdfc_ditf_dn6, locals.var_fcdfc_ditf_dn7, locals.var_fcdfc_ditf_dn8, locals.var_fcdfc_ditf_dn9,)
    }
};
            locals.var_fcdfc_ditf = assign6470_body42_e7369;
            locals.var_fcdfc_ditf_dn0 = assign6470_body42_e7369_d_n0;
            locals.var_fcdfc_ditf_dn1 = assign6470_body42_e7369_d_n1;
            locals.var_fcdfc_ditf_dn3 = assign6470_body42_e7369_d_n3;
            locals.var_fcdfc_ditf_dn4 = assign6470_body42_e7369_d_n4;
            locals.var_fcdfc_ditf_dn5 = assign6470_body42_e7369_d_n5;
            locals.var_fcdfc_ditf_dn6 = assign6470_body42_e7369_d_n6;
            locals.var_fcdfc_ditf_dn7 = assign6470_body42_e7369_d_n7;
            locals.var_fcdfc_ditf_dn8 = assign6470_body42_e7369_d_n8;
            locals.var_fcdfc_ditf_dn9 = assign6470_body42_e7369_d_n9;
            locals.var_fcdfc_ditf_rv = 0.0;
            let (assign6470_body43_e7387, assign6470_body43_e7387_d_n0, assign6470_body43_e7387_d_n1, assign6470_body43_e7387_d_n3, assign6470_body43_e7387_d_n4, assign6470_body43_e7387_d_n5, assign6470_body43_e7387_d_n6, assign6470_body43_e7387_d_n7, assign6470_body43_e7387_d_n8, assign6470_body43_e7387_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard139 == 0.0)) {
        let assign6470_body43_e7384: f64 = (locals.var_fck * p.p115);
        let assign6470_body43_e7385: f64 = (p.p116 - assign6470_body43_e7384);
        (assign6470_body43_e7385, (-(locals.var_fck_dn0 * p.p115)), (-(locals.var_fck_dn1 * p.p115)), (-(locals.var_fck_dn3 * p.p115)), (-(locals.var_fck_dn4 * p.p115)), (-(locals.var_fck_dn5 * p.p115)), (-(locals.var_fck_dn6 * p.p115)), (-(locals.var_fck_dn7 * p.p115)), (-(locals.var_fck_dn8 * p.p115)), (-(locals.var_fck_dn9 * p.p115)),)
    } else {
        (locals.var_fckdelta, locals.var_fckdelta_dn0, locals.var_fckdelta_dn1, locals.var_fckdelta_dn3, locals.var_fckdelta_dn4, locals.var_fckdelta_dn5, locals.var_fckdelta_dn6, locals.var_fckdelta_dn7, locals.var_fckdelta_dn8, locals.var_fckdelta_dn9,)
    }
};
            locals.var_fckdelta = assign6470_body43_e7387;
            locals.var_fckdelta_dn0 = assign6470_body43_e7387_d_n0;
            locals.var_fckdelta_dn1 = assign6470_body43_e7387_d_n1;
            locals.var_fckdelta_dn3 = assign6470_body43_e7387_d_n3;
            locals.var_fckdelta_dn4 = assign6470_body43_e7387_d_n4;
            locals.var_fckdelta_dn5 = assign6470_body43_e7387_d_n5;
            locals.var_fckdelta_dn6 = assign6470_body43_e7387_d_n6;
            locals.var_fckdelta_dn7 = assign6470_body43_e7387_d_n7;
            locals.var_fckdelta_dn8 = assign6470_body43_e7387_d_n8;
            locals.var_fckdelta_dn9 = assign6470_body43_e7387_d_n9;
            locals.var_fckdelta_rv = 0.0;
            let (assign6470_body44_e7405, assign6470_body44_e7405_d_n0, assign6470_body44_e7405_d_n1, assign6470_body44_e7405_d_n3, assign6470_body44_e7405_d_n4, assign6470_body44_e7405_d_n5, assign6470_body44_e7405_d_n6, assign6470_body44_e7405_d_n7, assign6470_body44_e7405_d_n8, assign6470_body44_e7405_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard139 == 0.0)) {
        let assign6470_body44_e7401: f64 = (locals.var_fck - 1.0);
        let assign6470_body44_e7403: f64 = (assign6470_body44_e7401 / locals.var_fckdelta);
        (assign6470_body44_e7403, (((locals.var_fck_dn0 * locals.var_fckdelta) - (assign6470_body44_e7401 * locals.var_fckdelta_dn0)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn1 * locals.var_fckdelta) - (assign6470_body44_e7401 * locals.var_fckdelta_dn1)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn3 * locals.var_fckdelta) - (assign6470_body44_e7401 * locals.var_fckdelta_dn3)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn4 * locals.var_fckdelta) - (assign6470_body44_e7401 * locals.var_fckdelta_dn4)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn5 * locals.var_fckdelta) - (assign6470_body44_e7401 * locals.var_fckdelta_dn5)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn6 * locals.var_fckdelta) - (assign6470_body44_e7401 * locals.var_fckdelta_dn6)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn7 * locals.var_fckdelta) - (assign6470_body44_e7401 * locals.var_fckdelta_dn7)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn8 * locals.var_fckdelta) - (assign6470_body44_e7401 * locals.var_fckdelta_dn8)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn9 * locals.var_fckdelta) - (assign6470_body44_e7401 * locals.var_fckdelta_dn9)) / (locals.var_fckdelta * locals.var_fckdelta)),)
    } else {
        (locals.var_fcw, locals.var_fcw_dn0, locals.var_fcw_dn1, locals.var_fcw_dn3, locals.var_fcw_dn4, locals.var_fcw_dn5, locals.var_fcw_dn6, locals.var_fcw_dn7, locals.var_fcw_dn8, locals.var_fcw_dn9,)
    }
};
            locals.var_fcw = assign6470_body44_e7405;
            locals.var_fcw_dn0 = assign6470_body44_e7405_d_n0;
            locals.var_fcw_dn1 = assign6470_body44_e7405_d_n1;
            locals.var_fcw_dn3 = assign6470_body44_e7405_d_n3;
            locals.var_fcw_dn4 = assign6470_body44_e7405_d_n4;
            locals.var_fcw_dn5 = assign6470_body44_e7405_d_n5;
            locals.var_fcw_dn6 = assign6470_body44_e7405_d_n6;
            locals.var_fcw_dn7 = assign6470_body44_e7405_d_n7;
            locals.var_fcw_dn8 = assign6470_body44_e7405_d_n8;
            locals.var_fcw_dn9 = assign6470_body44_e7405_d_n9;
            locals.var_fcw_rv = 0.0;
            let (assign6470_body45_e7423, assign6470_body45_e7423_d_n0, assign6470_body45_e7423_d_n1, assign6470_body45_e7423_d_n3, assign6470_body45_e7423_d_n4, assign6470_body45_e7423_d_n5, assign6470_body45_e7423_d_n6, assign6470_body45_e7423_d_n7, assign6470_body45_e7423_d_n8, assign6470_body45_e7423_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard139 == 0.0)) {
        let assign6470_body45_e7420: f64 = (p.p116 * locals.var_fcw);
        let assign6470_body45_e7421: f64 = (1.0 + assign6470_body45_e7420);
        (assign6470_body45_e7421, (p.p116 * locals.var_fcw_dn0), (p.p116 * locals.var_fcw_dn1), (p.p116 * locals.var_fcw_dn3), (p.p116 * locals.var_fcw_dn4), (p.p116 * locals.var_fcw_dn5), (p.p116 * locals.var_fcw_dn6), (p.p116 * locals.var_fcw_dn7), (p.p116 * locals.var_fcw_dn8), (p.p116 * locals.var_fcw_dn9),)
    } else {
        (locals.var_fciwzb_p1, locals.var_fciwzb_p1_dn0, locals.var_fciwzb_p1_dn1, locals.var_fciwzb_p1_dn3, locals.var_fciwzb_p1_dn4, locals.var_fciwzb_p1_dn5, locals.var_fciwzb_p1_dn6, locals.var_fciwzb_p1_dn7, locals.var_fciwzb_p1_dn8, locals.var_fciwzb_p1_dn9,)
    }
};
            locals.var_fciwzb_p1 = assign6470_body45_e7423;
            locals.var_fciwzb_p1_dn0 = assign6470_body45_e7423_d_n0;
            locals.var_fciwzb_p1_dn1 = assign6470_body45_e7423_d_n1;
            locals.var_fciwzb_p1_dn3 = assign6470_body45_e7423_d_n3;
            locals.var_fciwzb_p1_dn4 = assign6470_body45_e7423_d_n4;
            locals.var_fciwzb_p1_dn5 = assign6470_body45_e7423_d_n5;
            locals.var_fciwzb_p1_dn6 = assign6470_body45_e7423_d_n6;
            locals.var_fciwzb_p1_dn7 = assign6470_body45_e7423_d_n7;
            locals.var_fciwzb_p1_dn8 = assign6470_body45_e7423_d_n8;
            locals.var_fciwzb_p1_dn9 = assign6470_body45_e7423_d_n9;
            locals.var_fciwzb_p1_rv = 0.0;
            let (assign6470_body46_e7438, assign6470_body46_e7438_d_n0, assign6470_body46_e7438_d_n1, assign6470_body46_e7438_d_n3, assign6470_body46_e7438_d_n4, assign6470_body46_e7438_d_n5, assign6470_body46_e7438_d_n6, assign6470_body46_e7438_d_n7, assign6470_body46_e7438_d_n8, assign6470_body46_e7438_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard139 == 0.0)) {
        let assign6470_body46_e7436: f64 = (locals.var_fciwzb_p1).ln();
        (assign6470_body46_e7436, (locals.var_fciwzb_p1_dn0 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn1 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn3 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn4 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn5 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn6 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn7 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn8 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn9 / locals.var_fciwzb_p1),)
    } else {
        (locals.var_fcilnw_bl, locals.var_fcilnw_bl_dn0, locals.var_fcilnw_bl_dn1, locals.var_fcilnw_bl_dn3, locals.var_fcilnw_bl_dn4, locals.var_fcilnw_bl_dn5, locals.var_fcilnw_bl_dn6, locals.var_fcilnw_bl_dn7, locals.var_fcilnw_bl_dn8, locals.var_fcilnw_bl_dn9,)
    }
};
            locals.var_fcilnw_bl = assign6470_body46_e7438;
            locals.var_fcilnw_bl_dn0 = assign6470_body46_e7438_d_n0;
            locals.var_fcilnw_bl_dn1 = assign6470_body46_e7438_d_n1;
            locals.var_fcilnw_bl_dn3 = assign6470_body46_e7438_d_n3;
            locals.var_fcilnw_bl_dn4 = assign6470_body46_e7438_d_n4;
            locals.var_fcilnw_bl_dn5 = assign6470_body46_e7438_d_n5;
            locals.var_fcilnw_bl_dn6 = assign6470_body46_e7438_d_n6;
            locals.var_fcilnw_bl_dn7 = assign6470_body46_e7438_d_n7;
            locals.var_fcilnw_bl_dn8 = assign6470_body46_e7438_d_n8;
            locals.var_fcilnw_bl_dn9 = assign6470_body46_e7438_d_n9;
            locals.var_fcilnw_bl_rv = 0.0;
            let (assign6470_body47_e7454,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard139 == 0.0)) {
        let assign6470_body47_e7452: f64 = (locals.var_latb_6 * locals.var_inv_latl);
        (assign6470_body47_e7452,)
    } else {
        (locals.var_fcia,)
    }
};
            locals.var_fcia = assign6470_body47_e7454;
            locals.var_fcia_rv = 0.0;
            let (assign6470_body48_e7482, assign6470_body48_e7482_d_n0, assign6470_body48_e7482_d_n1, assign6470_body48_e7482_d_n3, assign6470_body48_e7482_d_n4, assign6470_body48_e7482_d_n5, assign6470_body48_e7482_d_n6, assign6470_body48_e7482_d_n7, assign6470_body48_e7482_d_n8, assign6470_body48_e7482_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard139 == 0.0)) {
        let assign6470_body48_e7469: f64 = (0.5 - locals.var_fcia);
        let assign6470_body48_e7470: f64 = (locals.var_fcilnw_bl * assign6470_body48_e7469);
        let assign6470_body48_e7472: f64 = (assign6470_body48_e7470 * locals.var_inv_latl);
        let assign6470_body48_e7476: f64 = (locals.var_latb_6 * locals.var_fcw);
        let assign6470_body48_e7477: f64 = (locals.var_fcia + assign6470_body48_e7476);
        let assign6470_body48_e7479: f64 = (assign6470_body48_e7477 * locals.var_fcw);
        let assign6470_body48_e7480: f64 = (assign6470_body48_e7472 + assign6470_body48_e7479);
        (assign6470_body48_e7480, (((locals.var_fcilnw_bl_dn0 * assign6470_body48_e7469) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn0) * locals.var_fcw) + (assign6470_body48_e7477 * locals.var_fcw_dn0))), (((locals.var_fcilnw_bl_dn1 * assign6470_body48_e7469) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn1) * locals.var_fcw) + (assign6470_body48_e7477 * locals.var_fcw_dn1))), (((locals.var_fcilnw_bl_dn3 * assign6470_body48_e7469) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn3) * locals.var_fcw) + (assign6470_body48_e7477 * locals.var_fcw_dn3))), (((locals.var_fcilnw_bl_dn4 * assign6470_body48_e7469) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn4) * locals.var_fcw) + (assign6470_body48_e7477 * locals.var_fcw_dn4))), (((locals.var_fcilnw_bl_dn5 * assign6470_body48_e7469) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn5) * locals.var_fcw) + (assign6470_body48_e7477 * locals.var_fcw_dn5))), (((locals.var_fcilnw_bl_dn6 * assign6470_body48_e7469) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn6) * locals.var_fcw) + (assign6470_body48_e7477 * locals.var_fcw_dn6))), (((locals.var_fcilnw_bl_dn7 * assign6470_body48_e7469) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn7) * locals.var_fcw) + (assign6470_body48_e7477 * locals.var_fcw_dn7))), (((locals.var_fcilnw_bl_dn8 * assign6470_body48_e7469) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn8) * locals.var_fcw) + (assign6470_body48_e7477 * locals.var_fcw_dn8))), (((locals.var_fcilnw_bl_dn9 * assign6470_body48_e7469) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn9) * locals.var_fcw) + (assign6470_body48_e7477 * locals.var_fcw_dn9))),)
    } else {
        (locals.var_fcf_csl, locals.var_fcf_csl_dn0, locals.var_fcf_csl_dn1, locals.var_fcf_csl_dn3, locals.var_fcf_csl_dn4, locals.var_fcf_csl_dn5, locals.var_fcf_csl_dn6, locals.var_fcf_csl_dn7, locals.var_fcf_csl_dn8, locals.var_fcf_csl_dn9,)
    }
};
            locals.var_fcf_csl = assign6470_body48_e7482;
            locals.var_fcf_csl_dn0 = assign6470_body48_e7482_d_n0;
            locals.var_fcf_csl_dn1 = assign6470_body48_e7482_d_n1;
            locals.var_fcf_csl_dn3 = assign6470_body48_e7482_d_n3;
            locals.var_fcf_csl_dn4 = assign6470_body48_e7482_d_n4;
            locals.var_fcf_csl_dn5 = assign6470_body48_e7482_d_n5;
            locals.var_fcf_csl_dn6 = assign6470_body48_e7482_d_n6;
            locals.var_fcf_csl_dn7 = assign6470_body48_e7482_d_n7;
            locals.var_fcf_csl_dn8 = assign6470_body48_e7482_d_n8;
            locals.var_fcf_csl_dn9 = assign6470_body48_e7482_d_n9;
            locals.var_fcf_csl_rv = 0.0;
            let (assign6470_body49_e7508, assign6470_body49_e7508_d_n0, assign6470_body49_e7508_d_n1, assign6470_body49_e7508_d_n3, assign6470_body49_e7508_d_n4, assign6470_body49_e7508_d_n5, assign6470_body49_e7508_d_n6, assign6470_body49_e7508_d_n7, assign6470_body49_e7508_d_n8, assign6470_body49_e7508_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard139 == 0.0)) {
        let assign6470_body49_e7496: f64 = (0.5 - locals.var_fcia);
        let assign6470_body49_e7498: f64 = (assign6470_body49_e7496 / locals.var_fciwzb_p1);
        let assign6470_body49_e7500: f64 = (assign6470_body49_e7498 + locals.var_fcia);
        let assign6470_body49_e7503: f64 = (locals.var_fcw * locals.var_latb_6);
        let assign6470_body49_e7505: f64 = (assign6470_body49_e7503 * 2.0);
        let assign6470_body49_e7506: f64 = (assign6470_body49_e7500 + assign6470_body49_e7505);
        (assign6470_body49_e7506, ((-((assign6470_body49_e7496 * locals.var_fciwzb_p1_dn0) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn0 * locals.var_latb_6) * 2.0)), ((-((assign6470_body49_e7496 * locals.var_fciwzb_p1_dn1) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn1 * locals.var_latb_6) * 2.0)), ((-((assign6470_body49_e7496 * locals.var_fciwzb_p1_dn3) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn3 * locals.var_latb_6) * 2.0)), ((-((assign6470_body49_e7496 * locals.var_fciwzb_p1_dn4) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn4 * locals.var_latb_6) * 2.0)), ((-((assign6470_body49_e7496 * locals.var_fciwzb_p1_dn5) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn5 * locals.var_latb_6) * 2.0)), ((-((assign6470_body49_e7496 * locals.var_fciwzb_p1_dn6) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn6 * locals.var_latb_6) * 2.0)), ((-((assign6470_body49_e7496 * locals.var_fciwzb_p1_dn7) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn7 * locals.var_latb_6) * 2.0)), ((-((assign6470_body49_e7496 * locals.var_fciwzb_p1_dn8) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn8 * locals.var_latb_6) * 2.0)), ((-((assign6470_body49_e7496 * locals.var_fciwzb_p1_dn9) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn9 * locals.var_latb_6) * 2.0)),)
    } else {
        (locals.var_fcdfcsl_dw, locals.var_fcdfcsl_dw_dn0, locals.var_fcdfcsl_dw_dn1, locals.var_fcdfcsl_dw_dn3, locals.var_fcdfcsl_dw_dn4, locals.var_fcdfcsl_dw_dn5, locals.var_fcdfcsl_dw_dn6, locals.var_fcdfcsl_dw_dn7, locals.var_fcdfcsl_dw_dn8, locals.var_fcdfcsl_dw_dn9,)
    }
};
            locals.var_fcdfcsl_dw = assign6470_body49_e7508;
            locals.var_fcdfcsl_dw_dn0 = assign6470_body49_e7508_d_n0;
            locals.var_fcdfcsl_dw_dn1 = assign6470_body49_e7508_d_n1;
            locals.var_fcdfcsl_dw_dn3 = assign6470_body49_e7508_d_n3;
            locals.var_fcdfcsl_dw_dn4 = assign6470_body49_e7508_d_n4;
            locals.var_fcdfcsl_dw_dn5 = assign6470_body49_e7508_d_n5;
            locals.var_fcdfcsl_dw_dn6 = assign6470_body49_e7508_d_n6;
            locals.var_fcdfcsl_dw_dn7 = assign6470_body49_e7508_d_n7;
            locals.var_fcdfcsl_dw_dn8 = assign6470_body49_e7508_d_n8;
            locals.var_fcdfcsl_dw_dn9 = assign6470_body49_e7508_d_n9;
            locals.var_fcdfcsl_dw_rv = 0.0;
            let (assign6470_body50_e7526, assign6470_body50_e7526_d_n0, assign6470_body50_e7526_d_n1, assign6470_body50_e7526_d_n3, assign6470_body50_e7526_d_n4, assign6470_body50_e7526_d_n5, assign6470_body50_e7526_d_n6, assign6470_body50_e7526_d_n7, assign6470_body50_e7526_d_n8, assign6470_body50_e7526_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard139 == 0.0)) {
        let assign6470_body50_e7523: f64 = (p.p115 * locals.var_fcw);
        let assign6470_body50_e7524: f64 = (1.0 + assign6470_body50_e7523);
        (assign6470_body50_e7524, (p.p115 * locals.var_fcw_dn0), (p.p115 * locals.var_fcw_dn1), (p.p115 * locals.var_fcw_dn3), (p.p115 * locals.var_fcw_dn4), (p.p115 * locals.var_fcw_dn5), (p.p115 * locals.var_fcw_dn6), (p.p115 * locals.var_fcw_dn7), (p.p115 * locals.var_fcw_dn8), (p.p115 * locals.var_fcw_dn9),)
    } else {
        (locals.var_fciwzb_p1, locals.var_fciwzb_p1_dn0, locals.var_fciwzb_p1_dn1, locals.var_fciwzb_p1_dn3, locals.var_fciwzb_p1_dn4, locals.var_fciwzb_p1_dn5, locals.var_fciwzb_p1_dn6, locals.var_fciwzb_p1_dn7, locals.var_fciwzb_p1_dn8, locals.var_fciwzb_p1_dn9,)
    }
};
            locals.var_fciwzb_p1 = assign6470_body50_e7526;
            locals.var_fciwzb_p1_dn0 = assign6470_body50_e7526_d_n0;
            locals.var_fciwzb_p1_dn1 = assign6470_body50_e7526_d_n1;
            locals.var_fciwzb_p1_dn3 = assign6470_body50_e7526_d_n3;
            locals.var_fciwzb_p1_dn4 = assign6470_body50_e7526_d_n4;
            locals.var_fciwzb_p1_dn5 = assign6470_body50_e7526_d_n5;
            locals.var_fciwzb_p1_dn6 = assign6470_body50_e7526_d_n6;
            locals.var_fciwzb_p1_dn7 = assign6470_body50_e7526_d_n7;
            locals.var_fciwzb_p1_dn8 = assign6470_body50_e7526_d_n8;
            locals.var_fciwzb_p1_dn9 = assign6470_body50_e7526_d_n9;
            locals.var_fciwzb_p1_rv = 0.0;
            let (assign6470_body51_e7541, assign6470_body51_e7541_d_n0, assign6470_body51_e7541_d_n1, assign6470_body51_e7541_d_n3, assign6470_body51_e7541_d_n4, assign6470_body51_e7541_d_n5, assign6470_body51_e7541_d_n6, assign6470_body51_e7541_d_n7, assign6470_body51_e7541_d_n8, assign6470_body51_e7541_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard139 == 0.0)) {
        let assign6470_body51_e7539: f64 = (locals.var_fciwzb_p1).ln();
        (assign6470_body51_e7539, (locals.var_fciwzb_p1_dn0 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn1 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn3 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn4 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn5 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn6 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn7 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn8 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn9 / locals.var_fciwzb_p1),)
    } else {
        (locals.var_fcilnw_bl, locals.var_fcilnw_bl_dn0, locals.var_fcilnw_bl_dn1, locals.var_fcilnw_bl_dn3, locals.var_fcilnw_bl_dn4, locals.var_fcilnw_bl_dn5, locals.var_fcilnw_bl_dn6, locals.var_fcilnw_bl_dn7, locals.var_fcilnw_bl_dn8, locals.var_fcilnw_bl_dn9,)
    }
};
            locals.var_fcilnw_bl = assign6470_body51_e7541;
            locals.var_fcilnw_bl_dn0 = assign6470_body51_e7541_d_n0;
            locals.var_fcilnw_bl_dn1 = assign6470_body51_e7541_d_n1;
            locals.var_fcilnw_bl_dn3 = assign6470_body51_e7541_d_n3;
            locals.var_fcilnw_bl_dn4 = assign6470_body51_e7541_d_n4;
            locals.var_fcilnw_bl_dn5 = assign6470_body51_e7541_d_n5;
            locals.var_fcilnw_bl_dn6 = assign6470_body51_e7541_d_n6;
            locals.var_fcilnw_bl_dn7 = assign6470_body51_e7541_d_n7;
            locals.var_fcilnw_bl_dn8 = assign6470_body51_e7541_d_n8;
            locals.var_fcilnw_bl_dn9 = assign6470_body51_e7541_d_n9;
            locals.var_fcilnw_bl_rv = 0.0;
            let (assign6470_body52_e7557,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard139 == 0.0)) {
        let assign6470_body52_e7555: f64 = (locals.var_latl_6 * locals.var_inv_latb);
        (assign6470_body52_e7555,)
    } else {
        (locals.var_fcia,)
    }
};
            locals.var_fcia = assign6470_body52_e7557;
            locals.var_fcia_rv = 0.0;
            let (assign6470_body53_e7585, assign6470_body53_e7585_d_n0, assign6470_body53_e7585_d_n1, assign6470_body53_e7585_d_n3, assign6470_body53_e7585_d_n4, assign6470_body53_e7585_d_n5, assign6470_body53_e7585_d_n6, assign6470_body53_e7585_d_n7, assign6470_body53_e7585_d_n8, assign6470_body53_e7585_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard139 == 0.0)) {
        let assign6470_body53_e7572: f64 = (0.5 - locals.var_fcia);
        let assign6470_body53_e7573: f64 = (locals.var_fcilnw_bl * assign6470_body53_e7572);
        let assign6470_body53_e7575: f64 = (assign6470_body53_e7573 * locals.var_inv_latb);
        let assign6470_body53_e7579: f64 = (locals.var_latl_6 * locals.var_fcw);
        let assign6470_body53_e7580: f64 = (locals.var_fcia + assign6470_body53_e7579);
        let assign6470_body53_e7582: f64 = (assign6470_body53_e7580 * locals.var_fcw);
        let assign6470_body53_e7583: f64 = (assign6470_body53_e7575 + assign6470_body53_e7582);
        (assign6470_body53_e7583, (((locals.var_fcilnw_bl_dn0 * assign6470_body53_e7572) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn0) * locals.var_fcw) + (assign6470_body53_e7580 * locals.var_fcw_dn0))), (((locals.var_fcilnw_bl_dn1 * assign6470_body53_e7572) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn1) * locals.var_fcw) + (assign6470_body53_e7580 * locals.var_fcw_dn1))), (((locals.var_fcilnw_bl_dn3 * assign6470_body53_e7572) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn3) * locals.var_fcw) + (assign6470_body53_e7580 * locals.var_fcw_dn3))), (((locals.var_fcilnw_bl_dn4 * assign6470_body53_e7572) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn4) * locals.var_fcw) + (assign6470_body53_e7580 * locals.var_fcw_dn4))), (((locals.var_fcilnw_bl_dn5 * assign6470_body53_e7572) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn5) * locals.var_fcw) + (assign6470_body53_e7580 * locals.var_fcw_dn5))), (((locals.var_fcilnw_bl_dn6 * assign6470_body53_e7572) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn6) * locals.var_fcw) + (assign6470_body53_e7580 * locals.var_fcw_dn6))), (((locals.var_fcilnw_bl_dn7 * assign6470_body53_e7572) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn7) * locals.var_fcw) + (assign6470_body53_e7580 * locals.var_fcw_dn7))), (((locals.var_fcilnw_bl_dn8 * assign6470_body53_e7572) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn8) * locals.var_fcw) + (assign6470_body53_e7580 * locals.var_fcw_dn8))), (((locals.var_fcilnw_bl_dn9 * assign6470_body53_e7572) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn9) * locals.var_fcw) + (assign6470_body53_e7580 * locals.var_fcw_dn9))),)
    } else {
        (locals.var_fcf_csb, locals.var_fcf_csb_dn0, locals.var_fcf_csb_dn1, locals.var_fcf_csb_dn3, locals.var_fcf_csb_dn4, locals.var_fcf_csb_dn5, locals.var_fcf_csb_dn6, locals.var_fcf_csb_dn7, locals.var_fcf_csb_dn8, locals.var_fcf_csb_dn9,)
    }
};
            locals.var_fcf_csb = assign6470_body53_e7585;
            locals.var_fcf_csb_dn0 = assign6470_body53_e7585_d_n0;
            locals.var_fcf_csb_dn1 = assign6470_body53_e7585_d_n1;
            locals.var_fcf_csb_dn3 = assign6470_body53_e7585_d_n3;
            locals.var_fcf_csb_dn4 = assign6470_body53_e7585_d_n4;
            locals.var_fcf_csb_dn5 = assign6470_body53_e7585_d_n5;
            locals.var_fcf_csb_dn6 = assign6470_body53_e7585_d_n6;
            locals.var_fcf_csb_dn7 = assign6470_body53_e7585_d_n7;
            locals.var_fcf_csb_dn8 = assign6470_body53_e7585_d_n8;
            locals.var_fcf_csb_dn9 = assign6470_body53_e7585_d_n9;
            locals.var_fcf_csb_rv = 0.0;
            let (assign6470_body54_e7611, assign6470_body54_e7611_d_n0, assign6470_body54_e7611_d_n1, assign6470_body54_e7611_d_n3, assign6470_body54_e7611_d_n4, assign6470_body54_e7611_d_n5, assign6470_body54_e7611_d_n6, assign6470_body54_e7611_d_n7, assign6470_body54_e7611_d_n8, assign6470_body54_e7611_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard139 == 0.0)) {
        let assign6470_body54_e7599: f64 = (0.5 - locals.var_fcia);
        let assign6470_body54_e7601: f64 = (assign6470_body54_e7599 / locals.var_fciwzb_p1);
        let assign6470_body54_e7603: f64 = (assign6470_body54_e7601 + locals.var_fcia);
        let assign6470_body54_e7606: f64 = (locals.var_fcw * locals.var_latl_6);
        let assign6470_body54_e7608: f64 = (assign6470_body54_e7606 * 2.0);
        let assign6470_body54_e7609: f64 = (assign6470_body54_e7603 + assign6470_body54_e7608);
        (assign6470_body54_e7609, ((-((assign6470_body54_e7599 * locals.var_fciwzb_p1_dn0) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn0 * locals.var_latl_6) * 2.0)), ((-((assign6470_body54_e7599 * locals.var_fciwzb_p1_dn1) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn1 * locals.var_latl_6) * 2.0)), ((-((assign6470_body54_e7599 * locals.var_fciwzb_p1_dn3) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn3 * locals.var_latl_6) * 2.0)), ((-((assign6470_body54_e7599 * locals.var_fciwzb_p1_dn4) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn4 * locals.var_latl_6) * 2.0)), ((-((assign6470_body54_e7599 * locals.var_fciwzb_p1_dn5) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn5 * locals.var_latl_6) * 2.0)), ((-((assign6470_body54_e7599 * locals.var_fciwzb_p1_dn6) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn6 * locals.var_latl_6) * 2.0)), ((-((assign6470_body54_e7599 * locals.var_fciwzb_p1_dn7) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn7 * locals.var_latl_6) * 2.0)), ((-((assign6470_body54_e7599 * locals.var_fciwzb_p1_dn8) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn8 * locals.var_latl_6) * 2.0)), ((-((assign6470_body54_e7599 * locals.var_fciwzb_p1_dn9) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn9 * locals.var_latl_6) * 2.0)),)
    } else {
        (locals.var_fcdfcsb_dw, locals.var_fcdfcsb_dw_dn0, locals.var_fcdfcsb_dw_dn1, locals.var_fcdfcsb_dw_dn3, locals.var_fcdfcsb_dw_dn4, locals.var_fcdfcsb_dw_dn5, locals.var_fcdfcsb_dw_dn6, locals.var_fcdfcsb_dw_dn7, locals.var_fcdfcsb_dw_dn8, locals.var_fcdfcsb_dw_dn9,)
    }
};
            locals.var_fcdfcsb_dw = assign6470_body54_e7611;
            locals.var_fcdfcsb_dw_dn0 = assign6470_body54_e7611_d_n0;
            locals.var_fcdfcsb_dw_dn1 = assign6470_body54_e7611_d_n1;
            locals.var_fcdfcsb_dw_dn3 = assign6470_body54_e7611_d_n3;
            locals.var_fcdfcsb_dw_dn4 = assign6470_body54_e7611_d_n4;
            locals.var_fcdfcsb_dw_dn5 = assign6470_body54_e7611_d_n5;
            locals.var_fcdfcsb_dw_dn6 = assign6470_body54_e7611_d_n6;
            locals.var_fcdfcsb_dw_dn7 = assign6470_body54_e7611_d_n7;
            locals.var_fcdfcsb_dw_dn8 = assign6470_body54_e7611_d_n8;
            locals.var_fcdfcsb_dw_dn9 = assign6470_body54_e7611_d_n9;
            locals.var_fcdfcsb_dw_rv = 0.0;
            let (assign6470_body55_e7629, assign6470_body55_e7629_d_n0, assign6470_body55_e7629_d_n1, assign6470_body55_e7629_d_n3, assign6470_body55_e7629_d_n4, assign6470_body55_e7629_d_n5, assign6470_body55_e7629_d_n6, assign6470_body55_e7629_d_n7, assign6470_body55_e7629_d_n8, assign6470_body55_e7629_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard139 == 0.0)) {
        let assign6470_body55_e7625: f64 = (locals.var_fcf_csl - locals.var_fcf_csb);
        let assign6470_body55_e7627: f64 = (assign6470_body55_e7625 / locals.var_lat_delta);
        (assign6470_body55_e7627, ((locals.var_fcf_csl_dn0 - locals.var_fcf_csb_dn0) / locals.var_lat_delta), ((locals.var_fcf_csl_dn1 - locals.var_fcf_csb_dn1) / locals.var_lat_delta), ((locals.var_fcf_csl_dn3 - locals.var_fcf_csb_dn3) / locals.var_lat_delta), ((locals.var_fcf_csl_dn4 - locals.var_fcf_csb_dn4) / locals.var_lat_delta), ((locals.var_fcf_csl_dn5 - locals.var_fcf_csb_dn5) / locals.var_lat_delta), ((locals.var_fcf_csl_dn6 - locals.var_fcf_csb_dn6) / locals.var_lat_delta), ((locals.var_fcf_csl_dn7 - locals.var_fcf_csb_dn7) / locals.var_lat_delta), ((locals.var_fcf_csl_dn8 - locals.var_fcf_csb_dn8) / locals.var_lat_delta), ((locals.var_fcf_csl_dn9 - locals.var_fcf_csb_dn9) / locals.var_lat_delta),)
    } else {
        (locals.var_fcf_ci, locals.var_fcf_ci_dn0, locals.var_fcf_ci_dn1, locals.var_fcf_ci_dn3, locals.var_fcf_ci_dn4, locals.var_fcf_ci_dn5, locals.var_fcf_ci_dn6, locals.var_fcf_ci_dn7, locals.var_fcf_ci_dn8, locals.var_fcf_ci_dn9,)
    }
};
            locals.var_fcf_ci = assign6470_body55_e7629;
            locals.var_fcf_ci_dn0 = assign6470_body55_e7629_d_n0;
            locals.var_fcf_ci_dn1 = assign6470_body55_e7629_d_n1;
            locals.var_fcf_ci_dn3 = assign6470_body55_e7629_d_n3;
            locals.var_fcf_ci_dn4 = assign6470_body55_e7629_d_n4;
            locals.var_fcf_ci_dn5 = assign6470_body55_e7629_d_n5;
            locals.var_fcf_ci_dn6 = assign6470_body55_e7629_d_n6;
            locals.var_fcf_ci_dn7 = assign6470_body55_e7629_d_n7;
            locals.var_fcf_ci_dn8 = assign6470_body55_e7629_d_n8;
            locals.var_fcf_ci_dn9 = assign6470_body55_e7629_d_n9;
            locals.var_fcf_ci_rv = 0.0;
            let (assign6470_body56_e7656, assign6470_body56_e7656_d_n0, assign6470_body56_e7656_d_n1, assign6470_body56_e7656_d_n3, assign6470_body56_e7656_d_n4, assign6470_body56_e7656_d_n5, assign6470_body56_e7656_d_n6, assign6470_body56_e7656_d_n7, assign6470_body56_e7656_d_n8, assign6470_body56_e7656_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard139 == 0.0)) {
        let assign6470_body56_e7642: f64 = (-2.0);
        let assign6470_body56_e7644: f64 = (assign6470_body56_e7642 * locals.var_lat_delta);
        let assign6470_body56_e7647: f64 = (locals.var_fckdelta * locals.var_fckdelta);
        let assign6470_body56_e7648: f64 = (assign6470_body56_e7644 / assign6470_body56_e7647);
        let assign6470_body56_e7650: f64 = (assign6470_body56_e7648 * locals.var_fck);
        let assign6470_body56_e7652: f64 = (assign6470_body56_e7650 * locals.var_ln_lat);
        let assign6470_body56_e7654: f64 = (assign6470_body56_e7652 * locals.var_fcdick_ditf);
        (assign6470_body56_e7654, ((((((-((assign6470_body56_e7644 * ((locals.var_fckdelta_dn0 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn0))) / (assign6470_body56_e7647 * assign6470_body56_e7647))) * locals.var_fck) + (assign6470_body56_e7648 * locals.var_fck_dn0)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign6470_body56_e7652 * locals.var_fcdick_ditf_dn0)), ((((((-((assign6470_body56_e7644 * ((locals.var_fckdelta_dn1 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn1))) / (assign6470_body56_e7647 * assign6470_body56_e7647))) * locals.var_fck) + (assign6470_body56_e7648 * locals.var_fck_dn1)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign6470_body56_e7652 * locals.var_fcdick_ditf_dn1)), ((((((-((assign6470_body56_e7644 * ((locals.var_fckdelta_dn3 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn3))) / (assign6470_body56_e7647 * assign6470_body56_e7647))) * locals.var_fck) + (assign6470_body56_e7648 * locals.var_fck_dn3)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign6470_body56_e7652 * locals.var_fcdick_ditf_dn3)), ((((((-((assign6470_body56_e7644 * ((locals.var_fckdelta_dn4 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn4))) / (assign6470_body56_e7647 * assign6470_body56_e7647))) * locals.var_fck) + (assign6470_body56_e7648 * locals.var_fck_dn4)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign6470_body56_e7652 * locals.var_fcdick_ditf_dn4)), ((((((-((assign6470_body56_e7644 * ((locals.var_fckdelta_dn5 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn5))) / (assign6470_body56_e7647 * assign6470_body56_e7647))) * locals.var_fck) + (assign6470_body56_e7648 * locals.var_fck_dn5)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign6470_body56_e7652 * locals.var_fcdick_ditf_dn5)), ((((((-((assign6470_body56_e7644 * ((locals.var_fckdelta_dn6 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn6))) / (assign6470_body56_e7647 * assign6470_body56_e7647))) * locals.var_fck) + (assign6470_body56_e7648 * locals.var_fck_dn6)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign6470_body56_e7652 * locals.var_fcdick_ditf_dn6)), ((((((-((assign6470_body56_e7644 * ((locals.var_fckdelta_dn7 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn7))) / (assign6470_body56_e7647 * assign6470_body56_e7647))) * locals.var_fck) + (assign6470_body56_e7648 * locals.var_fck_dn7)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign6470_body56_e7652 * locals.var_fcdick_ditf_dn7)), ((((((-((assign6470_body56_e7644 * ((locals.var_fckdelta_dn8 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn8))) / (assign6470_body56_e7647 * assign6470_body56_e7647))) * locals.var_fck) + (assign6470_body56_e7648 * locals.var_fck_dn8)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign6470_body56_e7652 * locals.var_fcdick_ditf_dn8)), ((((((-((assign6470_body56_e7644 * ((locals.var_fckdelta_dn9 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn9))) / (assign6470_body56_e7647 * assign6470_body56_e7647))) * locals.var_fck) + (assign6470_body56_e7648 * locals.var_fck_dn9)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign6470_body56_e7652 * locals.var_fcdick_ditf_dn9)),)
    } else {
        (locals.var_fcdw_ditf, locals.var_fcdw_ditf_dn0, locals.var_fcdw_ditf_dn1, locals.var_fcdw_ditf_dn3, locals.var_fcdw_ditf_dn4, locals.var_fcdw_ditf_dn5, locals.var_fcdw_ditf_dn6, locals.var_fcdw_ditf_dn7, locals.var_fcdw_ditf_dn8, locals.var_fcdw_ditf_dn9,)
    }
};
            locals.var_fcdw_ditf = assign6470_body56_e7656;
            locals.var_fcdw_ditf_dn0 = assign6470_body56_e7656_d_n0;
            locals.var_fcdw_ditf_dn1 = assign6470_body56_e7656_d_n1;
            locals.var_fcdw_ditf_dn3 = assign6470_body56_e7656_d_n3;
            locals.var_fcdw_ditf_dn4 = assign6470_body56_e7656_d_n4;
            locals.var_fcdw_ditf_dn5 = assign6470_body56_e7656_d_n5;
            locals.var_fcdw_ditf_dn6 = assign6470_body56_e7656_d_n6;
            locals.var_fcdw_ditf_dn7 = assign6470_body56_e7656_d_n7;
            locals.var_fcdw_ditf_dn8 = assign6470_body56_e7656_d_n8;
            locals.var_fcdw_ditf_dn9 = assign6470_body56_e7656_d_n9;
            locals.var_fcdw_ditf_rv = 0.0;
            let (assign6470_body57_e7676, assign6470_body57_e7676_d_n0, assign6470_body57_e7676_d_n1, assign6470_body57_e7676_d_n3, assign6470_body57_e7676_d_n4, assign6470_body57_e7676_d_n5, assign6470_body57_e7676_d_n6, assign6470_body57_e7676_d_n7, assign6470_body57_e7676_d_n8, assign6470_body57_e7676_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 != 0.0)) && (locals.var_guard139 == 0.0)) {
        let assign6470_body57_e7670: f64 = (locals.var_fcdfcsl_dw - locals.var_fcdfcsb_dw);
        let assign6470_body57_e7672: f64 = (assign6470_body57_e7670 * locals.var_fcdw_ditf);
        let assign6470_body57_e7674: f64 = (assign6470_body57_e7672 / locals.var_lat_delta);
        (assign6470_body57_e7674, ((((locals.var_fcdfcsl_dw_dn0 - locals.var_fcdfcsb_dw_dn0) * locals.var_fcdw_ditf) + (assign6470_body57_e7670 * locals.var_fcdw_ditf_dn0)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn1 - locals.var_fcdfcsb_dw_dn1) * locals.var_fcdw_ditf) + (assign6470_body57_e7670 * locals.var_fcdw_ditf_dn1)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn3 - locals.var_fcdfcsb_dw_dn3) * locals.var_fcdw_ditf) + (assign6470_body57_e7670 * locals.var_fcdw_ditf_dn3)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn4 - locals.var_fcdfcsb_dw_dn4) * locals.var_fcdw_ditf) + (assign6470_body57_e7670 * locals.var_fcdw_ditf_dn4)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn5 - locals.var_fcdfcsb_dw_dn5) * locals.var_fcdw_ditf) + (assign6470_body57_e7670 * locals.var_fcdw_ditf_dn5)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn6 - locals.var_fcdfcsb_dw_dn6) * locals.var_fcdw_ditf) + (assign6470_body57_e7670 * locals.var_fcdw_ditf_dn6)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn7 - locals.var_fcdfcsb_dw_dn7) * locals.var_fcdw_ditf) + (assign6470_body57_e7670 * locals.var_fcdw_ditf_dn7)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn8 - locals.var_fcdfcsb_dw_dn8) * locals.var_fcdw_ditf) + (assign6470_body57_e7670 * locals.var_fcdw_ditf_dn8)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn9 - locals.var_fcdfcsb_dw_dn9) * locals.var_fcdw_ditf) + (assign6470_body57_e7670 * locals.var_fcdw_ditf_dn9)) / locals.var_lat_delta),)
    } else {
        (locals.var_fcdfc_ditf, locals.var_fcdfc_ditf_dn0, locals.var_fcdfc_ditf_dn1, locals.var_fcdfc_ditf_dn3, locals.var_fcdfc_ditf_dn4, locals.var_fcdfc_ditf_dn5, locals.var_fcdfc_ditf_dn6, locals.var_fcdfc_ditf_dn7, locals.var_fcdfc_ditf_dn8, locals.var_fcdfc_ditf_dn9,)
    }
};
            locals.var_fcdfc_ditf = assign6470_body57_e7676;
            locals.var_fcdfc_ditf_dn0 = assign6470_body57_e7676_d_n0;
            locals.var_fcdfc_ditf_dn1 = assign6470_body57_e7676_d_n1;
            locals.var_fcdfc_ditf_dn3 = assign6470_body57_e7676_d_n3;
            locals.var_fcdfc_ditf_dn4 = assign6470_body57_e7676_d_n4;
            locals.var_fcdfc_ditf_dn5 = assign6470_body57_e7676_d_n5;
            locals.var_fcdfc_ditf_dn6 = assign6470_body57_e7676_d_n6;
            locals.var_fcdfc_ditf_dn7 = assign6470_body57_e7676_d_n7;
            locals.var_fcdfc_ditf_dn8 = assign6470_body57_e7676_d_n8;
            locals.var_fcdfc_ditf_dn9 = assign6470_body57_e7676_d_n9;
            locals.var_fcdfc_ditf_rv = 0.0;
            let (assign6470_body58_e7696, assign6470_body58_e7696_d_n0, assign6470_body58_e7696_d_n1, assign6470_body58_e7696_d_n3, assign6470_body58_e7696_d_n4, assign6470_body58_e7696_d_n5, assign6470_body58_e7696_d_n6, assign6470_body58_e7696_d_n7, assign6470_body58_e7696_d_n8, assign6470_body58_e7696_d_n9,) = {
    if ((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 == 0.0)) {
        let assign6470_body58_e7688: f64 = (1.0 - locals.var_fcick);
        let assign6470_body58_e7692: f64 = (locals.var_fcick * p.p115);
        let assign6470_body58_e7693: f64 = (1.0 + assign6470_body58_e7692);
        let assign6470_body58_e7694: f64 = (assign6470_body58_e7688 / assign6470_body58_e7693);
        (assign6470_body58_e7694, ((((-locals.var_fcick_dn0) * assign6470_body58_e7693) - (assign6470_body58_e7688 * (locals.var_fcick_dn0 * p.p115))) / (assign6470_body58_e7693 * assign6470_body58_e7693)), ((((-locals.var_fcick_dn1) * assign6470_body58_e7693) - (assign6470_body58_e7688 * (locals.var_fcick_dn1 * p.p115))) / (assign6470_body58_e7693 * assign6470_body58_e7693)), ((((-locals.var_fcick_dn3) * assign6470_body58_e7693) - (assign6470_body58_e7688 * (locals.var_fcick_dn3 * p.p115))) / (assign6470_body58_e7693 * assign6470_body58_e7693)), ((((-locals.var_fcick_dn4) * assign6470_body58_e7693) - (assign6470_body58_e7688 * (locals.var_fcick_dn4 * p.p115))) / (assign6470_body58_e7693 * assign6470_body58_e7693)), ((((-locals.var_fcick_dn5) * assign6470_body58_e7693) - (assign6470_body58_e7688 * (locals.var_fcick_dn5 * p.p115))) / (assign6470_body58_e7693 * assign6470_body58_e7693)), ((((-locals.var_fcick_dn6) * assign6470_body58_e7693) - (assign6470_body58_e7688 * (locals.var_fcick_dn6 * p.p115))) / (assign6470_body58_e7693 * assign6470_body58_e7693)), ((((-locals.var_fcick_dn7) * assign6470_body58_e7693) - (assign6470_body58_e7688 * (locals.var_fcick_dn7 * p.p115))) / (assign6470_body58_e7693 * assign6470_body58_e7693)), ((((-locals.var_fcick_dn8) * assign6470_body58_e7693) - (assign6470_body58_e7688 * (locals.var_fcick_dn8 * p.p115))) / (assign6470_body58_e7693 * assign6470_body58_e7693)), ((((-locals.var_fcick_dn9) * assign6470_body58_e7693) - (assign6470_body58_e7688 * (locals.var_fcick_dn9 * p.p115))) / (assign6470_body58_e7693 * assign6470_body58_e7693)),)
    } else {
        (locals.var_fcw, locals.var_fcw_dn0, locals.var_fcw_dn1, locals.var_fcw_dn3, locals.var_fcw_dn4, locals.var_fcw_dn5, locals.var_fcw_dn6, locals.var_fcw_dn7, locals.var_fcw_dn8, locals.var_fcw_dn9,)
    }
};
            locals.var_fcw = assign6470_body58_e7696;
            locals.var_fcw_dn0 = assign6470_body58_e7696_d_n0;
            locals.var_fcw_dn1 = assign6470_body58_e7696_d_n1;
            locals.var_fcw_dn3 = assign6470_body58_e7696_d_n3;
            locals.var_fcw_dn4 = assign6470_body58_e7696_d_n4;
            locals.var_fcw_dn5 = assign6470_body58_e7696_d_n5;
            locals.var_fcw_dn6 = assign6470_body58_e7696_d_n6;
            locals.var_fcw_dn7 = assign6470_body58_e7696_d_n7;
            locals.var_fcw_dn8 = assign6470_body58_e7696_d_n8;
            locals.var_fcw_dn9 = assign6470_body58_e7696_d_n9;
            locals.var_fcw_rv = 0.0;
            let (assign6470_body59_e7712, assign6470_body59_e7712_d_n0, assign6470_body59_e7712_d_n1, assign6470_body59_e7712_d_n3, assign6470_body59_e7712_d_n4, assign6470_body59_e7712_d_n5, assign6470_body59_e7712_d_n6, assign6470_body59_e7712_d_n7, assign6470_body59_e7712_d_n8, assign6470_body59_e7712_d_n9,) = {
    if ((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 == 0.0)) {
        let assign6470_body59_e7709: f64 = (p.p115 * locals.var_fcw);
        let assign6470_body59_e7710: f64 = (1.0 + assign6470_body59_e7709);
        (assign6470_body59_e7710, (p.p115 * locals.var_fcw_dn0), (p.p115 * locals.var_fcw_dn1), (p.p115 * locals.var_fcw_dn3), (p.p115 * locals.var_fcw_dn4), (p.p115 * locals.var_fcw_dn5), (p.p115 * locals.var_fcw_dn6), (p.p115 * locals.var_fcw_dn7), (p.p115 * locals.var_fcw_dn8), (p.p115 * locals.var_fcw_dn9),)
    } else {
        (locals.var_fclatbw, locals.var_fclatbw_dn0, locals.var_fclatbw_dn1, locals.var_fclatbw_dn3, locals.var_fclatbw_dn4, locals.var_fclatbw_dn5, locals.var_fclatbw_dn6, locals.var_fclatbw_dn7, locals.var_fclatbw_dn8, locals.var_fclatbw_dn9,)
    }
};
            locals.var_fclatbw = assign6470_body59_e7712;
            locals.var_fclatbw_dn0 = assign6470_body59_e7712_d_n0;
            locals.var_fclatbw_dn1 = assign6470_body59_e7712_d_n1;
            locals.var_fclatbw_dn3 = assign6470_body59_e7712_d_n3;
            locals.var_fclatbw_dn4 = assign6470_body59_e7712_d_n4;
            locals.var_fclatbw_dn5 = assign6470_body59_e7712_d_n5;
            locals.var_fclatbw_dn6 = assign6470_body59_e7712_d_n6;
            locals.var_fclatbw_dn7 = assign6470_body59_e7712_d_n7;
            locals.var_fclatbw_dn8 = assign6470_body59_e7712_d_n8;
            locals.var_fclatbw_dn9 = assign6470_body59_e7712_d_n9;
            locals.var_fclatbw_rv = 0.0;
            let (assign6470_body60_e7736, assign6470_body60_e7736_d_n0, assign6470_body60_e7736_d_n1, assign6470_body60_e7736_d_n3, assign6470_body60_e7736_d_n4, assign6470_body60_e7736_d_n5, assign6470_body60_e7736_d_n6, assign6470_body60_e7736_d_n7, assign6470_body60_e7736_d_n8, assign6470_body60_e7736_d_n9,) = {
    if ((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 == 0.0)) {
        let assign6470_body60_e7724: f64 = (locals.var_fcw * locals.var_fcw);
        let assign6470_body60_e7728: f64 = (locals.var_latb_6 * 2.0);
        let assign6470_body60_e7730: f64 = (assign6470_body60_e7728 * locals.var_fcw);
        let assign6470_body60_e7731: f64 = (1.0 + assign6470_body60_e7730);
        let assign6470_body60_e7732: f64 = (assign6470_body60_e7724 * assign6470_body60_e7731);
        let assign6470_body60_e7734: f64 = (assign6470_body60_e7732 / locals.var_fclatbw);
        (assign6470_body60_e7734, (((((((locals.var_fcw_dn0 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn0)) * assign6470_body60_e7731) + (assign6470_body60_e7724 * (assign6470_body60_e7728 * locals.var_fcw_dn0))) * locals.var_fclatbw) - (assign6470_body60_e7732 * locals.var_fclatbw_dn0)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn1 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn1)) * assign6470_body60_e7731) + (assign6470_body60_e7724 * (assign6470_body60_e7728 * locals.var_fcw_dn1))) * locals.var_fclatbw) - (assign6470_body60_e7732 * locals.var_fclatbw_dn1)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn3 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn3)) * assign6470_body60_e7731) + (assign6470_body60_e7724 * (assign6470_body60_e7728 * locals.var_fcw_dn3))) * locals.var_fclatbw) - (assign6470_body60_e7732 * locals.var_fclatbw_dn3)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn4 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn4)) * assign6470_body60_e7731) + (assign6470_body60_e7724 * (assign6470_body60_e7728 * locals.var_fcw_dn4))) * locals.var_fclatbw) - (assign6470_body60_e7732 * locals.var_fclatbw_dn4)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn5 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn5)) * assign6470_body60_e7731) + (assign6470_body60_e7724 * (assign6470_body60_e7728 * locals.var_fcw_dn5))) * locals.var_fclatbw) - (assign6470_body60_e7732 * locals.var_fclatbw_dn5)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn6 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn6)) * assign6470_body60_e7731) + (assign6470_body60_e7724 * (assign6470_body60_e7728 * locals.var_fcw_dn6))) * locals.var_fclatbw) - (assign6470_body60_e7732 * locals.var_fclatbw_dn6)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn7 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn7)) * assign6470_body60_e7731) + (assign6470_body60_e7724 * (assign6470_body60_e7728 * locals.var_fcw_dn7))) * locals.var_fclatbw) - (assign6470_body60_e7732 * locals.var_fclatbw_dn7)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn8 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn8)) * assign6470_body60_e7731) + (assign6470_body60_e7724 * (assign6470_body60_e7728 * locals.var_fcw_dn8))) * locals.var_fclatbw) - (assign6470_body60_e7732 * locals.var_fclatbw_dn8)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn9 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn9)) * assign6470_body60_e7731) + (assign6470_body60_e7724 * (assign6470_body60_e7728 * locals.var_fcw_dn9))) * locals.var_fclatbw) - (assign6470_body60_e7732 * locals.var_fclatbw_dn9)) / (locals.var_fclatbw * locals.var_fclatbw)),)
    } else {
        (locals.var_fcf_ci, locals.var_fcf_ci_dn0, locals.var_fcf_ci_dn1, locals.var_fcf_ci_dn3, locals.var_fcf_ci_dn4, locals.var_fcf_ci_dn5, locals.var_fcf_ci_dn6, locals.var_fcf_ci_dn7, locals.var_fcf_ci_dn8, locals.var_fcf_ci_dn9,)
    }
};
            locals.var_fcf_ci = assign6470_body60_e7736;
            locals.var_fcf_ci_dn0 = assign6470_body60_e7736_d_n0;
            locals.var_fcf_ci_dn1 = assign6470_body60_e7736_d_n1;
            locals.var_fcf_ci_dn3 = assign6470_body60_e7736_d_n3;
            locals.var_fcf_ci_dn4 = assign6470_body60_e7736_d_n4;
            locals.var_fcf_ci_dn5 = assign6470_body60_e7736_d_n5;
            locals.var_fcf_ci_dn6 = assign6470_body60_e7736_d_n6;
            locals.var_fcf_ci_dn7 = assign6470_body60_e7736_d_n7;
            locals.var_fcf_ci_dn8 = assign6470_body60_e7736_d_n8;
            locals.var_fcf_ci_dn9 = assign6470_body60_e7736_d_n9;
            locals.var_fcf_ci_rv = 0.0;
            let (assign6470_body61_e7757, assign6470_body61_e7757_d_n0, assign6470_body61_e7757_d_n1, assign6470_body61_e7757_d_n3, assign6470_body61_e7757_d_n4, assign6470_body61_e7757_d_n5, assign6470_body61_e7757_d_n6, assign6470_body61_e7757_d_n7, assign6470_body61_e7757_d_n8, assign6470_body61_e7757_d_n9,) = {
    if ((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 == 0.0)) {
        let assign6470_body61_e7747: f64 = (-locals.var_fcdick_ditf);
        let assign6470_body61_e7749: f64 = (assign6470_body61_e7747 * locals.var_fclatbw);
        let assign6470_body61_e7753: f64 = (locals.var_fcick * p.p115);
        let assign6470_body61_e7754: f64 = (1.0 + assign6470_body61_e7753);
        let assign6470_body61_e7755: f64 = (assign6470_body61_e7749 / assign6470_body61_e7754);
        (assign6470_body61_e7755, ((((((-locals.var_fcdick_ditf_dn0) * locals.var_fclatbw) + (assign6470_body61_e7747 * locals.var_fclatbw_dn0)) * assign6470_body61_e7754) - (assign6470_body61_e7749 * (locals.var_fcick_dn0 * p.p115))) / (assign6470_body61_e7754 * assign6470_body61_e7754)), ((((((-locals.var_fcdick_ditf_dn1) * locals.var_fclatbw) + (assign6470_body61_e7747 * locals.var_fclatbw_dn1)) * assign6470_body61_e7754) - (assign6470_body61_e7749 * (locals.var_fcick_dn1 * p.p115))) / (assign6470_body61_e7754 * assign6470_body61_e7754)), ((((((-locals.var_fcdick_ditf_dn3) * locals.var_fclatbw) + (assign6470_body61_e7747 * locals.var_fclatbw_dn3)) * assign6470_body61_e7754) - (assign6470_body61_e7749 * (locals.var_fcick_dn3 * p.p115))) / (assign6470_body61_e7754 * assign6470_body61_e7754)), ((((((-locals.var_fcdick_ditf_dn4) * locals.var_fclatbw) + (assign6470_body61_e7747 * locals.var_fclatbw_dn4)) * assign6470_body61_e7754) - (assign6470_body61_e7749 * (locals.var_fcick_dn4 * p.p115))) / (assign6470_body61_e7754 * assign6470_body61_e7754)), ((((((-locals.var_fcdick_ditf_dn5) * locals.var_fclatbw) + (assign6470_body61_e7747 * locals.var_fclatbw_dn5)) * assign6470_body61_e7754) - (assign6470_body61_e7749 * (locals.var_fcick_dn5 * p.p115))) / (assign6470_body61_e7754 * assign6470_body61_e7754)), ((((((-locals.var_fcdick_ditf_dn6) * locals.var_fclatbw) + (assign6470_body61_e7747 * locals.var_fclatbw_dn6)) * assign6470_body61_e7754) - (assign6470_body61_e7749 * (locals.var_fcick_dn6 * p.p115))) / (assign6470_body61_e7754 * assign6470_body61_e7754)), ((((((-locals.var_fcdick_ditf_dn7) * locals.var_fclatbw) + (assign6470_body61_e7747 * locals.var_fclatbw_dn7)) * assign6470_body61_e7754) - (assign6470_body61_e7749 * (locals.var_fcick_dn7 * p.p115))) / (assign6470_body61_e7754 * assign6470_body61_e7754)), ((((((-locals.var_fcdick_ditf_dn8) * locals.var_fclatbw) + (assign6470_body61_e7747 * locals.var_fclatbw_dn8)) * assign6470_body61_e7754) - (assign6470_body61_e7749 * (locals.var_fcick_dn8 * p.p115))) / (assign6470_body61_e7754 * assign6470_body61_e7754)), ((((((-locals.var_fcdick_ditf_dn9) * locals.var_fclatbw) + (assign6470_body61_e7747 * locals.var_fclatbw_dn9)) * assign6470_body61_e7754) - (assign6470_body61_e7749 * (locals.var_fcick_dn9 * p.p115))) / (assign6470_body61_e7754 * assign6470_body61_e7754)),)
    } else {
        (locals.var_fcdw_ditf, locals.var_fcdw_ditf_dn0, locals.var_fcdw_ditf_dn1, locals.var_fcdw_ditf_dn3, locals.var_fcdw_ditf_dn4, locals.var_fcdw_ditf_dn5, locals.var_fcdw_ditf_dn6, locals.var_fcdw_ditf_dn7, locals.var_fcdw_ditf_dn8, locals.var_fcdw_ditf_dn9,)
    }
};
            locals.var_fcdw_ditf = assign6470_body61_e7757;
            locals.var_fcdw_ditf_dn0 = assign6470_body61_e7757_d_n0;
            locals.var_fcdw_ditf_dn1 = assign6470_body61_e7757_d_n1;
            locals.var_fcdw_ditf_dn3 = assign6470_body61_e7757_d_n3;
            locals.var_fcdw_ditf_dn4 = assign6470_body61_e7757_d_n4;
            locals.var_fcdw_ditf_dn5 = assign6470_body61_e7757_d_n5;
            locals.var_fcdw_ditf_dn6 = assign6470_body61_e7757_d_n6;
            locals.var_fcdw_ditf_dn7 = assign6470_body61_e7757_d_n7;
            locals.var_fcdw_ditf_dn8 = assign6470_body61_e7757_d_n8;
            locals.var_fcdw_ditf_dn9 = assign6470_body61_e7757_d_n9;
            locals.var_fcdw_ditf_rv = 0.0;
            let (assign6470_body62_e7779, assign6470_body62_e7779_d_n0, assign6470_body62_e7779_d_n1, assign6470_body62_e7779_d_n3, assign6470_body62_e7779_d_n4, assign6470_body62_e7779_d_n5, assign6470_body62_e7779_d_n6, assign6470_body62_e7779_d_n7, assign6470_body62_e7779_d_n8, assign6470_body62_e7779_d_n9,) = {
    if ((((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) && (locals.var_guard138 == 0.0)) {
        let assign6470_body62_e7772: f64 = (locals.var_fclatbw * locals.var_fclatbw);
        let assign6470_body62_e7773: f64 = (1.0 / assign6470_body62_e7772);
        let assign6470_body62_e7774: f64 = (1.0 + assign6470_body62_e7773);
        let assign6470_body62_e7775: f64 = (locals.var_fcw * assign6470_body62_e7774);
        let assign6470_body62_e7777: f64 = (assign6470_body62_e7775 * locals.var_fcdw_ditf);
        (assign6470_body62_e7777, ((((locals.var_fcw_dn0 * assign6470_body62_e7774) + (locals.var_fcw * (-(((locals.var_fclatbw_dn0 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn0)) / (assign6470_body62_e7772 * assign6470_body62_e7772))))) * locals.var_fcdw_ditf) + (assign6470_body62_e7775 * locals.var_fcdw_ditf_dn0)), ((((locals.var_fcw_dn1 * assign6470_body62_e7774) + (locals.var_fcw * (-(((locals.var_fclatbw_dn1 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn1)) / (assign6470_body62_e7772 * assign6470_body62_e7772))))) * locals.var_fcdw_ditf) + (assign6470_body62_e7775 * locals.var_fcdw_ditf_dn1)), ((((locals.var_fcw_dn3 * assign6470_body62_e7774) + (locals.var_fcw * (-(((locals.var_fclatbw_dn3 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn3)) / (assign6470_body62_e7772 * assign6470_body62_e7772))))) * locals.var_fcdw_ditf) + (assign6470_body62_e7775 * locals.var_fcdw_ditf_dn3)), ((((locals.var_fcw_dn4 * assign6470_body62_e7774) + (locals.var_fcw * (-(((locals.var_fclatbw_dn4 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn4)) / (assign6470_body62_e7772 * assign6470_body62_e7772))))) * locals.var_fcdw_ditf) + (assign6470_body62_e7775 * locals.var_fcdw_ditf_dn4)), ((((locals.var_fcw_dn5 * assign6470_body62_e7774) + (locals.var_fcw * (-(((locals.var_fclatbw_dn5 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn5)) / (assign6470_body62_e7772 * assign6470_body62_e7772))))) * locals.var_fcdw_ditf) + (assign6470_body62_e7775 * locals.var_fcdw_ditf_dn5)), ((((locals.var_fcw_dn6 * assign6470_body62_e7774) + (locals.var_fcw * (-(((locals.var_fclatbw_dn6 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn6)) / (assign6470_body62_e7772 * assign6470_body62_e7772))))) * locals.var_fcdw_ditf) + (assign6470_body62_e7775 * locals.var_fcdw_ditf_dn6)), ((((locals.var_fcw_dn7 * assign6470_body62_e7774) + (locals.var_fcw * (-(((locals.var_fclatbw_dn7 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn7)) / (assign6470_body62_e7772 * assign6470_body62_e7772))))) * locals.var_fcdw_ditf) + (assign6470_body62_e7775 * locals.var_fcdw_ditf_dn7)), ((((locals.var_fcw_dn8 * assign6470_body62_e7774) + (locals.var_fcw * (-(((locals.var_fclatbw_dn8 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn8)) / (assign6470_body62_e7772 * assign6470_body62_e7772))))) * locals.var_fcdw_ditf) + (assign6470_body62_e7775 * locals.var_fcdw_ditf_dn8)), ((((locals.var_fcw_dn9 * assign6470_body62_e7774) + (locals.var_fcw * (-(((locals.var_fclatbw_dn9 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn9)) / (assign6470_body62_e7772 * assign6470_body62_e7772))))) * locals.var_fcdw_ditf) + (assign6470_body62_e7775 * locals.var_fcdw_ditf_dn9)),)
    } else {
        (locals.var_fcdfc_ditf, locals.var_fcdfc_ditf_dn0, locals.var_fcdfc_ditf_dn1, locals.var_fcdfc_ditf_dn3, locals.var_fcdfc_ditf_dn4, locals.var_fcdfc_ditf_dn5, locals.var_fcdfc_ditf_dn6, locals.var_fcdfc_ditf_dn7, locals.var_fcdfc_ditf_dn8, locals.var_fcdfc_ditf_dn9,)
    }
};
            locals.var_fcdfc_ditf = assign6470_body62_e7779;
            locals.var_fcdfc_ditf_dn0 = assign6470_body62_e7779_d_n0;
            locals.var_fcdfc_ditf_dn1 = assign6470_body62_e7779_d_n1;
            locals.var_fcdfc_ditf_dn3 = assign6470_body62_e7779_d_n3;
            locals.var_fcdfc_ditf_dn4 = assign6470_body62_e7779_d_n4;
            locals.var_fcdfc_ditf_dn5 = assign6470_body62_e7779_d_n5;
            locals.var_fcdfc_ditf_dn6 = assign6470_body62_e7779_d_n6;
            locals.var_fcdfc_ditf_dn7 = assign6470_body62_e7779_d_n7;
            locals.var_fcdfc_ditf_dn8 = assign6470_body62_e7779_d_n8;
            locals.var_fcdfc_ditf_dn9 = assign6470_body62_e7779_d_n9;
            locals.var_fcdfc_ditf_rv = 0.0;
            let (assign6470_body63_e7792, assign6470_body63_e7792_d_n0, assign6470_body63_e7792_d_n1, assign6470_body63_e7792_d_n3, assign6470_body63_e7792_d_n4, assign6470_body63_e7792_d_n5, assign6470_body63_e7792_d_n6, assign6470_body63_e7792_d_n7, assign6470_body63_e7792_d_n8, assign6470_body63_e7792_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) {
        let assign6470_body63_e7788: f64 = (p.p73 * locals.var_thcs_t);
        let assign6470_body63_e7790: f64 = (assign6470_body63_e7788 * locals.var_ffvc_exp);
        (assign6470_body63_e7790, (assign6470_body63_e7788 * locals.var_ffvc_exp_dn0), (assign6470_body63_e7788 * locals.var_ffvc_exp_dn1), (assign6470_body63_e7788 * locals.var_ffvc_exp_dn3), (((p.p73 * locals.var_thcs_t_dn4) * locals.var_ffvc_exp) + (assign6470_body63_e7788 * locals.var_ffvc_exp_dn4)), (assign6470_body63_e7788 * locals.var_ffvc_exp_dn5), (assign6470_body63_e7788 * locals.var_ffvc_exp_dn6), (assign6470_body63_e7788 * locals.var_ffvc_exp_dn7), (assign6470_body63_e7788 * locals.var_ffvc_exp_dn8), (assign6470_body63_e7788 * locals.var_ffvc_exp_dn9),)
    } else {
        (locals.var_dum_a, locals.var_dum_a_dn0, locals.var_dum_a_dn1, locals.var_dum_a_dn3, locals.var_dum_a_dn4, locals.var_dum_a_dn5, locals.var_dum_a_dn6, locals.var_dum_a_dn7, locals.var_dum_a_dn8, locals.var_dum_a_dn9,)
    }
};
            locals.var_dum_a = assign6470_body63_e7792;
            locals.var_dum_a_dn0 = assign6470_body63_e7792_d_n0;
            locals.var_dum_a_dn1 = assign6470_body63_e7792_d_n1;
            locals.var_dum_a_dn3 = assign6470_body63_e7792_d_n3;
            locals.var_dum_a_dn4 = assign6470_body63_e7792_d_n4;
            locals.var_dum_a_dn5 = assign6470_body63_e7792_d_n5;
            locals.var_dum_a_dn6 = assign6470_body63_e7792_d_n6;
            locals.var_dum_a_dn7 = assign6470_body63_e7792_d_n7;
            locals.var_dum_a_dn8 = assign6470_body63_e7792_d_n8;
            locals.var_dum_a_dn9 = assign6470_body63_e7792_d_n9;
            locals.var_dum_a_rv = 0.0;
            let (assign6470_body64_e7803, assign6470_body64_e7803_d_n0, assign6470_body64_e7803_d_n1, assign6470_body64_e7803_d_n3, assign6470_body64_e7803_d_n4, assign6470_body64_e7803_d_n5, assign6470_body64_e7803_d_n6, assign6470_body64_e7803_d_n7, assign6470_body64_e7803_d_n8, assign6470_body64_e7803_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) {
        let assign6470_body64_e7801: f64 = (locals.var_dum_a * locals.var_fcf_ci);
        (assign6470_body64_e7801, ((locals.var_dum_a_dn0 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn0)), ((locals.var_dum_a_dn1 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn1)), ((locals.var_dum_a_dn3 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn3)), ((locals.var_dum_a_dn4 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn4)), ((locals.var_dum_a_dn5 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn5)), ((locals.var_dum_a_dn6 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn6)), ((locals.var_dum_a_dn7 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn7)), ((locals.var_dum_a_dn8 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn8)), ((locals.var_dum_a_dn9 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn9)),)
    } else {
        (locals.var_dum_b, locals.var_dum_b_dn0, locals.var_dum_b_dn1, locals.var_dum_b_dn3, locals.var_dum_b_dn4, locals.var_dum_b_dn5, locals.var_dum_b_dn6, locals.var_dum_b_dn7, locals.var_dum_b_dn8, locals.var_dum_b_dn9,)
    }
};
            locals.var_dum_b = assign6470_body64_e7803;
            locals.var_dum_b_dn0 = assign6470_body64_e7803_d_n0;
            locals.var_dum_b_dn1 = assign6470_body64_e7803_d_n1;
            locals.var_dum_b_dn3 = assign6470_body64_e7803_d_n3;
            locals.var_dum_b_dn4 = assign6470_body64_e7803_d_n4;
            locals.var_dum_b_dn5 = assign6470_body64_e7803_d_n5;
            locals.var_dum_b_dn6 = assign6470_body64_e7803_d_n6;
            locals.var_dum_b_dn7 = assign6470_body64_e7803_d_n7;
            locals.var_dum_b_dn8 = assign6470_body64_e7803_d_n8;
            locals.var_dum_b_dn9 = assign6470_body64_e7803_d_n9;
            locals.var_dum_b_rv = 0.0;
            let (assign6470_body65_e7814, assign6470_body65_e7814_d_n0, assign6470_body65_e7814_d_n1, assign6470_body65_e7814_d_n3, assign6470_body65_e7814_d_n4, assign6470_body65_e7814_d_n5, assign6470_body65_e7814_d_n6, assign6470_body65_e7814_d_n7, assign6470_body65_e7814_d_n8, assign6470_body65_e7814_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) {
        let assign6470_body65_e7812: f64 = (locals.var_dum_b * locals.var_itf);
        (assign6470_body65_e7812, ((locals.var_dum_b_dn0 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn0)), ((locals.var_dum_b_dn1 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn1)), ((locals.var_dum_b_dn3 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn3)), ((locals.var_dum_b_dn4 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn4)), ((locals.var_dum_b_dn5 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn5)), ((locals.var_dum_b_dn6 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn6)), ((locals.var_dum_b_dn7 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn7)), ((locals.var_dum_b_dn8 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn8)), ((locals.var_dum_b_dn9 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn9)),)
    } else {
        (locals.var_ffdqcfc, locals.var_ffdqcfc_dn0, locals.var_ffdqcfc_dn1, locals.var_ffdqcfc_dn3, locals.var_ffdqcfc_dn4, locals.var_ffdqcfc_dn5, locals.var_ffdqcfc_dn6, locals.var_ffdqcfc_dn7, locals.var_ffdqcfc_dn8, locals.var_ffdqcfc_dn9,)
    }
};
            locals.var_ffdqcfc = assign6470_body65_e7814;
            locals.var_ffdqcfc_dn0 = assign6470_body65_e7814_d_n0;
            locals.var_ffdqcfc_dn1 = assign6470_body65_e7814_d_n1;
            locals.var_ffdqcfc_dn3 = assign6470_body65_e7814_d_n3;
            locals.var_ffdqcfc_dn4 = assign6470_body65_e7814_d_n4;
            locals.var_ffdqcfc_dn5 = assign6470_body65_e7814_d_n5;
            locals.var_ffdqcfc_dn6 = assign6470_body65_e7814_d_n6;
            locals.var_ffdqcfc_dn7 = assign6470_body65_e7814_d_n7;
            locals.var_ffdqcfc_dn8 = assign6470_body65_e7814_d_n8;
            locals.var_ffdqcfc_dn9 = assign6470_body65_e7814_d_n9;
            locals.var_ffdqcfc_rv = 0.0;
            let (assign6470_body66_e7835, assign6470_body66_e7835_d_n0, assign6470_body66_e7835_d_n1, assign6470_body66_e7835_d_n3, assign6470_body66_e7835_d_n4, assign6470_body66_e7835_d_n5, assign6470_body66_e7835_d_n6, assign6470_body66_e7835_d_n7, assign6470_body66_e7835_d_n8, assign6470_body66_e7835_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard137 == 0.0)) {
        let assign6470_body66_e7824: f64 = (locals.var_ffdqcfc * locals.var_ffdvc_ditf);
        let assign6470_body66_e7826: f64 = (assign6470_body66_e7824 * locals.var_ovt);
        let assign6470_body66_e7827: f64 = (locals.var_dum_b + assign6470_body66_e7826);
        let assign6470_body66_e7830: f64 = (locals.var_dum_a * locals.var_itf);
        let assign6470_body66_e7832: f64 = (assign6470_body66_e7830 * locals.var_fcdfc_ditf);
        let assign6470_body66_e7833: f64 = (assign6470_body66_e7827 + assign6470_body66_e7832);
        (assign6470_body66_e7833, ((locals.var_dum_b_dn0 + (((locals.var_ffdqcfc_dn0 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn0)) * locals.var_ovt)) + ((((locals.var_dum_a_dn0 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn0)) * locals.var_fcdfc_ditf) + (assign6470_body66_e7830 * locals.var_fcdfc_ditf_dn0))), ((locals.var_dum_b_dn1 + (((locals.var_ffdqcfc_dn1 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn1)) * locals.var_ovt)) + ((((locals.var_dum_a_dn1 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn1)) * locals.var_fcdfc_ditf) + (assign6470_body66_e7830 * locals.var_fcdfc_ditf_dn1))), ((locals.var_dum_b_dn3 + (((locals.var_ffdqcfc_dn3 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn3)) * locals.var_ovt)) + ((((locals.var_dum_a_dn3 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn3)) * locals.var_fcdfc_ditf) + (assign6470_body66_e7830 * locals.var_fcdfc_ditf_dn3))), ((locals.var_dum_b_dn4 + ((((locals.var_ffdqcfc_dn4 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn4)) * locals.var_ovt) + (assign6470_body66_e7824 * locals.var_ovt_dn4))) + ((((locals.var_dum_a_dn4 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn4)) * locals.var_fcdfc_ditf) + (assign6470_body66_e7830 * locals.var_fcdfc_ditf_dn4))), ((locals.var_dum_b_dn5 + (((locals.var_ffdqcfc_dn5 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn5)) * locals.var_ovt)) + ((((locals.var_dum_a_dn5 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn5)) * locals.var_fcdfc_ditf) + (assign6470_body66_e7830 * locals.var_fcdfc_ditf_dn5))), ((locals.var_dum_b_dn6 + (((locals.var_ffdqcfc_dn6 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn6)) * locals.var_ovt)) + ((((locals.var_dum_a_dn6 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn6)) * locals.var_fcdfc_ditf) + (assign6470_body66_e7830 * locals.var_fcdfc_ditf_dn6))), ((locals.var_dum_b_dn7 + (((locals.var_ffdqcfc_dn7 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn7)) * locals.var_ovt)) + ((((locals.var_dum_a_dn7 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn7)) * locals.var_fcdfc_ditf) + (assign6470_body66_e7830 * locals.var_fcdfc_ditf_dn7))), ((locals.var_dum_b_dn8 + (((locals.var_ffdqcfc_dn8 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn8)) * locals.var_ovt)) + ((((locals.var_dum_a_dn8 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn8)) * locals.var_fcdfc_ditf) + (assign6470_body66_e7830 * locals.var_fcdfc_ditf_dn8))), ((locals.var_dum_b_dn9 + (((locals.var_ffdqcfc_dn9 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn9)) * locals.var_ovt)) + ((((locals.var_dum_a_dn9 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn9)) * locals.var_fcdfc_ditf) + (assign6470_body66_e7830 * locals.var_fcdfc_ditf_dn9))),)
    } else {
        (locals.var_ffdtcfc, locals.var_ffdtcfc_dn0, locals.var_ffdtcfc_dn1, locals.var_ffdtcfc_dn3, locals.var_ffdtcfc_dn4, locals.var_ffdtcfc_dn5, locals.var_ffdtcfc_dn6, locals.var_ffdtcfc_dn7, locals.var_ffdtcfc_dn8, locals.var_ffdtcfc_dn9,)
    }
};
            locals.var_ffdtcfc = assign6470_body66_e7835;
            locals.var_ffdtcfc_dn0 = assign6470_body66_e7835_d_n0;
            locals.var_ffdtcfc_dn1 = assign6470_body66_e7835_d_n1;
            locals.var_ffdtcfc_dn3 = assign6470_body66_e7835_d_n3;
            locals.var_ffdtcfc_dn4 = assign6470_body66_e7835_d_n4;
            locals.var_ffdtcfc_dn5 = assign6470_body66_e7835_d_n5;
            locals.var_ffdtcfc_dn6 = assign6470_body66_e7835_d_n6;
            locals.var_ffdtcfc_dn7 = assign6470_body66_e7835_d_n7;
            locals.var_ffdtcfc_dn8 = assign6470_body66_e7835_d_n8;
            locals.var_ffdtcfc_dn9 = assign6470_body66_e7835_d_n9;
            locals.var_ffdtcfc_rv = 0.0;
            let (assign6470_body67_e7847, assign6470_body67_e7847_d_n0, assign6470_body67_e7847_d_n1, assign6470_body67_e7847_d_n3, assign6470_body67_e7847_d_n4, assign6470_body67_e7847_d_n5, assign6470_body67_e7847_d_n6, assign6470_body67_e7847_d_n7, assign6470_body67_e7847_d_n8, assign6470_body67_e7847_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign6470_body67_e7841: f64 = (1.0 - p.p73);
        let assign6470_body67_e7843: f64 = (assign6470_body67_e7841 * locals.var_ffdqfhc);
        let assign6470_body67_e7845: f64 = (assign6470_body67_e7843 * locals.var_itf);
        (assign6470_body67_e7845, (((assign6470_body67_e7841 * locals.var_ffdqfhc_dn0) * locals.var_itf) + (assign6470_body67_e7843 * locals.var_itf_dn0)), (((assign6470_body67_e7841 * locals.var_ffdqfhc_dn1) * locals.var_itf) + (assign6470_body67_e7843 * locals.var_itf_dn1)), (((assign6470_body67_e7841 * locals.var_ffdqfhc_dn3) * locals.var_itf) + (assign6470_body67_e7843 * locals.var_itf_dn3)), (((assign6470_body67_e7841 * locals.var_ffdqfhc_dn4) * locals.var_itf) + (assign6470_body67_e7843 * locals.var_itf_dn4)), (((assign6470_body67_e7841 * locals.var_ffdqfhc_dn5) * locals.var_itf) + (assign6470_body67_e7843 * locals.var_itf_dn5)), (((assign6470_body67_e7841 * locals.var_ffdqfhc_dn6) * locals.var_itf) + (assign6470_body67_e7843 * locals.var_itf_dn6)), (((assign6470_body67_e7841 * locals.var_ffdqfhc_dn7) * locals.var_itf) + (assign6470_body67_e7843 * locals.var_itf_dn7)), (((assign6470_body67_e7841 * locals.var_ffdqfhc_dn8) * locals.var_itf) + (assign6470_body67_e7843 * locals.var_itf_dn8)), (((assign6470_body67_e7841 * locals.var_ffdqfhc_dn9) * locals.var_itf) + (assign6470_body67_e7843 * locals.var_itf_dn9)),)
    } else {
        (locals.var_ffdqbfc, locals.var_ffdqbfc_dn0, locals.var_ffdqbfc_dn1, locals.var_ffdqbfc_dn3, locals.var_ffdqbfc_dn4, locals.var_ffdqbfc_dn5, locals.var_ffdqbfc_dn6, locals.var_ffdqbfc_dn7, locals.var_ffdqbfc_dn8, locals.var_ffdqbfc_dn9,)
    }
};
            locals.var_ffdqbfc = assign6470_body67_e7847;
            locals.var_ffdqbfc_dn0 = assign6470_body67_e7847_d_n0;
            locals.var_ffdqbfc_dn1 = assign6470_body67_e7847_d_n1;
            locals.var_ffdqbfc_dn3 = assign6470_body67_e7847_d_n3;
            locals.var_ffdqbfc_dn4 = assign6470_body67_e7847_d_n4;
            locals.var_ffdqbfc_dn5 = assign6470_body67_e7847_d_n5;
            locals.var_ffdqbfc_dn6 = assign6470_body67_e7847_d_n6;
            locals.var_ffdqbfc_dn7 = assign6470_body67_e7847_d_n7;
            locals.var_ffdqbfc_dn8 = assign6470_body67_e7847_d_n8;
            locals.var_ffdqbfc_dn9 = assign6470_body67_e7847_d_n9;
            locals.var_ffdqbfc_rv = 0.0;
            let (assign6470_body68_e7857, assign6470_body68_e7857_d_n0, assign6470_body68_e7857_d_n1, assign6470_body68_e7857_d_n3, assign6470_body68_e7857_d_n4, assign6470_body68_e7857_d_n5, assign6470_body68_e7857_d_n6, assign6470_body68_e7857_d_n7, assign6470_body68_e7857_d_n8, assign6470_body68_e7857_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign6470_body68_e7853: f64 = (1.0 - p.p73);
        let assign6470_body68_e7855: f64 = (assign6470_body68_e7853 * locals.var_ffdtfhc);
        (assign6470_body68_e7855, (assign6470_body68_e7853 * locals.var_ffdtfhc_dn0), (assign6470_body68_e7853 * locals.var_ffdtfhc_dn1), (assign6470_body68_e7853 * locals.var_ffdtfhc_dn3), (assign6470_body68_e7853 * locals.var_ffdtfhc_dn4), (assign6470_body68_e7853 * locals.var_ffdtfhc_dn5), (assign6470_body68_e7853 * locals.var_ffdtfhc_dn6), (assign6470_body68_e7853 * locals.var_ffdtfhc_dn7), (assign6470_body68_e7853 * locals.var_ffdtfhc_dn8), (assign6470_body68_e7853 * locals.var_ffdtfhc_dn9),)
    } else {
        (locals.var_ffdtbfc, locals.var_ffdtbfc_dn0, locals.var_ffdtbfc_dn1, locals.var_ffdtbfc_dn3, locals.var_ffdtbfc_dn4, locals.var_ffdtbfc_dn5, locals.var_ffdtbfc_dn6, locals.var_ffdtbfc_dn7, locals.var_ffdtbfc_dn8, locals.var_ffdtbfc_dn9,)
    }
};
            locals.var_ffdtbfc = assign6470_body68_e7857;
            locals.var_ffdtbfc_dn0 = assign6470_body68_e7857_d_n0;
            locals.var_ffdtbfc_dn1 = assign6470_body68_e7857_d_n1;
            locals.var_ffdtbfc_dn3 = assign6470_body68_e7857_d_n3;
            locals.var_ffdtbfc_dn4 = assign6470_body68_e7857_d_n4;
            locals.var_ffdtbfc_dn5 = assign6470_body68_e7857_d_n5;
            locals.var_ffdtbfc_dn6 = assign6470_body68_e7857_d_n6;
            locals.var_ffdtbfc_dn7 = assign6470_body68_e7857_d_n7;
            locals.var_ffdtbfc_dn8 = assign6470_body68_e7857_d_n8;
            locals.var_ffdtbfc_dn9 = assign6470_body68_e7857_d_n9;
            locals.var_ffdtbfc_rv = 0.0;
            let (assign6470_body69_e7867, assign6470_body69_e7867_d_n0, assign6470_body69_e7867_d_n1, assign6470_body69_e7867_d_n3, assign6470_body69_e7867_d_n4, assign6470_body69_e7867_d_n5, assign6470_body69_e7867_d_n6, assign6470_body69_e7867_d_n7, assign6470_body69_e7867_d_n8, assign6470_body69_e7867_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign6470_body69_e7863: f64 = (locals.var_ffdqbfb * locals.var_itf);
        let assign6470_body69_e7865: f64 = (assign6470_body69_e7863 + locals.var_ffdqbfc);
        (assign6470_body69_e7865, (((locals.var_ffdqbfb_dn0 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn0)) + locals.var_ffdqbfc_dn0), (((locals.var_ffdqbfb_dn1 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn1)) + locals.var_ffdqbfc_dn1), (((locals.var_ffdqbfb_dn3 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn3)) + locals.var_ffdqbfc_dn3), (((locals.var_ffdqbfb_dn4 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn4)) + locals.var_ffdqbfc_dn4), (((locals.var_ffdqbfb_dn5 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn5)) + locals.var_ffdqbfc_dn5), (((locals.var_ffdqbfb_dn6 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn6)) + locals.var_ffdqbfc_dn6), (((locals.var_ffdqbfb_dn7 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn7)) + locals.var_ffdqbfc_dn7), (((locals.var_ffdqbfb_dn8 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn8)) + locals.var_ffdqbfc_dn8), (((locals.var_ffdqbfb_dn9 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn9)) + locals.var_ffdqbfc_dn9),)
    } else {
        (locals.var_q_bf, locals.var_q_bf_dn0, locals.var_q_bf_dn1, locals.var_q_bf_dn3, locals.var_q_bf_dn4, locals.var_q_bf_dn5, locals.var_q_bf_dn6, locals.var_q_bf_dn7, locals.var_q_bf_dn8, locals.var_q_bf_dn9,)
    }
};
            locals.var_q_bf = assign6470_body69_e7867;
            locals.var_q_bf_dn0 = assign6470_body69_e7867_d_n0;
            locals.var_q_bf_dn1 = assign6470_body69_e7867_d_n1;
            locals.var_q_bf_dn3 = assign6470_body69_e7867_d_n3;
            locals.var_q_bf_dn4 = assign6470_body69_e7867_d_n4;
            locals.var_q_bf_dn5 = assign6470_body69_e7867_d_n5;
            locals.var_q_bf_dn6 = assign6470_body69_e7867_d_n6;
            locals.var_q_bf_dn7 = assign6470_body69_e7867_d_n7;
            locals.var_q_bf_dn8 = assign6470_body69_e7867_d_n8;
            locals.var_q_bf_dn9 = assign6470_body69_e7867_d_n9;
            locals.var_q_bf_rv = 0.0;
            let assign6470_body70_e7870: f64 = if p.p0 >= 310.0 { 1.0 } else { 0.0 };
            locals.var_guard140 = assign6470_body70_e7870;
            locals.var_guard140_rv = 0.0;
            let (assign6470_body71_e7884, assign6470_body71_e7884_d_n0, assign6470_body71_e7884_d_n1, assign6470_body71_e7884_d_n3, assign6470_body71_e7884_d_n4, assign6470_body71_e7884_d_n5, assign6470_body71_e7884_d_n6, assign6470_body71_e7884_d_n7, assign6470_body71_e7884_d_n8, assign6470_body71_e7884_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard140 != 0.0)) {
        let assign6470_body71_e7878: f64 = (locals.var_qf + locals.var_q_bf);
        let assign6470_body71_e7880: f64 = (assign6470_body71_e7878 + locals.var_ffdqef);
        let assign6470_body71_e7882: f64 = (assign6470_body71_e7880 + locals.var_ffdqcfc);
        (assign6470_body71_e7882, (((locals.var_qf_dn0 + locals.var_q_bf_dn0) + locals.var_ffdqef_dn0) + locals.var_ffdqcfc_dn0), (((locals.var_qf_dn1 + locals.var_q_bf_dn1) + locals.var_ffdqef_dn1) + locals.var_ffdqcfc_dn1), (((locals.var_qf_dn3 + locals.var_q_bf_dn3) + locals.var_ffdqef_dn3) + locals.var_ffdqcfc_dn3), (((locals.var_qf_dn4 + locals.var_q_bf_dn4) + locals.var_ffdqef_dn4) + locals.var_ffdqcfc_dn4), (((locals.var_qf_dn5 + locals.var_q_bf_dn5) + locals.var_ffdqef_dn5) + locals.var_ffdqcfc_dn5), (((locals.var_qf_dn6 + locals.var_q_bf_dn6) + locals.var_ffdqef_dn6) + locals.var_ffdqcfc_dn6), (((locals.var_qf_dn7 + locals.var_q_bf_dn7) + locals.var_ffdqef_dn7) + locals.var_ffdqcfc_dn7), (((locals.var_qf_dn8 + locals.var_q_bf_dn8) + locals.var_ffdqef_dn8) + locals.var_ffdqcfc_dn8), (((locals.var_qf_dn9 + locals.var_q_bf_dn9) + locals.var_ffdqef_dn9) + locals.var_ffdqcfc_dn9),)
    } else {
        (locals.var_qf, locals.var_qf_dn0, locals.var_qf_dn1, locals.var_qf_dn3, locals.var_qf_dn4, locals.var_qf_dn5, locals.var_qf_dn6, locals.var_qf_dn7, locals.var_qf_dn8, locals.var_qf_dn9,)
    }
};
            locals.var_qf = assign6470_body71_e7884;
            locals.var_qf_dn0 = assign6470_body71_e7884_d_n0;
            locals.var_qf_dn1 = assign6470_body71_e7884_d_n1;
            locals.var_qf_dn3 = assign6470_body71_e7884_d_n3;
            locals.var_qf_dn4 = assign6470_body71_e7884_d_n4;
            locals.var_qf_dn5 = assign6470_body71_e7884_d_n5;
            locals.var_qf_dn6 = assign6470_body71_e7884_d_n6;
            locals.var_qf_dn7 = assign6470_body71_e7884_d_n7;
            locals.var_qf_dn8 = assign6470_body71_e7884_d_n8;
            locals.var_qf_dn9 = assign6470_body71_e7884_d_n9;
            locals.var_qf_rv = 0.0;
            let (assign6470_body72_e7900, assign6470_body72_e7900_d_n0, assign6470_body72_e7900_d_n1, assign6470_body72_e7900_d_n3, assign6470_body72_e7900_d_n4, assign6470_body72_e7900_d_n5, assign6470_body72_e7900_d_n6, assign6470_body72_e7900_d_n7, assign6470_body72_e7900_d_n8, assign6470_body72_e7900_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard140 != 0.0)) {
        let assign6470_body72_e7893: f64 = (locals.var_ffdtbfb + locals.var_ffdtbfc);
        let assign6470_body72_e7894: f64 = (locals.var_tf + assign6470_body72_e7893);
        let assign6470_body72_e7896: f64 = (assign6470_body72_e7894 + locals.var_ffdtef);
        let assign6470_body72_e7898: f64 = (assign6470_body72_e7896 + locals.var_ffdtcfc);
        (assign6470_body72_e7898, (((locals.var_tf_dn0 + (locals.var_ffdtbfb_dn0 + locals.var_ffdtbfc_dn0)) + locals.var_ffdtef_dn0) + locals.var_ffdtcfc_dn0), (((locals.var_tf_dn1 + (locals.var_ffdtbfb_dn1 + locals.var_ffdtbfc_dn1)) + locals.var_ffdtef_dn1) + locals.var_ffdtcfc_dn1), (((locals.var_tf_dn3 + (locals.var_ffdtbfb_dn3 + locals.var_ffdtbfc_dn3)) + locals.var_ffdtef_dn3) + locals.var_ffdtcfc_dn3), (((locals.var_tf_dn4 + (locals.var_ffdtbfb_dn4 + locals.var_ffdtbfc_dn4)) + locals.var_ffdtef_dn4) + locals.var_ffdtcfc_dn4), (((locals.var_tf_dn5 + (locals.var_ffdtbfb_dn5 + locals.var_ffdtbfc_dn5)) + locals.var_ffdtef_dn5) + locals.var_ffdtcfc_dn5), (((locals.var_tf_dn6 + (locals.var_ffdtbfb_dn6 + locals.var_ffdtbfc_dn6)) + locals.var_ffdtef_dn6) + locals.var_ffdtcfc_dn6), (((locals.var_tf_dn7 + (locals.var_ffdtbfb_dn7 + locals.var_ffdtbfc_dn7)) + locals.var_ffdtef_dn7) + locals.var_ffdtcfc_dn7), (((locals.var_tf_dn8 + (locals.var_ffdtbfb_dn8 + locals.var_ffdtbfc_dn8)) + locals.var_ffdtef_dn8) + locals.var_ffdtcfc_dn8), (((locals.var_tf_dn9 + (locals.var_ffdtbfb_dn9 + locals.var_ffdtbfc_dn9)) + locals.var_ffdtef_dn9) + locals.var_ffdtcfc_dn9),)
    } else {
        (locals.var_tf, locals.var_tf_dn0, locals.var_tf_dn1, locals.var_tf_dn3, locals.var_tf_dn4, locals.var_tf_dn5, locals.var_tf_dn6, locals.var_tf_dn7, locals.var_tf_dn8, locals.var_tf_dn9,)
    }
};
            locals.var_tf = assign6470_body72_e7900;
            locals.var_tf_dn0 = assign6470_body72_e7900_d_n0;
            locals.var_tf_dn1 = assign6470_body72_e7900_d_n1;
            locals.var_tf_dn3 = assign6470_body72_e7900_d_n3;
            locals.var_tf_dn4 = assign6470_body72_e7900_d_n4;
            locals.var_tf_dn5 = assign6470_body72_e7900_d_n5;
            locals.var_tf_dn6 = assign6470_body72_e7900_d_n6;
            locals.var_tf_dn7 = assign6470_body72_e7900_d_n7;
            locals.var_tf_dn8 = assign6470_body72_e7900_d_n8;
            locals.var_tf_dn9 = assign6470_body72_e7900_d_n9;
            locals.var_tf_rv = 0.0;
            let (assign6470_body73_e7920, assign6470_body73_e7920_d_n0, assign6470_body73_e7920_d_n1, assign6470_body73_e7920_d_n3, assign6470_body73_e7920_d_n4, assign6470_body73_e7920_d_n5, assign6470_body73_e7920_d_n6, assign6470_body73_e7920_d_n7, assign6470_body73_e7920_d_n8, assign6470_body73_e7920_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard140 != 0.0)) {
        let assign6470_body73_e7909: f64 = (p.p5 * locals.var_q_bf);
        let assign6470_body73_e7910: f64 = (locals.var_q_ft + assign6470_body73_e7909);
        let assign6470_body73_e7913: f64 = (locals.var_hfe_t * locals.var_ffdqef);
        let assign6470_body73_e7914: f64 = (assign6470_body73_e7910 + assign6470_body73_e7913);
        let assign6470_body73_e7917: f64 = (locals.var_hfc_t * locals.var_ffdqcfc);
        let assign6470_body73_e7918: f64 = (assign6470_body73_e7914 + assign6470_body73_e7917);
        (assign6470_body73_e7918, (((locals.var_q_ft_dn0 + (p.p5 * locals.var_q_bf_dn0)) + (locals.var_hfe_t * locals.var_ffdqef_dn0)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn0)), (((locals.var_q_ft_dn1 + (p.p5 * locals.var_q_bf_dn1)) + (locals.var_hfe_t * locals.var_ffdqef_dn1)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn1)), (((locals.var_q_ft_dn3 + (p.p5 * locals.var_q_bf_dn3)) + (locals.var_hfe_t * locals.var_ffdqef_dn3)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn3)), (((locals.var_q_ft_dn4 + (p.p5 * locals.var_q_bf_dn4)) + ((locals.var_hfe_t_dn4 * locals.var_ffdqef) + (locals.var_hfe_t * locals.var_ffdqef_dn4))) + ((locals.var_hfc_t_dn4 * locals.var_ffdqcfc) + (locals.var_hfc_t * locals.var_ffdqcfc_dn4))), (((locals.var_q_ft_dn5 + (p.p5 * locals.var_q_bf_dn5)) + (locals.var_hfe_t * locals.var_ffdqef_dn5)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn5)), (((locals.var_q_ft_dn6 + (p.p5 * locals.var_q_bf_dn6)) + (locals.var_hfe_t * locals.var_ffdqef_dn6)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn6)), (((locals.var_q_ft_dn7 + (p.p5 * locals.var_q_bf_dn7)) + (locals.var_hfe_t * locals.var_ffdqef_dn7)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn7)), (((locals.var_q_ft_dn8 + (p.p5 * locals.var_q_bf_dn8)) + (locals.var_hfe_t * locals.var_ffdqef_dn8)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn8)), (((locals.var_q_ft_dn9 + (p.p5 * locals.var_q_bf_dn9)) + (locals.var_hfe_t * locals.var_ffdqef_dn9)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn9)),)
    } else {
        (locals.var_q_ft, locals.var_q_ft_dn0, locals.var_q_ft_dn1, locals.var_q_ft_dn3, locals.var_q_ft_dn4, locals.var_q_ft_dn5, locals.var_q_ft_dn6, locals.var_q_ft_dn7, locals.var_q_ft_dn8, locals.var_q_ft_dn9,)
    }
};
            locals.var_q_ft = assign6470_body73_e7920;
            locals.var_q_ft_dn0 = assign6470_body73_e7920_d_n0;
            locals.var_q_ft_dn1 = assign6470_body73_e7920_d_n1;
            locals.var_q_ft_dn3 = assign6470_body73_e7920_d_n3;
            locals.var_q_ft_dn4 = assign6470_body73_e7920_d_n4;
            locals.var_q_ft_dn5 = assign6470_body73_e7920_d_n5;
            locals.var_q_ft_dn6 = assign6470_body73_e7920_d_n6;
            locals.var_q_ft_dn7 = assign6470_body73_e7920_d_n7;
            locals.var_q_ft_dn8 = assign6470_body73_e7920_d_n8;
            locals.var_q_ft_dn9 = assign6470_body73_e7920_d_n9;
            locals.var_q_ft_rv = 0.0;
            let (assign6470_body74_e7942, assign6470_body74_e7942_d_n0, assign6470_body74_e7942_d_n1, assign6470_body74_e7942_d_n3, assign6470_body74_e7942_d_n4, assign6470_body74_e7942_d_n5, assign6470_body74_e7942_d_n6, assign6470_body74_e7942_d_n7, assign6470_body74_e7942_d_n8, assign6470_body74_e7942_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard140 != 0.0)) {
        let assign6470_body74_e7930: f64 = (locals.var_ffdtbfb + locals.var_ffdtbfc);
        let assign6470_body74_e7931: f64 = (p.p5 * assign6470_body74_e7930);
        let assign6470_body74_e7932: f64 = (locals.var_t_ft + assign6470_body74_e7931);
        let assign6470_body74_e7935: f64 = (locals.var_hfe_t * locals.var_ffdtef);
        let assign6470_body74_e7936: f64 = (assign6470_body74_e7932 + assign6470_body74_e7935);
        let assign6470_body74_e7939: f64 = (locals.var_hfc_t * locals.var_ffdtcfc);
        let assign6470_body74_e7940: f64 = (assign6470_body74_e7936 + assign6470_body74_e7939);
        (assign6470_body74_e7940, (((locals.var_t_ft_dn0 + (p.p5 * (locals.var_ffdtbfb_dn0 + locals.var_ffdtbfc_dn0))) + (locals.var_hfe_t * locals.var_ffdtef_dn0)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn0)), (((locals.var_t_ft_dn1 + (p.p5 * (locals.var_ffdtbfb_dn1 + locals.var_ffdtbfc_dn1))) + (locals.var_hfe_t * locals.var_ffdtef_dn1)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn1)), (((locals.var_t_ft_dn3 + (p.p5 * (locals.var_ffdtbfb_dn3 + locals.var_ffdtbfc_dn3))) + (locals.var_hfe_t * locals.var_ffdtef_dn3)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn3)), (((locals.var_t_ft_dn4 + (p.p5 * (locals.var_ffdtbfb_dn4 + locals.var_ffdtbfc_dn4))) + ((locals.var_hfe_t_dn4 * locals.var_ffdtef) + (locals.var_hfe_t * locals.var_ffdtef_dn4))) + ((locals.var_hfc_t_dn4 * locals.var_ffdtcfc) + (locals.var_hfc_t * locals.var_ffdtcfc_dn4))), (((locals.var_t_ft_dn5 + (p.p5 * (locals.var_ffdtbfb_dn5 + locals.var_ffdtbfc_dn5))) + (locals.var_hfe_t * locals.var_ffdtef_dn5)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn5)), (((locals.var_t_ft_dn6 + (p.p5 * (locals.var_ffdtbfb_dn6 + locals.var_ffdtbfc_dn6))) + (locals.var_hfe_t * locals.var_ffdtef_dn6)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn6)), (((locals.var_t_ft_dn7 + (p.p5 * (locals.var_ffdtbfb_dn7 + locals.var_ffdtbfc_dn7))) + (locals.var_hfe_t * locals.var_ffdtef_dn7)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn7)), (((locals.var_t_ft_dn8 + (p.p5 * (locals.var_ffdtbfb_dn8 + locals.var_ffdtbfc_dn8))) + (locals.var_hfe_t * locals.var_ffdtef_dn8)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn8)), (((locals.var_t_ft_dn9 + (p.p5 * (locals.var_ffdtbfb_dn9 + locals.var_ffdtbfc_dn9))) + (locals.var_hfe_t * locals.var_ffdtef_dn9)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn9)),)
    } else {
        (locals.var_t_ft, locals.var_t_ft_dn0, locals.var_t_ft_dn1, locals.var_t_ft_dn3, locals.var_t_ft_dn4, locals.var_t_ft_dn5, locals.var_t_ft_dn6, locals.var_t_ft_dn7, locals.var_t_ft_dn8, locals.var_t_ft_dn9,)
    }
};
            locals.var_t_ft = assign6470_body74_e7942;
            locals.var_t_ft_dn0 = assign6470_body74_e7942_d_n0;
            locals.var_t_ft_dn1 = assign6470_body74_e7942_d_n1;
            locals.var_t_ft_dn3 = assign6470_body74_e7942_d_n3;
            locals.var_t_ft_dn4 = assign6470_body74_e7942_d_n4;
            locals.var_t_ft_dn5 = assign6470_body74_e7942_d_n5;
            locals.var_t_ft_dn6 = assign6470_body74_e7942_d_n6;
            locals.var_t_ft_dn7 = assign6470_body74_e7942_d_n7;
            locals.var_t_ft_dn8 = assign6470_body74_e7942_d_n8;
            locals.var_t_ft_dn9 = assign6470_body74_e7942_d_n9;
            locals.var_t_ft_rv = 0.0;
            let (assign6470_body75_e7963, assign6470_body75_e7963_d_n0, assign6470_body75_e7963_d_n1, assign6470_body75_e7963_d_n3, assign6470_body75_e7963_d_n4, assign6470_body75_e7963_d_n5, assign6470_body75_e7963_d_n6, assign6470_body75_e7963_d_n7, assign6470_body75_e7963_d_n8, assign6470_body75_e7963_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard140 == 0.0)) {
        let assign6470_body75_e7951: f64 = (locals.var_hf0_t * locals.var_qf);
        let assign6470_body75_e7953: f64 = (assign6470_body75_e7951 + locals.var_q_bf);
        let assign6470_body75_e7956: f64 = (locals.var_hfe_t * locals.var_ffdqef);
        let assign6470_body75_e7957: f64 = (assign6470_body75_e7953 + assign6470_body75_e7956);
        let assign6470_body75_e7960: f64 = (locals.var_hfc_t * locals.var_ffdqcfc);
        let assign6470_body75_e7961: f64 = (assign6470_body75_e7957 + assign6470_body75_e7960);
        (assign6470_body75_e7961, ((((locals.var_hf0_t * locals.var_qf_dn0) + locals.var_q_bf_dn0) + (locals.var_hfe_t * locals.var_ffdqef_dn0)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn0)), ((((locals.var_hf0_t * locals.var_qf_dn1) + locals.var_q_bf_dn1) + (locals.var_hfe_t * locals.var_ffdqef_dn1)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn1)), ((((locals.var_hf0_t * locals.var_qf_dn3) + locals.var_q_bf_dn3) + (locals.var_hfe_t * locals.var_ffdqef_dn3)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn3)), (((((locals.var_hf0_t_dn4 * locals.var_qf) + (locals.var_hf0_t * locals.var_qf_dn4)) + locals.var_q_bf_dn4) + ((locals.var_hfe_t_dn4 * locals.var_ffdqef) + (locals.var_hfe_t * locals.var_ffdqef_dn4))) + ((locals.var_hfc_t_dn4 * locals.var_ffdqcfc) + (locals.var_hfc_t * locals.var_ffdqcfc_dn4))), ((((locals.var_hf0_t * locals.var_qf_dn5) + locals.var_q_bf_dn5) + (locals.var_hfe_t * locals.var_ffdqef_dn5)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn5)), ((((locals.var_hf0_t * locals.var_qf_dn6) + locals.var_q_bf_dn6) + (locals.var_hfe_t * locals.var_ffdqef_dn6)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn6)), ((((locals.var_hf0_t * locals.var_qf_dn7) + locals.var_q_bf_dn7) + (locals.var_hfe_t * locals.var_ffdqef_dn7)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn7)), ((((locals.var_hf0_t * locals.var_qf_dn8) + locals.var_q_bf_dn8) + (locals.var_hfe_t * locals.var_ffdqef_dn8)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn8)), ((((locals.var_hf0_t * locals.var_qf_dn9) + locals.var_q_bf_dn9) + (locals.var_hfe_t * locals.var_ffdqef_dn9)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn9)),)
    } else {
        (locals.var_q_ft, locals.var_q_ft_dn0, locals.var_q_ft_dn1, locals.var_q_ft_dn3, locals.var_q_ft_dn4, locals.var_q_ft_dn5, locals.var_q_ft_dn6, locals.var_q_ft_dn7, locals.var_q_ft_dn8, locals.var_q_ft_dn9,)
    }
};
            locals.var_q_ft = assign6470_body75_e7963;
            locals.var_q_ft_dn0 = assign6470_body75_e7963_d_n0;
            locals.var_q_ft_dn1 = assign6470_body75_e7963_d_n1;
            locals.var_q_ft_dn3 = assign6470_body75_e7963_d_n3;
            locals.var_q_ft_dn4 = assign6470_body75_e7963_d_n4;
            locals.var_q_ft_dn5 = assign6470_body75_e7963_d_n5;
            locals.var_q_ft_dn6 = assign6470_body75_e7963_d_n6;
            locals.var_q_ft_dn7 = assign6470_body75_e7963_d_n7;
            locals.var_q_ft_dn8 = assign6470_body75_e7963_d_n8;
            locals.var_q_ft_dn9 = assign6470_body75_e7963_d_n9;
            locals.var_q_ft_rv = 0.0;
            let (assign6470_body76_e7978, assign6470_body76_e7978_d_n0, assign6470_body76_e7978_d_n1, assign6470_body76_e7978_d_n3, assign6470_body76_e7978_d_n4, assign6470_body76_e7978_d_n5, assign6470_body76_e7978_d_n6, assign6470_body76_e7978_d_n7, assign6470_body76_e7978_d_n8, assign6470_body76_e7978_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard140 == 0.0)) {
        let assign6470_body76_e7972: f64 = (locals.var_qf + locals.var_q_bf);
        let assign6470_body76_e7974: f64 = (assign6470_body76_e7972 + locals.var_ffdqef);
        let assign6470_body76_e7976: f64 = (assign6470_body76_e7974 + locals.var_ffdqcfc);
        (assign6470_body76_e7976, (((locals.var_qf_dn0 + locals.var_q_bf_dn0) + locals.var_ffdqef_dn0) + locals.var_ffdqcfc_dn0), (((locals.var_qf_dn1 + locals.var_q_bf_dn1) + locals.var_ffdqef_dn1) + locals.var_ffdqcfc_dn1), (((locals.var_qf_dn3 + locals.var_q_bf_dn3) + locals.var_ffdqef_dn3) + locals.var_ffdqcfc_dn3), (((locals.var_qf_dn4 + locals.var_q_bf_dn4) + locals.var_ffdqef_dn4) + locals.var_ffdqcfc_dn4), (((locals.var_qf_dn5 + locals.var_q_bf_dn5) + locals.var_ffdqef_dn5) + locals.var_ffdqcfc_dn5), (((locals.var_qf_dn6 + locals.var_q_bf_dn6) + locals.var_ffdqef_dn6) + locals.var_ffdqcfc_dn6), (((locals.var_qf_dn7 + locals.var_q_bf_dn7) + locals.var_ffdqef_dn7) + locals.var_ffdqcfc_dn7), (((locals.var_qf_dn8 + locals.var_q_bf_dn8) + locals.var_ffdqef_dn8) + locals.var_ffdqcfc_dn8), (((locals.var_qf_dn9 + locals.var_q_bf_dn9) + locals.var_ffdqef_dn9) + locals.var_ffdqcfc_dn9),)
    } else {
        (locals.var_qf, locals.var_qf_dn0, locals.var_qf_dn1, locals.var_qf_dn3, locals.var_qf_dn4, locals.var_qf_dn5, locals.var_qf_dn6, locals.var_qf_dn7, locals.var_qf_dn8, locals.var_qf_dn9,)
    }
};
            locals.var_qf = assign6470_body76_e7978;
            locals.var_qf_dn0 = assign6470_body76_e7978_d_n0;
            locals.var_qf_dn1 = assign6470_body76_e7978_d_n1;
            locals.var_qf_dn3 = assign6470_body76_e7978_d_n3;
            locals.var_qf_dn4 = assign6470_body76_e7978_d_n4;
            locals.var_qf_dn5 = assign6470_body76_e7978_d_n5;
            locals.var_qf_dn6 = assign6470_body76_e7978_d_n6;
            locals.var_qf_dn7 = assign6470_body76_e7978_d_n7;
            locals.var_qf_dn8 = assign6470_body76_e7978_d_n8;
            locals.var_qf_dn9 = assign6470_body76_e7978_d_n9;
            locals.var_qf_rv = 0.0;
            let (assign6470_body77_e8001, assign6470_body77_e8001_d_n0, assign6470_body77_e8001_d_n1, assign6470_body77_e8001_d_n3, assign6470_body77_e8001_d_n4, assign6470_body77_e8001_d_n5, assign6470_body77_e8001_d_n6, assign6470_body77_e8001_d_n7, assign6470_body77_e8001_d_n8, assign6470_body77_e8001_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard140 == 0.0)) {
        let assign6470_body77_e7987: f64 = (locals.var_hf0_t * locals.var_tf);
        let assign6470_body77_e7990: f64 = (locals.var_ffdtbfb + locals.var_ffdtbfc);
        let assign6470_body77_e7991: f64 = (assign6470_body77_e7987 + assign6470_body77_e7990);
        let assign6470_body77_e7994: f64 = (locals.var_hfe_t * locals.var_ffdtef);
        let assign6470_body77_e7995: f64 = (assign6470_body77_e7991 + assign6470_body77_e7994);
        let assign6470_body77_e7998: f64 = (locals.var_hfc_t * locals.var_ffdtcfc);
        let assign6470_body77_e7999: f64 = (assign6470_body77_e7995 + assign6470_body77_e7998);
        (assign6470_body77_e7999, ((((locals.var_hf0_t * locals.var_tf_dn0) + (locals.var_ffdtbfb_dn0 + locals.var_ffdtbfc_dn0)) + (locals.var_hfe_t * locals.var_ffdtef_dn0)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn0)), ((((locals.var_hf0_t * locals.var_tf_dn1) + (locals.var_ffdtbfb_dn1 + locals.var_ffdtbfc_dn1)) + (locals.var_hfe_t * locals.var_ffdtef_dn1)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn1)), ((((locals.var_hf0_t * locals.var_tf_dn3) + (locals.var_ffdtbfb_dn3 + locals.var_ffdtbfc_dn3)) + (locals.var_hfe_t * locals.var_ffdtef_dn3)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn3)), (((((locals.var_hf0_t_dn4 * locals.var_tf) + (locals.var_hf0_t * locals.var_tf_dn4)) + (locals.var_ffdtbfb_dn4 + locals.var_ffdtbfc_dn4)) + ((locals.var_hfe_t_dn4 * locals.var_ffdtef) + (locals.var_hfe_t * locals.var_ffdtef_dn4))) + ((locals.var_hfc_t_dn4 * locals.var_ffdtcfc) + (locals.var_hfc_t * locals.var_ffdtcfc_dn4))), ((((locals.var_hf0_t * locals.var_tf_dn5) + (locals.var_ffdtbfb_dn5 + locals.var_ffdtbfc_dn5)) + (locals.var_hfe_t * locals.var_ffdtef_dn5)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn5)), ((((locals.var_hf0_t * locals.var_tf_dn6) + (locals.var_ffdtbfb_dn6 + locals.var_ffdtbfc_dn6)) + (locals.var_hfe_t * locals.var_ffdtef_dn6)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn6)), ((((locals.var_hf0_t * locals.var_tf_dn7) + (locals.var_ffdtbfb_dn7 + locals.var_ffdtbfc_dn7)) + (locals.var_hfe_t * locals.var_ffdtef_dn7)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn7)), ((((locals.var_hf0_t * locals.var_tf_dn8) + (locals.var_ffdtbfb_dn8 + locals.var_ffdtbfc_dn8)) + (locals.var_hfe_t * locals.var_ffdtef_dn8)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn8)), ((((locals.var_hf0_t * locals.var_tf_dn9) + (locals.var_ffdtbfb_dn9 + locals.var_ffdtbfc_dn9)) + (locals.var_hfe_t * locals.var_ffdtef_dn9)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn9)),)
    } else {
        (locals.var_t_ft, locals.var_t_ft_dn0, locals.var_t_ft_dn1, locals.var_t_ft_dn3, locals.var_t_ft_dn4, locals.var_t_ft_dn5, locals.var_t_ft_dn6, locals.var_t_ft_dn7, locals.var_t_ft_dn8, locals.var_t_ft_dn9,)
    }
};
            locals.var_t_ft = assign6470_body77_e8001;
            locals.var_t_ft_dn0 = assign6470_body77_e8001_d_n0;
            locals.var_t_ft_dn1 = assign6470_body77_e8001_d_n1;
            locals.var_t_ft_dn3 = assign6470_body77_e8001_d_n3;
            locals.var_t_ft_dn4 = assign6470_body77_e8001_d_n4;
            locals.var_t_ft_dn5 = assign6470_body77_e8001_d_n5;
            locals.var_t_ft_dn6 = assign6470_body77_e8001_d_n6;
            locals.var_t_ft_dn7 = assign6470_body77_e8001_d_n7;
            locals.var_t_ft_dn8 = assign6470_body77_e8001_d_n8;
            locals.var_t_ft_dn9 = assign6470_body77_e8001_d_n9;
            locals.var_t_ft_rv = 0.0;
            let (assign6470_body78_e8018, assign6470_body78_e8018_d_n0, assign6470_body78_e8018_d_n1, assign6470_body78_e8018_d_n3, assign6470_body78_e8018_d_n4, assign6470_body78_e8018_d_n5, assign6470_body78_e8018_d_n6, assign6470_body78_e8018_d_n7, assign6470_body78_e8018_d_n8, assign6470_body78_e8018_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard134 != 0.0)) && (locals.var_guard140 == 0.0)) {
        let assign6470_body78_e8011: f64 = (locals.var_ffdtbfb + locals.var_ffdtbfc);
        let assign6470_body78_e8012: f64 = (locals.var_tf + assign6470_body78_e8011);
        let assign6470_body78_e8014: f64 = (assign6470_body78_e8012 + locals.var_ffdtef);
        let assign6470_body78_e8016: f64 = (assign6470_body78_e8014 + locals.var_ffdtcfc);
        (assign6470_body78_e8016, (((locals.var_tf_dn0 + (locals.var_ffdtbfb_dn0 + locals.var_ffdtbfc_dn0)) + locals.var_ffdtef_dn0) + locals.var_ffdtcfc_dn0), (((locals.var_tf_dn1 + (locals.var_ffdtbfb_dn1 + locals.var_ffdtbfc_dn1)) + locals.var_ffdtef_dn1) + locals.var_ffdtcfc_dn1), (((locals.var_tf_dn3 + (locals.var_ffdtbfb_dn3 + locals.var_ffdtbfc_dn3)) + locals.var_ffdtef_dn3) + locals.var_ffdtcfc_dn3), (((locals.var_tf_dn4 + (locals.var_ffdtbfb_dn4 + locals.var_ffdtbfc_dn4)) + locals.var_ffdtef_dn4) + locals.var_ffdtcfc_dn4), (((locals.var_tf_dn5 + (locals.var_ffdtbfb_dn5 + locals.var_ffdtbfc_dn5)) + locals.var_ffdtef_dn5) + locals.var_ffdtcfc_dn5), (((locals.var_tf_dn6 + (locals.var_ffdtbfb_dn6 + locals.var_ffdtbfc_dn6)) + locals.var_ffdtef_dn6) + locals.var_ffdtcfc_dn6), (((locals.var_tf_dn7 + (locals.var_ffdtbfb_dn7 + locals.var_ffdtbfc_dn7)) + locals.var_ffdtef_dn7) + locals.var_ffdtcfc_dn7), (((locals.var_tf_dn8 + (locals.var_ffdtbfb_dn8 + locals.var_ffdtbfc_dn8)) + locals.var_ffdtef_dn8) + locals.var_ffdtcfc_dn8), (((locals.var_tf_dn9 + (locals.var_ffdtbfb_dn9 + locals.var_ffdtbfc_dn9)) + locals.var_ffdtef_dn9) + locals.var_ffdtcfc_dn9),)
    } else {
        (locals.var_tf, locals.var_tf_dn0, locals.var_tf_dn1, locals.var_tf_dn3, locals.var_tf_dn4, locals.var_tf_dn5, locals.var_tf_dn6, locals.var_tf_dn7, locals.var_tf_dn8, locals.var_tf_dn9,)
    }
};
            locals.var_tf = assign6470_body78_e8018;
            locals.var_tf_dn0 = assign6470_body78_e8018_d_n0;
            locals.var_tf_dn1 = assign6470_body78_e8018_d_n1;
            locals.var_tf_dn3 = assign6470_body78_e8018_d_n3;
            locals.var_tf_dn4 = assign6470_body78_e8018_d_n4;
            locals.var_tf_dn5 = assign6470_body78_e8018_d_n5;
            locals.var_tf_dn6 = assign6470_body78_e8018_d_n6;
            locals.var_tf_dn7 = assign6470_body78_e8018_d_n7;
            locals.var_tf_dn8 = assign6470_body78_e8018_d_n8;
            locals.var_tf_dn9 = assign6470_body78_e8018_d_n9;
            locals.var_tf_rv = 0.0;
            let (assign6470_body79_e8026, assign6470_body79_e8026_d_n0, assign6470_body79_e8026_d_n1, assign6470_body79_e8026_d_n3, assign6470_body79_e8026_d_n4, assign6470_body79_e8026_d_n5, assign6470_body79_e8026_d_n6, assign6470_body79_e8026_d_n7, assign6470_body79_e8026_d_n8, assign6470_body79_e8026_d_n9,) = {
    if (locals.var_guard131 != 0.0) {
        let assign6470_body79_e8022: f64 = (p.p7 * p.p85);
        let assign6470_body79_e8024: f64 = (assign6470_body79_e8022 * locals.var_itr);
        (assign6470_body79_e8024, (assign6470_body79_e8022 * locals.var_itr_dn0), (assign6470_body79_e8022 * locals.var_itr_dn1), (assign6470_body79_e8022 * locals.var_itr_dn3), (assign6470_body79_e8022 * locals.var_itr_dn4), (assign6470_body79_e8022 * locals.var_itr_dn5), (assign6470_body79_e8022 * locals.var_itr_dn6), (assign6470_body79_e8022 * locals.var_itr_dn7), (assign6470_body79_e8022 * locals.var_itr_dn8), (assign6470_body79_e8022 * locals.var_itr_dn9),)
    } else {
        (locals.var_q_rt, locals.var_q_rt_dn0, locals.var_q_rt_dn1, locals.var_q_rt_dn3, locals.var_q_rt_dn4, locals.var_q_rt_dn5, locals.var_q_rt_dn6, locals.var_q_rt_dn7, locals.var_q_rt_dn8, locals.var_q_rt_dn9,)
    }
};
            locals.var_q_rt = assign6470_body79_e8026;
            locals.var_q_rt_dn0 = assign6470_body79_e8026_d_n0;
            locals.var_q_rt_dn1 = assign6470_body79_e8026_d_n1;
            locals.var_q_rt_dn3 = assign6470_body79_e8026_d_n3;
            locals.var_q_rt_dn4 = assign6470_body79_e8026_d_n4;
            locals.var_q_rt_dn5 = assign6470_body79_e8026_d_n5;
            locals.var_q_rt_dn6 = assign6470_body79_e8026_d_n6;
            locals.var_q_rt_dn7 = assign6470_body79_e8026_d_n7;
            locals.var_q_rt_dn8 = assign6470_body79_e8026_d_n8;
            locals.var_q_rt_dn9 = assign6470_body79_e8026_d_n9;
            locals.var_q_rt_rv = 0.0;
            let (assign6470_body80_e8047, assign6470_body80_e8047_d_n0, assign6470_body80_e8047_d_n1, assign6470_body80_e8047_d_n3, assign6470_body80_e8047_d_n4, assign6470_body80_e8047_d_n5, assign6470_body80_e8047_d_n6, assign6470_body80_e8047_d_n7, assign6470_body80_e8047_d_n8, assign6470_body80_e8047_d_n9,) = {
    if (locals.var_guard131 != 0.0) {
        let assign6470_body80_e8031: f64 = (locals.var_q0_pt + locals.var_q_ft);
        let assign6470_body80_e8033: f64 = (assign6470_body80_e8031 + locals.var_q_rt);
        let assign6470_body80_e8034: f64 = (locals.var_q_pt - assign6470_body80_e8033);
        let assign6470_body80_e8035: f64 = (-assign6470_body80_e8034);
        let assign6470_body80_e8039: f64 = (locals.var_t_ft * locals.var_itf);
        let assign6470_body80_e8041: f64 = (assign6470_body80_e8039 + locals.var_q_rt);
        let assign6470_body80_e8043: f64 = (assign6470_body80_e8041 / locals.var_q_pt);
        let assign6470_body80_e8044: f64 = (1.0 + assign6470_body80_e8043);
        let assign6470_body80_e8045: f64 = (assign6470_body80_e8035 / assign6470_body80_e8044);
        (assign6470_body80_e8045, ((((-(locals.var_q_pt_dn0 - ((locals.var_q0_pt_dn0 + locals.var_q_ft_dn0) + locals.var_q_rt_dn0))) * assign6470_body80_e8044) - (assign6470_body80_e8035 * ((((((locals.var_t_ft_dn0 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn0)) + locals.var_q_rt_dn0) * locals.var_q_pt) - (assign6470_body80_e8041 * locals.var_q_pt_dn0)) / (locals.var_q_pt * locals.var_q_pt)))) / (assign6470_body80_e8044 * assign6470_body80_e8044)), ((((-(locals.var_q_pt_dn1 - ((locals.var_q0_pt_dn1 + locals.var_q_ft_dn1) + locals.var_q_rt_dn1))) * assign6470_body80_e8044) - (assign6470_body80_e8035 * ((((((locals.var_t_ft_dn1 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn1)) + locals.var_q_rt_dn1) * locals.var_q_pt) - (assign6470_body80_e8041 * locals.var_q_pt_dn1)) / (locals.var_q_pt * locals.var_q_pt)))) / (assign6470_body80_e8044 * assign6470_body80_e8044)), ((((-(locals.var_q_pt_dn3 - ((locals.var_q0_pt_dn3 + locals.var_q_ft_dn3) + locals.var_q_rt_dn3))) * assign6470_body80_e8044) - (assign6470_body80_e8035 * ((((((locals.var_t_ft_dn3 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn3)) + locals.var_q_rt_dn3) * locals.var_q_pt) - (assign6470_body80_e8041 * locals.var_q_pt_dn3)) / (locals.var_q_pt * locals.var_q_pt)))) / (assign6470_body80_e8044 * assign6470_body80_e8044)), ((((-(locals.var_q_pt_dn4 - ((locals.var_q0_pt_dn4 + locals.var_q_ft_dn4) + locals.var_q_rt_dn4))) * assign6470_body80_e8044) - (assign6470_body80_e8035 * ((((((locals.var_t_ft_dn4 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn4)) + locals.var_q_rt_dn4) * locals.var_q_pt) - (assign6470_body80_e8041 * locals.var_q_pt_dn4)) / (locals.var_q_pt * locals.var_q_pt)))) / (assign6470_body80_e8044 * assign6470_body80_e8044)), ((((-(locals.var_q_pt_dn5 - ((locals.var_q0_pt_dn5 + locals.var_q_ft_dn5) + locals.var_q_rt_dn5))) * assign6470_body80_e8044) - (assign6470_body80_e8035 * ((((((locals.var_t_ft_dn5 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn5)) + locals.var_q_rt_dn5) * locals.var_q_pt) - (assign6470_body80_e8041 * locals.var_q_pt_dn5)) / (locals.var_q_pt * locals.var_q_pt)))) / (assign6470_body80_e8044 * assign6470_body80_e8044)), ((((-(locals.var_q_pt_dn6 - ((locals.var_q0_pt_dn6 + locals.var_q_ft_dn6) + locals.var_q_rt_dn6))) * assign6470_body80_e8044) - (assign6470_body80_e8035 * ((((((locals.var_t_ft_dn6 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn6)) + locals.var_q_rt_dn6) * locals.var_q_pt) - (assign6470_body80_e8041 * locals.var_q_pt_dn6)) / (locals.var_q_pt * locals.var_q_pt)))) / (assign6470_body80_e8044 * assign6470_body80_e8044)), ((((-(locals.var_q_pt_dn7 - ((locals.var_q0_pt_dn7 + locals.var_q_ft_dn7) + locals.var_q_rt_dn7))) * assign6470_body80_e8044) - (assign6470_body80_e8035 * ((((((locals.var_t_ft_dn7 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn7)) + locals.var_q_rt_dn7) * locals.var_q_pt) - (assign6470_body80_e8041 * locals.var_q_pt_dn7)) / (locals.var_q_pt * locals.var_q_pt)))) / (assign6470_body80_e8044 * assign6470_body80_e8044)), ((((-(locals.var_q_pt_dn8 - ((locals.var_q0_pt_dn8 + locals.var_q_ft_dn8) + locals.var_q_rt_dn8))) * assign6470_body80_e8044) - (assign6470_body80_e8035 * ((((((locals.var_t_ft_dn8 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn8)) + locals.var_q_rt_dn8) * locals.var_q_pt) - (assign6470_body80_e8041 * locals.var_q_pt_dn8)) / (locals.var_q_pt * locals.var_q_pt)))) / (assign6470_body80_e8044 * assign6470_body80_e8044)), ((((-(locals.var_q_pt_dn9 - ((locals.var_q0_pt_dn9 + locals.var_q_ft_dn9) + locals.var_q_rt_dn9))) * assign6470_body80_e8044) - (assign6470_body80_e8035 * ((((((locals.var_t_ft_dn9 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn9)) + locals.var_q_rt_dn9) * locals.var_q_pt) - (assign6470_body80_e8041 * locals.var_q_pt_dn9)) / (locals.var_q_pt * locals.var_q_pt)))) / (assign6470_body80_e8044 * assign6470_body80_e8044)),)
    } else {
        (locals.var_d_q, locals.var_d_q_dn0, locals.var_d_q_dn1, locals.var_d_q_dn3, locals.var_d_q_dn4, locals.var_d_q_dn5, locals.var_d_q_dn6, locals.var_d_q_dn7, locals.var_d_q_dn8, locals.var_d_q_dn9,)
    }
};
            locals.var_d_q = assign6470_body80_e8047;
            locals.var_d_q_dn0 = assign6470_body80_e8047_d_n0;
            locals.var_d_q_dn1 = assign6470_body80_e8047_d_n1;
            locals.var_d_q_dn3 = assign6470_body80_e8047_d_n3;
            locals.var_d_q_dn4 = assign6470_body80_e8047_d_n4;
            locals.var_d_q_dn5 = assign6470_body80_e8047_d_n5;
            locals.var_d_q_dn6 = assign6470_body80_e8047_d_n6;
            locals.var_d_q_dn7 = assign6470_body80_e8047_d_n7;
            locals.var_d_q_dn8 = assign6470_body80_e8047_d_n8;
            locals.var_d_q_dn9 = assign6470_body80_e8047_d_n9;
            locals.var_d_q_rv = 0.0;
            let (assign6470_body81_e8054, assign6470_body81_e8054_d_n0, assign6470_body81_e8054_d_n1, assign6470_body81_e8054_d_n3, assign6470_body81_e8054_d_n4, assign6470_body81_e8054_d_n5, assign6470_body81_e8054_d_n6, assign6470_body81_e8054_d_n7, assign6470_body81_e8054_d_n8, assign6470_body81_e8054_d_n9,) = {
    if (locals.var_guard131 != 0.0) {
        let assign6470_body81_e8051: f64 = (0.3 * locals.var_q_pt);
        let assign6470_body81_e8052: f64 = (assign6470_body81_e8051).abs();
        (assign6470_body81_e8052, if assign6470_body81_e8051 >= 0.0 { (0.3 * locals.var_q_pt_dn0) } else { (-(0.3 * locals.var_q_pt_dn0)) }, if assign6470_body81_e8051 >= 0.0 { (0.3 * locals.var_q_pt_dn1) } else { (-(0.3 * locals.var_q_pt_dn1)) }, if assign6470_body81_e8051 >= 0.0 { (0.3 * locals.var_q_pt_dn3) } else { (-(0.3 * locals.var_q_pt_dn3)) }, if assign6470_body81_e8051 >= 0.0 { (0.3 * locals.var_q_pt_dn4) } else { (-(0.3 * locals.var_q_pt_dn4)) }, if assign6470_body81_e8051 >= 0.0 { (0.3 * locals.var_q_pt_dn5) } else { (-(0.3 * locals.var_q_pt_dn5)) }, if assign6470_body81_e8051 >= 0.0 { (0.3 * locals.var_q_pt_dn6) } else { (-(0.3 * locals.var_q_pt_dn6)) }, if assign6470_body81_e8051 >= 0.0 { (0.3 * locals.var_q_pt_dn7) } else { (-(0.3 * locals.var_q_pt_dn7)) }, if assign6470_body81_e8051 >= 0.0 { (0.3 * locals.var_q_pt_dn8) } else { (-(0.3 * locals.var_q_pt_dn8)) }, if assign6470_body81_e8051 >= 0.0 { (0.3 * locals.var_q_pt_dn9) } else { (-(0.3 * locals.var_q_pt_dn9)) },)
    } else {
        (locals.var_d_q_max, locals.var_d_q_max_dn0, locals.var_d_q_max_dn1, locals.var_d_q_max_dn3, locals.var_d_q_max_dn4, locals.var_d_q_max_dn5, locals.var_d_q_max_dn6, locals.var_d_q_max_dn7, locals.var_d_q_max_dn8, locals.var_d_q_max_dn9,)
    }
};
            locals.var_d_q_max = assign6470_body81_e8054;
            locals.var_d_q_max_dn0 = assign6470_body81_e8054_d_n0;
            locals.var_d_q_max_dn1 = assign6470_body81_e8054_d_n1;
            locals.var_d_q_max_dn3 = assign6470_body81_e8054_d_n3;
            locals.var_d_q_max_dn4 = assign6470_body81_e8054_d_n4;
            locals.var_d_q_max_dn5 = assign6470_body81_e8054_d_n5;
            locals.var_d_q_max_dn6 = assign6470_body81_e8054_d_n6;
            locals.var_d_q_max_dn7 = assign6470_body81_e8054_d_n7;
            locals.var_d_q_max_dn8 = assign6470_body81_e8054_d_n8;
            locals.var_d_q_max_dn9 = assign6470_body81_e8054_d_n9;
            locals.var_d_q_max_rv = 0.0;
            let assign6470_body82_e8056: f64 = (locals.var_d_q).abs();
            let assign6470_body82_e8058: f64 = if assign6470_body82_e8056 > locals.var_d_q_max { 1.0 } else { 0.0 };
            locals.var_guard141 = assign6470_body82_e8058;
            locals.var_guard141_rv = 0.0;
            let assign6470_body83_e8061: f64 = if locals.var_d_q >= 0.0 { 1.0 } else { 0.0 };
            locals.var_guard142 = assign6470_body83_e8061;
            locals.var_guard142_rv = 0.0;
            let (assign6470_body84_e8069, assign6470_body84_e8069_d_n0, assign6470_body84_e8069_d_n1, assign6470_body84_e8069_d_n3, assign6470_body84_e8069_d_n4, assign6470_body84_e8069_d_n5, assign6470_body84_e8069_d_n6, assign6470_body84_e8069_d_n7, assign6470_body84_e8069_d_n8, assign6470_body84_e8069_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard141 != 0.0)) && (locals.var_guard142 != 0.0)) {
        (locals.var_d_q_max, locals.var_d_q_max_dn0, locals.var_d_q_max_dn1, locals.var_d_q_max_dn3, locals.var_d_q_max_dn4, locals.var_d_q_max_dn5, locals.var_d_q_max_dn6, locals.var_d_q_max_dn7, locals.var_d_q_max_dn8, locals.var_d_q_max_dn9,)
    } else {
        (locals.var_d_q, locals.var_d_q_dn0, locals.var_d_q_dn1, locals.var_d_q_dn3, locals.var_d_q_dn4, locals.var_d_q_dn5, locals.var_d_q_dn6, locals.var_d_q_dn7, locals.var_d_q_dn8, locals.var_d_q_dn9,)
    }
};
            locals.var_d_q = assign6470_body84_e8069;
            locals.var_d_q_dn0 = assign6470_body84_e8069_d_n0;
            locals.var_d_q_dn1 = assign6470_body84_e8069_d_n1;
            locals.var_d_q_dn3 = assign6470_body84_e8069_d_n3;
            locals.var_d_q_dn4 = assign6470_body84_e8069_d_n4;
            locals.var_d_q_dn5 = assign6470_body84_e8069_d_n5;
            locals.var_d_q_dn6 = assign6470_body84_e8069_d_n6;
            locals.var_d_q_dn7 = assign6470_body84_e8069_d_n7;
            locals.var_d_q_dn8 = assign6470_body84_e8069_d_n8;
            locals.var_d_q_dn9 = assign6470_body84_e8069_d_n9;
            locals.var_d_q_rv = 0.0;
            let (assign6470_body85_e8079, assign6470_body85_e8079_d_n0, assign6470_body85_e8079_d_n1, assign6470_body85_e8079_d_n3, assign6470_body85_e8079_d_n4, assign6470_body85_e8079_d_n5, assign6470_body85_e8079_d_n6, assign6470_body85_e8079_d_n7, assign6470_body85_e8079_d_n8, assign6470_body85_e8079_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard141 != 0.0)) && (locals.var_guard142 == 0.0)) {
        let assign6470_body85_e8077: f64 = (-locals.var_d_q_max);
        (assign6470_body85_e8077, (-locals.var_d_q_max_dn0), (-locals.var_d_q_max_dn1), (-locals.var_d_q_max_dn3), (-locals.var_d_q_max_dn4), (-locals.var_d_q_max_dn5), (-locals.var_d_q_max_dn6), (-locals.var_d_q_max_dn7), (-locals.var_d_q_max_dn8), (-locals.var_d_q_max_dn9),)
    } else {
        (locals.var_d_q, locals.var_d_q_dn0, locals.var_d_q_dn1, locals.var_d_q_dn3, locals.var_d_q_dn4, locals.var_d_q_dn5, locals.var_d_q_dn6, locals.var_d_q_dn7, locals.var_d_q_dn8, locals.var_d_q_dn9,)
    }
};
            locals.var_d_q = assign6470_body85_e8079;
            locals.var_d_q_dn0 = assign6470_body85_e8079_d_n0;
            locals.var_d_q_dn1 = assign6470_body85_e8079_d_n1;
            locals.var_d_q_dn3 = assign6470_body85_e8079_d_n3;
            locals.var_d_q_dn4 = assign6470_body85_e8079_d_n4;
            locals.var_d_q_dn5 = assign6470_body85_e8079_d_n5;
            locals.var_d_q_dn6 = assign6470_body85_e8079_d_n6;
            locals.var_d_q_dn7 = assign6470_body85_e8079_d_n7;
            locals.var_d_q_dn8 = assign6470_body85_e8079_d_n8;
            locals.var_d_q_dn9 = assign6470_body85_e8079_d_n9;
            locals.var_d_q_rv = 0.0;
            let (assign6470_body86_e8085, assign6470_body86_e8085_d_n0, assign6470_body86_e8085_d_n1, assign6470_body86_e8085_d_n3, assign6470_body86_e8085_d_n4, assign6470_body86_e8085_d_n5, assign6470_body86_e8085_d_n6, assign6470_body86_e8085_d_n7, assign6470_body86_e8085_d_n8, assign6470_body86_e8085_d_n9,) = {
    if (locals.var_guard131 != 0.0) {
        let assign6470_body86_e8083: f64 = (locals.var_q_pt + locals.var_d_q);
        (assign6470_body86_e8083, (locals.var_q_pt_dn0 + locals.var_d_q_dn0), (locals.var_q_pt_dn1 + locals.var_d_q_dn1), (locals.var_q_pt_dn3 + locals.var_d_q_dn3), (locals.var_q_pt_dn4 + locals.var_d_q_dn4), (locals.var_q_pt_dn5 + locals.var_d_q_dn5), (locals.var_q_pt_dn6 + locals.var_d_q_dn6), (locals.var_q_pt_dn7 + locals.var_d_q_dn7), (locals.var_q_pt_dn8 + locals.var_d_q_dn8), (locals.var_q_pt_dn9 + locals.var_d_q_dn9),)
    } else {
        (locals.var_q_pt, locals.var_q_pt_dn0, locals.var_q_pt_dn1, locals.var_q_pt_dn3, locals.var_q_pt_dn4, locals.var_q_pt_dn5, locals.var_q_pt_dn6, locals.var_q_pt_dn7, locals.var_q_pt_dn8, locals.var_q_pt_dn9,)
    }
};
            locals.var_q_pt = assign6470_body86_e8085;
            locals.var_q_pt_dn0 = assign6470_body86_e8085_d_n0;
            locals.var_q_pt_dn1 = assign6470_body86_e8085_d_n1;
            locals.var_q_pt_dn3 = assign6470_body86_e8085_d_n3;
            locals.var_q_pt_dn4 = assign6470_body86_e8085_d_n4;
            locals.var_q_pt_dn5 = assign6470_body86_e8085_d_n5;
            locals.var_q_pt_dn6 = assign6470_body86_e8085_d_n6;
            locals.var_q_pt_dn7 = assign6470_body86_e8085_d_n7;
            locals.var_q_pt_dn8 = assign6470_body86_e8085_d_n8;
            locals.var_q_pt_dn9 = assign6470_body86_e8085_d_n9;
            locals.var_q_pt_rv = 0.0;
            let (assign6470_body87_e8091,) = {
    if (locals.var_guard131 != 0.0) {
        let assign6470_body87_e8089: f64 = (locals.var_l_it + 1.0);
        (assign6470_body87_e8089,)
    } else {
        (locals.var_l_it,)
    }
};
            locals.var_l_it = assign6470_body87_e8091;
            locals.var_l_it_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6480_e8097, assign6480_e8097_d_n0, assign6480_e8097_d_n1, assign6480_e8097_d_n3, assign6480_e8097_d_n4, assign6480_e8097_d_n5, assign6480_e8097_d_n6, assign6480_e8097_d_n7, assign6480_e8097_d_n8, assign6480_e8097_d_n9,) = {
    if (locals.var_guard131 != 0.0) {
        let assign6480_e8095: f64 = (locals.var_i_0f / locals.var_q_pt);
        (assign6480_e8095, (-((locals.var_i_0f * locals.var_q_pt_dn0) / (locals.var_q_pt * locals.var_q_pt))), (-((locals.var_i_0f * locals.var_q_pt_dn1) / (locals.var_q_pt * locals.var_q_pt))), (-((locals.var_i_0f * locals.var_q_pt_dn3) / (locals.var_q_pt * locals.var_q_pt))), (((locals.var_i_0f_dn4 * locals.var_q_pt) - (locals.var_i_0f * locals.var_q_pt_dn4)) / (locals.var_q_pt * locals.var_q_pt)), (-((locals.var_i_0f * locals.var_q_pt_dn5) / (locals.var_q_pt * locals.var_q_pt))), (((locals.var_i_0f_dn6 * locals.var_q_pt) - (locals.var_i_0f * locals.var_q_pt_dn6)) / (locals.var_q_pt * locals.var_q_pt)), (-((locals.var_i_0f * locals.var_q_pt_dn7) / (locals.var_q_pt * locals.var_q_pt))), (((locals.var_i_0f_dn8 * locals.var_q_pt) - (locals.var_i_0f * locals.var_q_pt_dn8)) / (locals.var_q_pt * locals.var_q_pt)), (-((locals.var_i_0f * locals.var_q_pt_dn9) / (locals.var_q_pt * locals.var_q_pt))),)
    } else {
        (locals.var_itf, locals.var_itf_dn0, locals.var_itf_dn1, locals.var_itf_dn3, locals.var_itf_dn4, locals.var_itf_dn5, locals.var_itf_dn6, locals.var_itf_dn7, locals.var_itf_dn8, locals.var_itf_dn9,)
    }
};
        locals.var_itf = assign6480_e8097;
        locals.var_itf_dn0 = assign6480_e8097_d_n0;
        locals.var_itf_dn1 = assign6480_e8097_d_n1;
        locals.var_itf_dn3 = assign6480_e8097_d_n3;
        locals.var_itf_dn4 = assign6480_e8097_d_n4;
        locals.var_itf_dn5 = assign6480_e8097_d_n5;
        locals.var_itf_dn6 = assign6480_e8097_d_n6;
        locals.var_itf_dn7 = assign6480_e8097_d_n7;
        locals.var_itf_dn8 = assign6480_e8097_d_n8;
        locals.var_itf_dn9 = assign6480_e8097_d_n9;
        locals.var_itf_rv = 0.0;

        let (assign6490_e8103, assign6490_e8103_d_n0, assign6490_e8103_d_n1, assign6490_e8103_d_n3, assign6490_e8103_d_n4, assign6490_e8103_d_n5, assign6490_e8103_d_n6, assign6490_e8103_d_n7, assign6490_e8103_d_n8, assign6490_e8103_d_n9,) = {
    if (locals.var_guard131 != 0.0) {
        let assign6490_e8101: f64 = (locals.var_i_0r / locals.var_q_pt);
        (assign6490_e8101, (-((locals.var_i_0r * locals.var_q_pt_dn0) / (locals.var_q_pt * locals.var_q_pt))), (-((locals.var_i_0r * locals.var_q_pt_dn1) / (locals.var_q_pt * locals.var_q_pt))), (-((locals.var_i_0r * locals.var_q_pt_dn3) / (locals.var_q_pt * locals.var_q_pt))), (((locals.var_i_0r_dn4 * locals.var_q_pt) - (locals.var_i_0r * locals.var_q_pt_dn4)) / (locals.var_q_pt * locals.var_q_pt)), (((locals.var_i_0r_dn5 * locals.var_q_pt) - (locals.var_i_0r * locals.var_q_pt_dn5)) / (locals.var_q_pt * locals.var_q_pt)), (-((locals.var_i_0r * locals.var_q_pt_dn6) / (locals.var_q_pt * locals.var_q_pt))), (-((locals.var_i_0r * locals.var_q_pt_dn7) / (locals.var_q_pt * locals.var_q_pt))), (((locals.var_i_0r_dn8 * locals.var_q_pt) - (locals.var_i_0r * locals.var_q_pt_dn8)) / (locals.var_q_pt * locals.var_q_pt)), (-((locals.var_i_0r * locals.var_q_pt_dn9) / (locals.var_q_pt * locals.var_q_pt))),)
    } else {
        (locals.var_itr, locals.var_itr_dn0, locals.var_itr_dn1, locals.var_itr_dn3, locals.var_itr_dn4, locals.var_itr_dn5, locals.var_itr_dn6, locals.var_itr_dn7, locals.var_itr_dn8, locals.var_itr_dn9,)
    }
};
        locals.var_itr = assign6490_e8103;
        locals.var_itr_dn0 = assign6490_e8103_d_n0;
        locals.var_itr_dn1 = assign6490_e8103_d_n1;
        locals.var_itr_dn3 = assign6490_e8103_d_n3;
        locals.var_itr_dn4 = assign6490_e8103_d_n4;
        locals.var_itr_dn5 = assign6490_e8103_d_n5;
        locals.var_itr_dn6 = assign6490_e8103_d_n6;
        locals.var_itr_dn7 = assign6490_e8103_d_n7;
        locals.var_itr_dn8 = assign6490_e8103_d_n8;
        locals.var_itr_dn9 = assign6490_e8103_d_n9;
        locals.var_itr_rv = 0.0;

        let (assign6500_e8107, assign6500_e8107_d_n0, assign6500_e8107_d_n1, assign6500_e8107_d_n3, assign6500_e8107_d_n4, assign6500_e8107_d_n5, assign6500_e8107_d_n6, assign6500_e8107_d_n7, assign6500_e8107_d_n8, assign6500_e8107_d_n9,) = {
    if (locals.var_guard131 != 0.0) {
        (locals.var_t_f0, 0.0, 0.0, 0.0, locals.var_t_f0_dn4, locals.var_t_f0_dn5, 0.0, 0.0, locals.var_t_f0_dn8, 0.0,)
    } else {
        (locals.var_tf, locals.var_tf_dn0, locals.var_tf_dn1, locals.var_tf_dn3, locals.var_tf_dn4, locals.var_tf_dn5, locals.var_tf_dn6, locals.var_tf_dn7, locals.var_tf_dn8, locals.var_tf_dn9,)
    }
};
        locals.var_tf = assign6500_e8107;
        locals.var_tf_dn0 = assign6500_e8107_d_n0;
        locals.var_tf_dn1 = assign6500_e8107_d_n1;
        locals.var_tf_dn3 = assign6500_e8107_d_n3;
        locals.var_tf_dn4 = assign6500_e8107_d_n4;
        locals.var_tf_dn5 = assign6500_e8107_d_n5;
        locals.var_tf_dn6 = assign6500_e8107_d_n6;
        locals.var_tf_dn7 = assign6500_e8107_d_n7;
        locals.var_tf_dn8 = assign6500_e8107_d_n8;
        locals.var_tf_dn9 = assign6500_e8107_d_n9;
        locals.var_tf_rv = 0.0;

        let (assign6510_e8113, assign6510_e8113_d_n0, assign6510_e8113_d_n1, assign6510_e8113_d_n3, assign6510_e8113_d_n4, assign6510_e8113_d_n5, assign6510_e8113_d_n6, assign6510_e8113_d_n7, assign6510_e8113_d_n8, assign6510_e8113_d_n9,) = {
    if (locals.var_guard131 != 0.0) {
        let assign6510_e8111: f64 = (locals.var_t_f0 * locals.var_itf);
        (assign6510_e8111, (locals.var_t_f0 * locals.var_itf_dn0), (locals.var_t_f0 * locals.var_itf_dn1), (locals.var_t_f0 * locals.var_itf_dn3), ((locals.var_t_f0_dn4 * locals.var_itf) + (locals.var_t_f0 * locals.var_itf_dn4)), ((locals.var_t_f0_dn5 * locals.var_itf) + (locals.var_t_f0 * locals.var_itf_dn5)), (locals.var_t_f0 * locals.var_itf_dn6), (locals.var_t_f0 * locals.var_itf_dn7), ((locals.var_t_f0_dn8 * locals.var_itf) + (locals.var_t_f0 * locals.var_itf_dn8)), (locals.var_t_f0 * locals.var_itf_dn9),)
    } else {
        (locals.var_qf, locals.var_qf_dn0, locals.var_qf_dn1, locals.var_qf_dn3, locals.var_qf_dn4, locals.var_qf_dn5, locals.var_qf_dn6, locals.var_qf_dn7, locals.var_qf_dn8, locals.var_qf_dn9,)
    }
};
        locals.var_qf = assign6510_e8113;
        locals.var_qf_dn0 = assign6510_e8113_d_n0;
        locals.var_qf_dn1 = assign6510_e8113_d_n1;
        locals.var_qf_dn3 = assign6510_e8113_d_n3;
        locals.var_qf_dn4 = assign6510_e8113_d_n4;
        locals.var_qf_dn5 = assign6510_e8113_d_n5;
        locals.var_qf_dn6 = assign6510_e8113_d_n6;
        locals.var_qf_dn7 = assign6510_e8113_d_n7;
        locals.var_qf_dn8 = assign6510_e8113_d_n8;
        locals.var_qf_dn9 = assign6510_e8113_d_n9;
        locals.var_qf_rv = 0.0;

        let assign6520_e8116: f64 = if p.p0 >= 310.0 { 1.0 } else { 0.0 };
        locals.var_guard143 = assign6520_e8116;
        locals.var_guard143_rv = 0.0;

        let (assign6530_e8124, assign6530_e8124_d_n0, assign6530_e8124_d_n1, assign6530_e8124_d_n3, assign6530_e8124_d_n4, assign6530_e8124_d_n5, assign6530_e8124_d_n6, assign6530_e8124_d_n7, assign6530_e8124_d_n8, assign6530_e8124_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard143 != 0.0)) {
        let assign6530_e8122: f64 = (locals.var_hf0_t * locals.var_t0_t);
        (assign6530_e8122, 0.0, 0.0, 0.0, ((locals.var_hf0_t_dn4 * locals.var_t0_t) + (locals.var_hf0_t * locals.var_t0_t_dn4)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t_ft, locals.var_t_ft_dn0, locals.var_t_ft_dn1, locals.var_t_ft_dn3, locals.var_t_ft_dn4, locals.var_t_ft_dn5, locals.var_t_ft_dn6, locals.var_t_ft_dn7, locals.var_t_ft_dn8, locals.var_t_ft_dn9,)
    }
};
        locals.var_t_ft = assign6530_e8124;
        locals.var_t_ft_dn0 = assign6530_e8124_d_n0;
        locals.var_t_ft_dn1 = assign6530_e8124_d_n1;
        locals.var_t_ft_dn3 = assign6530_e8124_d_n3;
        locals.var_t_ft_dn4 = assign6530_e8124_d_n4;
        locals.var_t_ft_dn5 = assign6530_e8124_d_n5;
        locals.var_t_ft_dn6 = assign6530_e8124_d_n6;
        locals.var_t_ft_dn7 = assign6530_e8124_d_n7;
        locals.var_t_ft_dn8 = assign6530_e8124_d_n8;
        locals.var_t_ft_dn9 = assign6530_e8124_d_n9;
        locals.var_t_ft_rv = 0.0;

        let (assign6540_e8132, assign6540_e8132_d_n0, assign6540_e8132_d_n1, assign6540_e8132_d_n3, assign6540_e8132_d_n4, assign6540_e8132_d_n5, assign6540_e8132_d_n6, assign6540_e8132_d_n7, assign6540_e8132_d_n8, assign6540_e8132_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard143 != 0.0)) {
        let assign6540_e8130: f64 = (locals.var_t_ft * locals.var_itf);
        (assign6540_e8130, ((locals.var_t_ft_dn0 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn0)), ((locals.var_t_ft_dn1 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn1)), ((locals.var_t_ft_dn3 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn3)), ((locals.var_t_ft_dn4 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn4)), ((locals.var_t_ft_dn5 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn5)), ((locals.var_t_ft_dn6 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn6)), ((locals.var_t_ft_dn7 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn7)), ((locals.var_t_ft_dn8 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn8)), ((locals.var_t_ft_dn9 * locals.var_itf) + (locals.var_t_ft * locals.var_itf_dn9)),)
    } else {
        (locals.var_q_ft, locals.var_q_ft_dn0, locals.var_q_ft_dn1, locals.var_q_ft_dn3, locals.var_q_ft_dn4, locals.var_q_ft_dn5, locals.var_q_ft_dn6, locals.var_q_ft_dn7, locals.var_q_ft_dn8, locals.var_q_ft_dn9,)
    }
};
        locals.var_q_ft = assign6540_e8132;
        locals.var_q_ft_dn0 = assign6540_e8132_d_n0;
        locals.var_q_ft_dn1 = assign6540_e8132_d_n1;
        locals.var_q_ft_dn3 = assign6540_e8132_d_n3;
        locals.var_q_ft_dn4 = assign6540_e8132_d_n4;
        locals.var_q_ft_dn5 = assign6540_e8132_d_n5;
        locals.var_q_ft_dn6 = assign6540_e8132_d_n6;
        locals.var_q_ft_dn7 = assign6540_e8132_d_n7;
        locals.var_q_ft_dn8 = assign6540_e8132_d_n8;
        locals.var_q_ft_dn9 = assign6540_e8132_d_n9;
        locals.var_q_ft_rv = 0.0;

        let (assign6550_e8141, assign6550_e8141_d_n0, assign6550_e8141_d_n1, assign6550_e8141_d_n3, assign6550_e8141_d_n4, assign6550_e8141_d_n5, assign6550_e8141_d_n6, assign6550_e8141_d_n7, assign6550_e8141_d_n8, assign6550_e8141_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard143 == 0.0)) {
        let assign6550_e8139: f64 = (locals.var_hf0_t * locals.var_qf);
        (assign6550_e8139, (locals.var_hf0_t * locals.var_qf_dn0), (locals.var_hf0_t * locals.var_qf_dn1), (locals.var_hf0_t * locals.var_qf_dn3), ((locals.var_hf0_t_dn4 * locals.var_qf) + (locals.var_hf0_t * locals.var_qf_dn4)), (locals.var_hf0_t * locals.var_qf_dn5), (locals.var_hf0_t * locals.var_qf_dn6), (locals.var_hf0_t * locals.var_qf_dn7), (locals.var_hf0_t * locals.var_qf_dn8), (locals.var_hf0_t * locals.var_qf_dn9),)
    } else {
        (locals.var_q_ft, locals.var_q_ft_dn0, locals.var_q_ft_dn1, locals.var_q_ft_dn3, locals.var_q_ft_dn4, locals.var_q_ft_dn5, locals.var_q_ft_dn6, locals.var_q_ft_dn7, locals.var_q_ft_dn8, locals.var_q_ft_dn9,)
    }
};
        locals.var_q_ft = assign6550_e8141;
        locals.var_q_ft_dn0 = assign6550_e8141_d_n0;
        locals.var_q_ft_dn1 = assign6550_e8141_d_n1;
        locals.var_q_ft_dn3 = assign6550_e8141_d_n3;
        locals.var_q_ft_dn4 = assign6550_e8141_d_n4;
        locals.var_q_ft_dn5 = assign6550_e8141_d_n5;
        locals.var_q_ft_dn6 = assign6550_e8141_d_n6;
        locals.var_q_ft_dn7 = assign6550_e8141_d_n7;
        locals.var_q_ft_dn8 = assign6550_e8141_d_n8;
        locals.var_q_ft_dn9 = assign6550_e8141_d_n9;
        locals.var_q_ft_rv = 0.0;

        let (assign6560_e8150, assign6560_e8150_d_n0, assign6560_e8150_d_n1, assign6560_e8150_d_n3, assign6560_e8150_d_n4, assign6560_e8150_d_n5, assign6560_e8150_d_n6, assign6560_e8150_d_n7, assign6560_e8150_d_n8, assign6560_e8150_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard143 == 0.0)) {
        let assign6560_e8148: f64 = (locals.var_hf0_t * locals.var_tf);
        (assign6560_e8148, (locals.var_hf0_t * locals.var_tf_dn0), (locals.var_hf0_t * locals.var_tf_dn1), (locals.var_hf0_t * locals.var_tf_dn3), ((locals.var_hf0_t_dn4 * locals.var_tf) + (locals.var_hf0_t * locals.var_tf_dn4)), (locals.var_hf0_t * locals.var_tf_dn5), (locals.var_hf0_t * locals.var_tf_dn6), (locals.var_hf0_t * locals.var_tf_dn7), (locals.var_hf0_t * locals.var_tf_dn8), (locals.var_hf0_t * locals.var_tf_dn9),)
    } else {
        (locals.var_t_ft, locals.var_t_ft_dn0, locals.var_t_ft_dn1, locals.var_t_ft_dn3, locals.var_t_ft_dn4, locals.var_t_ft_dn5, locals.var_t_ft_dn6, locals.var_t_ft_dn7, locals.var_t_ft_dn8, locals.var_t_ft_dn9,)
    }
};
        locals.var_t_ft = assign6560_e8150;
        locals.var_t_ft_dn0 = assign6560_e8150_d_n0;
        locals.var_t_ft_dn1 = assign6560_e8150_d_n1;
        locals.var_t_ft_dn3 = assign6560_e8150_d_n3;
        locals.var_t_ft_dn4 = assign6560_e8150_d_n4;
        locals.var_t_ft_dn5 = assign6560_e8150_d_n5;
        locals.var_t_ft_dn6 = assign6560_e8150_d_n6;
        locals.var_t_ft_dn7 = assign6560_e8150_d_n7;
        locals.var_t_ft_dn8 = assign6560_e8150_d_n8;
        locals.var_t_ft_dn9 = assign6560_e8150_d_n9;
        locals.var_t_ft_rv = 0.0;

        let (assign6570_e8154, assign6570_e8154_d_n0, assign6570_e8154_d_n1, assign6570_e8154_d_n3, assign6570_e8154_d_n4, assign6570_e8154_d_n5, assign6570_e8154_d_n6, assign6570_e8154_d_n7, assign6570_e8154_d_n8, assign6570_e8154_d_n9,) = {
    if (locals.var_guard131 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_bf, locals.var_q_bf_dn0, locals.var_q_bf_dn1, locals.var_q_bf_dn3, locals.var_q_bf_dn4, locals.var_q_bf_dn5, locals.var_q_bf_dn6, locals.var_q_bf_dn7, locals.var_q_bf_dn8, locals.var_q_bf_dn9,)
    }
};
        locals.var_q_bf = assign6570_e8154;
        locals.var_q_bf_dn0 = assign6570_e8154_d_n0;
        locals.var_q_bf_dn1 = assign6570_e8154_d_n1;
        locals.var_q_bf_dn3 = assign6570_e8154_d_n3;
        locals.var_q_bf_dn4 = assign6570_e8154_d_n4;
        locals.var_q_bf_dn5 = assign6570_e8154_d_n5;
        locals.var_q_bf_dn6 = assign6570_e8154_d_n6;
        locals.var_q_bf_dn7 = assign6570_e8154_d_n7;
        locals.var_q_bf_dn8 = assign6570_e8154_d_n8;
        locals.var_q_bf_dn9 = assign6570_e8154_d_n9;
        locals.var_q_bf_rv = 0.0;

        let assign6580_e8158: f64 = (1e-6 * locals.var_ick);
        let assign6580_e8163: f64 = if ((locals.var_itf >= assign6580_e8158) || (p.p0 >= 320.0)) { 1.0 } else { 0.0 };
        locals.var_guard144 = assign6580_e8163;
        locals.var_guard144_rv = 0.0;

        let (assign6590_e8171, assign6590_e8171_d_n0, assign6590_e8171_d_n1, assign6590_e8171_d_n3, assign6590_e8171_d_n4, assign6590_e8171_d_n5, assign6590_e8171_d_n6, assign6590_e8171_d_n7, assign6590_e8171_d_n8, assign6590_e8171_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign6590_e8169: f64 = (locals.var_itf / locals.var_ick);
        (assign6590_e8169, (locals.var_itf_dn0 / locals.var_ick), (locals.var_itf_dn1 / locals.var_ick), (locals.var_itf_dn3 / locals.var_ick), (((locals.var_itf_dn4 * locals.var_ick) - (locals.var_itf * locals.var_ick_dn4)) / (locals.var_ick * locals.var_ick)), (((locals.var_itf_dn5 * locals.var_ick) - (locals.var_itf * locals.var_ick_dn5)) / (locals.var_ick * locals.var_ick)), (((locals.var_itf_dn6 * locals.var_ick) - (locals.var_itf * locals.var_ick_dn6)) / (locals.var_ick * locals.var_ick)), (locals.var_itf_dn7 / locals.var_ick), (((locals.var_itf_dn8 * locals.var_ick) - (locals.var_itf * locals.var_ick_dn8)) / (locals.var_ick * locals.var_ick)), (locals.var_itf_dn9 / locals.var_ick),)
    } else {
        (locals.var_ffitf_ick, locals.var_ffitf_ick_dn0, locals.var_ffitf_ick_dn1, locals.var_ffitf_ick_dn3, locals.var_ffitf_ick_dn4, locals.var_ffitf_ick_dn5, locals.var_ffitf_ick_dn6, locals.var_ffitf_ick_dn7, locals.var_ffitf_ick_dn8, locals.var_ffitf_ick_dn9,)
    }
};
        locals.var_ffitf_ick = assign6590_e8171;
        locals.var_ffitf_ick_dn0 = assign6590_e8171_d_n0;
        locals.var_ffitf_ick_dn1 = assign6590_e8171_d_n1;
        locals.var_ffitf_ick_dn3 = assign6590_e8171_d_n3;
        locals.var_ffitf_ick_dn4 = assign6590_e8171_d_n4;
        locals.var_ffitf_ick_dn5 = assign6590_e8171_d_n5;
        locals.var_ffitf_ick_dn6 = assign6590_e8171_d_n6;
        locals.var_ffitf_ick_dn7 = assign6590_e8171_d_n7;
        locals.var_ffitf_ick_dn8 = assign6590_e8171_d_n8;
        locals.var_ffitf_ick_dn9 = assign6590_e8171_d_n9;
        locals.var_ffitf_ick_rv = 0.0;

        let (assign6600_e8183, assign6600_e8183_d_n0, assign6600_e8183_d_n1, assign6600_e8183_d_n3, assign6600_e8183_d_n4, assign6600_e8183_d_n5, assign6600_e8183_d_n6, assign6600_e8183_d_n7, assign6600_e8183_d_n8, assign6600_e8183_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign6600_e8178: f64 = (locals.var_ffitf_ick).ln();
        let assign6600_e8179: f64 = (p.p70 * assign6600_e8178);
        let assign6600_e8180: f64 = (assign6600_e8179).exp();
        let assign6600_e8181: f64 = (locals.var_tef0_t * assign6600_e8180);
        (assign6600_e8181, (locals.var_tef0_t * (assign6600_e8180 * (p.p70 * (locals.var_ffitf_ick_dn0 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign6600_e8180 * (p.p70 * (locals.var_ffitf_ick_dn1 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign6600_e8180 * (p.p70 * (locals.var_ffitf_ick_dn3 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign6600_e8180 * (p.p70 * (locals.var_ffitf_ick_dn4 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign6600_e8180 * (p.p70 * (locals.var_ffitf_ick_dn5 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign6600_e8180 * (p.p70 * (locals.var_ffitf_ick_dn6 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign6600_e8180 * (p.p70 * (locals.var_ffitf_ick_dn7 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign6600_e8180 * (p.p70 * (locals.var_ffitf_ick_dn8 / locals.var_ffitf_ick)))), (locals.var_tef0_t * (assign6600_e8180 * (p.p70 * (locals.var_ffitf_ick_dn9 / locals.var_ffitf_ick)))),)
    } else {
        (locals.var_ffdtef, locals.var_ffdtef_dn0, locals.var_ffdtef_dn1, locals.var_ffdtef_dn3, locals.var_ffdtef_dn4, locals.var_ffdtef_dn5, locals.var_ffdtef_dn6, locals.var_ffdtef_dn7, locals.var_ffdtef_dn8, locals.var_ffdtef_dn9,)
    }
};
        locals.var_ffdtef = assign6600_e8183;
        locals.var_ffdtef_dn0 = assign6600_e8183_d_n0;
        locals.var_ffdtef_dn1 = assign6600_e8183_d_n1;
        locals.var_ffdtef_dn3 = assign6600_e8183_d_n3;
        locals.var_ffdtef_dn4 = assign6600_e8183_d_n4;
        locals.var_ffdtef_dn5 = assign6600_e8183_d_n5;
        locals.var_ffdtef_dn6 = assign6600_e8183_d_n6;
        locals.var_ffdtef_dn7 = assign6600_e8183_d_n7;
        locals.var_ffdtef_dn8 = assign6600_e8183_d_n8;
        locals.var_ffdtef_dn9 = assign6600_e8183_d_n9;
        locals.var_ffdtef_rv = 0.0;

        let (assign6610_e8195, assign6610_e8195_d_n0, assign6610_e8195_d_n1, assign6610_e8195_d_n3, assign6610_e8195_d_n4, assign6610_e8195_d_n5, assign6610_e8195_d_n6, assign6610_e8195_d_n7, assign6610_e8195_d_n8, assign6610_e8195_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign6610_e8189: f64 = (locals.var_ffdtef * locals.var_itf);
        let assign6610_e8192: f64 = (1.0 + p.p70);
        let assign6610_e8193: f64 = (assign6610_e8189 / assign6610_e8192);
        (assign6610_e8193, (((locals.var_ffdtef_dn0 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn0)) / assign6610_e8192), (((locals.var_ffdtef_dn1 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn1)) / assign6610_e8192), (((locals.var_ffdtef_dn3 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn3)) / assign6610_e8192), (((locals.var_ffdtef_dn4 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn4)) / assign6610_e8192), (((locals.var_ffdtef_dn5 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn5)) / assign6610_e8192), (((locals.var_ffdtef_dn6 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn6)) / assign6610_e8192), (((locals.var_ffdtef_dn7 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn7)) / assign6610_e8192), (((locals.var_ffdtef_dn8 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn8)) / assign6610_e8192), (((locals.var_ffdtef_dn9 * locals.var_itf) + (locals.var_ffdtef * locals.var_itf_dn9)) / assign6610_e8192),)
    } else {
        (locals.var_ffdqef, locals.var_ffdqef_dn0, locals.var_ffdqef_dn1, locals.var_ffdqef_dn3, locals.var_ffdqef_dn4, locals.var_ffdqef_dn5, locals.var_ffdqef_dn6, locals.var_ffdqef_dn7, locals.var_ffdqef_dn8, locals.var_ffdqef_dn9,)
    }
};
        locals.var_ffdqef = assign6610_e8195;
        locals.var_ffdqef_dn0 = assign6610_e8195_d_n0;
        locals.var_ffdqef_dn1 = assign6610_e8195_d_n1;
        locals.var_ffdqef_dn3 = assign6610_e8195_d_n3;
        locals.var_ffdqef_dn4 = assign6610_e8195_d_n4;
        locals.var_ffdqef_dn5 = assign6610_e8195_d_n5;
        locals.var_ffdqef_dn6 = assign6610_e8195_d_n6;
        locals.var_ffdqef_dn7 = assign6610_e8195_d_n7;
        locals.var_ffdqef_dn8 = assign6610_e8195_d_n8;
        locals.var_ffdqef_dn9 = assign6610_e8195_d_n9;
        locals.var_ffdqef_rv = 0.0;

        let assign6620_e8200: f64 = (p.p75 / p.p74);
        let assign6620_e8201: f64 = (0.05 * assign6620_e8200);
        let assign6620_e8202: f64 = if p.p83 < assign6620_e8201 { 1.0 } else { 0.0 };
        locals.var_guard145 = assign6620_e8202;
        locals.var_guard145_rv = 0.0;

        let (assign6630_e8210, assign6630_e8210_d_n0, assign6630_e8210_d_n1, assign6630_e8210_d_n3, assign6630_e8210_d_n4, assign6630_e8210_d_n5, assign6630_e8210_d_n6, assign6630_e8210_d_n7, assign6630_e8210_d_n8, assign6630_e8210_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard145 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ffdvc, locals.var_ffdvc_dn0, locals.var_ffdvc_dn1, locals.var_ffdvc_dn3, locals.var_ffdvc_dn4, locals.var_ffdvc_dn5, locals.var_ffdvc_dn6, locals.var_ffdvc_dn7, locals.var_ffdvc_dn8, locals.var_ffdvc_dn9,)
    }
};
        locals.var_ffdvc = assign6630_e8210;
        locals.var_ffdvc_dn0 = assign6630_e8210_d_n0;
        locals.var_ffdvc_dn1 = assign6630_e8210_d_n1;
        locals.var_ffdvc_dn3 = assign6630_e8210_d_n3;
        locals.var_ffdvc_dn4 = assign6630_e8210_d_n4;
        locals.var_ffdvc_dn5 = assign6630_e8210_d_n5;
        locals.var_ffdvc_dn6 = assign6630_e8210_d_n6;
        locals.var_ffdvc_dn7 = assign6630_e8210_d_n7;
        locals.var_ffdvc_dn8 = assign6630_e8210_d_n8;
        locals.var_ffdvc_dn9 = assign6630_e8210_d_n9;
        locals.var_ffdvc_rv = 0.0;

        let (assign6640_e8218, assign6640_e8218_d_n0, assign6640_e8218_d_n1, assign6640_e8218_d_n3, assign6640_e8218_d_n4, assign6640_e8218_d_n5, assign6640_e8218_d_n6, assign6640_e8218_d_n7, assign6640_e8218_d_n8, assign6640_e8218_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard145 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ffdvc_ditf, locals.var_ffdvc_ditf_dn0, locals.var_ffdvc_ditf_dn1, locals.var_ffdvc_ditf_dn3, locals.var_ffdvc_ditf_dn4, locals.var_ffdvc_ditf_dn5, locals.var_ffdvc_ditf_dn6, locals.var_ffdvc_ditf_dn7, locals.var_ffdvc_ditf_dn8, locals.var_ffdvc_ditf_dn9,)
    }
};
        locals.var_ffdvc_ditf = assign6640_e8218;
        locals.var_ffdvc_ditf_dn0 = assign6640_e8218_d_n0;
        locals.var_ffdvc_ditf_dn1 = assign6640_e8218_d_n1;
        locals.var_ffdvc_ditf_dn3 = assign6640_e8218_d_n3;
        locals.var_ffdvc_ditf_dn4 = assign6640_e8218_d_n4;
        locals.var_ffdvc_ditf_dn5 = assign6640_e8218_d_n5;
        locals.var_ffdvc_ditf_dn6 = assign6640_e8218_d_n6;
        locals.var_ffdvc_ditf_dn7 = assign6640_e8218_d_n7;
        locals.var_ffdvc_ditf_dn8 = assign6640_e8218_d_n8;
        locals.var_ffdvc_ditf_dn9 = assign6640_e8218_d_n9;
        locals.var_ffdvc_ditf_rv = 0.0;

        let (assign6650_e8231, assign6650_e8231_d_n0, assign6650_e8231_d_n1, assign6650_e8231_d_n3, assign6650_e8231_d_n4, assign6650_e8231_d_n5, assign6650_e8231_d_n6, assign6650_e8231_d_n7, assign6650_e8231_d_n8, assign6650_e8231_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard145 == 0.0)) {
        let assign6650_e8227: f64 = (locals.var_itf - locals.var_ick);
        let assign6650_e8229: f64 = (assign6650_e8227 / p.p83);
        (assign6650_e8229, (locals.var_itf_dn0 / p.p83), (locals.var_itf_dn1 / p.p83), (locals.var_itf_dn3 / p.p83), ((locals.var_itf_dn4 - locals.var_ick_dn4) / p.p83), ((locals.var_itf_dn5 - locals.var_ick_dn5) / p.p83), ((locals.var_itf_dn6 - locals.var_ick_dn6) / p.p83), (locals.var_itf_dn7 / p.p83), ((locals.var_itf_dn8 - locals.var_ick_dn8) / p.p83), (locals.var_itf_dn9 / p.p83),)
    } else {
        (locals.var_ffib, locals.var_ffib_dn0, locals.var_ffib_dn1, locals.var_ffib_dn3, locals.var_ffib_dn4, locals.var_ffib_dn5, locals.var_ffib_dn6, locals.var_ffib_dn7, locals.var_ffib_dn8, locals.var_ffib_dn9,)
    }
};
        locals.var_ffib = assign6650_e8231;
        locals.var_ffib_dn0 = assign6650_e8231_d_n0;
        locals.var_ffib_dn1 = assign6650_e8231_d_n1;
        locals.var_ffib_dn3 = assign6650_e8231_d_n3;
        locals.var_ffib_dn4 = assign6650_e8231_d_n4;
        locals.var_ffib_dn5 = assign6650_e8231_d_n5;
        locals.var_ffib_dn6 = assign6650_e8231_d_n6;
        locals.var_ffib_dn7 = assign6650_e8231_d_n7;
        locals.var_ffib_dn8 = assign6650_e8231_d_n8;
        locals.var_ffib_dn9 = assign6650_e8231_d_n9;
        locals.var_ffib_rv = 0.0;

        let assign6660_e8234: f64 = (-10000000000.0);
        let assign6660_e8235: f64 = if locals.var_ffib < assign6660_e8234 { 1.0 } else { 0.0 };
        locals.var_guard146 = assign6660_e8235;
        locals.var_guard146_rv = 0.0;

        let (assign6670_e8247, assign6670_e8247_d_n0, assign6670_e8247_d_n1, assign6670_e8247_d_n3, assign6670_e8247_d_n4, assign6670_e8247_d_n5, assign6670_e8247_d_n6, assign6670_e8247_d_n7, assign6670_e8247_d_n8, assign6670_e8247_d_n9,) = {
    if ((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard145 == 0.0)) && (locals.var_guard146 != 0.0)) {
        let assign6670_e8245: f64 = (-10000000000.0);
        (assign6670_e8245, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ffib, locals.var_ffib_dn0, locals.var_ffib_dn1, locals.var_ffib_dn3, locals.var_ffib_dn4, locals.var_ffib_dn5, locals.var_ffib_dn6, locals.var_ffib_dn7, locals.var_ffib_dn8, locals.var_ffib_dn9,)
    }
};
        locals.var_ffib = assign6670_e8247;
        locals.var_ffib_dn0 = assign6670_e8247_d_n0;
        locals.var_ffib_dn1 = assign6670_e8247_d_n1;
        locals.var_ffib_dn3 = assign6670_e8247_d_n3;
        locals.var_ffib_dn4 = assign6670_e8247_d_n4;
        locals.var_ffib_dn5 = assign6670_e8247_d_n5;
        locals.var_ffib_dn6 = assign6670_e8247_d_n6;
        locals.var_ffib_dn7 = assign6670_e8247_d_n7;
        locals.var_ffib_dn8 = assign6670_e8247_d_n8;
        locals.var_ffib_dn9 = assign6670_e8247_d_n9;
        locals.var_ffib_rv = 0.0;

        let (assign6680_e8261, assign6680_e8261_d_n0, assign6680_e8261_d_n1, assign6680_e8261_d_n3, assign6680_e8261_d_n4, assign6680_e8261_d_n5, assign6680_e8261_d_n6, assign6680_e8261_d_n7, assign6680_e8261_d_n8, assign6680_e8261_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard145 == 0.0)) {
        let assign6680_e8256: f64 = (locals.var_ffib * locals.var_ffib);
        let assign6680_e8258: f64 = (assign6680_e8256 + p.p84);
        let assign6680_e8259: f64 = (assign6680_e8258).sqrt();
        (assign6680_e8259, (((locals.var_ffib_dn0 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn0)) / (2.0 * assign6680_e8259)), (((locals.var_ffib_dn1 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn1)) / (2.0 * assign6680_e8259)), (((locals.var_ffib_dn3 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn3)) / (2.0 * assign6680_e8259)), (((locals.var_ffib_dn4 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn4)) / (2.0 * assign6680_e8259)), (((locals.var_ffib_dn5 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn5)) / (2.0 * assign6680_e8259)), (((locals.var_ffib_dn6 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn6)) / (2.0 * assign6680_e8259)), (((locals.var_ffib_dn7 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn7)) / (2.0 * assign6680_e8259)), (((locals.var_ffib_dn8 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn8)) / (2.0 * assign6680_e8259)), (((locals.var_ffib_dn9 * locals.var_ffib) + (locals.var_ffib * locals.var_ffib_dn9)) / (2.0 * assign6680_e8259)),)
    } else {
        (locals.var_fffcbar, locals.var_fffcbar_dn0, locals.var_fffcbar_dn1, locals.var_fffcbar_dn3, locals.var_fffcbar_dn4, locals.var_fffcbar_dn5, locals.var_fffcbar_dn6, locals.var_fffcbar_dn7, locals.var_fffcbar_dn8, locals.var_fffcbar_dn9,)
    }
};
        locals.var_fffcbar = assign6680_e8261;
        locals.var_fffcbar_dn0 = assign6680_e8261_d_n0;
        locals.var_fffcbar_dn1 = assign6680_e8261_d_n1;
        locals.var_fffcbar_dn3 = assign6680_e8261_d_n3;
        locals.var_fffcbar_dn4 = assign6680_e8261_d_n4;
        locals.var_fffcbar_dn5 = assign6680_e8261_d_n5;
        locals.var_fffcbar_dn6 = assign6680_e8261_d_n6;
        locals.var_fffcbar_dn7 = assign6680_e8261_d_n7;
        locals.var_fffcbar_dn8 = assign6680_e8261_d_n8;
        locals.var_fffcbar_dn9 = assign6680_e8261_d_n9;
        locals.var_fffcbar_rv = 0.0;

        let (assign6690_e8278, assign6690_e8278_d_n0, assign6690_e8278_d_n1, assign6690_e8278_d_n3, assign6690_e8278_d_n4, assign6690_e8278_d_n5, assign6690_e8278_d_n6, assign6690_e8278_d_n7, assign6690_e8278_d_n8, assign6690_e8278_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard145 == 0.0)) {
        let assign6690_e8270: f64 = (-2.0);
        let assign6690_e8273: f64 = (locals.var_ffib + locals.var_fffcbar);
        let assign6690_e8274: f64 = (assign6690_e8270 / assign6690_e8273);
        let assign6690_e8275: f64 = (assign6690_e8274).exp();
        let assign6690_e8276: f64 = (p.p82 * assign6690_e8275);
        (assign6690_e8276, (p.p82 * (assign6690_e8275 * (-((assign6690_e8270 * (locals.var_ffib_dn0 + locals.var_fffcbar_dn0)) / (assign6690_e8273 * assign6690_e8273))))), (p.p82 * (assign6690_e8275 * (-((assign6690_e8270 * (locals.var_ffib_dn1 + locals.var_fffcbar_dn1)) / (assign6690_e8273 * assign6690_e8273))))), (p.p82 * (assign6690_e8275 * (-((assign6690_e8270 * (locals.var_ffib_dn3 + locals.var_fffcbar_dn3)) / (assign6690_e8273 * assign6690_e8273))))), (p.p82 * (assign6690_e8275 * (-((assign6690_e8270 * (locals.var_ffib_dn4 + locals.var_fffcbar_dn4)) / (assign6690_e8273 * assign6690_e8273))))), (p.p82 * (assign6690_e8275 * (-((assign6690_e8270 * (locals.var_ffib_dn5 + locals.var_fffcbar_dn5)) / (assign6690_e8273 * assign6690_e8273))))), (p.p82 * (assign6690_e8275 * (-((assign6690_e8270 * (locals.var_ffib_dn6 + locals.var_fffcbar_dn6)) / (assign6690_e8273 * assign6690_e8273))))), (p.p82 * (assign6690_e8275 * (-((assign6690_e8270 * (locals.var_ffib_dn7 + locals.var_fffcbar_dn7)) / (assign6690_e8273 * assign6690_e8273))))), (p.p82 * (assign6690_e8275 * (-((assign6690_e8270 * (locals.var_ffib_dn8 + locals.var_fffcbar_dn8)) / (assign6690_e8273 * assign6690_e8273))))), (p.p82 * (assign6690_e8275 * (-((assign6690_e8270 * (locals.var_ffib_dn9 + locals.var_fffcbar_dn9)) / (assign6690_e8273 * assign6690_e8273))))),)
    } else {
        (locals.var_ffdvc, locals.var_ffdvc_dn0, locals.var_ffdvc_dn1, locals.var_ffdvc_dn3, locals.var_ffdvc_dn4, locals.var_ffdvc_dn5, locals.var_ffdvc_dn6, locals.var_ffdvc_dn7, locals.var_ffdvc_dn8, locals.var_ffdvc_dn9,)
    }
};
        locals.var_ffdvc = assign6690_e8278;
        locals.var_ffdvc_dn0 = assign6690_e8278_d_n0;
        locals.var_ffdvc_dn1 = assign6690_e8278_d_n1;
        locals.var_ffdvc_dn3 = assign6690_e8278_d_n3;
        locals.var_ffdvc_dn4 = assign6690_e8278_d_n4;
        locals.var_ffdvc_dn5 = assign6690_e8278_d_n5;
        locals.var_ffdvc_dn6 = assign6690_e8278_d_n6;
        locals.var_ffdvc_dn7 = assign6690_e8278_d_n7;
        locals.var_ffdvc_dn8 = assign6690_e8278_d_n8;
        locals.var_ffdvc_dn9 = assign6690_e8278_d_n9;
        locals.var_ffdvc_rv = 0.0;

        let (assign6700_e8297, assign6700_e8297_d_n0, assign6700_e8297_d_n1, assign6700_e8297_d_n3, assign6700_e8297_d_n4, assign6700_e8297_d_n5, assign6700_e8297_d_n6, assign6700_e8297_d_n7, assign6700_e8297_d_n8, assign6700_e8297_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard145 == 0.0)) {
        let assign6700_e8287: f64 = (2.0 * locals.var_ffdvc);
        let assign6700_e8290: f64 = (p.p83 * locals.var_fffcbar);
        let assign6700_e8293: f64 = (locals.var_ffib + locals.var_fffcbar);
        let assign6700_e8294: f64 = (assign6700_e8290 * assign6700_e8293);
        let assign6700_e8295: f64 = (assign6700_e8287 / assign6700_e8294);
        (assign6700_e8295, ((((2.0 * locals.var_ffdvc_dn0) * assign6700_e8294) - (assign6700_e8287 * (((p.p83 * locals.var_fffcbar_dn0) * assign6700_e8293) + (assign6700_e8290 * (locals.var_ffib_dn0 + locals.var_fffcbar_dn0))))) / (assign6700_e8294 * assign6700_e8294)), ((((2.0 * locals.var_ffdvc_dn1) * assign6700_e8294) - (assign6700_e8287 * (((p.p83 * locals.var_fffcbar_dn1) * assign6700_e8293) + (assign6700_e8290 * (locals.var_ffib_dn1 + locals.var_fffcbar_dn1))))) / (assign6700_e8294 * assign6700_e8294)), ((((2.0 * locals.var_ffdvc_dn3) * assign6700_e8294) - (assign6700_e8287 * (((p.p83 * locals.var_fffcbar_dn3) * assign6700_e8293) + (assign6700_e8290 * (locals.var_ffib_dn3 + locals.var_fffcbar_dn3))))) / (assign6700_e8294 * assign6700_e8294)), ((((2.0 * locals.var_ffdvc_dn4) * assign6700_e8294) - (assign6700_e8287 * (((p.p83 * locals.var_fffcbar_dn4) * assign6700_e8293) + (assign6700_e8290 * (locals.var_ffib_dn4 + locals.var_fffcbar_dn4))))) / (assign6700_e8294 * assign6700_e8294)), ((((2.0 * locals.var_ffdvc_dn5) * assign6700_e8294) - (assign6700_e8287 * (((p.p83 * locals.var_fffcbar_dn5) * assign6700_e8293) + (assign6700_e8290 * (locals.var_ffib_dn5 + locals.var_fffcbar_dn5))))) / (assign6700_e8294 * assign6700_e8294)), ((((2.0 * locals.var_ffdvc_dn6) * assign6700_e8294) - (assign6700_e8287 * (((p.p83 * locals.var_fffcbar_dn6) * assign6700_e8293) + (assign6700_e8290 * (locals.var_ffib_dn6 + locals.var_fffcbar_dn6))))) / (assign6700_e8294 * assign6700_e8294)), ((((2.0 * locals.var_ffdvc_dn7) * assign6700_e8294) - (assign6700_e8287 * (((p.p83 * locals.var_fffcbar_dn7) * assign6700_e8293) + (assign6700_e8290 * (locals.var_ffib_dn7 + locals.var_fffcbar_dn7))))) / (assign6700_e8294 * assign6700_e8294)), ((((2.0 * locals.var_ffdvc_dn8) * assign6700_e8294) - (assign6700_e8287 * (((p.p83 * locals.var_fffcbar_dn8) * assign6700_e8293) + (assign6700_e8290 * (locals.var_ffib_dn8 + locals.var_fffcbar_dn8))))) / (assign6700_e8294 * assign6700_e8294)), ((((2.0 * locals.var_ffdvc_dn9) * assign6700_e8294) - (assign6700_e8287 * (((p.p83 * locals.var_fffcbar_dn9) * assign6700_e8293) + (assign6700_e8290 * (locals.var_ffib_dn9 + locals.var_fffcbar_dn9))))) / (assign6700_e8294 * assign6700_e8294)),)
    } else {
        (locals.var_ffdvc_ditf, locals.var_ffdvc_ditf_dn0, locals.var_ffdvc_ditf_dn1, locals.var_ffdvc_ditf_dn3, locals.var_ffdvc_ditf_dn4, locals.var_ffdvc_ditf_dn5, locals.var_ffdvc_ditf_dn6, locals.var_ffdvc_ditf_dn7, locals.var_ffdvc_ditf_dn8, locals.var_ffdvc_ditf_dn9,)
    }
};
        locals.var_ffdvc_ditf = assign6700_e8297;
        locals.var_ffdvc_ditf_dn0 = assign6700_e8297_d_n0;
        locals.var_ffdvc_ditf_dn1 = assign6700_e8297_d_n1;
        locals.var_ffdvc_ditf_dn3 = assign6700_e8297_d_n3;
        locals.var_ffdvc_ditf_dn4 = assign6700_e8297_d_n4;
        locals.var_ffdvc_ditf_dn5 = assign6700_e8297_d_n5;
        locals.var_ffdvc_ditf_dn6 = assign6700_e8297_d_n6;
        locals.var_ffdvc_ditf_dn7 = assign6700_e8297_d_n7;
        locals.var_ffdvc_ditf_dn8 = assign6700_e8297_d_n8;
        locals.var_ffdvc_ditf_dn9 = assign6700_e8297_d_n9;
        locals.var_ffdvc_ditf_rv = 0.0;

        let (assign6710_e8314, assign6710_e8314_d_n0, assign6710_e8314_d_n1, assign6710_e8314_d_n3, assign6710_e8314_d_n4, assign6710_e8314_d_n5, assign6710_e8314_d_n6, assign6710_e8314_d_n7, assign6710_e8314_d_n8, assign6710_e8314_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign6710_e8303: f64 = (1.0 - p.p73);
        let assign6710_e8305: f64 = (assign6710_e8303 * locals.var_thcs_t);
        let assign6710_e8308: f64 = (locals.var_ffdvc * locals.var_ovt);
        let assign6710_e8309: f64 = (assign6710_e8308).exp();
        let assign6710_e8311: f64 = (assign6710_e8309 - 1.0);
        let assign6710_e8312: f64 = (assign6710_e8305 * assign6710_e8311);
        (assign6710_e8312, (assign6710_e8305 * (assign6710_e8309 * (locals.var_ffdvc_dn0 * locals.var_ovt))), (assign6710_e8305 * (assign6710_e8309 * (locals.var_ffdvc_dn1 * locals.var_ovt))), (assign6710_e8305 * (assign6710_e8309 * (locals.var_ffdvc_dn3 * locals.var_ovt))), (((assign6710_e8303 * locals.var_thcs_t_dn4) * assign6710_e8311) + (assign6710_e8305 * (assign6710_e8309 * ((locals.var_ffdvc_dn4 * locals.var_ovt) + (locals.var_ffdvc * locals.var_ovt_dn4))))), (assign6710_e8305 * (assign6710_e8309 * (locals.var_ffdvc_dn5 * locals.var_ovt))), (assign6710_e8305 * (assign6710_e8309 * (locals.var_ffdvc_dn6 * locals.var_ovt))), (assign6710_e8305 * (assign6710_e8309 * (locals.var_ffdvc_dn7 * locals.var_ovt))), (assign6710_e8305 * (assign6710_e8309 * (locals.var_ffdvc_dn8 * locals.var_ovt))), (assign6710_e8305 * (assign6710_e8309 * (locals.var_ffdvc_dn9 * locals.var_ovt))),)
    } else {
        (locals.var_ffdqbfb, locals.var_ffdqbfb_dn0, locals.var_ffdqbfb_dn1, locals.var_ffdqbfb_dn3, locals.var_ffdqbfb_dn4, locals.var_ffdqbfb_dn5, locals.var_ffdqbfb_dn6, locals.var_ffdqbfb_dn7, locals.var_ffdqbfb_dn8, locals.var_ffdqbfb_dn9,)
    }
};
        locals.var_ffdqbfb = assign6710_e8314;
        locals.var_ffdqbfb_dn0 = assign6710_e8314_d_n0;
        locals.var_ffdqbfb_dn1 = assign6710_e8314_d_n1;
        locals.var_ffdqbfb_dn3 = assign6710_e8314_d_n3;
        locals.var_ffdqbfb_dn4 = assign6710_e8314_d_n4;
        locals.var_ffdqbfb_dn5 = assign6710_e8314_d_n5;
        locals.var_ffdqbfb_dn6 = assign6710_e8314_d_n6;
        locals.var_ffdqbfb_dn7 = assign6710_e8314_d_n7;
        locals.var_ffdqbfb_dn8 = assign6710_e8314_d_n8;
        locals.var_ffdqbfb_dn9 = assign6710_e8314_d_n9;
        locals.var_ffdqbfb_rv = 0.0;

        let (assign6720_e8337, assign6720_e8337_d_n0, assign6720_e8337_d_n1, assign6720_e8337_d_n3, assign6720_e8337_d_n4, assign6720_e8337_d_n5, assign6720_e8337_d_n6, assign6720_e8337_d_n7, assign6720_e8337_d_n8, assign6720_e8337_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign6720_e8321: f64 = (1.0 - p.p73);
        let assign6720_e8323: f64 = (assign6720_e8321 * locals.var_thcs_t);
        let assign6720_e8325: f64 = (assign6720_e8323 * locals.var_itf);
        let assign6720_e8328: f64 = (locals.var_ffdvc * locals.var_ovt);
        let assign6720_e8329: f64 = (assign6720_e8328).exp();
        let assign6720_e8330: f64 = (assign6720_e8325 * assign6720_e8329);
        let assign6720_e8332: f64 = (assign6720_e8330 * locals.var_ovt);
        let assign6720_e8334: f64 = (assign6720_e8332 * locals.var_ffdvc_ditf);
        let assign6720_e8335: f64 = (locals.var_ffdqbfb + assign6720_e8334);
        (assign6720_e8335, (locals.var_ffdqbfb_dn0 + ((((((assign6720_e8323 * locals.var_itf_dn0) * assign6720_e8329) + (assign6720_e8325 * (assign6720_e8329 * (locals.var_ffdvc_dn0 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign6720_e8332 * locals.var_ffdvc_ditf_dn0))), (locals.var_ffdqbfb_dn1 + ((((((assign6720_e8323 * locals.var_itf_dn1) * assign6720_e8329) + (assign6720_e8325 * (assign6720_e8329 * (locals.var_ffdvc_dn1 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign6720_e8332 * locals.var_ffdvc_ditf_dn1))), (locals.var_ffdqbfb_dn3 + ((((((assign6720_e8323 * locals.var_itf_dn3) * assign6720_e8329) + (assign6720_e8325 * (assign6720_e8329 * (locals.var_ffdvc_dn3 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign6720_e8332 * locals.var_ffdvc_ditf_dn3))), (locals.var_ffdqbfb_dn4 + (((((((((assign6720_e8321 * locals.var_thcs_t_dn4) * locals.var_itf) + (assign6720_e8323 * locals.var_itf_dn4)) * assign6720_e8329) + (assign6720_e8325 * (assign6720_e8329 * ((locals.var_ffdvc_dn4 * locals.var_ovt) + (locals.var_ffdvc * locals.var_ovt_dn4))))) * locals.var_ovt) + (assign6720_e8330 * locals.var_ovt_dn4)) * locals.var_ffdvc_ditf) + (assign6720_e8332 * locals.var_ffdvc_ditf_dn4))), (locals.var_ffdqbfb_dn5 + ((((((assign6720_e8323 * locals.var_itf_dn5) * assign6720_e8329) + (assign6720_e8325 * (assign6720_e8329 * (locals.var_ffdvc_dn5 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign6720_e8332 * locals.var_ffdvc_ditf_dn5))), (locals.var_ffdqbfb_dn6 + ((((((assign6720_e8323 * locals.var_itf_dn6) * assign6720_e8329) + (assign6720_e8325 * (assign6720_e8329 * (locals.var_ffdvc_dn6 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign6720_e8332 * locals.var_ffdvc_ditf_dn6))), (locals.var_ffdqbfb_dn7 + ((((((assign6720_e8323 * locals.var_itf_dn7) * assign6720_e8329) + (assign6720_e8325 * (assign6720_e8329 * (locals.var_ffdvc_dn7 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign6720_e8332 * locals.var_ffdvc_ditf_dn7))), (locals.var_ffdqbfb_dn8 + ((((((assign6720_e8323 * locals.var_itf_dn8) * assign6720_e8329) + (assign6720_e8325 * (assign6720_e8329 * (locals.var_ffdvc_dn8 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign6720_e8332 * locals.var_ffdvc_ditf_dn8))), (locals.var_ffdqbfb_dn9 + ((((((assign6720_e8323 * locals.var_itf_dn9) * assign6720_e8329) + (assign6720_e8325 * (assign6720_e8329 * (locals.var_ffdvc_dn9 * locals.var_ovt)))) * locals.var_ovt) * locals.var_ffdvc_ditf) + (assign6720_e8332 * locals.var_ffdvc_ditf_dn9))),)
    } else {
        (locals.var_ffdtbfb, locals.var_ffdtbfb_dn0, locals.var_ffdtbfb_dn1, locals.var_ffdtbfb_dn3, locals.var_ffdtbfb_dn4, locals.var_ffdtbfb_dn5, locals.var_ffdtbfb_dn6, locals.var_ffdtbfb_dn7, locals.var_ffdtbfb_dn8, locals.var_ffdtbfb_dn9,)
    }
};
        locals.var_ffdtbfb = assign6720_e8337;
        locals.var_ffdtbfb_dn0 = assign6720_e8337_d_n0;
        locals.var_ffdtbfb_dn1 = assign6720_e8337_d_n1;
        locals.var_ffdtbfb_dn3 = assign6720_e8337_d_n3;
        locals.var_ffdtbfb_dn4 = assign6720_e8337_d_n4;
        locals.var_ffdtbfb_dn5 = assign6720_e8337_d_n5;
        locals.var_ffdtbfb_dn6 = assign6720_e8337_d_n6;
        locals.var_ffdtbfb_dn7 = assign6720_e8337_d_n7;
        locals.var_ffdtbfb_dn8 = assign6720_e8337_d_n8;
        locals.var_ffdtbfb_dn9 = assign6720_e8337_d_n9;
        locals.var_ffdtbfb_rv = 0.0;

        let (assign6730_e8347, assign6730_e8347_d_n0, assign6730_e8347_d_n1, assign6730_e8347_d_n3, assign6730_e8347_d_n4, assign6730_e8347_d_n5, assign6730_e8347_d_n6, assign6730_e8347_d_n7, assign6730_e8347_d_n8, assign6730_e8347_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign6730_e8344: f64 = (1.0 / locals.var_ffitf_ick);
        let assign6730_e8345: f64 = (1.0 - assign6730_e8344);
        (assign6730_e8345, (-(-(locals.var_ffitf_ick_dn0 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn1 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn3 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn4 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn5 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn6 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn7 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn8 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))), (-(-(locals.var_ffitf_ick_dn9 / (locals.var_ffitf_ick * locals.var_ffitf_ick)))),)
    } else {
        (locals.var_ffic, locals.var_ffic_dn0, locals.var_ffic_dn1, locals.var_ffic_dn3, locals.var_ffic_dn4, locals.var_ffic_dn5, locals.var_ffic_dn6, locals.var_ffic_dn7, locals.var_ffic_dn8, locals.var_ffic_dn9,)
    }
};
        locals.var_ffic = assign6730_e8347;
        locals.var_ffic_dn0 = assign6730_e8347_d_n0;
        locals.var_ffic_dn1 = assign6730_e8347_d_n1;
        locals.var_ffic_dn3 = assign6730_e8347_d_n3;
        locals.var_ffic_dn4 = assign6730_e8347_d_n4;
        locals.var_ffic_dn5 = assign6730_e8347_d_n5;
        locals.var_ffic_dn6 = assign6730_e8347_d_n6;
        locals.var_ffic_dn7 = assign6730_e8347_d_n7;
        locals.var_ffic_dn8 = assign6730_e8347_d_n8;
        locals.var_ffic_dn9 = assign6730_e8347_d_n9;
        locals.var_ffic_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6740_e8367, assign6740_e8367_d_n0, assign6740_e8367_d_n1, assign6740_e8367_d_n3, assign6740_e8367_d_n4, assign6740_e8367_d_n5, assign6740_e8367_d_n6, assign6740_e8367_d_n7, assign6740_e8367_d_n8, assign6740_e8367_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign6740_e8354: f64 = (locals.var_ffic * locals.var_ffic);
        let assign6740_e8356: f64 = (assign6740_e8354 + p.p72);
        let assign6740_e8357: f64 = (assign6740_e8356).sqrt();
        let assign6740_e8358: f64 = (locals.var_ffic + assign6740_e8357);
        let assign6740_e8362: f64 = (1.0 + p.p72);
        let assign6740_e8363: f64 = (assign6740_e8362).sqrt();
        let assign6740_e8364: f64 = (1.0 + assign6740_e8363);
        let assign6740_e8365: f64 = (assign6740_e8358 / assign6740_e8364);
        (assign6740_e8365, ((locals.var_ffic_dn0 + (((locals.var_ffic_dn0 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn0)) / (2.0 * assign6740_e8357))) / assign6740_e8364), ((locals.var_ffic_dn1 + (((locals.var_ffic_dn1 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn1)) / (2.0 * assign6740_e8357))) / assign6740_e8364), ((locals.var_ffic_dn3 + (((locals.var_ffic_dn3 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn3)) / (2.0 * assign6740_e8357))) / assign6740_e8364), ((locals.var_ffic_dn4 + (((locals.var_ffic_dn4 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn4)) / (2.0 * assign6740_e8357))) / assign6740_e8364), ((locals.var_ffic_dn5 + (((locals.var_ffic_dn5 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn5)) / (2.0 * assign6740_e8357))) / assign6740_e8364), ((locals.var_ffic_dn6 + (((locals.var_ffic_dn6 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn6)) / (2.0 * assign6740_e8357))) / assign6740_e8364), ((locals.var_ffic_dn7 + (((locals.var_ffic_dn7 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn7)) / (2.0 * assign6740_e8357))) / assign6740_e8364), ((locals.var_ffic_dn8 + (((locals.var_ffic_dn8 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn8)) / (2.0 * assign6740_e8357))) / assign6740_e8364), ((locals.var_ffic_dn9 + (((locals.var_ffic_dn9 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn9)) / (2.0 * assign6740_e8357))) / assign6740_e8364),)
    } else {
        (locals.var_ffw, locals.var_ffw_dn0, locals.var_ffw_dn1, locals.var_ffw_dn3, locals.var_ffw_dn4, locals.var_ffw_dn5, locals.var_ffw_dn6, locals.var_ffw_dn7, locals.var_ffw_dn8, locals.var_ffw_dn9,)
    }
};
        locals.var_ffw = assign6740_e8367;
        locals.var_ffw_dn0 = assign6740_e8367_d_n0;
        locals.var_ffw_dn1 = assign6740_e8367_d_n1;
        locals.var_ffw_dn3 = assign6740_e8367_d_n3;
        locals.var_ffw_dn4 = assign6740_e8367_d_n4;
        locals.var_ffw_dn5 = assign6740_e8367_d_n5;
        locals.var_ffw_dn6 = assign6740_e8367_d_n6;
        locals.var_ffw_dn7 = assign6740_e8367_d_n7;
        locals.var_ffw_dn8 = assign6740_e8367_d_n8;
        locals.var_ffw_dn9 = assign6740_e8367_d_n9;
        locals.var_ffw_rv = 0.0;

        let (assign6750_e8378, assign6750_e8378_d_n0, assign6750_e8378_d_n1, assign6750_e8378_d_n3, assign6750_e8378_d_n4, assign6750_e8378_d_n5, assign6750_e8378_d_n6, assign6750_e8378_d_n7, assign6750_e8378_d_n8, assign6750_e8378_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign6750_e8373: f64 = (locals.var_ffdvc - p.p82);
        let assign6750_e8375: f64 = (assign6750_e8373 * locals.var_ovt);
        let assign6750_e8376: f64 = (assign6750_e8375).exp();
        (assign6750_e8376, (assign6750_e8376 * (locals.var_ffdvc_dn0 * locals.var_ovt)), (assign6750_e8376 * (locals.var_ffdvc_dn1 * locals.var_ovt)), (assign6750_e8376 * (locals.var_ffdvc_dn3 * locals.var_ovt)), (assign6750_e8376 * ((locals.var_ffdvc_dn4 * locals.var_ovt) + (assign6750_e8373 * locals.var_ovt_dn4))), (assign6750_e8376 * (locals.var_ffdvc_dn5 * locals.var_ovt)), (assign6750_e8376 * (locals.var_ffdvc_dn6 * locals.var_ovt)), (assign6750_e8376 * (locals.var_ffdvc_dn7 * locals.var_ovt)), (assign6750_e8376 * (locals.var_ffdvc_dn8 * locals.var_ovt)), (assign6750_e8376 * (locals.var_ffdvc_dn9 * locals.var_ovt)),)
    } else {
        (locals.var_ffvc_exp, locals.var_ffvc_exp_dn0, locals.var_ffvc_exp_dn1, locals.var_ffvc_exp_dn3, locals.var_ffvc_exp_dn4, locals.var_ffvc_exp_dn5, locals.var_ffvc_exp_dn6, locals.var_ffvc_exp_dn7, locals.var_ffvc_exp_dn8, locals.var_ffvc_exp_dn9,)
    }
};
        locals.var_ffvc_exp = assign6750_e8378;
        locals.var_ffvc_exp_dn0 = assign6750_e8378_d_n0;
        locals.var_ffvc_exp_dn1 = assign6750_e8378_d_n1;
        locals.var_ffvc_exp_dn3 = assign6750_e8378_d_n3;
        locals.var_ffvc_exp_dn4 = assign6750_e8378_d_n4;
        locals.var_ffvc_exp_dn5 = assign6750_e8378_d_n5;
        locals.var_ffvc_exp_dn6 = assign6750_e8378_d_n6;
        locals.var_ffvc_exp_dn7 = assign6750_e8378_d_n7;
        locals.var_ffvc_exp_dn8 = assign6750_e8378_d_n8;
        locals.var_ffvc_exp_dn9 = assign6750_e8378_d_n9;
        locals.var_ffvc_exp_rv = 0.0;

        let (assign6760_e8390, assign6760_e8390_d_n0, assign6760_e8390_d_n1, assign6760_e8390_d_n3, assign6760_e8390_d_n4, assign6760_e8390_d_n5, assign6760_e8390_d_n6, assign6760_e8390_d_n7, assign6760_e8390_d_n8, assign6760_e8390_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign6760_e8384: f64 = (locals.var_thcs_t * locals.var_ffw);
        let assign6760_e8386: f64 = (assign6760_e8384 * locals.var_ffw);
        let assign6760_e8388: f64 = (assign6760_e8386 * locals.var_ffvc_exp);
        (assign6760_e8388, (((((locals.var_thcs_t * locals.var_ffw_dn0) * locals.var_ffw) + (assign6760_e8384 * locals.var_ffw_dn0)) * locals.var_ffvc_exp) + (assign6760_e8386 * locals.var_ffvc_exp_dn0)), (((((locals.var_thcs_t * locals.var_ffw_dn1) * locals.var_ffw) + (assign6760_e8384 * locals.var_ffw_dn1)) * locals.var_ffvc_exp) + (assign6760_e8386 * locals.var_ffvc_exp_dn1)), (((((locals.var_thcs_t * locals.var_ffw_dn3) * locals.var_ffw) + (assign6760_e8384 * locals.var_ffw_dn3)) * locals.var_ffvc_exp) + (assign6760_e8386 * locals.var_ffvc_exp_dn3)), ((((((locals.var_thcs_t_dn4 * locals.var_ffw) + (locals.var_thcs_t * locals.var_ffw_dn4)) * locals.var_ffw) + (assign6760_e8384 * locals.var_ffw_dn4)) * locals.var_ffvc_exp) + (assign6760_e8386 * locals.var_ffvc_exp_dn4)), (((((locals.var_thcs_t * locals.var_ffw_dn5) * locals.var_ffw) + (assign6760_e8384 * locals.var_ffw_dn5)) * locals.var_ffvc_exp) + (assign6760_e8386 * locals.var_ffvc_exp_dn5)), (((((locals.var_thcs_t * locals.var_ffw_dn6) * locals.var_ffw) + (assign6760_e8384 * locals.var_ffw_dn6)) * locals.var_ffvc_exp) + (assign6760_e8386 * locals.var_ffvc_exp_dn6)), (((((locals.var_thcs_t * locals.var_ffw_dn7) * locals.var_ffw) + (assign6760_e8384 * locals.var_ffw_dn7)) * locals.var_ffvc_exp) + (assign6760_e8386 * locals.var_ffvc_exp_dn7)), (((((locals.var_thcs_t * locals.var_ffw_dn8) * locals.var_ffw) + (assign6760_e8384 * locals.var_ffw_dn8)) * locals.var_ffvc_exp) + (assign6760_e8386 * locals.var_ffvc_exp_dn8)), (((((locals.var_thcs_t * locals.var_ffw_dn9) * locals.var_ffw) + (assign6760_e8384 * locals.var_ffw_dn9)) * locals.var_ffvc_exp) + (assign6760_e8386 * locals.var_ffvc_exp_dn9)),)
    } else {
        (locals.var_ffdqfhc, locals.var_ffdqfhc_dn0, locals.var_ffdqfhc_dn1, locals.var_ffdqfhc_dn3, locals.var_ffdqfhc_dn4, locals.var_ffdqfhc_dn5, locals.var_ffdqfhc_dn6, locals.var_ffdqfhc_dn7, locals.var_ffdqfhc_dn8, locals.var_ffdqfhc_dn9,)
    }
};
        locals.var_ffdqfhc = assign6760_e8390;
        locals.var_ffdqfhc_dn0 = assign6760_e8390_d_n0;
        locals.var_ffdqfhc_dn1 = assign6760_e8390_d_n1;
        locals.var_ffdqfhc_dn3 = assign6760_e8390_d_n3;
        locals.var_ffdqfhc_dn4 = assign6760_e8390_d_n4;
        locals.var_ffdqfhc_dn5 = assign6760_e8390_d_n5;
        locals.var_ffdqfhc_dn6 = assign6760_e8390_d_n6;
        locals.var_ffdqfhc_dn7 = assign6760_e8390_d_n7;
        locals.var_ffdqfhc_dn8 = assign6760_e8390_d_n8;
        locals.var_ffdqfhc_dn9 = assign6760_e8390_d_n9;
        locals.var_ffdqfhc_rv = 0.0;

        let (assign6770_e8415, assign6770_e8415_d_n0, assign6770_e8415_d_n1, assign6770_e8415_d_n3, assign6770_e8415_d_n4, assign6770_e8415_d_n5, assign6770_e8415_d_n6, assign6770_e8415_d_n7, assign6770_e8415_d_n8, assign6770_e8415_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign6770_e8400: f64 = (locals.var_ffic * locals.var_ffic);
        let assign6770_e8402: f64 = (assign6770_e8400 + p.p72);
        let assign6770_e8403: f64 = (assign6770_e8402).sqrt();
        let assign6770_e8404: f64 = (locals.var_ffitf_ick * assign6770_e8403);
        let assign6770_e8405: f64 = (2.0 / assign6770_e8404);
        let assign6770_e8406: f64 = (1.0 + assign6770_e8405);
        let assign6770_e8409: f64 = (locals.var_ovt * locals.var_itf);
        let assign6770_e8411: f64 = (assign6770_e8409 * locals.var_ffdvc_ditf);
        let assign6770_e8412: f64 = (assign6770_e8406 + assign6770_e8411);
        let assign6770_e8413: f64 = (locals.var_ffdqfhc * assign6770_e8412);
        (assign6770_e8413, ((locals.var_ffdqfhc_dn0 * assign6770_e8412) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn0 * assign6770_e8403) + (locals.var_ffitf_ick * (((locals.var_ffic_dn0 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn0)) / (2.0 * assign6770_e8403))))) / (assign6770_e8404 * assign6770_e8404))) + (((locals.var_ovt * locals.var_itf_dn0) * locals.var_ffdvc_ditf) + (assign6770_e8409 * locals.var_ffdvc_ditf_dn0))))), ((locals.var_ffdqfhc_dn1 * assign6770_e8412) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn1 * assign6770_e8403) + (locals.var_ffitf_ick * (((locals.var_ffic_dn1 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn1)) / (2.0 * assign6770_e8403))))) / (assign6770_e8404 * assign6770_e8404))) + (((locals.var_ovt * locals.var_itf_dn1) * locals.var_ffdvc_ditf) + (assign6770_e8409 * locals.var_ffdvc_ditf_dn1))))), ((locals.var_ffdqfhc_dn3 * assign6770_e8412) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn3 * assign6770_e8403) + (locals.var_ffitf_ick * (((locals.var_ffic_dn3 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn3)) / (2.0 * assign6770_e8403))))) / (assign6770_e8404 * assign6770_e8404))) + (((locals.var_ovt * locals.var_itf_dn3) * locals.var_ffdvc_ditf) + (assign6770_e8409 * locals.var_ffdvc_ditf_dn3))))), ((locals.var_ffdqfhc_dn4 * assign6770_e8412) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn4 * assign6770_e8403) + (locals.var_ffitf_ick * (((locals.var_ffic_dn4 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn4)) / (2.0 * assign6770_e8403))))) / (assign6770_e8404 * assign6770_e8404))) + ((((locals.var_ovt_dn4 * locals.var_itf) + (locals.var_ovt * locals.var_itf_dn4)) * locals.var_ffdvc_ditf) + (assign6770_e8409 * locals.var_ffdvc_ditf_dn4))))), ((locals.var_ffdqfhc_dn5 * assign6770_e8412) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn5 * assign6770_e8403) + (locals.var_ffitf_ick * (((locals.var_ffic_dn5 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn5)) / (2.0 * assign6770_e8403))))) / (assign6770_e8404 * assign6770_e8404))) + (((locals.var_ovt * locals.var_itf_dn5) * locals.var_ffdvc_ditf) + (assign6770_e8409 * locals.var_ffdvc_ditf_dn5))))), ((locals.var_ffdqfhc_dn6 * assign6770_e8412) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn6 * assign6770_e8403) + (locals.var_ffitf_ick * (((locals.var_ffic_dn6 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn6)) / (2.0 * assign6770_e8403))))) / (assign6770_e8404 * assign6770_e8404))) + (((locals.var_ovt * locals.var_itf_dn6) * locals.var_ffdvc_ditf) + (assign6770_e8409 * locals.var_ffdvc_ditf_dn6))))), ((locals.var_ffdqfhc_dn7 * assign6770_e8412) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn7 * assign6770_e8403) + (locals.var_ffitf_ick * (((locals.var_ffic_dn7 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn7)) / (2.0 * assign6770_e8403))))) / (assign6770_e8404 * assign6770_e8404))) + (((locals.var_ovt * locals.var_itf_dn7) * locals.var_ffdvc_ditf) + (assign6770_e8409 * locals.var_ffdvc_ditf_dn7))))), ((locals.var_ffdqfhc_dn8 * assign6770_e8412) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn8 * assign6770_e8403) + (locals.var_ffitf_ick * (((locals.var_ffic_dn8 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn8)) / (2.0 * assign6770_e8403))))) / (assign6770_e8404 * assign6770_e8404))) + (((locals.var_ovt * locals.var_itf_dn8) * locals.var_ffdvc_ditf) + (assign6770_e8409 * locals.var_ffdvc_ditf_dn8))))), ((locals.var_ffdqfhc_dn9 * assign6770_e8412) + (locals.var_ffdqfhc * ((-((2.0 * ((locals.var_ffitf_ick_dn9 * assign6770_e8403) + (locals.var_ffitf_ick * (((locals.var_ffic_dn9 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn9)) / (2.0 * assign6770_e8403))))) / (assign6770_e8404 * assign6770_e8404))) + (((locals.var_ovt * locals.var_itf_dn9) * locals.var_ffdvc_ditf) + (assign6770_e8409 * locals.var_ffdvc_ditf_dn9))))),)
    } else {
        (locals.var_ffdtfhc, locals.var_ffdtfhc_dn0, locals.var_ffdtfhc_dn1, locals.var_ffdtfhc_dn3, locals.var_ffdtfhc_dn4, locals.var_ffdtfhc_dn5, locals.var_ffdtfhc_dn6, locals.var_ffdtfhc_dn7, locals.var_ffdtfhc_dn8, locals.var_ffdtfhc_dn9,)
    }
};
        locals.var_ffdtfhc = assign6770_e8415;
        locals.var_ffdtfhc_dn0 = assign6770_e8415_d_n0;
        locals.var_ffdtfhc_dn1 = assign6770_e8415_d_n1;
        locals.var_ffdtfhc_dn3 = assign6770_e8415_d_n3;
        locals.var_ffdtfhc_dn4 = assign6770_e8415_d_n4;
        locals.var_ffdtfhc_dn5 = assign6770_e8415_d_n5;
        locals.var_ffdtfhc_dn6 = assign6770_e8415_d_n6;
        locals.var_ffdtfhc_dn7 = assign6770_e8415_d_n7;
        locals.var_ffdtfhc_dn8 = assign6770_e8415_d_n8;
        locals.var_ffdtfhc_dn9 = assign6770_e8415_d_n9;
        locals.var_ffdtfhc_rv = 0.0;

        let assign6780_e8425: f64 = (locals.var_ffw * p.p115);
        let assign6780_e8431: f64 = (locals.var_ffw * p.p116);
        let assign6780_e8434: f64 = if ((((p.p115 < 0.01) && (p.p116 < 0.01)) && (assign6780_e8425 < 0.005)) && (assign6780_e8431 < 0.005)) { 1.0 } else { 0.0 };
        locals.var_guard147 = assign6780_e8434;
        locals.var_guard147_rv = 0.0;

        let (assign6790_e8446, assign6790_e8446_d_n0, assign6790_e8446_d_n1, assign6790_e8446_d_n3, assign6790_e8446_d_n4, assign6790_e8446_d_n5, assign6790_e8446_d_n6, assign6790_e8446_d_n7, assign6790_e8446_d_n8, assign6790_e8446_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 != 0.0)) {
        let assign6790_e8442: f64 = (p.p73 * locals.var_ffdqfhc);
        let assign6790_e8444: f64 = (assign6790_e8442 * locals.var_itf);
        (assign6790_e8444, (((p.p73 * locals.var_ffdqfhc_dn0) * locals.var_itf) + (assign6790_e8442 * locals.var_itf_dn0)), (((p.p73 * locals.var_ffdqfhc_dn1) * locals.var_itf) + (assign6790_e8442 * locals.var_itf_dn1)), (((p.p73 * locals.var_ffdqfhc_dn3) * locals.var_itf) + (assign6790_e8442 * locals.var_itf_dn3)), (((p.p73 * locals.var_ffdqfhc_dn4) * locals.var_itf) + (assign6790_e8442 * locals.var_itf_dn4)), (((p.p73 * locals.var_ffdqfhc_dn5) * locals.var_itf) + (assign6790_e8442 * locals.var_itf_dn5)), (((p.p73 * locals.var_ffdqfhc_dn6) * locals.var_itf) + (assign6790_e8442 * locals.var_itf_dn6)), (((p.p73 * locals.var_ffdqfhc_dn7) * locals.var_itf) + (assign6790_e8442 * locals.var_itf_dn7)), (((p.p73 * locals.var_ffdqfhc_dn8) * locals.var_itf) + (assign6790_e8442 * locals.var_itf_dn8)), (((p.p73 * locals.var_ffdqfhc_dn9) * locals.var_itf) + (assign6790_e8442 * locals.var_itf_dn9)),)
    } else {
        (locals.var_ffdqcfc, locals.var_ffdqcfc_dn0, locals.var_ffdqcfc_dn1, locals.var_ffdqcfc_dn3, locals.var_ffdqcfc_dn4, locals.var_ffdqcfc_dn5, locals.var_ffdqcfc_dn6, locals.var_ffdqcfc_dn7, locals.var_ffdqcfc_dn8, locals.var_ffdqcfc_dn9,)
    }
};
        locals.var_ffdqcfc = assign6790_e8446;
        locals.var_ffdqcfc_dn0 = assign6790_e8446_d_n0;
        locals.var_ffdqcfc_dn1 = assign6790_e8446_d_n1;
        locals.var_ffdqcfc_dn3 = assign6790_e8446_d_n3;
        locals.var_ffdqcfc_dn4 = assign6790_e8446_d_n4;
        locals.var_ffdqcfc_dn5 = assign6790_e8446_d_n5;
        locals.var_ffdqcfc_dn6 = assign6790_e8446_d_n6;
        locals.var_ffdqcfc_dn7 = assign6790_e8446_d_n7;
        locals.var_ffdqcfc_dn8 = assign6790_e8446_d_n8;
        locals.var_ffdqcfc_dn9 = assign6790_e8446_d_n9;
        locals.var_ffdqcfc_rv = 0.0;

        let (assign6800_e8456, assign6800_e8456_d_n0, assign6800_e8456_d_n1, assign6800_e8456_d_n3, assign6800_e8456_d_n4, assign6800_e8456_d_n5, assign6800_e8456_d_n6, assign6800_e8456_d_n7, assign6800_e8456_d_n8, assign6800_e8456_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 != 0.0)) {
        let assign6800_e8454: f64 = (p.p73 * locals.var_ffdtfhc);
        (assign6800_e8454, (p.p73 * locals.var_ffdtfhc_dn0), (p.p73 * locals.var_ffdtfhc_dn1), (p.p73 * locals.var_ffdtfhc_dn3), (p.p73 * locals.var_ffdtfhc_dn4), (p.p73 * locals.var_ffdtfhc_dn5), (p.p73 * locals.var_ffdtfhc_dn6), (p.p73 * locals.var_ffdtfhc_dn7), (p.p73 * locals.var_ffdtfhc_dn8), (p.p73 * locals.var_ffdtfhc_dn9),)
    } else {
        (locals.var_ffdtcfc, locals.var_ffdtcfc_dn0, locals.var_ffdtcfc_dn1, locals.var_ffdtcfc_dn3, locals.var_ffdtcfc_dn4, locals.var_ffdtcfc_dn5, locals.var_ffdtcfc_dn6, locals.var_ffdtcfc_dn7, locals.var_ffdtcfc_dn8, locals.var_ffdtcfc_dn9,)
    }
};
        locals.var_ffdtcfc = assign6800_e8456;
        locals.var_ffdtcfc_dn0 = assign6800_e8456_d_n0;
        locals.var_ffdtcfc_dn1 = assign6800_e8456_d_n1;
        locals.var_ffdtcfc_dn3 = assign6800_e8456_d_n3;
        locals.var_ffdtcfc_dn4 = assign6800_e8456_d_n4;
        locals.var_ffdtcfc_dn5 = assign6800_e8456_d_n5;
        locals.var_ffdtcfc_dn6 = assign6800_e8456_d_n6;
        locals.var_ffdtcfc_dn7 = assign6800_e8456_d_n7;
        locals.var_ffdtcfc_dn8 = assign6800_e8456_d_n8;
        locals.var_ffdtcfc_dn9 = assign6800_e8456_d_n9;
        locals.var_ffdtcfc_rv = 0.0;

        let (assign6810_e8467, assign6810_e8467_d_n0, assign6810_e8467_d_n1, assign6810_e8467_d_n3, assign6810_e8467_d_n4, assign6810_e8467_d_n5, assign6810_e8467_d_n6, assign6810_e8467_d_n7, assign6810_e8467_d_n8, assign6810_e8467_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) {
        let assign6810_e8465: f64 = (1.0 - locals.var_ffw);
        (assign6810_e8465, (-locals.var_ffw_dn0), (-locals.var_ffw_dn1), (-locals.var_ffw_dn3), (-locals.var_ffw_dn4), (-locals.var_ffw_dn5), (-locals.var_ffw_dn6), (-locals.var_ffw_dn7), (-locals.var_ffw_dn8), (-locals.var_ffw_dn9),)
    } else {
        (locals.var_fcick, locals.var_fcick_dn0, locals.var_fcick_dn1, locals.var_fcick_dn3, locals.var_fcick_dn4, locals.var_fcick_dn5, locals.var_fcick_dn6, locals.var_fcick_dn7, locals.var_fcick_dn8, locals.var_fcick_dn9,)
    }
};
        locals.var_fcick = assign6810_e8467;
        locals.var_fcick_dn0 = assign6810_e8467_d_n0;
        locals.var_fcick_dn1 = assign6810_e8467_d_n1;
        locals.var_fcick_dn3 = assign6810_e8467_d_n3;
        locals.var_fcick_dn4 = assign6810_e8467_d_n4;
        locals.var_fcick_dn5 = assign6810_e8467_d_n5;
        locals.var_fcick_dn6 = assign6810_e8467_d_n6;
        locals.var_fcick_dn7 = assign6810_e8467_d_n7;
        locals.var_fcick_dn8 = assign6810_e8467_d_n8;
        locals.var_fcick_dn9 = assign6810_e8467_d_n9;
        locals.var_fcick_rv = 0.0;

        let (assign6820_e8491, assign6820_e8491_d_n0, assign6820_e8491_d_n1, assign6820_e8491_d_n3, assign6820_e8491_d_n4, assign6820_e8491_d_n5, assign6820_e8491_d_n6, assign6820_e8491_d_n7, assign6820_e8491_d_n8, assign6820_e8491_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) {
        let assign6820_e8476: f64 = (locals.var_fcick - 1.0);
        let assign6820_e8479: f64 = (1.0 - locals.var_ffic);
        let assign6820_e8480: f64 = (assign6820_e8476 * assign6820_e8479);
        let assign6820_e8483: f64 = (locals.var_ffic * locals.var_ffic);
        let assign6820_e8485: f64 = (assign6820_e8483 + p.p72);
        let assign6820_e8486: f64 = (assign6820_e8485).sqrt();
        let assign6820_e8488: f64 = (assign6820_e8486 * locals.var_itf);
        let assign6820_e8489: f64 = (assign6820_e8480 / assign6820_e8488);
        (assign6820_e8489, (((((locals.var_fcick_dn0 * assign6820_e8479) + (assign6820_e8476 * (-locals.var_ffic_dn0))) * assign6820_e8488) - (assign6820_e8480 * (((((locals.var_ffic_dn0 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn0)) / (2.0 * assign6820_e8486)) * locals.var_itf) + (assign6820_e8486 * locals.var_itf_dn0)))) / (assign6820_e8488 * assign6820_e8488)), (((((locals.var_fcick_dn1 * assign6820_e8479) + (assign6820_e8476 * (-locals.var_ffic_dn1))) * assign6820_e8488) - (assign6820_e8480 * (((((locals.var_ffic_dn1 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn1)) / (2.0 * assign6820_e8486)) * locals.var_itf) + (assign6820_e8486 * locals.var_itf_dn1)))) / (assign6820_e8488 * assign6820_e8488)), (((((locals.var_fcick_dn3 * assign6820_e8479) + (assign6820_e8476 * (-locals.var_ffic_dn3))) * assign6820_e8488) - (assign6820_e8480 * (((((locals.var_ffic_dn3 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn3)) / (2.0 * assign6820_e8486)) * locals.var_itf) + (assign6820_e8486 * locals.var_itf_dn3)))) / (assign6820_e8488 * assign6820_e8488)), (((((locals.var_fcick_dn4 * assign6820_e8479) + (assign6820_e8476 * (-locals.var_ffic_dn4))) * assign6820_e8488) - (assign6820_e8480 * (((((locals.var_ffic_dn4 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn4)) / (2.0 * assign6820_e8486)) * locals.var_itf) + (assign6820_e8486 * locals.var_itf_dn4)))) / (assign6820_e8488 * assign6820_e8488)), (((((locals.var_fcick_dn5 * assign6820_e8479) + (assign6820_e8476 * (-locals.var_ffic_dn5))) * assign6820_e8488) - (assign6820_e8480 * (((((locals.var_ffic_dn5 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn5)) / (2.0 * assign6820_e8486)) * locals.var_itf) + (assign6820_e8486 * locals.var_itf_dn5)))) / (assign6820_e8488 * assign6820_e8488)), (((((locals.var_fcick_dn6 * assign6820_e8479) + (assign6820_e8476 * (-locals.var_ffic_dn6))) * assign6820_e8488) - (assign6820_e8480 * (((((locals.var_ffic_dn6 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn6)) / (2.0 * assign6820_e8486)) * locals.var_itf) + (assign6820_e8486 * locals.var_itf_dn6)))) / (assign6820_e8488 * assign6820_e8488)), (((((locals.var_fcick_dn7 * assign6820_e8479) + (assign6820_e8476 * (-locals.var_ffic_dn7))) * assign6820_e8488) - (assign6820_e8480 * (((((locals.var_ffic_dn7 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn7)) / (2.0 * assign6820_e8486)) * locals.var_itf) + (assign6820_e8486 * locals.var_itf_dn7)))) / (assign6820_e8488 * assign6820_e8488)), (((((locals.var_fcick_dn8 * assign6820_e8479) + (assign6820_e8476 * (-locals.var_ffic_dn8))) * assign6820_e8488) - (assign6820_e8480 * (((((locals.var_ffic_dn8 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn8)) / (2.0 * assign6820_e8486)) * locals.var_itf) + (assign6820_e8486 * locals.var_itf_dn8)))) / (assign6820_e8488 * assign6820_e8488)), (((((locals.var_fcick_dn9 * assign6820_e8479) + (assign6820_e8476 * (-locals.var_ffic_dn9))) * assign6820_e8488) - (assign6820_e8480 * (((((locals.var_ffic_dn9 * locals.var_ffic) + (locals.var_ffic * locals.var_ffic_dn9)) / (2.0 * assign6820_e8486)) * locals.var_itf) + (assign6820_e8486 * locals.var_itf_dn9)))) / (assign6820_e8488 * assign6820_e8488)),)
    } else {
        (locals.var_fcdick_ditf, locals.var_fcdick_ditf_dn0, locals.var_fcdick_ditf_dn1, locals.var_fcdick_ditf_dn3, locals.var_fcdick_ditf_dn4, locals.var_fcdick_ditf_dn5, locals.var_fcdick_ditf_dn6, locals.var_fcdick_ditf_dn7, locals.var_fcdick_ditf_dn8, locals.var_fcdick_ditf_dn9,)
    }
};
        locals.var_fcdick_ditf = assign6820_e8491;
        locals.var_fcdick_ditf_dn0 = assign6820_e8491_d_n0;
        locals.var_fcdick_ditf_dn1 = assign6820_e8491_d_n1;
        locals.var_fcdick_ditf_dn3 = assign6820_e8491_d_n3;
        locals.var_fcdick_ditf_dn4 = assign6820_e8491_d_n4;
        locals.var_fcdick_ditf_dn5 = assign6820_e8491_d_n5;
        locals.var_fcdick_ditf_dn6 = assign6820_e8491_d_n6;
        locals.var_fcdick_ditf_dn7 = assign6820_e8491_d_n7;
        locals.var_fcdick_ditf_dn8 = assign6820_e8491_d_n8;
        locals.var_fcdick_ditf_dn9 = assign6820_e8491_d_n9;
        locals.var_fcdick_ditf_rv = 0.0;

        let assign6830_e8493: f64 = (locals.var_lat_delta).abs();
        let assign6830_e8495: f64 = if assign6830_e8493 > 0.001 { 1.0 } else { 0.0 };
        locals.var_guard148 = assign6830_e8495;
        locals.var_guard148_rv = 0.0;

        let (assign6840_e8511, assign6840_e8511_d_n0, assign6840_e8511_d_n1, assign6840_e8511_d_n3, assign6840_e8511_d_n4, assign6840_e8511_d_n5, assign6840_e8511_d_n6, assign6840_e8511_d_n7, assign6840_e8511_d_n8, assign6840_e8511_d_n9,) = {
    if ((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) {
        let assign6840_e8506: f64 = (locals.var_fcick - 1.0);
        let assign6840_e8508: f64 = (assign6840_e8506 * locals.var_ln_lat);
        let assign6840_e8509: f64 = (assign6840_e8508).exp();
        (assign6840_e8509, (assign6840_e8509 * (locals.var_fcick_dn0 * locals.var_ln_lat)), (assign6840_e8509 * (locals.var_fcick_dn1 * locals.var_ln_lat)), (assign6840_e8509 * (locals.var_fcick_dn3 * locals.var_ln_lat)), (assign6840_e8509 * (locals.var_fcick_dn4 * locals.var_ln_lat)), (assign6840_e8509 * (locals.var_fcick_dn5 * locals.var_ln_lat)), (assign6840_e8509 * (locals.var_fcick_dn6 * locals.var_ln_lat)), (assign6840_e8509 * (locals.var_fcick_dn7 * locals.var_ln_lat)), (assign6840_e8509 * (locals.var_fcick_dn8 * locals.var_ln_lat)), (assign6840_e8509 * (locals.var_fcick_dn9 * locals.var_ln_lat)),)
    } else {
        (locals.var_fck, locals.var_fck_dn0, locals.var_fck_dn1, locals.var_fck_dn3, locals.var_fck_dn4, locals.var_fck_dn5, locals.var_fck_dn6, locals.var_fck_dn7, locals.var_fck_dn8, locals.var_fck_dn9,)
    }
};
        locals.var_fck = assign6840_e8511;
        locals.var_fck_dn0 = assign6840_e8511_d_n0;
        locals.var_fck_dn1 = assign6840_e8511_d_n1;
        locals.var_fck_dn3 = assign6840_e8511_d_n3;
        locals.var_fck_dn4 = assign6840_e8511_d_n4;
        locals.var_fck_dn5 = assign6840_e8511_d_n5;
        locals.var_fck_dn6 = assign6840_e8511_d_n6;
        locals.var_fck_dn7 = assign6840_e8511_d_n7;
        locals.var_fck_dn8 = assign6840_e8511_d_n8;
        locals.var_fck_dn9 = assign6840_e8511_d_n9;
        locals.var_fck_rv = 0.0;

        let assign6850_e8514: f64 = if locals.var_latmin < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard149 = assign6850_e8514;
        locals.var_guard149_rv = 0.0;

        let (assign6860_e8533, assign6860_e8533_d_n0, assign6860_e8533_d_n1, assign6860_e8533_d_n3, assign6860_e8533_d_n4, assign6860_e8533_d_n5, assign6860_e8533_d_n6, assign6860_e8533_d_n7, assign6860_e8533_d_n8, assign6860_e8533_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 != 0.0)) {
        let assign6860_e8527: f64 = (1.0 - locals.var_fck);
        let assign6860_e8530: f64 = (locals.var_fck * locals.var_latmax);
        let assign6860_e8531: f64 = (assign6860_e8527 / assign6860_e8530);
        (assign6860_e8531, ((((-locals.var_fck_dn0) * assign6860_e8530) - (assign6860_e8527 * (locals.var_fck_dn0 * locals.var_latmax))) / (assign6860_e8530 * assign6860_e8530)), ((((-locals.var_fck_dn1) * assign6860_e8530) - (assign6860_e8527 * (locals.var_fck_dn1 * locals.var_latmax))) / (assign6860_e8530 * assign6860_e8530)), ((((-locals.var_fck_dn3) * assign6860_e8530) - (assign6860_e8527 * (locals.var_fck_dn3 * locals.var_latmax))) / (assign6860_e8530 * assign6860_e8530)), ((((-locals.var_fck_dn4) * assign6860_e8530) - (assign6860_e8527 * (locals.var_fck_dn4 * locals.var_latmax))) / (assign6860_e8530 * assign6860_e8530)), ((((-locals.var_fck_dn5) * assign6860_e8530) - (assign6860_e8527 * (locals.var_fck_dn5 * locals.var_latmax))) / (assign6860_e8530 * assign6860_e8530)), ((((-locals.var_fck_dn6) * assign6860_e8530) - (assign6860_e8527 * (locals.var_fck_dn6 * locals.var_latmax))) / (assign6860_e8530 * assign6860_e8530)), ((((-locals.var_fck_dn7) * assign6860_e8530) - (assign6860_e8527 * (locals.var_fck_dn7 * locals.var_latmax))) / (assign6860_e8530 * assign6860_e8530)), ((((-locals.var_fck_dn8) * assign6860_e8530) - (assign6860_e8527 * (locals.var_fck_dn8 * locals.var_latmax))) / (assign6860_e8530 * assign6860_e8530)), ((((-locals.var_fck_dn9) * assign6860_e8530) - (assign6860_e8527 * (locals.var_fck_dn9 * locals.var_latmax))) / (assign6860_e8530 * assign6860_e8530)),)
    } else {
        (locals.var_fcw, locals.var_fcw_dn0, locals.var_fcw_dn1, locals.var_fcw_dn3, locals.var_fcw_dn4, locals.var_fcw_dn5, locals.var_fcw_dn6, locals.var_fcw_dn7, locals.var_fcw_dn8, locals.var_fcw_dn9,)
    }
};
        locals.var_fcw = assign6860_e8533;
        locals.var_fcw_dn0 = assign6860_e8533_d_n0;
        locals.var_fcw_dn1 = assign6860_e8533_d_n1;
        locals.var_fcw_dn3 = assign6860_e8533_d_n3;
        locals.var_fcw_dn4 = assign6860_e8533_d_n4;
        locals.var_fcw_dn5 = assign6860_e8533_d_n5;
        locals.var_fcw_dn6 = assign6860_e8533_d_n6;
        locals.var_fcw_dn7 = assign6860_e8533_d_n7;
        locals.var_fcw_dn8 = assign6860_e8533_d_n8;
        locals.var_fcw_dn9 = assign6860_e8533_d_n9;
        locals.var_fcw_rv = 0.0;

        let (assign6870_e8550, assign6870_e8550_d_n0, assign6870_e8550_d_n1, assign6870_e8550_d_n3, assign6870_e8550_d_n4, assign6870_e8550_d_n5, assign6870_e8550_d_n6, assign6870_e8550_d_n7, assign6870_e8550_d_n8, assign6870_e8550_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 != 0.0)) {
        let assign6870_e8547: f64 = (locals.var_latmax * locals.var_fcw);
        let assign6870_e8548: f64 = (1.0 + assign6870_e8547);
        (assign6870_e8548, (locals.var_latmax * locals.var_fcw_dn0), (locals.var_latmax * locals.var_fcw_dn1), (locals.var_latmax * locals.var_fcw_dn3), (locals.var_latmax * locals.var_fcw_dn4), (locals.var_latmax * locals.var_fcw_dn5), (locals.var_latmax * locals.var_fcw_dn6), (locals.var_latmax * locals.var_fcw_dn7), (locals.var_latmax * locals.var_fcw_dn8), (locals.var_latmax * locals.var_fcw_dn9),)
    } else {
        (locals.var_fclatw_p1, locals.var_fclatw_p1_dn0, locals.var_fclatw_p1_dn1, locals.var_fclatw_p1_dn3, locals.var_fclatw_p1_dn4, locals.var_fclatw_p1_dn5, locals.var_fclatw_p1_dn6, locals.var_fclatw_p1_dn7, locals.var_fclatw_p1_dn8, locals.var_fclatw_p1_dn9,)
    }
};
        locals.var_fclatw_p1 = assign6870_e8550;
        locals.var_fclatw_p1_dn0 = assign6870_e8550_d_n0;
        locals.var_fclatw_p1_dn1 = assign6870_e8550_d_n1;
        locals.var_fclatw_p1_dn3 = assign6870_e8550_d_n3;
        locals.var_fclatw_p1_dn4 = assign6870_e8550_d_n4;
        locals.var_fclatw_p1_dn5 = assign6870_e8550_d_n5;
        locals.var_fclatw_p1_dn6 = assign6870_e8550_d_n6;
        locals.var_fclatw_p1_dn7 = assign6870_e8550_d_n7;
        locals.var_fclatw_p1_dn8 = assign6870_e8550_d_n8;
        locals.var_fclatw_p1_dn9 = assign6870_e8550_d_n9;
        locals.var_fclatw_p1_rv = 0.0;

        let (assign6880_e8584, assign6880_e8584_d_n0, assign6880_e8584_d_n1, assign6880_e8584_d_n3, assign6880_e8584_d_n4, assign6880_e8584_d_n5, assign6880_e8584_d_n6, assign6880_e8584_d_n7, assign6880_e8584_d_n8, assign6880_e8584_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 != 0.0)) {
        let assign6880_e8564: f64 = (locals.var_latmax * locals.var_fcw);
        let assign6880_e8568: f64 = (0.25 * locals.var_latmax);
        let assign6880_e8570: f64 = (assign6880_e8568 * locals.var_fcw);
        let assign6880_e8571: f64 = (0.5 + assign6880_e8570);
        let assign6880_e8572: f64 = (assign6880_e8564 * assign6880_e8571);
        let assign6880_e8575: f64 = (locals.var_fclatw_p1).ln();
        let assign6880_e8576: f64 = (0.5 * assign6880_e8575);
        let assign6880_e8577: f64 = (assign6880_e8572 - assign6880_e8576);
        let assign6880_e8578: f64 = (2.0 * assign6880_e8577);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_latmax;
        let assign6880_e8580: f64 = (assign6880_e8578 * __rspice_inv_cse_0);
        let assign6880_e8582: f64 = (assign6880_e8580 * __rspice_inv_cse_0);
        (assign6880_e8582, (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn0) * assign6880_e8571) + (assign6880_e8564 * (assign6880_e8568 * locals.var_fcw_dn0))) - (0.5 * (locals.var_fclatw_p1_dn0 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn1) * assign6880_e8571) + (assign6880_e8564 * (assign6880_e8568 * locals.var_fcw_dn1))) - (0.5 * (locals.var_fclatw_p1_dn1 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn3) * assign6880_e8571) + (assign6880_e8564 * (assign6880_e8568 * locals.var_fcw_dn3))) - (0.5 * (locals.var_fclatw_p1_dn3 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn4) * assign6880_e8571) + (assign6880_e8564 * (assign6880_e8568 * locals.var_fcw_dn4))) - (0.5 * (locals.var_fclatw_p1_dn4 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn5) * assign6880_e8571) + (assign6880_e8564 * (assign6880_e8568 * locals.var_fcw_dn5))) - (0.5 * (locals.var_fclatw_p1_dn5 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn6) * assign6880_e8571) + (assign6880_e8564 * (assign6880_e8568 * locals.var_fcw_dn6))) - (0.5 * (locals.var_fclatw_p1_dn6 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn7) * assign6880_e8571) + (assign6880_e8564 * (assign6880_e8568 * locals.var_fcw_dn7))) - (0.5 * (locals.var_fclatw_p1_dn7 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn8) * assign6880_e8571) + (assign6880_e8564 * (assign6880_e8568 * locals.var_fcw_dn8))) - (0.5 * (locals.var_fclatw_p1_dn8 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax), (((2.0 * ((((locals.var_latmax * locals.var_fcw_dn9) * assign6880_e8571) + (assign6880_e8564 * (assign6880_e8568 * locals.var_fcw_dn9))) - (0.5 * (locals.var_fclatw_p1_dn9 / locals.var_fclatw_p1)))) / locals.var_latmax) / locals.var_latmax),)
    } else {
        (locals.var_fcf_ci, locals.var_fcf_ci_dn0, locals.var_fcf_ci_dn1, locals.var_fcf_ci_dn3, locals.var_fcf_ci_dn4, locals.var_fcf_ci_dn5, locals.var_fcf_ci_dn6, locals.var_fcf_ci_dn7, locals.var_fcf_ci_dn8, locals.var_fcf_ci_dn9,)
    }
};
        locals.var_fcf_ci = assign6880_e8584;
        locals.var_fcf_ci_dn0 = assign6880_e8584_d_n0;
        locals.var_fcf_ci_dn1 = assign6880_e8584_d_n1;
        locals.var_fcf_ci_dn3 = assign6880_e8584_d_n3;
        locals.var_fcf_ci_dn4 = assign6880_e8584_d_n4;
        locals.var_fcf_ci_dn5 = assign6880_e8584_d_n5;
        locals.var_fcf_ci_dn6 = assign6880_e8584_d_n6;
        locals.var_fcf_ci_dn7 = assign6880_e8584_d_n7;
        locals.var_fcf_ci_dn8 = assign6880_e8584_d_n8;
        locals.var_fcf_ci_dn9 = assign6880_e8584_d_n9;
        locals.var_fcf_ci_rv = 0.0;

        let (assign6890_e8604, assign6890_e8604_d_n0, assign6890_e8604_d_n1, assign6890_e8604_d_n3, assign6890_e8604_d_n4, assign6890_e8604_d_n5, assign6890_e8604_d_n6, assign6890_e8604_d_n7, assign6890_e8604_d_n8, assign6890_e8604_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 != 0.0)) {
        let assign6890_e8596: f64 = (-locals.var_ln_lat);
        let assign6890_e8598: f64 = (assign6890_e8596 * locals.var_fcdick_ditf);
        let assign6890_e8601: f64 = (locals.var_fck * locals.var_latmax);
        let assign6890_e8602: f64 = (assign6890_e8598 / assign6890_e8601);
        (assign6890_e8602, ((((assign6890_e8596 * locals.var_fcdick_ditf_dn0) * assign6890_e8601) - (assign6890_e8598 * (locals.var_fck_dn0 * locals.var_latmax))) / (assign6890_e8601 * assign6890_e8601)), ((((assign6890_e8596 * locals.var_fcdick_ditf_dn1) * assign6890_e8601) - (assign6890_e8598 * (locals.var_fck_dn1 * locals.var_latmax))) / (assign6890_e8601 * assign6890_e8601)), ((((assign6890_e8596 * locals.var_fcdick_ditf_dn3) * assign6890_e8601) - (assign6890_e8598 * (locals.var_fck_dn3 * locals.var_latmax))) / (assign6890_e8601 * assign6890_e8601)), ((((assign6890_e8596 * locals.var_fcdick_ditf_dn4) * assign6890_e8601) - (assign6890_e8598 * (locals.var_fck_dn4 * locals.var_latmax))) / (assign6890_e8601 * assign6890_e8601)), ((((assign6890_e8596 * locals.var_fcdick_ditf_dn5) * assign6890_e8601) - (assign6890_e8598 * (locals.var_fck_dn5 * locals.var_latmax))) / (assign6890_e8601 * assign6890_e8601)), ((((assign6890_e8596 * locals.var_fcdick_ditf_dn6) * assign6890_e8601) - (assign6890_e8598 * (locals.var_fck_dn6 * locals.var_latmax))) / (assign6890_e8601 * assign6890_e8601)), ((((assign6890_e8596 * locals.var_fcdick_ditf_dn7) * assign6890_e8601) - (assign6890_e8598 * (locals.var_fck_dn7 * locals.var_latmax))) / (assign6890_e8601 * assign6890_e8601)), ((((assign6890_e8596 * locals.var_fcdick_ditf_dn8) * assign6890_e8601) - (assign6890_e8598 * (locals.var_fck_dn8 * locals.var_latmax))) / (assign6890_e8601 * assign6890_e8601)), ((((assign6890_e8596 * locals.var_fcdick_ditf_dn9) * assign6890_e8601) - (assign6890_e8598 * (locals.var_fck_dn9 * locals.var_latmax))) / (assign6890_e8601 * assign6890_e8601)),)
    } else {
        (locals.var_fcdw_ditf, locals.var_fcdw_ditf_dn0, locals.var_fcdw_ditf_dn1, locals.var_fcdw_ditf_dn3, locals.var_fcdw_ditf_dn4, locals.var_fcdw_ditf_dn5, locals.var_fcdw_ditf_dn6, locals.var_fcdw_ditf_dn7, locals.var_fcdw_ditf_dn8, locals.var_fcdw_ditf_dn9,)
    }
};
        locals.var_fcdw_ditf = assign6890_e8604;
        locals.var_fcdw_ditf_dn0 = assign6890_e8604_d_n0;
        locals.var_fcdw_ditf_dn1 = assign6890_e8604_d_n1;
        locals.var_fcdw_ditf_dn3 = assign6890_e8604_d_n3;
        locals.var_fcdw_ditf_dn4 = assign6890_e8604_d_n4;
        locals.var_fcdw_ditf_dn5 = assign6890_e8604_d_n5;
        locals.var_fcdw_ditf_dn6 = assign6890_e8604_d_n6;
        locals.var_fcdw_ditf_dn7 = assign6890_e8604_d_n7;
        locals.var_fcdw_ditf_dn8 = assign6890_e8604_d_n8;
        locals.var_fcdw_ditf_dn9 = assign6890_e8604_d_n9;
        locals.var_fcdw_ditf_rv = 0.0;

        let (assign6900_e8625, assign6900_e8625_d_n0, assign6900_e8625_d_n1, assign6900_e8625_d_n3, assign6900_e8625_d_n4, assign6900_e8625_d_n5, assign6900_e8625_d_n6, assign6900_e8625_d_n7, assign6900_e8625_d_n8, assign6900_e8625_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 != 0.0)) {
        let assign6900_e8617: f64 = (1.0 + locals.var_fclatw_p1);
        let assign6900_e8619: f64 = (assign6900_e8617 * locals.var_fcw);
        let assign6900_e8621: f64 = (assign6900_e8619 * locals.var_fcdw_ditf);
        let assign6900_e8623: f64 = (assign6900_e8621 / locals.var_fclatw_p1);
        (assign6900_e8623, (((((((locals.var_fclatw_p1_dn0 * locals.var_fcw) + (assign6900_e8617 * locals.var_fcw_dn0)) * locals.var_fcdw_ditf) + (assign6900_e8619 * locals.var_fcdw_ditf_dn0)) * locals.var_fclatw_p1) - (assign6900_e8621 * locals.var_fclatw_p1_dn0)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn1 * locals.var_fcw) + (assign6900_e8617 * locals.var_fcw_dn1)) * locals.var_fcdw_ditf) + (assign6900_e8619 * locals.var_fcdw_ditf_dn1)) * locals.var_fclatw_p1) - (assign6900_e8621 * locals.var_fclatw_p1_dn1)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn3 * locals.var_fcw) + (assign6900_e8617 * locals.var_fcw_dn3)) * locals.var_fcdw_ditf) + (assign6900_e8619 * locals.var_fcdw_ditf_dn3)) * locals.var_fclatw_p1) - (assign6900_e8621 * locals.var_fclatw_p1_dn3)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn4 * locals.var_fcw) + (assign6900_e8617 * locals.var_fcw_dn4)) * locals.var_fcdw_ditf) + (assign6900_e8619 * locals.var_fcdw_ditf_dn4)) * locals.var_fclatw_p1) - (assign6900_e8621 * locals.var_fclatw_p1_dn4)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn5 * locals.var_fcw) + (assign6900_e8617 * locals.var_fcw_dn5)) * locals.var_fcdw_ditf) + (assign6900_e8619 * locals.var_fcdw_ditf_dn5)) * locals.var_fclatw_p1) - (assign6900_e8621 * locals.var_fclatw_p1_dn5)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn6 * locals.var_fcw) + (assign6900_e8617 * locals.var_fcw_dn6)) * locals.var_fcdw_ditf) + (assign6900_e8619 * locals.var_fcdw_ditf_dn6)) * locals.var_fclatw_p1) - (assign6900_e8621 * locals.var_fclatw_p1_dn6)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn7 * locals.var_fcw) + (assign6900_e8617 * locals.var_fcw_dn7)) * locals.var_fcdw_ditf) + (assign6900_e8619 * locals.var_fcdw_ditf_dn7)) * locals.var_fclatw_p1) - (assign6900_e8621 * locals.var_fclatw_p1_dn7)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn8 * locals.var_fcw) + (assign6900_e8617 * locals.var_fcw_dn8)) * locals.var_fcdw_ditf) + (assign6900_e8619 * locals.var_fcdw_ditf_dn8)) * locals.var_fclatw_p1) - (assign6900_e8621 * locals.var_fclatw_p1_dn8)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)), (((((((locals.var_fclatw_p1_dn9 * locals.var_fcw) + (assign6900_e8617 * locals.var_fcw_dn9)) * locals.var_fcdw_ditf) + (assign6900_e8619 * locals.var_fcdw_ditf_dn9)) * locals.var_fclatw_p1) - (assign6900_e8621 * locals.var_fclatw_p1_dn9)) / (locals.var_fclatw_p1 * locals.var_fclatw_p1)),)
    } else {
        (locals.var_fcdfc_ditf, locals.var_fcdfc_ditf_dn0, locals.var_fcdfc_ditf_dn1, locals.var_fcdfc_ditf_dn3, locals.var_fcdfc_ditf_dn4, locals.var_fcdfc_ditf_dn5, locals.var_fcdfc_ditf_dn6, locals.var_fcdfc_ditf_dn7, locals.var_fcdfc_ditf_dn8, locals.var_fcdfc_ditf_dn9,)
    }
};
        locals.var_fcdfc_ditf = assign6900_e8625;
        locals.var_fcdfc_ditf_dn0 = assign6900_e8625_d_n0;
        locals.var_fcdfc_ditf_dn1 = assign6900_e8625_d_n1;
        locals.var_fcdfc_ditf_dn3 = assign6900_e8625_d_n3;
        locals.var_fcdfc_ditf_dn4 = assign6900_e8625_d_n4;
        locals.var_fcdfc_ditf_dn5 = assign6900_e8625_d_n5;
        locals.var_fcdfc_ditf_dn6 = assign6900_e8625_d_n6;
        locals.var_fcdfc_ditf_dn7 = assign6900_e8625_d_n7;
        locals.var_fcdfc_ditf_dn8 = assign6900_e8625_d_n8;
        locals.var_fcdfc_ditf_dn9 = assign6900_e8625_d_n9;
        locals.var_fcdfc_ditf_rv = 0.0;

        let (assign6910_e8643, assign6910_e8643_d_n0, assign6910_e8643_d_n1, assign6910_e8643_d_n3, assign6910_e8643_d_n4, assign6910_e8643_d_n5, assign6910_e8643_d_n6, assign6910_e8643_d_n7, assign6910_e8643_d_n8, assign6910_e8643_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 == 0.0)) {
        let assign6910_e8640: f64 = (locals.var_fck * p.p115);
        let assign6910_e8641: f64 = (p.p116 - assign6910_e8640);
        (assign6910_e8641, (-(locals.var_fck_dn0 * p.p115)), (-(locals.var_fck_dn1 * p.p115)), (-(locals.var_fck_dn3 * p.p115)), (-(locals.var_fck_dn4 * p.p115)), (-(locals.var_fck_dn5 * p.p115)), (-(locals.var_fck_dn6 * p.p115)), (-(locals.var_fck_dn7 * p.p115)), (-(locals.var_fck_dn8 * p.p115)), (-(locals.var_fck_dn9 * p.p115)),)
    } else {
        (locals.var_fckdelta, locals.var_fckdelta_dn0, locals.var_fckdelta_dn1, locals.var_fckdelta_dn3, locals.var_fckdelta_dn4, locals.var_fckdelta_dn5, locals.var_fckdelta_dn6, locals.var_fckdelta_dn7, locals.var_fckdelta_dn8, locals.var_fckdelta_dn9,)
    }
};
        locals.var_fckdelta = assign6910_e8643;
        locals.var_fckdelta_dn0 = assign6910_e8643_d_n0;
        locals.var_fckdelta_dn1 = assign6910_e8643_d_n1;
        locals.var_fckdelta_dn3 = assign6910_e8643_d_n3;
        locals.var_fckdelta_dn4 = assign6910_e8643_d_n4;
        locals.var_fckdelta_dn5 = assign6910_e8643_d_n5;
        locals.var_fckdelta_dn6 = assign6910_e8643_d_n6;
        locals.var_fckdelta_dn7 = assign6910_e8643_d_n7;
        locals.var_fckdelta_dn8 = assign6910_e8643_d_n8;
        locals.var_fckdelta_dn9 = assign6910_e8643_d_n9;
        locals.var_fckdelta_rv = 0.0;

        let (assign6920_e8661, assign6920_e8661_d_n0, assign6920_e8661_d_n1, assign6920_e8661_d_n3, assign6920_e8661_d_n4, assign6920_e8661_d_n5, assign6920_e8661_d_n6, assign6920_e8661_d_n7, assign6920_e8661_d_n8, assign6920_e8661_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 == 0.0)) {
        let assign6920_e8657: f64 = (locals.var_fck - 1.0);
        let assign6920_e8659: f64 = (assign6920_e8657 / locals.var_fckdelta);
        (assign6920_e8659, (((locals.var_fck_dn0 * locals.var_fckdelta) - (assign6920_e8657 * locals.var_fckdelta_dn0)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn1 * locals.var_fckdelta) - (assign6920_e8657 * locals.var_fckdelta_dn1)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn3 * locals.var_fckdelta) - (assign6920_e8657 * locals.var_fckdelta_dn3)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn4 * locals.var_fckdelta) - (assign6920_e8657 * locals.var_fckdelta_dn4)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn5 * locals.var_fckdelta) - (assign6920_e8657 * locals.var_fckdelta_dn5)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn6 * locals.var_fckdelta) - (assign6920_e8657 * locals.var_fckdelta_dn6)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn7 * locals.var_fckdelta) - (assign6920_e8657 * locals.var_fckdelta_dn7)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn8 * locals.var_fckdelta) - (assign6920_e8657 * locals.var_fckdelta_dn8)) / (locals.var_fckdelta * locals.var_fckdelta)), (((locals.var_fck_dn9 * locals.var_fckdelta) - (assign6920_e8657 * locals.var_fckdelta_dn9)) / (locals.var_fckdelta * locals.var_fckdelta)),)
    } else {
        (locals.var_fcw, locals.var_fcw_dn0, locals.var_fcw_dn1, locals.var_fcw_dn3, locals.var_fcw_dn4, locals.var_fcw_dn5, locals.var_fcw_dn6, locals.var_fcw_dn7, locals.var_fcw_dn8, locals.var_fcw_dn9,)
    }
};
        locals.var_fcw = assign6920_e8661;
        locals.var_fcw_dn0 = assign6920_e8661_d_n0;
        locals.var_fcw_dn1 = assign6920_e8661_d_n1;
        locals.var_fcw_dn3 = assign6920_e8661_d_n3;
        locals.var_fcw_dn4 = assign6920_e8661_d_n4;
        locals.var_fcw_dn5 = assign6920_e8661_d_n5;
        locals.var_fcw_dn6 = assign6920_e8661_d_n6;
        locals.var_fcw_dn7 = assign6920_e8661_d_n7;
        locals.var_fcw_dn8 = assign6920_e8661_d_n8;
        locals.var_fcw_dn9 = assign6920_e8661_d_n9;
        locals.var_fcw_rv = 0.0;

        let (assign6930_e8679, assign6930_e8679_d_n0, assign6930_e8679_d_n1, assign6930_e8679_d_n3, assign6930_e8679_d_n4, assign6930_e8679_d_n5, assign6930_e8679_d_n6, assign6930_e8679_d_n7, assign6930_e8679_d_n8, assign6930_e8679_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 == 0.0)) {
        let assign6930_e8676: f64 = (p.p116 * locals.var_fcw);
        let assign6930_e8677: f64 = (1.0 + assign6930_e8676);
        (assign6930_e8677, (p.p116 * locals.var_fcw_dn0), (p.p116 * locals.var_fcw_dn1), (p.p116 * locals.var_fcw_dn3), (p.p116 * locals.var_fcw_dn4), (p.p116 * locals.var_fcw_dn5), (p.p116 * locals.var_fcw_dn6), (p.p116 * locals.var_fcw_dn7), (p.p116 * locals.var_fcw_dn8), (p.p116 * locals.var_fcw_dn9),)
    } else {
        (locals.var_fciwzb_p1, locals.var_fciwzb_p1_dn0, locals.var_fciwzb_p1_dn1, locals.var_fciwzb_p1_dn3, locals.var_fciwzb_p1_dn4, locals.var_fciwzb_p1_dn5, locals.var_fciwzb_p1_dn6, locals.var_fciwzb_p1_dn7, locals.var_fciwzb_p1_dn8, locals.var_fciwzb_p1_dn9,)
    }
};
        locals.var_fciwzb_p1 = assign6930_e8679;
        locals.var_fciwzb_p1_dn0 = assign6930_e8679_d_n0;
        locals.var_fciwzb_p1_dn1 = assign6930_e8679_d_n1;
        locals.var_fciwzb_p1_dn3 = assign6930_e8679_d_n3;
        locals.var_fciwzb_p1_dn4 = assign6930_e8679_d_n4;
        locals.var_fciwzb_p1_dn5 = assign6930_e8679_d_n5;
        locals.var_fciwzb_p1_dn6 = assign6930_e8679_d_n6;
        locals.var_fciwzb_p1_dn7 = assign6930_e8679_d_n7;
        locals.var_fciwzb_p1_dn8 = assign6930_e8679_d_n8;
        locals.var_fciwzb_p1_dn9 = assign6930_e8679_d_n9;
        locals.var_fciwzb_p1_rv = 0.0;

        let (assign6940_e8694, assign6940_e8694_d_n0, assign6940_e8694_d_n1, assign6940_e8694_d_n3, assign6940_e8694_d_n4, assign6940_e8694_d_n5, assign6940_e8694_d_n6, assign6940_e8694_d_n7, assign6940_e8694_d_n8, assign6940_e8694_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 == 0.0)) {
        let assign6940_e8692: f64 = (locals.var_fciwzb_p1).ln();
        (assign6940_e8692, (locals.var_fciwzb_p1_dn0 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn1 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn3 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn4 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn5 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn6 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn7 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn8 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn9 / locals.var_fciwzb_p1),)
    } else {
        (locals.var_fcilnw_bl, locals.var_fcilnw_bl_dn0, locals.var_fcilnw_bl_dn1, locals.var_fcilnw_bl_dn3, locals.var_fcilnw_bl_dn4, locals.var_fcilnw_bl_dn5, locals.var_fcilnw_bl_dn6, locals.var_fcilnw_bl_dn7, locals.var_fcilnw_bl_dn8, locals.var_fcilnw_bl_dn9,)
    }
};
        locals.var_fcilnw_bl = assign6940_e8694;
        locals.var_fcilnw_bl_dn0 = assign6940_e8694_d_n0;
        locals.var_fcilnw_bl_dn1 = assign6940_e8694_d_n1;
        locals.var_fcilnw_bl_dn3 = assign6940_e8694_d_n3;
        locals.var_fcilnw_bl_dn4 = assign6940_e8694_d_n4;
        locals.var_fcilnw_bl_dn5 = assign6940_e8694_d_n5;
        locals.var_fcilnw_bl_dn6 = assign6940_e8694_d_n6;
        locals.var_fcilnw_bl_dn7 = assign6940_e8694_d_n7;
        locals.var_fcilnw_bl_dn8 = assign6940_e8694_d_n8;
        locals.var_fcilnw_bl_dn9 = assign6940_e8694_d_n9;
        locals.var_fcilnw_bl_rv = 0.0;

        let (assign6950_e8710,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 == 0.0)) {
        let assign6950_e8708: f64 = (locals.var_latb_6 * locals.var_inv_latl);
        (assign6950_e8708,)
    } else {
        (locals.var_fcia,)
    }
};
        locals.var_fcia = assign6950_e8710;
        locals.var_fcia_rv = 0.0;

        let (assign6960_e8738, assign6960_e8738_d_n0, assign6960_e8738_d_n1, assign6960_e8738_d_n3, assign6960_e8738_d_n4, assign6960_e8738_d_n5, assign6960_e8738_d_n6, assign6960_e8738_d_n7, assign6960_e8738_d_n8, assign6960_e8738_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 == 0.0)) {
        let assign6960_e8725: f64 = (0.5 - locals.var_fcia);
        let assign6960_e8726: f64 = (locals.var_fcilnw_bl * assign6960_e8725);
        let assign6960_e8728: f64 = (assign6960_e8726 * locals.var_inv_latl);
        let assign6960_e8732: f64 = (locals.var_latb_6 * locals.var_fcw);
        let assign6960_e8733: f64 = (locals.var_fcia + assign6960_e8732);
        let assign6960_e8735: f64 = (assign6960_e8733 * locals.var_fcw);
        let assign6960_e8736: f64 = (assign6960_e8728 + assign6960_e8735);
        (assign6960_e8736, (((locals.var_fcilnw_bl_dn0 * assign6960_e8725) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn0) * locals.var_fcw) + (assign6960_e8733 * locals.var_fcw_dn0))), (((locals.var_fcilnw_bl_dn1 * assign6960_e8725) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn1) * locals.var_fcw) + (assign6960_e8733 * locals.var_fcw_dn1))), (((locals.var_fcilnw_bl_dn3 * assign6960_e8725) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn3) * locals.var_fcw) + (assign6960_e8733 * locals.var_fcw_dn3))), (((locals.var_fcilnw_bl_dn4 * assign6960_e8725) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn4) * locals.var_fcw) + (assign6960_e8733 * locals.var_fcw_dn4))), (((locals.var_fcilnw_bl_dn5 * assign6960_e8725) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn5) * locals.var_fcw) + (assign6960_e8733 * locals.var_fcw_dn5))), (((locals.var_fcilnw_bl_dn6 * assign6960_e8725) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn6) * locals.var_fcw) + (assign6960_e8733 * locals.var_fcw_dn6))), (((locals.var_fcilnw_bl_dn7 * assign6960_e8725) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn7) * locals.var_fcw) + (assign6960_e8733 * locals.var_fcw_dn7))), (((locals.var_fcilnw_bl_dn8 * assign6960_e8725) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn8) * locals.var_fcw) + (assign6960_e8733 * locals.var_fcw_dn8))), (((locals.var_fcilnw_bl_dn9 * assign6960_e8725) * locals.var_inv_latl) + (((locals.var_latb_6 * locals.var_fcw_dn9) * locals.var_fcw) + (assign6960_e8733 * locals.var_fcw_dn9))),)
    } else {
        (locals.var_fcf_csl, locals.var_fcf_csl_dn0, locals.var_fcf_csl_dn1, locals.var_fcf_csl_dn3, locals.var_fcf_csl_dn4, locals.var_fcf_csl_dn5, locals.var_fcf_csl_dn6, locals.var_fcf_csl_dn7, locals.var_fcf_csl_dn8, locals.var_fcf_csl_dn9,)
    }
};
        locals.var_fcf_csl = assign6960_e8738;
        locals.var_fcf_csl_dn0 = assign6960_e8738_d_n0;
        locals.var_fcf_csl_dn1 = assign6960_e8738_d_n1;
        locals.var_fcf_csl_dn3 = assign6960_e8738_d_n3;
        locals.var_fcf_csl_dn4 = assign6960_e8738_d_n4;
        locals.var_fcf_csl_dn5 = assign6960_e8738_d_n5;
        locals.var_fcf_csl_dn6 = assign6960_e8738_d_n6;
        locals.var_fcf_csl_dn7 = assign6960_e8738_d_n7;
        locals.var_fcf_csl_dn8 = assign6960_e8738_d_n8;
        locals.var_fcf_csl_dn9 = assign6960_e8738_d_n9;
        locals.var_fcf_csl_rv = 0.0;

        let (assign6970_e8764, assign6970_e8764_d_n0, assign6970_e8764_d_n1, assign6970_e8764_d_n3, assign6970_e8764_d_n4, assign6970_e8764_d_n5, assign6970_e8764_d_n6, assign6970_e8764_d_n7, assign6970_e8764_d_n8, assign6970_e8764_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 == 0.0)) {
        let assign6970_e8752: f64 = (0.5 - locals.var_fcia);
        let assign6970_e8754: f64 = (assign6970_e8752 / locals.var_fciwzb_p1);
        let assign6970_e8756: f64 = (assign6970_e8754 + locals.var_fcia);
        let assign6970_e8759: f64 = (locals.var_fcw * locals.var_latb_6);
        let assign6970_e8761: f64 = (assign6970_e8759 * 2.0);
        let assign6970_e8762: f64 = (assign6970_e8756 + assign6970_e8761);
        (assign6970_e8762, ((-((assign6970_e8752 * locals.var_fciwzb_p1_dn0) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn0 * locals.var_latb_6) * 2.0)), ((-((assign6970_e8752 * locals.var_fciwzb_p1_dn1) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn1 * locals.var_latb_6) * 2.0)), ((-((assign6970_e8752 * locals.var_fciwzb_p1_dn3) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn3 * locals.var_latb_6) * 2.0)), ((-((assign6970_e8752 * locals.var_fciwzb_p1_dn4) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn4 * locals.var_latb_6) * 2.0)), ((-((assign6970_e8752 * locals.var_fciwzb_p1_dn5) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn5 * locals.var_latb_6) * 2.0)), ((-((assign6970_e8752 * locals.var_fciwzb_p1_dn6) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn6 * locals.var_latb_6) * 2.0)), ((-((assign6970_e8752 * locals.var_fciwzb_p1_dn7) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn7 * locals.var_latb_6) * 2.0)), ((-((assign6970_e8752 * locals.var_fciwzb_p1_dn8) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn8 * locals.var_latb_6) * 2.0)), ((-((assign6970_e8752 * locals.var_fciwzb_p1_dn9) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn9 * locals.var_latb_6) * 2.0)),)
    } else {
        (locals.var_fcdfcsl_dw, locals.var_fcdfcsl_dw_dn0, locals.var_fcdfcsl_dw_dn1, locals.var_fcdfcsl_dw_dn3, locals.var_fcdfcsl_dw_dn4, locals.var_fcdfcsl_dw_dn5, locals.var_fcdfcsl_dw_dn6, locals.var_fcdfcsl_dw_dn7, locals.var_fcdfcsl_dw_dn8, locals.var_fcdfcsl_dw_dn9,)
    }
};
        locals.var_fcdfcsl_dw = assign6970_e8764;
        locals.var_fcdfcsl_dw_dn0 = assign6970_e8764_d_n0;
        locals.var_fcdfcsl_dw_dn1 = assign6970_e8764_d_n1;
        locals.var_fcdfcsl_dw_dn3 = assign6970_e8764_d_n3;
        locals.var_fcdfcsl_dw_dn4 = assign6970_e8764_d_n4;
        locals.var_fcdfcsl_dw_dn5 = assign6970_e8764_d_n5;
        locals.var_fcdfcsl_dw_dn6 = assign6970_e8764_d_n6;
        locals.var_fcdfcsl_dw_dn7 = assign6970_e8764_d_n7;
        locals.var_fcdfcsl_dw_dn8 = assign6970_e8764_d_n8;
        locals.var_fcdfcsl_dw_dn9 = assign6970_e8764_d_n9;
        locals.var_fcdfcsl_dw_rv = 0.0;

        let (assign6980_e8782, assign6980_e8782_d_n0, assign6980_e8782_d_n1, assign6980_e8782_d_n3, assign6980_e8782_d_n4, assign6980_e8782_d_n5, assign6980_e8782_d_n6, assign6980_e8782_d_n7, assign6980_e8782_d_n8, assign6980_e8782_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 == 0.0)) {
        let assign6980_e8779: f64 = (p.p115 * locals.var_fcw);
        let assign6980_e8780: f64 = (1.0 + assign6980_e8779);
        (assign6980_e8780, (p.p115 * locals.var_fcw_dn0), (p.p115 * locals.var_fcw_dn1), (p.p115 * locals.var_fcw_dn3), (p.p115 * locals.var_fcw_dn4), (p.p115 * locals.var_fcw_dn5), (p.p115 * locals.var_fcw_dn6), (p.p115 * locals.var_fcw_dn7), (p.p115 * locals.var_fcw_dn8), (p.p115 * locals.var_fcw_dn9),)
    } else {
        (locals.var_fciwzb_p1, locals.var_fciwzb_p1_dn0, locals.var_fciwzb_p1_dn1, locals.var_fciwzb_p1_dn3, locals.var_fciwzb_p1_dn4, locals.var_fciwzb_p1_dn5, locals.var_fciwzb_p1_dn6, locals.var_fciwzb_p1_dn7, locals.var_fciwzb_p1_dn8, locals.var_fciwzb_p1_dn9,)
    }
};
        locals.var_fciwzb_p1 = assign6980_e8782;
        locals.var_fciwzb_p1_dn0 = assign6980_e8782_d_n0;
        locals.var_fciwzb_p1_dn1 = assign6980_e8782_d_n1;
        locals.var_fciwzb_p1_dn3 = assign6980_e8782_d_n3;
        locals.var_fciwzb_p1_dn4 = assign6980_e8782_d_n4;
        locals.var_fciwzb_p1_dn5 = assign6980_e8782_d_n5;
        locals.var_fciwzb_p1_dn6 = assign6980_e8782_d_n6;
        locals.var_fciwzb_p1_dn7 = assign6980_e8782_d_n7;
        locals.var_fciwzb_p1_dn8 = assign6980_e8782_d_n8;
        locals.var_fciwzb_p1_dn9 = assign6980_e8782_d_n9;
        locals.var_fciwzb_p1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_19(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6990_e8797, assign6990_e8797_d_n0, assign6990_e8797_d_n1, assign6990_e8797_d_n3, assign6990_e8797_d_n4, assign6990_e8797_d_n5, assign6990_e8797_d_n6, assign6990_e8797_d_n7, assign6990_e8797_d_n8, assign6990_e8797_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 == 0.0)) {
        let assign6990_e8795: f64 = (locals.var_fciwzb_p1).ln();
        (assign6990_e8795, (locals.var_fciwzb_p1_dn0 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn1 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn3 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn4 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn5 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn6 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn7 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn8 / locals.var_fciwzb_p1), (locals.var_fciwzb_p1_dn9 / locals.var_fciwzb_p1),)
    } else {
        (locals.var_fcilnw_bl, locals.var_fcilnw_bl_dn0, locals.var_fcilnw_bl_dn1, locals.var_fcilnw_bl_dn3, locals.var_fcilnw_bl_dn4, locals.var_fcilnw_bl_dn5, locals.var_fcilnw_bl_dn6, locals.var_fcilnw_bl_dn7, locals.var_fcilnw_bl_dn8, locals.var_fcilnw_bl_dn9,)
    }
};
        locals.var_fcilnw_bl = assign6990_e8797;
        locals.var_fcilnw_bl_dn0 = assign6990_e8797_d_n0;
        locals.var_fcilnw_bl_dn1 = assign6990_e8797_d_n1;
        locals.var_fcilnw_bl_dn3 = assign6990_e8797_d_n3;
        locals.var_fcilnw_bl_dn4 = assign6990_e8797_d_n4;
        locals.var_fcilnw_bl_dn5 = assign6990_e8797_d_n5;
        locals.var_fcilnw_bl_dn6 = assign6990_e8797_d_n6;
        locals.var_fcilnw_bl_dn7 = assign6990_e8797_d_n7;
        locals.var_fcilnw_bl_dn8 = assign6990_e8797_d_n8;
        locals.var_fcilnw_bl_dn9 = assign6990_e8797_d_n9;
        locals.var_fcilnw_bl_rv = 0.0;

        let (assign7000_e8813,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 == 0.0)) {
        let assign7000_e8811: f64 = (locals.var_latl_6 * locals.var_inv_latb);
        (assign7000_e8811,)
    } else {
        (locals.var_fcia,)
    }
};
        locals.var_fcia = assign7000_e8813;
        locals.var_fcia_rv = 0.0;

        let (assign7010_e8841, assign7010_e8841_d_n0, assign7010_e8841_d_n1, assign7010_e8841_d_n3, assign7010_e8841_d_n4, assign7010_e8841_d_n5, assign7010_e8841_d_n6, assign7010_e8841_d_n7, assign7010_e8841_d_n8, assign7010_e8841_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 == 0.0)) {
        let assign7010_e8828: f64 = (0.5 - locals.var_fcia);
        let assign7010_e8829: f64 = (locals.var_fcilnw_bl * assign7010_e8828);
        let assign7010_e8831: f64 = (assign7010_e8829 * locals.var_inv_latb);
        let assign7010_e8835: f64 = (locals.var_latl_6 * locals.var_fcw);
        let assign7010_e8836: f64 = (locals.var_fcia + assign7010_e8835);
        let assign7010_e8838: f64 = (assign7010_e8836 * locals.var_fcw);
        let assign7010_e8839: f64 = (assign7010_e8831 + assign7010_e8838);
        (assign7010_e8839, (((locals.var_fcilnw_bl_dn0 * assign7010_e8828) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn0) * locals.var_fcw) + (assign7010_e8836 * locals.var_fcw_dn0))), (((locals.var_fcilnw_bl_dn1 * assign7010_e8828) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn1) * locals.var_fcw) + (assign7010_e8836 * locals.var_fcw_dn1))), (((locals.var_fcilnw_bl_dn3 * assign7010_e8828) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn3) * locals.var_fcw) + (assign7010_e8836 * locals.var_fcw_dn3))), (((locals.var_fcilnw_bl_dn4 * assign7010_e8828) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn4) * locals.var_fcw) + (assign7010_e8836 * locals.var_fcw_dn4))), (((locals.var_fcilnw_bl_dn5 * assign7010_e8828) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn5) * locals.var_fcw) + (assign7010_e8836 * locals.var_fcw_dn5))), (((locals.var_fcilnw_bl_dn6 * assign7010_e8828) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn6) * locals.var_fcw) + (assign7010_e8836 * locals.var_fcw_dn6))), (((locals.var_fcilnw_bl_dn7 * assign7010_e8828) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn7) * locals.var_fcw) + (assign7010_e8836 * locals.var_fcw_dn7))), (((locals.var_fcilnw_bl_dn8 * assign7010_e8828) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn8) * locals.var_fcw) + (assign7010_e8836 * locals.var_fcw_dn8))), (((locals.var_fcilnw_bl_dn9 * assign7010_e8828) * locals.var_inv_latb) + (((locals.var_latl_6 * locals.var_fcw_dn9) * locals.var_fcw) + (assign7010_e8836 * locals.var_fcw_dn9))),)
    } else {
        (locals.var_fcf_csb, locals.var_fcf_csb_dn0, locals.var_fcf_csb_dn1, locals.var_fcf_csb_dn3, locals.var_fcf_csb_dn4, locals.var_fcf_csb_dn5, locals.var_fcf_csb_dn6, locals.var_fcf_csb_dn7, locals.var_fcf_csb_dn8, locals.var_fcf_csb_dn9,)
    }
};
        locals.var_fcf_csb = assign7010_e8841;
        locals.var_fcf_csb_dn0 = assign7010_e8841_d_n0;
        locals.var_fcf_csb_dn1 = assign7010_e8841_d_n1;
        locals.var_fcf_csb_dn3 = assign7010_e8841_d_n3;
        locals.var_fcf_csb_dn4 = assign7010_e8841_d_n4;
        locals.var_fcf_csb_dn5 = assign7010_e8841_d_n5;
        locals.var_fcf_csb_dn6 = assign7010_e8841_d_n6;
        locals.var_fcf_csb_dn7 = assign7010_e8841_d_n7;
        locals.var_fcf_csb_dn8 = assign7010_e8841_d_n8;
        locals.var_fcf_csb_dn9 = assign7010_e8841_d_n9;
        locals.var_fcf_csb_rv = 0.0;

        let (assign7020_e8867, assign7020_e8867_d_n0, assign7020_e8867_d_n1, assign7020_e8867_d_n3, assign7020_e8867_d_n4, assign7020_e8867_d_n5, assign7020_e8867_d_n6, assign7020_e8867_d_n7, assign7020_e8867_d_n8, assign7020_e8867_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 == 0.0)) {
        let assign7020_e8855: f64 = (0.5 - locals.var_fcia);
        let assign7020_e8857: f64 = (assign7020_e8855 / locals.var_fciwzb_p1);
        let assign7020_e8859: f64 = (assign7020_e8857 + locals.var_fcia);
        let assign7020_e8862: f64 = (locals.var_fcw * locals.var_latl_6);
        let assign7020_e8864: f64 = (assign7020_e8862 * 2.0);
        let assign7020_e8865: f64 = (assign7020_e8859 + assign7020_e8864);
        (assign7020_e8865, ((-((assign7020_e8855 * locals.var_fciwzb_p1_dn0) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn0 * locals.var_latl_6) * 2.0)), ((-((assign7020_e8855 * locals.var_fciwzb_p1_dn1) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn1 * locals.var_latl_6) * 2.0)), ((-((assign7020_e8855 * locals.var_fciwzb_p1_dn3) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn3 * locals.var_latl_6) * 2.0)), ((-((assign7020_e8855 * locals.var_fciwzb_p1_dn4) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn4 * locals.var_latl_6) * 2.0)), ((-((assign7020_e8855 * locals.var_fciwzb_p1_dn5) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn5 * locals.var_latl_6) * 2.0)), ((-((assign7020_e8855 * locals.var_fciwzb_p1_dn6) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn6 * locals.var_latl_6) * 2.0)), ((-((assign7020_e8855 * locals.var_fciwzb_p1_dn7) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn7 * locals.var_latl_6) * 2.0)), ((-((assign7020_e8855 * locals.var_fciwzb_p1_dn8) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn8 * locals.var_latl_6) * 2.0)), ((-((assign7020_e8855 * locals.var_fciwzb_p1_dn9) / (locals.var_fciwzb_p1 * locals.var_fciwzb_p1))) + ((locals.var_fcw_dn9 * locals.var_latl_6) * 2.0)),)
    } else {
        (locals.var_fcdfcsb_dw, locals.var_fcdfcsb_dw_dn0, locals.var_fcdfcsb_dw_dn1, locals.var_fcdfcsb_dw_dn3, locals.var_fcdfcsb_dw_dn4, locals.var_fcdfcsb_dw_dn5, locals.var_fcdfcsb_dw_dn6, locals.var_fcdfcsb_dw_dn7, locals.var_fcdfcsb_dw_dn8, locals.var_fcdfcsb_dw_dn9,)
    }
};
        locals.var_fcdfcsb_dw = assign7020_e8867;
        locals.var_fcdfcsb_dw_dn0 = assign7020_e8867_d_n0;
        locals.var_fcdfcsb_dw_dn1 = assign7020_e8867_d_n1;
        locals.var_fcdfcsb_dw_dn3 = assign7020_e8867_d_n3;
        locals.var_fcdfcsb_dw_dn4 = assign7020_e8867_d_n4;
        locals.var_fcdfcsb_dw_dn5 = assign7020_e8867_d_n5;
        locals.var_fcdfcsb_dw_dn6 = assign7020_e8867_d_n6;
        locals.var_fcdfcsb_dw_dn7 = assign7020_e8867_d_n7;
        locals.var_fcdfcsb_dw_dn8 = assign7020_e8867_d_n8;
        locals.var_fcdfcsb_dw_dn9 = assign7020_e8867_d_n9;
        locals.var_fcdfcsb_dw_rv = 0.0;

        let (assign7030_e8885, assign7030_e8885_d_n0, assign7030_e8885_d_n1, assign7030_e8885_d_n3, assign7030_e8885_d_n4, assign7030_e8885_d_n5, assign7030_e8885_d_n6, assign7030_e8885_d_n7, assign7030_e8885_d_n8, assign7030_e8885_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 == 0.0)) {
        let assign7030_e8881: f64 = (locals.var_fcf_csl - locals.var_fcf_csb);
        let assign7030_e8883: f64 = (assign7030_e8881 / locals.var_lat_delta);
        (assign7030_e8883, ((locals.var_fcf_csl_dn0 - locals.var_fcf_csb_dn0) / locals.var_lat_delta), ((locals.var_fcf_csl_dn1 - locals.var_fcf_csb_dn1) / locals.var_lat_delta), ((locals.var_fcf_csl_dn3 - locals.var_fcf_csb_dn3) / locals.var_lat_delta), ((locals.var_fcf_csl_dn4 - locals.var_fcf_csb_dn4) / locals.var_lat_delta), ((locals.var_fcf_csl_dn5 - locals.var_fcf_csb_dn5) / locals.var_lat_delta), ((locals.var_fcf_csl_dn6 - locals.var_fcf_csb_dn6) / locals.var_lat_delta), ((locals.var_fcf_csl_dn7 - locals.var_fcf_csb_dn7) / locals.var_lat_delta), ((locals.var_fcf_csl_dn8 - locals.var_fcf_csb_dn8) / locals.var_lat_delta), ((locals.var_fcf_csl_dn9 - locals.var_fcf_csb_dn9) / locals.var_lat_delta),)
    } else {
        (locals.var_fcf_ci, locals.var_fcf_ci_dn0, locals.var_fcf_ci_dn1, locals.var_fcf_ci_dn3, locals.var_fcf_ci_dn4, locals.var_fcf_ci_dn5, locals.var_fcf_ci_dn6, locals.var_fcf_ci_dn7, locals.var_fcf_ci_dn8, locals.var_fcf_ci_dn9,)
    }
};
        locals.var_fcf_ci = assign7030_e8885;
        locals.var_fcf_ci_dn0 = assign7030_e8885_d_n0;
        locals.var_fcf_ci_dn1 = assign7030_e8885_d_n1;
        locals.var_fcf_ci_dn3 = assign7030_e8885_d_n3;
        locals.var_fcf_ci_dn4 = assign7030_e8885_d_n4;
        locals.var_fcf_ci_dn5 = assign7030_e8885_d_n5;
        locals.var_fcf_ci_dn6 = assign7030_e8885_d_n6;
        locals.var_fcf_ci_dn7 = assign7030_e8885_d_n7;
        locals.var_fcf_ci_dn8 = assign7030_e8885_d_n8;
        locals.var_fcf_ci_dn9 = assign7030_e8885_d_n9;
        locals.var_fcf_ci_rv = 0.0;

        let (assign7040_e8912, assign7040_e8912_d_n0, assign7040_e8912_d_n1, assign7040_e8912_d_n3, assign7040_e8912_d_n4, assign7040_e8912_d_n5, assign7040_e8912_d_n6, assign7040_e8912_d_n7, assign7040_e8912_d_n8, assign7040_e8912_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 == 0.0)) {
        let assign7040_e8898: f64 = (-2.0);
        let assign7040_e8900: f64 = (assign7040_e8898 * locals.var_lat_delta);
        let assign7040_e8903: f64 = (locals.var_fckdelta * locals.var_fckdelta);
        let assign7040_e8904: f64 = (assign7040_e8900 / assign7040_e8903);
        let assign7040_e8906: f64 = (assign7040_e8904 * locals.var_fck);
        let assign7040_e8908: f64 = (assign7040_e8906 * locals.var_ln_lat);
        let assign7040_e8910: f64 = (assign7040_e8908 * locals.var_fcdick_ditf);
        (assign7040_e8910, ((((((-((assign7040_e8900 * ((locals.var_fckdelta_dn0 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn0))) / (assign7040_e8903 * assign7040_e8903))) * locals.var_fck) + (assign7040_e8904 * locals.var_fck_dn0)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign7040_e8908 * locals.var_fcdick_ditf_dn0)), ((((((-((assign7040_e8900 * ((locals.var_fckdelta_dn1 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn1))) / (assign7040_e8903 * assign7040_e8903))) * locals.var_fck) + (assign7040_e8904 * locals.var_fck_dn1)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign7040_e8908 * locals.var_fcdick_ditf_dn1)), ((((((-((assign7040_e8900 * ((locals.var_fckdelta_dn3 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn3))) / (assign7040_e8903 * assign7040_e8903))) * locals.var_fck) + (assign7040_e8904 * locals.var_fck_dn3)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign7040_e8908 * locals.var_fcdick_ditf_dn3)), ((((((-((assign7040_e8900 * ((locals.var_fckdelta_dn4 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn4))) / (assign7040_e8903 * assign7040_e8903))) * locals.var_fck) + (assign7040_e8904 * locals.var_fck_dn4)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign7040_e8908 * locals.var_fcdick_ditf_dn4)), ((((((-((assign7040_e8900 * ((locals.var_fckdelta_dn5 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn5))) / (assign7040_e8903 * assign7040_e8903))) * locals.var_fck) + (assign7040_e8904 * locals.var_fck_dn5)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign7040_e8908 * locals.var_fcdick_ditf_dn5)), ((((((-((assign7040_e8900 * ((locals.var_fckdelta_dn6 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn6))) / (assign7040_e8903 * assign7040_e8903))) * locals.var_fck) + (assign7040_e8904 * locals.var_fck_dn6)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign7040_e8908 * locals.var_fcdick_ditf_dn6)), ((((((-((assign7040_e8900 * ((locals.var_fckdelta_dn7 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn7))) / (assign7040_e8903 * assign7040_e8903))) * locals.var_fck) + (assign7040_e8904 * locals.var_fck_dn7)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign7040_e8908 * locals.var_fcdick_ditf_dn7)), ((((((-((assign7040_e8900 * ((locals.var_fckdelta_dn8 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn8))) / (assign7040_e8903 * assign7040_e8903))) * locals.var_fck) + (assign7040_e8904 * locals.var_fck_dn8)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign7040_e8908 * locals.var_fcdick_ditf_dn8)), ((((((-((assign7040_e8900 * ((locals.var_fckdelta_dn9 * locals.var_fckdelta) + (locals.var_fckdelta * locals.var_fckdelta_dn9))) / (assign7040_e8903 * assign7040_e8903))) * locals.var_fck) + (assign7040_e8904 * locals.var_fck_dn9)) * locals.var_ln_lat) * locals.var_fcdick_ditf) + (assign7040_e8908 * locals.var_fcdick_ditf_dn9)),)
    } else {
        (locals.var_fcdw_ditf, locals.var_fcdw_ditf_dn0, locals.var_fcdw_ditf_dn1, locals.var_fcdw_ditf_dn3, locals.var_fcdw_ditf_dn4, locals.var_fcdw_ditf_dn5, locals.var_fcdw_ditf_dn6, locals.var_fcdw_ditf_dn7, locals.var_fcdw_ditf_dn8, locals.var_fcdw_ditf_dn9,)
    }
};
        locals.var_fcdw_ditf = assign7040_e8912;
        locals.var_fcdw_ditf_dn0 = assign7040_e8912_d_n0;
        locals.var_fcdw_ditf_dn1 = assign7040_e8912_d_n1;
        locals.var_fcdw_ditf_dn3 = assign7040_e8912_d_n3;
        locals.var_fcdw_ditf_dn4 = assign7040_e8912_d_n4;
        locals.var_fcdw_ditf_dn5 = assign7040_e8912_d_n5;
        locals.var_fcdw_ditf_dn6 = assign7040_e8912_d_n6;
        locals.var_fcdw_ditf_dn7 = assign7040_e8912_d_n7;
        locals.var_fcdw_ditf_dn8 = assign7040_e8912_d_n8;
        locals.var_fcdw_ditf_dn9 = assign7040_e8912_d_n9;
        locals.var_fcdw_ditf_rv = 0.0;

        let (assign7050_e8932, assign7050_e8932_d_n0, assign7050_e8932_d_n1, assign7050_e8932_d_n3, assign7050_e8932_d_n4, assign7050_e8932_d_n5, assign7050_e8932_d_n6, assign7050_e8932_d_n7, assign7050_e8932_d_n8, assign7050_e8932_d_n9,) = {
    if (((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 == 0.0)) {
        let assign7050_e8926: f64 = (locals.var_fcdfcsl_dw - locals.var_fcdfcsb_dw);
        let assign7050_e8928: f64 = (assign7050_e8926 * locals.var_fcdw_ditf);
        let assign7050_e8930: f64 = (assign7050_e8928 / locals.var_lat_delta);
        (assign7050_e8930, ((((locals.var_fcdfcsl_dw_dn0 - locals.var_fcdfcsb_dw_dn0) * locals.var_fcdw_ditf) + (assign7050_e8926 * locals.var_fcdw_ditf_dn0)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn1 - locals.var_fcdfcsb_dw_dn1) * locals.var_fcdw_ditf) + (assign7050_e8926 * locals.var_fcdw_ditf_dn1)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn3 - locals.var_fcdfcsb_dw_dn3) * locals.var_fcdw_ditf) + (assign7050_e8926 * locals.var_fcdw_ditf_dn3)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn4 - locals.var_fcdfcsb_dw_dn4) * locals.var_fcdw_ditf) + (assign7050_e8926 * locals.var_fcdw_ditf_dn4)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn5 - locals.var_fcdfcsb_dw_dn5) * locals.var_fcdw_ditf) + (assign7050_e8926 * locals.var_fcdw_ditf_dn5)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn6 - locals.var_fcdfcsb_dw_dn6) * locals.var_fcdw_ditf) + (assign7050_e8926 * locals.var_fcdw_ditf_dn6)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn7 - locals.var_fcdfcsb_dw_dn7) * locals.var_fcdw_ditf) + (assign7050_e8926 * locals.var_fcdw_ditf_dn7)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn8 - locals.var_fcdfcsb_dw_dn8) * locals.var_fcdw_ditf) + (assign7050_e8926 * locals.var_fcdw_ditf_dn8)) / locals.var_lat_delta), ((((locals.var_fcdfcsl_dw_dn9 - locals.var_fcdfcsb_dw_dn9) * locals.var_fcdw_ditf) + (assign7050_e8926 * locals.var_fcdw_ditf_dn9)) / locals.var_lat_delta),)
    } else {
        (locals.var_fcdfc_ditf, locals.var_fcdfc_ditf_dn0, locals.var_fcdfc_ditf_dn1, locals.var_fcdfc_ditf_dn3, locals.var_fcdfc_ditf_dn4, locals.var_fcdfc_ditf_dn5, locals.var_fcdfc_ditf_dn6, locals.var_fcdfc_ditf_dn7, locals.var_fcdfc_ditf_dn8, locals.var_fcdfc_ditf_dn9,)
    }
};
        locals.var_fcdfc_ditf = assign7050_e8932;
        locals.var_fcdfc_ditf_dn0 = assign7050_e8932_d_n0;
        locals.var_fcdfc_ditf_dn1 = assign7050_e8932_d_n1;
        locals.var_fcdfc_ditf_dn3 = assign7050_e8932_d_n3;
        locals.var_fcdfc_ditf_dn4 = assign7050_e8932_d_n4;
        locals.var_fcdfc_ditf_dn5 = assign7050_e8932_d_n5;
        locals.var_fcdfc_ditf_dn6 = assign7050_e8932_d_n6;
        locals.var_fcdfc_ditf_dn7 = assign7050_e8932_d_n7;
        locals.var_fcdfc_ditf_dn8 = assign7050_e8932_d_n8;
        locals.var_fcdfc_ditf_dn9 = assign7050_e8932_d_n9;
        locals.var_fcdfc_ditf_rv = 0.0;

        let (assign7060_e8952, assign7060_e8952_d_n0, assign7060_e8952_d_n1, assign7060_e8952_d_n3, assign7060_e8952_d_n4, assign7060_e8952_d_n5, assign7060_e8952_d_n6, assign7060_e8952_d_n7, assign7060_e8952_d_n8, assign7060_e8952_d_n9,) = {
    if ((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 == 0.0)) {
        let assign7060_e8944: f64 = (1.0 - locals.var_fcick);
        let assign7060_e8948: f64 = (locals.var_fcick * p.p115);
        let assign7060_e8949: f64 = (1.0 + assign7060_e8948);
        let assign7060_e8950: f64 = (assign7060_e8944 / assign7060_e8949);
        (assign7060_e8950, ((((-locals.var_fcick_dn0) * assign7060_e8949) - (assign7060_e8944 * (locals.var_fcick_dn0 * p.p115))) / (assign7060_e8949 * assign7060_e8949)), ((((-locals.var_fcick_dn1) * assign7060_e8949) - (assign7060_e8944 * (locals.var_fcick_dn1 * p.p115))) / (assign7060_e8949 * assign7060_e8949)), ((((-locals.var_fcick_dn3) * assign7060_e8949) - (assign7060_e8944 * (locals.var_fcick_dn3 * p.p115))) / (assign7060_e8949 * assign7060_e8949)), ((((-locals.var_fcick_dn4) * assign7060_e8949) - (assign7060_e8944 * (locals.var_fcick_dn4 * p.p115))) / (assign7060_e8949 * assign7060_e8949)), ((((-locals.var_fcick_dn5) * assign7060_e8949) - (assign7060_e8944 * (locals.var_fcick_dn5 * p.p115))) / (assign7060_e8949 * assign7060_e8949)), ((((-locals.var_fcick_dn6) * assign7060_e8949) - (assign7060_e8944 * (locals.var_fcick_dn6 * p.p115))) / (assign7060_e8949 * assign7060_e8949)), ((((-locals.var_fcick_dn7) * assign7060_e8949) - (assign7060_e8944 * (locals.var_fcick_dn7 * p.p115))) / (assign7060_e8949 * assign7060_e8949)), ((((-locals.var_fcick_dn8) * assign7060_e8949) - (assign7060_e8944 * (locals.var_fcick_dn8 * p.p115))) / (assign7060_e8949 * assign7060_e8949)), ((((-locals.var_fcick_dn9) * assign7060_e8949) - (assign7060_e8944 * (locals.var_fcick_dn9 * p.p115))) / (assign7060_e8949 * assign7060_e8949)),)
    } else {
        (locals.var_fcw, locals.var_fcw_dn0, locals.var_fcw_dn1, locals.var_fcw_dn3, locals.var_fcw_dn4, locals.var_fcw_dn5, locals.var_fcw_dn6, locals.var_fcw_dn7, locals.var_fcw_dn8, locals.var_fcw_dn9,)
    }
};
        locals.var_fcw = assign7060_e8952;
        locals.var_fcw_dn0 = assign7060_e8952_d_n0;
        locals.var_fcw_dn1 = assign7060_e8952_d_n1;
        locals.var_fcw_dn3 = assign7060_e8952_d_n3;
        locals.var_fcw_dn4 = assign7060_e8952_d_n4;
        locals.var_fcw_dn5 = assign7060_e8952_d_n5;
        locals.var_fcw_dn6 = assign7060_e8952_d_n6;
        locals.var_fcw_dn7 = assign7060_e8952_d_n7;
        locals.var_fcw_dn8 = assign7060_e8952_d_n8;
        locals.var_fcw_dn9 = assign7060_e8952_d_n9;
        locals.var_fcw_rv = 0.0;

        let (assign7070_e8968, assign7070_e8968_d_n0, assign7070_e8968_d_n1, assign7070_e8968_d_n3, assign7070_e8968_d_n4, assign7070_e8968_d_n5, assign7070_e8968_d_n6, assign7070_e8968_d_n7, assign7070_e8968_d_n8, assign7070_e8968_d_n9,) = {
    if ((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 == 0.0)) {
        let assign7070_e8965: f64 = (p.p115 * locals.var_fcw);
        let assign7070_e8966: f64 = (1.0 + assign7070_e8965);
        (assign7070_e8966, (p.p115 * locals.var_fcw_dn0), (p.p115 * locals.var_fcw_dn1), (p.p115 * locals.var_fcw_dn3), (p.p115 * locals.var_fcw_dn4), (p.p115 * locals.var_fcw_dn5), (p.p115 * locals.var_fcw_dn6), (p.p115 * locals.var_fcw_dn7), (p.p115 * locals.var_fcw_dn8), (p.p115 * locals.var_fcw_dn9),)
    } else {
        (locals.var_fclatbw, locals.var_fclatbw_dn0, locals.var_fclatbw_dn1, locals.var_fclatbw_dn3, locals.var_fclatbw_dn4, locals.var_fclatbw_dn5, locals.var_fclatbw_dn6, locals.var_fclatbw_dn7, locals.var_fclatbw_dn8, locals.var_fclatbw_dn9,)
    }
};
        locals.var_fclatbw = assign7070_e8968;
        locals.var_fclatbw_dn0 = assign7070_e8968_d_n0;
        locals.var_fclatbw_dn1 = assign7070_e8968_d_n1;
        locals.var_fclatbw_dn3 = assign7070_e8968_d_n3;
        locals.var_fclatbw_dn4 = assign7070_e8968_d_n4;
        locals.var_fclatbw_dn5 = assign7070_e8968_d_n5;
        locals.var_fclatbw_dn6 = assign7070_e8968_d_n6;
        locals.var_fclatbw_dn7 = assign7070_e8968_d_n7;
        locals.var_fclatbw_dn8 = assign7070_e8968_d_n8;
        locals.var_fclatbw_dn9 = assign7070_e8968_d_n9;
        locals.var_fclatbw_rv = 0.0;

        let (assign7080_e8992, assign7080_e8992_d_n0, assign7080_e8992_d_n1, assign7080_e8992_d_n3, assign7080_e8992_d_n4, assign7080_e8992_d_n5, assign7080_e8992_d_n6, assign7080_e8992_d_n7, assign7080_e8992_d_n8, assign7080_e8992_d_n9,) = {
    if ((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 == 0.0)) {
        let assign7080_e8980: f64 = (locals.var_fcw * locals.var_fcw);
        let assign7080_e8984: f64 = (locals.var_latb_6 * 2.0);
        let assign7080_e8986: f64 = (assign7080_e8984 * locals.var_fcw);
        let assign7080_e8987: f64 = (1.0 + assign7080_e8986);
        let assign7080_e8988: f64 = (assign7080_e8980 * assign7080_e8987);
        let assign7080_e8990: f64 = (assign7080_e8988 / locals.var_fclatbw);
        (assign7080_e8990, (((((((locals.var_fcw_dn0 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn0)) * assign7080_e8987) + (assign7080_e8980 * (assign7080_e8984 * locals.var_fcw_dn0))) * locals.var_fclatbw) - (assign7080_e8988 * locals.var_fclatbw_dn0)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn1 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn1)) * assign7080_e8987) + (assign7080_e8980 * (assign7080_e8984 * locals.var_fcw_dn1))) * locals.var_fclatbw) - (assign7080_e8988 * locals.var_fclatbw_dn1)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn3 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn3)) * assign7080_e8987) + (assign7080_e8980 * (assign7080_e8984 * locals.var_fcw_dn3))) * locals.var_fclatbw) - (assign7080_e8988 * locals.var_fclatbw_dn3)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn4 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn4)) * assign7080_e8987) + (assign7080_e8980 * (assign7080_e8984 * locals.var_fcw_dn4))) * locals.var_fclatbw) - (assign7080_e8988 * locals.var_fclatbw_dn4)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn5 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn5)) * assign7080_e8987) + (assign7080_e8980 * (assign7080_e8984 * locals.var_fcw_dn5))) * locals.var_fclatbw) - (assign7080_e8988 * locals.var_fclatbw_dn5)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn6 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn6)) * assign7080_e8987) + (assign7080_e8980 * (assign7080_e8984 * locals.var_fcw_dn6))) * locals.var_fclatbw) - (assign7080_e8988 * locals.var_fclatbw_dn6)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn7 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn7)) * assign7080_e8987) + (assign7080_e8980 * (assign7080_e8984 * locals.var_fcw_dn7))) * locals.var_fclatbw) - (assign7080_e8988 * locals.var_fclatbw_dn7)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn8 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn8)) * assign7080_e8987) + (assign7080_e8980 * (assign7080_e8984 * locals.var_fcw_dn8))) * locals.var_fclatbw) - (assign7080_e8988 * locals.var_fclatbw_dn8)) / (locals.var_fclatbw * locals.var_fclatbw)), (((((((locals.var_fcw_dn9 * locals.var_fcw) + (locals.var_fcw * locals.var_fcw_dn9)) * assign7080_e8987) + (assign7080_e8980 * (assign7080_e8984 * locals.var_fcw_dn9))) * locals.var_fclatbw) - (assign7080_e8988 * locals.var_fclatbw_dn9)) / (locals.var_fclatbw * locals.var_fclatbw)),)
    } else {
        (locals.var_fcf_ci, locals.var_fcf_ci_dn0, locals.var_fcf_ci_dn1, locals.var_fcf_ci_dn3, locals.var_fcf_ci_dn4, locals.var_fcf_ci_dn5, locals.var_fcf_ci_dn6, locals.var_fcf_ci_dn7, locals.var_fcf_ci_dn8, locals.var_fcf_ci_dn9,)
    }
};
        locals.var_fcf_ci = assign7080_e8992;
        locals.var_fcf_ci_dn0 = assign7080_e8992_d_n0;
        locals.var_fcf_ci_dn1 = assign7080_e8992_d_n1;
        locals.var_fcf_ci_dn3 = assign7080_e8992_d_n3;
        locals.var_fcf_ci_dn4 = assign7080_e8992_d_n4;
        locals.var_fcf_ci_dn5 = assign7080_e8992_d_n5;
        locals.var_fcf_ci_dn6 = assign7080_e8992_d_n6;
        locals.var_fcf_ci_dn7 = assign7080_e8992_d_n7;
        locals.var_fcf_ci_dn8 = assign7080_e8992_d_n8;
        locals.var_fcf_ci_dn9 = assign7080_e8992_d_n9;
        locals.var_fcf_ci_rv = 0.0;

        let (assign7090_e9013, assign7090_e9013_d_n0, assign7090_e9013_d_n1, assign7090_e9013_d_n3, assign7090_e9013_d_n4, assign7090_e9013_d_n5, assign7090_e9013_d_n6, assign7090_e9013_d_n7, assign7090_e9013_d_n8, assign7090_e9013_d_n9,) = {
    if ((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 == 0.0)) {
        let assign7090_e9003: f64 = (-locals.var_fcdick_ditf);
        let assign7090_e9005: f64 = (assign7090_e9003 * locals.var_fclatbw);
        let assign7090_e9009: f64 = (locals.var_fcick * p.p115);
        let assign7090_e9010: f64 = (1.0 + assign7090_e9009);
        let assign7090_e9011: f64 = (assign7090_e9005 / assign7090_e9010);
        (assign7090_e9011, ((((((-locals.var_fcdick_ditf_dn0) * locals.var_fclatbw) + (assign7090_e9003 * locals.var_fclatbw_dn0)) * assign7090_e9010) - (assign7090_e9005 * (locals.var_fcick_dn0 * p.p115))) / (assign7090_e9010 * assign7090_e9010)), ((((((-locals.var_fcdick_ditf_dn1) * locals.var_fclatbw) + (assign7090_e9003 * locals.var_fclatbw_dn1)) * assign7090_e9010) - (assign7090_e9005 * (locals.var_fcick_dn1 * p.p115))) / (assign7090_e9010 * assign7090_e9010)), ((((((-locals.var_fcdick_ditf_dn3) * locals.var_fclatbw) + (assign7090_e9003 * locals.var_fclatbw_dn3)) * assign7090_e9010) - (assign7090_e9005 * (locals.var_fcick_dn3 * p.p115))) / (assign7090_e9010 * assign7090_e9010)), ((((((-locals.var_fcdick_ditf_dn4) * locals.var_fclatbw) + (assign7090_e9003 * locals.var_fclatbw_dn4)) * assign7090_e9010) - (assign7090_e9005 * (locals.var_fcick_dn4 * p.p115))) / (assign7090_e9010 * assign7090_e9010)), ((((((-locals.var_fcdick_ditf_dn5) * locals.var_fclatbw) + (assign7090_e9003 * locals.var_fclatbw_dn5)) * assign7090_e9010) - (assign7090_e9005 * (locals.var_fcick_dn5 * p.p115))) / (assign7090_e9010 * assign7090_e9010)), ((((((-locals.var_fcdick_ditf_dn6) * locals.var_fclatbw) + (assign7090_e9003 * locals.var_fclatbw_dn6)) * assign7090_e9010) - (assign7090_e9005 * (locals.var_fcick_dn6 * p.p115))) / (assign7090_e9010 * assign7090_e9010)), ((((((-locals.var_fcdick_ditf_dn7) * locals.var_fclatbw) + (assign7090_e9003 * locals.var_fclatbw_dn7)) * assign7090_e9010) - (assign7090_e9005 * (locals.var_fcick_dn7 * p.p115))) / (assign7090_e9010 * assign7090_e9010)), ((((((-locals.var_fcdick_ditf_dn8) * locals.var_fclatbw) + (assign7090_e9003 * locals.var_fclatbw_dn8)) * assign7090_e9010) - (assign7090_e9005 * (locals.var_fcick_dn8 * p.p115))) / (assign7090_e9010 * assign7090_e9010)), ((((((-locals.var_fcdick_ditf_dn9) * locals.var_fclatbw) + (assign7090_e9003 * locals.var_fclatbw_dn9)) * assign7090_e9010) - (assign7090_e9005 * (locals.var_fcick_dn9 * p.p115))) / (assign7090_e9010 * assign7090_e9010)),)
    } else {
        (locals.var_fcdw_ditf, locals.var_fcdw_ditf_dn0, locals.var_fcdw_ditf_dn1, locals.var_fcdw_ditf_dn3, locals.var_fcdw_ditf_dn4, locals.var_fcdw_ditf_dn5, locals.var_fcdw_ditf_dn6, locals.var_fcdw_ditf_dn7, locals.var_fcdw_ditf_dn8, locals.var_fcdw_ditf_dn9,)
    }
};
        locals.var_fcdw_ditf = assign7090_e9013;
        locals.var_fcdw_ditf_dn0 = assign7090_e9013_d_n0;
        locals.var_fcdw_ditf_dn1 = assign7090_e9013_d_n1;
        locals.var_fcdw_ditf_dn3 = assign7090_e9013_d_n3;
        locals.var_fcdw_ditf_dn4 = assign7090_e9013_d_n4;
        locals.var_fcdw_ditf_dn5 = assign7090_e9013_d_n5;
        locals.var_fcdw_ditf_dn6 = assign7090_e9013_d_n6;
        locals.var_fcdw_ditf_dn7 = assign7090_e9013_d_n7;
        locals.var_fcdw_ditf_dn8 = assign7090_e9013_d_n8;
        locals.var_fcdw_ditf_dn9 = assign7090_e9013_d_n9;
        locals.var_fcdw_ditf_rv = 0.0;

        let (assign7100_e9035, assign7100_e9035_d_n0, assign7100_e9035_d_n1, assign7100_e9035_d_n3, assign7100_e9035_d_n4, assign7100_e9035_d_n5, assign7100_e9035_d_n6, assign7100_e9035_d_n7, assign7100_e9035_d_n8, assign7100_e9035_d_n9,) = {
    if ((((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) && (locals.var_guard148 == 0.0)) {
        let assign7100_e9028: f64 = (locals.var_fclatbw * locals.var_fclatbw);
        let assign7100_e9029: f64 = (1.0 / assign7100_e9028);
        let assign7100_e9030: f64 = (1.0 + assign7100_e9029);
        let assign7100_e9031: f64 = (locals.var_fcw * assign7100_e9030);
        let assign7100_e9033: f64 = (assign7100_e9031 * locals.var_fcdw_ditf);
        (assign7100_e9033, ((((locals.var_fcw_dn0 * assign7100_e9030) + (locals.var_fcw * (-(((locals.var_fclatbw_dn0 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn0)) / (assign7100_e9028 * assign7100_e9028))))) * locals.var_fcdw_ditf) + (assign7100_e9031 * locals.var_fcdw_ditf_dn0)), ((((locals.var_fcw_dn1 * assign7100_e9030) + (locals.var_fcw * (-(((locals.var_fclatbw_dn1 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn1)) / (assign7100_e9028 * assign7100_e9028))))) * locals.var_fcdw_ditf) + (assign7100_e9031 * locals.var_fcdw_ditf_dn1)), ((((locals.var_fcw_dn3 * assign7100_e9030) + (locals.var_fcw * (-(((locals.var_fclatbw_dn3 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn3)) / (assign7100_e9028 * assign7100_e9028))))) * locals.var_fcdw_ditf) + (assign7100_e9031 * locals.var_fcdw_ditf_dn3)), ((((locals.var_fcw_dn4 * assign7100_e9030) + (locals.var_fcw * (-(((locals.var_fclatbw_dn4 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn4)) / (assign7100_e9028 * assign7100_e9028))))) * locals.var_fcdw_ditf) + (assign7100_e9031 * locals.var_fcdw_ditf_dn4)), ((((locals.var_fcw_dn5 * assign7100_e9030) + (locals.var_fcw * (-(((locals.var_fclatbw_dn5 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn5)) / (assign7100_e9028 * assign7100_e9028))))) * locals.var_fcdw_ditf) + (assign7100_e9031 * locals.var_fcdw_ditf_dn5)), ((((locals.var_fcw_dn6 * assign7100_e9030) + (locals.var_fcw * (-(((locals.var_fclatbw_dn6 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn6)) / (assign7100_e9028 * assign7100_e9028))))) * locals.var_fcdw_ditf) + (assign7100_e9031 * locals.var_fcdw_ditf_dn6)), ((((locals.var_fcw_dn7 * assign7100_e9030) + (locals.var_fcw * (-(((locals.var_fclatbw_dn7 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn7)) / (assign7100_e9028 * assign7100_e9028))))) * locals.var_fcdw_ditf) + (assign7100_e9031 * locals.var_fcdw_ditf_dn7)), ((((locals.var_fcw_dn8 * assign7100_e9030) + (locals.var_fcw * (-(((locals.var_fclatbw_dn8 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn8)) / (assign7100_e9028 * assign7100_e9028))))) * locals.var_fcdw_ditf) + (assign7100_e9031 * locals.var_fcdw_ditf_dn8)), ((((locals.var_fcw_dn9 * assign7100_e9030) + (locals.var_fcw * (-(((locals.var_fclatbw_dn9 * locals.var_fclatbw) + (locals.var_fclatbw * locals.var_fclatbw_dn9)) / (assign7100_e9028 * assign7100_e9028))))) * locals.var_fcdw_ditf) + (assign7100_e9031 * locals.var_fcdw_ditf_dn9)),)
    } else {
        (locals.var_fcdfc_ditf, locals.var_fcdfc_ditf_dn0, locals.var_fcdfc_ditf_dn1, locals.var_fcdfc_ditf_dn3, locals.var_fcdfc_ditf_dn4, locals.var_fcdfc_ditf_dn5, locals.var_fcdfc_ditf_dn6, locals.var_fcdfc_ditf_dn7, locals.var_fcdfc_ditf_dn8, locals.var_fcdfc_ditf_dn9,)
    }
};
        locals.var_fcdfc_ditf = assign7100_e9035;
        locals.var_fcdfc_ditf_dn0 = assign7100_e9035_d_n0;
        locals.var_fcdfc_ditf_dn1 = assign7100_e9035_d_n1;
        locals.var_fcdfc_ditf_dn3 = assign7100_e9035_d_n3;
        locals.var_fcdfc_ditf_dn4 = assign7100_e9035_d_n4;
        locals.var_fcdfc_ditf_dn5 = assign7100_e9035_d_n5;
        locals.var_fcdfc_ditf_dn6 = assign7100_e9035_d_n6;
        locals.var_fcdfc_ditf_dn7 = assign7100_e9035_d_n7;
        locals.var_fcdfc_ditf_dn8 = assign7100_e9035_d_n8;
        locals.var_fcdfc_ditf_dn9 = assign7100_e9035_d_n9;
        locals.var_fcdfc_ditf_rv = 0.0;

        let (assign7110_e9048, assign7110_e9048_d_n0, assign7110_e9048_d_n1, assign7110_e9048_d_n3, assign7110_e9048_d_n4, assign7110_e9048_d_n5, assign7110_e9048_d_n6, assign7110_e9048_d_n7, assign7110_e9048_d_n8, assign7110_e9048_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) {
        let assign7110_e9044: f64 = (p.p73 * locals.var_thcs_t);
        let assign7110_e9046: f64 = (assign7110_e9044 * locals.var_ffvc_exp);
        (assign7110_e9046, (assign7110_e9044 * locals.var_ffvc_exp_dn0), (assign7110_e9044 * locals.var_ffvc_exp_dn1), (assign7110_e9044 * locals.var_ffvc_exp_dn3), (((p.p73 * locals.var_thcs_t_dn4) * locals.var_ffvc_exp) + (assign7110_e9044 * locals.var_ffvc_exp_dn4)), (assign7110_e9044 * locals.var_ffvc_exp_dn5), (assign7110_e9044 * locals.var_ffvc_exp_dn6), (assign7110_e9044 * locals.var_ffvc_exp_dn7), (assign7110_e9044 * locals.var_ffvc_exp_dn8), (assign7110_e9044 * locals.var_ffvc_exp_dn9),)
    } else {
        (locals.var_dum_a, locals.var_dum_a_dn0, locals.var_dum_a_dn1, locals.var_dum_a_dn3, locals.var_dum_a_dn4, locals.var_dum_a_dn5, locals.var_dum_a_dn6, locals.var_dum_a_dn7, locals.var_dum_a_dn8, locals.var_dum_a_dn9,)
    }
};
        locals.var_dum_a = assign7110_e9048;
        locals.var_dum_a_dn0 = assign7110_e9048_d_n0;
        locals.var_dum_a_dn1 = assign7110_e9048_d_n1;
        locals.var_dum_a_dn3 = assign7110_e9048_d_n3;
        locals.var_dum_a_dn4 = assign7110_e9048_d_n4;
        locals.var_dum_a_dn5 = assign7110_e9048_d_n5;
        locals.var_dum_a_dn6 = assign7110_e9048_d_n6;
        locals.var_dum_a_dn7 = assign7110_e9048_d_n7;
        locals.var_dum_a_dn8 = assign7110_e9048_d_n8;
        locals.var_dum_a_dn9 = assign7110_e9048_d_n9;
        locals.var_dum_a_rv = 0.0;

        let (assign7120_e9059, assign7120_e9059_d_n0, assign7120_e9059_d_n1, assign7120_e9059_d_n3, assign7120_e9059_d_n4, assign7120_e9059_d_n5, assign7120_e9059_d_n6, assign7120_e9059_d_n7, assign7120_e9059_d_n8, assign7120_e9059_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) {
        let assign7120_e9057: f64 = (locals.var_dum_a * locals.var_fcf_ci);
        (assign7120_e9057, ((locals.var_dum_a_dn0 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn0)), ((locals.var_dum_a_dn1 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn1)), ((locals.var_dum_a_dn3 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn3)), ((locals.var_dum_a_dn4 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn4)), ((locals.var_dum_a_dn5 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn5)), ((locals.var_dum_a_dn6 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn6)), ((locals.var_dum_a_dn7 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn7)), ((locals.var_dum_a_dn8 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn8)), ((locals.var_dum_a_dn9 * locals.var_fcf_ci) + (locals.var_dum_a * locals.var_fcf_ci_dn9)),)
    } else {
        (locals.var_dum_b, locals.var_dum_b_dn0, locals.var_dum_b_dn1, locals.var_dum_b_dn3, locals.var_dum_b_dn4, locals.var_dum_b_dn5, locals.var_dum_b_dn6, locals.var_dum_b_dn7, locals.var_dum_b_dn8, locals.var_dum_b_dn9,)
    }
};
        locals.var_dum_b = assign7120_e9059;
        locals.var_dum_b_dn0 = assign7120_e9059_d_n0;
        locals.var_dum_b_dn1 = assign7120_e9059_d_n1;
        locals.var_dum_b_dn3 = assign7120_e9059_d_n3;
        locals.var_dum_b_dn4 = assign7120_e9059_d_n4;
        locals.var_dum_b_dn5 = assign7120_e9059_d_n5;
        locals.var_dum_b_dn6 = assign7120_e9059_d_n6;
        locals.var_dum_b_dn7 = assign7120_e9059_d_n7;
        locals.var_dum_b_dn8 = assign7120_e9059_d_n8;
        locals.var_dum_b_dn9 = assign7120_e9059_d_n9;
        locals.var_dum_b_rv = 0.0;

        let (assign7130_e9070, assign7130_e9070_d_n0, assign7130_e9070_d_n1, assign7130_e9070_d_n3, assign7130_e9070_d_n4, assign7130_e9070_d_n5, assign7130_e9070_d_n6, assign7130_e9070_d_n7, assign7130_e9070_d_n8, assign7130_e9070_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) {
        let assign7130_e9068: f64 = (locals.var_dum_b * locals.var_itf);
        (assign7130_e9068, ((locals.var_dum_b_dn0 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn0)), ((locals.var_dum_b_dn1 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn1)), ((locals.var_dum_b_dn3 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn3)), ((locals.var_dum_b_dn4 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn4)), ((locals.var_dum_b_dn5 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn5)), ((locals.var_dum_b_dn6 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn6)), ((locals.var_dum_b_dn7 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn7)), ((locals.var_dum_b_dn8 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn8)), ((locals.var_dum_b_dn9 * locals.var_itf) + (locals.var_dum_b * locals.var_itf_dn9)),)
    } else {
        (locals.var_ffdqcfc, locals.var_ffdqcfc_dn0, locals.var_ffdqcfc_dn1, locals.var_ffdqcfc_dn3, locals.var_ffdqcfc_dn4, locals.var_ffdqcfc_dn5, locals.var_ffdqcfc_dn6, locals.var_ffdqcfc_dn7, locals.var_ffdqcfc_dn8, locals.var_ffdqcfc_dn9,)
    }
};
        locals.var_ffdqcfc = assign7130_e9070;
        locals.var_ffdqcfc_dn0 = assign7130_e9070_d_n0;
        locals.var_ffdqcfc_dn1 = assign7130_e9070_d_n1;
        locals.var_ffdqcfc_dn3 = assign7130_e9070_d_n3;
        locals.var_ffdqcfc_dn4 = assign7130_e9070_d_n4;
        locals.var_ffdqcfc_dn5 = assign7130_e9070_d_n5;
        locals.var_ffdqcfc_dn6 = assign7130_e9070_d_n6;
        locals.var_ffdqcfc_dn7 = assign7130_e9070_d_n7;
        locals.var_ffdqcfc_dn8 = assign7130_e9070_d_n8;
        locals.var_ffdqcfc_dn9 = assign7130_e9070_d_n9;
        locals.var_ffdqcfc_rv = 0.0;

        let (assign7140_e9091, assign7140_e9091_d_n0, assign7140_e9091_d_n1, assign7140_e9091_d_n3, assign7140_e9091_d_n4, assign7140_e9091_d_n5, assign7140_e9091_d_n6, assign7140_e9091_d_n7, assign7140_e9091_d_n8, assign7140_e9091_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard147 == 0.0)) {
        let assign7140_e9080: f64 = (locals.var_ffdqcfc * locals.var_ffdvc_ditf);
        let assign7140_e9082: f64 = (assign7140_e9080 * locals.var_ovt);
        let assign7140_e9083: f64 = (locals.var_dum_b + assign7140_e9082);
        let assign7140_e9086: f64 = (locals.var_dum_a * locals.var_itf);
        let assign7140_e9088: f64 = (assign7140_e9086 * locals.var_fcdfc_ditf);
        let assign7140_e9089: f64 = (assign7140_e9083 + assign7140_e9088);
        (assign7140_e9089, ((locals.var_dum_b_dn0 + (((locals.var_ffdqcfc_dn0 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn0)) * locals.var_ovt)) + ((((locals.var_dum_a_dn0 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn0)) * locals.var_fcdfc_ditf) + (assign7140_e9086 * locals.var_fcdfc_ditf_dn0))), ((locals.var_dum_b_dn1 + (((locals.var_ffdqcfc_dn1 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn1)) * locals.var_ovt)) + ((((locals.var_dum_a_dn1 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn1)) * locals.var_fcdfc_ditf) + (assign7140_e9086 * locals.var_fcdfc_ditf_dn1))), ((locals.var_dum_b_dn3 + (((locals.var_ffdqcfc_dn3 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn3)) * locals.var_ovt)) + ((((locals.var_dum_a_dn3 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn3)) * locals.var_fcdfc_ditf) + (assign7140_e9086 * locals.var_fcdfc_ditf_dn3))), ((locals.var_dum_b_dn4 + ((((locals.var_ffdqcfc_dn4 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn4)) * locals.var_ovt) + (assign7140_e9080 * locals.var_ovt_dn4))) + ((((locals.var_dum_a_dn4 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn4)) * locals.var_fcdfc_ditf) + (assign7140_e9086 * locals.var_fcdfc_ditf_dn4))), ((locals.var_dum_b_dn5 + (((locals.var_ffdqcfc_dn5 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn5)) * locals.var_ovt)) + ((((locals.var_dum_a_dn5 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn5)) * locals.var_fcdfc_ditf) + (assign7140_e9086 * locals.var_fcdfc_ditf_dn5))), ((locals.var_dum_b_dn6 + (((locals.var_ffdqcfc_dn6 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn6)) * locals.var_ovt)) + ((((locals.var_dum_a_dn6 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn6)) * locals.var_fcdfc_ditf) + (assign7140_e9086 * locals.var_fcdfc_ditf_dn6))), ((locals.var_dum_b_dn7 + (((locals.var_ffdqcfc_dn7 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn7)) * locals.var_ovt)) + ((((locals.var_dum_a_dn7 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn7)) * locals.var_fcdfc_ditf) + (assign7140_e9086 * locals.var_fcdfc_ditf_dn7))), ((locals.var_dum_b_dn8 + (((locals.var_ffdqcfc_dn8 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn8)) * locals.var_ovt)) + ((((locals.var_dum_a_dn8 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn8)) * locals.var_fcdfc_ditf) + (assign7140_e9086 * locals.var_fcdfc_ditf_dn8))), ((locals.var_dum_b_dn9 + (((locals.var_ffdqcfc_dn9 * locals.var_ffdvc_ditf) + (locals.var_ffdqcfc * locals.var_ffdvc_ditf_dn9)) * locals.var_ovt)) + ((((locals.var_dum_a_dn9 * locals.var_itf) + (locals.var_dum_a * locals.var_itf_dn9)) * locals.var_fcdfc_ditf) + (assign7140_e9086 * locals.var_fcdfc_ditf_dn9))),)
    } else {
        (locals.var_ffdtcfc, locals.var_ffdtcfc_dn0, locals.var_ffdtcfc_dn1, locals.var_ffdtcfc_dn3, locals.var_ffdtcfc_dn4, locals.var_ffdtcfc_dn5, locals.var_ffdtcfc_dn6, locals.var_ffdtcfc_dn7, locals.var_ffdtcfc_dn8, locals.var_ffdtcfc_dn9,)
    }
};
        locals.var_ffdtcfc = assign7140_e9091;
        locals.var_ffdtcfc_dn0 = assign7140_e9091_d_n0;
        locals.var_ffdtcfc_dn1 = assign7140_e9091_d_n1;
        locals.var_ffdtcfc_dn3 = assign7140_e9091_d_n3;
        locals.var_ffdtcfc_dn4 = assign7140_e9091_d_n4;
        locals.var_ffdtcfc_dn5 = assign7140_e9091_d_n5;
        locals.var_ffdtcfc_dn6 = assign7140_e9091_d_n6;
        locals.var_ffdtcfc_dn7 = assign7140_e9091_d_n7;
        locals.var_ffdtcfc_dn8 = assign7140_e9091_d_n8;
        locals.var_ffdtcfc_dn9 = assign7140_e9091_d_n9;
        locals.var_ffdtcfc_rv = 0.0;

        let (assign7150_e9103, assign7150_e9103_d_n0, assign7150_e9103_d_n1, assign7150_e9103_d_n3, assign7150_e9103_d_n4, assign7150_e9103_d_n5, assign7150_e9103_d_n6, assign7150_e9103_d_n7, assign7150_e9103_d_n8, assign7150_e9103_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign7150_e9097: f64 = (1.0 - p.p73);
        let assign7150_e9099: f64 = (assign7150_e9097 * locals.var_ffdqfhc);
        let assign7150_e9101: f64 = (assign7150_e9099 * locals.var_itf);
        (assign7150_e9101, (((assign7150_e9097 * locals.var_ffdqfhc_dn0) * locals.var_itf) + (assign7150_e9099 * locals.var_itf_dn0)), (((assign7150_e9097 * locals.var_ffdqfhc_dn1) * locals.var_itf) + (assign7150_e9099 * locals.var_itf_dn1)), (((assign7150_e9097 * locals.var_ffdqfhc_dn3) * locals.var_itf) + (assign7150_e9099 * locals.var_itf_dn3)), (((assign7150_e9097 * locals.var_ffdqfhc_dn4) * locals.var_itf) + (assign7150_e9099 * locals.var_itf_dn4)), (((assign7150_e9097 * locals.var_ffdqfhc_dn5) * locals.var_itf) + (assign7150_e9099 * locals.var_itf_dn5)), (((assign7150_e9097 * locals.var_ffdqfhc_dn6) * locals.var_itf) + (assign7150_e9099 * locals.var_itf_dn6)), (((assign7150_e9097 * locals.var_ffdqfhc_dn7) * locals.var_itf) + (assign7150_e9099 * locals.var_itf_dn7)), (((assign7150_e9097 * locals.var_ffdqfhc_dn8) * locals.var_itf) + (assign7150_e9099 * locals.var_itf_dn8)), (((assign7150_e9097 * locals.var_ffdqfhc_dn9) * locals.var_itf) + (assign7150_e9099 * locals.var_itf_dn9)),)
    } else {
        (locals.var_ffdqbfc, locals.var_ffdqbfc_dn0, locals.var_ffdqbfc_dn1, locals.var_ffdqbfc_dn3, locals.var_ffdqbfc_dn4, locals.var_ffdqbfc_dn5, locals.var_ffdqbfc_dn6, locals.var_ffdqbfc_dn7, locals.var_ffdqbfc_dn8, locals.var_ffdqbfc_dn9,)
    }
};
        locals.var_ffdqbfc = assign7150_e9103;
        locals.var_ffdqbfc_dn0 = assign7150_e9103_d_n0;
        locals.var_ffdqbfc_dn1 = assign7150_e9103_d_n1;
        locals.var_ffdqbfc_dn3 = assign7150_e9103_d_n3;
        locals.var_ffdqbfc_dn4 = assign7150_e9103_d_n4;
        locals.var_ffdqbfc_dn5 = assign7150_e9103_d_n5;
        locals.var_ffdqbfc_dn6 = assign7150_e9103_d_n6;
        locals.var_ffdqbfc_dn7 = assign7150_e9103_d_n7;
        locals.var_ffdqbfc_dn8 = assign7150_e9103_d_n8;
        locals.var_ffdqbfc_dn9 = assign7150_e9103_d_n9;
        locals.var_ffdqbfc_rv = 0.0;

        let (assign7160_e9113, assign7160_e9113_d_n0, assign7160_e9113_d_n1, assign7160_e9113_d_n3, assign7160_e9113_d_n4, assign7160_e9113_d_n5, assign7160_e9113_d_n6, assign7160_e9113_d_n7, assign7160_e9113_d_n8, assign7160_e9113_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign7160_e9109: f64 = (1.0 - p.p73);
        let assign7160_e9111: f64 = (assign7160_e9109 * locals.var_ffdtfhc);
        (assign7160_e9111, (assign7160_e9109 * locals.var_ffdtfhc_dn0), (assign7160_e9109 * locals.var_ffdtfhc_dn1), (assign7160_e9109 * locals.var_ffdtfhc_dn3), (assign7160_e9109 * locals.var_ffdtfhc_dn4), (assign7160_e9109 * locals.var_ffdtfhc_dn5), (assign7160_e9109 * locals.var_ffdtfhc_dn6), (assign7160_e9109 * locals.var_ffdtfhc_dn7), (assign7160_e9109 * locals.var_ffdtfhc_dn8), (assign7160_e9109 * locals.var_ffdtfhc_dn9),)
    } else {
        (locals.var_ffdtbfc, locals.var_ffdtbfc_dn0, locals.var_ffdtbfc_dn1, locals.var_ffdtbfc_dn3, locals.var_ffdtbfc_dn4, locals.var_ffdtbfc_dn5, locals.var_ffdtbfc_dn6, locals.var_ffdtbfc_dn7, locals.var_ffdtbfc_dn8, locals.var_ffdtbfc_dn9,)
    }
};
        locals.var_ffdtbfc = assign7160_e9113;
        locals.var_ffdtbfc_dn0 = assign7160_e9113_d_n0;
        locals.var_ffdtbfc_dn1 = assign7160_e9113_d_n1;
        locals.var_ffdtbfc_dn3 = assign7160_e9113_d_n3;
        locals.var_ffdtbfc_dn4 = assign7160_e9113_d_n4;
        locals.var_ffdtbfc_dn5 = assign7160_e9113_d_n5;
        locals.var_ffdtbfc_dn6 = assign7160_e9113_d_n6;
        locals.var_ffdtbfc_dn7 = assign7160_e9113_d_n7;
        locals.var_ffdtbfc_dn8 = assign7160_e9113_d_n8;
        locals.var_ffdtbfc_dn9 = assign7160_e9113_d_n9;
        locals.var_ffdtbfc_rv = 0.0;

        let (assign7170_e9123, assign7170_e9123_d_n0, assign7170_e9123_d_n1, assign7170_e9123_d_n3, assign7170_e9123_d_n4, assign7170_e9123_d_n5, assign7170_e9123_d_n6, assign7170_e9123_d_n7, assign7170_e9123_d_n8, assign7170_e9123_d_n9,) = {
    if ((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign7170_e9119: f64 = (locals.var_ffdqbfb * locals.var_itf);
        let assign7170_e9121: f64 = (assign7170_e9119 + locals.var_ffdqbfc);
        (assign7170_e9121, (((locals.var_ffdqbfb_dn0 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn0)) + locals.var_ffdqbfc_dn0), (((locals.var_ffdqbfb_dn1 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn1)) + locals.var_ffdqbfc_dn1), (((locals.var_ffdqbfb_dn3 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn3)) + locals.var_ffdqbfc_dn3), (((locals.var_ffdqbfb_dn4 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn4)) + locals.var_ffdqbfc_dn4), (((locals.var_ffdqbfb_dn5 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn5)) + locals.var_ffdqbfc_dn5), (((locals.var_ffdqbfb_dn6 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn6)) + locals.var_ffdqbfc_dn6), (((locals.var_ffdqbfb_dn7 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn7)) + locals.var_ffdqbfc_dn7), (((locals.var_ffdqbfb_dn8 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn8)) + locals.var_ffdqbfc_dn8), (((locals.var_ffdqbfb_dn9 * locals.var_itf) + (locals.var_ffdqbfb * locals.var_itf_dn9)) + locals.var_ffdqbfc_dn9),)
    } else {
        (locals.var_q_bf, locals.var_q_bf_dn0, locals.var_q_bf_dn1, locals.var_q_bf_dn3, locals.var_q_bf_dn4, locals.var_q_bf_dn5, locals.var_q_bf_dn6, locals.var_q_bf_dn7, locals.var_q_bf_dn8, locals.var_q_bf_dn9,)
    }
};
        locals.var_q_bf = assign7170_e9123;
        locals.var_q_bf_dn0 = assign7170_e9123_d_n0;
        locals.var_q_bf_dn1 = assign7170_e9123_d_n1;
        locals.var_q_bf_dn3 = assign7170_e9123_d_n3;
        locals.var_q_bf_dn4 = assign7170_e9123_d_n4;
        locals.var_q_bf_dn5 = assign7170_e9123_d_n5;
        locals.var_q_bf_dn6 = assign7170_e9123_d_n6;
        locals.var_q_bf_dn7 = assign7170_e9123_d_n7;
        locals.var_q_bf_dn8 = assign7170_e9123_d_n8;
        locals.var_q_bf_dn9 = assign7170_e9123_d_n9;
        locals.var_q_bf_rv = 0.0;

        let assign7180_e9126: f64 = if p.p0 >= 310.0 { 1.0 } else { 0.0 };
        locals.var_guard150 = assign7180_e9126;
        locals.var_guard150_rv = 0.0;

        let (assign7190_e9140, assign7190_e9140_d_n0, assign7190_e9140_d_n1, assign7190_e9140_d_n3, assign7190_e9140_d_n4, assign7190_e9140_d_n5, assign7190_e9140_d_n6, assign7190_e9140_d_n7, assign7190_e9140_d_n8, assign7190_e9140_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard150 != 0.0)) {
        let assign7190_e9134: f64 = (locals.var_qf + locals.var_q_bf);
        let assign7190_e9136: f64 = (assign7190_e9134 + locals.var_ffdqef);
        let assign7190_e9138: f64 = (assign7190_e9136 + locals.var_ffdqcfc);
        (assign7190_e9138, (((locals.var_qf_dn0 + locals.var_q_bf_dn0) + locals.var_ffdqef_dn0) + locals.var_ffdqcfc_dn0), (((locals.var_qf_dn1 + locals.var_q_bf_dn1) + locals.var_ffdqef_dn1) + locals.var_ffdqcfc_dn1), (((locals.var_qf_dn3 + locals.var_q_bf_dn3) + locals.var_ffdqef_dn3) + locals.var_ffdqcfc_dn3), (((locals.var_qf_dn4 + locals.var_q_bf_dn4) + locals.var_ffdqef_dn4) + locals.var_ffdqcfc_dn4), (((locals.var_qf_dn5 + locals.var_q_bf_dn5) + locals.var_ffdqef_dn5) + locals.var_ffdqcfc_dn5), (((locals.var_qf_dn6 + locals.var_q_bf_dn6) + locals.var_ffdqef_dn6) + locals.var_ffdqcfc_dn6), (((locals.var_qf_dn7 + locals.var_q_bf_dn7) + locals.var_ffdqef_dn7) + locals.var_ffdqcfc_dn7), (((locals.var_qf_dn8 + locals.var_q_bf_dn8) + locals.var_ffdqef_dn8) + locals.var_ffdqcfc_dn8), (((locals.var_qf_dn9 + locals.var_q_bf_dn9) + locals.var_ffdqef_dn9) + locals.var_ffdqcfc_dn9),)
    } else {
        (locals.var_qf, locals.var_qf_dn0, locals.var_qf_dn1, locals.var_qf_dn3, locals.var_qf_dn4, locals.var_qf_dn5, locals.var_qf_dn6, locals.var_qf_dn7, locals.var_qf_dn8, locals.var_qf_dn9,)
    }
};
        locals.var_qf = assign7190_e9140;
        locals.var_qf_dn0 = assign7190_e9140_d_n0;
        locals.var_qf_dn1 = assign7190_e9140_d_n1;
        locals.var_qf_dn3 = assign7190_e9140_d_n3;
        locals.var_qf_dn4 = assign7190_e9140_d_n4;
        locals.var_qf_dn5 = assign7190_e9140_d_n5;
        locals.var_qf_dn6 = assign7190_e9140_d_n6;
        locals.var_qf_dn7 = assign7190_e9140_d_n7;
        locals.var_qf_dn8 = assign7190_e9140_d_n8;
        locals.var_qf_dn9 = assign7190_e9140_d_n9;
        locals.var_qf_rv = 0.0;

        let (assign7200_e9156, assign7200_e9156_d_n0, assign7200_e9156_d_n1, assign7200_e9156_d_n3, assign7200_e9156_d_n4, assign7200_e9156_d_n5, assign7200_e9156_d_n6, assign7200_e9156_d_n7, assign7200_e9156_d_n8, assign7200_e9156_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard150 != 0.0)) {
        let assign7200_e9149: f64 = (locals.var_ffdtbfb + locals.var_ffdtbfc);
        let assign7200_e9150: f64 = (locals.var_tf + assign7200_e9149);
        let assign7200_e9152: f64 = (assign7200_e9150 + locals.var_ffdtef);
        let assign7200_e9154: f64 = (assign7200_e9152 + locals.var_ffdtcfc);
        (assign7200_e9154, (((locals.var_tf_dn0 + (locals.var_ffdtbfb_dn0 + locals.var_ffdtbfc_dn0)) + locals.var_ffdtef_dn0) + locals.var_ffdtcfc_dn0), (((locals.var_tf_dn1 + (locals.var_ffdtbfb_dn1 + locals.var_ffdtbfc_dn1)) + locals.var_ffdtef_dn1) + locals.var_ffdtcfc_dn1), (((locals.var_tf_dn3 + (locals.var_ffdtbfb_dn3 + locals.var_ffdtbfc_dn3)) + locals.var_ffdtef_dn3) + locals.var_ffdtcfc_dn3), (((locals.var_tf_dn4 + (locals.var_ffdtbfb_dn4 + locals.var_ffdtbfc_dn4)) + locals.var_ffdtef_dn4) + locals.var_ffdtcfc_dn4), (((locals.var_tf_dn5 + (locals.var_ffdtbfb_dn5 + locals.var_ffdtbfc_dn5)) + locals.var_ffdtef_dn5) + locals.var_ffdtcfc_dn5), (((locals.var_tf_dn6 + (locals.var_ffdtbfb_dn6 + locals.var_ffdtbfc_dn6)) + locals.var_ffdtef_dn6) + locals.var_ffdtcfc_dn6), (((locals.var_tf_dn7 + (locals.var_ffdtbfb_dn7 + locals.var_ffdtbfc_dn7)) + locals.var_ffdtef_dn7) + locals.var_ffdtcfc_dn7), (((locals.var_tf_dn8 + (locals.var_ffdtbfb_dn8 + locals.var_ffdtbfc_dn8)) + locals.var_ffdtef_dn8) + locals.var_ffdtcfc_dn8), (((locals.var_tf_dn9 + (locals.var_ffdtbfb_dn9 + locals.var_ffdtbfc_dn9)) + locals.var_ffdtef_dn9) + locals.var_ffdtcfc_dn9),)
    } else {
        (locals.var_tf, locals.var_tf_dn0, locals.var_tf_dn1, locals.var_tf_dn3, locals.var_tf_dn4, locals.var_tf_dn5, locals.var_tf_dn6, locals.var_tf_dn7, locals.var_tf_dn8, locals.var_tf_dn9,)
    }
};
        locals.var_tf = assign7200_e9156;
        locals.var_tf_dn0 = assign7200_e9156_d_n0;
        locals.var_tf_dn1 = assign7200_e9156_d_n1;
        locals.var_tf_dn3 = assign7200_e9156_d_n3;
        locals.var_tf_dn4 = assign7200_e9156_d_n4;
        locals.var_tf_dn5 = assign7200_e9156_d_n5;
        locals.var_tf_dn6 = assign7200_e9156_d_n6;
        locals.var_tf_dn7 = assign7200_e9156_d_n7;
        locals.var_tf_dn8 = assign7200_e9156_d_n8;
        locals.var_tf_dn9 = assign7200_e9156_d_n9;
        locals.var_tf_rv = 0.0;

        let (assign7210_e9176, assign7210_e9176_d_n0, assign7210_e9176_d_n1, assign7210_e9176_d_n3, assign7210_e9176_d_n4, assign7210_e9176_d_n5, assign7210_e9176_d_n6, assign7210_e9176_d_n7, assign7210_e9176_d_n8, assign7210_e9176_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard150 != 0.0)) {
        let assign7210_e9165: f64 = (p.p5 * locals.var_q_bf);
        let assign7210_e9166: f64 = (locals.var_q_ft + assign7210_e9165);
        let assign7210_e9169: f64 = (locals.var_hfe_t * locals.var_ffdqef);
        let assign7210_e9170: f64 = (assign7210_e9166 + assign7210_e9169);
        let assign7210_e9173: f64 = (locals.var_hfc_t * locals.var_ffdqcfc);
        let assign7210_e9174: f64 = (assign7210_e9170 + assign7210_e9173);
        (assign7210_e9174, (((locals.var_q_ft_dn0 + (p.p5 * locals.var_q_bf_dn0)) + (locals.var_hfe_t * locals.var_ffdqef_dn0)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn0)), (((locals.var_q_ft_dn1 + (p.p5 * locals.var_q_bf_dn1)) + (locals.var_hfe_t * locals.var_ffdqef_dn1)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn1)), (((locals.var_q_ft_dn3 + (p.p5 * locals.var_q_bf_dn3)) + (locals.var_hfe_t * locals.var_ffdqef_dn3)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn3)), (((locals.var_q_ft_dn4 + (p.p5 * locals.var_q_bf_dn4)) + ((locals.var_hfe_t_dn4 * locals.var_ffdqef) + (locals.var_hfe_t * locals.var_ffdqef_dn4))) + ((locals.var_hfc_t_dn4 * locals.var_ffdqcfc) + (locals.var_hfc_t * locals.var_ffdqcfc_dn4))), (((locals.var_q_ft_dn5 + (p.p5 * locals.var_q_bf_dn5)) + (locals.var_hfe_t * locals.var_ffdqef_dn5)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn5)), (((locals.var_q_ft_dn6 + (p.p5 * locals.var_q_bf_dn6)) + (locals.var_hfe_t * locals.var_ffdqef_dn6)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn6)), (((locals.var_q_ft_dn7 + (p.p5 * locals.var_q_bf_dn7)) + (locals.var_hfe_t * locals.var_ffdqef_dn7)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn7)), (((locals.var_q_ft_dn8 + (p.p5 * locals.var_q_bf_dn8)) + (locals.var_hfe_t * locals.var_ffdqef_dn8)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn8)), (((locals.var_q_ft_dn9 + (p.p5 * locals.var_q_bf_dn9)) + (locals.var_hfe_t * locals.var_ffdqef_dn9)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn9)),)
    } else {
        (locals.var_q_ft, locals.var_q_ft_dn0, locals.var_q_ft_dn1, locals.var_q_ft_dn3, locals.var_q_ft_dn4, locals.var_q_ft_dn5, locals.var_q_ft_dn6, locals.var_q_ft_dn7, locals.var_q_ft_dn8, locals.var_q_ft_dn9,)
    }
};
        locals.var_q_ft = assign7210_e9176;
        locals.var_q_ft_dn0 = assign7210_e9176_d_n0;
        locals.var_q_ft_dn1 = assign7210_e9176_d_n1;
        locals.var_q_ft_dn3 = assign7210_e9176_d_n3;
        locals.var_q_ft_dn4 = assign7210_e9176_d_n4;
        locals.var_q_ft_dn5 = assign7210_e9176_d_n5;
        locals.var_q_ft_dn6 = assign7210_e9176_d_n6;
        locals.var_q_ft_dn7 = assign7210_e9176_d_n7;
        locals.var_q_ft_dn8 = assign7210_e9176_d_n8;
        locals.var_q_ft_dn9 = assign7210_e9176_d_n9;
        locals.var_q_ft_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_20(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (assign7220_e9198, assign7220_e9198_d_n0, assign7220_e9198_d_n1, assign7220_e9198_d_n3, assign7220_e9198_d_n4, assign7220_e9198_d_n5, assign7220_e9198_d_n6, assign7220_e9198_d_n7, assign7220_e9198_d_n8, assign7220_e9198_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard150 != 0.0)) {
        let assign7220_e9186: f64 = (locals.var_ffdtbfb + locals.var_ffdtbfc);
        let assign7220_e9187: f64 = (p.p5 * assign7220_e9186);
        let assign7220_e9188: f64 = (locals.var_t_ft + assign7220_e9187);
        let assign7220_e9191: f64 = (locals.var_hfe_t * locals.var_ffdtef);
        let assign7220_e9192: f64 = (assign7220_e9188 + assign7220_e9191);
        let assign7220_e9195: f64 = (locals.var_hfc_t * locals.var_ffdtcfc);
        let assign7220_e9196: f64 = (assign7220_e9192 + assign7220_e9195);
        (assign7220_e9196, (((locals.var_t_ft_dn0 + (p.p5 * (locals.var_ffdtbfb_dn0 + locals.var_ffdtbfc_dn0))) + (locals.var_hfe_t * locals.var_ffdtef_dn0)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn0)), (((locals.var_t_ft_dn1 + (p.p5 * (locals.var_ffdtbfb_dn1 + locals.var_ffdtbfc_dn1))) + (locals.var_hfe_t * locals.var_ffdtef_dn1)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn1)), (((locals.var_t_ft_dn3 + (p.p5 * (locals.var_ffdtbfb_dn3 + locals.var_ffdtbfc_dn3))) + (locals.var_hfe_t * locals.var_ffdtef_dn3)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn3)), (((locals.var_t_ft_dn4 + (p.p5 * (locals.var_ffdtbfb_dn4 + locals.var_ffdtbfc_dn4))) + ((locals.var_hfe_t_dn4 * locals.var_ffdtef) + (locals.var_hfe_t * locals.var_ffdtef_dn4))) + ((locals.var_hfc_t_dn4 * locals.var_ffdtcfc) + (locals.var_hfc_t * locals.var_ffdtcfc_dn4))), (((locals.var_t_ft_dn5 + (p.p5 * (locals.var_ffdtbfb_dn5 + locals.var_ffdtbfc_dn5))) + (locals.var_hfe_t * locals.var_ffdtef_dn5)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn5)), (((locals.var_t_ft_dn6 + (p.p5 * (locals.var_ffdtbfb_dn6 + locals.var_ffdtbfc_dn6))) + (locals.var_hfe_t * locals.var_ffdtef_dn6)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn6)), (((locals.var_t_ft_dn7 + (p.p5 * (locals.var_ffdtbfb_dn7 + locals.var_ffdtbfc_dn7))) + (locals.var_hfe_t * locals.var_ffdtef_dn7)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn7)), (((locals.var_t_ft_dn8 + (p.p5 * (locals.var_ffdtbfb_dn8 + locals.var_ffdtbfc_dn8))) + (locals.var_hfe_t * locals.var_ffdtef_dn8)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn8)), (((locals.var_t_ft_dn9 + (p.p5 * (locals.var_ffdtbfb_dn9 + locals.var_ffdtbfc_dn9))) + (locals.var_hfe_t * locals.var_ffdtef_dn9)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn9)),)
    } else {
        (locals.var_t_ft, locals.var_t_ft_dn0, locals.var_t_ft_dn1, locals.var_t_ft_dn3, locals.var_t_ft_dn4, locals.var_t_ft_dn5, locals.var_t_ft_dn6, locals.var_t_ft_dn7, locals.var_t_ft_dn8, locals.var_t_ft_dn9,)
    }
};
        locals.var_t_ft = assign7220_e9198;
        locals.var_t_ft_dn0 = assign7220_e9198_d_n0;
        locals.var_t_ft_dn1 = assign7220_e9198_d_n1;
        locals.var_t_ft_dn3 = assign7220_e9198_d_n3;
        locals.var_t_ft_dn4 = assign7220_e9198_d_n4;
        locals.var_t_ft_dn5 = assign7220_e9198_d_n5;
        locals.var_t_ft_dn6 = assign7220_e9198_d_n6;
        locals.var_t_ft_dn7 = assign7220_e9198_d_n7;
        locals.var_t_ft_dn8 = assign7220_e9198_d_n8;
        locals.var_t_ft_dn9 = assign7220_e9198_d_n9;
        locals.var_t_ft_rv = 0.0;

        let (assign7230_e9219, assign7230_e9219_d_n0, assign7230_e9219_d_n1, assign7230_e9219_d_n3, assign7230_e9219_d_n4, assign7230_e9219_d_n5, assign7230_e9219_d_n6, assign7230_e9219_d_n7, assign7230_e9219_d_n8, assign7230_e9219_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard150 == 0.0)) {
        let assign7230_e9207: f64 = (locals.var_hf0_t * locals.var_qf);
        let assign7230_e9209: f64 = (assign7230_e9207 + locals.var_q_bf);
        let assign7230_e9212: f64 = (locals.var_hfe_t * locals.var_ffdqef);
        let assign7230_e9213: f64 = (assign7230_e9209 + assign7230_e9212);
        let assign7230_e9216: f64 = (locals.var_hfc_t * locals.var_ffdqcfc);
        let assign7230_e9217: f64 = (assign7230_e9213 + assign7230_e9216);
        (assign7230_e9217, ((((locals.var_hf0_t * locals.var_qf_dn0) + locals.var_q_bf_dn0) + (locals.var_hfe_t * locals.var_ffdqef_dn0)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn0)), ((((locals.var_hf0_t * locals.var_qf_dn1) + locals.var_q_bf_dn1) + (locals.var_hfe_t * locals.var_ffdqef_dn1)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn1)), ((((locals.var_hf0_t * locals.var_qf_dn3) + locals.var_q_bf_dn3) + (locals.var_hfe_t * locals.var_ffdqef_dn3)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn3)), (((((locals.var_hf0_t_dn4 * locals.var_qf) + (locals.var_hf0_t * locals.var_qf_dn4)) + locals.var_q_bf_dn4) + ((locals.var_hfe_t_dn4 * locals.var_ffdqef) + (locals.var_hfe_t * locals.var_ffdqef_dn4))) + ((locals.var_hfc_t_dn4 * locals.var_ffdqcfc) + (locals.var_hfc_t * locals.var_ffdqcfc_dn4))), ((((locals.var_hf0_t * locals.var_qf_dn5) + locals.var_q_bf_dn5) + (locals.var_hfe_t * locals.var_ffdqef_dn5)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn5)), ((((locals.var_hf0_t * locals.var_qf_dn6) + locals.var_q_bf_dn6) + (locals.var_hfe_t * locals.var_ffdqef_dn6)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn6)), ((((locals.var_hf0_t * locals.var_qf_dn7) + locals.var_q_bf_dn7) + (locals.var_hfe_t * locals.var_ffdqef_dn7)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn7)), ((((locals.var_hf0_t * locals.var_qf_dn8) + locals.var_q_bf_dn8) + (locals.var_hfe_t * locals.var_ffdqef_dn8)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn8)), ((((locals.var_hf0_t * locals.var_qf_dn9) + locals.var_q_bf_dn9) + (locals.var_hfe_t * locals.var_ffdqef_dn9)) + (locals.var_hfc_t * locals.var_ffdqcfc_dn9)),)
    } else {
        (locals.var_q_ft, locals.var_q_ft_dn0, locals.var_q_ft_dn1, locals.var_q_ft_dn3, locals.var_q_ft_dn4, locals.var_q_ft_dn5, locals.var_q_ft_dn6, locals.var_q_ft_dn7, locals.var_q_ft_dn8, locals.var_q_ft_dn9,)
    }
};
        locals.var_q_ft = assign7230_e9219;
        locals.var_q_ft_dn0 = assign7230_e9219_d_n0;
        locals.var_q_ft_dn1 = assign7230_e9219_d_n1;
        locals.var_q_ft_dn3 = assign7230_e9219_d_n3;
        locals.var_q_ft_dn4 = assign7230_e9219_d_n4;
        locals.var_q_ft_dn5 = assign7230_e9219_d_n5;
        locals.var_q_ft_dn6 = assign7230_e9219_d_n6;
        locals.var_q_ft_dn7 = assign7230_e9219_d_n7;
        locals.var_q_ft_dn8 = assign7230_e9219_d_n8;
        locals.var_q_ft_dn9 = assign7230_e9219_d_n9;
        locals.var_q_ft_rv = 0.0;

        let (assign7240_e9234, assign7240_e9234_d_n0, assign7240_e9234_d_n1, assign7240_e9234_d_n3, assign7240_e9234_d_n4, assign7240_e9234_d_n5, assign7240_e9234_d_n6, assign7240_e9234_d_n7, assign7240_e9234_d_n8, assign7240_e9234_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard150 == 0.0)) {
        let assign7240_e9228: f64 = (locals.var_qf + locals.var_q_bf);
        let assign7240_e9230: f64 = (assign7240_e9228 + locals.var_ffdqef);
        let assign7240_e9232: f64 = (assign7240_e9230 + locals.var_ffdqcfc);
        (assign7240_e9232, (((locals.var_qf_dn0 + locals.var_q_bf_dn0) + locals.var_ffdqef_dn0) + locals.var_ffdqcfc_dn0), (((locals.var_qf_dn1 + locals.var_q_bf_dn1) + locals.var_ffdqef_dn1) + locals.var_ffdqcfc_dn1), (((locals.var_qf_dn3 + locals.var_q_bf_dn3) + locals.var_ffdqef_dn3) + locals.var_ffdqcfc_dn3), (((locals.var_qf_dn4 + locals.var_q_bf_dn4) + locals.var_ffdqef_dn4) + locals.var_ffdqcfc_dn4), (((locals.var_qf_dn5 + locals.var_q_bf_dn5) + locals.var_ffdqef_dn5) + locals.var_ffdqcfc_dn5), (((locals.var_qf_dn6 + locals.var_q_bf_dn6) + locals.var_ffdqef_dn6) + locals.var_ffdqcfc_dn6), (((locals.var_qf_dn7 + locals.var_q_bf_dn7) + locals.var_ffdqef_dn7) + locals.var_ffdqcfc_dn7), (((locals.var_qf_dn8 + locals.var_q_bf_dn8) + locals.var_ffdqef_dn8) + locals.var_ffdqcfc_dn8), (((locals.var_qf_dn9 + locals.var_q_bf_dn9) + locals.var_ffdqef_dn9) + locals.var_ffdqcfc_dn9),)
    } else {
        (locals.var_qf, locals.var_qf_dn0, locals.var_qf_dn1, locals.var_qf_dn3, locals.var_qf_dn4, locals.var_qf_dn5, locals.var_qf_dn6, locals.var_qf_dn7, locals.var_qf_dn8, locals.var_qf_dn9,)
    }
};
        locals.var_qf = assign7240_e9234;
        locals.var_qf_dn0 = assign7240_e9234_d_n0;
        locals.var_qf_dn1 = assign7240_e9234_d_n1;
        locals.var_qf_dn3 = assign7240_e9234_d_n3;
        locals.var_qf_dn4 = assign7240_e9234_d_n4;
        locals.var_qf_dn5 = assign7240_e9234_d_n5;
        locals.var_qf_dn6 = assign7240_e9234_d_n6;
        locals.var_qf_dn7 = assign7240_e9234_d_n7;
        locals.var_qf_dn8 = assign7240_e9234_d_n8;
        locals.var_qf_dn9 = assign7240_e9234_d_n9;
        locals.var_qf_rv = 0.0;

        let (assign7250_e9257, assign7250_e9257_d_n0, assign7250_e9257_d_n1, assign7250_e9257_d_n3, assign7250_e9257_d_n4, assign7250_e9257_d_n5, assign7250_e9257_d_n6, assign7250_e9257_d_n7, assign7250_e9257_d_n8, assign7250_e9257_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard150 == 0.0)) {
        let assign7250_e9243: f64 = (locals.var_hf0_t * locals.var_tf);
        let assign7250_e9246: f64 = (locals.var_ffdtbfb + locals.var_ffdtbfc);
        let assign7250_e9247: f64 = (assign7250_e9243 + assign7250_e9246);
        let assign7250_e9250: f64 = (locals.var_hfe_t * locals.var_ffdtef);
        let assign7250_e9251: f64 = (assign7250_e9247 + assign7250_e9250);
        let assign7250_e9254: f64 = (locals.var_hfc_t * locals.var_ffdtcfc);
        let assign7250_e9255: f64 = (assign7250_e9251 + assign7250_e9254);
        (assign7250_e9255, ((((locals.var_hf0_t * locals.var_tf_dn0) + (locals.var_ffdtbfb_dn0 + locals.var_ffdtbfc_dn0)) + (locals.var_hfe_t * locals.var_ffdtef_dn0)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn0)), ((((locals.var_hf0_t * locals.var_tf_dn1) + (locals.var_ffdtbfb_dn1 + locals.var_ffdtbfc_dn1)) + (locals.var_hfe_t * locals.var_ffdtef_dn1)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn1)), ((((locals.var_hf0_t * locals.var_tf_dn3) + (locals.var_ffdtbfb_dn3 + locals.var_ffdtbfc_dn3)) + (locals.var_hfe_t * locals.var_ffdtef_dn3)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn3)), (((((locals.var_hf0_t_dn4 * locals.var_tf) + (locals.var_hf0_t * locals.var_tf_dn4)) + (locals.var_ffdtbfb_dn4 + locals.var_ffdtbfc_dn4)) + ((locals.var_hfe_t_dn4 * locals.var_ffdtef) + (locals.var_hfe_t * locals.var_ffdtef_dn4))) + ((locals.var_hfc_t_dn4 * locals.var_ffdtcfc) + (locals.var_hfc_t * locals.var_ffdtcfc_dn4))), ((((locals.var_hf0_t * locals.var_tf_dn5) + (locals.var_ffdtbfb_dn5 + locals.var_ffdtbfc_dn5)) + (locals.var_hfe_t * locals.var_ffdtef_dn5)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn5)), ((((locals.var_hf0_t * locals.var_tf_dn6) + (locals.var_ffdtbfb_dn6 + locals.var_ffdtbfc_dn6)) + (locals.var_hfe_t * locals.var_ffdtef_dn6)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn6)), ((((locals.var_hf0_t * locals.var_tf_dn7) + (locals.var_ffdtbfb_dn7 + locals.var_ffdtbfc_dn7)) + (locals.var_hfe_t * locals.var_ffdtef_dn7)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn7)), ((((locals.var_hf0_t * locals.var_tf_dn8) + (locals.var_ffdtbfb_dn8 + locals.var_ffdtbfc_dn8)) + (locals.var_hfe_t * locals.var_ffdtef_dn8)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn8)), ((((locals.var_hf0_t * locals.var_tf_dn9) + (locals.var_ffdtbfb_dn9 + locals.var_ffdtbfc_dn9)) + (locals.var_hfe_t * locals.var_ffdtef_dn9)) + (locals.var_hfc_t * locals.var_ffdtcfc_dn9)),)
    } else {
        (locals.var_t_ft, locals.var_t_ft_dn0, locals.var_t_ft_dn1, locals.var_t_ft_dn3, locals.var_t_ft_dn4, locals.var_t_ft_dn5, locals.var_t_ft_dn6, locals.var_t_ft_dn7, locals.var_t_ft_dn8, locals.var_t_ft_dn9,)
    }
};
        locals.var_t_ft = assign7250_e9257;
        locals.var_t_ft_dn0 = assign7250_e9257_d_n0;
        locals.var_t_ft_dn1 = assign7250_e9257_d_n1;
        locals.var_t_ft_dn3 = assign7250_e9257_d_n3;
        locals.var_t_ft_dn4 = assign7250_e9257_d_n4;
        locals.var_t_ft_dn5 = assign7250_e9257_d_n5;
        locals.var_t_ft_dn6 = assign7250_e9257_d_n6;
        locals.var_t_ft_dn7 = assign7250_e9257_d_n7;
        locals.var_t_ft_dn8 = assign7250_e9257_d_n8;
        locals.var_t_ft_dn9 = assign7250_e9257_d_n9;
        locals.var_t_ft_rv = 0.0;

        let (assign7260_e9274, assign7260_e9274_d_n0, assign7260_e9274_d_n1, assign7260_e9274_d_n3, assign7260_e9274_d_n4, assign7260_e9274_d_n5, assign7260_e9274_d_n6, assign7260_e9274_d_n7, assign7260_e9274_d_n8, assign7260_e9274_d_n9,) = {
    if (((locals.var_guard131 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_guard150 == 0.0)) {
        let assign7260_e9267: f64 = (locals.var_ffdtbfb + locals.var_ffdtbfc);
        let assign7260_e9268: f64 = (locals.var_tf + assign7260_e9267);
        let assign7260_e9270: f64 = (assign7260_e9268 + locals.var_ffdtef);
        let assign7260_e9272: f64 = (assign7260_e9270 + locals.var_ffdtcfc);
        (assign7260_e9272, (((locals.var_tf_dn0 + (locals.var_ffdtbfb_dn0 + locals.var_ffdtbfc_dn0)) + locals.var_ffdtef_dn0) + locals.var_ffdtcfc_dn0), (((locals.var_tf_dn1 + (locals.var_ffdtbfb_dn1 + locals.var_ffdtbfc_dn1)) + locals.var_ffdtef_dn1) + locals.var_ffdtcfc_dn1), (((locals.var_tf_dn3 + (locals.var_ffdtbfb_dn3 + locals.var_ffdtbfc_dn3)) + locals.var_ffdtef_dn3) + locals.var_ffdtcfc_dn3), (((locals.var_tf_dn4 + (locals.var_ffdtbfb_dn4 + locals.var_ffdtbfc_dn4)) + locals.var_ffdtef_dn4) + locals.var_ffdtcfc_dn4), (((locals.var_tf_dn5 + (locals.var_ffdtbfb_dn5 + locals.var_ffdtbfc_dn5)) + locals.var_ffdtef_dn5) + locals.var_ffdtcfc_dn5), (((locals.var_tf_dn6 + (locals.var_ffdtbfb_dn6 + locals.var_ffdtbfc_dn6)) + locals.var_ffdtef_dn6) + locals.var_ffdtcfc_dn6), (((locals.var_tf_dn7 + (locals.var_ffdtbfb_dn7 + locals.var_ffdtbfc_dn7)) + locals.var_ffdtef_dn7) + locals.var_ffdtcfc_dn7), (((locals.var_tf_dn8 + (locals.var_ffdtbfb_dn8 + locals.var_ffdtbfc_dn8)) + locals.var_ffdtef_dn8) + locals.var_ffdtcfc_dn8), (((locals.var_tf_dn9 + (locals.var_ffdtbfb_dn9 + locals.var_ffdtbfc_dn9)) + locals.var_ffdtef_dn9) + locals.var_ffdtcfc_dn9),)
    } else {
        (locals.var_tf, locals.var_tf_dn0, locals.var_tf_dn1, locals.var_tf_dn3, locals.var_tf_dn4, locals.var_tf_dn5, locals.var_tf_dn6, locals.var_tf_dn7, locals.var_tf_dn8, locals.var_tf_dn9,)
    }
};
        locals.var_tf = assign7260_e9274;
        locals.var_tf_dn0 = assign7260_e9274_d_n0;
        locals.var_tf_dn1 = assign7260_e9274_d_n1;
        locals.var_tf_dn3 = assign7260_e9274_d_n3;
        locals.var_tf_dn4 = assign7260_e9274_d_n4;
        locals.var_tf_dn5 = assign7260_e9274_d_n5;
        locals.var_tf_dn6 = assign7260_e9274_d_n6;
        locals.var_tf_dn7 = assign7260_e9274_d_n7;
        locals.var_tf_dn8 = assign7260_e9274_d_n8;
        locals.var_tf_dn9 = assign7260_e9274_d_n9;
        locals.var_tf_rv = 0.0;

        let (assign7270_e9280, assign7270_e9280_d_n0, assign7270_e9280_d_n1, assign7270_e9280_d_n3, assign7270_e9280_d_n4, assign7270_e9280_d_n5, assign7270_e9280_d_n6, assign7270_e9280_d_n7, assign7270_e9280_d_n8, assign7270_e9280_d_n9,) = {
    if (locals.var_guard131 != 0.0) {
        let assign7270_e9278: f64 = (p.p85 * locals.var_itr);
        (assign7270_e9278, (p.p85 * locals.var_itr_dn0), (p.p85 * locals.var_itr_dn1), (p.p85 * locals.var_itr_dn3), (p.p85 * locals.var_itr_dn4), (p.p85 * locals.var_itr_dn5), (p.p85 * locals.var_itr_dn6), (p.p85 * locals.var_itr_dn7), (p.p85 * locals.var_itr_dn8), (p.p85 * locals.var_itr_dn9),)
    } else {
        (locals.var_qr, locals.var_qr_dn0, locals.var_qr_dn1, locals.var_qr_dn3, locals.var_qr_dn4, locals.var_qr_dn5, locals.var_qr_dn6, locals.var_qr_dn7, locals.var_qr_dn8, locals.var_qr_dn9,)
    }
};
        locals.var_qr = assign7270_e9280;
        locals.var_qr_dn0 = assign7270_e9280_d_n0;
        locals.var_qr_dn1 = assign7270_e9280_d_n1;
        locals.var_qr_dn3 = assign7270_e9280_d_n3;
        locals.var_qr_dn4 = assign7270_e9280_d_n4;
        locals.var_qr_dn5 = assign7270_e9280_d_n5;
        locals.var_qr_dn6 = assign7270_e9280_d_n6;
        locals.var_qr_dn7 = assign7270_e9280_d_n7;
        locals.var_qr_dn8 = assign7270_e9280_d_n8;
        locals.var_qr_dn9 = assign7270_e9280_d_n9;
        locals.var_qr_rv = 0.0;

        let assign7280_e9283: f64 = (locals.var_itf - locals.var_itr);
        locals.var_it = assign7280_e9283;
        locals.var_it_dn0 = (locals.var_itf_dn0 - locals.var_itr_dn0);
        locals.var_it_dn1 = (locals.var_itf_dn1 - locals.var_itr_dn1);
        locals.var_it_dn3 = (locals.var_itf_dn3 - locals.var_itr_dn3);
        locals.var_it_dn4 = (locals.var_itf_dn4 - locals.var_itr_dn4);
        locals.var_it_dn5 = (locals.var_itf_dn5 - locals.var_itr_dn5);
        locals.var_it_dn6 = (locals.var_itf_dn6 - locals.var_itr_dn6);
        locals.var_it_dn7 = (locals.var_itf_dn7 - locals.var_itr_dn7);
        locals.var_it_dn8 = (locals.var_itf_dn8 - locals.var_itr_dn8);
        locals.var_it_dn9 = (locals.var_itf_dn9 - locals.var_itr_dn9);
        locals.var_it_rv = 0.0;

        locals.var_qdei = locals.var_qf;
        locals.var_qdei_dn0 = locals.var_qf_dn0;
        locals.var_qdei_dn1 = locals.var_qf_dn1;
        locals.var_qdei_dn3 = locals.var_qf_dn3;
        locals.var_qdei_dn4 = locals.var_qf_dn4;
        locals.var_qdei_dn5 = locals.var_qf_dn5;
        locals.var_qdei_dn6 = locals.var_qf_dn6;
        locals.var_qdei_dn7 = locals.var_qf_dn7;
        locals.var_qdei_dn8 = locals.var_qf_dn8;
        locals.var_qdei_dn9 = locals.var_qf_dn9;
        locals.var_qdei_rv = 0.0;

        locals.var_qdci = locals.var_qr;
        locals.var_qdci_dn0 = locals.var_qr_dn0;
        locals.var_qdci_dn1 = locals.var_qr_dn1;
        locals.var_qdci_dn3 = locals.var_qr_dn3;
        locals.var_qdci_dn4 = locals.var_qr_dn4;
        locals.var_qdci_dn5 = locals.var_qr_dn5;
        locals.var_qdci_dn6 = locals.var_qr_dn6;
        locals.var_qdci_dn7 = locals.var_qr_dn7;
        locals.var_qdci_dn8 = locals.var_qr_dn8;
        locals.var_qdci_dn9 = locals.var_qr_dn9;
        locals.var_qdci_rv = 0.0;

        let assign7310_e9288: f64 = (locals.var_t_f0 * locals.var_itf);
        let assign7310_e9290: f64 = (assign7310_e9288 * locals.var_ovt);
        locals.var_cdei = assign7310_e9290;
        locals.var_cdei_dn0 = ((locals.var_t_f0 * locals.var_itf_dn0) * locals.var_ovt);
        locals.var_cdei_dn1 = ((locals.var_t_f0 * locals.var_itf_dn1) * locals.var_ovt);
        locals.var_cdei_dn3 = ((locals.var_t_f0 * locals.var_itf_dn3) * locals.var_ovt);
        locals.var_cdei_dn4 = ((((locals.var_t_f0_dn4 * locals.var_itf) + (locals.var_t_f0 * locals.var_itf_dn4)) * locals.var_ovt) + (assign7310_e9288 * locals.var_ovt_dn4));
        locals.var_cdei_dn5 = (((locals.var_t_f0_dn5 * locals.var_itf) + (locals.var_t_f0 * locals.var_itf_dn5)) * locals.var_ovt);
        locals.var_cdei_dn6 = ((locals.var_t_f0 * locals.var_itf_dn6) * locals.var_ovt);
        locals.var_cdei_dn7 = ((locals.var_t_f0 * locals.var_itf_dn7) * locals.var_ovt);
        locals.var_cdei_dn8 = (((locals.var_t_f0_dn8 * locals.var_itf) + (locals.var_t_f0 * locals.var_itf_dn8)) * locals.var_ovt);
        locals.var_cdei_dn9 = ((locals.var_t_f0 * locals.var_itf_dn9) * locals.var_ovt);
        locals.var_cdei_rv = 0.0;

        let assign7320_e9293: f64 = (p.p85 * locals.var_itr);
        let assign7320_e9295: f64 = (assign7320_e9293 * locals.var_ovt);
        locals.var_cdci = assign7320_e9295;
        locals.var_cdci_dn0 = ((p.p85 * locals.var_itr_dn0) * locals.var_ovt);
        locals.var_cdci_dn1 = ((p.p85 * locals.var_itr_dn1) * locals.var_ovt);
        locals.var_cdci_dn3 = ((p.p85 * locals.var_itr_dn3) * locals.var_ovt);
        locals.var_cdci_dn4 = (((p.p85 * locals.var_itr_dn4) * locals.var_ovt) + (assign7320_e9293 * locals.var_ovt_dn4));
        locals.var_cdci_dn5 = ((p.p85 * locals.var_itr_dn5) * locals.var_ovt);
        locals.var_cdci_dn6 = ((p.p85 * locals.var_itr_dn6) * locals.var_ovt);
        locals.var_cdci_dn7 = ((p.p85 * locals.var_itr_dn7) * locals.var_ovt);
        locals.var_cdci_dn8 = ((p.p85 * locals.var_itr_dn8) * locals.var_ovt);
        locals.var_cdci_dn9 = ((p.p85 * locals.var_itr_dn9) * locals.var_ovt);
        locals.var_cdci_rv = 0.0;

        let assign7330_e9299: f64 = (locals.var_cjei + locals.var_cjci);
        let assign7330_e9301: f64 = (assign7330_e9299 + locals.var_cdei);
        let assign7330_e9303: f64 = (assign7330_e9301 + locals.var_cdci);
        let assign7330_e9304: f64 = (p.p93 * assign7330_e9303);
        locals.var_crbi = assign7330_e9304;
        locals.var_crbi_dn0 = (p.p93 * (((locals.var_cjei_dn0 + locals.var_cjci_dn0) + locals.var_cdei_dn0) + locals.var_cdci_dn0));
        locals.var_crbi_dn1 = (p.p93 * (((locals.var_cjei_dn1 + locals.var_cjci_dn1) + locals.var_cdei_dn1) + locals.var_cdci_dn1));
        locals.var_crbi_dn3 = (p.p93 * (((locals.var_cjei_dn3 + locals.var_cjci_dn3) + locals.var_cdei_dn3) + locals.var_cdci_dn3));
        locals.var_crbi_dn4 = (p.p93 * (((locals.var_cjei_dn4 + locals.var_cjci_dn4) + locals.var_cdei_dn4) + locals.var_cdci_dn4));
        locals.var_crbi_dn5 = (p.p93 * (((locals.var_cjei_dn5 + locals.var_cjci_dn5) + locals.var_cdei_dn5) + locals.var_cdci_dn5));
        locals.var_crbi_dn6 = (p.p93 * (((locals.var_cjei_dn6 + locals.var_cjci_dn6) + locals.var_cdei_dn6) + locals.var_cdci_dn6));
        locals.var_crbi_dn7 = (p.p93 * (((locals.var_cjei_dn7 + locals.var_cjci_dn7) + locals.var_cdei_dn7) + locals.var_cdci_dn7));
        locals.var_crbi_dn8 = (p.p93 * (((locals.var_cjei_dn8 + locals.var_cjci_dn8) + locals.var_cdei_dn8) + locals.var_cdci_dn8));
        locals.var_crbi_dn9 = (p.p93 * (((locals.var_cjei_dn9 + locals.var_cjci_dn9) + locals.var_cdei_dn9) + locals.var_cdci_dn9));
        locals.var_crbi_rv = 0.0;

        let assign7340_e9307: f64 = (locals.var_crbi * (nv7 - nv8));
        locals.var_qrbi = assign7340_e9307;
        locals.var_qrbi_dn0 = (locals.var_crbi_dn0 * (nv7 - nv8));
        locals.var_qrbi_dn1 = (locals.var_crbi_dn1 * (nv7 - nv8));
        locals.var_qrbi_dn3 = (locals.var_crbi_dn3 * (nv7 - nv8));
        locals.var_qrbi_dn4 = (locals.var_crbi_dn4 * (nv7 - nv8));
        locals.var_qrbi_dn5 = (locals.var_crbi_dn5 * (nv7 - nv8));
        locals.var_qrbi_dn6 = (locals.var_crbi_dn6 * (nv7 - nv8));
        locals.var_qrbi_dn7 = ((locals.var_crbi_dn7 * (nv7 - nv8)) + locals.var_crbi);
        locals.var_qrbi_dn8 = ((locals.var_crbi_dn8 * (nv7 - nv8)) + (-locals.var_crbi));
        locals.var_qrbi_dn9 = (locals.var_crbi_dn9 * (nv7 - nv8));
        locals.var_qrbi_rv = 0.0;

        let assign7350_e9310: f64 = if p.p23 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard151 = assign7350_e9310;
        locals.var_guard151_rv = 0.0;

        let (assign7360_e9318, assign7360_e9318_d_n4, assign7360_e9318_d_n5, assign7360_e9318_d_n6, assign7360_e9318_d_n7, assign7360_e9318_d_n8, assign7360_e9318_d_n9,) = {
    if (locals.var_guard151 != 0.0) {
        let assign7360_e9315: f64 = (p.p24 * locals.var_vt);
        let assign7360_e9316: f64 = (locals.var_vbici / assign7360_e9315);
        (assign7360_e9316, (-((locals.var_vbici * (p.p24 * locals.var_vt_dn4)) / (assign7360_e9315 * assign7360_e9315))), (locals.var_vbici_dn5 / assign7360_e9315), 0.0, 0.0, (locals.var_vbici_dn8 / assign7360_e9315), 0.0,)
    } else {
        (locals.var_dio_y, locals.var_dio_y_dn4, locals.var_dio_y_dn5, locals.var_dio_y_dn6, locals.var_dio_y_dn7, locals.var_dio_y_dn8, locals.var_dio_y_dn9,)
    }
};
        locals.var_dio_y = assign7360_e9318;
        locals.var_dio_y_dn4 = assign7360_e9318_d_n4;
        locals.var_dio_y_dn5 = assign7360_e9318_d_n5;
        locals.var_dio_y_dn6 = assign7360_e9318_d_n6;
        locals.var_dio_y_dn7 = assign7360_e9318_d_n7;
        locals.var_dio_y_dn8 = assign7360_e9318_d_n8;
        locals.var_dio_y_dn9 = assign7360_e9318_d_n9;
        locals.var_dio_y_rv = 0.0;

        let assign7370_e9321: f64 = if locals.var_dio_y > 80.0 { 1.0 } else { 0.0 };
        locals.var_guard152 = assign7370_e9321;
        locals.var_guard152_rv = 0.0;

        let (assign7380_e9331, assign7380_e9331_d_n4, assign7380_e9331_d_n5, assign7380_e9331_d_n6, assign7380_e9331_d_n7, assign7380_e9331_d_n8, assign7380_e9331_d_n9,) = {
    if ((locals.var_guard151 != 0.0) && (locals.var_guard152 != 0.0)) {
        let assign7380_e9328: f64 = (locals.var_dio_y - 80.0);
        let assign7380_e9329: f64 = (1.0 + assign7380_e9328);
        (assign7380_e9329, locals.var_dio_y_dn4, locals.var_dio_y_dn5, locals.var_dio_y_dn6, locals.var_dio_y_dn7, locals.var_dio_y_dn8, locals.var_dio_y_dn9,)
    } else {
        (locals.var_dio_le, locals.var_dio_le_dn4, locals.var_dio_le_dn5, locals.var_dio_le_dn6, locals.var_dio_le_dn7, locals.var_dio_le_dn8, locals.var_dio_le_dn9,)
    }
};
        locals.var_dio_le = assign7380_e9331;
        locals.var_dio_le_dn4 = assign7380_e9331_d_n4;
        locals.var_dio_le_dn5 = assign7380_e9331_d_n5;
        locals.var_dio_le_dn6 = assign7380_e9331_d_n6;
        locals.var_dio_le_dn7 = assign7380_e9331_d_n7;
        locals.var_dio_le_dn8 = assign7380_e9331_d_n8;
        locals.var_dio_le_dn9 = assign7380_e9331_d_n9;
        locals.var_dio_le_rv = 0.0;

        let (assign7390_e9337, assign7390_e9337_d_n4, assign7390_e9337_d_n5, assign7390_e9337_d_n6, assign7390_e9337_d_n7, assign7390_e9337_d_n8, assign7390_e9337_d_n9,) = {
    if ((locals.var_guard151 != 0.0) && (locals.var_guard152 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dio_y, locals.var_dio_y_dn4, locals.var_dio_y_dn5, locals.var_dio_y_dn6, locals.var_dio_y_dn7, locals.var_dio_y_dn8, locals.var_dio_y_dn9,)
    }
};
        locals.var_dio_y = assign7390_e9337;
        locals.var_dio_y_dn4 = assign7390_e9337_d_n4;
        locals.var_dio_y_dn5 = assign7390_e9337_d_n5;
        locals.var_dio_y_dn6 = assign7390_e9337_d_n6;
        locals.var_dio_y_dn7 = assign7390_e9337_d_n7;
        locals.var_dio_y_dn8 = assign7390_e9337_d_n8;
        locals.var_dio_y_dn9 = assign7390_e9337_d_n9;
        locals.var_dio_y_rv = 0.0;

        let (assign7400_e9344, assign7400_e9344_d_n4, assign7400_e9344_d_n5, assign7400_e9344_d_n6, assign7400_e9344_d_n7, assign7400_e9344_d_n8, assign7400_e9344_d_n9,) = {
    if ((locals.var_guard151 != 0.0) && (locals.var_guard152 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dio_le, locals.var_dio_le_dn4, locals.var_dio_le_dn5, locals.var_dio_le_dn6, locals.var_dio_le_dn7, locals.var_dio_le_dn8, locals.var_dio_le_dn9,)
    }
};
        locals.var_dio_le = assign7400_e9344;
        locals.var_dio_le_dn4 = assign7400_e9344_d_n4;
        locals.var_dio_le_dn5 = assign7400_e9344_d_n5;
        locals.var_dio_le_dn6 = assign7400_e9344_d_n6;
        locals.var_dio_le_dn7 = assign7400_e9344_d_n7;
        locals.var_dio_le_dn8 = assign7400_e9344_d_n8;
        locals.var_dio_le_dn9 = assign7400_e9344_d_n9;
        locals.var_dio_le_rv = 0.0;

        let assign7430_e9367: f64 = if ((p.p37 > 0.0) && (locals.var_vbici < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard153 = assign7430_e9367;
        locals.var_guard153_rv = 0.0;

        let assign7440_e9374: f64 = if ((locals.var_cjci0_t > 0.0) && (locals.var_vdci_t > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard154 = assign7440_e9374;
        locals.var_guard154_rv = 0.0;

        let (assign7450_e9390, assign7450_e9390_d_n0, assign7450_e9390_d_n1, assign7450_e9390_d_n3, assign7450_e9390_d_n4, assign7450_e9390_d_n5, assign7450_e9390_d_n6, assign7450_e9390_d_n7, assign7450_e9390_d_n8, assign7450_e9390_d_n9,) = {
    if ((locals.var_guard153 != 0.0) && (locals.var_guard154 != 0.0)) {
        let assign7450_e9380: f64 = (1.0 / p.p49);
        let assign7450_e9382: f64 = (assign7450_e9380 - 1.0);
        let assign7450_e9385: f64 = (locals.var_cjci / locals.var_cjci0_t);
        let assign7450_e9386: f64 = (assign7450_e9385).ln();
        let assign7450_e9387: f64 = (assign7450_e9382 * assign7450_e9386);
        let assign7450_e9388: f64 = (assign7450_e9387).exp();
        (assign7450_e9388, (assign7450_e9388 * (assign7450_e9382 * ((locals.var_cjci_dn0 / locals.var_cjci0_t) / assign7450_e9385))), (assign7450_e9388 * (assign7450_e9382 * ((locals.var_cjci_dn1 / locals.var_cjci0_t) / assign7450_e9385))), (assign7450_e9388 * (assign7450_e9382 * ((locals.var_cjci_dn3 / locals.var_cjci0_t) / assign7450_e9385))), (assign7450_e9388 * (assign7450_e9382 * ((((locals.var_cjci_dn4 * locals.var_cjci0_t) - (locals.var_cjci * locals.var_cjci0_t_dn4)) / (locals.var_cjci0_t * locals.var_cjci0_t)) / assign7450_e9385))), (assign7450_e9388 * (assign7450_e9382 * ((locals.var_cjci_dn5 / locals.var_cjci0_t) / assign7450_e9385))), (assign7450_e9388 * (assign7450_e9382 * ((locals.var_cjci_dn6 / locals.var_cjci0_t) / assign7450_e9385))), (assign7450_e9388 * (assign7450_e9382 * ((locals.var_cjci_dn7 / locals.var_cjci0_t) / assign7450_e9385))), (assign7450_e9388 * (assign7450_e9382 * ((locals.var_cjci_dn8 / locals.var_cjci0_t) / assign7450_e9385))), (assign7450_e9388 * (assign7450_e9382 * ((locals.var_cjci_dn9 / locals.var_cjci0_t) / assign7450_e9385))),)
    } else {
        (locals.var_dum_c, locals.var_dum_c_dn0, locals.var_dum_c_dn1, locals.var_dum_c_dn3, locals.var_dum_c_dn4, locals.var_dum_c_dn5, locals.var_dum_c_dn6, locals.var_dum_c_dn7, locals.var_dum_c_dn8, locals.var_dum_c_dn9,)
    }
};
        locals.var_dum_c = assign7450_e9390;
        locals.var_dum_c_dn0 = assign7450_e9390_d_n0;
        locals.var_dum_c_dn1 = assign7450_e9390_d_n1;
        locals.var_dum_c_dn3 = assign7450_e9390_d_n3;
        locals.var_dum_c_dn4 = assign7450_e9390_d_n4;
        locals.var_dum_c_dn5 = assign7450_e9390_d_n5;
        locals.var_dum_c_dn6 = assign7450_e9390_d_n6;
        locals.var_dum_c_dn7 = assign7450_e9390_d_n7;
        locals.var_dum_c_dn8 = assign7450_e9390_d_n8;
        locals.var_dum_c_dn9 = assign7450_e9390_d_n9;
        locals.var_dum_c_rv = 0.0;

        let (assign7460_e9403, assign7460_e9403_d_n0, assign7460_e9403_d_n1, assign7460_e9403_d_n3, assign7460_e9403_d_n4, assign7460_e9403_d_n5, assign7460_e9403_d_n6, assign7460_e9403_d_n7, assign7460_e9403_d_n8, assign7460_e9403_d_n9,) = {
    if ((locals.var_guard153 != 0.0) && (locals.var_guard154 != 0.0)) {
        let assign7460_e9395: f64 = (-locals.var_ibcts_t);
        let assign7460_e9397: f64 = (assign7460_e9395 * locals.var_vbici);
        let assign7460_e9400: f64 = (locals.var_vdci_t * locals.var_dum_c);
        let assign7460_e9401: f64 = (assign7460_e9397 / assign7460_e9400);
        (assign7460_e9401, (((((-locals.var_ibcts_t_dn0) * locals.var_vbici) * assign7460_e9400) - (assign7460_e9397 * (locals.var_vdci_t * locals.var_dum_c_dn0))) / (assign7460_e9400 * assign7460_e9400)), (((((-locals.var_ibcts_t_dn1) * locals.var_vbici) * assign7460_e9400) - (assign7460_e9397 * (locals.var_vdci_t * locals.var_dum_c_dn1))) / (assign7460_e9400 * assign7460_e9400)), (((((-locals.var_ibcts_t_dn3) * locals.var_vbici) * assign7460_e9400) - (assign7460_e9397 * (locals.var_vdci_t * locals.var_dum_c_dn3))) / (assign7460_e9400 * assign7460_e9400)), (((((-locals.var_ibcts_t_dn4) * locals.var_vbici) * assign7460_e9400) - (assign7460_e9397 * ((locals.var_vdci_t_dn4 * locals.var_dum_c) + (locals.var_vdci_t * locals.var_dum_c_dn4)))) / (assign7460_e9400 * assign7460_e9400)), ((((((-locals.var_ibcts_t_dn5) * locals.var_vbici) + (assign7460_e9395 * locals.var_vbici_dn5)) * assign7460_e9400) - (assign7460_e9397 * (locals.var_vdci_t * locals.var_dum_c_dn5))) / (assign7460_e9400 * assign7460_e9400)), (((((-locals.var_ibcts_t_dn6) * locals.var_vbici) * assign7460_e9400) - (assign7460_e9397 * (locals.var_vdci_t * locals.var_dum_c_dn6))) / (assign7460_e9400 * assign7460_e9400)), (((((-locals.var_ibcts_t_dn7) * locals.var_vbici) * assign7460_e9400) - (assign7460_e9397 * (locals.var_vdci_t * locals.var_dum_c_dn7))) / (assign7460_e9400 * assign7460_e9400)), ((((((-locals.var_ibcts_t_dn8) * locals.var_vbici) + (assign7460_e9395 * locals.var_vbici_dn8)) * assign7460_e9400) - (assign7460_e9397 * (locals.var_vdci_t * locals.var_dum_c_dn8))) / (assign7460_e9400 * assign7460_e9400)), (((((-locals.var_ibcts_t_dn9) * locals.var_vbici) * assign7460_e9400) - (assign7460_e9397 * (locals.var_vdci_t * locals.var_dum_c_dn9))) / (assign7460_e9400 * assign7460_e9400)),)
    } else {
        (locals.var_dum_a, locals.var_dum_a_dn0, locals.var_dum_a_dn1, locals.var_dum_a_dn3, locals.var_dum_a_dn4, locals.var_dum_a_dn5, locals.var_dum_a_dn6, locals.var_dum_a_dn7, locals.var_dum_a_dn8, locals.var_dum_a_dn9,)
    }
};
        locals.var_dum_a = assign7460_e9403;
        locals.var_dum_a_dn0 = assign7460_e9403_d_n0;
        locals.var_dum_a_dn1 = assign7460_e9403_d_n1;
        locals.var_dum_a_dn3 = assign7460_e9403_d_n3;
        locals.var_dum_a_dn4 = assign7460_e9403_d_n4;
        locals.var_dum_a_dn5 = assign7460_e9403_d_n5;
        locals.var_dum_a_dn6 = assign7460_e9403_d_n6;
        locals.var_dum_a_dn7 = assign7460_e9403_d_n7;
        locals.var_dum_a_dn8 = assign7460_e9403_d_n8;
        locals.var_dum_a_dn9 = assign7460_e9403_d_n9;
        locals.var_dum_a_rv = 0.0;

        let assign7870_e9781: f64 = if p.p18 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard181 = assign7870_e9781;
        locals.var_guard181_rv = 0.0;

        let (assign7880_e9789, assign7880_e9789_d_n4, assign7880_e9789_d_n5, assign7880_e9789_d_n6, assign7880_e9789_d_n7, assign7880_e9789_d_n8, assign7880_e9789_d_n9,) = {
    if (locals.var_guard181 != 0.0) {
        let assign7880_e9786: f64 = (p.p19 * locals.var_vt);
        let assign7880_e9787: f64 = (locals.var_vbpei / assign7880_e9786);
        (assign7880_e9787, (-((locals.var_vbpei * (p.p19 * locals.var_vt_dn4)) / (assign7880_e9786 * assign7880_e9786))), 0.0, (locals.var_vbpei_dn6 / assign7880_e9786), (locals.var_vbpei_dn7 / assign7880_e9786), 0.0, 0.0,)
    } else {
        (locals.var_dio_y, locals.var_dio_y_dn4, locals.var_dio_y_dn5, locals.var_dio_y_dn6, locals.var_dio_y_dn7, locals.var_dio_y_dn8, locals.var_dio_y_dn9,)
    }
};
        locals.var_dio_y = assign7880_e9789;
        locals.var_dio_y_dn4 = assign7880_e9789_d_n4;
        locals.var_dio_y_dn5 = assign7880_e9789_d_n5;
        locals.var_dio_y_dn6 = assign7880_e9789_d_n6;
        locals.var_dio_y_dn7 = assign7880_e9789_d_n7;
        locals.var_dio_y_dn8 = assign7880_e9789_d_n8;
        locals.var_dio_y_dn9 = assign7880_e9789_d_n9;
        locals.var_dio_y_rv = 0.0;

        let assign7890_e9792: f64 = if locals.var_dio_y > 80.0 { 1.0 } else { 0.0 };
        locals.var_guard182 = assign7890_e9792;
        locals.var_guard182_rv = 0.0;

        let (assign7900_e9802, assign7900_e9802_d_n4, assign7900_e9802_d_n5, assign7900_e9802_d_n6, assign7900_e9802_d_n7, assign7900_e9802_d_n8, assign7900_e9802_d_n9,) = {
    if ((locals.var_guard181 != 0.0) && (locals.var_guard182 != 0.0)) {
        let assign7900_e9799: f64 = (locals.var_dio_y - 80.0);
        let assign7900_e9800: f64 = (1.0 + assign7900_e9799);
        (assign7900_e9800, locals.var_dio_y_dn4, locals.var_dio_y_dn5, locals.var_dio_y_dn6, locals.var_dio_y_dn7, locals.var_dio_y_dn8, locals.var_dio_y_dn9,)
    } else {
        (locals.var_dio_le, locals.var_dio_le_dn4, locals.var_dio_le_dn5, locals.var_dio_le_dn6, locals.var_dio_le_dn7, locals.var_dio_le_dn8, locals.var_dio_le_dn9,)
    }
};
        locals.var_dio_le = assign7900_e9802;
        locals.var_dio_le_dn4 = assign7900_e9802_d_n4;
        locals.var_dio_le_dn5 = assign7900_e9802_d_n5;
        locals.var_dio_le_dn6 = assign7900_e9802_d_n6;
        locals.var_dio_le_dn7 = assign7900_e9802_d_n7;
        locals.var_dio_le_dn8 = assign7900_e9802_d_n8;
        locals.var_dio_le_dn9 = assign7900_e9802_d_n9;
        locals.var_dio_le_rv = 0.0;

        let (assign7910_e9808, assign7910_e9808_d_n4, assign7910_e9808_d_n5, assign7910_e9808_d_n6, assign7910_e9808_d_n7, assign7910_e9808_d_n8, assign7910_e9808_d_n9,) = {
    if ((locals.var_guard181 != 0.0) && (locals.var_guard182 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dio_y, locals.var_dio_y_dn4, locals.var_dio_y_dn5, locals.var_dio_y_dn6, locals.var_dio_y_dn7, locals.var_dio_y_dn8, locals.var_dio_y_dn9,)
    }
};
        locals.var_dio_y = assign7910_e9808;
        locals.var_dio_y_dn4 = assign7910_e9808_d_n4;
        locals.var_dio_y_dn5 = assign7910_e9808_d_n5;
        locals.var_dio_y_dn6 = assign7910_e9808_d_n6;
        locals.var_dio_y_dn7 = assign7910_e9808_d_n7;
        locals.var_dio_y_dn8 = assign7910_e9808_d_n8;
        locals.var_dio_y_dn9 = assign7910_e9808_d_n9;
        locals.var_dio_y_rv = 0.0;

        let (assign7920_e9815, assign7920_e9815_d_n4, assign7920_e9815_d_n5, assign7920_e9815_d_n6, assign7920_e9815_d_n7, assign7920_e9815_d_n8, assign7920_e9815_d_n9,) = {
    if ((locals.var_guard181 != 0.0) && (locals.var_guard182 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dio_le, locals.var_dio_le_dn4, locals.var_dio_le_dn5, locals.var_dio_le_dn6, locals.var_dio_le_dn7, locals.var_dio_le_dn8, locals.var_dio_le_dn9,)
    }
};
        locals.var_dio_le = assign7920_e9815;
        locals.var_dio_le_dn4 = assign7920_e9815_d_n4;
        locals.var_dio_le_dn5 = assign7920_e9815_d_n5;
        locals.var_dio_le_dn6 = assign7920_e9815_d_n6;
        locals.var_dio_le_dn7 = assign7920_e9815_d_n7;
        locals.var_dio_le_dn8 = assign7920_e9815_d_n8;
        locals.var_dio_le_dn9 = assign7920_e9815_d_n9;
        locals.var_dio_le_rv = 0.0;

        let assign7950_e9834: f64 = if p.p20 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard183 = assign7950_e9834;
        locals.var_guard183_rv = 0.0;

        let (assign7960_e9842, assign7960_e9842_d_n4, assign7960_e9842_d_n5, assign7960_e9842_d_n6, assign7960_e9842_d_n7, assign7960_e9842_d_n8, assign7960_e9842_d_n9,) = {
    if (locals.var_guard183 != 0.0) {
        let assign7960_e9839: f64 = (p.p21 * locals.var_vt);
        let assign7960_e9840: f64 = (locals.var_vbpei / assign7960_e9839);
        (assign7960_e9840, (-((locals.var_vbpei * (p.p21 * locals.var_vt_dn4)) / (assign7960_e9839 * assign7960_e9839))), 0.0, (locals.var_vbpei_dn6 / assign7960_e9839), (locals.var_vbpei_dn7 / assign7960_e9839), 0.0, 0.0,)
    } else {
        (locals.var_dio_y, locals.var_dio_y_dn4, locals.var_dio_y_dn5, locals.var_dio_y_dn6, locals.var_dio_y_dn7, locals.var_dio_y_dn8, locals.var_dio_y_dn9,)
    }
};
        locals.var_dio_y = assign7960_e9842;
        locals.var_dio_y_dn4 = assign7960_e9842_d_n4;
        locals.var_dio_y_dn5 = assign7960_e9842_d_n5;
        locals.var_dio_y_dn6 = assign7960_e9842_d_n6;
        locals.var_dio_y_dn7 = assign7960_e9842_d_n7;
        locals.var_dio_y_dn8 = assign7960_e9842_d_n8;
        locals.var_dio_y_dn9 = assign7960_e9842_d_n9;
        locals.var_dio_y_rv = 0.0;

        let assign7970_e9845: f64 = if locals.var_dio_y > 80.0 { 1.0 } else { 0.0 };
        locals.var_guard184 = assign7970_e9845;
        locals.var_guard184_rv = 0.0;

        let (assign7980_e9855, assign7980_e9855_d_n4, assign7980_e9855_d_n5, assign7980_e9855_d_n6, assign7980_e9855_d_n7, assign7980_e9855_d_n8, assign7980_e9855_d_n9,) = {
    if ((locals.var_guard183 != 0.0) && (locals.var_guard184 != 0.0)) {
        let assign7980_e9852: f64 = (locals.var_dio_y - 80.0);
        let assign7980_e9853: f64 = (1.0 + assign7980_e9852);
        (assign7980_e9853, locals.var_dio_y_dn4, locals.var_dio_y_dn5, locals.var_dio_y_dn6, locals.var_dio_y_dn7, locals.var_dio_y_dn8, locals.var_dio_y_dn9,)
    } else {
        (locals.var_dio_le, locals.var_dio_le_dn4, locals.var_dio_le_dn5, locals.var_dio_le_dn6, locals.var_dio_le_dn7, locals.var_dio_le_dn8, locals.var_dio_le_dn9,)
    }
};
        locals.var_dio_le = assign7980_e9855;
        locals.var_dio_le_dn4 = assign7980_e9855_d_n4;
        locals.var_dio_le_dn5 = assign7980_e9855_d_n5;
        locals.var_dio_le_dn6 = assign7980_e9855_d_n6;
        locals.var_dio_le_dn7 = assign7980_e9855_d_n7;
        locals.var_dio_le_dn8 = assign7980_e9855_d_n8;
        locals.var_dio_le_dn9 = assign7980_e9855_d_n9;
        locals.var_dio_le_rv = 0.0;

        let (assign7990_e9861, assign7990_e9861_d_n4, assign7990_e9861_d_n5, assign7990_e9861_d_n6, assign7990_e9861_d_n7, assign7990_e9861_d_n8, assign7990_e9861_d_n9,) = {
    if ((locals.var_guard183 != 0.0) && (locals.var_guard184 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dio_y, locals.var_dio_y_dn4, locals.var_dio_y_dn5, locals.var_dio_y_dn6, locals.var_dio_y_dn7, locals.var_dio_y_dn8, locals.var_dio_y_dn9,)
    }
};
        locals.var_dio_y = assign7990_e9861;
        locals.var_dio_y_dn4 = assign7990_e9861_d_n4;
        locals.var_dio_y_dn5 = assign7990_e9861_d_n5;
        locals.var_dio_y_dn6 = assign7990_e9861_d_n6;
        locals.var_dio_y_dn7 = assign7990_e9861_d_n7;
        locals.var_dio_y_dn8 = assign7990_e9861_d_n8;
        locals.var_dio_y_dn9 = assign7990_e9861_d_n9;
        locals.var_dio_y_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_21(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign8000_e9868, assign8000_e9868_d_n4, assign8000_e9868_d_n5, assign8000_e9868_d_n6, assign8000_e9868_d_n7, assign8000_e9868_d_n8, assign8000_e9868_d_n9,) = {
    if ((locals.var_guard183 != 0.0) && (locals.var_guard184 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dio_le, locals.var_dio_le_dn4, locals.var_dio_le_dn5, locals.var_dio_le_dn6, locals.var_dio_le_dn7, locals.var_dio_le_dn8, locals.var_dio_le_dn9,)
    }
};
        locals.var_dio_le = assign8000_e9868;
        locals.var_dio_le_dn4 = assign8000_e9868_d_n4;
        locals.var_dio_le_dn5 = assign8000_e9868_d_n5;
        locals.var_dio_le_dn6 = assign8000_e9868_d_n6;
        locals.var_dio_le_dn7 = assign8000_e9868_d_n7;
        locals.var_dio_le_dn8 = assign8000_e9868_d_n8;
        locals.var_dio_le_dn9 = assign8000_e9868_d_n9;
        locals.var_dio_le_rv = 0.0;

        let assign8030_e9887: f64 = if locals.var_cjep0_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard185 = assign8030_e9887;
        locals.var_guard185_rv = 0.0;

        let (assign8040_e9900, assign8040_e9900_d_n4,) = {
    if (locals.var_guard185 != 0.0) {
        let assign8040_e9892: f64 = (locals.var_ajep_t).ln();
        let assign8040_e9893: f64 = (-assign8040_e9892);
        let assign8040_e9895: f64 = (assign8040_e9893 / p.p45);
        let assign8040_e9896: f64 = (assign8040_e9895).exp();
        let assign8040_e9897: f64 = (1.0 - assign8040_e9896);
        let assign8040_e9898: f64 = (locals.var_vdep_t * assign8040_e9897);
        (assign8040_e9898, ((locals.var_vdep_t_dn4 * assign8040_e9897) + (locals.var_vdep_t * (-(assign8040_e9896 * ((-(locals.var_ajep_t_dn4 / locals.var_ajep_t)) / p.p45))))),)
    } else {
        (locals.var_dfv_f, locals.var_dfv_f_dn4,)
    }
};
        locals.var_dfv_f = assign8040_e9900;
        locals.var_dfv_f_dn4 = assign8040_e9900_d_n4;
        locals.var_dfv_f_rv = 0.0;

        let (assign8050_e9908, assign8050_e9908_d_n0, assign8050_e9908_d_n1, assign8050_e9908_d_n3, assign8050_e9908_d_n4, assign8050_e9908_d_n5, assign8050_e9908_d_n6, assign8050_e9908_d_n7, assign8050_e9908_d_n8, assign8050_e9908_d_n9,) = {
    if (locals.var_guard185 != 0.0) {
        let assign8050_e9904: f64 = (locals.var_dfv_f - locals.var_vbpei);
        let assign8050_e9906: f64 = (assign8050_e9904 * locals.var_ovt);
        (assign8050_e9906, 0.0, 0.0, 0.0, ((locals.var_dfv_f_dn4 * locals.var_ovt) + (assign8050_e9904 * locals.var_ovt_dn4)), 0.0, ((-locals.var_vbpei_dn6) * locals.var_ovt), ((-locals.var_vbpei_dn7) * locals.var_ovt), 0.0, 0.0,)
    } else {
        (locals.var_dfx, locals.var_dfx_dn0, locals.var_dfx_dn1, locals.var_dfx_dn3, locals.var_dfx_dn4, locals.var_dfx_dn5, locals.var_dfx_dn6, locals.var_dfx_dn7, locals.var_dfx_dn8, locals.var_dfx_dn9,)
    }
};
        locals.var_dfx = assign8050_e9908;
        locals.var_dfx_dn0 = assign8050_e9908_d_n0;
        locals.var_dfx_dn1 = assign8050_e9908_d_n1;
        locals.var_dfx_dn3 = assign8050_e9908_d_n3;
        locals.var_dfx_dn4 = assign8050_e9908_d_n4;
        locals.var_dfx_dn5 = assign8050_e9908_d_n5;
        locals.var_dfx_dn6 = assign8050_e9908_d_n6;
        locals.var_dfx_dn7 = assign8050_e9908_d_n7;
        locals.var_dfx_dn8 = assign8050_e9908_d_n8;
        locals.var_dfx_dn9 = assign8050_e9908_d_n9;
        locals.var_dfx_rv = 0.0;

        let (assign8060_e9917, assign8060_e9917_d_n0, assign8060_e9917_d_n1, assign8060_e9917_d_n3, assign8060_e9917_d_n4, assign8060_e9917_d_n5, assign8060_e9917_d_n6, assign8060_e9917_d_n7, assign8060_e9917_d_n8, assign8060_e9917_d_n9,) = {
    if (locals.var_guard185 != 0.0) {
        let assign8060_e9912: f64 = (locals.var_dfx * locals.var_dfx);
        let assign8060_e9914: f64 = (assign8060_e9912 + 1.921812);
        let assign8060_e9915: f64 = (assign8060_e9914).sqrt();
        (assign8060_e9915, (((locals.var_dfx_dn0 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn0)) / (2.0 * assign8060_e9915)), (((locals.var_dfx_dn1 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn1)) / (2.0 * assign8060_e9915)), (((locals.var_dfx_dn3 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn3)) / (2.0 * assign8060_e9915)), (((locals.var_dfx_dn4 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn4)) / (2.0 * assign8060_e9915)), (((locals.var_dfx_dn5 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn5)) / (2.0 * assign8060_e9915)), (((locals.var_dfx_dn6 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn6)) / (2.0 * assign8060_e9915)), (((locals.var_dfx_dn7 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn7)) / (2.0 * assign8060_e9915)), (((locals.var_dfx_dn8 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn8)) / (2.0 * assign8060_e9915)), (((locals.var_dfx_dn9 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn9)) / (2.0 * assign8060_e9915)),)
    } else {
        (locals.var_dfs_q, locals.var_dfs_q_dn0, locals.var_dfs_q_dn1, locals.var_dfs_q_dn3, locals.var_dfs_q_dn4, locals.var_dfs_q_dn5, locals.var_dfs_q_dn6, locals.var_dfs_q_dn7, locals.var_dfs_q_dn8, locals.var_dfs_q_dn9,)
    }
};
        locals.var_dfs_q = assign8060_e9917;
        locals.var_dfs_q_dn0 = assign8060_e9917_d_n0;
        locals.var_dfs_q_dn1 = assign8060_e9917_d_n1;
        locals.var_dfs_q_dn3 = assign8060_e9917_d_n3;
        locals.var_dfs_q_dn4 = assign8060_e9917_d_n4;
        locals.var_dfs_q_dn5 = assign8060_e9917_d_n5;
        locals.var_dfs_q_dn6 = assign8060_e9917_d_n6;
        locals.var_dfs_q_dn7 = assign8060_e9917_d_n7;
        locals.var_dfs_q_dn8 = assign8060_e9917_d_n8;
        locals.var_dfs_q_dn9 = assign8060_e9917_d_n9;
        locals.var_dfs_q_rv = 0.0;

        let (assign8070_e9925, assign8070_e9925_d_n0, assign8070_e9925_d_n1, assign8070_e9925_d_n3, assign8070_e9925_d_n4, assign8070_e9925_d_n5, assign8070_e9925_d_n6, assign8070_e9925_d_n7, assign8070_e9925_d_n8, assign8070_e9925_d_n9,) = {
    if (locals.var_guard185 != 0.0) {
        let assign8070_e9921: f64 = (locals.var_dfx + locals.var_dfs_q);
        let assign8070_e9923: f64 = (assign8070_e9921 * 0.5);
        (assign8070_e9923, ((locals.var_dfx_dn0 + locals.var_dfs_q_dn0) * 0.5), ((locals.var_dfx_dn1 + locals.var_dfs_q_dn1) * 0.5), ((locals.var_dfx_dn3 + locals.var_dfs_q_dn3) * 0.5), ((locals.var_dfx_dn4 + locals.var_dfs_q_dn4) * 0.5), ((locals.var_dfx_dn5 + locals.var_dfs_q_dn5) * 0.5), ((locals.var_dfx_dn6 + locals.var_dfs_q_dn6) * 0.5), ((locals.var_dfx_dn7 + locals.var_dfs_q_dn7) * 0.5), ((locals.var_dfx_dn8 + locals.var_dfs_q_dn8) * 0.5), ((locals.var_dfx_dn9 + locals.var_dfs_q_dn9) * 0.5),)
    } else {
        (locals.var_dfs_q2, locals.var_dfs_q2_dn0, locals.var_dfs_q2_dn1, locals.var_dfs_q2_dn3, locals.var_dfs_q2_dn4, locals.var_dfs_q2_dn5, locals.var_dfs_q2_dn6, locals.var_dfs_q2_dn7, locals.var_dfs_q2_dn8, locals.var_dfs_q2_dn9,)
    }
};
        locals.var_dfs_q2 = assign8070_e9925;
        locals.var_dfs_q2_dn0 = assign8070_e9925_d_n0;
        locals.var_dfs_q2_dn1 = assign8070_e9925_d_n1;
        locals.var_dfs_q2_dn3 = assign8070_e9925_d_n3;
        locals.var_dfs_q2_dn4 = assign8070_e9925_d_n4;
        locals.var_dfs_q2_dn5 = assign8070_e9925_d_n5;
        locals.var_dfs_q2_dn6 = assign8070_e9925_d_n6;
        locals.var_dfs_q2_dn7 = assign8070_e9925_d_n7;
        locals.var_dfs_q2_dn8 = assign8070_e9925_d_n8;
        locals.var_dfs_q2_dn9 = assign8070_e9925_d_n9;
        locals.var_dfs_q2_rv = 0.0;

        let (assign8080_e9933, assign8080_e9933_d_n0, assign8080_e9933_d_n1, assign8080_e9933_d_n3, assign8080_e9933_d_n4, assign8080_e9933_d_n5, assign8080_e9933_d_n6, assign8080_e9933_d_n7, assign8080_e9933_d_n8, assign8080_e9933_d_n9,) = {
    if (locals.var_guard185 != 0.0) {
        let assign8080_e9930: f64 = (locals.var_vt * locals.var_dfs_q2);
        let assign8080_e9931: f64 = (locals.var_dfv_f - assign8080_e9930);
        (assign8080_e9931, (-(locals.var_vt * locals.var_dfs_q2_dn0)), (-(locals.var_vt * locals.var_dfs_q2_dn1)), (-(locals.var_vt * locals.var_dfs_q2_dn3)), (locals.var_dfv_f_dn4 - ((locals.var_vt_dn4 * locals.var_dfs_q2) + (locals.var_vt * locals.var_dfs_q2_dn4))), (-(locals.var_vt * locals.var_dfs_q2_dn5)), (-(locals.var_vt * locals.var_dfs_q2_dn6)), (-(locals.var_vt * locals.var_dfs_q2_dn7)), (-(locals.var_vt * locals.var_dfs_q2_dn8)), (-(locals.var_vt * locals.var_dfs_q2_dn9)),)
    } else {
        (locals.var_dfv_j, locals.var_dfv_j_dn0, locals.var_dfv_j_dn1, locals.var_dfv_j_dn3, locals.var_dfv_j_dn4, locals.var_dfv_j_dn5, locals.var_dfv_j_dn6, locals.var_dfv_j_dn7, locals.var_dfv_j_dn8, locals.var_dfv_j_dn9,)
    }
};
        locals.var_dfv_j = assign8080_e9933;
        locals.var_dfv_j_dn0 = assign8080_e9933_d_n0;
        locals.var_dfv_j_dn1 = assign8080_e9933_d_n1;
        locals.var_dfv_j_dn3 = assign8080_e9933_d_n3;
        locals.var_dfv_j_dn4 = assign8080_e9933_d_n4;
        locals.var_dfv_j_dn5 = assign8080_e9933_d_n5;
        locals.var_dfv_j_dn6 = assign8080_e9933_d_n6;
        locals.var_dfv_j_dn7 = assign8080_e9933_d_n7;
        locals.var_dfv_j_dn8 = assign8080_e9933_d_n8;
        locals.var_dfv_j_dn9 = assign8080_e9933_d_n9;
        locals.var_dfv_j_rv = 0.0;

        let (assign8090_e9939, assign8090_e9939_d_n0, assign8090_e9939_d_n1, assign8090_e9939_d_n3, assign8090_e9939_d_n4, assign8090_e9939_d_n5, assign8090_e9939_d_n6, assign8090_e9939_d_n7, assign8090_e9939_d_n8, assign8090_e9939_d_n9,) = {
    if (locals.var_guard185 != 0.0) {
        let assign8090_e9937: f64 = (locals.var_dfs_q2 / locals.var_dfs_q);
        (assign8090_e9937, (((locals.var_dfs_q2_dn0 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn0)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn1 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn1)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn3 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn3)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn4 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn4)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn5 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn5)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn6 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn6)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn7 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn7)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn8 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn8)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn9 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn9)) / (locals.var_dfs_q * locals.var_dfs_q)),)
    } else {
        (locals.var_dfdvj_dv, locals.var_dfdvj_dv_dn0, locals.var_dfdvj_dv_dn1, locals.var_dfdvj_dv_dn3, locals.var_dfdvj_dv_dn4, locals.var_dfdvj_dv_dn5, locals.var_dfdvj_dv_dn6, locals.var_dfdvj_dv_dn7, locals.var_dfdvj_dv_dn8, locals.var_dfdvj_dv_dn9,)
    }
};
        locals.var_dfdvj_dv = assign8090_e9939;
        locals.var_dfdvj_dv_dn0 = assign8090_e9939_d_n0;
        locals.var_dfdvj_dv_dn1 = assign8090_e9939_d_n1;
        locals.var_dfdvj_dv_dn3 = assign8090_e9939_d_n3;
        locals.var_dfdvj_dv_dn4 = assign8090_e9939_d_n4;
        locals.var_dfdvj_dv_dn5 = assign8090_e9939_d_n5;
        locals.var_dfdvj_dv_dn6 = assign8090_e9939_d_n6;
        locals.var_dfdvj_dv_dn7 = assign8090_e9939_d_n7;
        locals.var_dfdvj_dv_dn8 = assign8090_e9939_d_n8;
        locals.var_dfdvj_dv_dn9 = assign8090_e9939_d_n9;
        locals.var_dfdvj_dv_rv = 0.0;

        let (assign8100_e9948, assign8100_e9948_d_n0, assign8100_e9948_d_n1, assign8100_e9948_d_n3, assign8100_e9948_d_n4, assign8100_e9948_d_n5, assign8100_e9948_d_n6, assign8100_e9948_d_n7, assign8100_e9948_d_n8, assign8100_e9948_d_n9,) = {
    if (locals.var_guard185 != 0.0) {
        let assign8100_e9944: f64 = (locals.var_dfv_j / locals.var_vdep_t);
        let assign8100_e9945: f64 = (1.0 - assign8100_e9944);
        let assign8100_e9946: f64 = (assign8100_e9945).ln();
        (assign8100_e9946, ((-(locals.var_dfv_j_dn0 / locals.var_vdep_t)) / assign8100_e9945), ((-(locals.var_dfv_j_dn1 / locals.var_vdep_t)) / assign8100_e9945), ((-(locals.var_dfv_j_dn3 / locals.var_vdep_t)) / assign8100_e9945), ((-(((locals.var_dfv_j_dn4 * locals.var_vdep_t) - (locals.var_dfv_j * locals.var_vdep_t_dn4)) / (locals.var_vdep_t * locals.var_vdep_t))) / assign8100_e9945), ((-(locals.var_dfv_j_dn5 / locals.var_vdep_t)) / assign8100_e9945), ((-(locals.var_dfv_j_dn6 / locals.var_vdep_t)) / assign8100_e9945), ((-(locals.var_dfv_j_dn7 / locals.var_vdep_t)) / assign8100_e9945), ((-(locals.var_dfv_j_dn8 / locals.var_vdep_t)) / assign8100_e9945), ((-(locals.var_dfv_j_dn9 / locals.var_vdep_t)) / assign8100_e9945),)
    } else {
        (locals.var_dfb, locals.var_dfb_dn0, locals.var_dfb_dn1, locals.var_dfb_dn3, locals.var_dfb_dn4, locals.var_dfb_dn5, locals.var_dfb_dn6, locals.var_dfb_dn7, locals.var_dfb_dn8, locals.var_dfb_dn9,)
    }
};
        locals.var_dfb = assign8100_e9948;
        locals.var_dfb_dn0 = assign8100_e9948_d_n0;
        locals.var_dfb_dn1 = assign8100_e9948_d_n1;
        locals.var_dfb_dn3 = assign8100_e9948_d_n3;
        locals.var_dfb_dn4 = assign8100_e9948_d_n4;
        locals.var_dfb_dn5 = assign8100_e9948_d_n5;
        locals.var_dfb_dn6 = assign8100_e9948_d_n6;
        locals.var_dfb_dn7 = assign8100_e9948_d_n7;
        locals.var_dfb_dn8 = assign8100_e9948_d_n8;
        locals.var_dfb_dn9 = assign8100_e9948_d_n9;
        locals.var_dfb_rv = 0.0;

        let (assign8110_e9958, assign8110_e9958_d_n0, assign8110_e9958_d_n1, assign8110_e9958_d_n3, assign8110_e9958_d_n4, assign8110_e9958_d_n5, assign8110_e9958_d_n6, assign8110_e9958_d_n7, assign8110_e9958_d_n8, assign8110_e9958_d_n9,) = {
    if (locals.var_guard185 != 0.0) {
        let assign8110_e9951: f64 = (-p.p45);
        let assign8110_e9953: f64 = (assign8110_e9951 * locals.var_dfb);
        let assign8110_e9954: f64 = (assign8110_e9953).exp();
        let assign8110_e9956: f64 = (assign8110_e9954 * locals.var_dfdvj_dv);
        (assign8110_e9956, (((assign8110_e9954 * (assign8110_e9951 * locals.var_dfb_dn0)) * locals.var_dfdvj_dv) + (assign8110_e9954 * locals.var_dfdvj_dv_dn0)), (((assign8110_e9954 * (assign8110_e9951 * locals.var_dfb_dn1)) * locals.var_dfdvj_dv) + (assign8110_e9954 * locals.var_dfdvj_dv_dn1)), (((assign8110_e9954 * (assign8110_e9951 * locals.var_dfb_dn3)) * locals.var_dfdvj_dv) + (assign8110_e9954 * locals.var_dfdvj_dv_dn3)), (((assign8110_e9954 * (assign8110_e9951 * locals.var_dfb_dn4)) * locals.var_dfdvj_dv) + (assign8110_e9954 * locals.var_dfdvj_dv_dn4)), (((assign8110_e9954 * (assign8110_e9951 * locals.var_dfb_dn5)) * locals.var_dfdvj_dv) + (assign8110_e9954 * locals.var_dfdvj_dv_dn5)), (((assign8110_e9954 * (assign8110_e9951 * locals.var_dfb_dn6)) * locals.var_dfdvj_dv) + (assign8110_e9954 * locals.var_dfdvj_dv_dn6)), (((assign8110_e9954 * (assign8110_e9951 * locals.var_dfb_dn7)) * locals.var_dfdvj_dv) + (assign8110_e9954 * locals.var_dfdvj_dv_dn7)), (((assign8110_e9954 * (assign8110_e9951 * locals.var_dfb_dn8)) * locals.var_dfdvj_dv) + (assign8110_e9954 * locals.var_dfdvj_dv_dn8)), (((assign8110_e9954 * (assign8110_e9951 * locals.var_dfb_dn9)) * locals.var_dfdvj_dv) + (assign8110_e9954 * locals.var_dfdvj_dv_dn9)),)
    } else {
        (locals.var_dfc_j1, locals.var_dfc_j1_dn0, locals.var_dfc_j1_dn1, locals.var_dfc_j1_dn3, locals.var_dfc_j1_dn4, locals.var_dfc_j1_dn5, locals.var_dfc_j1_dn6, locals.var_dfc_j1_dn7, locals.var_dfc_j1_dn8, locals.var_dfc_j1_dn9,)
    }
};
        locals.var_dfc_j1 = assign8110_e9958;
        locals.var_dfc_j1_dn0 = assign8110_e9958_d_n0;
        locals.var_dfc_j1_dn1 = assign8110_e9958_d_n1;
        locals.var_dfc_j1_dn3 = assign8110_e9958_d_n3;
        locals.var_dfc_j1_dn4 = assign8110_e9958_d_n4;
        locals.var_dfc_j1_dn5 = assign8110_e9958_d_n5;
        locals.var_dfc_j1_dn6 = assign8110_e9958_d_n6;
        locals.var_dfc_j1_dn7 = assign8110_e9958_d_n7;
        locals.var_dfc_j1_dn8 = assign8110_e9958_d_n8;
        locals.var_dfc_j1_dn9 = assign8110_e9958_d_n9;
        locals.var_dfc_j1_rv = 0.0;

        let (assign8130_e9987, assign8130_e9987_d_n0, assign8130_e9987_d_n1, assign8130_e9987_d_n3, assign8130_e9987_d_n4, assign8130_e9987_d_n5, assign8130_e9987_d_n6, assign8130_e9987_d_n7, assign8130_e9987_d_n8, assign8130_e9987_d_n9,) = {
    if (locals.var_guard185 != 0.0) {
        let assign8130_e9977: f64 = (1.0 - p.p45);
        let assign8130_e9978: f64 = (locals.var_dfb * assign8130_e9977);
        let assign8130_e9979: f64 = (assign8130_e9978).exp();
        let assign8130_e9980: f64 = (1.0 - assign8130_e9979);
        let assign8130_e9981: f64 = (locals.var_vdep_t * assign8130_e9980);
        let assign8130_e9984: f64 = (1.0 - p.p45);
        let assign8130_e9985: f64 = (assign8130_e9981 / assign8130_e9984);
        (assign8130_e9985, ((locals.var_vdep_t * (-(assign8130_e9979 * (locals.var_dfb_dn0 * assign8130_e9977)))) / assign8130_e9984), ((locals.var_vdep_t * (-(assign8130_e9979 * (locals.var_dfb_dn1 * assign8130_e9977)))) / assign8130_e9984), ((locals.var_vdep_t * (-(assign8130_e9979 * (locals.var_dfb_dn3 * assign8130_e9977)))) / assign8130_e9984), (((locals.var_vdep_t_dn4 * assign8130_e9980) + (locals.var_vdep_t * (-(assign8130_e9979 * (locals.var_dfb_dn4 * assign8130_e9977))))) / assign8130_e9984), ((locals.var_vdep_t * (-(assign8130_e9979 * (locals.var_dfb_dn5 * assign8130_e9977)))) / assign8130_e9984), ((locals.var_vdep_t * (-(assign8130_e9979 * (locals.var_dfb_dn6 * assign8130_e9977)))) / assign8130_e9984), ((locals.var_vdep_t * (-(assign8130_e9979 * (locals.var_dfb_dn7 * assign8130_e9977)))) / assign8130_e9984), ((locals.var_vdep_t * (-(assign8130_e9979 * (locals.var_dfb_dn8 * assign8130_e9977)))) / assign8130_e9984), ((locals.var_vdep_t * (-(assign8130_e9979 * (locals.var_dfb_dn9 * assign8130_e9977)))) / assign8130_e9984),)
    } else {
        (locals.var_dfq_j1, locals.var_dfq_j1_dn0, locals.var_dfq_j1_dn1, locals.var_dfq_j1_dn3, locals.var_dfq_j1_dn4, locals.var_dfq_j1_dn5, locals.var_dfq_j1_dn6, locals.var_dfq_j1_dn7, locals.var_dfq_j1_dn8, locals.var_dfq_j1_dn9,)
    }
};
        locals.var_dfq_j1 = assign8130_e9987;
        locals.var_dfq_j1_dn0 = assign8130_e9987_d_n0;
        locals.var_dfq_j1_dn1 = assign8130_e9987_d_n1;
        locals.var_dfq_j1_dn3 = assign8130_e9987_d_n3;
        locals.var_dfq_j1_dn4 = assign8130_e9987_d_n4;
        locals.var_dfq_j1_dn5 = assign8130_e9987_d_n5;
        locals.var_dfq_j1_dn6 = assign8130_e9987_d_n6;
        locals.var_dfq_j1_dn7 = assign8130_e9987_d_n7;
        locals.var_dfq_j1_dn8 = assign8130_e9987_d_n8;
        locals.var_dfq_j1_dn9 = assign8130_e9987_d_n9;
        locals.var_dfq_j1_rv = 0.0;

        let (assign8140_e9999, assign8140_e9999_d_n0, assign8140_e9999_d_n1, assign8140_e9999_d_n3, assign8140_e9999_d_n4, assign8140_e9999_d_n5, assign8140_e9999_d_n6, assign8140_e9999_d_n7, assign8140_e9999_d_n8, assign8140_e9999_d_n9,) = {
    if (locals.var_guard185 != 0.0) {
        let assign8140_e9994: f64 = (locals.var_vbpei - locals.var_dfv_j);
        let assign8140_e9995: f64 = (locals.var_ajep_t * assign8140_e9994);
        let assign8140_e9996: f64 = (locals.var_dfq_j1 + assign8140_e9995);
        let assign8140_e9997: f64 = (locals.var_cjep0_t * assign8140_e9996);
        (assign8140_e9997, (locals.var_cjep0_t * (locals.var_dfq_j1_dn0 + (locals.var_ajep_t * (-locals.var_dfv_j_dn0)))), (locals.var_cjep0_t * (locals.var_dfq_j1_dn1 + (locals.var_ajep_t * (-locals.var_dfv_j_dn1)))), (locals.var_cjep0_t * (locals.var_dfq_j1_dn3 + (locals.var_ajep_t * (-locals.var_dfv_j_dn3)))), ((locals.var_cjep0_t_dn4 * assign8140_e9996) + (locals.var_cjep0_t * (locals.var_dfq_j1_dn4 + ((locals.var_ajep_t_dn4 * assign8140_e9994) + (locals.var_ajep_t * (-locals.var_dfv_j_dn4)))))), (locals.var_cjep0_t * (locals.var_dfq_j1_dn5 + (locals.var_ajep_t * (-locals.var_dfv_j_dn5)))), (locals.var_cjep0_t * (locals.var_dfq_j1_dn6 + (locals.var_ajep_t * (locals.var_vbpei_dn6 - locals.var_dfv_j_dn6)))), (locals.var_cjep0_t * (locals.var_dfq_j1_dn7 + (locals.var_ajep_t * (locals.var_vbpei_dn7 - locals.var_dfv_j_dn7)))), (locals.var_cjep0_t * (locals.var_dfq_j1_dn8 + (locals.var_ajep_t * (-locals.var_dfv_j_dn8)))), (locals.var_cjep0_t * (locals.var_dfq_j1_dn9 + (locals.var_ajep_t * (-locals.var_dfv_j_dn9)))),)
    } else {
        (locals.var_qjep, locals.var_qjep_dn0, locals.var_qjep_dn1, locals.var_qjep_dn3, locals.var_qjep_dn4, locals.var_qjep_dn5, locals.var_qjep_dn6, locals.var_qjep_dn7, locals.var_qjep_dn8, locals.var_qjep_dn9,)
    }
};
        locals.var_qjep = assign8140_e9999;
        locals.var_qjep_dn0 = assign8140_e9999_d_n0;
        locals.var_qjep_dn1 = assign8140_e9999_d_n1;
        locals.var_qjep_dn3 = assign8140_e9999_d_n3;
        locals.var_qjep_dn4 = assign8140_e9999_d_n4;
        locals.var_qjep_dn5 = assign8140_e9999_d_n5;
        locals.var_qjep_dn6 = assign8140_e9999_d_n6;
        locals.var_qjep_dn7 = assign8140_e9999_d_n7;
        locals.var_qjep_dn8 = assign8140_e9999_d_n8;
        locals.var_qjep_dn9 = assign8140_e9999_d_n9;
        locals.var_qjep_rv = 0.0;

        let (assign8160_e10009, assign8160_e10009_d_n0, assign8160_e10009_d_n1, assign8160_e10009_d_n3, assign8160_e10009_d_n4, assign8160_e10009_d_n5, assign8160_e10009_d_n6, assign8160_e10009_d_n7, assign8160_e10009_d_n8, assign8160_e10009_d_n9,) = {
    if (locals.var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qjep, locals.var_qjep_dn0, locals.var_qjep_dn1, locals.var_qjep_dn3, locals.var_qjep_dn4, locals.var_qjep_dn5, locals.var_qjep_dn6, locals.var_qjep_dn7, locals.var_qjep_dn8, locals.var_qjep_dn9,)
    }
};
        locals.var_qjep = assign8160_e10009;
        locals.var_qjep_dn0 = assign8160_e10009_d_n0;
        locals.var_qjep_dn1 = assign8160_e10009_d_n1;
        locals.var_qjep_dn3 = assign8160_e10009_d_n3;
        locals.var_qjep_dn4 = assign8160_e10009_d_n4;
        locals.var_qjep_dn5 = assign8160_e10009_d_n5;
        locals.var_qjep_dn6 = assign8160_e10009_d_n6;
        locals.var_qjep_dn7 = assign8160_e10009_d_n7;
        locals.var_qjep_dn8 = assign8160_e10009_d_n8;
        locals.var_qjep_dn9 = assign8160_e10009_d_n9;
        locals.var_qjep_rv = 0.0;

        let assign8290_e10159: f64 = if p.p56 < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard191 = assign8290_e10159;
        locals.var_guard191_rv = 0.0;

        let assign8300_e10162: f64 = if locals.var_cjcx02_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard192 = assign8300_e10162;
        locals.var_guard192_rv = 0.0;

        let (assign8310_e10170,) = {
    if ((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) {
        let assign8310_e10168: f64 = (p.p54 / 4.0);
        (assign8310_e10168,)
    } else {
        (locals.var_dz_r,)
    }
};
        locals.var_dz_r = assign8310_e10170;
        locals.var_dz_r_rv = 0.0;

        let (assign8320_e10178, assign8320_e10178_d_n4,) = {
    if ((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) {
        let assign8320_e10176: f64 = (p.p56 - locals.var_vdcx_t);
        (assign8320_e10176, (-locals.var_vdcx_t_dn4),)
    } else {
        (locals.var_dv_p, locals.var_dv_p_dn4,)
    }
};
        locals.var_dv_p = assign8320_e10178;
        locals.var_dv_p_dn4 = assign8320_e10178_d_n4;
        locals.var_dv_p_rv = 0.0;

        let (assign8330_e10193, assign8330_e10193_d_n4,) = {
    if ((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) {
        let assign8330_e10185: f64 = (locals.var_ajcx_t).ln();
        let assign8330_e10186: f64 = (-assign8330_e10185);
        let assign8330_e10188: f64 = (assign8330_e10186 / p.p54);
        let assign8330_e10189: f64 = (assign8330_e10188).exp();
        let assign8330_e10190: f64 = (1.0 - assign8330_e10189);
        let assign8330_e10191: f64 = (locals.var_vdcx_t * assign8330_e10190);
        (assign8330_e10191, ((locals.var_vdcx_t_dn4 * assign8330_e10190) + (locals.var_vdcx_t * (-(assign8330_e10189 * ((-(locals.var_ajcx_t_dn4 / locals.var_ajcx_t)) / p.p54))))),)
    } else {
        (locals.var_dv_f, locals.var_dv_f_dn4,)
    }
};
        locals.var_dv_f = assign8330_e10193;
        locals.var_dv_f_dn4 = assign8330_e10193_d_n4;
        locals.var_dv_f_rv = 0.0;

        let (assign8340_e10201, assign8340_e10201_d_n4,) = {
    if ((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) {
        let assign8340_e10199: f64 = (locals.var_ajcx_t * locals.var_cjcx02_t);
        (assign8340_e10199, ((locals.var_ajcx_t_dn4 * locals.var_cjcx02_t) + (locals.var_ajcx_t * locals.var_cjcx02_t_dn4)),)
    } else {
        (locals.var_dc_max, locals.var_dc_max_dn4,)
    }
};
        locals.var_dc_max = assign8340_e10201;
        locals.var_dc_max_dn4 = assign8340_e10201_d_n4;
        locals.var_dc_max_rv = 0.0;

        let (assign8350_e10217, assign8350_e10217_d_n4,) = {
    if ((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) {
        let assign8350_e10208: f64 = (locals.var_dz_r - p.p54);
        let assign8350_e10211: f64 = (p.p56 / locals.var_vdcx_t);
        let assign8350_e10212: f64 = (assign8350_e10211).ln();
        let assign8350_e10213: f64 = (assign8350_e10208 * assign8350_e10212);
        let assign8350_e10214: f64 = (assign8350_e10213).exp();
        let assign8350_e10215: f64 = (locals.var_cjcx02_t * assign8350_e10214);
        (assign8350_e10215, ((locals.var_cjcx02_t_dn4 * assign8350_e10214) + (locals.var_cjcx02_t * (assign8350_e10214 * (assign8350_e10208 * ((-((p.p56 * locals.var_vdcx_t_dn4) / (locals.var_vdcx_t * locals.var_vdcx_t))) / assign8350_e10211))))),)
    } else {
        (locals.var_dc_c, locals.var_dc_c_dn4,)
    }
};
        locals.var_dc_c = assign8350_e10217;
        locals.var_dc_c_dn4 = assign8350_e10217_d_n4;
        locals.var_dc_c_rv = 0.0;

        let (assign8360_e10227, assign8360_e10227_d_n0, assign8360_e10227_d_n1, assign8360_e10227_d_n3, assign8360_e10227_d_n4, assign8360_e10227_d_n5, assign8360_e10227_d_n7, assign8360_e10227_d_n8, assign8360_e10227_d_n9,) = {
    if ((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) {
        let assign8360_e10223: f64 = (locals.var_dv_f - locals.var_vbpci);
        let assign8360_e10225: f64 = (assign8360_e10223 * locals.var_ovt);
        (assign8360_e10225, 0.0, 0.0, 0.0, ((locals.var_dv_f_dn4 * locals.var_ovt) + (assign8360_e10223 * locals.var_ovt_dn4)), ((-locals.var_vbpci_dn5) * locals.var_ovt), ((-locals.var_vbpci_dn7) * locals.var_ovt), 0.0, 0.0,)
    } else {
        (locals.var_dv_e, locals.var_dv_e_dn0, locals.var_dv_e_dn1, locals.var_dv_e_dn3, locals.var_dv_e_dn4, locals.var_dv_e_dn5, locals.var_dv_e_dn7, locals.var_dv_e_dn8, locals.var_dv_e_dn9,)
    }
};
        locals.var_dv_e = assign8360_e10227;
        locals.var_dv_e_dn0 = assign8360_e10227_d_n0;
        locals.var_dv_e_dn1 = assign8360_e10227_d_n1;
        locals.var_dv_e_dn3 = assign8360_e10227_d_n3;
        locals.var_dv_e_dn4 = assign8360_e10227_d_n4;
        locals.var_dv_e_dn5 = assign8360_e10227_d_n5;
        locals.var_dv_e_dn7 = assign8360_e10227_d_n7;
        locals.var_dv_e_dn8 = assign8360_e10227_d_n8;
        locals.var_dv_e_dn9 = assign8360_e10227_d_n9;
        locals.var_dv_e_rv = 0.0;

        let assign8370_e10230: f64 = if locals.var_dv_e < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard193 = assign8370_e10230;
        locals.var_guard193_rv = 0.0;

        let (assign8380_e10239, assign8380_e10239_d_n0, assign8380_e10239_d_n1, assign8380_e10239_d_n3, assign8380_e10239_d_n4, assign8380_e10239_d_n5, assign8380_e10239_d_n7, assign8380_e10239_d_n8, assign8380_e10239_d_n9,) = {
    if (((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) && (locals.var_guard193 != 0.0)) {
        let assign8380_e10237: f64 = (locals.var_dv_e).exp();
        (assign8380_e10237, (assign8380_e10237 * locals.var_dv_e_dn0), (assign8380_e10237 * locals.var_dv_e_dn1), (assign8380_e10237 * locals.var_dv_e_dn3), (assign8380_e10237 * locals.var_dv_e_dn4), (assign8380_e10237 * locals.var_dv_e_dn5), (assign8380_e10237 * locals.var_dv_e_dn7), (assign8380_e10237 * locals.var_dv_e_dn8), (assign8380_e10237 * locals.var_dv_e_dn9),)
    } else {
        (locals.var_de, locals.var_de_dn0, locals.var_de_dn1, locals.var_de_dn3, locals.var_de_dn4, locals.var_de_dn5, locals.var_de_dn7, locals.var_de_dn8, locals.var_de_dn9,)
    }
};
        locals.var_de = assign8380_e10239;
        locals.var_de_dn0 = assign8380_e10239_d_n0;
        locals.var_de_dn1 = assign8380_e10239_d_n1;
        locals.var_de_dn3 = assign8380_e10239_d_n3;
        locals.var_de_dn4 = assign8380_e10239_d_n4;
        locals.var_de_dn5 = assign8380_e10239_d_n5;
        locals.var_de_dn7 = assign8380_e10239_d_n7;
        locals.var_de_dn8 = assign8380_e10239_d_n8;
        locals.var_de_dn9 = assign8380_e10239_d_n9;
        locals.var_de_rv = 0.0;

        let (assign8390_e10251, assign8390_e10251_d_n0, assign8390_e10251_d_n1, assign8390_e10251_d_n3, assign8390_e10251_d_n4, assign8390_e10251_d_n5, assign8390_e10251_d_n7, assign8390_e10251_d_n8, assign8390_e10251_d_n9,) = {
    if (((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) && (locals.var_guard193 != 0.0)) {
        let assign8390_e10248: f64 = (1.0 + locals.var_de);
        let assign8390_e10249: f64 = (locals.var_de / assign8390_e10248);
        (assign8390_e10249, (((locals.var_de_dn0 * assign8390_e10248) - (locals.var_de * locals.var_de_dn0)) / (assign8390_e10248 * assign8390_e10248)), (((locals.var_de_dn1 * assign8390_e10248) - (locals.var_de * locals.var_de_dn1)) / (assign8390_e10248 * assign8390_e10248)), (((locals.var_de_dn3 * assign8390_e10248) - (locals.var_de * locals.var_de_dn3)) / (assign8390_e10248 * assign8390_e10248)), (((locals.var_de_dn4 * assign8390_e10248) - (locals.var_de * locals.var_de_dn4)) / (assign8390_e10248 * assign8390_e10248)), (((locals.var_de_dn5 * assign8390_e10248) - (locals.var_de * locals.var_de_dn5)) / (assign8390_e10248 * assign8390_e10248)), (((locals.var_de_dn7 * assign8390_e10248) - (locals.var_de * locals.var_de_dn7)) / (assign8390_e10248 * assign8390_e10248)), (((locals.var_de_dn8 * assign8390_e10248) - (locals.var_de * locals.var_de_dn8)) / (assign8390_e10248 * assign8390_e10248)), (((locals.var_de_dn9 * assign8390_e10248) - (locals.var_de * locals.var_de_dn9)) / (assign8390_e10248 * assign8390_e10248)),)
    } else {
        (locals.var_de_1, locals.var_de_1_dn0, locals.var_de_1_dn1, locals.var_de_1_dn3, locals.var_de_1_dn4, locals.var_de_1_dn5, locals.var_de_1_dn7, locals.var_de_1_dn8, locals.var_de_1_dn9,)
    }
};
        locals.var_de_1 = assign8390_e10251;
        locals.var_de_1_dn0 = assign8390_e10251_d_n0;
        locals.var_de_1_dn1 = assign8390_e10251_d_n1;
        locals.var_de_1_dn3 = assign8390_e10251_d_n3;
        locals.var_de_1_dn4 = assign8390_e10251_d_n4;
        locals.var_de_1_dn5 = assign8390_e10251_d_n5;
        locals.var_de_1_dn7 = assign8390_e10251_d_n7;
        locals.var_de_1_dn8 = assign8390_e10251_d_n8;
        locals.var_de_1_dn9 = assign8390_e10251_d_n9;
        locals.var_de_1_rv = 0.0;

        let (assign8400_e10266, assign8400_e10266_d_n0, assign8400_e10266_d_n1, assign8400_e10266_d_n3, assign8400_e10266_d_n4, assign8400_e10266_d_n5, assign8400_e10266_d_n7, assign8400_e10266_d_n8, assign8400_e10266_d_n9,) = {
    if (((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) && (locals.var_guard193 != 0.0)) {
        let assign8400_e10261: f64 = (1.0 + locals.var_de);
        let assign8400_e10262: f64 = (assign8400_e10261).ln();
        let assign8400_e10263: f64 = (locals.var_vt * assign8400_e10262);
        let assign8400_e10264: f64 = (locals.var_dv_f - assign8400_e10263);
        (assign8400_e10264, (-(locals.var_vt * (locals.var_de_dn0 / assign8400_e10261))), (-(locals.var_vt * (locals.var_de_dn1 / assign8400_e10261))), (-(locals.var_vt * (locals.var_de_dn3 / assign8400_e10261))), (locals.var_dv_f_dn4 - ((locals.var_vt_dn4 * assign8400_e10262) + (locals.var_vt * (locals.var_de_dn4 / assign8400_e10261)))), (-(locals.var_vt * (locals.var_de_dn5 / assign8400_e10261))), (-(locals.var_vt * (locals.var_de_dn7 / assign8400_e10261))), (-(locals.var_vt * (locals.var_de_dn8 / assign8400_e10261))), (-(locals.var_vt * (locals.var_de_dn9 / assign8400_e10261))),)
    } else {
        (locals.var_dv_j1, locals.var_dv_j1_dn0, locals.var_dv_j1_dn1, locals.var_dv_j1_dn3, locals.var_dv_j1_dn4, locals.var_dv_j1_dn5, locals.var_dv_j1_dn7, locals.var_dv_j1_dn8, locals.var_dv_j1_dn9,)
    }
};
        locals.var_dv_j1 = assign8400_e10266;
        locals.var_dv_j1_dn0 = assign8400_e10266_d_n0;
        locals.var_dv_j1_dn1 = assign8400_e10266_d_n1;
        locals.var_dv_j1_dn3 = assign8400_e10266_d_n3;
        locals.var_dv_j1_dn4 = assign8400_e10266_d_n4;
        locals.var_dv_j1_dn5 = assign8400_e10266_d_n5;
        locals.var_dv_j1_dn7 = assign8400_e10266_d_n7;
        locals.var_dv_j1_dn8 = assign8400_e10266_d_n8;
        locals.var_dv_j1_dn9 = assign8400_e10266_d_n9;
        locals.var_dv_j1_rv = 0.0;

        let (assign8410_e10275, assign8410_e10275_d_n0, assign8410_e10275_d_n1, assign8410_e10275_d_n3, assign8410_e10275_d_n4, assign8410_e10275_d_n5, assign8410_e10275_d_n7, assign8410_e10275_d_n8, assign8410_e10275_d_n9,) = {
    if (((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) && (locals.var_guard193 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_de_1, locals.var_de_1_dn0, locals.var_de_1_dn1, locals.var_de_1_dn3, locals.var_de_1_dn4, locals.var_de_1_dn5, locals.var_de_1_dn7, locals.var_de_1_dn8, locals.var_de_1_dn9,)
    }
};
        locals.var_de_1 = assign8410_e10275;
        locals.var_de_1_dn0 = assign8410_e10275_d_n0;
        locals.var_de_1_dn1 = assign8410_e10275_d_n1;
        locals.var_de_1_dn3 = assign8410_e10275_d_n3;
        locals.var_de_1_dn4 = assign8410_e10275_d_n4;
        locals.var_de_1_dn5 = assign8410_e10275_d_n5;
        locals.var_de_1_dn7 = assign8410_e10275_d_n7;
        locals.var_de_1_dn8 = assign8410_e10275_d_n8;
        locals.var_de_1_dn9 = assign8410_e10275_d_n9;
        locals.var_de_1_rv = 0.0;

        let (assign8420_e10284, assign8420_e10284_d_n0, assign8420_e10284_d_n1, assign8420_e10284_d_n3, assign8420_e10284_d_n4, assign8420_e10284_d_n5, assign8420_e10284_d_n7, assign8420_e10284_d_n8, assign8420_e10284_d_n9,) = {
    if (((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) && (locals.var_guard193 == 0.0)) {
        (locals.var_vbpci, 0.0, 0.0, 0.0, 0.0, locals.var_vbpci_dn5, locals.var_vbpci_dn7, 0.0, 0.0,)
    } else {
        (locals.var_dv_j1, locals.var_dv_j1_dn0, locals.var_dv_j1_dn1, locals.var_dv_j1_dn3, locals.var_dv_j1_dn4, locals.var_dv_j1_dn5, locals.var_dv_j1_dn7, locals.var_dv_j1_dn8, locals.var_dv_j1_dn9,)
    }
};
        locals.var_dv_j1 = assign8420_e10284;
        locals.var_dv_j1_dn0 = assign8420_e10284_d_n0;
        locals.var_dv_j1_dn1 = assign8420_e10284_d_n1;
        locals.var_dv_j1_dn3 = assign8420_e10284_d_n3;
        locals.var_dv_j1_dn4 = assign8420_e10284_d_n4;
        locals.var_dv_j1_dn5 = assign8420_e10284_d_n5;
        locals.var_dv_j1_dn7 = assign8420_e10284_d_n7;
        locals.var_dv_j1_dn8 = assign8420_e10284_d_n8;
        locals.var_dv_j1_dn9 = assign8420_e10284_d_n9;
        locals.var_dv_j1_rv = 0.0;

        let (assign8430_e10296, assign8430_e10296_d_n4,) = {
    if ((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) {
        let assign8430_e10290: f64 = (0.1 * locals.var_dv_p);
        let assign8430_e10293: f64 = (4.0 * locals.var_vt);
        let assign8430_e10294: f64 = (assign8430_e10290 + assign8430_e10293);
        (assign8430_e10294, ((0.1 * locals.var_dv_p_dn4) + (4.0 * locals.var_vt_dn4)),)
    } else {
        (locals.var_da, locals.var_da_dn4,)
    }
};
        locals.var_da = assign8430_e10296;
        locals.var_da_dn4 = assign8430_e10296_d_n4;
        locals.var_da_rv = 0.0;

        let (assign8440_e10306, assign8440_e10306_d_n0, assign8440_e10306_d_n1, assign8440_e10306_d_n3, assign8440_e10306_d_n4, assign8440_e10306_d_n5, assign8440_e10306_d_n7, assign8440_e10306_d_n8, assign8440_e10306_d_n9,) = {
    if ((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) {
        let assign8440_e10302: f64 = (locals.var_dv_p + locals.var_dv_j1);
        let assign8440_e10304: f64 = (assign8440_e10302 / locals.var_da);
        (assign8440_e10304, (locals.var_dv_j1_dn0 / locals.var_da), (locals.var_dv_j1_dn1 / locals.var_da), (locals.var_dv_j1_dn3 / locals.var_da), ((((locals.var_dv_p_dn4 + locals.var_dv_j1_dn4) * locals.var_da) - (assign8440_e10302 * locals.var_da_dn4)) / (locals.var_da * locals.var_da)), (locals.var_dv_j1_dn5 / locals.var_da), (locals.var_dv_j1_dn7 / locals.var_da), (locals.var_dv_j1_dn8 / locals.var_da), (locals.var_dv_j1_dn9 / locals.var_da),)
    } else {
        (locals.var_dv_r, locals.var_dv_r_dn0, locals.var_dv_r_dn1, locals.var_dv_r_dn3, locals.var_dv_r_dn4, locals.var_dv_r_dn5, locals.var_dv_r_dn7, locals.var_dv_r_dn8, locals.var_dv_r_dn9,)
    }
};
        locals.var_dv_r = assign8440_e10306;
        locals.var_dv_r_dn0 = assign8440_e10306_d_n0;
        locals.var_dv_r_dn1 = assign8440_e10306_d_n1;
        locals.var_dv_r_dn3 = assign8440_e10306_d_n3;
        locals.var_dv_r_dn4 = assign8440_e10306_d_n4;
        locals.var_dv_r_dn5 = assign8440_e10306_d_n5;
        locals.var_dv_r_dn7 = assign8440_e10306_d_n7;
        locals.var_dv_r_dn8 = assign8440_e10306_d_n8;
        locals.var_dv_r_dn9 = assign8440_e10306_d_n9;
        locals.var_dv_r_rv = 0.0;

        let assign8450_e10309: f64 = if locals.var_dv_r < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard194 = assign8450_e10309;
        locals.var_guard194_rv = 0.0;

        let (assign8460_e10318, assign8460_e10318_d_n0, assign8460_e10318_d_n1, assign8460_e10318_d_n3, assign8460_e10318_d_n4, assign8460_e10318_d_n5, assign8460_e10318_d_n7, assign8460_e10318_d_n8, assign8460_e10318_d_n9,) = {
    if (((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) && (locals.var_guard194 != 0.0)) {
        let assign8460_e10316: f64 = (locals.var_dv_r).exp();
        (assign8460_e10316, (assign8460_e10316 * locals.var_dv_r_dn0), (assign8460_e10316 * locals.var_dv_r_dn1), (assign8460_e10316 * locals.var_dv_r_dn3), (assign8460_e10316 * locals.var_dv_r_dn4), (assign8460_e10316 * locals.var_dv_r_dn5), (assign8460_e10316 * locals.var_dv_r_dn7), (assign8460_e10316 * locals.var_dv_r_dn8), (assign8460_e10316 * locals.var_dv_r_dn9),)
    } else {
        (locals.var_de, locals.var_de_dn0, locals.var_de_dn1, locals.var_de_dn3, locals.var_de_dn4, locals.var_de_dn5, locals.var_de_dn7, locals.var_de_dn8, locals.var_de_dn9,)
    }
};
        locals.var_de = assign8460_e10318;
        locals.var_de_dn0 = assign8460_e10318_d_n0;
        locals.var_de_dn1 = assign8460_e10318_d_n1;
        locals.var_de_dn3 = assign8460_e10318_d_n3;
        locals.var_de_dn4 = assign8460_e10318_d_n4;
        locals.var_de_dn5 = assign8460_e10318_d_n5;
        locals.var_de_dn7 = assign8460_e10318_d_n7;
        locals.var_de_dn8 = assign8460_e10318_d_n8;
        locals.var_de_dn9 = assign8460_e10318_d_n9;
        locals.var_de_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_22(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign8470_e10330, assign8470_e10330_d_n0, assign8470_e10330_d_n1, assign8470_e10330_d_n3, assign8470_e10330_d_n4, assign8470_e10330_d_n5, assign8470_e10330_d_n7, assign8470_e10330_d_n8, assign8470_e10330_d_n9,) = {
    if (((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) && (locals.var_guard194 != 0.0)) {
        let assign8470_e10327: f64 = (1.0 + locals.var_de);
        let assign8470_e10328: f64 = (locals.var_de / assign8470_e10327);
        (assign8470_e10328, (((locals.var_de_dn0 * assign8470_e10327) - (locals.var_de * locals.var_de_dn0)) / (assign8470_e10327 * assign8470_e10327)), (((locals.var_de_dn1 * assign8470_e10327) - (locals.var_de * locals.var_de_dn1)) / (assign8470_e10327 * assign8470_e10327)), (((locals.var_de_dn3 * assign8470_e10327) - (locals.var_de * locals.var_de_dn3)) / (assign8470_e10327 * assign8470_e10327)), (((locals.var_de_dn4 * assign8470_e10327) - (locals.var_de * locals.var_de_dn4)) / (assign8470_e10327 * assign8470_e10327)), (((locals.var_de_dn5 * assign8470_e10327) - (locals.var_de * locals.var_de_dn5)) / (assign8470_e10327 * assign8470_e10327)), (((locals.var_de_dn7 * assign8470_e10327) - (locals.var_de * locals.var_de_dn7)) / (assign8470_e10327 * assign8470_e10327)), (((locals.var_de_dn8 * assign8470_e10327) - (locals.var_de * locals.var_de_dn8)) / (assign8470_e10327 * assign8470_e10327)), (((locals.var_de_dn9 * assign8470_e10327) - (locals.var_de * locals.var_de_dn9)) / (assign8470_e10327 * assign8470_e10327)),)
    } else {
        (locals.var_de_2, locals.var_de_2_dn0, locals.var_de_2_dn1, locals.var_de_2_dn3, locals.var_de_2_dn4, locals.var_de_2_dn5, locals.var_de_2_dn7, locals.var_de_2_dn8, locals.var_de_2_dn9,)
    }
};
        locals.var_de_2 = assign8470_e10330;
        locals.var_de_2_dn0 = assign8470_e10330_d_n0;
        locals.var_de_2_dn1 = assign8470_e10330_d_n1;
        locals.var_de_2_dn3 = assign8470_e10330_d_n3;
        locals.var_de_2_dn4 = assign8470_e10330_d_n4;
        locals.var_de_2_dn5 = assign8470_e10330_d_n5;
        locals.var_de_2_dn7 = assign8470_e10330_d_n7;
        locals.var_de_2_dn8 = assign8470_e10330_d_n8;
        locals.var_de_2_dn9 = assign8470_e10330_d_n9;
        locals.var_de_2_rv = 0.0;

        let (assign8480_e10354, assign8480_e10354_d_n0, assign8480_e10354_d_n1, assign8480_e10354_d_n3, assign8480_e10354_d_n4, assign8480_e10354_d_n5, assign8480_e10354_d_n7, assign8480_e10354_d_n8, assign8480_e10354_d_n9,) = {
    if (((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) && (locals.var_guard194 != 0.0)) {
        let assign8480_e10337: f64 = (-locals.var_dv_p);
        let assign8480_e10341: f64 = (1.0 + locals.var_de);
        let assign8480_e10342: f64 = (assign8480_e10341).ln();
        let assign8480_e10345: f64 = (locals.var_dv_p + locals.var_dv_f);
        let assign8480_e10346: f64 = (-assign8480_e10345);
        let assign8480_e10348: f64 = (assign8480_e10346 / locals.var_da);
        let assign8480_e10349: f64 = (assign8480_e10348).exp();
        let assign8480_e10350: f64 = (assign8480_e10342 - assign8480_e10349);
        let assign8480_e10351: f64 = (locals.var_da * assign8480_e10350);
        let assign8480_e10352: f64 = (assign8480_e10337 + assign8480_e10351);
        (assign8480_e10352, (locals.var_da * (locals.var_de_dn0 / assign8480_e10341)), (locals.var_da * (locals.var_de_dn1 / assign8480_e10341)), (locals.var_da * (locals.var_de_dn3 / assign8480_e10341)), ((-locals.var_dv_p_dn4) + ((locals.var_da_dn4 * assign8480_e10350) + (locals.var_da * ((locals.var_de_dn4 / assign8480_e10341) - (assign8480_e10349 * ((((-(locals.var_dv_p_dn4 + locals.var_dv_f_dn4)) * locals.var_da) - (assign8480_e10346 * locals.var_da_dn4)) / (locals.var_da * locals.var_da))))))), (locals.var_da * (locals.var_de_dn5 / assign8480_e10341)), (locals.var_da * (locals.var_de_dn7 / assign8480_e10341)), (locals.var_da * (locals.var_de_dn8 / assign8480_e10341)), (locals.var_da * (locals.var_de_dn9 / assign8480_e10341)),)
    } else {
        (locals.var_dv_j2, locals.var_dv_j2_dn0, locals.var_dv_j2_dn1, locals.var_dv_j2_dn3, locals.var_dv_j2_dn4, locals.var_dv_j2_dn5, locals.var_dv_j2_dn7, locals.var_dv_j2_dn8, locals.var_dv_j2_dn9,)
    }
};
        locals.var_dv_j2 = assign8480_e10354;
        locals.var_dv_j2_dn0 = assign8480_e10354_d_n0;
        locals.var_dv_j2_dn1 = assign8480_e10354_d_n1;
        locals.var_dv_j2_dn3 = assign8480_e10354_d_n3;
        locals.var_dv_j2_dn4 = assign8480_e10354_d_n4;
        locals.var_dv_j2_dn5 = assign8480_e10354_d_n5;
        locals.var_dv_j2_dn7 = assign8480_e10354_d_n7;
        locals.var_dv_j2_dn8 = assign8480_e10354_d_n8;
        locals.var_dv_j2_dn9 = assign8480_e10354_d_n9;
        locals.var_dv_j2_rv = 0.0;

        let (assign8490_e10363, assign8490_e10363_d_n0, assign8490_e10363_d_n1, assign8490_e10363_d_n3, assign8490_e10363_d_n4, assign8490_e10363_d_n5, assign8490_e10363_d_n7, assign8490_e10363_d_n8, assign8490_e10363_d_n9,) = {
    if (((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) && (locals.var_guard194 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_de_2, locals.var_de_2_dn0, locals.var_de_2_dn1, locals.var_de_2_dn3, locals.var_de_2_dn4, locals.var_de_2_dn5, locals.var_de_2_dn7, locals.var_de_2_dn8, locals.var_de_2_dn9,)
    }
};
        locals.var_de_2 = assign8490_e10363;
        locals.var_de_2_dn0 = assign8490_e10363_d_n0;
        locals.var_de_2_dn1 = assign8490_e10363_d_n1;
        locals.var_de_2_dn3 = assign8490_e10363_d_n3;
        locals.var_de_2_dn4 = assign8490_e10363_d_n4;
        locals.var_de_2_dn5 = assign8490_e10363_d_n5;
        locals.var_de_2_dn7 = assign8490_e10363_d_n7;
        locals.var_de_2_dn8 = assign8490_e10363_d_n8;
        locals.var_de_2_dn9 = assign8490_e10363_d_n9;
        locals.var_de_2_rv = 0.0;

        let (assign8500_e10372, assign8500_e10372_d_n0, assign8500_e10372_d_n1, assign8500_e10372_d_n3, assign8500_e10372_d_n4, assign8500_e10372_d_n5, assign8500_e10372_d_n7, assign8500_e10372_d_n8, assign8500_e10372_d_n9,) = {
    if (((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) && (locals.var_guard194 == 0.0)) {
        (locals.var_dv_j1, locals.var_dv_j1_dn0, locals.var_dv_j1_dn1, locals.var_dv_j1_dn3, locals.var_dv_j1_dn4, locals.var_dv_j1_dn5, locals.var_dv_j1_dn7, locals.var_dv_j1_dn8, locals.var_dv_j1_dn9,)
    } else {
        (locals.var_dv_j2, locals.var_dv_j2_dn0, locals.var_dv_j2_dn1, locals.var_dv_j2_dn3, locals.var_dv_j2_dn4, locals.var_dv_j2_dn5, locals.var_dv_j2_dn7, locals.var_dv_j2_dn8, locals.var_dv_j2_dn9,)
    }
};
        locals.var_dv_j2 = assign8500_e10372;
        locals.var_dv_j2_dn0 = assign8500_e10372_d_n0;
        locals.var_dv_j2_dn1 = assign8500_e10372_d_n1;
        locals.var_dv_j2_dn3 = assign8500_e10372_d_n3;
        locals.var_dv_j2_dn4 = assign8500_e10372_d_n4;
        locals.var_dv_j2_dn5 = assign8500_e10372_d_n5;
        locals.var_dv_j2_dn7 = assign8500_e10372_d_n7;
        locals.var_dv_j2_dn8 = assign8500_e10372_d_n8;
        locals.var_dv_j2_dn9 = assign8500_e10372_d_n9;
        locals.var_dv_j2_rv = 0.0;

        let (assign8510_e10380, assign8510_e10380_d_n0, assign8510_e10380_d_n1, assign8510_e10380_d_n3, assign8510_e10380_d_n4, assign8510_e10380_d_n5, assign8510_e10380_d_n7, assign8510_e10380_d_n8, assign8510_e10380_d_n9,) = {
    if ((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) {
        let assign8510_e10378: f64 = (locals.var_vbpci - locals.var_dv_j1);
        (assign8510_e10378, (-locals.var_dv_j1_dn0), (-locals.var_dv_j1_dn1), (-locals.var_dv_j1_dn3), (-locals.var_dv_j1_dn4), (locals.var_vbpci_dn5 - locals.var_dv_j1_dn5), (locals.var_vbpci_dn7 - locals.var_dv_j1_dn7), (-locals.var_dv_j1_dn8), (-locals.var_dv_j1_dn9),)
    } else {
        (locals.var_dv_j4, locals.var_dv_j4_dn0, locals.var_dv_j4_dn1, locals.var_dv_j4_dn3, locals.var_dv_j4_dn4, locals.var_dv_j4_dn5, locals.var_dv_j4_dn7, locals.var_dv_j4_dn8, locals.var_dv_j4_dn9,)
    }
};
        locals.var_dv_j4 = assign8510_e10380;
        locals.var_dv_j4_dn0 = assign8510_e10380_d_n0;
        locals.var_dv_j4_dn1 = assign8510_e10380_d_n1;
        locals.var_dv_j4_dn3 = assign8510_e10380_d_n3;
        locals.var_dv_j4_dn4 = assign8510_e10380_d_n4;
        locals.var_dv_j4_dn5 = assign8510_e10380_d_n5;
        locals.var_dv_j4_dn7 = assign8510_e10380_d_n7;
        locals.var_dv_j4_dn8 = assign8510_e10380_d_n8;
        locals.var_dv_j4_dn9 = assign8510_e10380_d_n9;
        locals.var_dv_j4_rv = 0.0;

        let (assign8520_e10391, assign8520_e10391_d_n0, assign8520_e10391_d_n1, assign8520_e10391_d_n3, assign8520_e10391_d_n4, assign8520_e10391_d_n5, assign8520_e10391_d_n7, assign8520_e10391_d_n8, assign8520_e10391_d_n9,) = {
    if ((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) {
        let assign8520_e10387: f64 = (locals.var_dv_j1 / locals.var_vdcx_t);
        let assign8520_e10388: f64 = (1.0 - assign8520_e10387);
        let assign8520_e10389: f64 = (assign8520_e10388).ln();
        (assign8520_e10389, ((-(locals.var_dv_j1_dn0 / locals.var_vdcx_t)) / assign8520_e10388), ((-(locals.var_dv_j1_dn1 / locals.var_vdcx_t)) / assign8520_e10388), ((-(locals.var_dv_j1_dn3 / locals.var_vdcx_t)) / assign8520_e10388), ((-(((locals.var_dv_j1_dn4 * locals.var_vdcx_t) - (locals.var_dv_j1 * locals.var_vdcx_t_dn4)) / (locals.var_vdcx_t * locals.var_vdcx_t))) / assign8520_e10388), ((-(locals.var_dv_j1_dn5 / locals.var_vdcx_t)) / assign8520_e10388), ((-(locals.var_dv_j1_dn7 / locals.var_vdcx_t)) / assign8520_e10388), ((-(locals.var_dv_j1_dn8 / locals.var_vdcx_t)) / assign8520_e10388), ((-(locals.var_dv_j1_dn9 / locals.var_vdcx_t)) / assign8520_e10388),)
    } else {
        (locals.var_dcln1, locals.var_dcln1_dn0, locals.var_dcln1_dn1, locals.var_dcln1_dn3, locals.var_dcln1_dn4, locals.var_dcln1_dn5, locals.var_dcln1_dn7, locals.var_dcln1_dn8, locals.var_dcln1_dn9,)
    }
};
        locals.var_dcln1 = assign8520_e10391;
        locals.var_dcln1_dn0 = assign8520_e10391_d_n0;
        locals.var_dcln1_dn1 = assign8520_e10391_d_n1;
        locals.var_dcln1_dn3 = assign8520_e10391_d_n3;
        locals.var_dcln1_dn4 = assign8520_e10391_d_n4;
        locals.var_dcln1_dn5 = assign8520_e10391_d_n5;
        locals.var_dcln1_dn7 = assign8520_e10391_d_n7;
        locals.var_dcln1_dn8 = assign8520_e10391_d_n8;
        locals.var_dcln1_dn9 = assign8520_e10391_d_n9;
        locals.var_dcln1_rv = 0.0;

        let (assign8530_e10402, assign8530_e10402_d_n0, assign8530_e10402_d_n1, assign8530_e10402_d_n3, assign8530_e10402_d_n4, assign8530_e10402_d_n5, assign8530_e10402_d_n7, assign8530_e10402_d_n8, assign8530_e10402_d_n9,) = {
    if ((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) {
        let assign8530_e10398: f64 = (locals.var_dv_j2 / locals.var_vdcx_t);
        let assign8530_e10399: f64 = (1.0 - assign8530_e10398);
        let assign8530_e10400: f64 = (assign8530_e10399).ln();
        (assign8530_e10400, ((-(locals.var_dv_j2_dn0 / locals.var_vdcx_t)) / assign8530_e10399), ((-(locals.var_dv_j2_dn1 / locals.var_vdcx_t)) / assign8530_e10399), ((-(locals.var_dv_j2_dn3 / locals.var_vdcx_t)) / assign8530_e10399), ((-(((locals.var_dv_j2_dn4 * locals.var_vdcx_t) - (locals.var_dv_j2 * locals.var_vdcx_t_dn4)) / (locals.var_vdcx_t * locals.var_vdcx_t))) / assign8530_e10399), ((-(locals.var_dv_j2_dn5 / locals.var_vdcx_t)) / assign8530_e10399), ((-(locals.var_dv_j2_dn7 / locals.var_vdcx_t)) / assign8530_e10399), ((-(locals.var_dv_j2_dn8 / locals.var_vdcx_t)) / assign8530_e10399), ((-(locals.var_dv_j2_dn9 / locals.var_vdcx_t)) / assign8530_e10399),)
    } else {
        (locals.var_dcln2, locals.var_dcln2_dn0, locals.var_dcln2_dn1, locals.var_dcln2_dn3, locals.var_dcln2_dn4, locals.var_dcln2_dn5, locals.var_dcln2_dn7, locals.var_dcln2_dn8, locals.var_dcln2_dn9,)
    }
};
        locals.var_dcln2 = assign8530_e10402;
        locals.var_dcln2_dn0 = assign8530_e10402_d_n0;
        locals.var_dcln2_dn1 = assign8530_e10402_d_n1;
        locals.var_dcln2_dn3 = assign8530_e10402_d_n3;
        locals.var_dcln2_dn4 = assign8530_e10402_d_n4;
        locals.var_dcln2_dn5 = assign8530_e10402_d_n5;
        locals.var_dcln2_dn7 = assign8530_e10402_d_n7;
        locals.var_dcln2_dn8 = assign8530_e10402_d_n8;
        locals.var_dcln2_dn9 = assign8530_e10402_d_n9;
        locals.var_dcln2_rv = 0.0;

        let (assign8540_e10410,) = {
    if ((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) {
        let assign8540_e10408: f64 = (1.0 - p.p54);
        (assign8540_e10408,)
    } else {
        (locals.var_dz1,)
    }
};
        locals.var_dz1 = assign8540_e10410;
        locals.var_dz1_rv = 0.0;

        let (assign8550_e10418,) = {
    if ((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) {
        let assign8550_e10416: f64 = (1.0 - locals.var_dz_r);
        (assign8550_e10416,)
    } else {
        (locals.var_dzr1,)
    }
};
        locals.var_dzr1 = assign8550_e10418;
        locals.var_dzr1_rv = 0.0;

        let (assign8560_e10434, assign8560_e10434_d_n0, assign8560_e10434_d_n1, assign8560_e10434_d_n3, assign8560_e10434_d_n4, assign8560_e10434_d_n5, assign8560_e10434_d_n7, assign8560_e10434_d_n8, assign8560_e10434_d_n9,) = {
    if ((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) {
        let assign8560_e10425: f64 = (-p.p54);
        let assign8560_e10426: f64 = (locals.var_dcln2 * assign8560_e10425);
        let assign8560_e10427: f64 = (assign8560_e10426).exp();
        let assign8560_e10428: f64 = (locals.var_cjcx02_t * assign8560_e10427);
        let assign8560_e10430: f64 = (assign8560_e10428 * locals.var_de_1);
        let assign8560_e10432: f64 = (assign8560_e10430 * locals.var_de_2);
        (assign8560_e10432, (((((locals.var_cjcx02_t * (assign8560_e10427 * (locals.var_dcln2_dn0 * assign8560_e10425))) * locals.var_de_1) + (assign8560_e10428 * locals.var_de_1_dn0)) * locals.var_de_2) + (assign8560_e10430 * locals.var_de_2_dn0)), (((((locals.var_cjcx02_t * (assign8560_e10427 * (locals.var_dcln2_dn1 * assign8560_e10425))) * locals.var_de_1) + (assign8560_e10428 * locals.var_de_1_dn1)) * locals.var_de_2) + (assign8560_e10430 * locals.var_de_2_dn1)), (((((locals.var_cjcx02_t * (assign8560_e10427 * (locals.var_dcln2_dn3 * assign8560_e10425))) * locals.var_de_1) + (assign8560_e10428 * locals.var_de_1_dn3)) * locals.var_de_2) + (assign8560_e10430 * locals.var_de_2_dn3)), ((((((locals.var_cjcx02_t_dn4 * assign8560_e10427) + (locals.var_cjcx02_t * (assign8560_e10427 * (locals.var_dcln2_dn4 * assign8560_e10425)))) * locals.var_de_1) + (assign8560_e10428 * locals.var_de_1_dn4)) * locals.var_de_2) + (assign8560_e10430 * locals.var_de_2_dn4)), (((((locals.var_cjcx02_t * (assign8560_e10427 * (locals.var_dcln2_dn5 * assign8560_e10425))) * locals.var_de_1) + (assign8560_e10428 * locals.var_de_1_dn5)) * locals.var_de_2) + (assign8560_e10430 * locals.var_de_2_dn5)), (((((locals.var_cjcx02_t * (assign8560_e10427 * (locals.var_dcln2_dn7 * assign8560_e10425))) * locals.var_de_1) + (assign8560_e10428 * locals.var_de_1_dn7)) * locals.var_de_2) + (assign8560_e10430 * locals.var_de_2_dn7)), (((((locals.var_cjcx02_t * (assign8560_e10427 * (locals.var_dcln2_dn8 * assign8560_e10425))) * locals.var_de_1) + (assign8560_e10428 * locals.var_de_1_dn8)) * locals.var_de_2) + (assign8560_e10430 * locals.var_de_2_dn8)), (((((locals.var_cjcx02_t * (assign8560_e10427 * (locals.var_dcln2_dn9 * assign8560_e10425))) * locals.var_de_1) + (assign8560_e10428 * locals.var_de_1_dn9)) * locals.var_de_2) + (assign8560_e10430 * locals.var_de_2_dn9)),)
    } else {
        (locals.var_dc_j1, locals.var_dc_j1_dn0, locals.var_dc_j1_dn1, locals.var_dc_j1_dn3, locals.var_dc_j1_dn4, locals.var_dc_j1_dn5, locals.var_dc_j1_dn7, locals.var_dc_j1_dn8, locals.var_dc_j1_dn9,)
    }
};
        locals.var_dc_j1 = assign8560_e10434;
        locals.var_dc_j1_dn0 = assign8560_e10434_d_n0;
        locals.var_dc_j1_dn1 = assign8560_e10434_d_n1;
        locals.var_dc_j1_dn3 = assign8560_e10434_d_n3;
        locals.var_dc_j1_dn4 = assign8560_e10434_d_n4;
        locals.var_dc_j1_dn5 = assign8560_e10434_d_n5;
        locals.var_dc_j1_dn7 = assign8560_e10434_d_n7;
        locals.var_dc_j1_dn8 = assign8560_e10434_d_n8;
        locals.var_dc_j1_dn9 = assign8560_e10434_d_n9;
        locals.var_dc_j1_rv = 0.0;

        let (assign8570_e10450, assign8570_e10450_d_n0, assign8570_e10450_d_n1, assign8570_e10450_d_n3, assign8570_e10450_d_n4, assign8570_e10450_d_n5, assign8570_e10450_d_n7, assign8570_e10450_d_n8, assign8570_e10450_d_n9,) = {
    if ((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) {
        let assign8570_e10441: f64 = (-locals.var_dz_r);
        let assign8570_e10442: f64 = (locals.var_dcln1 * assign8570_e10441);
        let assign8570_e10443: f64 = (assign8570_e10442).exp();
        let assign8570_e10444: f64 = (locals.var_dc_c * assign8570_e10443);
        let assign8570_e10447: f64 = (1.0 - locals.var_de_2);
        let assign8570_e10448: f64 = (assign8570_e10444 * assign8570_e10447);
        (assign8570_e10448, (((locals.var_dc_c * (assign8570_e10443 * (locals.var_dcln1_dn0 * assign8570_e10441))) * assign8570_e10447) + (assign8570_e10444 * (-locals.var_de_2_dn0))), (((locals.var_dc_c * (assign8570_e10443 * (locals.var_dcln1_dn1 * assign8570_e10441))) * assign8570_e10447) + (assign8570_e10444 * (-locals.var_de_2_dn1))), (((locals.var_dc_c * (assign8570_e10443 * (locals.var_dcln1_dn3 * assign8570_e10441))) * assign8570_e10447) + (assign8570_e10444 * (-locals.var_de_2_dn3))), ((((locals.var_dc_c_dn4 * assign8570_e10443) + (locals.var_dc_c * (assign8570_e10443 * (locals.var_dcln1_dn4 * assign8570_e10441)))) * assign8570_e10447) + (assign8570_e10444 * (-locals.var_de_2_dn4))), (((locals.var_dc_c * (assign8570_e10443 * (locals.var_dcln1_dn5 * assign8570_e10441))) * assign8570_e10447) + (assign8570_e10444 * (-locals.var_de_2_dn5))), (((locals.var_dc_c * (assign8570_e10443 * (locals.var_dcln1_dn7 * assign8570_e10441))) * assign8570_e10447) + (assign8570_e10444 * (-locals.var_de_2_dn7))), (((locals.var_dc_c * (assign8570_e10443 * (locals.var_dcln1_dn8 * assign8570_e10441))) * assign8570_e10447) + (assign8570_e10444 * (-locals.var_de_2_dn8))), (((locals.var_dc_c * (assign8570_e10443 * (locals.var_dcln1_dn9 * assign8570_e10441))) * assign8570_e10447) + (assign8570_e10444 * (-locals.var_de_2_dn9))),)
    } else {
        (locals.var_dc_j2, locals.var_dc_j2_dn0, locals.var_dc_j2_dn1, locals.var_dc_j2_dn3, locals.var_dc_j2_dn4, locals.var_dc_j2_dn5, locals.var_dc_j2_dn7, locals.var_dc_j2_dn8, locals.var_dc_j2_dn9,)
    }
};
        locals.var_dc_j2 = assign8570_e10450;
        locals.var_dc_j2_dn0 = assign8570_e10450_d_n0;
        locals.var_dc_j2_dn1 = assign8570_e10450_d_n1;
        locals.var_dc_j2_dn3 = assign8570_e10450_d_n3;
        locals.var_dc_j2_dn4 = assign8570_e10450_d_n4;
        locals.var_dc_j2_dn5 = assign8570_e10450_d_n5;
        locals.var_dc_j2_dn7 = assign8570_e10450_d_n7;
        locals.var_dc_j2_dn8 = assign8570_e10450_d_n8;
        locals.var_dc_j2_dn9 = assign8570_e10450_d_n9;
        locals.var_dc_j2_rv = 0.0;

        let (assign8580_e10460, assign8580_e10460_d_n0, assign8580_e10460_d_n1, assign8580_e10460_d_n3, assign8580_e10460_d_n4, assign8580_e10460_d_n5, assign8580_e10460_d_n7, assign8580_e10460_d_n8, assign8580_e10460_d_n9,) = {
    if ((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) {
        let assign8580_e10457: f64 = (1.0 - locals.var_de_1);
        let assign8580_e10458: f64 = (locals.var_dc_max * assign8580_e10457);
        (assign8580_e10458, (locals.var_dc_max * (-locals.var_de_1_dn0)), (locals.var_dc_max * (-locals.var_de_1_dn1)), (locals.var_dc_max * (-locals.var_de_1_dn3)), ((locals.var_dc_max_dn4 * assign8580_e10457) + (locals.var_dc_max * (-locals.var_de_1_dn4))), (locals.var_dc_max * (-locals.var_de_1_dn5)), (locals.var_dc_max * (-locals.var_de_1_dn7)), (locals.var_dc_max * (-locals.var_de_1_dn8)), (locals.var_dc_max * (-locals.var_de_1_dn9)),)
    } else {
        (locals.var_dc_j3, locals.var_dc_j3_dn0, locals.var_dc_j3_dn1, locals.var_dc_j3_dn3, locals.var_dc_j3_dn4, locals.var_dc_j3_dn5, locals.var_dc_j3_dn7, locals.var_dc_j3_dn8, locals.var_dc_j3_dn9,)
    }
};
        locals.var_dc_j3 = assign8580_e10460;
        locals.var_dc_j3_dn0 = assign8580_e10460_d_n0;
        locals.var_dc_j3_dn1 = assign8580_e10460_d_n1;
        locals.var_dc_j3_dn3 = assign8580_e10460_d_n3;
        locals.var_dc_j3_dn4 = assign8580_e10460_d_n4;
        locals.var_dc_j3_dn5 = assign8580_e10460_d_n5;
        locals.var_dc_j3_dn7 = assign8580_e10460_d_n7;
        locals.var_dc_j3_dn8 = assign8580_e10460_d_n8;
        locals.var_dc_j3_dn9 = assign8580_e10460_d_n9;
        locals.var_dc_j3_rv = 0.0;

        let (assign8600_e10485, assign8600_e10485_d_n0, assign8600_e10485_d_n1, assign8600_e10485_d_n3, assign8600_e10485_d_n4, assign8600_e10485_d_n5, assign8600_e10485_d_n7, assign8600_e10485_d_n8, assign8600_e10485_d_n9,) = {
    if ((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) {
        let assign8600_e10478: f64 = (locals.var_dcln2 * locals.var_dz1);
        let assign8600_e10479: f64 = (assign8600_e10478).exp();
        let assign8600_e10480: f64 = (1.0 - assign8600_e10479);
        let assign8600_e10481: f64 = (locals.var_cjcx02_t * assign8600_e10480);
        let assign8600_e10483: f64 = (assign8600_e10481 / locals.var_dz1);
        (assign8600_e10483, ((locals.var_cjcx02_t * (-(assign8600_e10479 * (locals.var_dcln2_dn0 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cjcx02_t * (-(assign8600_e10479 * (locals.var_dcln2_dn1 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cjcx02_t * (-(assign8600_e10479 * (locals.var_dcln2_dn3 * locals.var_dz1)))) / locals.var_dz1), (((locals.var_cjcx02_t_dn4 * assign8600_e10480) + (locals.var_cjcx02_t * (-(assign8600_e10479 * (locals.var_dcln2_dn4 * locals.var_dz1))))) / locals.var_dz1), ((locals.var_cjcx02_t * (-(assign8600_e10479 * (locals.var_dcln2_dn5 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cjcx02_t * (-(assign8600_e10479 * (locals.var_dcln2_dn7 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cjcx02_t * (-(assign8600_e10479 * (locals.var_dcln2_dn8 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cjcx02_t * (-(assign8600_e10479 * (locals.var_dcln2_dn9 * locals.var_dz1)))) / locals.var_dz1),)
    } else {
        (locals.var_dq_j1, locals.var_dq_j1_dn0, locals.var_dq_j1_dn1, locals.var_dq_j1_dn3, locals.var_dq_j1_dn4, locals.var_dq_j1_dn5, locals.var_dq_j1_dn7, locals.var_dq_j1_dn8, locals.var_dq_j1_dn9,)
    }
};
        locals.var_dq_j1 = assign8600_e10485;
        locals.var_dq_j1_dn0 = assign8600_e10485_d_n0;
        locals.var_dq_j1_dn1 = assign8600_e10485_d_n1;
        locals.var_dq_j1_dn3 = assign8600_e10485_d_n3;
        locals.var_dq_j1_dn4 = assign8600_e10485_d_n4;
        locals.var_dq_j1_dn5 = assign8600_e10485_d_n5;
        locals.var_dq_j1_dn7 = assign8600_e10485_d_n7;
        locals.var_dq_j1_dn8 = assign8600_e10485_d_n8;
        locals.var_dq_j1_dn9 = assign8600_e10485_d_n9;
        locals.var_dq_j1_rv = 0.0;

        let (assign8610_e10500, assign8610_e10500_d_n0, assign8610_e10500_d_n1, assign8610_e10500_d_n3, assign8610_e10500_d_n4, assign8610_e10500_d_n5, assign8610_e10500_d_n7, assign8610_e10500_d_n8, assign8610_e10500_d_n9,) = {
    if ((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) {
        let assign8610_e10493: f64 = (locals.var_dcln1 * locals.var_dzr1);
        let assign8610_e10494: f64 = (assign8610_e10493).exp();
        let assign8610_e10495: f64 = (1.0 - assign8610_e10494);
        let assign8610_e10496: f64 = (locals.var_dc_c * assign8610_e10495);
        let assign8610_e10498: f64 = (assign8610_e10496 / locals.var_dzr1);
        (assign8610_e10498, ((locals.var_dc_c * (-(assign8610_e10494 * (locals.var_dcln1_dn0 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign8610_e10494 * (locals.var_dcln1_dn1 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign8610_e10494 * (locals.var_dcln1_dn3 * locals.var_dzr1)))) / locals.var_dzr1), (((locals.var_dc_c_dn4 * assign8610_e10495) + (locals.var_dc_c * (-(assign8610_e10494 * (locals.var_dcln1_dn4 * locals.var_dzr1))))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign8610_e10494 * (locals.var_dcln1_dn5 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign8610_e10494 * (locals.var_dcln1_dn7 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign8610_e10494 * (locals.var_dcln1_dn8 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign8610_e10494 * (locals.var_dcln1_dn9 * locals.var_dzr1)))) / locals.var_dzr1),)
    } else {
        (locals.var_dq_j2, locals.var_dq_j2_dn0, locals.var_dq_j2_dn1, locals.var_dq_j2_dn3, locals.var_dq_j2_dn4, locals.var_dq_j2_dn5, locals.var_dq_j2_dn7, locals.var_dq_j2_dn8, locals.var_dq_j2_dn9,)
    }
};
        locals.var_dq_j2 = assign8610_e10500;
        locals.var_dq_j2_dn0 = assign8610_e10500_d_n0;
        locals.var_dq_j2_dn1 = assign8610_e10500_d_n1;
        locals.var_dq_j2_dn3 = assign8610_e10500_d_n3;
        locals.var_dq_j2_dn4 = assign8610_e10500_d_n4;
        locals.var_dq_j2_dn5 = assign8610_e10500_d_n5;
        locals.var_dq_j2_dn7 = assign8610_e10500_d_n7;
        locals.var_dq_j2_dn8 = assign8610_e10500_d_n8;
        locals.var_dq_j2_dn9 = assign8610_e10500_d_n9;
        locals.var_dq_j2_rv = 0.0;

        let (assign8620_e10515, assign8620_e10515_d_n0, assign8620_e10515_d_n1, assign8620_e10515_d_n3, assign8620_e10515_d_n4, assign8620_e10515_d_n5, assign8620_e10515_d_n7, assign8620_e10515_d_n8, assign8620_e10515_d_n9,) = {
    if ((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) {
        let assign8620_e10508: f64 = (locals.var_dcln2 * locals.var_dzr1);
        let assign8620_e10509: f64 = (assign8620_e10508).exp();
        let assign8620_e10510: f64 = (1.0 - assign8620_e10509);
        let assign8620_e10511: f64 = (locals.var_dc_c * assign8620_e10510);
        let assign8620_e10513: f64 = (assign8620_e10511 / locals.var_dzr1);
        (assign8620_e10513, ((locals.var_dc_c * (-(assign8620_e10509 * (locals.var_dcln2_dn0 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign8620_e10509 * (locals.var_dcln2_dn1 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign8620_e10509 * (locals.var_dcln2_dn3 * locals.var_dzr1)))) / locals.var_dzr1), (((locals.var_dc_c_dn4 * assign8620_e10510) + (locals.var_dc_c * (-(assign8620_e10509 * (locals.var_dcln2_dn4 * locals.var_dzr1))))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign8620_e10509 * (locals.var_dcln2_dn5 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign8620_e10509 * (locals.var_dcln2_dn7 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign8620_e10509 * (locals.var_dcln2_dn8 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign8620_e10509 * (locals.var_dcln2_dn9 * locals.var_dzr1)))) / locals.var_dzr1),)
    } else {
        (locals.var_dq_j3, locals.var_dq_j3_dn0, locals.var_dq_j3_dn1, locals.var_dq_j3_dn3, locals.var_dq_j3_dn4, locals.var_dq_j3_dn5, locals.var_dq_j3_dn7, locals.var_dq_j3_dn8, locals.var_dq_j3_dn9,)
    }
};
        locals.var_dq_j3 = assign8620_e10515;
        locals.var_dq_j3_dn0 = assign8620_e10515_d_n0;
        locals.var_dq_j3_dn1 = assign8620_e10515_d_n1;
        locals.var_dq_j3_dn3 = assign8620_e10515_d_n3;
        locals.var_dq_j3_dn4 = assign8620_e10515_d_n4;
        locals.var_dq_j3_dn5 = assign8620_e10515_d_n5;
        locals.var_dq_j3_dn7 = assign8620_e10515_d_n7;
        locals.var_dq_j3_dn8 = assign8620_e10515_d_n8;
        locals.var_dq_j3_dn9 = assign8620_e10515_d_n9;
        locals.var_dq_j3_rv = 0.0;

        let (assign8630_e10531, assign8630_e10531_d_n0, assign8630_e10531_d_n1, assign8630_e10531_d_n3, assign8630_e10531_d_n4, assign8630_e10531_d_n5, assign8630_e10531_d_n6, assign8630_e10531_d_n7, assign8630_e10531_d_n8, assign8630_e10531_d_n9,) = {
    if ((locals.var_guard191 != 0.0) && (locals.var_guard192 != 0.0)) {
        let assign8630_e10521: f64 = (locals.var_dq_j1 + locals.var_dq_j2);
        let assign8630_e10523: f64 = (assign8630_e10521 - locals.var_dq_j3);
        let assign8630_e10525: f64 = (assign8630_e10523 * locals.var_vdcx_t);
        let assign8630_e10528: f64 = (locals.var_dc_max * locals.var_dv_j4);
        let assign8630_e10529: f64 = (assign8630_e10525 + assign8630_e10528);
        (assign8630_e10529, ((((locals.var_dq_j1_dn0 + locals.var_dq_j2_dn0) - locals.var_dq_j3_dn0) * locals.var_vdcx_t) + (locals.var_dc_max * locals.var_dv_j4_dn0)), ((((locals.var_dq_j1_dn1 + locals.var_dq_j2_dn1) - locals.var_dq_j3_dn1) * locals.var_vdcx_t) + (locals.var_dc_max * locals.var_dv_j4_dn1)), ((((locals.var_dq_j1_dn3 + locals.var_dq_j2_dn3) - locals.var_dq_j3_dn3) * locals.var_vdcx_t) + (locals.var_dc_max * locals.var_dv_j4_dn3)), (((((locals.var_dq_j1_dn4 + locals.var_dq_j2_dn4) - locals.var_dq_j3_dn4) * locals.var_vdcx_t) + (assign8630_e10523 * locals.var_vdcx_t_dn4)) + ((locals.var_dc_max_dn4 * locals.var_dv_j4) + (locals.var_dc_max * locals.var_dv_j4_dn4))), ((((locals.var_dq_j1_dn5 + locals.var_dq_j2_dn5) - locals.var_dq_j3_dn5) * locals.var_vdcx_t) + (locals.var_dc_max * locals.var_dv_j4_dn5)), 0.0, ((((locals.var_dq_j1_dn7 + locals.var_dq_j2_dn7) - locals.var_dq_j3_dn7) * locals.var_vdcx_t) + (locals.var_dc_max * locals.var_dv_j4_dn7)), ((((locals.var_dq_j1_dn8 + locals.var_dq_j2_dn8) - locals.var_dq_j3_dn8) * locals.var_vdcx_t) + (locals.var_dc_max * locals.var_dv_j4_dn8)), ((((locals.var_dq_j1_dn9 + locals.var_dq_j2_dn9) - locals.var_dq_j3_dn9) * locals.var_vdcx_t) + (locals.var_dc_max * locals.var_dv_j4_dn9)),)
    } else {
        (locals.var_qjcx0_t_p, locals.var_qjcx0_t_p_dn0, locals.var_qjcx0_t_p_dn1, locals.var_qjcx0_t_p_dn3, locals.var_qjcx0_t_p_dn4, locals.var_qjcx0_t_p_dn5, locals.var_qjcx0_t_p_dn6, locals.var_qjcx0_t_p_dn7, locals.var_qjcx0_t_p_dn8, locals.var_qjcx0_t_p_dn9,)
    }
};
        locals.var_qjcx0_t_p = assign8630_e10531;
        locals.var_qjcx0_t_p_dn0 = assign8630_e10531_d_n0;
        locals.var_qjcx0_t_p_dn1 = assign8630_e10531_d_n1;
        locals.var_qjcx0_t_p_dn3 = assign8630_e10531_d_n3;
        locals.var_qjcx0_t_p_dn4 = assign8630_e10531_d_n4;
        locals.var_qjcx0_t_p_dn5 = assign8630_e10531_d_n5;
        locals.var_qjcx0_t_p_dn6 = assign8630_e10531_d_n6;
        locals.var_qjcx0_t_p_dn7 = assign8630_e10531_d_n7;
        locals.var_qjcx0_t_p_dn8 = assign8630_e10531_d_n8;
        locals.var_qjcx0_t_p_dn9 = assign8630_e10531_d_n9;
        locals.var_qjcx0_t_p_rv = 0.0;

        let (assign8650_e10545, assign8650_e10545_d_n0, assign8650_e10545_d_n1, assign8650_e10545_d_n3, assign8650_e10545_d_n4, assign8650_e10545_d_n5, assign8650_e10545_d_n6, assign8650_e10545_d_n7, assign8650_e10545_d_n8, assign8650_e10545_d_n9,) = {
    if ((locals.var_guard191 != 0.0) && (locals.var_guard192 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qjcx0_t_p, locals.var_qjcx0_t_p_dn0, locals.var_qjcx0_t_p_dn1, locals.var_qjcx0_t_p_dn3, locals.var_qjcx0_t_p_dn4, locals.var_qjcx0_t_p_dn5, locals.var_qjcx0_t_p_dn6, locals.var_qjcx0_t_p_dn7, locals.var_qjcx0_t_p_dn8, locals.var_qjcx0_t_p_dn9,)
    }
};
        locals.var_qjcx0_t_p = assign8650_e10545;
        locals.var_qjcx0_t_p_dn0 = assign8650_e10545_d_n0;
        locals.var_qjcx0_t_p_dn1 = assign8650_e10545_d_n1;
        locals.var_qjcx0_t_p_dn3 = assign8650_e10545_d_n3;
        locals.var_qjcx0_t_p_dn4 = assign8650_e10545_d_n4;
        locals.var_qjcx0_t_p_dn5 = assign8650_e10545_d_n5;
        locals.var_qjcx0_t_p_dn6 = assign8650_e10545_d_n6;
        locals.var_qjcx0_t_p_dn7 = assign8650_e10545_d_n7;
        locals.var_qjcx0_t_p_dn8 = assign8650_e10545_d_n8;
        locals.var_qjcx0_t_p_dn9 = assign8650_e10545_d_n9;
        locals.var_qjcx0_t_p_rv = 0.0;

        let assign8660_e10548: f64 = if locals.var_cjcx02_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard195 = assign8660_e10548;
        locals.var_guard195_rv = 0.0;

        let (assign8670_e10564, assign8670_e10564_d_n4,) = {
    if ((locals.var_guard191 == 0.0) && (locals.var_guard195 != 0.0)) {
        let assign8670_e10556: f64 = (locals.var_ajcx_t).ln();
        let assign8670_e10557: f64 = (-assign8670_e10556);
        let assign8670_e10559: f64 = (assign8670_e10557 / p.p54);
        let assign8670_e10560: f64 = (assign8670_e10559).exp();
        let assign8670_e10561: f64 = (1.0 - assign8670_e10560);
        let assign8670_e10562: f64 = (locals.var_vdcx_t * assign8670_e10561);
        (assign8670_e10562, ((locals.var_vdcx_t_dn4 * assign8670_e10561) + (locals.var_vdcx_t * (-(assign8670_e10560 * ((-(locals.var_ajcx_t_dn4 / locals.var_ajcx_t)) / p.p54))))),)
    } else {
        (locals.var_dfv_f, locals.var_dfv_f_dn4,)
    }
};
        locals.var_dfv_f = assign8670_e10564;
        locals.var_dfv_f_dn4 = assign8670_e10564_d_n4;
        locals.var_dfv_f_rv = 0.0;

        let (assign8680_e10575, assign8680_e10575_d_n0, assign8680_e10575_d_n1, assign8680_e10575_d_n3, assign8680_e10575_d_n4, assign8680_e10575_d_n5, assign8680_e10575_d_n6, assign8680_e10575_d_n7, assign8680_e10575_d_n8, assign8680_e10575_d_n9,) = {
    if ((locals.var_guard191 == 0.0) && (locals.var_guard195 != 0.0)) {
        let assign8680_e10571: f64 = (locals.var_dfv_f - locals.var_vbpci);
        let assign8680_e10573: f64 = (assign8680_e10571 * locals.var_ovt);
        (assign8680_e10573, 0.0, 0.0, 0.0, ((locals.var_dfv_f_dn4 * locals.var_ovt) + (assign8680_e10571 * locals.var_ovt_dn4)), ((-locals.var_vbpci_dn5) * locals.var_ovt), 0.0, ((-locals.var_vbpci_dn7) * locals.var_ovt), 0.0, 0.0,)
    } else {
        (locals.var_dfx, locals.var_dfx_dn0, locals.var_dfx_dn1, locals.var_dfx_dn3, locals.var_dfx_dn4, locals.var_dfx_dn5, locals.var_dfx_dn6, locals.var_dfx_dn7, locals.var_dfx_dn8, locals.var_dfx_dn9,)
    }
};
        locals.var_dfx = assign8680_e10575;
        locals.var_dfx_dn0 = assign8680_e10575_d_n0;
        locals.var_dfx_dn1 = assign8680_e10575_d_n1;
        locals.var_dfx_dn3 = assign8680_e10575_d_n3;
        locals.var_dfx_dn4 = assign8680_e10575_d_n4;
        locals.var_dfx_dn5 = assign8680_e10575_d_n5;
        locals.var_dfx_dn6 = assign8680_e10575_d_n6;
        locals.var_dfx_dn7 = assign8680_e10575_d_n7;
        locals.var_dfx_dn8 = assign8680_e10575_d_n8;
        locals.var_dfx_dn9 = assign8680_e10575_d_n9;
        locals.var_dfx_rv = 0.0;

        let (assign8690_e10587, assign8690_e10587_d_n0, assign8690_e10587_d_n1, assign8690_e10587_d_n3, assign8690_e10587_d_n4, assign8690_e10587_d_n5, assign8690_e10587_d_n6, assign8690_e10587_d_n7, assign8690_e10587_d_n8, assign8690_e10587_d_n9,) = {
    if ((locals.var_guard191 == 0.0) && (locals.var_guard195 != 0.0)) {
        let assign8690_e10582: f64 = (locals.var_dfx * locals.var_dfx);
        let assign8690_e10584: f64 = (assign8690_e10582 + 1.921812);
        let assign8690_e10585: f64 = (assign8690_e10584).sqrt();
        (assign8690_e10585, (((locals.var_dfx_dn0 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn0)) / (2.0 * assign8690_e10585)), (((locals.var_dfx_dn1 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn1)) / (2.0 * assign8690_e10585)), (((locals.var_dfx_dn3 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn3)) / (2.0 * assign8690_e10585)), (((locals.var_dfx_dn4 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn4)) / (2.0 * assign8690_e10585)), (((locals.var_dfx_dn5 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn5)) / (2.0 * assign8690_e10585)), (((locals.var_dfx_dn6 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn6)) / (2.0 * assign8690_e10585)), (((locals.var_dfx_dn7 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn7)) / (2.0 * assign8690_e10585)), (((locals.var_dfx_dn8 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn8)) / (2.0 * assign8690_e10585)), (((locals.var_dfx_dn9 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn9)) / (2.0 * assign8690_e10585)),)
    } else {
        (locals.var_dfs_q, locals.var_dfs_q_dn0, locals.var_dfs_q_dn1, locals.var_dfs_q_dn3, locals.var_dfs_q_dn4, locals.var_dfs_q_dn5, locals.var_dfs_q_dn6, locals.var_dfs_q_dn7, locals.var_dfs_q_dn8, locals.var_dfs_q_dn9,)
    }
};
        locals.var_dfs_q = assign8690_e10587;
        locals.var_dfs_q_dn0 = assign8690_e10587_d_n0;
        locals.var_dfs_q_dn1 = assign8690_e10587_d_n1;
        locals.var_dfs_q_dn3 = assign8690_e10587_d_n3;
        locals.var_dfs_q_dn4 = assign8690_e10587_d_n4;
        locals.var_dfs_q_dn5 = assign8690_e10587_d_n5;
        locals.var_dfs_q_dn6 = assign8690_e10587_d_n6;
        locals.var_dfs_q_dn7 = assign8690_e10587_d_n7;
        locals.var_dfs_q_dn8 = assign8690_e10587_d_n8;
        locals.var_dfs_q_dn9 = assign8690_e10587_d_n9;
        locals.var_dfs_q_rv = 0.0;

        let (assign8700_e10598, assign8700_e10598_d_n0, assign8700_e10598_d_n1, assign8700_e10598_d_n3, assign8700_e10598_d_n4, assign8700_e10598_d_n5, assign8700_e10598_d_n6, assign8700_e10598_d_n7, assign8700_e10598_d_n8, assign8700_e10598_d_n9,) = {
    if ((locals.var_guard191 == 0.0) && (locals.var_guard195 != 0.0)) {
        let assign8700_e10594: f64 = (locals.var_dfx + locals.var_dfs_q);
        let assign8700_e10596: f64 = (assign8700_e10594 * 0.5);
        (assign8700_e10596, ((locals.var_dfx_dn0 + locals.var_dfs_q_dn0) * 0.5), ((locals.var_dfx_dn1 + locals.var_dfs_q_dn1) * 0.5), ((locals.var_dfx_dn3 + locals.var_dfs_q_dn3) * 0.5), ((locals.var_dfx_dn4 + locals.var_dfs_q_dn4) * 0.5), ((locals.var_dfx_dn5 + locals.var_dfs_q_dn5) * 0.5), ((locals.var_dfx_dn6 + locals.var_dfs_q_dn6) * 0.5), ((locals.var_dfx_dn7 + locals.var_dfs_q_dn7) * 0.5), ((locals.var_dfx_dn8 + locals.var_dfs_q_dn8) * 0.5), ((locals.var_dfx_dn9 + locals.var_dfs_q_dn9) * 0.5),)
    } else {
        (locals.var_dfs_q2, locals.var_dfs_q2_dn0, locals.var_dfs_q2_dn1, locals.var_dfs_q2_dn3, locals.var_dfs_q2_dn4, locals.var_dfs_q2_dn5, locals.var_dfs_q2_dn6, locals.var_dfs_q2_dn7, locals.var_dfs_q2_dn8, locals.var_dfs_q2_dn9,)
    }
};
        locals.var_dfs_q2 = assign8700_e10598;
        locals.var_dfs_q2_dn0 = assign8700_e10598_d_n0;
        locals.var_dfs_q2_dn1 = assign8700_e10598_d_n1;
        locals.var_dfs_q2_dn3 = assign8700_e10598_d_n3;
        locals.var_dfs_q2_dn4 = assign8700_e10598_d_n4;
        locals.var_dfs_q2_dn5 = assign8700_e10598_d_n5;
        locals.var_dfs_q2_dn6 = assign8700_e10598_d_n6;
        locals.var_dfs_q2_dn7 = assign8700_e10598_d_n7;
        locals.var_dfs_q2_dn8 = assign8700_e10598_d_n8;
        locals.var_dfs_q2_dn9 = assign8700_e10598_d_n9;
        locals.var_dfs_q2_rv = 0.0;

        let (assign8710_e10609, assign8710_e10609_d_n0, assign8710_e10609_d_n1, assign8710_e10609_d_n3, assign8710_e10609_d_n4, assign8710_e10609_d_n5, assign8710_e10609_d_n6, assign8710_e10609_d_n7, assign8710_e10609_d_n8, assign8710_e10609_d_n9,) = {
    if ((locals.var_guard191 == 0.0) && (locals.var_guard195 != 0.0)) {
        let assign8710_e10606: f64 = (locals.var_vt * locals.var_dfs_q2);
        let assign8710_e10607: f64 = (locals.var_dfv_f - assign8710_e10606);
        (assign8710_e10607, (-(locals.var_vt * locals.var_dfs_q2_dn0)), (-(locals.var_vt * locals.var_dfs_q2_dn1)), (-(locals.var_vt * locals.var_dfs_q2_dn3)), (locals.var_dfv_f_dn4 - ((locals.var_vt_dn4 * locals.var_dfs_q2) + (locals.var_vt * locals.var_dfs_q2_dn4))), (-(locals.var_vt * locals.var_dfs_q2_dn5)), (-(locals.var_vt * locals.var_dfs_q2_dn6)), (-(locals.var_vt * locals.var_dfs_q2_dn7)), (-(locals.var_vt * locals.var_dfs_q2_dn8)), (-(locals.var_vt * locals.var_dfs_q2_dn9)),)
    } else {
        (locals.var_dfv_j, locals.var_dfv_j_dn0, locals.var_dfv_j_dn1, locals.var_dfv_j_dn3, locals.var_dfv_j_dn4, locals.var_dfv_j_dn5, locals.var_dfv_j_dn6, locals.var_dfv_j_dn7, locals.var_dfv_j_dn8, locals.var_dfv_j_dn9,)
    }
};
        locals.var_dfv_j = assign8710_e10609;
        locals.var_dfv_j_dn0 = assign8710_e10609_d_n0;
        locals.var_dfv_j_dn1 = assign8710_e10609_d_n1;
        locals.var_dfv_j_dn3 = assign8710_e10609_d_n3;
        locals.var_dfv_j_dn4 = assign8710_e10609_d_n4;
        locals.var_dfv_j_dn5 = assign8710_e10609_d_n5;
        locals.var_dfv_j_dn6 = assign8710_e10609_d_n6;
        locals.var_dfv_j_dn7 = assign8710_e10609_d_n7;
        locals.var_dfv_j_dn8 = assign8710_e10609_d_n8;
        locals.var_dfv_j_dn9 = assign8710_e10609_d_n9;
        locals.var_dfv_j_rv = 0.0;

        let (assign8720_e10618, assign8720_e10618_d_n0, assign8720_e10618_d_n1, assign8720_e10618_d_n3, assign8720_e10618_d_n4, assign8720_e10618_d_n5, assign8720_e10618_d_n6, assign8720_e10618_d_n7, assign8720_e10618_d_n8, assign8720_e10618_d_n9,) = {
    if ((locals.var_guard191 == 0.0) && (locals.var_guard195 != 0.0)) {
        let assign8720_e10616: f64 = (locals.var_dfs_q2 / locals.var_dfs_q);
        (assign8720_e10616, (((locals.var_dfs_q2_dn0 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn0)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn1 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn1)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn3 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn3)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn4 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn4)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn5 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn5)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn6 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn6)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn7 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn7)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn8 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn8)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn9 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn9)) / (locals.var_dfs_q * locals.var_dfs_q)),)
    } else {
        (locals.var_dfdvj_dv, locals.var_dfdvj_dv_dn0, locals.var_dfdvj_dv_dn1, locals.var_dfdvj_dv_dn3, locals.var_dfdvj_dv_dn4, locals.var_dfdvj_dv_dn5, locals.var_dfdvj_dv_dn6, locals.var_dfdvj_dv_dn7, locals.var_dfdvj_dv_dn8, locals.var_dfdvj_dv_dn9,)
    }
};
        locals.var_dfdvj_dv = assign8720_e10618;
        locals.var_dfdvj_dv_dn0 = assign8720_e10618_d_n0;
        locals.var_dfdvj_dv_dn1 = assign8720_e10618_d_n1;
        locals.var_dfdvj_dv_dn3 = assign8720_e10618_d_n3;
        locals.var_dfdvj_dv_dn4 = assign8720_e10618_d_n4;
        locals.var_dfdvj_dv_dn5 = assign8720_e10618_d_n5;
        locals.var_dfdvj_dv_dn6 = assign8720_e10618_d_n6;
        locals.var_dfdvj_dv_dn7 = assign8720_e10618_d_n7;
        locals.var_dfdvj_dv_dn8 = assign8720_e10618_d_n8;
        locals.var_dfdvj_dv_dn9 = assign8720_e10618_d_n9;
        locals.var_dfdvj_dv_rv = 0.0;

        let (assign8730_e10630, assign8730_e10630_d_n0, assign8730_e10630_d_n1, assign8730_e10630_d_n3, assign8730_e10630_d_n4, assign8730_e10630_d_n5, assign8730_e10630_d_n6, assign8730_e10630_d_n7, assign8730_e10630_d_n8, assign8730_e10630_d_n9,) = {
    if ((locals.var_guard191 == 0.0) && (locals.var_guard195 != 0.0)) {
        let assign8730_e10626: f64 = (locals.var_dfv_j / locals.var_vdcx_t);
        let assign8730_e10627: f64 = (1.0 - assign8730_e10626);
        let assign8730_e10628: f64 = (assign8730_e10627).ln();
        (assign8730_e10628, ((-(locals.var_dfv_j_dn0 / locals.var_vdcx_t)) / assign8730_e10627), ((-(locals.var_dfv_j_dn1 / locals.var_vdcx_t)) / assign8730_e10627), ((-(locals.var_dfv_j_dn3 / locals.var_vdcx_t)) / assign8730_e10627), ((-(((locals.var_dfv_j_dn4 * locals.var_vdcx_t) - (locals.var_dfv_j * locals.var_vdcx_t_dn4)) / (locals.var_vdcx_t * locals.var_vdcx_t))) / assign8730_e10627), ((-(locals.var_dfv_j_dn5 / locals.var_vdcx_t)) / assign8730_e10627), ((-(locals.var_dfv_j_dn6 / locals.var_vdcx_t)) / assign8730_e10627), ((-(locals.var_dfv_j_dn7 / locals.var_vdcx_t)) / assign8730_e10627), ((-(locals.var_dfv_j_dn8 / locals.var_vdcx_t)) / assign8730_e10627), ((-(locals.var_dfv_j_dn9 / locals.var_vdcx_t)) / assign8730_e10627),)
    } else {
        (locals.var_dfb, locals.var_dfb_dn0, locals.var_dfb_dn1, locals.var_dfb_dn3, locals.var_dfb_dn4, locals.var_dfb_dn5, locals.var_dfb_dn6, locals.var_dfb_dn7, locals.var_dfb_dn8, locals.var_dfb_dn9,)
    }
};
        locals.var_dfb = assign8730_e10630;
        locals.var_dfb_dn0 = assign8730_e10630_d_n0;
        locals.var_dfb_dn1 = assign8730_e10630_d_n1;
        locals.var_dfb_dn3 = assign8730_e10630_d_n3;
        locals.var_dfb_dn4 = assign8730_e10630_d_n4;
        locals.var_dfb_dn5 = assign8730_e10630_d_n5;
        locals.var_dfb_dn6 = assign8730_e10630_d_n6;
        locals.var_dfb_dn7 = assign8730_e10630_d_n7;
        locals.var_dfb_dn8 = assign8730_e10630_d_n8;
        locals.var_dfb_dn9 = assign8730_e10630_d_n9;
        locals.var_dfb_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_23(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign8740_e10643, assign8740_e10643_d_n0, assign8740_e10643_d_n1, assign8740_e10643_d_n3, assign8740_e10643_d_n4, assign8740_e10643_d_n5, assign8740_e10643_d_n6, assign8740_e10643_d_n7, assign8740_e10643_d_n8, assign8740_e10643_d_n9,) = {
    if ((locals.var_guard191 == 0.0) && (locals.var_guard195 != 0.0)) {
        let assign8740_e10636: f64 = (-p.p54);
        let assign8740_e10638: f64 = (assign8740_e10636 * locals.var_dfb);
        let assign8740_e10639: f64 = (assign8740_e10638).exp();
        let assign8740_e10641: f64 = (assign8740_e10639 * locals.var_dfdvj_dv);
        (assign8740_e10641, (((assign8740_e10639 * (assign8740_e10636 * locals.var_dfb_dn0)) * locals.var_dfdvj_dv) + (assign8740_e10639 * locals.var_dfdvj_dv_dn0)), (((assign8740_e10639 * (assign8740_e10636 * locals.var_dfb_dn1)) * locals.var_dfdvj_dv) + (assign8740_e10639 * locals.var_dfdvj_dv_dn1)), (((assign8740_e10639 * (assign8740_e10636 * locals.var_dfb_dn3)) * locals.var_dfdvj_dv) + (assign8740_e10639 * locals.var_dfdvj_dv_dn3)), (((assign8740_e10639 * (assign8740_e10636 * locals.var_dfb_dn4)) * locals.var_dfdvj_dv) + (assign8740_e10639 * locals.var_dfdvj_dv_dn4)), (((assign8740_e10639 * (assign8740_e10636 * locals.var_dfb_dn5)) * locals.var_dfdvj_dv) + (assign8740_e10639 * locals.var_dfdvj_dv_dn5)), (((assign8740_e10639 * (assign8740_e10636 * locals.var_dfb_dn6)) * locals.var_dfdvj_dv) + (assign8740_e10639 * locals.var_dfdvj_dv_dn6)), (((assign8740_e10639 * (assign8740_e10636 * locals.var_dfb_dn7)) * locals.var_dfdvj_dv) + (assign8740_e10639 * locals.var_dfdvj_dv_dn7)), (((assign8740_e10639 * (assign8740_e10636 * locals.var_dfb_dn8)) * locals.var_dfdvj_dv) + (assign8740_e10639 * locals.var_dfdvj_dv_dn8)), (((assign8740_e10639 * (assign8740_e10636 * locals.var_dfb_dn9)) * locals.var_dfdvj_dv) + (assign8740_e10639 * locals.var_dfdvj_dv_dn9)),)
    } else {
        (locals.var_dfc_j1, locals.var_dfc_j1_dn0, locals.var_dfc_j1_dn1, locals.var_dfc_j1_dn3, locals.var_dfc_j1_dn4, locals.var_dfc_j1_dn5, locals.var_dfc_j1_dn6, locals.var_dfc_j1_dn7, locals.var_dfc_j1_dn8, locals.var_dfc_j1_dn9,)
    }
};
        locals.var_dfc_j1 = assign8740_e10643;
        locals.var_dfc_j1_dn0 = assign8740_e10643_d_n0;
        locals.var_dfc_j1_dn1 = assign8740_e10643_d_n1;
        locals.var_dfc_j1_dn3 = assign8740_e10643_d_n3;
        locals.var_dfc_j1_dn4 = assign8740_e10643_d_n4;
        locals.var_dfc_j1_dn5 = assign8740_e10643_d_n5;
        locals.var_dfc_j1_dn6 = assign8740_e10643_d_n6;
        locals.var_dfc_j1_dn7 = assign8740_e10643_d_n7;
        locals.var_dfc_j1_dn8 = assign8740_e10643_d_n8;
        locals.var_dfc_j1_dn9 = assign8740_e10643_d_n9;
        locals.var_dfc_j1_rv = 0.0;

        let (assign8760_e10678, assign8760_e10678_d_n0, assign8760_e10678_d_n1, assign8760_e10678_d_n3, assign8760_e10678_d_n4, assign8760_e10678_d_n5, assign8760_e10678_d_n6, assign8760_e10678_d_n7, assign8760_e10678_d_n8, assign8760_e10678_d_n9,) = {
    if ((locals.var_guard191 == 0.0) && (locals.var_guard195 != 0.0)) {
        let assign8760_e10668: f64 = (1.0 - p.p54);
        let assign8760_e10669: f64 = (locals.var_dfb * assign8760_e10668);
        let assign8760_e10670: f64 = (assign8760_e10669).exp();
        let assign8760_e10671: f64 = (1.0 - assign8760_e10670);
        let assign8760_e10672: f64 = (locals.var_vdcx_t * assign8760_e10671);
        let assign8760_e10675: f64 = (1.0 - p.p54);
        let assign8760_e10676: f64 = (assign8760_e10672 / assign8760_e10675);
        (assign8760_e10676, ((locals.var_vdcx_t * (-(assign8760_e10670 * (locals.var_dfb_dn0 * assign8760_e10668)))) / assign8760_e10675), ((locals.var_vdcx_t * (-(assign8760_e10670 * (locals.var_dfb_dn1 * assign8760_e10668)))) / assign8760_e10675), ((locals.var_vdcx_t * (-(assign8760_e10670 * (locals.var_dfb_dn3 * assign8760_e10668)))) / assign8760_e10675), (((locals.var_vdcx_t_dn4 * assign8760_e10671) + (locals.var_vdcx_t * (-(assign8760_e10670 * (locals.var_dfb_dn4 * assign8760_e10668))))) / assign8760_e10675), ((locals.var_vdcx_t * (-(assign8760_e10670 * (locals.var_dfb_dn5 * assign8760_e10668)))) / assign8760_e10675), ((locals.var_vdcx_t * (-(assign8760_e10670 * (locals.var_dfb_dn6 * assign8760_e10668)))) / assign8760_e10675), ((locals.var_vdcx_t * (-(assign8760_e10670 * (locals.var_dfb_dn7 * assign8760_e10668)))) / assign8760_e10675), ((locals.var_vdcx_t * (-(assign8760_e10670 * (locals.var_dfb_dn8 * assign8760_e10668)))) / assign8760_e10675), ((locals.var_vdcx_t * (-(assign8760_e10670 * (locals.var_dfb_dn9 * assign8760_e10668)))) / assign8760_e10675),)
    } else {
        (locals.var_dfq_j1, locals.var_dfq_j1_dn0, locals.var_dfq_j1_dn1, locals.var_dfq_j1_dn3, locals.var_dfq_j1_dn4, locals.var_dfq_j1_dn5, locals.var_dfq_j1_dn6, locals.var_dfq_j1_dn7, locals.var_dfq_j1_dn8, locals.var_dfq_j1_dn9,)
    }
};
        locals.var_dfq_j1 = assign8760_e10678;
        locals.var_dfq_j1_dn0 = assign8760_e10678_d_n0;
        locals.var_dfq_j1_dn1 = assign8760_e10678_d_n1;
        locals.var_dfq_j1_dn3 = assign8760_e10678_d_n3;
        locals.var_dfq_j1_dn4 = assign8760_e10678_d_n4;
        locals.var_dfq_j1_dn5 = assign8760_e10678_d_n5;
        locals.var_dfq_j1_dn6 = assign8760_e10678_d_n6;
        locals.var_dfq_j1_dn7 = assign8760_e10678_d_n7;
        locals.var_dfq_j1_dn8 = assign8760_e10678_d_n8;
        locals.var_dfq_j1_dn9 = assign8760_e10678_d_n9;
        locals.var_dfq_j1_rv = 0.0;

        let (assign8770_e10693, assign8770_e10693_d_n0, assign8770_e10693_d_n1, assign8770_e10693_d_n3, assign8770_e10693_d_n4, assign8770_e10693_d_n5, assign8770_e10693_d_n6, assign8770_e10693_d_n7, assign8770_e10693_d_n8, assign8770_e10693_d_n9,) = {
    if ((locals.var_guard191 == 0.0) && (locals.var_guard195 != 0.0)) {
        let assign8770_e10688: f64 = (locals.var_vbpci - locals.var_dfv_j);
        let assign8770_e10689: f64 = (locals.var_ajcx_t * assign8770_e10688);
        let assign8770_e10690: f64 = (locals.var_dfq_j1 + assign8770_e10689);
        let assign8770_e10691: f64 = (locals.var_cjcx02_t * assign8770_e10690);
        (assign8770_e10691, (locals.var_cjcx02_t * (locals.var_dfq_j1_dn0 + (locals.var_ajcx_t * (-locals.var_dfv_j_dn0)))), (locals.var_cjcx02_t * (locals.var_dfq_j1_dn1 + (locals.var_ajcx_t * (-locals.var_dfv_j_dn1)))), (locals.var_cjcx02_t * (locals.var_dfq_j1_dn3 + (locals.var_ajcx_t * (-locals.var_dfv_j_dn3)))), ((locals.var_cjcx02_t_dn4 * assign8770_e10690) + (locals.var_cjcx02_t * (locals.var_dfq_j1_dn4 + ((locals.var_ajcx_t_dn4 * assign8770_e10688) + (locals.var_ajcx_t * (-locals.var_dfv_j_dn4)))))), (locals.var_cjcx02_t * (locals.var_dfq_j1_dn5 + (locals.var_ajcx_t * (locals.var_vbpci_dn5 - locals.var_dfv_j_dn5)))), (locals.var_cjcx02_t * (locals.var_dfq_j1_dn6 + (locals.var_ajcx_t * (-locals.var_dfv_j_dn6)))), (locals.var_cjcx02_t * (locals.var_dfq_j1_dn7 + (locals.var_ajcx_t * (locals.var_vbpci_dn7 - locals.var_dfv_j_dn7)))), (locals.var_cjcx02_t * (locals.var_dfq_j1_dn8 + (locals.var_ajcx_t * (-locals.var_dfv_j_dn8)))), (locals.var_cjcx02_t * (locals.var_dfq_j1_dn9 + (locals.var_ajcx_t * (-locals.var_dfv_j_dn9)))),)
    } else {
        (locals.var_qjcx0_t_p, locals.var_qjcx0_t_p_dn0, locals.var_qjcx0_t_p_dn1, locals.var_qjcx0_t_p_dn3, locals.var_qjcx0_t_p_dn4, locals.var_qjcx0_t_p_dn5, locals.var_qjcx0_t_p_dn6, locals.var_qjcx0_t_p_dn7, locals.var_qjcx0_t_p_dn8, locals.var_qjcx0_t_p_dn9,)
    }
};
        locals.var_qjcx0_t_p = assign8770_e10693;
        locals.var_qjcx0_t_p_dn0 = assign8770_e10693_d_n0;
        locals.var_qjcx0_t_p_dn1 = assign8770_e10693_d_n1;
        locals.var_qjcx0_t_p_dn3 = assign8770_e10693_d_n3;
        locals.var_qjcx0_t_p_dn4 = assign8770_e10693_d_n4;
        locals.var_qjcx0_t_p_dn5 = assign8770_e10693_d_n5;
        locals.var_qjcx0_t_p_dn6 = assign8770_e10693_d_n6;
        locals.var_qjcx0_t_p_dn7 = assign8770_e10693_d_n7;
        locals.var_qjcx0_t_p_dn8 = assign8770_e10693_d_n8;
        locals.var_qjcx0_t_p_dn9 = assign8770_e10693_d_n9;
        locals.var_qjcx0_t_p_rv = 0.0;

        let (assign8790_e10709, assign8790_e10709_d_n0, assign8790_e10709_d_n1, assign8790_e10709_d_n3, assign8790_e10709_d_n4, assign8790_e10709_d_n5, assign8790_e10709_d_n6, assign8790_e10709_d_n7, assign8790_e10709_d_n8, assign8790_e10709_d_n9,) = {
    if ((locals.var_guard191 == 0.0) && (locals.var_guard195 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qjcx0_t_p, locals.var_qjcx0_t_p_dn0, locals.var_qjcx0_t_p_dn1, locals.var_qjcx0_t_p_dn3, locals.var_qjcx0_t_p_dn4, locals.var_qjcx0_t_p_dn5, locals.var_qjcx0_t_p_dn6, locals.var_qjcx0_t_p_dn7, locals.var_qjcx0_t_p_dn8, locals.var_qjcx0_t_p_dn9,)
    }
};
        locals.var_qjcx0_t_p = assign8790_e10709;
        locals.var_qjcx0_t_p_dn0 = assign8790_e10709_d_n0;
        locals.var_qjcx0_t_p_dn1 = assign8790_e10709_d_n1;
        locals.var_qjcx0_t_p_dn3 = assign8790_e10709_d_n3;
        locals.var_qjcx0_t_p_dn4 = assign8790_e10709_d_n4;
        locals.var_qjcx0_t_p_dn5 = assign8790_e10709_d_n5;
        locals.var_qjcx0_t_p_dn6 = assign8790_e10709_d_n6;
        locals.var_qjcx0_t_p_dn7 = assign8790_e10709_d_n7;
        locals.var_qjcx0_t_p_dn8 = assign8790_e10709_d_n8;
        locals.var_qjcx0_t_p_dn9 = assign8790_e10709_d_n9;
        locals.var_qjcx0_t_p_rv = 0.0;

        let assign8800_e10712: f64 = if p.p25 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard196 = assign8800_e10712;
        locals.var_guard196_rv = 0.0;

        let (assign8810_e10720, assign8810_e10720_d_n4, assign8810_e10720_d_n5, assign8810_e10720_d_n6, assign8810_e10720_d_n7, assign8810_e10720_d_n8, assign8810_e10720_d_n9,) = {
    if (locals.var_guard196 != 0.0) {
        let assign8810_e10717: f64 = (p.p26 * locals.var_vt);
        let assign8810_e10718: f64 = (locals.var_vbpci / assign8810_e10717);
        (assign8810_e10718, (-((locals.var_vbpci * (p.p26 * locals.var_vt_dn4)) / (assign8810_e10717 * assign8810_e10717))), (locals.var_vbpci_dn5 / assign8810_e10717), 0.0, (locals.var_vbpci_dn7 / assign8810_e10717), 0.0, 0.0,)
    } else {
        (locals.var_dio_y, locals.var_dio_y_dn4, locals.var_dio_y_dn5, locals.var_dio_y_dn6, locals.var_dio_y_dn7, locals.var_dio_y_dn8, locals.var_dio_y_dn9,)
    }
};
        locals.var_dio_y = assign8810_e10720;
        locals.var_dio_y_dn4 = assign8810_e10720_d_n4;
        locals.var_dio_y_dn5 = assign8810_e10720_d_n5;
        locals.var_dio_y_dn6 = assign8810_e10720_d_n6;
        locals.var_dio_y_dn7 = assign8810_e10720_d_n7;
        locals.var_dio_y_dn8 = assign8810_e10720_d_n8;
        locals.var_dio_y_dn9 = assign8810_e10720_d_n9;
        locals.var_dio_y_rv = 0.0;

        let assign8820_e10723: f64 = if locals.var_dio_y > 80.0 { 1.0 } else { 0.0 };
        locals.var_guard197 = assign8820_e10723;
        locals.var_guard197_rv = 0.0;

        let (assign8830_e10733, assign8830_e10733_d_n4, assign8830_e10733_d_n5, assign8830_e10733_d_n6, assign8830_e10733_d_n7, assign8830_e10733_d_n8, assign8830_e10733_d_n9,) = {
    if ((locals.var_guard196 != 0.0) && (locals.var_guard197 != 0.0)) {
        let assign8830_e10730: f64 = (locals.var_dio_y - 80.0);
        let assign8830_e10731: f64 = (1.0 + assign8830_e10730);
        (assign8830_e10731, locals.var_dio_y_dn4, locals.var_dio_y_dn5, locals.var_dio_y_dn6, locals.var_dio_y_dn7, locals.var_dio_y_dn8, locals.var_dio_y_dn9,)
    } else {
        (locals.var_dio_le, locals.var_dio_le_dn4, locals.var_dio_le_dn5, locals.var_dio_le_dn6, locals.var_dio_le_dn7, locals.var_dio_le_dn8, locals.var_dio_le_dn9,)
    }
};
        locals.var_dio_le = assign8830_e10733;
        locals.var_dio_le_dn4 = assign8830_e10733_d_n4;
        locals.var_dio_le_dn5 = assign8830_e10733_d_n5;
        locals.var_dio_le_dn6 = assign8830_e10733_d_n6;
        locals.var_dio_le_dn7 = assign8830_e10733_d_n7;
        locals.var_dio_le_dn8 = assign8830_e10733_d_n8;
        locals.var_dio_le_dn9 = assign8830_e10733_d_n9;
        locals.var_dio_le_rv = 0.0;

        let (assign8840_e10739, assign8840_e10739_d_n4, assign8840_e10739_d_n5, assign8840_e10739_d_n6, assign8840_e10739_d_n7, assign8840_e10739_d_n8, assign8840_e10739_d_n9,) = {
    if ((locals.var_guard196 != 0.0) && (locals.var_guard197 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dio_y, locals.var_dio_y_dn4, locals.var_dio_y_dn5, locals.var_dio_y_dn6, locals.var_dio_y_dn7, locals.var_dio_y_dn8, locals.var_dio_y_dn9,)
    }
};
        locals.var_dio_y = assign8840_e10739;
        locals.var_dio_y_dn4 = assign8840_e10739_d_n4;
        locals.var_dio_y_dn5 = assign8840_e10739_d_n5;
        locals.var_dio_y_dn6 = assign8840_e10739_d_n6;
        locals.var_dio_y_dn7 = assign8840_e10739_d_n7;
        locals.var_dio_y_dn8 = assign8840_e10739_d_n8;
        locals.var_dio_y_dn9 = assign8840_e10739_d_n9;
        locals.var_dio_y_rv = 0.0;

        let (assign8850_e10746, assign8850_e10746_d_n4, assign8850_e10746_d_n5, assign8850_e10746_d_n6, assign8850_e10746_d_n7, assign8850_e10746_d_n8, assign8850_e10746_d_n9,) = {
    if ((locals.var_guard196 != 0.0) && (locals.var_guard197 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dio_le, locals.var_dio_le_dn4, locals.var_dio_le_dn5, locals.var_dio_le_dn6, locals.var_dio_le_dn7, locals.var_dio_le_dn8, locals.var_dio_le_dn9,)
    }
};
        locals.var_dio_le = assign8850_e10746;
        locals.var_dio_le_dn4 = assign8850_e10746_d_n4;
        locals.var_dio_le_dn5 = assign8850_e10746_d_n5;
        locals.var_dio_le_dn6 = assign8850_e10746_d_n6;
        locals.var_dio_le_dn7 = assign8850_e10746_d_n7;
        locals.var_dio_le_dn8 = assign8850_e10746_d_n8;
        locals.var_dio_le_dn9 = assign8850_e10746_d_n9;
        locals.var_dio_le_rv = 0.0;

        let assign8880_e10765: f64 = if p.p56 < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard198 = assign8880_e10765;
        locals.var_guard198_rv = 0.0;

        let assign8890_e10768: f64 = if locals.var_cjcx01_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard199 = assign8890_e10768;
        locals.var_guard199_rv = 0.0;

        let (assign8900_e10776,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) {
        let assign8900_e10774: f64 = (p.p54 / 4.0);
        (assign8900_e10774,)
    } else {
        (locals.var_dz_r,)
    }
};
        locals.var_dz_r = assign8900_e10776;
        locals.var_dz_r_rv = 0.0;

        let (assign8910_e10784, assign8910_e10784_d_n4,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) {
        let assign8910_e10782: f64 = (p.p56 - locals.var_vdcx_t);
        (assign8910_e10782, (-locals.var_vdcx_t_dn4),)
    } else {
        (locals.var_dv_p, locals.var_dv_p_dn4,)
    }
};
        locals.var_dv_p = assign8910_e10784;
        locals.var_dv_p_dn4 = assign8910_e10784_d_n4;
        locals.var_dv_p_rv = 0.0;

        let (assign8920_e10799, assign8920_e10799_d_n4,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) {
        let assign8920_e10791: f64 = (locals.var_ajcx_t).ln();
        let assign8920_e10792: f64 = (-assign8920_e10791);
        let assign8920_e10794: f64 = (assign8920_e10792 / p.p54);
        let assign8920_e10795: f64 = (assign8920_e10794).exp();
        let assign8920_e10796: f64 = (1.0 - assign8920_e10795);
        let assign8920_e10797: f64 = (locals.var_vdcx_t * assign8920_e10796);
        (assign8920_e10797, ((locals.var_vdcx_t_dn4 * assign8920_e10796) + (locals.var_vdcx_t * (-(assign8920_e10795 * ((-(locals.var_ajcx_t_dn4 / locals.var_ajcx_t)) / p.p54))))),)
    } else {
        (locals.var_dv_f, locals.var_dv_f_dn4,)
    }
};
        locals.var_dv_f = assign8920_e10799;
        locals.var_dv_f_dn4 = assign8920_e10799_d_n4;
        locals.var_dv_f_rv = 0.0;

        let (assign8930_e10807, assign8930_e10807_d_n4,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) {
        let assign8930_e10805: f64 = (locals.var_ajcx_t * locals.var_cjcx01_t);
        (assign8930_e10805, ((locals.var_ajcx_t_dn4 * locals.var_cjcx01_t) + (locals.var_ajcx_t * locals.var_cjcx01_t_dn4)),)
    } else {
        (locals.var_dc_max, locals.var_dc_max_dn4,)
    }
};
        locals.var_dc_max = assign8930_e10807;
        locals.var_dc_max_dn4 = assign8930_e10807_d_n4;
        locals.var_dc_max_rv = 0.0;

        let (assign8940_e10823, assign8940_e10823_d_n4,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) {
        let assign8940_e10814: f64 = (locals.var_dz_r - p.p54);
        let assign8940_e10817: f64 = (p.p56 / locals.var_vdcx_t);
        let assign8940_e10818: f64 = (assign8940_e10817).ln();
        let assign8940_e10819: f64 = (assign8940_e10814 * assign8940_e10818);
        let assign8940_e10820: f64 = (assign8940_e10819).exp();
        let assign8940_e10821: f64 = (locals.var_cjcx01_t * assign8940_e10820);
        (assign8940_e10821, ((locals.var_cjcx01_t_dn4 * assign8940_e10820) + (locals.var_cjcx01_t * (assign8940_e10820 * (assign8940_e10814 * ((-((p.p56 * locals.var_vdcx_t_dn4) / (locals.var_vdcx_t * locals.var_vdcx_t))) / assign8940_e10817))))),)
    } else {
        (locals.var_dc_c, locals.var_dc_c_dn4,)
    }
};
        locals.var_dc_c = assign8940_e10823;
        locals.var_dc_c_dn4 = assign8940_e10823_d_n4;
        locals.var_dc_c_rv = 0.0;

        let (assign8950_e10833, assign8950_e10833_d_n0, assign8950_e10833_d_n1, assign8950_e10833_d_n3, assign8950_e10833_d_n4, assign8950_e10833_d_n5, assign8950_e10833_d_n7, assign8950_e10833_d_n8, assign8950_e10833_d_n9,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) {
        let assign8950_e10829: f64 = (locals.var_dv_f - locals.var_vbci);
        let assign8950_e10831: f64 = (assign8950_e10829 * locals.var_ovt);
        (assign8950_e10831, 0.0, ((-locals.var_vbci_dn1) * locals.var_ovt), 0.0, ((locals.var_dv_f_dn4 * locals.var_ovt) + (assign8950_e10829 * locals.var_ovt_dn4)), ((-locals.var_vbci_dn5) * locals.var_ovt), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dv_e, locals.var_dv_e_dn0, locals.var_dv_e_dn1, locals.var_dv_e_dn3, locals.var_dv_e_dn4, locals.var_dv_e_dn5, locals.var_dv_e_dn7, locals.var_dv_e_dn8, locals.var_dv_e_dn9,)
    }
};
        locals.var_dv_e = assign8950_e10833;
        locals.var_dv_e_dn0 = assign8950_e10833_d_n0;
        locals.var_dv_e_dn1 = assign8950_e10833_d_n1;
        locals.var_dv_e_dn3 = assign8950_e10833_d_n3;
        locals.var_dv_e_dn4 = assign8950_e10833_d_n4;
        locals.var_dv_e_dn5 = assign8950_e10833_d_n5;
        locals.var_dv_e_dn7 = assign8950_e10833_d_n7;
        locals.var_dv_e_dn8 = assign8950_e10833_d_n8;
        locals.var_dv_e_dn9 = assign8950_e10833_d_n9;
        locals.var_dv_e_rv = 0.0;

        let assign8960_e10836: f64 = if locals.var_dv_e < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard200 = assign8960_e10836;
        locals.var_guard200_rv = 0.0;

        let (assign8970_e10845, assign8970_e10845_d_n0, assign8970_e10845_d_n1, assign8970_e10845_d_n3, assign8970_e10845_d_n4, assign8970_e10845_d_n5, assign8970_e10845_d_n7, assign8970_e10845_d_n8, assign8970_e10845_d_n9,) = {
    if (((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard200 != 0.0)) {
        let assign8970_e10843: f64 = (locals.var_dv_e).exp();
        (assign8970_e10843, (assign8970_e10843 * locals.var_dv_e_dn0), (assign8970_e10843 * locals.var_dv_e_dn1), (assign8970_e10843 * locals.var_dv_e_dn3), (assign8970_e10843 * locals.var_dv_e_dn4), (assign8970_e10843 * locals.var_dv_e_dn5), (assign8970_e10843 * locals.var_dv_e_dn7), (assign8970_e10843 * locals.var_dv_e_dn8), (assign8970_e10843 * locals.var_dv_e_dn9),)
    } else {
        (locals.var_de, locals.var_de_dn0, locals.var_de_dn1, locals.var_de_dn3, locals.var_de_dn4, locals.var_de_dn5, locals.var_de_dn7, locals.var_de_dn8, locals.var_de_dn9,)
    }
};
        locals.var_de = assign8970_e10845;
        locals.var_de_dn0 = assign8970_e10845_d_n0;
        locals.var_de_dn1 = assign8970_e10845_d_n1;
        locals.var_de_dn3 = assign8970_e10845_d_n3;
        locals.var_de_dn4 = assign8970_e10845_d_n4;
        locals.var_de_dn5 = assign8970_e10845_d_n5;
        locals.var_de_dn7 = assign8970_e10845_d_n7;
        locals.var_de_dn8 = assign8970_e10845_d_n8;
        locals.var_de_dn9 = assign8970_e10845_d_n9;
        locals.var_de_rv = 0.0;

        let (assign8980_e10857, assign8980_e10857_d_n0, assign8980_e10857_d_n1, assign8980_e10857_d_n3, assign8980_e10857_d_n4, assign8980_e10857_d_n5, assign8980_e10857_d_n7, assign8980_e10857_d_n8, assign8980_e10857_d_n9,) = {
    if (((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard200 != 0.0)) {
        let assign8980_e10854: f64 = (1.0 + locals.var_de);
        let assign8980_e10855: f64 = (locals.var_de / assign8980_e10854);
        (assign8980_e10855, (((locals.var_de_dn0 * assign8980_e10854) - (locals.var_de * locals.var_de_dn0)) / (assign8980_e10854 * assign8980_e10854)), (((locals.var_de_dn1 * assign8980_e10854) - (locals.var_de * locals.var_de_dn1)) / (assign8980_e10854 * assign8980_e10854)), (((locals.var_de_dn3 * assign8980_e10854) - (locals.var_de * locals.var_de_dn3)) / (assign8980_e10854 * assign8980_e10854)), (((locals.var_de_dn4 * assign8980_e10854) - (locals.var_de * locals.var_de_dn4)) / (assign8980_e10854 * assign8980_e10854)), (((locals.var_de_dn5 * assign8980_e10854) - (locals.var_de * locals.var_de_dn5)) / (assign8980_e10854 * assign8980_e10854)), (((locals.var_de_dn7 * assign8980_e10854) - (locals.var_de * locals.var_de_dn7)) / (assign8980_e10854 * assign8980_e10854)), (((locals.var_de_dn8 * assign8980_e10854) - (locals.var_de * locals.var_de_dn8)) / (assign8980_e10854 * assign8980_e10854)), (((locals.var_de_dn9 * assign8980_e10854) - (locals.var_de * locals.var_de_dn9)) / (assign8980_e10854 * assign8980_e10854)),)
    } else {
        (locals.var_de_1, locals.var_de_1_dn0, locals.var_de_1_dn1, locals.var_de_1_dn3, locals.var_de_1_dn4, locals.var_de_1_dn5, locals.var_de_1_dn7, locals.var_de_1_dn8, locals.var_de_1_dn9,)
    }
};
        locals.var_de_1 = assign8980_e10857;
        locals.var_de_1_dn0 = assign8980_e10857_d_n0;
        locals.var_de_1_dn1 = assign8980_e10857_d_n1;
        locals.var_de_1_dn3 = assign8980_e10857_d_n3;
        locals.var_de_1_dn4 = assign8980_e10857_d_n4;
        locals.var_de_1_dn5 = assign8980_e10857_d_n5;
        locals.var_de_1_dn7 = assign8980_e10857_d_n7;
        locals.var_de_1_dn8 = assign8980_e10857_d_n8;
        locals.var_de_1_dn9 = assign8980_e10857_d_n9;
        locals.var_de_1_rv = 0.0;

        let (assign8990_e10872, assign8990_e10872_d_n0, assign8990_e10872_d_n1, assign8990_e10872_d_n3, assign8990_e10872_d_n4, assign8990_e10872_d_n5, assign8990_e10872_d_n7, assign8990_e10872_d_n8, assign8990_e10872_d_n9,) = {
    if (((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard200 != 0.0)) {
        let assign8990_e10867: f64 = (1.0 + locals.var_de);
        let assign8990_e10868: f64 = (assign8990_e10867).ln();
        let assign8990_e10869: f64 = (locals.var_vt * assign8990_e10868);
        let assign8990_e10870: f64 = (locals.var_dv_f - assign8990_e10869);
        (assign8990_e10870, (-(locals.var_vt * (locals.var_de_dn0 / assign8990_e10867))), (-(locals.var_vt * (locals.var_de_dn1 / assign8990_e10867))), (-(locals.var_vt * (locals.var_de_dn3 / assign8990_e10867))), (locals.var_dv_f_dn4 - ((locals.var_vt_dn4 * assign8990_e10868) + (locals.var_vt * (locals.var_de_dn4 / assign8990_e10867)))), (-(locals.var_vt * (locals.var_de_dn5 / assign8990_e10867))), (-(locals.var_vt * (locals.var_de_dn7 / assign8990_e10867))), (-(locals.var_vt * (locals.var_de_dn8 / assign8990_e10867))), (-(locals.var_vt * (locals.var_de_dn9 / assign8990_e10867))),)
    } else {
        (locals.var_dv_j1, locals.var_dv_j1_dn0, locals.var_dv_j1_dn1, locals.var_dv_j1_dn3, locals.var_dv_j1_dn4, locals.var_dv_j1_dn5, locals.var_dv_j1_dn7, locals.var_dv_j1_dn8, locals.var_dv_j1_dn9,)
    }
};
        locals.var_dv_j1 = assign8990_e10872;
        locals.var_dv_j1_dn0 = assign8990_e10872_d_n0;
        locals.var_dv_j1_dn1 = assign8990_e10872_d_n1;
        locals.var_dv_j1_dn3 = assign8990_e10872_d_n3;
        locals.var_dv_j1_dn4 = assign8990_e10872_d_n4;
        locals.var_dv_j1_dn5 = assign8990_e10872_d_n5;
        locals.var_dv_j1_dn7 = assign8990_e10872_d_n7;
        locals.var_dv_j1_dn8 = assign8990_e10872_d_n8;
        locals.var_dv_j1_dn9 = assign8990_e10872_d_n9;
        locals.var_dv_j1_rv = 0.0;

        let (assign9000_e10881, assign9000_e10881_d_n0, assign9000_e10881_d_n1, assign9000_e10881_d_n3, assign9000_e10881_d_n4, assign9000_e10881_d_n5, assign9000_e10881_d_n7, assign9000_e10881_d_n8, assign9000_e10881_d_n9,) = {
    if (((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard200 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_de_1, locals.var_de_1_dn0, locals.var_de_1_dn1, locals.var_de_1_dn3, locals.var_de_1_dn4, locals.var_de_1_dn5, locals.var_de_1_dn7, locals.var_de_1_dn8, locals.var_de_1_dn9,)
    }
};
        locals.var_de_1 = assign9000_e10881;
        locals.var_de_1_dn0 = assign9000_e10881_d_n0;
        locals.var_de_1_dn1 = assign9000_e10881_d_n1;
        locals.var_de_1_dn3 = assign9000_e10881_d_n3;
        locals.var_de_1_dn4 = assign9000_e10881_d_n4;
        locals.var_de_1_dn5 = assign9000_e10881_d_n5;
        locals.var_de_1_dn7 = assign9000_e10881_d_n7;
        locals.var_de_1_dn8 = assign9000_e10881_d_n8;
        locals.var_de_1_dn9 = assign9000_e10881_d_n9;
        locals.var_de_1_rv = 0.0;

        let (assign9010_e10890, assign9010_e10890_d_n0, assign9010_e10890_d_n1, assign9010_e10890_d_n3, assign9010_e10890_d_n4, assign9010_e10890_d_n5, assign9010_e10890_d_n7, assign9010_e10890_d_n8, assign9010_e10890_d_n9,) = {
    if (((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard200 == 0.0)) {
        (locals.var_vbci, 0.0, locals.var_vbci_dn1, 0.0, 0.0, locals.var_vbci_dn5, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dv_j1, locals.var_dv_j1_dn0, locals.var_dv_j1_dn1, locals.var_dv_j1_dn3, locals.var_dv_j1_dn4, locals.var_dv_j1_dn5, locals.var_dv_j1_dn7, locals.var_dv_j1_dn8, locals.var_dv_j1_dn9,)
    }
};
        locals.var_dv_j1 = assign9010_e10890;
        locals.var_dv_j1_dn0 = assign9010_e10890_d_n0;
        locals.var_dv_j1_dn1 = assign9010_e10890_d_n1;
        locals.var_dv_j1_dn3 = assign9010_e10890_d_n3;
        locals.var_dv_j1_dn4 = assign9010_e10890_d_n4;
        locals.var_dv_j1_dn5 = assign9010_e10890_d_n5;
        locals.var_dv_j1_dn7 = assign9010_e10890_d_n7;
        locals.var_dv_j1_dn8 = assign9010_e10890_d_n8;
        locals.var_dv_j1_dn9 = assign9010_e10890_d_n9;
        locals.var_dv_j1_rv = 0.0;

        let (assign9020_e10902, assign9020_e10902_d_n4,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) {
        let assign9020_e10896: f64 = (0.1 * locals.var_dv_p);
        let assign9020_e10899: f64 = (4.0 * locals.var_vt);
        let assign9020_e10900: f64 = (assign9020_e10896 + assign9020_e10899);
        (assign9020_e10900, ((0.1 * locals.var_dv_p_dn4) + (4.0 * locals.var_vt_dn4)),)
    } else {
        (locals.var_da, locals.var_da_dn4,)
    }
};
        locals.var_da = assign9020_e10902;
        locals.var_da_dn4 = assign9020_e10902_d_n4;
        locals.var_da_rv = 0.0;

        let (assign9030_e10912, assign9030_e10912_d_n0, assign9030_e10912_d_n1, assign9030_e10912_d_n3, assign9030_e10912_d_n4, assign9030_e10912_d_n5, assign9030_e10912_d_n7, assign9030_e10912_d_n8, assign9030_e10912_d_n9,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) {
        let assign9030_e10908: f64 = (locals.var_dv_p + locals.var_dv_j1);
        let assign9030_e10910: f64 = (assign9030_e10908 / locals.var_da);
        (assign9030_e10910, (locals.var_dv_j1_dn0 / locals.var_da), (locals.var_dv_j1_dn1 / locals.var_da), (locals.var_dv_j1_dn3 / locals.var_da), ((((locals.var_dv_p_dn4 + locals.var_dv_j1_dn4) * locals.var_da) - (assign9030_e10908 * locals.var_da_dn4)) / (locals.var_da * locals.var_da)), (locals.var_dv_j1_dn5 / locals.var_da), (locals.var_dv_j1_dn7 / locals.var_da), (locals.var_dv_j1_dn8 / locals.var_da), (locals.var_dv_j1_dn9 / locals.var_da),)
    } else {
        (locals.var_dv_r, locals.var_dv_r_dn0, locals.var_dv_r_dn1, locals.var_dv_r_dn3, locals.var_dv_r_dn4, locals.var_dv_r_dn5, locals.var_dv_r_dn7, locals.var_dv_r_dn8, locals.var_dv_r_dn9,)
    }
};
        locals.var_dv_r = assign9030_e10912;
        locals.var_dv_r_dn0 = assign9030_e10912_d_n0;
        locals.var_dv_r_dn1 = assign9030_e10912_d_n1;
        locals.var_dv_r_dn3 = assign9030_e10912_d_n3;
        locals.var_dv_r_dn4 = assign9030_e10912_d_n4;
        locals.var_dv_r_dn5 = assign9030_e10912_d_n5;
        locals.var_dv_r_dn7 = assign9030_e10912_d_n7;
        locals.var_dv_r_dn8 = assign9030_e10912_d_n8;
        locals.var_dv_r_dn9 = assign9030_e10912_d_n9;
        locals.var_dv_r_rv = 0.0;

        let assign9040_e10915: f64 = if locals.var_dv_r < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard201 = assign9040_e10915;
        locals.var_guard201_rv = 0.0;

        let (assign9050_e10924, assign9050_e10924_d_n0, assign9050_e10924_d_n1, assign9050_e10924_d_n3, assign9050_e10924_d_n4, assign9050_e10924_d_n5, assign9050_e10924_d_n7, assign9050_e10924_d_n8, assign9050_e10924_d_n9,) = {
    if (((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard201 != 0.0)) {
        let assign9050_e10922: f64 = (locals.var_dv_r).exp();
        (assign9050_e10922, (assign9050_e10922 * locals.var_dv_r_dn0), (assign9050_e10922 * locals.var_dv_r_dn1), (assign9050_e10922 * locals.var_dv_r_dn3), (assign9050_e10922 * locals.var_dv_r_dn4), (assign9050_e10922 * locals.var_dv_r_dn5), (assign9050_e10922 * locals.var_dv_r_dn7), (assign9050_e10922 * locals.var_dv_r_dn8), (assign9050_e10922 * locals.var_dv_r_dn9),)
    } else {
        (locals.var_de, locals.var_de_dn0, locals.var_de_dn1, locals.var_de_dn3, locals.var_de_dn4, locals.var_de_dn5, locals.var_de_dn7, locals.var_de_dn8, locals.var_de_dn9,)
    }
};
        locals.var_de = assign9050_e10924;
        locals.var_de_dn0 = assign9050_e10924_d_n0;
        locals.var_de_dn1 = assign9050_e10924_d_n1;
        locals.var_de_dn3 = assign9050_e10924_d_n3;
        locals.var_de_dn4 = assign9050_e10924_d_n4;
        locals.var_de_dn5 = assign9050_e10924_d_n5;
        locals.var_de_dn7 = assign9050_e10924_d_n7;
        locals.var_de_dn8 = assign9050_e10924_d_n8;
        locals.var_de_dn9 = assign9050_e10924_d_n9;
        locals.var_de_rv = 0.0;

        let (assign9060_e10936, assign9060_e10936_d_n0, assign9060_e10936_d_n1, assign9060_e10936_d_n3, assign9060_e10936_d_n4, assign9060_e10936_d_n5, assign9060_e10936_d_n7, assign9060_e10936_d_n8, assign9060_e10936_d_n9,) = {
    if (((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard201 != 0.0)) {
        let assign9060_e10933: f64 = (1.0 + locals.var_de);
        let assign9060_e10934: f64 = (locals.var_de / assign9060_e10933);
        (assign9060_e10934, (((locals.var_de_dn0 * assign9060_e10933) - (locals.var_de * locals.var_de_dn0)) / (assign9060_e10933 * assign9060_e10933)), (((locals.var_de_dn1 * assign9060_e10933) - (locals.var_de * locals.var_de_dn1)) / (assign9060_e10933 * assign9060_e10933)), (((locals.var_de_dn3 * assign9060_e10933) - (locals.var_de * locals.var_de_dn3)) / (assign9060_e10933 * assign9060_e10933)), (((locals.var_de_dn4 * assign9060_e10933) - (locals.var_de * locals.var_de_dn4)) / (assign9060_e10933 * assign9060_e10933)), (((locals.var_de_dn5 * assign9060_e10933) - (locals.var_de * locals.var_de_dn5)) / (assign9060_e10933 * assign9060_e10933)), (((locals.var_de_dn7 * assign9060_e10933) - (locals.var_de * locals.var_de_dn7)) / (assign9060_e10933 * assign9060_e10933)), (((locals.var_de_dn8 * assign9060_e10933) - (locals.var_de * locals.var_de_dn8)) / (assign9060_e10933 * assign9060_e10933)), (((locals.var_de_dn9 * assign9060_e10933) - (locals.var_de * locals.var_de_dn9)) / (assign9060_e10933 * assign9060_e10933)),)
    } else {
        (locals.var_de_2, locals.var_de_2_dn0, locals.var_de_2_dn1, locals.var_de_2_dn3, locals.var_de_2_dn4, locals.var_de_2_dn5, locals.var_de_2_dn7, locals.var_de_2_dn8, locals.var_de_2_dn9,)
    }
};
        locals.var_de_2 = assign9060_e10936;
        locals.var_de_2_dn0 = assign9060_e10936_d_n0;
        locals.var_de_2_dn1 = assign9060_e10936_d_n1;
        locals.var_de_2_dn3 = assign9060_e10936_d_n3;
        locals.var_de_2_dn4 = assign9060_e10936_d_n4;
        locals.var_de_2_dn5 = assign9060_e10936_d_n5;
        locals.var_de_2_dn7 = assign9060_e10936_d_n7;
        locals.var_de_2_dn8 = assign9060_e10936_d_n8;
        locals.var_de_2_dn9 = assign9060_e10936_d_n9;
        locals.var_de_2_rv = 0.0;

        let (assign9070_e10960, assign9070_e10960_d_n0, assign9070_e10960_d_n1, assign9070_e10960_d_n3, assign9070_e10960_d_n4, assign9070_e10960_d_n5, assign9070_e10960_d_n7, assign9070_e10960_d_n8, assign9070_e10960_d_n9,) = {
    if (((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard201 != 0.0)) {
        let assign9070_e10943: f64 = (-locals.var_dv_p);
        let assign9070_e10947: f64 = (1.0 + locals.var_de);
        let assign9070_e10948: f64 = (assign9070_e10947).ln();
        let assign9070_e10951: f64 = (locals.var_dv_p + locals.var_dv_f);
        let assign9070_e10952: f64 = (-assign9070_e10951);
        let assign9070_e10954: f64 = (assign9070_e10952 / locals.var_da);
        let assign9070_e10955: f64 = (assign9070_e10954).exp();
        let assign9070_e10956: f64 = (assign9070_e10948 - assign9070_e10955);
        let assign9070_e10957: f64 = (locals.var_da * assign9070_e10956);
        let assign9070_e10958: f64 = (assign9070_e10943 + assign9070_e10957);
        (assign9070_e10958, (locals.var_da * (locals.var_de_dn0 / assign9070_e10947)), (locals.var_da * (locals.var_de_dn1 / assign9070_e10947)), (locals.var_da * (locals.var_de_dn3 / assign9070_e10947)), ((-locals.var_dv_p_dn4) + ((locals.var_da_dn4 * assign9070_e10956) + (locals.var_da * ((locals.var_de_dn4 / assign9070_e10947) - (assign9070_e10955 * ((((-(locals.var_dv_p_dn4 + locals.var_dv_f_dn4)) * locals.var_da) - (assign9070_e10952 * locals.var_da_dn4)) / (locals.var_da * locals.var_da))))))), (locals.var_da * (locals.var_de_dn5 / assign9070_e10947)), (locals.var_da * (locals.var_de_dn7 / assign9070_e10947)), (locals.var_da * (locals.var_de_dn8 / assign9070_e10947)), (locals.var_da * (locals.var_de_dn9 / assign9070_e10947)),)
    } else {
        (locals.var_dv_j2, locals.var_dv_j2_dn0, locals.var_dv_j2_dn1, locals.var_dv_j2_dn3, locals.var_dv_j2_dn4, locals.var_dv_j2_dn5, locals.var_dv_j2_dn7, locals.var_dv_j2_dn8, locals.var_dv_j2_dn9,)
    }
};
        locals.var_dv_j2 = assign9070_e10960;
        locals.var_dv_j2_dn0 = assign9070_e10960_d_n0;
        locals.var_dv_j2_dn1 = assign9070_e10960_d_n1;
        locals.var_dv_j2_dn3 = assign9070_e10960_d_n3;
        locals.var_dv_j2_dn4 = assign9070_e10960_d_n4;
        locals.var_dv_j2_dn5 = assign9070_e10960_d_n5;
        locals.var_dv_j2_dn7 = assign9070_e10960_d_n7;
        locals.var_dv_j2_dn8 = assign9070_e10960_d_n8;
        locals.var_dv_j2_dn9 = assign9070_e10960_d_n9;
        locals.var_dv_j2_rv = 0.0;

        let (assign9080_e10969, assign9080_e10969_d_n0, assign9080_e10969_d_n1, assign9080_e10969_d_n3, assign9080_e10969_d_n4, assign9080_e10969_d_n5, assign9080_e10969_d_n7, assign9080_e10969_d_n8, assign9080_e10969_d_n9,) = {
    if (((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard201 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_de_2, locals.var_de_2_dn0, locals.var_de_2_dn1, locals.var_de_2_dn3, locals.var_de_2_dn4, locals.var_de_2_dn5, locals.var_de_2_dn7, locals.var_de_2_dn8, locals.var_de_2_dn9,)
    }
};
        locals.var_de_2 = assign9080_e10969;
        locals.var_de_2_dn0 = assign9080_e10969_d_n0;
        locals.var_de_2_dn1 = assign9080_e10969_d_n1;
        locals.var_de_2_dn3 = assign9080_e10969_d_n3;
        locals.var_de_2_dn4 = assign9080_e10969_d_n4;
        locals.var_de_2_dn5 = assign9080_e10969_d_n5;
        locals.var_de_2_dn7 = assign9080_e10969_d_n7;
        locals.var_de_2_dn8 = assign9080_e10969_d_n8;
        locals.var_de_2_dn9 = assign9080_e10969_d_n9;
        locals.var_de_2_rv = 0.0;

        let (assign9090_e10978, assign9090_e10978_d_n0, assign9090_e10978_d_n1, assign9090_e10978_d_n3, assign9090_e10978_d_n4, assign9090_e10978_d_n5, assign9090_e10978_d_n7, assign9090_e10978_d_n8, assign9090_e10978_d_n9,) = {
    if (((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) && (locals.var_guard201 == 0.0)) {
        (locals.var_dv_j1, locals.var_dv_j1_dn0, locals.var_dv_j1_dn1, locals.var_dv_j1_dn3, locals.var_dv_j1_dn4, locals.var_dv_j1_dn5, locals.var_dv_j1_dn7, locals.var_dv_j1_dn8, locals.var_dv_j1_dn9,)
    } else {
        (locals.var_dv_j2, locals.var_dv_j2_dn0, locals.var_dv_j2_dn1, locals.var_dv_j2_dn3, locals.var_dv_j2_dn4, locals.var_dv_j2_dn5, locals.var_dv_j2_dn7, locals.var_dv_j2_dn8, locals.var_dv_j2_dn9,)
    }
};
        locals.var_dv_j2 = assign9090_e10978;
        locals.var_dv_j2_dn0 = assign9090_e10978_d_n0;
        locals.var_dv_j2_dn1 = assign9090_e10978_d_n1;
        locals.var_dv_j2_dn3 = assign9090_e10978_d_n3;
        locals.var_dv_j2_dn4 = assign9090_e10978_d_n4;
        locals.var_dv_j2_dn5 = assign9090_e10978_d_n5;
        locals.var_dv_j2_dn7 = assign9090_e10978_d_n7;
        locals.var_dv_j2_dn8 = assign9090_e10978_d_n8;
        locals.var_dv_j2_dn9 = assign9090_e10978_d_n9;
        locals.var_dv_j2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_24(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9100_e10986, assign9100_e10986_d_n0, assign9100_e10986_d_n1, assign9100_e10986_d_n3, assign9100_e10986_d_n4, assign9100_e10986_d_n5, assign9100_e10986_d_n7, assign9100_e10986_d_n8, assign9100_e10986_d_n9,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) {
        let assign9100_e10984: f64 = (locals.var_vbci - locals.var_dv_j1);
        (assign9100_e10984, (-locals.var_dv_j1_dn0), (locals.var_vbci_dn1 - locals.var_dv_j1_dn1), (-locals.var_dv_j1_dn3), (-locals.var_dv_j1_dn4), (locals.var_vbci_dn5 - locals.var_dv_j1_dn5), (-locals.var_dv_j1_dn7), (-locals.var_dv_j1_dn8), (-locals.var_dv_j1_dn9),)
    } else {
        (locals.var_dv_j4, locals.var_dv_j4_dn0, locals.var_dv_j4_dn1, locals.var_dv_j4_dn3, locals.var_dv_j4_dn4, locals.var_dv_j4_dn5, locals.var_dv_j4_dn7, locals.var_dv_j4_dn8, locals.var_dv_j4_dn9,)
    }
};
        locals.var_dv_j4 = assign9100_e10986;
        locals.var_dv_j4_dn0 = assign9100_e10986_d_n0;
        locals.var_dv_j4_dn1 = assign9100_e10986_d_n1;
        locals.var_dv_j4_dn3 = assign9100_e10986_d_n3;
        locals.var_dv_j4_dn4 = assign9100_e10986_d_n4;
        locals.var_dv_j4_dn5 = assign9100_e10986_d_n5;
        locals.var_dv_j4_dn7 = assign9100_e10986_d_n7;
        locals.var_dv_j4_dn8 = assign9100_e10986_d_n8;
        locals.var_dv_j4_dn9 = assign9100_e10986_d_n9;
        locals.var_dv_j4_rv = 0.0;

        let (assign9110_e10997, assign9110_e10997_d_n0, assign9110_e10997_d_n1, assign9110_e10997_d_n3, assign9110_e10997_d_n4, assign9110_e10997_d_n5, assign9110_e10997_d_n7, assign9110_e10997_d_n8, assign9110_e10997_d_n9,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) {
        let assign9110_e10993: f64 = (locals.var_dv_j1 / locals.var_vdcx_t);
        let assign9110_e10994: f64 = (1.0 - assign9110_e10993);
        let assign9110_e10995: f64 = (assign9110_e10994).ln();
        (assign9110_e10995, ((-(locals.var_dv_j1_dn0 / locals.var_vdcx_t)) / assign9110_e10994), ((-(locals.var_dv_j1_dn1 / locals.var_vdcx_t)) / assign9110_e10994), ((-(locals.var_dv_j1_dn3 / locals.var_vdcx_t)) / assign9110_e10994), ((-(((locals.var_dv_j1_dn4 * locals.var_vdcx_t) - (locals.var_dv_j1 * locals.var_vdcx_t_dn4)) / (locals.var_vdcx_t * locals.var_vdcx_t))) / assign9110_e10994), ((-(locals.var_dv_j1_dn5 / locals.var_vdcx_t)) / assign9110_e10994), ((-(locals.var_dv_j1_dn7 / locals.var_vdcx_t)) / assign9110_e10994), ((-(locals.var_dv_j1_dn8 / locals.var_vdcx_t)) / assign9110_e10994), ((-(locals.var_dv_j1_dn9 / locals.var_vdcx_t)) / assign9110_e10994),)
    } else {
        (locals.var_dcln1, locals.var_dcln1_dn0, locals.var_dcln1_dn1, locals.var_dcln1_dn3, locals.var_dcln1_dn4, locals.var_dcln1_dn5, locals.var_dcln1_dn7, locals.var_dcln1_dn8, locals.var_dcln1_dn9,)
    }
};
        locals.var_dcln1 = assign9110_e10997;
        locals.var_dcln1_dn0 = assign9110_e10997_d_n0;
        locals.var_dcln1_dn1 = assign9110_e10997_d_n1;
        locals.var_dcln1_dn3 = assign9110_e10997_d_n3;
        locals.var_dcln1_dn4 = assign9110_e10997_d_n4;
        locals.var_dcln1_dn5 = assign9110_e10997_d_n5;
        locals.var_dcln1_dn7 = assign9110_e10997_d_n7;
        locals.var_dcln1_dn8 = assign9110_e10997_d_n8;
        locals.var_dcln1_dn9 = assign9110_e10997_d_n9;
        locals.var_dcln1_rv = 0.0;

        let (assign9120_e11008, assign9120_e11008_d_n0, assign9120_e11008_d_n1, assign9120_e11008_d_n3, assign9120_e11008_d_n4, assign9120_e11008_d_n5, assign9120_e11008_d_n7, assign9120_e11008_d_n8, assign9120_e11008_d_n9,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) {
        let assign9120_e11004: f64 = (locals.var_dv_j2 / locals.var_vdcx_t);
        let assign9120_e11005: f64 = (1.0 - assign9120_e11004);
        let assign9120_e11006: f64 = (assign9120_e11005).ln();
        (assign9120_e11006, ((-(locals.var_dv_j2_dn0 / locals.var_vdcx_t)) / assign9120_e11005), ((-(locals.var_dv_j2_dn1 / locals.var_vdcx_t)) / assign9120_e11005), ((-(locals.var_dv_j2_dn3 / locals.var_vdcx_t)) / assign9120_e11005), ((-(((locals.var_dv_j2_dn4 * locals.var_vdcx_t) - (locals.var_dv_j2 * locals.var_vdcx_t_dn4)) / (locals.var_vdcx_t * locals.var_vdcx_t))) / assign9120_e11005), ((-(locals.var_dv_j2_dn5 / locals.var_vdcx_t)) / assign9120_e11005), ((-(locals.var_dv_j2_dn7 / locals.var_vdcx_t)) / assign9120_e11005), ((-(locals.var_dv_j2_dn8 / locals.var_vdcx_t)) / assign9120_e11005), ((-(locals.var_dv_j2_dn9 / locals.var_vdcx_t)) / assign9120_e11005),)
    } else {
        (locals.var_dcln2, locals.var_dcln2_dn0, locals.var_dcln2_dn1, locals.var_dcln2_dn3, locals.var_dcln2_dn4, locals.var_dcln2_dn5, locals.var_dcln2_dn7, locals.var_dcln2_dn8, locals.var_dcln2_dn9,)
    }
};
        locals.var_dcln2 = assign9120_e11008;
        locals.var_dcln2_dn0 = assign9120_e11008_d_n0;
        locals.var_dcln2_dn1 = assign9120_e11008_d_n1;
        locals.var_dcln2_dn3 = assign9120_e11008_d_n3;
        locals.var_dcln2_dn4 = assign9120_e11008_d_n4;
        locals.var_dcln2_dn5 = assign9120_e11008_d_n5;
        locals.var_dcln2_dn7 = assign9120_e11008_d_n7;
        locals.var_dcln2_dn8 = assign9120_e11008_d_n8;
        locals.var_dcln2_dn9 = assign9120_e11008_d_n9;
        locals.var_dcln2_rv = 0.0;

        let (assign9130_e11016,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) {
        let assign9130_e11014: f64 = (1.0 - p.p54);
        (assign9130_e11014,)
    } else {
        (locals.var_dz1,)
    }
};
        locals.var_dz1 = assign9130_e11016;
        locals.var_dz1_rv = 0.0;

        let (assign9140_e11024,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) {
        let assign9140_e11022: f64 = (1.0 - locals.var_dz_r);
        (assign9140_e11022,)
    } else {
        (locals.var_dzr1,)
    }
};
        locals.var_dzr1 = assign9140_e11024;
        locals.var_dzr1_rv = 0.0;

        let (assign9150_e11040, assign9150_e11040_d_n0, assign9150_e11040_d_n1, assign9150_e11040_d_n3, assign9150_e11040_d_n4, assign9150_e11040_d_n5, assign9150_e11040_d_n7, assign9150_e11040_d_n8, assign9150_e11040_d_n9,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) {
        let assign9150_e11031: f64 = (-p.p54);
        let assign9150_e11032: f64 = (locals.var_dcln2 * assign9150_e11031);
        let assign9150_e11033: f64 = (assign9150_e11032).exp();
        let assign9150_e11034: f64 = (locals.var_cjcx01_t * assign9150_e11033);
        let assign9150_e11036: f64 = (assign9150_e11034 * locals.var_de_1);
        let assign9150_e11038: f64 = (assign9150_e11036 * locals.var_de_2);
        (assign9150_e11038, (((((locals.var_cjcx01_t * (assign9150_e11033 * (locals.var_dcln2_dn0 * assign9150_e11031))) * locals.var_de_1) + (assign9150_e11034 * locals.var_de_1_dn0)) * locals.var_de_2) + (assign9150_e11036 * locals.var_de_2_dn0)), (((((locals.var_cjcx01_t * (assign9150_e11033 * (locals.var_dcln2_dn1 * assign9150_e11031))) * locals.var_de_1) + (assign9150_e11034 * locals.var_de_1_dn1)) * locals.var_de_2) + (assign9150_e11036 * locals.var_de_2_dn1)), (((((locals.var_cjcx01_t * (assign9150_e11033 * (locals.var_dcln2_dn3 * assign9150_e11031))) * locals.var_de_1) + (assign9150_e11034 * locals.var_de_1_dn3)) * locals.var_de_2) + (assign9150_e11036 * locals.var_de_2_dn3)), ((((((locals.var_cjcx01_t_dn4 * assign9150_e11033) + (locals.var_cjcx01_t * (assign9150_e11033 * (locals.var_dcln2_dn4 * assign9150_e11031)))) * locals.var_de_1) + (assign9150_e11034 * locals.var_de_1_dn4)) * locals.var_de_2) + (assign9150_e11036 * locals.var_de_2_dn4)), (((((locals.var_cjcx01_t * (assign9150_e11033 * (locals.var_dcln2_dn5 * assign9150_e11031))) * locals.var_de_1) + (assign9150_e11034 * locals.var_de_1_dn5)) * locals.var_de_2) + (assign9150_e11036 * locals.var_de_2_dn5)), (((((locals.var_cjcx01_t * (assign9150_e11033 * (locals.var_dcln2_dn7 * assign9150_e11031))) * locals.var_de_1) + (assign9150_e11034 * locals.var_de_1_dn7)) * locals.var_de_2) + (assign9150_e11036 * locals.var_de_2_dn7)), (((((locals.var_cjcx01_t * (assign9150_e11033 * (locals.var_dcln2_dn8 * assign9150_e11031))) * locals.var_de_1) + (assign9150_e11034 * locals.var_de_1_dn8)) * locals.var_de_2) + (assign9150_e11036 * locals.var_de_2_dn8)), (((((locals.var_cjcx01_t * (assign9150_e11033 * (locals.var_dcln2_dn9 * assign9150_e11031))) * locals.var_de_1) + (assign9150_e11034 * locals.var_de_1_dn9)) * locals.var_de_2) + (assign9150_e11036 * locals.var_de_2_dn9)),)
    } else {
        (locals.var_dc_j1, locals.var_dc_j1_dn0, locals.var_dc_j1_dn1, locals.var_dc_j1_dn3, locals.var_dc_j1_dn4, locals.var_dc_j1_dn5, locals.var_dc_j1_dn7, locals.var_dc_j1_dn8, locals.var_dc_j1_dn9,)
    }
};
        locals.var_dc_j1 = assign9150_e11040;
        locals.var_dc_j1_dn0 = assign9150_e11040_d_n0;
        locals.var_dc_j1_dn1 = assign9150_e11040_d_n1;
        locals.var_dc_j1_dn3 = assign9150_e11040_d_n3;
        locals.var_dc_j1_dn4 = assign9150_e11040_d_n4;
        locals.var_dc_j1_dn5 = assign9150_e11040_d_n5;
        locals.var_dc_j1_dn7 = assign9150_e11040_d_n7;
        locals.var_dc_j1_dn8 = assign9150_e11040_d_n8;
        locals.var_dc_j1_dn9 = assign9150_e11040_d_n9;
        locals.var_dc_j1_rv = 0.0;

        let (assign9160_e11056, assign9160_e11056_d_n0, assign9160_e11056_d_n1, assign9160_e11056_d_n3, assign9160_e11056_d_n4, assign9160_e11056_d_n5, assign9160_e11056_d_n7, assign9160_e11056_d_n8, assign9160_e11056_d_n9,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) {
        let assign9160_e11047: f64 = (-locals.var_dz_r);
        let assign9160_e11048: f64 = (locals.var_dcln1 * assign9160_e11047);
        let assign9160_e11049: f64 = (assign9160_e11048).exp();
        let assign9160_e11050: f64 = (locals.var_dc_c * assign9160_e11049);
        let assign9160_e11053: f64 = (1.0 - locals.var_de_2);
        let assign9160_e11054: f64 = (assign9160_e11050 * assign9160_e11053);
        (assign9160_e11054, (((locals.var_dc_c * (assign9160_e11049 * (locals.var_dcln1_dn0 * assign9160_e11047))) * assign9160_e11053) + (assign9160_e11050 * (-locals.var_de_2_dn0))), (((locals.var_dc_c * (assign9160_e11049 * (locals.var_dcln1_dn1 * assign9160_e11047))) * assign9160_e11053) + (assign9160_e11050 * (-locals.var_de_2_dn1))), (((locals.var_dc_c * (assign9160_e11049 * (locals.var_dcln1_dn3 * assign9160_e11047))) * assign9160_e11053) + (assign9160_e11050 * (-locals.var_de_2_dn3))), ((((locals.var_dc_c_dn4 * assign9160_e11049) + (locals.var_dc_c * (assign9160_e11049 * (locals.var_dcln1_dn4 * assign9160_e11047)))) * assign9160_e11053) + (assign9160_e11050 * (-locals.var_de_2_dn4))), (((locals.var_dc_c * (assign9160_e11049 * (locals.var_dcln1_dn5 * assign9160_e11047))) * assign9160_e11053) + (assign9160_e11050 * (-locals.var_de_2_dn5))), (((locals.var_dc_c * (assign9160_e11049 * (locals.var_dcln1_dn7 * assign9160_e11047))) * assign9160_e11053) + (assign9160_e11050 * (-locals.var_de_2_dn7))), (((locals.var_dc_c * (assign9160_e11049 * (locals.var_dcln1_dn8 * assign9160_e11047))) * assign9160_e11053) + (assign9160_e11050 * (-locals.var_de_2_dn8))), (((locals.var_dc_c * (assign9160_e11049 * (locals.var_dcln1_dn9 * assign9160_e11047))) * assign9160_e11053) + (assign9160_e11050 * (-locals.var_de_2_dn9))),)
    } else {
        (locals.var_dc_j2, locals.var_dc_j2_dn0, locals.var_dc_j2_dn1, locals.var_dc_j2_dn3, locals.var_dc_j2_dn4, locals.var_dc_j2_dn5, locals.var_dc_j2_dn7, locals.var_dc_j2_dn8, locals.var_dc_j2_dn9,)
    }
};
        locals.var_dc_j2 = assign9160_e11056;
        locals.var_dc_j2_dn0 = assign9160_e11056_d_n0;
        locals.var_dc_j2_dn1 = assign9160_e11056_d_n1;
        locals.var_dc_j2_dn3 = assign9160_e11056_d_n3;
        locals.var_dc_j2_dn4 = assign9160_e11056_d_n4;
        locals.var_dc_j2_dn5 = assign9160_e11056_d_n5;
        locals.var_dc_j2_dn7 = assign9160_e11056_d_n7;
        locals.var_dc_j2_dn8 = assign9160_e11056_d_n8;
        locals.var_dc_j2_dn9 = assign9160_e11056_d_n9;
        locals.var_dc_j2_rv = 0.0;

        let (assign9170_e11066, assign9170_e11066_d_n0, assign9170_e11066_d_n1, assign9170_e11066_d_n3, assign9170_e11066_d_n4, assign9170_e11066_d_n5, assign9170_e11066_d_n7, assign9170_e11066_d_n8, assign9170_e11066_d_n9,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) {
        let assign9170_e11063: f64 = (1.0 - locals.var_de_1);
        let assign9170_e11064: f64 = (locals.var_dc_max * assign9170_e11063);
        (assign9170_e11064, (locals.var_dc_max * (-locals.var_de_1_dn0)), (locals.var_dc_max * (-locals.var_de_1_dn1)), (locals.var_dc_max * (-locals.var_de_1_dn3)), ((locals.var_dc_max_dn4 * assign9170_e11063) + (locals.var_dc_max * (-locals.var_de_1_dn4))), (locals.var_dc_max * (-locals.var_de_1_dn5)), (locals.var_dc_max * (-locals.var_de_1_dn7)), (locals.var_dc_max * (-locals.var_de_1_dn8)), (locals.var_dc_max * (-locals.var_de_1_dn9)),)
    } else {
        (locals.var_dc_j3, locals.var_dc_j3_dn0, locals.var_dc_j3_dn1, locals.var_dc_j3_dn3, locals.var_dc_j3_dn4, locals.var_dc_j3_dn5, locals.var_dc_j3_dn7, locals.var_dc_j3_dn8, locals.var_dc_j3_dn9,)
    }
};
        locals.var_dc_j3 = assign9170_e11066;
        locals.var_dc_j3_dn0 = assign9170_e11066_d_n0;
        locals.var_dc_j3_dn1 = assign9170_e11066_d_n1;
        locals.var_dc_j3_dn3 = assign9170_e11066_d_n3;
        locals.var_dc_j3_dn4 = assign9170_e11066_d_n4;
        locals.var_dc_j3_dn5 = assign9170_e11066_d_n5;
        locals.var_dc_j3_dn7 = assign9170_e11066_d_n7;
        locals.var_dc_j3_dn8 = assign9170_e11066_d_n8;
        locals.var_dc_j3_dn9 = assign9170_e11066_d_n9;
        locals.var_dc_j3_rv = 0.0;

        let (assign9190_e11091, assign9190_e11091_d_n0, assign9190_e11091_d_n1, assign9190_e11091_d_n3, assign9190_e11091_d_n4, assign9190_e11091_d_n5, assign9190_e11091_d_n7, assign9190_e11091_d_n8, assign9190_e11091_d_n9,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) {
        let assign9190_e11084: f64 = (locals.var_dcln2 * locals.var_dz1);
        let assign9190_e11085: f64 = (assign9190_e11084).exp();
        let assign9190_e11086: f64 = (1.0 - assign9190_e11085);
        let assign9190_e11087: f64 = (locals.var_cjcx01_t * assign9190_e11086);
        let assign9190_e11089: f64 = (assign9190_e11087 / locals.var_dz1);
        (assign9190_e11089, ((locals.var_cjcx01_t * (-(assign9190_e11085 * (locals.var_dcln2_dn0 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cjcx01_t * (-(assign9190_e11085 * (locals.var_dcln2_dn1 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cjcx01_t * (-(assign9190_e11085 * (locals.var_dcln2_dn3 * locals.var_dz1)))) / locals.var_dz1), (((locals.var_cjcx01_t_dn4 * assign9190_e11086) + (locals.var_cjcx01_t * (-(assign9190_e11085 * (locals.var_dcln2_dn4 * locals.var_dz1))))) / locals.var_dz1), ((locals.var_cjcx01_t * (-(assign9190_e11085 * (locals.var_dcln2_dn5 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cjcx01_t * (-(assign9190_e11085 * (locals.var_dcln2_dn7 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cjcx01_t * (-(assign9190_e11085 * (locals.var_dcln2_dn8 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cjcx01_t * (-(assign9190_e11085 * (locals.var_dcln2_dn9 * locals.var_dz1)))) / locals.var_dz1),)
    } else {
        (locals.var_dq_j1, locals.var_dq_j1_dn0, locals.var_dq_j1_dn1, locals.var_dq_j1_dn3, locals.var_dq_j1_dn4, locals.var_dq_j1_dn5, locals.var_dq_j1_dn7, locals.var_dq_j1_dn8, locals.var_dq_j1_dn9,)
    }
};
        locals.var_dq_j1 = assign9190_e11091;
        locals.var_dq_j1_dn0 = assign9190_e11091_d_n0;
        locals.var_dq_j1_dn1 = assign9190_e11091_d_n1;
        locals.var_dq_j1_dn3 = assign9190_e11091_d_n3;
        locals.var_dq_j1_dn4 = assign9190_e11091_d_n4;
        locals.var_dq_j1_dn5 = assign9190_e11091_d_n5;
        locals.var_dq_j1_dn7 = assign9190_e11091_d_n7;
        locals.var_dq_j1_dn8 = assign9190_e11091_d_n8;
        locals.var_dq_j1_dn9 = assign9190_e11091_d_n9;
        locals.var_dq_j1_rv = 0.0;

        let (assign9200_e11106, assign9200_e11106_d_n0, assign9200_e11106_d_n1, assign9200_e11106_d_n3, assign9200_e11106_d_n4, assign9200_e11106_d_n5, assign9200_e11106_d_n7, assign9200_e11106_d_n8, assign9200_e11106_d_n9,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) {
        let assign9200_e11099: f64 = (locals.var_dcln1 * locals.var_dzr1);
        let assign9200_e11100: f64 = (assign9200_e11099).exp();
        let assign9200_e11101: f64 = (1.0 - assign9200_e11100);
        let assign9200_e11102: f64 = (locals.var_dc_c * assign9200_e11101);
        let assign9200_e11104: f64 = (assign9200_e11102 / locals.var_dzr1);
        (assign9200_e11104, ((locals.var_dc_c * (-(assign9200_e11100 * (locals.var_dcln1_dn0 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9200_e11100 * (locals.var_dcln1_dn1 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9200_e11100 * (locals.var_dcln1_dn3 * locals.var_dzr1)))) / locals.var_dzr1), (((locals.var_dc_c_dn4 * assign9200_e11101) + (locals.var_dc_c * (-(assign9200_e11100 * (locals.var_dcln1_dn4 * locals.var_dzr1))))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9200_e11100 * (locals.var_dcln1_dn5 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9200_e11100 * (locals.var_dcln1_dn7 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9200_e11100 * (locals.var_dcln1_dn8 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9200_e11100 * (locals.var_dcln1_dn9 * locals.var_dzr1)))) / locals.var_dzr1),)
    } else {
        (locals.var_dq_j2, locals.var_dq_j2_dn0, locals.var_dq_j2_dn1, locals.var_dq_j2_dn3, locals.var_dq_j2_dn4, locals.var_dq_j2_dn5, locals.var_dq_j2_dn7, locals.var_dq_j2_dn8, locals.var_dq_j2_dn9,)
    }
};
        locals.var_dq_j2 = assign9200_e11106;
        locals.var_dq_j2_dn0 = assign9200_e11106_d_n0;
        locals.var_dq_j2_dn1 = assign9200_e11106_d_n1;
        locals.var_dq_j2_dn3 = assign9200_e11106_d_n3;
        locals.var_dq_j2_dn4 = assign9200_e11106_d_n4;
        locals.var_dq_j2_dn5 = assign9200_e11106_d_n5;
        locals.var_dq_j2_dn7 = assign9200_e11106_d_n7;
        locals.var_dq_j2_dn8 = assign9200_e11106_d_n8;
        locals.var_dq_j2_dn9 = assign9200_e11106_d_n9;
        locals.var_dq_j2_rv = 0.0;

        let (assign9210_e11121, assign9210_e11121_d_n0, assign9210_e11121_d_n1, assign9210_e11121_d_n3, assign9210_e11121_d_n4, assign9210_e11121_d_n5, assign9210_e11121_d_n7, assign9210_e11121_d_n8, assign9210_e11121_d_n9,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) {
        let assign9210_e11114: f64 = (locals.var_dcln2 * locals.var_dzr1);
        let assign9210_e11115: f64 = (assign9210_e11114).exp();
        let assign9210_e11116: f64 = (1.0 - assign9210_e11115);
        let assign9210_e11117: f64 = (locals.var_dc_c * assign9210_e11116);
        let assign9210_e11119: f64 = (assign9210_e11117 / locals.var_dzr1);
        (assign9210_e11119, ((locals.var_dc_c * (-(assign9210_e11115 * (locals.var_dcln2_dn0 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9210_e11115 * (locals.var_dcln2_dn1 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9210_e11115 * (locals.var_dcln2_dn3 * locals.var_dzr1)))) / locals.var_dzr1), (((locals.var_dc_c_dn4 * assign9210_e11116) + (locals.var_dc_c * (-(assign9210_e11115 * (locals.var_dcln2_dn4 * locals.var_dzr1))))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9210_e11115 * (locals.var_dcln2_dn5 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9210_e11115 * (locals.var_dcln2_dn7 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9210_e11115 * (locals.var_dcln2_dn8 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9210_e11115 * (locals.var_dcln2_dn9 * locals.var_dzr1)))) / locals.var_dzr1),)
    } else {
        (locals.var_dq_j3, locals.var_dq_j3_dn0, locals.var_dq_j3_dn1, locals.var_dq_j3_dn3, locals.var_dq_j3_dn4, locals.var_dq_j3_dn5, locals.var_dq_j3_dn7, locals.var_dq_j3_dn8, locals.var_dq_j3_dn9,)
    }
};
        locals.var_dq_j3 = assign9210_e11121;
        locals.var_dq_j3_dn0 = assign9210_e11121_d_n0;
        locals.var_dq_j3_dn1 = assign9210_e11121_d_n1;
        locals.var_dq_j3_dn3 = assign9210_e11121_d_n3;
        locals.var_dq_j3_dn4 = assign9210_e11121_d_n4;
        locals.var_dq_j3_dn5 = assign9210_e11121_d_n5;
        locals.var_dq_j3_dn7 = assign9210_e11121_d_n7;
        locals.var_dq_j3_dn8 = assign9210_e11121_d_n8;
        locals.var_dq_j3_dn9 = assign9210_e11121_d_n9;
        locals.var_dq_j3_rv = 0.0;

        let (assign9220_e11137, assign9220_e11137_d_n0, assign9220_e11137_d_n1, assign9220_e11137_d_n3, assign9220_e11137_d_n4, assign9220_e11137_d_n5, assign9220_e11137_d_n6, assign9220_e11137_d_n7, assign9220_e11137_d_n8, assign9220_e11137_d_n9,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard199 != 0.0)) {
        let assign9220_e11127: f64 = (locals.var_dq_j1 + locals.var_dq_j2);
        let assign9220_e11129: f64 = (assign9220_e11127 - locals.var_dq_j3);
        let assign9220_e11131: f64 = (assign9220_e11129 * locals.var_vdcx_t);
        let assign9220_e11134: f64 = (locals.var_dc_max * locals.var_dv_j4);
        let assign9220_e11135: f64 = (assign9220_e11131 + assign9220_e11134);
        (assign9220_e11135, ((((locals.var_dq_j1_dn0 + locals.var_dq_j2_dn0) - locals.var_dq_j3_dn0) * locals.var_vdcx_t) + (locals.var_dc_max * locals.var_dv_j4_dn0)), ((((locals.var_dq_j1_dn1 + locals.var_dq_j2_dn1) - locals.var_dq_j3_dn1) * locals.var_vdcx_t) + (locals.var_dc_max * locals.var_dv_j4_dn1)), ((((locals.var_dq_j1_dn3 + locals.var_dq_j2_dn3) - locals.var_dq_j3_dn3) * locals.var_vdcx_t) + (locals.var_dc_max * locals.var_dv_j4_dn3)), (((((locals.var_dq_j1_dn4 + locals.var_dq_j2_dn4) - locals.var_dq_j3_dn4) * locals.var_vdcx_t) + (assign9220_e11129 * locals.var_vdcx_t_dn4)) + ((locals.var_dc_max_dn4 * locals.var_dv_j4) + (locals.var_dc_max * locals.var_dv_j4_dn4))), ((((locals.var_dq_j1_dn5 + locals.var_dq_j2_dn5) - locals.var_dq_j3_dn5) * locals.var_vdcx_t) + (locals.var_dc_max * locals.var_dv_j4_dn5)), 0.0, ((((locals.var_dq_j1_dn7 + locals.var_dq_j2_dn7) - locals.var_dq_j3_dn7) * locals.var_vdcx_t) + (locals.var_dc_max * locals.var_dv_j4_dn7)), ((((locals.var_dq_j1_dn8 + locals.var_dq_j2_dn8) - locals.var_dq_j3_dn8) * locals.var_vdcx_t) + (locals.var_dc_max * locals.var_dv_j4_dn8)), ((((locals.var_dq_j1_dn9 + locals.var_dq_j2_dn9) - locals.var_dq_j3_dn9) * locals.var_vdcx_t) + (locals.var_dc_max * locals.var_dv_j4_dn9)),)
    } else {
        (locals.var_qjcx0_t_x, locals.var_qjcx0_t_x_dn0, locals.var_qjcx0_t_x_dn1, locals.var_qjcx0_t_x_dn3, locals.var_qjcx0_t_x_dn4, locals.var_qjcx0_t_x_dn5, locals.var_qjcx0_t_x_dn6, locals.var_qjcx0_t_x_dn7, locals.var_qjcx0_t_x_dn8, locals.var_qjcx0_t_x_dn9,)
    }
};
        locals.var_qjcx0_t_x = assign9220_e11137;
        locals.var_qjcx0_t_x_dn0 = assign9220_e11137_d_n0;
        locals.var_qjcx0_t_x_dn1 = assign9220_e11137_d_n1;
        locals.var_qjcx0_t_x_dn3 = assign9220_e11137_d_n3;
        locals.var_qjcx0_t_x_dn4 = assign9220_e11137_d_n4;
        locals.var_qjcx0_t_x_dn5 = assign9220_e11137_d_n5;
        locals.var_qjcx0_t_x_dn6 = assign9220_e11137_d_n6;
        locals.var_qjcx0_t_x_dn7 = assign9220_e11137_d_n7;
        locals.var_qjcx0_t_x_dn8 = assign9220_e11137_d_n8;
        locals.var_qjcx0_t_x_dn9 = assign9220_e11137_d_n9;
        locals.var_qjcx0_t_x_rv = 0.0;

        let (assign9240_e11151, assign9240_e11151_d_n0, assign9240_e11151_d_n1, assign9240_e11151_d_n3, assign9240_e11151_d_n4, assign9240_e11151_d_n5, assign9240_e11151_d_n6, assign9240_e11151_d_n7, assign9240_e11151_d_n8, assign9240_e11151_d_n9,) = {
    if ((locals.var_guard198 != 0.0) && (locals.var_guard199 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qjcx0_t_x, locals.var_qjcx0_t_x_dn0, locals.var_qjcx0_t_x_dn1, locals.var_qjcx0_t_x_dn3, locals.var_qjcx0_t_x_dn4, locals.var_qjcx0_t_x_dn5, locals.var_qjcx0_t_x_dn6, locals.var_qjcx0_t_x_dn7, locals.var_qjcx0_t_x_dn8, locals.var_qjcx0_t_x_dn9,)
    }
};
        locals.var_qjcx0_t_x = assign9240_e11151;
        locals.var_qjcx0_t_x_dn0 = assign9240_e11151_d_n0;
        locals.var_qjcx0_t_x_dn1 = assign9240_e11151_d_n1;
        locals.var_qjcx0_t_x_dn3 = assign9240_e11151_d_n3;
        locals.var_qjcx0_t_x_dn4 = assign9240_e11151_d_n4;
        locals.var_qjcx0_t_x_dn5 = assign9240_e11151_d_n5;
        locals.var_qjcx0_t_x_dn6 = assign9240_e11151_d_n6;
        locals.var_qjcx0_t_x_dn7 = assign9240_e11151_d_n7;
        locals.var_qjcx0_t_x_dn8 = assign9240_e11151_d_n8;
        locals.var_qjcx0_t_x_dn9 = assign9240_e11151_d_n9;
        locals.var_qjcx0_t_x_rv = 0.0;

        let assign9250_e11154: f64 = if locals.var_cjcx01_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard202 = assign9250_e11154;
        locals.var_guard202_rv = 0.0;

        let (assign9260_e11170, assign9260_e11170_d_n4,) = {
    if ((locals.var_guard198 == 0.0) && (locals.var_guard202 != 0.0)) {
        let assign9260_e11162: f64 = (locals.var_ajcx_t).ln();
        let assign9260_e11163: f64 = (-assign9260_e11162);
        let assign9260_e11165: f64 = (assign9260_e11163 / p.p54);
        let assign9260_e11166: f64 = (assign9260_e11165).exp();
        let assign9260_e11167: f64 = (1.0 - assign9260_e11166);
        let assign9260_e11168: f64 = (locals.var_vdcx_t * assign9260_e11167);
        (assign9260_e11168, ((locals.var_vdcx_t_dn4 * assign9260_e11167) + (locals.var_vdcx_t * (-(assign9260_e11166 * ((-(locals.var_ajcx_t_dn4 / locals.var_ajcx_t)) / p.p54))))),)
    } else {
        (locals.var_dfv_f, locals.var_dfv_f_dn4,)
    }
};
        locals.var_dfv_f = assign9260_e11170;
        locals.var_dfv_f_dn4 = assign9260_e11170_d_n4;
        locals.var_dfv_f_rv = 0.0;

        let (assign9270_e11181, assign9270_e11181_d_n0, assign9270_e11181_d_n1, assign9270_e11181_d_n3, assign9270_e11181_d_n4, assign9270_e11181_d_n5, assign9270_e11181_d_n6, assign9270_e11181_d_n7, assign9270_e11181_d_n8, assign9270_e11181_d_n9,) = {
    if ((locals.var_guard198 == 0.0) && (locals.var_guard202 != 0.0)) {
        let assign9270_e11177: f64 = (locals.var_dfv_f - locals.var_vbci);
        let assign9270_e11179: f64 = (assign9270_e11177 * locals.var_ovt);
        (assign9270_e11179, 0.0, ((-locals.var_vbci_dn1) * locals.var_ovt), 0.0, ((locals.var_dfv_f_dn4 * locals.var_ovt) + (assign9270_e11177 * locals.var_ovt_dn4)), ((-locals.var_vbci_dn5) * locals.var_ovt), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dfx, locals.var_dfx_dn0, locals.var_dfx_dn1, locals.var_dfx_dn3, locals.var_dfx_dn4, locals.var_dfx_dn5, locals.var_dfx_dn6, locals.var_dfx_dn7, locals.var_dfx_dn8, locals.var_dfx_dn9,)
    }
};
        locals.var_dfx = assign9270_e11181;
        locals.var_dfx_dn0 = assign9270_e11181_d_n0;
        locals.var_dfx_dn1 = assign9270_e11181_d_n1;
        locals.var_dfx_dn3 = assign9270_e11181_d_n3;
        locals.var_dfx_dn4 = assign9270_e11181_d_n4;
        locals.var_dfx_dn5 = assign9270_e11181_d_n5;
        locals.var_dfx_dn6 = assign9270_e11181_d_n6;
        locals.var_dfx_dn7 = assign9270_e11181_d_n7;
        locals.var_dfx_dn8 = assign9270_e11181_d_n8;
        locals.var_dfx_dn9 = assign9270_e11181_d_n9;
        locals.var_dfx_rv = 0.0;

        let (assign9280_e11193, assign9280_e11193_d_n0, assign9280_e11193_d_n1, assign9280_e11193_d_n3, assign9280_e11193_d_n4, assign9280_e11193_d_n5, assign9280_e11193_d_n6, assign9280_e11193_d_n7, assign9280_e11193_d_n8, assign9280_e11193_d_n9,) = {
    if ((locals.var_guard198 == 0.0) && (locals.var_guard202 != 0.0)) {
        let assign9280_e11188: f64 = (locals.var_dfx * locals.var_dfx);
        let assign9280_e11190: f64 = (assign9280_e11188 + 1.921812);
        let assign9280_e11191: f64 = (assign9280_e11190).sqrt();
        (assign9280_e11191, (((locals.var_dfx_dn0 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn0)) / (2.0 * assign9280_e11191)), (((locals.var_dfx_dn1 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn1)) / (2.0 * assign9280_e11191)), (((locals.var_dfx_dn3 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn3)) / (2.0 * assign9280_e11191)), (((locals.var_dfx_dn4 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn4)) / (2.0 * assign9280_e11191)), (((locals.var_dfx_dn5 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn5)) / (2.0 * assign9280_e11191)), (((locals.var_dfx_dn6 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn6)) / (2.0 * assign9280_e11191)), (((locals.var_dfx_dn7 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn7)) / (2.0 * assign9280_e11191)), (((locals.var_dfx_dn8 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn8)) / (2.0 * assign9280_e11191)), (((locals.var_dfx_dn9 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn9)) / (2.0 * assign9280_e11191)),)
    } else {
        (locals.var_dfs_q, locals.var_dfs_q_dn0, locals.var_dfs_q_dn1, locals.var_dfs_q_dn3, locals.var_dfs_q_dn4, locals.var_dfs_q_dn5, locals.var_dfs_q_dn6, locals.var_dfs_q_dn7, locals.var_dfs_q_dn8, locals.var_dfs_q_dn9,)
    }
};
        locals.var_dfs_q = assign9280_e11193;
        locals.var_dfs_q_dn0 = assign9280_e11193_d_n0;
        locals.var_dfs_q_dn1 = assign9280_e11193_d_n1;
        locals.var_dfs_q_dn3 = assign9280_e11193_d_n3;
        locals.var_dfs_q_dn4 = assign9280_e11193_d_n4;
        locals.var_dfs_q_dn5 = assign9280_e11193_d_n5;
        locals.var_dfs_q_dn6 = assign9280_e11193_d_n6;
        locals.var_dfs_q_dn7 = assign9280_e11193_d_n7;
        locals.var_dfs_q_dn8 = assign9280_e11193_d_n8;
        locals.var_dfs_q_dn9 = assign9280_e11193_d_n9;
        locals.var_dfs_q_rv = 0.0;

        let (assign9290_e11204, assign9290_e11204_d_n0, assign9290_e11204_d_n1, assign9290_e11204_d_n3, assign9290_e11204_d_n4, assign9290_e11204_d_n5, assign9290_e11204_d_n6, assign9290_e11204_d_n7, assign9290_e11204_d_n8, assign9290_e11204_d_n9,) = {
    if ((locals.var_guard198 == 0.0) && (locals.var_guard202 != 0.0)) {
        let assign9290_e11200: f64 = (locals.var_dfx + locals.var_dfs_q);
        let assign9290_e11202: f64 = (assign9290_e11200 * 0.5);
        (assign9290_e11202, ((locals.var_dfx_dn0 + locals.var_dfs_q_dn0) * 0.5), ((locals.var_dfx_dn1 + locals.var_dfs_q_dn1) * 0.5), ((locals.var_dfx_dn3 + locals.var_dfs_q_dn3) * 0.5), ((locals.var_dfx_dn4 + locals.var_dfs_q_dn4) * 0.5), ((locals.var_dfx_dn5 + locals.var_dfs_q_dn5) * 0.5), ((locals.var_dfx_dn6 + locals.var_dfs_q_dn6) * 0.5), ((locals.var_dfx_dn7 + locals.var_dfs_q_dn7) * 0.5), ((locals.var_dfx_dn8 + locals.var_dfs_q_dn8) * 0.5), ((locals.var_dfx_dn9 + locals.var_dfs_q_dn9) * 0.5),)
    } else {
        (locals.var_dfs_q2, locals.var_dfs_q2_dn0, locals.var_dfs_q2_dn1, locals.var_dfs_q2_dn3, locals.var_dfs_q2_dn4, locals.var_dfs_q2_dn5, locals.var_dfs_q2_dn6, locals.var_dfs_q2_dn7, locals.var_dfs_q2_dn8, locals.var_dfs_q2_dn9,)
    }
};
        locals.var_dfs_q2 = assign9290_e11204;
        locals.var_dfs_q2_dn0 = assign9290_e11204_d_n0;
        locals.var_dfs_q2_dn1 = assign9290_e11204_d_n1;
        locals.var_dfs_q2_dn3 = assign9290_e11204_d_n3;
        locals.var_dfs_q2_dn4 = assign9290_e11204_d_n4;
        locals.var_dfs_q2_dn5 = assign9290_e11204_d_n5;
        locals.var_dfs_q2_dn6 = assign9290_e11204_d_n6;
        locals.var_dfs_q2_dn7 = assign9290_e11204_d_n7;
        locals.var_dfs_q2_dn8 = assign9290_e11204_d_n8;
        locals.var_dfs_q2_dn9 = assign9290_e11204_d_n9;
        locals.var_dfs_q2_rv = 0.0;

        let (assign9300_e11215, assign9300_e11215_d_n0, assign9300_e11215_d_n1, assign9300_e11215_d_n3, assign9300_e11215_d_n4, assign9300_e11215_d_n5, assign9300_e11215_d_n6, assign9300_e11215_d_n7, assign9300_e11215_d_n8, assign9300_e11215_d_n9,) = {
    if ((locals.var_guard198 == 0.0) && (locals.var_guard202 != 0.0)) {
        let assign9300_e11212: f64 = (locals.var_vt * locals.var_dfs_q2);
        let assign9300_e11213: f64 = (locals.var_dfv_f - assign9300_e11212);
        (assign9300_e11213, (-(locals.var_vt * locals.var_dfs_q2_dn0)), (-(locals.var_vt * locals.var_dfs_q2_dn1)), (-(locals.var_vt * locals.var_dfs_q2_dn3)), (locals.var_dfv_f_dn4 - ((locals.var_vt_dn4 * locals.var_dfs_q2) + (locals.var_vt * locals.var_dfs_q2_dn4))), (-(locals.var_vt * locals.var_dfs_q2_dn5)), (-(locals.var_vt * locals.var_dfs_q2_dn6)), (-(locals.var_vt * locals.var_dfs_q2_dn7)), (-(locals.var_vt * locals.var_dfs_q2_dn8)), (-(locals.var_vt * locals.var_dfs_q2_dn9)),)
    } else {
        (locals.var_dfv_j, locals.var_dfv_j_dn0, locals.var_dfv_j_dn1, locals.var_dfv_j_dn3, locals.var_dfv_j_dn4, locals.var_dfv_j_dn5, locals.var_dfv_j_dn6, locals.var_dfv_j_dn7, locals.var_dfv_j_dn8, locals.var_dfv_j_dn9,)
    }
};
        locals.var_dfv_j = assign9300_e11215;
        locals.var_dfv_j_dn0 = assign9300_e11215_d_n0;
        locals.var_dfv_j_dn1 = assign9300_e11215_d_n1;
        locals.var_dfv_j_dn3 = assign9300_e11215_d_n3;
        locals.var_dfv_j_dn4 = assign9300_e11215_d_n4;
        locals.var_dfv_j_dn5 = assign9300_e11215_d_n5;
        locals.var_dfv_j_dn6 = assign9300_e11215_d_n6;
        locals.var_dfv_j_dn7 = assign9300_e11215_d_n7;
        locals.var_dfv_j_dn8 = assign9300_e11215_d_n8;
        locals.var_dfv_j_dn9 = assign9300_e11215_d_n9;
        locals.var_dfv_j_rv = 0.0;

        let (assign9310_e11224, assign9310_e11224_d_n0, assign9310_e11224_d_n1, assign9310_e11224_d_n3, assign9310_e11224_d_n4, assign9310_e11224_d_n5, assign9310_e11224_d_n6, assign9310_e11224_d_n7, assign9310_e11224_d_n8, assign9310_e11224_d_n9,) = {
    if ((locals.var_guard198 == 0.0) && (locals.var_guard202 != 0.0)) {
        let assign9310_e11222: f64 = (locals.var_dfs_q2 / locals.var_dfs_q);
        (assign9310_e11222, (((locals.var_dfs_q2_dn0 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn0)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn1 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn1)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn3 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn3)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn4 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn4)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn5 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn5)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn6 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn6)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn7 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn7)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn8 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn8)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn9 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn9)) / (locals.var_dfs_q * locals.var_dfs_q)),)
    } else {
        (locals.var_dfdvj_dv, locals.var_dfdvj_dv_dn0, locals.var_dfdvj_dv_dn1, locals.var_dfdvj_dv_dn3, locals.var_dfdvj_dv_dn4, locals.var_dfdvj_dv_dn5, locals.var_dfdvj_dv_dn6, locals.var_dfdvj_dv_dn7, locals.var_dfdvj_dv_dn8, locals.var_dfdvj_dv_dn9,)
    }
};
        locals.var_dfdvj_dv = assign9310_e11224;
        locals.var_dfdvj_dv_dn0 = assign9310_e11224_d_n0;
        locals.var_dfdvj_dv_dn1 = assign9310_e11224_d_n1;
        locals.var_dfdvj_dv_dn3 = assign9310_e11224_d_n3;
        locals.var_dfdvj_dv_dn4 = assign9310_e11224_d_n4;
        locals.var_dfdvj_dv_dn5 = assign9310_e11224_d_n5;
        locals.var_dfdvj_dv_dn6 = assign9310_e11224_d_n6;
        locals.var_dfdvj_dv_dn7 = assign9310_e11224_d_n7;
        locals.var_dfdvj_dv_dn8 = assign9310_e11224_d_n8;
        locals.var_dfdvj_dv_dn9 = assign9310_e11224_d_n9;
        locals.var_dfdvj_dv_rv = 0.0;

        let (assign9320_e11236, assign9320_e11236_d_n0, assign9320_e11236_d_n1, assign9320_e11236_d_n3, assign9320_e11236_d_n4, assign9320_e11236_d_n5, assign9320_e11236_d_n6, assign9320_e11236_d_n7, assign9320_e11236_d_n8, assign9320_e11236_d_n9,) = {
    if ((locals.var_guard198 == 0.0) && (locals.var_guard202 != 0.0)) {
        let assign9320_e11232: f64 = (locals.var_dfv_j / locals.var_vdcx_t);
        let assign9320_e11233: f64 = (1.0 - assign9320_e11232);
        let assign9320_e11234: f64 = (assign9320_e11233).ln();
        (assign9320_e11234, ((-(locals.var_dfv_j_dn0 / locals.var_vdcx_t)) / assign9320_e11233), ((-(locals.var_dfv_j_dn1 / locals.var_vdcx_t)) / assign9320_e11233), ((-(locals.var_dfv_j_dn3 / locals.var_vdcx_t)) / assign9320_e11233), ((-(((locals.var_dfv_j_dn4 * locals.var_vdcx_t) - (locals.var_dfv_j * locals.var_vdcx_t_dn4)) / (locals.var_vdcx_t * locals.var_vdcx_t))) / assign9320_e11233), ((-(locals.var_dfv_j_dn5 / locals.var_vdcx_t)) / assign9320_e11233), ((-(locals.var_dfv_j_dn6 / locals.var_vdcx_t)) / assign9320_e11233), ((-(locals.var_dfv_j_dn7 / locals.var_vdcx_t)) / assign9320_e11233), ((-(locals.var_dfv_j_dn8 / locals.var_vdcx_t)) / assign9320_e11233), ((-(locals.var_dfv_j_dn9 / locals.var_vdcx_t)) / assign9320_e11233),)
    } else {
        (locals.var_dfb, locals.var_dfb_dn0, locals.var_dfb_dn1, locals.var_dfb_dn3, locals.var_dfb_dn4, locals.var_dfb_dn5, locals.var_dfb_dn6, locals.var_dfb_dn7, locals.var_dfb_dn8, locals.var_dfb_dn9,)
    }
};
        locals.var_dfb = assign9320_e11236;
        locals.var_dfb_dn0 = assign9320_e11236_d_n0;
        locals.var_dfb_dn1 = assign9320_e11236_d_n1;
        locals.var_dfb_dn3 = assign9320_e11236_d_n3;
        locals.var_dfb_dn4 = assign9320_e11236_d_n4;
        locals.var_dfb_dn5 = assign9320_e11236_d_n5;
        locals.var_dfb_dn6 = assign9320_e11236_d_n6;
        locals.var_dfb_dn7 = assign9320_e11236_d_n7;
        locals.var_dfb_dn8 = assign9320_e11236_d_n8;
        locals.var_dfb_dn9 = assign9320_e11236_d_n9;
        locals.var_dfb_rv = 0.0;

        let (assign9330_e11249, assign9330_e11249_d_n0, assign9330_e11249_d_n1, assign9330_e11249_d_n3, assign9330_e11249_d_n4, assign9330_e11249_d_n5, assign9330_e11249_d_n6, assign9330_e11249_d_n7, assign9330_e11249_d_n8, assign9330_e11249_d_n9,) = {
    if ((locals.var_guard198 == 0.0) && (locals.var_guard202 != 0.0)) {
        let assign9330_e11242: f64 = (-p.p54);
        let assign9330_e11244: f64 = (assign9330_e11242 * locals.var_dfb);
        let assign9330_e11245: f64 = (assign9330_e11244).exp();
        let assign9330_e11247: f64 = (assign9330_e11245 * locals.var_dfdvj_dv);
        (assign9330_e11247, (((assign9330_e11245 * (assign9330_e11242 * locals.var_dfb_dn0)) * locals.var_dfdvj_dv) + (assign9330_e11245 * locals.var_dfdvj_dv_dn0)), (((assign9330_e11245 * (assign9330_e11242 * locals.var_dfb_dn1)) * locals.var_dfdvj_dv) + (assign9330_e11245 * locals.var_dfdvj_dv_dn1)), (((assign9330_e11245 * (assign9330_e11242 * locals.var_dfb_dn3)) * locals.var_dfdvj_dv) + (assign9330_e11245 * locals.var_dfdvj_dv_dn3)), (((assign9330_e11245 * (assign9330_e11242 * locals.var_dfb_dn4)) * locals.var_dfdvj_dv) + (assign9330_e11245 * locals.var_dfdvj_dv_dn4)), (((assign9330_e11245 * (assign9330_e11242 * locals.var_dfb_dn5)) * locals.var_dfdvj_dv) + (assign9330_e11245 * locals.var_dfdvj_dv_dn5)), (((assign9330_e11245 * (assign9330_e11242 * locals.var_dfb_dn6)) * locals.var_dfdvj_dv) + (assign9330_e11245 * locals.var_dfdvj_dv_dn6)), (((assign9330_e11245 * (assign9330_e11242 * locals.var_dfb_dn7)) * locals.var_dfdvj_dv) + (assign9330_e11245 * locals.var_dfdvj_dv_dn7)), (((assign9330_e11245 * (assign9330_e11242 * locals.var_dfb_dn8)) * locals.var_dfdvj_dv) + (assign9330_e11245 * locals.var_dfdvj_dv_dn8)), (((assign9330_e11245 * (assign9330_e11242 * locals.var_dfb_dn9)) * locals.var_dfdvj_dv) + (assign9330_e11245 * locals.var_dfdvj_dv_dn9)),)
    } else {
        (locals.var_dfc_j1, locals.var_dfc_j1_dn0, locals.var_dfc_j1_dn1, locals.var_dfc_j1_dn3, locals.var_dfc_j1_dn4, locals.var_dfc_j1_dn5, locals.var_dfc_j1_dn6, locals.var_dfc_j1_dn7, locals.var_dfc_j1_dn8, locals.var_dfc_j1_dn9,)
    }
};
        locals.var_dfc_j1 = assign9330_e11249;
        locals.var_dfc_j1_dn0 = assign9330_e11249_d_n0;
        locals.var_dfc_j1_dn1 = assign9330_e11249_d_n1;
        locals.var_dfc_j1_dn3 = assign9330_e11249_d_n3;
        locals.var_dfc_j1_dn4 = assign9330_e11249_d_n4;
        locals.var_dfc_j1_dn5 = assign9330_e11249_d_n5;
        locals.var_dfc_j1_dn6 = assign9330_e11249_d_n6;
        locals.var_dfc_j1_dn7 = assign9330_e11249_d_n7;
        locals.var_dfc_j1_dn8 = assign9330_e11249_d_n8;
        locals.var_dfc_j1_dn9 = assign9330_e11249_d_n9;
        locals.var_dfc_j1_rv = 0.0;

        let (assign9350_e11284, assign9350_e11284_d_n0, assign9350_e11284_d_n1, assign9350_e11284_d_n3, assign9350_e11284_d_n4, assign9350_e11284_d_n5, assign9350_e11284_d_n6, assign9350_e11284_d_n7, assign9350_e11284_d_n8, assign9350_e11284_d_n9,) = {
    if ((locals.var_guard198 == 0.0) && (locals.var_guard202 != 0.0)) {
        let assign9350_e11274: f64 = (1.0 - p.p54);
        let assign9350_e11275: f64 = (locals.var_dfb * assign9350_e11274);
        let assign9350_e11276: f64 = (assign9350_e11275).exp();
        let assign9350_e11277: f64 = (1.0 - assign9350_e11276);
        let assign9350_e11278: f64 = (locals.var_vdcx_t * assign9350_e11277);
        let assign9350_e11281: f64 = (1.0 - p.p54);
        let assign9350_e11282: f64 = (assign9350_e11278 / assign9350_e11281);
        (assign9350_e11282, ((locals.var_vdcx_t * (-(assign9350_e11276 * (locals.var_dfb_dn0 * assign9350_e11274)))) / assign9350_e11281), ((locals.var_vdcx_t * (-(assign9350_e11276 * (locals.var_dfb_dn1 * assign9350_e11274)))) / assign9350_e11281), ((locals.var_vdcx_t * (-(assign9350_e11276 * (locals.var_dfb_dn3 * assign9350_e11274)))) / assign9350_e11281), (((locals.var_vdcx_t_dn4 * assign9350_e11277) + (locals.var_vdcx_t * (-(assign9350_e11276 * (locals.var_dfb_dn4 * assign9350_e11274))))) / assign9350_e11281), ((locals.var_vdcx_t * (-(assign9350_e11276 * (locals.var_dfb_dn5 * assign9350_e11274)))) / assign9350_e11281), ((locals.var_vdcx_t * (-(assign9350_e11276 * (locals.var_dfb_dn6 * assign9350_e11274)))) / assign9350_e11281), ((locals.var_vdcx_t * (-(assign9350_e11276 * (locals.var_dfb_dn7 * assign9350_e11274)))) / assign9350_e11281), ((locals.var_vdcx_t * (-(assign9350_e11276 * (locals.var_dfb_dn8 * assign9350_e11274)))) / assign9350_e11281), ((locals.var_vdcx_t * (-(assign9350_e11276 * (locals.var_dfb_dn9 * assign9350_e11274)))) / assign9350_e11281),)
    } else {
        (locals.var_dfq_j1, locals.var_dfq_j1_dn0, locals.var_dfq_j1_dn1, locals.var_dfq_j1_dn3, locals.var_dfq_j1_dn4, locals.var_dfq_j1_dn5, locals.var_dfq_j1_dn6, locals.var_dfq_j1_dn7, locals.var_dfq_j1_dn8, locals.var_dfq_j1_dn9,)
    }
};
        locals.var_dfq_j1 = assign9350_e11284;
        locals.var_dfq_j1_dn0 = assign9350_e11284_d_n0;
        locals.var_dfq_j1_dn1 = assign9350_e11284_d_n1;
        locals.var_dfq_j1_dn3 = assign9350_e11284_d_n3;
        locals.var_dfq_j1_dn4 = assign9350_e11284_d_n4;
        locals.var_dfq_j1_dn5 = assign9350_e11284_d_n5;
        locals.var_dfq_j1_dn6 = assign9350_e11284_d_n6;
        locals.var_dfq_j1_dn7 = assign9350_e11284_d_n7;
        locals.var_dfq_j1_dn8 = assign9350_e11284_d_n8;
        locals.var_dfq_j1_dn9 = assign9350_e11284_d_n9;
        locals.var_dfq_j1_rv = 0.0;

        let (assign9360_e11299, assign9360_e11299_d_n0, assign9360_e11299_d_n1, assign9360_e11299_d_n3, assign9360_e11299_d_n4, assign9360_e11299_d_n5, assign9360_e11299_d_n6, assign9360_e11299_d_n7, assign9360_e11299_d_n8, assign9360_e11299_d_n9,) = {
    if ((locals.var_guard198 == 0.0) && (locals.var_guard202 != 0.0)) {
        let assign9360_e11294: f64 = (locals.var_vbci - locals.var_dfv_j);
        let assign9360_e11295: f64 = (locals.var_ajcx_t * assign9360_e11294);
        let assign9360_e11296: f64 = (locals.var_dfq_j1 + assign9360_e11295);
        let assign9360_e11297: f64 = (locals.var_cjcx01_t * assign9360_e11296);
        (assign9360_e11297, (locals.var_cjcx01_t * (locals.var_dfq_j1_dn0 + (locals.var_ajcx_t * (-locals.var_dfv_j_dn0)))), (locals.var_cjcx01_t * (locals.var_dfq_j1_dn1 + (locals.var_ajcx_t * (locals.var_vbci_dn1 - locals.var_dfv_j_dn1)))), (locals.var_cjcx01_t * (locals.var_dfq_j1_dn3 + (locals.var_ajcx_t * (-locals.var_dfv_j_dn3)))), ((locals.var_cjcx01_t_dn4 * assign9360_e11296) + (locals.var_cjcx01_t * (locals.var_dfq_j1_dn4 + ((locals.var_ajcx_t_dn4 * assign9360_e11294) + (locals.var_ajcx_t * (-locals.var_dfv_j_dn4)))))), (locals.var_cjcx01_t * (locals.var_dfq_j1_dn5 + (locals.var_ajcx_t * (locals.var_vbci_dn5 - locals.var_dfv_j_dn5)))), (locals.var_cjcx01_t * (locals.var_dfq_j1_dn6 + (locals.var_ajcx_t * (-locals.var_dfv_j_dn6)))), (locals.var_cjcx01_t * (locals.var_dfq_j1_dn7 + (locals.var_ajcx_t * (-locals.var_dfv_j_dn7)))), (locals.var_cjcx01_t * (locals.var_dfq_j1_dn8 + (locals.var_ajcx_t * (-locals.var_dfv_j_dn8)))), (locals.var_cjcx01_t * (locals.var_dfq_j1_dn9 + (locals.var_ajcx_t * (-locals.var_dfv_j_dn9)))),)
    } else {
        (locals.var_qjcx0_t_x, locals.var_qjcx0_t_x_dn0, locals.var_qjcx0_t_x_dn1, locals.var_qjcx0_t_x_dn3, locals.var_qjcx0_t_x_dn4, locals.var_qjcx0_t_x_dn5, locals.var_qjcx0_t_x_dn6, locals.var_qjcx0_t_x_dn7, locals.var_qjcx0_t_x_dn8, locals.var_qjcx0_t_x_dn9,)
    }
};
        locals.var_qjcx0_t_x = assign9360_e11299;
        locals.var_qjcx0_t_x_dn0 = assign9360_e11299_d_n0;
        locals.var_qjcx0_t_x_dn1 = assign9360_e11299_d_n1;
        locals.var_qjcx0_t_x_dn3 = assign9360_e11299_d_n3;
        locals.var_qjcx0_t_x_dn4 = assign9360_e11299_d_n4;
        locals.var_qjcx0_t_x_dn5 = assign9360_e11299_d_n5;
        locals.var_qjcx0_t_x_dn6 = assign9360_e11299_d_n6;
        locals.var_qjcx0_t_x_dn7 = assign9360_e11299_d_n7;
        locals.var_qjcx0_t_x_dn8 = assign9360_e11299_d_n8;
        locals.var_qjcx0_t_x_dn9 = assign9360_e11299_d_n9;
        locals.var_qjcx0_t_x_rv = 0.0;

        let (assign9380_e11315, assign9380_e11315_d_n0, assign9380_e11315_d_n1, assign9380_e11315_d_n3, assign9380_e11315_d_n4, assign9380_e11315_d_n5, assign9380_e11315_d_n6, assign9380_e11315_d_n7, assign9380_e11315_d_n8, assign9380_e11315_d_n9,) = {
    if ((locals.var_guard198 == 0.0) && (locals.var_guard202 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qjcx0_t_x, locals.var_qjcx0_t_x_dn0, locals.var_qjcx0_t_x_dn1, locals.var_qjcx0_t_x_dn3, locals.var_qjcx0_t_x_dn4, locals.var_qjcx0_t_x_dn5, locals.var_qjcx0_t_x_dn6, locals.var_qjcx0_t_x_dn7, locals.var_qjcx0_t_x_dn8, locals.var_qjcx0_t_x_dn9,)
    }
};
        locals.var_qjcx0_t_x = assign9380_e11315;
        locals.var_qjcx0_t_x_dn0 = assign9380_e11315_d_n0;
        locals.var_qjcx0_t_x_dn1 = assign9380_e11315_d_n1;
        locals.var_qjcx0_t_x_dn3 = assign9380_e11315_d_n3;
        locals.var_qjcx0_t_x_dn4 = assign9380_e11315_d_n4;
        locals.var_qjcx0_t_x_dn5 = assign9380_e11315_d_n5;
        locals.var_qjcx0_t_x_dn6 = assign9380_e11315_d_n6;
        locals.var_qjcx0_t_x_dn7 = assign9380_e11315_d_n7;
        locals.var_qjcx0_t_x_dn8 = assign9380_e11315_d_n8;
        locals.var_qjcx0_t_x_dn9 = assign9380_e11315_d_n9;
        locals.var_qjcx0_t_x_rv = 0.0;

        let assign9390_e11318: f64 = if p.p61 < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard203 = assign9390_e11318;
        locals.var_guard203_rv = 0.0;

        let assign9400_e11321: f64 = if locals.var_cjs0_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard204 = assign9400_e11321;
        locals.var_guard204_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_25(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9410_e11329,) = {
    if ((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign9410_e11327: f64 = (p.p59 / 4.0);
        (assign9410_e11327,)
    } else {
        (locals.var_dz_r,)
    }
};
        locals.var_dz_r = assign9410_e11329;
        locals.var_dz_r_rv = 0.0;

        let (assign9420_e11337, assign9420_e11337_d_n4,) = {
    if ((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign9420_e11335: f64 = (p.p61 - locals.var_vds_t);
        (assign9420_e11335, (-locals.var_vds_t_dn4),)
    } else {
        (locals.var_dv_p, locals.var_dv_p_dn4,)
    }
};
        locals.var_dv_p = assign9420_e11337;
        locals.var_dv_p_dn4 = assign9420_e11337_d_n4;
        locals.var_dv_p_rv = 0.0;

        let (assign9430_e11352, assign9430_e11352_d_n4,) = {
    if ((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign9430_e11344: f64 = (locals.var_ajs_t).ln();
        let assign9430_e11345: f64 = (-assign9430_e11344);
        let assign9430_e11347: f64 = (assign9430_e11345 / p.p59);
        let assign9430_e11348: f64 = (assign9430_e11347).exp();
        let assign9430_e11349: f64 = (1.0 - assign9430_e11348);
        let assign9430_e11350: f64 = (locals.var_vds_t * assign9430_e11349);
        (assign9430_e11350, ((locals.var_vds_t_dn4 * assign9430_e11349) + (locals.var_vds_t * (-(assign9430_e11348 * ((-(locals.var_ajs_t_dn4 / locals.var_ajs_t)) / p.p59))))),)
    } else {
        (locals.var_dv_f, locals.var_dv_f_dn4,)
    }
};
        locals.var_dv_f = assign9430_e11352;
        locals.var_dv_f_dn4 = assign9430_e11352_d_n4;
        locals.var_dv_f_rv = 0.0;

        let (assign9440_e11360, assign9440_e11360_d_n4,) = {
    if ((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign9440_e11358: f64 = (locals.var_ajs_t * locals.var_cjs0_t);
        (assign9440_e11358, ((locals.var_ajs_t_dn4 * locals.var_cjs0_t) + (locals.var_ajs_t * locals.var_cjs0_t_dn4)),)
    } else {
        (locals.var_dc_max, locals.var_dc_max_dn4,)
    }
};
        locals.var_dc_max = assign9440_e11360;
        locals.var_dc_max_dn4 = assign9440_e11360_d_n4;
        locals.var_dc_max_rv = 0.0;

        let (assign9450_e11376, assign9450_e11376_d_n4,) = {
    if ((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign9450_e11367: f64 = (locals.var_dz_r - p.p59);
        let assign9450_e11370: f64 = (p.p61 / locals.var_vds_t);
        let assign9450_e11371: f64 = (assign9450_e11370).ln();
        let assign9450_e11372: f64 = (assign9450_e11367 * assign9450_e11371);
        let assign9450_e11373: f64 = (assign9450_e11372).exp();
        let assign9450_e11374: f64 = (locals.var_cjs0_t * assign9450_e11373);
        (assign9450_e11374, ((locals.var_cjs0_t_dn4 * assign9450_e11373) + (locals.var_cjs0_t * (assign9450_e11373 * (assign9450_e11367 * ((-((p.p61 * locals.var_vds_t_dn4) / (locals.var_vds_t * locals.var_vds_t))) / assign9450_e11370))))),)
    } else {
        (locals.var_dc_c, locals.var_dc_c_dn4,)
    }
};
        locals.var_dc_c = assign9450_e11376;
        locals.var_dc_c_dn4 = assign9450_e11376_d_n4;
        locals.var_dc_c_rv = 0.0;

        let (assign9460_e11386, assign9460_e11386_d_n0, assign9460_e11386_d_n1, assign9460_e11386_d_n3, assign9460_e11386_d_n4, assign9460_e11386_d_n5, assign9460_e11386_d_n7, assign9460_e11386_d_n8, assign9460_e11386_d_n9,) = {
    if ((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign9460_e11382: f64 = (locals.var_dv_f - locals.var_vsici);
        let assign9460_e11384: f64 = (assign9460_e11382 * locals.var_ovt);
        (assign9460_e11384, 0.0, 0.0, 0.0, ((locals.var_dv_f_dn4 * locals.var_ovt) + (assign9460_e11382 * locals.var_ovt_dn4)), ((-locals.var_vsici_dn5) * locals.var_ovt), 0.0, 0.0, ((-locals.var_vsici_dn9) * locals.var_ovt),)
    } else {
        (locals.var_dv_e, locals.var_dv_e_dn0, locals.var_dv_e_dn1, locals.var_dv_e_dn3, locals.var_dv_e_dn4, locals.var_dv_e_dn5, locals.var_dv_e_dn7, locals.var_dv_e_dn8, locals.var_dv_e_dn9,)
    }
};
        locals.var_dv_e = assign9460_e11386;
        locals.var_dv_e_dn0 = assign9460_e11386_d_n0;
        locals.var_dv_e_dn1 = assign9460_e11386_d_n1;
        locals.var_dv_e_dn3 = assign9460_e11386_d_n3;
        locals.var_dv_e_dn4 = assign9460_e11386_d_n4;
        locals.var_dv_e_dn5 = assign9460_e11386_d_n5;
        locals.var_dv_e_dn7 = assign9460_e11386_d_n7;
        locals.var_dv_e_dn8 = assign9460_e11386_d_n8;
        locals.var_dv_e_dn9 = assign9460_e11386_d_n9;
        locals.var_dv_e_rv = 0.0;

        let assign9470_e11389: f64 = if locals.var_dv_e < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard205 = assign9470_e11389;
        locals.var_guard205_rv = 0.0;

        let (assign9480_e11398, assign9480_e11398_d_n0, assign9480_e11398_d_n1, assign9480_e11398_d_n3, assign9480_e11398_d_n4, assign9480_e11398_d_n5, assign9480_e11398_d_n7, assign9480_e11398_d_n8, assign9480_e11398_d_n9,) = {
    if (((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) && (locals.var_guard205 != 0.0)) {
        let assign9480_e11396: f64 = (locals.var_dv_e).exp();
        (assign9480_e11396, (assign9480_e11396 * locals.var_dv_e_dn0), (assign9480_e11396 * locals.var_dv_e_dn1), (assign9480_e11396 * locals.var_dv_e_dn3), (assign9480_e11396 * locals.var_dv_e_dn4), (assign9480_e11396 * locals.var_dv_e_dn5), (assign9480_e11396 * locals.var_dv_e_dn7), (assign9480_e11396 * locals.var_dv_e_dn8), (assign9480_e11396 * locals.var_dv_e_dn9),)
    } else {
        (locals.var_de, locals.var_de_dn0, locals.var_de_dn1, locals.var_de_dn3, locals.var_de_dn4, locals.var_de_dn5, locals.var_de_dn7, locals.var_de_dn8, locals.var_de_dn9,)
    }
};
        locals.var_de = assign9480_e11398;
        locals.var_de_dn0 = assign9480_e11398_d_n0;
        locals.var_de_dn1 = assign9480_e11398_d_n1;
        locals.var_de_dn3 = assign9480_e11398_d_n3;
        locals.var_de_dn4 = assign9480_e11398_d_n4;
        locals.var_de_dn5 = assign9480_e11398_d_n5;
        locals.var_de_dn7 = assign9480_e11398_d_n7;
        locals.var_de_dn8 = assign9480_e11398_d_n8;
        locals.var_de_dn9 = assign9480_e11398_d_n9;
        locals.var_de_rv = 0.0;

        let (assign9490_e11410, assign9490_e11410_d_n0, assign9490_e11410_d_n1, assign9490_e11410_d_n3, assign9490_e11410_d_n4, assign9490_e11410_d_n5, assign9490_e11410_d_n7, assign9490_e11410_d_n8, assign9490_e11410_d_n9,) = {
    if (((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) && (locals.var_guard205 != 0.0)) {
        let assign9490_e11407: f64 = (1.0 + locals.var_de);
        let assign9490_e11408: f64 = (locals.var_de / assign9490_e11407);
        (assign9490_e11408, (((locals.var_de_dn0 * assign9490_e11407) - (locals.var_de * locals.var_de_dn0)) / (assign9490_e11407 * assign9490_e11407)), (((locals.var_de_dn1 * assign9490_e11407) - (locals.var_de * locals.var_de_dn1)) / (assign9490_e11407 * assign9490_e11407)), (((locals.var_de_dn3 * assign9490_e11407) - (locals.var_de * locals.var_de_dn3)) / (assign9490_e11407 * assign9490_e11407)), (((locals.var_de_dn4 * assign9490_e11407) - (locals.var_de * locals.var_de_dn4)) / (assign9490_e11407 * assign9490_e11407)), (((locals.var_de_dn5 * assign9490_e11407) - (locals.var_de * locals.var_de_dn5)) / (assign9490_e11407 * assign9490_e11407)), (((locals.var_de_dn7 * assign9490_e11407) - (locals.var_de * locals.var_de_dn7)) / (assign9490_e11407 * assign9490_e11407)), (((locals.var_de_dn8 * assign9490_e11407) - (locals.var_de * locals.var_de_dn8)) / (assign9490_e11407 * assign9490_e11407)), (((locals.var_de_dn9 * assign9490_e11407) - (locals.var_de * locals.var_de_dn9)) / (assign9490_e11407 * assign9490_e11407)),)
    } else {
        (locals.var_de_1, locals.var_de_1_dn0, locals.var_de_1_dn1, locals.var_de_1_dn3, locals.var_de_1_dn4, locals.var_de_1_dn5, locals.var_de_1_dn7, locals.var_de_1_dn8, locals.var_de_1_dn9,)
    }
};
        locals.var_de_1 = assign9490_e11410;
        locals.var_de_1_dn0 = assign9490_e11410_d_n0;
        locals.var_de_1_dn1 = assign9490_e11410_d_n1;
        locals.var_de_1_dn3 = assign9490_e11410_d_n3;
        locals.var_de_1_dn4 = assign9490_e11410_d_n4;
        locals.var_de_1_dn5 = assign9490_e11410_d_n5;
        locals.var_de_1_dn7 = assign9490_e11410_d_n7;
        locals.var_de_1_dn8 = assign9490_e11410_d_n8;
        locals.var_de_1_dn9 = assign9490_e11410_d_n9;
        locals.var_de_1_rv = 0.0;

        let (assign9500_e11425, assign9500_e11425_d_n0, assign9500_e11425_d_n1, assign9500_e11425_d_n3, assign9500_e11425_d_n4, assign9500_e11425_d_n5, assign9500_e11425_d_n7, assign9500_e11425_d_n8, assign9500_e11425_d_n9,) = {
    if (((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) && (locals.var_guard205 != 0.0)) {
        let assign9500_e11420: f64 = (1.0 + locals.var_de);
        let assign9500_e11421: f64 = (assign9500_e11420).ln();
        let assign9500_e11422: f64 = (locals.var_vt * assign9500_e11421);
        let assign9500_e11423: f64 = (locals.var_dv_f - assign9500_e11422);
        (assign9500_e11423, (-(locals.var_vt * (locals.var_de_dn0 / assign9500_e11420))), (-(locals.var_vt * (locals.var_de_dn1 / assign9500_e11420))), (-(locals.var_vt * (locals.var_de_dn3 / assign9500_e11420))), (locals.var_dv_f_dn4 - ((locals.var_vt_dn4 * assign9500_e11421) + (locals.var_vt * (locals.var_de_dn4 / assign9500_e11420)))), (-(locals.var_vt * (locals.var_de_dn5 / assign9500_e11420))), (-(locals.var_vt * (locals.var_de_dn7 / assign9500_e11420))), (-(locals.var_vt * (locals.var_de_dn8 / assign9500_e11420))), (-(locals.var_vt * (locals.var_de_dn9 / assign9500_e11420))),)
    } else {
        (locals.var_dv_j1, locals.var_dv_j1_dn0, locals.var_dv_j1_dn1, locals.var_dv_j1_dn3, locals.var_dv_j1_dn4, locals.var_dv_j1_dn5, locals.var_dv_j1_dn7, locals.var_dv_j1_dn8, locals.var_dv_j1_dn9,)
    }
};
        locals.var_dv_j1 = assign9500_e11425;
        locals.var_dv_j1_dn0 = assign9500_e11425_d_n0;
        locals.var_dv_j1_dn1 = assign9500_e11425_d_n1;
        locals.var_dv_j1_dn3 = assign9500_e11425_d_n3;
        locals.var_dv_j1_dn4 = assign9500_e11425_d_n4;
        locals.var_dv_j1_dn5 = assign9500_e11425_d_n5;
        locals.var_dv_j1_dn7 = assign9500_e11425_d_n7;
        locals.var_dv_j1_dn8 = assign9500_e11425_d_n8;
        locals.var_dv_j1_dn9 = assign9500_e11425_d_n9;
        locals.var_dv_j1_rv = 0.0;

        let (assign9510_e11434, assign9510_e11434_d_n0, assign9510_e11434_d_n1, assign9510_e11434_d_n3, assign9510_e11434_d_n4, assign9510_e11434_d_n5, assign9510_e11434_d_n7, assign9510_e11434_d_n8, assign9510_e11434_d_n9,) = {
    if (((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) && (locals.var_guard205 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_de_1, locals.var_de_1_dn0, locals.var_de_1_dn1, locals.var_de_1_dn3, locals.var_de_1_dn4, locals.var_de_1_dn5, locals.var_de_1_dn7, locals.var_de_1_dn8, locals.var_de_1_dn9,)
    }
};
        locals.var_de_1 = assign9510_e11434;
        locals.var_de_1_dn0 = assign9510_e11434_d_n0;
        locals.var_de_1_dn1 = assign9510_e11434_d_n1;
        locals.var_de_1_dn3 = assign9510_e11434_d_n3;
        locals.var_de_1_dn4 = assign9510_e11434_d_n4;
        locals.var_de_1_dn5 = assign9510_e11434_d_n5;
        locals.var_de_1_dn7 = assign9510_e11434_d_n7;
        locals.var_de_1_dn8 = assign9510_e11434_d_n8;
        locals.var_de_1_dn9 = assign9510_e11434_d_n9;
        locals.var_de_1_rv = 0.0;

        let (assign9520_e11443, assign9520_e11443_d_n0, assign9520_e11443_d_n1, assign9520_e11443_d_n3, assign9520_e11443_d_n4, assign9520_e11443_d_n5, assign9520_e11443_d_n7, assign9520_e11443_d_n8, assign9520_e11443_d_n9,) = {
    if (((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) && (locals.var_guard205 == 0.0)) {
        (locals.var_vsici, 0.0, 0.0, 0.0, 0.0, locals.var_vsici_dn5, 0.0, 0.0, locals.var_vsici_dn9,)
    } else {
        (locals.var_dv_j1, locals.var_dv_j1_dn0, locals.var_dv_j1_dn1, locals.var_dv_j1_dn3, locals.var_dv_j1_dn4, locals.var_dv_j1_dn5, locals.var_dv_j1_dn7, locals.var_dv_j1_dn8, locals.var_dv_j1_dn9,)
    }
};
        locals.var_dv_j1 = assign9520_e11443;
        locals.var_dv_j1_dn0 = assign9520_e11443_d_n0;
        locals.var_dv_j1_dn1 = assign9520_e11443_d_n1;
        locals.var_dv_j1_dn3 = assign9520_e11443_d_n3;
        locals.var_dv_j1_dn4 = assign9520_e11443_d_n4;
        locals.var_dv_j1_dn5 = assign9520_e11443_d_n5;
        locals.var_dv_j1_dn7 = assign9520_e11443_d_n7;
        locals.var_dv_j1_dn8 = assign9520_e11443_d_n8;
        locals.var_dv_j1_dn9 = assign9520_e11443_d_n9;
        locals.var_dv_j1_rv = 0.0;

        let (assign9530_e11455, assign9530_e11455_d_n4,) = {
    if ((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign9530_e11449: f64 = (0.1 * locals.var_dv_p);
        let assign9530_e11452: f64 = (4.0 * locals.var_vt);
        let assign9530_e11453: f64 = (assign9530_e11449 + assign9530_e11452);
        (assign9530_e11453, ((0.1 * locals.var_dv_p_dn4) + (4.0 * locals.var_vt_dn4)),)
    } else {
        (locals.var_da, locals.var_da_dn4,)
    }
};
        locals.var_da = assign9530_e11455;
        locals.var_da_dn4 = assign9530_e11455_d_n4;
        locals.var_da_rv = 0.0;

        let (assign9540_e11465, assign9540_e11465_d_n0, assign9540_e11465_d_n1, assign9540_e11465_d_n3, assign9540_e11465_d_n4, assign9540_e11465_d_n5, assign9540_e11465_d_n7, assign9540_e11465_d_n8, assign9540_e11465_d_n9,) = {
    if ((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign9540_e11461: f64 = (locals.var_dv_p + locals.var_dv_j1);
        let assign9540_e11463: f64 = (assign9540_e11461 / locals.var_da);
        (assign9540_e11463, (locals.var_dv_j1_dn0 / locals.var_da), (locals.var_dv_j1_dn1 / locals.var_da), (locals.var_dv_j1_dn3 / locals.var_da), ((((locals.var_dv_p_dn4 + locals.var_dv_j1_dn4) * locals.var_da) - (assign9540_e11461 * locals.var_da_dn4)) / (locals.var_da * locals.var_da)), (locals.var_dv_j1_dn5 / locals.var_da), (locals.var_dv_j1_dn7 / locals.var_da), (locals.var_dv_j1_dn8 / locals.var_da), (locals.var_dv_j1_dn9 / locals.var_da),)
    } else {
        (locals.var_dv_r, locals.var_dv_r_dn0, locals.var_dv_r_dn1, locals.var_dv_r_dn3, locals.var_dv_r_dn4, locals.var_dv_r_dn5, locals.var_dv_r_dn7, locals.var_dv_r_dn8, locals.var_dv_r_dn9,)
    }
};
        locals.var_dv_r = assign9540_e11465;
        locals.var_dv_r_dn0 = assign9540_e11465_d_n0;
        locals.var_dv_r_dn1 = assign9540_e11465_d_n1;
        locals.var_dv_r_dn3 = assign9540_e11465_d_n3;
        locals.var_dv_r_dn4 = assign9540_e11465_d_n4;
        locals.var_dv_r_dn5 = assign9540_e11465_d_n5;
        locals.var_dv_r_dn7 = assign9540_e11465_d_n7;
        locals.var_dv_r_dn8 = assign9540_e11465_d_n8;
        locals.var_dv_r_dn9 = assign9540_e11465_d_n9;
        locals.var_dv_r_rv = 0.0;

        let assign9550_e11468: f64 = if locals.var_dv_r < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard206 = assign9550_e11468;
        locals.var_guard206_rv = 0.0;

        let (assign9560_e11477, assign9560_e11477_d_n0, assign9560_e11477_d_n1, assign9560_e11477_d_n3, assign9560_e11477_d_n4, assign9560_e11477_d_n5, assign9560_e11477_d_n7, assign9560_e11477_d_n8, assign9560_e11477_d_n9,) = {
    if (((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) && (locals.var_guard206 != 0.0)) {
        let assign9560_e11475: f64 = (locals.var_dv_r).exp();
        (assign9560_e11475, (assign9560_e11475 * locals.var_dv_r_dn0), (assign9560_e11475 * locals.var_dv_r_dn1), (assign9560_e11475 * locals.var_dv_r_dn3), (assign9560_e11475 * locals.var_dv_r_dn4), (assign9560_e11475 * locals.var_dv_r_dn5), (assign9560_e11475 * locals.var_dv_r_dn7), (assign9560_e11475 * locals.var_dv_r_dn8), (assign9560_e11475 * locals.var_dv_r_dn9),)
    } else {
        (locals.var_de, locals.var_de_dn0, locals.var_de_dn1, locals.var_de_dn3, locals.var_de_dn4, locals.var_de_dn5, locals.var_de_dn7, locals.var_de_dn8, locals.var_de_dn9,)
    }
};
        locals.var_de = assign9560_e11477;
        locals.var_de_dn0 = assign9560_e11477_d_n0;
        locals.var_de_dn1 = assign9560_e11477_d_n1;
        locals.var_de_dn3 = assign9560_e11477_d_n3;
        locals.var_de_dn4 = assign9560_e11477_d_n4;
        locals.var_de_dn5 = assign9560_e11477_d_n5;
        locals.var_de_dn7 = assign9560_e11477_d_n7;
        locals.var_de_dn8 = assign9560_e11477_d_n8;
        locals.var_de_dn9 = assign9560_e11477_d_n9;
        locals.var_de_rv = 0.0;

        let (assign9570_e11489, assign9570_e11489_d_n0, assign9570_e11489_d_n1, assign9570_e11489_d_n3, assign9570_e11489_d_n4, assign9570_e11489_d_n5, assign9570_e11489_d_n7, assign9570_e11489_d_n8, assign9570_e11489_d_n9,) = {
    if (((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) && (locals.var_guard206 != 0.0)) {
        let assign9570_e11486: f64 = (1.0 + locals.var_de);
        let assign9570_e11487: f64 = (locals.var_de / assign9570_e11486);
        (assign9570_e11487, (((locals.var_de_dn0 * assign9570_e11486) - (locals.var_de * locals.var_de_dn0)) / (assign9570_e11486 * assign9570_e11486)), (((locals.var_de_dn1 * assign9570_e11486) - (locals.var_de * locals.var_de_dn1)) / (assign9570_e11486 * assign9570_e11486)), (((locals.var_de_dn3 * assign9570_e11486) - (locals.var_de * locals.var_de_dn3)) / (assign9570_e11486 * assign9570_e11486)), (((locals.var_de_dn4 * assign9570_e11486) - (locals.var_de * locals.var_de_dn4)) / (assign9570_e11486 * assign9570_e11486)), (((locals.var_de_dn5 * assign9570_e11486) - (locals.var_de * locals.var_de_dn5)) / (assign9570_e11486 * assign9570_e11486)), (((locals.var_de_dn7 * assign9570_e11486) - (locals.var_de * locals.var_de_dn7)) / (assign9570_e11486 * assign9570_e11486)), (((locals.var_de_dn8 * assign9570_e11486) - (locals.var_de * locals.var_de_dn8)) / (assign9570_e11486 * assign9570_e11486)), (((locals.var_de_dn9 * assign9570_e11486) - (locals.var_de * locals.var_de_dn9)) / (assign9570_e11486 * assign9570_e11486)),)
    } else {
        (locals.var_de_2, locals.var_de_2_dn0, locals.var_de_2_dn1, locals.var_de_2_dn3, locals.var_de_2_dn4, locals.var_de_2_dn5, locals.var_de_2_dn7, locals.var_de_2_dn8, locals.var_de_2_dn9,)
    }
};
        locals.var_de_2 = assign9570_e11489;
        locals.var_de_2_dn0 = assign9570_e11489_d_n0;
        locals.var_de_2_dn1 = assign9570_e11489_d_n1;
        locals.var_de_2_dn3 = assign9570_e11489_d_n3;
        locals.var_de_2_dn4 = assign9570_e11489_d_n4;
        locals.var_de_2_dn5 = assign9570_e11489_d_n5;
        locals.var_de_2_dn7 = assign9570_e11489_d_n7;
        locals.var_de_2_dn8 = assign9570_e11489_d_n8;
        locals.var_de_2_dn9 = assign9570_e11489_d_n9;
        locals.var_de_2_rv = 0.0;

        let (assign9580_e11513, assign9580_e11513_d_n0, assign9580_e11513_d_n1, assign9580_e11513_d_n3, assign9580_e11513_d_n4, assign9580_e11513_d_n5, assign9580_e11513_d_n7, assign9580_e11513_d_n8, assign9580_e11513_d_n9,) = {
    if (((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) && (locals.var_guard206 != 0.0)) {
        let assign9580_e11496: f64 = (-locals.var_dv_p);
        let assign9580_e11500: f64 = (1.0 + locals.var_de);
        let assign9580_e11501: f64 = (assign9580_e11500).ln();
        let assign9580_e11504: f64 = (locals.var_dv_p + locals.var_dv_f);
        let assign9580_e11505: f64 = (-assign9580_e11504);
        let assign9580_e11507: f64 = (assign9580_e11505 / locals.var_da);
        let assign9580_e11508: f64 = (assign9580_e11507).exp();
        let assign9580_e11509: f64 = (assign9580_e11501 - assign9580_e11508);
        let assign9580_e11510: f64 = (locals.var_da * assign9580_e11509);
        let assign9580_e11511: f64 = (assign9580_e11496 + assign9580_e11510);
        (assign9580_e11511, (locals.var_da * (locals.var_de_dn0 / assign9580_e11500)), (locals.var_da * (locals.var_de_dn1 / assign9580_e11500)), (locals.var_da * (locals.var_de_dn3 / assign9580_e11500)), ((-locals.var_dv_p_dn4) + ((locals.var_da_dn4 * assign9580_e11509) + (locals.var_da * ((locals.var_de_dn4 / assign9580_e11500) - (assign9580_e11508 * ((((-(locals.var_dv_p_dn4 + locals.var_dv_f_dn4)) * locals.var_da) - (assign9580_e11505 * locals.var_da_dn4)) / (locals.var_da * locals.var_da))))))), (locals.var_da * (locals.var_de_dn5 / assign9580_e11500)), (locals.var_da * (locals.var_de_dn7 / assign9580_e11500)), (locals.var_da * (locals.var_de_dn8 / assign9580_e11500)), (locals.var_da * (locals.var_de_dn9 / assign9580_e11500)),)
    } else {
        (locals.var_dv_j2, locals.var_dv_j2_dn0, locals.var_dv_j2_dn1, locals.var_dv_j2_dn3, locals.var_dv_j2_dn4, locals.var_dv_j2_dn5, locals.var_dv_j2_dn7, locals.var_dv_j2_dn8, locals.var_dv_j2_dn9,)
    }
};
        locals.var_dv_j2 = assign9580_e11513;
        locals.var_dv_j2_dn0 = assign9580_e11513_d_n0;
        locals.var_dv_j2_dn1 = assign9580_e11513_d_n1;
        locals.var_dv_j2_dn3 = assign9580_e11513_d_n3;
        locals.var_dv_j2_dn4 = assign9580_e11513_d_n4;
        locals.var_dv_j2_dn5 = assign9580_e11513_d_n5;
        locals.var_dv_j2_dn7 = assign9580_e11513_d_n7;
        locals.var_dv_j2_dn8 = assign9580_e11513_d_n8;
        locals.var_dv_j2_dn9 = assign9580_e11513_d_n9;
        locals.var_dv_j2_rv = 0.0;

        let (assign9590_e11522, assign9590_e11522_d_n0, assign9590_e11522_d_n1, assign9590_e11522_d_n3, assign9590_e11522_d_n4, assign9590_e11522_d_n5, assign9590_e11522_d_n7, assign9590_e11522_d_n8, assign9590_e11522_d_n9,) = {
    if (((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) && (locals.var_guard206 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_de_2, locals.var_de_2_dn0, locals.var_de_2_dn1, locals.var_de_2_dn3, locals.var_de_2_dn4, locals.var_de_2_dn5, locals.var_de_2_dn7, locals.var_de_2_dn8, locals.var_de_2_dn9,)
    }
};
        locals.var_de_2 = assign9590_e11522;
        locals.var_de_2_dn0 = assign9590_e11522_d_n0;
        locals.var_de_2_dn1 = assign9590_e11522_d_n1;
        locals.var_de_2_dn3 = assign9590_e11522_d_n3;
        locals.var_de_2_dn4 = assign9590_e11522_d_n4;
        locals.var_de_2_dn5 = assign9590_e11522_d_n5;
        locals.var_de_2_dn7 = assign9590_e11522_d_n7;
        locals.var_de_2_dn8 = assign9590_e11522_d_n8;
        locals.var_de_2_dn9 = assign9590_e11522_d_n9;
        locals.var_de_2_rv = 0.0;

        let (assign9600_e11531, assign9600_e11531_d_n0, assign9600_e11531_d_n1, assign9600_e11531_d_n3, assign9600_e11531_d_n4, assign9600_e11531_d_n5, assign9600_e11531_d_n7, assign9600_e11531_d_n8, assign9600_e11531_d_n9,) = {
    if (((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) && (locals.var_guard206 == 0.0)) {
        (locals.var_dv_j1, locals.var_dv_j1_dn0, locals.var_dv_j1_dn1, locals.var_dv_j1_dn3, locals.var_dv_j1_dn4, locals.var_dv_j1_dn5, locals.var_dv_j1_dn7, locals.var_dv_j1_dn8, locals.var_dv_j1_dn9,)
    } else {
        (locals.var_dv_j2, locals.var_dv_j2_dn0, locals.var_dv_j2_dn1, locals.var_dv_j2_dn3, locals.var_dv_j2_dn4, locals.var_dv_j2_dn5, locals.var_dv_j2_dn7, locals.var_dv_j2_dn8, locals.var_dv_j2_dn9,)
    }
};
        locals.var_dv_j2 = assign9600_e11531;
        locals.var_dv_j2_dn0 = assign9600_e11531_d_n0;
        locals.var_dv_j2_dn1 = assign9600_e11531_d_n1;
        locals.var_dv_j2_dn3 = assign9600_e11531_d_n3;
        locals.var_dv_j2_dn4 = assign9600_e11531_d_n4;
        locals.var_dv_j2_dn5 = assign9600_e11531_d_n5;
        locals.var_dv_j2_dn7 = assign9600_e11531_d_n7;
        locals.var_dv_j2_dn8 = assign9600_e11531_d_n8;
        locals.var_dv_j2_dn9 = assign9600_e11531_d_n9;
        locals.var_dv_j2_rv = 0.0;

        let (assign9610_e11539, assign9610_e11539_d_n0, assign9610_e11539_d_n1, assign9610_e11539_d_n3, assign9610_e11539_d_n4, assign9610_e11539_d_n5, assign9610_e11539_d_n7, assign9610_e11539_d_n8, assign9610_e11539_d_n9,) = {
    if ((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign9610_e11537: f64 = (locals.var_vsici - locals.var_dv_j1);
        (assign9610_e11537, (-locals.var_dv_j1_dn0), (-locals.var_dv_j1_dn1), (-locals.var_dv_j1_dn3), (-locals.var_dv_j1_dn4), (locals.var_vsici_dn5 - locals.var_dv_j1_dn5), (-locals.var_dv_j1_dn7), (-locals.var_dv_j1_dn8), (locals.var_vsici_dn9 - locals.var_dv_j1_dn9),)
    } else {
        (locals.var_dv_j4, locals.var_dv_j4_dn0, locals.var_dv_j4_dn1, locals.var_dv_j4_dn3, locals.var_dv_j4_dn4, locals.var_dv_j4_dn5, locals.var_dv_j4_dn7, locals.var_dv_j4_dn8, locals.var_dv_j4_dn9,)
    }
};
        locals.var_dv_j4 = assign9610_e11539;
        locals.var_dv_j4_dn0 = assign9610_e11539_d_n0;
        locals.var_dv_j4_dn1 = assign9610_e11539_d_n1;
        locals.var_dv_j4_dn3 = assign9610_e11539_d_n3;
        locals.var_dv_j4_dn4 = assign9610_e11539_d_n4;
        locals.var_dv_j4_dn5 = assign9610_e11539_d_n5;
        locals.var_dv_j4_dn7 = assign9610_e11539_d_n7;
        locals.var_dv_j4_dn8 = assign9610_e11539_d_n8;
        locals.var_dv_j4_dn9 = assign9610_e11539_d_n9;
        locals.var_dv_j4_rv = 0.0;

        let (assign9620_e11550, assign9620_e11550_d_n0, assign9620_e11550_d_n1, assign9620_e11550_d_n3, assign9620_e11550_d_n4, assign9620_e11550_d_n5, assign9620_e11550_d_n7, assign9620_e11550_d_n8, assign9620_e11550_d_n9,) = {
    if ((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign9620_e11546: f64 = (locals.var_dv_j1 / locals.var_vds_t);
        let assign9620_e11547: f64 = (1.0 - assign9620_e11546);
        let assign9620_e11548: f64 = (assign9620_e11547).ln();
        (assign9620_e11548, ((-(locals.var_dv_j1_dn0 / locals.var_vds_t)) / assign9620_e11547), ((-(locals.var_dv_j1_dn1 / locals.var_vds_t)) / assign9620_e11547), ((-(locals.var_dv_j1_dn3 / locals.var_vds_t)) / assign9620_e11547), ((-(((locals.var_dv_j1_dn4 * locals.var_vds_t) - (locals.var_dv_j1 * locals.var_vds_t_dn4)) / (locals.var_vds_t * locals.var_vds_t))) / assign9620_e11547), ((-(locals.var_dv_j1_dn5 / locals.var_vds_t)) / assign9620_e11547), ((-(locals.var_dv_j1_dn7 / locals.var_vds_t)) / assign9620_e11547), ((-(locals.var_dv_j1_dn8 / locals.var_vds_t)) / assign9620_e11547), ((-(locals.var_dv_j1_dn9 / locals.var_vds_t)) / assign9620_e11547),)
    } else {
        (locals.var_dcln1, locals.var_dcln1_dn0, locals.var_dcln1_dn1, locals.var_dcln1_dn3, locals.var_dcln1_dn4, locals.var_dcln1_dn5, locals.var_dcln1_dn7, locals.var_dcln1_dn8, locals.var_dcln1_dn9,)
    }
};
        locals.var_dcln1 = assign9620_e11550;
        locals.var_dcln1_dn0 = assign9620_e11550_d_n0;
        locals.var_dcln1_dn1 = assign9620_e11550_d_n1;
        locals.var_dcln1_dn3 = assign9620_e11550_d_n3;
        locals.var_dcln1_dn4 = assign9620_e11550_d_n4;
        locals.var_dcln1_dn5 = assign9620_e11550_d_n5;
        locals.var_dcln1_dn7 = assign9620_e11550_d_n7;
        locals.var_dcln1_dn8 = assign9620_e11550_d_n8;
        locals.var_dcln1_dn9 = assign9620_e11550_d_n9;
        locals.var_dcln1_rv = 0.0;

        let (assign9630_e11561, assign9630_e11561_d_n0, assign9630_e11561_d_n1, assign9630_e11561_d_n3, assign9630_e11561_d_n4, assign9630_e11561_d_n5, assign9630_e11561_d_n7, assign9630_e11561_d_n8, assign9630_e11561_d_n9,) = {
    if ((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign9630_e11557: f64 = (locals.var_dv_j2 / locals.var_vds_t);
        let assign9630_e11558: f64 = (1.0 - assign9630_e11557);
        let assign9630_e11559: f64 = (assign9630_e11558).ln();
        (assign9630_e11559, ((-(locals.var_dv_j2_dn0 / locals.var_vds_t)) / assign9630_e11558), ((-(locals.var_dv_j2_dn1 / locals.var_vds_t)) / assign9630_e11558), ((-(locals.var_dv_j2_dn3 / locals.var_vds_t)) / assign9630_e11558), ((-(((locals.var_dv_j2_dn4 * locals.var_vds_t) - (locals.var_dv_j2 * locals.var_vds_t_dn4)) / (locals.var_vds_t * locals.var_vds_t))) / assign9630_e11558), ((-(locals.var_dv_j2_dn5 / locals.var_vds_t)) / assign9630_e11558), ((-(locals.var_dv_j2_dn7 / locals.var_vds_t)) / assign9630_e11558), ((-(locals.var_dv_j2_dn8 / locals.var_vds_t)) / assign9630_e11558), ((-(locals.var_dv_j2_dn9 / locals.var_vds_t)) / assign9630_e11558),)
    } else {
        (locals.var_dcln2, locals.var_dcln2_dn0, locals.var_dcln2_dn1, locals.var_dcln2_dn3, locals.var_dcln2_dn4, locals.var_dcln2_dn5, locals.var_dcln2_dn7, locals.var_dcln2_dn8, locals.var_dcln2_dn9,)
    }
};
        locals.var_dcln2 = assign9630_e11561;
        locals.var_dcln2_dn0 = assign9630_e11561_d_n0;
        locals.var_dcln2_dn1 = assign9630_e11561_d_n1;
        locals.var_dcln2_dn3 = assign9630_e11561_d_n3;
        locals.var_dcln2_dn4 = assign9630_e11561_d_n4;
        locals.var_dcln2_dn5 = assign9630_e11561_d_n5;
        locals.var_dcln2_dn7 = assign9630_e11561_d_n7;
        locals.var_dcln2_dn8 = assign9630_e11561_d_n8;
        locals.var_dcln2_dn9 = assign9630_e11561_d_n9;
        locals.var_dcln2_rv = 0.0;

        let (assign9640_e11569,) = {
    if ((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign9640_e11567: f64 = (1.0 - p.p59);
        (assign9640_e11567,)
    } else {
        (locals.var_dz1,)
    }
};
        locals.var_dz1 = assign9640_e11569;
        locals.var_dz1_rv = 0.0;

        let (assign9650_e11577,) = {
    if ((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign9650_e11575: f64 = (1.0 - locals.var_dz_r);
        (assign9650_e11575,)
    } else {
        (locals.var_dzr1,)
    }
};
        locals.var_dzr1 = assign9650_e11577;
        locals.var_dzr1_rv = 0.0;

        let (assign9660_e11593, assign9660_e11593_d_n0, assign9660_e11593_d_n1, assign9660_e11593_d_n3, assign9660_e11593_d_n4, assign9660_e11593_d_n5, assign9660_e11593_d_n7, assign9660_e11593_d_n8, assign9660_e11593_d_n9,) = {
    if ((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign9660_e11584: f64 = (-p.p59);
        let assign9660_e11585: f64 = (locals.var_dcln2 * assign9660_e11584);
        let assign9660_e11586: f64 = (assign9660_e11585).exp();
        let assign9660_e11587: f64 = (locals.var_cjs0_t * assign9660_e11586);
        let assign9660_e11589: f64 = (assign9660_e11587 * locals.var_de_1);
        let assign9660_e11591: f64 = (assign9660_e11589 * locals.var_de_2);
        (assign9660_e11591, (((((locals.var_cjs0_t * (assign9660_e11586 * (locals.var_dcln2_dn0 * assign9660_e11584))) * locals.var_de_1) + (assign9660_e11587 * locals.var_de_1_dn0)) * locals.var_de_2) + (assign9660_e11589 * locals.var_de_2_dn0)), (((((locals.var_cjs0_t * (assign9660_e11586 * (locals.var_dcln2_dn1 * assign9660_e11584))) * locals.var_de_1) + (assign9660_e11587 * locals.var_de_1_dn1)) * locals.var_de_2) + (assign9660_e11589 * locals.var_de_2_dn1)), (((((locals.var_cjs0_t * (assign9660_e11586 * (locals.var_dcln2_dn3 * assign9660_e11584))) * locals.var_de_1) + (assign9660_e11587 * locals.var_de_1_dn3)) * locals.var_de_2) + (assign9660_e11589 * locals.var_de_2_dn3)), ((((((locals.var_cjs0_t_dn4 * assign9660_e11586) + (locals.var_cjs0_t * (assign9660_e11586 * (locals.var_dcln2_dn4 * assign9660_e11584)))) * locals.var_de_1) + (assign9660_e11587 * locals.var_de_1_dn4)) * locals.var_de_2) + (assign9660_e11589 * locals.var_de_2_dn4)), (((((locals.var_cjs0_t * (assign9660_e11586 * (locals.var_dcln2_dn5 * assign9660_e11584))) * locals.var_de_1) + (assign9660_e11587 * locals.var_de_1_dn5)) * locals.var_de_2) + (assign9660_e11589 * locals.var_de_2_dn5)), (((((locals.var_cjs0_t * (assign9660_e11586 * (locals.var_dcln2_dn7 * assign9660_e11584))) * locals.var_de_1) + (assign9660_e11587 * locals.var_de_1_dn7)) * locals.var_de_2) + (assign9660_e11589 * locals.var_de_2_dn7)), (((((locals.var_cjs0_t * (assign9660_e11586 * (locals.var_dcln2_dn8 * assign9660_e11584))) * locals.var_de_1) + (assign9660_e11587 * locals.var_de_1_dn8)) * locals.var_de_2) + (assign9660_e11589 * locals.var_de_2_dn8)), (((((locals.var_cjs0_t * (assign9660_e11586 * (locals.var_dcln2_dn9 * assign9660_e11584))) * locals.var_de_1) + (assign9660_e11587 * locals.var_de_1_dn9)) * locals.var_de_2) + (assign9660_e11589 * locals.var_de_2_dn9)),)
    } else {
        (locals.var_dc_j1, locals.var_dc_j1_dn0, locals.var_dc_j1_dn1, locals.var_dc_j1_dn3, locals.var_dc_j1_dn4, locals.var_dc_j1_dn5, locals.var_dc_j1_dn7, locals.var_dc_j1_dn8, locals.var_dc_j1_dn9,)
    }
};
        locals.var_dc_j1 = assign9660_e11593;
        locals.var_dc_j1_dn0 = assign9660_e11593_d_n0;
        locals.var_dc_j1_dn1 = assign9660_e11593_d_n1;
        locals.var_dc_j1_dn3 = assign9660_e11593_d_n3;
        locals.var_dc_j1_dn4 = assign9660_e11593_d_n4;
        locals.var_dc_j1_dn5 = assign9660_e11593_d_n5;
        locals.var_dc_j1_dn7 = assign9660_e11593_d_n7;
        locals.var_dc_j1_dn8 = assign9660_e11593_d_n8;
        locals.var_dc_j1_dn9 = assign9660_e11593_d_n9;
        locals.var_dc_j1_rv = 0.0;

        let (assign9670_e11609, assign9670_e11609_d_n0, assign9670_e11609_d_n1, assign9670_e11609_d_n3, assign9670_e11609_d_n4, assign9670_e11609_d_n5, assign9670_e11609_d_n7, assign9670_e11609_d_n8, assign9670_e11609_d_n9,) = {
    if ((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign9670_e11600: f64 = (-locals.var_dz_r);
        let assign9670_e11601: f64 = (locals.var_dcln1 * assign9670_e11600);
        let assign9670_e11602: f64 = (assign9670_e11601).exp();
        let assign9670_e11603: f64 = (locals.var_dc_c * assign9670_e11602);
        let assign9670_e11606: f64 = (1.0 - locals.var_de_2);
        let assign9670_e11607: f64 = (assign9670_e11603 * assign9670_e11606);
        (assign9670_e11607, (((locals.var_dc_c * (assign9670_e11602 * (locals.var_dcln1_dn0 * assign9670_e11600))) * assign9670_e11606) + (assign9670_e11603 * (-locals.var_de_2_dn0))), (((locals.var_dc_c * (assign9670_e11602 * (locals.var_dcln1_dn1 * assign9670_e11600))) * assign9670_e11606) + (assign9670_e11603 * (-locals.var_de_2_dn1))), (((locals.var_dc_c * (assign9670_e11602 * (locals.var_dcln1_dn3 * assign9670_e11600))) * assign9670_e11606) + (assign9670_e11603 * (-locals.var_de_2_dn3))), ((((locals.var_dc_c_dn4 * assign9670_e11602) + (locals.var_dc_c * (assign9670_e11602 * (locals.var_dcln1_dn4 * assign9670_e11600)))) * assign9670_e11606) + (assign9670_e11603 * (-locals.var_de_2_dn4))), (((locals.var_dc_c * (assign9670_e11602 * (locals.var_dcln1_dn5 * assign9670_e11600))) * assign9670_e11606) + (assign9670_e11603 * (-locals.var_de_2_dn5))), (((locals.var_dc_c * (assign9670_e11602 * (locals.var_dcln1_dn7 * assign9670_e11600))) * assign9670_e11606) + (assign9670_e11603 * (-locals.var_de_2_dn7))), (((locals.var_dc_c * (assign9670_e11602 * (locals.var_dcln1_dn8 * assign9670_e11600))) * assign9670_e11606) + (assign9670_e11603 * (-locals.var_de_2_dn8))), (((locals.var_dc_c * (assign9670_e11602 * (locals.var_dcln1_dn9 * assign9670_e11600))) * assign9670_e11606) + (assign9670_e11603 * (-locals.var_de_2_dn9))),)
    } else {
        (locals.var_dc_j2, locals.var_dc_j2_dn0, locals.var_dc_j2_dn1, locals.var_dc_j2_dn3, locals.var_dc_j2_dn4, locals.var_dc_j2_dn5, locals.var_dc_j2_dn7, locals.var_dc_j2_dn8, locals.var_dc_j2_dn9,)
    }
};
        locals.var_dc_j2 = assign9670_e11609;
        locals.var_dc_j2_dn0 = assign9670_e11609_d_n0;
        locals.var_dc_j2_dn1 = assign9670_e11609_d_n1;
        locals.var_dc_j2_dn3 = assign9670_e11609_d_n3;
        locals.var_dc_j2_dn4 = assign9670_e11609_d_n4;
        locals.var_dc_j2_dn5 = assign9670_e11609_d_n5;
        locals.var_dc_j2_dn7 = assign9670_e11609_d_n7;
        locals.var_dc_j2_dn8 = assign9670_e11609_d_n8;
        locals.var_dc_j2_dn9 = assign9670_e11609_d_n9;
        locals.var_dc_j2_rv = 0.0;

        let (assign9680_e11619, assign9680_e11619_d_n0, assign9680_e11619_d_n1, assign9680_e11619_d_n3, assign9680_e11619_d_n4, assign9680_e11619_d_n5, assign9680_e11619_d_n7, assign9680_e11619_d_n8, assign9680_e11619_d_n9,) = {
    if ((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign9680_e11616: f64 = (1.0 - locals.var_de_1);
        let assign9680_e11617: f64 = (locals.var_dc_max * assign9680_e11616);
        (assign9680_e11617, (locals.var_dc_max * (-locals.var_de_1_dn0)), (locals.var_dc_max * (-locals.var_de_1_dn1)), (locals.var_dc_max * (-locals.var_de_1_dn3)), ((locals.var_dc_max_dn4 * assign9680_e11616) + (locals.var_dc_max * (-locals.var_de_1_dn4))), (locals.var_dc_max * (-locals.var_de_1_dn5)), (locals.var_dc_max * (-locals.var_de_1_dn7)), (locals.var_dc_max * (-locals.var_de_1_dn8)), (locals.var_dc_max * (-locals.var_de_1_dn9)),)
    } else {
        (locals.var_dc_j3, locals.var_dc_j3_dn0, locals.var_dc_j3_dn1, locals.var_dc_j3_dn3, locals.var_dc_j3_dn4, locals.var_dc_j3_dn5, locals.var_dc_j3_dn7, locals.var_dc_j3_dn8, locals.var_dc_j3_dn9,)
    }
};
        locals.var_dc_j3 = assign9680_e11619;
        locals.var_dc_j3_dn0 = assign9680_e11619_d_n0;
        locals.var_dc_j3_dn1 = assign9680_e11619_d_n1;
        locals.var_dc_j3_dn3 = assign9680_e11619_d_n3;
        locals.var_dc_j3_dn4 = assign9680_e11619_d_n4;
        locals.var_dc_j3_dn5 = assign9680_e11619_d_n5;
        locals.var_dc_j3_dn7 = assign9680_e11619_d_n7;
        locals.var_dc_j3_dn8 = assign9680_e11619_d_n8;
        locals.var_dc_j3_dn9 = assign9680_e11619_d_n9;
        locals.var_dc_j3_rv = 0.0;

        let (assign9700_e11644, assign9700_e11644_d_n0, assign9700_e11644_d_n1, assign9700_e11644_d_n3, assign9700_e11644_d_n4, assign9700_e11644_d_n5, assign9700_e11644_d_n7, assign9700_e11644_d_n8, assign9700_e11644_d_n9,) = {
    if ((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign9700_e11637: f64 = (locals.var_dcln2 * locals.var_dz1);
        let assign9700_e11638: f64 = (assign9700_e11637).exp();
        let assign9700_e11639: f64 = (1.0 - assign9700_e11638);
        let assign9700_e11640: f64 = (locals.var_cjs0_t * assign9700_e11639);
        let assign9700_e11642: f64 = (assign9700_e11640 / locals.var_dz1);
        (assign9700_e11642, ((locals.var_cjs0_t * (-(assign9700_e11638 * (locals.var_dcln2_dn0 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cjs0_t * (-(assign9700_e11638 * (locals.var_dcln2_dn1 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cjs0_t * (-(assign9700_e11638 * (locals.var_dcln2_dn3 * locals.var_dz1)))) / locals.var_dz1), (((locals.var_cjs0_t_dn4 * assign9700_e11639) + (locals.var_cjs0_t * (-(assign9700_e11638 * (locals.var_dcln2_dn4 * locals.var_dz1))))) / locals.var_dz1), ((locals.var_cjs0_t * (-(assign9700_e11638 * (locals.var_dcln2_dn5 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cjs0_t * (-(assign9700_e11638 * (locals.var_dcln2_dn7 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cjs0_t * (-(assign9700_e11638 * (locals.var_dcln2_dn8 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cjs0_t * (-(assign9700_e11638 * (locals.var_dcln2_dn9 * locals.var_dz1)))) / locals.var_dz1),)
    } else {
        (locals.var_dq_j1, locals.var_dq_j1_dn0, locals.var_dq_j1_dn1, locals.var_dq_j1_dn3, locals.var_dq_j1_dn4, locals.var_dq_j1_dn5, locals.var_dq_j1_dn7, locals.var_dq_j1_dn8, locals.var_dq_j1_dn9,)
    }
};
        locals.var_dq_j1 = assign9700_e11644;
        locals.var_dq_j1_dn0 = assign9700_e11644_d_n0;
        locals.var_dq_j1_dn1 = assign9700_e11644_d_n1;
        locals.var_dq_j1_dn3 = assign9700_e11644_d_n3;
        locals.var_dq_j1_dn4 = assign9700_e11644_d_n4;
        locals.var_dq_j1_dn5 = assign9700_e11644_d_n5;
        locals.var_dq_j1_dn7 = assign9700_e11644_d_n7;
        locals.var_dq_j1_dn8 = assign9700_e11644_d_n8;
        locals.var_dq_j1_dn9 = assign9700_e11644_d_n9;
        locals.var_dq_j1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_26(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9710_e11659, assign9710_e11659_d_n0, assign9710_e11659_d_n1, assign9710_e11659_d_n3, assign9710_e11659_d_n4, assign9710_e11659_d_n5, assign9710_e11659_d_n7, assign9710_e11659_d_n8, assign9710_e11659_d_n9,) = {
    if ((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign9710_e11652: f64 = (locals.var_dcln1 * locals.var_dzr1);
        let assign9710_e11653: f64 = (assign9710_e11652).exp();
        let assign9710_e11654: f64 = (1.0 - assign9710_e11653);
        let assign9710_e11655: f64 = (locals.var_dc_c * assign9710_e11654);
        let assign9710_e11657: f64 = (assign9710_e11655 / locals.var_dzr1);
        (assign9710_e11657, ((locals.var_dc_c * (-(assign9710_e11653 * (locals.var_dcln1_dn0 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9710_e11653 * (locals.var_dcln1_dn1 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9710_e11653 * (locals.var_dcln1_dn3 * locals.var_dzr1)))) / locals.var_dzr1), (((locals.var_dc_c_dn4 * assign9710_e11654) + (locals.var_dc_c * (-(assign9710_e11653 * (locals.var_dcln1_dn4 * locals.var_dzr1))))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9710_e11653 * (locals.var_dcln1_dn5 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9710_e11653 * (locals.var_dcln1_dn7 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9710_e11653 * (locals.var_dcln1_dn8 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9710_e11653 * (locals.var_dcln1_dn9 * locals.var_dzr1)))) / locals.var_dzr1),)
    } else {
        (locals.var_dq_j2, locals.var_dq_j2_dn0, locals.var_dq_j2_dn1, locals.var_dq_j2_dn3, locals.var_dq_j2_dn4, locals.var_dq_j2_dn5, locals.var_dq_j2_dn7, locals.var_dq_j2_dn8, locals.var_dq_j2_dn9,)
    }
};
        locals.var_dq_j2 = assign9710_e11659;
        locals.var_dq_j2_dn0 = assign9710_e11659_d_n0;
        locals.var_dq_j2_dn1 = assign9710_e11659_d_n1;
        locals.var_dq_j2_dn3 = assign9710_e11659_d_n3;
        locals.var_dq_j2_dn4 = assign9710_e11659_d_n4;
        locals.var_dq_j2_dn5 = assign9710_e11659_d_n5;
        locals.var_dq_j2_dn7 = assign9710_e11659_d_n7;
        locals.var_dq_j2_dn8 = assign9710_e11659_d_n8;
        locals.var_dq_j2_dn9 = assign9710_e11659_d_n9;
        locals.var_dq_j2_rv = 0.0;

        let (assign9720_e11674, assign9720_e11674_d_n0, assign9720_e11674_d_n1, assign9720_e11674_d_n3, assign9720_e11674_d_n4, assign9720_e11674_d_n5, assign9720_e11674_d_n7, assign9720_e11674_d_n8, assign9720_e11674_d_n9,) = {
    if ((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign9720_e11667: f64 = (locals.var_dcln2 * locals.var_dzr1);
        let assign9720_e11668: f64 = (assign9720_e11667).exp();
        let assign9720_e11669: f64 = (1.0 - assign9720_e11668);
        let assign9720_e11670: f64 = (locals.var_dc_c * assign9720_e11669);
        let assign9720_e11672: f64 = (assign9720_e11670 / locals.var_dzr1);
        (assign9720_e11672, ((locals.var_dc_c * (-(assign9720_e11668 * (locals.var_dcln2_dn0 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9720_e11668 * (locals.var_dcln2_dn1 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9720_e11668 * (locals.var_dcln2_dn3 * locals.var_dzr1)))) / locals.var_dzr1), (((locals.var_dc_c_dn4 * assign9720_e11669) + (locals.var_dc_c * (-(assign9720_e11668 * (locals.var_dcln2_dn4 * locals.var_dzr1))))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9720_e11668 * (locals.var_dcln2_dn5 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9720_e11668 * (locals.var_dcln2_dn7 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9720_e11668 * (locals.var_dcln2_dn8 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign9720_e11668 * (locals.var_dcln2_dn9 * locals.var_dzr1)))) / locals.var_dzr1),)
    } else {
        (locals.var_dq_j3, locals.var_dq_j3_dn0, locals.var_dq_j3_dn1, locals.var_dq_j3_dn3, locals.var_dq_j3_dn4, locals.var_dq_j3_dn5, locals.var_dq_j3_dn7, locals.var_dq_j3_dn8, locals.var_dq_j3_dn9,)
    }
};
        locals.var_dq_j3 = assign9720_e11674;
        locals.var_dq_j3_dn0 = assign9720_e11674_d_n0;
        locals.var_dq_j3_dn1 = assign9720_e11674_d_n1;
        locals.var_dq_j3_dn3 = assign9720_e11674_d_n3;
        locals.var_dq_j3_dn4 = assign9720_e11674_d_n4;
        locals.var_dq_j3_dn5 = assign9720_e11674_d_n5;
        locals.var_dq_j3_dn7 = assign9720_e11674_d_n7;
        locals.var_dq_j3_dn8 = assign9720_e11674_d_n8;
        locals.var_dq_j3_dn9 = assign9720_e11674_d_n9;
        locals.var_dq_j3_rv = 0.0;

        let (assign9730_e11690, assign9730_e11690_d_n0, assign9730_e11690_d_n1, assign9730_e11690_d_n3, assign9730_e11690_d_n4, assign9730_e11690_d_n5, assign9730_e11690_d_n6, assign9730_e11690_d_n7, assign9730_e11690_d_n8, assign9730_e11690_d_n9,) = {
    if ((locals.var_guard203 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign9730_e11680: f64 = (locals.var_dq_j1 + locals.var_dq_j2);
        let assign9730_e11682: f64 = (assign9730_e11680 - locals.var_dq_j3);
        let assign9730_e11684: f64 = (assign9730_e11682 * locals.var_vds_t);
        let assign9730_e11687: f64 = (locals.var_dc_max * locals.var_dv_j4);
        let assign9730_e11688: f64 = (assign9730_e11684 + assign9730_e11687);
        (assign9730_e11688, ((((locals.var_dq_j1_dn0 + locals.var_dq_j2_dn0) - locals.var_dq_j3_dn0) * locals.var_vds_t) + (locals.var_dc_max * locals.var_dv_j4_dn0)), ((((locals.var_dq_j1_dn1 + locals.var_dq_j2_dn1) - locals.var_dq_j3_dn1) * locals.var_vds_t) + (locals.var_dc_max * locals.var_dv_j4_dn1)), ((((locals.var_dq_j1_dn3 + locals.var_dq_j2_dn3) - locals.var_dq_j3_dn3) * locals.var_vds_t) + (locals.var_dc_max * locals.var_dv_j4_dn3)), (((((locals.var_dq_j1_dn4 + locals.var_dq_j2_dn4) - locals.var_dq_j3_dn4) * locals.var_vds_t) + (assign9730_e11682 * locals.var_vds_t_dn4)) + ((locals.var_dc_max_dn4 * locals.var_dv_j4) + (locals.var_dc_max * locals.var_dv_j4_dn4))), ((((locals.var_dq_j1_dn5 + locals.var_dq_j2_dn5) - locals.var_dq_j3_dn5) * locals.var_vds_t) + (locals.var_dc_max * locals.var_dv_j4_dn5)), 0.0, ((((locals.var_dq_j1_dn7 + locals.var_dq_j2_dn7) - locals.var_dq_j3_dn7) * locals.var_vds_t) + (locals.var_dc_max * locals.var_dv_j4_dn7)), ((((locals.var_dq_j1_dn8 + locals.var_dq_j2_dn8) - locals.var_dq_j3_dn8) * locals.var_vds_t) + (locals.var_dc_max * locals.var_dv_j4_dn8)), ((((locals.var_dq_j1_dn9 + locals.var_dq_j2_dn9) - locals.var_dq_j3_dn9) * locals.var_vds_t) + (locals.var_dc_max * locals.var_dv_j4_dn9)),)
    } else {
        (locals.var_qjs, locals.var_qjs_dn0, locals.var_qjs_dn1, locals.var_qjs_dn3, locals.var_qjs_dn4, locals.var_qjs_dn5, locals.var_qjs_dn6, locals.var_qjs_dn7, locals.var_qjs_dn8, locals.var_qjs_dn9,)
    }
};
        locals.var_qjs = assign9730_e11690;
        locals.var_qjs_dn0 = assign9730_e11690_d_n0;
        locals.var_qjs_dn1 = assign9730_e11690_d_n1;
        locals.var_qjs_dn3 = assign9730_e11690_d_n3;
        locals.var_qjs_dn4 = assign9730_e11690_d_n4;
        locals.var_qjs_dn5 = assign9730_e11690_d_n5;
        locals.var_qjs_dn6 = assign9730_e11690_d_n6;
        locals.var_qjs_dn7 = assign9730_e11690_d_n7;
        locals.var_qjs_dn8 = assign9730_e11690_d_n8;
        locals.var_qjs_dn9 = assign9730_e11690_d_n9;
        locals.var_qjs_rv = 0.0;

        let (assign9750_e11704, assign9750_e11704_d_n0, assign9750_e11704_d_n1, assign9750_e11704_d_n3, assign9750_e11704_d_n4, assign9750_e11704_d_n5, assign9750_e11704_d_n6, assign9750_e11704_d_n7, assign9750_e11704_d_n8, assign9750_e11704_d_n9,) = {
    if ((locals.var_guard203 != 0.0) && (locals.var_guard204 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qjs, locals.var_qjs_dn0, locals.var_qjs_dn1, locals.var_qjs_dn3, locals.var_qjs_dn4, locals.var_qjs_dn5, locals.var_qjs_dn6, locals.var_qjs_dn7, locals.var_qjs_dn8, locals.var_qjs_dn9,)
    }
};
        locals.var_qjs = assign9750_e11704;
        locals.var_qjs_dn0 = assign9750_e11704_d_n0;
        locals.var_qjs_dn1 = assign9750_e11704_d_n1;
        locals.var_qjs_dn3 = assign9750_e11704_d_n3;
        locals.var_qjs_dn4 = assign9750_e11704_d_n4;
        locals.var_qjs_dn5 = assign9750_e11704_d_n5;
        locals.var_qjs_dn6 = assign9750_e11704_d_n6;
        locals.var_qjs_dn7 = assign9750_e11704_d_n7;
        locals.var_qjs_dn8 = assign9750_e11704_d_n8;
        locals.var_qjs_dn9 = assign9750_e11704_d_n9;
        locals.var_qjs_rv = 0.0;

        let assign9760_e11707: f64 = if locals.var_cjs0_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard207 = assign9760_e11707;
        locals.var_guard207_rv = 0.0;

        let (assign9770_e11723, assign9770_e11723_d_n4,) = {
    if ((locals.var_guard203 == 0.0) && (locals.var_guard207 != 0.0)) {
        let assign9770_e11715: f64 = (locals.var_ajs_t).ln();
        let assign9770_e11716: f64 = (-assign9770_e11715);
        let assign9770_e11718: f64 = (assign9770_e11716 / p.p59);
        let assign9770_e11719: f64 = (assign9770_e11718).exp();
        let assign9770_e11720: f64 = (1.0 - assign9770_e11719);
        let assign9770_e11721: f64 = (locals.var_vds_t * assign9770_e11720);
        (assign9770_e11721, ((locals.var_vds_t_dn4 * assign9770_e11720) + (locals.var_vds_t * (-(assign9770_e11719 * ((-(locals.var_ajs_t_dn4 / locals.var_ajs_t)) / p.p59))))),)
    } else {
        (locals.var_dfv_f, locals.var_dfv_f_dn4,)
    }
};
        locals.var_dfv_f = assign9770_e11723;
        locals.var_dfv_f_dn4 = assign9770_e11723_d_n4;
        locals.var_dfv_f_rv = 0.0;

        let (assign9780_e11734, assign9780_e11734_d_n0, assign9780_e11734_d_n1, assign9780_e11734_d_n3, assign9780_e11734_d_n4, assign9780_e11734_d_n5, assign9780_e11734_d_n6, assign9780_e11734_d_n7, assign9780_e11734_d_n8, assign9780_e11734_d_n9,) = {
    if ((locals.var_guard203 == 0.0) && (locals.var_guard207 != 0.0)) {
        let assign9780_e11730: f64 = (locals.var_dfv_f - locals.var_vsici);
        let assign9780_e11732: f64 = (assign9780_e11730 * locals.var_ovt);
        (assign9780_e11732, 0.0, 0.0, 0.0, ((locals.var_dfv_f_dn4 * locals.var_ovt) + (assign9780_e11730 * locals.var_ovt_dn4)), ((-locals.var_vsici_dn5) * locals.var_ovt), 0.0, 0.0, 0.0, ((-locals.var_vsici_dn9) * locals.var_ovt),)
    } else {
        (locals.var_dfx, locals.var_dfx_dn0, locals.var_dfx_dn1, locals.var_dfx_dn3, locals.var_dfx_dn4, locals.var_dfx_dn5, locals.var_dfx_dn6, locals.var_dfx_dn7, locals.var_dfx_dn8, locals.var_dfx_dn9,)
    }
};
        locals.var_dfx = assign9780_e11734;
        locals.var_dfx_dn0 = assign9780_e11734_d_n0;
        locals.var_dfx_dn1 = assign9780_e11734_d_n1;
        locals.var_dfx_dn3 = assign9780_e11734_d_n3;
        locals.var_dfx_dn4 = assign9780_e11734_d_n4;
        locals.var_dfx_dn5 = assign9780_e11734_d_n5;
        locals.var_dfx_dn6 = assign9780_e11734_d_n6;
        locals.var_dfx_dn7 = assign9780_e11734_d_n7;
        locals.var_dfx_dn8 = assign9780_e11734_d_n8;
        locals.var_dfx_dn9 = assign9780_e11734_d_n9;
        locals.var_dfx_rv = 0.0;

        let (assign9790_e11746, assign9790_e11746_d_n0, assign9790_e11746_d_n1, assign9790_e11746_d_n3, assign9790_e11746_d_n4, assign9790_e11746_d_n5, assign9790_e11746_d_n6, assign9790_e11746_d_n7, assign9790_e11746_d_n8, assign9790_e11746_d_n9,) = {
    if ((locals.var_guard203 == 0.0) && (locals.var_guard207 != 0.0)) {
        let assign9790_e11741: f64 = (locals.var_dfx * locals.var_dfx);
        let assign9790_e11743: f64 = (assign9790_e11741 + 1.921812);
        let assign9790_e11744: f64 = (assign9790_e11743).sqrt();
        (assign9790_e11744, (((locals.var_dfx_dn0 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn0)) / (2.0 * assign9790_e11744)), (((locals.var_dfx_dn1 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn1)) / (2.0 * assign9790_e11744)), (((locals.var_dfx_dn3 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn3)) / (2.0 * assign9790_e11744)), (((locals.var_dfx_dn4 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn4)) / (2.0 * assign9790_e11744)), (((locals.var_dfx_dn5 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn5)) / (2.0 * assign9790_e11744)), (((locals.var_dfx_dn6 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn6)) / (2.0 * assign9790_e11744)), (((locals.var_dfx_dn7 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn7)) / (2.0 * assign9790_e11744)), (((locals.var_dfx_dn8 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn8)) / (2.0 * assign9790_e11744)), (((locals.var_dfx_dn9 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn9)) / (2.0 * assign9790_e11744)),)
    } else {
        (locals.var_dfs_q, locals.var_dfs_q_dn0, locals.var_dfs_q_dn1, locals.var_dfs_q_dn3, locals.var_dfs_q_dn4, locals.var_dfs_q_dn5, locals.var_dfs_q_dn6, locals.var_dfs_q_dn7, locals.var_dfs_q_dn8, locals.var_dfs_q_dn9,)
    }
};
        locals.var_dfs_q = assign9790_e11746;
        locals.var_dfs_q_dn0 = assign9790_e11746_d_n0;
        locals.var_dfs_q_dn1 = assign9790_e11746_d_n1;
        locals.var_dfs_q_dn3 = assign9790_e11746_d_n3;
        locals.var_dfs_q_dn4 = assign9790_e11746_d_n4;
        locals.var_dfs_q_dn5 = assign9790_e11746_d_n5;
        locals.var_dfs_q_dn6 = assign9790_e11746_d_n6;
        locals.var_dfs_q_dn7 = assign9790_e11746_d_n7;
        locals.var_dfs_q_dn8 = assign9790_e11746_d_n8;
        locals.var_dfs_q_dn9 = assign9790_e11746_d_n9;
        locals.var_dfs_q_rv = 0.0;

        let (assign9800_e11757, assign9800_e11757_d_n0, assign9800_e11757_d_n1, assign9800_e11757_d_n3, assign9800_e11757_d_n4, assign9800_e11757_d_n5, assign9800_e11757_d_n6, assign9800_e11757_d_n7, assign9800_e11757_d_n8, assign9800_e11757_d_n9,) = {
    if ((locals.var_guard203 == 0.0) && (locals.var_guard207 != 0.0)) {
        let assign9800_e11753: f64 = (locals.var_dfx + locals.var_dfs_q);
        let assign9800_e11755: f64 = (assign9800_e11753 * 0.5);
        (assign9800_e11755, ((locals.var_dfx_dn0 + locals.var_dfs_q_dn0) * 0.5), ((locals.var_dfx_dn1 + locals.var_dfs_q_dn1) * 0.5), ((locals.var_dfx_dn3 + locals.var_dfs_q_dn3) * 0.5), ((locals.var_dfx_dn4 + locals.var_dfs_q_dn4) * 0.5), ((locals.var_dfx_dn5 + locals.var_dfs_q_dn5) * 0.5), ((locals.var_dfx_dn6 + locals.var_dfs_q_dn6) * 0.5), ((locals.var_dfx_dn7 + locals.var_dfs_q_dn7) * 0.5), ((locals.var_dfx_dn8 + locals.var_dfs_q_dn8) * 0.5), ((locals.var_dfx_dn9 + locals.var_dfs_q_dn9) * 0.5),)
    } else {
        (locals.var_dfs_q2, locals.var_dfs_q2_dn0, locals.var_dfs_q2_dn1, locals.var_dfs_q2_dn3, locals.var_dfs_q2_dn4, locals.var_dfs_q2_dn5, locals.var_dfs_q2_dn6, locals.var_dfs_q2_dn7, locals.var_dfs_q2_dn8, locals.var_dfs_q2_dn9,)
    }
};
        locals.var_dfs_q2 = assign9800_e11757;
        locals.var_dfs_q2_dn0 = assign9800_e11757_d_n0;
        locals.var_dfs_q2_dn1 = assign9800_e11757_d_n1;
        locals.var_dfs_q2_dn3 = assign9800_e11757_d_n3;
        locals.var_dfs_q2_dn4 = assign9800_e11757_d_n4;
        locals.var_dfs_q2_dn5 = assign9800_e11757_d_n5;
        locals.var_dfs_q2_dn6 = assign9800_e11757_d_n6;
        locals.var_dfs_q2_dn7 = assign9800_e11757_d_n7;
        locals.var_dfs_q2_dn8 = assign9800_e11757_d_n8;
        locals.var_dfs_q2_dn9 = assign9800_e11757_d_n9;
        locals.var_dfs_q2_rv = 0.0;

        let (assign9810_e11768, assign9810_e11768_d_n0, assign9810_e11768_d_n1, assign9810_e11768_d_n3, assign9810_e11768_d_n4, assign9810_e11768_d_n5, assign9810_e11768_d_n6, assign9810_e11768_d_n7, assign9810_e11768_d_n8, assign9810_e11768_d_n9,) = {
    if ((locals.var_guard203 == 0.0) && (locals.var_guard207 != 0.0)) {
        let assign9810_e11765: f64 = (locals.var_vt * locals.var_dfs_q2);
        let assign9810_e11766: f64 = (locals.var_dfv_f - assign9810_e11765);
        (assign9810_e11766, (-(locals.var_vt * locals.var_dfs_q2_dn0)), (-(locals.var_vt * locals.var_dfs_q2_dn1)), (-(locals.var_vt * locals.var_dfs_q2_dn3)), (locals.var_dfv_f_dn4 - ((locals.var_vt_dn4 * locals.var_dfs_q2) + (locals.var_vt * locals.var_dfs_q2_dn4))), (-(locals.var_vt * locals.var_dfs_q2_dn5)), (-(locals.var_vt * locals.var_dfs_q2_dn6)), (-(locals.var_vt * locals.var_dfs_q2_dn7)), (-(locals.var_vt * locals.var_dfs_q2_dn8)), (-(locals.var_vt * locals.var_dfs_q2_dn9)),)
    } else {
        (locals.var_dfv_j, locals.var_dfv_j_dn0, locals.var_dfv_j_dn1, locals.var_dfv_j_dn3, locals.var_dfv_j_dn4, locals.var_dfv_j_dn5, locals.var_dfv_j_dn6, locals.var_dfv_j_dn7, locals.var_dfv_j_dn8, locals.var_dfv_j_dn9,)
    }
};
        locals.var_dfv_j = assign9810_e11768;
        locals.var_dfv_j_dn0 = assign9810_e11768_d_n0;
        locals.var_dfv_j_dn1 = assign9810_e11768_d_n1;
        locals.var_dfv_j_dn3 = assign9810_e11768_d_n3;
        locals.var_dfv_j_dn4 = assign9810_e11768_d_n4;
        locals.var_dfv_j_dn5 = assign9810_e11768_d_n5;
        locals.var_dfv_j_dn6 = assign9810_e11768_d_n6;
        locals.var_dfv_j_dn7 = assign9810_e11768_d_n7;
        locals.var_dfv_j_dn8 = assign9810_e11768_d_n8;
        locals.var_dfv_j_dn9 = assign9810_e11768_d_n9;
        locals.var_dfv_j_rv = 0.0;

        let (assign9820_e11777, assign9820_e11777_d_n0, assign9820_e11777_d_n1, assign9820_e11777_d_n3, assign9820_e11777_d_n4, assign9820_e11777_d_n5, assign9820_e11777_d_n6, assign9820_e11777_d_n7, assign9820_e11777_d_n8, assign9820_e11777_d_n9,) = {
    if ((locals.var_guard203 == 0.0) && (locals.var_guard207 != 0.0)) {
        let assign9820_e11775: f64 = (locals.var_dfs_q2 / locals.var_dfs_q);
        (assign9820_e11775, (((locals.var_dfs_q2_dn0 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn0)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn1 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn1)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn3 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn3)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn4 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn4)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn5 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn5)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn6 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn6)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn7 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn7)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn8 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn8)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn9 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn9)) / (locals.var_dfs_q * locals.var_dfs_q)),)
    } else {
        (locals.var_dfdvj_dv, locals.var_dfdvj_dv_dn0, locals.var_dfdvj_dv_dn1, locals.var_dfdvj_dv_dn3, locals.var_dfdvj_dv_dn4, locals.var_dfdvj_dv_dn5, locals.var_dfdvj_dv_dn6, locals.var_dfdvj_dv_dn7, locals.var_dfdvj_dv_dn8, locals.var_dfdvj_dv_dn9,)
    }
};
        locals.var_dfdvj_dv = assign9820_e11777;
        locals.var_dfdvj_dv_dn0 = assign9820_e11777_d_n0;
        locals.var_dfdvj_dv_dn1 = assign9820_e11777_d_n1;
        locals.var_dfdvj_dv_dn3 = assign9820_e11777_d_n3;
        locals.var_dfdvj_dv_dn4 = assign9820_e11777_d_n4;
        locals.var_dfdvj_dv_dn5 = assign9820_e11777_d_n5;
        locals.var_dfdvj_dv_dn6 = assign9820_e11777_d_n6;
        locals.var_dfdvj_dv_dn7 = assign9820_e11777_d_n7;
        locals.var_dfdvj_dv_dn8 = assign9820_e11777_d_n8;
        locals.var_dfdvj_dv_dn9 = assign9820_e11777_d_n9;
        locals.var_dfdvj_dv_rv = 0.0;

        let (assign9830_e11789, assign9830_e11789_d_n0, assign9830_e11789_d_n1, assign9830_e11789_d_n3, assign9830_e11789_d_n4, assign9830_e11789_d_n5, assign9830_e11789_d_n6, assign9830_e11789_d_n7, assign9830_e11789_d_n8, assign9830_e11789_d_n9,) = {
    if ((locals.var_guard203 == 0.0) && (locals.var_guard207 != 0.0)) {
        let assign9830_e11785: f64 = (locals.var_dfv_j / locals.var_vds_t);
        let assign9830_e11786: f64 = (1.0 - assign9830_e11785);
        let assign9830_e11787: f64 = (assign9830_e11786).ln();
        (assign9830_e11787, ((-(locals.var_dfv_j_dn0 / locals.var_vds_t)) / assign9830_e11786), ((-(locals.var_dfv_j_dn1 / locals.var_vds_t)) / assign9830_e11786), ((-(locals.var_dfv_j_dn3 / locals.var_vds_t)) / assign9830_e11786), ((-(((locals.var_dfv_j_dn4 * locals.var_vds_t) - (locals.var_dfv_j * locals.var_vds_t_dn4)) / (locals.var_vds_t * locals.var_vds_t))) / assign9830_e11786), ((-(locals.var_dfv_j_dn5 / locals.var_vds_t)) / assign9830_e11786), ((-(locals.var_dfv_j_dn6 / locals.var_vds_t)) / assign9830_e11786), ((-(locals.var_dfv_j_dn7 / locals.var_vds_t)) / assign9830_e11786), ((-(locals.var_dfv_j_dn8 / locals.var_vds_t)) / assign9830_e11786), ((-(locals.var_dfv_j_dn9 / locals.var_vds_t)) / assign9830_e11786),)
    } else {
        (locals.var_dfb, locals.var_dfb_dn0, locals.var_dfb_dn1, locals.var_dfb_dn3, locals.var_dfb_dn4, locals.var_dfb_dn5, locals.var_dfb_dn6, locals.var_dfb_dn7, locals.var_dfb_dn8, locals.var_dfb_dn9,)
    }
};
        locals.var_dfb = assign9830_e11789;
        locals.var_dfb_dn0 = assign9830_e11789_d_n0;
        locals.var_dfb_dn1 = assign9830_e11789_d_n1;
        locals.var_dfb_dn3 = assign9830_e11789_d_n3;
        locals.var_dfb_dn4 = assign9830_e11789_d_n4;
        locals.var_dfb_dn5 = assign9830_e11789_d_n5;
        locals.var_dfb_dn6 = assign9830_e11789_d_n6;
        locals.var_dfb_dn7 = assign9830_e11789_d_n7;
        locals.var_dfb_dn8 = assign9830_e11789_d_n8;
        locals.var_dfb_dn9 = assign9830_e11789_d_n9;
        locals.var_dfb_rv = 0.0;

        let (assign9840_e11802, assign9840_e11802_d_n0, assign9840_e11802_d_n1, assign9840_e11802_d_n3, assign9840_e11802_d_n4, assign9840_e11802_d_n5, assign9840_e11802_d_n6, assign9840_e11802_d_n7, assign9840_e11802_d_n8, assign9840_e11802_d_n9,) = {
    if ((locals.var_guard203 == 0.0) && (locals.var_guard207 != 0.0)) {
        let assign9840_e11795: f64 = (-p.p59);
        let assign9840_e11797: f64 = (assign9840_e11795 * locals.var_dfb);
        let assign9840_e11798: f64 = (assign9840_e11797).exp();
        let assign9840_e11800: f64 = (assign9840_e11798 * locals.var_dfdvj_dv);
        (assign9840_e11800, (((assign9840_e11798 * (assign9840_e11795 * locals.var_dfb_dn0)) * locals.var_dfdvj_dv) + (assign9840_e11798 * locals.var_dfdvj_dv_dn0)), (((assign9840_e11798 * (assign9840_e11795 * locals.var_dfb_dn1)) * locals.var_dfdvj_dv) + (assign9840_e11798 * locals.var_dfdvj_dv_dn1)), (((assign9840_e11798 * (assign9840_e11795 * locals.var_dfb_dn3)) * locals.var_dfdvj_dv) + (assign9840_e11798 * locals.var_dfdvj_dv_dn3)), (((assign9840_e11798 * (assign9840_e11795 * locals.var_dfb_dn4)) * locals.var_dfdvj_dv) + (assign9840_e11798 * locals.var_dfdvj_dv_dn4)), (((assign9840_e11798 * (assign9840_e11795 * locals.var_dfb_dn5)) * locals.var_dfdvj_dv) + (assign9840_e11798 * locals.var_dfdvj_dv_dn5)), (((assign9840_e11798 * (assign9840_e11795 * locals.var_dfb_dn6)) * locals.var_dfdvj_dv) + (assign9840_e11798 * locals.var_dfdvj_dv_dn6)), (((assign9840_e11798 * (assign9840_e11795 * locals.var_dfb_dn7)) * locals.var_dfdvj_dv) + (assign9840_e11798 * locals.var_dfdvj_dv_dn7)), (((assign9840_e11798 * (assign9840_e11795 * locals.var_dfb_dn8)) * locals.var_dfdvj_dv) + (assign9840_e11798 * locals.var_dfdvj_dv_dn8)), (((assign9840_e11798 * (assign9840_e11795 * locals.var_dfb_dn9)) * locals.var_dfdvj_dv) + (assign9840_e11798 * locals.var_dfdvj_dv_dn9)),)
    } else {
        (locals.var_dfc_j1, locals.var_dfc_j1_dn0, locals.var_dfc_j1_dn1, locals.var_dfc_j1_dn3, locals.var_dfc_j1_dn4, locals.var_dfc_j1_dn5, locals.var_dfc_j1_dn6, locals.var_dfc_j1_dn7, locals.var_dfc_j1_dn8, locals.var_dfc_j1_dn9,)
    }
};
        locals.var_dfc_j1 = assign9840_e11802;
        locals.var_dfc_j1_dn0 = assign9840_e11802_d_n0;
        locals.var_dfc_j1_dn1 = assign9840_e11802_d_n1;
        locals.var_dfc_j1_dn3 = assign9840_e11802_d_n3;
        locals.var_dfc_j1_dn4 = assign9840_e11802_d_n4;
        locals.var_dfc_j1_dn5 = assign9840_e11802_d_n5;
        locals.var_dfc_j1_dn6 = assign9840_e11802_d_n6;
        locals.var_dfc_j1_dn7 = assign9840_e11802_d_n7;
        locals.var_dfc_j1_dn8 = assign9840_e11802_d_n8;
        locals.var_dfc_j1_dn9 = assign9840_e11802_d_n9;
        locals.var_dfc_j1_rv = 0.0;

        let (assign9860_e11837, assign9860_e11837_d_n0, assign9860_e11837_d_n1, assign9860_e11837_d_n3, assign9860_e11837_d_n4, assign9860_e11837_d_n5, assign9860_e11837_d_n6, assign9860_e11837_d_n7, assign9860_e11837_d_n8, assign9860_e11837_d_n9,) = {
    if ((locals.var_guard203 == 0.0) && (locals.var_guard207 != 0.0)) {
        let assign9860_e11827: f64 = (1.0 - p.p59);
        let assign9860_e11828: f64 = (locals.var_dfb * assign9860_e11827);
        let assign9860_e11829: f64 = (assign9860_e11828).exp();
        let assign9860_e11830: f64 = (1.0 - assign9860_e11829);
        let assign9860_e11831: f64 = (locals.var_vds_t * assign9860_e11830);
        let assign9860_e11834: f64 = (1.0 - p.p59);
        let assign9860_e11835: f64 = (assign9860_e11831 / assign9860_e11834);
        (assign9860_e11835, ((locals.var_vds_t * (-(assign9860_e11829 * (locals.var_dfb_dn0 * assign9860_e11827)))) / assign9860_e11834), ((locals.var_vds_t * (-(assign9860_e11829 * (locals.var_dfb_dn1 * assign9860_e11827)))) / assign9860_e11834), ((locals.var_vds_t * (-(assign9860_e11829 * (locals.var_dfb_dn3 * assign9860_e11827)))) / assign9860_e11834), (((locals.var_vds_t_dn4 * assign9860_e11830) + (locals.var_vds_t * (-(assign9860_e11829 * (locals.var_dfb_dn4 * assign9860_e11827))))) / assign9860_e11834), ((locals.var_vds_t * (-(assign9860_e11829 * (locals.var_dfb_dn5 * assign9860_e11827)))) / assign9860_e11834), ((locals.var_vds_t * (-(assign9860_e11829 * (locals.var_dfb_dn6 * assign9860_e11827)))) / assign9860_e11834), ((locals.var_vds_t * (-(assign9860_e11829 * (locals.var_dfb_dn7 * assign9860_e11827)))) / assign9860_e11834), ((locals.var_vds_t * (-(assign9860_e11829 * (locals.var_dfb_dn8 * assign9860_e11827)))) / assign9860_e11834), ((locals.var_vds_t * (-(assign9860_e11829 * (locals.var_dfb_dn9 * assign9860_e11827)))) / assign9860_e11834),)
    } else {
        (locals.var_dfq_j1, locals.var_dfq_j1_dn0, locals.var_dfq_j1_dn1, locals.var_dfq_j1_dn3, locals.var_dfq_j1_dn4, locals.var_dfq_j1_dn5, locals.var_dfq_j1_dn6, locals.var_dfq_j1_dn7, locals.var_dfq_j1_dn8, locals.var_dfq_j1_dn9,)
    }
};
        locals.var_dfq_j1 = assign9860_e11837;
        locals.var_dfq_j1_dn0 = assign9860_e11837_d_n0;
        locals.var_dfq_j1_dn1 = assign9860_e11837_d_n1;
        locals.var_dfq_j1_dn3 = assign9860_e11837_d_n3;
        locals.var_dfq_j1_dn4 = assign9860_e11837_d_n4;
        locals.var_dfq_j1_dn5 = assign9860_e11837_d_n5;
        locals.var_dfq_j1_dn6 = assign9860_e11837_d_n6;
        locals.var_dfq_j1_dn7 = assign9860_e11837_d_n7;
        locals.var_dfq_j1_dn8 = assign9860_e11837_d_n8;
        locals.var_dfq_j1_dn9 = assign9860_e11837_d_n9;
        locals.var_dfq_j1_rv = 0.0;

        let (assign9870_e11852, assign9870_e11852_d_n0, assign9870_e11852_d_n1, assign9870_e11852_d_n3, assign9870_e11852_d_n4, assign9870_e11852_d_n5, assign9870_e11852_d_n6, assign9870_e11852_d_n7, assign9870_e11852_d_n8, assign9870_e11852_d_n9,) = {
    if ((locals.var_guard203 == 0.0) && (locals.var_guard207 != 0.0)) {
        let assign9870_e11847: f64 = (locals.var_vsici - locals.var_dfv_j);
        let assign9870_e11848: f64 = (locals.var_ajs_t * assign9870_e11847);
        let assign9870_e11849: f64 = (locals.var_dfq_j1 + assign9870_e11848);
        let assign9870_e11850: f64 = (locals.var_cjs0_t * assign9870_e11849);
        (assign9870_e11850, (locals.var_cjs0_t * (locals.var_dfq_j1_dn0 + (locals.var_ajs_t * (-locals.var_dfv_j_dn0)))), (locals.var_cjs0_t * (locals.var_dfq_j1_dn1 + (locals.var_ajs_t * (-locals.var_dfv_j_dn1)))), (locals.var_cjs0_t * (locals.var_dfq_j1_dn3 + (locals.var_ajs_t * (-locals.var_dfv_j_dn3)))), ((locals.var_cjs0_t_dn4 * assign9870_e11849) + (locals.var_cjs0_t * (locals.var_dfq_j1_dn4 + ((locals.var_ajs_t_dn4 * assign9870_e11847) + (locals.var_ajs_t * (-locals.var_dfv_j_dn4)))))), (locals.var_cjs0_t * (locals.var_dfq_j1_dn5 + (locals.var_ajs_t * (locals.var_vsici_dn5 - locals.var_dfv_j_dn5)))), (locals.var_cjs0_t * (locals.var_dfq_j1_dn6 + (locals.var_ajs_t * (-locals.var_dfv_j_dn6)))), (locals.var_cjs0_t * (locals.var_dfq_j1_dn7 + (locals.var_ajs_t * (-locals.var_dfv_j_dn7)))), (locals.var_cjs0_t * (locals.var_dfq_j1_dn8 + (locals.var_ajs_t * (-locals.var_dfv_j_dn8)))), (locals.var_cjs0_t * (locals.var_dfq_j1_dn9 + (locals.var_ajs_t * (locals.var_vsici_dn9 - locals.var_dfv_j_dn9)))),)
    } else {
        (locals.var_qjs, locals.var_qjs_dn0, locals.var_qjs_dn1, locals.var_qjs_dn3, locals.var_qjs_dn4, locals.var_qjs_dn5, locals.var_qjs_dn6, locals.var_qjs_dn7, locals.var_qjs_dn8, locals.var_qjs_dn9,)
    }
};
        locals.var_qjs = assign9870_e11852;
        locals.var_qjs_dn0 = assign9870_e11852_d_n0;
        locals.var_qjs_dn1 = assign9870_e11852_d_n1;
        locals.var_qjs_dn3 = assign9870_e11852_d_n3;
        locals.var_qjs_dn4 = assign9870_e11852_d_n4;
        locals.var_qjs_dn5 = assign9870_e11852_d_n5;
        locals.var_qjs_dn6 = assign9870_e11852_d_n6;
        locals.var_qjs_dn7 = assign9870_e11852_d_n7;
        locals.var_qjs_dn8 = assign9870_e11852_d_n8;
        locals.var_qjs_dn9 = assign9870_e11852_d_n9;
        locals.var_qjs_rv = 0.0;

        let (assign9890_e11868, assign9890_e11868_d_n0, assign9890_e11868_d_n1, assign9890_e11868_d_n3, assign9890_e11868_d_n4, assign9890_e11868_d_n5, assign9890_e11868_d_n6, assign9890_e11868_d_n7, assign9890_e11868_d_n8, assign9890_e11868_d_n9,) = {
    if ((locals.var_guard203 == 0.0) && (locals.var_guard207 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qjs, locals.var_qjs_dn0, locals.var_qjs_dn1, locals.var_qjs_dn3, locals.var_qjs_dn4, locals.var_qjs_dn5, locals.var_qjs_dn6, locals.var_qjs_dn7, locals.var_qjs_dn8, locals.var_qjs_dn9,)
    }
};
        locals.var_qjs = assign9890_e11868;
        locals.var_qjs_dn0 = assign9890_e11868_d_n0;
        locals.var_qjs_dn1 = assign9890_e11868_d_n1;
        locals.var_qjs_dn3 = assign9890_e11868_d_n3;
        locals.var_qjs_dn4 = assign9890_e11868_d_n4;
        locals.var_qjs_dn5 = assign9890_e11868_d_n5;
        locals.var_qjs_dn6 = assign9890_e11868_d_n6;
        locals.var_qjs_dn7 = assign9890_e11868_d_n7;
        locals.var_qjs_dn8 = assign9890_e11868_d_n8;
        locals.var_qjs_dn9 = assign9890_e11868_d_n9;
        locals.var_qjs_rv = 0.0;

        let assign9900_e11871: f64 = if p.p63 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard208 = assign9900_e11871;
        locals.var_guard208_rv = 0.0;

        let assign9910_e11874: f64 = if p.p65 < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard209 = assign9910_e11874;
        locals.var_guard209_rv = 0.0;

        let assign9920_e11877: f64 = if locals.var_cscp0_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard210 = assign9920_e11877;
        locals.var_guard210_rv = 0.0;

        let (assign9930_e11887,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) {
        let assign9930_e11885: f64 = (p.p64 / 4.0);
        (assign9930_e11885,)
    } else {
        (locals.var_dz_r,)
    }
};
        locals.var_dz_r = assign9930_e11887;
        locals.var_dz_r_rv = 0.0;

        let (assign9940_e11897, assign9940_e11897_d_n4,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) {
        let assign9940_e11895: f64 = (p.p65 - locals.var_vdsp_t);
        (assign9940_e11895, (-locals.var_vdsp_t_dn4),)
    } else {
        (locals.var_dv_p, locals.var_dv_p_dn4,)
    }
};
        locals.var_dv_p = assign9940_e11897;
        locals.var_dv_p_dn4 = assign9940_e11897_d_n4;
        locals.var_dv_p_rv = 0.0;

        let (assign9950_e11914, assign9950_e11914_d_n4,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) {
        let assign9950_e11906: f64 = (locals.var_ajsp_t).ln();
        let assign9950_e11907: f64 = (-assign9950_e11906);
        let assign9950_e11909: f64 = (assign9950_e11907 / p.p64);
        let assign9950_e11910: f64 = (assign9950_e11909).exp();
        let assign9950_e11911: f64 = (1.0 - assign9950_e11910);
        let assign9950_e11912: f64 = (locals.var_vdsp_t * assign9950_e11911);
        (assign9950_e11912, ((locals.var_vdsp_t_dn4 * assign9950_e11911) + (locals.var_vdsp_t * (-(assign9950_e11910 * ((-(locals.var_ajsp_t_dn4 / locals.var_ajsp_t)) / p.p64))))),)
    } else {
        (locals.var_dv_f, locals.var_dv_f_dn4,)
    }
};
        locals.var_dv_f = assign9950_e11914;
        locals.var_dv_f_dn4 = assign9950_e11914_d_n4;
        locals.var_dv_f_rv = 0.0;

        let (assign9960_e11924, assign9960_e11924_d_n4,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) {
        let assign9960_e11922: f64 = (locals.var_ajsp_t * locals.var_cscp0_t);
        (assign9960_e11922, ((locals.var_ajsp_t_dn4 * locals.var_cscp0_t) + (locals.var_ajsp_t * locals.var_cscp0_t_dn4)),)
    } else {
        (locals.var_dc_max, locals.var_dc_max_dn4,)
    }
};
        locals.var_dc_max = assign9960_e11924;
        locals.var_dc_max_dn4 = assign9960_e11924_d_n4;
        locals.var_dc_max_rv = 0.0;

        let (assign9970_e11942, assign9970_e11942_d_n4,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) {
        let assign9970_e11933: f64 = (locals.var_dz_r - p.p64);
        let assign9970_e11936: f64 = (p.p65 / locals.var_vdsp_t);
        let assign9970_e11937: f64 = (assign9970_e11936).ln();
        let assign9970_e11938: f64 = (assign9970_e11933 * assign9970_e11937);
        let assign9970_e11939: f64 = (assign9970_e11938).exp();
        let assign9970_e11940: f64 = (locals.var_cscp0_t * assign9970_e11939);
        (assign9970_e11940, ((locals.var_cscp0_t_dn4 * assign9970_e11939) + (locals.var_cscp0_t * (assign9970_e11939 * (assign9970_e11933 * ((-((p.p65 * locals.var_vdsp_t_dn4) / (locals.var_vdsp_t * locals.var_vdsp_t))) / assign9970_e11936))))),)
    } else {
        (locals.var_dc_c, locals.var_dc_c_dn4,)
    }
};
        locals.var_dc_c = assign9970_e11942;
        locals.var_dc_c_dn4 = assign9970_e11942_d_n4;
        locals.var_dc_c_rv = 0.0;

        let (assign9980_e11954, assign9980_e11954_d_n0, assign9980_e11954_d_n1, assign9980_e11954_d_n3, assign9980_e11954_d_n4, assign9980_e11954_d_n5, assign9980_e11954_d_n7, assign9980_e11954_d_n8, assign9980_e11954_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) {
        let assign9980_e11950: f64 = (locals.var_dv_f - locals.var_vsc);
        let assign9980_e11952: f64 = (assign9980_e11950 * locals.var_ovt);
        (assign9980_e11952, ((-locals.var_vsc_dn0) * locals.var_ovt), 0.0, ((-locals.var_vsc_dn3) * locals.var_ovt), ((locals.var_dv_f_dn4 * locals.var_ovt) + (assign9980_e11950 * locals.var_ovt_dn4)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dv_e, locals.var_dv_e_dn0, locals.var_dv_e_dn1, locals.var_dv_e_dn3, locals.var_dv_e_dn4, locals.var_dv_e_dn5, locals.var_dv_e_dn7, locals.var_dv_e_dn8, locals.var_dv_e_dn9,)
    }
};
        locals.var_dv_e = assign9980_e11954;
        locals.var_dv_e_dn0 = assign9980_e11954_d_n0;
        locals.var_dv_e_dn1 = assign9980_e11954_d_n1;
        locals.var_dv_e_dn3 = assign9980_e11954_d_n3;
        locals.var_dv_e_dn4 = assign9980_e11954_d_n4;
        locals.var_dv_e_dn5 = assign9980_e11954_d_n5;
        locals.var_dv_e_dn7 = assign9980_e11954_d_n7;
        locals.var_dv_e_dn8 = assign9980_e11954_d_n8;
        locals.var_dv_e_dn9 = assign9980_e11954_d_n9;
        locals.var_dv_e_rv = 0.0;

        let assign9990_e11957: f64 = if locals.var_dv_e < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard211 = assign9990_e11957;
        locals.var_guard211_rv = 0.0;

        let (assign10000_e11968, assign10000_e11968_d_n0, assign10000_e11968_d_n1, assign10000_e11968_d_n3, assign10000_e11968_d_n4, assign10000_e11968_d_n5, assign10000_e11968_d_n7, assign10000_e11968_d_n8, assign10000_e11968_d_n9,) = {
    if ((((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) && (locals.var_guard211 != 0.0)) {
        let assign10000_e11966: f64 = (locals.var_dv_e).exp();
        (assign10000_e11966, (assign10000_e11966 * locals.var_dv_e_dn0), (assign10000_e11966 * locals.var_dv_e_dn1), (assign10000_e11966 * locals.var_dv_e_dn3), (assign10000_e11966 * locals.var_dv_e_dn4), (assign10000_e11966 * locals.var_dv_e_dn5), (assign10000_e11966 * locals.var_dv_e_dn7), (assign10000_e11966 * locals.var_dv_e_dn8), (assign10000_e11966 * locals.var_dv_e_dn9),)
    } else {
        (locals.var_de, locals.var_de_dn0, locals.var_de_dn1, locals.var_de_dn3, locals.var_de_dn4, locals.var_de_dn5, locals.var_de_dn7, locals.var_de_dn8, locals.var_de_dn9,)
    }
};
        locals.var_de = assign10000_e11968;
        locals.var_de_dn0 = assign10000_e11968_d_n0;
        locals.var_de_dn1 = assign10000_e11968_d_n1;
        locals.var_de_dn3 = assign10000_e11968_d_n3;
        locals.var_de_dn4 = assign10000_e11968_d_n4;
        locals.var_de_dn5 = assign10000_e11968_d_n5;
        locals.var_de_dn7 = assign10000_e11968_d_n7;
        locals.var_de_dn8 = assign10000_e11968_d_n8;
        locals.var_de_dn9 = assign10000_e11968_d_n9;
        locals.var_de_rv = 0.0;

        let (assign10010_e11982, assign10010_e11982_d_n0, assign10010_e11982_d_n1, assign10010_e11982_d_n3, assign10010_e11982_d_n4, assign10010_e11982_d_n5, assign10010_e11982_d_n7, assign10010_e11982_d_n8, assign10010_e11982_d_n9,) = {
    if ((((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) && (locals.var_guard211 != 0.0)) {
        let assign10010_e11979: f64 = (1.0 + locals.var_de);
        let assign10010_e11980: f64 = (locals.var_de / assign10010_e11979);
        (assign10010_e11980, (((locals.var_de_dn0 * assign10010_e11979) - (locals.var_de * locals.var_de_dn0)) / (assign10010_e11979 * assign10010_e11979)), (((locals.var_de_dn1 * assign10010_e11979) - (locals.var_de * locals.var_de_dn1)) / (assign10010_e11979 * assign10010_e11979)), (((locals.var_de_dn3 * assign10010_e11979) - (locals.var_de * locals.var_de_dn3)) / (assign10010_e11979 * assign10010_e11979)), (((locals.var_de_dn4 * assign10010_e11979) - (locals.var_de * locals.var_de_dn4)) / (assign10010_e11979 * assign10010_e11979)), (((locals.var_de_dn5 * assign10010_e11979) - (locals.var_de * locals.var_de_dn5)) / (assign10010_e11979 * assign10010_e11979)), (((locals.var_de_dn7 * assign10010_e11979) - (locals.var_de * locals.var_de_dn7)) / (assign10010_e11979 * assign10010_e11979)), (((locals.var_de_dn8 * assign10010_e11979) - (locals.var_de * locals.var_de_dn8)) / (assign10010_e11979 * assign10010_e11979)), (((locals.var_de_dn9 * assign10010_e11979) - (locals.var_de * locals.var_de_dn9)) / (assign10010_e11979 * assign10010_e11979)),)
    } else {
        (locals.var_de_1, locals.var_de_1_dn0, locals.var_de_1_dn1, locals.var_de_1_dn3, locals.var_de_1_dn4, locals.var_de_1_dn5, locals.var_de_1_dn7, locals.var_de_1_dn8, locals.var_de_1_dn9,)
    }
};
        locals.var_de_1 = assign10010_e11982;
        locals.var_de_1_dn0 = assign10010_e11982_d_n0;
        locals.var_de_1_dn1 = assign10010_e11982_d_n1;
        locals.var_de_1_dn3 = assign10010_e11982_d_n3;
        locals.var_de_1_dn4 = assign10010_e11982_d_n4;
        locals.var_de_1_dn5 = assign10010_e11982_d_n5;
        locals.var_de_1_dn7 = assign10010_e11982_d_n7;
        locals.var_de_1_dn8 = assign10010_e11982_d_n8;
        locals.var_de_1_dn9 = assign10010_e11982_d_n9;
        locals.var_de_1_rv = 0.0;

        let (assign10020_e11999, assign10020_e11999_d_n0, assign10020_e11999_d_n1, assign10020_e11999_d_n3, assign10020_e11999_d_n4, assign10020_e11999_d_n5, assign10020_e11999_d_n7, assign10020_e11999_d_n8, assign10020_e11999_d_n9,) = {
    if ((((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) && (locals.var_guard211 != 0.0)) {
        let assign10020_e11994: f64 = (1.0 + locals.var_de);
        let assign10020_e11995: f64 = (assign10020_e11994).ln();
        let assign10020_e11996: f64 = (locals.var_vt * assign10020_e11995);
        let assign10020_e11997: f64 = (locals.var_dv_f - assign10020_e11996);
        (assign10020_e11997, (-(locals.var_vt * (locals.var_de_dn0 / assign10020_e11994))), (-(locals.var_vt * (locals.var_de_dn1 / assign10020_e11994))), (-(locals.var_vt * (locals.var_de_dn3 / assign10020_e11994))), (locals.var_dv_f_dn4 - ((locals.var_vt_dn4 * assign10020_e11995) + (locals.var_vt * (locals.var_de_dn4 / assign10020_e11994)))), (-(locals.var_vt * (locals.var_de_dn5 / assign10020_e11994))), (-(locals.var_vt * (locals.var_de_dn7 / assign10020_e11994))), (-(locals.var_vt * (locals.var_de_dn8 / assign10020_e11994))), (-(locals.var_vt * (locals.var_de_dn9 / assign10020_e11994))),)
    } else {
        (locals.var_dv_j1, locals.var_dv_j1_dn0, locals.var_dv_j1_dn1, locals.var_dv_j1_dn3, locals.var_dv_j1_dn4, locals.var_dv_j1_dn5, locals.var_dv_j1_dn7, locals.var_dv_j1_dn8, locals.var_dv_j1_dn9,)
    }
};
        locals.var_dv_j1 = assign10020_e11999;
        locals.var_dv_j1_dn0 = assign10020_e11999_d_n0;
        locals.var_dv_j1_dn1 = assign10020_e11999_d_n1;
        locals.var_dv_j1_dn3 = assign10020_e11999_d_n3;
        locals.var_dv_j1_dn4 = assign10020_e11999_d_n4;
        locals.var_dv_j1_dn5 = assign10020_e11999_d_n5;
        locals.var_dv_j1_dn7 = assign10020_e11999_d_n7;
        locals.var_dv_j1_dn8 = assign10020_e11999_d_n8;
        locals.var_dv_j1_dn9 = assign10020_e11999_d_n9;
        locals.var_dv_j1_rv = 0.0;

        let (assign10030_e12010, assign10030_e12010_d_n0, assign10030_e12010_d_n1, assign10030_e12010_d_n3, assign10030_e12010_d_n4, assign10030_e12010_d_n5, assign10030_e12010_d_n7, assign10030_e12010_d_n8, assign10030_e12010_d_n9,) = {
    if ((((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) && (locals.var_guard211 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_de_1, locals.var_de_1_dn0, locals.var_de_1_dn1, locals.var_de_1_dn3, locals.var_de_1_dn4, locals.var_de_1_dn5, locals.var_de_1_dn7, locals.var_de_1_dn8, locals.var_de_1_dn9,)
    }
};
        locals.var_de_1 = assign10030_e12010;
        locals.var_de_1_dn0 = assign10030_e12010_d_n0;
        locals.var_de_1_dn1 = assign10030_e12010_d_n1;
        locals.var_de_1_dn3 = assign10030_e12010_d_n3;
        locals.var_de_1_dn4 = assign10030_e12010_d_n4;
        locals.var_de_1_dn5 = assign10030_e12010_d_n5;
        locals.var_de_1_dn7 = assign10030_e12010_d_n7;
        locals.var_de_1_dn8 = assign10030_e12010_d_n8;
        locals.var_de_1_dn9 = assign10030_e12010_d_n9;
        locals.var_de_1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_27(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10040_e12021, assign10040_e12021_d_n0, assign10040_e12021_d_n1, assign10040_e12021_d_n3, assign10040_e12021_d_n4, assign10040_e12021_d_n5, assign10040_e12021_d_n7, assign10040_e12021_d_n8, assign10040_e12021_d_n9,) = {
    if ((((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) && (locals.var_guard211 == 0.0)) {
        (locals.var_vsc, locals.var_vsc_dn0, 0.0, locals.var_vsc_dn3, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dv_j1, locals.var_dv_j1_dn0, locals.var_dv_j1_dn1, locals.var_dv_j1_dn3, locals.var_dv_j1_dn4, locals.var_dv_j1_dn5, locals.var_dv_j1_dn7, locals.var_dv_j1_dn8, locals.var_dv_j1_dn9,)
    }
};
        locals.var_dv_j1 = assign10040_e12021;
        locals.var_dv_j1_dn0 = assign10040_e12021_d_n0;
        locals.var_dv_j1_dn1 = assign10040_e12021_d_n1;
        locals.var_dv_j1_dn3 = assign10040_e12021_d_n3;
        locals.var_dv_j1_dn4 = assign10040_e12021_d_n4;
        locals.var_dv_j1_dn5 = assign10040_e12021_d_n5;
        locals.var_dv_j1_dn7 = assign10040_e12021_d_n7;
        locals.var_dv_j1_dn8 = assign10040_e12021_d_n8;
        locals.var_dv_j1_dn9 = assign10040_e12021_d_n9;
        locals.var_dv_j1_rv = 0.0;

        let (assign10050_e12035, assign10050_e12035_d_n4,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) {
        let assign10050_e12029: f64 = (0.1 * locals.var_dv_p);
        let assign10050_e12032: f64 = (4.0 * locals.var_vt);
        let assign10050_e12033: f64 = (assign10050_e12029 + assign10050_e12032);
        (assign10050_e12033, ((0.1 * locals.var_dv_p_dn4) + (4.0 * locals.var_vt_dn4)),)
    } else {
        (locals.var_da, locals.var_da_dn4,)
    }
};
        locals.var_da = assign10050_e12035;
        locals.var_da_dn4 = assign10050_e12035_d_n4;
        locals.var_da_rv = 0.0;

        let (assign10060_e12047, assign10060_e12047_d_n0, assign10060_e12047_d_n1, assign10060_e12047_d_n3, assign10060_e12047_d_n4, assign10060_e12047_d_n5, assign10060_e12047_d_n7, assign10060_e12047_d_n8, assign10060_e12047_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) {
        let assign10060_e12043: f64 = (locals.var_dv_p + locals.var_dv_j1);
        let assign10060_e12045: f64 = (assign10060_e12043 / locals.var_da);
        (assign10060_e12045, (locals.var_dv_j1_dn0 / locals.var_da), (locals.var_dv_j1_dn1 / locals.var_da), (locals.var_dv_j1_dn3 / locals.var_da), ((((locals.var_dv_p_dn4 + locals.var_dv_j1_dn4) * locals.var_da) - (assign10060_e12043 * locals.var_da_dn4)) / (locals.var_da * locals.var_da)), (locals.var_dv_j1_dn5 / locals.var_da), (locals.var_dv_j1_dn7 / locals.var_da), (locals.var_dv_j1_dn8 / locals.var_da), (locals.var_dv_j1_dn9 / locals.var_da),)
    } else {
        (locals.var_dv_r, locals.var_dv_r_dn0, locals.var_dv_r_dn1, locals.var_dv_r_dn3, locals.var_dv_r_dn4, locals.var_dv_r_dn5, locals.var_dv_r_dn7, locals.var_dv_r_dn8, locals.var_dv_r_dn9,)
    }
};
        locals.var_dv_r = assign10060_e12047;
        locals.var_dv_r_dn0 = assign10060_e12047_d_n0;
        locals.var_dv_r_dn1 = assign10060_e12047_d_n1;
        locals.var_dv_r_dn3 = assign10060_e12047_d_n3;
        locals.var_dv_r_dn4 = assign10060_e12047_d_n4;
        locals.var_dv_r_dn5 = assign10060_e12047_d_n5;
        locals.var_dv_r_dn7 = assign10060_e12047_d_n7;
        locals.var_dv_r_dn8 = assign10060_e12047_d_n8;
        locals.var_dv_r_dn9 = assign10060_e12047_d_n9;
        locals.var_dv_r_rv = 0.0;

        let assign10070_e12050: f64 = if locals.var_dv_r < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard212 = assign10070_e12050;
        locals.var_guard212_rv = 0.0;

        let (assign10080_e12061, assign10080_e12061_d_n0, assign10080_e12061_d_n1, assign10080_e12061_d_n3, assign10080_e12061_d_n4, assign10080_e12061_d_n5, assign10080_e12061_d_n7, assign10080_e12061_d_n8, assign10080_e12061_d_n9,) = {
    if ((((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) && (locals.var_guard212 != 0.0)) {
        let assign10080_e12059: f64 = (locals.var_dv_r).exp();
        (assign10080_e12059, (assign10080_e12059 * locals.var_dv_r_dn0), (assign10080_e12059 * locals.var_dv_r_dn1), (assign10080_e12059 * locals.var_dv_r_dn3), (assign10080_e12059 * locals.var_dv_r_dn4), (assign10080_e12059 * locals.var_dv_r_dn5), (assign10080_e12059 * locals.var_dv_r_dn7), (assign10080_e12059 * locals.var_dv_r_dn8), (assign10080_e12059 * locals.var_dv_r_dn9),)
    } else {
        (locals.var_de, locals.var_de_dn0, locals.var_de_dn1, locals.var_de_dn3, locals.var_de_dn4, locals.var_de_dn5, locals.var_de_dn7, locals.var_de_dn8, locals.var_de_dn9,)
    }
};
        locals.var_de = assign10080_e12061;
        locals.var_de_dn0 = assign10080_e12061_d_n0;
        locals.var_de_dn1 = assign10080_e12061_d_n1;
        locals.var_de_dn3 = assign10080_e12061_d_n3;
        locals.var_de_dn4 = assign10080_e12061_d_n4;
        locals.var_de_dn5 = assign10080_e12061_d_n5;
        locals.var_de_dn7 = assign10080_e12061_d_n7;
        locals.var_de_dn8 = assign10080_e12061_d_n8;
        locals.var_de_dn9 = assign10080_e12061_d_n9;
        locals.var_de_rv = 0.0;

        let (assign10090_e12075, assign10090_e12075_d_n0, assign10090_e12075_d_n1, assign10090_e12075_d_n3, assign10090_e12075_d_n4, assign10090_e12075_d_n5, assign10090_e12075_d_n7, assign10090_e12075_d_n8, assign10090_e12075_d_n9,) = {
    if ((((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) && (locals.var_guard212 != 0.0)) {
        let assign10090_e12072: f64 = (1.0 + locals.var_de);
        let assign10090_e12073: f64 = (locals.var_de / assign10090_e12072);
        (assign10090_e12073, (((locals.var_de_dn0 * assign10090_e12072) - (locals.var_de * locals.var_de_dn0)) / (assign10090_e12072 * assign10090_e12072)), (((locals.var_de_dn1 * assign10090_e12072) - (locals.var_de * locals.var_de_dn1)) / (assign10090_e12072 * assign10090_e12072)), (((locals.var_de_dn3 * assign10090_e12072) - (locals.var_de * locals.var_de_dn3)) / (assign10090_e12072 * assign10090_e12072)), (((locals.var_de_dn4 * assign10090_e12072) - (locals.var_de * locals.var_de_dn4)) / (assign10090_e12072 * assign10090_e12072)), (((locals.var_de_dn5 * assign10090_e12072) - (locals.var_de * locals.var_de_dn5)) / (assign10090_e12072 * assign10090_e12072)), (((locals.var_de_dn7 * assign10090_e12072) - (locals.var_de * locals.var_de_dn7)) / (assign10090_e12072 * assign10090_e12072)), (((locals.var_de_dn8 * assign10090_e12072) - (locals.var_de * locals.var_de_dn8)) / (assign10090_e12072 * assign10090_e12072)), (((locals.var_de_dn9 * assign10090_e12072) - (locals.var_de * locals.var_de_dn9)) / (assign10090_e12072 * assign10090_e12072)),)
    } else {
        (locals.var_de_2, locals.var_de_2_dn0, locals.var_de_2_dn1, locals.var_de_2_dn3, locals.var_de_2_dn4, locals.var_de_2_dn5, locals.var_de_2_dn7, locals.var_de_2_dn8, locals.var_de_2_dn9,)
    }
};
        locals.var_de_2 = assign10090_e12075;
        locals.var_de_2_dn0 = assign10090_e12075_d_n0;
        locals.var_de_2_dn1 = assign10090_e12075_d_n1;
        locals.var_de_2_dn3 = assign10090_e12075_d_n3;
        locals.var_de_2_dn4 = assign10090_e12075_d_n4;
        locals.var_de_2_dn5 = assign10090_e12075_d_n5;
        locals.var_de_2_dn7 = assign10090_e12075_d_n7;
        locals.var_de_2_dn8 = assign10090_e12075_d_n8;
        locals.var_de_2_dn9 = assign10090_e12075_d_n9;
        locals.var_de_2_rv = 0.0;

        let (assign10100_e12101, assign10100_e12101_d_n0, assign10100_e12101_d_n1, assign10100_e12101_d_n3, assign10100_e12101_d_n4, assign10100_e12101_d_n5, assign10100_e12101_d_n7, assign10100_e12101_d_n8, assign10100_e12101_d_n9,) = {
    if ((((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) && (locals.var_guard212 != 0.0)) {
        let assign10100_e12084: f64 = (-locals.var_dv_p);
        let assign10100_e12088: f64 = (1.0 + locals.var_de);
        let assign10100_e12089: f64 = (assign10100_e12088).ln();
        let assign10100_e12092: f64 = (locals.var_dv_p + locals.var_dv_f);
        let assign10100_e12093: f64 = (-assign10100_e12092);
        let assign10100_e12095: f64 = (assign10100_e12093 / locals.var_da);
        let assign10100_e12096: f64 = (assign10100_e12095).exp();
        let assign10100_e12097: f64 = (assign10100_e12089 - assign10100_e12096);
        let assign10100_e12098: f64 = (locals.var_da * assign10100_e12097);
        let assign10100_e12099: f64 = (assign10100_e12084 + assign10100_e12098);
        (assign10100_e12099, (locals.var_da * (locals.var_de_dn0 / assign10100_e12088)), (locals.var_da * (locals.var_de_dn1 / assign10100_e12088)), (locals.var_da * (locals.var_de_dn3 / assign10100_e12088)), ((-locals.var_dv_p_dn4) + ((locals.var_da_dn4 * assign10100_e12097) + (locals.var_da * ((locals.var_de_dn4 / assign10100_e12088) - (assign10100_e12096 * ((((-(locals.var_dv_p_dn4 + locals.var_dv_f_dn4)) * locals.var_da) - (assign10100_e12093 * locals.var_da_dn4)) / (locals.var_da * locals.var_da))))))), (locals.var_da * (locals.var_de_dn5 / assign10100_e12088)), (locals.var_da * (locals.var_de_dn7 / assign10100_e12088)), (locals.var_da * (locals.var_de_dn8 / assign10100_e12088)), (locals.var_da * (locals.var_de_dn9 / assign10100_e12088)),)
    } else {
        (locals.var_dv_j2, locals.var_dv_j2_dn0, locals.var_dv_j2_dn1, locals.var_dv_j2_dn3, locals.var_dv_j2_dn4, locals.var_dv_j2_dn5, locals.var_dv_j2_dn7, locals.var_dv_j2_dn8, locals.var_dv_j2_dn9,)
    }
};
        locals.var_dv_j2 = assign10100_e12101;
        locals.var_dv_j2_dn0 = assign10100_e12101_d_n0;
        locals.var_dv_j2_dn1 = assign10100_e12101_d_n1;
        locals.var_dv_j2_dn3 = assign10100_e12101_d_n3;
        locals.var_dv_j2_dn4 = assign10100_e12101_d_n4;
        locals.var_dv_j2_dn5 = assign10100_e12101_d_n5;
        locals.var_dv_j2_dn7 = assign10100_e12101_d_n7;
        locals.var_dv_j2_dn8 = assign10100_e12101_d_n8;
        locals.var_dv_j2_dn9 = assign10100_e12101_d_n9;
        locals.var_dv_j2_rv = 0.0;

        let (assign10110_e12112, assign10110_e12112_d_n0, assign10110_e12112_d_n1, assign10110_e12112_d_n3, assign10110_e12112_d_n4, assign10110_e12112_d_n5, assign10110_e12112_d_n7, assign10110_e12112_d_n8, assign10110_e12112_d_n9,) = {
    if ((((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) && (locals.var_guard212 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_de_2, locals.var_de_2_dn0, locals.var_de_2_dn1, locals.var_de_2_dn3, locals.var_de_2_dn4, locals.var_de_2_dn5, locals.var_de_2_dn7, locals.var_de_2_dn8, locals.var_de_2_dn9,)
    }
};
        locals.var_de_2 = assign10110_e12112;
        locals.var_de_2_dn0 = assign10110_e12112_d_n0;
        locals.var_de_2_dn1 = assign10110_e12112_d_n1;
        locals.var_de_2_dn3 = assign10110_e12112_d_n3;
        locals.var_de_2_dn4 = assign10110_e12112_d_n4;
        locals.var_de_2_dn5 = assign10110_e12112_d_n5;
        locals.var_de_2_dn7 = assign10110_e12112_d_n7;
        locals.var_de_2_dn8 = assign10110_e12112_d_n8;
        locals.var_de_2_dn9 = assign10110_e12112_d_n9;
        locals.var_de_2_rv = 0.0;

        let (assign10120_e12123, assign10120_e12123_d_n0, assign10120_e12123_d_n1, assign10120_e12123_d_n3, assign10120_e12123_d_n4, assign10120_e12123_d_n5, assign10120_e12123_d_n7, assign10120_e12123_d_n8, assign10120_e12123_d_n9,) = {
    if ((((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) && (locals.var_guard212 == 0.0)) {
        (locals.var_dv_j1, locals.var_dv_j1_dn0, locals.var_dv_j1_dn1, locals.var_dv_j1_dn3, locals.var_dv_j1_dn4, locals.var_dv_j1_dn5, locals.var_dv_j1_dn7, locals.var_dv_j1_dn8, locals.var_dv_j1_dn9,)
    } else {
        (locals.var_dv_j2, locals.var_dv_j2_dn0, locals.var_dv_j2_dn1, locals.var_dv_j2_dn3, locals.var_dv_j2_dn4, locals.var_dv_j2_dn5, locals.var_dv_j2_dn7, locals.var_dv_j2_dn8, locals.var_dv_j2_dn9,)
    }
};
        locals.var_dv_j2 = assign10120_e12123;
        locals.var_dv_j2_dn0 = assign10120_e12123_d_n0;
        locals.var_dv_j2_dn1 = assign10120_e12123_d_n1;
        locals.var_dv_j2_dn3 = assign10120_e12123_d_n3;
        locals.var_dv_j2_dn4 = assign10120_e12123_d_n4;
        locals.var_dv_j2_dn5 = assign10120_e12123_d_n5;
        locals.var_dv_j2_dn7 = assign10120_e12123_d_n7;
        locals.var_dv_j2_dn8 = assign10120_e12123_d_n8;
        locals.var_dv_j2_dn9 = assign10120_e12123_d_n9;
        locals.var_dv_j2_rv = 0.0;

        let (assign10130_e12133, assign10130_e12133_d_n0, assign10130_e12133_d_n1, assign10130_e12133_d_n3, assign10130_e12133_d_n4, assign10130_e12133_d_n5, assign10130_e12133_d_n7, assign10130_e12133_d_n8, assign10130_e12133_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) {
        let assign10130_e12131: f64 = (locals.var_vsc - locals.var_dv_j1);
        (assign10130_e12131, (locals.var_vsc_dn0 - locals.var_dv_j1_dn0), (-locals.var_dv_j1_dn1), (locals.var_vsc_dn3 - locals.var_dv_j1_dn3), (-locals.var_dv_j1_dn4), (-locals.var_dv_j1_dn5), (-locals.var_dv_j1_dn7), (-locals.var_dv_j1_dn8), (-locals.var_dv_j1_dn9),)
    } else {
        (locals.var_dv_j4, locals.var_dv_j4_dn0, locals.var_dv_j4_dn1, locals.var_dv_j4_dn3, locals.var_dv_j4_dn4, locals.var_dv_j4_dn5, locals.var_dv_j4_dn7, locals.var_dv_j4_dn8, locals.var_dv_j4_dn9,)
    }
};
        locals.var_dv_j4 = assign10130_e12133;
        locals.var_dv_j4_dn0 = assign10130_e12133_d_n0;
        locals.var_dv_j4_dn1 = assign10130_e12133_d_n1;
        locals.var_dv_j4_dn3 = assign10130_e12133_d_n3;
        locals.var_dv_j4_dn4 = assign10130_e12133_d_n4;
        locals.var_dv_j4_dn5 = assign10130_e12133_d_n5;
        locals.var_dv_j4_dn7 = assign10130_e12133_d_n7;
        locals.var_dv_j4_dn8 = assign10130_e12133_d_n8;
        locals.var_dv_j4_dn9 = assign10130_e12133_d_n9;
        locals.var_dv_j4_rv = 0.0;

        let (assign10140_e12146, assign10140_e12146_d_n0, assign10140_e12146_d_n1, assign10140_e12146_d_n3, assign10140_e12146_d_n4, assign10140_e12146_d_n5, assign10140_e12146_d_n7, assign10140_e12146_d_n8, assign10140_e12146_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) {
        let assign10140_e12142: f64 = (locals.var_dv_j1 / locals.var_vdsp_t);
        let assign10140_e12143: f64 = (1.0 - assign10140_e12142);
        let assign10140_e12144: f64 = (assign10140_e12143).ln();
        (assign10140_e12144, ((-(locals.var_dv_j1_dn0 / locals.var_vdsp_t)) / assign10140_e12143), ((-(locals.var_dv_j1_dn1 / locals.var_vdsp_t)) / assign10140_e12143), ((-(locals.var_dv_j1_dn3 / locals.var_vdsp_t)) / assign10140_e12143), ((-(((locals.var_dv_j1_dn4 * locals.var_vdsp_t) - (locals.var_dv_j1 * locals.var_vdsp_t_dn4)) / (locals.var_vdsp_t * locals.var_vdsp_t))) / assign10140_e12143), ((-(locals.var_dv_j1_dn5 / locals.var_vdsp_t)) / assign10140_e12143), ((-(locals.var_dv_j1_dn7 / locals.var_vdsp_t)) / assign10140_e12143), ((-(locals.var_dv_j1_dn8 / locals.var_vdsp_t)) / assign10140_e12143), ((-(locals.var_dv_j1_dn9 / locals.var_vdsp_t)) / assign10140_e12143),)
    } else {
        (locals.var_dcln1, locals.var_dcln1_dn0, locals.var_dcln1_dn1, locals.var_dcln1_dn3, locals.var_dcln1_dn4, locals.var_dcln1_dn5, locals.var_dcln1_dn7, locals.var_dcln1_dn8, locals.var_dcln1_dn9,)
    }
};
        locals.var_dcln1 = assign10140_e12146;
        locals.var_dcln1_dn0 = assign10140_e12146_d_n0;
        locals.var_dcln1_dn1 = assign10140_e12146_d_n1;
        locals.var_dcln1_dn3 = assign10140_e12146_d_n3;
        locals.var_dcln1_dn4 = assign10140_e12146_d_n4;
        locals.var_dcln1_dn5 = assign10140_e12146_d_n5;
        locals.var_dcln1_dn7 = assign10140_e12146_d_n7;
        locals.var_dcln1_dn8 = assign10140_e12146_d_n8;
        locals.var_dcln1_dn9 = assign10140_e12146_d_n9;
        locals.var_dcln1_rv = 0.0;

        let (assign10150_e12159, assign10150_e12159_d_n0, assign10150_e12159_d_n1, assign10150_e12159_d_n3, assign10150_e12159_d_n4, assign10150_e12159_d_n5, assign10150_e12159_d_n7, assign10150_e12159_d_n8, assign10150_e12159_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) {
        let assign10150_e12155: f64 = (locals.var_dv_j2 / locals.var_vdsp_t);
        let assign10150_e12156: f64 = (1.0 - assign10150_e12155);
        let assign10150_e12157: f64 = (assign10150_e12156).ln();
        (assign10150_e12157, ((-(locals.var_dv_j2_dn0 / locals.var_vdsp_t)) / assign10150_e12156), ((-(locals.var_dv_j2_dn1 / locals.var_vdsp_t)) / assign10150_e12156), ((-(locals.var_dv_j2_dn3 / locals.var_vdsp_t)) / assign10150_e12156), ((-(((locals.var_dv_j2_dn4 * locals.var_vdsp_t) - (locals.var_dv_j2 * locals.var_vdsp_t_dn4)) / (locals.var_vdsp_t * locals.var_vdsp_t))) / assign10150_e12156), ((-(locals.var_dv_j2_dn5 / locals.var_vdsp_t)) / assign10150_e12156), ((-(locals.var_dv_j2_dn7 / locals.var_vdsp_t)) / assign10150_e12156), ((-(locals.var_dv_j2_dn8 / locals.var_vdsp_t)) / assign10150_e12156), ((-(locals.var_dv_j2_dn9 / locals.var_vdsp_t)) / assign10150_e12156),)
    } else {
        (locals.var_dcln2, locals.var_dcln2_dn0, locals.var_dcln2_dn1, locals.var_dcln2_dn3, locals.var_dcln2_dn4, locals.var_dcln2_dn5, locals.var_dcln2_dn7, locals.var_dcln2_dn8, locals.var_dcln2_dn9,)
    }
};
        locals.var_dcln2 = assign10150_e12159;
        locals.var_dcln2_dn0 = assign10150_e12159_d_n0;
        locals.var_dcln2_dn1 = assign10150_e12159_d_n1;
        locals.var_dcln2_dn3 = assign10150_e12159_d_n3;
        locals.var_dcln2_dn4 = assign10150_e12159_d_n4;
        locals.var_dcln2_dn5 = assign10150_e12159_d_n5;
        locals.var_dcln2_dn7 = assign10150_e12159_d_n7;
        locals.var_dcln2_dn8 = assign10150_e12159_d_n8;
        locals.var_dcln2_dn9 = assign10150_e12159_d_n9;
        locals.var_dcln2_rv = 0.0;

        let (assign10160_e12169,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) {
        let assign10160_e12167: f64 = (1.0 - p.p64);
        (assign10160_e12167,)
    } else {
        (locals.var_dz1,)
    }
};
        locals.var_dz1 = assign10160_e12169;
        locals.var_dz1_rv = 0.0;

        let (assign10170_e12179,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) {
        let assign10170_e12177: f64 = (1.0 - locals.var_dz_r);
        (assign10170_e12177,)
    } else {
        (locals.var_dzr1,)
    }
};
        locals.var_dzr1 = assign10170_e12179;
        locals.var_dzr1_rv = 0.0;

        let (assign10180_e12197, assign10180_e12197_d_n0, assign10180_e12197_d_n1, assign10180_e12197_d_n3, assign10180_e12197_d_n4, assign10180_e12197_d_n5, assign10180_e12197_d_n7, assign10180_e12197_d_n8, assign10180_e12197_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) {
        let assign10180_e12188: f64 = (-p.p64);
        let assign10180_e12189: f64 = (locals.var_dcln2 * assign10180_e12188);
        let assign10180_e12190: f64 = (assign10180_e12189).exp();
        let assign10180_e12191: f64 = (locals.var_cscp0_t * assign10180_e12190);
        let assign10180_e12193: f64 = (assign10180_e12191 * locals.var_de_1);
        let assign10180_e12195: f64 = (assign10180_e12193 * locals.var_de_2);
        (assign10180_e12195, (((((locals.var_cscp0_t * (assign10180_e12190 * (locals.var_dcln2_dn0 * assign10180_e12188))) * locals.var_de_1) + (assign10180_e12191 * locals.var_de_1_dn0)) * locals.var_de_2) + (assign10180_e12193 * locals.var_de_2_dn0)), (((((locals.var_cscp0_t * (assign10180_e12190 * (locals.var_dcln2_dn1 * assign10180_e12188))) * locals.var_de_1) + (assign10180_e12191 * locals.var_de_1_dn1)) * locals.var_de_2) + (assign10180_e12193 * locals.var_de_2_dn1)), (((((locals.var_cscp0_t * (assign10180_e12190 * (locals.var_dcln2_dn3 * assign10180_e12188))) * locals.var_de_1) + (assign10180_e12191 * locals.var_de_1_dn3)) * locals.var_de_2) + (assign10180_e12193 * locals.var_de_2_dn3)), ((((((locals.var_cscp0_t_dn4 * assign10180_e12190) + (locals.var_cscp0_t * (assign10180_e12190 * (locals.var_dcln2_dn4 * assign10180_e12188)))) * locals.var_de_1) + (assign10180_e12191 * locals.var_de_1_dn4)) * locals.var_de_2) + (assign10180_e12193 * locals.var_de_2_dn4)), (((((locals.var_cscp0_t * (assign10180_e12190 * (locals.var_dcln2_dn5 * assign10180_e12188))) * locals.var_de_1) + (assign10180_e12191 * locals.var_de_1_dn5)) * locals.var_de_2) + (assign10180_e12193 * locals.var_de_2_dn5)), (((((locals.var_cscp0_t * (assign10180_e12190 * (locals.var_dcln2_dn7 * assign10180_e12188))) * locals.var_de_1) + (assign10180_e12191 * locals.var_de_1_dn7)) * locals.var_de_2) + (assign10180_e12193 * locals.var_de_2_dn7)), (((((locals.var_cscp0_t * (assign10180_e12190 * (locals.var_dcln2_dn8 * assign10180_e12188))) * locals.var_de_1) + (assign10180_e12191 * locals.var_de_1_dn8)) * locals.var_de_2) + (assign10180_e12193 * locals.var_de_2_dn8)), (((((locals.var_cscp0_t * (assign10180_e12190 * (locals.var_dcln2_dn9 * assign10180_e12188))) * locals.var_de_1) + (assign10180_e12191 * locals.var_de_1_dn9)) * locals.var_de_2) + (assign10180_e12193 * locals.var_de_2_dn9)),)
    } else {
        (locals.var_dc_j1, locals.var_dc_j1_dn0, locals.var_dc_j1_dn1, locals.var_dc_j1_dn3, locals.var_dc_j1_dn4, locals.var_dc_j1_dn5, locals.var_dc_j1_dn7, locals.var_dc_j1_dn8, locals.var_dc_j1_dn9,)
    }
};
        locals.var_dc_j1 = assign10180_e12197;
        locals.var_dc_j1_dn0 = assign10180_e12197_d_n0;
        locals.var_dc_j1_dn1 = assign10180_e12197_d_n1;
        locals.var_dc_j1_dn3 = assign10180_e12197_d_n3;
        locals.var_dc_j1_dn4 = assign10180_e12197_d_n4;
        locals.var_dc_j1_dn5 = assign10180_e12197_d_n5;
        locals.var_dc_j1_dn7 = assign10180_e12197_d_n7;
        locals.var_dc_j1_dn8 = assign10180_e12197_d_n8;
        locals.var_dc_j1_dn9 = assign10180_e12197_d_n9;
        locals.var_dc_j1_rv = 0.0;

        let (assign10190_e12215, assign10190_e12215_d_n0, assign10190_e12215_d_n1, assign10190_e12215_d_n3, assign10190_e12215_d_n4, assign10190_e12215_d_n5, assign10190_e12215_d_n7, assign10190_e12215_d_n8, assign10190_e12215_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) {
        let assign10190_e12206: f64 = (-locals.var_dz_r);
        let assign10190_e12207: f64 = (locals.var_dcln1 * assign10190_e12206);
        let assign10190_e12208: f64 = (assign10190_e12207).exp();
        let assign10190_e12209: f64 = (locals.var_dc_c * assign10190_e12208);
        let assign10190_e12212: f64 = (1.0 - locals.var_de_2);
        let assign10190_e12213: f64 = (assign10190_e12209 * assign10190_e12212);
        (assign10190_e12213, (((locals.var_dc_c * (assign10190_e12208 * (locals.var_dcln1_dn0 * assign10190_e12206))) * assign10190_e12212) + (assign10190_e12209 * (-locals.var_de_2_dn0))), (((locals.var_dc_c * (assign10190_e12208 * (locals.var_dcln1_dn1 * assign10190_e12206))) * assign10190_e12212) + (assign10190_e12209 * (-locals.var_de_2_dn1))), (((locals.var_dc_c * (assign10190_e12208 * (locals.var_dcln1_dn3 * assign10190_e12206))) * assign10190_e12212) + (assign10190_e12209 * (-locals.var_de_2_dn3))), ((((locals.var_dc_c_dn4 * assign10190_e12208) + (locals.var_dc_c * (assign10190_e12208 * (locals.var_dcln1_dn4 * assign10190_e12206)))) * assign10190_e12212) + (assign10190_e12209 * (-locals.var_de_2_dn4))), (((locals.var_dc_c * (assign10190_e12208 * (locals.var_dcln1_dn5 * assign10190_e12206))) * assign10190_e12212) + (assign10190_e12209 * (-locals.var_de_2_dn5))), (((locals.var_dc_c * (assign10190_e12208 * (locals.var_dcln1_dn7 * assign10190_e12206))) * assign10190_e12212) + (assign10190_e12209 * (-locals.var_de_2_dn7))), (((locals.var_dc_c * (assign10190_e12208 * (locals.var_dcln1_dn8 * assign10190_e12206))) * assign10190_e12212) + (assign10190_e12209 * (-locals.var_de_2_dn8))), (((locals.var_dc_c * (assign10190_e12208 * (locals.var_dcln1_dn9 * assign10190_e12206))) * assign10190_e12212) + (assign10190_e12209 * (-locals.var_de_2_dn9))),)
    } else {
        (locals.var_dc_j2, locals.var_dc_j2_dn0, locals.var_dc_j2_dn1, locals.var_dc_j2_dn3, locals.var_dc_j2_dn4, locals.var_dc_j2_dn5, locals.var_dc_j2_dn7, locals.var_dc_j2_dn8, locals.var_dc_j2_dn9,)
    }
};
        locals.var_dc_j2 = assign10190_e12215;
        locals.var_dc_j2_dn0 = assign10190_e12215_d_n0;
        locals.var_dc_j2_dn1 = assign10190_e12215_d_n1;
        locals.var_dc_j2_dn3 = assign10190_e12215_d_n3;
        locals.var_dc_j2_dn4 = assign10190_e12215_d_n4;
        locals.var_dc_j2_dn5 = assign10190_e12215_d_n5;
        locals.var_dc_j2_dn7 = assign10190_e12215_d_n7;
        locals.var_dc_j2_dn8 = assign10190_e12215_d_n8;
        locals.var_dc_j2_dn9 = assign10190_e12215_d_n9;
        locals.var_dc_j2_rv = 0.0;

        let (assign10200_e12227, assign10200_e12227_d_n0, assign10200_e12227_d_n1, assign10200_e12227_d_n3, assign10200_e12227_d_n4, assign10200_e12227_d_n5, assign10200_e12227_d_n7, assign10200_e12227_d_n8, assign10200_e12227_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) {
        let assign10200_e12224: f64 = (1.0 - locals.var_de_1);
        let assign10200_e12225: f64 = (locals.var_dc_max * assign10200_e12224);
        (assign10200_e12225, (locals.var_dc_max * (-locals.var_de_1_dn0)), (locals.var_dc_max * (-locals.var_de_1_dn1)), (locals.var_dc_max * (-locals.var_de_1_dn3)), ((locals.var_dc_max_dn4 * assign10200_e12224) + (locals.var_dc_max * (-locals.var_de_1_dn4))), (locals.var_dc_max * (-locals.var_de_1_dn5)), (locals.var_dc_max * (-locals.var_de_1_dn7)), (locals.var_dc_max * (-locals.var_de_1_dn8)), (locals.var_dc_max * (-locals.var_de_1_dn9)),)
    } else {
        (locals.var_dc_j3, locals.var_dc_j3_dn0, locals.var_dc_j3_dn1, locals.var_dc_j3_dn3, locals.var_dc_j3_dn4, locals.var_dc_j3_dn5, locals.var_dc_j3_dn7, locals.var_dc_j3_dn8, locals.var_dc_j3_dn9,)
    }
};
        locals.var_dc_j3 = assign10200_e12227;
        locals.var_dc_j3_dn0 = assign10200_e12227_d_n0;
        locals.var_dc_j3_dn1 = assign10200_e12227_d_n1;
        locals.var_dc_j3_dn3 = assign10200_e12227_d_n3;
        locals.var_dc_j3_dn4 = assign10200_e12227_d_n4;
        locals.var_dc_j3_dn5 = assign10200_e12227_d_n5;
        locals.var_dc_j3_dn7 = assign10200_e12227_d_n7;
        locals.var_dc_j3_dn8 = assign10200_e12227_d_n8;
        locals.var_dc_j3_dn9 = assign10200_e12227_d_n9;
        locals.var_dc_j3_rv = 0.0;

        let (assign10220_e12256, assign10220_e12256_d_n0, assign10220_e12256_d_n1, assign10220_e12256_d_n3, assign10220_e12256_d_n4, assign10220_e12256_d_n5, assign10220_e12256_d_n7, assign10220_e12256_d_n8, assign10220_e12256_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) {
        let assign10220_e12249: f64 = (locals.var_dcln2 * locals.var_dz1);
        let assign10220_e12250: f64 = (assign10220_e12249).exp();
        let assign10220_e12251: f64 = (1.0 - assign10220_e12250);
        let assign10220_e12252: f64 = (locals.var_cscp0_t * assign10220_e12251);
        let assign10220_e12254: f64 = (assign10220_e12252 / locals.var_dz1);
        (assign10220_e12254, ((locals.var_cscp0_t * (-(assign10220_e12250 * (locals.var_dcln2_dn0 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cscp0_t * (-(assign10220_e12250 * (locals.var_dcln2_dn1 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cscp0_t * (-(assign10220_e12250 * (locals.var_dcln2_dn3 * locals.var_dz1)))) / locals.var_dz1), (((locals.var_cscp0_t_dn4 * assign10220_e12251) + (locals.var_cscp0_t * (-(assign10220_e12250 * (locals.var_dcln2_dn4 * locals.var_dz1))))) / locals.var_dz1), ((locals.var_cscp0_t * (-(assign10220_e12250 * (locals.var_dcln2_dn5 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cscp0_t * (-(assign10220_e12250 * (locals.var_dcln2_dn7 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cscp0_t * (-(assign10220_e12250 * (locals.var_dcln2_dn8 * locals.var_dz1)))) / locals.var_dz1), ((locals.var_cscp0_t * (-(assign10220_e12250 * (locals.var_dcln2_dn9 * locals.var_dz1)))) / locals.var_dz1),)
    } else {
        (locals.var_dq_j1, locals.var_dq_j1_dn0, locals.var_dq_j1_dn1, locals.var_dq_j1_dn3, locals.var_dq_j1_dn4, locals.var_dq_j1_dn5, locals.var_dq_j1_dn7, locals.var_dq_j1_dn8, locals.var_dq_j1_dn9,)
    }
};
        locals.var_dq_j1 = assign10220_e12256;
        locals.var_dq_j1_dn0 = assign10220_e12256_d_n0;
        locals.var_dq_j1_dn1 = assign10220_e12256_d_n1;
        locals.var_dq_j1_dn3 = assign10220_e12256_d_n3;
        locals.var_dq_j1_dn4 = assign10220_e12256_d_n4;
        locals.var_dq_j1_dn5 = assign10220_e12256_d_n5;
        locals.var_dq_j1_dn7 = assign10220_e12256_d_n7;
        locals.var_dq_j1_dn8 = assign10220_e12256_d_n8;
        locals.var_dq_j1_dn9 = assign10220_e12256_d_n9;
        locals.var_dq_j1_rv = 0.0;

        let (assign10230_e12273, assign10230_e12273_d_n0, assign10230_e12273_d_n1, assign10230_e12273_d_n3, assign10230_e12273_d_n4, assign10230_e12273_d_n5, assign10230_e12273_d_n7, assign10230_e12273_d_n8, assign10230_e12273_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) {
        let assign10230_e12266: f64 = (locals.var_dcln1 * locals.var_dzr1);
        let assign10230_e12267: f64 = (assign10230_e12266).exp();
        let assign10230_e12268: f64 = (1.0 - assign10230_e12267);
        let assign10230_e12269: f64 = (locals.var_dc_c * assign10230_e12268);
        let assign10230_e12271: f64 = (assign10230_e12269 / locals.var_dzr1);
        (assign10230_e12271, ((locals.var_dc_c * (-(assign10230_e12267 * (locals.var_dcln1_dn0 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign10230_e12267 * (locals.var_dcln1_dn1 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign10230_e12267 * (locals.var_dcln1_dn3 * locals.var_dzr1)))) / locals.var_dzr1), (((locals.var_dc_c_dn4 * assign10230_e12268) + (locals.var_dc_c * (-(assign10230_e12267 * (locals.var_dcln1_dn4 * locals.var_dzr1))))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign10230_e12267 * (locals.var_dcln1_dn5 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign10230_e12267 * (locals.var_dcln1_dn7 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign10230_e12267 * (locals.var_dcln1_dn8 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign10230_e12267 * (locals.var_dcln1_dn9 * locals.var_dzr1)))) / locals.var_dzr1),)
    } else {
        (locals.var_dq_j2, locals.var_dq_j2_dn0, locals.var_dq_j2_dn1, locals.var_dq_j2_dn3, locals.var_dq_j2_dn4, locals.var_dq_j2_dn5, locals.var_dq_j2_dn7, locals.var_dq_j2_dn8, locals.var_dq_j2_dn9,)
    }
};
        locals.var_dq_j2 = assign10230_e12273;
        locals.var_dq_j2_dn0 = assign10230_e12273_d_n0;
        locals.var_dq_j2_dn1 = assign10230_e12273_d_n1;
        locals.var_dq_j2_dn3 = assign10230_e12273_d_n3;
        locals.var_dq_j2_dn4 = assign10230_e12273_d_n4;
        locals.var_dq_j2_dn5 = assign10230_e12273_d_n5;
        locals.var_dq_j2_dn7 = assign10230_e12273_d_n7;
        locals.var_dq_j2_dn8 = assign10230_e12273_d_n8;
        locals.var_dq_j2_dn9 = assign10230_e12273_d_n9;
        locals.var_dq_j2_rv = 0.0;

        let (assign10240_e12290, assign10240_e12290_d_n0, assign10240_e12290_d_n1, assign10240_e12290_d_n3, assign10240_e12290_d_n4, assign10240_e12290_d_n5, assign10240_e12290_d_n7, assign10240_e12290_d_n8, assign10240_e12290_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) {
        let assign10240_e12283: f64 = (locals.var_dcln2 * locals.var_dzr1);
        let assign10240_e12284: f64 = (assign10240_e12283).exp();
        let assign10240_e12285: f64 = (1.0 - assign10240_e12284);
        let assign10240_e12286: f64 = (locals.var_dc_c * assign10240_e12285);
        let assign10240_e12288: f64 = (assign10240_e12286 / locals.var_dzr1);
        (assign10240_e12288, ((locals.var_dc_c * (-(assign10240_e12284 * (locals.var_dcln2_dn0 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign10240_e12284 * (locals.var_dcln2_dn1 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign10240_e12284 * (locals.var_dcln2_dn3 * locals.var_dzr1)))) / locals.var_dzr1), (((locals.var_dc_c_dn4 * assign10240_e12285) + (locals.var_dc_c * (-(assign10240_e12284 * (locals.var_dcln2_dn4 * locals.var_dzr1))))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign10240_e12284 * (locals.var_dcln2_dn5 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign10240_e12284 * (locals.var_dcln2_dn7 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign10240_e12284 * (locals.var_dcln2_dn8 * locals.var_dzr1)))) / locals.var_dzr1), ((locals.var_dc_c * (-(assign10240_e12284 * (locals.var_dcln2_dn9 * locals.var_dzr1)))) / locals.var_dzr1),)
    } else {
        (locals.var_dq_j3, locals.var_dq_j3_dn0, locals.var_dq_j3_dn1, locals.var_dq_j3_dn3, locals.var_dq_j3_dn4, locals.var_dq_j3_dn5, locals.var_dq_j3_dn7, locals.var_dq_j3_dn8, locals.var_dq_j3_dn9,)
    }
};
        locals.var_dq_j3 = assign10240_e12290;
        locals.var_dq_j3_dn0 = assign10240_e12290_d_n0;
        locals.var_dq_j3_dn1 = assign10240_e12290_d_n1;
        locals.var_dq_j3_dn3 = assign10240_e12290_d_n3;
        locals.var_dq_j3_dn4 = assign10240_e12290_d_n4;
        locals.var_dq_j3_dn5 = assign10240_e12290_d_n5;
        locals.var_dq_j3_dn7 = assign10240_e12290_d_n7;
        locals.var_dq_j3_dn8 = assign10240_e12290_d_n8;
        locals.var_dq_j3_dn9 = assign10240_e12290_d_n9;
        locals.var_dq_j3_rv = 0.0;

        let (assign10250_e12308, assign10250_e12308_d_n0, assign10250_e12308_d_n1, assign10250_e12308_d_n3, assign10250_e12308_d_n4, assign10250_e12308_d_n5, assign10250_e12308_d_n6, assign10250_e12308_d_n7, assign10250_e12308_d_n8, assign10250_e12308_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 != 0.0)) {
        let assign10250_e12298: f64 = (locals.var_dq_j1 + locals.var_dq_j2);
        let assign10250_e12300: f64 = (assign10250_e12298 - locals.var_dq_j3);
        let assign10250_e12302: f64 = (assign10250_e12300 * locals.var_vdsp_t);
        let assign10250_e12305: f64 = (locals.var_dc_max * locals.var_dv_j4);
        let assign10250_e12306: f64 = (assign10250_e12302 + assign10250_e12305);
        (assign10250_e12306, ((((locals.var_dq_j1_dn0 + locals.var_dq_j2_dn0) - locals.var_dq_j3_dn0) * locals.var_vdsp_t) + (locals.var_dc_max * locals.var_dv_j4_dn0)), ((((locals.var_dq_j1_dn1 + locals.var_dq_j2_dn1) - locals.var_dq_j3_dn1) * locals.var_vdsp_t) + (locals.var_dc_max * locals.var_dv_j4_dn1)), ((((locals.var_dq_j1_dn3 + locals.var_dq_j2_dn3) - locals.var_dq_j3_dn3) * locals.var_vdsp_t) + (locals.var_dc_max * locals.var_dv_j4_dn3)), (((((locals.var_dq_j1_dn4 + locals.var_dq_j2_dn4) - locals.var_dq_j3_dn4) * locals.var_vdsp_t) + (assign10250_e12300 * locals.var_vdsp_t_dn4)) + ((locals.var_dc_max_dn4 * locals.var_dv_j4) + (locals.var_dc_max * locals.var_dv_j4_dn4))), ((((locals.var_dq_j1_dn5 + locals.var_dq_j2_dn5) - locals.var_dq_j3_dn5) * locals.var_vdsp_t) + (locals.var_dc_max * locals.var_dv_j4_dn5)), 0.0, ((((locals.var_dq_j1_dn7 + locals.var_dq_j2_dn7) - locals.var_dq_j3_dn7) * locals.var_vdsp_t) + (locals.var_dc_max * locals.var_dv_j4_dn7)), ((((locals.var_dq_j1_dn8 + locals.var_dq_j2_dn8) - locals.var_dq_j3_dn8) * locals.var_vdsp_t) + (locals.var_dc_max * locals.var_dv_j4_dn8)), ((((locals.var_dq_j1_dn9 + locals.var_dq_j2_dn9) - locals.var_dq_j3_dn9) * locals.var_vdsp_t) + (locals.var_dc_max * locals.var_dv_j4_dn9)),)
    } else {
        (locals.var_qscp, locals.var_qscp_dn0, locals.var_qscp_dn1, locals.var_qscp_dn3, locals.var_qscp_dn4, locals.var_qscp_dn5, locals.var_qscp_dn6, locals.var_qscp_dn7, locals.var_qscp_dn8, locals.var_qscp_dn9,)
    }
};
        locals.var_qscp = assign10250_e12308;
        locals.var_qscp_dn0 = assign10250_e12308_d_n0;
        locals.var_qscp_dn1 = assign10250_e12308_d_n1;
        locals.var_qscp_dn3 = assign10250_e12308_d_n3;
        locals.var_qscp_dn4 = assign10250_e12308_d_n4;
        locals.var_qscp_dn5 = assign10250_e12308_d_n5;
        locals.var_qscp_dn6 = assign10250_e12308_d_n6;
        locals.var_qscp_dn7 = assign10250_e12308_d_n7;
        locals.var_qscp_dn8 = assign10250_e12308_d_n8;
        locals.var_qscp_dn9 = assign10250_e12308_d_n9;
        locals.var_qscp_rv = 0.0;

        let (assign10270_e12326, assign10270_e12326_d_n0, assign10270_e12326_d_n1, assign10270_e12326_d_n3, assign10270_e12326_d_n4, assign10270_e12326_d_n5, assign10270_e12326_d_n6, assign10270_e12326_d_n7, assign10270_e12326_d_n8, assign10270_e12326_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 != 0.0)) && (locals.var_guard210 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qscp, locals.var_qscp_dn0, locals.var_qscp_dn1, locals.var_qscp_dn3, locals.var_qscp_dn4, locals.var_qscp_dn5, locals.var_qscp_dn6, locals.var_qscp_dn7, locals.var_qscp_dn8, locals.var_qscp_dn9,)
    }
};
        locals.var_qscp = assign10270_e12326;
        locals.var_qscp_dn0 = assign10270_e12326_d_n0;
        locals.var_qscp_dn1 = assign10270_e12326_d_n1;
        locals.var_qscp_dn3 = assign10270_e12326_d_n3;
        locals.var_qscp_dn4 = assign10270_e12326_d_n4;
        locals.var_qscp_dn5 = assign10270_e12326_d_n5;
        locals.var_qscp_dn6 = assign10270_e12326_d_n6;
        locals.var_qscp_dn7 = assign10270_e12326_d_n7;
        locals.var_qscp_dn8 = assign10270_e12326_d_n8;
        locals.var_qscp_dn9 = assign10270_e12326_d_n9;
        locals.var_qscp_rv = 0.0;

        let assign10280_e12329: f64 = if locals.var_cscp0_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard213 = assign10280_e12329;
        locals.var_guard213_rv = 0.0;

        let (assign10290_e12347, assign10290_e12347_d_n4,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 == 0.0)) && (locals.var_guard213 != 0.0)) {
        let assign10290_e12339: f64 = (locals.var_ajsp_t).ln();
        let assign10290_e12340: f64 = (-assign10290_e12339);
        let assign10290_e12342: f64 = (assign10290_e12340 / p.p64);
        let assign10290_e12343: f64 = (assign10290_e12342).exp();
        let assign10290_e12344: f64 = (1.0 - assign10290_e12343);
        let assign10290_e12345: f64 = (locals.var_vdsp_t * assign10290_e12344);
        (assign10290_e12345, ((locals.var_vdsp_t_dn4 * assign10290_e12344) + (locals.var_vdsp_t * (-(assign10290_e12343 * ((-(locals.var_ajsp_t_dn4 / locals.var_ajsp_t)) / p.p64))))),)
    } else {
        (locals.var_dfv_f, locals.var_dfv_f_dn4,)
    }
};
        locals.var_dfv_f = assign10290_e12347;
        locals.var_dfv_f_dn4 = assign10290_e12347_d_n4;
        locals.var_dfv_f_rv = 0.0;

        let (assign10300_e12360, assign10300_e12360_d_n0, assign10300_e12360_d_n1, assign10300_e12360_d_n3, assign10300_e12360_d_n4, assign10300_e12360_d_n5, assign10300_e12360_d_n6, assign10300_e12360_d_n7, assign10300_e12360_d_n8, assign10300_e12360_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 == 0.0)) && (locals.var_guard213 != 0.0)) {
        let assign10300_e12356: f64 = (locals.var_dfv_f - locals.var_vsc);
        let assign10300_e12358: f64 = (assign10300_e12356 * locals.var_ovt);
        (assign10300_e12358, ((-locals.var_vsc_dn0) * locals.var_ovt), 0.0, ((-locals.var_vsc_dn3) * locals.var_ovt), ((locals.var_dfv_f_dn4 * locals.var_ovt) + (assign10300_e12356 * locals.var_ovt_dn4)), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dfx, locals.var_dfx_dn0, locals.var_dfx_dn1, locals.var_dfx_dn3, locals.var_dfx_dn4, locals.var_dfx_dn5, locals.var_dfx_dn6, locals.var_dfx_dn7, locals.var_dfx_dn8, locals.var_dfx_dn9,)
    }
};
        locals.var_dfx = assign10300_e12360;
        locals.var_dfx_dn0 = assign10300_e12360_d_n0;
        locals.var_dfx_dn1 = assign10300_e12360_d_n1;
        locals.var_dfx_dn3 = assign10300_e12360_d_n3;
        locals.var_dfx_dn4 = assign10300_e12360_d_n4;
        locals.var_dfx_dn5 = assign10300_e12360_d_n5;
        locals.var_dfx_dn6 = assign10300_e12360_d_n6;
        locals.var_dfx_dn7 = assign10300_e12360_d_n7;
        locals.var_dfx_dn8 = assign10300_e12360_d_n8;
        locals.var_dfx_dn9 = assign10300_e12360_d_n9;
        locals.var_dfx_rv = 0.0;

        let (assign10310_e12374, assign10310_e12374_d_n0, assign10310_e12374_d_n1, assign10310_e12374_d_n3, assign10310_e12374_d_n4, assign10310_e12374_d_n5, assign10310_e12374_d_n6, assign10310_e12374_d_n7, assign10310_e12374_d_n8, assign10310_e12374_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 == 0.0)) && (locals.var_guard213 != 0.0)) {
        let assign10310_e12369: f64 = (locals.var_dfx * locals.var_dfx);
        let assign10310_e12371: f64 = (assign10310_e12369 + 1.921812);
        let assign10310_e12372: f64 = (assign10310_e12371).sqrt();
        (assign10310_e12372, (((locals.var_dfx_dn0 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn0)) / (2.0 * assign10310_e12372)), (((locals.var_dfx_dn1 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn1)) / (2.0 * assign10310_e12372)), (((locals.var_dfx_dn3 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn3)) / (2.0 * assign10310_e12372)), (((locals.var_dfx_dn4 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn4)) / (2.0 * assign10310_e12372)), (((locals.var_dfx_dn5 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn5)) / (2.0 * assign10310_e12372)), (((locals.var_dfx_dn6 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn6)) / (2.0 * assign10310_e12372)), (((locals.var_dfx_dn7 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn7)) / (2.0 * assign10310_e12372)), (((locals.var_dfx_dn8 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn8)) / (2.0 * assign10310_e12372)), (((locals.var_dfx_dn9 * locals.var_dfx) + (locals.var_dfx * locals.var_dfx_dn9)) / (2.0 * assign10310_e12372)),)
    } else {
        (locals.var_dfs_q, locals.var_dfs_q_dn0, locals.var_dfs_q_dn1, locals.var_dfs_q_dn3, locals.var_dfs_q_dn4, locals.var_dfs_q_dn5, locals.var_dfs_q_dn6, locals.var_dfs_q_dn7, locals.var_dfs_q_dn8, locals.var_dfs_q_dn9,)
    }
};
        locals.var_dfs_q = assign10310_e12374;
        locals.var_dfs_q_dn0 = assign10310_e12374_d_n0;
        locals.var_dfs_q_dn1 = assign10310_e12374_d_n1;
        locals.var_dfs_q_dn3 = assign10310_e12374_d_n3;
        locals.var_dfs_q_dn4 = assign10310_e12374_d_n4;
        locals.var_dfs_q_dn5 = assign10310_e12374_d_n5;
        locals.var_dfs_q_dn6 = assign10310_e12374_d_n6;
        locals.var_dfs_q_dn7 = assign10310_e12374_d_n7;
        locals.var_dfs_q_dn8 = assign10310_e12374_d_n8;
        locals.var_dfs_q_dn9 = assign10310_e12374_d_n9;
        locals.var_dfs_q_rv = 0.0;

        let (assign10320_e12387, assign10320_e12387_d_n0, assign10320_e12387_d_n1, assign10320_e12387_d_n3, assign10320_e12387_d_n4, assign10320_e12387_d_n5, assign10320_e12387_d_n6, assign10320_e12387_d_n7, assign10320_e12387_d_n8, assign10320_e12387_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 == 0.0)) && (locals.var_guard213 != 0.0)) {
        let assign10320_e12383: f64 = (locals.var_dfx + locals.var_dfs_q);
        let assign10320_e12385: f64 = (assign10320_e12383 * 0.5);
        (assign10320_e12385, ((locals.var_dfx_dn0 + locals.var_dfs_q_dn0) * 0.5), ((locals.var_dfx_dn1 + locals.var_dfs_q_dn1) * 0.5), ((locals.var_dfx_dn3 + locals.var_dfs_q_dn3) * 0.5), ((locals.var_dfx_dn4 + locals.var_dfs_q_dn4) * 0.5), ((locals.var_dfx_dn5 + locals.var_dfs_q_dn5) * 0.5), ((locals.var_dfx_dn6 + locals.var_dfs_q_dn6) * 0.5), ((locals.var_dfx_dn7 + locals.var_dfs_q_dn7) * 0.5), ((locals.var_dfx_dn8 + locals.var_dfs_q_dn8) * 0.5), ((locals.var_dfx_dn9 + locals.var_dfs_q_dn9) * 0.5),)
    } else {
        (locals.var_dfs_q2, locals.var_dfs_q2_dn0, locals.var_dfs_q2_dn1, locals.var_dfs_q2_dn3, locals.var_dfs_q2_dn4, locals.var_dfs_q2_dn5, locals.var_dfs_q2_dn6, locals.var_dfs_q2_dn7, locals.var_dfs_q2_dn8, locals.var_dfs_q2_dn9,)
    }
};
        locals.var_dfs_q2 = assign10320_e12387;
        locals.var_dfs_q2_dn0 = assign10320_e12387_d_n0;
        locals.var_dfs_q2_dn1 = assign10320_e12387_d_n1;
        locals.var_dfs_q2_dn3 = assign10320_e12387_d_n3;
        locals.var_dfs_q2_dn4 = assign10320_e12387_d_n4;
        locals.var_dfs_q2_dn5 = assign10320_e12387_d_n5;
        locals.var_dfs_q2_dn6 = assign10320_e12387_d_n6;
        locals.var_dfs_q2_dn7 = assign10320_e12387_d_n7;
        locals.var_dfs_q2_dn8 = assign10320_e12387_d_n8;
        locals.var_dfs_q2_dn9 = assign10320_e12387_d_n9;
        locals.var_dfs_q2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_28(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let (assign10330_e12400, assign10330_e12400_d_n0, assign10330_e12400_d_n1, assign10330_e12400_d_n3, assign10330_e12400_d_n4, assign10330_e12400_d_n5, assign10330_e12400_d_n6, assign10330_e12400_d_n7, assign10330_e12400_d_n8, assign10330_e12400_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 == 0.0)) && (locals.var_guard213 != 0.0)) {
        let assign10330_e12397: f64 = (locals.var_vt * locals.var_dfs_q2);
        let assign10330_e12398: f64 = (locals.var_dfv_f - assign10330_e12397);
        (assign10330_e12398, (-(locals.var_vt * locals.var_dfs_q2_dn0)), (-(locals.var_vt * locals.var_dfs_q2_dn1)), (-(locals.var_vt * locals.var_dfs_q2_dn3)), (locals.var_dfv_f_dn4 - ((locals.var_vt_dn4 * locals.var_dfs_q2) + (locals.var_vt * locals.var_dfs_q2_dn4))), (-(locals.var_vt * locals.var_dfs_q2_dn5)), (-(locals.var_vt * locals.var_dfs_q2_dn6)), (-(locals.var_vt * locals.var_dfs_q2_dn7)), (-(locals.var_vt * locals.var_dfs_q2_dn8)), (-(locals.var_vt * locals.var_dfs_q2_dn9)),)
    } else {
        (locals.var_dfv_j, locals.var_dfv_j_dn0, locals.var_dfv_j_dn1, locals.var_dfv_j_dn3, locals.var_dfv_j_dn4, locals.var_dfv_j_dn5, locals.var_dfv_j_dn6, locals.var_dfv_j_dn7, locals.var_dfv_j_dn8, locals.var_dfv_j_dn9,)
    }
};
        locals.var_dfv_j = assign10330_e12400;
        locals.var_dfv_j_dn0 = assign10330_e12400_d_n0;
        locals.var_dfv_j_dn1 = assign10330_e12400_d_n1;
        locals.var_dfv_j_dn3 = assign10330_e12400_d_n3;
        locals.var_dfv_j_dn4 = assign10330_e12400_d_n4;
        locals.var_dfv_j_dn5 = assign10330_e12400_d_n5;
        locals.var_dfv_j_dn6 = assign10330_e12400_d_n6;
        locals.var_dfv_j_dn7 = assign10330_e12400_d_n7;
        locals.var_dfv_j_dn8 = assign10330_e12400_d_n8;
        locals.var_dfv_j_dn9 = assign10330_e12400_d_n9;
        locals.var_dfv_j_rv = 0.0;

        let (assign10340_e12411, assign10340_e12411_d_n0, assign10340_e12411_d_n1, assign10340_e12411_d_n3, assign10340_e12411_d_n4, assign10340_e12411_d_n5, assign10340_e12411_d_n6, assign10340_e12411_d_n7, assign10340_e12411_d_n8, assign10340_e12411_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 == 0.0)) && (locals.var_guard213 != 0.0)) {
        let assign10340_e12409: f64 = (locals.var_dfs_q2 / locals.var_dfs_q);
        (assign10340_e12409, (((locals.var_dfs_q2_dn0 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn0)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn1 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn1)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn3 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn3)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn4 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn4)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn5 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn5)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn6 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn6)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn7 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn7)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn8 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn8)) / (locals.var_dfs_q * locals.var_dfs_q)), (((locals.var_dfs_q2_dn9 * locals.var_dfs_q) - (locals.var_dfs_q2 * locals.var_dfs_q_dn9)) / (locals.var_dfs_q * locals.var_dfs_q)),)
    } else {
        (locals.var_dfdvj_dv, locals.var_dfdvj_dv_dn0, locals.var_dfdvj_dv_dn1, locals.var_dfdvj_dv_dn3, locals.var_dfdvj_dv_dn4, locals.var_dfdvj_dv_dn5, locals.var_dfdvj_dv_dn6, locals.var_dfdvj_dv_dn7, locals.var_dfdvj_dv_dn8, locals.var_dfdvj_dv_dn9,)
    }
};
        locals.var_dfdvj_dv = assign10340_e12411;
        locals.var_dfdvj_dv_dn0 = assign10340_e12411_d_n0;
        locals.var_dfdvj_dv_dn1 = assign10340_e12411_d_n1;
        locals.var_dfdvj_dv_dn3 = assign10340_e12411_d_n3;
        locals.var_dfdvj_dv_dn4 = assign10340_e12411_d_n4;
        locals.var_dfdvj_dv_dn5 = assign10340_e12411_d_n5;
        locals.var_dfdvj_dv_dn6 = assign10340_e12411_d_n6;
        locals.var_dfdvj_dv_dn7 = assign10340_e12411_d_n7;
        locals.var_dfdvj_dv_dn8 = assign10340_e12411_d_n8;
        locals.var_dfdvj_dv_dn9 = assign10340_e12411_d_n9;
        locals.var_dfdvj_dv_rv = 0.0;

        let (assign10350_e12425, assign10350_e12425_d_n0, assign10350_e12425_d_n1, assign10350_e12425_d_n3, assign10350_e12425_d_n4, assign10350_e12425_d_n5, assign10350_e12425_d_n6, assign10350_e12425_d_n7, assign10350_e12425_d_n8, assign10350_e12425_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 == 0.0)) && (locals.var_guard213 != 0.0)) {
        let assign10350_e12421: f64 = (locals.var_dfv_j / locals.var_vdsp_t);
        let assign10350_e12422: f64 = (1.0 - assign10350_e12421);
        let assign10350_e12423: f64 = (assign10350_e12422).ln();
        (assign10350_e12423, ((-(locals.var_dfv_j_dn0 / locals.var_vdsp_t)) / assign10350_e12422), ((-(locals.var_dfv_j_dn1 / locals.var_vdsp_t)) / assign10350_e12422), ((-(locals.var_dfv_j_dn3 / locals.var_vdsp_t)) / assign10350_e12422), ((-(((locals.var_dfv_j_dn4 * locals.var_vdsp_t) - (locals.var_dfv_j * locals.var_vdsp_t_dn4)) / (locals.var_vdsp_t * locals.var_vdsp_t))) / assign10350_e12422), ((-(locals.var_dfv_j_dn5 / locals.var_vdsp_t)) / assign10350_e12422), ((-(locals.var_dfv_j_dn6 / locals.var_vdsp_t)) / assign10350_e12422), ((-(locals.var_dfv_j_dn7 / locals.var_vdsp_t)) / assign10350_e12422), ((-(locals.var_dfv_j_dn8 / locals.var_vdsp_t)) / assign10350_e12422), ((-(locals.var_dfv_j_dn9 / locals.var_vdsp_t)) / assign10350_e12422),)
    } else {
        (locals.var_dfb, locals.var_dfb_dn0, locals.var_dfb_dn1, locals.var_dfb_dn3, locals.var_dfb_dn4, locals.var_dfb_dn5, locals.var_dfb_dn6, locals.var_dfb_dn7, locals.var_dfb_dn8, locals.var_dfb_dn9,)
    }
};
        locals.var_dfb = assign10350_e12425;
        locals.var_dfb_dn0 = assign10350_e12425_d_n0;
        locals.var_dfb_dn1 = assign10350_e12425_d_n1;
        locals.var_dfb_dn3 = assign10350_e12425_d_n3;
        locals.var_dfb_dn4 = assign10350_e12425_d_n4;
        locals.var_dfb_dn5 = assign10350_e12425_d_n5;
        locals.var_dfb_dn6 = assign10350_e12425_d_n6;
        locals.var_dfb_dn7 = assign10350_e12425_d_n7;
        locals.var_dfb_dn8 = assign10350_e12425_d_n8;
        locals.var_dfb_dn9 = assign10350_e12425_d_n9;
        locals.var_dfb_rv = 0.0;

        let (assign10360_e12440, assign10360_e12440_d_n0, assign10360_e12440_d_n1, assign10360_e12440_d_n3, assign10360_e12440_d_n4, assign10360_e12440_d_n5, assign10360_e12440_d_n6, assign10360_e12440_d_n7, assign10360_e12440_d_n8, assign10360_e12440_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 == 0.0)) && (locals.var_guard213 != 0.0)) {
        let assign10360_e12433: f64 = (-p.p64);
        let assign10360_e12435: f64 = (assign10360_e12433 * locals.var_dfb);
        let assign10360_e12436: f64 = (assign10360_e12435).exp();
        let assign10360_e12438: f64 = (assign10360_e12436 * locals.var_dfdvj_dv);
        (assign10360_e12438, (((assign10360_e12436 * (assign10360_e12433 * locals.var_dfb_dn0)) * locals.var_dfdvj_dv) + (assign10360_e12436 * locals.var_dfdvj_dv_dn0)), (((assign10360_e12436 * (assign10360_e12433 * locals.var_dfb_dn1)) * locals.var_dfdvj_dv) + (assign10360_e12436 * locals.var_dfdvj_dv_dn1)), (((assign10360_e12436 * (assign10360_e12433 * locals.var_dfb_dn3)) * locals.var_dfdvj_dv) + (assign10360_e12436 * locals.var_dfdvj_dv_dn3)), (((assign10360_e12436 * (assign10360_e12433 * locals.var_dfb_dn4)) * locals.var_dfdvj_dv) + (assign10360_e12436 * locals.var_dfdvj_dv_dn4)), (((assign10360_e12436 * (assign10360_e12433 * locals.var_dfb_dn5)) * locals.var_dfdvj_dv) + (assign10360_e12436 * locals.var_dfdvj_dv_dn5)), (((assign10360_e12436 * (assign10360_e12433 * locals.var_dfb_dn6)) * locals.var_dfdvj_dv) + (assign10360_e12436 * locals.var_dfdvj_dv_dn6)), (((assign10360_e12436 * (assign10360_e12433 * locals.var_dfb_dn7)) * locals.var_dfdvj_dv) + (assign10360_e12436 * locals.var_dfdvj_dv_dn7)), (((assign10360_e12436 * (assign10360_e12433 * locals.var_dfb_dn8)) * locals.var_dfdvj_dv) + (assign10360_e12436 * locals.var_dfdvj_dv_dn8)), (((assign10360_e12436 * (assign10360_e12433 * locals.var_dfb_dn9)) * locals.var_dfdvj_dv) + (assign10360_e12436 * locals.var_dfdvj_dv_dn9)),)
    } else {
        (locals.var_dfc_j1, locals.var_dfc_j1_dn0, locals.var_dfc_j1_dn1, locals.var_dfc_j1_dn3, locals.var_dfc_j1_dn4, locals.var_dfc_j1_dn5, locals.var_dfc_j1_dn6, locals.var_dfc_j1_dn7, locals.var_dfc_j1_dn8, locals.var_dfc_j1_dn9,)
    }
};
        locals.var_dfc_j1 = assign10360_e12440;
        locals.var_dfc_j1_dn0 = assign10360_e12440_d_n0;
        locals.var_dfc_j1_dn1 = assign10360_e12440_d_n1;
        locals.var_dfc_j1_dn3 = assign10360_e12440_d_n3;
        locals.var_dfc_j1_dn4 = assign10360_e12440_d_n4;
        locals.var_dfc_j1_dn5 = assign10360_e12440_d_n5;
        locals.var_dfc_j1_dn6 = assign10360_e12440_d_n6;
        locals.var_dfc_j1_dn7 = assign10360_e12440_d_n7;
        locals.var_dfc_j1_dn8 = assign10360_e12440_d_n8;
        locals.var_dfc_j1_dn9 = assign10360_e12440_d_n9;
        locals.var_dfc_j1_rv = 0.0;

        let (assign10380_e12479, assign10380_e12479_d_n0, assign10380_e12479_d_n1, assign10380_e12479_d_n3, assign10380_e12479_d_n4, assign10380_e12479_d_n5, assign10380_e12479_d_n6, assign10380_e12479_d_n7, assign10380_e12479_d_n8, assign10380_e12479_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 == 0.0)) && (locals.var_guard213 != 0.0)) {
        let assign10380_e12469: f64 = (1.0 - p.p64);
        let assign10380_e12470: f64 = (locals.var_dfb * assign10380_e12469);
        let assign10380_e12471: f64 = (assign10380_e12470).exp();
        let assign10380_e12472: f64 = (1.0 - assign10380_e12471);
        let assign10380_e12473: f64 = (locals.var_vdsp_t * assign10380_e12472);
        let assign10380_e12476: f64 = (1.0 - p.p64);
        let assign10380_e12477: f64 = (assign10380_e12473 / assign10380_e12476);
        (assign10380_e12477, ((locals.var_vdsp_t * (-(assign10380_e12471 * (locals.var_dfb_dn0 * assign10380_e12469)))) / assign10380_e12476), ((locals.var_vdsp_t * (-(assign10380_e12471 * (locals.var_dfb_dn1 * assign10380_e12469)))) / assign10380_e12476), ((locals.var_vdsp_t * (-(assign10380_e12471 * (locals.var_dfb_dn3 * assign10380_e12469)))) / assign10380_e12476), (((locals.var_vdsp_t_dn4 * assign10380_e12472) + (locals.var_vdsp_t * (-(assign10380_e12471 * (locals.var_dfb_dn4 * assign10380_e12469))))) / assign10380_e12476), ((locals.var_vdsp_t * (-(assign10380_e12471 * (locals.var_dfb_dn5 * assign10380_e12469)))) / assign10380_e12476), ((locals.var_vdsp_t * (-(assign10380_e12471 * (locals.var_dfb_dn6 * assign10380_e12469)))) / assign10380_e12476), ((locals.var_vdsp_t * (-(assign10380_e12471 * (locals.var_dfb_dn7 * assign10380_e12469)))) / assign10380_e12476), ((locals.var_vdsp_t * (-(assign10380_e12471 * (locals.var_dfb_dn8 * assign10380_e12469)))) / assign10380_e12476), ((locals.var_vdsp_t * (-(assign10380_e12471 * (locals.var_dfb_dn9 * assign10380_e12469)))) / assign10380_e12476),)
    } else {
        (locals.var_dfq_j1, locals.var_dfq_j1_dn0, locals.var_dfq_j1_dn1, locals.var_dfq_j1_dn3, locals.var_dfq_j1_dn4, locals.var_dfq_j1_dn5, locals.var_dfq_j1_dn6, locals.var_dfq_j1_dn7, locals.var_dfq_j1_dn8, locals.var_dfq_j1_dn9,)
    }
};
        locals.var_dfq_j1 = assign10380_e12479;
        locals.var_dfq_j1_dn0 = assign10380_e12479_d_n0;
        locals.var_dfq_j1_dn1 = assign10380_e12479_d_n1;
        locals.var_dfq_j1_dn3 = assign10380_e12479_d_n3;
        locals.var_dfq_j1_dn4 = assign10380_e12479_d_n4;
        locals.var_dfq_j1_dn5 = assign10380_e12479_d_n5;
        locals.var_dfq_j1_dn6 = assign10380_e12479_d_n6;
        locals.var_dfq_j1_dn7 = assign10380_e12479_d_n7;
        locals.var_dfq_j1_dn8 = assign10380_e12479_d_n8;
        locals.var_dfq_j1_dn9 = assign10380_e12479_d_n9;
        locals.var_dfq_j1_rv = 0.0;

        let (assign10390_e12496, assign10390_e12496_d_n0, assign10390_e12496_d_n1, assign10390_e12496_d_n3, assign10390_e12496_d_n4, assign10390_e12496_d_n5, assign10390_e12496_d_n6, assign10390_e12496_d_n7, assign10390_e12496_d_n8, assign10390_e12496_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 == 0.0)) && (locals.var_guard213 != 0.0)) {
        let assign10390_e12491: f64 = (locals.var_vsc - locals.var_dfv_j);
        let assign10390_e12492: f64 = (locals.var_ajsp_t * assign10390_e12491);
        let assign10390_e12493: f64 = (locals.var_dfq_j1 + assign10390_e12492);
        let assign10390_e12494: f64 = (locals.var_cscp0_t * assign10390_e12493);
        (assign10390_e12494, (locals.var_cscp0_t * (locals.var_dfq_j1_dn0 + (locals.var_ajsp_t * (locals.var_vsc_dn0 - locals.var_dfv_j_dn0)))), (locals.var_cscp0_t * (locals.var_dfq_j1_dn1 + (locals.var_ajsp_t * (-locals.var_dfv_j_dn1)))), (locals.var_cscp0_t * (locals.var_dfq_j1_dn3 + (locals.var_ajsp_t * (locals.var_vsc_dn3 - locals.var_dfv_j_dn3)))), ((locals.var_cscp0_t_dn4 * assign10390_e12493) + (locals.var_cscp0_t * (locals.var_dfq_j1_dn4 + ((locals.var_ajsp_t_dn4 * assign10390_e12491) + (locals.var_ajsp_t * (-locals.var_dfv_j_dn4)))))), (locals.var_cscp0_t * (locals.var_dfq_j1_dn5 + (locals.var_ajsp_t * (-locals.var_dfv_j_dn5)))), (locals.var_cscp0_t * (locals.var_dfq_j1_dn6 + (locals.var_ajsp_t * (-locals.var_dfv_j_dn6)))), (locals.var_cscp0_t * (locals.var_dfq_j1_dn7 + (locals.var_ajsp_t * (-locals.var_dfv_j_dn7)))), (locals.var_cscp0_t * (locals.var_dfq_j1_dn8 + (locals.var_ajsp_t * (-locals.var_dfv_j_dn8)))), (locals.var_cscp0_t * (locals.var_dfq_j1_dn9 + (locals.var_ajsp_t * (-locals.var_dfv_j_dn9)))),)
    } else {
        (locals.var_qscp, locals.var_qscp_dn0, locals.var_qscp_dn1, locals.var_qscp_dn3, locals.var_qscp_dn4, locals.var_qscp_dn5, locals.var_qscp_dn6, locals.var_qscp_dn7, locals.var_qscp_dn8, locals.var_qscp_dn9,)
    }
};
        locals.var_qscp = assign10390_e12496;
        locals.var_qscp_dn0 = assign10390_e12496_d_n0;
        locals.var_qscp_dn1 = assign10390_e12496_d_n1;
        locals.var_qscp_dn3 = assign10390_e12496_d_n3;
        locals.var_qscp_dn4 = assign10390_e12496_d_n4;
        locals.var_qscp_dn5 = assign10390_e12496_d_n5;
        locals.var_qscp_dn6 = assign10390_e12496_d_n6;
        locals.var_qscp_dn7 = assign10390_e12496_d_n7;
        locals.var_qscp_dn8 = assign10390_e12496_d_n8;
        locals.var_qscp_dn9 = assign10390_e12496_d_n9;
        locals.var_qscp_rv = 0.0;

        let (assign10410_e12516, assign10410_e12516_d_n0, assign10410_e12516_d_n1, assign10410_e12516_d_n3, assign10410_e12516_d_n4, assign10410_e12516_d_n5, assign10410_e12516_d_n6, assign10410_e12516_d_n7, assign10410_e12516_d_n8, assign10410_e12516_d_n9,) = {
    if (((locals.var_guard208 != 0.0) && (locals.var_guard209 == 0.0)) && (locals.var_guard213 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qscp, locals.var_qscp_dn0, locals.var_qscp_dn1, locals.var_qscp_dn3, locals.var_qscp_dn4, locals.var_qscp_dn5, locals.var_qscp_dn6, locals.var_qscp_dn7, locals.var_qscp_dn8, locals.var_qscp_dn9,)
    }
};
        locals.var_qscp = assign10410_e12516;
        locals.var_qscp_dn0 = assign10410_e12516_d_n0;
        locals.var_qscp_dn1 = assign10410_e12516_d_n1;
        locals.var_qscp_dn3 = assign10410_e12516_d_n3;
        locals.var_qscp_dn4 = assign10410_e12516_d_n4;
        locals.var_qscp_dn5 = assign10410_e12516_d_n5;
        locals.var_qscp_dn6 = assign10410_e12516_d_n6;
        locals.var_qscp_dn7 = assign10410_e12516_d_n7;
        locals.var_qscp_dn8 = assign10410_e12516_d_n8;
        locals.var_qscp_dn9 = assign10410_e12516_d_n9;
        locals.var_qscp_rv = 0.0;

        let (assign10430_e12528, assign10430_e12528_d_n0, assign10430_e12528_d_n1, assign10430_e12528_d_n3, assign10430_e12528_d_n4, assign10430_e12528_d_n5, assign10430_e12528_d_n6, assign10430_e12528_d_n7, assign10430_e12528_d_n8, assign10430_e12528_d_n9,) = {
    if (locals.var_guard208 == 0.0) {
        let assign10430_e12526: f64 = (p.p62 * locals.var_vsc);
        (assign10430_e12526, (p.p62 * locals.var_vsc_dn0), 0.0, (p.p62 * locals.var_vsc_dn3), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qscp, locals.var_qscp_dn0, locals.var_qscp_dn1, locals.var_qscp_dn3, locals.var_qscp_dn4, locals.var_qscp_dn5, locals.var_qscp_dn6, locals.var_qscp_dn7, locals.var_qscp_dn8, locals.var_qscp_dn9,)
    }
};
        locals.var_qscp = assign10430_e12528;
        locals.var_qscp_dn0 = assign10430_e12528_d_n0;
        locals.var_qscp_dn1 = assign10430_e12528_d_n1;
        locals.var_qscp_dn3 = assign10430_e12528_d_n3;
        locals.var_qscp_dn4 = assign10430_e12528_d_n4;
        locals.var_qscp_dn5 = assign10430_e12528_d_n5;
        locals.var_qscp_dn6 = assign10430_e12528_d_n6;
        locals.var_qscp_dn7 = assign10430_e12528_d_n7;
        locals.var_qscp_dn8 = assign10430_e12528_d_n8;
        locals.var_qscp_dn9 = assign10430_e12528_d_n9;
        locals.var_qscp_rv = 0.0;

        let assign10440_e12531: f64 = if p.p97 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard214 = assign10440_e12531;
        locals.var_guard214_rv = 0.0;

        let (assign10450_e12537, assign10450_e12537_d_n4,) = {
    if (locals.var_guard214 != 0.0) {
        let assign10450_e12535: f64 = (p.p98 * locals.var_vt);
        (assign10450_e12535, (p.p98 * locals.var_vt_dn4),)
    } else {
        (locals.var_msfvt, locals.var_msfvt_dn4,)
    }
};
        locals.var_msfvt = assign10450_e12537;
        locals.var_msfvt_dn4 = assign10450_e12537_d_n4;
        locals.var_msfvt_rv = 0.0;

        let (assign10460_e12544, assign10460_e12544_d_n4, assign10460_e12544_d_n5, assign10460_e12544_d_n7,) = {
    if (locals.var_guard214 != 0.0) {
        let assign10460_e12541: f64 = (locals.var_vbpci / locals.var_msfvt);
        let assign10460_e12542: f64 = { let limexp_arg = assign10460_e12541; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (assign10460_e12542, ({ let limexp_arg = assign10460_e12541; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-((locals.var_vbpci * locals.var_msfvt_dn4) / (locals.var_msfvt * locals.var_msfvt)))), ({ let limexp_arg = assign10460_e12541; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (locals.var_vbpci_dn5 / locals.var_msfvt)), ({ let limexp_arg = assign10460_e12541; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (locals.var_vbpci_dn7 / locals.var_msfvt)),)
    } else {
        (locals.var_its_f, locals.var_its_f_dn4, locals.var_its_f_dn5, locals.var_its_f_dn7,)
    }
};
        locals.var_its_f = assign10460_e12544;
        locals.var_its_f_dn4 = assign10460_e12544_d_n4;
        locals.var_its_f_dn5 = assign10460_e12544_d_n5;
        locals.var_its_f_dn7 = assign10460_e12544_d_n7;
        locals.var_its_f_rv = 0.0;

        let assign10490_e12562: f64 = if p.p101 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard218 = assign10490_e12562;
        locals.var_guard218_rv = 0.0;

        let (assign10500_e12572, assign10500_e12572_d_n4, assign10500_e12572_d_n5, assign10500_e12572_d_n7,) = {
    if ((locals.var_guard214 != 0.0) && (locals.var_guard218 != 0.0)) {
        let assign10500_e12568: f64 = (locals.var_tsf_t * locals.var_itss_t);
        let assign10500_e12570: f64 = (assign10500_e12568 * locals.var_its_f);
        (assign10500_e12570, ((((locals.var_tsf_t_dn4 * locals.var_itss_t) + (locals.var_tsf_t * locals.var_itss_t_dn4)) * locals.var_its_f) + (assign10500_e12568 * locals.var_its_f_dn4)), (assign10500_e12568 * locals.var_its_f_dn5), (assign10500_e12568 * locals.var_its_f_dn7),)
    } else {
        (locals.var_qdsu, locals.var_qdsu_dn4, locals.var_qdsu_dn5, locals.var_qdsu_dn7,)
    }
};
        locals.var_qdsu = assign10500_e12572;
        locals.var_qdsu_dn4 = assign10500_e12572_d_n4;
        locals.var_qdsu_dn5 = assign10500_e12572_d_n5;
        locals.var_qdsu_dn7 = assign10500_e12572_d_n7;
        locals.var_qdsu_rv = 0.0;

        let (assign10510_e12579, assign10510_e12579_d_n4, assign10510_e12579_d_n5, assign10510_e12579_d_n7,) = {
    if ((locals.var_guard214 != 0.0) && (locals.var_guard218 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdsu, locals.var_qdsu_dn4, locals.var_qdsu_dn5, locals.var_qdsu_dn7,)
    }
};
        locals.var_qdsu = assign10510_e12579;
        locals.var_qdsu_dn4 = assign10510_e12579_d_n4;
        locals.var_qdsu_dn5 = assign10510_e12579_d_n5;
        locals.var_qdsu_dn7 = assign10510_e12579_d_n7;
        locals.var_qdsu_rv = 0.0;

        let (assign10530_e12589, assign10530_e12589_d_n4, assign10530_e12589_d_n5, assign10530_e12589_d_n7,) = {
    if (locals.var_guard214 == 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdsu, locals.var_qdsu_dn4, locals.var_qdsu_dn5, locals.var_qdsu_dn7,)
    }
};
        locals.var_qdsu = assign10530_e12589;
        locals.var_qdsu_dn4 = assign10530_e12589_d_n4;
        locals.var_qdsu_dn5 = assign10530_e12589_d_n5;
        locals.var_qdsu_dn7 = assign10530_e12589_d_n7;
        locals.var_qdsu_rv = 0.0;

        let assign10540_e12592: f64 = if p.p99 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard219 = assign10540_e12592;
        locals.var_guard219_rv = 0.0;

        let (assign10550_e12600, assign10550_e12600_d_n4, assign10550_e12600_d_n5, assign10550_e12600_d_n6, assign10550_e12600_d_n7, assign10550_e12600_d_n8, assign10550_e12600_d_n9,) = {
    if (locals.var_guard219 != 0.0) {
        let assign10550_e12597: f64 = (p.p100 * locals.var_vt);
        let assign10550_e12598: f64 = (locals.var_vsici / assign10550_e12597);
        (assign10550_e12598, (-((locals.var_vsici * (p.p100 * locals.var_vt_dn4)) / (assign10550_e12597 * assign10550_e12597))), (locals.var_vsici_dn5 / assign10550_e12597), 0.0, 0.0, 0.0, (locals.var_vsici_dn9 / assign10550_e12597),)
    } else {
        (locals.var_dio_y, locals.var_dio_y_dn4, locals.var_dio_y_dn5, locals.var_dio_y_dn6, locals.var_dio_y_dn7, locals.var_dio_y_dn8, locals.var_dio_y_dn9,)
    }
};
        locals.var_dio_y = assign10550_e12600;
        locals.var_dio_y_dn4 = assign10550_e12600_d_n4;
        locals.var_dio_y_dn5 = assign10550_e12600_d_n5;
        locals.var_dio_y_dn6 = assign10550_e12600_d_n6;
        locals.var_dio_y_dn7 = assign10550_e12600_d_n7;
        locals.var_dio_y_dn8 = assign10550_e12600_d_n8;
        locals.var_dio_y_dn9 = assign10550_e12600_d_n9;
        locals.var_dio_y_rv = 0.0;

        let assign10560_e12603: f64 = if locals.var_dio_y > 80.0 { 1.0 } else { 0.0 };
        locals.var_guard220 = assign10560_e12603;
        locals.var_guard220_rv = 0.0;

        let (assign10570_e12613, assign10570_e12613_d_n4, assign10570_e12613_d_n5, assign10570_e12613_d_n6, assign10570_e12613_d_n7, assign10570_e12613_d_n8, assign10570_e12613_d_n9,) = {
    if ((locals.var_guard219 != 0.0) && (locals.var_guard220 != 0.0)) {
        let assign10570_e12610: f64 = (locals.var_dio_y - 80.0);
        let assign10570_e12611: f64 = (1.0 + assign10570_e12610);
        (assign10570_e12611, locals.var_dio_y_dn4, locals.var_dio_y_dn5, locals.var_dio_y_dn6, locals.var_dio_y_dn7, locals.var_dio_y_dn8, locals.var_dio_y_dn9,)
    } else {
        (locals.var_dio_le, locals.var_dio_le_dn4, locals.var_dio_le_dn5, locals.var_dio_le_dn6, locals.var_dio_le_dn7, locals.var_dio_le_dn8, locals.var_dio_le_dn9,)
    }
};
        locals.var_dio_le = assign10570_e12613;
        locals.var_dio_le_dn4 = assign10570_e12613_d_n4;
        locals.var_dio_le_dn5 = assign10570_e12613_d_n5;
        locals.var_dio_le_dn6 = assign10570_e12613_d_n6;
        locals.var_dio_le_dn7 = assign10570_e12613_d_n7;
        locals.var_dio_le_dn8 = assign10570_e12613_d_n8;
        locals.var_dio_le_dn9 = assign10570_e12613_d_n9;
        locals.var_dio_le_rv = 0.0;

        let (assign10580_e12619, assign10580_e12619_d_n4, assign10580_e12619_d_n5, assign10580_e12619_d_n6, assign10580_e12619_d_n7, assign10580_e12619_d_n8, assign10580_e12619_d_n9,) = {
    if ((locals.var_guard219 != 0.0) && (locals.var_guard220 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dio_y, locals.var_dio_y_dn4, locals.var_dio_y_dn5, locals.var_dio_y_dn6, locals.var_dio_y_dn7, locals.var_dio_y_dn8, locals.var_dio_y_dn9,)
    }
};
        locals.var_dio_y = assign10580_e12619;
        locals.var_dio_y_dn4 = assign10580_e12619_d_n4;
        locals.var_dio_y_dn5 = assign10580_e12619_d_n5;
        locals.var_dio_y_dn6 = assign10580_e12619_d_n6;
        locals.var_dio_y_dn7 = assign10580_e12619_d_n7;
        locals.var_dio_y_dn8 = assign10580_e12619_d_n8;
        locals.var_dio_y_dn9 = assign10580_e12619_d_n9;
        locals.var_dio_y_rv = 0.0;

        let (assign10590_e12626, assign10590_e12626_d_n4, assign10590_e12626_d_n5, assign10590_e12626_d_n6, assign10590_e12626_d_n7, assign10590_e12626_d_n8, assign10590_e12626_d_n9,) = {
    if ((locals.var_guard219 != 0.0) && (locals.var_guard220 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dio_le, locals.var_dio_le_dn4, locals.var_dio_le_dn5, locals.var_dio_le_dn6, locals.var_dio_le_dn7, locals.var_dio_le_dn8, locals.var_dio_le_dn9,)
    }
};
        locals.var_dio_le = assign10590_e12626;
        locals.var_dio_le_dn4 = assign10590_e12626_d_n4;
        locals.var_dio_le_dn5 = assign10590_e12626_d_n5;
        locals.var_dio_le_dn6 = assign10590_e12626_d_n6;
        locals.var_dio_le_dn7 = assign10590_e12626_d_n7;
        locals.var_dio_le_dn8 = assign10590_e12626_d_n8;
        locals.var_dio_le_dn9 = assign10590_e12626_d_n9;
        locals.var_dio_le_rv = 0.0;

        locals.var_qdeix = locals.var_qdei;
        locals.var_qdeix_dn0 = locals.var_qdei_dn0;
        locals.var_qdeix_dn1 = locals.var_qdei_dn1;
        locals.var_qdeix_dn3 = locals.var_qdei_dn3;
        locals.var_qdeix_dn4 = locals.var_qdei_dn4;
        locals.var_qdeix_dn5 = locals.var_qdei_dn5;
        locals.var_qdeix_dn6 = locals.var_qdei_dn6;
        locals.var_qdeix_dn7 = locals.var_qdei_dn7;
        locals.var_qdeix_dn8 = locals.var_qdei_dn8;
        locals.var_qdeix_dn9 = locals.var_qdei_dn9;
        locals.var_qdeix_dn12 = 0.0;
        locals.var_qdeix_rv = 0.0;

        let assign10780_e12817: f64 = if locals.var_use_nqs != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard232 = assign10780_e12817;
        locals.var_guard232_rv = 0.0;

        let (assign10860_e12871, assign10860_e12871_d_n12,) = {
    if (locals.var_guard232 != 0.0) {
        ((nv12 - 0.0), 1.0,)
    } else {
        (locals.var_vxf, locals.var_vxf_dn12,)
    }
};
        locals.var_vxf = assign10860_e12871;
        locals.var_vxf_dn12 = assign10860_e12871_d_n12;
        locals.var_vxf_rv = 0.0;

        let (assign10900_e12897, assign10900_e12897_d_n0, assign10900_e12897_d_n1, assign10900_e12897_d_n3, assign10900_e12897_d_n4, assign10900_e12897_d_n5, assign10900_e12897_d_n6, assign10900_e12897_d_n7, assign10900_e12897_d_n8, assign10900_e12897_d_n9, assign10900_e12897_d_n12,) = {
    if (locals.var_guard232 != 0.0) {
        (locals.var_vxf, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, locals.var_vxf_dn12,)
    } else {
        (locals.var_qdeix, locals.var_qdeix_dn0, locals.var_qdeix_dn1, locals.var_qdeix_dn3, locals.var_qdeix_dn4, locals.var_qdeix_dn5, locals.var_qdeix_dn6, locals.var_qdeix_dn7, locals.var_qdeix_dn8, locals.var_qdeix_dn9, locals.var_qdeix_dn12,)
    }
};
        locals.var_qdeix = assign10900_e12897;
        locals.var_qdeix_dn0 = assign10900_e12897_d_n0;
        locals.var_qdeix_dn1 = assign10900_e12897_d_n1;
        locals.var_qdeix_dn3 = assign10900_e12897_d_n3;
        locals.var_qdeix_dn4 = assign10900_e12897_d_n4;
        locals.var_qdeix_dn5 = assign10900_e12897_d_n5;
        locals.var_qdeix_dn6 = assign10900_e12897_d_n6;
        locals.var_qdeix_dn7 = assign10900_e12897_d_n7;
        locals.var_qdeix_dn8 = assign10900_e12897_d_n8;
        locals.var_qdeix_dn9 = assign10900_e12897_d_n9;
        locals.var_qdeix_dn12 = assign10900_e12897_d_n12;
        locals.var_qdeix_rv = 0.0;

        let assign10970_e12934: f64 = if ((p.p89 >= p.p149) && (p.p89 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard233 = assign10970_e12934;
        locals.var_guard233_rv = 0.0;

        let assign10980_e12937: f64 = if p.p93 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard234 = assign10980_e12937;
        locals.var_guard234_rv = 0.0;

        let assign11060_e12977: f64 = if ((p.p102 >= p.p149) && (p.p102 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard242 = assign11060_e12977;
        locals.var_guard242_rv = 0.0;

        let assign11070_e12980: f64 = if p.p103 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard243 = assign11070_e12980;
        locals.var_guard243_rv = 0.0;

        let assign11080_e12991: f64 = if (((p.p141 >= 1.0) && (p.p142 >= p.p149)) && (p.p142 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard244 = assign11080_e12991;
        locals.var_guard244_rv = 0.0;

        let assign11090_e12994: f64 = if p.p145 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard245 = assign11090_e12994;
        locals.var_guard245_rv = 0.0;

        let assign11230_e13085: f64 = if ((p.p109 == 1.0) && ((p.p88 > 0.0) && (p.p87 > 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard258 = assign11230_e13085;
        locals.var_guard258_rv = 0.0;

        let assign11240_e13088: f64 = if locals.var_ibei > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard264 = assign11240_e13088;
        locals.var_guard264_rv = 0.0;

        let (assign11250_e13096, assign11250_e13096_d_n0, assign11250_e13096_d_n1, assign11250_e13096_d_n3, assign11250_e13096_d_n4, assign11250_e13096_d_n5, assign11250_e13096_d_n6, assign11250_e13096_d_n7, assign11250_e13096_d_n8, assign11250_e13096_d_n9,) = {
    if ((locals.var_guard258 != 0.0) && (locals.var_guard264 != 0.0)) {
        let assign11250_e13094: f64 = (locals.var_it / locals.var_ibei);
        (assign11250_e13094, (locals.var_it_dn0 / locals.var_ibei), (locals.var_it_dn1 / locals.var_ibei), (locals.var_it_dn3 / locals.var_ibei), (((locals.var_it_dn4 * locals.var_ibei) - (locals.var_it * locals.var_ibei_dn4)) / (locals.var_ibei * locals.var_ibei)), (((locals.var_it_dn5 * locals.var_ibei) - (locals.var_it * locals.var_ibei_dn5)) / (locals.var_ibei * locals.var_ibei)), (((locals.var_it_dn6 * locals.var_ibei) - (locals.var_it * locals.var_ibei_dn6)) / (locals.var_ibei * locals.var_ibei)), (((locals.var_it_dn7 * locals.var_ibei) - (locals.var_it * locals.var_ibei_dn7)) / (locals.var_ibei * locals.var_ibei)), (((locals.var_it_dn8 * locals.var_ibei) - (locals.var_it * locals.var_ibei_dn8)) / (locals.var_ibei * locals.var_ibei)), (((locals.var_it_dn9 * locals.var_ibei) - (locals.var_it * locals.var_ibei_dn9)) / (locals.var_ibei * locals.var_ibei)),)
    } else {
        (locals.var_betadc_1, locals.var_betadc_1_dn0, locals.var_betadc_1_dn1, locals.var_betadc_1_dn3, locals.var_betadc_1_dn4, locals.var_betadc_1_dn5, locals.var_betadc_1_dn6, locals.var_betadc_1_dn7, locals.var_betadc_1_dn8, locals.var_betadc_1_dn9,)
    }
};
        locals.var_betadc_1 = assign11250_e13096;
        locals.var_betadc_1_dn0 = assign11250_e13096_d_n0;
        locals.var_betadc_1_dn1 = assign11250_e13096_d_n1;
        locals.var_betadc_1_dn3 = assign11250_e13096_d_n3;
        locals.var_betadc_1_dn4 = assign11250_e13096_d_n4;
        locals.var_betadc_1_dn5 = assign11250_e13096_d_n5;
        locals.var_betadc_1_dn6 = assign11250_e13096_d_n6;
        locals.var_betadc_1_dn7 = assign11250_e13096_d_n7;
        locals.var_betadc_1_dn8 = assign11250_e13096_d_n8;
        locals.var_betadc_1_dn9 = assign11250_e13096_d_n9;
        locals.var_betadc_1_rv = 0.0;

        let (assign11260_e13103, assign11260_e13103_d_n0, assign11260_e13103_d_n1, assign11260_e13103_d_n3, assign11260_e13103_d_n4, assign11260_e13103_d_n5, assign11260_e13103_d_n6, assign11260_e13103_d_n7, assign11260_e13103_d_n8, assign11260_e13103_d_n9,) = {
    if ((locals.var_guard258 != 0.0) && (locals.var_guard264 == 0.0)) {
        (1000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_betadc_1, locals.var_betadc_1_dn0, locals.var_betadc_1_dn1, locals.var_betadc_1_dn3, locals.var_betadc_1_dn4, locals.var_betadc_1_dn5, locals.var_betadc_1_dn6, locals.var_betadc_1_dn7, locals.var_betadc_1_dn8, locals.var_betadc_1_dn9,)
    }
};
        locals.var_betadc_1 = assign11260_e13103;
        locals.var_betadc_1_dn0 = assign11260_e13103_d_n0;
        locals.var_betadc_1_dn1 = assign11260_e13103_d_n1;
        locals.var_betadc_1_dn3 = assign11260_e13103_d_n3;
        locals.var_betadc_1_dn4 = assign11260_e13103_d_n4;
        locals.var_betadc_1_dn5 = assign11260_e13103_d_n5;
        locals.var_betadc_1_dn6 = assign11260_e13103_d_n6;
        locals.var_betadc_1_dn7 = assign11260_e13103_d_n7;
        locals.var_betadc_1_dn8 = assign11260_e13103_d_n8;
        locals.var_betadc_1_dn9 = assign11260_e13103_d_n9;
        locals.var_betadc_1_rv = 0.0;

        let (assign11270_e13107,) = {
    if (locals.var_guard258 != 0.0) {
        (1.0,)
    } else {
        (locals.var_n_w,)
    }
};
        locals.var_n_w = assign11270_e13107;
        locals.var_n_w_rv = 0.0;

        let (assign11280_e13113, assign11280_e13113_d_n0, assign11280_e13113_d_n1, assign11280_e13113_d_n3, assign11280_e13113_d_n4, assign11280_e13113_d_n5, assign11280_e13113_d_n6, assign11280_e13113_d_n7, assign11280_e13113_d_n8, assign11280_e13113_d_n9,) = {
    if (locals.var_guard258 != 0.0) {
        let assign11280_e13111: f64 = (locals.var_tf * p.p88);
        (assign11280_e13111, (locals.var_tf_dn0 * p.p88), (locals.var_tf_dn1 * p.p88), (locals.var_tf_dn3 * p.p88), (locals.var_tf_dn4 * p.p88), (locals.var_tf_dn5 * p.p88), (locals.var_tf_dn6 * p.p88), (locals.var_tf_dn7 * p.p88), (locals.var_tf_dn8 * p.p88), (locals.var_tf_dn9 * p.p88),)
    } else {
        (locals.var_n_1, locals.var_n_1_dn0, locals.var_n_1_dn1, locals.var_n_1_dn3, locals.var_n_1_dn4, locals.var_n_1_dn5, locals.var_n_1_dn6, locals.var_n_1_dn7, locals.var_n_1_dn8, locals.var_n_1_dn9,)
    }
};
        locals.var_n_1 = assign11280_e13113;
        locals.var_n_1_dn0 = assign11280_e13113_d_n0;
        locals.var_n_1_dn1 = assign11280_e13113_d_n1;
        locals.var_n_1_dn3 = assign11280_e13113_d_n3;
        locals.var_n_1_dn4 = assign11280_e13113_d_n4;
        locals.var_n_1_dn5 = assign11280_e13113_d_n5;
        locals.var_n_1_dn6 = assign11280_e13113_d_n6;
        locals.var_n_1_dn7 = assign11280_e13113_d_n7;
        locals.var_n_1_dn8 = assign11280_e13113_d_n8;
        locals.var_n_1_dn9 = assign11280_e13113_d_n9;
        locals.var_n_1_rv = 0.0;

        let (assign11290_e13125, assign11290_e13125_d_n0, assign11290_e13125_d_n1, assign11290_e13125_d_n3, assign11290_e13125_d_n4, assign11290_e13125_d_n5, assign11290_e13125_d_n6, assign11290_e13125_d_n7, assign11290_e13125_d_n8, assign11290_e13125_d_n9,) = {
    if (locals.var_guard258 != 0.0) {
        let assign11290_e13118: f64 = (2.0 * p.p87);
        let assign11290_e13121: f64 = (p.p88 * p.p88);
        let assign11290_e13122: f64 = (assign11290_e13118 - assign11290_e13121);
        let assign11290_e13123: f64 = (locals.var_betadc_1 * assign11290_e13122);
        (assign11290_e13123, (locals.var_betadc_1_dn0 * assign11290_e13122), (locals.var_betadc_1_dn1 * assign11290_e13122), (locals.var_betadc_1_dn3 * assign11290_e13122), (locals.var_betadc_1_dn4 * assign11290_e13122), (locals.var_betadc_1_dn5 * assign11290_e13122), (locals.var_betadc_1_dn6 * assign11290_e13122), (locals.var_betadc_1_dn7 * assign11290_e13122), (locals.var_betadc_1_dn8 * assign11290_e13122), (locals.var_betadc_1_dn9 * assign11290_e13122),)
    } else {
        (locals.var_sqrt_n2, locals.var_sqrt_n2_dn0, locals.var_sqrt_n2_dn1, locals.var_sqrt_n2_dn3, locals.var_sqrt_n2_dn4, locals.var_sqrt_n2_dn5, locals.var_sqrt_n2_dn6, locals.var_sqrt_n2_dn7, locals.var_sqrt_n2_dn8, locals.var_sqrt_n2_dn9,)
    }
};
        locals.var_sqrt_n2 = assign11290_e13125;
        locals.var_sqrt_n2_dn0 = assign11290_e13125_d_n0;
        locals.var_sqrt_n2_dn1 = assign11290_e13125_d_n1;
        locals.var_sqrt_n2_dn3 = assign11290_e13125_d_n3;
        locals.var_sqrt_n2_dn4 = assign11290_e13125_d_n4;
        locals.var_sqrt_n2_dn5 = assign11290_e13125_d_n5;
        locals.var_sqrt_n2_dn6 = assign11290_e13125_d_n6;
        locals.var_sqrt_n2_dn7 = assign11290_e13125_d_n7;
        locals.var_sqrt_n2_dn8 = assign11290_e13125_d_n8;
        locals.var_sqrt_n2_dn9 = assign11290_e13125_d_n9;
        locals.var_sqrt_n2_rv = 0.0;

        let assign11300_e13128: f64 = if locals.var_sqrt_n2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard265 = assign11300_e13128;
        locals.var_guard265_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_29(
        locals: &mut StampLocals,
    ) {
        let (assign11310_e13137, assign11310_e13137_d_n0, assign11310_e13137_d_n1, assign11310_e13137_d_n3, assign11310_e13137_d_n4, assign11310_e13137_d_n5, assign11310_e13137_d_n6, assign11310_e13137_d_n7, assign11310_e13137_d_n8, assign11310_e13137_d_n9,) = {
    if ((locals.var_guard258 != 0.0) && (locals.var_guard265 != 0.0)) {
        let assign11310_e13134: f64 = (locals.var_sqrt_n2).sqrt();
        let assign11310_e13135: f64 = (locals.var_tf * assign11310_e13134);
        (assign11310_e13135, ((locals.var_tf_dn0 * assign11310_e13134) + (locals.var_tf * (locals.var_sqrt_n2_dn0 / (2.0 * assign11310_e13134)))), ((locals.var_tf_dn1 * assign11310_e13134) + (locals.var_tf * (locals.var_sqrt_n2_dn1 / (2.0 * assign11310_e13134)))), ((locals.var_tf_dn3 * assign11310_e13134) + (locals.var_tf * (locals.var_sqrt_n2_dn3 / (2.0 * assign11310_e13134)))), ((locals.var_tf_dn4 * assign11310_e13134) + (locals.var_tf * (locals.var_sqrt_n2_dn4 / (2.0 * assign11310_e13134)))), ((locals.var_tf_dn5 * assign11310_e13134) + (locals.var_tf * (locals.var_sqrt_n2_dn5 / (2.0 * assign11310_e13134)))), ((locals.var_tf_dn6 * assign11310_e13134) + (locals.var_tf * (locals.var_sqrt_n2_dn6 / (2.0 * assign11310_e13134)))), ((locals.var_tf_dn7 * assign11310_e13134) + (locals.var_tf * (locals.var_sqrt_n2_dn7 / (2.0 * assign11310_e13134)))), ((locals.var_tf_dn8 * assign11310_e13134) + (locals.var_tf * (locals.var_sqrt_n2_dn8 / (2.0 * assign11310_e13134)))), ((locals.var_tf_dn9 * assign11310_e13134) + (locals.var_tf * (locals.var_sqrt_n2_dn9 / (2.0 * assign11310_e13134)))),)
    } else {
        (locals.var_n_2, locals.var_n_2_dn0, locals.var_n_2_dn1, locals.var_n_2_dn3, locals.var_n_2_dn4, locals.var_n_2_dn5, locals.var_n_2_dn6, locals.var_n_2_dn7, locals.var_n_2_dn8, locals.var_n_2_dn9,)
    }
};
        locals.var_n_2 = assign11310_e13137;
        locals.var_n_2_dn0 = assign11310_e13137_d_n0;
        locals.var_n_2_dn1 = assign11310_e13137_d_n1;
        locals.var_n_2_dn3 = assign11310_e13137_d_n3;
        locals.var_n_2_dn4 = assign11310_e13137_d_n4;
        locals.var_n_2_dn5 = assign11310_e13137_d_n5;
        locals.var_n_2_dn6 = assign11310_e13137_d_n6;
        locals.var_n_2_dn7 = assign11310_e13137_d_n7;
        locals.var_n_2_dn8 = assign11310_e13137_d_n8;
        locals.var_n_2_dn9 = assign11310_e13137_d_n9;
        locals.var_n_2_rv = 0.0;

        let (assign11320_e13144, assign11320_e13144_d_n0, assign11320_e13144_d_n1, assign11320_e13144_d_n3, assign11320_e13144_d_n4, assign11320_e13144_d_n5, assign11320_e13144_d_n6, assign11320_e13144_d_n7, assign11320_e13144_d_n8, assign11320_e13144_d_n9,) = {
    if ((locals.var_guard258 != 0.0) && (locals.var_guard265 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_n_2, locals.var_n_2_dn0, locals.var_n_2_dn1, locals.var_n_2_dn3, locals.var_n_2_dn4, locals.var_n_2_dn5, locals.var_n_2_dn6, locals.var_n_2_dn7, locals.var_n_2_dn8, locals.var_n_2_dn9,)
    }
};
        locals.var_n_2 = assign11320_e13144;
        locals.var_n_2_dn0 = assign11320_e13144_d_n0;
        locals.var_n_2_dn1 = assign11320_e13144_d_n1;
        locals.var_n_2_dn3 = assign11320_e13144_d_n3;
        locals.var_n_2_dn4 = assign11320_e13144_d_n4;
        locals.var_n_2_dn5 = assign11320_e13144_d_n5;
        locals.var_n_2_dn6 = assign11320_e13144_d_n6;
        locals.var_n_2_dn7 = assign11320_e13144_d_n7;
        locals.var_n_2_dn8 = assign11320_e13144_d_n8;
        locals.var_n_2_dn9 = assign11320_e13144_d_n9;
        locals.var_n_2_rv = 0.0;

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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let eq0_e157: f64 = (locals.var_ibei + locals.var_irei);
        let eq0_e157_d_n4: f64 = (locals.var_ibei_dn4 + locals.var_irei_dn4);
        let eq0_e157_d_n5: f64 = (locals.var_ibei_dn5 + locals.var_irei_dn5);
        let eq0_e157_d_n6: f64 = (locals.var_ibei_dn6 + locals.var_irei_dn6);
        let eq0_e157_d_n7: f64 = (locals.var_ibei_dn7 + locals.var_irei_dn7);
        let eq0_e157_d_n8: f64 = (locals.var_ibei_dn8 + locals.var_irei_dn8);
        let eq0_e157_d_n9: f64 = (locals.var_ibei_dn9 + locals.var_irei_dn9);
        let eq0_e159: f64 = (eq0_e157 + locals.var_ibetat);
        let eq0_e159_d_n4: f64 = (eq0_e157_d_n4 + locals.var_ibetat_dn4);
        let eq0_e159_d_n6: f64 = (eq0_e157_d_n6 + locals.var_ibetat_dn6);
        let eq0_e159_d_n8: f64 = (eq0_e157_d_n8 + locals.var_ibetat_dn8);
        let eq0_e161: f64 = (eq0_e159 + locals.var_ibh_rec);
        let eq0_e161_d_n4: f64 = (eq0_e159_d_n4 + locals.var_ibh_rec_dn4);
        let eq0_e161_d_n5: f64 = (eq0_e157_d_n5 + locals.var_ibh_rec_dn5);
        let eq0_e161_d_n6: f64 = (eq0_e159_d_n6 + locals.var_ibh_rec_dn6);
        let eq0_e161_d_n7: f64 = (eq0_e157_d_n7 + locals.var_ibh_rec_dn7);
        let eq0_e161_d_n8: f64 = (eq0_e159_d_n8 + locals.var_ibh_rec_dn8);
        let eq0_e161_d_n9: f64 = (eq0_e157_d_n9 + locals.var_ibh_rec_dn9);
        let eq0_e162: f64 = (p.p148 * eq0_e161);
        let eq0_e162_d_n0: f64 = (p.p148 * locals.var_ibh_rec_dn0);
        let eq0_e162_d_n1: f64 = (p.p148 * locals.var_ibh_rec_dn1);
        let eq0_e162_d_n3: f64 = (p.p148 * locals.var_ibh_rec_dn3);
        let eq0_e162_d_n4: f64 = (p.p148 * eq0_e161_d_n4);
        let eq0_e162_d_n5: f64 = (p.p148 * eq0_e161_d_n5);
        let eq0_e162_d_n6: f64 = (p.p148 * eq0_e161_d_n6);
        let eq0_e162_d_n7: f64 = (p.p148 * eq0_e161_d_n7);
        let eq0_e162_d_n8: f64 = (p.p148 * eq0_e161_d_n8);
        let eq0_e162_d_n9: f64 = (p.p148 * eq0_e161_d_n9);
        let eq0_e165: f64 = (locals.var_gmin * (nv8 - nv6));
        let eq0_e166: f64 = (eq0_e162 + eq0_e165);
        let eq0_e166_d_n6: f64 = (eq0_e162_d_n6 + (-locals.var_gmin));
        let eq0_e166_d_n8: f64 = (eq0_e162_d_n8 + locals.var_gmin);
        let eq0_value: f64 = eq0_e166;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq0_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq0_e162_d_n0), multiplicity * (eq0_e162_d_n1), multiplicity * (eq0_e162_d_n3), multiplicity * (eq0_e162_d_n4), multiplicity * (eq0_e162_d_n5), multiplicity * (eq0_e166_d_n6), multiplicity * (eq0_e162_d_n7), multiplicity * (eq0_e166_d_n8), multiplicity * (eq0_e162_d_n9)],
            [],
            [],
            1.0,
        );
        let eq1_e170: f64 = (locals.var_qdeix + locals.var_qjei);
        let eq1_e170_d_n0: f64 = (locals.var_qdeix_dn0 + locals.var_qjei_dn0);
        let eq1_e170_d_n1: f64 = (locals.var_qdeix_dn1 + locals.var_qjei_dn1);
        let eq1_e170_d_n3: f64 = (locals.var_qdeix_dn3 + locals.var_qjei_dn3);
        let eq1_e170_d_n4: f64 = (locals.var_qdeix_dn4 + locals.var_qjei_dn4);
        let eq1_e170_d_n5: f64 = (locals.var_qdeix_dn5 + locals.var_qjei_dn5);
        let eq1_e170_d_n6: f64 = (locals.var_qdeix_dn6 + locals.var_qjei_dn6);
        let eq1_e170_d_n7: f64 = (locals.var_qdeix_dn7 + locals.var_qjei_dn7);
        let eq1_e170_d_n8: f64 = (locals.var_qdeix_dn8 + locals.var_qjei_dn8);
        let eq1_e170_d_n9: f64 = (locals.var_qdeix_dn9 + locals.var_qjei_dn9);
        let eq1_e171: f64 = (p.p148 * eq1_e170);
        let eq1_e171_d_n0: f64 = (p.p148 * eq1_e170_d_n0);
        let eq1_e171_d_n1: f64 = (p.p148 * eq1_e170_d_n1);
        let eq1_e171_d_n3: f64 = (p.p148 * eq1_e170_d_n3);
        let eq1_e171_d_n4: f64 = (p.p148 * eq1_e170_d_n4);
        let eq1_e171_d_n5: f64 = (p.p148 * eq1_e170_d_n5);
        let eq1_e171_d_n6: f64 = (p.p148 * eq1_e170_d_n6);
        let eq1_e171_d_n7: f64 = (p.p148 * eq1_e170_d_n7);
        let eq1_e171_d_n8: f64 = (p.p148 * eq1_e170_d_n8);
        let eq1_e171_d_n9: f64 = (p.p148 * eq1_e170_d_n9);
        let eq1_e171_d_n12: f64 = (p.p148 * locals.var_qdeix_dn12);
        let eq1_e172: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq1_e171);
        let eq1_value: f64 = eq1_e172;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq1_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 12],
            [multiplicity * ((eq1_e171_d_n0 * ddt_scale)), multiplicity * ((eq1_e171_d_n1 * ddt_scale)), multiplicity * ((eq1_e171_d_n3 * ddt_scale)), multiplicity * ((eq1_e171_d_n4 * ddt_scale)), multiplicity * ((eq1_e171_d_n5 * ddt_scale)), multiplicity * ((eq1_e171_d_n6 * ddt_scale)), multiplicity * ((eq1_e171_d_n7 * ddt_scale)), multiplicity * ((eq1_e171_d_n8 * ddt_scale)), multiplicity * ((eq1_e171_d_n9 * ddt_scale)), multiplicity * ((eq1_e171_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq2_e176: f64 = (locals.var_ibci - locals.var_iavl);
        let eq2_e176_d_n4: f64 = (locals.var_ibci_dn4 - locals.var_iavl_dn4);
        let eq2_e176_d_n5: f64 = (locals.var_ibci_dn5 - locals.var_iavl_dn5);
        let eq2_e176_d_n6: f64 = (locals.var_ibci_dn6 - locals.var_iavl_dn6);
        let eq2_e176_d_n7: f64 = (locals.var_ibci_dn7 - locals.var_iavl_dn7);
        let eq2_e176_d_n8: f64 = (locals.var_ibci_dn8 - locals.var_iavl_dn8);
        let eq2_e176_d_n9: f64 = (locals.var_ibci_dn9 - locals.var_iavl_dn9);
        let eq2_e177: f64 = (p.p148 * eq2_e176);
        let eq2_e177_d_n0: f64 = (p.p148 * (-locals.var_iavl_dn0));
        let eq2_e177_d_n1: f64 = (p.p148 * (-locals.var_iavl_dn1));
        let eq2_e177_d_n3: f64 = (p.p148 * (-locals.var_iavl_dn3));
        let eq2_e177_d_n4: f64 = (p.p148 * eq2_e176_d_n4);
        let eq2_e177_d_n5: f64 = (p.p148 * eq2_e176_d_n5);
        let eq2_e177_d_n6: f64 = (p.p148 * eq2_e176_d_n6);
        let eq2_e177_d_n7: f64 = (p.p148 * eq2_e176_d_n7);
        let eq2_e177_d_n8: f64 = (p.p148 * eq2_e176_d_n8);
        let eq2_e177_d_n9: f64 = (p.p148 * eq2_e176_d_n9);
        let eq2_e180: f64 = (locals.var_gmin * (nv8 - nv5));
        let eq2_e181: f64 = (eq2_e177 + eq2_e180);
        let eq2_e181_d_n5: f64 = (eq2_e177_d_n5 + (-locals.var_gmin));
        let eq2_e181_d_n8: f64 = (eq2_e177_d_n8 + locals.var_gmin);
        let eq2_value: f64 = eq2_e181;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq2_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq2_e177_d_n0), multiplicity * (eq2_e177_d_n1), multiplicity * (eq2_e177_d_n3), multiplicity * (eq2_e177_d_n4), multiplicity * (eq2_e181_d_n5), multiplicity * (eq2_e177_d_n6), multiplicity * (eq2_e177_d_n7), multiplicity * (eq2_e181_d_n8), multiplicity * (eq2_e177_d_n9)],
            [],
            [],
            1.0,
        );
        let eq3_e185: f64 = (locals.var_qdci + locals.var_qjci);
        let eq3_e185_d_n0: f64 = (locals.var_qdci_dn0 + locals.var_qjci_dn0);
        let eq3_e185_d_n1: f64 = (locals.var_qdci_dn1 + locals.var_qjci_dn1);
        let eq3_e185_d_n3: f64 = (locals.var_qdci_dn3 + locals.var_qjci_dn3);
        let eq3_e185_d_n4: f64 = (locals.var_qdci_dn4 + locals.var_qjci_dn4);
        let eq3_e185_d_n5: f64 = (locals.var_qdci_dn5 + locals.var_qjci_dn5);
        let eq3_e185_d_n6: f64 = (locals.var_qdci_dn6 + locals.var_qjci_dn6);
        let eq3_e185_d_n7: f64 = (locals.var_qdci_dn7 + locals.var_qjci_dn7);
        let eq3_e185_d_n8: f64 = (locals.var_qdci_dn8 + locals.var_qjci_dn8);
        let eq3_e185_d_n9: f64 = (locals.var_qdci_dn9 + locals.var_qjci_dn9);
        let eq3_e186: f64 = (p.p148 * eq3_e185);
        let eq3_e186_d_n0: f64 = (p.p148 * eq3_e185_d_n0);
        let eq3_e186_d_n1: f64 = (p.p148 * eq3_e185_d_n1);
        let eq3_e186_d_n3: f64 = (p.p148 * eq3_e185_d_n3);
        let eq3_e186_d_n4: f64 = (p.p148 * eq3_e185_d_n4);
        let eq3_e186_d_n5: f64 = (p.p148 * eq3_e185_d_n5);
        let eq3_e186_d_n6: f64 = (p.p148 * eq3_e185_d_n6);
        let eq3_e186_d_n7: f64 = (p.p148 * eq3_e185_d_n7);
        let eq3_e186_d_n8: f64 = (p.p148 * eq3_e185_d_n8);
        let eq3_e186_d_n9: f64 = (p.p148 * eq3_e185_d_n9);
        let eq3_e187: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq3_e186);
        let eq3_value: f64 = eq3_e187;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq3_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq3_e186_d_n0 * ddt_scale)), multiplicity * ((eq3_e186_d_n1 * ddt_scale)), multiplicity * ((eq3_e186_d_n3 * ddt_scale)), multiplicity * ((eq3_e186_d_n4 * ddt_scale)), multiplicity * ((eq3_e186_d_n5 * ddt_scale)), multiplicity * ((eq3_e186_d_n6 * ddt_scale)), multiplicity * ((eq3_e186_d_n7 * ddt_scale)), multiplicity * ((eq3_e186_d_n8 * ddt_scale)), multiplicity * ((eq3_e186_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq4_e190: f64 = (p.p148 * locals.var_itxf);
        let eq4_e190_d_n0: f64 = (p.p148 * locals.var_itxf_dn0);
        let eq4_e190_d_n1: f64 = (p.p148 * locals.var_itxf_dn1);
        let eq4_e190_d_n3: f64 = (p.p148 * locals.var_itxf_dn3);
        let eq4_e190_d_n4: f64 = (p.p148 * locals.var_itxf_dn4);
        let eq4_e190_d_n5: f64 = (p.p148 * locals.var_itxf_dn5);
        let eq4_e190_d_n6: f64 = (p.p148 * locals.var_itxf_dn6);
        let eq4_e190_d_n7: f64 = (p.p148 * locals.var_itxf_dn7);
        let eq4_e190_d_n8: f64 = (p.p148 * locals.var_itxf_dn8);
        let eq4_e190_d_n9: f64 = (p.p148 * locals.var_itxf_dn9);
        let eq4_e190_d_n11: f64 = (p.p148 * locals.var_itxf_dn11);
        let eq4_value: f64 = eq4_e190;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq4_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 11],
            [multiplicity * (eq4_e190_d_n0), multiplicity * (eq4_e190_d_n1), multiplicity * (eq4_e190_d_n3), multiplicity * (eq4_e190_d_n4), multiplicity * (eq4_e190_d_n5), multiplicity * (eq4_e190_d_n6), multiplicity * (eq4_e190_d_n7), multiplicity * (eq4_e190_d_n8), multiplicity * (eq4_e190_d_n9), multiplicity * (eq4_e190_d_n11)],
            [],
            [],
            1.0,
        );
        let eq5_e193: f64 = (p.p148 * locals.var_itr);
        let eq5_e193_d_n0: f64 = (p.p148 * locals.var_itr_dn0);
        let eq5_e193_d_n1: f64 = (p.p148 * locals.var_itr_dn1);
        let eq5_e193_d_n3: f64 = (p.p148 * locals.var_itr_dn3);
        let eq5_e193_d_n4: f64 = (p.p148 * locals.var_itr_dn4);
        let eq5_e193_d_n5: f64 = (p.p148 * locals.var_itr_dn5);
        let eq5_e193_d_n6: f64 = (p.p148 * locals.var_itr_dn6);
        let eq5_e193_d_n7: f64 = (p.p148 * locals.var_itr_dn7);
        let eq5_e193_d_n8: f64 = (p.p148 * locals.var_itr_dn8);
        let eq5_e193_d_n9: f64 = (p.p148 * locals.var_itr_dn9);
        let eq5_value: f64 = eq5_e193;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(5),
            multiplicity * (eq5_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq5_e193_d_n0), multiplicity * (eq5_e193_d_n1), multiplicity * (eq5_e193_d_n3), multiplicity * (eq5_e193_d_n4), multiplicity * (eq5_e193_d_n5), multiplicity * (eq5_e193_d_n6), multiplicity * (eq5_e193_d_n7), multiplicity * (eq5_e193_d_n8), multiplicity * (eq5_e193_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq6_e199, eq6_e199_d_n0, eq6_e199_d_n1, eq6_e199_d_n3, eq6_e199_d_n4, eq6_e199_d_n5, eq6_e199_d_n6, eq6_e199_d_n7, eq6_e199_d_n8, eq6_e199_d_n9,) = {
    if (locals.var_guard233 != 0.0) {
        let eq6_e197: f64 = ((nv7 - nv8) / locals.var_rbi);
        let eq6_e197_d_n0: f64 = (-(((nv7 - nv8) * locals.var_rbi_dn0) / (locals.var_rbi * locals.var_rbi)));
        let eq6_e197_d_n1: f64 = (-(((nv7 - nv8) * locals.var_rbi_dn1) / (locals.var_rbi * locals.var_rbi)));
        let eq6_e197_d_n3: f64 = (-(((nv7 - nv8) * locals.var_rbi_dn3) / (locals.var_rbi * locals.var_rbi)));
        let eq6_e197_d_n4: f64 = (-(((nv7 - nv8) * locals.var_rbi_dn4) / (locals.var_rbi * locals.var_rbi)));
        let eq6_e197_d_n5: f64 = (-(((nv7 - nv8) * locals.var_rbi_dn5) / (locals.var_rbi * locals.var_rbi)));
        let eq6_e197_d_n6: f64 = (-(((nv7 - nv8) * locals.var_rbi_dn6) / (locals.var_rbi * locals.var_rbi)));
        let __rspice_inv_cse_0: f64 = 1.0 / (locals.var_rbi * locals.var_rbi);
        let eq6_e197_d_n7: f64 = ((locals.var_rbi - ((nv7 - nv8) * locals.var_rbi_dn7)) * __rspice_inv_cse_0);
        let eq6_e197_d_n8: f64 = (((-locals.var_rbi) - ((nv7 - nv8) * locals.var_rbi_dn8)) * __rspice_inv_cse_0);
        let eq6_e197_d_n9: f64 = (-(((nv7 - nv8) * locals.var_rbi_dn9) / (locals.var_rbi * locals.var_rbi)));
        (eq6_e197, eq6_e197_d_n0, eq6_e197_d_n1, eq6_e197_d_n3, eq6_e197_d_n4, eq6_e197_d_n5, eq6_e197_d_n6, eq6_e197_d_n7, eq6_e197_d_n8, eq6_e197_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e199;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(7),
            Some(8),
            multiplicity * (eq6_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq6_e199_d_n0), multiplicity * (eq6_e199_d_n1), multiplicity * (eq6_e199_d_n3), multiplicity * (eq6_e199_d_n4), multiplicity * (eq6_e199_d_n5), multiplicity * (eq6_e199_d_n6), multiplicity * (eq6_e199_d_n7), multiplicity * (eq6_e199_d_n8), multiplicity * (eq6_e199_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq7_e206, eq7_e206_d_n0, eq7_e206_d_n1, eq7_e206_d_n3, eq7_e206_d_n4, eq7_e206_d_n5, eq7_e206_d_n6, eq7_e206_d_n7, eq7_e206_d_n8, eq7_e206_d_n9,) = {
    if ((locals.var_guard233 != 0.0) && (locals.var_guard234 != 0.0)) {
        let eq7_e204: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, locals.var_qrbi);
        (eq7_e204, (locals.var_qrbi_dn0 * ddt_scale), (locals.var_qrbi_dn1 * ddt_scale), (locals.var_qrbi_dn3 * ddt_scale), (locals.var_qrbi_dn4 * ddt_scale), (locals.var_qrbi_dn5 * ddt_scale), (locals.var_qrbi_dn6 * ddt_scale), (locals.var_qrbi_dn7 * ddt_scale), (locals.var_qrbi_dn8 * ddt_scale), (locals.var_qrbi_dn9 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e206;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(7),
            Some(8),
            multiplicity * (eq7_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq7_e206_d_n0), multiplicity * (eq7_e206_d_n1), multiplicity * (eq7_e206_d_n3), multiplicity * (eq7_e206_d_n4), multiplicity * (eq7_e206_d_n5), multiplicity * (eq7_e206_d_n6), multiplicity * (eq7_e206_d_n7), multiplicity * (eq7_e206_d_n8), multiplicity * (eq7_e206_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq9_e218, eq9_e218_d_n0, eq9_e218_d_n1, eq9_e218_d_n3, eq9_e218_d_n4, eq9_e218_d_n5, eq9_e218_d_n6, eq9_e218_d_n7, eq9_e218_d_n8, eq9_e218_d_n9,) = {
    if (locals.var_guard235 != 0.0) {
        let eq9_e214: f64 = (-p.p148);
        let eq9_e216: f64 = (eq9_e214 * locals.var_ibebtb);
        let eq9_e216_d_n0: f64 = (eq9_e214 * locals.var_ibebtb_dn0);
        let eq9_e216_d_n1: f64 = (eq9_e214 * locals.var_ibebtb_dn1);
        let eq9_e216_d_n3: f64 = (eq9_e214 * locals.var_ibebtb_dn3);
        let eq9_e216_d_n4: f64 = (eq9_e214 * locals.var_ibebtb_dn4);
        let eq9_e216_d_n5: f64 = (eq9_e214 * locals.var_ibebtb_dn5);
        let eq9_e216_d_n6: f64 = (eq9_e214 * locals.var_ibebtb_dn6);
        let eq9_e216_d_n7: f64 = (eq9_e214 * locals.var_ibebtb_dn7);
        let eq9_e216_d_n8: f64 = (eq9_e214 * locals.var_ibebtb_dn8);
        let eq9_e216_d_n9: f64 = (eq9_e214 * locals.var_ibebtb_dn9);
        (eq9_e216, eq9_e216_d_n0, eq9_e216_d_n1, eq9_e216_d_n3, eq9_e216_d_n4, eq9_e216_d_n5, eq9_e216_d_n6, eq9_e216_d_n7, eq9_e216_d_n8, eq9_e216_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e218;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq9_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq9_e218_d_n0), multiplicity * (eq9_e218_d_n1), multiplicity * (eq9_e218_d_n3), multiplicity * (eq9_e218_d_n4), multiplicity * (eq9_e218_d_n5), multiplicity * (eq9_e218_d_n6), multiplicity * (eq9_e218_d_n7), multiplicity * (eq9_e218_d_n8), multiplicity * (eq9_e218_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq10_e226, eq10_e226_d_n0, eq10_e226_d_n1, eq10_e226_d_n3, eq10_e226_d_n4, eq10_e226_d_n5, eq10_e226_d_n6, eq10_e226_d_n7, eq10_e226_d_n8, eq10_e226_d_n9,) = {
    if (locals.var_guard235 == 0.0) {
        let eq10_e222: f64 = (-p.p148);
        let eq10_e224: f64 = (eq10_e222 * locals.var_ibebtb);
        let eq10_e224_d_n0: f64 = (eq10_e222 * locals.var_ibebtb_dn0);
        let eq10_e224_d_n1: f64 = (eq10_e222 * locals.var_ibebtb_dn1);
        let eq10_e224_d_n3: f64 = (eq10_e222 * locals.var_ibebtb_dn3);
        let eq10_e224_d_n4: f64 = (eq10_e222 * locals.var_ibebtb_dn4);
        let eq10_e224_d_n5: f64 = (eq10_e222 * locals.var_ibebtb_dn5);
        let eq10_e224_d_n6: f64 = (eq10_e222 * locals.var_ibebtb_dn6);
        let eq10_e224_d_n7: f64 = (eq10_e222 * locals.var_ibebtb_dn7);
        let eq10_e224_d_n8: f64 = (eq10_e222 * locals.var_ibebtb_dn8);
        let eq10_e224_d_n9: f64 = (eq10_e222 * locals.var_ibebtb_dn9);
        (eq10_e224, eq10_e224_d_n0, eq10_e224_d_n1, eq10_e224_d_n3, eq10_e224_d_n4, eq10_e224_d_n5, eq10_e224_d_n6, eq10_e224_d_n7, eq10_e224_d_n8, eq10_e224_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e226;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq10_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq10_e226_d_n0), multiplicity * (eq10_e226_d_n1), multiplicity * (eq10_e226_d_n3), multiplicity * (eq10_e226_d_n4), multiplicity * (eq10_e226_d_n5), multiplicity * (eq10_e226_d_n6), multiplicity * (eq10_e226_d_n7), multiplicity * (eq10_e226_d_n8), multiplicity * (eq10_e226_d_n9)],
            [],
            [],
            1.0,
        );
        let eq11_e228: f64 = (-p.p148);
        let eq11_e230: f64 = (eq11_e228 * locals.var_ibcbtb);
        let eq11_e230_d_n0: f64 = (eq11_e228 * locals.var_ibcbtb_dn0);
        let eq11_e230_d_n1: f64 = (eq11_e228 * locals.var_ibcbtb_dn1);
        let eq11_e230_d_n3: f64 = (eq11_e228 * locals.var_ibcbtb_dn3);
        let eq11_e230_d_n4: f64 = (eq11_e228 * locals.var_ibcbtb_dn4);
        let eq11_e230_d_n5: f64 = (eq11_e228 * locals.var_ibcbtb_dn5);
        let eq11_e230_d_n6: f64 = (eq11_e228 * locals.var_ibcbtb_dn6);
        let eq11_e230_d_n7: f64 = (eq11_e228 * locals.var_ibcbtb_dn7);
        let eq11_e230_d_n8: f64 = (eq11_e228 * locals.var_ibcbtb_dn8);
        let eq11_e230_d_n9: f64 = (eq11_e228 * locals.var_ibcbtb_dn9);
        let eq11_value: f64 = eq11_e230;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq11_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq11_e230_d_n0), multiplicity * (eq11_e230_d_n1), multiplicity * (eq11_e230_d_n3), multiplicity * (eq11_e230_d_n4), multiplicity * (eq11_e230_d_n5), multiplicity * (eq11_e230_d_n6), multiplicity * (eq11_e230_d_n7), multiplicity * (eq11_e230_d_n8), multiplicity * (eq11_e230_d_n9)],
            [],
            [],
            1.0,
        );
        let eq12_e234: f64 = (locals.var_ibep + locals.var_irep);
        let eq12_e234_d_n4: f64 = (locals.var_ibep_dn4 + locals.var_irep_dn4);
        let eq12_e234_d_n5: f64 = (locals.var_ibep_dn5 + locals.var_irep_dn5);
        let eq12_e234_d_n6: f64 = (locals.var_ibep_dn6 + locals.var_irep_dn6);
        let eq12_e234_d_n7: f64 = (locals.var_ibep_dn7 + locals.var_irep_dn7);
        let eq12_e234_d_n8: f64 = (locals.var_ibep_dn8 + locals.var_irep_dn8);
        let eq12_e234_d_n9: f64 = (locals.var_ibep_dn9 + locals.var_irep_dn9);
        let eq12_e235: f64 = (p.p148 * eq12_e234);
        let eq12_e235_d_n4: f64 = (p.p148 * eq12_e234_d_n4);
        let eq12_e235_d_n5: f64 = (p.p148 * eq12_e234_d_n5);
        let eq12_e235_d_n6: f64 = (p.p148 * eq12_e234_d_n6);
        let eq12_e235_d_n7: f64 = (p.p148 * eq12_e234_d_n7);
        let eq12_e235_d_n8: f64 = (p.p148 * eq12_e234_d_n8);
        let eq12_e235_d_n9: f64 = (p.p148 * eq12_e234_d_n9);
        let eq12_value: f64 = eq12_e235;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq12_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq12_e235_d_n4), multiplicity * (eq12_e235_d_n5), multiplicity * (eq12_e235_d_n6), multiplicity * (eq12_e235_d_n7), multiplicity * (eq12_e235_d_n8), multiplicity * (eq12_e235_d_n9)],
            [],
            [],
            1.0,
        );
        let eq13_e238: f64 = (p.p148 * locals.var_qjep);
        let eq13_e238_d_n0: f64 = (p.p148 * locals.var_qjep_dn0);
        let eq13_e238_d_n1: f64 = (p.p148 * locals.var_qjep_dn1);
        let eq13_e238_d_n3: f64 = (p.p148 * locals.var_qjep_dn3);
        let eq13_e238_d_n4: f64 = (p.p148 * locals.var_qjep_dn4);
        let eq13_e238_d_n5: f64 = (p.p148 * locals.var_qjep_dn5);
        let eq13_e238_d_n6: f64 = (p.p148 * locals.var_qjep_dn6);
        let eq13_e238_d_n7: f64 = (p.p148 * locals.var_qjep_dn7);
        let eq13_e238_d_n8: f64 = (p.p148 * locals.var_qjep_dn8);
        let eq13_e238_d_n9: f64 = (p.p148 * locals.var_qjep_dn9);
        let eq13_e239: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq13_e238);
        let eq13_value: f64 = eq13_e239;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq13_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq13_e238_d_n0 * ddt_scale)), multiplicity * ((eq13_e238_d_n1 * ddt_scale)), multiplicity * ((eq13_e238_d_n3 * ddt_scale)), multiplicity * ((eq13_e238_d_n4 * ddt_scale)), multiplicity * ((eq13_e238_d_n5 * ddt_scale)), multiplicity * ((eq13_e238_d_n6 * ddt_scale)), multiplicity * ((eq13_e238_d_n7 * ddt_scale)), multiplicity * ((eq13_e238_d_n8 * ddt_scale)), multiplicity * ((eq13_e238_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq14_e242: f64 = (p.p148 * locals.var_ijbcx);
        let eq14_e242_d_n4: f64 = (p.p148 * locals.var_ijbcx_dn4);
        let eq14_e242_d_n5: f64 = (p.p148 * locals.var_ijbcx_dn5);
        let eq14_e242_d_n6: f64 = (p.p148 * locals.var_ijbcx_dn6);
        let eq14_e242_d_n7: f64 = (p.p148 * locals.var_ijbcx_dn7);
        let eq14_e242_d_n8: f64 = (p.p148 * locals.var_ijbcx_dn8);
        let eq14_e242_d_n9: f64 = (p.p148 * locals.var_ijbcx_dn9);
        let eq14_value: f64 = eq14_e242;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(5),
            multiplicity * (eq14_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq14_e242_d_n4), multiplicity * (eq14_e242_d_n5), multiplicity * (eq14_e242_d_n6), multiplicity * (eq14_e242_d_n7), multiplicity * (eq14_e242_d_n8), multiplicity * (eq14_e242_d_n9)],
            [],
            [],
            1.0,
        );
        let eq15_e246: f64 = (locals.var_qjcx0_t_p + locals.var_qdsu);
        let eq15_e246_d_n4: f64 = (locals.var_qjcx0_t_p_dn4 + locals.var_qdsu_dn4);
        let eq15_e246_d_n5: f64 = (locals.var_qjcx0_t_p_dn5 + locals.var_qdsu_dn5);
        let eq15_e246_d_n7: f64 = (locals.var_qjcx0_t_p_dn7 + locals.var_qdsu_dn7);
        let eq15_e247: f64 = (p.p148 * eq15_e246);
        let eq15_e247_d_n0: f64 = (p.p148 * locals.var_qjcx0_t_p_dn0);
        let eq15_e247_d_n1: f64 = (p.p148 * locals.var_qjcx0_t_p_dn1);
        let eq15_e247_d_n3: f64 = (p.p148 * locals.var_qjcx0_t_p_dn3);
        let eq15_e247_d_n4: f64 = (p.p148 * eq15_e246_d_n4);
        let eq15_e247_d_n5: f64 = (p.p148 * eq15_e246_d_n5);
        let eq15_e247_d_n6: f64 = (p.p148 * locals.var_qjcx0_t_p_dn6);
        let eq15_e247_d_n7: f64 = (p.p148 * eq15_e246_d_n7);
        let eq15_e247_d_n8: f64 = (p.p148 * locals.var_qjcx0_t_p_dn8);
        let eq15_e247_d_n9: f64 = (p.p148 * locals.var_qjcx0_t_p_dn9);
        let eq15_e248: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq15_e247);
        let eq15_value: f64 = eq15_e248;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(7),
            Some(5),
            multiplicity * (eq15_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq15_e247_d_n0 * ddt_scale)), multiplicity * ((eq15_e247_d_n1 * ddt_scale)), multiplicity * ((eq15_e247_d_n3 * ddt_scale)), multiplicity * ((eq15_e247_d_n4 * ddt_scale)), multiplicity * ((eq15_e247_d_n5 * ddt_scale)), multiplicity * ((eq15_e247_d_n6 * ddt_scale)), multiplicity * ((eq15_e247_d_n7 * ddt_scale)), multiplicity * ((eq15_e247_d_n8 * ddt_scale)), multiplicity * ((eq15_e247_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq17_e255: f64 = (p.p148 * locals.var_qjcx0_t_x);
        let eq17_e255_d_n0: f64 = (p.p148 * locals.var_qjcx0_t_x_dn0);
        let eq17_e255_d_n1: f64 = (p.p148 * locals.var_qjcx0_t_x_dn1);
        let eq17_e255_d_n3: f64 = (p.p148 * locals.var_qjcx0_t_x_dn3);
        let eq17_e255_d_n4: f64 = (p.p148 * locals.var_qjcx0_t_x_dn4);
        let eq17_e255_d_n5: f64 = (p.p148 * locals.var_qjcx0_t_x_dn5);
        let eq17_e255_d_n6: f64 = (p.p148 * locals.var_qjcx0_t_x_dn6);
        let eq17_e255_d_n7: f64 = (p.p148 * locals.var_qjcx0_t_x_dn7);
        let eq17_e255_d_n8: f64 = (p.p148 * locals.var_qjcx0_t_x_dn8);
        let eq17_e255_d_n9: f64 = (p.p148 * locals.var_qjcx0_t_x_dn9);
        let eq17_e256: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq17_e255);
        let eq17_value: f64 = eq17_e256;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(5),
            multiplicity * (eq17_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq17_e255_d_n0 * ddt_scale)), multiplicity * ((eq17_e255_d_n1 * ddt_scale)), multiplicity * ((eq17_e255_d_n3 * ddt_scale)), multiplicity * ((eq17_e255_d_n4 * ddt_scale)), multiplicity * ((eq17_e255_d_n5 * ddt_scale)), multiplicity * ((eq17_e255_d_n6 * ddt_scale)), multiplicity * ((eq17_e255_d_n7 * ddt_scale)), multiplicity * ((eq17_e255_d_n8 * ddt_scale)), multiplicity * ((eq17_e255_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let (eq19_e266, eq19_e266_d_n1, eq19_e266_d_n4, eq19_e266_d_n7,) = {
    if (locals.var_guard236 != 0.0) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_rbx_t;
        let eq19_e264: f64 = ((nv1 - nv7) * __rspice_inv_cse_1);
        let eq19_e264_d_n1: f64 = (1.0 * __rspice_inv_cse_1);
        let eq19_e264_d_n4: f64 = (-(((nv1 - nv7) * locals.var_rbx_t_dn4) / (locals.var_rbx_t * locals.var_rbx_t)));
        let eq19_e264_d_n7: f64 = (-1.0 / locals.var_rbx_t);
        (eq19_e264, eq19_e264_d_n1, eq19_e264_d_n4, eq19_e264_d_n7,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e266;
        stamper.stamp_current_node3_local(
            Some(1),
            Some(7),
            multiplicity * (eq19_value),
            1,
            multiplicity * (eq19_e266_d_n1),
            4,
            multiplicity * (eq19_e266_d_n4),
            7,
            multiplicity * (eq19_e266_d_n7),
        );
        let (eq21_e277, eq21_e277_d_n2, eq21_e277_d_n4, eq21_e277_d_n6,) = {
    if (locals.var_guard237 != 0.0) {
        let __rspice_inv_cse_2: f64 = 1.0 / locals.var_re_t;
        let eq21_e275: f64 = ((nv6 - nv2) * __rspice_inv_cse_2);
        let eq21_e275_d_n2: f64 = ((-1.0) * __rspice_inv_cse_2);
        let eq21_e275_d_n4: f64 = (-(((nv6 - nv2) * locals.var_re_t_dn4) / (locals.var_re_t * locals.var_re_t)));
        let eq21_e275_d_n6: f64 = (1.0 / locals.var_re_t);
        (eq21_e275, eq21_e275_d_n2, eq21_e275_d_n4, eq21_e275_d_n6,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e277;
        stamper.stamp_current_node3_local(
            Some(6),
            Some(2),
            multiplicity * (eq21_value),
            2,
            multiplicity * (eq21_e277_d_n2),
            4,
            multiplicity * (eq21_e277_d_n4),
            6,
            multiplicity * (eq21_e277_d_n6),
        );
        let (eq23_e288, eq23_e288_d_n0, eq23_e288_d_n4, eq23_e288_d_n5,) = {
    if (locals.var_guard238 != 0.0) {
        let __rspice_inv_cse_3: f64 = 1.0 / locals.var_rcx_t;
        let eq23_e286: f64 = ((nv5 - nv0) * __rspice_inv_cse_3);
        let eq23_e286_d_n0: f64 = ((-1.0) * __rspice_inv_cse_3);
        let eq23_e286_d_n4: f64 = (-(((nv5 - nv0) * locals.var_rcx_t_dn4) / (locals.var_rcx_t * locals.var_rcx_t)));
        let eq23_e286_d_n5: f64 = (1.0 / locals.var_rcx_t);
        (eq23_e286, eq23_e286_d_n0, eq23_e286_d_n4, eq23_e286_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e288;
        stamper.stamp_current_node3_local(
            Some(5),
            Some(0),
            multiplicity * (eq23_value),
            0,
            multiplicity * (eq23_e288_d_n0),
            4,
            multiplicity * (eq23_e288_d_n4),
            5,
            multiplicity * (eq23_e288_d_n5),
        );
        let eq28_e308: f64 = (p.p148 * locals.var_it_sub);
        let eq28_e308_d_n4: f64 = (p.p148 * locals.var_it_sub_dn4);
        let eq28_e308_d_n5: f64 = (p.p148 * locals.var_it_sub_dn5);
        let eq28_e308_d_n7: f64 = (p.p148 * locals.var_it_sub_dn7);
        let eq28_e308_d_n9: f64 = (p.p148 * locals.var_it_sub_dn9);
        let eq28_value: f64 = eq28_e308;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq28_value),
            [4, 5, 7, 9],
            [multiplicity * (eq28_e308_d_n4), multiplicity * (eq28_e308_d_n5), multiplicity * (eq28_e308_d_n7), multiplicity * (eq28_e308_d_n9)],
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq29_e316, eq29_e316_d_n4, eq29_e316_d_n5, eq29_e316_d_n6, eq29_e316_d_n7, eq29_e316_d_n8, eq29_e316_d_n9,) = {
    if ((locals.var_guard239 != 0.0) && (locals.var_guard240 != 0.0)) {
        let eq29_e314: f64 = (p.p148 * locals.var_ijsc);
        let eq29_e314_d_n4: f64 = (p.p148 * locals.var_ijsc_dn4);
        let eq29_e314_d_n5: f64 = (p.p148 * locals.var_ijsc_dn5);
        let eq29_e314_d_n6: f64 = (p.p148 * locals.var_ijsc_dn6);
        let eq29_e314_d_n7: f64 = (p.p148 * locals.var_ijsc_dn7);
        let eq29_e314_d_n8: f64 = (p.p148 * locals.var_ijsc_dn8);
        let eq29_e314_d_n9: f64 = (p.p148 * locals.var_ijsc_dn9);
        (eq29_e314, eq29_e314_d_n4, eq29_e314_d_n5, eq29_e314_d_n6, eq29_e314_d_n7, eq29_e314_d_n8, eq29_e314_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e316;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(5),
            multiplicity * (eq29_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq29_e316_d_n4), multiplicity * (eq29_e316_d_n5), multiplicity * (eq29_e316_d_n6), multiplicity * (eq29_e316_d_n7), multiplicity * (eq29_e316_d_n8), multiplicity * (eq29_e316_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq31_e331, eq31_e331_d_n4, eq31_e331_d_n5, eq31_e331_d_n6, eq31_e331_d_n7, eq31_e331_d_n8, eq31_e331_d_n9,) = {
    if (locals.var_guard239 == 0.0) {
        let eq31_e329: f64 = (p.p148 * locals.var_ijsc);
        let eq31_e329_d_n4: f64 = (p.p148 * locals.var_ijsc_dn4);
        let eq31_e329_d_n5: f64 = (p.p148 * locals.var_ijsc_dn5);
        let eq31_e329_d_n6: f64 = (p.p148 * locals.var_ijsc_dn6);
        let eq31_e329_d_n7: f64 = (p.p148 * locals.var_ijsc_dn7);
        let eq31_e329_d_n8: f64 = (p.p148 * locals.var_ijsc_dn8);
        let eq31_e329_d_n9: f64 = (p.p148 * locals.var_ijsc_dn9);
        (eq31_e329, eq31_e329_d_n4, eq31_e329_d_n5, eq31_e329_d_n6, eq31_e329_d_n7, eq31_e329_d_n8, eq31_e329_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e331;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(5),
            multiplicity * (eq31_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq31_e331_d_n4), multiplicity * (eq31_e331_d_n5), multiplicity * (eq31_e331_d_n6), multiplicity * (eq31_e331_d_n7), multiplicity * (eq31_e331_d_n8), multiplicity * (eq31_e331_d_n9)],
            [],
            [],
            1.0,
        );
        let eq33_e343: f64 = (p.p148 * locals.var_qjs);
        let eq33_e343_d_n0: f64 = (p.p148 * locals.var_qjs_dn0);
        let eq33_e343_d_n1: f64 = (p.p148 * locals.var_qjs_dn1);
        let eq33_e343_d_n3: f64 = (p.p148 * locals.var_qjs_dn3);
        let eq33_e343_d_n4: f64 = (p.p148 * locals.var_qjs_dn4);
        let eq33_e343_d_n5: f64 = (p.p148 * locals.var_qjs_dn5);
        let eq33_e343_d_n6: f64 = (p.p148 * locals.var_qjs_dn6);
        let eq33_e343_d_n7: f64 = (p.p148 * locals.var_qjs_dn7);
        let eq33_e343_d_n8: f64 = (p.p148 * locals.var_qjs_dn8);
        let eq33_e343_d_n9: f64 = (p.p148 * locals.var_qjs_dn9);
        let eq33_e344: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq33_e343);
        let eq33_value: f64 = eq33_e344;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(9),
            Some(5),
            multiplicity * (eq33_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq33_e343_d_n0 * ddt_scale)), multiplicity * ((eq33_e343_d_n1 * ddt_scale)), multiplicity * ((eq33_e343_d_n3 * ddt_scale)), multiplicity * ((eq33_e343_d_n4 * ddt_scale)), multiplicity * ((eq33_e343_d_n5 * ddt_scale)), multiplicity * ((eq33_e343_d_n6 * ddt_scale)), multiplicity * ((eq33_e343_d_n7 * ddt_scale)), multiplicity * ((eq33_e343_d_n8 * ddt_scale)), multiplicity * ((eq33_e343_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq34_e347: f64 = (p.p148 * locals.var_qscp);
        let eq34_e347_d_n0: f64 = (p.p148 * locals.var_qscp_dn0);
        let eq34_e347_d_n1: f64 = (p.p148 * locals.var_qscp_dn1);
        let eq34_e347_d_n3: f64 = (p.p148 * locals.var_qscp_dn3);
        let eq34_e347_d_n4: f64 = (p.p148 * locals.var_qscp_dn4);
        let eq34_e347_d_n5: f64 = (p.p148 * locals.var_qscp_dn5);
        let eq34_e347_d_n6: f64 = (p.p148 * locals.var_qscp_dn6);
        let eq34_e347_d_n7: f64 = (p.p148 * locals.var_qscp_dn7);
        let eq34_e347_d_n8: f64 = (p.p148 * locals.var_qscp_dn8);
        let eq34_e347_d_n9: f64 = (p.p148 * locals.var_qscp_dn9);
        let eq34_e348: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq34_e347);
        let eq34_value: f64 = eq34_e348;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(3),
            Some(0),
            multiplicity * (eq34_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq34_e347_d_n0 * ddt_scale)), multiplicity * ((eq34_e347_d_n1 * ddt_scale)), multiplicity * ((eq34_e347_d_n3 * ddt_scale)), multiplicity * ((eq34_e347_d_n4 * ddt_scale)), multiplicity * ((eq34_e347_d_n5 * ddt_scale)), multiplicity * ((eq34_e347_d_n6 * ddt_scale)), multiplicity * ((eq34_e347_d_n7 * ddt_scale)), multiplicity * ((eq34_e347_d_n8 * ddt_scale)), multiplicity * ((eq34_e347_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let (eq36_e363, eq36_e363_d_n3, eq36_e363_d_n9,) = {
    if ((locals.var_guard242 != 0.0) && (locals.var_guard243 != 0.0)) {
        let eq36_e360: f64 = (p.p103 * (nv9 - nv3));
        let eq36_e361: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq36_e360);
        (eq36_e361, ((-p.p103) * ddt_scale), (p.p103 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e363;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(3),
            multiplicity * (eq36_value),
            3,
            multiplicity * (eq36_e363_d_n3),
            9,
            multiplicity * (eq36_e363_d_n9),
        );
        let (eq38_e376, eq38_e376_d_n0, eq38_e376_d_n1, eq38_e376_d_n2, eq38_e376_d_n3, eq38_e376_d_n4, eq38_e376_d_n5, eq38_e376_d_n6, eq38_e376_d_n7, eq38_e376_d_n8, eq38_e376_d_n9,) = {
    if (locals.var_guard244 != 0.0) {
        let eq38_e372: f64 = ((nv4 - 0.0) / locals.var_rth_t);
        let eq38_e372_d_n4: f64 = ((locals.var_rth_t - ((nv4 - 0.0) * locals.var_rth_t_dn4)) / (locals.var_rth_t * locals.var_rth_t));
        let eq38_e374: f64 = (eq38_e372 - locals.var_pterm);
        let eq38_e374_d_n4: f64 = (eq38_e372_d_n4 - locals.var_pterm_dn4);
        (eq38_e374, (-locals.var_pterm_dn0), (-locals.var_pterm_dn1), (-locals.var_pterm_dn2), (-locals.var_pterm_dn3), eq38_e374_d_n4, (-locals.var_pterm_dn5), (-locals.var_pterm_dn6), (-locals.var_pterm_dn7), (-locals.var_pterm_dn8), (-locals.var_pterm_dn9),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e376;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(4),
            None,
            multiplicity * (eq38_value),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq38_e376_d_n0), multiplicity * (eq38_e376_d_n1), multiplicity * (eq38_e376_d_n2), multiplicity * (eq38_e376_d_n3), multiplicity * (eq38_e376_d_n4), multiplicity * (eq38_e376_d_n5), multiplicity * (eq38_e376_d_n6), multiplicity * (eq38_e376_d_n7), multiplicity * (eq38_e376_d_n8), multiplicity * (eq38_e376_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq39_e385, eq39_e385_d_n4,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard245 != 0.0)) {
        let eq39_e382: f64 = (p.p145 * (nv4 - 0.0));
        let eq39_e383: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq39_e382);
        (eq39_e383, (p.p145 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e385;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq39_value),
            4,
            multiplicity * (eq39_e385_d_n4),
        );
        let eq41_value: f64 = locals.var_ixf1;
        let eq41_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq41_node_derivatives: [f64; 11] = [locals.var_ixf1_dn0, locals.var_ixf1_dn1, locals.var_ixf1_dn3, locals.var_ixf1_dn4, locals.var_ixf1_dn5, locals.var_ixf1_dn6, locals.var_ixf1_dn7, locals.var_ixf1_dn8, locals.var_ixf1_dn9, locals.var_ixf1_dn10, locals.var_ixf1_dn11];
        let eq41_branch_derivative_indices: [usize; 0] = [];
        let eq41_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            None,
            multiplicity * (eq41_value),
            &eq41_node_derivative_indices,
            &eq41_node_derivatives,
            &eq41_branch_derivative_indices,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let eq43_value: f64 = locals.var_ixf2;
        let eq43_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq43_node_derivatives: [f64; 11] = [locals.var_ixf2_dn0, locals.var_ixf2_dn1, locals.var_ixf2_dn3, locals.var_ixf2_dn4, locals.var_ixf2_dn5, locals.var_ixf2_dn6, locals.var_ixf2_dn7, locals.var_ixf2_dn8, locals.var_ixf2_dn9, locals.var_ixf2_dn10, locals.var_ixf2_dn11];
        let eq43_branch_derivative_indices: [usize; 0] = [];
        let eq43_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            None,
            multiplicity * (eq43_value),
            &eq43_node_derivative_indices,
            &eq43_node_derivatives,
            &eq43_branch_derivative_indices,
            &eq43_branch_derivatives,
            multiplicity,
        );
        let eq45_value: f64 = locals.var_ixf;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(12),
            None,
            multiplicity * (eq45_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 12],
            [multiplicity * (locals.var_ixf_dn0), multiplicity * (locals.var_ixf_dn1), multiplicity * (locals.var_ixf_dn3), multiplicity * (locals.var_ixf_dn4), multiplicity * (locals.var_ixf_dn5), multiplicity * (locals.var_ixf_dn6), multiplicity * (locals.var_ixf_dn7), multiplicity * (locals.var_ixf_dn8), multiplicity * (locals.var_ixf_dn9), multiplicity * (locals.var_ixf_dn12)],
            [],
            [],
            1.0,
        );
        let (eq65_e534, eq65_e534_d_n0, eq65_e534_d_n1, eq65_e534_d_n3, eq65_e534_d_n4, eq65_e534_d_n5, eq65_e534_d_n6, eq65_e534_d_n7, eq65_e534_d_n8, eq65_e534_d_n9, eq65_e534_d_n13,) = {
    if (locals.var_guard258 != 0.0) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_n_w;
        let eq65_e527: f64 = (locals.var_n_2 * __rspice_inv_cse_0);
        let eq65_e527_d_n0: f64 = (locals.var_n_2_dn0 * __rspice_inv_cse_0);
        let eq65_e527_d_n1: f64 = (locals.var_n_2_dn1 * __rspice_inv_cse_0);
        let eq65_e527_d_n3: f64 = (locals.var_n_2_dn3 * __rspice_inv_cse_0);
        let eq65_e527_d_n4: f64 = (locals.var_n_2_dn4 * __rspice_inv_cse_0);
        let eq65_e527_d_n5: f64 = (locals.var_n_2_dn5 * __rspice_inv_cse_0);
        let eq65_e527_d_n6: f64 = (locals.var_n_2_dn6 * __rspice_inv_cse_0);
        let eq65_e527_d_n7: f64 = (locals.var_n_2_dn7 * __rspice_inv_cse_0);
        let eq65_e527_d_n8: f64 = (locals.var_n_2_dn8 * __rspice_inv_cse_0);
        let eq65_e527_d_n9: f64 = (locals.var_n_2_dn9 * __rspice_inv_cse_0);
        let eq65_e530: f64 = (locals.var_n_w * (nv13 - 0.0));
        let eq65_e531: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq65_e530);
        let eq65_e532: f64 = (eq65_e527 * eq65_e531);
        let eq65_e532_d_n0: f64 = (eq65_e527_d_n0 * eq65_e531);
        let eq65_e532_d_n1: f64 = (eq65_e527_d_n1 * eq65_e531);
        let eq65_e532_d_n3: f64 = (eq65_e527_d_n3 * eq65_e531);
        let eq65_e532_d_n4: f64 = (eq65_e527_d_n4 * eq65_e531);
        let eq65_e532_d_n5: f64 = (eq65_e527_d_n5 * eq65_e531);
        let eq65_e532_d_n6: f64 = (eq65_e527_d_n6 * eq65_e531);
        let eq65_e532_d_n7: f64 = (eq65_e527_d_n7 * eq65_e531);
        let eq65_e532_d_n8: f64 = (eq65_e527_d_n8 * eq65_e531);
        let eq65_e532_d_n9: f64 = (eq65_e527_d_n9 * eq65_e531);
        let eq65_e532_d_n13: f64 = (eq65_e527 * (locals.var_n_w * ddt_scale));
        (eq65_e532, eq65_e532_d_n0, eq65_e532_d_n1, eq65_e532_d_n3, eq65_e532_d_n4, eq65_e532_d_n5, eq65_e532_d_n6, eq65_e532_d_n7, eq65_e532_d_n8, eq65_e532_d_n9, eq65_e532_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e534;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq65_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 13],
            [multiplicity * (eq65_e534_d_n0), multiplicity * (eq65_e534_d_n1), multiplicity * (eq65_e534_d_n3), multiplicity * (eq65_e534_d_n4), multiplicity * (eq65_e534_d_n5), multiplicity * (eq65_e534_d_n6), multiplicity * (eq65_e534_d_n7), multiplicity * (eq65_e534_d_n8), multiplicity * (eq65_e534_d_n9), multiplicity * (eq65_e534_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq66_e545, eq66_e545_d_n0, eq66_e545_d_n1, eq66_e545_d_n3, eq66_e545_d_n4, eq66_e545_d_n5, eq66_e545_d_n6, eq66_e545_d_n7, eq66_e545_d_n8, eq66_e545_d_n9, eq66_e545_d_n14,) = {
    if (locals.var_guard258 != 0.0) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_n_w;
        let eq66_e538: f64 = (locals.var_n_1 * __rspice_inv_cse_1);
        let eq66_e538_d_n0: f64 = (locals.var_n_1_dn0 * __rspice_inv_cse_1);
        let eq66_e538_d_n1: f64 = (locals.var_n_1_dn1 * __rspice_inv_cse_1);
        let eq66_e538_d_n3: f64 = (locals.var_n_1_dn3 * __rspice_inv_cse_1);
        let eq66_e538_d_n4: f64 = (locals.var_n_1_dn4 * __rspice_inv_cse_1);
        let eq66_e538_d_n5: f64 = (locals.var_n_1_dn5 * __rspice_inv_cse_1);
        let eq66_e538_d_n6: f64 = (locals.var_n_1_dn6 * __rspice_inv_cse_1);
        let eq66_e538_d_n7: f64 = (locals.var_n_1_dn7 * __rspice_inv_cse_1);
        let eq66_e538_d_n8: f64 = (locals.var_n_1_dn8 * __rspice_inv_cse_1);
        let eq66_e538_d_n9: f64 = (locals.var_n_1_dn9 * __rspice_inv_cse_1);
        let eq66_e541: f64 = (locals.var_n_w * (nv14 - 0.0));
        let eq66_e542: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, eq66_e541);
        let eq66_e543: f64 = (eq66_e538 * eq66_e542);
        let eq66_e543_d_n0: f64 = (eq66_e538_d_n0 * eq66_e542);
        let eq66_e543_d_n1: f64 = (eq66_e538_d_n1 * eq66_e542);
        let eq66_e543_d_n3: f64 = (eq66_e538_d_n3 * eq66_e542);
        let eq66_e543_d_n4: f64 = (eq66_e538_d_n4 * eq66_e542);
        let eq66_e543_d_n5: f64 = (eq66_e538_d_n5 * eq66_e542);
        let eq66_e543_d_n6: f64 = (eq66_e538_d_n6 * eq66_e542);
        let eq66_e543_d_n7: f64 = (eq66_e538_d_n7 * eq66_e542);
        let eq66_e543_d_n8: f64 = (eq66_e538_d_n8 * eq66_e542);
        let eq66_e543_d_n9: f64 = (eq66_e538_d_n9 * eq66_e542);
        let eq66_e543_d_n14: f64 = (eq66_e538 * (locals.var_n_w * ddt_scale));
        (eq66_e543, eq66_e543_d_n0, eq66_e543_d_n1, eq66_e543_d_n3, eq66_e543_d_n4, eq66_e543_d_n5, eq66_e543_d_n6, eq66_e543_d_n7, eq66_e543_d_n8, eq66_e543_d_n9, eq66_e543_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e545;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq66_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 14],
            [multiplicity * (eq66_e545_d_n0), multiplicity * (eq66_e545_d_n1), multiplicity * (eq66_e545_d_n3), multiplicity * (eq66_e545_d_n4), multiplicity * (eq66_e545_d_n5), multiplicity * (eq66_e545_d_n6), multiplicity * (eq66_e545_d_n7), multiplicity * (eq66_e545_d_n8), multiplicity * (eq66_e545_d_n9), multiplicity * (eq66_e545_d_n14)],
            [],
            [],
            1.0,
        );
    }
}
